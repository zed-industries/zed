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

//mb_cache.h
#ifndef WELS_MACROBLOCK_CACHE_H__
#define WELS_MACROBLOCK_CACHE_H__

#include "typedefs.h"

namespace WelsDec {

#define REF_NOT_AVAIL    -2
#define REF_NOT_IN_LIST  -1  //intra

/*
 *  MB Cache information, such one cache should be defined within a slice
 */
/*
 * Cache for Luma               Cache for Chroma(Cb, Cr)
 *
 *  TL T T T T                  TL T T
 *   L - - - -                   L - -
 *   L - - - -                   L - - TR
 *   L - - - -
 *   L - - - - TR
 *
 */

////////////////////////mapping scan index////////////////////////

extern const uint8_t g_kuiScan4[16];

typedef struct TagNeighborAvail {
int32_t iTopAvail;
int32_t iLeftAvail;
int32_t iRightTopAvail;
int32_t iLeftTopAvail;  //used for check intra_pred_mode avail or not   //1: avail; 0: unavail

int32_t iLeftType;
int32_t iTopType;
int32_t iLeftTopType;
int32_t iRightTopType;

int8_t  iTopCbp;
int8_t  iLeftCbp;
int8_t iDummy[2]; //for align
} SWelsNeighAvail, *PWelsNeighAvail;

} // namespace WelsDec

#endif//WELS_MACROBLOCK_CACHE_H__
