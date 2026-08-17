//! Glyph data table.

use super::{raw_tag, Bytes, RawTag, Stream};

pub const GLYF: RawTag = raw_tag(b"glyf");
pub const LOCA: RawTag = raw_tag(b"loca");
pub const CVT_: RawTag = raw_tag(b"cvt ");
pub const FPGM: RawTag = raw_tag(b"fpgm");
pub const PREP: RawTag = raw_tag(b"prep");
pub const CVAR: RawTag = raw_tag(b"cvar");
pub const GVAR: RawTag = raw_tag(b"gvar");

/// Returns the data for the specified glyph.
pub fn get<'a>(
    data: &'a [u8],
    loca_fmt: u8,
    loca: u32,
    glyf: u32,
    glyph_id: u16,
) -> Option<&'a [u8]> {
    let range = {
        let b = Bytes::with_offset(data, loca as usize)?;
        let (start, end) = if loca_fmt == 0 {
            let offset = glyph_id as usize * 2;
            let start = b.read::<u16>(offset)? as usize * 2;
            let end = b.read::<u16>(offset + 2)? as usize * 2;
            (start, end)
        } else if loca_fmt == 1 {
            let offset = glyph_id as usize * 4;
            let start = b.read::<u32>(offset)? as usize;
            let end = b.read::<u32>(offset + 4)? as usize;
            (start, end)
        } else {
            return None;
        };
        if end < start {
            return None;
        }
        start..end
    };
    let glyf = Bytes::with_offset(data, glyf as usize)?;
    glyf.data().get(range)
}

/// Returns the y-max value of the specified glyph from the bounding box in the
/// `glyf` table.
pub fn ymax(data: &[u8], loca_fmt: u8, loca: u32, glyf: u32, glyph_id: u16) -> Option<i16> {
    let glyph_data = get(data, loca_fmt, loca, glyf, glyph_id)?;
    let mut s = Stream::new(glyph_data);
    s.skip(8)?;
    s.read()
}
