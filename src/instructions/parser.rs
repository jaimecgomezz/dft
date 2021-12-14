use std::error::Error;
use std::io::BufRead;

use crate::definitions::executables::*;
use crate::definitions::traits::Buildable;
use crate::definitions::types::Instructions;

pub struct InstructionParser;

impl InstructionParser {
    pub fn parse(reader: Box<dyn BufRead>) -> Result<Instructions, Box<dyn Error>> {
        let mut result: Instructions = vec![];

        let mut nline = 1;
        for rline in reader.lines() {
            let line = rline?;

            let tokens: Vec<&str> = line.split(" ").collect();

            match tokens.first() {
                Some(instruction) => {
                    let executable = match *instruction {
                        "DISTINCT" => Distinct::from_tokens(tokens, &nline)?,
                        "IGNORE" => Ignore::from_tokens(tokens, &nline)?,
                        "ALIAS" => Alias::from_tokens(tokens, &nline)?,
                        "RENAME" => Rename::from_tokens(tokens, &nline)?,
                        "MERGE" => Merge::from_tokens(tokens, &nline)?,
                        "VALIDATE" => Validate::from_tokens(tokens, &nline)?,
                        "FILTER" => Filter::from_tokens(tokens, &nline)?,
                        "COERCE" => Coerce::from_tokens(tokens, &nline)?,
                        "ADD" => Add::from_tokens(tokens, &nline)?,
                        _ => panic!("Invalid instruction on line {}", &nline),
                    };

                    result.push(executable);
                }
                None => (),
            }

            nline += 1;
        }

        Ok(result)
    }
}
