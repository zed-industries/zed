/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_PORTS_X86_H_
#define AOM_AOM_PORTS_X86_H_
#include <stdlib.h>

#if defined(_MSC_VER)
#include <intrin.h> /* For __cpuidex, __rdtsc */
#endif

#include "aom/aom_integer.h"
#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  AOM_CPU_UNKNOWN = -1,
  AOM_CPU_AMD,
  AOM_CPU_AMD_OLD,
  AOM_CPU_CENTAUR,
  AOM_CPU_CYRIX,
  AOM_CPU_INTEL,
  AOM_CPU_NEXGEN,
  AOM_CPU_NSC,
  AOM_CPU_RISE,
  AOM_CPU_SIS,
  AOM_CPU_TRANSMETA,
  AOM_CPU_TRANSMETA_OLD,
  AOM_CPU_UMC,
  AOM_CPU_VIA,

  AOM_CPU_LAST
} aom_cpu_t;

#if defined(__GNUC__) || defined(__ANDROID__)
#if AOM_ARCH_X86_64
#define cpuid(func, func2, ax, bx, cx, dx)                      \
  __asm__ __volatile__("cpuid           \n\t"                   \
                       : "=a"(ax), "=b"(bx), "=c"(cx), "=d"(dx) \
                       : "a"(func), "c"(func2))
#else
#define cpuid(func, func2, ax, bx, cx, dx)     \
  __asm__ __volatile__(                        \
      "mov %%ebx, %%edi   \n\t"                \
      "cpuid              \n\t"                \
      "xchg %%edi, %%ebx  \n\t"                \
      : "=a"(ax), "=D"(bx), "=c"(cx), "=d"(dx) \
      : "a"(func), "c"(func2))
#endif
#elif defined(__SUNPRO_C) || \
    defined(__SUNPRO_CC) /* end __GNUC__ or __ANDROID__*/
#if AOM_ARCH_X86_64
#define cpuid(func, func2, ax, bx, cx, dx)     \
  asm volatile(                                \
      "xchg %rsi, %rbx \n\t"                   \
      "cpuid           \n\t"                   \
      "movl %ebx, %edi \n\t"                   \
      "xchg %rsi, %rbx \n\t"                   \
      : "=a"(ax), "=D"(bx), "=c"(cx), "=d"(dx) \
      : "a"(func), "c"(func2))
#else
#define cpuid(func, func2, ax, bx, cx, dx)     \
  asm volatile(                                \
      "pushl %ebx       \n\t"                  \
      "cpuid            \n\t"                  \
      "movl %ebx, %edi  \n\t"                  \
      "popl %ebx        \n\t"                  \
      : "=a"(ax), "=D"(bx), "=c"(cx), "=d"(dx) \
      : "a"(func), "c"(func2))
#endif
#else /* end __SUNPRO__ */
#if AOM_ARCH_X86_64
#if defined(_MSC_VER) && _MSC_VER > 1500
#define cpuid(func, func2, a, b, c, d) \
  do {                                 \
    int regs[4];                       \
    __cpuidex(regs, func, func2);      \
    a = regs[0];                       \
    b = regs[1];                       \
    c = regs[2];                       \
    d = regs[3];                       \
  } while (0)
#else
#define cpuid(func, func2, a, b, c, d) \
  do {                                 \
    int regs[4];                       \
    __cpuid(regs, func);               \
    a = regs[0];                       \
    b = regs[1];                       \
    c = regs[2];                       \
    d = regs[3];                       \
  } while (0)
#endif
#else
/* clang-format off */
#define cpuid(func, func2, a, b, c, d) \
  __asm mov eax, func                  \
  __asm mov ecx, func2                 \
  __asm cpuid                          \
  __asm mov a, eax                     \
  __asm mov b, ebx                     \
  __asm mov c, ecx                     \
  __asm mov d, edx
#endif
/* clang-format on */
#endif /* end others */

// NaCl has no support for xgetbv or the raw opcode.
#if !defined(__native_client__) && (defined(__i386__) || defined(__x86_64__))
static inline uint64_t xgetbv(void) {
  const uint32_t ecx = 0;
  uint32_t eax, edx;
  // Use the raw opcode for xgetbv for compatibility with older toolchains.
  __asm__ volatile(".byte 0x0f, 0x01, 0xd0\n"
                   : "=a"(eax), "=d"(edx)
                   : "c"(ecx));
  return ((uint64_t)edx << 32) | eax;
}
#elif (defined(_M_X64) || defined(_M_IX86)) && defined(_MSC_FULL_VER) && \
    _MSC_FULL_VER >= 160040219  // >= VS2010 SP1
#include <immintrin.h>
#define xgetbv() _xgetbv(0)
#elif defined(_MSC_VER) && defined(_M_IX86)
static inline uint64_t xgetbv(void) {
  uint32_t eax_, edx_;
  __asm {
    xor ecx, ecx  // ecx = 0
    // Use the raw opcode for xgetbv for compatibility with older toolchains.
    __asm _emit 0x0f __asm _emit 0x01 __asm _emit 0xd0
    mov eax_, eax
    mov edx_, edx
  }
  return ((uint64_t)edx_ << 32) | eax_;
}
#else
#define xgetbv() 0U  // no AVX for older x64 or unrecognized toolchains.
#endif

