# Interop with `core-foundation`-like crates

The `objc2` project does not [yet](https://github.com/madsmtm/objc2/issues/556) provide bindings to CoreFoundation and similar frameworks.

To interact with these, you will have to use existing crates like [`core-foundation`], [`core-graphics`], [`security-framework`], [`system-configuration`] and so on.

This can, however, pose a bit of an issue, since `objc2` and [`block2`] impose certain requirements on the types involved.

[`core-foundation`]: https://crates.io/crates/core-foundation
[`core-graphics`]: https://crates.io/crates/core-graphics
[`security-framework`]: https://crates.io/crates/security-framework
[`system-configuration`]: https://crates.io/crates/system-configuration
[`block2`]: https://docs.rs/block2/latest/block2/


## Implementing `Encode` for a newtype wrapper

Due to Rust's orphan rules, `core-foundation` types do not implement `Encode`, and as such cannot be passed to/from methods and in blocks by default.

We're (slowly) working on fixing this, see [servo/core-foundation-rs#628], but in the meantime, you can use a newtype wrapper around the `CF...Ref`, and implement [`Encode`] for that wrapper.

[servo/core-foundation-rs#628]: https://github.com/servo/core-foundation-rs/pull/628
[`Encode`]: crate::encode::Encode


### Examples

Declaring an external function to [`CFRunLoopObserverCreateWithHandler`](https://developer.apple.com/documentation/corefoundation/1542816-cfrunloopobservercreatewithhandl?language=objc), which uses blocks and `block2`, and as such require its parameters to implement `Encode`.

```rust
use std::ptr;

use block2::{RcBlock, Block};
use core_foundation::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, TCFType};
use core_foundation::runloop::{CFRunLoopActivity, CFRunLoopObserver, CFRunLoopObserverRef, kCFRunLoopAllActivities};
use objc2::encode::{Encode, Encoding};

#[repr(transparent)]
struct CFRunLoopObserverRefWrapper(CFRunLoopObserverRef);

// SAFETY: `CFRunLoopObserverRefWrapper` is `#[repr(transparent)]` over
// `CFRunLoopObserverRef`, which is a typedef to `struct __CFRunLoopObserver *`.
unsafe impl Encode for CFRunLoopObserverRefWrapper {
    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Struct("__CFRunLoopObserver", &[]));
}

extern "C" {
    fn CFRunLoopObserverCreateWithHandler(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: Boolean,
        order: CFIndex,
        block: &Block<dyn Fn(CFRunLoopObserverRefWrapper, CFRunLoopActivity)>
    ) -> CFRunLoopObserverRef;
}

let block = RcBlock::new(|observer: CFRunLoopObserverRefWrapper, activity| {
    // Extract `CFRunLoopObserverRef` from `CFRunLoopObserverRefWrapper`
    let observer = observer.0;
});

let observer = unsafe {
    CFRunLoopObserver::wrap_under_create_rule(CFRunLoopObserverCreateWithHandler(
        ptr::null(),
        kCFRunLoopAllActivities,
        false as Boolean,
        0,
        &block,
    ))
};
#
# assert!(CFRunLoopObserverRefWrapper::ENCODING.equivalent_to_str("^{__CFRunLoopObserver=}"));
```

An alternative, if you don't want to go through the trouble of creating a newtype, is to use the `"relax-void-encoding"` Cargo feature.

Here we set the [`-[CALayer borderColor]`](https://developer.apple.com/documentation/quartzcore/calayer/1410903-bordercolor?language=objc) property (which uses `CGColorRef`).

```rust, ignore
use core_foundation::base::ToVoid;
use core_graphics::color::CGColor;
use objc2_quartz_core::CALayer;
use objc2::msg_send;

fn set_border_color(layer: &CALayer, color: &CGColor) {
    let color = color.to_void();
    // Passing `*const c_void` here requires the "relax-void-encoding" feature
    unsafe { msg_send![layer, setBorderColor: color] }
}

let layer = unsafe { CALayer::new() };
let color = CGColor::rgb(1.0, 0.0, 0.0, 1.0);
set_border_color(&layer, &color);
```


## Toll-free bridging

Certain CoreFoundation types are documented to be ["toll-free bridged"], which means that they're completely interoperable with the Foundation types. To convert between these in Rust, you'll have to cast the pointers, e.g. between `CFStringRef` and `*const NSString`.

["toll-free bridged"]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFDesignConcepts/Articles/tollFreeBridgedTypes.html


### Example

Toll-free bridging between `CFString` and `NSString`.

```rust
use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use objc2_foundation::{NSString, ns_string};
use objc2::rc::Retained;

fn cf_string_to_ns(s: &CFString) -> &NSString {
    let ptr: CFStringRef = s.as_concrete_TypeRef();
    let ptr: *const NSString = ptr.cast();
    // SAFETY: CFString is toll-free bridged with NSString.
    unsafe { ptr.as_ref().unwrap() }
}

// Note: `NSString` is currently a bit special, and requires that we convert from
// `Retained<NSString>`, as it could otherwise have come from `&NSMutableString`,
// and then we'd loose lifetime information by converting to `CFString`.
//
// This will be changed in the future, see https://github.com/madsmtm/objc2/issues/563.
fn ns_string_to_cf(s: Retained<NSString>) -> CFString {
    // Yield ownership over the string
    let ptr: *const NSString = Retained::into_raw(s);
    let ptr: CFStringRef = ptr.cast();
    // SAFETY: NSString is toll-free bridged with CFString, and
    // ownership was passed above with `Retained::into_raw`.
    unsafe { CFString::wrap_under_create_rule(ptr) }
}

let cf = CFString::new("foo");
let ns = NSString::from_str("foo");
assert_eq!(cf_string_to_ns(&cf), &*ns);
assert_eq!(cf, ns_string_to_cf(ns));
```
