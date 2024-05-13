use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInput {
    pub username: String,
    pub name: String,
    pub password: String,
    pub email: String,
}
