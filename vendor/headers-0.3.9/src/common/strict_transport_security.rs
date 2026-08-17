use std::fmt;
use std::time::Duration;

use util::{self, IterExt, Seconds};

/// `StrictTransportSecurity` header, defined in [RFC6797](https://tools.ietf.org/html/rfc6797)
///
/// This specification defines a mechanism enabling web sites to declare
/// themselves accessible only via secure connections and/or for users to be
/// able to direct their user agent(s) to interact with given sites only over
/// secure connections.  This overall policy is referred to as HTTP Strict
/// Transport Security (HSTS).  The policy is declared by web sites via the
/// Strict-Transport-Security HTTP response header field and/or by other means,
/// such as user agent configuration, for example.
///
/// # ABNF
///
/// ```text
///      [ directive ]  *( ";" [ directive ] )
///
///      directive                 = directive-name [ "=" directive-value ]
///      directive-name            = token
///      directive-value           = token | quoted-string
///
/// ```
///
/// # Example values
///
/// * `max-age=31536000`
/// * `max-age=15768000 ; includeSubdomains`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use std::time::Duration;
/// use headers::StrictTransportSecurity;
///
/// let sts = StrictTransportSecurity::including_subdomains(Duration::from_secs(31_536_000));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct StrictTransportSecurity {
    /// Signals the UA that the HSTS Policy applies to this HSTS Host as well as
    /// any subdomains of the host's domain name.
    include_subdomains: bool,

    /// Specifies the number of seconds, after the reception of the STS header
    /// field, during which the UA regards the host (from whom the message was
    /// received) as a Known HSTS Host.
    max_age: Seconds,
}

impl StrictTransportSecurity {
    // NOTE: The two constructors exist to make a user *have* to decide if
    // subdomains can be included or not, instead of forgetting due to an
    // incorrect assumption about a default.

    /// Create an STS header that includes subdomains
    pub fn including_subdomains(max_age: Duration) -> StrictTransportSecurity {
        StrictTransportSecurity {
            max_age: max_age.into(),
            include_subdomains: true,
        }
    }

    /// Create an STS header that excludes subdomains
    pub fn excluding_subdomains(max_age: Duration) -> StrictTransportSecurity {
        StrictTransportSecurity {
            max_age: max_age.into(),
            include_subdomains: false,
        }
    }

    // getters

    /// Get whether this should include subdomains.
    pub fn include_subdomains(&self) -> bool {
        self.include_subdomains
    }

    /// Get the max-age.
    pub fn max_age(&self) -> Duration {
        self.max_age.into()
    }
}

enum Directive {
    MaxAge(u64),
    IncludeSubdomains,
    Unknown,
}

fn from_str(s: &str) -> Result<StrictTransportSecurity, ::Error> {
    s.split(';')
        .map(str::trim)
        .map(|sub| {
            if sub.eq_ignore_ascii_case("includeSubdomains") {
                Some(Directive::IncludeSubdomains)
            } else {
                let mut sub = sub.splitn(2, '=');
                match (sub.next(), sub.next()) {
                    (Some(left), Some(right)) if left.trim().eq_ignore_ascii_case("max-age") => {
                        right
                            .trim()
                            .trim_matches('"')
                            .parse()
                            .ok()
                            .map(Directive::MaxAge)
                    }
                    _ => Some(Directive::Unknown),
                }
            }
        })
        .fold(Some((None, None)), |res, dir| match (res, dir) {
            (Some((None, sub)), Some(Directive::MaxAge(age))) => Some((Some(age), sub)),
            (Some((age, None)), Some(Directive::IncludeSubdomains)) => Some((age, Some(()))),
            (Some((Some(_), _)), Some(Directive::MaxAge(_)))
            | (Some((_, Some(_))), Some(Directive::IncludeSubdomains))
            | (_, None) => None,
            (res, _) => res,
        })
        .and_then(|res| match res {
            (Some(age), sub) => Some(StrictTransportSecurity {
                max_age: Duration::from_secs(age).into(),
                include_subdomains: sub.is_some(),
            }),
            _ => None,
        })
        .ok_or_else(::Error::invalid)
}

impl ::Header for StrictTransportSecurity {
    fn name() -> &'static ::HeaderName {
        &::http::header::STRICT_TRANSPORT_SECURITY
    }

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .just_one()
            .and_then(|v| v.to_str().ok())
            .map(from_str)
            .unwrap_or_else(|| Err(::Error::invalid()))
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        struct Adapter<'a>(&'a StrictTransportSecurity);

        impl<'a> fmt::Display for Adapter<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.0.include_subdomains {
                    write!(f, "max-age={}; includeSubdomains", self.0.max_age)
                } else {
                    write!(f, "max-age={}", self.0.max_age)
                }
            }
        }

        values.extend(::std::iter::once(util::fmt(Adapter(self))));
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::StrictTransportSecurity;
    use std::time::Duration;

    #[test]
    fn test_parse_max_age() {
        let h = test_decode::<StrictTransportSecurity>(&["max-age=31536000"]).unwrap();
        assert_eq!(
            h,
            StrictTransportSecurity {
                include_subdomains: false,
                max_age: Duration::from_secs(31536000).into(),
            }
        );
    }

    #[test]
    fn test_parse_max_age_no_value() {
        assert_eq!(test_decode::<StrictTransportSecurity>(&["max-age"]), None,);
    }

    #[test]
    fn test_parse_quoted_max_age() {
        let h = test_decode::<StrictTransportSecurity>(&["max-age=\"31536000\""]).unwrap();
        assert_eq!(
            h,
            StrictTransportSecurity {
                include_subdomains: false,
                max_age: Duration::from_secs(31536000).into(),
            }
        );
    }

    #[test]
    fn test_parse_spaces_max_age() {
        let h = test_decode::<StrictTransportSecurity>(&["max-age = 31536000"]).unwrap();
        assert_eq!(
            h,
            StrictTransportSecurity {
                include_subdomains: false,
                max_age: Duration::from_secs(31536000).into(),
            }
        );
    }

    #[test]
    fn test_parse_include_subdomains() {
        let h = test_decode::<StrictTransportSecurity>(&["max-age=15768000 ; includeSubDomains"])
            .unwrap();
        assert_eq!(
            h,
            StrictTransportSecurity {
                include_subdomains: true,
                max_age: Duration::from_secs(15768000).into(),
            }
        );
    }

    #[test]
    fn test_parse_no_max_age() {
        assert_eq!(
            test_decode::<StrictTransportSecurity>(&["includeSubdomains"]),
            None,
        );
    }

    #[test]
    fn test_parse_max_age_nan() {
        assert_eq!(
            test_decode::<StrictTransportSecurity>(&["max-age = izzy"]),
            None,
        );
    }

    #[test]
    fn test_parse_duplicate_directives() {
        assert_eq!(
            test_decode::<StrictTransportSecurity>(&["max-age=1; max-age=2"]),
            None,
        );
    }
}

//bench_header!(bench, StrictTransportSecurity, { vec![b"max-age=15768000 ; includeSubDomains".to_vec()] });
