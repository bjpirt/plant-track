use chrono::{DateTime, Utc};
use random_string::generate;
use serde::Serialize;

#[derive(Serialize)]
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
    pub fn new(username: &String, name: &String, email: &String, password: &String) -> Self {
        let charset: &str = "abcdefghjklmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
        Self {
            id: generate(20, charset),
            username: username.clone(),
            name: name.clone(),
            email: email.clone(),
            password: password.clone(),
            created: Utc::now(),
            validated: false,
            contact: true,
        }
    }
}
