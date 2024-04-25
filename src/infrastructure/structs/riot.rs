use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RiotMatchInfo {
    #[serde(alias="matchId")]
    match_id: String,
    #[serde(alias="mapId")]
    map_id: String,
    #[serde(alias="gameLengthMillis")]
    game_length_millis: i64,
    #[serde(alias="gameStartMillis")]
    game_start_millis: i64,
    #[serde(alias="provisioningFlowId")]
    provisioning_flow_id: String,
    #[serde(alias="isCompleted")]
    is_completed: bool,
    #[serde(alias="customGameName")]
    custom_game_name: String,
    #[serde(alias="queueId")]
    queue_id: String,
    #[serde(alias="gameMode")]
    game_mode: String,
    #[serde(alias="isRanked")]
    is_ranked: bool,
    #[serde(alias="seasonId")]
    season_id: String,
}
impl RiotMatchInfo {
    pub fn new(match_id: String, map_id: String, game_length_millis: i64, game_start_millis: i64, provisioning_flow_id: String, is_completed: bool, custom_game_name: String, queue_id: String, game_mode: String, is_ranked: bool, season_id: String) -> Self {
        Self {
            match_id,
            map_id,
            game_length_millis,
            game_start_millis,
            provisioning_flow_id,
            is_completed,
            custom_game_name,
            queue_id,
            game_mode,
            is_ranked,
            season_id
        }
    }
    pub fn get_match_id(&self) -> &String { &self.match_id }
    pub fn get_map_id(&self) -> &String { &self.map_id }
    pub fn get_game_length_millis(&self) -> &i64 { &self.game_length_millis }
    pub fn get_game_start_millis(&self) -> &i64 { &self.game_start_millis }
    pub fn get_provisioning_flow_id(&self) -> &String { &self.provisioning_flow_id }
    pub fn get_is_completed(&self) -> &bool { &self.is_completed }
    pub fn get_custom_game_name(&self) -> &String { &self.custom_game_name }
    pub fn get_queue_id(&self) -> &String { &self.queue_id }
    pub fn get_game_mode(&self) -> &String { &self.game_mode }
    pub fn get_is_ranked(&self) -> &bool { &self.is_ranked }
    pub fn get_season_id(&self) -> &String { &self.season_id }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RiotPlayer {
    puuid: String,
    #[serde(alias="gameName")]
    game_name: String,
    #[serde(alias="tagLine")]
    tag_line: String,
    #[serde(alias="teamId")]
    team_id: String,
    #[serde(alias="partyId")]
    party_id: String,
    #[serde(alias="characterId")]
    character_id: String,
    stats: RiotPlayerStats,
    #[serde(alias="competitiveTier")]
    competitive_tier: i32,
    #[serde(alias="playerCard")]
    player_card: String,
    #[serde(alias="playerTitle")]
    player_title: String,
}

impl RiotPlayer {
    pub fn new(puuid: String, game_name: String, tag_line: String, team_id: String, party_id: String, character_id: String, stats: RiotPlayerStats, competitive_tier: i32, player_card: String, player_title: String) -> Self {
        Self {
            puuid,
            game_name,
            tag_line,
            team_id,
            party_id,
            character_id,
            stats,
            competitive_tier,
            player_card,
            player_title
        }
    }
    pub fn get_puuid(&self) -> &String { &self.puuid }
    pub fn get_game_name(&self) -> &String { &self.game_name }
    pub fn get_tag_line(&self) -> &String { &self.tag_line }
    pub fn get_team_id(&self) -> &String { &self.team_id }
    pub fn get_party_id(&self) -> &String { &self.party_id }
    pub fn get_character_id(&self) -> &String { &self.character_id }
    pub fn get_stats(&self) -> &RiotPlayerStats { &self.stats }
    pub fn get_competitive_tier(&self) -> &i32 { &self.competitive_tier }
    pub fn get_player_card(&self) -> &String { &self.player_card }
    pub fn get_player_title(&self) -> &String { &self.player_title }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RiotPlayerStats {
    score: i32,
    #[serde(alias="roundsPlayed")]
    rounds_played: i32,
    kills: i32,
    deaths: i32,
    assists: i32,
    #[serde(alias="playtimeMillis")]
    playtime_millis: i32,
}

impl RiotPlayerStats {
    pub fn new(score: i32, rounds_played: i32, kills: i32, deaths: i32, assists: i32, playtime_millis: i32) -> Self {
        Self {
            score,
            rounds_played,
            kills,
            deaths,
            assists,
            playtime_millis
        }
    }
    pub fn get_score(&self) -> &i32 { &self.score }
    pub fn get_rounds_played(&self) -> &i32 { &self.rounds_played }
    pub fn get_kills(&self) -> &i32 { &self.kills }
    pub fn get_deaths(&self) -> &i32 { &self.deaths }
    pub fn get_assists(&self) -> &i32 { &self.assists }
    pub fn get_playtime_millis(&self) -> &i32 { &self.playtime_millis }
}

#[derive(Deserialize, Serialize)]
pub struct RiotTeam {
    #[serde(alias="teamId")]
    team_id: String,
    won: bool,
    #[serde(alias="roundsPlayed")]
    rounds_played: i32,
    #[serde(alias="roundsWon")]
    rounds_won: i32,
    #[serde(alias="numPoints")]
    num_points: Option<i32>
}

impl RiotTeam {
    pub fn new(team_id: String, won: bool, rounds_played: i32, rounds_won: i32, num_points: Option<i32>) -> Self {
        Self {
            team_id,
            won,
            rounds_played,
            rounds_won,
            num_points
        }
    }
    pub fn get_team_id(&self) -> &String { &self.team_id }
    pub fn get_won(&self) -> &bool { &self.won }
    pub fn get_rounds_played(&self) -> &i32 { &self.rounds_played }
    pub fn get_rounds_won(&self) -> &i32 { &self.rounds_won }
    pub fn get_num_points(&self) -> &Option<i32> { &self.num_points }
}

#[derive(Deserialize, Serialize)]
pub struct RiotRoundResult {
    #[serde(alias="roundNum")]
    round_num: i32,
    #[serde(alias="roundResult")]
    round_result: String,
    #[serde(alias="roundCeremony")]
    round_ceremony: String,
    #[serde(alias="winningTeam")]
    winning_team: String,
    #[serde(alias="bombPlanter")]
    bomb_planter: Option<String>,
    #[serde(alias="bombDefuser")]
    bomb_defuser: Option<String>,
    #[serde(alias="plantRoundTime")]
    plant_round_time: i32,
    #[serde(alias="playerStats")]
    player_stats: Vec<RiotPlayerRoundStats>
}

impl RiotRoundResult {
    pub fn new(round_num: i32, round_result: String, round_ceremony: String, winning_team: String, bomb_planter: Option<String>, bomb_defuser: Option<String>, plant_round_time: i32, player_stats: Vec<RiotPlayerRoundStats>) -> Self {
        Self {
            round_num,
            round_result,
            round_ceremony,
            winning_team,
            bomb_planter,
            bomb_defuser,
            plant_round_time,
            player_stats,
        }
    }
    pub fn get_round_num(&self) -> &i32 { &self.round_num }
    pub fn get_round_result(&self) -> &String { &self.round_result }
    pub fn get_round_ceremony(&self) -> &String { &self.round_ceremony }
    pub fn get_winning_team(&self) -> &String { &self.winning_team }
    pub fn get_bomb_planter(&self) -> &Option<String> { &self.bomb_planter }
    pub fn get_bomb_defuser(&self) -> &Option<String> { &self.bomb_defuser }
    pub fn get_plant_round_time(&self) -> &i32 { &self.plant_round_time }
    pub fn get_player_stats(&self) -> &Vec<RiotPlayerRoundStats> { &self.player_stats }
}

#[derive(Deserialize, Serialize)]
pub struct RiotPlayerRoundStats {
    #[serde(alias="puuid")]
    puuid: String,
    #[serde(alias="kills")]
    kill: Vec<RiotKill>,
    damage: Vec<RiotDamage>,
    score: i32,
}

impl RiotPlayerRoundStats {
    pub fn new(puuid: String, kill: Vec<RiotKill>, damage: Vec<RiotDamage>, score: i32) -> Self {
        Self {
            puuid,
            kill,
            damage,
            score
        }
    }
    pub fn get_puuid(&self) -> &String { &self.puuid }
    pub fn get_kills(&self) -> &Vec<RiotKill> { &self.kill }
    pub fn get_damage(&self) -> &Vec<RiotDamage> { &self.damage }
    pub fn get_score(&self) -> &i32 { &self.score }

}

#[derive(Deserialize, Serialize)]
pub struct RiotKill {
    #[serde(alias="timeSinceGameStartMillis")]
    time_since_game_start_millis: i32,
    #[serde(alias="timeSinceRoundStartMillis")]
    time_since_round_start_millis: i32,
    #[serde(alias="killer")]
    killer: String,
    #[serde(alias="victim")]
    victim: String,
    #[serde(alias="victimLocation")]
    victim_location: RiotLocation,
    #[serde(alias="assistants")]
    assistants: Vec<String>,
    #[serde(alias="playerLocations")]
    player_locations: Vec<RiotPlayerLocations>,
    #[serde(alias="finishingDamage")]
    finishing_damage: RiotFinishingDamage,
}

impl RiotKill {
    pub fn new(time_since_game_start_millis: i32, time_since_round_start_millis: i32, killer: String, victim: String, victim_location: RiotLocation, assister: Vec<String>, player_locations: Vec<RiotPlayerLocations>, finishing_damage: RiotFinishingDamage) -> Self {
        Self {
            time_since_game_start_millis,
            time_since_round_start_millis,
            killer,
            victim,
            victim_location,
            assistants: assister,
            player_locations,
            finishing_damage
        }
    }
    pub fn get_time_since_game_start_millis(&self) -> &i32 { &self.time_since_game_start_millis }
    pub fn get_time_since_round_start_millis(&self) -> &i32 { &self.time_since_round_start_millis }
    pub fn get_killer(&self) -> &String { &self.killer }
    pub fn get_victim(&self) -> &String { &self.victim }
    pub fn get_victim_location(&self) -> &RiotLocation { &self.victim_location }
    pub fn get_assistants(&self) -> &Vec<String> { &self.assistants }
}

#[derive(Deserialize, Serialize)]
pub struct RiotLocation {
    x: f64,
    y: f64
}

impl RiotLocation {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn get_x(&self) -> &f64 { &self.x }
    pub fn get_y(&self) -> &f64 { &self.y }
}

#[derive(Deserialize, Serialize)]
pub struct RiotPlayerLocations {
    puuid: String,
    #[serde(alias="viewRadians")]
    view_radians: f64,
    location: RiotLocation
}

impl RiotPlayerLocations {
    pub fn new(puuid: String, view_radians: f64, location: RiotLocation) -> Self {
        Self { puuid, view_radians, location }
    }
    pub fn get_puuid(&self) -> &String { &self.puuid }
    pub fn get_view_radians(&self) -> &f64 { &self.view_radians }
    pub fn get_location(&self) -> &RiotLocation { &self.location }
}

#[derive(Deserialize, Serialize)]
pub struct RiotFinishingDamage {
    #[serde(alias="damageType")]
    damage_type: String,
    #[serde(alias="damageItem")]
    damage_item: String,
    #[serde(alias="isSecondaryFireMode")]
    is_secondary_fire_mode: bool
}

impl RiotFinishingDamage {
    pub fn new(damage_type: String, damage_item: String, is_secondary_fire_mode: bool) -> Self {
        Self { damage_type, damage_item, is_secondary_fire_mode }
    }
    pub fn get_damage_type(&self) -> &String { &self.damage_type }
    pub fn get_damage_item(&self) -> &String { &self.damage_item }
    pub fn get_is_secondary_fire_mode(&self) -> &bool { &self.is_secondary_fire_mode }
}

#[derive(Deserialize, Serialize)]
pub struct RiotDamage {
    receiver: String,
    damage: i32,
    legshots: i32,
    bodyshots: i32,
    headshots: i32
}

impl RiotDamage {
    pub fn new(receiver: String, damage: i32, legshots: i32, bodyshots: i32, headshots: i32) -> Self {
        Self { receiver, damage, legshots, bodyshots, headshots }
    }
    pub fn get_receiver(&self) -> &String { &self.receiver }
    pub fn get_damage(&self) -> &i32 { &self.damage }
    pub fn get_legshots(&self) -> &i32 { &self.legshots }
    pub fn get_bodyshots(&self) -> &i32 { &self.bodyshots }
    pub fn get_headshots(&self) -> &i32 { &self.headshots }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RiotMatchlistEntry {
    #[serde(alias="matchId")]
    match_id: String,
    #[serde(alias="gameStartTimeMillis")]
    game_start_time_millis: i64,
    #[serde(alias="queueId")]
    queue: String,
}

impl RiotMatchlistEntry {
    pub fn new(match_id: String, game_start_time_millis: i64, queue: String) -> Self {
        Self { match_id, game_start_time_millis, queue }
    }
    pub fn get_match_id(&self) -> &String { &self.match_id }
    pub fn get_game_start_time_millis(&self) -> &i64 { &self.game_start_time_millis }
    pub fn get_queue(&self) -> &String { &self.queue }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RiotMatchList {
    puuid: String,
    history: Vec<RiotMatchlistEntry>
}

impl RiotMatchList {
    pub fn new(puuid: String, history: Vec<RiotMatchlistEntry>) -> Self {
        Self { puuid, history }
    }
    pub fn get_puuid(&self) -> &String { &self.puuid }
    pub fn get_history(&self) -> &Vec<RiotMatchlistEntry> { &self.history }
}


#[derive(Deserialize, Serialize)]
pub struct RiotMatch {
    #[serde(alias="matchInfo")]
    pub match_info: RiotMatchInfo,
    pub players: Vec<RiotPlayer>,
    pub teams: Vec<RiotTeam>,
    #[serde(alias="roundResults")]
    pub round_results: Vec<RiotRoundResult>,
}

#[derive(Deserialize, Serialize)]
pub struct RiotAccount {
    puuid: String,
    #[serde(alias="gameName")]
    game_name: String,
    #[serde(alias="tagLine")]
    tag_line: String,
}

impl RiotAccount {
    pub fn new(puuid: String, game_name: String, tag_line: String) -> Self {
        Self { puuid, game_name, tag_line }
    }
    pub fn get_puuid(&self) -> &String { &self.puuid }
    pub fn get_game_name(&self) -> &String { &self.game_name }
    pub fn get_tag_line(&self) -> &String { &self.tag_line }
}