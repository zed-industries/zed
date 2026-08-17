/*!
 * \copy
 *     Copyright (c)  2004-2013, Cisco Systems
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
 *  ratectl.c
 *
 *  Abstract
 *      Include file for ratectl.c
 *
 *  History
 *      9/8/2004 Created
 *    12/26/2011 Modified
 *
 *
 *************************************************************************/
#ifndef RC_H
#define RC_H


#include "codec_app_def.h"
#include "svc_enc_macroblock.h"
#include "slice.h"

namespace WelsEnc {

typedef struct TagWelsEncCtx sWelsEncCtx;

//trace
#define GOM_TRACE_FLAG 0
#define GOM_H_SCC               8

enum {
  BITS_NORMAL,
  BITS_LIMITED,
  BITS_EXCEEDED
};

enum {
//virtual gop size
  VGOP_SIZE             = 8,

//qp information
  GOM_MIN_QP_MODE       = 12,
  GOM_MAX_QP_MODE       = 36,
  MAX_LOW_BR_QP         = 42,
  MIN_IDR_QP            = 26,
  MAX_IDR_QP            = 32,
  MIN_SCREEN_QP         = 26,
  MAX_SCREEN_QP         = 35,
  DELTA_QP              = 2,
  DELTA_QP_BGD_THD      = 3,
  QP_MIN_VALUE          = 0,
  QP_MAX_VALUE          = 51,

//frame skip constants
  SKIP_QP_90P           = 24,
  SKIP_QP_180P          = 24,
  SKIP_QP_360P          = 31,
  SKIP_QP_720P          = 31,
  LAST_FRAME_QP_RANGE_UPPER_MODE0  = 3,
  LAST_FRAME_QP_RANGE_LOWER_MODE0  = 2,
  LAST_FRAME_QP_RANGE_UPPER_MODE1  = 5,
  LAST_FRAME_QP_RANGE_LOWER_MODE1  = 3,

