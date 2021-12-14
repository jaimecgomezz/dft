use std::error::Error;
use std::io::BufRead;

use crate::definitions::executables::*;
use crate::definitions::types::Executables;

pub struct InstructionParser;

impl InstructionParser {
    pub fn parse(reader: Box<dyn BufRead>) -> Result<Executables, Box<dyn Error>> {
        let mut result: Executables = vec![];

        let mut nline = 1;
        for rline in reader.lines() {
            let line = rline?;

            let tokens: Vec<&str> = line.split(" ").collect();

            match tokens.first() {
                Some(instruction) => {
                    let executable = match *instruction {
                        "DISTINCT" => Distinct::from_tokens(tokens),
                        "IGNORE" => Ignore::from_tokens(tokens),
                        "ALIAS" => Alias::from_tokens(tokens),
                        "RENAME" => Rename::from_tokens(tokens),
                        "MERGE" => Merge::from_tokens(tokens),
                        "FILTER" => Filter::from_tokens(tokens),
                        "COERCE" => Coerce::from_tokens(tokens),
                        "ADD" => Add::from_tokens(tokens),
                        _ => None,
                    };

                    match executable {
                        Some(e) => result.push(e),
                        None => panic!("Invalid instruction on line {}", nline),
                    }
                }
                None => (),
            }

            nline += 1;
        }

        Ok(result)
    }
}
