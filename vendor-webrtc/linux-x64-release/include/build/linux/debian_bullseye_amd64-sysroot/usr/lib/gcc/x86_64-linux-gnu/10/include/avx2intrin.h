/* Copyright (C) 2011-2020 Free Software Foundation, Inc.

   This file is part of GCC.

   GCC is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   GCC is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   Under Section 7 of GPL version 3, you are granted additional
   permissions described in the GCC Runtime Library Exception, version
   3.1, as published by the Free Software Foundation.

   You should have received a copy of the GNU General Public License and
   a copy of the GCC Runtime Library Exception along with this program;
   see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
   <http://www.gnu.org/licenses/>.  */

#ifndef _IMMINTRIN_H_INCLUDED
# error "Never use <avx2intrin.h> directly; include <immintrin.h> instead."
#endif

#ifndef _AVX2INTRIN_H_INCLUDED
#define _AVX2INTRIN_H_INCLUDED

#ifndef __AVX2__
#pragma GCC push_options
#pragma GCC target("avx2")
#define __DISABLE_AVX2__
#endif /* __AVX2__ */

/* Sum absolute 8-bit integer difference of adjacent groups of 4
   byte integers in the first 2 operands.  Starting offsets within
   operands are determined by the 3rd mask operand.  */
#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mpsadbw_epu8 (__m256i __X, __m256i __Y, const int __M)
{
  return (__m256i) __builtin_ia32_mpsadbw256 ((__v32qi)__X,
					      (__v32qi)__Y, __M);
}
#else
#define _mm256_mpsadbw_epu8(X, Y, M)					\
  ((__m256i) __builtin_ia32_mpsadbw256 ((__v32qi)(__m256i)(X),		\
					(__v32qi)(__m256i)(Y), (int)(M)))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_abs_epi8 (__m256i __A)
{
  return (__m256i)__builtin_ia32_pabsb256 ((__v32qi)__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_abs_epi16 (__m256i __A)
{
  return (__m256i)__builtin_ia32_pabsw256 ((__v16hi)__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_abs_epi32 (__m256i __A)
{
  return (__m256i)__builtin_ia32_pabsd256 ((__v8si)__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_packs_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_packssdw256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_packs_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_packsswb256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_packus_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_packusdw256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_packus_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_packuswb256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_add_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v32qu)__A + (__v32qu)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_add_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v16hu)__A + (__v16hu)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_add_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v8su)__A + (__v8su)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_add_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du)__A + (__v4du)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_adds_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_paddsb256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_adds_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_paddsw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_adds_epu8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_paddusb256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_adds_epu16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_paddusw256 ((__v16hi)__A, (__v16hi)__B);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_alignr_epi8 (__m256i __A, __m256i __B, const int __N)
{
  return (__m256i) __builtin_ia32_palignr256 ((__v4di)__A,
					      (__v4di)__B,
					      __N * 8);
}
#else
/* In that case (__N*8) will be in vreg, and insn will not be matched. */
/* Use define instead */
#define _mm256_alignr_epi8(A, B, N)				   \
  ((__m256i) __builtin_ia32_palignr256 ((__v4di)(__m256i)(A),	   \
					(__v4di)(__m256i)(B),	   \
					(int)(N) * 8))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_and_si256 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du)__A & (__v4du)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_andnot_si256 (__m256i __A, __m256i __B)
{
  return (__m256i) __builtin_ia32_andnotsi256 ((__v4di)__A, (__v4di)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_avg_epu8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pavgb256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_avg_epu16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pavgw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_blendv_epi8 (__m256i __X, __m256i __Y, __m256i __M)
{
  return (__m256i) __builtin_ia32_pblendvb256 ((__v32qi)__X,
					       (__v32qi)__Y,
					       (__v32qi)__M);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_blend_epi16 (__m256i __X, __m256i __Y, const int __M)
{
  return (__m256i) __builtin_ia32_pblendw256 ((__v16hi)__X,
					      (__v16hi)__Y,
					       __M);
}
#else
#define _mm256_blend_epi16(X, Y, M)					\
  ((__m256i) __builtin_ia32_pblendw256 ((__v16hi)(__m256i)(X),		\
					(__v16hi)(__m256i)(Y), (int)(M)))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v32qi)__A == (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v16hi)__A == (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v8si)__A == (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpeq_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4di)__A == (__v4di)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v32qs)__A > (__v32qs)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v16hi)__A > (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v8si)__A > (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmpgt_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4di)__A > (__v4di)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hadd_epi16 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_phaddw256 ((__v16hi)__X,
					     (__v16hi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hadd_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_phaddd256 ((__v8si)__X, (__v8si)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hadds_epi16 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_phaddsw256 ((__v16hi)__X,
					      (__v16hi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hsub_epi16 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_phsubw256 ((__v16hi)__X,
					     (__v16hi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hsub_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_phsubd256 ((__v8si)__X, (__v8si)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_hsubs_epi16 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_phsubsw256 ((__v16hi)__X,
					      (__v16hi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maddubs_epi16 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmaddubsw256 ((__v32qi)__X,
						(__v32qi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_madd_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmaddwd256 ((__v16hi)__A,
					     (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmaxsb256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmaxsw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmaxsd256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epu8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmaxub256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epu16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmaxuw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_max_epu32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmaxud256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pminsb256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pminsw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pminsd256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epu8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pminub256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epu16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pminuw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_min_epu32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pminud256 ((__v8si)__A, (__v8si)__B);
}

extern __inline int
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_movemask_epi8 (__m256i __A)
{
  return __builtin_ia32_pmovmskb256 ((__v32qi)__A);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi8_epi16 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxbw256 ((__v16qi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi8_epi32 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxbd256 ((__v16qi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi8_epi64 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxbq256 ((__v16qi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi16_epi32 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxwd256 ((__v8hi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi16_epi64 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxwq256 ((__v8hi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepi32_epi64 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovsxdq256 ((__v4si)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu8_epi16 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxbw256 ((__v16qi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu8_epi32 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxbd256 ((__v16qi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu8_epi64 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxbq256 ((__v16qi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu16_epi32 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxwd256 ((__v8hi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu16_epi64 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxwq256 ((__v8hi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cvtepu32_epi64 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pmovzxdq256 ((__v4si)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mul_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmuldq256 ((__v8si)__X, (__v8si)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mulhrs_epi16 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pmulhrsw256 ((__v16hi)__X,
					       (__v16hi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mulhi_epu16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmulhuw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mulhi_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmulhw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mullo_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v16hu)__A * (__v16hu)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mullo_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v8su)__A * (__v8su)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mul_epu32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_pmuludq256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_or_si256 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du)__A | (__v4du)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sad_epu8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_psadbw256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_epi8 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_pshufb256 ((__v32qi)__X,
					     (__v32qi)__Y);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shuffle_epi32 (__m256i __A, const int __mask)
{
  return (__m256i)__builtin_ia32_pshufd256 ((__v8si)__A, __mask);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shufflehi_epi16 (__m256i __A, const int __mask)
{
  return (__m256i)__builtin_ia32_pshufhw256 ((__v16hi)__A, __mask);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_shufflelo_epi16 (__m256i __A, const int __mask)
{
  return (__m256i)__builtin_ia32_pshuflw256 ((__v16hi)__A, __mask);
}
#else
#define _mm256_shuffle_epi32(A, N) \
  ((__m256i)__builtin_ia32_pshufd256 ((__v8si)(__m256i)(A), (int)(N)))
#define _mm256_shufflehi_epi16(A, N) \
  ((__m256i)__builtin_ia32_pshufhw256 ((__v16hi)(__m256i)(A), (int)(N)))
#define _mm256_shufflelo_epi16(A, N) \
  ((__m256i)__builtin_ia32_pshuflw256 ((__v16hi)(__m256i)(A), (int)(N)))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sign_epi8 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psignb256 ((__v32qi)__X, (__v32qi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sign_epi16 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psignw256 ((__v16hi)__X, (__v16hi)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sign_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psignd256 ((__v8si)__X, (__v8si)__Y);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_bslli_epi128 (__m256i __A, const int __N)
{
  return (__m256i)__builtin_ia32_pslldqi256 (__A, __N * 8);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_slli_si256 (__m256i __A, const int __N)
{
  return (__m256i)__builtin_ia32_pslldqi256 (__A, __N * 8);
}
#else
#define _mm256_bslli_epi128(A, N) \
  ((__m256i)__builtin_ia32_pslldqi256 ((__m256i)(A), (int)(N) * 8))
#define _mm256_slli_si256(A, N) \
  ((__m256i)__builtin_ia32_pslldqi256 ((__m256i)(A), (int)(N) * 8))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_slli_epi16 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_psllwi256 ((__v16hi)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sll_epi16 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_psllw256((__v16hi)__A, (__v8hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_slli_epi32 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_pslldi256 ((__v8si)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sll_epi32 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_pslld256((__v8si)__A, (__v4si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_slli_epi64 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_psllqi256 ((__v4di)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sll_epi64 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_psllq256((__v4di)__A, (__v2di)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srai_epi16 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_psrawi256 ((__v16hi)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sra_epi16 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_psraw256 ((__v16hi)__A, (__v8hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srai_epi32 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_psradi256 ((__v8si)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sra_epi32 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_psrad256 ((__v8si)__A, (__v4si)__B);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_bsrli_epi128 (__m256i __A, const int __N)
{
  return (__m256i)__builtin_ia32_psrldqi256 (__A, __N * 8);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srli_si256 (__m256i __A, const int __N)
{
  return (__m256i)__builtin_ia32_psrldqi256 (__A, __N * 8);
}
#else
#define _mm256_bsrli_epi128(A, N) \
  ((__m256i)__builtin_ia32_psrldqi256 ((__m256i)(A), (int)(N) * 8))
#define _mm256_srli_si256(A, N) \
  ((__m256i)__builtin_ia32_psrldqi256 ((__m256i)(A), (int)(N) * 8))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srli_epi16 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_psrlwi256 ((__v16hi)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srl_epi16 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_psrlw256((__v16hi)__A, (__v8hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srli_epi32 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_psrldi256 ((__v8si)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srl_epi32 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_psrld256((__v8si)__A, (__v4si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srli_epi64 (__m256i __A, int __B)
{
  return (__m256i)__builtin_ia32_psrlqi256 ((__v4di)__A, __B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srl_epi64 (__m256i __A, __m128i __B)
{
  return (__m256i)__builtin_ia32_psrlq256((__v4di)__A, (__v2di)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sub_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v32qu)__A - (__v32qu)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sub_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v16hu)__A - (__v16hu)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sub_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v8su)__A - (__v8su)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sub_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du)__A - (__v4du)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_subs_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_psubsb256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_subs_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_psubsw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_subs_epu8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_psubusb256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_subs_epu16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_psubusw256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpackhi_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpckhbw256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpackhi_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpckhwd256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpackhi_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpckhdq256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpackhi_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpckhqdq256 ((__v4di)__A, (__v4di)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpacklo_epi8 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpcklbw256 ((__v32qi)__A, (__v32qi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpacklo_epi16 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpcklwd256 ((__v16hi)__A, (__v16hi)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpacklo_epi32 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpckldq256 ((__v8si)__A, (__v8si)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_unpacklo_epi64 (__m256i __A, __m256i __B)
{
  return (__m256i)__builtin_ia32_punpcklqdq256 ((__v4di)__A, (__v4di)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_xor_si256 (__m256i __A, __m256i __B)
{
  return (__m256i) ((__v4du)__A ^ (__v4du)__B);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_stream_load_si256 (__m256i const *__X)
{
  return (__m256i) __builtin_ia32_movntdqa256 ((__v4di *) __X);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcastss_ps (__m128 __X)
{
  return (__m128) __builtin_ia32_vbroadcastss_ps ((__v4sf)__X);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastss_ps (__m128 __X)
{
  return (__m256) __builtin_ia32_vbroadcastss_ps256 ((__v4sf)__X);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastsd_pd (__m128d __X)
{
  return (__m256d) __builtin_ia32_vbroadcastsd_pd256 ((__v2df)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastsi128_si256 (__m128i __X)
{
  return (__m256i) __builtin_ia32_vbroadcastsi256 ((__v2di)__X);
}

#ifdef __OPTIMIZE__
extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_blend_epi32 (__m128i __X, __m128i __Y, const int __M)
{
  return (__m128i) __builtin_ia32_pblendd128 ((__v4si)__X,
					      (__v4si)__Y,
					      __M);
}
#else
#define _mm_blend_epi32(X, Y, M)					\
  ((__m128i) __builtin_ia32_pblendd128 ((__v4si)(__m128i)(X),		\
					(__v4si)(__m128i)(Y), (int)(M)))
#endif

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_blend_epi32 (__m256i __X, __m256i __Y, const int __M)
{
  return (__m256i) __builtin_ia32_pblendd256 ((__v8si)__X,
					      (__v8si)__Y,
					      __M);
}
#else
#define _mm256_blend_epi32(X, Y, M)					\
  ((__m256i) __builtin_ia32_pblendd256 ((__v8si)(__m256i)(X),		\
					(__v8si)(__m256i)(Y), (int)(M)))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastb_epi8 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pbroadcastb256 ((__v16qi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastw_epi16 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pbroadcastw256 ((__v8hi)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastd_epi32 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pbroadcastd256 ((__v4si)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_broadcastq_epi64 (__m128i __X)
{
  return (__m256i) __builtin_ia32_pbroadcastq256 ((__v2di)__X);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcastb_epi8 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pbroadcastb128 ((__v16qi)__X);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcastw_epi16 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pbroadcastw128 ((__v8hi)__X);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcastd_epi32 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pbroadcastd128 ((__v4si)__X);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_broadcastq_epi64 (__m128i __X)
{
  return (__m128i) __builtin_ia32_pbroadcastq128 ((__v2di)__X);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutevar8x32_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_permvarsi256 ((__v8si)__X, (__v8si)__Y);
}

#ifdef __OPTIMIZE__
extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute4x64_pd (__m256d __X, const int __M)
{
  return (__m256d) __builtin_ia32_permdf256 ((__v4df)__X, __M);
}
#else
#define _mm256_permute4x64_pd(X, M)			       \
  ((__m256d) __builtin_ia32_permdf256 ((__v4df)(__m256d)(X), (int)(M)))
#endif

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permutevar8x32_ps (__m256 __X, __m256i __Y)
{
  return (__m256) __builtin_ia32_permvarsf256 ((__v8sf)__X, (__v8si)__Y);
}

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute4x64_epi64 (__m256i __X, const int __M)
{
  return (__m256i) __builtin_ia32_permdi256 ((__v4di)__X, __M);
}
#else
#define _mm256_permute4x64_epi64(X, M)			       \
  ((__m256i) __builtin_ia32_permdi256 ((__v4di)(__m256i)(X), (int)(M)))
#endif


#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute2x128_si256 (__m256i __X, __m256i __Y, const int __M)
{
  return (__m256i) __builtin_ia32_permti256 ((__v4di)__X, (__v4di)__Y, __M);
}
#else
#define _mm256_permute2x128_si256(X, Y, M)				\
  ((__m256i) __builtin_ia32_permti256 ((__v4di)(__m256i)(X), (__v4di)(__m256i)(Y), (int)(M)))
#endif

#ifdef __OPTIMIZE__
extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_extracti128_si256 (__m256i __X, const int __M)
{
  return (__m128i) __builtin_ia32_extract128i256 ((__v4di)__X, __M);
}
#else
#define _mm256_extracti128_si256(X, M)				\
  ((__m128i) __builtin_ia32_extract128i256 ((__v4di)(__m256i)(X), (int)(M)))
#endif

#ifdef __OPTIMIZE__
extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_inserti128_si256 (__m256i __X, __m128i __Y, const int __M)
{
  return (__m256i) __builtin_ia32_insert128i256 ((__v4di)__X, (__v2di)__Y, __M);
}
#else
#define _mm256_inserti128_si256(X, Y, M)			 \
  ((__m256i) __builtin_ia32_insert128i256 ((__v4di)(__m256i)(X), \
					   (__v2di)(__m128i)(Y), \
					   (int)(M)))
#endif

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskload_epi32 (int const *__X, __m256i __M )
{
  return (__m256i) __builtin_ia32_maskloadd256 ((const __v8si *)__X,
						(__v8si)__M);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskload_epi64 (long long const *__X, __m256i __M )
{
  return (__m256i) __builtin_ia32_maskloadq256 ((const __v4di *)__X,
						(__v4di)__M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskload_epi32 (int const *__X, __m128i __M )
{
  return (__m128i) __builtin_ia32_maskloadd ((const __v4si *)__X,
					     (__v4si)__M);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskload_epi64 (long long const *__X, __m128i __M )
{
  return (__m128i) __builtin_ia32_maskloadq ((const __v2di *)__X,
					     (__v2di)__M);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskstore_epi32 (int *__X, __m256i __M, __m256i __Y )
{
  __builtin_ia32_maskstored256 ((__v8si *)__X, (__v8si)__M, (__v8si)__Y);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_maskstore_epi64 (long long *__X, __m256i __M, __m256i __Y )
{
  __builtin_ia32_maskstoreq256 ((__v4di *)__X, (__v4di)__M, (__v4di)__Y);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskstore_epi32 (int *__X, __m128i __M, __m128i __Y )
{
  __builtin_ia32_maskstored ((__v4si *)__X, (__v4si)__M, (__v4si)__Y);
}

extern __inline void
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_maskstore_epi64 (long long *__X, __m128i __M, __m128i __Y )
{
  __builtin_ia32_maskstoreq (( __v2di *)__X, (__v2di)__M, (__v2di)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sllv_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psllv8si ((__v8si)__X, (__v8si)__Y);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sllv_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psllv4si ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_sllv_epi64 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psllv4di ((__v4di)__X, (__v4di)__Y);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_sllv_epi64 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psllv2di ((__v2di)__X, (__v2di)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srav_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psrav8si ((__v8si)__X, (__v8si)__Y);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_srav_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrav4si ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srlv_epi32 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psrlv8si ((__v8si)__X, (__v8si)__Y);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_srlv_epi32 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrlv4si ((__v4si)__X, (__v4si)__Y);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_srlv_epi64 (__m256i __X, __m256i __Y)
{
  return (__m256i) __builtin_ia32_psrlv4di ((__v4di)__X, (__v4di)__Y);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_srlv_epi64 (__m128i __X, __m128i __Y)
{
  return (__m128i) __builtin_ia32_psrlv2di ((__v2di)__X, (__v2di)__Y);
}

#ifdef __OPTIMIZE__
extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32gather_pd (double const *__base, __m128i __index, const int __scale)
{
  __v2df __zero = _mm_setzero_pd ();
  __v2df __mask = _mm_cmpeq_pd (__zero, __zero);

  return (__m128d) __builtin_ia32_gathersiv2df (_mm_undefined_pd (),
						__base,
						(__v4si)__index,
						__mask,
						__scale);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32gather_pd (__m128d __src, double const *__base, __m128i __index,
		       __m128d __mask, const int __scale)
{
  return (__m128d) __builtin_ia32_gathersiv2df ((__v2df)__src,
						__base,
						(__v4si)__index,
						(__v2df)__mask,
						__scale);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32gather_pd (double const *__base, __m128i __index, const int __scale)
{
  __v4df __zero = _mm256_setzero_pd ();
  __v4df __mask = _mm256_cmp_pd (__zero, __zero, _CMP_EQ_OQ);

  return (__m256d) __builtin_ia32_gathersiv4df (_mm256_undefined_pd (),
						__base,
						(__v4si)__index,
						__mask,
						__scale);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32gather_pd (__m256d __src, double const *__base,
			  __m128i __index, __m256d __mask, const int __scale)
{
  return (__m256d) __builtin_ia32_gathersiv4df ((__v4df)__src,
						__base,
						(__v4si)__index,
						(__v4df)__mask,
						__scale);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64gather_pd (double const *__base, __m128i __index, const int __scale)
{
  __v2df __src = _mm_setzero_pd ();
  __v2df __mask = _mm_cmpeq_pd (__src, __src);

  return (__m128d) __builtin_ia32_gatherdiv2df (__src,
						__base,
						(__v2di)__index,
						__mask,
						__scale);
}

extern __inline __m128d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64gather_pd (__m128d __src, double const *__base, __m128i __index,
		       __m128d __mask, const int __scale)
{
  return (__m128d) __builtin_ia32_gatherdiv2df ((__v2df)__src,
						__base,
						(__v2di)__index,
						(__v2df)__mask,
						__scale);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64gather_pd (double const *__base, __m256i __index, const int __scale)
{
  __v4df __src = _mm256_setzero_pd ();
  __v4df __mask = _mm256_cmp_pd (__src, __src, _CMP_EQ_OQ);

  return (__m256d) __builtin_ia32_gatherdiv4df (__src,
						__base,
						(__v4di)__index,
						__mask,
						__scale);
}

extern __inline __m256d
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64gather_pd (__m256d __src, double const *__base,
			  __m256i __index, __m256d __mask, const int __scale)
{
  return (__m256d) __builtin_ia32_gatherdiv4df ((__v4df)__src,
						__base,
						(__v4di)__index,
						(__v4df)__mask,
						__scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32gather_ps (float const *__base, __m128i __index, const int __scale)
{
  __v4sf __src = _mm_setzero_ps ();
  __v4sf __mask = _mm_cmpeq_ps (__src, __src);

  return (__m128) __builtin_ia32_gathersiv4sf (__src,
					       __base,
					       (__v4si)__index,
					       __mask,
					       __scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32gather_ps (__m128 __src, float const *__base, __m128i __index,
		       __m128 __mask, const int __scale)
{
  return (__m128) __builtin_ia32_gathersiv4sf ((__v4sf)__src,
					       __base,
					       (__v4si)__index,
					       (__v4sf)__mask,
					       __scale);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32gather_ps (float const *__base, __m256i __index, const int __scale)
{
  __v8sf __src = _mm256_setzero_ps ();
  __v8sf __mask = _mm256_cmp_ps (__src, __src, _CMP_EQ_OQ);

  return (__m256) __builtin_ia32_gathersiv8sf (__src,
					       __base,
					       (__v8si)__index,
					       __mask,
					       __scale);
}

extern __inline __m256
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32gather_ps (__m256 __src, float const *__base,
			  __m256i __index, __m256 __mask, const int __scale)
{
  return (__m256) __builtin_ia32_gathersiv8sf ((__v8sf)__src,
					       __base,
					       (__v8si)__index,
					       (__v8sf)__mask,
					       __scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64gather_ps (float const *__base, __m128i __index, const int __scale)
{
  __v4sf __src = _mm_setzero_ps ();
  __v4sf __mask = _mm_cmpeq_ps (__src, __src);

  return (__m128) __builtin_ia32_gatherdiv4sf (__src,
					       __base,
					       (__v2di)__index,
					       __mask,
					       __scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64gather_ps (__m128 __src, float const *__base, __m128i __index,
		       __m128 __mask, const int __scale)
{
  return (__m128) __builtin_ia32_gatherdiv4sf ((__v4sf)__src,
						__base,
						(__v2di)__index,
						(__v4sf)__mask,
						__scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64gather_ps (float const *__base, __m256i __index, const int __scale)
{
  __v4sf __src = _mm_setzero_ps ();
  __v4sf __mask = _mm_cmpeq_ps (__src, __src);

  return (__m128) __builtin_ia32_gatherdiv4sf256 (__src,
						  __base,
						  (__v4di)__index,
						  __mask,
						  __scale);
}

extern __inline __m128
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64gather_ps (__m128 __src, float const *__base,
			  __m256i __index, __m128 __mask, const int __scale)
{
  return (__m128) __builtin_ia32_gatherdiv4sf256 ((__v4sf)__src,
						  __base,
						  (__v4di)__index,
						  (__v4sf)__mask,
						  __scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32gather_epi64 (long long int const *__base,
		     __m128i __index, const int __scale)
{
  __v2di __src = __extension__ (__v2di){ 0, 0 };
  __v2di __mask = __extension__ (__v2di){ ~0, ~0 };

  return (__m128i) __builtin_ia32_gathersiv2di (__src,
						__base,
						(__v4si)__index,
						__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32gather_epi64 (__m128i __src, long long int const *__base,
			  __m128i __index, __m128i __mask, const int __scale)
{
  return (__m128i) __builtin_ia32_gathersiv2di ((__v2di)__src,
						__base,
						(__v4si)__index,
						(__v2di)__mask,
						__scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32gather_epi64 (long long int const *__base,
			__m128i __index, const int __scale)
{
  __v4di __src = __extension__ (__v4di){ 0, 0, 0, 0 };
  __v4di __mask = __extension__ (__v4di){ ~0, ~0, ~0, ~0 };

  return (__m256i) __builtin_ia32_gathersiv4di (__src,
						__base,
						(__v4si)__index,
						__mask,
						__scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32gather_epi64 (__m256i __src, long long int const *__base,
			     __m128i __index, __m256i __mask,
			     const int __scale)
{
  return (__m256i) __builtin_ia32_gathersiv4di ((__v4di)__src,
						__base,
						(__v4si)__index,
						(__v4di)__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64gather_epi64 (long long int const *__base,
		     __m128i __index, const int __scale)
{
  __v2di __src = __extension__ (__v2di){ 0, 0 };
  __v2di __mask = __extension__ (__v2di){ ~0, ~0 };

  return (__m128i) __builtin_ia32_gatherdiv2di (__src,
						__base,
						(__v2di)__index,
						__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64gather_epi64 (__m128i __src, long long int const *__base,
			  __m128i __index, __m128i __mask, const int __scale)
{
  return (__m128i) __builtin_ia32_gatherdiv2di ((__v2di)__src,
						__base,
						(__v2di)__index,
						(__v2di)__mask,
						__scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64gather_epi64 (long long int const *__base,
			__m256i __index, const int __scale)
{
  __v4di __src = __extension__ (__v4di){ 0, 0, 0, 0 };
  __v4di __mask = __extension__ (__v4di){ ~0, ~0, ~0, ~0 };

  return (__m256i) __builtin_ia32_gatherdiv4di (__src,
						__base,
						(__v4di)__index,
						__mask,
						__scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64gather_epi64 (__m256i __src, long long int const *__base,
			     __m256i __index, __m256i __mask,
			     const int __scale)
{
  return (__m256i) __builtin_ia32_gatherdiv4di ((__v4di)__src,
						__base,
						(__v4di)__index,
						(__v4di)__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i32gather_epi32 (int const *__base, __m128i __index, const int __scale)
{
  __v4si __src = __extension__ (__v4si){ 0, 0, 0, 0 };
  __v4si __mask = __extension__ (__v4si){ ~0, ~0, ~0, ~0 };

  return (__m128i) __builtin_ia32_gathersiv4si (__src,
						__base,
						(__v4si)__index,
						__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i32gather_epi32 (__m128i __src, int const *__base, __m128i __index,
			  __m128i __mask, const int __scale)
{
  return (__m128i) __builtin_ia32_gathersiv4si ((__v4si)__src,
						__base,
						(__v4si)__index,
						(__v4si)__mask,
						__scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i32gather_epi32 (int const *__base, __m256i __index, const int __scale)
{
  __v8si __src = __extension__ (__v8si){ 0, 0, 0, 0, 0, 0, 0, 0 };
  __v8si __mask = __extension__ (__v8si){ ~0, ~0, ~0, ~0, ~0, ~0, ~0, ~0 };

  return (__m256i) __builtin_ia32_gathersiv8si (__src,
						__base,
						(__v8si)__index,
						__mask,
						__scale);
}

extern __inline __m256i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i32gather_epi32 (__m256i __src, int const *__base,
			     __m256i __index, __m256i __mask,
			     const int __scale)
{
  return (__m256i) __builtin_ia32_gathersiv8si ((__v8si)__src,
						__base,
						(__v8si)__index,
						(__v8si)__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_i64gather_epi32 (int const *__base, __m128i __index, const int __scale)
{
  __v4si __src = __extension__ (__v4si){ 0, 0, 0, 0 };
  __v4si __mask = __extension__ (__v4si){ ~0, ~0, ~0, ~0 };

  return (__m128i) __builtin_ia32_gatherdiv4si (__src,
						__base,
						(__v2di)__index,
						__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm_mask_i64gather_epi32 (__m128i __src, int const *__base, __m128i __index,
			  __m128i __mask, const int __scale)
{
  return (__m128i) __builtin_ia32_gatherdiv4si ((__v4si)__src,
						__base,
						(__v2di)__index,
						(__v4si)__mask,
						__scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_i64gather_epi32 (int const *__base, __m256i __index, const int __scale)
{
  __v4si __src = __extension__ (__v4si){ 0, 0, 0, 0 };
  __v4si __mask = __extension__ (__v4si){ ~0, ~0, ~0, ~0 };

  return (__m128i) __builtin_ia32_gatherdiv4si256 (__src,
						   __base,
						   (__v4di)__index,
						   __mask,
						   __scale);
}

extern __inline __m128i
__attribute__ ((__gnu_inline__, __always_inline__, __artificial__))
_mm256_mask_i64gather_epi32 (__m128i __src, int const *__base,
			     __m256i __index, __m128i __mask,
			     const int __scale)
{
  return (__m128i) __builtin_ia32_gatherdiv4si256 ((__v4si)__src,
						   __base,
						   (__v4di)__index,
						   (__v4si)__mask,
						   __scale);
}
#else /* __OPTIMIZE__ */
#define _mm_i32gather_pd(BASE, INDEX, SCALE)				\
  (__m128d) __builtin_ia32_gathersiv2df ((__v2df) _mm_setzero_pd (),	\
					 (double const *) (BASE),	\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v2df)			\
					 _mm_cmpeq_pd (_mm_setzero_pd (),\
						       _mm_setzero_pd ()),\
					 (int) (SCALE))

#define _mm_mask_i32gather_pd(SRC, BASE, INDEX, MASK, SCALE)	 	\
  (__m128d) __builtin_ia32_gathersiv2df ((__v2df)(__m128d) (SRC),	\
					 (double const *) (BASE),	\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v2df)(__m128d) (MASK),	\
					 (int) (SCALE))

#define _mm256_i32gather_pd(BASE, INDEX, SCALE)				\
  (__m256d) __builtin_ia32_gathersiv4df ((__v4df) _mm256_setzero_pd (),	\
					 (double const *) (BASE),	\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v4df)			\
					 _mm256_cmp_pd (_mm256_setzero_pd (),\
							_mm256_setzero_pd (),\
							_CMP_EQ_OQ),	\
					 (int) (SCALE))

#define _mm256_mask_i32gather_pd(SRC, BASE, INDEX, MASK, SCALE)		\
  (__m256d) __builtin_ia32_gathersiv4df ((__v4df)(__m256d) (SRC),	\
					 (double const *) (BASE),	\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v4df)(__m256d) (MASK),	\
					 (int) (SCALE))

#define _mm_i64gather_pd(BASE, INDEX, SCALE)				\
  (__m128d) __builtin_ia32_gatherdiv2df ((__v2df) _mm_setzero_pd (),	\
					 (double const *) (BASE),	\
					 (__v2di)(__m128i) (INDEX),	\
					 (__v2df)			\
					 _mm_cmpeq_pd (_mm_setzero_pd (),\
						       _mm_setzero_pd ()),\
					 (int) (SCALE))

#define _mm_mask_i64gather_pd(SRC, BASE, INDEX, MASK, SCALE)		\
  (__m128d) __builtin_ia32_gatherdiv2df ((__v2df)(__m128d) (SRC),	\
					 (double const *) (BASE),	\
					 (__v2di)(__m128i) (INDEX),	\
					 (__v2df)(__m128d) (MASK),	\
					 (int) (SCALE))

#define _mm256_i64gather_pd(BASE, INDEX, SCALE)				\
  (__m256d) __builtin_ia32_gatherdiv4df ((__v4df) _mm256_setzero_pd (),	\
					 (double const *) (BASE),	\
					 (__v4di)(__m256i) (INDEX),	\
					 (__v4df)			\
					 _mm256_cmp_pd (_mm256_setzero_pd (),\
							_mm256_setzero_pd (),\
							_CMP_EQ_OQ),	\
					 (int) (SCALE))

#define _mm256_mask_i64gather_pd(SRC, BASE, INDEX, MASK, SCALE)	 	\
  (__m256d) __builtin_ia32_gatherdiv4df ((__v4df)(__m256d) (SRC),	\
					 (double const *) (BASE),	\
					 (__v4di)(__m256i) (INDEX),	\
					 (__v4df)(__m256d) (MASK),	\
					 (int) (SCALE))

#define _mm_i32gather_ps(BASE, INDEX, SCALE)				\
  (__m128) __builtin_ia32_gathersiv4sf ((__v4sf) _mm_setzero_ps (),	\
					(float const *) (BASE),		\
					(__v4si)(__m128i) (INDEX),	\
					(__v4sf)			\
					_mm_cmpeq_ps (_mm_setzero_ps (),\
						      _mm_setzero_ps ()),\
					(int) (SCALE))

#define _mm_mask_i32gather_ps(SRC, BASE, INDEX, MASK, SCALE)	 	\
  (__m128) __builtin_ia32_gathersiv4sf ((__v4sf)(__m128) (SRC),		\
					(float const *) (BASE),		\
					(__v4si)(__m128i) (INDEX),	\
					(__v4sf)(__m128) (MASK),	\
					(int) (SCALE))

#define _mm256_i32gather_ps(BASE, INDEX, SCALE)				\
  (__m256) __builtin_ia32_gathersiv8sf ((__v8sf) _mm256_setzero_ps (),	\
					(float const *) (BASE),		\
					(__v8si)(__m256i) (INDEX),	\
					(__v8sf)			\
					_mm256_cmp_ps (_mm256_setzero_ps (),\
						       _mm256_setzero_ps (),\
						       _CMP_EQ_OQ),	\
					(int) (SCALE))

#define _mm256_mask_i32gather_ps(SRC, BASE, INDEX, MASK, SCALE)		\
  (__m256) __builtin_ia32_gathersiv8sf ((__v8sf)(__m256) (SRC),		\
					(float const *) (BASE),		\
					(__v8si)(__m256i) (INDEX),	\
					(__v8sf)(__m256) (MASK),	\
					(int) (SCALE))

#define _mm_i64gather_ps(BASE, INDEX, SCALE)				\
  (__m128) __builtin_ia32_gatherdiv4sf ((__v4sf) _mm_setzero_pd (),	\
					(float const *) (BASE),		\
					(__v2di)(__m128i) (INDEX),	\
					(__v4sf)			\
					_mm_cmpeq_ps (_mm_setzero_ps (),\
						      _mm_setzero_ps ()),\
					(int) (SCALE))

#define _mm_mask_i64gather_ps(SRC, BASE, INDEX, MASK, SCALE)		\
  (__m128) __builtin_ia32_gatherdiv4sf ((__v4sf)(__m128) (SRC),		\
					(float const *) (BASE),		\
					(__v2di)(__m128i) (INDEX),	\
					(__v4sf)(__m128) (MASK),	\
					(int) (SCALE))

#define _mm256_i64gather_ps(BASE, INDEX, SCALE)				\
  (__m128) __builtin_ia32_gatherdiv4sf256 ((__v4sf) _mm_setzero_ps (),	\
					   (float const *) (BASE),	\
					   (__v4di)(__m256i) (INDEX),	\
					   (__v4sf)			\
					   _mm_cmpeq_ps (_mm_setzero_ps (),\
							 _mm_setzero_ps ()),\
					   (int) (SCALE))

#define _mm256_mask_i64gather_ps(SRC, BASE, INDEX, MASK, SCALE)	   	\
  (__m128) __builtin_ia32_gatherdiv4sf256 ((__v4sf)(__m128) (SRC),	\
					   (float const *) (BASE),	\
					   (__v4di)(__m256i) (INDEX),	\
					   (__v4sf)(__m128) (MASK),	\
					   (int) (SCALE))

#define _mm_i32gather_epi64(BASE, INDEX, SCALE)				\
  (__m128i) __builtin_ia32_gathersiv2di ((__v2di) _mm_setzero_si128 (), \
					 (long long const *) (BASE),	\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v2di)_mm_set1_epi64x (-1),	\
					 (int) (SCALE))

#define _mm_mask_i32gather_epi64(SRC, BASE, INDEX, MASK, SCALE)	  	\
  (__m128i) __builtin_ia32_gathersiv2di ((__v2di)(__m128i) (SRC),	\
					 (long long const *) (BASE),	\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v2di)(__m128i) (MASK),	\
					 (int) (SCALE))

#define _mm256_i32gather_epi64(BASE, INDEX, SCALE)			   \
  (__m256i) __builtin_ia32_gathersiv4di ((__v4di) _mm256_setzero_si256 (), \
					 (long long const *) (BASE),	   \
					 (__v4si)(__m128i) (INDEX),	   \
					 (__v4di)_mm256_set1_epi64x (-1),  \
					 (int) (SCALE))

#define _mm256_mask_i32gather_epi64(SRC, BASE, INDEX, MASK, SCALE)	\
  (__m256i) __builtin_ia32_gathersiv4di ((__v4di)(__m256i) (SRC),	\
					 (long long const *) (BASE),	\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v4di)(__m256i) (MASK),	\
					 (int) (SCALE))

#define _mm_i64gather_epi64(BASE, INDEX, SCALE)				\
  (__m128i) __builtin_ia32_gatherdiv2di ((__v2di) _mm_setzero_si128 (), \
					 (long long const *) (BASE),	\
					 (__v2di)(__m128i) (INDEX),	\
					 (__v2di)_mm_set1_epi64x (-1),	\
					 (int) (SCALE))

#define _mm_mask_i64gather_epi64(SRC, BASE, INDEX, MASK, SCALE)		\
  (__m128i) __builtin_ia32_gatherdiv2di ((__v2di)(__m128i) (SRC),	\
					 (long long const *) (BASE),	\
					 (__v2di)(__m128i) (INDEX),	\
					 (__v2di)(__m128i) (MASK),	\
					 (int) (SCALE))

#define _mm256_i64gather_epi64(BASE, INDEX, SCALE)			   \
  (__m256i) __builtin_ia32_gatherdiv4di ((__v4di) _mm256_setzero_si256 (), \
					 (long long const *) (BASE),	   \
					 (__v4di)(__m256i) (INDEX),	   \
					 (__v4di)_mm256_set1_epi64x (-1),  \
					 (int) (SCALE))

#define _mm256_mask_i64gather_epi64(SRC, BASE, INDEX, MASK, SCALE) 	\
  (__m256i) __builtin_ia32_gatherdiv4di ((__v4di)(__m256i) (SRC),	\
					 (long long const *) (BASE),	\
					 (__v4di)(__m256i) (INDEX),	\
					 (__v4di)(__m256i) (MASK),	\
					 (int) (SCALE))

#define _mm_i32gather_epi32(BASE, INDEX, SCALE)				\
  (__m128i) __builtin_ia32_gathersiv4si ((__v4si) _mm_setzero_si128 (),	\
					 (int const *) (BASE),		\
					 (__v4si)(__m128i) (INDEX),	\
					 (__v4si)_mm_set1_epi32 (-1),	\
					 (int) (SCALE))

#define _mm_mask_i32gather_epi32(SRC, BASE, INDEX, MASK, SCALE)		\
  (__m128i) __builtin_ia32_gathersiv4si ((__v4si)(__m128i) (SRC),	\
					(int const *) (BASE),		\
					(__v4si)(__m128i) (INDEX),	\
					(__v4si)(__m128i) (MASK),	\
					(int) (SCALE))

#define _mm256_i32gather_epi32(BASE, INDEX, SCALE)			   \
  (__m256i) __builtin_ia32_gathersiv8si ((__v8si) _mm256_setzero_si256 (), \
					 (int const *) (BASE),		   \
					 (__v8si)(__m256i) (INDEX),	   \
					 (__v8si)_mm256_set1_epi32 (-1),   \
					 (int) (SCALE))

#define _mm256_mask_i32gather_epi32(SRC, BASE, INDEX, MASK, SCALE)	\
  (__m256i) __builtin_ia32_gathersiv8si ((__v8si)(__m256i) (SRC),	\
					(int const *) (BASE),	   	\
					(__v8si)(__m256i) (INDEX),	\
					(__v8si)(__m256i) (MASK),	\
					(int) (SCALE))

#define _mm_i64gather_epi32(BASE, INDEX, SCALE)				\
  (__m128i) __builtin_ia32_gatherdiv4si ((__v4si) _mm_setzero_si128 (),	\
					 (int const *) (BASE),		\
					 (__v2di)(__m128i) (INDEX),	\
					 (__v4si)_mm_set1_epi32 (-1),	\
					 (int) (SCALE))

#define _mm_mask_i64gather_epi32(SRC, BASE, INDEX, MASK, SCALE)		\
  (__m128i) __builtin_ia32_gatherdiv4si ((__v4si)(__m128i) (SRC),	\
					(int const *) (BASE),		\
					(__v2di)(__m128i) (INDEX),	\
					(__v4si)(__m128i) (MASK),	\
					(int) (SCALE))

#define _mm256_i64gather_epi32(BASE, INDEX, SCALE)			   \
  (__m128i) __builtin_ia32_gatherdiv4si256 ((__v4si) _mm_setzero_si128 (), \
					    (int const *) (BASE),	   \
					    (__v4di)(__m256i) (INDEX),	   \
					    (__v4si)_mm_set1_epi32(-1),	   \
					    (int) (SCALE))

#define _mm256_mask_i64gather_epi32(SRC, BASE, INDEX, MASK, SCALE)	\
  (__m128i) __builtin_ia32_gatherdiv4si256 ((__v4si)(__m128i) (SRC),	\
					   (int const *) (BASE),	\
					   (__v4di)(__m256i) (INDEX),	\
					   (__v4si)(__m128i) (MASK),	\
					   (int) (SCALE))
#endif  /* __OPTIMIZE__ */

#ifdef __DISABLE_AVX2__
#undef __DISABLE_AVX2__
#pragma GCC pop_options
#endif /* __DISABLE_AVX2__ */

#endif /* _AVX2INTRIN_H_INCLUDED */
