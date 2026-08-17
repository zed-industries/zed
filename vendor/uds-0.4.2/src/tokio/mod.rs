//! Provides support for `SEQPACKET` sockets in Tokio.
//!
//! This requires this librarys `tokio` feature to be enabled.  
//! See the README for example of how to enable it.

mod seqpacket;
pub use seqpacket::*;
mod traits;
pub use traits::*;
