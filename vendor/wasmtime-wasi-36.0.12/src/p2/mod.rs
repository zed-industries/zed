//! # Wasmtime's WASIp2 Implementation
//!
//!
//! This module provides a Wasmtime host implementation of WASI 0.2 (aka WASIp2
//! aka Preview 2) and WASI 0.1 (aka WASIp1 aka Preview 1). WASI is implemented
//! with the Rust crates [`tokio`] and [`cap-std`] primarily, meaning that
//! operations are implemented in terms of their native platform equivalents by
//! default.
//!
//! # WASIp2 interfaces
//!
//! This module contains implementations of the following interfaces:
//!
//! * [`wasi:cli/environment`]
//! * [`wasi:cli/exit`]
//! * [`wasi:cli/stderr`]
//! * [`wasi:cli/stdin`]
//! * [`wasi:cli/stdout`]
//! * [`wasi:cli/terminal-input`]
//! * [`wasi:cli/terminal-output`]
//! * [`wasi:cli/terminal-stderr`]
//! * [`wasi:cli/terminal-stdin`]
//! * [`wasi:cli/terminal-stdout`]
//! * [`wasi:clocks/monotonic-clock`]
//! * [`wasi:clocks/wall-clock`]
//! * [`wasi:filesystem/preopens`]
//! * [`wasi:filesystem/types`]
//! * [`wasi:random/insecure-seed`]
//! * [`wasi:random/insecure`]
//! * [`wasi:random/random`]
//! * [`wasi:sockets/instance-network`]
//! * [`wasi:sockets/ip-name-lookup`]
//! * [`wasi:sockets/network`]
//! * [`wasi:sockets/tcp-create-socket`]
//! * [`wasi:sockets/tcp`]
//! * [`wasi:sockets/udp-create-socket`]
//! * [`wasi:sockets/udp`]
//!
//! Most traits are implemented for [`WasiCtxView`] trait which provides
//! access to [`WasiCtx`] and [`ResourceTable`], which defines the configuration
//! for WASI and handle state. The [`WasiView`] trait is used to acquire and
//! construct a [`WasiCtxView`].
//!
//! The [`wasmtime-wasi-io`] crate contains implementations of the
//! following interfaces, and this module reuses those implementations:
//!
//! * [`wasi:io/error`]
//! * [`wasi:io/poll`]
//! * [`wasi:io/streams`]
//!
//! These traits are implemented directly for [`ResourceTable`]. All aspects of
//! `wasmtime-wasi-io` that are used by this module are re-exported. Unless you
//! are implementing other host functionality that needs to interact with the
//! WASI scheduler and don't want to use other functionality provided by
//! `wasmtime-wasi`, you don't need to take a direct dependency on
//! `wasmtime-wasi-io`.
//!
//! # Generated Bindings
//!
//! This module uses [`wasmtime::component::bindgen!`] to generate bindings for
//! all WASI interfaces. Raw bindings are available in the [`bindings`] submodule
//! of this module. Downstream users can either implement these traits themselves
//! or you can use the built-in implementations in this module for
//! `WasiImpl<T: WasiView>`.
//!
//! # The `WasiView` trait
//!
//! This module's implementation of WASI is done in terms of an implementation of
//! [`WasiView`]. This trait provides a "view" into WASI-related state that is
//! contained within a [`Store<T>`](wasmtime::Store).
//!
//! For all of the generated bindings in this module (Host traits),
//! implementations are provided looking like:
//!
//! ```
//! # use wasmtime_wasi::WasiCtxView;
//! # trait WasiView {}
//! # mod bindings { pub mod wasi { pub trait Host {} } }
//! impl bindings::wasi::Host for WasiCtxView<'_> {
//!     // ...
//! }
//! ```
//!
//! where the [`WasiCtxView`] type comes from [`WasiView::ctx`] for the type
//! contained within the `Store<T>`. The [`add_to_linker_sync`] and
//! [`add_to_linker_async`] function then require that `T: WasiView` with
//! [`Linker<T>`](wasmtime::component::Linker).
//!
//! To implement the [`WasiView`] trait you will first select a
//! `T` to put in `Store<T>` (typically, by defining your own struct).
//! Somewhere within `T` you'll store:
//!
//! * [`ResourceTable`] - created through default constructors.
//! * [`WasiCtx`] - created through [`WasiCtxBuilder`].
//!
//! You'll then write an implementation of the [`WasiView`]
//! trait to access those items in your `T`. For example:
//! ```
//! use wasmtime::component::ResourceTable;
//! use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
//!
//! struct MyCtx {
//!     table: ResourceTable,
//!     wasi: WasiCtx,
//! }
//!
//! impl WasiView for MyCtx {
//!     fn ctx(&mut self) -> WasiCtxView<'_> {
//!         WasiCtxView { ctx: &mut self.wasi, table: &mut self.table }
//!     }
//! }
//! ```
//!
//! # Async and Sync
//!
//! As of WASI0.2, WASI functions are not blocking from WebAssembly's point of
//! view: a WebAssembly call into these functions returns when they are
//! complete.
//!
//! This module provides an implementation of those functions in the host,
//! where for some functions, it is appropriate to implement them using
//! async Rust and the Tokio executor, so that the host implementation can be
//! nonblocking when Wasmtime's [`Config::async_support`][async] is set.
//! Synchronous wrappers are provided for all async implementations, which
//! creates a private Tokio executor.
//!
//! Users can choose between these modes of implementation using variants
//! of the add_to_linker functions:
//!
//! * For non-async users (the default of `Config`), use [`add_to_linker_sync`].
//! * For async users, use [`add_to_linker_async`].
//!
//! Note that bindings are generated once for async and once for sync. Most
//! interfaces do not change, however, so only interfaces with blocking
//! functions have bindings generated twice. Bindings are organized as:
//!
//! * [`bindings`] - default location of all bindings, blocking functions are
//!   `async`
//! * [`bindings::sync`] - blocking interfaces have synchronous versions here.
//!
//! # Module-specific traits
//!
//! This module's default implementation of WASI bindings to native primitives
//! for the platform that it is compiled for. For example opening a TCP socket
//! uses the native platform to open a TCP socket (so long as [`WasiCtxBuilder`]
//! allows it). There are a few important traits, however, that are specific to
//! this module.
//!
//! * [`InputStream`] and [`OutputStream`] - these are the host traits
//!   behind the WASI `input-stream` and `output-stream` types in the
//!   `wasi:io/streams` interface. These enable embedders to build their own
//!   custom stream and insert them into a [`ResourceTable`] (as a boxed trait
//!   object, see [`DynInputStream`] and [`DynOutputStream`]) to be used from
//!   wasm.
//!
//! * [`Pollable`] - this trait enables building arbitrary logic to get hooked
//!   into a `pollable` resource from `wasi:io/poll`. A pollable resource is
//!   created through the [`subscribe`] function.
//!
//! * [`HostWallClock`](crate::HostWallClock) and [`HostMonotonicClock`](crate::HostMonotonicClock) are used in conjunction with
//!   [`WasiCtxBuilder::wall_clock`] and [`WasiCtxBuilder::monotonic_clock`] if
//!   the defaults host's clock should not be used.
//!
//! * [`StdinStream`] and [`StdoutStream`] are used to provide custom
//!   stdin/stdout streams if they're not inherited (or null, which is the
//!   default).
//!
//! These traits enable embedders to customize small portions of WASI interfaces
//! provided while still providing all other interfaces.
//!
//! # Examples
//!
//! Usage of this module is done through a few steps to get everything hooked up:
//!
//! 1. First implement [`WasiView`] for your type which is the
//!    `T` in `Store<T>`.
//! 2. Add WASI interfaces to a `wasmtime::component::Linker<T>`. This is either
//!    done through top-level functions like [`add_to_linker_sync`] or through
//!    individual `add_to_linker` functions in generated bindings throughout
//!    this module.
//! 3. Create a [`WasiCtx`] for each `Store<T>` through [`WasiCtxBuilder`]. Each
//!    WASI context is "null" or "empty" by default, so items must be explicitly
//!    added to get accessed by wasm (such as env vars or program arguments).
//! 4. Use the previous `Linker<T>` to instantiate a `Component` within a
//!    `Store<T>`.
//!
//! For examples see each of [`WasiView`], [`WasiCtx`], [`WasiCtxBuilder`],
//! [`add_to_linker_sync`], and [`bindings::Command`].
//!
//! [`wasmtime::component::bindgen!`]: https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html
//! [`tokio`]: https://crates.io/crates/tokio
//! [`cap-std`]: https://crates.io/crates/cap-std
//! [`wasmtime-wasi-io`]: https://crates.io/crates/wasmtime-wasi-io
//! [`wasi:cli/environment`]: bindings::cli::environment::Host
//! [`wasi:cli/exit`]: bindings::cli::exit::Host
//! [`wasi:cli/stderr`]: bindings::cli::stderr::Host
//! [`wasi:cli/stdin`]: bindings::cli::stdin::Host
//! [`wasi:cli/stdout`]: bindings::cli::stdout::Host
//! [`wasi:cli/terminal-input`]: bindings::cli::terminal_input::Host
//! [`wasi:cli/terminal-output`]: bindings::cli::terminal_output::Host
//! [`wasi:cli/terminal-stdin`]: bindings::cli::terminal_stdin::Host
//! [`wasi:cli/terminal-stdout`]: bindings::cli::terminal_stdout::Host
//! [`wasi:cli/terminal-stderr`]: bindings::cli::terminal_stderr::Host
//! [`wasi:clocks/monotonic-clock`]: bindings::clocks::monotonic_clock::Host
//! [`wasi:clocks/wall-clock`]: bindings::clocks::wall_clock::Host
//! [`wasi:filesystem/preopens`]: bindings::filesystem::preopens::Host
//! [`wasi:filesystem/types`]: bindings::filesystem::types::Host
//! [`wasi:io/error`]: wasmtime_wasi_io::bindings::wasi::io::error::Host
//! [`wasi:io/poll`]: wasmtime_wasi_io::bindings::wasi::io::poll::Host
//! [`wasi:io/streams`]: wasmtime_wasi_io::bindings::wasi::io::streams::Host
//! [`wasi:random/insecure-seed`]: bindings::random::insecure_seed::Host
//! [`wasi:random/insecure`]: bindings::random::insecure::Host
//! [`wasi:random/random`]: bindings::random::random::Host
//! [`wasi:sockets/instance-network`]: bindings::sockets::instance_network::Host
//! [`wasi:sockets/ip-name-lookup`]: bindings::sockets::ip_name_lookup::Host
//! [`wasi:sockets/network`]: bindings::sockets::network::Host
//! [`wasi:sockets/tcp-create-socket`]: bindings::sockets::tcp_create_socket::Host
//! [`wasi:sockets/tcp`]: bindings::sockets::tcp::Host
//! [`wasi:sockets/udp-create-socket`]: bindings::sockets::udp_create_socket::Host
//! [`wasi:sockets/udp`]: bindings::sockets::udp::Host
//! [async]: https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#method.async_support
//! [`ResourceTable`]: wasmtime::component::ResourceTable

