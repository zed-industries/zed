//! CFF charset support.

use super::{
    CharsetFormat0, CharsetFormat1, CharsetFormat2, CharsetRange1, CharsetRange2, CustomCharset,
    FontData, FontRead, GlyphId, ReadError, StringId,
};

/// Character set for mapping from glyph to string identifiers.
///
/// See <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=21>
#[derive(Clone)]
pub struct Charset<'a> {
    kind: CharsetKind<'a>,
    num_glyphs: u32,
}

impl<'a> Charset<'a> {
    pub fn new(
        cff_data: FontData<'a>,
        charset_offset: usize,
        num_glyphs: u32,
    ) -> Result<Self, ReadError> {
        let kind = match charset_offset {
            0 => CharsetKind::IsoAdobe,
            1 => CharsetKind::Expert,
            2 => CharsetKind::ExpertSubset,
            _ => {
                let data = cff_data
                    .split_off(charset_offset)
                    .ok_or(ReadError::OutOfBounds)?;
                CharsetKind::Custom(CustomCharset::read(data)?)
            }
        };
        Ok(Self { kind, num_glyphs })
    }

    pub fn kind(&self) -> &CharsetKind<'a> {
        &self.kind
    }

    pub fn num_glyphs(&self) -> u32 {
        self.num_glyphs
    }

    /// Returns the string identifier for the given glyph identifier.
    pub fn string_id(&self, glyph_id: GlyphId) -> Result<StringId, ReadError> {
        let gid = glyph_id.to_u32();
        if gid >= self.num_glyphs {
            return Err(ReadError::OutOfBounds);
        }
        match &self.kind {
            CharsetKind::IsoAdobe => {
                // The ISOAdobe charset is an identity mapping of gid->sid up
                // to 228 entries
                // <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=45>
                if gid <= 228 {
                    Ok(StringId::new(gid as u16))
                } else {
                    Err(ReadError::OutOfBounds)
                }
            }
            CharsetKind::Expert => EXPERT_CHARSET
                .get(gid as usize)
                .copied()
                .ok_or(ReadError::OutOfBounds)
                .map(StringId::new),
            CharsetKind::ExpertSubset => EXPERT_SUBSET_CHARSET
                .get(gid as usize)
                .copied()
                .ok_or(ReadError::OutOfBounds)
                .map(StringId::new),
            CharsetKind::Custom(custom) => match custom {
                CustomCharset::Format0(fmt) => fmt.string_id(glyph_id),
                CustomCharset::Format1(fmt) => fmt.string_id(glyph_id),
                CustomCharset::Format2(fmt) => fmt.string_id(glyph_id),
            },
        }
    }

    /// Returns the glyph identifier for the given string identifier.
    pub fn glyph_id(&self, string_id: StringId) -> Result<GlyphId, ReadError> {
        let sid = string_id.to_u16();
        match &self.kind {
            CharsetKind::IsoAdobe => {
                // The ISOAdobe charset is an identity mapping of gid->sid up
                // to 228 entries
                // <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=45>
                if sid <= 228 {
                    Ok(GlyphId::from(sid))
                } else {
                    Err(ReadError::OutOfBounds)
                }
            }
            CharsetKind::Expert => EXPERT_CHARSET
                .iter()
                .position(|n| *n == sid)
                .map(|pos| GlyphId::new(pos as u32))
                .ok_or(ReadError::OutOfBounds),
            CharsetKind::ExpertSubset => EXPERT_SUBSET_CHARSET
                .iter()
                .position(|n| *n == sid)
                .map(|pos| GlyphId::new(pos as u32))
                .ok_or(ReadError::OutOfBounds),
            CharsetKind::Custom(custom) => match custom {
                CustomCharset::Format0(fmt) => fmt.glyph_id(string_id),
                CustomCharset::Format1(fmt) => fmt.glyph_id(string_id),
                CustomCharset::Format2(fmt) => fmt.glyph_id(string_id),
            },
        }
    }

    /// Returns an iterator over all of the glyph and string identifier
    /// mappings.
    pub fn iter(&self) -> CharsetIter<'a> {
        match &self.kind {
            CharsetKind::IsoAdobe
            | CharsetKind::Expert
            | CharsetKind::ExpertSubset
            | CharsetKind::Custom(CustomCharset::Format0(_)) => {
                CharsetIter(Iter::Simple(self.clone(), 0))
            }
            CharsetKind::Custom(CustomCharset::Format1(custom)) => CharsetIter(Iter::Custom1(
                RangeIter::new(custom.ranges(), self.num_glyphs),
            )),
            CharsetKind::Custom(CustomCharset::Format2(custom)) => CharsetIter(Iter::Custom2(
                RangeIter::new(custom.ranges(), self.num_glyphs),
            )),
        }
    }
}

