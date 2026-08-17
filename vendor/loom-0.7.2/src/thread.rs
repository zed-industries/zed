//! Mock implementation of `std::thread`.

pub use crate::rt::thread::AccessError;
pub use crate::rt::yield_now;
use crate::rt::{self, Execution, Location};

#[doc(no_inline)]
pub use std::thread::panicking;

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::{fmt, io};

use tracing::trace;

/// Mock implementation of `std::thread::JoinHandle`.
pub struct JoinHandle<T> {
    result: Arc<Mutex<Option<std::thread::Result<T>>>>,
    notify: rt::Notify,
    thread: Thread,
}

/// Mock implementation of `std::thread::Thread`.
#[derive(Clone, Debug)]
pub struct Thread {
    id: ThreadId,
    name: Option<String>,
}

impl Thread {
    /// Returns a unique identifier for this thread
    pub fn id(&self) -> ThreadId {
        self.id
    }

    /// Returns the (optional) name of this thread
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Mock implementation of [`std::thread::Thread::unpark`].
    ///
    /// Atomically makes the handle's token available if it is not already.
    ///
    /// Every thread is equipped with some basic low-level blocking support, via
    /// the [`park`] function and the `unpark()` method. These can be
    /// used as a more CPU-efficient implementation of a spinlock.
    ///
    /// See the [park documentation][park] for more details.
    pub fn unpark(&self) {
        rt::execution(|execution| execution.threads.unpark(self.id.id));
    }
}

/// Mock implementation of `std::thread::ThreadId`.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct ThreadId {
    id: crate::rt::thread::Id,
}

impl std::fmt::Debug for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ThreadId({})", self.id.public_id())
    }
}

/// Mock implementation of `std::thread::LocalKey`.
pub struct LocalKey<T> {
    // Sadly, these fields have to be public, since function pointers in const
    // fns are unstable. When fn pointer arguments to const fns stabilize, these
    // should be made private and replaced with a `const fn new`.
    //
    // User code should not rely on the existence of these fields.
    #[doc(hidden)]
    pub init: fn() -> T,
    #[doc(hidden)]
    pub _p: PhantomData<fn(T)>,
}

/// Thread factory, which can be used in order to configure the properties of
/// a new thread.
#[derive(Debug)]
pub struct Builder {
    name: Option<String>,
    stack_size: Option<usize>,
}

static CURRENT_THREAD_KEY: LocalKey<Thread> = LocalKey {
    init: || unreachable!(),
    _p: PhantomData,
};

fn init_current(execution: &mut Execution, name: Option<String>) -> Thread {
    let id = execution.threads.active_id();
    let thread = Thread {
        id: ThreadId { id },
        name,
    };

    execution
        .threads
        .local_init(&CURRENT_THREAD_KEY, thread.clone());

    thread
}

/// Returns a handle to the current thread.
pub fn current() -> Thread {
    rt::execution(|execution| {
        let thread = execution.threads.local(&CURRENT_THREAD_KEY);
        if let Some(thread) = thread {
            thread.unwrap().clone()
        } else {
            // Lazily initialize the current() Thread. This is done to help
            // handle the initial (unnamed) bootstrap thread.
            init_current(execution, None)
        }
    })
}

/// Mock implementation of `std::thread::spawn`.
///
/// Note that you may only have [`MAX_THREADS`](crate::MAX_THREADS) threads in a given loom tests
/// _including_ the main thread.
#[track_caller]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: 'static,
    T: 'static,
{
    spawn_internal(f, None, None, location!())
}

/// Mock implementation of `std::thread::park`.
///
///  Blocks unless or until the current thread's token is made available.
///
/// A call to `park` does not guarantee that the thread will remain parked
/// forever, and callers should be prepared for this possibility.
#[track_caller]
pub fn park() {
    rt::park(location!());
}

