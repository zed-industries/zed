// Copyright 2018 Trent Clarke.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Generates an implementation of the Debug trait for a type that defers to the
// Debug implementation for a given field.

#![allow(missing_docs)]

macro_rules! derive_debug_via_id {
    ($typename:ident) => {
        impl ::core::fmt::Debug for $typename {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error> {
                ::core::fmt::Debug::fmt(&self.id, f)
            }
        }
    };
}
pub(crate) use derive_debug_via_id;

#[allow(unused_macros)]
macro_rules! derive_debug_via_field {
    ($type:ty, $field:ident) => {
        derive_debug_via_field!($type, stringify!($type), $field);
    };

    ($type:ty, $typename:expr, $field:ident) => {
        impl ::core::fmt::Debug for $type {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error> {
                f.debug_struct($typename)
                    .field(stringify!($field), &self.$field)
                    .finish()
            }
        }
    };
}

// Generates an implementation of the Debug trait for a type that outputs the
// hex encoding of the byte slice representation of the value.
#[allow(unused_macros)]
macro_rules! derive_debug_self_as_ref_hex_bytes {
    ($typename:ident) => {
        impl ::core::fmt::Debug for $typename {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error> {
                crate::debug::write_hex_tuple(f, stringify!($typename), self)
            }
        }
    };
}

pub(crate) fn write_hex_bytes(
    fmt: &mut core::fmt::Formatter,
    bytes: &[u8],
) -> Result<(), core::fmt::Error> {
    for byte in bytes {
        write!(fmt, "{byte:02x}")?;
    }
    Ok(())
}
