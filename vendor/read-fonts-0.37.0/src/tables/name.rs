//! The [name (Naming)](https://docs.microsoft.com/en-us/typography/opentype/spec/name) table

include!("../../generated/generated_name.rs");

pub use types::NameId;

impl<'a> Name<'a> {
    /// The FontData containing the encoded name strings.
    pub fn string_data(&self) -> FontData<'a> {
        let base = self.offset_data();
        let off = self.storage_offset();
        base.split_off(off as usize).unwrap_or_default()
    }
}

impl NameRecord {
    /// Return a type that can decode the string data for this name entry.
    pub fn string<'a>(&self, data: FontData<'a>) -> Result<NameString<'a>, ReadError> {
        let start = self.string_offset().non_null().unwrap_or(0);
        let end = start + self.length() as usize;

        let data = data
            .as_bytes()
            .get(start..end)
            .ok_or(ReadError::OutOfBounds)?;

        let encoding = Encoding::new(self.platform_id(), self.encoding_id());
        Ok(NameString { data, encoding })
    }

    // reference from fonttools:
    // https://github.com/fonttools/fonttools/blob/c2119229cfb02cdb7c5a63374ef29d3d514259e8/Lib/fontTools/ttLib/tables/_n_a_m_e.py#L509
    pub fn is_unicode(&self) -> bool {
        self.platform_id() == 0
            || (self.platform_id() == 3 && [0, 1, 10].contains(&self.encoding_id()))
    }
}

impl LangTagRecord {
    /// Return a type that can decode the string data for this name entry.
    pub fn lang_tag<'a>(&self, data: FontData<'a>) -> Result<NameString<'a>, ReadError> {
        let start = self.lang_tag_offset().non_null().unwrap_or(0);
        let end = start + self.length() as usize;

        let data = data
            .as_bytes()
            .get(start..end)
            .ok_or(ReadError::OutOfBounds)?;

        let encoding = Encoding::Utf16Be;
        Ok(NameString { data, encoding })
    }
}

//-- all this is from pinot https://github.com/dfrg/pinot/blob/eff5239018ca50290fb890a84da3dd51505da364/src/name.rs
/// Entry for a name in the naming table.
///
/// This provides an iterator over characters.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NameString<'a> {
    data: &'a [u8],
    encoding: Encoding,
}

impl<'a> NameString<'a> {
    /// An iterator over the `char`s in this name.
    pub fn chars(&self) -> CharIter<'a> {
        CharIter {
            data: self.data,
            encoding: self.encoding,
            pos: 0,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> traversal::SomeString<'a> for NameString<'a> {
    fn iter_chars(&self) -> Box<dyn Iterator<Item = char> + 'a> {
        Box::new(self.into_iter())
    }
}

#[cfg(feature = "experimental_traverse")]
impl NameRecord {
    fn traverse_string<'a>(&self, data: FontData<'a>) -> traversal::FieldType<'a> {
        FieldType::StringOffset(traversal::StringOffset {
            offset: self.string_offset().into(),
            target: self.string(data).map(|s| Box::new(s) as _),
        })
    }
}

#[cfg(feature = "experimental_traverse")]
impl LangTagRecord {
    fn traverse_lang_tag<'a>(&self, data: FontData<'a>) -> traversal::FieldType<'a> {
        FieldType::StringOffset(traversal::StringOffset {
            offset: self.lang_tag_offset().into(),
            target: self.lang_tag(data).map(|s| Box::new(s) as _),
        })
    }
}

impl<'a> IntoIterator for NameString<'a> {
    type Item = char;
    type IntoIter = CharIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

impl std::fmt::Display for NameString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in self.chars() {
            c.fmt(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for NameString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{self}\"")
    }
}

/// An iterator over the chars of a name record.
#[derive(Clone)]
pub struct CharIter<'a> {
    data: &'a [u8],
    encoding: Encoding,
    pos: usize,
}

impl CharIter<'_> {
    fn bump_u16(&mut self) -> Option<u16> {
        let result = self
            .data
            .get(self.pos..self.pos + 2)
            .map(|x| u16::from_be_bytes(x.try_into().unwrap()))?;
        self.pos += 2;
        Some(result)
    }

    fn bump_u8(&mut self) -> Option<u8> {
        let result = self.data.get(self.pos)?;
        self.pos += 1;
        Some(*result)
    }
}

impl Iterator for CharIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let rep = core::char::REPLACEMENT_CHARACTER;
        let raw_c = match self.encoding {
            Encoding::Utf16Be => {
                let c1 = self.bump_u16()? as u32;
                if (0xD800..0xDC00).contains(&c1) {
                    let Some(c2) = self.bump_u16() else {
                        return Some(rep);
                    };
                    ((c1 & 0x3FF) << 10) + (c2 as u32 & 0x3FF) + 0x10000
                } else {
                    c1
                }
            }
            Encoding::MacRoman => {
                let c = self.bump_u8()?;
                MacRomanMapping.decode(c) as u32
            }
            _ => return None,
        };
        Some(std::char::from_u32(raw_c).unwrap_or(rep))
    }
}

/// The encoding used by the name table.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Encoding {
    Utf16Be,
    MacRoman,
    Unknown,
}

impl Encoding {
    /// Determine the coding from the platform and encoding id.
    pub fn new(platform_id: u16, encoding_id: u16) -> Encoding {
        match (platform_id, encoding_id) {
            (0, _) => Encoding::Utf16Be,
            (1, 0) => Encoding::MacRoman,
            (3, 0) => Encoding::Utf16Be,
            (3, 1) => Encoding::Utf16Be,
            (3, 10) => Encoding::Utf16Be,
            _ => Encoding::Unknown,
        }
    }
}

