/*!
 * \copy
 *     Copyright (c)  2011-2013, Cisco Systems
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
 * \file    wels_preprocess.h
 *
 * \brief   interface of video pre-process plugins
 *
 * \date    03/15/2011
 *
 * \description : this class is designed as an interface to unify video pre-processing
 *                class implement sets such as denoise,colorspace conversion etc...
 *
 *************************************************************************************
 */

#ifndef WELS_PREPROCESS_H
#define WELS_PREPROCESS_H

#include "typedefs.h"
#include "picture.h"
#include "wels_const.h"
#include "IWelsVP.h"
#include "param_svc.h"

namespace WelsEnc {

typedef struct TagWelsEncCtx sWelsEncCtx;

typedef  struct {
  SPicture*     pScaledInputPicture;
  int32_t       iScaledWidth[MAX_DEPENDENCY_LAYER];
  int32_t       iScaledHeight[MAX_DEPENDENCY_LAYER];
} Scaled_Picture;


typedef struct {
  int64_t iMinFrameComplexity;
  int64_t iMinFrameComplexity08;
  int64_t iMinFrameComplexity11;

  int32_t iMinFrameNumGap;
  int32_t iMinFrameQp;
} SRefJudgement;

typedef struct {
  SPicture*   pRefPicture;
  int32_t     iSrcListIdx;   //idx in  h->spatial_pic[base_did];
  bool        bSceneLtrFlag;
  unsigned char*        pBestBlockStaticIdc;
} SRefInfoParam;

typedef struct TagVAAFrameInfo {
  SVAACalcResult        sVaaCalcInfo;
  SAdaptiveQuantizationParam sAdaptiveQuantParam;
  SComplexityAnalysisParam sComplexityAnalysisParam;

  int32_t       iPicWidth;          // maximal iWidth of picture in samples for svc coding
  int32_t       iPicHeight;         // maximal iHeight of picture in samples for svc coding
  int32_t       iPicStride;         //luma
  int32_t       iPicStrideUV;

  uint8_t*      pRefY; //pRef
  uint8_t*      pCurY; //cur
  uint8_t*      pRefU; //pRef
  uint8_t*      pCurU; //cur
  uint8_t*      pRefV; //pRef
  uint8_t*      pCurV; //cur

  int8_t*       pVaaBackgroundMbFlag;
  uint8_t       uiValidLongTermPicIdx;
  uint8_t       uiMarkLongTermPicIdx;

  ESceneChangeIdc eSceneChangeIdc;
  bool          bSceneChangeFlag;
  bool          bIdrPeriodFlag;
} SVAAFrameInfo;

typedef struct SVAAFrameInfoExt_t: public SVAAFrameInfo {
  SComplexityAnalysisScreenParam    sComplexityScreenParam;
  SScrollDetectionParam    sScrollDetectInfo;
  SRefInfoParam    sVaaStrBestRefCandidate[MAX_REF_PIC_COUNT];
  SRefInfoParam    sVaaLtrBestRefCandidate[MAX_REF_PIC_COUNT];
  int32_t    iNumOfAvailableRef;

  int32_t     iVaaBestRefFrameNum;
  uint8_t*    pVaaBestBlockStaticIdc;//pointer
  uint8_t*    pVaaBlockStaticIdc[16];//real memory,
} SVAAFrameInfoExt;

class CWelsPreProcess {
 public:
  CWelsPreProcess (sWelsEncCtx* pEncCtx);
  virtual  ~CWelsPreProcess();

  static CWelsPreProcess* CreatePreProcess (sWelsEncCtx* pEncCtx);

