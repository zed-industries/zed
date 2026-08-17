use crate::io::ProtocolEncode;

pub struct SslRequest;

impl SslRequest {
    // https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-SSLREQUEST
    pub const BYTES: &'static [u8] = b"\x00\x00\x00\x08\x04\xd2\x16\x2f";
}

// Cannot impl FrontendMessage because it does not have a format code
impl ProtocolEncode<'_> for SslRequest {
    #[inline(always)]
    fn encode_with(&self, buf: &mut Vec<u8>, _context: ()) -> Result<(), crate::Error> {
        buf.extend_from_slice(Self::BYTES);
        Ok(())
    }
}

#[test]
fn test_encode_ssl_request() {
    let mut buf = Vec::new();

    // Int32(8)
    // Length of message contents in bytes, including self.
    buf.extend_from_slice(&8_u32.to_be_bytes());

    // Int32(80877103)
    // The SSL request code. The value is chosen to contain 1234 in the most significant 16 bits,
    // and 5679 in the least significant 16 bits.
    // (To avoid confusion, this code must not be the same as any protocol version number.)
    buf.extend_from_slice(&(((1234 << 16) | 5679) as u32).to_be_bytes());

    let mut encoded = Vec::new();
    SslRequest.encode(&mut encoded).unwrap();

    assert_eq!(buf, SslRequest::BYTES);
    assert_eq!(buf, encoded);
}
