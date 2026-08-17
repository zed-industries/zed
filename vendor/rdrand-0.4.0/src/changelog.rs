//! Project changelog

/// ## Breaking changes
///
/// Crate gained an enabled-by-default `std` feature. If you relied on rdrand being `core`-able
/// change your dependency to appear as such:
///
/// ```toml
/// rdrand = { version = "0.4", default-features = false }
/// ```
///
/// This is done so that an advantage of the common feature detection functionality could be
/// employed by users that are not constrained by `core`. This functionality is faster, caches the
/// results and is shared between all users of the functionality.
///
/// For `core` usage the feature detection has also been improved and will not be done if e.g.
/// crate is built with `rdrand` instructions enabled globally.
pub mod r0_4_0 {}

/// Crate now works on stable!
///
/// ## Breaking changes
///
/// * Updated to `rand_core = ^0.3`.
pub mod r0_3_0 {}