  virtual SPicture* GetCurrentOrigFrame (int32_t iDIdx) = 0;
 public:
  int32_t WelsPreprocessReset (sWelsEncCtx* pEncCtx, int32_t iWidth, int32_t iHeight);
  int32_t AllocSpatialPictures (sWelsEncCtx* pCtx, SWelsSvcCodingParam* pParam);
  void    FreeSpatialPictures (sWelsEncCtx* pCtx);
  int32_t BuildSpatialPicList (sWelsEncCtx* pEncCtx, const SSourcePicture* kpSrcPic);
  int32_t AnalyzeSpatialPic (sWelsEncCtx* pEncCtx, const int32_t kiDIdx);
  int32_t UpdateSpatialPictures (sWelsEncCtx* pEncCtx, SWelsSvcCodingParam* pParam, const int8_t iCurTid,
                                 const int32_t d_idx);
  int32_t GetRefFrameInfo (int32_t iRefIdx, bool bCurrentFrameIsSceneLtr, SPicture*& pRefOri);
  void    AnalyzePictureComplexity (sWelsEncCtx* pCtx, SPicture* pCurPicture, SPicture* pRefPicture,
                                    const int32_t kiDependencyId, const bool kbCalculateBGD);
  int32_t UpdateBlockIdcForScreen (uint8_t*  pCurBlockStaticPointer, const SPicture* kpRefPic, const SPicture* kpSrcPic);


  void UpdateSrcList (SPicture* pCurPicture, const int32_t kiCurDid, SPicture** pShortRefList,
                      const uint32_t kuiShortRefCount);
  void UpdateSrcListLosslessScreenRefSelectionWithLtr (SPicture* pCurPicture, const int32_t kiCurDid,
      const int32_t kuiMarkLongTermPicIdx, SPicture** pLongRefList);


 protected:
  bool GetSceneChangeFlag (ESceneChangeIdc eSceneChangeIdc);
  virtual  ESceneChangeIdc  DetectSceneChange (SPicture* pCurPicture, SPicture* pRefPicture = NULL) = 0;

  void InitPixMap (const SPicture* pPicture, SPixMap* pPixMap);

  int32_t GetCurPicPosition (const int32_t kiDidx);

 private:
  int32_t WelsPreprocessCreate();
  int32_t WelsPreprocessDestroy();
  int32_t InitLastSpatialPictures (sWelsEncCtx* pEncCtx);

 private:
  int32_t SingleLayerPreprocess (sWelsEncCtx* pEncCtx, const SSourcePicture* kpSrc, Scaled_Picture* m_sScaledPicture);

  void  BilateralDenoising (SPicture* pSrc, const int32_t iWidth, const int32_t iHeight);

  int32_t DownsamplePadding (SPicture* pSrc, SPicture* pDstPic,  int32_t iSrcWidth, int32_t iSrcHeight,
                             int32_t iShrinkWidth, int32_t iShrinkHeight, int32_t iTargetWidth, int32_t iTargetHeight,
                             bool bForceCopy);

  void    VaaCalculation (SVAAFrameInfo* pVaaInfo, SPicture* pCurPicture, SPicture* pRefPicture, bool bCalculateSQDiff,
                          bool bCalculateVar, bool bCalculateBGD);
  void    BackgroundDetection (SVAAFrameInfo* pVaaInfo, SPicture* pCurPicture, SPicture* pRefPicture, bool bDetectFlag);
  void    AdaptiveQuantCalculation (SVAAFrameInfo* pVaaInfo, SPicture* pCurPicture, SPicture* pRefPicture);
  void    Padding (uint8_t* pSrcY, uint8_t* pSrcU, uint8_t* pSrcV, int32_t iStrideY, int32_t iStrideUV,
                   int32_t iActualWidth, int32_t iPaddingWidth, int32_t iActualHeight, int32_t iPaddingHeight);
  void    SetRefMbType (sWelsEncCtx* pCtx, uint32_t** pRefMbTypeArray, int32_t iRefPicType);

  int32_t ColorspaceConvert (SWelsSvcCodingParam* pSvcParam, SPicture* pDstPic, const SSourcePicture* kpSrc,
                             const int32_t kiWidth, const int32_t kiHeight);
  void WelsMoveMemoryWrapper (SWelsSvcCodingParam* pSvcParam, SPicture* pDstPic, const SSourcePicture* kpSrc,
                              const int32_t kiWidth, const int32_t kiHeight);

