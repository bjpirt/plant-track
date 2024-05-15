use db::dynamo_gateway::DynamoGateway;
use rocket::{response::status::Unauthorized, serde::json::Json};
mod db;
mod handlers;
mod jwt;
mod lib;
mod types;
use jwt::validate_token::validate_token;
use rocket_authorization::oauth::OAuth;
use rocket_authorization::Credential;
use types::{
    error_response::ErrorResponse, login_input::LoginInput, login_response::LoginResponse,
    plant::Plant, plant_input::PlantInput, user::User, user_input::UserInput,
};

#[macro_use]
extern crate rocket;

#[post("/users", data = "<user_input>")]
async fn create_user(user_input: Json<UserInput>) -> Result<Json<User>, String> {
    let dynamo_gateway: DynamoGateway = DynamoGateway::new().await;
    let result: Result<types::user::User, String> =
        handlers::create_user::create_user(&user_input, &dynamo_gateway).await;
    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

#[post("/users/login", data = "<login_input>")]
async fn login(
    login_input: Json<LoginInput>,
) -> Result<Json<LoginResponse>, Unauthorized<Json<ErrorResponse>>> {
    let dynamo_gateway: DynamoGateway = DynamoGateway::new().await;
    let result: Result<LoginResponse, String> =
        handlers::login::login(&login_input.email, &login_input.password, &dynamo_gateway).await;
    match result {
        Ok(token) => Ok(Json(token)),
        Err(e) => Err(Unauthorized(Json(ErrorResponse { message: e }))),
    }
}

#[post("/users/<username>/plants", data = "<plant_input>")]
async fn create_plant(
    username: &str,
    plant_input: Json<PlantInput>,
    auth: Credential<OAuth>,
) -> Result<Json<Plant>, Unauthorized<Json<ErrorResponse>>> {
    let dynamo_gateway: DynamoGateway = DynamoGateway::new().await;

    let user = validate_token(&auth.token, &username.to_string(), &dynamo_gateway).await;

    match user {
        Err(e) => Err(Unauthorized(Json(ErrorResponse { message: e }))),
        Ok(user) => {
            let result: Result<Plant, String> =
                handlers::create_plant::create_plant(&user, &plant_input, &dynamo_gateway).await;
            match result {
                Ok(plant) => Ok(Json(plant)),
                Err(e) => Err(Unauthorized(Json(ErrorResponse { message: e }))),
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_user])
        .mount("/", routes![login])
        .mount("/", routes![create_plant])
}
