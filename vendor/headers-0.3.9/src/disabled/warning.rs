use std::fmt;
use std::str::{FromStr};
use {Header, HttpDate, Raw};
use parsing::from_one_raw_str;

/// `Warning` header, defined in [RFC7234](https://tools.ietf.org/html/rfc7234#section-5.5)
///
/// The `Warning` header field can be be used to carry additional information
/// about the status or transformation of a message that might not be reflected
/// in the status code. This header is sometimes used as backwards
/// compatible way to notify of a deprecated API.
///
/// # ABNF
///
/// ```text
/// Warning       = 1#warning-value
/// warning-value = warn-code SP warn-agent SP warn-text
///                                       [ SP warn-date ]
/// warn-code  = 3DIGIT
/// warn-agent = ( uri-host [ ":" port ] ) / pseudonym
///                 ; the name or pseudonym of the server adding
///                 ; the Warning header field, for use in debugging
///                 ; a single "-" is recommended when agent unknown
/// warn-text  = quoted-string
/// warn-date  = DQUOTE HTTP-date DQUOTE
/// ```
///
/// # Example values
///
/// * `Warning: 112 - "network down" "Sat, 25 Aug 2012 23:34:45 GMT"`
/// * `Warning: 299 - "Deprecated API " "Tue, 15 Nov 1994 08:12:31 GMT"`
/// * `Warning: 299 api.hyper.rs:8080 "Deprecated API : use newapi.hyper.rs instead."`
/// * `Warning: 299 api.hyper.rs:8080 "Deprecated API : use newapi.hyper.rs instead." "Tue, 15 Nov 1994 08:12:31 GMT"`
///
/// # Examples
///
/// ```
/// use headers::{Headers, Warning};
///
/// let mut headers = Headers::new();
/// headers.set(
///     Warning{
///         code: 299,
///         agent: "api.hyper.rs".to_owned(),
///         text: "Deprecated".to_owned(),
///         date: None
///     }
/// );
/// ```
///
/// ```
/// use headers::{Headers, HttpDate, Warning};
///
/// let mut headers = Headers::new();
/// headers.set(
///     Warning{
///         code: 299,
///         agent: "api.hyper.rs".to_owned(),
///         text: "Deprecated".to_owned(),
///         date: "Tue, 15 Nov 1994 08:12:31 GMT".parse::<HttpDate>().ok()
///     }
/// );
/// ```
///
/// ```
/// use std::time::SystemTime;
/// use headers::{Headers, Warning};
///
/// let mut headers = Headers::new();
/// headers.set(
///     Warning{
///         code: 199,
///         agent: "api.hyper.rs".to_owned(),
///         text: "Deprecated".to_owned(),
///         date: Some(SystemTime::now().into())
///     }
/// );
/// ```
#[derive(PartialEq, Clone, Debug)]
pub struct Warning {
    /// The 3 digit warn code.
    pub code: u16,
    /// The name or pseudonym of the server adding this header.
    pub agent: String,
    /// The warning message describing the error.
    pub text: String,
    /// An optional warning date.
    pub date: Option<HttpDate>
}

impl Header for Warning {
    fn header_name() -> &'static str {
        static NAME: &'static str = "Warning";
        NAME
    }

    fn parse_header(raw: &Raw) -> ::Result<Warning> {
        from_one_raw_str(raw)
    }

    fn fmt_header(&self, f: &mut ::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.date {
            Some(date) => write!(f, "{:03} {} \"{}\" \"{}\"", self.code, self.agent, self.text, date),
            None => write!(f, "{:03} {} \"{}\"", self.code, self.agent, self.text)
        }
    }
}

impl FromStr for Warning {
    type Err = ::Error;

    fn from_str(s: &str) -> ::Result<Warning> {
        let mut warning_split = s.split_whitespace();
        let code = match warning_split.next() {
            Some(c) => match c.parse::<u16>() {
                Ok(c) => c,
                Err(..) => return Err(::Error::Header)
            },
            None => return Err(::Error::Header)
        };
        let agent = match warning_split.next() {
            Some(a) => a.to_string(),
            None => return Err(::Error::Header)
        };

        let mut warning_split = s.split('"').skip(1);
        let text = match warning_split.next() {
            Some(t) => t.to_string(),
            None => return Err(::Error::Header)
        };
        let date = match warning_split.skip(1).next() {
            Some(d) => d.parse::<HttpDate>().ok(),
            None => None // Optional
        };

        Ok(Warning {
            code: code,
            agent: agent,
            text: text,
            date: date
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Warning;
    use {Header, HttpDate};

    #[test]
    fn test_parsing() {
        let warning = Header::parse_header(&vec![b"112 - \"network down\" \"Sat, 25 Aug 2012 23:34:45 GMT\"".to_vec()].into());
        assert_eq!(warning.ok(), Some(Warning {
            code: 112,
            agent: "-".to_owned(),
            text: "network down".to_owned(),
            date: "Sat, 25 Aug 2012 23:34:45 GMT".parse::<HttpDate>().ok()
        }));

        let warning = Header::parse_header(&vec![b"299 api.hyper.rs:8080 \"Deprecated API : use newapi.hyper.rs instead.\"".to_vec()].into());
        assert_eq!(warning.ok(), Some(Warning {
            code: 299,
            agent: "api.hyper.rs:8080".to_owned(),
            text: "Deprecated API : use newapi.hyper.rs instead.".to_owned(),
            date: None
        }));

        let warning = Header::parse_header(&vec![b"299 api.hyper.rs:8080 \"Deprecated API : use newapi.hyper.rs instead.\" \"Tue, 15 Nov 1994 08:12:31 GMT\"".to_vec()].into());
        assert_eq!(warning.ok(), Some(Warning {
            code: 299,
            agent: "api.hyper.rs:8080".to_owned(),
            text: "Deprecated API : use newapi.hyper.rs instead.".to_owned(),
            date: "Tue, 15 Nov 1994 08:12:31 GMT".parse::<HttpDate>().ok()
        }));
    }
}
