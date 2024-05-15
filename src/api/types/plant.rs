use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::lib::random_id::random_id;

#[derive(Serialize, PartialEq, Debug)]
pub struct Plant {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub species: String,
    pub created: DateTime<Utc>,
}

impl Plant {
    pub fn new(name: &str, species: &str, user_id: &str) -> Self {
        Self {
            id: random_id(10),
            user_id: user_id.to_string(),
            name: name.to_string(),
            species: species.to_string(),
            created: Utc::now(),
        }
    }
}
