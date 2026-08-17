use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::NSProcessInfo;

impl UnwindSafe for NSProcessInfo {}
impl RefUnwindSafe for NSProcessInfo {}

impl fmt::Debug for NSProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("NSProcessInfo");

        #[cfg(feature = "NSString")]
        debug.field("processName", &self.processName());

        debug.finish_non_exhaustive()
    }
}
