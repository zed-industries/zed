/// A legacy 16/16 version encoding
/// Packed 32-bit value with major and minor version numbers.
///
/// This is a legacy type with an unusual representation. See [the spec][spec] for
/// additional details.
///
/// [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-version-numbers
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct Version16Dot16(u32);

/// A type representing a major, minor version pair.
///
/// This is not part of [the spec][spec], but versions in the spec are frequently
/// represented as a `major_version`, `minor_version` pair. This type encodes
/// those as a single type, which is useful for some of the generated code that
/// parses out a version.
///
/// [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-version-numbers
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(C)]
pub struct MajorMinor {
    /// The major version number
    pub major: u16,
    /// The minor version number
    pub minor: u16,
}

/// A trait for determining whether versions are compatible.
pub trait Compatible<Rhs = Self>: Sized {
    /// return `true` if this version is field-compatible with `other`.
    ///
    /// This is kind of poorly defined, but basically means 'same major version,
    /// greater than or equal minor version'.
    fn compatible(&self, other: Rhs) -> bool;
}

impl Version16Dot16 {
    /// Version 0.5
    pub const VERSION_0_5: Version16Dot16 = Version16Dot16::new(0, 5);
    /// Version 1.0
    pub const VERSION_1_0: Version16Dot16 = Version16Dot16::new(1, 0);
    /// Version 1.1
    pub const VERSION_1_1: Version16Dot16 = Version16Dot16::new(1, 1);
    /// Version 2.0
    pub const VERSION_2_0: Version16Dot16 = Version16Dot16::new(2, 0);
    /// Version 2.5
    pub const VERSION_2_5: Version16Dot16 = Version16Dot16::new(2, 5);
    /// Version 3.0
    pub const VERSION_3_0: Version16Dot16 = Version16Dot16::new(3, 0);

    /// Create a new version with the provided major and minor parts.
    ///
    /// The minor version must be in the range 0..=9.
    ///
    /// # Panics
    ///
    /// Panics if `minor > 9`.
    pub const fn new(major: u16, minor: u16) -> Self {
        assert!(minor < 10, "minor version must be in the range [0, 9]");
        let version = ((major as u32) << 16) | ((minor as u32) << 12);
        Version16Dot16(version)
    }

    /// Return the separate major & minor version numbers.
    pub const fn to_major_minor(self) -> (u16, u16) {
        let major = (self.0 >> 16) as u16;
        let minor = ((self.0 & 0xFFFF) >> 12) as u16;
        (major, minor)
    }

    /// The representation of this version as a big-endian byte array.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

crate::newtype_scalar!(Version16Dot16, [u8; 4]);

impl MajorMinor {
    /// Version 1.0
    pub const VERSION_1_0: MajorMinor = MajorMinor::new(1, 0);
    /// Version 1.1
    pub const VERSION_1_1: MajorMinor = MajorMinor::new(1, 1);
    /// Version 1.2
    pub const VERSION_1_2: MajorMinor = MajorMinor::new(1, 2);
    /// Version 1.3
    pub const VERSION_1_3: MajorMinor = MajorMinor::new(1, 3);
    /// Version 2.0
    pub const VERSION_2_0: MajorMinor = MajorMinor::new(2, 0);

    /// Create a new version with major and minor parts.
    #[inline]
    pub const fn new(major: u16, minor: u16) -> Self {
        MajorMinor { major, minor }
    }

    /// The representation of this version as a big-endian byte array.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        let [a, b] = self.major.to_be_bytes();
        let [c, d] = self.minor.to_be_bytes();
        [a, b, c, d]
    }
}

impl crate::Scalar for MajorMinor {
    type Raw = [u8; 4];

    fn from_raw(raw: Self::Raw) -> Self {
        let major = u16::from_be_bytes([raw[0], raw[1]]);
        let minor = u16::from_be_bytes([raw[2], raw[3]]);
        Self { major, minor }
    }

    fn to_raw(self) -> Self::Raw {
        self.to_be_bytes()
    }
}

impl Compatible for Version16Dot16 {
    #[inline]
    fn compatible(&self, other: Self) -> bool {
        let (self_major, self_minor) = self.to_major_minor();
        let (other_major, other_minor) = other.to_major_minor();
        self_major == other_major && self_minor >= other_minor
    }
}

impl Compatible<(u16, u16)> for Version16Dot16 {
    fn compatible(&self, other: (u16, u16)) -> bool {
        self.compatible(Version16Dot16::new(other.0, other.1))
    }
}

impl Compatible for MajorMinor {
    #[inline]
    fn compatible(&self, other: Self) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}

impl Compatible<(u16, u16)> for MajorMinor {
    fn compatible(&self, other: (u16, u16)) -> bool {
        self.compatible(MajorMinor::new(other.0, other.1))
    }
}

impl Compatible for u16 {
    #[inline]
    fn compatible(&self, other: Self) -> bool {
        *self >= other
    }
}

impl std::fmt::Debug for Version16Dot16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Version16Dot16({:08x})", self.0)
    }
}

impl std::fmt::Display for Version16Dot16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (major, minor) = self.to_major_minor();
        write!(f, "{major}.{minor}")
    }
}

impl std::fmt::Display for MajorMinor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let MajorMinor { major, minor } = self;
        write!(f, "{major}.{minor}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_smoke_test() {
        assert_eq!(Version16Dot16(0x00005000).to_major_minor(), (0, 5));
        assert_eq!(Version16Dot16(0x00011000).to_major_minor(), (1, 1));
        assert_eq!(Version16Dot16::new(0, 5).0, 0x00005000);
        assert_eq!(Version16Dot16::new(1, 1).0, 0x00011000);
    }

    #[test]
    #[should_panic]
    fn minor_version_out_of_range_test() {
        Version16Dot16::new(1, 10);
    }
}
