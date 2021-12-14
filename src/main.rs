mod adapters;
mod cli;
mod definitions;
mod instructions;
mod process;

use cli::Cli;
use process::Process;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::args();
    let mut process = Process::new();

    // TODO: error handling
    process.read_instructions(cli.instructor()?)?;
    process.read_records(cli.reader()?, cli.iformat()?)?;
    process.run()?;
    process.log(cli.logger()?)?;
    process.write_result(cli.writer()?, cli.oformat()?)?;

    Ok(())
}
