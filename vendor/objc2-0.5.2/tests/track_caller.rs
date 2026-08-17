#![cfg(all(target_pointer_width = "64", not(feature = "catch-all")))]
#![allow(dead_code)]
//! Test that our use of #[track_caller] is making the correct line number
//! show up.
use std::panic;
use std::process::abort;
use std::ptr;
use std::sync::Mutex;

use objc2::encode::Encode;
use objc2::rc::{self, Allocated, Retained};
use objc2::runtime::{self, NSObject};
use objc2::{class, declare_class, msg_send, msg_send_id, mutability, ClassType, DeclaredClass};

#[path = "../src/rc/test_object.rs"]
#[allow(dead_code)]
mod test_object;
use self::test_object::RcTestObject;

static EXPECTED_MESSAGE: Mutex<String> = Mutex::new(String::new());
static EXPECTED_LINE: Mutex<u32> = Mutex::new(0);

struct PanicChecker(());

impl PanicChecker {
    fn new() -> Self {
        panic::set_hook(Box::new(|info| {
            let expected_message = EXPECTED_MESSAGE.lock().unwrap();
            let expected_line = EXPECTED_LINE.lock().unwrap();

            let payload = info.payload();
            let message = if let Some(payload) = payload.downcast_ref::<&'static str>() {
                payload.to_string()
            } else if let Some(payload) = payload.downcast_ref::<String>() {
                payload.clone()
            } else {
                format!("could not extract message: {payload:?}")
            };
            let location = info.location().expect("location");

            if !message.contains(&*expected_message) {
                eprintln!("expected {expected_message:?}, got: {message:?}");
                abort();
            }
            if location.file() != file!() {
                eprintln!("expected file {:?}, got: {:?}", file!(), location.file());
                abort();
            }
            if location.line() != *expected_line {
                eprintln!("expected line {expected_line}, got: {}", location.line());
                abort();
            }
        }));
        Self(())
    }

    fn assert_panics(&self, message: &str, line: u32, f: impl FnOnce()) {
        *EXPECTED_MESSAGE.lock().unwrap() = message.to_string();
        *EXPECTED_LINE.lock().unwrap() = line;

        let res = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            f();
        }));
        assert!(res.is_err());

        *EXPECTED_MESSAGE.lock().unwrap() = "unknown".to_string();
        *EXPECTED_LINE.lock().unwrap() = 0;
    }
}

impl Drop for PanicChecker {
    fn drop(&mut self) {
        let _ = panic::take_hook();
    }
}

#[test]
fn test_track_caller() {
    let checker = PanicChecker::new();

    #[cfg(debug_assertions)]
    {
        test_nil(&checker);
        test_verify(&checker);
        test_error_methods(&checker);
    }

    test_id_unwrap(&checker);

    #[cfg(feature = "catch-all")]
    test_catch_all(&checker);

    test_unwind(&checker);

    #[cfg(not(feature = "unstable-static-class"))]
    test_unknown_class(&checker);
}

fn test_nil(checker: &PanicChecker) {
    let nil: *mut NSObject = ptr::null_mut();

    let msg = "messsaging description to nil";
    checker.assert_panics(msg, line!() + 1, || {
        let _: *mut NSObject = unsafe { msg_send![nil, description] };
    });
    checker.assert_panics(msg, line!() + 1, || {
        let _: *mut NSObject = unsafe { msg_send![super(nil, NSObject::class()), description] };
    });
    checker.assert_panics(msg, line!() + 1, || {
        let _: Option<Retained<NSObject>> = unsafe { msg_send_id![nil, description] };
    });
}

fn test_verify(checker: &PanicChecker) {
    let obj = NSObject::new();

    let msg = "invalid message send to -[NSObject description]: expected return to have type code '@', but found 'v'";
    checker.assert_panics(msg, line!() + 1, || {
        let _: () = unsafe { msg_send![&obj, description] };
    });

    let msg = format!("invalid message send to -[NSObject hash]: expected return to have type code '{}', but found '@'", usize::ENCODING);
    checker.assert_panics(&msg, line!() + 1, || {
        let _: Option<Retained<NSObject>> = unsafe { msg_send_id![&obj, hash] };
    });
}

