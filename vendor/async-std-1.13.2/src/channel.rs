//! Channels
//!
//! Multi-producer, multi-consumer queues, used for message-based
//! communication. Can provide a lightweight inter-task synchronisation
//! mechanism, at the cost of some extra memory.

#[doc(inline)]
pub use async_channel::*;
