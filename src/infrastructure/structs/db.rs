use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PgTierInfo {
    pub tier: String,
    pub tier_name: String,
    pub background_color: String,
    pub color: String,
    pub division: String,
    pub division_name: String,
    pub large_icon: String,
    pub small_icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PgMap {
    pub id: String,
    pub name: String,
    pub image: String,
    pub asset_name: String,
    pub asset_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PgAgent {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub log_image: String,
}