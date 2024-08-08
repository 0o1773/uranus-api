use crate::objects::error::MatchResultRepositoryError;
use crate::objects::match_result::{MatchResult, ID};

pub trait MatchResultRepository: Sync + Send + 'static {
  async fn create(&self, match_id: String, kill: i32, death: i32, assist: i32, win: bool) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn find_by_id(&self, id: ID) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn update(&self, id: ID, match_result: MatchResult) -> Result<MatchResult, MatchResultRepositoryError>;
  async fn delete(&self, id: ID) -> Result<(), MatchResultRepositoryError>;
  async fn list(&self) -> Result<Vec<MatchResult>, MatchResultRepositoryError>;
}