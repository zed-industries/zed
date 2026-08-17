// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/base.h>
#include "internal.h"

#if !defined(OPENSSL_NO_ASM) && (defined(OPENSSL_X86) || defined(OPENSSL_X86_64))

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif
#include <inttypes.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(_MSC_VER)
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <immintrin.h>
#include <intrin.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif

// OPENSSL_cpuid runs the cpuid instruction. |leaf| is passed in as EAX and ECX
// is set to zero. It writes EAX, EBX, ECX, and EDX to |*out_eax| through
// |*out_edx|.
static void OPENSSL_cpuid(uint32_t *out_eax, uint32_t *out_ebx,
                          uint32_t *out_ecx, uint32_t *out_edx, uint32_t leaf) {
#if defined(_MSC_VER)
  int tmp[4];
  __cpuid(tmp, (int)leaf);
  *out_eax = (uint32_t)tmp[0];
  *out_ebx = (uint32_t)tmp[1];
  *out_ecx = (uint32_t)tmp[2];
  *out_edx = (uint32_t)tmp[3];
#elif defined(__pic__) && defined(OPENSSL_32_BIT)
  // Inline assembly may not clobber the PIC register. For 32-bit, this is EBX.
  // See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=47602.
  __asm__ volatile (
    "xor %%ecx, %%ecx\n"
    "mov %%ebx, %%edi\n"
    "cpuid\n"
    "xchg %%edi, %%ebx\n"
    : "=a"(*out_eax), "=D"(*out_ebx), "=c"(*out_ecx), "=d"(*out_edx)
    : "a"(leaf)
  );
#else
  __asm__ volatile (
    "xor %%ecx, %%ecx\n"
    "cpuid\n"
    : "=a"(*out_eax), "=b"(*out_ebx), "=c"(*out_ecx), "=d"(*out_edx)
    : "a"(leaf)
  );
#endif
}

// OPENSSL_xgetbv returns the value of an Intel Extended Control Register (XCR).
// Currently only XCR0 is defined by Intel so |xcr| should always be zero.
static uint64_t OPENSSL_xgetbv(uint32_t xcr) {
#if defined(_MSC_VER)
  return (uint64_t)_xgetbv(xcr);
#else
  uint32_t eax, edx;
#if defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
  // Some old assemblers don't support the xgetbv instruction so we emit
  // the opcode of xgetbv directly.
  __asm__ volatile (".byte 0x0f, 0x01, 0xd0" : "=a"(eax), "=d"(edx) : "c"(xcr));
#else
  __asm__ volatile ("xgetbv" : "=a"(eax), "=d"(edx) : "c"(xcr));
#endif
  return (((uint64_t)edx) << 32) | eax;
#endif
}

static bool os_supports_avx512(uint64_t xcr0) {
#if defined(OPENSSL_APPLE)
  // The Darwin kernel had a bug where it could corrupt the opmask registers.
  // See
  // https://community.intel.com/t5/Software-Tuning-Performance/MacOS-Darwin-kernel-bug-clobbers-AVX-512-opmask-register-state/m-p/1327259
  // Darwin also does not initially set the XCR0 bits for AVX512, but they are
  // set if the thread tries to use AVX512 anyway.  Thus, to safely and
  // consistently use AVX512 on macOS we'd need to check the kernel version as
  // well as detect AVX512 support using a macOS-specific method.  We don't
  // bother with this, especially given Apple's transition to arm64.
  return false;
#else
  return (xcr0 & 0xe6) == 0xe6;
#endif
}

