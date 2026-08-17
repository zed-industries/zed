/// Enumerate the current poll state.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum PollState {
    /// There is no associated future or stream.
    /// This can be because no item was placed to begin with, or because there
    /// are was previously an item but there no longer is.
    None,
    /// Polling the associated future or stream.
    Pending,
    /// Data has been written to the output structure, and is now ready to be
    /// read.
    Ready,
}

impl PollState {
    /// Returns `true` if the metadata is [`None`][Self::None].
    #[must_use]
    #[inline]
    #[allow(unused)]
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the metadata is [`Pending`][Self::Pending].
    #[must_use]
    #[inline]
    pub(crate) fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if the poll state is [`Ready`][Self::Ready].
    #[must_use]
    #[inline]
    pub(crate) fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Sets the poll state to [`None`][Self::None].
    #[inline]
    pub(crate) fn set_none(&mut self) {
        *self = PollState::None;
    }

    /// Sets the poll state to [`Ready`][Self::Pending].
    #[inline]
    #[allow(unused)]
    pub(crate) fn set_pending(&mut self) {
        *self = PollState::Pending;
    }

    /// Sets the poll state to [`Ready`][Self::Ready].
    #[inline]
    pub(crate) fn set_ready(&mut self) {
        *self = PollState::Ready;
    }
}
