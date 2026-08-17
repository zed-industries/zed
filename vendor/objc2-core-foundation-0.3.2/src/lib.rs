//! # Bindings to the `CoreFoundation` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/corefoundation/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(feature = "unstable-coerce-pointee", feature(derive_coerce_pointee))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-core-foundation/0.3.2")]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub mod __cf_macro_helpers;
#[cfg(feature = "CFArray")]
mod array;
mod base;
#[cfg(feature = "CFBundle")]
mod bundle;
mod cf_type;
#[cfg(feature = "CFData")]
mod data;
#[cfg(feature = "CFDate")]
mod date;
#[cfg(feature = "CFDictionary")]
mod dictionary;
#[cfg(feature = "CFError")]
mod error;
#[cfg(feature = "CFFileDescriptor")]
mod filedescriptor;
// Allow `default` methods on CFAllocator and CFTimeZone
#[allow(clippy::should_implement_trait)]
mod generated;
#[cfg(feature = "CFCGTypes")]
mod geometry;
#[cfg(feature = "CFNumber")]
mod number;
mod opaque;
mod retained;
#[cfg(feature = "CFString")]
mod string;
mod thread_safety;
#[cfg(feature = "CFTimeZone")]
mod timezone;
mod type_traits;
#[cfg(feature = "CFURL")]
mod url;
#[cfg(feature = "CFUserNotification")]
mod user_notification;
#[cfg(feature = "CFUUID")]
mod uuid;

#[cfg(feature = "CFArray")]
pub use self::array::*;
pub use self::base::*;
#[cfg(feature = "CFBundle")]
pub use self::bundle::CFBundleRefNum;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "CFCGTypes")]
pub use self::geometry::*;
pub use self::retained::CFRetained;
pub use self::type_traits::{ConcreteType, Type};

// This is not exposed publicly, so the only way to use this in types with
// generics is to use it through the default type (e.g. the user should write
// `CFArray` instead of `CFArray<Opaque>`).
#[allow(unused_imports)]
pub(crate) use self::opaque::Opaque;

// MacTypes.h
#[allow(dead_code)]
mod mac_types {
    pub(crate) type Boolean = u8; // unsigned char
    pub(crate) type ConstStr255Param = *const core::ffi::c_char;
    pub(crate) type ConstStringPtr = *const core::ffi::c_char;
    pub(crate) type FourCharCode = u32;
    pub(crate) type LangCode = i16;
    pub(crate) type OSType = FourCharCode;
    pub(crate) type RegionCode = i16;
    pub(crate) type ResType = FourCharCode;
    pub(crate) type StringPtr = *mut core::ffi::c_char;
    pub(crate) type UniChar = u16;
    pub(crate) type UTF32Char = u32; // Or maybe Rust's char?
}

#[allow(unused_imports)]
pub(crate) use self::mac_types::*;
