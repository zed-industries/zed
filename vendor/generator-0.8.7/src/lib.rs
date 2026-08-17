//! # generator
//!
//! Rust generator library
//!

#![deny(warnings, missing_docs)]
#![allow(deprecated)]

#[macro_use]
extern crate log;

mod detail;
mod gen_impl;
mod reg_context;
mod rt;
mod scope;
mod stack;
mod yield_;

pub use crate::gen_impl::{Generator, Gn, LocalGenerator, DEFAULT_STACK_SIZE};
pub use crate::rt::{get_local_data, is_generator, Error};
pub use crate::scope::Scope;
pub use crate::yield_::{
    co_get_yield, co_set_para, co_yield_with, done, get_yield, yield_, yield_from, yield_with,
};
