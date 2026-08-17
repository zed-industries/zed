//! Diagnostic reporting support for the codespan crate.

#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

// for no_std
extern crate alloc;

pub mod diagnostic;
pub mod files;
pub mod term;
