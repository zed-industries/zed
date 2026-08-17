// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation UUID objects.

#[cfg(feature = "with-uuid")]
extern crate uuid;

use core_foundation_sys::base::kCFAllocatorDefault;
pub use core_foundation_sys::uuid::*;

use crate::base::TCFType;

#[cfg(feature = "with-uuid")]
use self::uuid::Uuid;

declare_TCFType! {
    /// A UUID.
    CFUUID, CFUUIDRef
}
impl_TCFType!(CFUUID, CFUUIDRef, CFUUIDGetTypeID);
impl_CFTypeDescription!(CFUUID);

impl CFUUID {
    #[inline]
    pub fn new() -> CFUUID {
        unsafe {
            let uuid_ref = CFUUIDCreate(kCFAllocatorDefault);
            TCFType::wrap_under_create_rule(uuid_ref)
        }
    }
}

impl Default for CFUUID {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "with-uuid")]
impl From<CFUUID> for Uuid {
    fn from(val: CFUUID) -> Self {
        let b = unsafe { CFUUIDGetUUIDBytes(val.0) };
        let bytes = [
            b.byte0, b.byte1, b.byte2, b.byte3, b.byte4, b.byte5, b.byte6, b.byte7, b.byte8,
            b.byte9, b.byte10, b.byte11, b.byte12, b.byte13, b.byte14, b.byte15,
        ];
        Uuid::from_bytes(&bytes).unwrap()
    }
}

#[cfg(feature = "with-uuid")]
impl From<Uuid> for CFUUID {
    fn from(uuid: Uuid) -> CFUUID {
        let b = uuid.as_bytes();
        let bytes = CFUUIDBytes {
            byte0: b[0],
            byte1: b[1],
            byte2: b[2],
            byte3: b[3],
            byte4: b[4],
            byte5: b[5],
            byte6: b[6],
            byte7: b[7],
            byte8: b[8],
            byte9: b[9],
            byte10: b[10],
            byte11: b[11],
            byte12: b[12],
            byte13: b[13],
            byte14: b[14],
            byte15: b[15],
        };
        unsafe {
            let uuid_ref = CFUUIDCreateFromUUIDBytes(kCFAllocatorDefault, bytes);
            TCFType::wrap_under_create_rule(uuid_ref)
        }
    }
}

#[cfg(test)]
#[cfg(feature = "with-uuid")]
mod test {
    use super::CFUUID;
    use uuid::Uuid;

    #[test]
    fn uuid_conversion() {
        let cf_uuid = CFUUID::new();
        let uuid: Uuid = cf_uuid.clone().into();
        let converted = CFUUID::from(uuid);
        assert_eq!(cf_uuid, converted);
    }
}
