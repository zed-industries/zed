use crate::sys::jfieldID;

/// Wrapper around [`jfieldID`] that implements `Send` + `Sync` since method IDs
/// are valid across threads (not tied to a `JNIEnv`).
///
/// There is no lifetime associated with these since they aren't garbage
/// collected like objects and their lifetime is not implicitly connected with
/// the scope in which they are queried.
///
/// It matches C's representation of the raw pointer, so it can be used in any
/// of the extern function argument positions that would take a [`jfieldID`].
///
/// # Safety
///
/// According to the JNI spec field IDs may be invalidated when the
/// corresponding class is unloaded.
///
/// Since this constraint can't be encoded as a Rust lifetime, and to avoid the
/// excessive cost of having every Method ID be associated with a global
/// reference to the corresponding class then it is the developers
/// responsibility to ensure they hold some class reference for the lifetime of
/// cached method IDs.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct JFieldID {
    internal: jfieldID,
}

// Field IDs are valid across threads (not tied to a JNIEnv)
unsafe impl Send for JFieldID {}
unsafe impl Sync for JFieldID {}

impl JFieldID {
    /// Creates a [`JFieldID`] that wraps the given `raw` [`jfieldID`]
    ///
    /// # Safety
    ///
    /// Expects a valid, non-`null` ID
    pub unsafe fn from_raw(raw: jfieldID) -> Self {
        debug_assert!(!raw.is_null(), "from_raw fieldID argument");
        Self { internal: raw }
    }

    /// Unwrap to the internal jni type.
    pub fn into_raw(self) -> jfieldID {
        self.internal
    }
}

impl AsRef<JFieldID> for JFieldID {
    fn as_ref(&self) -> &JFieldID {
        self
    }
}

impl AsMut<JFieldID> for JFieldID {
    fn as_mut(&mut self) -> &mut JFieldID {
        self
    }
}
