// Copyright (c) 2022, Robert Nagy <robert@openbsd.org>
// SPDX-License-Identifier: ISC

#include <openssl/cpu.h>

#if defined(OPENSSL_AARCH64) && defined(OPENSSL_OPENBSD) && \
    !defined(OPENSSL_STATIC_ARMCAP)

#include <sys/sysctl.h>
#include <machine/cpu.h>
#include <machine/armreg.h>
#include <stdio.h>

#include <openssl/arm_arch.h>

#include "internal.h"


void OPENSSL_cpuid_setup(void) {
  // CTL_MACHDEP from sys/sysctl.h
  // CPU_ID_AA64ISAR0 from machine/cpu.h
  int isar0_mib[] = { CTL_MACHDEP, CPU_ID_AA64ISAR0 };
  size_t len = sizeof(uint64_t);
  uint64_t cpu_id = 0;

  if (sysctl(isar0_mib, 2, &cpu_id, &len, NULL, 0) < 0)
    return;

  OPENSSL_armcap_P |= ARMV7_NEON;

  if (ID_AA64ISAR0_AES(cpu_id) >= ID_AA64ISAR0_AES_BASE)
    OPENSSL_armcap_P |= ARMV8_AES;

  if (ID_AA64ISAR0_AES(cpu_id) >= ID_AA64ISAR0_AES_PMULL)
    OPENSSL_armcap_P |= ARMV8_PMULL;

  if (ID_AA64ISAR0_SHA1(cpu_id) >= ID_AA64ISAR0_SHA1_BASE)
    OPENSSL_armcap_P |= ARMV8_SHA1;

  if (ID_AA64ISAR0_SHA2(cpu_id) >= ID_AA64ISAR0_SHA2_BASE)
    OPENSSL_armcap_P |= ARMV8_SHA256;

  if (ID_AA64ISAR0_SHA2(cpu_id) >= ID_AA64ISAR0_SHA2_512)
    OPENSSL_armcap_P |= ARMV8_SHA512;

  OPENSSL_cpucap_initialized = 1;
}

#endif  // OPENSSL_AARCH64 && OPENSSL_OPENBSD && !OPENSSL_STATIC_ARMCAP
