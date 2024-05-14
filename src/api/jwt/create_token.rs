use std::env;

use crate::jwt::claims::Claims;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

pub fn create_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        user_id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
}
