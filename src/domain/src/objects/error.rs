pub enum AccountRepositoryError {
  InsertError,
  ParseError,
  QueryError,
  AccountNotFound,
  UpdateError,
  DeleteError
}

pub enum RiotAccountRepositoryError {
  InsertError,
  AccountNotFound,
  QueryError,
  ParseError,
  UpdateError,
  DeleteError,
}

pub enum DiscordAccountRepositoryError {
  InsertError,
  AccountNotFound,
  QueryError,
  ParseError,
  UpdateError,
  DeleteError,
  InvalidToken,
  FailedToFetchUser,
  InvalidTokenScope,
}

pub enum AgentRepositoryError {
  
}

pub enum MapRepositoryError {
  
}

pub enum MatchResultRepositoryError {
  
}