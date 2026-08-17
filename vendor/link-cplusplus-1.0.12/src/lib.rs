//! [![github]](https://github.com/dtolnay/link-cplusplus)&ensp;[![crates-io]](https://crates.io/crates/link-cplusplus)&ensp;[![docs-rs]](https://docs.rs/link-cplusplus)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! # `-lstdc++` or `-lc++`
//!
//! This crate exists for the purpose of passing `-lstdc++` or `-lc++` to the
//! linker, while making it possible for an application to make that choice on
//! behalf of its library dependencies.
//!
//! Without this crate, a library would need to:
//!
//! - pick one or the other to link, with no way for downstream applications to
//!   override the choice;
//! - or link neither and require an explicit link flag provided by downstream
//!   applications even if they would be fine with a default choice;
//!
//! neither of which are good experiences.
//!
//! <br>
//!
//! # Options
//!
//! An application or library that is fine with either of libstdc++ or libc++
//! being linked, whichever is the platform's default, should use the following
//! in Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! link-cplusplus = "1"
//! ```
//!
//! An application that wants a particular one or the other linked should use:
//!
//! ```toml
//! [dependencies]
//! link-cplusplus = { version = "1", features = ["libstdc++"] }
//!
//! # or
//!
//! link-cplusplus = { version = "1", features = ["libc++"] }
//! ```
//!
//! An application that wants to handle its own more complicated logic for link
//! flags from its build script can make this crate do nothing by using:
//!
//! ```toml
//! [dependencies]
//! link-cplusplus = { version = "1", features = ["nothing"] }
//! ```
//!
//! Lastly, make sure to add an explicit `extern crate` dependency to your crate
//! root, since the link-cplusplus crate will be otherwise unused and its link
//! flags dropped.
//!
//! ```
//! // src/lib.rs
//!
//! extern crate link_cplusplus;
//! ```

#![no_std]
