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

#include "dec_frame.h"
#include "decoder_context.h"

#define RETURN_ERR_IF_NULL(pRefPic0) \
if ( pRefPic0 == NULL) \
  return GENERATE_ERROR_NO(ERR_LEVEL_MB_DATA, ERR_INFO_INVALID_REF_INDEX)

namespace WelsDec {

/*!
* \brief     update mv and ref_index cache for current MB, only for P_16x16 (SKIP inclusive)
* \param
* \param
*/
void UpdateP16x16MotionInfo (PDqLayer pCurDqLayer, int32_t listIdx, int8_t iRef, int16_t iMVs[2]);

/*!
* \brief     update ref_index cache for current MB, only for P_16x16 (SKIP inclusive)
* \param
* \param
*/
void UpdateP16x16RefIdx (PDqLayer pCurDqLayer, int32_t listIdx, int8_t iRef);

/*!
* \brief     update mv only cache for current MB, only for P_16x16 (SKIP inclusive)
* \param
* \param
*/
void UpdateP16x16MotionOnly (PDqLayer pCurDqLayer, int32_t listIdx, int16_t iMVs[2]);

/*!
* \brief   update mv and ref_index cache for current MB, only for P_16x8
* \param
* \param
*/
void UpdateP16x8MotionInfo (PDqLayer pCurDqLayer, int16_t iMotionVector[LIST_A][30][MV_A],
                            int8_t iRefIndex[LIST_A][30],
                            int32_t listIdx, int32_t iPartIdx, int8_t iRef, int16_t iMVs[2]);


/*!
 * \brief    update mv and ref_index cache for current MB, only for P_8x16
 * \param
 * \param
 */
void UpdateP8x16MotionInfo (PDqLayer pCurDqLayer, int16_t iMotionVector[LIST_A][30][MV_A],
                            int8_t iRefIndex[LIST_A][30],
                            int32_t listIdx, int32_t iPartIdx, int8_t iRef, int16_t iMVs[2]);

/*!
 * \brief   get the motion predictor for skip mode
 * \param
 * \param   output iMvp[]
 */
void PredPSkipMvFromNeighbor (PDqLayer pCurDqLayer, int16_t iMvp[2]);

/*!
* \brief   get the motion predictor and reference for B-slice direct mode version 2
* \param
* \param   output iMvp[] and ref
*/
int32_t  PredMvBDirectSpatial (PWelsDecoderContext pCtx, int16_t iMvp[LIST_A][2], int8_t ref[LIST_A],
                               SubMbType& subMbType);

/*!
* \brief   get Colocated MB for both Spatial and Temporal Direct Mode
* \param
* \param   output MbType and SubMbType
*/
int32_t GetColocatedMb (PWelsDecoderContext pCtx, MbType& mbType, SubMbType& subMbType);

/*!
* \brief   get the motion predictor for B-slice temporal direct mode 16x16
*/
int32_t PredBDirectTemporal (PWelsDecoderContext pCtx, int16_t iMvp[LIST_A][2], int8_t ref[LIST_A],
                             SubMbType& subMbType);

/*!
* \brief   get the motion params for B-slice spatial direct mode
* \param
* \param   output iMvp[]
*/

/*!
 * \brief   get the motion predictor for 4*4 or 8*8 or 16*16 block
 * \param
 * \param   output iMvp[]
 */
void PredMv (int16_t iMotionVector[LIST_A][30][MV_A], int8_t iRefIndex[LIST_A][30],
             int32_t listIdx, int32_t iPartIdx, int32_t iPartWidth, int8_t iRef, int16_t iMVP[2]);

/*!
 * \brief   get the motion predictor for inter16x8 MB
 * \param
 * \param   output mvp_x and mvp_y
 */
void PredInter16x8Mv (int16_t iMotionVector[LIST_A][30][MV_A], int8_t iRefIndex[LIST_A][30],
                      int32_t listIdx, int32_t iPartIdx, int8_t iRef, int16_t iMVP[2]);

/*!
 * \brief   get the motion predictor for inter8x16 MB
 * \param
 * \param   output mvp_x and mvp_y
 */
void PredInter8x16Mv (int16_t iMotionVector[LIST_A][30][MV_A], int8_t iRefIndex[LIST_A][30],
                      int32_t listIdx, int32_t iPartIdx, int8_t iRef, int16_t iMVP[2]);

/*!
* \brief   Fill the spatial direct motion vectors for 8x8 direct MB
* \param
* \param   output motion vector cache and motion vector deviation cache
*/
void FillSpatialDirect8x8Mv (PDqLayer pCurDqLayer, const int16_t& iIdx8, const int8_t& iPartCount, const int8_t& iPartW,
                             const SubMbType& subMbType, const bool& bIsLongRef, int16_t pMvDirect[LIST_A][2], int8_t iRef[LIST_A],
                             int16_t pMotionVector[LIST_A][30][MV_A], int16_t pMvdCache[LIST_A][30][MV_A]);

/*!
* \brief   Fill the temporal direct motion vectors for 8x8 direct MB
* \param
* \param   output motion vector cache and motion vector deviation cache
*/
void FillTemporalDirect8x8Mv (PDqLayer pCurDqLayer, const int16_t& iIdx8, const int8_t& iPartCount,
                              const int8_t& iPartW,
                              const SubMbType& subMbType, int8_t iRef[LIST_A], int16_t (*mvColoc)[2],
                              int16_t pMotionVector[LIST_A][30][MV_A], int16_t pMvdCache[LIST_A][30][MV_A]);

/*!
* \brief   returns ref_index in List_0 from the colocated ref_index in LIST_0.
* \param
*  returns ref_index in List_0 of ref picture LIST_0
*/
int8_t MapColToList0 (PWelsDecoderContext& pCtx, const int8_t& colocRefIndexL0,
                      const int32_t& ref0Count); //ISO/IEC 14496-10:2009(E) (8-193)

/*!
* \brief     update ref_index cache for current MB, for 8x8
* \param
* \param
*/
void Update8x8RefIdx (PDqLayer& pCurDqLayer, const int16_t& iPartIdx, const int32_t& listIdx, const int8_t& iRef);

inline uint32_t* GetMbType (PDqLayer& pCurDqLayer) {
  if (pCurDqLayer->pDec != NULL) {
    return pCurDqLayer->pDec->pMbType;
  } else {
    return pCurDqLayer->pMbType;
  }
}

} // namespace WelsDec

#endif//WELS_MV_PRED_H__
