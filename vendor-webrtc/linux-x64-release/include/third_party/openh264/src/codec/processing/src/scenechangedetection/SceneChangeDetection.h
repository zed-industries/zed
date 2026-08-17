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
 * \file         :  SceneChangeDetection.h
 *
 * \brief        :  scene change detection class of wels video processor class
 *
 * \date         :  2011/03/14
 *
 * \description  :  1. rewrite the package code of scene change detection class
 *
 *************************************************************************************
 */

#ifndef WELSVP_SCENECHANGEDETECTION_H
#define WELSVP_SCENECHANGEDETECTION_H

#include "util.h"
#include "memory.h"
#include "cpu.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"
#include "common.h"

#define HIGH_MOTION_BLOCK_THRESHOLD 320
#define SCENE_CHANGE_MOTION_RATIO_LARGE_VIDEO   0.85f
#define SCENE_CHANGE_MOTION_RATIO_MEDIUM  0.50f
#define SCENE_CHANGE_MOTION_RATIO_LARGE_SCREEN    0.80f

WELSVP_NAMESPACE_BEGIN

typedef struct {
  int32_t iWidth;
  int32_t iHeight;
  int32_t iBlock8x8Width;
  int32_t iBlock8x8Height;
  uint8_t* pRefY;
  uint8_t* pCurY;
  int32_t iRefStride;
  int32_t iCurStride;
  uint8_t* pStaticBlockIdc;
} SLocalParam;

class CSceneChangeDetectorVideo {
 public:
  CSceneChangeDetectorVideo (SSceneChangeResult& sParam, int32_t iCpuFlag) : m_sParam (sParam) {
    m_pfSad = WelsSampleSad8x8_c;
#ifdef X86_ASM
    if (iCpuFlag & WELS_CPU_SSE2) {
      m_pfSad = WelsSampleSad8x8_sse21;
    }
#endif
#ifdef HAVE_NEON
    if (iCpuFlag & WELS_CPU_NEON) {
      m_pfSad = WelsProcessingSampleSad8x8_neon;
    }
#endif

#ifdef HAVE_NEON_AARCH64
    if (iCpuFlag & WELS_CPU_NEON) {
      m_pfSad = WelsProcessingSampleSad8x8_AArch64_neon;
    }
#endif

#ifdef HAVE_MMI
    if (iCpuFlag & WELS_CPU_MMI) {
      m_pfSad = WelsSampleSad8x8_mmi;
    }
#endif

#ifdef HAVE_LASX
    if (iCpuFlag & WELS_CPU_LASX) {
      m_pfSad = WelsSampleSad8x8_lasx;
    }
#endif

    m_fSceneChangeMotionRatioLarge = SCENE_CHANGE_MOTION_RATIO_LARGE_VIDEO;
    m_fSceneChangeMotionRatioMedium = SCENE_CHANGE_MOTION_RATIO_MEDIUM;
  }
  virtual ~CSceneChangeDetectorVideo() {
  }
  void operator() (SLocalParam& sLocalParam) {
    int32_t iRefRowStride = 0, iCurRowStride = 0;
    uint8_t* pRefY = sLocalParam.pRefY;
    uint8_t* pCurY = sLocalParam.pCurY;
    uint8_t* pRefTmp = NULL, *pCurTmp = NULL;

    iRefRowStride  = sLocalParam.iRefStride << 3;
    iCurRowStride  = sLocalParam.iCurStride << 3;

    for (int32_t j = 0; j < sLocalParam.iBlock8x8Height; j++) {
      pRefTmp = pRefY;
      pCurTmp = pCurY;
      for (int32_t i = 0; i < sLocalParam.iBlock8x8Width; i++) {
        int32_t iSad = m_pfSad (pCurTmp, sLocalParam.iCurStride, pRefTmp, sLocalParam.iRefStride);
        m_sParam.iMotionBlockNum += iSad > HIGH_MOTION_BLOCK_THRESHOLD;
        pRefTmp += 8;
        pCurTmp += 8;
      }
      pRefY += iRefRowStride;
      pCurY += iCurRowStride;
    }
  }
  float  GetSceneChangeMotionRatioLarge() const {
    return m_fSceneChangeMotionRatioLarge;
  }
  float  GetSceneChangeMotionRatioMedium() const {
    return m_fSceneChangeMotionRatioMedium;
  }
 protected:
  SadFuncPtr m_pfSad;
  SSceneChangeResult& m_sParam;
  float    m_fSceneChangeMotionRatioLarge;
  float    m_fSceneChangeMotionRatioMedium;
};

