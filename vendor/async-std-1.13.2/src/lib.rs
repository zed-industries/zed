#![allow(rustdoc::invalid_html_tags)]
//! # `async-std` has been discontinued; use `smol` instead
//!
//! We created `async-std` to demonstrate the value of making a library as close to
//! `std` as possible, but async. We think that demonstration was successful, and
//! we hope it will influence future design and development directions of async in
//! `std`. However, in the meantime, the [`smol`](https://github.com/smol-rs/smol/)
//! project came about and provided a great executor and libraries for asynchronous
//! use in the Rust ecosystem. We think that resources would be better spent
//! consolidating around `smol`, rather than continuing to provide occasional
//! maintenance of `async-std`. As such, we recommend that all users of
//! `async-std`, and all libraries built on `async-std`, switch to `smol` instead.
//!
//! In addition to the `smol` project as a direct replacement, you may find other
//! parts of the futures ecosystem useful, including `futures-concurrency`,
//! `async-io`, `futures-lite`, and `async-compat`.
//!
//! # Async version of the Rust standard library
//!
//! `async-std` is a foundation of portable Rust software, a set of minimal and battle-tested
//! shared abstractions for the [broader Rust ecosystem][crates.io]. It offers std types, like
//! [`Future`] and [`Stream`], library-defined [operations on language primitives](#primitives),
//! [standard macros](#macros), [I/O] and [multithreading], among [many other things][other].
//!
//! `async-std` is available from [crates.io]. Once included, `async-std` can be accessed
//! in [`use`] statements through the path `async_std`, as in [`use async_std::future`].
//!
//! [I/O]: io/index.html
//! [multithreading]: task/index.html
//! [other]: #what-is-in-the-standard-library-documentation
//! [`use`]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
//! [`use async_std::future`]: future/index.html
//! [crates.io]: https://crates.io
//! [`Future`]: future/trait.Future.html
//! [`Stream`]: stream/trait.Stream.html
//!
//! # How to read this documentation
//!
//! If you already know the name of what you are looking for, the fastest way to
//! find it is to use the <a href="#" onclick="focusSearchBar();">search
//! bar</a> at the top of the page.
//!
//! Otherwise, you may want to jump to one of these useful sections:
//!
//! * [`async_std::*` modules](#modules)
//! * [Async macros](#macros)
//! * [The Async Prelude](prelude/index.html)
//! * [Cargo.toml feature flags](#features)
//! * [Examples](#examples)
//!
//! If this is your first time, the documentation for `async-std` is
//! written to be casually perused. Clicking on interesting things should
//! generally lead you to interesting places. Still, there are important bits
//! you don't want to miss, so read on for a tour of the `async-std` and
//! its documentation!
//!
//! Once you are familiar with the contents of `async-std` you may
//! begin to find the verbosity of the prose distracting. At this stage in your
//! development you may want to press the `[-]` button near the top of the
//! page to collapse it into a more skimmable view.
//!
//! While you are looking at that `[-]` button also notice the `[src]`
//! button. Rust's API documentation comes with the source code and you are
//! encouraged to read it. The `async-std` source is generally high
//! quality and a peek behind the curtains is often enlightening.
//!
//! Modules in this crate are organized in the same way as in `std`, except blocking
//! functions have been replaced with async functions and threads have been replaced with
//! lightweight tasks.
//!
//! You can find more information, reading materials, and other resources here:
//!
//! * [The async-std website](https://async.rs/)
//! * [The async-std book](https://book.async.rs)
//! * [GitHub repository](https://github.com/async-rs/async-std)
//! * [List of code examples](https://github.com/async-rs/async-std/tree/HEAD/examples)
//! * [Discord chat](https://discord.gg/JvZeVNe)
//!
//! # What is in the `async-std` documentation?
//!
//! First, `async-std` is divided into a number of focused
//! modules, [all listed further down this page](#modules). These modules are
//! the bedrock upon which async Rust is forged, and they have mighty names
//! like [`async_std::os`] and [`async_std::task`]. Modules' documentation
//! typically includes an overview of the module along with examples, and are
//! a smart place to start familiarizing yourself with the library.
//!
//! Second, `async-std` defines [The Async Prelude], a small collection
//! of items - mostly traits - that should be imported into every module of
//! every async crate. The traits in the prelude are pervasive, making the
//! prelude documentation a good entry point to learning about the library.
//!
//! [The Async Prelude]: prelude/index.html
//! [`async_std::os`]: os/index.html
//! [`async_std::task`]: task/index.html
//!
//! And finally, `async-std` exports a number of async macros, and
//! [lists them on this page](#macros).
//!
//! # Contributing changes to the documentation
//!
//! Check out `async-std`'s contribution guidelines [here](https://async.rs/contribute).
//! The source for this documentation can be found on [GitHub](https://github.com/async-rs).
//! To contribute changes, make sure you read the guidelines first, then submit
//! pull requests for your suggested changes.
//!
//! Contributions are appreciated! If you see a part of the docs that can be
//! improved, submit a PR, or chat with us first on
//! [Discord](https://discord.gg/JvZeVNe).
//!
//! # A tour of `async-std`
//!
//! The rest of this crate documentation is dedicated to pointing out notable
//! features of `async-std`.
//!
//! ## Platform abstractions and I/O
//!
//! Besides basic data types, `async-std` is largely concerned with
//! abstracting over differences in common platforms, most notably Windows and
//! Unix derivatives.
//!
//! Common types of I/O, including [files], [TCP], [UDP], are defined in the
//! [`io`], [`fs`], and [`net`] modules.
//!
//! The [`task`] module contains `async-std`'s task abstractions. [`sync`]
//! contains further primitive shared memory types. [`channel`]  contains the channel types for message passing.
//!
//! [files]: fs/struct.File.html
//! [TCP]: net/struct.TcpStream.html
//! [UDP]: net/struct.UdpSocket.html
//! [`io`]: io/index.html
//! [`sync`]: sync/index.html
//! [`channel`]: channel/index.html
//!
//! ## Timeouts, intervals, and delays
//!
//! `async-std` provides several methods to manipulate time:
//!
//! * [`task::sleep`] to wait for a duration to pass without blocking.
//! * [`stream::interval`] for emitting an event at a set interval.
//! * [`future::timeout`] to time-out futures if they don't resolve within a
//!   set interval.
//!
//! [`task::sleep`]: task/fn.sleep.html
//! [`stream::interval`]: stream/fn.interval.html
//! [`future::timeout`]: future/fn.timeout.html
//!
//! # Examples
//!
//! All examples require the [`"attributes"` feature](#features) to be enabled.
//! This feature is not enabled by default because it significantly impacts
//! compile times. See [`task::block_on`] for an alternative way to start
//! executing tasks.
//!
//! Call an async function from the main function:
//!
#![cfg_attr(feature = "attributes", doc = "```")]
#![cfg_attr(not(feature = "attributes"), doc = "```ignore")]
//! async fn say_hello() {
//!     println!("Hello, world!");
//! }
//!
//! #[async_std::main]
//! async fn main() {
//!     say_hello().await;
//! }
//! ```
//!
//! Await two futures concurrently, and return a tuple of their output:
//!
#![cfg_attr(feature = "attributes", doc = "```")]
#![cfg_attr(not(feature = "attributes"), doc = "```ignore")]
//! use async_std::prelude::*;
//!
//! #[async_std::main]
//! async fn main() {
//!     let a = async { 1u8 };
//!     let b = async { 2u8 };
//!     assert_eq!(a.join(b).await, (1u8, 2u8))
//! }
//! ```
//!
//! Create a UDP server that echoes back each received message to the sender:
//!
#![cfg_attr(feature = "attributes", doc = "```no_run")]
#![cfg_attr(not(feature = "attributes"), doc = "```ignore")]
//! use async_std::net::UdpSocket;
//!
//! #[async_std::main]
//! async fn main() -> std::io::Result<()> {
//!     let socket = UdpSocket::bind("127.0.0.1:8080").await?;
//!     println!("Listening on {}", socket.local_addr()?);
//!
//!     let mut buf = vec![0u8; 1024];
//!
//!     loop {
//!         let (recv, peer) = socket.recv_from(&mut buf).await?;
//!         let sent = socket.send_to(&buf[..recv], &peer).await?;
//!         println!("Sent {} out of {} bytes to {}", sent, recv, peer);
//!     }
//! }
//! ```
//! [`task::block_on`]: task/fn.block_on.html
//!
//! # Features
//!
//! Items marked with
//! <span
//!   class="module-item stab portability"
//!   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
//! > <code>unstable</code> </span>
//! are available only when the `unstable` Cargo feature is enabled:
//!
//! ```toml
//! [dependencies.async-std]
//! version = "1.7.0"
//! features = ["unstable"]
//! ```
//!
//! Items marked with
//! <span
//!   class="module-item stab portability"
//!   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
//! > <code>attributes</code> </span>
//! are available only when the `attributes` Cargo feature is enabled:
//!
//! ```toml
//! [dependencies.async-std]
//! version = "1.7.0"
//! features = ["attributes"]
//! ```
//!
//! Compatibility with the `tokio` 1.0 runtime is also simultaneously possible
//! using the `tokio1` Cargo feature:
//!
//! ```toml
//! [dependencies.async-std]
//! version = "1.7.0"
//! features = ["tokio1"]
//! ```
//!
//! Compatibility with the `tokio` 0.2 runtime is possible using the `tokio02`
//! Cargo feature:
//!
//! ```toml
//! [dependencies.async-std]
//! version = "1.7.0"
//! features = ["tokio02"]
//! ```
//!
//! Compatibility with the `tokio` 0.3 runtime is also simultaneously possible
//! using the `tokio03` Cargo feature:
//!
//! ```toml
//! [dependencies.async-std]
//! version = "1.7.0"
//! features = ["tokio03"]
//! ```
//!
//! Additionally it's possible to only use the core traits and combinators by
//! only enabling the `std` Cargo feature:
//!
//! ```toml
//! [dependencies.async-std]
//! version = "1.7.0"
//! default-features = false
//! features = ["std"]
//! ```
//!
//! And to use async-std on `no_std` targets that only support `alloc` only
//! enable the `alloc` Cargo feature:
//!
//! ```toml
//! [dependencies.async-std]
//! version = "1.7.0"
//! default-features = false
//! features = ["alloc"]
//! ```
//!
//! # Runtime configuration
//!
//! Several environment variables are available to tune the async-std
//! runtime:
//!
//! * `ASYNC_STD_THREAD_COUNT`: The number of threads that the
//! async-std runtime will start. By default, this is one per logical
//! cpu as determined by [async-global-executor](async_global_executor),
//! which may be different than the number of physical cpus. Async-std
//! _will panic_ if this is set to any value other than a positive
//! integer.
//! * `ASYNC_STD_THREAD_NAME`: The name that async-std's runtime
//! threads report to the operating system. The default value is
//! `"async-std/runtime"`.
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::mutex_atomic, clippy::module_inception)]
#![doc(test(attr(deny(rust_2018_idioms))))]
#![doc(test(attr(allow(unused_extern_crates, unused_variables))))]
#![doc(html_logo_url = "https://async.rs/images/logo--hero.svg")]

#[macro_use]
mod utils;

#[cfg(feature = "attributes")]
#[cfg_attr(feature = "docs", doc(cfg(attributes)))]
#[doc(inline)]
pub use async_attributes::{main, test};

#[cfg(feature = "std")]
mod macros;

cfg_alloc! {
    pub mod task;
    pub mod future;
    pub mod stream;
}

cfg_std! {
    pub mod io;
    pub mod os;
    pub mod prelude;
    pub mod sync;
    pub mod channel;
}

cfg_default! {
    #[cfg(not(target_os = "unknown"))]
    pub mod fs;
    pub mod path;
    pub mod net;
    #[cfg(not(target_os = "unknown"))]
    pub(crate) mod rt;
}

cfg_unstable! {
    pub mod pin;
    #[cfg(all(not(target_os = "unknown"), feature = "std"))]
    pub mod process;

    mod unit;
    mod vec;
    mod result;
    mod option;
    mod string;
    mod collections;
}

cfg_unstable_default! {
    #[doc(inline)]
    pub use std::{write, writeln};
}
