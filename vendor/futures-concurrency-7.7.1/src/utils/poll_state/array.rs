use core::ops::{Deref, DerefMut};

use super::PollState;

pub(crate) struct PollArray<const N: usize> {
    state: [PollState; N],
}

impl<const N: usize> PollArray<N> {
    /// Create a new `PollArray` with all state marked as `None`
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            state: [PollState::None; N],
        }
    }

    /// Create a new `PollArray` with all state marked as `Pending`
    pub(crate) fn new_pending() -> Self {
        Self {
            state: [PollState::Pending; N],
        }
    }

    /// Mark all items as "pending"
    #[inline]
    pub(crate) fn set_all_pending(&mut self) {
        self.fill(PollState::Pending);
    }

    /// Mark all items as "none"
    #[inline]
    #[allow(unused)]
    pub(crate) fn set_all_none(&mut self) {
        self.fill(PollState::None);
    }

    /// Get an iterator of indexes of all items which are "ready".
    pub(crate) fn ready_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter()
            .cloned()
            .enumerate()
            .filter(|(_, state)| state.is_ready())
            .map(|(i, _)| i)
    }

    /// Get an iterator of indexes of all items which are "pending".
    pub(crate) fn pending_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter()
            .cloned()
            .enumerate()
            .filter(|(_, state)| state.is_pending())
            .map(|(i, _)| i)
    }
}

impl<const N: usize> Deref for PollArray<N> {
    type Target = [PollState];

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<const N: usize> DerefMut for PollArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