#if defined(_MSC_VER) && _MSC_VER >= 1700
#undef NOMINMAX
#define NOMINMAX
#undef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_FAMILY_APP)
#define getenv(x) NULL
#endif
#endif

#define HAS_MMX 0x01
#define HAS_SSE 0x02
#define HAS_SSE2 0x04
#define HAS_SSE3 0x08
#define HAS_SSSE3 0x10
#define HAS_SSE4_1 0x20
#define HAS_AVX 0x40
#define HAS_AVX2 0x80
#define HAS_SSE4_2 0x100
#ifndef BIT
#define BIT(n) (1u << (n))
#endif

#define MMX_BITS BIT(23)
#define SSE_BITS BIT(25)
#define SSE2_BITS BIT(26)
#define SSE3_BITS BIT(0)
#define SSSE3_BITS BIT(9)
#define SSE4_1_BITS BIT(19)
// Bits 27 (OSXSAVE) & 28 (256-bit AVX)
#define AVX_BITS (BIT(27) | BIT(28))
#define AVX2_BITS BIT(5)

#define FEATURE_SET(reg, feature) \
  (((reg) & (feature##_BITS)) == (feature##_BITS))

static inline int x86_simd_caps(void) {
  unsigned int flags = 0;
  unsigned int mask = ~0u;
  unsigned int max_cpuid_val, reg_eax, reg_ebx, reg_ecx, reg_edx;
  char *env;

  /* See if the CPU capabilities are being overridden by the environment */
  env = getenv("AOM_SIMD_CAPS");
  if (env && *env) return (int)strtol(env, NULL, 0);

  env = getenv("AOM_SIMD_CAPS_MASK");
  if (env && *env) mask = (unsigned int)strtoul(env, NULL, 0);

  /* Ensure that the CPUID instruction supports extended features */
  cpuid(0, 0, max_cpuid_val, reg_ebx, reg_ecx, reg_edx);

  if (max_cpuid_val < 1) return 0;

  /* Get the standard feature flags */
  cpuid(1, 0, reg_eax, reg_ebx, reg_ecx, reg_edx);

  flags |= FEATURE_SET(reg_edx, MMX) ? HAS_MMX : 0;
  flags |= FEATURE_SET(reg_edx, SSE) ? HAS_SSE : 0;
  flags |= FEATURE_SET(reg_edx, SSE2) ? HAS_SSE2 : 0;
  flags |= FEATURE_SET(reg_ecx, SSE3) ? HAS_SSE3 : 0;
  flags |= FEATURE_SET(reg_ecx, SSSE3) ? HAS_SSSE3 : 0;
  flags |= FEATURE_SET(reg_ecx, SSE4_1) ? HAS_SSE4_1 : 0;

  // bits 27 (OSXSAVE) & 28 (256-bit AVX)
  if (FEATURE_SET(reg_ecx, AVX)) {
    // Check for OS-support of YMM state. Necessary for AVX and AVX2.
    if ((xgetbv() & 0x6) == 0x6) {
      flags |= HAS_AVX;
      if (max_cpuid_val >= 7) {
        /* Get the leaf 7 feature flags. Needed to check for AVX2 support */
        cpuid(7, 0, reg_eax, reg_ebx, reg_ecx, reg_edx);
        flags |= FEATURE_SET(reg_ebx, AVX2) ? HAS_AVX2 : 0;
      }
    }
  }
  (void)reg_eax;  // Avoid compiler warning on unused-but-set variable.
  return flags & mask;
}

// Fine-Grain Measurement Functions
//
// If you are timing a small region of code, access the timestamp counter
// (TSC) via:
//
// unsigned int start = x86_tsc_start();
//   ...
// unsigned int end = x86_tsc_end();
// unsigned int diff = end - start;
//
// The start/end functions introduce a few more instructions than using
// x86_readtsc directly, but prevent the CPU's out-of-order execution from
// affecting the measurement (by having earlier/later instructions be evaluated
// in the time interval). See the white paper, "How to Benchmark Code
// Execution Times on Intel(R) IA-32 and IA-64 Instruction Set Architectures" by
// Gabriele Paoloni for more information.
//
// If you are timing a large function (CPU time > a couple of seconds), use
// x86_readtsc64 to read the timestamp counter in a 64-bit integer. The
// out-of-order leakage that can occur is minimal compared to total runtime.
static inline unsigned int x86_readtsc(void) {
#if defined(__GNUC__)
  unsigned int tsc;
  __asm__ __volatile__("rdtsc\n\t" : "=a"(tsc) :);
  return tsc;
#elif defined(__SUNPRO_C) || defined(__SUNPRO_CC)
  unsigned int tsc;
  asm volatile("rdtsc\n\t" : "=a"(tsc) :);
  return tsc;
#else
#if AOM_ARCH_X86_64
  return (unsigned int)__rdtsc();
#else
  __asm rdtsc;
#endif
#endif
}
// 64-bit CPU cycle counter
static inline uint64_t x86_readtsc64(void) {
#if defined(__GNUC__)
  uint32_t hi, lo;
  __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
  return ((uint64_t)hi << 32) | lo;
#elif defined(__SUNPRO_C) || defined(__SUNPRO_CC)
  uint_t hi, lo;
  asm volatile("rdtsc\n\t" : "=a"(lo), "=d"(hi));
  return ((uint64_t)hi << 32) | lo;
#else
#if AOM_ARCH_X86_64
  return (uint64_t)__rdtsc();
#else
  __asm rdtsc;
#endif
#endif
}

// 32-bit CPU cycle counter with a partial fence against out-of-order execution.
static inline unsigned int x86_readtscp(void) {
#if defined(__GNUC__)
  unsigned int tscp;
  __asm__ __volatile__("rdtscp\n\t" : "=a"(tscp) :);
  return tscp;
#elif defined(__SUNPRO_C) || defined(__SUNPRO_CC)
  unsigned int tscp;
  asm volatile("rdtscp\n\t" : "=a"(tscp) :);
  return tscp;
#elif defined(_MSC_VER)
  unsigned int ui;
  return (unsigned int)__rdtscp(&ui);
#else
#if AOM_ARCH_X86_64
  return (unsigned int)__rdtscp();
#else
  __asm rdtscp;
#endif
#endif
}

static inline unsigned int x86_tsc_start(void) {
  unsigned int reg_eax, reg_ebx, reg_ecx, reg_edx;
  // This call should not be removed. See function notes above.
  cpuid(0, 0, reg_eax, reg_ebx, reg_ecx, reg_edx);
  // Avoid compiler warnings on unused-but-set variables.
  (void)reg_eax;
  (void)reg_ebx;
  (void)reg_ecx;
  (void)reg_edx;
  return x86_readtsc();
}

static inline unsigned int x86_tsc_end(void) {
  uint32_t v = x86_readtscp();
  unsigned int reg_eax, reg_ebx, reg_ecx, reg_edx;
  // This call should not be removed. See function notes above.
  cpuid(0, 0, reg_eax, reg_ebx, reg_ecx, reg_edx);
  // Avoid compiler warnings on unused-but-set variables.
  (void)reg_eax;
  (void)reg_ebx;
  (void)reg_ecx;
  (void)reg_edx;
  return v;
}

#if defined(__GNUC__)
#define x86_pause_hint() __asm__ __volatile__("pause \n\t")
#elif defined(__SUNPRO_C) || defined(__SUNPRO_CC)
#define x86_pause_hint() asm volatile("pause \n\t")
#else
#if AOM_ARCH_X86_64
#define x86_pause_hint() _mm_pause();
#else
#define x86_pause_hint() __asm pause
#endif
#endif

#if defined(__GNUC__)
static void x87_set_control_word(unsigned short mode) {
  __asm__ __volatile__("fldcw %0" : : "m"(*&mode));
}
static unsigned short x87_get_control_word(void) {
  unsigned short mode;
  __asm__ __volatile__("fstcw %0\n\t" : "=m"(*&mode) :);
  return mode;
}
#elif defined(__SUNPRO_C) || defined(__SUNPRO_CC)
static void x87_set_control_word(unsigned short mode) {
  asm volatile("fldcw %0" : : "m"(*&mode));
}
static unsigned short x87_get_control_word(void) {
  unsigned short mode;
  asm volatile("fstcw %0\n\t" : "=m"(*&mode) :);
  return mode;
}
#elif AOM_ARCH_X86_64
/* No fldcw intrinsics on Windows x64, punt to external asm */
extern void aom_winx64_fldcw(unsigned short mode);
extern unsigned short aom_winx64_fstcw(void);
#define x87_set_control_word aom_winx64_fldcw
#define x87_get_control_word aom_winx64_fstcw
#else
static void x87_set_control_word(unsigned short mode) {
  __asm { fldcw mode }
}
static unsigned short x87_get_control_word(void) {
  unsigned short mode;
  __asm { fstcw mode }
  return mode;
}
#endif

static inline unsigned int x87_set_double_precision(void) {
  unsigned int mode = x87_get_control_word();
  // Intel 64 and IA-32 Architectures Developer's Manual: Vol. 1
  // https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-1-manual.pdf
  // 8.1.5.2 Precision Control Field
  // Bits 8 and 9 (0x300) of the x87 FPU Control Word ("Precision Control")
  // determine the number of bits used in floating point calculations. To match
  // later SSE instructions restrict x87 operations to Double Precision (0x200).
  // Precision                     PC Field
  // Single Precision (24-Bits)    00B
  // Reserved                      01B
  // Double Precision (53-Bits)    10B
  // Extended Precision (64-Bits)  11B
  x87_set_control_word((mode & ~0x300u) | 0x200u);
  return mode;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_PORTS_X86_H_
