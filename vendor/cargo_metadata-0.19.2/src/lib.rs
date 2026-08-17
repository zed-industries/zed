#![deny(missing_docs)]
//! Structured access to the output of `cargo metadata` and `cargo --message-format=json`.
//! Usually used from within a `cargo-*` executable
//!
//! See the [cargo book](https://doc.rust-lang.org/cargo/index.html) for
//! details on cargo itself.
//!
//! ## Examples
//!
//! ```rust
//! # extern crate cargo_metadata;
//! # use std::path::Path;
//! let mut args = std::env::args().skip_while(|val| !val.starts_with("--manifest-path"));
//!
//! let mut cmd = cargo_metadata::MetadataCommand::new();
//! let manifest_path = match args.next() {
//!     Some(ref p) if p == "--manifest-path" => {
//!         cmd.manifest_path(args.next().unwrap());
//!     }
//!     Some(p) => {
//!         cmd.manifest_path(p.trim_start_matches("--manifest-path="));
//!     }
//!     None => {}
//! };
//!
//! let _metadata = cmd.exec().unwrap();
//! ```
//!
//! Pass features flags
//!
//! ```rust
//! # // This should be kept in sync with the equivalent example in the readme.
//! # extern crate cargo_metadata;
//! # use std::path::Path;
//! # fn main() {
//! use cargo_metadata::{MetadataCommand, CargoOpt};
//!
//! let _metadata = MetadataCommand::new()
//!     .manifest_path("./Cargo.toml")
//!     .features(CargoOpt::AllFeatures)
//!     .exec()
//!     .unwrap();
//! # }
//! ```
//!
//! Parse message-format output:
//!
//! ```
//! # extern crate cargo_metadata;
//! use std::process::{Stdio, Command};
//! use cargo_metadata::Message;
//!
//! let mut command = Command::new("cargo")
//!     .args(&["build", "--message-format=json-render-diagnostics"])
//!     .stdout(Stdio::piped())
//!     .spawn()
//!     .unwrap();
//!
//! let reader = std::io::BufReader::new(command.stdout.take().unwrap());
//! for message in cargo_metadata::Message::parse_stream(reader) {
//!     match message.unwrap() {
//!         Message::CompilerMessage(msg) => {
//!             println!("{:?}", msg);
//!         },
//!         Message::CompilerArtifact(artifact) => {
//!             println!("{:?}", artifact);
//!         },
//!         Message::BuildScriptExecuted(script) => {
//!             println!("{:?}", script);
//!         },
//!         Message::BuildFinished(finished) => {
//!             println!("{:?}", finished);
//!         },
//!         _ => () // Unknown message
//!     }
//! }
//!
//! let output = command.wait().expect("Couldn't get cargo's exit status");
//! ```

use camino::Utf8PathBuf;
#[cfg(feature = "builder")]
use derive_builder::Builder;
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::{from_utf8, FromStr};

pub use camino;
pub use semver;
use semver::Version;

#[cfg(feature = "builder")]
pub use dependency::DependencyBuilder;
pub use dependency::{Dependency, DependencyKind};
use diagnostic::Diagnostic;
pub use errors::{Error, Result};
#[cfg(feature = "unstable")]
pub use libtest::TestMessage;
#[allow(deprecated)]
pub use messages::parse_messages;
pub use messages::{
    Artifact, ArtifactDebuginfo, ArtifactProfile, BuildFinished, BuildScript, CompilerMessage,
    Message, MessageIter,
};
#[cfg(feature = "builder")]
pub use messages::{
    ArtifactBuilder, ArtifactProfileBuilder, BuildFinishedBuilder, BuildScriptBuilder,
    CompilerMessageBuilder,
};
use serde::{Deserialize, Deserializer, Serialize};

mod dependency;
pub mod diagnostic;
mod errors;
#[cfg(feature = "unstable")]
pub mod libtest;
mod messages;

/// An "opaque" identifier for a package.
///
/// It is possible to inspect the `repr` field, if the need arises, but its
/// precise format is an implementation detail and is subject to change.
///
/// `Metadata` can be indexed by `PackageId`.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct PackageId {
    /// The underlying string representation of id.
    pub repr: String,
}

impl fmt::Display for PackageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.repr, f)
    }
}

