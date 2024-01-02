use std::sync::Arc;

use crate::{domain::delete_pokemon, repositories::pokemon::Repository};

use super::prompt_number;

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();

    match number {
        Ok(number) => match delete_pokemon::execute(repo, delete_pokemon::Request { number }) {
            Ok(_res) => println!("The pokemon has been deleted"),
            Err(delete_pokemon::Error::BadRequest) => println!("The request is invalid"),
            Err(delete_pokemon::Error::NotFound) => println!("The Pokemon does not exist"),
            Err(delete_pokemon::Error::Unknown) => println!("An unknown error occured"),
        },
        Err(_) => {
            println!("An error occured during the prompt");
        }
    }
}
