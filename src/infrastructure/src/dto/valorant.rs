use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchDto {
  #[serde(rename = "matchInfo")]
  pub match_info: MatchInfoDto,
  pub players: Vec<PlayerDto>,
  pub coaches: Vec<CoachDto>,
  pub teams: Vec<TeamDto>,
  #[serde(rename = "roundResults")]
  pub rounds: Vec<RoundDto>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchInfoDto {
  #[serde(rename = "matchId")]
  pub match_id: String,
  #[serde(rename = "mapId")]
  pub map_id: String,
  #[serde(rename = "gameLengthMillis")]
  pub game_length: String,
  #[serde(rename = "gameStartMillis")]
  pub start_time: String,
  #[serde(rename = "provisioningFlowId")]
  pub provisioning_flow_id: String,
  #[serde(rename = "isCompleted")]
  pub is_completed: bool,
  #[serde(rename = "customGameName")]
  pub custom_game_name: String,
  #[serde(rename = "queueId")]
  pub queue_id: String,
  #[serde(rename = "isRanked")]
  pub is_ranked: bool,
  #[serde(rename = "seasonId")]
  pub season_id: String,
}
//
// #[derive(Serialize, Deserialize, Clone)]
// struct PremierMatchInfoDto {
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerDto {
  pub puuid: String,
  #[serde(rename = "gameName")]
  pub game_name: String,
  #[serde(rename = "tagLine")]
  pub tag_line: String,
  #[serde(rename = "teamId")]
  pub team_id: String,
  #[serde(rename = "partyId")]
  pub party_id: String,
  #[serde(rename = "characterId")]
  pub agent_id: String,
  #[serde(rename = "stats")]
  pub stats: PlayerStatsDto,
  #[serde(rename = "competitiveTier")]
  pub competitive_tier: i32,
  #[serde(rename = "playerCard")]
  pub player_card: String,
  #[serde(rename = "playerTitle")]
  pub player_title: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerStatsDto {
  pub score: i32,
  pub kills: i32,
  pub deaths: i32,
  pub assists: i32,
  #[serde(rename = "roundsPlayed")]
  pub rounds_played: i32,
  #[serde(rename = "playtimeMillis")]
  pub playtime: i32,
  #[serde(rename = "abilityCasts")]
  pub ability_casts: Option<AbilityCastsDto>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AbilityCastsDto {
  #[serde(rename = "grenadeCasts")]
  pub grenade_casts: i32,
  #[serde(rename = "ability1Casts")]
  pub ability1_casts: i32,
  #[serde(rename = "ability2Casts")]
  pub ability2_casts: i32,
  #[serde(rename = "ultimateCasts")]
  pub ultimate_casts: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CoachDto {
  pub puuid: String,
  #[serde(rename = "teamId")]
  pub team_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeamDto {
  #[serde(rename = "teamId")]
  pub team_id: String,
  #[serde(rename = "won")]
  pub won: bool,
  #[serde(rename = "roundsPlayed")]
  pub rounds_played: i32,
  #[serde(rename = "roundsWon")]
  pub rounds_won: i32,
  #[serde(rename = "numPoints")]
  pub num_points: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoundDto {
  #[serde(rename = "roundNum")]
  pub round_num: i32,
  #[serde(rename = "roundResult")]
  pub round_result: String,
  #[serde(rename = "roundCeremony")]
  pub round_ceremony: String,
  #[serde(rename = "winningTeam")]
  pub winning_team: String,
  #[serde(rename = "bombPlanter")]
  pub bomb_planter: String,
  #[serde(rename = "bombDefuser")]
  pub bomb_defuser: String,
  #[serde(rename = "plantRoundTime")]
  pub plant_round_time: i32,
  #[serde(rename = "plantPlayerLocations")]
  pub plant_player_locations: Option<Vec<PlayerLocationsDto>>,
  #[serde(rename = "plantLocation")]
  pub plant_location: Option<LocationDto>,
  #[serde(rename = "plantSite")]
  pub plant_site: Option<String>,
  #[serde(rename = "defuseRoundTime")]
  pub defuse_round_time: Option<i32>,
  #[serde(rename = "defusePlayerLocations")]
  pub defuse_player_locations: Option<Vec<PlayerLocationsDto>>,
  #[serde(rename = "defuseLocation")]
  pub defuse_location: Option<LocationDto>,
  #[serde(rename = "playerStats")]
  pub player_stats: Vec<PlayerRoundStatsDto>,
  #[serde(rename = "roundResultCode")]
  pub round_result_code: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerLocationsDto {
  pub puuid: String,
  #[serde(rename = "viewRadians")]
  pub view_radians: f32,
  pub location: LocationDto,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LocationDto {
  pub x: f32,
  pub y: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerRoundStatsDto {
 pub puuid: String,
 pub kills: Vec<KillDto>,
 pub damage: Vec<DamageDto>,
 pub score: i32,
 pub economy: EconomyDto,
 pub ability: AbilityDto,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KillDto {
  #[serde(rename = "timeSinceGameStartMillis")]
  pub time_since_game_start: i32,
  #[serde(rename = "timeSinceRoundStartMillis")]
  pub time_since_round_start: i32,
  pub killer: String,
  pub victim: String,
  #[serde(rename = "victimLocation")]
  pub victim_location: LocationDto,
  #[serde(rename = "assistants")]
  pub assistants: Vec<String>,
  #[serde(rename = "playerLocations")]
  pub player_locations: Vec<PlayerLocationsDto>,
  #[serde(rename = "finishingDamage")]
  pub finishing_damage: FinishingDamageDto,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FinishingDamageDto {
  #[serde(rename = "damageType")]
  pub damage_type: String,
  #[serde(rename = "isSecondaryFireMode")]
  pub is_secondary_fire_mode: bool,
  #[serde(rename = "damageItem")]
  pub damage_item: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DamageDto {
  pub receiver: String,
  #[serde(rename = "damage")]
  pub damage: i32,
  pub legshots: i32,
  pub bodyshots: i32,
  pub headshots: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EconomyDto {
  #[serde(rename = "loadoutValue")]
  pub loadout_value: i32,
  pub weapon: String,
  pub armor: String,
  pub remaining: i32,
  pub spent: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AbilityDto {
  #[serde(rename = "grenadeEffects")]
  pub grenade_effects: String,
  #[serde(rename = "ability1Effects")]
  pub ability1_effects: String,
  #[serde(rename = "ability2Effects")]
  pub ability2_effects: String,
  #[serde(rename = "ultimateEffects")]
  pub ultimate_effects: String,
}