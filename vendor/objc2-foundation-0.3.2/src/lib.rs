//! # Bindings to the `Foundation` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/foundation/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//! This is the [`std`] equivalent for Objective-C, containing essential data
//! types, collections, and operating-system services.
//!
//!
//! ## Rust vs. Objective-C types
//!
//! A quick overview of some types you will encounter often in Objective-C,
//! and their approximate Rust equivalent.
//!
//! | Objective-C                  | (approximately) equivalent Rust               |
//! | ---------------------------- | --------------------------------------------- |
//! | `NSData*`                    | `Rc<[u8]>`                                    |
//! | `NSMutableData*`             | `Rc<Cell<Vec<u8>>>`                           |
//! | `NSString*`                  | `Rc<str>`                                     |
//! | `NSMutableString*`           | `Rc<Cell<String>>`                            |
//! | `NSValue*`                   | `Rc<dyn Any>`                                 |
//! | `NSNumber*`                  | `Arc<enum { I8(i8), U8(u8), I16(i16), ... }>` |
//! | `NSError*`                   | `Arc<dyn Error + Send + Sync>`                |
//! | `NSException*`               | `Arc<dyn Error + Send + Sync>`                |
//! | `NSRange`                    | `ops::Range<usize>`                           |
//! | `NSComparisonResult`         | `cmp::Ordering`                               |
//! | `NSEnumerator<T>*`           | `Rc<dyn Iterator<Item = Retained<T>>>`        |
//! | `NSCopying*`                 | `Rc<dyn Clone>`                               |
//! | `NSArray<T>*`                | `Rc<[Retained<T>]>`                           |
//! | `NSMutableArray<T>*`         | `Rc<Cell<Vec<Retained<T>>>>`                  |
//! | `NSDictionary<K, V>*`        | `Rc<HashMap<Retained<K>, Retained<V>>>`       |
//! | `NSMutableDictionary<K, V>*` | `Rc<Cell<HashMap<Retained<K>, Retained<V>>>>` |
//!
//! Note, in particular, that all "Mutable" variants use interior mutability,
//! and that some things are thread-safe (`Arc`), while others are not (`Rc`).
//!
//!
//! ## Examples
//!
//! Basic usage of a few Foundation types.
//!
//! ```console
//! $ cargo add objc2-foundation
//! ```
//!
#![cfg_attr(
    all(
        feature = "NSObject",
        feature = "NSString",
        feature = "NSArray",
        feature = "NSDictionary",
        feature = "NSEnumerator"
    ),
    doc = "```"
)]
#![cfg_attr(
    not(all(
        feature = "NSObject",
        feature = "NSString",
        feature = "NSArray",
        feature = "NSDictionary",
        feature = "NSEnumerator"
    )),
    doc = "```ignore"
)]
//! use objc2_foundation::{ns_string, NSArray, NSDictionary, NSObject};
//!
//! // Create and compare NSObjects
//! let obj = NSObject::new();
//! #[allow(clippy::eq_op)]
//! {
//!     println!("{obj:?} == {obj:?}? {:?}", obj == obj);
//! }
//!
//! let obj2 = NSObject::new();
//! println!("{obj:?} == {obj2:?}? {:?}", obj == obj2);
//!
//! // Create an NSArray from a Vec
//! let objs = vec![obj, obj2];
//! let array = NSArray::from_retained_slice(&objs);
//! for obj in array.iter() {
//!     println!("{obj:?}");
//! }
//! println!("{}", array.len());
//!
//! // Turn the NSArray back into a Vec
//! let mut objs = array.to_vec();
//! let obj = objs.pop().unwrap();
//!
//! // Create a static NSString
//! let string = ns_string!("Hello, world!");
//! // And use the `ToString` implementation to convert it into a string
//! let _s = string.to_string();
//! // Or use the `Display` implementation directly
//! println!("{string}");
//!
//! // Create a dictionary mapping strings to objects
//! let keys = &[string];
//! let objects = &[&*obj];
//! let dict = NSDictionary::from_slices(keys, objects);
//! println!("{:?}", dict.objectForKey(string));
//! println!("{}", dict.len());
//! ```
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-foundation/0.3.2")]
#![allow(non_snake_case)]
#![recursion_limit = "512"]

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature currently must be enabled.");

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub mod __ns_macro_helpers;
#[cfg(feature = "NSEnumerator")]
#[macro_use]
mod iter;
#[cfg(feature = "NSArray")]
pub mod array;
#[cfg(feature = "NSAttributedString")]
mod attributed_string;
#[cfg(feature = "NSBundle")]
mod bundle;
#[cfg(feature = "NSObjCRuntime")]
mod comparison_result;
#[cfg(feature = "NSObject")]
mod copying;
#[cfg(feature = "NSData")]
mod data;
#[cfg(feature = "NSDecimal")]
mod decimal;
#[cfg(feature = "NSDictionary")]
pub mod dictionary;
#[cfg(feature = "NSEnumerator")]
pub mod enumerator;
#[cfg(feature = "NSError")]
mod error;
#[cfg(feature = "NSException")]
mod exception;
#[cfg(feature = "NSEnumerator")]
mod fast_enumeration_state;
mod generated;
#[cfg(feature = "NSGeometry")]
mod geometry;
mod macros;
mod ns_consumed;
#[cfg(feature = "NSValue")]
mod number;
#[cfg(feature = "NSProcessInfo")]
mod process_info;
#[cfg(feature = "NSRange")]
mod range;
#[cfg(feature = "NSSet")]
pub mod set;
#[cfg(feature = "NSString")]
mod string;
#[cfg(test)]
mod tests;
#[cfg(feature = "NSThread")]
mod thread;
#[cfg(feature = "NSObject")]
mod to_owned;
#[cfg(feature = "NSURL")]
mod url;
#[cfg(feature = "NSUserDefaults")]
mod user_defaults;
mod util;
#[cfg(feature = "NSUUID")]
mod uuid;
#[cfg(feature = "NSValue")]
mod value;