  MB_WIDTH_THRESHOLD_90P   = 15,
  MB_WIDTH_THRESHOLD_180P  = 30,
  MB_WIDTH_THRESHOLD_360P  = 60,

//Mode 0 parameter
  GOM_ROW_MODE0_90P     = 2,
  GOM_ROW_MODE0_180P    = 2,
  GOM_ROW_MODE0_360P    = 4,
  GOM_ROW_MODE0_720P    = 4,
  QP_RANGE_MODE0        = 3,

//Mode 1 parameter
  GOM_ROW_MODE1_90P     = 1,
  GOM_ROW_MODE1_180P    = 1,
  GOM_ROW_MODE1_360P    = 2,
  GOM_ROW_MODE1_720P    = 2,
  QP_RANGE_UPPER_MODE1  = 9,
  QP_RANGE_LOWER_MODE1  = 4,
  QP_RANGE_INTRA_MODE1  = 3
};

//bits allocation
#define MAX_BITS_VARY_PERCENTAGE 100 //bits vary range in percentage
#define MAX_BITS_VARY_PERCENTAGE_x3d2 150 //bits vary range in percentage * 3/2
#define INT_MULTIPLY 100 // use to multiply in Double to Int Conversion, should be same as AQ_QSTEP_INT_MULTIPLY in WelsVP
#define WEIGHT_MULTIPLY 2000
#define REMAIN_BITS_TH (1)
#define VGOP_BITS_PERCENTAGE_DIFF 5
#define IDR_BITRATE_RATIO  4
#define FRAME_iTargetBits_VARY_RANGE 50 // *INT_MULTIPLY
//R-Q Model
#define LINEAR_MODEL_DECAY_FACTOR 80 // *INT_MULTIPLY
#define FRAME_CMPLX_RATIO_RANGE 20 // *INT_MULTIPLY
#define SMOOTH_FACTOR_MIN_VALUE 2 // *INT_MULTIPLY
//#define VGOP_BITS_MIN_RATIO 0.8
//skip and padding
#define TIME_CHECK_WINDOW 5000 // ms
#define SKIP_RATIO  50 // *INT_MULTIPLY
#define LAST_FRAME_PREDICT_WEIGHT 0.5
#define PADDING_BUFFER_RATIO 50 // *INT_MULTIPLY
#define PADDING_THRESHOLD    5 //*INT_MULTIPLY

#define VIRTUAL_BUFFER_LOW_TH   120 //*INT_MULTIPLY
#define VIRTUAL_BUFFER_HIGH_TH  180 //*INT_MULTIPLY

#define _BITS_RANGE 0

enum {
  EVEN_TIME_WINDOW  =0,
  ODD_TIME_WINDOW   =1,
  TIME_WINDOW_TOTAL =2
};

typedef struct TagRCTemporal {
int32_t   iMinBitsTl;
int32_t   iMaxBitsTl;
int32_t   iTlayerWeight;
int32_t   iGopBitsDq;
//P frame level R-Q Model
int64_t   iLinearCmplx; // *INT_MULTIPLY
int32_t   iPFrameNum;
int64_t   iFrameCmplxMean;
int32_t   iMaxQp;
int32_t   iMinQp;
} SRCTemporal;

typedef struct TagWelsRc {
int32_t   iRcVaryPercentage;
int32_t   iRcVaryRatio;

int32_t   iInitialQp; //initial qp
int64_t   iBitRate; // Note: although the max bit rate is 240000*1200 which can be represented by int32, but there are many multipler of this iBitRate in the calculation of RC, so use int64 to avoid type conversion at all such places
int32_t   iPreviousBitrate;
int32_t   iPreviousGopSize;
double    fFrameRate;
int32_t   iBitsPerFrame;
int32_t   iMaxBitsPerFrame;
double    dPreviousFps;

// bits allocation and status
int32_t   iLastAllocatedBits;
int32_t   iRemainingBits;
int32_t   iBitsPerMb;
int32_t   iTargetBits;
int32_t   iCurrentBitsLevel;//0:normal; 1:limited; 2:exceeded.

int32_t   iIdrNum;
int64_t   iIntraComplexity; //255*255(MaxMbSAD)*36864(MaxFS) make the highest bit of 32-bit integer 1
int32_t   iIntraMbCount;
int64_t   iIntraComplxMean;

int8_t    iTlOfFrames[VGOP_SIZE];
int32_t   iRemainingWeights;
int32_t   iFrameDqBits;

bool       bGomRC;
double*    pGomComplexity;
int32_t*   pGomForegroundBlockNum;
int32_t*   pCurrentFrameGomSad;
int32_t*   pGomCost;

int32_t   bEnableGomQp;
int32_t   iAverageFrameQp;
int32_t   iMinFrameQp;
int32_t   iMaxFrameQp;
int32_t   iNumberMbFrame;
int32_t   iNumberMbGom;
int32_t   iGomSize;

int32_t   iSkipFrameNum;
int32_t   iFrameCodedInVGop;
int32_t   iSkipFrameInVGop;
int32_t   iGopNumberInVGop;
int32_t   iGopIndexInVGop;

int32_t   iSkipQpValue;
int32_t   iQpRangeUpperInFrame;
int32_t   iQpRangeLowerInFrame;
int32_t   iMinQp;
int32_t   iMaxQp;
//int32_t   delta_adaptive_qp;
int32_t   iSkipBufferRatio;

int32_t   iQStep; // *INT_MULTIPLY
int32_t   iFrameDeltaQpUpper;
int32_t   iFrameDeltaQpLower;
int32_t   iLastCalculatedQScale;

//for skip frame and padding
int32_t   iBufferSizeSkip;
int64_t   iBufferFullnessSkip;
int64_t   iBufferMaxBRFullness[TIME_WINDOW_TOTAL];//0: EVEN_TIME_WINDOW; 1: ODD_TIME_WINDOW
int32_t   iPredFrameBit;
bool      bNeedShiftWindowCheck[TIME_WINDOW_TOTAL];
int32_t   iBufferSizePadding;
int32_t   iBufferFullnessPadding;
int32_t   iPaddingSize;
int32_t   iPaddingBitrateStat;
bool      bSkipFlag;
int32_t   iContinualSkipFrames;
SRCTemporal* pTemporalOverRc;

//for scc
int64_t     iAvgCost2Bits;
int64_t     iCost2BitsIntra;
int32_t    iBaseQp;
long long  uiLastTimeStamp;

//for statistics and online adjustments
int32_t   iActualBitRate; // TODO: to complete later
float     fLatestFrameRate; // TODO: to complete later
} SWelsSvcRc;

typedef  void (*PWelsRCPictureInitFunc) (sWelsEncCtx* pCtx,long long uiTimeStamp);
typedef  void (*PWelsRCPictureDelayJudgeFunc) (sWelsEncCtx* pCtx,long long uiTimeStamp,int32_t iDidIdx);
typedef  void (*PWelsRCPictureInfoUpdateFunc) (sWelsEncCtx* pCtx, int32_t iLayerSize);
typedef  void (*PWelsRCMBInfoUpdateFunc) (sWelsEncCtx* pCtx, SMB* pCurMb, int32_t iCostLuma, SSlice* pSlice);
typedef  void (*PWelsRCMBInitFunc) (sWelsEncCtx* pCtx, SMB* pCurMb, SSlice* pSlice);
typedef  void (*PWelsCheckFrameSkipBasedMaxbrFunc) (sWelsEncCtx* pCtx, const long long uiTimeStamp, int32_t iDidIdx);
typedef  void (*PWelsUpdateBufferWhenFrameSkippedFunc)(sWelsEncCtx* pCtx, int32_t iSpatialNum);
typedef  void (*PWelsUpdateMaxBrCheckWindowStatusFunc)(sWelsEncCtx* pCtx, int32_t iSpatialNum, const long long uiTimeStamp);
typedef  bool (*PWelsRCPostFrameSkippingFunc)(sWelsEncCtx* pCtx, const int32_t iDid, const long long uiTimeStamp);

typedef  struct  WelsRcFunc_s {
PWelsRCPictureInitFunc          pfWelsRcPictureInit;
PWelsRCPictureDelayJudgeFunc    pfWelsRcPicDelayJudge;
PWelsRCPictureInfoUpdateFunc    pfWelsRcPictureInfoUpdate;
PWelsRCMBInitFunc               pfWelsRcMbInit;
PWelsRCMBInfoUpdateFunc         pfWelsRcMbInfoUpdate;
PWelsCheckFrameSkipBasedMaxbrFunc pfWelsCheckSkipBasedMaxbr;
PWelsUpdateBufferWhenFrameSkippedFunc pfWelsUpdateBufferWhenSkip;
PWelsUpdateMaxBrCheckWindowStatusFunc pfWelsUpdateMaxBrWindowStatus;

PWelsRCPostFrameSkippingFunc    pfWelsRcPostFrameSkipping;
} SWelsRcFunc;

void GomRCInitForOneSlice(SSlice* pSlice, const int32_t kiBitsPerMb);
void CheckFrameSkipBasedMaxbr (sWelsEncCtx* pCtx,const long long uiTimeStamp, int32_t iDidIdx);
void UpdateBufferWhenFrameSkipped(sWelsEncCtx* pCtx, int32_t iSpatialNum);
void UpdateMaxBrCheckWindowStatus(sWelsEncCtx* pCtx, int32_t iSpatialNum, const long long uiTimeStamp);
bool WelsRcPostFrameSkipping(sWelsEncCtx* pCtx, const int32_t iDid, const long long uiTimeStamp);
void WelsRcPostFrameSkippedUpdate (sWelsEncCtx* pCtx, const int32_t iDid);

void RcTraceFrameBits (sWelsEncCtx* pEncCtx, long long uiTimeStamp, int32_t iFrameSize);
void WelsRcInitModule (sWelsEncCtx* pCtx, RC_MODES iRcMode);
void WelsRcInitFuncPointers (sWelsEncCtx* pEncCtx, RC_MODES iRcMode);
void WelsRcFreeMemory (sWelsEncCtx* pCtx);
bool WelsRcCheckFrameStatus (sWelsEncCtx* pEncCtx,long long uiTimeStamp,int32_t iSpatialNum,int32_t iCurDid);
bool WelsUpdateSkipFrameStatus();
long long GetTimestampForRc(const long long uiTimeStamp, const long long uiLastTimeStamp, const float fFrameRate);

}
#endif //RC_H
