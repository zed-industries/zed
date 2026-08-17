/*!
 * \copy
 *     Copyright (c)  2013, Cisco Systems
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef WELS_COPY_MB_H_
#define WELS_COPY_MB_H_

#include "typedefs.h"

/****************************************************************************
 * Copy functions
 ****************************************************************************/
void WelsCopy4x4_c (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x4_c (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy4x8_c (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x8_c (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x16_c (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS); //
void WelsCopy16x8_c (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS); //
void WelsCopy16x16_c (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#if defined (X86_ASM)

void WelsCopy8x8_mmx (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x16_mmx (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x8NotAligned_sse2 (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
void WelsCopy16x16_sse2 (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
void WelsCopy16x16NotAligned_sse2 (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
#endif//X86_ASM

#if defined (HAVE_NEON)
void WelsCopy8x8_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x16_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x16NotAligned_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x8NotAligned_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x16_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
#endif

#if defined (HAVE_NEON_AARCH64)
void WelsCopy8x8_AArch64_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x16_AArch64_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x16NotAligned_AArch64_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x8NotAligned_AArch64_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x16_AArch64_neon (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
#endif

#if defined (HAVE_MMI)
void WelsCopy8x8_mmi (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x16_mmi (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x8NotAligned_mmi (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
void WelsCopy16x16_mmi (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
void WelsCopy16x16NotAligned_mmi (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
#endif//HAVE_MMI

#if defined (HAVE_MSA)
void WelsCopy8x8_msa (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy8x16_msa (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x8_msa (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
void WelsCopy16x16_msa (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
#endif//HAVE_MSA

#if defined (HAVE_LSX)
void WelsCopy8x8_lsx (uint8_t* pDst, int32_t iStrideD, uint8_t* pSrc, int32_t iStrideS);
void WelsCopy16x16_lsx (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
void WelsCopy16x16NotAligned_lsx (uint8_t* Dst, int32_t  iStrideD, uint8_t* Src, int32_t  iStrideS);
#endif

#if defined(__cplusplus)
}
#endif//__cplusplus

#endif //SAMPLE_H_
