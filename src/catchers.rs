use serde_json::{json, Value};

#[catch(404)]
pub fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
pub fn unauthorized() -> Value {
    json!("Unauthorized!")
}

#[catch(422)]
pub fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity: One or more fields are missing!")
}

#[catch(500)]
pub fn internal_server_error() -> Value {
    json!("Internal Server Error!")
}
