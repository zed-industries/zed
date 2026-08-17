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

#ifndef WELS_PARAMETER_SETS_H__
#define WELS_PARAMETER_SETS_H__

#include "typedefs.h"
#include "wels_const.h"
#include "wels_common_basis.h"

namespace WelsDec {
/* VUI syntax in Sequence Parameter Set, refer to E.1 in Rec */
typedef struct TagVui {
  bool bAspectRatioInfoPresentFlag;
  uint32_t uiAspectRatioIdc;
  uint32_t uiSarWidth;
  uint32_t uiSarHeight;
  bool bOverscanInfoPresentFlag;
  bool bOverscanAppropriateFlag;
  bool bVideoSignalTypePresentFlag;
  uint8_t uiVideoFormat;
  bool bVideoFullRangeFlag;
  bool bColourDescripPresentFlag;
  uint8_t uiColourPrimaries;
  uint8_t uiTransferCharacteristics;
  uint8_t uiMatrixCoeffs;
  bool bChromaLocInfoPresentFlag;
  uint32_t uiChromaSampleLocTypeTopField;
  uint32_t uiChromaSampleLocTypeBottomField;
  bool bTimingInfoPresentFlag;
  uint32_t uiNumUnitsInTick;
  uint32_t uiTimeScale;
  bool bFixedFrameRateFlag;
  bool bNalHrdParamPresentFlag;
  bool bVclHrdParamPresentFlag;
  bool bPicStructPresentFlag;
  bool bBitstreamRestrictionFlag;
  bool bMotionVectorsOverPicBoundariesFlag;
  uint32_t uiMaxBytesPerPicDenom;
  uint32_t uiMaxBitsPerMbDenom;
  uint32_t uiLog2MaxMvLengthHorizontal;
  uint32_t uiLog2MaxMvLengthVertical;
  uint32_t uiMaxNumReorderFrames;
  uint32_t uiMaxDecFrameBuffering;
} SVui, *PVui;

/* Sequence Parameter Set, refer to Page 57 in JVT X201wcm */
typedef struct TagSps {
  int32_t       iSpsId;
  uint32_t      iMbWidth;
  uint32_t      iMbHeight;
  uint32_t      uiTotalMbCount; //used in decode_slice_data()

  uint32_t      uiLog2MaxFrameNum;
  uint32_t      uiPocType;
  /* POC type 0 */
  int32_t       iLog2MaxPocLsb;
  /* POC type 1 */
  int32_t       iOffsetForNonRefPic;

  int32_t       iOffsetForTopToBottomField;
  int32_t       iNumRefFramesInPocCycle;
  int8_t        iOffsetForRefFrame[256];
  int32_t       iNumRefFrames;

  SPosOffset    sFrameCrop;

  ProfileIdc    uiProfileIdc;
  uint8_t       uiLevelIdc;
  uint8_t       uiChromaFormatIdc;
  uint8_t       uiChromaArrayType;

  uint8_t       uiBitDepthLuma;
  uint8_t       uiBitDepthChroma;
  /* TO BE CONTINUE: POC type 1 */
  bool          bDeltaPicOrderAlwaysZeroFlag;
  bool          bGapsInFrameNumValueAllowedFlag;

  bool          bFrameMbsOnlyFlag;
  bool          bMbaffFlag;     // MB Adapative Frame Field
  bool          bDirect8x8InferenceFlag;
  bool          bFrameCroppingFlag;

  bool          bVuiParamPresentFlag;
//  bool          bTimingInfoPresentFlag;
//  bool          bFixedFrameRateFlag;
  bool          bConstraintSet0Flag;
  bool          bConstraintSet1Flag;
  bool          bConstraintSet2Flag;
  bool          bConstraintSet3Flag;
  bool          bSeparateColorPlaneFlag;
  bool          bQpPrimeYZeroTransfBypassFlag;
  bool          bSeqScalingMatrixPresentFlag;
  bool          bSeqScalingListPresentFlag[12];
  //Add scaling list supporting
  uint8_t  iScalingList4x4[6][16];
  uint8_t  iScalingList8x8[6][64];
  SVui sVui;
  const SLevelLimits* pSLevelLimits;
} SSps, *PSps;


/* Sequence Parameter Set extension syntax, refer to Page 58 in JVT X201wcm */
//typedef struct TagSpsExt{
//  uint32_t      iSpsId;
//  uint32_t      uiAuxFormatIdc;
//  int32_t       iAlphaOpaqueValue;
//  int32_t       iAlphaTransparentValue;

//  uint8_t       uiBitDepthAux;
//  bool          bAlphaIncrFlag;
//  bool          bAdditionalExtFlag;
//}SSpsExt, *PSpsExt;

/* Sequence Parameter Set extension syntax, refer to Page 391 in JVT X201wcm */
typedef struct TagSpsSvcExt {
  SPosOffset    sSeqScaledRefLayer;

  uint8_t       uiExtendedSpatialScalability;   // ESS
  uint8_t       uiChromaPhaseXPlus1Flag;
  uint8_t       uiChromaPhaseYPlus1;
  uint8_t       uiSeqRefLayerChromaPhaseXPlus1Flag;
  uint8_t       uiSeqRefLayerChromaPhaseYPlus1;
  bool          bInterLayerDeblockingFilterCtrlPresentFlag;
  bool          bSeqTCoeffLevelPredFlag;
  bool          bAdaptiveTCoeffLevelPredFlag;
  bool          bSliceHeaderRestrictionFlag;
} SSpsSvcExt, *PSpsSvcExt;

/* Subset sequence parameter set syntax, refer to Page 391 in JVT X201wcm */
typedef struct TagSubsetSps {
  SSps          sSps;
  SSpsSvcExt    sSpsSvcExt;
  bool          bSvcVuiParamPresentFlag;
  bool          bAdditionalExtension2Flag;
  bool          bAdditionalExtension2DataFlag;
} SSubsetSps, *PSubsetSps;

/* Picture parameter set syntax, refer to Page 59 in JVT X201wcm */
typedef struct TagPps {
  int32_t       iSpsId;
  int32_t       iPpsId;

  uint32_t      uiNumSliceGroups;
  uint32_t      uiSliceGroupMapType;
  /* slice_group_map_type = 0 */
  uint32_t      uiRunLength[MAX_SLICEGROUP_IDS];
  /* slice_group_map_type = 2 */
  uint32_t      uiTopLeft[MAX_SLICEGROUP_IDS];
  uint32_t      uiBottomRight[MAX_SLICEGROUP_IDS];
  /* slice_group_map_type = 3, 4 or 5 */
  uint32_t      uiSliceGroupChangeRate;
  /* slice_group_map_type = 6 */
  uint32_t      uiPicSizeInMapUnits;
  uint32_t      uiSliceGroupId[MAX_SLICEGROUP_IDS];

  uint32_t      uiNumRefIdxL0Active;
  uint32_t      uiNumRefIdxL1Active;

  int32_t       iPicInitQp;
  int32_t       iPicInitQs;
  int32_t       iChromaQpIndexOffset[2];//cb,cr

  bool          bEntropyCodingModeFlag;
  bool          bPicOrderPresentFlag;
  /* slice_group_map_type = 3, 4 or 5 */
  bool          bSliceGroupChangeDirectionFlag;
  bool          bDeblockingFilterControlPresentFlag;

  bool          bConstainedIntraPredFlag;
  bool          bRedundantPicCntPresentFlag;
  bool          bWeightedPredFlag;
  uint8_t       uiWeightedBipredIdc;

  bool    bTransform8x8ModeFlag;
  //Add for scalinglist support
  bool    bPicScalingMatrixPresentFlag;
  bool    bPicScalingListPresentFlag[12];
  uint8_t  iScalingList4x4[6][16];
  uint8_t  iScalingList8x8[6][64];

  int32_t iSecondChromaQPIndexOffset; //second_chroma_qp_index_offset

} SPps, *PPps;

} // namespace WelsDec

#endif //WELS_PARAMETER_SETS_H__
