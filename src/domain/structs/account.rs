use serde::{Deserialize, Serialize};

use crate::domain::{
    structs::discord_account::DiscordAccount,
    structs::player_info::PlayerInfo,
    structs::user::User,
};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Account {
    riot: PlayerInfo,
    user: User,
    discord: DiscordAccount,
}

impl Account {
    pub fn new(riot: PlayerInfo, user: User, discord: DiscordAccount) -> Self {
        Self { riot, user, discord }
    }
    pub fn get_riot(&self) -> &PlayerInfo {
        &self.riot
    }
    pub fn get_user(&self) -> &User {
        &self.user
    }
    pub fn get_discord(&self) -> &DiscordAccount {
        &self.discord
    }
}