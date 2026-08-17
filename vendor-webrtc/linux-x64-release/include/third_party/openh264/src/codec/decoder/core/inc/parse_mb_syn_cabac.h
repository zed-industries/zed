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
 * \file    parse_mb_syn_cabac.h
 *
 * \brief   cabac parse for syntax elements
 *
 * \date    10/10/2014 Created
 *
 *************************************************************************************
 */
#ifndef WELS_PARSE_MB_SYN_CABAC_H__
#define WELS_PARSE_MB_SYN_CABAC_H__

#include "decoder_context.h"
#include "cabac_decoder.h"
namespace WelsDec {
int32_t ParseEndOfSliceCabac (PWelsDecoderContext pCtx, uint32_t& uiBinVal);
int32_t ParseSkipFlagCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiSkip);
int32_t ParseMBTypeISliceCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiBinVal);
int32_t ParseMBTypePSliceCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiBinVal);
int32_t ParseMBTypeBSliceCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiBinVal);
int32_t ParseTransformSize8x8FlagCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail,
                                        bool& bTransformSize8x8Flag);
int32_t ParseSubMBTypeCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiSubMbType);
int32_t ParseBSubMBTypeCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiSubMbType);
int32_t ParseIntraPredModeLumaCabac (PWelsDecoderContext pCtx, int32_t& iBinVal);
int32_t ParseIntraPredModeChromaCabac (PWelsDecoderContext pCtx, uint8_t uiNeighAvail, int32_t& iBinVal);
int32_t ParseInterPMotionInfoCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCount,
                                    int16_t pMotionVector[LIST_A][30][MV_A], int16_t pMvdCache[LIST_A][30][MV_A], int8_t pRefIndex[LIST_A][30]);
int32_t ParseInterBMotionInfoCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCount,
                                    int16_t pMotionVector[LIST_A][30][MV_A], int16_t pMvdCache[LIST_A][30][MV_A], int8_t pRefIndex[LIST_A][30],
                                    int8_t pDirect[30]);
int32_t ParseRefIdxCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint8_t* nzc,
                          int8_t ref_idx[LIST_A][30], int8_t direct[30],
                          int32_t iListIdx, int32_t index, int32_t iActiveRefNum, int32_t b8mode, int8_t& iRefIdxVal);
int32_t ParseMvdInfoCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, int8_t pRefIndex[LIST_A][30],
                           int16_t pMvdCache[LIST_A][30][2], int32_t index, int8_t iListIdx, int8_t iMvComp, int16_t& iMvdVal);
int32_t ParseCbpInfoCabac (PWelsDecoderContext pCtx, PWelsNeighAvail pNeighAvail, uint32_t& uiBinVal);
int32_t ParseDeltaQpCabac (PWelsDecoderContext pCtx, int32_t& iQpDelta);
int32_t ParseCbfInfoCabac (PWelsNeighAvail pNeighAvail, uint8_t* pNzcCache, int32_t index, int32_t iResProperty,
                           PWelsDecoderContext pCtx, uint32_t& uiCbpBit);
int32_t ParseSignificantMapCabac (int32_t* pSignificantMap, int32_t iResProperty, PWelsDecoderContext pCtx,
                                  uint32_t& uiBinVal);
int32_t ParseSignificantCoeffCabac (int32_t* significant, int32_t iResProperty, PWelsDecoderContext pCtx);
int32_t ParseResidualBlockCabac (PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCountCache, SBitStringAux* pBsAux,
                                 int32_t index, int32_t iMaxNumCoeff, const uint8_t* pScanTable, int32_t iResProperty, int16_t* sTCoeff, uint8_t uiQp,
                                 PWelsDecoderContext pCtx);
int32_t ParseResidualBlockCabac8x8 (PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCountCache, SBitStringAux* pBsAux,
                                    int32_t index, int32_t iMaxNumCoeff, const uint8_t* pScanTable, int32_t iResProperty, int16_t* sTCoeff, uint8_t uiQp,
                                    PWelsDecoderContext pCtx);
int32_t ParseIPCMInfoCabac (PWelsDecoderContext pCtx);
void    UpdateP16x16MvdCabac (SDqLayer* pCurDqLayer, int16_t pMvd[2], const int8_t iListIdx);
void    UpdateP8x8RefIdxCabac (PDqLayer pCurDqLayer, int8_t pRefIndex[LIST_A][30], int32_t iPartIdx, const int8_t iRef,
                               const int8_t iListIdx);
void    UpdateP8x8DirectCabac (PDqLayer pCurDqLayer, int32_t iPartIdx);
void    UpdateP16x16DirectCabac (PDqLayer pCurDqLayer);
void    UpdateP8x8RefCacheIdxCabac (int8_t pRefIndex[LIST_A][30], const int16_t& iPartIdx, const int32_t& listIdx,
                                    const int8_t& iRef);
}
//#pragma pack()
#endif
