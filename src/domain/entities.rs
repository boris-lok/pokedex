use crate::domain::entities::PokemonType::{Electric, Fire};

#[derive(PartialEq, Clone)]
pub struct PokemonNumber(u16);

#[cfg(test)]
impl PokemonNumber {
    pub fn pikachu() -> Self {
        Self(25)
    }
}

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

#[derive(Clone)]
pub struct PokemonName(String);

#[cfg(test)]
impl PokemonName {
    pub fn pikachu() -> Self {
        Self(String::from("Pikachu"))
    }

    pub fn charmader() -> Self {
        Self(String::from("Charmander"))
    }

    pub fn empty() -> Self {
        Self(String::from(""))
    }
}

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.is_empty() {
            return Err(());
        }

        Ok(Self(val))
    }
}

impl From<PokemonName> for String {
    fn from(val: PokemonName) -> Self {
        val.0
    }
}

#[derive(Clone)]
pub struct PokemonTypes(Vec<PokemonType>);

#[cfg(test)]
impl PokemonTypes {
    pub fn pikachu() -> Self {
        Self(vec![PokemonType::Electric])
    }

    pub fn charmander() -> Self {
        Self(vec![PokemonType::Fire])
    }
}

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

impl From<PokemonTypes> for Vec<String> {
    fn from(val: PokemonTypes) -> Self {
        val.0.into_iter().map(String::from).collect::<_>()
    }
}

#[derive(Clone)]
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

impl From<PokemonType> for String {
    fn from(val: PokemonType) -> Self {
        String::from(match val {
            Electric => "Electric",
            Fire => "Fire",
        })
    }
}

#[derive(Clone)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
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
