/*!
 * \copy
 *     Copyright (c)  2013, Cisco Systems
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
 */

#ifndef WELS_VLC_ENCODER_H__
#define WELS_VLC_ENCODER_H__

#include "svc_enc_golomb.h"

/************************************************************************/
/* VLC FOR WELS ENCODER                                                 */
/************************************************************************/

namespace WelsEnc {

//g_kuiVlcCoeffToken[uiNc][total-coeff][trailing-ones][0--value, 1--bit count]
extern const uint8_t g_kuiVlcCoeffToken[5][17][4][2];
extern const uint8_t g_kuiVlcLevelPrefix[15][2];
//g_kuiVlcTotalZeros[tzVlcIndex][uiTotalZeros][0--value, 1--bit count]
extern const uint8_t g_kuiVlcTotalZeros[16][16][2];
extern const uint8_t g_kuiVlcTotalZerosChromaDc[4][4][2];
//add for mgs
extern const uint8_t g_kuiVlcTotalZerosChromaDc422[8][8][2];
//g_kuiVlcRunBefore[zeros-left][run-before][0--value, 1--bit count]
extern const uint8_t g_kuiVlcRunBefore[8][15][2];
extern const ALIGNED_DECLARE (uint8_t, g_kuiEncNcMapTable[18], 16);

#define    CHROMA_DC_NC_OFFSET       17

static inline int32_t WriteTotalCoeffTrailingones (SBitStringAux* pBs, uint8_t uiNc, uint8_t uiTotalCoeff,
    uint8_t uiTrailingOnes) {
const uint8_t kuiNcIdx      = g_kuiEncNcMapTable[uiNc];
const uint8_t* kpCoeffToken = &g_kuiVlcCoeffToken[kuiNcIdx][uiTotalCoeff][uiTrailingOnes][0];
return BsWriteBits (pBs,  kpCoeffToken[1], kpCoeffToken[0]);
}

static inline int32_t WriteTotalcoeffTrailingonesChroma (SBitStringAux* pBs, uint8_t uiTotalCoeff,
    uint8_t uiTrailingOnes) {
const uint8_t* kpCoeffToken = &g_kuiVlcCoeffToken[4][uiTotalCoeff][uiTrailingOnes][0];
return BsWriteBits (pBs, kpCoeffToken[1], kpCoeffToken[0]);
}

//kuiZeroCount = level_prefix;
static inline int32_t WriteLevelPrefix (SBitStringAux* pBs, const uint32_t kuiZeroCount) {
BsWriteBits (pBs, kuiZeroCount + 1, 1);
return 0;
}

static inline int32_t WriteTotalZeros (SBitStringAux* pBs, uint32_t uiTotalCoeff, uint32_t uiTotalZeros) {
const uint8_t* kpTotalZeros = &g_kuiVlcTotalZeros[uiTotalCoeff][uiTotalZeros][0];
return BsWriteBits (pBs, kpTotalZeros[1], kpTotalZeros[0]);
}

static inline int32_t WriteTotalZerosChromaDc (SBitStringAux* pBs, uint32_t uiTotalCoeff, uint32_t uiTotalZeros) {
const uint8_t* kpTotalZerosChromaDc = &g_kuiVlcTotalZerosChromaDc[uiTotalCoeff][uiTotalZeros][0];
return BsWriteBits (pBs, kpTotalZerosChromaDc[1], kpTotalZerosChromaDc[0]);
}

static inline int32_t WriteRunBefore (SBitStringAux* pBs, uint8_t uiZeroLeft, uint8_t uiRunBefore) {
const uint8_t* kpRunBefore = &g_kuiVlcRunBefore[uiZeroLeft][uiRunBefore][0];
return BsWriteBits (pBs, kpRunBefore[1], kpRunBefore[0]);
}
}
#endif
