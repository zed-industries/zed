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
 * \file    intra_pred_common.h
 *
 * \brief   interfaces for intra predictor about 16x16.
 *
 * \date    4/2/2014 Created
 *
 *************************************************************************************
 */

#ifndef INTRA_PRED_COMMON_H
#define INTRA_PRED_COMMON_H

#include "typedefs.h"


void WelsI16x16LumaPredV_c (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
void WelsI16x16LumaPredH_c (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);


#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#if defined(X86_ASM)
//for intra-prediction ASM functions
void WelsI16x16LumaPredV_sse2 (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
void WelsI16x16LumaPredH_sse2 (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
#endif//X86_ASM

#if defined(HAVE_NEON)
void WelsI16x16LumaPredV_neon (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
void WelsI16x16LumaPredH_neon (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
#endif//HAVE_NEON

#if defined(HAVE_NEON_AARCH64)
void WelsI16x16LumaPredV_AArch64_neon (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
void WelsI16x16LumaPredH_AArch64_neon (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
#endif//HAVE_NEON_AARCH64

#if defined(HAVE_MMI)
void WelsI16x16LumaPredV_mmi (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
void WelsI16x16LumaPredH_mmi (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
#endif//HAVE_MMI

#if defined(HAVE_LSX)
void WelsI16x16LumaPredV_lsx (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
void WelsI16x16LumaPredH_lsx (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);
#endif//HAVE_LSX
#if defined(__cplusplus)
}
#endif//__cplusplus
#endif//



