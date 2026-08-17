//! Examples of output of the [`bindgen!`] macro.
//!
//! This module is only included in docs.rs documentation and is not present in
//! the actual crate when compiling from crates.io. The purpose of this module
//! is to showcase what the output of the [`bindgen!`] macro looks like and some
//! examples of how to use it.
//!
//! If you're confused or lost in [`bindgen!`] feel free to [open an issue]
//! with a description of your issue and it can hopefully lead to a new example
//! being added here for others to use as reference.
//!
//! ## Including `*.wit` files in your project
//!
//! Note that most of the examples in this module will use the `inline` key of
//! the [`bindgen!`] macro. This is done as it's easy to show the example and
//! WIT all in one self-contained snippet of Rust code. Typically though a
//! project will have a `wit` directory next to `Cargo.toml` which contains WIT
//! files.
//!
//! The general layout of a `wit` directory is that:
//!
//! * All `*.wit` files at `wit/*.wit` are parsed and included in the same
//!   package.
//! * If the `wit/deps` folder is present then it can either contain:
//!   * Subdirectories with a package-per-directory. For example
//!     `wit/deps/wasi-http` and `wit/deps/wasi-cli`.
//!   * WIT files that are a single-file rendering of a package, for example
//!     `wit/deps/wasi-http.wit`
//!   * WIT packages encoded as WebAssembly binaries for a package, for example
//!     `wit/deps/wasi-http.wasm`
//!
//! This means that at this time you'll need to copy around `*.wit` files or
//! WIT packages encoded as `*.wasm` and check them in to your project's `wit`
//! directory. The hope is that in the future it will be easier to manage these
//! files with registry tooling and they won't have to be copied manually.
//! For reference documentation on the layout of the `wit` directory see
//! [`wit_parser::Resolve::push_dir`].
//!
//! [`bindgen!`]: crate::component::bindgen
//! [`wit_parser::Resolve::push_dir`]: https://docs.rs/wit-parser/latest/wit_parser/struct.Resolve.html#method.push_dir
//! [open an issue]: https://github.com/bytecodealliance/wasmtime/issues/new

#![expect(
    missing_docs,
    reason = "bindgen-generated types known to not have docs"
)]

// This "hack" will shadow the `bindgen` macro in general and be inherited to
// following modules by default. This enables documenting sources as-is while
// additionally customizing them to working within the wasmtime crate itself by
// injecting a configuration option to change how the `wasmtime` crate is
// referenced in the generated output.
//
// Note that this has an additional "hack" such that when docs.rs is documenting
// this crate (or CI) then `include_generated_code_from_file` is unconditionally
// turned on. This makes `[source]` links on documentation show the actual
// generated code rather than just the `bindgen!` macro invocation, which can be
// helpful when exploring code.
#[cfg(docsrs)]
macro_rules! bindgen {
    ({$($t:tt)*}) => (crate::component::bindgen!({
        $($t)*
        wasmtime_crate: crate,
        include_generated_code_from_file: true,
    }););
}
#[cfg(not(docsrs))]
macro_rules! bindgen {
    ({$($t:tt)*}) => (crate::component::bindgen!({
        $($t)*
        wasmtime_crate: crate,
    }););
}

