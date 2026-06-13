//! Hostname-allowlisting primitives for confining sandboxed network access.
//!
//! This crate grows over a short stack of PRs:
//!
//! - [`allowlist`]: the policy types ([`HostPattern`], [`Allowlist`]) that
//!   decide which hosts a sandboxed command may reach.
//! - [`UpstreamProxy`]: parsing an upstream HTTP proxy from the environment
//!   (`HTTPS_PROXY` / `NO_PROXY` etc.) to chain through.
//! - the proxy server itself (next): an in-process HTTP/HTTPS proxy that
//!   enforces an [`Allowlist`] and is the only network egress a sandboxed
//!   command is permitted.

mod allowlist;
mod proxy;

pub use allowlist::{Allowlist, HostPattern, HostPatternError};
pub use proxy::UpstreamProxy;
