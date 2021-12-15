use std::error::Error;
use std::io::{BufRead, Write};
use std::str::FromStr;

use crate::adapters::{input, output};
use crate::definitions::enums::{InputFormat, Instruction, OutputFormat};
use crate::definitions::structs::Optionals;
use crate::definitions::traits::{Executable, InputAdapter, OutputAdapter};
use crate::definitions::types::{Fields, Instructions, Logs, OutputWriter, Records};
use crate::utils::flineerror;

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

    pub fn read_instructions(&mut self, reader: Box<dyn BufRead>) -> Result<usize, String> {
        for (nline, rline) in reader.lines().enumerate() {
            let executable: Result<Box<dyn Executable>, String> = match rline {
                Ok(line) => match Instruction::from_str(line.as_ref()) {
                    Ok(instruction) => match instruction.build() {
                        Ok(built) => Ok(built),
                        Err(e) => Err(flineerror("Instructions", nline, e)),
                    },
                    Err(e) => Err(flineerror("Instructions", nline, e)),
                },
                Err(e) => Err(flineerror("Instructions", nline, e.to_string())),
            };

            self.instructions.push(executable?);
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
