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



#ifndef WELS_VIDEO_CODEC_APPLICATION_DEFINITION_H__
#define WELS_VIDEO_CODEC_APPLICATION_DEFINITION_H__
/**
  * @file  codec_app_def.h
  * @brief Data and /or structures introduced in Cisco OpenH264 application
*/

#include "codec_def.h"
/* Constants */
#define MAX_TEMPORAL_LAYER_NUM          4
#define MAX_SPATIAL_LAYER_NUM           4
#define MAX_QUALITY_LAYER_NUM           4

#define MAX_LAYER_NUM_OF_FRAME          128
#define MAX_NAL_UNITS_IN_LAYER          128     ///< predetermined here, adjust it later if need

#define MAX_RTP_PAYLOAD_LEN             1000
#define AVERAGE_RTP_PAYLOAD_LEN         800


#define SAVED_NALUNIT_NUM_TMP           ( (MAX_SPATIAL_LAYER_NUM*MAX_QUALITY_LAYER_NUM) + 1 + MAX_SPATIAL_LAYER_NUM )  ///< SPS/PPS + SEI/SSEI + PADDING_NAL
#define MAX_SLICES_NUM_TMP              ( ( MAX_NAL_UNITS_IN_LAYER - SAVED_NALUNIT_NUM_TMP ) / 3 )


#define AUTO_REF_PIC_COUNT  -1          ///< encoder selects the number of reference frame automatically
#define UNSPECIFIED_BIT_RATE 0          ///< to do: add detail comment

/**
 * @brief Struct of OpenH264 version
 */
///
/// E.g. SDK version is 1.2.0.0, major version number is 1, minor version number is 2, and revision number is 0.
typedef struct  _tagVersion {
  unsigned int uMajor;                  ///< The major version number
  unsigned int uMinor;                  ///< The minor version number
  unsigned int uRevision;               ///< The revision number
  unsigned int uReserved;               ///< The reserved number, it should be 0.
} OpenH264Version;

/**
* @brief Decoding status
*/
typedef enum {
  /**
  * Errors derived from bitstream parsing
  */
  dsErrorFree = 0x00,   ///< bit stream error-free
  dsFramePending = 0x01,   ///< need more throughput to generate a frame output,
  dsRefLost = 0x02,   ///< layer lost at reference frame with temporal id 0
  dsBitstreamError = 0x04,   ///< error bitstreams(maybe broken internal frame) the decoder cared
  dsDepLayerLost = 0x08,   ///< dependented layer is ever lost
  dsNoParamSets = 0x10,   ///< no parameter set NALs involved
  dsDataErrorConcealed = 0x20,   ///< current data error concealed specified
  dsRefListNullPtrs = 0x40, ///<ref picure list contains null ptrs within uiRefCount range

  /**
  * Errors derived from logic level
  */
  dsInvalidArgument     = 0x1000, ///< invalid argument specified
  dsInitialOptExpected  = 0x2000, ///< initializing operation is expected
  dsOutOfMemory         = 0x4000, ///< out of memory due to new request
  /**
  * ANY OTHERS?
  */
  dsDstBufNeedExpan     = 0x8000  ///< actual picture size exceeds size of dst pBuffer feed in decoder, so need expand its size

} DECODING_STATE;

