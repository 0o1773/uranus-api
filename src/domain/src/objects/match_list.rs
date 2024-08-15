use crate::objects::account;
use crate::objects::match_result::MatchResult;

#[derive(Clone)]
pub struct MatchList {
  matches: Vec<MatchResult>,
  account_id: account::ID,
}

impl MatchList {
  pub fn new_match_list(account_id: account::ID) -> MatchList {
    MatchList {
      matches: Vec::new(),
      account_id
    }
  }

  pub fn new_match_list_with_matches(account_id: account::ID, matches: Vec<MatchResult>) -> MatchList {
    MatchList {
      matches,
      account_id
    }
  }

  pub fn add_match(&mut self, match_result: MatchResult) {
    self.matches.push(match_result);
  }

  pub fn account_id(&self) -> account::ID {
    self.account_id.clone()
  }

  pub fn matches(&self) -> Vec<MatchResult> {
    self.matches.clone()
  }

  pub fn set_account_id(&mut self, account_id: account::ID) {
    self.account_id = account_id;
  }
}