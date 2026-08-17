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
 * \file    au_set.h
 *
 * \brief   Interfaces introduced in Access Unit level based writer
 *
 * \date    05/18/2009 Created
 *          05/21/2009 Added init_sps and init_pps
 *
 *************************************************************************************
 */

#ifndef WELS_ACCESS_UNIT_WRITER_H__
#define WELS_ACCESS_UNIT_WRITER_H__

#include "parameter_sets.h"
#include "paraset_strategy.h"
#include "param_svc.h"
#include "utils.h"
namespace WelsEnc {
/*!
 *************************************************************************************
 * \brief   to write Sequence Parameter Set (SPS)
 *
 * \param   pSps        SWelsSPS to be wrote
 * \param   bs_aux      bitstream writer auxiliary
 *
 * \return  0 - successed
 *          1 - failed
 *
 * \note    Call it in case EWelsNalUnitType is SPS.
 *************************************************************************************
 */

int32_t WelsWriteSpsNal (SWelsSPS* pSps, SBitStringAux* pBitStringAux, int32_t* pSpsIdDelta);


/*!
 *************************************************************************************
 * \brief   to write SubSet Sequence Parameter Set
 *
 * \param   sub_sps     subset pSps parsed
 * \param   bs_aux      bitstream writer auxiliary
 *
 * \return  0 - successed
 *          1 - failed
 *
 * \note    Call it in case EWelsNalUnitType is SubSet SPS.
 *************************************************************************************
 */
int32_t WelsWriteSubsetSpsSyntax (SSubsetSps* pSubsetSps, SBitStringAux* pBitStringAux , int32_t* pSpsIdDelta);


/*!
 *************************************************************************************
 * \brief   to write Picture Parameter Set (PPS)
 *
 * \param   pPps        pPps
 * \param   bs_aux      bitstream writer auxiliary
 *
 * \return  0 - successed
 *          1 - failed
 *
 * \note    Call it in case EWelsNalUnitType is PPS.
 *************************************************************************************
 */
int32_t WelsWritePpsSyntax (SWelsPPS* pPps, SBitStringAux* pBitStringAux, IWelsParametersetStrategy* pParametersetStrategy);

/*!
 * \brief   initialize pSps based on configurable parameters in svc
 * \param   pSps                SWelsSPS*
 * \param   pLayerParam         SSpatialLayerConfig  dependency layer parameter
 * \param   pLayerParamInternal SSpatialLayerInternal*, internal dependency layer parameter
 * \param   iSpsId              SPS Id
 * \return  0 - successful
 *          1 - failed
 */
int32_t WelsInitSps (SWelsSPS* pSps, SSpatialLayerConfig* pLayerParam, SSpatialLayerInternal* pLayerParamInternal,
                     const uint32_t kuiIntraPeriod, const int32_t kiNumRefFrame,
                     const uint32_t kiSpsId, const bool kbEnableFrameCropping, bool bEnableRc,
                     const int32_t kiDlayerCount,bool bSVCBaselayer);

/*!
 * \brief   initialize subset pSps based on configurable parameters in svc
 * \param   pSubsetSps          SSubsetSps*
 * \param   pLayerParam         SSpatialLayerConfig  dependency layer parameter
 * \param   pLayerParamInternal SSpatialLayerInternal*, internal dependency layer parameter
 * \param   kiSpsId             SPS Id
 * \return  0 - successful
 *          1 - failed
 */
int32_t WelsInitSubsetSps (SSubsetSps* pSubsetSps, SSpatialLayerConfig* pLayerParam,
                           SSpatialLayerInternal* pLayerParamInternal,
                           const uint32_t kuiIntraPeriod, const int32_t kiNumRefFrame,
                           const uint32_t kiSpsId, const bool kbEnableFrameCropping, bool bEnableRc,
                           const int32_t kiDlayerCount);

/*!
 * \brief   initialize pPps based on configurable parameters and pSps(subset pSps) in svc
 * \param   pPps                            SWelsPPS*
 * \param   pSps                            SWelsSPS*
 * \param   pSubsetSps                      SSubsetSps*
 * \param   kbDeblockingFilterPresentFlag   bool
 * \param   kiPpsId                         PPS Id
 * \param   kbUsingSubsetSps                bool
 * \return  0 - successful
 *          1 - failed
 */
int32_t WelsInitPps (SWelsPPS* pPps,
                     SWelsSPS* pSps,
                     SSubsetSps* pSubsetSps,
                     const uint32_t kuiPpsId,
                     const bool kbDeblockingFilterPresentFlag,
                     const bool kbUsingSubsetSps,
                     const bool kbEntropyCodingModeFlag);

int32_t WelsCheckRefFrameLimitationNumRefFirst (SLogContext* pLogCtx, SWelsSvcCodingParam* pParam);
int32_t WelsCheckRefFrameLimitationLevelIdcFirst (SLogContext* pLogCtx, SWelsSvcCodingParam* pParam);

int32_t WelsAdjustLevel (SSpatialLayerConfig* pSpatialLayer,const SLevelLimits *pCurLevel);

}
#endif//WELS_ACCESS_UNIT_PARSER_H__
