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
 * \file    md.h
 *
 * \brief   mode decision
 *
 * \date    2009.5.14 Created
 *
 *************************************************************************************
 */
#ifndef WELS_MACROBLOCK_MODE_DECISION_H__
#define WELS_MACROBLOCK_MODE_DECISION_H__

#include "svc_motion_estimate.h"
#include "svc_enc_macroblock.h"
#include "encode_mb_aux.h"
#include "wels_func_ptr_def.h"

namespace WelsEnc {
#define ME_REFINE_BUF_STRIDE       32
#define ME_REFINE_BUF_WIDTH_BLK4   8
#define ME_REFINE_BUF_WIDTH_BLK8   16
#define ME_REFINE_BUF_STRIDE_BLK4  160
#define ME_REFINE_BUF_STRIDE_BLK8  320

#define REFINE_ME_NO_BEST_HALF_PIXEL 0 //( 0,  0)
#define REFINE_ME_HALF_PIXEL_LEFT    3 //(-2,  0)
#define REFINE_ME_HALF_PIXEL_RIGHT   4 //( 2,  0)
#define REFINE_ME_HALF_PIXEL_TOP     1 //( 0, -2)
#define REFINE_ME_HALF_PIXEL_BOTTOM  2 //( 0,  2)

#define ME_NO_BEST_QUAR_PIXEL 1 //( 0,  0) or best half pixel
#define ME_QUAR_PIXEL_LEFT    2 //(-1,  0)
#define ME_QUAR_PIXEL_RIGHT   3 //( 1,  0)
#define ME_QUAR_PIXEL_TOP     4 //( 0, -1)
#define ME_QUAR_PIXEL_BOTTOM  5 //( 0,  1)

#define NO_BEST_FRAC_PIX   1 // REFINE_ME_NO_BEST_HALF_PIXEL + ME_NO_BEST_QUAR_PIXEL

//for vaa constants
#define MBVAASIGN_FLAT       15
#define MBVAASIGN_HOR1      3
#define MBVAASIGN_HOR2      12
#define MBVAASIGN_VER1       5
#define MBVAASIGN_VER2       10
#define MBVAASIGN_CMPX1    6
#define MBVAASIGN_CMPX2    9

extern const int32_t g_kiQpCostTable[52];
extern const int8_t g_kiMapModeI16x16[7];
//extern const int8_t g_kiMapModeI4x4[14];
extern const int8_t g_kiMapModeIntraChroma[7];

/////////////////////////////

// if we want keep total sizeof(SWelsMD) <= 256, we maybe need to seperate three member of SWelsME.
typedef struct TagWelsMD {
int32_t         iLambda;
uint16_t*       pMvdCost;

int32_t         iCostLuma;
int32_t         iCostChroma;//satd+lambda(best_pred_mode) //i_sad_chroma;
int32_t         iSadPredMb;

uint8_t         uiRef; //uiRefIndex appointed by Encoder, used for MC
bool            bMdUsingSad;
uint16_t        uiReserved;

int32_t         iCostSkipMb;
int32_t         iSadPredSkip;

int32_t         iMbPixX;                // pixel position of MB in horizontal axis
int32_t         iMbPixY;                // pixel position of MB in vertical axis
int32_t         iBlock8x8StaticIdc[4];

//NO B frame in our Wels, we can ignore list1

struct {
  SWelsME       sMe16x16;               //adjust each SWelsME for 8 D-word!
  SWelsME       sMe8x8[4];
  SWelsME       sMe16x8[2];
  SWelsME       sMe8x16[2];
  SWelsME       sMe4x4[4][4];
  SWelsME       sMe8x4[4][2];
  SWelsME       sMe4x8[4][2];
//  SMVUnitXY     i_mvbs[MB_BLOCK8x8_NUM];        //scaled MVB
} sMe;

} SWelsMD;

typedef struct TagMeRefinePointer {
uint8_t* pHalfPixH;
uint8_t* pHalfPixV;
uint8_t* pHalfPixHV;

uint8_t* pQuarPixBest;
uint8_t* pQuarPixTmp;

PCopyFunc pfCopyBlockByMode;
} SMeRefinePointer;

void FillNeighborCacheIntra (SMbCache* pMbCache, SMB* pCurMb, int32_t iMbWidth/*, bool constrained_intra_pred_flag*/);
void FillNeighborCacheInterWithoutBGD (SMbCache* pMbCache, SMB* pCurMb, int32_t iMbWidth,
                                       int8_t* pVaaBgMbFlag); //BGD spatial func
void FillNeighborCacheInterWithBGD (SMbCache* pMbCache, SMB* pCurMb, int32_t iMbWidth, int8_t* pVaaBgMbFlag);
void InitFillNeighborCacheInterFunc (SWelsFuncPtrList* pFuncList, const int32_t kiFlag);

void MvdCostInit (uint16_t* pMvdCostInter, const int32_t kiMvdSz);

void PredictSad (int8_t* pRefIndexCache, int32_t* pSadCostCache, int32_t uiRef, int32_t* pSadPred);


void PredictSadSkip (int8_t* pRefIndexCache, bool* pMbSkipCache, int32_t* pSadCostCache, int32_t uiRef,
                     int32_t* iSadPredSkip);

//  for pfGetVarianceFromIntraVaa function ptr adaptive by CPU features, 6/7/2010
void InitIntraAnalysisVaaInfo (SWelsFuncPtrList* pFuncList, const uint32_t kuiCpuFlag);
bool MdIntraAnalysisVaaInfo (sWelsEncCtx* pEncCtx, uint8_t* pEncMb);

uint8_t MdInterAnalysisVaaInfo_c (int32_t* pSad8x8);


void InitMeRefinePointer (SMeRefinePointer* pMeRefine, SMbCache* pMbCache, int32_t iStride);
void MeRefineFracPixel (sWelsEncCtx* pEncCtx, uint8_t* pMemPredInterMb, SWelsME* pMe,
                        SMeRefinePointer* pMeRefine, int32_t iWidth, int32_t iHeight);

void InitBlkStrideWithRef (int32_t* pBlkStride, const int32_t kiStrideRef);

void UpdateMbMv_c (SMVUnitXY* pMvBuffer, const SMVUnitXY ksMv);

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#if defined(X86_ASM)

//  for pfGetVarianceFromIntraVaa SIMD optimization, 6/7/2010
int32_t AnalysisVaaInfoIntra_sse2 (uint8_t* pDataY, const int32_t kiLineSize);
int32_t AnalysisVaaInfoIntra_ssse3 (uint8_t* pDataY, const int32_t kiLineSize);
uint8_t MdInterAnalysisVaaInfo_sse2 (int32_t* pSad8x8);
uint8_t MdInterAnalysisVaaInfo_sse41 (int32_t* pSad8x8);
void UpdateMbMv_sse2 (SMVUnitXY* pMvBuffer, const SMVUnitXY ksMv);

#endif//X86_ASM

#if defined(__cplusplus)
}
#endif//__cplusplus

}
#endif//WELS_MACROBLOCK_MODE_DECISION_H__

