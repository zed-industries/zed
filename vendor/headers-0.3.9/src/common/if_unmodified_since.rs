use std::time::SystemTime;
use util::HttpDate;

/// `If-Unmodified-Since` header, defined in
/// [RFC7232](http://tools.ietf.org/html/rfc7232#section-3.4)
///
/// The `If-Unmodified-Since` header field makes the request method
/// conditional on the selected representation's last modification date
/// being earlier than or equal to the date provided in the field-value.
/// This field accomplishes the same purpose as If-Match for cases where
/// the user agent does not have an entity-tag for the representation.
///
/// # ABNF
///
/// ```text
/// If-Unmodified-Since = HTTP-date
/// ```
///
/// # Example values
///
/// * `Sat, 29 Oct 1994 19:43:31 GMT`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::IfUnmodifiedSince;
/// use std::time::{SystemTime, Duration};
///
/// let time = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
/// let if_unmod = IfUnmodifiedSince::from(time);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IfUnmodifiedSince(HttpDate);

derive_header! {
    IfUnmodifiedSince(_),
    name: IF_UNMODIFIED_SINCE
}

impl IfUnmodifiedSince {
    /// Check if the supplied time passes the precondtion.
    pub fn precondition_passes(&self, last_modified: SystemTime) -> bool {
        self.0 >= last_modified.into()
    }
}

impl From<SystemTime> for IfUnmodifiedSince {
    fn from(time: SystemTime) -> IfUnmodifiedSince {
        IfUnmodifiedSince(time.into())
    }
}

impl From<IfUnmodifiedSince> for SystemTime {
    fn from(date: IfUnmodifiedSince) -> SystemTime {
        date.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn precondition_passes() {
        let newer = SystemTime::now();
        let exact = newer - Duration::from_secs(2);
        let older = newer - Duration::from_secs(4);

        let if_unmod = IfUnmodifiedSince::from(exact);
        assert!(!if_unmod.precondition_passes(newer));
        assert!(if_unmod.precondition_passes(exact));
        assert!(if_unmod.precondition_passes(older));
    }
}
