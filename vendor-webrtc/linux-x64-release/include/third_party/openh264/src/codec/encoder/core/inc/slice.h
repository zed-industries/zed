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

//wels_slice.h
#ifndef WELS_SLICE_H__
#define WELS_SLICE_H__

#include "typedefs.h"
#include "wels_const.h"
#include "wels_common_basis.h"
#include "mb_cache.h"
#include "picture.h"
#include "parameter_sets.h"
#include "svc_enc_slice_segment.h"
#include "set_mb_syn_cabac.h"
#include "nal_encap.h"

namespace WelsEnc {

/*******************************sub struct of slice header****************************/


/*
 *  Reference picture list reordering syntax, refer to page 64 in JVT X201wcm
 */
typedef struct TagRefPicListReorderSyntax {
struct {
  uint32_t      uiAbsDiffPicNumMinus1; //uiAbsDiffPicNumMinus1 SHOULD be in the range of [4, (1<<pSps->uiLog2MaxFrameNum)-1], {p104, JVT-X201wcm1}
  //but int8_t can't cover the range, SHOULD modify it.
  uint16_t      iLongTermPicNum;
  uint16_t      uiReorderingOfPicNumsIdc; //in order to pack 2-uint16_t into 1-(u)int32_t, so modify the type into uint16_t.
} SReorderingSyntax[MAX_REFERENCE_REORDER_COUNT_NUM];   // MAX_REF_PIC_COUNT
} SRefPicListReorderSyntax;


/* Decoded reference picture marking syntax, refer to Page 66 in JVT X201wcm */
typedef struct TagRefPicMarking {
struct {
  int32_t       iMmcoType;
  int32_t       iShortFrameNum;
  int32_t       iDiffOfPicNum;
  int32_t       iLongTermPicNum;
  int32_t       iLongTermFrameIdx;
  int32_t       iMaxLongTermFrameIdx;
} SMmcoRef[MAX_REFERENCE_MMCO_COUNT_NUM];       // MAX_MMCO_COUNT

// int32_t         mmco_index;
uint8_t         uiMmcoCount;
bool            bNoOutputOfPriorPicsFlag;
bool            bLongTermRefFlag;
bool            bAdaptiveRefPicMarkingModeFlag;
} SRefPicMarking;

// slice level rc statistic info
typedef struct TagRCSlicing {
  int32_t   iComplexityIndexSlice;
  int32_t   iCalculatedQpSlice;
  int32_t   iStartMbSlice;
  int32_t   iEndMbSlice;
  int32_t   iTotalQpSlice;
  int32_t   iTotalMbSlice;
  int32_t   iTargetBitsSlice;
  int32_t   iBsPosSlice;
  int32_t   iFrameBitsSlice;
  int32_t   iGomBitsSlice;
  int32_t   iGomTargetBits;
  //int32_t   gom_coded_mb;
} SRCSlicing;

/* Header of slice syntax elements, refer to Page 63 in JVT X201wcm */
typedef struct TagSliceHeader {
/*****************************slice header syntax and generated****************************/
int32_t         iFirstMbInSlice;
// uint32_t        pic_parameter_set_id;
int32_t         iFrameNum;
int32_t         iPicOrderCntLsb;

// int32_t         delta_pic_order_cnt_bottom;
// int32_t         delta_pic_order_cnt[2];
// int32_t         redundant_pic_cnt;

EWelsSliceType  eSliceType;
uint8_t         uiNumRefIdxL0Active;                    //
//int32_t         num_ref_idx_l1_active_minus1    //B frame is not supported
uint8_t         uiRefCount;
//Ref_Pic         *ref_pic;
uint8_t         uiRefIndex;     // exact reference picture index for slice

int8_t          iSliceQpDelta;
// int32_t         slice_qp;
// int32_t         slice_qs_delta;         // For SP/SI slices
uint8_t         uiDisableDeblockingFilterIdc;
int8_t          iSliceAlphaC0Offset;
int8_t          iSliceBetaOffset;
#if !defined(DISABLE_FMO_FEATURE)
int32_t         iSliceGroupChangeCycle;
#endif//!DISABLE_FMO_FEATURE

SWelsSPS*       pSps;
SWelsPPS*       pPps;
int32_t         iSpsId;
int32_t         iPpsId;

uint16_t        uiIdrPicId;
// uint8_t         color_plane_id;//from?

bool            bNumRefIdxActiveOverrideFlag;
// bool            field_pic_flag;         //not supported in base profile
// bool            bottom_field_flag;              //not supported in base profile
uint8_t         uiPadding1Bytes;

SRefPicMarking  sRefMarking;    // Decoded reference picture marking syntaxs

SRefPicListReorderSyntax        sRefReordering; // Reference picture list reordering syntaxs
} SSliceHeader, *PSliceHeader;


/* SSlice header in scalable extension syntax, refer to Page 394 in JVT X201wcm */
typedef struct TagSliceHeaderExt {
SSliceHeader    sSliceHeader;

SSubsetSps*     pSubsetSps;

uint32_t        uiNumMbsInSlice;

bool            bStoreRefBasePicFlag;
bool            bConstrainedIntraResamplingFlag;
bool            bSliceSkipFlag;

bool            bAdaptiveBaseModeFlag;
bool            bDefaultBaseModeFlag;
bool            bAdaptiveMotionPredFlag;
bool            bDefaultMotionPredFlag;

bool            bAdaptiveResidualPredFlag;
bool            bDefaultResidualPredFlag;
bool            bTcoeffLevelPredFlag;
uint8_t         uiDisableInterLayerDeblockingFilterIdc;

} SSliceHeaderExt, *PSliceHeaderExt;


typedef struct TagSlice {
// mainly for multiple threads imp.
SMbCache        sMbCacheInfo;   // MBCache is introduced within slice dependency
SBitStringAux*  pSliceBsa;
SWelsSliceBs    sSliceBs;

/*******************************sSliceHeader****************************/
SSliceHeaderExt sSliceHeaderExt;

SMVUnitXY       sMvStartMin;
SMVUnitXY       sMvStartMax;
SMVUnitXY       sMvc[5];
uint8_t         uiMvcNum;
uint8_t         sScaleShift;

int32_t         iSliceIdx;
uint32_t        uiBufferIdx;
bool            bSliceHeaderExtFlag; // Indicate which slice header is used, avc or ext?
uint8_t         uiLastMbQp;             // stored qp for last mb coded, maybe more efficient for mb skip detection etc.

bool            bDynamicSlicingSliceSizeCtrlFlag;
uint8_t         uiAssumeLog2BytePerMb;

uint32_t        uiSliceFMECostDown;//TODO: for FME switch under MT, to opt after ME final?

uint8_t         uiReservedFillByte;     // reserved to meet 4 bytes alignment

SCabacCtx       sCabacCtx;
int32_t         iCabacInitIdc;
int32_t         iMbSkipRun;

int32_t         iCountMbNumInSlice;
uint32_t        uiSliceConsumeTime;
int32_t         iSliceComplexRatio;

SRCSlicing      sSlicingOverRc;   //slice level rc statistic info
} SSlice, *PSlice;

}
#endif//WELS_SLICE_H__

