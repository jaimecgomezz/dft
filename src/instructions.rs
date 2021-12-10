use std::fmt::Debug;
use std::io::BufRead;

pub type Field = String;
pub type FieldList = Vec<Field>;
pub type Type = String;
pub type Value = String;
pub type Expression = String;

pub trait Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn execute(&self) -> Result<(), &str> {
        println!("Executing");

        Ok(())
    }
}

#[derive(Debug)]
pub struct Distinct {
    fields: FieldList,
}

impl Distinct {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let fields = InstructionParser::parse_token_list(tokens.get(1));

        Some(Box::new(Distinct { fields }))
    }
}

impl Executable for Distinct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Distinct")
            .field("fields", &self.fields)
            .finish()
    }
}

#[derive(Debug)]
pub struct Ignore {
    fields: FieldList,
}

impl Ignore {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let fields = InstructionParser::parse_token_list(tokens.get(1));

        Some(Box::new(Ignore { fields }))
    }
}

impl Executable for Ignore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ignore")
            .field("fields", &self.fields)
            .finish()
    }
}

#[derive(Debug)]
pub struct Alias {
    to: Field,
    field: Field,
}

impl Alias {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let to = InstructionParser::parse_token(tokens.get(3));
        let field = InstructionParser::parse_token(tokens.get(1));

        Some(Box::new(Alias { to, field }))
    }
}

impl Executable for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Alias")
            .field("field", &self.field)
            .field("to", &self.to)
            .finish()
    }
}

#[derive(Debug)]
pub struct Rename {
    to: Field,
    field: Field,
}

impl Rename {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let to = InstructionParser::parse_token(tokens.get(3));
        let field = InstructionParser::parse_token(tokens.get(1));

        Some(Box::new(Rename { to, field }))
    }
}

impl Executable for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rename")
            .field("field", &self.field)
            .field("to", &self.to)
            .finish()
    }
}

#[derive(Debug)]
pub struct Merge {
    to: Field,
    fields: FieldList,
}

impl Merge {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let to = InstructionParser::parse_token(tokens.get(3));
        let fields = InstructionParser::parse_token_list(tokens.get(1));

        Some(Box::new(Merge { to, fields }))
    }
}

impl Executable for Merge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Merge")
            .field("fields", &self.fields)
            .field("to", &self.to)
            .finish()
    }
}

#[derive(Debug)]
pub struct Filter {
    fields: FieldList,
    expression: Expression,
    value: Value,
}

impl Filter {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let value = InstructionParser::parse_token(tokens.get(4));
        let expression = InstructionParser::parse_token(tokens.get(3));
        let fields = InstructionParser::parse_token_list(tokens.get(1));

        Some(Box::new(Filter {
            fields,
            value,
            expression,
        }))
    }
}

impl Executable for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Filter")
            .field("fields", &self.fields)
            .field("expression", &self.expression)
            .field("value", &self.value)
            .finish()
    }
}

#[derive(Debug)]
pub struct Coerce {
    fields: FieldList,
    typed: Type,
    rescue: Value,
}

impl Coerce {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let rescue = InstructionParser::parse_token(tokens.get(5));
        let typed = InstructionParser::parse_token(tokens.get(3));
        let fields = InstructionParser::parse_token_list(tokens.get(1));

        Some(Box::new(Coerce {
            fields,
            typed,
            rescue,
        }))
    }
}

impl Executable for Coerce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coerce")
            .field("fields", &self.fields)
            .field("typed", &self.typed)
            .field("rescue", &self.rescue)
            .finish()
    }
}

#[derive(Debug)]
pub struct Add {
    fields: FieldList,
    typed: Type,
    default: Value,
}

impl Add {
    fn from_tokens(tokens: Vec<&str>) -> Option<Box<dyn Executable>> {
        let default = InstructionParser::parse_token(tokens.get(5));
        let typed = InstructionParser::parse_token(tokens.get(3));
        let fields = InstructionParser::parse_token_list(tokens.get(1));

        Some(Box::new(Add {
            fields,
            typed,
            default,
        }))
    }
}

impl Executable for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Add")
            .field("fields", &self.fields)
            .field("typed", &self.typed)
            .field("default", &self.default)
            .finish()
    }
}

#[derive(Debug)]
pub struct InstructionParser {}

impl InstructionParser {
    pub fn from_instructor(instructor: Option<Box<dyn BufRead>>) -> Vec<Box<dyn Executable>> {
        match instructor {
            Some(reader) => {
                let mut result: Vec<Box<dyn Executable>> = vec![];

                for rline in reader.lines() {
                    match rline {
                        Ok(line) => match Self::from_line(line) {
                            Some(executable) => result.push(executable),
                            None => (),
                        },
                        Err(_) => (),
                    }
                }

                return result;
            }
            None => vec![],
        }
    }

    pub fn from_line(line: String) -> Option<Box<dyn Executable>> {
        let tokens: Vec<&str> = line.split(" ").collect();

        match tokens.first() {
            Some(instruction) => match *instruction {
                "DISTINCT" => Distinct::from_tokens(tokens),
                "IGNORE" => Ignore::from_tokens(tokens),
                "ALIAS" => Alias::from_tokens(tokens),
                "RENAME" => Rename::from_tokens(tokens),
                "MERGE" => Merge::from_tokens(tokens),
                "FILTER" => Filter::from_tokens(tokens),
                "COERCE" => Coerce::from_tokens(tokens),
                "ADD" => Add::from_tokens(tokens),
                _ => None,
            },
            None => None,
        }
    }

    fn parse_token(slice: Option<&&str>) -> String {
        match slice {
            Some(s) => s.to_string(),
            None => String::from(""),
        }
    }

    fn parse_token_list(slice: Option<&&str>) -> Vec<String> {
        match slice {
            Some(s) => {
                let pre: Vec<&str> = s.split(",").collect();

                pre.into_iter().map(|s| s.to_string()).collect()
            }
            None => vec![],
        }
    }
}

impl Debug for dyn Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}
