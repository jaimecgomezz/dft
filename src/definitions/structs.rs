use std::fmt::Debug;

use crate::definitions::enums::Type;
use crate::definitions::types::Values;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub typed: Type,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Field {
            name: name.to_string(),
            typed: Type::STRING,
        }
    }
}

#[derive(Debug)]
pub struct Record {
    pub id: u64,
    pub discarded: bool,
    pub values: Values,
}

impl Record {
    pub fn new(id: u64, values: Values) -> Self {
        Record {
            id,
            values,
            discarded: false,
        }
    }
}

#[derive(Debug)]
pub struct Optionals {
    pub tname: String,
    pub ispaces: u16,
}