#[cfg(feature = "NSObjCRuntime")]
pub use self::comparison_result::NSComparisonResult;
#[cfg(feature = "NSObject")]
pub use self::copying::{CopyingHelper, MutableCopyingHelper, NSCopying, NSMutableCopying};
#[cfg(feature = "NSDecimal")]
pub use self::decimal::NSDecimal;
#[cfg(feature = "NSEnumerator")]
pub use self::fast_enumeration_state::NSFastEnumerationState;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "NSGeometry")]
pub use self::geometry::NSRectEdge;
#[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
pub use self::geometry::{NSPoint, NSRect, NSSize};
#[cfg(feature = "NSMapTable")]
pub use self::ns_consumed::NSFreeMapTable;
#[cfg(feature = "NSRange")]
pub use self::range::NSRange;
#[cfg(feature = "NSThread")]
pub use self::thread::*;

// Available under Foundation, so makes sense here as well:
// https://developer.apple.com/documentation/foundation/numbers_data_and_basic_values?language=objc
pub use objc2::ffi::{NSInteger, NSUInteger};

// Special types that are stored in `objc2`, but really belong here
#[doc(inline)]
#[cfg(feature = "NSZone")]
pub use objc2::runtime::NSZone;
#[doc(inline)]
#[cfg(feature = "NSProxy")]
pub use objc2::runtime::__NSProxy as NSProxy;
pub use objc2::runtime::{NSObject, NSObjectProtocol};
#[deprecated = "Moved to `objc2::MainThreadMarker`"]
pub use objc2::MainThreadMarker;

#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-base", kind = "dylib"))]
extern "C" {}

// MacTypes.h
#[allow(unused)]
pub(crate) type Boolean = u8; // unsigned char
#[allow(unused)]
pub(crate) type FourCharCode = u32;
#[allow(unused)]
pub(crate) type OSType = FourCharCode;
#[allow(unused)]
pub(crate) type UTF32Char = u32; // Or maybe Rust's char?
#[allow(unused)]
#[cfg(target_pointer_width = "64")]
pub(crate) type SRefCon = *mut core::ffi::c_void;
#[allow(unused)]
#[cfg(target_pointer_width = "32")]
pub(crate) type SRefCon = i32;
#[allow(unused)]
pub(crate) type OSErr = i16;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nstimeintervalsince1970?language=objc)
#[allow(non_upper_case_globals)]
#[cfg(feature = "NSDate")]
pub const NSTimeIntervalSince1970: crate::NSTimeInterval = 978307200.0;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsurlresponseunknownlength?language=objc)
#[allow(non_upper_case_globals)]
#[cfg(feature = "NSURLResponse")]
pub const NSURLResponseUnknownLength: core::ffi::c_longlong = -1;
