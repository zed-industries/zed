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
 * \file        :  downsample.h
 *
 * \brief       :  downsample class of wels video processor class
 *
 * \date        :  2011/03/33
 *
 * \description :  1. rewrite the package code of downsample class
 *
 *************************************************************************************
 */

#ifndef WELSVP_DOWNSAMPLE_H
#define WELSVP_DOWNSAMPLE_H

#include "util.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"
#include "macros.h"

WELSVP_NAMESPACE_BEGIN


typedef void (HalveDownsampleFunc) (uint8_t* pDst, const int32_t kiDstStride,
                                    uint8_t* pSrc, const int32_t kiSrcStride,
                                    const int32_t kiSrcWidth, const int32_t kiSrcHeight);

typedef void (SpecificDownsampleFunc) (uint8_t* pDst, const int32_t kiDstStride,
                                       uint8_t* pSrc, const int32_t kiSrcStride,
                                       const int32_t kiSrcWidth, const int32_t kiHeight);

typedef void (GeneralDownsampleFunc) (uint8_t* pDst, const int32_t kiDstStride, const int32_t kiDstWidth,
                                      const int32_t kiDstHeight,
                                      uint8_t* pSrc, const int32_t kiSrcStride, const int32_t kiSrcWidth, const int32_t kiSrcHeight);

typedef HalveDownsampleFunc*    PHalveDownsampleFunc;
typedef SpecificDownsampleFunc* PSpecificDownsampleFunc;
typedef GeneralDownsampleFunc*  PGeneralDownsampleFunc;

HalveDownsampleFunc     DyadicBilinearDownsampler_c;
GeneralDownsampleFunc GeneralBilinearFastDownsampler_c;
GeneralDownsampleFunc GeneralBilinearAccurateDownsampler_c;
SpecificDownsampleFunc  DyadicBilinearOneThirdDownsampler_c;
SpecificDownsampleFunc  DyadicBilinearQuarterDownsampler_c;

typedef struct {
  PHalveDownsampleFunc          pfHalfAverageWidthx32;
  PHalveDownsampleFunc          pfHalfAverageWidthx16;
  PSpecificDownsampleFunc       pfOneThirdDownsampler;
  PSpecificDownsampleFunc       pfQuarterDownsampler;
  PGeneralDownsampleFunc        pfGeneralRatioLuma;
  PGeneralDownsampleFunc        pfGeneralRatioChroma;
} SDownsampleFuncs;


#ifdef X86_ASM
WELSVP_EXTERN_C_BEGIN
// used for scr width is multipler of 8 pixels
HalveDownsampleFunc     DyadicBilinearDownsamplerWidthx8_sse;
// iSrcWidth= x16 pixels
HalveDownsampleFunc     DyadicBilinearDownsamplerWidthx16_sse;
// iSrcWidth= x32 pixels
HalveDownsampleFunc     DyadicBilinearDownsamplerWidthx32_sse;
// used for scr width is multipler of 16 pixels
HalveDownsampleFunc     DyadicBilinearDownsamplerWidthx16_ssse3;
// iSrcWidth= x32 pixels
HalveDownsampleFunc     DyadicBilinearDownsamplerWidthx32_ssse3;

GeneralDownsampleFunc GeneralBilinearFastDownsamplerWrap_sse2;
GeneralDownsampleFunc GeneralBilinearAccurateDownsamplerWrap_sse2;
GeneralDownsampleFunc GeneralBilinearFastDownsamplerWrap_ssse3;
GeneralDownsampleFunc GeneralBilinearAccurateDownsamplerWrap_sse41;
#ifdef HAVE_AVX2
GeneralDownsampleFunc GeneralBilinearFastDownsamplerWrap_avx2;
GeneralDownsampleFunc GeneralBilinearAccurateDownsamplerWrap_avx2;
#endif

SpecificDownsampleFunc  DyadicBilinearOneThirdDownsampler_ssse3;
SpecificDownsampleFunc  DyadicBilinearOneThirdDownsampler_sse4;
SpecificDownsampleFunc  DyadicBilinearQuarterDownsampler_sse;
SpecificDownsampleFunc  DyadicBilinearQuarterDownsampler_ssse3;
SpecificDownsampleFunc  DyadicBilinearQuarterDownsampler_sse4;

void GeneralBilinearFastDownsampler_sse2 (uint8_t* pDst, const int32_t kiDstStride, const int32_t kiDstWidth,
    const int32_t kiDstHeight, uint8_t* pSrc, const int32_t kiSrcStride, const uint32_t kuiScaleX,
    const uint32_t kuiScaleY);
