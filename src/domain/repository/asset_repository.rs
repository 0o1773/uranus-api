use tonic::async_trait;
use crate::domain::structs::assets::{Agent, Map, TierInfo};

#[async_trait]
pub trait AssetRepository: Sync + Send + 'static {
    async fn fetch_agent(&self, agent_id: &str) -> Agent;
    async fn fetch_map(&self, map_id: &str) -> Map;
    async fn fetch_tier(&self, tier: &i32) -> TierInfo;
}