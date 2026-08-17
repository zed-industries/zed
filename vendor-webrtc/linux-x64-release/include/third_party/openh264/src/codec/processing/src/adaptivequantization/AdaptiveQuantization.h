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
 * \file         :  AdaptiveQuantization.h
 *
 * \brief        :  adaptive quantization class of wels video processor class
 *
 * \date         :  2011/03/21
 *
 * \description  :  1. rewrite the package code of scene change detection class
 *
 */

#ifndef WELSVP_ADAPTIVEQUANTIZATION_H
#define WELSVP_ADAPTIVEQUANTIZATION_H

#include "util.h"
#include "memory.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"
#include "cpu.h"

WELSVP_NAMESPACE_BEGIN

typedef void (VarFunc) (uint8_t* pRefY, int32_t iRefStrideY, uint8_t* pSrc, int32_t iSrcStrideY,
                        SMotionTextureUnit* pMotionTexture);

typedef VarFunc*   PVarFunc;

VarFunc      SampleVariance16x16_c;

#ifdef X86_ASM
WELSVP_EXTERN_C_BEGIN
VarFunc      SampleVariance16x16_sse2;
WELSVP_EXTERN_C_END
#endif

#ifdef HAVE_NEON
WELSVP_EXTERN_C_BEGIN
VarFunc      SampleVariance16x16_neon;
WELSVP_EXTERN_C_END
#endif

#ifdef HAVE_NEON_AARCH64
WELSVP_EXTERN_C_BEGIN
VarFunc      SampleVariance16x16_AArch64_neon;
WELSVP_EXTERN_C_END
#endif

class CAdaptiveQuantization : public IStrategy {
 public:
  CAdaptiveQuantization (int32_t iCpuFlag);
  ~CAdaptiveQuantization();

  EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* pRef);
  EResult Set (int32_t iType, void* pParam);
  EResult Get (int32_t iType, void* pParam);

 private:
  void WelsInitVarFunc (PVarFunc& pfVar, int32_t iCpuFlag);

 private:
  PVarFunc                      m_pfVar;
  int32_t                       m_CPUFlag;
  SAdaptiveQuantizationParam    m_sAdaptiveQuantParam;
};

WELSVP_NAMESPACE_END

#endif
