// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
// Mon Jun 29 12:41:07 UTC 2026

pub(super) const CRYPTO_LIBRARY: &[&str] = &[
    "generated-src/win-x86/crypto/chacha/chacha-x86.asm",
    "generated-src/win-x86/crypto/fipsmodule/aesni-x86.asm",
    "generated-src/win-x86/crypto/fipsmodule/bn-586.asm",
    "generated-src/win-x86/crypto/fipsmodule/co-586.asm",
    "generated-src/win-x86/crypto/fipsmodule/ghash-ssse3-x86.asm",
    "generated-src/win-x86/crypto/fipsmodule/ghash-x86.asm",
    "generated-src/win-x86/crypto/fipsmodule/md5-586.asm",
    "generated-src/win-x86/crypto/fipsmodule/sha1-586.asm",
    "generated-src/win-x86/crypto/fipsmodule/sha256-586.asm",
    "generated-src/win-x86/crypto/fipsmodule/sha512-586.asm",
    "generated-src/win-x86/crypto/fipsmodule/vpaes-x86.asm",
    "generated-src/win-x86/crypto/fipsmodule/x86-mont.asm",
    "generated-src/win-x86/crypto/test/trampoline-x86.asm",
];
