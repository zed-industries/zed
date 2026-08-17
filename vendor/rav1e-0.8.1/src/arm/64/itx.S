/******************************************************************************
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2019, Martin Storsjo
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
 *****************************************************************************/

#include "src/arm/asm.S"
#include "util.S"

// The exported functions in this file have got the following signature:
// void itxfm_add(pixel *dst, ptrdiff_t dst_stride, coef *coeff, int eob);

// Most of the functions use the following register layout:
// x0-x3  external parameters
// x4     function pointer to first transform
// x5     function pointer to second transform
// x6     output parameter for helper function
// x7     input parameter for helper function
// x8     input stride for helper function
// x9-x12 scratch variables for helper functions
// x13    pointer to list of eob thresholds
// x14    return pointer for helper function
// x15    return pointer for main function

// The SIMD registers most often use the following layout:
// v0-v1   multiplication coefficients
// v2-v7   scratch registers
// v8-v15  unused
// v16-v31 inputs/outputs of transforms

// Potential further optimizations, that are left unimplemented for now:
// - Trying to keep multiplication coefficients in registers across multiple
//   transform functions. (The register layout is designed to potentially
//   allow this.)
// - Use a simplified version of the transforms themselves for cases where
//   we know a significant number of inputs are zero. E.g. if the eob value
//   indicates only a quarter of input values are set, for idct16 and up,
//   a significant amount of calculation can be skipped, at the cost of more
//   code duplication and special casing.

const idct_coeffs, align=4
        // idct4
        .short          2896, 2896*8, 1567, 3784
        // idct8
        .short          799, 4017, 3406, 2276
        // idct16
        .short          401, 4076, 3166, 2598
        .short          1931, 3612, 3920, 1189
        // idct32
        .short          201, 4091, 3035, 2751
        .short          1751, 3703, 3857, 1380
        .short          995, 3973, 3513, 2106
        .short          2440, 3290, 4052, 601
endconst

const idct64_coeffs, align=4
        .short          101*8, 4095*8, 2967*8, -2824*8
        .short          1660*8, 3745*8, 3822*8, -1474*8
        .short          4076, 401, 4017, 799
        .short          0, 0, 0, 0

        .short          4036*8, -700*8, 2359*8, 3349*8
        .short          3461*8, -2191*8, 897*8, 3996*8
        .short          -3166, -2598, -799, -4017
        .short          0, 0, 0, 0

        .short          501*8, 4065*8, 3229*8, -2520*8
        .short          2019*8, 3564*8, 3948*8, -1092*8
        .short          3612, 1931, 2276, 3406
        .short          0, 0, 0, 0

        .short          4085*8, -301*8, 2675*8, 3102*8
        .short          3659*8, -1842*8, 1285*8, 3889*8
        .short          -3920, -1189, -3406, -2276
        .short          0, 0, 0, 0
endconst

const iadst4_coeffs, align=4
        // .h[4-5] can be interpreted as .s[2]
        .short          1321, 3803, 2482, 3344, 3344, 0
endconst

const iadst8_coeffs, align=4
        .short          4076, 401, 3612, 1931
        .short          2598, 3166, 1189, 3920
        // idct_coeffs
        .short          2896, 0, 1567, 3784, 0, 0, 0, 0
endconst

const iadst16_coeffs, align=4
        .short          4091, 201, 3973, 995
        .short          3703, 1751, 3290, 2440
        .short          2751, 3035, 2106, 3513
        .short          1380, 3857, 601, 4052
endconst

.macro smull_smlal d0, d1, s0, s1, c0, c1, sz
        smull           \d0\().4s, \s0\().4h, \c0
        smlal           \d0\().4s, \s1\().4h, \c1
.ifc \sz, .8h
        smull2          \d1\().4s, \s0\().8h, \c0
        smlal2          \d1\().4s, \s1\().8h, \c1
.endif
.endm

.macro smull_smlsl d0, d1, s0, s1, c0, c1, sz
        smull           \d0\().4s, \s0\().4h, \c0
        smlsl           \d0\().4s, \s1\().4h, \c1
.ifc \sz, .8h
        smull2          \d1\().4s, \s0\().8h, \c0
        smlsl2          \d1\().4s, \s1\().8h, \c1
.endif
.endm

.macro sqrshrn_sz d0, s0, s1, shift, sz
        sqrshrn         \d0\().4h, \s0\().4s, \shift
.ifc \sz, .8h
        sqrshrn2        \d0\().8h, \s1\().4s, \shift
.endif
.endm

.macro scale_input sz, c, r0, r1, r2 r3, r4, r5, r6, r7
        sqrdmulh        \r0\sz,  \r0\sz,  \c
        sqrdmulh        \r1\sz,  \r1\sz,  \c
        sqrdmulh        \r2\sz,  \r2\sz,  \c
        sqrdmulh        \r3\sz,  \r3\sz,  \c
.ifnb \r4
        sqrdmulh        \r4\sz,  \r4\sz,  \c
        sqrdmulh        \r5\sz,  \r5\sz,  \c
        sqrdmulh        \r6\sz,  \r6\sz,  \c
        sqrdmulh        \r7\sz,  \r7\sz,  \c
.endif
.endm

.macro load_add_store load, shift, addsrc, adddst, narrowsrc, narrowdst, store, dst, src, shiftbits=4
.ifnb \load
        ld1             {\load},  [\src], x1
.endif
.ifnb \shift
        srshr           \shift,  \shift,  #\shiftbits
.endif
.ifnb \addsrc
        uaddw           \adddst, \adddst, \addsrc
.endif
.ifnb \narrowsrc
        sqxtun          \narrowdst, \narrowsrc
.endif
.ifnb \store
        st1             {\store},  [\dst], x1
.endif
.endm
.macro load_add_store_8x16 dst, src
        mov             \src, \dst
        load_add_store  v2.8b, v16.8h,      ,       ,       ,      ,      , \dst, \src
        load_add_store  v3.8b, v17.8h,      ,       ,       ,      ,      , \dst, \src
        load_add_store  v4.8b, v18.8h, v2.8b, v16.8h,       ,      ,      , \dst, \src
        load_add_store  v5.8b, v19.8h, v3.8b, v17.8h, v16.8h, v2.8b,      , \dst, \src
        load_add_store  v6.8b, v20.8h, v4.8b, v18.8h, v17.8h, v3.8b, v2.8b, \dst, \src
        load_add_store  v7.8b, v21.8h, v5.8b, v19.8h, v18.8h, v4.8b, v3.8b, \dst, \src
        load_add_store  v2.8b, v22.8h, v6.8b, v20.8h, v19.8h, v5.8b, v4.8b, \dst, \src
        load_add_store  v3.8b, v23.8h, v7.8b, v21.8h, v20.8h, v6.8b, v5.8b, \dst, \src
        load_add_store  v4.8b, v24.8h, v2.8b, v22.8h, v21.8h, v7.8b, v6.8b, \dst, \src
        load_add_store  v5.8b, v25.8h, v3.8b, v23.8h, v22.8h, v2.8b, v7.8b, \dst, \src
        load_add_store  v6.8b, v26.8h, v4.8b, v24.8h, v23.8h, v3.8b, v2.8b, \dst, \src
        load_add_store  v7.8b, v27.8h, v5.8b, v25.8h, v24.8h, v4.8b, v3.8b, \dst, \src
        load_add_store  v2.8b, v28.8h, v6.8b, v26.8h, v25.8h, v5.8b, v4.8b, \dst, \src
        load_add_store  v3.8b, v29.8h, v7.8b, v27.8h, v26.8h, v6.8b, v5.8b, \dst, \src
        load_add_store  v4.8b, v30.8h, v2.8b, v28.8h, v27.8h, v7.8b, v6.8b, \dst, \src
        load_add_store  v5.8b, v31.8h, v3.8b, v29.8h, v28.8h, v2.8b, v7.8b, \dst, \src
        load_add_store       ,       , v4.8b, v30.8h, v29.8h, v3.8b, v2.8b, \dst, \src
        load_add_store       ,       , v5.8b, v31.8h, v30.8h, v4.8b, v3.8b, \dst, \src
        load_add_store       ,       ,      ,       , v31.8h, v5.8b, v4.8b, \dst, \src
        load_add_store       ,       ,      ,       ,       ,      , v5.8b, \dst, \src
.endm
.macro load_add_store_8x8 dst, src, shiftbits=4
        mov             \src, \dst
        load_add_store  v2.8b, v16.8h,      ,       ,       ,      ,      , \dst, \src, \shiftbits
        load_add_store  v3.8b, v17.8h,      ,       ,       ,      ,      , \dst, \src, \shiftbits
        load_add_store  v4.8b, v18.8h, v2.8b, v16.8h,       ,      ,      , \dst, \src, \shiftbits
        load_add_store  v5.8b, v19.8h, v3.8b, v17.8h, v16.8h, v2.8b,      , \dst, \src, \shiftbits
        load_add_store  v6.8b, v20.8h, v4.8b, v18.8h, v17.8h, v3.8b, v2.8b, \dst, \src, \shiftbits
        load_add_store  v7.8b, v21.8h, v5.8b, v19.8h, v18.8h, v4.8b, v3.8b, \dst, \src, \shiftbits
        load_add_store  v2.8b, v22.8h, v6.8b, v20.8h, v19.8h, v5.8b, v4.8b, \dst, \src, \shiftbits
        load_add_store  v3.8b, v23.8h, v7.8b, v21.8h, v20.8h, v6.8b, v5.8b, \dst, \src, \shiftbits
        load_add_store       ,       , v2.8b, v22.8h, v21.8h, v7.8b, v6.8b, \dst, \src, \shiftbits
        load_add_store       ,       , v3.8b, v23.8h, v22.8h, v2.8b, v7.8b, \dst, \src, \shiftbits
        load_add_store       ,       ,      ,       , v23.8h, v3.8b, v2.8b, \dst, \src, \shiftbits
        load_add_store       ,       ,      ,       ,       ,      , v3.8b, \dst, \src, \shiftbits
.endm
.macro load_add_store_8x4 dst, src
        mov             \src, \dst
        load_add_store  v2.8b, v16.8h,      ,       ,       ,      ,      , \dst, \src
        load_add_store  v3.8b, v17.8h,      ,       ,       ,      ,      , \dst, \src
        load_add_store  v4.8b, v18.8h, v2.8b, v16.8h,       ,      ,      , \dst, \src
        load_add_store  v5.8b, v19.8h, v3.8b, v17.8h, v16.8h, v2.8b,      , \dst, \src
        load_add_store       ,       , v4.8b, v18.8h, v17.8h, v3.8b, v2.8b, \dst, \src
        load_add_store       ,       , v5.8b, v19.8h, v18.8h, v4.8b, v3.8b, \dst, \src
        load_add_store       ,       ,      ,       , v19.8h, v5.8b, v4.8b, \dst, \src
        load_add_store       ,       ,      ,       ,       ,      , v5.8b, \dst, \src
.endm
.macro load_add_store4 load, inssrc, insdst, shift, addsrc, adddst, narrowsrc, narrowdst, store, dst, src
.ifnb \load
        ld1             {\load}[0],  [\src], x1
.endif
.ifnb \inssrc
        ins             \insdst\().d[1],   \inssrc\().d[0]
.endif
.ifnb \shift
        srshr           \shift,  \shift,  #4
.endif
.ifnb \load
        ld1             {\load}[1],  [\src], x1
.endif
.ifnb \addsrc
        uaddw           \adddst, \adddst, \addsrc
.endif
.ifnb \store
        st1             {\store}[0],  [\dst], x1
.endif
.ifnb \narrowsrc
        sqxtun          \narrowdst, \narrowsrc
.endif
.ifnb \store
        st1             {\store}[1],  [\dst], x1
.endif
.endm
.macro load_add_store_4x16 dst, src
        mov             \src, \dst
        load_add_store4 v0.s, v17, v16,       ,      ,       ,       ,      ,     , \dst, \src
        load_add_store4 v1.s, v19, v18,       ,      ,       ,       ,      ,     , \dst, \src
        load_add_store4 v2.s, v21, v20, v16.8h,      ,       ,       ,      ,     , \dst, \src
        load_add_store4 v3.s, v23, v22, v18.8h, v0.8b, v16.8h,       ,      ,     , \dst, \src
        load_add_store4 v4.s, v25, v24, v20.8h, v1.8b, v18.8h, v16.8h, v0.8b,     , \dst, \src
        load_add_store4 v5.s, v27, v26, v22.8h, v2.8b, v20.8h, v18.8h, v1.8b, v0.s, \dst, \src
        load_add_store4 v6.s, v29, v28, v24.8h, v3.8b, v22.8h, v20.8h, v2.8b, v1.s, \dst, \src
        load_add_store4 v7.s, v31, v30, v26.8h, v4.8b, v24.8h, v22.8h, v3.8b, v2.s, \dst, \src
        load_add_store4     ,    ,    , v28.8h, v5.8b, v26.8h, v24.8h, v4.8b, v3.s, \dst, \src
        load_add_store4     ,    ,    , v30.8h, v6.8b, v28.8h, v26.8h, v5.8b, v4.s, \dst, \src
        load_add_store4     ,    ,    ,       , v7.8b, v30.8h, v28.8h, v6.8b, v5.s, \dst, \src
        load_add_store4     ,    ,    ,       ,      ,       , v30.8h, v7.8b, v6.s, \dst, \src
        load_add_store4     ,    ,    ,       ,      ,       ,       ,      , v7.s, \dst, \src
.endm
.macro load_add_store_4x8 dst, src
        mov             \src, \dst
        load_add_store4 v0.s, v17, v16,       ,      ,       ,       ,      ,     , \dst, \src
        load_add_store4 v1.s, v19, v18,       ,      ,       ,       ,      ,     , \dst, \src
        load_add_store4 v2.s, v21, v20, v16.8h,      ,       ,       ,      ,     , \dst, \src
        load_add_store4 v3.s, v23, v22, v18.8h, v0.8b, v16.8h,       ,      ,     , \dst, \src
        load_add_store4     ,    ,    , v20.8h, v1.8b, v18.8h, v16.8h, v0.8b,     , \dst, \src
        load_add_store4     ,    ,    , v22.8h, v2.8b, v20.8h, v18.8h, v1.8b, v0.s, \dst, \src
        load_add_store4     ,    ,    ,       , v3.8b, v22.8h, v20.8h, v2.8b, v1.s, \dst, \src
        load_add_store4     ,    ,    ,       ,      ,       , v22.8h, v3.8b, v2.s, \dst, \src
        load_add_store4     ,    ,    ,       ,      ,       ,       ,      , v3.s, \dst, \src
.endm

.macro idct_dc w, h, shift
        cbnz            w3,  1f
        mov             w16, #2896*8
        ld1r            {v16.8h}, [x2]
        dup             v0.4h,   w16
        sqrdmulh        v16.8h,  v16.8h,  v0.h[0]
        strh            wzr, [x2]
.if (\w == 2*\h) || (2*\w == \h)
        sqrdmulh        v16.8h,  v16.8h,  v0.h[0]
.endif
.if \shift > 0
        srshr           v16.8h,  v16.8h,  #\shift
.endif
        sqrdmulh        v16.8h,  v16.8h,  v0.h[0]
        srshr           v16.8h,  v16.8h,  #4
        mov             w4,  #\h
        b               idct_dc_w\w\()_neon
1:
.endm

