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
 * \file    param_svc.h
 *
 * \brief   Configurable parameters in H.264/SVC Encoder
 *
 * \date    4/20/2009 Created
 *
 *************************************************************************************
 */
#if !defined(WELS_ENCODER_PARAMETER_SVC_H__)
#define WELS_ENCODER_PARAMETER_SVC_H__

#include <string.h>
#include <math.h>
#include "typedefs.h"
#include "codec_def.h"
#include "macros.h"
#include "wels_const.h"
#include "rc.h"
#include "svc_enc_slice_segment.h"
#include "as264_common.h"

namespace WelsEnc {

#define   INVALID_TEMPORAL_ID   ((uint8_t)0xff)

extern const uint8_t   g_kuiTemporalIdListTable[MAX_TEMPORAL_LEVEL][MAX_GOP_SIZE + 1];

/*!
* \brief    get Logarithms base 2 of (upper/base)
* \param    base    based scaler
* \param    upper   input upper value
* \return   2 based scaling factor
*/
static inline uint32_t GetLogFactor (float base, float upper) {
#if defined(_M_X64) && _MSC_VER == 1800
  _set_FMA3_enable(0);
#endif
  const double dLog2factor      = log10 (1.0 * upper / base) / log10 (2.0);
  const double dEpsilon         = 0.0001;
  const double dRound           = floor (dLog2factor + 0.5);

  if (dLog2factor < dRound + dEpsilon && dRound < dLog2factor + dEpsilon) {
    return (uint32_t) (dRound);
  }
  return UINT_MAX;
}

/*
 *  Dependency Layer Parameter
 */
typedef struct TagDLayerParam {
  int32_t       iActualWidth;                   // input source picture actual width
  int32_t       iActualHeight;                  // input source picture actual height
  int32_t       iTemporalResolution;
  int32_t       iDecompositionStages;
  uint8_t       uiCodingIdx2TemporalId[ (1 << MAX_TEMPORAL_LEVEL) + 1];

  int8_t        iHighestTemporalId;
  float         fInputFrameRate;                // input frame rate
  float         fOutputFrameRate;               // output frame rate
  uint16_t          uiIdrPicId;           // IDR picture id: [0, 65535], this one is used for LTR
  int32_t           iCodingIndex;
  int32_t           iFrameIndex;            // count how many frames elapsed during coding context currently
  bool              bEncCurFrmAsIdrFlag;
  int32_t           iFrameNum;              // current frame number coding
  int32_t           iPOC;                   // frame iPOC
#ifdef ENABLE_FRAME_DUMP
  char          sRecFileName[MAX_FNAME_LEN];    // file to be constructed
#endif//ENABLE_FRAME_DUMP
} SSpatialLayerInternal;

/*
 *  Cisco OpenH264 Encoder Parameter Configuration
 */
typedef struct TagWelsSvcCodingParam: SEncParamExt {
  SSpatialLayerInternal sDependencyLayers[MAX_DEPENDENCY_LAYER];

  /* General */
  uint32_t uiGopSize;                      // GOP size (at maximal frame rate: 16)
  struct {
    int32_t iLeft;
    int32_t iTop;
    int32_t iWidth;
    int32_t iHeight;
  } SUsedPicRect;       // the rect in input picture that encoder actually used

  char*       pCurPath; // record current lib path such as:/pData/pData/com.wels.enc/lib/

  bool      bDeblockingParallelFlag;        // deblocking filter parallelization control flag
  int32_t   iBitsVaryPercentage;

  int8_t   iDecompStages;          // GOP size dependency
  int32_t  iMaxNumRefFrame;

 public:
  TagWelsSvcCodingParam() {
    FillDefault();
  }
  ~TagWelsSvcCodingParam() {}

  static void FillDefault (SEncParamExt& param) {
    memset (&param, 0, sizeof (param));
    param.uiIntraPeriod         = 0;                    // intra period (multiple of GOP size as desired)
    param.iNumRefFrame          = AUTO_REF_PIC_COUNT;// number of reference frame used

    param.iPicWidth             = 0;    //   actual input picture width
    param.iPicHeight            = 0;    //   actual input picture height

    param.fMaxFrameRate         = MAX_FRAME_RATE;       // maximal frame rate [Hz / fps]

    param.iComplexityMode       = LOW_COMPLEXITY;
    param.iTargetBitrate        = UNSPECIFIED_BIT_RATE; // overall target bitrate introduced in RC module
    param.iMaxBitrate           = UNSPECIFIED_BIT_RATE;
    param.iMultipleThreadIdc    = 1;
    param.bUseLoadBalancing = true;

    param.iLTRRefNum            = 0;
    param.iLtrMarkPeriod        = 30;   //the min distance of two int32_t references

    param.bEnableSSEI           = false;
    param.bSimulcastAVC         = false;
    param.bEnableFrameCroppingFlag = true; // enable frame cropping flag: true alwayse in application
    // false: Streaming Video Sharing; true: Video Conferencing Meeting;

    /* Deblocking loop filter */
    param.iLoopFilterDisableIdc         = 0;    // 0: on, 1: off, 2: on except for slice boundaries
    param.iLoopFilterAlphaC0Offset      = 0;    // AlphaOffset: valid range [-6, 6], default 0
    param.iLoopFilterBetaOffset         = 0;    // BetaOffset:  valid range [-6, 6], default 0

    /* Rate Control */
    param.iRCMode                       = RC_QUALITY_MODE;
    param.iPaddingFlag                  = 0;
    param.iEntropyCodingModeFlag        = 0;
    param.bEnableDenoise                = false;        // denoise control
    param.bEnableSceneChangeDetect      = true;         // scene change detection control
    param.bEnableBackgroundDetection    = true;         // background detection control
    param.bEnableAdaptiveQuant          = true;         // adaptive quantization control
    param.bEnableFrameSkip              = true;         // frame skipping
    param.bEnableLongTermReference      = false;        // long term reference control
    param.eSpsPpsIdStrategy             = INCREASING_ID;// pSps pPps id addition control
    param.bPrefixNalAddingCtrl          = false;        // prefix NAL adding control
    param.iSpatialLayerNum              = 1;            // number of dependency(Spatial/CGS) layers used to be encoded
    param.iTemporalLayerNum             = 1;            // number of temporal layer specified

    param.iMaxQp = QP_MAX_VALUE;
    param.iMinQp = QP_MIN_VALUE;
    param.iUsageType = CAMERA_VIDEO_REAL_TIME;
    param.uiMaxNalSize = 0;
    param.bIsLosslessLink = false;
    param.bFixRCOverShoot = true;
    param.iIdrBitrateRatio = IDR_BITRATE_RATIO * 100;
    param.bPsnrY = false;
    param.bPsnrU = false;
    param.bPsnrV = false;
    for (int32_t iLayer = 0; iLayer < MAX_SPATIAL_LAYER_NUM; iLayer++) {
      param.sSpatialLayers[iLayer].uiProfileIdc = PRO_UNKNOWN;
      param.sSpatialLayers[iLayer].uiLevelIdc = LEVEL_UNKNOWN;
      param.sSpatialLayers[iLayer].iDLayerQp = SVC_QUALITY_BASE_QP;
      param.sSpatialLayers[iLayer].fFrameRate = param.fMaxFrameRate;

      param.sSpatialLayers[iLayer].iMaxSpatialBitrate = UNSPECIFIED_BIT_RATE;

      param.sSpatialLayers[iLayer].sSliceArgument.uiSliceMode = SM_SINGLE_SLICE;
      param.sSpatialLayers[iLayer].sSliceArgument.uiSliceNum = 0; //AUTO, using number of CPU cores
      param.sSpatialLayers[iLayer].sSliceArgument.uiSliceSizeConstraint = 1500;

      param.sSpatialLayers[iLayer].bAspectRatioPresent = false; // do not write any of the following information to the header
      param.sSpatialLayers[iLayer].eAspectRatio = ASP_UNSPECIFIED;
      param.sSpatialLayers[iLayer].sAspectRatioExtWidth = 0;
      param.sSpatialLayers[iLayer].sAspectRatioExtHeight = 0;

      const int32_t kiLesserSliceNum = ((MAX_SLICES_NUM < MAX_SLICES_NUM_TMP) ? MAX_SLICES_NUM : MAX_SLICES_NUM_TMP);
      for (int32_t idx = 0; idx < kiLesserSliceNum; idx++)
        param.sSpatialLayers[iLayer].sSliceArgument.uiSliceMbNum[idx] = 0; //default, using one row a slice if uiSliceMode is SM_RASTER_MODE

      // See codec_app_def.h for more info about members bVideoSignalTypePresent through uiColorMatrix.  The default values
      // used below preserve the previous behavior; i.e., no additional information will be written to the output file.
      param.sSpatialLayers[iLayer].bVideoSignalTypePresent = false;			// do not write any of the following information to the header
      param.sSpatialLayers[iLayer].uiVideoFormat = VF_UNDEF;				// undefined
      param.sSpatialLayers[iLayer].bFullRange = false;						// analog video data range [16, 235]
      param.sSpatialLayers[iLayer].bColorDescriptionPresent = false;		// do not write any of the following three items to the header
      param.sSpatialLayers[iLayer].uiColorPrimaries = CP_UNDEF;				// undefined
      param.sSpatialLayers[iLayer].uiTransferCharacteristics = TRC_UNDEF;	// undefined
      param.sSpatialLayers[iLayer].uiColorMatrix = CM_UNDEF;				// undefined
    }
  }

  void FillDefault() {
    FillDefault (*this);
    uiGopSize = 1;                      // GOP size (at maximal frame rate: 16)
    iMaxNumRefFrame = AUTO_REF_PIC_COUNT;
    SUsedPicRect.iLeft =
      SUsedPicRect.iTop =
        SUsedPicRect.iWidth =
          SUsedPicRect.iHeight = 0;     // the rect in input picture that encoder actually used

    pCurPath                    = NULL; // record current lib path such as:/pData/pData/com.wels.enc/lib/

    bDeblockingParallelFlag     = false;// deblocking filter parallelization control flag

    iDecompStages               = 0;    // GOP size dependency, unknown here and be revised later
    iBitsVaryPercentage = 10;
  }

  int32_t ParamBaseTranscode (const SEncParamBase& pCodingParam) {

    fMaxFrameRate  = WELS_CLIP3 (pCodingParam.fMaxFrameRate, MIN_FRAME_RATE, MAX_FRAME_RATE);
    iTargetBitrate = pCodingParam.iTargetBitrate;
    iUsageType  = pCodingParam.iUsageType;
    iPicWidth   = pCodingParam.iPicWidth;
    iPicHeight  = pCodingParam.iPicHeight;

    SUsedPicRect.iLeft = 0;
    SUsedPicRect.iTop  = 0;
    SUsedPicRect.iWidth = ((iPicWidth >> 1) * (1 << 1));
    SUsedPicRect.iHeight = ((iPicHeight >> 1) * (1 << 1));

    iRCMode = pCodingParam.iRCMode;    // rc mode

    int8_t iIdxSpatial = 0;
    EProfileIdc uiProfileIdc = PRO_UNKNOWN;
    if (iEntropyCodingModeFlag)
      uiProfileIdc = PRO_MAIN;
    SSpatialLayerInternal* pDlp = &sDependencyLayers[0];

    while (iIdxSpatial < iSpatialLayerNum) {

      sSpatialLayers->uiProfileIdc              = uiProfileIdc;
      sSpatialLayers->uiLevelIdc                = LEVEL_UNKNOWN;
      sSpatialLayers[iIdxSpatial].fFrameRate    = WELS_CLIP3 (pCodingParam.fMaxFrameRate,
          MIN_FRAME_RATE, MAX_FRAME_RATE);
      pDlp->fInputFrameRate =
        pDlp->fOutputFrameRate = WELS_CLIP3 (sSpatialLayers[iIdxSpatial].fFrameRate, MIN_FRAME_RATE,
                                             MAX_FRAME_RATE);
#ifdef ENABLE_FRAME_DUMP
      pDlp->sRecFileName[0] = '\0'; // file to be constructed
#endif//ENABLE_FRAME_DUMP
      pDlp->iActualWidth = sSpatialLayers[iIdxSpatial].iVideoWidth = iPicWidth;
      pDlp->iActualHeight = sSpatialLayers[iIdxSpatial].iVideoHeight = iPicHeight;

      sSpatialLayers->iSpatialBitrate =
        sSpatialLayers[iIdxSpatial].iSpatialBitrate = pCodingParam.iTargetBitrate; // target bitrate for current spatial layer

      sSpatialLayers->iMaxSpatialBitrate = UNSPECIFIED_BIT_RATE;
      sSpatialLayers->iDLayerQp = SVC_QUALITY_BASE_QP;

      uiProfileIdc = (!bSimulcastAVC) ? PRO_SCALABLE_BASELINE : uiProfileIdc;
      ++ pDlp;
      ++ iIdxSpatial;
    }
    SetActualPicResolution();

    return 0;
  }
  void GetBaseParams (SEncParamBase* pCodingParam) {
    pCodingParam->iUsageType     = iUsageType;
    pCodingParam->iPicWidth      = iPicWidth;
    pCodingParam->iPicHeight     = iPicHeight;
    pCodingParam->iTargetBitrate = iTargetBitrate;
    pCodingParam->iRCMode        = iRCMode;
    pCodingParam->fMaxFrameRate  = fMaxFrameRate;
  }
  int32_t ParamTranscode (const SEncParamExt& pCodingParam) {
    float fParamMaxFrameRate = WELS_CLIP3 (pCodingParam.fMaxFrameRate, MIN_FRAME_RATE, MAX_FRAME_RATE);
    iUsageType = pCodingParam.iUsageType;
    iPicWidth   = pCodingParam.iPicWidth;
    iPicHeight  = pCodingParam.iPicHeight;
    fMaxFrameRate = fParamMaxFrameRate;
    iComplexityMode = pCodingParam.iComplexityMode;

    SUsedPicRect.iLeft = 0;
    SUsedPicRect.iTop  = 0;
    SUsedPicRect.iWidth = ((iPicWidth >> 1) << 1);
    SUsedPicRect.iHeight = ((iPicHeight >> 1) << 1);

    iMultipleThreadIdc = pCodingParam.iMultipleThreadIdc;
    bUseLoadBalancing = pCodingParam.bUseLoadBalancing;

    /* Deblocking loop filter */
    iLoopFilterDisableIdc       = pCodingParam.iLoopFilterDisableIdc;      // 0: on, 1: off, 2: on except for slice boundaries,
    iLoopFilterAlphaC0Offset    = pCodingParam.iLoopFilterAlphaC0Offset;   // AlphaOffset: valid range [-6, 6], default 0
    iLoopFilterBetaOffset       = pCodingParam.iLoopFilterBetaOffset;      // BetaOffset:  valid range [-6, 6], default 0
    iEntropyCodingModeFlag      = pCodingParam.iEntropyCodingModeFlag;
    bEnableFrameCroppingFlag    = pCodingParam.bEnableFrameCroppingFlag;

    /* Rate Control */
    iRCMode = pCodingParam.iRCMode;    // rc mode
    bSimulcastAVC = pCodingParam.bSimulcastAVC;
    iPaddingFlag = pCodingParam.iPaddingFlag;

    iTargetBitrate      = pCodingParam.iTargetBitrate;  // target bitrate
    iMaxBitrate         = pCodingParam.iMaxBitrate;
    if ((iMaxBitrate != UNSPECIFIED_BIT_RATE) && (iMaxBitrate < iTargetBitrate)) {
      iMaxBitrate  = iTargetBitrate;
    }
    iMaxQp = pCodingParam.iMaxQp;
    iMinQp = pCodingParam.iMinQp;
    uiMaxNalSize          = pCodingParam.uiMaxNalSize;
    /* Denoise Control */
    bEnableDenoise = pCodingParam.bEnableDenoise ? true : false;    // Denoise Control  // only support 0 or 1 now

    /* Scene change detection control */
    bEnableSceneChangeDetect   = pCodingParam.bEnableSceneChangeDetect;

    /* Background detection Control */
    bEnableBackgroundDetection = pCodingParam.bEnableBackgroundDetection ? true : false;

    /* Adaptive quantization control */
    bEnableAdaptiveQuant       = pCodingParam.bEnableAdaptiveQuant ? true : false;

    /* Frame skipping */
    bEnableFrameSkip           = pCodingParam.bEnableFrameSkip ? true : false;

    /* Enable int32_t term reference */
    bEnableLongTermReference   = pCodingParam.bEnableLongTermReference ? true : false;
    iLtrMarkPeriod = pCodingParam.iLtrMarkPeriod;
    bIsLosslessLink = pCodingParam.bIsLosslessLink;
    bFixRCOverShoot = pCodingParam.bFixRCOverShoot;
    iIdrBitrateRatio = pCodingParam.iIdrBitrateRatio;
    bPsnrY = pCodingParam.bPsnrY;
    bPsnrU = pCodingParam.bPsnrU;
    bPsnrV = pCodingParam.bPsnrV;
    if (iUsageType == SCREEN_CONTENT_REAL_TIME && !bIsLosslessLink && bEnableLongTermReference) {
      bEnableLongTermReference = false;
    }

    /* For ssei information */
    bEnableSSEI         = pCodingParam.bEnableSSEI;
    bSimulcastAVC       = pCodingParam.bSimulcastAVC;

    /* Layer definition */
    iSpatialLayerNum = (int8_t)WELS_CLIP3 (pCodingParam.iSpatialLayerNum, 1,
                                            MAX_DEPENDENCY_LAYER); // number of dependency(Spatial/CGS) layers used to be encoded
    iTemporalLayerNum = (int8_t)WELS_CLIP3 (pCodingParam.iTemporalLayerNum, 1,
                          MAX_TEMPORAL_LEVEL); // number of temporal layer specified

    uiGopSize           = 1 << (iTemporalLayerNum - 1); // Override GOP size based temporal layer
    iDecompStages       = iTemporalLayerNum - 1;        // WELS_LOG2( uiGopSize );// GOP size dependency
    uiIntraPeriod       = pCodingParam.uiIntraPeriod;// intra period (multiple of GOP size as desired)
    if (uiIntraPeriod == (uint32_t) (-1))
      uiIntraPeriod = 0;
    else if (uiIntraPeriod & (uiGopSize - 1)) // none multiple of GOP size
      uiIntraPeriod = ((uiIntraPeriod + uiGopSize - 1) / uiGopSize) * uiGopSize;

    if (((pCodingParam.iNumRefFrame != AUTO_REF_PIC_COUNT)
         && !((pCodingParam.iNumRefFrame > MAX_REF_PIC_COUNT) || (pCodingParam.iNumRefFrame < MIN_REF_PIC_COUNT)))
        || ((iNumRefFrame != AUTO_REF_PIC_COUNT) && (pCodingParam.iNumRefFrame == AUTO_REF_PIC_COUNT))) {
      iNumRefFrame  = pCodingParam.iNumRefFrame;
    }
    if ((iNumRefFrame != AUTO_REF_PIC_COUNT) && (iNumRefFrame > iMaxNumRefFrame)) {
      iMaxNumRefFrame = iNumRefFrame;
    }
    iLTRRefNum  = (pCodingParam.bEnableLongTermReference ? pCodingParam.iLTRRefNum : 0);
    iLtrMarkPeriod  = pCodingParam.iLtrMarkPeriod;

    bPrefixNalAddingCtrl = pCodingParam.bPrefixNalAddingCtrl;

    if ( (CONSTANT_ID == pCodingParam.eSpsPpsIdStrategy)
        || (INCREASING_ID == pCodingParam.eSpsPpsIdStrategy)
        || (SPS_LISTING == pCodingParam.eSpsPpsIdStrategy)
        || (SPS_LISTING_AND_PPS_INCREASING == pCodingParam.eSpsPpsIdStrategy)
        || (SPS_PPS_LISTING == pCodingParam.eSpsPpsIdStrategy)) {
    eSpsPpsIdStrategy =
      pCodingParam.eSpsPpsIdStrategy;//For SVC meeting application, to avoid mosaic issue caused by cross-IDR reference.
    //SHOULD enable this feature.
    } else {
      // keep the default value
    }

    SSpatialLayerInternal* pDlp        = &sDependencyLayers[0];
    SSpatialLayerConfig* pSpatialLayer = &sSpatialLayers[0];
    EProfileIdc uiProfileIdc           = iEntropyCodingModeFlag ? PRO_HIGH : PRO_BASELINE;
    int8_t iIdxSpatial  = 0;
    while (iIdxSpatial < iSpatialLayerNum) {
      pSpatialLayer->uiProfileIdc      = (pCodingParam.sSpatialLayers[iIdxSpatial].uiProfileIdc == PRO_UNKNOWN) ? uiProfileIdc :
                                      pCodingParam.sSpatialLayers[iIdxSpatial].uiProfileIdc;
      pSpatialLayer->uiLevelIdc        = pCodingParam.sSpatialLayers[iIdxSpatial].uiLevelIdc;

      float fLayerFrameRate     = WELS_CLIP3 (pCodingParam.sSpatialLayers[iIdxSpatial].fFrameRate,
                                          MIN_FRAME_RATE, fParamMaxFrameRate);
      pDlp->fInputFrameRate     = fParamMaxFrameRate;
      pSpatialLayer->fFrameRate =
        pDlp->fOutputFrameRate  = WELS_CLIP3 (fLayerFrameRate, MIN_FRAME_RATE, fParamMaxFrameRate);

#ifdef ENABLE_FRAME_DUMP
      pDlp->sRecFileName[0]     = '\0'; // file to be constructed
#endif//ENABLE_FRAME_DUMP
      pSpatialLayer->iVideoWidth = WELS_CLIP3 (pCodingParam.sSpatialLayers[iIdxSpatial].iVideoWidth, 0,
                                   iPicWidth);  // frame width
      pSpatialLayer->iVideoHeight = WELS_CLIP3 (pCodingParam.sSpatialLayers[iIdxSpatial].iVideoHeight, 0,
                                    iPicHeight);// frame height

      pSpatialLayer->iSpatialBitrate    =
        pCodingParam.sSpatialLayers[iIdxSpatial].iSpatialBitrate;       // target bitrate for current spatial layer
      pSpatialLayer->iMaxSpatialBitrate =
        pCodingParam.sSpatialLayers[iIdxSpatial].iMaxSpatialBitrate;

      if ((iSpatialLayerNum==1) && (iIdxSpatial==0)) {
        if (pSpatialLayer->iVideoWidth == 0) {
          pSpatialLayer->iVideoWidth = iPicWidth;
        }
        if (pSpatialLayer->iVideoHeight == 0) {
          pSpatialLayer->iVideoHeight = iPicHeight;
        }
        if (pSpatialLayer->iSpatialBitrate == 0) {
          pSpatialLayer->iSpatialBitrate = iTargetBitrate;
        }
        if (pSpatialLayer->iMaxSpatialBitrate == 0) {
          pSpatialLayer->iMaxSpatialBitrate = iMaxBitrate;
        }
      }

      //multi slice
      pSpatialLayer->sSliceArgument = pCodingParam.sSpatialLayers[iIdxSpatial].sSliceArgument;

      memcpy (&(pSpatialLayer->sSliceArgument),
              &(pCodingParam.sSpatialLayers[iIdxSpatial].sSliceArgument), // confirmed_safe_unsafe_usage
              sizeof (SSliceArgument)) ;

      pSpatialLayer->iDLayerQp = pCodingParam.sSpatialLayers[iIdxSpatial].iDLayerQp;

      // See codec_app_def.h and parameter_sets.h for more info about members bVideoSignalTypePresent through uiColorMatrix.
      pSpatialLayer->bVideoSignalTypePresent =   pCodingParam.sSpatialLayers[iIdxSpatial].bVideoSignalTypePresent;
      pSpatialLayer->uiVideoFormat =             pCodingParam.sSpatialLayers[iIdxSpatial].uiVideoFormat;
      pSpatialLayer->bFullRange =                pCodingParam.sSpatialLayers[iIdxSpatial].bFullRange;
      pSpatialLayer->bColorDescriptionPresent =  pCodingParam.sSpatialLayers[iIdxSpatial].bColorDescriptionPresent;
      pSpatialLayer->uiColorPrimaries =          pCodingParam.sSpatialLayers[iIdxSpatial].uiColorPrimaries;
      pSpatialLayer->uiTransferCharacteristics = pCodingParam.sSpatialLayers[iIdxSpatial].uiTransferCharacteristics;
      pSpatialLayer->uiColorMatrix =             pCodingParam.sSpatialLayers[iIdxSpatial].uiColorMatrix;

      pSpatialLayer->bAspectRatioPresent = pCodingParam.sSpatialLayers[iIdxSpatial].bAspectRatioPresent;
      pSpatialLayer->eAspectRatio = pCodingParam.sSpatialLayers[iIdxSpatial].eAspectRatio;
      pSpatialLayer->sAspectRatioExtWidth = pCodingParam.sSpatialLayers[iIdxSpatial].sAspectRatioExtWidth;
      pSpatialLayer->sAspectRatioExtHeight = pCodingParam.sSpatialLayers[iIdxSpatial].sAspectRatioExtHeight;

      uiProfileIdc = (!bSimulcastAVC) ? PRO_SCALABLE_BASELINE : uiProfileIdc; //it is used in the D>0 layer if SVC is applied, so set to PRO_SCALABLE_BASELINE
      ++ pDlp;
      ++ pSpatialLayer;
      ++ iIdxSpatial;
    }

    SetActualPicResolution();

    return 0;
  }

// assuming that the width/height ratio of all spatial layers are the same

  void SetActualPicResolution() {
    int32_t iSpatialIdx = iSpatialLayerNum - 1;
    for (; iSpatialIdx >= 0; iSpatialIdx --) {
      SSpatialLayerInternal* pDlayerInternal = &sDependencyLayers[iSpatialIdx];
      SSpatialLayerConfig* pDlayer =  &sSpatialLayers[iSpatialIdx];

      pDlayerInternal->iActualWidth = pDlayer->iVideoWidth;
      pDlayerInternal->iActualHeight = pDlayer->iVideoHeight;
      pDlayer->iVideoWidth = WELS_ALIGN (pDlayerInternal->iActualWidth, MB_WIDTH_LUMA);
      pDlayer->iVideoHeight = WELS_ALIGN (pDlayerInternal->iActualHeight, MB_HEIGHT_LUMA);
    }
  }

  /*!
  * \brief  determined key coding tables for temporal scalability, uiProfileIdc etc for each spatial layer settings
  * \param  SWelsSvcCodingParam, and carried with known GOP size, max, input and output frame rate of each spatial
  * \return NONE (should ensure valid parameter before this procedure)
  */
  int32_t DetermineTemporalSettings() {
    const int32_t iDecStages = WELS_LOG2 (uiGopSize); // (int8_t)GetLogFactor(1.0f, 1.0f * pcfg->uiGopSize);  //log2(uiGopSize)
    const uint8_t* pTemporalIdList = &g_kuiTemporalIdListTable[iDecStages][0];
    SSpatialLayerInternal* pDlp    = &sDependencyLayers[0];
    int8_t i = 0;

    while (i < iSpatialLayerNum) {
      const uint32_t kuiLogFactorInOutRate = GetLogFactor (pDlp->fOutputFrameRate, pDlp->fInputFrameRate);
      const uint32_t kuiLogFactorMaxInRate = GetLogFactor (pDlp->fInputFrameRate, fMaxFrameRate);
      if (UINT_MAX == kuiLogFactorInOutRate || UINT_MAX == kuiLogFactorMaxInRate) {
        return ENC_RETURN_INVALIDINPUT;
      }
      int32_t iNotCodedMask = 0;
      int8_t iMaxTemporalId = 0;

      memset (pDlp->uiCodingIdx2TemporalId, INVALID_TEMPORAL_ID, sizeof (pDlp->uiCodingIdx2TemporalId));
      iNotCodedMask = (1 << (kuiLogFactorInOutRate + kuiLogFactorMaxInRate)) - 1;
      for (uint32_t uiFrameIdx = 0; uiFrameIdx <= uiGopSize; ++ uiFrameIdx) {
        if (0 == (uiFrameIdx & iNotCodedMask)) {
          const int8_t kiTemporalId = pTemporalIdList[uiFrameIdx];
          pDlp->uiCodingIdx2TemporalId[uiFrameIdx] = kiTemporalId;
          if (kiTemporalId > iMaxTemporalId) {
            iMaxTemporalId = kiTemporalId;
          }
        }
      }

      pDlp->iHighestTemporalId   = iMaxTemporalId;
      pDlp->iTemporalResolution  = kuiLogFactorMaxInRate + kuiLogFactorInOutRate;
      pDlp->iDecompositionStages = iDecStages - kuiLogFactorMaxInRate - kuiLogFactorInOutRate;
      if (pDlp->iDecompositionStages < 0) {
        return ENC_RETURN_INVALIDINPUT;
      }
      ++ pDlp;
      ++ i;
    }
    iDecompStages = (int8_t)iDecStages;
    return ENC_RETURN_SUCCESS;
  }

} SWelsSvcCodingParam;


typedef struct TagExistingParasetList {
  SWelsSPS            sSps[MAX_SPS_COUNT];
  SSubsetSps          sSubsetSps[MAX_SPS_COUNT];
  SWelsPPS            sPps[MAX_PPS_COUNT];

  uint32_t            uiInUseSpsNum;
  uint32_t            uiInUseSubsetSpsNum;
  uint32_t            uiInUsePpsNum;
} SExistingParasetList;


static inline int32_t FreeCodingParam (SWelsSvcCodingParam** pParam, CMemoryAlign* pMa) {
  if (pParam == NULL || *pParam == NULL || pMa == NULL)
    return 1;
  pMa->WelsFree (*pParam, "SWelsSvcCodingParam");
  *pParam = NULL;
  return 0;
}

static inline int32_t AllocCodingParam (SWelsSvcCodingParam** pParam, CMemoryAlign* pMa) {
  if (pParam == NULL || pMa == NULL)
    return 1;
  if (*pParam != NULL) {
    FreeCodingParam (pParam, pMa);
  }
  SWelsSvcCodingParam* pCodingParam = (SWelsSvcCodingParam*)pMa->WelsMallocz (sizeof (SWelsSvcCodingParam),
                                      "SWelsSvcCodingParam");
  if (NULL == pCodingParam)
    return 1;
  *pParam = pCodingParam;
  return 0;
}

}//end of namespace WelsEnc

#endif//WELS_ENCODER_PARAMETER_SVC_H__
