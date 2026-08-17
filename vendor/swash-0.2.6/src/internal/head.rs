//! Font header tables.

use super::{raw_tag, Bytes, RawFont, RawTag};

pub const HEAD: RawTag = raw_tag(b"head");
pub const OS_2: RawTag = raw_tag(b"OS/2");
pub const POST: RawTag = raw_tag(b"post");
pub const MAXP: RawTag = raw_tag(b"maxp");
pub const HHEA: RawTag = raw_tag(b"hhea");
pub const VHEA: RawTag = raw_tag(b"vhea");

/// Font header table.
#[derive(Copy, Clone)]
pub struct Head<'a>(Bytes<'a>);

impl<'a> Head<'a> {
    /// The expected value of the 'magic' field in the header table.
    pub const MAGIC: u32 = 0x5F0F3CF5;

    /// Creates a font header table wrapping the specified data.
    pub fn new(data: &'a [u8]) -> Self {
        Self(Bytes::new(data))
    }

    /// Creates a font header table from the specified font.
    /// Returns `None` if the font does not contain a `head` table.    
    pub fn from_font(font: impl RawFont<'a>) -> Option<Self> {
        Some(Self::new(font.table_data(HEAD)?))
    }

    /// Returns the underlying bytes of the table.
    pub fn data(&self) -> &'a [u8] {
        self.0.data()
    }

    /// Returns the major version of the header table. Set to 1.
    pub fn major_version(&self) -> u16 {
        self.0.read(0).unwrap_or(0)
    }

    /// Returns the minor version of the header table. Set to 0.
    pub fn minor_version(&self) -> u16 {
        self.0.read(2).unwrap_or(0)
    }

    /// Returns a revision value. Set by font manufacturer.
    pub fn revision(&self) -> u32 {
        self.0.read(4).unwrap_or(0)
    }

    /// Returns a checksum adjustment value.
    pub fn checksum_adjustment(&self) -> u32 {
        self.0.read(8).unwrap_or(0)
    }

    /// Returns a magic number for validation. Set to 0x5F0F3CF5.
    pub fn magic(&self) -> u32 {
        self.0.read(12).unwrap_or(0)
    }

    /// Returns a set of header bit flags.
    /// - 0: Baseline at y = 0
    /// - 1: Left sidebearing at x = 0
    /// - 2: Instructions may depend on point size
    /// - 3: Force ppem to integer values
    /// - 4: Instructions may alter advance width
    /// - 5-10: Unused
    /// - 11: Font data is lossless
    /// - 12: Font has been converted
    /// - 13: Optimized for ClearType
    /// - 14: Last resort font
    pub fn flags(&self) -> u16 {
        self.0.read(16).unwrap_or(0)
    }

    /// Returns the design units per em. Valid values are 16..=16384.
    pub fn units_per_em(&self) -> u16 {
        self.0.read(18).unwrap_or(0)
    }

    /// Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone.
    pub fn created(&self) -> u64 {
        self.0.read(20).unwrap_or(0)
    }

    /// Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone.
    pub fn modified(&self) -> u64 {
        self.0.read(28).unwrap_or(0)
    }

    /// Returns the union of all glyph bounding boxes.
    pub fn bounds(&self) -> [(i16, i16); 2] {
        [
            (
                self.0.read_or_default::<i16>(36),
                self.0.read_or_default::<i16>(38),
            ),
            (
                self.0.read_or_default::<i16>(40),
                self.0.read_or_default::<i16>(42),
            ),
        ]
    }

    /// Returns the mac style bit flags.
    /// - 0: Bold
    /// - 1: Italic
    /// - 2: Underline
    /// - 3: Outline
    /// - 4: Shadow
    /// - 5: Condensed
    /// - 6: Extended
    /// - 7-15: Reserved
    pub fn mac_style(&self) -> u16 {
        self.0.read(44).unwrap_or(0)
    }

    /// Returns the smallest readable size in pixels.
    pub fn lowest_recommended_ppem(&self) -> u16 {
        self.0.read(46).unwrap_or(0)
    }

    /// Deprecated. Returns a hint about the directionality of the glyphs.
    /// Set to 2.
    pub fn direction_hint(&self) -> u16 {
        self.0.read(48).unwrap_or(0)
    }

    /// Returns the format the the offset array in the 'loca' table.
    /// - 0: 16-bit offsets (divided by 2)
    /// - 1: 32-bit offsets
    pub fn index_to_location_format(&self) -> u16 {
        self.0.read(50).unwrap_or(0)
    }

    /// Unused. Set to 0.
    pub fn glyph_data_format(&self) -> i16 {
        self.0.read(52).unwrap_or(0)
    }
}

