#![deny(missing_docs)]
#![allow(dead_code)]
//! Structured access to the output of `cargo metadata`
//! Usually used from within a `cargo-*` executable

// Forked from `https://github.com/oli-obk/cargo_metadata`
// Modifications:
//   1. Remove `resolve` from Metadata because it was causing parse failures
//   2. Fix the `manifest-path` argument
//   3. Add `--all-features` argument
//   4. Remove the `--no-deps` argument

use std::borrow::{Borrow, Cow};
use std::collections::{HashMap, HashSet};
use std::env;
use std::error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::Path;
use std::process::{Command, Output};
use std::str::Utf8Error;

#[derive(Clone, Deserialize, Debug)]
/// Starting point for metadata returned by `cargo metadata`
pub struct Metadata {
    /// A list of all crates referenced by this crate (and the crate itself)
    pub packages: HashSet<Package>,
    version: usize,
    /// path to the workspace containing the `Cargo.lock`
    pub workspace_root: String,
}

/// A reference to a package including it's name and the specific version.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackageRef {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
/// A crate
pub struct Package {
    #[serde(flatten)]
    pub name_and_version: PackageRef,
    id: String,
    source: Option<String>,
    /// List of dependencies of this particular package
    pub dependencies: HashSet<Dependency>,
    /// Targets provided by the crate (lib, bin, example, test, ...)
    pub targets: Vec<Target>,
    features: HashMap<String, Vec<String>>,
    /// path containing the `Cargo.toml`
    pub manifest_path: String,
}

#[derive(Clone, Deserialize, Debug)]
/// A dependency of the main crate
pub struct Dependency {
    /// Name as given in the `Cargo.toml`
    pub name: String,
    source: Option<String>,
    /// Whether this is required or optional
    pub req: String,
    kind: Option<String>,
    optional: bool,
    uses_default_features: bool,
    features: Vec<String>,
    pub target: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
/// A single target (lib, bin, example, ...) provided by a crate
pub struct Target {
    /// Name as given in the `Cargo.toml` or generated from the file name
    pub name: String,
    /// Kind of target ("bin", "example", "test", "bench", "lib")
    pub kind: Vec<String>,
    /// Almost the same as `kind`, except when an example is a library instad of an executable.
    /// In that case `crate_types` contains things like `rlib` and `dylib` while `kind` is `example`
    #[serde(default)]
    pub crate_types: Vec<String>,
    /// Path to the main source file of the target
    pub src_path: String,
}

#[derive(Debug)]
/// Possible errors that can occur during metadata parsing.
pub enum Error {
    /// Error during execution of `cargo metadata`
    Io(io::Error),
    /// Metadata extraction failure
    Metadata(Output),
    /// Output of `cargo metadata` was not valid utf8
    Utf8(Utf8Error),
    /// Deserialization error (structure of json did not match expected structure)
    Json(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ref err) => err.fmt(f),
            Error::Metadata(_) => write!(f, "Metadata error"),
            Error::Utf8(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::Metadata(_) => None,
            Error::Utf8(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
        }
    }
}

// Implementations that let us lookup Packages and Dependencies by name (string)

impl Borrow<PackageRef> for Package {
    fn borrow(&self) -> &PackageRef {
        &self.name_and_version
    }
}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name_and_version.hash(state);
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name_and_version == other.name_and_version
    }
}

impl Eq for Package {}

impl Borrow<str> for Dependency {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl Hash for Dependency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Dependency {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Dependency {}

fn discover_target(manifest_path: &Path) -> Option<String> {
    if let Ok(target) = std::env::var("TARGET") {
        return Some(target);
    }

    // We must be running as a standalone script, not under cargo.
    // Let's use the host platform instead.
    // We figure out the host platform through rustc and use that.
    // We unfortunatelly cannot go through cargo, since cargo rustc _also_ builds.
    // If `rustc` fails to run, we just fall back to not passing --filter-platforms.
    //
    // NOTE: We set the current directory in case of rustup shenanigans.
    let rustc = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));
    debug!("Discovering host platform by {:?}", rustc);

    let rustc_output = Command::new(rustc)
        .current_dir(manifest_path.parent().unwrap())
        .arg("-vV")
        .output();
    let rustc_output = match rustc_output {
        Ok(ref out) => String::from_utf8_lossy(&out.stdout),
        Err(..) => return None,
    };

    let field = "host: ";
    rustc_output
        .lines()
        .find_map(|l| l.strip_prefix(field).map(|stripped| stripped.to_string()))
}

/// The main entry point to obtaining metadata
pub fn metadata(
    manifest_path: &Path,
    existing_metadata_file: Option<&Path>,
    only_target: bool,
) -> Result<Metadata, Error> {
    let output;
    let metadata = match existing_metadata_file {
        Some(path) => Cow::Owned(std::fs::read_to_string(path)?),
        None => {
            let target = if only_target {
                let target = discover_target(manifest_path);
                if target.is_none() {
                    warn!(
                        "Failed to discover host platform for cargo metadata; \
                        will fetch dependencies for all platforms."
                    );
                }
                target
            } else {
                None
            };

            let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
            let mut cmd = Command::new(cargo);
            cmd.arg("metadata");
            cmd.arg("--all-features");
            cmd.arg("--format-version").arg("1");
            if let Some(target) = target {
                cmd.arg("--filter-platform").arg(target);
            }
            cmd.arg("--manifest-path");
            cmd.arg(manifest_path);
            output = cmd.output()?;
            if !output.status.success() {
                return Err(Error::Metadata(output));
            }
            Cow::Borrowed(std::str::from_utf8(&output.stdout)?)
        }
    };

    let meta: Metadata = serde_json::from_str(&metadata)?;
    Ok(meta)
}
