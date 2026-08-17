/*!
Utilities for reference counting Objective-C objects.

The utilities of the `rc` module provide ARC-like semantics for working with
Objective-C's reference counted objects in Rust.
A `StrongPtr` retains an object and releases the object when dropped.
A `WeakPtr` will not retain the object, but can be upgraded to a `StrongPtr`
and safely fails if the object has been deallocated.

These utilities are not intended to provide a fully safe interface, but can be
useful when writing higher-level Rust wrappers for Objective-C code.

For more information on Objective-C's reference counting, see Apple's documentation:
<https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/MemoryMgmt.html>

# Example

``` no_run
# #[macro_use] extern crate objc;
# use objc::rc::{autoreleasepool, StrongPtr};
# fn main() {
// StrongPtr will release the object when dropped
let obj = unsafe {
    StrongPtr::new(msg_send![class!(NSObject), new])
};

// Cloning retains the object an additional time
let cloned = obj.clone();
autoreleasepool(|| {
    // Autorelease consumes the StrongPtr, but won't
    // actually release until the end of an autoreleasepool
    cloned.autorelease();
});

// Weak references won't retain the object
let weak = obj.weak();
drop(obj);
assert!(weak.load().is_null());
# }
```
*/

mod strong;
mod weak;
mod autorelease;

pub use self::strong::StrongPtr;
pub use self::weak::WeakPtr;
pub use self::autorelease::autoreleasepool;

// These tests use NSObject, which isn't present for GNUstep
#[cfg(all(test, any(target_os = "macos", target_os = "ios")))]
mod tests {
    use runtime::Object;
    use super::StrongPtr;
    use super::autoreleasepool;

    #[test]
    fn test_strong_clone() {
        fn retain_count(obj: *mut Object) -> usize {
            unsafe { msg_send![obj, retainCount] }
        }

        let obj = unsafe {
            StrongPtr::new(msg_send![class!(NSObject), new])
        };
        assert!(retain_count(*obj) == 1);

        let cloned = obj.clone();
        assert!(retain_count(*cloned) == 2);
        assert!(retain_count(*obj) == 2);

        drop(obj);
        assert!(retain_count(*cloned) == 1);
    }

    #[test]
    fn test_weak() {
        let obj = unsafe {
            StrongPtr::new(msg_send![class!(NSObject), new])
        };
        let weak = obj.weak();

        let strong = weak.load();
        assert!(*strong == *obj);
        drop(strong);

        drop(obj);
        assert!(weak.load().is_null());
    }

    #[test]
    fn test_weak_copy() {
        let obj = unsafe {
            StrongPtr::new(msg_send![class!(NSObject), new])
        };
        let weak = obj.weak();

        let weak2 = weak.clone();
        let strong = weak2.load();
        assert!(*strong == *obj);
    }

    #[test]
    fn test_autorelease() {
        let obj = unsafe {
            StrongPtr::new(msg_send![class!(NSObject), new])
        };

        fn retain_count(obj: *mut Object) -> usize {
            unsafe { msg_send![obj, retainCount] }
        }
        let cloned = obj.clone();

        autoreleasepool(|| {
                        obj.autorelease();
                        assert!(retain_count(*cloned) == 2);
        });

        // make sure that the autoreleased value has been released
        assert!(retain_count(*cloned) == 1);
    }
}
