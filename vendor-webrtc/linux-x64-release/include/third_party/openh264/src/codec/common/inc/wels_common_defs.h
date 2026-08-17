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

//wels_common_defs.h
#ifndef WELS_COMMON_DEFS_H__
#define WELS_COMMON_DEFS_H__

#include "typedefs.h"
#include "macros.h"
#include "codec_app_def.h"


namespace WelsCommon {
/*common use table*/

#define  CTX_NA 0
#define  WELS_CONTEXT_COUNT 460
#define LEVEL_NUMBER 17
typedef struct TagLevelLimits {
  ELevelIdc uiLevelIdc;  // level idc
  uint32_t uiMaxMBPS; // Max macroblock processing rate(MB/s)
  uint32_t uiMaxFS;   // Max frame sizea(MBs)
  uint32_t uiMaxDPBMbs;// Max decoded picture buffer size(MBs)
  uint32_t uiMaxBR; // Max video bit rate
  uint32_t uiMaxCPB; // Max CPB size
  int16_t iMinVmv; // Vertical MV component range upper bound
  int16_t iMaxVmv; // Vertical MV component range lower bound
  uint16_t uiMinCR;  // Min compression ration
  int16_t iMaxMvsPer2Mb; // Max number of motion vectors per two consecutive MBs
} SLevelLimits;

#define CpbBrNalFactor 1200  //baseline,main,and extended profiles.
extern const SLevelLimits g_ksLevelLimits[LEVEL_NUMBER];
extern const uint32_t g_kuiLevelMaps[LEVEL_NUMBER];
extern const uint8_t g_kuiMbCountScan4Idx[24];
extern const uint8_t g_kuiCache30ScanIdx[16];
extern const uint8_t g_kuiCache48CountScan4Idx[24];

extern const uint8_t g_kuiMatrixV[6][8][8];

extern const uint8_t g_kuiDequantScaling4x4Default[2][16];
extern const uint8_t g_kuiDequantScaling8x8Default[2][64];
extern const ALIGNED_DECLARE (uint16_t, g_kuiDequantCoeff[52][8], 16);
extern const ALIGNED_DECLARE (uint16_t, g_kuiDequantCoeff8x8[52][64], 16);
extern const uint8_t g_kuiChromaQpTable[52];

extern const uint8_t g_kuiCabacRangeLps[64][4];
extern const int8_t g_kiCabacGlobalContextIdx[WELS_CONTEXT_COUNT][4][2];
extern const uint8_t g_kuiStateTransTable[64][2];
extern const uint32_t g_kuiGolombUELength[256];
/*
 *  NAL Unit Type (5 Bits)
 */
enum EWelsNalUnitType {
  NAL_UNIT_UNSPEC_0             = 0,
  NAL_UNIT_CODED_SLICE          = 1,
  NAL_UNIT_CODED_SLICE_DPA      = 2,
  NAL_UNIT_CODED_SLICE_DPB      = 3,
  NAL_UNIT_CODED_SLICE_DPC      = 4,
  NAL_UNIT_CODED_SLICE_IDR      = 5,
  NAL_UNIT_SEI                  = 6,
  NAL_UNIT_SPS                  = 7,
  NAL_UNIT_PPS                  = 8,
  NAL_UNIT_AU_DELIMITER         = 9,
  NAL_UNIT_END_OF_SEQ           = 10,
  NAL_UNIT_END_OF_STR           = 11,
  NAL_UNIT_FILLER_DATA          = 12,
  NAL_UNIT_SPS_EXT              = 13,
  NAL_UNIT_PREFIX               = 14,
  NAL_UNIT_SUBSET_SPS           = 15,
  NAL_UNIT_DEPTH_PARAM          = 16, // NAL_UNIT_RESV_16
  NAL_UNIT_RESV_17              = 17,
  NAL_UNIT_RESV_18              = 18,
  NAL_UNIT_AUX_CODED_SLICE      = 19,
  NAL_UNIT_CODED_SLICE_EXT      = 20,
  NAL_UNIT_MVC_SLICE_EXT        = 21, // NAL_UNIT_RESV_21
  NAL_UNIT_RESV_22              = 22,
  NAL_UNIT_RESV_23              = 23,
  NAL_UNIT_UNSPEC_24            = 24,
  NAL_UNIT_UNSPEC_25            = 25,
  NAL_UNIT_UNSPEC_26            = 26,
  NAL_UNIT_UNSPEC_27            = 27,
  NAL_UNIT_UNSPEC_28            = 28,
  NAL_UNIT_UNSPEC_29            = 29,
  NAL_UNIT_UNSPEC_30            = 30,
  NAL_UNIT_UNSPEC_31            = 31
};

/*
 *  NAL Reference IDC (2 Bits)
 */

enum EWelsNalRefIdc {
  NRI_PRI_LOWEST        = 0,
  NRI_PRI_LOW           = 1,
  NRI_PRI_HIGH          = 2,
  NRI_PRI_HIGHEST       = 3
};

/*
 * VCL TYPE
 */

enum EVclType {
  NON_VCL   = 0,
  VCL       = 1,
  NOT_APP   = 2
};

/*
 *  vcl type map for given NAL unit type and corresponding H264 type (0: AVC; 1: SVC).
 */
extern const EVclType g_keTypeMap[32][2];

#define IS_VCL_NAL(t, ext_idx)                  (g_keTypeMap[t][ext_idx] == VCL)
#define IS_PARAM_SETS_NALS(t)                   ( (t) == NAL_UNIT_SPS || (t) == NAL_UNIT_PPS || (t) == NAL_UNIT_SUBSET_SPS )
#define IS_SPS_NAL(t)                           ( (t) == NAL_UNIT_SPS )
#define IS_SUBSET_SPS_NAL(t)                    ( (t) == NAL_UNIT_SUBSET_SPS )
#define IS_PPS_NAL(t)                           ( (t) == NAL_UNIT_PPS )
#define IS_SEI_NAL(t)                           ( (t) == NAL_UNIT_SEI )
#define IS_AU_DELIMITER_NAL(t)                  ( (t) == NAL_UNIT_AU_DELIMITER )
#define IS_PREFIX_NAL(t)                        ( (t) == NAL_UNIT_PREFIX )
#define IS_SUBSET_SPS_USED(t)                   ( (t) == NAL_UNIT_SUBSET_SPS || (t) == NAL_UNIT_CODED_SLICE_EXT )
#define IS_VCL_NAL_AVC_BASE(t)                  ( (t) == NAL_UNIT_CODED_SLICE || (t) == NAL_UNIT_CODED_SLICE_IDR )
#define IS_NEW_INTRODUCED_SVC_NAL(t)            ( (t) == NAL_UNIT_PREFIX || (t) == NAL_UNIT_CODED_SLICE_EXT )


/* Base SSlice Types
 * Invalid in case of eSliceType exceeds 9,
 * Need trim when eSliceType > 4 as fixed SliceType(eSliceType-4),
 * meaning mapped version after eSliceType minus 4.
 */

enum EWelsSliceType {
  P_SLICE       = 0,
  B_SLICE       = 1,
  I_SLICE       = 2,
  SP_SLICE      = 3,
  SI_SLICE      = 4,
  UNKNOWN_SLICE = 5
};

/* SSlice Types in scalable extension */
enum ESliceTypeExt {
  EP_SLICE = 0, // EP_SLICE: 0, 5
  EB_SLICE = 1, // EB_SLICE: 1, 6
  EI_SLICE = 2  // EI_SLICE: 2, 7
};

/* List Index */
enum EListIndex {
  LIST_0    = 0,
  LIST_1    = 1,
  LIST_A    = 2
};



/* Motion Vector components */
enum EMvComp {
  MV_X  = 0,
  MV_Y  = 1,
  MV_A  = 2
};

/* Chroma Components */

enum EChromaComp {
  CHROMA_CB     = 0,
  CHROMA_CR     = 1,
  CHROMA_A      = 2
};



/*
 *  Memory Management Control Operation (MMCO) code
 */
enum EMmcoCode {
  MMCO_END          = 0,
  MMCO_SHORT2UNUSED = 1,
  MMCO_LONG2UNUSED  = 2,
  MMCO_SHORT2LONG   = 3,
  MMCO_SET_MAX_LONG = 4,
  MMCO_RESET        = 5,
  MMCO_LONG         = 6
};

enum EVuiVideoFormat {
  VUI_COMPONENT   = 0,
  VUI_PAL         = 1,
  VUI_NTSC        = 2,
  VUI_SECAM       = 3,
  VUI_MAC         = 4,
  VUI_UNSPECIFIED = 5,
  VUI_RESERVED1   = 6,
  VUI_RESERVED2   = 7
};

/*
 *  Bit-stream auxiliary reading / writing
 */
typedef struct TagBitStringAux {
  uint8_t* pStartBuf;   // buffer to start position
  uint8_t* pEndBuf;     // buffer + length
  int32_t  iBits;       // count bits of overall bitstreaming input

  intX_t   iIndex;      //only for cavlc usage
  uint8_t* pCurBuf;     // current reading position
  uint32_t uiCurBits;
  int32_t  iLeftBits;   // count number of available bits left ([1, 8]),
  // need pointer to next byte start position in case 0 bit left then 8 instead
} SBitStringAux, *PBitStringAux;

/* NAL Unix Header in AVC, refer to Page 56 in JVT X201wcm */
typedef struct TagNalUnitHeader {
  uint8_t             uiForbiddenZeroBit;
  uint8_t             uiNalRefIdc;
  EWelsNalUnitType    eNalUnitType;
  uint8_t             uiReservedOneByte;                // only padding usage
} SNalUnitHeader, *PNalUnitHeader;

/* NAL Unit Header in scalable extension syntax, refer to Page 390 in JVT X201wcm */
typedef struct TagNalUnitHeaderExt {
  SNalUnitHeader      sNalUnitHeader;

  // uint8_t   reserved_one_bit;
  bool      bIdrFlag;
  uint8_t   uiPriorityId;
  int8_t    iNoInterLayerPredFlag;      // change as int8_t to support 3 values probably in encoder
  uint8_t   uiDependencyId;

  uint8_t   uiQualityId;
  uint8_t   uiTemporalId;
  bool      bUseRefBasePicFlag;
  bool      bDiscardableFlag;

  bool      bOutputFlag;
  uint8_t   uiReservedThree2Bits;
  // Derived variable(s)
  uint8_t   uiLayerDqId;
  bool      bNalExtFlag;
} SNalUnitHeaderExt, *PNalUnitHeaderExt;

/* AVC MB types*/
#define MB_TYPE_INTRA4x4    0x00000001
#define MB_TYPE_INTRA16x16  0x00000002
#define MB_TYPE_INTRA8x8    0x00000004
#define MB_TYPE_16x16       0x00000008
#define MB_TYPE_16x8        0x00000010
#define MB_TYPE_8x16        0x00000020
#define MB_TYPE_8x8         0x00000040
#define MB_TYPE_8x8_REF0    0x00000080
#define MB_TYPE_SKIP        0x00000100
#define MB_TYPE_INTRA_PCM   0x00000200
#define MB_TYPE_INTRA_BL    0x00000400
#define MB_TYPE_DIRECT      0x00000800
#define MB_TYPE_P0L0        0x00001000
#define MB_TYPE_P1L0        0x00002000
#define MB_TYPE_P0L1        0x00004000
#define MB_TYPE_P1L1        0x00008000
#define MB_TYPE_L0        (MB_TYPE_P0L0 | MB_TYPE_P1L0)
#define MB_TYPE_L1        (MB_TYPE_P0L1 | MB_TYPE_P1L1)

#define SUB_MB_TYPE_8x8     0x00000001
#define SUB_MB_TYPE_8x4     0x00000002
#define SUB_MB_TYPE_4x8     0x00000004
#define SUB_MB_TYPE_4x4     0x00000008

#define MB_TYPE_INTRA     (MB_TYPE_INTRA4x4 | MB_TYPE_INTRA16x16 | MB_TYPE_INTRA8x8 | MB_TYPE_INTRA_PCM)
#define MB_TYPE_INTER     (MB_TYPE_16x16 | MB_TYPE_16x8 | MB_TYPE_8x16 | MB_TYPE_8x8 | MB_TYPE_8x8_REF0 | MB_TYPE_SKIP | MB_TYPE_DIRECT)
#define IS_INTRA4x4(type) ( MB_TYPE_INTRA4x4 == (type) )
#define IS_INTRA8x8(type) ( MB_TYPE_INTRA8x8 == (type) )
#define IS_INTRANxN(type) ( MB_TYPE_INTRA4x4 == (type) || MB_TYPE_INTRA8x8 == (type) )
#define IS_INTRA16x16(type) ( MB_TYPE_INTRA16x16 == (type) )
#define IS_INTRA(type) ( (type)&MB_TYPE_INTRA )
#define IS_INTER(type) ( (type)&MB_TYPE_INTER )
#define IS_INTER_16x16(type) ( (type)&MB_TYPE_16x16 )
#define IS_INTER_16x8(type) ( (type)&MB_TYPE_16x8 )
#define IS_INTER_8x16(type) ( (type)&MB_TYPE_8x16 )
#define IS_TYPE_L0(type) ( (type)&MB_TYPE_L0 )
#define IS_TYPE_L1(type) ( (type)&MB_TYPE_L1 )
#define IS_DIR(a, part, list) ((a) & (MB_TYPE_P0L0<<((part)+2*(list))))


#define IS_SKIP(type) ( ((type)&MB_TYPE_SKIP) != 0 )
#define IS_DIRECT(type) ( ((type)&MB_TYPE_DIRECT) != 0 )
#define IS_SVC_INTER(type) IS_INTER(type)
#define IS_I_BL(type) ( (type) == MB_TYPE_INTRA_BL )
#define IS_SVC_INTRA(type) ( IS_I_BL(type) || IS_INTRA(type) )
#define IS_Inter_8x8(type) ( ((type)&MB_TYPE_8x8) != 0)
#define IS_SUB_8x8(sub_type) (((sub_type)&SUB_MB_TYPE_8x8) != 0)
#define IS_SUB_8x4(sub_type) (((sub_type)&SUB_MB_TYPE_8x4) != 0)
#define IS_SUB_4x8(sub_type) (((sub_type)&SUB_MB_TYPE_4x8) != 0)
#define IS_SUB_4x4(sub_type) (((sub_type)&SUB_MB_TYPE_4x4) != 0)

#define REF_NOT_AVAIL   -2
#define REF_NOT_IN_LIST -1  //intra

/////////intra16x16  Luma
#define I16_PRED_INVALID   -1
#define I16_PRED_V       0
#define I16_PRED_H       1
#define I16_PRED_DC      2
#define I16_PRED_P       3

#define I16_PRED_DC_L    4
#define I16_PRED_DC_T    5
#define I16_PRED_DC_128  6
#define I16_PRED_DC_A  7
//////////intra4x4   Luma
// Here, I8x8 also use these definitions
#define I4_PRED_INVALID    0
#define I4_PRED_V        0
#define I4_PRED_H        1
#define I4_PRED_DC       2
#define I4_PRED_DDL      3 //diagonal_down_left
#define I4_PRED_DDR      4 //diagonal_down_right
#define I4_PRED_VR       5 //vertical_right
#define I4_PRED_HD       6 //horizon_down
#define I4_PRED_VL       7 //vertical_left
#define I4_PRED_HU       8 //horizon_up

#define I4_PRED_DC_L     9
#define I4_PRED_DC_T     10
#define I4_PRED_DC_128   11

#define I4_PRED_DDL_TOP  12 //right-top replacing by padding rightmost pixel of top
#define I4_PRED_VL_TOP   13 //right-top replacing by padding rightmost pixel of top
#define I4_PRED_A   14

//////////intra Chroma
#define C_PRED_INVALID   -1
#define C_PRED_DC        0
#define C_PRED_H         1
#define C_PRED_V         2
#define C_PRED_P         3

#define C_PRED_DC_L      4
#define C_PRED_DC_T      5
#define C_PRED_DC_128    6
#define C_PRED_A    7
}
#endif//WELS_COMMON_DEFS_H__
