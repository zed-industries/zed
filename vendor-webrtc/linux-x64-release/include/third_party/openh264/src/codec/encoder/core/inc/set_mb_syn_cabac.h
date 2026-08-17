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
 * \file    set_mb_syn_cabac.h
 *
 * \brief   Seting all syntax elements of mb and encoding residual with cabac
 *
 * \date    09/27/2014 Created
 *
 *************************************************************************************
 */

#ifndef SET_MB_SYN_CABAC_H_
#define SET_MB_SYN_CABAC_H_

#include "typedefs.h"
#include "wels_common_defs.h"

using namespace WelsCommon;

namespace WelsEnc {

#define  WELS_QP_MAX    51

typedef uint64_t cabac_low_t;
enum { CABAC_LOW_WIDTH = sizeof (cabac_low_t) / sizeof (uint8_t) * 8 };

typedef struct TagStateCtx {
  // Packed representation of state and MPS as state << 1 | MPS.
  uint8_t   m_uiStateMps;

  uint8_t Mps()   const { return m_uiStateMps  & 1; }
  uint8_t State() const { return m_uiStateMps >> 1; }
  void Set (uint8_t uiState, uint8_t uiMps) { m_uiStateMps = uiState * 2 + uiMps; }
} SStateCtx;
typedef struct TagCabacCtx {
  cabac_low_t m_uiLow;
  int32_t   m_iLowBitCnt;
  int32_t   m_iRenormCnt;
  uint32_t  m_uiRange;
  SStateCtx   m_sStateCtx[WELS_CONTEXT_COUNT];
  uint8_t*   m_pBufStart;
  uint8_t*   m_pBufEnd;
  uint8_t*   m_pBufCur;
} SCabacCtx;


void WelsCabacContextInit (void* pCtx, SCabacCtx* pCbCtx, int32_t iModel);
void WelsCabacEncodeInit (SCabacCtx* pCbCtx, uint8_t* pBuf,  uint8_t* pEnd);
inline void WelsCabacEncodeDecision (SCabacCtx* pCbCtx, int32_t iCtx, uint32_t uiBin);
inline void WelsCabacEncodeBypassOne (SCabacCtx* pCbCtx, int32_t uiBin);
void WelsCabacEncodeTerminate (SCabacCtx* pCbCtx, uint32_t uiBin);
void WelsCabacEncodeUeBypass (SCabacCtx* pCbCtx, int32_t iExpBits, uint32_t uiVal);
void WelsCabacEncodeFlush (SCabacCtx* pCbCtx);
uint8_t* WelsCabacEncodeGetPtr (SCabacCtx* pCbCtx);
int32_t  WriteBlockResidualCabac (void* pEncCtx,  int16_t* pCoffLevel, int32_t iEndIdx,
                                  int32_t iCalRunLevelFlag,
                                  int32_t iResidualProperty, int8_t iNC, SBitStringAux* pBs);


// private functions used by public inline functions.
void WelsCabacEncodeDecisionLps_ (SCabacCtx* pCbCtx, int32_t iCtx);
void WelsCabacEncodeUpdateLowNontrivial_ (SCabacCtx* pCbCtx);
inline void WelsCabacEncodeUpdateLow_ (SCabacCtx* pCbCtx) {
  if (pCbCtx->m_iLowBitCnt + pCbCtx->m_iRenormCnt < CABAC_LOW_WIDTH) {
    pCbCtx->m_iLowBitCnt  += pCbCtx->m_iRenormCnt;
    pCbCtx->m_uiLow      <<= pCbCtx->m_iRenormCnt;
  } else {
    WelsCabacEncodeUpdateLowNontrivial_ (pCbCtx);
  }
  pCbCtx->m_iRenormCnt = 0;
}

// inline function definitions.
void WelsCabacEncodeDecision (SCabacCtx* pCbCtx, int32_t iCtx, uint32_t uiBin) {
  if (uiBin == pCbCtx->m_sStateCtx[iCtx].Mps()) {
    const int32_t kiState = pCbCtx->m_sStateCtx[iCtx].State();
    uint32_t uiRange = pCbCtx->m_uiRange;
    uint32_t uiRangeLps = g_kuiCabacRangeLps[kiState][(uiRange & 0xff) >> 6];
    uiRange -= uiRangeLps;

    const int32_t kiRenormAmount = uiRange >> 8 ^ 1;
    pCbCtx->m_uiRange = uiRange << kiRenormAmount;
    pCbCtx->m_iRenormCnt += kiRenormAmount;
    pCbCtx->m_sStateCtx[iCtx].Set (g_kuiStateTransTable[kiState][1], uiBin);
  } else {
    WelsCabacEncodeDecisionLps_ (pCbCtx, iCtx);
  }
}

void WelsCabacEncodeBypassOne (SCabacCtx* pCbCtx, int32_t uiBin) {
  const uint32_t kuiBinBitmask = -uiBin;
  pCbCtx->m_iRenormCnt++;
  WelsCabacEncodeUpdateLow_ (pCbCtx);
  pCbCtx->m_uiLow += kuiBinBitmask & pCbCtx->m_uiRange;
}

}
#endif
