use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
