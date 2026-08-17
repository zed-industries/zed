// Prefer `ipnetwork` over `ipnet` because it was implemented first (want to avoid breaking change).
#[cfg(not(feature = "ipnetwork"))]
mod ipaddr;

// Parent module is named after the `ipnet` crate, this is named after the `IpNet` type.
#[allow(clippy::module_inception)]
mod ipnet;
