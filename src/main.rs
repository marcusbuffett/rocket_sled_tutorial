#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct User {
    username: String,
    favorite_food: String,
}

type EndpointResult<T> = Result<T, &'static str>;

#[get("/users/<username>")]
fn get_user(username: String) -> EndpointResult<Json<User>> {
    todo!()
}

#[delete("/users/<username>")]
fn delete_user(username: String) -> EndpointResult<Json<User>> {
    todo!()
}

#[put("/users", data = "<user>")]
fn put_user(user: Json<User>) -> EndpointResult<Json<User>> {
    todo!()
}

fn main() {
    rocket::ignite()
        .mount("/api/", routes![get_user, put_user, delete_user])
        .launch();
}
