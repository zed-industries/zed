// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
// Mon Jun 29 12:41:07 UTC 2026

pub(super) const CRYPTO_LIBRARY: &[&str] = &[
    "generated-src/win-x86_64/crypto/chacha/chacha-x86_64.asm",
    "generated-src/win-x86_64/crypto/cipher_extra/aes128gcmsiv-x86_64.asm",
    "generated-src/win-x86_64/crypto/cipher_extra/aesni-sha1-x86_64.asm",
    "generated-src/win-x86_64/crypto/cipher_extra/aesni-sha256-x86_64.asm",
    "generated-src/win-x86_64/crypto/cipher_extra/chacha20_poly1305_x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/aesni-gcm-avx512.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/aesni-gcm-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/aesni-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/aesni-xts-avx512.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/ghash-ssse3-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/ghash-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/md5-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/p256-x86_64-asm.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/p256_beeu-x86_64-asm.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/rdrand-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/rsaz-2k-avx512.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/rsaz-3k-avx512.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/rsaz-4k-avx512.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/rsaz-avx2.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/sha1-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/sha256-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/sha512-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/vpaes-x86_64.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/x86_64-mont.asm",
    "generated-src/win-x86_64/crypto/fipsmodule/x86_64-mont5.asm",
    "generated-src/win-x86_64/crypto/test/trampoline-x86_64.asm",
];
