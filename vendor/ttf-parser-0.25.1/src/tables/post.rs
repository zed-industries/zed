//! A [PostScript Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/post) implementation.

use crate::parser::{Fixed, LazyArray16, Stream};
#[cfg(feature = "glyph-names")]
use crate::GlyphId;
use crate::LineMetrics;

const ITALIC_ANGLE_OFFSET: usize = 4;
const UNDERLINE_POSITION_OFFSET: usize = 8;
const UNDERLINE_THICKNESS_OFFSET: usize = 10;
const IS_FIXED_PITCH_OFFSET: usize = 12;

// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html
/// A list of Macintosh glyph names.
#[cfg(feature = "glyph-names")]
const MACINTOSH_NAMES: &[&str] = &[
    ".notdef",
    ".null",
    "nonmarkingreturn",
    "space",
    "exclam",
    "quotedbl",
    "numbersign",
    "dollar",
    "percent",
    "ampersand",
    "quotesingle",
    "parenleft",
    "parenright",
    "asterisk",
    "plus",
    "comma",
    "hyphen",
    "period",
    "slash",
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "colon",
    "semicolon",
    "less",
    "equal",
    "greater",
    "question",
    "at",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "bracketleft",
    "backslash",
    "bracketright",
    "asciicircum",
    "underscore",
    "grave",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "braceleft",
    "bar",
    "braceright",
    "asciitilde",
    "Adieresis",
    "Aring",
    "Ccedilla",
    "Eacute",
    "Ntilde",
    "Odieresis",
    "Udieresis",
    "aacute",
    "agrave",
    "acircumflex",
    "adieresis",
    "atilde",
    "aring",
    "ccedilla",
    "eacute",
    "egrave",
    "ecircumflex",
    "edieresis",
    "iacute",
    "igrave",
    "icircumflex",
    "idieresis",
    "ntilde",
    "oacute",
    "ograve",
    "ocircumflex",
    "odieresis",
    "otilde",
    "uacute",
    "ugrave",
    "ucircumflex",
    "udieresis",
    "dagger",
    "degree",
    "cent",
    "sterling",
    "section",
    "bullet",
    "paragraph",
    "germandbls",
    "registered",
    "copyright",
    "trademark",
    "acute",
    "dieresis",
    "notequal",
    "AE",
    "Oslash",
    "infinity",
    "plusminus",
    "lessequal",
    "greaterequal",
    "yen",
    "mu",
    "partialdiff",
    "summation",
    "product",
    "pi",
    "integral",
    "ordfeminine",
    "ordmasculine",
    "Omega",
    "ae",
    "oslash",
    "questiondown",
    "exclamdown",
    "logicalnot",
    "radical",
    "florin",
    "approxequal",
    "Delta",
    "guillemotleft",
    "guillemotright",
    "ellipsis",
    "nonbreakingspace",
    "Agrave",
    "Atilde",
    "Otilde",
    "OE",
    "oe",
    "endash",
    "emdash",
    "quotedblleft",
    "quotedblright",
    "quoteleft",
    "quoteright",
    "divide",
    "lozenge",
    "ydieresis",
    "Ydieresis",
    "fraction",
    "currency",
    "guilsinglleft",
    "guilsinglright",
    "fi",
    "fl",
    "daggerdbl",
    "periodcentered",
    "quotesinglbase",
    "quotedblbase",
    "perthousand",
    "Acircumflex",
    "Ecircumflex",
    "Aacute",
    "Edieresis",
    "Egrave",
    "Iacute",
    "Icircumflex",
    "Idieresis",
    "Igrave",
    "Oacute",
    "Ocircumflex",
    "apple",
    "Ograve",
    "Uacute",
    "Ucircumflex",
    "Ugrave",
    "dotlessi",
    "circumflex",
    "tilde",
    "macron",
    "breve",
    "dotaccent",
    "ring",
    "cedilla",
    "hungarumlaut",
    "ogonek",
    "caron",
    "Lslash",
    "lslash",
    "Scaron",
    "scaron",
    "Zcaron",
    "zcaron",
    "brokenbar",
    "Eth",
    "eth",
    "Yacute",
    "yacute",
    "Thorn",
    "thorn",
    "minus",
    "multiply",
    "onesuperior",
    "twosuperior",
    "threesuperior",
    "onehalf",
    "onequarter",
    "threequarters",
    "franc",
    "Gbreve",
    "gbreve",
    "Idotaccent",
    "Scedilla",
    "scedilla",
    "Cacute",
    "cacute",
    "Ccaron",
    "ccaron",
    "dcroat",
];