/// OS/2 and Windows metrics table.
#[derive(Copy, Clone)]
pub struct Os2<'a>(Bytes<'a>);

impl<'a> Os2<'a> {
    /// Creates an OS/2 table wrapping the specified data.
    pub fn new(data: &'a [u8]) -> Self {
        Self(Bytes::new(data))
    }

    /// Creates an OS/2 table from the specified font.
    /// Returns `None` if the font does not contain an `OS/2` table.    
    pub fn from_font(font: impl RawFont<'a>) -> Option<Self> {
        Some(Self::new(font.table_data(OS_2)?))
    }

    /// Returns the underlying bytes of the table.
    pub fn data(&self) -> &'a [u8] {
        self.0.data()
    }

    /// Returns the version number for the OS/2 table: 0x0000 to 0x0005.
    pub fn version(&self) -> u16 {
        self.0.read(0).unwrap_or(0)
    }

    /// Returns the average advance width of all non-zero width glyphs in the
    /// font.
    pub fn average_char_width(&self) -> i16 {
        self.0.read(2).unwrap_or(0)
    }

    /// Returns the visual weight class on a scale from 1 to 1000.  
    /// Common values:
    /// - 100: Thin
    /// - 200: Extra-light (Ultra-light)
    /// - 300: Light
    /// - 400: Normal (Regular)
    /// - 500: Medium
    /// - 600: Semi-bold
    /// - 700: Bold
    /// - 800: Extra-bold (Ultra-bold)
    /// - 900: Black (Heavy)
    pub fn weight_class(&self) -> i16 {
        self.0.read(4).unwrap_or(0)
    }

    /// Returns the visual width class-- a relative change from the normal aspect
    /// ratio.
    /// - 1: Ultra-condensed
    /// - 2: Extra-condensed
    /// - 3: Condensed
    /// - 4: Semi-condensed
    /// - 5: Medium (Normal)
    /// - 6: Semi-expanded
    /// - 7: Expanded
    /// - 8: Extra-expanded
    /// - 9: Ultra-expanded
    pub fn width_class(&self) -> i16 {
        self.0.read(6).unwrap_or(0)
    }

    /// Returns the font type bit flags.  
    /// Bits:
    /// - 0-3: Usage permissions
    /// - 4-7: Reserved (set to 0)
    /// - 8: No subsetting
    /// - 9: Bitmap embedding only
    /// - 10-15: Reserved (set to 0)
    pub fn type_flags(&self) -> i16 {
        self.0.read(8).unwrap_or(0)
    }

    /// Returns a rectangle describing suggested subscript positioning.
    pub fn subscript(&self) -> [(i32, i32); 2] {
        [
            (
                self.0.read::<i16>(14).unwrap_or(0) as i32,
                self.0.read::<i16>(16).unwrap_or(0) as i32,
            ),
            (
                self.0.read::<i16>(10).unwrap_or(0) as i32,
                self.0.read::<i16>(12).unwrap_or(0) as i32,
            ),
        ]
    }

    /// Returns a rectangle describing suggested superscript positioning.    
    pub fn superscript(&self) -> [(i32, i32); 2] {
        [
            (
                self.0.read::<i16>(22).unwrap_or(0) as i32,
                self.0.read::<i16>(24).unwrap_or(0) as i32,
            ),
            (
                self.0.read::<i16>(18).unwrap_or(0) as i32,
                self.0.read::<i16>(20).unwrap_or(0) as i32,
            ),
        ]
    }

    /// Returns the suggested position of the top of the strikeout stroke from
    /// the baseline.
    pub fn strikeout_position(&self) -> i16 {
        self.0.read(28).unwrap_or(0)
    }

    /// Returns the suggested thickness for the strikeout stroke.
    pub fn strikeout_size(&self) -> i16 {
        self.0.read(26).unwrap_or(0)
    }

    /// Returns the font family class and subclass. For values:
    /// <https://docs.microsoft.com/en-us/typography/opentype/spec/ibmfc>
    pub fn family_class(&self) -> i16 {
        self.0.read(30).unwrap_or(0)
    }

    /// Returns a 10-byte PANOSE classification number.
    /// <https://monotype.github.io/panose/>
    pub fn panose(&self) -> &'a [u8] {
        self.0
            .read_bytes(32, 10)
            .unwrap_or(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    /// Returns a 128-bit value describing the Unicode blocks that are
    /// supported by the font.
    pub fn unicode_range(&self) -> (u32, u32, u32, u32) {
        (
            self.0.read::<u32>(42).unwrap_or(0),
            self.0.read::<u32>(46).unwrap_or(0),
            self.0.read::<u32>(50).unwrap_or(0),
            self.0.read::<u32>(54).unwrap_or(0),
        )
    }

    /// Returns a four character font vendor identifier.
    pub fn vendor_id(&self) -> &'a str {
        core::str::from_utf8(self.0.read_bytes(58, 4).unwrap_or(b"none")).unwrap_or("none")
    }

    /// Returns the font selection bit flags.  
    /// Bits:
    /// - 0: Italic
    /// - 1: Underscore
    /// - 2: Negative
    /// - 3: Outlined
    /// - 4: Strikeout
    /// - 5: Bold
    /// - 6: Regular
    /// - 7: Use typographic metrics
    /// - 8: WWS (Weight/Width/Slope names)
    /// - 9: Oblique
    /// - 10-15: Reserved (set to 0)
    pub fn selection_flags(&self) -> Flags {
        Flags(self.0.read(62).unwrap_or(0))
    }

    /// Returns the minimum and maximum Unicode codepoints supported by the
    /// font. Note that this does not cover supplementary characters.
    pub fn char_range(&self) -> (u16, u16) {
        (
            self.0.read::<u16>(64).unwrap_or(0),
            self.0.read::<u16>(66).unwrap_or(0),
        )
    }

    /// Returns the typographic ascender.
    pub fn typographic_ascender(&self) -> i16 {
        self.0.read(68).unwrap_or(0)
    }

    /// Returns the typographic descender.
    pub fn typographic_descender(&self) -> i16 {
        self.0.read(70).unwrap_or(0)
    }

    /// Returns the typographic line gap.
    pub fn typographic_line_gap(&self) -> i16 {
        self.0.read(72).unwrap_or(0)
    }

    /// Returns a Windows specific value that defines the upper extent of
    /// the clipping region.
    pub fn win_ascent(&self) -> u16 {
        self.0.read(74).unwrap_or(0)
    }

    /// Returns a Windows specific value that defines the lower extent of
    /// the clipping region.
    pub fn win_descent(&self) -> u16 {
        self.0.read(76).unwrap_or(0)
    }

    /// Returns Windows specific code page ranges supported by the font.
    /// (table version >= 1)
    pub fn code_page_range(&self) -> (u32, u32) {
        if self.version() < 1 {
            return (0, 0);
        }
        (
            self.0.read::<u32>(78).unwrap_or(0),
            self.0.read::<u32>(82).unwrap_or(0),
        )
    }

    /// Returns the approximate distance above the baseline for non-descending
    /// lowercase letters (table version >= 2)
    pub fn x_height(&self) -> i16 {
        if self.version() < 2 {
            return 0;
        }
        self.0.read(86).unwrap_or(0)
    }

    /// Returns the approximate distance above the baseline for uppercase letters.
    /// (table version >= 2)
    pub fn cap_height(&self) -> i16 {
        if self.version() < 2 {
            return 0;
        }
        self.0.read(88).unwrap_or(0)
    }

    /// Returns a Unicode codepoint for the default character to use if
    /// a requested character is not supported by the font.
    /// (table version >= 2)
    pub fn default_char(&self) -> u16 {
        if self.version() < 2 {
            return 0;
        }
        self.0.read(90).unwrap_or(0)
    }

    /// Returns a Unicode codepoint for the default character used to separate
    /// words and justify text. (table version >= 2)
    pub fn break_char(&self) -> u16 {
        if self.version() < 2 {
            return 0;
        }
        self.0.read(92).unwrap_or(0)
    }

    /// Returns the maximum length of a target glyph context for any feature in
    /// the font. (table version >= 2)
    pub fn max_context(&self) -> u16 {
        if self.version() < 2 {
            return 0;
        }
        self.0.read(94).unwrap_or(0)
    }

    /// Returns the lower value of the size range for which this font has been
    /// designed. The units are TWIPS (1/20 points). (table version >= 5)
    pub fn lower_optical_point_size(&self) -> u16 {
        if self.version() < 5 {
            return 0;
        }
        self.0.read(96).unwrap_or(0)
    }

    /// Returns the upper value of the size range for which this font has been
    /// designed. The units are TWIPS (1/20 points). (table version >= 5)
    pub fn upper_optical_point_size(&self) -> u16 {
        if self.version() < 5 {
            return 0;
        }
        self.0.read(98).unwrap_or(0)
    }
}

