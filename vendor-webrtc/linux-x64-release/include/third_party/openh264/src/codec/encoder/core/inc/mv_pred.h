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
 * \file    mv_pred.h
 *
 * \brief   Get MV predictor and update motion vector of mb cache
 *
 * \date    05/22/2009 Created
 *
 *************************************************************************************
 */

#ifndef WELS_MV_PRED_H__
#define WELS_MV_PRED_H__


#include "svc_enc_macroblock.h"
#include "mb_cache.h"

namespace WelsEnc {
/*!
 * \brief   update pMv and uiRefIndex cache for current MB, only for P_16x16 (SKIP inclusive)
 * \param
 * \param
 */

/*!
 * \brief   update pMv and uiRefIndex cache for current MB and pMbCache, only for P_16x16 (SKIP inclusive)
 * \param
 * \param
 */
void UpdateP16x16MotionInfo (SMbCache* pMbCache, SMB* pCurMb, const int8_t kiRef, SMVUnitXY* pMv); //for encoder

/*!
 * \brief   update pMv and uiRefIndex cache for current MB and pMbCache, only for P_16x8
 * \param
 * \param
 */
void UpdateP16x8MotionInfo (SMbCache* pMbCache, SMB* pCurMb, const int32_t kiPartIdx, const int8_t kiRef,
                            SMVUnitXY* pMv);

/*!
 * \brief   update pMv and uiRefIndex cache for current MB and pMbCache, only for P_8x16
 * \param
 * \param
 */
void update_P8x16_motion_info (SMbCache* pMbCache, SMB* pCurMb, const int32_t kiPartIdx, const int8_t kiRef,
                               SMVUnitXY* pMv);

/*!
 * \brief   update pMv and uiRefIndex cache for current MB and pMbCache, only for P_8x8
 * \param
 * \param
 */
void UpdateP8x8MotionInfo (SMbCache* pMbCache, SMB* pCurMb, const int32_t kiPartIdx, const int8_t kiRef,
                           SMVUnitXY* pMv);

/*!
 * \brief   update pMv and uiRefIndex cache for current MB and pMbCache, only for P_4x4
 * \param
 * \param
 */
void UpdateP4x4MotionInfo (SMbCache* pMbCache, SMB* pCurMb, const int32_t kiPartIdx, const int8_t kiRef,
                           SMVUnitXY* pMv);

/*!
 * \brief   update pMv and uiRefIndex cache for current MB and pMbCache, only for P_8x4
 * \param
 * \param
 */
void UpdateP8x4MotionInfo (SMbCache* pMbCache, SMB* pCurMb, const int32_t kiPartIdx, const int8_t kiRef,
                           SMVUnitXY* pMv);

/*!
 * \brief   update pMv and uiRefIndex cache for current MB and pMbCache, only for P_4x8
 * \param
 * \param
 */
void UpdateP4x8MotionInfo (SMbCache* pMbCache, SMB* pCurMb, const int32_t kiPartIdx, const int8_t kiRef,
                           SMVUnitXY* pMv);

/*!
 * \brief   get the motion predictor for 4*4 or 8*8 or 16*16 block
 * \param
 * \param   output mvp_x and mvp_y
 */
void PredMv (const SMVComponentUnit* kpMvComp, int8_t iPartIdx, int8_t iPartW, int32_t iRef, SMVUnitXY* sMvp);


/*!
 * \brief   get the motion predictor for SKIP MB
 * \param
 * \param   output mvp_x and mvp_y
 */
void PredSkipMv (SMbCache* pMbCache, SMVUnitXY* sMvp);


/*!
 * \brief   get the motion predictor for inter16x8 MB
 * \param
 * \param   output mvp_x and mvp_y
 */
void PredInter16x8Mv (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* sMvp);


/*!
 * \brief   get the motion predictor for inter8x16 MB
 * \param
 * \param   output mvp_x and mvp_y
 */
void PredInter8x16Mv (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* sMvp);

//=========================update motion info(MV and ref_idx) into Mb_cache==========================
/*!
 * \brief   only update pMv cache for current MB, only for P_16x16
 * \param
 * \param
 */
//void update_p16x16_motion2cache(SMbCache* pMbCache, int8_t pRef, SMVUnitXY* pMv);

/*!
 * \brief   only update pMv cache for current MB, only for P_16x8
 * \param
 * \param
 */
void UpdateP16x8Motion2Cache (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* pMv);

/*!
 * \brief   only update pMv cache for current MB, only for P_8x16
 * \param
 * \param
 */
void UpdateP8x16Motion2Cache (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* pMv);

/*!
 * \brief   only update pMv cache for current MB, only for P_8x8
 * \param
 * \param
 */
void UpdateP8x8Motion2Cache (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* pMv);

/*!
 * \brief   only update pMv cache for current MB, only for P_4x4
 * \param
 * \param
 */
void UpdateP4x4Motion2Cache (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* pMv);

/*!
 * \brief   only update pMv cache for current MB, only for P_8x4
 * \param
 * \param
 */
void UpdateP8x4Motion2Cache (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* pMv);

/*!
 * \brief   only update pMv cache for current MB, only for P_4x8
 * \param
 * \param
 */
void UpdateP4x8Motion2Cache (SMbCache* pMbCache, int32_t iPartIdx, int8_t iRef, SMVUnitXY* pMv);
}
#endif//WELS_MV_PRED_H__
