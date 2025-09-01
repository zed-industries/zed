//! Zed's debugger data layer is implemented in terms of 3 concepts:
//! - DAP store - that knows about all of the available debug sessions.
//! - Debug sessions - that bear responsibility of communicating with debug adapters and managing the state of each individual session.
//!   For the most part it is agnostic over the communication layer (it'll use RPC for peers and actual DAP requests for the host).
//! - Breakpoint store - that knows about all breakpoints set for a project.
//!
//! There are few reasons for this divide:
//! - Breakpoints persist across debug sessions and they're not really specific to any particular session. Sure, we have to send protocol messages for them
//!   (so they're a "thing" in the protocol), but we also want to set them before any session starts up.
//! - Debug clients are doing the heavy lifting, and this is where UI grabs all of it's data from. They also rely on breakpoint store during initialization to obtain
//!   current set of breakpoints.
//! - Since DAP store knows about all of the available debug sessions, it is responsible for routing RPC requests to sessions. It also knows how to find adapters for particular kind of session.

pub mod breakpoint_store;
pub mod dap_command;
pub mod dap_store;
pub mod locators;
mod memory;
pub mod session;

#[cfg(any(feature = "test-support", test))]
pub mod test;
pub use memory::MemoryCell;