class CSceneChangeDetectorScreen : public CSceneChangeDetectorVideo {
 public:
  CSceneChangeDetectorScreen (SSceneChangeResult& sParam, int32_t iCpuFlag) : CSceneChangeDetectorVideo (sParam,
        iCpuFlag) {
    m_fSceneChangeMotionRatioLarge = SCENE_CHANGE_MOTION_RATIO_LARGE_SCREEN;
    m_fSceneChangeMotionRatioMedium = SCENE_CHANGE_MOTION_RATIO_MEDIUM;
  }
  virtual ~CSceneChangeDetectorScreen() {
  }
  void operator() (SLocalParam& sLocalParam) {
    bool bScrollDetectFlag = m_sParam.sScrollResult.bScrollDetectFlag;
    int32_t iScrollMvX = m_sParam.sScrollResult.iScrollMvX;
    int32_t iScrollMvY = m_sParam.sScrollResult.iScrollMvY;

    int32_t iRefRowStride = 0, iCurRowStride = 0;
    uint8_t* pRefY = sLocalParam.pRefY;
    uint8_t* pCurY = sLocalParam.pCurY;
    uint8_t* pRefTmp = NULL, *pCurTmp = NULL;
    int32_t iWidth = sLocalParam.iWidth;
    int32_t iHeight = sLocalParam.iHeight;

    iRefRowStride  = sLocalParam.iRefStride << 3;
    iCurRowStride  = sLocalParam.iCurStride << 3;

    for (int32_t j = 0; j < sLocalParam.iBlock8x8Height; j++) {
      pRefTmp = pRefY;
      pCurTmp = pCurY;
      for (int32_t i = 0; i < sLocalParam.iBlock8x8Width; i++) {
        int32_t iBlockPointX = i << 3;
        int32_t iBlockPointY = j << 3;
        uint8_t uiBlockIdcTmp = NO_STATIC;
        int32_t iSad = m_pfSad (pCurTmp, sLocalParam.iCurStride, pRefTmp, sLocalParam.iRefStride);
        if (iSad == 0) {
          uiBlockIdcTmp = COLLOCATED_STATIC;
        } else if (bScrollDetectFlag && (!iScrollMvX || !iScrollMvY) && (iBlockPointX + iScrollMvX >= 0)
                   && (iBlockPointX + iScrollMvX <= iWidth - 8) &&
                   (iBlockPointY + iScrollMvY >= 0) && (iBlockPointY + iScrollMvY <= iHeight - 8)) {
          uint8_t* pRefTmpScroll = pRefTmp + iScrollMvY * sLocalParam.iRefStride + iScrollMvX;
          int32_t iSadScroll = m_pfSad (pCurTmp, sLocalParam.iCurStride, pRefTmpScroll, sLocalParam.iRefStride);

          if (iSadScroll == 0) {
            uiBlockIdcTmp = SCROLLED_STATIC;
          } else {
            m_sParam.iFrameComplexity += iSad;
            m_sParam.iMotionBlockNum += iSad > HIGH_MOTION_BLOCK_THRESHOLD;
          }
        } else {
          m_sParam.iFrameComplexity += iSad;
          m_sParam.iMotionBlockNum += iSad > HIGH_MOTION_BLOCK_THRESHOLD;
        }
        * (sLocalParam.pStaticBlockIdc) ++ = uiBlockIdcTmp;
        pRefTmp += 8;
        pCurTmp += 8;
      }
      pRefY += iRefRowStride;
      pCurY += iCurRowStride;
    }
  }
};

