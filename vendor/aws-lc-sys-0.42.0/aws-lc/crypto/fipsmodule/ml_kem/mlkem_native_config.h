// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef MLK_CONFIG_H
#define MLK_CONFIG_H

#if !defined(__ASSEMBLER__)
#include "../../internal.h"
#endif

// Namespacing: All symbols are of the form mlkem*. Level-specific
// symbols are further prefixed with their security level, e.g.
// mlkem512*, mlkem768*, mlkem1024*.
#define MLK_CONFIG_NAMESPACE_PREFIX mlkem

// Replace mlkem-native's FIPS 202 headers with glue code to
// AWS-LC's own FIPS 202 implementation.
#define MLK_CONFIG_FIPS202_CUSTOM_HEADER "../fips202_glue.h"
#define MLK_CONFIG_FIPS202X4_CUSTOM_HEADER "../fips202x4_glue.h"

// Everything is built in a single CU, so both internal and external
// mlkem-native API can have internal linkage.
#define MLK_CONFIG_INTERNAL_API_QUALIFIER static
#define MLK_CONFIG_EXTERNAL_API_QUALIFIER static

// Enable PCT if and only if AWS-LC is built in FIPS-mode.
#if defined(AWSLC_FIPS)
#define MLK_CONFIG_KEYGEN_PCT
#endif

// Map the CPU capability function to the ones used by AWS-LC
#define MLK_CONFIG_CUSTOM_CAPABILITY_FUNC
#if !defined(__ASSEMBLER__)
#include <stdint.h>
#include "mlkem/sys.h"
static MLK_INLINE int mlk_sys_check_capability(mlk_sys_cap cap)
{
#if defined(MLK_SYS_X86_64)
  if (cap == MLK_SYS_CAP_AVX2)
  {
    return CRYPTO_is_AVX2_capable();
  }
#endif
  return 0;
}
#endif

#if defined(BORINGSSL_FIPS_BREAK_TESTS)
#define MLK_CONFIG_KEYGEN_PCT_BREAKAGE_TEST
#if !defined(__ASSEMBLER__) && !defined(MLK_CONFIG_MULTILEVEL_NO_SHARED)
#include "mlkem/sys.h"
static MLK_INLINE int mlk_break_pct(void) {
  return boringssl_fips_break_test("MLKEM_PWCT");
}
#endif // !__ASSEMBLER__
#endif // BORINGSSL_FIPS_BREAK_TESTS

// Enable valgrind-based assertions in mlkem-native through macro
// from AWS-LC/BoringSSL.
#if defined(BORINGSSL_CONSTANT_TIME_VALIDATION)
#define MLK_CONFIG_CT_TESTING_ENABLED
#endif

// Map zeroization function to the one used by AWS-LC
#define MLK_CONFIG_CUSTOM_ZEROIZE
#if !defined(__ASSEMBLER__) && !defined(MLK_CONFIG_MULTILEVEL_NO_SHARED)
#include <stdint.h>
#include "mlkem/sys.h"
#include <openssl/base.h>
static MLK_INLINE void mlk_zeroize(void *ptr, size_t len) {
    OPENSSL_cleanse(ptr, len);
}
#endif // !__ASSEMBLER__

// Map randombytes function to the one used by AWS-LC
#define MLK_CONFIG_CUSTOM_RANDOMBYTES
#if !defined(__ASSEMBLER__) && !defined(MLK_CONFIG_MULTILEVEL_NO_SHARED)
#include <stdint.h>
#include "mlkem/sys.h"
#include <openssl/rand.h>
static MLK_INLINE int mlk_randombytes(void *ptr, size_t len) {
    // RAND_bytes returns 1 on success
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(ptr, len));
    return 0;
}
#endif // !__ASSEMBLER__

// Map memcpy function to the one used by AWS-LC
#define MLK_CONFIG_CUSTOM_MEMCPY
#if !defined(__ASSEMBLER__)
#include <stdint.h>
#include "mlkem/sys.h"
static MLK_INLINE void *mlk_memcpy(void *dest, const void *src, size_t n) {
    return OPENSSL_memcpy(dest, src, n);
}
#endif // !__ASSEMBLER__

// Map memset function to the one used by AWS-LC
#define MLK_CONFIG_CUSTOM_MEMSET
#if !defined(__ASSEMBLER__)
#include <stdint.h>
#include "mlkem/sys.h"
static MLK_INLINE void *mlk_memset(void *s, int c, size_t n) {
    return OPENSSL_memset(s, c, n);
}
#endif // !__ASSEMBLER__

#if defined(OPENSSL_NO_ASM)
#define MLK_CONFIG_NO_ASM
#endif

// Enable AArch64 arithmetic backend and set path
#define MLK_CONFIG_USE_NATIVE_BACKEND_ARITH
#define MLK_CONFIG_ARITH_BACKEND_FILE "../mlkem_native_backend.h"

// We require all of ML-KEM-512/768/1024
//
// This option only affects mlkem_native.h. It is therefore, at present,
// irrelevant for AWS-LC since we directly embed the mlkem_native_bcm.c and
// hence inherit its declarations, removing the need or mlkem_native.h.
// Still, it's more robust to specify this here.
#define MLK_CONFIG_MULTILEVEL_BUILD

#endif // MLkEM_NATIVE_CONFIG_H
