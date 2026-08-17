use std::iter::FromIterator;

use util::FlatCsv;
use {HeaderName, HeaderValue};

/// `Access-Control-Allow-Headers` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)
///
/// The `Access-Control-Allow-Headers` header indicates, as part of the
/// response to a preflight request, which header field names can be used
/// during the actual request.
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Headers: "Access-Control-Allow-Headers" ":" #field-name
/// ```
///
/// # Example values
/// * `accept-language, date`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// extern crate http;
/// use http::header::{CACHE_CONTROL, CONTENT_TYPE};
/// use headers::AccessControlAllowHeaders;
///
/// let allow_headers = vec![CACHE_CONTROL, CONTENT_TYPE]
///     .into_iter()
///     .collect::<AccessControlAllowHeaders>();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct AccessControlAllowHeaders(FlatCsv);

derive_header! {
    AccessControlAllowHeaders(_),
    name: ACCESS_CONTROL_ALLOW_HEADERS
}

impl AccessControlAllowHeaders {
    /// Returns an iterator over `HeaderName`s contained within.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = HeaderName> + 'a {
        self.0
            .iter()
            .map(|s| s.parse().ok())
            .take_while(|val| val.is_some())
            .filter_map(|val| val)
    }
}

impl FromIterator<HeaderName> for AccessControlAllowHeaders {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = HeaderName>,
    {
        let flat = iter.into_iter().map(HeaderValue::from).collect();
        AccessControlAllowHeaders(flat)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn iter() {
        let allow_headers = test_decode::<AccessControlAllowHeaders>(&["foo, bar"]).unwrap();

        let as_vec = allow_headers.iter().collect::<Vec<_>>();
        assert_eq!(as_vec.len(), 2);
        assert_eq!(as_vec[0], "foo");
        assert_eq!(as_vec[1], "bar");
    }

    #[test]
    fn from_iter() {
        let allow: AccessControlAllowHeaders =
            vec![::http::header::CACHE_CONTROL, ::http::header::IF_RANGE]
                .into_iter()
                .collect();

        let headers = test_encode(allow);
        assert_eq!(
            headers["access-control-allow-headers"],
            "cache-control, if-range"
        );
    }

    #[test]
    fn test_with_invalid() {
        let allow_headers = test_decode::<AccessControlAllowHeaders>(&["foo foo, bar"]).unwrap();

        assert!(allow_headers.iter().collect::<Vec<_>>().is_empty());
    }
}