use crate::random::WasiRandom;
use crate::{WasiCtxView, WasiView};
use wasmtime::component::{HasData, Linker, ResourceTable};

pub mod bindings;
pub(crate) mod filesystem;
mod host;
mod ip_name_lookup;
mod network;
pub mod pipe;
mod poll;
mod stdio;
mod tcp;
mod udp;
mod write_stream;

pub use self::filesystem::{FsError, FsResult};
pub use self::network::{Network, SocketError, SocketResult};
pub use self::stdio::IsATTY;
// These contents of wasmtime-wasi-io are re-exported by this module for compatibility:
// they were originally defined in this module before being factored out, and many
// users of this module depend on them at these names.
pub use wasmtime_wasi_io::poll::{DynFuture, DynPollable, MakeFuture, Pollable, subscribe};
pub use wasmtime_wasi_io::streams::{
    DynInputStream, DynOutputStream, Error as IoError, InputStream, OutputStream, StreamError,
    StreamResult,
};

/// Add all WASI interfaces from this crate into the `linker` provided.
///
/// This function will add the `async` variant of all interfaces into the
/// [`Linker`] provided. By `async` this means that this function is only
/// compatible with [`Config::async_support(true)`][async]. For embeddings with
/// async support disabled see [`add_to_linker_sync`] instead.
///
/// This function will add all interfaces implemented by this crate to the
/// [`Linker`], which corresponds to the `wasi:cli/imports` world supported by
/// this crate.
///
/// [async]: wasmtime::Config::async_support
///
/// # Example
///
/// ```
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::{ResourceTable, Linker};
/// use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
///     // ... add any further functionality to `linker` if desired ...
///
///     let mut builder = WasiCtx::builder();
///
///     // ... configure `builder` more to add env vars, args, etc ...
///
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             ctx: builder.build(),
///             table: ResourceTable::new(),
///         },
///     );
///
///     // ... use `linker` to instantiate within `store` ...
///
///     Ok(())
/// }
///
/// struct MyState {
///     ctx: WasiCtx,
///     table: ResourceTable,
/// }
///
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> WasiCtxView<'_> {
///         WasiCtxView { ctx: &mut self.ctx, table: &mut self.table }
///     }
/// }
/// ```
pub fn add_to_linker_async<T: WasiView>(linker: &mut Linker<T>) -> anyhow::Result<()> {
    let options = bindings::LinkOptions::default();
    add_to_linker_with_options_async(linker, &options)
}

