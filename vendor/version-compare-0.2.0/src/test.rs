use crate::{Cmp, Manifest};

/// A manifest configuration for GNU versions.
const MANIFEST_GNU: Option<Manifest> = Some(Manifest {
    gnu_ordering: true,
    max_depth: None,
    ignore_text: false,
});

/// Struct containing a version number with some meta data.
/// Such a set can be used for testing.
///
/// # Arguments
///
/// - `0`: The version string.
/// - `1`: Number of version parts.
pub struct Version(pub &'static str, pub usize);

/// List of version numbers with metadata for dynamic tests
pub const VERSIONS: &[Version] = &[
    Version("1", 1),
    Version("1.2", 2),
    Version("1.2.3.4", 4),
    Version("1.2.3.4.5.6.7.8", 8),
    Version("0", 1),
    Version("0.0.0", 3),
    Version("1.0.0", 3),
    Version("0.0.1", 3),
    Version("", 0),
    Version(".", 0),
    Version("...", 0),
    Version("1.2.dev", 3),
    Version("1.2-dev", 3),
    Version("1.2.alpha.4", 4),
    Version("1.2-alpha-4", 4),
    Version("snapshot.1.2", 3),
    Version("snapshot-1.2", 3),
    // Issue: https://github.com/timvisee/version-compare/issues/26
    Version("0.0.1-test.0222426166a", 6),
    Version("0.0.1-test.0222426166565421816516584651684351354", 5),
    Version("0.0.1-test.02224261665a", 5),
    Version("0.0.1-test.02224261665d7b1b689816d12f6bcacb", 5),
];

/// List of version numbers that contain errors with metadata for dynamic tests
pub const VERSIONS_ERROR: &[Version] = &[
    Version("abc", 1),
    Version("alpha.dev.snapshot", 3),
    Version("test. .snapshot", 3),
];

/// Struct containing two version numbers, and the comparison operator.
/// Such a set can be used for testing.
///
/// # Arguments
///
/// - `0`: The main version.
/// - `1`: The other version.
/// - `2`: The comparison operator.
/// - `3`: An optional custom manifest.
pub struct VersionCombi(
    pub &'static str,
    pub &'static str,
    pub Cmp,
    pub Option<Manifest>,
);

impl VersionCombi {
    /// Get versions.
    pub fn versions(&self) -> (crate::Version, crate::Version) {
        match self.3 {
            Some(ref manifest) => (
                crate::Version::from_manifest(self.0, manifest).unwrap(),
                crate::Version::from_manifest(self.1, manifest).unwrap(),
            ),
            None => (
                crate::Version::from(self.0).unwrap(),
                crate::Version::from(self.1).unwrap(),
            ),
        }
    }
}

/// List of version combinations for dynamic tests
pub const COMBIS: &[VersionCombi] = &[
    VersionCombi("1", "1", Cmp::Eq, None),
    VersionCombi("1.0.0.0", "1", Cmp::Eq, None),
    VersionCombi("1", "1.0.0.0", Cmp::Eq, None),
    VersionCombi("0", "0", Cmp::Eq, None),
    VersionCombi("0.0.0", "0", Cmp::Eq, None),
    VersionCombi("0", "0.0.0", Cmp::Eq, None),
    VersionCombi("", "", Cmp::Eq, None),
    VersionCombi("", "0.0", Cmp::Eq, None),
    VersionCombi("0.0", "", Cmp::Eq, None),
    VersionCombi("", "0.1", Cmp::Lt, None),
    VersionCombi("0.1", "", Cmp::Gt, None),
    VersionCombi("1.2.3", "1.2.3", Cmp::Eq, None),
    VersionCombi("1.2.3", "1.2.4", Cmp::Lt, None),
    VersionCombi("1.0.0.1", "1.0.0.0", Cmp::Gt, None),
    VersionCombi("1.0.0.0", "1.0.0.1", Cmp::Lt, None),
    VersionCombi("1.2.3.4", "1.2", Cmp::Gt, None),
    VersionCombi("1.2", "1.2.3.4", Cmp::Lt, None),
    VersionCombi("1.2.3.4", "2", Cmp::Lt, None),
    VersionCombi("2", "1.2.3.4", Cmp::Gt, None),
    VersionCombi("123", "123", Cmp::Eq, None),
    VersionCombi("123", "1.2.3", Cmp::Gt, None),
    VersionCombi("1.2.3", "123", Cmp::Lt, None),
    VersionCombi("1.1.2", "1.1.30-dev", Cmp::Lt, None),
    VersionCombi("1.2.3", "1.2.3.alpha", Cmp::Gt, None),
    VersionCombi("1.2.3", "1.2.3-dev", Cmp::Gt, None),
    VersionCombi("1.2.3 RC0", "1.2.3 rc1", Cmp::Lt, None),
    VersionCombi("1.2.3 rc2", "1.2.3 RC99", Cmp::Lt, None),
    VersionCombi("1.2.3 RC3", "1.2.3 RC1", Cmp::Gt, None),
    VersionCombi("1.2.3a", "1.2.3b", Cmp::Lt, None),
    VersionCombi("1.2.3b", "1.2.3a", Cmp::Gt, None),
    VersionCombi("1.2.3.dev", "1.2.3.alpha", Cmp::Gt, None),
    VersionCombi("1.2.3-dev", "1.2.3-alpha", Cmp::Gt, None),
    VersionCombi("1.2.3.dev.1", "1.2.3.alpha", Cmp::Gt, None),
    VersionCombi("1.2.3-dev-1", "1.2.3-alpha", Cmp::Gt, None),
    VersionCombi("version-compare 3.2.0 / build 0932", "3.2.5", Cmp::Lt, None),
    VersionCombi("version-compare 3.2.0 / build 0932", "3.1.1", Cmp::Gt, None),
    VersionCombi(
        "version-compare 1.4.1 / build 0043",
        "version-compare 1.4.1 / build 0043",
        Cmp::Eq,
        None,
    ),
    VersionCombi(
        "version-compare 1.4.1 / build 0042",
        "version-compare 1.4.1 / build 0043",
        Cmp::Lt,
        None,
    ),
    // Issue: https://github.com/timvisee/version-compare/issues/24
    VersionCombi("7.2p1", "7.1", Cmp::Gt, None),
    // GNU style versioning, issue: https://github.com/timvisee/version-compare/issues/27
    VersionCombi("1.1", "1.02", Cmp::Lt, MANIFEST_GNU),
    VersionCombi("1.02", "1.2", Cmp::Lt, MANIFEST_GNU),
    VersionCombi("1.02", "1.03", Cmp::Lt, MANIFEST_GNU),
    VersionCombi("1.0.2", "1.02", Cmp::Lt, MANIFEST_GNU),
    VersionCombi(
        "string start 5.3.0 end of str",
        "string start 5.04.0 end of str",
        Cmp::Lt,
        MANIFEST_GNU,
    ),
    VersionCombi(
        "string start 5.04.0 end of str",
        "string start 5.4.0 end of str",
        Cmp::Lt,
        MANIFEST_GNU,
    ),
    // TODO: inspect these cases
    VersionCombi("snapshot.1.2.3", "1.2.3.alpha", Cmp::Lt, None),
    VersionCombi("snapshot-1.2.3", "1.2.3-alpha", Cmp::Lt, None),
];

/// List of invalid version combinations for dynamic tests
pub const COMBIS_ERROR: &[VersionCombi] = &[
    VersionCombi("1.2.3", "1.2.3", Cmp::Lt, None),
    VersionCombi("1.2", "1.2.0.0", Cmp::Ne, None),
    VersionCombi("1.2.3.dev", "dev", Cmp::Eq, None),
    VersionCombi("snapshot", "1", Cmp::Lt, None),
];
