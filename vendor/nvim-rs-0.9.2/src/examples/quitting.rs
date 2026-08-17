//! # Quitting
//!
//! A very small example showing how to handle quitting an application.
//!
//! * Running as an rpc plugin, just have neovim close the channel corresponding
//! to our plugin
//! * When embedding neovim, do the same via a command, as shown in this
//! example. Note that this final request _will_ receive an error, since it will
//! not get an answer from neovim.
//!
//! Also note that all other pending requests will receive an EOF error as well.
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
//! cargo run --example quitting
//! ```
//!
//! ## Description
//!
//! Some overview over the code:
//!
//! * Since we're not interested in handling anything, our `NeovimHandler` is a
//! [`Dummy`](crate::rpc::handler::Dummy) that does nothing on requests and
//! notifications. We need to pass something that implements
//! [`Spawn`](futures::task::Spawn), which is represented by the `Spawner`. It
//! doesn't carry any data here. We implement `Spawn` in the most trivial way
//! possible, by calling [`tokio::spawn`](tokio::spawn) that in turn calls out
//! to the surrounding runtime.
//!
//! * Any shutdown logic should be handled after the channel was closed. We
//! don't actually need to inspect the error, since the application will shut
//! down no matter what. If we need access to our handler for that, we should
//! implement [`Drop`](std::ops::Drop) for it, see
//! [`handler_drop`](crate::examples::handler_drop).
//!
//! * The last command (the one that instructs neovim to close the channel) will
//! not receive an answer anymore, but an error. We just show the error and its
//! source for demonstation purposes. We use the
//! [`is_channel_closed`](crate::error::CallError::is_channel_closed) method
//! to verify that the error originates from this.
