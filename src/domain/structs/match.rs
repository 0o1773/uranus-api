use serde::{Deserialize, Serialize};
use crate::domain:: {
    structs::assets::{Agent, TierInfo},
    structs::player_info::PlayerInfo,
    structs::round::Round,
    structs::stats::{RecentStats, Stats},
};
use crate::domain::structs::assets::Map;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MatchInfo {
    queue: String,
    isWin: bool,
    startTimestamp: i64,
    winRoundCount: i32,
    totalRoundCount: i32,
    map: Map
}

impl MatchInfo {
    pub fn new(queue: &String, is_win: &bool, start_timestamp: &i64, win_round_count: &i32, total_round_count: &i32, map: Map) -> Self {
        Self {
            queue: queue.to_string(),
            isWin: is_win.clone(),
            startTimestamp: start_timestamp.clone(),
            winRoundCount: win_round_count.clone(),
            totalRoundCount: total_round_count.clone(),
            map,
        }
    }

    pub fn get_queue(&self) -> &String { &self.queue }
    pub fn get_is_win(&self) -> &bool { &self.isWin }
    pub fn get_start_timestamp(&self) -> &i64 { &self.startTimestamp }
    pub fn get_win_round_count(&self) -> &i32 { &self.winRoundCount }
    pub fn get_total_round_count(&self) -> &i32 { &self.totalRoundCount }
    pub fn get_map(&self) -> &Map { &self.map }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MatchPlayer {
    playerInfo: PlayerInfo,
    tier: TierInfo,
    stats: Stats,
    recentStats: RecentStats,
    agent: Agent,
    teamId: String, 
}

impl MatchPlayer {
    pub fn new(player_info: PlayerInfo, tier: TierInfo, stats: Stats, recent_stats: RecentStats, agent: Agent, team_id: &String) -> Self {
        Self {
            playerInfo: player_info,
            tier,
            stats,
            recentStats: recent_stats,
            agent,
            teamId: team_id.clone(),
        }
    }

    pub fn get_player_info(&self) -> &PlayerInfo { &self.playerInfo }
    pub fn get_tier(&self) -> &TierInfo { &self.tier }
    pub fn get_stats(&self) -> &Stats { &self.stats }
    pub fn get_recent_stats(&self) -> &RecentStats { &self.recentStats }
    pub fn get_agent(&self) -> &Agent { &self.agent }
    pub fn get_team_id(&self) -> &String { &self.teamId }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Match {
    matchInfo: MatchInfo,
    players: Vec<MatchPlayer>,
    rounds: Vec<Round>,
}

impl Match {
    pub fn new(match_info: MatchInfo, players: Vec<MatchPlayer>, rounds: Vec<Round>) -> Self {
        Self {
            matchInfo: match_info,
            players,
            rounds
        }
    }
    pub fn get_match_info(&self) -> &MatchInfo { &self.matchInfo }
    pub fn get_players(&self) -> &Vec<MatchPlayer> { &self.players }
    pub fn get_rounds(&self) -> &Vec<Round> { &self.rounds }
}