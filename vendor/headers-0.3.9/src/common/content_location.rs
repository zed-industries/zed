use HeaderValue;

/// `Content-Location` header, defined in
/// [RFC7231](https://tools.ietf.org/html/rfc7231#section-3.1.4.2)
///
/// The header can be used by both the client in requests and the server
/// in responses with different semantics. Client sets `Content-Location`
/// to refer to the URI where original representation of the body was
/// obtained.
///
/// In responses `Content-Location` represents URI for the representation
/// that was content negotiated, created or for the response payload.
///
/// # ABNF
///
/// ```text
/// Content-Location = absolute-URI / partial-URI
/// ```
///
/// # Example values
///
/// * `/hypertext/Overview.html`
/// * `http://www.example.org/hypertext/Overview.html`
///
/// # Examples
///
#[derive(Clone, Debug, PartialEq)]
pub struct ContentLocation(HeaderValue);

derive_header! {
    ContentLocation(_),
    name: CONTENT_LOCATION
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::*;

    #[test]
    fn absolute_uri() {
        let s = "http://www.example.net/index.html";
        let loc = test_decode::<ContentLocation>(&[s]).unwrap();

        assert_eq!(loc, ContentLocation(HeaderValue::from_static(s)));
    }

    #[test]
    fn relative_uri_with_fragment() {
        let s = "/People.html#tim";
        let loc = test_decode::<ContentLocation>(&[s]).unwrap();

        assert_eq!(loc, ContentLocation(HeaderValue::from_static(s)));
    }
}
