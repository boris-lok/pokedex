use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
}

fn execute(req: Request) -> Response {
    match (PokemonNumber::try_from(req.number), PokemonName::try_from(req.name), PokemonTypes::try_from(req.types)) {
        (Ok(id), Ok(_), Ok(_)) => Response::Ok(id.into()),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);

        match res {
            Response::Ok(n) => {
                assert_eq!(n, number)
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }
}
