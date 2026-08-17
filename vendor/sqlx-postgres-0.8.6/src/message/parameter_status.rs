use sqlx_core::bytes::Bytes;

use crate::error::Error;
use crate::io::BufExt;
use crate::message::{BackendMessage, BackendMessageFormat};

#[derive(Debug)]
pub struct ParameterStatus {
    pub name: String,
    pub value: String,
}

impl BackendMessage for ParameterStatus {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::ParameterStatus;

    fn decode_body(mut buf: Bytes) -> Result<Self, Error> {
        let name = buf.get_str_nul()?;
        let value = buf.get_str_nul()?;

        Ok(Self { name, value })
    }
}

#[test]
fn test_decode_parameter_status() {
    const DATA: &[u8] = b"client_encoding\x00UTF8\x00";

    let m = ParameterStatus::decode_body(DATA.into()).unwrap();

    assert_eq!(&m.name, "client_encoding");
    assert_eq!(&m.value, "UTF8")
}

#[test]
fn test_decode_empty_parameter_status() {
    const DATA: &[u8] = b"\x00\x00";

    let m = ParameterStatus::decode_body(DATA.into()).unwrap();

    assert!(m.name.is_empty());
    assert!(m.value.is_empty());
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_decode_parameter_status(b: &mut test::Bencher) {
    const DATA: &[u8] = b"client_encoding\x00UTF8\x00";

    b.iter(|| {
        ParameterStatus::decode_body(test::black_box(Bytes::from_static(DATA))).unwrap();
    });
}

#[test]
fn test_decode_parameter_status_response() {
    const PARAMETER_STATUS_RESPONSE: &[u8] = b"crdb_version\0CockroachDB CCL v21.1.0 (x86_64-unknown-linux-gnu, built 2021/05/17 13:49:40, go1.15.11)\0";

    let message = ParameterStatus::decode_body(Bytes::from(PARAMETER_STATUS_RESPONSE)).unwrap();

    assert_eq!(message.name, "crdb_version");
    assert_eq!(
        message.value,
        "CockroachDB CCL v21.1.0 (x86_64-unknown-linux-gnu, built 2021/05/17 13:49:40, go1.15.11)"
    );
}
