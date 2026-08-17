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
 * \file  svc motion estimate.h
 *
 * \brief  Interfaces introduced in svc mb motion estimation
 *
 * \date  08/11/2009 Created
 *
 *************************************************************************************
 */
#ifndef SVC_MOTION_ESTIMATE_
#define SVC_MOTION_ESTIMATE_

#include "typedefs.h"
#include "encoder_context.h"
#include "wels_func_ptr_def.h"

namespace WelsEnc {
#define CAMERA_STARTMV_RANGE (64)
#define  ITERATIVE_TIMES  (16)
#define CAMERA_MV_RANGE (CAMERA_STARTMV_RANGE+ITERATIVE_TIMES)
#define CAMERA_MVD_RANGE  ((CAMERA_MV_RANGE+1)<<1) //mvd=mv_range*2;
#define  BASE_MV_MB_NMB  ((2*CAMERA_MV_RANGE/MB_WIDTH_LUMA)-1)
#define CAMERA_HIGHLAYER_MVD_RANGE (243)//mvd range;
#define EXPANDED_MV_RANGE (504) //=512-8 rather than 511 to sacrifice same edge point but save complexity in assemblys
#define EXPANDED_MVD_RANGE ((504+1)<<1)

enum {
ME_DIA    = 0x01,  // LITTLE DIAMOND= 0x01
ME_CROSS  = 0x02,  // CROSS=  0x02
ME_FME    = 0x04,  // FME = 0x04
ME_FULL    = 0x10,  // FULL

// derived ME methods combination
ME_DIA_CROSS    = (ME_DIA | ME_CROSS),   // DIA+CROSS
ME_DIA_CROSS_FME  = (ME_DIA_CROSS | ME_FME)  // DIA+CROSS+FME
};

union SadPredISatdUnit {
uint32_t  uiSadPred;
uint32_t  uiSatd;    //reuse the sad_pred as a temp satd pData
};
typedef struct TagWelsME {
/* input */
uint16_t*          pMvdCost;
union SadPredISatdUnit  uSadPredISatd; //reuse the sad_pred as a temp pData
uint32_t
uiSadCost;  //used by ME and RC //max SAD should be max_delta*size+lambda*mvdsize = 255*256+91*33*2 = 65280 + 6006 = 71286 > (2^16)-1 = 65535
uint32_t          uiSatdCost; /* satd + lm * nbits */
uint32_t          uiSadCostThreshold;
int32_t            iCurMeBlockPixX;
int32_t            iCurMeBlockPixY;
uint8_t            uiBlockSize;   /* BLOCK_WxH */
uint8_t            uiReserved;

uint8_t*            pEncMb;
uint8_t*            pRefMb;
uint8_t*            pColoRefMb;

SMVUnitXY          sMvp;
SMVUnitXY          sMvBase;
SMVUnitXY          sDirectionalMv;

SScreenBlockFeatureStorage* pRefFeatureStorage;

/* output */
SMVUnitXY          sMv;
} SWelsME;

typedef struct TagFeatureSearchIn {
PSampleSadSatdCostFunc pSad;

uint32_t* pTimesOfFeature;
uint16_t** pQpelLocationOfFeature;
uint16_t* pMvdCostX;
uint16_t* pMvdCostY;

uint8_t* pEnc;
uint8_t* pColoRef;
int32_t iEncStride;
int32_t iRefStride;
uint16_t uiSadCostThresh;

int32_t iFeatureOfCurrent;

int32_t iCurPixX;
int32_t iCurPixY;
int32_t iCurPixXQpel;
int32_t iCurPixYQpel;

int32_t iMinQpelX;
int32_t iMinQpelY;
int32_t iMaxQpelX;
int32_t iMaxQpelY;
} SFeatureSearchIn;

typedef struct TagFeatureSearchOut {
SMVUnitXY sBestMv;
uint32_t uiBestSadCost;
uint8_t* pBestRef;
} SFeatureSearchOut;

#define  COST_MVD(table, mx, my)  (table[mx] + table[my])
extern const int32_t QStepx16ByQp[52];

// Function definitions below

void WelsInitMeFunc (SWelsFuncPtrList* pFuncList, uint32_t uiCpuFlag, bool bScreenContent);

/*!
 * \brief  BL mb motion estimate search
 *
 * \param  enc      Wels encoder context
 * \param  m          Wels me information
 *
 * \return  NONE
 */
void WelsMotionEstimateSearch (SWelsFuncPtrList* pFuncList, SDqLayer* pLplayer, SWelsME* pLpme, SSlice* pLpslice);
void WelsMotionEstimateSearchStatic (SWelsFuncPtrList* pFuncList, SDqLayer* pLplayer, SWelsME* pLpme, SSlice* pLpslice);
void WelsMotionEstimateSearchScrolled (SWelsFuncPtrList* pFuncList, SDqLayer* pLplayer, SWelsME* pLpme, SSlice* pLpslice);
/*!
 * \brief  BL mb motion estimate initial point testing
 *
 * \param  enc      Wels encoder context
 * \param  m          Wels me information
 * \param  mv_range  search range in motion estimate
 * \param  point      the best match point in motion estimation
 *
 * \return  NONE
 */


/*!
 * \brief  EL mb motion estimate initial point testing
 *
 * \param  pix_func  SSampleDealingFunc
 * \param  m          Wels me information
 * \param  mv_range  search range in motion estimate
 * \param  point      the best match point in motion estimation
 *
 * \return  NONE
 */

bool WelsMotionEstimateInitialPoint (SWelsFuncPtrList* pFuncList, SWelsME* pMe, SSlice* pSlice,
                                     const int32_t kiStrideEnc, const int32_t kiStrideRef);

/*!
 * \brief  mb iterative motion estimate search
 *
 * \param  enc      Wels encoder context
 * \param  m          Wels me information
 * \param  point      the best match point in motion estimation
 *
 * \return  NONE
 */
void WelsDiamondSearch (SWelsFuncPtrList* pFuncList, SWelsME* pMe, SSlice* pSlice, const int32_t kiEncStride,
                        const int32_t kiRefStride);

bool WelsMeSadCostSelect (int32_t* pSadCost, const uint16_t* kpMvdCost, int32_t* pBestCost, const int32_t kiDx,
                          const int32_t kiDy, int32_t* pIx, int32_t* pIy);

void CalculateSatdCost (PSampleSadSatdCostFunc pSatd, SWelsME* pMe, const int32_t kiEncStride, const int32_t kiRefStride);
void NotCalculateSatdCost (PSampleSadSatdCostFunc pSatd, SWelsME* pMe, const int32_t kiEncStride,
                           const int32_t kiRefStride);
bool CheckDirectionalMv (PSampleSadSatdCostFunc pSad, SWelsME* pMe,
                         const SMVUnitXY ksMinMv, const SMVUnitXY ksMaxMv, const int32_t kiEncStride, const int32_t kiRefStride,
                         int32_t& iBestSadCost);
bool CheckDirectionalMvFalse (PSampleSadSatdCostFunc pSad, SWelsME* pMe,
                              const SMVUnitXY ksMinMv, const SMVUnitXY ksMaxMv, const int32_t kiEncStride, const int32_t kiRefStride,
                              int32_t& iBestSadCost);

// Cross Search Basics
void LineFullSearch_c (SWelsFuncPtrList* pFuncList, SWelsME* pMe,
                       uint16_t* pMvdTable,
                       const int32_t kiEncStride, const int32_t kiRefStride,
                       const int16_t kiMinMv, const int16_t kiMaxMv,
                       const bool bVerticalSearch);
#ifdef X86_ASM
extern "C"
{
uint32_t SampleSad8x8Hor8_sse41 (uint8_t*, int32_t, uint8_t*, int32_t, uint16_t*, int32_t*);
uint32_t SampleSad16x16Hor8_sse41 (uint8_t*, int32_t, uint8_t*, int32_t, uint16_t*, int32_t*);
}

void VerticalFullSearchUsingSSE41 (SWelsFuncPtrList* pFuncList, SWelsME* pMe,
                                   uint16_t* pMvdTable,
                                   const int32_t kiEncStride, const int32_t kiRefStride,
                                   const int16_t kiMinMv, const int16_t kiMaxMv,
                                   const bool bVerticalSearch);
void HorizontalFullSearchUsingSSE41 (SWelsFuncPtrList* pFuncList, SWelsME* pMe,
                                     uint16_t* pMvdTable,
                                     const int32_t kiEncStride, const int32_t kiRefStride,
                                     const int16_t kiMinMv, const int16_t kiMaxMv,
                                     const bool bVerticalSearch);
#endif
void WelsMotionCrossSearch (SWelsFuncPtrList* pFuncList, SWelsME* pMe, SSlice* pSlice,
                            const int32_t kiEncStride, const int32_t kiRefStride);
void WelsDiamondCrossSearch (SWelsFuncPtrList* pFuncList, SWelsME* pMe, SSlice* pSlice,
                             const int32_t kiEncStride, const int32_t kiRefStride);

// Feature Search Basics
#define LIST_SIZE_SUM_16x16 0x0FF01  //(256*255+1)
#define LIST_SIZE_SUM_8x8     0x03FC1  //(64*255+1)
#define LIST_SIZE_MSE_16x16 0x00878  //(avg+mse)/2, max= (255+16*255)/2

#define FME_DEFAULT_FEATURE_INDEX (0)
#define FMESWITCH_DEFAULT_GOODFRAME_NUM (2)
#define FMESWITCH_MBSAD_THRESHOLD   30 // empirically set.

void InitializeHashforFeature_c (uint32_t* pTimesOfFeatureValue, uint16_t* pBuf, const int32_t kiListSize,
                                 uint16_t** pLocationOfFeature, uint16_t** pFeatureValuePointerList);
void FillQpelLocationByFeatureValue_c (uint16_t* pFeatureOfBlock, const int32_t kiWidth, const int32_t kiHeight,
                                       uint16_t** pFeatureValuePointerList);
int32_t SumOf8x8SingleBlock_c (uint8_t* pRef, const int32_t kiRefStride);
int32_t SumOf16x16SingleBlock_c (uint8_t* pRef, const int32_t kiRefStride);
void SumOf8x8BlockOfFrame_c (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                             const int32_t kiRefStride,
                             uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
void SumOf16x16BlockOfFrame_c (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                               const int32_t kiRefStride,
                               uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);

#ifdef X86_ASM
extern "C"
{
void InitializeHashforFeature_sse2 (uint32_t* pTimesOfFeatureValue, uint16_t* pBuf, const int32_t kiListSize,
                                     uint16_t** pLocationOfFeature, uint16_t** pFeatureValuePointerList);
void FillQpelLocationByFeatureValue_sse2 (uint16_t* pFeatureOfBlock, const int32_t kiWidth, const int32_t kiHeight,
                                           uint16_t** pFeatureValuePointerList);
int32_t SumOf8x8SingleBlock_sse2 (uint8_t* pRef, const int32_t kiRefStride);
int32_t SumOf16x16SingleBlock_sse2 (uint8_t* pRef, const int32_t kiRefStride);
void SumOf8x8BlockOfFrame_sse2 (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                const int32_t kiRefStride, uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
void SumOf16x16BlockOfFrame_sse2 (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                const int32_t kiRefStride, uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
void SumOf8x8BlockOfFrame_sse4 (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                const int32_t kiRefStride, uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
void SumOf16x16BlockOfFrame_sse4 (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                const int32_t kiRefStride, uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
}
#endif
#ifdef HAVE_NEON
extern "C"
{
void InitializeHashforFeature_neon (uint32_t* pTimesOfFeatureValue, uint16_t* pBuf, const int32_t kiListSize,
                                    uint16_t** pLocationOfFeature, uint16_t** pFeatureValuePointerList);
void FillQpelLocationByFeatureValue_neon (uint16_t* pFeatureOfBlock, const int32_t kiWidth, const int32_t kiHeight,
                                          uint16_t** pFeatureValuePointerList);
int32_t SumOf8x8SingleBlock_neon (uint8_t* pRef, const int32_t kiRefStride);
int32_t SumOf16x16SingleBlock_neon (uint8_t* pRef, const int32_t kiRefStride);
void SumOf8x8BlockOfFrame_neon (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                                const int32_t kiRefStride,
                                uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
void SumOf16x16BlockOfFrame_neon (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                                  const int32_t kiRefStride,
                                  uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
}
#endif

#ifdef HAVE_NEON_AARCH64
extern "C"
{
void InitializeHashforFeature_AArch64_neon (uint32_t* pTimesOfFeatureValue, uint16_t* pBuf, const int32_t kiListSize,
                                    uint16_t** pLocationOfFeature, uint16_t** pFeatureValuePointerList);
void FillQpelLocationByFeatureValue_AArch64_neon (uint16_t* pFeatureOfBlock, const int32_t kiWidth, const int32_t kiHeight,
                                          uint16_t** pFeatureValuePointerList);
int32_t SumOf8x8SingleBlock_AArch64_neon (uint8_t* pRef, const int32_t kiRefStride);
int32_t SumOf16x16SingleBlock_AArch64_neon (uint8_t* pRef, const int32_t kiRefStride);
void SumOf8x8BlockOfFrame_AArch64_neon (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                                const int32_t kiRefStride,
                                uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
void SumOf16x16BlockOfFrame_AArch64_neon (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                                  const int32_t kiRefStride,
                                  uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
}
#endif

#ifdef HAVE_LSX
extern "C"
{
int32_t SumOf8x8SingleBlock_lsx (uint8_t* pRef, const int32_t kiRefStride);
void SumOf8x8BlockOfFrame_lsx (uint8_t* pRefPicture, const int32_t kiWidth, const int32_t kiHeight,
                const int32_t kiRefStride, uint16_t* pFeatureOfBlock, uint32_t pTimesOfFeatureValue[]);
}
#endif

int32_t RequestScreenBlockFeatureStorage (CMemoryAlign* pMa, const int32_t kiFrameWidth,  const int32_t kiFrameHeight,
    const int32_t iNeedFeatureStorage,
    SScreenBlockFeatureStorage* pScreenBlockFeatureStorage);
int32_t ReleaseScreenBlockFeatureStorage (CMemoryAlign* pMa, SScreenBlockFeatureStorage* pScreenBlockFeatureStorage);
int32_t RequestFeatureSearchPreparation (CMemoryAlign* pMa, const int32_t kiFrameWidth,  const int32_t kiFrameHeight,
    const int32_t iNeedFeatureStorage,
    SFeatureSearchPreparation* pFeatureSearchPreparation);
int32_t ReleaseFeatureSearchPreparation (CMemoryAlign* pMa, uint16_t*& pFeatureOfBlock);

#define FMESWITCH_DEFAULT_GOODFRAME_NUM (2)
#define FME_DEFAULT_FEATURE_INDEX (0)


void PerformFMEPreprocess (SWelsFuncPtrList* pFunc, SPicture* pRef, uint16_t* pFeatureOfBlock,
                           SScreenBlockFeatureStorage* pScreenBlockFeatureStorage);
bool SetFeatureSearchIn (SWelsFuncPtrList* pFunc,  const SWelsME& sMe,
                         const SSlice* pSlice, SScreenBlockFeatureStorage* pRefFeatureStorage,
                         const int32_t kiEncStride, const int32_t kiRefStride,
                         SFeatureSearchIn* pFeatureSearchIn);
void MotionEstimateFeatureFullSearch (SFeatureSearchIn& sFeatureSearchIn,
                                      const uint32_t kuiMaxSearchPoint,
                                      SWelsME* pMe);
void UpdateFMESwitch (SDqLayer* pCurLayer);
void UpdateFMESwitchNull (SDqLayer* pCurLayer);

void WelsDiamondCrossFeatureSearch (SWelsFuncPtrList* pFuncList, SWelsME* pMe, SSlice* pSlice,
                                    const int32_t kiEncStride, const int32_t kiRefStride);

//inline functions
inline void SetMvWithinIntegerMvRange (const int32_t kiMbWidth, const int32_t kiMbHeight, const int32_t kiMbX,
                                       const int32_t kiMbY,
                                       const int32_t kiMaxMvRange,
                                       SMVUnitXY* pMvMin, SMVUnitXY* pMvMax) {
pMvMin->iMvX = WELS_MAX (-1 * ((kiMbX + 1) * (1 << 4)) + INTPEL_NEEDED_MARGIN, -1 * kiMaxMvRange);
pMvMin->iMvY = WELS_MAX (-1 * ((kiMbY + 1) * (1 << 4)) + INTPEL_NEEDED_MARGIN, -1 * kiMaxMvRange);
pMvMax->iMvX = WELS_MIN (((kiMbWidth - kiMbX) * (1 << 4)) - INTPEL_NEEDED_MARGIN, kiMaxMvRange);
pMvMax->iMvY = WELS_MIN (((kiMbHeight - kiMbY) * (1 << 4)) - INTPEL_NEEDED_MARGIN, kiMaxMvRange);
}

inline bool CheckMvInRange (const SMVUnitXY ksCurrentMv, const SMVUnitXY ksMinMv, const SMVUnitXY ksMaxMv) {
return (CheckInRangeCloseOpen (ksCurrentMv.iMvX, ksMinMv.iMvX, ksMaxMv.iMvX)
        && CheckInRangeCloseOpen (ksCurrentMv.iMvY, ksMinMv.iMvY, ksMaxMv.iMvY));
}
//FME switch related
inline bool CalcFMESwitchFlag (const uint8_t uiFMEGoodFrameCount, const int32_t iHighFreMbPrecentage,
                               const int32_t iAvgMbSAD, const bool bScrollingDetected) {
return (bScrollingDetected || (uiFMEGoodFrameCount > 0 && iAvgMbSAD > FMESWITCH_MBSAD_THRESHOLD));
//TODO: add the logic of iHighFreMbPrecentage
//return ( iHighFreMbPrecentage > 2
//            && ( bScrollingDetected || iHighFreMbPrecentage >15
//            ||( uiFMEGoodFrameCount>0 && iFrameSAD > FMESWITCH_FRAMESAD_THRESHOLD ) ) );
}
}
#endif
