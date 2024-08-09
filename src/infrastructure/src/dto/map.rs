use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MapResponseDto {
  pub status: i32,
  pub data: Vec<MapDto>,
}

#[derive(Serialize, Deserialize)]
pub struct MapDto {
  #[serde(rename = "mapUrl")]
  pub map_id: String,
  #[serde(rename = "displayName")]
  pub name: String,
  #[serde(rename = "premierBackgroundImage")]
  pub image: String,
}