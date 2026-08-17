/*!
 * \copy
 *     Copyright (c)  2013, Cisco Systems
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
 */

#ifndef MC_H
#define MC_H

#include "typedefs.h"

typedef void (*PWelsMcFunc) (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int16_t iMvX, int16_t iMvY, int32_t iWidth, int32_t iHeight);

typedef void (*PWelsLumaHalfpelMcFunc) (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                        int32_t iWidth, int32_t iHeight);
typedef void (*PWelsSampleAveragingFunc) (uint8_t*, int32_t, const uint8_t*, int32_t, const uint8_t*, int32_t,
    int32_t, int32_t);

typedef struct TagMcFunc {
  PWelsLumaHalfpelMcFunc      pfLumaHalfpelHor;
  PWelsLumaHalfpelMcFunc      pfLumaHalfpelVer;
  PWelsLumaHalfpelMcFunc      pfLumaHalfpelCen;
  PWelsMcFunc                 pMcChromaFunc;

  PWelsMcFunc                 pMcLumaFunc;
  PWelsSampleAveragingFunc    pfSampleAveraging;
} SMcFunc;

namespace WelsCommon {

void InitMcFunc (SMcFunc* pMcFunc, uint32_t iCpu);

} // namespace WelsCommon


#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#if defined(HAVE_NEON)
void McCopyWidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);

void McCopyWidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);

void McCopyWidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);

void McChromaWidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                            int32_t* pWeights, int32_t iHeight);

void McChromaWidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                            int32_t* pWeights, int32_t iHeight);

void PixelAvgWidthEq16_neon (uint8_t* pDst, int32_t iDstStride, uint8_t* pSrcA, uint8_t* pSrcB, int32_t iHeight);
void PixelAvgWidthEq8_neon (uint8_t* pDst, int32_t iDstStride, uint8_t* pSrcA, uint8_t* pSrcB, int32_t iHeight);
void PixelAvgWidthEq4_neon (uint8_t* pDst, int32_t iDstStride, uint8_t* pSrcA, uint8_t* pSrcB, int32_t iHeight);

void McHorVer01WidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer01WidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer01WidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer03WidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer03WidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer03WidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);

void McHorVer10WidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer10WidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer10WidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer30WidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer30WidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer30WidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);

//horizontal filter to gain half sample, that is (2, 0) location in quarter sample
void McHorVer20WidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer20WidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer20WidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);

//vertical filter to gain half sample, that is (0, 2) location in quarter sample
void McHorVer02WidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer02WidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer02WidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);

//horizontal and vertical filter to gain half sample, that is (2, 2) location in quarter sample
void McHorVer22WidthEq16_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer22WidthEq8_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer22WidthEq4_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);

void PixStrideAvgWidthEq16_neon (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcStrideA,
                                 const uint8_t* pSrcB, int32_t iSrcStrideB, int32_t iHeight);
void PixStrideAvgWidthEq8_neon (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcStrideA,
                                const uint8_t* pSrcB, int32_t iSrcStrideB, int32_t iHeight);

void McHorVer20Width17_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);// width+1
void McHorVer20Width9_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                            int32_t iHeight);// width+1
void McHorVer20Width5_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                            int32_t iHeight);// width+1

void McHorVer02Height17_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);// height+1
void McHorVer02Height9_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);// height+1
void McHorVer02Height5_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);// height+1

void McHorVer22Width17_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);//width+1&&height+1
void McHorVer22Width9_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                            int32_t iHeight);//width+1&&height+1
void McHorVer22Width5_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                            int32_t iHeight);//width+1&&height+1
#endif

#if defined(HAVE_NEON_AARCH64)
void McCopyWidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                  int32_t iHeight);
void McCopyWidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                  int32_t iHeight);
void McCopyWidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                   int32_t iHeight);
void McChromaWidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                    int32_t* pWeights, int32_t iHeight);
void McChromaWidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                    int32_t* pWeights, int32_t iHeight);
void PixelAvgWidthEq16_AArch64_neon (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                                     const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);
void PixelAvgWidthEq8_AArch64_neon (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                                    const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);
void PixelAvgWidthEq4_AArch64_neon (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                                    const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);
void McHorVer01WidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                       int32_t iHeight);
void McHorVer01WidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer01WidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer03WidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                       int32_t iHeight);
void McHorVer03WidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer03WidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer10WidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                       int32_t iHeight);
void McHorVer10WidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer10WidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer30WidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                       int32_t iHeight);
void McHorVer30WidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer30WidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
//horizontal filter to gain half sample, that is (2, 0) location in quarter sample
void McHorVer20WidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                       int32_t iHeight);
void McHorVer20WidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer20WidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
//vertical filter to gain half sample, that is (0, 2) location in quarter sample
void McHorVer02WidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                       int32_t iHeight);
void McHorVer02WidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer02WidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
//horizontal and vertical filter to gain half sample, that is (2, 2) location in quarter sample
void McHorVer22WidthEq16_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                       int32_t iHeight);
void McHorVer22WidthEq8_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void McHorVer22WidthEq4_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);
void PixStrideAvgWidthEq16_AArch64_neon (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcStrideA,
    const uint8_t* pSrcB, int32_t iSrcStrideB, int32_t iHeight);
