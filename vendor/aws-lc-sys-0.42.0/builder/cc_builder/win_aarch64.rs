// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
// Mon Jun 29 12:41:07 UTC 2026

pub(super) const CRYPTO_LIBRARY: &[&str] = &[
    "generated-src/win-aarch64/crypto/chacha/chacha-armv8.S",
    "generated-src/win-aarch64/crypto/cipher_extra/chacha20_poly1305_armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/aesv8-armx.S",
    "generated-src/win-aarch64/crypto/fipsmodule/aesv8-gcm-armv8-unroll8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/aesv8-gcm-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/armv8-mont.S",
    "generated-src/win-aarch64/crypto/fipsmodule/bn-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/ghash-neon-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/ghashv8-armx.S",
    "generated-src/win-aarch64/crypto/fipsmodule/keccak1600-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/md5-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/p256-armv8-asm.S",
    "generated-src/win-aarch64/crypto/fipsmodule/p256_beeu-armv8-asm.S",
    "generated-src/win-aarch64/crypto/fipsmodule/rndr-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/sha1-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/sha256-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/sha512-armv8.S",
    "generated-src/win-aarch64/crypto/fipsmodule/vpaes-armv8.S",
    "generated-src/win-aarch64/crypto/test/trampoline-armv8.S",
    "third_party/s2n-bignum/s2n-bignum-to-be-imported/arm/aes/aes-xts-dec.S",
    "third_party/s2n-bignum/s2n-bignum-to-be-imported/arm/aes/aes-xts-enc.S",
];
