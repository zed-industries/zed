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
 * \file    extern.h
 *
 * \brief   extern interfaces between core and plus of wels encoder
 *
 * \date    4/21/2009 Created
 *
 *************************************************************************************
 */
#if !defined(WELS_ENCODER_EXTERN_H__)
#define WELS_ENCODER_EXTERN_H__

#include "typedefs.h"
#include "encoder_context.h"

namespace WelsEnc {

/*!
 * \brief   initialize source picture body
 * \param   kpSrc       SSourcePicture*
 * \param   kiCsp       internal csp format
 * \param   kiWidth widht of picture in pixels
 * \param   kiHeight    height of picture in pixels
 * \return  successful - 0; otherwise none 0 for failed
 */
int32_t InitPic (const void* kpSrc, const int32_t kiCsp, const int32_t kiWidth, const int32_t kiHeight);

/*
 *  SVC core encoder external interfaces
 */

/*!
 * \brief   validate checking in parameter configuration
 * \pParam  pParam      SWelsSvcCodingParam*
 * \return  successful - 0; otherwise none 0 for failed
 */
int32_t ParamValidationExt (SLogContext* pCtx, SWelsSvcCodingParam* pParam);

// GOM based RC related for uiSliceNum decision
void GomValidCheck (const int32_t kiMbWidth, const int32_t kiMbHeight, int32_t* pSliceNum);

/*!
 * \brief   initialize Wels avc encoder core library
 * \param   ppCtx       sWelsEncCtx**
 * \param   para        SWelsSvcCodingParam*
 * \return  successful - 0; otherwise none 0 for failed
 */
int32_t WelsInitEncoderExt (sWelsEncCtx** ppCtx, SWelsSvcCodingParam* pPara, SLogContext* pLogCtx,
                            SExistingParasetList* pExistingParasetList);

/*!
 * \brief   uninitialize Wels encoder core library
 * \param   pEncCtx     sWelsEncCtx*
 * \return  none
 */
void WelsUninitEncoderExt (sWelsEncCtx** ppCtx);

/*!
 * \brief   core svc encoding process
 *
 * \param   h           sWelsEncCtx*, encoder context
 * \param   pFbi        FrameBSInfo*
 * \param   kpSrcPic    Source picture
 * \return  EFrameType (videoFrameTypeIDR/videoFrameTypeI/videoFrameTypeP)
 */
int32_t WelsEncoderEncodeExt (sWelsEncCtx*, SFrameBSInfo* pFbi, const SSourcePicture* kpSrcPic);

int32_t WelsEncoderEncodeParameterSets (sWelsEncCtx* pCtx, void* pDst);

/*
 * Force coding IDR as follows
 */
int32_t ForceCodingIDR (sWelsEncCtx* pCtx,int32_t iLayerId);

/*!
 * \brief   Wels SVC encoder parameters adjustment
 *          SVC adjustment results in new requirement in memory blocks adjustment
 */
int32_t WelsBitRateVerification(SLogContext* pLogCtx,SSpatialLayerConfig* pLayerParam,int32_t iLayerId);
int32_t WelsEncoderParamAdjust (sWelsEncCtx** ppCtx, SWelsSvcCodingParam* pNew);
void WelsEncoderApplyFrameRate (SWelsSvcCodingParam* pParam);
int32_t WelsEncoderApplyBitRate (SLogContext* pLogCtx, SWelsSvcCodingParam* pParam, int32_t iLayer);
int32_t WelsEncoderApplyBitVaryRang(SLogContext* pLogCtx, SWelsSvcCodingParam* pParam, int32_t iRang);
int32_t WelsEncoderApplyLTR (SLogContext* pLogCtx, sWelsEncCtx** ppCtx, SLTRConfig* pLTRValue);
int32_t DynSliceRealloc(sWelsEncCtx* pCtx,SFrameBSInfo* pFrameBsInfo,SLayerBSInfo* pLayerBsInfo);
int32_t FilterLTRRecoveryRequest (sWelsEncCtx* pCtx, SLTRRecoverRequest* pLTRRecoverRequest);
void CheckProfileSetting (SLogContext* pLogCtx,SWelsSvcCodingParam* pParam,int32_t iLayer, EProfileIdc uiProfileIdc);
void CheckLevelSetting (SLogContext* pLogCtx,SWelsSvcCodingParam* pParam,int32_t iLayer, ELevelIdc uiLevelIdc);
void CheckReferenceNumSetting (SLogContext* pLogCtx,  SWelsSvcCodingParam* pParam,int32_t iNumRef);
void FilterLTRMarkingFeedback (sWelsEncCtx* pCtx, SLTRMarkingFeedback* pLTRMarkingFeedback);
}

#endif//WELS_ENCODER_CALLBACK_H__

