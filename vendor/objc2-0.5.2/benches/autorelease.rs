use core::ffi::c_void;
use std::mem::ManuallyDrop;

use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::{AnyClass, NSObject, Sel};
use objc2::{class, msg_send, sel};

const BYTES: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

fn empty() {}

fn pool_cleanup() {
    autoreleasepool(|_| {});
}

fn class() -> &'static AnyClass {
    class!(NSObject)
}

fn sel() -> Sel {
    sel!(alloc)
}

fn send_message() -> &'static AnyClass {
    unsafe { msg_send![class!(NSObject), class] }
}

fn alloc_nsobject() -> *mut NSObject {
    unsafe { msg_send![class!(NSObject), alloc] }
}

fn new_nsobject() -> Retained<NSObject> {
    let obj = alloc_nsobject();
    let obj: *mut NSObject = unsafe { msg_send![obj, init] };
    unsafe { Retained::from_raw(obj).unwrap_unchecked() }
}

fn new_nsdata() -> Retained<NSObject> {
    let bytes_ptr: *const c_void = BYTES.as_ptr().cast();
    let obj: *mut NSObject = unsafe { msg_send![class!(NSData), alloc] };
    let obj: *mut NSObject = unsafe {
        msg_send![
            obj,
            initWithBytes: bytes_ptr,
            length: BYTES.len(),
        ]
    };
    unsafe { Retained::from_raw(obj).unwrap_unchecked() }
}

fn new_leaked_nsdata() -> *const NSObject {
    Retained::as_ptr(&*ManuallyDrop::new(new_nsdata()))
}

fn autoreleased_nsdata() -> *const NSObject {
    // let bytes_ptr: *const c_void = BYTES.as_ptr().cast();
    // unsafe {
    //     msg_send![
    //         class!(NSData),
    //         dataWithBytes: bytes_ptr,
    //         length: BYTES.len(),
    //     ]
    // }
    unsafe { msg_send![new_leaked_nsdata(), autorelease] }
}

fn new_nsstring() -> Retained<NSObject> {
    let obj: *mut NSObject = unsafe { msg_send![class!(NSString), alloc] };
    let obj: *mut NSObject = unsafe { msg_send![obj, init] };
    unsafe { Retained::from_raw(obj).unwrap_unchecked() }
}

fn new_leaked_nsstring() -> *const NSObject {
    Retained::as_ptr(&*ManuallyDrop::new(new_nsstring()))
}

fn autoreleased_nsstring() -> *const NSObject {
    // unsafe { msg_send![class!(NSString), string] }
    unsafe { msg_send![new_leaked_nsstring(), autorelease] }
}

fn retain_autoreleased(obj: *const NSObject) -> Retained<NSObject> {
    unsafe { Retained::retain_autoreleased((obj as *mut NSObject).cast()).unwrap_unchecked() }
}

fn autoreleased_nsdata_pool_cleanup() -> *const NSObject {
    autoreleasepool(|_| autoreleased_nsdata())
}

fn autoreleased_nsdata_fast_caller_cleanup() -> Retained<NSObject> {
    retain_autoreleased(autoreleased_nsdata())
}

fn autoreleased_nsdata_fast_caller_cleanup_pool_cleanup() -> Retained<NSObject> {
    autoreleasepool(|_| retain_autoreleased(autoreleased_nsdata()))
}

fn autoreleased_nsstring_pool_cleanup() -> *const NSObject {
    autoreleasepool(|_| autoreleased_nsstring())
}

fn autoreleased_nsstring_fast_caller_cleanup() -> Retained<NSObject> {
    retain_autoreleased(autoreleased_nsstring())
}

fn autoreleased_nsstring_fast_caller_cleanup_pool_cleanup() -> Retained<NSObject> {
    autoreleasepool(|_| retain_autoreleased(autoreleased_nsstring()))
}

macro_rules! main_with_warmup {
    ($($f:ident,)+) => {
        mod warmup_fns {
            $(
                #[inline(never)]
                pub(crate) fn $f() {
                    let _ = iai::black_box(super::$f());
                }
            )+
        }

        // Required to get DYLD to resolve the stubs on x86_64
        fn warmup() {
            $(
                warmup_fns::$f();
            )+
        }

        iai::main! {
            warmup,
            $(
                $f,
            )+
        }
    };
}

main_with_warmup! {
    // Baseline
    empty,
    pool_cleanup,
    class,
    sel,
    send_message,
    alloc_nsobject,
    new_nsobject,
    // NSData
    new_nsdata,
    new_leaked_nsdata,
    autoreleased_nsdata,
    autoreleased_nsdata_pool_cleanup,
    autoreleased_nsdata_fast_caller_cleanup,
    autoreleased_nsdata_fast_caller_cleanup_pool_cleanup,
    // NSString
    new_nsstring,
    new_leaked_nsstring,
    autoreleased_nsstring,
    autoreleased_nsstring_pool_cleanup,
    autoreleased_nsstring_fast_caller_cleanup,
    autoreleased_nsstring_fast_caller_cleanup_pool_cleanup,
}
