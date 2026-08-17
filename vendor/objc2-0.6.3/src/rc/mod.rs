//! # Reference counting utilities.
//!
//! The types in this module provide roughly the same benefits as ARC
//! (Automatic Reference Counting) does to Objective-C.
//!
//! Most importantly, a smart pointer [`Retained`] is provided to ensure that
//! objects are correctly retained and released when created and dropped,
//! respectively.
//!
//! Weak references may be created using the [`Weak`] struct; these will not
//! retain the object, but one can attempt to load them and obtain an `Retained`, or
//! safely fail if the object has been deallocated.
//!
//! See [the clang documentation][clang-arc] and [the Apple article on memory
//! management][mem-mgmt] (similar document exists [for Core Foundation][cf])
//! for more information on automatic and manual reference counting.
//!
//! It can also be useful to [enable Malloc Debugging][mem-debug] if you're trying
//! to figure out if/where your application has memory errors and leaks.
//!
//! [clang-arc]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html
//! [mem-mgmt]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/MemoryMgmt.html
//! [cf]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html
//! [mem-debug]: https://developer.apple.com/library/archive/documentation/Performance/Conceptual/ManagingMemory/Articles/MallocDebug.html
//!
//!
//! ## Example
//!
//! ```
//! use objc2::rc::{autoreleasepool, Retained, Weak};
//! use objc2::runtime::NSObject;
//!
//! // Allocate and initialize a new `NSObject`.
//! // `Retained` will release the object when dropped.
//! let obj: Retained<NSObject> = NSObject::new();
//!
//! // Cloning retains the object an additional time
//! let cloned = obj.clone();
//! autoreleasepool(|pool| {
//!     // Autorelease consumes the Retained, but won't actually
//!     // release it until the end of the autoreleasepool
//!     // SAFETY: The given is the innermost pool.
//!     let obj_ref: &NSObject = unsafe { Retained::autorelease(cloned, pool) };
//! });
//!
//! // Weak references won't retain the object
//! let weak = Weak::from_retained(&obj);
//! drop(obj);
//! assert!(weak.load().is_none());
//! ```

mod allocated_partial_init;
mod autorelease;
mod retained;
mod retained_forwarding_impls;
mod retained_traits;
#[cfg(test)]
mod test_object;
mod weak;

pub use self::allocated_partial_init::{Allocated, PartialInit};
pub use self::autorelease::{
    autoreleasepool, autoreleasepool_leaking, AutoreleasePool, AutoreleaseSafe,
};
// Re-export `Id` for backwards compatibility, but still mark it as deprecated.
#[allow(deprecated)]
pub use self::retained::Id;
pub use self::retained::Retained;
pub use self::retained_traits::{DefaultRetained, RetainedFromIterator, RetainedIntoIterator};
#[cfg(test)]
pub(crate) use self::test_object::{RcTestObject, ThreadTestData};
pub use self::weak::Weak;
// Same as above.
#[allow(deprecated)]
pub use self::weak::WeakId;
