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
 * \file    expand_pic.h
 *
 * \brief   Interface for expanding reconstructed picture to be used for reference
 *
 * \date    06/08/2009
 *************************************************************************************
 */

#ifndef EXPAND_PICTURE_H
#define EXPAND_PICTURE_H

#include "typedefs.h"

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#define PADDING_LENGTH 32 // reference extension
#define CHROMA_PADDING_LENGTH 16 // chroma reference extension

#if defined(X86_ASM)
void ExpandPictureLuma_sse2 (uint8_t* pDst,
                             const int32_t kiStride,
                             const int32_t kiPicW,
                             const int32_t kiPicH);
void ExpandPictureChromaAlign_sse2 (uint8_t* pDst,
                                    const int32_t kiStride,
                                    const int32_t kiPicW,
                                    const int32_t kiPicH);
void ExpandPictureChromaUnalign_sse2 (uint8_t* pDst,
                                      const int32_t kiStride,
                                      const int32_t kiPicW,
                                      const int32_t kiPicH);
#endif//X86_ASM

#if defined(HAVE_NEON)
void ExpandPictureLuma_neon (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW, const int32_t kiPicH);
void ExpandPictureChroma_neon (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW, const int32_t kiPicH);
#endif
#if defined(HAVE_NEON_AARCH64)
void ExpandPictureLuma_AArch64_neon (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW, const int32_t kiPicH);
void ExpandPictureChroma_AArch64_neon (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW,
                                       const int32_t kiPicH);
#endif

#if defined(HAVE_MMI)
void ExpandPictureLuma_mmi (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW,
                            const int32_t kiPicH);
void ExpandPictureChromaAlign_mmi (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW,
                                   const int32_t kiPicH);
void ExpandPictureChromaUnalign_mmi (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW,
                                     const int32_t kiPicH);
#endif//HAVE_MMI

typedef void (*PExpandPictureFunc) (uint8_t* pDst, const int32_t kiStride, const int32_t kiPicW, const int32_t kiPicH);

typedef struct TagExpandPicFunc {
  PExpandPictureFunc pfExpandLumaPicture;
  PExpandPictureFunc pfExpandChromaPicture[2];
} SExpandPicFunc;

void PadMBLuma_c (uint8_t*& pDst, const int32_t& kiStride, const int32_t& kiPicW, const int32_t& kiPicH,
                  const int32_t& kiMbX, const int32_t& kiMbY, const int32_t& kiMBWidth, const int32_t& kiMBHeight);
void PadMBChroma_c (uint8_t*& pDst, const int32_t& kiStride, const int32_t& kiPicW, const int32_t& kiPicH,
                    const int32_t& kiMbX, const int32_t& kiMbY, const int32_t& kiMBWidth, const int32_t& kiMBHeight);

void ExpandReferencingPicture (uint8_t* pData[3], int32_t iWidth, int32_t iHeight, int32_t iStride[3],
                               PExpandPictureFunc pExpLuma, PExpandPictureFunc pExpChrom[2]);

void InitExpandPictureFunc (SExpandPicFunc* pExpandPicFunc, const uint32_t kuiCPUFlags);

#if defined(__cplusplus)
}
#endif//__cplusplus

#endif
