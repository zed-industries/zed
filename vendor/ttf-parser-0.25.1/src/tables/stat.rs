//! A [Style Attributes Table](https://docs.microsoft.com/en-us/typography/opentype/spec/stat) implementation.

use crate::{
    parser::{Offset, Offset16, Offset32, Stream},
    Fixed, FromData, LazyArray16, Tag,
};

/// Axis-value pairing for [`AxisValueSubtableFormat4`].
#[derive(Clone, Copy, Debug)]
pub struct AxisValue {
    /// Zero-based index into [`Table::axes`].
    pub axis_index: u16,
    /// Numeric value for this axis.
    pub value: Fixed,
}

impl FromData for AxisValue {
    const SIZE: usize = 6;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let axis_index = s.read::<u16>()?;
        let value = s.read::<Fixed>()?;

        Some(AxisValue { axis_index, value })
    }
}

/// Iterator over axis value subtables.
#[derive(Clone, Debug)]
pub struct AxisValueSubtables<'a> {
    data: Stream<'a>,
    start: Offset32,
    offsets: LazyArray16<'a, Offset16>,
    index: u16,
    version: u32,
}

impl<'a> Iterator for AxisValueSubtables<'a> {
    type Item = AxisValueSubtable<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.offsets.len() {
            return None;
        }

        let mut s = Stream::new_at(
            self.data.tail()?,
            self.offsets.get(self.index)?.to_usize() + self.start.to_usize(),
        )?;
        self.index += 1;

        let format_variant = s.read::<u16>()?;

        let value = match format_variant {
            1 => {
                let value = s.read::<AxisValueSubtableFormat1>()?;
                Self::Item::Format1(value)
            }
            2 => {
                let value = s.read::<AxisValueSubtableFormat2>()?;
                Self::Item::Format2(value)
            }
            3 => {
                let value = s.read::<AxisValueSubtableFormat3>()?;
                Self::Item::Format3(value)
            }
            4 => {
                // Format 4 tables didn't exist until v1.2.
                if self.version < 0x00010002 {
                    return None;
                }

                let value = AxisValueSubtableFormat4::parse(s.tail()?)?;
                Self::Item::Format4(value)
            }
            _ => return None,
        };

        Some(value)
    }
}

/// The [axis record](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-records) struct provides information about a single design axis.
#[derive(Clone, Copy, Debug)]
pub struct AxisRecord {
    /// Axis tag.
    pub tag: Tag,
    /// The name ID for entries in the 'name' table that provide a display string for this axis.
    pub name_id: u16,
    /// Sort order for e.g. composing font family or face names.
    pub ordering: u16,
}

impl FromData for AxisRecord {
    const SIZE: usize = 8;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(AxisRecord {
            tag: s.read::<Tag>()?,
            name_id: s.read::<u16>()?,
            ordering: s.read::<u16>()?,
        })
    }
}

/// [Flags](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#flags) for [`AxisValueSubtable`].
#[derive(Clone, Copy)]
pub struct AxisValueFlags(u16);

#[rustfmt::skip]
impl AxisValueFlags {
    /// If set, this value also applies to older versions of this font.
    #[inline] pub fn older_sibling_attribute(self) -> bool { self.0 & (1 << 0) != 0 }

    /// If set, this value is the normal (a.k.a. "regular") value for the font family.
    #[inline] pub fn elidable(self) -> bool { self.0 & (1 << 1) != 0 }
}

impl core::fmt::Debug for AxisValueFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut dbg = f.debug_set();

        if self.older_sibling_attribute() {
            dbg.entry(&"OLDER_SIBLING_FONT_ATTRIBUTE");
        }
        if self.elidable() {
            dbg.entry(&"ELIDABLE_AXIS_VALUE_NAME");
        }

        dbg.finish()
    }
}

/// Axis value subtable [format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-1).
#[derive(Clone, Copy, Debug)]
pub struct AxisValueSubtableFormat1 {
    /// Zero-based index into [`Table::axes`].
    pub axis_index: u16,
    /// Flags for [`AxisValueSubtable`].
    pub flags: AxisValueFlags,
    /// The name ID of the display string.
    pub value_name_id: u16,
    /// Numeric value for this record.
    pub value: Fixed,
}

impl FromData for AxisValueSubtableFormat1 {
    const SIZE: usize = 10;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(AxisValueSubtableFormat1 {
            axis_index: s.read::<u16>()?,
            flags: AxisValueFlags(s.read::<u16>()?),
            value_name_id: s.read::<u16>()?,
            value: s.read::<Fixed>()?,
        })
    }
}