/// Similar to [`add_to_linker_async`], but with the ability to enable unstable features.
pub fn add_to_linker_with_options_async<T: WasiView>(
    linker: &mut Linker<T>,
    options: &bindings::LinkOptions,
) -> anyhow::Result<()> {
    add_async_io_to_linker(linker)?;
    add_nonblocking_to_linker(linker, options)?;

    let l = linker;
    bindings::filesystem::types::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    bindings::sockets::tcp::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    bindings::sockets::udp::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    Ok(())
}

/// Shared functionality for [`add_to_linker_async`] and [`add_to_linker_sync`].
fn add_nonblocking_to_linker<'a, T: WasiView, O>(
    linker: &mut Linker<T>,
    options: &'a O,
) -> anyhow::Result<()>
where
    bindings::sockets::network::LinkOptions: From<&'a O>,
    bindings::cli::exit::LinkOptions: From<&'a O>,
{
    use crate::p2::bindings::{cli, clocks, filesystem, random, sockets};

    let l = linker;
    clocks::wall_clock::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    clocks::monotonic_clock::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    filesystem::preopens::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    random::random::add_to_linker::<T, WasiRandom>(l, |t| &mut t.ctx().ctx.random)?;
    random::insecure::add_to_linker::<T, WasiRandom>(l, |t| &mut t.ctx().ctx.random)?;
    random::insecure_seed::add_to_linker::<T, WasiRandom>(l, |t| &mut t.ctx().ctx.random)?;
    cli::exit::add_to_linker::<T, HasWasi>(l, &options.into(), T::ctx)?;
    cli::environment::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::stdin::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::stdout::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::stderr::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::terminal_input::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::terminal_output::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::terminal_stdin::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::terminal_stdout::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::terminal_stderr::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    sockets::tcp_create_socket::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    sockets::udp_create_socket::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    sockets::instance_network::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    sockets::network::add_to_linker::<T, HasWasi>(l, &options.into(), T::ctx)?;
    sockets::ip_name_lookup::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    Ok(())
}

