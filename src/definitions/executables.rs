use std::fmt;
use std::str;

use crate::definitions::enums::{Action, Connector, Expression, Format, Token, Type};
use crate::definitions::traits::{Executable, Tokenizable};
use crate::utils::{finvalid, finvalidkind, fmissing};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Distinct {
    fields: Vec<String>,
}

impl str::FromStr for Distinct {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.first() {
            Some(s) => Ok(Distinct {
                fields: s.tokenize_string(","),
            }),
            None => Err(fmissing(Token::FIELDS)),
        }
    }
}

impl Executable for Distinct {}

impl fmt::Display for Distinct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Ignore {
    fields: Vec<String>,
}

impl str::FromStr for Ignore {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.first() {
            Some(s) => Ok(Ignore {
                fields: s.tokenize_string(","),
            }),
            None => Err(fmissing(Token::FIELDS)),
        }
    }
}

impl Executable for Ignore {}

impl fmt::Display for Ignore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Alias {
    to: String,
    field: String,
}

impl str::FromStr for Alias {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(field) => match tokens.get(1) {
                Some(connector) => match Connector::from_str(&connector) {
                    Ok(_) => match tokens.get(2) {
                        Some(to) => Ok(Alias {
                            to: to.to_string(),
                            field: field.to_string(),
                        }),
                        None => Err(fmissing(Token::FIELD)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::TO, e)),
                },
                None => Err(fmissing(Connector::TO)),
            },
            None => Err(fmissing(Token::FIELD)),
        }
    }
}

impl Executable for Alias {}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Rename {
    to: String,
    field: String,
}

impl str::FromStr for Rename {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(field) => match tokens.get(1) {
                Some(connector) => match Connector::from_str(&connector) {
                    Ok(_) => match tokens.get(2) {
                        Some(to) => Ok(Rename {
                            to: to.to_string(),
                            field: field.to_string(),
                        }),
                        None => Err(fmissing(Token::FIELDS)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::TO, e)),
                },
                None => Err(fmissing(Connector::TO)),
            },
            None => Err(fmissing(Token::FIELD)),
        }
    }
}

impl Executable for Rename {}

impl fmt::Display for Rename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Merge {
    to: String,
    fields: Vec<String>,
}

impl str::FromStr for Merge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(fields) => match tokens.get(1) {
                Some(connector) => match Connector::from_str(&connector) {
                    Ok(_) => match tokens.get(2) {
                        Some(to) => Ok(Merge {
                            to: to.to_string(),
                            fields: fields.tokenize_string(","),
                        }),
                        None => Err(fmissing(Token::FIELD)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::TO, e)),
                },
                None => Err(fmissing(Connector::TO)),
            },
            None => Err(fmissing(Token::FIELD)),
        }
    }
}

impl Executable for Merge {}

impl fmt::Display for Merge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Set {
    typed: String,
    fields: Vec<String>,
}

impl str::FromStr for Set {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(fields) => match tokens.get(1) {
                Some(connector) => match Connector::from_str(&connector) {
                    Ok(_) => match tokens.get(2) {
                        Some(t) => match Type::from_str(t) {
                            Ok(typed) => Ok(Set {
                                typed: typed.to_string(),
                                fields: fields.tokenize_string(","),
                            }),
                            Err(e) => Err(finvalid(Token::TYPE, e)),
                        },
                        None => Err(fmissing(Token::FIELDS)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::TYPED, e)),
                },
                None => Err(fmissing(Connector::TYPED)),
            },
            None => Err(fmissing(Token::FIELDS)),
        }
    }
}

impl Executable for Set {}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Filter {
    fields: Vec<String>,
    expression: Expression,
    value: String,
}

impl str::FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(fields) => match tokens.get(1) {
                Some(connector) => match Connector::from_str(&connector) {
                    Ok(_) => match tokens.get(2) {
                        Some(exp) => match Expression::from_str(&exp) {
                            Ok(expression) => match tokens.get(3) {
                                Some(value) => Ok(Filter {
                                    expression,
                                    value: value.to_string(),
                                    fields: fields.tokenize_string(","),
                                }),
                                None => Err(fmissing(Token::VALUE)),
                            },
                            Err(e) => Err(finvalid(Token::EXPRESSION, e)),
                        },
                        None => Err(fmissing(Token::EXPRESSION)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::MATCHING, e)),
                },
                None => Err(fmissing(Connector::MATCHING)),
            },
            None => Err(fmissing(Token::FIELDS)),
        }
    }
}