/// Helpers for default metadata fields
fn is_null(value: &serde_json::Value) -> bool {
    matches!(value, serde_json::Value::Null)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
/// Starting point for metadata returned by `cargo metadata`
pub struct Metadata {
    /// A list of all crates referenced by this crate (and the crate itself)
    pub packages: Vec<Package>,
    /// A list of all workspace members
    pub workspace_members: Vec<PackageId>,
    /// The list of default workspace members
    ///
    /// This is not available if running with a version of Cargo older than 1.71.
    ///
    /// You can check whether it is available or missing using respectively
    /// [`WorkspaceDefaultMembers::is_available`] and [`WorkspaceDefaultMembers::is_missing`].
    #[serde(default, skip_serializing_if = "WorkspaceDefaultMembers::is_missing")]
    pub workspace_default_members: WorkspaceDefaultMembers,
    /// Dependencies graph
    pub resolve: Option<Resolve>,
    /// Workspace root
    pub workspace_root: Utf8PathBuf,
    /// Build directory
    pub target_directory: Utf8PathBuf,
    /// The workspace-level metadata object. Null if non-existent.
    #[serde(rename = "metadata", default, skip_serializing_if = "is_null")]
    pub workspace_metadata: serde_json::Value,
    /// The metadata format version
    version: usize,
}

impl Metadata {
    /// Get the workspace's root package of this metadata instance.
    pub fn root_package(&self) -> Option<&Package> {
        match &self.resolve {
            Some(resolve) => {
                // if dependencies are resolved, use Cargo's answer
                let root = resolve.root.as_ref()?;
                self.packages.iter().find(|pkg| &pkg.id == root)
            }
            None => {
                // if dependencies aren't resolved, check for a root package manually
                let root_manifest_path = self.workspace_root.join("Cargo.toml");
                self.packages
                    .iter()
                    .find(|pkg| pkg.manifest_path == root_manifest_path)
            }
        }
    }

    /// Get the workspace packages.
    pub fn workspace_packages(&self) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|&p| self.workspace_members.contains(&p.id))
            .collect()
    }

    /// Get the workspace default packages.
    ///
    /// # Panics
    ///
    /// This will panic if running with a version of Cargo older than 1.71.
    pub fn workspace_default_packages(&self) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|&p| self.workspace_default_members.contains(&p.id))
            .collect()
    }
}

impl<'a> std::ops::Index<&'a PackageId> for Metadata {
    type Output = Package;

