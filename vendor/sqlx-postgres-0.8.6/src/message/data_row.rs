use byteorder::{BigEndian, ByteOrder};
use sqlx_core::bytes::Bytes;
use std::ops::Range;

use crate::error::Error;
use crate::message::{BackendMessage, BackendMessageFormat};

/// A row of data from the database.
#[derive(Debug)]
pub struct DataRow {
    pub(crate) storage: Bytes,

    /// Ranges into the stored row data.
    /// This uses `u32` instead of usize to reduce the size of this type. Values cannot be larger
    /// than `i32` in postgres.
    pub(crate) values: Vec<Option<Range<u32>>>,
}

impl DataRow {
    #[inline]
    pub(crate) fn get(&self, index: usize) -> Option<&'_ [u8]> {
        self.values[index]
            .as_ref()
            .map(|col| &self.storage[(col.start as usize)..(col.end as usize)])
    }
}

impl BackendMessage for DataRow {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::DataRow;

    fn decode_body(buf: Bytes) -> Result<Self, Error> {
        if buf.len() < 2 {
            return Err(err_protocol!(
                "expected at least 2 bytes, got {}",
                buf.len()
            ));
        }

        let cnt = BigEndian::read_u16(&buf) as usize;

        let mut values = Vec::with_capacity(cnt);
        let mut offset: u32 = 2;

        for _ in 0..cnt {
            let value_start = offset
                .checked_add(4)
                .ok_or_else(|| err_protocol!("next value start out of range (offset: {offset})"))?;

            // widen both to a larger type for a safe comparison
            if (buf.len() as u64) < (value_start as u64) {
                return Err(err_protocol!(
                    "expected 4 bytes at offset {offset}, got {}",
                    (value_start as u64) - (buf.len() as u64)
                ));
            }

            // Length of the column value, in bytes (this count does not include itself).
            // Can be zero. As a special case, -1 indicates a NULL column value.
            // No value bytes follow in the NULL case.
            //
            // we know `offset` is within range of `buf.len()` from the above check
            #[allow(clippy::cast_possible_truncation)]
            let length = BigEndian::read_i32(&buf[(offset as usize)..]);

            if let Ok(length) = u32::try_from(length) {
                let value_end = value_start.checked_add(length).ok_or_else(|| {
                    err_protocol!("value_start + length out of range ({offset} + {length})")
                })?;

                values.push(Some(value_start..value_end));
                offset = value_end;
            } else {
                // Negative values signify NULL
                values.push(None);
                // `value_start` is actually the next value now.
                offset = value_start;
            }
        }

        Ok(Self {
            storage: buf,
            values,
        })
    }
}

#[test]
fn test_decode_data_row() {
    const DATA: &[u8] = b"\
        \x00\x08\
        \xff\xff\xff\xff\
        \x00\x00\x00\x04\
        \x00\x00\x00\n\
        \xff\xff\xff\xff\
        \x00\x00\x00\x04\
        \x00\x00\x00\x14\
        \xff\xff\xff\xff\
        \x00\x00\x00\x04\
        \x00\x00\x00(\
        \xff\xff\xff\xff\
        \x00\x00\x00\x04\
        \x00\x00\x00P";

    let row = DataRow::decode_body(DATA.into()).unwrap();

    assert_eq!(row.values.len(), 8);

    assert!(row.get(0).is_none());
    assert_eq!(row.get(1).unwrap(), &[0_u8, 0, 0, 10][..]);
    assert!(row.get(2).is_none());
    assert_eq!(row.get(3).unwrap(), &[0_u8, 0, 0, 20][..]);
    assert!(row.get(4).is_none());
    assert_eq!(row.get(5).unwrap(), &[0_u8, 0, 0, 40][..]);
    assert!(row.get(6).is_none());
    assert_eq!(row.get(7).unwrap(), &[0_u8, 0, 0, 80][..]);
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_data_row_get(b: &mut test::Bencher) {
    const DATA: &[u8] = b"\x00\x08\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\n\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\x14\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00(\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00P";

    let row = DataRow::decode_body(test::black_box(Bytes::from_static(DATA))).unwrap();

    b.iter(|| {
        let _value = test::black_box(&row).get(3);
    });
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_decode_data_row(b: &mut test::Bencher) {
    const DATA: &[u8] = b"\x00\x08\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\n\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\x14\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00(\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00P";

    b.iter(|| {
        let _ = DataRow::decode_body(test::black_box(Bytes::from_static(DATA)));
    });
}
