use std::marker::PhantomData;

use crate::sys::jobject;

#[cfg(doc)]
use crate::{objects::GlobalRef, JNIEnv};

/// Wrapper around [`sys::jobject`] that adds a lifetime to ensure that
/// the underlying JNI pointer won't be accessible to safe Rust code if the
/// object reference is released.
///
/// It matches C's representation of the raw pointer, so it can be used in any
/// of the extern function argument positions that would take a `jobject`.
///
/// Most other types in the `objects` module deref to this, as they do in the C
/// representation.
///
/// The lifetime `'local` represents the local reference frame that this
/// reference belongs to. See the [`JNIEnv`] documentation for more information
/// about local reference frames. If `'local` is `'static`, then this reference
/// does not belong to a local reference frame, that is, it is either null or a
/// [global reference][GlobalRef].
///
/// Note that an *owned* `JObject` is always a local reference and will never
/// have the `'static` lifetime. [`GlobalRef`] does implement
/// <code>[AsRef]&lt;JObject&lt;'static>></code>, but this only yields a
/// *borrowed* `&JObject<'static>`, never an owned `JObject<'static>`.
///
/// Local references belong to a single thread and are not safe to share across
/// threads. This type implements [`Send`] and [`Sync`] if and only if the
/// lifetime `'local` is `'static`.
#[repr(transparent)]
#[derive(Debug)]
pub struct JObject<'local> {
    internal: jobject,
    lifetime: PhantomData<&'local ()>,
}

unsafe impl Send for JObject<'static> {}
unsafe impl Sync for JObject<'static> {}

impl<'local> AsRef<JObject<'local>> for JObject<'local> {
    fn as_ref(&self) -> &JObject<'local> {
        self
    }
}

impl<'local> AsMut<JObject<'local>> for JObject<'local> {
    fn as_mut(&mut self) -> &mut JObject<'local> {
        self
    }
}

impl<'local> ::std::ops::Deref for JObject<'local> {
    type Target = jobject;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'local> JObject<'local> {
    /// Creates a [`JObject`] that wraps the given `raw` [`jobject`]
    ///
    /// # Safety
    ///
    /// `raw` may be a null pointer. If `raw` is not a null pointer, then:
    ///
    /// * `raw` must be a valid raw JNI local reference.
    /// * There must not be any other `JObject` representing the same local reference.
    /// * The lifetime `'local` must not outlive the local reference frame that the local reference
    ///   was created in.
    pub unsafe fn from_raw(raw: jobject) -> Self {
        Self {
            internal: raw,
            lifetime: PhantomData,
        }
    }

    /// Returns the raw JNI pointer.
    pub fn as_raw(&self) -> jobject {
        self.internal
    }

    /// Unwrap to the internal jni type.
    pub fn into_raw(self) -> jobject {
        self.internal
    }

    /// Creates a new null reference.
    ///
    /// Null references are always valid and do not belong to a local reference frame. Therefore,
    /// the returned `JObject` always has the `'static` lifetime.
    pub fn null() -> JObject<'static> {
        unsafe { JObject::from_raw(std::ptr::null_mut() as jobject) }
    }
}

impl<'local> std::default::Default for JObject<'local> {
    fn default() -> Self {
        Self::null()
    }
}
