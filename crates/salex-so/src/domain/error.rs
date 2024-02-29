use std::{error::Error as StdError, fmt::Display};
#[derive(Debug)]
pub enum Error {
    UnknownHvTyp(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownHvTyp(err) => write!(f, "Unknown HV type: {}", err),
        }
    }
}

impl StdError for Error {}
