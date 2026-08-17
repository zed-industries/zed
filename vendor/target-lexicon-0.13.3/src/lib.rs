//! LLVM target triple types.

#![deny(missing_docs, trivial_numeric_casts, unused_extern_crates)]
#![warn(unused_import_braces)]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::float_arithmetic,
        clippy::mut_mut,
        clippy::nonminimal_bool,
        clippy::option_map_unwrap_or,
        clippy::option_map_unwrap_or_else,
        clippy::print_stdout,
        clippy::unicode_not_nfc,
        clippy::use_self,
    )
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

mod data_model;
mod host;
mod parse_error;
mod targets;
#[macro_use]
mod triple;

pub use self::data_model::{CDataModel, Size};
pub use self::host::HOST;
pub use self::parse_error::ParseError;
pub use self::targets::{
    Aarch64Architecture, Architecture, ArmArchitecture, BinaryFormat, CleverArchitecture,
    CustomVendor, DeploymentTarget, Environment, Mips32Architecture, Mips64Architecture,
    OperatingSystem, Riscv32Architecture, Riscv64Architecture, Vendor, X86_32Architecture,
};
pub use self::triple::{CallingConvention, Endianness, PointerWidth, Triple};

/// A simple wrapper around `Triple` that provides an implementation of
/// `Default` which defaults to `Triple::host()`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DefaultToHost(pub Triple);

impl Default for DefaultToHost {
    fn default() -> Self {
        Self(Triple::host())
    }
}

/// A simple wrapper around `Triple` that provides an implementation of
/// `Default` which defaults to `Triple::unknown()`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DefaultToUnknown(pub Triple);

impl Default for DefaultToUnknown {
    fn default() -> Self {
        Self(Triple::unknown())
    }
}

// For some reason, the below `serde` impls don't work when they're in the
// `triple` module.

#[cfg(feature = "serde_support")]
impl serde::Serialize for Triple {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde_support")]
impl<'de> serde::de::Deserialize<'de> for Triple {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde_support")]
#[test]
fn test_serialize() {
    let triples: Vec<Triple> = vec![
        "x86_64-unknown-linux-gnu".parse().unwrap(),
        "i686-pc-windows-gnu".parse().unwrap(),
    ];

    let json = serde_json::to_string(&triples).unwrap();
    assert_eq!(
        json,
        r#"["x86_64-unknown-linux-gnu","i686-pc-windows-gnu"]"#
    );
}

#[cfg(feature = "serde_support")]
#[test]
fn test_deserialize() {
    let triples: Vec<Triple> = vec![
        "x86_64-unknown-linux-gnu".parse().unwrap(),
        "i686-pc-windows-gnu".parse().unwrap(),
    ];

    let vals: Vec<Triple> =
        serde_json::from_str(r#"["x86_64-unknown-linux-gnu","i686-pc-windows-gnu"]"#).unwrap();

    assert_eq!(vals, triples);
}
