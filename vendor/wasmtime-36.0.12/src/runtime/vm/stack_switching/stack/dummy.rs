use anyhow::Result;
use core::ops::Range;

use crate::runtime::vm::stack_switching::VMHostArray;
use crate::runtime::vm::{VMContext, VMFuncRef, ValRaw};

/// Making sure that this has the same size as the non-dummy version, to
/// make some tests happy.
#[derive(Debug)]
#[repr(C)]
pub struct VMContinuationStack {
    _top: *mut u8,
    _len: usize,
    _match_size_on_unix: u8,
}

impl VMContinuationStack {
    pub fn new(_size: usize) -> Result<Self> {
        anyhow::bail!("Stack switching disabled or not implemented on this platform")
    }

    pub fn unallocated() -> Self {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub fn is_unallocated(&self) -> bool {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub unsafe fn from_raw_parts(_base: *mut u8, _guard_size: usize, _len: usize) -> Result<Self> {
        anyhow::bail!("Stack switching disabled or not implemented on this platform")
    }

    pub fn is_from_raw_parts(&self) -> bool {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub fn top(&self) -> Option<*mut u8> {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub fn range(&self) -> Option<Range<usize>> {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub fn control_context_instruction_pointer(&self) -> usize {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub fn control_context_frame_pointer(&self) -> usize {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub fn control_context_stack_pointer(&self) -> usize {
        panic!("Stack switching disabled or not implemented on this platform")
    }

    pub fn initialize(
        &self,
        _func_ref: *const VMFuncRef,
        _caller_vmctx: *mut VMContext,
        _args: *mut VMHostArray<ValRaw>,
        _parameter_count: u32,
        _return_value_count: u32,
    ) {
    }
}
