//! A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2)
//! implementation.

use crate::parser::Stream;
use crate::LineMetrics;

const WEIGHT_CLASS_OFFSET: usize = 4;
const WIDTH_CLASS_OFFSET: usize = 6;
const TYPE_OFFSET: usize = 8;
const Y_SUBSCRIPT_X_SIZE_OFFSET: usize = 10;
const Y_SUPERSCRIPT_X_SIZE_OFFSET: usize = 18;
const Y_STRIKEOUT_SIZE_OFFSET: usize = 26;
const Y_STRIKEOUT_POSITION_OFFSET: usize = 28;
const UNICODE_RANGES_OFFSET: usize = 42;
const SELECTION_OFFSET: usize = 62;
const TYPO_ASCENDER_OFFSET: usize = 68;
const TYPO_DESCENDER_OFFSET: usize = 70;
const TYPO_LINE_GAP_OFFSET: usize = 72;
const WIN_ASCENT: usize = 74;
const WIN_DESCENT: usize = 76;
const X_HEIGHT_OFFSET: usize = 86;
const CAP_HEIGHT_OFFSET: usize = 88;

/// A face [weight](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass).
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Weight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    Other(u16),
}

impl Weight {
    /// Returns a numeric representation of a weight.
    #[inline]
    pub fn to_number(self) -> u16 {
        match self {
            Weight::Thin => 100,
            Weight::ExtraLight => 200,
            Weight::Light => 300,
            Weight::Normal => 400,
            Weight::Medium => 500,
            Weight::SemiBold => 600,
            Weight::Bold => 700,
            Weight::ExtraBold => 800,
            Weight::Black => 900,
            Weight::Other(n) => n,
        }
    }
}

impl From<u16> for Weight {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            100 => Weight::Thin,
            200 => Weight::ExtraLight,
            300 => Weight::Light,
            400 => Weight::Normal,
            500 => Weight::Medium,
            600 => Weight::SemiBold,
            700 => Weight::Bold,
            800 => Weight::ExtraBold,
            900 => Weight::Black,
            _ => Weight::Other(value),
        }
    }
}

impl Default for Weight {
    #[inline]
    fn default() -> Self {
        Weight::Normal
    }
}

/// A face [width](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass).
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum Width {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Width {
    /// Returns a numeric representation of a width.
    #[inline]
    pub fn to_number(self) -> u16 {
        match self {
            Width::UltraCondensed => 1,
            Width::ExtraCondensed => 2,
            Width::Condensed => 3,
            Width::SemiCondensed => 4,
            Width::Normal => 5,
            Width::SemiExpanded => 6,
            Width::Expanded => 7,
            Width::ExtraExpanded => 8,
            Width::UltraExpanded => 9,
        }
    }
}

impl Default for Width {
    #[inline]
    fn default() -> Self {
        Width::Normal
    }
}

/// Face [permissions](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fst).
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum Permissions {
    Installable,
    Restricted,
    PreviewAndPrint,
    Editable,
}

/// A face style.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Style {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Default for Style {
    #[inline]
    fn default() -> Style {
        Style::Normal
    }
}

/// A script metrics used by subscript and superscript.
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct ScriptMetrics {
    /// Horizontal face size.
    pub x_size: i16,

    /// Vertical face size.
    pub y_size: i16,

    /// X offset.
    pub x_offset: i16,

    /// Y offset.
    pub y_offset: i16,
}

// https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fsselection
#[derive(Clone, Copy)]
struct SelectionFlags(u16);

#[rustfmt::skip]
impl SelectionFlags {
    #[inline] fn italic(self) -> bool { self.0 & (1 << 0) != 0 }
    #[inline] fn bold(self) -> bool { self.0 & (1 << 5) != 0 }
    // #[inline] fn regular(self) -> bool { self.0 & (1 << 6) != 0 }
    #[inline] fn use_typo_metrics(self) -> bool { self.0 & (1 << 7) != 0 }
    #[inline] fn oblique(self) -> bool { self.0 & (1 << 9) != 0 }
}

