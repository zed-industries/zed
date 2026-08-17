//! Authorization header and types.

use base64::engine::general_purpose::STANDARD as ENGINE;
use base64::Engine;
use bytes::Bytes;

use util::HeaderValueString;
use HeaderValue;

/// `Authorization` header, defined in [RFC7235](https://tools.ietf.org/html/rfc7235#section-4.2)
///
/// The `Authorization` header field allows a user agent to authenticate
/// itself with an origin server -- usually, but not necessarily, after
/// receiving a 401 (Unauthorized) response.  Its value consists of
/// credentials containing the authentication information of the user
/// agent for the realm of the resource being requested.
///
/// # ABNF
///
/// ```text
/// Authorization = credentials
/// ```
///
/// # Example values
/// * `Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==`
/// * `Bearer fpKL54jvWmEGVoRdCNjG`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::Authorization;
///
/// let basic = Authorization::basic("Aladdin", "open sesame");
/// let bearer = Authorization::bearer("some-opaque-token").unwrap();
/// ```
///
#[derive(Clone, PartialEq, Debug)]
pub struct Authorization<C: Credentials>(pub C);

impl Authorization<Basic> {
    /// Create a `Basic` authorization header.
    pub fn basic(username: &str, password: &str) -> Self {
        let colon_pos = username.len();
        let decoded = format!("{}:{}", username, password);

        Authorization(Basic { decoded, colon_pos })
    }

    /// View the decoded username.
    pub fn username(&self) -> &str {
        self.0.username()
    }

    /// View the decoded password.
    pub fn password(&self) -> &str {
        self.0.password()
    }
}

impl Authorization<Bearer> {
    /// Try to create a `Bearer` authorization header.
    pub fn bearer(token: &str) -> Result<Self, InvalidBearerToken> {
        HeaderValueString::from_string(format!("Bearer {}", token))
            .map(|val| Authorization(Bearer(val)))
            .ok_or_else(|| InvalidBearerToken { _inner: () })
    }

    /// View the token part as a `&str`.
    pub fn token(&self) -> &str {
        self.0.token()
    }
}

impl<C: Credentials> ::Header for Authorization<C> {
    fn name() -> &'static ::HeaderName {
        &::http::header::AUTHORIZATION
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .next()
            .and_then(|val| {
                let slice = val.as_bytes();
                if slice.starts_with(C::SCHEME.as_bytes())
                    && slice.len() > C::SCHEME.len()
                    && slice[C::SCHEME.len()] == b' '
                {
                    C::decode(val).map(Authorization)
                } else {
                    None
                }
            })
            .ok_or_else(::Error::invalid)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        let mut value = self.0.encode();
        value.set_sensitive(true);
        debug_assert!(
            value.as_bytes().starts_with(C::SCHEME.as_bytes()),
            "Credentials::encode should include its scheme: scheme = {:?}, encoded = {:?}",
            C::SCHEME,
            value,
        );

        values.extend(::std::iter::once(value));
    }
}

/// Credentials to be used in the `Authorization` header.
pub trait Credentials: Sized {
    /// The scheme identify the format of these credentials.
    ///
    /// This is the static string that always prefixes the actual credentials,
    /// like `"Basic"` in basic authorization.
    const SCHEME: &'static str;

    /// Try to decode the credentials from the `HeaderValue`.
    ///
    /// The `SCHEME` will be the first part of the `value`.
    fn decode(value: &HeaderValue) -> Option<Self>;

    /// Encode the credentials to a `HeaderValue`.
    ///
    /// The `SCHEME` must be the first part of the `value`.
    fn encode(&self) -> HeaderValue;
}

/// Credential holder for Basic Authentication
#[derive(Clone, PartialEq, Debug)]
pub struct Basic {
    decoded: String,
    colon_pos: usize,
}

impl Basic {
    /// View the decoded username.
    pub fn username(&self) -> &str {
        &self.decoded[..self.colon_pos]
    }

    /// View the decoded password.
    pub fn password(&self) -> &str {
        &self.decoded[self.colon_pos + 1..]
    }
}

impl Credentials for Basic {
    const SCHEME: &'static str = "Basic";

