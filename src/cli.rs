use std::env::current_dir;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use crate::definitions::enums::{InputFormat, OutputFormat};
use crate::definitions::types::Optionals;

#[derive(StructOpt, Debug)]
#[structopt(name = "dft", about = "data files transformer")]
pub struct Cli {
    /// Reverse evaluate?
    #[structopt(short, long)]
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

    /// Table name. Only useful when output format is set to sql.
    /// Defaults to table_name
    #[structopt(long = "table-name")]
    pub tname: Option<String>,

    /// Json indentation spaces. Only useful when output is set to json.
    /// Defaults to 4
    #[structopt(long = "indent-spaces")]
    pub ispaces: Option<u16>,
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
                                let components: Vec<&str> = fstr.split(".").collect();

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
                                let components: Vec<&str> = fstr.split(".").collect();

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
            tname: self.tname(),
            ispaces: self.ispaces(),
        }
    }

    fn ispaces(&self) -> u16 {
        match &self.ispaces {
            Some(spaces) => spaces.to_owned(),
            None => 4,
        }
    }

    fn tname(&self) -> String {
        match &self.tname {
            Some(name) => name.to_owned(),
            None => "table_name".to_string(),
        }
    }
}
