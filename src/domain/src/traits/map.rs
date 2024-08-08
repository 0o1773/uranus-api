use crate::objects::error::MapRepositoryError;
use crate::objects::map::{Map, ID};

pub trait MapRepository: Sync + Send + 'static { 
  async fn create(&self, map_id: String, name: String, image: String) -> Result<Map, MapRepositoryError>;
  async fn find_by_id(&self, id: ID) -> Result<Map, MapRepositoryError>;
  async fn update(&self, id: ID, map: Map) -> Result<Map, MapRepositoryError>;
  async fn delete(&self, id: ID) -> Result<(), MapRepositoryError>;
  async fn list(&self) -> Result<Vec<Map>, MapRepositoryError>;
}