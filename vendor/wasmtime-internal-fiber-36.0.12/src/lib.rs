//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.

#![no_std]

#[cfg(any(feature = "std", unix, windows))]
#[macro_use]
extern crate std;
extern crate alloc;

use alloc::boxed::Box;
use anyhow::Error;
use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::Range;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod nostd;
        use nostd as imp;
    } else if #[cfg(miri)] {
        mod miri;
        use miri as imp;
    } else if #[cfg(windows)] {
        mod windows;
        use windows as imp;
    } else if #[cfg(unix)] {
        mod unix;
        use unix as imp;
    } else {
        compile_error!("fibers are not supported on this platform");
    }
}

// Our own stack switcher routines are used on Unix and no_std
// platforms, but not on Windows (it has its own fiber API).
#[cfg(any(unix, not(feature = "std")))]
pub(crate) mod stackswitch;

/// Represents an execution stack to use for a fiber.
pub struct FiberStack(imp::FiberStack);

fn _assert_send_sync() {
    fn _assert_send<T: Send>() {}
    fn _assert_sync<T: Sync>() {}

    _assert_send::<FiberStack>();
    _assert_sync::<FiberStack>();
}

pub type Result<T, E = imp::Error> = core::result::Result<T, E>;

impl FiberStack {
    /// Creates a new fiber stack of the given size.
    pub fn new(size: usize, zeroed: bool) -> Result<Self> {
        Ok(Self(imp::FiberStack::new(size, zeroed)?))
    }

    /// Creates a new fiber stack of the given size.
    pub fn from_custom(custom: Box<dyn RuntimeFiberStack>) -> Result<Self> {
        Ok(Self(imp::FiberStack::from_custom(custom)?))
    }

    /// Creates a new fiber stack with the given pointer to the bottom of the
    /// stack plus how large the guard size and stack size are.
    ///
    /// The bytes from `bottom` to `bottom.add(guard_size)` should all be
    /// guaranteed to be unmapped. The bytes from `bottom.add(guard_size)` to
    /// `bottom.add(guard_size + len)` should be addressable.
    ///
    /// # Safety
    ///
    /// This is unsafe because there is no validation of the given pointer.
    ///
    /// The caller must properly allocate the stack space with a guard page and
    /// make the pages accessible for correct behavior.
    pub unsafe fn from_raw_parts(bottom: *mut u8, guard_size: usize, len: usize) -> Result<Self> {
        Ok(Self(unsafe {
            imp::FiberStack::from_raw_parts(bottom, guard_size, len)?
        }))
    }

    /// Gets the top of the stack.
    ///
    /// Returns `None` if the platform does not support getting the top of the
    /// stack.
    pub fn top(&self) -> Option<*mut u8> {
        self.0.top()
    }

    /// Returns the range of where this stack resides in memory if the platform
    /// supports it.
    pub fn range(&self) -> Option<Range<usize>> {
        self.0.range()
    }

    /// Is this a manually-managed stack created from raw parts? If so, it is up
    /// to whoever created it to manage the stack's memory allocation.
    pub fn is_from_raw_parts(&self) -> bool {
        self.0.is_from_raw_parts()
    }

    /// Returns the range of memory that the guard page(s) reside in.
    pub fn guard_range(&self) -> Option<Range<*mut u8>> {
        self.0.guard_range()
    }
}

/// A creator of RuntimeFiberStacks.
pub unsafe trait RuntimeFiberStackCreator: Send + Sync {
    /// Creates a new RuntimeFiberStack with the specified size, guard pages should be included,
    /// memory should be zeroed.
    ///
    /// This is useful to plugin previously allocated memory instead of mmap'ing a new stack for
    /// every instance.
    fn new_stack(&self, size: usize, zeroed: bool) -> Result<Box<dyn RuntimeFiberStack>, Error>;
}

/// A fiber stack backed by custom memory.
pub unsafe trait RuntimeFiberStack: Send + Sync {
    /// The top of the allocated stack.
    fn top(&self) -> *mut u8;
    /// The valid range of the stack without guard pages.
    fn range(&self) -> Range<usize>;
    /// The range of the guard page(s)
    fn guard_range(&self) -> Range<*mut u8>;
}

pub struct Fiber<'a, Resume, Yield, Return> {
    stack: Option<FiberStack>,
    inner: imp::Fiber,
    done: Cell<bool>,
    _phantom: PhantomData<&'a (Resume, Yield, Return)>,
}

pub struct Suspend<Resume, Yield, Return> {
    inner: imp::Suspend,
    _phantom: PhantomData<(Resume, Yield, Return)>,
}

