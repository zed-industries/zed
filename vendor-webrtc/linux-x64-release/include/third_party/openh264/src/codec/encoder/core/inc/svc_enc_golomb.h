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
 * \file    svc_enc_golomb.h
 *
 * \brief   Exponential Golomb entropy coding routine
 *
 * \date    03/13/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_EXPONENTIAL_GOLOMB_ENTROPY_CODING_H__
#define WELS_EXPONENTIAL_GOLOMB_ENTROPY_CODING_H__

#include "wels_common_defs.h"
#include "golomb_common.h"

using namespace WelsCommon;

namespace WelsEnc {

/************************************************************************/
/* GOLOMB CODIMG FOR WELS ENCODER ONLY                                  */
/************************************************************************/


/*
 *  Get size of unsigned exp golomb codes
 */
static inline uint32_t BsSizeUE (const uint32_t kiValue) {
if (256 > kiValue) {
  return g_kuiGolombUELength[kiValue];
} else {
  uint32_t n = 0;
  uint32_t iTmpValue = kiValue + 1;

  if (iTmpValue & 0xffff0000) {
    iTmpValue >>= 16;
    n += 16;
  }
  if (iTmpValue & 0xff00) {
    iTmpValue >>= 8;
    n += 8;
  }

  //n += (g_kuiGolombUELength[iTmpValue] >> 1);
  n += (g_kuiGolombUELength[iTmpValue - 1] >> 1);
  return ((n << 1) + 1);

}
}

/*
 *  Get size of signed exp golomb codes
 */
static inline uint32_t BsSizeSE (const int32_t kiValue) {
uint32_t iTmpValue;
if (0 == kiValue) {
  return 1;
} else if (0 < kiValue) {
  iTmpValue = (kiValue << 1) - 1;
  return BsSizeUE (iTmpValue);
} else {
  iTmpValue = ((-kiValue) << 1);
  return BsSizeUE (iTmpValue);
}
}

/*
 *  Write truncated exp golomb codes
 */
static inline void BsWriteTE (SBitStringAux* pBs, const int32_t kiX, const uint32_t kuiValue) {
if (1 == kiX) {
  BsWriteOneBit (pBs, !kuiValue);
} else {
  BsWriteUE (pBs, kuiValue);
}
}

static inline int32_t BsGetBitsPos (SBitStringAux* pBs) {
return (int32_t) (((pBs->pCurBuf - pBs->pStartBuf) << 3) + 32 - pBs->iLeftBits);
}

static inline void BsAlign( SBitStringAux* pBs )
{
   if( pBs->iLeftBits&7 )
   {
      pBs->uiCurBits <<= pBs->iLeftBits&7;
      pBs->uiCurBits |= (1 << (pBs->iLeftBits&7)) - 1;
      pBs->iLeftBits &= ~7;
   }
   BsFlush(pBs );
}
}
#endif//WELS_EXPONENTIAL_GOLOMB_ENTROPY_CODING_H__
