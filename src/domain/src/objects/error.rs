pub enum AccountRepositoryError {
  InsertError,
  ParseError,
  QueryError,
  DatabaseError,
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
  DatabaseError,
}

pub enum DiscordAccountRepositoryError {
  InsertError,
  AccountNotFound,
  QueryError,
  ParseError,
  UpdateError,
  DeleteError,
  DatabaseError,
  InvalidToken,
  FailedToFetchUser,
  InvalidTokenScope,
}

pub enum AgentRepositoryError {
  DatabaseError,
  InsertError,
  FindError,
  QueryError,
  UpdateError,
  DeleteError,
  ListError,
  NotFound,
}

#[derive(Debug)]
pub enum MapRepositoryError {
  DatabaseError,
  InsertError,
  MapNotFound,
  QueryError,
  UpdateError,
  DeleteError,
  ParseError,
  RequestError,
}

pub enum MatchResultRepositoryError {
  RiotApiKeyNotFound,
  RiotApiError,
  DatabaseError,
  RiotApiParseError,
  PlayerDataNotFound,
  AccountNotFound,
  ParseError,
  PlayerNotFound,
  MapNotFound,
  NotFound,
  AgentNotFound,
  FindError,
  UpdateError,
  DeleteError,
}

pub enum MatchListRepositoryError {

}

pub enum AccountApplicationError {

}