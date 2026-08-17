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

        .short          4036*8, -700*8, 2359*8, 3349*8
        .short          3461*8, -2191*8, 897*8, 3996*8
        .short          -3166, -2598, -799, -4017

        .short          501*8, 4065*8, 3229*8, -2520*8
        .short          2019*8, 3564*8, 3948*8, -1092*8
        .short          3612, 1931, 2276, 3406

        .short          4085*8, -301*8, 2675*8, 3102*8
        .short          3659*8, -1842*8, 1285*8, 3889*8
        .short          -3920, -1189, -3406, -2276
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

.macro vmull_vmlal d0, s0, s1, c0, c1
        vmull.s16       \d0, \s0, \c0
        vmlal.s16       \d0, \s1, \c1
.endm

.macro vmull_vmlal_8h d0, d1, s0, s1, s2, s3, c0, c1
        vmull.s16       \d0, \s0, \c0
        vmlal.s16       \d0, \s2, \c1
        vmull.s16       \d1, \s1, \c0
        vmlal.s16       \d1, \s3, \c1
.endm

.macro vmull_vmlsl d0, s0, s1, c0, c1
        vmull.s16       \d0, \s0, \c0
        vmlsl.s16       \d0, \s1, \c1
.endm

.macro vmull_vmlsl_8h d0, d1, s0, s1, s2, s3, c0, c1
        vmull.s16       \d0, \s0, \c0
        vmlsl.s16       \d0, \s2, \c1
        vmull.s16       \d1, \s1, \c0
        vmlsl.s16       \d1, \s3, \c1
.endm

.macro vqrshrn_8h d0, d1, s0, s1, shift
        vqrshrn.s32     \d0, \s0, \shift
        vqrshrn.s32     \d1, \s1, \shift
.endm

.macro scale_input c, r0, r1, r2 r3, r4, r5, r6, r7
        vqrdmulh.s16    \r0, \r0, \c
        vqrdmulh.s16    \r1, \r1, \c
.ifnb \r2
        vqrdmulh.s16    \r2, \r2, \c
        vqrdmulh.s16    \r3, \r3, \c
.endif
.ifnb \r4
        vqrdmulh.s16    \r4, \r4, \c
        vqrdmulh.s16    \r5, \r5, \c
        vqrdmulh.s16    \r6, \r6, \c
        vqrdmulh.s16    \r7, \r7, \c
.endif
.endm

.macro load_add_store load, shift, addsrc, adddst, narrowsrc, narrowdst, store, dst, src, shiftbits=4
.ifnb \load
        vld1.8          {\load},  [\src, :64], r1
.endif
.ifnb \shift
        vrshr.s16       \shift,  \shift,  #\shiftbits
.endif
.ifnb \addsrc
        vaddw.u8        \adddst, \adddst, \addsrc
.endif
.ifnb \narrowsrc
        vqmovun.s16     \narrowdst, \narrowsrc
.endif
.ifnb \store
        vst1.8          {\store},  [\dst, :64], r1
.endif
.endm
.macro load_add_store_8x8 dst, src, shiftbits=4
        mov             \src, \dst
        load_add_store  d2,  q8,    ,    ,    ,    ,    , \dst, \src, \shiftbits
        load_add_store  d3,  q9,    ,    ,    ,    ,    , \dst, \src, \shiftbits
        load_add_store  d4,  q10, d2,  q8,    ,    ,    , \dst, \src, \shiftbits
        load_add_store  d5,  q11, d3,  q9,  q8,  d2,    , \dst, \src, \shiftbits
        load_add_store  d6,  q12, d4,  q10, q9,  d3,  d2, \dst, \src, \shiftbits
        load_add_store  d7,  q13, d5,  q11, q10, d4,  d3, \dst, \src, \shiftbits
        load_add_store  d2,  q14, d6,  q12, q11, d5,  d4, \dst, \src, \shiftbits
        load_add_store  d3,  q15, d7,  q13, q12, d6,  d5, \dst, \src, \shiftbits
        load_add_store    ,     , d2,  q14, q13, d7,  d6, \dst, \src, \shiftbits
        load_add_store    ,     , d3,  q15, q14, d2,  d7, \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     , q15, d3,  d2, \dst, \src, \shiftbits
        load_add_store    ,     ,   ,     ,    ,   ,  d3, \dst, \src, \shiftbits
.endm
.macro load_add_store_8x4 dst, src
        mov             \src, \dst
        load_add_store  d2,  q8,    ,    ,    ,    ,    ,  \dst, \src
        load_add_store  d3,  q9,    ,    ,    ,    ,    ,  \dst, \src
        load_add_store  d4,  q10, d2,  q8,    ,    ,    ,  \dst, \src
        load_add_store  d5,  q11, d3,  q9,  q8,  d2,    ,  \dst, \src
        load_add_store    ,     , d4,  q10, q9,  d3,  d2,  \dst, \src
        load_add_store    ,     , d5,  q11, q10, d4,  d3,  \dst, \src
        load_add_store    ,     ,   ,     , q11, d5,  d4,  \dst, \src
        load_add_store    ,     ,   ,     ,    ,   ,  d5,  \dst, \src
.endm
.macro load_add_store4 load, shift, addsrc, adddst, narrowsrc, narrowdst, store, dst, src
.ifnb \load
        vld1.32         {\load[0]},  [\src, :32], r1
.endif
.ifnb \shift
        vrshr.s16       \shift,  \shift,  #4
.endif
.ifnb \load
        vld1.32         {\load[1]},  [\src, :32], r1
.endif
.ifnb \addsrc
        vaddw.u8        \adddst, \adddst, \addsrc
.endif
.ifnb \store
        vst1.32         {\store[0]},  [\dst, :32], r1
.endif
.ifnb \narrowsrc
        vqmovun.s16     \narrowdst, \narrowsrc
.endif
.ifnb \store
        vst1.32         {\store[1]},  [\dst, :32], r1
.endif
.endm
.macro load_add_store_4x16 dst, src
        mov             \src, \dst
        load_add_store4 d0,    ,    ,    ,    ,    ,    ,  \dst, \src
        load_add_store4 d1,  q8,    ,    ,    ,    ,    ,  \dst, \src
        load_add_store4 d2,  q9,  d0,  q8,    ,    ,    ,  \dst, \src
        load_add_store4 d3,  q10, d1,  q9,  q8,  d0,    ,  \dst, \src
        load_add_store4 d4,  q11, d2,  q10, q9,  d1,  d0,  \dst, \src
        load_add_store4 d5,  q12, d3,  q11, q10, d2,  d1,  \dst, \src
        load_add_store4 d6,  q13, d4,  q12, q11, d3,  d2,  \dst, \src
        load_add_store4 d7,  q14, d5,  q13, q12, d4,  d3,  \dst, \src
        load_add_store4   ,  q15, d6,  q14, q13, d5,  d4,  \dst, \src
        load_add_store4   ,     , d7,  q15, q14, d6,  d5,  \dst, \src
        load_add_store4   ,     ,   ,     , q15, d7,  d6,  \dst, \src
        load_add_store4   ,     ,   ,     ,    ,   ,  d7,  \dst, \src
.endm
.macro load_add_store_4x8 dst, src
        mov             \src, \dst
        load_add_store4 d0,    ,    ,    ,    ,    ,    ,  \dst, \src
        load_add_store4 d1,  q8,    ,    ,    ,    ,    ,  \dst, \src
        load_add_store4 d2,  q9,  d0,  q8,    ,    ,    ,  \dst, \src
        load_add_store4 d3,  q10, d1,  q9,  q8,  d0,    ,  \dst, \src
        load_add_store4   ,  q11, d2,  q10, q9,  d1,  d0,  \dst, \src
        load_add_store4   ,     , d3,  q11, q10, d2,  d1,  \dst, \src
        load_add_store4   ,     ,   ,     , q11, d3,  d2,  \dst, \src
        load_add_store4   ,     ,   ,     ,    ,   ,  d3,  \dst, \src
.endm

.macro idct_dc w, h, shift
        cmp             r3,  #0
        bne             1f
        vmov.i16        d30, #0
        movw            r12, #2896*8
        vld1.16         {d16[]},  [r2, :16]
        vdup.16         d0,  r12
        vqrdmulh.s16    d16, d16, d0[0]
        vst1.16         {d30[0]}, [r2, :16]
.if (\w == 2*\h) || (2*\w == \h)
        vqrdmulh.s16    d16, d16, d0[0]
.endif
.if \shift > 0
        vrshr.s16       d16, d16, #\shift
.endif
        vqrdmulh.s16    d20, d16, d0[0]
        mov             r3,  #\h
        vrshr.s16       d16, d20, #4
        vrshr.s16       d17, d20, #4
        b               idct_dc_w\w\()_neon
1:
.endm

function idct_dc_w4_neon
1:
        vld1.32         {d0[0]}, [r0, :32], r1
        vld1.32         {d0[1]}, [r0, :32], r1
        vld1.32         {d1[0]}, [r0, :32], r1
        vld1.32         {d1[1]}, [r0, :32], r1
        subs            r3,  r3,  #4
        sub             r0,  r0,  r1, lsl #2
        vaddw.u8        q10, q8,  d0
        vqmovun.s16     d0,  q10
        vaddw.u8        q11, q8,  d1
        vst1.32         {d0[0]}, [r0, :32], r1
        vqmovun.s16     d1,  q11
        vst1.32         {d0[1]}, [r0, :32], r1
        vst1.32         {d1[0]}, [r0, :32], r1
        vst1.32         {d1[1]}, [r0, :32], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w8_neon
1:
        vld1.8          {d0}, [r0, :64], r1
        vld1.8          {d1}, [r0, :64], r1
        vld1.8          {d2}, [r0, :64], r1
        vaddw.u8        q10, q8,  d0
        vld1.8          {d3}, [r0, :64], r1
        sub             r0,  r0,  r1, lsl #2
        subs            r3,  r3,  #4
        vaddw.u8        q11, q8,  d1
        vqmovun.s16     d0,  q10
        vaddw.u8        q12, q8,  d2
        vqmovun.s16     d1,  q11
        vaddw.u8        q13, q8,  d3
        vst1.8          {d0}, [r0, :64], r1
        vqmovun.s16     d2,  q12
        vst1.8          {d1}, [r0, :64], r1
        vqmovun.s16     d3,  q13
        vst1.8          {d2}, [r0, :64], r1
        vst1.8          {d3}, [r0, :64], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w16_neon
1:
        vld1.8          {q0}, [r0, :128], r1
        vld1.8          {q1}, [r0, :128], r1
        vld1.8          {q2}, [r0, :128], r1
        subs            r3,  r3,  #4
        vaddw.u8        q10, q8,  d0
        vaddw.u8        q11, q8,  d1
        vld1.8          {q3}, [r0, :128], r1
        vaddw.u8        q12, q8,  d2
        vaddw.u8        q13, q8,  d3
        sub             r0,  r0,  r1, lsl #2
        vaddw.u8        q14, q8,  d4
        vaddw.u8        q15, q8,  d5
        vqmovun.s16     d0,  q10
        vqmovun.s16     d1,  q11
        vaddw.u8        q10, q8,  d6
        vaddw.u8        q11, q8,  d7
        vqmovun.s16     d2,  q12
        vqmovun.s16     d3,  q13
        vqmovun.s16     d4,  q14
        vqmovun.s16     d5,  q15
        vst1.8          {q0}, [r0, :128], r1
        vqmovun.s16     d6,  q10
        vqmovun.s16     d7,  q11
        vst1.8          {q1}, [r0, :128], r1
        vst1.8          {q2}, [r0, :128], r1
        vst1.8          {q3}, [r0, :128], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w32_neon
1:
        vld1.8          {q0, q1}, [r0, :128], r1
        subs            r3,  r3,  #2
        vld1.8          {q2, q3}, [r0, :128], r1
        vaddw.u8        q10, q8,  d0
        vaddw.u8        q11, q8,  d1
        vaddw.u8        q12, q8,  d2
        vaddw.u8        q13, q8,  d3
        sub             r0,  r0,  r1, lsl #1
        vaddw.u8        q14, q8,  d4
        vaddw.u8        q15, q8,  d5
        vqmovun.s16     d0,  q10
        vqmovun.s16     d1,  q11
        vaddw.u8        q10, q8,  d6
        vaddw.u8        q11, q8,  d7
        vqmovun.s16     d2,  q12
        vqmovun.s16     d3,  q13
        vqmovun.s16     d4,  q14
        vqmovun.s16     d5,  q15
        vst1.8          {q0, q1}, [r0, :128], r1
        vqmovun.s16     d6,  q10
        vqmovun.s16     d7,  q11
        vst1.8          {q2, q3}, [r0, :128], r1
        bgt             1b
        bx              lr
endfunc

function idct_dc_w64_neon
        sub             r1,  r1,  #32
1:
        vld1.8          {q0, q1}, [r0, :128]!
        subs            r3,  r3,  #1
        vld1.8          {q2, q3}, [r0, :128]
        vaddw.u8        q10, q8,  d0
        vaddw.u8        q11, q8,  d1
        vaddw.u8        q12, q8,  d2
        vaddw.u8        q13, q8,  d3
        sub             r0,  r0,  #32
        vaddw.u8        q14, q8,  d4
        vaddw.u8        q15, q8,  d5
        vqmovun.s16     d0,  q10
        vqmovun.s16     d1,  q11
        vaddw.u8        q10, q8,  d6
        vaddw.u8        q11, q8,  d7
        vqmovun.s16     d2,  q12
        vqmovun.s16     d3,  q13
        vqmovun.s16     d4,  q14
        vqmovun.s16     d5,  q15
        vst1.8          {q0, q1}, [r0, :128]!
        vqmovun.s16     d6,  q10
        vqmovun.s16     d7,  q11
        vst1.8          {q2, q3}, [r0, :128], r1
        bgt             1b
        bx              lr
endfunc

.macro iwht4
        vadd.i16        d16, d16, d17
        vsub.i16        d21, d18, d19
        vsub.i16        d20, d16, d21
        vshr.s16        d20, d20, #1
        vsub.i16        d18, d20, d17
        vsub.i16        d17, d20, d19
        vadd.i16        d19, d21, d18
        vsub.i16        d16, d16, d17
.endm

