//! This module abstracts over `loom` and `std::sync` types depending on whether we
//! are running loom tests or not.

pub(crate) mod sync {
    #[cfg(all(test, loom))]
    pub(crate) use loom::sync::{Arc, Mutex, MutexGuard};
    #[cfg(not(all(test, loom)))]
    pub(crate) use std::sync::{Arc, Mutex, MutexGuard};
}
