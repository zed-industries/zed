use super::{Frame, MaybeType, Vec};
use core::mem;

#[derive(Clone, PartialEq)]
pub enum Transaction {
    Active(RollbackLog),
    Inactive(RollbackLogAllocations),
}

#[derive(Clone, Default, PartialEq)]
pub struct RollbackLog {
    /// A trace of operands popped. Pushes are recorded as `None`.
    pub operands: Vec<Option<MaybeType>>,

    /// A trace of frames popped. Pushes are recorded as `None`.
    pub frames: Vec<Option<Frame>>,

    /// A trace of local init indices reset at the end of a frame.
    pub inits: Vec<u32>,

    /// The local init height when transaction was begun.
    pub init_height: usize,

    /// Whether the current frame was made unreachable.
    pub unreachable: bool,
}

#[derive(Clone, Default, PartialEq)]
pub struct RollbackLogAllocations {
    operands: Vec<Option<MaybeType>>,
    frames: Vec<Option<Frame>>,
    inits: Vec<u32>,
}

impl Transaction {
    pub fn new(allocs: RollbackLogAllocations) -> Self {
        Transaction::Inactive(allocs)
    }

    pub fn begin(&mut self, init_height: usize) {
        match self {
            Transaction::Active(_) => panic!("transaction already in progress"),
            Transaction::Inactive(allocs) => {
                *self = Transaction::Active(RollbackLog::new(init_height, mem::take(allocs)))
            }
        }
    }

    pub fn end(&mut self) {
        if let Transaction::Active(log) = self {
            *self = Transaction::Inactive(mem::take(log).into_allocations())
        }
    }

    pub fn into_allocations(self) -> RollbackLogAllocations {
        match self {
            Transaction::Active(log) => log.into_allocations(),
            Transaction::Inactive(allocs) => allocs,
        }
    }

    pub fn map(&mut self, f: impl FnOnce(&mut RollbackLog)) {
        if let Transaction::Active(log) = self {
            f(log);
        }
    }
}

impl RollbackLog {
    pub fn new(init_height: usize, allocs: RollbackLogAllocations) -> Self {
        let RollbackLogAllocations {
            operands,
            frames,
            inits,
        } = allocs;
        debug_assert!(operands.is_empty());
        debug_assert!(frames.is_empty());
        debug_assert!(inits.is_empty());
        Self {
            operands,
            frames,
            inits,
            init_height,
            unreachable: false,
        }
    }

    pub fn into_allocations(self) -> RollbackLogAllocations {
        fn clear<T>(mut tmp: Vec<T>) -> Vec<T> {
            tmp.clear();
            tmp
        }
        RollbackLogAllocations {
            operands: clear(self.operands),
            frames: clear(self.frames),
            inits: clear(self.inits),
        }
    }

    pub fn record_push(&mut self) {
        self.operands.push(None);
    }

    pub fn record_pop(&mut self, ty: MaybeType) {
        self.operands.push(Some(ty));
    }

    pub fn push_ctrl(&mut self) {
        self.frames.push(None);
    }

    pub fn pop_ctrl(&mut self, frame: Frame, inits: Vec<u32>) {
        self.frames.push(Some(frame));
        self.inits.extend(inits);
    }

    pub fn set_unreachable(&mut self) {
        self.unreachable = true;
    }
}
