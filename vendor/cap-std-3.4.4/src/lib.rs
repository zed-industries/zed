//! A capability-based API modeled after [`std`].
//!
//! This corresponds to [`std`].
//!
//! Capability-based APIs represent access to external resources as values
//! which can be passed around between different parts of a program.
//!
//! Two notable features are the [`Dir`] and [`Pool`] types:
//!  - `Dir` represents an open directory in a filesystem. Instead of opening
//!    files by absolute paths or paths relative to the current working
//!    directory, files are opened via paths relative to a `Dir`. The concepts
//!    of a process-wide "current working directory" and a single global
//!    filesystem namespace are de-emphasized.
//!  - `Pool` represents a set of network addresses. Instead of allowing
//!    applications to request access to any address and then applying
//!    process-wide filtering rules, filtering rules are built into pools which
//!    may be passed through the program.
//!
//! On WASI, use of this library closely reflects the underlying system
//! API, so it avoids compatibility layers.
//!
//! [`Dir`]: fs::Dir
//! [`Pool`]: net::Pool

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(can_vector, feature(can_vector))]
#![cfg_attr(write_all_vectored, feature(write_all_vectored))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

pub mod fs;
#[cfg(feature = "fs_utf8")]
pub mod fs_utf8;
#[cfg(not(target_os = "wasi"))] // Disable `net` on WASI until it has networking support.
pub mod net;
pub mod os;
pub mod time;
// Re-export ambient_authority etc. so that users can use our version.
#[doc(hidden)]
pub use cap_primitives::ambient_authority_known_at_compile_time;
pub use cap_primitives::{ambient_authority, AmbientAuthority};
// And these are also part of our public API
pub use cap_primitives::ipnet;
pub use io_lifetimes;
