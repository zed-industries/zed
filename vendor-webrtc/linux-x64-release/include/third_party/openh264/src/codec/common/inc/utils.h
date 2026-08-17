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
 * \brief   Tool kits for decoder
 *          ( malloc, realloc, free, log output and PSNR calculation and so on )
 *
 * \date    03/10/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_UTILS_H__
#define WELS_UTILS_H__

#include <stdarg.h>
#include "typedefs.h"

#define MAX_LOG_SIZE    1024
#define MAX_MBS_PER_FRAME 36864 //in accordance with max level support in Rec
/*
 *  Function pointer declaration for various tool sets
 */
// wels log output
typedef void (*PWelsLogCallbackFunc) (void* pCtx, const int32_t iLevel, const char* kpFmt, va_list argv);

typedef struct TagLogContext {
  PWelsLogCallbackFunc pfLog;
  void* pLogCtx;
  void* pCodecInstance;
} SLogContext;


#ifdef __GNUC__
extern void WelsLog (SLogContext* pCtx, int32_t iLevel, const char* kpFmt, ...) __attribute__ ((__format__ (__printf__,
    3,
    4)));
#else
extern void WelsLog (SLogContext* pCtx, int32_t iLevel, const char* kpFmt, ...);
#endif

/*
 *  PSNR calculation routines
 */
/*!
 *************************************************************************************
 * \brief   PSNR calculation utilization in Wels
 *
 * \param   kpTarPic    target picture to be calculated in Picture pData format
 * \param   kiTarStride stride of target picture pData pBuffer
 * \param   kpRefPic    base referencing picture samples
 * \param   kiRefStride stride of reference picture pData pBuffer
 * \param   kiWidth     picture iWidth in pixel
 * \param   kiHeight    picture iHeight in pixel
 *
 * \return  actual PSNR result;
 *
 * \note    N/A
 *************************************************************************************
 */
float WelsCalcPsnr (const void* kpTarPic,
                    const int32_t kiTarStride,
                    const void* kpRefPic,
                    const int32_t kiRefStride,
                    const int32_t kiWidth,
                    const int32_t kiHeight);


#endif//WELS_UTILS_H__