/// Predefined and custom character sets.
#[derive(Clone)]
pub enum CharsetKind<'a> {
    IsoAdobe,
    Expert,
    ExpertSubset,
    Custom(CustomCharset<'a>),
}

impl CharsetFormat0<'_> {
    fn string_id(&self, glyph_id: GlyphId) -> Result<StringId, ReadError> {
        let gid = glyph_id.to_u32() as usize;
        if gid == 0 {
            Ok(StringId::new(0))
        } else {
            self.glyph()
                .get(gid - 1)
                .map(|id| StringId::new(id.get()))
                .ok_or(ReadError::OutOfBounds)
        }
    }

    fn glyph_id(&self, string_id: StringId) -> Result<GlyphId, ReadError> {
        if string_id.to_u16() == 0 {
            return Ok(GlyphId::NOTDEF);
        }
        self.glyph()
            .iter()
            .position(|n| n.get() == string_id.to_u16())
            .map(|n| GlyphId::from((n as u16).saturating_add(1)))
            .ok_or(ReadError::OutOfBounds)
    }
}

impl CharsetFormat1<'_> {
    fn string_id(&self, glyph_id: GlyphId) -> Result<StringId, ReadError> {
        string_id_from_ranges(self.ranges(), glyph_id)
    }

    fn glyph_id(&self, string_id: StringId) -> Result<GlyphId, ReadError> {
        glyph_id_from_ranges(self.ranges(), string_id)
    }
}

impl CharsetFormat2<'_> {
    fn string_id(&self, glyph_id: GlyphId) -> Result<StringId, ReadError> {
        string_id_from_ranges(self.ranges(), glyph_id)
    }

    fn glyph_id(&self, string_id: StringId) -> Result<GlyphId, ReadError> {
        glyph_id_from_ranges(self.ranges(), string_id)
    }
}

fn string_id_from_ranges<T: CharsetRange>(
    ranges: &[T],
    glyph_id: GlyphId,
) -> Result<StringId, ReadError> {
    let mut gid = glyph_id.to_u32();
    // The notdef glyph isn't explicitly mapped so we need to special case
    // it and add -1 and +1 at a few places when processing ranges
    if gid == 0 {
        return Ok(StringId::new(0));
    }
    gid -= 1;
    let mut end = 0u32;
    // Each range provides the string ids for `n_left + 1` glyphs with
    // the sequence of string ids starting at `first`. Since the counts
    // are cumulative, we must scan them all in order until we find
    // the range that contains our requested glyph.
    for range in ranges {
        let next_end = end
            .checked_add(range.n_left() + 1)
            .ok_or(ReadError::OutOfBounds)?;
        if gid < next_end {
            return (gid - end)
                .checked_add(range.first())
                .and_then(|sid| sid.try_into().ok())
                .ok_or(ReadError::OutOfBounds)
                .map(StringId::new);
        }
        end = next_end;
    }
    Err(ReadError::OutOfBounds)
}

