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
