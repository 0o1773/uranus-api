use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AgentsResponseDto {
  pub status: i32,
  pub data: Vec<AgentDto>,
}

#[derive(Serialize, Deserialize)]
pub struct AgentResponseDto {
  pub status: i32,
  pub data: AgentDto,
}


#[derive(Serialize, Deserialize)]
pub struct AgentDto {
  #[serde(rename = "uuid")]
  pub agent_id: String,
  #[serde(rename = "displayName")]
  pub name: String,
  #[serde(rename = "role")]
  pub role: AgentRoleDto,
  #[serde(rename = "displayIcon")]
  pub icon: String,
}

#[derive(Serialize, Deserialize)]
pub struct AgentRoleDto {
  pub uuid: String,
  #[serde(rename = "displayName")]
  pub name: String,
}