use crate::objects::discord_account::{DiscordAccount, ID};
use crate::objects::error::DiscordAccountRepositoryError;

pub trait DiscordAccountRepository: Send + Sync + 'static  {
  async fn create(&self, discord_account: DiscordAccount) -> Result<DiscordAccount, DiscordAccountRepositoryError>;
  async fn find_by_id(&self, id: ID) -> Result<DiscordAccount, DiscordAccountRepositoryError>;
  async fn update(&self, id: ID, discord_account: DiscordAccount) -> Result<DiscordAccount, DiscordAccountRepositoryError>;
  async fn delete(&self, id: ID) -> Result<(), DiscordAccountRepositoryError>;
  async fn find_by_discord_id(&self, discord_id: String) -> Result<DiscordAccount, DiscordAccountRepositoryError>;
  async fn find_by_username(&self, username: String) -> Result<DiscordAccount, DiscordAccountRepositoryError>;
}