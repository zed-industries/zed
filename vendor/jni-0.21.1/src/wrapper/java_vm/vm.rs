use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
    thread::{current, Thread},
};

use log::{debug, error};

use crate::{errors::*, sys, JNIEnv};

#[cfg(feature = "invocation")]
use {
    crate::InitArgs,
    std::os::raw::c_void,
    std::{ffi::OsStr, path::PathBuf},
};

/// The Java VM, providing [Invocation API][invocation-api] support.
///
/// The JavaVM can be obtained either via [`JNIEnv#get_java_vm`][get-vm] in an already attached
/// thread, or it can be [launched](#launching-jvm-from-rust) from Rust via `JavaVM#new`.
///
/// ## Attaching Native Threads
///
/// A native thread must «attach» itself to be able to call Java methods outside of a native Java
/// method. This library provides two modes of attachment, each ensuring the thread is promptly
/// detached:
/// * A scoped attachment with [`attach_current_thread`][act].
///   The thread will automatically detach itself once the returned guard is dropped.
/// * A permanent attachment with [`attach_current_thread_permanently`][actp]
///   or [`attach_current_thread_as_daemon`][actd].
///   The thread will automatically detach itself before it terminates.
///
/// As attachment and detachment of a thread is an expensive operation, the scoped attachment
/// shall be used if happens infrequently. If you have an undefined scope where you need
/// to use `JNIEnv` and cannot keep the `AttachGuard`, consider attaching the thread
/// permanently.
///
/// ### Local Reference Management
///
/// Remember that the native thread attached to the VM **must** manage the local references
/// properly, i.e., do not allocate an excessive number of references and release them promptly
/// when they are no longer needed to enable the GC to collect them. A common approach is to use
/// an appropriately-sized local frame for larger code fragments
/// (see [`with_local_frame`](struct.JNIEnv.html#method.with_local_frame) and [Executor](#executor))
/// and [auto locals](struct.JNIEnv.html#method.auto_local) in loops.
///
/// See also the [JNI specification][spec-references] for details on referencing Java objects.
///
/// ### Executor
///
/// Jni-rs provides an [`Executor`](struct.Executor.html) — a helper struct that allows to
/// execute a closure with `JNIEnv`. It combines the performance benefits of permanent attaches
/// *and* automatic local reference management. Prefer it to manual permanent attaches if
/// they happen in various parts of the code to reduce the burden of local reference management.
///
/// ## Launching JVM from Rust
///
/// To [launch][launch-vm] a JVM from a native process, enable the `invocation`
/// feature in the Cargo.toml:
///
/// ```toml
/// jni = { version = "0.21.1", features = ["invocation"] }
/// ```
///
/// The application will be able to use [`JavaVM::new`] which will dynamically
/// load a `jvm` library (which is distributed with the JVM) at runtime:
///
/// ```rust
/// # use jni::errors;
/// # //
/// # // Ignore this test without invocation feature, so that simple `cargo test` works
/// # #[cfg(feature = "invocation")]
/// # fn main() -> errors::StartJvmResult<()> {
/// # use jni::{AttachGuard, objects::JValue, InitArgsBuilder, JNIEnv, JNIVersion, JavaVM, sys::jint};
/// # //
/// // Build the VM properties
/// let jvm_args = InitArgsBuilder::new()
///           // Pass the JNI API version (default is 8)
///           .version(JNIVersion::V8)
///           // You can additionally pass any JVM options (standard, like a system property,
///           // or VM-specific).
///           // Here we enable some extra JNI checks useful during development
///           .option("-Xcheck:jni")
///           .build()
///           .unwrap();
///
/// // Create a new VM
/// let jvm = JavaVM::new(jvm_args)?;
///
/// // Attach the current thread to call into Java — see extra options in
/// // "Attaching Native Threads" section.
/// //
/// // This method returns the guard that will detach the current thread when dropped,
/// // also freeing any local references created in it
/// let mut env = jvm.attach_current_thread()?;
///
/// // Call Java Math#abs(-10)
/// let x = JValue::from(-10);
/// let val: jint = env.call_static_method("java/lang/Math", "abs", "(I)I", &[x])?
///   .i()?;
///
/// assert_eq!(val, 10);
///
/// # Ok(()) }
/// #
/// # // This is a stub that gets run instead if the invocation feature is not built
/// # #[cfg(not(feature = "invocation"))]
/// # fn main() {}
/// ```
///
/// At runtime, the JVM installation path is determined via the [java-locator] crate:
/// 1. By the `JAVA_HOME` environment variable, if it is set.
/// 2. Otherwise — from `java` output.
///
/// It is recommended to set `JAVA_HOME`
///
/// For the operating system to correctly load the `jvm` library it may also be
/// necessary to update the path that the OS uses to find dependencies of the
/// `jvm` library.
/// * On **Windows**, append the path to `$JAVA_HOME/bin` to the `PATH` environment variable.
/// * On **MacOS**, append the path to `libjvm.dylib` to `LD_LIBRARY_PATH` environment variable.
/// * On **Linux**, append the path to `libjvm.so` to `LD_LIBRARY_PATH` environment variable.
///
/// The exact relative path to `jvm` library is version-specific.
///
/// [invocation-api]: https://docs.oracle.com/en/java/javase/12/docs/specs/jni/invocation.html
/// [get-vm]: struct.JNIEnv.html#method.get_java_vm
/// [launch-vm]: struct.JavaVM.html#method.new
/// [act]: struct.JavaVM.html#method.attach_current_thread
/// [actp]: struct.JavaVM.html#method.attach_current_thread_permanently
/// [actd]: struct.JavaVM.html#method.attach_current_thread_as_daemon
/// [spec-references]: https://docs.oracle.com/en/java/javase/12/docs/specs/jni/design.html#referencing-java-objects
/// [java-locator]: https://crates.io/crates/java-locator
#[repr(transparent)]
#[derive(Debug)]
pub struct JavaVM(*mut sys::JavaVM);

