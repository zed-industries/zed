//! Generate Rust bindings for C and C++ libraries.
//!
//! Provide a C/C++ header file, receive Rust FFI code to call into C/C++
//! functions and use types defined in the header.
//!
//! See the [`Builder`](./struct.Builder.html) struct for usage.
//!
//! See the [Users Guide](https://rust-lang.github.io/rust-bindgen/) for
//! additional documentation.
#![deny(missing_docs)]
#![deny(unused_extern_crates)]
#![deny(clippy::disallowed_methods)]
// To avoid rather annoying warnings when matching with CXCursor_xxx as a
// constant.
#![allow(non_upper_case_globals)]
// `quote!` nests quite deeply.
#![recursion_limit = "128"]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate quote;

#[cfg(feature = "logging")]
#[macro_use]
extern crate log;

#[cfg(not(feature = "logging"))]
#[macro_use]
mod log_stubs;

#[macro_use]
mod extra_assertions;

mod codegen;
mod deps;
mod options;
mod time;

pub mod callbacks;

mod clang;
#[cfg(feature = "experimental")]
mod diagnostics;
mod features;
mod ir;
mod parse;
mod regex_set;

pub use codegen::{
    AliasVariation, EnumVariation, MacroTypeVariation, NonCopyUnionStyle,
};
pub use features::{RustEdition, RustTarget, LATEST_STABLE_RUST};
pub use ir::annotations::FieldVisibilityKind;
pub use ir::function::Abi;
#[cfg(feature = "__cli")]
pub use options::cli::builder_from_flags;

use codegen::CodegenError;
use features::RustFeatures;
use ir::comment;
use ir::context::{BindgenContext, ItemId};
use ir::item::Item;
use options::BindgenOptions;
use parse::ParseError;

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::env;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::str::FromStr;

// Some convenient typedefs for a fast hash map and hash set.
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
type HashSet<K> = rustc_hash::FxHashSet<K>;

/// Default prefix for the anon fields.
pub const DEFAULT_ANON_FIELDS_PREFIX: &str = "__bindgen_anon_";

const DEFAULT_NON_EXTERN_FNS_SUFFIX: &str = "__extern";

fn file_is_cpp(name_file: &str) -> bool {
    name_file.ends_with(".hpp") ||
        name_file.ends_with(".hxx") ||
        name_file.ends_with(".hh") ||
        name_file.ends_with(".h++")
}

fn args_are_cpp(clang_args: &[Box<str>]) -> bool {
    for w in clang_args.windows(2) {
        if w[0].as_ref() == "-xc++" || w[1].as_ref() == "-xc++" {
            return true;
        }
        if w[0].as_ref() == "-x" && w[1].as_ref() == "c++" {
            return true;
        }
        if w[0].as_ref() == "-include" && file_is_cpp(w[1].as_ref()) {
            return true;
        }
    }
    false
}

bitflags! {
    /// A type used to indicate which kind of items we have to generate.
    #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CodegenConfig: u32 {
        /// Whether to generate functions.
        const FUNCTIONS = 1 << 0;
        /// Whether to generate types.
        const TYPES = 1 << 1;
        /// Whether to generate constants.
        const VARS = 1 << 2;
        /// Whether to generate methods.
        const METHODS = 1 << 3;
        /// Whether to generate constructors
        const CONSTRUCTORS = 1 << 4;
        /// Whether to generate destructors.
        const DESTRUCTORS = 1 << 5;
    }
}

impl CodegenConfig {
    /// Returns true if functions should be generated.
    pub fn functions(self) -> bool {
        self.contains(CodegenConfig::FUNCTIONS)
    }

    /// Returns true if types should be generated.
    pub fn types(self) -> bool {
        self.contains(CodegenConfig::TYPES)
    }

    /// Returns true if constants should be generated.
    pub fn vars(self) -> bool {
        self.contains(CodegenConfig::VARS)
    }

    /// Returns true if methods should be generated.
    pub fn methods(self) -> bool {
        self.contains(CodegenConfig::METHODS)
    }

    /// Returns true if constructors should be generated.
    pub fn constructors(self) -> bool {
        self.contains(CodegenConfig::CONSTRUCTORS)
    }

    /// Returns true if destructors should be generated.
    pub fn destructors(self) -> bool {
        self.contains(CodegenConfig::DESTRUCTORS)
    }
}

impl Default for CodegenConfig {
    fn default() -> Self {
        CodegenConfig::all()
    }
}

/// Formatting tools that can be used to format the bindings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Formatter {
    /// Do not format the bindings.
    None,
    /// Use `rustfmt` to format the bindings.
    Rustfmt,
    #[cfg(feature = "prettyplease")]
    /// Use `prettyplease` to format the bindings.
    Prettyplease,
}

impl Default for Formatter {
    fn default() -> Self {
        Self::Rustfmt
    }
}

impl FromStr for Formatter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "rustfmt" => Ok(Self::Rustfmt),
            #[cfg(feature = "prettyplease")]
            "prettyplease" => Ok(Self::Prettyplease),
            _ => Err(format!("`{s}` is not a valid formatter")),
        }
    }
}