/// A structure that is stored on a stack frame of a call to `Fiber::resume`.
///
/// This is used to both transmit data to a fiber (the resume step) as well as
/// acquire data from a fiber (the suspension step).
enum RunResult<Resume, Yield, Return> {
    /// The fiber is currently executing meaning it picked up whatever it was
    /// resuming with and hasn't yet completed.
    Executing,

    /// Resume with this value. Called for each invocation of
    /// `Fiber::resume`.
    Resuming(Resume),

    /// The fiber hasn't finished but has provided the following value
    /// during its suspension.
    Yield(Yield),

    /// The fiber has completed with the provided value and can no
    /// longer be resumed.
    Returned(Return),

    /// The fiber execution panicked.
    #[cfg(feature = "std")]
    Panicked(Box<dyn core::any::Any + Send>),
}

impl<'a, Resume, Yield, Return> Fiber<'a, Resume, Yield, Return> {
    /// Creates a new fiber which will execute `func` on the given stack.
    ///
    /// This function returns a `Fiber` which, when resumed, will execute `func`
    /// to completion. When desired the `func` can suspend itself via
    /// `Fiber::suspend`.
    pub fn new(
        stack: FiberStack,
        func: impl FnOnce(Resume, &mut Suspend<Resume, Yield, Return>) -> Return + 'a,
    ) -> Result<Self> {
        let inner = imp::Fiber::new(&stack.0, func)?;

        Ok(Self {
            stack: Some(stack),
            inner,
            done: Cell::new(false),
            _phantom: PhantomData,
        })
    }

    /// Resumes execution of this fiber.
    ///
    /// This function will transfer execution to the fiber and resume from where
    /// it last left off.
    ///
    /// Returns `true` if the fiber finished or `false` if the fiber was
    /// suspended in the middle of execution.
    ///
    /// # Panics
    ///
    /// Panics if the current thread is already executing a fiber or if this
    /// fiber has already finished.
    ///
    /// Note that if the fiber itself panics during execution then the panic
    /// will be propagated to this caller.
    pub fn resume(&self, val: Resume) -> Result<Return, Yield> {
        assert!(!self.done.replace(true), "cannot resume a finished fiber");
        let result = Cell::new(RunResult::Resuming(val));
        self.inner.resume(&self.stack().0, &result);
        match result.into_inner() {
            RunResult::Resuming(_) | RunResult::Executing => unreachable!(),
            RunResult::Yield(y) => {
                self.done.set(false);
                Err(y)
            }
            RunResult::Returned(r) => Ok(r),
            #[cfg(feature = "std")]
            RunResult::Panicked(_payload) => {
                use std::panic;
                panic::resume_unwind(_payload);
            }
        }
    }

    /// Returns whether this fiber has finished executing.
    pub fn done(&self) -> bool {
        self.done.get()
    }

    /// Gets the stack associated with this fiber.
    pub fn stack(&self) -> &FiberStack {
        self.stack.as_ref().unwrap()
    }

    /// When this fiber has finished executing, reclaim its stack.
    pub fn into_stack(mut self) -> FiberStack {
        assert!(self.done());
        self.stack.take().unwrap()
    }
}

impl<Resume, Yield, Return> Suspend<Resume, Yield, Return> {
    /// Suspend execution of a currently running fiber.
    ///
    /// This function will switch control back to the original caller of
    /// `Fiber::resume`. This function will then return once the `Fiber::resume`
    /// function is called again.
    ///
    /// # Panics
    ///
    /// Panics if the current thread is not executing a fiber from this library.
    pub fn suspend(&mut self, value: Yield) -> Resume {
        self.inner
            .switch::<Resume, Yield, Return>(RunResult::Yield(value))
    }

    fn execute(
        inner: imp::Suspend,
        initial: Resume,
        func: impl FnOnce(Resume, &mut Suspend<Resume, Yield, Return>) -> Return,
    ) {
        let mut suspend = Suspend {
            inner,
            _phantom: PhantomData,
        };

        #[cfg(feature = "std")]
        let result = {
            use std::panic::{self, AssertUnwindSafe};
            let result = panic::catch_unwind(AssertUnwindSafe(|| (func)(initial, &mut suspend)));
            match result {
                Ok(result) => RunResult::Returned(result),
                Err(panic) => RunResult::Panicked(panic),
            }
        };

        // Note that it is sound to omit the `catch_unwind` here: it
        // will not result in unwinding going off the top of the fiber
        // stack, because the code on the fiber stack is invoked via
        // an extern "C" boundary which will panic on unwinds.
        #[cfg(not(feature = "std"))]
        let result = RunResult::Returned((func)(initial, &mut suspend));

        suspend.inner.exit::<Resume, Yield, Return>(result);
    }
}

