/*
 * Copyright © 2024, VideoLAN and dav1d authors
 * Copyright © 2024, Luca Barbato
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

#ifndef DAV1D_SRC_PPC_UTILS_H
#define DAV1D_SRC_PPC_UTILS_H

#include "src/ppc/dav1d_types.h"

#define assert_eq(a, b) \
    if ((a) != (b)) \
        printf("%d: %d vs %d\n", __LINE__, a, b); \
    assert((a) == (b));

#define MERGE_I32(a, b, h, l) \
{ \
    h = vec_mergeh(a, b); \
    l = vec_mergel(a, b); \
}

#define DECLARE_MERGE_I32(a, b, h, l) \
    i32x4 h, l; \
    MERGE_I32(a, b, h, l)


// Transpose a 4x4 matrix of i32x4 vectors
#define TRANSPOSE4_I32(c0, c1, c2, c3) \
{ \
    DECLARE_MERGE_I32(c0, c2, m02h, m02l) \
    DECLARE_MERGE_I32(c1, c3, m13h, m13l) \
\
    MERGE_I32(m02h, m13h, c0, c1) \
    MERGE_I32(m02l, m13l, c2, c3) \
}

// Transpose a 8x8 matrix of i32x4 vectors
#define TRANSPOSE8_I32(c0, c1, c2, c3, c4, c5, c6, c7, \
                       c8, c9, cA, cB, cC, cD, cE, cF) \
{ \
    DECLARE_MERGE_I32(c0, c2, m02h, m02l) \
    DECLARE_MERGE_I32(c1, c3, m13h, m13l) \
    DECLARE_MERGE_I32(c4, c6, m46h, m46l) \
    DECLARE_MERGE_I32(c5, c7, m57h, m57l) \
    DECLARE_MERGE_I32(c8, cA, m8Ah, m8Al) \
    DECLARE_MERGE_I32(c9, cB, m9Bh, m9Bl) \
    DECLARE_MERGE_I32(cC, cE, mCEh, mCEl) \
    DECLARE_MERGE_I32(cD, cF, mDFh, mDFl) \
\
    MERGE_I32(m02h, m13h, c0, c1) \
    MERGE_I32(m02l, m13l, c2, c3) \
    MERGE_I32(m46h, m57h, c8, c9) \
    MERGE_I32(m46l, m57l, cA, cB) \
    MERGE_I32(m8Ah, m9Bh, c4, c5) \
    MERGE_I32(m8Al, m9Bl, c6, c7) \
    MERGE_I32(mCEh, mDFh, cC, cD) \
    MERGE_I32(mCEl, mDFl, cE, cF) \
}

// Transpose a 4x16 matrix of i32x4 vectors
#define TRANSPOSE4x16_I32(c0, c1, c2, c3, c4, c5, c6, c7, \
                          c8, c9, cA, cB, cC, cD, cE, cF) \
{ \
    DECLARE_MERGE_I32(c0, c2, m02h, m02l) \
    DECLARE_MERGE_I32(c1, c3, m13h, m13l) \
    DECLARE_MERGE_I32(c4, c6, m46h, m46l) \
    DECLARE_MERGE_I32(c5, c7, m57h, m57l) \
    DECLARE_MERGE_I32(c8, cA, m8Ah, m8Al) \
    DECLARE_MERGE_I32(c9, cB, m9Bh, m9Bl) \
    DECLARE_MERGE_I32(cC, cE, mCEh, mCEl) \
    DECLARE_MERGE_I32(cD, cF, mDFh, mDFl) \
\
    MERGE_I32(m02h, m13h, c0, c1) \
    MERGE_I32(m02l, m13l, c2, c3) \
    MERGE_I32(m46h, m57h, c4, c5) \
    MERGE_I32(m46l, m57l, c6, c7) \
    MERGE_I32(m8Ah, m9Bh, c8, c9) \
    MERGE_I32(m8Al, m9Bl, cA, cB) \
    MERGE_I32(mCEh, mDFh, cC, cD) \
    MERGE_I32(mCEl, mDFl, cE, cF) \
}

#endif // DAV1D_SRC_PPC_UTILS_H
