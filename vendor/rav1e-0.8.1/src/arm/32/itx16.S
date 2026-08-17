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
// void itxfm_add(pixel *dst, ptrdiff_t dst_stride, coef *coeff, int eob);

// Most of the functions use the following register layout:
// r0-r3   external parameters
// r4      function pointer to first transform
// r5      function pointer to second transform
// r6      output parameter for helper function
// r7      input parameter for helper function
// r8      input stride for helper function
// r9      scratch variable for helper functions
// r10-r11 pointer to list of eob thresholds, eob threshold value,
//         scratch variables within helper functions (backed up)

// The SIMD registers most often use the following layout:
// d0-d3   multiplication coefficients
// d4-d7   scratch registers
// d8-d15  unused in some transforms, used for scratch registers in others
// d16-v31 inputs/outputs of transforms

// Potential further optimizations, that are left unimplemented for now:
// - Trying to keep multiplication coefficients in registers across multiple
//   transform functions. (The register layout is designed to potentially
//   allow this.)
// - Use a simplified version of the transforms themselves for cases where
//   we know a significant number of inputs are zero. E.g. if the eob value
//   indicates only a quarter of input values are set, for idct16 and up,
//   a significant amount of calculation can be skipped, at the cost of more
//   code duplication and special casing.

// A macro for cases where a thumb mov can express the constant in one
// instruction, while arm mode requires two separate movw+movt pairs.
.macro mov_const reg, val
#if CONFIG_THUMB
        mov.w           \reg, #\val
#else
        movw            \reg, #((\val) & 0xffff)
        movt            \reg, #(((\val) >> 16) & 0xffff)
#endif
.endm

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

.macro vmul_vmla d0, s0, s1, c0, c1
        vmul.i32        \d0, \s0, \c0
        vmla.i32        \d0, \s1, \c1
.endm

.macro vmul_vmls d0, s0, s1, c0, c1
        vmul.i32        \d0, \s0, \c0
        vmls.i32        \d0, \s1, \c1
.endm

.macro scale_input c, r0, r1, r2 r3, r4, r5, r6, r7
        vqrdmulh.s32    \r0, \r0, \c
        vqrdmulh.s32    \r1, \r1, \c
.ifnb \r2
        vqrdmulh.s32    \r2, \r2, \c
        vqrdmulh.s32    \r3, \r3, \c
.endif
.ifnb \r4
        vqrdmulh.s32    \r4, \r4, \c
        vqrdmulh.s32    \r5, \r5, \c
        vqrdmulh.s32    \r6, \r6, \c
        vqrdmulh.s32    \r7, \r7, \c
.endif
.endm

.macro load_add_store load, shift, addsrc, adddst, max, min, store, dst, src, shiftbits=4
.ifnb \load
        vld1.16         {\load},  [\src, :128], r1
.endif
.ifnb \shift
        vrshr.s16       \shift,  \shift,  #\shiftbits
.endif
.ifnb \addsrc
        vqadd.s16       \adddst, \adddst, \addsrc
.endif
.ifnb \max
        vmax.s16        \max, \max, q6
.endif
.ifnb \min
        vmin.s16        \min, \min, q7
.endif
.ifnb \store
        vst1.16         {\store},  [\dst, :128], r1
.endif
.endm
.macro load_add_store_8x8 dst, src, shiftbits=4
        mov             \src, \dst
        vmov.i16        q6,  #0
        vmvn.i16        q7,  #0xfc00 // 0x3ff
        load_add_store  q0,  q8,    ,    ,    ,    ,    ,  \dst, \src, \shiftbits
        load_add_store  q1,  q9,    ,    ,    ,    ,    ,  \dst, \src, \shiftbits
        load_add_store  q2,  q10, q0,  q8,    ,    ,    ,  \dst, \src, \shiftbits
        load_add_store  q3,  q11, q1,  q9,  q8,    ,    ,  \dst, \src, \shiftbits
        load_add_store  q4,  q12, q2,  q10, q9,  q8,    ,  \dst, \src, \shiftbits
        load_add_store  q5,  q13, q3,  q11, q10, q9,  q8,  \dst, \src, \shiftbits
        load_add_store  q0,  q14, q4,  q12, q11, q10, q9,  \dst, \src, \shiftbits
        load_add_store  q1,  q15, q5,  q13, q12, q11, q10, \dst, \src, \shiftbits
        load_add_store    ,     , q0,  q14, q13, q12, q11, \dst, \src, \shiftbits
        load_add_store    ,     , q1,  q15, q14, q13, q12, \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     , q15, q14, q13, \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     ,    , q15, q14, \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     ,    ,    , q15, \dst, \src, \shiftbits
.endm
.macro load_add_store_8x4 dst, src, shiftbits=4
        mov             \src, \dst
        vmov.i16        q6,  #0
        vmvn.i16        q7,  #0xfc00 // 0x3ff
        load_add_store  q0,  q8,    ,    ,    ,    ,    ,  \dst, \src, \shiftbits
        load_add_store  q1,  q9,    ,    ,    ,    ,    ,  \dst, \src, \shiftbits
        load_add_store  q2,  q10, q0,  q8,    ,    ,    ,  \dst, \src, \shiftbits
        load_add_store  q3,  q11, q1,  q9,  q8,    ,    ,  \dst, \src, \shiftbits
        load_add_store    ,     , q2,  q10, q9,  q8,    ,  \dst, \src, \shiftbits
        load_add_store    ,     , q3,  q11, q10, q9,  q8,  \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     , q11, q10, q9,  \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     ,    , q11, q10, \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     ,    ,    , q11, \dst, \src, \shiftbits
.endm
.macro load_add_store4 load1, load2, shift, addsrc, adddst, max, min, store1, store2, dst, src, shiftbits=4
.ifnb \load1
        vld1.16         {\load1},  [\src, :64], r1
.endif
.ifnb \shift
        vrshr.s16       \shift,  \shift,  #\shiftbits
.endif
.ifnb \load2
        vld1.16         {\load2},  [\src, :64], r1
.endif
.ifnb \addsrc
        vqadd.s16       \adddst, \adddst, \addsrc
.endif
.ifnb \max
        vmax.s16        \max, \max, q6
.endif
.ifnb \store1
        vst1.16         {\store1},  [\dst, :64], r1
.endif
.ifnb \min
        vmin.s16        \min, \min, q7
.endif
.ifnb \store2
        vst1.16         {\store2},  [\dst, :64], r1
.endif
.endm
.macro load_add_store_4x16 dst, src
        mov             \src, \dst
        vmov.i16        q6,  #0
        vmvn.i16        q7,  #0xfc00 // 0x3ff
        mov             \src, \dst
        load_add_store4 d0,  d1,  q8,    ,    ,    ,    ,     ,    , \dst, \src
        load_add_store4 d2,  d3,  q9,    ,    ,    ,    ,     ,    , \dst, \src
        load_add_store4 d4,  d5,  q10, q0,  q8,    ,    ,     ,    , \dst, \src
        load_add_store4 d6,  d7,  q11, q1,  q9,  q8,    ,     ,    , \dst, \src
        load_add_store4 d8,  d9,  q12, q2,  q10, q9,  q8,     ,    , \dst, \src
        load_add_store4 d10, d11, q13, q3,  q11, q10, q9,  d16, d17, \dst, \src
        load_add_store4 d0,  d1,  q14, q4,  q12, q11, q10, d18, d19, \dst, \src
        load_add_store4 d2,  d3,  q15, q5,  q13, q12, q11, d20, d21, \dst, \src
        load_add_store4   ,    ,    ,  q0,  q14, q13, q12, d22, d23, \dst, \src
        load_add_store4   ,    ,    ,  q1,  q15, q14, q13, d24, d25, \dst, \src
        load_add_store4   ,    ,    ,    ,   ,   q15, q14, d26, d27, \dst, \src
        load_add_store4   ,    ,    ,    ,   ,     ,  q15, d28, d29, \dst, \src
        load_add_store4   ,    ,    ,    ,   ,     ,     , d30, d31, \dst, \src
.endm
.macro load_add_store_4x8 dst, src, shiftbits=4
        mov             \src, \dst
        vmov.i16        q6,  #0
        vmvn.i16        q7,  #0xfc00 // 0x3ff
        mov             \src, \dst
        load_add_store4 d0,  d1,  q8,    ,    ,    ,    ,     ,    , \dst, \src, \shiftbits
        load_add_store4 d2,  d3,  q9,    ,    ,    ,    ,     ,    , \dst, \src, \shiftbits
        load_add_store4 d4,  d5,  q10, q0,  q8,    ,    ,     ,    , \dst, \src, \shiftbits
        load_add_store4 d6,  d7,  q11, q1,  q9,  q8,    ,     ,    , \dst, \src, \shiftbits
        load_add_store4   ,    ,     , q2,  q10, q9,  q8,     ,    , \dst, \src, \shiftbits
        load_add_store4   ,    ,     , q3,  q11, q10, q9,  d16, d17, \dst, \src, \shiftbits
        load_add_store4   ,    ,     ,   ,     , q11, q10, d18, d19, \dst, \src, \shiftbits
        load_add_store4   ,    ,     ,   ,     ,    , q11, d20, d21, \dst, \src, \shiftbits
        load_add_store4   ,    ,     ,   ,     ,    ,    , d22, d23, \dst, \src, \shiftbits
.endm
.macro load_add_store_4x4 dst, src, shiftbits=4
        mov             \src, \dst
        vmov.i16        q6,  #0
        vmvn.i16        q7,  #0xfc00 // 0x3ff
        mov             \src, \dst
        load_add_store4 d0,  d1,  q8,   ,   ,   ,   ,    ,    , \dst, \src, \shiftbits
        load_add_store4 d2,  d3,  q9, q0, q8,   ,   ,    ,    , \dst, \src, \shiftbits
        load_add_store4   ,    ,    , q1, q9, q8,   ,    ,    , \dst, \src, \shiftbits
        load_add_store4   ,    ,    ,   ,   , q9, q8,    ,    , \dst, \src, \shiftbits
        load_add_store4   ,    ,    ,   ,   ,   , q9, d16, d17, \dst, \src, \shiftbits
        load_add_store4   ,    ,    ,   ,   ,   ,   , d18, d19, \dst, \src, \shiftbits
.endm

.macro idct_dc w, h, shift
        cmp             r3,  #0
        bne             1f
        vmov.i16        q14, #0
        mov_const       r12, 2896*8*(1<<16)
        vld1.32         {d24[], d25[]}, [r2, :32]
        vdup.32         d0,  r12
        vqrdmulh.s32    q13, q12, d0[0]
        vst1.32         {d28[0]}, [r2, :32]
.if (\w == 2*\h) || (2*\w == \h)
        vqrdmulh.s32    q13, q13, d0[0]
.endif
.if \shift > 0
        vqrshrn.s32     d24, q13, #\shift
        vqrshrn.s32     d25, q13, #\shift
.else
        vqmovn.s32      d24, q13
        vqmovn.s32      d25, q13
.endif
        vqrdmulh.s16    q12, q12, d0[1]
        mov             r3,  #\h
        vrshr.s16       q12, q12, #4
        b               idct_dc_w\w\()_neon
1:
.endm

function idct_dc_w4_neon
        vmvn.i16        q15, #0xfc00 // 0x3ff
1:
        vld1.16         {d0}, [r0, :64], r1
        vld1.16         {d1}, [r0, :64], r1
        vld1.16         {d2}, [r0, :64], r1
        vld1.16         {d3}, [r0, :64], r1
        subs            r3,  r3,  #4
        vqadd.s16       q0,  q0,  q12
        sub             r0,  r0,  r1, lsl #2
        vqadd.s16       q1,  q1,  q12
        vmax.s16        q0,  q0,  q14
        vmax.s16        q1,  q1,  q14
        vmin.s16        q0,  q0,  q15
        vst1.16         {d0}, [r0, :64], r1
        vmin.s16        q1,  q1,  q15
        vst1.16         {d1}, [r0, :64], r1
        vst1.16         {d2}, [r0, :64], r1
        vst1.16         {d3}, [r0, :64], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w8_neon
        vmvn.i16        q15, #0xfc00 // 0x3ff
1:
        vld1.16         {q0}, [r0, :128], r1
        subs            r3,  r3,  #4
        vld1.16         {q1}, [r0, :128], r1
        vqadd.s16       q0,  q0,  q12
        vld1.16         {q2}, [r0, :128], r1
        vqadd.s16       q1,  q1,  q12
        vld1.16         {q3}, [r0, :128], r1
        vqadd.s16       q2,  q2,  q12
        vqadd.s16       q3,  q3,  q12
        sub             r0,  r0,  r1, lsl #2
        vmax.s16        q0,  q0,  q14
        vmax.s16        q1,  q1,  q14
        vmax.s16        q2,  q2,  q14
        vmax.s16        q3,  q3,  q14
        vmin.s16        q0,  q0,  q15
        vmin.s16        q1,  q1,  q15
        vst1.16         {q0}, [r0, :128], r1
        vmin.s16        q2,  q2,  q15
        vst1.16         {q1}, [r0, :128], r1
        vmin.s16        q3,  q3,  q15
        vst1.16         {q2}, [r0, :128], r1
        vst1.16         {q3}, [r0, :128], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w16_neon
        vmvn.i16        q15, #0xfc00 // 0x3ff
1:
        vld1.16         {q0, q1}, [r0, :128], r1
        subs            r3,  r3,  #2
        vld1.16         {q2, q3}, [r0, :128], r1
        vqadd.s16       q0,  q0,  q12
        vqadd.s16       q1,  q1,  q12
        vqadd.s16       q2,  q2,  q12
        vqadd.s16       q3,  q3,  q12
        sub             r0,  r0,  r1, lsl #1
        vmax.s16        q0,  q0,  q14
        vmax.s16        q1,  q1,  q14
        vmax.s16        q2,  q2,  q14
        vmax.s16        q3,  q3,  q14
        vmin.s16        q0,  q0,  q15
        vmin.s16        q1,  q1,  q15
        vmin.s16        q2,  q2,  q15
        vst1.16         {q0, q1}, [r0, :128], r1
        vmin.s16        q3,  q3,  q15
        vst1.16         {q2, q3}, [r0, :128], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w32_neon
        sub             r1,  r1,  #32
        vmvn.i16        q15, #0xfc00 // 0x3ff
1:
        vld1.16         {q0, q1}, [r0, :128]!
        subs            r3,  r3,  #1
        vld1.16         {q2, q3}, [r0, :128]
        vqadd.s16       q0,  q0,  q12
        vqadd.s16       q1,  q1,  q12
        vqadd.s16       q2,  q2,  q12
        vqadd.s16       q3,  q3,  q12
        sub             r0,  r0,  #32
        vmax.s16        q0,  q0,  q14
        vmax.s16        q1,  q1,  q14
        vmax.s16        q2,  q2,  q14
        vmax.s16        q3,  q3,  q14
        vmin.s16        q0,  q0,  q15
        vmin.s16        q1,  q1,  q15
        vmin.s16        q2,  q2,  q15
        vst1.16         {q0, q1}, [r0, :128]!
        vmin.s16        q3,  q3,  q15
        vst1.16         {q2, q3}, [r0, :128], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w64_neon
        sub             r1,  r1,  #96
        vmvn.i16        q15, #0xfc00 // 0x3ff
