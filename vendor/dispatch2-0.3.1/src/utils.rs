use alloc::boxed::Box;
use core::ffi::c_void;

pub(crate) extern "C" fn function_wrapper<F>(work_boxed: *mut c_void)
where
    F: FnOnce(),
{
    // Safety: we reconstruct from a Box.
    let work = unsafe { Box::from_raw(work_boxed.cast::<F>()) };

    (*work)();
}
