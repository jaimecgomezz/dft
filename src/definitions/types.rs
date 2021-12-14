use std::fmt::Debug;
use std::io::{BufRead, Write};

use crate::definitions::traits::Execute;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub typed: String,
}

impl Field {
    pub fn new(name: &str, typed: &str) -> Self {
        Field {
            name: name.to_string(),
            typed: typed.to_string(),
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

pub type Executable = Box<dyn Execute>;
pub type Executables = Vec<Executable>;

pub type Logs = Vec<String>;

pub type OutputWriter = Box<dyn Write>;
pub type InputReader = Box<dyn BufRead>;

#[derive(Debug)]
pub enum InputFormat {
    CSV,
}

#[derive(Debug)]
pub enum OutputFormat {
    CSV,
}
