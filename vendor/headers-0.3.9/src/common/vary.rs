use util::FlatCsv;

use HeaderValue;

/// `Vary` header, defined in [RFC7231](https://tools.ietf.org/html/rfc7231#section-7.1.4)
///
/// The "Vary" header field in a response describes what parts of a
/// request message, aside from the method, Host header field, and
/// request target, might influence the origin server's process for
/// selecting and representing this response.  The value consists of
/// either a single asterisk ("*") or a list of header field names
/// (case-insensitive).
///
/// # ABNF
///
/// ```text
/// Vary = "*" / 1#field-name
/// ```
///
/// # Example values
///
/// * `accept-encoding, accept-language`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::Vary;
///
/// let vary = Vary::any();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Vary(FlatCsv);

derive_header! {
    Vary(_),
    name: VARY
}

impl Vary {
    /// Create a new `Very: *` header.
    pub fn any() -> Vary {
        Vary(HeaderValue::from_static("*").into())
    }

    /// Check if this includes `*`.
    pub fn is_any(&self) -> bool {
        self.0.iter().any(|val| val == "*")
    }

    /// Iterate the header names of this `Vary`.
    pub fn iter_strs(&self) -> impl Iterator<Item = &str> {
        self.0.iter()
    }
}

/*
test_vary {
    test_header!(test1, vec![b"accept-encoding, accept-language"]);

    #[test]
    fn test2() {
        let mut vary: ::Result<Vary>;

        vary = Header::parse_header(&"*".into());
        assert_eq!(vary.ok(), Some(Vary::Any));

        vary = Header::parse_header(&"etag,cookie,allow".into());
        assert_eq!(vary.ok(), Some(Vary::Items(vec!["eTag".parse().unwrap(),
                                                    "cookIE".parse().unwrap(),
                                                    "AlLOw".parse().unwrap(),])));
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_is_any() {
        assert!(Vary::any().is_any());
    }
}
