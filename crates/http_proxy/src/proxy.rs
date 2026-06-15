//! The proxy module. For now it holds only the upstream-proxy configuration
//! type; the proxy server (listener, connection handling) lands in a later
//! PR.

mod upstream;

pub use upstream::UpstreamProxy;
