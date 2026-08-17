//! Raw API bindings to the [WebAssembly System Interface (WASI)][WASI]
//!
//! [WASI]: https://github.com/WebAssembly/WASI
//!
//! This crate provides Rust API bindings to the imports of [WASI] [worlds] such
//! as:
//!
//! * [`wasi:cli/command`]
//! * [`wasi:http/proxy`]
//!
//! This crate is procedurally generated with the [`wit-bindgen`] bindings
//! generator. Note that generated code is published to crates.io to slim this
//! crate down in terms of build dependencies and resources.
//!
//! # What is WASI?
//!
//! [WASI] is a set of APIs defined for the WebAssembly [Component Model] to
//! help components interact with the outside world. Core WebAssembly has no
//! intrinsic ability to access the host, for example `println!` don't work, but
//! [WASI] defines how to do so with the [`wasi:cli/stdio`] package.
//!
//! [WASI] is defined by an IDL called [WIT] using files that have the extension
//! `*.wit`. [WASI] and [WIT] are themselves then both defined in terms of the
//! [Component Model] in terms of types available and base semantics for APIs.
//!
//! [WASI] defines a number of standard "worlds" which are a description of a
//! what a WebAssembly component can import from an embedding and must export to
//! an embedding. An example world is [`wasi:cli/command`] which is a world for
//! running CLI applications. This world provides basic system utilities such as
//! clocks, a filesystem, CLI arguments, etc. The one required export is a main
//! function.
//!
//! The purpose of this crate is to provide pregenerated bindings to access
//! [WASI]-defined imports available to components.
//!
//! # What is a Component?
//!
//! An important aspect of [WASI] is that it is defined in terms of the
//! [Component Model]. The [Component Model] is a proposal for WebAssembly which
//! is a new format for wasm binaries, a component. A component contains "core"
//! WebAssembly modules (which are [standard WebAssembly modules]) but also has
//! the ability to do more:
//!
//! * A component can contain multiple core WebAssembly modules.
//! * Types used with component imports and exports are more comprehensive than
//!   core WebAssembly. Core WebAssembly provides integers and floats, for
//!   example, and components build on this and add strings, records (aka a Rust
//!   `struct`), variants (aka a Rust `enum`), and resources (think a file
//!   descriptor on Unix).
//! * A component provides procedural instructions of how to instantiate its
//!   internal core WebAssembly modules with the imports it has.
//!
//! The [Component Model] is a not considered an official WebAssembly standard
//! at this time. It has been in development for 5 years (as of January 2024),
//! however, and the WASI 0.2.0 milestone (more on versioning in a moment) in
//! January 2024 represents a concrete target for ecosystems to use. Runtimes
//! such as [Wasmtime] support the [Component Model] for out-of-browser usage
//! and [jco] is an example of how components can be run in a browser.
//!
//! A full description of the component model is out of scope for this crate's
//! documentation but it suffices to say that [WASI], and this crate, are
//! intended to target components. Components use core WebAssembly modules as an
//! important technical detail, but the final output of this crate is intended
//! to be a component.
//!
//! # What are generated bindings?
//!
//! Above it was seen that [WASI] is defined with [WIT]. These programmatic
//! descriptions of [WASI] APIs are not suitable for use directly in Rust,
//! however these descriptions define how Rust can use them. Each [WIT] function
//! has a defined meaning in core WebAssembly via the [Canonical ABI]. This is a
//! lower level than most users want to operate at, however, so the generated
//! bindings in this crate serve as the bridge.
//!
//! More specifically the generated functions in this crate take the [Canonical
//! ABI] format of [WIT] functions and provide idiomatic Rust functions to call.
//! For example the [`wasi:cli/environment`] definition includes:
//!
//! ```wit
//! interface environment {
//!     // ...
//!     get-environment: func() -> list<tuple<string, string>>;
//!     // ...
//! }
//! ```
//!
//! This corresponds to
//! [`wasi::cli::environment::get_environment`](crate::cli::environment::get_environment).
//!
//! Bindings are pre-generated in this crate with the [`wit-bindgen`] tool. You
//! can also generate your own bindings with [`wit-bindgen`] and [WASI] [WIT]
//! files too, but that's not covered by this crate.
//!
//! # WASI Today and `wasi_snapshot_preview1`
//!
//! This crate is based on the 0.2.0 version of [WASI] APIs. This version of
//! [WASI] was declared "phase 3" (suitable for general use and testing) in
//! January of 2024. Prior to this 0.2.0 "preview2" release of [WASI] there was
//! `wasi_snapshot_preview1`. This previous "preview1" release of [WASI] was
//! circa 2019 and was the initial vision for [WASI] as a standard. Development
//! of [WASI] migrated to the [Component Model] in the meantime.
//!
//! This means that the old `wasi_snapshot_preview1` interfaces are no longer
//! provided by this crate because [WASI] is no longer defined by those
//! interfaces. This includes the historical `*.witx` format which has now been
//! sueprseded. Note that the 0.11.x release series of this crate contains
//! bindings to the historical `wasi_snapshot_preview1` APIs if you're
//! interested in using them.
//!
//! # Crate Organization
//!
//! This crate is currently entirely generated by [`wit-bindgen`] which has the
//! following structure:
//!
//! * Each [WIT] package with bindings corresponds to a top-level module. For
//!   example [`wasi:random`] can be found in the [`random`] module.
//! * Each [WIT] interface then corresponds to a submodule of its package's
//!   module. For example [`wasi:random/insecure`] can be found in the
//!   [`random::insecure`] module.
//! * Each [WIT] function has a Rust function with an idiomatic signature.
//!   module. For example [`random::insecure::get_insecure_random_u64`].
//!
//! Note that [WIT] documentation is rendered as rustdoc documentation in these
//! APIs as well.
//!
//! # Using this Crate
//!
//! This crate is intended to be easiest to use with a future
//! `wasm32-wasip2` target added to the Rust compiler. In the meantime
//! it's recommended to use the `wasm32-wasip1` target instead:
//!
//! ```sh
//! $ cargo build --target wasm32-wasip1
//! ```
//!
//! Note that the output of the `wasm32-wasip1` target is a core wasm module, not
//! a component, so to turn it into a component you can use the [`wasm-tools`]
//! CLI in combination with an "adapter module" for the `wasi_snapshot_preview1`
//! APIs that the Rust standard library uses (example adapters can be found on
//! [Wasmtime's release page][adapters] as
//! [`wasi_snapshot_preview1.command.wasm`] for example)
//!
//! ```sh
//! $ wasm-tools component new ./target/wasm32-wasip1/debug/my-app.wasm \
//!     --adapt ./wasi_snapshot_preview1.command.wasm \
//!     -o my-component.wasm
//! ```
//!
//! ## Export Macros
//!
//! In addition to providing bindings for imports this crate also provides
//! macros to export the `wasi:cli/run` and `wasi:http/proxy` worlds, see their
//! respective documentation for more information:
//!
//! - [`wasi::cli::command::export!`](crate::cli::command::export)
//! - [`wasi::http::proxy::export!`](crate::http::proxy::export)
//!
//! [worlds]: https://component-model.bytecodealliance.org/design/worlds.html
//! [`wasi:cli/command`]: https://github.com/WebAssembly/wasi-cli/
//! [`wasi:http/proxy`]: https://github.com/WebAssembly/wasi-http
//! [`wasi:cli/stdio`]: https://github.com/WebAssembly/wasi-cli/blob/main/wit/stdio.wit
//! [`wit-bindgen`]: https://github.com/bytecodealliance/wit-bindgen/
//! [Component Model]: https://component-model.bytecodealliance.org/
//! [WIT]: https://component-model.bytecodealliance.org/design/wit.html
//! [standard WebAssembly modules]: https://webassembly.github.io/spec/
//! [Wasmtime]: https://github.com/bytecodealliance/wasmtime
//! [jco]: https://github.com/bytecodealliance/jco
//! [Canonical ABI]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
//! [`wasi:cli/environment`]: https://github.com/WebAssembly/wasi-cli/blob/main/wit/environment.wit
//! [`wasi:random`]: https://github.com/WebAssembly/wasi-random
//! [`wasi:random/insecure`]: https://github.com/WebAssembly/wasi-random/blob/main/wit/insecure.wit
//! [`wasm-tools`]: https://github.com/bytecodealliance/wasm-tools
//! [adapters]: https://github.com/bytecodealliance/wasmtime/releases
//! [`wasi_snapshot_preview1.command.wasm`]: https://github.com/bytecodealliance/wasmtime/releases/download/v17.0.0/wasi_snapshot_preview1.command.wasm

