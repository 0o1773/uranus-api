use ulid::Ulid;

struct Account {
  id: [u8; 16],
  riot_account_id: [u8; 16],
  discord_account_id: [u8; 16],
}

impl Account {
  fn new_account(riot_account_id: [u8; 16], discord_account_id: [u8; 16]) -> &'static Account {
    let ulid = Ulid::new();

    &Account {
      id: ulid.to_bytes(),
      riot_account_id,
      discord_account_id,
    }
  }

  fn new_account_with_id(id: [u8; 16], riot_account_id: [u8; 16], discord_account_id: [u8; 16]) -> &'static Account {
    &Account {
      id,
      riot_account_id,
      discord_account_id,
    }
  }
  
  fn riot_account_id(&self) -> [u8; 16] {
    self.discord_account_id.clone()
  }
  
  fn discord_account_id(&self) -> [u8; 16] {
    self.discord_account_id.clone()
  }
  
  fn id(&self) -> [u8; 16] {
    self.id.clone()
  }
  
  fn set_riot_account_id(&mut self, riot_account_id: [u8; 16]) {
    self.riot_account_id = riot_account_id;
  }
  
  fn set_discord_account_id(&mut self, discord_account_id: [u8; 16]) {
    self.discord_account_id = discord_account_id;
  }
  
  fn set_id(&mut self, id: [u8; 16]) {
    self.id = id;
  }
}

