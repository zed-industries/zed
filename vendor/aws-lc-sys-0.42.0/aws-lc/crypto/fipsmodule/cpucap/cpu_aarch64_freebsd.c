// Copyright (c) 2022, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#if defined(OPENSSL_AARCH64) && defined(OPENSSL_FREEBSD) && \
    !defined(OPENSSL_STATIC_ARMCAP)

#include <machine/armreg.h>
#include <sys/types.h>

#include <openssl/arm_arch.h>


// ID_AA64ISAR0_*_VAL are defined starting FreeBSD 13.0. When FreeBSD
// 12.x is out of support, these compatibility macros can be removed.

#ifndef ID_AA64ISAR0_AES_VAL
#define ID_AA64ISAR0_AES_VAL ID_AA64ISAR0_AES
#endif
#ifndef ID_AA64ISAR0_SHA1_VAL
#define ID_AA64ISAR0_SHA1_VAL ID_AA64ISAR0_SHA1
#endif
#ifndef ID_AA64ISAR0_SHA2_VAL
#define ID_AA64ISAR0_SHA2_VAL ID_AA64ISAR0_SHA2
#endif

void OPENSSL_cpuid_setup(void) {
  uint64_t id_aa64isar0 = READ_SPECIALREG(id_aa64isar0_el1);

  OPENSSL_armcap_P |= ARMV7_NEON;

  if (ID_AA64ISAR0_AES_VAL(id_aa64isar0) >= ID_AA64ISAR0_AES_BASE) {
    OPENSSL_armcap_P |= ARMV8_AES;
  }
  if (ID_AA64ISAR0_AES_VAL(id_aa64isar0) >= ID_AA64ISAR0_AES_PMULL) {
    OPENSSL_armcap_P |= ARMV8_PMULL;
  }
  if (ID_AA64ISAR0_SHA1_VAL(id_aa64isar0) >= ID_AA64ISAR0_SHA1_BASE) {
    OPENSSL_armcap_P |= ARMV8_SHA1;
  }
  if (ID_AA64ISAR0_SHA2_VAL(id_aa64isar0) >= ID_AA64ISAR0_SHA2_BASE) {
    OPENSSL_armcap_P |= ARMV8_SHA256;
  }
  if (ID_AA64ISAR0_SHA2_VAL(id_aa64isar0) >= ID_AA64ISAR0_SHA2_512) {
    OPENSSL_armcap_P |= ARMV8_SHA512;
  }

  OPENSSL_cpucap_initialized = 1;
}

#endif  // OPENSSL_AARCH64 && OPENSSL_FREEBSD && !OPENSSL_STATIC_ARMCAP
