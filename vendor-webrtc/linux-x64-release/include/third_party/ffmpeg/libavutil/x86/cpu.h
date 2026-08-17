/*
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

#ifndef AVUTIL_X86_CPU_H
#define AVUTIL_X86_CPU_H

#include "libavutil/cpu.h"
#include "libavutil/cpu_internal.h"

#define AV_CPU_FLAG_AMD3DNOW    AV_CPU_FLAG_3DNOW
#define AV_CPU_FLAG_AMD3DNOWEXT AV_CPU_FLAG_3DNOWEXT

#define X86_AMD3DNOW(flags)         CPUEXT(flags, AMD3DNOW)
#define X86_AMD3DNOWEXT(flags)      CPUEXT(flags, AMD3DNOWEXT)
#define X86_MMX(flags)              CPUEXT(flags, MMX)
#define X86_MMXEXT(flags)           CPUEXT(flags, MMXEXT)
#define X86_SSE(flags)              CPUEXT(flags, SSE)
#define X86_SSE2(flags)             CPUEXT(flags, SSE2)
#define X86_SSE2_FAST(flags)        CPUEXT_FAST(flags, SSE2)
#define X86_SSE2_SLOW(flags)        CPUEXT_SLOW(flags, SSE2)
#define X86_SSE3(flags)             CPUEXT(flags, SSE3)
#define X86_SSE3_FAST(flags)        CPUEXT_FAST(flags, SSE3)
#define X86_SSE3_SLOW(flags)        CPUEXT_SLOW(flags, SSE3)
#define X86_SSSE3(flags)            CPUEXT(flags, SSSE3)
#define X86_SSSE3_FAST(flags)       CPUEXT_FAST(flags, SSSE3)
#define X86_SSSE3_SLOW(flags)       CPUEXT_SLOW(flags, SSSE3)
#define X86_SSE4(flags)             CPUEXT(flags, SSE4)
#define X86_SSE42(flags)            CPUEXT(flags, SSE42)
#define X86_AVX(flags)              CPUEXT(flags, AVX)
#define X86_AVX_FAST(flags)         CPUEXT_FAST(flags, AVX)
#define X86_AVX_SLOW(flags)         CPUEXT_SLOW(flags, AVX)
#define X86_XOP(flags)              CPUEXT(flags, XOP)
#define X86_FMA3(flags)             CPUEXT(flags, FMA3)
#define X86_FMA4(flags)             CPUEXT(flags, FMA4)
#define X86_AVX2(flags)             CPUEXT(flags, AVX2)
#define X86_AESNI(flags)            CPUEXT(flags, AESNI)
#define X86_AVX512(flags)           CPUEXT(flags, AVX512)

#define EXTERNAL_AMD3DNOW(flags)    CPUEXT_SUFFIX(flags, _EXTERNAL, AMD3DNOW)
#define EXTERNAL_AMD3DNOWEXT(flags) CPUEXT_SUFFIX(flags, _EXTERNAL, AMD3DNOWEXT)
#define EXTERNAL_MMX(flags)         CPUEXT_SUFFIX(flags, _EXTERNAL, MMX)
#define EXTERNAL_MMXEXT(flags)      CPUEXT_SUFFIX(flags, _EXTERNAL, MMXEXT)
#define EXTERNAL_SSE(flags)         CPUEXT_SUFFIX(flags, _EXTERNAL, SSE)
#define EXTERNAL_SSE2(flags)        CPUEXT_SUFFIX(flags, _EXTERNAL, SSE2)
#define EXTERNAL_SSE2_FAST(flags)   CPUEXT_SUFFIX_FAST(flags, _EXTERNAL, SSE2)
#define EXTERNAL_SSE2_SLOW(flags)   CPUEXT_SUFFIX_SLOW(flags, _EXTERNAL, SSE2)
#define EXTERNAL_SSE3(flags)        CPUEXT_SUFFIX(flags, _EXTERNAL, SSE3)
#define EXTERNAL_SSE3_FAST(flags)   CPUEXT_SUFFIX_FAST(flags, _EXTERNAL, SSE3)
#define EXTERNAL_SSE3_SLOW(flags)   CPUEXT_SUFFIX_SLOW(flags, _EXTERNAL, SSE3)
#define EXTERNAL_SSSE3(flags)       CPUEXT_SUFFIX(flags, _EXTERNAL, SSSE3)
#define EXTERNAL_SSSE3_FAST(flags)  CPUEXT_SUFFIX_FAST(flags, _EXTERNAL, SSSE3)
#define EXTERNAL_SSSE3_SLOW(flags)  CPUEXT_SUFFIX_SLOW(flags, _EXTERNAL, SSSE3)
#define EXTERNAL_SSE4(flags)        CPUEXT_SUFFIX(flags, _EXTERNAL, SSE4)
#define EXTERNAL_SSE42(flags)       CPUEXT_SUFFIX(flags, _EXTERNAL, SSE42)
#define EXTERNAL_AVX(flags)         CPUEXT_SUFFIX(flags, _EXTERNAL, AVX)
#define EXTERNAL_AVX_FAST(flags)    CPUEXT_SUFFIX_FAST(flags, _EXTERNAL, AVX)
#define EXTERNAL_AVX_SLOW(flags)    CPUEXT_SUFFIX_SLOW(flags, _EXTERNAL, AVX)
#define EXTERNAL_XOP(flags)         CPUEXT_SUFFIX(flags, _EXTERNAL, XOP)
#define EXTERNAL_FMA3(flags)        CPUEXT_SUFFIX(flags, _EXTERNAL, FMA3)
#define EXTERNAL_FMA3_FAST(flags)   CPUEXT_SUFFIX_FAST2(flags, _EXTERNAL, FMA3, AVX)
#define EXTERNAL_FMA3_SLOW(flags)   CPUEXT_SUFFIX_SLOW2(flags, _EXTERNAL, FMA3, AVX)
#define EXTERNAL_FMA4(flags)        CPUEXT_SUFFIX(flags, _EXTERNAL, FMA4)
#define EXTERNAL_AVX2(flags)        CPUEXT_SUFFIX(flags, _EXTERNAL, AVX2)
#define EXTERNAL_AVX2_FAST(flags)   CPUEXT_SUFFIX_FAST2(flags, _EXTERNAL, AVX2, AVX)
#define EXTERNAL_AVX2_SLOW(flags)   CPUEXT_SUFFIX_SLOW2(flags, _EXTERNAL, AVX2, AVX)
#define EXTERNAL_AESNI(flags)       CPUEXT_SUFFIX(flags, _EXTERNAL, AESNI)
#define EXTERNAL_AVX512(flags)      CPUEXT_SUFFIX(flags, _EXTERNAL, AVX512)
#define EXTERNAL_AVX512ICL(flags)   CPUEXT_SUFFIX(flags, _EXTERNAL, AVX512ICL)

#define INLINE_AMD3DNOW(flags)      CPUEXT_SUFFIX(flags, _INLINE, AMD3DNOW)
#define INLINE_AMD3DNOWEXT(flags)   CPUEXT_SUFFIX(flags, _INLINE, AMD3DNOWEXT)
#define INLINE_MMX(flags)           CPUEXT_SUFFIX(flags, _INLINE, MMX)
#define INLINE_MMXEXT(flags)        CPUEXT_SUFFIX(flags, _INLINE, MMXEXT)
#define INLINE_SSE(flags)           CPUEXT_SUFFIX(flags, _INLINE, SSE)
#define INLINE_SSE2(flags)          CPUEXT_SUFFIX(flags, _INLINE, SSE2)
#define INLINE_SSE2_FAST(flags)     CPUEXT_SUFFIX_FAST(flags, _INLINE, SSE2)
#define INLINE_SSE2_SLOW(flags)     CPUEXT_SUFFIX_SLOW(flags, _INLINE, SSE2)
#define INLINE_SSE3(flags)          CPUEXT_SUFFIX(flags, _INLINE, SSE3)
#define INLINE_SSE3_FAST(flags)     CPUEXT_SUFFIX_FAST(flags, _INLINE, SSE3)
#define INLINE_SSE3_SLOW(flags)     CPUEXT_SUFFIX_SLOW(flags, _INLINE, SSE3)
#define INLINE_SSSE3(flags)         CPUEXT_SUFFIX(flags, _INLINE, SSSE3)
#define INLINE_SSSE3_FAST(flags)    CPUEXT_SUFFIX_FAST(flags, _INLINE, SSSE3)
#define INLINE_SSSE3_SLOW(flags)    CPUEXT_SUFFIX_SLOW(flags, _INLINE, SSSE3)
#define INLINE_SSE4(flags)          CPUEXT_SUFFIX(flags, _INLINE, SSE4)
#define INLINE_SSE42(flags)         CPUEXT_SUFFIX(flags, _INLINE, SSE42)
#define INLINE_AVX(flags)           CPUEXT_SUFFIX(flags, _INLINE, AVX)
#define INLINE_AVX_FAST(flags)      CPUEXT_SUFFIX_FAST(flags, _INLINE, AVX)
#define INLINE_AVX_SLOW(flags)      CPUEXT_SUFFIX_SLOW(flags, _INLINE, AVX)
#define INLINE_XOP(flags)           CPUEXT_SUFFIX(flags, _INLINE, XOP)
#define INLINE_FMA3(flags)          CPUEXT_SUFFIX(flags, _INLINE, FMA3)
#define INLINE_FMA4(flags)          CPUEXT_SUFFIX(flags, _INLINE, FMA4)
#define INLINE_AVX2(flags)          CPUEXT_SUFFIX(flags, _INLINE, AVX2)
#define INLINE_AESNI(flags)         CPUEXT_SUFFIX(flags, _INLINE, AESNI)

void ff_cpu_cpuid(int index, int *eax, int *ebx, int *ecx, int *edx);
void ff_cpu_xgetbv(int op, int *eax, int *edx);
int  ff_cpu_cpuid_test(void);

#endif /* AVUTIL_X86_CPU_H */
