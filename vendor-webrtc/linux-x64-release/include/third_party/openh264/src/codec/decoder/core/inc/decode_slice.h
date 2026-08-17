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

#ifndef WELS_DECODE_SLICE_H__
#define WELS_DECODE_SLICE_H__

#include "decoder_context.h"

namespace WelsDec {

int32_t WelsActualDecodeMbCavlcISlice (PWelsDecoderContext pCtx);
int32_t WelsDecodeMbCavlcISlice (PWelsDecoderContext pCtx, PNalUnit pNalCur, uint32_t& uiEosFlag);

int32_t WelsActualDecodeMbCavlcPSlice (PWelsDecoderContext pCtx);
int32_t WelsDecodeMbCavlcPSlice (PWelsDecoderContext pCtx, PNalUnit pNalCur, uint32_t& uiEosFlag);

int32_t WelsActualDecodeMbCavlcBSlice (PWelsDecoderContext pCtx);
int32_t WelsDecodeMbCavlcBSlice (PWelsDecoderContext pCtx, PNalUnit pNalCur, uint32_t& uiEosFlag);

typedef int32_t (*PWelsDecMbFunc) (PWelsDecoderContext pCtx, PNalUnit pNalCur, uint32_t& uiEosFlag);

int32_t WelsDecodeMbCabacISlice (PWelsDecoderContext pCtx, PNalUnit pNalCur, uint32_t& uiEosFlag);
int32_t WelsDecodeMbCabacPSlice (PWelsDecoderContext pCtx, PNalUnit pNalCur, uint32_t& uiEosFlag);
int32_t WelsDecodeMbCabacBSlice (PWelsDecoderContext pCtx, PNalUnit pNalCur, uint32_t& uiEosFlag);
int32_t WelsDecodeMbCabacISliceBaseMode0 (PWelsDecoderContext pCtx, uint32_t& uiEosFlag);
int32_t WelsDecodeMbCabacPSliceBaseMode0 (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiEosFlag);
int32_t WelsDecodeMbCabacBSliceBaseMode0 (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiEosFlag);

int32_t WelsTargetSliceConstruction (PWelsDecoderContext pCtx); //construction based on slice

int32_t WelsDecodeSlice (PWelsDecoderContext pCtx, bool bFirstSliceInLayer, PNalUnit pNalCur);
int32_t WelsDecodeAndConstructSlice (PWelsDecoderContext pCtx);

int32_t WelsTargetMbConstruction (PWelsDecoderContext pCtx);

int32_t WelsMbIntraPredictionConstruction (PWelsDecoderContext pCtx, PDqLayer pCurDqLayer, bool bOutput);
int32_t WelsMbInterSampleConstruction (PWelsDecoderContext pCtx, PDqLayer pCurDqLayer,
                                       uint8_t* pDstY, uint8_t* pDstU, uint8_t* pDstV, int32_t iStrideL, int32_t iStrideC);
int32_t WelsMbInterConstruction (PWelsDecoderContext pCtx, PDqLayer pCurDqLayer);
void WelsLumaDcDequantIdct (int16_t* pBlock, int32_t iQp, PWelsDecoderContext pCtx);
int32_t WelsMbInterPrediction (PWelsDecoderContext pCtx, PDqLayer pCurDqLayer);
void WelsChromaDcIdct (int16_t* pBlock);
bool ComputeColocatedTemporalScaling (PWelsDecoderContext pCtx);

#ifdef __cplusplus
extern "C" {
#endif//__cplusplus

#if defined(X86_ASM)
void WelsBlockZero16x16_sse2 (int16_t* block, int32_t stride);
void WelsBlockZero8x8_sse2 (int16_t* block, int32_t stride);
#endif

#if defined(HAVE_NEON)
void WelsBlockZero16x16_neon (int16_t* block, int32_t stride);
void WelsBlockZero8x8_neon (int16_t* block, int32_t stride);
#endif

#if defined(HAVE_NEON_AARCH64)
void WelsBlockZero16x16_AArch64_neon (int16_t* block, int32_t stride);
void WelsBlockZero8x8_AArch64_neon (int16_t* block, int32_t stride);
#endif
#ifdef __cplusplus
}
#endif//__cplusplus

void WelsBlockFuncInit (SBlockFunc* pFunc,  int32_t iCpu);
void WelsBlockZero16x16_c (int16_t* block, int32_t stride);
void WelsBlockZero8x8_c (int16_t* block, int32_t stride);

} // namespace WelsDec

#endif //WELS_DECODE_SLICE_H__


