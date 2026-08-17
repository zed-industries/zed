use std::fmt;

use mime::{self, Mime};

/// `Content-Type` header, defined in
/// [RFC7231](http://tools.ietf.org/html/rfc7231#section-3.1.1.5)
///
/// The `Content-Type` header field indicates the media type of the
/// associated representation: either the representation enclosed in the
/// message payload or the selected representation, as determined by the
/// message semantics.  The indicated media type defines both the data
/// format and how that data is intended to be processed by a recipient,
/// within the scope of the received message semantics, after any content
/// codings indicated by Content-Encoding are decoded.
///
/// Although the `mime` crate allows the mime options to be any slice, this crate
/// forces the use of Vec. This is to make sure the same header can't have more than 1 type. If
/// this is an issue, it's possible to implement `Header` on a custom struct.
///
/// # ABNF
///
/// ```text
/// Content-Type = media-type
/// ```
///
/// # Example values
///
/// * `text/html; charset=utf-8`
/// * `application/json`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::ContentType;
///
/// let ct = ContentType::json();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ContentType(Mime);

impl ContentType {
    /// A constructor  to easily create a `Content-Type: application/json` header.
    #[inline]
    pub fn json() -> ContentType {
        ContentType(mime::APPLICATION_JSON)
    }

    /// A constructor  to easily create a `Content-Type: text/plain` header.
    #[inline]
    pub fn text() -> ContentType {
        ContentType(mime::TEXT_PLAIN)
    }

    /// A constructor  to easily create a `Content-Type: text/plain; charset=utf-8` header.
    #[inline]
    pub fn text_utf8() -> ContentType {
        ContentType(mime::TEXT_PLAIN_UTF_8)
    }

    /// A constructor  to easily create a `Content-Type: text/html` header.
    #[inline]
    pub fn html() -> ContentType {
        ContentType(mime::TEXT_HTML)
    }

    /// A constructor  to easily create a `Content-Type: text/xml` header.
    #[inline]
    pub fn xml() -> ContentType {
        ContentType(mime::TEXT_XML)
    }

    /// A constructor  to easily create a `Content-Type: application/www-form-url-encoded` header.
    #[inline]
    pub fn form_url_encoded() -> ContentType {
        ContentType(mime::APPLICATION_WWW_FORM_URLENCODED)
    }
    /// A constructor  to easily create a `Content-Type: image/jpeg` header.
    #[inline]
    pub fn jpeg() -> ContentType {
        ContentType(mime::IMAGE_JPEG)
    }

    /// A constructor  to easily create a `Content-Type: image/png` header.
    #[inline]
    pub fn png() -> ContentType {
        ContentType(mime::IMAGE_PNG)
    }

    /// A constructor  to easily create a `Content-Type: application/octet-stream` header.
    #[inline]
    pub fn octet_stream() -> ContentType {
        ContentType(mime::APPLICATION_OCTET_STREAM)
    }
}

impl ::Header for ContentType {
    fn name() -> &'static ::HeaderName {
        &::http::header::CONTENT_TYPE
    }

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .next()
            .and_then(|v| v.to_str().ok()?.parse().ok())
            .map(ContentType)
            .ok_or_else(::Error::invalid)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        let value = self
            .0
            .as_ref()
            .parse()
            .expect("Mime is always a valid HeaderValue");
        values.extend(::std::iter::once(value));
    }
}

impl From<mime::Mime> for ContentType {
    fn from(m: mime::Mime) -> ContentType {
        ContentType(m)
    }
}

impl From<ContentType> for mime::Mime {
    fn from(ct: ContentType) -> mime::Mime {
        ct.0
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::str::FromStr for ContentType {
    type Err = ::Error;

    fn from_str(s: &str) -> Result<ContentType, Self::Err> {
        s.parse::<Mime>()
            .map(|m| m.into())
            .map_err(|_| ::Error::invalid())
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::ContentType;

    #[test]
    fn json() {
        assert_eq!(
            test_decode::<ContentType>(&["application/json"]),
            Some(ContentType::json()),
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "application/json".parse::<ContentType>().unwrap(),
            ContentType::json(),
        );
        assert!("invalid-mimetype".parse::<ContentType>().is_err());
    }

    bench_header!(bench_plain, ContentType, "text/plain");
    bench_header!(bench_json, ContentType, "application/json");
    bench_header!(
        bench_formdata,
        ContentType,
        "multipart/form-data; boundary=---------------abcd"
    );
}