/// OS/2 selection flags.
#[derive(Copy, Clone)]
pub struct Flags(pub u16);

impl Flags {
    /// Font contains italic glyphs.
    pub fn italic(self) -> bool {
        self.bit(0)
    }

    /// Glyphs are underscored.
    pub fn underscore(self) -> bool {
        self.bit(1)
    }

    /// Glyphs have their foreground and background reversed.
    pub fn negative(self) -> bool {
        self.bit(2)
    }

    /// Hollow glyphs.
    pub fn outlined(self) -> bool {
        self.bit(3)
    }

    /// Glyphs are overstruck.
    pub fn strikeout(self) -> bool {
        self.bit(4)
    }

    /// Glyphs are emboldened.
    pub fn bold(self) -> bool {
        self.bit(5)
    }

    /// Glyphs are in the standard weight/style for the font.
    pub fn regular(self) -> bool {
        self.bit(6)
    }

    /// Typographic metrics are recommended for default line spacing.
    pub fn use_typographic_metrics(self) -> bool {
        self.bit(7)
    }

    /// Font has name table strings consistent with WWS family naming.
    pub fn wws_names(self) -> bool {
        self.bit(8)
    }

    /// Font contains oblique glyphs.
    pub fn oblique(self) -> bool {
        self.bit(9)
    }