impl std::fmt::Display for Formatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::None => "none",
            Self::Rustfmt => "rustfmt",
            #[cfg(feature = "prettyplease")]
            Self::Prettyplease => "prettyplease",
        };

        s.fmt(f)
    }
}

/// Configure and generate Rust bindings for a C/C++ header.
///
/// This is the main entry point to the library.
///
/// ```ignore
/// use bindgen::builder;
///
/// // Configure and generate bindings.
/// let bindings = builder().header("path/to/input/header")
///     .allowlist_type("SomeCoolClass")
///     .allowlist_function("do_some_cool_thing")
///     .generate()?;
///
/// // Write the generated bindings to an output file.
/// bindings.write_to_file("path/to/output.rs")?;
/// ```
///
/// # Enums
///
/// Bindgen can map C/C++ enums into Rust in different ways. The way bindgen maps enums depends on
/// the pattern passed to several methods:
///
/// 1. [`constified_enum_module()`](#method.constified_enum_module)
/// 2. [`bitfield_enum()`](#method.bitfield_enum)
/// 3. [`newtype_enum()`](#method.newtype_enum)
/// 4. [`rustified_enum()`](#method.rustified_enum)
/// 5. [`rustified_non_exhaustive_enum()`](#method.rustified_non_exhaustive_enum)
///
/// For each C enum, bindgen tries to match the pattern in the following order:
///
/// 1. Constified enum module
/// 2. Bitfield enum
/// 3. Newtype enum
/// 4. Rustified enum
///
/// If none of the above patterns match, then bindgen will generate a set of Rust constants.
///
/// # Clang arguments
///
/// Extra arguments can be passed to with clang:
/// 1. [`clang_arg()`](#method.clang_arg): takes a single argument
/// 2. [`clang_args()`](#method.clang_args): takes an iterator of arguments
/// 3. `BINDGEN_EXTRA_CLANG_ARGS` environment variable: whitespace separate
///    environment variable of arguments
///
/// Clang arguments specific to your crate should be added via the
/// `clang_arg()`/`clang_args()` methods.
///
/// End-users of the crate may need to set the `BINDGEN_EXTRA_CLANG_ARGS` environment variable to
/// add additional arguments. For example, to build against a different sysroot a user could set
/// `BINDGEN_EXTRA_CLANG_ARGS` to `--sysroot=/path/to/sysroot`.
///
/// # Regular expression arguments
///
/// Some [`Builder`] methods, such as `allowlist_*` and `blocklist_*`, allow regular
/// expressions as arguments. These regular expressions will be enclosed in parentheses and
/// anchored with `^` and `$`. So, if the argument passed is `<regex>`, the regular expression to be
/// stored will be `^(<regex>)$`.
///
/// As a consequence, regular expressions passed to `bindgen` will try to match the whole name of
/// an item instead of a section of it, which means that to match any items with the prefix
/// `prefix`, the `prefix.*` regular expression must be used.
///
/// Certain methods, like [`Builder::allowlist_function`], use regular expressions over function
/// names. To match C++ methods, prefix the name of the type where they belong, followed by an
/// underscore. So, if the type `Foo` has a method `bar`, it can be matched with the `Foo_bar`
/// regular expression.
///
/// Additionally, Objective-C interfaces can be matched by prefixing the regular expression with
/// `I`. For example, the `IFoo` regular expression matches the `Foo` interface, and the `IFoo_foo`
/// regular expression matches the `foo` method of the `Foo` interface.
///
/// Releases of `bindgen` with a version lesser or equal to `0.62.0` used to accept the wildcard
/// pattern `*` as a valid regular expression. This behavior has been deprecated, and the `.*`
/// regular expression must be used instead.
#[derive(Debug, Default, Clone)]
pub struct Builder {
    options: BindgenOptions,
}

/// Construct a new [`Builder`](./struct.Builder.html).
pub fn builder() -> Builder {
    Default::default()
}

fn get_extra_clang_args(
    parse_callbacks: &[Rc<dyn callbacks::ParseCallbacks>],
) -> Vec<String> {
    // Add any extra arguments from the environment to the clang command line.
    let extra_clang_args = match get_target_dependent_env_var(
        parse_callbacks,
        "BINDGEN_EXTRA_CLANG_ARGS",
    ) {
        None => return vec![],
        Some(s) => s,
    };

    // Try to parse it with shell quoting. If we fail, make it one single big argument.
    if let Some(strings) = shlex::split(&extra_clang_args) {
        return strings;
    }
    vec![extra_clang_args]
}

