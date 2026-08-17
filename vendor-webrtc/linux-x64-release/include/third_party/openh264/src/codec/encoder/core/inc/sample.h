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

#ifndef SAMPLE_H_
#define SAMPLE_H_

#include "typedefs.h"
#include "wels_func_ptr_def.h"

namespace WelsEnc {

//======================SATD======================//
int32_t WelsSampleSatd16x16_c (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_c (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_c (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x8_c (uint8_t*, int32_t, uint8_t*, int32_t);
//int32_t WelsSampleSatd8x4( uint8_t *, int32_t, uint8_t *, int32_t );
//int32_t WelsSampleSatd4x8( uint8_t *, int32_t, uint8_t *, int32_t );
int32_t WelsSampleSatd4x4_c (uint8_t*, int32_t, uint8_t*, int32_t);

int32_t WelsSampleSatdIntra4x4Combined3_c (uint8_t*, int32_t, uint8_t*, int32_t, uint8_t*, int32_t*, int32_t, int32_t,
    int32_t);
int32_t WelsSampleSatdIntra16x16Combined3_c (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsSampleSadIntra16x16Combined3_c (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsSampleSatdIntra8x8Combined3_c (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*,
    uint8_t*, uint8_t*);
int32_t WelsSampleSadIntra8x8Combined3_c (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*,
    uint8_t*, uint8_t*);

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#if defined (X86_ASM)


int32_t WelsSampleSatd8x8_sse2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_sse2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_sse2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x16_sse2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd4x4_sse2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatdThree4x4_sse2 (uint8_t*, int32_t, uint8_t*, int32_t, uint8_t*, int32_t*, int32_t, int32_t,
                                     int32_t);

int32_t WelsSampleSatd8x8_sse41 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_sse41 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_sse41 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x16_sse41 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd4x4_sse41 (uint8_t*, int32_t, uint8_t*, int32_t);

int32_t WelsIntra16x16Combined3Satd_sse41 (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsIntra16x16Combined3Sad_ssse3 (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsIntraChroma8x8Combined3Satd_sse41 (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*,
    uint8_t*, uint8_t*);

int32_t WelsSampleSatd8x8_avx2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_avx2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_avx2 (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x16_avx2 (uint8_t*, int32_t, uint8_t*, int32_t);

#endif//X86_ASM

#if defined (HAVE_NEON)


int32_t WelsSampleSatd8x8_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x16_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd4x4_neon (uint8_t*, int32_t, uint8_t*, int32_t);

int32_t WelsIntra16x16Combined3Satd_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsIntra16x16Combined3Sad_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsIntra8x8Combined3Satd_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*, uint8_t*,
                                        uint8_t*);
int32_t WelsIntra8x8Combined3Sad_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*, uint8_t*,
                                       uint8_t*);
int32_t WelsIntra4x4Combined3Satd_neon (uint8_t*, int32_t, uint8_t*, int32_t, uint8_t*, int32_t*, int32_t, int32_t,
                                        int32_t);

#endif

#if defined (HAVE_NEON_AARCH64)
int32_t WelsSampleSatd4x4_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x16_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x8_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsIntra16x16Combined3Satd_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsIntra16x16Combined3Sad_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*);
int32_t WelsIntra8x8Combined3Satd_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*, uint8_t*,
                                            uint8_t*);
int32_t WelsIntra8x8Combined3Sad_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*, uint8_t*,
                                           uint8_t*);
int32_t WelsIntra4x4Combined3Satd_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t, uint8_t*, int32_t*, int32_t, int32_t,
                                            int32_t);
#endif

#if defined (HAVE_MMI)
int32_t WelsSampleSatd8x8_mmi (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_mmi (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_mmi (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x16_mmi (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd4x4_mmi (uint8_t*, int32_t, uint8_t*, int32_t);
#endif//HAVE_MMI

#if defined (HAVE_LASX)
int32_t WelsSampleSatd4x4_lasx (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x8_lasx (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x8_lasx (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd8x16_lasx (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsSampleSatd16x16_lasx (uint8_t*, int32_t, uint8_t*, int32_t);
int32_t WelsIntra8x8Combined3Sad_lasx (uint8_t*, int32_t, uint8_t*, int32_t, int32_t*, int32_t, uint8_t*, uint8_t*,
                                       uint8_t*);
#endif

#if defined(__cplusplus)
}
#endif//__cplusplus

void WelsInitSampleSadFunc (SWelsFuncPtrList* pFuncList, uint32_t uiCpuFlag);

}

#endif //SAMPLE_H_
