use byteorder::{BigEndian, ByteOrder};
use sqlx_core::bytes::Bytes;

use crate::error::Error;
use crate::message::{BackendMessage, BackendMessageFormat};

/// Contains cancellation key data. The frontend must save these values if it
/// wishes to be able to issue `CancelRequest` messages later.
#[derive(Debug)]
pub struct BackendKeyData {
    /// The process ID of this database.
    pub process_id: u32,

    /// The secret key of this database.
    pub secret_key: u32,
}

impl BackendMessage for BackendKeyData {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::BackendKeyData;

    fn decode_body(buf: Bytes) -> Result<Self, Error> {
        let process_id = BigEndian::read_u32(&buf);
        let secret_key = BigEndian::read_u32(&buf[4..]);

        Ok(Self {
            process_id,
            secret_key,
        })
    }
}

#[test]
fn test_decode_backend_key_data() {
    const DATA: &[u8] = b"\0\0'\xc6\x89R\xc5+";

    let m = BackendKeyData::decode_body(DATA.into()).unwrap();

    assert_eq!(m.process_id, 10182);
    assert_eq!(m.secret_key, 2303903019);
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_decode_backend_key_data(b: &mut test::Bencher) {
    const DATA: &[u8] = b"\0\0'\xc6\x89R\xc5+";

    b.iter(|| {
        BackendKeyData::decode_body(test::black_box(Bytes::from_static(DATA))).unwrap();
    });
}