impl Builder {
    /// Generate the Rust bindings using the options built up thus far.
    pub fn generate(mut self) -> Result<Bindings, BindgenError> {
        // Keep rust_features synced with rust_target
        self.options.rust_features = match self.options.rust_edition {
            Some(edition) => {
                if !edition.is_available(self.options.rust_target) {
                    return Err(BindgenError::UnsupportedEdition(
                        edition,
                        self.options.rust_target,
                    ));
                }
                RustFeatures::new(self.options.rust_target, edition)
            }
            None => {
                RustFeatures::new_with_latest_edition(self.options.rust_target)
            }
        };

        // Add any extra arguments from the environment to the clang command line.
        self.options.clang_args.extend(
            get_extra_clang_args(&self.options.parse_callbacks)
                .into_iter()
                .map(String::into_boxed_str),
        );

        for header in &self.options.input_headers {
            self.options
                .for_each_callback(|cb| cb.header_file(header.as_ref()));
        }

        // Transform input headers to arguments on the clang command line.
        self.options.clang_args.extend(
            self.options.input_headers
                [..self.options.input_headers.len().saturating_sub(1)]
                .iter()
                .flat_map(|header| ["-include".into(), header.clone()]),
        );

        let input_unsaved_files =
            std::mem::take(&mut self.options.input_header_contents)
                .into_iter()
                .map(|(name, contents)| {
                    clang::UnsavedFile::new(name.as_ref(), contents.as_ref())
                })
                .collect::<Vec<_>>();

        Bindings::generate(self.options, input_unsaved_files)
    }

    /// Preprocess and dump the input header files to disk.
    ///
    /// This is useful when debugging bindgen, using C-Reduce, or when filing
    /// issues. The resulting file will be named something like `__bindgen.i` or
    /// `__bindgen.ii`
    pub fn dump_preprocessed_input(&self) -> io::Result<()> {
        let clang =
            clang_sys::support::Clang::find(None, &[]).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "Cannot find clang executable",
                )
            })?;

        // The contents of a wrapper file that includes all the input header
        // files.
        let mut wrapper_contents = String::new();

        // Whether we are working with C or C++ inputs.
        let mut is_cpp = args_are_cpp(&self.options.clang_args);

        // For each input header, add `#include "$header"`.
        for header in &self.options.input_headers {
            is_cpp |= file_is_cpp(header);

            wrapper_contents.push_str("#include \"");
            wrapper_contents.push_str(header);
            wrapper_contents.push_str("\"\n");
        }

        // For each input header content, add a prefix line of `#line 0 "$name"`
        // followed by the contents.
        for (name, contents) in &self.options.input_header_contents {
            is_cpp |= file_is_cpp(name);

            wrapper_contents.push_str("#line 0 \"");
            wrapper_contents.push_str(name);
            wrapper_contents.push_str("\"\n");
            wrapper_contents.push_str(contents);
        }

        let wrapper_path = PathBuf::from(if is_cpp {
            "__bindgen.cpp"
        } else {
            "__bindgen.c"
        });

        {
            let mut wrapper_file = File::create(&wrapper_path)?;
            wrapper_file.write_all(wrapper_contents.as_bytes())?;
        }

        let mut cmd = Command::new(clang.path);
        cmd.arg("-save-temps")
            .arg("-E")
            .arg("-C")
            .arg("-c")
            .arg(&wrapper_path)
            .stdout(Stdio::piped());

        for a in &self.options.clang_args {
            cmd.arg(a.as_ref());
        }

        for a in get_extra_clang_args(&self.options.parse_callbacks) {
            cmd.arg(a);
        }

        let mut child = cmd.spawn()?;

        let mut preprocessed = child.stdout.take().unwrap();
        let mut file = File::create(if is_cpp {
            "__bindgen.ii"
        } else {
            "__bindgen.i"
        })?;
        io::copy(&mut preprocessed, &mut file)?;

        if child.wait()?.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "clang exited with non-zero status",
            ))
        }
    }
}

impl BindgenOptions {
    fn build(&mut self) {
        const REGEX_SETS_LEN: usize = 29;

        let regex_sets: [_; REGEX_SETS_LEN] = [
            &mut self.blocklisted_types,
            &mut self.blocklisted_functions,
            &mut self.blocklisted_items,
            &mut self.blocklisted_files,
            &mut self.blocklisted_vars,
            &mut self.opaque_types,
            &mut self.allowlisted_vars,
            &mut self.allowlisted_types,
            &mut self.allowlisted_functions,
            &mut self.allowlisted_files,
            &mut self.allowlisted_items,
            &mut self.bitfield_enums,
            &mut self.constified_enums,
            &mut self.constified_enum_modules,
            &mut self.newtype_enums,
            &mut self.newtype_global_enums,
            &mut self.rustified_enums,
            &mut self.rustified_non_exhaustive_enums,
            &mut self.type_alias,
            &mut self.new_type_alias,
            &mut self.new_type_alias_deref,
            &mut self.bindgen_wrapper_union,
            &mut self.manually_drop_union,
            &mut self.no_partialeq_types,
            &mut self.no_copy_types,
            &mut self.no_debug_types,
            &mut self.no_default_types,
            &mut self.no_hash_types,
            &mut self.must_use_types,
        ];

        let record_matches = self.record_matches;
        #[cfg(feature = "experimental")]
        {
            let sets_len = REGEX_SETS_LEN + self.abi_overrides.len();
            let names = if self.emit_diagnostics {
                <[&str; REGEX_SETS_LEN]>::into_iter([
                    "--blocklist-type",
                    "--blocklist-function",
                    "--blocklist-item",
                    "--blocklist-file",
                    "--blocklist-var",
                    "--opaque-type",
                    "--allowlist-type",
                    "--allowlist-function",
                    "--allowlist-var",
                    "--allowlist-file",
                    "--allowlist-item",
                    "--bitfield-enum",
                    "--newtype-enum",
                    "--newtype-global-enum",
                    "--rustified-enum",
                    "--rustified-enum-non-exhaustive",
                    "--constified-enum-module",
                    "--constified-enum",
                    "--type-alias",
                    "--new-type-alias",
                    "--new-type-alias-deref",
                    "--bindgen-wrapper-union",
                    "--manually-drop-union",
                    "--no-partialeq",
                    "--no-copy",
                    "--no-debug",
                    "--no-default",
                    "--no-hash",
                    "--must-use",
                ])
                .chain((0..self.abi_overrides.len()).map(|_| "--override-abi"))
                .map(Some)
                .collect()
            } else {
                vec![None; sets_len]
            };

            for (regex_set, name) in
                self.abi_overrides.values_mut().chain(regex_sets).zip(names)
            {
                regex_set.build_with_diagnostics(record_matches, name);
            }
        }
        #[cfg(not(feature = "experimental"))]
        for regex_set in self.abi_overrides.values_mut().chain(regex_sets) {
            regex_set.build(record_matches);
        }
    }

