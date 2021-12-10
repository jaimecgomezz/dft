mod cli;
mod instructions;

use cli::Cli;
use instructions::InstructionParser;

fn main() {
    println!(
        "{:#?}",
        InstructionParser::from_instructor(Cli::args().instructor())
    );
}