/**
* @brief Option types introduced in SVC encoder application
*/
typedef enum {
  ENCODER_OPTION_DATAFORMAT = 0,
  ENCODER_OPTION_IDR_INTERVAL,               ///< IDR period,0/-1 means no Intra period (only the first frame); lager than 0 means the desired IDR period, must be multiple of (2^temporal_layer)
  ENCODER_OPTION_SVC_ENCODE_PARAM_BASE,      ///< structure of Base Param
  ENCODER_OPTION_SVC_ENCODE_PARAM_EXT,       ///< structure of Extension Param
  ENCODER_OPTION_FRAME_RATE,                 ///< maximal input frame rate, current supported range: MAX_FRAME_RATE = 30,MIN_FRAME_RATE = 1
  ENCODER_OPTION_BITRATE,
  ENCODER_OPTION_MAX_BITRATE,
  ENCODER_OPTION_INTER_SPATIAL_PRED,
  ENCODER_OPTION_RC_MODE,
  ENCODER_OPTION_RC_FRAME_SKIP,
  ENCODER_PADDING_PADDING,                   ///< 0:disable padding;1:padding

  ENCODER_OPTION_PROFILE,                    ///< assgin the profile for each layer
  ENCODER_OPTION_LEVEL,                      ///< assgin the level for each layer
  ENCODER_OPTION_NUMBER_REF,                 ///< the number of refererence frame
  ENCODER_OPTION_DELIVERY_STATUS,            ///< the delivery info which is a feedback from app level

  ENCODER_LTR_RECOVERY_REQUEST,
  ENCODER_LTR_MARKING_FEEDBACK,
  ENCODER_LTR_MARKING_PERIOD,
  ENCODER_OPTION_LTR,                        ///< 0:disable LTR;larger than 0 enable LTR; LTR number is fixed to be 2 in current encoder
  ENCODER_OPTION_COMPLEXITY,

  ENCODER_OPTION_ENABLE_SSEI,                ///< enable SSEI: true--enable ssei; false--disable ssei
  ENCODER_OPTION_ENABLE_PREFIX_NAL_ADDING,   ///< enable prefix: true--enable prefix; false--disable prefix
  ENCODER_OPTION_SPS_PPS_ID_STRATEGY, ///< different stategy in adjust ID in SPS/PPS: 0- constant ID, 1-additional ID, 6-mapping and additional

  ENCODER_OPTION_CURRENT_PATH,
  ENCODER_OPTION_DUMP_FILE,                  ///< dump layer reconstruct frame to a specified file
  ENCODER_OPTION_TRACE_LEVEL,                ///< trace info based on the trace level
  ENCODER_OPTION_TRACE_CALLBACK,             ///< a void (*)(void* context, int level, const char* message) function which receives log messages
  ENCODER_OPTION_TRACE_CALLBACK_CONTEXT,     ///< context info of trace callback

  ENCODER_OPTION_GET_STATISTICS,             ///< read only
  ENCODER_OPTION_STATISTICS_LOG_INTERVAL,    ///< log interval in millisecond

  ENCODER_OPTION_IS_LOSSLESS_LINK,            ///< advanced algorithmetic settings

  ENCODER_OPTION_BITS_VARY_PERCENTAGE        ///< bit vary percentage
} ENCODER_OPTION;

/**
* @brief Option types introduced in decoder application
*/
typedef enum {
  DECODER_OPTION_END_OF_STREAM = 1,     ///< end of stream flag
  DECODER_OPTION_VCL_NAL,               ///< feedback whether or not have VCL NAL in current AU for application layer
  DECODER_OPTION_TEMPORAL_ID,           ///< feedback temporal id for application layer
  DECODER_OPTION_FRAME_NUM,             ///< feedback current decoded frame number
  DECODER_OPTION_IDR_PIC_ID,            ///< feedback current frame belong to which IDR period
  DECODER_OPTION_LTR_MARKING_FLAG,      ///< feedback wether current frame mark a LTR
  DECODER_OPTION_LTR_MARKED_FRAME_NUM,  ///< feedback frame num marked by current Frame
  DECODER_OPTION_ERROR_CON_IDC,         ///< indicate decoder error concealment method
  DECODER_OPTION_TRACE_LEVEL,
  DECODER_OPTION_TRACE_CALLBACK,        ///< a void (*)(void* context, int level, const char* message) function which receives log messages
  DECODER_OPTION_TRACE_CALLBACK_CONTEXT,///< context info of trace callbac

  DECODER_OPTION_GET_STATISTICS,        ///< feedback decoder statistics
  DECODER_OPTION_GET_SAR_INFO,          ///< feedback decoder Sample Aspect Ratio info in Vui
  DECODER_OPTION_PROFILE,               ///< get current AU profile info, only is used in GetOption
  DECODER_OPTION_LEVEL,                 ///< get current AU level info,only is used in GetOption
  DECODER_OPTION_STATISTICS_LOG_INTERVAL,///< set log output interval
  DECODER_OPTION_IS_REF_PIC,             ///< feedback current frame is ref pic or not
  DECODER_OPTION_NUM_OF_FRAMES_REMAINING_IN_BUFFER,  ///< number of frames remaining in decoder buffer when pictures are required to re-ordered into display-order.
  DECODER_OPTION_NUM_OF_THREADS,         ///< number of decoding threads. The maximum thread count is equal or less than lesser of (cpu core counts and 16).
} DECODER_OPTION;

/**
* @brief Enumerate the type of error concealment methods
*/
typedef enum {
  ERROR_CON_DISABLE = 0,
  ERROR_CON_FRAME_COPY,
  ERROR_CON_SLICE_COPY,
  ERROR_CON_FRAME_COPY_CROSS_IDR,
  ERROR_CON_SLICE_COPY_CROSS_IDR,
  ERROR_CON_SLICE_COPY_CROSS_IDR_FREEZE_RES_CHANGE,
  ERROR_CON_SLICE_MV_COPY_CROSS_IDR,
  ERROR_CON_SLICE_MV_COPY_CROSS_IDR_FREEZE_RES_CHANGE
} ERROR_CON_IDC;
/**
* @brief Feedback that whether or not have VCL NAL in current AU
*/
typedef enum {
  FEEDBACK_NON_VCL_NAL = 0,
  FEEDBACK_VCL_NAL,
  FEEDBACK_UNKNOWN_NAL
} FEEDBACK_VCL_NAL_IN_AU;

