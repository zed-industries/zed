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
 * \file    encoder.h
 *
 * \brief   core encoder
 *
 * \date    5/14/2009
 *
 *************************************************************************************
 */
#if !defined(WELS_CORE_ENCODER_H__)
#define WELS_CORE_ENCODER_H__

#include "encoder_context.h"

namespace WelsEnc {
/*!
 * \brief   request specific memory for SVC
 * \param   pEncCtx     sWelsEncCtx*
 * \return  successful - 0; otherwise none 0 for failed
 */
int32_t RequestMemorySvc (sWelsEncCtx** ppCtx, SExistingParasetList* pExistingParasetList);

/*!
 * \brief   free memory in SVC core encoder
 * \param   pEncCtx     sWelsEncCtx**
 * \return  none
 */
void FreeMemorySvc (sWelsEncCtx** ppCtx);

/*!
 * \brief    allocate or reallocate the output bs buffer
 * \return:  successful - 0; otherwise none 0 for failed
 */
int32_t AllocateBsOutputBuffer (CMemoryAlign* pMa, const int32_t iNeededLen, int32_t iOrigLen, const char* kpTag,
                                uint8_t*& pOutputBuffer);
//TODO: to finish this function and call it

/*!
 * \brief   initialize function pointers that potentially used in Wels encoding
 * \param   pEncCtx     sWelsEncCtx*
 * \return  successful - 0; otherwise none 0 for failed
 */
int32_t InitFunctionPointers (sWelsEncCtx* pEncCtx, SWelsSvcCodingParam* _param, uint32_t  uiCpuFlag);

///*!
// * \brief decide frame type (IDR/P frame)
// * \param uiFrameType frame type output
// * \param frame_idx   frame index elapsed currently
// * \param idr         IDR interval
// * \return    successful - 0; otherwise none 0 for failed
// */
/*!
 * \brief   initialize frame coding
 */
void InitFrameCoding (sWelsEncCtx* pEncCtx, const EVideoFrameType keFrameType, const int32_t kiDidx);
void LoadBackFrameNum (sWelsEncCtx* pEncCtx, const int32_t kiDidx);

EVideoFrameType DecideFrameType (sWelsEncCtx* pEncCtx, const int8_t kiSpatialNum, const int32_t kiDidx,
                                 bool bSkipFrameFlag);
void InitBitStream (sWelsEncCtx* pEncCtx);
int32_t GetTemporalLevel (SSpatialLayerInternal* fDlp, const int32_t kiFrameNum, const int32_t kiGopSize);
/*!
 * \brief   Dump reconstruction for dependency layer
 */

extern "C" void DumpDependencyRec (SPicture* pSrcPic, const char* kpFileName, const int8_t kiDid, bool bAppend,
                                   SDqLayer* pDqLayer, bool bSimulCastAVC);

/*!
 * \brief   Dump the reconstruction pictures
 */
void DumpRecFrame (SPicture* pSrcPic, const char* kpFileName, const int8_t kiDid, bool bAppend, SDqLayer* pDqLayer);


/*!
 * \brief   encode overall slices pData in a frame
 * \param   pEncCtx             sWelsEncCtx*, encoder context
 * \param   count_slice_num     count number of slices in a frame
 * \param   eNalType            EWelsNalUnitType for a frame
 * \param   nal_idc             EWelsNalRefIdc for a frame
 * \return  successful - 0; otherwise none 0 for failed
 */
int32_t EncodeFrame (sWelsEncCtx* pEncCtx,
                     const int32_t kiSliceNumCount,
                     const EWelsNalUnitType keNalType,
                     const EWelsNalRefIdc keNalIdc);


/**********************************************************************************
 * memzero Function
***********************************************************************************/
void WelsSetMemZero_c (void* pDst, int32_t iSize); // confirmed_safe_unsafe_usage

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#ifdef X86_ASM
void WelsSetMemZeroAligned64_sse2 (void* pDst, int32_t iSize);
void WelsSetMemZeroSize64_mmx (void* pDst, int32_t iSize);
void WelsSetMemZeroSize8_mmx (void* pDst, int32_t iSize);
void WelsPrefetchZero_mmx (int8_t const* kpDst);
#elif defined(HAVE_NEON)
void WelsSetMemZero_neon (void* pDst, int32_t iSize);
#elif defined(HAVE_NEON_AARCH64)
void WelsSetMemZero_AArch64_neon (void* pDst, int32_t iSize);
#endif

#if defined(__cplusplus)
}
#endif//__cplusplus

/**********************************************************************************
 * Function points type
***********************************************************************************/
}

#endif//WELS_CORE_ENCODER_H__
