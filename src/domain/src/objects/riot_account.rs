use ulid::Ulid;

struct RiotAccount {
  id: [u8; 16],
  puuid: &'static str,
  game_name: &'static str,
  tag_line: &'static str,
  player_card: &'static str,
}

impl RiotAccount {
  fn new_riot_account(puuid: &'static str, game_name: &'static str, tag_line: &'static str, player_card: &'static str) -> &'static RiotAccount {
    let ulid = Ulid::new();

    &RiotAccount {
      id: ulid.to_bytes(),
      puuid,
      game_name,
      tag_line,
      player_card,
    }
  }

  fn new_riot_account_with_id(id: [u8; 16], puuid: &'static str, game_name: &'static str, tag_line: &'static str, player_card: &'static str) -> &'static RiotAccount {
    &RiotAccount {
      id,
      puuid,
      game_name,
      tag_line,
      player_card,
    }
  }

  fn puuid(&self) -> &str {
    self.puuid.clone()
  }

  fn game_name(&self) -> &str {
    self.game_name.clone()
  }

  fn tag_line(&self) -> &str {
    self.tag_line.clone()
  }

  fn player_card(&self) -> &str {
    self.player_card.clone()
  }

  fn id(&self) -> [u8; 16] {
    self.id
  }

  fn set_puuid(&mut self, puuid: &str) {
    self.puuid = puuid;
  }

  fn set_game_name(&mut self, game_name: &str) {
    self.game_name = game_name;
  }

  fn set_tag_line(&mut self, tag_line: &str) {
    self.tag_line = tag_line;
  }

  fn set_player_card(&mut self, player_card: &str) {
    self.player_card = player_card;
  }

  fn set_id(&mut self, id: [u8; 16]) {
    self.id = id;
  }
}