use crate::time::ArchivedDuration;
use std::time::Duration;

impl PartialEq<Duration> for ArchivedDuration {
    #[inline]
    fn eq(&self, other: &Duration) -> bool {
        self.as_nanos() == other.as_nanos() && self.as_secs() == other.as_secs()
    }
}

impl PartialEq<ArchivedDuration> for Duration {
    #[inline]
    fn eq(&self, other: &ArchivedDuration) -> bool {
        other.eq(self)
    }
}
