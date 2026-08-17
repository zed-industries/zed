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
 * \file        :  denoise.h
 *
 * \brief       :  denoise class of wels video processor class
 *
 * \date        :  2011/03/15
 *
 * \description :  1. rewrite the package code of denoise class
 *
 *************************************************************************************
 */

#ifndef WELSVP_DENOISE_H
#define WELSVP_DENOISE_H

#include "util.h"
#include "memory.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"


#define DENOISE_GRAY_RADIUS (1)
#define DENOISE_GRAY_SIGMA  (2)

#define UV_WINDOWS_RADIUS   (2)
#define TAIL_OF_LINE8       (7)

#define DENOISE_Y_COMPONENT (1)
#define DENOISE_U_COMPONENT (2)
#define DENOISE_V_COMPONENT (4)
#define DENOISE_ALL_COMPONENT (7)


WELSVP_NAMESPACE_BEGIN

void Gauss3x3Filter (uint8_t* pixels, int32_t stride);

typedef void (DenoiseFilterFunc) (uint8_t* pixels, int32_t stride);

typedef DenoiseFilterFunc* DenoiseFilterFuncPtr;

DenoiseFilterFunc     BilateralLumaFilter8_c;
DenoiseFilterFunc     WaverageChromaFilter8_c;

#ifdef X86_ASM
WELSVP_EXTERN_C_BEGIN
DenoiseFilterFunc     BilateralLumaFilter8_sse2 ;
DenoiseFilterFunc     WaverageChromaFilter8_sse2 ;
WELSVP_EXTERN_C_END
#endif

typedef  struct TagDenoiseFuncs {
  DenoiseFilterFuncPtr pfBilateralLumaFilter8;//on 8 samples
  DenoiseFilterFuncPtr pfWaverageChromaFilter8;//on 8 samples
} SDenoiseFuncs;

class CDenoiser : public IStrategy {
 public:
  CDenoiser (int32_t iCpuFlag);
  ~CDenoiser();

  EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* dst);

 private:
  void InitDenoiseFunc (SDenoiseFuncs& pf, int32_t cpu);
  void BilateralDenoiseLuma (uint8_t* p_y_data, int32_t width, int32_t height, int32_t stride);
  void WaverageDenoiseChroma (uint8_t* pSrcUV, int32_t width, int32_t height, int32_t stride);

 private:
  float          m_fSigmaGrey;                  //sigma for grey scale similarity, suggestion 2.5-3
  uint16_t       m_uiSpaceRadius;               //filter windows radius: 1-3x3, 2-5x5,3-7x7. Larger size, slower speed
  uint16_t       m_uiType;                      //do denoising on which component 1-Y, 2-U, 4-V; 7-YUV, 3-YU, 5-YV, 6-UV

  SDenoiseFuncs m_pfDenoise;
  int32_t      m_CPUFlag;
};

WELSVP_NAMESPACE_END

#endif
