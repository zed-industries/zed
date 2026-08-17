// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/base.h>

#if defined(OPENSSL_PPC64LE)

// On Linux, use the shared getauxval helper (which falls back to reading
// /proc/self/auxv when <sys/auxv.h> is unavailable). On FreeBSD, use the
// system-provided <sys/auxv.h>/elf_aux_info. On other platforms no
// auxv-style API is available; OPENSSL_ppc64le_hwcap2 stays zero-initialized.
#if defined(OPENSSL_LINUX)
#include "cpu_getauxval_linux.h"
#elif defined(OPENSSL_FREEBSD)
#include <sys/auxv.h>
#endif

#if !defined(PPC_FEATURE2_HAS_VCRYPTO)
// PPC_FEATURE2_HAS_VCRYPTO was taken from section 4.1.2.3 of the “OpenPOWER
// ABI for Linux Supplement”.
#define PPC_FEATURE2_HAS_VCRYPTO 0x02000000
#endif

static void handle_cpu_env(unsigned long *out, const char *in) {
  OPENSSL_STATIC_ASSERT(sizeof(unsigned long) == 8, PPC64LE_UNSIGNED_LONG_NOT_8_BYTES);

  const int invert = in[0] == '~';
  const int or = in[0] == '|';
  const int skip_first_byte = (invert || or) ? 1 : 0;
  const int hex = in[skip_first_byte] == '0' && in[skip_first_byte+1] == 'x';
  unsigned long ppccap = *out;

  int sscanf_result;
  uint64_t reqcap;
  if (hex) {
    sscanf_result = sscanf(in + skip_first_byte + 2, "%" PRIx64, &reqcap);
  } else {
    sscanf_result = sscanf(in + skip_first_byte, "%" PRIu64, &reqcap);
  }

  if (!sscanf_result) {
    return;
  }

  // Detect if the user is trying to use the environment variable to set
  // a capability that is _not_ available on the CPU.
  // The case of invert cannot enable an unexisting capability;
  // it can only disable an existing one.
  if (!invert && ppccap && (~ppccap & reqcap)) {
    fprintf(stderr,
            "Fatal Error: HW capability found: 0x%02lX, but HW capability requested: 0x%02lX.\n",
            ppccap, reqcap);
    abort();
  }

  if (invert) {
    *out &= ~reqcap;
  } else if (or) {
    *out |= reqcap;
  } else {
    *out = reqcap;
  }
}

extern uint8_t OPENSSL_cpucap_initialized;

void OPENSSL_cpuid_setup(void) {
#if defined(OPENSSL_LINUX)
  OPENSSL_ppc64le_hwcap2 = getauxval(AT_HWCAP2);
#elif defined(OPENSSL_FREEBSD) && defined(AT_HWCAP2)
  elf_aux_info(AT_HWCAP2, &OPENSSL_ppc64le_hwcap2, sizeof(OPENSSL_ppc64le_hwcap2));
#endif
  OPENSSL_cpucap_initialized = 1;

  // OPENSSL_ppccap is a 64-bit hex string which may start with "0x".
  // Prior to the value, a '~' or '|' may be given.
  //
  // If the '~' prefix is present:
  //   the value is inverted and ANDed with the probed CPUID result
  // If the '|' prefix is present:
  //   the value is ORed with the probed CPUID result
  // Otherwise:
  //   the value is taken as the result of the CPUID
  const char *env;
  env = getenv("OPENSSL_ppccap");
  if (env != NULL) {
    handle_cpu_env(&OPENSSL_ppc64le_hwcap2, env);
  }

}

int CRYPTO_is_PPC64LE_vcrypto_capable(void) {
  return (OPENSSL_ppc64le_hwcap2 & PPC_FEATURE2_HAS_VCRYPTO) != 0;
}

#endif  // OPENSSL_PPC64LE
