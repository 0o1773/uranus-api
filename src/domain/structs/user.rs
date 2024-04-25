use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]

#[derive(Deserialize, Serialize)]
pub struct User {
    id: String,
    email: Option<String>,
}

impl User {
    pub fn new(id: String, email: Option<String>) -> Self {
        Self { id, email }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_email(&self) -> &Option<String> {
        &self.email
    }
}