.macro idct_4h_x4 r0, r1, r2, r3
        vmull_vmlal     q3,  \r1, \r3, d0[3], d0[2]
        vmull_vmlsl     q2,  \r1, \r3, d0[2], d0[3]
        vmull_vmlal     q1,  \r0, \r2, d0[0], d0[0]
        vqrshrn.s32     d6,  q3,  #12
        vqrshrn.s32     d7,  q2,  #12
        vmull_vmlsl     q2,  \r0, \r2, d0[0], d0[0]
        vqrshrn.s32     d2,  q1,  #12
        vqrshrn.s32     d3,  q2,  #12
        vqadd.s16       \r0, d2,  d6
        vqsub.s16       \r3, d2,  d6
        vqadd.s16       \r1, d3,  d7
        vqsub.s16       \r2, d3,  d7
.endm

.macro idct_8h_x4 q0, q1, q2, q3, r0, r1, r2, r3, r4, r5, r6, r7
        vmull_vmlal_8h  q6,  q7,  \r2, \r3, \r6, \r7, d0[3], d0[2]
        vmull_vmlsl_8h  q4,  q5,  \r2, \r3, \r6, \r7, d0[2], d0[3]
        vmull_vmlal_8h  q2,  q3,  \r0, \r1, \r4, \r5, d0[0], d0[0]
        vqrshrn_8h      d12, d13, q6,  q7,  #12
        vqrshrn_8h      d14, d15, q4,  q5,  #12
        vmull_vmlsl_8h  q4,  q5,  \r0, \r1, \r4, \r5, d0[0], d0[0]
        vqrshrn_8h      d4,  d5,  q2,  q3,  #12
        vqrshrn_8h      d6,  d7,  q4,  q5,  #12
        vqadd.s16       \q0, q2,  q6
        vqsub.s16       \q3, q2,  q6
        vqadd.s16       \q1, q3,  q7
        vqsub.s16       \q2, q3,  q7
.endm

function inv_dct_4h_x4_neon, export=1
        movrel_local    r12, idct_coeffs
        vld1.16         {d0}, [r12, :64]
        idct_4h_x4      d16, d17, d18, d19
        bx              lr
endfunc

function inv_dct_8h_x4_neon, export=1
        movrel_local    r12, idct_coeffs
        vld1.16         {d0}, [r12, :64]
        idct_8h_x4      q8,  q9,  q10, q11, d16, d17, d18, d19, d20, d21, d22, d23
        bx              lr
endfunc

.macro iadst_4x4 o0, o1, o2, o3
        movrel_local    r12, iadst4_coeffs
        vld1.16         {d0, d1}, [r12, :128]

        vsubl.s16       q1,  d16, d18
        vmull.s16       q2,  d16, d0[0]
        vmlal.s16       q2,  d18, d0[1]
        vmlal.s16       q2,  d19, d0[2]
        vmull.s16       q10, d17, d0[3]
        vaddw.s16       q1,  q1,  d19
        vmull.s16       q3,  d16, d0[2]
        vmlsl.s16       q3,  d18, d0[0]
        vmlsl.s16       q3,  d19, d0[1]

        vadd.s32        q11, q2,  q3
        vmul.s32        q1,  q1,  d1[0]
        vadd.s32        q2,  q2,  q10
        vadd.s32        q3,  q3,  q10
        vsub.s32        q11, q11, q10

        vqrshrn.s32     \o0, q2,  #12
        vqrshrn.s32     \o2, q1,  #12
        vqrshrn.s32     \o1, q3,  #12
        vqrshrn.s32     \o3, q11, #12
.endm

function inv_adst_4h_x4_neon, export=1
        iadst_4x4       d16, d17, d18, d19
        bx              lr
endfunc

function inv_flipadst_4h_x4_neon, export=1
        iadst_4x4       d19, d18, d17, d16
        bx              lr
endfunc

.macro iadst_8x4 o0, o1, o2, o3, o4, o5, o6, o7
        movrel_local    r12, iadst4_coeffs
        vld1.16         {d0, d1}, [r12, :128]

        vsubl.s16       q2,  d16, d20
        vsubl.s16       q3,  d17, d21
        vmull.s16       q4,  d16, d0[0]
        vmlal.s16       q4,  d20, d0[1]
        vmlal.s16       q4,  d22, d0[2]
        vmull.s16       q5,  d17, d0[0]
        vmlal.s16       q5,  d21, d0[1]
        vmlal.s16       q5,  d23, d0[2]
        vaddw.s16       q2,  q2,  d22
        vaddw.s16       q3,  q3,  d23
        vmull.s16       q6,  d16, d0[2]
        vmlsl.s16       q6,  d20, d0[0]
        vmlsl.s16       q6,  d22, d0[1]
        vmull.s16       q7,  d17, d0[2]
        vmlsl.s16       q7,  d21, d0[0]
        vmlsl.s16       q7,  d23, d0[1]

        vmul.s32        q10, q2,  d1[0]
        vmul.s32        q11, q3,  d1[0]

        vmull.s16       q2,  d18, d0[3]
        vmull.s16       q3,  d19, d0[3]

        vadd.s32        q8,  q4,  q2 // out0
        vadd.s32        q9,  q5,  q3

        vadd.s32        q4,  q4,  q6 // out3
        vadd.s32        q5,  q5,  q7

        vadd.s32        q6,  q6,  q2 // out1
        vadd.s32        q7,  q7,  q3

        vsub.s32        q4,  q4,  q2 // out3
        vsub.s32        q5,  q5,  q3

        vqrshrn.s32     d20, q10, #12
        vqrshrn.s32     d21, q11, #12

        vqrshrn.s32     \o0, q8,  #12
        vqrshrn.s32     \o1, q9,  #12

.ifc \o4, d18
        vmov            q9,  q10
.endif

        vqrshrn.s32     \o2, q6,  #12
        vqrshrn.s32     \o3, q7,  #12

        vqrshrn.s32     \o6, q4,  #12
        vqrshrn.s32     \o7, q5,  #12
.endm

function inv_adst_8h_x4_neon, export=1
        iadst_8x4       d16, d17, d18, d19, d20, d21, d22, d23
        bx              lr
endfunc

function inv_flipadst_8h_x4_neon, export=1
        iadst_8x4       d22, d23, d20, d21, d18, d19, d16, d17
        bx              lr
endfunc

function inv_identity_4h_x4_neon, export=1
        movw            r12, #(5793-4096)*8
        vdup.16         d0,  r12
        vqrdmulh.s16    q2,  q8,  d0[0]
        vqrdmulh.s16    q3,  q9,  d0[0]
        vqadd.s16       q8,  q8,  q2
        vqadd.s16       q9,  q9,  q3
        bx              lr
endfunc

function inv_identity_8h_x4_neon, export=1
        movw            r12, #(5793-4096)*8
        vdup.16         d0,  r12
        vqrdmulh.s16    q1,  q8,  d0[0]
        vqrdmulh.s16    q2,  q9,  d0[0]
        vqrdmulh.s16    q3,  q10, d0[0]
        vqadd.s16       q8,  q8,  q1
        vqrdmulh.s16    q1,  q11, d0[0]
        vqadd.s16       q9,  q9,  q2
        vqadd.s16       q10, q10, q3
        vqadd.s16       q11, q11, q1
        bx              lr
endfunc

.macro identity_8x4_shift1 r0, r1, r2, r3, c
.irp i, \r0, \r1, \r2, \r3
        vqrdmulh.s16    q1,  \i,  \c
        vrhadd.s16      \i,  \i,  q1
.endr
.endm

function inv_txfm_add_wht_wht_4x4_8bpc_neon, export=1
        push            {r4-r5,lr}
        vmov.i16        q15, #0
        vld1.16         {d16, d17, d18, d19}, [r2, :128]
        vst1.16         {q15}, [r2, :128]!

        vshr.s16        q8,  q8,  #2
        vshr.s16        q9,  q9,  #2

        iwht4

        vst1.16         {q15}, [r2, :128]!
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19

        iwht4

        vld1.32         {d0[]},  [r0, :32], r1
        vld1.32         {d0[1]}, [r0, :32], r1
        vld1.32         {d1[]},  [r0, :32], r1
        vld1.32         {d1[1]}, [r0, :32], r1

        b               L(itx_4x4_end)
endfunc

function inv_txfm_add_4x4_neon
        vmov.i16        q15, #0
        vld1.16         {d16, d17, d18, d19}, [r2, :128]
        vst1.16         {q15}, [r2, :128]!

        blx             r4

        vst1.16         {q15}, [r2, :128]!
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19

        blx             r5

        vld1.32         {d0[]},  [r0, :32], r1
        vld1.32         {d0[1]}, [r0, :32], r1
        vld1.32         {d1[]},  [r0, :32], r1
        vld1.32         {d1[1]}, [r0, :32], r1
        vrshr.s16       q8,  q8,  #4
        vrshr.s16       q9,  q9,  #4

L(itx_4x4_end):
        sub             r0,  r0,  r1, lsl #2
        vaddw.u8        q8,  q8,  d0
        vqmovun.s16     d0,  q8
        vaddw.u8        q9,  q9,  d1
        vst1.32         {d0[0]}, [r0, :32], r1
        vqmovun.s16     d1,  q9
        vst1.32         {d0[1]}, [r0, :32], r1
        vst1.32         {d1[0]}, [r0, :32], r1
        vst1.32         {d1[1]}, [r0, :32], r1

        pop             {r4-r5,pc}
endfunc

.macro def_fn_4x4 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_4x4_8bpc_neon, export=1
        push            {r4-r5,lr}

.ifc \txfm1\()_\txfm2, dct_dct
        cmp             r3,  #0
        bne             1f
        vmov.i16        d30, #0
        movw            r12, #2896*8
        vld1.16         {d16[]},  [r2, :16]
        vdup.16         d4,  r12
        vst1.16         {d30[0]}, [r2, :16]
        vqrdmulh.s16    d16, d16, d4[0]
        vld1.32         {d0[0]},  [r0, :32], r1
        vqrdmulh.s16    d20, d16, d4[0]
        vld1.32         {d0[1]},  [r0, :32], r1
        vrshr.s16       d16, d20, #4
        vrshr.s16       d17, d20, #4
        vld1.32         {d1[0]},  [r0, :32], r1
        vmov            q9,  q8
        vld1.32         {d1[1]}, [r0, :32], r1
        b               L(itx_4x4_end)
1:
.endif
        movrel_local    r4,  inv_\txfm1\()_4h_x4_neon
        movrel_local    r5,  inv_\txfm2\()_4h_x4_neon
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

.macro idct_8h_x8 q0, q1, q2, q3, q4, q5, q6, q7, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15
        idct_8h_x4      \q0, \q2, \q4, \q6, \r0, \r1, \r4, \r5, \r8, \r9, \r12, \r13

        vmull_vmlsl_8h  q2,   q3,   \r2,  \r3,  \r14, \r15, d1[0], d1[1] // -> t4a
        vmull_vmlal_8h  q4,   q5,   \r2,  \r3,  \r14, \r15, d1[1], d1[0] // -> t7a
        vmull_vmlsl_8h  q6,   q7,   \r10, \r11, \r6,  \r7,  d1[2], d1[3] // -> t5a
        vqrshrn_8h      \r2,  \r3,  q2,   q3,   #12         // t4a
        vqrshrn_8h      \r14, \r15, q4,   q5,   #12         // t7a
        vmull_vmlal_8h  q2,   q3,   \r10, \r11, \r6,  \r7,  d1[3], d1[2] // -> t6a
        vqrshrn_8h      \r6,  \r7,  q6,   q7,   #12         // t5a
        vqrshrn_8h      \r10, \r11, q2,   q3,   #12         // t6a

        vqadd.s16       q2,   \q1,  \q3 // t4
        vqsub.s16       \q1,  \q1,  \q3 // t5a
        vqadd.s16       q3,   \q7,  \q5 // t7
        vqsub.s16       \q3,  \q7,  \q5 // t6a

        vmull_vmlsl_8h  q4,   q5,   \r6,  \r7,  \r2,  \r3,  d0[0], d0[0] // -> t5
        vmull_vmlal_8h  q6,   q7,   \r6,  \r7,  \r2,  \r3,  d0[0], d0[0] // -> t6
        vqrshrn_8h      d8,   d9,   q4,   q5,  #12 // t5
        vqrshrn_8h      d10,  d11,  q6,   q7,  #12 // t6

        vqsub.s16       \q7,  \q0,  q3 // out7
        vqadd.s16       \q0,  \q0,  q3 // out0
        vqadd.s16       \q1,  \q2,  q5 // out1
        vqsub.s16       q6,   \q2,  q5 // out6
        vqadd.s16       \q2,  \q4,  q4 // out2
        vqsub.s16       \q5,  \q4,  q4 // out5
        vqadd.s16       \q3,  \q6,  q2 // out3
        vqsub.s16       \q4,  \q6,  q2 // out4
        vmov            \q6,  q6       // out6
.endm

.macro idct_4h_x8 r0, r1, r2, r3, r4, r5, r6, r7
        idct_4h_x4      \r0, \r2, \r4, \r6

        vmull_vmlsl     q1,   \r1,  \r7, d1[0], d1[1] // -> t4a
        vmull_vmlal     q2,   \r1,  \r7, d1[1], d1[0] // -> t7a
        vmull_vmlsl     q3,   \r5,  \r3, d1[2], d1[3] // -> t5a
        vqrshrn.s32     \r1,  q1,   #12               // t4a
        vmull_vmlal     q1,   \r5,  \r3, d1[3], d1[2] // -> t6a
        vqrshrn.s32     \r7,  q2,   #12               // t7a
        vqrshrn.s32     \r3,  q3,   #12               // t5a
        vqrshrn.s32     \r5,  q1,   #12               // taa

        vqadd.s16       d2,   \r1,  \r3 // t4
        vqsub.s16       \r1,  \r1,  \r3 // t5a
        vqadd.s16       d3,   \r7,  \r5 // t7
        vqsub.s16       \r3,  \r7,  \r5 // t6a

        vmull_vmlsl     q2,   \r3,  \r1, d0[0], d0[0] // -> t5
        vmull_vmlal     q3,   \r3,  \r1, d0[0], d0[0] // -> t6
        vqrshrn.s32     d4,   q2,   #12               // t5
        vqrshrn.s32     d5,   q3,   #12               // t6

        vqsub.s16       \r7,  \r0,  d3 // out7
        vqadd.s16       \r0,  \r0,  d3 // out0
        vqadd.s16       \r1,  \r2,  d5 // out1
        vqsub.s16       d6,   \r2,  d5 // out6
        vqadd.s16       \r2,  \r4,  d4 // out2
        vqsub.s16       \r5,  \r4,  d4 // out5
        vqadd.s16       \r3,  \r6,  d2 // out3
        vqsub.s16       \r4,  \r6,  d2 // out4
        vmov            \r6,  d6       // out6
