use tonic::async_trait;
use crate::application::interactor::MatchService;
use crate::application::traits::r#match::MatchApplications;
use crate::domain::repository::match_repository::{HistoryRepository, MatchRepository};
use crate::domain::structs::errors::DomainError;
use crate::domain::structs::r#match::Match;

#[async_trait]
impl<MR, HR> MatchApplications for MatchService<MR, HR>
where
    MR: MatchRepository,
    HR: HistoryRepository,
{
    async fn fetch_matches(&self, puuid: &str, max: &i32, queue: Option<String>) -> Result<Vec<Match>, DomainError> {
        self.history_repository.fetch_matches(puuid, max, queue).await
     }

    async fn fetch_match(&self, puuid: &str, match_id: &str) -> Result<Match, DomainError> {
        self.match_repository.fetch_match(puuid, match_id).await
    }
}