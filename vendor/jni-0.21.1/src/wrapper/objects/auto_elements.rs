use log::error;
use std::ptr::NonNull;

use crate::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use crate::wrapper::objects::ReleaseMode;
use crate::{errors::*, sys, JNIEnv};

use super::JPrimitiveArray;

#[cfg(doc)]
use super::JByteArray;

mod type_array_sealed {
    use crate::sys::{jarray, jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
    use crate::{errors::*, JNIEnv};
    use std::ptr::NonNull;

    /// Trait to define type array access/release
    ///
    /// # Safety
    ///
    /// The methods of this trait must uphold the invariants described in [`JNIEnv::unsafe_clone`] when
    /// using the provided [`JNIEnv`].
    ///
    /// The `get` method must return a valid pointer to the beginning of the JNI array.
    ///
    /// The `release` method must not invalidate the `ptr` if the `mode` is [`sys::JNI_COMMIT`].
    pub unsafe trait TypeArraySealed: Copy {
        /// getter
        ///
        /// # Safety
        ///
        /// `array` must be a valid pointer to an `Array` object, or `null`
        ///
        /// The caller is responsible for passing the returned pointer to [`release`], along
        /// with the same `env` and `array` reference (which needs to still be valid)
        unsafe fn get(env: &mut JNIEnv, array: jarray, is_copy: &mut jboolean)
            -> Result<*mut Self>;

        /// releaser
        ///
        /// # Safety
        ///
        /// `ptr` must have been previously returned by the `get` function.
        ///
        /// If `mode` is not [`sys::JNI_COMMIT`], `ptr` must not be used again after calling this
        /// function.
        unsafe fn release(
            env: &mut JNIEnv,
            array: jarray,
            ptr: NonNull<Self>,
            mode: i32,
        ) -> Result<()>;
    }

    // TypeArray builder
    macro_rules! type_array {
        ( $jni_type:ty, $jni_get:tt, $jni_release:tt ) => {
            /// $jni_type array access/release impl
            unsafe impl TypeArraySealed for $jni_type {
                /// Get Java $jni_type array
                unsafe fn get(
                    env: &mut JNIEnv,
                    array: jarray,
                    is_copy: &mut jboolean,
                ) -> Result<*mut Self> {
                    let internal = env.get_native_interface();
                    // Even though this method may throw OoME, use `jni_unchecked`
                    // instead of `jni_non_null_call` to remove (a slight) overhead
                    // of exception checking. An error will still be detected as a `null`
                    // result inside AutoElements ctor. Also, modern Hotspot in case of lack
                    // of memory will return null and won't throw an exception:
                    // https://sourcegraph.com/github.com/openjdk/jdk/-/blob/src/hotspot/share/memory/allocation.hpp#L488-489
                    let res = jni_unchecked!(internal, $jni_get, array, is_copy);
                    Ok(res)
                }

                /// Release Java $jni_type array
                unsafe fn release(
                    env: &mut JNIEnv,
                    array: jarray,
                    ptr: NonNull<Self>,
                    mode: i32,
                ) -> Result<()> {
                    let internal = env.get_native_interface();
                    jni_unchecked!(internal, $jni_release, array, ptr.as_ptr(), mode as i32);
                    Ok(())
                }
            }
        };
    }

    type_array!(jint, GetIntArrayElements, ReleaseIntArrayElements);
    type_array!(jlong, GetLongArrayElements, ReleaseLongArrayElements);
    type_array!(jbyte, GetByteArrayElements, ReleaseByteArrayElements);
    type_array!(
        jboolean,
        GetBooleanArrayElements,
        ReleaseBooleanArrayElements
    );
    type_array!(jchar, GetCharArrayElements, ReleaseCharArrayElements);
    type_array!(jshort, GetShortArrayElements, ReleaseShortArrayElements);
    type_array!(jfloat, GetFloatArrayElements, ReleaseFloatArrayElements);
    type_array!(jdouble, GetDoubleArrayElements, ReleaseDoubleArrayElements);
}

/// A sealed trait to define type array access/release for primitive JNI types
pub trait TypeArray: type_array_sealed::TypeArraySealed {}

impl TypeArray for jint {}
impl TypeArray for jlong {}
impl TypeArray for jbyte {}
impl TypeArray for jboolean {}
impl TypeArray for jchar {}
impl TypeArray for jshort {}
impl TypeArray for jfloat {}
impl TypeArray for jdouble {}

/// Auto-release wrapper for a mutable pointer to the elements of a [`JPrimitiveArray`]
/// (such as [`JByteArray`])
///
/// This type is used to wrap pointers returned by `Get<Type>ArrayElements`
/// and ensure the pointer is released via `Release<Type>ArrayElements` when dropped.
pub struct AutoElements<'local, 'other_local, 'array, T: TypeArray> {
    array: &'array JPrimitiveArray<'other_local, T>,
    len: usize,
    ptr: NonNull<T>,
    mode: ReleaseMode,
    is_copy: bool,
    env: JNIEnv<'local>,
}