    /// Update rust target version
    pub fn set_rust_target(&mut self, rust_target: RustTarget) {
        self.rust_target = rust_target;
    }

    /// Get features supported by target Rust version
    pub fn rust_features(&self) -> RustFeatures {
        self.rust_features
    }

    fn last_callback<T>(
        &self,
        f: impl Fn(&dyn callbacks::ParseCallbacks) -> Option<T>,
    ) -> Option<T> {
        self.parse_callbacks
            .iter()
            .filter_map(|cb| f(cb.as_ref()))
            .last()
    }

    fn all_callbacks<T>(
        &self,
        f: impl Fn(&dyn callbacks::ParseCallbacks) -> Vec<T>,
    ) -> Vec<T> {
        self.parse_callbacks
            .iter()
            .flat_map(|cb| f(cb.as_ref()))
            .collect()
    }

    fn for_each_callback(&self, f: impl Fn(&dyn callbacks::ParseCallbacks)) {
        self.parse_callbacks.iter().for_each(|cb| f(cb.as_ref()));
    }

    fn process_comment(&self, comment: &str) -> String {
        let comment = comment::preprocess(comment);
        self.parse_callbacks
            .last()
            .and_then(|cb| cb.process_comment(&comment))
            .unwrap_or(comment)
    }
}

#[cfg(feature = "runtime")]
fn ensure_libclang_is_loaded() {
    use std::sync::{Arc, OnceLock};

    if clang_sys::is_loaded() {
        return;
    }

    // XXX (issue #350): Ensure that our dynamically loaded `libclang`
    // doesn't get dropped prematurely, nor is loaded multiple times
    // across different threads.

    static LIBCLANG: OnceLock<Arc<clang_sys::SharedLibrary>> = OnceLock::new();
    let libclang = LIBCLANG.get_or_init(|| {
        clang_sys::load().expect("Unable to find libclang");
        clang_sys::get_library()
            .expect("We just loaded libclang and it had better still be here!")
    });

    clang_sys::set_library(Some(libclang.clone()));
}

#[cfg(not(feature = "runtime"))]
fn ensure_libclang_is_loaded() {}

/// Error type for rust-bindgen.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BindgenError {
    /// The header was a folder.
    FolderAsHeader(PathBuf),
    /// Permissions to read the header is insufficient.
    InsufficientPermissions(PathBuf),
    /// The header does not exist.
    NotExist(PathBuf),
    /// Clang diagnosed an error.
    ClangDiagnostic(String),
    /// Code generation reported an error.
    Codegen(CodegenError),
    /// The passed edition is not available on that Rust target.
    UnsupportedEdition(RustEdition, RustTarget),
}

impl std::fmt::Display for BindgenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BindgenError::FolderAsHeader(h) => {
                write!(f, "'{}' is a folder", h.display())
            }
            BindgenError::InsufficientPermissions(h) => {
                write!(f, "insufficient permissions to read '{}'", h.display())
            }
            BindgenError::NotExist(h) => {
                write!(f, "header '{}' does not exist.", h.display())
            }
            BindgenError::ClangDiagnostic(message) => {
                write!(f, "clang diagnosed error: {message}")
            }
            BindgenError::Codegen(err) => {
                write!(f, "codegen error: {err}")
            }
            BindgenError::UnsupportedEdition(edition, target) => {
                write!(f, "edition {edition} is not available on Rust {target}")
            }
        }
    }
}

impl std::error::Error for BindgenError {}

/// Generated Rust bindings.
#[derive(Debug)]
pub struct Bindings {
    options: BindgenOptions,
    module: proc_macro2::TokenStream,
}

pub(crate) const HOST_TARGET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/host-target.txt"));

