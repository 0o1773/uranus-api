use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter};
use domain::objects::discord_account::{DiscordAccount, ID};
use domain::objects::error::{DiscordAccountRepositoryError};
use domain::traits::discord_account::DiscordAccountRepository;
use crate::client::db::entity::discord_account;
use crate::dto::discord_account::DiscordApiAccountDto;

pub struct DiscordAccountRepositoryImpl {
  db: DbConn
}

impl DiscordAccountRepositoryImpl {
  pub fn new(db: DbConn) -> DiscordAccountRepositoryImpl {
    DiscordAccountRepositoryImpl {
      db
    }
  }
}

impl DiscordAccountRepository for DiscordAccountRepositoryImpl {
  async fn create(&self, account: DiscordAccount) -> Result<DiscordAccount, DiscordAccountRepositoryError> {
    let db = self.db.clone();
    let mut header = HeaderMap::new();
    header.insert(HeaderName::try_from("Authorization").unwrap(), HeaderValue::from_str(&format!("Bearer {}", account.access_token())).unwrap());
    header.insert(HeaderName::try_from("Content-Type").unwrap(), HeaderValue::from_str("application/json").unwrap());

    let req_builder = reqwest::ClientBuilder::new().default_headers(header).build().unwrap();
    let res = match req_builder.get("https://discord.com/api/users/@me")
      .send()
      .await {
        Ok(res) => res,
        Err(_) => return Err(DiscordAccountRepositoryError::FailedToFetchUser)
      };

    if !res.status().is_success() {
      return Err(DiscordAccountRepositoryError::InvalidToken);
    }

    let discord_user = match res.json::<DiscordApiAccountDto>().await {
      Ok(user) => user,
      Err(_) => return Err(DiscordAccountRepositoryError::FailedToFetchUser)
    };

    if discord_user.email.is_none() {
      return Err(DiscordAccountRepositoryError::InvalidTokenScope);
    }

    let dt = chrono::Utc::now().naive_utc();

    let pear = discord_account::ActiveModel {
      id: Set(account.id().to_vec()),
      discord_id: Set(account.discord_id().to_string()),
      username: Set(account.username().to_string()),
      access_token: Set(account.access_token().to_string()),
      refresh_token: Set(account.refresh_token().to_string()),
      expires_at: Set(account.expires_at() as i32),

      avatar_url: Set("https://cdn.discordapp.com/".to_owned() + &*account.discord_id() + "/" + &*account.discord_id() + "/" + discord_user.avatar.as_str() +".png"),
      created_at: Set(dt),
      updated_at: Set(dt),
    };

    let result = discord_account::Entity::insert(pear).exec(&db).await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(DiscordAccountRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(_) => Ok(account),
      Err(_) => Err(DiscordAccountRepositoryError::InsertError)
    }
  }

  async fn find_by_id(&self, id: ID) -> Result<DiscordAccount, DiscordAccountRepositoryError> {
    let db = self.db.clone();
    let result = discord_account::Entity::find_by_id(id.to_vec())
      .one(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(DiscordAccountRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(Some(pear)) => {
        let account = DiscordAccount::new_discord_account_with_id(
          id,
          pear.discord_id,
          pear.username,
          pear.access_token,
          pear.refresh_token,
          pear.expires_at as i64,
          pear.avatar_url,
          pear.created_at.and_utc().timestamp(),
          pear.updated_at.and_utc().timestamp(),
        );
        Ok(account)
      },
      Ok(None) => Err(DiscordAccountRepositoryError::AccountNotFound),
      Err(_) => Err(DiscordAccountRepositoryError::QueryError)
    }
  }

  async fn update(&self, id: ID, discord_account: DiscordAccount) -> Result<DiscordAccount, DiscordAccountRepositoryError> {
    let db = self.db.clone();
    let created_at_dt = chrono::DateTime::from_timestamp(discord_account.created_at(), 0).unwrap().naive_utc();
    let dt = chrono::Utc::now().naive_utc();

    let pear = discord_account::ActiveModel {
      id: Set(discord_account.id().to_vec()),
      discord_id: Set(discord_account.discord_id().to_string()),
      username: Set(discord_account.username().to_string()),
      access_token: Set(discord_account.access_token().to_string()),
      refresh_token: Set(discord_account.refresh_token().to_string()),
      expires_at: Set(discord_account.expires_at() as i32),
      avatar_url: Set(discord_account.avatar_url().to_string()),
      created_at: Set(created_at_dt),
      updated_at: Set(dt),
    };
    
    let result = pear.update(&db).await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(DiscordAccountRepositoryError::DatabaseError)
    }
    
    match result { 
      Ok(_) => Ok(discord_account),
      Err(_) => Err(DiscordAccountRepositoryError::UpdateError)
    }
  }

  async fn delete(&self, id: ID) -> Result<(), DiscordAccountRepositoryError> {
    let db = self.db.clone();
    let result = discord_account::Entity::delete_by_id(id.to_vec())
      .exec(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(DiscordAccountRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(DiscordAccountRepositoryError::DeleteError)
    
    }
  }

  async fn find_by_discord_id(&self, discord_id: String) -> Result<DiscordAccount, DiscordAccountRepositoryError> {
    let db = self.db.clone();
    let result = discord_account::Entity::find()
      .filter(discord_account::Column::DiscordId.contains(discord_id))
      .one(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(DiscordAccountRepositoryError::DatabaseError)
    }

    match result {
      Ok(Some(pear)) => {
        let account = DiscordAccount::new_discord_account_with_id(
          pear.id.try_into().unwrap(),
          pear.discord_id,
          pear.username,
          pear.access_token,
          pear.refresh_token,
          pear.expires_at as i64,
          pear.avatar_url,
          pear.created_at.and_utc().timestamp(),
          pear.updated_at.and_utc().timestamp(),
        );
        Ok(account)
      },
      Ok(None) => Err(DiscordAccountRepositoryError::AccountNotFound),
      Err(_) => Err(DiscordAccountRepositoryError::QueryError)
    }
  }
  async fn find_by_username(&self, username: String) -> Result<DiscordAccount, DiscordAccountRepositoryError> {
    let db = self.db.clone();
    let result = discord_account::Entity::find()
      .filter(discord_account::Column::Username.contains(username))
      .one(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(DiscordAccountRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(Some(pear)) => {
        let id: ID = match pear.id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(DiscordAccountRepositoryError::ParseError)
        };
        let account = DiscordAccount::new_discord_account_with_id(
          id,
          pear.discord_id,
          pear.username,
          pear.access_token,
          pear.refresh_token,
          pear.expires_at as i64,
          pear.avatar_url,
          pear.created_at.and_utc().timestamp(),
          pear.updated_at.and_utc().timestamp(),
        );
        Ok(account)
      },
      Ok(None) => Err(DiscordAccountRepositoryError::AccountNotFound),
      Err(_) => Err(DiscordAccountRepositoryError::QueryError)
    }
  }
}