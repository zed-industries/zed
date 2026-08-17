use {Header, HeaderValue};

/// `Content-Length` header, defined in
/// [RFC7230](http://tools.ietf.org/html/rfc7230#section-3.3.2)
///
/// When a message does not have a `Transfer-Encoding` header field, a
/// Content-Length header field can provide the anticipated size, as a
/// decimal number of octets, for a potential payload body.  For messages
/// that do include a payload body, the Content-Length field-value
/// provides the framing information necessary for determining where the
/// body (and message) ends.  For messages that do not include a payload
/// body, the Content-Length indicates the size of the selected
/// representation.
///
/// Note that setting this header will *remove* any previously set
/// `Transfer-Encoding` header, in accordance with
/// [RFC7230](http://tools.ietf.org/html/rfc7230#section-3.3.2):
///
/// > A sender MUST NOT send a Content-Length header field in any message
/// > that contains a Transfer-Encoding header field.
///
/// ## ABNF
///
/// ```text
/// Content-Length = 1*DIGIT
/// ```
///
/// ## Example values
///
/// * `3495`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::ContentLength;
///
/// let len = ContentLength(1_000);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContentLength(pub u64);

impl Header for ContentLength {
    fn name() -> &'static ::http::header::HeaderName {
        &::http::header::CONTENT_LENGTH
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        // If multiple Content-Length headers were sent, everything can still
        // be alright if they all contain the same value, and all parse
        // correctly. If not, then it's an error.
        let mut len = None;
        for value in values {
            let parsed = value
                .to_str()
                .map_err(|_| ::Error::invalid())?
                .parse::<u64>()
                .map_err(|_| ::Error::invalid())?;

            if let Some(prev) = len {
                if prev != parsed {
                    return Err(::Error::invalid());
                }
            } else {
                len = Some(parsed);
            }
        }

        len.map(ContentLength).ok_or_else(::Error::invalid)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        values.extend(::std::iter::once(self.0.into()));
    }
}

/*
__hyper__tm!(ContentLength, tests {
    // Testcase from RFC
    test_header!(test1, vec![b"3495"], Some(HeaderField(3495)));

    test_header!(test_invalid, vec![b"34v95"], None);

    // Can't use the test_header macro because "5, 5" gets cleaned to "5".
    #[test]
    fn test_duplicates() {
        let parsed = HeaderField::parse_header(&vec![b"5".to_vec(),
                                                 b"5".to_vec()].into()).unwrap();
        assert_eq!(parsed, HeaderField(5));
        assert_eq!(format!("{}", parsed), "5");
    }

    test_header!(test_duplicates_vary, vec![b"5", b"6", b"5"], None);
});
*/
