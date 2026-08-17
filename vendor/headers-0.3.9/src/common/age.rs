use std::time::Duration;

use util::Seconds;

/// `Age` header, defined in [RFC7234](https://tools.ietf.org/html/rfc7234#section-5.1)
///
/// The "Age" header field conveys the sender's estimate of the amount of
/// time since the response was generated or successfully validated at
/// the origin server.  Age values are calculated as specified in
/// [Section 4.2.3](https://tools.ietf.org/html/rfc7234#section-4.2.3).
///
/// ## ABNF
///
/// ```text
/// Age = delta-seconds
/// ```
///
/// The Age field-value is a non-negative integer, representing time in
/// seconds (see [Section 1.2.1](https://tools.ietf.org/html/rfc7234#section-1.2.1)).
///
/// The presence of an Age header field implies that the response was not
/// generated or validated by the origin server for this request.
/// However, lack of an Age header field does not imply the origin was
/// contacted, since the response might have been received from an
/// HTTP/1.0 cache that does not implement Age.
///
/// ## Example values
///
/// * `3600`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::Age;
///
/// let len = Age::from_secs(60);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Age(Seconds);

derive_header! {
  Age(_),
  name: AGE
}

impl Age {
    /// Creates a new `Age` header from the specified number of whole seconds.
    pub fn from_secs(secs: u64) -> Self {
        Self(Seconds::from_secs(secs))
    }

    /// Returns the number of seconds for this `Age` header.
    pub fn as_secs(&self) -> u64 {
        self.0.as_u64()
    }
}

impl From<Duration> for Age {
    fn from(dur: Duration) -> Self {
        Age(Seconds::from(dur))
    }
}

impl From<Age> for Duration {
    fn from(age: Age) -> Self {
        age.0.into()
    }
}
