use std::time::{Duration, SystemTime};

use util::{HttpDate, Seconds, TryFromValues};
use HeaderValue;

/// The `Retry-After` header.
///
/// The `Retry-After` response-header field can be used with a 503 (Service
/// Unavailable) response to indicate how long the service is expected to be
/// unavailable to the requesting client. This field MAY also be used with any
/// 3xx (Redirection) response to indicate the minimum time the user-agent is
/// asked wait before issuing the redirected request. The value of this field
/// can be either an HTTP-date or an integer number of seconds (in decimal)
/// after the time of the response.
///
/// # Examples
/// ```
/// # extern crate headers;
/// use std::time::{Duration, SystemTime};
/// use headers::RetryAfter;
///
/// let delay = RetryAfter::delay(Duration::from_secs(300));
/// let date = RetryAfter::date(SystemTime::now());
/// ```

/// Retry-After header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.3)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryAfter(After);

derive_header! {
    RetryAfter(_),
    name: RETRY_AFTER
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum After {
    /// Retry after the given DateTime
    DateTime(HttpDate),
    /// Retry after this duration has elapsed
    Delay(Seconds),
}

impl RetryAfter {
    /// Create an `RetryAfter` header with a date value.
    pub fn date(time: SystemTime) -> RetryAfter {
        RetryAfter(After::DateTime(time.into()))
    }

    /// Create an `RetryAfter` header with a date value.
    pub fn delay(dur: Duration) -> RetryAfter {
        RetryAfter(After::Delay(dur.into()))
    }
}

impl TryFromValues for After {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .and_then(|val| {
                if let Some(delay) = Seconds::from_val(val) {
                    return Some(After::Delay(delay));
                }

                let date = HttpDate::from_val(val)?;
                Some(After::DateTime(date))
            })
            .ok_or_else(::Error::invalid)
    }
}

impl<'a> From<&'a After> for HeaderValue {
    fn from(after: &'a After) -> HeaderValue {
        match *after {
            After::Delay(ref delay) => delay.into(),
            After::DateTime(ref date) => date.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_decode;
    use super::RetryAfter;
    use std::time::Duration;
    use util::HttpDate;

    #[test]
    fn delay_decode() {
        let r: RetryAfter = test_decode(&["1234"]).unwrap();
        assert_eq!(r, RetryAfter::delay(Duration::from_secs(1234)),);
    }

    macro_rules! test_retry_after_datetime {
        ($name:ident, $s:expr) => {
            #[test]
            fn $name() {
                let r: RetryAfter = test_decode(&[$s]).unwrap();
                let dt = "Sun, 06 Nov 1994 08:49:37 GMT".parse::<HttpDate>().unwrap();

                assert_eq!(r, RetryAfter(super::After::DateTime(dt)));
            }
        };
    }

    test_retry_after_datetime!(date_decode_rfc1123, "Sun, 06 Nov 1994 08:49:37 GMT");
    test_retry_after_datetime!(date_decode_rfc850, "Sunday, 06-Nov-94 08:49:37 GMT");
    test_retry_after_datetime!(date_decode_asctime, "Sun Nov  6 08:49:37 1994");
}