fn test_error_methods(checker: &PanicChecker) {
    let nil: *mut NSObject = ptr::null_mut();

    let msg = "messsaging someSelectorWithError: to nil";
    checker.assert_panics(msg, line!() + 1, || {
        let _: Result<(), Retained<NSObject>> = unsafe { msg_send![nil, someSelectorWithError: _] };
    });
    checker.assert_panics(msg, line!() + 2, || {
        let _: Result<(), Retained<NSObject>> =
            unsafe { msg_send![super(nil, NSObject::class()), someSelectorWithError: _] };
    });
    checker.assert_panics(msg, line!() + 2, || {
        let _: Result<Retained<NSObject>, Retained<NSObject>> =
            unsafe { msg_send_id![nil, someSelectorWithError: _] };
    });

    let msg = "invalid message send to -[NSObject someSelectorWithError:]: method not found";
    checker.assert_panics(msg, line!() + 3, || {
        let obj = RcTestObject::new();
        let _: Result<(), Retained<NSObject>> =
            unsafe { msg_send![super(&obj), someSelectorWithError: _] };
    });
}

fn test_id_unwrap(checker: &PanicChecker) {
    let cls = RcTestObject::class();
    let obj = RcTestObject::new();

    let msg = "failed creating new instance using +[__RcTestObject newReturningNull]";
    checker.assert_panics(msg, line!() + 1, || {
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![cls, newReturningNull] };
    });

    let msg = if cfg!(debug_assertions) {
        "messsaging init to nil"
    } else {
        "failed allocating object"
    };
    checker.assert_panics(msg, line!() + 2, || {
        let obj: Allocated<RcTestObject> = unsafe { msg_send_id![cls, allocReturningNull] };
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![obj, init] };
    });

    let msg = "failed initializing object with -initReturningNull";
    checker.assert_panics(msg, line!() + 2, || {
        let _obj: Retained<RcTestObject> =
            unsafe { msg_send_id![RcTestObject::alloc(), initReturningNull] };
    });

    let msg = "failed copying object";
    checker.assert_panics(msg, line!() + 1, || {
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![&obj, copyReturningNull] };
    });

    let msg = "unexpected NULL returned from -[__RcTestObject methodReturningNull]";
    checker.assert_panics(msg, line!() + 1, || {
        let _obj: Retained<RcTestObject> = unsafe { msg_send_id![&obj, methodReturningNull] };
    });
}

fn test_catch_all(checker: &PanicChecker) {
    let obj: Retained<NSObject> = unsafe { msg_send_id![class!(NSArray), new] };

    let msg = "NSRangeException";
    checker.assert_panics(msg, line!() + 1, || {
        let _: *mut NSObject = unsafe { msg_send![&obj, objectAtIndex: 0usize] };
    });

    let msg = "NSRangeException";
    checker.assert_panics(msg, line!() + 1, || {
        let _: Retained<NSObject> = unsafe { msg_send_id![&obj, objectAtIndex: 0usize] };
    });
}

declare_class!(
    struct PanickingClass;

    unsafe impl ClassType for PanickingClass {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "PanickingClass";
    }

    impl DeclaredClass for PanickingClass {}

    unsafe impl PanickingClass {
        #[method(panic)]
        fn _panic() -> *mut Self {
            panic!("panic in PanickingClass")
        }
    }
);

fn test_unwind(checker: &PanicChecker) {
    let msg = "panic in PanickingClass";
    let line = line!() - 7;
    checker.assert_panics(msg, line, || {
        let _: *mut NSObject = unsafe { msg_send![PanickingClass::class(), panic] };
    });
    checker.assert_panics(msg, line, || {
        let _: Retained<NSObject> = unsafe { msg_send_id![PanickingClass::class(), panic] };
    });
}

#[cfg(not(feature = "unstable-static-class"))]
fn test_unknown_class(checker: &PanicChecker) {
    let msg = "class NonExistantClass could not be found";
    checker.assert_panics(msg, line!() + 1, || {
        let _ = class!(NonExistantClass);
    });
}
