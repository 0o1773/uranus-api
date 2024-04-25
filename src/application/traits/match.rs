use tonic::async_trait;
use crate::domain::structs::{
    errors::DomainError,
    r#match::Match,
};

#[async_trait]
pub trait MatchApplications: Sync + Send + 'static {
    async fn fetch_matches(&self, puuid: &str, max: &i32, queue: Option<String>) -> Result<Vec<Match>, DomainError>;
    async fn fetch_match(&self, puuid: &str, match_id: &str) -> Result<Match, DomainError>;
}