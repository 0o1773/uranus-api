use ulid::Ulid;

pub type ID = [u8; 16];

pub struct DiscordAccount {
  id: [u8; 16],
  discord_id: &'static str,
  username: &'static str,
  access_token: &'static str,
  refresh_token: &'static str,
  expires_at: i64,
  avatar_url: &'static str,
}

impl DiscordAccount {
  fn new_discord_account(discord_id: &'static str, username: &'static str, access_token: &'static str, refresh_token: &'static str, expires_at: i64, avatar: &'static str) -> DiscordAccount {
    let ulid = Ulid::new();

    DiscordAccount {
      id: ulid.to_bytes(),
      discord_id,
      username,
      access_token,
      refresh_token,
      expires_at,
      avatar_url: avatar,
    }
  }

  fn new_discord_account_with_id(id: [u8; 16], discord_id: &'static str, username: &'static str, access_token: &'static str, refresh_token: &'static str, expires_at: i64, avatar: &'static str) -> DiscordAccount {
    DiscordAccount {
      id,
      discord_id,
      username,
      access_token,
      refresh_token,
      expires_at,
      avatar_url: avatar,
    }
  }

  fn discord_id(&self) -> &str {
    self.discord_id.clone()
  }

  fn username(&self) -> &str {
    self.username.clone()
  }

  fn access_token(&self) -> &str {
    self.access_token.clone()
  }

  fn refresh_token(&self) -> &str {
    self.refresh_token.clone()
  }

  fn expires_at(&self) -> i64 {
    self.expires_at
  }

  fn avatar(&self) -> &str {
    self.avatar_url.clone()
  }

  fn id(&self) -> [u8; 16] {
    self.id
  }

  fn set_discord_id(&mut self, discord_id: &'static str) {
    self.discord_id = discord_id;
  }

  fn set_username(&mut self, username: &'static str) {
    self.username = username;
  }

  fn set_access_token(&mut self, access_token: &'static str) {
    self.access_token = access_token;
  }

  fn set_refresh_token(&mut self, refresh_token: &'static str) {
    self.refresh_token = refresh_token;
  }

  fn set_expires_at(&mut self, expires_at: i64) {
    self.expires_at = expires_at;
  }

  fn set_avatar(&mut self, avatar: &'static str) {
    self.avatar_url = avatar;
  }

  fn set_id(&mut self, id: [u8; 16]) {
    self.id = id;
  }
}