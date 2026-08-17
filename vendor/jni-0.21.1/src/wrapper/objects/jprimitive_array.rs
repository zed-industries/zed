use std::marker::PhantomData;

use crate::{
    objects::JObject,
    sys::{jarray, jobject},
};

use super::TypeArray;

#[cfg(doc)]
use crate::JNIEnv;

/// Lifetime'd representation of a [`jarray`] which wraps a [`JObject`] reference
///
/// This is a wrapper type for a [`JObject`] local reference that's used to
/// differentiate JVM array types.
#[repr(transparent)]
#[derive(Debug)]
pub struct JPrimitiveArray<'local, T: TypeArray> {
    obj: JObject<'local>,
    lifetime: PhantomData<&'local T>,
}

impl<'local, T: TypeArray> AsRef<JPrimitiveArray<'local, T>> for JPrimitiveArray<'local, T> {
    fn as_ref(&self) -> &JPrimitiveArray<'local, T> {
        self
    }
}

impl<'local, T: TypeArray> AsMut<JPrimitiveArray<'local, T>> for JPrimitiveArray<'local, T> {
    fn as_mut(&mut self) -> &mut JPrimitiveArray<'local, T> {
        self
    }
}

impl<'local, T: TypeArray> AsRef<JObject<'local>> for JPrimitiveArray<'local, T> {
    fn as_ref(&self) -> &JObject<'local> {
        &self.obj
    }
}

impl<'local, T: TypeArray> ::std::ops::Deref for JPrimitiveArray<'local, T> {
    type Target = JObject<'local>;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl<'local, T: TypeArray> From<JPrimitiveArray<'local, T>> for JObject<'local> {
    fn from(other: JPrimitiveArray<'local, T>) -> JObject {
        other.obj
    }
}

/// This conversion assumes that the `JObject` is a pointer to a class object.
impl<'local, T: TypeArray> From<JObject<'local>> for JPrimitiveArray<'local, T> {
    fn from(other: JObject) -> Self {
        unsafe { Self::from_raw(other.into_raw()) }
    }
}

/// This conversion assumes that the `JObject` is a pointer to a class object.
impl<'local, 'obj_ref, T: TypeArray> From<&'obj_ref JObject<'local>>
    for &'obj_ref JPrimitiveArray<'local, T>
{
    fn from(other: &'obj_ref JObject<'local>) -> Self {
        // Safety: `JPrimitiveArray` is `repr(transparent)` around `JObject`.
        unsafe { &*(other as *const JObject<'local> as *const JPrimitiveArray<'local, T>) }
    }
}

impl<'local, T: TypeArray> std::default::Default for JPrimitiveArray<'local, T> {
    fn default() -> Self {
        Self {
            obj: JObject::null(),
            lifetime: PhantomData,
        }
    }
}

impl<'local, T: TypeArray> JPrimitiveArray<'local, T> {
    /// Creates a [`JPrimitiveArray`] that wraps the given `raw` [`jarray`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw JNI local reference.
    /// * There must not be any other `JObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub unsafe fn from_raw(raw: jarray) -> Self {
        Self {
            obj: JObject::from_raw(raw as jobject),
            lifetime: PhantomData,
        }
    }

    /// Unwrap to the raw jni type.
    pub fn into_raw(self) -> jarray {
        self.obj.into_raw() as jarray
    }
}

/// Lifetime'd representation of a [`crate::sys::jbooleanArray`] which wraps a [`JObject`] reference
pub type JBooleanArray<'local> = JPrimitiveArray<'local, crate::sys::jboolean>;

/// Lifetime'd representation of a [`crate::sys::jbyteArray`] which wraps a [`JObject`] reference
pub type JByteArray<'local> = JPrimitiveArray<'local, crate::sys::jbyte>;

/// Lifetime'd representation of a [`crate::sys::jcharArray`] which wraps a [`JObject`] reference
pub type JCharArray<'local> = JPrimitiveArray<'local, crate::sys::jchar>;

/// Lifetime'd representation of a [`crate::sys::jshortArray`] which wraps a [`JObject`] reference
pub type JShortArray<'local> = JPrimitiveArray<'local, crate::sys::jshort>;

/// Lifetime'd representation of a [`crate::sys::jintArray`] which wraps a [`JObject`] reference
pub type JIntArray<'local> = JPrimitiveArray<'local, crate::sys::jint>;

/// Lifetime'd representation of a [`crate::sys::jlongArray`] which wraps a [`JObject`] reference
pub type JLongArray<'local> = JPrimitiveArray<'local, crate::sys::jlong>;

/// Lifetime'd representation of a [`crate::sys::jfloatArray`] which wraps a [`JObject`] reference
pub type JFloatArray<'local> = JPrimitiveArray<'local, crate::sys::jfloat>;

/// Lifetime'd representation of a [`crate::sys::jdoubleArray`] which wraps a [`JObject`] reference
pub type JDoubleArray<'local> = JPrimitiveArray<'local, crate::sys::jdouble>;

/// Trait to access the raw `jarray` pointer for types that wrap an array reference
///
/// # Safety
///
/// Implementing this trait will allow a type to be passed to [`JNIEnv::get_array_length()`]
/// or other JNI APIs that only work with a valid reference to an array (or `null`)
///
pub unsafe trait AsJArrayRaw<'local>: AsRef<JObject<'local>> {
    /// Returns the raw JNI pointer as a `jarray`
    fn as_jarray_raw(&self) -> jarray {
        self.as_ref().as_raw() as jarray
    }
}

unsafe impl<'local, T: TypeArray> AsJArrayRaw<'local> for JPrimitiveArray<'local, T> {}
