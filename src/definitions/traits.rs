use std::error::Error;

use crate::definitions::types::{Fields, InputReader, OutputWriter, Records};

pub trait Adapter {
    fn read(&self, reader: InputReader) -> Result<(Fields, Records), Box<dyn Error>>;

    fn write(
        &self,
        writer: OutputWriter,
        fields: &Fields,
        records: &Records,
    ) -> Result<usize, Box<dyn Error>>;
}

pub trait Execute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn execute(&self) -> Result<String, Box<dyn Error>> {
        Ok(format!("Executing..."))
    }
}
