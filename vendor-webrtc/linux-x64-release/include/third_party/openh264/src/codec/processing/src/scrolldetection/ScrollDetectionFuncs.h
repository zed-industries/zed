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
 * \file         :  ScrollDetectionFuncs.h
 *
 * \brief        :  scroll detection class of wels video processor class
 *
 * \date         :  2011/04/26
 *
 * \description  :  rewrite the package code of scroll detection class
 *
 *************************************************************************************
 */

WELSVP_NAMESPACE_BEGIN

#define MINIMUM_DETECT_WIDTH 50  // no less than 16
#define CHECK_OFFSET 25
#define MAX_SCROLL_MV_Y 511
#define REGION_NUMBER 9
#define RECORD_COLOR(a, x)      \
{                               \
  int32_t _t = (uint8_t)(a);    \
  x[_t>>5] |= (1 << (_t&31));   \
}

int32_t CheckLine (uint8_t* pData, int32_t iWidth);
int32_t SelectTestLine (uint8_t* pY, int32_t iWidth, int32_t iHeight, int32_t iPicHeight,
                        int32_t iStride, int32_t iOffsetX, int32_t iOffsetY);
int32_t CompareLine (uint8_t* pYSrc, uint8_t* pYRef, const int32_t kiWidth);
void ScrollDetectionCore (SPixMap* pSrcPixMap, SPixMap* pRefPixMap, int32_t iWidth, int32_t iHeight,
                          int32_t iOffsetX, int32_t iOffsetY, SScrollDetectionParam& sScrollDetectionParam);

WELSVP_NAMESPACE_END
