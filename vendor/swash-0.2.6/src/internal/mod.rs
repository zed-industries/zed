//! Low level OpenType parsing and internal data types.

#![allow(dead_code)]

pub mod fixed;

pub mod aat;
pub mod at;
pub mod cmap;
pub mod glyf;
pub mod head;
pub mod var;
pub mod vorg;
pub mod xmtx;

mod parse;

pub use parse::*;

use head::*;

pub type RawTag = u32;

/// Returns a tag value for the specified four bytes.
pub const fn raw_tag(bytes: &[u8; 4]) -> RawTag {
    (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
}

/// Functions for checking the validity of a font file and extracting
/// fonts from collections.
pub mod raw_data {
    use super::{raw_tag, Bytes, RawTag};

    const OTTO: RawTag = raw_tag(b"OTTO");
    const TTCF: RawTag = raw_tag(b"ttcf");
    const FONT: RawTag = 0x10000;
    const TRUE: RawTag = raw_tag(b"true");

    /// Returns true if the data represents a font collection.
    pub fn is_collection(data: &[u8]) -> bool {
        Bytes::new(data).read_u32(0) == Some(TTCF)
    }

    /// Returns true if the data represents a font at the specified offset.
    pub fn is_font(data: &[u8], offset: u32) -> bool {
        let tag = Bytes::new(data).read_u32(offset as usize).unwrap_or(0);
        tag == FONT || tag == OTTO || tag == TRUE
    }

    /// Returns the number of fonts contained in the specified data.
    pub fn count(data: &[u8]) -> u32 {
        if is_collection(data) {
            Bytes::new(data).read_u32(8).unwrap_or(0)
        } else if is_font(data, 0) {
            1
        } else {
            0
        }
    }

    /// Returns the byte offset for the font at the specified index in the data.
    pub fn offset(data: &[u8], index: u32) -> Option<u32> {
        if index >= count(data) {
            return None;
        }
        if is_font(data, 0) {
            Some(0)
        } else {
            Bytes::new(data).read_u32(12 + index as usize * 4)
        }
    }
}

/// Trait for types that can supply font tables.
pub trait RawFont<'a>: Sized {
    /// Returns the font data.
    fn data(&self) -> &'a [u8];

    /// Returns the offset to the table directory.
    fn offset(&self) -> u32;

    #[cfg(feature = "std")]
    fn dump_tables(&self) -> Option<()> {
        let base = self.offset() as usize;
        let b = Bytes::new(self.data());
        let len = b.read_u16(base.checked_add(4)?)? as usize;
        let record_base = base.checked_add(12)?;
        let reclen = 16usize;
        for i in 0..len {
            let recbase = reclen.checked_mul(i)?.checked_add(record_base)?;
            let mut s = b.stream_at(recbase)?;
            let table_tag = s.read_u32()?;
            let tb = table_tag.to_be_bytes();
            println!("{}", core::str::from_utf8(&tb).unwrap_or("??"));
        }
        Some(())
    }

    /// Returns the range for the table with the specified tag.
    fn table_range(&self, tag: RawTag) -> Option<(u32, u32)> {
        let base = self.offset() as usize;
        let b = Bytes::new(self.data());
        let len = b.read_u16(base.checked_add(4)?)? as usize;
        let record_base = base.checked_add(12)?;
        let reclen = 16usize;
        let mut l = 0;
        let mut h = len;
        while l < h {
            use core::cmp::Ordering::*;
            let i = (l + h) / 2;
            let recbase = reclen.checked_mul(i)?.checked_add(record_base)?;
            let mut s = b.stream_at(recbase)?;
            let table_tag = s.read_u32()?;
            match tag.cmp(&table_tag) {
                Less => h = i,
                Greater => l = i + 1,
                Equal => {
                    s.skip(4)?;
                    let start = s.read_u32()?;
                    let len = s.read_u32()?;
                    let end = start.checked_add(len)?;
                    return Some((start, end));
                }
            }
        }
        None
    }

    /// Returns the byte offset of the table with the specified tag.
    fn table_offset(&self, tag: RawTag) -> u32 {
        self.table_range(tag).map(|r| r.0).unwrap_or(0)
    }

    /// Returns the data for the table with the specified tag.
    fn table_data(&self, tag: RawTag) -> Option<&'a [u8]> {
        let r = self.table_range(tag)?;
        self.data().get(r.0 as usize..r.1 as usize)
    }

    /// Returns the font header table.
    fn head(&self) -> Option<Head<'a>> {
        Head::from_font(self)
    }

    /// Returns the OS/2 and Windows metrics table.
    fn os2(&self) -> Option<Os2<'a>> {
        Os2::from_font(self)
    }

    /// Returns the PostScript table.
    fn post(&self) -> Option<Post<'a>> {
        Post::from_font(self)
    }

    /// Returns the maximum profile table.
    fn maxp(&self) -> Option<Maxp<'a>> {
        Maxp::from_font(self)
    }

    /// Returns the horizontal header table.
    fn hhea(&self) -> Option<Hhea<'a>> {
        Hhea::from_font(self)
    }
    /// Returns the vertical header table.
    fn vhea(&self) -> Option<Vhea<'a>> {
        Vhea::from_font(self)
    }
}

impl<'a, T> RawFont<'a> for &T
where
    T: RawFont<'a>,
{
    fn data(&self) -> &'a [u8] {
        (*self).data()
    }

    fn offset(&self) -> u32 {
        (*self).offset()
    }
}
