use rocket::{
    response::status::Unauthorized,
    serde::{json::Json, Deserialize, Serialize},
};
mod db;
mod handlers;
mod types;
use types::{plant::Plant, plant_input::PlantInput, user::User};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct IncomingUser {
    username: String,
    name: String,
    password: String,
    email: String,
}

#[post("/users", data = "<user>")]
async fn create_user(user: Json<IncomingUser>) -> String {
    let new_user: User = User::new(&user.username, &user.name, &user.email, &user.password);
    let result: Result<(), String> = handlers::create_user::create_user(&new_user).await;
    match result {
        Ok(()) => format!("Hello, {0} ({1})!", new_user.name, new_user.username),
        Err(e) => e,
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub token: String,
}

#[post("/users/login", data = "<login_data>")]
async fn login(
    login_data: Json<LoginData>,
) -> Result<Json<LoginResponse>, Unauthorized<Json<ErrorResponse>>> {
    let result: Result<String, String> =
        handlers::login::login(login_data.email.clone(), login_data.password.clone()).await;
    match result {
        Ok(token) => Ok(Json(LoginResponse { token })),
        Err(e) => Err(Unauthorized(Json(ErrorResponse { message: e }))),
    }
}

#[post("/users/<username>/plants", data = "<plant_input>")]
async fn create_plant(
    username: &str,
    plant_input: Json<PlantInput>,
) -> Result<Json<Plant>, Unauthorized<Json<ErrorResponse>>> {
    let result: Result<Plant, String> =
        handlers::create_plant::create_plant(username, &plant_input).await;
    match result {
        Ok(plant) => Ok(Json(plant)),
        Err(e) => Err(Unauthorized(Json(ErrorResponse { message: e }))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_user])
        .mount("/", routes![login])
        .mount("/", routes![create_plant])
}
