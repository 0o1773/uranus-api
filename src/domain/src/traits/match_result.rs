use crate::objects::{account, agent, map};
use crate::objects::error::MatchResultRepositoryError;
use crate::objects::match_result::{MatchResult, ID};
use crate::objects::role::Role;

pub struct SearchOption {
  pub agent_id: Option<agent::ID>,
  pub map_id: Option<map::ID>,
  pub role: Option<Role>,
}

pub trait MatchResultRepository: Sync + Send + 'static {
  async fn create(&self, match_id: String, puuid: String) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn find_by_id(&self, id: ID) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn update(&self, match_result: MatchResult) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn delete(&self, id: ID) -> Result<(), MatchResultRepositoryError>;
  async fn list(&self, account_id: account::ID, offset: Option<i32>, limit: Option<i32>, option: SearchOption) -> Result<Vec<MatchResult>, MatchResultRepositoryError>;
}