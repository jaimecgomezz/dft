use std::error::Error;
use std::io::BufRead;
use std::io::Write;

use crate::adapters::input;
use crate::adapters::output;
use crate::definitions::enums::{InputFormat, OutputFormat};
use crate::definitions::traits::{InputAdapter, OutputAdapter};
use crate::definitions::types::{Fields, Instructions, Logs, OutputWriter, Records};
use crate::instructions::parser::InstructionParser;

#[derive(Debug)]
pub struct Process {
    pub instructions: Instructions,
    pub records: Records,
    pub fields: Fields,
    pub logs: Logs,
}

impl Process {
    pub fn new() -> Self {
        Process {
            logs: vec![],
            fields: vec![],
            records: vec![],
            instructions: vec![],
        }
    }

    pub fn read_instructions(&mut self, reader: Box<dyn BufRead>) -> Result<usize, Box<dyn Error>> {
        let mut executables = InstructionParser::parse(reader)?;

        self.instructions.append(&mut executables);

        Ok(executables.len())
    }

    pub fn read_records(
        &mut self,
        reader: Box<dyn BufRead>,
        format: InputFormat,
    ) -> Result<usize, Box<dyn Error>> {
        let adapter = match format {
            InputFormat::CSV => input::csv::Adapter,
        };

        let (mut fields, mut records) = adapter.read(reader)?;

        self.fields.append(&mut fields);
        self.records.append(&mut records);

        Ok(self.records.len())
    }

    pub fn run(&mut self) -> Result<usize, Box<dyn Error>> {
        for executable in &self.instructions {
            self.logs.push(executable.execute()?);
        }

        Ok(self.logs.len())
    }

    pub fn log(&self, mut writer: OutputWriter) -> Result<usize, Box<dyn Error>> {
        let mut logged = 0;

        for log in &self.logs {
            writeln!(writer, "{}", log)?;

            logged += 1;
        }

        Ok(logged)
    }

    pub fn write_result(
        &self,
        writer: Box<dyn Write>,
        format: OutputFormat,
    ) -> Result<usize, Box<dyn Error>> {
        let adapter: Box<dyn OutputAdapter> = match format {
            OutputFormat::CSV => Box::new(output::csv::Adapter),
            OutputFormat::JSON => Box::new(output::json::Adapter),
        };

        Ok(adapter.write(writer, &self.fields, &self.records)?)
    }
}