/// An iterator over glyph names.
///
/// The `post` table doesn't provide the glyph names count,
/// so we have to simply iterate over all of them to find it out.
#[derive(Clone, Copy, Default)]
pub struct Names<'a> {
    data: &'a [u8],
    offset: usize,
}

impl core::fmt::Debug for Names<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Names {{ ... }}")
    }
}

impl<'a> Iterator for Names<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Glyph names are stored as Pascal Strings.
        // Meaning u8 (len) + [u8] (data).

        if self.offset >= self.data.len() {
            return None;
        }

        let len = self.data[self.offset];
        self.offset += 1;

        // An empty name is an error.
        if len == 0 {
            return None;
        }

        let name = self.data.get(self.offset..self.offset + usize::from(len))?;
        self.offset += usize::from(len);
        core::str::from_utf8(name).ok()
    }
}

/// A [PostScript Table](https://docs.microsoft.com/en-us/typography/opentype/spec/post).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// Italic angle in counter-clockwise degrees from the vertical.
    pub italic_angle: f32,
    /// Underline metrics.
    pub underline_metrics: LineMetrics,
    /// Flag that indicates that the font is monospaced.
    pub is_monospaced: bool,
    glyph_indexes: LazyArray16<'a, u16>,
    names_data: &'a [u8],
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        // Do not check the exact length, because some fonts include
        // padding in table's length in table records, which is incorrect.
        if data.len() < 32 {
            return None;
        }

        let version = Stream::new(data).read::<u32>()?;
        if !(version == 0x00010000
            || version == 0x00020000
            || version == 0x00025000
            || version == 0x00030000
            || version == 0x00040000)
        {
            return None;
        }

        let italic_angle = Stream::read_at::<Fixed>(data, ITALIC_ANGLE_OFFSET)?.0;

        let underline_metrics = LineMetrics {
            position: Stream::read_at::<i16>(data, UNDERLINE_POSITION_OFFSET)?,
            thickness: Stream::read_at::<i16>(data, UNDERLINE_THICKNESS_OFFSET)?,
        };

        let is_monospaced = Stream::read_at::<u32>(data, IS_FIXED_PITCH_OFFSET)? != 0;

        let mut names_data: &[u8] = &[];
        let mut glyph_indexes = LazyArray16::default();
        // Only version 2.0 of the table has data at the end.
        if version == 0x00020000 {
            let mut s = Stream::new_at(data, 32)?;
            let indexes_count = s.read::<u16>()?;
            glyph_indexes = s.read_array16::<u16>(indexes_count)?;
            names_data = s.tail()?;
        }

        Some(Table {
            italic_angle,
            underline_metrics,
            is_monospaced,
            names_data,
            glyph_indexes,
        })
    }

    /// Returns a glyph name by ID.
    #[cfg(feature = "glyph-names")]
    pub fn glyph_name(&self, glyph_id: GlyphId) -> Option<&'a str> {
        let mut index = self.glyph_indexes.get(glyph_id.0)?;

        // 'If the name index is between 0 and 257, treat the name index
        // as a glyph index in the Macintosh standard order.'
        if usize::from(index) < MACINTOSH_NAMES.len() {
            Some(MACINTOSH_NAMES[usize::from(index)])
        } else {
            // 'If the name index is between 258 and 65535, then subtract 258 and use that
            // to index into the list of Pascal strings at the end of the table.'
            index -= MACINTOSH_NAMES.len() as u16;
            self.names().nth(usize::from(index))
        }
    }

    /// Returns a glyph ID by a name.
    #[cfg(feature = "glyph-names")]
    pub fn glyph_index_by_name(&self, name: &str) -> Option<GlyphId> {
        let id = if let Some(index) = MACINTOSH_NAMES.iter().position(|n| *n == name) {
            self.glyph_indexes
                .into_iter()
                .position(|i| usize::from(i) == index)?
        } else {
            let mut index = self.names().position(|n| n == name)?;
            index += MACINTOSH_NAMES.len();
            self.glyph_indexes
                .into_iter()
                .position(|i| usize::from(i) == index)?
        };

        Some(GlyphId(id as u16))
    }

    /// Returns an iterator over glyph names.
    ///
    /// Default/predefined names are not included. Just the one in the font file.
    pub fn names(&self) -> Names<'a> {
        Names {
            data: self.names_data,
            offset: 0,
        }
    }
}
