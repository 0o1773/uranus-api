use crate::objects::{account, agent, map, match_result};
use crate::objects::error::MatchListRepositoryError;
use crate::objects::match_list::MatchList;
use crate::objects::match_result::MatchResult;
use crate::objects::role::Role;

pub struct SearchOption {
  agent_id: Option<agent::ID>,
  map_id: Option<map::ID>,
  role: Option<Role>,
}

pub trait MatchListRepository: Sync + Send + 'static  {
  async fn create(&self, matches: Vec<MatchResult>) -> Result<(), MatchListRepositoryError>;
  async fn list(&self, account_id: account::ID, offset: Option<i32>, limit: Option<i32>, option: Option<SearchOption>) -> Result<MatchList, MatchListRepositoryError>;
  async fn find_by_id(&self, id: match_result::ID) -> Result<MatchResult, MatchListRepositoryError>;
  async fn update(&self, match_result: MatchResult) -> Result<MatchResult, MatchListRepositoryError>;
  async fn remove(&self, id: match_result::ID) -> Result<(), MatchListRepositoryError>;
  async fn delete_all(&self) -> Result<(), MatchListRepositoryError>;
}