use std::env::current_dir;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use crate::definitions::enums::{InputFormat, OutputFormat};
use crate::definitions::structs::Optionals;
use crate::definitions::traits::Tokenizable;

#[derive(StructOpt, Debug)]
#[structopt(name = "dft", about = "data files transformer")]
pub struct Cli {
    /// Reverse evaluate instructions
    #[structopt(long)]
    pub undo: bool,

    /// Overwrite output
    #[structopt(long)]
    pub overwrite: bool,

    /// Verbose mode (-v, -vv, -vvv, ...)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Input file, defaults to stdin
    #[structopt(long, short)]
    pub input: Option<PathBuf>,

    /// Output file, defaults to stdout
    #[structopt(long, short)]
    pub output: Option<PathBuf>,

    /// Input format, see availables
    #[structopt(long, short)]
    pub from: Option<String>,

    /// Output format, see availables
    #[structopt(long, short)]
    pub to: Option<String>,

    /// Instructions file
    #[structopt(long, short = "z", default_value = "instructions.dft")]
    pub instructions: String,

    /// Logger output, defaults to stdout
    #[structopt(short, long)]
    pub logger: Option<String>,

    /// Input and output format, overwrites <from> and <to>
    #[structopt(short = "F", long)]
    pub format: Option<String>,

    /// Only useful when output format is set to sql
    #[structopt(long = "table-name", default_value = "table_name")]
    pub tname: String,

    /// Only useful when output is set to json
    #[structopt(long = "indent-spaces", default_value = "4")]
    pub ispaces: u16,
}

impl Cli {
    pub fn args() -> Self {
        Self::from_args()
    }

    pub fn iformat(&self) -> Result<InputFormat, Box<dyn Error>> {
        let mut result = String::from("");

        result = match &self.format {
            Some(format) => format.to_owned(),
            None => match &self.from {
                Some(from) => from.to_owned(),
                None => match &self.input {
                    Some(path) => match path.file_name() {
                        Some(filename) => match filename.to_str() {
                            Some(fstr) => {
                                let components = fstr.tokenize_str(".");

                                match components.last() {
                                    Some(extension) => extension.to_string(),
                                    None => result,
                                }
                            }
                            None => result,
                        },
                        None => result,
                    },
                    None => result,
                },
            },
        };

        match result.as_str() {
            "csv" => Ok(InputFormat::CSV),
            _ => panic!("Invalid input format"),
        }
    }

    pub fn oformat(&self) -> Result<OutputFormat, Box<dyn Error>> {
        let mut result = String::from("");

        result = match &self.format {
            Some(format) => format.to_owned(),
            None => match &self.to {
                Some(to) => to.to_owned(),
                None => match &self.output {
                    Some(path) => match path.file_name() {
                        Some(filename) => match filename.to_str() {
                            Some(fstr) => {
                                let components = fstr.tokenize_str(".");

                                match components.last() {
                                    Some(extension) => extension.to_string(),
                                    None => result,
                                }
                            }
                            None => result,
                        },
                        None => result,
                    },
                    None => result,
                },
            },
        };

        match result.as_str() {
            "csv" => Ok(OutputFormat::CSV),
            "json" => Ok(OutputFormat::JSON),
            "sql" => Ok(OutputFormat::SQL),
            _ => panic!("Invalid output format"),
        }
    }

    pub fn reader(&self) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        match &self.input {
            None => Ok(Box::new(BufReader::new(stdin()))),
            Some(filename) => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }

    pub fn writer(&self) -> Result<Box<dyn Write>, Box<dyn Error>> {
        match &self.output {
            None => Ok(Box::new(stdout())),
            Some(filename) => {
                let mut path = current_dir()?;

                path.push(filename);

                let file = OpenOptions::new()
                    .read(true)
                    .write(self.overwrite)
                    .create(true)
                    .open(path)?;

                Ok(Box::new(file))
            }
        }
    }

    pub fn instructor(&self) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        let file = File::open(&self.instructions)?;

        Ok(Box::new(BufReader::new(file)))
    }

    pub fn logger(&self) -> Result<Box<dyn Write>, Box<dyn Error>> {
        match &self.logger {
            None => Ok(Box::new(stdout())),
            Some(filename) => {
                let mut path = current_dir()?;

                path.push(filename);

                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path)?;

                Ok(Box::new(file))
            }
        }
    }

    pub fn optionals(&self) -> Optionals {
        Optionals {
            ispaces: self.ispaces,
            tname: self.tname.to_string(),
        }
    }
}
