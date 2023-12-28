use crate::domain::entities::PokemonType::{Electric, Fire};
use std::convert::Infallible;

#[derive(PartialEq, Clone)]
pub struct PokemonNumber(u16);

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(val: u16) -> Result<Self, Self::Error> {
        if val > 0 && val < 899 {
            Ok(Self(val))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(val: PokemonNumber) -> Self {
        val.0
    }
}

pub struct PokemonName(String);

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.is_empty() {
            return Err(());
        }

        return Ok(Self(val));
    }
}

pub struct PokemonTypes(Vec<PokemonType>);

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(val: Vec<String>) -> Result<Self, Self::Error> {
        if val.is_empty() {
            return Err(());
        }

        let mut pts = vec![];
        for t in val.iter() {
            match PokemonType::try_from(String::from(t)) {
                Ok(pt) => pts.push(pt),
                Err(_) => return Err(()),
            }
        }

        Ok(Self(pts))
    }
}

enum PokemonType {
    Electric,
    Fire,
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(val: String) -> Result<Self, Self::Error> {
        match val.as_str() {
            "Electric" => Ok(Electric),
            "Fire" => Ok(Fire),
            _ => Err(()),
        }
    }
}

pub struct Pokemon {
    pub number: PokemonNumber,
    name: PokemonName,
    types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}
