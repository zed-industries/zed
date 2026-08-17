//! Font and metric variation tables.

use super::{fixed::Fixed, raw_tag, Array, Bytes, RawFont, RawTag, U24};

pub const FVAR: RawTag = raw_tag(b"fvar");
pub const AVAR: RawTag = raw_tag(b"avar");
pub const HVAR: RawTag = raw_tag(b"HVAR");
pub const VVAR: RawTag = raw_tag(b"VVAR");
pub const MVAR: RawTag = raw_tag(b"MVAR");

/// Font variations table.
#[derive(Copy, Clone)]
pub struct Fvar<'a> {
    data: Bytes<'a>,
    axis_offset: u16,
    axis_count: u16,
    axis_size: u16,
    inst_count: u16,
    inst_size: u16,
}

impl<'a> Fvar<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let b = Bytes::new(data);
        let axis_offset = b.read_or_default::<u16>(4);
        let axis_count = b.read_or_default::<u16>(8);
        let axis_size = b.read_or_default::<u16>(10);
        let inst_count = b.read_or_default::<u16>(12);
        let inst_size = b.read_or_default::<u16>(14);
        Self {
            data: b,
            axis_offset,
            axis_count,
            axis_size,
            inst_count,
            inst_size,
        }
    }

    pub fn from_font(font: impl RawFont<'a>) -> Option<Self> {
        Some(Self::new(font.table_data(FVAR)?))
    }

    pub fn axis_count(&self) -> u16 {
        self.axis_count
    }

    pub fn get_axis(&self, index: u16) -> Option<VarAxis> {
        if index >= self.axis_count {
            return None;
        }
        let b = &self.data;
        let base = self.axis_offset as usize;
        let offset = base + index as usize * self.axis_size as usize;
        let tag = b.read::<u32>(offset)?;
        let min = Fixed(b.read::<i32>(offset + 4)?);
        let default = Fixed(b.read::<i32>(offset + 8)?);
        let max = Fixed(b.read::<i32>(offset + 12)?);
        let flags = b.read::<u16>(offset + 16)?;
        let name_id = b.read::<u16>(offset + 18)?;
        Some(VarAxis {
            index,
            tag,
            name_id,
            flags,
            min,
            default,
            max,
        })
    }

    pub fn get_axis_by_tag(&self, tag: RawTag) -> Option<VarAxis> {
        let b = &self.data;
        let base = self.axis_offset as usize;
        let axis_size = self.axis_size as usize;
        for i in 0..self.axis_count as usize {
            let tag_offset = base + i * axis_size;
            if b.read_u32(tag_offset) == Some(tag) {
                return self.get_axis(i as u16);
            }
        }
        None
    }

    pub fn instance_count(&self) -> u16 {
        self.inst_count
    }

    pub fn get_instance(&self, index: u16) -> Option<VarInstance<'a>> {
        if index >= self.inst_count {
            return None;
        }
        let b = &self.data;
        let base = self.axis_offset as usize + (self.axis_count as usize * self.axis_size as usize);
        let offset = base + index as usize * self.inst_size as usize;
        let name_id = b.read::<u16>(offset)?;
        let values = b.read_array::<Fixed>(offset + 4, self.axis_count as usize)?;
        let ps_name_offset = 4 + self.axis_count as usize * 4;
        let postscript_name_id = if ps_name_offset == self.inst_size as usize - 2 {
            b.read::<u16>(ps_name_offset)
        } else {
            None
        };
        Some(VarInstance {
            index,
            name_id,
            postscript_name_id,
            values,
        })
    }
}

/// Axis of variation in a variable font.
#[derive(Copy, Clone, Default)]
pub struct VarAxis {
    pub index: u16,
    pub tag: RawTag,
    pub name_id: u16,
    pub flags: u16,
    pub min: Fixed,
    pub default: Fixed,
    pub max: Fixed,
}

impl VarAxis {
    /// Returns true if the axis should be hidden in a user interface.
    pub fn is_hidden(&self) -> bool {
        self.flags & 1 != 0
    }

    /// Returns a normalized axis coordinate for the specified value in 2.14
    /// fixed point format.
    pub fn normalized_coord(&self, mut value: Fixed, avar: Option<(&[u8], u32)>) -> i16 {
        use core::cmp::Ordering::*;
        if value < self.min {
            value = self.min;
        } else if value > self.max {
            value = self.max;
        }
        value = match value.cmp(&self.default) {
            Less => -((self.default - value) / (self.default - self.min)),
            Greater => (value - self.default) / (self.max - self.default),
            Equal => Fixed(0),
        };
        value = value.min(Fixed::ONE).max(-Fixed::ONE);
        value = avar
            .and_then(|(data, avar)| adjust_axis(data, avar, self.index, value))
            .unwrap_or(value);
        value.to_f2dot14()
    }
}

/// Named instance in a variable font.
#[derive(Copy, Clone)]
pub struct VarInstance<'a> {
    pub index: u16,
    pub name_id: u16,
    pub postscript_name_id: Option<u16>,
    pub values: Array<'a, Fixed>,
}

