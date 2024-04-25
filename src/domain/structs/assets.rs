use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Map {
    id: String,
    name: String,
}
impl Map {
    pub fn new(id: String, name: String) -> Self {
        Self { name, id }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn default() -> Self {
        Self {
            name: String::from("Unknown"),
            id: String::from("Unknown"),
        }
    }
}


#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TierInfo {
    color: String,
    tier: i32,
    tierName: String,
    divisionName: String,
    icon: String,
}

impl TierInfo {
    pub fn new(tier: i32, tier_name: String, division_name: String, icon: String, color: String) -> Self {
        Self {
            color,
            tier,
            tierName: tier_name,
            divisionName: division_name,
            icon
        }
    }
    pub fn get_color(&self) -> &String {
        &self.color
    }
    pub fn get_tier(&self) -> &i32 {
        &self.tier
    }
    pub fn get_tier_name(&self) -> &String {
        &self.tierName
    }
    pub fn get_division_name(&self) -> &String {
        &self.divisionName
    }
    pub fn get_icon(&self) -> &String {
        &self.icon
    }
    pub fn default() -> Self {
        Self {
            color: String::from("Unknown"),
            tier: 0,
            tierName: String::from("Unknown"),
            divisionName: String::from("Unknown"),
            icon: String::from("https://media.valorant-api.com/competitivetiers/564d8e28-c226-3180-6285-e48a390db8b1/0/smallicon.png")
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Agent {
    icon: String,
    logImg: String,
    id: String,
    name: String,
    role: String,
}

impl Agent {
    pub fn new(id: String, name: String, icon: String, log_img: String, role: String) -> Self {
        Self { icon, logImg: log_img, id, name, role }
    }
    pub fn get_icon(&self) -> &String {
        &self.icon
    }
    pub fn get_log_img(&self) -> &String {
        &self.logImg
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_role(&self) -> &String {
        &self.role
    }
    pub fn default() -> Self {
        Self {
            icon: String::from("https://media.valorant-api.com/competitivetiers/564d8e28-c226-3180-6285-e48a390db8b1/0/smallicon.png"),
            logImg: String::from("https://media.valorant-api.com/competitivetiers/564d8e28-c226-3180-6285-e48a390db8b1/0/smallicon.png"),
            id: String::from("Unknown"),
            name: String::from("unknown"),
            role: String::from("unknown"),
        }
    }
}