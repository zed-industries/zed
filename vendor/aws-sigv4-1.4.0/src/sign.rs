/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Functions to create signing keys and calculate signatures.

// macro lifted from aws-smithy-runtime-api—eventually just inline these and delete macro.
macro_rules! builder_methods {
    ($fn_name:ident, $arg_name:ident, $ty:ty, $doc:literal, $($tail:tt)+) => {
        builder_methods!($fn_name, $arg_name, $ty, $doc);
        builder_methods!($($tail)+);
    };
    ($fn_name:ident, $arg_name:ident, $ty:ty, $doc:literal) => {
        #[doc = $doc]
        pub fn $fn_name(&mut self, $arg_name: Option<$ty>) -> &mut Self {
            self.$arg_name = $arg_name;
            self
        }

        #[doc = $doc]
        pub fn $arg_name(mut self, $arg_name: $ty) -> Self {
            self.$arg_name = Some($arg_name);
            self
        }
    };
}

/// Support for Sigv4 signing
pub mod v4;

/// Support for Sigv4a signing
#[cfg(feature = "sigv4a")]
pub mod v4a;
