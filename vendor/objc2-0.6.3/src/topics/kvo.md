# Key-Value Observing

Key-Value Observing (KVO) is a mechanism that allows an object to be notified of changes to properties of another object.

This is exposed via the `NSKeyValueCoding` and `NSKeyValueObserving` categories / informal protocols on `NSObject`.

See Apple's documentation on [Key-Value Coding][kvc-doc], [Key-Value Observing][kvo-doc] and [Swift's documentation on KVO][swift-kvo] for more information.

[kvc-doc]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/KeyValueCoding/index.html
[kvo-doc]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/KeyValueObserving/KeyValueObserving.html
[swift-kvo]: https://developer.apple.com/documentation/swift/using-key-value-observing-in-swift


## Example

Create a helper class to help with observing key-value changes.

```rust
use core::ffi::c_void;
use core::ptr;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{define_class, msg_send, AnyThread, ClassType, DefinedClass};
use objc2_foundation::{
    ns_string, NSCopying, NSDictionary, NSKeyValueChangeKey, NSKeyValueObservingOptions, NSObject,
    NSObjectNSKeyValueObserverRegistration, NSObjectProtocol, NSString,
};

struct Ivars {
    object: Retained<NSObject>,
    key_path: Retained<NSString>,
    handler: Box<dyn Fn(&NSDictionary<NSKeyValueChangeKey, AnyObject>) + 'static>,
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - MyObserver implements `Drop` and ensures that:
    //   - It does not call an overridden method.
    //   - It does not `retain` itself.
    #[unsafe(super(NSObject))]
    #[ivars = Ivars]
    struct MyObserver;

    impl MyObserver {
        #[unsafe(method(observeValueForKeyPath:ofObject:change:context:))]
        fn observe_value(
            &self,
            _key_path: Option<&NSString>,
            _object: Option<&AnyObject>,
            change: Option<&NSDictionary<NSKeyValueChangeKey, AnyObject>>,
            _context: *mut c_void,
        ) {
            if let Some(change) = change {
                (self.ivars().handler)(change);
            } else {
                (self.ivars().handler)(&NSDictionary::new());
            }
        }
    }

    unsafe impl NSObjectProtocol for MyObserver {}
);

impl MyObserver {
    fn new(
        object: Retained<NSObject>,
        key_path: &NSString,
        options: NSKeyValueObservingOptions,
        // TODO: Thread safety? This probably depends on whether the observed
        // object is later moved to another thread.
        handler: impl Fn(&NSDictionary<NSKeyValueChangeKey, AnyObject>) + 'static + Send + Sync,
    ) -> Retained<Self> {
        let observer = Self::alloc().set_ivars(Ivars {
            object,
            key_path: key_path.copy(),
            handler: Box::new(handler),
        });
        let observer: Retained<Self> = unsafe { msg_send![super(observer), init] };

        // SAFETY: We make sure to un-register the observer before it's deallocated.
        //
        // Passing `NULL` as the `context` parameter here is fine, as the observer does not
        // have any subclasses, and the superclass (NSObject) is not observing anything.
        unsafe {
            observer
                .ivars()
                .object
                .addObserver_forKeyPath_options_context(
                    &observer,
                    key_path,
                    options,
                    ptr::null_mut(),
                );
        }

        observer
    }
}

impl Drop for MyObserver {
    fn drop(&mut self) {
        unsafe {
            self.ivars()
                .object
                .removeObserver_forKeyPath(&self, &self.ivars().key_path);
        }
    }
}

fn main() {
    let obj;
    # obj = NSObject::new();

    let _observer = MyObserver::new(
        obj,
        ns_string!("myKeyPath"),
        NSKeyValueObservingOptions::New | NSKeyValueObservingOptions::Old,
        |change| {
            println!("object changed: {:?}", change);
        },
    );

    // Do something that triggers the observer
}
```
