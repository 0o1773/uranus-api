use sea_orm::{Database};
use tonic::transport::Server;

use uranus_grpc_server::dependency_injection::{dependency_injection};
use uranus_grpc_server::presentation::{AccountsHandler, ValHandler, accounts, val};

use val::valorant_service_server::{ValorantServiceServer};
use accounts::{
    accounts_service_server::{AccountsServiceServer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            panic!("DATABASE_URL is not set");
        }
    };
    let db = Database::connect(db_url.as_str()).await?;
    let api_key = match std::env::var("RIOT_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            panic!("RIOT_API_KEY is not set");
        }
    };
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-Riot-Token", reqwest::header::HeaderValue::from_str(api_key.to_string().as_str()).unwrap());

    let req_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();


    let (account_service, asset_service, match_service) = dependency_injection(
        db,
        req_client,
    );
    
    
    let addr = "[::0]:9000".parse().unwrap();
    let account_handler = AccountsHandler::new(account_service);
    let valorant_handler = ValHandler::new(asset_service, match_service);
    
    println!("{:?}", addr);
    
    Server::builder()
        .add_service(AccountsServiceServer::new(account_handler))
        .add_service(ValorantServiceServer::new(valorant_handler))
        .serve(addr)
        .await?;
    
    Ok(())
}
