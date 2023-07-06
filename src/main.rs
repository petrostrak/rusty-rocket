#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};
use schema::rustaceans;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;

#[database("sqlite_path")]
struct DB(diesel::SqliteConnection);

// curl http://127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DB) -> Value {
    db.run(|c| {
        let result = rustaceans::table
            .limit(100)
            .load::<Rustacean>(c)
            .expect("Failed to read rustaceans entries");
        json!(result)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DB) -> Value {
    db.run(move |c| {
        let rustacean = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
            .expect("Failed to retrieve rustacean");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DB, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Failed to create new rustacean");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, db: DB, rustacean: Json<Rustacean>) -> Value {
    db.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)
            .expect("Failed to update rustacean");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DB) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Failed to delete rustacean");
        status::NoContent
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized!")
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity: One or more fields are missing!")
}

#[catch(500)]
fn internal_server_error() -> Value {
    json!("Internal Server Error!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register(
            "/",
            catchers![
                not_found,
                unauthorized,
                unprocessable_entity,
                internal_server_error
            ],
        )
        .attach(DB::fairing())
        .launch()
        .await;
}