/**
* @brief Type of layer being encoded
*/
typedef enum {
  NON_VIDEO_CODING_LAYER = 0,
  VIDEO_CODING_LAYER = 1
} LAYER_TYPE;

/**
* @brief Spatial layer num
*/
typedef enum {
  SPATIAL_LAYER_0 = 0,
  SPATIAL_LAYER_1 = 1,
  SPATIAL_LAYER_2 = 2,
  SPATIAL_LAYER_3 = 3,
  SPATIAL_LAYER_ALL = 4
} LAYER_NUM;

/**
* @brief Enumerate the type of video bitstream which is provided to decoder
*/
typedef enum {
  VIDEO_BITSTREAM_AVC               = 0,
  VIDEO_BITSTREAM_SVC               = 1,
  VIDEO_BITSTREAM_DEFAULT           = VIDEO_BITSTREAM_SVC
} VIDEO_BITSTREAM_TYPE;

/**
* @brief Enumerate the type of key frame request
*/
typedef enum {
  NO_RECOVERY_REQUSET  = 0,
  LTR_RECOVERY_REQUEST = 1,
  IDR_RECOVERY_REQUEST = 2,
  NO_LTR_MARKING_FEEDBACK = 3,
  LTR_MARKING_SUCCESS = 4,
  LTR_MARKING_FAILED = 5
} KEY_FRAME_REQUEST_TYPE;

/**
* @brief Structure for LTR recover request
*/
typedef struct {
  unsigned int uiFeedbackType;       ///< IDR request or LTR recovery request
  unsigned int uiIDRPicId;           ///< distinguish request from different IDR
  int          iLastCorrectFrameNum;
  int          iCurrentFrameNum;     ///< specify current decoder frame_num.
  int          iLayerId;           //specify the layer for recovery request
} SLTRRecoverRequest;

/**
* @brief Structure for LTR marking feedback
*/
typedef struct {
  unsigned int  uiFeedbackType; ///< mark failed or successful
  unsigned int  uiIDRPicId;     ///< distinguish request from different IDR
  int           iLTRFrameNum;   ///< specify current decoder frame_num
  int           iLayerId;        //specify the layer for LTR marking feedback
} SLTRMarkingFeedback;

/**
* @brief Structure for LTR configuration
*/
typedef struct {
  bool   bEnableLongTermReference; ///< 1: on, 0: off
  int    iLTRRefNum;               ///< TODO: not supported to set it arbitrary yet
} SLTRConfig;

/**
* @brief Enumerate the type of rate control mode
*/
typedef enum {
  RC_QUALITY_MODE = 0,     ///< quality mode
  RC_BITRATE_MODE = 1,     ///< bitrate mode
  RC_BUFFERBASED_MODE = 2, ///< no bitrate control,only using buffer status,adjust the video quality
  RC_TIMESTAMP_MODE = 3, //rate control based timestamp
  RC_BITRATE_MODE_POST_SKIP = 4, ///< this is in-building RC MODE, WILL BE DELETED after algorithm tuning!
  RC_OFF_MODE = -1,         ///< rate control off mode
} RC_MODES;

/**
* @brief Enumerate the type of profile id
*/
typedef enum {
  PRO_UNKNOWN   = 0,
  PRO_BASELINE  = 66,
  PRO_MAIN      = 77,
  PRO_EXTENDED  = 88,
  PRO_HIGH      = 100,
  PRO_HIGH10    = 110,
  PRO_HIGH422   = 122,
  PRO_HIGH444   = 144,
  PRO_CAVLC444  = 244,

  PRO_SCALABLE_BASELINE = 83,
  PRO_SCALABLE_HIGH     = 86
} EProfileIdc;

/**
* @brief Enumerate the type of level id
*/
typedef enum {
  LEVEL_UNKNOWN = 0,
  LEVEL_1_0 = 10,
  LEVEL_1_B = 9,
  LEVEL_1_1 = 11,
  LEVEL_1_2 = 12,
  LEVEL_1_3 = 13,
  LEVEL_2_0 = 20,
  LEVEL_2_1 = 21,
  LEVEL_2_2 = 22,
  LEVEL_3_0 = 30,
  LEVEL_3_1 = 31,
  LEVEL_3_2 = 32,
  LEVEL_4_0 = 40,
  LEVEL_4_1 = 41,
  LEVEL_4_2 = 42,
  LEVEL_5_0 = 50,
  LEVEL_5_1 = 51,
  LEVEL_5_2 = 52
} ELevelIdc;

