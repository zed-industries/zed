use core::mem::ManuallyDrop;

use objc2::msg_send;
use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::{NSObject, NSObjectProtocol};

fn create_obj() -> Retained<NSObject> {
    let obj = ManuallyDrop::new(NSObject::new());
    unsafe {
        let obj: *mut NSObject = msg_send![&*obj, autorelease];
        // All code between the `msg_send!` and the `retain_autoreleased` must
        // be able to be optimized away for this to work.
        Retained::retain_autoreleased(obj).unwrap()
    }
}

#[test]
fn test_retain_autoreleased() {
    autoreleasepool(|_| {
        // Run once to allow DYLD to resolve the symbol stubs.
        // Required for making `retain_autoreleased` work on x86_64.
        let _data = create_obj();

        // When compiled in release mode / with optimizations enabled,
        // subsequent usage of `retain_autoreleased` will succeed in retaining
        // the autoreleased value!
        #[allow(clippy::if_same_then_else)]
        let expected = if cfg!(feature = "gnustep-1-7") {
            1
        } else if cfg!(all(target_arch = "arm", panic = "unwind")) {
            // 32-bit ARM unwinding interferes with the optimization
            2
        } else if cfg!(any(debug_assertions, feature = "catch-all")) {
            2
        } else {
            1
        };

        let data = create_obj();
        assert_eq!(data.retainCount(), expected);

        let data = create_obj();
        assert_eq!(data.retainCount(), expected);

        // Here we manually clean up the autorelease, so it will always be 1.
        let data = autoreleasepool(|_| create_obj());
        assert_eq!(data.retainCount(), 1);
    });
}