/// A helper for encoding and decoding Mac OS Roman encoded strings.
pub struct MacRomanMapping;

impl MacRomanMapping {
    const START_REMAP: u8 = 128;
    /// Convert from a mac-roman encoded byte to a `char`
    pub fn decode(self, raw: u8) -> char {
        if raw < Self::START_REMAP {
            raw as char
        } else {
            let idx = raw - Self::START_REMAP;
            char::from_u32(MAC_ROMAN_DECODE[idx as usize] as u32).unwrap()
        }
    }

    /// convert from a char to a mac-roman encoded byte, if the char is in the mac-roman charset.
    pub fn encode(self, c: char) -> Option<u8> {
        let raw_c = c as u32;
        let raw_c: u16 = raw_c.try_into().ok()?;
        if raw_c < Self::START_REMAP as u16 {
            Some(raw_c as u8)
        } else {
            match MAC_ROMAN_ENCODE.binary_search_by_key(&raw_c, |(unic, _)| *unic) {
                Ok(idx) => Some(MAC_ROMAN_ENCODE[idx].1),
                Err(_) => None,
            }
        }
    }
}

/// A lookup table for the Mac Roman encoding. This matches the values `128..=255`
/// to specific Unicode values.
#[rustfmt::skip]
static MAC_ROMAN_DECODE: [u16; 128] = [
    196, 197, 199, 201, 209, 214, 220, 225, 224, 226, 228, 227, 229, 231, 233,
    232, 234, 235, 237, 236, 238, 239, 241, 243, 242, 244, 246, 245, 250, 249,
    251, 252, 8224, 176, 162, 163, 167, 8226, 182, 223, 174, 169, 8482, 180,
    168, 8800, 198, 216, 8734, 177, 8804, 8805, 165, 181, 8706, 8721, 8719,
    960, 8747, 170, 186, 937, 230, 248, 191, 161, 172, 8730, 402, 8776, 8710,
    171, 187, 8230, 160, 192, 195, 213, 338, 339, 8211, 8212, 8220, 8221, 8216,
    8217, 247, 9674, 255, 376, 8260, 8364, 8249, 8250, 64257, 64258, 8225, 183,
    8218, 8222, 8240, 194, 202, 193, 203, 200, 205, 206, 207, 204, 211, 212,
    63743, 210, 218, 219, 217, 305, 710, 732, 175, 728, 729, 730, 184, 733,
    731, 711,
];

/// A lookup pairing (sorted) Unicode values to Mac Roman values
#[rustfmt::skip]
static MAC_ROMAN_ENCODE: [(u16, u8); 128] = [
    (160, 202), (161, 193), (162, 162), (163, 163),
    (165, 180), (167, 164), (168, 172), (169, 169),
    (170, 187), (171, 199), (172, 194), (174, 168),
    (175, 248), (176, 161), (177, 177), (180, 171),
    (181, 181), (182, 166), (183, 225), (184, 252),
    (186, 188), (187, 200), (191, 192), (192, 203),
    (193, 231), (194, 229), (195, 204), (196, 128),
    (197, 129), (198, 174), (199, 130), (200, 233),
    (201, 131), (202, 230), (203, 232), (204, 237),
    (205, 234), (206, 235), (207, 236), (209, 132),
    (210, 241), (211, 238), (212, 239), (213, 205),
    (214, 133), (216, 175), (217, 244), (218, 242),
    (219, 243), (220, 134), (223, 167), (224, 136),
    (225, 135), (226, 137), (227, 139), (228, 138),
    (229, 140), (230, 190), (231, 141), (232, 143),
    (233, 142), (234, 144), (235, 145), (236, 147),
    (237, 146), (238, 148), (239, 149), (241, 150),
    (242, 152), (243, 151), (244, 153), (245, 155),
    (246, 154), (247, 214), (248, 191), (249, 157),
    (250, 156), (251, 158), (252, 159), (255, 216),
    (305, 245), (338, 206), (339, 207), (376, 217),
    (402, 196), (710, 246), (711, 255), (728, 249),
    (729, 250), (730, 251), (731, 254), (732, 247),
    (733, 253), (937, 189), (960, 185), (8211, 208),
    (8212, 209), (8216, 212), (8217, 213), (8218, 226),
    (8220, 210), (8221, 211), (8222, 227), (8224, 160),
    (8225, 224), (8226, 165), (8230, 201), (8240, 228),
    (8249, 220), (8250, 221), (8260, 218), (8364, 219),
    (8482, 170), (8706, 182), (8710, 198), (8719, 184),
    (8721, 183), (8730, 195), (8734, 176), (8747, 186),
    (8776, 197), (8800, 173), (8804, 178), (8805, 179),
    (9674, 215), (63743, 240), (64257, 222), (64258, 223),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_roman() {
        static INPUT: &str = "Joachim Müller-Lancé";
        for c in INPUT.chars() {
            let enc = MacRomanMapping.encode(c).unwrap();
            assert_eq!(MacRomanMapping.decode(enc), c);
        }
    }

    #[test]
    fn lone_surrogate_at_end() {
        let chars = CharIter {
            // DEVANAGARI LETTER SHORT A (U+0904), unpaired high surrogate (0xD800)
            data: &[0x09, 0x04, 0xD8, 0x00],
            encoding: Encoding::Utf16Be,
            pos: 0,
        };
        assert!(chars.eq(['ऄ', std::char::REPLACEMENT_CHARACTER].into_iter()))
    }
}
