use crate::{objects::JObject, sys::jobject};

/// Lifetime'd representation of a `jobject` that is an instance of the
/// ByteBuffer Java class. Just a `JObject` wrapped in a new class.
#[repr(transparent)]
#[derive(Debug)]
pub struct JByteBuffer<'local>(JObject<'local>);

impl<'local> AsRef<JByteBuffer<'local>> for JByteBuffer<'local> {
    fn as_ref(&self) -> &JByteBuffer<'local> {
        self
    }
}

impl<'local> AsRef<JObject<'local>> for JByteBuffer<'local> {
    fn as_ref(&self) -> &JObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for JByteBuffer<'local> {
    type Target = JObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<JByteBuffer<'local>> for JObject<'local> {
    fn from(other: JByteBuffer) -> JObject {
        other.0
    }
}

impl<'local> From<JObject<'local>> for JByteBuffer<'local> {
    fn from(other: JObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

impl<'local, 'obj_ref> From<&'obj_ref JObject<'local>> for &'obj_ref JByteBuffer<'local> {
    fn from(other: &'obj_ref JObject<'local>) -> Self {
        // Safety: `JByteBuffer` is `repr(transparent)` around `JObject`.
        unsafe { &*(other as *const JObject<'local> as *const JByteBuffer<'local>) }
    }
}

impl<'local> std::default::Default for JByteBuffer<'local> {
    fn default() -> Self {
        Self(JObject::null())
    }
}

impl<'local> JByteBuffer<'local> {
    /// Creates a [`JByteBuffer`] that wraps the given `raw` [`jobject`]
    ///
    /// # Safety
    /// No runtime check is made to verify that the given [`jobject`] is an instance of
    /// a `ByteBuffer`.
    pub unsafe fn from_raw(raw: jobject) -> Self {
        Self(JObject::from_raw(raw as jobject))
    }

    /// Unwrap to the raw jni type.
    pub fn into_raw(self) -> jobject {
        self.0.into_raw() as jobject
    }
}
