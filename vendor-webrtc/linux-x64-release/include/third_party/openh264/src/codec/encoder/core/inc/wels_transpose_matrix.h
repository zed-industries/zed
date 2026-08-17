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
#ifndef WELS_RUBY_ENCODER_TRANSPOSE_MATRIX_H__
#define WELS_RUBY_ENCODER_TRANSPOSE_MATRIX_H__

#include "typedefs.h"

namespace WelsEnc {

#ifdef X86_ASM
extern "C"
{
  void TransposeMatrixBlocksx16_sse2 (void* pDst, const int32_t kiDstStride, void* pSrc, const int32_t kiSrcStride,
                                      const int32_t kiBlocksNum);
  void TransposeMatrixBlock16x16_sse2 (void* pDst, const int32_t kiDstStride, void* pSrc, const int32_t kiSrcStride);
  void TransposeMatrixBlocksx8_mmx (void* pDst, const int32_t kiDstStride, void* pSrc, const int32_t kiSrcStride,
                                    const int32_t kiBlocksNum);
  void TransposeMatrixBlock8x8_mmx (void* pDst, const int32_t kiDstStride, void* pSrc, const int32_t kiSrcStride);
}
#endif

typedef void (*PTransposeMatrixBlockFunc) (void* pDst, const int32_t kiDstStride, void* pSrc,
    const int32_t kiSrcStride);
typedef void (*PTransposeMatrixBlocksFunc) (void* pDst, const int32_t kiDstStride, void* pSrc,
    const int32_t kiSrcStride, const int32_t kiBlocksNum);

}// end of namespace declaration

#endif//WELS_RUBY_ENCODER_TRANSPOSE_MATRIX_H__
