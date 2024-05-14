use std::env;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{db::base_gateway::BaseGateway, types::user::User};

use super::claims::Claims;

pub async fn validate_token(
    token: &String,
    username: &String,
    gateway: &impl BaseGateway,
) -> Result<User, String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let tokendata: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> =
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS512),
        );

    if let Ok(tokendata) = tokendata {
        let user_result = gateway.get_user_by_username(username).await;
        if let Ok(user) = user_result {
            if user.id == tokendata.claims.user_id {
                return Ok(user);
            }
        }
    }

    Err("Unauthorized: Invalid credentials".to_string())
}
