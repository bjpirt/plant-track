use chrono::{DateTime, Utc};
use random_string::generate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Plant {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub species: String,
    pub created: DateTime<Utc>,
}

impl Plant {
    pub fn new(name: &String, species: &String, user_id: &String) -> Self {
        let charset: &str = "abcdefghjklmnpqrstuvwxyz23456789";
        Self {
            id: generate(10, charset),
            user_id: user_id.clone(),
            name: name.clone(),
            species: species.clone(),
            created: Utc::now(),
        }
    }
}
