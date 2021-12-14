use std::fmt::Debug;
use std::io::{BufRead, Write};

use crate::definitions::enums::Type;
use crate::definitions::traits::Executable;

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

pub type Values = Vec<String>;
pub type Fields = Vec<Field>;
pub type Records = Vec<Record>;

pub type Instruction = Box<dyn Executable>;
pub type Instructions = Vec<Instruction>;

pub type Logs = Vec<String>;

pub type OutputWriter = Box<dyn Write>;
pub type InputReader = Box<dyn BufRead>;

pub struct Optionals {
    pub tname: String,
    pub ispaces: u16,
}