impl<A, B, C> Drop for Fiber<'_, A, B, C> {
    fn drop(&mut self) {
        debug_assert!(self.done.get(), "fiber dropped without finishing");
        unsafe {
            self.inner.drop::<A, B, C>();
        }
    }
}

#[cfg(all(test))]
mod tests {
    use super::{Fiber, FiberStack};
    use alloc::string::ToString;
    use std::cell::Cell;
    use std::rc::Rc;

    fn fiber_stack(size: usize) -> FiberStack {
        FiberStack::new(size, false).unwrap()
    }

    #[test]
    fn small_stacks() {
        Fiber::<(), (), ()>::new(fiber_stack(0), |_, _| {})
            .unwrap()
            .resume(())
            .unwrap();
        Fiber::<(), (), ()>::new(fiber_stack(1), |_, _| {})
            .unwrap()
            .resume(())
            .unwrap();
    }

    #[test]
    fn smoke() {
        let hit = Rc::new(Cell::new(false));
        let hit2 = hit.clone();
        let fiber = Fiber::<(), (), ()>::new(fiber_stack(1024 * 1024), move |_, _| {
            hit2.set(true);
        })
        .unwrap();
        assert!(!hit.get());
        fiber.resume(()).unwrap();
        assert!(hit.get());
    }

    #[test]
    fn suspend_and_resume() {
        let hit = Rc::new(Cell::new(false));
        let hit2 = hit.clone();
        let fiber = Fiber::<(), (), ()>::new(fiber_stack(1024 * 1024), move |_, s| {
            s.suspend(());
            hit2.set(true);
            s.suspend(());
        })
        .unwrap();
        assert!(!hit.get());
        assert!(fiber.resume(()).is_err());
        assert!(!hit.get());
        assert!(fiber.resume(()).is_err());
        assert!(hit.get());
        assert!(fiber.resume(()).is_ok());
        assert!(hit.get());
    }

    #[test]
    fn backtrace_traces_to_host() {
        #[inline(never)] // try to get this to show up in backtraces
        fn look_for_me() {
            run_test();
        }
        fn assert_contains_host() {
            let trace = backtrace::Backtrace::new();
            println!("{trace:?}");
            assert!(
                trace
                .frames()
                .iter()
                .flat_map(|f| f.symbols())
                .filter_map(|s| Some(s.name()?.to_string()))
                .any(|s| s.contains("look_for_me"))
                // TODO: apparently windows unwind routines don't unwind through fibers, so this will always fail. Is there a way we can fix that?
                || cfg!(windows)
                // TODO: the system libunwind is broken (#2808)
                || cfg!(all(target_os = "macos", target_arch = "aarch64"))
                // TODO: see comments in `arm.rs` about how this seems to work
                // in gdb but not at runtime, unsure why at this time.
                || cfg!(target_arch = "arm")
                // asan does weird things
                || cfg!(asan)
                // miri is a bit of a stretch to get working here
                || cfg!(miri)
            );
        }

        fn run_test() {
            let fiber = Fiber::<(), (), ()>::new(fiber_stack(1024 * 1024), move |(), s| {
                assert_contains_host();
                s.suspend(());
                assert_contains_host();
                s.suspend(());
                assert_contains_host();
            })
            .unwrap();
            assert!(fiber.resume(()).is_err());
            assert!(fiber.resume(()).is_err());
            assert!(fiber.resume(()).is_ok());
        }

        look_for_me();
    }

    #[test]
    #[cfg(feature = "std")]
    fn panics_propagated() {
        use std::panic::{self, AssertUnwindSafe};

        let a = Rc::new(Cell::new(false));
        let b = SetOnDrop(a.clone());
        let fiber = Fiber::<(), (), ()>::new(fiber_stack(1024 * 1024), move |(), _s| {
            let _ = &b;
            panic!();
        })
        .unwrap();
        assert!(panic::catch_unwind(AssertUnwindSafe(|| fiber.resume(()))).is_err());
        assert!(a.get());

        struct SetOnDrop(Rc<Cell<bool>>);

        impl Drop for SetOnDrop {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
    }

    #[test]
    fn suspend_and_resume_values() {
        let fiber = Fiber::new(fiber_stack(1024 * 1024), move |first, s| {
            assert_eq!(first, 2.0);
            assert_eq!(s.suspend(4), 3.0);
            "hello".to_string()
        })
        .unwrap();
        assert_eq!(fiber.resume(2.0), Err(4));
        assert_eq!(fiber.resume(3.0), Ok("hello".to_string()));
    }
}
