#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("sled db error")]
    SledError(#[from] sled_extensions::Error),
}

struct Database {
    users: Tree<User>,
}

#[derive(Deserialize, Serialize, Clone)]
struct User {
    username: String,
    favorite_food: String,
}

type EndpointResult<T> = Result<T, ServerError>;

#[get("/users/<username>")]
fn get_user(db: State<Database>, username: String) -> EndpointResult<Json<User>> {
    todo!()
}

#[delete("/users/<username>")]
fn delete_user(db: State<Database>, username: String) -> EndpointResult<Json<User>> {
    todo!()
}

#[put("/users", data = "<user>")]
fn put_user(db: State<Database>, user: Json<User>) -> EndpointResult<Json<User>> {
    db.users
        .insert(user.username.as_bytes(), user.clone())
        .unwrap();
    Ok(Json(user.0))
}

fn main() {
    let db = sled_extensions::Config::default()
        .path("./sled_data")
        .open()
        .expect("Failed to open sled db");
    rocket::ignite()
        .manage(Database {
            users: db
                .open_bincode_tree("users")
                .expect("failed to open user tree"),
        })
        .mount("/api/", routes![get_user, put_user, delete_user])
        .launch();
}