    fn index(&self, idx: &'a PackageId) -> &Self::Output {
        self.packages
            .iter()
            .find(|p| p.id == *idx)
            .unwrap_or_else(|| panic!("no package with this id: {:?}", idx))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
#[serde(transparent)]
/// A list of default workspace members.
///
/// See [`Metadata::workspace_default_members`].
///
/// It is only available if running a version of Cargo of 1.71 or newer.
///
/// # Panics
///
/// Dereferencing when running an older version of Cargo will panic.
pub struct WorkspaceDefaultMembers(Option<Vec<PackageId>>);

impl WorkspaceDefaultMembers {
    /// Return `true` if the list of workspace default members is supported by
    /// the called cargo-metadata version and `false` otherwise.
    ///
    /// In particular useful when parsing the output of `cargo-metadata` for
    /// versions of Cargo < 1.71, as dereferencing [`WorkspaceDefaultMembers`]
    /// for these versions will panic.
    ///
    /// Opposite of [`WorkspaceDefaultMembers::is_missing`].
    pub fn is_available(&self) -> bool {
        self.0.is_some()
    }

    /// Return `false` if the list of workspace default members is supported by
    /// the called cargo-metadata version and `true` otherwise.
    ///
    /// In particular useful when parsing the output of `cargo-metadata` for
    /// versions of Cargo < 1.71, as dereferencing [`WorkspaceDefaultMembers`]
    /// for these versions will panic.
    ///
    /// Opposite of [`WorkspaceDefaultMembers::is_available`].
    pub fn is_missing(&self) -> bool {
        self.0.is_none()
    }
}

impl core::ops::Deref for WorkspaceDefaultMembers {
    type Target = [PackageId];

    fn deref(&self) -> &Self::Target {
        self.0
            .as_ref()
            .expect("WorkspaceDefaultMembers should only be dereferenced on Cargo versions >= 1.71")
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
/// A dependency graph
pub struct Resolve {
    /// Nodes in a dependencies graph
    pub nodes: Vec<Node>,

    /// The crate for which the metadata was read.
    pub root: Option<PackageId>,
}

impl<'a> std::ops::Index<&'a PackageId> for Resolve {
    type Output = Node;

    fn index(&self, idx: &'a PackageId) -> &Self::Output {
        self.nodes
            .iter()
            .find(|p| p.id == *idx)
            .unwrap_or_else(|| panic!("no Node with this id: {:?}", idx))
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
/// A node in a dependencies graph
pub struct Node {
    /// An opaque identifier for a package
    pub id: PackageId,
    /// Dependencies in a structured format.
    ///
    /// `deps` handles renamed dependencies whereas `dependencies` does not.
    #[serde(default)]
    pub deps: Vec<NodeDep>,

    /// List of opaque identifiers for this node's dependencies.
    /// It doesn't support renamed dependencies. See `deps`.
    pub dependencies: Vec<PackageId>,

    /// Features enabled on the crate
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
/// A dependency in a node
pub struct NodeDep {
    /// The name of the dependency's library target.
    /// If the crate was renamed, it is the new name.
    pub name: String,
    /// Package ID (opaque unique identifier)
    pub pkg: PackageId,
    /// The kinds of dependencies.
    ///
    /// This field was added in Rust 1.41.
    #[serde(default)]
    pub dep_kinds: Vec<DepKindInfo>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
/// Information about a dependency kind.
pub struct DepKindInfo {
    /// The kind of dependency.
    #[serde(deserialize_with = "dependency::parse_dependency_kind")]
    pub kind: DependencyKind,
    /// The target platform for the dependency.
    ///
    /// This is `None` if it is not a target dependency.
    ///
    /// Use the [`Display`] trait to access the contents.
    ///
    /// By default all platform dependencies are included in the resolve
    /// graph. Use Cargo's `--filter-platform` flag if you only want to
    /// include dependencies for a specific platform.
    ///
    /// [`Display`]: std::fmt::Display
    pub target: Option<dependency::Platform>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
/// One or more crates described by a single `Cargo.toml`
///
/// Each [`target`][Package::targets] of a `Package` will be built as a crate.
/// For more information, see <https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html>.
pub struct Package {
    /// The [`name` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field) as given in the `Cargo.toml`
    // (We say "given in" instead of "specified in" since the `name` key cannot be inherited from the workspace.)
    pub name: String,
    /// The [`version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field) as specified in the `Cargo.toml`
    pub version: Version,
    /// The [`authors` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-authors-field) as specified in the `Cargo.toml`
    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub authors: Vec<String>,
    /// An opaque identifier for a package
    pub id: PackageId,
    /// The source of the package, e.g.
    /// crates.io or `None` for local projects.
    #[cfg_attr(feature = "builder", builder(default))]
    pub source: Option<Source>,
    /// The [`description` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-description-field) as specified in the `Cargo.toml`
    #[cfg_attr(feature = "builder", builder(default))]
    pub description: Option<String>,
    /// List of dependencies of this particular package
    #[cfg_attr(feature = "builder", builder(default))]
    pub dependencies: Vec<Dependency>,
    /// The [`license` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) as specified in the `Cargo.toml`
    #[cfg_attr(feature = "builder", builder(default))]
    pub license: Option<String>,
    /// The [`license-file` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) as specified in the `Cargo.toml`.
    /// If the package is using a nonstandard license, this key may be specified instead of
    /// `license`, and must point to a file relative to the manifest.
    #[cfg_attr(feature = "builder", builder(default))]
    pub license_file: Option<Utf8PathBuf>,
    /// Targets provided by the crate (lib, bin, example, test, ...)
    #[cfg_attr(feature = "builder", builder(default))]
    pub targets: Vec<Target>,
    /// Features provided by the crate, mapped to the features required by that feature.
    #[cfg_attr(feature = "builder", builder(default))]
    pub features: BTreeMap<String, Vec<String>>,
    /// Path containing the `Cargo.toml`
    pub manifest_path: Utf8PathBuf,
    /// The [`categories` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-categories-field) as specified in the `Cargo.toml`
    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub categories: Vec<String>,
    /// The [`keywords` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field) as specified in the `Cargo.toml`
    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub keywords: Vec<String>,
    /// The [`readme` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field) as specified in the `Cargo.toml`
    #[cfg_attr(feature = "builder", builder(default))]
    pub readme: Option<Utf8PathBuf>,
    /// The [`repository` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field) as specified in the `Cargo.toml`
    // can't use `url::Url` because that requires a more recent stable compiler
    #[cfg_attr(feature = "builder", builder(default))]
    pub repository: Option<String>,
    /// The [`homepage` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-homepage-field) as specified in the `Cargo.toml`.
    ///
    /// On versions of cargo before 1.49, this will always be [`None`].
    #[cfg_attr(feature = "builder", builder(default))]
    pub homepage: Option<String>,
    /// The [`documentation` URL](https://doc.rust-lang.org/cargo/reference/manifest.html#the-documentation-field) as specified in the `Cargo.toml`.
    ///
    /// On versions of cargo before 1.49, this will always be [`None`].
    #[cfg_attr(feature = "builder", builder(default))]
    pub documentation: Option<String>,
    /// The default Rust edition for the package (either what's specified in the [`edition` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field)
    /// or defaulting to [`Edition::E2015`]).
    ///
    /// Beware that individual targets may specify their own edition in
    /// [`Target::edition`].
    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub edition: Edition,
    /// Contents of the free form [`package.metadata` section](https://doc.rust-lang.org/cargo/reference/manifest.html#the-metadata-table).
    ///
    /// This contents can be serialized to a struct using serde:
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use serde_json::json;
    ///
    /// #[derive(Debug, Deserialize)]
    /// struct SomePackageMetadata {
    ///     some_value: i32,
    /// }
    ///
    /// let value = json!({
    ///     "some_value": 42,
    /// });
    ///
    /// let package_metadata: SomePackageMetadata = serde_json::from_value(value).unwrap();
    /// assert_eq!(package_metadata.some_value, 42);
    ///
    /// ```
    #[serde(default, skip_serializing_if = "is_null")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub metadata: serde_json::Value,
    /// The name of a native library the package is linking to.
    #[cfg_attr(feature = "builder", builder(default))]
    pub links: Option<String>,
    /// List of registries to which this package may be published (derived from the [`publish` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)).
    ///
    /// Publishing is unrestricted if `None`, and forbidden if the `Vec` is empty.
    ///
    /// This is always `None` if running with a version of Cargo older than 1.39.
    #[cfg_attr(feature = "builder", builder(default))]
    pub publish: Option<Vec<String>>,
    /// The [`default-run` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-default-run-field) as given in the `Cargo.toml`
    // (We say "given in" instead of "specified in" since the `default-run` key cannot be inherited from the workspace.)
    /// The default binary to run by `cargo run`.
    ///
    /// This is always `None` if running with a version of Cargo older than 1.55.
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_run: Option<String>,
    /// The [`rust-version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field) as specified in the `Cargo.toml`.
    /// The minimum supported Rust version of this package.
    ///
    /// This is always `None` if running with a version of Cargo older than 1.58.
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_rust_version")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rust_version: Option<Version>,
}

#[cfg(feature = "builder")]
impl PackageBuilder {
    /// Construct a new `PackageBuilder` with all required fields.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<Version>,
        id: impl Into<PackageId>,
        path: impl Into<Utf8PathBuf>,
    ) -> Self {
        Self::default()
            .name(name)
            .version(version)
            .id(id)
            .manifest_path(path)
    }
}

impl Package {
    /// Full path to the license file if one is present in the manifest
    pub fn license_file(&self) -> Option<Utf8PathBuf> {
        self.license_file.as_ref().map(|file| {
            self.manifest_path
                .parent()
                .unwrap_or(&self.manifest_path)
                .join(file)
        })
    }

    /// Full path to the readme file if one is present in the manifest
    pub fn readme(&self) -> Option<Utf8PathBuf> {
        self.readme.as_ref().map(|file| {
            self.manifest_path
                .parent()
                .unwrap_or(&self.manifest_path)
                .join(file)
        })
    }
}

/// The source of a package such as crates.io.
///
/// It is possible to inspect the `repr` field, if the need arises, but its
/// precise format is an implementation detail and is subject to change.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Source {
    /// The underlying string representation of a source.
    pub repr: String,
}

impl Source {
    /// Returns true if the source is crates.io.
    pub fn is_crates_io(&self) -> bool {
        self.repr == "registry+https://github.com/rust-lang/crates.io-index"
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.repr, f)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
#[non_exhaustive]
/// A single target (lib, bin, example, ...) provided by a crate
pub struct Target {
    /// Name as given in the `Cargo.toml` or generated from the file name
    pub name: String,
    /// Kind of target.
    ///
    /// The possible values are `example`, `test`, `bench`, `custom-build` and
    /// [Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
    /// `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
    ///
    /// Other possible values may be added in the future.
    pub kind: Vec<TargetKind>,
    /// Similar to `kind`, but only reports the
    /// [Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
    /// `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
    /// Everything that's not a proc macro or a library of some kind is reported as "bin".
    ///
    /// Other possible values may be added in the future.
    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub crate_types: Vec<CrateType>,

    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(rename = "required-features")]
    /// This target is built only if these features are enabled.
    /// It doesn't apply to `lib` targets.
    pub required_features: Vec<String>,
    /// Path to the main source file of the target
    pub src_path: Utf8PathBuf,
    /// Rust edition for this target
    #[serde(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub edition: Edition,
    /// Whether or not this target has doc tests enabled, and the target is
    /// compatible with doc testing.
    ///
    /// This is always `true` if running with a version of Cargo older than 1.37.
    #[serde(default = "default_true")]
    #[cfg_attr(feature = "builder", builder(default = "true"))]
    pub doctest: bool,
    /// Whether or not this target is tested by default by `cargo test`.
    ///
    /// This is always `true` if running with a version of Cargo older than 1.47.
    #[serde(default = "default_true")]
    #[cfg_attr(feature = "builder", builder(default = "true"))]
    pub test: bool,
    /// Whether or not this target is documented by `cargo doc`.
    ///
    /// This is always `true` if running with a version of Cargo older than 1.50.
    #[serde(default = "default_true")]
    #[cfg_attr(feature = "builder", builder(default = "true"))]
    pub doc: bool,
}

macro_rules! methods_target_is_kind {
    ($($name:ident => $kind:expr),*) => {
        $(
            /// Return true if this target is of kind `$kind`.
            pub fn $name(&self) -> bool {
                self.is_kind($kind)
            }
        )*
    }
}

impl Target {
    /// Return true if this target is of the given kind.
    pub fn is_kind(&self, name: TargetKind) -> bool {
        self.kind.iter().any(|kind| kind == &name)
    }

    // Generate `is_*` methods for each `TargetKind`
    methods_target_is_kind! {
        is_lib => TargetKind::Lib,
        is_bin => TargetKind::Bin,
        is_example => TargetKind::Example,
        is_test => TargetKind::Test,
        is_bench => TargetKind::Bench,
        is_custom_build => TargetKind::CustomBuild,
        is_proc_macro => TargetKind::ProcMacro,
        is_cdylib => TargetKind::CDyLib,
        is_dylib => TargetKind::DyLib,
        is_rlib => TargetKind::RLib,
        is_staticlib => TargetKind::StaticLib
    }
}

/// Kind of target.
///
/// The possible values are `example`, `test`, `bench`, `custom-build` and
/// [Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
/// `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
///
/// Other possible values may be added in the future.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TargetKind {
    /// `cargo bench` target
    #[serde(rename = "bench")]
    Bench,
    /// Binary executable target
    #[serde(rename = "bin")]
    Bin,
    /// Custom build target
    #[serde(rename = "custom-build")]
    CustomBuild,
    /// Dynamic system library target
    #[serde(rename = "cdylib")]
    CDyLib,
    /// Dynamic Rust library target
    #[serde(rename = "dylib")]
    DyLib,
    /// Example target
    #[serde(rename = "example")]
    Example,
    /// Rust library
    #[serde(rename = "lib")]
    Lib,
    /// Procedural Macro
    #[serde(rename = "proc-macro")]
    ProcMacro,
    /// Rust library for use as an intermediate artifact
    #[serde(rename = "rlib")]
    RLib,
    /// Static system library
    #[serde(rename = "staticlib")]
    StaticLib,
    /// Test target
    #[serde(rename = "test")]
    Test,
    /// Unknown type
    #[serde(untagged)]
    Unknown(String),
}

impl From<&str> for TargetKind {
    fn from(value: &str) -> Self {
        match value {
            "example" => TargetKind::Example,
            "test" => TargetKind::Test,
            "bench" => TargetKind::Bench,
            "custom-build" => TargetKind::CustomBuild,
            "bin" => TargetKind::Bin,
            "lib" => TargetKind::Lib,
            "rlib" => TargetKind::RLib,
            "dylib" => TargetKind::DyLib,
            "cdylib" => TargetKind::CDyLib,
            "staticlib" => TargetKind::StaticLib,
            "proc-macro" => TargetKind::ProcMacro,
            x => TargetKind::Unknown(x.to_string()),
        }
    }
}

impl FromStr for TargetKind {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TargetKind::from(s))
    }
}

impl fmt::Display for TargetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bench => "bench".fmt(f),
            Self::Bin => "bin".fmt(f),
            Self::CustomBuild => "custom-build".fmt(f),
            Self::CDyLib => "cdylib".fmt(f),
            Self::DyLib => "dylib".fmt(f),
            Self::Example => "example".fmt(f),
            Self::Lib => "lib".fmt(f),
            Self::ProcMacro => "proc-macro".fmt(f),
            Self::RLib => "rlib".fmt(f),
            Self::StaticLib => "staticlib".fmt(f),
            Self::Test => "test".fmt(f),
            Self::Unknown(x) => x.fmt(f),
        }
    }
}

/// Similar to `kind`, but only reports the
/// [Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
///
/// `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
/// Everything that's not a proc macro or a library of some kind is reported as "bin".
///
/// Other possible values may be added in the future.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum CrateType {
    /// Binary executable target
    #[serde(rename = "bin")]
    Bin,
    /// Dynamic system library target
    #[serde(rename = "cdylib")]
    CDyLib,
    /// Dynamic Rust library target
    #[serde(rename = "dylib")]
    DyLib,
    /// Rust library
    #[serde(rename = "lib")]
    Lib,
    /// Procedural Macro
    #[serde(rename = "proc-macro")]
    ProcMacro,
    /// Rust library for use as an intermediate artifact
    #[serde(rename = "rlib")]
    RLib,
    /// Static system library
    #[serde(rename = "staticlib")]
    StaticLib,
    /// Unkown type
    #[serde(untagged)]
    Unknown(String),
}

