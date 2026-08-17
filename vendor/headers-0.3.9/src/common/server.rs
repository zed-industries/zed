use std::fmt;
use std::str::FromStr;

use util::HeaderValueString;

/// `Server` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.4.2)
///
/// The `Server` header field contains information about the software
/// used by the origin server to handle the request, which is often used
/// by clients to help identify the scope of reported interoperability
/// problems, to work around or tailor requests to avoid particular
/// server limitations, and for analytics regarding server or operating
/// system use.  An origin server MAY generate a Server field in its
/// responses.
///
/// # ABNF
///
/// ```text
/// Server = product *( RWS ( product / comment ) )
/// ```
///
/// # Example values
/// * `CERN/3.0 libwww/2.17`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::Server;
///
/// let server = Server::from_static("hyper/0.12.2");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Server(HeaderValueString);

derive_header! {
    Server(_),
    name: SERVER
}

impl Server {
    /// Construct a `Server` from a static string.
    ///
    /// # Panic
    ///
    /// Panics if the static string is not a legal header value.
    pub fn from_static(s: &'static str) -> Server {
        Server(HeaderValueString::from_static(s))
    }

    /// View this `Server` as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

error_type!(InvalidServer);

impl FromStr for Server {
    type Err = InvalidServer;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        HeaderValueString::from_str(src)
            .map(Server)
            .map_err(|_| InvalidServer { _inner: () })
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
