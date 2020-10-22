#![feature(decl_macro)]
#![feature(try_trait)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket::{http::Status, response::Responder};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;
use std::option::NoneError;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("sled db error")]
    SledError(#[from] sled_extensions::Error),
    #[error("resource not found")]
    NotFound,
}

impl From<NoneError> for ServerError {
    fn from(_: NoneError) -> Self {
        ServerError::NotFound
    }
}

impl<'a> Responder<'a> for ServerError {
    fn respond_to(self, _: &rocket::Request) -> Result<rocket::Response<'a>, Status> {
        match self {
            Self::SledError(_) => Err(Status::InternalServerError),
            Self::NotFound => Err(Status::NotFound),
        }
    }
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
    Ok(Json(db.users.get(username.as_bytes())??))
}

#[delete("/users/<username>")]
fn delete_user(db: State<Database>, username: String) -> EndpointResult<Json<User>> {
    let user = db.users.remove(username.as_bytes())??;
    Ok(Json(user))
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
