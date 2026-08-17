/*===---- avx10_2copyintrin.h - AVX10.2 Copy intrinsics -------------------===
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */
#ifndef __IMMINTRIN_H
#error                                                                         \
    "Never use <avx10_2copyintrin.h> directly; include <immintrin.h> instead."
#endif // __IMMINTRIN_H

#ifndef __AVX10_2COPYINTRIN_H
#define __AVX10_2COPYINTRIN_H

/* Define the default attributes for the functions in this file. */
#define __DEFAULT_FN_ATTRS128                                                  \
  __attribute__((__always_inline__, __nodebug__, __target__("avx10.2-256"),    \
                 __min_vector_width__(128)))

/// Constructs a 128-bit integer vector, setting the lower 32 bits to the
///    lower 32 bits of the parameter \a __A; the upper bits are zeoroed.
///
/// \code{.operation}
/// result[31:0] := __A[31:0]
/// result[MAX:32] := 0
/// \endcode
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> VMOVD </c> instruction.
///
/// \param __A
///    A 128-bit integer vector.
/// \returns A 128-bit integer vector. The lower 32 bits are copied from the
///    parameter \a __A; the upper bits are zeroed.
static __inline__ __m128i __DEFAULT_FN_ATTRS128 _mm_move_epi32(__m128i __A) {
  return (__m128i)__builtin_shufflevector(
      (__v4si)__A, (__v4si)_mm_setzero_si128(), 0, 4, 4, 4);
}

/// Constructs a 128-bit integer vector, setting the lower 16 bits to the
///    lower 16 bits of the parameter \a __A; the upper bits are zeoroed.
///
/// \code{.operation}
/// result[15:0] := __A[15:0]
/// result[MAX:16] := 0
/// \endcode
///
/// \headerfile <immintrin.h>
///
/// This intrinsic corresponds to the <c> VMOVW </c> instruction.
///
/// \param __A
///    A 128-bit integer vector.
/// \returns A 128-bit integer vector. The lower 16 bits are copied from the
///    parameter \a __A; the upper bits are zeroed.
static __inline__ __m128i __DEFAULT_FN_ATTRS128 _mm_move_epi16(__m128i __A) {
  return (__m128i)__builtin_shufflevector(
      (__v8hi)__A, (__v8hi)_mm_setzero_si128(), 0, 8, 8, 8, 8, 8, 8, 8);
}

#undef __DEFAULT_FN_ATTRS128

#endif // __AVX10_2COPYINTRIN_H
