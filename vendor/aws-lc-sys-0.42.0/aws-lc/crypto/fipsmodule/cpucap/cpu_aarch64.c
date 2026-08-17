// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#if defined(OPENSSL_AARCH64) && !defined(OPENSSL_STATIC_ARMCAP)

#include "cpu_aarch64.h"

void handle_cpu_env(uint32_t *out, const char *in) {
  const int invert = in[0] == '~';
  const int or = in[0] == '|';
  const int skip_first_byte = invert || or;
  const int hex = in[skip_first_byte] == '0' && in[skip_first_byte+1] == 'x';
  uint32_t armcap = out[0];

  int sscanf_result;
  uint32_t v;
  if (hex) {
    sscanf_result = sscanf(in + skip_first_byte + 2, "%" PRIx32, &v);
  } else {
    sscanf_result = sscanf(in + skip_first_byte, "%" PRIu32, &v);
  }

  if (!sscanf_result) {
    return;
  }

  // Detect if the user is trying to use the environment variable to set
  // a capability that is _not_ available on the CPU:
  // If the runtime capability check (e.g via getauxval() on Linux)
  // returned a non-zero hwcap in `armcap` (out)
  // and a bit set in the requested `v` is not set in `armcap`,
  // abort instead of crashing later.
  // The case of invert cannot enable an unexisting capability;
  // it can only disable an existing one.
  if (!invert && armcap && (~armcap & v))
  {
    fprintf(stderr,
            "Fatal Error: HW capability found: 0x%02X, but HW capability requested: 0x%02X.\n",
            armcap, v);
    abort();
  }

  if (invert) {
    out[0] &= ~v;
  } else if (or) {
    out[0] |= v;
  } else {
    out[0] = v;
  }
}

#if defined(AARCH64_DIT_SUPPORTED)
// "DIT" is not recognised as a register name by clang-10 (at least)
// Register's encoded name is from e.g.
// https://github.com/ashwio/arm64-sysreg-lib/blob/d421e249a026f6f14653cb6f9c4edd8c5d898595/include/sysreg/dit.h#L286
#define DIT_REGISTER s3_3_c4_c2_5
DEFINE_STATIC_MUTEX(OPENSSL_armcap_P_lock)

uint64_t armv8_get_dit(void) {
  if (CRYPTO_is_ARMv8_DIT_capable()) {
    uint64_t val = 0;
    __asm__ volatile("mrs %0, s3_3_c4_c2_5" : "=r" (val));
    return (val >> 24) & 1;
  } else {
    return 0;
  }
}

// See https://github.com/torvalds/linux/blob/53eaeb7fbe2702520125ae7d72742362c071a1f2/arch/arm64/include/asm/sysreg.h#L82
// As per Arm ARM for v8-A, Section "C.5.1.3 op0 == 0b00, architectural hints,
// barriers and CLREX, and PSTATE access", ARM DDI 0487 J.a, system instructions
// for accessing PSTATE fields have the following encoding
// and C5.2.4 DIT, Data Independent Timing:
//	Op0 = 0, CRn = 4
//	Op1 (3 for DIT) , Op2 (5 for DIT) encodes the PSTATE field modified and defines the constraints.
//	CRm = Imm4 (#0 or #1 below)
//	Rt = 0x1f
uint64_t armv8_set_dit(void) {
  if (CRYPTO_is_ARMv8_DIT_capable()) {
    uint64_t original_dit = armv8_get_dit();
    // Encoding of "msr dit, #1"
    __asm__ volatile(".inst 0xd503415f");
    return original_dit;
  } else {
    return 0;
  }
}

void armv8_restore_dit(volatile uint64_t *original_dit) {
  if (*original_dit != 1 && CRYPTO_is_ARMv8_DIT_capable()) {
    // Encoding of "msr dit, #0"
    __asm__ volatile(".inst 0xd503405f");
  }
}

void armv8_disable_dit(void) {
  CRYPTO_STATIC_MUTEX_lock_write(OPENSSL_armcap_P_lock_bss_get());
  OPENSSL_armcap_P &= ~ARMV8_DIT_ALLOWED;
  CRYPTO_STATIC_MUTEX_unlock_write(OPENSSL_armcap_P_lock_bss_get());
}

void armv8_enable_dit(void) {
  CRYPTO_STATIC_MUTEX_lock_write(OPENSSL_armcap_P_lock_bss_get());
  OPENSSL_armcap_P |= ARMV8_DIT_ALLOWED;
  CRYPTO_STATIC_MUTEX_unlock_write(OPENSSL_armcap_P_lock_bss_get());
}

int CRYPTO_is_ARMv8_DIT_capable_for_testing(void) {
  return CRYPTO_is_ARMv8_DIT_capable();
}

#endif  // AARCH64_DIT_SUPPORTED

#endif // OPENSSL_AARCH64 && !OPENSSL_STATIC_ARMCAP