.endm

function inv_dct_8h_x8_neon, export=1
        movrel_local    r12, idct_coeffs
        vld1.16         {q0}, [r12, :128]
        idct_8h_x8      q8,  q9,  q10, q11, q12, q13, q14, q15, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        bx              lr
endfunc

function inv_dct_4h_x8_neon, export=1
        movrel_local    r12, idct_coeffs
        vld1.16         {q0}, [r12, :128]
        idct_4h_x8      d16, d17, d18, d19, d20, d21, d22, d23
        bx              lr
endfunc

.macro iadst_8h_x8 q0, q1, q2, q3, q4, q5, q6, q7, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15
        movrel_local    r12, iadst8_coeffs
        vld1.16         {d0, d1, d2}, [r12, :64]

        vmull_vmlal_8h  q2,  q3,  d30, d31, d16, d17, d0[0], d0[1]
        vmull_vmlsl_8h  q4,  q5,  d30, d31, d16, d17, d0[1], d0[0]
        vmull_vmlal_8h  q6,  q7,  d26, d27, d20, d21, d0[2], d0[3]
        vqrshrn_8h      d16, d17, q2,  q3,  #12  // t0a
        vqrshrn_8h      d30, d31, q4,  q5,  #12  // t1a
        vmull_vmlsl_8h  q2,  q3,  d26, d27, d20, d21, d0[3], d0[2]
        vmull_vmlal_8h  q4,  q5,  d22, d23, d24, d25, d1[0], d1[1]
        vqrshrn_8h      d20, d21, q6,  q7,  #12  // t2a
        vqrshrn_8h      d26, d27, q2,  q3,  #12  // t3a
        vmull_vmlsl_8h  q6,  q7,  d22, d23, d24, d25, d1[1], d1[0]
        vmull_vmlal_8h  q2,  q3,  d18, d19, d28, d29, d1[2], d1[3]
        vqrshrn_8h      d24, d25, q4,  q5,  #12  // t4a
        vqrshrn_8h      d22, d23, q6,  q7,  #12  // t5a
        vmull_vmlsl_8h  q4,  q5,  d18, d19, d28, d29, d1[3], d1[2]
        vqrshrn_8h      d28, d29, q2,  q3,  #12  // t6a
        vqrshrn_8h      d18, d19, q4,  q5,  #12  // t7a

        vqadd.s16       q2,  q8,  q12 // t0
        vqsub.s16       q3,  q8,  q12 // t4
        vqadd.s16       q4,  q15, q11 // t1
        vqsub.s16       q5,  q15, q11 // t5
        vqadd.s16       q6,  q10, q14 // t2
        vqsub.s16       q7,  q10, q14 // t6
        vqadd.s16       q10, q13, q9  // t3
        vqsub.s16       q11, q13, q9  // t7

        vmull_vmlal_8h  q8,  q9,  d6,  d7,  d10, d11, d2[3], d2[2]
        vmull_vmlsl_8h  q12, q13, d6,  d7,  d10, d11, d2[2], d2[3]
        vmull_vmlsl_8h  q14, q15, d22, d23, d14, d15, d2[3], d2[2]

        vqrshrn_8h      d6,  d7,  q8,  q9,  #12  // t4a
        vqrshrn_8h      d10, d11, q12, q13, #12  // t5a

        vmull_vmlal_8h  q8,  q9,  d22, d23, d14, d15, d2[2], d2[3]

        vqrshrn_8h      d14, d15, q14, q15, #12  // t6a
        vqrshrn_8h      d22, d23, q8,  q9,  #12  // t7a

        vqadd.s16       \q0, q2,  q6  // out0
        vqsub.s16       q2,  q2,  q6  // t2
        vqadd.s16       \q7, q4,  q10 // out7
        vqsub.s16       q4,  q4,  q10 // t3
        vqneg.s16       \q7, \q7     // out7

        vqadd.s16       \q1, q3,  q7  // out1
        vqsub.s16       q3,  q3,  q7  // t6
        vqadd.s16       \q6, q5,  q11 // out6
        vqsub.s16       q5,  q5,  q11 // t7
        vqneg.s16       \q1, \q1     // out1

        vmull_vmlal_8h  q10, q11, d4,  d5,  d8,  d9,  d2[0], d2[0] // -> out3 (q11 or q12)
        vmull_vmlsl_8h  q6,  q7,  d4,  d5,  d8,  d9,  d2[0], d2[0] // -> out4 (q12 or q11)
        vmull_vmlsl_8h  q12, q13, d6,  d7,  d10, d11, d2[0], d2[0] // -> out5 (q13 or q10)
        vqrshrn_8h      d4,  d5,  q10, q11, #12 // out3
        vmull_vmlal_8h  q10, q11, d6,  d7,  d10, d11, d2[0], d2[0] // -> out2 (q10 or q13)
        vqrshrn_8h      d6,  d7,  q12, q13, #12 // out5
        vqrshrn_8h      \r4, \r5, q10, q11, #12 // out2 (q10 or q13)
        vqrshrn_8h      \r8, \r9, q6,  q7,  #12 // out4 (q12 or q11)

        vqneg.s16       \q3, q2     // out3
        vqneg.s16       \q5, q3     // out5
.endm

.macro iadst_4h_x8 r0, r1, r2, r3, r4, r5, r6, r7
        movrel_local    r12, iadst8_coeffs
        vld1.16         {d0, d1, d2}, [r12, :64]

        vmull_vmlal     q2,  d23, d16, d0[0], d0[1]
        vmull_vmlsl     q3,  d23, d16, d0[1], d0[0]
        vmull_vmlal     q4,  d21, d18, d0[2], d0[3]
        vqrshrn.s32     d16, q2,  #12 // t0a
        vqrshrn.s32     d23, q3,  #12 // t1a
        vmull_vmlsl     q5,  d21, d18, d0[3], d0[2]
        vmull_vmlal     q6,  d19, d20, d1[0], d1[1]
        vqrshrn.s32     d18, q4,  #12 // t2a
        vqrshrn.s32     d21, q5,  #12 // t3a
        vmull_vmlsl     q7,  d19, d20, d1[1], d1[0]
        vmull_vmlal     q2,  d17, d22, d1[2], d1[3]
        vqrshrn.s32     d20, q6,  #12 // t4a
        vqrshrn.s32     d19, q7,  #12 // t5a
        vmull_vmlsl     q3,  d17, d22, d1[3], d1[2]
        vqrshrn.s32     d22, q2,  #12 // t6a
        vqrshrn.s32     d17, q3,  #12 // t7a

        vqadd.s16       d4,  d16, d20 // t0
        vqsub.s16       d5,  d16, d20 // t4
        vqadd.s16       d6,  d23, d19 // t1
        vqsub.s16       d7,  d23, d19 // t5
        vqadd.s16       d8,  d18, d22 // t2
        vqsub.s16       d9,  d18, d22 // t6
        vqadd.s16       d18, d21, d17 // t3
        vqsub.s16       d19, d21, d17 // t7

        vmull_vmlal     q8,  d5,  d7,  d2[3], d2[2]
        vmull_vmlsl     q10, d5,  d7,  d2[2], d2[3]
        vmull_vmlsl     q11, d19, d9,  d2[3], d2[2]

        vqrshrn.s32     d5,  q8,  #12 // t4a
        vqrshrn.s32     d7,  q10, #12 // t5a

        vmull_vmlal     q8,  d19, d9,  d2[2], d2[3]

        vqrshrn.s32     d9,  q11, #12 // t6a
        vqrshrn.s32     d19, q8,  #12 // t7a

        vqadd.s16       \r0, d4,  d8  // out0
        vqsub.s16       d4,  d4,  d8  // t2
        vqadd.s16       \r7, d6,  d18 // out7
        vqsub.s16       d6,  d6,  d18 // t3
        vqneg.s16       \r7, \r7      // out7

        vqadd.s16       \r1, d5,  d9  // out1
        vqsub.s16       d5,  d5,  d9  // t6
        vqadd.s16       \r6, d7,  d19 // out6
        vqsub.s16       d7,  d7,  d19 // t7
        vqneg.s16       \r1, \r1      // out1

        vmull_vmlal     q9,  d4,  d6,  d2[0], d2[0] // -> out3 (d19 or d20)
        vmull_vmlsl     q4,  d4,  d6,  d2[0], d2[0] // -> out4 (d20 or d19)
        vmull_vmlsl     q10, d5,  d7,  d2[0], d2[0] // -> out5 (d21 or d18)
        vqrshrn.s32     d4,  q9,  #12 // out3
        vmull_vmlal     q9,  d5,  d7,  d2[0], d2[0] // -> out2 (d18 or d21)
        vqrshrn.s32     d5,  q10, #12 // out5
        vqrshrn.s32     \r2, q9,  #12 // out2 (d18 or d21)
        vqrshrn.s32     \r4, q4,  #12 // out4 (d20 or d19)

        vqneg.s16       \r3, d4       // out3
        vqneg.s16       \r5, d5       // out5
.endm

function inv_adst_8h_x8_neon, export=1
        iadst_8h_x8     q8,  q9,  q10, q11, q12, q13, q14, q15, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        bx              lr
endfunc

function inv_flipadst_8h_x8_neon, export=1
        iadst_8h_x8     q15, q14, q13, q12, q11, q10, q9,  q8,  d30, d31, d28, d29, d26, d27, d24, d25, d22, d23, d20, d21, d18, d19, d16, d17
        bx              lr
endfunc

function inv_adst_4h_x8_neon, export=1
        iadst_4h_x8     d16, d17, d18, d19, d20, d21, d22, d23
        bx              lr
endfunc

function inv_flipadst_4h_x8_neon, export=1
        iadst_4h_x8     d23, d22, d21, d20, d19, d18, d17, d16
        bx              lr
endfunc

function inv_identity_8h_x8_neon, export=1
        vqshl.s16       q8,  q8,  #1
        vqshl.s16       q9,  q9,  #1
        vqshl.s16       q10, q10, #1
        vqshl.s16       q11, q11, #1
        vqshl.s16       q12, q12, #1
        vqshl.s16       q13, q13, #1
        vqshl.s16       q14, q14, #1
        vqshl.s16       q15, q15, #1
        bx              lr
endfunc

function inv_identity_4h_x8_neon, export=1
        vqshl.s16       q8,  q8,  #1
        vqshl.s16       q9,  q9,  #1
        vqshl.s16       q10, q10, #1
        vqshl.s16       q11, q11, #1
        bx              lr
endfunc

.macro def_fn_8x8_base variant
function inv_txfm_\variant\()add_8x8_neon
        vmov.i16        q0,  #0
        vmov.i16        q1,  #0
        vld1.16         {q8,  q9},  [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]!
        vld1.16         {q10, q11}, [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]!
        vld1.16         {q12, q13}, [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]!
        vld1.16         {q14, q15}, [r2, :128]
        vst1.16         {q0,  q1},  [r2, :128]

.ifc \variant, identity_
        // The identity shl #1 and downshift srshr #1 cancel out
.else
        blx             r4

        vrshr.s16       q8,  q8,  #1
        vrshr.s16       q9,  q9,  #1
        vrshr.s16       q10, q10, #1
        vrshr.s16       q11, q11, #1
        vrshr.s16       q12, q12, #1
        vrshr.s16       q13, q13, #1
        vrshr.s16       q14, q14, #1
        vrshr.s16       q15, q15, #1
.endif

        transpose_8x8h  q8,  q9,  q10, q11, q12, q13, q14, q15, d17, d19, d21, d23, d24, d26, d28, d30

        blx             r5

        load_add_store_8x8 r0, r7
        vpop            {q4-q7}
        pop             {r4-r5,r7,pc}
endfunc
.endm

def_fn_8x8_base
def_fn_8x8_base identity_

.macro def_fn_8x8 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_8x8_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         8,   8,   1
.endif
        push            {r4-r5,r7,lr}
        vpush           {q4-q7}
        movrel_local    r5,  inv_\txfm2\()_8h_x8_neon
.ifc \txfm1, identity
        b               inv_txfm_identity_add_8x8_neon
.else
        movrel_local    r4,  inv_\txfm1\()_8h_x8_neon
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
        vmov.i16        q14, #0
        vmov.i16        q15, #0
        movw            r12, #2896*8
        vdup.16         d0,  r12
        vld1.16         {d16, d17, d18, d19}, [r2, :128]
        vst1.16         {q14, q15}, [r2, :128]!
        vld1.16         {d20, d21, d22, d23}, [r2, :128]
        vst1.16         {q14, q15}, [r2, :128]

        scale_input     d0[0], q8,  q9, q10, q11

        blx             r4

        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        vswp            d17, d20
        vswp            d19, d21
        vswp            d18, d20
        vswp            d21, d22

        blx             r5

        load_add_store_8x4 r0, r7
        vpop            {q4-q7}
        pop             {r4-r5,r7,pc}
endfunc

function inv_txfm_add_4x8_neon
        vmov.i16        q14, #0
        vmov.i16        q15, #0
        movw            r12, #2896*8
        vdup.16         d0,  r12
        vld1.16         {q8,  q9},  [r2, :128]
        vst1.16         {q14, q15}, [r2, :128]!
        vld1.16         {q10, q11}, [r2, :128]
        vst1.16         {q14, q15}, [r2, :128]

        scale_input     d0[0], q8,  q9, q10, q11

        blx             r4

        transpose_4x8h  q8,  q9,  q10, q11
        vswp            d17, d20
        vswp            d19, d21
        vswp            d17, d18
        vswp            d19, d22

        blx             r5

        load_add_store_4x8 r0, r7
        vpop            {q4-q7}
        pop             {r4-r5,r7,pc}
endfunc

.macro def_fn_48 w, h, txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  0
.endif
        push            {r4-r5,r7,lr}
        vpush           {q4-q7}
        movrel_local    r4,  inv_\txfm1\()_\h\()h_x\w\()_neon
        movrel_local    r5,  inv_\txfm2\()_\w\()h_x\h\()_neon
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

