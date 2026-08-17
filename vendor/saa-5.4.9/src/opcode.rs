//! Primitive synchronization operation types.

use crate::wait_queue::WaitQueue;

/// Operation types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Opcode {
    /// Acquires exclusive ownership.
    Exclusive,
    /// Acquires shared ownership.
    Shared,
    /// Barrier operation where the boolean flag indicates whether the operation is wait-only.
    Barrier(bool),
    /// Acquires semaphores.
    Semaphore(u8),
    /// Waits until the desired resources are available.
    Wait(u8),
}

impl Opcode {
    /// Checks if the resource expressed in `self` can be released from `state`.
    #[inline]
    pub(crate) const fn can_release(self, state: usize) -> bool {
        match self {
            Opcode::Exclusive => {
                let data = state & WaitQueue::DATA_MASK;
                data == WaitQueue::DATA_MASK
            }
            Opcode::Shared => {
                let data = state & WaitQueue::DATA_MASK;
                data >= 1 && data != WaitQueue::DATA_MASK
            }
            Opcode::Semaphore(count) => {
                let data = state & WaitQueue::DATA_MASK;
                let count = count as usize;
                data >= count
            }
            Opcode::Barrier(_) | Opcode::Wait(_) => true,
        }
    }

    /// Converts the operation mode into a `usize` value representing the desired resources.
    #[inline]
    pub(crate) const fn desired_count(self) -> usize {
        match self {
            Opcode::Exclusive => WaitQueue::DATA_MASK,
            Opcode::Shared => 1,
            Opcode::Barrier(wait_only) => {
                if wait_only {
                    0
                } else {
                    1
                }
            }
            Opcode::Semaphore(count) | Opcode::Wait(count) => {
                let count = count as usize;
                debug_assert!(count < WaitQueue::LOCKED_FLAG);
                count
            }
        }
    }

    /// Converts the operation mode into a `usize` value representing the resource held by the
    /// corresponding synchronization primitive.
    #[inline]
    pub(crate) const fn acquired_count(self) -> usize {
        match self {
            Opcode::Exclusive => WaitQueue::DATA_MASK,
            Opcode::Shared => 1,
            Opcode::Semaphore(count) => {
                let count = count as usize;
                debug_assert!(count < WaitQueue::LOCKED_FLAG);
                count
            }
            Opcode::Barrier(_) | Opcode::Wait(_) => 0,
        }
    }
}