    fn decode(value: &HeaderValue) -> Option<Self> {
        debug_assert!(
            value.as_bytes().starts_with(b"Basic "),
            "HeaderValue to decode should start with \"Basic ..\", received = {:?}",
            value,
        );

        let bytes = &value.as_bytes()["Basic ".len()..];
        let non_space_pos = bytes.iter().position(|b| *b != b' ')?;
        let bytes = &bytes[non_space_pos..];

        let bytes = ENGINE.decode(bytes).ok()?;

        let decoded = String::from_utf8(bytes).ok()?;

        let colon_pos = decoded.find(':')?;

        Some(Basic { decoded, colon_pos })
    }

    fn encode(&self) -> HeaderValue {
        let mut encoded = String::from("Basic ");
        ENGINE.encode_string(&self.decoded, &mut encoded);

        let bytes = Bytes::from(encoded);
        HeaderValue::from_maybe_shared(bytes)
            .expect("base64 encoding is always a valid HeaderValue")
    }
}

#[derive(Clone, PartialEq, Debug)]
/// Token holder for Bearer Authentication, most often seen with oauth
pub struct Bearer(HeaderValueString);

impl Bearer {
    /// View the token part as a `&str`.
    pub fn token(&self) -> &str {
        &self.0.as_str()["Bearer ".len()..]
    }
}

impl Credentials for Bearer {
    const SCHEME: &'static str = "Bearer";

    fn decode(value: &HeaderValue) -> Option<Self> {
        debug_assert!(
            value.as_bytes().starts_with(b"Bearer "),
            "HeaderValue to decode should start with \"Bearer ..\", received = {:?}",
            value,
        );

        HeaderValueString::from_val(value).ok().map(Bearer)
    }

    fn encode(&self) -> HeaderValue {
        (&self.0).into()
    }
}

error_type!(InvalidBearerToken);

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::{Authorization, Basic, Bearer};
    use http::header::HeaderMap;
    use HeaderMapExt;

    #[test]
    fn basic_encode() {
        let auth = Authorization::basic("Aladdin", "open sesame");
        let headers = test_encode(auth);

        assert_eq!(
            headers["authorization"],
            "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==",
        );
    }

    #[test]
    fn basic_roundtrip() {
        let auth = Authorization::basic("Aladdin", "open sesame");
        let mut h = HeaderMap::new();
        h.typed_insert(auth.clone());
        assert_eq!(h.typed_get(), Some(auth));
    }

    #[test]
    fn basic_encode_no_password() {
        let auth = Authorization::basic("Aladdin", "");
        let headers = test_encode(auth);

        assert_eq!(headers["authorization"], "Basic QWxhZGRpbjo=",);
    }

    #[test]
    fn basic_decode() {
        let auth: Authorization<Basic> =
            test_decode(&["Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="]).unwrap();
        assert_eq!(auth.0.username(), "Aladdin");
        assert_eq!(auth.0.password(), "open sesame");
    }

    #[test]
    fn basic_decode_no_password() {
        let auth: Authorization<Basic> = test_decode(&["Basic QWxhZGRpbjo="]).unwrap();
        assert_eq!(auth.0.username(), "Aladdin");
        assert_eq!(auth.0.password(), "");
    }

    #[test]
    fn bearer_encode() {
        let auth = Authorization::bearer("fpKL54jvWmEGVoRdCNjG").unwrap();

        let headers = test_encode(auth);

        assert_eq!(headers["authorization"], "Bearer fpKL54jvWmEGVoRdCNjG",);
    }

    #[test]
    fn bearer_decode() {
        let auth: Authorization<Bearer> = test_decode(&["Bearer fpKL54jvWmEGVoRdCNjG"]).unwrap();
        assert_eq!(auth.0.token().as_bytes(), b"fpKL54jvWmEGVoRdCNjG");
    }
}

//bench_header!(raw, Authorization<String>, { vec![b"foo bar baz".to_vec()] });
//bench_header!(basic, Authorization<Basic>, { vec![b"Basic QWxhZGRpbjpuIHNlc2FtZQ==".to_vec()] });
//bench_header!(bearer, Authorization<Bearer>, { vec![b"Bearer fpKL54jvWmEGVoRdCNjG".to_vec()] });
