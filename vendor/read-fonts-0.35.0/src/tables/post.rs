//! the [post (PostScript)](https://docs.microsoft.com/en-us/typography/opentype/spec/post#header) table

include!("../../generated/generated_post.rs");

impl<'a> Post<'a> {
    /// The number of glyph names covered by this table
    pub fn num_names(&self) -> usize {
        match self.version() {
            Version16Dot16::VERSION_1_0 => DEFAULT_GLYPH_NAMES.len(),
            Version16Dot16::VERSION_2_0 => self.num_glyphs().unwrap() as usize,
            _ => 0,
        }
    }

    pub fn glyph_name(&self, glyph_id: GlyphId16) -> Option<&str> {
        let glyph_id = glyph_id.to_u16() as usize;
        match self.version() {
            Version16Dot16::VERSION_1_0 => DEFAULT_GLYPH_NAMES.get(glyph_id).copied(),
            Version16Dot16::VERSION_2_0 => {
                let idx = self.glyph_name_index()?.get(glyph_id)?.get() as usize;
                if idx < DEFAULT_GLYPH_NAMES.len() {
                    return DEFAULT_GLYPH_NAMES.get(idx).copied();
                }
                let idx = idx - DEFAULT_GLYPH_NAMES.len();
                match self.string_data().unwrap().get(idx) {
                    Some(Ok(s)) => Some(s.0),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    //FIXME: how do we want to traverse this? I want to stop needing to
    // add special cases for things...
    #[cfg(feature = "experimental_traverse")]
    fn traverse_string_data(&self) -> FieldType<'a> {
        FieldType::I8(-42) // meaningless value
    }
}

/// A string in the post table.
///
/// This is basically just a newtype that knows how to parse from a Pascal-style
/// string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PString<'a>(&'a str);

impl<'a> PString<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl std::ops::Deref for PString<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialEq<&str> for PString<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<'a> FontRead<'a> for PString<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let len: u8 = data.read_at(0)?;
        let pstring = data
            .as_bytes()
            .get(1..len as usize + 1)
            .ok_or(ReadError::OutOfBounds)?;

        if pstring.is_ascii() {
            Ok(PString(std::str::from_utf8(pstring).unwrap()))
        } else {
            //FIXME not really sure how we want to handle this?
            Err(ReadError::MalformedData("Must be valid ascii"))
        }
    }
}

impl VarSize for PString<'_> {
    type Size = u8;
}

/// The 258 glyph names defined for Macintosh TrueType fonts
#[rustfmt::skip]
pub static DEFAULT_GLYPH_NAMES: [&str; 258] = [
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

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::{bebuffer::BeBuffer, post as test_data};

    #[test]
    fn test_post() {
        let table = Post::read(test_data::SIMPLE.into()).unwrap();
        assert_eq!(table.version(), Version16Dot16::VERSION_2_0);
        assert_eq!(table.underline_position(), FWord::new(-75));
        assert_eq!(table.glyph_name(GlyphId16::new(1)), Some(".notdef"));
        assert_eq!(table.glyph_name(GlyphId16::new(2)), Some("space"));
        assert_eq!(table.glyph_name(GlyphId16::new(7)), Some("hello"));
        assert_eq!(table.glyph_name(GlyphId16::new(8)), Some("hi"));
        assert_eq!(table.glyph_name(GlyphId16::new(9)), Some("hola"));
    }

    #[test]
    fn parse_versioned_fields() {
        fn make_basic_post(version: Version16Dot16) -> BeBuffer {
            BeBuffer::new()
                .push(version)
                .push(Fixed::from_i32(5))
                .extend([FWord::new(6), FWord::new(7)]) //underline pos/thickness
                .push(0u32) // isFixedPitch
                .extend([7u32, 8, 9, 10]) // min/max mem x
        }

        // basic table should not parse in v2.0, because that adds another field:
        let buf = make_basic_post(Version16Dot16::VERSION_2_0);
        assert!(Post::read(buf.data().into()).is_err());

        // but it should be fine on version 3.0, which does not require any extra fields:
        let buf = make_basic_post(Version16Dot16::VERSION_3_0);
        assert!(Post::read(buf.data().into()).is_ok());
    }
}