function inv_dct_4h_x16_neon, export=1
        movrel_local    r12, idct_coeffs
        vld1.16         {q0, q1}, [r12, :128]

        vmull_vmlsl     q2,  d17, d31, d2[0], d2[1]  // -> t8a
        vmull_vmlal     q3,  d17, d31, d2[1], d2[0]  // -> t15a
        vmull_vmlsl     q4,  d25, d23, d2[2], d2[3]  // -> t9a
        vqrshrn.s32     d17, q2,  #12                // t8a
        vqrshrn.s32     d31, q3,  #12                // t15a
        vmull_vmlal     q2,  d25, d23, d2[3], d2[2]  // -> t14a
        vmull_vmlsl     q3,  d21, d27, d3[0], d3[1]  // -> t10a
        vqrshrn.s32     d23, q4,  #12                // t9a
        vqrshrn.s32     d25, q2,  #12                // t14a
        vmull_vmlal     q4,  d21, d27, d3[1], d3[0]  // -> t13a
        vmull_vmlsl     q2,  d29, d19, d3[2], d3[3]  // -> t11a
        vqrshrn.s32     d21, q3,  #12                // t10a
        vqrshrn.s32     d27, q4,  #12                // t13a
        vmull_vmlal     q3,  d29, d19, d3[3], d3[2]  // -> t12a
        vqrshrn.s32     d19, q2,  #12                // t11a
        vqrshrn.s32     d29, q3,  #12                // t12a

        idct_4h_x8      d16, d18, d20, d22, d24, d26, d28, d30

        vqsub.s16       d4,  d17, d23  // t9
        vqadd.s16       d17, d17, d23  // t8
        vqsub.s16       d5,  d31, d25  // t14
        vqadd.s16       d31, d31, d25  // t15
        vqsub.s16       d23, d19, d21  // t10
        vqadd.s16       d19, d19, d21  // t11
        vqadd.s16       d25, d29, d27  // t12
        vqsub.s16       d29, d29, d27  // t13

        vmull_vmlsl     q3,  d5,  d4,  d0[2], d0[3]  // -> t9a
        vmull_vmlal     q4,  d5,  d4,  d0[3], d0[2]  // -> t14a
        vqrshrn.s32     d21, q3,  #12                // t9a
        vqrshrn.s32     d27, q4,  #12                // t14a

        vmull_vmlsl     q3,  d29, d23, d0[2], d0[3]  // -> t13a
        vmull_vmlal     q4,  d29, d23, d0[3], d0[2]  // -> t10a
        vqrshrn.s32     d29, q3,  #12                // t13a
        vneg.s32        q4,  q4
        vqrshrn.s32     d23, q4,  #12                // t10a

        vqsub.s16       d4,  d17, d19  // t11a
        vqadd.s16       d17, d17, d19  // t8a
        vqsub.s16       d5,  d31, d25  // t12a
        vqadd.s16       d31, d31, d25  // t15a
        vqadd.s16       d19, d21, d23  // t9
        vqsub.s16       d21, d21, d23  // t10
        vqsub.s16       d25, d27, d29  // t13
        vqadd.s16       d27, d27, d29  // t14

        vmull_vmlsl     q3,  d5,  d4,  d0[0], d0[0]  // -> t11
        vmull_vmlal     q4,  d5,  d4,  d0[0], d0[0]  // -> t12
        vmull_vmlsl     q2,  d25, d21, d0[0], d0[0]  // -> t10a

        vqrshrn.s32     d6,  q3,  #12  // t11
        vqrshrn.s32     d7,  q4,  #12  // t12
        vmull_vmlal     q4,  d25, d21, d0[0], d0[0]  // -> t13a
        vqrshrn.s32     d4,  q2,  #12  // t10a
        vqrshrn.s32     d5,  q4,  #12  // t13a

        vqadd.s16       d8,  d16, d31  // out0
        vqsub.s16       d31, d16, d31  // out15
        vmov            d16, d8
        vqadd.s16       d23, d30, d17  // out7
        vqsub.s16       d9,  d30, d17  // out8
        vqadd.s16       d17, d18, d27  // out1
        vqsub.s16       d30, d18, d27  // out14
        vqadd.s16       d18, d20, d5   // out2
        vqsub.s16       d29, d20, d5   // out13
        vqadd.s16       d5,  d28, d19  // out6
        vqsub.s16       d25, d28, d19  // out9
        vqadd.s16       d19, d22, d7   // out3
        vqsub.s16       d28, d22, d7   // out12
        vqadd.s16       d20, d24, d6   // out4
        vqsub.s16       d27, d24, d6   // out11
        vqadd.s16       d21, d26, d4   // out5
        vqsub.s16       d26, d26, d4   // out10
        vmov            d24, d9
        vmov            d22, d5

        bx              lr
endfunc

.macro iadst_16 o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15
        movrel_local    r12, iadst16_coeffs
        vld1.16         {q0, q1}, [r12, :128]
        movrel_local    r12, idct_coeffs

        vmull_vmlal     q2,  d31, d16, d0[0], d0[1] // -> t0
        vmull_vmlsl     q3,  d31, d16, d0[1], d0[0] // -> t1
        vmull_vmlal     q4,  d29, d18, d0[2], d0[3] // -> t2
        vqrshrn.s32     d16, q2,  #12               // t0
        vqrshrn.s32     d31, q3,  #12               // t1
        vmull_vmlsl     q2,  d29, d18, d0[3], d0[2] // -> t3
        vmull_vmlal     q3,  d27, d20, d1[0], d1[1] // -> t4
        vqrshrn.s32     d18, q4,  #12               // t2
        vqrshrn.s32     d29, q2,  #12               // t3
        vmull_vmlsl     q4,  d27, d20, d1[1], d1[0] // -> t5
        vmull_vmlal     q2,  d25, d22, d1[2], d1[3] // -> t6
        vqrshrn.s32     d20, q3,  #12               // t4
        vqrshrn.s32     d27, q4,  #12               // t5
        vmull_vmlsl     q3,  d25, d22, d1[3], d1[2] // -> t7
        vmull_vmlal     q4,  d23, d24, d2[0], d2[1] // -> t8
        vqrshrn.s32     d22, q2,  #12               // t6
        vqrshrn.s32     d25, q3,  #12               // t7
        vmull_vmlsl     q2,  d23, d24, d2[1], d2[0] // -> t9
        vmull_vmlal     q3,  d21, d26, d2[2], d2[3] // -> t10
        vqrshrn.s32     d23, q4,  #12               // t8
        vqrshrn.s32     d24, q2,  #12               // t9
        vmull_vmlsl     q4,  d21, d26, d2[3], d2[2] // -> t11
        vmull_vmlal     q2,  d19, d28, d3[0], d3[1] // -> t12
        vqrshrn.s32     d21, q3,  #12               // t10
        vqrshrn.s32     d26, q4,  #12               // t11
        vmull_vmlsl     q3,  d19, d28, d3[1], d3[0] // -> t13
        vmull_vmlal     q4,  d17, d30, d3[2], d3[3] // -> t14
        vqrshrn.s32     d19, q2,  #12               // t12
        vqrshrn.s32     d28, q3,  #12               // t13
        vmull_vmlsl     q2,  d17, d30, d3[3], d3[2] // -> t15
        vqrshrn.s32     d17, q4,  #12               // t14
        vqrshrn.s32     d30, q2,  #12               // t15

        vld1.16         {q0}, [r12, :128]

        vqsub.s16       d2,  d16, d23 // t8a
        vqadd.s16       d16, d16, d23 // t0a
        vqsub.s16       d3,  d31, d24 // t9a
        vqadd.s16       d31, d31, d24 // t1a
        vqadd.s16       d23, d18, d21 // t2a
        vqsub.s16       d18, d18, d21 // t10a
        vqadd.s16       d24, d29, d26 // t3a
        vqsub.s16       d29, d29, d26 // t11a
        vqadd.s16       d21, d20, d19 // t4a
        vqsub.s16       d20, d20, d19 // t12a
        vqadd.s16       d26, d27, d28 // t5a
        vqsub.s16       d27, d27, d28 // t13a
        vqadd.s16       d19, d22, d17 // t6a
        vqsub.s16       d22, d22, d17 // t14a
        vqadd.s16       d28, d25, d30 // t7a
        vqsub.s16       d25, d25, d30 // t15a

        vmull_vmlal     q2,  d2,  d3,  d1[1], d1[0] // -> t8
        vmull_vmlsl     q3,  d2,  d3,  d1[0], d1[1] // -> t9
        vmull_vmlal     q4,  d18, d29, d1[3], d1[2] // -> t10
        vqrshrn.s32     d17, q2,  #12               // t8
        vqrshrn.s32     d30, q3,  #12               // t9
        vmull_vmlsl     q2,  d18, d29, d1[2], d1[3] // -> t11
        vmull_vmlsl     q3,  d27, d20, d1[1], d1[0] // -> t12
        vqrshrn.s32     d18, q4,  #12               // t10
        vqrshrn.s32     d29, q2,  #12               // t11
        vmull_vmlal     q4,  d27, d20, d1[0], d1[1] // -> t13
        vmull_vmlsl     q2,  d25, d22, d1[3], d1[2] // -> t14
        vqrshrn.s32     d27, q3,  #12               // t12
        vqrshrn.s32     d20, q4,  #12               // t13
        vmull_vmlal     q3,  d25, d22, d1[2], d1[3] // -> t15
        vqrshrn.s32     d25, q2,  #12               // t14
        vqrshrn.s32     d22, q3,  #12               // t15

        vqsub.s16       d2,  d16, d21 // t4
        vqadd.s16       d16, d16, d21 // t0
        vqsub.s16       d3,  d31, d26 // t5
        vqadd.s16       d31, d31, d26 // t1
        vqadd.s16       d21, d23, d19 // t2
        vqsub.s16       d23, d23, d19 // t6
        vqadd.s16       d26, d24, d28 // t3
        vqsub.s16       d24, d24, d28 // t7
        vqadd.s16       d19, d17, d27 // t8a
        vqsub.s16       d17, d17, d27 // t12a
        vqadd.s16       d28, d30, d20 // t9a
        vqsub.s16       d30, d30, d20 // t13a
        vqadd.s16       d27, d18, d25 // t10a
        vqsub.s16       d18, d18, d25 // t14a
        vqadd.s16       d20, d29, d22 // t11a
        vqsub.s16       d29, d29, d22 // t15a

        vmull_vmlal     q2,  d2,  d3,  d0[3], d0[2] // -> t4a
        vmull_vmlsl     q3,  d2,  d3,  d0[2], d0[3] // -> t5a
        vmull_vmlsl     q4,  d24, d23, d0[3], d0[2] // -> t6a
        vqrshrn.s32     d22, q2,  #12               // t4a
        vqrshrn.s32     d25, q3,  #12               // t5a
        vmull_vmlal     q2,  d24, d23, d0[2], d0[3] // -> t7a
        vmull_vmlal     q3,  d17, d30, d0[3], d0[2] // -> t12
        vqrshrn.s32     d24, q4,  #12               // t6a
        vqrshrn.s32     d23, q2,  #12               // t7a
        vmull_vmlsl     q4,  d17, d30, d0[2], d0[3] // -> t13
        vmull_vmlsl     q2,  d29, d18, d0[3], d0[2] // -> t14
        vqrshrn.s32     d17, q3,  #12               // t12
        vmull_vmlal     q3,  d29, d18, d0[2], d0[3] // -> t15
        vqrshrn.s32     d29, q4,  #12               // t13
        vqrshrn.s32     d30, q2,  #12               // t14
        vqrshrn.s32     d18, q3,  #12               // t15

        vqsub.s16       d2,  d16, d21 // t2a
.ifc \o0, d16
        vqadd.s16       \o0, d16, d21 // out0
        vqsub.s16       d21, d31, d26 // t3a
        vqadd.s16       \o15,d31, d26 // out15
.else
        vqadd.s16       d4,  d16, d21 // out0
        vqsub.s16       d21, d31, d26 // t3a
        vqadd.s16       \o15,d31, d26 // out15
        vmov            \o0, d4
.endif
        vqneg.s16       \o15, \o15    // out15

        vqsub.s16       d3,  d29, d18 // t15a
        vqadd.s16       \o13,d29, d18 // out13
        vqadd.s16       \o2, d17, d30 // out2
        vqsub.s16       d26, d17, d30 // t14a
        vqneg.s16       \o13,\o13     // out13

        vqadd.s16       \o1, d19, d27 // out1
        vqsub.s16       d27, d19, d27 // t10
        vqadd.s16       \o14,d28, d20 // out14
        vqsub.s16       d20, d28, d20 // t11
        vqneg.s16       \o1, \o1      // out1

        vqadd.s16       \o3, d22, d24 // out3
        vqsub.s16       d22, d22, d24 // t6
        vqadd.s16       \o12,d25, d23 // out12
        vqsub.s16       d23, d25, d23 // t7
        vqneg.s16       \o3, \o3      // out3

        vmull_vmlsl     q12, d2,  d21, d0[0], d0[0] // -> out8 (d24 or d23)
        vmull_vmlal     q2,  d2,  d21, d0[0], d0[0] // -> out7 (d23 or d24)
        vmull_vmlal     q3,  d26, d3,  d0[0], d0[0] // -> out5 (d21 or d26)

        vqrshrn.s32     d24, q12, #12 // out8
        vqrshrn.s32     d4,  q2,  #12 // out7
        vqrshrn.s32     d5,  q3,  #12 // out5
        vmull_vmlsl     q4,  d26, d3,  d0[0], d0[0] // -> out10 (d26 or d21)
        vmull_vmlal     q1,  d22, d23, d0[0], d0[0] // -> out4 (d20 or d27)
        vqrshrn.s32     d26, q4,  #12 // out10

        vmull_vmlsl     q4,  d22, d23, d0[0], d0[0] // -> out11 (d27 or d20)
        vmull_vmlal     q11, d27, d20, d0[0], d0[0] // -> out6 (d22 or d25)
        vmull_vmlsl     q3,  d27, d20, d0[0], d0[0] // -> out9 (d25 or d22)

        vqrshrn.s32     \o4, q1,  #12 // out4
        vqrshrn.s32     d7,  q3,  #12 // out9
        vqrshrn.s32     d6,  q4,  #12 // out11
        vqrshrn.s32     \o6, q11, #12 // out6

.ifc \o8, d23
        vmov            \o8, d24
        vmov            \o10,d26
.endif

        vqneg.s16       \o7, d4  // out7
        vqneg.s16       \o5, d5  // out5
        vqneg.s16       \o11,d6  // out11
        vqneg.s16       \o9, d7  // out9