unsafe impl Send for JavaVM {}
unsafe impl Sync for JavaVM {}

impl JavaVM {
    /// Launch a new JavaVM using the provided init args.
    ///
    /// Unlike original JNI API, the main thread (the thread from which this method is called) will
    /// not be attached to JVM. You must explicitly use `attach_current_thread…` methods (refer
    /// to [Attaching Native Threads section](#attaching-native-threads)).
    ///
    /// *This API requires the "invocation" feature to be enabled,
    /// see ["Launching JVM from Rust"](struct.JavaVM.html#launching-jvm-from-rust).*
    ///
    /// This will attempt to locate a JVM using
    /// [java-locator], if the JVM has not already been loaded. Use the
    /// [`with_libjvm`][Self::with_libjvm] method to give an explicit location for the JVM shared
    /// library (`jvm.dll`, `libjvm.so`, or `libjvm.dylib`, depending on the platform).
    #[cfg(feature = "invocation")]
    pub fn new(args: InitArgs) -> StartJvmResult<Self> {
        Self::with_libjvm(args, || {
            Ok([
                java_locator::locate_jvm_dyn_library()
                    .map_err(StartJvmError::NotFound)?
                    .as_str(),
                java_locator::get_jvm_dyn_lib_file_name(),
            ]
            .iter()
            .collect::<PathBuf>())
        })
    }

    /// Launch a new JavaVM using the provided init args, loading it from the given shared library file if it's not already loaded.
    ///
    /// Unlike original JNI API, the main thread (the thread from which this method is called) will
    /// not be attached to JVM. You must explicitly use `attach_current_thread…` methods (refer
    /// to [Attaching Native Threads section](#attaching-native-threads)).
    ///
    /// *This API requires the "invocation" feature to be enabled,
    /// see ["Launching JVM from Rust"](struct.JavaVM.html#launching-jvm-from-rust).*
    ///
    /// The `libjvm_path` parameter takes a *closure* which returns the path to the JVM shared
    /// library. The closure is only called if the JVM is not already loaded. Any work that needs
    /// to be done to locate the JVM shared library should be done inside that closure.
    #[cfg(feature = "invocation")]
    pub fn with_libjvm<P: AsRef<OsStr>>(
        args: InitArgs,
        libjvm_path: impl FnOnce() -> StartJvmResult<P>,
    ) -> StartJvmResult<Self> {
        // Determine the path to the shared library.
        let libjvm_path = libjvm_path()?;
        let libjvm_path_string = libjvm_path.as_ref().to_string_lossy().into_owned();

        // Try to load it.
        let libjvm = match unsafe { libloading::Library::new(libjvm_path.as_ref()) } {
            Ok(ok) => ok,
            Err(error) => return Err(StartJvmError::LoadError(libjvm_path_string, error)),
        };

        unsafe {
            // Try to find the `JNI_CreateJavaVM` function in the loaded library.
            let create_fn = libjvm
                .get(b"JNI_CreateJavaVM\0")
                .map_err(|error| StartJvmError::LoadError(libjvm_path_string.to_owned(), error))?;

            // Create the JVM.
            Self::with_create_fn_ptr(args, *create_fn).map_err(StartJvmError::Create)
        }
    }

