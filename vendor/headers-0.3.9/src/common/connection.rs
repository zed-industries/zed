use std::iter::FromIterator;

use self::sealed::AsConnectionOption;
use util::FlatCsv;
use {HeaderName, HeaderValue};

/// `Connection` header, defined in
/// [RFC7230](http://tools.ietf.org/html/rfc7230#section-6.1)
///
/// The `Connection` header field allows the sender to indicate desired
/// control options for the current connection.  In order to avoid
/// confusing downstream recipients, a proxy or gateway MUST remove or
/// replace any received connection options before forwarding the
/// message.
///
/// # ABNF
///
/// ```text
/// Connection        = 1#connection-option
/// connection-option = token
///
/// # Example values
/// * `close`
/// * `keep-alive`
/// * `upgrade`
/// ```
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::Connection;
///
/// let keep_alive = Connection::keep_alive();
/// ```
// This is frequently just 1 or 2 values, so optimize for that case.
#[derive(Clone, Debug)]
pub struct Connection(FlatCsv);

derive_header! {
    Connection(_),
    name: CONNECTION
}

impl Connection {
    /// A constructor to easily create a `Connection: close` header.
    #[inline]
    pub fn close() -> Connection {
        Connection(HeaderValue::from_static("close").into())
    }

    /// A constructor to easily create a `Connection: keep-alive` header.
    #[inline]
    pub fn keep_alive() -> Connection {
        Connection(HeaderValue::from_static("keep-alive").into())
    }

    /// A constructor to easily create a `Connection: Upgrade` header.
    #[inline]
    pub fn upgrade() -> Connection {
        Connection(HeaderValue::from_static("upgrade").into())
    }

    /// Check if this header contains a given "connection option".
    ///
    /// This can be used with various argument types:
    ///
    /// - `&str`
    /// - `&HeaderName`
    /// - `HeaderName`
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate headers;
    /// extern crate http;
    ///
    /// use http::header::UPGRADE;
    /// use headers::Connection;
    ///
    /// let conn = Connection::keep_alive();
    ///
    /// assert!(!conn.contains("close"));
    /// assert!(!conn.contains(UPGRADE));
    /// assert!(conn.contains("keep-alive"));
    /// assert!(conn.contains("Keep-Alive"));
    /// ```
    pub fn contains(&self, name: impl AsConnectionOption) -> bool {
        let s = name.as_connection_option();
        self.0
            .iter()
            .find(|&opt| opt.eq_ignore_ascii_case(s))
            .is_some()
    }
}

impl FromIterator<HeaderName> for Connection {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = HeaderName>,
    {
        let flat = iter.into_iter().map(HeaderValue::from).collect();
        Connection(flat)
    }
}

mod sealed {
    pub trait AsConnectionOption: Sealed {
        fn as_connection_option(&self) -> &str;
    }
    pub trait Sealed {}

    impl<'a> AsConnectionOption for &'a str {
        fn as_connection_option(&self) -> &str {
            *self
        }
    }

    impl<'a> Sealed for &'a str {}

    impl<'a> AsConnectionOption for &'a ::HeaderName {
        fn as_connection_option(&self) -> &str {
            self.as_ref()
        }
    }

    impl<'a> Sealed for &'a ::HeaderName {}

    impl AsConnectionOption for ::HeaderName {
        fn as_connection_option(&self) -> &str {
            self.as_ref()
        }
    }

    impl Sealed for ::HeaderName {}
}