.endm

function inv_adst_4h_x16_neon, export=1
        iadst_16        d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        bx              lr
endfunc

function inv_flipadst_4h_x16_neon, export=1
        iadst_16        d31, d30, d29, d28, d27, d26, d25, d24, d23, d22, d21, d20, d19, d18, d17, d16
        bx              lr
endfunc

function inv_identity_4h_x16_neon, export=1
        movw            r12, #2*(5793-4096)*8
        vdup.16         d0,  r12
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vqrdmulh.s16    q1,  \i,  d0[0]
        vqadd.s16       \i,  \i,  \i
        vqadd.s16       \i,  \i,  q1
.endr
        bx              lr
endfunc

.macro identity_4x16_shift2 c
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vqrdmulh.s16    q2,  \i,  \c
        vshr.s16        q2,  q2,  #1
        vrhadd.s16      \i,  \i,  q2
.endr
.endm

.macro identity_4x16_shift1 c
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vqrdmulh.s16    q2,  \i,  \c
        vrshr.s16       q2,  q2,  #1
        vqadd.s16       \i,  \i,  q2
.endr
.endm

.macro identity_8x8_shift1 c
        identity_4x16_shift1 \c
.endm

.macro identity_8x8 c
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vqrdmulh.s16    q2,  \i,  \c
        vqadd.s16       \i,  \i,  \i
        vqadd.s16       \i,  \i,  q2
.endr
.endm

.macro def_horz_16 scale=0, identity=0, shift=2, suffix
function inv_txfm_horz\suffix\()_16x4_neon
        push            {lr}
        vmov.i16        d7,  #0
.if \identity
        movw            r12, #2*(5793-4096)*8
        vdup.16         d0,  r12
.endif
.if \scale
        movw            r12, #2896*8
        vdup.16         d1,  r12
.endif
.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.16         {\i}, [r7, :64]
        vst1.16         {d7}, [r7, :64], r8
.endr
.if \scale
        scale_input     d1[0], q8,  q9, q10, q11, q12, q13, q14, q15
.endif
.if \identity
.if \shift == -2
        identity_4x16_shift2 d0[0]
.else
        identity_4x16_shift1 d0[0]
.endif
.else
        blx             r4
.endif
.if \shift > 0
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vrshr.s16       \i,  \i,  #\shift
.endr
.endif
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q12, q13, d24, d25, d26, d27
        transpose_4x4h  q14, q15, d28, d29, d30, d31

.irp i, d16, d20, d24, d28, d17, d21, d25, d29, d18, d22, d26, d30, d19, d23, d27, d31
        vst1.16         {\i}, [r6, :64]!
.endr

        pop             {pc}
endfunc
.endm

def_horz_16 scale=0, identity=0, shift=2
def_horz_16 scale=1, identity=0, shift=1, suffix=_scale
def_horz_16 scale=0, identity=1, shift=-2, suffix=_identity
def_horz_16 scale=1, identity=1, shift=-1, suffix=_scale_identity

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
.irp i, 0, 4, 8, 12
        add             r6,  sp,  #(\i*16*2)
.if \i > 0
        mov             r8,  #(16 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 12
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #16*2
        blx             r9
.endr
        b               3f
1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #4
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b
3:
.irp i, 0, 4, 8, 12
        add             r6,  r0,  #(\i)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    512
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

const eob_16x16
        .short 10, 36, 78, 256
endconst

const eob_16x16_identity
        .short 4, 8, 12, 256
endconst

.macro def_fn_16x16 txfm1, txfm2
function inv_txfm_add_\txfm1\()_\txfm2\()_16x16_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         16,  16,  2
.endif
        push            {r4-r11,lr}
        vpush           {q4}
.ifc \txfm1, identity
        movrel_local    r9,  inv_txfm_horz_identity_16x4_neon
.else
        movrel_local    r9,  inv_txfm_horz_16x4_neon
        movrel_local    r4,  inv_\txfm1\()_4h_x16_neon
.endif
        movrel_local    r5,  inv_\txfm2\()_4h_x16_neon
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

.macro def_fn_416_base variant
function inv_txfm_\variant\()add_16x4_neon

.ifc \variant, identity_
        vmov.i16        d4,  #0
.irp i, d16, d18, d20, d22
        vld1.16         {\i}, [r2, :64]
        vst1.16         {d4}, [r2, :64]!
.endr
.irp i, d17, d19, d21, d23
        vld1.16         {\i}, [r2, :64]
        vst1.16         {d4}, [r2, :64]!
.endr
        movw            r12, #2*(5793-4096)*8
        vdup.16         d0,  r12
.irp i, d24, d26, d28, d30
        vld1.16         {\i}, [r2, :64]
        vst1.16         {d4}, [r2, :64]!
.endr
.irp i, d25, d27, d29, d31
        vld1.16         {\i}, [r2, :64]
        vst1.16         {d4}, [r2, :64]!
.endr

        identity_4x16_shift1 d0[0]
.else
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
        vld1.16         {d16, d17, d18, d19}, [r2, :128]
        vst1.16         {q2,  q3}, [r2, :128]!
        vld1.16         {d20, d21, d22, d23}, [r2, :128]
        vst1.16         {q2,  q3}, [r2, :128]!
        vld1.16         {d24, d25, d26, d27}, [r2, :128]
        vst1.16         {q2,  q3}, [r2, :128]!
        vld1.16         {d28, d29, d30, d31}, [r2, :128]
        vst1.16         {q2,  q3}, [r2, :128]!

        blx             r4

        vswp            d17, d20
        vswp            d19, d22
        vswp            d18, d20
        vswp            d19, d21
.irp i, q8, q9, q10, q11
        vrshr.s16       \i,  \i,  #1
.endr
.endif
        transpose_4x8h  q8,  q9,  q10, q11
        blx             r5
        mov             r6,  r0
        load_add_store_8x4 r6, r7

.ifc \variant, identity_
        vmov            q8,  q12
        vmov            q9,  q13
        vmov            q10, q14
        vmov            q11, q15
.else
        vswp            d25, d28
        vswp            d27, d30
        vswp            d26, d28
        vswp            d27, d29
        vrshr.s16       q8,  q12, #1
        vrshr.s16       q9,  q13, #1
        vrshr.s16       q10, q14, #1
        vrshr.s16       q11, q15, #1
.endif
        transpose_4x8h  q8,  q9,  q10, q11
        blx             r5
        add             r6,  r0,  #8
        load_add_store_8x4 r6, r7

        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_\variant\()add_4x16_neon
        vmov.i16        q2,  #0

        mov             r11, #32
        cmp             r3,  r10
        blt             1f

        add             r6,  r2,  #16
.ifc \variant, identity_
.irp i, q12, q13, q14, q15
        vld1.16         {\i}, [r6, :128]
        vst1.16         {q2}, [r6, :128], r11
.endr
        movw            r12, #(5793-4096)*8
        vdup.16         d0,  r12
        identity_8x4_shift1 q12, q13, q14, q15, d0[0]
.else
.irp i, q8,  q9,  q10, q11
        vld1.16         {\i}, [r6, :128]
        vst1.16         {q2}, [r6, :128], r11
.endr
        blx             r4
        vrshr.s16       q12, q8,  #1
        vrshr.s16       q13, q9,  #1
        vrshr.s16       q14, q10, #1
        vrshr.s16       q15, q11, #1
.endif
        transpose_4x8h  q12, q13, q14, q15
        vswp            d27, d29
        vswp            d26, d28
        vswp            d27, d30
        vswp            d25, d28

        b               2f
1:
.irp i, q12, q13, q14, q15
        vmov.i16        \i,  #0
.endr
2:
        vmov.i16        q2,  #0
.irp i, q8,  q9,  q10, q11
        vld1.16         {\i}, [r2, :128]
        vst1.16         {q2}, [r2, :128], r11
.endr
.ifc \variant, identity_
        movw            r12, #(5793-4096)*8
        vdup.16         d0,  r12
        identity_8x4_shift1 q8,  q9,  q10, q11, d0[0]
.else
        blx             r4
.irp i, q8, q9, q10, q11
        vrshr.s16       \i,  \i,  #1
.endr
.endif
        transpose_4x8h  q8,  q9,  q10, q11
        vswp            d19, d21
        vswp            d18, d20
        vswp            d19, d22
        vswp            d17, d20

        blx             r5

        load_add_store_4x16 r0, r6

        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc
.endm

def_fn_416_base
def_fn_416_base identity_

.macro def_fn_416 w, h, txfm1, txfm2, eob_half
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
        push            {r4-r11,lr}
        vpush           {q4-q7}
.if \w == 4
        movrel_local    r4,  inv_\txfm1\()_8h_x\w\()_neon
        movrel_local    r5,  inv_\txfm2\()_4h_x\h\()_neon
        mov             r10, #\eob_half
.else
        movrel_local    r4,  inv_\txfm1\()_4h_x\w\()_neon
        movrel_local    r5,  inv_\txfm2\()_8h_x\h\()_neon
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
        sub_sp_align    256

.irp i, 0, 4
        add             r6,  sp,  #(\i*16*2)
.if \i > 0
        cmp             r3,  r10
        blt             1f
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #8*2
        blx             r9
.endr
        b               2f
1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
2:

.irp i, 0, 8
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32
.irp j, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\j}, [r7, :128], r8
.endr
        blx             r5

        add             r6,  r0,  #(\i)
        load_add_store_8x8 r6, r7
.endr

        add_sp_align    256
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_\variant\()add_8x16_neon
        sub_sp_align    256

.irp i, 0, 8
        add             r6,  sp,  #(\i*8*2)
.if \i > 0
        cmp             r3,  r10
        blt             1f
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #16*2

        vmov.i16        q2,  #0
        movw            r12, #2896*8
        vdup.16         d0,  r12

.irp j, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\j}, [r7, :128]
        vst1.16         {q2}, [r7, :128], r8
.endr
        scale_input     d0[0], q8,  q9,  q10, q11, q12, q13, q14, q15
.ifc \variant, identity_
        // The identity shl #1 and downshift vrshr #1 cancel out
.else
        blx             r4
.irp j, q8, q9, q10, q11, q12, q13, q14, q15
        vrshr.s16       \j,  \j,  #1
.endr
.endif
        transpose_8x8h  q8,  q9,  q10, q11, q12, q13, q14, q15, d17, d19, d21, d23, d24, d26, d28, d30
        vst1.16         {q8,  q9},  [r6, :128]!
        vst1.16         {q10, q11}, [r6, :128]!
        vst1.16         {q12, q13}, [r6, :128]!
        vst1.16         {q14, q15}, [r6, :128]!
.endr
        b               2f
1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
2:

.irp i, 0, 4
        add             r6,  r0,  #(\i)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #16
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    256
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc
.endm

def_fn_816_base
def_fn_816_base identity_

.macro def_fn_816 w, h, txfm1, txfm2, eob_8x8, eob_4x4
function inv_txfm_add_\txfm1\()_\txfm2\()_\w\()x\h\()_8bpc_neon, export=1
.ifc \txfm1\()_\txfm2, dct_dct
        idct_dc         \w,  \h,  1
.endif
        push            {r4-r11,lr}
        vpush           {q4-q7}
.if \w == 8
        movrel_local    r4,  inv_\txfm1\()_8h_x8_neon
        movrel_local    r5,  inv_\txfm2\()_4h_x16_neon
.else
.ifc \txfm1, identity
        movrel_local    r9,  inv_txfm_horz_scale_identity_16x4_neon
.else
        movrel_local    r4,  inv_\txfm1\()_4h_x16_neon
        movrel_local    r9,  inv_txfm_horz_scale_16x4_neon
.endif
        movrel_local    r5,  inv_\txfm2\()_8h_x8_neon
.endif
.if \w == 8
        mov             r10, #\eob_8x8
.else
        mov             r10, #\eob_4x4
.endif
.ifc \txfm1, identity
        b               inv_txfm_identity_add_\w\()x\h\()_neon
.else
        b               inv_txfm_add_\w\()x\h\()_neon
.endif
endfunc
.endm

.macro def_fns_816 w, h
def_fn_816 \w, \h, dct, dct, 43, 10
def_fn_816 \w, \h, identity, identity, 43, 10
def_fn_816 \w, \h, dct, adst, 43, 10
def_fn_816 \w, \h, dct, flipadst, 43, 10
def_fn_816 \w, \h, dct, identity, 8, 4
def_fn_816 \w, \h, adst, dct, 43, 10
def_fn_816 \w, \h, adst, adst, 43, 10
def_fn_816 \w, \h, adst, flipadst, 43, 10
def_fn_816 \w, \h, flipadst, dct, 43, 10
def_fn_816 \w, \h, flipadst, adst, 43, 10
def_fn_816 \w, \h, flipadst, flipadst, 43, 10
def_fn_816 \w, \h, identity, dct, 64, 4
def_fn_816 \w, \h, adst, identity, 8, 4
def_fn_816 \w, \h, flipadst, identity, 8, 4
def_fn_816 \w, \h, identity, adst, 64, 4
def_fn_816 \w, \h, identity, flipadst, 64, 4
.endm

def_fns_816 8, 16
def_fns_816 16, 8

