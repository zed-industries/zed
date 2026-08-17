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
 * \file         :  ScrollDectection.h
 *
 * \brief        :  scroll detection class of wels video processor class
 *
 * \date         :  2011/04/26
 *
 * \description  :  rewrite the package code of scroll detection class
 *
 *************************************************************************************
 */

#include "util.h"
#include "memory.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"

WELSVP_NAMESPACE_BEGIN

class CScrollDetection : public IStrategy {
 public:
  CScrollDetection (int32_t iCpuFlag) {
    m_eMethod = METHOD_SCROLL_DETECTION;
    WelsMemset (&m_sScrollDetectionParam, 0, sizeof (m_sScrollDetectionParam));
  }
  ~CScrollDetection() {
  }
  EResult Process (int32_t iType, SPixMap* pSrcPixMap, SPixMap* pRefPixMap);
  EResult Set (int32_t iType, void* pParam);
  EResult Get (int32_t iType, void* pParam);

 private:
  void ScrollDetectionWithMask (SPixMap* pSrcPixMap, SPixMap* pRefPixMap);
  void ScrollDetectionWithoutMask (SPixMap* pSrcPixMap, SPixMap* pRefPixMap);
 private:
  SScrollDetectionParam m_sScrollDetectionParam;
};

WELSVP_NAMESPACE_END