    #[cfg(feature = "invocation")]
    unsafe fn with_create_fn_ptr(
        args: InitArgs,
        create_fn_ptr: unsafe extern "system" fn(
            pvm: *mut *mut sys::JavaVM,
            penv: *mut *mut c_void,
            args: *mut c_void,
        ) -> sys::jint,
    ) -> Result<Self> {
        let mut ptr: *mut sys::JavaVM = ::std::ptr::null_mut();
        let mut env: *mut sys::JNIEnv = ::std::ptr::null_mut();

        jni_error_code_to_result(create_fn_ptr(
            &mut ptr as *mut _,
            &mut env as *mut *mut sys::JNIEnv as *mut *mut c_void,
            args.inner_ptr(),
        ))?;

        let vm = Self::from_raw(ptr)?;
        java_vm_unchecked!(vm.0, DetachCurrentThread);

        Ok(vm)
    }

    /// Create a JavaVM from a raw pointer.
    ///
    /// # Safety
    ///
    /// Expects a valid pointer retrieved from the `JNI_CreateJavaVM` JNI function. Only does null check.
    pub unsafe fn from_raw(ptr: *mut sys::JavaVM) -> Result<Self> {
        non_null!(ptr, "from_raw ptr argument");
        Ok(JavaVM(ptr))
    }

    /// Returns underlying `sys::JavaVM` interface.
    pub fn get_java_vm_pointer(&self) -> *mut sys::JavaVM {
        self.0
    }

    /// Attaches the current thread to the JVM. Calling this for a thread that is already attached
    /// is a no-op.
    ///
    /// The thread will detach itself automatically when it exits.
    ///
    /// Attached threads [block JVM exit][block]. If it is not desirable — consider using
    /// [`attach_current_thread_as_daemon`][attach-as-daemon].
    ///
    /// [block]: https://docs.oracle.com/en/java/javase/12/docs/specs/jni/invocation.html#unloading-the-vm
    /// [attach-as-daemon]: struct.JavaVM.html#method.attach_current_thread_as_daemon
    pub fn attach_current_thread_permanently(&self) -> Result<JNIEnv> {
        match self.get_env() {
            Ok(env) => Ok(env),
            Err(_) => self.attach_current_thread_impl(ThreadType::Normal),
        }
    }

    /// Attaches the current thread to the Java VM. The returned `AttachGuard`
    /// can be dereferenced to a `JNIEnv` and automatically detaches the thread
    /// when dropped. Calling this in a thread that is already attached is a no-op, and
    /// will neither change its daemon status nor prematurely detach it.
    ///
    /// Attached threads [block JVM exit][block].
    ///
    /// Attaching and detaching a thread is an expensive operation. If you use it frequently
    /// in the same threads, consider either [attaching them permanently][attach-as-daemon],
    /// or, if the scope where you need the `JNIEnv` is well-defined, keeping the returned guard.
    ///
    /// [block]: https://docs.oracle.com/en/java/javase/12/docs/specs/jni/invocation.html#unloading-the-vm
    /// [attach-as-daemon]: struct.JavaVM.html#method.attach_current_thread_as_daemon
    pub fn attach_current_thread(&self) -> Result<AttachGuard> {
        match self.get_env() {
            Ok(env) => Ok(AttachGuard::new_nested(env)),
            Err(_) => {
                let env = self.attach_current_thread_impl(ThreadType::Normal)?;
                Ok(AttachGuard::new(env))
            }
        }
    }