    fn bit(self, bit: u16) -> bool {
        self.0 & 1 << bit != 0
    }
}

/// PostScript table.
#[derive(Copy, Clone)]
pub struct Post<'a>(Bytes<'a>);

impl<'a> Post<'a> {
    /// Creates a PostScript table wrapping the specified data.
    pub fn new(data: &'a [u8]) -> Self {
        Self(Bytes::new(data))
    }

    /// Creates a PostScript table from the specified font.
    /// Returns `None` if the font does not contain a `post` table.    
    pub fn from_font(font: impl RawFont<'a>) -> Option<Self> {
        Some(Self::new(font.table_data(POST)?))
    }

    /// Returns the underlying bytes of the table.
    pub fn data(&self) -> &'a [u8] {
        self.0.data()
    }

    /// Returns the version of the PostScript table.
    pub fn version(&self) -> u32 {
        self.0.read(0).unwrap_or(0)
    }

    /// Returns the italic angle in counter-clockwise degrees from the vertical.
    pub fn italic_angle(&self) -> u32 {
        self.0.read(4).unwrap_or(0)
    }

    /// Returns the suggested position of the top of the underline stroke from
    /// the baseline.
    pub fn underline_position(&self) -> i16 {
        self.0.read(8).unwrap_or(0)
    }

    /// Returns the suggested thickness for the underline stroke.
    pub fn underline_size(&self) -> i16 {
        self.0.read(10).unwrap_or(0)
    }

    /// Returns true if the font is not proportionally spaced (i.e. monospaced).
    pub fn is_fixed_pitch(&self) -> bool {
        self.0.read::<u32>(12).unwrap_or(0) != 0
    }

    /// Returns true if the table can provide glyph names. Only versions 1.0
    /// (0x00010000) and 2.0 (0x00020000).
    pub fn has_names(&self) -> bool {
        let v = self.version();
        v == 0x10000 || v == 0x20000
    }

    /// Returns the name of the specified glyph id if available.
    pub fn name(&self, glyph_id: u16) -> Option<&'a str> {
        if !self.has_names() {
            return None;
        }
        let v = self.version();
        if v == 0x10000 {
            if glyph_id >= 258 {
                return None;
            }
            return Some(DEFAULT_GLYPH_NAMES[glyph_id as usize]);
        } else if v == 0x20000 {
            let b = &self.0;
            let count = b.read::<u16>(32)?;
            if glyph_id >= count {
                return None;
            }
            let mut index = b.read::<u16>(34 + glyph_id as usize * 2)? as usize;
            if index < 258 {
                return Some(DEFAULT_GLYPH_NAMES[index]);
            }
            index -= 258;
            let mut base = 34 + count as usize * 2;
            for _ in 0..index {
                let len = b.read::<u8>(base)? as usize;
                base += len + 1;
            }
            let len = b.read::<u8>(base)? as usize;
            base += 1;
            let bytes = b.read_bytes(base, len)?;
            return core::str::from_utf8(bytes).ok();
        }
        None
    }
}