impl From<&str> for CrateType {
    fn from(value: &str) -> Self {
        match value {
            "bin" => CrateType::Bin,
            "lib" => CrateType::Lib,
            "rlib" => CrateType::RLib,
            "dylib" => CrateType::DyLib,
            "cdylib" => CrateType::CDyLib,
            "staticlib" => CrateType::StaticLib,
            "proc-macro" => CrateType::ProcMacro,
            x => CrateType::Unknown(x.to_string()),
        }
    }
}

impl FromStr for CrateType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CrateType::from(s))
    }
}

impl fmt::Display for CrateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bin => "bin".fmt(f),
            Self::CDyLib => "cdylib".fmt(f),
            Self::DyLib => "dylib".fmt(f),
            Self::Lib => "lib".fmt(f),
            Self::ProcMacro => "proc-macro".fmt(f),
            Self::RLib => "rlib".fmt(f),
            Self::StaticLib => "staticlib".fmt(f),
            Self::Unknown(x) => x.fmt(f),
        }
    }
}

/// The Rust edition
///
/// As of writing this comment rust editions 2024, 2027 and 2030 are not actually a thing yet but are parsed nonetheless for future proofing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Edition {
    /// Edition 2015
    #[serde(rename = "2015")]
    E2015,
    /// Edition 2018
    #[serde(rename = "2018")]
    E2018,
    /// Edition 2021
    #[serde(rename = "2021")]
    E2021,
    /// Edition 2024
    #[serde(rename = "2024")]
    E2024,
    #[doc(hidden)]
    #[serde(rename = "2027")]
    _E2027,
    #[doc(hidden)]
    #[serde(rename = "2030")]
    _E2030,
}

