#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Role {
  Duelist,
  Initiator,
  Controller,
  Sentinel,
  Unknown
}

impl Role {
  pub fn from_string(role: &str) -> Role {
    match role {
      "Duelist" => Role::Duelist,
      "duelist" => Role::Duelist,
      "Initiator" => Role::Initiator,
      "initiator" => Role::Initiator,
      "Controller" => Role::Controller,
      "controller" => Role::Controller,
      "Sentinel" => Role::Sentinel,
      "sentinel" => Role::Sentinel,
      _ => Role::Unknown,
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Role::Duelist => "Duelist".to_string(),
      Role::Initiator => "Initiator".to_string(),
      Role::Controller => "Controller".to_string(),
      Role::Sentinel => "Sentinel".to_string(),
      Role::Unknown => "Unknown".to_string(),
    }
  }
}