//! Hostname-allowlisting primitives for confining sandboxed network access.
//!
//! This crate grows over a short stack of PRs:
//!
//! - [`allowlist`]: the policy types ([`HostPattern`], [`Allowlist`]) that
//!   decide which hosts a sandboxed command may reach.
//! - `upstream` (next): parsing an upstream HTTP proxy from the environment.
//! - the proxy server itself (last): an in-process HTTP/HTTPS proxy that
//!   enforces an [`Allowlist`] and is the only network egress a sandboxed
//!   command is permitted.

mod allowlist;

pub use allowlist::{Allowlist, HostPattern, HostPatternError};
