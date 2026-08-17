//! Code generation library for Winch.

// Unless this library is compiled with `all-arch`, the rust compiler
// is going to emit dead code warnings.
#![cfg_attr(
    not(feature = "all-arch"),
    allow(
        dead_code,
        reason = "this is fine as long as we run CI at least once with the `all-arch` feature enabled"
    )
)]

mod abi;
pub use codegen::{BuiltinFunctions, FuncEnv};
mod codegen;
mod frame;
pub mod isa;
pub use isa::*;
mod constant_pool;
mod masm;
mod regalloc;
mod regset;
mod stack;
mod visitor;
