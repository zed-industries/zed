//! In-process HTTP/HTTPS proxy that enforces a hostname allowlist.
//!
//! Spawned per terminal command from the parent process. The sandbox is
//! configured to permit network only to this proxy's port; everything the
//! sandboxed command tries to reach the network for has to come through here.
//!
//! The proxy:
//!
//! - Speaks HTTP CONNECT for HTTPS tunnels and HTTP forward proxying for
//!   plain HTTP. Other protocols cannot reach it (the seatbelt rule limits
//!   the sandboxed process to this one TCP destination, and this proxy only
//!   speaks HTTP).
//! - Checks the destination hostname against an allowlist of exact hostnames
//!   and leading-`*.` subdomain wildcards. Unless the allowlist allows any
//!   host, IP-literal targets are denied, and hostnames whose DNS resolves
//!   only into loopback / private / link-local space are denied too
//!   (DNS-rebinding protection — the proxy runs outside the sandbox, so it
//!   must not reopen the local network the Seatbelt rule closed off).
//! - Pins each TCP connection to the destination approved for its first
//!   request: directly (to the vetted resolved addresses) or via a CONNECT
//!   tunnel through an optional upstream HTTP proxy from the parent's
//!   environment (`HTTPS_PROXY` / `HTTP_PROXY`), honoring `NO_PROXY`. Plain
//!   HTTP is also tunneled when chaining, so keep-alive requests after the
//!   first can never be routed to a different host by the upstream.
//! - Reports per-connection events (allowed, denied, completed) over an
//!   mpsc supplied by the caller.
//!
//! ## Trust assumptions
//!
//! The proxy's sole client is model-driven code running inside the sandbox —
//! exactly the party the sandbox distrusts — and the proxy itself runs inside
//! the editor process. It therefore caps request header sizes and concurrent
//! connections, and bounds connect/handshake waits with timeouts, so a
//! malicious command can't exhaust the editor's memory, threads, or file
//! descriptors through it. Bandwidth is deliberately not capped; the
//! command's lifetime bounds it.
//!
//! ## "No proxy here" principle
//!
//! The agent and tools running inside the sandbox should not need to know
//! that a proxy is in front of them. The only response code the proxy
//! synthesizes itself is `511 Network Authentication Required`, used solely
//! for policy denials (with `Via:` and `Proxy-Status:` headers and a
//! plain-text body explaining the policy decision). Other failure modes
//! (upstream connection failure, malformed input from the client, etc.) are
//! handled by silently closing the connection — same behavior the client
//! would see from a direct network failure, no proxy fingerprint.

mod allowlist;
mod proxy;

pub use allowlist::{Allowlist, HostPattern, HostPatternError};
pub use proxy::{
    DenyReason, ProxyConfig, ProxyEvent, ProxyHandle, RequestMethod, RequestOutcome, UpstreamProxy,
};