  /*!
  * \brief  exchange two picture pData planes
  * \param  ppPic1      picture pointer to picture 1
  * \param  ppPic2      picture pointer to picture 2
  * \return none
  */
  void WelsExchangeSpatialPictures (SPicture** ppPic1, SPicture** ppPic2);

  SPicture* GetBestRefPic (EUsageType iUsageType, bool bSceneLtr, EWelsSliceType eSliceType, int32_t kiDidx,
                           int32_t iRefTemporalIdx);
  SPicture* GetBestRefPic (const int32_t kiDidx, const int32_t iRefTemporalIdx);
 protected:
  IWelsVP*         m_pInterfaceVp;
  sWelsEncCtx*     m_pEncCtx;
  uint8_t          m_uiSpatialLayersInTemporal[MAX_DEPENDENCY_LAYER];

 private:
  Scaled_Picture   m_sScaledPicture;
  SPicture*        m_pLastSpatialPicture[MAX_DEPENDENCY_LAYER][2];
  bool             m_bInitDone;
  uint8_t          m_uiSpatialPicNum[MAX_DEPENDENCY_LAYER];
 protected:
  /* For Downsampling & VAA I420 based source pictures */
  SPicture*        m_pSpatialPic[MAX_DEPENDENCY_LAYER][MAX_REF_PIC_COUNT + 1];
  // need memory requirement with total number of num_of_ref + 1, "+1" is for current frame
  int32_t           m_iAvaliableRefInSpatialPicList;

};

class CWelsPreProcessVideo : public CWelsPreProcess {
 public:
  CWelsPreProcessVideo (sWelsEncCtx* pEncCtx) : CWelsPreProcess (pEncCtx) {};

  virtual SPicture* GetCurrentOrigFrame (int32_t iDIdx);

  virtual ESceneChangeIdc  DetectSceneChange (SPicture* pCurPicture, SPicture* pRefPicture = NULL);
};



class CWelsPreProcessScreen : public CWelsPreProcess {
 public:
  CWelsPreProcessScreen (sWelsEncCtx* pEncCtx) : CWelsPreProcess (pEncCtx) {};

  virtual SPicture* GetCurrentOrigFrame (int32_t iDIdx);

  virtual ESceneChangeIdc  DetectSceneChange (SPicture* pCurPicture, SPicture* pRefPicture = NULL);

 private:
  SPicture** GetReferenceSrcPicList(int32_t iTargetDid);

  void GetAvailableRefListLosslessScreenRefSelection (SPicture** pSrcPicList, uint8_t iCurTid,
      const int32_t iClosestLtrFrameNum,
      SRefInfoParam* pAvailableRefList, int32_t& iAvailableRefNum, int32_t& iAvailableSceneRefNum);

  void GetAvailableRefList (SPicture** pSrcPicList, uint8_t iCurTid, const int32_t iClosestLtrFrameNum,
                            SRefInfoParam* pAvailableRefList, int32_t& iAvailableRefNum, int32_t& iAvailableSceneRefNum);
  void InitRefJudgement (SRefJudgement* pRefJudgement);

  bool JudgeBestRef (SPicture* pRefPic, const SRefJudgement& sRefJudgement, const int64_t iFrameComplexity,
                     const bool bIsClosestLtrFrame);
  void SaveBestRefToJudgement (const int32_t iRefPictureAvQP, const int64_t iComplexity, SRefJudgement* pRefJudgement);
  void SaveBestRefToLocal (SRefInfoParam* pRefPicInfo, const SSceneChangeResult& sSceneChangeResult,
                           SRefInfoParam* pRefSaved);
  void SaveBestRefToVaa (SRefInfoParam& sRefSaved, SRefInfoParam* pVaaBestRef);
};


}

#endif
