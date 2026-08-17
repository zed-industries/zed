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
 * \file    fmo.h
 *
 * \brief   Flexible Macroblock Ordering implementation
 *
 * \date    2/4/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_FLEXIBLE_MACROBLOCK_ORDERING_H__
#define WELS_FLEXIBLE_MACROBLOCK_ORDERING_H__

#include "typedefs.h"
#include "wels_const.h"
#include "parameter_sets.h"
#include "memory_align.h"

namespace WelsDec {

#ifndef MB_XY_T
#define MB_XY_T int32_t
#endif//MB_XY_T

/*!
 * \brief   Wels Flexible Macroblock Ordering (FMO)
 */
typedef struct TagFmo {
  uint8_t*        pMbAllocMap;
  int32_t         iCountMbNum;
  int32_t         iSliceGroupCount;
  int32_t         iSliceGroupType;
  bool            bActiveFlag;
  uint8_t         uiReserved[3];          // reserved padding bytes
} SFmo, *PFmo;


/*!
 * \brief   Initialize Wels Flexible Macroblock Ordering (FMO)
 *
 * \param   pFmo        Wels fmo to be initialized
 * \param   pPps        PPps
 * \param   kiMbWidth   mb width
 * \param   kiMbHeight  mb height
 *
 * \return  0 - successful; none 0 - failed;
 */
int32_t InitFmo (PFmo pFmo, PPps pPps, const int32_t kiMbWidth, const int32_t kiMbHeight, CMemoryAlign* pMa);

/*!
 * \brief   Uninitialize Wels Flexible Macroblock Ordering (FMO) list
 *
 * \param   pFmo        Wels base fmo ptr to be uninitialized
 * \param   kiCnt       count number of PPS per list
 * \param   kiAvail     count available number of PPS in list
 *
 * \return  NONE
 */
void UninitFmoList (PFmo pFmo, const int32_t kiCnt, const int32_t kiAvail, CMemoryAlign* pMa);

/*!
 * \brief   update/insert FMO parameter unit
 *
 * \param   pFmo    FMO context
 * \param   pSps    PSps
 * \param   pPps    PPps
 * \param   pActiveFmoNum   int32_t* [in/out]
 *
 * \return  true - update/insert successfully; false - failed;
 */
int32_t FmoParamUpdate (PFmo pFmo, PSps pSps, PPps pPps, int32_t* pActiveFmoNum, CMemoryAlign* pMa);

/*!
 * \brief   Get successive mb to be processed with given current mb_xy
 *
 * \param   pFmo            Wels fmo context
 * \param   iMbXy           current mb_xy
 *
 * \return  iNextMb - successful; -1 - failed;
 */
MB_XY_T FmoNextMb (PFmo pFmo, const MB_XY_T kiMbXy);

} // namespace WelsDec

#endif//WELS_FLEXIBLE_MACROBLOCK_ORDERING_H__
