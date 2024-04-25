use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct DiscordAccount {
    id: String,
}

impl DiscordAccount {
    pub fn new(id: String) -> Self {
        Self { id }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
}