void GeneralBilinearAccurateDownsampler_sse2 (uint8_t* pDst, const int32_t kiDstStride, const int32_t kiDstWidth,
    const int32_t kiDstHeight, uint8_t* pSrc, const int32_t kiSrcStride, const uint32_t kuiScaleX,
    const uint32_t kuiScaleY);
void GeneralBilinearFastDownsampler_ssse3 (uint8_t* pDst, int32_t iDstStride, int32_t iDstWidth,
    int32_t iDstHeight, uint8_t* pSrc, int32_t iSrcStride, uint32_t uiScaleX,
    uint32_t uiScaleY);
void GeneralBilinearAccurateDownsampler_sse41 (uint8_t* pDst, int32_t iDstStride, int32_t iDstWidth,
    int32_t iDstHeight, uint8_t* pSrc, int32_t iSrcStride, uint32_t uiScaleX,
    uint32_t uiScaleY);
#ifdef HAVE_AVX2
void GeneralBilinearFastDownsampler_avx2 (uint8_t* pDst, int32_t iDstStride, int32_t iDstWidth,
    int32_t iDstHeight, uint8_t* pSrc, int32_t iSrcStride, uint32_t uiScaleX,
    uint32_t uiScaleY);
void GeneralBilinearAccurateDownsampler_avx2 (uint8_t* pDst, int32_t iDstStride, int32_t iDstWidth,
    int32_t iDstHeight, uint8_t* pSrc, int32_t iSrcStride, uint32_t uiScaleX,
    uint32_t uiScaleY);
#endif

WELSVP_EXTERN_C_END
#endif

#ifdef HAVE_NEON
WELSVP_EXTERN_C_BEGIN
// iSrcWidth no limitation
HalveDownsampleFunc     DyadicBilinearDownsampler_neon;
// iSrcWidth = x32 pixels
HalveDownsampleFunc     DyadicBilinearDownsamplerWidthx32_neon;

GeneralDownsampleFunc   GeneralBilinearAccurateDownsamplerWrap_neon;

SpecificDownsampleFunc  DyadicBilinearOneThirdDownsampler_neon;

SpecificDownsampleFunc  DyadicBilinearQuarterDownsampler_neon;

void GeneralBilinearAccurateDownsampler_neon (uint8_t* pDst, const int32_t kiDstStride, const int32_t kiDstWidth,
    const int32_t kiDstHeight,
    uint8_t* pSrc, const int32_t kiSrcStride, const uint32_t kuiScaleX, const uint32_t kuiScaleY);

WELSVP_EXTERN_C_END
#endif

#ifdef HAVE_NEON_AARCH64
WELSVP_EXTERN_C_BEGIN
// iSrcWidth no limitation
HalveDownsampleFunc     DyadicBilinearDownsampler_AArch64_neon;
// iSrcWidth = x32 pixels
HalveDownsampleFunc     DyadicBilinearDownsamplerWidthx32_AArch64_neon;

GeneralDownsampleFunc   GeneralBilinearAccurateDownsamplerWrap_AArch64_neon;

SpecificDownsampleFunc  DyadicBilinearOneThirdDownsampler_AArch64_neon;

SpecificDownsampleFunc  DyadicBilinearQuarterDownsampler_AArch64_neon;

void GeneralBilinearAccurateDownsampler_AArch64_neon (uint8_t* pDst, const int32_t kiDstStride,
    const int32_t kiDstWidth, const int32_t kiDstHeight,
    uint8_t* pSrc, const int32_t kiSrcStride, const uint32_t kuiScaleX, const uint32_t kuiScaleY);

WELSVP_EXTERN_C_END
#endif


class CDownsampling : public IStrategy {
 public:
  CDownsampling (int32_t iCpuFlag);
  ~CDownsampling();

  EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* pDst);

 private:
  void InitDownsampleFuncs (SDownsampleFuncs& sDownsampleFunc, int32_t iCpuFlag);

  void DownsampleHalfAverage (uint8_t* pDst, int32_t iDstStride,
      uint8_t* pSrc, int32_t iSrcStride, int32_t iSrcWidth, int32_t iSrcHeight);
  bool AllocateSampleBuffer();
  void FreeSampleBuffer();
 private:
  SDownsampleFuncs m_pfDownsample;
  int32_t  m_iCPUFlag;
  uint8_t  *m_pSampleBuffer[2][3];
  bool     m_bNoSampleBuffer;
};

WELSVP_NAMESPACE_END

#endif
