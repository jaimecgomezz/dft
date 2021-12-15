use std::error::Error;
use std::fmt;
use std::str;

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

pub trait Tokenizable {
    fn tokenize_str(&self, s: &str) -> Vec<&str>;
    fn tokenize_string(&self, s: &str) -> Vec<String>;
}

impl Tokenizable for &str {
    fn tokenize_str(&self, s: &str) -> Vec<&str> {
        self.split(s).collect()
    }

    fn tokenize_string(&self, s: &str) -> Vec<String> {
        let tokens: Vec<&str> = self.split(s).collect();
        tokens.iter().map(|s| s.to_string()).collect()
    }
}
