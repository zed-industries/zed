use crate::{FontId, LineLayout, Pixels};
use std::ops::Range;

/// A complete shaped line with retained glyph positions.
#[derive(Debug)]
pub enum RetainedLineLayout {
    /// A dense layout, glyph x positions, and line width.
    Dense(LineLayout, Vec<f64>, f64),
    /// A lossless one-glyph-per-byte layout.
    Compact(CompactLineLayout),
}

/// Native output with identity byte indices and zero glyph y offsets.
#[derive(Debug)]
pub struct CompactLineLayout {
    /// The native run's font.
    pub font_id: FontId,
    /// The requested font size.
    pub font_size: Pixels,
    /// The line ascent.
    pub ascent: Pixels,
    /// The line descent.
    pub descent: Pixels,
    /// The native line width.
    pub width: f64,
    /// Whether the native run uses emoji rendering.
    pub is_emoji: bool,
    /// The native glyph IDs in byte order.
    pub glyphs: Vec<u16>,
    /// The native glyph x positions, excluding the terminal boundary.
    pub positions: CompactGlyphPositions,
}

/// Losslessly encoded floating-point position bits.
#[derive(Debug)]
pub struct CompactGlyphPositions {
    blocks: Vec<[u64; 3]>,
    raw: Vec<f64>,
    len: usize,
}

impl CompactGlyphPositions {
    /// The maximum number of positions requested per read.
    pub const BLOCK_LEN: usize = 1_024;

    /// Encodes positions from a stable source, reading raw blocks twice.
    pub fn new(
        len: usize,
        mut read: impl FnMut(Range<usize>, &mut [f64]) -> Option<()>,
    ) -> Option<Self> {
        let mut blocks = Vec::with_capacity(len.div_ceil(Self::BLOCK_LEN));
        let mut values = [0_f64; Self::BLOCK_LEN];
        let mut raw_len = 0usize;
        for start in (0..len).step_by(Self::BLOCK_LEN) {
            let end = start.saturating_add(Self::BLOCK_LEN).min(len);
            let values = &mut values[..end - start];
            read(start..end, values)?;
            let first = values.first()?.to_bits();
            let stride = values
                .get(1)
                .map_or(0, |value| value.to_bits().wrapping_sub(first));
            if values.iter().enumerate().all(|(index, value)| {
                value.to_bits() == first.wrapping_add(stride.wrapping_mul(index as u64))
            }) {
                blocks.push([first, stride, 0]);
            } else {
                blocks.push([first, u64::try_from(raw_len).ok()?, 1]);
                raw_len = raw_len.checked_add(values.len())?;
            }
        }
        let mut raw = vec![0_f64; raw_len];
        for (index, block) in blocks.iter().enumerate() {
            if block[2] == 0 {
                continue;
            }
            let start = index * Self::BLOCK_LEN;
            let end = start.saturating_add(Self::BLOCK_LEN).min(len);
            let offset = usize::try_from(block[1]).ok()?;
            read(start..end, raw.get_mut(offset..offset + (end - start))?)?;
        }
        Some(Self { blocks, raw, len })
    }

    /// Returns the number of glyph positions.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether there are no glyph positions.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns an exact glyph position, or none past the final glyph.
    pub fn get(&self, index: usize) -> Option<f64> {
        if index >= self.len {
            return None;
        }
        let block = self.blocks.get(index / Self::BLOCK_LEN)?;
        let offset = index % Self::BLOCK_LEN;
        if block[2] == 0 {
            Some(f64::from_bits(
                block[0].wrapping_add(block[1].wrapping_mul(offset as u64)),
            ))
        } else {
            self.raw
                .get(usize::try_from(block[1]).ok()? + offset)
                .copied()
        }
    }

