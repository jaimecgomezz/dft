use std::error::Error;
use std::fmt;

use crate::definitions::enums::{Actions, Connector, Expressions, Formats, Types};
use crate::definitions::traits::{Buildable, Executable};
use crate::instructions::utils::*;

#[derive(Debug)]
pub struct Distinct {
    fields: Vec<String>,
}

impl Buildable for Distinct {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let fields = parse_fields(tokens.get(1), line)?;

        Ok(Box::new(Distinct { fields }))
    }
}

impl Executable for Distinct {}

impl fmt::Display for Distinct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Ignore {
    fields: Vec<String>,
}

impl Buildable for Ignore {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let fields = parse_fields(tokens.get(1), line)?;

        Ok(Box::new(Ignore { fields }))
    }
}

impl Executable for Ignore {}

impl fmt::Display for Ignore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Alias {
    to: String,
    field: String,
}

impl Buildable for Alias {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let field = parse_field(tokens.get(1), line)?;
        validate_connector(tokens.get(2), Connector::TO, line)?;
        let to = parse_field(tokens.get(3), line)?;

        Ok(Box::new(Alias { to, field }))
    }
}

impl Executable for Alias {}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Rename {
    to: String,
    field: String,
}

impl Buildable for Rename {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let field = parse_field(tokens.get(1), line)?;
        validate_connector(tokens.get(2), Connector::TO, line)?;
        let to = parse_field(tokens.get(3), line)?;

        Ok(Box::new(Rename { to, field }))
    }
}

impl Executable for Rename {}

impl fmt::Display for Rename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Merge {
    to: String,
    fields: Vec<String>,
}

impl Buildable for Merge {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let fields = parse_fields(tokens.get(1), line)?;
        validate_connector(tokens.get(2), Connector::TO, line)?;
        let to = parse_field(tokens.get(3), line)?;

        Ok(Box::new(Merge { to, fields }))
    }
}

impl Executable for Merge {}

impl fmt::Display for Merge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Filter {
    fields: Vec<String>,
    expression: Expressions,
    value: String,
}

impl Buildable for Filter {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let fields = parse_fields(tokens.get(1), line)?;
        validate_connector(tokens.get(2), Connector::MATCHING, line)?;
        let expression = parse_expression(tokens.get(3), line)?;
        let value = parse_value(tokens.get(4), line)?;

        Ok(Box::new(Filter {
            fields,
            value,
            expression,
        }))
    }
}

impl Executable for Filter {}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Validate {
    field: String,
    format: Formats,
    action: Actions,
}

impl Buildable for Validate {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let field = parse_field(tokens.get(1), line)?;
        validate_connector(tokens.get(2), Connector::MATCHING, line)?;
        let format = parse_format(tokens.get(3), line)?;
        validate_connector(tokens.get(4), Connector::OR, line)?;
        let action = parse_action(tokens.get(5), line)?;

        Ok(Box::new(Validate {
            field,
            format,
            action,
        }))
    }
}

impl Executable for Validate {}

impl fmt::Display for Validate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Coerce {
    fields: Vec<String>,
    typed: Types,
    rescue: String,
}

impl Buildable for Coerce {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let fields = parse_fields(tokens.get(1), line)?;
        validate_connector(tokens.get(2), Connector::TYPED, line)?;
        let typed = parse_typed(tokens.get(3), line)?;
        validate_connector(tokens.get(4), Connector::RESCUE, line)?;
        let rescue = parse_field(tokens.get(5), line)?;

        Ok(Box::new(Coerce {
            fields,
            typed,
            rescue,
        }))
    }
}

impl Executable for Coerce {}

impl fmt::Display for Coerce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Add {
    fields: Vec<String>,
    typed: Types,
    default: String,
}

impl Buildable for Add {
    fn from_tokens(tokens: Vec<&str>, line: &usize) -> Result<Box<dyn Executable>, Box<dyn Error>> {
        let fields = parse_fields(tokens.get(1), line)?;
        validate_connector(tokens.get(2), Connector::TYPED, line)?;
        let typed = parse_typed(tokens.get(3), line)?;
        validate_connector(tokens.get(4), Connector::DEFAULT, line)?;
        let default = parse_value(tokens.get(5), line)?;

        Ok(Box::new(Add {
            fields,
            typed,
            default,
        }))
    }
}

impl Executable for Add {}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
