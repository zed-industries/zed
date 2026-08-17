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
 * \file    svc_mode_decision.h
 *
 * \brief   SVC Spatial Enhancement Layer MD
 *
 * \date    2009.7.29 Created
 *
 *************************************************************************************
 */

#ifndef SVC_MODE_DECISION_H
#define SVC_MODE_DECISION_H
#include "encoder_context.h"
#include "svc_encode_mb.h"
#include "svc_encode_slice.h"
#include "svc_enc_macroblock.h"
#include "md.h"


namespace WelsEnc {
////////////////////////
// INTERFACE, called by svc_encode_slice.c
///////////////////////
#define DELTA_QP_SCD_THD 5

typedef enum {
STATIC,
SCROLLED
} ESkipModes;

// NOILP ILFMD ENTRANCE
void WelsMdSpatialelInterMbIlfmdNoilp (sWelsEncCtx* pEncCtx, SWelsMD* pWelsMd, SSlice* pSlice, SMB* pCurMb,
                                       const Mb_Type kuiRefMbType);
void WelsMdInterMbEnhancelayer (sWelsEncCtx* pEnc, SWelsMD* pMd, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache);
SMB* GetRefMb (SDqLayer* pCurLayer, SMB* pCurMb);
void SetMvBaseEnhancelayer (SWelsMD* pMd, SMB* pCurMb, const SMB* kpRefMb);

//////////////
// MD from background detection
//////////////
bool WelsMdInterJudgeBGDPskip (sWelsEncCtx* pEnc, SWelsMD* pMd, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache,
                               bool* bKeepSkip);
bool WelsMdInterJudgeBGDPskipFalse (sWelsEncCtx* pEnc, SWelsMD* pMd, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache,
                                    bool* bKeepSkip);

void WelsMdUpdateBGDInfo (SDqLayer* pCurLayer,  SMB* pCurMb, const bool kbCollocatedPredFlag,
                               const int32_t kiRefPictureType);
void WelsMdUpdateBGDInfoNULL (SDqLayer* pCurLayer,  SMB* pCurMb, const bool kbCollocatedPredFlag,
                                   const int32_t kiRefPictureType);

//////////////
// MD for screen contents
//////////////
bool MdInterSCDPskipProcess (sWelsEncCtx* pEncCtx, SWelsMD* pMd, SSlice* pSlice, SMB* pCurMb, SMbCache* pMbCache,
                             ESkipModes eSkipMode);
typedef bool (*pJudgeSkipFun) (sWelsEncCtx* pEncCtx, SMB* pCurMb, SMbCache* pMbCache, SWelsMD* pWelsMd);
void SetBlockStaticIdcToMd (void* pVaa, void* pMd, SMB* pCurMb, void* pDqLay);
void WelsInitSCDPskipFunc (SWelsFuncPtrList* pFuncList, const bool bScrollingDetection);

void SetScrollingMvToMd (SVAAFrameInfo* pVaa, SWelsMD* pWelsMd);
void SetScrollingMvToMdNull (SVAAFrameInfo* pVaa, SWelsMD* pWelsMd);
}
#endif //SVC_MODE_DECISION_H

