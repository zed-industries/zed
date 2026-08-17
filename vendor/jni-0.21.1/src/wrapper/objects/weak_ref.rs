use std::sync::Arc;

use log::{debug, warn};

use crate::{
    errors::Result,
    objects::{GlobalRef, JObject},
    sys, JNIEnv, JavaVM,
};

// Note: `WeakRef` must not implement `Into<JObject>`! If it did, then it would be possible to
// wrap it in `AutoLocal`, which would cause undefined behavior upon drop as a result of calling
// the wrong JNI function to delete the reference.

/// A *weak* global JVM reference. These are global in scope like
/// [`GlobalRef`], and may outlive the `JNIEnv` they came from, but are
/// *not* guaranteed to not get collected until released.
///
/// `WeakRef` can be cloned to use _the same_ weak reference in different
/// contexts. If you want to create yet another weak ref to the same java object, call
/// [`WeakRef::clone_in_jvm`].
///
/// Underlying weak reference will be dropped, when the last instance
/// of `WeakRef` leaves its scope.
///
/// It is _recommended_ that a native thread that drops the weak reference is attached
/// to the Java thread (i.e., has an instance of `JNIEnv`). If the native thread is *not* attached,
/// the `WeakRef#drop` will print a warning and implicitly `attach` and `detach` it, which
/// significantly affects performance.

#[derive(Clone)]
pub struct WeakRef {
    inner: Arc<WeakRefGuard>,
}

struct WeakRefGuard {
    raw: sys::jweak,
    vm: JavaVM,
}

unsafe impl Send for WeakRef {}
unsafe impl Sync for WeakRef {}

impl WeakRef {
    /// Creates a new wrapper for a global reference.
    ///
    /// # Safety
    ///
    /// Expects a valid raw weak global reference that should be created with `NewWeakGlobalRef`
    /// JNI function.
    pub(crate) unsafe fn from_raw(vm: JavaVM, raw: sys::jweak) -> Self {
        WeakRef {
            inner: Arc::new(WeakRefGuard { raw, vm }),
        }
    }

    /// Returns the raw JNI weak reference.
    pub fn as_raw(&self) -> sys::jweak {
        self.inner.raw
    }

    /// Creates a new local reference to this object.
    ///
    /// This object may have already been garbage collected by the time this method is called. If
    /// so, this method returns `Ok(None)`. Otherwise, it returns `Ok(Some(r))` where `r` is the
    /// new local reference.
    ///
    /// If this method returns `Ok(Some(r))`, it is guaranteed that the object will not be garbage
    /// collected at least until `r` is deleted or becomes invalid.
    pub fn upgrade_local<'local>(&self, env: &JNIEnv<'local>) -> Result<Option<JObject<'local>>> {
        let r = env.new_local_ref(unsafe { JObject::from_raw(self.as_raw()) })?;

        // Per JNI spec, `NewLocalRef` will return a null pointer if the object was GC'd.
        if r.is_null() {
            Ok(None)
        } else {
            Ok(Some(r))
        }
    }

    /// Creates a new strong global reference to this object.
    ///
    /// This object may have already been garbage collected by the time this method is called. If
    /// so, this method returns `Ok(None)`. Otherwise, it returns `Ok(Some(r))` where `r` is the
    /// new strong global reference.
    ///
    /// If this method returns `Ok(Some(r))`, it is guaranteed that the object will not be garbage
    /// collected at least until `r` is dropped.
    pub fn upgrade_global(&self, env: &JNIEnv) -> Result<Option<GlobalRef>> {
        let r = env.new_global_ref(unsafe { JObject::from_raw(self.as_raw()) })?;

        // Unlike `NewLocalRef`, the JNI spec does *not* guarantee that `NewGlobalRef` will return a
        // null pointer if the object was GC'd, so we'll have to check.
        if env.is_same_object(&r, JObject::null())? {
            Ok(None)
        } else {
            Ok(Some(r))
        }
    }

    /// Checks if the object referred to by this `WeakRef` has been garbage collected.
    ///
    /// Note that garbage collection can happen at any moment, so a return of `Ok(true)` from this
    /// method does not guarantee that [`WeakRef::upgrade_local`] or [`WeakRef::upgrade_global`]
    /// will succeed.
    ///
    /// This is equivalent to
    /// <code>self.[is_same_object][WeakRef::is_same_object](env, [JObject::null]\())</code>.
    pub fn is_garbage_collected(&self, env: &JNIEnv) -> Result<bool> {
        self.is_same_object(env, JObject::null())
    }

    // The following methods are wrappers around those `JNIEnv` methods that make sense for a weak
    // reference. These methods exist because they use `JObject::from_raw` on the raw pointer of a
    // weak reference. Although this usage is sound, it is `unsafe`. It's also confusing because
    // `JObject` normally represents a strong reference.

    /// Returns true if this weak reference refers to the given object. Otherwise returns false.
    ///
    /// If `object` is [null][JObject::null], then this method is equivalent to
    /// [`WeakRef::is_garbage_collected`]: it returns true if the object referred to by this
    /// `WeakRef` has been garbage collected, or false if the object has not yet been garbage
    /// collected.
    pub fn is_same_object<'local, O>(&self, env: &JNIEnv<'local>, object: O) -> Result<bool>
    where
        O: AsRef<JObject<'local>>,
    {
        env.is_same_object(unsafe { JObject::from_raw(self.as_raw()) }, object)
    }

    /// Returns true if this weak reference refers to the same object as another weak reference.
    /// Otherwise returns false.
    ///
    /// This method will also return true if both weak references refer to an object that has been
    /// garbage collected.
    pub fn is_weak_ref_to_same_object(&self, env: &JNIEnv, other: &WeakRef) -> Result<bool> {
        self.is_same_object(env, unsafe { JObject::from_raw(other.as_raw()) })
    }

    /// Creates a new weak reference to the same object that this one refers to.
    ///
    /// `WeakRef` implements [`Clone`], which should normally be used whenever a new `WeakRef` to
    /// the same object is needed. However, that only increments an internal reference count and
    /// does not actually create a new weak reference in the JVM. If you specifically need to have
    /// the JVM create a new weak reference, use this method instead of `Clone`.
    ///
    /// This method returns `Ok(None)` if the object has already been garbage collected.
    pub fn clone_in_jvm(&self, env: &JNIEnv) -> Result<Option<WeakRef>> {
        env.new_weak_ref(unsafe { JObject::from_raw(self.as_raw()) })
    }
}

impl Drop for WeakRefGuard {
    fn drop(&mut self) {
        fn drop_impl(env: &JNIEnv, raw: sys::jweak) -> Result<()> {
            let internal = env.get_native_interface();
            // This method is safe to call in case of pending exceptions (see chapter 2 of the spec)
            jni_unchecked!(internal, DeleteWeakGlobalRef, raw);
            Ok(())
        }

        let res = match self.vm.get_env() {
            Ok(env) => drop_impl(&env, self.raw),
            Err(_) => {
                warn!("Dropping a WeakRef in a detached thread. Fix your code if this message appears frequently (see the WeakRef docs).");
                self.vm
                    .attach_current_thread()
                    .and_then(|env| drop_impl(&env, self.raw))
            }
        };

        if let Err(err) = res {
            debug!("error dropping weak ref: {:#?}", err);
        }
    }
}
