use std::fmt;
use std::str::FromStr;
use {Header, Raw};
use parsing::{from_comma_delimited, fmt_comma_delimited};

/// `Prefer` header, defined in [RFC7240](http://tools.ietf.org/html/rfc7240)
///
/// The `Prefer` header field can be used by a client to request that certain
/// behaviors be employed by a server while processing a request.
///
/// # ABNF
///
/// ```text
/// Prefer     = "Prefer" ":" 1#preference
/// preference = token [ BWS "=" BWS word ]
///              *( OWS ";" [ OWS parameter ] )
/// parameter  = token [ BWS "=" BWS word ]
/// ```
///
/// # Example values
/// * `respond-async`
/// * `return=minimal`
/// * `wait=30`
///
/// # Examples
///
/// ```
/// use headers::{Headers, Prefer, Preference};
///
/// let mut headers = Headers::new();
/// headers.set(
///     Prefer(vec![Preference::RespondAsync])
/// );
/// ```
///
/// ```
/// use headers::{Headers, Prefer, Preference};
///
/// let mut headers = Headers::new();
/// headers.set(
///     Prefer(vec![
///         Preference::RespondAsync,
///         Preference::ReturnRepresentation,
///         Preference::Wait(10u32),
///         Preference::Extension("foo".to_owned(),
///                               "bar".to_owned(),
///                               vec![]),
///     ])
/// );
/// ```
#[derive(PartialEq, Clone, Debug)]
pub struct Prefer(pub Vec<Preference>);

__hyper__deref!(Prefer => Vec<Preference>);

impl Header for Prefer {
    fn header_name() -> &'static str {
        static NAME: &'static str = "Prefer";
        NAME
    }

    fn parse_header(raw: &Raw) -> ::Result<Prefer> {
        let preferences = try!(from_comma_delimited(raw));
        if !preferences.is_empty() {
            Ok(Prefer(preferences))
        } else {
            Err(::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut ::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl fmt::Display for Prefer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_comma_delimited(f, &self[..])
    }
}

/// Prefer contains a list of these preferences.
#[derive(PartialEq, Clone, Debug)]
pub enum Preference {
    /// "respond-async"
    RespondAsync,
    /// "return=representation"
    ReturnRepresentation,
    /// "return=minimal"
    ReturnMinimal,
    /// "handling=strict"
    HandlingStrict,
    /// "handling=lenient"
    HandlingLenient,
    /// "wait=delta"
    Wait(u32),

    /// Extension preferences. Always has a value, if none is specified it is
    /// just "". A preference can also have a list of parameters.
    Extension(String, String, Vec<(String, String)>)
}

impl fmt::Display for Preference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Preference::*;
        fmt::Display::fmt(match *self {
            RespondAsync => "respond-async",
            ReturnRepresentation => "return=representation",
            ReturnMinimal => "return=minimal",
            HandlingStrict => "handling=strict",
            HandlingLenient => "handling=lenient",

            Wait(secs) => return write!(f, "wait={}", secs),

            Extension(ref name, ref value, ref params) => {
                try!(write!(f, "{}", name));
                if value != "" { try!(write!(f, "={}", value)); }
                if !params.is_empty() {
                    for &(ref name, ref value) in params {
                        try!(write!(f, "; {}", name));
                        if value != "" { try!(write!(f, "={}", value)); }
                    }
                }
                return Ok(());
            }
        }, f)
    }
}

impl FromStr for Preference {
    type Err = Option<<u32 as FromStr>::Err>;
    fn from_str(s: &str) -> Result<Preference, Option<<u32 as FromStr>::Err>> {
        use self::Preference::*;
        let mut params = s.split(';').map(|p| {
            let mut param = p.splitn(2, '=');
            match (param.next(), param.next()) {
                (Some(name), Some(value)) => (name.trim(), value.trim().trim_matches('"')),
                (Some(name), None) => (name.trim(), ""),
                // This can safely be unreachable because the [`splitn`][1]
                // function (used above) will always have at least one value.
                //
                // [1]: http://doc.rust-lang.org/std/primitive.str.html#method.splitn
                _ => { unreachable!(); }
            }
        });
        match params.nth(0) {
            Some(param) => {
                let rest: Vec<(String, String)> = params.map(|(l, r)| (l.to_owned(), r.to_owned())).collect();
                match param {
                    ("respond-async", "") => if rest.is_empty() { Ok(RespondAsync) } else { Err(None) },
                    ("return", "representation") => if rest.is_empty() { Ok(ReturnRepresentation) } else { Err(None) },
                    ("return", "minimal") => if rest.is_empty() { Ok(ReturnMinimal) } else { Err(None) },
                    ("handling", "strict") => if rest.is_empty() { Ok(HandlingStrict) } else { Err(None) },
                    ("handling", "lenient") => if rest.is_empty() { Ok(HandlingLenient) } else { Err(None) },
                    ("wait", secs) => if rest.is_empty() { secs.parse().map(Wait).map_err(Some) } else { Err(None) },
                    (left, right) => Ok(Extension(left.to_owned(), right.to_owned(), rest))
                }
            },
            None => Err(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use Header;
    use super::*;

    #[test]
    fn test_parse_multiple_headers() {
        let prefer = Header::parse_header(&"respond-async, return=representation".into());
        assert_eq!(prefer.ok(), Some(Prefer(vec![Preference::RespondAsync,
                                           Preference::ReturnRepresentation])))
    }

    #[test]
    fn test_parse_argument() {
        let prefer = Header::parse_header(&"wait=100, handling=lenient, respond-async".into());
        assert_eq!(prefer.ok(), Some(Prefer(vec![Preference::Wait(100),
                                           Preference::HandlingLenient,
                                           Preference::RespondAsync])))
    }

    #[test]
    fn test_parse_quote_form() {
        let prefer = Header::parse_header(&"wait=\"200\", handling=\"strict\"".into());
        assert_eq!(prefer.ok(), Some(Prefer(vec![Preference::Wait(200),
                                           Preference::HandlingStrict])))
    }

    #[test]
    fn test_parse_extension() {
        let prefer = Header::parse_header(&"foo, bar=baz, baz; foo; bar=baz, bux=\"\"; foo=\"\", buz=\"some parameter\"".into());
        assert_eq!(prefer.ok(), Some(Prefer(vec![
            Preference::Extension("foo".to_owned(), "".to_owned(), vec![]),
            Preference::Extension("bar".to_owned(), "baz".to_owned(), vec![]),
            Preference::Extension("baz".to_owned(), "".to_owned(), vec![("foo".to_owned(), "".to_owned()), ("bar".to_owned(), "baz".to_owned())]),
            Preference::Extension("bux".to_owned(), "".to_owned(), vec![("foo".to_owned(), "".to_owned())]),
            Preference::Extension("buz".to_owned(), "some parameter".to_owned(), vec![])])))
    }

    #[test]
    fn test_fail_with_args() {
        let prefer: ::Result<Prefer> = Header::parse_header(&"respond-async; foo=bar".into());
        assert_eq!(prefer.ok(), None);
    }
}

bench_header!(normal,
    Prefer, { vec![b"respond-async, return=representation".to_vec(), b"wait=100".to_vec()] });
