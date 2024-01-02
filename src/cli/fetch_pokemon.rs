use std::sync::Arc;

use crate::{domain::fetch_pokemon, repositories::pokemon::Repository};

use super::prompt_number;

#[derive(Debug)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();

    match number {
        Ok(number) => match fetch_pokemon::execute(repo, fetch_pokemon::Request { number }) {
            Ok(res) => {
                println!(
                    "{:?}",
                    Response {
                        number: res.number,
                        name: res.name,
                        types: res.types,
                    }
                )
            }
            Err(fetch_pokemon::Error::BadRequest) => println!("The request is invalid"),
            Err(fetch_pokemon::Error::NotFound) => println!("The Pokemon does not exist"),
            Err(fetch_pokemon::Error::Unknown) => println!("An unknown error occured"),
        },
        Err(_) => {
            println!("An error occured during the prompt");
        }
    }
}
