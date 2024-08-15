use crate::objects::account;
use crate::objects::error::MatchResultRepositoryError;
use crate::objects::match_result::{MatchResult, ID};

pub trait MatchResultRepository: Sync + Send + 'static {
  async fn create(&self, match_id: String, puuid: String) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn find_by_id(&self, id: ID) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn update(&self, match_result: MatchResult) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn delete(&self, id: ID) -> Result<(), MatchResultRepositoryError>;
  async fn list(&self, account_id: account::ID) -> Result<Vec<MatchResult>, MatchResultRepositoryError>;
}