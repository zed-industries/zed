//! Contains code for selecting features

#![deny(unused_extern_crates)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(deprecated)]

use std::str::FromStr;
use std::{fmt, io};

/// Represents the version of the Rust language to target.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RustTarget(Version);

impl RustTarget {
    /// Create a new [`RustTarget`] for a stable release of Rust.
    pub fn stable(minor: u64, patch: u64) -> Result<Self, InvalidRustTarget> {
        let target = Self(Version::Stable(minor, patch));

        if target < EARLIEST_STABLE_RUST {
            return Err(InvalidRustTarget::TooEarly);
        }

        Ok(target)
    }

    const fn minor(&self) -> Option<u64> {
        match self.0 {
            Version::Nightly => None,
            Version::Stable(minor, _) => Some(minor),
        }
    }

    const fn is_compatible(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (Version::Stable(minor, _), Version::Stable(other_minor, _)) => {
                // We ignore the patch version number as they only include backwards compatible bug
                // fixes.
                minor >= other_minor
            }
            // Nightly is compatible with everything
            (Version::Nightly, _) => true,
            // No stable release is compatible with nightly
            (Version::Stable { .. }, Version::Nightly) => false,
        }
    }
}

impl Default for RustTarget {
    fn default() -> Self {
        LATEST_STABLE_RUST
    }
}

impl fmt::Display for RustTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Version::Stable(minor, patch) => write!(f, "1.{minor}.{patch}"),
            Version::Nightly => "nightly".fmt(f),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Version {
    Stable(u64, u64),
    Nightly,
}

pub enum InvalidRustTarget {
    TooEarly,
}

impl fmt::Display for InvalidRustTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooEarly => write!(f, "the earliest Rust version supported by bindgen is {EARLIEST_STABLE_RUST}"),
        }
    }
}

/// This macro defines the Rust editions supported by bindgen.
macro_rules! define_rust_editions {
    ($($variant:ident($value:literal) => $minor:literal,)*) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[doc = "Represents Rust Edition for the generated bindings"]
        pub enum RustEdition {
            $(
                #[doc = concat!("The ", stringify!($value), " edition of Rust.")]
                $variant,
            )*
        }

        impl FromStr for RustEdition {
            type Err = InvalidRustEdition;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($value) => Ok(Self::$variant),)*
                    _ => Err(InvalidRustEdition(s.to_owned())),
                }
            }
        }

        impl fmt::Display for RustEdition {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant => stringify!($value).fmt(f),)*
                }
            }
        }

        impl RustEdition {
            pub(crate) const ALL: [Self; [$($value,)*].len()] = [$(Self::$variant,)*];

            pub(crate) fn is_available(self, target: RustTarget) -> bool {
                let Some(minor) = target.minor() else {
                    return true;
                };

                match self {
                    $(Self::$variant => $minor <= minor,)*
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct InvalidRustEdition(String);

impl fmt::Display for InvalidRustEdition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" is not a valid Rust edition", self.0)
    }
}

impl std::error::Error for InvalidRustEdition {}

define_rust_editions! {
    Edition2018(2018) => 31,
    Edition2021(2021) => 56,
    Edition2024(2024) => 85,
}

impl RustTarget {
    /// Returns the latest edition supported by this target.
    pub(crate) fn latest_edition(self) -> RustEdition {
        RustEdition::ALL
            .iter()
            .rev()
            .find(|edition| edition.is_available(self))
            .copied()
            .expect("bindgen should always support at least one edition")
    }
}

impl Default for RustEdition {
    fn default() -> Self {
        RustTarget::default().latest_edition()
    }
}

