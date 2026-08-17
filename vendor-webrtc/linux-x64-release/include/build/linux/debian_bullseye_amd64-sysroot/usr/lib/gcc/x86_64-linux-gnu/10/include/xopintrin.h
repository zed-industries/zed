/* Copyright (C) 2007-2020 Free Software Foundation, Inc.

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

#ifndef _X86INTRIN_H_INCLUDED
# error "Never use <xopintrin.h> directly; include <x86intrin.h> instead."
#endif

#ifndef _XOPMMINTRIN_H_INCLUDED
#define _XOPMMINTRIN_H_INCLUDED

#include <fma4intrin.h>

#ifndef __XOP__
#pragma GCC push_options
#pragma GCC target("xop")
#define __DISABLE_XOP__
#endif /* __XOP__ */

/* Integer multiply/add instructions. */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maccs_epi16(__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpmacssww ((__v8hi)__A,(__v8hi)__B, (__v8hi)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_macc_epi16(__m128i __A, __m128i __B, __m128i __C)
{
  return (__m128i) __builtin_ia32_vpmacsww ((__v8hi)__A, (__v8hi)__B, (__v8hi)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maccsd_epi16(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacsswd ((__v8hi)__A, (__v8hi)__B, (__v4si)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maccd_epi16(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacswd ((__v8hi)__A, (__v8hi)__B, (__v4si)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maccs_epi32(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacssdd ((__v4si)__A, (__v4si)__B, (__v4si)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_macc_epi32(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacsdd ((__v4si)__A, (__v4si)__B, (__v4si)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maccslo_epi32(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacssdql ((__v4si)__A, (__v4si)__B, (__v2di)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_macclo_epi32(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacsdql ((__v4si)__A, (__v4si)__B, (__v2di)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maccshi_epi32(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacssdqh ((__v4si)__A, (__v4si)__B, (__v2di)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_macchi_epi32(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmacsdqh ((__v4si)__A, (__v4si)__B, (__v2di)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maddsd_epi16(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmadcsswd ((__v8hi)__A,(__v8hi)__B,(__v4si)__C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_maddd_epi16(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpmadcswd ((__v8hi)__A,(__v8hi)__B,(__v4si)__C);
}

/* Packed Integer Horizontal Add and Subtract */
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddw_epi8(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddbw ((__v16qi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddd_epi8(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddbd ((__v16qi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddq_epi8(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddbq ((__v16qi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddd_epi16(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddwd ((__v8hi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddq_epi16(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddwq ((__v8hi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddq_epi32(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphadddq ((__v4si)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddw_epu8(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddubw ((__v16qi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddd_epu8(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddubd ((__v16qi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddq_epu8(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddubq ((__v16qi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddd_epu16(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphadduwd ((__v8hi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddq_epu16(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphadduwq ((__v8hi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_haddq_epu32(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphaddudq ((__v4si)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsubw_epi8(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphsubbw ((__v16qi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsubd_epi16(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphsubwd ((__v8hi)__A);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_hsubq_epi32(__m128i __A)
{
  return  (__m128i) __builtin_ia32_vphsubdq ((__v4si)__A);
}

/* Vector conditional move and permute */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_cmov_si128(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpcmov (__A, __B, __C);
}

extern __inline __m256i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_cmov_si256(__m256i __A, __m256i __B, __m256i __C)
{
  return  (__m256i) __builtin_ia32_vpcmov256 (__A, __B, __C);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_perm_epi8(__m128i __A, __m128i __B, __m128i __C)
{
  return  (__m128i) __builtin_ia32_vpperm ((__v16qi)__A, (__v16qi)__B, (__v16qi)__C);
}

/* Packed Integer Rotates and Shifts
   Rotates - Non-Immediate form */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_rot_epi8(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vprotb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_rot_epi16(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vprotw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_rot_epi32(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vprotd ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_rot_epi64(__m128i __A,  __m128i __B)
{
  return (__m128i)  __builtin_ia32_vprotq ((__v2di)__A, (__v2di)__B);
}

/* Rotates - Immediate form */

#ifdef __OPTIMIZE__
extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_roti_epi8(__m128i __A, const int __B)
{
  return  (__m128i) __builtin_ia32_vprotbi ((__v16qi)__A, __B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_roti_epi16(__m128i __A, const int __B)
{
  return  (__m128i) __builtin_ia32_vprotwi ((__v8hi)__A, __B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_roti_epi32(__m128i __A, const int __B)
{
  return  (__m128i) __builtin_ia32_vprotdi ((__v4si)__A, __B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_roti_epi64(__m128i __A, const int __B)
{
  return  (__m128i) __builtin_ia32_vprotqi ((__v2di)__A, __B);
}
#else
#define _mm_roti_epi8(A, N) \
  ((__m128i) __builtin_ia32_vprotbi ((__v16qi)(__m128i)(A), (int)(N)))
#define _mm_roti_epi16(A, N) \
  ((__m128i) __builtin_ia32_vprotwi ((__v8hi)(__m128i)(A), (int)(N)))
#define _mm_roti_epi32(A, N) \
  ((__m128i) __builtin_ia32_vprotdi ((__v4si)(__m128i)(A), (int)(N)))
#define _mm_roti_epi64(A, N) \
  ((__m128i) __builtin_ia32_vprotqi ((__v2di)(__m128i)(A), (int)(N)))
#endif

/* Shifts */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shl_epi8(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshlb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shl_epi16(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshlw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shl_epi32(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshld ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_shl_epi64(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshlq ((__v2di)__A, (__v2di)__B);
}


extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sha_epi8(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshab ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sha_epi16(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshaw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sha_epi32(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshad ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_sha_epi64(__m128i __A,  __m128i __B)
{
  return  (__m128i) __builtin_ia32_vpshaq ((__v2di)__A, (__v2di)__B);
}

/* Compare and Predicate Generation
   pcom (integer, unsigned bytes) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltub ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomleub ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtub ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgeub ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomequb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomnequb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalseub ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epu8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtrueub ((__v16qi)__A, (__v16qi)__B);
}

/*pcom (integer, unsigned words) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltuw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomleuw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtuw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgeuw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomequw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomnequw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalseuw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epu16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtrueuw ((__v8hi)__A, (__v8hi)__B);
}

/*pcom (integer, unsigned double words) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltud ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomleud ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtud ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgeud ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomequd ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomnequd ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalseud ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epu32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtrueud ((__v4si)__A, (__v4si)__B);
}

/*pcom (integer, unsigned quad words) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltuq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomleuq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtuq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgeuq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomequq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomnequq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalseuq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epu64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtrueuq ((__v2di)__A, (__v2di)__B);
}

/*pcom (integer, signed bytes) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomleb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgeb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomeqb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomneqb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalseb ((__v16qi)__A, (__v16qi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epi8(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtrueb ((__v16qi)__A, (__v16qi)__B);
}

/*pcom (integer, signed words) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomlew ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgew ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomeqw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomneqw ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalsew ((__v8hi)__A, (__v8hi)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epi16(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtruew ((__v8hi)__A, (__v8hi)__B);
}

/*pcom (integer, signed double words) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltd ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomled ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtd ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomged ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomeqd ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomneqd ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalsed ((__v4si)__A, (__v4si)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epi32(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtrued ((__v4si)__A, (__v4si)__B);
}

/*pcom (integer, signed quad words) */

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comlt_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomltq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comle_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomleq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comgt_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgtq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comge_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomgeq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comeq_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomeqq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comneq_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomneqq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comfalse_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomfalseq ((__v2di)__A, (__v2di)__B);
}

extern __inline __m128i __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_comtrue_epi64(__m128i __A, __m128i __B)
{
  return (__m128i) __builtin_ia32_vpcomtrueq ((__v2di)__A, (__v2di)__B);
}

/* FRCZ */

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_frcz_ps (__m128 __A)
{
  return (__m128) __builtin_ia32_vfrczps ((__v4sf)__A);
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_frcz_pd (__m128d __A)
{
  return (__m128d) __builtin_ia32_vfrczpd ((__v2df)__A);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_frcz_ss (__m128 __A, __m128 __B)
{
  return (__m128) __builtin_ia32_movss ((__v4sf)__A,
					(__v4sf)
					__builtin_ia32_vfrczss ((__v4sf)__B));
}

extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_frcz_sd (__m128d __A, __m128d __B)
{
  return (__m128d) __builtin_ia32_movsd ((__v2df)__A,
					 (__v2df)
					 __builtin_ia32_vfrczsd ((__v2df)__B));
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_frcz_ps (__m256 __A)
{
  return (__m256) __builtin_ia32_vfrczps256 ((__v8sf)__A);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_frcz_pd (__m256d __A)
{
  return (__m256d) __builtin_ia32_vfrczpd256 ((__v4df)__A);
}

/* PERMIL2 */

#ifdef __OPTIMIZE__
extern __inline __m128d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_permute2_pd (__m128d __X, __m128d __Y, __m128i __C, const int __I)
{
  return (__m128d) __builtin_ia32_vpermil2pd ((__v2df)__X,
					      (__v2df)__Y,
					      (__v2di)__C,
					      __I);
}

extern __inline __m256d __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute2_pd (__m256d __X, __m256d __Y, __m256i __C, const int __I)
{
  return (__m256d) __builtin_ia32_vpermil2pd256 ((__v4df)__X,
						 (__v4df)__Y,
						 (__v4di)__C,
						 __I);
}

extern __inline __m128 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm_permute2_ps (__m128 __X, __m128 __Y, __m128i __C, const int __I)
{
  return (__m128) __builtin_ia32_vpermil2ps ((__v4sf)__X,
					     (__v4sf)__Y,
					     (__v4si)__C,
					     __I);
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_permute2_ps (__m256 __X, __m256 __Y, __m256i __C, const int __I)
{
  return (__m256) __builtin_ia32_vpermil2ps256 ((__v8sf)__X,
						(__v8sf)__Y,
						(__v8si)__C,
						__I);
}
#else
#define _mm_permute2_pd(X, Y, C, I)					\
  ((__m128d) __builtin_ia32_vpermil2pd ((__v2df)(__m128d)(X),		\
					(__v2df)(__m128d)(Y),		\
					(__v2di)(__m128i)(C),		\
					(int)(I)))

#define _mm256_permute2_pd(X, Y, C, I)					\
  ((__m256d) __builtin_ia32_vpermil2pd256 ((__v4df)(__m256d)(X),	\
					   (__v4df)(__m256d)(Y),	\
					   (__v4di)(__m256i)(C),	\
					   (int)(I)))

#define _mm_permute2_ps(X, Y, C, I)					\
  ((__m128) __builtin_ia32_vpermil2ps ((__v4sf)(__m128)(X),		\
				       (__v4sf)(__m128)(Y),		\
				       (__v4si)(__m128i)(C),		\
				       (int)(I)))

#define _mm256_permute2_ps(X, Y, C, I)					\
  ((__m256) __builtin_ia32_vpermil2ps256 ((__v8sf)(__m256)(X),		\
					  (__v8sf)(__m256)(Y),  	\
					  (__v8si)(__m256i)(C),		\
 					  (int)(I)))
#endif /* __OPTIMIZE__ */

#ifdef __DISABLE_XOP__
#undef __DISABLE_XOP__
#pragma GCC pop_options
#endif /* __DISABLE_XOP__ */

#endif /* _XOPMMINTRIN_H_INCLUDED */
