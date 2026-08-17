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
 *  \file   manage_dec_ref.h
 *
 *  Abstract
 *      Interface for managing reference picture
 *
 *  History
 *      08/14/2009 Created
 *
 *****************************************************************************/
#ifndef WELS_MANAGE_DEC_REF_H__
#define WELS_MANAGE_DEC_REF_H__


#include "typedefs.h"
#include "decoder_context.h"

namespace WelsDec {

void  WelsResetRefPic (PWelsDecoderContext pCtx);
void  WelsResetRefPicWithoutUnRef (PWelsDecoderContext pCtx);
int32_t WelsInitRefList (PWelsDecoderContext pCtx, int32_t iPoc);
int32_t WelsInitBSliceRefList (PWelsDecoderContext pCtx, int32_t iPoc);
int32_t WelsReorderRefList (PWelsDecoderContext pCtx);
int32_t WelsReorderRefList2 (PWelsDecoderContext pCtx);
int32_t WelsMarkAsRef (PWelsDecoderContext pCtx, PPicture pLastDec = NULL);

} // namespace WelsDec

#endif//WELS_MANAGE_DEC_REF_H__


