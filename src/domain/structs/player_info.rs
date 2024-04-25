use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PlayerInfo {
    puuid: String,
    gameName: String,
    tagLine: String,
}

impl PlayerInfo {
    pub fn new(puuid: String, game_name: String, tag_line: String) -> Self {
        Self {
            puuid,
            gameName: game_name,
            tagLine: tag_line
        }
    }

    pub fn get_puuid(&self) -> &String { &self.puuid }
    pub fn get_game_name(&self) -> &String { &self.gameName }
    pub fn get_tag_line(&self) -> &String { &self.tagLine }
}