use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub enum InputFormat {
    CSV,
}

impl Display for InputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputFormat::CSV => write!(f, "csv"),
        }
    }
}

impl FromStr for InputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CSV" => Ok(InputFormat::CSV),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum OutputFormat {
    CSV,
    SQL,
    JSON,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::CSV => write!(f, "csv"),
            OutputFormat::SQL => write!(f, "sql"),
            OutputFormat::JSON => write!(f, "json"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CSV" => Ok(OutputFormat::CSV),
            "SQL" => Ok(OutputFormat::SQL),
            "JSON" => Ok(OutputFormat::JSON),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Type {
    FLOAT,
    STRING,
    INTEGER,
    BOOLEAN,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::FLOAT => write!(f, "FLOAT"),
            Type::STRING => write!(f, "STRING"),
            Type::INTEGER => write!(f, "INTEGER"),
            Type::BOOLEAN => write!(f, "BOOLEAN"),
        }
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FLOAT" => Ok(Type::FLOAT),
            "STRING" => Ok(Type::STRING),
            "INTEGER" => Ok(Type::INTEGER),
            "BOOLEAN" => Ok(Type::BOOLEAN),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    EQUALS,
    LESSER,
    GREATER,
    DIFFERS,
    EQLESSER,
    EQGREATER,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::EQUALS => write!(f, "EQUALS"),
            Expression::LESSER => write!(f, "LESSER"),
            Expression::GREATER => write!(f, "GREATER"),
            Expression::DIFFERS => write!(f, "DIFFERS"),
            Expression::EQLESSER => write!(f, "EQLESSER"),
            Expression::EQGREATER => write!(f, "EQGREATER"),
        }
    }
}

impl FromStr for Expression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EQUALS" => Ok(Expression::EQUALS),
            "LESSER" => Ok(Expression::LESSER),
            "GREATER" => Ok(Expression::GREATER),
            "DIFFERS" => Ok(Expression::DIFFERS),
            "EQLESSER" => Ok(Expression::EQLESSER),
            "EQGREATER" => Ok(Expression::EQGREATER),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Format {
    URI,
    UUID,
    DATE,
    TIME,
    EMAIL,
    DATETIME,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::URI => write!(f, "URI"),
            Format::UUID => write!(f, "UUID"),
            Format::DATE => write!(f, "DATE"),
            Format::TIME => write!(f, "TIME"),
            Format::EMAIL => write!(f, "EMAIL"),
            Format::DATETIME => write!(f, "DATETIME"),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "URI" => Ok(Format::URI),
            "DATE" => Ok(Format::DATE),
            "UUID" => Ok(Format::UUID),
            "TIME" => Ok(Format::TIME),
            "EMAIL" => Ok(Format::EMAIL),
            "DATETIME" => Ok(Format::DATETIME),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    HALT,
    NOTIFY,
    DISCARD,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::HALT => write!(f, "HALT"),
            Action::NOTIFY => write!(f, "NOTIFY"),
            Action::DISCARD => write!(f, "DISCARD"),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HALT" => Ok(Action::HALT),
            "NOTIFY" => Ok(Action::NOTIFY),
            "DISCARD" => Ok(Action::DISCARD),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Connector {
    TO,
    OR,
    TYPED,
    RESCUE,
    DEFAULT,
    MATCHING,
}

impl Display for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connector::TO => write!(f, "TO"),
            Connector::OR => write!(f, "OR"),
            Connector::TYPED => write!(f, "TYPED"),
            Connector::RESCUE => write!(f, "RESCUE"),
            Connector::DEFAULT => write!(f, "DEFAULT"),
            Connector::MATCHING => write!(f, "MATCHING"),
        }
    }
}

impl FromStr for Connector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TO" => Ok(Connector::TO),
            "OR" => Ok(Connector::OR),
            "TYPED" => Ok(Connector::TYPED),
            "RESCUE" => Ok(Connector::RESCUE),
            "DEFAULT" => Ok(Connector::DEFAULT),
            "MATCHING" => Ok(Connector::MATCHING),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    ADD(Vec<String>),
    ALIAS(Vec<String>),
    MERGE(Vec<String>),
    IGNORE(Vec<String>),
    COERCE(Vec<String>),
    RENAME(Vec<String>),
    FILTER(Vec<String>),
    DISTINCT(Vec<String>),
    VALIDATE(Vec<String>),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens: Vec<String> = s.split(" ").map(|s| s.to_owned()).collect();

        if tokens.is_empty() {
            return Err(s.to_string());
        }

        match tokens.remove(0).as_str() {
            "ADD" => Ok(Instruction::ADD(tokens)),
            "ALIAS" => Ok(Instruction::ALIAS(tokens)),
            "MERGE" => Ok(Instruction::MERGE(tokens)),
            "IGNORE" => Ok(Instruction::IGNORE(tokens)),
            "COERCE" => Ok(Instruction::COERCE(tokens)),
            "RENAME" => Ok(Instruction::RENAME(tokens)),
            "FILTER" => Ok(Instruction::FILTER(tokens)),
            "DISTINCT" => Ok(Instruction::DISTINCT(tokens)),
            "VALIDATE" => Ok(Instruction::VALIDATE(tokens)),
            _ => Err(s.to_string()),
        }
    }
}
