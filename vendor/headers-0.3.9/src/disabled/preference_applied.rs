use std::fmt;
use {Header, Raw, Preference};
use parsing::{from_comma_delimited, fmt_comma_delimited};

/// `Preference-Applied` header, defined in [RFC7240](http://tools.ietf.org/html/rfc7240)
///
/// The `Preference-Applied` response header may be included within a
/// response message as an indication as to which `Prefer` header tokens were
/// honored by the server and applied to the processing of a request.
///
/// # ABNF
///
/// ```text
/// Preference-Applied = "Preference-Applied" ":" 1#applied-pref
/// applied-pref = token [ BWS "=" BWS word ]
/// ```
///
/// # Example values
///
/// * `respond-async`
/// * `return=minimal`
/// * `wait=30`
///
/// # Examples
///
/// ```
/// use headers::{Headers, PreferenceApplied, Preference};
///
/// let mut headers = Headers::new();
/// headers.set(
///     PreferenceApplied(vec![Preference::RespondAsync])
/// );
/// ```
///
/// ```
/// use headers::{Headers, PreferenceApplied, Preference};
///
/// let mut headers = Headers::new();
/// headers.set(
///     PreferenceApplied(vec![
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
pub struct PreferenceApplied(pub Vec<Preference>);

__hyper__deref!(PreferenceApplied => Vec<Preference>);

impl Header for PreferenceApplied {
    fn header_name() -> &'static str {
        static NAME: &'static str = "Preference-Applied";
        NAME
    }

    fn parse_header(raw: &Raw) -> ::Result<PreferenceApplied> {
        let preferences = try!(from_comma_delimited(raw));
        if !preferences.is_empty() {
            Ok(PreferenceApplied(preferences))
        } else {
            Err(::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut ::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl fmt::Display for PreferenceApplied {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: format this without allocating a Vec and cloning contents
        let preferences: Vec<_> = self.0.iter().map(|pref| match pref {
            // The spec ignores parameters in `Preferences-Applied`
            &Preference::Extension(ref name, ref value, _) => Preference::Extension(
              name.to_owned(),
              value.to_owned(),
              vec![]
            ),
            preference => preference.clone()
        }).collect();
        fmt_comma_delimited(f, &preferences)
    }
}

#[cfg(test)]
mod tests {
    use Preference;
    use super::*;

    #[test]
    fn test_format_ignore_parameters() {
        assert_eq!(
            format!("{}", PreferenceApplied(vec![Preference::Extension(
                "foo".to_owned(),
                "bar".to_owned(),
                vec![("bar".to_owned(), "foo".to_owned()), ("buz".to_owned(), "".to_owned())]
            )])),
            "foo=bar".to_owned()
        );
    }
}

bench_header!(normal,
    PreferenceApplied, { vec![b"respond-async, return=representation".to_vec(), b"wait=100".to_vec()] });
