Objective-C Runtime bindings and wrapper for Rust.

* Documentation: http://ssheldon.github.io/rust-objc/objc/
* Crate: https://crates.io/crates/objc

## Messaging objects

Objective-C objects can be messaged using the `msg_send!` macro:

``` rust
let cls = class!(NSObject);
let obj: *mut Object = msg_send![cls, new];
let hash: usize = msg_send![obj, hash];
let is_kind: BOOL = msg_send![obj, isKindOfClass:cls];
// Even void methods must have their return type annotated
let _: () = msg_send![obj, release];
```

## Reference counting

The utilities of the `rc` module provide ARC-like semantics for working with
Objective-C's reference counted objects in Rust.
A `StrongPtr` retains an object and releases the object when dropped.
A `WeakPtr` will not retain the object, but can be upgraded to a `StrongPtr`
and safely fails if the object has been deallocated.

``` rust
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
```

## Declaring classes

Classes can be declared using the `ClassDecl` struct. Instance variables and
methods can then be added before the class is ultimately registered.

The following example demonstrates declaring a class named `MyNumber` that has
one ivar, a `u32` named `_number` and a `number` method that returns it:

``` rust
let superclass = class!(NSObject);
let mut decl = ClassDecl::new("MyNumber", superclass).unwrap();

// Add an instance variable
decl.add_ivar::<u32>("_number");

// Add an ObjC method for getting the number
extern fn my_number_get(this: &Object, _cmd: Sel) -> u32 {
    unsafe { *this.get_ivar("_number") }
}
unsafe {
    decl.add_method(sel!(number),
        my_number_get as extern fn(&Object, Sel) -> u32);
}

decl.register();
```

## Exceptions

By default, if the `msg_send!` macro causes an exception to be thrown, this
will unwind into Rust resulting in unsafe, undefined behavior.
However, this crate has an `"exception"` feature which, when enabled, wraps
each `msg_send!` in a `@try`/`@catch` and panics if an exception is caught,
preventing Objective-C from unwinding into Rust.

## Message type verification

The Objective-C runtime includes encodings for each method that describe the
argument and return types. This crate can take advantage of these encodings to
verify that the types used in Rust match the types encoded for the method.

To use this functionality, enable the `"verify_message"` feature.
With this feature enabled, type checking is performed for every message send,
which also requires that all arguments and return values for all messages
implement `Encode`.

If this requirement is burdensome or you'd rather just verify specific messages,
you can call the `Message::verify_message` method for specific selectors.

## Support for other Operating Systems

The bindings can be used on Linux or *BSD utilizing the
[GNUstep Objective-C runtime](https://www.github.com/gnustep/libobjc2).
