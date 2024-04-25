use tonic::async_trait;
use crate::domain::structs::account::Account;
use crate::domain::structs::errors::AccountRepositoryError;
use crate::domain::structs::player_info::PlayerInfo;

#[async_trait]
pub trait AccountApplications: Sync + Send + 'static {
    async fn fetch_account_by_puuid(&self, puuid: &str) -> Result<Account, AccountRepositoryError>;
    async fn fetch_account_by_discord_id(&self, discord_id: &str) -> Result<Account, AccountRepositoryError>;
    async fn fetch_account_by_user_id(&self, user_id: &str) -> Result<Account, AccountRepositoryError>;
    async fn fetch_player_info_by_puuid(&self, puuid: &str) -> Result<PlayerInfo, AccountRepositoryError>;
}