/**
* @brief Enumerate the type of wels log
*/
enum {
  WELS_LOG_QUIET       = 0x00,          ///< quiet mode
  WELS_LOG_ERROR       = 1 << 0,        ///< error log iLevel
  WELS_LOG_WARNING     = 1 << 1,        ///< Warning log iLevel
  WELS_LOG_INFO        = 1 << 2,        ///< information log iLevel
  WELS_LOG_DEBUG       = 1 << 3,        ///< debug log, critical algo log
  WELS_LOG_DETAIL      = 1 << 4,        ///< per packet/frame log
  WELS_LOG_RESV        = 1 << 5,        ///< resversed log iLevel
  WELS_LOG_LEVEL_COUNT = 6,
  WELS_LOG_DEFAULT     = WELS_LOG_WARNING   ///< default log iLevel in Wels codec
};

/**
 * @brief Enumerate the type of slice mode
 */
typedef enum {
  SM_SINGLE_SLICE         = 0, ///< | SliceNum==1
  SM_FIXEDSLCNUM_SLICE    = 1, ///< | according to SliceNum        | enabled dynamic slicing for multi-thread
  SM_RASTER_SLICE         = 2, ///< | according to SlicesAssign    | need input of MB numbers each slice. In addition, if other constraint in SSliceArgument is presented, need to follow the constraints. Typically if MB num and slice size are both constrained, re-encoding may be involved.
  SM_SIZELIMITED_SLICE           = 3, ///< | according to SliceSize       | slicing according to size, the slicing will be dynamic(have no idea about slice_nums until encoding current frame)
  SM_RESERVED             = 4
} SliceModeEnum;

/**
 * @brief Structure for slice argument
 */
typedef struct {
  SliceModeEnum uiSliceMode;    ///< by default, uiSliceMode will be SM_SINGLE_SLICE
  unsigned int
  uiSliceNum;     ///< only used when uiSliceMode=1, when uiSliceNum=0 means auto design it with cpu core number
  unsigned int
  uiSliceMbNum[MAX_SLICES_NUM_TMP]; ///< only used when uiSliceMode=2; when =0 means setting one MB row a slice
  unsigned int  uiSliceSizeConstraint; ///< now only used when uiSliceMode=4
} SSliceArgument;

/**
* @brief Enumerate the type of video format
*/
typedef enum {
  VF_COMPONENT,
  VF_PAL,
  VF_NTSC,
  VF_SECAM,
  VF_MAC,
  VF_UNDEF,
  VF_NUM_ENUM
} EVideoFormatSPS;  // EVideoFormat is already defined/used elsewhere!

/**
* @brief Enumerate the type of color primaries
*/
typedef enum {
  CP_RESERVED0,
  CP_BT709,
  CP_UNDEF,
  CP_RESERVED3,
  CP_BT470M,
  CP_BT470BG,
  CP_SMPTE170M,
  CP_SMPTE240M,
  CP_FILM,
  CP_BT2020,
  CP_NUM_ENUM
} EColorPrimaries;

/**
* @brief Enumerate the type of transfer characteristics
*/
typedef enum {
  TRC_RESERVED0,
  TRC_BT709,
  TRC_UNDEF,
  TRC_RESERVED3,
  TRC_BT470M,
  TRC_BT470BG,
  TRC_SMPTE170M,
  TRC_SMPTE240M,
  TRC_LINEAR,
  TRC_LOG100,
  TRC_LOG316,
  TRC_IEC61966_2_4,
  TRC_BT1361E,
  TRC_IEC61966_2_1,
  TRC_BT2020_10,
  TRC_BT2020_12,
  TRC_NUM_ENUM
} ETransferCharacteristics;

/**
* @brief Enumerate the type of color matrix
*/
typedef enum {
  CM_GBR,
  CM_BT709,
  CM_UNDEF,
  CM_RESERVED3,
  CM_FCC,
  CM_BT470BG,
  CM_SMPTE170M,
  CM_SMPTE240M,
  CM_YCGCO,
  CM_BT2020NC,
  CM_BT2020C,
  CM_NUM_ENUM
} EColorMatrix;


/**
* @brief Enumerate the type of sample aspect ratio
*/
typedef enum {
  ASP_UNSPECIFIED = 0,
  ASP_1x1 = 1,
  ASP_12x11 = 2,
  ASP_10x11 = 3,
  ASP_16x11 = 4,
  ASP_40x33 = 5,
  ASP_24x11 = 6,
  ASP_20x11 = 7,
  ASP_32x11 = 8,
  ASP_80x33 = 9,
  ASP_18x11 = 10,
  ASP_15x11 = 11,
  ASP_64x33 = 12,
  ASP_160x99 = 13,

  ASP_EXT_SAR = 255
} ESampleAspectRatio;


