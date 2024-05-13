use std::env;

use crate::{
    db::base_gateway::BaseGateway,
    types::{login_response::LoginResponse, user::User},
};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

#[derive(Serialize)]
pub struct Claims {
    pub user_id: String,
    exp: usize,
}

fn generate_auth_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        user_id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub async fn login(
    email: &String,
    password: &String,
    gateway: &impl BaseGateway,
) -> Result<LoginResponse, String> {
    let result: Result<User, String> = gateway.get_user_by_email(email).await;

    match result {
        Ok(user) => {
            if user.validated && user.password == *password {
                if let Ok(token) = generate_auth_token(user.id) {
                    return Ok(LoginResponse { token });
                }
            }

            Err(String::from("Error logging in"))
        }
        Err(_) => Err(String::from("Error logging in")),
    }
}
