use util::{FlatCsv, SemiColon};

/// `Cookie` header, defined in [RFC6265](http://tools.ietf.org/html/rfc6265#section-5.4)
///
/// If the user agent does attach a Cookie header field to an HTTP
/// request, the user agent must send the cookie-string
/// as the value of the header field.
///
/// When the user agent generates an HTTP request, the user agent MUST NOT
/// attach more than one Cookie header field.
///
/// # Example values
/// * `SID=31d4d96e407aad42`
/// * `SID=31d4d96e407aad42; lang=en-US`
///
#[derive(Clone, Debug)]
pub struct Cookie(FlatCsv<SemiColon>);

derive_header! {
    Cookie(_),
    name: COOKIE
}

impl Cookie {
    /// Lookup a value for a cookie name.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate headers;
    /// use headers::{Cookie, HeaderMap, HeaderMapExt, HeaderValue};
    ///
    /// // Setup the header map with strings...
    /// let mut headers = HeaderMap::new();
    /// headers.insert("cookie", HeaderValue::from_static("lang=en-US"));
    ///
    /// // Parse a `Cookie` so we can play with it...
    /// let cookie = headers
    ///     .typed_get::<Cookie>()
    ///     .expect("we just inserted a valid Cookie");
    ///
    /// assert_eq!(cookie.get("lang"), Some("en-US"));
    /// assert_eq!(cookie.get("SID"), None);
    /// ```
    pub fn get(&self, name: &str) -> Option<&str> {
        self.iter()
            .find(|&(key, _)| key == name)
            .map(|(_, val)| val)
    }

    /// Get the number of key-value pairs this `Cookie` contains.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Iterator the key-value pairs of this `Cookie` header.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().filter_map(|kv| {
            let mut iter = kv.splitn(2, '=');
            let key = iter.next()?.trim();
            let val = iter.next()?.trim();
            Some((key, val))
        })
    }
}

/*
impl PartialEq for Cookie {
    fn eq(&self, other: &Cookie) -> bool {
        if self.0.len() == other.0.len() {
            for &(ref k, ref v) in self.0.iter() {
                if other.get(k) != Some(v) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::Cookie;

    #[test]
    fn test_parse() {
        let cookie = test_decode::<Cookie>(&["foo=bar"]).unwrap();

        assert_eq!(cookie.get("foo"), Some("bar"));
        assert_eq!(cookie.get("bar"), None);
    }

    #[test]
    fn test_multipe_same_name() {
        let cookie = test_decode::<Cookie>(&["foo=bar; foo=baz"]).unwrap();

        assert_eq!(cookie.get("foo"), Some("bar"));
    }

    #[test]
    fn test_multipe_lines() {
        let cookie = test_decode::<Cookie>(&["foo=bar", "lol = cat"]).unwrap();

        assert_eq!(cookie.get("foo"), Some("bar"));
        assert_eq!(cookie.get("lol"), Some("cat"));
    }

    /*
    #[test]
    fn test_set_and_get() {
        let mut cookie = Cookie::new();
        cookie.append("foo", "bar");
        cookie.append(String::from("dyn"), String::from("amic"));

        assert_eq!(cookie.get("foo"), Some("bar"));
        assert_eq!(cookie.get("dyn"), Some("amic"));
        assert!(cookie.get("nope").is_none());

        cookie.append("foo", "notbar");
        assert_eq!(cookie.get("foo"), Some("bar"));

        cookie.set("foo", "hi");
        assert_eq!(cookie.get("foo"), Some("hi"));
        assert_eq!(cookie.get("dyn"), Some("amic"));
    }

    #[test]
    fn test_eq() {
        let mut cookie = Cookie::new();
        let mut cookie2 = Cookie::new();

        // empty is equal
        assert_eq!(cookie, cookie2);

        // left has more params
        cookie.append("foo", "bar");
        assert_ne!(cookie, cookie2);

        // same len, different params
        cookie2.append("bar", "foo");
        assert_ne!(cookie, cookie2);


        // right has more params, and matching KV
        cookie2.append("foo", "bar");
        assert_ne!(cookie, cookie2);

        // same params, different order
        cookie.append("bar", "foo");
        assert_eq!(cookie, cookie2);
    }

    #[test]
    fn test_parse() {
        let mut cookie = Cookie::new();

        let parsed = Cookie::parse_header(&b"foo=bar".to_vec().into()).unwrap();
        cookie.append("foo", "bar");
        assert_eq!(cookie, parsed);

        let parsed = Cookie::parse_header(&b"foo=bar;".to_vec().into()).unwrap();
        assert_eq!(cookie, parsed);

        let parsed = Cookie::parse_header(&b"foo=bar; baz=quux".to_vec().into()).unwrap();
        cookie.append("baz", "quux");
        assert_eq!(cookie, parsed);

        let parsed = Cookie::parse_header(&b"foo=bar;; baz=quux".to_vec().into()).unwrap();
        assert_eq!(cookie, parsed);

        let parsed = Cookie::parse_header(&b"foo=bar; invalid ; bad; ;; baz=quux".to_vec().into())
            .unwrap();
        assert_eq!(cookie, parsed);

        let parsed = Cookie::parse_header(&b" foo  =    bar;baz= quux  ".to_vec().into()).unwrap();
        assert_eq!(cookie, parsed);

        let parsed =
            Cookie::parse_header(&vec![b"foo  =    bar".to_vec(), b"baz= quux  ".to_vec()].into())
                .unwrap();
        assert_eq!(cookie, parsed);

        let parsed = Cookie::parse_header(&b"foo=bar; baz=quux ; empty=".to_vec().into()).unwrap();
        cookie.append("empty", "");
        assert_eq!(cookie, parsed);


        let mut cookie = Cookie::new();

        let parsed = Cookie::parse_header(&b"middle=equals=in=the=middle".to_vec().into()).unwrap();
        cookie.append("middle", "equals=in=the=middle");
        assert_eq!(cookie, parsed);

        let parsed =
            Cookie::parse_header(&b"middle=equals=in=the=middle; double==2".to_vec().into())
                .unwrap();
        cookie.append("double", "=2");
        assert_eq!(cookie, parsed);
    }
    */
}