    /// Finds the partition boundary for a monotone predicate.
    pub fn partition_point(&self, mut predicate: impl FnMut(f64) -> bool) -> usize {
        let mut left = 0;
        let mut right = self.len;
        while left < right {
            let middle = left + (right - left) / 2;
            if self.get(middle).is_some_and(&mut predicate) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::CompactGlyphPositions;

    #[test]
    fn test_compact_positions_preserve_exact_bits() {
        for len in [0, 1, 2, 1_023, 1_024, 1_025, 2_048, 2_049] {
            for first in [0, 1, 1_f64.to_bits(), (-1_f64).to_bits(), u64::MAX] {
                for stride in [0, 1, 3, 1 << 52, u64::MAX] {
                    let mut value = first;
                    let bits = (0..len)
                        .map(|_| {
                            let bits = value;
                            value = value.wrapping_add(stride);
                            bits
                        })
                        .collect::<Vec<_>>();
                    let positions = CompactGlyphPositions::new(len, |range, values| {
                        for (value, bits) in values.iter_mut().zip(&bits[range]) {
                            *value = f64::from_bits(*bits);
                        }
                        Some(())
                    })
                    .unwrap();
                    assert_eq!(positions.len(), len);
                    assert_eq!(positions.is_empty(), len == 0);
                    assert_eq!(positions.raw.len(), 0);
                    assert_eq!(positions.get(len), None);
                    assert_eq!(
                        (0..len)
                            .map(|index| positions.get(index).unwrap().to_bits())
                            .collect::<Vec<_>>(),
                        bits,
                    );
                }
            }
        }
        let mut bits = (0..4_097_u64)
            .map(|index| 1_f64.to_bits() + index * 3)
            .collect::<Vec<_>>();
        bits[1_023] ^= 1;
        bits[3_071] ^= 1;
        let positions = CompactGlyphPositions::new(bits.len(), |range, values| {
            for (value, bits) in values.iter_mut().zip(&bits[range]) {
                *value = f64::from_bits(*bits);
            }
            Some(())
        })
        .unwrap();
        assert_eq!(positions.raw.len(), 2_048);
        assert_eq!(
            (0..bits.len())
                .map(|index| positions.get(index).unwrap().to_bits())
                .collect::<Vec<_>>(),
            bits,
        );
        let bits = [
            0,
            1 << 63,
            1,
            0x0010_0000_0000_0000,
            0x7fef_ffff_ffff_ffff,
            0x7ff0_0000_0000_0000,
            0xfff0_0000_0000_0000,
            0x7ff0_0000_0000_0001,
        ];
        let positions = CompactGlyphPositions::new(bits.len(), |range, values| {
            for (value, bits) in values.iter_mut().zip(&bits[range]) {
                *value = f64::from_bits(*bits);
            }
            Some(())
        })
        .unwrap();
        assert_eq!(
            (0..bits.len())
                .map(|index| positions.get(index).unwrap().to_bits())
                .collect::<Vec<_>>(),
            bits,
        );
        assert!(CompactGlyphPositions::new(1, |_, _| None).is_none());
        let mut reads = 0;
        assert!(
            CompactGlyphPositions::new(bits.len(), |range, values| {
                reads += 1;
                if reads == 2 {
                    return None;
                }
                for (value, bits) in values.iter_mut().zip(&bits[range]) {
                    *value = f64::from_bits(*bits);
                }
                Some(())
            })
            .is_none()
        );
    }

    #[test]
    fn test_compact_positions_partition_point() {
        let values = (0..2_049)
            .map(|index| (index / 3) as f64)
            .collect::<Vec<_>>();
        let positions = CompactGlyphPositions::new(values.len(), |range, output| {
            output.copy_from_slice(&values[range]);
            Some(())
        })
        .unwrap();
        for target in [-1., 0., 0.5, 1., 341., 682., 683., f64::INFINITY] {
            assert_eq!(
                positions.partition_point(|position| position < target),
                values
                    .iter()
                    .position(|position| *position >= target)
                    .unwrap_or(values.len()),
            );
            assert_eq!(
                positions.partition_point(|position| position <= target),
                values
                    .iter()
                    .position(|position| *position > target)
                    .unwrap_or(values.len()),
            );
        }
    }
}
