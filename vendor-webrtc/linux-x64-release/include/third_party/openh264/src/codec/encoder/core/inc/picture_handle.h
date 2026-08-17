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
 * \file    picture_handle.h
 *
 * \brief   picture pData handling
 *
 * \date    5/20/2009 Created
 *
 *************************************************************************************/
#if !defined(WELS_ENCODER_PICTURE_HANDLE_H__)
#define WELS_ENCODER_PICTURE_HANDLE_H__

#include "picture.h"
#include "typedefs.h"
#include "memory_align.h"

namespace WelsEnc {
/*!
 * \brief   alloc picture pData with borders for each plane based width and height of picture
 * \param   kiWidth                 width of picture in pixels
 * \param   kiHeight                height of picture in pixels
 * \param   bNeedMbInfo             need pData allocation
 * \pram    iNeedFeatureStorage     need storage for FME
 * \return  successful if effective picture pointer returned, otherwise failed with NULL
 */
SPicture* AllocPicture (CMemoryAlign* pMa, const int32_t kiWidth, const int32_t kiHeight, bool bNeedMbInfo,
                        int32_t iNeedFeatureStorage);

/*!
 * \brief   free picture pData planes
 * \param   pic     picture pointer to be destoryed
 * \return  none
 */
void FreePicture (CMemoryAlign* pMa, SPicture** ppPic);

}
#endif//WELS_ENCODER_PICTURE_HANDLE_H__
