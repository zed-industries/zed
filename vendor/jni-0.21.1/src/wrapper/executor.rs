use std::sync::Arc;

use crate::{errors::*, JNIEnv, JavaVM};

/// The capacity of local frames, allocated for attached threads by default. Same as the default
/// value Hotspot uses when calling native Java methods.
pub const DEFAULT_LOCAL_FRAME_CAPACITY: i32 = 32;

/// Thread attachment manager. It allows to execute closures in attached threads with automatic
/// local references management done with `with_local_frame`. It combines the performance benefits
/// of permanent attaches whilst removing the risk of local references leaks if used consistently.
///
/// Although all locals are freed on closure exit, it might be needed to manually free
/// locals _inside_ the closure if an unbounded number of them is created (e.g., in a loop).
/// See ["Local Reference Management"](struct.JavaVM.html#local-reference-management) for details.
///
/// Threads using the Executor are attached on the first invocation as daemons,
/// hence they do not block JVM exit. Finished threads detach automatically.
///
/// ## Example
///
/// ```rust
/// # use jni::errors;
/// # //
/// # // Ignore this test without invocation feature, so that simple `cargo test` works
/// # #[cfg(feature = "invocation")]
/// # fn main() -> errors::StartJvmResult<()> {
/// # //
/// # use jni::{objects::JValue, Executor, InitArgsBuilder, JavaVM, sys::jint};
/// # use std::sync::Arc;
/// # //
/// # let jvm_args = InitArgsBuilder::new()
/// #         .build()
/// #         .unwrap();
/// # // Create a new VM
/// # let jvm = Arc::new(JavaVM::new(jvm_args)?);
///
/// let exec = Executor::new(jvm);
///
/// let val: jint = exec.with_attached(|env| {
///    let x = JValue::from(-10);
///    env.call_static_method("java/lang/Math", "abs", "(I)I", &[x])?.i()
/// })?;
///
/// assert_eq!(val, 10);
///
/// # Ok(()) }
/// #
/// # // This is a stub that gets run instead if the invocation feature is not built
/// # #[cfg(not(feature = "invocation"))]
/// # fn main() {}
/// ```
#[derive(Clone)]
pub struct Executor {
    vm: Arc<JavaVM>,
}

impl Executor {
    /// Creates new Executor with specified JVM.
    pub fn new(vm: Arc<JavaVM>) -> Self {
        Self { vm }
    }

    /// Executes a provided closure, making sure that the current thread
    /// is attached to the JVM. Additionally ensures that local object references are freed after
    /// call.
    ///
    /// Allocates a local frame with the specified capacity.
    pub fn with_attached_capacity<F, T, E>(&self, capacity: i32, f: F) -> std::result::Result<T, E>
    where
        F: FnOnce(&mut JNIEnv) -> std::result::Result<T, E>,
        E: From<Error>,
    {
        assert!(capacity > 0, "capacity should be a positive integer");

        let mut jni_env = self.vm.attach_current_thread_as_daemon()?;
        jni_env.with_local_frame(capacity, |jni_env| f(jni_env))
    }

    /// Executes a provided closure, making sure that the current thread
    /// is attached to the JVM. Additionally ensures that local object references are freed after
    /// call.
    ///
    /// Allocates a local frame with
    /// [the default capacity](constant.DEFAULT_LOCAL_FRAME_CAPACITY.html).
    pub fn with_attached<F, T, E>(&self, f: F) -> std::result::Result<T, E>
    where
        F: FnOnce(&mut JNIEnv) -> std::result::Result<T, E>,
        E: From<Error>,
    {
        self.with_attached_capacity(DEFAULT_LOCAL_FRAME_CAPACITY, f)
    }
}