/// [Unicode Ranges](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#ur).
#[derive(Clone, Copy, Default, Debug)]
pub struct UnicodeRanges(pub u128);

impl UnicodeRanges {
    /// Checks if ranges contain the specified character.
    pub fn contains_char(&self, c: char) -> bool {
        let range = char_range_index(c);
        if range >= 0 {
            self.0 & (1 << range as u128) != 0
        } else {
            false
        }
    }
}

fn char_range_index(c: char) -> i8 {
    match c as u32 {
        0x0000..=0x007F => 0,
        0x0080..=0x00FF => 1,
        0x0100..=0x017F => 2,
        0x0180..=0x024F => 3,
        0x0250..=0x02AF => 4,
        0x1D00..=0x1DBF => 4,
        0x02B0..=0x02FF => 5,
        0xA700..=0xA71F => 5,
        0x0300..=0x036F => 6,
        0x1DC0..=0x1DFF => 6,
        0x0370..=0x03FF => 7,
        0x2C80..=0x2CFF => 8,
        0x0400..=0x052F => 9,
        0x2DE0..=0x2DFF => 9,
        0xA640..=0xA69F => 9,
        0x0530..=0x058F => 10,
        0x0590..=0x05FF => 11,
        0xA500..=0xA63F => 12,
        0x0600..=0x06FF => 13,
        0x0750..=0x077F => 13,
        0x07C0..=0x07FF => 14,
        0x0900..=0x097F => 15,
        0x0980..=0x09FF => 16,
        0x0A00..=0x0A7F => 17,
        0x0A80..=0x0AFF => 18,
        0x0B00..=0x0B7F => 19,
        0x0B80..=0x0BFF => 20,
        0x0C00..=0x0C7F => 21,
        0x0C80..=0x0CFF => 22,
        0x0D00..=0x0D7F => 23,
        0x0E00..=0x0E7F => 24,
        0x0E80..=0x0EFF => 25,
        0x10A0..=0x10FF => 26,
        0x2D00..=0x2D2F => 26,
        0x1B00..=0x1B7F => 27,
        0x1100..=0x11FF => 28,
        0x1E00..=0x1EFF => 29,
        0x2C60..=0x2C7F => 29,
        0xA720..=0xA7FF => 29,
        0x1F00..=0x1FFF => 30,
        0x2000..=0x206F => 31,
        0x2E00..=0x2E7F => 31,
        0x2070..=0x209F => 32,
        0x20A0..=0x20CF => 33,
        0x20D0..=0x20FF => 34,
        0x2100..=0x214F => 35,
        0x2150..=0x218F => 36,
        0x2190..=0x21FF => 37,
        0x27F0..=0x27FF => 37,
        0x2900..=0x297F => 37,
        0x2B00..=0x2BFF => 37,
        0x2200..=0x22FF => 38,
        0x2A00..=0x2AFF => 38,
        0x27C0..=0x27EF => 38,
        0x2980..=0x29FF => 38,
        0x2300..=0x23FF => 39,
        0x2400..=0x243F => 40,
        0x2440..=0x245F => 41,
        0x2460..=0x24FF => 42,
        0x2500..=0x257F => 43,
        0x2580..=0x259F => 44,
        0x25A0..=0x25FF => 45,
        0x2600..=0x26FF => 46,
        0x2700..=0x27BF => 47,
        0x3000..=0x303F => 48,
        0x3040..=0x309F => 49,
        0x30A0..=0x30FF => 50,
        0x31F0..=0x31FF => 50,
        0x3100..=0x312F => 51,
        0x31A0..=0x31BF => 51,
        0x3130..=0x318F => 52,
        0xA840..=0xA87F => 53,
        0x3200..=0x32FF => 54,
        0x3300..=0x33FF => 55,
        0xAC00..=0xD7AF => 56,
        // Ignore Non-Plane 0 (57), since this is not a real range.
        0x10900..=0x1091F => 58,
        0x4E00..=0x9FFF => 59,
        0x2E80..=0x2FDF => 59,
        0x2FF0..=0x2FFF => 59,
        0x3400..=0x4DBF => 59,
        0x20000..=0x2A6DF => 59,
        0x3190..=0x319F => 59,
        0xE000..=0xF8FF => 60,
        0x31C0..=0x31EF => 61,
        0xF900..=0xFAFF => 61,
        0x2F800..=0x2FA1F => 61,
        0xFB00..=0xFB4F => 62,
        0xFB50..=0xFDFF => 63,
        0xFE20..=0xFE2F => 64,
        0xFE10..=0xFE1F => 65,
        0xFE30..=0xFE4F => 65,
        0xFE50..=0xFE6F => 66,
        0xFE70..=0xFEFF => 67,
        0xFF00..=0xFFEF => 68,
        0xFFF0..=0xFFFF => 69,
        0x0F00..=0x0FFF => 70,
        0x0700..=0x074F => 71,
        0x0780..=0x07BF => 72,
        0x0D80..=0x0DFF => 73,
        0x1000..=0x109F => 74,
        0x1200..=0x139F => 75,
        0x2D80..=0x2DDF => 75,
        0x13A0..=0x13FF => 76,
        0x1400..=0x167F => 77,
        0x1680..=0x169F => 78,
        0x16A0..=0x16FF => 79,
        0x1780..=0x17FF => 80,
        0x19E0..=0x19FF => 80,
        0x1800..=0x18AF => 81,
        0x2800..=0x28FF => 82,
        0xA000..=0xA48F => 83,
        0xA490..=0xA4CF => 83,
        0x1700..=0x177F => 84,
        0x10300..=0x1032F => 85,
        0x10330..=0x1034F => 86,
        0x10400..=0x1044F => 87,
        0x1D000..=0x1D24F => 88,
        0x1D400..=0x1D7FF => 89,
        0xF0000..=0xFFFFD => 90,
        0x100000..=0x10FFFD => 90,
        0xFE00..=0xFE0F => 91,
        0xE0100..=0xE01EF => 91,
        0xE0000..=0xE007F => 92,
        0x1900..=0x194F => 93,
        0x1950..=0x197F => 94,
        0x1980..=0x19DF => 95,
        0x1A00..=0x1A1F => 96,
        0x2C00..=0x2C5F => 97,
        0x2D30..=0x2D7F => 98,
        0x4DC0..=0x4DFF => 99,
        0xA800..=0xA82F => 100,
        0x10000..=0x1013F => 101,
        0x10140..=0x1018F => 102,
        0x10380..=0x1039F => 103,
        0x103A0..=0x103DF => 104,
        0x10450..=0x1047F => 105,
        0x10480..=0x104AF => 106,
        0x10800..=0x1083F => 107,
        0x10A00..=0x10A5F => 108,
        0x1D300..=0x1D35F => 109,
        0x12000..=0x123FF => 110,
        0x12400..=0x1247F => 110,
        0x1D360..=0x1D37F => 111,
        0x1B80..=0x1BBF => 112,
        0x1C00..=0x1C4F => 113,
        0x1C50..=0x1C7F => 114,
        0xA880..=0xA8DF => 115,
        0xA900..=0xA92F => 116,
        0xA930..=0xA95F => 117,
        0xAA00..=0xAA5F => 118,
        0x10190..=0x101CF => 119,
        0x101D0..=0x101FF => 120,
        0x102A0..=0x102DF => 121,
        0x10280..=0x1029F => 121,
        0x10920..=0x1093F => 121,
        0x1F030..=0x1F09F => 122,
        0x1F000..=0x1F02F => 122,
        _ => -1,
    }
}

