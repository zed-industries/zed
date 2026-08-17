//! Examples on how to use [`nvim-rs`](crate).
//!
//! The code in question is in the `examples` directory of the project. The
//! files in `src/examples/` contain the documentation.
//!
//! # Contents
//!
//! ### `handler_drop`
//!
//! An example showing how to implement cleanup-logic by implementing
//! [`Drop`](std::ops::Drop) for the [`handler`](crate::rpc::handler::Handler).
//!
//! ### `quitting`
//!
//! An example showing how to handle quitting in a plugin by catching a [`closed
//! channel`](crate::error::CallError::is_channel_closed).
//!
//!
//! ## `scorched_earth`
//!
//! A port of a real existing plugin.
//!
//! ## `scorched_earth_as`
//!
//! A port of the `scorched_earth` example to `async-std`.
//!
//! ## `bench_*`
//!
//! Some crude benchmarks to measure performance. After running
//!
//! ```sh
//! cargo build --examples --features use_tokio --release
//! cargo build --examples --features use_async-std --release
//! cargo build --examples --features use_neovim_lib --release
//! ```
//!
//! (the features aren't all compatible, so you need to run those separately
//! indeed) you can run `nvim -u bench_examples.vim`, and after so and so long
//! get a table in a modified buffer that tells you some numbers.
//!
//! The benchmarks of `tokio` and `async-std` should be pretty comparable, but
//! note that tokio's runtime takes parameters that influence performance.
//! Tweaking those, I found the runtimes don't differ by much.
//!
//! The benchmark of `neovim_lib` (called `bench_sync`) can't be designed the
//! way the others are, since they use nested requests. I tried to get around
//! that somewhat sneakily, but it's not 100% clear those benchmarks are
//! equivalent (but, if anything, they should favor neovim-lib a tad).
pub mod handler_drop;
pub mod quitting;
pub mod scorched_earth;
pub mod scorched_earth_as;
