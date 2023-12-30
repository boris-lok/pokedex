use std::sync::Arc;

use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};
use crate::repositories::pokemon::{InsertError, Repository};

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub struct InsertResponse {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<InsertResponse, Error> {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(id), Ok(name), Ok(types)) => match repo.insert(id, name, types) {
            Ok(pokemon) => Ok(InsertResponse {
                number: u16::from(pokemon.number),
                name: String::from(pokemon.name),
                types: Vec::<String>::from(pokemon.types),
            }),
            Err(InsertError::Conflict) => Err(Error::Conflict),
            Err(InsertError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::pokemon::InMemoryRepository;

    impl Request {
        fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
            Self {
                number: u16::from(number),
                name: String::from(name),
                types: Vec::<String>::from(types),
            }
        }
    }

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );
        let res = execute(repo, req);

        match res {
            Ok(InsertResponse {
                number,
                name,
                types,
            }) => {
                assert_eq!(number, 25);
                assert_eq!(name, String::from("Pikachu"));
                assert_eq!(types, vec![String::from("Electric")])
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::empty(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        // Mock
        // Insert a Pokemon with the same number using the use case.
        let number = PokemonNumber::pikachu();
        let name = PokemonName::pikachu();
        let types = PokemonTypes::pikachu();
        let repo = Arc::new(InMemoryRepository::new());
        let _ = repo.insert(number, name, types);

        // Act
        // create a Pokemon with the same number.
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::charmader(),
            PokemonTypes::charmander(),
        );
        let res = execute(repo, req);

        // Assert
        match res {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happen() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);
        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }
}
