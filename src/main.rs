use base64::{engine::general_purpose, Engine};
use rocket::{
    response::status,
    serde::json::{json, Value},
};

#[macro_use]
extern crate rocket;
extern crate base64;

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }
        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = general_purpose::STANDARD.decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());
        Some(BasicAuth { username, password })
    }
}

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([{"id": 1, "name": "John Doe" }, {"id": 2, "name": "Doe John" }])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(404)]
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
        .launch()
        .await;
}
