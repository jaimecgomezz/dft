use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "dft", about = "data files transformer cli")]
pub struct Cli {
    /// Reverse evaluate
    #[structopt(short, long)]
    pub undo: bool,

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
}

impl Cli {
    pub fn args() -> Self {
        Self::from_args()
    }

    pub fn instructor(&self) -> Option<Box<dyn BufRead>> {
        match File::open(&self.instructions) {
            Err(_) => None,
            Ok(file) => Some(Box::new(BufReader::new(file))),
        }
    }

    pub fn reader(&self) -> Box<dyn BufRead> {
        match &self.input {
            None => Box::new(BufReader::new(io::stdin())),
            Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
        }
    }

    pub fn writer(&self) -> Box<dyn Write> {
        match &self.output {
            None => Box::new(io::stdout()),
            Some(filename) => Box::new(File::create(filename).unwrap()),
        }
    }
}
