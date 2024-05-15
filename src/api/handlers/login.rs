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
    use crate::{
        db::{base_gateway::BaseGateway, dynamo_gateway::DynamoGateway},
        types::user::User,
    };

    #[async_test]
    async fn it_returns_a_token_for_the_user() -> Result<(), String> {
        let dynamo_gateway = DynamoGateway::new().await;
        let new_user = User::new("test-user", "Test User", "test@test.com", "1234");
        //test no user exists with the id
        let existing_user = dynamo_gateway.get_user(&new_user.id).await;
        assert_eq!(existing_user.err(), Some("Could not find user".to_string()));
        //create user
        dynamo_gateway.create_user(&new_user).await?;
        //test user exists
        let created_user = dynamo_gateway.get_user(&new_user.id).await?;
        assert_eq!(created_user, new_user);
        Ok(())
    }
}
