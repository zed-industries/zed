# Layered Safety

Objective-C is different from Rust<sup>[citation needed]</sup>. In particular,
Rust has a concept of "safety" (see the [nomicon] for details), which
Objective-C completely lacks.

You will find when using the framework crates that basically everything (that
has not been manually audited) is `unsafe`. So you might rightfully ask:
What's the point then? Can't I just use `msg_send!`, and save the extra
dependency?
Yes, you could, but in fact the framework crates are much safer than doing
method calling manually, even though you may end up writing `unsafe` just as
many times. I dub this "layered safety"<sup>1</sup> to capture the fact that
_not all usage of `unsafe` is created equally_!

Simply put, when using an `unsafe` method in e.g. `objc2-foundation`, you have
to ensure the compiler of much fewer things than when doing method calling
manually.
To see why this is the case, let me guide you through the various abstraction
layers that the framework crates and `objc2` provide, and we'll see how each
step makes things safer!

The framework crates are not perfect though, and there may be cases where you
have to drop down into lower-level details; luckily though, the fact that we
have this layered architecture with each step exposed along the way allows you
to do exactly that!

<sup>1: I haven't heard this concept named before, if you know of prior art on this please let me know.</sup>

[citation needed]: https://xkcd.com/285/
[nomicon]: https://doc.rust-lang.org/nomicon/intro.html


## Layer 1: `objc_msgSend`

Unlike C APIs where you define an `extern "C"` function that you want to call,
method calling is done in Objective-C using the "trampoline functions"
[`objc_msgSend`], `objc_msgSend_stret`, `objc_msgSend_fpret` and so on.
Which of these is correct depends on the target architecture and the calling
convention. Furthermore, to use these you first have to cast them to the
correct function signature using `mem::transmute`.

This is actually what's done [in the standard library][std-objc], since they
need to do it so rarely, and the extra dependency on a crate wouldn't be worth
the cost.

[`objc_msgSend`]: crate::ffi::objc_msgSend
[std-objc]: https://github.com/rust-lang/rust/blob/aa0189170057a6b56f445f05b9840caf6f260212/library/std/src/sys/unix/args.rs#L196-L248


### Example

Doing the Rust equivalent of Objective-C's `NSUInteger hash_code = [obj hash];`.

```rust, ignore
# // Fails with `unstable-c-unwind`, so disabled for now.
use std::mem::transmute;
use std::ffi::c_char;
use objc2::ffi::{objc_object, objc_msgSend, sel_registerName, NSUInteger, SEL};

let obj: *const objc_object;
# let obj = &*objc2::runtime::NSObject::new() as *const objc2::runtime::NSObject as *const _;
let sel = unsafe { sel_registerName(b"hash\0".as_ptr() as *const c_char) };
let msg_send_fn = unsafe {
    transmute::<
        unsafe extern "C" fn(),
        unsafe extern "C" fn(*const objc_object, SEL) -> NSUInteger,
    >(objc_msgSend)
};
let hash_code = unsafe { msg_send_fn(obj, sel) };
```


## Layer 2: `MessageReceiver`

We can improve on this using [`MessageReceiver::send_message`], which
abstracts away the calling convention details, as well as adding an `Encode`
bound on all the involved types. This ensures that we don't accidentally try
to pass e.g. a `Vec<T>`, which does not have a stable memory layout.

Additionally, when `debug_assertions` are enabled, the types involved in the
message send are compared to the types exposed in the Objective-C runtime.
This cannot catch mistakes like passing `null` where a non-null object was
expected, but it helps a lot with accidentally passing a `&c_int` where `int`
was expected.

[`MessageReceiver::send_message`]: crate::runtime::MessageReceiver::send_message


### Example

We'll reuse the `hash` example from above again.

```rust
use objc2::ffi::NSUInteger;
use objc2::runtime::{MessageReceiver, NSObject, Sel};

let obj: &NSObject;
# let obj = &*objc2::runtime::NSObject::new();
let sel = Sel::register("hash");
let hash_code: NSUInteger = unsafe {
    MessageReceiver::send_message(obj, sel, ())
};
```


