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

namespace WelsEnc {

/* Sequence Parameter Set, refer to Page 57 in JVT X201wcm */
typedef struct TagWelsSPS {
uint32_t        uiSpsId;
int16_t         iMbWidth;
int16_t         iMbHeight;
uint32_t        uiLog2MaxFrameNum;
uint32_t        uiPocType;
/* POC type 0 */
int32_t         iLog2MaxPocLsb;
/* POC type 1 */
// int32_t         iOffsetForNonRefPic;

// int32_t         iOffsetForTopToBottomField;
// int32_t         iNumRefFramesInPocCycle;
// int8_t          iOffsetForRefFrame[256];
SCropOffset     sFrameCrop;
int16_t         iNumRefFrames;
// uint32_t        uiNumUnitsInTick;
// uint32_t        uiTimeScale;

uint8_t         uiProfileIdc;
uint8_t         iLevelIdc;
// uint8_t         uiChromaFormatIdc;
// uint8_t         uiChromaArrayType;              //support =1

// uint8_t         uiBitDepthLuma;         //=8, only used in decoder, encoder in general_***; it can be removed when removed general up_sample
// uint8_t         uiBitDepthChroma;               //=8
/* TO BE CONTINUE: POC type 1 */
// bool            bDeltaPicOrderAlwaysZeroFlag;
bool            bGapsInFrameNumValueAllowedFlag;

// bool            bFrameMbsOnlyFlag;
// bool            bMbaffFlag;     // MB Adapative Frame Field
// bool            bDirect8x8InferenceFlag;
bool            bFrameCroppingFlag;

bool            bVuiParamPresentFlag;
// bool            bTimingInfoPresentFlag;
// bool            bFixedFrameRateFlag;

// Note: members bVideoSignalTypePresent through uiColorMatrix below are also defined in SSpatialLayerConfig in codec_app_def.h,
// along with definitions for enumerators EVideoFormatSPS, EColorPrimaries, ETransferCharacteristics, and EColorMatrix.
bool	bVideoSignalTypePresent;	// false => do not write any of the following information to the header
uint8_t	uiVideoFormat;				// EVideoFormatSPS; 3 bits in header; 0-5 => component, kpal, ntsc, secam, mac, undef
bool	bFullRange;					// false => analog video data range [16, 235]; true => full data range [0,255]
bool	bColorDescriptionPresent;	// false => do not write any of the following three items to the header
uint8_t	uiColorPrimaries;			// EColorPrimaries; 8 bits in header; 0 - 9 => ???, bt709, undef, ???, bt470m, bt470bg,
                                    //    smpte170m, smpte240m, film, bt2020
uint8_t	uiTransferCharacteristics;	// ETransferCharacteristics; 8 bits in header; 0 - 15 => ???, bt709, undef, ???, bt470m, bt470bg, smpte170m,
                                    //   smpte240m, linear, log100, log316, iec61966-2-4, bt1361e, iec61966-2-1, bt2020-10, bt2020-12
uint8_t	uiColorMatrix;				// EColorMatrix; 8 bits in header (corresponds to FFmpeg "colorspace"); 0 - 10 => GBR, bt709,
                                    //   undef, ???, fcc, bt470bg, smpte170m, smpte240m, YCgCo, bt2020nc, bt2020c

bool            bConstraintSet0Flag;
bool            bConstraintSet1Flag;
bool            bConstraintSet2Flag;
bool            bConstraintSet3Flag;
// bool            bSeparateColorPlaneFlag;  // =false,: only used in decoder, encoder in general_***; it can be removed when removed general up_sample

// aspect ratio in VUI
bool            bAspectRatioPresent;
ESampleAspectRatio  eAspectRatio;
uint16_t            sAspectRatioExtWidth;
uint16_t            sAspectRatioExtHeight;

} SWelsSPS, *PWelsSPS;


/* Sequence Parameter Set SVC extension syntax, refer to Page 391 in JVT X201wcm */
typedef struct TagSpsSvcExt {
// SCropOffset     sSeqScaledRefLayer;

uint8_t         iExtendedSpatialScalability;    // ESS
// uint8_t         uiChromaPhaseXPlus1Flag;
// uint8_t         uiChromaPhaseYPlus1;
// uint8_t         uiSeqRefLayerChromaPhaseXPlus1Flag;
// uint8_t         uiSeqRefLayerChromaPhaseYPlus1;
// bool            bInterLayerDeblockingFilterCtrlPresentFlag;
bool            bSeqTcoeffLevelPredFlag;
bool            bAdaptiveTcoeffLevelPredFlag;
bool            bSliceHeaderRestrictionFlag;
} SSpsSvcExt, *PSpsSvcExt;

/* Subset sequence parameter set syntax, refer to Page 391 in JVT X201wcm */
typedef struct TagSubsetSps {
SWelsSPS                pSps;
SSpsSvcExt      sSpsSvcExt;

// bool            bSvcVuiParamPresentFlag;
// bool            bAdditionalExtension2Flag;
// bool            bAdditionalExtension2DataFlag;
} SSubsetSps, *PSubsetSps;

/* Picture parameter set syntax, refer to Page 59 in JVT X201wcm */
typedef struct TagWelsPPS {
uint32_t        iSpsId;
uint32_t        iPpsId;

#if !defined(DISABLE_FMO_FEATURE)
uint32_t        uiNumSliceGroups;
uint32_t        uiSliceGroupMapType;
/* uiSliceGroupMapType = 0 */
uint32_t        uiRunLength[MAX_SLICEGROUP_IDS];
/* uiSliceGroupMapType = 2 */
uint32_t        uiTopLeft[MAX_SLICEGROUP_IDS];
uint32_t        uiBottomRight[MAX_SLICEGROUP_IDS];
/* uiSliceGroupMapType = 3, 4 or 5 */
/* uiSliceGroupMapType = 3, 4 or 5 */
bool            bSliceGroupChangeDirectionFlag;
uint32_t        uiSliceGroupChangeRate;
/* uiSliceGroupMapType = 6 */
uint32_t        uiPicSizeInMapUnits;
uint32_t        uiSliceGroupId[MAX_SLICEGROUP_IDS];
#endif//!DISABLE_FMO_FEATURE

// uint32_t        uiNumRefIdxL0Active;
// uint32_t        uiNumRefIdxL1Active;

int8_t          iPicInitQp;
int8_t          iPicInitQs;
uint8_t         uiChromaQpIndexOffset;

/* potential application for High profile */
// int32_t         iSecondChromaQpIndexOffset;
// /* potential application for High profile */

// bool            bPicOrderPresentFlag;
bool    bEntropyCodingModeFlag;
bool            bDeblockingFilterControlPresentFlag;

// bool            bConstainedIntraPredFlag;
// bool            bRedundantPicCntPresentFlag;
// bool            bWeightedPredFlag;
// uint8_t         uiWeightedBiPredIdc;

} SWelsPPS, *PWelsPPPS;

}

#endif //WELS_PARAMETER_SETS_H__