function idct_dc_w4_neon
1:
        ld1             {v0.s}[0], [x0], x1
        ld1             {v0.s}[1], [x0], x1
        ld1             {v1.s}[0], [x0], x1
        ld1             {v1.s}[1], [x0], x1
        subs            w4,  w4,  #4
        sub             x0,  x0,  x1, lsl #2
        uaddw           v0.8h,   v16.8h,  v0.8b
        sqxtun          v0.8b,   v0.8h
        uaddw           v1.8h,   v16.8h,  v1.8b
        st1             {v0.s}[0], [x0], x1
        sqxtun          v1.8b,   v1.8h
        st1             {v0.s}[1], [x0], x1
        st1             {v1.s}[0], [x0], x1
        st1             {v1.s}[1], [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w8_neon
1:
        ld1             {v0.8b}, [x0], x1
        ld1             {v1.8b}, [x0], x1
        ld1             {v2.8b}, [x0], x1
        uaddw           v20.8h,  v16.8h, v0.8b
        ld1             {v3.8b}, [x0], x1
        sub             x0,  x0,  x1, lsl #2
        subs            w4,  w4,  #4
        uaddw           v21.8h,  v16.8h, v1.8b
        sqxtun          v0.8b,   v20.8h
        uaddw           v22.8h,  v16.8h, v2.8b
        sqxtun          v1.8b,   v21.8h
        uaddw           v23.8h,  v16.8h, v3.8b
        st1             {v0.8b}, [x0], x1
        sqxtun          v2.8b,   v22.8h
        st1             {v1.8b}, [x0], x1
        sqxtun          v3.8b,   v23.8h
        st1             {v2.8b}, [x0], x1
        st1             {v3.8b}, [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w16_neon
1:
        ld1             {v0.16b}, [x0], x1
        ld1             {v1.16b}, [x0], x1
        ld1             {v2.16b}, [x0], x1
        subs            w4,  w4,  #4
        uaddw           v20.8h,  v16.8h, v0.8b
        uaddw2          v21.8h,  v16.8h, v0.16b
        ld1             {v3.16b}, [x0], x1
        uaddw           v22.8h,  v16.8h, v1.8b
        uaddw2          v23.8h,  v16.8h, v1.16b
        sub             x0,  x0,  x1, lsl #2
        uaddw           v24.8h,  v16.8h, v2.8b
        uaddw2          v25.8h,  v16.8h, v2.16b
        sqxtun          v0.8b,   v20.8h
        sqxtun2         v0.16b,  v21.8h
        uaddw           v26.8h,  v16.8h, v3.8b
        uaddw2          v27.8h,  v16.8h, v3.16b
        sqxtun          v1.8b,   v22.8h
        sqxtun2         v1.16b,  v23.8h
        sqxtun          v2.8b,   v24.8h
        sqxtun2         v2.16b,  v25.8h
        st1             {v0.16b}, [x0], x1
        sqxtun          v3.8b,   v26.8h
        sqxtun2         v3.16b,  v27.8h
        st1             {v1.16b}, [x0], x1
        st1             {v2.16b}, [x0], x1
        st1             {v3.16b}, [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w32_neon
1:
        ld1             {v0.16b, v1.16b},  [x0], x1
        subs            w4,  w4,  #2
        uaddw           v20.8h,  v16.8h, v0.8b
        uaddw2          v21.8h,  v16.8h, v0.16b
        ld1             {v2.16b, v3.16b},  [x0]
        uaddw           v22.8h,  v16.8h, v1.8b
        uaddw2          v23.8h,  v16.8h, v1.16b
        sub             x0,  x0,  x1
        uaddw           v24.8h,  v16.8h, v2.8b
        uaddw2          v25.8h,  v16.8h, v2.16b
        sqxtun          v0.8b,   v20.8h
        sqxtun2         v0.16b,  v21.8h
        uaddw           v26.8h,  v16.8h, v3.8b
        uaddw2          v27.8h,  v16.8h, v3.16b
        sqxtun          v1.8b,   v22.8h
        sqxtun2         v1.16b,  v23.8h
        sqxtun          v2.8b,   v24.8h
        sqxtun2         v2.16b,  v25.8h
        st1             {v0.16b, v1.16b},  [x0], x1
        sqxtun          v3.8b,   v26.8h
        sqxtun2         v3.16b,  v27.8h
        st1             {v2.16b, v3.16b},  [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w64_neon
1:
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b},  [x0]
        subs            w4,  w4,  #1
        uaddw           v20.8h,  v16.8h, v0.8b
        uaddw2          v21.8h,  v16.8h, v0.16b
        uaddw           v22.8h,  v16.8h, v1.8b
        uaddw2          v23.8h,  v16.8h, v1.16b
        uaddw           v24.8h,  v16.8h, v2.8b
        uaddw2          v25.8h,  v16.8h, v2.16b
        sqxtun          v0.8b,   v20.8h
        sqxtun2         v0.16b,  v21.8h
        uaddw           v26.8h,  v16.8h, v3.8b
        uaddw2          v27.8h,  v16.8h, v3.16b
        sqxtun          v1.8b,   v22.8h
        sqxtun2         v1.16b,  v23.8h
        sqxtun          v2.8b,   v24.8h
        sqxtun2         v2.16b,  v25.8h
        sqxtun          v3.8b,   v26.8h
        sqxtun2         v3.16b,  v27.8h
        st1             {v0.16b, v1.16b, v2.16b, v3.16b},  [x0], x1
        b.gt            1b
        ret
endfunc

.macro iwht4
        add             v16.4h,  v16.4h,  v17.4h
        sub             v21.4h,  v18.4h,  v19.4h
        sub             v20.4h,  v16.4h,  v21.4h
        sshr            v20.4h,  v20.4h,  #1
        sub             v18.4h,  v20.4h,  v17.4h
        sub             v17.4h,  v20.4h,  v19.4h
        add             v19.4h,  v21.4h,  v18.4h
        sub             v16.4h,  v16.4h,  v17.4h
.endm

.macro idct_4 r0, r1, r2, r3, sz
        smull_smlal     v6,  v7,  \r1, \r3, v0.h[3], v0.h[2], \sz
        smull_smlsl     v4,  v5,  \r1, \r3, v0.h[2], v0.h[3], \sz
        smull_smlal     v2,  v3,  \r0, \r2, v0.h[0], v0.h[0], \sz
        sqrshrn_sz      v6,  v6,  v7,  #12, \sz
        sqrshrn_sz      v7,  v4,  v5,  #12, \sz
        smull_smlsl     v4,  v5,  \r0, \r2, v0.h[0], v0.h[0], \sz
        sqrshrn_sz      v2,  v2,  v3,  #12, \sz
        sqrshrn_sz      v3,  v4,  v5,  #12, \sz
        sqadd           \r0\sz,  v2\sz,   v6\sz
        sqsub           \r3\sz,  v2\sz,   v6\sz
        sqadd           \r1\sz,  v3\sz,   v7\sz
        sqsub           \r2\sz,  v3\sz,   v7\sz
.endm

function inv_dct_4h_x4_neon, export=1
        movrel          x16, idct_coeffs
        ld1             {v0.4h}, [x16]
        idct_4          v16, v17, v18, v19, .4h
        ret
endfunc

function inv_dct_8h_x4_neon, export=1
        movrel          x16, idct_coeffs
        ld1             {v0.4h}, [x16]
        idct_4          v16, v17, v18, v19, .8h
        ret
endfunc

.macro iadst_4x4 o0, o1, o2, o3
        movrel          x16, iadst4_coeffs
        ld1             {v0.8h}, [x16]

        ssubl           v3.4s,   v16.4h,  v18.4h
        smull           v4.4s,   v16.4h,  v0.h[0]
        smlal           v4.4s,   v18.4h,  v0.h[1]
        smlal           v4.4s,   v19.4h,  v0.h[2]
        smull           v7.4s,   v17.4h,  v0.h[3]
        saddw           v3.4s,   v3.4s,   v19.4h
        smull           v5.4s,   v16.4h,  v0.h[2]
        smlsl           v5.4s,   v18.4h,  v0.h[0]
        smlsl           v5.4s,   v19.4h,  v0.h[1]

        add             \o3\().4s, v4.4s,     v5.4s
        mul             \o2\().4s, v3.4s,     v0.s[2]
        add             \o0\().4s, v4.4s,     v7.4s
        add             \o1\().4s, v5.4s,     v7.4s
        sub             \o3\().4s, \o3\().4s, v7.4s

        sqrshrn         \o0\().4h, \o0\().4s, #12
        sqrshrn         \o2\().4h, \o2\().4s, #12
        sqrshrn         \o1\().4h, \o1\().4s, #12
        sqrshrn         \o3\().4h, \o3\().4s, #12
.endm

function inv_adst_4h_x4_neon, export=1
        iadst_4x4       v16, v17, v18, v19
        ret
endfunc

function inv_flipadst_4h_x4_neon, export=1
        iadst_4x4       v19, v18, v17, v16
        ret
endfunc

.macro iadst_8x4 o0, o1, o2, o3
        movrel          x16, iadst4_coeffs
        ld1             {v0.8h}, [x16]

        ssubl           v2.4s,   v16.4h,  v18.4h
        ssubl2          v3.4s,   v16.8h,  v18.8h
        smull           v4.4s,   v16.4h,  v0.h[0]
        smlal           v4.4s,   v18.4h,  v0.h[1]
        smlal           v4.4s,   v19.4h,  v0.h[2]
        smull2          v5.4s,   v16.8h,  v0.h[0]
        smlal2          v5.4s,   v18.8h,  v0.h[1]
        smlal2          v5.4s,   v19.8h,  v0.h[2]
        saddw           v2.4s,   v2.4s,   v19.4h
        saddw2          v3.4s,   v3.4s,   v19.8h
        smull           v6.4s,   v16.4h,  v0.h[2]
        smlsl           v6.4s,   v18.4h,  v0.h[0]
        smlsl           v6.4s,   v19.4h,  v0.h[1]
        smull2          v7.4s,   v16.8h,  v0.h[2]
        smlsl2          v7.4s,   v18.8h,  v0.h[0]
        smlsl2          v7.4s,   v19.8h,  v0.h[1]

        mul             v18.4s,  v2.4s,   v0.s[2]
        mul             v19.4s,  v3.4s,   v0.s[2]

        smull           v2.4s,   v17.4h,  v0.h[3]
        smull2          v3.4s,   v17.8h,  v0.h[3]

        add             v16.4s,  v4.4s,   v2.4s // out0
        add             v17.4s,  v5.4s,   v3.4s

        add             v4.4s,   v4.4s,   v6.4s // out3
        add             v5.4s,   v5.4s,   v7.4s

        add             v6.4s,   v6.4s,   v2.4s // out1
        add             v7.4s,   v7.4s,   v3.4s

        sub             v4.4s,   v4.4s,   v2.4s // out3
        sub             v5.4s,   v5.4s,   v3.4s

        sqrshrn         v18.4h,  v18.4s, #12
        sqrshrn2        v18.8h,  v19.4s, #12

        sqrshrn         \o0\().4h, v16.4s, #12
        sqrshrn2        \o0\().8h, v17.4s, #12

.ifc \o2, v17
        mov             v17.16b,   v18.16b
.endif

        sqrshrn         \o1\().4h, v6.4s,  #12
        sqrshrn2        \o1\().8h, v7.4s,  #12

        sqrshrn         \o3\().4h, v4.4s,  #12
        sqrshrn2        \o3\().8h, v5.4s,  #12
.endm

function inv_adst_8h_x4_neon, export=1
        iadst_8x4       v16, v17, v18, v19
        ret
endfunc

function inv_flipadst_8h_x4_neon, export=1
        iadst_8x4       v19, v18, v17, v16
        ret
endfunc

function inv_identity_4h_x4_neon, export=1
        mov             w16, #(5793-4096)*8
        dup             v0.4h,   w16
        sqrdmulh        v4.4h,   v16.4h,  v0.h[0]
        sqrdmulh        v5.4h,   v17.4h,  v0.h[0]
        sqrdmulh        v6.4h,   v18.4h,  v0.h[0]
        sqrdmulh        v7.4h,   v19.4h,  v0.h[0]
        sqadd           v16.4h,  v16.4h,  v4.4h
        sqadd           v17.4h,  v17.4h,  v5.4h
        sqadd           v18.4h,  v18.4h,  v6.4h
        sqadd           v19.4h,  v19.4h,  v7.4h
        ret
endfunc

function inv_identity_8h_x4_neon, export=1
        mov             w16, #(5793-4096)*8
        dup             v0.4h,   w16
        sqrdmulh        v4.8h,   v16.8h,  v0.h[0]
        sqrdmulh        v5.8h,   v17.8h,  v0.h[0]
        sqrdmulh        v6.8h,   v18.8h,  v0.h[0]
        sqrdmulh        v7.8h,   v19.8h,  v0.h[0]
        sqadd           v16.8h,  v16.8h,  v4.8h
        sqadd           v17.8h,  v17.8h,  v5.8h
        sqadd           v18.8h,  v18.8h,  v6.8h
        sqadd           v19.8h,  v19.8h,  v7.8h
        ret
endfunc

.macro identity_8x4_shift1 r0, r1, r2, r3, c
.irp i, \r0\().8h, \r1\().8h, \r2\().8h, \r3\().8h
        sqrdmulh        v2.8h,  \i,  \c
        srhadd          \i,     \i,  v2.8h
.endr
.endm

function inv_txfm_add_wht_wht_4x4_8bpc_neon, export=1
        mov             x15, x30
        movi            v31.8h,  #0
        ld1             {v16.4h,v17.4h,v18.4h,v19.4h}, [x2]
        st1             {v31.8h}, [x2], #16

        sshr            v16.4h,  v16.4h,  #2
        sshr            v17.4h,  v17.4h,  #2
        sshr            v18.4h,  v18.4h,  #2
        sshr            v19.4h,  v19.4h,  #2

        iwht4

        st1             {v31.8h}, [x2], #16
        transpose_4x4h  v16, v17, v18, v19, v20, v21, v22, v23

        iwht4

        ld1             {v0.s}[0], [x0], x1
        ld1             {v0.s}[1], [x0], x1
        ins             v16.d[1], v17.d[0]
        ins             v18.d[1], v19.d[0]
        ld1             {v1.s}[0], [x0], x1
        ld1             {v1.s}[1], [x0], x1

        b               L(itx_4x4_end)
endfunc

function inv_txfm_add_4x4_neon
        movi            v31.8h,  #0
        ld1             {v16.4h,v17.4h,v18.4h,v19.4h}, [x2]
        st1             {v31.8h}, [x2], #16

        blr             x4

        st1             {v31.8h}, [x2], #16
        transpose_4x4h  v16, v17, v18, v19, v20, v21, v22, v23

        blr             x5

        ld1             {v0.s}[0], [x0], x1
        ld1             {v0.s}[1], [x0], x1
        ins             v16.d[1], v17.d[0]
        ins             v18.d[1], v19.d[0]
        ld1             {v1.s}[0], [x0], x1
        ld1             {v1.s}[1], [x0], x1
        srshr           v16.8h,  v16.8h,  #4
        srshr           v18.8h,  v18.8h,  #4

L(itx_4x4_end):
        sub             x0,  x0,  x1, lsl #2
        uaddw           v16.8h,  v16.8h,  v0.8b
        sqxtun          v0.8b,   v16.8h
        uaddw           v18.8h,  v18.8h,  v1.8b
        st1             {v0.s}[0], [x0], x1
        sqxtun          v1.8b,   v18.8h
        st1             {v0.s}[1], [x0], x1
        st1             {v1.s}[0], [x0], x1
        st1             {v1.s}[1], [x0], x1

        ret             x15
endfunc

.macro def_fn_4x4 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_4x4_8bpc_neon, export=1
        mov             x15, x30

.ifc \txfm1\()_\txfm2, dct_dct
        cbnz            w3,  1f
        mov             w16, #2896*8
        ld1r            {v16.8h}, [x2]
        dup             v4.8h,   w16
        strh            wzr, [x2]
        sqrdmulh        v16.8h,  v16.8h,  v4.h[0]
        ld1             {v0.s}[0], [x0], x1
        sqrdmulh        v20.8h,  v16.8h,  v4.h[0]
        ld1             {v0.s}[1], [x0], x1
        srshr           v16.8h,  v20.8h,  #4
        ld1             {v1.s}[0], [x0], x1
        srshr           v18.8h,  v20.8h,  #4
        ld1             {v1.s}[1], [x0], x1
        b               L(itx_4x4_end)
1:
.endif
        adr             x4,  inv_\txfm1\()_4h_x4_neon
        adr             x5,  inv_\txfm2\()_4h_x4_neon
        b               inv_txfm_add_4x4_neon
endfunc
.endm

def_fn_4x4 dct, dct
def_fn_4x4 identity, identity
def_fn_4x4 dct, adst
def_fn_4x4 dct, flipadst
def_fn_4x4 dct, identity
def_fn_4x4 adst, dct
def_fn_4x4 adst, adst
def_fn_4x4 adst, flipadst
def_fn_4x4 flipadst, dct
def_fn_4x4 flipadst, adst
def_fn_4x4 flipadst, flipadst
def_fn_4x4 identity, dct

def_fn_4x4 adst, identity
def_fn_4x4 flipadst, identity
def_fn_4x4 identity, adst
def_fn_4x4 identity, flipadst

.macro idct_8 r0, r1, r2, r3, r4, r5, r6, r7, sz, szb
        idct_4          \r0, \r2, \r4, \r6, \sz

        smull_smlsl     v2,  v3,  \r1, \r7, v0.h[4], v0.h[5], \sz // -> t4a
        smull_smlal     v4,  v5,  \r1, \r7, v0.h[5], v0.h[4], \sz // -> t7a
        smull_smlsl     v6,  v7,  \r5, \r3, v0.h[6], v0.h[7], \sz // -> t5a
        sqrshrn_sz      \r1, v2,  v3,  #12, \sz                   // t4a
        sqrshrn_sz      \r7, v4,  v5,  #12, \sz                   // t7a
        smull_smlal     v2,  v3,  \r5, \r3, v0.h[7], v0.h[6], \sz // -> t6a
        sqrshrn_sz      \r3, v6,  v7,  #12, \sz                   // t5a
        sqrshrn_sz      \r5, v2,  v3,  #12, \sz                   // t6a

        sqadd           v2\sz,   \r1\sz,  \r3\sz // t4
        sqsub           \r1\sz,  \r1\sz,  \r3\sz // t5a
        sqadd           v3\sz,   \r7\sz,  \r5\sz // t7
        sqsub           \r3\sz,  \r7\sz,  \r5\sz // t6a

        smull_smlsl     v4,  v5,  \r3, \r1, v0.h[0], v0.h[0], \sz // -> t5
        smull_smlal     v6,  v7,  \r3, \r1, v0.h[0], v0.h[0], \sz // -> t6
        sqrshrn_sz      v4,  v4,  v5,  #12, \sz // t5
        sqrshrn_sz      v5,  v6,  v7,  #12, \sz // t6

        sqsub           \r7\sz,  \r0\sz,  v3\sz // out7
        sqadd           \r0\sz,  \r0\sz,  v3\sz // out0
        sqadd           \r1\sz,  \r2\sz,  v5\sz // out1
        sqsub           v6\sz,   \r2\sz,  v5\sz // out6
        sqadd           \r2\sz,  \r4\sz,  v4\sz // out2
        sqsub           \r5\sz,  \r4\sz,  v4\sz // out5
        sqadd           \r3\sz,  \r6\sz,  v2\sz // out3
        sqsub           \r4\sz,  \r6\sz,  v2\sz // out4
        mov             \r6\szb, v6\szb         // out6
.endm

function inv_dct_8h_x8_neon, export=1
        movrel          x16, idct_coeffs
        ld1             {v0.8h}, [x16]
        idct_8          v16, v17, v18, v19, v20, v21, v22, v23, .8h, .16b
        ret
endfunc

function inv_dct_4h_x8_neon, export=1
        movrel          x16, idct_coeffs
        ld1             {v0.8h}, [x16]
        idct_8          v16, v17, v18, v19, v20, v21, v22, v23, .4h, .8b
        ret
endfunc

.macro iadst_8 o0, o1, o2, o3, o4, o5, o6, o7, sz
        movrel          x16, iadst8_coeffs
        ld1             {v0.8h, v1.8h}, [x16]

        smull_smlal     v2,  v3,  v23, v16, v0.h[0], v0.h[1], \sz
        smull_smlsl     v4,  v5,  v23, v16, v0.h[1], v0.h[0], \sz
        smull_smlal     v6,  v7,  v21, v18, v0.h[2], v0.h[3], \sz
        sqrshrn_sz      v16, v2,  v3,  #12, \sz  // t0a
        sqrshrn_sz      v23, v4,  v5,  #12, \sz  // t1a
        smull_smlsl     v2,  v3,  v21, v18, v0.h[3], v0.h[2], \sz
        smull_smlal     v4,  v5,  v19, v20, v0.h[4], v0.h[5], \sz
        sqrshrn_sz      v18, v6,  v7,  #12, \sz  // t2a
        sqrshrn_sz      v21, v2,  v3,  #12, \sz  // t3a
        smull_smlsl     v6,  v7,  v19, v20, v0.h[5], v0.h[4], \sz
        smull_smlal     v2,  v3,  v17, v22, v0.h[6], v0.h[7], \sz
        sqrshrn_sz      v20, v4,  v5,  #12, \sz  // t4a
        sqrshrn_sz      v19, v6,  v7,  #12, \sz  // t5a
        smull_smlsl     v4,  v5,  v17, v22, v0.h[7], v0.h[6], \sz
        sqrshrn_sz      v22, v2,  v3,  #12, \sz  // t6a
        sqrshrn_sz      v17, v4,  v5,  #12, \sz  // t7a

        sqadd           v2\sz,   v16\sz,  v20\sz // t0
        sqsub           v3\sz,   v16\sz,  v20\sz // t4
        sqadd           v4\sz,   v23\sz,  v19\sz // t1
        sqsub           v5\sz,   v23\sz,  v19\sz // t5
        sqadd           v6\sz,   v18\sz,  v22\sz // t2
        sqsub           v7\sz,   v18\sz,  v22\sz // t6
        sqadd           v18\sz,  v21\sz,  v17\sz // t3
        sqsub           v19\sz,  v21\sz,  v17\sz // t7

        smull_smlal     v16, v17, v3,  v5,  v1.h[3], v1.h[2], \sz
        smull_smlsl     v20, v21, v3,  v5,  v1.h[2], v1.h[3], \sz
        smull_smlsl     v22, v23, v19, v7,  v1.h[3], v1.h[2], \sz

        sqrshrn_sz      v3,  v16, v17, #12, \sz  // t4a
        sqrshrn_sz      v5,  v20, v21, #12, \sz  // t5a

        smull_smlal     v16, v17, v19, v7,  v1.h[2], v1.h[3], \sz

        sqrshrn_sz      v7,  v22, v23, #12, \sz  // t6a
        sqrshrn_sz      v19, v16, v17, #12, \sz  // t7a

        sqadd           \o0\()\sz, v2\sz, v6\sz  // out0
        sqsub           v2\sz,     v2\sz, v6\sz  // t2
        sqadd           \o7\()\sz, v4\sz, v18\sz // out7
        sqsub           v4\sz,     v4\sz, v18\sz // t3
        sqneg           \o7\()\sz, \o7\()\sz     // out7

        sqadd           \o1\()\sz, v3\sz, v7\sz  // out1
        sqsub           v3\sz,     v3\sz, v7\sz  // t6
        sqadd           \o6\()\sz, v5\sz, v19\sz // out6
        sqsub           v5\sz,     v5\sz, v19\sz // t7
        sqneg           \o1\()\sz, \o1\()\sz     // out1

        smull_smlal     v18, v19, v2,  v4,  v1.h[0], v1.h[0], \sz // -> out3 (v19 or v20)
        smull_smlsl     v6,  v7,  v2,  v4,  v1.h[0], v1.h[0], \sz // -> out4 (v20 or v19)
        smull_smlsl     v20, v21, v3,  v5,  v1.h[0], v1.h[0], \sz // -> out5 (v21 or v18)
        sqrshrn_sz      v2,  v18, v19, #12, \sz // out3
        smull_smlal     v18, v19, v3,  v5,  v1.h[0], v1.h[0], \sz // -> out2 (v18 or v21)
        sqrshrn_sz      v3,  v20, v21, #12, \sz // out5
        sqrshrn_sz      \o2, v18, v19, #12, \sz // out2 (v18 or v21)
        sqrshrn_sz      \o4, v6,  v7,  #12, \sz // out4 (v20 or v19)

        sqneg           \o3\()\sz, v2\sz     // out3
        sqneg           \o5\()\sz, v3\sz     // out5
.endm

function inv_adst_8h_x8_neon, export=1
        iadst_8         v16, v17, v18, v19, v20, v21, v22, v23, .8h
        ret
endfunc

function inv_flipadst_8h_x8_neon, export=1
        iadst_8         v23, v22, v21, v20, v19, v18, v17, v16, .8h
        ret
endfunc

function inv_adst_4h_x8_neon, export=1
        iadst_8         v16, v17, v18, v19, v20, v21, v22, v23, .4h
        ret
endfunc

function inv_flipadst_4h_x8_neon, export=1
        iadst_8         v23, v22, v21, v20, v19, v18, v17, v16, .4h
        ret
endfunc

function inv_identity_8h_x8_neon, export=1
        sqshl           v16.8h,  v16.8h,  #1
        sqshl           v17.8h,  v17.8h,  #1
        sqshl           v18.8h,  v18.8h,  #1
        sqshl           v19.8h,  v19.8h,  #1
        sqshl           v20.8h,  v20.8h,  #1
        sqshl           v21.8h,  v21.8h,  #1
        sqshl           v22.8h,  v22.8h,  #1
        sqshl           v23.8h,  v23.8h,  #1
        ret
endfunc

function inv_identity_4h_x8_neon, export=1
        sqshl           v16.4h,  v16.4h,  #1
        sqshl           v17.4h,  v17.4h,  #1
        sqshl           v18.4h,  v18.4h,  #1
        sqshl           v19.4h,  v19.4h,  #1
        sqshl           v20.4h,  v20.4h,  #1
        sqshl           v21.4h,  v21.4h,  #1
        sqshl           v22.4h,  v22.4h,  #1
        sqshl           v23.4h,  v23.4h,  #1
        ret
endfunc

.macro def_fn_8x8_base variant
function inv_txfm_\variant\()add_8x8_neon
        movi            v28.8h,  #0
        movi            v29.8h,  #0
        movi            v30.8h,  #0
        movi            v31.8h,  #0
        ld1             {v16.8h,v17.8h,v18.8h,v19.8h}, [x2]
        st1             {v28.8h,v29.8h,v30.8h,v31.8h}, [x2], #64
        ld1             {v20.8h,v21.8h,v22.8h,v23.8h}, [x2]
        st1             {v28.8h,v29.8h,v30.8h,v31.8h}, [x2]

.ifc \variant, identity_
        // The identity shl #1 and downshift srshr #1 cancel out
.else
        blr             x4

        srshr           v16.8h,  v16.8h,  #1
        srshr           v17.8h,  v17.8h,  #1
        srshr           v18.8h,  v18.8h,  #1
        srshr           v19.8h,  v19.8h,  #1
        srshr           v20.8h,  v20.8h,  #1
        srshr           v21.8h,  v21.8h,  #1
        srshr           v22.8h,  v22.8h,  #1
        srshr           v23.8h,  v23.8h,  #1
.endif

        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v24, v25

        blr             x5

        load_add_store_8x8 x0, x7
        ret             x15
endfunc
.endm

def_fn_8x8_base
def_fn_8x8_base identity_

.macro def_fn_8x8 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_8x8_8bpc_neon, export=1
        mov             x15, x30

.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         8,   8,   1
.endif
        adr             x5,  inv_\txfm2\()_8h_x8_neon
.ifc \txfm1, identity
        b               inv_txfm_identity_add_8x8_neon
.else
        adr             x4,  inv_\txfm1\()_8h_x8_neon
        b               inv_txfm_add_8x8_neon
.endif
endfunc
.endm

def_fn_8x8 dct, dct
def_fn_8x8 identity, identity
def_fn_8x8 dct, adst
def_fn_8x8 dct, flipadst
def_fn_8x8 dct, identity
def_fn_8x8 adst, dct
def_fn_8x8 adst, adst
def_fn_8x8 adst, flipadst
def_fn_8x8 flipadst, dct
def_fn_8x8 flipadst, adst
def_fn_8x8 flipadst, flipadst
def_fn_8x8 identity, dct
def_fn_8x8 adst, identity
def_fn_8x8 flipadst, identity
def_fn_8x8 identity, adst
def_fn_8x8 identity, flipadst

function inv_txfm_add_8x4_neon
        movi            v30.8h,  #0
        movi            v31.8h,  #0
        mov             w16, #2896*8
        dup             v0.4h,   w16
        ld1             {v16.4h,v17.4h,v18.4h,v19.4h}, [x2]
        st1             {v30.8h,v31.8h}, [x2], #32
        ld1             {v20.4h,v21.4h,v22.4h,v23.4h}, [x2]
        st1             {v30.8h,v31.8h}, [x2]

        scale_input     .4h, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23

        blr             x4

        transpose_4x4h  v16, v17, v18, v19, v4,  v5,  v6,  v7
        transpose_4x4h  v20, v21, v22, v23, v4,  v5,  v6,  v7
        ins             v16.d[1], v20.d[0]
        ins             v17.d[1], v21.d[0]
        ins             v18.d[1], v22.d[0]
        ins             v19.d[1], v23.d[0]

        blr             x5

        load_add_store_8x4 x0, x7
        ret             x15
endfunc

function inv_txfm_add_4x8_neon
        movi            v28.8h,  #0
        movi            v29.8h,  #0
        movi            v30.8h,  #0
        movi            v31.8h,  #0
        mov             w16, #2896*8
        dup             v0.4h,   w16
        ld1             {v16.8h,v17.8h,v18.8h,v19.8h}, [x2]
        st1             {v28.8h,v29.8h,v30.8h,v31.8h}, [x2]

        scale_input     .8h, v0.h[0], v16, v17, v18, v19

        blr             x4

        transpose_4x8h  v16, v17, v18, v19, v4,  v5,  v6,  v7
        ins             v20.d[0], v16.d[1]
        ins             v21.d[0], v17.d[1]
        ins             v22.d[0], v18.d[1]
        ins             v23.d[0], v19.d[1]

        blr             x5

        load_add_store_4x8 x0, x7
        ret             x15
endfunc

.macro def_fn_48 w, h, txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_8bpc_neon, export=1
        mov             x15, x30

.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  0
.endif
        adr             x4,  inv_\txfm1\()_\h\()h_x\w\()_neon
        adr             x5,  inv_\txfm2\()_\w\()h_x\h\()_neon
        b               inv_txfm_add_\w\()x\h\()_neon
endfunc
.endm

.macro def_fns_48 w, h
def_fn_48 \w, \h, dct, dct
def_fn_48 \w, \h, identity, identity
def_fn_48 \w, \h, dct, adst
def_fn_48 \w, \h, dct, flipadst
def_fn_48 \w, \h, dct, identity
def_fn_48 \w, \h, adst, dct
def_fn_48 \w, \h, adst, adst
def_fn_48 \w, \h, adst, flipadst
def_fn_48 \w, \h, flipadst, dct
def_fn_48 \w, \h, flipadst, adst
def_fn_48 \w, \h, flipadst, flipadst
def_fn_48 \w, \h, identity, dct
def_fn_48 \w, \h, adst, identity
def_fn_48 \w, \h, flipadst, identity
def_fn_48 \w, \h, identity, adst
def_fn_48 \w, \h, identity, flipadst
.endm

def_fns_48 4, 8
def_fns_48 8, 4


.macro idct_16 sz, szb
        idct_8          v16, v18, v20, v22, v24, v26, v28, v30, \sz, \szb

        smull_smlsl     v2,  v3,  v17, v31, v1.h[0], v1.h[1], \sz // -> t8a
        smull_smlal     v4,  v5,  v17, v31, v1.h[1], v1.h[0], \sz // -> t15a
        smull_smlsl     v6,  v7,  v25, v23, v1.h[2], v1.h[3], \sz // -> t9a
        sqrshrn_sz      v17, v2,  v3,  #12, \sz                   // t8a
        sqrshrn_sz      v31, v4,  v5,  #12, \sz                   // t15a
        smull_smlal     v2,  v3,  v25, v23, v1.h[3], v1.h[2], \sz // -> t14a
        smull_smlsl     v4,  v5,  v21, v27, v1.h[4], v1.h[5], \sz // -> t10a
        sqrshrn_sz      v23, v6,  v7,  #12, \sz                   // t9a
        sqrshrn_sz      v25, v2,  v3,  #12, \sz                   // t14a
        smull_smlal     v6,  v7,  v21, v27, v1.h[5], v1.h[4], \sz // -> t13a
        smull_smlsl     v2,  v3,  v29, v19, v1.h[6], v1.h[7], \sz // -> t11a
        sqrshrn_sz      v21, v4,  v5,  #12, \sz                   // t10a
        sqrshrn_sz      v27, v6,  v7,  #12, \sz                   // t13a
        smull_smlal     v4,  v5,  v29, v19, v1.h[7], v1.h[6], \sz // -> t12a
        sqrshrn_sz      v19, v2,  v3,  #12, \sz                   // t11a
        sqrshrn_sz      v29, v4,  v5,  #12, \sz                   // t12a

        sqsub           v2\sz,   v17\sz,  v23\sz  // t9
        sqadd           v17\sz,  v17\sz,  v23\sz  // t8
        sqsub           v3\sz,   v31\sz,  v25\sz  // t14
        sqadd           v31\sz,  v31\sz,  v25\sz  // t15
        sqsub           v23\sz,  v19\sz,  v21\sz  // t10
        sqadd           v19\sz,  v19\sz,  v21\sz  // t11
        sqadd           v25\sz,  v29\sz,  v27\sz  // t12
        sqsub           v29\sz,  v29\sz,  v27\sz  // t13

        smull_smlsl     v4,  v5,  v3,  v2,  v0.h[2], v0.h[3], \sz // -> t9a
        smull_smlal     v6,  v7,  v3,  v2,  v0.h[3], v0.h[2], \sz // -> t14a
        sqrshrn_sz      v21, v4,  v5,  #12, \sz                   // t9a
        sqrshrn_sz      v27, v6,  v7,  #12, \sz                   // t14a

        smull_smlsl     v4,  v5,  v29, v23, v0.h[2], v0.h[3], \sz // -> t13a
        smull_smlal     v6,  v7,  v29, v23, v0.h[3], v0.h[2], \sz // -> t10a
        sqrshrn_sz      v29, v4,  v5,  #12, \sz                   // t13a
        neg             v6.4s,   v6.4s
.ifc \sz, .8h
        neg             v7.4s,   v7.4s
.endif
        sqrshrn_sz      v23, v6,  v7,  #12, \sz                   // t10a

        sqsub           v2\sz,   v17\sz,  v19\sz  // t11a
        sqadd           v17\sz,  v17\sz,  v19\sz  // t8a
        sqsub           v3\sz,   v31\sz,  v25\sz  // t12a
        sqadd           v31\sz,  v31\sz,  v25\sz  // t15a
        sqadd           v19\sz,  v21\sz,  v23\sz  // t9
        sqsub           v21\sz,  v21\sz,  v23\sz  // t10
        sqsub           v25\sz,  v27\sz,  v29\sz  // t13
        sqadd           v27\sz,  v27\sz,  v29\sz  // t14

        smull_smlsl     v4,  v5,  v3,  v2,  v0.h[0], v0.h[0], \sz // -> t11
        smull_smlal     v6,  v7,  v3,  v2,  v0.h[0], v0.h[0], \sz // -> t12
        smull_smlsl     v2,  v3,  v25, v21, v0.h[0], v0.h[0], \sz // -> t10a

        sqrshrn_sz      v4,  v4,  v5,  #12, \sz   // t11
        sqrshrn_sz      v5,  v6,  v7,  #12, \sz   // t12
        smull_smlal     v6,  v7,  v25, v21, v0.h[0], v0.h[0], \sz // -> t13a
        sqrshrn_sz      v2,  v2,  v3,  #12, \sz   // t10a
        sqrshrn_sz      v3,  v6,  v7,  #12, \sz   // t13a

        sqadd           v6\sz,   v16\sz,  v31\sz  // out0
        sqsub           v31\sz,  v16\sz,  v31\sz  // out15
        mov             v16\szb, v6\szb
        sqadd           v23\sz,  v30\sz,  v17\sz  // out7
        sqsub           v7\sz,   v30\sz,  v17\sz  // out8
        sqadd           v17\sz,  v18\sz,  v27\sz  // out1
        sqsub           v30\sz,  v18\sz,  v27\sz  // out14
        sqadd           v18\sz,  v20\sz,  v3\sz   // out2
        sqsub           v29\sz,  v20\sz,  v3\sz   // out13
        sqadd           v3\sz,   v28\sz,  v19\sz  // out6
        sqsub           v25\sz,  v28\sz,  v19\sz  // out9
        sqadd           v19\sz,  v22\sz,  v5\sz   // out3
        sqsub           v28\sz,  v22\sz,  v5\sz   // out12
        sqadd           v20\sz,  v24\sz,  v4\sz   // out4
        sqsub           v27\sz,  v24\sz,  v4\sz   // out11
        sqadd           v21\sz,  v26\sz,  v2\sz   // out5
        sqsub           v26\sz,  v26\sz,  v2\sz   // out10
        mov             v24\szb, v7\szb
        mov             v22\szb, v3\szb
.endm

function inv_dct_8h_x16_neon, export=1
        movrel          x16, idct_coeffs
        ld1             {v0.8h, v1.8h}, [x16]
        idct_16         .8h, .16b
        ret
endfunc

function inv_dct_4h_x16_neon, export=1
        movrel          x16, idct_coeffs
        ld1             {v0.8h, v1.8h}, [x16]
        idct_16         .4h, .8b
        ret
endfunc

.macro iadst_16 o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15, sz, szb
        movrel          x16, iadst16_coeffs
        ld1             {v0.8h, v1.8h}, [x16]
        movrel          x16, idct_coeffs

        smull_smlal     v2,  v3,  v31, v16, v0.h[0], v0.h[1], \sz // -> t0
        smull_smlsl     v4,  v5,  v31, v16, v0.h[1], v0.h[0], \sz // -> t1
        smull_smlal     v6,  v7,  v29, v18, v0.h[2], v0.h[3], \sz // -> t2
        sqrshrn_sz      v16, v2,  v3,  #12, \sz   // t0
        sqrshrn_sz      v31, v4,  v5,  #12, \sz   // t1
        smull_smlsl     v2,  v3,  v29, v18, v0.h[3], v0.h[2], \sz // -> t3
        smull_smlal     v4,  v5,  v27, v20, v0.h[4], v0.h[5], \sz // -> t4
        sqrshrn_sz      v18, v6,  v7,  #12, \sz   // t2
        sqrshrn_sz      v29, v2,  v3,  #12, \sz   // t3
        smull_smlsl     v6,  v7,  v27, v20, v0.h[5], v0.h[4], \sz // -> t5
        smull_smlal     v2,  v3,  v25, v22, v0.h[6], v0.h[7], \sz // -> t6
        sqrshrn_sz      v20, v4,  v5,  #12, \sz   // t4
        sqrshrn_sz      v27, v6,  v7,  #12, \sz   // t5
        smull_smlsl     v4,  v5,  v25, v22, v0.h[7], v0.h[6], \sz // -> t7
        smull_smlal     v6,  v7,  v23, v24, v1.h[0], v1.h[1], \sz // -> t8
        sqrshrn_sz      v22, v2,  v3,  #12, \sz   // t6
        sqrshrn_sz      v25, v4,  v5,  #12, \sz   // t7
        smull_smlsl     v2,  v3,  v23, v24, v1.h[1], v1.h[0], \sz // -> t9
        smull_smlal     v4,  v5,  v21, v26, v1.h[2], v1.h[3], \sz // -> t10
        sqrshrn_sz      v23, v6,  v7,  #12, \sz   // t8
        sqrshrn_sz      v24, v2,  v3,  #12, \sz   // t9
        smull_smlsl     v6,  v7,  v21, v26, v1.h[3], v1.h[2], \sz // -> t11
        smull_smlal     v2,  v3,  v19, v28, v1.h[4], v1.h[5], \sz // -> t12
        sqrshrn_sz      v21, v4,  v5,  #12, \sz   // t10
        sqrshrn_sz      v26, v6,  v7,  #12, \sz   // t11
        smull_smlsl     v4,  v5,  v19, v28, v1.h[5], v1.h[4], \sz // -> t13
        smull_smlal     v6,  v7,  v17, v30, v1.h[6], v1.h[7], \sz // -> t14
        sqrshrn_sz      v19, v2,  v3,  #12, \sz   // t12
        sqrshrn_sz      v28, v4,  v5,  #12, \sz   // t13
        smull_smlsl     v2,  v3,  v17, v30, v1.h[7], v1.h[6], \sz // -> t15
        sqrshrn_sz      v17, v6,  v7,  #12, \sz   // t14
        sqrshrn_sz      v30, v2,  v3,  #12, \sz   // t15

        ld1             {v0.8h}, [x16]

        sqsub           v2\sz,   v16\sz,  v23\sz // t8a
        sqadd           v16\sz,  v16\sz,  v23\sz // t0a
        sqsub           v3\sz,   v31\sz,  v24\sz // t9a
        sqadd           v31\sz,  v31\sz,  v24\sz // t1a
        sqadd           v23\sz,  v18\sz,  v21\sz // t2a
        sqsub           v18\sz,  v18\sz,  v21\sz // t10a
        sqadd           v24\sz,  v29\sz,  v26\sz // t3a
        sqsub           v29\sz,  v29\sz,  v26\sz // t11a
        sqadd           v21\sz,  v20\sz,  v19\sz // t4a
        sqsub           v20\sz,  v20\sz,  v19\sz // t12a
        sqadd           v26\sz,  v27\sz,  v28\sz // t5a
        sqsub           v27\sz,  v27\sz,  v28\sz // t13a
        sqadd           v19\sz,  v22\sz,  v17\sz // t6a
        sqsub           v22\sz,  v22\sz,  v17\sz // t14a
        sqadd           v28\sz,  v25\sz,  v30\sz // t7a
        sqsub           v25\sz,  v25\sz,  v30\sz // t15a

        smull_smlal     v4,  v5,  v2,  v3,  v0.h[5], v0.h[4], \sz // -> t8
        smull_smlsl     v6,  v7,  v2,  v3,  v0.h[4], v0.h[5], \sz // -> t9
        smull_smlal     v2,  v3,  v18, v29, v0.h[7], v0.h[6], \sz // -> t10
        sqrshrn_sz      v17, v4,  v5,  #12, \sz  // t8
        sqrshrn_sz      v30, v6,  v7,  #12, \sz  // t9
        smull_smlsl     v4,  v5,  v18, v29, v0.h[6], v0.h[7], \sz // -> t11
        smull_smlsl     v6,  v7,  v27, v20, v0.h[5], v0.h[4], \sz // -> t12
        sqrshrn_sz      v18, v2,  v3,  #12, \sz  // t10
        sqrshrn_sz      v29, v4,  v5,  #12, \sz  // t11
        smull_smlal     v2,  v3,  v27, v20, v0.h[4], v0.h[5], \sz // -> t13
        smull_smlsl     v4,  v5,  v25, v22, v0.h[7], v0.h[6], \sz // -> t14
        sqrshrn_sz      v27, v6,  v7,  #12, \sz  // t12
        sqrshrn_sz      v20, v2,  v3,  #12, \sz  // t13
        smull_smlal     v6,  v7,  v25, v22, v0.h[6], v0.h[7], \sz // -> t15
        sqrshrn_sz      v25, v4,  v5,  #12, \sz  // t14
        sqrshrn_sz      v22, v6,  v7,  #12, \sz  // t15

        sqsub           v2\sz,   v16\sz,  v21\sz // t4
        sqadd           v16\sz,  v16\sz,  v21\sz // t0
        sqsub           v3\sz,   v31\sz,  v26\sz // t5
        sqadd           v31\sz,  v31\sz,  v26\sz // t1
        sqadd           v21\sz,  v23\sz,  v19\sz // t2
        sqsub           v23\sz,  v23\sz,  v19\sz // t6
        sqadd           v26\sz,  v24\sz,  v28\sz // t3
        sqsub           v24\sz,  v24\sz,  v28\sz // t7
        sqadd           v19\sz,  v17\sz,  v27\sz // t8a
        sqsub           v17\sz,  v17\sz,  v27\sz // t12a
        sqadd           v28\sz,  v30\sz,  v20\sz // t9a
        sqsub           v30\sz,  v30\sz,  v20\sz // t13a
        sqadd           v27\sz,  v18\sz,  v25\sz // t10a
        sqsub           v18\sz,  v18\sz,  v25\sz // t14a
        sqadd           v20\sz,  v29\sz,  v22\sz // t11a
        sqsub           v29\sz,  v29\sz,  v22\sz // t15a

        smull_smlal     v4,  v5,  v2,  v3,  v0.h[3], v0.h[2], \sz // -> t4a
        smull_smlsl     v6,  v7,  v2,  v3,  v0.h[2], v0.h[3], \sz // -> t5a
        smull_smlsl     v2,  v3,  v24, v23, v0.h[3], v0.h[2], \sz // -> t6a
        sqrshrn_sz      v22, v4,  v5,  #12, \sz // t4a
        sqrshrn_sz      v25, v6,  v7,  #12, \sz // t5a
        smull_smlal     v4,  v5,  v24, v23, v0.h[2], v0.h[3], \sz // -> t7a
        smull_smlal     v6,  v7,  v17, v30, v0.h[3], v0.h[2], \sz // -> t12
        sqrshrn_sz      v24, v2,  v3,  #12, \sz // t6a
        sqrshrn_sz      v23, v4,  v5,  #12, \sz // t7a
        smull_smlsl     v2,  v3,  v17, v30, v0.h[2], v0.h[3], \sz // -> t13
        smull_smlsl     v4,  v5,  v29, v18, v0.h[3], v0.h[2], \sz // -> t14
        sqrshrn_sz      v17, v6,  v7,  #12, \sz // t12
        smull_smlal     v6,  v7,  v29, v18, v0.h[2], v0.h[3], \sz // -> t15
        sqrshrn_sz      v29, v2,  v3,  #12, \sz // t13
        sqrshrn_sz      v30, v4,  v5,  #12, \sz // t14
        sqrshrn_sz      v18, v6,  v7,  #12, \sz // t15

        sqsub           v2\sz,   v16\sz,  v21\sz // t2a
.ifc \o0, v16
        sqadd           \o0\sz,  v16\sz,  v21\sz // out0
        sqsub           v21\sz,  v31\sz,  v26\sz // t3a
        sqadd           \o15\sz, v31\sz,  v26\sz // out15
.else
        sqadd           v4\sz,   v16\sz,  v21\sz // out0
        sqsub           v21\sz,  v31\sz,  v26\sz // t3a
        sqadd           \o15\sz, v31\sz,  v26\sz // out15
        mov             \o0\szb, v4\szb
.endif
        sqneg           \o15\sz, \o15\sz         // out15

        sqsub           v3\sz,   v29\sz,  v18\sz // t15a
        sqadd           \o13\sz, v29\sz,  v18\sz // out13
        sqadd           \o2\sz,  v17\sz,  v30\sz // out2
        sqsub           v26\sz,  v17\sz,  v30\sz // t14a
        sqneg           \o13\sz, \o13\sz         // out13

        sqadd           \o1\sz,  v19\sz,  v27\sz // out1
        sqsub           v27\sz,  v19\sz,  v27\sz // t10
        sqadd           \o14\sz, v28\sz,  v20\sz // out14
        sqsub           v20\sz,  v28\sz,  v20\sz // t11
        sqneg           \o1\sz,  \o1\sz          // out1

        sqadd           \o3\sz,  v22\sz,  v24\sz // out3
        sqsub           v22\sz,  v22\sz,  v24\sz // t6
        sqadd           \o12\sz, v25\sz,  v23\sz // out12
        sqsub           v23\sz,  v25\sz,  v23\sz // t7
        sqneg           \o3\sz,  \o3\sz          // out3

        smull_smlsl     v24, v25, v2,  v21, v0.h[0], v0.h[0], \sz // -> out8 (v24 or v23)
        smull_smlal     v4,  v5,  v2,  v21, v0.h[0], v0.h[0], \sz // -> out7 (v23 or v24)
        smull_smlal     v6,  v7,  v26, v3,  v0.h[0], v0.h[0], \sz // -> out5 (v21 or v26)

        sqrshrn_sz      v24, v24, v25, #12, \sz // out8
        sqrshrn_sz      v4,  v4,  v5,  #12, \sz // out7
        sqrshrn_sz      v5,  v6,  v7,  #12, \sz // out5
        smull_smlsl     v6,  v7,  v26, v3,  v0.h[0], v0.h[0], \sz // -> out10 (v26 or v21)
        smull_smlal     v2,  v3,  v22, v23, v0.h[0], v0.h[0], \sz // -> out4 (v20 or v27)
        sqrshrn_sz      v26, v6,  v7,  #12, \sz // out10

        smull_smlsl     v6,  v7,  v22, v23, v0.h[0], v0.h[0], \sz // -> out11 (v27 or v20)
        smull_smlal     v22, v23, v27, v20, v0.h[0], v0.h[0], \sz // -> out6 (v22 or v25)
        smull_smlsl     v21, v25, v27, v20, v0.h[0], v0.h[0], \sz // -> out9 (v25 or v22)

        sqrshrn_sz      \o4, v2,  v3,  #12, \sz // out4
        sqrshrn_sz      v6,  v6,  v7,  #12, \sz // out11
        sqrshrn_sz      v7,  v21, v25, #12, \sz // out9
        sqrshrn_sz      \o6, v22, v23, #12, \sz // out6

.ifc \o8, v23
        mov             \o8\szb,  v24\szb
        mov             \o10\szb, v26\szb
.endif

        sqneg           \o7\sz,  v4\sz // out7
        sqneg           \o5\sz,  v5\sz // out5
        sqneg           \o11\sz, v6\sz // out11
        sqneg           \o9\sz,  v7\sz // out9
.endm

function inv_adst_8h_x16_neon, export=1
        iadst_16        v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, .8h, .16b
        ret
endfunc

function inv_flipadst_8h_x16_neon, export=1
        iadst_16        v31, v30, v29, v28, v27, v26, v25, v24, v23, v22, v21, v20, v19, v18, v17, v16, .8h, .16b
        ret
endfunc

function inv_adst_4h_x16_neon, export=1
        iadst_16        v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, .4h, .8b
        ret
endfunc

function inv_flipadst_4h_x16_neon, export=1
        iadst_16        v31, v30, v29, v28, v27, v26, v25, v24, v23, v22, v21, v20, v19, v18, v17, v16, .4h, .8b
        ret
endfunc

function inv_identity_8h_x16_neon, export=1
        mov             w16, #2*(5793-4096)*8
        dup             v0.4h,   w16
.irp i, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        sqrdmulh        v2.8h,      v\i\().8h,  v0.h[0]
        sqadd           v\i\().8h,  v\i\().8h,  v\i\().8h
        sqadd           v\i\().8h,  v\i\().8h,  v2.8h
.endr
        ret
endfunc

function inv_identity_4h_x16_neon, export=1
        mov             w16, #2*(5793-4096)*8
        dup             v0.4h,   w16
.irp i, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        sqrdmulh        v2.4h,      v\i\().4h,  v0.h[0]
        sqadd           v\i\().4h,  v\i\().4h,  v\i\().4h
        sqadd           v\i\().4h,  v\i\().4h,  v2.4h
.endr
        ret
endfunc

.macro identity_8x16_shift2 c
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        sqrdmulh        v2.8h,   \i,      \c
        sshr            v2.8h,   v2.8h,   #1
        srhadd          \i,      \i,      v2.8h
.endr
.endm

.macro identity_8x16_shift1 c
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        sqrdmulh        v2.8h,   \i,      \c
        srshr           v2.8h,   v2.8h,   #1
        sqadd           \i,      \i,      v2.8h
.endr
.endm

.macro identity_8x8_shift1 c
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        sqrdmulh        v2.8h,   \i,      \c
        srshr           v2.8h,   v2.8h,   #1
        sqadd           \i,      \i,      v2.8h
.endr
.endm

.macro identity_8x8 c
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        sqrdmulh        v2.8h,   \i,      \c
        sqadd           \i,      \i,      \i
        sqadd           \i,      \i,      v2.8h
.endr
.endm

.macro def_horz_16 scale=0, identity=0, shift=2, suffix
function inv_txfm_horz\suffix\()_16x8_neon
        AARCH64_VALID_CALL_TARGET
        mov             x14, x30
        movi            v7.8h,  #0
.if \identity
        mov             w16, #2*(5793-4096)*8
        dup             v0.4h,   w16
.elseif \scale
        mov             w16, #2896*8
        dup             v0.4h,   w16
.endif
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        ld1             {\i}, [x7]
        st1             {v7.8h}, [x7], x8
.endr
.if \scale
        scale_input     .8h, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .8h, v0.h[0], v24, v25, v26, v27, v28, v29, v30, v31
.endif
.if \identity
        identity_8x16_shift2 v0.h[0]
.else
        blr             x4
.endif
.if \shift > 0
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        srshr           \i,  \i,  #\shift
.endr
.endif
        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5
        transpose_8x8h  v24, v25, v26, v27, v28, v29, v30, v31, v4, v5

.irp i, v16.8h, v24.8h, v17.8h, v25.8h, v18.8h, v26.8h, v19.8h, v27.8h, v20.8h, v28.8h, v21.8h, v29.8h, v22.8h, v30.8h, v23.8h, v31.8h
        st1             {\i}, [x6], #16
.endr

        ret             x14
endfunc
.endm

def_horz_16 scale=0, identity=0, shift=2
def_horz_16 scale=1, identity=0, shift=1, suffix=_scale
def_horz_16 scale=0, identity=1, shift=0, suffix=_identity

function inv_txfm_add_vert_8x16_neon
        mov             x14, x30
.irp i, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        ld1             {v\i\().8h}, [x7], x8
.endr
        blr             x5
        load_add_store_8x16 x6, x7
        ret             x14
endfunc

function inv_txfm_add_16x16_neon
        mov             x15, x30
        sub             sp,  sp,  #512
.irp i, 0, 8
        add             x6,  sp,  #(\i*16*2)
.if \i == 8
        cmp             w3,  w13
        b.lt            1f
.endif
        add             x7,  x2,  #(\i*2)
        mov             x8,  #16*2
        blr             x9
.endr
        b               2f
1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
2:
.irp i, 0, 8
        add             x6,  x0,  #(\i)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #32
        bl              inv_txfm_add_vert_8x16_neon
.endr

        add             sp,  sp,  #512
        ret             x15
endfunc

.macro def_fn_16x16 txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_16x16_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         16,  16,  2
.endif
.ifc \txfm1, identity
        adr             x9,  inv_txfm_horz_identity_16x8_neon
.else
        adr             x9,  inv_txfm_horz_16x8_neon
        adr             x4,  inv_\txfm1\()_8h_x16_neon
.endif
        adr             x5,  inv_\txfm2\()_8h_x16_neon
        mov             x13, #\eob_half
        b               inv_txfm_add_16x16_neon
endfunc
.endm

def_fn_16x16 dct, dct, 36
def_fn_16x16 identity, identity, 36
def_fn_16x16 dct, adst, 36
def_fn_16x16 dct, flipadst, 36
def_fn_16x16 dct, identity, 8
def_fn_16x16 adst, dct, 36
def_fn_16x16 adst, adst, 36
def_fn_16x16 adst, flipadst, 36
def_fn_16x16 flipadst, dct, 36
def_fn_16x16 flipadst, adst, 36
def_fn_16x16 flipadst, flipadst, 36
def_fn_16x16 identity, dct, 8

.macro def_fn_416_base variant
function inv_txfm_\variant\()add_16x4_neon
        mov             x15, x30
        movi            v4.8h,  #0

.ifc \variant, identity_
.irp i, v16.4h, v17.4h, v18.4h, v19.4h
        ld1             {\i},    [x2]
        st1             {v4.4h}, [x2], #8
.endr
.irp i, v16.d, v17.d, v18.d, v19.d
        ld1             {\i}[1], [x2]
        st1             {v4.4h}, [x2], #8
.endr
        mov             w16, #2*(5793-4096)*8
        dup             v0.4h,   w16
.irp i, v20.4h, v21.4h, v22.4h, v23.4h
        ld1             {\i},    [x2]
        st1             {v4.4h}, [x2], #8
.endr
.irp i, v20.d, v21.d, v22.d, v23.d
        ld1             {\i}[1], [x2]
        st1             {v4.4h}, [x2], #8
.endr

        identity_8x16_shift1 v0.h[0]
.else
.irp i, v16.4h, v17.4h, v18.4h, v19.4h, v20.4h, v21.4h, v22.4h, v23.4h, v24.4h, v25.4h, v26.4h, v27.4h, v28.4h, v29.4h, v30.4h, v31.4h
        ld1             {\i},    [x2]
        st1             {v4.4h}, [x2], #8
.endr

        blr             x4

        ins             v16.d[1], v20.d[0]
        ins             v17.d[1], v21.d[0]
        ins             v18.d[1], v22.d[0]
        ins             v19.d[1], v23.d[0]
.irp i, v16.8h, v17.8h, v18.8h, v19.8h
        srshr           \i,  \i,  #1
.endr
.endif
        transpose_4x8h  v16, v17, v18, v19, v2,  v3,  v4,  v5
        blr             x5
        mov             x6,  x0
        load_add_store_8x4 x6, x7

.ifc \variant, identity_
        mov             v16.16b, v20.16b
        mov             v17.16b, v21.16b
        mov             v18.16b, v22.16b
        mov             v19.16b, v23.16b
.else
        ins             v24.d[1], v28.d[0]
        ins             v25.d[1], v29.d[0]
        ins             v26.d[1], v30.d[0]
        ins             v27.d[1], v31.d[0]
        srshr           v16.8h,  v24.8h,  #1
        srshr           v17.8h,  v25.8h,  #1
        srshr           v18.8h,  v26.8h,  #1
        srshr           v19.8h,  v27.8h,  #1
.endif
        transpose_4x8h  v16, v17, v18, v19, v2,  v3,  v4,  v5
        blr             x5
        add             x6,  x0,  #8
        load_add_store_8x4 x6, x7

        ret             x15
endfunc

function inv_txfm_\variant\()add_4x16_neon
        mov             x15, x30
        movi            v2.8h,   #0

        mov             x11, #32
        cmp             w3,  w13
        b.lt            1f

        add             x6,  x2,  #16
.ifc \variant, identity_
.irp i, v24.8h, v25.8h, v26.8h, v27.8h
        ld1             {\i},    [x6]
        st1             {v2.8h}, [x6], x11
.endr
        mov             w16, #(5793-4096)*8
        dup             v0.4h,   w16
        identity_8x4_shift1 v24, v25, v26, v27, v0.h[0]
.else
.irp i, v16.8h, v17.8h, v18.8h, v19.8h
        ld1             {\i},    [x6]
        st1             {v2.8h}, [x6], x11
.endr
        blr             x4
        srshr           v24.8h,  v16.8h,  #1
        srshr           v25.8h,  v17.8h,  #1
        srshr           v26.8h,  v18.8h,  #1
        srshr           v27.8h,  v19.8h,  #1
.endif
        transpose_4x8h  v24, v25, v26, v27, v4,  v5,  v6,  v7
        ins             v28.d[0], v24.d[1]
        ins             v29.d[0], v25.d[1]
        ins             v30.d[0], v26.d[1]
        ins             v31.d[0], v27.d[1]

        b               2f
1:
.irp i, v24.4h, v25.4h, v26.4h, v27.4h, v28.4h, v29.4h, v30.4h, v31.4h
        movi            \i,  #0
.endr
2:
        movi            v2.8h,   #0
.irp i, v16.8h, v17.8h, v18.8h, v19.8h
        ld1             {\i},    [x2]
        st1             {v2.8h}, [x2], x11
.endr
.ifc \variant, identity_
        mov             w16, #(5793-4096)*8
        dup             v0.4h,   w16
        identity_8x4_shift1 v16, v17, v18, v19, v0.h[0]
.else
        blr             x4
.irp i, v16.8h, v17.8h, v18.8h, v19.8h
        srshr           \i,  \i,  #1
.endr
.endif
        transpose_4x8h  v16, v17, v18, v19, v4,  v5,  v6,  v7
        ins             v20.d[0], v16.d[1]
        ins             v21.d[0], v17.d[1]
        ins             v22.d[0], v18.d[1]
        ins             v23.d[0], v19.d[1]

        blr             x5

        load_add_store_4x16 x0, x6

        ret             x15
endfunc
.endm

def_fn_416_base
def_fn_416_base identity_

.macro def_fn_416 w, h, txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
.if \w == 4
        adr             x4,  inv_\txfm1\()_8h_x\w\()_neon
        adr             x5,  inv_\txfm2\()_4h_x\h\()_neon
        mov             w13, #\eob_half
.else
        adr             x4,  inv_\txfm1\()_4h_x\w\()_neon
        adr             x5,  inv_\txfm2\()_8h_x\h\()_neon
.endif
.ifc \txfm1, identity
        b               inv_txfm_identity_add_\w\()x\h\()_neon
.else
        b               inv_txfm_add_\w\()x\h\()_neon
.endif
endfunc
.endm

.macro def_fns_416 w, h
def_fn_416 \w, \h, dct, dct, 29
def_fn_416 \w, \h, identity, identity, 29
def_fn_416 \w, \h, dct, adst, 29
def_fn_416 \w, \h, dct, flipadst, 29
def_fn_416 \w, \h, dct, identity, 8
def_fn_416 \w, \h, adst, dct, 29
def_fn_416 \w, \h, adst, adst, 29
def_fn_416 \w, \h, adst, flipadst, 29
def_fn_416 \w, \h, flipadst, dct, 29
def_fn_416 \w, \h, flipadst, adst, 29
def_fn_416 \w, \h, flipadst, flipadst, 29
def_fn_416 \w, \h, identity, dct, 32
def_fn_416 \w, \h, adst, identity, 8
def_fn_416 \w, \h, flipadst, identity, 8
def_fn_416 \w, \h, identity, adst, 32
def_fn_416 \w, \h, identity, flipadst, 32
.endm

def_fns_416 4, 16
def_fns_416 16, 4


.macro def_fn_816_base variant
function inv_txfm_\variant\()add_16x8_neon
        mov             x15, x30
        movi            v4.8h,  #0
        mov             w16, #2896*8
        dup             v0.4h,   w16

.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        ld1             {\i},    [x2]
        st1             {v4.8h}, [x2], #16
.endr

        scale_input     .8h, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .8h, v0.h[0], v24, v25, v26, v27, v28, v29, v30, v31
.ifc \variant, identity_
        mov             w16, #2*(5793-4096)*8
        dup             v0.4h,   w16
        identity_8x16_shift1 v0.h[0]
.else
        blr             x4

.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        srshr           \i,  \i,  #1
.endr
.endif
        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v2, v3

        blr             x5

        mov             x6,  x0
        load_add_store_8x8 x6, x7

.ifc \variant, identity_
        mov             v16.16b, v24.16b
        mov             v17.16b, v25.16b
        mov             v18.16b, v26.16b
        mov             v19.16b, v27.16b
        mov             v20.16b, v28.16b
        mov             v21.16b, v29.16b
        mov             v22.16b, v30.16b
        mov             v23.16b, v31.16b
.else
        srshr           v16.8h,  v24.8h,  #1
        srshr           v17.8h,  v25.8h,  #1
        srshr           v18.8h,  v26.8h,  #1
        srshr           v19.8h,  v27.8h,  #1
        srshr           v20.8h,  v28.8h,  #1
        srshr           v21.8h,  v29.8h,  #1
        srshr           v22.8h,  v30.8h,  #1
        srshr           v23.8h,  v31.8h,  #1
.endif

        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v2, v3

        blr             x5

        add             x0,  x0,  #8
        load_add_store_8x8 x0, x7

        ret             x15
endfunc

function inv_txfm_\variant\()add_8x16_neon
        mov             x15, x30
        movi            v4.8h,   #0
        mov             w16, #2896*8
        dup             v0.4h,   w16
        mov             x11, #32

        cmp             w3,  w13
        b.lt            1f

        add             x6,  x2,  #16
.ifc \variant, identity_
.irp i, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        ld1             {\i},    [x6]
        st1             {v4.8h}, [x6], x11
.endr
        scale_input     .8h, v0.h[0], v24, v25, v26, v27, v28, v29, v30, v31
        // The identity shl #1 and downshift srshr #1 cancel out
.else
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        ld1             {\i},    [x6]
        st1             {v4.8h}, [x6], x11
.endr
        scale_input     .8h, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23
        blr             x4

        srshr           v24.8h,  v16.8h,  #1
        srshr           v25.8h,  v17.8h,  #1
        srshr           v26.8h,  v18.8h,  #1
        srshr           v27.8h,  v19.8h,  #1
        srshr           v28.8h,  v20.8h,  #1
        srshr           v29.8h,  v21.8h,  #1
        srshr           v30.8h,  v22.8h,  #1
        srshr           v31.8h,  v23.8h,  #1
.endif
        transpose_8x8h  v24, v25, v26, v27, v28, v29, v30, v31, v2, v3

        b               2f

1:
.irp i, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        movi            \i,  #0
.endr

2:
        movi            v4.8h,   #0
        mov             w16, #2896*8
        dup             v0.4h,   w16

.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        ld1             {\i},    [x2]
        st1             {v4.8h}, [x2], x11
.endr
        scale_input     .8h, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23
.ifc \variant, identity_
        // The identity shl #1 and downshift srshr #1 cancel out
.else
        blr             x4

.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        srshr           \i,  \i,  #1
.endr
.endif

        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v2, v3

        blr             x5

        load_add_store_8x16 x0, x6

        ret             x15
endfunc
.endm

def_fn_816_base
def_fn_816_base identity_

.macro def_fn_816 w, h, txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
        adr             x4,  inv_\txfm1\()_8h_x\w\()_neon
        adr             x5,  inv_\txfm2\()_8h_x\h\()_neon
.if \w == 8
        mov             x13, #\eob_half
.endif
.ifc \txfm1, identity
        b               inv_txfm_identity_add_\w\()x\h\()_neon
.else
        b               inv_txfm_add_\w\()x\h\()_neon
.endif
endfunc
.endm

.macro def_fns_816 w, h
def_fn_816 \w, \h, dct, dct, 43
def_fn_816 \w, \h, identity, identity, 43
def_fn_816 \w, \h, dct, adst, 43
def_fn_816 \w, \h, dct, flipadst, 43
def_fn_816 \w, \h, dct, identity, 8
def_fn_816 \w, \h, adst, dct, 43
def_fn_816 \w, \h, adst, adst, 43
def_fn_816 \w, \h, adst, flipadst, 43
def_fn_816 \w, \h, flipadst, dct, 43
def_fn_816 \w, \h, flipadst, adst, 43
def_fn_816 \w, \h, flipadst, flipadst, 43
def_fn_816 \w, \h, identity, dct, 64
def_fn_816 \w, \h, adst, identity, 8
def_fn_816 \w, \h, flipadst, identity, 8
def_fn_816 \w, \h, identity, adst, 64
def_fn_816 \w, \h, identity, flipadst, 64
.endm

def_fns_816 8, 16
def_fns_816 16, 8

function inv_dct32_odd_8h_x16_neon, export=1
        movrel          x16, idct_coeffs, 2*16
        ld1             {v0.8h, v1.8h}, [x16]
        sub             x16, x16, #2*16

        smull_smlsl     v2,  v3,  v16, v31, v0.h[0], v0.h[1], .8h // -> t16a
        smull_smlal     v4,  v5,  v16, v31, v0.h[1], v0.h[0], .8h // -> t31a
        smull_smlsl     v6,  v7,  v24, v23, v0.h[2], v0.h[3], .8h // -> t17a
        sqrshrn_sz      v16, v2,  v3,  #12, .8h                   // t16a
        sqrshrn_sz      v31, v4,  v5,  #12, .8h                   // t31a
        smull_smlal     v2,  v3,  v24, v23, v0.h[3], v0.h[2], .8h // -> t30a
        smull_smlsl     v4,  v5,  v20, v27, v0.h[4], v0.h[5], .8h // -> t18a
        sqrshrn_sz      v24, v6,  v7,  #12, .8h                   // t17a
        sqrshrn_sz      v23, v2,  v3,  #12, .8h                   // t30a
        smull_smlal     v6,  v7,  v20, v27, v0.h[5], v0.h[4], .8h // -> t29a
        smull_smlsl     v2,  v3,  v28, v19, v0.h[6], v0.h[7], .8h // -> t19a
        sqrshrn_sz      v20, v4,  v5,  #12, .8h                   // t18a
        sqrshrn_sz      v27, v6,  v7,  #12, .8h                   // t29a
        smull_smlal     v4,  v5,  v28, v19, v0.h[7], v0.h[6], .8h // -> t28a
        smull_smlsl     v6,  v7,  v18, v29, v1.h[0], v1.h[1], .8h // -> t20a
        sqrshrn_sz      v28, v2,  v3,  #12, .8h                   // t19a
        sqrshrn_sz      v19, v4,  v5,  #12, .8h                   // t28a
        smull_smlal     v2,  v3,  v18, v29, v1.h[1], v1.h[0], .8h // -> t27a
        smull_smlsl     v4,  v5,  v26, v21, v1.h[2], v1.h[3], .8h // -> t21a
        sqrshrn_sz      v18, v6,  v7,  #12, .8h                   // t20a
        sqrshrn_sz      v29, v2,  v3,  #12, .8h                   // t27a
        smull_smlal     v6,  v7,  v26, v21, v1.h[3], v1.h[2], .8h // -> t26a
        smull_smlsl     v2,  v3,  v22, v25, v1.h[4], v1.h[5], .8h // -> t22a
        sqrshrn_sz      v26, v4,  v5,  #12, .8h                   // t21a
        sqrshrn_sz      v21, v6,  v7,  #12, .8h                   // t26a
        smull_smlal     v4,  v5,  v22, v25, v1.h[5], v1.h[4], .8h // -> t25a
        smull_smlsl     v6,  v7,  v30, v17, v1.h[6], v1.h[7], .8h // -> t23a
        sqrshrn_sz      v22, v2,  v3,  #12, .8h                   // t22a
        sqrshrn_sz      v25, v4,  v5,  #12, .8h                   // t25a
        smull_smlal     v2,  v3,  v30, v17, v1.h[7], v1.h[6], .8h // -> t24a
        sqrshrn_sz      v30, v6,  v7,  #12, .8h                   // t23a
        sqrshrn_sz      v17, v2,  v3,  #12, .8h                   // t24a

        ld1             {v0.8h}, [x16]

        sqsub           v2.8h,   v16.8h,  v24.8h // t17
        sqadd           v16.8h,  v16.8h,  v24.8h // t16
        sqsub           v3.8h,   v31.8h,  v23.8h // t30
        sqadd           v31.8h,  v31.8h,  v23.8h // t31
        sqsub           v24.8h,  v28.8h,  v20.8h // t18
        sqadd           v28.8h,  v28.8h,  v20.8h // t19
        sqadd           v23.8h,  v18.8h,  v26.8h // t20
        sqsub           v18.8h,  v18.8h,  v26.8h // t21
        sqsub           v20.8h,  v30.8h,  v22.8h // t22
        sqadd           v30.8h,  v30.8h,  v22.8h // t23
        sqadd           v26.8h,  v17.8h,  v25.8h // t24
        sqsub           v17.8h,  v17.8h,  v25.8h // t25
        sqsub           v22.8h,  v29.8h,  v21.8h // t26
        sqadd           v29.8h,  v29.8h,  v21.8h // t27
        sqadd           v25.8h,  v19.8h,  v27.8h // t28
        sqsub           v19.8h,  v19.8h,  v27.8h // t29

        smull_smlsl     v4,  v5,  v3,  v2,  v0.h[4], v0.h[5], .8h // -> t17a
        smull_smlal     v6,  v7,  v3,  v2,  v0.h[5], v0.h[4], .8h // -> t30a
        smull_smlal     v2,  v3,  v19, v24, v0.h[5], v0.h[4], .8h // -> t18a
        sqrshrn_sz      v21, v4,  v5,  #12, .8h                   // t17a
        sqrshrn_sz      v27, v6,  v7,  #12, .8h                   // t30a
        neg             v2.4s,   v2.4s                            // -> t18a
        neg             v3.4s,   v3.4s                            // -> t18a
        smull_smlsl     v4,  v5,  v19, v24, v0.h[4], v0.h[5], .8h // -> t29a
        smull_smlsl     v6,  v7,  v22, v18, v0.h[6], v0.h[7], .8h // -> t21a
        sqrshrn_sz      v19, v2,  v3,  #12, .8h                   // t18a
        sqrshrn_sz      v24, v4,  v5,  #12, .8h                   // t29a
        smull_smlal     v2,  v3,  v22, v18, v0.h[7], v0.h[6], .8h // -> t26a
        smull_smlal     v4,  v5,  v17, v20, v0.h[7], v0.h[6], .8h // -> t22a
        sqrshrn_sz      v22, v6,  v7,  #12, .8h                   // t21a
        sqrshrn_sz      v18, v2,  v3,  #12, .8h                   // t26a
        neg             v4.4s,   v4.4s                            // -> t22a
        neg             v5.4s,   v5.4s                            // -> t22a
        smull_smlsl     v6,  v7,  v17, v20, v0.h[6], v0.h[7], .8h // -> t25a
        sqrshrn_sz      v17, v4,  v5,  #12, .8h                   // t22a
        sqrshrn_sz      v20, v6,  v7,  #12, .8h                   // t25a

        sqsub           v2.8h,   v27.8h,  v24.8h // t29
        sqadd           v27.8h,  v27.8h,  v24.8h // t30
        sqsub           v3.8h,   v21.8h,  v19.8h // t18
        sqadd           v21.8h,  v21.8h,  v19.8h // t17
        sqsub           v24.8h,  v16.8h,  v28.8h // t19a
        sqadd           v16.8h,  v16.8h,  v28.8h // t16a
        sqsub           v19.8h,  v30.8h,  v23.8h // t20a
        sqadd           v30.8h,  v30.8h,  v23.8h // t23a
        sqsub           v28.8h,  v17.8h,  v22.8h // t21
        sqadd           v17.8h,  v17.8h,  v22.8h // t22
        sqadd           v23.8h,  v26.8h,  v29.8h // t24a
        sqsub           v26.8h,  v26.8h,  v29.8h // t27a
        sqadd           v22.8h,  v20.8h,  v18.8h // t25
        sqsub           v20.8h,  v20.8h,  v18.8h // t26
        sqsub           v29.8h,  v31.8h,  v25.8h // t28a
        sqadd           v31.8h,  v31.8h,  v25.8h // t31a

        smull_smlsl     v4,  v5,  v2,  v3,  v0.h[2], v0.h[3], .8h // -> t18a
        smull_smlal     v6,  v7,  v2,  v3,  v0.h[3], v0.h[2], .8h // -> t29a
        smull_smlsl     v2,  v3,  v29, v24, v0.h[2], v0.h[3], .8h // -> t19
        sqrshrn_sz      v18, v4,  v5,  #12, .8h                   // t18a
        sqrshrn_sz      v25, v6,  v7,  #12, .8h                   // t29a
        smull_smlal     v4,  v5,  v29, v24, v0.h[3], v0.h[2], .8h // -> t28
        smull_smlal     v6,  v7,  v26, v19, v0.h[3], v0.h[2], .8h // -> t20
        sqrshrn_sz      v29, v2,  v3,  #12, .8h                   // t19
        sqrshrn_sz      v24, v4,  v5,  #12, .8h                   // t28
        neg             v6.4s,   v6.4s                            // -> t20
        neg             v7.4s,   v7.4s                            // -> t20
        smull_smlsl     v2,  v3,  v26, v19, v0.h[2], v0.h[3], .8h // -> t27
        smull_smlal     v4,  v5,  v20, v28, v0.h[3], v0.h[2], .8h // -> t21a
        sqrshrn_sz      v26, v6,  v7,  #12, .8h                   // t20
        sqrshrn_sz      v19, v2,  v3,  #12, .8h                   // t27
        neg             v4.4s,   v4.4s                            // -> t21a
        neg             v5.4s,   v5.4s                            // -> t21a
        smull_smlsl     v6,  v7,  v20, v28, v0.h[2], v0.h[3], .8h // -> t26a
        sqrshrn_sz      v20, v4,  v5,  #12, .8h                   // t21a
        sqrshrn_sz      v28, v6,  v7,  #12, .8h                   // t26a

        sqsub           v2.8h,   v16.8h,  v30.8h // t23
        sqadd           v16.8h,  v16.8h,  v30.8h // t16 = out16
        sqsub           v3.8h,   v31.8h,  v23.8h // t24
        sqadd           v31.8h,  v31.8h,  v23.8h // t31 = out31
        sqsub           v23.8h,  v21.8h,  v17.8h // t22a
        sqadd           v17.8h,  v21.8h,  v17.8h // t17a = out17
        sqadd           v30.8h,  v27.8h,  v22.8h // t30a = out30
        sqsub           v21.8h,  v27.8h,  v22.8h // t25a
        sqsub           v27.8h,  v18.8h,  v20.8h // t21
        sqadd           v18.8h,  v18.8h,  v20.8h // t18 = out18
        sqadd           v4.8h,   v29.8h,  v26.8h // t19a = out19
        sqsub           v26.8h,  v29.8h,  v26.8h // t20a
        sqadd           v29.8h,  v25.8h,  v28.8h // t29 = out29
        sqsub           v25.8h,  v25.8h,  v28.8h // t26
        sqadd           v28.8h,  v24.8h,  v19.8h // t28a = out28
        sqsub           v24.8h,  v24.8h,  v19.8h // t27a
        mov             v19.16b, v4.16b          // out19

        smull_smlsl     v4,  v5,  v24, v26, v0.h[0], v0.h[0], .8h // -> t20
        smull_smlal     v6,  v7,  v24, v26, v0.h[0], v0.h[0], .8h // -> t27
        sqrshrn_sz      v20, v4,  v5,  #12, .8h   // t20
        sqrshrn_sz      v22, v6,  v7,  #12, .8h   // t27

        smull_smlal     v4,  v5,  v25, v27, v0.h[0], v0.h[0], .8h // -> t26a
        smull_smlsl     v6,  v7,  v25, v27, v0.h[0], v0.h[0], .8h // -> t21a
        mov             v27.16b,  v22.16b         // t27
        sqrshrn_sz      v26, v4,  v5,  #12, .8h   // t26a

        smull_smlsl     v24, v25, v21, v23, v0.h[0], v0.h[0], .8h // -> t22
        smull_smlal     v4,  v5,  v21, v23, v0.h[0], v0.h[0], .8h // -> t25
        sqrshrn_sz      v21, v6,  v7,  #12, .8h   // t21a
        sqrshrn_sz      v22, v24, v25, #12, .8h   // t22
        sqrshrn_sz      v25, v4,  v5,  #12, .8h   // t25

        smull_smlsl     v4,  v5,  v3,  v2,  v0.h[0], v0.h[0], .8h // -> t23a
        smull_smlal     v6,  v7,  v3,  v2,  v0.h[0], v0.h[0], .8h // -> t24a
        sqrshrn_sz      v23, v4,  v5,  #12, .8h   // t23a
        sqrshrn_sz      v24, v6,  v7,  #12, .8h   // t24a

        ret
endfunc

.macro def_horz_32 scale=0, shift=2, suffix
function inv_txfm_horz\suffix\()_dct_32x8_neon
        mov             x14, x30
        movi            v7.8h,  #0
        lsl             x8,  x8,  #1
.if \scale
        mov             w16, #2896*8
        dup             v0.4h,   w16
.endif

.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        ld1             {\i}, [x7]
        st1             {v7.8h}, [x7], x8
.endr
        sub             x7,  x7,  x8, lsl #4
        add             x7,  x7,  x8, lsr #1
.if \scale
        scale_input     .8h, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .8h, v0.h[0], v24, v25, v26, v27, v28, v29, v30, v31
.endif
        bl              inv_dct_8h_x16_neon
        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5
        transpose_8x8h  v24, v25, v26, v27, v28, v29, v30, v31, v4, v5

.macro store1 r0, r1
        st1             {\r0}, [x6], #16
        st1             {\r1}, [x6], #16
        add             x6,  x6,  #32
.endm
        store1          v16.8h,  v24.8h
        store1          v17.8h,  v25.8h
        store1          v18.8h,  v26.8h
        store1          v19.8h,  v27.8h
        store1          v20.8h,  v28.8h
        store1          v21.8h,  v29.8h
        store1          v22.8h,  v30.8h
        store1          v23.8h,  v31.8h
.purgem store1
        sub             x6,  x6,  #64*8

        movi            v7.8h,  #0
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        ld1             {\i}, [x7]
        st1             {v7.8h}, [x7], x8
.endr
.if \scale
        // This relies on the fact that the idct also leaves the right coeff in v0.h[1]
        scale_input     .8h, v0.h[1], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .8h, v0.h[1], v24, v25, v26, v27, v28, v29, v30, v31
.endif
        bl              inv_dct32_odd_8h_x16_neon
        transpose_8x8h  v31, v30, v29, v28, v27, v26, v25, v24, v4, v5
        transpose_8x8h  v23, v22, v21, v20, v19, v18, v17, v16, v4, v5
.macro store2 r0, r1, shift
        ld1             {v4.8h, v5.8h}, [x6]
        sqsub           v7.8h,   v4.8h,   \r0
        sqsub           v6.8h,   v5.8h,   \r1
        sqadd           v4.8h,   v4.8h,   \r0
        sqadd           v5.8h,   v5.8h,   \r1
        rev64           v6.8h,   v6.8h
        rev64           v7.8h,   v7.8h
        srshr           v4.8h,   v4.8h,   #\shift
        srshr           v5.8h,   v5.8h,   #\shift
        srshr           v6.8h,   v6.8h,   #\shift
        srshr           v7.8h,   v7.8h,   #\shift
        ext             v6.16b,  v6.16b,  v6.16b,  #8
        st1             {v4.8h, v5.8h}, [x6], #32
        ext             v7.16b,  v7.16b,  v7.16b,  #8
        st1             {v6.8h, v7.8h}, [x6], #32
.endm

        store2          v31.8h,  v23.8h, \shift
        store2          v30.8h,  v22.8h, \shift
        store2          v29.8h,  v21.8h, \shift
        store2          v28.8h,  v20.8h, \shift
        store2          v27.8h,  v19.8h, \shift
        store2          v26.8h,  v18.8h, \shift
        store2          v25.8h,  v17.8h, \shift
        store2          v24.8h,  v16.8h, \shift
.purgem store2
        ret             x14
endfunc
.endm

def_horz_32 scale=0, shift=2
def_horz_32 scale=1, shift=1, suffix=_scale

function inv_txfm_add_vert_dct_8x32_neon
        mov             x14, x30
        lsl             x8,  x8,  #1

.irp i, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        ld1             {v\i\().8h}, [x7], x8
.endr
        sub             x7,  x7,  x8, lsl #4

        bl              inv_dct_8h_x16_neon

.irp i, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        st1             {v\i\().8h}, [x7], x8
.endr
        sub             x7,  x7,  x8, lsl #4
        add             x7,  x7,  x8, lsr #1

.irp i, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        ld1             {v\i\().8h}, [x7], x8
.endr
        sub             x7,  x7,  x8, lsl #4
        sub             x7,  x7,  x8, lsr #1
        bl              inv_dct32_odd_8h_x16_neon

        neg             x9,  x8
        mov             x10, x6
.macro combine r0, r1, r2, r3, op, stride
        ld1             {v5.8h}, [x7],    \stride
        ld1             {v2.8b}, [x10],   x1
        ld1             {v6.8h}, [x7],    \stride
        ld1             {v3.8b}, [x10],   x1
        \op             v5.8h,   v5.8h,   \r0
        ld1             {v7.8h}, [x7],    \stride
        ld1             {v4.8b}, [x10],   x1
        srshr           v5.8h,   v5.8h,   #4
        \op             v6.8h,   v6.8h,   \r1
        uaddw           v5.8h,   v5.8h,   v2.8b
        srshr           v6.8h,   v6.8h,   #4
        \op             v7.8h,   v7.8h,   \r2
        sqxtun          v2.8b,   v5.8h
        ld1             {v5.8h}, [x7],    \stride
        uaddw           v6.8h,   v6.8h,   v3.8b
        srshr           v7.8h,   v7.8h,   #4
        \op             v5.8h,   v5.8h,   \r3
        st1             {v2.8b}, [x6],    x1
        ld1             {v2.8b}, [x10],   x1
        sqxtun          v3.8b,   v6.8h
        uaddw           v7.8h,   v7.8h,   v4.8b
        srshr           v5.8h,   v5.8h,   #4
        st1             {v3.8b}, [x6],    x1
        sqxtun          v4.8b,   v7.8h
        uaddw           v5.8h,   v5.8h,   v2.8b
        st1             {v4.8b}, [x6],    x1
        sqxtun          v2.8b,   v5.8h
        st1             {v2.8b}, [x6],    x1
.endm
        combine         v31.8h, v30.8h, v29.8h, v28.8h, sqadd, x8
        combine         v27.8h, v26.8h, v25.8h, v24.8h, sqadd, x8
        combine         v23.8h, v22.8h, v21.8h, v20.8h, sqadd, x8
        combine         v19.8h, v18.8h, v17.8h, v16.8h, sqadd, x8
        sub             x7,  x7,  x8
        combine         v16.8h, v17.8h, v18.8h, v19.8h, sqsub, x9
        combine         v20.8h, v21.8h, v22.8h, v23.8h, sqsub, x9
        combine         v24.8h, v25.8h, v26.8h, v27.8h, sqsub, x9
        combine         v28.8h, v29.8h, v30.8h, v31.8h, sqsub, x9
.purgem combine

        ret             x14
endfunc

const eob_32x32
        .short 36, 136, 300, 1024
endconst

const eob_16x32
        .short 36, 151, 279, 512
endconst

const eob_16x32_shortside
        .short 36, 512
endconst

const eob_8x32
        .short 43, 107, 171, 256
endconst

function inv_txfm_add_identity_identity_32x32_8bpc_neon, export=1
        movi            v0.8h,  #0
        movrel          x13, eob_32x32

        mov             x8,  #2*32
1:
        mov             w9,  #0
        movrel          x12, eob_32x32
2:
        add             w9,  w9,  #8
.irp i, 16, 17, 18, 19, 20, 21, 22, 23
        ld1             {v\i\().8h}, [x2]
        st1             {v0.8h}, [x2], x8
.endr
        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5

        load_add_store_8x8 x0, x7, shiftbits=2
        ldrh            w11, [x12], #2
        sub             x0,  x0,  x1, lsl #3
        add             x0,  x0,  #8
        cmp             w3,  w11
        b.ge            2b

        ldrh            w11, [x13], #2
        cmp             w3,  w11
        b.lt            9f

        sub             x0,  x0,  w9, uxtw
        add             x0,  x0,  x1, lsl #3
        msub            x2,  x8,  x9,  x2
        add             x2,  x2,  #2*8
        b               1b
9:
        ret
endfunc

.macro shift_8_regs op, shift
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        \op             \i,  \i,  #\shift
.endr
.endm

.macro def_identity_1632 w, h, wshort, hshort
function inv_txfm_add_identity_identity_\w\()x\h\()_8bpc_neon, export=1
        mov             w16, #2896*8
        mov             w17, #2*(5793-4096)*8
        dup             v1.4h,   w16
        movi            v0.8h,   #0
        mov             v1.h[1], w17
        movrel          x13, eob_16x32\hshort

        mov             x8,  #2*\h
1:
        mov             w9,  #0
        movrel          x12, eob_16x32\wshort
2:
        add             w9,  w9,  #8
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        ld1             {\i}, [x2]
        st1             {v0.8h}, [x2], x8
.endr
        scale_input     .8h, v1.h[0], v16, v17, v18, v19, v20, v21, v22, v23

.if \w == 16
        // 16x32
        identity_8x8_shift1 v1.h[1]
.else
        // 32x16
        shift_8_regs    sqshl, 1
        identity_8x8    v1.h[1]
.endif

        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5

.if \w == 16
        load_add_store_8x8 x0, x7, shiftbits=2
.else
        load_add_store_8x8 x0, x7, shiftbits=4
.endif
        ldrh            w11, [x12], #2
        sub             x0,  x0,  x1, lsl #3
        add             x0,  x0,  #8
        cmp             w3,  w11
        b.ge            2b

        ldrh            w11, [x13], #2
        cmp             w3,  w11
        b.lt            9f

        sub             x0,  x0,  w9, uxtw
        add             x0,  x0,  x1, lsl #3
        msub            x2,  x8,  x9,  x2
        add             x2,  x2,  #2*8
        b               1b
9:
        ret
endfunc
.endm

def_identity_1632 16, 32, _shortside,
def_identity_1632 32, 16, , _shortside

.macro def_identity_832 w, h
function inv_txfm_add_identity_identity_\w\()x\h\()_8bpc_neon, export=1
        movi            v0.8h,  #0
        movrel          x13, eob_8x32

        mov             w8,  #2*\h
1:
        ldrh            w12, [x13], #2
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
        ld1             {\i}, [x2]
        st1             {v0.8h}, [x2], x8
.endr

.if \w == 8
        // 8x32
        shift_8_regs    srshr, 1
.endif

        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5

        cmp             w3,  w12
.if \w == 8
        load_add_store_8x8 x0, x7, shiftbits=2
.else
        load_add_store_8x8 x0, x7, shiftbits=3
.endif

        b.lt            9f
.if \w == 8
        sub             x2,  x2,  x8, lsl #3
        add             x2,  x2,  #2*8
.else
        sub             x0,  x0,  x1, lsl #3
        add             x0,  x0,  #8
.endif
        b               1b

9:
        ret
endfunc
.endm

def_identity_832 8, 32
def_identity_832 32, 8

function inv_txfm_add_dct_dct_32x32_8bpc_neon, export=1
        idct_dc         32,  32,  2

        mov             x15, x30
        sub             sp,  sp,  #2048
        movrel          x13, eob_32x32
        ldrh            w12, [x13], #2

.irp i, 0, 8, 16, 24
        add             x6,  sp,  #(\i*32*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 24
        ldrh            w12, [x13], #2
.endif
.endif
        add             x7,  x2,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_horz_dct_32x8_neon
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #4
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8, 16, 24
        add             x6,  x0,  #(\i)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_add_vert_dct_8x32_neon
.endr

        add             sp,  sp,  #2048
        ret             x15
endfunc

function inv_txfm_add_dct_dct_16x32_8bpc_neon, export=1
        idct_dc         16,  32,  1

        mov             x15, x30
        sub             sp,  sp,  #1024
        movrel          x13, eob_16x32
        ldrh            w12, [x13], #2
        adr             x4,  inv_dct_8h_x16_neon

.irp i, 0, 8, 16, 24
        add             x6,  sp,  #(\i*16*2)
        add             x7,  x2,  #(\i*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 24
        ldrh            w12, [x13], #2
.endif
.endif
        mov             x8,  #2*32
        bl              inv_txfm_horz_scale_16x8_neon
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #8
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8
        add             x6,  x0,  #(\i)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #16*2
        bl              inv_txfm_add_vert_dct_8x32_neon
.endr

        add             sp,  sp,  #1024
        ret             x15
endfunc

function inv_txfm_add_dct_dct_32x16_8bpc_neon, export=1
        idct_dc         32,  16,  1

        mov             x15, x30
        sub             sp,  sp,  #1024

        adr             x5,  inv_dct_8h_x16_neon

.irp i, 0, 8
        add             x6,  sp,  #(\i*32*2)
        add             x7,  x2,  #(\i*2)
.if \i > 0
        mov             w8,  #(16 - \i)
        cmp             w3,  #36
        b.lt            1f
.endif
        mov             x8,  #2*16
        bl              inv_txfm_horz_scale_dct_32x8_neon
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #4
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8, 16, 24
        add             x6,  x0,  #(\i)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_add_vert_8x16_neon
.endr

        add             sp,  sp,  #1024
        ret             x15
endfunc

function inv_txfm_add_dct_dct_8x32_8bpc_neon, export=1
        idct_dc         8,   32, 2

        mov             x15, x30
        sub             sp,  sp,  #512

        movrel          x13, eob_8x32

        movi            v28.8h,  #0
        mov             x8,  #2*32
        mov             w9,  #32
        mov             x6,  sp
1:
.irp i, 16, 17, 18, 19, 20, 21, 22, 23
        ld1             {v\i\().8h}, [x2]
        st1             {v28.8h}, [x2], x8
.endr
        ldrh            w12, [x13], #2
        sub             x2,  x2,  x8, lsl #3
        sub             w9,  w9,  #8
        add             x2,  x2,  #2*8

        bl              inv_dct_8h_x8_neon

.irp i, 16, 17, 18, 19, 20, 21, 22, 23
        srshr           v\i\().8h,  v\i\().8h,  #2
.endr

        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v24, v25

        st1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x6], #64
        cmp             w3,  w12
        st1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x6], #64

        b.ge            1b
        cbz             w9,  3f

        movi            v29.8h,  #0
        movi            v30.8h,  #0
        movi            v31.8h,  #0
2:
        subs            w9,  w9,  #8
.rept 2
        st1             {v28.8h,v29.8h,v30.8h,v31.8h}, [x6], #64
.endr
        b.gt            2b

3:
        mov             x6,  x0
        mov             x7,  sp
        mov             x8,  #8*2
        bl              inv_txfm_add_vert_dct_8x32_neon

        add             sp,  sp,  #512
        ret             x15
endfunc

function inv_txfm_add_dct_dct_32x8_8bpc_neon, export=1
        idct_dc         32,  8,   2

        mov             x15, x30
        sub             sp,  sp,  #512

        mov             x6,  sp
        mov             x7,  x2
        mov             x8,  #8*2
        bl              inv_txfm_horz_dct_32x8_neon

        mov             x8,  #2*32
        mov             w9,  #0
1:
        add             x6,  x0,  x9
        add             x7,  sp,  x9, lsl #1 // #(\i*2)

.irp i, 16, 17, 18, 19, 20, 21, 22, 23
        ld1             {v\i\().8h}, [x7], x8
.endr
        add             w9,  w9,  #8

        bl              inv_dct_8h_x8_neon

        cmp             w9,  #32

        load_add_store_8x8 x6, x7

        b.lt            1b

        add             sp,  sp,  #512
        ret             x15
endfunc

function inv_dct64_step1_neon
        // in1/31/17/15 -> t32a/33/34a/35/60/61a/62/63a
        // in7/25/23/ 9 -> t56a/57/58a/59/36/37a/38/39a
        // in5/27/21/11 -> t40a/41/42a/43/52/53a/54/55a
        // in3/29/19/13 -> t48a/49/50a/51/44/45a/46/47a

        ld1             {v0.8h, v1.8h}, [x17], #32

        sqrdmulh        v23.8h,  v16.8h,  v0.h[1]   // t63a
        sqrdmulh        v16.8h,  v16.8h,  v0.h[0]   // t32a
        sqrdmulh        v22.8h,  v17.8h,  v0.h[2]   // t62a
        sqrdmulh        v17.8h,  v17.8h,  v0.h[3]   // t33a
        sqrdmulh        v21.8h,  v18.8h,  v0.h[5]   // t61a
        sqrdmulh        v18.8h,  v18.8h,  v0.h[4]   // t34a
        sqrdmulh        v20.8h,  v19.8h,  v0.h[6]   // t60a
        sqrdmulh        v19.8h,  v19.8h,  v0.h[7]   // t35a

        sqadd           v24.8h,  v16.8h,  v17.8h    // t32
        sqsub           v25.8h,  v16.8h,  v17.8h    // t33
        sqsub           v26.8h,  v19.8h,  v18.8h    // t34
        sqadd           v27.8h,  v19.8h,  v18.8h    // t35
        sqadd           v28.8h,  v20.8h,  v21.8h    // t60
        sqsub           v29.8h,  v20.8h,  v21.8h    // t61
        sqsub           v30.8h,  v23.8h,  v22.8h    // t62
        sqadd           v31.8h,  v23.8h,  v22.8h    // t63

        smull_smlal     v2,  v3,  v29, v26, v1.h[0], v1.h[1], .8h // -> t34a
        smull_smlsl     v4,  v5,  v29, v26, v1.h[1], v1.h[0], .8h // -> t61a
        neg             v2.4s,   v2.4s              // t34a
        neg             v3.4s,   v3.4s              // t34a
        smull_smlsl     v6,  v7,  v30, v25, v1.h[1], v1.h[0], .8h // -> t33a
        sqrshrn_sz      v26, v2,  v3,  #12, .8h     // t34a
        smull_smlal     v2,  v3,  v30, v25, v1.h[0], v1.h[1], .8h // -> t62a
        sqrshrn_sz      v29, v4,  v5,  #12, .8h     // t61a
        sqrshrn_sz      v25, v6,  v7,  #12, .8h     // t33a
        sqrshrn_sz      v30, v2,  v3,  #12, .8h     // t62a

        sqadd           v16.8h,  v24.8h,  v27.8h    // t32a
        sqsub           v19.8h,  v24.8h,  v27.8h    // t35a
        sqadd           v17.8h,  v25.8h,  v26.8h    // t33
        sqsub           v18.8h,  v25.8h,  v26.8h    // t34
        sqsub           v20.8h,  v31.8h,  v28.8h    // t60a
        sqadd           v23.8h,  v31.8h,  v28.8h    // t63a
        sqsub           v21.8h,  v30.8h,  v29.8h    // t61
        sqadd           v22.8h,  v30.8h,  v29.8h    // t62

        smull_smlal     v2,  v3,  v21, v18, v1.h[2], v1.h[3], .8h // -> t61a
        smull_smlsl     v4,  v5,  v21, v18, v1.h[3], v1.h[2], .8h // -> t34a
        smull_smlal     v6,  v7,  v20, v19, v1.h[2], v1.h[3], .8h // -> t60
        sqrshrn_sz      v21, v2,  v3,  #12, .8h     // t61a
        sqrshrn_sz      v18, v4,  v5,  #12, .8h     // t34a
        smull_smlsl     v2,  v3,  v20, v19, v1.h[3], v1.h[2], .8h // -> t35
        sqrshrn_sz      v20, v6,  v7,  #12, .8h     // t60
        sqrshrn_sz      v19, v2,  v3,  #12, .8h     // t35

        st1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x6], #64
        st1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x6], #64

        ret
