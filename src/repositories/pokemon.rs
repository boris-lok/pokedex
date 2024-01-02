use std::sync::{Mutex, MutexGuard};

use rusqlite::{params, params_from_iter, Connection, OpenFlags};

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

pub enum DeleteError {
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

    fn delete_pokemon(&self, number: PokemonNumber) -> Result<(), DeleteError>;
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

    fn delete_pokemon(&self, number: PokemonNumber) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }

        let mut lock = match self.data.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(DeleteError::Unknown),
        };

        match lock.iter().position(|p| p.number == number) {
            Some(index) => {
                lock.remove(index);
                Ok(())
            }
            None => Err(DeleteError::NotFound),
        }
    }
}

pub struct SqliteRepository {
    connection: Mutex<Connection>,
}

impl SqliteRepository {
    pub fn try_new(path: &str) -> Result<Self, ()> {
        let connection = match Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)
        {
            Ok(connection) => connection,
            _ => return Err(()),
        };

        match connection.execute("pragma foreign_keys = 1", []) {
            Ok(_) => Ok(Self {
                connection: Mutex::new(connection),
            }),
            _ => Err(()),
        }
    }
}

impl Repository for SqliteRepository {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError> {
        let mut lock = match self.connection.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown),
        };

        let transaction = match lock.transaction() {
            Ok(transaction) => transaction,
            Err(_) => return Err(InsertError::Unknown),
        };

        match transaction.execute(
            "insert into pokemons (number, name) values (?, ?)",
            params![u16::from(number.clone()), String::from(name.clone())],
        ) {
            Ok(_) => {}
            Err(rusqlite::Error::SqliteFailure(_, Some(message))) => {
                if message == "UNIQUE constraint failed: pokemons.number" {
                    return Err(InsertError::Conflict);
                } else {
                    return Err(InsertError::Unknown);
                }
            }
            _ => return Err(InsertError::Unknown),
        }

        for _type in Vec::<String>::from(types.clone()) {
            if transaction
                .execute(
                    "insert into types (pokemon_number, name) values (?, ?)",
                    params![u16::from(number.clone()), _type],
                )
                .is_err()
            {
                return Err(InsertError::Unknown);
            }
        }

        match transaction.commit() {
            Ok(_) => Ok(Pokemon::new(number, name, types)),
            _ => Err(InsertError::Unknown),
        }
    }

    fn fetch_all(&self) -> Result<Vec<Pokemon>, RetrieveAllError> {
        let lock = match self.connection.lock() {
            Ok(lock) => lock,
            _ => return Err(RetrieveAllError::Unknown),
        };

        let pokemon_rows = match self::fetch_pokemon_rows(&lock, None) {
            Ok(pokemon_rows) => pokemon_rows,
            _ => return Err(RetrieveAllError::Unknown),
        };

        let mut pokemons = vec![];

        for pokemon_row in pokemon_rows {
            let type_rows = match fetch_type_rows(&lock, pokemon_row.0) {
                Ok(type_rows) => type_rows,
                _ => return Err(RetrieveAllError::Unknown),
            };

            match (
                PokemonNumber::try_from(pokemon_row.0),
                PokemonName::try_from(pokemon_row.1),
                PokemonTypes::try_from(type_rows),
            ) {
                (Ok(number), Ok(name), Ok(types)) => {
                    pokemons.push(Pokemon::new(number, name, types))
                }
                _ => return Err(RetrieveAllError::Unknown),
            }
        }

        Ok(pokemons)
    }

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, RetrieveError> {
        let lock = match self.connection.lock() {
            Ok(lock) => lock,
            _ => return Err(RetrieveError::Unknown),
        };

        let pokemon_rows = match self::fetch_pokemon_rows(&lock, Some(u16::from(number.clone()))) {
            Ok(pokemon_rows) => pokemon_rows,
            _ => return Err(RetrieveError::Unknown),
        };

        let pokemon_row = match pokemon_rows.first() {
            Some(pokemon) => pokemon.to_owned(),
            _ => return Err(RetrieveError::NotFound),
        };

        let type_rows = match fetch_type_rows(&lock, pokemon_row.0) {
            Ok(type_rows) => type_rows,
            _ => return Err(RetrieveError::Unknown),
        };

        match (
            PokemonNumber::try_from(pokemon_row.0),
            PokemonName::try_from(pokemon_row.1),
            PokemonTypes::try_from(type_rows),
        ) {
            (Ok(number), Ok(name), Ok(types)) => Ok(Pokemon::new(number, name, types)),
            _ => Err(RetrieveError::Unknown),
        }
    }

    fn delete_pokemon(&self, number: PokemonNumber) -> Result<(), DeleteError> {
        let lock = match self.connection.lock() {
            Ok(lock) => lock,
            _ => return Err(DeleteError::Unknown),
        };

        match lock.execute(
            "delete from pokemons where number = ?",
            params![u16::from(number)],
        ) {
            Ok(0) => Err(DeleteError::NotFound),
            Ok(_) => Ok(()),
            Err(_) => Err(DeleteError::Unknown),
        }
    }
}

fn fetch_pokemon_rows(
    lock: &MutexGuard<'_, Connection>,
    number: Option<u16>,
) -> Result<Vec<(u16, String)>, ()> {
    let (query, params) = match number {
        Some(number) => (
            "select number, name from pokemons where number = ?",
            vec![number],
        ),
        _ => ("select number, name from pokemons", vec![]),
    };

    let mut stmt = match lock.prepare(query) {
        Ok(stmt) => stmt,
        _ => return Err(()),
    };

    let mut rows = match stmt.query(params_from_iter(params)) {
        Ok(rows) => rows,
        _ => return Err(()),
    };

    let mut pokemons = vec![];

    while let Ok(Some(row)) = rows.next() {
        match (row.get::<usize, u16>(0), row.get::<usize, String>(1)) {
            (Ok(number), Ok(name)) => pokemons.push((number, name)),
            _ => return Err(()),
        }
    }

    Ok(pokemons)
}

fn fetch_type_rows(lock: &MutexGuard<Connection>, number: u16) -> Result<Vec<String>, ()> {
    let mut stmt = match lock.prepare("select name from types where pokemon_number = ?") {
        Ok(stmt) => stmt,
        Err(_) => return Err(()),
    };

    let mut rows = match stmt.query([number]) {
        Ok(rows) => rows,
        Err(_) => return Err(()),
    };

    let mut type_rows = vec![];

    while let Ok(Some(row)) = rows.next() {
        match row.get::<usize, String>(0) {
            Ok(name) => type_rows.push(name),
            _ => return Err(()),
        }
    }

    Ok(type_rows)
}
