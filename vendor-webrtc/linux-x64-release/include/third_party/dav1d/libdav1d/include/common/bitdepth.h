/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_COMMON_BITDEPTH_H
#define DAV1D_COMMON_BITDEPTH_H

#include <stdint.h>
#include <string.h>

#include "common/attributes.h"

#if !defined(BITDEPTH)
typedef uint8_t pixel; /* can't be void due to pointer-to-array usage */
typedef void coef;
#define HIGHBD_DECL_SUFFIX /* nothing */
#define HIGHBD_CALL_SUFFIX /* nothing */
#define HIGHBD_TAIL_SUFFIX /* nothing */
#elif BITDEPTH == 8
typedef uint8_t pixel;
typedef int16_t coef;
#define PIXEL_TYPE uint8_t
#define COEF_TYPE int16_t
#define pixel_copy memcpy
#define pixel_set memset
#define iclip_pixel iclip_u8
#define PIX_HEX_FMT "%02x"
#define bitfn(x) x##_8bpc
#define BF(x, suffix) x##_8bpc_##suffix
#define PXSTRIDE(x) (x)
#define highbd_only(x)
#define HIGHBD_DECL_SUFFIX /* nothing */
#define HIGHBD_CALL_SUFFIX /* nothing */
#define HIGHBD_TAIL_SUFFIX /* nothing */
#define bitdepth_from_max(x) 8
#define BITDEPTH_MAX 0xff
#elif BITDEPTH == 16
typedef uint16_t pixel;
typedef int32_t coef;
#define PIXEL_TYPE uint16_t
#define COEF_TYPE int32_t
#define pixel_copy(a, b, c) memcpy(a, b, (c) << 1)
static inline void pixel_set(pixel *const dst, const int val, const int num) {
    for (int n = 0; n < num; n++)
        dst[n] = val;
}
#define PIX_HEX_FMT "%03x"
#define iclip_pixel(x) iclip(x, 0, bitdepth_max)
#define HIGHBD_DECL_SUFFIX , const int bitdepth_max
#define HIGHBD_CALL_SUFFIX , f->bitdepth_max
#define HIGHBD_TAIL_SUFFIX , bitdepth_max
#define bitdepth_from_max(bitdepth_max) (32 - clz(bitdepth_max))
#define BITDEPTH_MAX bitdepth_max
#define bitfn(x) x##_16bpc
#define BF(x, suffix) x##_16bpc_##suffix
static inline ptrdiff_t PXSTRIDE(const ptrdiff_t x) {
    assert(!(x & 1));
    return x >> 1;
}
#define highbd_only(x) x
#else
#error invalid value for bitdepth
#endif
#define bytefn(x) bitfn(x)

#define bitfn_decls(name, ...) \
name##_8bpc(__VA_ARGS__); \
name##_16bpc(__VA_ARGS__)

#endif /* DAV1D_COMMON_BITDEPTH_H */
