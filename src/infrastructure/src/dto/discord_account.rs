use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscordApiAccountDto {
  pub id: String,
  pub username: String,
  pub avatar: String,
  pub email: Option<String>,
}