/// A "hello world" style example.
///
/// This example loads a component which has access to a single host function.
/// The exported function is called on an instantiation of the component.
///
/// ```rust
/// use wasmtime::component::*;
/// use wasmtime::{Engine, Store};
///
#[doc = include_str!("./_0_hello_world.rs")]
///
/// struct MyState {
///     name: String,
/// }
///
/// // Imports into the world, like the `name` import for this world, are
/// // satisfied through traits.
/// impl HelloWorldImports for MyState {
///     fn name(&mut self) -> String {
///         self.name.clone()
///     }
/// }
///
/// fn main() -> wasmtime::Result<()> {
/// #   if true { return Ok(()) }
///     // Compile the `Component` that is being run for the application.
///     let engine = Engine::default();
///     let component = Component::from_file(&engine, "./your-component.wasm")?;
///
///     // Instantiation of bindings always happens through a `Linker`.
///     // Configuration of the linker is done through a generated `add_to_linker`
///     // method on the bindings structure.
///     //
///     // Note that the function provided here is a projection from `T` in
///     // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
///     // trait. In this case the `T`, `MyState`, is stored directly in the
///     // structure so no projection is necessary here.
///     //
///     // Note that the second type parameter of `add_to_linker` is chosen here
///     // as the built-in `HasSelf` type in Wasmtime. This effectively says
///     // that our function isn't actually projecting, it's returning the
///     // input, so `HasSelf<_>` is a convenience to avoid writing a custom
///     // `HasData` implementation.
///     let mut linker = Linker::new(&engine);
///     HelloWorld::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
///
///     // As with the core wasm API of Wasmtime instantiation occurs within a
///     // `Store`. The bindings structure contains an `instantiate` method which
///     // takes the store, component, and linker. This returns the `bindings`
///     // structure which is an instance of `HelloWorld` and supports typed access
///     // to the exports of the component.
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             name: "me".to_string(),
///         },
///     );
///     let bindings = HelloWorld::instantiate(&mut store, &component, &linker)?;
///
///     // Here our `greet` function doesn't take any parameters for the component,
///     // but in the Wasmtime embedding API the first argument is always a `Store`.
///     bindings.call_greet(&mut store)?;
///     Ok(())
/// }
/// ```
pub mod _0_hello_world;

/// An example of generated bindings for top-level imported functions and
/// interfaces into a world.
///
/// The code used to generate this module is:
///
/// ```rust
/// use wasmtime::component::*;
/// use wasmtime::{Engine, Store};
///
#[doc = include_str!("./_1_world_imports.rs")]
///
/// struct MyState {
///     // ...
/// }
///
/// impl my_custom_host::Host for MyState {
///     fn tick(&mut self) {
///         todo!()
///     }
/// }
///
/// impl MyWorldImports for MyState {
///     fn greet(&mut self) -> String {
///         todo!()
///     }
///
///     fn log(&mut self, msg: String) {
///         println!("{msg}");
///     }
/// }
///
/// fn main() -> wasmtime::Result<()> {
/// #   if true { return Ok(()) }
///     let engine = Engine::default();
///     let component = Component::from_file(&engine, "./your-component.wasm")?;
///
///     let mut linker = Linker::new(&engine);
///     MyWorld::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
///
///     let mut store = Store::new(
///         &engine,
///         MyState { /* ... */ },
///     );
///     let bindings = MyWorld::instantiate(&mut store, &component, &linker)?;
///
///     // ... NB: this world has no exports just yet so not much can be done
///     // with `bindings`.
///
///     Ok(())
/// }
/// ```
pub mod _1_world_imports;

/// An example of generated bindings for top-level exported functions for a
/// world.
///
/// Some notable generated items here are:
///
/// * [`my::project::host::Host`](_2_world_exports::my::project::host::Host) -
///   the generated trait for the `interface host` import.
/// * [`exports::demo::Guest`](_2_world_exports::exports::demo::Guest) -
///   the generated structured used to invoke exports on the returned instance.
/// * [`HelloWorld`](_2_world_exports::HelloWorld) -
///   the overall generated structure representing our `world`.
///
/// ```rust
/// use wasmtime::component::*;
/// use wasmtime::{Engine, Store};
///
#[doc = include_str!("./_2_world_exports.rs")]
///
/// struct MyState {
///     // ...
/// }
///
/// # mod rand { pub fn thread_rng() -> G { G } pub struct G; impl G { pub fn r#gen(&self) -> u32 { 0 } } }
/// // Note that the trait here is per-interface and within a submodule now.
/// impl my::project::host::Host for MyState {
///     fn gen_random_integer(&mut self) -> u32 {
///         rand::thread_rng().r#gen()
///     }
///
///     fn sha256(&mut self, bytes: Vec<u8>) -> String {
///         // ...
/// #       panic!()
///     }
/// }
///
/// fn main() -> wasmtime::Result<()> {
/// #   if true { return Ok(()) }
///     let engine = Engine::default();
///     let component = Component::from_file(&engine, "./your-component.wasm")?;
///
///     let mut linker = Linker::new(&engine);
///     HelloWorld::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
///
///     let mut store = Store::new(
///         &engine,
///         MyState { /* ... */ },
///     );
///     let bindings = HelloWorld::instantiate(&mut store, &component, &linker)?;
///
///     // Note that the `demo` method returns a `&exports::Demo::Guest`
///     // through which we can run the methods on that interface.
///     bindings.demo().call_run(&mut store)?;
///     Ok(())
/// }
/// ```
pub mod _2_world_exports;

