#[derive(Debug)]
pub enum InputFormat {
    CSV,
}

#[derive(Debug)]
pub enum OutputFormat {
    CSV,
    JSON,
}

#[derive(Debug)]
pub enum Type {
    NUMBER,
    BOOLEAN,
    STRING,
}

#[derive(Debug)]
pub enum Expression {
    EQUALS,
    GREATER,
    EQGREATER,
    LESSER,
    EQLESSER,
    DIFFERS,
}

#[derive(Debug)]
pub enum Format {
    DATE,
    TIME,
    DATETIME,
    URI,
    EMAIL,
    UUID,
}

#[derive(Debug)]
pub enum Action {
    DISCARD,
    NOTIFY,
    HALT,
}

#[derive(Debug, PartialEq)]
pub enum Connector {
    TO,
    MATCHING,
    TYPED,
    OR,
    RESCUE,
    DEFAULT,
}
