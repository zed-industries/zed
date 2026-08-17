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
 * \file        :  downsample.h
 *
 * \brief       :  image rotate class of wels video processor class
 *
 * \date        :  2011/04/06
 *
 * \description :
 *
 *************************************************************************************
 */

#ifndef WELSVP_IMAGEROTATE_H
#define WELSVP_IMAGEROTATE_H

#include "util.h"
#include "WelsFrameWork.h"
#include "IWelsVP.h"

WELSVP_NAMESPACE_BEGIN

typedef void (ImageRotateFunc) (uint8_t* pSrc, uint32_t uiBytesPerPixel, uint32_t iWidth, uint32_t iHeight,
                                uint8_t* pDst);

typedef ImageRotateFunc* ImageRotateFuncPtr;

ImageRotateFunc   ImageRotate90D_c;
ImageRotateFunc   ImageRotate180D_c;
ImageRotateFunc   ImageRotate270D_c;

typedef struct {
  ImageRotateFuncPtr    pfImageRotate90D;
  ImageRotateFuncPtr    pfImageRotate180D;
  ImageRotateFuncPtr    pfImageRotate270D;
} SImageRotateFuncs;

class CImageRotating : public IStrategy {
 public:
  CImageRotating (int32_t iCpuFlag);
  ~CImageRotating();

  EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* pDst);

 private:
  void InitImageRotateFuncs (SImageRotateFuncs& pf, int32_t iCpuFlag);
  EResult ProcessImageRotate (int32_t iType, uint8_t* pSrc, uint32_t uiBytesPerPixel, uint32_t iWidth, uint32_t iHeight,
                              uint8_t* pDst);

 private:
  SImageRotateFuncs m_pfRotateImage;
  int32_t          m_iCPUFlag;
};

WELSVP_NAMESPACE_END

#endif
