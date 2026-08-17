use core::ops::{Deref, DerefMut};
use smallvec::{smallvec, SmallVec};

use super::PollState;

/// The maximum number of entries that `PollStates` can store without
/// dynamic memory allocation.
///
/// The heap variant is the minimum size the data structure can have.
/// It consists of a boxed slice (=2 usizes) and space for the enum
/// tag (another usize because of padding), so 3 usizes.
/// The inline variant then consists of `3 * size_of(usize) - 2` entries.
/// Each entry is a byte and we subtract one byte for a length field,
/// and another byte for the enum tag.
///
/// ```txt
///                                 Boxed
///                                 vvvvv
/// tag
///  | <-------padding----> <--- Box<[T]>::len ---> <--- Box<[T]>::ptr --->
/// 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23  <bytes
///  |  | <------------------- entries ----------------------------------->
/// tag |
///    len                          ^^^^^
///                                 Inline
/// ```
const MAX_INLINE_ENTRIES: usize = core::mem::size_of::<usize>() * 3 - 2;

#[derive(Default)]
pub(crate) struct PollVec(SmallVec<[PollState; MAX_INLINE_ENTRIES]>);

impl PollVec {
    pub(crate) fn new(len: usize) -> Self {
        Self(smallvec![PollState::None; len])
    }

    pub(crate) fn new_pending(len: usize) -> Self {
        Self(smallvec![PollState::Pending; len])
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
    #[allow(unused)]
    pub(crate) fn pending_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter()
            .cloned()
            .enumerate()
            .filter(|(_, state)| state.is_pending())
            .map(|(i, _)| i)
    }

    /// Get an iterator of indexes of all items which are "consumed".
    #[allow(unused)]
    pub(crate) fn consumed_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter()
            .cloned()
            .enumerate()
            .filter(|(_, state)| state.is_none())
            .map(|(i, _)| i)
    }

    /// Mark all items as "pending"
    #[inline]
    pub(crate) fn set_all_pending(&mut self) {
        self.0.fill(PollState::Pending);
    }

    /// Mark all items as "none"
    #[inline]
    #[allow(unused)]
    pub(crate) fn set_all_none(&mut self) {
        self.0.fill(PollState::None);
    }

    /// Resize the `PollVec`
    pub(crate) fn resize(&mut self, len: usize) {
        self.0.resize_with(len, || PollState::None)
    }
}

impl Deref for PollVec {
    type Target = [PollState];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PollVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{PollVec, MAX_INLINE_ENTRIES};

    #[test]
    fn type_size() {
        // PollVec is three words plus two bits
        assert_eq!(
            core::mem::size_of::<PollVec>(),
            core::mem::size_of::<usize>() * 4
        );
    }

    #[test]
    fn boxed_does_not_allocate_twice() {
        // Make sure the debug_assertions in PollStates::new() don't fail.
        let _ = PollVec::new_pending(MAX_INLINE_ENTRIES + 10);
    }
}
