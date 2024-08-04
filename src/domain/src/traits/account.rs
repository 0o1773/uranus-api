use crate::objects::error::AccountRepositoryError;
pub trait Account: Send + Sync + 'static  {
  async fn create_account(&self, riot_account_id: Option<[u8; 16]>, discord_account_id: Option<[u8; 16]>) -> Result<Account, AccountRepositoryError>;
}