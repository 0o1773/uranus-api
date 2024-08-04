use crate::objects::account;
use crate::objects::discord_account::{DiscordAccount, ID};
use crate::objects::error::DiscordAccountRepositoryError;

pub trait DiscordAccountRepository: Send + Sync + 'static  {
  async fn create(id: ID, account_id: account::ID, discord_id: String, discord_name: String) -> Result<&'static DiscordAccount, DiscordAccountRepositoryError>;
  async fn find_by_id(id: ID) -> Result<&'static DiscordAccount, DiscordAccountRepositoryError>;
  async fn update(id: ID, discord_account: DiscordAccount) -> Result<&'static DiscordAccount, DiscordAccountRepositoryError>;
  async fn delete(id: ID) -> Result<&'static DiscordAccount, DiscordAccountRepositoryError>;
  async fn find_by_discord_id(discord_id: String) -> Result<&'static DiscordAccount, DiscordAccountRepositoryError>;
}