/// Example of generating bindings for imported interfaces in a world.
///
/// Notable parts of this example are:
///
/// * Imported interfaces use the Rust module system to encapsulate themselves.
///   The interface imported here is `example:interface-imports/logging` so the
///   generated trait and types are located in
///   [`example::interface_imports::logging`][module].
/// * Types in the `logging` interface are generated in the `logging` module,
///   for example [`Level`].
/// * Generated types have implementations of [`ComponentType`], [`Lift`], and
///   [`Lower`] derived.
/// * The generated trait that host's must implement is always called [`Host`]
///   and is located in the generated module.
///
/// [module]: _3_interface_imports::example::interface_imports::logging
/// [`Level`]: _3_interface_imports::example::interface_imports::logging::Level
/// [`Host`]: _3_interface_imports::example::interface_imports::logging::Host
/// [`ComponentType`]: crate::component::ComponentType
/// [`Lift`]: crate::component::Lift
/// [`Lower`]: crate::component::Lower
///
/// ```rust
/// use wasmtime::component::bindgen;
/// use example::interface_imports::logging::Level;
///
#[doc = include_str!("./_3_interface_imports.rs")]
///
/// struct MyState {
///     // ...
/// }
///
/// impl example::interface_imports::logging::Host for MyState {
///     fn log(&mut self, level: Level, msg: String) {
///         // ...
///     }
/// }
/// ```
pub mod _3_interface_imports;

/// Example of generating bindings for imported resources in a world.
///
/// Notable parts of this example are:
///
/// * Imported resources from the host are represented as traits, in this case
///   [`HostLogger`].
/// * The per-interface [`Host`] trait still exists but has a supertrait of
///   [`HostLogger`].
/// * Resources are represented as [`Resource<T>`] and it's recommended to
///   specify a `with` key to indicate what host type you'd like to use for
///   each resource.
/// * A [`ResourceTable`] can be used to manage resources when working with
///   guests.
///
/// [`Host`]: _4_imported_resources::example::imported_resources::logging::Host
/// [`HostLogger`]: _4_imported_resources::example::imported_resources::logging::HostLogger
/// [`Resource<T>`]: crate::component::Resource
/// [`ResourceTable`]: crate::component::ResourceTable
///
/// ```rust
/// use wasmtime::Result;
/// use wasmtime::component::{bindgen, ResourceTable, Resource};
/// use example::imported_resources::logging::{Level, Host, HostLogger};
///
#[doc = include_str!("./_4_imported_resources.rs")]
///
/// #[derive(Default)]
/// struct MyState {
///     // Manages the mapping of `MyLogger` structures to `Resource<MyLogger>`.
///     table: ResourceTable,
/// }
///
/// // There are no free-functions on `interface logging`, so this is an empty
/// // impl.
/// impl Host for MyState {}
///
/// // This separate `HostLogger` trait serves to act as a namespace for just
/// // the `logger`-related resource methods.
/// impl HostLogger for MyState {
///     // A `constructor` in WIT maps to a `new` function in Rust.
///     fn new(&mut self, max_level: Level) -> Result<Resource<MyLogger>> {
///         let id = self.table.push(MyLogger { max_level })?;
///         Ok(id)
///     }
///
///     fn get_max_level(&mut self, logger: Resource<MyLogger>) -> Result<Level> {
///         debug_assert!(!logger.owned());
///         let logger = self.table.get(&logger)?;
///         Ok(logger.max_level)
///     }
///
///     fn set_max_level(&mut self, logger: Resource<MyLogger>, level: Level) -> Result<()> {
///         debug_assert!(!logger.owned());
///         let logger = self.table.get_mut(&logger)?;
///         logger.max_level = level;
///         Ok(())
///     }
///
///     fn log(&mut self, logger: Resource<MyLogger>, level: Level, msg: String) -> Result<()> {
///         debug_assert!(!logger.owned());
///         let logger = self.table.get_mut(&logger)?;
///         if (level as u32) <= (logger.max_level as u32) {
///             println!("{msg}");
///         }
///         Ok(())
///     }
///
///     fn drop(&mut self, logger: Resource<MyLogger>) -> Result<()> {
///         debug_assert!(logger.owned());
///         let _logger: MyLogger = self.table.delete(logger)?;
///         // ... custom destruction logic here if necessary, otherwise
///         // a `Drop for MyLogger` would also work.
///         Ok(())
///     }
/// }
///
/// # fn main() {}
/// ```
pub mod _4_imported_resources;

