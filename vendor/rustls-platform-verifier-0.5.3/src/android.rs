//! On Android, initialization must be done before any verification is attempted.
//!
//! <div class="warning">
//! Some manual setup is required outside of cargo to use this crate on Android. In order to use
//! Android’s certificate verifier, the crate needs to call into the JVM. A small Kotlin component
//! must be included in your app’s build to support rustls-platform-verifier.
//!
//! See the [crate's Android section][crate#android] for more details.
//! </div>
//!
//! # Examples
//!
//! ```
//! // A typical entrypoint signature for obtaining the necessary pointers
//! pub fn android_init(raw_env: *mut c_void, raw_context: *mut c_void) -> Result<(), jni::errors::Error> {
//!     let mut env = unsafe { JNIEnv::from_raw(raw_env as *mut jni::sys::JNIEnv).unwrap() };
//!     let context = unsafe { JObject::from_raw(raw_context as jni::sys::jobject) };
//!     rustls_platform_verifier::android::init_with_env(&mut env, context)?;
//! }
//! ```

use jni::errors::Error as JNIError;
use jni::objects::{GlobalRef, JClass, JObject, JValue};
use jni::{JNIEnv, JavaVM};
use once_cell::sync::OnceCell;

static GLOBAL: OnceCell<Global> = OnceCell::new();

/// A layer to access the Android runtime which is hosting the current
/// application process.
///
/// Generally this trait should be implemented in your Rust app component's FFI
/// initialization layer.
pub trait Runtime: Send + Sync {
    /// Returns a handle to the current process' JVM.
    fn java_vm(&self) -> &JavaVM;
    /// Returns a reference to the current app's [Context].
    ///
    /// [Context]: <https://developer.android.com/reference/android/content/Context>
    fn context(&self) -> &GlobalRef;
    /// Returns a reference to the class returned by the current JVM's `getClassLoader` call.
    fn class_loader(&self) -> &GlobalRef;
}

enum Global {
    Internal {
        java_vm: JavaVM,
        context: GlobalRef,
        loader: GlobalRef,
    },
    External(&'static dyn Runtime),
}

impl Global {
    fn env(&self) -> Result<JNIEnv, Error> {
        let vm = match self {
            Global::Internal { java_vm, .. } => java_vm,
            Global::External(global) => global.java_vm(),
        };
        Ok(vm.attach_current_thread_permanently()?)
    }

    fn context(&self) -> Result<Context, Error> {
        let env = self.env()?;

        let context = match self {
            Global::Internal { context, .. } => context,
            Global::External(global) => global.context(),
        };

        let loader = match self {
            Global::Internal { loader, .. } => loader,
            Global::External(global) => global.class_loader(),
        };

        Ok(Context {
            context: env.new_global_ref(context)?,
            loader: env.new_global_ref(loader)?,
            env,
        })
    }
}

fn global() -> &'static Global {
    GLOBAL
        .get()
        .expect("Expect rustls-platform-verifier to be initialized")
}

