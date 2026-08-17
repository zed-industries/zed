use crate::io::BufMutExt;
use crate::message::{FrontendMessage, FrontendMessageFormat};
use sqlx_core::Error;
use std::num::Saturating;

#[derive(Debug)]
pub struct Query<'a>(pub &'a str);

impl FrontendMessage for Query<'_> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Query;

    fn body_size_hint(&self) -> Saturating<usize> {
        let mut size = Saturating(0);

        size += self.0.len();
        size += 1; // NUL terminator

        size
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        buf.put_str_nul(self.0);
        Ok(())
    }
}

#[test]
fn test_encode_query() {
    const EXPECTED: &[u8] = b"Q\0\0\0\x0DSELECT 1\0";

    let mut buf = Vec::new();
    let m = Query("SELECT 1");

    m.encode_msg(&mut buf).unwrap();

    assert_eq!(buf, EXPECTED);
}
