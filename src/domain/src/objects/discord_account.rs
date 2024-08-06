use ulid::Ulid;

pub type ID = [u8; 16];

pub struct DiscordAccount {
  id: [u8; 16],
  discord_id: String,
  username: String,
  access_token: String,
  refresh_token: String,
  expires_at: i64,
  avatar_url: String,
  created_at: i64,
  updated_at: i64,
}

impl DiscordAccount {
  pub fn new(discord_id: String, username: String, access_token: String, refresh_token: String, expires_at: i64, avatar: String, created_at: i64, updated_at: i64) -> DiscordAccount {
    let ulid = Ulid::new();

    DiscordAccount {
      id: ulid.to_bytes(),
      discord_id,
      username,
      access_token,
      refresh_token,
      expires_at,
      avatar_url: avatar,
      created_at,
      updated_at,
    }
  }

  pub fn new_discord_account_with_id(id: [u8; 16], discord_id: String, username: String, access_token: String, refresh_token: String, expires_at: i64, avatar: String, created_at: i64, updated_at: i64) -> DiscordAccount {
    DiscordAccount {
      id,
      discord_id,
      username,
      access_token,
      refresh_token,
      expires_at,
      avatar_url: avatar,
      created_at,
      updated_at,
    }
  }

  pub fn discord_id(&self) -> String {
    self.discord_id.clone()
  }

  pub fn username(&self) -> String {
    self.username.clone()
  }

  pub fn access_token(&self) -> String {
    self.access_token.clone()
  }

  pub fn refresh_token(&self) -> String {
    self.refresh_token.clone()
  }

  pub fn expires_at(&self) -> i64 {
    self.expires_at
  }

  pub fn avatar_url(&self) -> String {
    self.avatar_url.clone()
  }
  
  pub fn created_at(&self) -> i64 {
    self.created_at
  }
  
  pub fn updated_at(&self) -> i64 {
    self.updated_at
  }

  pub fn id(&self) -> [u8; 16] {
    self.id
  }

  fn set_discord_id(&mut self, discord_id: String) {
    self.discord_id = discord_id;
  }

  fn set_username(&mut self, username: String) {
    self.username = username;
  }

  fn set_access_token(&mut self, access_token: String) {
    self.access_token = access_token;
  }

  fn set_refresh_token(&mut self, refresh_token: String) {
    self.refresh_token = refresh_token;
  }

  fn set_expires_at(&mut self, expires_at: i64) {
    self.expires_at = expires_at;
  }

  fn set_avatar(&mut self, avatar: String) {
    self.avatar_url = avatar;
  }

  fn set_id(&mut self, id: [u8; 16]) {
    self.id = id;
  }
}