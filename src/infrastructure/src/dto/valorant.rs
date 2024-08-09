use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MatchDto {
  #[serde(rename = "matchInfo")]
  match_info: MatchInfoDto,
  players: Vec<PlayerDto>,
  coaches: Vec<CoachDto>,
  teams: Vec<TeamDto>,
  #[serde(rename = "roundResults")]
  rounds: Vec<RoundDto>,
}

#[derive(Serialize, Deserialize)]
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
// #[derive(Serialize, Deserialize)]
// struct PremierMatchInfoDto {
// }

#[derive(Serialize, Deserialize)]
pub struct PlayerDto {
  puuid: String,
  #[serde(rename = "gameName")]
  game_name: String,
  #[serde(rename = "tagLine")]
  tag_line: String,
  #[serde(rename = "teamId")]
  team_id: String,
  #[serde(rename = "partyId")]
  party_id: String,
  #[serde(rename = "characterId")]
  agent_id: String,
  #[serde(rename = "stats")]
  stats: PlayerStatsDto,
  #[serde(rename = "competitiveTier")]
  competitive_tier: i32,
  #[serde(rename = "playerCard")]
  player_card: String,
  #[serde(rename = "playerTitle")]
  player_title: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerStatsDto {
  score: i32,
  kills: i32,
  deaths: i32,
  assists: i32,
  #[serde(rename = "roundsPlayed")]
  rounds_played: i32,
  #[serde(rename = "playtimeMillis")]
  playtime: i32,
  #[serde(rename = "abilityCasts")]
  ability_casts: Option<AbilityCastsDto>,
}

#[derive(Serialize, Deserialize)]
pub struct AbilityCastsDto {
  #[serde(rename = "grenadeCasts")]
  grenade_casts: i32,
  #[serde(rename = "ability1Casts")]
  ability1_casts: i32,
  #[serde(rename = "ability2Casts")]
  ability2_casts: i32,
  #[serde(rename = "ultimateCasts")]
  ultimate_casts: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CoachDto {
  puuid: String,
  #[serde(rename = "teamId")]
  team_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct TeamDto {
  #[serde(rename = "teamId")]
  team_id: String,
  #[serde(rename = "won")]
  won: bool,
  #[serde(rename = "roundsPlayed")]
  rounds_played: i32,
  #[serde(rename = "roundsWon")]
  rounds_won: i32,
  #[serde(rename = "numPoints")]
  num_points: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RoundDto {
  #[serde(rename = "roundNum")]
  round_num: i32,
  #[serde(rename = "roundResult")]
  round_result: String,
  #[serde(rename = "roundCeremony")]
  round_ceremony: String,
  #[serde(rename = "winningTeam")]
  winning_team: String,
  #[serde(rename = "bombPlanter")]
  bomb_planter: String,
  #[serde(rename = "bombDefuser")]
  bomb_defuser: String,
  #[serde(rename = "plantRoundTime")]
  plant_round_time: i32,
  #[serde(rename = "plantPlayerLocations")]
  plant_player_locations: Option<Vec<PlayerLocationsDto>>,
  #[serde(rename = "plantLocation")]
  plant_location: Option<LocationDto>,
  #[serde(rename = "plantSite")]
  plant_site: Option<String>,
  #[serde(rename = "defuseRoundTime")]
  defuse_round_time: Option<i32>,
  #[serde(rename = "defusePlayerLocations")]
  defuse_player_locations: Option<Vec<PlayerLocationsDto>>,
  #[serde(rename = "defuseLocation")]
  defuse_location: Option<LocationDto>,
  #[serde(rename = "playerStats")]
  player_stats: Vec<PlayerRoundStatsDto>,
  #[serde(rename = "roundResultCode")]
  round_result_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerLocationsDto {
  puuid: String,
  #[serde(rename = "viewRadians")]
  view_radians: f32,
  location: LocationDto,
}

#[derive(Serialize, Deserialize)]
pub struct LocationDto {
  x: f32,
  y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerRoundStatsDto {
  puuid: String,
  kills: Vec<KillDto>,
  damage: Vec<DamageDto>,
  score: i32,
  economy: EconomyDto,
  ability: AbilityDto,
}

#[derive(Serialize, Deserialize)]
pub struct KillDto {
  #[serde(rename = "timeSinceGameStartMillis")]
  time_since_game_start: i32,
  #[serde(rename = "timeSinceRoundStartMillis")]
  time_since_round_start: i32,
  killer: String,
  victim: String,
  #[serde(rename = "victimLocation")]
  victim_location: LocationDto,
  #[serde(rename = "assistants")]
  assistants: Vec<String>,
  #[serde(rename = "playerLocations")]
  player_locations: Vec<PlayerLocationsDto>,
  #[serde(rename = "finishingDamage")]  
  finishing_damage: FinishingDamageDto,
}

#[derive(Serialize, Deserialize)]
pub struct FinishingDamageDto {
  #[serde(rename = "damageType")]
  damage_type: String,
  #[serde(rename = "isSecondaryFireMode")]
  is_secondary_fire_mode: bool,
  #[serde(rename = "damageItem")]
  damage_item: String,
}

#[derive(Serialize, Deserialize)]
pub struct DamageDto {
  receiver: String,
  #[serde(rename = "damage")]
  damage: i32,
  legshots: i32,
  bodyshots: i32,
  headshots: i32,
}

#[derive(Serialize, Deserialize)]
pub struct EconomyDto {
  #[serde(rename = "loadoutValue")]
  loadout_value: i32,
  weapon: String,
  armor: String,
  remaining: i32,
  spent: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AbilityDto {
  #[serde(rename = "grenadeEffects")]
  grenade_effects: String,
  #[serde(rename = "ability1Effects")]
  ability1_effects: String,
  #[serde(rename = "ability2Effects")]
  ability2_effects: String,
  #[serde(rename = "ultimateEffects")]
  ultimate_effects: String,
}