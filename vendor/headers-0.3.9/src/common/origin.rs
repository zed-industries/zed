use std::convert::TryFrom;
use std::fmt;

use bytes::Bytes;
use http::uri::{self, Authority, Scheme, Uri};

use util::{IterExt, TryFromValues};
use HeaderValue;

/// The `Origin` header.
///
/// The `Origin` header is a version of the `Referer` header that is used for all HTTP fetches and `POST`s whose CORS flag is set.
/// This header is often used to inform recipients of the security context of where the request was initiated.
///
/// Following the spec, [https://fetch.spec.whatwg.org/#origin-header][url], the value of this header is composed of
/// a String (scheme), Host (host/port)
///
/// [url]: https://fetch.spec.whatwg.org/#origin-header
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::Origin;
///
/// let origin = Origin::NULL;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Origin(OriginOrNull);

derive_header! {
    Origin(_),
    name: ORIGIN
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OriginOrNull {
    Origin(Scheme, Authority),
    Null,
}

impl Origin {
    /// The literal `null` Origin header.
    pub const NULL: Origin = Origin(OriginOrNull::Null);

    /// Checks if `Origin` is `null`.
    #[inline]
    pub fn is_null(&self) -> bool {
        match self.0 {
            OriginOrNull::Null => true,
            _ => false,
        }
    }

    /// Get the "scheme" part of this origin.
    #[inline]
    pub fn scheme(&self) -> &str {
        match self.0 {
            OriginOrNull::Origin(ref scheme, _) => scheme.as_str(),
            OriginOrNull::Null => "",
        }
    }

    /// Get the "hostname" part of this origin.
    #[inline]
    pub fn hostname(&self) -> &str {
        match self.0 {
            OriginOrNull::Origin(_, ref auth) => auth.host(),
            OriginOrNull::Null => "",
        }
    }

    /// Get the "port" part of this origin.
    #[inline]
    pub fn port(&self) -> Option<u16> {
        match self.0 {
            OriginOrNull::Origin(_, ref auth) => auth.port_u16(),
            OriginOrNull::Null => None,
        }
    }

    /// Tries to build a `Origin` from three parts, the scheme, the host and an optional port.
    pub fn try_from_parts(
        scheme: &str,
        host: &str,
        port: impl Into<Option<u16>>,
    ) -> Result<Self, InvalidOrigin> {
        struct MaybePort(Option<u16>);

        impl fmt::Display for MaybePort {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if let Some(port) = self.0 {
                    write!(f, ":{}", port)
                } else {
                    Ok(())
                }
            }
        }

        let bytes = Bytes::from(format!("{}://{}{}", scheme, host, MaybePort(port.into())));
        HeaderValue::from_maybe_shared(bytes)
            .ok()
            .and_then(|val| Self::try_from_value(&val))
            .ok_or_else(|| InvalidOrigin { _inner: () })
    }

    // Used in AccessControlAllowOrigin
    pub(super) fn try_from_value(value: &HeaderValue) -> Option<Self> {
        OriginOrNull::try_from_value(value).map(Origin)
    }

    pub(super) fn into_value(&self) -> HeaderValue {
        (&self.0).into()
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            OriginOrNull::Origin(ref scheme, ref auth) => write!(f, "{}://{}", scheme, auth),
            OriginOrNull::Null => f.write_str("null"),
        }
    }
}

error_type!(InvalidOrigin);

impl OriginOrNull {
    fn try_from_value(value: &HeaderValue) -> Option<Self> {
        if value == "null" {
            return Some(OriginOrNull::Null);
        }

        let uri = Uri::try_from(value.as_bytes()).ok()?;

        let (scheme, auth) = match uri.into_parts() {
            uri::Parts {
                scheme: Some(scheme),
                authority: Some(auth),
                path_and_query: None,
                ..
            } => (scheme, auth),
            uri::Parts {
                scheme: Some(ref scheme),
                authority: Some(ref auth),
                path_and_query: Some(ref p),
                ..
            } if p == "/" => (scheme.clone(), auth.clone()),
            _ => {
                return None;
            }
        };

        Some(OriginOrNull::Origin(scheme, auth))
    }
}

impl TryFromValues for OriginOrNull {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(OriginOrNull::try_from_value)
            .ok_or_else(::Error::invalid)
    }
}

impl<'a> From<&'a OriginOrNull> for HeaderValue {
    fn from(origin: &'a OriginOrNull) -> HeaderValue {
        match origin {
            OriginOrNull::Origin(ref scheme, ref auth) => {
                let s = format!("{}://{}", scheme, auth);
                let bytes = Bytes::from(s);
                HeaderValue::from_maybe_shared(bytes)
                    .expect("Scheme and Authority are valid header values")
            }
            // Serialized as "null" per ASCII serialization of an origin
            // https://html.spec.whatwg.org/multipage/browsers.html#ascii-serialisation-of-an-origin
            OriginOrNull::Null => HeaderValue::from_static("null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn origin() {
        let s = "http://web-platform.test:8000";
        let origin = test_decode::<Origin>(&[s]).unwrap();
        assert_eq!(origin.scheme(), "http");
        assert_eq!(origin.hostname(), "web-platform.test");
        assert_eq!(origin.port(), Some(8000));

        let headers = test_encode(origin);
        assert_eq!(headers["origin"], s);
    }

    #[test]
    fn null() {
        assert_eq!(test_decode::<Origin>(&["null"]), Some(Origin::NULL),);

        let headers = test_encode(Origin::NULL);
        assert_eq!(headers["origin"], "null");
    }
}