fn glyph_id_from_ranges<T: CharsetRange>(
    ranges: &[T],
    string_id: StringId,
) -> Result<GlyphId, ReadError> {
    let sid = string_id.to_u16() as u32;
    // notdef glyph is not explicitly mapped
    if sid == 0 {
        return Ok(GlyphId::NOTDEF);
    }
    let mut gid = 1u32;
    for range in ranges {
        let first = range.first();
        let n_left = range.n_left();
        if first <= sid && sid <= (first + n_left) {
            gid += sid - first;
            return Ok(GlyphId::new(gid));
        }
        gid += n_left + 1;
    }
    Err(ReadError::OutOfBounds)
}

/// Trait that unifies ranges for formats 1 and 2 so that we can implement
/// the tricky search logic once.
trait CharsetRange {
    fn first(&self) -> u32;
    fn n_left(&self) -> u32;
}

impl CharsetRange for CharsetRange1 {
    fn first(&self) -> u32 {
        self.first.get() as u32
    }

    fn n_left(&self) -> u32 {
        self.n_left as u32
    }
}

impl CharsetRange for CharsetRange2 {
    fn first(&self) -> u32 {
        self.first.get() as u32
    }

    fn n_left(&self) -> u32 {
        self.n_left.get() as u32
    }
}

/// Iterator over the glyph and string identifier mappings in a character set.
#[derive(Clone)]
pub struct CharsetIter<'a>(Iter<'a>);

impl Iterator for CharsetIter<'_> {
    type Item = (GlyphId, StringId);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Iter::Simple(charset, cur) => {
                let gid = GlyphId::new(*cur);
                let sid = charset.string_id(gid).ok()?;
                *cur = cur.checked_add(1)?;
                Some((gid, sid))
            }
            Iter::Custom1(custom) => custom.next(),
            Iter::Custom2(custom) => custom.next(),
        }
    }
}

