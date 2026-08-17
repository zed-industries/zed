//! Parsing for PostScript INDEX objects.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#5-index-data>

use super::{Error, Index1, Index2};
use crate::codegen_prelude::*;

/// Common type for uniform access to CFF and CFF2 index formats.
#[derive(Clone)]
pub enum Index<'a> {
    Empty,
    Format1(Index1<'a>),
    Format2(Index2<'a>),
}

impl<'a> Index<'a> {
    /// Creates a new index from the given data.
    ///
    /// The caller must specify whether the data comes from a `CFF2` table.
    pub fn new(data: &'a [u8], is_cff2: bool) -> Result<Self, Error> {
        let data = FontData::new(data);
        Ok(if is_cff2 {
            if data.len() == 4 && data.read_at::<u32>(0)? == 0 {
                return Ok(Self::Empty);
            }
            Index2::read(data).map(|ix| ix.into())?
        } else {
            if data.len() == 2 && data.read_at::<u16>(0)? == 0 {
                return Ok(Self::Empty);
            }
            Index1::read(data).map(|ix| ix.into())?
        })
    }

    /// Returns the number of objects in the index.
    pub fn count(&self) -> u32 {
        match self {
            Self::Empty => 0,
            Self::Format1(ix) => ix.count() as u32,
            Self::Format2(ix) => ix.count(),
        }
    }

    /// Computes a bias that is added to a subroutine operator in a
    /// charstring.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#9-local-and-global-subr-indexes>
    pub fn subr_bias(&self) -> i32 {
        let count = self.count();
        if count < 1240 {
            107
        } else if count < 33900 {
            1131
        } else {
            32768
        }
    }

    /// Returns the total size in bytes of the index table.
    pub fn size_in_bytes(&self) -> Result<usize, ReadError> {
        match self {
            Self::Empty => Ok(0),
            Self::Format1(ix) => ix.size_in_bytes(),
            Self::Format2(ix) => ix.size_in_bytes(),
        }
    }

    /// Returns the offset at the given index.
    pub fn get_offset(&self, index: usize) -> Result<usize, Error> {
        match self {
            Self::Empty => Err(ReadError::OutOfBounds.into()),
            Self::Format1(ix) => ix.get_offset(index),
            Self::Format2(ix) => ix.get_offset(index),
        }
    }

    /// Returns the data for the object at the given index.
    pub fn get(&self, index: usize) -> Result<&'a [u8], Error> {
        match self {
            Self::Empty => Err(ReadError::OutOfBounds.into()),
            Self::Format1(ix) => ix.get(index),
            Self::Format2(ix) => ix.get(index),
        }
    }

    pub fn off_size(&self) -> u8 {
        match self {
            Self::Empty => 0,
            Self::Format1(ix) => ix.off_size(),
            Self::Format2(ix) => ix.off_size(),
        }
    }
}

impl<'a> From<Index1<'a>> for Index<'a> {
    fn from(value: Index1<'a>) -> Self {
        Self::Format1(value)
    }
}

impl<'a> From<Index2<'a>> for Index<'a> {
    fn from(value: Index2<'a>) -> Self {
        Self::Format2(value)
    }
}

impl Default for Index<'_> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<'a> Index1<'a> {
    /// Returns the total size in bytes of the index table.
    pub fn size_in_bytes(&self) -> Result<usize, ReadError> {
        // 2 byte count + 1 byte off_size
        const HEADER_SIZE: usize = 3;
        // An empty CFF index contains only a 2 byte count field
        const EMPTY_SIZE: usize = 2;
        let count = self.count() as usize;
        Ok(match count {
            0 => EMPTY_SIZE,
            _ => {
                HEADER_SIZE
                    + self.offsets().len()
                    + self.get_offset(count).map_err(|_| ReadError::OutOfBounds)?
            }
        })
    }

    /// Returns the offset of the object at the given index.
    pub fn get_offset(&self, index: usize) -> Result<usize, Error> {
        read_offset(
            index,
            self.count() as usize,
            self.off_size(),
            self.offsets(),
        )
    }

    /// Returns the data for the object at the given index.
    pub fn get(&self, index: usize) -> Result<&'a [u8], Error> {
        self.data()
            .get(self.get_offset(index)?..self.get_offset(index + 1)?)
            .ok_or(ReadError::OutOfBounds.into())
    }
}

impl<'a> Index2<'a> {
    /// Returns the total size in bytes of the index table.
    pub fn size_in_bytes(&self) -> Result<usize, ReadError> {
        // 4 byte count + 1 byte off_size
        const HEADER_SIZE: usize = 5;
        // An empty CFF2 index contains only a 4 byte count field
        const EMPTY_SIZE: usize = 4;
        let count = self.count() as usize;
        Ok(match count {
            0 => EMPTY_SIZE,
            _ => {
                HEADER_SIZE
                    + self.offsets().len()
                    + self.get_offset(count).map_err(|_| ReadError::OutOfBounds)?
            }
        })
    }

