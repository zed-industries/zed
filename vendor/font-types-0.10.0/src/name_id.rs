//! Name Identifiers
//!
//! Although these are treated as u16s in the spec, we choose to represent them
//! as a distinct type.

use core::fmt;

/// Identifier for an informational string (or name).
///
/// A set of predefined identifiers exist for accessing names and other various metadata
/// about the font and those are provided as associated constants on this type.
///
/// IDs 26 to 255, inclusive, are reserved for future standard names. IDs 256 to 32767,
/// inclusive, are reserved for font-specific names such as those referenced by a font's
/// layout features.
///
/// For more detail, see <https://learn.microsoft.com/en-us/typography/opentype/spec/name#name-ids>
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct NameId(u16);

impl NameId {
    /// Copyright notice.
    pub const COPYRIGHT_NOTICE: Self = Self(0);

    /// Font family name.
    ///
    /// The font family name is used in combination with font subfamily name (ID 2),
    /// and should be shared among at most four fonts that differ only in weight or style (as described below).
    pub const FAMILY_NAME: Self = Self(1);

    /// Font subfamily name.
    ///
    /// The font subfamily name is used in combination with font family name (ID 1),
    /// and distinguishes the fonts in a group with the same font family name. This should be used for style
    /// and weight variants only (as described below).
    pub const SUBFAMILY_NAME: Self = Self(2);

    /// Unique font identifier.
    pub const UNIQUE_ID: Self = Self(3);

    /// Full font name that reflects all family and relevant subfamily descriptors.
    ///
    /// The full font name is generally a combination of IDs 1 and 2, or of IDs 16 and
    /// 17, or a similar human-readable variant.
    pub const FULL_NAME: Self = Self(4);

    /// Version string.
    ///
    /// Should begin with the syntax “Version number.number” (upper case, lower case, or mixed,
    /// with a space between “Version” and the number).
    pub const VERSION_STRING: Self = Self(5);

    /// PostScript name for the font.
    ///
    /// ID 6 specifies a string which is used to invoke a PostScript language font
    /// that corresponds to this OpenType font. When translated to ASCII, the name string must be no longer than 63
    /// characters and restricted to the printable ASCII subset, codes 33 to 126, except for the 10 characters '[', ']',
    /// '(', ')', '{', '}', '<', '>', '/', '%'.
    pub const POSTSCRIPT_NAME: Self = Self(6);

    /// Trademark; this is used to save any trademark notice/information for this font.
    ///
    /// Such information should be based on legal advice. This is distinctly separate from the copyright.
    pub const TRADEMARK: Self = Self(7);

    /// Manufacturer name.
    pub const MANUFACTURER: Self = Self(8);

    /// Name of the designer of the typeface.
    pub const DESIGNER: Self = Self(9);

    /// Description of the typeface.
    ///
    /// Can contain revision information, usage recommendations, history, features, etc.
    pub const DESCRIPTION: Self = Self(10);

    /// URL of font vendor (with protocol, e.g., http://, ftp://).
    ///
    /// If a unique serial number is embedded in the URL, it can be used to register the font.
    pub const VENDOR_URL: Self = Self(11);

    /// URL of typeface designer (with protocol, e.g., http://, ftp://).
    pub const DESIGNER_URL: Self = Self(12);

    /// License description.
    ///
    /// A description of how the font may be legally used, or different example scenarios for licensed use.
    /// This field should be written in plain language, not legalese.
    pub const LICENSE_DESCRIPTION: Self = Self(13);

    /// URL where additional licensing information can be found.
    pub const LICENSE_URL: Self = Self(14);

    /// Typographic family name.
    ///
    /// The typographic family grouping doesn't impose any constraints on the number of faces within it,
    /// in contrast with the 4-style family grouping (ID 1), which is present both for historical reasons and to express style
    /// linking groups.
    pub const TYPOGRAPHIC_FAMILY_NAME: Self = Self(16);

    /// Typographic subfamily name.
    ///
    /// This allows font designers to specify a subfamily name within the typographic family grouping.
    /// This string must be unique within a particular typographic family.
    pub const TYPOGRAPHIC_SUBFAMILY_NAME: Self = Self(17);

    /// Compatible full (Macintosh only).
    ///
    /// On the Macintosh, the menu name is constructed using the FOND resource. This usually matches
    /// the full name. If you want the name of the font to appear differently than the Full Name, you can insert the compatible full
    /// name in ID 18.
    pub const COMPATIBLE_FULL_NAME: Self = Self(18);

    /// Sample text.
    ///
    /// This can be the font name, or any other text that the designer thinks is the best sample to display the font in.
    pub const SAMPLE_TEXT: Self = Self(19);

    /// PostScript CID findfont name.
    ///
    /// Its presence in a font means that the ID 6 holds a PostScript font name that is meant to be
    /// used with the “composefont” invocation in order to invoke the font in a PostScript interpreter.
    pub const POSTSCRIPT_CID_NAME: Self = Self(20);

    /// WWS family name.
    ///
    /// Used to provide a WWS-conformant family name in case the entries for IDs 16 and 17 do not conform to the WWS model.
    pub const WWS_FAMILY_NAME: Self = Self(21);

