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
#include "picture.h"
#include "parameter_sets.h"

namespace WelsDec {

/*
 *  Reference picture list reordering syntax, refer to page 64 in JVT X201wcm
 */
typedef struct TagRefPicListReorderSyntax {
  struct {
    uint32_t    uiAbsDiffPicNumMinus1;
    uint16_t    uiLongTermPicNum;
    uint16_t    uiReorderingOfPicNumsIdc;
  } sReorderingSyn[LIST_A][MAX_REF_PIC_COUNT + 1];
  bool          bRefPicListReorderingFlag[LIST_A];
} SRefPicListReorderSyn, *PRefPicListReorderSyn;

/*
 *  Prediction weight table syntax, refer to page 65 in JVT X201wcm
 */
typedef struct TagPredWeightTabSyntax {
  uint32_t  uiLumaLog2WeightDenom;
  uint32_t  uiChromaLog2WeightDenom;
  struct {
    int32_t iLumaWeight[MAX_REF_PIC_COUNT];
    int32_t iLumaOffset[MAX_REF_PIC_COUNT];
    int32_t iChromaWeight[MAX_REF_PIC_COUNT][2];
    int32_t iChromaOffset[MAX_REF_PIC_COUNT][2];
    bool    bLumaWeightFlag;
    bool    bChromaWeightFlag;
  } sPredList[LIST_A];
  int32_t   iImplicitWeight[MAX_REF_PIC_COUNT][MAX_REF_PIC_COUNT];
} SPredWeightTabSyn, *PPredWeightTabSyn;

/* Decoded reference picture marking syntax, refer to Page 66 in JVT X201wcm */
typedef struct TagRefPicMarking {
  struct {
    uint32_t    uiMmcoType;
    int32_t     iShortFrameNum;
    int32_t     iDiffOfPicNum;
    uint32_t    uiLongTermPicNum;
    int32_t     iLongTermFrameIdx;
    int32_t     iMaxLongTermFrameIdx;
  } sMmcoRef[MAX_MMCO_COUNT];

  bool          bNoOutputOfPriorPicsFlag;
  bool          bLongTermRefFlag;
  bool          bAdaptiveRefPicMarkingModeFlag;
} SRefPicMarking, *PRefPicMarking;

/* Decode reference base picture marking syntax in Page 396 of JVT X201wcm */
typedef struct TagRefBasePicMarkingSyn {
  struct {
    uint32_t      uiMmcoType;
    int32_t       iShortFrameNum;
    uint32_t      uiDiffOfPicNums;
    uint32_t      uiLongTermPicNum; //should uint32_t, cover larger range of iFrameNum.
  } mmco_base[MAX_MMCO_COUNT];    // MAX_REF_PIC for reference picture based on frame

  bool            bAdaptiveRefBasePicMarkingModeFlag;
} SRefBasePicMarking, *PRefBasePicMarking;

/* Header of slice syntax elements, refer to Page 63 in JVT X201wcm */
typedef struct TagSliceHeaders {
  /*****************************slice header syntax and generated****************************/
  int32_t         iFirstMbInSlice;
  int32_t         iFrameNum;
  int32_t         iPicOrderCntLsb;
  int32_t         iDeltaPicOrderCntBottom;
  int32_t         iDeltaPicOrderCnt[2];
  int32_t         iRedundantPicCnt;
  int32_t         iDirectSpatialMvPredFlag; //!< Direct Mode type to be used (0: Temporal, 1: Spatial)
  int32_t         uiRefCount[LIST_A];
  int32_t         iSliceQpDelta;  //no use for iSliceQp is used directly
  int32_t         iSliceQp;
  int32_t         iSliceQsDelta;  // For SP/SI slices
  uint32_t        uiDisableDeblockingFilterIdc;
  int32_t         iSliceAlphaC0Offset;
  int32_t         iSliceBetaOffset;
  int32_t         iSliceGroupChangeCycle;

  PSps            pSps;
  PPps            pPps;
  int32_t         iSpsId;
  int32_t         iPpsId;
  bool            bIdrFlag;

  /*********************got from other layer for efficency if possible*********************/
  SRefPicListReorderSyn   pRefPicListReordering;  // Reference picture list reordering syntaxs
  SPredWeightTabSyn       sPredWeightTable;
  int32_t                 iCabacInitIdc;
  int32_t                 iMbWidth;       //from?
  int32_t                 iMbHeight; //from?
  SRefPicMarking          sRefMarking;    // Decoded reference picture marking syntaxs

  uint16_t    uiIdrPicId;
  EWelsSliceType  eSliceType;
  bool            bNumRefIdxActiveOverrideFlag;
  bool            bFieldPicFlag;          //not supported in base profile
  bool            bBottomFiledFlag;               //not supported in base profile
  uint8_t         uiPadding1Byte;
  bool            bSpForSwitchFlag;                       // For SP/SI slices
  int16_t         iPadding2Bytes;
} SSliceHeader, *PSliceHeader;


/* Slice header in scalable extension syntax, refer to Page 394 in JVT X201wcm */
typedef struct TagSliceHeaderExt {
  SSliceHeader    sSliceHeader;
  PSubsetSps      pSubsetSps;

  uint32_t        uiDisableInterLayerDeblockingFilterIdc;
  int32_t         iInterLayerSliceAlphaC0Offset;
  int32_t         iInterLayerSliceBetaOffset;

//SPosOffset sScaledRefLayer;
  int32_t         iScaledRefLayerPicWidthInSampleLuma;
  int32_t         iScaledRefLayerPicHeightInSampleLuma;

  SRefBasePicMarking sRefBasePicMarking;
  bool            bBasePredWeightTableFlag;
  bool            bStoreRefBasePicFlag;
  bool            bConstrainedIntraResamplingFlag;
  bool            bSliceSkipFlag;

  bool            bAdaptiveBaseModeFlag;
  bool            bDefaultBaseModeFlag;
  bool            bAdaptiveMotionPredFlag;
  bool            bDefaultMotionPredFlag;
  bool            bAdaptiveResidualPredFlag;
  bool            bDefaultResidualPredFlag;
  bool            bTCoeffLevelPredFlag;
  uint8_t         uiRefLayerChromaPhaseXPlus1Flag;

  uint8_t         uiRefLayerChromaPhaseYPlus1;
  uint8_t         uiRefLayerDqId;
  uint8_t         uiScanIdxStart;
  uint8_t         uiScanIdxEnd;
} SSliceHeaderExt, *PSliceHeaderExt;


typedef struct TagSlice {
  /*******************************slice_header****************************/
  SSliceHeaderExt sSliceHeaderExt;

  /*******************************use for future****************************/
// for Macroblock coding within slice
  int32_t         iLastMbQp;              // stored qp for last mb coded, maybe more efficient for mb skip detection etc.

  /*******************************slice_data****************************/
  /*slice_data_ext()*/
  int32_t         iMbSkipRun;
  int32_t         iTotalMbInCurSlice; //record the total number of MB in current slice.

  /*slice_data_ext() generate*/

  /*******************************misc use****************************/
  bool            bSliceHeaderExtFlag; // Indicate which slice header is used, avc or ext?
  /*************got from other layer for effiency if possible***************/
  /*from lower layer: slice header*/
  uint8_t         eSliceType;
  uint8_t         uiPadding[2];
  int32_t         iLastDeltaQp;
  int16_t         iMvScale[LIST_A][MAX_DPB_COUNT]; //Moton vector scale For Temporal Direct Mode Type
} SSlice, *PSlice;

} // namespace WelsDec

#endif//WELS_SLICE_H__
