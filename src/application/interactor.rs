pub mod account;
pub mod r#match;


pub struct AccountService<UR, RAR, DAR> {
    pub user_repository: UR,
    pub riot_account_repository: RAR,
    pub discord_account_repository: DAR,
}

pub struct MatchService<MR, HR> {
    pub match_repository: MR,
    pub history_repository: HR,
}

pub struct AssetService<AR> {
    pub asset_repository: AR
}