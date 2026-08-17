#![doc(html_root_url = "https://docs.rs/prost-build/0.12.6")]
#![allow(clippy::option_as_ref_deref, clippy::format_push_string)]

//! `prost-build` compiles `.proto` files into Rust.
//!
//! `prost-build` is designed to be used for build-time code generation as part of a Cargo
//! build-script.
//!
//! ## Example
//!
//! Let's create a small crate, `snazzy`, that defines a collection of
//! snazzy new items in a protobuf file.
//!
//! ```bash
//! $ cargo new snazzy && cd snazzy
//! ```
//!
//! First, add `prost-build`, `prost` and its public dependencies to `Cargo.toml`
//! (see [crates.io](https://crates.io/crates/prost) for the current versions):
//!
//! ```toml
//! [dependencies]
//! bytes = <bytes-version>
//! prost = <prost-version>
//!
//! [build-dependencies]
//! prost-build = { version = <prost-version> }
//! ```
//!
//! Next, add `src/items.proto` to the project:
//!
//! ```proto
//! syntax = "proto3";
//!
//! package snazzy.items;
//!
//! // A snazzy new shirt!
//! message Shirt {
//!     enum Size {
//!         SMALL = 0;
//!         MEDIUM = 1;
//!         LARGE = 2;
//!     }
//!
//!     string color = 1;
//!     Size size = 2;
//! }
//! ```
//!
//! To generate Rust code from `items.proto`, we use `prost-build` in the crate's
//! `build.rs` build-script:
//!
//! ```rust,no_run
//! use std::io::Result;
//! fn main() -> Result<()> {
//!     prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
//!     Ok(())
//! }
//! ```
//!
//! And finally, in `lib.rs`, include the generated code:
//!
//! ```rust,ignore
//! // Include the `items` module, which is generated from items.proto.
//! // It is important to maintain the same structure as in the proto.
//! pub mod snazzy {
//!     pub mod items {
//!         include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
//!     }
//! }
//!
//! use snazzy::items;
//!
//! pub fn create_large_shirt(color: String) -> items::Shirt {
//!     let mut shirt = items::Shirt::default();
//!     shirt.color = color;
//!     shirt.set_size(items::shirt::Size::Large);
//!     shirt
//! }
//! ```
//!
//! That's it! Run `cargo doc` to see documentation for the generated code. The full
//! example project can be found on [GitHub](https://github.com/danburkert/snazzy).
//!
//! ## Feature Flags
//! - `format`: Format the generated output. This feature is enabled by default.
//! - `cleanup-markdown`: Clean up Markdown in protobuf docs. Enable this to clean up protobuf files from third parties.
//!
//! ### Cleaning up Markdown in code docs
//!
//! If you are using protobuf files from third parties, where the author of the protobuf
//! is not treating comments as Markdown, or is, but has codeblocks in their docs,
//! then you may need to clean up the documentation in order that `cargo test --doc`
//! will not fail spuriously, and that `cargo doc` doesn't attempt to render the
//! codeblocks as Rust code.
//!
//! To do this, in your `Cargo.toml`, add `features = ["cleanup-markdown"]` to the inclusion
//! of the `prost-build` crate and when your code is generated, the code docs will automatically
//! be cleaned up a bit.
//!
//! ## Sourcing `protoc`
//!
//! `prost-build` depends on the Protocol Buffers compiler, `protoc`, to parse `.proto` files into
//! a representation that can be transformed into Rust. If set, `prost-build` uses the `PROTOC`
//! for locating `protoc`. For example, on a macOS system where Protobuf is installed
//! with Homebrew, set the environment variables to:
//!
//! ```bash
//! PROTOC=/usr/local/bin/protoc
//! ```
//!
//! and in a typical Linux installation:
//!
//! ```bash
//! PROTOC=/usr/bin/protoc
//! ```
//!
//! If no `PROTOC` environment variable is set then `prost-build` will search the
//! current path for `protoc` or `protoc.exe`. If `prost-build` can not find `protoc`
//! via these methods the `compile_protos` method will fail.
//!
//! ### Compiling `protoc` from source
//!
//! To compile `protoc` from source you can use the `protobuf-src` crate and
//! set the correct environment variables.
//! ```no_run,ignore, rust
//! std::env::set_var("PROTOC", protobuf_src::protoc());
//!
//! // Now compile your proto files via prost-build
//! ```
//!
//! [`protobuf-src`]: https://docs.rs/protobuf-src

use std::io::Result;
use std::path::Path;

use prost_types::FileDescriptorSet;

mod ast;
pub use crate::ast::{Comments, Method, Service};

mod collections;
pub(crate) use collections::{BytesType, MapType};