1:
        vld1.16         {q0,  q1},  [r0, :128]!
        subs            r3,  r3,  #1
        vld1.16         {q2,  q3},  [r0, :128]!
        vqadd.s16       q0,  q0,  q12
        vld1.16         {q8,  q9},  [r0, :128]!
        vqadd.s16       q1,  q1,  q12
        vld1.16         {q10, q11}, [r0, :128]
        vqadd.s16       q2,  q2,  q12
        vqadd.s16       q3,  q3,  q12
        vqadd.s16       q8,  q8,  q12
        vqadd.s16       q9,  q9,  q12
        vqadd.s16       q10, q10, q12
        vqadd.s16       q11, q11, q12
        sub             r0,  r0,  #96
        vmax.s16        q0,  q0,  q14
        vmax.s16        q1,  q1,  q14
        vmax.s16        q2,  q2,  q14
        vmax.s16        q3,  q3,  q14
        vmax.s16        q8,  q8,  q14
        vmax.s16        q9,  q9,  q14
        vmax.s16        q10, q10, q14
        vmax.s16        q11, q11, q14
        vmin.s16        q0,  q0,  q15
        vmin.s16        q1,  q1,  q15
        vmin.s16        q2,  q2,  q15
        vmin.s16        q3,  q3,  q15
        vmin.s16        q8,  q8,  q15
        vst1.16         {q0,  q1},  [r0, :128]!
        vmin.s16        q9,  q9,  q15
        vst1.16         {q2,  q3},  [r0, :128]!
        vmin.s16        q10, q10, q15
        vst1.16         {q8,  q9},  [r0, :128]!
        vmin.s16        q11, q11, q15
        vst1.16         {q10, q11}, [r0, :128], r1
        bgt             1b
        bx              lr
endfunc

.macro iwht4
        vadd.i32        q8,  q8,  q9
        vsub.i32        q13, q10, q11
        vsub.i32        q12, q8,  q13
        vshr.s32        q12, q12, #1
        vsub.i32        q10, q12, q9
        vsub.i32        q9,  q12, q11
        vadd.i32        q11, q13, q10
        vsub.i32        q8,  q8,  q9
.endm

.macro idct_4s_x4 r0, r1, r2, r3
        vmul_vmla       q4,  \r1, \r3, d1[1], d1[0]
        vmul_vmla       q2,  \r0, \r2, d0[0], d0[0]
        vmul_vmls       q3,  \r1, \r3, d1[0], d1[1]
        vmul_vmls       q5,  \r0, \r2, d0[0], d0[0]
        vrshr.s32       q4,  q4,  #12
        vrshr.s32       q2,  q2,  #12
        vrshr.s32       q3,  q3,  #12
        vrshr.s32       q5,  q5,  #12
        vqadd.s32       \r0, q2,  q4
        vqsub.s32       \r3, q2,  q4
        vqadd.s32       \r1, q5,  q3
        vqsub.s32       \r2, q5,  q3
.endm

.macro idct_2s_x4 r0, r1, r2, r3
        vmul_vmla       d6,  \r1, \r3, d1[1], d1[0]
        vmul_vmla       d4,  \r0, \r2, d0[0], d0[0]
        vmul_vmls       d5,  \r1, \r3, d1[0], d1[1]
        vmul_vmls       d7,  \r0, \r2, d0[0], d0[0]
        vrshr.s32       d6,  d6,  #12
        vrshr.s32       d4,  d4,  #12
        vrshr.s32       d5,  d5,  #12
        vrshr.s32       d7,  d7,  #12
        vqadd.s32       \r0, d4,  d6
        vqsub.s32       \r3, d4,  d6
        vqadd.s32       \r1, d7,  d5
        vqsub.s32       \r2, d7,  d5
.endm

function inv_dct_4s_x4_neon
        movrel_local    r12, idct_coeffs
        vld1.32         {d0, d1}, [r12, :128]
        idct_4s_x4      q8,  q9,  q10, q11
        bx              lr
endfunc

.macro iadst_4x4 o0, o1, o2, o3
        movrel_local    r12, iadst4_coeffs
        vld1.32         {d0, d1}, [r12, :128]

        vsub.i32        q1,  q8,  q10
        vmul.i32        q2,  q8,  d0[0]
        vmla.i32        q2,  q10, d0[1]
        vmla.i32        q2,  q11, d1[0]
        vmul.i32        q4,  q9,  d1[1]
        vadd.i32        q1,  q1,  q11
        vmul.i32        q3,  q8,  d1[0]
        vmls.i32        q3,  q10, d0[0]
        vmls.i32        q3,  q11, d0[1]

        vadd.i32        \o3, q2,  q3
        vmul.i32        \o2, q1,  d1[1]
        vadd.i32        \o0, q2,  q4
        vadd.i32        \o1, q3,  q4
        vsub.i32        \o3, \o3, q4

        vrshr.s32       \o0, \o0, #12
        vrshr.s32       \o2, \o2, #12
        vrshr.s32       \o1, \o1, #12
        vrshr.s32       \o3, \o3, #12
.endm

function inv_adst_4s_x4_neon
        iadst_4x4       q8,  q9,  q10, q11
        bx              lr
endfunc

function inv_flipadst_4s_x4_neon
        iadst_4x4       q11, q10, q9,  q8
        bx              lr
endfunc

function inv_identity_4s_x4_neon
        mov             r12, #0
        movt            r12, #(5793-4096)*8
        vdup.32         d0,  r12
        vqrdmulh.s32    q1,  q8,  d0[0]
        vqrdmulh.s32    q2,  q9,  d0[0]
        vqrdmulh.s32    q3,  q10, d0[0]
        vqrdmulh.s32    q4,  q11, d0[0]
        vqadd.s32       q8,  q8,  q1
        vqadd.s32       q9,  q9,  q2
        vqadd.s32       q10, q10, q3
        vqadd.s32       q11, q11, q4
        bx              lr
endfunc

function inv_txfm_add_wht_wht_4x4_16bpc_neon, export=1
        push            {r4-r5,lr}
        vpush           {q4-q5}
        vmov.i16        q14, #0
        vmov.i16        q15, #0
        vld1.32         {q8,  q9},  [r2, :128]
        vst1.32         {q14, q15}, [r2, :128]!
        vshr.s16        q8,  q8,  #2
        vld1.32         {q10, q11}, [r2, :128]
        vshr.s16        q9,  q9,  #2
        vshr.s16        q10, q10, #2
        vshr.s16        q11, q11, #2

        iwht4

        vst1.32         {q14, q15}, [r2, :128]
        transpose_4x4s  q8,  q9,  q10, q11, d16, d17, d18, d19, d20, d21, d22, d23

        iwht4

        vld1.16         {d0}, [r0, :64], r1
        vqmovn.s32      d16, q8
        vld1.16         {d1}, [r0, :64], r1
        vqmovn.s32      d17, q9
        vld1.16         {d2}, [r0, :64], r1
        vqmovn.s32      d18, q10
        vld1.16         {d3}, [r0, :64], r1
        vqmovn.s32      d19, q11

        b               L(itx_4x4_end)
endfunc

function inv_txfm_add_4x4_neon
        vmov.i16        q14, #0
        vmov.i16        q15, #0
        vld1.32         {q8,  q9},  [r2, :128]
        vst1.16         {q14, q15}, [r2, :128]!
        vld1.32         {q10, q11}, [r2, :128]
        vst1.16         {q14, q15}, [r2, :128]

        blx             r4

        vqmovn.s32      d16, q8
        vqmovn.s32      d17, q9
        vqmovn.s32      d18, q10
        vqmovn.s32      d19, q11
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19

        blx             r5

        vld1.16         {d0}, [r0, :64], r1
        vld1.16         {d1}, [r0, :64], r1
        vrshr.s16       q8,  q8,  #4
        vld1.16         {d2}, [r0, :64], r1
        vrshr.s16       q9,  q9,  #4
        vld1.16         {d3}, [r0, :64], r1

L(itx_4x4_end):
        vmvn.i16        q15, #0xfc00 // 0x3ff
        sub             r0,  r0,  r1, lsl #2
        vqadd.s16       q8,  q8,  q0
        vqadd.s16       q9,  q9,  q1
        vmax.s16        q8,  q8,  q14
        vmax.s16        q9,  q9,  q14
        vmin.s16        q8,  q8,  q15
        vmin.s16        q9,  q9,  q15
        vst1.16         {d16}, [r0, :64], r1
        vst1.16         {d17}, [r0, :64], r1
        vst1.16         {d18}, [r0, :64], r1
        vst1.16         {d19}, [r0, :64], r1

        vpop            {q4-q5}
        pop             {r4-r5,pc}
endfunc

.macro def_fn_4x4 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_4x4_16bpc_neon, export=1
        push            {r4-r5,lr}
        vpush           {q4-q5}

.ifc \txfm1\()_\txfm2, dct_dct
        cmp             r3,  #0
        bne             1f
        vmov.i16        q14, #0
        mov_const       r12, 2896*8*(1<<16)
        vld1.32         {d16[], d17[]},  [r2, :32]
        vdup.32         d4,  r12
        vst1.32         {d28[0]}, [r2, :32]
        vqrdmulh.s32    q8,  q8,  d4[0]
        vld1.16         {d0}, [r0, :64], r1
        vqmovn.s32      d20, q8
        vqmovn.s32      d21, q8
        vld1.16         {d1}, [r0, :64], r1
        vqrdmulh.s16    q10, q10, d4[1]
        vld1.16         {d2}, [r0, :64], r1
        vrshr.s16       q8,  q10, #4
        vld1.16         {d3}, [r0, :64], r1
        vrshr.s16       q9,  q10, #4
        b               L(itx_4x4_end)
1:
.endif
        movrel_local    r4,  inv_\txfm1\()_4s_x4_neon
        movrel          r5,  X(inv_\txfm2\()_4h_x4_neon)
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

.macro idct_4s_x8 r0, r1, r2, r3, r4, r5, r6, r7
        idct_4s_x4      \r0, \r2, \r4, \r6

        vmov.i32        q5,  #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        q4,  #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000
.irp r, \r0, \r2, \r4, \r6
        vmin.s32        \r,  \r,  q5
.endr
.irp r, \r0, \r2, \r4, \r6
        vmax.s32        \r,  \r,  q4
.endr

        vmul_vmls       q2,  \r1, \r7, d2[0], d2[1] // -> t4a
        vmul_vmla       q3,  \r1, \r7, d2[1], d2[0] // -> t7a
        vmul_vmls       q6,  \r5, \r3, d3[0], d3[1] // -> t5a
        vmul_vmla       q7,  \r5, \r3, d3[1], d3[0] // -> t6a
        vrshr.s32       \r1, q2,  #12               // t4a
        vrshr.s32       \r7, q3,  #12               // t7a
        vrshr.s32       \r3, q6,  #12               // t5a
        vrshr.s32       \r5, q7,  #12               // t6a

        vqadd.s32       q2,  \r1, \r3               // t4
        vqsub.s32       \r1, \r1, \r3               // t5a
        vqadd.s32       q3,  \r7, \r5               // t7
        vqsub.s32       \r3, \r7, \r5               // t6a

.irp r, q2, \r1, q3, \r3
        vmin.s32        \r,  \r,  q5
.endr
.irp r, q2, \r1, q3, \r3
        vmax.s32        \r,  \r,  q4
.endr

        vmul_vmls       q7,  \r3, \r1, d0[0], d0[0] // -> t5
        vmul_vmla       q6,  \r3, \r1, d0[0], d0[0] // -> t6
        vrshr.s32       q7,  q7,  #12               // t5
        vrshr.s32       q5,  q6,  #12               // t6

        vqsub.s32       \r7, \r0, q3  // out7
        vqadd.s32       \r0, \r0, q3  // out0
        vqadd.s32       \r1, \r2, q5  // out1
        vqsub.s32       q6,  \r2, q5  // out6
        vqadd.s32       \r2, \r4, q7  // out2
        vqsub.s32       \r5, \r4, q7  // out5
        vqadd.s32       \r3, \r6, q2  // out3
        vqsub.s32       \r4, \r6, q2  // out4
        vmov            \r6, q6       // out6
.endm

.macro idct_2s_x8 r0, r1, r2, r3, r4, r5, r6, r7
        idct_2s_x4      \r0, \r2, \r4, \r6

        vmov.i32        d9,  #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        d8,  #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000
.irp r, \r0, \r2, \r4, \r6
        vmin.s32        \r,  \r,  d9
.endr
.irp r, \r0, \r2, \r4, \r6
        vmax.s32        \r,  \r,  d8
.endr

        vmul_vmls       d4,  \r1, \r7, d2[0], d2[1] // -> t4a
        vmul_vmla       d5,  \r1, \r7, d2[1], d2[0] // -> t7a
        vmul_vmls       d6,  \r5, \r3, d3[0], d3[1] // -> t5a
        vmul_vmla       d7,  \r5, \r3, d3[1], d3[0] // -> t6a
        vrshr.s32       \r1, d4,  #12               // t4a
        vrshr.s32       \r7, d5,  #12               // t7a
        vrshr.s32       \r3, d6,  #12               // t5a
        vrshr.s32       \r5, d7,  #12               // t6a

        vqadd.s32       d4,  \r1, \r3               // t4
        vqsub.s32       \r1, \r1, \r3               // t5a
        vqadd.s32       d5,  \r7, \r5               // t7
        vqsub.s32       \r3, \r7, \r5               // t6a

.irp r, d4, \r1, d5, \r3
        vmin.s32        \r,  \r,  d9
.endr
.irp r, d4, \r1, d5, \r3
        vmax.s32        \r,  \r,  d8
.endr

        vmul_vmls       d6,  \r3, \r1, d0[0], d0[0] // -> t5
        vmul_vmla       d7,  \r3, \r1, d0[0], d0[0] // -> t6
        vrshr.s32       d6,  d6,  #12               // t5
        vrshr.s32       d7,  d7,  #12               // t6

        vqsub.s32       \r7, \r0, d5  // out7
        vqadd.s32       \r0, \r0, d5  // out0
        vqadd.s32       \r1, \r2, d7  // out1
        vqsub.s32       d7,  \r2, d7  // out6
        vqadd.s32       \r2, \r4, d6  // out2
        vqsub.s32       \r5, \r4, d6  // out5
        vqadd.s32       \r3, \r6, d4  // out3
        vqsub.s32       \r4, \r6, d4  // out4
        vmov            \r6, d7       // out6
.endm

function inv_dct_4s_x8_neon
        movrel_local    r12, idct_coeffs
        vld1.32         {q0, q1}, [r12, :128]
        idct_4s_x8      q8,  q9,  q10, q11, q12, q13, q14, q15
        bx              lr
endfunc

.macro iadst_4s_x8 r0, r1, r2, r3, r4, r5, r6, r7
        movrel_local    r12, iadst8_coeffs
        vld1.32         {q0, q1}, [r12, :128]!

        vmul_vmla       q2,  q15, q8,  d0[0], d0[1]
        vmul_vmls       q3,  q15, q8,  d0[1], d0[0]
        vmul_vmla       q4,  q13, q10, d1[0], d1[1]
        vrshr.s32       q8,  q2,  #12 // t0a
        vrshr.s32       q15, q3,  #12 // t1a
        vmul_vmls       q5,  q13, q10, d1[1], d1[0]
        vmul_vmla       q6,  q11, q12, d2[0], d2[1]
        vrshr.s32       q10, q4,  #12 // t2a
        vrshr.s32       q13, q5,  #12 // t3a
        vmul_vmls       q7,  q11, q12, d2[1], d2[0]
        vmul_vmla       q2,  q9,  q14, d3[0], d3[1]
        vrshr.s32       q12, q6,  #12 // t4a
        vrshr.s32       q11, q7,  #12 // t5a
        vmul_vmls       q3,  q9,  q14, d3[1], d3[0]
        vrshr.s32       q14, q2,  #12 // t6a
        vrshr.s32       q9,  q3,  #12 // t7a

        vld1.32         {q0}, [r12]

        vqadd.s32       q2,  q8,  q12 // t0
        vqsub.s32       q3,  q8,  q12 // t4
        vmov.i32        q12, #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vqadd.s32       q4,  q15, q11 // t1
        vqsub.s32       q5,  q15, q11 // t5
        vqadd.s32       q6,  q10, q14 // t2
        vqsub.s32       q7,  q10, q14 // t6
        vmvn.i32        q14, #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000
        vqadd.s32       q10, q13, q9  // t3
        vqsub.s32       q11, q13, q9  // t7

