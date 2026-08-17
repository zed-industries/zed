//! # Bindings to the `IOKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/iokit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-io-kit/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "libc")]
mod consumes_argument;
mod generated;
mod macros;

#[cfg(feature = "libc")]
#[allow(unused_imports, unreachable_pub)]
pub use self::consumes_argument::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::macros::*;

// IOKit/IOReturn.h
/// [Apple's documentation](https://developer.apple.com/documentation/iokit/ioreturn?language=objc)
pub type IOReturn = core::ffi::c_int; // kern_return_t

/// [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnsuccess?language=objc)
#[allow(non_upper_case_globals)]
pub const kIOReturnSuccess: IOReturn = 0;

// IOKit/IOPM.h
/// [Apple's documentation](https://developer.apple.com/documentation/kernel/2876248-anonymous/kiopsfamilycodeunsupported/)
#[allow(non_upper_case_globals)]
pub const kIOPSFamilyCodeUnsupported: core::ffi::c_int = kIOReturnUnsupported as core::ffi::c_int;

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;
#[allow(dead_code)]
pub(crate) type AbsoluteTime = i32;
#[allow(dead_code)]
pub(crate) type NumVersion = u32; // Actually a struct with 4 u8s
#[allow(dead_code)]
pub(crate) type FourCharCode = u32;

// device/device_types.h
#[allow(dead_code, non_camel_case_types)]
pub(crate) type io_name_t = *mut [core::ffi::c_char; 128];
#[allow(dead_code, non_camel_case_types)]
pub(crate) type io_string_t = *mut [core::ffi::c_char; 512];
#[allow(dead_code, non_camel_case_types)]
pub(crate) type io_struct_inband_t = *mut [core::ffi::c_char; 4096];

// uuid/uuid_t.h
#[allow(dead_code, non_camel_case_types)]
pub(crate) type uuid_t = [u8; 16]; // Usage sites are all in structs

/// [Apple's documentation](https://developer.apple.com/documentation/iokit/io_object_null?language=objc)
#[cfg(feature = "libc")]
pub const IO_OBJECT_NULL: io_object_t = 0;

// mach/mach_types.h
#[allow(dead_code, non_camel_case_types)]
#[cfg(feature = "libc")]
pub(crate) type task_port_t = libc::task_t;
