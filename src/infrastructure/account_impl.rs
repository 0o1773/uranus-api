use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tonic::async_trait;
use crate::domain:: {
    repository::account_repository::{DiscordAccountRepository, RiotAccountRepository, UserRepository},
    structs::discord_account::DiscordAccount,
    structs::errors::DomainError,
    structs::player_info::PlayerInfo,
    structs::user::User
};
use crate::entities::{
    user,
    riot_account,
    prelude::{
        User as UserEntity,
        RiotAccount as RiotAccountEntity
}};

pub struct PgUserRepository {
    pg_client: DatabaseConnection,
}

impl PgUserRepository {
    pub fn new(pg_client: &DatabaseConnection) -> Self {
        Self {
            pg_client: pg_client.clone(),
        }
    }
}

pub struct PgRiotAccountRepository {
    pg_client:  DatabaseConnection,
}

impl PgRiotAccountRepository {
    pub fn new(pg_client: &DatabaseConnection) -> Self {
        Self {
            pg_client: pg_client.clone(),
        }
    }
}

pub struct PgDiscordAccountRepository {
    pg_client: DatabaseConnection
}

impl PgDiscordAccountRepository {
    pub fn new(pg_client: &DatabaseConnection) -> Self {
        Self {
            pg_client: pg_client.clone()
        }
    }
}


#[async_trait]
impl UserRepository for PgUserRepository {
    async fn fetch_user(&self, user_id: &str) -> Result<User, DomainError> {
        let users = match UserEntity::find_by_id(user_id).one(&self.pg_client).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                return Err(DomainError::SqlQueryExecError);
            }
        };

        if let Some(user) = users {
            Ok(User::new(user.id, user.email))
        } else {
            Err(DomainError::DbRowNotFound)
        }
    }
    async fn fetch_user_by_discord_id(&self, discord_id: &str) -> Result<User, DomainError> {
        let users = match UserEntity::find()
            .filter(user::Column::DiscordId.contains(discord_id))
            .one(&self.pg_client).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                return Err(DomainError::SqlQueryExecError);
            }
        };

        if let Some(user) = users {
            Ok(User::new(user.id, user.email))
        } else {
            Err(DomainError::DbRowNotFound)
        }
    }
    async fn fetch_user_by_puuid(&self, puuid: &str) -> Result<User, DomainError> {
        let riot_accounts =  match RiotAccountEntity::find_by_id(puuid).one(&self.pg_client).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                return Err(DomainError::SqlQueryExecError);
            }
        };

        if let Some(riot_account) = riot_accounts {
            self.fetch_user(riot_account.user_id.as_str()).await
        } else {
            Err(DomainError::DbRowNotFound)
        }
    }
}

#[async_trait]
impl RiotAccountRepository for PgRiotAccountRepository {
    async fn fetch_riot_by_puuid(&self, puuid: &str) -> Result<PlayerInfo, DomainError> {
        let riot_accounts =  match RiotAccountEntity::find_by_id(puuid).one(&self.pg_client).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                return Err(DomainError::SqlQueryExecError);
            }
        };

        if let Some(riot_account) = riot_accounts {
            Ok(PlayerInfo::new(riot_account.puuid, riot_account.name, riot_account.tag))
        } else {
            Err(DomainError::DbRowNotFound)
        }
    }
    async fn fetch_riot_account(&self, user_id: &str) -> Result<PlayerInfo, DomainError> {
        let riot_accounts = match RiotAccountEntity::find()
            .filter(riot_account::Column::UserId.contains(user_id))
            .one(&self.pg_client).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                return Err(DomainError::SqlQueryExecError);
            }
        };

        if let Some(riot_account) = riot_accounts {
            Ok(PlayerInfo::new(riot_account.puuid, riot_account.name, riot_account.tag))
        } else {
            Err(DomainError::DbRowNotFound)
        }
    }
}

#[async_trait]
impl DiscordAccountRepository for PgDiscordAccountRepository {
    async fn fetch_discord_account(&self, user_id: &str) -> Result<DiscordAccount, DomainError> {
        let users = match UserEntity::find()
            .filter(user::Column::Id.contains(user_id))
            .one(&self.pg_client).await {
            Ok(rows) => rows,
            Err(e) => {
                println!("{:?}", e);
                return Err(DomainError::SqlQueryExecError);
            }
        };

        if let Some(row) = users {
            match row.discord_id {
                Some(discord_id) => {
                    Ok(DiscordAccount::new(discord_id))
                }
                None => {
                    Err(DomainError::SqlQueryExecError)
                }
            }
        } else {
            Err(DomainError::DbRowNotFound)
        }
    }

}