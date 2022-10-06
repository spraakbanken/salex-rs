pub mod application;
pub mod domain;
use std::{error::Error as StdError, fmt::Display, sync::Arc};

pub type DynStdError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub enum Error {
    General(DynStdError),
    Unknown(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown error",)
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        "unknwon"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
