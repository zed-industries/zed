//! Module used to encode failure messages associated with traps in an adapter
//! module.
//!
//! This module is a bit forward-looking in an attempt to help assist with
//! debugging issues with adapter modules and their implementation. This isn't
//! actually wired up to any decoder at this time and may end up getting deleted
//! entirely depending on how things go.
//!
//! Currently in core wasm the `unreachable` instruction and other traps have no
//! ability to assign failure messages with traps. This means that if an adapter
//! fails all you have is the program counter into the wasm function, but
//! there's not actually any source corresponding to wasm adapters either. This
//! module is an attempt to assign optional string messages to `unreachable`
//! trap instructions so, when sufficient debugging options are enabled, these
//! trap messages could be displayed instead of a bland "unreachable" trap
//! message.
//!
//! This information is currently encoded as a custom section in the wasm
//! module.

use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use wasm_encoder::Encode;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum Trap {
    CannotLeave,
    CannotEnter,
    UnalignedPointer,
    InvalidDiscriminant,
    InvalidChar,
    ListByteLengthOverflow,
    StringLengthTooBig,
    StringLengthOverflow,
    AssertFailed(&'static str),
}

#[derive(Default)]
pub struct TrapSection {
    trap_to_index: HashMap<Trap, usize>,
    trap_list: Vec<Trap>,
    function_traps: Vec<(u32, Vec<(usize, usize)>)>,
}

impl TrapSection {
    /// Appends a list of traps found within a function.
    ///
    /// The `func` is the core wasm function index that is being described. The
    /// `traps` is a list of `(offset, trap)` where `offset` is the offset
    /// within the function body itself and `trap` is the description of the
    /// trap of the opcode at that offset.
    pub fn append(&mut self, func: u32, traps: Vec<(usize, Trap)>) {
        if traps.is_empty() {
            return;
        }

        // Deduplicate `Trap` annotations to avoid repeating the trap string
        // internally within the custom section.
        let traps = traps
            .into_iter()
            .map(|(offset, trap)| {
                let trap = *self.trap_to_index.entry(trap).or_insert_with(|| {
                    let idx = self.trap_list.len();
                    self.trap_list.push(trap);
                    idx
                });
                (offset, trap)
            })
            .collect();
        self.function_traps.push((func, traps));
    }

    /// Creates the custom section payload of this section to be encoded into a
    /// core wasm module.
    pub fn finish(self) -> Vec<u8> {
        let mut data = Vec::new();

        // First append all trap messages which will be indexed below.
        self.trap_list.len().encode(&mut data);
        for trap in self.trap_list.iter() {
            trap.to_string().encode(&mut data);
        }

        // Afterwards encode trap information for all known functions where
        // offsets are relative to the body of the function index specified and
        // the trap message is a pointer into the table built above this.
        self.function_traps.len().encode(&mut data);
        for (func, traps) in self.function_traps.iter() {
            func.encode(&mut data);
            traps.len().encode(&mut data);
            for (func_offset, trap_message) in traps {
                func_offset.encode(&mut data);
                trap_message.encode(&mut data);
            }
        }

        data
    }
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trap::CannotLeave => "cannot leave instance".fmt(f),
            Trap::CannotEnter => "cannot enter instance".fmt(f),
            Trap::UnalignedPointer => "pointer not aligned correctly".fmt(f),
            Trap::InvalidDiscriminant => "invalid variant discriminant".fmt(f),
            Trap::InvalidChar => "invalid char value specified".fmt(f),
            Trap::ListByteLengthOverflow => "byte size of list too large for i32".fmt(f),
            Trap::StringLengthTooBig => "string byte size exceeds maximum".fmt(f),
            Trap::StringLengthOverflow => "string byte size overflows i32".fmt(f),
            Trap::AssertFailed(s) => write!(f, "assertion failure: {s}"),
        }
    }
}
