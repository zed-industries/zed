//! Internal types for stdio.
//!
//! This module is a port of `libstd/io/stdio.rs`,and contains internal types for `print`/`eprint`.

use crate::io::{stderr, stdout};
use crate::prelude::*;
use std::fmt;

#[doc(hidden)]
pub async fn _print(args: fmt::Arguments<'_>) {
    if let Err(e) = stdout().write_fmt(args).await {
        panic!("failed printing to stdout: {}", e);
    }
}

#[doc(hidden)]
pub async fn _eprint(args: fmt::Arguments<'_>) {
    if let Err(e) = stderr().write_fmt(args).await {
        panic!("failed printing to stderr: {}", e);
    }
}
