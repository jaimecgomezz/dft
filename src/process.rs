use std::error::Error;
use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;

use crate::adapters::input;
use crate::adapters::output;
use crate::definitions::enums::Instruction;
use crate::definitions::enums::{InputFormat, OutputFormat};
use crate::definitions::executables::*;
use crate::definitions::structs::Optionals;
use crate::definitions::traits::Executable;
use crate::definitions::traits::{InputAdapter, OutputAdapter};
use crate::definitions::types::{Fields, Instructions, Logs, OutputWriter, Records};

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
        for (_nline, rline) in reader.lines().enumerate() {
            let line = rline?;

            match Instruction::from_str(line.as_str()) {
                Ok(instruction) => {
                    let built: Box<dyn Executable> = match instruction {
                        Instruction::ADD(rest) => Box::new(Add::from_str(&rest)?),
                        Instruction::ALIAS(rest) => Box::new(Alias::from_str(&rest)?),
                        Instruction::MERGE(rest) => Box::new(Merge::from_str(&rest)?),
                        Instruction::IGNORE(rest) => Box::new(Ignore::from_str(&rest)?),
                        Instruction::COERCE(rest) => Box::new(Coerce::from_str(&rest)?),
                        Instruction::RENAME(rest) => Box::new(Rename::from_str(&rest)?),
                        Instruction::FILTER(rest) => Box::new(Filter::from_str(&rest)?),
                        Instruction::DISTINCT(rest) => Box::new(Distinct::from_str(&rest)?),
                        Instruction::VALIDATE(rest) => Box::new(Validate::from_str(&rest)?),
                    };

                    self.instructions.push(built);
                }
                Err(e) => panic!("Invalid <instruction>, found: {}", e),
            }
        }

        Ok(self.instructions.len())
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
        optionals: Optionals,
    ) -> Result<usize, Box<dyn Error>> {
        let adapter: Box<dyn OutputAdapter> = match format {
            OutputFormat::CSV => Box::new(output::csv::Adapter),
            OutputFormat::JSON => Box::new(output::json::Adapter),
            OutputFormat::SQL => Box::new(output::sql::Adapter),
        };

        Ok(adapter.write(writer, &self.fields, &self.records, optionals)?)
    }
}