function inv_dct32_odd_4h_x16_neon, export=1
        movrel_local    r12, idct_coeffs, 2*16
        vld1.16         {q0, q1}, [r12, :128]
        sub             r12, r12, #2*16

        vmull_vmlsl     q2,  d16, d31, d0[0], d0[1] // -> t16a
        vmull_vmlal     q3,  d16, d31, d0[1], d0[0] // -> t31a
        vmull_vmlsl     q4,  d24, d23, d0[2], d0[3] // -> t17a
        vqrshrn.s32     d16, q2,  #12               // t16a
        vqrshrn.s32     d31, q3,  #12               // t31a
        vmull_vmlal     q2,  d24, d23, d0[3], d0[2] // -> t30a
        vmull_vmlsl     q3,  d20, d27, d1[0], d1[1] // -> t18a
        vqrshrn.s32     d24, q4,  #12               // t17a
        vqrshrn.s32     d23, q2,  #12               // t30a
        vmull_vmlal     q4,  d20, d27, d1[1], d1[0] // -> t29a
        vmull_vmlsl     q2,  d28, d19, d1[2], d1[3] // -> t19a
        vqrshrn.s32     d20, q3,  #12               // t18a
        vqrshrn.s32     d27, q4,  #12               // t29a
        vmull_vmlal     q3,  d28, d19, d1[3], d1[2] // -> t28a
        vmull_vmlsl     q4,  d18, d29, d2[0], d2[1] // -> t20a
        vqrshrn.s32     d28, q2,  #12               // t19a
        vqrshrn.s32     d19, q3,  #12               // t28a
        vmull_vmlal     q2,  d18, d29, d2[1], d2[0] // -> t27a
        vmull_vmlsl     q3,  d26, d21, d2[2], d2[3] // -> t21a
        vqrshrn.s32     d18, q4,  #12               // t20a
        vqrshrn.s32     d29, q2,  #12               // t27a
        vmull_vmlal     q4,  d26, d21, d2[3], d2[2] // -> t26a
        vmull_vmlsl     q2,  d22, d25, d3[0], d3[1] // -> t22a
        vqrshrn.s32     d26, q3,  #12               // t21a
        vqrshrn.s32     d21, q4,  #12               // t26a
        vmull_vmlal     q3,  d22, d25, d3[1], d3[0] // -> t25a
        vmull_vmlsl     q4,  d30, d17, d3[2], d3[3] // -> t23a
        vqrshrn.s32     d22, q2,  #12               // t22a
        vqrshrn.s32     d25, q3,  #12               // t25a
        vmull_vmlal     q2,  d30, d17, d3[3], d3[2] // -> t24a
        vqrshrn.s32     d30, q4,  #12               // t23a
        vqrshrn.s32     d17, q2,  #12               // t24a

        vld1.16         {q0}, [r12, :128]

        vqsub.s16       d2,  d16, d24 // t17
        vqadd.s16       d16, d16, d24 // t16
        vqsub.s16       d3,  d31, d23 // t30
        vqadd.s16       d31, d31, d23 // t31
        vqsub.s16       d24, d28, d20 // t18
        vqadd.s16       d28, d28, d20 // t19
        vqadd.s16       d23, d18, d26 // t20
        vqsub.s16       d18, d18, d26 // t21
        vqsub.s16       d20, d30, d22 // t22
        vqadd.s16       d30, d30, d22 // t23
        vqadd.s16       d26, d17, d25 // t24
        vqsub.s16       d17, d17, d25 // t25
        vqsub.s16       d22, d29, d21 // t26
        vqadd.s16       d29, d29, d21 // t27
        vqadd.s16       d25, d19, d27 // t28
        vqsub.s16       d19, d19, d27 // t29

        vmull_vmlsl     q2,  d3,  d2,  d1[0], d1[1] // -> t17a
        vmull_vmlal     q3,  d3,  d2,  d1[1], d1[0] // -> t30a
        vmull_vmlal     q4,  d19, d24, d1[1], d1[0] // -> t18a
        vqrshrn.s32     d21, q2,  #12               // t17a
        vqrshrn.s32     d27, q3,  #12               // t30a
        vneg.s32        q4,  q4                     // -> t18a
        vmull_vmlsl     q1,  d19, d24, d1[0], d1[1] // -> t29a
        vmull_vmlsl     q2,  d22, d18, d1[2], d1[3] // -> t21a
        vqrshrn.s32     d19, q4,  #12               // t18a
        vqrshrn.s32     d24, q1,  #12               // t29a
        vmull_vmlal     q3,  d22, d18, d1[3], d1[2] // -> t26a
        vmull_vmlal     q4,  d17, d20, d1[3], d1[2] // -> t22a
        vqrshrn.s32     d22, q2,  #12               // t21a
        vqrshrn.s32     d18, q3,  #12               // t26a
        vneg.s32        q4,  q4                     // -> t22a
        vmull_vmlsl     q1,  d17, d20, d1[2], d1[3] // -> t25a
        vqrshrn.s32     d17, q4,  #12               // t22a
        vqrshrn.s32     d20, q1,  #12               // t25a

        vqsub.s16       d2,  d27, d24 // t29
        vqadd.s16       d27, d27, d24 // t30
        vqsub.s16       d3,  d21, d19 // t18
        vqadd.s16       d21, d21, d19 // t17
        vqsub.s16       d24, d16, d28 // t19a
        vqadd.s16       d16, d16, d28 // t16a
        vqsub.s16       d19, d30, d23 // t20a
        vqadd.s16       d30, d30, d23 // t23a
        vqsub.s16       d28, d17, d22 // t21
        vqadd.s16       d17, d17, d22 // t22
        vqadd.s16       d23, d26, d29 // t24a
        vqsub.s16       d26, d26, d29 // t27a
        vqadd.s16       d22, d20, d18 // t25
        vqsub.s16       d20, d20, d18 // t26
        vqsub.s16       d29, d31, d25 // t28a
        vqadd.s16       d31, d31, d25 // t31a

        vmull_vmlsl     q2,  d2,  d3,  d0[2], d0[3] // -> t18a
        vmull_vmlal     q3,  d2,  d3,  d0[3], d0[2] // -> t29a
        vmull_vmlsl     q4,  d29, d24, d0[2], d0[3] // -> t19
        vqrshrn.s32     d18, q2,  #12               // t18a
        vqrshrn.s32     d25, q3,  #12               // t29a
        vmull_vmlal     q1,  d29, d24, d0[3], d0[2] // -> t28
        vmull_vmlal     q2,  d26, d19, d0[3], d0[2] // -> t20
        vqrshrn.s32     d29, q4,  #12               // t19
        vqrshrn.s32     d24, q1,  #12               // t28
        vneg.s32        q2,  q2                     // -> t20
        vmull_vmlsl     q3,  d26, d19, d0[2], d0[3] // -> t27
        vmull_vmlal     q4,  d20, d28, d0[3], d0[2] // -> t21a
        vqrshrn.s32     d26, q2,  #12               // t20
        vqrshrn.s32     d19, q3,  #12               // t27
        vneg.s32        q4,  q4                     // -> t21a
        vmull_vmlsl     q1,  d20, d28, d0[2], d0[3] // -> t26a
        vqrshrn.s32     d20, q4,  #12               // t21a
        vqrshrn.s32     d28, q1,  #12               // t26a

        vqsub.s16       d2,  d16, d30 // t23
        vqadd.s16       d16, d16, d30 // t16 = out16
        vqsub.s16       d3,  d31, d23 // t24
        vqadd.s16       d31, d31, d23 // t31 = out31
        vqsub.s16       d23, d21, d17 // t22a
        vqadd.s16       d17, d21, d17 // t17a = out17
        vqadd.s16       d30, d27, d22 // t30a = out30
        vqsub.s16       d21, d27, d22 // t25a
        vqsub.s16       d27, d18, d20 // t21
        vqadd.s16       d18, d18, d20 // t18 = out18
        vqadd.s16       d4,  d29, d26 // t19a = out19
        vqsub.s16       d26, d29, d26 // t20a
        vqadd.s16       d29, d25, d28 // t29 = out29
        vqsub.s16       d25, d25, d28 // t26
        vqadd.s16       d28, d24, d19 // t28a = out28
        vqsub.s16       d24, d24, d19 // t27a
        vmov            d19, d4       // out19

        vmull_vmlsl     q2,  d24, d26, d0[0], d0[0] // -> t20
        vmull_vmlal     q3,  d24, d26, d0[0], d0[0] // -> t27
        vqrshrn.s32     d20, q2,  #12   // t20
        vqrshrn.s32     d22, q3,  #12   // t27

        vmull_vmlal     q2,  d25, d27, d0[0], d0[0] // -> t26a
        vmull_vmlsl     q3,  d25, d27, d0[0], d0[0] // -> t21a
        vmov            d27, d22        // t27
        vqrshrn.s32     d26, q2,  #12   // t26a

        vmull_vmlsl     q12, d21, d23, d0[0], d0[0] // -> t22
        vmull_vmlal     q2,  d21, d23, d0[0], d0[0] // -> t25
        vqrshrn.s32     d21, q3,  #12   // t21a
        vqrshrn.s32     d22, q12, #12   // t22
        vqrshrn.s32     d25, q2,  #12   // t25

        vmull_vmlsl     q2,  d3,  d2,  d0[0], d0[0] // -> t23a
        vmull_vmlal     q3,  d3,  d2,  d0[0], d0[0] // -> t24a
        vqrshrn.s32     d23, q2,  #12   // t23a
        vqrshrn.s32     d24, q3,  #12   // t24a

        bx              lr
endfunc

.macro def_horz_32 scale=0, shift=2, suffix
function inv_txfm_horz\suffix\()_dct_32x4_neon
        push            {lr}
        vmov.i16        d7,  #0
        lsl             r8,  r8,  #1
.if \scale
        movw            r12, #2896*8
        vdup.16         d0,  r12
.endif

.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.16         {\i}, [r7, :64]
        vst1.16         {d7}, [r7, :64], r8
.endr
        sub             r7,  r7,  r8, lsl #4
        add             r7,  r7,  r8, lsr #1
.if \scale
        scale_input     d0[0], q8,  q9,  q10, q11, q12, q13, q14, q15
.endif
        bl              inv_dct_4h_x16_neon
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q12, q13, d24, d25, d26, d27
        transpose_4x4h  q14, q15, d28, d29, d30, d31

.macro store1 r0, r1, r2, r3
        vst1.16         {\r0}, [r6, :64]!
        vst1.16         {\r1}, [r6, :64]!
        vst1.16         {\r2}, [r6, :64]!
        vst1.16         {\r3}, [r6, :64]!
        add             r6,  r6,  #32
.endm
        store1          d16, d20, d24, d28
        store1          d17, d21, d25, d29
        store1          d18, d22, d26, d30
        store1          d19, d23, d27, d31
.purgem store1
        sub             r6,  r6,  #64*4

        vmov.i16        d7,  #0
.irp i, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25, d26, d27, d28, d29, d30, d31
        vld1.16         {\i}, [r7, :64]
        vst1.16         {d7}, [r7, :64], r8
.endr
.if \scale
        // This relies on the fact that the idct also leaves the right coeff in d0[1]
        scale_input     d0[1], q8,  q9,  q10, q11, q12, q13, q14, q15
.endif
        bl              inv_dct32_odd_4h_x16_neon
        transpose_4x4h  q15, q14, d31, d30, d29, d28
        transpose_4x4h  q13, q12, d27, d26, d25, d24
        transpose_4x4h  q11, q10, d23, d22, d21, d20
        transpose_4x4h  q9,  q8,  d19, d18, d17, d16
.macro store2 r0, r1, r2, r3, shift
        vld1.16         {q0, q1}, [r6, :128]
        vqsub.s16       d7,  d0,  \r0
        vqadd.s16       d0,  d0,  \r0
        vqsub.s16       d6,  d1,  \r1
        vqadd.s16       d1,  d1,  \r1
        vqsub.s16       d5,  d2,  \r2
        vqadd.s16       d2,  d2,  \r2
        vqsub.s16       d4,  d3,  \r3
        vqadd.s16       d3,  d3,  \r3
        vrev64.16       q2,  q2
        vrev64.16       q3,  q3
        vrshr.s16       q0,  q0,  #\shift
        vrshr.s16       q1,  q1,  #\shift
        vrshr.s16       q2,  q2,  #\shift
        vrshr.s16       q3,  q3,  #\shift
        vst1.16         {q0, q1}, [r6, :128]!
        vst1.16         {q2, q3}, [r6, :128]!
.endm

        store2          d31, d27, d23, d19, \shift
        store2          d30, d26, d22, d18, \shift
        store2          d29, d25, d21, d17, \shift
        store2          d28, d24, d20, d16, \shift
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

        bl              inv_dct_4h_x16_neon

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
        bl              inv_dct32_odd_4h_x16_neon

        neg             r9,  r8
        mov             r10, r6
.macro combine r0, r1, r2, r3, op, stride
        vld1.16         {d4},    [r7,  :64], \stride
        vld1.32         {d2[0]}, [r10, :32], r1
        vld1.16         {d5},    [r7,  :64],  \stride
        vld1.32         {d2[1]}, [r10, :32], r1
        \op\().s16      d4,  d4,  \r0
        vld1.16         {d6},    [r7,  :64], \stride
        vld1.32         {d3[0]}, [r10, :32], r1
        \op\().s16      d5,  d5,  \r1
        vld1.32         {d3[1]}, [r10, :32], r1
        vrshr.s16       q2,  q2,  #4
        \op\().s16      d6,  d6,  \r2
        vld1.16         {d7},    [r7,  :64], \stride
        vaddw.u8        q2,  q2,  d2
        \op\().s16      d7,  d7,  \r3
        vqmovun.s16     d2,  q2
        vrshr.s16       q3,  q3,  #4
        vst1.32         {d2[0]}, [r6,  :32], r1
        vaddw.u8        q3,  q3,  d3
        vst1.32         {d2[1]}, [r6,  :32], r1
        vqmovun.s16     d3,  q3
        vst1.32         {d3[0]}, [r6,  :32], r1
        vst1.32         {d3[1]}, [r6,  :32], r1
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
        .short 10, 36, 78, 136, 210, 300, 406, 1024
endconst

const eob_16x32
        .short 10, 36, 78, 151, 215, 279, 343, 512
endconst

const eob_16x32_shortside
        .short 10, 36, 78, 512
endconst

const eob_8x32
        // Contrary to the others, this one is only ever used in increments of 8x8
        .short 43, 107, 171, 256
endconst

function inv_txfm_add_identity_identity_32x32_8bpc_neon, export=1
        push            {r4-r7,lr}
        vmov.i16        q0,  #0
        movrel_local    r5,  eob_32x32, 2

        mov             r6,  #2*32
1:
        mov             r12, #0
        movrel_local    r4,  eob_32x32, 2
2:
        add             r12, r12, #8
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\i}, [r2, :128]
        vst1.16         {q0}, [r2, :128], r6
.endr
        transpose_8x8h  q8,  q9,  q10, q11, q12, q13, q14, q15, d17, d19, d21, d23, d24, d26, d28, d30

        load_add_store_8x8 r0, r7, shiftbits=2
        ldrh            lr,  [r4], #4
        sub             r0,  r0,  r1, lsl #3
        cmp             r3,  lr
        add             r0,  r0,  #8
        bge             2b

        ldrh            lr,  [r5], #4
        cmp             r3,  lr
        blt             9f

        sub             r0,  r0,  r12
        add             r0,  r0,  r1, lsl #3
        mls             r2,  r6,  r12, r2
        add             r2,  r2,  #2*8
        b               1b