// handle_cpu_env applies the value from |in| to the CPUID values in |out[0]|
// and |out[1]|. See the comment in |OPENSSL_cpuid_setup| about this.
static void handle_cpu_env(uint32_t *out, const char *in) {
  const int invert = in[0] == '~';
  const int or = in[0] == '|';
  const int skip_first_byte = invert || or;
  const int hex = in[skip_first_byte] == '0' && in[skip_first_byte+1] == 'x';
  uint32_t intelcap0 = out[0];
  uint32_t intelcap1 = out[1];

  int sscanf_result;
  uint64_t v;
  if (hex) {
    sscanf_result = sscanf(in + skip_first_byte + 2, "%" PRIx64, &v);
  } else {
    sscanf_result = sscanf(in + skip_first_byte, "%" PRIu64, &v);
  }

  if (!sscanf_result) {
    return;
  }

  uint32_t reqcap0 = (uint32_t)(v & UINT32_MAX);
  uint32_t reqcap1 = (uint32_t)(v >> 32);

  // Detect if the user is trying to use the environment variable to set
  // a capability that is _not_ available on the CPU.
  // The case of invert cannot enable an unexisting capability;
  // it can only disable an existing one.
  if (!invert && (intelcap0 || intelcap1)) {
    // Allow Intel indicator bit to be set for testing
    if((~(1u << 30 | intelcap0) & reqcap0) || (~intelcap1 & reqcap1)) {
      fprintf(stderr,
              "Fatal Error: HW capability found: 0x%02X 0x%02X, but HW capability requested: 0x%02X 0x%02X.\n",
              intelcap0, intelcap1, reqcap0, reqcap1);
      abort();
    }
  }

  if (invert) {
    out[0] &= ~reqcap0;
    out[1] &= ~reqcap1;
  } else if (or) {
    out[0] |= reqcap0;
    out[1] |= reqcap1;
  } else {
    out[0] = reqcap0;
    out[1] = reqcap1;
  }
}

extern uint8_t OPENSSL_cpucap_initialized;

static int amd_rdrand_maybe_apply_restrictions(const uint32_t family,
                                               const uint32_t model) {

  // Disable RDRAND on AMD families before 0x17 (Zen) due to reported failures
  // after suspend. https://bugzilla.redhat.com/show_bug.cgi?id=1150286
  // Also disable for family 0x17, models 0x70–0x7f, due to possible RDRAND
  // failures there too.
  if (family < 0x17 || (family == 0x17 && 0x70 <= model && model <= 0x7f)) {
    return 1;
  }

  // Zen2 EPYC have prohibitively slow RDRAND implementations. Specifically,
  // measured on the model EPYC 7R32. Please see q/VxC3AiwXpAjJ.
  // We assume that slow implementations is universal to all AMD models based
  // on the Zen2 uarch. Additionally, extend this assumptions to Zen1 based
  // AMD models as well because Zen1 and Zen2 shares family number.
  if (family == 0x17) {
    return 1;
  }

  // No restrictions.
  return 0;
}