/// This macro defines the [`RustTarget`] and [`RustFeatures`] types.
macro_rules! define_rust_targets {
    (
        Nightly => {$($nightly_feature:ident $(($nightly_edition:literal))|* $(: #$issue:literal)?),* $(,)?} $(,)?
        $(
            $variant:ident($minor:literal) => {$($feature:ident $(($edition:literal))|* $(: #$pull:literal)?),* $(,)?},
        )*
        $(,)?
    ) => {

        impl RustTarget {
            /// The nightly version of Rust, which introduces the following features:"
            $(#[doc = concat!(
                "- [`", stringify!($nightly_feature), "`]",
                "(", $("https://github.com/rust-lang/rust/pull/", stringify!($issue),)* ")",
            )])*
            #[deprecated = "The use of this constant is deprecated, please use `RustTarget::nightly` instead."]
            pub const Nightly: Self = Self::nightly();

            /// The nightly version of Rust, which introduces the following features:"
            $(#[doc = concat!(
                "- [`", stringify!($nightly_feature), "`]",
                "(", $("https://github.com/rust-lang/rust/pull/", stringify!($issue),)* ")",
            )])*
            pub const fn nightly() -> Self {
                Self(Version::Nightly)
            }

            $(
                #[doc = concat!("Version 1.", stringify!($minor), " of Rust, which introduced the following features:")]
                $(#[doc = concat!(
                    "- [`", stringify!($feature), "`]",
                    "(", $("https://github.com/rust-lang/rust/pull/", stringify!($pull),)* ")",
                )])*
                #[deprecated = "The use of this constant is deprecated, please use `RustTarget::stable` instead."]
                pub const $variant: Self = Self(Version::Stable($minor, 0));
            )*

            const fn stable_releases() -> [(Self, u64); [$($minor,)*].len()] {
                [$((Self::$variant, $minor),)*]
            }
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub(crate) struct RustFeatures {
            $($(pub(crate) $feature: bool,)*)*
            $(pub(crate) $nightly_feature: bool,)*
        }

        impl RustFeatures {
            /// Compute the features that must be enabled in a specific Rust target with a specific edition.
            pub(crate) fn new(target: RustTarget, edition: RustEdition) -> Self {
                let mut features = Self {
                    $($($feature: false,)*)*
                    $($nightly_feature: false,)*
                };

                if target.is_compatible(&RustTarget::nightly()) {
                    $(
                        let editions: &[RustEdition] = &[$(stringify!($nightly_edition).parse::<RustEdition>().ok().expect("invalid edition"),)*];

                        if editions.is_empty() || editions.contains(&edition) {
                            features.$nightly_feature = true;
                        }
                    )*
                }

                $(
                    if target.is_compatible(&RustTarget::$variant) {
                        $(
                            let editions: &[RustEdition] = &[$(stringify!($edition).parse::<RustEdition>().ok().expect("invalid edition"),)*];

                            if editions.is_empty() || editions.contains(&edition) {
                                features.$feature = true;
                            }
                        )*
                    }
                )*

                features
            }
        }
    };
}

// NOTE: When adding or removing features here, make sure to add the stabilization PR
// number for the feature if it has been stabilized or the tracking issue number if the feature is
// not stable.
define_rust_targets! {
    Nightly => {
        vectorcall_abi: #124485,
        ptr_metadata: #81513,
        layout_for_ptr: #69835,
    },
    Stable_1_82(82) => {
        unsafe_extern_blocks: #127921,
    },
    Stable_1_77(77) => {
        offset_of: #106655,
        literal_cstr(2021)|(2024): #117472,
    },
    Stable_1_73(73) => { thiscall_abi: #42202 },
    Stable_1_71(71) => { c_unwind_abi: #106075 },
    Stable_1_68(68) => { abi_efiapi: #105795 },
    Stable_1_64(64) => { core_ffi_c: #94503 },
    Stable_1_51(51) => {
        raw_ref_macros: #80886,
        min_const_generics: #74878,
    },
    Stable_1_59(59) => { const_cstr: #54745 },
    Stable_1_47(47) => { larger_arrays: #74060 },
    Stable_1_43(43) => { associated_constants: #68952 },
    Stable_1_40(40) => { non_exhaustive: #44109 },
    Stable_1_36(36) => { maybe_uninit: #60445 },
    Stable_1_33(33) => { repr_packed_n: #57049 },
}

/// Latest stable release of Rust that is supported by bindgen
pub const LATEST_STABLE_RUST: RustTarget = {
    // FIXME: replace all this code by
    // ```
    // RustTarget::stable_releases()
    //     .into_iter()
    //     .max_by_key(|(_, m)| m)
    //     .map(|(t, _)| t)
    //     .unwrap()
    // ```
    // once those operations can be used in constants.
    let targets = RustTarget::stable_releases();

    let mut i = 0;
    let mut latest_target = None;
    let mut latest_minor = 0;

    while i < targets.len() {
        let (target, minor) = targets[i];

        if latest_minor < minor {
            latest_minor = minor;
            latest_target = Some(target);
        }

        i += 1;
    }

    match latest_target {
        Some(target) => target,
        None => unreachable!(),
    }
};

/// Earliest stable release of Rust that is supported by bindgen
pub const EARLIEST_STABLE_RUST: RustTarget = {
    // FIXME: replace all this code by
    // ```
    // RustTarget::stable_releases()
    //     .into_iter()
    //     .min_by_key(|(_, m)| m)
    //     .map(|(t, _)| t)
    //     .unwrap_or(LATEST_STABLE_RUST)
    // ```
    // once those operations can be used in constants.
    let targets = RustTarget::stable_releases();

    let mut i = 0;
    let mut earliest_target = None;
    let Some(mut earliest_minor) = LATEST_STABLE_RUST.minor() else {
        unreachable!()
    };

    while i < targets.len() {
        let (target, minor) = targets[i];

        if earliest_minor > minor {
            earliest_minor = minor;
            earliest_target = Some(target);
        }

        i += 1;
    }

    match earliest_target {
        Some(target) => target,
        None => unreachable!(),
    }
};

fn invalid_input(input: &str, msg: impl fmt::Display) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("\"{input}\" is not a valid Rust target, {msg}"),
    )
}

impl FromStr for RustTarget {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "nightly" {
            return Ok(Self::Nightly);
        }

        let Some((major_str, tail)) = input.split_once('.') else {
            return Err(invalid_input(input, "accepted values are of the form \"1.71\", \"1.71.1\" or \"nightly\"." ) );
        };

        if major_str != "1" {
            return Err(invalid_input(
                input,
                "The largest major version of Rust released is \"1\"",
            ));
        }

        let (minor, patch) = if let Some((minor_str, patch_str)) =
            tail.split_once('.')
        {
            let Ok(minor) = minor_str.parse::<u64>() else {
                return Err(invalid_input(input, "the minor version number must be an unsigned 64-bit integer"));
            };
            let Ok(patch) = patch_str.parse::<u64>() else {
                return Err(invalid_input(input, "the patch version number must be an unsigned 64-bit integer"));
            };
            (minor, patch)
        } else {
            let Ok(minor) = tail.parse::<u64>() else {
                return Err(invalid_input(input, "the minor version number must be an unsigned 64-bit integer"));
            };
            (minor, 0)
        };

        Self::stable(minor, patch).map_err(|err| invalid_input(input, err))
    }
}

impl RustFeatures {
    /// Compute the features that must be enabled in a specific Rust target with the latest edition
    /// available in that target.
    pub(crate) fn new_with_latest_edition(target: RustTarget) -> Self {
        Self::new(target, target.latest_edition())
    }
}

impl Default for RustFeatures {
    fn default() -> Self {
        Self::new_with_latest_edition(RustTarget::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn release_versions_for_editions() {
        assert_eq!(
            "1.33".parse::<RustTarget>().unwrap().latest_edition(),
            RustEdition::Edition2018
        );
        assert_eq!(
            "1.56".parse::<RustTarget>().unwrap().latest_edition(),
            RustEdition::Edition2021
        );
        assert_eq!(
            "1.85".parse::<RustTarget>().unwrap().latest_edition(),
            RustEdition::Edition2024
        );
        assert_eq!(
            "nightly".parse::<RustTarget>().unwrap().latest_edition(),
            RustEdition::Edition2024
        );
    }

    #[test]
    fn target_features() {
        let features =
            RustFeatures::new_with_latest_edition(RustTarget::Stable_1_71);
        assert!(
            features.c_unwind_abi &&
                features.abi_efiapi &&
                !features.thiscall_abi
        );

        let features = RustFeatures::new(
            RustTarget::Stable_1_77,
            RustEdition::Edition2018,
        );
        assert!(!features.literal_cstr);

        let features =
            RustFeatures::new_with_latest_edition(RustTarget::Stable_1_77);
        assert!(features.literal_cstr);

        let f_nightly =
            RustFeatures::new_with_latest_edition(RustTarget::Nightly);
        assert!(
            f_nightly.vectorcall_abi &&
                f_nightly.ptr_metadata &&
                f_nightly.layout_for_ptr
        );
    }

    fn test_target(input: &str, expected: RustTarget) {
        // Two targets are equivalent if they enable the same set of features
        let expected = RustFeatures::new_with_latest_edition(expected);
        let found = RustFeatures::new_with_latest_edition(
            input.parse::<RustTarget>().unwrap(),
        );
        assert_eq!(
            expected,
            found,
            "target {input} enables features:\n{found:#?}\nand should enable features:\n{expected:#?}"
        );
    }

    fn test_invalid_target(input: &str) {
        assert!(
            input.parse::<RustTarget>().is_err(),
            "{input} should be an invalid target"
        );
    }

    #[test]
    fn valid_targets() {
        test_target("1.71", RustTarget::Stable_1_71);
        test_target("1.71.0", RustTarget::Stable_1_71);
        test_target("1.71.1", RustTarget::Stable_1_71);
        test_target("1.72", RustTarget::Stable_1_71);
        test_target("1.73", RustTarget::Stable_1_73);
        test_target("1.18446744073709551615", LATEST_STABLE_RUST);
        test_target("nightly", RustTarget::Nightly);
    }

    #[test]
    fn invalid_targets() {
        test_invalid_target("2.0");
        test_invalid_target("1.cat");
        test_invalid_target("1.0.cat");
        test_invalid_target("1.18446744073709551616");
        test_invalid_target("1.0.18446744073709551616");
        test_invalid_target("1.-1.0");
        test_invalid_target("1.0.-1");
        test_invalid_target("beta");
        test_invalid_target("1.0.0");
        test_invalid_target("1.32.0");
    }
}
