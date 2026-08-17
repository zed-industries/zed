use std::fmt;
use std::str::FromStr;
use std::time::SystemTime;

use bytes::Bytes;
use http::header::HeaderValue;
use httpdate;

use super::IterExt;

/// A timestamp with HTTP formatting and parsing
//   Prior to 1995, there were three different formats commonly used by
//   servers to communicate timestamps.  For compatibility with old
//   implementations, all three are defined here.  The preferred format is
//   a fixed-length and single-zone subset of the date and time
//   specification used by the Internet Message Format [RFC5322].
//
//     HTTP-date    = IMF-fixdate / obs-date
//
//   An example of the preferred format is
//
//     Sun, 06 Nov 1994 08:49:37 GMT    ; IMF-fixdate
//
//   Examples of the two obsolete formats are
//
//     Sunday, 06-Nov-94 08:49:37 GMT   ; obsolete RFC 850 format
//     Sun Nov  6 08:49:37 1994         ; ANSI C's asctime() format
//
//   A recipient that parses a timestamp value in an HTTP header field
//   MUST accept all three HTTP-date formats.  When a sender generates a
//   header field that contains one or more timestamps defined as
//   HTTP-date, the sender MUST generate those timestamps in the
//   IMF-fixdate format.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HttpDate(httpdate::HttpDate);

impl HttpDate {
    pub(crate) fn from_val(val: &HeaderValue) -> Option<Self> {
        val.to_str().ok()?.parse().ok()
    }
}

// TODO: remove this and FromStr?
#[derive(Debug)]
pub struct Error(());

impl super::TryFromValues for HttpDate {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(HttpDate::from_val)
            .ok_or_else(::Error::invalid)
    }
}

impl From<HttpDate> for HeaderValue {
    fn from(date: HttpDate) -> HeaderValue {
        (&date).into()
    }
}

impl<'a> From<&'a HttpDate> for HeaderValue {
    fn from(date: &'a HttpDate) -> HeaderValue {
        // TODO: could be just BytesMut instead of String
        let s = date.to_string();
        let bytes = Bytes::from(s);
        HeaderValue::from_maybe_shared(bytes).expect("HttpDate always is a valid value")
    }
}

impl FromStr for HttpDate {
    type Err = Error;
    fn from_str(s: &str) -> Result<HttpDate, Error> {
        Ok(HttpDate(s.parse().map_err(|_| Error(()))?))
    }
}

impl fmt::Debug for HttpDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Display for HttpDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<SystemTime> for HttpDate {
    fn from(sys: SystemTime) -> HttpDate {
        HttpDate(sys.into())
    }
}

impl From<HttpDate> for SystemTime {
    fn from(date: HttpDate) -> SystemTime {
        SystemTime::from(date.0)
    }
}

#[cfg(test)]
mod tests {
    use super::HttpDate;

    use std::time::{Duration, UNIX_EPOCH};

    // The old tests had Sunday, but 1994-11-07 is a Monday.
    // See https://github.com/pyfisch/httpdate/pull/6#issuecomment-846881001
    fn nov_07() -> HttpDate {
        HttpDate((UNIX_EPOCH + Duration::new(784198117, 0)).into())
    }

    #[test]
    fn test_display_is_imf_fixdate() {
        assert_eq!("Mon, 07 Nov 1994 08:48:37 GMT", &nov_07().to_string());
    }

    #[test]
    fn test_imf_fixdate() {
        assert_eq!(
            "Mon, 07 Nov 1994 08:48:37 GMT".parse::<HttpDate>().unwrap(),
            nov_07()
        );
    }

    #[test]
    fn test_rfc_850() {
        assert_eq!(
            "Monday, 07-Nov-94 08:48:37 GMT"
                .parse::<HttpDate>()
                .unwrap(),
            nov_07()
        );
    }

    #[test]
    fn test_asctime() {
        assert_eq!(
            "Mon Nov  7 08:48:37 1994".parse::<HttpDate>().unwrap(),
            nov_07()
        );
    }

    #[test]
    fn test_no_date() {
        assert!("this-is-no-date".parse::<HttpDate>().is_err());
    }
}
