use std::io::{BufRead, Write};

pub use crate::definitions::structs::{Field, Record};
use crate::definitions::traits::Executable;

pub type Values = Vec<String>;
pub type Fields = Vec<Field>;
pub type Records = Vec<Record>;

pub type Instruction = Box<dyn Executable>;
pub type Instructions = Vec<Instruction>;

pub type Logs = Vec<String>;

pub type OutputWriter = Box<dyn Write>;
pub type InputReader = Box<dyn BufRead>;
