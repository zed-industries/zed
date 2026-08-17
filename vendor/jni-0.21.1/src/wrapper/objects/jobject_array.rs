use crate::{
    objects::JObject,
    sys::{jobject, jobjectArray},
};

use super::AsJArrayRaw;

/// Lifetime'd representation of a [`jobjectArray`] which wraps a [`JObject`] reference
#[repr(transparent)]
#[derive(Debug)]
pub struct JObjectArray<'local>(JObject<'local>);

impl<'local> AsRef<JObjectArray<'local>> for JObjectArray<'local> {
    fn as_ref(&self) -> &JObjectArray<'local> {
        self
    }
}

impl<'local> AsRef<JObject<'local>> for JObjectArray<'local> {
    fn as_ref(&self) -> &JObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for JObjectArray<'local> {
    type Target = JObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'local> From<JObjectArray<'local>> for JObject<'local> {
    fn from(other: JObjectArray) -> JObject {
        other.0
    }
}

/// This conversion assumes that the `JObject` is a pointer to a class object.
impl<'local> From<JObject<'local>> for JObjectArray<'local> {
    fn from(other: JObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

/// This conversion assumes that the `JObject` is a pointer to a class object.
impl<'local, 'obj_ref> From<&'obj_ref JObject<'local>> for &'obj_ref JObjectArray<'local> {
    fn from(other: &'obj_ref JObject<'local>) -> Self {
        // Safety: `JObjectArray` is `repr(transparent)` around `JObject`.
        unsafe { &*(other as *const JObject<'local> as *const JObjectArray<'local>) }
    }
}

impl<'local> std::default::Default for JObjectArray<'local> {
    fn default() -> Self {
        Self(JObject::null())
    }
}

unsafe impl<'local> AsJArrayRaw<'local> for JObjectArray<'local> {}

impl<'local> JObjectArray<'local> {
    /// Creates a [`JObjectArray`] that wraps the given `raw` [`jobjectArray`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw JNI local reference.
    /// * There must not be any other `JObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub unsafe fn from_raw(raw: jobjectArray) -> Self {
        Self(JObject::from_raw(raw as jobject))
    }

    /// Unwrap to the raw jni type.
    pub fn into_raw(self) -> jobjectArray {
        self.0.into_raw() as jobjectArray
    }
}
