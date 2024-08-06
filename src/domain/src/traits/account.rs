use crate::objects::{account, discord_account, riot_account};
use crate::objects::error::AccountRepositoryError;
use crate::objects::account::Account;
pub trait AccountRepository: Send + Sync + 'static  {
  async fn create(&self, riot_account_id: riot_account::ID, discord_account_id: discord_account::ID) -> Result<Account, AccountRepositoryError>;
  async fn find_by_id(&self, id: account::ID) -> Result<Account, AccountRepositoryError>;
  async fn update(&self, id: account::ID, account: Account) -> Result<Account, AccountRepositoryError>;
  async fn delete(&self, id: account::ID) -> Result<(), AccountRepositoryError>;
  async fn find_by_riot_account_id(&self, riot_account_id: riot_account::ID) -> Result<Account, AccountRepositoryError>;
  async fn find_by_discord_account_id(&self, discord_account_id: discord_account::ID) -> Result<Account, AccountRepositoryError>;
}