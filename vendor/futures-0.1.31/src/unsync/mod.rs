//! Future-aware single-threaded synchronization
//!
//! This module contains similar abstractions to `sync`, for communications
//! between tasks on the same thread only.

pub mod mpsc;
pub mod oneshot;
