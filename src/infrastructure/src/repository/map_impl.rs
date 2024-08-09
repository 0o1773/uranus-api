use reqwest::Client;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use domain::objects::error::MapRepositoryError;
use domain::objects::map::{id_from_vec, Map, ID};
use domain::traits::map::MapRepository;
use crate::client::db::entity::map;
use crate::dto::map::MapResponseDto;

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
    let db = self.db.clone();
    let map = Map::new_map(map_id, name, image);
    
    let pear = map::ActiveModel {
      id: Set(map.id().to_vec()),
      map_id: Set(map.map_id()),
      name: Set(map.name()),
      image: Set(map.image()),
    };
    
    let result = map::Entity::insert(pear).exec(&db).await;
    
    
    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(MapRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(_) => Ok(map),
      Err(_) => Err(MapRepositoryError::InsertError)
    }
  }

  async fn find_by_id(&self, id: ID) -> Result<Map, MapRepositoryError> {
    let db = self.db.clone();
    let result = map::Entity::find_by_id(id.to_vec())
      .one(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(MapRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(Some(map)) => {
        Ok(Map::new_map_with_id(
          id,
          map.map_id,
          map.name,
          map.image
        ))
      },
      Ok(None) => {
        Err(MapRepositoryError::MapNotFound)
      },
      Err(e) => {
        eprintln!("{:?}", e);
        Err(MapRepositoryError::QueryError)
      }
    }
  }

  async fn find_by_map_id(&self, map_id: String) -> Result<Map, MapRepositoryError> {
    let db = self.db.clone();

    let result = map::Entity::find()
      .filter(map::Column::MapId.contains(map_id.clone()))
      .one(&db)
      .await;

    match result {
      Ok(Some(map)) => {
        match db.close().await {
          Ok(_) => {},
          Err(_) => return Err(MapRepositoryError::DatabaseError)
        };
        Ok(Map::new_map_with_id(
          id_from_vec(map.id),
          map.map_id,
          map.name,
          map.image
        ))
      },
      Ok(None) => {
        let res = match self.client.get("https://valorant-api.com/v1/maps")
          .send()
          .await {
          Ok(res) => res,
          Err(_) => return Err(MapRepositoryError::QueryError)
        };
        
        let maps_dto = match res.json::<MapResponseDto>().await {
          Ok(body) => body,
          Err(_) => return Err(MapRepositoryError::QueryError)
        };
        
        let map_dto = match maps_dto.data.iter().find(|map| map.map_id == map_id.clone()) {
          Some(map) => map,
          None => return Err(MapRepositoryError::MapNotFound)
        };
        
        let map = self.create(map_id.clone(), map_dto.name.clone(), map_dto.image.clone()).await?;
        
        Ok(map)
      },
      Err(_) => {
        match db.close().await {
          Ok(_) => {},
          Err(_) => return Err(MapRepositoryError::DatabaseError)
        };
        Err(MapRepositoryError::QueryError)
      }
    }
  }

  async fn update(&self, map: Map) -> Result<Map, MapRepositoryError> {
    let db = self.db.clone();
    let pear = map::ActiveModel {
      id: Set(map.id().to_vec()),
      map_id: Set(map.map_id()),
      name: Set(map.name()),
      image: Set(map.image()),
    };
    
    let result = map::Entity::update(pear).exec(&db).await;
    
    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(MapRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(_) => Ok(map),
      Err(_) => Err(MapRepositoryError::UpdateError)
    }
  }

  async fn delete(&self, id: ID) -> Result<(), MapRepositoryError> {
    let db = self.db.clone();
    let result = map::Entity::delete_by_id(id.to_vec())
      .exec(&db)
      .await;
    
    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(MapRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(MapRepositoryError::DeleteError)
    }
  }

  async fn list(&self) -> Result<Vec<Map>, MapRepositoryError> {
    let db = self.db.clone();
    let result = map::Entity::find().all(&db).await;
    
    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(MapRepositoryError::DatabaseError)
    }
    
    match result {
      Ok(maps) => Ok(maps.into_iter().map(|map| {
        Map::new_map_with_id(
          id_from_vec(map.id),
          map.map_id,
          map.name,
          map.image
        )
      }).collect()),
      Err(_) => Err(MapRepositoryError::QueryError)
    }
  }
}