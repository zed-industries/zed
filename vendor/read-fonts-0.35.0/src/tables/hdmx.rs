//! The [Horizontal Device Metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/hdmx) table.

include!("../../generated/generated_hdmx.rs");

use std::cmp::Ordering;

impl<'a> Hdmx<'a> {
    /// Returns for the device record that exactly matches the given
    /// size (as ppem).
    pub fn record_for_size(&self, size: u8) -> Option<DeviceRecord<'a>> {
        let records = self.records();
        // Need a custom binary search because we're working with
        // ComputedArray
        let mut lo = 0;
        let mut hi = records.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let record = records.get(mid).ok()?;
            match record.pixel_size.cmp(&size) {
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
                Ordering::Equal => return Some(record),
            }
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceRecord<'a> {
    /// Pixel size for following widths (as ppem).
    pub pixel_size: u8,
    /// Maximum width.
    pub max_width: u8,
    /// Array of glyphs (numGlyphs is from the 'maxp' table).
    pub widths: &'a [u8],
}

impl<'a> DeviceRecord<'a> {
    /// Pixel size for following widths (as ppem).
    pub fn pixel_size(&self) -> u8 {
        self.pixel_size
    }

    /// Maximum width.
    pub fn max_width(&self) -> u8 {
        self.max_width
    }

    /// Array of widths, indexed by glyph id.
    pub fn widths(&self) -> &'a [u8] {
        self.widths
    }
}

impl ReadArgs for DeviceRecord<'_> {
    type Args = (u16, u32);
}

impl ComputeSize for DeviceRecord<'_> {
    fn compute_size(args: &(u16, u32)) -> Result<usize, ReadError> {
        let (_num_glyphs, size_device_record) = *args;
        // Record size is explicitly defined in the parent hdmx table
        Ok(size_device_record as usize)
    }
}

impl<'a> FontReadWithArgs<'a> for DeviceRecord<'a> {
    fn read_with_args(data: FontData<'a>, args: &(u16, u32)) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let (num_glyphs, _size_device_record) = *args;
        Ok(Self {
            pixel_size: cursor.read()?,
            max_width: cursor.read()?,
            widths: cursor.read_array(num_glyphs as usize)?,
        })
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a> DeviceRecord<'a> {
    /// A constructor that requires additional arguments.
    ///
    /// This type requires some external state in order to be
    /// parsed.
    pub fn read(
        data: FontData<'a>,
        num_glyphs: u16,
        size_device_record: u32,
    ) -> Result<Self, ReadError> {
        let args = (num_glyphs, size_device_record);
        Self::read_with_args(data, &args)
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for DeviceRecord<'a> {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "DeviceRecord",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("pixel_size", self.pixel_size())),
                1usize => Some(Field::new("max_width", self.max_width())),
                2usize => Some(Field::new("widths", self.widths())),
                _ => None,
            }),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::{be_buffer, bebuffer::BeBuffer};

    #[test]
    fn read_hdmx() {
        let buf = make_hdmx();
        let hdmx = Hdmx::read(buf.data().into(), 3).unwrap();
        assert_eq!(hdmx.version(), 0);
        assert_eq!(hdmx.num_records(), 3);
        // Note: this table has sizes for 3 glyphs making each device
        // record 5 bytes in actual length, but each entry in the array
        // should be aligned to 32-bits so the size is bumped to 8 bytes
        //
        // See the HdmxHeader::sizeDeviceRecord field at
        // <https://learn.microsoft.com/en-us/typography/opentype/spec/hdmx>
        // "Size of a device record, 32-bit aligned."
        assert_eq!(hdmx.size_device_record(), 8);
        let records = hdmx
            .records()
            .iter()
            .map(|rec| rec.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 3);
        let expected_records = [
            DeviceRecord {
                pixel_size: 8,
                max_width: 13,
                widths: &[10, 12, 13],
            },
            DeviceRecord {
                pixel_size: 16,
                max_width: 21,
                widths: &[18, 20, 21],
            },
            DeviceRecord {
                pixel_size: 32,
                max_width: 52,
                widths: &[38, 40, 52],
            },
        ];
        assert_eq!(records, expected_records);
    }

    #[test]
    fn find_by_size() {
        let buf = make_hdmx();
        let hdmx = Hdmx::read(buf.data().into(), 3).unwrap();
        assert_eq!(hdmx.record_for_size(8).unwrap().pixel_size, 8);
        assert_eq!(hdmx.record_for_size(16).unwrap().pixel_size, 16);
        assert_eq!(hdmx.record_for_size(32).unwrap().pixel_size, 32);
        assert!(hdmx.record_for_size(7).is_none());
        assert!(hdmx.record_for_size(20).is_none());
        assert!(hdmx.record_for_size(72).is_none());
    }

    fn make_hdmx() -> BeBuffer {
        be_buffer! {
            0u16,           // version
            3u16,           // num_records
            8u32,           // size_device_record
            // 3 records [pixel_size, max_width, width0, width1, ..padding]
            [8u8, 13, 10, 12, 13, 0, 0, 0],
            [16u8, 21, 18, 20, 21, 0, 0, 0],
            [32u8, 52, 38, 40, 52, 0, 0, 0]
        }
    }
}
