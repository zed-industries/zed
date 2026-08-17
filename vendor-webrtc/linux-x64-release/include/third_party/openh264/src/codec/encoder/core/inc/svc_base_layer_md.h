/*!
 * \copy
 *     Copyright (c)  2010-2013, Cisco Systems
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
 * \file    svc_base_layer_md.h
 *
 * \brief   mode decision
 *
 * \date    2009.08.10 Created
 *
 *************************************************************************************
 */
#ifndef SVC_BASE_LAYER_MACROBLOCK_MODE_DECISION_H__
#define SVC_BASE_LAYER_MACROBLOCK_MODE_DECISION_H__

#include "md.h"
#include "mb_cache.h"

namespace WelsEnc {
void WelsMdIntraInit (sWelsEncCtx* pEncCtx, SMB* pCurMb, SMbCache* pMbCache, const int32_t kiSliceFirstMbXY);
int32_t WelsMdI16x16 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SMbCache* pMbCache, int32_t iLambda);
int32_t WelsMdIntraChroma (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SMbCache* pMbCache, int32_t iLambda);

int32_t WelsMdI4x4 (sWelsEncCtx* pEnc, SWelsMD* pMd, SMB* pCurMb, SMbCache* pMbCache);
int32_t WelsMdI4x4Fast (sWelsEncCtx* pEnc, SWelsMD* pMd, SMB* pCurMb, SMbCache* pMbCache);

int32_t WelsMdIntraFinePartition (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SMB* pCurMb, SMbCache* pMbCache);
int32_t WelsMdIntraFinePartitionVaa (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SMB* pCurMb, SMbCache* pMbCache);

void WelsMdIntraMb (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SMB* pCurMb, SMbCache* pMbCache);

void WelsMdBackgroundMbEnc (sWelsEncCtx* pEnc, SWelsMD* pMd, SMB* pCurMb, SMbCache* pMbCache, SSlice* pSlice, bool bSkipMbFlag);
bool WelsMdPSkipEnc (sWelsEncCtx* pEnc, SWelsMD* pMd, SMB* pCurMb, SMbCache* pMbCache);
int32_t WelsMdP16x16 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SWelsMD* pWelsMd, SSlice* pSlice, SMB* pCurMb);

int32_t WelsMdP16x8 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SWelsMD* pWelsMd, SSlice* pSlice);
int32_t WelsMdP8x16 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SWelsMD* pWelsMd, SSlice* pSlice);
int32_t WelsMdP8x8 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SWelsMD* pWelsMd, SSlice* pSlice);
int32_t WelsMdP4x4 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SWelsMD* pWelsMd, SSlice* pSlice, const int32_t ki8x8Idx);
int32_t WelsMdP8x4 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SWelsMD* pWelsMd, SSlice* pSlice, const int32_t ki8x8Idx);
int32_t WelsMdP4x8 (SWelsFuncPtrList* pFunc, SDqLayer* pCurDqLayer, SWelsMD* pWelsMd, SSlice* pSlice, const int32_t ki8x8Idx);
/*static*/  void WelsMdInterInit (sWelsEncCtx* pEncCtx, SSlice* pSlice, SMB* pCurMb, const int32_t kiSliceFirstMbXY);
/*static*/ void WelsMdInterFinePartition (sWelsEncCtx* pEnc, SWelsMD* pMd, SSlice* pSlice, SMB* pCurMb, int32_t bestCost);
/*static*/ void WelsMdInterFinePartitionVaa (sWelsEncCtx* pEnc, SWelsMD* pMd, SSlice* pSlice, SMB* pCurMb, int32_t bestCost);
/*static*/ void WelsMdInterFinePartitionVaaOnScreen (sWelsEncCtx* pEnc, SWelsMD* pMd, SSlice* pSlice, SMB* pCurMb,
    int32_t bestCost);
void WelsMdInterMbRefinement (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SMB* pCurMb, SMbCache* pMbCache);
bool WelsMdFirstIntraMode (sWelsEncCtx* pEnc, SWelsMD* pMd, SMB* pCurMb, SMbCache* pMbCache);
//bool svc_md_first_intra_mode_constrained(sWelsEncCtx* pEnc, SWelsMD* pMd, SMB* pCurMb, SMbCache *pMbCache);
void WelsMdInterMb (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SSlice* pSlice, SMB* pCurMb, SMbCache* pUnused);

//both used in BL and EL
//void wels_md_inter_init ( SWelsMD* pMd, const uint8_t ref_idx, const bool is_highest_dlayer_flag );



bool WelsMdInterJudgePskip (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache,
                            bool bTrySkip);
void WelsMdInterUpdatePskip (SDqLayer* pCurDqLayer, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache);
void WelsMdInterDecidedPskip (sWelsEncCtx* pEncCtx, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache);

void WelsMdInterDoubleCheckPskip (SMB* pCurMb, SMbCache* pMbCache);
void WelsMdInterEncode (sWelsEncCtx* pEncCtx, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache);

void WelsMdInterSaveSadAndRefMbType (Mb_Type* pRefMbTypeList, SMbCache* pMbCache, const SMB*  kpCurMb,
                                     const SWelsMD* kpMd);

void WelsMdInterSecondaryModesEnc (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SSlice* pSlice, SMB* pCurMb,
                                   SMbCache* pMbCache, const bool kbSkip);
void WelsMdIntraSecondaryModesEnc (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SMB* pCurMb, SMbCache* pMbCache);
//end of: both used in BL and EL


}
#endif//WELS_MACROBLOCK_MODE_DECISION_H__
