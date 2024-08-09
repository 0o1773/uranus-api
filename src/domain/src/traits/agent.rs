use crate::objects::agent::{Agent, ID};
use crate::objects::error::AgentRepositoryError;

pub trait AgentRepository: Send + Sync + 'static {
  async fn create(&self, riot_agent_id: String, name: String, role: String, icon: String) -> Result<Agent, AgentRepositoryError>;
  async fn find_by_id(&self, id: ID) -> Result<Agent, AgentRepositoryError>;
  async fn find_by_agent_id(&self, riot_agent_id: String) -> Result<Agent, AgentRepositoryError>;
  async fn update(&self, agent: Agent) -> Result<Agent, AgentRepositoryError>;
  async fn delete(&self, id: ID) -> Result<(), AgentRepositoryError>;
  async fn list(&self) -> Result<Vec<Agent>, AgentRepositoryError>;
  async fn list_by_role(&self, role: String) -> Result<Vec<Agent>, AgentRepositoryError>;
}