//! A GPOS ValueRecord

use font_types::Nullable;
use types::{BigEndian, F2Dot14, FixedSize, Offset16};

use super::ValueFormat;
use crate::{
    tables::{
        layout::DeviceOrVariationIndex,
        variations::{DeltaSetIndex, ItemVariationStore},
    },
    ResolveNullableOffset,
};

#[cfg(feature = "experimental_traverse")]
use crate::traversal::{Field, FieldType, RecordResolver, SomeRecord};
use crate::{ComputeSize, FontData, FontReadWithArgs, ReadArgs, ReadError};

impl ValueFormat {
    /// A mask with all the device/variation index bits set
    pub const ANY_DEVICE_OR_VARIDX: Self = ValueFormat {
        bits: 0x0010 | 0x0020 | 0x0040 | 0x0080,
    };

    /// Return the number of bytes required to store a [`ValueRecord`] in this format.
    #[inline]
    pub fn record_byte_len(self) -> usize {
        self.bits().count_ones() as usize * u16::RAW_BYTE_LEN
    }
}

/// A context for resolving [`Value`]s and [`ValueRecord`]s.
///
/// In particular, this handles processing of the embedded
/// [`DeviceOrVariationIndex`] tables.
#[derive(Clone, Default)]
pub struct ValueContext<'a> {
    coords: &'a [F2Dot14],
    var_store: Option<ItemVariationStore<'a>>,
}

impl<'a> ValueContext<'a> {
    /// Creates a new value context that doesn't do any additional processing.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the normalized variation coordinates for this value context.
    pub fn with_coords(mut self, coords: &'a [F2Dot14]) -> Self {
        self.coords = coords;
        self
    }

    /// Sets the item variation store for this value context.
    ///
    /// This comes from the [`Gdef`](super::super::gdef::Gdef) table.
    pub fn with_var_store(mut self, var_store: Option<ItemVariationStore<'a>>) -> Self {
        self.var_store = var_store;
        self
    }

    fn var_store_and_coords(&self) -> Option<(&ItemVariationStore<'a>, &'a [F2Dot14])> {
        Some((self.var_store.as_ref()?, self.coords))
    }
}

/// A fully resolved [`ValueRecord`].
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Value {
    pub format: ValueFormat,
    pub x_placement: i16,
    pub y_placement: i16,
    pub x_advance: i16,
    pub y_advance: i16,
    pub x_placement_delta: i32,
    pub y_placement_delta: i32,
    pub x_advance_delta: i32,
    pub y_advance_delta: i32,
}

impl Value {
    /// Reads a value directly from font data.
    ///
    /// The `offset_data` parameter must be the offset data for the table
    /// containing the value record.
    #[inline]
    pub fn read(
        offset_data: FontData,
        offset: usize,
        format: ValueFormat,
        context: &ValueContext,
    ) -> Result<Self, ReadError> {
        let mut value = Self {
            format,
            ..Default::default()
        };
        let mut cursor = offset_data.cursor();
        cursor.advance_by(offset);
        if format.contains(ValueFormat::X_PLACEMENT) {
            value.x_placement = cursor.read()?;
        }
        if format.contains(ValueFormat::Y_PLACEMENT) {
            value.y_placement = cursor.read()?;
        }
        if format.contains(ValueFormat::X_ADVANCE) {
            value.x_advance = cursor.read()?;
        }
        if format.contains(ValueFormat::Y_ADVANCE) {
            value.y_advance = cursor.read()?;
        }
        if !format.contains(ValueFormat::ANY_DEVICE_OR_VARIDX) {
            return Ok(value);
        }
        if let Some((ivs, coords)) = context.var_store_and_coords() {
            let compute_delta = |offset: u16| {
                let rec_offset = offset_data.read_at::<u16>(offset as usize).ok()? as usize;
                let format = offset_data.read_at::<u16>(rec_offset + 4).ok()?;
                // DeltaFormat specifier for a VariationIndex table
                // See <https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#device-and-variationindex-tables>
                const VARIATION_INDEX_FORMAT: u16 = 0x8000;
                if format != VARIATION_INDEX_FORMAT {
                    return Some(0);
                }
                let outer = offset_data.read_at::<u16>(rec_offset).ok()?;
                let inner = offset_data.read_at::<u16>(rec_offset + 2).ok()?;
                ivs.compute_delta(DeltaSetIndex { outer, inner }, coords)
                    .ok()
            };
            if format.contains(ValueFormat::X_PLACEMENT_DEVICE) {
                value.x_placement_delta = compute_delta(cursor.read()?).unwrap_or_default();
            }
            if format.contains(ValueFormat::Y_PLACEMENT_DEVICE) {
                value.y_placement_delta = compute_delta(cursor.read()?).unwrap_or_default();
            }
            if format.contains(ValueFormat::X_ADVANCE_DEVICE) {
                value.x_advance_delta = compute_delta(cursor.read()?).unwrap_or_default();
            }
            if format.contains(ValueFormat::Y_ADVANCE_DEVICE) {
                value.y_advance_delta = compute_delta(cursor.read()?).unwrap_or_default();
            }
        }
        Ok(value)
    }
}

