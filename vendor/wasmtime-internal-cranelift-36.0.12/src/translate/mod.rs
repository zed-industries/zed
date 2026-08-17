//! Performs translation from a wasm module in binary format to the in-memory form
//! of Cranelift IR. More particularly, it translates the code of all the functions bodies and
//! interacts with an environment implementing the
//! [`ModuleEnvironment`](trait.ModuleEnvironment.html)
//! trait to deal with tables, globals and linear memory.
//!
//! The main function of this module is [`translate_module`](fn.translate_module.html).
//!
//! Note that this module used to be the `cranelift-wasm` crate historically and
//! it's in a transitionary period of being slurped up into
//! `wasmtime-cranelift`.

mod code_translator;
mod environ;
mod func_translator;
mod heap;
mod stack;
mod table;
mod translation_utils;

pub use self::environ::{GlobalVariable, StructFieldsVec, TargetEnvironment};
pub use self::func_translator::FuncTranslator;
pub use self::heap::{Heap, HeapData};
pub use self::stack::FuncTranslationStacks;
pub use self::table::{TableData, TableSize};
pub use self::translation_utils::*;
