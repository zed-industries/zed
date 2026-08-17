//! Generation of the throw-stub.
//!
//! In order to throw exceptions from within Cranelift-compiled code,
//! we provide a runtime function helper meant to be called by host
//! code that is invoked by guest code.
//!
//! The helper below must be provided a delimited range on the stack
//! corresponding to Cranelift frames above the current host code. It
//! will look for any handlers in this code, given a closure that
//! knows how to use an absolute PC to look up a module's exception
//! table and its start-of-code-segment. If a handler is found, the
//! helper below will return the SP, FP and PC that must be
//! restored. Architecture-specific helpers are provided to jump to
//! this new context with payload values. Otherwise, if no handler is
//! found, the return type indicates this, and it is the caller's
//! responsibility to invoke alternative behavior (e.g., abort the
//! program or unwind all the way to initial Cranelift-code entry).

use crate::{ExceptionTable, Unwind};
use core::ops::ControlFlow;

/// Throw action to perform.
#[derive(Clone, Debug)]
pub enum ThrowAction {
    /// Jump to the given handler with the given SP and FP values.
    Handler {
        /// Program counter of handler return point.
        pc: usize,
        /// Stack pointer to restore before jumping to handler.
        sp: usize,
        /// Frame pointer to restore before jumping to handler.
        fp: usize,
    },
    /// No handler found.
    None,
}

/// Implementation of stack-walking to find a handler.
///
/// This function searches for a handler in the given range of stack
/// frames, starting from the throw stub and up to a specified entry
/// frame.
///
/// # Safety
///
/// The safety of this function is the same as [`crate::visit_frames`] where the
/// values passed in configuring the frame pointer walk must be correct and
/// Wasm-defined for this to not have UB.
pub unsafe fn compute_throw_action<'a, F: Fn(usize) -> Option<(usize, ExceptionTable<'a>)>>(
    unwind: &dyn Unwind,
    module_lookup: F,
    exit_pc: usize,
    exit_frame: usize,
    entry_frame: usize,
    tag: u32,
) -> ThrowAction {
    let mut last_fp = exit_frame;

    // SAFETY: the safety of `visit_frames` relies on the correctness of the
    // parameters passed in which is forwarded as a contract to this function
    // tiself.
    let result = unsafe {
        crate::stackwalk::visit_frames(unwind, exit_pc, exit_frame, entry_frame, |frame| {
            if let Some((base, table)) = module_lookup(frame.pc()) {
                let relative_pc = u32::try_from(
                    frame
                        .pc()
                        .checked_sub(base)
                        .expect("module lookup did not return a module base below the PC"),
                )
                .expect("module larger than 4GiB");

                if let Some(handler) = table.lookup(relative_pc, tag) {
                    let abs_handler_pc = base
                        .checked_add(usize::try_from(handler).unwrap())
                        .expect("Handler address computation overflowed");

                    return ControlFlow::Break(ThrowAction::Handler {
                        pc: abs_handler_pc,
                        sp: last_fp + unwind.next_older_sp_from_fp_offset(),
                        fp: frame.fp(),
                    });
                }
            }
            last_fp = frame.fp();
            ControlFlow::Continue(())
        })
    };
    match result {
        ControlFlow::Break(action) => action,
        ControlFlow::Continue(()) => ThrowAction::None,
    }
}
