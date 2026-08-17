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
 * \file    slice_segment.h
 *
 * \brief   SSlice segment routine (Single slice/multiple slice/fmo arrangement exclusive)
 *
 * \date    2/4/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_SLICE_SEGMENT_H__
#define WELS_SLICE_SEGMENT_H__

#include "typedefs.h"
#include "macros.h"
#include "as264_common.h"
#include "memory_align.h"

#include "codec_app_def.h"
#include "set_mb_syn_cabac.h"

using namespace WelsCommon;

namespace WelsEnc {


// NOTE:
// if PREFIX_NALs are used in base layer(iDid=0, qid=0), MAX_SLICES_NUM will be half of MAX_NAL_UNITS_IN_LAYER in case ST or MT without PACKING_ONE_SLICE_PER_LAYER
// in case MT and PACKING_ONE_SLICE_PER_LAYER, MAX_SLICES_NUM should not be exceeding MAX_LAYER_NUM_OF_FRAME
// for AVC cases, maximal resolution we can support up to (?x1024) for SM_ROWMB_SLICE slice mode
// fine solution for MAX_SLICES_NUM, need us use the variable instead of MACRO for any resolution combining any multiple-slice mode adaptive
#define SAVED_NALUNIT_NUM                       ( (MAX_SPATIAL_LAYER_NUM*MAX_QUALITY_LAYER_NUM) + 1 + MAX_SPATIAL_LAYER_NUM ) // SPS/PPS + SEI/SSEI + PADDING_NAL
#define MAX_SLICES_NUM                          ( ( MAX_NAL_UNITS_IN_LAYER - SAVED_NALUNIT_NUM ) / 3 )  // Also MAX_SLICES_NUM need constrained by implementation: uiSliceIdc allocated in SSliceCtx.pOverallMbMap need a byte range as expected
#define AVERSLICENUM_CONSTRAINT         (MAX_SLICES_NUM)                        // used in sNalList initialization,

#define MIN_NUM_MB_PER_SLICE                    48                                                      // (128/16 * 96/16), addressing the lowest resolution for multiple slicing is 128x96 above

#define DEFAULT_MAXPACKETSIZE_CONSTRAINT        (1200)          //in bytes
//#define MINPACKETSIZE_CONSTRAINT                (1200)

#define AVER_MARGIN_BYTES                       ( 100 ) //in bytes
#define JUMPPACKETSIZE_CONSTRAINT(max_byte)             ( max_byte - AVER_MARGIN_BYTES ) //in bytes
#define JUMPPACKETSIZE_JUDGE(len,mb_idx,max_byte)       ( (len) > JUMPPACKETSIZE_CONSTRAINT(max_byte) ) //( (mb_idx+1)%40/*16slice for compare*/ == 0 )        //
//cur_mb_idx is for early tests, can be omit in optimization
typedef struct TagSlice      SSlice;
typedef struct TagDqLayer    SDqLayer;
typedef struct TagWelsEncCtx sWelsEncCtx;
/*!
 * \brief   SSlice context
 */
/* Single/multiple slices */
typedef struct SlicepEncCtx_s {
SliceModeEnum           uiSliceMode;            /* 0: single slice in frame; 1: multiple slices in frame; */
int16_t                 iMbWidth;               /* width of picture size in mb */
int16_t                 iMbHeight;              /* height of picture size in mb */
int32_t                 iSliceNumInFrame;       /* count number of slices in frame; */
int32_t                 iMbNumInFrame;          /* count number of MBs in frame */
uint16_t*               pOverallMbMap;          /* overall MB map in frame, store virtual slice idc; */
uint32_t                uiSliceSizeConstraint;  /* in byte */
int32_t                 iMaxSliceNumConstraint; /* maximal number of slices constraint */

} SSliceCtx;


typedef struct TagDynamicSlicingStack {
int32_t         iStartPos;
int32_t         iCurrentPos;

uint8_t*        pBsStackBufPtr; // current writing position
uint32_t        uiBsStackCurBits;
int32_t         iBsStackLeftBits;

SCabacCtx       sStoredCabac;
int32_t         iMbSkipRunStack;
uint8_t         uiLastMbQp;
uint8_t*        pRestoreBuffer;
} SDynamicSlicingStack;

/*!
 * \brief   Initialize Wels SSlice context (Single/multiple slices and FMO)
 *
 * \param   pCurDq          current layer which its SSlice context will be initialized
 * \param   bFmoUseFlag     flag of using fmo
 * \param   iMbWidth        MB width
 * \param   iMbHeight       MB height
 * \param   uiSliceMode     slice mode
 * \param   mul_slice_arg   argument for multiple slice if it is applicable
 * \param   pPpsArg         argument for pPps parameter
 *
 * \return  0 - successful; none 0 - failed;
 */
int32_t InitSlicePEncCtx (SDqLayer* pCurDq,
                          CMemoryAlign* pMa,
                          bool bFmoUseFlag,
                          int32_t iMbWidth,
                          int32_t iMbHeight,
                          SSliceArgument* pSliceArgument,
                          void* pPpsArg);


/*!
 * \brief   Uninitialize Wels SSlice context (Single/multiple slices and FMO)
 *
 * \param   pCurDq       curent layer which its SSlice context will be initialized
 *
 * \return  NONE;
 */
void UninitSlicePEncCtx (SDqLayer* pCurDq, CMemoryAlign* pMa);

/*!
 * \brief   Get slice idc for given iMbXY (apply in Single/multiple slices and FMO)
 *
 * \param   pCurDq    current layer info
 * \param   kiMbXY    MB xy index
 *
 * \return  uiSliceIdc - successful; (uint8_t)(-1) - failed;
 */
uint16_t WelsMbToSliceIdc (SDqLayer* pCurDq, const int32_t kiMbXY);

/*!
 * \brief   Get first mb in slice/slice_group: uiSliceIdc (apply in Single/multiple slices and FMO)
 *
 * \param   pCurLayer       current layer
 * \param   kiSliceIdc      slice idc
 *
 * \return  first_mb - successful; -1 - failed;
 */
int32_t WelsGetFirstMbOfSlice (SDqLayer* pCurLayer, const int32_t kiSliceIdc);

/*!
 * \brief   Get successive mb to be processed in slice/slice_group: uiSliceIdc (apply in Single/multiple slices and FMO)
 *
 * \param   pCurDq       current layer info
 * \param   kiMbXY       MB xy index
 *
 * \return  next_mb - successful; -1 - failed;
 */
int32_t WelsGetNextMbOfSlice (SDqLayer* pCurDq, const int32_t kiMbXY);

/*!
 * \brief   Get previous mb to be processed in slice/slice_group: uiSliceIdc (apply in Single/multiple slices and FMO)
 *
 * \param   pCurDq          current layer info
 * \param   kiMbXY          MB xy index
 *
 * \return  prev_mb - successful; -1 - failed;
 */
int32_t WelsGetPrevMbOfSlice (SDqLayer* pCurDq, const int32_t kiMbXY);

/*!
 * \brief   Get number of mb in slice/slice_group: uiSliceIdc (apply in Single/multiple slices and FMO)
 *
 * \param   pCurDq          current layer info
 * \param   pSlice          slice which request slice num
 * \param   kiSliceIdc      slice/slice_group idc
 *
 * \return  count_num_of_mb - successful; -1 - failed;
 */
int32_t WelsGetNumMbInSlice (SDqLayer* pCurDq, SSlice* pSlice, const int32_t kuiSliceIdc);

/*!
 *  Get slice count for multiple slice segment
 *
 */
int32_t GetInitialSliceNum (SSliceArgument* pSliceArgument);
int32_t GetCurrentSliceNum (const SDqLayer* pCurDq);
SSlice* GetSliceByIndex(sWelsEncCtx* pCtx, const int32_t kiSliceIdc);

//checking valid para
int32_t DynamicMaxSliceNumConstraint (uint32_t uiMaximumNum, int32_t uiConsumedNum, uint32_t uiDulplicateTimes);

bool CheckFixedSliceNumMultiSliceSetting (const int32_t kiMbNumInFrame,  SSliceArgument* pSliceArg);
bool CheckRasterMultiSliceSetting (const int32_t kiMbNumInFrame, SSliceArgument* pSliceArg);
bool CheckRowMbMultiSliceSetting (const int32_t kiMbWidth,  SSliceArgument* pSliceArg);

bool GomValidCheckSliceNum (const int32_t kiMbWidth, const int32_t kiMbHeight, uint32_t* pSliceNum);
bool GomValidCheckSliceMbNum (const int32_t kiMbWidth, const int32_t kiMbHeight,  SSliceArgument* pSliceArg);
//end of checking valid para

int32_t DynamicAdjustSlicePEncCtxAll (SDqLayer* pCurDq,
                                      int32_t* pRunLength);
}
#endif//WELS_SLICE_SEGMENT_H__
