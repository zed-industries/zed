#![cfg(not(loom))]

//! UDP framing

mod frame;
pub use frame::UdpFramed;