/// Maximum profile table.
#[derive(Copy, Clone)]
pub struct Maxp<'a>(Bytes<'a>);

impl<'a> Maxp<'a> {
    /// Creates a maximum profile table wrapping the specified data.
    pub fn new(data: &'a [u8]) -> Self {
        Self(Bytes::new(data))
    }

    /// Creates a maximum profile table from the specified font.
    /// Returns `None` if the font does not contain a `maxp` table.    
    pub fn from_font(font: impl RawFont<'a>) -> Option<Self> {
        Some(Self::new(font.table_data(MAXP)?))
    }

    /// Returns the underlying bytes of the table.
    pub fn data(&self) -> &'a [u8] {
        self.0.data()
    }

    /// Returns the version of the table.
    /// - 0x00005000: Version 0.5 - only `num_glyphs` will return a meaningful value.
    /// - 0x00010000: Version 1.0
    pub fn version(&self) -> u32 {
        self.0.read(0).unwrap_or(0)
    }

    /// Returns the number of glyphs in the font.
    pub fn glyph_count(&self) -> u16 {
        self.0.read(4).unwrap_or(0)
    }

    /// Returns true if the 'max_' methods will return meaningful values--
    /// specifically, if the table version is 1.0 (0x00010000).
    pub fn is_truetype(&self) -> bool {
        self.version() == 0x00010000
    }

    /// Returns the maximum points in a simple glyph.
    pub fn max_points(&self) -> u16 {
        self.0.read(6).unwrap_or(0)
    }

    /// Returns the maximum contours in a simple glyph.
    pub fn max_contours(&self) -> u16 {
        self.0.read(8).unwrap_or(0)
    }

    /// Returns the maximum points in a composite glyph.
    pub fn max_composite_points(&self) -> u16 {
        self.0.read(10).unwrap_or(0)
    }

    /// Returns the maximum contours in a composite glyph.
    pub fn max_composite_contours(&self) -> u16 {
        self.0.read(12).unwrap_or(0)
    }

    /// Returns 2 if instructions require a 'twilight zone' or 1 otherwise.
    pub fn max_zones(&self) -> u16 {
        self.0.read(14).unwrap_or(0)
    }

    /// Returns the maximum twilight points used in zone 0.
    pub fn max_twilight_points(&self) -> u16 {
        self.0.read(16).unwrap_or(0)
    }

    /// Returns the maximum storage area locations.
    pub fn max_storage(&self) -> u16 {
        self.0.read(18).unwrap_or(0)
    }

    /// Returns the maximum function definitions.
    pub fn max_function_definitions(&self) -> u16 {
        self.0.read(20).unwrap_or(0)
    }

    /// Returns the maximum instruction definitions.
    pub fn max_instruction_definitions(&self) -> u16 {
        self.0.read(22).unwrap_or(0)
    }

    /// Returns the maximum stack depth across all programs in the font.
    pub fn max_stack_depth(&self) -> u16 {
        self.0.read(24).unwrap_or(0)
    }

    /// Returns the maximum size of glyph instructions.
    pub fn max_instructions_size(&self) -> u16 {
        self.0.read(26).unwrap_or(0)
    }

    /// Returns the maximum number of components for a single composite glyph.
    pub fn max_component_elements(&self) -> u16 {
        self.0.read(28).unwrap_or(0)
    }

    /// Returns the maximum nesting level for any composite glyph.
    pub fn max_component_depth(&self) -> u16 {
        self.0.read(30).unwrap_or(0)
    }
}

/// Horizontal header table.
#[derive(Copy, Clone)]
pub struct Hhea<'a>(Bytes<'a>);

impl<'a> Hhea<'a> {
    /// Creates a horizontal header table wrapping the specified data.
    pub fn new(data: &'a [u8]) -> Self {
        Self(Bytes::new(data))
    }

    /// Creates a horizontal header table from the specified font.
    /// Returns `None` if the font does not contain an `hhea` table.    
    pub fn from_font(font: impl RawFont<'a>) -> Option<Self> {
        Some(Self::new(font.table_data(HHEA)?))
    }

