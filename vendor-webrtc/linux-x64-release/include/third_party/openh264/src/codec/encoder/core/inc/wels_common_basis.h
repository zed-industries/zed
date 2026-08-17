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

#include "wels_const.h"

#include "wels_common_defs.h"

using namespace WelsCommon;

namespace WelsEnc {

typedef int32_t WelsErrorType;

struct SMVUnitXY { // each 4 Bytes
  int16_t iMvX;
  int16_t iMvY;
 public:
  SMVUnitXY& sDeltaMv (const SMVUnitXY& _v0, const SMVUnitXY& _v1) {
    iMvX = _v0.iMvX - _v1.iMvX;
    iMvY = _v0.iMvY - _v1.iMvY;
    return (*this);
  };
  SMVUnitXY& sAssignMv (const SMVUnitXY& _v0) {
    iMvX = _v0.iMvX;
    iMvY = _v0.iMvY;
    return (*this);
  };
};

typedef struct TagMVComponentUnit {             // each LIST_0/LIST_1
  SMVUnitXY     sMotionVectorCache[5 * 6 - 1];  // Luma only: 5 x 6 - 1 = 29 D-Words
  int8_t        iRefIndexCache[5 * 6];          // Luma only: 5 x 6 = 30 bytes
} SMVComponentUnit, *PMVComponentUnit;


typedef struct TagParaSetOffsetVariable {
  int32_t       iParaSetIdDelta[MAX_DQ_LAYER_NUM/*+1*/];//mark delta between SPS_ID_in_bs and sps_id_in_encoder, can be minus, for each dq-layer
//need not extra +1 due no MGS and FMO case so far
  bool          bUsedParaSetIdInBs[MAX_PPS_COUNT];      //mark the used SPS_ID with 1
  uint32_t      uiNextParaSetIdToUseInBs;               //mark the next SPS_ID_in_bs, for all layers
} SParaSetOffsetVariable;

typedef struct TagParaSetOffset {
//in PS0 design, "sParaSetOffsetVariable" record the previous paras before current IDR, AND NEED to be stacked and recover across IDR
  SParaSetOffsetVariable
  sParaSetOffsetVariable[PARA_SET_TYPE]; //PARA_SET_TYPE=3; paraset_type = 0: AVC_SPS; =1: Subset_SPS; =2: PPS
//in PSO design, "bPpsIdMappingIntoSubsetsps" uses the current para of current IDR period
  bool
  bPpsIdMappingIntoSubsetsps[MAX_DQ_LAYER_NUM/*+1*/];   // need not extra +1 due no MGS and FMO case so far

  int32_t  iPpsIdList[MAX_DQ_LAYER_NUM][MAX_PPS_COUNT]; //index0: max pps types; index1: for differnt IDRs, if only index0=1, index1 can reach MAX_PPS_COUNT

#if _DEBUG
  int32_t  eSpsPpsIdStrategy;
#endif

  uint32_t uiNeededSpsNum;
  uint32_t uiNeededSubsetSpsNum;
  uint32_t uiNeededPpsNum;

  uint32_t uiInUseSpsNum;
  uint32_t uiInUseSubsetSpsNum;
  uint32_t uiInUsePpsNum;
} SParaSetOffset;



/* Position Offset structure */
typedef struct TagCropOffset {
  int16_t       iCropLeft;
  int16_t       iCropRight;
  int16_t       iCropTop;
  int16_t       iCropBottom;
} SCropOffset;


/* Transform Type */

enum ETransType {
  T_4x4     = 0,
  T_8x8     = 1,
  T_16x16   = 2,
  T_PCM     = 3
};

enum EMbPosition {
  LEFT_MB_POS           = 0x01, // A
  TOP_MB_POS            = 0x02, // B
  TOPRIGHT_MB_POS       = 0x04, // C
  TOPLEFT_MB_POS        = 0x08, // D,
  RIGHT_MB_POS          = 0x10, //  add followed four case to reuse when intra up-sample
  BOTTOM_MB_POS         = 0x20, //
  BOTTOMRIGHT_MB_POS    = 0x40, //
  BOTTOMLEFT_MB_POS     = 0x80, //
  MB_POS_A              = 0x100
};

/* MB Type & Sub-MB Type */
typedef uint32_t Mb_Type;

#define MB_LEFT_BIT             0// add to use in intra up-sample
#define MB_TOP_BIT              1
#define MB_TOPRIGHT_BIT         2
#define MB_TOPLEFT_BIT          3
#define MB_RIGHT_BIT            4
#define MB_BOTTOM_BIT           5
#define MB_BTMRIGHT_BIT         6
#define MB_BTMLEFT_BIT          7

#define MB_TYPE_BACKGROUND      0x00010000  // conditional BG skip_mb

enum {
  Intra4x4      = 0,
  Intra16x16    = 1,
  Inter16x16    = 2,
  Inter16x8     = 3,
  Inter8x16     = 4,
  Inter8x8      = 5,
  PSkip         = 6
};


}
#endif//WELS_COMMON_BASIS_H__
