use std::{mem, ops::Deref, sync::Arc};

use log::{debug, warn};

use crate::{errors::Result, objects::JObject, sys, JNIEnv, JavaVM};

// Note: `GlobalRef` must not implement `Into<JObject>`! If it did, then it would be possible to
// wrap it in `AutoLocal`, which would cause undefined behavior upon drop as a result of calling
// the wrong JNI function to delete the reference.

/// A global JVM reference. These are "pinned" by the garbage collector and are
/// guaranteed to not get collected until released. Thus, this is allowed to
/// outlive the `JNIEnv` that it came from and can be used in other threads.
///
/// `GlobalRef` can be cloned to use _the same_ global reference in different
/// contexts. If you want to create yet another global ref to the same java object
/// you may call `JNIEnv#new_global_ref` just like you do when create `GlobalRef`
/// from a local reference.
///
/// Underlying global reference will be dropped, when the last instance
/// of `GlobalRef` leaves its scope.
///
/// It is _recommended_ that a native thread that drops the global reference is attached
/// to the Java thread (i.e., has an instance of `JNIEnv`). If the native thread is *not* attached,
/// the `GlobalRef#drop` will print a warning and implicitly `attach` and `detach` it, which
/// significantly affects performance.

#[derive(Clone, Debug)]
pub struct GlobalRef {
    inner: Arc<GlobalRefGuard>,
}

#[derive(Debug)]
struct GlobalRefGuard {
    obj: JObject<'static>,
    vm: JavaVM,
}

impl AsRef<GlobalRef> for GlobalRef {
    fn as_ref(&self) -> &GlobalRef {
        self
    }
}

impl AsRef<JObject<'static>> for GlobalRef {
    fn as_ref(&self) -> &JObject<'static> {
        self
    }
}

impl Deref for GlobalRef {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner.obj
    }
}

impl GlobalRef {
    /// Creates a new wrapper for a global reference.
    ///
    /// # Safety
    ///
    /// Expects a valid raw global reference that should be created with `NewGlobalRef` JNI function.
    pub(crate) unsafe fn from_raw(vm: JavaVM, raw_global_ref: sys::jobject) -> Self {
        GlobalRef {
            inner: Arc::new(GlobalRefGuard::from_raw(vm, raw_global_ref)),
        }
    }

    /// Get the object from the global ref
    ///
    /// This borrows the ref and prevents it from being dropped as long as the
    /// JObject sticks around.
    pub fn as_obj(&self) -> &JObject<'static> {
        self.as_ref()
    }
}

impl GlobalRefGuard {
    /// Creates a new global reference guard. This assumes that `NewGlobalRef`
    /// has already been called.
    unsafe fn from_raw(vm: JavaVM, obj: sys::jobject) -> Self {
        GlobalRefGuard {
            obj: JObject::from_raw(obj),
            vm,
        }
    }
}

impl Drop for GlobalRefGuard {
    fn drop(&mut self) {
        let raw: sys::jobject = mem::take(&mut self.obj).into_raw();

        let drop_impl = |env: &JNIEnv| -> Result<()> {
            let internal = env.get_native_interface();
            // This method is safe to call in case of pending exceptions (see chapter 2 of the spec)
            jni_unchecked!(internal, DeleteGlobalRef, raw);
            Ok(())
        };

        let res = match self.vm.get_env() {
            Ok(env) => drop_impl(&env),
            Err(_) => {
                warn!("Dropping a GlobalRef in a detached thread. Fix your code if this message appears frequently (see the GlobalRef docs).");
                self.vm
                    .attach_current_thread()
                    .and_then(|env| drop_impl(&env))
            }
        };

        if let Err(err) = res {
            debug!("error dropping global ref: {:#?}", err);
        }
    }
}
