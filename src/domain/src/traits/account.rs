use crate::objects::{account, discord_account, riot_account};
use crate::objects::error::AccountRepositoryError;
use crate::objects::account::Account;
pub trait AccountRepository: Send + Sync + 'static  {
  async fn create(id: account::ID, riot_account_id: riot_account::ID, discord_account_id: discord_account::ID) -> Result<&'static Account, AccountRepositoryError>;
  async fn find_by_id(id: account::ID) -> Result<&'static Account, AccountRepositoryError>;
  async fn update(id: account::ID, account: Account) -> Result<&'static Account, AccountRepositoryError>;
  async fn delete(id: account::ID) -> Result<&'static Account, AccountRepositoryError>;
  async fn find_by_riot_account_id(riot_account_id: riot_account::ID) -> Result<&'static Account, AccountRepositoryError>;
  async fn find_by_discord_account_id(discord_account_id: discord_account::ID) -> Result<&'static Account, AccountRepositoryError>;
}