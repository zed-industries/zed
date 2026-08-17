/*
 * Copyright (c) The mlkem-native project authors
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#ifndef MLD_SYS_H
#define MLD_SYS_H

#if !defined(MLD_CONFIG_NO_ASM) && (defined(__GNUC__) || defined(__clang__))
#define MLD_HAVE_INLINE_ASM
#endif

/* Try to find endianness, if not forced through CFLAGS already */
#if !defined(MLD_SYS_LITTLE_ENDIAN) && !defined(MLD_SYS_BIG_ENDIAN)
#if defined(__BYTE_ORDER__)
#if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
#define MLD_SYS_LITTLE_ENDIAN
#elif __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#define MLD_SYS_BIG_ENDIAN
#else
#error "__BYTE_ORDER__ defined, but don't recognize value."
#endif
#endif /* __BYTE_ORDER__ */

/* MSVC does not define __BYTE_ORDER__. However, MSVC only supports
 * little endian x86, x86_64, and AArch64. It is, hence, safe to assume
 * little endian. */
#if defined(_MSC_VER) && (defined(_M_X64) || defined(_M_AMD64) || \
                          defined(_M_IX86) || defined(_M_ARM64))
#define MLD_SYS_LITTLE_ENDIAN
#endif

#endif /* !MLD_SYS_LITTLE_ENDIAN && !MLD_SYS_BIG_ENDIAN */

/* Check if we're running on an AArch64 little endian system. _M_ARM64 is set by
 * MSVC. */
#if defined(__AARCH64EL__) || defined(_M_ARM64)
#define MLD_SYS_AARCH64
#endif

/* Check if we're running on an AArch64 big endian system. */
#if defined(__AARCH64EB__)
#define MLD_SYS_AARCH64_EB
#endif

/* Check if we're running on an Armv8.1-M system with MVE */
#if defined(__ARM_ARCH_8_1M_MAIN__) || defined(__ARM_FEATURE_MVE)
#define MLD_SYS_ARMV81M_MVE
#endif

#if defined(__x86_64__)
#define MLD_SYS_X86_64
#if defined(__AVX2__)
#define MLD_SYS_X86_64_AVX2
#endif
#endif /* __x86_64__ */

#if defined(MLD_SYS_LITTLE_ENDIAN) && defined(__powerpc64__)
#define MLD_SYS_PPC64LE
#endif

#if defined(__riscv) && defined(__riscv_xlen) && __riscv_xlen == 64
#define MLD_SYS_RISCV64
#endif

#if defined(MLD_SYS_RISCV64) && defined(__riscv_vector) && \
    defined(__riscv_v_intrinsic)
#define MLD_SYS_RISCV64_RVV
#endif

#if defined(__riscv) && defined(__riscv_xlen) && __riscv_xlen == 32
#define MLD_SYS_RISCV32
#endif

#if defined(_WIN64) || defined(_WIN32)
#define MLD_SYS_WINDOWS
#endif

#if defined(__linux__)
#define MLD_SYS_LINUX
#endif

#if defined(__APPLE__)
#define MLD_SYS_APPLE
#endif

/* If MLD_FORCE_AARCH64 is set, assert that we're indeed on an AArch64 system.
 */
#if defined(MLD_FORCE_AARCH64) && !defined(MLD_SYS_AARCH64)
#error "MLD_FORCE_AARCH64 is set, but we don't seem to be on an AArch64 system."
#endif

/* If MLD_FORCE_AARCH64_EB is set, assert that we're indeed on a big endian
 * AArch64 system. */
#if defined(MLD_FORCE_AARCH64_EB) && !defined(MLD_SYS_AARCH64_EB)
#error \
    "MLD_FORCE_AARCH64_EB is set, but we don't seem to be on an AArch64 system."
#endif

/* If MLD_FORCE_X86_64 is set, assert that we're indeed on an X86_64 system. */
#if defined(MLD_FORCE_X86_64) && !defined(MLD_SYS_X86_64)
#error "MLD_FORCE_X86_64 is set, but we don't seem to be on an X86_64 system."
#endif

#if defined(MLD_FORCE_PPC64LE) && !defined(MLD_SYS_PPC64LE)
#error "MLD_FORCE_PPC64LE is set, but we don't seem to be on a PPC64LE system."
#endif

#if defined(MLD_FORCE_RISCV64) && !defined(MLD_SYS_RISCV64)
#error "MLD_FORCE_RISCV64 is set, but we don't seem to be on a RISCV64 system."
#endif

#if defined(MLD_FORCE_RISCV32) && !defined(MLD_SYS_RISCV32)
#error "MLD_FORCE_RISCV32 is set, but we don't seem to be on a RISCV32 system."
#endif

/*
 * MLD_INLINE: Hint for inlining.
 * - MSVC: __inline
 * - C99+: inline
 * - GCC/Clang C90: __attribute__((unused)) to silence warnings
 * - Other C90: empty
 */
#if !defined(MLD_INLINE)
#if defined(_MSC_VER)
#define MLD_INLINE __inline
#elif defined(inline) || \
    (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L)
#define MLD_INLINE inline
#elif defined(__GNUC__) || defined(__clang__)
#define MLD_INLINE __attribute__((unused))
#else
#define MLD_INLINE
#endif
#endif /* !MLD_INLINE */