.irp r, q2, q3, q4, q5, q6, q7, q10, q11
        vmin.s32        \r,  \r,  q12
.endr
.irp r, q2, q3, q4, q5, q6, q7, q10, q11
        vmax.s32        \r,  \r,  q14
.endr

        vmul_vmla       q8,  q3,  q5,  d1[1], d1[0]
        vmul_vmls       q13, q3,  q5,  d1[0], d1[1]
        vmul_vmls       q14, q11, q7,  d1[1], d1[0]

        vrshr.s32       q3,  q8,  #12 // t4a
        vrshr.s32       q5,  q13, #12 // t5a

        vmul_vmla       q8,  q11, q7,  d1[0], d1[1]

        vrshr.s32       q7,  q14, #12 // t6a
        vrshr.s32       q11, q8,  #12 // t7a

        vqadd.s32       \r0, q2,  q6  // out0
        vqsub.s32       q2,  q2,  q6  // t2
        vqadd.s32       \r7, q4,  q10 // out7
        vqsub.s32       q4,  q4,  q10 // t3

        vmvn.i32        q10, #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000

        vqadd.s32       \r1, q3,  q7  // out1
        vqsub.s32       q3,  q3,  q7  // t6
        vqadd.s32       \r6, q5,  q11 // out6
        vqsub.s32       q5,  q5,  q11 // t7

        // Not clipping the output registers, as they will be downshifted and
        // narrowed afterwards anyway.
.irp r, q2, q4, q3, q5
        vmin.s32        \r,  \r,  q12
.endr
.irp r, q2, q4, q3, q5
        vmax.s32        \r,  \r,  q10
.endr

        vqneg.s32       \r7, \r7      // out7
        vqneg.s32       \r1, \r1      // out1

        vmul_vmla       q10, q2,  q4,  d0[0], d0[0] // -> out3 (q11 or q12)
        vmul_vmls       q6,  q2,  q4,  d0[0], d0[0] // -> out4 (q12 or q11)
        vmul_vmls       q12, q3,  q5,  d0[0], d0[0] // -> out5 (q13 or q10)
        vrshr.s32       q2,  q10, #12 // out3
        vmul_vmla       q10, q3,  q5,  d0[0], d0[0] // -> out2 (q10 or q13)
        vrshr.s32       q3,  q12, #12 // out5
        vrshr.s32       \r2, q10, #12 // out2 (q10 or q13)
        vrshr.s32       \r4, q6,  #12 // out4 (q12 or q11)

        vqneg.s32       \r3, q2       // out3
        vqneg.s32       \r5, q3       // out5
.endm

function inv_adst_4s_x8_neon
        iadst_4s_x8     q8,  q9,  q10, q11, q12, q13, q14, q15
        bx              lr
endfunc

function inv_flipadst_4s_x8_neon
        iadst_4s_x8     q15, q14, q13, q12, q11, q10, q9,  q8
        bx              lr
endfunc

function inv_identity_4s_x8_neon
        vqshl.s32       q8,  q8,  #1
        vqshl.s32       q9,  q9,  #1
        vqshl.s32       q10, q10, #1
        vqshl.s32       q11, q11, #1
        vqshl.s32       q12, q12, #1
        vqshl.s32       q13, q13, #1
        vqshl.s32       q14, q14, #1
        vqshl.s32       q15, q15, #1
        bx              lr
endfunc

function inv_txfm_add_8x8_neon
        vmov.i32        q0,  #0
        mov             r7,  #8*4
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.32         {\i},  [r2, :128]
        vst1.32         {q0},  [r2, :128], r7
.endr

        blx             r4

        vqrshrn.s32     d16, q8,  #1
        vqrshrn.s32     d17, q12, #1
        vqrshrn.s32     d18, q9,  #1
        vqrshrn.s32     d19, q13, #1
        vqrshrn.s32     d20, q10, #1
        vqrshrn.s32     d21, q14, #1
        vqrshrn.s32     d22, q11, #1
        vqrshrn.s32     d23, q15, #1

        cmp             r3,  r10
        transpose_4x8h  q8,  q9,  q10, q11

        blt             1f

        sub             r2,  r2,  r7, lsl #3
        vpush           {q8-q11}

        add             r2,  r2,  #16
        vmov.i32        q0,  #0
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.32         {\i},  [r2, :128]
        vst1.32         {q0},  [r2, :128], r7
.endr

        blx             r4

        vqrshrn.s32     d31, q15, #1
        vqrshrn.s32     d30, q11, #1
        vqrshrn.s32     d29, q14, #1
        vqrshrn.s32     d28, q10, #1
        vqrshrn.s32     d27, q13, #1
        vqrshrn.s32     d26, q9,  #1
        vqrshrn.s32     d25, q12, #1
        vqrshrn.s32     d24, q8,  #1
        vpop            {q8-q11}

        transpose_4x8h  q12, q13, q14, q15

        b               2f

1:
        vmov.i16        q12, #0
        vmov.i16        q13, #0
        vmov.i16        q14, #0
        vmov.i16        q15, #0

2:
        blx             r5

        load_add_store_8x8 r0, r7
        vpop            {q4-q7}
        pop             {r4-r5,r7,r10,pc}
endfunc

.macro def_fn_8x8 txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_8x8_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         8,   8,   1
.endif
        push            {r4-r5,r7,r10,lr}
        vpush           {q4-q7}
        mov             r10, #\eob_half
        movrel_local    r4,  inv_\txfm1\()_4s_x8_neon
        movrel          r5,  X(inv_\txfm2\()_8h_x8_neon)
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
        mov_const       r12, 2896*8*(1<<16)
        vmov.i32        q0,  #0
        vmov.i32        q1,  #0
        vld1.16         {q8,  q9},  [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]!
        vdup.32         d4,  r12
        vld1.16         {q10, q11}, [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]!
        vld1.16         {q12, q13}, [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]!
        vld1.16         {q14, q15}, [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]!

        scale_input     d4[0], q8,  q9, q10, q11, q12, q13, q14, q15

        blx             r4

        vqmovn.s32      d16, q8
        vqmovn.s32      d17, q9
        vqmovn.s32      d18, q10
        vqmovn.s32      d19, q11
        vqmovn.s32      d20, q12
        vqmovn.s32      d21, q13
        vqmovn.s32      d22, q14
        vqmovn.s32      d23, q15
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        vswp            d17, d20
        vswp            d19, d21
        vswp            d18, d20
        vswp            d21, d22

        blx             r5

        load_add_store_8x4 r0, r7
        vpop            {q4-q7}
        pop             {r4-r5,r7,r10,pc}
endfunc

function inv_txfm_add_4x8_neon
        mov_const       r12, 2896*8*(1<<16)
        vmov.i32        q0,  #0
        cmp             r3,  r10
        mov             r7,  #32
        blt             1f

        add             r2,  r2,  #16
        vdup.32         d2,  r12
.irp i, q8, q9, q10, q11
        vld1.32         {\i},  [r2, :128]
        vst1.32         {q0},  [r2, :128], r7
.endr

        scale_input     d2[0], q8,  q9, q10, q11
        sub             r2,  r2,  r7, lsl #2

        blx             r4

        sub             r2,  r2,  #16

        vqmovn.s32      d24, q8
        vqmovn.s32      d25, q9
        vqmovn.s32      d26, q10
        vqmovn.s32      d27, q11
        transpose_4x4h  q12, q13, d24, d25, d26, d27

        b               2f

1:
        vmov.i16        q12, #0
        vmov.i16        q13, #0

2:
        mov_const       r12, 2896*8*(1<<16)
        vmov.i32        q0,  #0
        vdup.32         d2,  r12
.irp i, q8, q9, q10, q11
        vld1.32         {\i},  [r2, :128]
        vst1.32         {q0},  [r2, :128], r7
.endr
        scale_input     d2[0], q8,  q9, q10, q11
        blx             r4

        vqmovn.s32      d16, q8
        vqmovn.s32      d17, q9
        vqmovn.s32      d18, q10
        vqmovn.s32      d19, q11
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19

        vmov            q10, q12
        vmov            q11, q13

        blx             r5

        load_add_store_4x8 r0, r7
        vpop            {q4-q7}
        pop             {r4-r5,r7,r10,pc}
endfunc

.macro def_fn_48 w, h, txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  0
.endif
        push            {r4-r5,r7,r10,lr}
        vpush           {q4-q7}
        movrel_local    r4,  inv_\txfm1\()_4s_x\w\()_neon
.if \w == 4
        mov             r10, #\eob_half
.endif
        movrel          r5,  X(inv_\txfm2\()_\w\()h_x\h\()_neon)
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

function inv_dct_2s_x16_neon
        movrel_local    r12, idct_coeffs
        vld1.32         {q0, q1}, [r12, :128]!

        idct_2s_x8      d16, d18, d20, d22, d24, d26, d28, d30

        // idct_8 leaves the row_clip_max/min constants in d9 and d8
.irp r, d16, d18, d20, d22, d24, d26, d28, d30
        vmin.s32        \r,  \r,  d9
.endr
.irp r, d16, d18, d20, d22, d24, d26, d28, d30
        vmax.s32        \r,  \r,  d8
.endr

        vld1.32         {q0, q1}, [r12, :128]
        sub             r12, r12, #32

        vmul_vmls       d4,  d17, d31, d0[0], d0[1]  // -> t8a
        vmul_vmla       d5,  d17, d31, d0[1], d0[0]  // -> t15a
        vmul_vmls       d6,  d25, d23, d1[0], d1[1]  // -> t9a
        vrshr.s32       d17, d4,  #12                // t8a
        vrshr.s32       d31, d5,  #12                // t15a
        vmul_vmla       d4,  d25, d23, d1[1], d1[0]  // -> t14a
        vmul_vmls       d5,  d21, d27, d2[0], d2[1]  // -> t10a
        vrshr.s32       d23, d6,  #12                // t9a
        vrshr.s32       d25, d4,  #12                // t14a
        vmul_vmla       d6,  d21, d27, d2[1], d2[0]  // -> t13a
        vmul_vmls       d4,  d29, d19, d3[0], d3[1]  // -> t11a
        vrshr.s32       d21, d5,  #12                // t10a
        vrshr.s32       d27, d6,  #12                // t13a
        vmul_vmla       d5,  d29, d19, d3[1], d3[0]  // -> t12a
        vrshr.s32       d19, d4,  #12                // t11a
        vrshr.s32       d29, d5,  #12                // t12a

        vld1.32         {q0}, [r12, :128]

        vqsub.s32       d4,  d17, d23  // t9
        vqadd.s32       d17, d17, d23  // t8
        vqsub.s32       d5,  d31, d25  // t14
        vqadd.s32       d31, d31, d25  // t15
        vqsub.s32       d23, d19, d21  // t10
        vqadd.s32       d19, d19, d21  // t11
        vqadd.s32       d25, d29, d27  // t12
        vqsub.s32       d29, d29, d27  // t13

.irp r, d4, d17, d5, d31, d23, d19, d25, d29
        vmin.s32        \r,  \r,  d9
.endr
.irp r, d4, d17, d5, d31, d23, d19, d25, d29
        vmax.s32        \r,  \r,  d8
.endr

        vmul_vmls       d6,  d5,  d4,  d1[0], d1[1]  // -> t9a
        vmul_vmla       d7,  d5,  d4,  d1[1], d1[0]  // -> t14a
        vrshr.s32       d21, d6,  #12                // t9a
        vrshr.s32       d27, d7,  #12                // t14a

        vmul_vmls       d6,  d29, d23, d1[0], d1[1]  // -> t13a
        vmul_vmla       d7,  d29, d23, d1[1], d1[0]  // -> t10a
        vrshr.s32       d29, d6,  #12                // t13a
        vneg.s32        d7,  d7
        vrshr.s32       d23, d7,  #12                // t10a

        vqsub.s32       d4,  d17, d19  // t11a
        vqadd.s32       d17, d17, d19  // t8a
        vqsub.s32       d5,  d31, d25  // t12a
        vqadd.s32       d31, d31, d25  // t15a
        vqadd.s32       d19, d21, d23  // t9
        vqsub.s32       d21, d21, d23  // t10
        vqsub.s32       d25, d27, d29  // t13
        vqadd.s32       d27, d27, d29  // t14

.irp r, d4, d17, d5, d31, d19, d21, d25, d27
        vmin.s32        \r,  \r,  d9
.endr
.irp r, d4, d17, d5, d31, d19, d21, d25, d27
        vmax.s32        \r,  \r,  d8
.endr

        vmul_vmls       d6,  d5,  d4,  d0[0], d0[0]  // -> t11
        vmul_vmla       d7,  d5,  d4,  d0[0], d0[0]  // -> t12
        vmul_vmls       d4,  d25, d21, d0[0], d0[0]  // -> t10a

        vrshr.s32       d6,  d6,  #12  // t11
        vrshr.s32       d7,  d7,  #12  // t12
        vmul_vmla       d5,  d25, d21, d0[0], d0[0]  // -> t13a
        vrshr.s32       d4,  d4,  #12  // t10a
        vrshr.s32       d5,  d5,  #12  // t13a

        vqadd.s32       d8,  d16, d31  // out0
        vqsub.s32       d31, d16, d31  // out15
        vmov            d16, d8
        vqadd.s32       d23, d30, d17  // out7
        vqsub.s32       d9,  d30, d17  // out8
        vqadd.s32       d17, d18, d27  // out1
        vqsub.s32       d30, d18, d27  // out14
        vqadd.s32       d18, d20, d5   // out2
        vqsub.s32       d29, d20, d5   // out13
        vqadd.s32       d5,  d28, d19  // out6
        vqsub.s32       d25, d28, d19  // out9
        vqadd.s32       d19, d22, d7   // out3
        vqsub.s32       d28, d22, d7   // out12
        vqadd.s32       d20, d24, d6   // out4
        vqsub.s32       d27, d24, d6   // out11
        vqadd.s32       d21, d26, d4   // out5
        vqsub.s32       d26, d26, d4   // out10
        vmov            d24, d9
        vmov            d22, d5

        bx              lr
endfunc

