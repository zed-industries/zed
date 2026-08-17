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
 * \file    slice_multi_threading.c
 *
 * \brief   slice based multiple threading
 *
 * \date    04/16/2010 Created
 *
 *************************************************************************************
 */
#ifndef SVC_SLICE_MULTIPLE_THREADING_H__
#define SVC_SLICE_MULTIPLE_THREADING_H__


#include "typedefs.h"
#include "codec_app_def.h"
#include "param_svc.h"
#include "encoder_context.h"
#include "svc_enc_frame.h"
#include "svc_enc_macroblock.h"
#include "svc_enc_slice_segment.h"
#include "WelsThreadLib.h"

namespace WelsEnc {
void UpdateMbListNeighborParallel (SDqLayer* pCurDq,
                                   SMB* pMbList,
                                   const int32_t kiSliceIdc);

void CalcSliceComplexRatio (SDqLayer* pCurDq);

int32_t NeedDynamicAdjust (SSlice** ppSliceInLayer, const int32_t iSliceNum);

void DynamicAdjustSlicing (sWelsEncCtx* pCtx,
                           SDqLayer* pCurDqLayer,
                           int32_t iCurDid);

int32_t RequestMtResource (sWelsEncCtx** ppCtx, SWelsSvcCodingParam* pParam, const int32_t kiCountBsLen,
                           const int32_t kiTargetSpatialBsSize, bool bDynamicSlice);

void ReleaseMtResource (sWelsEncCtx** ppCtx);

int32_t AppendSliceToFrameBs (sWelsEncCtx* pCtx, SLayerBSInfo* pLbi, const int32_t kiSliceCount);

#if !defined(_WIN32)
WELS_THREAD_ROUTINE_TYPE UpdateMbListThreadProc (void* arg);
#endif//!_WIN32

int32_t DynamicDetectCpuCores();


int32_t AdjustBaseLayer (sWelsEncCtx* pCtx);
int32_t AdjustEnhanceLayer (sWelsEncCtx* pCtx, int32_t iCurDid);



#if defined(MT_DEBUG)
void TrackSliceComplexities (sWelsEncCtx* pCtx, const int32_t kiCurDid);
#endif
#if defined(MT_DEBUG)
void TrackSliceConsumeTime (sWelsEncCtx* pCtx, int32_t* pDidList, const int32_t kiSpatialNum);
#endif//defined(MT_DEBUG)

void SetOneSliceBsBufferUnderMultithread(sWelsEncCtx* pCtx, const int32_t kiThreadIdx, SSlice* pSlice);
int32_t WriteSliceBs (sWelsEncCtx* pCtx,SWelsSliceBs* pSliceBs,const int32_t iSliceIdx,int32_t& iSliceSize);
}

#endif//SVC_SLICE_MULTIPLE_THREADING_H__