    /// Explicitly detaches the current thread from the JVM.
    ///
    /// _**Note**: This operation is _rarely_ appropriate to use, because the
    /// attachment methods [ensure](#attaching-native-threads) that the thread
    /// is automatically detached._
    ///
    /// Detaching a non-attached thread is a no-op.
    ///
    /// To support the use of `JavaVM::destroy()` it may be necessary to use this API to
    /// explicitly detach daemon threads before `JavaVM::destroy()` is called because
    /// `JavaVM::destroy()` does not synchronize and wait for daemon threads.
    ///
    /// Any daemon thread that is still "attached" after `JavaVM::destroy()` returns would
    /// cause undefined behaviour if it then tries to make any JNI calls or tries
    /// to detach itself.
    ///
    /// Normally `jni-rs` will automatically detach threads from the `JavaVM` by storing
    /// a guard in thread-local-storage that will detach on `Drop` but this will cause
    /// undefined behaviour if `JavaVM::destroy()` has been called.
    ///
    /// Calling this will clear the thread-local-storage guard and detach the thread
    /// early to avoid any attempt to automatically detach when the thread exits.
    ///
    /// # Safety
    ///
    /// __Any existing `JNIEnv`s and `AttachGuard`s created in the calling thread
    /// will be invalidated after this method completes. It is the__ caller’s __responsibility
    /// to ensure that no JNI calls are subsequently performed on these objects.__
    /// Failure to do so will result in unspecified errors, possibly, the process crash.
    ///
    /// Given some care is exercised, this method can be used to detach permanently attached
    /// threads _before_ they exit (when automatic detachment occurs). However, it is
    /// never appropriate to use it with the scoped attachment (`attach_current_thread`).
    // This method is hidden because it is almost never needed and its use requires some
    // extra care. Its status might be reconsidered if we learn of any use cases that require it.
    pub unsafe fn detach_current_thread(&self) {
        InternalAttachGuard::clear_tls();
    }

    /// Attaches the current thread to the Java VM as a _daemon_. Calling this in a thread
    /// that is already attached is a no-op, and will not change its status to a daemon thread.
    ///
    /// The thread will detach itself automatically when it exits.
    pub fn attach_current_thread_as_daemon(&self) -> Result<JNIEnv> {
        match self.get_env() {
            Ok(env) => Ok(env),
            Err(_) => self.attach_current_thread_impl(ThreadType::Daemon),
        }
    }

    /// Returns the current number of threads attached to the JVM.
    ///
    /// This method is provided mostly for diagnostic purposes.
    pub fn threads_attached(&self) -> usize {
        ATTACHED_THREADS.load(Ordering::SeqCst)
    }

    /// Get the `JNIEnv` associated with the current thread, or
    /// `ErrorKind::Detached`
    /// if the current thread is not attached to the java VM.
    pub fn get_env(&self) -> Result<JNIEnv> {
        let mut ptr = ptr::null_mut();
        unsafe {
            let res = java_vm_unchecked!(self.0, GetEnv, &mut ptr, sys::JNI_VERSION_1_1);
            jni_error_code_to_result(res)?;

            JNIEnv::from_raw(ptr as *mut sys::JNIEnv)
        }
    }

    /// Creates `InternalAttachGuard` and attaches current thread.
    fn attach_current_thread_impl(&self, thread_type: ThreadType) -> Result<JNIEnv> {
        let guard = InternalAttachGuard::new(self.get_java_vm_pointer());
        let env_ptr = unsafe {
            if thread_type == ThreadType::Daemon {
                guard.attach_current_thread_as_daemon()?
            } else {
                guard.attach_current_thread()?
            }
        };

        InternalAttachGuard::fill_tls(guard);

        unsafe { JNIEnv::from_raw(env_ptr as *mut sys::JNIEnv) }
    }

