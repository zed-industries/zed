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

#ifndef WELS_VLC_DECODER_H__
#define WELS_VLC_DECODER_H__

#include "bit_stream.h"
#include "dec_golomb.h"

namespace WelsDec {

typedef struct TagVlcTable {
const uint8_t (*kpCoeffTokenVlcTable[4][8])[2];
const uint8_t (*kpChromaCoeffTokenVlcTable)[2];
const uint8_t (*kpZeroTable[7])[2];
const uint8_t (*kpTotalZerosTable[2][15])[2];
} SVlcTable;

// for data sharing cross modules and try to reduce size of binary generated
extern const uint8_t g_kuiVlcChromaTable[256][2];
extern const uint8_t g_kuiVlcTable_0[256][2];
extern const uint8_t g_kuiVlcTable_0_0[256][2];
extern const uint8_t g_kuiVlcTable_0_1[4][2];
extern const uint8_t g_kuiVlcTable_0_2[2][2];
extern const uint8_t g_kuiVlcTable_0_3[2][2];
extern const uint8_t g_kuiVlcTable_1[256][2];
extern const uint8_t g_kuiVlcTable_1_0[64][2];
extern const uint8_t g_kuiVlcTable_1_1[8][2];
extern const uint8_t g_kuiVlcTable_1_2[2][2];
extern const uint8_t g_kuiVlcTable_1_3[2][2];
extern const uint8_t g_kuiVlcTable_2[256][2];
extern const uint8_t g_kuiVlcTable_2_0[4][2];
extern const uint8_t g_kuiVlcTable_2_1[4][2];
extern const uint8_t g_kuiVlcTable_2_2[4][2];
extern const uint8_t g_kuiVlcTable_2_3[4][2];
extern const uint8_t g_kuiVlcTable_2_4[2][2];
extern const uint8_t g_kuiVlcTable_2_5[2][2];
extern const uint8_t g_kuiVlcTable_2_6[2][2];
extern const uint8_t g_kuiVlcTable_2_7[2][2];
extern const uint8_t g_kuiVlcTable_3[64][2];
extern const uint8_t g_kuiVlcTableNeedMoreBitsThread[3];
extern const uint8_t g_kuiVlcTableMoreBitsCount0[4];
extern const uint8_t g_kuiVlcTableMoreBitsCount1[4];
extern const uint8_t g_kuiVlcTableMoreBitsCount2[8];
extern const uint8_t g_kuiNcMapTable[17];
extern const uint8_t g_kuiVlcTrailingOneTotalCoeffTable[62][2];
extern const uint8_t g_kuiTotalZerosTable0[512][2];
extern const uint8_t g_kuiTotalZerosTable1[64][2];
extern const uint8_t g_kuiTotalZerosTable2[64][2];
extern const uint8_t g_kuiTotalZerosTable3[32][2];
extern const uint8_t g_kuiTotalZerosTable4[32][2];
extern const uint8_t g_kuiTotalZerosTable5[64][2];
extern const uint8_t g_kuiTotalZerosTable6[64][2];
extern const uint8_t g_kuiTotalZerosTable7[64][2];
extern const uint8_t g_kuiTotalZerosTable8[64][2];
extern const uint8_t g_kuiTotalZerosTable9[32][2];
extern const uint8_t g_kuiTotalZerosTable10[16][2];
extern const uint8_t g_kuiTotalZerosTable11[16][2];
extern const uint8_t g_kuiTotalZerosTable12[8][2];
extern const uint8_t g_kuiTotalZerosTable13[4][2];
extern const uint8_t g_kuiTotalZerosTable14[2][2];
extern const uint8_t g_kuiTotalZerosBitNumMap[15];
extern const uint8_t g_kuiTotalZerosChromaTable0[8][2];
extern const uint8_t g_kuiTotalZerosChromaTable1[4][2];
extern const uint8_t g_kuiTotalZerosChromaTable2[2][2];
extern const uint8_t g_kuiTotalZerosBitNumChromaMap[3];
extern const uint8_t g_kuiZeroLeftTable0[2][2];
extern const uint8_t g_kuiZeroLeftTable1[4][2];
extern const uint8_t g_kuiZeroLeftTable2[4][2];
extern const uint8_t g_kuiZeroLeftTable3[8][2];
extern const uint8_t g_kuiZeroLeftTable4[8][2];
extern const uint8_t g_kuiZeroLeftTable5[8][2];
extern const uint8_t g_kuiZeroLeftTable6[8][2];
extern const uint8_t g_kuiZeroLeftBitNumMap[16];

#if defined(_MSC_VER) && defined(_M_IX86)
//TODO need linux version
#define WELS_GET_PREFIX_BITS(inval,outval){\
  uint32_t local = inval;\
  __asm xor       eax,    eax\
  __asm bsr       eax,    local\
  __asm sub       eax,    32\
  __asm neg       eax\
  __asm mov       outval, eax\
}
#else
#define WELS_GET_PREFIX_BITS(inval, outval) outval = GetPrefixBits(inval)
#endif