/// Metrics variation table.
pub struct Mvar<'a> {
    data: Bytes<'a>,
    coords: &'a [i16],
    rec_size: usize,
    rec_count: usize,
    store: u32,
}

impl<'a> Mvar<'a> {
    pub fn new(data: &'a [u8], mvar: u32, coords: &'a [i16]) -> Option<Self> {
        let b = Bytes::with_offset(data, mvar as usize)?;
        let rec_size = b.read::<u16>(6)? as usize;
        let rec_count = b.read::<u16>(8)? as usize;
        let store = b.read::<u16>(10)? as u32;
        if rec_count == 0 || store == 0 {
            return None;
        }
        Some(Self {
            data: b,
            coords,
            rec_size,
            rec_count,
            store,
        })
    }

    pub fn delta(&self, metric: RawTag) -> f32 {
        self.read_delta(metric).map(|d| d.to_f32()).unwrap_or(0.)
    }

    #[inline(always)]
    fn read_delta(&self, metric: RawTag) -> Option<Fixed> {
        let base = 12;
        let b = &self.data;
        let rec_size = self.rec_size;
        let mut l = 0;
        let mut h = self.rec_count;
        while l < h {
            use core::cmp::Ordering::*;
            let i = (l + h) / 2;
            let offset = base + i * rec_size;
            let t = b.read::<u32>(offset)?;
            match metric.cmp(&t) {
                Less => h = i,
                Greater => l = i + 1,
                Equal => {
                    let inner = b.read::<u16>(offset + 4)?;
                    let outer = b.read::<u16>(offset + 6)?;
                    return item_delta(b.data(), self.store, outer, inner, self.coords);
                }
            }
        }
        None
    }
}

/// Returns the advance delta for the specified glyph.
pub fn advance_delta(data: &[u8], xvar: u32, glyph_id: u16, coords: &[i16]) -> f32 {
    metric_delta(data, xvar, 8, glyph_id, coords)
        .map(|d| d.to_f32())
        .unwrap_or(0.)
}

/// Returns the side bearing delta for the specified glyph.
pub fn sb_delta(data: &[u8], xvar: u32, glyph_id: u16, coords: &[i16]) -> f32 {
    metric_delta(data, xvar, 12, glyph_id, coords)
        .map(|d| d.to_f32())
        .unwrap_or(0.)
}

/// Applies adjustments to a coordinate according to the optional axis
/// variation table.
pub fn adjust_axis(data: &[u8], avar: u32, axis: u16, coord: Fixed) -> Option<Fixed> {
    use skrifa::raw::{tables::avar::Avar, types::Fixed, FontData, FontRead};

    if avar == 0 {
        return None;
    }
    let avar = Avar::read(FontData::new(data.get(avar as usize..)?)).ok()?;
    let mapping = avar
        .axis_segment_maps()
        .get(axis as usize)
        .transpose()
        .ok()??;
    Some(Fixed(mapping.apply(Fixed::from_bits(coord.0)).to_bits()))
}

/// Returns a delta from an item variation store.
pub fn item_delta(
    data: &[u8],
    offset: u32,
    outer: u16,
    inner: u16,
    coords: &[i16],
) -> Option<Fixed> {
    if offset == 0 {
        return None;
    }
    let b = Bytes::new(data);
    let store = offset as usize;
    if outer >= b.read::<u16>(store + 6)? {
        return None;
    }
    let region_base = store + b.read::<u32>(store + 2)? as usize;
    let axis_count = b.read::<u16>(region_base)? as usize;
    let region_record_size = axis_count * 6;
    let region_count = b.read::<u16>(region_base + 2)? as usize;
    let data_base = store + b.read::<u32>(store + 8 + outer as usize * 4)? as usize;
    let region_index_base = data_base + 6;
    let region_index_count = b.read::<u16>(data_base + 4)? as usize;
    let (short_count, mut delta_base) = {
        let inner = inner as usize;
        let short_count = b.read::<u16>(data_base + 2)? as usize;
        let count = region_index_count;
        let base = data_base + 6 + count * 2;
        let elem_len = (count - short_count) + short_count * 2;
        let offset = base + inner * elem_len;
        (short_count, offset)
    };
    const ZERO: Fixed = Fixed::ZERO;
    const ONE: Fixed = Fixed::ONE;
    let mut idx = 0;
    let mut delta = ZERO;
    for i in 0..region_index_count {
        let region_index = b.read::<u16>(region_index_base + i * 2)? as usize;
        if region_index >= region_count {
            return None;
        }
        let region_offset = region_base + 4 + region_index * region_record_size;
        let mut scalar = ONE;
        for axis in 0..axis_count {
            let region_axis_base = region_offset + axis * 6;
            let start = Fixed::from_f2dot14(b.read::<i16>(region_axis_base)?);
            let peak = Fixed::from_f2dot14(b.read::<i16>(region_axis_base + 2)?);
            let end = Fixed::from_f2dot14(b.read::<i16>(region_axis_base + 4)?);
            let coord = coords
                .get(axis)
                .map(|c| Fixed::from_f2dot14(*c))
                .unwrap_or(ZERO);
            if start > peak || peak > end || peak == ZERO || start < ZERO && end > ZERO {
                continue;
            } else if coord < start || coord > end {
                scalar = ZERO;
                break;
            } else if coord == peak {
                continue;
            } else if coord < peak {
                scalar = scalar * (coord - start) / (peak - start);
            } else {
                scalar = scalar * (end - coord) / (end - peak);
            };
        }
        let val = if idx >= short_count {
            delta_base += 1;
            b.read::<i8>(delta_base - 1)? as i16
        } else {
            delta_base += 2;
            b.read::<i16>(delta_base - 2)?
        };
        idx += 1;
        delta += scalar * Fixed::from_i32(val as i32);
    }
    Some(delta)
}

