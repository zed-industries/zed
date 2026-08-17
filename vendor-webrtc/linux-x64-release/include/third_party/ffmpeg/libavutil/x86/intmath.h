/*
 * Copyright (c) 2015 James Almer
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVUTIL_X86_INTMATH_H
#define AVUTIL_X86_INTMATH_H

#include <stdint.h>
#include <stdlib.h>
#if HAVE_FAST_CLZ
#if defined(_MSC_VER)
#include <intrin.h>
#elif defined(__INTEL_COMPILER)
#include <immintrin.h>
#endif
#endif
#include "config.h"

#if HAVE_FAST_CLZ
#if (defined(__INTEL_COMPILER) && (__INTEL_COMPILER>=1216)) || defined(_MSC_VER)
#   if defined(__INTEL_COMPILER)
#       define ff_log2(x) (_bit_scan_reverse((x)|1))
#   else
#       define ff_log2 ff_log2_x86
static av_always_inline av_const int ff_log2_x86(unsigned int v)
{
    unsigned long n;
    _BitScanReverse(&n, v|1);
    return n;
}
#   endif
#   define ff_log2_16bit av_log2

#if defined(__INTEL_COMPILER) || (defined(_MSC_VER) && (_MSC_VER >= 1700) && \
                                  (defined(__BMI__) || !defined(__clang__)))
#   define ff_ctz(v) _tzcnt_u32(v)

#   if ARCH_X86_64
#       define ff_ctzll(v) _tzcnt_u64(v)
#   else
#       define ff_ctzll ff_ctzll_x86
static av_always_inline av_const int ff_ctzll_x86(long long v)
{
    return ((uint32_t)v == 0) ? _tzcnt_u32((uint32_t)(v >> 32)) + 32 : _tzcnt_u32((uint32_t)v);
}
#   endif
#endif /* _MSC_VER */

#endif /* __INTEL_COMPILER */

#endif /* HAVE_FAST_CLZ */

#if defined(__GNUC__)

/* Our generic version of av_popcount is faster than GCC's built-in on
 * CPUs that don't support the popcnt instruction.
 */
#if defined(__POPCNT__)
    #define av_popcount   __builtin_popcount
#if ARCH_X86_64
    #define av_popcount64 __builtin_popcountll
#endif

#endif /* __POPCNT__ */

#if defined(__BMI2__)

#if AV_GCC_VERSION_AT_LEAST(5,1)
#if defined(ASSERT_LEVEL) && ASSERT_LEVEL >= 2
#define av_zero_extend av_zero_extend_bmi2
static av_always_inline av_const unsigned av_zero_extend_bmi2(unsigned a, unsigned p)
{
    if (p > 31) abort();
    return __builtin_ia32_bzhi_si(a, p);
}
#else
#define av_zero_extend __builtin_ia32_bzhi_si
#endif
#elif HAVE_INLINE_ASM
/* GCC releases before 5.1.0 have a broken bzhi builtin, so for those we
 * implement it using inline assembly
 */
#define av_zero_extend av_zero_extend_bmi2
static av_always_inline av_const unsigned av_zero_extend_bmi2(unsigned a, unsigned p)
{
#if defined(ASSERT_LEVEL) && ASSERT_LEVEL >= 2
    if (p > 31) abort();
#endif
    if (av_builtin_constant_p(p))
        return a & ((1U << p) - 1);
    else {
        unsigned x;
        __asm__ ("bzhi %2, %1, %0 \n\t" : "=r"(x) : "rm"(a), "r"(p));
        return x;
    }
}
#endif /* AV_GCC_VERSION_AT_LEAST */

#endif /* __BMI2__ */

#if defined(__SSE2__) && !defined(__INTEL_COMPILER)

#define av_clipd av_clipd_sse2
static av_always_inline av_const double av_clipd_sse2(double a, double amin, double amax)
{
#if defined(ASSERT_LEVEL) && ASSERT_LEVEL >= 2
    if (amin > amax) abort();
#endif
    __asm__ ("maxsd %1, %0 \n\t"
             "minsd %2, %0 \n\t"
             : "+&x"(a) : "xm"(amin), "xm"(amax));
    return a;
}

#endif /* __SSE2__ */

#if defined(__SSE__) && !defined(__INTEL_COMPILER)

#define av_clipf av_clipf_sse
static av_always_inline av_const float av_clipf_sse(float a, float amin, float amax)
{
#if defined(ASSERT_LEVEL) && ASSERT_LEVEL >= 2
    if (amin > amax) abort();
#endif
    __asm__ ("maxss %1, %0 \n\t"
             "minss %2, %0 \n\t"
             : "+&x"(a) : "xm"(amin), "xm"(amax));
    return a;
}

#endif /* __SSE__ */

#if defined(__AVX__) && !defined(__INTEL_COMPILER)

#undef av_clipd
#define av_clipd av_clipd_avx
static av_always_inline av_const double av_clipd_avx(double a, double amin, double amax)
{
#if defined(ASSERT_LEVEL) && ASSERT_LEVEL >= 2
    if (amin > amax) abort();
#endif
    __asm__ ("vmaxsd %1, %0, %0 \n\t"
             "vminsd %2, %0, %0 \n\t"
             : "+&x"(a) : "xm"(amin), "xm"(amax));
    return a;
}

#undef av_clipf
#define av_clipf av_clipf_avx
static av_always_inline av_const float av_clipf_avx(float a, float amin, float amax)
{
#if defined(ASSERT_LEVEL) && ASSERT_LEVEL >= 2
    if (amin > amax) abort();
#endif
    __asm__ ("vmaxss %1, %0, %0 \n\t"
             "vminss %2, %0, %0 \n\t"
             : "+&x"(a) : "xm"(amin), "xm"(amax));
    return a;
}

#endif /* __AVX__ */

#endif /* __GNUC__ */

#endif /* AVUTIL_X86_INTMATH_H */
