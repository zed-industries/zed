/*
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
 */

#include "src/arm/asm.S"

#define FILTER_OUT_STRIDE 384

.macro sgr_funcs bpc
// void dav1d_sgr_finish_filter1_Xbpc_neon(int16_t *tmp,
//                                         const pixel *src, const ptrdiff_t stride,
//                                         const int32_t *a, const int16_t *b,
//                                         const int w, const int h);
function sgr_finish_filter1_\bpc\()bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldr             r6,  [sp, #108]
        sub             r7,  r3,  #(4*SUM_STRIDE)
        add             r8,  r3,  #(4*SUM_STRIDE)
        sub             r9,  r4,  #(2*SUM_STRIDE)
        add             r10, r4,  #(2*SUM_STRIDE)
        mov             r11, #SUM_STRIDE
        mov             r12, #FILTER_OUT_STRIDE
        add             lr,  r5,  #3
        bic             lr,  lr,  #3 // Aligned width
.if \bpc == 8
        sub             r2,  r2,  lr
.else
        sub             r2,  r2,  lr, lsl #1
.endif
        sub             r12, r12, lr
        sub             r11, r11, lr
        sub             r11, r11, #4 // We read 4 extra elements from both a and b
        mov             lr,  r5
        vmov.i16        q14, #3
        vmov.i32        q15, #3
1:
        vld1.16         {q0},       [r9,  :128]!
        vld1.16         {q1},       [r4,  :128]!
        vld1.16         {q2},       [r10, :128]!
        vld1.32         {q8,  q9},  [r7,  :128]!
        vld1.32         {q10, q11}, [r3,  :128]!
        vld1.32         {q12, q13}, [r8,  :128]!

2:
        subs            r5,  r5,  #4
        vext.8          d6,  d0,  d1,  #2  // -stride
        vext.8          d7,  d2,  d3,  #2  // 0
        vext.8          d8,  d4,  d5,  #2  // +stride
        vext.8          d9,  d0,  d1,  #4  // +1-stride
        vext.8          d10, d2,  d3,  #4  // +1
        vext.8          d11, d4,  d5,  #4  // +1+stride
        vadd.i16        d2,  d2,  d6       // -1, -stride
        vadd.i16        d7,  d7,  d8       // 0, +stride
        vadd.i16        d0,  d0,  d9       // -1-stride, +1-stride
        vadd.i16        d2,  d2,  d7
        vadd.i16        d4,  d4,  d11      // -1+stride, +1+stride
        vadd.i16        d2,  d2,  d10      // +1
        vadd.i16        d0,  d0,  d4

        vext.8          q3,  q8,  q9,  #4  // -stride
        vshl.i16        d2,  d2,  #2
        vext.8          q4,  q8,  q9,  #8  // +1-stride
        vext.8          q5,  q10, q11, #4  // 0
        vext.8          q6,  q10, q11, #8  // +1
        vmla.i16        d2,  d0,  d28      // * 3 -> a
        vadd.i32        q3,  q3,  q10      // -stride, -1
        vadd.i32        q8,  q8,  q4       // -1-stride, +1-stride
        vadd.i32        q5,  q5,  q6       // 0, +1
        vadd.i32        q8,  q8,  q12      // -1+stride
        vadd.i32        q3,  q3,  q5
        vext.8          q7,  q12, q13, #4  // +stride
        vext.8          q10, q12, q13, #8  // +1+stride
.if \bpc == 8
        vld1.32         {d24[0]}, [r1, :32]! // src
.else
        vld1.16         {d24}, [r1, :64]!    // src
.endif
        vadd.i32        q3,  q3,  q7       // +stride
        vadd.i32        q8,  q8,  q10      // +1+stride
        vshl.i32        q3,  q3,  #2
        vmla.i32        q3,  q8,  q15      // * 3 -> b
.if \bpc == 8
        vmovl.u8        q12, d24           // src
.endif
        vmov            d0,  d1
        vmlal.u16       q3,  d2,  d24      // b + a * src
        vmov            d2,  d3
        vrshrn.i32      d6,  q3,  #9
        vmov            d4,  d5
        vst1.16         {d6}, [r0]!

        ble             3f
        vmov            q8,  q9
        vmov            q10, q11
        vmov            q12, q13
        vld1.16         {d1},  [r9,  :64]!
        vld1.16         {d3},  [r4,  :64]!
        vld1.16         {d5},  [r10, :64]!
        vld1.32         {q9},  [r7,  :128]!
        vld1.32         {q11}, [r3,  :128]!
        vld1.32         {q13}, [r8,  :128]!
        b               2b

3:
        subs            r6,  r6,  #1
        ble             0f
        mov             r5,  lr
        add             r0,  r0,  r12, lsl #1
        add             r1,  r1,  r2
        add             r3,  r3,  r11, lsl #2
        add             r7,  r7,  r11, lsl #2
        add             r8,  r8,  r11, lsl #2
        add             r4,  r4,  r11, lsl #1
        add             r9,  r9,  r11, lsl #1
        add             r10, r10, r11, lsl #1
        b               1b
0:
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

// void dav1d_sgr_finish_filter2_Xbpc_neon(int16_t *tmp,
//                                         const pixel *src, const ptrdiff_t stride,
//                                         const int32_t *a, const int16_t *b,
//                                         const int w, const int h);
function sgr_finish_filter2_\bpc\()bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldr             r6,  [sp, #108]
        add             r7,  r3,  #(4*(SUM_STRIDE))
        sub             r3,  r3,  #(4*(SUM_STRIDE))
        add             r8,  r4,  #(2*(SUM_STRIDE))
        sub             r4,  r4,  #(2*(SUM_STRIDE))
        mov             r9,  #(2*SUM_STRIDE)
        mov             r10, #FILTER_OUT_STRIDE
        add             r11, r5,  #7
        bic             r11, r11, #7 // Aligned width
.if \bpc == 8
        sub             r2,  r2,  r11
.else
        sub             r2,  r2,  r11, lsl #1
.endif
        sub             r10, r10, r11
        sub             r9,  r9,  r11
        sub             r9,  r9,  #4 // We read 4 extra elements from a
        sub             r12, r9,  #4 // We read 8 extra elements from b
        mov             lr,  r5

1:
        vld1.16         {q0,  q1},  [r4, :128]!
        vld1.16         {q2,  q3},  [r8, :128]!
        vld1.32         {q8,  q9},  [r3, :128]!
        vld1.32         {q11, q12}, [r7, :128]!
        vld1.32         {q10},      [r3, :128]!
        vld1.32         {q13},      [r7, :128]!

2:
        vmov.i16        q14, #5
        vmov.i16        q15, #6
        subs            r5,  r5,  #8
        vext.8          q4,  q0,  q1,  #4  // +1-stride
        vext.8          q5,  q2,  q3,  #4  // +1+stride
        vext.8          q6,  q0,  q1,  #2  // -stride
        vext.8          q7,  q2,  q3,  #2  // +stride
        vadd.i16        q0,  q0,  q4       // -1-stride, +1-stride
        vadd.i16        q5,  q2,  q5       // -1+stride, +1+stride
        vadd.i16        q2,  q6,  q7       // -stride, +stride
        vadd.i16        q0,  q0,  q5

        vext.8          q4,  q8,  q9,  #8  // +1-stride
        vext.8          q5,  q9,  q10, #8
        vext.8          q6,  q11, q12, #8  // +1+stride
        vext.8          q7,  q12, q13, #8
        vmul.i16        q0,  q0,  q14      // * 5
        vmla.i16        q0,  q2,  q15      // * 6
        vadd.i32        q4,  q4,  q8       // -1-stride, +1-stride
        vadd.i32        q5,  q5,  q9
        vadd.i32        q6,  q6,  q11      // -1+stride, +1+stride
        vadd.i32        q7,  q7,  q12
        vadd.i32        q4,  q4,  q6
        vadd.i32        q5,  q5,  q7
        vext.8          q6,  q8,  q9,  #4  // -stride
        vext.8          q7,  q9,  q10, #4
        vext.8          q8,  q11, q12, #4  // +stride
        vext.8          q11, q12, q13, #4

.if \bpc == 8
        vld1.8          {d4}, [r1, :64]!
.else
        vld1.8          {q2}, [r1, :128]!
.endif

        vmov.i32        q14, #5
        vmov.i32        q15, #6

        vadd.i32        q6,  q6,  q8       // -stride, +stride
        vadd.i32        q7,  q7,  q11
        vmul.i32        q4,  q4,  q14      // * 5
        vmla.i32        q4,  q6,  q15      // * 6
        vmul.i32        q5,  q5,  q14      // * 5
        vmla.i32        q5,  q7,  q15      // * 6

.if \bpc == 8
        vmovl.u8        q2,  d4
.endif
        vmlal.u16       q4,  d0,  d4       // b + a * src
        vmlal.u16       q5,  d1,  d5       // b + a * src
        vmov            q0,  q1
        vrshrn.i32      d8,  q4,  #9
        vrshrn.i32      d9,  q5,  #9
        vmov            q2,  q3
        vst1.16         {q4}, [r0, :128]!

        ble             3f
        vmov            q8,  q10
        vmov            q11, q13
        vld1.16         {q1},       [r4, :128]!
        vld1.16         {q3},       [r8, :128]!
        vld1.32         {q9,  q10}, [r3, :128]!
        vld1.32         {q12, q13}, [r7, :128]!
        b               2b

3:
        subs            r6,  r6,  #1
        ble             0f
        mov             r5,  lr
        add             r0,  r0,  r10, lsl #1
        add             r1,  r1,  r2
        add             r3,  r3,  r9,  lsl #2
        add             r7,  r7,  r9,  lsl #2
        add             r4,  r4,  r12, lsl #1
        add             r8,  r8,  r12, lsl #1

        vld1.32         {q8, q9}, [r3, :128]!
        vld1.16         {q0, q1}, [r4, :128]!
        vld1.32         {q10},    [r3, :128]!

        vmov.i16        q12, #5
        vmov.i16        q13, #6

4:
        subs            r5,  r5,  #8
        vext.8          q3,  q0,  q1,  #4  // +1
        vext.8          q2,  q0,  q1,  #2  // 0
        vadd.i16        q0,  q0,  q3       // -1, +1

        vext.8          q4,  q8,  q9,  #4  // 0
        vext.8          q5,  q9,  q10, #4
        vext.8          q6,  q8,  q9,  #8  // +1
        vext.8          q7,  q9,  q10, #8
        vmul.i16        q2,  q2,  q13      // * 6
        vmla.i16        q2,  q0,  q12      // * 5 -> a
.if \bpc == 8
        vld1.8          {d22}, [r1, :64]!
.else
        vld1.16         {q11}, [r1, :128]!
.endif
        vadd.i32        q8,  q8,  q6       // -1, +1
        vadd.i32        q9,  q9,  q7
.if \bpc == 8
        vmovl.u8        q11, d22
.endif
        vmul.i32        q4,  q4,  q15      // * 6
        vmla.i32        q4,  q8,  q14      // * 5 -> b
        vmul.i32        q5,  q5,  q15      // * 6
        vmla.i32        q5,  q9,  q14      // * 5 -> b

        vmlal.u16       q4,  d4,  d22      // b + a * src
        vmlal.u16       q5,  d5,  d23
        vmov            q0,  q1
        vrshrn.i32      d8,  q4,  #8
        vrshrn.i32      d9,  q5,  #8
        vmov            q8,  q10
        vst1.16         {q4}, [r0, :128]!

        ble             5f
        vld1.16         {q1},      [r4, :128]!
        vld1.32         {q9, q10}, [r3, :128]!
        b               4b

5:
        subs            r6,  r6,  #1
        ble             0f
        mov             r5,  lr
        sub             r3,  r3,  r11, lsl #2 // Rewind r3/r4 to where they started
        sub             r4,  r4,  r11, lsl #1
        add             r0,  r0,  r10, lsl #1
        add             r1,  r1,  r2
        sub             r3,  r3,  #16
        sub             r4,  r4,  #16
        b               1b
0:
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

// void dav1d_sgr_weighted1_Xbpc_neon(pixel *dst, const ptrdiff_t dst_stride,
//                                    const pixel *src, const ptrdiff_t src_stride,
//                                    const int16_t *t1, const int w, const int h,
//                                    const int wt, const int bitdepth_max);
function sgr_weighted1_\bpc\()bpc_neon, export=1
        push            {r4-r9,lr}
        ldrd            r4,  r5,  [sp, #28]
        ldrd            r6,  r7,  [sp, #36]
.if \bpc == 16
        ldr             r8,  [sp, #44]
.endif
        vdup.16         d31, r7
        cmp             r6,  #2
.if \bpc == 16
        vdup.16         q14, r8
.endif
        add             r9,  r0,  r1
        add             r12, r2,  r3
        add             lr,  r4,  #2*FILTER_OUT_STRIDE
        mov             r7,  #(4*FILTER_OUT_STRIDE)
        lsl             r1,  r1,  #1
        lsl             r3,  r3,  #1
        add             r8,  r5,  #7
        bic             r8,  r8,  #7 // Aligned width
.if \bpc == 8
        sub             r1,  r1,  r8
        sub             r3,  r3,  r8
.else
        sub             r1,  r1,  r8, lsl #1
        sub             r3,  r3,  r8, lsl #1
.endif
        sub             r7,  r7,  r8, lsl #1
        mov             r8,  r5
        blt             2f
1:
.if \bpc == 8
        vld1.8          {d0},  [r2,  :64]!
        vld1.8          {d16}, [r12, :64]!
.else
        vld1.16         {q0},  [r2,  :128]!
        vld1.16         {q8},  [r12, :128]!
.endif
        vld1.16         {q1},  [r4,  :128]!
        vld1.16         {q9},  [lr,  :128]!
        subs            r5,  r5,  #8
.if \bpc == 8
        vshll.u8        q0,  d0,  #4     // u
        vshll.u8        q8,  d16, #4     // u
.else
        vshl.i16        q0,  q0,  #4     // u
        vshl.i16        q8,  q8,  #4     // u
.endif
        vsub.i16        q1,  q1,  q0     // t1 - u
        vsub.i16        q9,  q9,  q8     // t1 - u
        vshll.u16       q2,  d0,  #7     // u << 7
        vshll.u16       q3,  d1,  #7     // u << 7
        vshll.u16       q10, d16, #7     // u << 7
        vshll.u16       q11, d17, #7     // u << 7
        vmlal.s16       q2,  d2,  d31    // v
        vmlal.s16       q3,  d3,  d31    // v
        vmlal.s16       q10, d18, d31    // v
        vmlal.s16       q11, d19, d31    // v
.if \bpc == 8
        vrshrn.i32      d4,  q2,  #11
        vrshrn.i32      d5,  q3,  #11
        vrshrn.i32      d20, q10, #11
        vrshrn.i32      d21, q11, #11
        vqmovun.s16     d4,  q2
        vqmovun.s16     d20, q10
        vst1.8          {d4},  [r0, :64]!
        vst1.8          {d20}, [r9, :64]!
.else
        vqrshrun.s32    d4,  q2,  #11
        vqrshrun.s32    d5,  q3,  #11
        vqrshrun.s32    d20, q10, #11
        vqrshrun.s32    d21, q11, #11
        vmin.u16        q2,  q2,  q14
        vmin.u16        q10, q10, q14
        vst1.16         {q2},  [r0, :128]!
        vst1.16         {q10}, [r9, :128]!
.endif
        bgt             1b

        sub             r6,  r6,  #2
        cmp             r6,  #1
        blt             0f
        mov             r5,  r8
        add             r0,  r0,  r1
        add             r9,  r9,  r1
        add             r2,  r2,  r3
        add             r12, r12, r3
        add             r4,  r4,  r7
        add             lr,  lr,  r7
        beq             2f
        b               1b

2:
.if \bpc == 8
        vld1.8          {d0}, [r2, :64]!
.else
        vld1.16         {q0}, [r2, :128]!
.endif
        vld1.16         {q1}, [r4, :128]!
        subs            r5,  r5,  #8
.if \bpc == 8
        vshll.u8        q0,  d0,  #4     // u
.else
        vshl.i16        q0,  q0,  #4     // u
.endif
        vsub.i16        q1,  q1,  q0     // t1 - u
        vshll.u16       q2,  d0,  #7     // u << 7
        vshll.u16       q3,  d1,  #7     // u << 7
        vmlal.s16       q2,  d2,  d31    // v
        vmlal.s16       q3,  d3,  d31    // v
.if \bpc == 8
        vrshrn.i32      d4,  q2,  #11
        vrshrn.i32      d5,  q3,  #11
        vqmovun.s16     d2,  q2
        vst1.8          {d2}, [r0, :64]!
.else
        vqrshrun.s32    d4,  q2,  #11
        vqrshrun.s32    d5,  q3,  #11
        vmin.u16        q2,  q2,  q14
        vst1.16         {q2}, [r0, :128]!
.endif
        bgt             2b
0:
        pop             {r4-r9,pc}
endfunc

// void dav1d_sgr_weighted2_Xbpc_neon(pixel *dst, const ptrdiff_t stride,
//                                    const pixel *src, const ptrdiff_t src_stride,
//                                    const int16_t *t1, const int16_t *t2,
//                                    const int w, const int h,
//                                    const int16_t wt[2], const int bitdepth_max);
function sgr_weighted2_\bpc\()bpc_neon, export=1
        push            {r4-r11,lr}
        ldrd            r4,  r5,  [sp, #36]
        ldrd            r6,  r7,  [sp, #44]
.if \bpc == 8
        ldr             r8,  [sp, #52]
.else
        ldrd            r8,  r9,  [sp, #52]
.endif
        cmp             r7,  #2
        add             r10, r0,  r1
        add             r11, r2,  r3
        add             r12, r4,  #2*FILTER_OUT_STRIDE
        add             lr,  r5,  #2*FILTER_OUT_STRIDE
        vld2.16         {d30[], d31[]}, [r8] // wt[0], wt[1]
.if \bpc == 16
        vdup.16         q14, r9
.endif
        mov             r8,  #4*FILTER_OUT_STRIDE
        lsl             r1,  r1,  #1
        lsl             r3,  r3,  #1
        add             r9,  r6,  #7
        bic             r9,  r9,  #7 // Aligned width
.if \bpc == 8
        sub             r1,  r1,  r9
        sub             r3,  r3,  r9
.else
        sub             r1,  r1,  r9, lsl #1
        sub             r3,  r3,  r9, lsl #1
.endif
        sub             r8,  r8,  r9, lsl #1
        mov             r9,  r6
        blt             2f
1:
.if \bpc == 8
        vld1.8          {d0},  [r2,  :64]!
        vld1.8          {d16}, [r11, :64]!
.else
        vld1.16         {q0},  [r2,  :128]!
        vld1.16         {q8},  [r11, :128]!
.endif
        vld1.16         {q1},  [r4,  :128]!
        vld1.16         {q9},  [r12, :128]!
        vld1.16         {q2},  [r5,  :128]!
        vld1.16         {q10}, [lr,  :128]!
        subs            r6,  r6,  #8
.if \bpc == 8
        vshll.u8        q0,  d0,  #4     // u
        vshll.u8        q8,  d16, #4     // u
.else
        vshl.i16        q0,  q0,  #4     // u
        vshl.i16        q8,  q8,  #4     // u
.endif
        vsub.i16        q1,  q1,  q0     // t1 - u
        vsub.i16        q2,  q2,  q0     // t2 - u
        vsub.i16        q9,  q9,  q8     // t1 - u
        vsub.i16        q10, q10, q8     // t2 - u
        vshll.u16       q3,  d0,  #7     // u << 7
        vshll.u16       q0,  d1,  #7     // u << 7
        vshll.u16       q11, d16, #7     // u << 7
        vshll.u16       q8,  d17, #7     // u << 7
        vmlal.s16       q3,  d2,  d30    // wt[0] * (t1 - u)
        vmlal.s16       q3,  d4,  d31    // wt[1] * (t2 - u)
        vmlal.s16       q0,  d3,  d30    // wt[0] * (t1 - u)
        vmlal.s16       q0,  d5,  d31    // wt[1] * (t2 - u)
        vmlal.s16       q11, d18, d30    // wt[0] * (t1 - u)
        vmlal.s16       q11, d20, d31    // wt[1] * (t2 - u)
        vmlal.s16       q8,  d19, d30    // wt[0] * (t1 - u)
        vmlal.s16       q8,  d21, d31    // wt[1] * (t2 - u)
.if \bpc == 8
        vrshrn.i32      d6,  q3,  #11
        vrshrn.i32      d7,  q0,  #11
        vrshrn.i32      d22, q11, #11
        vrshrn.i32      d23, q8,  #11
        vqmovun.s16     d6,  q3
        vqmovun.s16     d22, q11
        vst1.8          {d6},  [r0,  :64]!
        vst1.8          {d22}, [r10, :64]!
.else
        vqrshrun.s32    d6,  q3,  #11
        vqrshrun.s32    d7,  q0,  #11
        vqrshrun.s32    d22, q11, #11
        vqrshrun.s32    d23, q8,  #11
        vmin.u16        q3,  q3,  q14
        vmin.u16        q11, q11, q14
        vst1.16         {q3},  [r0,  :128]!
        vst1.16         {q11}, [r10, :128]!
.endif
        bgt             1b

        subs            r7,  r7,  #2
        cmp             r7,  #1
        blt             0f
        mov             r6,  r9
        add             r0,  r0,  r1
        add             r10, r10, r1
        add             r2,  r2,  r3
        add             r11, r11, r3
        add             r4,  r4,  r8
        add             r12, r12, r8
        add             r5,  r5,  r8
        add             lr,  lr,  r8
        beq             2f
        b               1b

2:
.if \bpc == 8
        vld1.8          {d0}, [r2, :64]!
.else
        vld1.16         {q0}, [r2, :128]!
.endif
        vld1.16         {q1}, [r4, :128]!
        vld1.16         {q2}, [r5, :128]!
        subs            r6,  r6,  #8
.if \bpc == 8
        vshll.u8        q0,  d0,  #4     // u
.else
        vshl.i16        q0,  q0,  #4     // u
.endif
        vsub.i16        q1,  q1,  q0     // t1 - u
        vsub.i16        q2,  q2,  q0     // t2 - u
        vshll.u16       q3,  d0,  #7     // u << 7
        vshll.u16       q0,  d1,  #7     // u << 7
        vmlal.s16       q3,  d2,  d30    // wt[0] * (t1 - u)
        vmlal.s16       q3,  d4,  d31    // wt[1] * (t2 - u)
        vmlal.s16       q0,  d3,  d30    // wt[0] * (t1 - u)
        vmlal.s16       q0,  d5,  d31    // wt[1] * (t2 - u)
.if \bpc == 8
        vrshrn.i32      d6,  q3,  #11
        vrshrn.i32      d7,  q0,  #11
        vqmovun.s16     d6,  q3
        vst1.8          {d6}, [r0, :64]!
.else
        vqrshrun.s32    d6,  q3,  #11
        vqrshrun.s32    d7,  q0,  #11
        vmin.u16        q3,  q3,  q14
        vst1.16         {q3}, [r0, :128]!
.endif
        bgt             1b
0:
        pop             {r4-r11,pc}
endfunc
.endm
