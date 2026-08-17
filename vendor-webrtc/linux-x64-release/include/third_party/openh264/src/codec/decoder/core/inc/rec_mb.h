/*!
 * \copy
 *     Copyright (c)  2009-2013, Cisco Systems
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
 *
 * \file    rec_mb.h
 *
 * \brief   interfaces for all macroblock decoding process after mb syntax parsing and residual decoding with cavlc.
 *
 * \date    3/4/2009 Created
 *
 *************************************************************************************
 */

#ifndef WELS_REC_MB_H__
#define WELS_REC_MB_H__

#include "typedefs.h"
#include "wels_common_basis.h"
#include "error_code.h"

#include "decoder_context.h"

namespace WelsDec {

#define WELS_B_MB_REC_VERIFY(uiRet) do{ \
  uint32_t uiRetTmp = (uint32_t)uiRet; \
  if( uiRetTmp != ERR_NONE ) \
    return uiRetTmp; \
}while(0)

typedef struct TagMCRefMember {
  uint8_t* pDstY;
  uint8_t* pDstU;
  uint8_t* pDstV;

  uint8_t* pSrcY;
  uint8_t* pSrcU;
  uint8_t* pSrcV;

  int32_t iSrcLineLuma;
  int32_t iSrcLineChroma;

  int32_t iDstLineLuma;
  int32_t iDstLineChroma;

  int32_t iPicWidth;
  int32_t iPicHeight;
} sMCRefMember;

void BaseMC (PWelsDecoderContext pCtx, sMCRefMember* pMCRefMem, const int32_t& listIdx, const int8_t& iRefIdx,
             int32_t iXOffset, int32_t iYOffset, SMcFunc* pMCFunc,
             int32_t iBlkWidth, int32_t iBlkHeight, int16_t iMVs[2]);

void WelsFillRecNeededMbInfo (PWelsDecoderContext pCtx, bool bOutput, PDqLayer pCurDqLayer);

int32_t RecI4x4Mb (int32_t iMBXY, PWelsDecoderContext pCtx, int16_t* pScoeffLevel, PDqLayer pDqLayer);

int32_t RecI4x4Luma (int32_t iMBXY, PWelsDecoderContext pCtx, int16_t* pScoeffLevel, PDqLayer pDqLayer);

int32_t RecI4x4Chroma (int32_t iMBXY, PWelsDecoderContext pCtx, int16_t* pScoeffLevel, PDqLayer pDqLayer);

int32_t RecI8x8Mb (int32_t iMbXy, PWelsDecoderContext pCtx, int16_t* pScoeffLevel, PDqLayer pDqLayer);

int32_t RecI8x8Luma (int32_t iMbXy, PWelsDecoderContext pCtx, int16_t* pScoeffLevel, PDqLayer pDqLayer);

int32_t RecI16x16Mb (int32_t iMBXY, PWelsDecoderContext pCtx, int16_t* pScoeffLevel, PDqLayer pDqLayer);

int32_t RecChroma (int32_t iMBXY, PWelsDecoderContext pCtx, int16_t* pScoeffLevel, PDqLayer pDqLayer);

int32_t GetInterPred (uint8_t* pPredY, uint8_t* pPredCb, uint8_t* pPredCr, PWelsDecoderContext pCtx);

int32_t GetInterBPred (uint8_t* pPredYCbCr[3], uint8_t* pTempPredYCbCr[3], PWelsDecoderContext pCtx);

} // namespace WelsDec

#endif //WELS_REC_MB_H__

