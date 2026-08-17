use libc::c_void;
use std::io;
use std::ptr;
use windows_sys::core::BOOL;
use windows_sys::Win32::System::Memory::VirtualQuery;
use windows_sys::Win32::System::Threading::{
    ConvertFiberToThread, ConvertThreadToFiber, CreateFiber, DeleteFiber, IsThreadAFiber,
    SetThreadStackGuarantee, SwitchToFiber,
};

// Make sure the libstacker.a (implemented in C) is linked.
// See https://github.com/rust-lang/rust/issues/65610
#[link(name = "stacker")]
extern "C" {
    fn __stacker_get_current_fiber() -> *mut c_void;
}

struct FiberInfo<F> {
    callback: std::mem::MaybeUninit<F>,
    panic: Option<Box<dyn std::any::Any + Send + 'static>>,
    parent_fiber: *mut c_void,
}

unsafe extern "system" fn fiber_proc<F: FnOnce()>(data: *mut c_void) {
    // This function is the entry point to our inner fiber, and as argument we get an
    // instance of `FiberInfo`. We will set-up the "runtime" for the callback and execute
    // it.
    let data = &mut *(data as *mut FiberInfo<F>);
    let old_stack_limit = crate::get_stack_limit();
    crate::set_stack_limit(guess_os_stack_limit());
    let callback = data.callback.as_ptr();
    data.panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback.read())).err();

    // Restore to the previous Fiber
    crate::set_stack_limit(old_stack_limit);
    SwitchToFiber(data.parent_fiber);
}

pub fn _grow(stack_size: usize, callback: &mut dyn FnMut()) {
    // Fibers (or stackful coroutines) is the only official way to create new stacks on the
    // same thread on Windows. So in order to extend the stack we create fiber and switch
    // to it so we can use it's stack. After running `callback` within our fiber, we switch
    // back to the current stack and destroy the fiber and its associated stack.
    unsafe {
        let was_fiber = IsThreadAFiber() == 1 as BOOL;
        let mut data = FiberInfo {
            callback: std::mem::MaybeUninit::new(callback),
            panic: None,
            parent_fiber: {
                if was_fiber {
                    // Get a handle to the current fiber. We need to use a C implementation
                    // for this as GetCurrentFiber is an header only function.
                    __stacker_get_current_fiber()
                } else {
                    // Convert the current thread to a fiber, so we are able to switch back
                    // to the current stack. Threads coverted to fibers still act like
                    // regular threads, but they have associated fiber data. We later
                    // convert it back to a regular thread and free the fiber data.
                    ConvertThreadToFiber(ptr::null_mut())
                }
            },
        };

        if data.parent_fiber.is_null() {
            panic!(
                "unable to convert thread to fiber: {}",
                io::Error::last_os_error()
            );
        }

        let fiber = CreateFiber(
            stack_size as usize,
            Some(fiber_proc::<&mut dyn FnMut()>),
            &mut data as *mut FiberInfo<&mut dyn FnMut()> as *mut _,
        );
        if fiber.is_null() {
            panic!("unable to allocate fiber: {}", io::Error::last_os_error());
        }

        // Switch to the fiber we created. This changes stacks and starts executing
        // fiber_proc on it. fiber_proc will run `callback` and then switch back to run the
        // next statement.
        SwitchToFiber(fiber);
        DeleteFiber(fiber);

        // Clean-up.
        if !was_fiber && ConvertFiberToThread() == 0 {
            // FIXME: Perhaps should not panic here?
            panic!(
                "unable to convert back to thread: {}",
                io::Error::last_os_error()
            );
        }

        if let Some(p) = data.panic {
            std::panic::resume_unwind(p);
        }
    }
}

#[inline(always)]
fn get_thread_stack_guarantee() -> Option<usize> {
    let min_guarantee = if cfg!(target_pointer_width = "32") {
        0x1000
    } else {
        0x2000
    };
    let mut stack_guarantee = 0;
    unsafe {
        // Read the current thread stack guarantee
        // This is the stack reserved for stack overflow
        // exception handling.
        // This doesn't return the true value so we need
        // some further logic to calculate the real stack
        // guarantee. This logic is what is used on x86-32 and
        // x86-64 Windows 10. Other versions and platforms may differ
        let ret = SetThreadStackGuarantee(&mut stack_guarantee);
        if ret == 0 {
            return None;
        }
    };
    Some(std::cmp::max(stack_guarantee, min_guarantee) as usize + 0x1000)
}

#[inline(always)]
pub unsafe fn guess_os_stack_limit() -> Option<usize> {
    // Query the allocation which contains our stack pointer in order
    // to discover the size of the stack
    //
    // FIXME: we could read stack base from the TIB, specifically the 3rd element of it.
    type QueryT = windows_sys::Win32::System::Memory::MEMORY_BASIC_INFORMATION;
    let mut mi = std::mem::MaybeUninit::<QueryT>::uninit();
    let res = VirtualQuery(
        psm::stack_pointer() as *const _,
        mi.as_mut_ptr(),
        std::mem::size_of::<QueryT>() as usize,
    );
    if res == 0 {
        return None;
    }
    Some(mi.assume_init().AllocationBase as usize + get_thread_stack_guarantee()? + 0x1000)
}
