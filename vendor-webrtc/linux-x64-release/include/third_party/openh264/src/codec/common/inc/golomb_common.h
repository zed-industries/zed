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
 * \file    golomb_common.h
 *
 * \brief   Exponential Golomb entropy coding/decoding routine
 *
 * \date    03/12/2015 Created
 *
 *************************************************************************************
 */
#ifndef EXPONENTIAL_GOLOMB_ENTROPY_CODING_COMMON_H__
#define EXPONENTIAL_GOLOMB_ENTROPY_CODING_COMMON_H__

#include "typedefs.h"

namespace WelsCommon {

#define WRITE_BE_32(ptr, val) do { \
        (ptr)[0] = (val) >> 24; \
        (ptr)[1] = (val) >> 16; \
        (ptr)[2] = (val) >>  8; \
        (ptr)[3] = (val) >>  0; \
    } while (0)
/************************************************************************/
/* GOLOMB CODIMG FOR WELS COMMON                                        */
/************************************************************************/


/*!
 * \brief   initialize bitstream writing
 *
 * \param   pBs     Bit string auxiliary pointer
 * \param   pBuf    bit-stream pBuffer
 * \param   iSize   iSize in bits for decoder; iSize in bytes for encoder
 *
 * \return  iSize of pBuffer pData in byte; failed in -1 return
 */
static inline int32_t InitBits (SBitStringAux* pBs, const uint8_t* kpBuf, const int32_t kiSize) {
  uint8_t* ptr = (uint8_t*)kpBuf;

  pBs->pStartBuf = ptr;
  pBs->pCurBuf   = ptr;
  pBs->pEndBuf   = ptr + kiSize;
  pBs->iLeftBits = 32;
  pBs->uiCurBits = 0;

  return kiSize;
}

static inline int32_t BsWriteBits (PBitStringAux pBitString, int32_t iLen, const uint32_t kuiValue) {
  if (iLen < pBitString->iLeftBits) {
    pBitString->uiCurBits = (pBitString->uiCurBits << iLen) | kuiValue;
    pBitString->iLeftBits -= iLen;
  } else {
    iLen -= pBitString->iLeftBits;
    pBitString->uiCurBits = (pBitString->uiCurBits << pBitString->iLeftBits) | (kuiValue >> iLen);
    WRITE_BE_32 (pBitString->pCurBuf, pBitString->uiCurBits);
    pBitString->pCurBuf += 4;
    pBitString->uiCurBits = kuiValue & ((1 << iLen) - 1);
    pBitString->iLeftBits = 32 - iLen;
  }
  return 0;
}

/*
 *  Write 1 bit
 */
static inline int32_t BsWriteOneBit (PBitStringAux pBitString, const uint32_t kuiValue) {
  BsWriteBits (pBitString, 1, kuiValue);
  return 0;
}

static inline int32_t BsFlush (PBitStringAux pBitString) {
  WRITE_BE_32 (pBitString->pCurBuf, pBitString->uiCurBits << pBitString->iLeftBits);
  pBitString->pCurBuf += 4 - pBitString->iLeftBits / 8;
  pBitString->iLeftBits = 32;
  pBitString->uiCurBits = 0;
  return 0;
}

/*
 *  Write unsigned exp golomb codes
 */

static inline int32_t BsWriteUE (PBitStringAux pBitString, const uint32_t kuiValue) {
  uint32_t iTmpValue = kuiValue + 1;
  if (256 > kuiValue) {
    BsWriteBits (pBitString, g_kuiGolombUELength[kuiValue], kuiValue + 1);
  } else {
    uint32_t n = 0;
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
    BsWriteBits (pBitString, (n << 1) + 1, kuiValue + 1);
  }
  return 0;
}

/*
 *  Write signed exp golomb codes
 */
static inline int32_t BsWriteSE (PBitStringAux pBitString, const int32_t kiValue) {
  uint32_t iTmpValue;
  if (0 == kiValue) {
    BsWriteOneBit (pBitString, 1);
  } else if (0 < kiValue) {
    iTmpValue = (kiValue << 1) - 1;
    BsWriteUE (pBitString, iTmpValue);
  } else {
    iTmpValue = ((-kiValue) << 1);
    BsWriteUE (pBitString, iTmpValue);
  }
  return 0;
}


/*
 *  Write RBSP trailing bits
 */
static inline int32_t BsRbspTrailingBits (PBitStringAux pBitString) {
  BsWriteOneBit (pBitString, 1);
  BsFlush (pBitString);

  return 0;
}

}
#endif//EXPONENTIAL_GOLOMB_ENTROPY_CODING_COMMON_H__
