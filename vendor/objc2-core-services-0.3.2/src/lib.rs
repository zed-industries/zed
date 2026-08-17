//! # Bindings to the `CoreServices` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/coreservices/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-core-services/0.2.2")]
// We ignore everything that depends on CarbonCore for now.
#![allow(unexpected_cfgs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "CarbonCore", feature = "Files"))]
mod files;
mod generated;

#[cfg(all(feature = "CarbonCore", feature = "Files"))]
pub use self::files::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

#[cfg(feature = "FSEvents")]
pub type ConstFSEventStreamRef = *const __FSEventStream;

#[allow(non_upper_case_globals)]
#[cfg(feature = "FSEvents")]
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kfseventstreameventidsincenow?language=objc)
pub const kFSEventStreamEventIdSinceNow: FSEventStreamEventId = 0xFFFFFFFFFFFFFFFF;

// MacTypes.h
#[allow(dead_code)]
mod mac_types {
    #[cfg(feature = "objc2")]
    use objc2::encode::{Encode, Encoding, RefEncode};

    pub(crate) type Fract = i32;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub(crate) struct Float80 {
        exp: i16,
        man: [i16; 4],
    }

    #[cfg(feature = "objc2")]
    unsafe impl Encode for Float80 {
        const ENCODING: Encoding =
            Encoding::Struct("Float80", &[<i16>::ENCODING, <[i16; 4]>::ENCODING]);
    }

    #[cfg(feature = "objc2")]
    unsafe impl RefEncode for Float80 {
        const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
    }

    pub(crate) type Ptr = *mut core::ffi::c_char;
    pub(crate) type Handle = *mut Ptr;
    pub(crate) type Size = core::ffi::c_long;

    pub(crate) type OSErr = i16;
    pub(crate) type OSStatus = i32;
    pub(crate) type LogicalAddress = *mut core::ffi::c_void;
    pub(crate) type ConstLogicalAddress = *const core::ffi::c_void;
    pub(crate) type PhysicalAddress = *mut core::ffi::c_void;
    pub(crate) type BytePtr = *mut u8;
    pub(crate) type ByteCount = core::ffi::c_ulong;
    pub(crate) type ByteOffset = core::ffi::c_ulong;
    pub(crate) type Duration = i32;
    pub(crate) type AbsoluteTime = i32;
    pub(crate) type OptionBits = u32;
    pub(crate) type ItemCount = core::ffi::c_ulong;
    pub(crate) type PBVersion = u32;
    pub(crate) type ScriptCode = i16;
    pub(crate) type LangCode = i16;
    pub(crate) type RegionCode = i16;
    pub(crate) type FourCharCode = u32;
    pub(crate) type OSType = FourCharCode;
    pub(crate) type ResType = FourCharCode;
    pub(crate) type OSTypePtr = *mut OSType;
    pub(crate) type ResTypePtr = *mut ResType;

    pub(crate) type Boolean = u8;

    #[cfg(target_pointer_width = "64")]
    pub(crate) type URefCon = *mut core::ffi::c_void;
    #[cfg(target_pointer_width = "64")]
    pub(crate) type SRefCon = *mut core::ffi::c_void;
    #[cfg(target_pointer_width = "32")]
    pub(crate) type URefCon = u32;
    #[cfg(target_pointer_width = "32")]
    pub(crate) type SRefCon = i32;

    #[allow(non_camel_case_types)]
    pub(crate) type extended80 = Float80;

    pub(crate) type UniChar = u16;
    pub(crate) type UniCharCount = core::ffi::c_ulong;
    pub(crate) type ConstStr255Param = *const core::ffi::c_char;
    pub(crate) type StringPtr = *mut core::ffi::c_char;
    pub(crate) type ConstStringPtr = *const core::ffi::c_char;
    pub(crate) type Str255 = [core::ffi::c_uchar; 255];
    pub(crate) type Str63 = [core::ffi::c_uchar; 64];
    pub(crate) type Str32 = [core::ffi::c_uchar; 33];
    pub(crate) type Str31 = [core::ffi::c_uchar; 32];
    pub(crate) type Str27 = [core::ffi::c_uchar; 28];
    pub(crate) type Str15 = [core::ffi::c_uchar; 16];
    pub(crate) type Str32Field = [core::ffi::c_uchar; 34];

    pub(crate) type SignedByte = i8;
}

#[allow(unused_imports)]
pub(crate) use self::mac_types::*;
