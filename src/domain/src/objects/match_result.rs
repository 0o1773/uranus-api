use ulid::Ulid;
use crate::objects::account::ID as AccountID;
use crate::objects::agent::ID as AgentID;
use crate::objects::map::ID as MapID;

pub type ID = [u8; 16];

#[derive(Clone)]
pub struct MatchResult {
  id: ID,
  match_id: String,
  puuid: String,
  account_id: AccountID,
  agent_id: AgentID,
  map_id: MapID,
  
  kill: i32,
  death: i32,
  assist: i32,
  hs_rate: f32,
  first_blood: i32,
  first_death: i32,
  
  combat_score: i32,
  damage: i32,
  
  played_rounds: i32,
  win_rounds: i32,
  win: bool,

  match_start_time: i64,
}

impl MatchResult {
  pub fn new_match_result(match_id: String, puuid: String, account_id: AccountID, agent_id: AgentID, map_id: MapID, kill: i32, death: i32, assist: i32, hs_rate: f32, first_blood: i32, first_death: i32, combat_score: i32, damage: i32, played_rounds: i32, win_rounds: i32, win: bool, match_start_time: i64) -> MatchResult {
    let ulid = Ulid::new();
    MatchResult {
      id: ulid.to_bytes(),
      match_id,
      puuid,
      account_id,
      agent_id,
      map_id,
      kill,
      death,
      assist,
      hs_rate,
      first_blood,
      first_death,
      combat_score,
      damage,
      played_rounds,
      win_rounds,
      win,
      match_start_time,
    }
  }
  
  pub fn new_match_result_with_id(id: ID, match_id: String, puuid: String, account_id: AccountID, agent_id: AgentID, map_id: MapID, kill: i32, death: i32, assist: i32, hs_rate: f32, first_blood: i32, first_death: i32, combat_score: i32, damage: i32, played_rounds: i32, win_rounds: i32, win: bool, match_start_time: i64) -> MatchResult {
    MatchResult {
      id,
      match_id,
      puuid,
      account_id,
      agent_id,
      map_id,
      kill,
      death,
      assist,
      hs_rate,
      first_blood,
      first_death,
      combat_score,
      damage,
      played_rounds,
      win_rounds,
      win,
      match_start_time,
    }
  }
  
  pub fn match_id(&self) -> String {
    self.match_id.clone()
  }
  
  pub fn puuid(&self) -> String {
    self.puuid.clone()
  }
  
  pub fn account_id(&self) -> AccountID {
    self.account_id.clone()
  }
  
  pub fn map_id(&self) -> MapID {
    self.map_id.clone()
  }
  
  pub fn kill(&self) -> i32 {
    self.kill
  }
  
  pub fn death(&self) -> i32 {
    self.death
  }
  
  pub fn assist(&self) -> i32 {
    self.assist
  }
  
  pub fn hs_rate(&self) -> f32 {
    self.hs_rate
  }
  
  pub fn first_blood(&self) -> i32 {
    self.first_blood
  }
  
  pub fn first_death(&self) -> i32 {
    self.first_death
  }
  
  pub fn combat_score(&self) -> i32 {
    self.combat_score
  }
  
  pub fn damage(&self) -> i32 {
    self.damage
  }
  
  pub fn played_rounds(&self) -> i32 {
    self.played_rounds
  }
  
  pub fn win_rounds(&self) -> i32 {
    self.win_rounds
  }
  
  pub fn win(&self) -> bool {
    self.win
  }

  pub fn match_start_time(&self) -> i64 {
    self.match_start_time
  }

  pub fn agent_id(&self) -> AgentID {
    self.agent_id.clone()
  }

  pub fn id(&self) -> ID {
    self.id.clone()
  }
  
  pub fn set_match_id(&mut self, match_id: String) {
    self.match_id = match_id;
  }
  
  pub fn set_puuid(&mut self, puuid: String) {
    self.puuid = puuid;
  }
  
  pub fn set_account_id(&mut self, account_id: AccountID) {
    self.account_id = account_id;
  }
  
  pub fn set_map_id(&mut self, map_id: MapID) {
    self.map_id = map_id;
  }
  
  pub fn set_kill(&mut self, kill: i32) {
    self.kill = kill;
  }
  
  pub fn set_death(&mut self, death: i32) {
    self.death = death;
  }
  
  pub fn set_assist(&mut self, assist: i32) {
    self.assist = assist;
  }
  
  pub fn set_hs_rate(&mut self, hs_rate: f32) {
    self.hs_rate = hs_rate;
  }
  
  pub fn set_first_blood(&mut self, first_blood: i32) {
    self.first_blood = first_blood;
  }
  
  pub fn set_first_death(&mut self, first_death: i32) {
    self.first_death = first_death;
  }
  
  pub fn set_combat_score(&mut self, combat_score: i32) {
    self.combat_score = combat_score;
  }
  
  pub fn set_damage(&mut self, damage: i32) {
    self.damage = damage;
  }
  
  pub fn set_played_rounds(&mut self, played_rounds: i32) {
    self.played_rounds = played_rounds;
  }
  
  pub fn set_win_rounds(&mut self, win_rounds: i32) {
    self.win_rounds = win_rounds;
  }
  
  pub fn set_win(&mut self, win: bool) {
    self.win = win;
  }

  pub fn set_match_start_time(&mut self, match_start_time: i64) {
    self.match_start_time = match_start_time;
  }

  pub fn set_agent_id(&mut self, agent_id: AgentID) {
    self.agent_id = agent_id;
  }

  pub fn set_id(&mut self, id: ID) {
    self.id = id;
  }
}