    /// Returns the offset of the object at the given index.
    pub fn get_offset(&self, index: usize) -> Result<usize, Error> {
        read_offset(
            index,
            self.count() as usize,
            self.off_size(),
            self.offsets(),
        )
    }

    /// Returns the data for the object at the given index.
    pub fn get(&self, index: usize) -> Result<&'a [u8], Error> {
        self.data()
            .get(self.get_offset(index)?..self.get_offset(index + 1)?)
            .ok_or(ReadError::OutOfBounds.into())
    }
}

/// Reads an offset which is encoded as a variable sized integer.
fn read_offset(
    index: usize,
    count: usize,
    offset_size: u8,
    offset_data: &[u8],
) -> Result<usize, Error> {
    // There are actually count + 1 entries in the offset array.
    //
    // "Offsets in the offset array are relative to the byte that precedes
    // the object data. Therefore the first element of the offset array is
    // always 1. (This ensures that every object has a corresponding offset
    // which is always nonzero and permits the efficient implementation of
    // dynamic object loading.)"
    //
    // See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-7-index-format>
    if index > count {
        Err(ReadError::OutOfBounds)?;
    }
    let data_offset = index * offset_size as usize;
    let offset_data = FontData::new(offset_data);
    match offset_size {
        1 => offset_data.read_at::<u8>(data_offset)? as usize,
        2 => offset_data.read_at::<u16>(data_offset)? as usize,
        3 => offset_data.read_at::<Uint24>(data_offset)?.to_u32() as usize,
        4 => offset_data.read_at::<u32>(data_offset)? as usize,
        _ => return Err(Error::InvalidIndexOffsetSize(offset_size)),
    }
    // As above, subtract one to get the actual offset.
    .checked_sub(1)
    .ok_or(Error::ZeroOffsetInIndex)
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    enum IndexParams {
        Format1 { off_size: u8, count: usize },
        Format2 { off_size: u8, count: usize },
    }

    #[test]
    fn index_format1_offsize1_count4() {
        test_index(IndexParams::Format1 {
            off_size: 1,
            count: 4,
        });
    }

    #[test]
    fn index_format1_offsize2_count64() {
        test_index(IndexParams::Format1 {
            off_size: 2,
            count: 64,
        });
    }

    #[test]
    fn index_format1_offsize3_count128() {
        test_index(IndexParams::Format1 {
            off_size: 3,
            count: 128,
        });
    }

    #[test]
    fn index_format1_offsize4_count256() {
        test_index(IndexParams::Format1 {
            off_size: 4,
            count: 256,
        });
    }

    #[test]
    fn index_format2_offsize1_count4() {
        test_index(IndexParams::Format2 {
            off_size: 4,
            count: 256,
        });
    }

    #[test]
    fn index_format2_offsize2_count64() {
        test_index(IndexParams::Format2 {
            off_size: 2,
            count: 64,
        });
    }

    #[test]
    fn index_format2_offsize3_count128() {
        test_index(IndexParams::Format2 {
            off_size: 3,
            count: 128,
        });
    }

    #[test]
    fn index_format2_offsize4_count256() {
        test_index(IndexParams::Format2 {
            off_size: 4,
            count: 256,
        });
    }

    fn test_index(params: IndexParams) {
        let (fmt, off_size, count) = match params {
            IndexParams::Format1 { off_size, count } => (1, off_size, count),
            IndexParams::Format2 { off_size, count } => (2, off_size, count),
        };
        let buf = make_index(fmt, off_size, count);
        let index = Index::new(buf.data(), fmt == 2).unwrap();
        let built_off_size = match &index {
            Index::Empty => 0,
            Index::Format1(v1) => v1.off_size(),
            Index::Format2(v2) => v2.off_size(),
        };
        assert_eq!(built_off_size, off_size);
        assert_eq!(index.count(), count as u32);
        for i in 0..count {
            let object = index.get(i).unwrap();
            let expected_len = (i + 1) * 10;
            let expected_bytes = vec![i as u8; expected_len];
            assert_eq!(object, expected_bytes);
        }
    }

    fn make_index(fmt: u8, off_size: u8, count: usize) -> BeBuffer {
        // We'll add `count` objects to the INDEX, each containing
        // `(i + 1) * 10` bytes of the value `i`.
        let mut buf = BeBuffer::new();
        match fmt {
            1 => buf = buf.push(count as u16),
            2 => buf = buf.push(count as u32),
            _ => panic!("INDEX fmt should be 1 or 2"),
        }
        if count == 0 {
            return buf;
        }
        buf = buf.push(off_size);
        // Offsets start at 1.
        let mut offset = 1usize;
        for i in 0..count + 1 {
            buf = match off_size {
                1 => buf.push(offset as u8),
                2 => buf.push(offset as u16),
                3 => buf.push(Uint24::checked_new(offset as u32).unwrap()),
                4 => buf.push(offset as u32),
                _ => panic!("off_size should be 1-4"),
            };
            offset += (i + 1) * 10;
        }
        // Now the data
        for i in 0..count {
            buf = buf.extend(std::iter::repeat(i as u8).take((i + 1) * 10));
        }
        buf
    }
}