.macro iadst_16 o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15
        movrel_local    r12, iadst16_coeffs
        vld1.32         {q0, q1}, [r12, :128]!

        vmul_vmla       d4,  d31, d16, d0[0], d0[1] // -> t0
        vmul_vmls       d6,  d31, d16, d0[1], d0[0] // -> t1
        vmul_vmla       d8,  d29, d18, d1[0], d1[1] // -> t2
        vrshr.s32       d16, d4,  #12               // t0
        vrshr.s32       d31, d6,  #12               // t1
        vmul_vmls       d4,  d29, d18, d1[1], d1[0] // -> t3
        vmul_vmla       d6,  d27, d20, d2[0], d2[1] // -> t4
        vrshr.s32       d18, d8,  #12               // t2
        vrshr.s32       d29, d4,  #12               // t3
        vmul_vmls       d8,  d27, d20, d2[1], d2[0] // -> t5
        vmul_vmla       d4,  d25, d22, d3[0], d3[1] // -> t6
        vrshr.s32       d20, d6,  #12               // t4
        vrshr.s32       d27, d8,  #12               // t5
        vmul_vmls       d6,  d25, d22, d3[1], d3[0] // -> t7
        vld1.32         {q0, q1}, [r12, :128]
        movrel_local    r12, idct_coeffs
        vmul_vmla       d8,  d23, d24, d0[0], d0[1] // -> t8
        vrshr.s32       d22, d4,  #12               // t6
        vrshr.s32       d25, d6,  #12               // t7
        vmul_vmls       d4,  d23, d24, d0[1], d0[0] // -> t9
        vmul_vmla       d6,  d21, d26, d1[0], d1[1] // -> t10
        vrshr.s32       d23, d8,  #12               // t8
        vrshr.s32       d24, d4,  #12               // t9
        vmul_vmls       d8,  d21, d26, d1[1], d1[0] // -> t11
        vmul_vmla       d4,  d19, d28, d2[0], d2[1] // -> t12
        vrshr.s32       d21, d6,  #12               // t10
        vrshr.s32       d26, d8,  #12               // t11
        vmul_vmls       d6,  d19, d28, d2[1], d2[0] // -> t13
        vmul_vmla       d8,  d17, d30, d3[0], d3[1] // -> t14
        vrshr.s32       d19, d4,  #12               // t12
        vrshr.s32       d28, d6,  #12               // t13
        vmul_vmls       d4,  d17, d30, d3[1], d3[0] // -> t15
        vrshr.s32       d17, d8,  #12               // t14
        vrshr.s32       d30, d4,  #12               // t15

        vld1.32         {q0, q1}, [r12, :128]

        vmov.i32        d11, #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        d10, #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000

        vqsub.s32       d5,  d16, d23 // t8a
        vqadd.s32       d16, d16, d23 // t0a
        vqsub.s32       d7,  d31, d24 // t9a
        vqadd.s32       d31, d31, d24 // t1a
        vqadd.s32       d23, d18, d21 // t2a
        vqsub.s32       d18, d18, d21 // t10a
        vqadd.s32       d24, d29, d26 // t3a
        vqsub.s32       d29, d29, d26 // t11a
        vqadd.s32       d21, d20, d19 // t4a
        vqsub.s32       d20, d20, d19 // t12a
        vqadd.s32       d26, d27, d28 // t5a
        vqsub.s32       d27, d27, d28 // t13a
        vqadd.s32       d19, d22, d17 // t6a
        vqsub.s32       d22, d22, d17 // t14a
        vqadd.s32       d28, d25, d30 // t7a
        vqsub.s32       d25, d25, d30 // t15a

.irp r, d5, d16, d7, d31, d23, d18, d24, d29, d21, d20, d26, d27, d19, d22, d28, d25
        vmin.s32        \r,  \r,  d11
.endr
.irp r, d5, d16, d7, d31, d23, d18, d24, d29, d21, d20, d26, d27, d19, d22, d28, d25
        vmax.s32        \r,  \r,  d10
.endr

        vmul_vmla       d4,  d5,  d7,  d2[1], d2[0] // -> t8
        vmul_vmls       d6,  d5,  d7,  d2[0], d2[1] // -> t9
        vmul_vmla       d8,  d18, d29, d3[1], d3[0] // -> t10
        vrshr.s32       d17, d4,  #12               // t8
        vrshr.s32       d30, d6,  #12               // t9
        vmul_vmls       d4,  d18, d29, d3[0], d3[1] // -> t11
        vmul_vmls       d6,  d27, d20, d2[1], d2[0] // -> t12
        vrshr.s32       d18, d8,  #12               // t10
        vrshr.s32       d29, d4,  #12               // t11
        vmul_vmla       d8,  d27, d20, d2[0], d2[1] // -> t13
        vmul_vmls       d4,  d25, d22, d3[1], d3[0] // -> t14
        vrshr.s32       d27, d6,  #12               // t12
        vrshr.s32       d20, d8,  #12               // t13
        vmul_vmla       d6,  d25, d22, d3[0], d3[1] // -> t15
        vrshr.s32       d25, d4,  #12               // t14
        vrshr.s32       d22, d6,  #12               // t15

        vqsub.s32       d2,  d16, d21 // t4
        vqadd.s32       d16, d16, d21 // t0
        vqsub.s32       d3,  d31, d26 // t5
        vqadd.s32       d31, d31, d26 // t1
        vqadd.s32       d21, d23, d19 // t2
        vqsub.s32       d23, d23, d19 // t6
        vqadd.s32       d26, d24, d28 // t3
        vqsub.s32       d24, d24, d28 // t7
        vqadd.s32       d19, d17, d27 // t8a
        vqsub.s32       d17, d17, d27 // t12a
        vqadd.s32       d28, d30, d20 // t9a
        vqsub.s32       d30, d30, d20 // t13a
        vqadd.s32       d27, d18, d25 // t10a
        vqsub.s32       d18, d18, d25 // t14a
        vqadd.s32       d20, d29, d22 // t11a
        vqsub.s32       d29, d29, d22 // t15a

.irp r, d2, d16, d3, d31, d21, d23, d26, d24, d19, d17, d28, d30, d27, d18, d20, d29
        vmin.s32        \r,  \r,  d11
.endr
.irp r, d2, d16, d3, d31, d21, d23, d26, d24, d19, d17, d28, d30, d27, d18, d20, d29
        vmax.s32        \r,  \r,  d10
.endr

        vmul_vmla       d4,  d2,  d3,  d1[1], d1[0] // -> t4a
        vmul_vmls       d6,  d2,  d3,  d1[0], d1[1] // -> t5a
        vmul_vmls       d8,  d24, d23, d1[1], d1[0] // -> t6a
        vrshr.s32       d22, d4,  #12               // t4a
        vrshr.s32       d25, d6,  #12               // t5a
        vmul_vmla       d4,  d24, d23, d1[0], d1[1] // -> t7a
        vmul_vmla       d6,  d17, d30, d1[1], d1[0] // -> t12
        vrshr.s32       d24, d8,  #12               // t6a
        vrshr.s32       d23, d4,  #12               // t7a
        vmul_vmls       d8,  d17, d30, d1[0], d1[1] // -> t13
        vmul_vmls       d4,  d29, d18, d1[1], d1[0] // -> t14
        vrshr.s32       d17, d6,  #12               // t12
        vmul_vmla       d6,  d29, d18, d1[0], d1[1] // -> t15
        vrshr.s32       d29, d8,  #12               // t13
        vrshr.s32       d30, d4,  #12               // t14
        vrshr.s32       d18, d6,  #12               // t15

        vqsub.s32       d2,  d16, d21 // t2a
.ifc \o0, d16
        vqadd.s32       \o0, d16, d21 // out0
        vqsub.s32       d21, d31, d26 // t3a
        vqadd.s32       \o15,d31, d26 // out15
.else
        vqadd.s32       d4,  d16, d21 // out0
        vqsub.s32       d21, d31, d26 // t3a
        vqadd.s32       \o15,d31, d26 // out15
        vmov            \o0, d4
.endif

        vqsub.s32       d3,  d29, d18 // t15a
        vqadd.s32       \o13,d29, d18 // out13
        vqadd.s32       \o2, d17, d30 // out2
        vqsub.s32       d26, d17, d30 // t14a

        vqadd.s32       \o1, d19, d27 // out1
        vqsub.s32       d27, d19, d27 // t10
        vqadd.s32       \o14,d28, d20 // out14
        vqsub.s32       d20, d28, d20 // t11

        vqadd.s32       \o3, d22, d24 // out3
        vqsub.s32       d22, d22, d24 // t6
        vqadd.s32       \o12,d25, d23 // out12
        vqsub.s32       d23, d25, d23 // t7

        // Not clipping the output registers, as they will be downshifted and
        // narrowed afterwards anyway.
.irp r, d2, d21, d3, d26, d27, d20, d22, d23
        vmin.s32        \r,  \r,  d11
.endr
.irp r, d2, d21, d3, d26, d27, d20, d22, d23
        vmax.s32        \r,  \r,  d10
.endr

        vqneg.s32       \o15, \o15    // out15
        vqneg.s32       \o13,\o13     // out13
        vqneg.s32       \o1, \o1      // out1
        vqneg.s32       \o3, \o3      // out3

        vmul_vmls       d24, d2,  d21, d0[0], d0[0] // -> out8 (d24 or d23)
        vmul_vmla       d4,  d2,  d21, d0[0], d0[0] // -> out7 (d23 or d24)
        vmul_vmla       d6,  d26, d3,  d0[0], d0[0] // -> out5 (d21 or d26)

        vrshr.s32       d24, d24, #12 // out8
        vrshr.s32       d4,  d4,  #12 // out7
        vrshr.s32       d5,  d6,  #12 // out5
        vmul_vmls       d8,  d26, d3,  d0[0], d0[0] // -> out10 (d26 or d21)
        vmul_vmla       d2,  d22, d23, d0[0], d0[0] // -> out4 (d20 or d27)
        vrshr.s32       d26, d8,  #12 // out10

        vmul_vmls       d8,  d22, d23, d0[0], d0[0] // -> out11 (d27 or d20)
        vmul_vmla       d22, d27, d20, d0[0], d0[0] // -> out6 (d22 or d25)
        vmul_vmls       d6,  d27, d20, d0[0], d0[0] // -> out9 (d25 or d22)

        vrshr.s32       \o4, d2,  #12 // out4
        vrshr.s32       d7,  d6,  #12 // out9
        vrshr.s32       d6,  d8,  #12 // out11
        vrshr.s32       \o6, d22, #12 // out6

.ifc \o8, d23
        vmov            \o8, d24
        vmov            \o10,d26
.endif

        vqneg.s32       \o7, d4  // out7
        vqneg.s32       \o5, d5  // out5
        vqneg.s32       \o11,d6  // out11
        vqneg.s32       \o9, d7  // out9
.endm

function inv_adst_2s_x16_neon
        iadst_16        d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        bx              lr
endfunc

function inv_flipadst_2s_x16_neon
        iadst_16        d31, d30, d29, d28, d27, d26, d25, d24, d23, d22, d21, d20, d19, d18, d17, d16
        bx              lr
endfunc

function inv_identity_2s_x16_neon
        mov             r12, #0
        movt            r12, #2*(5793-4096)*8
        vdup.32         d0,  r12
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vqrdmulh.s32    q1,  \i,  d0[0]
        vqadd.s32       \i,  \i,  \i
        vqadd.s32       \i,  \i,  q1
.endr
        bx              lr
endfunc

.macro identity_8x4_shift1 c
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vqrdmulh.s32    q2,  \i,  \c
        vrshr.s32       q2,  q2,  #1
        vqadd.s32       \i,  \i,  q2
.endr
.endm

.macro identity_8x4 c
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vqrdmulh.s32    q2,  \i,  \c
        vqadd.s32       \i,  \i,  \i
        vqadd.s32       \i,  \i,  q2
.endr
.endm

.macro def_horz_16 scale=0, shift=2, suffix
function inv_txfm_horz\suffix\()_16x2_neon
        push            {lr}
        vmov.i32        d7,  #0
.if \scale
        mov_const       r12, 2896*8*(1<<16)
        vdup.32         d1,  r12
.endif
.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.32         {\i}, [r7, :64]
        vst1.32         {d7}, [r7, :64], r8
.endr
.if \scale
        scale_input     d1[0], q8,  q9, q10, q11, q12, q13, q14, q15
.endif
        blx             r4
        vqrshrn.s32     d16, q8,  #\shift
        vqrshrn.s32     d17, q9,  #\shift
        vqrshrn.s32     d18, q10, #\shift
        vqrshrn.s32     d19, q11, #\shift
        vqrshrn.s32     d20, q12, #\shift
        vqrshrn.s32     d21, q13, #\shift
        vqrshrn.s32     d22, q14, #\shift
        vqrshrn.s32     d23, q15, #\shift
        vuzp.16         q8,  q9
        vuzp.16         q10, q11

.irp i, q8, q10, q9, q11
        vst1.16         {\i}, [r6, :128]!
.endr

        pop             {pc}
endfunc
.endm

def_horz_16 scale=0, shift=2
def_horz_16 scale=1, shift=1, suffix=_scale

function inv_txfm_add_vert_4x16_neon
        push            {lr}
.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.16         {\i}, [r7, :64], r8
.endr
        blx             r5
        load_add_store_4x16 r6, r7
        pop             {pc}
endfunc

function inv_txfm_add_16x16_neon
        sub_sp_align    512
        ldrh            r11, [r10], #2
.irp i, 0, 2, 4, 6, 8, 10, 12, 14
        add             r6,  sp,  #(\i*16*2)
.if \i > 0
        mov             r8,  #(16 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 14
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #16*4
        bl              inv_txfm_horz_16x2_neon
.endr
        b               3f
1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 2
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b
3:
.irp i, 0, 4, 8, 12
        add             r6,  r0,  #(\i*2)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    512
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

const eob_16x16
        .short 3, 10, 21, 36, 55, 78, 105, 256
endconst

const eob_16x16_identity
        .short 2, 4, 6, 8, 10, 12, 14, 256
endconst

.macro def_fn_16x16 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_16x16_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         16,  16,  2
.endif
        push            {r4-r11,lr}
        vpush           {q4-q7}
        movrel_local    r4,  inv_\txfm1\()_2s_x16_neon
        movrel          r5,  X(inv_\txfm2\()_4h_x16_neon)
.ifc \txfm1, identity
.ifc \txfm2, identity
        movrel_local    r10, eob_16x16
.else
        movrel_local    r10, eob_16x16_identity
.endif
.else
.ifc \txfm2, identity
        movrel_local    r10, eob_16x16_identity
.else
        movrel_local    r10, eob_16x16
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
        cmp             r3,  r10
        mov             r11, #16
        blt             1f

        add             r6,  r2,  #8
        vmov.i32        d4,  #0
.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.32         {\i}, [r6, :64]
        vst1.32         {d4}, [r6, :64], r11
.endr
        blx             r4

        vqrshrn.s32     d16, q8,  #1
        vqrshrn.s32     d17, q9,  #1
        vqrshrn.s32     d18, q10, #1
        vqrshrn.s32     d19, q11, #1
        vqrshrn.s32     d20, q12, #1
        vqrshrn.s32     d21, q13, #1
        vqrshrn.s32     d22, q14, #1
        vqrshrn.s32     d23, q15, #1
        vuzp.16         q8,  q9
        mov             r6,  sp
        vuzp.16         q10, q11
        vpush           {q8-q11}

        b               2f

1:
        vmov.i16        q8,  #0
        vmov.i16        q9,  #0
        mov             r6,  sp
        vpush           {q8-q9}
        vpush           {q8-q9}

2:
        vmov.i32        d4,  #0
.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.32         {\i}, [r2, :64]
        vst1.32         {d4}, [r2, :64], r11
