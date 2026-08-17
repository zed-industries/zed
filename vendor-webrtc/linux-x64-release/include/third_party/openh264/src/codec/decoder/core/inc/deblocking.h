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
 * \file    deblocking.h
 *
 * \brief   Interfaces introduced in frame deblocking filtering
 *
 * \date    05/14/2009 Created
 *
 *************************************************************************************
 */

#ifndef WELS_DEBLOCKING_H__
#define WELS_DEBLOCKING_H__

#include "decoder_context.h"
#include "deblocking_common.h"
namespace WelsDec {

/*!
 * \brief   deblocking module initialize
 *
 * \param   pf
 *          cpu
 *
 * \return  NONE
 */

void  DeblockingInit (PDeblockingFunc pDeblockingFunc,  int32_t iCpu);


/*!
 * \brief   deblocking filtering target slice
 *
 * \param   dec         Wels decoder context
 *
 * \return  NONE
 */
void WelsDeblockingFilterSlice (PWelsDecoderContext pCtx, PDeblockingFilterMbFunc pDeblockMb);

/*!
* \brief   AVC slice init deblocking filtering target layer
*
* \in and out param   SDeblockingFilter
* \in and out param   iFilterIdc
*
* \return  NONE
*/
void WelsDeblockingInitFilter (PWelsDecoderContext pCtx, SDeblockingFilter& pFilter, int32_t& iFilterIdc);

/*!
* \brief   AVC MB deblocking filtering target layer
*
* \param   DqLayer which has the current location of MB to be deblocked.
*
* \return  NONE
*/
void WelsDeblockingFilterMB (PDqLayer pCurDqLayer, SDeblockingFilter& pFilter, int32_t& iFilterIdc,
                             PDeblockingFilterMbFunc pDeblockMb);

/*!
 * \brief   pixel deblocking filtering
 *
 * \param   filter                deblocking filter
 * \param   pix                   pixel value
 * \param   stride                frame stride
 * \param   bs                    boundary strength
 *
 * \return  NONE
 */

uint32_t DeblockingBsMarginalMBAvcbase (PDeblockingFilter  pFilter, PDqLayer pCurDqLayer, int32_t iEdge,
                                        int32_t iNeighMb, int32_t iMbXy);
uint32_t DeblockingBSliceBsMarginalMBAvcbase (PDqLayer pCurDqLayer, int32_t iEdge, int32_t iNeighMb, int32_t iMbXy);

int32_t DeblockingAvailableNoInterlayer (PDqLayer pCurDqLayer, int32_t iFilterIdc);

void WelsDeblockingMb (PDqLayer pCurDqLayer, PDeblockingFilter  pFilter, int32_t iBoundryFlag);

inline int8_t* GetPNzc (PDqLayer pCurDqLayer, int32_t iMbXy) {
  if (pCurDqLayer->pDec != NULL && pCurDqLayer->pDec->pNzc != NULL) {
    return pCurDqLayer->pDec->pNzc[iMbXy];
  }
  return pCurDqLayer->pNzc[iMbXy];
}

} // namespace WelsDec

#endif //WELS_DEBLOCKING_H__