void OPENSSL_cpuid_setup(void) {
  // Determine the vendor and maximum input value.
  uint32_t eax, ebx, ecx, edx;
  OPENSSL_cpuid(&eax, &ebx, &ecx, &edx, 0);

  uint32_t num_ids = eax;

  int is_intel = ebx == 0x756e6547 /* Genu */ &&
                 edx == 0x49656e69 /* ineI */ &&
                 ecx == 0x6c65746e /* ntel */;
  int is_amd = ebx == 0x68747541 /* Auth */ &&
               edx == 0x69746e65 /* enti */ &&
               ecx == 0x444d4163 /* cAMD */;

  uint32_t extended_features[2] = {0};
  if (num_ids >= 7) {
    OPENSSL_cpuid(&eax, &ebx, &ecx, &edx, 7);
    extended_features[0] = ebx;
    extended_features[1] = ecx;
  }

  OPENSSL_cpuid(&eax, &ebx, &ecx, &edx, 1);

  if (is_amd) {
    // See https://www.amd.com/system/files/TechDocs/25481.pdf, page 10.
    const uint32_t base_family = (eax >> 8) & 15;
    const uint32_t base_model = (eax >> 4) & 15;

    uint32_t family = base_family;
    uint32_t model = base_model;
    if (base_family == 0xf) {
      const uint32_t ext_family = (eax >> 20) & 255;
      family += ext_family;
      const uint32_t ext_model = (eax >> 16) & 15;
      model |= ext_model << 4;
    }

    if (amd_rdrand_maybe_apply_restrictions(family, model) != 0) {
      ecx &= ~(1u << 30);
    }
  }

  // Force the hyper-threading bit so that the more conservative path is always
  // chosen.
  edx |= 1u << 28;

  // Reserved bit #20 was historically repurposed to control the in-memory
  // representation of RC4 state. Always set it to zero.
  edx &= ~(1u << 20);

  // Reserved bit #30 is repurposed to signal an Intel CPU.
  if (is_intel) {
    edx |= (1u << 30);

    // Clear the XSAVE bit on Knights Landing to mimic Silvermont. This enables
    // some Silvermont-specific codepaths which perform better. See OpenSSL
    // commit 64d92d74985ebb3d0be58a9718f9e080a14a8e7f and
    // |CRYPTO_cpu_perf_is_like_silvermont|.
    if ((eax & 0x0fff0ff0) == 0x00050670 /* Knights Landing */ ||
        (eax & 0x0fff0ff0) == 0x00080650 /* Knights Mill (per SDE) */) {
      ecx &= ~(1u << 26);
    }
  } else {
    edx &= ~(1u << 30);
  }

  // The SDBG bit is repurposed to denote AMD XOP support. Don't ever use AMD
  // XOP code paths.
  ecx &= ~(1u << 11);

  uint64_t xcr0 = 0;
  if (ecx & (1u << 27)) {
    // XCR0 may only be queried if the OSXSAVE bit is set.
    xcr0 = OPENSSL_xgetbv(0);
  }
  // See Intel manual, volume 1, section 14.3.
  if ((xcr0 & 6) != 6) {
    // YMM registers cannot be used.
    ecx &= ~(1u << 28);  // AVX
    ecx &= ~(1u << 12);  // FMA
    ecx &= ~(1u << 11);  // AMD XOP
    // Clear AVX2 and AVX512* bits.
    //
    // TODO(davidben): Should bits 17 and 26-28 also be cleared? Upstream
    // doesn't clear those. See the comments in
    // |CRYPTO_hardware_supports_XSAVE|.
    extended_features[0] &=
        ~((1u << 5) | (1u << 16) | (1u << 21) | (1u << 30) | (1u << 31));
  }
  // See Intel manual, volume 1, section 15.2.
  if (!os_supports_avx512(xcr0)) {
    // Clear AVX512F. Note we don't touch other AVX512 extensions because they
    // can be used with YMM.
    extended_features[0] &= ~(1u << 16);
  }

  // Disable ADX instructions on Knights Landing. See OpenSSL commit
  // 64d92d74985ebb3d0be58a9718f9e080a14a8e7f.
  if ((ecx & (1u << 26)) == 0) {
    extended_features[0] &= ~(1u << 19);
  }

  OPENSSL_ia32cap_P[0] = edx;
  OPENSSL_ia32cap_P[1] = ecx;
  OPENSSL_ia32cap_P[2] = extended_features[0];
  OPENSSL_ia32cap_P[3] = extended_features[1];

  OPENSSL_cpucap_initialized = 1;

  const char *env1, *env2;
  env1 = getenv("OPENSSL_ia32cap");
  if (env1 == NULL) {
    return;
  }

  // OPENSSL_ia32cap can contain zero, one or two values, separated with a ':'.
  // Each value is a 64-bit, unsigned value which may start with "0x" to
  // indicate a hex value. Prior to the 64-bit value, a '~' or '|' may be given.
  //
  // If the '~' prefix is present:
  //   the value is inverted and ANDed with the probed CPUID result
  // If the '|' prefix is present:
  //   the value is ORed with the probed CPUID result
  // Otherwise:
  //   the value is taken as the result of the CPUID
  //
  // The first value determines OPENSSL_ia32cap_P[0] and [1]. The second [2]
  // and [3].

  handle_cpu_env(&OPENSSL_ia32cap_P[0], env1);
  env2 = strchr(env1, ':');
  if (env2 != NULL) {
    handle_cpu_env(&OPENSSL_ia32cap_P[2], env2 + 1);
  }
}

#endif  // !OPENSSL_NO_ASM && (OPENSSL_X86 || OPENSSL_X86_64)
