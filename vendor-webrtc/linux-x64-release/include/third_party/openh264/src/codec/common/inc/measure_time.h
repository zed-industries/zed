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
 * \file    measure_time.h
 *
 * \brief   time cost measure utilization
 *
 * \date    04/28/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_TIME_COST_MEASURE_UTIL_H__
#define WELS_TIME_COST_MEASURE_UTIL_H__

#include <stdlib.h>

#include "typedefs.h"
#ifndef _WIN32
#include <sys/time.h>
#else
#include <windows.h>
#endif
#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif//__cplusplus

/*!
 * \brief   time cost measure utilization
 * \param   void
 * \return  time elapsed since run (unit: microsecond)
 */

static inline int64_t WelsTime (void) {
#ifndef _WIN32
  struct timeval tv_date;

  gettimeofday (&tv_date, NULL);
  return ((int64_t) tv_date.tv_sec * 1000000 + (int64_t) tv_date.tv_usec);
#else
  static int64_t iMtimeFreq = 0;
  int64_t iMtimeCur = 0;
  int64_t iResult = 0;
  if (!iMtimeFreq) {
    QueryPerformanceFrequency ((LARGE_INTEGER*)&iMtimeFreq);
    if (!iMtimeFreq)
      iMtimeFreq = 1;
  }
  QueryPerformanceCounter ((LARGE_INTEGER*)&iMtimeCur);
  iResult = (int64_t) ((double)iMtimeCur * 1e6 / (double)iMtimeFreq + 0.5);
  return iResult;
#endif//_WIN32
}

#ifdef __cplusplus
}
#endif

#endif//WELS_TIME_COST_MEASURE_UTIL_H__
