use crate::objects::discord_account::ID;
use crate::objects::error::RiotAccountRepositoryError;
use crate::objects::riot_account::RiotAccount;

pub trait RiotAccountRepository: Send + Sync + 'static  {
  async fn create(&self, account: RiotAccount) -> Result<RiotAccount, RiotAccountRepositoryError>;
  async fn find_by_id(&self, id: ID) -> Result<RiotAccount, RiotAccountRepositoryError>;
  async fn find_by_game_name(&self, game_name: String) -> Result<RiotAccount, RiotAccountRepositoryError>;
  async fn find_by_puuid(&self, puuid: String) -> Result<RiotAccount, RiotAccountRepositoryError>;
  async fn update(&self, id: ID, riot_account: RiotAccount) -> Result<RiotAccount, RiotAccountRepositoryError>;
  async fn delete(&self, id: ID) -> Result<(), RiotAccountRepositoryError>;
}