// Some architecture triplets are different between rust and libclang, see #1211
// and duplicates.
fn rust_to_clang_target(rust_target: &str) -> Box<str> {
    const TRIPLE_HYPHENS_MESSAGE: &str = "Target triple should contain hyphens";

    let mut clang_target = rust_target.to_owned();

    if clang_target.starts_with("riscv32") {
        let idx = clang_target.find('-').expect(TRIPLE_HYPHENS_MESSAGE);

        clang_target.replace_range(..idx, "riscv32");
    } else if clang_target.starts_with("riscv64") {
        let idx = clang_target.find('-').expect(TRIPLE_HYPHENS_MESSAGE);

        clang_target.replace_range(..idx, "riscv64");
    } else if clang_target.starts_with("aarch64-apple-") {
        let idx = clang_target.find('-').expect(TRIPLE_HYPHENS_MESSAGE);

        clang_target.replace_range(..idx, "arm64");
    }

    if clang_target.ends_with("-espidf") {
        let idx = clang_target.rfind('-').expect(TRIPLE_HYPHENS_MESSAGE);

        clang_target.replace_range((idx + 1).., "elf");
    }

    clang_target.into()
}

/// Returns the effective target, and whether it was explicitly specified on the
/// clang flags.
fn find_effective_target(clang_args: &[Box<str>]) -> (Box<str>, bool) {
    let mut args = clang_args.iter();
    while let Some(opt) = args.next() {
        if opt.starts_with("--target=") {
            let mut split = opt.split('=');
            split.next();
            return (split.next().unwrap().into(), true);
        }

        if opt.as_ref() == "-target" {
            if let Some(target) = args.next() {
                return (target.clone(), true);
            }
        }
    }

    // If we're running from a build script, try to find the cargo target.
    if let Ok(t) = env::var("TARGET") {
        return (rust_to_clang_target(&t), false);
    }

    (rust_to_clang_target(HOST_TARGET), false)
}

impl Bindings {
    /// Generate bindings for the given options.
    pub(crate) fn generate(
        mut options: BindgenOptions,
        input_unsaved_files: Vec<clang::UnsavedFile>,
    ) -> Result<Bindings, BindgenError> {
        ensure_libclang_is_loaded();

        #[cfg(feature = "runtime")]
        match clang_sys::get_library().unwrap().version() {
            None => {
                warn!("Could not detect a Clang version, make sure you are using libclang 9 or newer");
            }
            Some(version) => {
                if version < clang_sys::Version::V9_0 {
                    warn!("Detected Clang version {version:?} which is unsupported and can cause invalid code generation, use libclang 9 or newer");
                }
            }
        }

        #[cfg(feature = "runtime")]
        debug!(
            "Generating bindings, libclang at {}",
            clang_sys::get_library().unwrap().path().display()
        );
        #[cfg(not(feature = "runtime"))]
        debug!("Generating bindings, libclang linked");

        options.build();

        let (effective_target, explicit_target) =
            find_effective_target(&options.clang_args);

        let is_host_build =
            rust_to_clang_target(HOST_TARGET) == effective_target;

        // NOTE: The is_host_build check wouldn't be sound normally in some
        // cases if we were to call a binary (if you have a 32-bit clang and are
        // building on a 64-bit system for example).  But since we rely on
        // opening libclang.so, it has to be the same architecture and thus the
        // check is fine.
        if !explicit_target && !is_host_build {
            options.clang_args.insert(
                0,
                format!("--target={effective_target}").into_boxed_str(),
            );
        };

        fn detect_include_paths(options: &mut BindgenOptions) {
            if !options.detect_include_paths {
                return;
            }

            // Filter out include paths and similar stuff, so we don't incorrectly
            // promote them to `-isystem`.
            let clang_args_for_clang_sys = {
                let mut last_was_include_prefix = false;
                options
                    .clang_args
                    .iter()
                    .filter(|arg| {
                        if last_was_include_prefix {
                            last_was_include_prefix = false;
                            return false;
                        }

                        let arg = arg.as_ref();

                        // https://clang.llvm.org/docs/ClangCommandLineReference.html
                        // -isystem and -isystem-after are harmless.
                        if arg == "-I" || arg == "--include-directory" {
                            last_was_include_prefix = true;
                            return false;
                        }

                        if arg.starts_with("-I") ||
                            arg.starts_with("--include-directory=")
                        {
                            return false;
                        }

                        true
                    })
                    .map(|arg| arg.clone().into())
                    .collect::<Vec<_>>()
            };

            debug!(
                "Trying to find clang with flags: {clang_args_for_clang_sys:?}"
            );

            let clang = match clang_sys::support::Clang::find(
                None,
                &clang_args_for_clang_sys,
            ) {
                None => return,
                Some(clang) => clang,
            };

            debug!("Found clang: {clang:?}");

            // Whether we are working with C or C++ inputs.
            let is_cpp = args_are_cpp(&options.clang_args) ||
                options.input_headers.iter().any(|h| file_is_cpp(h));

            let search_paths = if is_cpp {
                clang.cpp_search_paths
            } else {
                clang.c_search_paths
            };

            if let Some(search_paths) = search_paths {
                for path in search_paths.into_iter() {
                    if let Ok(path) = path.into_os_string().into_string() {
                        options.clang_args.push("-isystem".into());
                        options.clang_args.push(path.into_boxed_str());
                    }
                }
            }
        }

        detect_include_paths(&mut options);

        #[cfg(unix)]
        fn can_read(perms: &std::fs::Permissions) -> bool {
            use std::os::unix::fs::PermissionsExt;
            perms.mode() & 0o444 > 0
        }

        #[cfg(not(unix))]
        fn can_read(_: &std::fs::Permissions) -> bool {
            true
        }

        if let Some(h) = options.input_headers.last() {
            let path = Path::new(h.as_ref());
            if let Ok(md) = std::fs::metadata(path) {
                if md.is_dir() {
                    return Err(BindgenError::FolderAsHeader(path.into()));
                }
                if !can_read(&md.permissions()) {
                    return Err(BindgenError::InsufficientPermissions(
                        path.into(),
                    ));
                }
                options.clang_args.push(h.clone());
            } else {
                return Err(BindgenError::NotExist(path.into()));
            }
        }

        for (idx, f) in input_unsaved_files.iter().enumerate() {
            if idx != 0 || !options.input_headers.is_empty() {
                options.clang_args.push("-include".into());
            }
            options.clang_args.push(f.name.to_str().unwrap().into());
        }

        debug!("Fixed-up options: {options:?}");

        let time_phases = options.time_phases;
        let mut context = BindgenContext::new(options, &input_unsaved_files);

        if is_host_build {
            debug_assert_eq!(
                context.target_pointer_size(),
                size_of::<*mut ()>(),
                "{effective_target:?} {HOST_TARGET:?}"
            );
        }

        {
            let _t = time::Timer::new("parse").with_output(time_phases);
            parse(&mut context)?;
        }

        let (module, options) =
            codegen::codegen(context).map_err(BindgenError::Codegen)?;

        Ok(Bindings { options, module })
    }

