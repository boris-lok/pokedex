use std::sync::Arc;

use crate::repositories::pokemon::{DeleteError, Repository};

use super::entities::PokemonNumber;

pub struct Request {
    pub number: u16,
}

pub enum Error {
    Unknown,
    BadRequest,
    NotFound,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<(), Error> {
    match PokemonNumber::try_from(req.number) {
        Ok(number) => match repo.delete_pokemon(number) {
            Ok(_) => Ok(()),
            Err(DeleteError::NotFound) => Err(Error::NotFound),
            Err(DeleteError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        domain::entities::{PokemonName, PokemonTypes},
        repositories::pokemon::InMemoryRepository,
    };

    impl Request {
        pub fn new(number: PokemonNumber) -> Self {
            Self {
                number: u16::from(number),
            }
        }
    }

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request::new(PokemonNumber::pikachu());
        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::bad());

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_not_found_error_when_the_repo_does_not_contain_the_pokemon() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_ok_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        let _ = repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        }
    }
}
