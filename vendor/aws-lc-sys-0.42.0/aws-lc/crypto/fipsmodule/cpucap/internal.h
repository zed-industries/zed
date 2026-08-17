// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_CPUCAP_INTERNAL_H
#define OPENSSL_HEADER_CPUCAP_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64) || defined(OPENSSL_ARM) || \
    defined(OPENSSL_AARCH64) || defined(OPENSSL_PPC64LE)
#define HAS_OPENSSL_CPUID_SETUP
// OPENSSL_cpuid_setup initializes the platform-specific feature cache.
void OPENSSL_cpuid_setup(void);
#endif

// Runtime CPU feature support

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
// OPENSSL_ia32cap_P contains the Intel CPUID bits when running on an x86 or
// x86-64 system.
//
//   Index 0:
//     EDX for CPUID where EAX = 1
//     Bit 20 is always zero
//     Bit 28 is adjusted to reflect whether the data cache is shared between
//       multiple logical cores
//     Bit 30 is used to indicate an Intel CPU
//   Index 1:
//     ECX for CPUID where EAX = 1
//     Bit 11 is used to indicate AMD XOP support, not SDBG
//   Index 2:
//     EBX for CPUID where EAX = 7
//   Index 3:
//     ECX for CPUID where EAX = 7
//
// Note: the CPUID bits are pre-adjusted for the OSXSAVE bit and the YMM and XMM
// bits in XCR0, so it is not necessary to check those. (WARNING: See caveats
// in cpu_intel.c.)
extern uint32_t OPENSSL_ia32cap_P[4];

#if defined(BORINGSSL_FIPS) && !defined(BORINGSSL_SHARED_LIBRARY)
// The FIPS module, as a static library, requires an out-of-line version of
// |OPENSSL_ia32cap_get| so accesses can be rewritten by delocate. Mark the
// function const so multiple accesses can be optimized together.
const uint32_t *OPENSSL_ia32cap_get(void) __attribute__((const));
#else
OPENSSL_INLINE const uint32_t *OPENSSL_ia32cap_get(void) {
  return OPENSSL_ia32cap_P;
}
#endif

// See Intel manual, volume 2A, table 3-11.

