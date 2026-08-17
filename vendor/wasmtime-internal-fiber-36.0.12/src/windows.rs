use crate::{RunResult, RuntimeFiberStack};
use alloc::boxed::Box;
use std::cell::Cell;
use std::ffi::c_void;
use std::io;
use std::ops::Range;
use std::ptr;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::Threading::*;

pub type Error = io::Error;

#[derive(Debug)]
pub struct FiberStack(usize);

impl FiberStack {
    pub fn new(size: usize, zeroed: bool) -> io::Result<Self> {
        // We don't support fiber stack zeroing on windows.
        let _ = zeroed;

        Ok(Self(size))
    }

    pub unsafe fn from_raw_parts(
        _base: *mut u8,
        _guard_size: usize,
        _len: usize,
    ) -> io::Result<Self> {
        Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32))
    }

    pub fn is_from_raw_parts(&self) -> bool {
        false
    }

    pub fn from_custom(_custom: Box<dyn RuntimeFiberStack>) -> io::Result<Self> {
        Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32))
    }

    pub fn top(&self) -> Option<*mut u8> {
        None
    }

    pub fn range(&self) -> Option<Range<usize>> {
        None
    }

    pub fn guard_range(&self) -> Option<Range<*mut u8>> {
        None
    }
}

pub struct Fiber {
    fiber: *mut c_void,
    state: Box<StartState>,
}

pub struct Suspend {
    state: *const StartState,
}

struct StartState {
    parent: Cell<*mut c_void>,
    initial_closure: Cell<*mut u8>,
    result_location: Cell<*const u8>,
}

const FIBER_FLAG_FLOAT_SWITCH: u32 = 1;

unsafe extern "C" {
    #[wasmtime_versioned_export_macros::versioned_link]
    fn wasmtime_fiber_get_current() -> *mut c_void;
}

unsafe extern "system" fn fiber_start<F, A, B, C>(data: *mut c_void)
where
    F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
{
    unsafe {
        // Set the stack guarantee to be consistent with what Rust expects for threads
        // This value is taken from:
        // https://github.com/rust-lang/rust/blob/0d97f7a96877a96015d70ece41ad08bb7af12377/library/std/src/sys/windows/stack_overflow.rs
        if SetThreadStackGuarantee(&mut 0x5000) == 0 {
            panic!("failed to set fiber stack guarantee");
        }

        let state = data.cast::<StartState>();
        let func = Box::from_raw((*state).initial_closure.get().cast::<F>());
        (*state).initial_closure.set(ptr::null_mut());
        let suspend = Suspend { state };
        let initial = suspend.take_resume::<A, B, C>();
        super::Suspend::<A, B, C>::execute(suspend, initial, *func);
    }
}

impl Fiber {
    pub fn new<F, A, B, C>(stack: &FiberStack, func: F) -> io::Result<Self>
    where
        F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
    {
        unsafe {
            let state = Box::new(StartState {
                initial_closure: Cell::new(Box::into_raw(Box::new(func)).cast()),
                parent: Cell::new(ptr::null_mut()),
                result_location: Cell::new(ptr::null()),
            });

            let fiber = CreateFiberEx(
                0,
                stack.0,
                FIBER_FLAG_FLOAT_SWITCH,
                Some(fiber_start::<F, A, B, C>),
                &*state as *const StartState as *mut _,
            );

            if fiber.is_null() {
                drop(Box::from_raw(state.initial_closure.get().cast::<F>()));
                return Err(io::Error::last_os_error());
            }

            Ok(Self { fiber, state })
        }
    }

    pub(crate) fn resume<A, B, C>(&self, _stack: &FiberStack, result: &Cell<RunResult<A, B, C>>) {
        unsafe {
            let is_fiber = IsThreadAFiber() != 0;
            let parent_fiber = if is_fiber {
                wasmtime_fiber_get_current()
            } else {
                ConvertThreadToFiber(ptr::null_mut())
            };
            assert!(
                !parent_fiber.is_null(),
                "failed to make current thread a fiber"
            );
            self.state
                .result_location
                .set(result as *const _ as *const _);
            self.state.parent.set(parent_fiber);
            SwitchToFiber(self.fiber);
            self.state.parent.set(ptr::null_mut());
            self.state.result_location.set(ptr::null());
            if !is_fiber {
                let res = ConvertFiberToThread();
                assert!(res != 0, "failed to convert main thread back");
            }
        }
    }

    pub(crate) unsafe fn drop<A, B, C>(&mut self) {}
}

impl Drop for Fiber {
    fn drop(&mut self) {
        unsafe {
            DeleteFiber(self.fiber);
        }
    }
}

impl Suspend {
    pub(crate) fn switch<A, B, C>(&self, result: RunResult<A, B, C>) -> A {
        unsafe {
            (*self.result_location::<A, B, C>()).set(result);
            debug_assert!(IsThreadAFiber() != 0);
            let parent = (*self.state).parent.get();
            debug_assert!(!parent.is_null());
            SwitchToFiber(parent);
            self.take_resume::<A, B, C>()
        }
    }

    pub(crate) fn exit<A, B, C>(&mut self, result: RunResult<A, B, C>) {
        self.switch(result);
        unreachable!()
    }

    unsafe fn take_resume<A, B, C>(&self) -> A {
        unsafe {
            match (*self.result_location::<A, B, C>()).replace(RunResult::Executing) {
                RunResult::Resuming(val) => val,
                _ => panic!("not in resuming state"),
            }
        }
    }

    unsafe fn result_location<A, B, C>(&self) -> *const Cell<RunResult<A, B, C>> {
        unsafe {
            let ret = (*self.state)
                .result_location
                .get()
                .cast::<Cell<RunResult<A, B, C>>>();
            assert!(!ret.is_null());
            return ret;
        }
    }
}