/**
* @brief  Structure for spatial layer configuration
*/
typedef struct {
  int   iVideoWidth;           ///< width of picture in luminance samples of a layer
  int   iVideoHeight;          ///< height of picture in luminance samples of a layer
  float fFrameRate;            ///< frame rate specified for a layer
  int   iSpatialBitrate;       ///< target bitrate for a spatial layer, in unit of bps
  int   iMaxSpatialBitrate;    ///< maximum  bitrate for a spatial layer, in unit of bps
  EProfileIdc  uiProfileIdc;   ///< value of profile IDC (PRO_UNKNOWN for auto-detection)
  ELevelIdc    uiLevelIdc;     ///< value of profile IDC (0 for auto-detection)
  int          iDLayerQp;      ///< value of level IDC (0 for auto-detection)

  SSliceArgument sSliceArgument;

  // Note: members bVideoSignalTypePresent through uiColorMatrix below are also defined in SWelsSPS in parameter_sets.h.
  bool      bVideoSignalTypePresent;  // false => do not write any of the following information to the header
  unsigned char
  uiVideoFormat;        // EVideoFormatSPS; 3 bits in header; 0-5 => component, kpal, ntsc, secam, mac, undef
  bool      bFullRange;         // false => analog video data range [16, 235]; true => full data range [0,255]
  bool      bColorDescriptionPresent; // false => do not write any of the following three items to the header
  unsigned char
  uiColorPrimaries;     // EColorPrimaries; 8 bits in header; 0 - 9 => ???, bt709, undef, ???, bt470m, bt470bg,
  //    smpte170m, smpte240m, film, bt2020
  unsigned char
  uiTransferCharacteristics;  // ETransferCharacteristics; 8 bits in header; 0 - 15 => ???, bt709, undef, ???, bt470m, bt470bg, smpte170m,
  //   smpte240m, linear, log100, log316, iec61966-2-4, bt1361e, iec61966-2-1, bt2020-10, bt2020-12
  unsigned char
  uiColorMatrix;        // EColorMatrix; 8 bits in header (corresponds to FFmpeg "colorspace"); 0 - 10 => GBR, bt709,
  //   undef, ???, fcc, bt470bg, smpte170m, smpte240m, YCgCo, bt2020nc, bt2020c

  bool bAspectRatioPresent; ///< aspect ratio present in VUI
  ESampleAspectRatio eAspectRatio; ///< aspect ratio idc
  unsigned short sAspectRatioExtWidth; ///< use if aspect ratio idc == 255
  unsigned short sAspectRatioExtHeight; ///< use if aspect ratio idc == 255

} SSpatialLayerConfig;

/**
* @brief Encoder usage type
*/
typedef enum {
  CAMERA_VIDEO_REAL_TIME,      ///< camera video for real-time communication
  SCREEN_CONTENT_REAL_TIME,    ///< screen content signal
  CAMERA_VIDEO_NON_REAL_TIME,
  SCREEN_CONTENT_NON_REAL_TIME,
  INPUT_CONTENT_TYPE_ALL,
} EUsageType;

/**
* @brief Enumulate the complexity mode
*/
typedef enum {
  LOW_COMPLEXITY = 0,              ///< the lowest compleixty,the fastest speed,
  MEDIUM_COMPLEXITY,          ///< medium complexity, medium speed,medium quality
  HIGH_COMPLEXITY             ///< high complexity, lowest speed, high quality
} ECOMPLEXITY_MODE;

/**
 * @brief Enumulate for the stategy of SPS/PPS strategy
 */
typedef enum {
  CONSTANT_ID = 0,           ///< constant id in SPS/PPS
  INCREASING_ID = 0x01,      ///< SPS/PPS id increases at each IDR
  SPS_LISTING  = 0x02,       ///< using SPS in the existing list if possible
  SPS_LISTING_AND_PPS_INCREASING  = 0x03,
  SPS_PPS_LISTING  = 0x06,
} EParameterSetStrategy;

// TODO:  Refine the parameters definition.
/**
* @brief SVC Encoding Parameters
*/
typedef struct TagEncParamBase {
  EUsageType
  iUsageType;                 ///< application type; please refer to the definition of EUsageType

  int       iPicWidth;        ///< width of picture in luminance samples (the maximum of all layers if multiple spatial layers presents)
  int       iPicHeight;       ///< height of picture in luminance samples((the maximum of all layers if multiple spatial layers presents)
  int       iTargetBitrate;   ///< target bitrate desired, in unit of bps
  RC_MODES  iRCMode;          ///< rate control mode
  float     fMaxFrameRate;    ///< maximal input frame rate

} SEncParamBase, *PEncParamBase;

