use karp_lex::EntryDto;
use std::collections::HashMap;

use super::ListSuperlemma;
use crate::{domain::Superlemma, Error};

pub struct InMemoryListSuperlemma {
    storage: Vec<EntryDto<Superlemma>>,
}

impl InMemoryListSuperlemma {
    pub fn new(storage: Vec<EntryDto<Superlemma>>) -> Self {
        Self { storage }
    }
}

impl ListSuperlemma for InMemoryListSuperlemma {
    fn query(&self, superlemman: &mut HashMap<String, EntryDto<Superlemma>>) -> Result<(), Error> {
        for entry in &self.storage {}
        Ok(())
    }
}
