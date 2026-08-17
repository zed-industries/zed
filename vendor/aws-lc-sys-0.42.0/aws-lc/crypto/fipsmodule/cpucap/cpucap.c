// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// The contents of this file were copied from crypto/crypto.c to this file
// as part of the change that moves the CPU capability logic inside the FIPS
// module to satisfy the FIPS requirements.

#include <openssl/cpu.h>

// Our assembly does not use the GOT to reference symbols, which means
// references to visible symbols will often require a TEXTREL. This is
// undesirable, so all assembly-referenced symbols should be hidden. CPU
// capabilities are the only such symbols defined in C. Explicitly hide them,
// rather than rely on being built with -fvisibility=hidden.
#if defined(OPENSSL_WINDOWS)
#define HIDDEN
#else
#define HIDDEN __attribute__((visibility("hidden")))
#endif

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)

// This value must be explicitly initialised to zero in order to work around a
// bug in libtool or the linker on OS X.
//
// If not initialised then it becomes a "common symbol". When put into an
// archive, linking on OS X will fail to resolve common symbols. By
// initialising it to zero, it becomes a "data symbol", which isn't so
// affected.
HIDDEN uint32_t OPENSSL_ia32cap_P[4] = {0};

#elif defined(OPENSSL_PPC64LE)

HIDDEN unsigned long OPENSSL_ppc64le_hwcap2 = 0;

#elif defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)

#include <openssl/arm_arch.h>

#if defined(OPENSSL_STATIC_ARMCAP)

// See ARM ACLE for the definitions of these macros. Note |__ARM_FEATURE_AES|
// covers both AES and PMULL and |__ARM_FEATURE_SHA2| covers SHA-1 and SHA-256.
// https://developer.arm.com/architectures/system-architectures/software-standards/acle
// https://github.com/ARM-software/acle/issues/152
//
// TODO(davidben): Do we still need |OPENSSL_STATIC_ARMCAP_*| or are the
// standard flags and -march sufficient?
HIDDEN uint32_t OPENSSL_armcap_P =
#if defined(OPENSSL_STATIC_ARMCAP_NEON) || defined(__ARM_NEON)
    ARMV7_NEON |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_AES) || defined(__ARM_FEATURE_AES)
    ARMV8_AES |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_PMULL) || defined(__ARM_FEATURE_AES)
    ARMV8_PMULL |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_SHA1) || defined(__ARM_FEATURE_SHA2)
    ARMV8_SHA1 |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_SHA256) || defined(__ARM_FEATURE_SHA2)
    ARMV8_SHA256 |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_SHA512) || defined(__ARM_FEATURE_SHA512)
    ARMV8_SHA512 |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_SHA3) || defined(__ARM_FEATURE_SHA3)
    ARMV8_SHA3 |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_NEOVERSE_N1) || defined(__ARM_FEATURE_NEOVERSE_N1)
    ARMV8_NEOVERSE_N1 |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_NEOVERSE_V1) || defined(__ARM_FEATURE_NEOVERSE_V1)
    ARMV8_NEOVERSE_V1 |
#endif
#if defined(OPENSSL_STATIC_ARMCAP_NEOVERSE_V2) || defined(__ARM_FEATURE_NEOVERSE_V2)
    ARMV8_NEOVERSE_V2 |
#endif
    0;

#else
HIDDEN uint32_t OPENSSL_armcap_P = 0;
#endif

#endif

#if defined(BORINGSSL_DISPATCH_TEST)
// This value must be explicitly initialized to zero. See similar comment above.
HIDDEN uint8_t BORINGSSL_function_hit[15] = {0};
#endif // BORINGSSL_DISPATCH_TEST

// This variable is used only for testing purposes to ensure that the library
// constructor is executed and the capability variable is initialized.
HIDDEN uint8_t OPENSSL_cpucap_initialized = 0;
