use core::convert::TryFrom;

use crate::parser::Stream;

#[derive(Clone, Copy, Debug)]
pub(crate) struct DeltaSetIndexMap<'a> {
    data: &'a [u8],
}

impl<'a> DeltaSetIndexMap<'a> {
    #[inline]
    pub(crate) fn new(data: &'a [u8]) -> Self {
        DeltaSetIndexMap { data }
    }

    #[inline]
    pub(crate) fn map(&self, mut index: u32) -> Option<(u16, u16)> {
        let mut s = Stream::new(self.data);
        let format = s.read::<u8>()?;
        let entry_format = s.read::<u8>()?;
        let map_count = if format == 0 {
            s.read::<u16>()? as u32
        } else {
            s.read::<u32>()?
        };

        if map_count == 0 {
            return None;
        }

        // 'If a given glyph ID is greater than mapCount-1, then the last entry is used.'
        if index >= map_count {
            index = map_count - 1;
        }

        let entry_size = ((entry_format >> 4) & 3) + 1;
        let inner_index_bit_count = u32::from((entry_format & 0xF) + 1);

        s.advance(usize::from(entry_size) * usize::try_from(index).ok()?);

        let mut n = 0u32;
        for b in s.read_bytes(usize::from(entry_size))? {
            n = (n << 8) + u32::from(*b);
        }

        let outer_index = n >> inner_index_bit_count;
        let inner_index = n & ((1 << inner_index_bit_count) - 1);
        Some((
            u16::try_from(outer_index).ok()?,
            u16::try_from(inner_index).ok()?,
        ))
    }
}
