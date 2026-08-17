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
 * \file        :  BackgroundDetection.h
 *
 * \brief       :  background detection class of wels video processor class
 *
 * \date        :  2011/03/17
 *
 * \description :  1. rewrite the package code of background detection class
 *
 */

#ifndef WELSVP_BACKGROUNDDETECTION_H
#define WELSVP_BACKGROUNDDETECTION_H

#include "util.h"
#include "memory.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"

WELSVP_NAMESPACE_BEGIN

typedef struct {
  int32_t       iBackgroundFlag;
  int32_t       iSAD;
  int32_t       iSD;
  int32_t       iMAD;
  int32_t       iMinSubMad;
  int32_t       iMaxDiffSubSd;
} SBackgroundOU;

class CBackgroundDetection : public IStrategy {
 public:
  CBackgroundDetection (int32_t iCpuFlag);
  ~CBackgroundDetection();

  EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* pRef);
  EResult Set (int32_t iType, void* pParam);

 private:
  struct vBGDParam {
    uint8_t*   pCur[3];
    uint8_t*   pRef[3];
    int32_t    iBgdWidth;
    int32_t    iBgdHeight;
    int32_t    iStride[3];
    SBackgroundOU*   pOU_array;
    int8_t*    pBackgroundMbFlag;
    SVAACalcResult*  pCalcRes;
  } m_BgdParam;

  int32_t     m_iLargestFrameSize;

 private:
  inline SBackgroundOU* AllocateOUArrayMemory (int32_t iWidth, int32_t iHeight);
  inline int32_t  CalculateAsdChromaEdge (uint8_t* pOriRef, uint8_t* pOriCur, int32_t iStride);
  inline bool   ForegroundDilation23Luma (SBackgroundOU* pBackgroundOU,
                                          SBackgroundOU* pOUNeighbours[]); //Foreground_Dilation_2_3_Luma
  inline bool   ForegroundDilation23Chroma (int8_t iNeighbourForegroundFlags, int32_t iStartSamplePos,
      int32_t iPicStrideUV, vBGDParam* pBgdParam);//Foreground_Dilation_2_3_Chroma
  inline void     ForegroundDilation (SBackgroundOU* pBackgroundOU, SBackgroundOU* pOUNeighbours[], vBGDParam* pBgdParam,
                                      int32_t iChromaSampleStartPos);
  inline void     BackgroundErosion (SBackgroundOU* pBackgroundOU, SBackgroundOU* pOUNeighbours[]);
  inline void     SetBackgroundMbFlag (int8_t* pBackgroundMbFlag, int32_t iPicWidthInMb, int32_t iBackgroundMbFlag);
  inline void     UpperOUForegroundCheck (SBackgroundOU* pCurOU, int8_t* pBackgroundMbFlag, int32_t iPicWidthInOU,
                                          int32_t iPicWidthInMb);

  void    GetOUParameters (SVAACalcResult* sVaaCalcInfo, int32_t iMbIndex, int32_t iMbWidth,
                           SBackgroundOU* pBackgroundOU);
  void    ForegroundBackgroundDivision (vBGDParam* pBgdParam);
  void    ForegroundDilationAndBackgroundErosion (vBGDParam* pBgdParam);
  void    BackgroundDetection (vBGDParam* pBgdParam);
};

WELSVP_NAMESPACE_END

#endif
