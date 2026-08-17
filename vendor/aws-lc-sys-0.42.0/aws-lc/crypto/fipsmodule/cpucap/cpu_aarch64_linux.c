// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#if defined(OPENSSL_AARCH64) && defined(OPENSSL_LINUX) && \
    !defined(OPENSSL_STATIC_ARMCAP)

#include "cpu_getauxval_linux.h"

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif

#include <openssl/arm_arch.h>

#include "cpu_aarch64.h"

static uint64_t armv8_cpuid_probe(void) {
  uint64_t val;
  __asm__ volatile("mrs %0, MIDR_EL1" : "=r" (val));
  return val;
}

void OPENSSL_cpuid_setup(void) {
  unsigned long hwcap = getauxval(AT_HWCAP);
  unsigned long hwcap2 = getauxval(AT_HWCAP2);

  // See /usr/include/asm/hwcap.h on an aarch64 installation for the source of
  // these values.
  static const unsigned long kNEON = 1 << 1;
  static const unsigned long kAES = 1 << 3;
  static const unsigned long kPMULL = 1 << 4;
  static const unsigned long kSHA1 = 1 << 5;
  static const unsigned long kSHA256 = 1 << 6;
  static const unsigned long kSHA512 = 1 << 21;
  static const unsigned long kSHA3 = 1 << 17;
  static const unsigned long kCPUID = 1 << 11;

  static const unsigned long kRNGhwcap2 = 1 << 16;;

  uint64_t OPENSSL_arm_midr = 0;

  if ((hwcap & kNEON) == 0) {
    // Matching OpenSSL, if NEON is missing, don't report other features
    // either.
    return;
  }

  OPENSSL_armcap_P |= ARMV7_NEON;

  if (hwcap & kAES) {
    OPENSSL_armcap_P |= ARMV8_AES;
  }
  if (hwcap & kPMULL) {
    OPENSSL_armcap_P |= ARMV8_PMULL;
  }
  if (hwcap & kSHA1) {
    OPENSSL_armcap_P |= ARMV8_SHA1;
  }
  if (hwcap & kSHA256) {
    OPENSSL_armcap_P |= ARMV8_SHA256;
  }
  if (hwcap & kSHA512) {
    OPENSSL_armcap_P |= ARMV8_SHA512;
  }
  if (hwcap & kSHA3) {
    OPENSSL_armcap_P |= ARMV8_SHA3;
  }

  // Before calling armv8_cpuid_probe and reading from MIDR_EL1 check that it
  // is supported. As of Valgrind 3.21 trying to read from that register will
  // cause Valgrind to crash.
  if (hwcap & kCPUID) {
    OPENSSL_arm_midr = armv8_cpuid_probe();
    if (MIDR_IS_CPU_MODEL(OPENSSL_arm_midr, ARM_CPU_IMP_ARM, ARM_CPU_PART_N1)) {
      OPENSSL_armcap_P |= ARMV8_NEOVERSE_N1;
    }
    if (MIDR_IS_CPU_MODEL(OPENSSL_arm_midr, ARM_CPU_IMP_ARM, ARM_CPU_PART_V1)) {
      OPENSSL_armcap_P |= ARMV8_NEOVERSE_V1;
      // CPU capabilities of N1 are a subset of CPU capabilities of V1
      OPENSSL_armcap_P |= ARMV8_NEOVERSE_N1;
    }
    if (MIDR_IS_CPU_MODEL(OPENSSL_arm_midr, ARM_CPU_IMP_ARM, ARM_CPU_PART_V2)) {
      OPENSSL_armcap_P |= ARMV8_NEOVERSE_V2;
      // CPU capabilities of N1 are a subset of CPU capabilities of V2
      OPENSSL_armcap_P |= ARMV8_NEOVERSE_N1;
    }
  }

  static const unsigned long kDIT = 1 << 24;
  // Before setting/resetting the DIT flag, check it's available in HWCAP
  if (hwcap & kDIT) {
    OPENSSL_armcap_P |= (ARMV8_DIT | ARMV8_DIT_ALLOWED);
  }

  if (hwcap2 & kRNGhwcap2) {
    OPENSSL_armcap_P |= ARMV8_RNG;
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

#endif  // OPENSSL_AARCH64 && OPENSSL_LINUX && !OPENSSL_STATIC_ARMCAP