9:
        pop             {r4-r7,pc}
endfunc

.macro shift_8_regs op, shift
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        \op             \i,  \i,  #\shift
.endr
.endm

.macro def_identity_1632 w, h, wshort, hshort
function inv_txfm_add_identity_identity_\w\()x\h\()_8bpc_neon, export=1
        push            {r4-r7,lr}
        movw            r6,  #2896*8
        movw            r7,  #2*(5793-4096)*8
        vdup.i16        d0,  r6
        movrel_local    r5,  eob_16x32\hshort, 2
        vmov.16         d0[1], r7

        mov             r6,  #2*\h
1:
        mov             r12, #0
        movrel_local    r4,  eob_16x32\wshort, 2
2:
        vmov.i16        q1,  #0
        add             r12, r12, #8
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\i}, [r2, :128]
        vst1.16         {q1}, [r2, :128], r6
.endr
        scale_input     d0[0], q8,  q9, q10, q11, q12, q13, q14, q15

.if \w == 16
        // 16x32
        identity_8x8_shift1 d0[1]
.else
        // 32x16
        shift_8_regs    vqshl.s16, 1
        identity_8x8    d0[1]
.endif

        transpose_8x8h  q8,  q9,  q10, q11, q12, q13, q14, q15, d17, d19, d21, d23, d24, d26, d28, d30

.if \w == 16
        load_add_store_8x8 r0, r7, shiftbits=2
.else
        load_add_store_8x8 r0, r7, shiftbits=4
.endif
        ldrh            lr,  [r4], #4
        sub             r0,  r0,  r1, lsl #3
        cmp             r3,  lr
        add             r0,  r0,  #8
        bge             2b

        ldrh            lr,  [r5], #4
        cmp             r3,  lr
        blt             9f

        sub             r0,  r0,  r12
        add             r0,  r0,  r1, lsl #3
        mls             r2,  r6,  r12, r2
        add             r2,  r2,  #2*8
        b               1b
9:
        pop             {r4-r7,pc}
endfunc
.endm

def_identity_1632 16, 32, _shortside,
def_identity_1632 32, 16, , _shortside

.macro def_identity_832 w, h
function inv_txfm_add_identity_identity_\w\()x\h\()_8bpc_neon, export=1
        push            {r4-r5,lr}
        vmov.i16        q0,  #0
        movrel_local    r4,  eob_8x32

        mov             r12, #2*\h
1:
        ldrh            lr,  [r4], #2
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\i}, [r2, :128]
        vst1.16         {q0}, [r2, :128], r12
.endr

.if \w == 8
        // 8x32
        shift_8_regs    vrshr.s16, 1
.endif

        transpose_8x8h  q8,  q9,  q10, q11, q12, q13, q14, q15, d17, d19, d21, d23, d24, d26, d28, d30

        cmp             r3,  lr
.if \w == 8
        load_add_store_8x8 r0, r5, shiftbits=2
.else
        load_add_store_8x8 r0, r5, shiftbits=3
.endif

        blt             9f
.if \w == 8
        sub             r2,  r2,  r12, lsl #3
        add             r2,  r2,  #2*8
.else
        sub             r0,  r0,  r1, lsl #3
        add             r0,  r0,  #8
.endif
        b               1b

9:
        pop             {r4-r5,pc}
endfunc
.endm

def_identity_832 8, 32
def_identity_832 32, 8

function inv_txfm_add_dct_dct_32x32_8bpc_neon, export=1
        idct_dc         32,  32,  2

        push            {r4-r11,lr}
        vpush           {q4}
        sub_sp_align    2048
        movrel_local    r10, eob_32x32
        ldrh            r11, [r10], #2

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  sp,  #(\i*32*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 28
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_horz_dct_32x4_neon
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
        add             r6,  r0,  #(\i)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    2048
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_16x32_8bpc_neon, export=1
        idct_dc         16,  32,  1

        push            {r4-r11,lr}
        vpush           {q4}
        sub_sp_align    1024
        movrel_local    r10, eob_16x32
        ldrh            r11, [r10], #2
        movrel_local    r4,  inv_dct_4h_x16_neon

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  sp,  #(\i*16*2)
        add             r7,  r2,  #(\i*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 28
        ldrh            r11, [r10], #2
.endif
.endif
        mov             r8,  #2*32
        bl              inv_txfm_horz_scale_16x4_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #4
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12
        add             r6,  r0,  #(\i)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #16*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    1024
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_32x16_8bpc_neon, export=1
        idct_dc         32,  16,  1

        push            {r4-r11,lr}
        vpush           {q4}
        sub_sp_align    1024
        movrel_local    r10, eob_16x32
        ldrh            r11, [r10], #2
        movrel_local    r5,  inv_dct_4h_x16_neon

.irp i, 0, 4, 8, 12
        add             r6,  sp,  #(\i*32*2)
        add             r7,  r2,  #(\i*2)
.if \i > 0
        mov             r8,  #(16 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 12
        ldrh            r11, [r10], #2
.endif
.endif
        mov             r8,  #2*16
        bl              inv_txfm_horz_scale_dct_32x4_neon
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
        add             r6,  r0,  #(\i)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    1024
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_8x32_8bpc_neon, export=1
        idct_dc         8,   32,  2

        push            {r4-r11,lr}
        vpush           {q4-q7}
        sub_sp_align    512

        movrel_local    r10, eob_8x32

        mov             r8,  #2*32
        mov             r9,  #32
        mov             r6,  sp
1:
        vmov.i16        q0,  #0
.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\i}, [r2, :128]
        vst1.16         {q0}, [r2, :128], r8
.endr
        ldrh            r11, [r10], #2
        sub             r2,  r2,  r8, lsl #3
        sub             r9,  r9,  #8
        add             r2,  r2,  #2*8

        bl              inv_dct_8h_x8_neon

.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vrshr.s16       \i,  \i,  #2
.endr

        transpose_8x8h  q8,  q9,  q10, q11, q12, q13, q14, q15, d17, d19, d21, d23, d24, d26, d28, d30

        vst1.16         {q8,  q9},  [r6, :128]!
        cmp             r3,  r11
        vst1.16         {q10, q11}, [r6, :128]!
        vst1.16         {q12, q13}, [r6, :128]!
        vst1.16         {q14, q15}, [r6, :128]!

        bge             1b
        cmp             r9,  #0
        beq             3f

        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r9,  r9,  #8
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4
        add             r6,  r0,  #(\i)
        add             r7,  sp,  #(\i*2)
        mov             r8,  #8*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    512
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_32x8_8bpc_neon, export=1
        idct_dc         32,  8,   2

        push            {r4-r11,lr}
        vpush           {q4-q7}
        sub_sp_align    512

.irp i, 0, 4
        add             r6,  sp,  #(\i*32*2)
        add             r7,  r2,  #(\i*2)
.if \i > 0
        cmp             r3,  #10
        blt             1f
.endif
        mov             r8,  #8*2
        bl              inv_txfm_horz_dct_32x4_neon
.endr
        b               2f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
.rept 8
        vst1.16         {q2, q3}, [r6, :128]!
.endr

2:
        mov             r8,  #2*32
        mov             r9,  #0
1:
        add             r6,  r0,  r9
        add             r7,  sp,  r9, lsl #1 // #(\i*2)

.irp i, q8, q9, q10, q11, q12, q13, q14, q15
        vld1.16         {\i}, [r7, :128], r8
.endr
        add             r9,  r9,  #8

        bl              inv_dct_8h_x8_neon

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

        vld1.16         {d0, d1, d2}, [r12, :64]!

        vqrdmulh.s16    d23, d16, d0[1]  // t63a
        vqrdmulh.s16    d16, d16, d0[0]  // t32a
        vqrdmulh.s16    d22, d17, d0[2]  // t62a
        vqrdmulh.s16    d17, d17, d0[3]  // t33a
        vqrdmulh.s16    d21, d18, d1[1]  // t61a
        vqrdmulh.s16    d18, d18, d1[0]  // t34a
        vqrdmulh.s16    d20, d19, d1[2]  // t60a
        vqrdmulh.s16    d19, d19, d1[3]  // t35a

        vqadd.s16       d24, d16, d17    // t32
        vqsub.s16       d25, d16, d17    // t33
        vqsub.s16       d26, d19, d18    // t34
        vqadd.s16       d27, d19, d18    // t35
        vqadd.s16       d28, d20, d21    // t60
        vqsub.s16       d29, d20, d21    // t61
        vqsub.s16       d30, d23, d22    // t62
        vqadd.s16       d31, d23, d22    // t63

        vmull_vmlal     q2,  d29, d26, d2[0], d2[1] // -> t34a
        vmull_vmlsl     q3,  d29, d26, d2[1], d2[0] // -> t61a
        vneg.s32        q2,  q2                     // t34a
        vmull_vmlsl     q4,  d30, d25, d2[1], d2[0] // -> t33a
        vqrshrn.s32     d26, q2,  #12               // t34a
        vmull_vmlal     q2,  d30, d25, d2[0], d2[1] // -> t62a
        vqrshrn.s32     d29, q3,  #12               // t61a
        vqrshrn.s32     d25, q4,  #12               // t33a
        vqrshrn.s32     d30, q2,  #12               // t62a

        vqadd.s16       d16, d24, d27    // t32a
        vqsub.s16       d19, d24, d27    // t35a
        vqadd.s16       d17, d25, d26    // t33
        vqsub.s16       d18, d25, d26    // t34
        vqsub.s16       d20, d31, d28    // t60a
        vqadd.s16       d23, d31, d28    // t63a
        vqsub.s16       d21, d30, d29    // t61
        vqadd.s16       d22, d30, d29    // t62

        vmull_vmlal     q2,  d21, d18, d2[2], d2[3] // -> t61a
        vmull_vmlsl     q3,  d21, d18, d2[3], d2[2] // -> t34a
        vmull_vmlal     q4,  d20, d19, d2[2], d2[3] // -> t60
        vqrshrn.s32     d21, q2,  #12               // t61a
        vqrshrn.s32     d18, q3,  #12               // t34a
        vmull_vmlsl     q2,  d20, d19, d2[3], d2[2] // -> t35
        vqrshrn.s32     d20, q4,  #12               // t60
        vqrshrn.s32     d19, q2,  #12               // t35

        vst1.16         {d16, d17, d18, d19}, [r6, :128]!
        vst1.16         {d20, d21, d22, d23}, [r6, :128]!

        bx              lr
endfunc

function inv_dct64_step2_neon
        movrel_local    r12, idct_coeffs
        vld1.16         {d0}, [r12, :64]