#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod ext;

// These modules are all auto-generated by `./ci/regenerate.sh`
#[allow(unused_imports)]
mod command;
mod imports;
#[allow(unused_imports)]
mod proxy;

// generated bindings start with the package namespace, which in this case is
// `wasi`, but the crate is already called wasi, so lift everything up one level
// to the root of this crate.
pub use imports::wasi::*;

// Expand the `cli` and `http` modules with `export!` macros for the
// command/proxy worlds, but also retain all the contents defined in the
// `bindings` module as well.
pub mod cli {
    pub use super::imports::wasi::cli::*;

    pub mod command {
        /// Generate an exported instance of the `wasi:cli/command` world.
        ///
        /// This macro generate the `#[no_mangle]` functions necessary to
        /// export this interface. It takes an argument which is a type that
        /// must implement the
        /// [`exports::cli::run::Guest`](crate::exports::cli::run::Guest)
        /// trait.
        ///
        /// ```
        /// struct MyCliRunner;
        ///
        /// impl wasip2::exports::cli::run::Guest for MyCliRunner {
        ///     fn run() -> Result<(), ()> {
        ///         // ...
        /// # panic!();
        ///     }
        /// }
        ///
        /// wasip2::cli::command::export!(MyCliRunner);
        /// ```
        ///
        /// ## Compatibility with `wasm32-wasip1` targets
        ///
        /// This macro is not compatible with `wasm32-wasip1` `bin` targets
        /// which instead use a `fn main()` with the
        /// `wasi_snapshot_preview1.command.wasm` adapter. This macro _can_ be
        /// used with the `reactor` or `proxy` adapters.
        ///
        /// <!--
        /// The marker above hides the generated documentation by wit-bindgen for this
        /// macro.
        /// -->
        #[doc(inline)]
        pub use crate::command::_export_command as export;
    }
}

