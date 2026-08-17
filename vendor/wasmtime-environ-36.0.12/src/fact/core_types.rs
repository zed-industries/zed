use crate::prelude::*;
use std::collections::HashMap;
use wasm_encoder::{TypeSection, ValType};

/// A simple representation of the type section which automatically intern's
/// types and ensures they're only defined once.
#[derive(Default)]
pub struct CoreTypes {
    pub section: TypeSection,
    intern: HashMap<(Vec<ValType>, Vec<ValType>), u32>,
}

impl CoreTypes {
    pub fn function(&mut self, params: &[ValType], results: &[ValType]) -> u32 {
        *self
            .intern
            .entry((params.to_vec(), results.to_vec()))
            .or_insert_with(|| {
                let idx = self.section.len();
                self.section
                    .ty()
                    .function(params.iter().copied(), results.iter().copied());
                idx
            })
    }
}
