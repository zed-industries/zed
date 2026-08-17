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
 * \file         :  SceneChangeDetectionCommon.h
 *
 * \brief        :  scene change detection class of wels video processor class
 *
 * \date         :  2011/03/14
 *
 * \description  :  1. rewrite the package code of scene change detection class
 *
 */

#ifndef WELSVP_COMMON_H
#define WELSVP_COMMON_H

#include "util.h"
#include "memory.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"
#include "sad_common.h"
#include "intra_pred_common.h"



typedef void (GetIntraPred) (uint8_t* pPred, uint8_t* pRef, const int32_t kiStride);

typedef GetIntraPred*  GetIntraPredPtr;

GetIntraPred     WelsI16x16LumaPredV_c;
GetIntraPred     WelsI16x16LumaPredH_c;

WELSVP_NAMESPACE_BEGIN

typedef  int32_t (SadFunc) (uint8_t* pSrcY, int32_t iSrcStrideY, uint8_t* pRefY, int32_t iRefStrideY);

typedef SadFunc*   SadFuncPtr;

typedef int32_t (Sad16x16Func) (uint8_t* pSrcY, int32_t iSrcStrideY, uint8_t* pRefY, int32_t iRefStrideY);
typedef Sad16x16Func*      PSad16x16Func;


#ifdef HAVE_NEON
WELSVP_EXTERN_C_BEGIN
int32_t WelsProcessingSampleSad8x8_neon (uint8_t*, int32_t, uint8_t*, int32_t);
WELSVP_EXTERN_C_END
#endif

#ifdef HAVE_NEON_AARCH64
WELSVP_EXTERN_C_BEGIN
int32_t WelsProcessingSampleSad8x8_AArch64_neon (uint8_t*, int32_t, uint8_t*, int32_t);
WELSVP_EXTERN_C_END
#endif

WELSVP_NAMESPACE_END

#endif