/// Initialize given a typical Android NDK [`JNIEnv`] and [`JObject`] context.
///
/// This method will setup and store an environment locally. This is useful if nothing else in your
/// application needs to access the Android runtime.
pub fn init_with_env(env: &mut JNIEnv, context: JObject) -> Result<(), JNIError> {
    GLOBAL.get_or_try_init(|| -> Result<_, JNIError> {
        let loader =
            env.call_method(&context, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?;

        Ok(Global::Internal {
            java_vm: env.get_java_vm()?,
            context: env.new_global_ref(context)?,
            loader: env.new_global_ref(JObject::try_from(loader)?)?,
        })
    })?;
    Ok(())
}

/// *Deprecated*: This is the original method name for [`init_with_env`] and is functionally
/// identical.
pub fn init_hosted(env: &mut JNIEnv, context: JObject) -> Result<(), JNIError> {
    init_with_env(env, context)
}

/// Initialize with a runtime that can dynamically serve references to
/// the JVM, context, and class loader.
///
/// This is the most flexible option, and is useful for advanced use cases.
///
/// This function will never panic.
pub fn init_with_runtime(runtime: &'static dyn Runtime) {
    GLOBAL.get_or_init(|| Global::External(runtime));
}

/// *Deprecated*: This is the original method name for [`init_with_runtime`] and is functionally
/// identical.
pub fn init_external(runtime: &'static dyn Runtime) {
    init_with_runtime(runtime);
}

/// Initialize with references to the JVM, context, and class loader.
///
/// This is useful when you're already interacting with `jni-rs` wrapped objects and want to use
/// global references to objects for efficiency.
///
/// This function will never panic.
///
/// # Examples
///
/// ```
/// pub fn android_init(raw_env: *mut c_void, raw_context: *mut c_void) -> Result<(), jni::errors::Error> {
///     let mut env = unsafe { jni::JNIEnv::from_raw(raw_env as *mut jni::sys::JNIEnv).unwrap() };
///     let context = unsafe { JObject::from_raw(raw_context as jni::sys::jobject) };
///     let loader = env.call_method(&context, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?;
///
///     rustls_platform_verifier::android::init_with_refs(
///         env.get_java_vm()?,
///         env.new_global_ref(context)?,
///         env.new_global_ref(JObject::try_from(loader)?)?,
///     );
/// }
/// ```
pub fn init_with_refs(java_vm: JavaVM, context: GlobalRef, loader: GlobalRef) {
    GLOBAL.get_or_init(|| Global::Internal {
        java_vm,
        context,
        loader,
    });
}

/// Wrapper for JNI errors that will log and clear exceptions
/// It should generally be preferred to `jni::errors::Error`
#[derive(Debug)]
pub(super) struct Error;

impl From<JNIError> for Error {
    #[track_caller]
    fn from(cause: JNIError) -> Self {
        if let JNIError::JavaException = cause {
            if let Ok(env) = global().env() {
                // These should not fail if we are already in a throwing state unless
                // things are very broken. In that case, there isn't much we can do.
                let _ = env.exception_describe();
                let _ = env.exception_clear();
            }
        }

        Self
    }
}

pub(super) struct Context<'a> {
    env: JNIEnv<'a>,
    context: GlobalRef,
    loader: GlobalRef,
}

impl<'a> Context<'a> {
    /// Borrow a reference to the JNI Environment executing the Android application
    pub(super) fn env(&mut self) -> &mut JNIEnv<'a> {
        &mut self.env
    }

    /// Borrow the `applicationContext` from the Android application
    /// <https://developer.android.com/reference/android/app/Application>
    pub(super) fn application_context(&self) -> &JObject<'a> {
        &self.context
    }

    /// Load a class from the application class loader
    ///
    /// This should be used instead of `JNIEnv::find_class` to ensure all classes
    /// in the application can be found.
    pub(super) fn load_class(&mut self, name: &str) -> Result<JClass<'a>, Error> {
        let name = self.env.new_string(name)?;
        let class = self.env.call_method(
            &self.loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[JValue::from(&name)],
        )?;

        Ok(JObject::try_from(class)?.into())
    }
}

/// Borrow the Android application context and execute the closure
/// `with_context, ensuring locals are properly freed and exceptions
/// are cleared.
pub(super) fn with_context<F, T>(f: F) -> Result<T, Error>
where
    F: FnOnce(&mut Context, &mut JNIEnv) -> Result<T, Error>,
{
    let mut context = global().context()?;
    let mut binding = global().context()?;
    let env = binding.env();

    // 16 is the default capacity in the JVM, we can make this configurable if necessary
    env.with_local_frame(16, |env| f(&mut context, env))
}

/// Loads and caches a class on first use
pub(super) struct CachedClass {
    name: &'static str,
    class: OnceCell<GlobalRef>,
}

impl CachedClass {
    /// Creates a lazily initialized class reference to the class with `name`.
    pub(super) const fn new(name: &'static str) -> Self {
        Self {
            name,
            class: OnceCell::new(),
        }
    }

    /// Gets the cached class reference, loaded on first use
    pub(super) fn get<'a: 'b, 'b>(&'a self, cx: &mut Context<'b>) -> Result<&'a JClass<'b>, Error> {
        let class = self.class.get_or_try_init(|| -> Result<_, Error> {
            let class = cx.load_class(self.name)?;

            Ok(cx.env().new_global_ref(class)?)
        })?;

        Ok(class.as_obj().into())
    }
}
