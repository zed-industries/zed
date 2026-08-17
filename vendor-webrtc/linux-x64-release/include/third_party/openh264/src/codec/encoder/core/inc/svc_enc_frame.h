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

//wels_svc_layer.h
#ifndef WELS_SVC_EXTENSION_LAYER_H__
#define WELS_SVC_EXTENSION_LAYER_H__

#include "typedefs.h"
#include "wels_const.h"
#include "wels_common_basis.h"
#include "parameter_sets.h"
#include "slice.h"
#include "picture.h"
#include "svc_enc_macroblock.h"


#include "svc_enc_slice_segment.h"
namespace WelsEnc {

/*
 *  Frame level in SVC DQLayer instead.
 *  Dependency-Quaility layer struction definition for SVC extension of H.264/AVC
 */

///////////////////////////////////DQ Layer level///////////////////////////////////

typedef struct TagDqLayer   SDqLayer;
typedef SDqLayer*           pDqLayer;

typedef struct TagFeatureSearchPreparation {
SScreenBlockFeatureStorage*     pRefBlockFeature;//point the the ref frame storage

uint16_t*       pFeatureOfBlock;                // Feature of every block (8x8), begin with the point
uint8_t         uiFeatureStrategyIndex;// index of hash strategy

/* for FME frame-level switch */
bool bFMESwitchFlag;
uint8_t uiFMEGoodFrameCount;
int32_t iHighFreMbCount;
} SFeatureSearchPreparation; //maintain only one

typedef struct TagSliceBufferInfo {
SSlice*                 pSliceBuffer;  // slice buffer for multi thread,
int32_t                 iMaxSliceNum;
int32_t                 iCodedSliceNum;
}SSliceBufferInfo;

typedef struct TagLayerInfo {
SNalUnitHeaderExt       sNalHeaderExt;
SSubsetSps*             pSubsetSpsP;    // current pSubsetSps used, memory alloc in external
SWelsSPS*               pSpsP;          // current pSps based avc used, memory alloc in external
SWelsPPS*               pPpsP;          // current pPps used
} SLayerInfo;
/* Layer Representation */
struct TagDqLayer {
SLayerInfo              sLayerInfo;
SSliceBufferInfo        sSliceBufferInfo[MAX_THREADS_NUM];
SSlice**                ppSliceInLayer;
SSliceCtx               sSliceEncCtx;   // current slice context
uint8_t*                pCsData[3];     // pointer to reconstructed picture pData
int32_t                 iCsStride[3];   // Cs stride

uint8_t*                pEncData[3];    // pData picture to be encoded in current layer
int32_t                 iEncStride[3];  // pData picture stride

SMB*                    sMbDataP;       // pointer to mb of mbAddr equal to 0 in slice, mb_data_ptr = mb_base_ptr + (1+iMbStride).
int16_t                 iMbWidth;       // MB width of this picture, equal to pSps.iMbWidth
int16_t                 iMbHeight;      // MB height of this picture, equal to pSps.iMbHeight;

bool                    bBaseLayerAvailableFlag;        // whether base layer is available for prediction?
bool                    bSatdInMdFlag; // whether SATD is calculated in ME and integer-pel MD

uint8_t                 iLoopFilterDisableIdc;  // 0: on, 1: off, 2: on except for slice boundaries
int8_t                  iLoopFilterAlphaC0Offset;// AlphaOffset: valid range [-6, 6], default 0
int8_t                  iLoopFilterBetaOffset;  // BetaOffset:  valid range [-6, 6], default 0
uint8_t                 uiDisableInterLayerDeblockingFilterIdc;
int8_t                  iInterLayerSliceAlphaC0Offset;
int8_t                  iInterLayerSliceBetaOffset;
bool                    bDeblockingParallelFlag; //parallel_deblocking_flag

SPicture*               pRefPic;        // reference picture pointer
SPicture*               pDecPic;        // reconstruction picture pointer for layer
SPicture*               pRefOri[MAX_REF_PIC_COUNT];

bool                    bThreadSlcBufferFlag;
bool                    bSliceBsBufferFlag;
int32_t                 iMaxSliceNum;
int32_t                 NumSliceCodedOfPartition[MAX_THREADS_NUM];      // for dynamic slicing mode
int32_t                 LastCodedMbIdxOfPartition[MAX_THREADS_NUM];     // for dynamic slicing mode
int32_t                 FirstMbIdxOfPartition[MAX_THREADS_NUM];         // for dynamic slicing mode
int32_t                 EndMbIdxOfPartition[MAX_THREADS_NUM];           // for dynamic slicing mode
int32_t*                pFirstMbIdxOfSlice;
int32_t*                pCountMbNumInSlice;

bool                    bNeedAdjustingSlicing;

SFeatureSearchPreparation* pFeatureSearchPreparation;

SDqLayer*               pRefLayer;              // pointer to referencing dq_layer of current layer to be decoded
};

///////////////////////////////////////////////////////////////////////

// frame structure for svc
typedef SDqLayer SWelsSvcFrame;
}
#endif//WELS_SVC_EXTENSION_LAYER_H__