mod code_generator;
mod extern_paths;
mod ident;
mod message_graph;
mod path;

mod config;
pub use config::{
    error_message_protoc_not_found, protoc_from_env, protoc_include_from_env, Config,
};

mod module;
pub use module::Module;

/// A service generator takes a service descriptor and generates Rust code.
///
/// `ServiceGenerator` can be used to generate application-specific interfaces
/// or implementations for Protobuf service definitions.
///
/// Service generators are registered with a code generator using the
/// `Config::service_generator` method.
///
/// A viable scenario is that an RPC framework provides a service generator. It generates a trait
/// describing methods of the service and some glue code to call the methods of the trait, defining
/// details like how errors are handled or if it is asynchronous. Then the user provides an
/// implementation of the generated trait in the application code and plugs it into the framework.
///
/// Such framework isn't part of Prost at present.
pub trait ServiceGenerator {
    /// Generates a Rust interface or implementation for a service, writing the
    /// result to `buf`.
    fn generate(&mut self, service: Service, buf: &mut String);

    /// Finalizes the generation process.
    ///
    /// In case there's something that needs to be output at the end of the generation process, it
    /// goes here. Similar to [`generate`](#method.generate), the output should be appended to
    /// `buf`.
    ///
    /// An example can be a module or other thing that needs to appear just once, not for each
    /// service generated.
    ///
    /// This still can be called multiple times in a lifetime of the service generator, because it
    /// is called once per `.proto` file.
    ///
    /// The default implementation is empty and does nothing.
    fn finalize(&mut self, _buf: &mut String) {}

    /// Finalizes the generation process for an entire protobuf package.
    ///
    /// This differs from [`finalize`](#method.finalize) by where (and how often) it is called
    /// during the service generator life cycle. This method is called once per protobuf package,
    /// making it ideal for grouping services within a single package spread across multiple
    /// `.proto` files.
    ///
    /// The default implementation is empty and does nothing.
    fn finalize_package(&mut self, _package: &str, _buf: &mut String) {}
}

/// Compile `.proto` files into Rust files during a Cargo build.
///
/// The generated `.rs` files are written to the Cargo `OUT_DIR` directory, suitable for use with
/// the [include!][1] macro. See the [Cargo `build.rs` code generation][2] example for more info.
///
/// This function should be called in a project's `build.rs`.
///
/// # Arguments
///
/// **`protos`** - Paths to `.proto` files to compile. Any transitively [imported][3] `.proto`
/// files are automatically be included.
///
/// **`includes`** - Paths to directories in which to search for imports. Directories are searched
/// in order. The `.proto` files passed in **`protos`** must be found in one of the provided
/// include directories.
///
/// # Errors
///
/// This function can fail for a number of reasons:
///
///   - Failure to locate or download `protoc`.
///   - Failure to parse the `.proto`s.
///   - Failure to locate an imported `.proto`.
///   - Failure to compile a `.proto` without a [package specifier][4].
///
/// It's expected that this function call be `unwrap`ed in a `build.rs`; there is typically no
/// reason to gracefully recover from errors during a build.
///
/// # Example `build.rs`
///
/// ```rust,no_run
/// # use std::io::Result;
/// fn main() -> Result<()> {
///   prost_build::compile_protos(&["src/frontend.proto", "src/backend.proto"], &["src"])?;
///   Ok(())
/// }
/// ```
///
/// [1]: https://doc.rust-lang.org/std/macro.include.html
/// [2]: http://doc.crates.io/build-script.html#case-study-code-generation
/// [3]: https://developers.google.com/protocol-buffers/docs/proto3#importing-definitions
/// [4]: https://developers.google.com/protocol-buffers/docs/proto#packages
pub fn compile_protos(protos: &[impl AsRef<Path>], includes: &[impl AsRef<Path>]) -> Result<()> {
    Config::new().compile_protos(protos, includes)
}

