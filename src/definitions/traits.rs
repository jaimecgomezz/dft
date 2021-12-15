use std::error::Error;
use std::fmt;

use crate::definitions::structs::Optionals;
use crate::definitions::types::{Fields, InputReader, OutputWriter, Records};

pub trait InputAdapter {
    fn read(&self, reader: InputReader) -> Result<(Fields, Records), Box<dyn Error>>;
}

pub trait OutputAdapter {
    fn write(
        &self,
        writer: OutputWriter,
        fields: &Fields,
        records: &Records,
        optionals: Optionals,
    ) -> Result<usize, Box<dyn Error>>;
}

pub trait Executable: fmt::Display + fmt::Debug {
    fn execute(&self) -> Result<String, Box<dyn Error>> {
        Ok(format!("Executing: {:?}", self))
    }
}

pub trait Buildable {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>>;
}