/// Axis value subtable [format 2](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-2).
#[derive(Clone, Copy, Debug)]
pub struct AxisValueSubtableFormat2 {
    /// Zero-based index into [`Table::axes`].
    pub axis_index: u16,
    /// Flags for [`AxisValueSubtable`].
    pub flags: AxisValueFlags,
    /// The name ID of the display string.
    pub value_name_id: u16,
    /// Nominal numeric value for this record.
    pub nominal_value: Fixed,
    /// The minimum value for this record.
    pub range_min_value: Fixed,
    /// The maximum value for this record.
    pub range_max_value: Fixed,
}

impl FromData for AxisValueSubtableFormat2 {
    const SIZE: usize = 18;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(AxisValueSubtableFormat2 {
            axis_index: s.read::<u16>()?,
            flags: AxisValueFlags(s.read::<u16>()?),
            value_name_id: s.read::<u16>()?,
            nominal_value: s.read::<Fixed>()?,
            range_min_value: s.read::<Fixed>()?,
            range_max_value: s.read::<Fixed>()?,
        })
    }
}

/// Axis value subtable [format 3](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-3).
#[derive(Clone, Copy, Debug)]
pub struct AxisValueSubtableFormat3 {
    /// Zero-based index into [`Table::axes`].
    pub axis_index: u16,
    /// Flags for [`AxisValueSubtable`].
    pub flags: AxisValueFlags,
    /// The name ID of the display string.
    pub value_name_id: u16,
    /// Numeric value for this record.
    pub value: Fixed,
    /// Numeric value for a style-linked mapping.
    pub linked_value: Fixed,
}

impl FromData for AxisValueSubtableFormat3 {
    const SIZE: usize = 14;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(AxisValueSubtableFormat3 {
            axis_index: s.read::<u16>()?,
            flags: AxisValueFlags(s.read::<u16>()?),
            value_name_id: s.read::<u16>()?,
            value: s.read::<Fixed>()?,
            linked_value: s.read::<Fixed>()?,
        })
    }
}

/// Axis value subtable [format 4](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-4).
#[derive(Clone, Copy, Debug)]
pub struct AxisValueSubtableFormat4<'a> {
    /// Flags for [`AxisValueSubtable`].
    pub flags: AxisValueFlags,
    /// The name ID of the display string.
    pub value_name_id: u16,
    /// List of axis-value pairings.
    pub values: LazyArray16<'a, AxisValue>,
}

impl<'a> AxisValueSubtableFormat4<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let axis_count = s.read::<u16>()?;
        let flags = AxisValueFlags(s.read::<u16>()?);
        let value_name_id = s.read::<u16>()?;
        let values = s.read_array16::<AxisValue>(axis_count)?;

        Some(AxisValueSubtableFormat4 {
            flags,
            value_name_id,
            values,
        })
    }
}

/// An [axis value subtable](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-tables).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum AxisValueSubtable<'a> {
    Format1(AxisValueSubtableFormat1),
    Format2(AxisValueSubtableFormat2),
    Format3(AxisValueSubtableFormat3),
    Format4(AxisValueSubtableFormat4<'a>),
}

impl<'a> AxisValueSubtable<'a> {
    /// Returns the value from an axis value subtable.
    ///
    /// For formats 1 and 3 the value is returned, for formats 2 and 4 `None` is returned as there
    /// is no single value associated with those formats.
    pub fn value(&self) -> Option<Fixed> {
        match self {
            Self::Format1(AxisValueSubtableFormat1 { value, .. })
            | Self::Format3(AxisValueSubtableFormat3 { value, .. }) => Some(*value),
            _ => None,
        }
    }

    /// Returns `true` if the axis subtable either is the value or is a range that contains the
    /// value passed in as an argument.
    ///
    /// Note: this will always return false for format 4 subtables as they may contain multiple
    /// axes.
    pub fn contains(&self, value: Fixed) -> bool {
        if let Some(subtable_value) = self.value() {
            if subtable_value.0 == value.0 {
                return true;
            }
        }

        if let Self::Format2(AxisValueSubtableFormat2 {
            range_min_value,
            range_max_value,
            ..
        }) = self
        {
            // core::ops::Range doesn't work here because Fixed doesn't implement
            // the required comparison traits.
            if value.0 >= range_min_value.0 && value.0 < range_max_value.0 {
                return true;
            }
        }

        false
    }

    /// Returns the associated name ID.
    pub fn name_id(&self) -> u16 {
        match self {
            Self::Format1(AxisValueSubtableFormat1 { value_name_id, .. })
            | Self::Format2(AxisValueSubtableFormat2 { value_name_id, .. })
            | Self::Format3(AxisValueSubtableFormat3 { value_name_id, .. })
            | Self::Format4(AxisValueSubtableFormat4 { value_name_id, .. }) => *value_name_id,
        }
    }

