//! Support executing the interpreter loop through tail-calls rather than a
//! source-level `loop`.
//!
//! This is an alternative means of executing the interpreter loop of Pulley.
//! The other method is in `match_loop.rs` which is a `loop` over a `match`
//! (more-or-less). This file instead transitions between opcodes with
//! tail-calls.
//!
//! At this time this module is more performant but disabled by default. Rust
//! does not have guaranteed tail call elimination at this time so this is not
//! a suitable means of writing an interpreter loop. That being said this is
//! included nonetheless for us to experiment and analyze with.
//!
//! There are two methods of using this module:
//!
//! * `RUSTFLAGS=--cfg=pulley_assume_llvm_makes_tail_calls` - this compilation
//!   flag indicates that we should assume that LLVM will optimize to making
//!   tail calls for things that look like tail calls. Practically this
//!   probably only happens with `--release` and for popular native
//!   architectures. It's up to the person compiling to manually
//!   audit/verify/test that TCO is happening.
//!
//! * `RUSTFLAGS=--cfg=pulley_tail_calls` - this compilation flag indicates that
//!   Rust's nightly-only support for guaranteed tail calls should be used. This
//!   uses the `become` keyword, for example. At this time this feature of Rust
//!   is highly experimental and not even complete. It only passes `cargo check`
//!   at this time but doesn't actually run anywhere.

use super::*;
use crate::ExtendedOpcode;
use crate::decode::ExtendedOpVisitor;
use crate::opcode::Opcode;
use crate::profile::ExecutingPcRef;

/// ABI signature of each opcode handler.
///
/// Note that this "explodes" the internals of `Interpreter` to individual
/// arguments to help get them all into registers.
type Handler = fn(&mut MachineState, UnsafeBytecodeStream, ExecutingPcRef<'_>) -> Done;

/// The extra indirection through a macro is necessary to avoid a compiler error
/// when compiling without `#![feature(explicit_tail_calls)]` enabled (via
/// `--cfg pulley_tail_calls`).
///
/// It seems rustc first parses the function, encounters `become` and emits
/// an error about using an unstable keyword on a stable compiler, then applies
/// `#[cfg(...)` after parsing to disable the function.
///
/// Macro bodies are just bags of tokens; the body is not parsed until after
/// they are expanded, and this macro is only expanded when `pulley_tail_calls`
/// is enabled.
#[cfg(pulley_tail_calls)]
macro_rules! tail_call {
    ($e:expr) => {
        become $e
    };
}

#[cfg(pulley_assume_llvm_makes_tail_calls)]
macro_rules! tail_call {
    ($e:expr) => {
        return $e
    };
}

impl Interpreter<'_> {
    pub fn run(self) -> Done {
        dispatch(self.state, self.pc, self.executing_pc)
    }
}

fn debug<'a>(
    state: &'a mut MachineState,
    pc: UnsafeBytecodeStream,
    executing_pc: ExecutingPcRef<'a>,
) -> debug::Debug<'a> {
    debug::Debug(Interpreter {
        state,
        pc,
        executing_pc,
    })
}

fn dispatch(
    state: &mut MachineState,
    pc: UnsafeBytecodeStream,
    executing_pc: ExecutingPcRef<'_>,
) -> Done {
    // Perform a dynamic dispatch through a function pointer indexed by
    // opcode.
    let mut debug = debug(state, pc, executing_pc);
    debug.before_visit();
    let Ok(opcode) = Opcode::decode(debug.bytecode());
    let handler = OPCODE_HANDLER_TABLE[opcode as usize];
    tail_call!(handler(debug.0.state, debug.0.pc, debug.0.executing_pc));
}

/// Same as `Interpreter::run`, except for extended opcodes.
fn run_extended(
    state: &mut MachineState,
    pc: UnsafeBytecodeStream,
    pc_ref: ExecutingPcRef<'_>,
) -> Done {
    let mut i = debug(state, pc, pc_ref);
    let Ok(opcode) = ExtendedOpcode::decode(i.bytecode());
    let handler = EXTENDED_OPCODE_HANDLER_TABLE[opcode as usize];
    tail_call!(handler(i.0.state, i.0.pc, i.0.executing_pc));
}

static OPCODE_HANDLER_TABLE: [Handler; Opcode::MAX as usize + 1] = {
    macro_rules! define_opcode_handler_table {
        ($(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( {
                $(
                    $( #[$field_attr:meta] )*
                    $field:ident : $field_ty:ty
                ),*
            } )?;
        )*) => {
            [
                $($snake_name,)* // refers to functions defined down below
                run_extended,
            ]
        };
    }

    for_each_op!(define_opcode_handler_table)
};

// same as above, but without a +1 for handling of extended ops as this is the
// extended ops.
static EXTENDED_OPCODE_HANDLER_TABLE: [Handler; ExtendedOpcode::MAX as usize] = {
    macro_rules! define_extended_opcode_handler_table {
        ($(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( {
                $(
                    $( #[$field_attr:meta] )*
                    $field:ident : $field_ty:ty
                ),*
            } )?;
        )*) => {
            [
                $($snake_name,)* // refers to functions defined down below
            ]
        };
    }

    for_each_extended_op!(define_extended_opcode_handler_table)
};

// Define a top-level function for each opcode. Each function here is the
// destination of the indirect return-call-indirect of above. Each function is
// also specialized to a single opcode and should be thoroughly inlined to
// ensure that everything "boils away".
macro_rules! define_opcode_handler {
    ($(
        $( #[$attr:meta] )*
        $snake_name:ident = $name:ident $( {
            $(
                $( #[$field_attr:meta] )*
                $field:ident : $field_ty:ty
            ),*
        } )?;
    )*) => {$(
        fn $snake_name(
            state: &mut MachineState,
            pc: UnsafeBytecodeStream,
            executing_pc: ExecutingPcRef<'_>,
        ) -> Done {
            let mut debug = debug(state, pc, executing_pc);
            $(
                let Ok(($($field,)*)) = crate::decode::operands::$snake_name(debug.0.bytecode());
            )?
            let result = debug.$snake_name($($($field),*)?);
            debug.after_visit();
            match result {
                ControlFlow::Continue(()) => {
                    tail_call!(dispatch(debug.0.state, debug.0.pc, debug.0.executing_pc))
                }
                ControlFlow::Break(done) => done,
            }
        }
    )*};
}

for_each_op!(define_opcode_handler);
for_each_extended_op!(define_opcode_handler);
