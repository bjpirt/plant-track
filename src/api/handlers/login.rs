use crate::{
    db::base_gateway::BaseGateway,
    jwt::create_token::create_token,
    types::{login_response::LoginResponse, user::User},
};

pub async fn login(
    email: &String,
    password: &String,
    gateway: &impl BaseGateway,
) -> Result<LoginResponse, String> {
    let result: Result<User, String> = gateway.get_user_by_email(email).await;

    if let Ok(user) = result {
        if user.validated && user.password == *password {
            if let Ok(token) = create_token(user.id) {
                return Ok(LoginResponse { token });
            }
        }
    }

    return Err(String::from("Error logging in"));
}

#[cfg(test)]
mod tests {
    use std::env;

    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

    use crate::{
        db::{base_gateway::BaseGateway, dynamo_gateway::DynamoGateway},
        handlers::login::login,
        jwt::claims::Claims,
        lib::random_id::random_id,
        types::user::User,
    };

    #[async_test]
    async fn it_returns_a_token_for_the_user() -> Result<(), String> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
        let dynamo_gateway = DynamoGateway::new().await;
        // create a user
        let email: String = format!("{}@test.com", random_id(20));
        println!("{email}");
        let mut user = User::new("test-user", "Test User", &email, "1234");
        user.validated = true;
        dynamo_gateway.create_user(&user).await?;
        //log in as user
        let login_result = login(&email, &user.password, &dynamo_gateway).await?;
        let tokendata = decode::<Claims>(
            &login_result.token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS512),
        );
        match tokendata {
            Ok(claims) => {
                assert_eq!(claims.claims.user_id, user.id);
                Ok(())
            }
            Err(_) => Err("Error".to_string()),
        }
    }

    #[async_test]
    async fn it_returns_error_for_unvalidated_user() -> Result<(), String> {
        let dynamo_gateway = DynamoGateway::new().await;
        // create a user
        let email: String = format!("{}@test.com", random_id(20));
        println!("{email}");
        let user = User::new("test-user", "Test User", &email, "1234");
        dynamo_gateway.create_user(&user).await?;
        //log in as user
        let login_result = login(&email, &user.password, &dynamo_gateway).await;

        match login_result {
            Ok(_) => Err("Should not be Ok".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[async_test]
    async fn it_returns_error_for_wrong_password() -> Result<(), String> {
        let dynamo_gateway = DynamoGateway::new().await;
        // create a user
        let email: String = format!("{}@test.com", random_id(20));
        println!("{email}");
        let mut user = User::new("test-user", "Test User", &email, "1234");
        user.validated = true;
        dynamo_gateway.create_user(&user).await?;
        //log in as user
        let login_result = login(&email, &"WrongPassword".to_string(), &dynamo_gateway).await;

        match login_result {
            Ok(_) => Err("Should not be Ok".to_string()),
            Err(_) => Ok(()),
        }
    }
}