/// A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2).
#[derive(Clone, Copy)]
pub struct Table<'a> {
    /// Table version.
    pub version: u8,
    data: &'a [u8],
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let version = s.read::<u16>()?;

        let table_len = match version {
            0 => 78,
            1 => 86,
            2 => 96,
            3 => 96,
            4 => 96,
            5 => 100,
            _ => return None,
        };

        // Do not check the exact length, because some fonts include
        // padding in table's length in table records, which is incorrect.
        if data.len() < table_len {
            return None;
        }

        Some(Table {
            version: version as u8,
            data,
        })
    }

    /// Returns weight class.
    #[inline]
    pub fn weight(&self) -> Weight {
        Weight::from(Stream::read_at::<u16>(self.data, WEIGHT_CLASS_OFFSET).unwrap_or(0))
    }

    /// Returns face width.
    #[inline]
    pub fn width(&self) -> Width {
        match Stream::read_at::<u16>(self.data, WIDTH_CLASS_OFFSET).unwrap_or(0) {
            1 => Width::UltraCondensed,
            2 => Width::ExtraCondensed,
            3 => Width::Condensed,
            4 => Width::SemiCondensed,
            5 => Width::Normal,
            6 => Width::SemiExpanded,
            7 => Width::Expanded,
            8 => Width::ExtraExpanded,
            9 => Width::UltraExpanded,
            _ => Width::Normal,
        }
    }

    /// Returns face permissions.
    ///
    /// Returns `None` in case of a malformed value.
    #[inline]
    pub fn permissions(&self) -> Option<Permissions> {
        let n = Stream::read_at::<u16>(self.data, TYPE_OFFSET).unwrap_or(0);
        if self.version <= 2 {
            // Version 2 and prior, applications are allowed to take
            // the most permissive of provided flags
            let permission = if n & 0xF == 0 {
                Permissions::Installable
            } else if n & 8 != 0 {
                Permissions::Editable
            } else if n & 4 != 0 {
                Permissions::PreviewAndPrint
            } else {
                Permissions::Restricted
            };

            Some(permission)
        } else {
            // Version 3 onwards, flags must be mutually exclusive.
            match n & 0xF {
                0 => Some(Permissions::Installable),
                2 => Some(Permissions::Restricted),
                4 => Some(Permissions::PreviewAndPrint),
                8 => Some(Permissions::Editable),
                _ => None,
            }
        }
    }

    /// Checks if the face allows embedding a subset, further restricted by [`Self::permissions`].
    #[inline]
    pub fn is_subsetting_allowed(&self) -> bool {
        if self.version <= 1 {
            // Flag introduced in version 2
            true
        } else {
            let n = Stream::read_at::<u16>(self.data, TYPE_OFFSET).unwrap_or(0);
            n & 0x0100 == 0
        }
    }

    /// Checks if the face allows outline data to be embedded.
    ///
    /// If false, only bitmaps may be embedded in accordance with [`Self::permissions`].
    ///
    /// If the font contains no bitmaps and this flag is not set, it implies no embedding is allowed.
    #[inline]
    pub fn is_outline_embedding_allowed(&self) -> bool {
        if self.version <= 1 {
            // Flag introduced in version 2
            true
        } else {
            let n = Stream::read_at::<u16>(self.data, TYPE_OFFSET).unwrap_or(0);
            n & 0x0200 == 0
        }
    }

    /// Returns subscript metrics.
    #[inline]
    pub fn subscript_metrics(&self) -> ScriptMetrics {
        let mut s = Stream::new_at(self.data, Y_SUBSCRIPT_X_SIZE_OFFSET).unwrap_or_default();
        ScriptMetrics {
            x_size: s.read::<i16>().unwrap_or(0),
            y_size: s.read::<i16>().unwrap_or(0),
            x_offset: s.read::<i16>().unwrap_or(0),
            y_offset: s.read::<i16>().unwrap_or(0),
        }
    }

    /// Returns superscript metrics.
    #[inline]
    pub fn superscript_metrics(&self) -> ScriptMetrics {
        let mut s = Stream::new_at(self.data, Y_SUPERSCRIPT_X_SIZE_OFFSET).unwrap_or_default();
        ScriptMetrics {
            x_size: s.read::<i16>().unwrap_or(0),
            y_size: s.read::<i16>().unwrap_or(0),
            x_offset: s.read::<i16>().unwrap_or(0),
            y_offset: s.read::<i16>().unwrap_or(0),
        }
    }

    /// Returns strikeout metrics.
    #[inline]
    pub fn strikeout_metrics(&self) -> LineMetrics {
        LineMetrics {
            thickness: Stream::read_at::<i16>(self.data, Y_STRIKEOUT_SIZE_OFFSET).unwrap_or(0),
            position: Stream::read_at::<i16>(self.data, Y_STRIKEOUT_POSITION_OFFSET).unwrap_or(0),
        }
    }

    /// Returns Unicode ranges.
    #[inline]
    pub fn unicode_ranges(&self) -> UnicodeRanges {
        let mut s = Stream::new_at(self.data, UNICODE_RANGES_OFFSET).unwrap();
        let n1 = s.read::<u32>().unwrap_or(0) as u128;
        let n2 = s.read::<u32>().unwrap_or(0) as u128;
        let n3 = s.read::<u32>().unwrap_or(0) as u128;
        let n4 = s.read::<u32>().unwrap_or(0) as u128;
        UnicodeRanges(n4 << 96 | n3 << 64 | n2 << 32 | n1)
    }

    #[inline]
    fn fs_selection(&self) -> u16 {
        Stream::read_at::<u16>(self.data, SELECTION_OFFSET).unwrap_or(0)
    }

    /// Returns style.
    pub fn style(&self) -> Style {
        let flags = SelectionFlags(self.fs_selection());
        if flags.italic() {
            Style::Italic
        } else if self.version >= 4 && flags.oblique() {
            Style::Oblique
        } else {
            Style::Normal
        }
    }

    /// Checks if face is bold.
    ///
    /// Do not confuse with [`Weight::Bold`].
    #[inline]
    pub fn is_bold(&self) -> bool {
        SelectionFlags(self.fs_selection()).bold()
    }

    /// Checks if typographic metrics should be used.
    #[inline]
    pub fn use_typographic_metrics(&self) -> bool {
        if self.version < 4 {
            false
        } else {
            SelectionFlags(self.fs_selection()).use_typo_metrics()
        }
    }

    /// Returns typographic ascender.
    #[inline]
    pub fn typographic_ascender(&self) -> i16 {
        Stream::read_at::<i16>(self.data, TYPO_ASCENDER_OFFSET).unwrap_or(0)
    }

    /// Returns typographic descender.
    #[inline]
    pub fn typographic_descender(&self) -> i16 {
        Stream::read_at::<i16>(self.data, TYPO_DESCENDER_OFFSET).unwrap_or(0)
    }

    /// Returns typographic line gap.
    #[inline]
    pub fn typographic_line_gap(&self) -> i16 {
        Stream::read_at::<i16>(self.data, TYPO_LINE_GAP_OFFSET).unwrap_or(0)
    }

    /// Returns Windows ascender.
    #[inline]
    pub fn windows_ascender(&self) -> i16 {
        Stream::read_at::<i16>(self.data, WIN_ASCENT).unwrap_or(0)
    }

    /// Returns Windows descender.
    #[inline]
    pub fn windows_descender(&self) -> i16 {
        // Should be negated.
        -Stream::read_at::<i16>(self.data, WIN_DESCENT).unwrap_or(0)
    }

    /// Returns x height.
    ///
    /// Returns `None` version is < 2.
    #[inline]
    pub fn x_height(&self) -> Option<i16> {
        if self.version < 2 {
            None
        } else {
            Stream::read_at::<i16>(self.data, X_HEIGHT_OFFSET)
        }
    }

    /// Returns capital height.
    ///
    /// Returns `None` version is < 2.
    #[inline]
    pub fn capital_height(&self) -> Option<i16> {
        if self.version < 2 {
            None
        } else {
            Stream::read_at::<i16>(self.data, CAP_HEIGHT_OFFSET)
        }
    }
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}