    /// Unloads the JavaVM and frees all it's associated resources
    ///
    /// Firstly if this thread is not already attached to the `JavaVM` then
    /// it will be attached.
    ///
    /// This thread will then wait until there are no other non-daemon threads
    /// attached to the `JavaVM` before unloading it (including threads spawned
    /// by Java and those that are attached via JNI)
    ///
    /// # Safety
    ///
    /// IF YOU ARE USING DAEMON THREADS THIS MAY BE DIFFICULT TO USE SAFELY!
    ///
    /// ## Daemon thread rules
    ///
    /// Since the JNI spec makes it clear that `DestroyJavaVM` will not wait for
    /// attached deamon threads to exit, this also means that if you do have any
    /// attached daemon threads it is your responsibility to ensure that they
    /// don't try and use JNI after the `JavaVM` is destroyed and you won't be able
    /// to detach them after the `JavaVM` has been destroyed.
    ///
    /// This creates a very unsafe hazard in `jni-rs` because it normally automatically
    /// ensures that any thread that gets attached will be detached before it exits.
    ///
    /// Normally `jni-rs` will automatically detach threads from the `JavaVM` by storing
    /// a guard in thread-local-storage that will detach on `Drop` but this will cause
    /// undefined behaviour if `JavaVM::destroy()` has been called before the thread
    /// exits.
    ///
    /// To clear this thread-local-storage guard from daemon threads you can call
    /// [`JavaVM::detach_current_thread()`] within each daemon thread, before calling
    /// this API.
    ///
    /// Calling this will clear the thread-local-storage guard and detach the thread
    /// early to avoid any attempt to automatically detach when the thread exits.
    ///
    /// ## Don't call from a Java native function
    ///
    /// There must be no Java methods on the call stack when `JavaVM::destroy()` is called.
    ///
    /// ## Drop all JNI state, including auto-release types before calling `JavaVM::destroy()`
    ///
    /// There is currently no `'vm` lifetime associated with a `JavaVM` that
    /// would allow the borrow checker to enforce that all `jni` resources
    /// associated with the `JavaVM` have been released.
    ///
    /// Since these JNI resources could lead to undefined behaviour through any
    /// use after the `JavaVM` has been destroyed then it is your responsibility
    /// to release these resources.
    ///
    /// In particular, there are numerous auto-release types in the `jni` API
    /// that will automatically make JNI calls within their `Drop`
    /// implementation. All such types _must_ be dropped before `destroy()` is
    /// called to avoid undefined bahaviour.
    ///
    /// Here is an non-exhaustive list of auto-release types to consider:
    /// - `AttachGuard`
    /// - `AutoElements`
    /// - `AutoElementsCritical`
    /// - `AutoLocal`
    /// - `GlobalRef`
    /// - `JavaStr`
    /// - `JMap`
    /// - `WeakRef`
    ///
    /// ## Invalid `JavaVM` on return
    ///
    /// After `destroy()` returns then the `JavaVM` will be in an undefined state
    /// and must be dropped (e.g. via `std::mem::drop()`) to avoid undefined behaviour.
    ///
    /// This method doesn't take ownership of the `JavaVM` before it is
    /// destroyed because the `JavaVM` may have been shared (E.g. via an `Arc`)
    /// between all the threads that have not yet necessarily exited before this
    /// is called.
    ///
    /// So although the `JavaVM` won't necessarily be solely owned by this
    /// thread when `destroy()` is first called it will conceptually own the
    /// `JavaVM` before `destroy()` returns.
    pub unsafe fn destroy(&self) -> Result<()> {
        unsafe {
            let res = java_vm_unchecked!(self.0, DestroyJavaVM);
            jni_error_code_to_result(res)
        }
    }
}

thread_local! {
    static THREAD_ATTACH_GUARD: RefCell<Option<InternalAttachGuard>> = RefCell::new(None)
}

static ATTACHED_THREADS: AtomicUsize = AtomicUsize::new(0);

/// A RAII implementation of scoped guard which detaches the current thread
/// when dropped. The attached `JNIEnv` can be accessed through this guard
/// via its `Deref` implementation.
pub struct AttachGuard<'local> {
    env: JNIEnv<'local>,
    should_detach: bool,
}

impl<'local> AttachGuard<'local> {
    /// AttachGuard created with this method will detach current thread on drop
    fn new(env: JNIEnv<'local>) -> Self {
        Self {
            env,
            should_detach: true,
        }
    }

