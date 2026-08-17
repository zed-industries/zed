#[cfg(feature = "NSString")]
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::rc::{Allocated, Retained};
use objc2::{extern_methods, ClassType};

use crate::Foundation::NSUUID;

impl UnwindSafe for NSUUID {}
impl RefUnwindSafe for NSUUID {}

/// The headers describe `initWithUUIDBytes:` and `getUUIDBytes:` as
/// taking `uuid_t`, but something fishy is going on, in reality they
/// expect a reference to these!
///
/// Hence we create this newtype to change the encoding.
#[repr(transparent)]
struct UuidBytes([u8; 16]);

unsafe impl RefEncode for UuidBytes {
    // Encoding depends on Foundation version, we hack it for now.
    const ENCODING_REF: Encoding = if cfg!(target_arch = "aarch64") {
        Encoding::String
    } else {
        Encoding::Array(16, &u8::ENCODING)
    };
}

extern_methods!(
    unsafe impl NSUUID {
        #[method_id(initWithUUIDBytes:)]
        fn initWithUUIDBytes(this: Allocated<Self>, bytes: &UuidBytes) -> Retained<Self>;

        #[method(getUUIDBytes:)]
        fn getUUIDBytes(&self, bytes: &mut UuidBytes);
    }
);

impl NSUUID {
    /// The 'nil UUID'.
    pub fn nil() -> Retained<Self> {
        Self::from_bytes([0; 16])
    }

    /// Create a new `NSUUID` from the given bytes.
    ///
    ///
    /// # Example
    ///
    /// Create a new `NSUUID` from the `uuid` crate.
    ///
    /// ```ignore
    /// use uuid::Uuid;
    /// use objc2_foundation::NSUUID;
    ///
    /// let uuid: Uuid;
    /// # uuid = todo!();
    /// let obj = NSUUID::from_bytes(uuid.into_bytes());
    /// assert_eq!(obj.as_bytes(), uuid.into_bytes());
    /// ```
    pub fn from_bytes(bytes: [u8; 16]) -> Retained<Self> {
        let bytes = UuidBytes(bytes);
        Self::initWithUUIDBytes(Self::alloc(), &bytes)
    }

    #[cfg(feature = "NSString")]
    pub fn from_string(string: &crate::NSString) -> Option<Retained<Self>> {
        Self::initWithUUIDString(Self::alloc(), string)
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        let mut bytes = UuidBytes([0; 16]);
        self.getUUIDBytes(&mut bytes);
        bytes.0
    }
}

#[cfg(feature = "NSString")]
impl fmt::Display for NSUUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.UUIDString(), f)
    }
}

#[cfg(feature = "NSString")]
impl fmt::Debug for NSUUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `uuid` crate does `Debug` and `Display` equally, and so do we
        fmt::Display::fmt(&self.UUIDString(), f)
    }
}

// UUID `compare:` is broken for some reason?

// impl PartialOrd for NSUUID {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for NSUUID {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         let res: NSComparisonResult = unsafe { msg_send![self, compare: other] };
//         res.into()
//     }
// }