.endr

        blx             r4

        vqrshrn.s32     d16, q8,  #1
        vqrshrn.s32     d17, q9,  #1
        vqrshrn.s32     d18, q10, #1
        vqrshrn.s32     d19, q11, #1
        vqrshrn.s32     d20, q12, #1
        vqrshrn.s32     d21, q13, #1
        vqrshrn.s32     d22, q14, #1
        vqrshrn.s32     d23, q15, #1
        vuzp.16         q8,  q9
        mov             r6,  sp
        vuzp.16         q10, q11

        vmov            q12, q10
        vmov            q13, q11

        vpop            {q10-q11}
        blx             r5
        mov             r6,  r0
        load_add_store_8x4 r6, r7

        vpop            {q10-q11}
        vmov            q8,  q12
        vmov            q9,  q13
        blx             r5
        add             r6,  r0,  #16
        load_add_store_8x4 r6, r7

        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_4x16_neon
        ldrh            r9,  [r10, #4]

        mov             r11, #64
        cmp             r3,  r9
        ldrh            r9,  [r10, #2]
        blt             1f

        add             r6,  r2,  #48
        vmov.i32        q2,  #0
.irp i, q8,  q9,  q10, q11
        vld1.32         {\i}, [r6, :128]
        vst1.32         {q2}, [r6, :128], r11
.endr
        blx             r4
        vqrshrn.s32     d28, q8,  #1
        vqrshrn.s32     d29, q9,  #1
        vqrshrn.s32     d30, q10, #1
        vqrshrn.s32     d31, q11, #1
        transpose_4x4h  q14, q15, d28, d29, d30, d31

        b               2f
1:
        vmov.i16        q14, #0
        vmov.i16        q15, #0
2:
        cmp             r3,  r9
        ldrh            r9,  [r10]
        blt             1f

        add             r6,  r2,  #32
        vmov.i32        q2,  #0
.irp i, q8,  q9,  q10, q11
        vld1.32         {\i}, [r6, :128]
        vst1.32         {q2}, [r6, :128], r11
.endr
        blx             r4
        vqrshrn.s32     d24, q8,  #1
        vqrshrn.s32     d25, q9,  #1
        vqrshrn.s32     d26, q10, #1
        vqrshrn.s32     d27, q11, #1
        transpose_4x4h  q12, q13, d24, d25, d26, d27

        b               2f
1:
        vmov.i16        q12, #0
        vmov.i16        q13, #0
2:
        cmp             r3,  r9
        blt             1f

        add             r6,  r2,  #16
        vmov.i32        q2,  #0
.irp i, q8,  q9,  q10, q11
        vld1.32         {\i}, [r6, :128]
        vst1.32         {q2}, [r6, :128], r11
.endr
        blx             r4
        vqrshrn.s32     d16, q8,  #1
        vqrshrn.s32     d17, q9,  #1
        vqrshrn.s32     d18, q10, #1
        vqrshrn.s32     d19, q11, #1
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19

        b               2f
1:
        vmov.i16        q8,  #0
        vmov.i16        q9,  #0
2:
        vmov.i16        q2,  #0
        vpush           {q8-q9}
.irp i, q8,  q9,  q10, q11
        vld1.16         {\i}, [r2, :128]
        vst1.16         {q2}, [r2, :128], r11
.endr
        blx             r4
        vqrshrn.s32     d16, q8,  #1
        vqrshrn.s32     d17, q9,  #1
        vqrshrn.s32     d18, q10, #1
        vqrshrn.s32     d19, q11, #1
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        vpop            {q10-q11}

        blx             r5

        load_add_store_4x16 r0, r6

        vpop            {q4-q7}
        pop             {r4-r11,pc}
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

.macro def_fn_416 w, h, txfm1, txfm2, eob_16x4
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
        push            {r4-r11,lr}
        vpush           {q4-q7}
.if \w == 4
        movrel_local    r4,  inv_\txfm1\()_4s_x\w\()_neon
        movrel          r5,  X(inv_\txfm2\()_4h_x\h\()_neon)
.ifc \txfm1, identity
.ifc \txfm2, identity
        movrel_local    r10, eob_4x16
.else
        movrel_local    r10, eob_4x16_identity1
.endif
.else
.ifc \txfm2, identity
        movrel_local    r10, eob_4x16_identity2
.else
        movrel_local    r10, eob_4x16
.endif
.endif
.else
        mov             r10, #\eob_16x4
        movrel_local    r4,  inv_\txfm1\()_2s_x\w\()_neon
        movrel          r5,  X(inv_\txfm2\()_8h_x\h\()_neon)
.endif
        b               inv_txfm_add_\w\()x\h\()_neon
endfunc
.endm

.macro def_fns_416 w, h
def_fn_416 \w, \h, dct, dct, 3
def_fn_416 \w, \h, identity, identity, 3
def_fn_416 \w, \h, dct, adst, 3
def_fn_416 \w, \h, dct, flipadst, 3
def_fn_416 \w, \h, dct, identity, 2
def_fn_416 \w, \h, adst, dct, 3
def_fn_416 \w, \h, adst, adst, 3
def_fn_416 \w, \h, adst, flipadst, 3
def_fn_416 \w, \h, flipadst, dct, 3
def_fn_416 \w, \h, flipadst, adst, 3
def_fn_416 \w, \h, flipadst, flipadst, 3
def_fn_416 \w, \h, identity, dct, 2
def_fn_416 \w, \h, adst, identity, 2
def_fn_416 \w, \h, flipadst, identity, 2
def_fn_416 \w, \h, identity, adst, 2
def_fn_416 \w, \h, identity, flipadst, 2
.endm

def_fns_416 4, 16
def_fns_416 16, 4

function inv_txfm_add_16x8_neon
        sub_sp_align    256
        ldrh            r11, [r10], #2

.irp i, 0, 2, 4, 6
        add             r6,  sp,  #(\i*16*2)
.if \i > 0
        mov             r8,  #(8 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 6
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #8*4
        bl              inv_txfm_horz_scale_16x2_neon
.endr
        b               3f
1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 2
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b
3:

.irp i, 0, 8
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32
.irp j, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\j}, [r7, :128], r8
.endr
        blx             r5

        add             r6,  r0,  #(\i*2)
        load_add_store_8x8 r6, r7
.endr

        add_sp_align    256
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_8x16_neon
        add             r10, r10, #2
        sub_sp_align    256
        ldrh            r11, [r10], #4

.irp i, 0, 4, 8, 12
        add             r6,  sp,  #(\i*8*2)
.if \i > 0
        mov             r8,  #(16 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 12
        ldrh            r11, [r10], #4
.endif
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #16*4

        mov_const       r12, 2896*8*(1<<16)
        vmov.i32        q2,  #0
        vdup.32         d0,  r12

.irp j, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.32         {\j}, [r7, :128]
        vst1.32         {q2}, [r7, :128], r8
.endr
        scale_input     d0[0], q8,  q9,  q10, q11, q12, q13, q14, q15
        blx             r4
        vqrshrn.s32     d16, q8,  #1
        vqrshrn.s32     d17, q9,  #1
        vqrshrn.s32     d18, q10, #1
        vqrshrn.s32     d19, q11, #1
        vqrshrn.s32     d20, q12, #1
        vqrshrn.s32     d21, q13, #1
        vqrshrn.s32     d22, q14, #1
        vqrshrn.s32     d23, q15, #1
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q10, q11, d20, d21, d22, d23
.irp j, d16, d20, d17, d21, d18, d22, d19, d23
        vst1.16         {\j},  [r6, :64]!
.endr
.endr
        b               3f
1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #4
.rept 2
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b
3:

.irp i, 0, 4
        add             r6,  r0,  #(\i*2)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #16
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    256
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

const eob_8x16
        .short 3, 10, 21, 43, 59, 75, 91, 128
endconst

const eob_8x16_identity1
        .short 2, 4, 6, 64, 80, 96, 112, 128
endconst

const eob_8x16_identity2
        .short 2, 4, 6, 8, 10, 12, 14, 128
endconst

.macro def_fn_816 w, h, txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_16bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
        push            {r4-r11,lr}
        vpush           {q4-q7}
.if \w == 8
        movrel_local    r4,  inv_\txfm1\()_4s_x8_neon
        movrel          r5,  X(inv_\txfm2\()_4h_x16_neon)
.else
        movrel_local    r4,  inv_\txfm1\()_2s_x16_neon
        movrel          r5,  X(inv_\txfm2\()_8h_x8_neon)
.endif
.ifc \txfm1, identity
.ifc \txfm2, identity
        movrel_local    r10, eob_8x16
.else
        movrel_local    r10, eob_8x16_identity1
.endif
.else
.ifc \txfm2, identity
        movrel_local    r10, eob_8x16_identity2
.else
        movrel_local    r10, eob_8x16
.endif
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

function inv_dct32_odd_2s_x16_neon
        movrel_local    r12, idct_coeffs, 4*16
        vld1.32         {q0, q1}, [r12, :128]!

        vmul_vmls       d4,  d16, d31, d0[0], d0[1] // -> t16a
        vmul_vmla       d6,  d16, d31, d0[1], d0[0] // -> t31a
        vmul_vmls       d8,  d24, d23, d1[0], d1[1] // -> t17a
        vrshr.s32       d16, d4,  #12               // t16a
        vrshr.s32       d31, d6,  #12               // t31a
        vmul_vmla       d4,  d24, d23, d1[1], d1[0] // -> t30a
        vmul_vmls       d6,  d20, d27, d2[0], d2[1] // -> t18a
        vrshr.s32       d24, d8,  #12               // t17a
        vrshr.s32       d23, d4,  #12               // t30a
        vmul_vmla       d8,  d20, d27, d2[1], d2[0] // -> t29a
        vmul_vmls       d4,  d28, d19, d3[0], d3[1] // -> t19a
        vrshr.s32       d20, d6,  #12               // t18a
        vrshr.s32       d27, d8,  #12               // t29a
        vmul_vmla       d6,  d28, d19, d3[1], d3[0] // -> t28a
        vld1.32         {q0, q1}, [r12, :128]
        sub             r12, r12, #4*24
        vmul_vmls       d8,  d18, d29, d0[0], d0[1] // -> t20a
        vrshr.s32       d28, d4,  #12               // t19a
        vrshr.s32       d19, d6,  #12               // t28a
        vmul_vmla       d4,  d18, d29, d0[1], d0[0] // -> t27a
        vmul_vmls       d6,  d26, d21, d1[0], d1[1] // -> t21a
        vrshr.s32       d18, d8,  #12               // t20a
        vrshr.s32       d29, d4,  #12               // t27a
        vmul_vmla       d8,  d26, d21, d1[1], d1[0] // -> t26a
        vmul_vmls       d4,  d22, d25, d2[0], d2[1] // -> t22a
        vrshr.s32       d26, d6,  #12               // t21a
        vrshr.s32       d21, d8,  #12               // t26a
        vmul_vmla       d6,  d22, d25, d2[1], d2[0] // -> t25a
        vmul_vmls       d8,  d30, d17, d3[0], d3[1] // -> t23a
        vrshr.s32       d22, d4,  #12               // t22a
        vrshr.s32       d25, d6,  #12               // t25a
        vmul_vmla       d4,  d30, d17, d3[1], d3[0] // -> t24a
        vrshr.s32       d30, d8,  #12               // t23a
        vrshr.s32       d17, d4,  #12               // t24a

        vld1.32         {q0, q1}, [r12, :128]

        vmov.i32        d11, #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        d10, #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000

        vqsub.s32       d5,  d16, d24 // t17
        vqadd.s32       d16, d16, d24 // t16
        vqsub.s32       d7,  d31, d23 // t30
        vqadd.s32       d31, d31, d23 // t31
        vqsub.s32       d24, d28, d20 // t18
        vqadd.s32       d28, d28, d20 // t19
        vqadd.s32       d23, d18, d26 // t20
        vqsub.s32       d18, d18, d26 // t21
        vqsub.s32       d20, d30, d22 // t22
        vqadd.s32       d30, d30, d22 // t23
        vqadd.s32       d26, d17, d25 // t24
        vqsub.s32       d17, d17, d25 // t25
        vqsub.s32       d22, d29, d21 // t26
        vqadd.s32       d29, d29, d21 // t27
        vqadd.s32       d25, d19, d27 // t28
        vqsub.s32       d19, d19, d27 // t29

.irp r, d5, d16, d7, d31, d24, d28, d23, d18, d20, d30, d26, d17, d22, d29, d25, d19
        vmin.s32        \r,  \r,  d11
.endr
.irp r, d5, d16, d7, d31, d24, d28, d23, d18, d20, d30, d26, d17, d22, d29, d25, d19
        vmax.s32        \r,  \r,  d10
.endr

        vmul_vmls       d4,  d7,  d5,  d2[0], d2[1] // -> t17a
        vmul_vmla       d6,  d7,  d5,  d2[1], d2[0] // -> t30a
        vmul_vmla       d8,  d19, d24, d2[1], d2[0] // -> t18a
        vrshr.s32       d21, d4,  #12               // t17a
        vrshr.s32       d27, d6,  #12               // t30a
        vneg.s32        d8,  d8                     // -> t18a
        vmul_vmls       d5,  d19, d24, d2[0], d2[1] // -> t29a
        vmul_vmls       d4,  d22, d18, d3[0], d3[1] // -> t21a
        vrshr.s32       d19, d8,  #12               // t18a
        vrshr.s32       d24, d5,  #12               // t29a
        vmul_vmla       d6,  d22, d18, d3[1], d3[0] // -> t26a
        vmul_vmla       d8,  d17, d20, d3[1], d3[0] // -> t22a
        vrshr.s32       d22, d4,  #12               // t21a
        vrshr.s32       d18, d6,  #12               // t26a
        vneg.s32        d8,  d8                     // -> t22a
        vmul_vmls       d5,  d17, d20, d3[0], d3[1] // -> t25a
        vrshr.s32       d17, d8,  #12               // t22a
        vrshr.s32       d20, d5,  #12               // t25a

        vqsub.s32       d2,  d27, d24 // t29
        vqadd.s32       d27, d27, d24 // t30
        vqsub.s32       d3,  d21, d19 // t18
        vqadd.s32       d21, d21, d19 // t17
        vqsub.s32       d24, d16, d28 // t19a
        vqadd.s32       d16, d16, d28 // t16a
        vqsub.s32       d19, d30, d23 // t20a
        vqadd.s32       d30, d30, d23 // t23a
        vqsub.s32       d28, d17, d22 // t21
        vqadd.s32       d17, d17, d22 // t22
        vqadd.s32       d23, d26, d29 // t24a
        vqsub.s32       d26, d26, d29 // t27a
        vqadd.s32       d22, d20, d18 // t25
        vqsub.s32       d20, d20, d18 // t26
        vqsub.s32       d29, d31, d25 // t28a
        vqadd.s32       d31, d31, d25 // t31a

.irp r, d2, d27, d3, d21, d24, d16, d19, d30, d28, d17, d23, d26, d22, d20, d29, d31
        vmin.s32        \r,  \r,  d11
.endr
.irp r, d2, d27, d3, d21, d24, d16, d19, d30, d28, d17, d23, d26, d22, d20, d29, d31
        vmax.s32        \r,  \r,  d10
.endr

        vmul_vmls       d4,  d2,  d3,  d1[0], d1[1] // -> t18a
        vmul_vmla       d6,  d2,  d3,  d1[1], d1[0] // -> t29a
        vmul_vmls       d8,  d29, d24, d1[0], d1[1] // -> t19
        vrshr.s32       d18, d4,  #12               // t18a
        vrshr.s32       d25, d6,  #12               // t29a
        vmul_vmla       d5,  d29, d24, d1[1], d1[0] // -> t28
        vmul_vmla       d4,  d26, d19, d1[1], d1[0] // -> t20
        vrshr.s32       d29, d8,  #12               // t19
        vrshr.s32       d24, d5,  #12               // t28
        vneg.s32        d4,  d4                     // -> t20
        vmul_vmls       d6,  d26, d19, d1[0], d1[1] // -> t27
        vmul_vmla       d8,  d20, d28, d1[1], d1[0] // -> t21a
        vrshr.s32       d26, d4,  #12               // t20
        vrshr.s32       d19, d6,  #12               // t27
        vneg.s32        d8,  d8                     // -> t21a
        vmul_vmls       d5,  d20, d28, d1[0], d1[1] // -> t26a
        vrshr.s32       d20, d8,  #12               // t21a
        vrshr.s32       d28, d5,  #12               // t26a

        vqsub.s32       d2,  d16, d30 // t23
        vqadd.s32       d16, d16, d30 // t16 = out16
        vqsub.s32       d3,  d31, d23 // t24
        vqadd.s32       d31, d31, d23 // t31 = out31
        vqsub.s32       d23, d21, d17 // t22a
        vqadd.s32       d17, d21, d17 // t17a = out17
        vqadd.s32       d30, d27, d22 // t30a = out30
        vqsub.s32       d21, d27, d22 // t25a
        vqsub.s32       d27, d18, d20 // t21
        vqadd.s32       d18, d18, d20 // t18 = out18
        vqadd.s32       d4,  d29, d26 // t19a = out19
        vqsub.s32       d26, d29, d26 // t20a
        vqadd.s32       d29, d25, d28 // t29 = out29
        vqsub.s32       d25, d25, d28 // t26
        vqadd.s32       d28, d24, d19 // t28a = out28
        vqsub.s32       d24, d24, d19 // t27a
        vmov            d19, d4       // out19

.irp r, d2, d16, d3, d31, d23, d17, d30, d21, d27, d18, d19, d26, d29, d25, d28, d24
        vmin.s32        \r,  \r,  d11
.endr
.irp r, d2, d16, d3, d31, d23, d17, d30, d21, d27, d18, d19, d26, d29, d25, d28, d24
        vmax.s32        \r,  \r,  d10
.endr

        vmul_vmls       d4,  d24, d26, d0[0], d0[0] // -> t20
        vmul_vmla       d6,  d24, d26, d0[0], d0[0] // -> t27
        vrshr.s32       d20, d4,  #12   // t20
        vrshr.s32       d22, d6,  #12   // t27

        vmul_vmla       d4,  d25, d27, d0[0], d0[0] // -> t26a
        vmul_vmls       d6,  d25, d27, d0[0], d0[0] // -> t21a
        vmov            d27, d22        // t27
        vrshr.s32       d26, d4,  #12   // t26a

        vmul_vmls       d24, d21, d23, d0[0], d0[0] // -> t22
        vmul_vmla       d4,  d21, d23, d0[0], d0[0] // -> t25
        vrshr.s32       d21, d6,  #12   // t21a
        vrshr.s32       d22, d24, #12   // t22
        vrshr.s32       d25, d4,  #12   // t25

        vmul_vmls       d4,  d3,  d2,  d0[0], d0[0] // -> t23a
        vmul_vmla       d6,  d3,  d2,  d0[0], d0[0] // -> t24a
        vrshr.s32       d23, d4,  #12   // t23a
        vrshr.s32       d24, d6,  #12   // t24a

        bx              lr
endfunc

.macro def_horz_32 scale=0, shift=2, suffix
function inv_txfm_horz\suffix\()_dct_32x2_neon
        push            {lr}
        vmov.i32        d7,  #0
        lsl             r8,  r8,  #1
.if \scale
        mov_const       r12, 2896*8*(1<<16)
        vdup.32         d0,  r12
.endif

.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.32         {\i}, [r7, :64]
        vst1.32         {d7}, [r7, :64], r8
.endr
        sub             r7,  r7,  r8, lsl #4
        add             r7,  r7,  r8, lsr #1
.if \scale
        scale_input     d0[0], q8,  q9,  q10, q11, q12, q13, q14, q15
.endif
        bl              inv_dct_2s_x16_neon

        // idct_16 leaves the row_clip_max/min constants in d9 and d8,
        // but here we want to use full q registers for clipping.
        vmov.i32        q3,  #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        q2,  #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000
.irp r, q8, q9, q10, q11, q12, q13, q14, q15
        vmin.s32        \r,  \r,  q3
.endr
.irp r, q8, q9, q10, q11, q12, q13, q14, q15
        vmax.s32        \r,  \r,  q2
.endr

        vtrn.32         d16, d17
        vtrn.32         d18, d19
        vtrn.32         d20, d21
        vtrn.32         d22, d23
        vtrn.32         d24, d25
        vtrn.32         d26, d27
        vtrn.32         d28, d29
        vtrn.32         d30, d31

.macro store1 r0, r1, r2, r3
        vst1.16         {\r0}, [r6, :64]!
        vst1.16         {\r1}, [r6, :64]!
        vst1.16         {\r2}, [r6, :64]!
        vst1.16         {\r3}, [r6, :64]!
.endm
        store1          d16, d18, d20, d22
        store1          d24, d26, d28, d30
        store1          d17, d19, d21, d23
        store1          d25, d27, d29, d31
.purgem store1
        sub             r6,  r6,  #64*2

        vmov.i32        d7,  #0
.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.32         {\i}, [r7, :64]
        vst1.32         {d7}, [r7, :64], r8
.endr
.if \scale
        // This relies on the fact that the idct also leaves the right coeff in d0[1]
        scale_input     d0[1], q8,  q9,  q10, q11, q12, q13, q14, q15
.endif
        bl              inv_dct32_odd_2s_x16_neon
        vtrn.32         d31, d30
        vtrn.32         d29, d28
        vtrn.32         d27, d26
        vtrn.32         d25, d24
        vtrn.32         d23, d22
        vtrn.32         d21, d20
        vtrn.32         d19, d18
        vtrn.32         d17, d16
.macro store2 r0, r1, r2, r3, r4, r5, r6, r7, shift
        vld1.32         {q0, q1}, [r6, :128]!
        vld1.32         {q2, q3}, [r6, :128]
        sub             r6,  r6,  #32
        vqsub.s32       d15, d0,  \r0
        vqadd.s32       d0,  d0,  \r0
        vqsub.s32       d14, d1,  \r1
        vqadd.s32       d1,  d1,  \r1
        vqsub.s32       d13, d2,  \r2
        vqadd.s32       d2,  d2,  \r2
        vqsub.s32       d12, d3,  \r3
        vqadd.s32       d3,  d3,  \r3
        vqsub.s32       d11, d4,  \r4
        vqadd.s32       d4,  d4,  \r4
        vqsub.s32       d10, d5,  \r5
        vqadd.s32       d5,  d5,  \r5
        vqsub.s32       d9,  d6,  \r6
        vqadd.s32       d6,  d6,  \r6
        vqsub.s32       d8,  d7,  \r7
        vqadd.s32       d7,  d7,  \r7
        vqrshrn.s32     d0,  q0,  #\shift
        vqrshrn.s32     d1,  q1,  #\shift
        vqrshrn.s32     d2,  q2,  #\shift
        vqrshrn.s32     d3,  q3,  #\shift
        vqrshrn.s32     d4,  q4,  #\shift
        vqrshrn.s32     d5,  q5,  #\shift
        vqrshrn.s32     d6,  q6,  #\shift
        vqrshrn.s32     d7,  q7,  #\shift
        vrev32.16       q2,  q2
        vrev32.16       q3,  q3
        vst1.16         {q0, q1}, [r6, :128]!
        vst1.16         {q2, q3}, [r6, :128]!
.endm

        store2          d31, d29, d27, d25, d23, d21, d19, d17, \shift
        store2          d30, d28, d26, d24, d22, d20, d18, d16, \shift
.purgem store2
        pop             {pc}
endfunc
.endm

def_horz_32 scale=0, shift=2
def_horz_32 scale=1, shift=1, suffix=_scale

function inv_txfm_add_vert_dct_4x32_neon
        push            {r10-r11,lr}
        lsl             r8,  r8,  #1

.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.16         {\i}, [r7, :64], r8
.endr
        sub             r7,  r7,  r8, lsl #4

        bl              X(inv_dct_4h_x16_neon)

.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vst1.16         {\i}, [r7, :64], r8
.endr
        sub             r7,  r7,  r8, lsl #4
        add             r7,  r7,  r8, lsr #1

.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.16         {\i}, [r7, :64], r8
.endr
        sub             r7,  r7,  r8, lsl #4
        sub             r7,  r7,  r8, lsr #1
        bl              X(inv_dct32_odd_4h_x16_neon)

        neg             r9,  r8
        mov             r10, r6
        vmov.i16        q6,  #0
        vmvn.i16        q7,  #0xfc00 // 0x3ff
.macro combine r0, r1, r2, r3, op, stride
        vld1.16         {d4}, [r7,  :64], \stride
        vld1.16         {d0}, [r10, :64], r1
        vld1.16         {d5}, [r7,  :64],  \stride
        vld1.16         {d1}, [r10, :64], r1
        \op\().s16      d4,  d4,  \r0
        vld1.16         {d6}, [r7,  :64], \stride
        vld1.16         {d2}, [r10, :64], r1
        \op\().s16      d5,  d5,  \r1
        vld1.16         {d3}, [r10, :64], r1
        vrshr.s16       q2,  q2,  #4
        \op\().s16      d6,  d6,  \r2
        vld1.16         {d7}, [r7,  :64], \stride
        vqadd.s16       q0,  q0,  q2
        \op\().s16      d7,  d7,  \r3
        vmax.s16        q0,  q0,  q6
        vrshr.s16       q3,  q3,  #4
        vmin.s16        q0,  q0,  q7
        vqadd.s16       q1,  q1,  q3
        vst1.16         {d0}, [r6,  :64], r1
        vmax.s16        q1,  q1,  q6
        vst1.16         {d1}, [r6,  :64], r1
        vmin.s16        q1,  q1,  q7
        vst1.16         {d2}, [r6,  :64], r1
        vst1.16         {d3}, [r6,  :64], r1
.endm
        combine         d31, d30, d29, d28, vqadd, r8
        combine         d27, d26, d25, d24, vqadd, r8
        combine         d23, d22, d21, d20, vqadd, r8
        combine         d19, d18, d17, d16, vqadd, r8
        sub             r7,  r7,  r8
        combine         d16, d17, d18, d19, vqsub, r9
        combine         d20, d21, d22, d23, vqsub, r9
        combine         d24, d25, d26, d27, vqsub, r9
        combine         d28, d29, d30, d31, vqsub, r9
.purgem combine

        pop             {r10-r11,pc}
endfunc

const eob_32x32
        .short 3, 10, 21, 36, 55, 78, 105, 136, 171, 210, 253, 300, 351, 406, 465, 1024
endconst

const eob_16x32
        .short 3, 10, 21, 36, 55, 78, 105, 151, 183, 215, 247, 279, 311, 343, 375, 512
endconst

const eob_16x32_shortside
        .short 3, 10, 21, 36, 55, 78, 105, 512
endconst

const eob_8x32
        .short 3, 10, 21, 43, 59, 75, 91, 107, 123, 139, 155, 171, 187, 203, 219, 256
endconst

function inv_txfm_add_identity_identity_32x32_16bpc_neon, export=1
        push            {r4-r7,lr}
        vpush           {q6-q7}
        movrel_local    r5,  eob_32x32, 2

        mov             r6,  #4*32
1:
        mov             r12, #0
        movrel_local    r4,  eob_32x32, 6
2:
        vmov.i32        q0,  #0
        add             r12, r12, #8
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.32         {\i}, [r2, :128]
        vst1.32         {q0}, [r2, :128], r6
.endr
        vqmovn.s32      d16, q8
        vqmovn.s32      d17, q12
        vqmovn.s32      d18, q9
        vqmovn.s32      d19, q13
        vqmovn.s32      d20, q10
        vqmovn.s32      d21, q14
        vqmovn.s32      d22, q11
        vqmovn.s32      d23, q15
        transpose_4x8h  q8,  q9,  q10, q11

        load_add_store_8x4 r0, r7, shiftbits=2
        ldrh            lr,  [r4], #8
        sub             r0,  r0,  r1, lsl #2
        cmp             r3,  lr
        add             r0,  r0,  #2*8
        bge             2b

        ldrh            lr,  [r5], #4
        cmp             r3,  lr
        blt             9f

        sub             r0,  r0,  r12, lsl #1
        add             r0,  r0,  r1,  lsl #2
        mls             r2,  r6,  r12, r2
        add             r2,  r2,  #4*4
        b               1b
9:
        vpop            {q6-q7}
        pop             {r4-r7,pc}
endfunc

.macro shift_8_regs op, shift
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        \op             \i,  \i,  #\shift
.endr
.endm

.macro def_identity_1632 w, h, wshort, hshort
function inv_txfm_add_identity_identity_\w\()x\h\()_16bpc_neon, export=1
        push            {r4-r9,lr}
        vpush           {q6-q7}
        mov             r9,  #0
        mov_const       r8,  2896*8*(1<<16)
        movt            r9,  #2*(5793-4096)*8
        movrel_local    r5,  eob_16x32\hshort, 2

        mov             r6,  #4*\h
1:
        mov             r12, #0
        movrel_local    r4,  eob_16x32\wshort, 6
2:
        vdup.i32        d0,  r8
        vmov.i32        q1,  #0
        vmov.32         d0[1], r9
        add             r12, r12, #8
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.32         {\i}, [r2, :128]
        vst1.32         {q1}, [r2, :128], r6
.endr
        scale_input     d0[0], q8,  q9, q10, q11, q12, q13, q14, q15

.if \w == 16
        // 16x32
        identity_8x4_shift1 d0[1]
.else
        // 32x16
        shift_8_regs    vqshl.s32, 1
        identity_8x4    d0[1]
.endif
        vqmovn.s32      d16, q8
        vqmovn.s32      d17, q12
        vqmovn.s32      d18, q9
        vqmovn.s32      d19, q13
        vqmovn.s32      d20, q10
        vqmovn.s32      d21, q14
        vqmovn.s32      d22, q11
        vqmovn.s32      d23, q15
        transpose_4x8h  q8,  q9,  q10, q11

.if \w == 16
        load_add_store_8x4 r0, r7, shiftbits=2
.else
        load_add_store_8x4 r0, r7, shiftbits=4
.endif
        ldrh            lr,  [r4], #8
        sub             r0,  r0,  r1, lsl #2
        cmp             r3,  lr
        add             r0,  r0,  #2*8
        bge             2b

        ldrh            lr,  [r5], #4
        cmp             r3,  lr
        blt             9f

        sub             r0,  r0,  r12, lsl #1
        add             r0,  r0,  r1,  lsl #2
        mls             r2,  r6,  r12, r2
        add             r2,  r2,  #4*4
        b               1b
9:
        vpop            {q6-q7}
        pop             {r4-r9,pc}
endfunc
.endm

def_identity_1632 16, 32, _shortside,
def_identity_1632 32, 16, , _shortside

.macro def_identity_832 w, h
function inv_txfm_add_identity_identity_\w\()x\h\()_16bpc_neon, export=1
        push            {r4-r5,lr}
        vpush           {q6-q7}
        movrel_local    r4,  eob_8x32, 2

        mov             r12, #4*\h
1:
        ldrh            lr,  [r4], #4
.if \w == 8
        vmov.i32        q0,  #0
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.32         {\i}, [r2, :128]
        vst1.32         {q0}, [r2, :128], r12
.endr

        vqrshrn.s32     d16, q8,  #1
        vqrshrn.s32     d17, q12, #1
        vqrshrn.s32     d18, q9,  #1
        vqrshrn.s32     d19, q13, #1
        vqrshrn.s32     d20, q10, #1
        vqrshrn.s32     d21, q14, #1
        vqrshrn.s32     d22, q11, #1
        vqrshrn.s32     d23, q15, #1

        transpose_4x8h  q8,  q9,  q10, q11

        cmp             r3,  lr
        load_add_store_8x4 r0, r5, shiftbits=2
        blt             9f
        sub             r2,  r2,  r12, lsl #3
        add             r2,  r2,  #4*4
.else
        vmov.i32        q0,  #0
        vmov.i32        q1,  #0
        vld1.32         {q8,  q9},  [r2, :128]
        vst1.32         {q0,  q1},  [r2, :128], r12
        vld1.32         {q10, q11}, [r2, :128]
        vst1.32         {q0,  q1},  [r2, :128], r12
        vld1.32         {q12, q13}, [r2, :128]
        vst1.32         {q0,  q1},  [r2, :128], r12
        vld1.32         {q14, q15}, [r2, :128]
        vst1.32         {q0,  q1},  [r2, :128], r12
        vqmovn.s32      d16, q8
        vqmovn.s32      d17, q10
        vqmovn.s32      d20, q9
        vqmovn.s32      d21, q11
        vqmovn.s32      d18, q12
        vqmovn.s32      d19, q14
        vqmovn.s32      d22, q13
        vqmovn.s32      d23, q15

        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q10, q11, d20, d21, d22, d23

        cmp             r3,  lr
        load_add_store_4x8 r0, r5, shiftbits=3
        blt             9f
        sub             r0,  r0,  r1, lsl #3
        add             r0,  r0,  #2*4
.endif
        b               1b

9:
        vpop            {q6-q7}
        pop             {r4-r5,pc}
endfunc
.endm

def_identity_832 8, 32
def_identity_832 32, 8

function inv_txfm_add_dct_dct_32x32_16bpc_neon, export=1
        idct_dc         32,  32,  2

        push            {r4-r11,lr}
        vpush           {q4-q7}
        sub_sp_align    2048
        movrel_local    r10, eob_32x32
        ldrh            r11, [r10], #2

.irp i, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
        add             r6,  sp,  #(\i*32*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 30
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #32*4
        bl              inv_txfm_horz_dct_32x2_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  r0,  #(\i*2)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    2048
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_16x32_16bpc_neon, export=1
        idct_dc         16,  32,  1

        push            {r4-r11,lr}
        vpush           {q4-q7}
        sub_sp_align    1024
        movrel_local    r10, eob_16x32
        ldrh            r11, [r10], #2
        movrel_local    r4,  inv_dct_2s_x16_neon

.irp i, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
        add             r6,  sp,  #(\i*16*2)
        add             r7,  r2,  #(\i*4)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 30
        ldrh            r11, [r10], #2
.endif
.endif
        mov             r8,  #4*32
        bl              inv_txfm_horz_scale_16x2_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 2
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12
        add             r6,  r0,  #(\i*2)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #16*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    1024
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_32x16_16bpc_neon, export=1
        idct_dc         32,  16,  1

        push            {r4-r11,lr}
        vpush           {q4-q7}
        sub_sp_align    1024
        movrel_local    r10, eob_16x32
        ldrh            r11, [r10], #2
        movrel          r5,  X(inv_dct_4h_x16_neon)

.irp i, 0, 2, 4, 6, 8, 10, 12, 14
        add             r6,  sp,  #(\i*32*2)
        add             r7,  r2,  #(\i*4)
.if \i > 0
        mov             r8,  #(16 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 14
        ldrh            r11, [r10], #2
.endif
.endif
        mov             r8,  #4*16
        bl              inv_txfm_horz_scale_dct_32x2_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  r0,  #(\i*2)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    1024
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_8x32_16bpc_neon, export=1
        idct_dc         8,   32,  2

        push            {r4-r11,lr}
        vpush           {q4-q7}
        sub_sp_align    512

        movrel_local    r10, eob_8x32, 2

        mov             r8,  #4*32
        mov             r9,  #32
        mov             r6,  sp
1:
        vmov.i32        q0,  #0
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.32         {\i}, [r2, :128]
        vst1.32         {q0}, [r2, :128], r8
.endr
        ldrh            r11, [r10], #4
        sub             r2,  r2,  r8, lsl #3
        sub             r9,  r9,  #4
        add             r2,  r2,  #4*4

        bl              inv_dct_4s_x8_neon

        vqrshrn.s32     d16, q8,  #2
        vqrshrn.s32     d18, q9,  #2
        vqrshrn.s32     d20, q10, #2
        vqrshrn.s32     d22, q11, #2
        vqrshrn.s32     d17, q12, #2
        vqrshrn.s32     d19, q13, #2
        vqrshrn.s32     d21, q14, #2
        vqrshrn.s32     d23, q15, #2

        transpose_4x8h  q8,  q9,  q10, q11

        vst1.16         {q8,  q9},  [r6, :128]!
        cmp             r3,  r11
        vst1.16         {q10, q11}, [r6, :128]!

        bge             1b
        cmp             r9,  #0
        beq             3f

        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r9,  r9,  #4
.rept 2
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4
        add             r6,  r0,  #(\i*2)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #8*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    512
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_32x8_16bpc_neon, export=1
        idct_dc         32,  8,   2

        push            {r4-r11,lr}
        vpush           {q4-q7}
        movrel_local    r10, eob_8x32
        sub_sp_align    512
        ldrh            r11, [r10], #2

.irp i, 0, 2, 4, 6
        add             r6,  sp,  #(\i*32*2)
        add             r7,  r2,  #(\i*4)
.if \i > 0
        cmp             r3,  r11
        mov             r8,  #(8 - \i)
        blt             1f
.if \i < 6
        ldrh            r11, [r10], #2
.endif
.endif
        mov             r8,  #8*4
        bl              inv_txfm_horz_dct_32x2_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
        mov             r8,  #2*32
        mov             r9,  #0
1:
        add             r6,  r0,  r9, lsl #1
        add             r7,  sp,  r9, lsl #1 // #(\i*2)

.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\i}, [r7, :128], r8
.endr
        add             r9,  r9,  #8

        bl              X(inv_dct_8h_x8_neon)

        cmp             r9,  #32

        load_add_store_8x8 r6, r7

        blt             1b

        add_sp_align    512
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_dct64_step1_neon
        // in1/31/17/15 -> t32a/33/34a/35/60/61a/62/63a
        // in7/25/23/ 9 -> t56a/57/58a/59/36/37a/38/39a
        // in5/27/21/11 -> t40a/41/42a/43/52/53a/54/55a
        // in3/29/19/13 -> t48a/49/50a/51/44/45a/46/47a

        vld1.32         {q0, q1}, [r12, :128]!

        vqrdmulh.s32    d23, d16, d0[1]  // t63a
        vqrdmulh.s32    d16, d16, d0[0]  // t32a
        vqrdmulh.s32    d22, d17, d1[0]  // t62a
        vqrdmulh.s32    d17, d17, d1[1]  // t33a
        vqrdmulh.s32    d21, d18, d2[1]  // t61a
        vqrdmulh.s32    d18, d18, d2[0]  // t34a
        vqrdmulh.s32    d20, d19, d3[0]  // t60a
        vqrdmulh.s32    d19, d19, d3[1]  // t35a

        vld1.32         {q0}, [r12, :128]!

        vqadd.s32       d24, d16, d17    // t32
        vqsub.s32       d25, d16, d17    // t33
        vqsub.s32       d26, d19, d18    // t34
        vqadd.s32       d27, d19, d18    // t35
        vqadd.s32       d28, d20, d21    // t60
        vqsub.s32       d29, d20, d21    // t61
        vqsub.s32       d30, d23, d22    // t62
        vqadd.s32       d31, d23, d22    // t63

