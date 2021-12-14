use std::fmt::Debug;

use crate::definitions::traits::Execute;
use crate::instructions::utils::{parse_token, parse_token_list};

pub struct Distinct {
    fields: Vec<String>,
}

impl Distinct {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Distinct { fields }))
    }
}

impl Execute for Distinct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Distinct")
            .field("fields", &self.fields)
            .finish()
    }
}

pub struct Ignore {
    fields: Vec<String>,
}

impl Ignore {
    pub fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Execute>> {
        let fields = parse_token_list(tokens.get(1));

        Some(Box::new(Ignore { fields }))
    }
}

impl Execute for Ignore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ignore")
            .field("fields", &self.fields)
            .finish()
    }
}

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

impl Execute for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Alias")
            .field("field", &self.field)
            .field("to", &self.to)
            .finish()
    }
}

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

impl Execute for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rename")
            .field("field", &self.field)
            .field("to", &self.to)
            .finish()
    }
}

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

impl Execute for Merge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Merge")
            .field("fields", &self.fields)
            .field("to", &self.to)
            .finish()
    }
}

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

impl Execute for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Filter")
            .field("fields", &self.fields)
            .field("expression", &self.expression)
            .field("value", &self.value)
            .finish()
    }
}

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

impl Execute for Coerce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coerce")
            .field("fields", &self.fields)
            .field("typed", &self.typed)
            .field("rescue", &self.rescue)
            .finish()
    }
}

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

impl Execute for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Add")
            .field("fields", &self.fields)
            .field("typed", &self.typed)
            .field("default", &self.default)
            .finish()
    }
}

impl Debug for dyn Execute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}
