use crate::{
    objects::JObject,
    sys::{jobject, jstring},
};

/// Lifetime'd representation of a `jstring`. Just a `JObject` wrapped in a new
/// class.
#[repr(transparent)]
pub struct JString<'local>(JObject<'local>);

impl<'local> AsRef<JString<'local>> for JString<'local> {
    fn as_ref(&self) -> &JString<'local> {
        self
    }
}

impl<'local> AsRef<JObject<'local>> for JString<'local> {
    fn as_ref(&self) -> &JObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for JString<'local> {
    type Target = JObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<JString<'local>> for JObject<'local> {
    fn from(other: JString) -> JObject {
        other.0
    }
}

impl<'local> From<JObject<'local>> for JString<'local> {
    fn from(other: JObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

impl<'local, 'obj_ref> From<&'obj_ref JObject<'local>> for &'obj_ref JString<'local> {
    fn from(other: &'obj_ref JObject<'local>) -> Self {
        // Safety: `JString` is `repr(transparent)` around `JObject`.
        unsafe { &*(other as *const JObject<'local> as *const JString<'local>) }
    }
}

impl<'local> std::default::Default for JString<'local> {
    fn default() -> Self {
        Self(JObject::null())
    }
}

impl<'local> JString<'local> {
    /// Creates a [`JString`] that wraps the given `raw` [`jstring`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw JNI local reference.
    /// * There must not be any other `JObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub unsafe fn from_raw(raw: jstring) -> Self {
        Self(JObject::from_raw(raw as jobject))
    }

    /// Unwrap to the raw jni type.
    pub fn into_raw(self) -> jstring {
        self.0.into_raw() as jstring
    }
}
