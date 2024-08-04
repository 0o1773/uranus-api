use crate::objects::account;
use crate::objects::discord_account::ID;
use crate::objects::error::RiotAccountRepositoryError;
use crate::objects::riot_account::RiotAccount;

pub trait RiotAccountRepository: Send + Sync + 'static  {
  async fn create(id: ID, account_id: account::ID, puuid: String, summoner_id: String, summoner_name: String) -> Result<&'static RiotAccount, RiotAccountRepositoryError>;
  async fn find_by_id(id: ID) -> Result<&'static RiotAccount, RiotAccountRepositoryError>;
  async fn find_by_puuid(puuid: String) -> Result<&'static RiotAccount, RiotAccountRepositoryError>;
  async fn update(id: ID, riot_account: RiotAccount) -> Result<&'static RiotAccount, RiotAccountRepositoryError>;
  async fn delete(id: ID) -> Result<&'static RiotAccount, RiotAccountRepositoryError>;
  async fn is_expired(id: ID) -> Result<bool, RiotAccountRepositoryError>;
}