#[derive(Clone)]
enum Iter<'a> {
    /// Predefined sets and custom format 0 are just array lookups so we use
    /// the builtin mapping function.
    Simple(Charset<'a>, u32),
    Custom1(RangeIter<'a, CharsetRange1>),
    Custom2(RangeIter<'a, CharsetRange2>),
}

/// Custom iterator for range based formats.
///
/// Each individual lookup requires a linear scan through the ranges so this
/// provides a more efficient code path for iteration.
#[derive(Clone)]
struct RangeIter<'a, T> {
    ranges: std::slice::Iter<'a, T>,
    num_glyphs: u32,
    gid: u32,
    first: u32,
    end: u32,
    prev_end: u32,
}

impl<'a, T> RangeIter<'a, T>
where
    T: CharsetRange,
{
    fn new(ranges: &'a [T], num_glyphs: u32) -> Self {
        let mut ranges = ranges.iter();
        let (first, end) = next_range(&mut ranges).unwrap_or_default();
        Self {
            ranges,
            num_glyphs,
            gid: 0,
            first,
            end,
            prev_end: 0,
        }
    }

    fn next(&mut self) -> Option<(GlyphId, StringId)> {
        if self.gid >= self.num_glyphs {
            return None;
        }
        // The notdef glyph isn't explicitly mapped so we need to special case
        // it and add -1 and +1 at a few places when processing ranges
        if self.gid == 0 {
            self.gid += 1;
            return Some((GlyphId::new(0), StringId::new(0)));
        }
        let gid = self.gid - 1;
        self.gid = self.gid.checked_add(1)?;
        while gid >= self.end {
            let (first, end) = next_range(&mut self.ranges)?;
            self.prev_end = self.end;
            self.first = first;
            self.end = self.prev_end.checked_add(end)?;
        }
        let sid = self
            .first
            .checked_add(gid.checked_sub(self.prev_end)?)?
            .try_into()
            .ok()?;
        Some((GlyphId::new(gid + 1), StringId::new(sid)))
    }
}

fn next_range<T: CharsetRange>(ranges: &mut std::slice::Iter<T>) -> Option<(u32, u32)> {
    ranges
        .next()
        .map(|range| (range.first(), range.n_left() + 1))
}

/// See "Expert" charset at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=47>
#[rustfmt::skip]
const EXPERT_CHARSET: &[u16] = &[
      0,    1,  229,  230,  231,  232,  233,  234,  235,  236,  237,  238,   13,   14,   15,   99,
    239,  240,  241,  242,  243,  244,  245,  246,  247,  248,   27,   28,  249,  250,  251,  252,
    253,  254,  255,  256,  257,  258,  259,  260,  261,  262,  263,  264,  265,  266,  109,  110,
    267,  268,  269,  270,  271,  272,  273,  274,  275,  276,  277,  278,  279,  280,  281,  282,
    283,  284,  285,  286,  287,  288,  289,  290,  291,  292,  293,  294,  295,  296,  297,  298,
    299,  300,  301,  302,  303,  304,  305,  306,  307,  308,  309,  310,  311,  312,  313,  314,
    315,  316,  317,  318,  158,  155,  163,  319,  320,  321,  322,  323,  324,  325,  326,  150,
    164,  169,  327,  328,  329,  330,  331,  332,  333,  334,  335,  336,  337,  338,  339,  340,
    341,  342,  343,  344,  345,  346,  347,  348,  349,  350,  351,  352,  353,  354,  355,  356,
    357,  358,  359,  360,  361,  362,  363,  364,  365,  366,  367,  368,  369,  370,  371,  372,
    373,  374,  375,  376,  377,  378,
];

/// See "Expert Subset" charset at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=49>
#[rustfmt::skip]
const EXPERT_SUBSET_CHARSET: &[u16] = &[
      0,    1,  231,  232,  235,  236,  237,  238,   13,   14,   15,   99,  239,  240,  241,  242,
    243,  244,  245,  246,  247,  248,   27,   28,  249,  250,  251,  253,  254,  255,  256,  257,
    258,  259,  260,  261,  262,  263,  264,  265,  266,  109,  110,  267,  268,  269,  270,  272,
    300,  301,  302,  305,  314,  315,  158,  155,  163,  320,  321,  322,  323,  324,  325,  326,
    150,  164,  169,  327,  328,  329,  330,  331,  332,  333,  334,  335,  336,  337,  338,  339,
    340,  341,  342,  343,  344,  345,  346
];

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::bebuffer::BeBuffer;

    #[test]
    fn iso_adobe_charset() {
        // Offset of 0 signifies the ISOAdobe charset
        let charset_offset = 0;
        let num_glyphs = 64;
        // This is an identity mapping
        let expected = |gid: GlyphId| Some(gid.to_u32());
        test_simple_mapping(charset_offset, num_glyphs, expected);
    }

    #[test]
    fn expert_charset() {
        // Offset 1 signifies the expert charset
        let charset_offset = 1;
        let num_glyphs = 64;
        // This is an array based mapping
        let expected = |gid: GlyphId| {
            EXPERT_CHARSET
                .get(gid.to_u32() as usize)
                .map(|id| *id as u32)
        };
        test_simple_mapping(charset_offset, num_glyphs, expected);
    }

    #[test]
    fn expert_subset_charset() {
        // Offset 2 signifies the expert subset charset
        let charset_offset = 2;
        let num_glyphs = 64;
        // This is an array based mapping
        let expected = |gid: GlyphId| {
            EXPERT_SUBSET_CHARSET
                .get(gid.to_u32() as usize)
                .map(|id| *id as u32)
        };
        test_simple_mapping(charset_offset, num_glyphs, expected);
    }

    // Common test setup for identity or array based charset mappings
    fn test_simple_mapping(
        charset_offset: usize,
        num_glyphs: u32,
        expected: impl Fn(GlyphId) -> Option<u32>,
    ) {
        let charset = Charset::new(FontData::new(&[]), charset_offset, num_glyphs).unwrap();
        for gid in 0..num_glyphs {
            let gid = GlyphId::new(gid);
            let sid = expected(gid).unwrap();
            assert_eq!(charset.string_id(gid).unwrap().to_u16() as u32, sid);
            assert_eq!(charset.glyph_id(StringId::new(sid as _)).unwrap(), gid);
        }
        // Don't map glyphs beyond num_glyphs
        for gid in num_glyphs..u16::MAX as u32 {
            assert_eq!(charset.string_id(GlyphId::new(gid)).ok(), None);
        }
    }

    #[test]
    fn custom_mapping_format0() {
        let mut buf = BeBuffer::new();
        let num_glyphs = 6;
        // Add some padding so we can generate an offset greater than 2
        buf = buf.extend([0u8; 4]);
        // format 0
        buf = buf.push(0u8);
        // glyph array: each sid is gid * 2
        buf = buf.extend([2u16, 4, 6, 8, 10]);
        let charset = Charset::new(FontData::new(buf.data()), 4, num_glyphs).unwrap();
        // Test lookup code path
        for gid in 0..num_glyphs {
            assert_eq!(
                charset.string_id(GlyphId::new(gid)).unwrap().to_u16() as u32,
                gid * 2
            )
        }
        // Test iterator code path
        for (gid, sid) in charset.iter() {
            assert_eq!(sid.to_u16() as u32, gid.to_u32() * 2);
        }
        assert_eq!(charset.iter().count() as u32, num_glyphs);
        // Test out of bounds glyphs
        for gid in num_glyphs..u16::MAX as u32 {
            assert_eq!(charset.string_id(GlyphId::new(gid)).ok(), None);
        }
    }

    #[test]
    fn custom_mapping_format1() {
        let mut buf = BeBuffer::new();
        let num_glyphs = 7;
        // Add some padding so we can generate an offset greater than 2
        buf = buf.extend([0u8; 4]);
        // format 1
        buf = buf.push(1u8);
        // Three disjoint range mappings
        buf = buf.push(8u16).push(2u8);
        buf = buf.push(1200u16).push(0u8);
        buf = buf.push(20u16).push(1u8);
        let expected_sids = [0, 8, 9, 10, 1200, 20, 21];
        test_range_mapping(buf.data(), num_glyphs, &expected_sids);
    }

    #[test]
    fn custom_mapping_format2() {
        let mut buf = BeBuffer::new();
        // Add some padding so we can generate an offset greater than 2
        buf = buf.extend([0u8; 4]);
        // format 2
        buf = buf.push(2u8);
        // Three disjoint range mappings
        buf = buf.push(8u16).push(2u16);
        buf = buf.push(1200u16).push(0u16);
        buf = buf.push(20u16).push(800u16);
        let mut expected_sids = vec![0, 8, 9, 10, 1200];
        for i in 0..=800 {
            expected_sids.push(i + 20);
        }
        let num_glyphs = expected_sids.len() as u32;
        test_range_mapping(buf.data(), num_glyphs, &expected_sids);
    }

    // Common code for testing range based mappings
    fn test_range_mapping(data: &[u8], num_glyphs: u32, expected_sids: &[u32]) {
        let charset = Charset::new(FontData::new(data), 4, num_glyphs).unwrap();
        // Test lookup code path
        for (gid, sid) in expected_sids.iter().enumerate() {
            assert_eq!(
                charset.string_id(GlyphId::new(gid as _)).unwrap().to_u16() as u32,
                *sid
            )
        }
        // Test iterator code path
        assert!(charset.iter().eq(expected_sids
            .iter()
            .enumerate()
            .map(|(gid, sid)| (GlyphId::new(gid as u32), StringId::new(*sid as u16)))));
        assert_eq!(charset.iter().count() as u32, num_glyphs);
        // Test out of bounds glyphs
        for gid in num_glyphs..u16::MAX as u32 {
            assert_eq!(charset.string_id(GlyphId::new(gid)).ok(), None);
        }
        // Test reverse mapping
        for (gid, sid) in expected_sids.iter().enumerate() {
            assert_eq!(
                charset.glyph_id(StringId::new(*sid as u16)),
                Ok(GlyphId::new(gid as u32))
            );
        }
    }
}
