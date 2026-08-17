//! Stubs for when profiling is disabled to have the "executing_pc" field
//! basically compiled away.

use core::marker;

#[derive(Default, Clone)]
pub(crate) struct ExecutingPc;

impl ExecutingPc {
    pub(crate) fn as_ref(&self) -> ExecutingPcRef<'_> {
        ExecutingPcRef {
            _marker: marker::PhantomData,
        }
    }

    pub(crate) fn set_done(&self) {}
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct ExecutingPcRef<'a> {
    _marker: marker::PhantomData<&'a ()>,
}

impl ExecutingPcRef<'_> {
    pub(crate) fn record(&self, pc: usize) {
        let _ = pc;
    }
}
