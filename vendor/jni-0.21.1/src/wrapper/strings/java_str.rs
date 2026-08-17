use jni_sys::{jboolean, JNI_TRUE};
use std::{borrow::Cow, os::raw::c_char};

use log::warn;

use crate::{errors::*, objects::JString, strings::JNIStr, JNIEnv};

/// Reference to a string in the JVM. Holds a pointer to the array
/// returned by `GetStringUTFChars`. Calls `ReleaseStringUTFChars` on Drop.
/// Can be converted to a `&JNIStr` with the same cost as the `&CStr.from_ptr`
/// conversion.
pub struct JavaStr<'local, 'other_local: 'obj_ref, 'obj_ref> {
    internal: *const c_char,
    obj: &'obj_ref JString<'other_local>,
    env: JNIEnv<'local>,
}

impl<'local, 'other_local: 'obj_ref, 'obj_ref> JavaStr<'local, 'other_local, 'obj_ref> {
    /// Get a pointer to the character array beneath a [JString]
    ///
    /// The string will be `NULL` terminated and encoded as
    /// [Modified UTF-8](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8) /
    /// [CESU-8](https://en.wikipedia.org/wiki/CESU-8).
    ///
    /// The implementation may either create a copy of the character array for
    /// the given `String` or it may pin it to avoid it being collected by the
    /// garbage collector.
    ///
    /// Returns a tuple with the pointer and the status of whether the implementation
    /// created a copy of the underlying character array.
    ///
    /// # Warning
    ///
    /// The caller must release the array when they are done with it via
    /// [Self::release_string_utf_chars]
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the Object passed in is an instance of `java.lang.String`,
    /// passing in anything else will lead to undefined behaviour (The JNI implementation
    /// is likely to crash or abort the process).
    unsafe fn get_string_utf_chars(
        env: &JNIEnv<'_>,
        obj: &JString<'_>,
    ) -> Result<(*const c_char, bool)> {
        non_null!(obj, "get_string_utf_chars obj argument");
        let mut is_copy: jboolean = 0;
        let ptr: *const c_char = jni_non_null_call!(
            env.get_raw(),
            GetStringUTFChars,
            obj.as_raw(),
            &mut is_copy as *mut _
        );

        let is_copy = is_copy == JNI_TRUE;
        Ok((ptr, is_copy))
    }

    /// Release the backing string
    ///
    /// This will either free the copy that was made by `GetStringUTFChars` or unpin it so it
    /// may be released by the garbage collector once there are no further references to the string.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that [Self::internal] was constructed from a valid pointer obtained from [Self::get_string_utf_chars]
    unsafe fn release_string_utf_chars(&mut self) -> Result<()> {
        non_null!(self.obj, "release_string_utf_chars obj argument");
        // This method is safe to call in case of pending exceptions (see the chapter 2 of the spec)
        jni_unchecked!(
            self.env.get_raw(),
            ReleaseStringUTFChars,
            self.obj.as_raw(),
            self.internal
        );

        Ok(())
    }

    /// Get a [JavaStr] from a [JNIEnv] and a [JString].
    /// You probably want [JNIEnv::get_string] instead of this method.
    pub fn from_env(env: &JNIEnv<'local>, obj: &'obj_ref JString<'other_local>) -> Result<Self> {
        Ok(unsafe {
            let (ptr, _) = Self::get_string_utf_chars(env, obj)?;

            Self::from_raw(env, obj, ptr)
        })
    }

    /// Get the raw string pointer from the JavaStr.
    ///
    /// The string will be `NULL` terminated and encoded as
    /// [Modified UTF-8](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8) /
    /// [CESU-8](https://en.wikipedia.org/wiki/CESU-8).
    pub fn get_raw(&self) -> *const c_char {
        self.internal
    }

    /// Consumes the `JavaStr`, returning the raw string pointer
    ///
    /// The string will be `NULL` terminated and encoded as
    /// [Modified UTF-8](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8) /
    /// [CESU-8](https://en.wikipedia.org/wiki/CESU-8).
    ///
    /// # Warning
    /// The programmer is responsible for making sure the backing string gets
    /// released when they are done with it, for example by reconstructing a
    /// [JavaStr] with [`Self::from_raw`], which will release the backing string
    /// when it is dropped.
    pub fn into_raw(self) -> *const c_char {
        let mut _dont_call_drop = std::mem::ManuallyDrop::new(self);

        // Drop the `JNIEnv` in place. As of this writing, that's a no-op, but if `JNIEnv`
        // gains any drop code in the future, this will run it.
        //
        // Safety: The `&mut` proves that `self.env` is valid and not aliased. It is not
        // accessed again after this point. Because `self` has been moved into `ManuallyDrop`,
        // the `JNIEnv` will not be dropped twice.
        unsafe {
            std::ptr::drop_in_place(&mut _dont_call_drop.env);
        }

        _dont_call_drop.internal
    }

    /// Get a [JavaStr] from it's raw components
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `ptr` is a valid, non-null pointer returned by [`Self::into_raw`],
    /// and that `obj` is the same `String` object originally used to create the [JavaStr]
    ///
    /// # Example
    /// ```rust,no_run
    /// # use jni::{errors::Result, JNIEnv, strings::JavaStr};
    /// #
    /// # fn example(env: &mut JNIEnv) -> Result<()> {
    /// let jstring = env.new_string("foo")?;
    /// let java_str = env.get_string(&jstring)?;
    ///
    /// let ptr = java_str.into_raw();
    /// // Do whatever you need with the pointer
    /// let java_str = unsafe { JavaStr::from_raw(env, &jstring, ptr) };
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn from_raw(
        env: &JNIEnv<'local>,
        obj: &'obj_ref JString<'other_local>,
        ptr: *const c_char,
    ) -> Self {
        Self {
            internal: ptr,
            obj,

            // Safety: The cloned `JNIEnv` will not be used to create any local references, only to
            // release `ptr`.
            env: env.unsafe_clone(),
        }
    }
}

impl<'local, 'other_local: 'obj_ref, 'obj_ref> ::std::ops::Deref
    for JavaStr<'local, 'other_local, 'obj_ref>
{
    type Target = JNIStr;
    fn deref(&self) -> &Self::Target {
        self.into()
    }
}

impl<'local, 'other_local: 'obj_ref, 'obj_ref: 'java_str, 'java_str>
    From<&'java_str JavaStr<'local, 'other_local, 'obj_ref>> for &'java_str JNIStr
{
    fn from(other: &'java_str JavaStr) -> &'java_str JNIStr {
        unsafe { JNIStr::from_ptr(other.internal) }
    }
}

impl<'local, 'other_local: 'obj_ref, 'obj_ref: 'java_str, 'java_str>
    From<&'java_str JavaStr<'local, 'other_local, 'obj_ref>> for Cow<'java_str, str>
{
    fn from(other: &'java_str JavaStr) -> Cow<'java_str, str> {
        let jni_str: &JNIStr = other;
        jni_str.into()
    }
}

impl<'local, 'other_local: 'obj_ref, 'obj_ref> From<JavaStr<'local, 'other_local, 'obj_ref>>
    for String
{
    fn from(other: JavaStr) -> String {
        let cow: Cow<str> = (&other).into();
        cow.into_owned()
    }
}

impl<'local, 'other_local: 'obj_ref, 'obj_ref> Drop for JavaStr<'local, 'other_local, 'obj_ref> {
    fn drop(&mut self) {
        match unsafe { self.release_string_utf_chars() } {
            Ok(()) => {}
            Err(e) => warn!("error dropping java str: {}", e),
        }
    }
}
