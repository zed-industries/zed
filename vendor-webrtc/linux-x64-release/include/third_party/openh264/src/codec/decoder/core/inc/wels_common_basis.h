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

//wels_common_basis.h
#ifndef WELS_COMMON_BASIS_H__
#define WELS_COMMON_BASIS_H__

#include "typedefs.h"
#include "macros.h"

#include "wels_common_defs.h"

using namespace WelsCommon;

namespace WelsDec {

/*common use table*/
extern const uint8_t g_kuiScan8[24];
extern const uint8_t g_kuiLumaDcZigzagScan[16];
extern const uint8_t g_kuiChromaDcScan[4];
extern const uint8_t g_kMbNonZeroCountIdx[24];
extern const uint8_t g_kCacheNzcScanIdx[4 * 4 + 4 + 4 + 3];
extern const uint8_t g_kCache26ScanIdx[16];
extern const uint8_t g_kCache30ScanIdx[16];
extern const uint8_t g_kNonZeroScanIdxC[4];
/* Profile IDC */
typedef uint8_t ProfileIdc;

/* Position Offset structure */
typedef struct TagPosOffset {
  int32_t iLeftOffset;
  int32_t iTopOffset;
  int32_t iRightOffset;
  int32_t iBottomOffset;
} SPosOffset;

/* MB Type & Sub-MB Type */
typedef uint32_t MbType;
typedef uint32_t SubMbType;

#define I16_LUMA_DC  1
#define I16_LUMA_AC  2
#define LUMA_DC_AC   3
#define CHROMA_DC    4
#define CHROMA_AC    5
#define LUMA_DC_AC_8  6
#define CHROMA_DC_U  7
#define CHROMA_DC_V  8
#define CHROMA_AC_U  9
#define CHROMA_AC_V  10
#define LUMA_DC_AC_INTRA 11
#define LUMA_DC_AC_INTER 12
#define CHROMA_DC_U_INTER  13
#define CHROMA_DC_V_INTER  14
#define CHROMA_AC_U_INTER  15
#define CHROMA_AC_V_INTER  16
#define LUMA_DC_AC_INTRA_8  17
#define LUMA_DC_AC_INTER_8  18

#define SHIFT_BUFFER(pBitsCache)        { pBitsCache->pBuf+=2; pBitsCache->uiRemainBits += 16; pBitsCache->uiCache32Bit |= (((pBitsCache->pBuf[2] << 8) | pBitsCache->pBuf[3]) << (32 - pBitsCache->uiRemainBits)); }
#define POP_BUFFER(pBitsCache, iCount)  { pBitsCache->uiCache32Bit <<= iCount;  pBitsCache->uiRemainBits -= iCount; }

static const uint8_t g_kuiZigzagScan[16] = { //4*4block residual zig-zag scan order
  0,  1,  4,  8,
  5,  2,  3,  6,
  9, 12, 13, 10,
  7, 11, 14, 15,
};

static const uint8_t g_kuiZigzagScan8x8[64] = { //8x8 block residual zig-zag scan order
  0,  1,  8,  16, 9,  2,  3,  10,
  17, 24, 32, 25, 18, 11, 4,  5,
  12, 19, 26, 33, 40, 48, 41, 34,
  27, 20, 13, 6,  7,  14, 21, 28,
  35, 42, 49, 56, 57, 50, 43, 36,
  29, 22, 15, 23, 30, 37, 44, 51,
  58, 59, 52, 45, 38, 31, 39, 46,
  53, 60, 61, 54, 47, 55, 62, 63,
};

static const uint8_t g_kuiIdx2CtxSignificantCoeffFlag8x8[64] = {  // Table 9-43, Page 289
  0,  1,  2,  3,  4,  5,  5,  4,
  4,  3,  3,  4,  4,  4,  5,  5,
  4,  4,  4,  4,  3,  3,  6,  7,
  7,  7,  8,  9, 10,  9,  8,  7,
  7,  6, 11, 12, 13, 11,  6,  7,
  8,  9, 14, 10,  9,  8,  6, 11,
  12, 13, 11, 6,  9, 14, 10,  9,
  11, 12, 13, 11, 14, 10, 12, 14,
};

static const uint8_t g_kuiIdx2CtxLastSignificantCoeffFlag8x8[64] = { // Table 9-43, Page 289
  0,  1,  1,  1,  1,  1,  1,  1,
  1,  1,  1,  1,  1,  1,  1,  1,
  2,  2,  2,  2,  2,  2,  2,  2,
  2,  2,  2,  2,  2,  2,  2,  2,
  3,  3,  3,  3,  3,  3,  3,  3,
  4,  4,  4,  4,  4,  4,  4,  4,
  5,  5,  5,  5,  6,  6,  6,  6,
  7,  7,  7,  7,  8,  8,  8,  8,
};

static inline void GetMbResProperty (int32_t* pMBproperty, int32_t* pResidualProperty, bool bCavlc) {
  switch (*pResidualProperty) {
  case CHROMA_AC_U:
    *pMBproperty = 1;
    *pResidualProperty = bCavlc ? CHROMA_AC : CHROMA_AC_U;
    break;
  case CHROMA_AC_V:
    *pMBproperty = 2;
    *pResidualProperty = bCavlc ? CHROMA_AC : CHROMA_AC_V;
    break;
  case LUMA_DC_AC_INTRA:
    *pMBproperty = 0;
    *pResidualProperty = LUMA_DC_AC;
    break;
  case CHROMA_DC_U:
    *pMBproperty = 1;
    *pResidualProperty =  bCavlc ? CHROMA_DC : CHROMA_DC_U;
    break;
  case CHROMA_DC_V:
    *pMBproperty = 2;
    *pResidualProperty =  bCavlc ? CHROMA_DC : CHROMA_DC_V;
    break;
  case I16_LUMA_AC:
    *pMBproperty = 0;
    break;
  case I16_LUMA_DC:
    *pMBproperty = 0;
    break;
  case LUMA_DC_AC_INTER:
    *pMBproperty = 3;
    *pResidualProperty = LUMA_DC_AC;
    break;
  case CHROMA_DC_U_INTER:
    *pMBproperty = 4;
    *pResidualProperty =  bCavlc ? CHROMA_DC : CHROMA_DC_U;
    break;
  case CHROMA_DC_V_INTER:
    *pMBproperty = 5;
    *pResidualProperty =  bCavlc ? CHROMA_DC : CHROMA_DC_V;
    break;
  case CHROMA_AC_U_INTER:
    *pMBproperty = 4;
    *pResidualProperty =  bCavlc ? CHROMA_AC : CHROMA_AC_U;
    break;
  case CHROMA_AC_V_INTER:
    *pMBproperty = 5;
    *pResidualProperty =  bCavlc ? CHROMA_AC : CHROMA_AC_V;
    break;
  // Reference to Table 7-2
  case LUMA_DC_AC_INTRA_8:
    *pMBproperty = 6;
    *pResidualProperty = LUMA_DC_AC_8;
    break;
  case LUMA_DC_AC_INTER_8:
    *pMBproperty = 7;
    *pResidualProperty = LUMA_DC_AC_8;
    break;
  }
}

typedef struct TagI16PredInfo {
  int8_t iPredMode;
  int8_t iLeftAvail;
  int8_t iTopAvail;
  int8_t iLeftTopAvail;
} SI16PredInfo;
static const SI16PredInfo g_ksI16PredInfo[4] = {
  {I16_PRED_V, 0, 1, 0},
  {I16_PRED_H, 1, 0, 0},
  {         0, 0, 0, 0},
  {I16_PRED_P, 1, 1, 1},
};

static const SI16PredInfo g_ksChromaPredInfo[4] = {
  {       0, 0, 0, 0},
  {C_PRED_H, 1, 0, 0},
  {C_PRED_V, 0, 1, 0},
  {C_PRED_P, 1, 1, 1},
};


typedef struct TagI4PredInfo {
  int8_t iPredMode;
  int8_t iLeftAvail;
  int8_t iTopAvail;
  int8_t iLeftTopAvail;
  // int8_t right_top_avail; //when right_top unavailable but top avail, we can pad the right-top with the rightmost pixel of top
} SI4PredInfo;
static const SI4PredInfo g_ksI4PredInfo[9] = {
  {  I4_PRED_V, 0, 1, 0},
  {  I4_PRED_H, 1, 0, 0},
  {          0, 0, 0, 0},
  {I4_PRED_DDL, 0, 1, 0},
  {I4_PRED_DDR, 1, 1, 1},
  { I4_PRED_VR, 1, 1, 1},
  { I4_PRED_HD, 1, 1, 1},
  { I4_PRED_VL, 0, 1, 0},
  { I4_PRED_HU, 1, 0, 0},
};

static const uint8_t g_kuiI16CbpTable[6] = {0, 16, 32, 15, 31, 47};


typedef struct TagPartMbInfo {
  MbType iType;
  int8_t iPartCount; //P_16*16, P_16*8, P_8*16, P_8*8 based on 8*8 block; P_8*4, P_4*8, P_4*4 based on 4*4 block
  int8_t iPartWidth; //based on 4*4 block
} SPartMbInfo;

//Table 7.13. Macroblock type values 0 to 4 for P slices.
static const SPartMbInfo g_ksInterPMbTypeInfo[5] = {
  {MB_TYPE_16x16,    1, 4},
  {MB_TYPE_16x8,     2, 4},
  {MB_TYPE_8x16,     2, 2},
  {MB_TYPE_8x8,      4, 4},
  {MB_TYPE_8x8_REF0, 4, 4}, //ref0--ref_idx not present in bit-stream and default as 0
};

//Table 7.14. Macroblock type values 0 to 22 for B slices.
static const SPartMbInfo g_ksInterBMbTypeInfo[] = {
  //            Part 0        Part 1
  { MB_TYPE_DIRECT, 1, 4 }, //B_Direct_16x16
  { MB_TYPE_16x16 | MB_TYPE_P0L0, 1, 4 }, //B_L0_16x16
  { MB_TYPE_16x16 | MB_TYPE_P0L1, 1, 4 }, //B_L1_16x16
  { MB_TYPE_16x16 | MB_TYPE_P0L0 | MB_TYPE_P0L1, 1, 4 },  //B_Bi_16x16
  { MB_TYPE_16x8  | MB_TYPE_P0L0 | MB_TYPE_P1L0, 2, 4 },    //B_L0_L0_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L0 | MB_TYPE_P1L0, 2, 2 },    //B_L0_L0_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L1 | MB_TYPE_P1L1, 2, 4 },    //B_L1_L1_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L1 | MB_TYPE_P1L1, 2, 2 },    //B_L1_L1_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L0 | MB_TYPE_P1L1, 2, 4 },    //B_L0_L1_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L0 | MB_TYPE_P1L1, 2, 2 },    //B_L0_L1_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L1 | MB_TYPE_P1L0, 2, 4 },    //B_L1_L0_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L1 | MB_TYPE_P1L0, 2, 2 },    //B_L1_L0_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L0 | MB_TYPE_P1L0 | MB_TYPE_P1L1, 2, 4 },   //B_L0_Bi_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L0 | MB_TYPE_P1L0 | MB_TYPE_P1L1, 2, 2 },   //B_L0_Bi_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L1 | MB_TYPE_P1L0 | MB_TYPE_P1L1, 2, 4 },   //B_L1_Bi_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L1 | MB_TYPE_P1L0 | MB_TYPE_P1L1, 2, 2 },   //B_L1_Bi_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L0 | MB_TYPE_P0L1 | MB_TYPE_P1L0, 2, 4 },   //B_Bi_L0_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L0 | MB_TYPE_P0L1 | MB_TYPE_P1L0, 2, 2 },   //B_Bi_L0_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L0 | MB_TYPE_P0L1 | MB_TYPE_P1L1, 2, 4 },   //B_Bi_L1_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L0 | MB_TYPE_P0L1 | MB_TYPE_P1L1, 2, 2 },   //B_Bi_L1_8x16
  { MB_TYPE_16x8  | MB_TYPE_P0L0 | MB_TYPE_P0L1 | MB_TYPE_P1L0 | MB_TYPE_P1L1, 2, 4 },    //B_Bi_Bi_16x8
  { MB_TYPE_8x16  | MB_TYPE_P0L0 | MB_TYPE_P0L1 | MB_TYPE_P1L0 | MB_TYPE_P1L1, 2, 2 },    //B_Bi_Bi_8x16
  { MB_TYPE_8x8   | MB_TYPE_P0L0 | MB_TYPE_P0L1 | MB_TYPE_P1L0 | MB_TYPE_P1L1,  4, 4 }    //B_8x8
};

//Table 7.17 Sub-macroblock types in B macroblocks.
static const SPartMbInfo g_ksInterPSubMbTypeInfo[4] = {
  {SUB_MB_TYPE_8x8, 1, 2},
  {SUB_MB_TYPE_8x4, 2, 2},
  {SUB_MB_TYPE_4x8, 2, 1},
  {SUB_MB_TYPE_4x4, 4, 1},
};

//Table 7.18 Sub-macroblock types in B macroblocks.
static const SPartMbInfo g_ksInterBSubMbTypeInfo[] = {
  { MB_TYPE_DIRECT,                               1, 2 }, //B_Direct_8x8
  { SUB_MB_TYPE_8x8 | MB_TYPE_P0L0,                 1, 2 }, //B_L0_8x8
  { SUB_MB_TYPE_8x8 | MB_TYPE_P0L1,                 1, 2 }, //B_L1_8x8
  { SUB_MB_TYPE_8x8 | MB_TYPE_P0L0 | MB_TYPE_P0L1,  1, 2 }, //B_Bi_8x8
  { SUB_MB_TYPE_8x4 | MB_TYPE_P0L0,                 2, 2 }, //B_L0_8x4
  { SUB_MB_TYPE_4x8 | MB_TYPE_P0L0,                 2, 1 }, //B_L0_4x8
  { SUB_MB_TYPE_8x4 | MB_TYPE_P0L1,                 2, 2 }, //B_L1_8x4
  { SUB_MB_TYPE_4x8 | MB_TYPE_P0L1,                 2, 1 }, //B_L1_4x8
  { SUB_MB_TYPE_8x4 | MB_TYPE_P0L0 | MB_TYPE_P0L1,  2, 2 }, //B_Bi_8x4
  { SUB_MB_TYPE_4x8 | MB_TYPE_P0L0 | MB_TYPE_P0L1,  2, 1 }, //B_Bi_4x8
  { SUB_MB_TYPE_4x4 | MB_TYPE_P0L0,                 4, 1 }, //B_L0_4x4
  { SUB_MB_TYPE_4x4 | MB_TYPE_P0L1,                 4, 1 }, //B_L1_4x4
  { SUB_MB_TYPE_4x4 | MB_TYPE_P0L0 | MB_TYPE_P0L1,  4, 1 }  //B_Bi_4x4
};

typedef struct TagSar {
  uint32_t uiWidth;
  uint32_t uiHeight;
} sSar;
static const sSar g_ksVuiSampleAspectRatio[17] = { //Table E-1
  { 0,  0}, { 1,  1}, {12, 11}, { 10, 11}, {16, 11}, //0~4
  {40, 33}, {24, 11}, {20, 11}, { 32, 11}, {80, 33}, //5~9
  {18, 11}, {15, 11}, {64, 33}, {160, 99}, { 4, 3}, //10~14
  { 3,  2}, { 2,  1}                                //15~16
};


} // namespace WelsDec

#endif//WELS_COMMON_BASIS_H__
