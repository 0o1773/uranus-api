use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter};
use domain::objects::discord_account::ID;
use domain::objects::error::RiotAccountRepositoryError;
use domain::objects::riot_account::RiotAccount;
use domain::traits::riot_account::RiotAccountRepository;
use crate::client::db::entity::riot_account;

struct RiotAccountRepositoryImpl {
    db: DbConn
}

impl RiotAccountRepositoryImpl {
    pub fn new(db: DbConn) -> RiotAccountRepositoryImpl {
        RiotAccountRepositoryImpl {
            db
        }
    }
}

impl RiotAccountRepository for RiotAccountRepositoryImpl {
  async fn create(&self, account: RiotAccount) -> Result<RiotAccount, RiotAccountRepositoryError> {
    let db = self.db.clone();

    let created_at_dt = chrono::DateTime::from_timestamp(account.created_at(), 0).unwrap().naive_utc();
    let dt = chrono::Utc::now().naive_utc();
    
    let pear = riot_account::ActiveModel {
      id: Set(account.id().to_vec()),
      puuid: Set(account.puuid().to_string()),
      game_name: Set(account.game_name().to_string()),
      tag_line: Set(account.tag_line().to_string()),
      player_card: Set(account.player_card().to_string()),
      created_at: Set(created_at_dt),
      updated_at: Set(dt),
    };
    
    let result = riot_account::Entity::insert(pear).exec(&db).await;
    
    match result {
      Ok(_) => Ok(account),
      Err(_) => Err(RiotAccountRepositoryError::InsertError)
    }
  }

  async fn find_by_id(&self, id: ID) -> Result<RiotAccount, RiotAccountRepositoryError> {
    let db = self.db.clone();
    let result = riot_account::Entity::find_by_id(id.to_vec())
      .one(&db)
      .await;

    match result {
      Ok(Some(pear)) => {
        let account = RiotAccount::new_riot_account_with_id(
          id,
          pear.puuid,
          pear.game_name,
          pear.tag_line,
          pear.player_card,
          pear.created_at.and_utc().timestamp(),
          pear.updated_at.and_utc().timestamp(),
        );
        Ok(account)
      },
      Ok(None) => Err(RiotAccountRepositoryError::AccountNotFound),
      Err(_) => Err(RiotAccountRepositoryError::QueryError)
    }
  }

  async fn find_by_puuid(&self, puuid: String) -> Result<RiotAccount, RiotAccountRepositoryError> {
    let db = self.db.clone();
    let result = riot_account::Entity::find()
      .filter(riot_account::Column::Puuid.contains(puuid))
      .one(&db)
      .await;

    match result {
      Ok(Some(pear)) => {
        let id: ID = match pear.id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(RiotAccountRepositoryError::ParseError)
        };
        let account = RiotAccount::new_riot_account_with_id(
          id,
          pear.puuid,
          pear.game_name,
          pear.tag_line,
          pear.player_card,
          pear.created_at.and_utc().timestamp(),
          pear.updated_at.and_utc().timestamp(),
        );
        Ok(account)
      },
      Ok(None) => Err(RiotAccountRepositoryError::AccountNotFound),
      Err(_) => Err(RiotAccountRepositoryError::QueryError)
    }
  }
  async fn find_by_game_name(&self, game_name: String) -> Result<RiotAccount, RiotAccountRepositoryError> {
    let db = self.db.clone();
    let result = riot_account::Entity::find()
      .filter(riot_account::Column::GameName.contains(game_name))
      .one(&db)
      .await;
    
    match result {
      Ok(Some(pear)) => {
        let id: ID = match pear.id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(RiotAccountRepositoryError::ParseError)
        };
        let account = RiotAccount::new_riot_account_with_id(
          id,
          pear.puuid,
          pear.game_name,
          pear.tag_line,
          pear.player_card,
          pear.created_at.and_utc().timestamp(),
          pear.updated_at.and_utc().timestamp(),
        );
        Ok(account)
      },
      Ok(None) => Err(RiotAccountRepositoryError::AccountNotFound),
      Err(_) => Err(RiotAccountRepositoryError::QueryError)
    }
  }
  
  async fn update(&self, id: ID, riot_account: RiotAccount) -> Result<RiotAccount, RiotAccountRepositoryError> {
    let db = self.db.clone();

    let created_at_dt = chrono::DateTime::from_timestamp(riot_account.created_at(), 0).unwrap().naive_utc();
    let dt = chrono::Utc::now().naive_utc();
    
    let pear = riot_account::ActiveModel {
      id: Set(riot_account.id().to_vec()),
      puuid: Set(riot_account.puuid().to_string()),
      game_name: Set(riot_account.game_name().to_string()),
      tag_line: Set(riot_account.tag_line().to_string()),
      player_card: Set(riot_account.player_card().to_string()),
      created_at: Set(created_at_dt),
      updated_at: Set(dt),
    };
    
    let result = pear.update(&db).await;
    match result { 
      Ok(_) => Ok(riot_account),
      Err(_) => Err(RiotAccountRepositoryError::UpdateError)
    }
  }

  async fn delete(&self, id: ID) -> Result<(), RiotAccountRepositoryError> {
    let db = self.db.clone();
    let result = riot_account::Entity::delete_by_id(id.to_vec())
      .exec(&db)
      .await;
    
    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(RiotAccountRepositoryError::DeleteError)
    }
  }
}