endfunc

function inv_dct64_step2_neon
        movrel          x16, idct_coeffs
        ld1             {v0.4h}, [x16]
1:
        // t32a/33/34a/35/60/61a/62/63a
        // t56a/57/58a/59/36/37a/38/39a
        // t40a/41/42a/43/52/53a/54/55a
        // t48a/49/50a/51/44/45a/46/47a
        ldr             q16, [x6, #2*8*0]  // t32a
        ldr             q17, [x9, #2*8*8]  // t39a
        ldr             q18, [x9, #2*8*0]  // t63a
        ldr             q19, [x6, #2*8*8]  // t56a
        ldr             q20, [x6, #2*8*16] // t40a
        ldr             q21, [x9, #2*8*24] // t47a
        ldr             q22, [x9, #2*8*16] // t55a
        ldr             q23, [x6, #2*8*24] // t48a

        sqadd           v24.8h,  v16.8h, v17.8h // t32
        sqsub           v25.8h,  v16.8h, v17.8h // t39
        sqadd           v26.8h,  v18.8h, v19.8h // t63
        sqsub           v27.8h,  v18.8h, v19.8h // t56
        sqsub           v28.8h,  v21.8h, v20.8h // t40
        sqadd           v29.8h,  v21.8h, v20.8h // t47
        sqadd           v30.8h,  v23.8h, v22.8h // t48
        sqsub           v31.8h,  v23.8h, v22.8h // t55

        smull_smlal     v2,  v3,  v27, v25, v0.h[3], v0.h[2], .8h // -> t56a
        smull_smlsl     v4,  v5,  v27, v25, v0.h[2], v0.h[3], .8h // -> t39a
        smull_smlal     v6,  v7,  v31, v28, v0.h[3], v0.h[2], .8h // -> t40a
        sqrshrn_sz      v25, v2,  v3,  #12, .8h     // t56a
        sqrshrn_sz      v27, v4,  v5,  #12, .8h     // t39a
        neg             v6.4s,   v6.4s              // t40a
        neg             v7.4s,   v7.4s              // t40a
        smull_smlsl     v2,  v3,  v31, v28, v0.h[2], v0.h[3], .8h // -> t55a
        sqrshrn_sz      v31, v6,  v7,  #12, .8h     // t40a
        sqrshrn_sz      v28, v2,  v3,  #12, .8h     // t55a

        sqadd           v16.8h,  v24.8h,  v29.8h    // t32a
        sqsub           v19.8h,  v24.8h,  v29.8h    // t47a
        sqadd           v17.8h,  v27.8h,  v31.8h    // t39
        sqsub           v18.8h,  v27.8h,  v31.8h    // t40
        sqsub           v20.8h,  v26.8h,  v30.8h    // t48a
        sqadd           v23.8h,  v26.8h,  v30.8h    // t63a
        sqsub           v21.8h,  v25.8h,  v28.8h    // t55
        sqadd           v22.8h,  v25.8h,  v28.8h    // t56

        smull_smlsl     v2,  v3,  v21, v18, v0.h[0], v0.h[0], .8h // -> t40a
        smull_smlal     v4,  v5,  v21, v18, v0.h[0], v0.h[0], .8h // -> t55a
        smull_smlsl     v6,  v7,  v20, v19, v0.h[0], v0.h[0], .8h // -> t47
        sqrshrn_sz      v18, v2,  v3,  #12, .8h     // t40a
        sqrshrn_sz      v21, v4,  v5,  #12, .8h     // t55a
        smull_smlal     v2,  v3,  v20, v19, v0.h[0], v0.h[0], .8h // -> t48
        sqrshrn_sz      v19, v6,  v7,  #12, .8h     // t47
        sqrshrn_sz      v20, v2,  v3,  #12, .8h     // t48

        str             q16, [x6, #2*8*0]  // t32a
        str             q17, [x9, #2*8*0]  // t39
        str             q18, [x6, #2*8*8]  // t40a
        str             q19, [x9, #2*8*8]  // t47
        str             q20, [x6, #2*8*16] // t48
        str             q21, [x9, #2*8*16] // t55a
        str             q22, [x6, #2*8*24] // t56
        str             q23, [x9, #2*8*24] // t63a

        add             x6,  x6,  #2*8
        sub             x9,  x9,  #2*8
        cmp             x6,  x9
        b.lt            1b
        ret
endfunc

.macro load8 src, strd, zero, clear
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h
.if \clear
        ld1             {\i}, [\src]
        st1             {\zero}, [\src], \strd
.else
        ld1             {\i}, [\src], \strd
.endif
.endr
.endm

.macro store16 dst
.irp i, v16.8h, v17.8h, v18.8h, v19.8h, v20.8h, v21.8h, v22.8h, v23.8h, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        st1             {\i}, [\dst], #16
.endr
.endm

.macro clear_upper8
.irp i, v24.8h, v25.8h, v26.8h, v27.8h, v28.8h, v29.8h, v30.8h, v31.8h
        movi            \i,  #0
.endr
.endm

.macro movi_if reg, val, cond
.if \cond
        movi            \reg, \val
.endif
.endm

.macro movdup_if reg, gpr, val, cond
.if \cond
        mov             \gpr, \val
        dup             \reg, \gpr
.endif
.endm

.macro st1_if regs, dst, cond
.if \cond
        st1             \regs, \dst
.endif
.endm

.macro str_if reg, dst, cond
.if \cond
        str             \reg, \dst
.endif
.endm

.macro stroff_if reg, dst, dstoff, cond
.if \cond
        str             \reg, \dst, \dstoff
.endif
.endm

.macro scale_if cond, c, r0, r1, r2, r3, r4, r5, r6, r7
.if \cond
        scale_input     .8h, \c, \r0, \r1, \r2, \r3, \r4, \r5, \r6, \r7
.endif
.endm

.macro def_dct64_func suffix, clear=0, scale=0
function inv_txfm_dct\suffix\()_8h_x64_neon, export=1
        mov             x14, x30
        mov             x6,  sp
        lsl             x8,  x8,  #2

        movdup_if       v0.4h, w16, #2896*8, \scale
        movi_if         v7.8h,  #0, \clear
        load8           x7,  x8,  v7.8h, \clear
        clear_upper8
        sub             x7,  x7,  x8, lsl #3
        add             x7,  x7,  x8, lsr #1
        scale_if        \scale, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23

        bl              inv_dct_8h_x16_neon

        store16         x6

        movdup_if       v0.4h, w16, #2896*8, \scale
        movi_if         v7.8h,  #0, \clear
        load8           x7,  x8,  v7.8h, \clear
        clear_upper8
        sub             x7,  x7,  x8, lsl #3
        lsr             x8,  x8,  #1
        sub             x7,  x7,  x8, lsr #1
        scale_if        \scale, v0.h[0], v16, v17, v18, v19, v20, v21, v22, v23

        bl              inv_dct32_odd_8h_x16_neon

        add             x10, x6,  #16*15
        sub             x6,  x6,  #16*16

        mov             x9,  #-16

.macro store_addsub r0, r1, r2, r3
        ld1             {v2.8h}, [x6], #16
        ld1             {v3.8h}, [x6], #16
        sqadd           v6.8h,  v2.8h,  \r0
        sqsub           \r0,    v2.8h,  \r0
        ld1             {v4.8h}, [x6], #16
        sqadd           v7.8h,  v3.8h,  \r1
        sqsub           \r1,    v3.8h,  \r1
        ld1             {v5.8h}, [x6], #16
        sqadd           v2.8h,  v4.8h,  \r2
        sub             x6,  x6,  #16*4
        sqsub           \r2,    v4.8h,  \r2
        st1             {v6.8h}, [x6], #16
        st1             {\r0},   [x10], x9
        sqadd           v3.8h,  v5.8h,  \r3
        sqsub           \r3,    v5.8h,  \r3
        st1             {v7.8h}, [x6], #16
        st1             {\r1},   [x10], x9
        st1             {v2.8h}, [x6], #16
        st1             {\r2},   [x10], x9
        st1             {v3.8h}, [x6], #16
        st1             {\r3},   [x10], x9
.endm
        store_addsub    v31.8h, v30.8h, v29.8h, v28.8h
        store_addsub    v27.8h, v26.8h, v25.8h, v24.8h
        store_addsub    v23.8h, v22.8h, v21.8h, v20.8h
        store_addsub    v19.8h, v18.8h, v17.8h, v16.8h
.purgem store_addsub

        add             x6,  x6,  #2*8*16

        movrel          x17, idct64_coeffs
        movdup_if       v0.4h, w16, #2896*8, \scale
        movi_if         v7.8h,  #0, \clear
        add             x9,  x7,  x8, lsl #4 // offset 16
        add             x10, x7,  x8, lsl #3 // offset 8
        sub             x9,  x9,  x8         // offset 15
        sub             x11, x10, x8         // offset 7
        ld1             {v16.8h}, [x7]  // in1  (offset 0)
        ld1             {v17.8h}, [x9]  // in31 (offset 15)
        ld1             {v18.8h}, [x10] // in17 (offset 8)
        ld1             {v19.8h}, [x11] // in15 (offset 7)
        st1_if          {v7.8h}, [x7],  \clear
        st1_if          {v7.8h}, [x9],  \clear
        st1_if          {v7.8h}, [x10], \clear
        st1_if          {v7.8h}, [x11], \clear
        scale_if        \scale, v0.h[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon
        movdup_if       v0.4h, w16, #2896*8, \scale
        movi_if         v7.8h,  #0, \clear
        add             x7,  x7,  x8, lsl #2 // offset 4
        sub             x9,  x9,  x8, lsl #2 // offset 11
        sub             x10, x7,  x8         // offset 3
        add             x11, x9,  x8         // offset 12
        ld1             {v16.8h}, [x10] // in7  (offset 3)
        ld1             {v17.8h}, [x11] // in25 (offset 12)
        ld1             {v18.8h}, [x9]  // in23 (offset 11)
        ld1             {v19.8h}, [x7]  // in9  (offset 4)
        st1_if          {v7.8h}, [x7],  \clear
        st1_if          {v7.8h}, [x9],  \clear
        st1_if          {v7.8h}, [x10], \clear
        st1_if          {v7.8h}, [x11], \clear
        scale_if        \scale, v0.h[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon
        movdup_if       v0.4h, w16, #2896*8, \scale
        movi_if         v7.8h,  #0, \clear
        sub             x10, x10, x8, lsl #1 // offset 1
        sub             x9,  x9,  x8, lsl #1 // offset 9
        add             x7,  x7,  x8         // offset 5
        add             x11, x11, x8         // offset 13
        ldr             q16, [x10, x8] // in5  (offset 2)
        ldr             q17, [x11]     // in27 (offset 13)
        ldr             q18, [x9,  x8] // in21 (offset 10)
        ldr             q19, [x7]      // in11 (offset 5)
        stroff_if       q7,  [x10, x8], \clear
        str_if          q7,  [x11],     \clear
        stroff_if       q7,  [x9,  x8], \clear
        str_if          q7,  [x7],      \clear
        scale_if        \scale, v0.h[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon
        movdup_if       v0.4h, w16, #2896*8, \scale
        movi_if         v7.8h,  #0, \clear
        ldr             q16, [x10]     // in3  (offset 1)
        ldr             q17, [x11, x8] // in29 (offset 14)
        ldr             q18, [x9]      // in19 (offset 9)
        ldr             q19, [x7,  x8] // in13 (offset 6)
        str_if          q7,  [x10],     \clear
        stroff_if       q7,  [x11, x8], \clear
        str_if          q7,  [x9],      \clear
        stroff_if       q7,  [x7,  x8], \clear
        scale_if        \scale, v0.h[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon

        sub             x6,  x6,  #2*8*32
        add             x9,  x6,  #2*8*7

        bl              inv_dct64_step2_neon

        ret             x14
endfunc
.endm

def_dct64_func
def_dct64_func _clear, clear=1
def_dct64_func _clear_scale, clear=1, scale=1


function inv_txfm_horz_dct_64x8_neon
        mov             x14, x30

        mov             x7,  sp
        add             x8,  sp,  #2*8*(64 - 4)
        add             x9,  x6,  #2*56
        mov             x10, #2*64
        mov             x11, #-2*8*4

        dup             v7.8h,  w12
1:
        ld1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x7], #64
        ld1             {v28.8h, v29.8h, v30.8h, v31.8h}, [x8], x11
        ld1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x7], #64
        ld1             {v24.8h, v25.8h, v26.8h, v27.8h}, [x8], x11
        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5
        transpose_8x8h  v31, v30, v29, v28, v27, v26, v25, v24, v4, v5

.macro store_addsub src0, src1, src2, src3
        sqsub           v1.8h,   \src0,   \src1
        sqadd           v0.8h,   \src0,   \src1
        sqsub           v3.8h,   \src2,   \src3
        srshl           v1.8h,   v1.8h,   v7.8h
        sqadd           v2.8h,   \src2,   \src3
        srshl           v0.8h,   v0.8h,   v7.8h
        srshl           v3.8h,   v3.8h,   v7.8h
        rev64           v1.8h,   v1.8h
        srshl           v2.8h,   v2.8h,   v7.8h
        rev64           v3.8h,   v3.8h
        ext             v1.16b,  v1.16b,  v1.16b,  #8
        st1             {v0.8h},  [x6], x10
        ext             v3.16b,  v3.16b,  v3.16b,  #8
        st1             {v1.8h},  [x9], x10
        st1             {v2.8h},  [x6], x10
        st1             {v3.8h},  [x9], x10
.endm
        store_addsub    v16.8h,  v31.8h,  v17.8h,  v30.8h
        store_addsub    v18.8h,  v29.8h,  v19.8h,  v28.8h
        store_addsub    v20.8h,  v27.8h,  v21.8h,  v26.8h
        store_addsub    v22.8h,  v25.8h,  v23.8h,  v24.8h
.purgem store_addsub
        sub             x6,  x6,  x10, lsl #3
        sub             x9,  x9,  x10, lsl #3
        add             x6,  x6,  #16
        sub             x9,  x9,  #16

        cmp             x7,  x8
        b.lt            1b
        ret             x14
endfunc

function inv_txfm_add_vert_dct_8x64_neon
        mov             x14, x30
        lsl             x8,  x8,  #1

        mov             x7,  sp
        add             x8,  sp,  #2*8*(64 - 4)
        add             x9,  x6,  x1, lsl #6
        sub             x9,  x9,  x1
        neg             x10, x1
        mov             x11, #-2*8*4

1:
        ld1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x7], #64
        ld1             {v28.8h, v29.8h, v30.8h, v31.8h}, [x8], x11
        ld1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x7], #64
        ld1             {v24.8h, v25.8h, v26.8h, v27.8h}, [x8], x11

.macro add_dest_addsub src0, src1, src2, src3
        ld1             {v0.8b}, [x6], x1
        ld1             {v1.8b}, [x9], x10
        sqadd           v4.8h,   \src0,   \src1
        ld1             {v2.8b}, [x6]
        sqsub           v5.8h,   \src0,   \src1
        ld1             {v3.8b}, [x9]
        sqadd           v6.8h,   \src2,   \src3
        sqsub           v7.8h,   \src2,   \src3
        sub             x6,  x6,  x1
        sub             x9,  x9,  x10
        srshr           v4.8h,   v4.8h,   #4
        srshr           v5.8h,   v5.8h,   #4
        srshr           v6.8h,   v6.8h,   #4
        uaddw           v4.8h,   v4.8h,   v0.8b
        srshr           v7.8h,   v7.8h,   #4
        uaddw           v5.8h,   v5.8h,   v1.8b
        uaddw           v6.8h,   v6.8h,   v2.8b
        sqxtun          v0.8b,   v4.8h
        uaddw           v7.8h,   v7.8h,   v3.8b
        sqxtun          v1.8b,   v5.8h
        st1             {v0.8b}, [x6], x1
        sqxtun          v2.8b,   v6.8h
        st1             {v1.8b}, [x9], x10
        sqxtun          v3.8b,   v7.8h
        st1             {v2.8b}, [x6], x1
        st1             {v3.8b}, [x9], x10
.endm
        add_dest_addsub v16.8h,  v31.8h,  v17.8h,  v30.8h
        add_dest_addsub v18.8h,  v29.8h,  v19.8h,  v28.8h
        add_dest_addsub v20.8h,  v27.8h,  v21.8h,  v26.8h
        add_dest_addsub v22.8h,  v25.8h,  v23.8h,  v24.8h
.purgem add_dest_addsub
        cmp             x7,  x8
        b.lt            1b

        ret             x14
endfunc

function inv_txfm_add_dct_dct_64x64_8bpc_neon, export=1
        idct_dc         64,  64,  2

        mov             x15, x30

        sub_sp          64*32*2+64*8*2
        add             x5,  sp, #64*8*2

        movrel          x13, eob_32x32

.irp i, 0, 8, 16, 24
        add             x6,  x5,  #(\i*64*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.endif
        add             x7,  x2,  #(\i*2)
        mov             x8,  #32*2
        mov             x12, #-2 // shift
        bl              inv_txfm_dct_clear_8h_x64_neon
        add             x6,  x5,  #(\i*64*2)
        bl              inv_txfm_horz_dct_64x8_neon
.if \i < 24
        ldrh            w12, [x13], #2
.endif
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #2
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8, 16, 24, 32, 40, 48, 56
        add             x7,  x5,  #(\i*2)
        mov             x8,  #64*2
        bl              inv_txfm_dct_8h_x64_neon
        add             x6,  x0,  #(\i)
        bl              inv_txfm_add_vert_dct_8x64_neon
.endr

        add             sp,  x5,  #64*32*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_64x32_8bpc_neon, export=1
        idct_dc         64,  32,  1

        mov             x15, x30

        sub_sp          64*32*2+64*8*2
        add             x5,  sp, #64*8*2

        movrel          x13, eob_32x32

.irp i, 0, 8, 16, 24
        add             x6,  x5,  #(\i*64*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.endif
        add             x7,  x2,  #(\i*2)
        mov             x8,  #32*2
        mov             x12, #-1 // shift
        bl              inv_txfm_dct_clear_scale_8h_x64_neon
        add             x6,  x5,  #(\i*64*2)
        bl              inv_txfm_horz_dct_64x8_neon
.if \i < 24
        ldrh            w12, [x13], #2
.endif
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #2
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8, 16, 24, 32, 40, 48, 56
        add             x6,  x0,  #(\i)
        add             x7,  x5,  #(\i*2)
        mov             x8,  #64*2
        bl              inv_txfm_add_vert_dct_8x32_neon
.endr

        add             sp,  x5,  #64*32*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_32x64_8bpc_neon, export=1
        idct_dc         32,  64,  1

        mov             x15, x30

        sub_sp          32*32*2+64*8*2
        add             x5,  sp, #64*8*2

        movrel          x13, eob_32x32
        ldrh            w12, [x13], #2

.irp i, 0, 8, 16, 24
        add             x6,  x5,  #(\i*32*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 24
        ldrh            w12, [x13], #2
.endif
.endif
        add             x7,  x2,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_horz_scale_dct_32x8_neon
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #4
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8, 16, 24
        add             x7,  x5,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_dct_8h_x64_neon
        add             x6,  x0,  #(\i)
        bl              inv_txfm_add_vert_dct_8x64_neon
.endr

        add             sp,  x5,  #32*32*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_64x16_8bpc_neon, export=1
        idct_dc         64,  16,  2

        mov             x15, x30

        sub_sp          64*16*2+64*8*2
        add             x4,  sp, #64*8*2

        movrel          x13, eob_16x32

.irp i, 0, 8
        add             x6,  x4,  #(\i*64*2)
.if \i > 0
        mov             w8,  #(16 - \i)
        cmp             w3,  w12
        b.lt            1f
.endif
        add             x7,  x2,  #(\i*2)
        mov             x8,  #16*2
        mov             x12, #-2 // shift
        bl              inv_txfm_dct_clear_8h_x64_neon
        add             x6,  x4,  #(\i*64*2)
        bl              inv_txfm_horz_dct_64x8_neon
.if \i < 8
        ldrh            w12, [x13], #2
.endif
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #2
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
        adr             x5,  inv_dct_8h_x16_neon
.irp i, 0, 8, 16, 24, 32, 40, 48, 56
        add             x6,  x0,  #(\i)
        add             x7,  x4,  #(\i*2)
        mov             x8,  #64*2
        bl              inv_txfm_add_vert_8x16_neon
.endr

        add             sp,  x4,  #64*16*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_16x64_8bpc_neon, export=1
        idct_dc         16,  64,  2

        mov             x15, x30

        sub_sp          16*32*2+64*8*2
        add             x5,  sp, #64*8*2

        movrel          x13, eob_16x32
        ldrh            w12, [x13], #2

        adr             x4,  inv_dct_8h_x16_neon
.irp i, 0, 8, 16, 24
        add             x6,  x5,  #(\i*16*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 24
        ldrh            w12, [x13], #2
.endif
.endif
        add             x7,  x2,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_horz_16x8_neon
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #8
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8
        add             x7,  x5,  #(\i*2)
        mov             x8,  #16*2
        bl              inv_txfm_dct_8h_x64_neon
        add             x6,  x0,  #(\i)
        bl              inv_txfm_add_vert_dct_8x64_neon
.endr

        add             sp,  x5,  #16*32*2
        ret             x15
endfunc
