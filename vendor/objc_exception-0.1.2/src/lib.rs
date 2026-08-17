//! Rust interface for Objective-C's `@throw` and `@try`/`@catch` statements.

use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;

#[link(name = "objc", kind = "dylib")]
extern { }

extern {
    fn RustObjCExceptionThrow(exception: *mut c_void);
    fn RustObjCExceptionTryCatch(try: extern fn(*mut c_void),
            context: *mut c_void, error: *mut *mut c_void) -> c_int;
}

/// An opaque type representing any Objective-C object thrown as an exception.
pub enum Exception { }

/// Throws an Objective-C exception.
/// The argument must be a pointer to an Objective-C object.
///
/// Unsafe because this unwinds from Objective-C.
pub unsafe fn throw(exception: *mut Exception) -> ! {
    RustObjCExceptionThrow(exception as *mut _);
    unreachable!();
}

unsafe fn try_no_ret<F>(closure: F) -> Result<(), *mut Exception>
        where F: FnOnce() {
    extern fn try_objc_execute_closure<F>(closure: &mut Option<F>)
            where F: FnOnce() {
        // This is always passed Some, so it's safe to unwrap
        let closure = closure.take().unwrap();
        closure();
    }

    let f: extern fn(&mut Option<F>) = try_objc_execute_closure;
    let f: extern fn(*mut c_void) = mem::transmute(f);
    // Wrap the closure in an Option so it can be taken
    let mut closure = Some(closure);
    let context = &mut closure as *mut _ as *mut c_void;

    let mut exception = ptr::null_mut();
    let success = RustObjCExceptionTryCatch(f, context, &mut exception);

    if success == 0 {
        Ok(())
    } else {
        Err(exception as *mut _)
    }
}

/// Tries to execute the given closure and catches an Objective-C exception
/// if one is thrown.
///
/// Returns a `Result` that is either `Ok` if the closure succeeded without an
/// exception being thrown, or an `Err` with a pointer to an exception if one
/// was thrown. The exception is retained and so must be released.
///
/// Unsafe because this encourages unwinding through the closure from
/// Objective-C, which is not safe.
pub unsafe fn try<F, R>(closure: F) -> Result<R, *mut Exception>
        where F: FnOnce() -> R {
    let mut value = None;
    let result = {
        let value_ref = &mut value;
        try_no_ret(move || {
            *value_ref = Some(closure());
        })
    };
    // If the try succeeded, this was set so it's safe to unwrap
    result.map(|_| value.unwrap())
}

#[cfg(test)]
mod tests {
    use std::ptr;
    use super::{throw, try};

    #[test]
    fn test_try() {
        unsafe {
            let s = "Hello".to_string();
            let result = try(move || {
                if s.len() > 0 {
                    throw(ptr::null_mut());
                }
                s.len()
            });
            assert!(result.unwrap_err() == ptr::null_mut());

            let mut s = "Hello".to_string();
            let result = try(move || {
                s.push_str(", World!");
                s
            });
            assert!(result.unwrap() == "Hello, World!");
        }
    }
}
