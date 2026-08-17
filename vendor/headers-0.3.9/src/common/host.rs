use std::convert::TryFrom;
use std::fmt;

use http::uri::Authority;

/// The `Host` header.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub struct Host(Authority);

impl Host {
    /// Get the hostname, such as example.domain.
    pub fn hostname(&self) -> &str {
        self.0.host()
    }

    /// Get the optional port number.
    pub fn port(&self) -> Option<u16> {
        self.0.port_u16()
    }
}

impl ::Header for Host {
    fn name() -> &'static ::HeaderName {
        &::http::header::HOST
    }

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .next()
            .cloned()
            .and_then(|val| Authority::try_from(val.as_bytes()).ok())
            .map(Host)
            .ok_or_else(::Error::invalid)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        let bytes = self.0.as_str().as_bytes();
        let val = ::HeaderValue::from_bytes(bytes).expect("Authority is a valid HeaderValue");

        values.extend(::std::iter::once(val));
    }
}

impl From<Authority> for Host {
    fn from(auth: Authority) -> Host {
        Host(auth)
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
