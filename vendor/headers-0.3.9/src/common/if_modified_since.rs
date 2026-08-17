use std::time::SystemTime;
use util::HttpDate;

/// `If-Modified-Since` header, defined in
/// [RFC7232](http://tools.ietf.org/html/rfc7232#section-3.3)
///
/// The `If-Modified-Since` header field makes a GET or HEAD request
/// method conditional on the selected representation's modification date
/// being more recent than the date provided in the field-value.
/// Transfer of the selected representation's data is avoided if that
/// data has not changed.
///
/// # ABNF
///
/// ```text
/// If-Modified-Since = HTTP-date
/// ```
///
/// # Example values
/// * `Sat, 29 Oct 1994 19:43:31 GMT`
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::IfModifiedSince;
/// use std::time::{Duration, SystemTime};
///
/// let time = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
/// let if_mod = IfModifiedSince::from(time);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IfModifiedSince(HttpDate);

derive_header! {
    IfModifiedSince(_),
    name: IF_MODIFIED_SINCE
}

impl IfModifiedSince {
    /// Check if the supplied time means the resource has been modified.
    pub fn is_modified(&self, last_modified: SystemTime) -> bool {
        self.0 < last_modified.into()
    }
}

impl From<SystemTime> for IfModifiedSince {
    fn from(time: SystemTime) -> IfModifiedSince {
        IfModifiedSince(time.into())
    }
}

impl From<IfModifiedSince> for SystemTime {
    fn from(date: IfModifiedSince) -> SystemTime {
        date.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn is_modified() {
        let newer = SystemTime::now();
        let exact = newer - Duration::from_secs(2);
        let older = newer - Duration::from_secs(4);

        let if_mod = IfModifiedSince::from(exact);
        assert!(if_mod.is_modified(newer));
        assert!(!if_mod.is_modified(exact));
        assert!(!if_mod.is_modified(older));
    }
}