    /// Write these bindings as source text to a file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.as_ref())?;
        self.write(Box::new(file))?;
        Ok(())
    }

    /// Write these bindings as source text to the given `Write`able.
    pub fn write<'a>(&self, mut writer: Box<dyn Write + 'a>) -> io::Result<()> {
        const NL: &str = if cfg!(windows) { "\r\n" } else { "\n" };

        if !self.options.disable_header_comment {
            let version =
                option_env!("CARGO_PKG_VERSION").unwrap_or("(unknown version)");
            write!(
                writer,
                "/* automatically generated by rust-bindgen {version} */{NL}{NL}",
            )?;
        }

        for line in &self.options.raw_lines {
            writer.write_all(line.as_bytes())?;
            writer.write_all(NL.as_bytes())?;
        }

        if !self.options.raw_lines.is_empty() {
            writer.write_all(NL.as_bytes())?;
        }

        match self.format_tokens(&self.module) {
            Ok(formatted_bindings) => {
                writer.write_all(formatted_bindings.as_bytes())?;
            }
            Err(err) => {
                eprintln!(
                    "Failed to run rustfmt: {err} (non-fatal, continuing)"
                );
                writer.write_all(self.module.to_string().as_bytes())?;
            }
        }
        Ok(())
    }

    /// Gets the rustfmt path to rustfmt the generated bindings.
    fn rustfmt_path(&self) -> io::Result<Cow<PathBuf>> {
        debug_assert!(matches!(self.options.formatter, Formatter::Rustfmt));
        if let Some(ref p) = self.options.rustfmt_path {
            return Ok(Cow::Borrowed(p));
        }
        if let Ok(rustfmt) = env::var("RUSTFMT") {
            return Ok(Cow::Owned(rustfmt.into()));
        }
        // No rustfmt binary was specified, so assume that the binary is called
        // "rustfmt" and that it is in the user's PATH.
        Ok(Cow::Owned("rustfmt".into()))
    }

    /// Formats a token stream with the formatter set up in `BindgenOptions`.
    fn format_tokens(
        &self,
        tokens: &proc_macro2::TokenStream,
    ) -> io::Result<String> {
        let _t = time::Timer::new("rustfmt_generated_string")
            .with_output(self.options.time_phases);

        match self.options.formatter {
            Formatter::None => return Ok(tokens.to_string()),
            #[cfg(feature = "prettyplease")]
            Formatter::Prettyplease => {
                return Ok(prettyplease::unparse(&syn::parse_quote!(#tokens)));
            }
            Formatter::Rustfmt => (),
        }

        let rustfmt = self.rustfmt_path()?;
        let mut cmd = Command::new(&*rustfmt);

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        if let Some(path) = self
            .options
            .rustfmt_configuration_file
            .as_ref()
            .and_then(|f| f.to_str())
        {
            cmd.args(["--config-path", path]);
        }

        let mut child = cmd.spawn()?;
        let mut child_stdin = child.stdin.take().unwrap();
        let mut child_stdout = child.stdout.take().unwrap();

        let source = tokens.to_string();

        // Write to stdin in a new thread, so that we can read from stdout on this
        // thread. This keeps the child from blocking on writing to its stdout which
        // might block us from writing to its stdin.
        let stdin_handle = ::std::thread::spawn(move || {
            let _ = child_stdin.write_all(source.as_bytes());
            source
        });

        let mut output = vec![];
        io::copy(&mut child_stdout, &mut output)?;

        let status = child.wait()?;
        let source = stdin_handle.join().expect(
            "The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
        );

        match String::from_utf8(output) {
            Ok(bindings) => match status.code() {
                Some(0) => Ok(bindings),
                Some(2) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Rustfmt parsing errors.".to_string(),
                )),
                Some(3) => {
                    rustfmt_non_fatal_error_diagnostic(
                        "Rustfmt could not format some lines",
                        &self.options,
                    );
                    Ok(bindings)
                }
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Internal rustfmt error".to_string(),
                )),
            },
            _ => Ok(source),
        }
    }
}

