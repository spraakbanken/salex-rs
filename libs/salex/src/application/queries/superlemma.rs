use std::collections::HashMap;

use crate::domain::Superlemma;
use crate::Error;
use karp_lex::EntryDto;

pub trait ListSuperlemma {
    fn query(&self, superlemman: &mut HashMap<String, EntryDto<Superlemma>>) -> Result<(), Error>;
}
#[async_trait::async_trait]
pub trait AsyncListSuperlemma {
    fn query(&self, superlemman: &mut HashMap<String, EntryDto<Superlemma>>) -> Result<(), Error>;
}