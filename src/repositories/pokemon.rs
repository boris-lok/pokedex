use std::sync::Mutex;

use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum RetrieveAllError {
    Unknown,
}

pub enum RetrieveError {
    Unknown,
    NotFound,
}

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError>;

    fn fetch_all(&self) -> Result<Vec<Pokemon>, RetrieveAllError>;

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, RetrieveError>;
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
    ) -> Result<Pokemon, InsertError> {
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
        let pokemon = Pokemon::new(number_clone, name, types);
        lock.push(pokemon.clone());
        Ok(pokemon)
    }

    fn fetch_all(&self) -> Result<Vec<Pokemon>, RetrieveAllError> {
        if self.error {
            return Err(RetrieveAllError::Unknown);
        }

        let lock = match self.data.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RetrieveAllError::Unknown),
        };

        let mut pokemons = lock.to_vec();
        pokemons.sort_by(|a, b| a.number.cmp(&b.number));
        Ok(pokemons)
    }

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, RetrieveError> {
        if self.error {
            return Err(RetrieveError::Unknown);
        }

        let lock = match self.data.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RetrieveError::Unknown),
        };

        match lock.iter().find(|p| p.number == number).cloned() {
            Some(pokemon) => Ok(pokemon),
            None => Err(RetrieveError::NotFound),
        }
    }
}
