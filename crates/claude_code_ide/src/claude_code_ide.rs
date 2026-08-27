//! Makes Zed a recognised Claude Code "IDE", per workspace.
//!
//! The CLI finds its host editor by scanning `~/.claude/ide/` for a `<port>.lock`
//! file, then opening a loopback WebSocket to that port and speaking MCP. The VS
//! Code and JetBrains integrations implement the same contract. [`lockfile`] writes
//! the discovery file, [`server`] serves the protocol, [`selection`] answers the
//! selection tools and turns editor selection changes into notifications,
//! [`selection_socket`] takes selections from programs in the workspace's terminals,
//! and [`bridge`] owns the lifecycle.
//!
//! Anthropic documents what the integrations do, including the lock file, but not the
//! wire protocol. This implements it as reverse-engineered by the Neovim port,
//! corrected where the real CLI disagreed with that write-up.
//!
//! See:
//! - <https://code.claude.com/docs/en/ide-integrations>
//! - <https://github.com/coder/claudecode.nvim/blob/main/PROTOCOL.md>

pub mod bridge;
pub mod lockfile;
pub mod peer_cred;
pub mod protocol;
pub mod selection;
pub mod selection_socket;
pub mod server;

pub use bridge::init;

#[cfg(test)]
mod bridge_tests;
#[cfg(test)]
mod server_tests;