static inline void InitVlcTable (SVlcTable* pVlcTable) {
pVlcTable->kpChromaCoeffTokenVlcTable = g_kuiVlcChromaTable;

pVlcTable->kpCoeffTokenVlcTable[0][0] = g_kuiVlcTable_0;
pVlcTable->kpCoeffTokenVlcTable[0][1] = g_kuiVlcTable_1;
pVlcTable->kpCoeffTokenVlcTable[0][2] = g_kuiVlcTable_2;
pVlcTable->kpCoeffTokenVlcTable[0][3] = g_kuiVlcTable_3;

pVlcTable->kpCoeffTokenVlcTable[1][0] = g_kuiVlcTable_0_0;
pVlcTable->kpCoeffTokenVlcTable[1][1] = g_kuiVlcTable_0_1;
pVlcTable->kpCoeffTokenVlcTable[1][2] = g_kuiVlcTable_0_2;
pVlcTable->kpCoeffTokenVlcTable[1][3] = g_kuiVlcTable_0_3;

pVlcTable->kpCoeffTokenVlcTable[2][0] = g_kuiVlcTable_1_0;
pVlcTable->kpCoeffTokenVlcTable[2][1] = g_kuiVlcTable_1_1;
pVlcTable->kpCoeffTokenVlcTable[2][2] = g_kuiVlcTable_1_2;
pVlcTable->kpCoeffTokenVlcTable[2][3] = g_kuiVlcTable_1_3;

pVlcTable->kpCoeffTokenVlcTable[3][0] = g_kuiVlcTable_2_0;
pVlcTable->kpCoeffTokenVlcTable[3][1] = g_kuiVlcTable_2_1;
pVlcTable->kpCoeffTokenVlcTable[3][2] = g_kuiVlcTable_2_2;
pVlcTable->kpCoeffTokenVlcTable[3][3] = g_kuiVlcTable_2_3;
pVlcTable->kpCoeffTokenVlcTable[3][4] = g_kuiVlcTable_2_4;
pVlcTable->kpCoeffTokenVlcTable[3][5] = g_kuiVlcTable_2_5;
pVlcTable->kpCoeffTokenVlcTable[3][6] = g_kuiVlcTable_2_6;
pVlcTable->kpCoeffTokenVlcTable[3][7] = g_kuiVlcTable_2_7;

pVlcTable->kpZeroTable[0] = g_kuiZeroLeftTable0;
pVlcTable->kpZeroTable[1] = g_kuiZeroLeftTable1;
pVlcTable->kpZeroTable[2] = g_kuiZeroLeftTable2;
pVlcTable->kpZeroTable[3] = g_kuiZeroLeftTable3;
pVlcTable->kpZeroTable[4] = g_kuiZeroLeftTable4;
pVlcTable->kpZeroTable[5] = g_kuiZeroLeftTable5;
pVlcTable->kpZeroTable[6] = g_kuiZeroLeftTable6;

pVlcTable->kpTotalZerosTable[0][0] = g_kuiTotalZerosTable0;
pVlcTable->kpTotalZerosTable[0][1] = g_kuiTotalZerosTable1;
pVlcTable->kpTotalZerosTable[0][2] = g_kuiTotalZerosTable2;
pVlcTable->kpTotalZerosTable[0][3] = g_kuiTotalZerosTable3;
pVlcTable->kpTotalZerosTable[0][4] = g_kuiTotalZerosTable4;
pVlcTable->kpTotalZerosTable[0][5] = g_kuiTotalZerosTable5;
pVlcTable->kpTotalZerosTable[0][6] = g_kuiTotalZerosTable6;
pVlcTable->kpTotalZerosTable[0][7] = g_kuiTotalZerosTable7;
pVlcTable->kpTotalZerosTable[0][8] = g_kuiTotalZerosTable8;
pVlcTable->kpTotalZerosTable[0][9] = g_kuiTotalZerosTable9;
pVlcTable->kpTotalZerosTable[0][10] = g_kuiTotalZerosTable10;
pVlcTable->kpTotalZerosTable[0][11] = g_kuiTotalZerosTable11;
pVlcTable->kpTotalZerosTable[0][12] = g_kuiTotalZerosTable12;
pVlcTable->kpTotalZerosTable[0][13] = g_kuiTotalZerosTable13;
pVlcTable->kpTotalZerosTable[0][14] = g_kuiTotalZerosTable14;
pVlcTable->kpTotalZerosTable[1][0] = g_kuiTotalZerosChromaTable0;
pVlcTable->kpTotalZerosTable[1][1] = g_kuiTotalZerosChromaTable1;
pVlcTable->kpTotalZerosTable[1][2] = g_kuiTotalZerosChromaTable2;

}

} // namespace WelsDec

#endif//WELS_VLC_DECODER_H__
