//! Vertical origin table.

use super::{raw_tag, Bytes, RawTag};

pub const VORG: RawTag = raw_tag(b"VORG");

/// Returns the vertical origin for the specified glyph.
pub fn origin(data: &[u8], vorg: u32, glyph_id: u16) -> Option<i16> {
    if vorg == 0 {
        return None;
    }
    let b = Bytes::new(data);
    let base = vorg as usize;
    let default = b.read::<i16>(base + 4)?;
    let count = b.read::<u16>(base + 6)? as usize;
    let mut l = 0;
    let mut h = count;
    while l < h {
        use core::cmp::Ordering::*;
        let i = (l + h) / 2;
        let rec = base + 8 + i * 4;
        let g = b.read::<u16>(rec)?;
        match glyph_id.cmp(&g) {
            Less => h = i,
            Greater => l = i + 1,
            Equal => return b.read::<i16>(rec + 2),
        }
    }
    Some(default)
}