/// Example of all kinds of structures of exports from a world.
///
/// * Top-level functions in a `world` are exported directly on the generated
///   structure such as [`call_run`].
/// * All other exports are otherwise scoped with generated traits/types
///   in a top level [`exports`] module.
/// * Exported named interfaces are located at the root of the [`exports`]
///   module, such as [`exports::environment`].
/// * Interfaces are all bound with a structure called `Guest` which has typed
///   functions for each export that can be called. For example
///   [`exports::environment::Guest`][guest1] and
///   [`exports::example::world_exports::units::Guest`][guest2].
/// * Interfaces exported by their id are modeled with multiple namespacing
///   modules, such as [`exports::example::world_exports::units`][units].
///
/// [`call_run`]: _5_all_world_export_kinds::WithExports::call_run
/// [`exports`]: _5_all_world_export_kinds::exports
/// [`exports::environment`]: _5_all_world_export_kinds::exports::environment
/// [guest1]: _5_all_world_export_kinds::exports::environment::Guest
/// [guest2]: _5_all_world_export_kinds::exports::example::world_exports::units::Guest
/// [units]: _5_all_world_export_kinds::exports::example::world_exports::units
///
/// ```rust
/// use wasmtime::{Result, Engine, Store};
/// use wasmtime::component::{bindgen, Component, Linker, HasSelf};
///
#[doc = include_str!("./_5_all_world_export_kinds.rs")]
///
/// struct MyState;
///
/// impl WithExportsImports for MyState {
///     fn log(&mut self, msg: String) {
///         println!("{msg}");
///     }
/// }
///
/// fn main() -> Result<()> {
/// #   if true { return Ok(()) }
///     let engine = Engine::default();
///     let component = Component::from_file(&engine, "./your-component.wasm")?;
///
///     let mut linker = Linker::new(&engine);
///     WithExports::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
///
///     let mut store = Store::new(&engine, MyState);
///     let bindings = WithExports::instantiate(&mut store, &component, &linker)?;
///
///     // top-level functions are exported directly on `WithExports` and are
///     // all prefixed with `call_*`.
///     bindings.call_run(&mut store)?;
///
///     // exported named interfaces are named directly after their export name
///     // and the `&Guest` return value has `call_*` functions on it.
///     bindings.environment().call_set(&mut store, "key", "value")?;
///     let value = bindings.environment().call_get(&mut store, "key")?;
///     assert_eq!(value, "value");
///
///     // exported interfaces by id are similar to export-by-name except that
///     // the exported name is modeled after the full id, not just the name.
///     let units = bindings.example_world_exports_units();
///     let bytes = 1 << 30 + 1 << 20;
///     let s = units.call_bytes_to_string(&mut store, bytes)?;
///     println!("{bytes} = {s}");
///
///     let (seconds, ns) = (1 << 20, 12345);
///     let s = units.call_duration_to_string(&mut store, seconds, ns)?;
///     println!("{seconds}s + {ns}ns = {s}");
///     Ok(())
/// }
/// ```
pub mod _5_all_world_export_kinds;

