use std::iter::FromIterator;

use http::Method;

use util::FlatCsv;

/// `Allow` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.4.1)
///
/// The `Allow` header field lists the set of methods advertised as
/// supported by the target resource.  The purpose of this field is
/// strictly to inform the recipient of valid request methods associated
/// with the resource.
///
/// # ABNF
///
/// ```text
/// Allow = #method
/// ```
///
/// # Example values
/// * `GET, HEAD, PUT`
/// * `OPTIONS, GET, PUT, POST, DELETE, HEAD, TRACE, CONNECT, PATCH, fOObAr`
/// * ``
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// extern crate http;
/// use headers::Allow;
/// use http::Method;
///
/// let allow = vec![Method::GET, Method::POST]
///     .into_iter()
///     .collect::<Allow>();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Allow(FlatCsv);

derive_header! {
    Allow(_),
    name: ALLOW
}

impl Allow {
    /// Returns an iterator over `Method`s contained within.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Method> + 'a {
        self.0.iter().filter_map(|s| s.parse().ok())
    }
}

impl FromIterator<Method> for Allow {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Method>,
    {
        let flat = iter
            .into_iter()
            .map(|method| {
                method
                    .as_str()
                    .parse::<::HeaderValue>()
                    .expect("Method is a valid HeaderValue")
            })
            .collect();
        Allow(flat)
    }
}
