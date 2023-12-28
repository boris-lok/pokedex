use std::sync::Mutex;

use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub enum Insert {
    Ok(PokemonNumber),
    Conflict,
    Error,
}

pub trait Repository: Send + Sync {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert;
}

pub struct InMemoryRepository {
    data: Mutex<Vec<Pokemon>>,
    error: bool,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(vec![]),
            error: false,
        }
    }

    #[allow(dead_code)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

impl Repository for InMemoryRepository {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        let mut lock = match self.data.lock() {
            Ok(lock) => lock,
            Err(_) => return Insert::Error,
        };

        if lock.iter().any(|pokemon| pokemon.number == number) {
            return Insert::Conflict;
        }

        let number_clone = number.clone();
        lock.push(Pokemon::new(number_clone, name, types));
        Insert::Ok(number)
    }
}
