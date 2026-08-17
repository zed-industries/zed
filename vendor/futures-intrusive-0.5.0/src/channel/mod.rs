//! Asynchronous channels.
//!
//! This module provides various channels that can be used to communicate between
//! asynchronous tasks.

mod error;
pub use self::error::{ChannelSendError, TryReceiveError, TrySendError};

mod channel_future;
use channel_future::{
    ChannelReceiveAccess, ChannelSendAccess, RecvPollState, RecvWaitQueueEntry,
    SendPollState, SendWaitQueueEntry,
};
pub use channel_future::{
    ChannelReceiveFuture, ChannelSendFuture, CloseStatus,
};

mod oneshot;

pub use self::oneshot::{GenericOneshotChannel, LocalOneshotChannel};

#[cfg(feature = "std")]
pub use self::oneshot::OneshotChannel;

mod oneshot_broadcast;

pub use self::oneshot_broadcast::{
    GenericOneshotBroadcastChannel, LocalOneshotBroadcastChannel,
};

#[cfg(feature = "std")]
pub use self::oneshot_broadcast::OneshotBroadcastChannel;

mod state_broadcast;
pub use state_broadcast::{
    GenericStateBroadcastChannel, LocalStateBroadcastChannel, StateId,
    StateReceiveFuture,
};

#[cfg(feature = "std")]
pub use self::state_broadcast::StateBroadcastChannel;

mod mpmc;

pub use self::mpmc::{
    ChannelStream, GenericChannel, LocalChannel, LocalUnbufferedChannel,
};

#[cfg(feature = "std")]
pub use self::mpmc::{Channel, UnbufferedChannel};

#[cfg(feature = "alloc")]
mod if_alloc {

    /// Channel implementations where Sender and Receiver sides are cloneable
    /// and owned.
    /// The Futures produced by channels in this module don't require a lifetime
    /// parameter.
    pub mod shared {
        pub use super::super::channel_future::shared::*;
        pub use super::super::mpmc::shared::*;
        pub use super::super::oneshot::shared::*;
        pub use super::super::oneshot_broadcast::shared::*;
        pub use super::super::state_broadcast::shared::*;
    }
}

#[cfg(feature = "alloc")]
pub use self::if_alloc::*;
