use tonic::async_trait;
use crate::domain::structs::discord_account::DiscordAccount;
use crate::domain::structs::errors::DomainError;
use crate::domain::structs::player_info::PlayerInfo;
use crate::domain::structs::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn fetch_user(&self, user_id: &str) -> Result<User, DomainError>;
    async fn fetch_user_by_discord_id(&self, discord_id: &str) -> Result<User, DomainError>;
    async fn fetch_user_by_puuid(&self, puuid: &str) -> Result<User, DomainError>;
}

#[async_trait]
pub trait RiotAccountRepository: Sync + Send + 'static {
    async fn fetch_riot_by_puuid(&self, puuid: &str) -> Result<PlayerInfo, DomainError>;
    async fn fetch_riot_account(&self, user_id: &str) -> Result<PlayerInfo, DomainError>;
}

#[async_trait]
pub trait DiscordAccountRepository: Sync + Send + 'static {
    async fn fetch_discord_account(&self, user_id: &str) -> Result<DiscordAccount, DomainError>;
}