.irp r, q12, q13, q14, q15
        vmin.s32        \r,  \r,  q5
.endr
.irp r, q12, q13, q14, q15
        vmax.s32        \r,  \r,  q4
.endr

        vmul_vmla       d4,  d29, d26, d0[0], d0[1] // -> t34a
        vmul_vmls       d6,  d29, d26, d0[1], d0[0] // -> t61a
        vneg.s32        d4,  d4                     // t34a
        vmul_vmls       d7,  d30, d25, d0[1], d0[0] // -> t33a
        vrshr.s32       d26, d4,  #12               // t34a
        vmul_vmla       d4,  d30, d25, d0[0], d0[1] // -> t62a
        vrshr.s32       d29, d6,  #12               // t61a
        vrshr.s32       d25, d7,  #12               // t33a
        vrshr.s32       d30, d4,  #12               // t62a

        vqadd.s32       d16, d24, d27    // t32a
        vqsub.s32       d19, d24, d27    // t35a
        vqadd.s32       d17, d25, d26    // t33
        vqsub.s32       d18, d25, d26    // t34
        vqsub.s32       d20, d31, d28    // t60a
        vqadd.s32       d23, d31, d28    // t63a
        vqsub.s32       d21, d30, d29    // t61
        vqadd.s32       d22, d30, d29    // t62