/// Compile a [`FileDescriptorSet`] into Rust files during a Cargo build.
///
/// The generated `.rs` files are written to the Cargo `OUT_DIR` directory, suitable for use with
/// the [include!][1] macro. See the [Cargo `build.rs` code generation][2] example for more info.
///
/// This function should be called in a project's `build.rs`.
///
/// This function can be combined with a crate like [`protox`] which outputs a
/// [`FileDescriptorSet`] and is a pure Rust implementation of `protoc`.
///
/// [`protox`]: https://github.com/andrewhickman/protox
///
/// # Example
/// ```rust,no_run
/// # use prost_types::FileDescriptorSet;
/// # fn fds() -> FileDescriptorSet { todo!() }
/// fn main() -> std::io::Result<()> {
///   let file_descriptor_set = fds();
///
///   prost_build::compile_fds(file_descriptor_set)
/// }
/// ```
pub fn compile_fds(fds: FileDescriptorSet) -> Result<()> {
    Config::new().compile_fds(fds)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::fs::File;
    use std::io::Read;
    use std::rc::Rc;

    use super::*;

    /// An example service generator that generates a trait with methods corresponding to the
    /// service methods.
    struct ServiceTraitGenerator;

    impl ServiceGenerator for ServiceTraitGenerator {
        fn generate(&mut self, service: Service, buf: &mut String) {
            // Generate a trait for the service.
            service.comments.append_with_indent(0, buf);
            buf.push_str(&format!("trait {} {{\n", &service.name));

            // Generate the service methods.
            for method in service.methods {
                method.comments.append_with_indent(1, buf);
                buf.push_str(&format!(
                    "    fn {}(_: {}) -> {};\n",
                    method.name, method.input_type, method.output_type
                ));
            }

            // Close out the trait.
            buf.push_str("}\n");
        }
        fn finalize(&mut self, buf: &mut String) {
            // Needs to be present only once, no matter how many services there are
            buf.push_str("pub mod utils { }\n");
        }
    }

    /// Implements `ServiceGenerator` and provides some state for assertions.
    struct MockServiceGenerator {
        state: Rc<RefCell<MockState>>,
    }

    /// Holds state for `MockServiceGenerator`
    #[derive(Default)]
    struct MockState {
        service_names: Vec<String>,
        package_names: Vec<String>,
        finalized: u32,
    }

    impl MockServiceGenerator {
        fn new(state: Rc<RefCell<MockState>>) -> Self {
            Self { state }
        }
    }

    impl ServiceGenerator for MockServiceGenerator {
        fn generate(&mut self, service: Service, _buf: &mut String) {
            let mut state = self.state.borrow_mut();
            state.service_names.push(service.name);
        }

        fn finalize(&mut self, _buf: &mut String) {
            let mut state = self.state.borrow_mut();
            state.finalized += 1;
        }

        fn finalize_package(&mut self, package: &str, _buf: &mut String) {
            let mut state = self.state.borrow_mut();
            state.package_names.push(package.to_string());
        }
    }

    #[test]
    fn smoke_test() {
        let _ = env_logger::try_init();
        let tempdir = tempfile::tempdir().unwrap();

        Config::new()
            .service_generator(Box::new(ServiceTraitGenerator))
            .out_dir(tempdir.path())
            .compile_protos(&["src/fixtures/smoke_test/smoke_test.proto"], &["src"])
            .unwrap();
    }

    #[test]
    fn finalize_package() {
        let _ = env_logger::try_init();
        let tempdir = tempfile::tempdir().unwrap();

        let state = Rc::new(RefCell::new(MockState::default()));
        let gen = MockServiceGenerator::new(Rc::clone(&state));

        Config::new()
            .service_generator(Box::new(gen))
            .include_file("_protos.rs")
            .out_dir(tempdir.path())
            .compile_protos(
                &[
                    "src/fixtures/helloworld/hello.proto",
                    "src/fixtures/helloworld/goodbye.proto",
                ],
                &["src/fixtures/helloworld"],
            )
            .unwrap();

        let state = state.borrow();
        assert_eq!(&state.service_names, &["Greeting", "Farewell"]);
        assert_eq!(&state.package_names, &["helloworld"]);
        assert_eq!(state.finalized, 3);
    }

    #[test]
    fn test_generate_message_attributes() {
        let _ = env_logger::try_init();
        let tempdir = tempfile::tempdir().unwrap();

        Config::new()
            .out_dir(tempdir.path())
            .message_attribute(".", "#[derive(derive_builder::Builder)]")
            .enum_attribute(".", "#[some_enum_attr(u8)]")
            .compile_protos(
                &["src/fixtures/helloworld/hello.proto"],
                &["src/fixtures/helloworld"],
            )
            .unwrap();

        let out_file = tempdir.path().join("helloworld.rs");
        #[cfg(feature = "format")]
        let expected_content =
            read_all_content("src/fixtures/helloworld/_expected_helloworld_formatted.rs")
                .replace("\r\n", "\n");
        #[cfg(not(feature = "format"))]
        let expected_content = read_all_content("src/fixtures/helloworld/_expected_helloworld.rs")
            .replace("\r\n", "\n");
        let content = read_all_content(out_file).replace("\r\n", "\n");
        assert_eq!(
            expected_content, content,
            "Unexpected content: \n{}",
            content
        );
    }

    #[test]
    fn test_generate_no_empty_outputs() {
        let _ = env_logger::try_init();
        let state = Rc::new(RefCell::new(MockState::default()));
        let gen = MockServiceGenerator::new(Rc::clone(&state));
        let include_file = "_include.rs";
        let tempdir = tempfile::tempdir().unwrap();
        let previously_empty_proto_path = tempdir.path().join(Path::new("google.protobuf.rs"));

        Config::new()
            .service_generator(Box::new(gen))
            .include_file(include_file)
            .out_dir(tempdir.path())
            .compile_protos(
                &["src/fixtures/imports_empty/imports_empty.proto"],
                &["src/fixtures/imports_empty"],
            )
            .unwrap();

        // Prior to PR introducing this test, the generated include file would have the file
        // google.protobuf.rs which was an empty file. Now that file should only exist if it has content
        if let Ok(mut f) = File::open(previously_empty_proto_path) {
            // Since this file was generated, it should not be empty.
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            assert!(!contents.is_empty());
        } else {
            // The file wasn't generated so the result include file should not reference it
            let expected = read_all_content("src/fixtures/imports_empty/_expected_include.rs");
            let actual = read_all_content(tempdir.path().join(Path::new(include_file)));
            // Normalizes windows and Linux-style EOL
            let expected = expected.replace("\r\n", "\n");
            let actual = actual.replace("\r\n", "\n");
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_generate_field_attributes() {
        let _ = env_logger::try_init();
        let tempdir = tempfile::tempdir().unwrap();

        Config::new()
            .out_dir(tempdir.path())
            .boxed("Container.data.foo")
            .boxed("Bar.qux")
            .compile_protos(
                &["src/fixtures/field_attributes/field_attributes.proto"],
                &["src/fixtures/field_attributes"],
            )
            .unwrap();

        let out_file = tempdir.path().join("field_attributes.rs");

        let content = read_all_content(out_file).replace("\r\n", "\n");

        #[cfg(feature = "format")]
        let expected_content = read_all_content(
            "src/fixtures/field_attributes/_expected_field_attributes_formatted.rs",
        )
        .replace("\r\n", "\n");
        #[cfg(not(feature = "format"))]
        let expected_content =
            read_all_content("src/fixtures/field_attributes/_expected_field_attributes.rs")
                .replace("\r\n", "\n");

        assert_eq!(
            expected_content, content,
            "Unexpected content: \n{}",
            content
        );
    }

    #[test]
    fn deterministic_include_file() {
        let _ = env_logger::try_init();

        for _ in 1..10 {
            let state = Rc::new(RefCell::new(MockState::default()));
            let gen = MockServiceGenerator::new(Rc::clone(&state));
            let include_file = "_include.rs";
            let tempdir = tempfile::tempdir().unwrap();

            Config::new()
                .service_generator(Box::new(gen))
                .include_file(include_file)
                .out_dir(tempdir.path())
                .compile_protos(
                    &[
                        "src/fixtures/alphabet/a.proto",
                        "src/fixtures/alphabet/b.proto",
                        "src/fixtures/alphabet/c.proto",
                        "src/fixtures/alphabet/d.proto",
                        "src/fixtures/alphabet/e.proto",
                        "src/fixtures/alphabet/f.proto",
                    ],
                    &["src/fixtures/alphabet"],
                )
                .unwrap();

            let expected = read_all_content("src/fixtures/alphabet/_expected_include.rs");
            let actual = read_all_content(tempdir.path().join(Path::new(include_file)));
            // Normalizes windows and Linux-style EOL
            let expected = expected.replace("\r\n", "\n");
            let actual = actual.replace("\r\n", "\n");

            assert_eq!(expected, actual);
        }
    }

    fn read_all_content(filepath: impl AsRef<Path>) -> String {
        let mut f = File::open(filepath).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn write_includes() {
        let modules = [
            Module::from_protobuf_package_name("foo.bar.baz"),
            Module::from_protobuf_package_name(""),
            Module::from_protobuf_package_name("foo.bar"),
            Module::from_protobuf_package_name("bar"),
            Module::from_protobuf_package_name("foo"),
            Module::from_protobuf_package_name("foo.bar.qux"),
            Module::from_protobuf_package_name("foo.bar.a.b.c"),
        ];

        let file_names = modules
            .iter()
            .map(|m| (m.clone(), m.to_file_name_or("_.default")))
            .collect();

        let mut buf = Vec::new();
        Config::new()
            .default_package_filename("_.default")
            .write_includes(modules.iter().collect(), &mut buf, None, &file_names)
            .unwrap();
        let expected =
            read_all_content("src/fixtures/write_includes/_.includes.rs").replace("\r\n", "\n");
        let actual = String::from_utf8(buf).unwrap();
        assert_eq!(expected, actual);
    }
}
