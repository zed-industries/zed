// Copyright (c) 2021, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#if defined(OPENSSL_AARCH64) && defined(OPENSSL_APPLE) && \
    !defined(OPENSSL_STATIC_ARMCAP)

#include <sys/sysctl.h>
#include <sys/types.h>

#include <openssl/arm_arch.h>

#include "cpu_aarch64.h"

static int has_hw_feature(const char *name) {
  int value;
  size_t len = sizeof(value);
  if (sysctlbyname(name, &value, &len, NULL, 0) != 0) {
    return 0;
  }
  if (len != sizeof(int)) {
    // This should not happen. All the values queried should be integer-valued.
    assert(0);
    return 0;
  }

  // Per sys/sysctl.h:
  //
  //   Selectors that return errors are not support on the system. Supported
  //   features will return 1 if they are recommended or 0 if they are supported
  //   but are not expected to help performance. Future versions of these
  //   selectors may return larger values as necessary so it is best to test for
  //   non zero.
  return value != 0;
}

// This function compares the brand retrieved with the input string
// up to the length of the shortest of these 2 strings.
static int is_brand(const char *in_str) {
  char brand[64];
  size_t len = sizeof(brand);
  if (sysctlbyname("machdep.cpu.brand_string", brand, &len, NULL, 0) != 0 ||
      strncmp(brand, in_str, strnlen(in_str, len)) != 0) {
    return 0;
  }

  if (len > sizeof(brand)) {
    // This should not happen; too large of a brand for this function.
    assert(0);
    return 0;
  }

  return 1;
}

void OPENSSL_cpuid_setup(void) {
  // Apple ARM64 platforms have NEON and cryptography extensions available
  // statically, so we do not need to query them. In particular, there sometimes
  // are no sysctls corresponding to such features. See below.
#if !defined(__ARM_NEON) || !defined(__ARM_FEATURE_AES) || \
    !defined(__ARM_FEATURE_SHA2)
#error "NEON and crypto extensions should be statically available."
#endif
  OPENSSL_armcap_P =
      ARMV7_NEON | ARMV8_AES | ARMV8_PMULL | ARMV8_SHA1 | ARMV8_SHA256;

  // See Apple's documentation for sysctl names:
  // https://developer.apple.com/documentation/kernel/1387446-sysctlbyname/determining_instruction_set_characteristics
  //
  // The new feature names, e.g. "hw.optional.arm.FEAT_SHA512", are only
  // available in macOS 12. For compatibility with macOS 11, we also support
  // the old names. The old names don't have values for features like FEAT_AES,
  // so instead we detect them statically above.
  //
  // If querying new sysctls, update the Chromium sandbox definition. See
  // https://crrev.com/c/4415225.
  if (has_hw_feature("hw.optional.arm.FEAT_SHA512") ||
      has_hw_feature("hw.optional.armv8_2_sha512")) {
    OPENSSL_armcap_P |= ARMV8_SHA512;
  }

  if (has_hw_feature("hw.optional.armv8_2_sha3")) {
    OPENSSL_armcap_P |= ARMV8_SHA3;
  }

  if (is_brand("Apple M")) {
    OPENSSL_armcap_P |= ARMV8_APPLE_M;
  }

  if (has_hw_feature("hw.optional.arm.FEAT_DIT")) {
    OPENSSL_armcap_P |= (ARMV8_DIT | ARMV8_DIT_ALLOWED);
  }

  // OPENSSL_armcap is a 32-bit, unsigned value which may start with "0x" to
  // indicate a hex value. Prior to the 32-bit value, a '~' or '|' may be given.
  //
  // If the '~' prefix is present:
  //   the value is inverted and ANDed with the probed CPUID result
  // If the '|' prefix is present:
  //   the value is ORed with the probed CPUID result
  // Otherwise:
  //   the value is taken as the result of the CPUID
  const char *env;
  env = getenv("OPENSSL_armcap");
  if (env != NULL) {
    handle_cpu_env(&OPENSSL_armcap_P, env);
  }

  OPENSSL_cpucap_initialized = 1;
}

#endif  // OPENSSL_AARCH64 && OPENSSL_APPLE && !OPENSSL_STATIC_ARMCAP