/// A Positioning ValueRecord.
///
/// NOTE: we create these manually, since parsing is weird and depends on the
/// associated valueformat. That said, this isn't a great representation?
/// we could definitely do something much more in the zero-copy mode..
#[derive(Clone, Default, Eq)]
pub struct ValueRecord {
    pub x_placement: Option<BigEndian<i16>>,
    pub y_placement: Option<BigEndian<i16>>,
    pub x_advance: Option<BigEndian<i16>>,
    pub y_advance: Option<BigEndian<i16>>,
    pub x_placement_device: BigEndian<Nullable<Offset16>>,
    pub y_placement_device: BigEndian<Nullable<Offset16>>,
    pub x_advance_device: BigEndian<Nullable<Offset16>>,
    pub y_advance_device: BigEndian<Nullable<Offset16>>,
    #[doc(hidden)]
    // exposed so that we can preserve format when we round-trip a value record
    pub format: ValueFormat,
}

// we ignore the format for the purpose of equality testing, it's redundant
impl PartialEq for ValueRecord {
    fn eq(&self, other: &Self) -> bool {
        self.x_placement == other.x_placement
            && self.y_placement == other.y_placement
            && self.x_advance == other.x_advance
            && self.y_advance == other.y_advance
            && self.x_placement_device == other.x_placement_device
            && self.y_placement_device == other.y_placement_device
            && self.x_advance_device == other.x_advance_device
            && self.y_advance_device == other.y_advance_device
    }
}

impl ValueRecord {
    pub fn read(data: FontData, format: ValueFormat) -> Result<Self, ReadError> {
        let mut this = ValueRecord {
            format,
            ..Default::default()
        };
        let mut cursor = data.cursor();

        if format.contains(ValueFormat::X_PLACEMENT) {
            this.x_placement = Some(cursor.read_be()?);
        }
        if format.contains(ValueFormat::Y_PLACEMENT) {
            this.y_placement = Some(cursor.read_be()?);
        }
        if format.contains(ValueFormat::X_ADVANCE) {
            this.x_advance = Some(cursor.read_be()?);
        }
        if format.contains(ValueFormat::Y_ADVANCE) {
            this.y_advance = Some(cursor.read_be()?);
        }
        if format.contains(ValueFormat::X_PLACEMENT_DEVICE) {
            this.x_placement_device = cursor.read_be()?;
        }
        if format.contains(ValueFormat::Y_PLACEMENT_DEVICE) {
            this.y_placement_device = cursor.read_be()?;
        }
        if format.contains(ValueFormat::X_ADVANCE_DEVICE) {
            this.x_advance_device = cursor.read_be()?;
        }
        if format.contains(ValueFormat::Y_ADVANCE_DEVICE) {
            this.y_advance_device = cursor.read_be()?;
        }
        Ok(this)
    }

    pub fn x_placement(&self) -> Option<i16> {
        self.x_placement.map(|val| val.get())
    }

    pub fn y_placement(&self) -> Option<i16> {
        self.y_placement.map(|val| val.get())
    }

    pub fn x_advance(&self) -> Option<i16> {
        self.x_advance.map(|val| val.get())
    }

    pub fn y_advance(&self) -> Option<i16> {
        self.y_advance.map(|val| val.get())
    }

    pub fn x_placement_device<'a>(
        &self,
        data: FontData<'a>,
    ) -> Option<Result<DeviceOrVariationIndex<'a>, ReadError>> {
        self.x_placement_device.get().resolve(data)
    }

    pub fn y_placement_device<'a>(
        &self,
        data: FontData<'a>,
    ) -> Option<Result<DeviceOrVariationIndex<'a>, ReadError>> {
        self.y_placement_device.get().resolve(data)
    }

    pub fn x_advance_device<'a>(
        &self,
        data: FontData<'a>,
    ) -> Option<Result<DeviceOrVariationIndex<'a>, ReadError>> {
        self.x_advance_device.get().resolve(data)
    }

    pub fn y_advance_device<'a>(
        &self,
        data: FontData<'a>,
    ) -> Option<Result<DeviceOrVariationIndex<'a>, ReadError>> {
        self.y_advance_device.get().resolve(data)
    }

    /// Returns a resolved value for the given normalized coordinates and
    /// item variation store.
    ///
    /// The `offset_data` parameter must be the offset data for the table
    /// containing the value record.
    pub fn value(&self, offset_data: FontData, context: &ValueContext) -> Result<Value, ReadError> {
        let mut value = Value {
            format: self.format,
            x_placement: self.x_placement.unwrap_or_default().get(),
            y_placement: self.y_placement.unwrap_or_default().get(),
            x_advance: self.x_advance.unwrap_or_default().get(),
            y_advance: self.y_advance.unwrap_or_default().get(),
            ..Default::default()
        };
        if let Some((ivs, coords)) = context.var_store_and_coords() {
            let compute_delta = |value: DeviceOrVariationIndex| match value {
                DeviceOrVariationIndex::VariationIndex(var_idx) => {
                    let outer = var_idx.delta_set_outer_index();
                    let inner = var_idx.delta_set_inner_index();
                    ivs.compute_delta(DeltaSetIndex { outer, inner }, coords)
                        .ok()
                }
                _ => None,
            };
            if let Some(device) = self.x_placement_device(offset_data) {
                value.x_placement_delta = compute_delta(device?).unwrap_or_default();
            }
            if let Some(device) = self.y_placement_device(offset_data) {
                value.y_placement_delta = compute_delta(device?).unwrap_or_default();
            }
            if let Some(device) = self.x_advance_device(offset_data) {
                value.x_advance_delta = compute_delta(device?).unwrap_or_default();
            }
            if let Some(device) = self.y_advance_device(offset_data) {
                value.y_advance_delta = compute_delta(device?).unwrap_or_default();
            }
        }
        Ok(value)
    }
}

