use std::sync::Mutex;

use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub enum InsertError {
    Conflict,
    Unknown,
}

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<PokemonNumber, InsertError>;
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

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

impl Repository for InMemoryRepository {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<PokemonNumber, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut lock = match self.data.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(InsertError::Unknown),
        };

        if lock.iter().any(|pokemon| pokemon.number == number) {
            return Err(InsertError::Conflict);
        }

        let number_clone = number.clone();
        lock.push(Pokemon::new(number_clone, name, types));
        Ok(number)
    }
}
