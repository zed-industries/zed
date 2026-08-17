#[cfg(feature = "NSString")]
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::Foundation::NSProcessInfo;

// SAFETY: The documentation explicitly states:
// > NSProcessInfo is thread-safe in macOS 10.7 and later.
unsafe impl Send for NSProcessInfo {}
unsafe impl Sync for NSProcessInfo {}

impl UnwindSafe for NSProcessInfo {}
impl RefUnwindSafe for NSProcessInfo {}

#[cfg(feature = "NSString")]
impl fmt::Debug for NSProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSProcessInfo")
            .field("processName", &self.processName())
            .finish_non_exhaustive()
    }
}