    /// WWS subfamily name.
    ///
    /// Used in conjunction with ID 21, this ID provides a WWS-conformant subfamily name (reflecting only weight, width
    /// and slope attributes) in case the entries for IDs 16 and 17 do not conform to the WWS model.
    pub const WWS_SUBFAMILY_NAME: Self = Self(22);

    /// Light background palette.
    ///
    /// This ID, if used in the CPAL table's Palette Labels Array, specifies that the corresponding color palette in
    /// the CPAL table is appropriate to use with the font when displaying it on a light background such as white.
    pub const LIGHT_BACKGROUND_PALETTE: Self = Self(23);

    /// Dark background palette.
    ///
    /// This ID, if used in the CPAL table's Palette Labels Array, specifies that the corresponding color palette in
    /// the CPAL table is appropriate to use with the font when displaying it on a dark background such as black.
    pub const DARK_BACKGROUND_PALETTE: Self = Self(24);

    /// Variations PostScript name prefix.
    ///
    /// If present in a variable font, it may be used as the family prefix in the PostScript Name Generation
    /// for Variation Fonts algorithm.
    pub const VARIATIONS_POSTSCRIPT_NAME_PREFIX: Self = Self(25);

    /// The last value that is explicitly reserved for standard names.
    ///
    /// Values after this are available to be used for font-specific names.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/name#name-ids>
    pub const LAST_RESERVED_NAME_ID: Self = Self(255);

    /// The last value that is available for font-specific names.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/name#name-ids>
    pub const LAST_ALLOWED_NAME_ID: Self = Self(32767);
}

impl NameId {
    /// Create a new identifier from a raw u16 value.
    #[inline]
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// Returns an iterator over the set of predefined identifiers according to the
    /// specification.
    #[inline]
    pub fn predefined() -> impl Iterator<Item = Self> + Clone {
        // Poor name id 15 got lost...
        (0..15).chain(16..=25).map(Self)
    }

    /// Return the identifier as a u16.
    #[inline]
    pub const fn to_u16(self) -> u16 {
        self.0
    }

    /// Returns `true` if self is in the range `0..=255` (reserved for standard names.)
    #[inline]
    pub const fn is_reserved(self) -> bool {
        self.0 <= Self::LAST_RESERVED_NAME_ID.0
    }

    /// Returns a new `NameId` by adding `rhs` to the current value.
    ///
    /// Returns `None` if the result would be greater than `Self::LAST_ALLOWED_NAME_ID`.
    #[inline]
    pub const fn checked_add(self, rhs: u16) -> Option<Self> {
        let result = Self(self.0.saturating_add(rhs));
        if result.0 > Self::LAST_ALLOWED_NAME_ID.0 {
            None
        } else {
            Some(result)
        }
    }

    /// Return the memory representation of this identifier as a byte array in big-endian
    /// (network) byte order.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl Default for NameId {
    fn default() -> Self {
        Self(0xFFFF)
    }
}

impl From<u16> for NameId {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl fmt::Debug for NameId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::COPYRIGHT_NOTICE => "COPYRIGHT_NOTICE",
            Self::FAMILY_NAME => "FAMILY_NAME",
            Self::SUBFAMILY_NAME => "SUBFAMILY_NAME",
            Self::UNIQUE_ID => "UNIQUE_ID",
            Self::FULL_NAME => "FULL_NAME",
            Self::VERSION_STRING => "VERSION_STRING",
            Self::POSTSCRIPT_NAME => "POSTSCRIPT_NAME",
            Self::TRADEMARK => "TRADEMARK",
            Self::MANUFACTURER => "MANUFACTURER",
            Self::DESIGNER => "DESIGNER",
            Self::DESCRIPTION => "DESCRIPTION",
            Self::VENDOR_URL => "VENDOR_URL",
            Self::DESIGNER_URL => "DESIGNER_URL",
            Self::LICENSE_DESCRIPTION => "LICENSE_DESCRIPTION",
            Self::LICENSE_URL => "LICENSE_URL",
            Self::TYPOGRAPHIC_FAMILY_NAME => "TYPOGRAPHIC_FAMILY_NAME",
            Self::TYPOGRAPHIC_SUBFAMILY_NAME => "TYPOGRAPHIC_SUBFAMILY_NAME",
            Self::COMPATIBLE_FULL_NAME => "COMPATIBLE_FULL_NAME",
            Self::SAMPLE_TEXT => "SAMPLE_TEXT",
            Self::POSTSCRIPT_CID_NAME => "POSTSCRIPT_CID_NAME",
            Self::WWS_FAMILY_NAME => "WWS_FAMILY_NAME",
            Self::WWS_SUBFAMILY_NAME => "WWS_SUBFAMILY_NAME",
            Self::LIGHT_BACKGROUND_PALETTE => "LIGHT_BACKGROUND_PALETTE",
            Self::DARK_BACKGROUND_PALETTE => "DARK_BACKGROUND_PALETTE",
            Self::VARIATIONS_POSTSCRIPT_NAME_PREFIX => "VARIATIONS_POSTSCRIPT_NAME_PREFIX",
            _ => return write!(f, "NameId {}", self.0),
        };
        f.write_str(name)
    }
}

impl fmt::Display for NameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

crate::newtype_scalar!(NameId, [u8; 2]);
