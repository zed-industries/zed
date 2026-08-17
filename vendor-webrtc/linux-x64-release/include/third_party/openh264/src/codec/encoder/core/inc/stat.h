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
 * \file    stat.h
 *
 * \brief   statistical pData information
 *
 * \date    4/22/2009 Created
 *
 *************************************************************************************
 */
#if !defined(WELS_ENCODER_STATISTICAL_DATA_H__)
#define WELS_ENCODER_STATISTICAL_DATA_H__

namespace WelsEnc {

/*
 *  Stat quality
 */
typedef struct TagStatQuality {

float   rYPsnr[5];
float   rUPsnr[5];
float   rVPsnr[5];

} SStatQuality;

/*
 *  Stat complexity pData
 */
typedef struct TagComplexityStat {

#ifdef FME_TEST
int32_t         cost_time;
int32_t         me_time;
int32_t         mvp_time;
int32_t         mvb_time;
#endif

// any else?

} SComplexityStat;

/*
 *  Stat slice details information
 */
typedef struct TagStatSliceInfo {

/* per slice info */
int32_t         iSliceCount[5];
int32_t         iSliceSize [5];
int32_t         iMbCount   [5][18];

} SStatSliceInfo;

/*
 *  For overall statistical pData
 */
typedef struct TagStatData {

// Quality
SStatQuality    sQualityStat;

// Complexity
SComplexityStat sComplexityStat;

// SSlice information output
SStatSliceInfo  sSliceData;

} SStatData;

}

#endif//WELS_ENCODER_STATISTICAL_DATA_H__
