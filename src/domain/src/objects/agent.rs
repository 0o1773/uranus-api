use ulid::Ulid;
use crate::objects::role::Role;

pub type ID = [u8; 16];

pub struct Agent {
  id: ID,
  riot_agent_id: String,
  name: String,
  role: Role,
  icon: String,
}

impl Agent {
  pub fn new_agent(riot_agent_id: String, name: String, role: Role, icon: String) -> Agent {
    let ulid = Ulid::new();
    Agent {
      id: ulid.to_bytes(),
      riot_agent_id,
      name,
      role,
      icon,
    }
  }
  
  pub fn new_agent_with_id(id: ID, riot_agent_id: String, name: String, role: Role, icon: String) -> Agent {
    Agent {
      id,
      riot_agent_id,
      name,
      role,
      icon,
    }
  }
  
  pub fn riot_agent_id(&self) -> String {
    self.riot_agent_id.clone()
  }
  
  pub fn name(&self) -> String {
    self.name.clone()
  }
  
  pub fn role(&self) -> Role {
    self.role.clone()
  }
  
  pub fn icon(&self) -> String {
    self.icon.clone()
  }
  
  pub fn id(&self) -> ID {
    self.id.clone()
  }
  
  pub fn set_riot_agent_id(&mut self, riot_agent_id: String) {
    self.riot_agent_id = riot_agent_id;
  }
  
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
  
  pub fn set_role(&mut self, role: Role) {
    self.role = role;
  }
  
  pub fn set_icon(&mut self, icon: String) {
    self.icon = icon;
  }
  
  pub fn set_id(&mut self, id: ID) {
    self.id = id;
  }
}