    #[inline]
    fn flags(&self) -> AxisValueFlags {
        match self {
            Self::Format1(AxisValueSubtableFormat1 { flags, .. })
            | Self::Format2(AxisValueSubtableFormat2 { flags, .. })
            | Self::Format3(AxisValueSubtableFormat3 { flags, .. })
            | Self::Format4(AxisValueSubtableFormat4 { flags, .. }) => *flags,
        }
    }

    /// Returns `true` if the axis subtable has the `ELIDABLE_AXIS_VALUE_NAME` flag set.
    pub fn is_elidable(&self) -> bool {
        self.flags().elidable()
    }

    /// Returns `true` if the axis subtable has the `OLDER_SIBLING_FONT_ATTRIBUTE` flag set.
    pub fn is_older_sibling(&self) -> bool {
        self.flags().older_sibling_attribute()
    }
}

/// A [Style Attributes Table](https://docs.microsoft.com/en-us/typography/opentype/spec/stat).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// List of axes
    pub axes: LazyArray16<'a, AxisRecord>,
    /// Fallback name when everything can be elided.
    pub fallback_name_id: Option<u16>,
    version: u32,
    data: &'a [u8],
    value_lookup_start: Offset32,
    value_offsets: LazyArray16<'a, Offset16>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let version = s.read::<u32>()?;

        // Supported versions are:
        // - 1.0
        // - 1.1 adds elidedFallbackNameId
        // - 1.2 adds format 4 axis value table
        if !(version == 0x00010000 || version == 0x00010001 || version == 0x00010002) {
            return None;
        }

        let _axis_size = s.read::<u16>()?;
        let axis_count = s.read::<u16>()?;
        let axis_offset = s.read::<Offset32>()?.to_usize();

        let value_count = s.read::<u16>()?;
        let value_lookup_start = s.read::<Offset32>()?;

        let fallback_name_id = if version >= 0x00010001 {
            // If version >= 1.1 the field is required
            Some(s.read::<u16>()?)
        } else {
            None
        };

        let mut s = Stream::new_at(data, axis_offset)?;
        let axes = s.read_array16::<AxisRecord>(axis_count)?;

        let mut s = Stream::new_at(data, value_lookup_start.to_usize())?;
        let value_offsets = s.read_array16::<Offset16>(value_count)?;

        Some(Self {
            axes,
            data,
            value_lookup_start,
            value_offsets,
            fallback_name_id,
            version,
        })
    }

    /// Returns an iterator over the collection of axis value tables.
    pub fn subtables(&self) -> AxisValueSubtables<'a> {
        AxisValueSubtables {
            data: Stream::new(self.data),
            start: self.value_lookup_start,
            offsets: self.value_offsets,
            index: 0,
            version: self.version,
        }
    }

    /// Returns the first matching subtable for a given axis.
    ///
    /// If no match value is given the first subtable for the axis is returned. If a match value is
    /// given, the first subtable for the axis where the value matches is returned. A value matches
    /// if it is equal to the subtable's value or contained within the range defined by the
    /// subtable. If no matches are found `None` is returned. Typically a match value is not
    /// specified for non-variable fonts as multiple subtables for a given axis ought not exist. For
    /// variable fonts a non-`None` match value should be specified as multiple records for each of
    /// the variation axes exist.
    ///
    /// Note: Format 4 subtables are explicitly ignored in this function.
    pub fn subtable_for_axis(
        &self,
        axis: Tag,
        match_value: Option<Fixed>,
    ) -> Option<AxisValueSubtable> {
        for subtable in self.subtables() {
            match subtable {
                AxisValueSubtable::Format1(AxisValueSubtableFormat1 {
                    axis_index, value, ..
                })
                | AxisValueSubtable::Format3(AxisValueSubtableFormat3 {
                    axis_index, value, ..
                }) => {
                    if self.axes.get(axis_index)?.tag != axis {
                        continue;
                    }

                    match match_value {
                        Some(match_value) => {
                            if match_value.0 == value.0 {
                                return Some(subtable);
                            }
                        }
                        None => return Some(subtable),
                    }
                }
                AxisValueSubtable::Format2(AxisValueSubtableFormat2 {
                    axis_index,
                    range_min_value,
                    range_max_value,
                    ..
                }) => {
                    if self.axes.get(axis_index)?.tag == axis {
                        continue;
                    }

                    match match_value {
                        Some(match_value) => {
                            if match_value.0 >= range_min_value.0
                                && match_value.0 < range_max_value.0
                            {
                                return Some(subtable);
                            }
                        }
                        None => return Some(subtable),
                    }
                }
                AxisValueSubtable::Format4(_) => {
                    // A query that's intended to search format 4 subtables can be performed
                    // across multiple axes. A separate function that takes a collection of
                    // axis-value pairs is more suitable than this.
                    continue;
                }
            }
        }

        None
    }
}
