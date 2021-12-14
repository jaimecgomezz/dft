use std::fmt;

use crate::definitions::traits::Execute;
use crate::instructions::utils::{parse_token, parse_token_list};

#[derive(Debug)]
pub struct Distinct {
    fields: Vec<String>,
}

impl Distinct {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Distinct { fields }))
    }
}

impl Execute for Distinct {}

impl fmt::Display for Distinct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Ignore {
    fields: Vec<String>,
}

impl Ignore {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Ignore { fields }))
    }
}

impl Execute for Ignore {}

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

impl Alias {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let to = parse_token(tokens.get(3));
        let field = parse_token(tokens.get(1));

        Some(Box::new(Alias { to, field }))
    }
}

impl Execute for Alias {}

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

impl Rename {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let to = parse_token(tokens.get(3));
        let field = parse_token(tokens.get(1));

        Some(Box::new(Rename { to, field }))
    }
}

impl Execute for Rename {}

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

impl Merge {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let to = parse_token(tokens.get(3));
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Merge { to, fields }))
    }
}

impl Execute for Merge {}

impl fmt::Display for Merge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Filter {
    fields: Vec<String>,
    expression: String,
    value: String,
}

impl Filter {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let value = parse_token(tokens.get(4));
        let expression = parse_token(tokens.get(3));
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Filter {
            fields,
            value,
            expression,
        }))
    }
}

impl Execute for Filter {}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Coerce {
    fields: Vec<String>,
    typed: String,
    rescue: String,
}

impl Coerce {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let rescue = parse_token(tokens.get(5));
        let typed = parse_token(tokens.get(3));
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Coerce {
            fields,
            typed,
            rescue,
        }))
    }
}

impl Execute for Coerce {}

impl fmt::Display for Coerce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct Add {
    fields: Vec<String>,
    typed: String,
    default: String,
}

impl Add {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let default = parse_token(tokens.get(5));
        let typed = parse_token(tokens.get(3));
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Add {
            fields,
            typed,
            default,
        }))
    }
}

impl Execute for Add {}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
