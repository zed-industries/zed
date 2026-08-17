use super::CallThreadState;
use crate::prelude::*;
use crate::runtime::vm::{Backtrace, VMStoreContext};
use wasm_encoder::CoreDumpValue;

/// A WebAssembly Coredump
#[derive(Debug)]
pub struct CoreDumpStack {
    /// The backtrace containing the stack frames for the CoreDump
    pub bt: Backtrace,

    /// The locals for each frame in the backtrace.
    ///
    /// This is not currently implemented.
    #[expect(dead_code, reason = "not implemented yet")]
    pub locals: Vec<Vec<CoreDumpValue>>,

    /// The operands for each stack frame
    ///
    /// This is not currently implemented.
    #[expect(dead_code, reason = "not implemented yet")]
    pub operand_stack: Vec<Vec<CoreDumpValue>>,
}

impl CallThreadState {
    pub(super) fn capture_coredump(
        &self,
        vm_store_context: *const VMStoreContext,
        trap_pc_and_fp: Option<(usize, usize)>,
    ) -> Option<CoreDumpStack> {
        if !self.capture_coredump {
            return None;
        }
        let bt = unsafe {
            Backtrace::new_with_trap_state(vm_store_context, self.unwinder, self, trap_pc_and_fp)
        };

        Some(CoreDumpStack {
            bt,
            locals: vec![],
            operand_stack: vec![],
        })
    }
}
