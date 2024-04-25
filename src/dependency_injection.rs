use sea_orm::DatabaseConnection;
use crate::application::interactor::{AccountService, AssetService, MatchService};
use crate::infrastructure::account_impl::{
    PgUserRepository,
    PgRiotAccountRepository,
    PgDiscordAccountRepository,
};
use crate::infrastructure::asset_impl::{PgAssetRepository};
use crate::infrastructure::match_impl::{APIHistoryRepository, APIMatchRepository};

pub type AccServ = AccountService<PgUserRepository, PgRiotAccountRepository, PgDiscordAccountRepository>;
pub type AssetServ = AssetService<PgAssetRepository>;
type MatchRepository = APIMatchRepository<PgAssetRepository>;
pub type MatchServ = MatchService<MatchRepository, APIHistoryRepository<MatchRepository>>;


pub fn dependency_injection(
    pg_client: DatabaseConnection,
    req_client: reqwest::Client,
) -> (AccServ, AssetServ, MatchServ) {
    let req_client = Box::new(req_client);

    let asset_repository = PgAssetRepository::new(&pg_client);
    let match_repository = APIMatchRepository::<PgAssetRepository>::new(&req_client, asset_repository.clone());

    (
        AccountService {
            user_repository: PgUserRepository::new(&pg_client),
            riot_account_repository: PgRiotAccountRepository::new(&pg_client),
            discord_account_repository: PgDiscordAccountRepository::new(&pg_client),
        },
        AssetService {
            asset_repository: asset_repository.clone(),
        },
        MatchService {
            match_repository: APIMatchRepository::<PgAssetRepository>::new(&req_client, asset_repository.clone()),
            history_repository: APIHistoryRepository::<APIMatchRepository<PgAssetRepository>>::new(&req_client, match_repository),
        }
    )
}