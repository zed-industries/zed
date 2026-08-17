//! PostScript string identifiers.

/// PostScript string identifier (SID).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StringId(u16);

impl StringId {
    /// Creates an identifier from a 16-bit unsigned integer.
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// Returns the underlying identifier as a 16-bit unsigned integer.
    pub const fn to_u16(self) -> u16 {
        self.0
    }

    /// Resolves the identifier as a standard string.
    ///
    /// If the identifier represents a standard string, returns `Ok(string)`,
    /// otherwise returns `Err(index)` with the index that should be used to
    /// retrieve the string from the CFF string INDEX.
    ///
    /// The standard string set is available in the section
    /// "Appendix A - Standard Strings" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf>.
    pub fn standard_string(self) -> Result<Latin1String<'static>, usize> {
        let ix = self.0 as usize;
        if let Some(string) = STANDARD_STRINGS.get(ix) {
            // The standard strings are all ASCII so it's safe to interpret them
            // as Latin-1. This is verified in a unit test.
            Ok(Latin1String::new(string.as_bytes()))
        } else {
            Err(ix - STANDARD_STRINGS.len())
        }
    }
}

impl From<i32> for StringId {
    fn from(value: i32) -> Self {
        Self::new(value as u16)
    }
}

/// Reference to a Latin-1 encoded string.
///
/// Strings stored in all PostScript defined fonts are usually ASCII but are
/// technically encoded in Latin-1. This type wraps the raw string data to
/// prevent attempts to decode as UTF-8.
///
/// This implements `PartialEq<&str>` to support easy comparison with UTF-8
/// strings.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Latin1String<'a> {
    chars: &'a [u8],
}

impl<'a> Latin1String<'a> {
    /// Creates a new Latin-1 encoded string reference from the given bytes,
    /// with each representing a character.
    pub const fn new(chars: &'a [u8]) -> Self {
        Self { chars }
    }

    /// Returns an iterator over the characters of the string.
    ///
    /// This simply converts each byte to `char`.
    pub fn chars(&self) -> impl Iterator<Item = char> + Clone + 'a {
        self.chars.iter().map(|b| *b as char)
    }

    /// Returns the raw bytes of the string.
    pub fn bytes(&self) -> &'a [u8] {
        self.chars
    }
}

impl PartialEq<&str> for Latin1String<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.chars().eq(other.chars())
    }
}

