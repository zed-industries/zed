//! # `handler_drop`
//!
//! An example of handling cleanup logic by implementing
//! [`Drop`](std::ops::Drop) for the handler. The plugin attaches to the current
//! buffer, then sets the first 2 lines, which get sent back to us with a
//! `nvim_buf_lines_event` notification. We handle this notification by saving
//! the lines in the handler. We then let nvim close the channel, and wait for
//! the IO loop to finish. The handler gets dropped, and so our cleanup logic is
//! executed.
//!
//! ## Usage
//!
//! First, build the neovim included as a submodule:
//!
//! ```sh
//! cd neovim
//! make
//! ```
//!
//! See <https://github.com/neovim/neovim/wiki/Building-Neovim> for build options.
//! Nothing is really needed to run the example.
//!
//! After that, run the example via
//!
//! ```sh
//! cargo run --example handler_drop
//! ```
//!
//! You can verify it worked by looking at the file `handler_drop.txt` in the
//! project directory which should contain 2 lines ("xyz" and "abc").
//!
//! ## Description
//!
//! * The associated type for our [`Handler`](crate::rpc::handler::Handler) is
//! the stdin of our child. But tokio's
//! [`ChildStdin`](tokio::process::ChildStdin) does not implement
//! [`futures::io::AsyncWrite`](futures::io::AsyncWrite), so it needs to be
//! wrapped in the provided [`Compat`](crate::compat::tokio::Compat) type.
//!
//! * Implementing [`Drop`](std::ops::Drop) is straightforward, except that we
//! cannot do so asynchronously. Since dropping the handler is one of the last
//! things our plugin does, it's not problem to run even larger code bodies
//! synchronously here.
//!
//! * The event handling code is not efficient, because we just read the
//! arguments by reference and clone them. It's easy to take ownership directly
//! by matching on the enum [`Value`](rmpv::Value) directly, though.
//!
//! * There's basically no error handling, other than `unwrap`ing all the
//! `Result`s.
//!
//! * `await`ing the io future handle is probably not necessary, but feels like
//! a nice thing to do.
//!
//! * As with the other examples, we implement [`Spawn`](futures::task::Spawn)
//! for our `NeovimHandler` most trivially.
