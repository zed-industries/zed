use crate::sys::JNI_ABORT;

#[cfg(doc)]
use super::{AutoElements, AutoElementsCritical};

/// ReleaseMode
///
/// This defines the release mode of [`AutoElements`] (and [`AutoElementsCritical`]) resources, and
/// related release array functions.
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ReleaseMode {
    /// Copy back the content and free the elems buffer. For read-only access, prefer
    /// [`NoCopyBack`](ReleaseMode::NoCopyBack).
    CopyBack = 0,
    /// Free the buffer without copying back the possible changes.
    NoCopyBack = JNI_ABORT,
}
