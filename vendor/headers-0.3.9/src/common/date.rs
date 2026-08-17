use std::time::SystemTime;
use util::HttpDate;

/// `Date` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.1.2)
///
/// The `Date` header field represents the date and time at which the
/// message was originated.
///
/// ## ABNF
///
/// ```text
/// Date = HTTP-date
/// ```
///
/// ## Example values
///
/// * `Tue, 15 Nov 1994 08:12:31 GMT`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::Date;
/// use std::time::SystemTime;
///
/// let date = Date::from(SystemTime::now());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(HttpDate);

derive_header! {
    Date(_),
    name: DATE
}

impl From<SystemTime> for Date {
    fn from(time: SystemTime) -> Date {
        Date(time.into())
    }
}

impl From<Date> for SystemTime {
    fn from(date: Date) -> SystemTime {
        date.0.into()
    }
}
