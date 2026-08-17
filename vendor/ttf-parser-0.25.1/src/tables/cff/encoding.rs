use super::charset::Charset;
use super::StringId;
use crate::parser::{FromData, LazyArray16, Stream};
use crate::GlyphId;

/// The Standard Encoding as defined in the Adobe Technical Note #5176 Appendix B.
#[rustfmt::skip]
pub const STANDARD_ENCODING: [u8; 256] = [
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
      1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,  15,  16,
     17,  18,  19,  20,  21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31,  32,
     33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,  45,  46,  47,  48,
     49,  50,  51,  52,  53,  54,  55,  56,  57,  58,  59,  60,  61,  62,  63,  64,
     65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,
     81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  91,  92,  93,  94,  95,   0,
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
      0,  96,  97,  98,  99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
      0, 111, 112, 113, 114,   0, 115, 116, 117, 118, 119, 120, 121, 122,   0, 123,
      0, 124, 125, 126, 127, 128, 129, 130, 131,   0, 132, 133,   0, 134, 135, 136,
    137,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
      0, 138,   0, 139,   0,   0,   0,   0, 140, 141, 142, 143,   0,   0,   0,   0,
      0, 144,   0,   0,   0, 145,   0,   0, 146, 147, 148, 149,   0,   0,   0,   0,
];

#[derive(Clone, Copy, Debug)]
pub(crate) struct Format1Range {
    first: u8,
    left: u8,
}

impl FromData for Format1Range {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Format1Range {
            first: s.read::<u8>()?,
            left: s.read::<u8>()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Supplement {
    code: u8,
    name: StringId,
}

impl FromData for Supplement {
    const SIZE: usize = 3;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Supplement {
            code: s.read::<u8>()?,
            name: s.read::<StringId>()?,
        })
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Encoding<'a> {
    kind: EncodingKind<'a>,
    supplemental: LazyArray16<'a, Supplement>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum EncodingKind<'a> {
    Standard,
    Expert,
    Format0(LazyArray16<'a, u8>),
    Format1(LazyArray16<'a, Format1Range>),
}

impl Default for EncodingKind<'_> {
    fn default() -> Self {
        Self::Standard
    }
}

impl Encoding<'_> {
    pub fn new_standard() -> Self {
        Encoding {
            kind: EncodingKind::Standard,
            supplemental: LazyArray16::default(),
        }
    }

    pub fn new_expert() -> Self {
        Encoding {
            kind: EncodingKind::Expert,
            supplemental: LazyArray16::default(),
        }
    }

    pub fn code_to_gid(&self, charset: &Charset, code: u8) -> Option<GlyphId> {
        if !self.supplemental.is_empty() {
            if let Some(ref s) = self.supplemental.into_iter().find(|s| s.code == code) {
                return charset.sid_to_gid(s.name);
            }
        }

        let index = usize::from(code);
        match self.kind {
            // Standard encodings store a StringID/SID and not GlyphID/GID.
            // Therefore we have to get SID first and then convert it to GID via Charset.
            // Custom encodings (FormatN) store GID directly.
            //
            // Indexing for predefined encodings never fails,
            // because `code` is always `u8` and encodings have 256 entries.
            //
            // We treat `Expert` as `Standard` as well, since we allow only 8bit codepoints.
            EncodingKind::Standard | EncodingKind::Expert => {
                let sid = StringId(u16::from(STANDARD_ENCODING[index]));
                charset.sid_to_gid(sid)
            }
            EncodingKind::Format0(ref table) => {
                // +1 because .notdef is implicit.
                table
                    .into_iter()
                    .position(|c| c == code)
                    .map(|i| (i + 1) as u16)
                    .map(GlyphId)
            }
            EncodingKind::Format1(ref table) => {
                // Starts from 1 because .notdef is implicit.
                let mut gid: u16 = 1;
                for range in table.into_iter() {
                    let end = range.first.saturating_add(range.left);
                    if (range.first..=end).contains(&code) {
                        gid += u16::from(code - range.first);
                        return Some(GlyphId(gid));
                    } else {
                        gid += u16::from(range.left) + 1;
                    }
                }

                None
            }
        }
    }
}

pub(crate) fn parse_encoding<'a>(s: &mut Stream<'a>) -> Option<Encoding<'a>> {
    let format = s.read::<u8>()?;
    // The first high-bit in format indicates that a Supplemental encoding is present.
    // Check it and clear.
    let has_supplemental = format & 0x80 != 0;
    let format = format & 0x7f;

    let count = u16::from(s.read::<u8>()?);
    let kind = match format {
        // TODO: read_array8?
        0 => s.read_array16::<u8>(count).map(EncodingKind::Format0)?,
        1 => s
            .read_array16::<Format1Range>(count)
            .map(EncodingKind::Format1)?,
        _ => return None,
    };

    let supplemental = if has_supplemental {
        let count = u16::from(s.read::<u8>()?);
        s.read_array16::<Supplement>(count)?
    } else {
        LazyArray16::default()
    };

    Some(Encoding { kind, supplemental })
}
