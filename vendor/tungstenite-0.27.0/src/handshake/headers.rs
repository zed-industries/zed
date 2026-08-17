//! HTTP Request and response header handling.

use http::header::{HeaderMap, HeaderName, HeaderValue};
use httparse::Status;

use super::machine::TryParse;
use crate::error::Result;

/// Limit for the number of header lines.
pub const MAX_HEADERS: usize = 124;

/// Trait to convert raw objects into HTTP parseables.
pub(crate) trait FromHttparse<T>: Sized {
    /// Convert raw object into parsed HTTP headers.
    fn from_httparse(raw: T) -> Result<Self>;
}

impl<'b: 'h, 'h> FromHttparse<&'b [httparse::Header<'h>]> for HeaderMap {
    fn from_httparse(raw: &'b [httparse::Header<'h>]) -> Result<Self> {
        let mut headers = HeaderMap::new();
        for h in raw {
            headers.append(
                HeaderName::from_bytes(h.name.as_bytes())?,
                HeaderValue::from_bytes(h.value)?,
            );
        }

        Ok(headers)
    }
}
impl TryParse for HeaderMap {
    fn try_parse(buf: &[u8]) -> Result<Option<(usize, Self)>> {
        let mut hbuffer = [httparse::EMPTY_HEADER; MAX_HEADERS];
        Ok(match httparse::parse_headers(buf, &mut hbuffer)? {
            Status::Partial => None,
            Status::Complete((size, hdr)) => Some((size, HeaderMap::from_httparse(hdr)?)),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::{super::machine::TryParse, HeaderMap};

    #[test]
    fn headers() {
        const DATA: &[u8] = b"Host: foo.com\r\n\
             Connection: Upgrade\r\n\
             Upgrade: websocket\r\n\
             \r\n";
        let (_, hdr) = HeaderMap::try_parse(DATA).unwrap().unwrap();
        assert_eq!(hdr.get("Host").unwrap(), &b"foo.com"[..]);
        assert_eq!(hdr.get("Upgrade").unwrap(), &b"websocket"[..]);
        assert_eq!(hdr.get("Connection").unwrap(), &b"Upgrade"[..]);
    }

    #[test]
    fn headers_iter() {
        const DATA: &[u8] = b"Host: foo.com\r\n\
              Sec-WebSocket-Extensions: permessage-deflate\r\n\
              Connection: Upgrade\r\n\
              Sec-WebSocket-ExtenSIONS: permessage-unknown\r\n\
              Upgrade: websocket\r\n\
              \r\n";
        let (_, hdr) = HeaderMap::try_parse(DATA).unwrap().unwrap();
        let mut iter = hdr.get_all("Sec-WebSocket-Extensions").iter();
        assert_eq!(iter.next().unwrap(), &b"permessage-deflate"[..]);
        assert_eq!(iter.next().unwrap(), &b"permessage-unknown"[..]);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn headers_incomplete() {
        const DATA: &[u8] = b"Host: foo.com\r\n\
              Connection: Upgrade\r\n\
              Upgrade: websocket\r\n";
        let hdr = HeaderMap::try_parse(DATA).unwrap();
        assert!(hdr.is_none());
    }
}
