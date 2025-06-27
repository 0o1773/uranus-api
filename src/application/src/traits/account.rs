use domain::objects::account::Account;
use domain::objects::error::AccountApplicationError;

pub trait AccountApplications {
    fn fetch_account(&self, id: i32) -> Result<Account, AccountApplicationError>;
    fn fetch_account_by_discord_id(&self, discord_id: String) -> Result<Account, AccountApplicationError>;

}