.irp r, q8, q9, q10, q11
        vmin.s32        \r,  \r,  q5
.endr
.irp r, q8, q9, q10, q11
        vmax.s32        \r,  \r,  q4
.endr

        vmul_vmla       d4,  d21, d18, d1[0], d1[1] // -> t61a
        vmul_vmls       d6,  d21, d18, d1[1], d1[0] // -> t34a
        vmul_vmla       d7,  d20, d19, d1[0], d1[1] // -> t60
        vrshr.s32       d21, d4,  #12               // t61a
        vrshr.s32       d18, d6,  #12               // t34a
        vmul_vmls       d4,  d20, d19, d1[1], d1[0] // -> t35
        vrshr.s32       d20, d7,  #12               // t60
        vrshr.s32       d19, d4,  #12               // t35

        vst1.32         {d16, d17, d18, d19}, [r6, :128]!
        vst1.32         {d20, d21, d22, d23}, [r6, :128]!

        bx              lr
endfunc

function inv_dct64_step2_neon
        movrel_local    r12, idct_coeffs
        vld1.32         {q0}, [r12, :128]
1:
        // t32a/33/34a/35/60/61a/62/63a
        // t56a/57/58a/59/36/37a/38/39a
        // t40a/41/42a/43/52/53a/54/55a
        // t48a/49/50a/51/44/45a/46/47a
        vldr            d16, [r6, #4*2*0]  // t32a
        vldr            d17, [r9, #4*2*8]  // t39a
        vldr            d18, [r9, #4*2*0]  // t63a
        vldr            d19, [r6, #4*2*8]  // t56a
        vldr            d20, [r6, #4*2*16] // t40a
        vldr            d21, [r9, #4*2*24] // t47a
        vldr            d22, [r9, #4*2*16] // t55a
        vldr            d23, [r6, #4*2*24] // t48a

        vqadd.s32       d24, d16, d17      // t32
        vqsub.s32       d25, d16, d17      // t39
        vqadd.s32       d26, d18, d19      // t63
        vqsub.s32       d27, d18, d19      // t56
        vqsub.s32       d28, d21, d20      // t40
        vqadd.s32       d29, d21, d20      // t47
        vqadd.s32       d30, d23, d22      // t48
        vqsub.s32       d31, d23, d22      // t55

.irp r, q12, q13, q14, q15
        vmin.s32        \r,  \r,  q5
.endr
.irp r, q12, q13, q14, q15
        vmax.s32        \r,  \r,  q4
.endr

        vmul_vmla       d4,  d27, d25, d1[1], d1[0] // -> t56a
        vmul_vmls       d6,  d27, d25, d1[0], d1[1] // -> t39a
        vmul_vmla       d7,  d31, d28, d1[1], d1[0] // -> t40a
        vrshr.s32       d25, d4,  #12               // t56a
        vrshr.s32       d27, d6,  #12               // t39a
        vneg.s32        d7,  d7                     // t40a
        vmul_vmls       d4,  d31, d28, d1[0], d1[1] // -> t55a
        vrshr.s32       d31, d7,  #12               // t40a
        vrshr.s32       d28, d4,  #12               // t55a

        vqadd.s32       d16, d24, d29      // t32a
        vqsub.s32       d19, d24, d29      // t47a
        vqadd.s32       d17, d27, d31      // t39
        vqsub.s32       d18, d27, d31      // t40
        vqsub.s32       d20, d26, d30      // t48a
        vqadd.s32       d23, d26, d30      // t63a
        vqsub.s32       d21, d25, d28      // t55
        vqadd.s32       d22, d25, d28      // t56

.irp r, q8, q9, q10, q11
        vmin.s32        \r,  \r,  q5
.endr
.irp r, q8, q9, q10, q11
        vmax.s32        \r,  \r,  q4
.endr

        vmul_vmls       d4,  d21, d18, d0[0], d0[0] // -> t40a
        vmul_vmla       d6,  d21, d18, d0[0], d0[0] // -> t55a
        vmul_vmls       d7,  d20, d19, d0[0], d0[0] // -> t47
        vrshr.s32       d18, d4,  #12               // t40a
        vrshr.s32       d21, d6,  #12               // t55a
        vmul_vmla       d4,  d20, d19, d0[0], d0[0] // -> t48
        vrshr.s32       d19, d7,  #12               // t47
        vrshr.s32       d20, d4,  #12               // t48

        vstr            d16, [r6, #4*2*0]  // t32a
        vstr            d17, [r9, #4*2*0]  // t39
        vstr            d18, [r6, #4*2*8]  // t40a
        vstr            d19, [r9, #4*2*8]  // t47
        vstr            d20, [r6, #4*2*16] // t48
        vstr            d21, [r9, #4*2*16] // t55a
        vstr            d22, [r6, #4*2*24] // t56
        vstr            d23, [r9, #4*2*24] // t63a

        add             r6,  r6,  #4*2
        sub             r9,  r9,  #4*2
        cmp             r6,  r9
        blt             1b
        bx              lr
endfunc

.macro load8 src, strd, zero, clear
.irp i, d16, d17, d18, d19, d20, d21, d22, d23
.if \clear
        vld1.32         {\i}, [\src, :64]
        vst1.32         {\zero}, [\src, :64], \strd
.else
        vld1.32         {\i}, [\src, :64], \strd
.endif
.endr
.endm

.macro store16 dst
        vst1.32         {q8,  q9},  [\dst, :128]!
        vst1.32         {q10, q11}, [\dst, :128]!
        vst1.32         {q12, q13}, [\dst, :128]!
        vst1.32         {q14, q15}, [\dst, :128]!
.endm

.macro clear_upper8
.irp i, q12, q13, q14, q15
        vmov.i32        \i,  #0
.endr
.endm

.macro vmov_if reg, val, cond
.if \cond
        vmov.i32        \reg, \val
.endif
.endm

.macro movdup_if reg, gpr, val, cond
.if \cond
        mov_const       \gpr, \val
        vdup.32         \reg, \gpr
.endif
.endm

.macro vst1_if regs, dst, dstalign, cond
.if \cond
        vst1.32         \regs, \dst, \dstalign
.endif
.endm

.macro scale_if cond, c, r0, r1, r2, r3, r4, r5, r6, r7
.if \cond
        scale_input     \c, \r0, \r1, \r2, \r3, \r4, \r5, \r6, \r7
.endif
.endm

.macro def_dct64_func suffix, clear=0, scale=0
function inv_txfm_dct\suffix\()_2s_x64_neon
        mov             r6,  sp

        push            {r10-r11,lr}

        lsl             r8,  r8,  #2

        movdup_if       d0,  r12, 2896*8*(1<<16), \scale
        vmov_if         d7,  #0,  \clear
        load8           r7,  r8,  d7,  \clear
        clear_upper8
        sub             r7,  r7,  r8, lsl #3
        add             r7,  r7,  r8, lsr #1
        scale_if        \scale, d0[0], q8, q9, q10, q11

        bl              inv_dct_2s_x16_neon

        // idct_16 leaves the row_clip_max/min constants in d9 and d8,
        // but here we want to use full q registers for clipping.
        vmov.i32        q3,  #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        q2,  #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000
.irp r, q8, q9, q10, q11, q12, q13, q14, q15
        vmin.s32        \r,  \r,  q3
.endr
.irp r, q8, q9, q10, q11, q12, q13, q14, q15
        vmax.s32        \r,  \r,  q2
.endr

        store16         r6

        movdup_if       d0,  r12, 2896*8*(1<<16), \scale
        vmov_if         d7,  #0,  \clear
        load8           r7,  r8,  d7,  \clear
        clear_upper8
        sub             r7,  r7,  r8, lsl #3
        lsr             r8,  r8,  #1
        sub             r7,  r7,  r8, lsr #1
        scale_if        \scale, d0[0], q8, q9, q10, q11

        bl              inv_dct32_odd_2s_x16_neon

        add             r10, r6,  #8*15
        sub             r6,  r6,  #8*16

        mov             r9,  #-8

        vmov.i32        d1,  #0x1ffff // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        d0,  #0x1ffff // row_clip_min = (~bdmax << 7), 0xfffe0000
.macro store_addsub r0, r1, r2, r3
        vld1.32         {d2},  [r6, :64]!
        vld1.32         {d3},  [r6, :64]!
        vqadd.s32       d6,  d2,  \r0
        vqsub.s32       \r0, d2,  \r0
        vld1.32         {d4},  [r6, :64]!
        vqadd.s32       d7,  d3,  \r1
        vqsub.s32       \r1, d3,  \r1
        vmin.s32        d6,  d6,  d1
        vmin.s32        \r0, \r0, d1
        vld1.32         {d5},  [r6, :64]!
        vqadd.s32       d2,  d4,  \r2
        sub             r6,  r6,  #8*4
        vmax.s32        d6,  d6,  d0
        vmax.s32        \r0, \r0, d0
        vqsub.s32       \r2, d4,  \r2
        vmin.s32        d7,  d7,  d1
        vmin.s32        \r1, \r1, d1
        vst1.32         {d6},  [r6,  :64]!
        vst1.32         {\r0}, [r10, :64], r9
        vmin.s32        d2,  d2,  d1
        vmin.s32        \r2, \r2, d1
        vmax.s32        d7,  d7,  d0
        vmax.s32        \r1, \r1, d0
        vqadd.s32       d3,  d5,  \r3
        vqsub.s32       \r3, d5,  \r3
        vmax.s32        d2,  d2,  d0
        vmax.s32        \r2, \r2, d0
        vmin.s32        d3,  d3,  d1
        vmin.s32        \r3, \r3, d1
        vst1.32         {d7},  [r6,  :64]!
        vst1.32         {\r1}, [r10, :64], r9
        vmax.s32        d3,  d3,  d0
        vmax.s32        \r3, \r3, d0
        vst1.32         {d2},  [r6,  :64]!
        vst1.32         {\r2}, [r10, :64], r9
        vst1.32         {d3},  [r6,  :64]!
        vst1.32         {\r3}, [r10, :64], r9
.endm
        store_addsub    d31, d30, d29, d28
        store_addsub    d27, d26, d25, d24
        store_addsub    d23, d22, d21, d20
        store_addsub    d19, d18, d17, d16
.purgem store_addsub

        add             r6,  r6,  #2*4*16

        movrel_local    r12, idct64_coeffs
        vmov.i32        q5,  #0x1ffff        // row_clip_max = ~(~bdmax << 7), 0x1ffff
        vmvn.i32        q4,  #0x1ffff        // row_clip_min = (~bdmax << 7), 0xfffe0000
        movdup_if       d0,  lr,  2896*8*(1<<16), \scale
        vmov_if         d7,  #0,  \clear
        add             r9,  r7,  r8, lsl #4 // offset 16
        add             r10, r7,  r8, lsl #3 // offset 8
        sub             r9,  r9,  r8         // offset 15
        sub             r11, r10, r8         // offset 7
        vld1.32         {d16}, [r7,  :64]    // in1  (offset 0)
        vld1.32         {d17}, [r9,  :64]    // in31 (offset 15)
        vld1.32         {d18}, [r10, :64]    // in17 (offset 8)
        vld1.32         {d19}, [r11, :64]    // in15 (offset 7)
        vst1_if         {d7},  [r7,  :64], \clear
        vst1_if         {d7},  [r9,  :64], \clear
        vst1_if         {d7},  [r10, :64], \clear
        vst1_if         {d7},  [r11, :64], \clear
        scale_if        \scale, d0[0], q8, q9
        bl              inv_dct64_step1_neon
        movdup_if       d0,  lr,  2896*8*(1<<16), \scale
        vmov_if         d7,  #0,  \clear
        add             r7,  r7,  r8, lsl #2 // offset 4
        sub             r9,  r9,  r8, lsl #2 // offset 11
        sub             r10, r7,  r8         // offset 3
        add             r11, r9,  r8         // offset 12
        vld1.32         {d16}, [r10, :64]    // in7  (offset 3)
        vld1.32         {d17}, [r11, :64]    // in25 (offset 12)
        vld1.32         {d18}, [r9,  :64]    // in23 (offset 11)
        vld1.32         {d19}, [r7,  :64]    // in9  (offset 4)
        vst1_if         {d7},  [r7,  :64], \clear
        vst1_if         {d7},  [r9,  :64], \clear
        vst1_if         {d7},  [r10, :64], \clear
        vst1_if         {d7},  [r11, :64], \clear
        scale_if        \scale, d0[0], q8, q9
        bl              inv_dct64_step1_neon
        movdup_if       d0,  lr,  2896*8*(1<<16), \scale
        vmov_if         d7,  #0,  \clear
        sub             r10, r10, r8, lsl #1 // offset 1
        sub             r9,  r9,  r8, lsl #1 // offset 9
        add             r10, r10, r8         // offset 2
        add             r9,  r9,  r8         // offset 10
        add             r7,  r7,  r8         // offset 5
        add             r11, r11, r8         // offset 13
        vld1.32         d16, [r10, :64]      // in5  (offset 2)
        vld1.32         d17, [r11, :64]      // in27 (offset 13)
        vld1.32         d18, [r9,  :64]      // in21 (offset 10)
        vld1.32         d19, [r7,  :64]      // in11 (offset 5)
        vst1_if         d7,  [r10, :64], \clear
        vst1_if         d7,  [r11, :64], \clear
        vst1_if         d7,  [r9,  :64], \clear
        vst1_if         d7,  [r7,  :64], \clear
        scale_if        \scale, d0[0], q8, q9
        bl              inv_dct64_step1_neon
        movdup_if       d0,  lr,  2896*8*(1<<16), \scale
        vmov_if         d7,  #0,  \clear
        sub             r10, r10, r8         // offset 1
        sub             r9,  r9,  r8         // offset 9
        add             r11, r11, r8         // offset 14
        add             r7,  r7,  r8         // offset 6
        vld1.32         d16, [r10, :64]      // in3  (offset 1)
        vld1.32         d17, [r11, :64]      // in29 (offset 14)
        vld1.32         d18, [r9,  :64]      // in19 (offset 9)
        vld1.32         d19, [r7,  :64]      // in13 (offset 6)
        vst1_if         d7,  [r10, :64], \clear
        vst1_if         d7,  [r11, :64], \clear
        vst1_if         d7,  [r9,  :64], \clear
        vst1_if         d7,  [r7,  :64], \clear
        scale_if        \scale, d0[0], q8, q9
        bl              inv_dct64_step1_neon

        sub             r6,  r6,  #2*4*32
        add             r9,  r6,  #2*4*7

        bl              inv_dct64_step2_neon

        pop             {r10-r11,pc}
endfunc
.endm

def_dct64_func _clear, clear=1
def_dct64_func _clear_scale, clear=1, scale=1

function inv_txfm_horz_dct_64x2_neon
        vdup.32         q4,  r9

        mov             r7,  sp
        add             r8,  sp,  #2*4*(64 - 4)
        add             r9,  r6,  #2*56

        push            {r10-r11,lr}

        mov             r10, #2*64
        mov             r11, #-2*4*4

1:
        vld1.32         {d16, d17, d18, d19}, [r7, :128]!
        vld1.32         {d28, d29, d30, d31}, [r8, :128], r11
        vld1.32         {d20, d21, d22, d23}, [r7, :128]!
        vld1.32         {d24, d25, d26, d27}, [r8, :128], r11
        vtrn.32         d16, d17
        vtrn.32         d18, d19
        vtrn.32         d20, d21
        vtrn.32         d22, d23
        vtrn.32         d31, d30
        vtrn.32         d29, d28
        vtrn.32         d27, d26
        vtrn.32         d25, d24

.macro store_addsub src0, src1, src2, src3, src4, src5, src6, src7
        vqsub.s32       d7,  \src0,  \src1
        vqsub.s32       d6,  \src2,  \src3
        vqsub.s32       d5,  \src4,  \src5
        vqsub.s32       d4,  \src6,  \src7
        vqadd.s32       d0,  \src0,  \src1
        vqadd.s32       d1,  \src2,  \src3
        vqadd.s32       d2,  \src4,  \src5
        vqadd.s32       d3,  \src6,  \src7
        vrshl.s32       q3,  q3,  q4
        vrshl.s32       q2,  q2,  q4
        vrshl.s32       q0,  q0,  q4
        vrshl.s32       q1,  q1,  q4
        vqmovn.s32      d7,  q3
        vqmovn.s32      d6,  q2
        vqmovn.s32      d0,  q0
        vqmovn.s32      d1,  q1
        vrev32.16       q3,  q3
        vst1.16         {q0},  [r6, :128], r10
        vst1.16         {q3},  [r9, :128], r10
.endm
        store_addsub    d16, d31, d18, d29, d20, d27, d22, d25
        store_addsub    d17, d30, d19, d28, d21, d26, d23, d24
.purgem store_addsub
        sub             r6,  r6,  r10, lsl #1
        sub             r9,  r9,  r10, lsl #1
        add             r6,  r6,  #16
        sub             r9,  r9,  #16

        cmp             r7,  r8
        blt             1b
        pop             {r10-r11,pc}
endfunc

function inv_txfm_add_vert_dct_4x64_neon
        lsl             r8,  r8,  #1

        mov             r7,  sp
        add             r8,  sp,  #2*4*(64 - 4)
        add             r9,  r6,  r1, lsl #6
        sub             r9,  r9,  r1

        push            {r10-r11,lr}

        neg             r10, r1
        mov             r11, #-2*4*4

1:
        vld1.16         {d16, d17, d18, d19}, [r7, :128]!
        vld1.16         {d28, d29, d30, d31}, [r8, :128], r11
        vld1.16         {d20, d21, d22, d23}, [r7, :128]!
        vld1.16         {d24, d25, d26, d27}, [r8, :128], r11

        vmov.i16        q6,  #0
        vmvn.i16        q7,  #0xfc00 // 0x3ff
.macro add_dest_addsub src0, src1, src2, src3
        vld1.16         {d0}, [r6, :64], r1
        vld1.16         {d1}, [r9, :64], r10
        vqadd.s16       d4,  \src0,  \src1
        vld1.16         {d2}, [r6, :64]
        vqsub.s16       d5,  \src0,  \src1
        vld1.16         {d3}, [r9, :64]
        vqadd.s16       d6,  \src2,  \src3
        vqsub.s16       d7,  \src2,  \src3
        sub             r6,  r6,  r1
        sub             r9,  r9,  r10
        vrshr.s16       q2,  q2,  #4
        vrshr.s16       q3,  q3,  #4
        vqadd.s16       q2,  q2,  q0
        vqadd.s16       q3,  q3,  q1
        vmax.s16        q2,  q2,  q6
        vmax.s16        q3,  q3,  q6
        vmin.s16        q2,  q2,  q7
        vmin.s16        q3,  q3,  q7
        vst1.16         {d4}, [r6, :64], r1
        vst1.16         {d5}, [r9, :64], r10
        vst1.16         {d6}, [r6, :64], r1
        vst1.16         {d7}, [r9, :64], r10
.endm
        add_dest_addsub d16, d31, d17, d30
        add_dest_addsub d18, d29, d19, d28
        add_dest_addsub d20, d27, d21, d26
        add_dest_addsub d22, d25, d23, d24
.purgem add_dest_addsub
        cmp             r7,  r8
        blt             1b

        pop             {r10-r11,pc}
endfunc

function inv_txfm_add_dct_dct_64x64_16bpc_neon, export=1
        idct_dc         64,  64,  2

        push            {r4-r11,lr}
        vpush           {q4-q7}

        sub_sp_align    64*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_32x32

.irp i, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
        add             r6,  r5,  #(\i*64*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #32*4
        bl              inv_txfm_dct_clear_2s_x64_neon
        add             r6,  r5,  #(\i*64*2)
        mov             r9,  #-2 // shift
        bl              inv_txfm_horz_dct_64x2_neon
.if \i < 30
        ldrh            r11, [r10], #2
.endif
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 8
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60
        add             r7,  r5,  #(\i*2)
        mov             r8,  #64*2
        bl              X(inv_txfm_dct_4h_x64_neon)
        add             r6,  r0,  #(\i*2)
        bl              inv_txfm_add_vert_dct_4x64_neon
.endr

        add_sp_align    64*32*2+64*4*2
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_64x32_16bpc_neon, export=1
        idct_dc         64,  32,  1

        push            {r4-r11,lr}
        vpush           {q4-q7}

        sub_sp_align    64*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_32x32

.irp i, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
        add             r6,  r5,  #(\i*64*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #32*4
        bl              inv_txfm_dct_clear_scale_2s_x64_neon
        add             r6,  r5,  #(\i*64*2)
        mov             r9,  #-1 // shift
        bl              inv_txfm_horz_dct_64x2_neon
.if \i < 30
        ldrh            r11, [r10], #2
.endif
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 8
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60
        add             r6,  r0,  #(\i*2)
        add             r7,  r5,  #(\i*2)
        mov             r8,  #64*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    64*32*2+64*4*2
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_32x64_16bpc_neon, export=1
        idct_dc         32,  64,  1

        push            {r4-r11,lr}
        vpush           {q4-q7}

        sub_sp_align    32*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_32x32
        ldrh            r11, [r10], #2

.irp i, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
        add             r6,  r5,  #(\i*32*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 30
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #32*4
        bl              inv_txfm_horz_scale_dct_32x2_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r7,  r5,  #(\i*2)
        mov             r8,  #32*2
        bl              X(inv_txfm_dct_4h_x64_neon)
        add             r6,  r0,  #(\i*2)
        bl              inv_txfm_add_vert_dct_4x64_neon
.endr

        add_sp_align    32*32*2+64*4*2
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_64x16_16bpc_neon, export=1
        idct_dc         64,  16,  2

        push            {r4-r11,lr}
        vpush           {q4-q7}

        sub_sp_align    64*16*2+64*4*2
        add             r4,  sp,  #64*4*2

        movrel_local    r10, eob_16x32

.irp i, 0, 2, 4, 6, 8, 10, 12, 14
        add             r6,  r4,  #(\i*64*2)
.if \i > 0
        mov             r8,  #(16 - \i)
        cmp             r3,  r11
        blt             1f
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #16*4
        bl              inv_txfm_dct_clear_2s_x64_neon
        add             r6,  r4,  #(\i*64*2)
        mov             r9,  #-2 // shift
        bl              inv_txfm_horz_dct_64x2_neon
.if \i < 8
        ldrh            r11, [r10], #2
.endif
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 8
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
        movrel          r5,  X(inv_dct_4h_x16_neon)
.irp i, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60
        add             r6,  r0,  #(\i*2)
        add             r7,  r4,  #(\i*2)
        mov             r8,  #64*2
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    64*16*2+64*4*2
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_16x64_16bpc_neon, export=1
        idct_dc         16,  64,  2

        push            {r4-r11,lr}
        vpush           {q4-q7}

        sub_sp_align    16*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_16x32
        ldrh            r11, [r10], #2

        movrel_local    r4,  inv_dct_2s_x16_neon
.irp i, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
        add             r6,  r5,  #(\i*16*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 30
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*4)
        mov             r8,  #32*4
        bl              inv_txfm_horz_16x2_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #2
.rept 2
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12
        add             r7,  r5,  #(\i*2)
        mov             r8,  #16*2
        bl              X(inv_txfm_dct_4h_x64_neon)
        add             r6,  r0,  #(\i*2)
        bl              inv_txfm_add_vert_dct_4x64_neon
.endr

        add_sp_align    16*32*2+64*4*2
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc
