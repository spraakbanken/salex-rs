pub mod application;
pub mod domain;
pub use crate::domain::{SaolLemma, Superlemma};
pub use application::queries::ListSuperlemma;
pub use karp_lex::EntryDto;
use std::{error::Error as StdError, fmt::Display, sync::Arc};
pub type SuperlemmaEntryDto = EntryDto<Superlemma>;

pub type DynStdError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
    Infrastructure(String),
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
