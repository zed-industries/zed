//! A small and fast async runtime.
//!
//! This crate simply re-exports other smaller async crates (see the source).
//!
//! To use tokio-based libraries with smol, apply the [`async-compat`] adapter to futures and I/O
//! types.
//!
//! See the [`smol-macros`] crate if you want a no proc-macro, fast compiling, easy-to-use
//! async main and/or multi-threaded Executor setup out of the box.
//!
//! # Examples
//!
//! Connect to an HTTP website, make a GET request, and pipe the response to the standard output:
//!
//! ```
//! use smol::{io, net, prelude::*, Unblock};
//!
//! fn main() -> io::Result<()> {
//!     smol::block_on(async {
//!         let mut stream = net::TcpStream::connect("example.com:80").await?;
//!         let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
//!         stream.write_all(req).await?;
//!
//!         let mut stdout = Unblock::new(std::io::stdout());
//!         io::copy(stream, &mut stdout).await?;
//!         Ok(())
//!     })
//! }
//! ```
//!
//! There's a lot more in the [examples] directory.
//!
//! [`async-compat`]: https://docs.rs/async-compat/latest/async_compat/
//! [`smol-macros`]: https://docs.rs/smol-macros/latest/smol_macros/
//! [examples]: https://github.com/smol-rs/smol/tree/master/examples

#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[doc(inline)]
pub use {
    async_executor::{Executor, LocalExecutor, Task},
    async_io::{block_on, Async, Timer},
    blocking::{unblock, Unblock},
    futures_lite::{future, io, pin, prelude, ready, stream},
};

#[doc(inline)]
pub use {async_channel as channel, async_fs as fs, async_lock as lock, async_net as net};

#[cfg(not(target_os = "espidf"))]
#[doc(inline)]
pub use async_process as process;

mod spawn;
pub use spawn::spawn;
