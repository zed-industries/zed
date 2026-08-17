/// The error which is returned when sending a value into a channel fails.
///
/// The `send` operation can only fail if the channel has been closed, which
/// would prevent the other actors to ever retrieve the value.
///
/// The error recovers the value that has been sent.
#[derive(PartialEq, Debug)]
pub struct ChannelSendError<T>(pub T);

/// The error which is returned when trying to receive from a channel
/// without waiting fails.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TryReceiveError {
    /// The channel is empty. No value is available for reception.
    Empty,
    /// The channel had been closed and no more value is available for reception.
    Closed,
}

impl TryReceiveError {
    /// Returns whether the error is the `Empty` variant.
    pub fn is_empty(self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    /// Returns whether the error is the `Closed` variant.
    pub fn is_closed(self) -> bool {
        match self {
            Self::Closed => true,
            _ => false,
        }
    }
}

/// The error which is returned when trying to send on a channel
/// without waiting fails.
#[derive(PartialEq, Debug)]
pub enum TrySendError<T> {
    /// The channel is full.
    Full(T),
    /// The channel was closed.
    Closed(T),
}

impl<T> TrySendError<T> {
    /// Converts the error into its inner value.
    pub fn into_inner(self) -> T {
        match self {
            Self::Closed(inner) => inner,
            Self::Full(inner) => inner,
        }
    }

    /// Returns whether the error is the `WouldBlock` variant.
    pub fn is_full(&self) -> bool {
        match self {
            Self::Full(_) => true,
            _ => false,
        }
    }

    /// Returns whether the error is the `Closed` variant.
    pub fn is_closed(&self) -> bool {
        match self {
            Self::Closed(_) => true,
            _ => false,
        }
    }
}