    /// AttachGuard created with this method will not detach current thread on drop, which is
    /// the case for nested attaches.
    fn new_nested(env: JNIEnv<'local>) -> Self {
        Self {
            env,
            should_detach: false,
        }
    }
}

impl<'local> Deref for AttachGuard<'local> {
    type Target = JNIEnv<'local>;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl<'local> DerefMut for AttachGuard<'local> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.env
    }
}

impl<'local> Drop for AttachGuard<'local> {
    fn drop(&mut self) {
        if self.should_detach {
            InternalAttachGuard::clear_tls();
        }
    }
}

#[derive(PartialEq)]
enum ThreadType {
    Normal,
    Daemon,
}

#[derive(Debug)]
struct InternalAttachGuard {
    java_vm: *mut sys::JavaVM,
    /// A call std::thread::current() function can panic in case the local data has been destroyed
    /// before the thead local variables. The possibility of this happening depends on the platform
    /// implementation of the crate::sys_common::thread_local_dtor::register_dtor_fallback.
    /// The InternalAttachGuard is a thread-local vairable, so capture the thread meta-data
    /// during creation
    thread: Thread,
}

impl InternalAttachGuard {
    fn new(java_vm: *mut sys::JavaVM) -> Self {
        Self {
            java_vm,
            thread: current(),
        }
    }

    /// Stores guard in thread local storage.
    fn fill_tls(guard: InternalAttachGuard) {
        THREAD_ATTACH_GUARD.with(move |f| {
            *f.borrow_mut() = Some(guard);
        });
    }

    /// Clears thread local storage, dropping the InternalAttachGuard and causing detach of
    /// the current thread.
    fn clear_tls() {
        THREAD_ATTACH_GUARD.with(move |f| {
            *f.borrow_mut() = None;
        });
    }

    unsafe fn attach_current_thread(&self) -> Result<*mut sys::JNIEnv> {
        let mut env_ptr = ptr::null_mut();
        let res = java_vm_unchecked!(
            self.java_vm,
            AttachCurrentThread,
            &mut env_ptr,
            ptr::null_mut()
        );
        jni_error_code_to_result(res)?;

        ATTACHED_THREADS.fetch_add(1, Ordering::SeqCst);

        debug!(
            "Attached thread {} ({:?}). {} threads attached",
            self.thread.name().unwrap_or_default(),
            self.thread.id(),
            ATTACHED_THREADS.load(Ordering::SeqCst)
        );

        Ok(env_ptr as *mut sys::JNIEnv)
    }

    unsafe fn attach_current_thread_as_daemon(&self) -> Result<*mut sys::JNIEnv> {
        let mut env_ptr = ptr::null_mut();
        let res = java_vm_unchecked!(
            self.java_vm,
            AttachCurrentThreadAsDaemon,
            &mut env_ptr,
            ptr::null_mut()
        );
        jni_error_code_to_result(res)?;

        ATTACHED_THREADS.fetch_add(1, Ordering::SeqCst);

        debug!(
            "Attached daemon thread {} ({:?}). {} threads attached",
            self.thread.name().unwrap_or_default(),
            self.thread.id(),
            ATTACHED_THREADS.load(Ordering::SeqCst)
        );

        Ok(env_ptr as *mut sys::JNIEnv)
    }

    fn detach(&mut self) -> Result<()> {
        unsafe {
            java_vm_unchecked!(self.java_vm, DetachCurrentThread);
        }
        ATTACHED_THREADS.fetch_sub(1, Ordering::SeqCst);
        debug!(
            "Detached thread {} ({:?}). {} threads remain attached",
            self.thread.name().unwrap_or_default(),
            self.thread.id(),
            ATTACHED_THREADS.load(Ordering::SeqCst)
        );

        Ok(())
    }
}

impl Drop for InternalAttachGuard {
    fn drop(&mut self) {
        if let Err(e) = self.detach() {
            error!(
                "Error detaching current thread: {:#?}\nThread {} id={:?}",
                e,
                self.thread.name().unwrap_or_default(),
                self.thread.id(),
            );
        }
    }
}
