use std::error::Error;

use crate::definitions::enums::{Actions, Connector, Expressions, Formats, Types};

pub fn parse_typed(slice: Option<&&str>, line: &usize) -> Result<Types, Box<dyn Error>> {
    match slice {
        Some(s) => match *s {
            "number" => Ok(Types::NUMBER),
            "boolean" => Ok(Types::BOOLEAN),
            "string" => Ok(Types::STRING),
            _ => panic!("Invalid <type> on line {}", line),
        },
        None => panic!("Missing <type> on line {}", line),
    }
}

pub fn parse_expression(slice: Option<&&str>, line: &usize) -> Result<Expressions, Box<dyn Error>> {
    match slice {
        Some(s) => match *s {
            "equals" => Ok(Expressions::EQUALS),
            "greater" => Ok(Expressions::GREATER),
            "eqgreater" => Ok(Expressions::EQGREATER),
            "lesser" => Ok(Expressions::LESSER),
            "eqlesser" => Ok(Expressions::EQLESSER),
            "differs" => Ok(Expressions::DIFFERS),
            _ => panic!("Invalid <expression> on line {}", line),
        },
        None => panic!("Missing <expression> on line {}", line),
    }
}

pub fn parse_format(slice: Option<&&str>, line: &usize) -> Result<Formats, Box<dyn Error>> {
    match slice {
        Some(s) => match *s {
            "date" => Ok(Formats::DATE),
            "time" => Ok(Formats::TIME),
            "datetime" => Ok(Formats::DATETIME),
            "uri" => Ok(Formats::URI),
            "email" => Ok(Formats::EMAIL),
            "uuid" => Ok(Formats::UUID),
            _ => panic!("Invalid <format> on line {}", line),
        },
        None => panic!("Missing <format> on line {}", line),
    }
}

pub fn parse_action(slice: Option<&&str>, line: &usize) -> Result<Actions, Box<dyn Error>> {
    match slice {
        Some(s) => match *s {
            "discard" => Ok(Actions::DISCARD),
            "notify" => Ok(Actions::NOTIFY),
            "halt" => Ok(Actions::HALT),
            _ => panic!("Invalid <action> on line {}", line),
        },
        None => panic!("Missing <action> on line {}", line),
    }
}

pub fn validate_connector(
    slice: Option<&&str>,
    expected: Connector,
    line: &usize,
) -> Result<(), Box<dyn Error>> {
    let connector = match slice {
        Some(s) => match *s {
            "TO" => Connector::TO,
            "MATCHING" => Connector::MATCHING,
            "TYPED" => Connector::TYPED,
            "OR" => Connector::OR,
            "RESCUE" => Connector::RESCUE,
            "DEFAULT" => Connector::DEFAULT,
            _ => panic!("Invalid <connector> on line {}", line),
        },
        None => panic!("Missing {:?} on line {}", expected, line),
    };

    if connector == expected {
        Ok(())
    } else {
        panic!("Expected {:?} on line {}", expected, line);
    }
}

pub fn parse_value(slice: Option<&&str>, line: &usize) -> Result<String, Box<dyn Error>> {
    match slice {
        Some(s) => Ok(s.to_string()),
        None => panic!("Missing <value> on line {}", line),
    }
}

pub fn parse_field(slice: Option<&&str>, line: &usize) -> Result<String, Box<dyn Error>> {
    match slice {
        Some(s) => Ok(s.to_string()),
        None => panic!("Missing <field> on line {}", line),
    }
}

pub fn parse_fields(slice: Option<&&str>, line: &usize) -> Result<Vec<String>, Box<dyn Error>> {
    match slice {
        Some(s) => {
            let pre: Vec<&str> = s.split(",").collect();

            if pre.is_empty() {
                panic!("Invalid <fields> on line {}", line);
            }

            Ok(pre.into_iter().map(|s| s.to_string()).collect())
        }
        None => panic!("Missing <fields> on line {}", line),
    }
}
