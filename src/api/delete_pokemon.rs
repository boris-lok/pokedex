use std::sync::Arc;

use crate::domain::delete_pokemon;
use crate::{api::Status, repositories::pokemon::Repository};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(repo: Arc<dyn Repository>, number: u16) -> rouille::Response {
    let req = delete_pokemon::Request { number };
    match delete_pokemon::execute(repo, req) {
        Ok(()) => rouille::Response::from(Status::Ok),
        Err(delete_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
        Err(delete_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
        Err(delete_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
    }
}
