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
 * \file    set_mb_syn_cavlc.h
 *
 * \brief   Seting all syntax elements of mb and decoding residual with cavlc
 *
 * \date    05/19/2009 Created
 *
 *************************************************************************************
 */

#ifndef SET_MB_SYN_CAVLC_H_
#define SET_MB_SYN_CAVLC_H_

#include "typedefs.h"
#include "wels_func_ptr_def.h"

namespace WelsEnc {


enum ECtxBlockCat {
  LUMA_DC     = 0,
  LUMA_AC     = 1,
  LUMA_4x4    = 2,
  CHROMA_DC   = 3,
  CHROMA_AC   = 4
};


#define LUMA_DC_AC    0x04

typedef struct TagCavlcTableItem {
  uint16_t uiBits;
  uint8_t  uiLen;
  uint8_t  uiSuffixLength;
} SCavlcTableItem;

void  InitCoeffFunc (SWelsFuncPtrList* pFuncList, const uint32_t uiCpuFlag,int32_t iEntropyCodingModeFlag);

int32_t  WriteBlockResidualCavlc (SWelsFuncPtrList* pFuncList, int16_t* pCoffLevel, int32_t iEndIdx,
                                  int32_t iCalRunLevelFlag,
                                  int32_t iResidualProperty, int8_t iNC, SBitStringAux* pBs);


#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

int32_t CavlcParamCal_c (int16_t* pCoffLevel, uint8_t* pRun, int16_t* pLevel, int32_t* pTotalCoeffs ,
                         int32_t iEndIdx);
#ifdef  X86_ASM
int32_t CavlcParamCal_sse2 (int16_t* pCoffLevel, uint8_t* pRun, int16_t* pLevel, int32_t* pTotalCoeffs ,
                            int32_t iEndIdx);
int32_t CavlcParamCal_sse42 (int16_t* pCoffLevel, uint8_t* pRun, int16_t* pLevel, int32_t* pTotalCoeffs ,
                             int32_t iEndIdx);
#endif

#if defined(__cplusplus)
}
#endif//__cplusplus

}
#endif
