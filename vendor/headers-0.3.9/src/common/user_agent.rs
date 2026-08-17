use std::fmt;
use std::str::FromStr;

use util::HeaderValueString;

/// `User-Agent` header, defined in
/// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.5.3)
///
/// The `User-Agent` header field contains information about the user
/// agent originating the request, which is often used by servers to help
/// identify the scope of reported interoperability problems, to work
/// around or tailor responses to avoid particular user agent
/// limitations, and for analytics regarding browser or operating system
/// use.  A user agent SHOULD send a User-Agent field in each request
/// unless specifically configured not to do so.
///
/// # ABNF
///
/// ```text
/// User-Agent = product *( RWS ( product / comment ) )
/// product         = token ["/" product-version]
/// product-version = token
/// ```
///
/// # Example values
///
/// * `CERN-LineMode/2.15 libwww/2.17b3`
/// * `Bunnies`
///
/// # Notes
///
/// * The parser does not split the value
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::UserAgent;
///
/// let ua = UserAgent::from_static("hyper/0.12.2");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserAgent(HeaderValueString);

derive_header! {
    UserAgent(_),
    name: USER_AGENT
}

impl UserAgent {
    /// Create a `UserAgent` from a static string.
    ///
    /// # Panic
    ///
    /// Panics if the static string is not a legal header value.
    pub fn from_static(src: &'static str) -> UserAgent {
        UserAgent(HeaderValueString::from_static(src))
    }

    /// View this `UserAgent` as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

error_type!(InvalidUserAgent);

impl FromStr for UserAgent {
    type Err = InvalidUserAgent;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        HeaderValueString::from_str(src)
            .map(UserAgent)
            .map_err(|_| InvalidUserAgent { _inner: () })
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
