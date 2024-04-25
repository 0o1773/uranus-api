#[derive(Debug, PartialEq)]
pub enum DomainError {
    ReqClientBuildError,
    RequestFailed,
    ResponseParseError,
    ApiKeyNotFound,
    SqlQueryExecError,
    DbRowNotFound,
    GetTeamFail,
    KVStoreError,
    GetUserFail,
    GetMapFail
}

#[derive(Debug, PartialEq)]
pub enum AccountRepositoryError {
    FetchAccountFailed,
    AccountNotFound,
    FetchPlayerInfoFailed,
}