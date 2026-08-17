// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef MLD_CONFIG_H
#define MLD_CONFIG_H

#if !defined(__ASSEMBLER__)
#include "../../internal.h"
#endif

// Namespacing: All symbols are of the form mldsa*. Level-specific
// symbols are further prefixed with their security level, e.g.
// mldsa44*, mldsa65*, mldsa87*.
#define MLD_CONFIG_NAMESPACE_PREFIX mldsa

// Replace mldsa-native's FIPS 202 headers with glue code to
// AWS-LC's own FIPS 202 implementation.
#define MLD_CONFIG_FIPS202_CUSTOM_HEADER "../fips202_glue.h"
#define MLD_CONFIG_FIPS202X4_CUSTOM_HEADER "../fips202x4_glue.h"

// Everything is built in a single CU, so both internal and external
// mldsa-native API can have internal linkage.
// Mark as unused to suppress warnings for functions we don't expose
#if defined(__GNUC__) || defined(__clang__)
#define MLD_CONFIG_INTERNAL_API_QUALIFIER static __attribute__((unused))
#define MLD_CONFIG_EXTERNAL_API_QUALIFIER static __attribute__((unused))
#else
#define MLD_CONFIG_INTERNAL_API_QUALIFIER static
#define MLD_CONFIG_EXTERNAL_API_QUALIFIER static
#endif

// Enable PCT if and only if AWS-LC is built in FIPS-mode.
#if defined(AWSLC_FIPS)
#define MLD_CONFIG_KEYGEN_PCT
#endif

// Map the CPU capability function to the ones used by AWS-LC
#define MLD_CONFIG_CUSTOM_CAPABILITY_FUNC
#if !defined(__ASSEMBLER__)
#include <stdint.h>
#include "mldsa/sys.h"
static MLD_INLINE int mld_sys_check_capability(mld_sys_cap cap)
{
#if defined(MLD_SYS_X86_64)
  if (cap == MLD_SYS_CAP_AVX2)
  {
    return CRYPTO_is_AVX2_capable();
  }
#endif
  return 0;
}
#endif

#if defined(BORINGSSL_FIPS_BREAK_TESTS)
#define MLD_CONFIG_KEYGEN_PCT_BREAKAGE_TEST
#if !defined(__ASSEMBLER__) && !defined(MLD_CONFIG_MULTILEVEL_NO_SHARED)
#include "mldsa/sys.h"
static MLD_INLINE int mld_break_pct(void) {
  return boringssl_fips_break_test("MLDSA_PWCT");
}
#endif // !__ASSEMBLER__
#endif // BORINGSSL_FIPS_BREAK_TESTS

// Enable valgrind-based assertions in mldsa-native through macro
// from AWS-LC/BoringSSL.
#if defined(BORINGSSL_CONSTANT_TIME_VALIDATION)
#define MLD_CONFIG_CT_TESTING_ENABLED
#endif

// Map zeroization function to the one used by AWS-LC
#define MLD_CONFIG_CUSTOM_ZEROIZE
#if !defined(__ASSEMBLER__) && !defined(MLD_CONFIG_MULTILEVEL_NO_SHARED)
#include <stdint.h>
#include "mldsa/sys.h"
#include <openssl/base.h>
static MLD_INLINE void mld_zeroize(void *ptr, size_t len) {
    OPENSSL_cleanse(ptr, len);
}
#endif // !__ASSEMBLER__

// Map randombytes function to the one used by AWS-LC
// Note: mldsa-native expects int return (0 on success, non-zero on failure)
// unlike mlkem-native which expects void return
#define MLD_CONFIG_CUSTOM_RANDOMBYTES
#if !defined(__ASSEMBLER__) && !defined(MLD_CONFIG_MULTILEVEL_NO_SHARED)
#include <stdint.h>
#include "mldsa/sys.h"
#include <openssl/rand.h>
static MLD_INLINE int mld_randombytes(uint8_t *ptr, size_t len) {
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(ptr, len));
    return 0;
}
#endif // !__ASSEMBLER__

// Map memcpy function to the one used by AWS-LC
#define MLD_CONFIG_CUSTOM_MEMCPY
#if !defined(__ASSEMBLER__)
#include <stdint.h>
#include "mldsa/sys.h"
static MLD_INLINE void *mld_memcpy(void *dest, const void *src, size_t n) {
    return OPENSSL_memcpy(dest, src, n);
}
#endif // !__ASSEMBLER__

// Map memset function to the one used by AWS-LC
#define MLD_CONFIG_CUSTOM_MEMSET
#if !defined(__ASSEMBLER__)
#include <stdint.h>
#include "mldsa/sys.h"
static MLD_INLINE void *mld_memset(void *s, int c, size_t n) {
    return OPENSSL_memset(s, c, n);
}
#endif // !__ASSEMBLER__

#if defined(OPENSSL_NO_ASM)
#define MLD_CONFIG_NO_ASM
#endif

// Enable x86_64 arithmetic backend and set path
#define MLD_CONFIG_USE_NATIVE_BACKEND_ARITH
#define MLD_CONFIG_ARITH_BACKEND_FILE "../mldsa_native_backend.h"

#endif // MLD_CONFIG_H