    /// Returns the underlying bytes of the table.
    pub fn data(&self) -> &'a [u8] {
        self.0.data()
    }

    /// Returns the major version of the horizontal header table. Set to 1.
    pub fn major_version(&self) -> u16 {
        self.0.read(0).unwrap_or(0)
    }

    /// Returns the minor version of the horizontal header table. Set to 0.
    pub fn minor_version(&self) -> u16 {
        self.0.read(2).unwrap_or(0)
    }

    /// Returns the typographic ascender.
    pub fn ascender(&self) -> i16 {
        self.0.read(4).unwrap_or(0)
    }

    /// Returns the typographic descender.
    pub fn descender(&self) -> i16 {
        self.0.read(6).unwrap_or(0)
    }

    /// Returns the typographic line gap.
    pub fn line_gap(&self) -> i16 {
        self.0.read(8).unwrap_or(0)
    }

    /// Returns the maximum advance width.
    pub fn max_advance(&self) -> u16 {
        self.0.read(10).unwrap_or(0)
    }

    /// Returns the minimum left sidebearing.
    pub fn min_lsb(&self) -> i16 {
        self.0.read(12).unwrap_or(0)
    }

    /// Returns the minimum right sidebearing. (min(advance - lsb - (xmax - xmin)))
    pub fn min_rsb(&self) -> i16 {
        self.0.read(14).unwrap_or(0)
    }

    /// Returns the maximum horizontal extent. (max(lsb + (xmax - xmin)))
    pub fn max_extent(&self) -> i16 {
        self.0.read(16).unwrap_or(0)
    }

    /// Returns the slope of the cursor in the form (rise, run).
    pub fn caret_slope(&self) -> (i16, i16) {
        (self.0.read(18).unwrap_or(0), self.0.read(20).unwrap_or(0))
    }

    /// Returns the amount by which a slanted highlight on a glyph should be
    /// shifted.
    pub fn caret_offset(&self) -> i16 {
        self.0.read(22).unwrap_or(0)
    }

    /// Unused in current format. Set to 0.
    pub fn metric_data_format(&self) -> i16 {
        self.0.read(32).unwrap_or(0)
    }

    /// Returns the number of "long" metric entries in the horizontal metrics
    /// table.
    pub fn num_long_metrics(&self) -> u16 {
        self.0.read(34).unwrap_or(0)
    }
}

/// Vertical header table.
#[derive(Copy, Clone)]
pub struct Vhea<'a>(Bytes<'a>);

impl<'a> Vhea<'a> {
    /// Creates a vertical header table wrapping the specified data.
    pub fn new(data: &'a [u8]) -> Self {
        Self(Bytes::new(data))
    }

    /// Creates a vertical header table from the specified font.
    /// Returns `None` if the font does not contain a `vhea` table.    
    pub fn from_font(font: impl RawFont<'a>) -> Option<Self> {
        Some(Self::new(font.table_data(VHEA)?))
    }

    /// Returns the underlying bytes of the table.
    pub fn data(&self) -> &'a [u8] {
        self.0.data()
    }

    /// Returns the major version of the vertical header table. Set to 1.
    pub fn major_version(&self) -> u16 {
        self.0.read(0).unwrap_or(0)
    }

    /// Returns the minor version of the vertical header table. Set to 0.
    pub fn minor_version(&self) -> u16 {
        self.0.read(2).unwrap_or(0)
    }

    /// Returns the distance in design units from the centerline to the
    /// previous line's descent.
    pub fn ascender(&self) -> i16 {
        self.0.read(4).unwrap_or(0)
    }

    /// Returns the distance in design units from the centerline to the next
    /// line's ascent.
    pub fn descender(&self) -> i16 {
        self.0.read(6).unwrap_or(0)
    }

    /// Recommended additional spacing between columns of vertical text.
    pub fn line_gap(&self) -> i16 {
        self.0.read(8).unwrap_or(0)
    }

    /// Returns the maximum advance height.
    pub fn max_advance(&self) -> u16 {
        self.0.read(10).unwrap_or(0)
    }

    /// Returns the minimum top sidebearing.
    pub fn min_tsb(&self) -> i16 {
        self.0.read(12).unwrap_or(0)
    }

    /// Returns the minimum bottom sidebearing.
    pub fn min_bsb(&self) -> i16 {
        self.0.read(14).unwrap_or(0)
    }

    /// Returns the maximum vertical extent. (max(tsb + (ymax - ymin)))
    pub fn max_extent(&self) -> i16 {
        self.0.read(16).unwrap_or(0)
    }

    /// Returns the slope of the cursor in the form (rise, run).
    pub fn caret_slope(&self) -> (i16, i16) {
        (self.0.read(18).unwrap_or(0), self.0.read(20).unwrap_or(0))
    }

    /// Returns the amount by which a slanted highlight on a glyph should be
    /// shifted.
    pub fn caret_offset(&self) -> i16 {
        self.0.read(22).unwrap_or(0)
    }

    /// Unused in current format. Set to 0.
    pub fn metric_data_format(&self) -> i16 {
        self.0.read(32).unwrap_or(0)
    }

    /// Returns the number of "long" metric entries in the vertical metrics
    /// table.
    pub fn num_long_metrics(&self) -> u16 {
        self.0.read(34).unwrap_or(0)
    }
}