/// Same as [`add_to_linker_async`] except that this only adds interfaces
/// present in the `wasi:http/proxy` world.
pub fn add_to_linker_proxy_interfaces_async<T: WasiView>(
    linker: &mut Linker<T>,
) -> anyhow::Result<()> {
    add_async_io_to_linker(linker)?;
    add_proxy_interfaces_nonblocking(linker)
}

/// Same as [`add_to_linker_sync`] except that this only adds interfaces
/// present in the `wasi:http/proxy` world.
#[doc(hidden)]
pub fn add_to_linker_proxy_interfaces_sync<T: WasiView>(
    linker: &mut Linker<T>,
) -> anyhow::Result<()> {
    add_sync_wasi_io(linker)?;
    add_proxy_interfaces_nonblocking(linker)
}

fn add_proxy_interfaces_nonblocking<T: WasiView>(linker: &mut Linker<T>) -> anyhow::Result<()> {
    use crate::p2::bindings::{cli, clocks, random};

    let l = linker;
    clocks::wall_clock::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    clocks::monotonic_clock::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    random::random::add_to_linker::<T, WasiRandom>(l, |t| &mut t.ctx().ctx.random)?;
    cli::stdin::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::stdout::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    cli::stderr::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    Ok(())
}

/// Add all WASI interfaces from this crate into the `linker` provided.
///
/// This function will add the synchronous variant of all interfaces into the
/// [`Linker`] provided. By synchronous this means that this function is only
/// compatible with [`Config::async_support(false)`][async]. For embeddings
/// with async support enabled see [`add_to_linker_async`] instead.
///
/// This function will add all interfaces implemented by this crate to the
/// [`Linker`], which corresponds to the `wasi:cli/imports` world supported by
/// this crate.
///
/// [async]: wasmtime::Config::async_support
///
/// # Example
///
/// ```
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::{ResourceTable, Linker};
/// use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
///
/// fn main() -> Result<()> {
///     let engine = Engine::default();
///
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
///     // ... add any further functionality to `linker` if desired ...
///
///     let mut builder = WasiCtx::builder();
///
///     // ... configure `builder` more to add env vars, args, etc ...
///
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             ctx: builder.build(),
///             table: ResourceTable::new(),
///         },
///     );
///
///     // ... use `linker` to instantiate within `store` ...
///
///     Ok(())
/// }
///
/// struct MyState {
///     ctx: WasiCtx,
///     table: ResourceTable,
/// }
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> WasiCtxView<'_> {
///         WasiCtxView { ctx: &mut self.ctx, table: &mut self.table }
///     }
/// }
/// ```
pub fn add_to_linker_sync<T: WasiView>(
    linker: &mut wasmtime::component::Linker<T>,
) -> anyhow::Result<()> {
    let options = bindings::sync::LinkOptions::default();
    add_to_linker_with_options_sync(linker, &options)
}

