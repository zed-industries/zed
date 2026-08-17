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

#ifndef DAV1D_SRC_CTX_H
#define DAV1D_SRC_CTX_H

#include <stdint.h>

#include "common/attributes.h"
#include "common/intops.h"

union alias64 { uint64_t u64; uint8_t u8[8]; } ATTR_ALIAS;
union alias32 { uint32_t u32; uint8_t u8[4]; } ATTR_ALIAS;
union alias16 { uint16_t u16; uint8_t u8[2]; } ATTR_ALIAS;
union alias8 { uint8_t u8; } ATTR_ALIAS;

typedef void (*dav1d_memset_pow2_fn)(void *ptr, int value);
EXTERN const dav1d_memset_pow2_fn dav1d_memset_pow2[6];

static inline void dav1d_memset_likely_pow2(void *const ptr, const int value, const int n) {
    assert(n >= 1 && n <= 32);
    if ((n&(n-1)) == 0) {
        dav1d_memset_pow2[ulog2(n)](ptr, value);
    } else {
        memset(ptr, value, n);
    }
}

// For smaller sizes use multiplication to broadcast bytes. memset misbehaves on the smaller sizes.
// For the larger sizes, we want to use memset to get access to vector operations.
#define set_ctx1(var, off, val) \
    ((union alias8 *) &(var)[off])->u8 = (val) * 0x01
#define set_ctx2(var, off, val) \
    ((union alias16 *) &(var)[off])->u16 = (val) * 0x0101
#define set_ctx4(var, off, val) \
    ((union alias32 *) &(var)[off])->u32 = (val) * 0x01010101U
#define set_ctx8(var, off, val) \
    ((union alias64 *) &(var)[off])->u64 = (val) * 0x0101010101010101ULL
#define set_ctx16(var, off, val) do { \
        memset(&(var)[off], val, 16); \
    } while (0)
#define set_ctx32(var, off, val) do { \
        memset(&(var)[off], val, 32); \
    } while (0)
#define case_set(var) \
    switch (var) { \
    case 0: set_ctx(set_ctx1); break; \
    case 1: set_ctx(set_ctx2); break; \
    case 2: set_ctx(set_ctx4); break; \
    case 3: set_ctx(set_ctx8); break; \
    case 4: set_ctx(set_ctx16); break; \
    case 5: set_ctx(set_ctx32); break; \
    default: assert(0); \
    }
#define case_set_upto16(var) \
    switch (var) { \
    case 0: set_ctx(set_ctx1); break; \
    case 1: set_ctx(set_ctx2); break; \
    case 2: set_ctx(set_ctx4); break; \
    case 3: set_ctx(set_ctx8); break; \
    case 4: set_ctx(set_ctx16); break; \
    default: assert(0); \
    }

#endif /* DAV1D_SRC_CTX_H */
