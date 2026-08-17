/*
 * Copyright (c) 2002 Fabrice Bellard
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

#ifndef AVUTIL_MEM_INTERNAL_H
#define AVUTIL_MEM_INTERNAL_H

#include "config.h"

#include <stdint.h>
#ifndef _MSC_VER
#include <stdalign.h>
#endif

#include "attributes.h"
#include "macros.h"

/**
 * @def DECLARE_ALIGNED(n,t,v)
 * Declare a variable that is aligned in memory.
 *
 * @code{.c}
 * DECLARE_ALIGNED(16, uint16_t, aligned_int) = 42;
 * DECLARE_ALIGNED(32, uint8_t, aligned_array)[128];
 *
 * // The default-alignment equivalent would be
 * uint16_t aligned_int = 42;
 * uint8_t aligned_array[128];
 * @endcode
 *
 * @param n Minimum alignment in bytes
 * @param t Type of the variable (or array element)
 * @param v Name of the variable
 */

/**
 * @def DECLARE_ASM_ALIGNED(n,t,v)
 * Declare an aligned variable appropriate for use in inline assembly code.
 *
 * @code{.c}
 * DECLARE_ASM_ALIGNED(16, uint64_t, pw_08) = UINT64_C(0x0008000800080008);
 * @endcode
 *
 * @param n Minimum alignment in bytes
 * @param t Type of the variable (or array element)
 * @param v Name of the variable
 */

/**
 * @def DECLARE_ASM_CONST(n,t,v)
 * Declare a static constant aligned variable appropriate for use in inline
 * assembly code.
 *
 * @code{.c}
 * DECLARE_ASM_CONST(16, uint64_t, pw_08) = UINT64_C(0x0008000800080008);
 * @endcode
 *
 * @param n Minimum alignment in bytes
 * @param t Type of the variable (or array element)
 * @param v Name of the variable
 */

#if defined(__DJGPP__)
    #define DECLARE_ALIGNED_T(n,t,v)    alignas(FFMIN(n, 16)) t v
    #define DECLARE_ASM_ALIGNED(n,t,v)  alignas(FFMIN(n, 16)) t av_used v
    #define DECLARE_ASM_CONST(n,t,v)    alignas(FFMIN(n, 16)) static const t av_used v
#elif defined(_MSC_VER)
    #define DECLARE_ALIGNED_T(n,t,v)    __declspec(align(n)) t v
    #define DECLARE_ASM_ALIGNED(n,t,v)  __declspec(align(n)) t v
    #define DECLARE_ASM_CONST(n,t,v)    __declspec(align(n)) static const t v
#else
    #define DECLARE_ALIGNED_T(n,t,v)    alignas(n) t v
    #define DECLARE_ASM_ALIGNED(n,t,v)  alignas(n) t av_used v
    #define DECLARE_ASM_CONST(n,t,v)    alignas(n) static const t av_used v
#endif

#if HAVE_SIMD_ALIGN_64
    #define ALIGN_64 64
    #define ALIGN_32 32
#elif HAVE_SIMD_ALIGN_32
    #define ALIGN_64 32
    #define ALIGN_32 32
#else
    #define ALIGN_64 16
    #define ALIGN_32 16
#endif

#define DECLARE_ALIGNED(n,t,v) DECLARE_ALIGNED_V(n,t,v)

// Macro needs to be double-wrapped in order to expand
// possible other macros being passed for n.
#define DECLARE_ALIGNED_V(n,t,v) DECLARE_ALIGNED_##n(t,v)

#define DECLARE_ALIGNED_4(t,v)  DECLARE_ALIGNED_T(       4, t, v)
#define DECLARE_ALIGNED_8(t,v)  DECLARE_ALIGNED_T(       8, t, v)
#define DECLARE_ALIGNED_16(t,v) DECLARE_ALIGNED_T(      16, t, v)
#define DECLARE_ALIGNED_32(t,v) DECLARE_ALIGNED_T(ALIGN_32, t, v)
#define DECLARE_ALIGNED_64(t,v) DECLARE_ALIGNED_T(ALIGN_64, t, v)

// Some broken preprocessors need a second expansion
// to be forced to tokenize __VA_ARGS__
#define E1(x) x

#define LOCAL_ALIGNED_D(a, t, v, s, o, ...)             \
    DECLARE_ALIGNED(a, t, la_##v) s o;                  \
    t (*v) o = la_##v

#define LOCAL_ALIGNED(a, t, v, ...) LOCAL_ALIGNED_##a(t, v, __VA_ARGS__)

#define LOCAL_ALIGNED_4(t, v, ...) E1(LOCAL_ALIGNED_D(4, t, v, __VA_ARGS__,,))

#define LOCAL_ALIGNED_8(t, v, ...) E1(LOCAL_ALIGNED_D(8, t, v, __VA_ARGS__,,))

#define LOCAL_ALIGNED_16(t, v, ...) E1(LOCAL_ALIGNED_D(16, t, v, __VA_ARGS__,,))

#define LOCAL_ALIGNED_32(t, v, ...) E1(LOCAL_ALIGNED_D(32, t, v, __VA_ARGS__,,))

#endif /* AVUTIL_MEM_INTERNAL_H */