#[rustfmt::skip]
const DEFAULT_GLYPH_NAMES: [&'static str; 258] = [
    ".notdef", ".null", "nonmarkingreturn", "space", "exclam", "quotedbl", "numbersign", "dollar", 
    "percent", "ampersand", "quotesingle", "parenleft", "parenright", "asterisk", "plus", "comma", 
    "hyphen", "period", "slash", "zero", "one", "two", "three", "four", "five", "six", "seven", 
    "eight", "nine", "colon", "semicolon", "less", "equal", "greater", "question", "at", "A", "B", 
    "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", 
    "V", "W", "X", "Y", "Z", "bracketleft", "backslash", "bracketright", "asciicircum", 
    "underscore", "grave", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", 
    "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "braceleft", "bar", "braceright", 
    "asciitilde", "Adieresis", "Aring", "Ccedilla", "Eacute", "Ntilde", "Odieresis", "Udieresis", 
    "aacute", "agrave", "acircumflex", "adieresis", "atilde", "aring", "ccedilla", "eacute", 
    "egrave", "ecircumflex", "edieresis", "iacute", "igrave", "icircumflex", "idieresis", "ntilde", 
    "oacute", "ograve", "ocircumflex", "odieresis", "otilde", "uacute", "ugrave", "ucircumflex", 
    "udieresis", "dagger", "degree", "cent", "sterling", "section", "bullet", "paragraph", 
    "germandbls", "registered", "copyright", "trademark", "acute", "dieresis", "notequal", "AE", 
    "Oslash", "infinity", "plusminus", "lessequal", "greaterequal", "yen", "mu", "partialdiff", 
    "summation", "product", "pi", "integral", "ordfeminine", "ordmasculine", "Omega", "ae", 
    "oslash", "questiondown", "exclamdown", "logicalnot", "radical", "florin", "approxequal", 
    "Delta", "guillemotleft", "guillemotright", "ellipsis", "nonbreakingspace", "Agrave", "Atilde", 
    "Otilde", "OE", "oe", "endash", "emdash", "quotedblleft", "quotedblright", "quoteleft", 
    "quoteright", "divide", "lozenge", "ydieresis", "Ydieresis", "fraction", "currency", 
    "guilsinglleft", "guilsinglright", "fi", "fl", "daggerdbl", "periodcentered", "quotesinglbase", 
    "quotedblbase", "perthousand", "Acircumflex", "Ecircumflex", "Aacute", "Edieresis", "Egrave", 
    "Iacute", "Icircumflex", "Idieresis", "Igrave", "Oacute", "Ocircumflex", "apple", "Ograve", 
    "Uacute", "Ucircumflex", "Ugrave", "dotlessi", "circumflex", "tilde", "macron", "breve", 
    "dotaccent", "ring", "cedilla", "hungarumlaut", "ogonek", "caron", "Lslash", "lslash", 
    "Scaron", "scaron", "Zcaron", "zcaron", "brokenbar", "Eth", "eth", "Yacute", "yacute", "Thorn", 
    "thorn", "minus", "multiply", "onesuperior", "twosuperior", "threesuperior", "onehalf", 
    "onequarter", "threequarters", "franc", "Gbreve", "gbreve", "Idotaccent", "Scedilla", 
    "scedilla", "Cacute", "cacute", "Ccaron", "ccaron", "dcroat",     
];