impl Edition {
    /// Return the string representation of the edition
    pub fn as_str(&self) -> &'static str {
        use Edition::*;
        match self {
            E2015 => "2015",
            E2018 => "2018",
            E2021 => "2021",
            E2024 => "2024",
            _E2027 => "2027",
            _E2030 => "2030",
        }
    }
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for Edition {
    fn default() -> Self {
        Self::E2015
    }
}

fn default_true() -> bool {
    true
}

/// Cargo features flags
#[derive(Debug, Clone)]
pub enum CargoOpt {
    /// Run cargo with `--features-all`
    AllFeatures,
    /// Run cargo with `--no-default-features`
    NoDefaultFeatures,
    /// Run cargo with `--features <FEATURES>`
    SomeFeatures(Vec<String>),
}

/// A builder for configuring `cargo metadata` invocation.
#[derive(Debug, Clone, Default)]
pub struct MetadataCommand {
    /// Path to `cargo` executable.  If not set, this will use the
    /// the `$CARGO` environment variable, and if that is not set, will
    /// simply be `cargo`.
    cargo_path: Option<PathBuf>,
    /// Path to `Cargo.toml`
    manifest_path: Option<PathBuf>,
    /// Current directory of the `cargo metadata` process.
    current_dir: Option<PathBuf>,
    /// Output information only about workspace members and don't fetch dependencies.
    no_deps: bool,
    /// Collections of `CargoOpt::SomeFeatures(..)`
    features: Vec<String>,
    /// Latched `CargoOpt::AllFeatures`
    all_features: bool,
    /// Latched `CargoOpt::NoDefaultFeatures`
    no_default_features: bool,
    /// Arbitrary command line flags to pass to `cargo`.  These will be added
    /// to the end of the command line invocation.
    other_options: Vec<String>,
    /// Arbitrary environment variables to set when running `cargo`.  These will be merged into
    /// the calling environment, overriding any which clash.
    env: BTreeMap<OsString, OsString>,
    /// Show stderr
    verbose: bool,
}

