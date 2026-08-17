//! Bindings generation support for Rust with the Component Model.
//!
//! This crate is a bindings generator for [WIT] and the [Component Model].
//! Users are likely interested in the [`generate!`] macro which actually
//! generates bindings. Otherwise this crate provides any runtime support
//! necessary for the macro-generated code.
//!
//! [WIT]: https://component-model.bytecodealliance.org/design/wit.html
//! [Component Model]: https://component-model.bytecodealliance.org/

#![no_std]

#[cfg(not(feature = "rustc-dep-of-std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Generate bindings for an input WIT document.
///
/// This macro is the bread-and-butter of the `wit-bindgen` crate. The macro
/// here will parse [WIT] as input and generate Rust bindings to work with the
/// `world` that's specified in the [WIT]. For a primer on WIT see [this
/// documentation][WIT] and for a primer on worlds see [here][worlds].
///
/// [WIT]: https://component-model.bytecodealliance.org/design/wit.html
/// [worlds]: https://component-model.bytecodealliance.org/design/worlds.html
///
/// This macro takes as input a [WIT package] as well as a [`world`][worlds]
/// within that package. It will then generate a Rust function for all `import`s
/// into the world. If there are any `export`s then a Rust `trait` will be
/// generated for you to implement. The macro additionally takes a number of
/// configuration parameters documented below as well.
///
/// Basic invocation of the macro can look like:
///
/// ```
/// use wit_bindgen::generate;
/// # macro_rules! generate { ($($t:tt)*) => () }
///
/// generate!();
/// ```
///
/// This will parse a WIT package in the `wit` folder adjacent to your project's
/// `Cargo.toml` file. Within this WIT package there must be precisely one
/// `world` and that world will be the one that has bindings generated for it.
/// All other options remain at their default values (more on this below).
///
/// If your WIT package has more than one `world`, or if you want to select a
/// world from the dependencies, you can specify a world explicitly:
///
/// ```
/// use wit_bindgen::generate;
/// # macro_rules! generate { ($($t:tt)*) => () }
///
/// generate!("my-world");
/// generate!("wasi:cli/imports");
/// ```
///
/// This form of the macro takes a single string as an argument which is a
/// "world specifier" to select which world is being generated. As a single
/// string, such as `"my-world"`, this selects the world named `my-world` in the
/// package being parsed in the `wit` folder. The longer form specification
/// `"wasi:cli/imports"` indicates that the `wasi:cli` package, located in the
/// `wit/deps` folder, will have a world named `imports` and those bindings will
/// be generated.
///
/// If your WIT package is located in a different directory than one called
/// `wit` then it can be specified with the `in` keyword:
///
/// ```
/// use wit_bindgen::generate;
/// # macro_rules! generate { ($($t:tt)*) => () }
///
/// generate!(in "./my/other/path/to/wit");
/// generate!("a-world" in "../path/to/wit");
/// ```
///
/// The full-form of the macro, however, takes a braced structure which is a
/// "bag of options":
///
/// ```
/// use wit_bindgen::generate;
/// # macro_rules! generate { ($($t:tt)*) => () }
///
/// generate!({
///     world: "my-world",
///     path: "../path/to/wit",
///     // ...
/// });
/// ```
///
/// For documentation on each option, see below.
///
/// ## Exploring generated bindings
///
/// Once bindings have been generated they can be explored via a number of means
/// to see what was generated:
///
/// * Using `cargo doc` should render all of the generated bindings in addition
///   to the original comments in the WIT format itself.
/// * If your IDE supports `rust-analyzer` code completion should be available
///   to explore and see types.
/// * The `wit-bindgen` CLI tool, packaged as `wit-bindgen-cli` on crates.io,
///   can be executed the same as the `generate!` macro and the output can be
///   read.
/// * If you're seeing an error, `WIT_BINDGEN_DEBUG=1` can help debug what's
///   happening (more on this below) by emitting macro output to a file.
/// * This documentation can be consulted for various constructs as well.
///
/// Currently browsing generated code may have road bumps on the way. If you run
/// into issues or have idea of how to improve the situation please [file an
/// issue].
///
/// [file an issue]: https://github.com/bytecodealliance/wit-bindgen/issues/new
///
/// ## Namespacing
///
/// In WIT, worlds can import and export `interface`s, functions, and types. Each
/// `interface` can either be "anonymous" and only named within the context of a
/// `world` or it can have a "package ID" associated with it. Names in Rust take
/// into account all the names associated with a WIT `interface`. For example
/// the package ID `foo:bar/baz` would create a `mod foo` which contains a `mod
/// bar` which contains a `mod baz`.
///
/// WIT imports and exports are additionally separated into their own
/// namespaces. Imports are generated at the level of the `generate!` macro
/// where exports are generated under an `exports` namespace.
///
/// ## Imports
///
/// Imports into a `world` can be types, resources, functions, and interfaces.
/// Each of these is bound as a Rust type, function, or module. The intent is
/// that the WIT interfaces map to what is roughly idiomatic Rust for the given
/// interface.
///
/// ### Imports: Top-level functions and types
///
/// Imports at the top-level of a world are generated directly where the
/// `generate!` macro is invoked.
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     inline: r"
///         package a:b;
///
///         world the-world {
///             record fahrenheit {
///                 degrees: f32,
///             }
///
///             import what-temperature-is-it: func() -> fahrenheit;
///
///             record celsius {
///                 degrees: f32,
///             }
///
///             import convert-to-celsius: func(a: fahrenheit) -> celsius;
///         }
///     ",
/// });
///
/// fn test() {
///     let current_temp = what_temperature_is_it();
///     println!("current temp in fahrenheit is {}", current_temp.degrees);
///     let in_celsius: Celsius = convert_to_celsius(current_temp);
///     println!("current temp in celsius is {}", in_celsius.degrees);
/// }
/// ```
///
/// ### Imports: Interfaces
///
/// Interfaces are placed into submodules where the `generate!` macro is
/// invoked and are namespaced based on their identifiers.
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     inline: r"
///         package my:test;
///
///         interface logging {
///             enum level {
///                 debug,
///                 info,
///                 error,
///             }
///             log: func(level: level, msg: string);
///         }
///
///         world the-world {
///             import logging;
///             import global-logger: interface {
///                 use logging.{level};
///
///                 set-current-level: func(level: level);
///                 get-current-level: func() -> level;
///             }
///         }
///     ",
/// });
///
/// // `my` and `test` are from `package my:test;` and `logging` is for the
/// // interfac name.
/// use my::test::logging::Level;
///
/// fn test() {
///     let current_level = global_logger::get_current_level();
///     println!("current logging level is {current_level:?}");
///     global_logger::set_current_level(Level::Error);
///
///     my::test::logging::log(Level::Info, "Hello there!");
/// }
/// #
/// # fn main() {}
/// ```
///
/// ### Imports: Resources
///
/// Imported resources generate a type named after the name of the resource.
/// This type is then used both for borrows as `&T` as well as via ownership as
/// `T`. Resource methods are bound as methods on the type `T`.
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     inline: r#"
///         package my:test;
///
///         interface logger {
///             enum level {
///                 debug,
///                 info,
///                 error,
///             }
///
///             resource logger {
///                 constructor(destination: string);
///                 log: func(level: level, msg: string);
///             }
///         }
///
///         // Note that while this world does not textually import the above
///         // `logger` interface it is a transitive dependency via the `use`
///         // statement so the "elaborated world" imports the logger.
///         world the-world {
///             use logger.{logger};
///
///             import get-global-logger: func() -> logger;
///         }
///     "#,
/// });
///
/// use my::test::logger::Level;
///
/// fn test() {
///     let logger = get_global_logger();
///     logger.log(Level::Debug, "This is a global message");
///
///     let logger2 = Logger::new("/tmp/other.log");
///     logger2.log(Level::Info, "This is not a global message");
/// }
/// #
/// # fn main() {}
/// ```
///
/// Note in the above example the lack of import of `Logger`. The `use`
/// statement imported the `Logger` type, an alias of it, from the `logger`
/// interface into `the-world`. This generated a Rust `type` alias so `Logger`
/// was available at the top-level.
///
/// ## Exports: Basic Usage
///
/// A WIT world can not only `import` functionality but can additionally
/// `export` functionality as well. An `export` represents a contract that the
/// Rust program must implement to be able to work correctly. The `generate!`
/// macro's goal is to take care of all the low-level and ABI details for you,
/// so the end result is that `generate!`, for exports, will generate Rust
/// `trait`s that you must implement.
///
/// A minimal example of this is:
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     inline: r#"
///         package my:test;
///
///         world my-world {
///             export hello: func();
///         }
///     "#,
/// });
///
/// struct MyComponent;
///
/// impl Guest for MyComponent {
///     fn hello() {}
/// }
///
/// export!(MyComponent);
/// #
/// # fn main() {}
/// ```
///
/// Here the `Guest` trait was generated by the `generate!` macro and represents
/// the functions at the top-level of `my-world`, in this case the function
/// `hello`. A custom type, here called `MyComponent`, is created and the trait
/// is implemented for that type.
///
/// Additionally a macro is generated by `generate!` (macros generating macros)
/// called `export!`. The `export!` macro is given a component that implements
/// the export `trait`s and then it will itself generate all necessary
/// `#[unsafe(no_mangle)]` functions to implement the ABI required.
///
/// ## Exports: Multiple Interfaces
///
/// Each `interface` in WIT will generate a `trait` that must be implemented in
/// addition to the top-level `trait` for the world. All traits are named
/// `Guest` here and are namespaced appropriately in modules:
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     inline: r#"
///         package my:test;
///
///         interface a {
///             func-in-a: func();
///             second-func-in-a: func();
///         }
///
///         world my-world {
///             export a;
///             export b: interface {
///                 func-in-b: func();
///             }
///             export c: func();
///         }
///     "#,
/// });
///
/// struct MyComponent;
///
/// impl Guest for MyComponent {
///     fn c() {}
/// }
///
/// impl exports::my::test::a::Guest for MyComponent {
///     fn func_in_a() {}
///     fn second_func_in_a() {}
/// }
///
/// impl exports::b::Guest for MyComponent {
///     fn func_in_b() {}
/// }
///
/// export!(MyComponent);
/// #
/// # fn main() {}
/// ```
///
/// Here note that there were three `Guest` traits generated for each of the
/// three groups: two interfaces and one `world`. Also note that traits (and
/// types) for exports are namespaced in an `exports` module.
///
/// Note that when the top-level `world` does not have any exported functions,
/// or if an interface does not have any functions, then no `trait` is
/// generated:
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     inline: r#"
///         package my:test;
///
///         interface a {
///             type my-type = u32;
///         }
///
///         world my-world {
///             export b: interface {
///                 use a.{my-type};
///
///                 foo: func() -> my-type;
///             }
///         }
///     "#,
/// });
///
/// struct MyComponent;
///
/// impl exports::b::Guest for MyComponent {
///     fn foo() -> u32 {
///         42
///     }
/// }
///
/// export!(MyComponent);
/// #
/// # fn main() {}
/// ```
///
/// ## Exports: Resources
///
/// Exporting a resource is significantly different than importing a resource.
/// A component defining a resource can create new resources of that type at any
/// time, for example. Additionally resources can be "dereferenced" into their
/// underlying values within the component.
///
/// Owned resources have a custom type generated and borrowed resources are
/// generated with a type of the same name suffixed with `Borrow<'_>`, such as
/// `MyResource` and `MyResourceBorrow<'_>`.
///
/// Like `interface`s the methods and functions used with a `resource` are
/// packaged up into a `trait`.
///
/// Specifying a custom resource type is done with an associated type on the
/// corresponding trait for the resource's containing interface/world:
///
/// ```
/// use wit_bindgen::generate;
/// use std::cell::{RefCell, Cell};
///
/// generate!({
///     inline: r#"
///         package my:test;
///
///         interface logging {
///             enum level {
///                 debug,
///                 info,
///                 error,
///             }
///
///             resource logger {
///                 constructor(level: level);
///                 log: func(level: level, msg: string);
///                 level: func() -> level;
///                 set-level: func(level: level);
///             }
///         }
///
///         world my-world {
///             export logging;
///         }
///     "#,
/// });
///
/// use exports::my::test::logging::{Guest, GuestLogger, Level};
///
/// struct MyComponent;
///
/// // Note that the `logging` interface has no methods of its own but a trait
/// // is required to be implemented here to specify the type of `Logger`.
/// impl Guest for MyComponent {
///     type Logger = MyLogger;
/// }
///
/// struct MyLogger {
///     level: Cell<Level>,
///     contents: RefCell<String>,
/// }
///
/// impl GuestLogger for MyLogger {
///     fn new(level: Level) -> MyLogger {
///         MyLogger {
///             level: Cell::new(level),
///             contents: RefCell::new(String::new()),
///         }
///     }
///
///     fn log(&self, level: Level, msg: String) {
///         if level as u32 <= self.level.get() as u32 {
///             self.contents.borrow_mut().push_str(&msg);
///             self.contents.borrow_mut().push_str("\n");
///         }
///     }
///
///     fn level(&self) -> Level {
///         self.level.get()
///     }
///
///     fn set_level(&self, level: Level) {
///         self.level.set(level);
///     }
/// }
///
/// export!(MyComponent);
/// #
/// # fn main() {}
/// ```
///
/// It's important to note that resources in Rust do not get `&mut self` as
/// methods, but instead are required to be defined with `&self`. This requires
/// the use of interior mutability such as `Cell` and `RefCell` above from the
/// `std::cell` module.
///
/// ## Exports: The `export!` macro
///
/// Components are created by having exported WebAssembly functions with
/// specific names, and these functions are not created when `generate!` is
/// invoked. Instead these functions are created afterwards once you've defined
/// your own type an implemented the various `trait`s for it. The
/// `#[unsafe(no_mangle)]` functions that will become the component are created
/// with the generated `export!` macro.
///
/// Each call to `generate!` will itself generate a macro called `export!`.
/// The macro's first argument is the name of a type that implements the traits
/// generated:
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     inline: r#"
///         package my:test;
///
///         world my-world {
/// #           export hello: func();
///             // ...
///         }
///     "#,
/// });
///
/// struct MyComponent;
///
/// impl Guest for MyComponent {
/// #   fn hello() {}
///     // ...
/// }
///
/// export!(MyComponent);
/// #
/// # fn main() {}
/// ```
///
/// This argument is a Rust type which implements the `Guest` traits generated
/// by `generate!`. Note that all `Guest` traits must be implemented for the
/// type provided or an error will be generated.
///
/// This macro additionally accepts a second argument. The macro itself needs to
/// be able to find the module where the `generate!` macro itself was originally
/// invoked. Currently that can't be done automatically so a path to where
/// `generate!` was provided can also be passed to the macro. By default, the
/// argument is set to `self`:
///
/// ```
/// use wit_bindgen::generate;
///
/// generate!({
///     // ...
/// #   inline: r#"
/// #       package my:test;
/// #
/// #       world my-world {
/// #           export hello: func();
/// #           // ...
/// #       }
/// #   "#,
/// });
/// #
/// # struct MyComponent;
/// #
/// # impl Guest for MyComponent {
/// #   fn hello() {}
/// #     // ...
/// # }
/// #
/// export!(MyComponent with_types_in self);
/// #
/// # fn main() {}
/// ```
///
/// This indicates that the current module, referred to with `self`, is the one
/// which had the `generate!` macro expanded.
///
/// If, however, the `generate!` macro was run in a different module then that
/// must be configured:
///
/// ```
/// mod bindings {
///     wit_bindgen::generate!({
///         // ...
/// #   inline: r#"
/// #       package my:test;
/// #
/// #       world my-world {
/// #           export hello: func();
/// #           // ...
/// #       }
/// #   "#,
///     });
/// }
/// #
/// # struct MyComponent;
/// #
/// # impl bindings::Guest for MyComponent {
/// #   fn hello() {}
/// #     // ...
/// # }
/// #
/// bindings::export!(MyComponent with_types_in bindings);
/// #
/// # fn main() {}
/// ```
///
/// ## Debugging output to `generate!`
///
/// While `wit-bindgen` is tested to the best of our ability there are
/// inevitably bugs and issues that arise. These can range from bad error
/// messages to misconfigured invocations to bugs in the macro itself. To assist
/// with debugging these situations the macro recognizes an environment
/// variable:
///
/// ```shell
/// export WIT_BINDGEN_DEBUG=1
/// ```
///
/// When set the macro will emit the result of expansion to a file and then
/// `include!` that file. Any error messages generated by `rustc` should then
/// point to the generated file and allow you to open it up, read it, and
/// inspect it. This can often provide better context to the error than rustc
/// provides by default with macros.
///
/// It is not recommended to set this environment variable by default as it will
/// cause excessive rebuilds of Cargo projects. It's recommended to only use it
/// as necessary to debug issues.
///
/// ## Options to `generate!`
///
/// The full list of options that can be passed to the `generate!` macro are as
/// follows. Note that there are no required options, they all have default
/// values.
///
///
/// ```
/// use wit_bindgen::generate;
/// # macro_rules! generate { ($($t:tt)*) => () }
///
/// generate!({
///     // The name of the world that bindings are being generated for. If this
///     // is not specified then it's required that the package selected
///     // below has a single `world` in it.
///     world: "my-world",
///
///     // Path to parse WIT and its dependencies from. Defaults to the `wit`
///     // folder adjacent to your `Cargo.toml`.
///     //
///     // This parameter also supports the form of a list, such as:
///     // ["../path/to/wit1", "../path/to/wit2"]
///     // Usually used in testing, our test suite may want to generate code
///     // from wit files located in multiple paths within a single mod, and we
///     // don't want to copy these files again. Currently these locations must
///     // be ordered, as later paths can't contain dependencies on earlier
///     // paths. This restriction may be lifted in the future.
///     path: "../path/to/wit",
///
///     // Enables passing "inline WIT". If specified this is the default
///     // package that a world is selected from. Any dependencies that this
///     // inline WIT refers to must be defined in the `path` option above.
///     //
///     // By default this is not specified.
///     inline: "
///         world my-world {
///             import wasi:cli/imports;
///
///             export my-run: func()
///         }
///     ",
///
///     // Additional traits to derive for all defined types. Note that not all
///     // types may be able to implement these traits, such as resources.
///     //
///     // By default this set is empty.
///     additional_derives: [PartialEq, Eq, Hash, Clone],
///
///     // When generating bindings for interfaces that are not defined in the
///     // same package as `world`, this option can be used to either generate
///     // those bindings or point to already generated bindings.
///     // For example, if your world refers to WASI types then the `wasi` crate
///     // already has generated bindings for all WASI types and structures. In this
///     // situation the key `with` here can be used to use those types
///     // elsewhere rather than regenerating types.
///     // If for example your world refers to some type and you want to use
///     // your own custom implementation of that type then you can specify
///     // that here as well. There is a requirement on the remapped (custom)
///     // type to have the same internal structure and identical to what would
///     // wit-bindgen generate (including alignment, etc.), since
///     // lifting/lowering uses its fields directly.
///     //
///     // If, however, your world refers to interfaces for which you don't have
///     // already generated bindings then you can use the special `generate` value
///     // to have those bindings generated.
///     //
///     // The `with` key here works for interfaces and individual types.
///     //
///     // When an interface or type is specified here no bindings will be
///     // generated at all. It's assumed bindings are fully generated
///     // somewhere else. This is an indicator that any further references to types
///     // defined in these interfaces should use the upstream paths specified
///     // here instead.
///     //
///     // Any unused keys in this map are considered an error.
///     with: {
///         "wasi:io/poll": wasi::io::poll,
///         "some:package/my-interface": generate,
///         "some:package/my-interface/my-type": my_crate::types::MyType,
///     },
///
///     // Indicates that all interfaces not present in `with` should be assumed
///     // to be marked with `generate`.
///     generate_all,
///
///     // An optional list of function names to skip generating bindings for.
///     // This is only applicable to imports and the name specified is the name
///     // of the function.
///     skip: ["foo", "bar", "baz"],
///
///     // Configuration of how Rust types are generated.
///     //
///     // This option will change how WIT types are mapped to Rust types. There
///     // are a number of ways this can be done depending on the context. For
///     // example a Rust `&str` is suitable to pass to an imported function but
///     // an exported function receives a `String`. These both represent the
///     // WIT type `string`, however.
///     //
///     // Type generation becomes extra-significant when aggregates come into
///     // play (such as a WIT `record` or `variant`), especially when the
///     // aggregate is used both in an imported function and exported one.
///     //
///     // There are three modes of ownership, documented here, but only one
///     // can be specified.
///     //
///     // The default mode is "Owning" meaning that all Rust types will by
///     // default contain their owned containers. For example a `record` with
///     // a `string` will map to a Rust `struct` containing a `String`. This
///     // maximizes the chance that types can be shared between imports and
///     // exports but can come at a cost where calling an import may require
///     // more allocations than necessary.
///     ownership: Owning,
///
///     // Specifies an alternative name for the `export!` macro generated for
///     // any exports this world has.
///     //
///     // Defaults to "export"
///     export_macro_name: "export",
///
///     // Indicates whether the `export!` macro is `pub` or just `pub(crate)`.
///     //
///     // This defaults to `false`.
///     pub_export_macro: false,
///
///     // The second mode of ownership is "Borrowing". This mode then
///     // additionally has a boolean flag indicating whether duplicate types
///     // should be generated if necessary.
///     //
///     // This mode will prefer using borrowed values in Rust to represent WIT
///     // values where possible. For example if the argument to an imported
///     // function is a record-with-a-string then in Rust that will generate a
///     // `struct` with a lifetime parameter storing `&'a str`.
///     //
///     // The `duplicate_if_necessary` flag will cause duplicate types to be
///     // generated when a WIT type is used both in an import and export. In
///     // this situation one will be called `FooParam` and one will be called
///     // `FooResult` (where `foo` is the WIT name).
///     //
///     // It's generally recommended to not turn this on unless performance
///     // requires it. Even if so, please feel free to open an issue on the
///     // `wit-bindgen` repository to help improve the default "Owning" use
///     // case above if possible.
///     ownership: Borrowing { duplicate_if_necessary: false },
///
///     // The generated `export!` macro, if any, will by default look for
///     // generated types adjacent to where the `export!` macro is invoked
///     // through the `self` module. This option can be used to change the
///     // defaults to look somewhere else instead.
///     default_bindings_module: "path::to::bindings",
///
///     // This will suffix the custom section containing component type
///     // information with the specified string. This is not required by
///     // default but if the same world is generated in two different locations
///     // in the crate then one bindings generation location will need this
///     // suffix to avoid having the custom sections corrupt each other.
///     type_section_suffix: "suffix",
///
///     // Configures the path to the `wit-bindgen` crate itself. By default
///     // this is `wit_bindgen` assuming that your crate depends on the
///     // `wit-bindgen` crate itself.
///     runtime_path: "path::to::wit_bindgen",
///
///     // Configure where the `bitflags` crate is located. By default this
///     // is `wit_bindgen::bitflags` which already reexports `bitflags` for
///     // you.
///     bitflags_path: "path::to::bitflags",
///
///     // Indicates that instead of `&str` and `String` the `&[u8]` and
///     // `Vec<u8>` types should be used. Only intended for cases where
///     // compiled size is of the utmost concern as this can avoid pulling in
///     // UTF-8 validation.
///     raw_strings,
///
///     // Emits `#[cfg(feature = "std")]` around `impl Error for ... {}` blocks
///     // for generated types. This is a niche option that is only here to
///     // support the standard library itself depending on this crate one day.
///     std_feature,
///
///     // Disable a workaround to force wasm constructors to be run only once
///     // when exported functions are called.
///     disable_run_ctors_once_workaround: false,
///
///     // Whether to generate unused `record`, `enum`, `variant` types.
///     // By default, they will not be generated unless they are used as input
///     // or return value of a function.
///     generate_unused_types: false,
///
///     // A list of "features" which correspond to WIT features to activate
///     // when parsing WIT files. This enables `@unstable` annotations showing
///     // up and having bindings generated for them.
///     //
///     // By default this is an empty list.
///     features: ["foo", "bar", "baz"],
///
///     // Disables generation of a `#[used]` static to try harder to get the
///     // custom section describing WIT types linked into the binary when
///     // used in library-like situations. This is `false` by default with
///     // `#[used]` statics being emitted.
///     disable_custom_section_link_helpers: false,
///
///     // Write generated code to a .rs file, which allows the compiler to
///     // emit more useful diagnostics for errors in the generated code.  This
///     // is primarily useful for `wit-bindgen` developers.
///     //
///     // This does the same thing as setting `WIT_BINDGEN_DEBUG=1`, except
///     // that it can be used on a more fine-grained basis (i.e. it only affects
///     // the specific `generate!` call where it is used.
///     debug: true,
///
///     // Generate async import and/or export bindings.
///     //
///     // The resulting bindings will use the component model
///     // [async ABI](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Async.md).
///     //
///     // If this option is not provided then the WIT's source annotation will
///     // be used instead.
///     async: true,    // all bindings are async
///     async: false,   // all bindings are sync
///     // With an array per-function configuration can be specified. A leading
///     // '-' will disable async for that particular function.
///     async: [
///         "wasi:http/types@0.3.0-draft#[static]body.finish",
///         "import:wasi:http/handler@0.3.0-draft#handle",
///         "-export:wasi:http/handler@0.3.0-draft#handle",
///         "all",
///     ],
/// });
/// ```
///
/// [WIT package]: https://component-model.bytecodealliance.org/design/packages.html
#[cfg(feature = "macros")]
pub use wit_bindgen_rust_macro::generate;

#[cfg(docsrs)]
pub mod examples;

#[doc(hidden)]
pub mod rt;

#[cfg(feature = "async")]
pub use rt::async_support::{
    backpressure_dec, backpressure_inc, backpressure_set, block_on, spawn, yield_async,
    yield_blocking, AbiBuffer, FutureRead, FutureReader, FutureWrite, FutureWriteCancel,
    FutureWriteError, FutureWriter, StreamRead, StreamReader, StreamResult, StreamWrite,
    StreamWriter,
};
