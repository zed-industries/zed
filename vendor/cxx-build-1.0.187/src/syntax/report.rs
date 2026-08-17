use quote::ToTokens;
use std::fmt::Display;
use syn::{Error, Result};

pub(crate) struct Errors {
    errors: Vec<Error>,
}

impl Errors {
    pub(crate) fn new() -> Self {
        Errors { errors: Vec::new() }
    }

    pub(crate) fn error(&mut self, sp: impl ToTokens, msg: impl Display) {
        self.errors.push(Error::new_spanned(sp, msg));
    }

    pub(crate) fn push(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub(crate) fn propagate(&mut self) -> Result<()> {
        let mut iter = self.errors.drain(..);
        let Some(mut all_errors) = iter.next() else {
            return Ok(());
        };
        for err in iter {
            all_errors.combine(err);
        }
        Err(all_errors)
    }
}
