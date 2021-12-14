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
pub enum Types {
    NUMBER,
    BOOLEAN,
    STRING,
}

#[derive(Debug)]
pub enum Expressions {
    EQUALS,
    GREATER,
    EQGREATER,
    LESSER,
    EQLESSER,
    DIFFERS,
}

#[derive(Debug)]
pub enum Formats {
    DATE,
    TIME,
    DATETIME,
    URI,
    EMAIL,
    UUID,
}

#[derive(Debug)]
pub enum Actions {
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
