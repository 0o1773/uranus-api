use sea_orm::{DatabaseConnection, EntityTrait};
use tonic::async_trait;
use crate::domain::repository::asset_repository::AssetRepository;
use crate::domain::structs::assets::{Agent, Map, TierInfo};
use crate::entities::{
    prelude::{
        Agent as AgentEntity,
        Map as MapEntity,
        Tier as TierEntity
    }};

#[derive(Clone)]
pub struct PgAssetRepository {
    pg_client: DatabaseConnection,
}

impl PgAssetRepository {
    pub fn new(pg_client: &DatabaseConnection) -> Self {
        Self {
            pg_client: pg_client.clone(),
        }
    }
}

#[async_trait]
impl AssetRepository for PgAssetRepository {
    async fn fetch_agent(&self, agent_id: &str) -> Agent {
        let agents = match AgentEntity::find_by_id(agent_id).one(&self.pg_client).await {
            Ok(agent) => agent,
            Err(e) => {
                println!("{:?}", e);
                return Agent::default();
            }
        };

        return if let Some(agent) = agents {
            Agent::new(agent.id, agent.name, agent.icon, agent.log_img, agent.role)
        } else {
            Agent::default()
        }
    }

    async fn fetch_map(&self, map_id: &str) -> Map {
        
        let maps = match MapEntity::find_by_id(map_id).one(&self.pg_client).await {
            Ok(map) => map,
            Err(e) => {
                println!("{:?}", e);
                return Map::default();
            }
        };

        return if let Some(map) = maps {
            Map::new(map.id, map.name)
        } else {
            Map::default()
        }
    }

    async fn fetch_tier(&self, tier_int: &i32) -> TierInfo {
        let tiers = match TierEntity::find_by_id(*tier_int).one(&self.pg_client).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                return TierInfo::default();
            }
        };

        return if let Some(tier) = tiers {
            TierInfo::new(tier.tier, tier.tier_name, tier.division_name, tier.icon, tier.color)
        } else {
            TierInfo::default()
        }
    }
}