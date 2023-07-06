use rocket::{
    http::Status,
    response::status::{self, Custom},
    serde::json::Json,
};
use serde_json::{json, Value};

use crate::{
    auth::BasicAuth,
    models::{NewRustacean, Rustacean},
    repositories::RustaceanRepository,
};

#[database("sqlite_path")]
pub struct DB(diesel::SqliteConnection);

// curl http://127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans")]
pub async fn get_rustaceans(_auth: BasicAuth, db: DB) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::get_all(c, 100)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, _auth: BasicAuth, db: DB) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::get_by_id(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    _auth: BasicAuth,
    db: DB,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DB,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DB,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}
