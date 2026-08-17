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
 * \file        :  util.h
 *
 * \brief       :  utils for wels video processor class
 *
 * \date        :  2011/01/04
 *
 * \description :
 *
 *************************************************************************************
 */

#ifndef WELSVP_UTIL_H
#define WELSVP_UTIL_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <assert.h>

#include "typedef.h"
#include "memory.h"
#include "IWelsVP.h"

WELSVP_NAMESPACE_BEGIN

#define MAX_MBS_PER_FRAME 36864 //in accordance with max level support in Rec

#define MB_WIDTH_LUMA  (16)
#define PESN               (1e-6)       // desired float precision
#define AQ_INT_MULTIPLY                   10000000
#define AQ_TIME_INT_MULTIPLY                   10000
#define AQ_QSTEP_INT_MULTIPLY                   100
#define AQ_PESN 10 // (1e-6)*AQ_INT_MULTIPLY

#define MB_TYPE_INTRA4x4                0x00000001
#define MB_TYPE_INTRA16x16              0x00000002
#define MB_TYPE_INTRA_PCM               0x00000004
#define MB_TYPE_INTRA                     (MB_TYPE_INTRA4x4 | MB_TYPE_INTRA16x16 | MB_TYPE_INTRA_PCM)
#define IS_INTRA(type) ((type)&MB_TYPE_INTRA)

#define WELS_MAX(x, y) ((x) > (y) ? (x) : (y))
#define WELS_MIN(x, y) ((x) < (y) ? (x) : (y))

#ifndef WELS_SIGN
#define WELS_SIGN(a) ((int32_t)(a) >> 31)
#endif

#ifndef WELS_ABS
#define WELS_ABS(a) ((WELS_SIGN(a) ^ (int32_t)(a)) - WELS_SIGN(a))
#endif

#define WELS_CLAMP(x, minv, maxv)  WELS_MIN(WELS_MAX(x, minv), maxv)

#define ALIGNBYTES         (16)       /* Worst case is requiring alignment to an 16 byte boundary */

#define WelsCastFromPointer(p)      (reinterpret_cast<intptr_t>(p))
#define WelsStaticCast(type, p)  (static_cast<type>(p))
#define WelsDynamicCast(type, p) (dynamic_cast<type>(p))

#define GET_METHOD(x)  ((x) & 0xff)          // mask method as the lowest 8bits
#define GET_SPECIAL(x) (((x) >> 8) & 0xff)   // mask special flag as 8bits

inline EMethods WelsVpGetValidMethod (int32_t a) {
  int32_t iMethod = GET_METHOD (a);
  return WelsStaticCast (EMethods, WELS_CLAMP (iMethod, METHOD_NULL + 1, METHOD_MASK - 1));
}



WELSVP_NAMESPACE_END

#endif


