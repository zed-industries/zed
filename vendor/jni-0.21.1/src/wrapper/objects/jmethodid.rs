use crate::sys::jmethodID;

/// Wrapper around [`jmethodID`] that implements `Send` + `Sync` since method IDs
/// are valid across threads (not tied to a `JNIEnv`).
///
/// There is no lifetime associated with these since they aren't garbage
/// collected like objects and their lifetime is not implicitly connected with
/// the scope in which they are queried.
///
/// It matches C's representation of the raw pointer, so it can be used in any
/// of the extern function argument positions that would take a [`jmethodID`].
///
/// # Safety
///
/// According to the JNI spec method IDs may be invalidated when the
/// corresponding class is unloaded.
///
/// Since this constraint can't be encoded as a Rust lifetime, and to avoid the
/// excessive cost of having every Method ID be associated with a global
/// reference to the corresponding class then it is the developers
/// responsibility to ensure they hold some class reference for the lifetime of
/// cached method IDs.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct JMethodID {
    internal: jmethodID,
}

// Method IDs are valid across threads (not tied to a JNIEnv)
unsafe impl Send for JMethodID {}
unsafe impl Sync for JMethodID {}

impl JMethodID {
    /// Creates a [`JMethodID`] that wraps the given `raw` [`jmethodID`]
    ///
    /// # Safety
    ///
    /// Expects a valid, non-`null` ID
    pub unsafe fn from_raw(raw: jmethodID) -> Self {
        debug_assert!(!raw.is_null(), "from_raw methodID argument");
        Self { internal: raw }
    }

    /// Unwrap to the internal jni type.
    pub fn into_raw(self) -> jmethodID {
        self.internal
    }
}

impl AsRef<JMethodID> for JMethodID {
    fn as_ref(&self) -> &JMethodID {
        self
    }
}

impl AsMut<JMethodID> for JMethodID {
    fn as_mut(&mut self) -> &mut JMethodID {
        self
    }
}