fn rustfmt_non_fatal_error_diagnostic(msg: &str, _options: &BindgenOptions) {
    warn!("{msg}");

    #[cfg(feature = "experimental")]
    if _options.emit_diagnostics {
        use crate::diagnostics::{Diagnostic, Level};

        Diagnostic::default()
            .with_title(msg, Level::Warning)
            .add_annotation(
                "The bindings will be generated but not formatted.",
                Level::Note,
            )
            .display();
    }
}

impl std::fmt::Display for Bindings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bytes = vec![];
        self.write(Box::new(&mut bytes) as Box<dyn Write>)
            .expect("writing to a vec cannot fail");
        f.write_str(
            std::str::from_utf8(&bytes)
                .expect("we should only write bindings that are valid utf-8"),
        )
    }
}

/// Determines whether the given cursor is in any of the files matched by the
/// options.
fn filter_builtins(ctx: &BindgenContext, cursor: &clang::Cursor) -> bool {
    ctx.options().builtins || !cursor.is_builtin()
}

/// Parse one `Item` from the Clang cursor.
fn parse_one(
    ctx: &mut BindgenContext,
    cursor: clang::Cursor,
    parent: Option<ItemId>,
) {
    if !filter_builtins(ctx, &cursor) {
        return;
    }

    match Item::parse(cursor, parent, ctx) {
        Ok(..) => {}
        Err(ParseError::Continue) => {}
        Err(ParseError::Recurse) => {
            cursor
                .visit_sorted(ctx, |ctx, child| parse_one(ctx, child, parent));
        }
    }
}

/// Parse the Clang AST into our `Item` internal representation.
fn parse(context: &mut BindgenContext) -> Result<(), BindgenError> {
    use clang_sys::*;

    let mut error = None;
    for d in &context.translation_unit().diags() {
        let msg = d.format();
        let is_err = d.severity() >= CXDiagnostic_Error;
        if is_err {
            let error = error.get_or_insert_with(String::new);
            error.push_str(&msg);
            error.push('\n');
        } else {
            eprintln!("clang diag: {msg}");
        }
    }

    if let Some(message) = error {
        return Err(BindgenError::ClangDiagnostic(message));
    }

    let cursor = context.translation_unit().cursor();

    if context.options().emit_ast {
        fn dump_if_not_builtin(cur: &clang::Cursor) -> CXChildVisitResult {
            if cur.is_builtin() {
                CXChildVisit_Continue
            } else {
                clang::ast_dump(cur, 0)
            }
        }
        cursor.visit(|cur| dump_if_not_builtin(&cur));
    }

    let root = context.root_module();
    context.with_module(root, |ctx| {
        cursor.visit_sorted(ctx, |ctx, child| parse_one(ctx, child, None));
    });

    assert!(
        context.current_module() == context.root_module(),
        "How did this happen?"
    );
    Ok(())
}

/// Extracted Clang version data
#[derive(Debug)]
pub struct ClangVersion {
    /// Major and minor semver, if parsing was successful
    pub parsed: Option<(u32, u32)>,
    /// full version string
    pub full: String,
}

/// Get the major and the minor semver numbers of Clang's version
pub fn clang_version() -> ClangVersion {
    ensure_libclang_is_loaded();

    //Debian clang version 11.0.1-2
    let raw_v: String = clang::extract_clang_version();
    let split_v: Option<Vec<&str>> = raw_v
        .split_whitespace()
        .find(|t| t.chars().next().is_some_and(|v| v.is_ascii_digit()))
        .map(|v| v.split('.').collect());
    if let Some(v) = split_v {
        if v.len() >= 2 {
            let maybe_major = v[0].parse::<u32>();
            let maybe_minor = v[1].parse::<u32>();
            if let (Ok(major), Ok(minor)) = (maybe_major, maybe_minor) {
                return ClangVersion {
                    parsed: Some((major, minor)),
                    full: raw_v.clone(),
                };
            }
        }
    };
    ClangVersion {
        parsed: None,
        full: raw_v.clone(),
    }
}

fn env_var<K: AsRef<str> + AsRef<OsStr>>(
    parse_callbacks: &[Rc<dyn callbacks::ParseCallbacks>],
    key: K,
) -> Result<String, env::VarError> {
    for callback in parse_callbacks {
        callback.read_env_var(key.as_ref());
    }
    env::var(key)
}

