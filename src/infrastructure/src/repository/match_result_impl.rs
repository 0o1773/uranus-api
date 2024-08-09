use reqwest::Client;
use sea_orm::DbConn;
use domain::objects::error::MatchResultRepositoryError;
use domain::objects::match_result::{MatchResult, ID};
use domain::traits::match_result::MatchResultRepository;

struct MatchResultRepositoryImpl {
  db: DbConn,
  client: Client
}

impl MatchResultRepositoryImpl {
  pub fn new(db: DbConn, client: Client) -> MatchResultRepositoryImpl {
    MatchResultRepositoryImpl {
      db,
      client
    }
  }
}

impl MatchResultRepository for MatchResultRepositoryImpl {
  async fn create(&self, match_id: String, puuid: String) -> Result<MatchResult, MatchResultRepositoryError> {
    let db = self.db.clone();
    
    todo!()
  }

  async fn find_by_id(&self, id: ID) -> Result<MatchResult, MatchResultRepositoryError> {
    todo!()
  }

  async fn update(&self, match_result: MatchResult) -> Result<MatchResult, MatchResultRepositoryError> {
    todo!()
  }

  async fn delete(&self, id: ID) -> Result<(), MatchResultRepositoryError> {
    todo!()
  }

  async fn list(&self) -> Result<Vec<MatchResult>, MatchResultRepositoryError> {
    todo!()
  }
}