impl std::fmt::Display for Latin1String<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for ch in self.chars() {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

/// The PostScript standard string set.
///
/// See "Appendix A - Standard Strings" in <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf>
pub const STANDARD_STRINGS: &[&str] = &[
    ".notdef",
    "space",
    "exclam",
    "quotedbl",
    "numbersign",
    "dollar",
    "percent",
    "ampersand",
    "quoteright",
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
    "quoteleft",
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
    "exclamdown",
    "cent",
    "sterling",
    "fraction",
    "yen",
    "florin",
    "section",
    "currency",
    "quotesingle",
    "quotedblleft",
    "guillemotleft",
    "guilsinglleft",
    "guilsinglright",
    "fi",
    "fl",
    "endash",
    "dagger",
    "daggerdbl",
    "periodcentered",
    "paragraph",
    "bullet",
    "quotesinglbase",
    "quotedblbase",
    "quotedblright",
    "guillemotright",
    "ellipsis",
    "perthousand",
    "questiondown",
    "grave",
    "acute",
    "circumflex",
    "tilde",
    "macron",
    "breve",
    "dotaccent",
    "dieresis",
    "ring",
    "cedilla",
    "hungarumlaut",
    "ogonek",
    "caron",
    "emdash",
    "AE",
    "ordfeminine",
    "Lslash",
    "Oslash",
    "OE",
    "ordmasculine",
    "ae",
    "dotlessi",
    "lslash",
    "oslash",
    "oe",
    "germandbls",
    "onesuperior",
    "logicalnot",
    "mu",
    "trademark",
    "Eth",
    "onehalf",
    "plusminus",
    "Thorn",
    "onequarter",
    "divide",
    "brokenbar",
    "degree",
    "thorn",
    "threequarters",
    "twosuperior",
    "registered",
    "minus",
    "eth",
    "multiply",
    "threesuperior",
    "copyright",
    "Aacute",
    "Acircumflex",
    "Adieresis",
    "Agrave",
    "Aring",
    "Atilde",
    "Ccedilla",
    "Eacute",
    "Ecircumflex",
    "Edieresis",
    "Egrave",
    "Iacute",
    "Icircumflex",
    "Idieresis",
    "Igrave",
    "Ntilde",
    "Oacute",
    "Ocircumflex",
    "Odieresis",
    "Ograve",
    "Otilde",
    "Scaron",
    "Uacute",
    "Ucircumflex",
    "Udieresis",
    "Ugrave",
    "Yacute",
    "Ydieresis",
    "Zcaron",
    "aacute",
    "acircumflex",
    "adieresis",
    "agrave",
    "aring",
    "atilde",
    "ccedilla",
    "eacute",
    "ecircumflex",
    "edieresis",
    "egrave",
    "iacute",
    "icircumflex",
    "idieresis",
    "igrave",
    "ntilde",
    "oacute",
    "ocircumflex",
    "odieresis",
    "ograve",
    "otilde",
    "scaron",
    "uacute",
    "ucircumflex",
    "udieresis",
    "ugrave",
    "yacute",
    "ydieresis",
    "zcaron",
    "exclamsmall",
    "Hungarumlautsmall",
    "dollaroldstyle",
    "dollarsuperior",
    "ampersandsmall",
    "Acutesmall",
    "parenleftsuperior",
    "parenrightsuperior",
    "twodotenleader",
    "onedotenleader",
    "zerooldstyle",
    "oneoldstyle",
    "twooldstyle",
    "threeoldstyle",
    "fouroldstyle",
    "fiveoldstyle",
    "sixoldstyle",
    "sevenoldstyle",
    "eightoldstyle",
    "nineoldstyle",
    "commasuperior",
    "threequartersemdash",
    "periodsuperior",
    "questionsmall",
    "asuperior",
    "bsuperior",
    "centsuperior",
    "dsuperior",
    "esuperior",
    "isuperior",
    "lsuperior",
    "msuperior",
    "nsuperior",
    "osuperior",
    "rsuperior",
    "ssuperior",
    "tsuperior",
    "ff",
    "ffi",
    "ffl",
    "parenleftinferior",
    "parenrightinferior",
    "Circumflexsmall",
    "hyphensuperior",
    "Gravesmall",
    "Asmall",
    "Bsmall",
    "Csmall",
    "Dsmall",
    "Esmall",
    "Fsmall",
    "Gsmall",
    "Hsmall",
    "Ismall",
    "Jsmall",
    "Ksmall",
    "Lsmall",
    "Msmall",
    "Nsmall",
    "Osmall",
    "Psmall",
    "Qsmall",
    "Rsmall",
    "Ssmall",
    "Tsmall",
    "Usmall",
    "Vsmall",
    "Wsmall",
    "Xsmall",
    "Ysmall",
    "Zsmall",
    "colonmonetary",
    "onefitted",
    "rupiah",
    "Tildesmall",
    "exclamdownsmall",
    "centoldstyle",
    "Lslashsmall",
    "Scaronsmall",
    "Zcaronsmall",
    "Dieresissmall",
    "Brevesmall",
    "Caronsmall",
    "Dotaccentsmall",
    "Macronsmall",
    "figuredash",
    "hypheninferior",
    "Ogoneksmall",
    "Ringsmall",
    "Cedillasmall",
    "questiondownsmall",
    "oneeighth",
    "threeeighths",
    "fiveeighths",
    "seveneighths",
    "onethird",
    "twothirds",
    "zerosuperior",
    "foursuperior",
    "fivesuperior",
    "sixsuperior",
    "sevensuperior",
    "eightsuperior",
    "ninesuperior",
    "zeroinferior",
    "oneinferior",
    "twoinferior",
    "threeinferior",
    "fourinferior",
    "fiveinferior",
    "sixinferior",
    "seveninferior",
    "eightinferior",
    "nineinferior",
    "centinferior",
    "dollarinferior",
    "periodinferior",
    "commainferior",
    "Agravesmall",
    "Aacutesmall",
    "Acircumflexsmall",
    "Atildesmall",
    "Adieresissmall",
    "Aringsmall",
    "AEsmall",
    "Ccedillasmall",
    "Egravesmall",
    "Eacutesmall",
    "Ecircumflexsmall",
    "Edieresissmall",
    "Igravesmall",
    "Iacutesmall",
    "Icircumflexsmall",
    "Idieresissmall",
    "Ethsmall",
    "Ntildesmall",
    "Ogravesmall",
    "Oacutesmall",
    "Ocircumflexsmall",
    "Otildesmall",
    "Odieresissmall",
    "OEsmall",
    "Oslashsmall",
    "Ugravesmall",
    "Uacutesmall",
    "Ucircumflexsmall",
    "Udieresissmall",
    "Yacutesmall",
    "Thornsmall",
    "Ydieresissmall",
    "001.000",
    "001.001",
    "001.002",
    "001.003",
    "Black",
    "Bold",
    "Book",
    "Light",
    "Medium",
    "Regular",
    "Roman",
    "Semibold",
];

#[cfg(test)]
mod tests {
    use super::{Latin1String, StringId, STANDARD_STRINGS};

    #[test]
    fn lets_latin1() {
        let latin1 = Latin1String::new(&[223, 214, 209, 208]);
        let utf8 = "ßÖÑÐ";
        assert_ne!(latin1.chars, utf8.as_bytes());
        assert_eq!(latin1, utf8);
    }

    #[test]
    fn standard_strings() {
        for (i, &std_string) in STANDARD_STRINGS.iter().enumerate() {
            let sid = StringId::new(i as _);
            let latin1 = sid.standard_string().unwrap();
            // Ensure we can compare directly with &str
            assert_eq!(latin1, std_string);
            // Ensure our to_string() conversion works (via the Display impl)
            assert_eq!(latin1.to_string(), std_string);
        }
    }

    #[test]
    fn not_a_standard_string() {
        let sid = StringId::new(STANDARD_STRINGS.len() as _);
        assert!(sid.standard_string().is_err());
        assert_eq!(sid.standard_string().unwrap_err(), 0);
    }
}