OPENSSL_INLINE int CRYPTO_is_FXSR_capable(void) {
  return (OPENSSL_ia32cap_get()[0] & (1 << 24)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_intel_cpu(void) {
  // The reserved bit 30 is used to indicate an Intel CPU.
  return (OPENSSL_ia32cap_get()[0] & (1 << 30)) != 0;
}

// See Intel manual, volume 2A, table 3-10.

OPENSSL_INLINE int CRYPTO_is_PCLMUL_capable(void) {
  return (OPENSSL_ia32cap_get()[1] & (1 << 1)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_SSSE3_capable(void) {
  return (OPENSSL_ia32cap_get()[1] & (1 << 9)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_SSE4_1_capable(void) {
  return (OPENSSL_ia32cap_get()[1] & (1 << 19)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_MOVBE_capable(void) {
  return (OPENSSL_ia32cap_get()[1] & (1 << 22)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_AESNI_capable(void) {
  return (OPENSSL_ia32cap_get()[1] & (1 << 25)) != 0;
}

// We intentionally avoid defining a |CRYPTO_is_XSAVE_capable| function. See
// |CRYPTO_cpu_perf_is_like_silvermont|.

OPENSSL_INLINE int CRYPTO_is_AVX_capable(void) {
  return (OPENSSL_ia32cap_get()[1] & (1 << 28)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_RDRAND_capable(void) {
  return (OPENSSL_ia32cap_get()[1] & (1u << 30)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_AMD_XOP_support(void) {
  return (OPENSSL_ia32cap_get()[1] & (1 << 11)) != 0;
}

// See Intel manual, volume 2A, table 3-8.

OPENSSL_INLINE int CRYPTO_is_BMI1_capable(void) {
  return (OPENSSL_ia32cap_get()[2] & (1 << 3)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_AVX2_capable(void) {
  return (OPENSSL_ia32cap_get()[2] & (1 << 5)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_BMI2_capable(void) {
  return (OPENSSL_ia32cap_get()[2] & (1 << 8)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ADX_capable(void) {
  return (OPENSSL_ia32cap_get()[2] & (1 << 19)) != 0;
}

OPENSSL_INLINE int CRYPTO_is_SHAEXT_capable(void) {
  return (OPENSSL_ia32cap_get()[2] & (1 << 29)) != 0;
}

// AVX512VL  | AVX512BW | AVX512DQ | AVX512F
// 1u << 31  | 1u << 30 | 1u << 17 | 1u << 16
// 1100_0000_0000_0011_0000_0000_0000_0000
#define CPU_CAP_AVX512_BITFLAGS 0xC0030000
OPENSSL_INLINE int CRYPTO_is_AVX512_capable(void) {
  return (OPENSSL_ia32cap_get()[2] & CPU_CAP_AVX512_BITFLAGS) == CPU_CAP_AVX512_BITFLAGS;
}

OPENSSL_INLINE int CRYPTO_is_VAES_capable(void) {
  return (OPENSSL_ia32cap_get()[3] & (1u << (41 - 32))) != 0;
}

OPENSSL_INLINE int CRYPTO_is_VPCLMULQDQ_capable(void) {
  return (OPENSSL_ia32cap_get()[3] & (1u << (42 - 32))) != 0;
}

// AVX512VL  | AVX512BW | AVX512_IFMA | AVX512DQ | AVX512F
// 1u << 31  | 1u << 30 | 1u << 21    | 1u << 17 | 1u << 16
// 1100_0000_0010_0011_0000_0000_0000_0000
#define CPU_CAP_AVX512IFMA_BITFLAGS 0xC0230000
OPENSSL_INLINE int CRYPTO_is_AVX512IFMA_capable(void) {
  return (OPENSSL_ia32cap_get()[2] & CPU_CAP_AVX512IFMA_BITFLAGS) ==
         CPU_CAP_AVX512IFMA_BITFLAGS;
}

OPENSSL_INLINE int CRYPTO_is_VBMI2_capable(void) {
  return (OPENSSL_ia32cap_get()[3] & (1 << 6)) != 0;
}

// CRYPTO_cpu_perf_is_like_silvermont returns one if, based on a heuristic, the
// CPU has Silvermont-like performance characteristics. It is often faster to
// run different codepaths on these CPUs than the available instructions would
// otherwise select. See chacha-x86_64.pl.
//
// Bonnell, Silvermont's predecessor in the Atom lineup, will also be matched by
// this. |OPENSSL_cpuid_setup| forces Knights Landing to also be matched by
// this. Goldmont (Silvermont's successor in the Atom lineup) added XSAVE so it
// isn't matched by this. Various sources indicate AMD first implemented MOVBE
// and XSAVE at the same time in Jaguar, so it seems like AMD chips will not be
// matched by this. That seems to be the case for other x86(-64) CPUs.
OPENSSL_INLINE int CRYPTO_cpu_perf_is_like_silvermont(void) {
  // WARNING: This MUST NOT be used to guard the execution of the XSAVE
  // instruction. This is the "hardware supports XSAVE" bit, not the OSXSAVE bit
  // that indicates whether we can safely execute XSAVE. This bit may be set
  // even when XSAVE is disabled (by the operating system). See the comment in
  // cpu_intel.c and check how the users of this bit use it.
  //
  // We do not use |__XSAVE__| for static detection because the hack in
  // |OPENSSL_cpuid_setup| for Knights Landing CPUs needs to override it.
  int hardware_supports_xsave = (OPENSSL_ia32cap_get()[1] & (1u << 26)) != 0;
  return !hardware_supports_xsave && CRYPTO_is_MOVBE_capable();
}

#endif  // OPENSSL_X86 || OPENSSL_X86_64

#if defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)

#if defined(OPENSSL_APPLE) && defined(OPENSSL_ARM)
// We do not detect any features at runtime for Apple's 32-bit ARM platforms. On
// 64-bit ARM, we detect some post-ARMv8.0 features.
#define OPENSSL_STATIC_ARMCAP
#endif

#include <openssl/arm_arch.h>

extern uint32_t OPENSSL_armcap_P;
extern uint8_t OPENSSL_cpucap_initialized;

// Normalize some older feature flags to their modern ACLE values.
// https://developer.arm.com/architectures/system-architectures/software-standards/acle
#if defined(__ARM_NEON__) && !defined(__ARM_NEON)
#define __ARM_NEON 1
#endif
#if defined(__ARM_FEATURE_CRYPTO)
#if !defined(__ARM_FEATURE_AES)
#define __ARM_FEATURE_AES 1
#endif
#if !defined(__ARM_FEATURE_SHA2)
#define __ARM_FEATURE_SHA2 1
#endif
#endif

// CRYPTO_is_NEON_capable returns true if the current CPU has a NEON unit.
// If this is known statically, it is a constant inline function.
// Otherwise, the capability is checked at runtime by checking the corresponding
// bit in |OPENSSL_armcap_P|. This is also the same for
// |CRYPTO_is_ARMv8_AES_capable| and |CRYPTO_is_ARMv8_PMULL_capable|
// for checking the support for AES and PMULL instructions, respectively.
OPENSSL_INLINE int CRYPTO_is_NEON_capable(void) {
  return (OPENSSL_armcap_P & ARMV7_NEON) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_AES_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_AES) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_PMULL_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_PMULL) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_SHA1_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_SHA1) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_SHA256_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_SHA256) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_SHA512_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_SHA512) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_SHA3_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_SHA3) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_GCM_8x_capable(void) {
  return (CRYPTO_is_ARMv8_SHA3_capable() &&
          ((OPENSSL_armcap_P & ARMV8_NEOVERSE_V1) != 0 ||
           (OPENSSL_armcap_P & ARMV8_NEOVERSE_V2) != 0 ||
           (OPENSSL_armcap_P & ARMV8_APPLE_M) != 0));
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_wide_multiplier_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_NEOVERSE_V1) != 0 ||
           (OPENSSL_armcap_P & ARMV8_NEOVERSE_V2) != 0 ||
           (OPENSSL_armcap_P & ARMV8_APPLE_M) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_DIT_capable(void) {
  return (OPENSSL_armcap_P & (ARMV8_DIT | ARMV8_DIT_ALLOWED)) ==
    (ARMV8_DIT | ARMV8_DIT_ALLOWED);
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_RNDR_capable(void) {
  return (OPENSSL_armcap_P & ARMV8_RNG) != 0;
}

OPENSSL_INLINE int CRYPTO_is_Neoverse_N1(void) {
  // It is Neoverse N1 if only ARMV8_NEOVERSE_N1 = 1 and
  // ARMV8_NEOVERSE_<V1|V2> = 0.
  return ((OPENSSL_armcap_P & ARMV8_NEOVERSE_N1) != 0 &&
          (OPENSSL_armcap_P & (ARMV8_NEOVERSE_V1 | ARMV8_NEOVERSE_V2)) == 0);
}

OPENSSL_INLINE int CRYPTO_is_Neoverse_V1(void) {
  return (OPENSSL_armcap_P & ARMV8_NEOVERSE_V1) != 0;
}

OPENSSL_INLINE int CRYPTO_is_Neoverse_V2(void) {
  return (OPENSSL_armcap_P & ARMV8_NEOVERSE_V2) != 0;
}

OPENSSL_INLINE int CRYPTO_is_ARMv8_Apple_M(void) {
  return (OPENSSL_armcap_P & ARMV8_APPLE_M) != 0;
}

// This function is used only for testing; hence, not inlined
OPENSSL_EXPORT int CRYPTO_is_ARMv8_DIT_capable_for_testing(void);

#endif  // OPENSSL_ARM || OPENSSL_AARCH64

#if defined(AARCH64_DIT_SUPPORTED)
// (TODO): See if we can detect the DIT capability in Windows environment

// armv8_get_dit gets the value of the DIT flag from the CPU.
OPENSSL_EXPORT uint64_t armv8_get_dit(void);

// armv8_set_dit sets the CPU DIT flag to 1 and returns its original value
// before it was called.
OPENSSL_EXPORT uint64_t armv8_set_dit(void);

// armv8_restore_dit takes as input a value to restore the CPU DIT flag to.
OPENSSL_EXPORT void armv8_restore_dit(volatile uint64_t *original_dit);

#if defined(ENABLE_AUTO_SET_RESET_DIT)
// SET_DIT_AUTO_RESET can be inserted in the caller's application at
// the beginning of the code section that makes repeated calls to AWS-LC
// functions. The flag will be automatically restored to its original value
// at the end of the scope.
// This can minimise the effect on performance of repeatedly setting and
// disabling DIT.
// Instead of the macro, the functions above can be used.
// An example of their usage is present in the benchmarking function
// `Speed()` in `tool/speed.cc` when the option `-dit` is passed in.
#define SET_DIT_AUTO_RESET                      \
  volatile uint64_t _dit_restore_orig                \
         __attribute__((cleanup(armv8_restore_dit))) \
          OPENSSL_UNUSED = armv8_set_dit();

#else
#define SET_DIT_AUTO_RESET
#endif  // ENABLE_AUTO_SET_RESET_DIT

#else
#define SET_DIT_AUTO_RESET
#endif  // AARCH64_DIT_SUPPORTED

#if defined(OPENSSL_PPC64LE)

// CRYPTO_is_PPC64LE_vcrypto_capable returns true iff the current CPU supports
// the Vector.AES category of instructions.
int CRYPTO_is_PPC64LE_vcrypto_capable(void);

extern unsigned long OPENSSL_ppc64le_hwcap2;

#endif  // OPENSSL_PPC64LE

#if defined(__cplusplus)
}
#endif

#endif // OPENSSL_HEADER_CPUCAP_INTERNAL_H
