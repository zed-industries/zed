#![forbid(unsafe_code)]
#![allow(clippy::redundant_field_names)]
#![doc = include_str!("../README.md")]

mod validation;

use validation::RawVersionInfo;

use serde::{Deserialize, Serialize};

use std::str::FromStr;

/// Dependency tree embedded in the binary.
///
/// Implements `Serialize` and `Deserialize` traits from `serde`, so you can use
/// [all the usual methods from serde-json](https://docs.rs/serde_json/1.0.57/serde_json/#functions)
/// to read and write it.
///
/// `from_str()` that parses JSON is also implemented for your convenience:
/// ```rust
/// use auditable_serde::VersionInfo;
/// use std::str::FromStr;
/// let json_str = r#"{"packages":[{
///     "name":"adler",
///     "version":"0.2.3",
///     "source":"registry"
/// }]}"#;
/// let info = VersionInfo::from_str(json_str).unwrap();
/// assert_eq!(&info.packages[0].name, "adler");
/// ```
///
/// If deserialization succeeds, it is guaranteed that there is only one root package,
/// and that are no cyclic dependencies.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(try_from = "RawVersionInfo")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VersionInfo {
    pub packages: Vec<Package>,
}

/// A single package in the dependency tree
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Package {
    /// Crate name specified in the `name` field in Cargo.toml file. Examples: "libc", "rand"
    pub name: String,
    /// The package's version in the [semantic version](https://semver.org) format.
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub version: semver::Version,
    /// Currently "git", "local", "crates.io" or "registry". Designed to be extensible with other revision control systems, etc.
    pub source: Source,
    /// "build" or "runtime". May be omitted if set to "runtime".
    /// If it's both a build and a runtime dependency, "runtime" is recorded.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub kind: DependencyKind,
    /// Packages are stored in an ordered array both in the `VersionInfo` struct and in JSON.
    /// Here we refer to each package by its index in the array.
    /// May be omitted if the list is empty.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub dependencies: Vec<usize>,
    /// Whether this is the root package in the dependency tree.
    /// There should only be one root package.
    /// May be omitted if set to `false`.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub root: bool,
}

/// Serializes to "git", "local", "crates.io" or "registry". Designed to be extensible with other revision control systems, etc.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(from = "&str")]
#[serde(into = "String")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Source {
    CratesIo,
    Git,
    Local,
    Registry,
    Other(String),
}

impl From<&str> for Source {
    fn from(s: &str) -> Self {
        match s {
            "crates.io" => Self::CratesIo,
            "git" => Self::Git,
            "local" => Self::Local,
            "registry" => Self::Registry,
            other_str => Self::Other(other_str.to_string()),
        }
    }
}

impl From<Source> for String {
    fn from(s: Source) -> String {
        match s {
            Source::CratesIo => "crates.io".to_owned(),
            Source::Git => "git".to_owned(),
            Source::Local => "local".to_owned(),
            Source::Registry => "registry".to_owned(),
            Source::Other(string) => string,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum DependencyKind {
    // The values are ordered from weakest to strongest so that casting to integer would make sense
    #[serde(rename = "build")]
    Build,
    #[default]
    #[serde(rename = "runtime")]
    Runtime,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    let default_value = T::default();
    value == &default_value
}

impl FromStr for VersionInfo {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)] // otherwise conditional compilation emits warnings
    use super::*;
    use std::fs;
    use std::{
        convert::TryInto,
        path::{Path, PathBuf},
    };

    #[cfg(feature = "schema")]
    /// Generate a JsonSchema for VersionInfo
    fn generate_schema() -> schemars::schema::RootSchema {
        let mut schema = schemars::schema_for!(VersionInfo);
        let mut metadata = *schema.schema.metadata.clone().unwrap();

        let title = "cargo-auditable schema".to_string();
        metadata.title = Some(title);
        metadata.id = Some("https://rustsec.org/schemas/cargo-auditable.json".to_string());
        metadata.examples = [].to_vec();
        metadata.description = Some(
            "Describes the `VersionInfo` JSON data structure that cargo-auditable embeds into Rust binaries."
                .to_string(),
        );
        schema.schema.metadata = Some(Box::new(metadata));
        schema
    }

    #[test]
    #[cfg(feature = "schema")]
    fn verify_schema() {
        use schemars::schema::RootSchema;

        let expected = generate_schema();
        // Printing here makes it easier to update the schema when required
        println!(
            "expected schema:\n{}",
            serde_json::to_string_pretty(&expected).unwrap()
        );

        let contents = fs::read_to_string(
            // `CARGO_MANIFEST_DIR` env is path to dir containing auditable-serde's Cargo.toml
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("cargo-auditable.schema.json"),
        )
        .expect("error reading existing schema");
        let actual: RootSchema =
            serde_json::from_str(&contents).expect("error deserializing existing schema");

        assert_eq!(expected, actual);
    }
}
