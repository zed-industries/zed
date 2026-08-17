use std::fmt;
use std::time::Duration;

use util::IterExt;
use HeaderValue;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Seconds(Duration);

impl Seconds {
    pub(crate) fn from_val(val: &HeaderValue) -> Option<Self> {
        let secs = val.to_str().ok()?.parse().ok()?;

        Some(Self::from_secs(secs))
    }

    pub(crate) fn from_secs(secs: u64) -> Self {
        Self::from(Duration::from_secs(secs))
    }

    pub(crate) fn as_u64(&self) -> u64 {
        self.0.as_secs()
    }
}

impl super::TryFromValues for Seconds {
    fn try_from_values<'i, I>(values: &mut I) -> Result<Self, ::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .just_one()
            .and_then(Seconds::from_val)
            .ok_or_else(::Error::invalid)
    }
}

impl<'a> From<&'a Seconds> for HeaderValue {
    fn from(secs: &'a Seconds) -> HeaderValue {
        secs.0.as_secs().into()
    }
}

impl From<Duration> for Seconds {
    fn from(dur: Duration) -> Seconds {
        debug_assert!(dur.subsec_nanos() == 0);
        Seconds(dur)
    }
}

impl From<Seconds> for Duration {
    fn from(secs: Seconds) -> Duration {
        secs.0
    }
}

impl fmt::Debug for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s", self.0.as_secs())
    }
}

impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0.as_secs(), f)
    }
}
