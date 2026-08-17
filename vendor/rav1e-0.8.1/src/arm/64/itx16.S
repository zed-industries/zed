/******************************************************************************
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2020, Martin Storsjo
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
// void itxfm_add(pixel *dst, ptrdiff_t dst_stride, coef *coeff, int eob,
//                int bitdepth_max);

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

const idct_coeffs, align=4
        // idct4
        .int            2896, 2896*8*(1<<16), 1567, 3784
        // idct8
        .int            799, 4017, 3406, 2276
        // idct16
        .int            401, 4076, 3166, 2598
        .int            1931, 3612, 3920, 1189
        // idct32
        .int            201, 4091, 3035, 2751
        .int            1751, 3703, 3857, 1380
        .int            995, 3973, 3513, 2106
        .int            2440, 3290, 4052, 601
endconst

const idct64_coeffs, align=4
        .int            101*8*(1<<16), 4095*8*(1<<16), 2967*8*(1<<16), -2824*8*(1<<16)
        .int            1660*8*(1<<16), 3745*8*(1<<16), 3822*8*(1<<16), -1474*8*(1<<16)
        .int            4076, 401, 4017, 799

        .int            4036*8*(1<<16), -700*8*(1<<16), 2359*8*(1<<16), 3349*8*(1<<16)
        .int            3461*8*(1<<16), -2191*8*(1<<16), 897*8*(1<<16), 3996*8*(1<<16)
        .int            -3166, -2598, -799, -4017

        .int            501*8*(1<<16), 4065*8*(1<<16), 3229*8*(1<<16), -2520*8*(1<<16)
        .int            2019*8*(1<<16), 3564*8*(1<<16), 3948*8*(1<<16), -1092*8*(1<<16)
        .int            3612, 1931, 2276, 3406

        .int            4085*8*(1<<16), -301*8*(1<<16), 2675*8*(1<<16), 3102*8*(1<<16)
        .int            3659*8*(1<<16), -1842*8*(1<<16), 1285*8*(1<<16), 3889*8*(1<<16)
        .int            -3920, -1189, -3406, -2276
endconst

const iadst4_coeffs, align=4
        .int            1321, 3803, 2482, 3344
endconst

const iadst8_coeffs, align=4
        .int            4076, 401, 3612, 1931
        .int            2598, 3166, 1189, 3920
        // idct_coeffs
        .int            2896, 0, 1567, 3784
endconst

const iadst16_coeffs, align=4
        .int            4091, 201, 3973, 995
        .int            3703, 1751, 3290, 2440
        .int            2751, 3035, 2106, 3513
        .int            1380, 3857, 601, 4052
endconst

.macro mul_mla d, s0, s1, c0, c1
        mul             \d\().4s, \s0\().4s, \c0
        mla             \d\().4s, \s1\().4s, \c1
.endm

.macro mul_mls d, s0, s1, c0, c1
        mul             \d\().4s, \s0\().4s, \c0
        mls             \d\().4s, \s1\().4s, \c1
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

.macro smin_4s r0, r1, r2
        smin            \r0\().4s, \r1\().4s, \r2\().4s
.endm
.macro smax_4s r0, r1, r2
        smax            \r0\().4s, \r1\().4s, \r2\().4s
.endm

.macro load_add_store load, shift, addsrc, adddst, min, store, dst, src, shiftbits=4
.ifnb \load
        ld1             {\load},  [\src], x1
.endif
.ifnb \shift
        srshr           \shift,  \shift,  #\shiftbits
.endif
.ifnb \addsrc
        usqadd          \adddst, \addsrc
.endif
.ifnb \min
        smin            \min,  \min,  v7.8h
.endif
.ifnb \store
        st1             {\store},  [\dst], x1
.endif
.endm
.macro load_add_store_8x16 dst, src
        mov             \src, \dst
        mvni            v7.8h,   #0xfc, lsl #8 // 0x3ff
        load_add_store  v2.8h,  v16.8h,       ,      ,       ,       ,  \dst, \src
        load_add_store  v3.8h,  v17.8h,       ,      ,       ,       ,  \dst, \src
        load_add_store  v4.8h,  v18.8h, v16.8h, v2.8h,       ,       ,  \dst, \src
        load_add_store  v5.8h,  v19.8h, v17.8h, v3.8h,  v2.8h,       ,  \dst, \src
        load_add_store  v16.8h, v20.8h, v18.8h, v4.8h,  v3.8h,  v2.8h,  \dst, \src
        load_add_store  v17.8h, v21.8h, v19.8h, v5.8h,  v4.8h,  v3.8h,  \dst, \src
        load_add_store  v18.8h, v22.8h, v20.8h, v16.8h, v5.8h,  v4.8h,  \dst, \src
        load_add_store  v19.8h, v23.8h, v21.8h, v17.8h, v16.8h, v5.8h,  \dst, \src
        load_add_store  v20.8h, v24.8h, v22.8h, v18.8h, v17.8h, v16.8h, \dst, \src
        load_add_store  v21.8h, v25.8h, v23.8h, v19.8h, v18.8h, v17.8h, \dst, \src
        load_add_store  v22.8h, v26.8h, v24.8h, v20.8h, v19.8h, v18.8h, \dst, \src
        load_add_store  v23.8h, v27.8h, v25.8h, v21.8h, v20.8h, v19.8h, \dst, \src
        load_add_store  v24.8h, v28.8h, v26.8h, v22.8h, v21.8h, v20.8h, \dst, \src
        load_add_store  v25.8h, v29.8h, v27.8h, v23.8h, v22.8h, v21.8h, \dst, \src
        load_add_store  v26.8h, v30.8h, v28.8h, v24.8h, v23.8h, v22.8h, \dst, \src
        load_add_store  v27.8h, v31.8h, v29.8h, v25.8h, v24.8h, v23.8h, \dst, \src
        load_add_store        ,       , v30.8h, v26.8h, v25.8h, v24.8h, \dst, \src
        load_add_store        ,       , v31.8h, v27.8h, v26.8h, v25.8h, \dst, \src
        load_add_store        ,       ,       ,       , v27.8h, v26.8h, \dst, \src
        load_add_store        ,       ,       ,       ,       , v27.8h, \dst, \src
.endm
.macro load_add_store_8x8 dst, src, shiftbits=4
        mov             \src, \dst
        mvni            v7.8h,   #0xfc, lsl #8 // 0x3ff
        load_add_store  v2.8h,  v16.8h,       ,      ,       ,       ,  \dst, \src, \shiftbits
        load_add_store  v3.8h,  v17.8h,       ,      ,       ,       ,  \dst, \src, \shiftbits
        load_add_store  v4.8h,  v18.8h, v16.8h, v2.8h,       ,       ,  \dst, \src, \shiftbits
        load_add_store  v5.8h,  v19.8h, v17.8h, v3.8h,  v2.8h,       ,  \dst, \src, \shiftbits
        load_add_store  v16.8h, v20.8h, v18.8h, v4.8h,  v3.8h,  v2.8h,  \dst, \src, \shiftbits
        load_add_store  v17.8h, v21.8h, v19.8h, v5.8h,  v4.8h,  v3.8h,  \dst, \src, \shiftbits
        load_add_store  v18.8h, v22.8h, v20.8h, v16.8h, v5.8h,  v4.8h,  \dst, \src, \shiftbits
        load_add_store  v19.8h, v23.8h, v21.8h, v17.8h, v16.8h, v5.8h,  \dst, \src, \shiftbits
        load_add_store        ,       , v22.8h, v18.8h, v17.8h, v16.8h, \dst, \src, \shiftbits
        load_add_store        ,       , v23.8h, v19.8h, v18.8h, v17.8h, \dst, \src, \shiftbits
        load_add_store        ,       ,       ,       , v19.8h, v18.8h, \dst, \src, \shiftbits
        load_add_store        ,       ,       ,       ,       , v19.8h, \dst, \src, \shiftbits
.endm
.macro load_add_store_8x4 dst, src, shiftbits=4
        mov             \src, \dst
        mvni            v7.8h,   #0xfc, lsl #8 // 0x3ff
        load_add_store  v2.8h, v16.8h,       ,      ,      ,      , \dst, \src, \shiftbits
        load_add_store  v3.8h, v17.8h,       ,      ,      ,      , \dst, \src, \shiftbits
        load_add_store  v4.8h, v18.8h, v16.8h, v2.8h,      ,      , \dst, \src, \shiftbits
        load_add_store  v5.8h, v19.8h, v17.8h, v3.8h, v2.8h,      , \dst, \src, \shiftbits
        load_add_store       ,       , v18.8h, v4.8h, v3.8h, v2.8h, \dst, \src, \shiftbits
        load_add_store       ,       , v19.8h, v5.8h, v4.8h, v3.8h, \dst, \src, \shiftbits
        load_add_store       ,       ,       ,      , v5.8h, v4.8h, \dst, \src, \shiftbits
        load_add_store       ,       ,       ,      ,      , v5.8h, \dst, \src, \shiftbits
.endm
.macro load_add_store4 load, inssrc, insdst, shift, addsrc, adddst, min, store, dst, src
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
        usqadd          \adddst, \addsrc
.endif
.ifnb \store
        st1             {\store}[0],  [\dst], x1
.endif
.ifnb \min
        smin            \min,  \min,  v7.8h
.endif
.ifnb \store
        st1             {\store}[1],  [\dst], x1
.endif
.endm
.macro load_add_store_4x16 dst, src
        mov             \src, \dst
        mvni            v7.8h,   #0xfc, lsl #8 // 0x3ff
        load_add_store4 v0.d,  v17, v16,       ,       ,      ,       ,      ,  \dst, \src
        load_add_store4 v1.d,  v19, v18,       ,       ,      ,       ,      ,  \dst, \src
        load_add_store4 v2.d,  v21, v20, v16.8h,       ,      ,       ,      ,  \dst, \src
        load_add_store4 v3.d,  v23, v22, v18.8h, v16.8h, v0.8h,       ,      ,  \dst, \src
        load_add_store4 v17.d, v25, v24, v20.8h, v18.8h, v1.8h,  v0.8h,      ,  \dst, \src
        load_add_store4 v19.d, v27, v26, v22.8h, v20.8h, v2.8h,  v1.8h,  v0.d,  \dst, \src
        load_add_store4 v21.d, v29, v28, v24.8h, v22.8h, v3.8h,  v2.8h,  v1.d,  \dst, \src
        load_add_store4 v23.d, v31, v30, v26.8h, v24.8h, v17.8h, v3.8h,  v2.d,  \dst, \src
        load_add_store4      ,    ,    , v28.8h, v26.8h, v19.8h, v17.8h, v3.d,  \dst, \src
        load_add_store4      ,    ,    , v30.8h, v28.8h, v21.8h, v19.8h, v17.d, \dst, \src
        load_add_store4      ,    ,    ,       , v30.8h, v23.8h, v21.8h, v19.d, \dst, \src
        load_add_store4      ,    ,    ,       ,      ,        , v23.8h, v21.d, \dst, \src
        load_add_store4      ,    ,    ,       ,      ,        ,       , v23.d, \dst, \src
.endm
.macro load_add_store_4x8 dst, src
        mov             \src, \dst
        mvni            v7.8h,   #0xfc, lsl #8 // 0x3ff
        load_add_store4 v0.d, v17, v16,       ,       ,      ,      ,     , \dst, \src
        load_add_store4 v1.d, v19, v18,       ,       ,      ,      ,     , \dst, \src
        load_add_store4 v2.d, v21, v20, v16.8h,       ,      ,      ,     , \dst, \src
        load_add_store4 v3.d, v23, v22, v18.8h, v16.8h, v0.8h,      ,     , \dst, \src
        load_add_store4     ,    ,    , v20.8h, v18.8h, v1.8h, v0.8h,     , \dst, \src
        load_add_store4     ,    ,    , v22.8h, v20.8h, v2.8h, v1.8h, v0.d, \dst, \src
        load_add_store4     ,    ,    ,       , v22.8h, v3.8h, v2.8h, v1.d, \dst, \src
        load_add_store4     ,    ,    ,       ,       ,      , v3.8h, v2.d, \dst, \src
        load_add_store4     ,    ,    ,       ,       ,      ,      , v3.d, \dst, \src
.endm

.macro idct_dc w, h, shift
        cbnz            w3,  1f
        movz            w16, #2896*8, lsl #16
        ld1r            {v16.4s}, [x2]
        dup             v0.2s,   w16
        sqrdmulh        v20.4s,  v16.4s,  v0.s[0]
        str             wzr, [x2]
.if (\w == 2*\h) || (2*\w == \h)
        sqrdmulh        v20.4s,  v20.4s,  v0.s[0]
.endif
.if \shift > 0
        sqrshrn         v16.4h,  v20.4s,  #\shift
        sqrshrn2        v16.8h,  v20.4s,  #\shift
.else
        sqxtn           v16.4h,  v20.4s
        sqxtn2          v16.8h,  v20.4s
.endif
        sqrdmulh        v16.8h,  v16.8h,  v0.h[1]
        srshr           v16.8h,  v16.8h,  #4
        mov             w4,  #\h
        b               idct_dc_w\w\()_neon
1:
.endm

function idct_dc_w4_neon
        mvni            v31.8h,  #0xfc, lsl #8 // 0x3ff
1:
        ld1             {v0.d}[0], [x0], x1
        ld1             {v0.d}[1], [x0], x1
        ld1             {v1.d}[0], [x0], x1
        subs            w4,  w4,  #4
        ld1             {v1.d}[1], [x0], x1
        usqadd          v0.8h,   v16.8h
        sub             x0,  x0,  x1, lsl #2
        usqadd          v1.8h,   v16.8h
        smin            v0.8h,   v0.8h,   v31.8h
        st1             {v0.d}[0], [x0], x1
        smin            v1.8h,   v1.8h,   v31.8h
        st1             {v0.d}[1], [x0], x1
        st1             {v1.d}[0], [x0], x1
        st1             {v1.d}[1], [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w8_neon
        mvni            v31.8h,  #0xfc, lsl #8 // 0x3ff
1:
        ld1             {v0.8h}, [x0], x1
        subs            w4,  w4,  #4
        ld1             {v1.8h}, [x0], x1
        usqadd          v0.8h,   v16.8h
        ld1             {v2.8h}, [x0], x1
        usqadd          v1.8h,   v16.8h
        ld1             {v3.8h}, [x0], x1
        usqadd          v2.8h,   v16.8h
        usqadd          v3.8h,   v16.8h
        sub             x0,  x0,  x1, lsl #2
        smin            v0.8h,   v0.8h,   v31.8h
        smin            v1.8h,   v1.8h,   v31.8h
        st1             {v0.8h}, [x0], x1
        smin            v2.8h,   v2.8h,   v31.8h
        st1             {v1.8h}, [x0], x1
        smin            v3.8h,   v3.8h,   v31.8h
        st1             {v2.8h}, [x0], x1
        st1             {v3.8h}, [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w16_neon
        mvni            v31.8h,  #0xfc, lsl #8 // 0x3ff
1:
        ld1             {v0.8h, v1.8h}, [x0], x1
        subs            w4,  w4,  #2
        ld1             {v2.8h, v3.8h}, [x0], x1
        usqadd          v0.8h,   v16.8h
        usqadd          v1.8h,   v16.8h
        sub             x0,  x0,  x1, lsl #1
        usqadd          v2.8h,   v16.8h
        usqadd          v3.8h,   v16.8h
        smin            v0.8h,   v0.8h,   v31.8h
        smin            v1.8h,   v1.8h,   v31.8h
        smin            v2.8h,   v2.8h,   v31.8h
        st1             {v0.8h, v1.8h}, [x0], x1
        smin            v3.8h,   v3.8h,   v31.8h
        st1             {v2.8h, v3.8h}, [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w32_neon
        mvni            v31.8h,  #0xfc, lsl #8 // 0x3ff
1:
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0]
        subs            w4,  w4,  #1
        usqadd          v0.8h,   v16.8h
        usqadd          v1.8h,   v16.8h
        usqadd          v2.8h,   v16.8h
        usqadd          v3.8h,   v16.8h
        smin            v0.8h,   v0.8h,   v31.8h
        smin            v1.8h,   v1.8h,   v31.8h
        smin            v2.8h,   v2.8h,   v31.8h
        smin            v3.8h,   v3.8h,   v31.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        b.gt            1b
        ret
endfunc

function idct_dc_w64_neon
        mvni            v31.8h,  #0xfc, lsl #8 // 0x3ff
        sub             x1,  x1,  #64
1:
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        subs            w4,  w4,  #1
        usqadd          v0.8h,   v16.8h
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0]
        usqadd          v1.8h,   v16.8h
        sub             x0,  x0,  #64
        usqadd          v2.8h,   v16.8h
        usqadd          v3.8h,   v16.8h
        usqadd          v4.8h,   v16.8h
        usqadd          v5.8h,   v16.8h
        usqadd          v6.8h,   v16.8h
        usqadd          v7.8h,   v16.8h
        smin            v0.8h,   v0.8h,   v31.8h
        smin            v1.8h,   v1.8h,   v31.8h
        smin            v2.8h,   v2.8h,   v31.8h
        smin            v3.8h,   v3.8h,   v31.8h
        smin            v4.8h,   v4.8h,   v31.8h
        smin            v5.8h,   v5.8h,   v31.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        smin            v6.8h,   v6.8h,   v31.8h
        smin            v7.8h,   v7.8h,   v31.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], x1
        b.gt            1b
        ret
endfunc

.macro iwht4
        add             v16.4s,  v16.4s,  v17.4s
        sub             v21.4s,  v18.4s,  v19.4s
        sub             v20.4s,  v16.4s,  v21.4s
        sshr            v20.4s,  v20.4s,  #1
        sub             v18.4s,  v20.4s,  v17.4s
        sub             v17.4s,  v20.4s,  v19.4s
        add             v19.4s,  v21.4s,  v18.4s
        sub             v16.4s,  v16.4s,  v17.4s
.endm

.macro idct_4 r0, r1, r2, r3
        mul_mla         v6,  \r1, \r3, v0.s[3], v0.s[2]
        mul_mla         v2,  \r0, \r2, v0.s[0], v0.s[0]
        mul_mls         v4,  \r1, \r3, v0.s[2], v0.s[3]
        mul_mls         v3,  \r0, \r2, v0.s[0], v0.s[0]
        srshr           v6.4s,  v6.4s,  #12
        srshr           v2.4s,  v2.4s,  #12
        srshr           v7.4s,  v4.4s,  #12
        srshr           v3.4s,  v3.4s,  #12
        sqadd           \r0\().4s,  v2.4s,   v6.4s
        sqsub           \r3\().4s,  v2.4s,   v6.4s
        sqadd           \r1\().4s,  v3.4s,   v7.4s
        sqsub           \r2\().4s,  v3.4s,   v7.4s
.endm

function inv_dct_4s_x4_neon
        AARCH64_VALID_CALL_TARGET
        movrel          x16, idct_coeffs
        ld1             {v0.4s}, [x16]
        idct_4          v16, v17, v18, v19
        ret
endfunc

.macro iadst_4x4 o0, o1, o2, o3
        movrel          x16, iadst4_coeffs
        ld1             {v0.4s}, [x16]

        sub             v3.4s,   v16.4s,  v18.4s
        mul             v4.4s,   v16.4s,  v0.s[0]
        mla             v4.4s,   v18.4s,  v0.s[1]
        mla             v4.4s,   v19.4s,  v0.s[2]
        mul             v7.4s,   v17.4s,  v0.s[3]
        add             v3.4s,   v3.4s,   v19.4s
        mul             v5.4s,   v16.4s,  v0.s[2]
        mls             v5.4s,   v18.4s,  v0.s[0]
        mls             v5.4s,   v19.4s,  v0.s[1]

        add             \o3\().4s, v4.4s,     v5.4s
        mul             \o2\().4s, v3.4s,     v0.s[3]
        add             \o0\().4s, v4.4s,     v7.4s
        add             \o1\().4s, v5.4s,     v7.4s
        sub             \o3\().4s, \o3\().4s, v7.4s

        srshr           \o0\().4s, \o0\().4s, #12
        srshr           \o2\().4s, \o2\().4s, #12
        srshr           \o1\().4s, \o1\().4s, #12
        srshr           \o3\().4s, \o3\().4s, #12
.endm

function inv_adst_4s_x4_neon
        AARCH64_VALID_CALL_TARGET
        iadst_4x4       v16, v17, v18, v19
        ret
endfunc

function inv_flipadst_4s_x4_neon
        AARCH64_VALID_CALL_TARGET
        iadst_4x4       v19, v18, v17, v16
        ret
endfunc

function inv_identity_4s_x4_neon
        AARCH64_VALID_CALL_TARGET
        movz            w16, #(5793-4096)*8, lsl #16
        dup             v0.2s,   w16
        sqrdmulh        v4.4s,   v16.4s,  v0.s[0]
        sqrdmulh        v5.4s,   v17.4s,  v0.s[0]
        sqrdmulh        v6.4s,   v18.4s,  v0.s[0]
        sqrdmulh        v7.4s,   v19.4s,  v0.s[0]
        sqadd           v16.4s,  v16.4s,  v4.4s
        sqadd           v17.4s,  v17.4s,  v5.4s
        sqadd           v18.4s,  v18.4s,  v6.4s
        sqadd           v19.4s,  v19.4s,  v7.4s
        ret
endfunc

function inv_txfm_add_wht_wht_4x4_16bpc_neon, export=1
        mov             x15, x30
        movi            v30.4s,  #0
        movi            v31.4s,  #0
        ld1             {v16.4s,v17.4s,v18.4s,v19.4s}, [x2]
        st1             {v30.4s, v31.4s}, [x2], #32

        sshr            v16.4s,  v16.4s,  #2
        sshr            v17.4s,  v17.4s,  #2
        sshr            v18.4s,  v18.4s,  #2
        sshr            v19.4s,  v19.4s,  #2

        iwht4

        st1             {v30.4s, v31.4s}, [x2], #32
        transpose_4x4s  v16, v17, v18, v19, v20, v21, v22, v23

        iwht4

        ld1             {v0.d}[0], [x0], x1
        sqxtn           v16.4h,  v16.4s
        ld1             {v0.d}[1], [x0], x1
        sqxtn2          v16.8h,  v17.4s
        ld1             {v1.d}[0], [x0], x1
        sqxtn           v18.4h,  v18.4s
        ld1             {v1.d}[1], [x0], x1
        sqxtn2          v18.8h,  v19.4s

        b               L(itx_4x4_end)
endfunc

function inv_txfm_add_4x4_neon
        movi            v30.4s,  #0
        movi            v31.4s,  #0
        ld1             {v16.4s,v17.4s,v18.4s,v19.4s}, [x2]
        st1             {v30.4s, v31.4s}, [x2], #32

        blr             x4

        st1             {v30.4s, v31.4s}, [x2], #32
        sqxtn           v16.4h,  v16.4s
        sqxtn           v17.4h,  v17.4s
        sqxtn           v18.4h,  v18.4s
        sqxtn           v19.4h,  v19.4s
        transpose_4x4h  v16, v17, v18, v19, v20, v21, v22, v23

        blr             x5

        ld1             {v0.d}[0], [x0], x1
        ld1             {v0.d}[1], [x0], x1
        ins             v16.d[1], v17.d[0]
        ins             v18.d[1], v19.d[0]
        ld1             {v1.d}[0], [x0], x1
        ld1             {v1.d}[1], [x0], x1
        srshr           v16.8h,  v16.8h,  #4
        srshr           v18.8h,  v18.8h,  #4

L(itx_4x4_end):
        mvni            v31.8h,  #0xfc, lsl #8 // 0x3ff
        sub             x0,  x0,  x1, lsl #2
        usqadd          v0.8h,   v16.8h
        usqadd          v1.8h,   v18.8h
        smin            v0.8h,   v0.8h,   v31.8h
        st1             {v0.d}[0], [x0], x1
        smin            v1.8h,   v1.8h,   v31.8h
        st1             {v0.d}[1], [x0], x1
        st1             {v1.d}[0], [x0], x1
        st1             {v1.d}[1], [x0], x1

        ret             x15
endfunc

.macro def_fn_4x4 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_4x4_16bpc_neon, export=1
        mov             x15, x30

.ifc \txfm1\()_\txfm2, dct_dct
        cbnz            w3,  1f
        movz            w16, #2896*8, lsl #16
        ld1r            {v16.4s}, [x2]
        dup             v4.2s,   w16
        str             wzr, [x2]
        sqrdmulh        v16.4s,  v16.4s,  v4.s[0]
        ld1             {v0.d}[0], [x0], x1
        sqxtn           v20.4h,  v16.4s
        sqxtn2          v20.8h,  v16.4s
        ld1             {v0.d}[1], [x0], x1
        sqrdmulh        v20.8h,  v20.8h,  v4.h[1]
        ld1             {v1.d}[0], [x0], x1
        srshr           v16.8h,  v20.8h,  #4
        ld1             {v1.d}[1], [x0], x1
        srshr           v18.8h,  v20.8h,  #4
        movi            v30.8h,  #0
        b               L(itx_4x4_end)
1:
.endif
        adr             x4,  inv_\txfm1\()_4s_x4_neon
        movrel          x5,  X(inv_\txfm2\()_4h_x4_neon)
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

.macro idct_8 r0, r1, r2, r3, r4, r5, r6, r7
        idct_4          \r0, \r2, \r4, \r6

        movi            v5.4s,  #1, msl #16 // row_clip_max = ~(~bdmax << 7), 0x1ffff
        mvni            v4.4s,  #1, msl #16 // row_clip_min = (~bdmax << 7), 0xfffe0000
.irp r, \r0, \r2, \r4, \r6
        smin_4s         \r, \r, v5
.endr
.irp r, \r0, \r2, \r4, \r6
        smax_4s         \r, \r, v4
.endr

        mul_mls         v2,  \r1, \r7, v1.s[0], v1.s[1]  // -> t4a
        mul_mla         v3,  \r1, \r7, v1.s[1], v1.s[0]  // -> t7a
        mul_mls         v6,  \r5, \r3, v1.s[2], v1.s[3]  // -> t5a
        mul_mla         v7,  \r5, \r3, v1.s[3], v1.s[2]  // -> t6a
        srshr           \r1\().4s, v2.4s,  #12           // t4a
        srshr           \r7\().4s, v3.4s,  #12           // t7a
        srshr           \r3\().4s, v6.4s,  #12           // t5a
        srshr           \r5\().4s, v7.4s,  #12           // t6a

        sqadd           v2.4s,     \r1\().4s,  \r3\().4s // t4
        sqsub           \r1\().4s, \r1\().4s,  \r3\().4s // t5a
        sqadd           v3.4s,     \r7\().4s,  \r5\().4s // t7
        sqsub           \r3\().4s, \r7\().4s,  \r5\().4s // t6a

.irp r, v2, \r1, v3, \r3
        smin_4s         \r, \r, v5
.endr
.irp r, v2, \r1, v3, \r3
        smax_4s         \r, \r, v4
.endr

        mul_mls         v7,  \r3, \r1, v0.s[0], v0.s[0]  // -> t5
        mul_mla         v6,  \r3, \r1, v0.s[0], v0.s[0]  // -> t6
        srshr           v7.4s,  v7.4s,  #12              // t5
        srshr           v6.4s,  v6.4s,  #12              // t6

        sqsub           \r7\().4s,  \r0\().4s,  v3.4s    // out7
        sqadd           \r0\().4s,  \r0\().4s,  v3.4s    // out0
        sqadd           \r1\().4s,  \r2\().4s,  v6.4s    // out1
        sqsub           v6.4s,      \r2\().4s,  v6.4s    // out6
        sqadd           \r2\().4s,  \r4\().4s,  v7.4s    // out2
        sqsub           \r5\().4s,  \r4\().4s,  v7.4s    // out5
        sqadd           \r3\().4s,  \r6\().4s,  v2.4s    // out3
        sqsub           \r4\().4s,  \r6\().4s,  v2.4s    // out4
        mov             \r6\().16b, v6.16b               // out6
.endm

function inv_dct_4s_x8_neon
        AARCH64_VALID_CALL_TARGET
        movrel          x16, idct_coeffs
        ld1             {v0.4s, v1.4s}, [x16]
        idct_8          v16, v17, v18, v19, v20, v21, v22, v23
        ret
endfunc

.macro iadst_8 o0, o1, o2, o3, o4, o5, o6, o7
        movrel          x16, iadst8_coeffs
        ld1             {v0.4s, v1.4s}, [x16], #32

        mul_mla         v2,  v23, v16, v0.s[0], v0.s[1]
        mul_mls         v4,  v23, v16, v0.s[1], v0.s[0]
        mul_mla         v6,  v21, v18, v0.s[2], v0.s[3]
        srshr           v16.4s, v2.4s,  #12  // t0a
        srshr           v23.4s, v4.4s,  #12  // t1a
        mul_mls         v2,  v21, v18, v0.s[3], v0.s[2]
        mul_mla         v4,  v19, v20, v1.s[0], v1.s[1]
        srshr           v18.4s, v6.4s,  #12  // t2a
        srshr           v21.4s, v2.4s,  #12  // t3a
        mul_mls         v6,  v19, v20, v1.s[1], v1.s[0]
        mul_mla         v2,  v17, v22, v1.s[2], v1.s[3]
        srshr           v20.4s, v4.4s,  #12  // t4a
        srshr           v19.4s, v6.4s,  #12  // t5a
        mul_mls         v4,  v17, v22, v1.s[3], v1.s[2]
        srshr           v22.4s, v2.4s,  #12  // t6a
        srshr           v17.4s, v4.4s,  #12  // t7a

        ld1             {v0.4s}, [x16]

        movi            v1.4s,   #1, msl #16     // row_clip_max = ~(~bdmax << 7), 0x1ffff

        sqadd           v2.4s,   v16.4s,  v20.4s // t0
        sqsub           v3.4s,   v16.4s,  v20.4s // t4
        mvni            v20.4s,  #1, msl #16     // row_clip_min = (~bdmax << 7), 0xfffe0000
        sqadd           v4.4s,   v23.4s,  v19.4s // t1
        sqsub           v5.4s,   v23.4s,  v19.4s // t5
        sqadd           v6.4s,   v18.4s,  v22.4s // t2
        sqsub           v7.4s,   v18.4s,  v22.4s // t6
        sqadd           v18.4s,  v21.4s,  v17.4s // t3
        sqsub           v19.4s,  v21.4s,  v17.4s // t7

.irp r, v2, v3, v4, v5, v6, v7, v18, v19
        smin_4s         \r, \r, v1
.endr
.irp r, v2, v3, v4, v5, v6, v7, v18, v19
        smax_4s         \r, \r, v20
.endr

        mul_mla         v16, v3,  v5,  v0.s[3], v0.s[2]
        mul_mls         v20, v3,  v5,  v0.s[2], v0.s[3]
        mul_mls         v22, v19, v7,  v0.s[3], v0.s[2]

        srshr           v3.4s,  v16.4s, #12  // t4a
        srshr           v5.4s,  v20.4s, #12  // t5a

        mul_mla         v16, v19, v7,  v0.s[2], v0.s[3]

        srshr           v7.4s,  v22.4s, #12  // t6a
        srshr           v19.4s, v16.4s, #12  // t7a

        sqadd           \o0\().4s, v2.4s, v6.4s  // out0
        sqsub           v2.4s,     v2.4s, v6.4s  // t2
        sqadd           \o7\().4s, v4.4s, v18.4s // out7
        sqsub           v4.4s,     v4.4s, v18.4s // t3

        mvni            v18.4s,  #1, msl #16     // row_clip_min = (~bdmax << 7), 0xfffe0000

        sqadd           \o1\().4s, v3.4s, v7.4s  // out1
        sqsub           v3.4s,     v3.4s, v7.4s  // t6
        sqadd           \o6\().4s, v5.4s, v19.4s // out6
        sqsub           v5.4s,     v5.4s, v19.4s // t7

        // Not clipping the output registers, as they will be downshifted and
        // narrowed afterwards anyway.
.irp r, v2, v4, v3, v5
        smin_4s         \r, \r, v1
.endr
.irp r, v2, v4, v3, v5
        smax_4s         \r, \r, v18
.endr

        sqneg           \o7\().4s, \o7\().4s     // out7
        sqneg           \o1\().4s, \o1\().4s     // out1

        mul_mla         v18, v2,  v4,  v0.s[0], v0.s[0] // -> out3 (v19 or v20)
        mul_mls         v6,  v2,  v4,  v0.s[0], v0.s[0] // -> out4 (v20 or v19)
        mul_mls         v20, v3,  v5,  v0.s[0], v0.s[0] // -> out5 (v21 or v18)
        srshr           v2.4s,  v18.4s, #12 // out3
        mul_mla         v18, v3,  v5,  v0.s[0], v0.s[0] // -> out2 (v18 or v21)
        srshr           v3.4s,  v20.4s, #12 // out5
        srshr           \o2\().4s, v18.4s, #12 // out2 (v18 or v21)
        srshr           \o4\().4s, v6.4s,  #12 // out4 (v20 or v19)

        sqneg           \o3\().4s, v2.4s     // out3
        sqneg           \o5\().4s, v3.4s     // out5
.endm

function inv_adst_4s_x8_neon
        AARCH64_VALID_CALL_TARGET
        iadst_8         v16, v17, v18, v19, v20, v21, v22, v23
        ret
endfunc

function inv_flipadst_4s_x8_neon
        AARCH64_VALID_CALL_TARGET
        iadst_8         v23, v22, v21, v20, v19, v18, v17, v16
        ret
endfunc

function inv_identity_4s_x8_neon
        AARCH64_VALID_CALL_TARGET
        sqshl           v16.4s,  v16.4s,  #1
        sqshl           v17.4s,  v17.4s,  #1
        sqshl           v18.4s,  v18.4s,  #1
        sqshl           v19.4s,  v19.4s,  #1
        sqshl           v20.4s,  v20.4s,  #1
        sqshl           v21.4s,  v21.4s,  #1
        sqshl           v22.4s,  v22.4s,  #1
        sqshl           v23.4s,  v23.4s,  #1
        ret
endfunc

function inv_txfm_add_8x8_neon
        movi            v31.4s,  #0

        cmp             w3,  w13
        mov             x11, #32
        b.lt            1f

        add             x6,  x2,  #16
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s
        ld1             {\i},     [x6]
        st1             {v31.4s}, [x6], x11
.endr

        blr             x4

        sqrshrn         v24.4h,  v16.4s,  #1
        sqrshrn         v25.4h,  v17.4s,  #1
        sqrshrn         v26.4h,  v18.4s,  #1
        sqrshrn         v27.4h,  v19.4s,  #1
        sqrshrn2        v24.8h,  v20.4s,  #1
        sqrshrn2        v25.8h,  v21.4s,  #1
        sqrshrn2        v26.8h,  v22.4s,  #1
        sqrshrn2        v27.8h,  v23.4s,  #1

        transpose_4x8h  v24, v25, v26, v27, v2, v3, v4, v5

        b               2f

1:
.irp i, v24.8h, v25.8h, v26.8h, v27.8h
        movi            \i,  #0
.endr

2:

.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s
        ld1             {\i},     [x2]
        st1             {v31.4s}, [x2], x11
.endr

        blr             x4

        sqrshrn         v16.4h,  v16.4s,  #1
        sqrshrn         v17.4h,  v17.4s,  #1
        sqrshrn         v18.4h,  v18.4s,  #1
        sqrshrn         v19.4h,  v19.4s,  #1
        sqrshrn2        v16.8h,  v20.4s,  #1
        sqrshrn2        v17.8h,  v21.4s,  #1
        sqrshrn2        v18.8h,  v22.4s,  #1
        sqrshrn2        v19.8h,  v23.4s,  #1

        transpose_4x8h  v16, v17, v18, v19, v20, v21, v22, v23

        mov             v20.16b, v24.16b
        mov             v21.16b, v25.16b
        mov             v22.16b, v26.16b
        mov             v23.16b, v27.16b

        blr             x5

        load_add_store_8x8 x0, x7
        ret             x15
endfunc

.macro def_fn_8x8 txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_8x8_16bpc_neon, export=1
        mov             x15, x30

.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         8,   8,   1
.endif
        movrel          x5,  X(inv_\txfm2\()_8h_x8_neon)
        mov             w13, #\eob_half
        adr             x4,  inv_\txfm1\()_4s_x8_neon
        b               inv_txfm_add_8x8_neon
endfunc
.endm

def_fn_8x8 dct, dct, 10
def_fn_8x8 identity, identity, 10
def_fn_8x8 dct, adst, 10
def_fn_8x8 dct, flipadst, 10
def_fn_8x8 dct, identity, 4
def_fn_8x8 adst, dct, 10
def_fn_8x8 adst, adst, 10
def_fn_8x8 adst, flipadst, 10
def_fn_8x8 flipadst, dct, 10
def_fn_8x8 flipadst, adst, 10
def_fn_8x8 flipadst, flipadst, 10
def_fn_8x8 identity, dct, 4
def_fn_8x8 adst, identity, 4
def_fn_8x8 flipadst, identity, 4
def_fn_8x8 identity, adst, 4
def_fn_8x8 identity, flipadst, 4

function inv_txfm_add_8x4_neon
        movi            v28.4s,  #0
        movi            v29.4s,  #0
        movi            v30.4s,  #0
        movi            v31.4s,  #0
        ld1             {v16.4s,v17.4s,v18.4s,v19.4s}, [x2]
        st1             {v28.4s,v29.4s,v30.4s,v31.4s}, [x2], #64
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16
        ld1             {v20.4s,v21.4s,v22.4s,v23.4s}, [x2]
        st1             {v28.4s,v29.4s,v30.4s,v31.4s}, [x2]

        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23

        blr             x4

        sqxtn           v16.4h,  v16.4s
        sqxtn           v17.4h,  v17.4s
        sqxtn           v18.4h,  v18.4s
        sqxtn           v19.4h,  v19.4s
        sqxtn           v20.4h,  v20.4s
        sqxtn           v21.4h,  v21.4s
        sqxtn           v22.4h,  v22.4s
        sqxtn           v23.4h,  v23.4s

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
        movz            w16, #2896*8, lsl #16
        movi            v31.4s,  #0
        dup             v30.2s,  w16

        cmp             w3,  w13
        mov             x11, #32
        b.lt            1f

        add             x6,  x2,  #16
.irp i, v16.4s, v17.4s, v18.4s, v19.4s
        ld1             {\i},     [x6]
        st1             {v31.4s}, [x6], x11
.endr
        scale_input     .4s, v30.s[0], v16, v17, v18, v19
        blr             x4
        sqxtn           v20.4h,  v16.4s
        sqxtn           v21.4h,  v17.4s
        sqxtn           v22.4h,  v18.4s
        sqxtn           v23.4h,  v19.4s
        transpose_4x4h  v20, v21, v22, v23, v4,  v5,  v6,  v7

        b               2f

1:
.irp i, v20, v21, v22, v23
        movi            \i\().4h, #0
.endr

2:

.irp i, v16.4s, v17.4s, v18.4s, v19.4s
        ld1             {\i},     [x2]
        st1             {v31.4s}, [x2], x11
.endr
        scale_input     .4s, v30.s[0], v16, v17, v18, v19
        blr             x4
        sqxtn           v16.4h,  v16.4s
        sqxtn           v17.4h,  v17.4s
        sqxtn           v18.4h,  v18.4s
        sqxtn           v19.4h,  v19.4s
        transpose_4x4h  v16, v17, v18, v19, v4,  v5,  v6,  v7

        blr             x5

        load_add_store_4x8 x0, x7
        ret             x15
endfunc

.macro def_fn_48 w, h, txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_16bpc_neon, export=1
        mov             x15, x30

.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  0
.endif
        adr             x4,  inv_\txfm1\()_4s_x\w\()_neon
.if \w == 4
        mov             w13, #\eob_half
.endif
        movrel          x5,  X(inv_\txfm2\()_\w\()h_x\h\()_neon)
        b               inv_txfm_add_\w\()x\h\()_neon
endfunc
.endm

.macro def_fns_48 w, h
def_fn_48 \w, \h, dct, dct, 13
def_fn_48 \w, \h, identity, identity, 13
def_fn_48 \w, \h, dct, adst, 13
def_fn_48 \w, \h, dct, flipadst, 13
def_fn_48 \w, \h, dct, identity, 4
def_fn_48 \w, \h, adst, dct, 13
def_fn_48 \w, \h, adst, adst, 13
def_fn_48 \w, \h, adst, flipadst, 13
def_fn_48 \w, \h, flipadst, dct, 13
def_fn_48 \w, \h, flipadst, adst, 13
def_fn_48 \w, \h, flipadst, flipadst, 13
def_fn_48 \w, \h, identity, dct, 16
def_fn_48 \w, \h, adst, identity, 4
def_fn_48 \w, \h, flipadst, identity, 4
def_fn_48 \w, \h, identity, adst, 16
def_fn_48 \w, \h, identity, flipadst, 16
.endm

def_fns_48 4, 8
def_fns_48 8, 4


function inv_dct_4s_x16_neon
        AARCH64_VALID_CALL_TARGET
        movrel          x16, idct_coeffs
        ld1             {v0.4s, v1.4s}, [x16], #32

        idct_8          v16, v18, v20, v22, v24, v26, v28, v30

        // idct_8 leaves the row_clip_max/min constants in v5 and v4
.irp r, v16, v18, v20, v22, v24, v26, v28, v30
        smin            \r\().4s,  \r\().4s,  v5.4s
.endr
.irp r, v16, v18, v20, v22, v24, v26, v28, v30
        smax            \r\().4s,  \r\().4s,  v4.4s
.endr

        ld1             {v0.4s, v1.4s}, [x16]
        sub             x16, x16, #32

        mul_mls         v2,  v17, v31, v0.s[0], v0.s[1] // -> t8a
        mul_mla         v3,  v17, v31, v0.s[1], v0.s[0] // -> t15a
        mul_mls         v6,  v25, v23, v0.s[2], v0.s[3] // -> t9a
        srshr           v17.4s, v2.4s,  #12             // t8a
        srshr           v31.4s, v3.4s,  #12             // t15a
        mul_mla         v2,  v25, v23, v0.s[3], v0.s[2] // -> t14a
        mul_mls         v3,  v21, v27, v1.s[0], v1.s[1] // -> t10a
        srshr           v23.4s, v6.4s,  #12             // t9a
        srshr           v25.4s, v2.4s,  #12             // t14a
        mul_mla         v6,  v21, v27, v1.s[1], v1.s[0] // -> t13a
        mul_mls         v2,  v29, v19, v1.s[2], v1.s[3] // -> t11a
        srshr           v21.4s, v3.4s,  #12             // t10a
        srshr           v27.4s, v6.4s,  #12             // t13a
        mul_mla         v3,  v29, v19, v1.s[3], v1.s[2] // -> t12a
        srshr           v19.4s, v2.4s,  #12             // t11a
        srshr           v29.4s, v3.4s,  #12             // t12a

        ld1             {v0.4s}, [x16]

        sqsub           v2.4s,   v17.4s,  v23.4s  // t9
        sqadd           v17.4s,  v17.4s,  v23.4s  // t8
        sqsub           v3.4s,   v31.4s,  v25.4s  // t14
        sqadd           v31.4s,  v31.4s,  v25.4s  // t15
        sqsub           v23.4s,  v19.4s,  v21.4s  // t10
        sqadd           v19.4s,  v19.4s,  v21.4s  // t11
        sqadd           v25.4s,  v29.4s,  v27.4s  // t12
        sqsub           v29.4s,  v29.4s,  v27.4s  // t13

.irp r, v2, v17, v3, v31, v23, v19, v25, v29
        smin            \r\().4s,  \r\().4s,  v5.4s
.endr
.irp r, v2, v17, v3, v31, v23, v19, v25, v29
        smax            \r\().4s,  \r\().4s,  v4.4s
.endr

        mul_mls         v7,  v3,  v2,  v0.s[2], v0.s[3] // -> t9a
        mul_mla         v6,  v3,  v2,  v0.s[3], v0.s[2] // -> t14a
        srshr           v21.4s, v7.4s,  #12             // t9a
        srshr           v27.4s, v6.4s,  #12             // t14a

        mul_mls         v7,  v29, v23, v0.s[2], v0.s[3] // -> t13a
        mul_mla         v6,  v29, v23, v0.s[3], v0.s[2] // -> t10a
        srshr           v29.4s, v7.4s,  #12             // t13a
        neg             v6.4s,   v6.4s
        srshr           v23.4s, v6.4s,  #12             // t10a

        sqsub           v2.4s,   v17.4s,  v19.4s  // t11a
        sqadd           v17.4s,  v17.4s,  v19.4s  // t8a
        sqsub           v3.4s,   v31.4s,  v25.4s  // t12a
        sqadd           v31.4s,  v31.4s,  v25.4s  // t15a
        sqadd           v19.4s,  v21.4s,  v23.4s  // t9
        sqsub           v21.4s,  v21.4s,  v23.4s  // t10
        sqsub           v25.4s,  v27.4s,  v29.4s  // t13
        sqadd           v27.4s,  v27.4s,  v29.4s  // t14

.irp r, v2, v17, v3, v31, v19, v21, v25, v27
        smin            \r\().4s,  \r\().4s,  v5.4s
.endr
.irp r, v2, v17, v3, v31, v19, v21, v25, v27
        smax            \r\().4s,  \r\().4s,  v4.4s
.endr

        mul_mls         v7,  v3,  v2,  v0.s[0], v0.s[0] // -> t11
        mul_mla         v6,  v3,  v2,  v0.s[0], v0.s[0] // -> t12
        mul_mls         v2,  v25, v21, v0.s[0], v0.s[0] // -> t10a

        srshr           v7.4s,  v7.4s,  #12   // t11
        srshr           v6.4s,  v6.4s,  #12   // t12
        mul_mla         v3,  v25, v21, v0.s[0], v0.s[0] // -> t13a
        srshr           v2.4s,  v2.4s,  #12   // t10a
        srshr           v3.4s,  v3.4s,  #12   // t13a

        sqadd           v1.4s,   v16.4s,  v31.4s  // out0
        sqsub           v31.4s,  v16.4s,  v31.4s  // out15
        mov             v16.16b, v1.16b
        sqadd           v23.4s,  v30.4s,  v17.4s  // out7
        sqsub           v1.4s,   v30.4s,  v17.4s  // out8
        sqadd           v17.4s,  v18.4s,  v27.4s  // out1
        sqsub           v30.4s,  v18.4s,  v27.4s  // out14
        sqadd           v18.4s,  v20.4s,  v3.4s   // out2
        sqsub           v29.4s,  v20.4s,  v3.4s   // out13
        sqadd           v3.4s,   v28.4s,  v19.4s  // out6
        sqsub           v25.4s,  v28.4s,  v19.4s  // out9
        sqadd           v19.4s,  v22.4s,  v6.4s   // out3
        sqsub           v28.4s,  v22.4s,  v6.4s   // out12
        sqadd           v20.4s,  v24.4s,  v7.4s   // out4
        sqsub           v27.4s,  v24.4s,  v7.4s   // out11
        sqadd           v21.4s,  v26.4s,  v2.4s   // out5
        sqsub           v26.4s,  v26.4s,  v2.4s   // out10
        mov             v24.16b, v1.16b
        mov             v22.16b, v3.16b

        ret
endfunc

.macro iadst_16 o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15
        movrel          x16, iadst16_coeffs
        ld1             {v0.4s, v1.4s}, [x16], #32

        mul_mla         v2,  v31, v16, v0.s[0], v0.s[1] // -> t0
        mul_mls         v4,  v31, v16, v0.s[1], v0.s[0] // -> t1
        mul_mla         v6,  v29, v18, v0.s[2], v0.s[3] // -> t2
        srshr           v16.4s, v2.4s,  #12             // t0
        srshr           v31.4s, v4.4s,  #12             // t1
        mul_mls         v2,  v29, v18, v0.s[3], v0.s[2] // -> t3
        mul_mla         v4,  v27, v20, v1.s[0], v1.s[1] // -> t4
        srshr           v18.4s, v6.4s,  #12             // t2
        srshr           v29.4s, v2.4s,  #12             // t3
        mul_mls         v6,  v27, v20, v1.s[1], v1.s[0] // -> t5
        mul_mla         v2,  v25, v22, v1.s[2], v1.s[3] // -> t6
        srshr           v20.4s, v4.4s,  #12             // t4
        srshr           v27.4s, v6.4s,  #12             // t5
        mul_mls         v4,  v25, v22, v1.s[3], v1.s[2] // -> t7
        ld1             {v0.4s, v1.4s}, [x16]
        movrel          x16, idct_coeffs
        mul_mla         v6,  v23, v24, v0.s[0], v0.s[1] // -> t8
        srshr           v22.4s, v2.4s,  #12             // t6
        srshr           v25.4s, v4.4s,  #12             // t7
        mul_mls         v2,  v23, v24, v0.s[1], v0.s[0] // -> t9
        mul_mla         v4,  v21, v26, v0.s[2], v0.s[3] // -> t10
        srshr           v23.4s, v6.4s,  #12             // t8
        srshr           v24.4s, v2.4s,  #12             // t9
        mul_mls         v6,  v21, v26, v0.s[3], v0.s[2] // -> t11
        mul_mla         v2,  v19, v28, v1.s[0], v1.s[1] // -> t12
        srshr           v21.4s, v4.4s,  #12             // t10
        srshr           v26.4s, v6.4s,  #12             // t11
        mul_mls         v4,  v19, v28, v1.s[1], v1.s[0] // -> t13
        mul_mla         v6,  v17, v30, v1.s[2], v1.s[3] // -> t14
        srshr           v19.4s, v2.4s,  #12             // t12
        srshr           v28.4s, v4.4s,  #12             // t13
        mul_mls         v2,  v17, v30, v1.s[3], v1.s[2] // -> t15
        srshr           v17.4s, v6.4s,  #12             // t14
        srshr           v30.4s, v2.4s,  #12             // t15

        ld1             {v0.4s, v1.4s}, [x16]

        movi            v5.4s,   #1, msl #16     // row_clip_max = ~(~bdmax << 7), 0x1ffff
        mvni            v7.4s,   #1, msl #16     // row_clip_min = (~bdmax << 7), 0xfffe0000

        sqsub           v2.4s,   v16.4s,  v23.4s // t8a
        sqadd           v16.4s,  v16.4s,  v23.4s // t0a
        sqsub           v3.4s,   v31.4s,  v24.4s // t9a
        sqadd           v31.4s,  v31.4s,  v24.4s // t1a
        sqadd           v23.4s,  v18.4s,  v21.4s // t2a
        sqsub           v18.4s,  v18.4s,  v21.4s // t10a
        sqadd           v24.4s,  v29.4s,  v26.4s // t3a
        sqsub           v29.4s,  v29.4s,  v26.4s // t11a
        sqadd           v21.4s,  v20.4s,  v19.4s // t4a
        sqsub           v20.4s,  v20.4s,  v19.4s // t12a
        sqadd           v26.4s,  v27.4s,  v28.4s // t5a
        sqsub           v27.4s,  v27.4s,  v28.4s // t13a
        sqadd           v19.4s,  v22.4s,  v17.4s // t6a
        sqsub           v22.4s,  v22.4s,  v17.4s // t14a
        sqadd           v28.4s,  v25.4s,  v30.4s // t7a
        sqsub           v25.4s,  v25.4s,  v30.4s // t15a

.irp r, v2, v16, v3, v31, v23, v18, v24, v29, v21, v20, v26, v27, v19, v22, v28, v25
        smin_4s         \r, \r, v5
.endr
.irp r, v2, v16, v3, v31, v23, v18, v24, v29, v21, v20, v26, v27, v19, v22, v28, v25
        smax_4s         \r, \r, v7
.endr

        mul_mla         v4,  v2,  v3,  v1.s[1], v1.s[0] // -> t8
        mul_mls         v6,  v2,  v3,  v1.s[0], v1.s[1] // -> t9
        mul_mla         v2,  v18, v29, v1.s[3], v1.s[2] // -> t10
        srshr           v17.4s, v4.4s,  #12             // t8
        srshr           v30.4s, v6.4s,  #12             // t9
        mul_mls         v4,  v18, v29, v1.s[2], v1.s[3] // -> t11
        mul_mls         v6,  v27, v20, v1.s[1], v1.s[0] // -> t12
        srshr           v18.4s, v2.4s,  #12             // t10
        srshr           v29.4s, v4.4s,  #12             // t11
        mul_mla         v2,  v27, v20, v1.s[0], v1.s[1] // -> t13
        mul_mls         v4,  v25, v22, v1.s[3], v1.s[2] // -> t14
        srshr           v27.4s, v6.4s,  #12             // t12
        srshr           v20.4s, v2.4s,  #12             // t13
        mul_mla         v6,  v25, v22, v1.s[2], v1.s[3] // -> t15
        srshr           v25.4s, v4.4s,  #12             // t14
        srshr           v22.4s, v6.4s,  #12             // t15

        sqsub           v2.4s,   v16.4s,  v21.4s // t4
        sqadd           v16.4s,  v16.4s,  v21.4s // t0
        sqsub           v3.4s,   v31.4s,  v26.4s // t5
        sqadd           v31.4s,  v31.4s,  v26.4s // t1
        sqadd           v21.4s,  v23.4s,  v19.4s // t2
        sqsub           v23.4s,  v23.4s,  v19.4s // t6
        sqadd           v26.4s,  v24.4s,  v28.4s // t3
        sqsub           v24.4s,  v24.4s,  v28.4s // t7
        sqadd           v19.4s,  v17.4s,  v27.4s // t8a
        sqsub           v17.4s,  v17.4s,  v27.4s // t12a
        sqadd           v28.4s,  v30.4s,  v20.4s // t9a
        sqsub           v30.4s,  v30.4s,  v20.4s // t13a
        sqadd           v27.4s,  v18.4s,  v25.4s // t10a
        sqsub           v18.4s,  v18.4s,  v25.4s // t14a
        sqadd           v20.4s,  v29.4s,  v22.4s // t11a
        sqsub           v29.4s,  v29.4s,  v22.4s // t15a

.irp r, v2, v16, v3, v31, v21, v23, v26, v24, v19, v17, v28, v30, v27, v18, v20, v29
        smin_4s         \r, \r, v5
.endr
.irp r, v2, v16, v3, v31, v21, v23, v26, v24, v19, v17, v28, v30, v27, v18, v20, v29
        smax_4s         \r, \r, v7
.endr

        mul_mla         v4,  v2,  v3,  v0.s[3], v0.s[2] // -> t4a
        mul_mls         v6,  v2,  v3,  v0.s[2], v0.s[3] // -> t5a
        mul_mls         v2,  v24, v23, v0.s[3], v0.s[2] // -> t6a
        srshr           v22.4s, v4.4s,  #12             // t4a
        srshr           v25.4s, v6.4s,  #12             // t5a
        mul_mla         v4,  v24, v23, v0.s[2], v0.s[3] // -> t7a
        mul_mla         v6,  v17, v30, v0.s[3], v0.s[2] // -> t12
        srshr           v24.4s, v2.4s,  #12             // t6a
        srshr           v23.4s, v4.4s,  #12             // t7a
        mul_mls         v2,  v17, v30, v0.s[2], v0.s[3] // -> t13
        mul_mls         v4,  v29, v18, v0.s[3], v0.s[2] // -> t14
        srshr           v17.4s, v6.4s,  #12             // t12
        mul_mla         v6,  v29, v18, v0.s[2], v0.s[3] // -> t15
        srshr           v29.4s, v2.4s,  #12             // t13
        srshr           v30.4s, v4.4s,  #12             // t14
        srshr           v18.4s, v6.4s,  #12             // t15

        sqsub           v2.4s,   v16.4s,  v21.4s // t2a
.ifc \o0, v16
        sqadd           \o0\().4s,  v16.4s,  v21.4s // out0
        sqsub           v21.4s,     v31.4s,  v26.4s // t3a
        sqadd           \o15\().4s, v31.4s,  v26.4s // out15
.else
        sqadd           v4.4s,      v16.4s,  v21.4s // out0
        sqsub           v21.4s,     v31.4s,  v26.4s // t3a
        sqadd           \o15\().4s, v31.4s,  v26.4s // out15
        mov             \o0\().16b, v4.16b
.endif

        sqsub           v3.4s,      v29.4s,  v18.4s // t15a
        sqadd           \o13\().4s, v29.4s,  v18.4s // out13
        sqadd           \o2\().4s,  v17.4s,  v30.4s // out2
        sqsub           v26.4s,     v17.4s,  v30.4s // t14a

        sqadd           \o1\().4s,  v19.4s,  v27.4s // out1
        sqsub           v27.4s,     v19.4s,  v27.4s // t10
        sqadd           \o14\().4s, v28.4s,  v20.4s // out14
        sqsub           v20.4s,     v28.4s,  v20.4s // t11

        sqadd           \o3\().4s,  v22.4s,  v24.4s // out3
        sqsub           v22.4s,     v22.4s,  v24.4s // t6
        sqadd           \o12\().4s, v25.4s,  v23.4s // out12
        sqsub           v23.4s,     v25.4s,  v23.4s // t7

        // Not clipping the output registers, as they will be downshifted and
        // narrowed afterwards anyway.
.irp r, v2, v21, v3, v26, v27, v20, v22, v23
        smin_4s         \r, \r, v5
.endr
.irp r, v2, v21, v3, v26, v27, v20, v22, v23
        smax_4s         \r, \r, v7
.endr

        sqneg           \o15\().4s, \o15\().4s      // out15
        sqneg           \o13\().4s, \o13\().4s      // out13
        sqneg           \o1\().4s,  \o1\().4s       // out1
        sqneg           \o3\().4s,  \o3\().4s       // out3

        mul_mls         v24, v2,  v21, v0.s[0], v0.s[0] // -> out8 (v24 or v23)
        mul_mla         v4,  v2,  v21, v0.s[0], v0.s[0] // -> out7 (v23 or v24)
        mul_mla         v6,  v26, v3,  v0.s[0], v0.s[0] // -> out5 (v21 or v26)

        srshr           v24.4s, v24.4s, #12             // out8
        srshr           v4.4s,  v4.4s,  #12             // out7
        srshr           v5.4s,  v6.4s,  #12             // out5
        mul_mls         v6,  v26, v3,  v0.s[0], v0.s[0] // -> out10 (v26 or v21)
        mul_mla         v2,  v22, v23, v0.s[0], v0.s[0] // -> out4 (v20 or v27)
        srshr           v26.4s, v6.4s,  #12             // out10

        mul_mls         v6,  v22, v23, v0.s[0], v0.s[0] // -> out11 (v27 or v20)
        mul_mla         v22, v27, v20, v0.s[0], v0.s[0] // -> out6 (v22 or v25)
        mul_mls         v21, v27, v20, v0.s[0], v0.s[0] // -> out9 (v25 or v22)

        srshr           \o4\().4s,   v2.4s,  #12        // out4
        srshr           v6.4s,       v6.4s,  #12        // out11
        srshr           v7.4s,       v21.4s, #12        // out9
        srshr           \o6\().4s,   v22.4s, #12        // out6

.ifc \o8, v23
        mov             \o8\().16b,  v24.16b
        mov             \o10\().16b, v26.16b
.endif

        sqneg           \o7\().4s,   v4.4s // out7
        sqneg           \o5\().4s,   v5.4s // out5
        sqneg           \o11\().4s,  v6.4s // out11
        sqneg           \o9\().4s,   v7.4s // out9
.endm

function inv_adst_4s_x16_neon
        AARCH64_VALID_CALL_TARGET
        iadst_16        v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31
        ret
endfunc

function inv_flipadst_4s_x16_neon
        AARCH64_VALID_CALL_TARGET
        iadst_16        v31, v30, v29, v28, v27, v26, v25, v24, v23, v22, v21, v20, v19, v18, v17, v16
        ret
endfunc

function inv_identity_4s_x16_neon
        AARCH64_VALID_CALL_TARGET
        movz            w16, #2*(5793-4096)*8, lsl #16
        dup             v0.2s,   w16
.irp i, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        sqrdmulh        v2.4s,      v\i\().4s,  v0.s[0]
        sqadd           v\i\().4s,  v\i\().4s,  v\i\().4s
        sqadd           v\i\().4s,  v\i\().4s,  v2.4s
.endr
        ret
endfunc

.macro identity_4x16_shift1 c
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        sqrdmulh        v3.4s,   \i,      \c
        srshr           v3.4s,   v3.4s,   #1
        sqadd           \i,      \i,      v3.4s
.endr
.endm

.macro identity_4x16 c
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        sqrdmulh        v3.4s,   \i,      \c
        sqadd           \i,      \i,      \i
        sqadd           \i,      \i,      v3.4s
.endr
.endm

.macro def_horz_16 scale=0, shift=2, suffix
function inv_txfm_horz\suffix\()_16x4_neon
        mov             x14, x30
        movi            v7.4s,  #0
.if \scale
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16
.endif
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        ld1             {\i}, [x7]
        st1             {v7.4s}, [x7], x8
.endr
.if \scale
        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .4s, v0.s[0], v24, v25, v26, v27, v28, v29, v30, v31
.endif
        blr             x4
        sqrshrn         v16.4h,  v16.4s,  #\shift
        sqrshrn         v17.4h,  v17.4s,  #\shift
        sqrshrn         v18.4h,  v18.4s,  #\shift
        sqrshrn         v19.4h,  v19.4s,  #\shift
        sqrshrn2        v16.8h,  v20.4s,  #\shift
        sqrshrn2        v17.8h,  v21.4s,  #\shift
        sqrshrn2        v18.8h,  v22.4s,  #\shift
        sqrshrn2        v19.8h,  v23.4s,  #\shift
        sqrshrn         v20.4h,  v24.4s,  #\shift
        sqrshrn         v21.4h,  v25.4s,  #\shift
        sqrshrn         v22.4h,  v26.4s,  #\shift
        sqrshrn         v23.4h,  v27.4s,  #\shift
        sqrshrn2        v20.8h,  v28.4s,  #\shift
        sqrshrn2        v21.8h,  v29.4s,  #\shift
        sqrshrn2        v22.8h,  v30.4s,  #\shift
        sqrshrn2        v23.8h,  v31.4s,  #\shift
        transpose_4x8h  v16, v17, v18, v19, v4,  v5,  v6,  v7
        transpose_4x8h  v20, v21, v22, v23, v4,  v5,  v6,  v7

.irp i, v16.8h, v20.8h, v17.8h, v21.8h, v18.8h, v22.8h, v19.8h, v23.8h
        st1             {\i}, [x6], #16
.endr

        ret             x14
endfunc
.endm

def_horz_16 scale=0, shift=2
def_horz_16 scale=1, shift=1, suffix=_scale

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
        ldrh            w12, [x13], #2
.irp i, 0, 4, 8, 12
        add             x6,  sp,  #(\i*16*2)
.if \i > 0
        mov             w8,  #(16 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 12
        ldrh            w12, [x13], #2
.endif
.endif
        add             x7,  x2,  #(\i*4)
        mov             x8,  #16*4
        bl              inv_txfm_horz_16x4_neon
.endr
        b               3f
1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #4
.rept 2
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b
3:
.irp i, 0, 8
        add             x6,  x0,  #(\i*2)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #32
        bl              inv_txfm_add_vert_8x16_neon
.endr

        add             sp,  sp,  #512
        ret             x15
endfunc

const eob_16x16
        .short 10, 36, 78, 256
endconst

const eob_16x16_identity
        .short 4, 8, 12, 256
endconst

.macro def_fn_16x16 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_16x16_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         16,  16,  2
.endif
        adr             x4,  inv_\txfm1\()_4s_x16_neon
        movrel          x5,  X(inv_\txfm2\()_8h_x16_neon)
.ifc \txfm1, identity
.ifc \txfm2, identity
        movrel          x13, eob_16x16
.else
        movrel          x13, eob_16x16_identity
.endif
.else
.ifc \txfm2, identity
        movrel          x13, eob_16x16_identity
.else
        movrel          x13, eob_16x16
.endif
.endif
        b               inv_txfm_add_16x16_neon
endfunc
.endm

def_fn_16x16 dct, dct
def_fn_16x16 identity, identity
def_fn_16x16 dct, adst
def_fn_16x16 dct, flipadst
def_fn_16x16 dct, identity
def_fn_16x16 adst, dct
def_fn_16x16 adst, adst
def_fn_16x16 adst, flipadst
def_fn_16x16 flipadst, dct
def_fn_16x16 flipadst, adst
def_fn_16x16 flipadst, flipadst
def_fn_16x16 identity, dct

function inv_txfm_add_16x4_neon
        mov             x15, x30
        movi            v4.4s,  #0

.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        ld1             {\i},    [x2]
        st1             {v4.4s}, [x2], #16
.endr

        blr             x4

        sqrshrn         v16.4h,  v16.4s,  #1
        sqrshrn         v17.4h,  v17.4s,  #1
        sqrshrn         v18.4h,  v18.4s,  #1
        sqrshrn         v19.4h,  v19.4s,  #1
        sqrshrn2        v16.8h,  v20.4s,  #1
        sqrshrn2        v17.8h,  v21.4s,  #1
        sqrshrn2        v18.8h,  v22.4s,  #1
        sqrshrn2        v19.8h,  v23.4s,  #1
        transpose_4x8h  v16, v17, v18, v19, v2,  v3,  v4,  v5
        blr             x5
        mov             x6,  x0
        load_add_store_8x4 x6, x7

        sqrshrn         v16.4h,  v24.4s,  #1
        sqrshrn         v17.4h,  v25.4s,  #1
        sqrshrn         v18.4h,  v26.4s,  #1
        sqrshrn         v19.4h,  v27.4s,  #1
        sqrshrn2        v16.8h,  v28.4s,  #1
        sqrshrn2        v17.8h,  v29.4s,  #1
        sqrshrn2        v18.8h,  v30.4s,  #1
        sqrshrn2        v19.8h,  v31.4s,  #1
        transpose_4x8h  v16, v17, v18, v19, v2,  v3,  v4,  v5
        blr             x5
        add             x6,  x0,  #16
        load_add_store_8x4 x6, x7

        ret             x15
endfunc

function inv_txfm_add_4x16_neon
        ldrh            w12, [x13, #4]
        mov             x15, x30

        mov             x11, #64

        cmp             w3,  w12
        ldrh            w12, [x13, #2]
        b.lt            1f

        add             x6,  x2,  #48
        movi            v2.4s,   #0
.irp i, v16.4s, v17.4s, v18.4s, v19.4s
        ld1             {\i},    [x6]
        st1             {v2.4s}, [x6], x11
.endr
        blr             x4
        sqrshrn         v28.4h,  v16.4s,  #1
        sqrshrn         v29.4h,  v17.4s,  #1
        sqrshrn         v30.4h,  v18.4s,  #1
        sqrshrn         v31.4h,  v19.4s,  #1
        transpose_4x4h  v28, v29, v30, v31, v4,  v5,  v6,  v7

        b               2f
1:
.irp i, v28.4h, v29.4h, v30.4h, v31.4h
        movi            \i,  #0
.endr
2:
        cmp             w3,  w12
        ldrh            w12, [x13, #0]
        b.lt            1f

        add             x6,  x2,  #32
        movi            v2.4s,   #0
.irp i, v16.4s, v17.4s, v18.4s, v19.4s
        ld1             {\i},    [x6]
        st1             {v2.4s}, [x6], x11
.endr
        blr             x4
        sqrshrn         v24.4h,  v16.4s,  #1
        sqrshrn         v25.4h,  v17.4s,  #1
        sqrshrn         v26.4h,  v18.4s,  #1
        sqrshrn         v27.4h,  v19.4s,  #1
        transpose_4x4h  v24, v25, v26, v27, v4,  v5,  v6,  v7

        b               2f
1:
.irp i, v24.4h, v25.4h, v26.4h, v27.4h
        movi            \i,  #0
.endr
2:
        cmp             w3,  w12
        b.lt            1f

        add             x6,  x2,  #16
        movi            v2.4s,   #0
.irp i, v16.4s, v17.4s, v18.4s, v19.4s
        ld1             {\i},    [x6]
        st1             {v2.4s}, [x6], x11
.endr
        blr             x4
        sqrshrn         v20.4h,  v16.4s,  #1
        sqrshrn         v21.4h,  v17.4s,  #1
        sqrshrn         v22.4h,  v18.4s,  #1
        sqrshrn         v23.4h,  v19.4s,  #1
        transpose_4x4h  v20, v21, v22, v23, v4,  v5,  v6,  v7

        b               2f
1:
.irp i, v20.4h, v21.4h, v22.4h, v23.4h
        movi            \i,  #0
.endr
2:

        movi            v2.4s,   #0
.irp i, v16.4s, v17.4s, v18.4s, v19.4s
        ld1             {\i},    [x2]
        st1             {v2.4s}, [x2], x11
.endr
        blr             x4
        sqrshrn         v16.4h,  v16.4s,  #1
        sqrshrn         v17.4h,  v17.4s,  #1
        sqrshrn         v18.4h,  v18.4s,  #1
        sqrshrn         v19.4h,  v19.4s,  #1
        transpose_4x8h  v16, v17, v18, v19, v4,  v5,  v6,  v7

        blr             x5

        load_add_store_4x16 x0, x6

        ret             x15
endfunc

const eob_4x16
        .short 13, 29, 45, 64
endconst

const eob_4x16_identity1
        .short 16, 32, 48, 64
endconst

const eob_4x16_identity2
        .short 4, 8, 12, 64
endconst

.macro def_fn_416 w, h, txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
.if \w == 4
        adr             x4,  inv_\txfm1\()_4s_x\w\()_neon
        movrel          x5,  X(inv_\txfm2\()_4h_x\h\()_neon)
.ifc \txfm1, identity
.ifc \txfm2, identity
        movrel          x13, eob_4x16
.else
        movrel          x13, eob_4x16_identity1
.endif
.else
.ifc \txfm2, identity
        movrel          x13, eob_4x16_identity2
.else
        movrel          x13, eob_4x16
.endif
.endif
.else
        adr             x4,  inv_\txfm1\()_4s_x\w\()_neon
        movrel          x5,  X(inv_\txfm2\()_8h_x\h\()_neon)
.endif
        b               inv_txfm_add_\w\()x\h\()_neon
endfunc
.endm

.macro def_fns_416 w, h
def_fn_416 \w, \h, dct, dct
def_fn_416 \w, \h, identity, identity
def_fn_416 \w, \h, dct, adst
def_fn_416 \w, \h, dct, flipadst
def_fn_416 \w, \h, dct, identity
def_fn_416 \w, \h, adst, dct
def_fn_416 \w, \h, adst, adst
def_fn_416 \w, \h, adst, flipadst
def_fn_416 \w, \h, flipadst, dct
def_fn_416 \w, \h, flipadst, adst
def_fn_416 \w, \h, flipadst, flipadst
def_fn_416 \w, \h, identity, dct
def_fn_416 \w, \h, adst, identity
def_fn_416 \w, \h, flipadst, identity
def_fn_416 \w, \h, identity, adst
def_fn_416 \w, \h, identity, flipadst
.endm

def_fns_416 4, 16
def_fns_416 16, 4


function inv_txfm_add_16x8_neon
        mov             x15, x30
        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

        cmp             w3,  w13
        mov             x11, #32
        b.lt            1f

        movi            v4.4s,  #0
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16

        add             x6,  x2,  #16
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        ld1             {\i},    [x6]
        st1             {v4.4s}, [x6], x11
.endr

        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .4s, v0.s[0], v24, v25, v26, v27, v28, v29, v30, v31
        blr             x4

        sqrshrn         v8.4h,   v16.4s,  #1
        sqrshrn         v9.4h,   v17.4s,  #1
        sqrshrn         v10.4h,  v18.4s,  #1
        sqrshrn         v11.4h,  v19.4s,  #1
        sqrshrn2        v8.8h,   v20.4s,  #1
        sqrshrn2        v9.8h,   v21.4s,  #1
        sqrshrn2        v10.8h,  v22.4s,  #1
        sqrshrn2        v11.8h,  v23.4s,  #1
        sqrshrn         v12.4h,  v24.4s,  #1
        sqrshrn         v13.4h,  v25.4s,  #1
        sqrshrn         v14.4h,  v26.4s,  #1
        sqrshrn         v15.4h,  v27.4s,  #1
        sqrshrn2        v12.8h,  v28.4s,  #1
        sqrshrn2        v13.8h,  v29.4s,  #1
        sqrshrn2        v14.8h,  v30.4s,  #1
        sqrshrn2        v15.8h,  v31.4s,  #1

        transpose_4x8h  v8,  v9,  v10, v11, v2,  v3,  v4,  v5
        transpose_4x8h  v12, v13, v14, v15, v2,  v3,  v4,  v5

        b               2f
1:
.irp i, v8.8h, v9.8h, v10.8h, v11.8h, v12.8h, v13.8h, v14.8h, v15.8h
        movi            \i,  #0
.endr
2:
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16

        movi            v4.4s,  #0
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        ld1             {\i},    [x2]
        st1             {v4.4s}, [x2], x11
.endr

        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .4s, v0.s[0], v24, v25, v26, v27, v28, v29, v30, v31
        blr             x4

        sqrshrn         v16.4h,  v16.4s,  #1
        sqrshrn         v17.4h,  v17.4s,  #1
        sqrshrn         v18.4h,  v18.4s,  #1
        sqrshrn         v19.4h,  v19.4s,  #1
        sqrshrn2        v16.8h,  v20.4s,  #1
        sqrshrn2        v17.8h,  v21.4s,  #1
        sqrshrn2        v18.8h,  v22.4s,  #1
        sqrshrn2        v19.8h,  v23.4s,  #1

        mov             v20.16b, v8.16b
        mov             v21.16b, v9.16b
        mov             v22.16b, v10.16b
        mov             v23.16b, v11.16b

        transpose_4x8h  v16, v17, v18, v19, v2,  v3,  v4,  v5

        sqrshrn         v8.4h,   v24.4s,  #1
        sqrshrn         v9.4h,   v25.4s,  #1
        sqrshrn         v10.4h,  v26.4s,  #1
        sqrshrn         v11.4h,  v27.4s,  #1
        sqrshrn2        v8.8h,   v28.4s,  #1
        sqrshrn2        v9.8h,   v29.4s,  #1
        sqrshrn2        v10.8h,  v30.4s,  #1
        sqrshrn2        v11.8h,  v31.4s,  #1

        transpose_4x8h  v8,  v9, v10, v11, v2,  v3,  v4,  v5

        blr             x5

        mov             x6,  x0
        load_add_store_8x8 x6, x7

        mov             v16.16b, v8.16b
        mov             v17.16b, v9.16b
        mov             v18.16b, v10.16b
        mov             v19.16b, v11.16b
        mov             v20.16b, v12.16b
        mov             v21.16b, v13.16b
        mov             v22.16b, v14.16b
        mov             v23.16b, v15.16b

        blr             x5

        add             x0,  x0,  #16
        load_add_store_8x8 x0, x7

        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40
        ret             x15
endfunc

function inv_txfm_add_8x16_neon
        mov             x15, x30
        stp             d8,  d9,  [sp, #-0x20]!
        stp             d10, d11, [sp, #0x10]
        ldrh            w12, [x13, #4]

        mov             x11, #64

        cmp             w3,  w12
        ldrh            w12, [x13, #2]
        b.lt            1f

        add             x6,  x2,  #48
        movi            v4.4s,   #0
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s
        ld1             {\i},    [x6]
        st1             {v4.4s}, [x6], x11
.endr
        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        blr             x4

        sqrshrn         v28.4h,  v16.4s,  #1
        sqrshrn         v29.4h,  v17.4s,  #1
        sqrshrn         v30.4h,  v18.4s,  #1
        sqrshrn         v31.4h,  v19.4s,  #1
        sqrshrn2        v28.8h,  v20.4s,  #1
        sqrshrn2        v29.8h,  v21.4s,  #1
        sqrshrn2        v30.8h,  v22.4s,  #1
        sqrshrn2        v31.8h,  v23.4s,  #1
        transpose_4x8h  v28, v29, v30, v31, v2, v3, v4, v5

        b               2f

1:
.irp i, v28.8h, v29.8h, v30.8h, v31.8h
        movi            \i,  #0
.endr

2:
        cmp             w3,  w12
        ldrh            w12, [x13, #0]
        b.lt            1f

        add             x6,  x2,  #32
        movi            v4.4s,   #0
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s
        ld1             {\i},    [x6]
        st1             {v4.4s}, [x6], x11
.endr
        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        blr             x4

        sqrshrn         v24.4h,  v16.4s,  #1
        sqrshrn         v25.4h,  v17.4s,  #1
        sqrshrn         v26.4h,  v18.4s,  #1
        sqrshrn         v27.4h,  v19.4s,  #1
        sqrshrn2        v24.8h,  v20.4s,  #1
        sqrshrn2        v25.8h,  v21.4s,  #1
        sqrshrn2        v26.8h,  v22.4s,  #1
        sqrshrn2        v27.8h,  v23.4s,  #1
        transpose_4x8h  v24, v25, v26, v27, v2, v3, v4, v5

        b               2f

1:
.irp i, v24.8h, v25.8h, v26.8h, v27.8h
        movi            \i,  #0
.endr

2:
        cmp             w3,  w12
        b.lt            1f

        add             x6,  x2,  #16
        movi            v4.4s,   #0
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s
        ld1             {\i},    [x6]
        st1             {v4.4s}, [x6], x11
.endr
        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        blr             x4

        sqrshrn         v8.4h,   v16.4s,  #1
        sqrshrn         v9.4h,   v17.4s,  #1
        sqrshrn         v10.4h,  v18.4s,  #1
        sqrshrn         v11.4h,  v19.4s,  #1
        sqrshrn2        v8.8h,   v20.4s,  #1
        sqrshrn2        v9.8h,   v21.4s,  #1
        sqrshrn2        v10.8h,  v22.4s,  #1
        sqrshrn2        v11.8h,  v23.4s,  #1
        transpose_4x8h  v8,  v9,  v10, v11, v2, v3, v4, v5

        b               2f

1:
.irp i, v8.8h, v9.8h, v10.8h, v11.8h
        movi            \i,  #0
.endr

2:
        movi            v4.4s,   #0
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s
        ld1             {\i},    [x2]
        st1             {v4.4s}, [x2], x11
.endr
        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        blr             x4

        sqrshrn         v16.4h,  v16.4s,  #1
        sqrshrn         v17.4h,  v17.4s,  #1
        sqrshrn         v18.4h,  v18.4s,  #1
        sqrshrn         v19.4h,  v19.4s,  #1
        sqrshrn2        v16.8h,  v20.4s,  #1
        sqrshrn2        v17.8h,  v21.4s,  #1
        sqrshrn2        v18.8h,  v22.4s,  #1
        sqrshrn2        v19.8h,  v23.4s,  #1
        transpose_4x8h  v16, v17, v18, v19, v2, v3, v4, v5

        mov             v20.16b, v8.16b
        mov             v21.16b, v9.16b
        mov             v22.16b, v10.16b
        mov             v23.16b, v11.16b

        blr             x5

        load_add_store_8x16 x0, x6

        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x20

        ret             x15
endfunc

const eob_8x16
        .short 10, 43, 75, 128
endconst

const eob_8x16_identity1
        .short 4, 64, 96, 128
endconst

const eob_8x16_identity2
        .short 4, 8, 12, 128
endconst

.macro def_fn_816 w, h, txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
        adr             x4,  inv_\txfm1\()_4s_x\w\()_neon
        movrel          x5,  X(inv_\txfm2\()_8h_x\h\()_neon)
.ifc \txfm1, identity
.ifc \txfm2, identity
        movrel          x13, eob_8x16
.else
        movrel          x13, eob_8x16_identity1
.endif
.else
.ifc \txfm2, identity
        movrel          x13, eob_8x16_identity2
.else
        movrel          x13, eob_8x16
.endif
.endif
.if \h == 8
        ldrh            w13, [x13]
.endif
        b               inv_txfm_add_\w\()x\h\()_neon
endfunc
.endm

.macro def_fns_816 w, h
def_fn_816 \w, \h, dct, dct
def_fn_816 \w, \h, identity, identity
def_fn_816 \w, \h, dct, adst
def_fn_816 \w, \h, dct, flipadst
def_fn_816 \w, \h, dct, identity
def_fn_816 \w, \h, adst, dct
def_fn_816 \w, \h, adst, adst
def_fn_816 \w, \h, adst, flipadst
def_fn_816 \w, \h, flipadst, dct
def_fn_816 \w, \h, flipadst, adst
def_fn_816 \w, \h, flipadst, flipadst
def_fn_816 \w, \h, identity, dct
def_fn_816 \w, \h, adst, identity
def_fn_816 \w, \h, flipadst, identity
def_fn_816 \w, \h, identity, adst
def_fn_816 \w, \h, identity, flipadst
.endm

def_fns_816 8, 16
def_fns_816 16, 8

function inv_dct32_odd_4s_x16_neon
        movrel          x16, idct_coeffs, 4*16
        ld1             {v0.4s, v1.4s}, [x16], #32

        mul_mls         v2,  v16, v31, v0.s[0], v0.s[1] // -> t16a
        mul_mla         v4,  v16, v31, v0.s[1], v0.s[0] // -> t31a
        mul_mls         v6,  v24, v23, v0.s[2], v0.s[3] // -> t17a
        srshr           v16.4s, v2.4s,  #12             // t16a
        srshr           v31.4s, v4.4s,  #12             // t31a
        mul_mla         v2,  v24, v23, v0.s[3], v0.s[2] // -> t30a
        mul_mls         v4,  v20, v27, v1.s[0], v1.s[1] // -> t18a
        srshr           v24.4s, v6.4s,  #12             // t17a
        srshr           v23.4s, v2.4s,  #12             // t30a
        mul_mla         v6,  v20, v27, v1.s[1], v1.s[0] // -> t29a
        mul_mls         v2,  v28, v19, v1.s[2], v1.s[3] // -> t19a
        srshr           v20.4s, v4.4s,  #12             // t18a
        srshr           v27.4s, v6.4s,  #12             // t29a
        mul_mla         v4,  v28, v19, v1.s[3], v1.s[2] // -> t28a
        ld1             {v0.4s, v1.4s}, [x16]
        sub             x16, x16, #4*24
        mul_mls         v6,  v18, v29, v0.s[0], v0.s[1] // -> t20a
        srshr           v28.4s, v2.4s,  #12             // t19a
        srshr           v19.4s, v4.4s,  #12             // t28a
        mul_mla         v2,  v18, v29, v0.s[1], v0.s[0] // -> t27a
        mul_mls         v4,  v26, v21, v0.s[2], v0.s[3] // -> t21a
        srshr           v18.4s, v6.4s,  #12             // t20a
        srshr           v29.4s, v2.4s,  #12             // t27a
        mul_mla         v6,  v26, v21, v0.s[3], v0.s[2] // -> t26a
        mul_mls         v2,  v22, v25, v1.s[0], v1.s[1] // -> t22a
        srshr           v26.4s, v4.4s,  #12             // t21a
        srshr           v21.4s, v6.4s,  #12             // t26a
        mul_mla         v4,  v22, v25, v1.s[1], v1.s[0] // -> t25a
        mul_mls         v6,  v30, v17, v1.s[2], v1.s[3] // -> t23a
        srshr           v22.4s, v2.4s,  #12             // t22a
        srshr           v25.4s, v4.4s,  #12             // t25a
        mul_mla         v2,  v30, v17, v1.s[3], v1.s[2] // -> t24a
        srshr           v30.4s, v6.4s,  #12             // t23a
        srshr           v17.4s, v2.4s,  #12             // t24a

        ld1             {v0.4s, v1.4s}, [x16]

        movi            v5.4s,   #1, msl #16     // row_clip_max = ~(~bdmax << 7), 0x1ffff
        mvni            v4.4s,   #1, msl #16     // row_clip_min = (~bdmax << 7), 0xfffe0000

        sqsub           v2.4s,   v16.4s,  v24.4s // t17
        sqadd           v16.4s,  v16.4s,  v24.4s // t16
        sqsub           v3.4s,   v31.4s,  v23.4s // t30
        sqadd           v31.4s,  v31.4s,  v23.4s // t31
        sqsub           v24.4s,  v28.4s,  v20.4s // t18
        sqadd           v28.4s,  v28.4s,  v20.4s // t19
        sqadd           v23.4s,  v18.4s,  v26.4s // t20
        sqsub           v18.4s,  v18.4s,  v26.4s // t21
        sqsub           v20.4s,  v30.4s,  v22.4s // t22
        sqadd           v30.4s,  v30.4s,  v22.4s // t23
        sqadd           v26.4s,  v17.4s,  v25.4s // t24
        sqsub           v17.4s,  v17.4s,  v25.4s // t25
        sqsub           v22.4s,  v29.4s,  v21.4s // t26
        sqadd           v29.4s,  v29.4s,  v21.4s // t27
        sqadd           v25.4s,  v19.4s,  v27.4s // t28
        sqsub           v19.4s,  v19.4s,  v27.4s // t29

.irp r, v2, v16, v3, v31, v24, v28, v23, v18, v20, v30, v26, v17, v22, v29, v25, v19
        smin            \r\().4s, \r\().4s, v5.4s
.endr
.irp r, v2, v16, v3, v31, v24, v28, v23, v18, v20, v30, v26, v17, v22, v29, v25, v19
        smax            \r\().4s, \r\().4s, v4.4s
.endr

        mul_mls         v7,  v3,  v2,  v1.s[0], v1.s[1] // -> t17a
        mul_mla         v6,  v3,  v2,  v1.s[1], v1.s[0] // -> t30a
        mul_mla         v2,  v19, v24, v1.s[1], v1.s[0] // -> t18a
        srshr           v21.4s, v7.4s,  #12             // t17a
        srshr           v27.4s, v6.4s,  #12             // t30a
        neg             v2.4s,   v2.4s                  // -> t18a
        mul_mls         v7,  v19, v24, v1.s[0], v1.s[1] // -> t29a
        mul_mls         v6,  v22, v18, v1.s[2], v1.s[3] // -> t21a
        srshr           v19.4s, v2.4s,  #12             // t18a
        srshr           v24.4s, v7.4s,  #12             // t29a
        mul_mla         v2,  v22, v18, v1.s[3], v1.s[2] // -> t26a
        mul_mla         v7,  v17, v20, v1.s[3], v1.s[2] // -> t22a
        srshr           v22.4s, v6.4s,  #12             // t21a
        srshr           v18.4s, v2.4s,  #12             // t26a
        neg             v7.4s,   v7.4s                  // -> t22a
        mul_mls         v6,  v17, v20, v1.s[2], v1.s[3] // -> t25a
        srshr           v17.4s, v7.4s,  #12             // t22a
        srshr           v20.4s, v6.4s,  #12             // t25a

        sqsub           v2.4s,   v27.4s,  v24.4s // t29
        sqadd           v27.4s,  v27.4s,  v24.4s // t30
        sqsub           v3.4s,   v21.4s,  v19.4s // t18
        sqadd           v21.4s,  v21.4s,  v19.4s // t17
        sqsub           v24.4s,  v16.4s,  v28.4s // t19a
        sqadd           v16.4s,  v16.4s,  v28.4s // t16a
        sqsub           v19.4s,  v30.4s,  v23.4s // t20a
        sqadd           v30.4s,  v30.4s,  v23.4s // t23a
        sqsub           v28.4s,  v17.4s,  v22.4s // t21
        sqadd           v17.4s,  v17.4s,  v22.4s // t22
        sqadd           v23.4s,  v26.4s,  v29.4s // t24a
        sqsub           v26.4s,  v26.4s,  v29.4s // t27a
        sqadd           v22.4s,  v20.4s,  v18.4s // t25
        sqsub           v20.4s,  v20.4s,  v18.4s // t26
        sqsub           v29.4s,  v31.4s,  v25.4s // t28a
        sqadd           v31.4s,  v31.4s,  v25.4s // t31a

.irp r, v2, v27, v3, v21, v24, v16, v19, v30, v28, v17, v23, v26, v22, v20, v29, v31
        smin            \r\().4s, \r\().4s, v5.4s
.endr
.irp r, v2, v27, v3, v21, v24, v16, v19, v30, v28, v17, v23, v26, v22, v20, v29, v31
        smax            \r\().4s, \r\().4s, v4.4s
.endr

        mul_mls         v7,  v2,  v3,  v0.s[2], v0.s[3] // -> t18a
        mul_mla         v6,  v2,  v3,  v0.s[3], v0.s[2] // -> t29a
        mul_mls         v2,  v29, v24, v0.s[2], v0.s[3] // -> t19
        srshr           v18.4s, v7.4s,  #12             // t18a
        srshr           v25.4s, v6.4s,  #12             // t29a
        mul_mla         v7,  v29, v24, v0.s[3], v0.s[2] // -> t28
        mul_mla         v6,  v26, v19, v0.s[3], v0.s[2] // -> t20
        srshr           v29.4s, v2.4s,  #12             // t19
        srshr           v24.4s, v7.4s,  #12             // t28
        neg             v6.4s,   v6.4s                  // -> t20
        mul_mls         v2,  v26, v19, v0.s[2], v0.s[3] // -> t27
        mul_mla         v7,  v20, v28, v0.s[3], v0.s[2] // -> t21a
        srshr           v26.4s, v6.4s,  #12             // t20
        srshr           v19.4s, v2.4s,  #12             // t27
        neg             v7.4s,   v7.4s                  // -> t21a
        mul_mls         v6,  v20, v28, v0.s[2], v0.s[3] // -> t26a
        srshr           v20.4s, v7.4s,  #12             // t21a
        srshr           v28.4s, v6.4s,  #12             // t26a

        sqsub           v2.4s,   v16.4s,  v30.4s // t23
        sqadd           v16.4s,  v16.4s,  v30.4s // t16 = out16
        sqsub           v3.4s,   v31.4s,  v23.4s // t24
        sqadd           v31.4s,  v31.4s,  v23.4s // t31 = out31
        sqsub           v23.4s,  v21.4s,  v17.4s // t22a
        sqadd           v17.4s,  v21.4s,  v17.4s // t17a = out17
        sqadd           v30.4s,  v27.4s,  v22.4s // t30a = out30
        sqsub           v21.4s,  v27.4s,  v22.4s // t25a
        sqsub           v27.4s,  v18.4s,  v20.4s // t21
        sqadd           v18.4s,  v18.4s,  v20.4s // t18 = out18
        sqadd           v7.4s,   v29.4s,  v26.4s // t19a = out19
        sqsub           v26.4s,  v29.4s,  v26.4s // t20a
        sqadd           v29.4s,  v25.4s,  v28.4s // t29 = out29
        sqsub           v25.4s,  v25.4s,  v28.4s // t26
        sqadd           v28.4s,  v24.4s,  v19.4s // t28a = out28
        sqsub           v24.4s,  v24.4s,  v19.4s // t27a
        mov             v19.16b, v7.16b          // out19

.irp r, v2, v16, v3, v31, v23, v17, v30, v21, v27, v18, v19, v26, v29, v25, v28, v24
        smin            \r\().4s, \r\().4s, v5.4s
.endr
.irp r, v2, v16, v3, v31, v23, v17, v30, v21, v27, v18, v19, v26, v29, v25, v28, v24
        smax            \r\().4s, \r\().4s, v4.4s
.endr

        mul_mls         v7,  v24, v26, v0.s[0], v0.s[0] // -> t20
        mul_mla         v6,  v24, v26, v0.s[0], v0.s[0] // -> t27
        srshr           v20.4s, v7.4s,  #12             // t20
        srshr           v22.4s, v6.4s,  #12             // t27

        mul_mla         v7,  v25, v27, v0.s[0], v0.s[0] // -> t26a
        mul_mls         v6,  v25, v27, v0.s[0], v0.s[0] // -> t21a
        mov             v27.16b,  v22.16b               // t27
        srshr           v26.4s, v7.4s,  #12             // t26a

        mul_mls         v24, v21, v23, v0.s[0], v0.s[0] // -> t22
        mul_mla         v7,  v21, v23, v0.s[0], v0.s[0] // -> t25
        srshr           v21.4s, v6.4s,  #12             // t21a
        srshr           v22.4s, v24.4s, #12             // t22
        srshr           v25.4s, v7.4s,  #12             // t25

        mul_mls         v7,  v3,  v2,  v0.s[0], v0.s[0] // -> t23a
        mul_mla         v6,  v3,  v2,  v0.s[0], v0.s[0] // -> t24a
        srshr           v23.4s, v7.4s,  #12             // t23a
        srshr           v24.4s, v6.4s,  #12             // t24a

        ret
endfunc

.macro def_horz_32 scale=0, shift=2, suffix
function inv_txfm_horz\suffix\()_dct_32x4_neon
        mov             x14, x30
        movi            v7.4s,  #0
        lsl             x8,  x8,  #1
.if \scale
        movz            w16, #2896*8, lsl #16
        dup             v0.2s,   w16
.endif

.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        ld1             {\i}, [x7]
        st1             {v7.4s}, [x7], x8
.endr
        sub             x7,  x7,  x8, lsl #4
        add             x7,  x7,  x8, lsr #1
.if \scale
        scale_input     .4s, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .4s, v0.s[0], v24, v25, v26, v27, v28, v29, v30, v31
.endif
        bl              inv_dct_4s_x16_neon

        // idct_16 leaves the row_clip_max/min constants in v5 and v4
.irp r, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31
        smin_4s         \r, \r, v5
.endr
.irp r, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31
        smax_4s         \r, \r, v4
.endr

        transpose_4x4s  v16, v17, v18, v19, v2,  v3,  v4,  v5
        transpose_4x4s  v20, v21, v22, v23, v2,  v3,  v4,  v5
        transpose_4x4s  v24, v25, v26, v27, v2,  v3,  v4,  v5
        transpose_4x4s  v28, v29, v30, v31, v2,  v3,  v4,  v5

.macro store1 r0, r1, r2, r3
        st1             {\r0}, [x6], #16
        st1             {\r1}, [x6], #16
        st1             {\r2}, [x6], #16
        st1             {\r3}, [x6], #16
.endm
        store1          v16.4s,  v20.4s,  v24.4s,  v28.4s
        store1          v17.4s,  v21.4s,  v25.4s,  v29.4s
        store1          v18.4s,  v22.4s,  v26.4s,  v30.4s
        store1          v19.4s,  v23.4s,  v27.4s,  v31.4s
.purgem store1
        sub             x6,  x6,  #64*4

        movi            v7.4s,  #0
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        ld1             {\i}, [x7]
        st1             {v7.4s}, [x7], x8
.endr
.if \scale
        // This relies on the fact that the idct also leaves the right coeff in v0.s[1]
        scale_input     .4s, v0.s[1], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .4s, v0.s[1], v24, v25, v26, v27, v28, v29, v30, v31
.endif
        bl              inv_dct32_odd_4s_x16_neon
        transpose_4x4s  v31, v30, v29, v28, v2,  v3,  v4,  v5
        transpose_4x4s  v27, v26, v25, v24, v2,  v3,  v4,  v5
        transpose_4x4s  v23, v22, v21, v20, v2,  v3,  v4,  v5
        transpose_4x4s  v19, v18, v17, v16, v2,  v3,  v4,  v5
.macro store2 r0, r1, r2, r3, shift
        ld1             {v0.4s, v1.4s, v2.4s, v3.4s}, [x6]
        sqsub           v4.4s,   v0.4s,   \r0
        sqadd           v0.4s,   v0.4s,   \r0
        sqsub           v5.4s,   v1.4s,   \r1
        sqadd           v1.4s,   v1.4s,   \r1
        sqsub           v6.4s,   v2.4s,   \r2
        sqadd           v2.4s,   v2.4s,   \r2
        sqsub           v7.4s,   v3.4s,   \r3
        sqadd           v3.4s,   v3.4s,   \r3
        sqrshrn         v0.4h,   v0.4s,   #\shift
        sqrshrn2        v0.8h,   v1.4s,   #\shift
        sqrshrn         v1.4h,   v2.4s,   #\shift
        sqrshrn2        v1.8h,   v3.4s,   #\shift
        sqrshrn         v2.4h,   v7.4s,   #\shift
        sqrshrn2        v2.8h,   v6.4s,   #\shift
        sqrshrn         v3.4h,   v5.4s,   #\shift
        sqrshrn2        v3.8h,   v4.4s,   #\shift
        st1             {v0.8h, v1.8h}, [x6], #32
        rev64           v2.8h,   v2.8h
        rev64           v3.8h,   v3.8h
        st1             {v2.8h, v3.8h}, [x6], #32
.endm

        store2          v31.4s,  v27.4s,  v23.4s,  v19.4s,  \shift
        store2          v30.4s,  v26.4s,  v22.4s,  v18.4s,  \shift
        store2          v29.4s,  v25.4s,  v21.4s,  v17.4s,  \shift
        store2          v28.4s,  v24.4s,  v20.4s,  v16.4s,  \shift
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

        bl              X(inv_dct_8h_x16_neon)

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
        bl              X(inv_dct32_odd_8h_x16_neon)

        neg             x9,  x8
        mov             x10, x6
        mvni            v1.8h,   #0xfc, lsl #8 // 0x3ff
.macro combine r0, r1, r2, r3, op, stride
        ld1             {v5.8h}, [x7],    \stride
        ld1             {v2.8h}, [x10],   x1
        ld1             {v6.8h}, [x7],    \stride
        ld1             {v3.8h}, [x10],   x1
        \op             v5.8h,   v5.8h,   \r0
        ld1             {v7.8h}, [x7],    \stride
        ld1             {v4.8h}, [x10],   x1
        srshr           v5.8h,   v5.8h,   #4
        \op             v6.8h,   v6.8h,   \r1
        usqadd          v2.8h,   v5.8h
        srshr           v6.8h,   v6.8h,   #4
        \op             v7.8h,   v7.8h,   \r2
        ld1             {v5.8h}, [x7],    \stride
        usqadd          v3.8h,   v6.8h
        smin            v2.8h,   v2.8h,   v1.8h
        srshr           v7.8h,   v7.8h,   #4
        \op             v5.8h,   v5.8h,   \r3
        st1             {v2.8h}, [x6],    x1
        ld1             {v2.8h}, [x10],   x1
        usqadd          v4.8h,   v7.8h
        smin            v3.8h,   v3.8h,   v1.8h
        srshr           v5.8h,   v5.8h,   #4
        st1             {v3.8h}, [x6],    x1
        usqadd          v2.8h,   v5.8h
        smin            v4.8h,   v4.8h,   v1.8h
        st1             {v4.8h}, [x6],    x1
        smin            v2.8h,   v2.8h,   v1.8h
        st1             {v2.8h}, [x6],    x1
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
        .short 10, 36, 78, 136, 210, 300, 406, 1024
endconst

const eob_16x32
        .short 10, 36, 78, 151, 215, 279, 343, 512
endconst

const eob_16x32_shortside
        .short 10, 36, 78, 512
endconst

const eob_8x32
        .short 10, 43, 75, 107, 139, 171, 203, 256
endconst

function inv_txfm_add_identity_identity_32x32_16bpc_neon, export=1
        movi            v0.8h,  #0
        movi            v1.8h,  #0
        movrel          x13, eob_32x32, 2

        mov             x8,  #4*32
1:
        mov             w9,  #0
        movrel          x12, eob_32x32, 2
2:
        add             w9,  w9,  #8
        ld1             {v16.4s, v17.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v18.4s, v19.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v20.4s, v21.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v22.4s, v23.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v24.4s, v25.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v26.4s, v27.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v28.4s, v29.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v30.4s, v31.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        sqxtn           v16.4h,  v16.4s
        sqxtn2          v16.8h,  v17.4s
        sqxtn           v17.4h,  v18.4s
        sqxtn2          v17.8h,  v19.4s
        sqxtn           v18.4h,  v20.4s
        sqxtn2          v18.8h,  v21.4s
        sqxtn           v19.4h,  v22.4s
        sqxtn2          v19.8h,  v23.4s
        sqxtn           v20.4h,  v24.4s
        sqxtn2          v20.8h,  v25.4s
        sqxtn           v21.4h,  v26.4s
        sqxtn2          v21.8h,  v27.4s
        sqxtn           v22.4h,  v28.4s
        sqxtn2          v22.8h,  v29.4s
        sqxtn           v23.4h,  v30.4s
        sqxtn2          v23.8h,  v31.4s
        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5

        load_add_store_8x8 x0, x7, shiftbits=2
        ldrh            w11, [x12], #4
        sub             x0,  x0,  x1, lsl #3
        add             x0,  x0,  #2*8
        cmp             w3,  w11
        b.ge            2b

        ldrh            w11, [x13], #4
        cmp             w3,  w11
        b.lt            9f

        sub             x0,  x0,  w9, uxtw #1
        add             x0,  x0,  x1, lsl #3
        msub            x2,  x8,  x9,  x2
        add             x2,  x2,  #4*8
        b               1b
9:
        ret
endfunc

.macro shift_16_regs op, shift
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        \op             \i,  \i,  #\shift
.endr
.endm

.macro def_identity_1632 w, h, wshort, hshort
function inv_txfm_add_identity_identity_\w\()x\h\()_16bpc_neon, export=1
        movz            w16, #2896*8, lsl #16
        movz            w17, #2*(5793-4096)*8, lsl #16
        movi            v0.4s,   #0
        movi            v1.4s,   #0
        movrel          x13, eob_16x32\hshort, 2

        mov             x8,  #4*\h
1:
        mov             w9,  #0
        movrel          x12, eob_16x32\wshort, 2
2:
        add             w9,  w9,  #8
        ld1             {v16.4s, v17.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        dup             v2.2s,   w16
        ld1             {v18.4s, v19.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        mov             v2.s[1], w17
        ld1             {v20.4s, v21.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v22.4s, v23.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v24.4s, v25.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v26.4s, v27.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v28.4s, v29.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v30.4s, v31.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        scale_input     .4s, v2.s[0], v16, v17, v18, v19, v20, v21, v22, v23
        scale_input     .4s, v2.s[0], v24, v25, v26, v27, v28, v29, v30, v31

.if \w == 16
        // 16x32
        identity_4x16_shift1 v2.s[1]
.else
        // 32x16
        shift_16_regs   sqshl, 1
        identity_4x16   v2.s[1]
.endif
        sqxtn           v16.4h,  v16.4s
        sqxtn2          v16.8h,  v17.4s
        sqxtn           v17.4h,  v18.4s
        sqxtn2          v17.8h,  v19.4s
        sqxtn           v18.4h,  v20.4s
        sqxtn2          v18.8h,  v21.4s
        sqxtn           v19.4h,  v22.4s
        sqxtn2          v19.8h,  v23.4s
        sqxtn           v20.4h,  v24.4s
        sqxtn2          v20.8h,  v25.4s
        sqxtn           v21.4h,  v26.4s
        sqxtn2          v21.8h,  v27.4s
        sqxtn           v22.4h,  v28.4s
        sqxtn2          v22.8h,  v29.4s
        sqxtn           v23.4h,  v30.4s
        sqxtn2          v23.8h,  v31.4s

        transpose_8x8h  v16, v17, v18, v19, v20, v21, v22, v23, v4, v5

.if \w == 16
        load_add_store_8x8 x0, x7, shiftbits=2
.else
        load_add_store_8x8 x0, x7, shiftbits=4
.endif
        ldrh            w11, [x12], #4
        sub             x0,  x0,  x1, lsl #3
        add             x0,  x0,  #16
        cmp             w3,  w11
        b.ge            2b

        ldrh            w11, [x13], #4
        cmp             w3,  w11
        b.lt            9f

        sub             x0,  x0,  w9, uxtw #1
        add             x0,  x0,  x1, lsl #3
        msub            x2,  x8,  x9,  x2
        add             x2,  x2,  #4*8
        b               1b
9:
        ret
endfunc
.endm

def_identity_1632 16, 32, _shortside,
def_identity_1632 32, 16, , _shortside

.macro def_identity_832 w, h
function inv_txfm_add_identity_identity_\w\()x\h\()_16bpc_neon, export=1
        movi            v0.4s,  #0
        movi            v1.4s,  #0
        // Working on 8x8 blocks, read every other entry from eob_8x32
        movrel          x13, eob_8x32, 2

        mov             w8,  #4*\h
1:
        // Working on 8x8 blocks, read every other entry from eob_8x32
        ldrh            w12, [x13], #4
        ld1             {v16.4s, v17.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v18.4s, v19.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v20.4s, v21.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v22.4s, v23.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v24.4s, v25.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v26.4s, v27.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v28.4s, v29.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8
        ld1             {v30.4s, v31.4s}, [x2]
        st1             {v0.4s, v1.4s},   [x2], x8

.if \w == 8
        sqrshrn         v16.4h,  v16.4s,  #1
        sqrshrn2        v16.8h,  v17.4s,  #1
        sqrshrn         v17.4h,  v18.4s,  #1
        sqrshrn2        v17.8h,  v19.4s,  #1
        sqrshrn         v18.4h,  v20.4s,  #1
        sqrshrn2        v18.8h,  v21.4s,  #1
        sqrshrn         v19.4h,  v22.4s,  #1
        sqrshrn2        v19.8h,  v23.4s,  #1
        sqrshrn         v20.4h,  v24.4s,  #1
        sqrshrn2        v20.8h,  v25.4s,  #1
        sqrshrn         v21.4h,  v26.4s,  #1
        sqrshrn2        v21.8h,  v27.4s,  #1
        sqrshrn         v22.4h,  v28.4s,  #1
        sqrshrn2        v22.8h,  v29.4s,  #1
        sqrshrn         v23.4h,  v30.4s,  #1
        sqrshrn2        v23.8h,  v31.4s,  #1
.else
        sqxtn           v16.4h,  v16.4s
        sqxtn2          v16.8h,  v17.4s
        sqxtn           v17.4h,  v18.4s
        sqxtn2          v17.8h,  v19.4s
        sqxtn           v18.4h,  v20.4s
        sqxtn2          v18.8h,  v21.4s
        sqxtn           v19.4h,  v22.4s
        sqxtn2          v19.8h,  v23.4s
        sqxtn           v20.4h,  v24.4s
        sqxtn2          v20.8h,  v25.4s
        sqxtn           v21.4h,  v26.4s
        sqxtn2          v21.8h,  v27.4s
        sqxtn           v22.4h,  v28.4s
        sqxtn2          v22.8h,  v29.4s
        sqxtn           v23.4h,  v30.4s
        sqxtn2          v23.8h,  v31.4s
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
        add             x2,  x2,  #4*8
.else
        sub             x0,  x0,  x1, lsl #3
        add             x0,  x0,  #2*8
.endif
        b               1b

9:
        ret
endfunc
.endm

def_identity_832 8, 32
def_identity_832 32, 8

function inv_txfm_add_dct_dct_32x32_16bpc_neon, export=1
        idct_dc         32,  32,  2

        mov             x15, x30
        sub             sp,  sp,  #2048
        movrel          x13, eob_32x32
        ldrh            w12, [x13], #2

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             x6,  sp,  #(\i*32*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 28
        ldrh            w12, [x13], #2
.endif
.endif
        add             x7,  x2,  #(\i*4)
        mov             x8,  #32*4
        bl              inv_txfm_horz_dct_32x4_neon
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
        add             x6,  x0,  #(\i*2)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_add_vert_dct_8x32_neon
.endr

        add             sp,  sp,  #2048
        ret             x15
endfunc

function inv_txfm_add_dct_dct_16x32_16bpc_neon, export=1
        idct_dc         16,  32,  1

        mov             x15, x30
        sub             sp,  sp,  #1024
        movrel          x13, eob_16x32
        ldrh            w12, [x13], #2
        adr             x4,  inv_dct_4s_x16_neon

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             x6,  sp,  #(\i*16*2)
        add             x7,  x2,  #(\i*4)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 28
        ldrh            w12, [x13], #2
.endif
.endif
        mov             x8,  #4*32
        bl              inv_txfm_horz_scale_16x4_neon
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #4
.rept 2
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8
        add             x6,  x0,  #(\i*2)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #16*2
        bl              inv_txfm_add_vert_dct_8x32_neon
.endr

        add             sp,  sp,  #1024
        ret             x15
endfunc

function inv_txfm_add_dct_dct_32x16_16bpc_neon, export=1
        idct_dc         32,  16,  1

        mov             x15, x30
        sub             sp,  sp,  #1024

        movrel          x13, eob_16x32
        movrel          x5,  X(inv_dct_8h_x16_neon)
        ldrh            w12, [x13], #2

.irp i, 0, 4, 8, 12
        add             x6,  sp,  #(\i*32*2)
        add             x7,  x2,  #(\i*4)
.if \i > 0
        mov             w8,  #(16 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 12
        ldrh            w12, [x13], #2
.endif
.endif
        mov             x8,  #4*16
        bl              inv_txfm_horz_scale_dct_32x4_neon
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
        add             x6,  x0,  #(\i*2)
        add             x7,  sp,  #(\i*2)
        mov             x8,  #32*2
        bl              inv_txfm_add_vert_8x16_neon
.endr

        add             sp,  sp,  #1024
        ret             x15
endfunc

function inv_txfm_add_dct_dct_8x32_16bpc_neon, export=1
        idct_dc         8,   32, 2

        mov             x15, x30
        sub             sp,  sp,  #512

        movrel          x13, eob_8x32

        movi            v28.4s,  #0
        mov             x8,  #4*32
        mov             w9,  #32
        mov             x6,  sp
        mov             x7,  x2
1:
.irp i, 16, 17, 18, 19, 20, 21, 22, 23
        ld1             {v\i\().4s}, [x7]
        st1             {v28.4s}, [x7], x8
.endr
        ldrh            w12, [x13], #2
        sub             w9,  w9,  #4
        sub             x7,  x7,  x8, lsl #3
        add             x7,  x7,  #4*4

        bl              inv_dct_4s_x8_neon

        sqrshrn         v16.4h,  v16.4s,  #2
        sqrshrn         v17.4h,  v17.4s,  #2
        sqrshrn         v18.4h,  v18.4s,  #2
        sqrshrn         v19.4h,  v19.4s,  #2
        sqrshrn2        v16.8h,  v20.4s,  #2
        sqrshrn2        v17.8h,  v21.4s,  #2
        sqrshrn2        v18.8h,  v22.4s,  #2
        sqrshrn2        v19.8h,  v23.4s,  #2

        transpose_4x8h  v16, v17, v18, v19, v2,  v3,  v4,  v5

        cmp             w3,  w12
        st1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x6], #64

        b.ge            1b
        cbz             w9,  3f

        movi            v29.8h,  #0
        movi            v30.8h,  #0
        movi            v31.8h,  #0
2:
        subs            w9,  w9,  #4
        st1             {v28.8h,v29.8h,v30.8h,v31.8h}, [x6], #64
        b.gt            2b

3:
        mov             x6,  x0
        mov             x7,  sp
        mov             x8,  #8*2
        bl              inv_txfm_add_vert_dct_8x32_neon

        add             sp,  sp,  #512
        ret             x15
endfunc

function inv_txfm_add_dct_dct_32x8_16bpc_neon, export=1
        idct_dc         32,  8,   2

        mov             x15, x30
        sub             sp,  sp,  #512

.irp i, 0, 4
        add             x6,  sp,  #(\i*32*2)
        add             x7,  x2,  #(\i*4)
.if \i > 0
        cmp             w3,  #10
        b.lt            1f
.endif
        mov             x8,  #8*4
        bl              inv_txfm_horz_dct_32x4_neon
.endr
        b               2f

1:
        movi            v4.8h,   #0
        movi            v5.8h,   #0
        movi            v6.8h,   #0
        movi            v7.8h,   #0
.rept 4
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr

2:
        mov             x8,  #2*32
        mov             w9,  #0
1:
        add             x6,  x0,  x9, lsl #1
        add             x7,  sp,  x9, lsl #1 // #(\i*2)

.irp i, 16, 17, 18, 19, 20, 21, 22, 23
        ld1             {v\i\().8h}, [x7], x8
.endr
        add             w9,  w9,  #8

        bl              X(inv_dct_8h_x8_neon)

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

        ld1             {v0.4s, v1.4s}, [x17], #32

        sqrdmulh        v23.4s,  v16.4s,  v0.s[1]       // t63a
        sqrdmulh        v16.4s,  v16.4s,  v0.s[0]       // t32a
        sqrdmulh        v22.4s,  v17.4s,  v0.s[2]       // t62a
        sqrdmulh        v17.4s,  v17.4s,  v0.s[3]       // t33a
        sqrdmulh        v21.4s,  v18.4s,  v1.s[1]       // t61a
        sqrdmulh        v18.4s,  v18.4s,  v1.s[0]       // t34a
        sqrdmulh        v20.4s,  v19.4s,  v1.s[2]       // t60a
        sqrdmulh        v19.4s,  v19.4s,  v1.s[3]       // t35a

        ld1             {v0.4s}, [x17], #16

        sqadd           v24.4s,  v16.4s,  v17.4s        // t32
        sqsub           v25.4s,  v16.4s,  v17.4s        // t33
        sqsub           v26.4s,  v19.4s,  v18.4s        // t34
        sqadd           v27.4s,  v19.4s,  v18.4s        // t35
        sqadd           v28.4s,  v20.4s,  v21.4s        // t60
        sqsub           v29.4s,  v20.4s,  v21.4s        // t61
        sqsub           v30.4s,  v23.4s,  v22.4s        // t62
        sqadd           v31.4s,  v23.4s,  v22.4s        // t63

.irp r, v24, v25, v26, v27, v28, v29, v30, v31
        smin_4s         \r, \r, v5
.endr
.irp r, v24, v25, v26, v27, v28, v29, v30, v31
        smax_4s         \r, \r, v4
.endr

        mul_mla         v2,  v29, v26, v0.s[0], v0.s[1] // -> t34a
        mul_mls         v7,  v29, v26, v0.s[1], v0.s[0] // -> t61a
        neg             v2.4s,   v2.4s                  // t34a
        mul_mls         v6,  v30, v25, v0.s[1], v0.s[0] // -> t33a
        srshr           v26.4s, v2.4s,  #12             // t34a
        mul_mla         v2,  v30, v25, v0.s[0], v0.s[1] // -> t62a
        srshr           v29.4s, v7.4s,  #12             // t61a
        srshr           v25.4s, v6.4s,  #12             // t33a
        srshr           v30.4s, v2.4s,  #12             // t62a

        sqadd           v16.4s,  v24.4s,  v27.4s        // t32a
        sqsub           v19.4s,  v24.4s,  v27.4s        // t35a
        sqadd           v17.4s,  v25.4s,  v26.4s        // t33
        sqsub           v18.4s,  v25.4s,  v26.4s        // t34
        sqsub           v20.4s,  v31.4s,  v28.4s        // t60a
        sqadd           v23.4s,  v31.4s,  v28.4s        // t63a
        sqsub           v21.4s,  v30.4s,  v29.4s        // t61
        sqadd           v22.4s,  v30.4s,  v29.4s        // t62

.irp r, v16, v19, v17, v18, v20, v23, v21, v22
        smin_4s         \r, \r, v5
.endr
.irp r, v16, v19, v17, v18, v20, v23, v21, v22
        smax_4s         \r, \r, v4
.endr

        mul_mla         v2,  v21, v18, v0.s[2], v0.s[3] // -> t61a
        mul_mls         v7,  v21, v18, v0.s[3], v0.s[2] // -> t34a
        mul_mla         v6,  v20, v19, v0.s[2], v0.s[3] // -> t60
        srshr           v21.4s, v2.4s,  #12             // t61a
        srshr           v18.4s, v7.4s,  #12             // t34a
        mul_mls         v2,  v20, v19, v0.s[3], v0.s[2] // -> t35
        srshr           v20.4s, v6.4s,  #12             // t60
        srshr           v19.4s, v2.4s,  #12             // t35

        st1             {v16.4s, v17.4s, v18.4s, v19.4s}, [x6], #64
        st1             {v20.4s, v21.4s, v22.4s, v23.4s}, [x6], #64

        ret
endfunc

function inv_dct64_step2_neon
        movrel          x16, idct_coeffs
        ld1             {v0.4s}, [x16]
1:
        // t32a/33/34a/35/60/61a/62/63a
        // t56a/57/58a/59/36/37a/38/39a
        // t40a/41/42a/43/52/53a/54/55a
        // t48a/49/50a/51/44/45a/46/47a
        ldr             q16, [x6, #4*4*0]  // t32a
        ldr             q17, [x9, #4*4*8]  // t39a
        ldr             q18, [x9, #4*4*0]  // t63a
        ldr             q19, [x6, #4*4*8]  // t56a
        ldr             q20, [x6, #4*4*16] // t40a
        ldr             q21, [x9, #4*4*24] // t47a
        ldr             q22, [x9, #4*4*16] // t55a
        ldr             q23, [x6, #4*4*24] // t48a

        sqadd           v24.4s,  v16.4s, v17.4s         // t32
        sqsub           v25.4s,  v16.4s, v17.4s         // t39
        sqadd           v26.4s,  v18.4s, v19.4s         // t63
        sqsub           v27.4s,  v18.4s, v19.4s         // t56
        sqsub           v28.4s,  v21.4s, v20.4s         // t40
        sqadd           v29.4s,  v21.4s, v20.4s         // t47
        sqadd           v30.4s,  v23.4s, v22.4s         // t48
        sqsub           v31.4s,  v23.4s, v22.4s         // t55

.irp r, v24, v25, v26, v27, v28, v29, v30, v31
        smin_4s         \r, \r, v5
.endr
.irp r, v24, v25, v26, v27, v28, v29, v30, v31
        smax_4s         \r, \r, v4
.endr

        mul_mla         v2,  v27, v25, v0.s[3], v0.s[2] // -> t56a
        mul_mls         v7,  v27, v25, v0.s[2], v0.s[3] // -> t39a
        mul_mla         v6,  v31, v28, v0.s[3], v0.s[2] // -> t40a
        srshr           v25.4s, v2.4s,  #12             // t56a
        srshr           v27.4s, v7.4s,  #12             // t39a
        neg             v6.4s,   v6.4s                  // t40a
        mul_mls         v2,  v31, v28, v0.s[2], v0.s[3] // -> t55a
        srshr           v31.4s, v6.4s,  #12             // t40a
        srshr           v28.4s, v2.4s,  #12             // t55a

        sqadd           v16.4s,  v24.4s,  v29.4s        // t32a
        sqsub           v19.4s,  v24.4s,  v29.4s        // t47a
        sqadd           v17.4s,  v27.4s,  v31.4s        // t39
        sqsub           v18.4s,  v27.4s,  v31.4s        // t40
        sqsub           v20.4s,  v26.4s,  v30.4s        // t48a
        sqadd           v23.4s,  v26.4s,  v30.4s        // t63a
        sqsub           v21.4s,  v25.4s,  v28.4s        // t55
        sqadd           v22.4s,  v25.4s,  v28.4s        // t56

.irp r, v16, v19, v17, v18, v20, v23, v21, v22
        smin_4s         \r, \r, v5
.endr
.irp r, v16, v19, v17, v18, v20, v23, v21, v22
        smax_4s         \r, \r, v4
.endr

        mul_mls         v2,  v21, v18, v0.s[0], v0.s[0] // -> t40a
        mul_mla         v7,  v21, v18, v0.s[0], v0.s[0] // -> t55a
        mul_mls         v6,  v20, v19, v0.s[0], v0.s[0] // -> t47
        srshr           v18.4s, v2.4s,  #12             // t40a
        srshr           v21.4s, v7.4s,  #12             // t55a
        mul_mla         v2,  v20, v19, v0.s[0], v0.s[0] // -> t48
        srshr           v19.4s, v6.4s,  #12             // t47
        srshr           v20.4s, v2.4s,  #12             // t48

        str             q16, [x6, #4*4*0]  // t32a
        str             q17, [x9, #4*4*0]  // t39
        str             q18, [x6, #4*4*8]  // t40a
        str             q19, [x9, #4*4*8]  // t47
        str             q20, [x6, #4*4*16] // t48
        str             q21, [x9, #4*4*16] // t55a
        str             q22, [x6, #4*4*24] // t56
        str             q23, [x9, #4*4*24] // t63a

        add             x6,  x6,  #4*4
        sub             x9,  x9,  #4*4
        cmp             x6,  x9
        b.lt            1b
        ret
endfunc

.macro load8 src, strd, zero, clear
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s
.if \clear
        ld1             {\i}, [\src]
        st1             {\zero}, [\src], \strd
.else
        ld1             {\i}, [\src], \strd
.endif
.endr
.endm

.macro store16 dst
.irp i, v16.4s, v17.4s, v18.4s, v19.4s, v20.4s, v21.4s, v22.4s, v23.4s, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        st1             {\i}, [\dst], #16
.endr
.endm

.macro clear_upper8
.irp i, v24.4s, v25.4s, v26.4s, v27.4s, v28.4s, v29.4s, v30.4s, v31.4s
        movi            \i,  #0
.endr
.endm

.macro movi_if reg, val, cond
.if \cond
        movi            \reg, \val
.endif
.endm

.macro movz16dup_if reg, gpr, val, cond
.if \cond
        movz            \gpr, \val, lsl #16
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
        scale_input     .4s, \c, \r0, \r1, \r2, \r3, \r4, \r5, \r6, \r7
.endif
.endm

.macro def_dct64_func suffix, clear=0, scale=0
function inv_txfm_dct\suffix\()_4s_x64_neon
        mov             x14, x30
        mov             x6,  sp
        lsl             x8,  x8,  #2

        movz16dup_if    v0.2s, w16, #2896*8, \scale
        movi_if         v7.4s,  #0, \clear
        load8           x7,  x8,  v7.4s, \clear
        clear_upper8
        sub             x7,  x7,  x8, lsl #3
        add             x7,  x7,  x8, lsr #1
        scale_if        \scale, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23

        bl              inv_dct_4s_x16_neon

        // idct_16 leaves the row_clip_max/min constants in v5 and v4
.irp r, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31
        smin_4s         \r, \r, v5
.endr
.irp r, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31
        smax_4s         \r, \r, v4
.endr

        store16         x6

        movz16dup_if    v0.2s, w16, #2896*8, \scale
        movi_if         v7.8h,  #0, \clear
        load8           x7,  x8,  v7.4s, \clear
        clear_upper8
        sub             x7,  x7,  x8, lsl #3
        lsr             x8,  x8,  #1
        sub             x7,  x7,  x8, lsr #1
        scale_if        \scale, v0.s[0], v16, v17, v18, v19, v20, v21, v22, v23

        bl              inv_dct32_odd_4s_x16_neon

        add             x10, x6,  #16*15
        sub             x6,  x6,  #16*16

        mov             x9,  #-16

        movi            v1.4s,  #1, msl #16 // row_clip_max = ~(~bdmax << 7), 0x1ffff
        mvni            v0.4s,  #1, msl #16 // row_clip_min = (~bdmax << 7), 0xfffe0000

.macro store_addsub r0, r1, r2, r3
        ld1             {v2.4s}, [x6], #16
        ld1             {v3.4s}, [x6], #16
        sqadd           v6.4s,  v2.4s,  \r0
        sqsub           \r0,    v2.4s,  \r0
        ld1             {v4.4s}, [x6], #16
        sqadd           v7.4s,  v3.4s,  \r1
        sqsub           \r1,    v3.4s,  \r1
        smin            v6.4s,  v6.4s,  v1.4s
        smin            \r0,    \r0,    v1.4s
        ld1             {v5.4s}, [x6], #16
        sqadd           v2.4s,  v4.4s,  \r2
        sub             x6,  x6,  #16*4
        smax            v6.4s,  v6.4s,  v0.4s
        smax            \r0,    \r0,    v0.4s
        sqsub           \r2,    v4.4s,  \r2
        smin            v7.4s,  v7.4s,  v1.4s
        smin            \r1,    \r1,    v1.4s
        st1             {v6.4s}, [x6], #16
        st1             {\r0},   [x10], x9
        smin            v2.4s,  v2.4s,  v1.4s
        smin            \r2,    \r2,    v1.4s
        smax            v7.4s,  v7.4s,  v0.4s
        smax            \r1,    \r1,    v0.4s
        sqadd           v3.4s,  v5.4s,  \r3
        sqsub           \r3,    v5.4s,  \r3
        smax            v2.4s,  v2.4s,  v0.4s
        smax            \r2,    \r2,    v0.4s
        smin            v3.4s,  v3.4s,  v1.4s
        smin            \r3,    \r3,    v1.4s
        st1             {v7.4s}, [x6], #16
        st1             {\r1},   [x10], x9
        smax            v3.4s,  v3.4s,  v0.4s
        smax            \r3,    \r3,    v0.4s
        st1             {v2.4s}, [x6], #16
        st1             {\r2},   [x10], x9
        st1             {v3.4s}, [x6], #16
        st1             {\r3},   [x10], x9
.endm
        store_addsub    v31.4s, v30.4s, v29.4s, v28.4s
        store_addsub    v27.4s, v26.4s, v25.4s, v24.4s
        store_addsub    v23.4s, v22.4s, v21.4s, v20.4s
        store_addsub    v19.4s, v18.4s, v17.4s, v16.4s
.purgem store_addsub

        add             x6,  x6,  #4*4*16

        movrel          x17, idct64_coeffs
        movi            v5.4s,  #1, msl #16  // row_clip_max = ~(~bdmax << 7), 0x1ffff
        mvni            v4.4s,  #1, msl #16  // row_clip_min = (~bdmax << 7), 0xfffe0000
        movz16dup_if    v0.2s, w16, #2896*8, \scale
        movi_if         v7.4s,  #0, \clear
        add             x9,  x7,  x8, lsl #4 // offset 16
        add             x10, x7,  x8, lsl #3 // offset 8
        sub             x9,  x9,  x8         // offset 15
        sub             x11, x10, x8         // offset 7
        ld1             {v16.4s}, [x7]  // in1  (offset 0)
        ld1             {v17.4s}, [x9]  // in31 (offset 15)
        ld1             {v18.4s}, [x10] // in17 (offset 8)
        ld1             {v19.4s}, [x11] // in15 (offset 7)
        st1_if          {v7.4s}, [x7],  \clear
        st1_if          {v7.4s}, [x9],  \clear
        st1_if          {v7.4s}, [x10], \clear
        st1_if          {v7.4s}, [x11], \clear
        scale_if        \scale, v0.s[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon
        movz16dup_if    v0.2s, w16, #2896*8, \scale
        movi_if         v7.4s,  #0, \clear
        add             x7,  x7,  x8, lsl #2 // offset 4
        sub             x9,  x9,  x8, lsl #2 // offset 11
        sub             x10, x7,  x8         // offset 3
        add             x11, x9,  x8         // offset 12
        ld1             {v16.4s}, [x10] // in7  (offset 3)
        ld1             {v17.4s}, [x11] // in25 (offset 12)
        ld1             {v18.4s}, [x9]  // in23 (offset 11)
        ld1             {v19.4s}, [x7]  // in9  (offset 4)
        st1_if          {v7.4s}, [x7],  \clear
        st1_if          {v7.4s}, [x9],  \clear
        st1_if          {v7.4s}, [x10], \clear
        st1_if          {v7.4s}, [x11], \clear
        scale_if        \scale, v0.s[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon
        movz16dup_if    v0.2s, w16, #2896*8, \scale
        movi_if         v7.4s,  #0, \clear
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
        scale_if        \scale, v0.s[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon
        movz16dup_if    v0.2s, w16, #2896*8, \scale
        movi_if         v7.4s,  #0, \clear
        ldr             q16, [x10]     // in3  (offset 1)
        ldr             q17, [x11, x8] // in29 (offset 14)
        ldr             q18, [x9]      // in19 (offset 9)
        ldr             q19, [x7,  x8] // in13 (offset 6)
        str_if          q7,  [x10],     \clear
        stroff_if       q7,  [x11, x8], \clear
        str_if          q7,  [x9],      \clear
        stroff_if       q7,  [x7,  x8], \clear
        scale_if        \scale, v0.s[0], v16, v17, v18, v19
        bl              inv_dct64_step1_neon

        sub             x6,  x6,  #4*4*32
        add             x9,  x6,  #4*4*7

        bl              inv_dct64_step2_neon

        ret             x14
endfunc
.endm

def_dct64_func _clear, clear=1
def_dct64_func _clear_scale, clear=1, scale=1


function inv_txfm_horz_dct_64x4_neon
        mov             x14, x30

        mov             x7,  sp
        add             x8,  sp,  #4*4*(64 - 4)
        add             x9,  x6,  #2*56
        mov             x10, #2*64
        mov             x11, #-4*4*4

        dup             v7.4s,  w12
1:
        ld1             {v16.4s, v17.4s, v18.4s, v19.4s}, [x7], #64
        ld1             {v28.4s, v29.4s, v30.4s, v31.4s}, [x8], x11
        ld1             {v20.4s, v21.4s, v22.4s, v23.4s}, [x7], #64
        ld1             {v24.4s, v25.4s, v26.4s, v27.4s}, [x8], x11
        transpose_4x4s  v16, v17, v18, v19, v2,  v3,  v4,  v5
        transpose_4x4s  v20, v21, v22, v23, v2,  v3,  v4,  v5
        transpose_4x4s  v31, v30, v29, v28, v2,  v3,  v4,  v5
        transpose_4x4s  v27, v26, v25, v24, v2,  v3,  v4,  v5

.macro store_addsub src0, src1, src2, src3
        sqsub           v1.4s,   \src0,   \src1
        sqadd           v0.4s,   \src0,   \src1
        sqsub           v3.4s,   \src2,   \src3
        srshl           v1.4s,   v1.4s,   v7.4s
        sqadd           v2.4s,   \src2,   \src3
        srshl           v3.4s,   v3.4s,   v7.4s
        srshl           v0.4s,   v0.4s,   v7.4s
        srshl           v2.4s,   v2.4s,   v7.4s
        sqxtn           v3.4h,   v3.4s
        sqxtn2          v3.8h,   v1.4s
        sqxtn           v0.4h,   v0.4s
        sqxtn2          v0.8h,   v2.4s
        rev64           v3.8h,   v3.8h
        st1             {v0.8h},  [x6], x10
        st1             {v3.8h},  [x9], x10
.endm
        store_addsub    v16.4s,  v31.4s,  v20.4s,  v27.4s
        store_addsub    v17.4s,  v30.4s,  v21.4s,  v26.4s
        store_addsub    v18.4s,  v29.4s,  v22.4s,  v25.4s
        store_addsub    v19.4s,  v28.4s,  v23.4s,  v24.4s
.purgem store_addsub
        sub             x6,  x6,  x10, lsl #2
        sub             x9,  x9,  x10, lsl #2
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

        mvni            v7.8h,   #0xfc, lsl #8 // 0x3ff
.macro add_dest_addsub src0, src1, src2, src3
        ld1             {v0.8h}, [x6], x1
        ld1             {v1.8h}, [x9], x10
        sqadd           v4.8h,   \src0,   \src1
        ld1             {v2.8h}, [x6]
        sqsub           \src0,   \src0,   \src1
        ld1             {v3.8h}, [x9]
        sqadd           v5.8h,   \src2,   \src3
        sqsub           \src2,   \src2,   \src3
        sub             x6,  x6,  x1
        sub             x9,  x9,  x10
        srshr           v4.8h,   v4.8h,   #4
        srshr           v5.8h,   v5.8h,   #4
        srshr           \src0,   \src0,   #4
        usqadd          v0.8h,   v4.8h
        srshr           \src2,   \src2,   #4
        usqadd          v1.8h,   \src0
        usqadd          v2.8h,   v5.8h
        smin            v0.8h,   v0.8h,   v7.8h
        usqadd          v3.8h,   \src2
        smin            v1.8h,   v1.8h,   v7.8h
        st1             {v0.8h}, [x6], x1
        smin            v2.8h,   v2.8h,   v7.8h
        st1             {v1.8h}, [x9], x10
        smin            v3.8h,   v3.8h,   v7.8h
        st1             {v2.8h}, [x6], x1
        st1             {v3.8h}, [x9], x10
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

function inv_txfm_add_dct_dct_64x64_16bpc_neon, export=1
        idct_dc         64,  64,  2

        mov             x15, x30

        sub_sp          64*32*2+64*4*4
        add             x5,  sp, #64*4*4

        movrel          x13, eob_32x32

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             x6,  x5,  #(\i*64*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.endif
        add             x7,  x2,  #(\i*4)
        mov             x8,  #32*4
        mov             x12, #-2 // shift
        bl              inv_txfm_dct_clear_4s_x64_neon
        add             x6,  x5,  #(\i*64*2)
        bl              inv_txfm_horz_dct_64x4_neon
.if \i < 28
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
        bl              X(inv_txfm_dct_8h_x64_neon)
        add             x6,  x0,  #(\i*2)
        bl              inv_txfm_add_vert_dct_8x64_neon
.endr

        add             sp,  x5,  #64*32*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_64x32_16bpc_neon, export=1
        idct_dc         64,  32,  1

        mov             x15, x30

        sub_sp          64*32*2+64*4*4
        add             x5,  sp, #64*4*4

        movrel          x13, eob_32x32

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             x6,  x5,  #(\i*64*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.endif
        add             x7,  x2,  #(\i*4)
        mov             x8,  #32*4
        mov             x12, #-1 // shift
        bl              inv_txfm_dct_clear_scale_4s_x64_neon
        add             x6,  x5,  #(\i*64*2)
        bl              inv_txfm_horz_dct_64x4_neon
.if \i < 28
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
        add             x6,  x0,  #(\i*2)
        add             x7,  x5,  #(\i*2)
        mov             x8,  #64*2
        bl              inv_txfm_add_vert_dct_8x32_neon
.endr

        add             sp,  x5,  #64*32*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_32x64_16bpc_neon, export=1
        idct_dc         32,  64,  1

        mov             x15, x30

        sub_sp          32*32*2+64*8*2
        add             x5,  sp, #64*8*2

        movrel          x13, eob_32x32
        ldrh            w12, [x13], #2

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             x6,  x5,  #(\i*32*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
        ldrh            w12, [x13], #2
.endif
        add             x7,  x2,  #(\i*4)
        mov             x8,  #32*4
        bl              inv_txfm_horz_scale_dct_32x4_neon
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
        bl              X(inv_txfm_dct_8h_x64_neon)
        add             x6,  x0,  #(\i*2)
        bl              inv_txfm_add_vert_dct_8x64_neon
.endr

        add             sp,  x5,  #32*32*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_64x16_16bpc_neon, export=1
        idct_dc         64,  16,  2

        mov             x15, x30

        sub_sp          64*16*2+64*4*4
        add             x4,  sp, #64*4*4

        movrel          x13, eob_16x32

.irp i, 0, 4, 8, 12
        add             x6,  x4,  #(\i*64*2)
.if \i > 0
        mov             w8,  #(16 - \i)
        cmp             w3,  w12
        b.lt            1f
.endif
        add             x7,  x2,  #(\i*4)
        mov             x8,  #16*4
        mov             x12, #-2 // shift
        bl              inv_txfm_dct_clear_4s_x64_neon
        add             x6,  x4,  #(\i*64*2)
        bl              inv_txfm_horz_dct_64x4_neon
.if \i < 12
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
        movrel          x5,  X(inv_dct_8h_x16_neon)
.irp i, 0, 8, 16, 24, 32, 40, 48, 56
        add             x6,  x0,  #(\i*2)
        add             x7,  x4,  #(\i*2)
        mov             x8,  #64*2
        bl              inv_txfm_add_vert_8x16_neon
.endr

        add             sp,  x4,  #64*16*2
        ret             x15
endfunc

function inv_txfm_add_dct_dct_16x64_16bpc_neon, export=1
        idct_dc         16,  64,  2

        mov             x15, x30

        sub_sp          16*32*2+64*8*2
        add             x5,  sp, #64*8*2

        movrel          x13, eob_16x32
        ldrh            w12, [x13], #2

        adr             x4,  inv_dct_4s_x16_neon
.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             x6,  x5,  #(\i*16*2)
.if \i > 0
        mov             w8,  #(32 - \i)
        cmp             w3,  w12
        b.lt            1f
.if \i < 28
        ldrh            w12, [x13], #2
.endif
.endif
        add             x7,  x2,  #(\i*4)
        mov             x8,  #32*4
        bl              inv_txfm_horz_16x4_neon
.endr
        b               3f

1:
        movi            v4.8h,  #0
        movi            v5.8h,  #0
        movi            v6.8h,  #0
        movi            v7.8h,  #0
2:
        subs            w8,  w8,  #4
.rept 2
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64
.endr
        b.gt            2b

3:
.irp i, 0, 8
        add             x7,  x5,  #(\i*2)
        mov             x8,  #16*2
        bl              X(inv_txfm_dct_8h_x64_neon)
        add             x6,  x0,  #(\i*2)
        bl              inv_txfm_add_vert_dct_8x64_neon
.endr

        add             sp,  x5,  #16*32*2
        ret             x15
endfunc
