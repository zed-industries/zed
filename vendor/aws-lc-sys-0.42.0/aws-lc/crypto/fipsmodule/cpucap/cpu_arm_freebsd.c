// Copyright (c) 2022, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#if defined(OPENSSL_ARM) && defined(OPENSSL_FREEBSD) && \
    !defined(OPENSSL_STATIC_ARMCAP)
#include <sys/auxv.h>
#include <sys/types.h>

#include <openssl/arm_arch.h>
#include <openssl/mem.h>


void OPENSSL_cpuid_setup(void) {
  unsigned long hwcap = 0, hwcap2 = 0;

  // |elf_aux_info| may fail, in which case |hwcap| and |hwcap2| will be
  // left at zero. The rest of this function will then gracefully report
  // the features are absent.
  elf_aux_info(AT_HWCAP, &hwcap, sizeof(hwcap));
#if defined(AT_HWCAP2)
  elf_aux_info(AT_HWCAP2, &hwcap2, sizeof(hwcap2));
#endif

  // Matching OpenSSL, only report other features if NEON is present.
  if (hwcap & HWCAP_NEON) {
    OPENSSL_armcap_P |= ARMV7_NEON;

    if (hwcap2 & HWCAP2_AES) {
      OPENSSL_armcap_P |= ARMV8_AES;
    }
    if (hwcap2 & HWCAP2_PMULL) {
      OPENSSL_armcap_P |= ARMV8_PMULL;
    }
    if (hwcap2 & HWCAP2_SHA1) {
      OPENSSL_armcap_P |= ARMV8_SHA1;
    }
    if (hwcap2 & HWCAP2_SHA2) {
      OPENSSL_armcap_P |= ARMV8_SHA256;
    }
  }
}

#endif  // OPENSSL_ARM && OPENSSL_OPENBSD && !OPENSSL_STATIC_ARMCAP
