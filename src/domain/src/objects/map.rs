use ulid::Ulid;

pub type ID = [u8; 16];

pub struct Map {
  id: ID,
  map_id: String,
  name: String,
  image: String,
}

impl Map {
  pub fn new_map(map_id: String, name: String, image: String) -> Map {
    let ulid = Ulid::new();
    Map {
      id: ulid.to_bytes(),
      map_id,
      name,
      image,
    }
  }
  
  pub fn new_map_with_id(id: ID, map_id: String, name: String, image: String) -> Map {
    Map {
      id,
      map_id,
      name,
      image,
    }
  }
  
  pub fn map_id(&self) -> String {
    self.map_id.clone()
  }
  
  pub fn name(&self) -> String {
    self.name.clone()
  }
  
  pub fn image(&self) -> String {
    self.image.clone()
  }
  
  pub fn id(&self) -> ID {
    self.id.clone()
  }
  
  pub fn set_map_id(&mut self, map_id: String) {
    self.map_id = map_id;
  }
  
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
  
  pub fn set_image(&mut self, image: String) {
    self.image = image;
  }
  
  pub fn set_id(&mut self, id: ID) {
    self.id = id;
  }
}

pub fn id_from_vec(vec: Vec<u8>) -> ID {
  let mut id = [0; 16];
  id.copy_from_slice(&vec);
  id
}
