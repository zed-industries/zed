use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::rc::{Allocated, Retained};
use objc2::runtime::NSObject;
use objc2::{extern_methods, msg_send, AnyThread};

use crate::{util, NSUUID};

impl UnwindSafe for NSUUID {}
impl RefUnwindSafe for NSUUID {}

#[repr(transparent)]
struct UuidBytes([u8; 16]);

unsafe impl RefEncode for UuidBytes {
    // Encoding actually depends on the instance class, `__NSConcreteUUID` has
    // a different method encoding than `NSUUID`, the former takes a `char*`.
    //
    // This may also depend on the Foundation / OS version.
    const ENCODING_REF: Encoding = Encoding::Array(16, &u8::ENCODING);
}

impl NSUUID {
    extern_methods!(
        #[unsafe(method(initWithUUIDBytes:))]
        fn initWithUUIDBytes(this: Allocated<Self>, bytes: &UuidBytes) -> Retained<Self>;

        #[unsafe(method(getUUIDBytes:))]
        fn getUUIDBytes(&self, bytes: &mut UuidBytes);
    );
}

impl NSUUID {
    /// The 'nil UUID'.
    pub fn nil() -> Retained<Self> {
        Self::from_bytes([0; 16])
    }

    /// Create a new `NSUUID` from the given bytes.
    ///
    /// NOTE: The headers describe `initWithUUIDBytes:` as taking `uuid_t`,
    /// but their actual implementation may use something else.
    ///
    /// Thus, to use this method, you must currently disable encoding
    /// verification using the `"disable-encoding-assertions"` Cargo feature
    /// in `objc2`.
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

    /// Convert the UUID to an array.
    ///
    /// NOTE: The headers describe `getUUIDBytes:` as taking `uuid_t`, but
    /// their actual implementation may use something else.
    ///
    /// Thus, to use this method, you must currently disable encoding
    /// verification using the `"disable-encoding-assertions"` Cargo feature
    /// in `objc2`.
    pub fn as_bytes(&self) -> [u8; 16] {
        let mut bytes = UuidBytes([0; 16]);
        self.getUUIDBytes(&mut bytes);
        bytes.0
    }
}

impl fmt::Display for NSUUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: Retained<NSObject> = unsafe { msg_send![self, UUIDString] };
        // SAFETY: `UUIDString` returns `NSString`.
        unsafe { util::display_string(&string, f) }
    }
}

impl fmt::Debug for NSUUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `uuid` crate does `Debug` and `Display` equally, and so do we.

        let string: Retained<NSObject> = unsafe { msg_send![self, UUIDString] };
        // SAFETY: `UUIDString` returns `NSString`.
        unsafe { util::display_string(&string, f) }
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
