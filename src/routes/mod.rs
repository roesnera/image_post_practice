use rocket::{http::Status, response::status::Custom, serde::json::{json, Value}};

pub mod routes;

pub fn server_error(e: Box< dyn std::error::Error>) -> Custom<Value> {
  log::error!("{}", e);
  Custom(Status::InternalServerError, json!({ "error": e.to_string() }))
}