/*
 * MLD_ALWAYS_INLINE: Force inlining.
 * - MSVC: __forceinline
 * - GCC/Clang C99+: MLD_INLINE __attribute__((always_inline))
 * - Other: MLD_INLINE (no forced inlining)
 */
#if !defined(MLD_ALWAYS_INLINE)
#if defined(_MSC_VER)
#define MLD_ALWAYS_INLINE __forceinline
#elif (defined(__GNUC__) || defined(__clang__)) && \
    (defined(inline) ||                            \
     (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L))
#define MLD_ALWAYS_INLINE MLD_INLINE __attribute__((always_inline))
#else
#define MLD_ALWAYS_INLINE MLD_INLINE
#endif
#endif /* !MLD_ALWAYS_INLINE */

#ifndef MLD_STATIC_TESTABLE
#define MLD_STATIC_TESTABLE static
#endif

/*
 * C90 does not have the restrict compiler directive yet.
 * We don't use it in C90 builds.
 */
#if !defined(restrict)
#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L
#define MLD_RESTRICT restrict
#else
#define MLD_RESTRICT
#endif

#else /* !restrict */

#define MLD_RESTRICT restrict
#endif /* restrict */

#define MLD_DEFAULT_ALIGN 32
#define MLD_ALIGN_UP(N) \
  ((((N) + (MLD_DEFAULT_ALIGN - 1)) / MLD_DEFAULT_ALIGN) * MLD_DEFAULT_ALIGN)
#if defined(__GNUC__)
#define MLD_ALIGN __attribute__((aligned(MLD_DEFAULT_ALIGN)))
#elif defined(_MSC_VER)
#define MLD_ALIGN __declspec(align(MLD_DEFAULT_ALIGN))
#else
#define MLD_ALIGN /* No known support for alignment constraints */
#endif


/* New X86_64 CPUs support Conflow-flow protection using the CET instructions.
 * When enabled (through -fcf-protection=), all compilation units (including
 * empty ones) need to support CET for this to work.
 * For assembly, this means that source files need to signal support for
 * CET by setting the appropriate note.gnu.property section.
 * This can be achieved by including the <cet.h> header in all assembly file.
 * This file also provides the _CET_ENDBR macro which needs to be placed at
 * every potential target of an indirect branch.
 * If CET is enabled _CET_ENDBR maps to the endbr64 instruction, otherwise
 * it is empty.
 * In case the compiler does not support CET (e.g., <gcc8, <clang11),
 * the __CET__ macro is not set and we default to nothing.
 * Note that we only issue _CET_ENDBR instructions through the MLD_ASM_FN_SYMBOL
 * macro as the global symbols are the only possible targets of indirect
 * branches in our code.
 */
#if defined(MLD_SYS_X86_64)
#if defined(__CET__)
#include <cet.h>
#define MLD_CET_ENDBR _CET_ENDBR
#else
#define MLD_CET_ENDBR
#endif
#endif /* MLD_SYS_X86_64 */

#if defined(MLD_CONFIG_CT_TESTING_ENABLED) && !defined(__ASSEMBLER__)
#include <valgrind/memcheck.h>
#define MLD_CT_TESTING_SECRET(ptr, len) \
  VALGRIND_MAKE_MEM_UNDEFINED((ptr), (len))
#define MLD_CT_TESTING_DECLASSIFY(ptr, len) \
  VALGRIND_MAKE_MEM_DEFINED((ptr), (len))
#else /* MLD_CONFIG_CT_TESTING_ENABLED && !__ASSEMBLER__ */
#define MLD_CT_TESTING_SECRET(ptr, len) \
  do                                    \
  {                                     \
  } while (0)
#define MLD_CT_TESTING_DECLASSIFY(ptr, len) \
  do                                        \
  {                                         \
  } while (0)
#endif /* !(MLD_CONFIG_CT_TESTING_ENABLED && !__ASSEMBLER__) */

#if defined(__GNUC__) || defined(__clang__)
#define MLD_MUST_CHECK_RETURN_VALUE __attribute__((warn_unused_result))
#else
#define MLD_MUST_CHECK_RETURN_VALUE
#endif


#if !defined(__ASSEMBLER__)
/* System capability enumeration */
typedef enum
{
  /* x86_64 */
  MLD_SYS_CAP_AVX2,
  /* AArch64 */
  MLD_SYS_CAP_SHA3
} mld_sys_cap;

#if !defined(MLD_CONFIG_CUSTOM_CAPABILITY_FUNC)
#include "cbmc.h"

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_sys_check_capability(mld_sys_cap cap)
__contract__(
  ensures(return_value == 0 || return_value == 1)
)
{
  /* By default, we rely on compile-time feature detection/specification:
   * If a feature is enabled at compile-time, we assume it is supported by
   * the host that the resulting library/binary will be built on.
   * If this assumption is not true, you MUST overwrite this function.
   * See the documentation of MLD_CONFIG_CUSTOM_CAPABILITY_FUNC in
   * mldsa_native_config.h for more information. */
  (void)cap;
  return 1;
}
#endif /* !MLD_CONFIG_CUSTOM_CAPABILITY_FUNC */
#endif /* !__ASSEMBLER__ */

#endif /* !MLD_SYS_H */
