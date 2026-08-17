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

#ifndef DECODE_MB_AUX_H
#define DECODE_MB_AUX_H

#include "typedefs.h"
#include "macros.h"
#include "wels_func_ptr_def.h"

namespace WelsEnc {
void WelsDequantLumaDc4x4 (int16_t* pRes, const int32_t kiQp);
void WelsIHadamard4x4Dc (int16_t* pRes);

void WelsInitReconstructionFuncs (SWelsFuncPtrList* pList, uint32_t  iCpuFlags);
void WelsGetEncBlockStrideOffset (int32_t* pBlock, const int32_t kiStrideY, const int32_t kiStrideUV);

void WelsDequantFour4x4_c (int16_t* pRes, const uint16_t* kpQpTable);
void WelsDequant4x4_c (int16_t* pRes, const uint16_t* kpQpTable);
void WelsDequantIHadamard4x4_c (int16_t* pRes, const uint16_t kuiMF);
void WelsDequantIHadamard2x2Dc (int16_t* pDct, const uint16_t kuiMF);

void WelsIDctT4RecOnMb (uint8_t* pDst, int32_t iDstStride, uint8_t* pPred, int32_t iPredStride, int16_t* pDct,
                        PIDctFunc pfIDctFourT4);
void WelsIDctT4Rec_c (uint8_t* pRec, int32_t iStride, uint8_t* pPred, int32_t iPredStride, int16_t* pDct);
void WelsIDctFourT4Rec_c (uint8_t* pRec, int32_t iStride, uint8_t* pPred, int32_t iPredStride, int16_t* pDct);
void WelsIDctRecI16x16Dc_c (uint8_t* pRec, int32_t iStride, uint8_t* pPred, int32_t iPredStride, int16_t* pDctDc);

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#if defined(X86_ASM)
void WelsDequant4x4_sse2 (int16_t* pDct, const uint16_t* kpMF);
void WelsDequantFour4x4_sse2 (int16_t* pDct, const uint16_t* kpMF);
void WelsDequantIHadamard4x4_sse2 (int16_t* pRes, const uint16_t kuiMF);

void WelsIDctT4Rec_mmx (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctT4Rec_sse2 (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctFourT4Rec_sse2 (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctRecI16x16Dc_sse2 (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride,
                               int16_t* pDctDc);
void WelsIDctT4Rec_avx2 (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctFourT4Rec_avx2 (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
#endif//X86_ASM

#ifdef HAVE_NEON
void WelsDequantFour4x4_neon (int16_t* pDct, const uint16_t* kpMF);
void WelsDequant4x4_neon (int16_t* pDct, const uint16_t* kpMF);
void WelsDequantIHadamard4x4_neon (int16_t* pRes, const uint16_t kuiMF);

void WelsIDctT4Rec_neon (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctFourT4Rec_neon (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctRecI16x16Dc_neon (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride,
                               int16_t* pDctDc);
#endif

#ifdef HAVE_NEON_AARCH64
void WelsDequantFour4x4_AArch64_neon (int16_t* pDct, const uint16_t* kpMF);
void WelsDequant4x4_AArch64_neon (int16_t* pDct, const uint16_t* kpMF);
void WelsDequantIHadamard4x4_AArch64_neon (int16_t* pRes, const uint16_t kuiMF);

void WelsIDctT4Rec_AArch64_neon (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctFourT4Rec_AArch64_neon (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctRecI16x16Dc_AArch64_neon (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride,
                                 int16_t* pDctDc);
#endif

#if defined(HAVE_MMI)
void WelsIDctT4Rec_mmi (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctFourT4Rec_mmi (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctRecI16x16Dc_mmi (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDctDc);
#endif//HAVE_MMI

#if defined(HAVE_LASX)
void WelsIDctT4Rec_lasx (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
void WelsIDctFourT4Rec_lasx (uint8_t* pRec, int32_t iStride, uint8_t* pPrediction, int32_t iPredStride, int16_t* pDct);
#endif
#if defined(__cplusplus)
}
#endif//__cplusplus
}
#endif