/**
* @brief SVC Encoding Parameters extention
*/
typedef struct TagEncParamExt {
  EUsageType
  iUsageType;                          ///< same as in TagEncParamBase

  int       iPicWidth;                 ///< same as in TagEncParamBase
  int       iPicHeight;                ///< same as in TagEncParamBase
  int       iTargetBitrate;            ///< same as in TagEncParamBase
  RC_MODES  iRCMode;                   ///< same as in TagEncParamBase
  float     fMaxFrameRate;             ///< same as in TagEncParamBase

  int       iTemporalLayerNum;         ///< temporal layer number, max temporal layer = 4
  int       iSpatialLayerNum;          ///< spatial layer number,1<= iSpatialLayerNum <= MAX_SPATIAL_LAYER_NUM, MAX_SPATIAL_LAYER_NUM = 4
  SSpatialLayerConfig sSpatialLayers[MAX_SPATIAL_LAYER_NUM];

  ECOMPLEXITY_MODE iComplexityMode;
  unsigned int      uiIntraPeriod;     ///< period of Intra frame
  int               iNumRefFrame;      ///< number of reference frame used
  EParameterSetStrategy
  eSpsPpsIdStrategy;       ///< different stategy in adjust ID in SPS/PPS: 0- constant ID, 1-additional ID, 6-mapping and additional
  bool    bPrefixNalAddingCtrl;        ///< false:not use Prefix NAL; true: use Prefix NAL
  bool    bEnableSSEI;                 ///< false:not use SSEI; true: use SSEI -- TODO: planning to remove the interface of SSEI
  bool    bSimulcastAVC;               ///< (when encoding more than 1 spatial layer) false: use SVC syntax for higher layers; true: use Simulcast AVC
  int     iPaddingFlag;                ///< 0:disable padding;1:padding
  int     iEntropyCodingModeFlag;      ///< 0:CAVLC  1:CABAC.

  /* rc control */
  bool    bEnableFrameSkip;            ///< False: don't skip frame even if VBV buffer overflow.True: allow skipping frames to keep the bitrate within limits
  int     iMaxBitrate;                 ///< the maximum bitrate, in unit of bps, set it to UNSPECIFIED_BIT_RATE if not needed
  int     iMaxQp;                      ///< the maximum QP encoder supports
  int     iMinQp;                      ///< the minmum QP encoder supports
  unsigned int uiMaxNalSize;           ///< the maximum NAL size.  This value should be not 0 for dynamic slice mode

  /*LTR settings*/
  bool     bEnableLongTermReference;   ///< 1: on, 0: off
  int      iLTRRefNum;                 ///< the number of LTR(long term reference),TODO: not supported to set it arbitrary yet
  unsigned int      iLtrMarkPeriod;    ///< the LTR marked period that is used in feedback.
  /* multi-thread settings*/
  unsigned short
  iMultipleThreadIdc;                  ///< 1 # 0: auto(dynamic imp. internal encoder); 1: multiple threads imp. disabled; lager than 1: count number of threads;
  bool  bUseLoadBalancing; ///< only used when uiSliceMode=1 or 3, will change slicing of a picture during the run-time of multi-thread encoding, so the result of each run may be different

  /* Deblocking loop filter */
  int       iLoopFilterDisableIdc;     ///< 0: on, 1: off, 2: on except for slice boundaries
  int       iLoopFilterAlphaC0Offset;  ///< AlphaOffset: valid range [-6, 6], default 0
  int       iLoopFilterBetaOffset;     ///< BetaOffset: valid range [-6, 6], default 0
  /*pre-processing feature*/
  bool    bEnableDenoise;              ///< denoise control
  bool    bEnableBackgroundDetection;  ///< background detection control //VAA_BACKGROUND_DETECTION //BGD cmd
  bool    bEnableAdaptiveQuant;        ///< adaptive quantization control
  bool    bEnableFrameCroppingFlag;    ///< enable frame cropping flag: TRUE always in application
  bool    bEnableSceneChangeDetect;

  bool    bIsLosslessLink;             ///< LTR advanced setting
  bool    bFixRCOverShoot;             ///< fix rate control overshooting
  int     iIdrBitrateRatio;            ///< the target bits of IDR is (idr_bitrate_ratio/100) * average target bit per frame.
  bool    bPsnrY;                      ///< get Y PSNR stats for the whole video sequence
  bool    bPsnrU;                      ///< get U PSNR stats for the whole video sequence
  bool    bPsnrV;                      ///< get V PSNR stats for the whole video sequence
} SEncParamExt;

/**
* @brief Define a new struct to show the property of video bitstream.
*/
typedef struct {
  unsigned int          size;          ///< size of the struct
  VIDEO_BITSTREAM_TYPE  eVideoBsType;  ///< video stream type (AVC/SVC)
} SVideoProperty;

