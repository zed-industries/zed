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
* \file         :  ComplexityAnalysis.h
*
* \brief        :  complexity analysis class of wels video processor class
*
* \date         :  2011/03/28
*
* \description  :  1. rewrite the package code of complexity analysis class
*
*************************************************************************************
*/

#ifndef WELSVP_COMPLEXITYANALYSIS_H
#define WELSVP_COMPLEXITYANALYSIS_H

#include "util.h"
#include "memory.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"
#include "common.h"

WELSVP_NAMESPACE_BEGIN

typedef  void (GOMSadFunc) (uint32_t* pGomSad, int32_t* pGomForegroundBlockNum, int32_t* pSad8x8,
                            uint8_t pBackgroundMbFlag);

typedef GOMSadFunc*   PGOMSadFunc;

GOMSadFunc      GomSampleSad;
GOMSadFunc      GomSampleSadExceptBackground;

class CComplexityAnalysis : public IStrategy {
 public:
  CComplexityAnalysis (int32_t iCpuFlag);
  ~CComplexityAnalysis();

  EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* pRef);
  EResult Set (int32_t iType, void* pParam);
  EResult Get (int32_t iType, void* pParam);

 private:
  void AnalyzeFrameComplexityViaSad (SPixMap* pSrc, SPixMap* pRef);
  int32_t GetFrameSadExcludeBackground (SPixMap* pSrc, SPixMap* pRef);

  void AnalyzeGomComplexityViaSad (SPixMap* pSrc, SPixMap* pRef);
  void AnalyzeGomComplexityViaVar (SPixMap* pSrc, SPixMap* pRef);

 private:
  PGOMSadFunc m_pfGomSad;
  SComplexityAnalysisParam m_sComplexityAnalysisParam;
};


//for screen content

class CComplexityAnalysisScreen : public IStrategy {
 public:
  CComplexityAnalysisScreen (int32_t cpu_flag);
  ~CComplexityAnalysisScreen();

  EResult Process (int32_t nType, SPixMap* src, SPixMap* ref);
  EResult Set (int32_t nType, void* pParam);
  EResult Get (int32_t nType, void* pParam);

 private:
  void GomComplexityAnalysisIntra (SPixMap* pSrc);
  void GomComplexityAnalysisInter (SPixMap* pSrc, SPixMap* pRef, bool bScrollFlag);

 private:
  PSad16x16Func m_pSadFunc;
  GetIntraPredPtr m_pIntraFunc[2];
  SComplexityAnalysisScreenParam m_ComplexityAnalysisParam;
};


WELSVP_NAMESPACE_END

#endif
