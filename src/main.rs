#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod catchers;
mod handlers;
mod models;
mod repositories;
mod schema;

use catchers::*;
use handlers::*;

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
