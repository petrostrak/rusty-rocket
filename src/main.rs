#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

use diesel::prelude::*;
use models::Rustacean;
use rocket::{
    response::status,
    serde::json::{json, Value},
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
fn view_rustacean(id: i32, _auth: BasicAuth, _db: DB) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth, _db: DB) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth, _db: DB) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized!")
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
        .register("/", catchers![not_found, unauthorized])
        .attach(DB::fairing())
        .launch()
        .await;
}
