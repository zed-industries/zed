//! Actions crate that generates actions.json at build time.
//!
//! This crate uses build.rs to automatically regenerate the actions.json file
//! whenever the crate is built, eliminating the need for manual updates.

// This crate is primarily used for its build script
// The lib.rs is minimal as the main functionality is in build.rs
