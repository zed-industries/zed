use HeaderValue;

/// The `Pragma` header defined by HTTP/1.0.
///
/// > The "Pragma" header field allows backwards compatibility with
/// > HTTP/1.0 caches, so that clients can specify a "no-cache" request
/// > that they will understand (as Cache-Control was not defined until
/// > HTTP/1.1).  When the Cache-Control header field is also present and
/// > understood in a request, Pragma is ignored.
/// > In HTTP/1.0, Pragma was defined as an extensible field for
/// > implementation-specified directives for recipients.  This
/// > specification deprecates such extensions to improve interoperability.
///
/// Spec: [https://tools.ietf.org/html/rfc7234#section-5.4][url]
///
/// [url]: https://tools.ietf.org/html/rfc7234#section-5.4
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::Pragma;
///
/// let pragma = Pragma::no_cache();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Pragma(HeaderValue);

derive_header! {
    Pragma(_),
    name: PRAGMA
}

impl Pragma {
    /// Construct the literal `no-cache` Pragma header.
    pub fn no_cache() -> Pragma {
        Pragma(HeaderValue::from_static("no-cache"))
    }

    /// Return whether this pragma is `no-cache`.
    pub fn is_no_cache(&self) -> bool {
        self.0 == "no-cache"
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::Pragma;

    #[test]
    fn no_cache_is_no_cache() {
        assert!(Pragma::no_cache().is_no_cache());
    }

    #[test]
    fn etc_is_not_no_cache() {
        let ext = test_decode::<Pragma>(&["dexter"]).unwrap();
        assert!(!ext.is_no_cache());
    }
}