/// Example of a world which exports a resource.
///
/// * Guest resources are modeled as [`ResourceAny`]. Note that this type is not
///   specialized per-resource at this time so care must be taken to not mix
///   them up.
/// * Resource-related methods are a projection from a [`Guest`] structure, for
///   example to [`GuestLogger`] here.
/// * Resource-related methods all take a [`ResourceAny`] as an argument or
///   a return value.
/// * The [`ResourceAny`] must be explicitly dropped.
///
/// [`ResourceAny`]: crate::component::ResourceAny
/// [`Guest`]: _6_exported_resources::exports::example::exported_resources::logging::Guest
/// [`GuestLogger`]: _6_exported_resources::exports::example::exported_resources::logging::GuestLogger
///
/// ```rust
/// use wasmtime::{Result, Engine, Store};
/// use wasmtime::component::{bindgen, Component, Linker};
/// use self::exports::example::exported_resources::logging::Level;
///
#[doc = include_str!("./_6_exported_resources.rs")]
///
/// struct MyState;
///
/// fn main() -> Result<()> {
/// #   if true { return Ok(()) }
///     let engine = Engine::default();
///     let component = Component::from_file(&engine, "./your-component.wasm")?;
///
///     let linker = Linker::new(&engine);
///     // ... this small example has no imports so nothing is added here, but
///     // if you had imports this is where they'd go.
///
///     let mut store = Store::new(&engine, MyState);
///     let bindings = ExportSomeResources::instantiate(&mut store, &component, &linker)?;
///     let guest = bindings.example_exported_resources_logging();
///     let logger = guest.logger();
///
///     // Resource methods are all attached to `logger` and take the
///     // `ResourceAny` parameter explicitly.
///     let my_logger = logger.call_constructor(&mut store, Level::Warn)?;
///     assert_eq!(logger.call_get_max_level(&mut store, my_logger)?, Level::Warn);
///     logger.call_set_max_level(&mut store, my_logger, Level::Info)?;
///
///     logger.call_log(&mut store, my_logger, Level::Debug, "hello!")?;
///
///     // The `ResourceAny` type has no destructor but when the host is done
///     // with it it needs to invoke the guest-level destructor.
///     my_logger.resource_drop(&mut store)?;
///
///     Ok(())
/// }
/// ```
pub mod _6_exported_resources;

/// Example of generating **async** bindings for imported resources in a world.
///
/// Notable differences from [`_4_imported_resources`] are:
/// * async functions are used
/// * enabled async in bindgen! macro
///
/// See [wasi_async_example](https://github.com/bytecodealliance/wasmtime/blob/main/examples/wasip1-async/main.rs) for async function calls on a host.
///
/// ```rust
/// use wasmtime::Result;
/// use wasmtime::component::{bindgen, ResourceTable, Resource};
/// use example::imported_resources::logging::{Level, Host, HostLogger};
///
#[doc = include_str!("./_7_async.rs")]
///
/// #[derive(Default)]
/// struct MyState {
///     // Manages the mapping of `MyLogger` structures to `Resource<MyLogger>`.
///     table: ResourceTable,
/// }
///
/// // There are no free-functions on `interface logging`, so this is an empty
/// // impl.
/// impl Host for MyState {}
///
/// // This separate `HostLogger` trait serves to act as a namespace for just
/// // the `logger`-related resource methods.
/// impl HostLogger for MyState {
///     // A `constructor` in WIT maps to a `new` function in Rust.
///     async fn new(&mut self, max_level: Level) -> Result<Resource<MyLogger>> {
///         let id = self.table.push(MyLogger { max_level })?;
///         Ok(id)
///     }
///
///     async fn get_max_level(&mut self, logger: Resource<MyLogger>) -> Result<Level> {
///         debug_assert!(!logger.owned());
///         let logger = self.table.get(&logger)?;
///         Ok(logger.max_level)
///     }
///
///     async fn set_max_level(&mut self, logger: Resource<MyLogger>, level: Level) -> Result<()> {
///         debug_assert!(!logger.owned());
///         let logger = self.table.get_mut(&logger)?;
///         logger.max_level = level;
///         Ok(())
///     }
///
///     async fn log(&mut self, logger: Resource<MyLogger>, level: Level, msg: String) -> Result<()> {
///         debug_assert!(!logger.owned());
///         let logger = self.table.get_mut(&logger)?;
///         if (level as u32) <= (logger.max_level as u32) {
///             println!("{msg}");
///         }
///         Ok(())
///     }
///
///     async fn drop(&mut self, logger: Resource<MyLogger>) -> Result<()> {
///         debug_assert!(logger.owned());
///         let _logger: MyLogger = self.table.delete(logger)?;
///         // ... custom destruction logic here if necessary, otherwise
///         // a `Drop for MyLogger` would also work.
///         Ok(())
///     }
/// }
///
/// # fn main() {}
/// ```
pub mod _7_async;
