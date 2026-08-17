use std::iter::FromIterator;

use util::FlatCsv;
use {HeaderName, HeaderValue};

/// `Access-Control-Request-Headers` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-request-headers-request-header)
///
/// The `Access-Control-Request-Headers` header indicates which headers will
/// be used in the actual request as part of the preflight request.
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
/// # fn main() {
/// use http::header::{ACCEPT_LANGUAGE, DATE};
/// use headers::AccessControlRequestHeaders;
///
/// let req_headers = vec![ACCEPT_LANGUAGE, DATE]
///     .into_iter()
///     .collect::<AccessControlRequestHeaders>();
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct AccessControlRequestHeaders(FlatCsv);

derive_header! {
    AccessControlRequestHeaders(_),
    name: ACCESS_CONTROL_REQUEST_HEADERS
}

impl AccessControlRequestHeaders {
    /// Returns an iterator over `HeaderName`s contained within.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = HeaderName> + 'a {
        self.0.iter().filter_map(|s| s.parse().ok())
    }
}

impl FromIterator<HeaderName> for AccessControlRequestHeaders {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = HeaderName>,
    {
        let flat = iter.into_iter().map(HeaderValue::from).collect();
        AccessControlRequestHeaders(flat)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn iter() {
        let req_headers = test_decode::<AccessControlRequestHeaders>(&["foo, bar"]).unwrap();

        let as_vec = req_headers.iter().collect::<Vec<_>>();
        assert_eq!(as_vec.len(), 2);
        assert_eq!(as_vec[0], "foo");
        assert_eq!(as_vec[1], "bar");
    }

    #[test]
    fn from_iter() {
        let req_headers: AccessControlRequestHeaders =
            vec![::http::header::CACHE_CONTROL, ::http::header::IF_RANGE]
                .into_iter()
                .collect();

        let headers = test_encode(req_headers);
        assert_eq!(
            headers["access-control-request-headers"],
            "cache-control, if-range"
        );
    }
}