void PixStrideAvgWidthEq8_AArch64_neon (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcStrideA,
                                        const uint8_t* pSrcB, int32_t iSrcStrideB, int32_t iHeight);
void McHorVer20Width17_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                     int32_t iHeight);// width+1
void McHorVer20Width9_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                    int32_t iHeight);// width+1
void McHorVer20Width5_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                    int32_t iHeight);// width+1
void McHorVer02Height17_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                      int32_t iHeight);// height+1
void McHorVer02Height9_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                     int32_t iHeight);// height+1
void McHorVer02Height5_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                     int32_t iHeight);// height+1
void McHorVer22Width17_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                     int32_t iHeight);//width+1&&height+1
void McHorVer22Width9_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                    int32_t iHeight);//width+1&&height+1
void McHorVer22Width5_AArch64_neon (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                    int32_t iHeight);//width+1&&height+1
#endif

#if defined(X86_ASM)
//***************************************************************************//
//                       MMXEXT definition                                   //
//***************************************************************************//
void McHorVer20WidthEq4_mmx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);
void McChromaWidthEq4_mmx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                           const uint8_t* kpABCD, int32_t iHeight);
void McCopyWidthEq8_mmx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                         int32_t iHeight);
void PixelAvgWidthEq4_mmx (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                           const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);
void PixelAvgWidthEq8_mmx (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                           const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);

//***************************************************************************//
//                       SSE2 definition                                     //
//***************************************************************************//
void McChromaWidthEq8_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                            const uint8_t* kpABCD, int32_t iHeight);
void McCopyWidthEq16_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                           int32_t iHeight);
void McHorVer20WidthEq8_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer20WidthEq16_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                               int32_t iHeight);
void McHorVer02WidthEq8_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer22Width8HorFirst_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                    int32_t iHeight);
void McHorVer22Width8VerLastAlign_sse2 (const uint8_t* pTap, int32_t iTapStride, uint8_t* pDst, int32_t iDstStride,
                                        int32_t iWidth, int32_t iHeight);
void McHorVer22Width8VerLastUnAlign_sse2 (const uint8_t* pTap, int32_t iTapStride, uint8_t* pDst, int32_t iDstStride,
    int32_t iWidth, int32_t iHeight);

void PixelAvgWidthEq16_sse2 (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                             const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);

void McHorVer20Width9Or17_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                int32_t iWidth,
                                int32_t iHeight);
void McHorVer20Width5_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                int32_t iWidth, int32_t iHeight);

void McHorVer02Height9Or17_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                 int32_t iWidth,
                                 int32_t iHeight);
void McHorVer02Height5_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                                 int32_t iWidth, int32_t iHeight);

void McHorVer22HorFirst_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pTap, int32_t iTapStride,
                              int32_t iWidth,
                              int32_t iHeight);
void McHorVer22Width5HorFirst_sse2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pTap, int32_t iTapStride,
                              int32_t iWidth, int32_t iHeight);
void McHorVer22Width4VerLastAlign_sse2 (const uint8_t* pTap, int32_t iTapStride, uint8_t* pDst, int32_t iDstStride,
                                        int32_t iWidth, int32_t iHeight);
void McHorVer22Width4VerLastUnAlign_sse2 (const uint8_t* pTap, int32_t iTapStride, uint8_t* pDst, int32_t iDstStride,
        int32_t iWidth, int32_t iHeight);

//***************************************************************************//
//                       SSE3 definition                                     //
//***************************************************************************//
void McCopyWidthEq16_sse3 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                           int32_t iHeight);

