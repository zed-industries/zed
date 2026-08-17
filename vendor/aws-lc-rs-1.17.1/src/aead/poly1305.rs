// Copyright 2015-2016 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// TODO: enforce maximum input length.

use super::{Tag, TAG_LEN};
use crate::aws_lc::{CRYPTO_poly1305_finish, CRYPTO_poly1305_init, CRYPTO_poly1305_update};
use crate::cipher::block::BLOCK_LEN;
use core::mem::MaybeUninit;

/// A Poly1305 key.
pub(super) struct Key {
    pub(super) key_and_nonce: [u8; KEY_LEN],
}

const KEY_LEN: usize = 2 * BLOCK_LEN;

impl Key {
    #[inline]
    #[allow(dead_code)]
    pub(super) fn new(key_and_nonce: [u8; KEY_LEN]) -> Self {
        Self { key_and_nonce }
    }
}

pub struct Context {
    state: poly1305_state,
}

// Keep in sync with `poly1305_state` in GFp/poly1305.h.
//
// The C code, in particular the way the `poly1305_aligned_state` functions
// are used, is only correct when the state buffer is 64-byte aligned.
#[repr(C, align(64))]
#[allow(non_camel_case_types)]
struct poly1305_state(aws_lc::poly1305_state);

impl Context {
    #[inline]
    pub(super) fn from_key(Key { key_and_nonce }: Key) -> Self {
        unsafe {
            // Use `zeroed()` instead of `uninit()` for MSAN compatibility.
            // `CRYPTO_poly1305_init` may not write all 512 bytes of the opaque
            // `poly1305_state` buffer, so MSAN would flag `assume_init()` as
            // reading uninitialized memory. Zeroing is safe since the underlying
            // type is `[u8; 512]`.
            let mut state = MaybeUninit::<poly1305_state>::zeroed();
            CRYPTO_poly1305_init(state.as_mut_ptr().cast(), key_and_nonce.as_ptr());
            Self {
                state: state.assume_init(),
            }
        }
    }

    #[inline]
    pub fn update(&mut self, input: &[u8]) {
        unsafe {
            CRYPTO_poly1305_update(
                self.state.0.as_mut_ptr().cast(),
                input.as_ptr(),
                input.len(),
            );
        }
    }

    #[inline]
    pub(super) fn finish(mut self) -> Tag {
        unsafe {
            let mut tag = MaybeUninit::<[u8; TAG_LEN]>::uninit();
            CRYPTO_poly1305_finish(self.state.0.as_mut_ptr().cast(), tag.as_mut_ptr().cast());
            crate::fips::set_fips_service_status_unapproved();
            Tag(tag.assume_init(), TAG_LEN)
        }
    }
}

/// Implements the original, non-IETF padding semantics.
///
/// This is used by `chacha20_poly1305_openssh` and the standalone
/// poly1305 test vectors.
#[inline]
pub(super) fn sign(key: Key, input: &[u8]) -> Tag {
    let mut ctx = Context::from_key(key);
    ctx.update(input);
    ctx.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test, test_file};

    // Adapted from BoringSSL's crypto/poly1305/poly1305_test.cc.
    #[test]
    pub fn test_poly1305() {
        test::run(
            test_file!("data/poly1305_test.txt"),
            |section, test_case| {
                assert_eq!(section, "");
                let key = test_case.consume_bytes("Key");
                let key: &[u8; BLOCK_LEN * 2] = key.as_slice().try_into().unwrap();
                let input = test_case.consume_bytes("Input");
                let expected_mac = test_case.consume_bytes("MAC");
                let key = Key::new(*key);
                let Tag(actual_mac, _) = sign(key, &input);
                assert_eq!(expected_mac, actual_mac.as_ref());

                Ok(())
            },
        );
    }
}