#[inline(always)]
fn metric_delta(
    data: &[u8],
    base: u32,
    which: usize,
    glyph_id: u16,
    coords: &[i16],
) -> Option<Fixed> {
    if base == 0 {
        return None;
    }
    let b = Bytes::new(data);
    let mut store = b.read::<u32>(base as usize + 4)?;
    if store == 0 {
        return None;
    }
    store += base;
    let mut offset = b.read::<u32>(base as usize + which)? as usize;
    if offset == 0 {
        if which == 8 {
            return item_delta(data, store, 0, glyph_id, coords);
        } else {
            return None;
        }
    }
    offset += base as usize;
    let format = b.read::<u16>(offset)? as u32;
    let count = b.read::<u16>(offset + 2)?;
    let bit_count = (format & 0xF) + 1;
    let entry_size = ((format & 0x30) >> 4) + 1;
    let base = offset + 4;
    let index = if glyph_id >= count {
        count - 1
    } else {
        glyph_id
    } as usize;
    let entry = match entry_size {
        1 => b.read::<u8>(base + index)? as u32,
        2 => b.read::<u16>(base + index * 2)? as u32,
        3 => b.read::<U24>(base + index * 3)?.0,
        4 => b.read::<u32>(base + index * 4)?,
        _ => return None,
    };
    let outer = entry >> bit_count;
    let inner = entry & ((1 << bit_count) - 1);
    item_delta(data, store, outer as u16, inner as u16, coords)
}

/// Tags for metrics from the `MVAR` table.
pub mod mvar_tags {
    use super::{raw_tag, RawTag};

    /// Horizontal ascender.
    pub const HASC: RawTag = raw_tag(b"hasc");
    /// Horizontal descender.
    pub const HDSC: RawTag = raw_tag(b"hdsc");
    /// Horizontal line gap.
    pub const HLGP: RawTag = raw_tag(b"hlgp");

    /// Horizontal caret rise.
    pub const HCRS: RawTag = raw_tag(b"hcrs");
    /// Horizontal caret run.
    pub const HCRN: RawTag = raw_tag(b"hcrn");
    /// Horizontal caret offset.
    pub const HCOF: RawTag = raw_tag(b"hcof");

    /// Horizontal clipping ascent.
    pub const HCLA: RawTag = raw_tag(b"hcla");
    /// Horizontal clipping descent.
    pub const HCLD: RawTag = raw_tag(b"hcld");

    /// Vertical ascender.
    pub const VASC: RawTag = raw_tag(b"vasc");
    /// Vertical descender.
    pub const VDSC: RawTag = raw_tag(b"vdsc");
    /// Vertical line gap.
    pub const VLGP: RawTag = raw_tag(b"vlgp");

    /// Vertical caret rise.
    pub const VCRS: RawTag = raw_tag(b"vcrs");
    /// Vertical caret run.
    pub const VCRN: RawTag = raw_tag(b"vcrn");
    /// Vertical caret offset.
    pub const VCOF: RawTag = raw_tag(b"vcof");

    /// X-height.
    pub const XHGT: RawTag = raw_tag(b"xhgt");
    /// Cap height.
    pub const CPHT: RawTag = raw_tag(b"cpht");

    /// Underline offset.
    pub const UNDO: RawTag = raw_tag(b"undo");
    /// Underline size.
    pub const UNDS: RawTag = raw_tag(b"unds");

    /// Strikeout offset.
    pub const STRO: RawTag = raw_tag(b"stro");
    /// Strikeout size.
    pub const STRS: RawTag = raw_tag(b"strs");

    /// Subscript x-offset.
    pub const SBXO: RawTag = raw_tag(b"sbxo");
    /// Subscript y-offset.
    pub const SBYO: RawTag = raw_tag(b"sbyo");
    /// Subscript x-size.
    pub const SBXS: RawTag = raw_tag(b"sbxs");
    /// Subscript y-size.
    pub const SBYS: RawTag = raw_tag(b"sbys");

    /// Superscript x-offset.
    pub const SPXO: RawTag = raw_tag(b"spxo");
    /// Superscript y-offset.
    pub const SPYO: RawTag = raw_tag(b"spyo");
    /// Superscript x-size.
    pub const SPXS: RawTag = raw_tag(b"spxs");
    /// Superscript y-size.
    pub const SPYS: RawTag = raw_tag(b"spys");
}
