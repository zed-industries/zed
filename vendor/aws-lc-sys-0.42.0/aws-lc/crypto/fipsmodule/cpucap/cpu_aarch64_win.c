// Copyright (c) 2018, Google Inc.
// Copyright (c) 2020, Arm Ltd.
// SPDX-License-Identifier: ISC

#include "internal.h"

#if defined(OPENSSL_AARCH64) && defined(OPENSSL_WINDOWS) && \
    !defined(OPENSSL_STATIC_ARMCAP)

#include <windows.h>

#include <openssl/arm_arch.h>

void OPENSSL_cpuid_setup(void) {
  // We do not need to check for the presence of NEON, as Armv8-A always has it
  OPENSSL_armcap_P |= ARMV7_NEON;

  if (IsProcessorFeaturePresent(PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE)) {
    // These are all covered by one call in Windows
    OPENSSL_armcap_P |= ARMV8_AES;
    OPENSSL_armcap_P |= ARMV8_PMULL;
    OPENSSL_armcap_P |= ARMV8_SHA1;
    OPENSSL_armcap_P |= ARMV8_SHA256;
  }
  // As of writing, Windows does not have a |PF_*| value for ARMv8.2 SHA-512
  // extensions. When it does, add it here.

  OPENSSL_cpucap_initialized = 1;
}

#endif  // OPENSSL_AARCH64 && OPENSSL_WINDOWS && !OPENSSL_STATIC_ARMCAP
