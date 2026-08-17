//! # Scorched earth
//!
//! A minimal port of
//! [scorched earth by boxofrox](https://github.com/boxofrox/neovim-scorched-earth)
//! to nvim-rs. Works the same, but foregoes any error handling, removes the
//! customisation of color, and removes some abstractions that aren't helpfull
//! in an example.
//!
//! Note that this example uses `tokio`, while `scorched_earth_as` uses
//! async-std.
//!
//! ## Usage
//!
//! First, build this example via
//!
//! ```sh
//! cargo build --example scorched_earth
//! ```
//!
//! Then follow the steps described in the [`README`](README.md) for the examples.
//!
//! ## Description
//!
//! Some overview over the code:
//!
//! * The associated type for our [`Handler`](crate::rpc::handler::Handler) is
//! out stdout. But tokio's [`Stdout`](tokio::io::Stdout) does not implement
//! [`futures::io::AsyncWrite`](futures::io::AsyncWrite), so it needs to be
//! wrapped in the provided [`Compat`](crate::compat::tokio::Compat) type.
//!
//! * The handler struct `NeovimHandler` needs to contain some plugin state,
//! namely two cursor positions `start` and `end`. It needs to be `Send` and
//! `Sync`, and we need mutable access, so we wrap it in a `Arc<Mutex<_>>`.
//!
//! * Implementing the [`Handler`](crate::Handler) trait requires some magic
//! because of the async functions, we we use the
//! [`async_trait`](https://docs.rs/async-trait/0.1.21/async_trait/) macro.
//!
//! * We use `Stdout` as the type for the `Writer` because neovim acts as our
//! parent, so it reads from our stdout. Note that this is the [async
//! version](tokio::io::Stdout) from Tokio.
//!
//! * We only implement `handle_notify` since we don't want to serve requests.
//! It gets a [`Neovim`](crate::Neovim) passed that we can use to send
//! requests to neovim. All requests are async methods, so we need to `await`
//! them.
//!
//! * The main function is denoted `#[tokio::main]` to use async notation, but
//! it would be perfectly feasible to explicitely create a runtime and use that.
//!
//! * After creation of the handler, we connect to neovim via one of the
//! [`new_*`](crate::create) functions. It gives back a
//! [`Neovim`](crate::Neovim) instance which we could use for requests, and a
//! handle for the io loop.
//!
//! * The plugin quits by ending the IO task when neovim closes the channel, so
//!   we don't need to do anything special. Any cleanup-logic can happen after
//!   the IO task has finished. Note that we're loosing access to our
//!   [`Handler`](crate::Handler), so we might need to implement
//!   [`Drop`](std::ops::Drop) for it, see the
//!   [example](crate::examples::handler_drop).
//!
//! * After the IO task has finished, we're inspecting the errors to see why it
//! went. A join error simply gets printed, then we inspect potential errors
//! from the io loop itself. First, if we did not see a general reader error, we
//! try to send some last notification to the neovim user. Secondly, we quietly
//! ignore the channel being closed, because this usually means that it was
//! closed by neovim, which isn't always an error.
//!
//!   *Note*: A closed channel could still mean an error, so the plugin has the
//!   option to react to this.
//!
//! * As with the other examples, we implement [`Spawn`](futures::task::Spawn)
//! for our `NeovimHandler` most trivially.
