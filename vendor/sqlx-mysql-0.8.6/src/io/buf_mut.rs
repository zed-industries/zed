use bytes::BufMut;

pub trait MySqlBufMutExt: BufMut {
    fn put_uint_lenenc(&mut self, v: u64);

    fn put_str_lenenc(&mut self, v: &str);

    fn put_bytes_lenenc(&mut self, v: &[u8]);
}

impl MySqlBufMutExt for Vec<u8> {
    fn put_uint_lenenc(&mut self, v: u64) {
        // https://dev.mysql.com/doc/internals/en/integer.html
        // https://mariadb.com/kb/en/library/protocol-data-types/#length-encoded-integers

        let encoded_le = v.to_le_bytes();

        match v {
            0..=250 => self.push(encoded_le[0]),
            251..=0xFF_FF => {
                self.push(0xfc);
                self.extend_from_slice(&encoded_le[..2]);
            }
            0x1_00_00..=0xFF_FF_FF => {
                self.push(0xfd);
                self.extend_from_slice(&encoded_le[..3]);
            }
            _ => {
                self.push(0xfe);
                self.extend_from_slice(&encoded_le);
            }
        }
    }

    fn put_str_lenenc(&mut self, v: &str) {
        self.put_bytes_lenenc(v.as_bytes());
    }

    fn put_bytes_lenenc(&mut self, v: &[u8]) {
        self.put_uint_lenenc(v.len() as u64);
        self.extend(v);
    }
}

#[test]
fn test_encodes_int_lenenc_u8() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(0xFA as u64);

    assert_eq!(&buf[..], b"\xFA");
}

#[test]
fn test_encodes_int_lenenc_u16() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(std::u16::MAX as u64);

    assert_eq!(&buf[..], b"\xFC\xFF\xFF");
}

#[test]
fn test_encodes_int_lenenc_u24() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(0xFF_FF_FF as u64);

    assert_eq!(&buf[..], b"\xFD\xFF\xFF\xFF");
}

#[test]
fn test_encodes_int_lenenc_u64() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(std::u64::MAX);

    assert_eq!(&buf[..], b"\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF");
}

#[test]
fn test_encodes_int_lenenc_fb() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(0xFB as u64);

    assert_eq!(&buf[..], b"\xFC\xFB\x00");
}

#[test]
fn test_encodes_int_lenenc_fc() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(0xFC as u64);

    assert_eq!(&buf[..], b"\xFC\xFC\x00");
}

#[test]
fn test_encodes_int_lenenc_fd() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(0xFD as u64);

    assert_eq!(&buf[..], b"\xFC\xFD\x00");
}

#[test]
fn test_encodes_int_lenenc_fe() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(0xFE as u64);

    assert_eq!(&buf[..], b"\xFC\xFE\x00");
}

#[test]
fn test_encodes_int_lenenc_ff() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_uint_lenenc(0xFF as u64);

    assert_eq!(&buf[..], b"\xFC\xFF\x00");
}

#[test]
fn test_encodes_string_lenenc() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_str_lenenc("random_string");

    assert_eq!(&buf[..], b"\x0Drandom_string");
}

#[test]
fn test_encodes_byte_lenenc() {
    let mut buf = Vec::with_capacity(1024);
    buf.put_bytes_lenenc(b"random_string");

    assert_eq!(&buf[..], b"\x0Drandom_string");
}
