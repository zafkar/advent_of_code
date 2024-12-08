use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct AdventError(String);

impl AdventError {
    pub fn new(msg: &str) -> AdventError {
        AdventError(msg.to_string())
    }
}

impl Display for AdventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AdventError {}