impl ReadArgs for ValueRecord {
    type Args = ValueFormat;
}

impl<'a> FontReadWithArgs<'a> for ValueRecord {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        ValueRecord::read(data, *args)
    }
}

impl std::fmt::Debug for ValueRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("ValueRecord");
        self.x_placement.map(|x| f.field("x_placement", &x));
        self.y_placement.map(|y| f.field("y_placement", &y));
        self.x_advance.map(|x| f.field("x_advance", &x));
        self.y_advance.map(|y| f.field("y_advance", &y));
        if !self.x_placement_device.get().is_null() {
            f.field("x_placement_device", &self.x_placement_device.get());
        }
        if !self.y_placement_device.get().is_null() {
            f.field("y_placement_device", &self.y_placement_device.get());
        }
        if !self.x_advance_device.get().is_null() {
            f.field("x_advance_device", &self.x_advance_device.get());
        }
        if !self.y_advance_device.get().is_null() {
            f.field("y_advance_device", &self.y_advance_device.get());
        }
        f.finish()
    }
}

impl ComputeSize for ValueRecord {
    #[inline]
    fn compute_size(args: &ValueFormat) -> Result<usize, ReadError> {
        Ok(args.record_byte_len())
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> ValueRecord {
    pub(crate) fn traversal_type(&self, data: FontData<'a>) -> FieldType<'a> {
        FieldType::Record(self.clone().traverse(data))
    }

    pub(crate) fn get_field(&self, idx: usize, data: FontData<'a>) -> Option<Field<'a>> {
        let fields = [
            self.x_placement.is_some().then_some("x_placement"),
            self.y_placement.is_some().then_some("y_placement"),
            self.x_advance.is_some().then_some("x_advance"),
            self.y_advance.is_some().then_some("y_advance"),
            (!self.x_placement_device.get().is_null()).then_some("x_placement_device"),
            (!self.y_placement_device.get().is_null()).then_some("y_placement_device"),
            (!self.x_advance_device.get().is_null()).then_some("x_advance_device"),
            (!self.y_advance_device.get().is_null()).then_some("y_advance_device"),
        ];

        let name = fields.iter().filter_map(|x| *x).nth(idx)?;
        let typ: FieldType = match name {
            "x_placement" => self.x_placement().unwrap().into(),
            "y_placement" => self.y_placement().unwrap().into(),
            "x_advance" => self.x_advance().unwrap().into(),
            "y_advance" => self.y_advance().unwrap().into(),
            "x_placement_device" => {
                FieldType::offset(self.x_placement_device.get(), self.x_placement_device(data))
            }
            "y_placement_device" => {
                FieldType::offset(self.y_placement_device.get(), self.y_placement_device(data))
            }
            "x_advance_device" => {
                FieldType::offset(self.x_advance_device.get(), self.x_advance_device(data))
            }
            "y_advance_device" => {
                FieldType::offset(self.y_advance_device.get(), self.y_advance_device(data))
            }
            _ => panic!("hmm"),
        };

        Some(Field::new(name, typ))
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for ValueRecord {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "ValueRecord",
            data,
            get_field: Box::new(move |idx, data| self.get_field(idx, data)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check_format_const() {
        let format = ValueFormat::X_ADVANCE_DEVICE
            | ValueFormat::Y_ADVANCE_DEVICE
            | ValueFormat::Y_PLACEMENT_DEVICE
            | ValueFormat::X_PLACEMENT_DEVICE;
        assert_eq!(format, ValueFormat::ANY_DEVICE_OR_VARIDX);
        assert_eq!(format.record_byte_len(), 4 * 2);
    }
}