impl<'local, 'other_local, 'array, T: TypeArray> AutoElements<'local, 'other_local, 'array, T> {
    /// # Safety
    ///
    /// `len` must be the correct length (number of elements) of the given `array`
    pub(crate) unsafe fn new_with_len(
        env: &mut JNIEnv<'local>,
        array: &'array JPrimitiveArray<'other_local, T>,
        len: usize,
        mode: ReleaseMode,
    ) -> Result<Self> {
        // Safety: The cloned `JNIEnv` will not be used to create any local references. It will be
        // passed to the methods of the `TypeArray` implementation, but that trait is `unsafe` and
        // implementations are required to uphold the invariants of `unsafe_clone`.
        let mut env = unsafe { env.unsafe_clone() };

        let mut is_copy: jboolean = 0xff;
        let ptr = unsafe { T::get(&mut env, array.as_raw(), &mut is_copy) }?;
        Ok(AutoElements {
            array,
            len,
            ptr: NonNull::new(ptr).ok_or(Error::NullPtr("Non-null ptr expected"))?,
            mode,
            is_copy: is_copy == sys::JNI_TRUE,
            env,
        })
    }

    pub(crate) fn new(
        env: &mut JNIEnv<'local>,
        array: &'array JPrimitiveArray<'other_local, T>,
        mode: ReleaseMode,
    ) -> Result<Self> {
        let len = env.get_array_length(array)? as usize;
        unsafe { Self::new_with_len(env, array, len, mode) }
    }

    /// Get a reference to the wrapped pointer
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Commits the changes to the array, if it is a copy
    pub fn commit(&mut self) -> Result<()> {
        unsafe { self.release_array_elements(sys::JNI_COMMIT) }
    }

    /// Calls the release function.
    ///
    /// # Safety
    ///
    /// `mode` must be a valid parameter to the JNI `Release<PrimitiveType>ArrayElements`' `mode`
    /// parameter.
    ///
    /// If `mode` is not [`sys::JNI_COMMIT`], then `self.ptr` must not have already been released.
    unsafe fn release_array_elements(&mut self, mode: i32) -> Result<()> {
        T::release(&mut self.env, self.array.as_raw(), self.ptr, mode)
    }

    /// Don't copy back the changes to the array on release (if it is a copy).
    ///
    /// This has no effect if the array is not a copy.
    ///
    /// This method is useful to change the release mode of an array originally created
    /// with `ReleaseMode::CopyBack`.
    pub fn discard(&mut self) {
        self.mode = ReleaseMode::NoCopyBack;
    }

    /// Indicates if the array is a copy or not
    pub fn is_copy(&self) -> bool {
        self.is_copy
    }

    /// Returns the array length (number of elements)
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<'local, 'other_local, 'array, T: TypeArray>
    AsRef<AutoElements<'local, 'other_local, 'array, T>>
    for AutoElements<'local, 'other_local, 'array, T>
{
    fn as_ref(&self) -> &AutoElements<'local, 'other_local, 'array, T> {
        self
    }
}

impl<'local, 'other_local, 'array, T: TypeArray> Drop
    for AutoElements<'local, 'other_local, 'array, T>
{
    fn drop(&mut self) {
        // Safety: `self.mode` is valid and the array has not yet been released.
        let res = unsafe { self.release_array_elements(self.mode as i32) };

        match res {
            Ok(()) => {}
            Err(e) => error!("error releasing array: {:#?}", e),
        }
    }
}

impl<'local, 'other_local, 'array, T: TypeArray>
    From<&AutoElements<'local, 'other_local, 'array, T>> for *mut T
{
    fn from(other: &AutoElements<T>) -> *mut T {
        other.as_ptr()
    }
}

impl<'local, 'other_local, 'array, T: TypeArray> std::ops::Deref
    for AutoElements<'local, 'other_local, 'array, T>
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<'local, 'other_local, 'array, T: TypeArray> std::ops::DerefMut
    for AutoElements<'local, 'other_local, 'array, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_mut(), self.len) }
    }
}