pub mod http {
    pub use super::proxy::wasi::http::*;

    pub mod proxy {
        /// Generate an exported instance of the `wasi:http/proxy` world.
        ///
        /// This macro will generate `#[no_mangle]` functions as necessary to
        /// export an implementation of the
        /// [`exports::http::incoming_handler::Guest`](crate::exports::http::incoming_handler::Guest)
        /// trait.  This macro takes
        /// an argument which is a type that implements this trait:
        ///
        /// ```
        /// use wasip2::http::types::{IncomingRequest, ResponseOutparam};
        ///
        /// struct MyIncomingHandler;
        ///
        /// impl wasip2::exports::http::incoming_handler::Guest for MyIncomingHandler {
        ///     fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        ///         // ...
        /// # panic!();
        ///     }
        /// }
        ///
        /// wasip2::http::proxy::export!(MyIncomingHandler);
        /// ```
        ///
        /// <!--
        /// The marker above hides the generated documentation by wit-bindgen
        /// for this macro.
        /// -->
        #[doc(inline)]
        pub use crate::proxy::_export_proxy as export;
    }
}

pub mod exports {
    // This is required by the `export!` macros of this crate which assume that
    // the types it's referring to show up as `exports::wasi::...`.
    //
    // This isn't part of the public interface, though, so hide this.
    #[doc(hidden)]
    pub mod wasi {
        pub use crate::command::exports::wasi::*;
        pub use crate::proxy::exports::wasi::*;
    }

    // These are the restructured public interface of this crate.
    pub use crate::command::exports::wasi::cli;
    pub use crate::proxy::exports::wasi::http;
}

// These macros are used by recursive invocations of the macro, but they're
// `#[doc(hidden)]` as it's not part of the public interface.
#[doc(hidden)]
pub use crate::command::_export_command;
#[doc(hidden)]
pub use crate::proxy::_export_proxy;
