use crate::application::{
    interactor::AccountService,
    traits::account::AccountApplications,
};
use crate::domain::{
    repository::account_repository::{DiscordAccountRepository, RiotAccountRepository, UserRepository},
    structs::account::Account,
    structs::discord_account::DiscordAccount,
    structs::errors::{AccountRepositoryError, DomainError},
};
use crate::domain::structs::player_info::PlayerInfo;

#[tonic::async_trait]
impl<UR, RAR, DAR> AccountApplications for AccountService<UR, RAR, DAR>
where
    UR: UserRepository,
    RAR: RiotAccountRepository,
    DAR: DiscordAccountRepository,
{
    async fn fetch_account_by_puuid(&self, puuid: &str) -> Result<Account, AccountRepositoryError> {
        if let Ok(user) = self.user_repository.fetch_user_by_puuid(puuid).await {
            let discord = match self.discord_account_repository.fetch_discord_account(user.get_id()).await {
                Ok(d) => d,
                Err(e) => {
                    return if e == DomainError::DbRowNotFound {
                        Err(AccountRepositoryError::AccountNotFound)
                    } else {
                        Err(AccountRepositoryError::FetchAccountFailed)
                    };
                }
            };
            let riot = match self.riot_account_repository.fetch_riot_account(user.get_id()).await {
                Ok(r) => r,
                Err(e) => {
                    return if e == DomainError::DbRowNotFound {
                        Err(AccountRepositoryError::AccountNotFound)
                    } else {
                        Err(AccountRepositoryError::FetchAccountFailed)
                    };
                }
            };

            Ok(Account::new(riot, user, discord))
        } else {
            Err(AccountRepositoryError::AccountNotFound)
        }
    }
    async fn fetch_account_by_discord_id(&self, discord_id: &str) -> Result<Account, AccountRepositoryError> {
        let user = match self.user_repository.fetch_user_by_discord_id(discord_id).await {
            Ok(user) => user,
            Err(e) => {
                return if e == DomainError::DbRowNotFound {
                    Err(AccountRepositoryError::AccountNotFound)
                } else {
                    Err(AccountRepositoryError::FetchAccountFailed)
                };
            }
        };
        let riot = match self.riot_account_repository.fetch_riot_account(user.get_id()).await {
            Ok(r) => r,
            Err(e) => {
                return if e == DomainError::DbRowNotFound {
                    Err(AccountRepositoryError::AccountNotFound)
                } else {
                    Err(AccountRepositoryError::FetchAccountFailed)
                };
            }
        };

        Ok(Account::new(riot, user, DiscordAccount::new(discord_id.to_string())))
    }
    async fn fetch_account_by_user_id(&self, user_id: &str) -> Result<Account, AccountRepositoryError> {
        let user = match self.user_repository.fetch_user(user_id).await {
            Ok(user) => user,
            Err(e) => {
                println!("{:?}", e);
                return if e == DomainError::DbRowNotFound {
                    Err(AccountRepositoryError::AccountNotFound)
                } else {
                    Err(AccountRepositoryError::FetchAccountFailed)
                };
            }
        };

        let riot = match self.riot_account_repository.fetch_riot_account(user.get_id()).await {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);
                return if e == DomainError::DbRowNotFound {
                    Err(AccountRepositoryError::AccountNotFound)
                } else {
                    Err(AccountRepositoryError::FetchAccountFailed)
                };
            }
        };

        let discord = match self.discord_account_repository.fetch_discord_account(user.get_id()).await {
            Ok(d) => d,
            Err(e) => {
                println!("{:?}", e);
                return if e == DomainError::DbRowNotFound {
                    Err(AccountRepositoryError::AccountNotFound)
                } else {
                    Err(AccountRepositoryError::FetchAccountFailed)
                };
            }
        };

        Ok(Account::new(riot, user, discord))
    }
    async fn fetch_player_info_by_puuid(&self, puuid: &str) -> Result<PlayerInfo, AccountRepositoryError> {
        match self.riot_account_repository.fetch_riot_by_puuid(puuid).await {
            Ok(p) => Ok(p),
            Err(e) => {
                println!("{:?}", e);
                return if e == DomainError::DbRowNotFound {
                    Err(AccountRepositoryError::AccountNotFound)
                } else {
                    println!("{:?}", e);
                    Err(AccountRepositoryError::FetchAccountFailed)
                }
            }
        }
    }
}