impl MetadataCommand {
    /// Creates a default `cargo metadata` command, which will look for
    /// `Cargo.toml` in the ancestors of the current directory.
    pub fn new() -> MetadataCommand {
        MetadataCommand::default()
    }
    /// Path to `cargo` executable.  If not set, this will use the
    /// the `$CARGO` environment variable, and if that is not set, will
    /// simply be `cargo`.
    pub fn cargo_path(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand {
        self.cargo_path = Some(path.into());
        self
    }
    /// Path to `Cargo.toml`
    pub fn manifest_path(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand {
        self.manifest_path = Some(path.into());
        self
    }
    /// Current directory of the `cargo metadata` process.
    pub fn current_dir(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand {
        self.current_dir = Some(path.into());
        self
    }
    /// Output information only about workspace members and don't fetch dependencies.
    pub fn no_deps(&mut self) -> &mut MetadataCommand {
        self.no_deps = true;
        self
    }
    /// Which features to include.
    ///
    /// Call this multiple times to specify advanced feature configurations:
    ///
    /// ```no_run
    /// # use cargo_metadata::{CargoOpt, MetadataCommand};
    /// MetadataCommand::new()
    ///     .features(CargoOpt::NoDefaultFeatures)
    ///     .features(CargoOpt::SomeFeatures(vec!["feat1".into(), "feat2".into()]))
    ///     .features(CargoOpt::SomeFeatures(vec!["feat3".into()]))
    ///     // ...
    ///     # ;
    /// ```
    ///
    /// # Panics
    ///
    /// `cargo metadata` rejects multiple `--no-default-features` flags. Similarly, the `features()`
    /// method panics when specifying multiple `CargoOpt::NoDefaultFeatures`:
    ///
    /// ```should_panic
    /// # use cargo_metadata::{CargoOpt, MetadataCommand};
    /// MetadataCommand::new()
    ///     .features(CargoOpt::NoDefaultFeatures)
    ///     .features(CargoOpt::NoDefaultFeatures) // <-- panic!
    ///     // ...
    ///     # ;
    /// ```
    ///
    /// The method also panics for multiple `CargoOpt::AllFeatures` arguments:
    ///
    /// ```should_panic
    /// # use cargo_metadata::{CargoOpt, MetadataCommand};
    /// MetadataCommand::new()
    ///     .features(CargoOpt::AllFeatures)
    ///     .features(CargoOpt::AllFeatures) // <-- panic!
    ///     // ...
    ///     # ;
    /// ```
    pub fn features(&mut self, features: CargoOpt) -> &mut MetadataCommand {
        match features {
            CargoOpt::SomeFeatures(features) => self.features.extend(features),
            CargoOpt::NoDefaultFeatures => {
                assert!(
                    !self.no_default_features,
                    "Do not supply CargoOpt::NoDefaultFeatures more than once!"
                );
                self.no_default_features = true;
            }
            CargoOpt::AllFeatures => {
                assert!(
                    !self.all_features,
                    "Do not supply CargoOpt::AllFeatures more than once!"
                );
                self.all_features = true;
            }
        }
        self
    }
    /// Arbitrary command line flags to pass to `cargo`.  These will be added
    /// to the end of the command line invocation.
    pub fn other_options(&mut self, options: impl Into<Vec<String>>) -> &mut MetadataCommand {
        self.other_options = options.into();
        self
    }

    /// Arbitrary environment variables to set when running `cargo`.  These will be merged into
    /// the calling environment, overriding any which clash.
    ///
    /// Some examples of when you may want to use this:
    /// 1. Setting cargo config values without needing a .cargo/config.toml file, e.g. to set
    ///    `CARGO_NET_GIT_FETCH_WITH_CLI=true`
    /// 2. To specify a custom path to RUSTC if your rust toolchain components aren't laid out in
    ///    the way cargo expects by default.
    ///
    /// ```no_run
    /// # use cargo_metadata::{CargoOpt, MetadataCommand};
    /// MetadataCommand::new()
    ///     .env("CARGO_NET_GIT_FETCH_WITH_CLI", "true")
    ///     .env("RUSTC", "/path/to/rustc")
    ///     // ...
    ///     # ;
    /// ```
    pub fn env<K: Into<OsString>, V: Into<OsString>>(
        &mut self,
        key: K,
        val: V,
    ) -> &mut MetadataCommand {
        self.env.insert(key.into(), val.into());
        self
    }

    /// Set whether to show stderr
    pub fn verbose(&mut self, verbose: bool) -> &mut MetadataCommand {
        self.verbose = verbose;
        self
    }

    /// Builds a command for `cargo metadata`.  This is the first
    /// part of the work of `exec`.
    pub fn cargo_command(&self) -> Command {
        let cargo = self
            .cargo_path
            .clone()
            .or_else(|| env::var("CARGO").map(PathBuf::from).ok())
            .unwrap_or_else(|| PathBuf::from("cargo"));
        let mut cmd = Command::new(cargo);
        cmd.args(["metadata", "--format-version", "1"]);

        if self.no_deps {
            cmd.arg("--no-deps");
        }

        if let Some(path) = self.current_dir.as_ref() {
            cmd.current_dir(path);
        }

        if !self.features.is_empty() {
            cmd.arg("--features").arg(self.features.join(","));
        }
        if self.all_features {
            cmd.arg("--all-features");
        }
        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        if let Some(manifest_path) = &self.manifest_path {
            cmd.arg("--manifest-path").arg(manifest_path.as_os_str());
        }
        cmd.args(&self.other_options);

        cmd.envs(&self.env);

        cmd
    }

    /// Parses `cargo metadata` output.  `data` must have been
    /// produced by a command built with `cargo_command`.
    pub fn parse<T: AsRef<str>>(data: T) -> Result<Metadata> {
        let meta = serde_json::from_str(data.as_ref())?;
        Ok(meta)
    }

    /// Runs configured `cargo metadata` and returns parsed `Metadata`.
    pub fn exec(&self) -> Result<Metadata> {
        let mut command = self.cargo_command();
        if self.verbose {
            command.stderr(Stdio::inherit());
        }
        let output = command.output()?;
        if !output.status.success() {
            return Err(Error::CargoMetadata {
                stderr: String::from_utf8(output.stderr)?,
            });
        }
        let stdout = from_utf8(&output.stdout)?
            .lines()
            .find(|line| line.starts_with('{'))
            .ok_or(Error::NoJson)?;
        Self::parse(stdout)
    }
}

/// As per the Cargo Book the [`rust-version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field) must:
///
/// > be a bare version number with two or three components;
/// > it cannot include semver operators or pre-release identifiers.
///
/// [`semver::Version`] however requires three components. This function takes
/// care of appending `.0` if the provided version number only has two components
/// and ensuring that it does not contain a pre-release version or build metadata.
fn deserialize_rust_version<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Version>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut buf = match Option::<String>::deserialize(deserializer)? {
        None => return Ok(None),
        Some(buf) => buf,
    };

    for char in buf.chars() {
        if char == '-' {
            return Err(serde::de::Error::custom(
                "pre-release identifiers are not supported in rust-version",
            ));
        } else if char == '+' {
            return Err(serde::de::Error::custom(
                "build metadata is not supported in rust-version",
            ));
        }
    }

    if buf.matches('.').count() == 1 {
        // e.g. 1.0 -> 1.0.0
        buf.push_str(".0");
    }

    Ok(Some(
        Version::parse(&buf).map_err(serde::de::Error::custom)?,
    ))
}

#[cfg(test)]
mod test {
    use semver::Version;

    #[derive(Debug, serde::Deserialize)]
    struct BareVersion(
        #[serde(deserialize_with = "super::deserialize_rust_version")] Option<semver::Version>,
    );

    fn bare_version(str: &str) -> Version {
        serde_json::from_str::<BareVersion>(&format!(r#""{}""#, str))
            .unwrap()
            .0
            .unwrap()
    }

    fn bare_version_err(str: &str) -> String {
        serde_json::from_str::<BareVersion>(&format!(r#""{}""#, str))
            .unwrap_err()
            .to_string()
    }

    #[test]
    fn test_deserialize_rust_version() {
        assert_eq!(bare_version("1.2"), Version::new(1, 2, 0));
        assert_eq!(bare_version("1.2.0"), Version::new(1, 2, 0));
        assert_eq!(
            bare_version_err("1.2.0-alpha"),
            "pre-release identifiers are not supported in rust-version"
        );
        assert_eq!(
            bare_version_err("1.2.0+123"),
            "build metadata is not supported in rust-version"
        );
    }
}
