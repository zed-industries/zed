use std::{
    marker::PhantomData,
    panic::{catch_unwind, resume_unwind, AssertUnwindSafe},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

use super::{signal::Signal, utils::is_web_worker_thread, Builder, JoinInner};

/// A scope to spawn scoped threads in.
///
/// See [`scope`] for details.
pub struct Scope<'scope, 'env: 'scope> {
    data: Arc<ScopeData>,
    /// Invariance over 'scope, to make sure 'scope cannot shrink,
    /// which is necessary for soundness.
    ///
    /// Without invariance, this would compile fine but be unsound:
    ///
    /// ```compile_fail,E0373
    /// std::thread::scope(|s| {
    ///     s.spawn(|| {
    ///         let a = String::from("abcd");
    ///         s.spawn(|| println!("{a:?}")); // might run after `a` is dropped
    ///     });
    /// });
    /// ```
    scope: PhantomData<&'scope mut &'scope ()>,
    env: PhantomData<&'env mut &'env ()>,
}

/// An owned permission to join on a scoped thread (block on its termination).
///
/// See [`Scope::spawn`] for details.
pub struct ScopedJoinHandle<'scope, T>(JoinInner<'scope, T>);

pub(crate) struct ScopeData {
    num_running_threads: AtomicUsize,
    a_thread_panicked: AtomicBool,
    signal: Signal,
}

impl ScopeData {
    pub(crate) fn increment_num_running_threads(&self) {
        // We check for 'overflow' with usize::MAX / 2, to make sure there's no
        // chance it overflows to 0, which would result in unsoundness.
        if self.num_running_threads.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            // This can only reasonably happen by mem::forget()'ing a lot of ScopedJoinHandles.
            self.decrement_num_running_threads(false);
            panic!("too many running threads in thread scope");
        }
    }

    pub(crate) fn decrement_num_running_threads(&self, panic: bool) {
        if panic {
            self.a_thread_panicked.store(true, Ordering::Relaxed);
        }

        if self.num_running_threads.fetch_sub(1, Ordering::Release) == 1 {
            // All threads have terminated
            self.signal.signal();
        }
    }
}

/// Create a scope for spawning scoped threads.
///
/// The function passed to `scope` will be provided a [`Scope`] object,
/// through which scoped threads can be [spawned][`Scope::spawn`].
///
/// Unlike non-scoped threads, scoped threads can borrow non-`'static` data,
/// as the scope guarantees all threads will be joined at the end of the scope.
///
/// All threads spawned within the scope that haven't been manually joined
/// will be automatically joined before this function returns.
///
/// # Panics
///
/// If any of the automatically joined threads panicked, this function will panic.
///
/// If you want to handle panics from spawned threads,
/// [`join`][ScopedJoinHandle::join] them before the end of the scope.
///
/// On wasm, this will panic on main thread because blocking join is not allowed.
pub fn scope<'env, F, T>(f: F) -> T
where
    F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T,
{
    // Fail early to avoid flaky panics that depend on execution time
    if !is_web_worker_thread() {
        panic!("scope is not allowed on the main thread");
    }

    // We put the `ScopeData` into an `Arc` so that other threads can finish their
    // `decrement_num_running_threads` even after this function returns.
    let scope = Scope {
        data: Arc::new(ScopeData {
            num_running_threads: AtomicUsize::new(0),
            a_thread_panicked: AtomicBool::new(false),
            signal: Signal::new(),
        }),
        env: PhantomData,
        scope: PhantomData,
    };

    // Run `f`, but catch panics so we can make sure to wait for all the threads to join.
    let result = catch_unwind(AssertUnwindSafe(|| f(&scope)));

    // Wait until all the threads are finished.
    while scope.data.num_running_threads.load(Ordering::Acquire) != 0 {
        scope.data.signal.wait();
    }

    // Throw any panic from `f`, or the return value of `f` if no thread panicked.
    match result {
        Err(e) => resume_unwind(e),
        Ok(_) if scope.data.a_thread_panicked.load(Ordering::Relaxed) => {
            panic!("a scoped thread panicked")
        }
        Ok(result) => result,
    }
}

impl<'scope, 'env> Scope<'scope, 'env> {
    /// Spawns a new thread within a scope, returning a [`ScopedJoinHandle`] for it.
    ///
    /// Unlike non-scoped threads, threads spawned with this function may
    /// borrow non-`'static` data from the outside the scope. See [`scope`] for
    /// details.
    ///
    /// The join handle provides a [`join`] method that can be used to join the spawned
    /// thread. If the spawned thread panics, [`join`] will return an [`Err`] containing
    /// the panic payload.
    ///
    /// If the join handle is dropped, the spawned thread will implicitly joined at the
    /// end of the scope. In that case, if the spawned thread panics, [`scope`] will
    /// panic after all threads are joined.
    ///
    /// This call will create a thread using default parameters of [`Builder`].
    /// If you want to specify the stack size or the name of the thread, use
    /// [`Builder::spawn_scoped`] instead.
    ///
    /// # Panics
    ///
    /// Panics if the OS fails to create a thread; use [`Builder::spawn_scoped`]
    /// to recover from such errors.
    ///
    /// [`join`]: ScopedJoinHandle::join
    pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    where
        F: FnOnce() -> T + Send + 'scope,
        T: Send + 'scope,
    {
        Builder::new().spawn_scoped(self, f).expect("failed to spawn thread")
    }
}

impl Builder {
    /// Spawns a new scoped thread using the settings set through this `Builder`.
    ///
    /// Unlike [Scope::spawn], this method yields an [std::io::Result] to
    /// capture any failure to create the thread at the OS level.
    pub fn spawn_scoped<'scope, 'env, F, T>(
        self,
        scope: &'scope Scope<'scope, 'env>,
        f: F,
    ) -> std::io::Result<ScopedJoinHandle<'scope, T>>
    where
        F: FnOnce() -> T + Send + 'scope,
        T: Send + 'scope,
    {
        Ok(ScopedJoinHandle(unsafe {
            self.spawn_unchecked_(f, Some(scope.data.clone()))
        }?))
    }
}

impl<'scope, T> ScopedJoinHandle<'scope, T> {
    pub fn join(self) -> super::Result<T> {
        self.0.join()
    }

    pub async fn join_async(self) -> super::Result<T> {
        self.0.join_async().await
    }
}