fn spawn_internal<F, T>(
    f: F,
    name: Option<String>,
    stack_size: Option<usize>,
    location: Location,
) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: 'static,
    T: 'static,
{
    let result = Arc::new(Mutex::new(None));
    let notify = rt::Notify::new(true, false);

    let id = {
        let name = name.clone();
        let result = result.clone();
        rt::spawn(stack_size, move || {
            rt::execution(|execution| {
                init_current(execution, name);
            });

            *result.lock().unwrap() = Some(Ok(f()));
            notify.notify(location);
        })
    };

    JoinHandle {
        result,
        notify,
        thread: Thread {
            id: ThreadId { id },
            name,
        },
    }
}

impl Builder {
    /// Generates the base configuration for spawning a thread, from which
    /// configuration methods can be chained.
    // `std::thread::Builder` does not implement `Default`, so this type does
    // not either, as it's a mock version of the `std` type.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Builder {
        Builder {
            name: None,
            stack_size: None,
        }
    }

    /// Names the thread-to-be. Currently the name is used for identification
    /// only in panic messages.
    pub fn name(mut self, name: String) -> Builder {
        self.name = Some(name);

        self
    }

    /// Sets the size of the stack (in bytes) for the new thread.
    pub fn stack_size(mut self, size: usize) -> Builder {
        self.stack_size = Some(size);

        self
    }

    /// Spawns a new thread by taking ownership of the `Builder`, and returns an
    /// `io::Result` to its `JoinHandle`.
    #[track_caller]
    pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        Ok(spawn_internal(f, self.name, self.stack_size, location!()))
    }
}

impl<T> JoinHandle<T> {
    /// Waits for the associated thread to finish.
    #[track_caller]
    pub fn join(self) -> std::thread::Result<T> {
        self.notify.wait(location!());
        self.result.lock().unwrap().take().unwrap()
    }

    /// Gets a handle to the underlying [`Thread`]
    pub fn thread(&self) -> &Thread {
        &self.thread
    }
}

impl<T: fmt::Debug> fmt::Debug for JoinHandle<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("JoinHandle").finish()
    }
}

fn _assert_traits() {
    fn assert<T: Send + Sync>() {}

    assert::<JoinHandle<()>>();
}

impl<T: 'static> LocalKey<T> {
    /// Mock implementation of `std::thread::LocalKey::with`.
    pub fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        self.try_with(f)
            .expect("cannot access a (mock) TLS value during or after it is destroyed")
    }

    /// Mock implementation of `std::thread::LocalKey::try_with`.
    pub fn try_with<F, R>(&'static self, f: F) -> Result<R, AccessError>
    where
        F: FnOnce(&T) -> R,
    {
        let value = match unsafe { self.get() } {
            Some(v) => v?,
            None => {
                // Init the value out of the `rt::execution`
                let value = (self.init)();

                rt::execution(|execution| {
                    trace!("LocalKey::try_with");

                    execution.threads.local_init(self, value);
                });

                unsafe { self.get() }.expect("bug")?
            }
        };
        Ok(f(value))
    }

    unsafe fn get(&'static self) -> Option<Result<&T, AccessError>> {
        unsafe fn transmute_lt<'a, 'b, T>(t: &'a T) -> &'b T {
            std::mem::transmute::<&'a T, &'b T>(t)
        }

        rt::execution(|execution| {
            trace!("LocalKey::get");

            let res = execution.threads.local(self)?;

            let local = match res {
                Ok(l) => l,
                Err(e) => return Some(Err(e)),
            };

            // This is, sadly, necessary to allow nested `with` blocks to access
            // different thread locals. The borrow on the thread-local needs to
            // escape the lifetime of the borrow on `execution`, since
            // `rt::execution` mutably borrows a RefCell, and borrowing it twice will
            // cause a panic. This should be safe, as we know the function being
            // passed the thread local will not outlive the thread on which
            // it's executing, by construction --- it's just kind of unfortunate.
            Some(Ok(transmute_lt(local)))
        })
    }
}

impl<T: 'static> fmt::Debug for LocalKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("LocalKey { .. }")
    }
}