template<typename T>
class CSceneChangeDetection : public IStrategy {
 public:
  CSceneChangeDetection (EMethods eMethod, int32_t iCpuFlag): m_cDetector (m_sSceneChangeParam, iCpuFlag) {
    m_eMethod   = eMethod;
    WelsMemset (&m_sSceneChangeParam, 0, sizeof (m_sSceneChangeParam));
  }

  ~CSceneChangeDetection() {
  }

  EResult Process (int32_t iType, SPixMap* pSrcPixMap, SPixMap* pRefPixMap) {
    EResult eReturn = RET_INVALIDPARAM;

    m_sLocalParam.iWidth = pSrcPixMap->sRect.iRectWidth;
    m_sLocalParam.iHeight = pSrcPixMap->sRect.iRectHeight;
    m_sLocalParam.iBlock8x8Width = m_sLocalParam.iWidth >> 3;
    m_sLocalParam.iBlock8x8Height = m_sLocalParam.iHeight >> 3;
    m_sLocalParam.pRefY = (uint8_t*)pRefPixMap->pPixel[0];
    m_sLocalParam.pCurY = (uint8_t*)pSrcPixMap->pPixel[0];
    m_sLocalParam.iRefStride = pRefPixMap->iStride[0];
    m_sLocalParam.iCurStride = pSrcPixMap->iStride[0];
    m_sLocalParam.pStaticBlockIdc = m_sSceneChangeParam.pStaticBlockIdc;

    int32_t iBlock8x8Num = m_sLocalParam.iBlock8x8Width * m_sLocalParam.iBlock8x8Height;
    int32_t iSceneChangeThresholdLarge = WelsStaticCast (int32_t,
                                         m_cDetector.GetSceneChangeMotionRatioLarge() * iBlock8x8Num + 0.5f + PESN);
    int32_t iSceneChangeThresholdMedium = WelsStaticCast (int32_t,
                                          m_cDetector.GetSceneChangeMotionRatioMedium() * iBlock8x8Num + 0.5f + PESN);

    m_sSceneChangeParam.iMotionBlockNum = 0;
    m_sSceneChangeParam.iFrameComplexity = 0;
    m_sSceneChangeParam.eSceneChangeIdc = SIMILAR_SCENE;

    m_cDetector (m_sLocalParam);

    if (m_sSceneChangeParam.iMotionBlockNum >= iSceneChangeThresholdLarge) {
      m_sSceneChangeParam.eSceneChangeIdc = LARGE_CHANGED_SCENE;
    } else if (m_sSceneChangeParam.iMotionBlockNum >= iSceneChangeThresholdMedium) {
      m_sSceneChangeParam.eSceneChangeIdc = MEDIUM_CHANGED_SCENE;
    }

    eReturn = RET_SUCCESS;

    return eReturn;
  }

  EResult Get (int32_t iType, void* pParam) {
    if (pParam == NULL) {
      return RET_INVALIDPARAM;
    }
    * (SSceneChangeResult*)pParam = m_sSceneChangeParam;
    return RET_SUCCESS;
  }

  EResult Set (int32_t iType, void* pParam) {
    if (pParam == NULL) {
      return RET_INVALIDPARAM;
    }
    m_sSceneChangeParam = * (SSceneChangeResult*)pParam;
    return RET_SUCCESS;
  }
 private:
  SSceneChangeResult m_sSceneChangeParam;
  SLocalParam m_sLocalParam;
  T          m_cDetector;
};

IStrategy* BuildSceneChangeDetection (EMethods eMethod, int32_t iCpuFlag);

WELSVP_NAMESPACE_END

#endif