impl Executable for Filter {}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Validate {
    field: String,
    format: Format,
    action: Action,
}

impl str::FromStr for Validate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(field) => match tokens.get(1) {
                Some(c1) => match Connector::from_str(&c1) {
                    Ok(_) => match tokens.get(2) {
                        Some(f) => match Format::from_str(&f) {
                            Ok(format) => match tokens.get(3) {
                                Some(c2) => match Connector::from_str(c2) {
                                    Ok(_) => match tokens.get(4) {
                                        Some(a) => match Action::from_str(a) {
                                            Ok(action) => Ok(Validate {
                                                format,
                                                action,
                                                field: field.to_string(),
                                            }),
                                            Err(e) => Err(finvalid(Token::ACTION, e)),
                                        },
                                        None => Err(fmissing(Token::ACTION)),
                                    },
                                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::OR, e)),
                                },
                                None => Err(fmissing(Connector::OR)),
                            },
                            Err(e) => Err(finvalid(Token::FORMAT, e)),
                        },
                        None => Err(fmissing(Token::FORMAT)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::MATCHING, e)),
                },
                None => Err(fmissing(Connector::MATCHING)),
            },
            None => Err(fmissing(Token::FIELD)),
        }
    }
}

impl Executable for Validate {}

impl fmt::Display for Validate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Coerce {
    fields: Vec<String>,
    typed: Type,
    rescue: String,
}

impl str::FromStr for Coerce {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(fields) => match tokens.get(1) {
                Some(c1) => match Connector::from_str(c1) {
                    Ok(_) => match tokens.get(2) {
                        Some(t) => match Type::from_str(t) {
                            Ok(typed) => match tokens.get(3) {
                                Some(c2) => match Connector::from_str(&c2) {
                                    Ok(_) => match tokens.get(4) {
                                        Some(rescue) => Ok(Coerce {
                                            typed,
                                            rescue: rescue.to_string(),
                                            fields: fields.tokenize_string(","),
                                        }),
                                        None => Err(fmissing(Token::VALUE)),
                                    },
                                    Err(e) => {
                                        Err(finvalidkind(Token::CONNECTOR, Connector::RESCUE, e))
                                    }
                                },
                                None => Err(fmissing(Connector::RESCUE)),
                            },
                            Err(e) => Err(finvalid(Token::TYPE, e)),
                        },
                        None => Err(fmissing(Token::TYPE)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::TYPED, e)),
                },
                None => Err(fmissing(Connector::TYPED)),
            },
            None => Err(fmissing(Token::FIELDS)),
        }
    }
}

impl Executable for Coerce {}

impl fmt::Display for Coerce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Add {
    fields: Vec<String>,
    typed: Type,
    default: String,
}

impl str::FromStr for Add {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.tokenize_str(" ");

        match tokens.get(0) {
            Some(fields) => match tokens.get(1) {
                Some(c1) => match Connector::from_str(c1) {
                    Ok(_) => match tokens.get(2) {
                        Some(t) => match Type::from_str(t) {
                            Ok(typed) => match tokens.get(3) {
                                Some(c2) => match Connector::from_str(&c2) {
                                    Ok(_) => match tokens.get(4) {
                                        Some(default) => Ok(Add {
                                            typed,
                                            default: default.to_string(),
                                            fields: fields.tokenize_string(","),
                                        }),
                                        None => Err(fmissing(Token::VALUE)),
                                    },
                                    Err(e) => {
                                        Err(finvalidkind(Token::CONNECTOR, Connector::DEFAULT, e))
                                    }
                                },
                                None => Err(fmissing(Connector::DEFAULT)),
                            },
                            Err(e) => Err(finvalid(Token::TYPE, e)),
                        },
                        None => Err(fmissing(Token::TYPE)),
                    },
                    Err(e) => Err(finvalidkind(Token::CONNECTOR, Connector::TYPED, e)),
                },
                None => Err(fmissing(Connector::TYPED)),
            },
            None => Err(fmissing(Token::FIELDS)),
        }
    }
}

impl Executable for Add {}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
