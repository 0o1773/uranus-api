use sea_orm::{ActiveModelTrait, DbBackend, DbConn, EntityTrait, QueryFilter, Set, Statement};
use domain:: {
  objects::account::{Account, ID},
  objects::error::AccountRepositoryError,
  objects::discord_account,
  objects::riot_account,
  traits::account::AccountRepository
};
use crate::client::db::entity::account;

pub struct AccountRepositoryImpl {
  db: DbConn
}

impl AccountRepositoryImpl {
  pub fn new(db: DbConn) -> AccountRepositoryImpl {
    AccountRepositoryImpl {
      db
    }
  }
}

impl AccountRepository for AccountRepositoryImpl {
  async fn create(&self, riot_account_id: riot_account::ID, discord_account_id: discord_account::ID) -> Result<Account, AccountRepositoryError> {
    let db = self.db.clone();

    let account = Account::new_account(riot_account_id, discord_account_id);
    let pear = account::ActiveModel {
      id: Set(account.id().to_vec()),
      riot_account_id: Set(account.riot_account_id().to_vec()),
      discord_account_id: Set(account.discord_account_id().to_vec())
    };

    let result = account::Entity::insert(pear).exec(&db).await;
    match result {
      Ok(_) => Ok(account),
      Err(_) => Err(AccountRepositoryError::InsertError)
    }
  }

  async fn find_by_id(&self, id: ID) -> Result<Account, AccountRepositoryError> {
    let db = self.db.clone();
    let result = account::Entity::find_by_id(id.to_vec())
      .one(&db)
      .await;



    match result {
      Ok(Some(pear)) => {
        let riot_id: [u8; 16] = match pear.riot_account_id.try_into() {
          Ok(riotId) => riotId,
          Err(_) => return Err(AccountRepositoryError::ParseError)
        };

        let discord_id: [u8; 16] = match pear.discord_account_id.try_into() {
          Ok(discordId) => discordId,
          Err(_) => return Err(AccountRepositoryError::ParseError)
        };

        let account = Account::new_account_with_id(
          id,
          riot_id,
          discord_id
        );
        Ok(account)
      },
      Ok(None) => Err(AccountRepositoryError::AccountNotFound),
      Err(_) => Err(AccountRepositoryError::QueryError)
    }
  }

  async fn update(&self, id: ID, account: Account) -> Result<Account, AccountRepositoryError> {
    let db = self.db.clone();
    let pear = account::ActiveModel {
      id: Set(account.id().to_vec()),
      riot_account_id: Set(account.riot_account_id().to_vec()),
      discord_account_id: Set(account.discord_account_id().to_vec())
    };

    let result = pear.update(&db).await;

    match result {
      Ok(_) => Ok(account),
      Err(_) => Err(AccountRepositoryError::UpdateError)
    }
  }

  async fn delete(&self, id: ID) -> Result<(), AccountRepositoryError> {
    let db = self.db.clone();

    let res = account::Entity::delete_by_id(id.to_vec())
      .exec(&db)
      .await;

    match res {
      Ok(_) => Ok(()),
      Err(_) => Err(AccountRepositoryError::DeleteError)
    }
  }

  async fn find_by_riot_account_id(&self, riot_account_id: riot_account::ID) -> Result<Account, AccountRepositoryError> {
    let db = self.db.clone();
    let result = account::Entity::find()
      .from_raw_sql(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT "id", "riot_account_id", "discord_id" FROM "account" WHERE "riot_account_id" = $1"#,
          [riot_account_id.to_vec().into()]
      ))
      .one(&db)
      .await;
    
    match result {
      Ok(Some(pear)) => {
        let id: [u8; 16] = match pear.id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(AccountRepositoryError::ParseError)
        };

        let discord_id: [u8; 16] = match pear.discord_account_id.try_into() {
          Ok(discord_id) => discord_id,
          Err(_) => return Err(AccountRepositoryError::ParseError)
        };

        let account = Account::new_account_with_id(
          id,
          riot_account_id,
          discord_id
        );
        Ok(account)
      },
      Ok(None) => Err(AccountRepositoryError::AccountNotFound),
      Err(_) => Err(AccountRepositoryError::QueryError)
    }
  }

  async fn find_by_discord_account_id(&self, discord_account_id: discord_account::ID) -> Result<Account, AccountRepositoryError> {
    let db = self.db.clone();
    let result = account::Entity::find()
      .from_raw_sql(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT "id", "riot_account_id", "discord_id" FROM "account" WHERE "discord_account_id" = $1"#,
          [discord_account_id.to_vec().into()]
      ))
      .one(&db)
      .await;
    
    match result {
      Ok(Some(pear)) => {
        let id: [u8; 16] = match pear.id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(AccountRepositoryError::ParseError)
        };

        let riot_id: [u8; 16] = match pear.riot_account_id.try_into() {
          Ok(riot_id) => riot_id,
          Err(_) => return Err(AccountRepositoryError::ParseError)
        };

        let account = Account::new_account_with_id(
          id,
          riot_id,
          discord_account_id
        );
        Ok(account)
      },
      Ok(None) => Err(AccountRepositoryError::AccountNotFound),
      Err(_) => Err(AccountRepositoryError::QueryError)
    }
  }
}