1:
        // t32a/33/34a/35/60/61a/62/63a
        // t56a/57/58a/59/36/37a/38/39a
        // t40a/41/42a/43/52/53a/54/55a
        // t48a/49/50a/51/44/45a/46/47a
        vldr            d16, [r6, #2*4*0]  // t32a
        vldr            d17, [r9, #2*4*8]  // t39a
        vldr            d18, [r9, #2*4*0]  // t63a
        vldr            d19, [r6, #2*4*8]  // t56a
        vldr            d20, [r6, #2*4*16] // t40a
        vldr            d21, [r9, #2*4*24] // t47a
        vldr            d22, [r9, #2*4*16] // t55a
        vldr            d23, [r6, #2*4*24] // t48a

        vqadd.s16       d24, d16, d17      // t32
        vqsub.s16       d25, d16, d17      // t39
        vqadd.s16       d26, d18, d19      // t63
        vqsub.s16       d27, d18, d19      // t56
        vqsub.s16       d28, d21, d20      // t40
        vqadd.s16       d29, d21, d20      // t47
        vqadd.s16       d30, d23, d22      // t48
        vqsub.s16       d31, d23, d22      // t55

        vmull_vmlal     q2,  d27, d25, d0[3], d0[2] // -> t56a
        vmull_vmlsl     q3,  d27, d25, d0[2], d0[3] // -> t39a
        vmull_vmlal     q4,  d31, d28, d0[3], d0[2] // -> t40a
        vqrshrn.s32     d25, q2,  #12               // t56a
        vqrshrn.s32     d27, q3,  #12               // t39a
        vneg.s32        q4,  q4                     // t40a
        vmull_vmlsl     q2,  d31, d28, d0[2], d0[3] // -> t55a
        vqrshrn.s32     d31, q4,  #12               // t40a
        vqrshrn.s32     d28, q2,  #12               // t55a

        vqadd.s16       d16, d24, d29      // t32a
        vqsub.s16       d19, d24, d29      // t47a
        vqadd.s16       d17, d27, d31      // t39
        vqsub.s16       d18, d27, d31      // t40
        vqsub.s16       d20, d26, d30      // t48a
        vqadd.s16       d23, d26, d30      // t63a
        vqsub.s16       d21, d25, d28      // t55
        vqadd.s16       d22, d25, d28      // t56

        vmull_vmlsl     q2,  d21, d18, d0[0], d0[0] // -> t40a
        vmull_vmlal     q3,  d21, d18, d0[0], d0[0] // -> t55a
        vmull_vmlsl     q4,  d20, d19, d0[0], d0[0] // -> t47
        vqrshrn.s32     d18, q2,  #12               // t40a
        vqrshrn.s32     d21, q3,  #12               // t55a
        vmull_vmlal     q2,  d20, d19, d0[0], d0[0] // -> t48
        vqrshrn.s32     d19, q4,  #12               // t47
        vqrshrn.s32     d20, q2,  #12               // t48

        vstr            d16, [r6, #2*4*0]  // t32a
        vstr            d17, [r9, #2*4*0]  // t39
        vstr            d18, [r6, #2*4*8]  // t40a
        vstr            d19, [r9, #2*4*8]  // t47
        vstr            d20, [r6, #2*4*16] // t48
        vstr            d21, [r9, #2*4*16] // t55a
        vstr            d22, [r6, #2*4*24] // t56
        vstr            d23, [r9, #2*4*24] // t63a

        add             r6,  r6,  #2*4
        sub             r9,  r9,  #2*4
        cmp             r6,  r9
        blt             1b
        bx              lr
endfunc

.macro load8 src, strd, zero, clear
.irp i, d16, d17, d18, d19, d20, d21, d22, d23
.if \clear
        vld1.16         {\i}, [\src, :64]
        vst1.16         {\zero}, [\src, :64], \strd
.else
        vld1.16         {\i}, [\src, :64], \strd
.endif
.endr
.endm

.macro store16 dst
        vst1.16         {q8,  q9},  [\dst, :128]!
        vst1.16         {q10, q11}, [\dst, :128]!
        vst1.16         {q12, q13}, [\dst, :128]!
        vst1.16         {q14, q15}, [\dst, :128]!
.endm

.macro clear_upper8
.irp i, q12, q13, q14, q15
        vmov.i16        \i,  #0
.endr
.endm

.macro vmov_if reg, val, cond
.if \cond
        vmov.i16        \reg, \val
.endif
.endm

.macro movdup_if reg, gpr, val, cond
.if \cond
        movw            \gpr, \val
        vdup.16         \reg, \gpr
.endif
.endm

.macro vst1_if regs, dst, dstalign, cond
.if \cond
        vst1.16         \regs, \dst, \dstalign
.endif
.endm

.macro scale_if cond, c, r0, r1, r2, r3, r4, r5, r6, r7
.if \cond
        scale_input     \c, \r0, \r1, \r2, \r3, \r4, \r5, \r6, \r7
.endif
.endm

.macro def_dct64_func suffix, clear=0, scale=0
function inv_txfm_dct\suffix\()_4h_x64_neon, export=1
        mov             r6,  sp

        push            {r10-r11,lr}

        lsl             r8,  r8,  #2

        movdup_if       d0,  r12, #2896*8, \scale
        vmov_if         d7,  #0,  \clear
        load8           r7,  r8,  d7,  \clear
        clear_upper8
        sub             r7,  r7,  r8, lsl #3
        add             r7,  r7,  r8, lsr #1
        scale_if        \scale, d0[0], q8, q9, q10, q11

        bl              inv_dct_4h_x16_neon

        store16         r6

        movdup_if       d0,  r12, #2896*8, \scale
        vmov_if         d7,  #0,  \clear
        load8           r7,  r8,  d7,  \clear
        clear_upper8
        sub             r7,  r7,  r8, lsl #3
        lsr             r8,  r8,  #1
        sub             r7,  r7,  r8, lsr #1
        scale_if        \scale, d0[0], q8, q9, q10, q11

        bl              inv_dct32_odd_4h_x16_neon

        add             r10, r6,  #8*15
        sub             r6,  r6,  #8*16

        mov             r9,  #-8

.macro store_addsub r0, r1, r2, r3
        vld1.16         {d2},  [r6, :64]!
        vld1.16         {d3},  [r6, :64]!
        vqadd.s16       d6,  d2,  \r0
        vqsub.s16       \r0, d2,  \r0
        vld1.16         {d4},  [r6, :64]!
        vqadd.s16       d7,  d3,  \r1
        vqsub.s16       \r1, d3,  \r1
        vld1.16         {d5},  [r6, :64]!
        vqadd.s16       d2,  d4,  \r2
        sub             r6,  r6,  #8*4
        vqsub.s16       \r2, d4,  \r2
        vst1.16         {d6},  [r6,  :64]!
        vst1.16         {\r0}, [r10, :64], r9
        vqadd.s16       d3,  d5,  \r3
        vqsub.s16       \r3, d5,  \r3
        vst1.16         {d7},  [r6,  :64]!
        vst1.16         {\r1}, [r10, :64], r9
        vst1.16         {d2},  [r6,  :64]!
        vst1.16         {\r2}, [r10, :64], r9
        vst1.16         {d3},  [r6,  :64]!
        vst1.16         {\r3}, [r10, :64], r9
.endm
        store_addsub    d31, d30, d29, d28
        store_addsub    d27, d26, d25, d24
        store_addsub    d23, d22, d21, d20
        store_addsub    d19, d18, d17, d16
.purgem store_addsub

        add             r6,  r6,  #2*4*16

        movrel_local    r12, idct64_coeffs
        movdup_if       d0,  lr,  #2896*8, \scale
        vmov_if         d7,  #0,  \clear
        add             r9,  r7,  r8, lsl #4 // offset 16
        add             r10, r7,  r8, lsl #3 // offset 8
        sub             r9,  r9,  r8         // offset 15
        sub             r11, r10, r8         // offset 7
        vld1.16         {d16}, [r7,  :64]    // in1  (offset 0)
        vld1.16         {d17}, [r9,  :64]    // in31 (offset 15)
        vld1.16         {d18}, [r10, :64]    // in17 (offset 8)
        vld1.16         {d19}, [r11, :64]    // in15 (offset 7)
        vst1_if         {d7},  [r7,  :64], \clear
        vst1_if         {d7},  [r9,  :64], \clear
        vst1_if         {d7},  [r10, :64], \clear
        vst1_if         {d7},  [r11, :64], \clear
        scale_if        \scale, d0[0], q8, q9
        bl              inv_dct64_step1_neon
        movdup_if       d0,  lr,  #2896*8, \scale
        vmov_if         d7,  #0,  \clear
        add             r7,  r7,  r8, lsl #2 // offset 4
        sub             r9,  r9,  r8, lsl #2 // offset 11
        sub             r10, r7,  r8         // offset 3
        add             r11, r9,  r8         // offset 12
        vld1.16         {d16}, [r10, :64]    // in7  (offset 3)
        vld1.16         {d17}, [r11, :64]    // in25 (offset 12)
        vld1.16         {d18}, [r9,  :64]    // in23 (offset 11)
        vld1.16         {d19}, [r7,  :64]    // in9  (offset 4)
        vst1_if         {d7},  [r7,  :64], \clear
        vst1_if         {d7},  [r9,  :64], \clear
        vst1_if         {d7},  [r10, :64], \clear
        vst1_if         {d7},  [r11, :64], \clear
        scale_if        \scale, d0[0], q8, q9
        bl              inv_dct64_step1_neon
        movdup_if       d0,  lr,  #2896*8, \scale
        vmov_if         d7,  #0,  \clear
        sub             r10, r10, r8, lsl #1 // offset 1
        sub             r9,  r9,  r8, lsl #1 // offset 9
        add             r10, r10, r8         // offset 2
        add             r9,  r9,  r8         // offset 10
        add             r7,  r7,  r8         // offset 5
        add             r11, r11, r8         // offset 13
        vld1.16         d16, [r10, :64]      // in5  (offset 2)
        vld1.16         d17, [r11, :64]      // in27 (offset 13)
        vld1.16         d18, [r9,  :64]      // in21 (offset 10)
        vld1.16         d19, [r7,  :64]      // in11 (offset 5)
        vst1_if         d7,  [r10, :64], \clear
        vst1_if         d7,  [r11, :64], \clear
        vst1_if         d7,  [r9,  :64], \clear
        vst1_if         d7,  [r7,  :64], \clear
        scale_if        \scale, d0[0], q8, q9
        bl              inv_dct64_step1_neon
        movdup_if       d0,  lr,  #2896*8, \scale
        vmov_if         d7,  #0,  \clear
        sub             r10, r10, r8         // offset 1
        sub             r9,  r9,  r8         // offset 9
        add             r11, r11, r8         // offset 14
        add             r7,  r7,  r8         // offset 6
        vld1.16         d16, [r10, :64]      // in3  (offset 1)
        vld1.16         d17, [r11, :64]      // in29 (offset 14)
        vld1.16         d18, [r9,  :64]      // in19 (offset 9)
        vld1.16         d19, [r7,  :64]      // in13 (offset 6)
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

def_dct64_func
def_dct64_func _clear, clear=1
def_dct64_func _clear_scale, clear=1, scale=1

function inv_txfm_horz_dct_64x4_neon
        vdup.16         q3,  r9

        mov             r7,  sp
        add             r8,  sp,  #2*4*(64 - 4)
        add             r9,  r6,  #2*56

        push            {r10-r11,lr}

        mov             r10, #2*64
        mov             r11, #-2*4*4

1:
        vld1.16         {d16, d17, d18, d19}, [r7, :128]!
        vld1.16         {d28, d29, d30, d31}, [r8, :128], r11
        vld1.16         {d20, d21, d22, d23}, [r7, :128]!
        vld1.16         {d24, d25, d26, d27}, [r8, :128], r11
        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q15, q14, d31, d30, d29, d28
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q13, q12, d27, d26, d25, d24

.macro store_addsub src0, src1, src2, src3
        vqsub.s16       d3,  \src0,  \src1
        vqsub.s16       d2,  \src2,  \src3
        vqadd.s16       d0,  \src0,  \src1
        vqadd.s16       d1,  \src2,  \src3
        vrshl.s16       q1,  q1,  q3
        vrshl.s16       q0,  q0,  q3
        vrev64.16       q1,  q1
        vst1.16         {q0},  [r6, :128], r10
        vst1.16         {q1},  [r9, :128], r10
.endm
        store_addsub    d16, d31, d20, d27
        store_addsub    d17, d30, d21, d26
        store_addsub    d18, d29, d22, d25
        store_addsub    d19, d28, d23, d24
.purgem store_addsub
        sub             r6,  r6,  r10, lsl #2
        sub             r9,  r9,  r10, lsl #2
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

.macro add_dest_addsub src0, src1, src2, src3
        vld1.32         {d0[0]}, [r6, :32], r1
        vld1.32         {d1[0]}, [r9, :32], r10
        vqadd.s16       d4,  \src0,  \src1
        vld1.32         {d0[1]}, [r6, :32]
        vqadd.s16       d5,  \src2,  \src3
        vld1.32         {d1[1]}, [r9, :32]
        vqsub.s16       d6,  \src0,  \src1
        vqsub.s16       d7,  \src2,  \src3
        sub             r6,  r6,  r1
        sub             r9,  r9,  r10
        vrshr.s16       q2,  q2,  #4
        vrshr.s16       q3,  q3,  #4
        vaddw.u8        q2,  q2,  d0
        vaddw.u8        q3,  q3,  d1
        vqmovun.s16     d0,  q2
        vqmovun.s16     d1,  q3
        vst1.32         {d0[0]}, [r6, :32], r1
        vst1.32         {d1[0]}, [r9, :32], r10
        vst1.32         {d0[1]}, [r6, :32], r1
        vst1.32         {d1[1]}, [r9, :32], r10
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

function inv_txfm_add_dct_dct_64x64_8bpc_neon, export=1
        idct_dc         64,  64,  2

        push            {r4-r11,lr}
        vpush           {q4}

        sub_sp_align    64*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_32x32

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  r5,  #(\i*64*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_dct_clear_4h_x64_neon
        add             r6,  r5,  #(\i*64*2)
        mov             r9,  #-2 // shift
        bl              inv_txfm_horz_dct_64x4_neon
.if \i < 28
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
        bl              inv_txfm_dct_4h_x64_neon
        add             r6,  r0,  #(\i)
        bl              inv_txfm_add_vert_dct_4x64_neon
.endr

        add_sp_align    64*32*2+64*4*2
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_64x32_8bpc_neon, export=1
        idct_dc         64,  32,  1

        push            {r4-r11,lr}
        vpush           {q4}

        sub_sp_align    64*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_32x32

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  r5,  #(\i*64*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_dct_clear_scale_4h_x64_neon
        add             r6,  r5,  #(\i*64*2)
        mov             r9,  #-1 // shift
        bl              inv_txfm_horz_dct_64x4_neon
.if \i < 28
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
        add             r6,  r0,  #(\i)
        add             r7,  r5,  #(\i*2)
        mov             r8,  #64*2
        bl              inv_txfm_add_vert_dct_4x32_neon
.endr

        add_sp_align    64*32*2+64*4*2
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_32x64_8bpc_neon, export=1
        idct_dc         32,  64,  1

        push            {r4-r11,lr}
        vpush           {q4}

        sub_sp_align    32*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_32x32
        ldrh            r11, [r10], #2

.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  r5,  #(\i*32*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 28
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_horz_scale_dct_32x4_neon
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
        bl              inv_txfm_dct_4h_x64_neon
        add             r6,  r0,  #(\i)
        bl              inv_txfm_add_vert_dct_4x64_neon
.endr

        add_sp_align    32*32*2+64*4*2
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_64x16_8bpc_neon, export=1
        idct_dc         64,  16,  2

        push            {r4-r11,lr}
        vpush           {q4}

        sub_sp_align    64*16*2+64*4*2
        add             r4,  sp,  #64*4*2

        movrel_local    r10, eob_16x32

.irp i, 0, 4, 8, 12
        add             r6,  r4,  #(\i*64*2)
.if \i > 0
        mov             r8,  #(16 - \i)
        cmp             r3,  r11
        blt             1f
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #16*2
        bl              inv_txfm_dct_clear_4h_x64_neon
        add             r6,  r4,  #(\i*64*2)
        mov             r9,  #-2 // shift
        bl              inv_txfm_horz_dct_64x4_neon
.if \i < 12
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
        movrel_local    r5,  inv_dct_4h_x16_neon
.irp i, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60
        add             r6,  r0,  #(\i)
        add             r7,  r4,  #(\i*2)
        mov             r8,  #64*2
        bl              inv_txfm_add_vert_4x16_neon
.endr

        add_sp_align    64*16*2+64*4*2
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc

function inv_txfm_add_dct_dct_16x64_8bpc_neon, export=1
        idct_dc         16,  64,  2

        push            {r4-r11,lr}
        vpush           {q4}

        sub_sp_align    16*32*2+64*4*2
        add             r5,  sp,  #64*4*2

        movrel_local    r10, eob_16x32
        ldrh            r11, [r10], #2

        movrel_local    r4,  inv_dct_4h_x16_neon
.irp i, 0, 4, 8, 12, 16, 20, 24, 28
        add             r6,  r5,  #(\i*16*2)
.if \i > 0
        mov             r8,  #(32 - \i)
        cmp             r3,  r11
        blt             1f
.if \i < 28
        ldrh            r11, [r10], #2
.endif
.endif
        add             r7,  r2,  #(\i*2)
        mov             r8,  #32*2
        bl              inv_txfm_horz_16x4_neon
.endr
        b               3f

1:
        vmov.i16        q2,  #0
        vmov.i16        q3,  #0
2:
        subs            r8,  r8,  #4
.rept 4
        vst1.16         {q2, q3}, [r6, :128]!
.endr
        bgt             2b

3:
.irp i, 0, 4, 8, 12
        add             r7,  r5,  #(\i*2)
        mov             r8,  #16*2
        bl              inv_txfm_dct_4h_x64_neon
        add             r6,  r0,  #(\i)
        bl              inv_txfm_add_vert_dct_4x64_neon
.endr

        add_sp_align    16*32*2+64*4*2
        vpop            {q4}
        pop             {r4-r11,pc}
endfunc
