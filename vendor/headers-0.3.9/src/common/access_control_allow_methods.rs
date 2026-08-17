use std::iter::FromIterator;

use http::Method;

use util::FlatCsv;

/// `Access-Control-Allow-Methods` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-allow-methods-response-header)
///
/// The `Access-Control-Allow-Methods` header indicates, as part of the
/// response to a preflight request, which methods can be used during the
/// actual request.
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Methods: "Access-Control-Allow-Methods" ":" #Method
/// ```
///
/// # Example values
/// * `PUT, DELETE, XMODIFY`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// extern crate http;
/// use http::Method;
/// use headers::AccessControlAllowMethods;
///
/// let allow_methods = vec![Method::GET, Method::PUT]
///     .into_iter()
///     .collect::<AccessControlAllowMethods>();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct AccessControlAllowMethods(FlatCsv);

derive_header! {
    AccessControlAllowMethods(_),
    name: ACCESS_CONTROL_ALLOW_METHODS
}

impl AccessControlAllowMethods {
    /// Returns an iterator over `Method`s contained within.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Method> + 'a {
        self.0.iter().filter_map(|s| s.parse().ok())
    }
}

impl FromIterator<Method> for AccessControlAllowMethods {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Method>,
    {
        let methods = iter
            .into_iter()
            .map(|method| {
                method
                    .as_str()
                    .parse::<::HeaderValue>()
                    .expect("Method is a valid HeaderValue")
            })
            .collect();

        AccessControlAllowMethods(methods)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn iter() {
        let allowed = test_decode::<AccessControlAllowMethods>(&["GET, PUT"]).unwrap();

        let as_vec = allowed.iter().collect::<Vec<_>>();
        assert_eq!(as_vec.len(), 2);
        assert_eq!(as_vec[0], Method::GET);
        assert_eq!(as_vec[1], Method::PUT);
    }

    #[test]
    fn from_iter() {
        let allow: AccessControlAllowMethods = vec![Method::GET, Method::PUT].into_iter().collect();

        let headers = test_encode(allow);
        assert_eq!(headers["access-control-allow-methods"], "GET, PUT");
    }
}
