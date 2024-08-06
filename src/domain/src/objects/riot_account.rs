use ulid::Ulid;

pub type ID = [u8; 16];

pub struct RiotAccount {
  id: [u8; 16],
  puuid: String,
  game_name: String,
  tag_line: String,
  player_card: String,
  created_at: i64,
  updated_at: i64,
}

impl RiotAccount {
  fn new_riot_account(puuid: String, game_name: String, tag_line: String, player_card: String, created_at: i64, updated_at: i64) -> RiotAccount {
    let ulid = Ulid::new();

    RiotAccount {
      id: ulid.to_bytes(),
      puuid,
      game_name,
      tag_line,
      player_card,
      created_at,
      updated_at,
    }
  }

  pub fn new_riot_account_with_id(id: [u8; 16], puuid: String, game_name: String, tag_line: String, player_card: String, created_at: i64, updated_at: i64) -> RiotAccount {
    RiotAccount {
      id,
      puuid,
      game_name,
      tag_line,
      player_card,
      created_at,
      updated_at,
    }
  }

  pub fn puuid(&self) -> String {
    self.puuid.clone()
  }

  pub fn game_name(&self) -> String {
    self.game_name.clone()
  }

  pub fn tag_line(&self) -> String {
    self.tag_line.clone()
  }

  pub fn player_card(&self) -> String {
    self.player_card.clone()
  }

  pub fn id(&self) -> [u8; 16] {
    self.id
  }
  
  pub fn created_at(&self) -> i64 {
    self.created_at
  }
  
  pub fn updated_at(&self) -> i64 {
    self.updated_at
  }

  fn set_puuid(&mut self, puuid: String) {
    self.puuid = puuid;
  }

  fn set_game_name(&mut self, game_name: String) {
    self.game_name = game_name;
  }

  fn set_tag_line(&mut self, tag_line: String) {
    self.tag_line = tag_line;
  }

  fn set_player_card(&mut self, player_card: String) {
    self.player_card = player_card;
  }

  fn set_id(&mut self, id: [u8; 16]) {
    self.id = id;
  }
}