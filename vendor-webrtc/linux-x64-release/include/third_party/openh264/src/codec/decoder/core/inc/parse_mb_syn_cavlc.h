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
 * \file    parse_mb_syn_cavlc.h
 *
 * \brief   Parsing all syntax elements of mb and decoding residual with cavlc
 *
 * \date    03/17/2009 Created
 *
 *************************************************************************************
 */


#ifndef WELS_PARSE_MB_SYN_CAVLC_H__
#define WELS_PARSE_MB_SYN_CAVLC_H__

#include "wels_common_basis.h"
#include "decoder_context.h"
#include "dec_frame.h"
#include "slice.h"

namespace WelsDec {



void GetNeighborAvailMbType (PWelsNeighAvail pNeighAvail, PDqLayer pCurDqLayer);
void WelsFillCacheNonZeroCount (PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCount, PDqLayer pCurDqLayer);
void WelsFillCacheConstrain0IntraNxN (PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCount, int8_t* pIntraPredMode,
                                      PDqLayer pCurDqLayer);
void WelsFillCacheConstrain1IntraNxN (PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCount, int8_t* pIntraPredMode,
                                      PDqLayer pCurDqLayer);
void WelsFillCacheInterCabac (PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCount,
                              int16_t iMvArray[LIST_A][30][MV_A], int16_t iMvdCache[LIST_A][30][MV_A], int8_t iRefIdxArray[LIST_A][30],
                              PDqLayer pCurDqLayer);
void WelsFillDirectCacheCabac (PWelsNeighAvail pNeighAvail, int8_t iDirect[30], PDqLayer pCurDqLayer);
void WelsFillCacheInter (PWelsNeighAvail pNeighAvail, uint8_t* pNonZeroCount,
                         int16_t iMvArray[LIST_A][30][MV_A], int8_t iRefIdxArray[LIST_A][30], PDqLayer pCurDqLayer);

/*!
 * \brief   check iPredMode for intra16x16 eligible or not
 * \param   input : current iPredMode
 * \param   output: 0 indicating decoding correctly; -1 means error occurence
 */
int32_t CheckIntra16x16PredMode (uint8_t uiSampleAvail, int8_t* pMode);

/*!
 * \brief   check iPredMode for intraNxN eligible or not
 * \param   input : current iPredMode
 * \param   output: 0 indicating decoding correctly; -1 means error occurence
 */
int32_t CheckIntraNxNPredMode (int32_t* pSampleAvail, int8_t* pMode, int32_t iIndex, bool b8x8);

/*!
 * \brief   check iPredMode for chroma eligible or not
 * \param   input : current iPredMode
 * \param   output: 0 indicating decoding correctly; -1 means error occurence
 */
int32_t CheckIntraChromaPredMode (uint8_t uiSampleAvail, int8_t* pMode);

/*!
 * \brief   predict the mode of intra4x4
 * \param   input : current intra4x4 block index
 * \param   output: mode index
 */
int32_t PredIntra4x4Mode (int8_t* pIntraPredMode, int32_t iIdx4);


void BsStartCavlc (PBitStringAux pBs);
void BsEndCavlc (PBitStringAux pBs);

int32_t WelsResidualBlockCavlc (SVlcTable* pVlcTable,
                                uint8_t* pNonZeroCountCache,
                                PBitStringAux pBs,
                                /*int16_t* coeff_level,*/
                                int32_t iIndex,
                                int32_t iMaxNumCoeff,
                                const uint8_t* kpZigzagTable,
                                int32_t iResidualProperty,
                                /*short *tCoeffLevel,*/
                                int16_t* pTCoeff,
                                uint8_t uiQp,
                                PWelsDecoderContext pCtx);

// Transform8x8
int32_t WelsResidualBlockCavlc8x8 (SVlcTable* pVlcTable,
                                   uint8_t* pNonZeroCountCache,
                                   PBitStringAux pBs,
                                   /*int16_t* coeff_level,*/
                                   int32_t iIndex,
                                   int32_t iMaxNumCoeff,
                                   const uint8_t* kpZigzagTable,
                                   int32_t iResidualProperty,
                                   /*short *tCoeffLevel,*/
                                   int16_t* pTCoeff,
                                   int32_t  iIdx4x4,
                                   uint8_t uiQp,
                                   PWelsDecoderContext pCtx);

/*!
 * \brief   parsing inter info (including ref_index and pMvd)
 * \param   input : decoding context, current mb, bit-stream
 * \param   output: 0 indicating decoding correctly; -1 means error
 */
int32_t ParseInterInfo (PWelsDecoderContext pCtx, int16_t iMvArray[LIST_A][30][MV_A], int8_t iRefIdxArray[LIST_A][30],
                        PBitStringAux pBs);
int32_t ParseInterBInfo (PWelsDecoderContext pCtx, int16_t iMvArray[LIST_A][30][MV_A],
                         int8_t iRefIdxArray[LIST_A][30], PBitStringAux pBs);
} // namespace WelsDec
#endif//WELS_PARSE_MB_SYN_CAVLC_H__
