use crate::{time::ArchivedDuration, Archive, Deserialize, Fallible, Serialize};
use core::time::Duration;

impl Archive for Duration {
    type Archived = ArchivedDuration;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
        ArchivedDuration::emplace(self.as_secs(), self.subsec_nanos(), out);
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Duration {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<Duration, D> for ArchivedDuration {
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<Duration, D::Error> {
        Ok(Duration::new(self.as_secs(), self.subsec_nanos()))
    }
}

impl From<ArchivedDuration> for Duration {
    #[inline]
    fn from(duration: ArchivedDuration) -> Self {
        Self::new(duration.as_secs(), duration.subsec_nanos())
    }
}
