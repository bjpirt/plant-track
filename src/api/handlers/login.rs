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
