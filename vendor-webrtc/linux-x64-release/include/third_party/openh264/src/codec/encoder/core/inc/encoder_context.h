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
 * \file    encoder_context.h
 *
 * \brief   Main pData to be operated over Wels encoder all modules
 *
 * \date    2/4/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_ENCODER_CONTEXT_H__
#define WELS_ENCODER_CONTEXT_H__

#include <stdio.h>
#include "typedefs.h"
#include "param_svc.h"
#include "nal_encap.h"
#include "picture.h"
#include "paraset_strategy.h"
#include "dq_map.h"
#include "stat.h"
#include "macros.h"
#include "rc.h"
#include "as264_common.h"
#include "wels_preprocess.h"
#include "wels_func_ptr_def.h"
#include "crt_util_safe_x.h"
#include "utils.h"

#include "mt_defs.h" // for multiple threadin,
#include "WelsThreadLib.h"
#include "wels_task_management.h"

namespace WelsEnc {

class IWelsTaskManage;
class IWelsReferenceStrategy;

/*
 *  reference list for each quality layer in SVC
 */
typedef struct TagRefList {
  SPicture*     pShortRefList[1 + MAX_SHORT_REF_COUNT]; // reference list 0 - int16_t
  SPicture*     pLongRefList[1 + MAX_REF_PIC_COUNT];    // reference list 1 - int32_t
  SPicture*     pNextBuffer;
  SPicture*     pRef[1 + MAX_REF_PIC_COUNT];    // plus 1 for swap intend
  uint8_t       uiShortRefCount;
  uint8_t       uiLongRefCount; // dependend on pRef pic module
} SRefList;

typedef struct TagLTRState {
// LTR mark feedback
  uint32_t      uiLtrMarkState; // LTR mark state, indicate whether there is a LTR mark feedback unsolved
  int32_t       iLtrMarkFbFrameNum;// the unsolved LTR mark feedback, the marked iFrameNum feedback from decoder

// LTR used as recovery reference
  int32_t       iLastRecoverFrameNum; // reserve the last LTR or IDR recover iFrameNum
  int32_t       iLastCorFrameNumDec; // reserved the last correct position in decoder side, use to select valid LTR to recover or to decide the LTR mark validation
  int32_t       iCurFrameNumInDec; // current iFrameNum in decoder side, use to select valid LTR to recover or to decide the LTR mark validation

// LTR mark
  int32_t       iLTRMarkMode; // direct mark or delay mark
  int32_t       iLTRMarkSuccessNum; //successful marked num, for mark mode switch
  int32_t       iCurLtrIdx;// current int32_t term reference index to mark
  int32_t       iLastLtrIdx[MAX_TEMPORAL_LAYER_NUM];
  int32_t       iSceneLtrIdx;// related to Scene LTR, used by screen content

  uint32_t      uiLtrMarkInterval;// the interval from the last int32_t term pRef mark

  bool          bLTRMarkingFlag;        //decide whether current frame marked as LTR
  bool          bLTRMarkEnable; //when LTR is confirmed and the interval is no smaller than the marking period
  bool          bReceivedT0LostFlag;    // indicate whether a t0 lost feedback is recieved, for LTR recovery
} SLTRState;

typedef struct TagSpatialPicIndex {
  SPicture*     pSrc;   // I420 based and after color space converted
  int32_t       iDid;   // dependency id
} SSpatialPicIndex;

typedef struct TagStrideTables {
  int32_t*      pStrideDecBlockOffset[MAX_DEPENDENCY_LAYER][2]; // [iDid][tid==0][24 x 4]: luma+chroma= 24 x 4
  int32_t*      pStrideEncBlockOffset[MAX_DEPENDENCY_LAYER];    // [iDid][24 x 4]: luma+chroma= 24 x 4
  int16_t*      pMbIndexX[MAX_DEPENDENCY_LAYER];                // [iDid][iMbX]: map for iMbX in each spatial layer coding
  int16_t*      pMbIndexY[MAX_DEPENDENCY_LAYER];                // [iDid][iMbY]: map for iMbY in each spatial layer coding
} SStrideTables;

typedef struct TagWelsEncCtx {
  SLogContext sLogCtx;
// Input
  SWelsSvcCodingParam* pSvcParam;   // SVC parameter, WelsSVCParamConfig in svc_param_settings.h

  int32_t*          pSadCostMb;
  /* MVD cost tables for Inter MB */
  int32_t           iMvRange;
  uint16_t*         pMvdCostTable; //[52];  // adaptive to spatial layers
  int32_t           iMvdCostTableSize; //the size of above table
  int32_t           iMvdCostTableStride; //the stride of above table
  SMVUnitXY*
  pMvUnitBlock4x4;      // (*pMvUnitBlock4x4[2])[MB_BLOCK4x4_NUM];          // for store each 4x4 blocks' mv unit, the two swap after different d layer
  int8_t*
  pRefIndexBlock4x4;    // (*pRefIndexBlock4x4[2])[MB_BLOCK8x8_NUM];        // for store each 4x4 blocks' pRef index, the two swap after different d layer
  int8_t*           pNonZeroCountBlocks;     // (*pNonZeroCountBlocks)[MB_LUMA_CHROMA_BLOCK4x4_NUM];
  int8_t*
  pIntra4x4PredModeBlocks;      // (*pIntra4x4PredModeBlocks)[INTRA_4x4_MODE_NUM];  //last byte is not used; the first 4 byte is for the bottom 12,13,14,15 4x4 block intra mode, and 3 byte for (3,7,11)

  SMB**             ppMbListD;     // [MAX_DEPENDENCY_LAYER];
  SStrideTables*    pStrideTab;     // stride tables for internal coding used
  SWelsFuncPtrList* pFuncList;

  SSliceThreading*  pSliceThreading;
  IWelsTaskManage*  pTaskManage; //was planning to put it under CWelsH264SVCEncoder but it may be updated (lock/no lock) when param is changed
  IWelsReferenceStrategy* pReferenceStrategy;

  // pointers
  SPicture*         pEncPic;                // pointer to current picture to be encoded
  SPicture*         pDecPic;                // pointer to current picture being reconstructed
  SPicture*         pRefPic;                // pointer to current reference picture

  SDqLayer*         pCurDqLayer;            // DQ layer context used to being encoded currently, for reference base layer to refer: pCurDqLayer->pRefLayer if applicable
  SDqLayer**        ppDqLayerList;          // overall DQ layers encoded for storage

  SRefList**        ppRefPicListExt;        // reference picture list for SVC
  SPicture*         pRefList0[16];
  SLTRState*        pLtr;//[MAX_DEPENDENCY_LAYER];
  bool              bCurFrameMarkedAsSceneLtr;
// Derived

  EWelsSliceType    eSliceType;             // currently coding slice type
  EWelsNalUnitType  eNalType;               // NAL type
  EWelsNalRefIdc    eNalPriority;           // NAL_Reference_Idc currently
  EWelsNalRefIdc    eLastNalPriority[MAX_DEPENDENCY_LAYER];       // NAL_Reference_Idc in last frame
  uint8_t           iNumRef0;

  uint8_t           uiDependencyId;         // Idc of dependecy layer to be coded
  uint8_t           uiTemporalId;           // Idc of temporal layer to be coded
  bool              bNeedPrefixNalFlag;     // whether add prefix nal

// Rate control routine
  SWelsSvcRc*       pWelsSvcRc;
  bool              bCheckWindowStatusRefreshFlag;
  int64_t           iCheckWindowStartTs;
  int64_t           iCheckWindowCurrentTs;
  int32_t           iCheckWindowInterval;
  int32_t           iCheckWindowIntervalShift;
  bool              bCheckWindowShiftResetFlag;
  int32_t           iGlobalQp;      // global qp

// VAA
  SVAAFrameInfo*    pVaa;           // VAA information of reference
  CWelsPreProcess*  pVpp;

  SWelsSPS*         pSpsArray;      // MAX_SPS_COUNT by standard compatible
  SWelsSPS*         pSps;
  SWelsPPS*         pPPSArray;      // MAX_PPS_COUNT by standard compatible
  SWelsPPS*         pPps;
  /* SVC only */
  SSubsetSps*       pSubsetArray;   // MAX_SPS_COUNT by standard compatible
  SSubsetSps*       pSubsetSps;
  int32_t           iSpsNum;        // number of pSps used
  int32_t           iSubsetSpsNum;  // number of pSps used
  int32_t           iPpsNum;        // number of pPps used

// Output
  SWelsEncoderOutput* pOut;         // for NAL raw pData (need allocating memory for sNalList internal)
  uint8_t*          pFrameBs;       // restoring bitstream pBuffer of all NALs in a frame
  int32_t           iFrameBsSize;   // count size of frame bs in bytes allocated
  int32_t           iPosBsBuffer;   // current writing position of frame bs pBuffer

  SSpatialPicIndex  sSpatialIndexMap[MAX_DEPENDENCY_LAYER];
  int32_t           iSliceBufferSize[MAX_DEPENDENCY_LAYER];

  bool              bRefOfCurTidIsLtr[MAX_DEPENDENCY_LAYER][MAX_TEMPORAL_LEVEL];
 // uint16_t          uiIdrPicId;           // IDR picture id: [0, 65535], this one is used for LTR
  int32_t           iMaxSliceCount;// maximal count number of slices for all layers observation
  int16_t           iActiveThreadsNum;      // number of threads active so far

  /*
   * DQ layer idc map for svc encoding, might be a better scheme than that of design before,
   * can aware idc of referencing layer and that idc of successive layer to be coded
   */
  /* SVC only */
  SDqIdc*           pDqIdcMap;    // overall DQ map of full scalability in specific frame (All full D/T/Q layers involved)                                                                                                // pDqIdcMap[dq_index] for each SDqIdc pData

  SParaSetOffset    sPSOVector;
  SParaSetOffset*   pPSOVector;
  CMemoryAlign*     pMemAlign;

#if defined(STAT_OUTPUT)
// overall stat pData, refer to SStatData in stat.h, in case avc to use stat[0][0]
  SStatData         sStatData [ MAX_DEPENDENCY_LAYER ] [ MAX_QUALITY_LEVEL ];
  SStatSliceInfo    sPerInfo;
#endif//STAT_OUTPUT

  //related to Statistics
  int64_t            uiStartTimestamp;
  SEncoderStatistics sEncoderStatistics[MAX_DEPENDENCY_LAYER];
  int32_t            iStatisticsLogInterval;
  int64_t            iLastStatisticsLogTs;

  int32_t iEncoderError;
  WELS_MUTEX mutexEncoderError;
  bool bDeliveryFlag;
  SStateCtx sWelsCabacContexts[4][WELS_QP_MAX + 1][WELS_CONTEXT_COUNT];
#ifdef ENABLE_FRAME_DUMP
  bool bDependencyRecFlag[MAX_DEPENDENCY_LAYER];
#endif
  int64_t            uiLastTimestamp;
  uint8_t*           pDynamicBsBuffer[MAX_THREADS_NUM];
} sWelsEncCtx/*, *PWelsEncCtx*/;
}
#endif//sWelsEncCtx_H__
