use reqwest::Client;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use domain::objects::agent::{Agent, ID};
use domain::objects::error::AgentRepositoryError;
use domain::traits::agent::AgentRepository;
use crate::client::db::entity::agent;
use crate::dto::agent::AgentResponseDto;

struct AgentRepositoryImpl {
  db: DbConn,
  client: Client
}

impl AgentRepositoryImpl {
  pub fn new(db: DbConn, client: Client) -> AgentRepositoryImpl {
    AgentRepositoryImpl {
      db,
      client
    }
  }
}

impl AgentRepository for AgentRepositoryImpl {
  async fn create(&self, riot_agent_id: String, name: String, role: String, icon: String) -> Result<Agent, AgentRepositoryError> {
    let db = self.db.clone();
    let agent = Agent::new_agent(riot_agent_id, name, role, icon);

    let pear = agent::ActiveModel {
      id: Set(agent.id().to_vec()),
      agent_id: Set(agent.riot_agent_id()),
      name: Set(agent.name()),
      role: Set(agent.role()),
      icon: Set(agent.icon()),
    };

    let result = agent::Entity::insert(pear).exec(&db).await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(AgentRepositoryError::DatabaseError)
    }

    match result {
      Ok(_) => Ok(agent),
      Err(_) => Err(AgentRepositoryError::InsertError)
    }
  }

  async fn find_by_id(&self, id: ID) -> Result<Agent, AgentRepositoryError> {
    let db = self.db.clone();
    let result = agent::Entity::find_by_id(id.to_vec())
      .one(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(AgentRepositoryError::DatabaseError)
    }

    match result {
      Ok(Some(pear)) => {
        let agent = Agent::new_agent_with_id(
          pear.id.try_into().unwrap(),
          pear.agent_id.to_string(),
          pear.name.to_string(),
          pear.role.to_string(),
          pear.icon.to_string()
        );
        Ok(agent)
      },
      Ok(None) => Err(AgentRepositoryError::NotFound),
      Err(_) => Err(AgentRepositoryError::FindError)
    }
  }

  async fn find_by_agent_id(&self, riot_agent_id: String) -> Result<Agent, AgentRepositoryError> {
    let db = self.db.clone();
    let result = agent::Entity::find()
      .filter(agent::Column::AgentId.contains(riot_agent_id.clone()))
      .one(&db)
      .await;
    match result {
      Ok(Some(pear)) => {
        match db.close().await {
          Ok(_) => {},
          Err(_) => return Err(AgentRepositoryError::DatabaseError)
        }

        let agent = Agent::new_agent_with_id(
          pear.id.try_into().unwrap(),
          pear.agent_id.to_string(),
          pear.name.to_string(),
          pear.role.to_string(),
          pear.icon.to_string()
        );
        Ok(agent)
      },
      Ok(None) => {
        let client = self.client.clone();
        let res = match client.get(format!("https://valorant-api.com/v1/agents/{}", riot_agent_id))
          .send()
          .await {
          Ok(res) => res,
          Err(_) => return Err(AgentRepositoryError::QueryError)
        };

        let agent_dto = match res.json::<AgentResponseDto>().await {
          Ok(body) => body.data,
          Err(_) => return Err(AgentRepositoryError::QueryError)
        };

        let agent = Agent::new_agent(
          agent_dto.agent_id,
          agent_dto.name,
          agent_dto.role.name,
          agent_dto.icon
        );

        let pear = agent::ActiveModel {
          id: Set(agent.id().to_vec()),
          agent_id: Set(agent.riot_agent_id()),
          name: Set(agent.name()),
          role: Set(agent.role()),
          icon: Set(agent.icon()),
        };

        let result = agent::Entity::insert(pear).exec(&db).await;

        match db.close().await {
          Ok(_) => {},
          Err(_) => return Err(AgentRepositoryError::DatabaseError)
        }

        match result {
          Ok(_) => Ok(agent),
          Err(_) => Err(AgentRepositoryError::InsertError)
        }
      },
      Err(_) => {
        match db.close().await {
          Ok(_) => {},
          Err(_) => return Err(AgentRepositoryError::DatabaseError)
        }

        Err(AgentRepositoryError::FindError)
      }
    }
  }

  async fn update(&self,  agent: Agent) -> Result<Agent, AgentRepositoryError> {
    let db = self.db.clone();
    let pear = agent::ActiveModel {
      id: Set(agent.id().to_vec()),
      agent_id: Set(agent.riot_agent_id()),
      name: Set(agent.name()),
      role: Set(agent.role()),
      icon: Set(agent.icon()),
    };
    
    let result = agent::Entity::update(pear).exec(&db).await;
    
    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(AgentRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(_) => Ok(agent),
      Err(_) => Err(AgentRepositoryError::UpdateError)
    }
  }

  async fn delete(&self, id: ID) -> Result<(), AgentRepositoryError> {
    let db = self.db.clone();
    let result = agent::Entity::delete_by_id(id.to_vec())
      .exec(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(AgentRepositoryError::DatabaseError)
    }

    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(AgentRepositoryError::DeleteError)
    }
  }

  async fn list(&self) -> Result<Vec<Agent>, AgentRepositoryError> {
    let db = self.db.clone();
    let result = agent::Entity::find().all(&db).await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(AgentRepositoryError::DatabaseError)
    }

    match result {
      Ok(pear) => {
        let agents = pear.iter().map(|p| {
          Agent::new_agent_with_id(
            p.clone().id.try_into().unwrap(),
            p.clone().agent_id.to_string(),
            p.clone().name.to_string(),
            p.clone().role.to_string(),
            p.clone().icon.to_string()
          )
        }).collect();
        Ok(agents)
      },
      Err(_) => Err(AgentRepositoryError::ListError)
    }
  }

  async fn list_by_role(&self, role: String) -> Result<Vec<Agent>, AgentRepositoryError> {
    let db = self.db.clone();
    let result = agent::Entity::find()
      .filter(agent::Column::Role.contains(role.clone()))
      .all(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(AgentRepositoryError::DatabaseError)
    }

    match result {
      Ok(pear) => {
        let agents = pear.iter().map(|p| {
          Agent::new_agent_with_id(
            p.clone().id.try_into().unwrap(),
            p.clone().agent_id.to_string(),
            p.clone().name.to_string(),
            p.clone().role.to_string(),
            p.clone().icon.to_string()
          )
        }).collect();
        Ok(agents)
      },
      Err(_) => Err(AgentRepositoryError::ListError)
    }
  }
}