//***************************************************************************//
//                       SSSE3 definition                                    //
//***************************************************************************//
void McChromaWidthEq8_ssse3 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             const uint8_t* kpABCD, int32_t iHeight);
void McHorVer02_ssse3 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                       int32_t iWidth, int32_t iHeight);
void McHorVer02Width4S16ToU8_ssse3 (const int16_t* pSrc, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McHorVer02Width5S16ToU8_ssse3 (const int16_t* pSrc, int32_t iSrcStride,
                                    uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McHorVer02WidthGe8S16ToU8_ssse3 (const int16_t* pSrc, int32_t iSrcStride,
                                      uint8_t* pDst, int32_t iDstStride, int32_t iWidth, int32_t iHeight);
void McHorVer20_ssse3 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                       int32_t iWidth, int32_t iHeight);
void McHorVer20Width4U8ToS16_ssse3 (const uint8_t* pSrc, int32_t iSrcStride, int16_t* pDst, int32_t iHeight);
void McHorVer20Width5Or9Or17_ssse3 (const uint8_t* pSrc, int32_t iSrcStride,
                                    uint8_t* pDst, int32_t iDstStride, int32_t iWidth, int32_t iHeight);
void McHorVer20Width8U8ToS16_ssse3 (const uint8_t* pSrc, int32_t iSrcStride,
                                    int16_t* pDst, int32_t iDstStride, int32_t iHeight);
void McHorVer20Width9Or17U8ToS16_ssse3 (const uint8_t* pSrc, int32_t iSrcStride,
                                        int16_t* pDst, int32_t iDstStride, int32_t iWidth, int32_t iHeight);

//***************************************************************************//
//                       AVX2 definition                                     //
//***************************************************************************//
#ifdef HAVE_AVX2
void McHorVer02_avx2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                      int32_t iWidth, int32_t iHeight);
void McHorVer02Width4S16ToU8_avx2 (const int16_t* pSrc, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McHorVer02Width5S16ToU8_avx2 (const int16_t* pSrc, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McHorVer02Width8S16ToU8_avx2 (const int16_t* pSrc, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McHorVer02Width9S16ToU8_avx2 (const int16_t* pSrc, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McHorVer02Width16Or17S16ToU8_avx2 (const int16_t* pSrc, int32_t iSrcStride,
                                        uint8_t* pDst, int32_t iDstStride, int32_t iWidth, int32_t iHeight);
void McHorVer20_avx2 (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                      int32_t iWidth, int32_t iHeight);
void McHorVer20Width5Or9Or17_avx2 (const uint8_t* pSrc, int32_t iSrcStride,
                                   uint8_t* pDst, int32_t iDstStride, int32_t iWidth, int32_t iHeight);
void McHorVer20Width4U8ToS16_avx2 (const uint8_t* pSrc, int32_t iSrcStride, int16_t* pDst, int32_t iHeight);
void McHorVer20Width8U8ToS16_avx2 (const uint8_t* pSrc, int32_t iSrcStride, int16_t* pDst, int32_t iHeight);
void McHorVer20Width16U8ToS16_avx2 (const uint8_t* pSrc, int32_t iSrcStride, int16_t* pDst, int32_t iHeight);
void McHorVer20Width17U8ToS16_avx2 (const uint8_t* pSrc, int32_t iSrcStride, int16_t* pDst, int32_t iHeight);
#endif //HAVE_AVX2

#endif //X86_ASM

//***************************************************************************//
//                       LSX definition                                      //
//***************************************************************************//
#if defined(HAVE_LSX)
void McCopyWidthEq4_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McCopyWidthEq8_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);
void McCopyWidthEq16_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride, int32_t iHeight);

void McChromaWidthEq4_lsx (const uint8_t *pSrc, int32_t iSrcStride, uint8_t *pDst, int32_t iDstStride,
                           const uint8_t *pABCD, int32_t iHeight);
void McChromaWidthEq8_lsx (const uint8_t *pSrc, int32_t iSrcStride, uint8_t *pDst, int32_t iDstStride,
                           const uint8_t *pABCD, int32_t iHeight);
void PixelAvgWidthEq4_lsx (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                           const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);
void PixelAvgWidthEq8_lsx (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                           const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);
void PixelAvgWidthEq16_lsx (uint8_t* pDst, int32_t iDstStride, const uint8_t* pSrcA, int32_t iSrcAStride,
                           const uint8_t* pSrcB, int32_t iSrcBStride, int32_t iHeight);
void McHorVer02WidthEq8_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);
void McHorVer02WidthEq16_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer20WidthEq4_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);
void McHorVer20WidthEq5_lsx (const uint8_t *pSrc, int32_t iSrcStride, uint8_t *pDst, int32_t iDstStride,
                             int32_t iHeight);
void McHorVer20WidthEq8_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);
void McHorVer20WidthEq9_lsx (const uint8_t *pSrc, int32_t iSrcStride, uint8_t *pDst, int32_t iDstStride,
                             int32_t iHeight);
void McHorVer20WidthEq17_lsx (const uint8_t *pSrc, int32_t iSrcStride, uint8_t *pDst, int32_t iDstStride,
                              int iHeight);
void McHorVer20WidthEq16_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                              int32_t iHeight);
void McHorVer22WidthEq5_lsx(const uint8_t *pSrc, int32_t iSrcStride, uint8_t *pDst, int32_t iDstStride,
                            int32_t iHeight);
void McHorVer22WidthEq8_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);
void McHorVer22WidthEq9_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);
void McHorVer22WidthEq17_lsx (const uint8_t* pSrc, int32_t iSrcStride, uint8_t* pDst, int32_t iDstStride,
                             int32_t iHeight);
#endif//HAVE_LSX

#if defined(__cplusplus)
}
#endif//__cplusplus

#endif//MC_H
