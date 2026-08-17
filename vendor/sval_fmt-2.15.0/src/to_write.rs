use crate::{
    writer::{GenericWriter, Writer},
    TokenWrite,
};
use core::fmt::{self, Write};

/**
Format a value into an underlying formatter.

This method will use a default format that's like Rust's `Debug`.
*/
pub fn stream_to_write(fmt: impl Write, v: impl sval::Value) -> fmt::Result {
    v.stream(&mut Writer::new(GenericWriter(fmt)))
        .map_err(|_| fmt::Error)
}

/**
Format a value into an underlying token-aware formatter.

This method is like [`stream_to_write`], but can be used to customize the way
values are formatted through the implementation of [`TokenWrite`].
*/
pub fn stream_to_token_write(fmt: impl TokenWrite, v: impl sval::Value) -> fmt::Result {
    v.stream(&mut Writer::new(fmt)).map_err(|_| fmt::Error)
}
