use reqwest::Client;
use sea_orm::DbConn;
use domain::objects::error::MapRepositoryError;
use domain::objects::map::{Map, ID};
use domain::traits::map::MapRepository;

struct MapRepositoryImpl {
  db: DbConn,
  client: Client
}

impl MapRepositoryImpl {
  pub fn new(db: DbConn, client: Client) -> MapRepositoryImpl {
    MapRepositoryImpl {
      db,
      client
    }
  }
}

impl MapRepository for MapRepositoryImpl {
  async fn create(&self, map_id: String, name: String, image: String) -> Result<Map, MapRepositoryError> {
    let map = Map::new_map(map_id, name, image);
    
  }

  async fn find_by_id(&self, id: ID) -> Result<Map, MapRepositoryError> {
    todo!()
  }

  async fn update(&self, id: ID, map: Map) -> Result<Map, MapRepositoryError> {
    todo!()
  }

  async fn delete(&self, id: ID) -> Result<(), MapRepositoryError> {
    todo!()
  }

  async fn list(&self) -> Result<Vec<Map>, MapRepositoryError> {
    todo!()
  }
}