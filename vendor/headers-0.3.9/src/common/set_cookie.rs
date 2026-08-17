/// `Set-Cookie` header, defined [RFC6265](http://tools.ietf.org/html/rfc6265#section-4.1)
///
/// The Set-Cookie HTTP response header is used to send cookies from the
/// server to the user agent.
///
/// Informally, the Set-Cookie response header contains the header name
/// "Set-Cookie" followed by a ":" and a cookie.  Each cookie begins with
/// a name-value-pair, followed by zero or more attribute-value pairs.
///
/// # ABNF
///
/// ```text
/// set-cookie-header = "Set-Cookie:" SP set-cookie-string
/// set-cookie-string = cookie-pair *( ";" SP cookie-av )
/// cookie-pair       = cookie-name "=" cookie-value
/// cookie-name       = token
/// cookie-value      = *cookie-octet / ( DQUOTE *cookie-octet DQUOTE )
/// cookie-octet      = %x21 / %x23-2B / %x2D-3A / %x3C-5B / %x5D-7E
///                       ; US-ASCII characters excluding CTLs,
///                       ; whitespace DQUOTE, comma, semicolon,
///                       ; and backslash
/// token             = <token, defined in [RFC2616], Section 2.2>
///
/// cookie-av         = expires-av / max-age-av / domain-av /
///                    path-av / secure-av / httponly-av /
///                     extension-av
/// expires-av        = "Expires=" sane-cookie-date
/// sane-cookie-date  = <rfc1123-date, defined in [RFC2616], Section 3.3.1>
/// max-age-av        = "Max-Age=" non-zero-digit *DIGIT
///                       ; In practice, both expires-av and max-age-av
///                       ; are limited to dates representable by the
///                       ; user agent.
/// non-zero-digit    = %x31-39
///                       ; digits 1 through 9
/// domain-av         = "Domain=" domain-value
/// domain-value      = <subdomain>
///                       ; defined in [RFC1034], Section 3.5, as
///                       ; enhanced by [RFC1123], Section 2.1
/// path-av           = "Path=" path-value
/// path-value        = <any CHAR except CTLs or ";">
/// secure-av         = "Secure"
/// httponly-av       = "HttpOnly"
/// extension-av      = <any CHAR except CTLs or ";">
/// ```
///
/// # Example values
///
/// * `SID=31d4d96e407aad42`
/// * `lang=en-US; Expires=Wed, 09 Jun 2021 10:18:14 GMT`
/// * `lang=; Expires=Sun, 06 Nov 1994 08:49:37 GMT`
/// * `lang=en-US; Path=/; Domain=example.com`
///
/// # Example
#[derive(Clone, Debug)]
pub struct SetCookie(Vec<::HeaderValue>);

impl ::Header for SetCookie {
    fn name() -> &'static ::HeaderName {
        &::http::header::SET_COOKIE
    }

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        let vec = values.cloned().collect::<Vec<_>>();

        if !vec.is_empty() {
            Ok(SetCookie(vec))
        } else {
            Err(::Error::invalid())
        }
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        values.extend(self.0.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::*;

    #[test]
    fn decode() {
        let set_cookie = test_decode::<SetCookie>(&["foo=bar", "baz=quux"]).unwrap();
        assert_eq!(set_cookie.0.len(), 2);
        assert_eq!(set_cookie.0[0], "foo=bar");
        assert_eq!(set_cookie.0[1], "baz=quux");
    }

    #[test]
    fn encode() {
        let set_cookie = SetCookie(vec![
            ::HeaderValue::from_static("foo=bar"),
            ::HeaderValue::from_static("baz=quux"),
        ]);

        let headers = test_encode(set_cookie);
        let mut vals = headers.get_all("set-cookie").into_iter();
        assert_eq!(vals.next().unwrap(), "foo=bar");
        assert_eq!(vals.next().unwrap(), "baz=quux");
        assert_eq!(vals.next(), None);
    }
}
