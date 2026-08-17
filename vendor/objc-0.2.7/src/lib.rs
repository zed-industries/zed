/*!
Objective-C Runtime bindings and wrapper for Rust.

# Messaging objects

Objective-C objects can be messaged using the [`msg_send!`](macro.msg_send!.html) macro:

``` no_run
# #[macro_use] extern crate objc;
# use objc::runtime::{BOOL, Class, Object};
# fn main() {
# unsafe {
let cls = class!(NSObject);
let obj: *mut Object = msg_send![cls, new];
let hash: usize = msg_send![obj, hash];
let is_kind: BOOL = msg_send![obj, isKindOfClass:cls];
// Even void methods must have their return type annotated
let _: () = msg_send![obj, release];
# }
# }
```

# Reference counting

Utilities for reference counting Objective-C objects are provided in the
[`rc`](rc/index.html) module.

# Declaring classes

Objective-C classes can even be declared from Rust using the functionality of
the [`declare`](declare/index.html) module.

# Exceptions

By default, if the `msg_send!` macro causes an exception to be thrown, this
will unwind into Rust resulting in unsafe, undefined behavior.
However, this crate has an `"exception"` feature which, when enabled, wraps
each `msg_send!` in a `@try`/`@catch` and panics if an exception is caught,
preventing Objective-C from unwinding into Rust.

# Message type verification

The Objective-C runtime includes encodings for each method that describe the
argument and return types. This crate can take advantage of these encodings to
verify that the types used in Rust match the types encoded for the method.

To use this functionality, enable the `"verify_message"` feature.
With this feature enabled, type checking is performed for every message send,
which also requires that all arguments and return values for all messages
implement `Encode`.

If this requirement is burdensome or you'd rather
just verify specific messages, you can call the
[`Message::verify_message`](trait.Message.html#method.verify_message) method
for specific selectors.

# Support for other Operating Systems

The bindings can be used on Linux or *BSD utilizing the
[GNUstep Objective-C runtime](https://www.github.com/gnustep/libobjc2).
*/

#![crate_name = "objc"]
#![crate_type = "lib"]

#![warn(missing_docs)]

extern crate malloc_buf;
#[cfg(feature = "exception")]
extern crate objc_exception;

pub use encode::{Encode, EncodeArguments, Encoding};
pub use message::{Message, MessageArguments, MessageError};

pub use message::send_message as __send_message;
pub use message::send_super_message as __send_super_message;

#[macro_use]
mod macros;

pub mod runtime;
pub mod declare;
pub mod rc;
mod encode;
#[cfg(feature = "exception")]
mod exception;
mod message;

#[cfg(test)]
mod test_utils;
