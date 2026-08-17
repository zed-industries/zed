use super::{Frame, MaybeType, Vec};

#[derive(Clone, PartialEq)]
pub struct Transaction {}

#[derive(Clone, Default, PartialEq)]
pub struct RollbackLog {}

#[derive(Clone, Default, PartialEq)]
pub struct RollbackLogAllocations {}

impl Transaction {
    pub fn new(_: RollbackLogAllocations) -> Self {
        Self {}
    }
    pub fn begin(&mut self, _: usize) {}
    pub fn end(&mut self) {}
    pub fn into_allocations(self) -> RollbackLogAllocations {
        RollbackLogAllocations {}
    }
    pub fn map(&mut self, _: impl FnOnce(&mut RollbackLog)) {}
}

impl RollbackLog {
    pub fn record_push(&mut self) {}
    pub fn record_pop(&mut self, _: MaybeType) {}
    pub fn push_ctrl(&mut self) {}
    pub fn pop_ctrl(&mut self, _: Frame, _: Vec<u32>) {}
    pub fn set_unreachable(&mut self) {}
}
