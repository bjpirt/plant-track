use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::lib::random_id::random_id;

#[derive(PartialEq, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created: DateTime<Utc>,
    pub validated: bool,
    pub contact: bool,
}

impl User {
    pub fn new(username: &str, name: &str, email: &str, password: &str) -> Self {
        Self {
            id: random_id(20),
            username: username.to_string(),
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created: Utc::now(),
            validated: false,
            contact: true,
        }
    }
}