/**
* @brief SVC Decoding Parameters, reserved here and potential applicable in the future
*/
typedef struct TagSVCDecodingParam {
  char*     pFileNameRestructed;       ///< file name of reconstructed frame used for PSNR calculation based debug

  unsigned int  uiCpuLoad;             ///< CPU load
  unsigned char uiTargetDqLayer;       ///< setting target dq layer id

  ERROR_CON_IDC eEcActiveIdc;          ///< whether active error concealment feature in decoder
  bool bParseOnly;                     ///< decoder for parse only, no reconstruction. When it is true, SPS/PPS size should not exceed SPS_PPS_BS_SIZE (128). Otherwise, it will return error info

  SVideoProperty   sVideoProperty;    ///< video stream property
} SDecodingParam, *PDecodingParam;

/**
* @brief Bitstream inforamtion of a layer being encoded
*/
typedef struct {
  unsigned char uiTemporalId;
  unsigned char uiSpatialId;
  unsigned char uiQualityId;
  EVideoFrameType eFrameType;
  unsigned char uiLayerType;

  /**
   * The sub sequence layers are ordered hierarchically based on their dependency on each other so that any picture in a layer shall not be
   * predicted from any picture on any higher layer.
  */
  int   iSubSeqId;                ///< refer to D.2.11 Sub-sequence information SEI message semantics
  int   iNalCount;              ///< count number of NAL coded already
  int*  pNalLengthInByte;       ///< length of NAL size in byte from 0 to iNalCount-1
  unsigned char*  pBsBuf;       ///< buffer of bitstream contained
  float rPsnr[3];               ///< PSNR values for Y/U/V
} SLayerBSInfo, *PLayerBSInfo;

/**
* @brief Frame bit stream info
*/
typedef struct {
  int           iLayerNum;
  SLayerBSInfo  sLayerInfo[MAX_LAYER_NUM_OF_FRAME];

  EVideoFrameType eFrameType;
  int iFrameSizeInBytes;
  long long uiTimeStamp;
} SFrameBSInfo, *PFrameBSInfo;

/**
*  @brief Structure for source picture
*/
typedef struct Source_Picture_s {
  int       iColorFormat;          ///< color space type
  int       iStride[4];            ///< stride for each plane pData
  unsigned char*  pData[4];        ///< plane pData
  int       iPicWidth;             ///< luma picture width in x coordinate
  int       iPicHeight;            ///< luma picture height in y coordinate
  long long uiTimeStamp;           ///< timestamp of the source picture, unit: millisecond
  bool      bPsnrY;                ///< get Y PSNR for this frame
  bool      bPsnrU;                ///< get U PSNR for this frame
  bool      bPsnrV;                ///< get V PSNR for this frame
} SSourcePicture;

/**
* @brief Structure for bit rate info
*/
typedef struct TagBitrateInfo {
  LAYER_NUM iLayer;
  int iBitrate;                    ///< the maximum bitrate
} SBitrateInfo;

/**
* @brief Structure for dump layer info
*/
typedef struct TagDumpLayer {
  int iLayer;
  char* pFileName;
} SDumpLayer;

/**
* @brief Structure for profile info in layer
*
*/
typedef struct TagProfileInfo {
  int iLayer;
  EProfileIdc uiProfileIdc;        ///< the profile info
} SProfileInfo;

/**
* @brief  Structure for level info in layer
*
*/
typedef struct TagLevelInfo {
  int iLayer;
  ELevelIdc uiLevelIdc;            ///< the level info
} SLevelInfo;
/**
* @brief Structure for dilivery status
*
*/
typedef struct TagDeliveryStatus {
  bool bDeliveryFlag;              ///< 0: the previous frame isn't delivered,1: the previous frame is delivered
  int iDropFrameType;              ///< the frame type that is dropped; reserved
  int iDropFrameSize;              ///< the frame size that is dropped; reserved
} SDeliveryStatus;

/**
* @brief The capability of decoder, for SDP negotiation
*/
typedef struct TagDecoderCapability {
  int iProfileIdc;     ///< profile_idc
  int iProfileIop;     ///< profile-iop
  int iLevelIdc;       ///< level_idc
  int iMaxMbps;        ///< max-mbps
  int iMaxFs;          ///< max-fs
  int iMaxCpb;         ///< max-cpb
  int iMaxDpb;         ///< max-dpb
  int iMaxBr;          ///< max-br
  bool bRedPicCap;     ///< redundant-pic-cap
} SDecoderCapability;

/**
* @brief Structure for parse only output
*/
typedef struct TagParserBsInfo {
  int iNalNum;                                 ///< total NAL number in current AU
  int* pNalLenInByte;  ///< each nal length
  unsigned char* pDstBuff;                     ///< outputted dst buffer for parsed bitstream
  int iSpsWidthInPixel;                        ///< required SPS width info
  int iSpsHeightInPixel;                       ///< required SPS height info
  unsigned long long uiInBsTimeStamp;               ///< input BS timestamp
  unsigned long long uiOutBsTimeStamp;             ///< output BS timestamp
} SParserBsInfo, *PParserBsInfo;

