use std::convert::TryFrom;

use super::origin::Origin;
use util::{IterExt, TryFromValues};
use HeaderValue;

/// The `Access-Control-Allow-Origin` response header,
/// part of [CORS](http://www.w3.org/TR/cors/#access-control-allow-origin-response-header)
///
/// The `Access-Control-Allow-Origin` header indicates whether a resource
/// can be shared based by returning the value of the Origin request header,
/// `*`, or `null` in the response.
///
/// ## ABNF
///
/// ```text
/// Access-Control-Allow-Origin = "Access-Control-Allow-Origin" ":" origin-list-or-null | "*"
/// ```
///
/// ## Example values
/// * `null`
/// * `*`
/// * `http://google.com/`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::AccessControlAllowOrigin;
/// use std::convert::TryFrom;
///
/// let any_origin = AccessControlAllowOrigin::ANY;
/// let null_origin = AccessControlAllowOrigin::NULL;
/// let origin = AccessControlAllowOrigin::try_from("http://web-platform.test:8000");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControlAllowOrigin(OriginOrAny);

derive_header! {
    AccessControlAllowOrigin(_),
    name: ACCESS_CONTROL_ALLOW_ORIGIN
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OriginOrAny {
    Origin(Origin),
    /// Allow all origins
    Any,
}

impl AccessControlAllowOrigin {
    /// `Access-Control-Allow-Origin: *`
    pub const ANY: AccessControlAllowOrigin = AccessControlAllowOrigin(OriginOrAny::Any);
    /// `Access-Control-Allow-Origin: null`
    pub const NULL: AccessControlAllowOrigin =
        AccessControlAllowOrigin(OriginOrAny::Origin(Origin::NULL));

    /// Returns the origin if there's one specified.
    pub fn origin(&self) -> Option<&Origin> {
        match self.0 {
            OriginOrAny::Origin(ref origin) => Some(origin),
            _ => None,
        }
    }
}

impl TryFrom<&str> for AccessControlAllowOrigin {
    type Error = ::Error;

    fn try_from(s: &str) -> Result<Self, ::Error> {
        let header_value = HeaderValue::from_str(s).map_err(|_| ::Error::invalid())?;
        let origin = OriginOrAny::try_from(&header_value)?;
        Ok(Self(origin))
    }
}

impl TryFrom<&HeaderValue> for OriginOrAny {
    type Error = ::Error;

    fn try_from(header_value: &HeaderValue) -> Result<Self, ::Error> {
        Origin::try_from_value(header_value)
            .map(OriginOrAny::Origin)
            .ok_or_else(::Error::invalid)
    }
}

impl TryFromValues for OriginOrAny {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(|value| {
                if value == "*" {
                    return Some(OriginOrAny::Any);
                }

                Origin::try_from_value(value).map(OriginOrAny::Origin)
            })
            .ok_or_else(::Error::invalid)
    }
}

impl<'a> From<&'a OriginOrAny> for HeaderValue {
    fn from(origin: &'a OriginOrAny) -> HeaderValue {
        match origin {
            OriginOrAny::Origin(ref origin) => origin.into_value(),
            OriginOrAny::Any => HeaderValue::from_static("*"),
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

        let allow_origin = test_decode::<AccessControlAllowOrigin>(&[s]).unwrap();
        {
            let origin = allow_origin.origin().unwrap();
            assert_eq!(origin.scheme(), "http");
            assert_eq!(origin.hostname(), "web-platform.test");
            assert_eq!(origin.port(), Some(8000));
        }

        let headers = test_encode(allow_origin);
        assert_eq!(headers["access-control-allow-origin"], s);
    }

    #[test]
    fn try_from_origin() {
        let s = "http://web-platform.test:8000";

        let allow_origin = AccessControlAllowOrigin::try_from(s).unwrap();
        {
            let origin = allow_origin.origin().unwrap();
            assert_eq!(origin.scheme(), "http");
            assert_eq!(origin.hostname(), "web-platform.test");
            assert_eq!(origin.port(), Some(8000));
        }

        let headers = test_encode(allow_origin);
        assert_eq!(headers["access-control-allow-origin"], s);
    }

    #[test]
    fn any() {
        let allow_origin = test_decode::<AccessControlAllowOrigin>(&["*"]).unwrap();
        assert_eq!(allow_origin, AccessControlAllowOrigin::ANY);

        let headers = test_encode(allow_origin);
        assert_eq!(headers["access-control-allow-origin"], "*");
    }

    #[test]
    fn null() {
        let allow_origin = test_decode::<AccessControlAllowOrigin>(&["null"]).unwrap();
        assert_eq!(allow_origin, AccessControlAllowOrigin::NULL);

        let headers = test_encode(allow_origin);
        assert_eq!(headers["access-control-allow-origin"], "null");
    }
}
