use rocket::serde::{json::Json, Deserialize};
mod handlers;
mod types;
use types::user::User;

#[macro_use]
extern crate rocket;

#[post("/users/login")]
async fn login() -> &'static str {
    "Hello, world!"
}

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_user])
        .mount("/", routes![login])
}
