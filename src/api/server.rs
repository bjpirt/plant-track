use rocket::serde::{json::Json, Deserialize};
mod db;
mod handlers;
mod types;
use types::user::User;

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

#[post("/login", data = "<login_data>")]
async fn login(login_data: Json<LoginData>) -> String {
    let result: Result<String, String> =
        handlers::login::login(login_data.email.clone(), login_data.password.clone()).await;
    match result {
        Ok(token) => token,
        Err(e) => e,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_user])
        .mount("/", routes![login])
}