/**
* @brief Structure for encoder statistics
*/
typedef struct TagVideoEncoderStatistics {
  unsigned int uiWidth;                        ///< the width of encoded frame
  unsigned int uiHeight;                       ///< the height of encoded frame
  //following standard, will be 16x aligned, if there are multiple spatial, this is of the highest
  float fAverageFrameSpeedInMs;                ///< average_Encoding_Time

  // rate control related
  float fAverageFrameRate;                     ///< the average frame rate in, calculate since encoding starts, supposed that the input timestamp is in unit of ms
  float fLatestFrameRate;                      ///< the frame rate in, in the last second, supposed that the input timestamp is in unit of ms (? useful for checking BR, but is it easy to calculate?
  unsigned int uiBitRate;                      ///< sendrate in Bits per second, calculated within the set time-window
  unsigned int uiAverageFrameQP;                    ///< the average QP of last encoded frame

  unsigned int uiInputFrameCount;              ///< number of frames
  unsigned int uiSkippedFrameCount;            ///< number of frames

  unsigned int uiResolutionChangeTimes;        ///< uiResolutionChangeTimes
  unsigned int uiIDRReqNum;                    ///< number of IDR requests
  unsigned int uiIDRSentNum;                   ///< number of actual IDRs sent
  unsigned int uiLTRSentNum;                   ///< number of LTR sent/marked

  long long    iStatisticsTs;                  ///< Timestamp of updating the statistics

  unsigned long iTotalEncodedBytes;
  unsigned long iLastStatisticsBytes;
  unsigned long iLastStatisticsFrameCount;
} SEncoderStatistics;

/**
* @brief  Structure for decoder statistics
*/
typedef struct TagVideoDecoderStatistics {
  unsigned int uiWidth;                        ///< the width of encode/decode frame
  unsigned int uiHeight;                       ///< the height of encode/decode frame
  float fAverageFrameSpeedInMs;                ///< average_Decoding_Time
  float fActualAverageFrameSpeedInMs;          ///< actual average_Decoding_Time, including freezing pictures
  unsigned int uiDecodedFrameCount;            ///< number of frames
  unsigned int uiResolutionChangeTimes;        ///< uiResolutionChangeTimes
  unsigned int uiIDRCorrectNum;                ///< number of correct IDR received
  //EC on related
  unsigned int
  uiAvgEcRatio;                                ///< when EC is on, the average ratio of total EC areas, can be an indicator of reconstruction quality
  unsigned int
  uiAvgEcPropRatio;                            ///< when EC is on, the rough average ratio of propogate EC areas, can be an indicator of reconstruction quality
  unsigned int uiEcIDRNum;                     ///< number of actual unintegrity IDR or not received but eced
  unsigned int uiEcFrameNum;                   ///<
  unsigned int uiIDRLostNum;                   ///< number of whole lost IDR
  unsigned int
  uiFreezingIDRNum;               ///< number of freezing IDR with error (partly received), under resolution change
  unsigned int uiFreezingNonIDRNum;            ///< number of freezing non-IDR with error
  int iAvgLumaQp;                              ///< average luma QP. default: -1, no correct frame outputted
  int iSpsReportErrorNum;                      ///< number of Sps Invalid report
  int iSubSpsReportErrorNum;                   ///< number of SubSps Invalid report
  int iPpsReportErrorNum;                      ///< number of Pps Invalid report
  int iSpsNoExistNalNum;                       ///< number of Sps NoExist Nal
  int iSubSpsNoExistNalNum;                    ///< number of SubSps NoExist Nal
  int iPpsNoExistNalNum;                       ///< number of Pps NoExist Nal

  unsigned int uiProfile;                ///< Profile idc in syntax
  unsigned int uiLevel;                  ///< level idc according to Annex A-1

  int iCurrentActiveSpsId;                     ///< current active SPS id
  int iCurrentActivePpsId;                     ///< current active PPS id

  unsigned int iStatisticsLogInterval;                  ///< frame interval of statistics log
} SDecoderStatistics; // in building, coming soon

/**
* @brief Structure for sample aspect ratio (SAR) info in VUI
*/
typedef struct TagVuiSarInfo {
  unsigned int uiSarWidth;                     ///< SAR width
  unsigned int uiSarHeight;                    ///< SAR height
  bool bOverscanAppropriateFlag;               ///< SAR overscan flag
} SVuiSarInfo, *PVuiSarInfo;

#endif//WELS_VIDEO_CODEC_APPLICATION_DEFINITION_H__
