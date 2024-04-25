use tonic::async_trait;
use crate::domain::structs::errors::DomainError;
use crate::domain::structs::r#match::Match;

#[async_trait]
pub trait MatchRepository: Sync + Send + 'static{
    async fn fetch_match(&self, puuid: &str, match_id: &str) -> Result<Match, DomainError>;
}

#[async_trait]
pub trait HistoryRepository: Sync + Send + 'static {
    async fn fetch_matches(&self, puuid: &str, max: &i32, queue: Option<String>) -> Result<Vec<Match>, DomainError>;
}