/// Looks for the env var `var_${TARGET}`, and falls back to just `var` when it is not found.
fn get_target_dependent_env_var(
    parse_callbacks: &[Rc<dyn callbacks::ParseCallbacks>],
    var: &str,
) -> Option<String> {
    if let Ok(target) = env_var(parse_callbacks, "TARGET") {
        if let Ok(v) = env_var(parse_callbacks, format!("{var}_{target}")) {
            return Some(v);
        }
        if let Ok(v) = env_var(
            parse_callbacks,
            format!("{var}_{}", target.replace('-', "_")),
        ) {
            return Some(v);
        }
    }

    env_var(parse_callbacks, var).ok()
}

/// A `ParseCallbacks` implementation that will act on file includes by echoing a rerun-if-changed
/// line and on env variable usage by echoing a rerun-if-env-changed line
///
/// When running inside a `build.rs` script, this can be used to make cargo invalidate the
/// generated bindings whenever any of the files included from the header change:
/// ```
/// use bindgen::builder;
/// let bindings = builder()
///     .header("path/to/input/header")
///     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
///     .generate();
/// ```
#[derive(Debug)]
pub struct CargoCallbacks {
    rerun_on_header_files: bool,
}

/// Create a new `CargoCallbacks` value with [`CargoCallbacks::rerun_on_header_files`] disabled.
///
/// This constructor has been deprecated in favor of [`CargoCallbacks::new`] where
/// [`CargoCallbacks::rerun_on_header_files`] is enabled by default.
#[deprecated = "Use `CargoCallbacks::new()` instead. Please, check the documentation for further information."]
pub const CargoCallbacks: CargoCallbacks = CargoCallbacks {
    rerun_on_header_files: false,
};

impl CargoCallbacks {
    /// Create a new `CargoCallbacks` value.
    pub fn new() -> Self {
        Self {
            rerun_on_header_files: true,
        }
    }

    /// Whether Cargo should re-run the build script if any of the input header files has changed.
    ///
    /// This option is enabled by default unless the deprecated [`const@CargoCallbacks`]
    /// constructor is used.
    pub fn rerun_on_header_files(mut self, doit: bool) -> Self {
        self.rerun_on_header_files = doit;
        self
    }
}

impl Default for CargoCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl callbacks::ParseCallbacks for CargoCallbacks {
    fn header_file(&self, filename: &str) {
        if self.rerun_on_header_files {
            println!("cargo:rerun-if-changed={filename}");
        }
    }

    fn include_file(&self, filename: &str) {
        println!("cargo:rerun-if-changed={filename}");
    }

    fn read_env_var(&self, key: &str) {
        println!("cargo:rerun-if-env-changed={key}");
    }
}

/// Test `command_line_flag` function.
#[test]
fn commandline_flag_unit_test_function() {
    //Test 1
    let bindings = builder();
    let command_line_flags = bindings.command_line_flags();

    let test_cases = [
        "--rust-target",
        "--no-derive-default",
        "--generate",
        "functions,types,vars,methods,constructors,destructors",
    ]
    .iter()
    .map(|&x| x.into())
    .collect::<Vec<String>>();

    assert!(test_cases.iter().all(|x| command_line_flags.contains(x)));

    //Test 2
    let bindings = builder()
        .header("input_header")
        .allowlist_type("Distinct_Type")
        .allowlist_function("safe_function");

    let command_line_flags = bindings.command_line_flags();
    let test_cases = [
        "--rust-target",
        "input_header",
        "--no-derive-default",
        "--generate",
        "functions,types,vars,methods,constructors,destructors",
        "--allowlist-type",
        "Distinct_Type",
        "--allowlist-function",
        "safe_function",
    ]
    .iter()
    .map(|&x| x.into())
    .collect::<Vec<String>>();
    println!("{command_line_flags:?}");

    assert!(test_cases.iter().all(|x| command_line_flags.contains(x)));
}

#[test]
fn test_rust_to_clang_target() {
    assert_eq!(
        rust_to_clang_target("aarch64-apple-ios").as_ref(),
        "arm64-apple-ios"
    );
}

#[test]
fn test_rust_to_clang_target_riscv() {
    assert_eq!(
        rust_to_clang_target("riscv64gc-unknown-linux-gnu").as_ref(),
        "riscv64-unknown-linux-gnu"
    );
    assert_eq!(
        rust_to_clang_target("riscv64imac-unknown-none-elf").as_ref(),
        "riscv64-unknown-none-elf"
    );
    assert_eq!(
        rust_to_clang_target("riscv32imc-unknown-none-elf").as_ref(),
        "riscv32-unknown-none-elf"
    );
    assert_eq!(
        rust_to_clang_target("riscv32imac-unknown-none-elf").as_ref(),
        "riscv32-unknown-none-elf"
    );
    assert_eq!(
        rust_to_clang_target("riscv32imafc-unknown-none-elf").as_ref(),
        "riscv32-unknown-none-elf"
    );
    assert_eq!(
        rust_to_clang_target("riscv32i-unknown-none-elf").as_ref(),
        "riscv32-unknown-none-elf"
    );
}

#[test]
fn test_rust_to_clang_target_espidf() {
    assert_eq!(
        rust_to_clang_target("riscv32imc-esp-espidf").as_ref(),
        "riscv32-esp-elf"
    );
    assert_eq!(
        rust_to_clang_target("xtensa-esp32-espidf").as_ref(),
        "xtensa-esp32-elf"
    );
}
