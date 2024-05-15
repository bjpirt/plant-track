use crate::db::base_gateway::BaseGateway;

use crate::types::{user::User, user_input::UserInput};

pub async fn create_user(user: &UserInput, gateway: &impl BaseGateway) -> Result<User, String> {
    let new_user: User = User::new(&user.username, &user.name, &user.email, &user.password);
    let result = gateway.create_user(&new_user).await;

    match result {
        Ok(_) => Ok(new_user),
        Err(_) => Err(String::from("Error creating plant for user")),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::{base_gateway::BaseGateway, dynamo_gateway::DynamoGateway},
        handlers::create_user::create_user,
        lib::random_id::random_id,
        types::user_input::UserInput,
    };

    #[async_test]
    async fn it_creates_a_new_user() -> Result<(), String> {
        let dynamo_gateway = DynamoGateway::new().await;
        let email: String = format!("{}@test.com", random_id(20));
        let new_user = UserInput {
            username: "test-user".to_string(),
            name: "Test User".to_string(),
            email: email.to_string(),
            password: "1234".to_string(),
        };
        //test no user exists with the id
        let existing_user = dynamo_gateway.get_user_by_email(&email).await;
        assert_eq!(existing_user.err(), Some("Could not find user".to_string()));
        //create user
        create_user(&new_user, &dynamo_gateway).await?;
        //test user exists
        let created_user = dynamo_gateway.get_user_by_email(&email).await?;
        assert_eq!(created_user.username, new_user.username);
        assert_eq!(created_user.name, new_user.name);
        assert_eq!(created_user.email, new_user.email);
        assert_eq!(created_user.password, new_user.password);
        Ok(())
    }
}
