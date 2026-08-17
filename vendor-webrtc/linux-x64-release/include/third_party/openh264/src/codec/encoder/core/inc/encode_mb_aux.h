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

#ifndef ENCODE_MB_AUX_H
#define ENCODE_MB_AUX_H

#include "typedefs.h"
#include "wels_func_ptr_def.h"
#include "copy_mb.h"

namespace WelsEnc {
void WelsInitEncodingFuncs (SWelsFuncPtrList* pFuncList, uint32_t  uiCpuFlag);
int32_t WelsGetNoneZeroCount_c (int16_t* pLevel);

/****************************************************************************
 * Scan and Score functions
 ****************************************************************************/
void    WelsScan4x4Ac_c (int16_t* pZigValue, int16_t* pDct);
void    WelsScan4x4Dc (int16_t* pLevel, int16_t* pDct);
void    WelsScan4x4DcAc_c (int16_t* pLevel, int16_t* pDct);
int32_t WelsCalculateSingleCtr4x4_c (int16_t* pDct);

/****************************************************************************
 * HDM and Quant functions
 ****************************************************************************/
void WelsHadamardT4Dc_c (int16_t* pLumaDc, int16_t* pDct);
int32_t WelsHadamardQuant2x2_c (int16_t* pRes, const int16_t kiFF, int16_t iMF, int16_t* pDct, int16_t* pBlock);
int32_t WelsHadamardQuant2x2Skip_c (int16_t* pRes, int16_t iFF,  int16_t iMF);

void WelsQuant4x4_c (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuant4x4Dc_c (int16_t* pDct, int16_t iFF,  int16_t iMF);
void WelsQuantFour4x4_c (int16_t* pDct, const int16_t* pFF, const int16_t* pQpTable);
void WelsQuantFour4x4Max_c (int16_t* pDct, const int16_t* pF, const int16_t* pQpTable, int16_t* pMax);


/****************************************************************************
 * DCT functions
 ****************************************************************************/
void WelsDctT4_c (int16_t* pDct, uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
// dct_data is no-use here, just for the same interface with dct_save functions
void WelsDctFourT4_c (int16_t* pDct, uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#ifdef X86_ASM

int32_t WelsGetNoneZeroCount_sse2 (int16_t* pLevel);
int32_t WelsGetNoneZeroCount_sse42 (int16_t* pLevel);

/****************************************************************************
 * Scan and Score functions
 ****************************************************************************/
void WelsScan4x4Ac_sse2 (int16_t* zig_value, int16_t* pDct);
void WelsScan4x4DcAc_ssse3 (int16_t* pLevel, int16_t* pDct);
void WelsScan4x4DcAc_sse2 (int16_t* pLevel, int16_t* pDct);
int32_t WelsCalculateSingleCtr4x4_sse2 (int16_t* pDct);

/****************************************************************************
 * DCT functions
 ****************************************************************************/
void WelsDctT4_mmx (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctT4_sse2 (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctFourT4_sse2 (int16_t* pDct, uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctT4_avx2 (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctFourT4_avx2 (int16_t* pDct, uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);

/****************************************************************************
 * HDM and Quant functions
 ****************************************************************************/
int32_t WelsHadamardQuant2x2_mmx (int16_t* pRes, const int16_t kiFF, int16_t iMF, int16_t* pDct, int16_t* pBlock);
void WelsHadamardT4Dc_sse2 (int16_t* pLumaDc, int16_t* pDct);
int32_t WelsHadamardQuant2x2Skip_mmx (int16_t* pRes, int16_t iFF,  int16_t iMF);

void WelsQuant4x4_sse2 (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuant4x4Dc_sse2 (int16_t* pDct, int16_t iFF, int16_t iMF);
void WelsQuantFour4x4_sse2 (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuantFour4x4Max_sse2 (int16_t* pDct, const int16_t* pFF, const int16_t* pMF, int16_t* pMax);

void WelsQuant4x4_avx2 (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuant4x4Dc_avx2 (int16_t* pDct, int16_t iFF, int16_t iMF);
void WelsQuantFour4x4_avx2 (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuantFour4x4Max_avx2 (int16_t* pDct, const int16_t* pFF, const int16_t* pMF, int16_t* pMax);

#endif

#ifdef HAVE_NEON
void WelsHadamardT4Dc_neon (int16_t* pLumaDc, int16_t* pDct);
int32_t WelsHadamardQuant2x2_neon (int16_t* pRes, const int16_t kiFF, int16_t iMF, int16_t* pDct, int16_t* pBlock);
int32_t WelsHadamardQuant2x2Skip_neon (int16_t* pRes, int16_t iFF,  int16_t iMF);
int32_t WelsHadamardQuant2x2SkipKernel_neon (int16_t* pRes, int16_t iThreshold); // avoid divide operator

void WelsDctT4_neon (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctFourT4_neon (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);

int32_t WelsGetNoneZeroCount_neon (int16_t* pLevel);

void WelsQuant4x4_neon (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuant4x4Dc_neon (int16_t* pDct, int16_t iFF, int16_t iMF);
void WelsQuantFour4x4_neon (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuantFour4x4Max_neon (int16_t* pDct, const int16_t* pFF, const int16_t* pMF, int16_t* pMax);
#endif

#ifdef HAVE_NEON_AARCH64
void WelsHadamardT4Dc_AArch64_neon (int16_t* pLumaDc, int16_t* pDct);
int32_t WelsHadamardQuant2x2_AArch64_neon (int16_t* pRes, const int16_t kiFF, int16_t iMF, int16_t* pDct, int16_t* pBlock);
int32_t WelsHadamardQuant2x2Skip_AArch64_neon (int16_t* pRes, int16_t iFF,  int16_t iMF);
int32_t WelsHadamardQuant2x2SkipKernel_AArch64_neon (int16_t* pRes, int16_t iThreshold); // avoid divide operator

void WelsDctT4_AArch64_neon (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctFourT4_AArch64_neon (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);

int32_t WelsGetNoneZeroCount_AArch64_neon (int16_t* pLevel);

void WelsQuant4x4_AArch64_neon (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuant4x4Dc_AArch64_neon (int16_t* pDct, int16_t iFF, int16_t iMF);
void WelsQuantFour4x4_AArch64_neon (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuantFour4x4Max_AArch64_neon (int16_t* pDct, const int16_t* pFF, const int16_t* pMF, int16_t* pMax);
#endif

#ifdef HAVE_MMI
int32_t WelsGetNoneZeroCount_mmi (int16_t* pLevel);

/****************************************************************************
 *  * Scan and Score functions
 *   ****************************************************************************/
void WelsScan4x4Ac_mmi (int16_t* zig_value, int16_t* pDct);
void WelsScan4x4DcAc_mmi (int16_t* pLevel, int16_t* pDct);
int32_t WelsCalculateSingleCtr4x4_mmi (int16_t* pDct);

/****************************************************************************
 *  * DCT functions
 *   ****************************************************************************/
void WelsDctT4_mmi (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctFourT4_mmi (int16_t* pDct, uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);

/****************************************************************************
 *  * HDM and Quant functions
 *   ****************************************************************************/
void WelsHadamardT4Dc_mmi (int16_t* pLumaDc, int16_t* pDct);

void WelsQuant4x4_mmi (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuant4x4Dc_mmi (int16_t* pDct, int16_t iFF, int16_t iMF);
void WelsQuantFour4x4_mmi (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuantFour4x4Max_mmi (int16_t* pDct, const int16_t* pFF, const int16_t* pMF, int16_t* pMax);
#endif//HAVE_MMI

#ifdef HAVE_LSX
void WelsQuantFour4x4_lsx (int16_t* pDct, const int16_t* pFF, const int16_t* pMF);
void WelsQuantFour4x4Max_lsx (int16_t* pDct, const int16_t* pFF, const int16_t* pMF, int16_t* pMax);
#endif//HAVE_LSX

#ifdef HAVE_LASX
/****************************************************************************
 *  * DCT functions
 *   ****************************************************************************/
void WelsDctT4_lasx (int16_t* pDct,  uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
void WelsDctFourT4_lasx (int16_t* pDct, uint8_t* pPixel1, int32_t iStride1, uint8_t* pPixel2, int32_t iStride2);
#endif

#if defined(__cplusplus)
}
#endif//__cplusplus

ALIGNED_DECLARE (extern const int16_t, g_kiQuantInterFF[58][8], 16);
#define g_iQuantIntraFF (g_kiQuantInterFF +6 )
ALIGNED_DECLARE (extern const int16_t, g_kiQuantMF[52][8], 16);
}
#endif//ENCODE_MB_AUX_H
