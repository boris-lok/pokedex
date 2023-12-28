use std::sync::Arc;

use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};
use crate::repositories::pokemon::{Insert, Repository};

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(id), Ok(name), Ok(types)) => match repo.insert(id, name, types) {
            Insert::Ok(id) => Response::Ok(id.into()),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::pokemon::InMemoryRepository;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);

        match res {
            Response::Ok(n) => {
                assert_eq!(n, number)
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        // Mock
        // Insert a Pokemon with the same number using the use case.
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from("Pikachu".to_string()).unwrap();
        let types = PokemonTypes::try_from(vec!["Electric".to_string()]).unwrap();
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(number, name, types);

        // Act
        // create a Pokemon with the same number.
        let req = Request {
            number: 25,
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };
        let res = execute(repo, req);

        // Assert
        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happen() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);
        match res {
            Response::Error => {}
            _ => unreachable!(),
        }
    }
}
