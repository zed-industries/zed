// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/cpu.h>

#if defined(OPENSSL_AARCH64) && defined(OPENSSL_NETBSD) && \
    !defined(OPENSSL_STATIC_ARMCAP)

#include <aarch64/armreg.h>
#include <aarch64/cpu.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/param.h>
#include <sys/sysctl.h>

#include <openssl/arm_arch.h>

#include "internal.h"

// Helper function to query a specific CPU's capabilities
static int get_cpu_id(int cpu_num, struct aarch64_sysctl_cpu_id *id) {
  char sysctl_name[64];
  size_t len = sizeof(*id);

  snprintf(sysctl_name, sizeof(sysctl_name), "machdep.cpu%d.cpu_id", cpu_num);

  if (sysctlbyname(sysctl_name, id, &len, NULL, 0) < 0) {
    return -1;
  }

  if (len != sizeof(*id)) {
    return -1;
  }

  return 0;
}

void OPENSSL_cpuid_setup(void) {
  struct aarch64_sysctl_cpu_id cpu_id;

  // NetBSD's machdep.cpuN.cpu_id sysctl reads each core's ID registers
  // directly, so it reflects that specific core's capabilities, not a
  // system-wide minimum.

  // Initialize with all features enabled (we'll AND them together)
  uint64_t common_aa64isar0 = UINT64_MAX;
  uint64_t common_aa64pfr0 = UINT64_MAX;
  int found_cpu = 0;

  // Query up to 256 CPUs (arbitrary but reasonable upper limit)
  // Scan all possible CPU indices, tolerating gaps from offline/absent CPUs.
  // On big.LITTLE systems, CPU indices may not be contiguous (e.g., an offline
  // core creates a gap), so we must not stop at the first missing index.
  // NOTE: This is still subject to a TOCTOU race if CPUs come online after
  // this scan completes. The only fully correct solution would be a
  // kernel-provided feature intersection (like Linux's AT_HWCAP).
  for (size_t cpu_num = 0; cpu_num < 256; cpu_num++) {
    if (get_cpu_id(cpu_num, &cpu_id) < 0) {
      // Failed to read this CPU - either it doesn't exist or is offline.
      // Continue scanning: there may be higher-indexed cores with different
      // (potentially fewer) capabilities.
      continue;
    }

    found_cpu++;

    // Take the bitwise AND to get the intersection of capabilities
    // Only features present on ALL cores will remain set
    common_aa64isar0 &= cpu_id.ac_aa64isar0;
    common_aa64pfr0 &= cpu_id.ac_aa64pfr0;
  }

  // If we couldn't read any CPU info, return early without setting any features
  if (!found_cpu) {
    return;
  }

  // NEON (Advanced SIMD) is mandatory on all ARMv8-A cores
  OPENSSL_armcap_P |= ARMV7_NEON;

  // Inspired by the implementation of `cpu_identify2` here:
  // https://github.com/NetBSD/src/blob/62c785e59d064070166dab5d2a4492055effba89/sys/arch/aarch64/aarch64/cpu.c#L363

  // Macros below found in "armreg.h"
  // https://github.com/NetBSD/src/blame/62c785e59d064070166dab5d2a4492055effba89/sys/arch/aarch64/include/armreg.h

  // Check for AES and PMULL
  const uint64_t aes_detection =
      __SHIFTOUT(common_aa64isar0, ID_AA64ISAR0_EL1_AES);
  if (aes_detection >= ID_AA64ISAR0_EL1_AES_AES) {
    OPENSSL_armcap_P |= ARMV8_AES;
  }
  if (aes_detection >= ID_AA64ISAR0_EL1_AES_PMUL) {
    OPENSSL_armcap_P |= ARMV8_PMULL;
  }

  // Check for SHA1 support across all cores
  if (__SHIFTOUT(common_aa64isar0, ID_AA64ISAR0_EL1_SHA1) >=
      ID_AA64ISAR0_EL1_SHA1_SHA1CPMHSU) {
    OPENSSL_armcap_P |= ARMV8_SHA1;
  }

  // Check for SHA256 support across all cores
  const uint64_t sha2_detection =
      __SHIFTOUT(common_aa64isar0, ID_AA64ISAR0_EL1_SHA2);
  if (sha2_detection >= ID_AA64ISAR0_EL1_SHA2_SHA256HSU) {
    OPENSSL_armcap_P |= ARMV8_SHA256;
  }
  if (sha2_detection >= ID_AA64ISAR0_EL1_SHA2_SHA512HSU) {
    OPENSSL_armcap_P |= ARMV8_SHA512;
  }

  // Check for SHA3 support across all cores
  if (__SHIFTOUT(common_aa64isar0, ID_AA64ISAR0_EL1_SHA3) >=
      ID_AA64ISAR0_EL1_SHA3_EOR3) {
    OPENSSL_armcap_P |= ARMV8_SHA3;
  }

  // Check for RNG (RNDR/RNDRRS) support across all cores
  if (__SHIFTOUT(common_aa64isar0, ID_AA64ISAR0_EL1_RNDR) >=
      ID_AA64ISAR0_EL1_RNDR_RNDRRS) {
    OPENSSL_armcap_P |= ARMV8_RNG;
  }

  // Check for DIT (Data Independent Timing) support across all cores
  if (__SHIFTOUT(common_aa64pfr0, ID_AA64PFR0_EL1_DIT) >=
      ID_AA64PFR0_EL1_DIT_IMPL) {
    OPENSSL_armcap_P |= (ARMV8_DIT | ARMV8_DIT_ALLOWED);
  }

  OPENSSL_cpucap_initialized = 1;
}

#endif  // OPENSSL_AARCH64 && OPENSSL_NETBSD && !OPENSSL_STATIC_ARMCAP