## Layer 3a: `msg_send!`

Introducing macros: [`msg_send!`] can abstract away the tediousness of writing
the selector expression, as well as ensuring that the number of arguments to
the method is correct. It also handles details surrounding Objective-C's
`BOOL` type.

[`msg_send!`]: crate::msg_send


### Examples

The `hash` example again.

```rust
use objc2::ffi::NSUInteger;
use objc2::runtime::NSObject;
use objc2::msg_send;

let obj: &NSObject;
# let obj = &*objc2::runtime::NSObject::new();
let hash_code: NSUInteger = unsafe { msg_send![obj, hash] };
```

That example is now pretty close to as minimal as it gets, so let's introduce
something more complex; creating and using an instance of [`NSData`].

```rust
use objc2::ffi::NSUInteger;
use objc2::runtime::NSObject;
use objc2::{class, msg_send};

let obj: *const NSObject = unsafe { msg_send![class!(NSData), new] };
let length: NSUInteger = unsafe { msg_send![obj, length] };
// We have to specify the return type here, see layer 4 below
let _: () = unsafe { msg_send![obj, release] };
```

[`NSData`]: https://developer.apple.com/documentation/foundation/nsdata?language=objc


## Layer 3b: `msg_send_id!`

As you can see in the new example involving `NSData`, it can be quite tedious
to remember the `release` call when you're done with the object. Furthermore,
whether you need to `retain` and `release` the object involves subtle rules
that depend on the name of the method!

Objective-C solved this years ago with the introduction of "ARC". Similarly,
we can solve this with [`msg_send_id!`] and the smart pointer [`rc::Retained`],
which work together to ensure that the memory management of the object is done
correctly.

[`msg_send_id!`]: crate::msg_send_id
[`rc::Retained`]: crate::rc::Retained


### Example

The `NSData` example again.

```rust
use objc2::ffi::NSUInteger;
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{class, msg_send, msg_send_id};

let obj: Retained<NSObject> = unsafe { msg_send_id![class!(NSData), new] };
let length: NSUInteger = unsafe { msg_send![&obj, length] };
// `obj` goes out of scope, `release` is automatically sent to the object
```


## Layer 4: `extern_x` macros

There's still a problem with the above: we can't actually make a reusable
`hash` nor `length` function, since `NSObject` can refer to any object, and
all objects do not actually respond to that method.

To help with this, we have the [`extern_class!`] macro, which define a new
type resembling `NSObject`, but which represents the `NSData` class instead.

This allows us to make a completely safe API for downstream users!

Along with this, we can now use the [`extern_methods!`] macro to help with
defining our methods, which is also a big improvement over the `msg_send!` /
`msg_send_id!` macros, since it allows us to directly "see" the types, instead
of having them work by type-inference.

[`extern_class!`]: crate::extern_class
[`extern_methods!`]: crate::extern_methods


### Example

The `NSData` example again.

```rust
use objc2::ffi::NSUInteger;
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, mutability, ClassType};

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSData;

    unsafe impl ClassType for NSData {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    unsafe impl NSData {
        #[method_id(new)]
        pub fn new() -> Retained<Self>;

        #[method(length)]
        pub fn length(&self) -> NSUInteger;
    }
);

let obj = NSData::new();
let length = obj.length();
```


## Layer 5: Framework crates

Apple has a _lot_ of Objective-C code, and manually defining an interface to
all of it would take a lifetime. Especially keeping track of which methods are
nullable, and which are not, is difficult.

Instead, we can autogenerate the above definition from the headers directly
using type information exposed by `clang`, giving us a very high confidence
that it is correct!


### Example

The `NSData` example again.

```rust, ignore
use objc2_foundation::NSData;

let obj = NSData::new();
let length = obj.length();
```