/// Similar to [`add_to_linker_sync`], but with the ability to enable unstable features.
pub fn add_to_linker_with_options_sync<T: WasiView>(
    linker: &mut wasmtime::component::Linker<T>,
    options: &bindings::sync::LinkOptions,
) -> anyhow::Result<()> {
    add_nonblocking_to_linker(linker, options)?;
    add_sync_wasi_io(linker)?;

    let l = linker;
    bindings::sync::filesystem::types::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    bindings::sync::sockets::tcp::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    bindings::sync::sockets::udp::add_to_linker::<T, HasWasi>(l, T::ctx)?;
    Ok(())
}

/// Shared functionality of [`add_to_linker_sync`]` and
/// [`add_to_linker_proxy_interfaces_sync`].
fn add_sync_wasi_io<T: WasiView>(
    linker: &mut wasmtime::component::Linker<T>,
) -> anyhow::Result<()> {
    let l = linker;
    wasmtime_wasi_io::bindings::wasi::io::error::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)?;
    bindings::sync::io::poll::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)?;
    bindings::sync::io::streams::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)?;
    Ok(())
}

struct HasIo;

impl HasData for HasIo {
    type Data<'a> = &'a mut ResourceTable;
}

struct HasWasi;

impl HasData for HasWasi {
    type Data<'a> = WasiCtxView<'a>;
}

// FIXME: it's a bit unfortunate that this can't use
// `wasmtime_wasi_io::add_to_linker` and that's because `T: WasiView`, here,
// not `T: IoView`. Ideally we'd have `impl<T: WasiView> IoView for T` but
// that's not possible with these two traits in separate crates. For now this
// is some small duplication but if this gets worse over time then we'll want
// to massage this.
fn add_async_io_to_linker<T: WasiView>(l: &mut Linker<T>) -> anyhow::Result<()> {
    wasmtime_wasi_io::bindings::wasi::io::error::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)?;
    wasmtime_wasi_io::bindings::wasi::io::poll::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)?;
    wasmtime_wasi_io::bindings::wasi::io::streams::add_to_linker::<T, HasIo>(l, |t| t.ctx().table)?;
    Ok(())
}
