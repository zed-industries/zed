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
#include "util.S"

#define SUM_STRIDE (384+16)

// void dav1d_sgr_box3_v_neon(int32_t *sumsq, int16_t *sum,
//                            const int w, const int h,
//                            const enum LrEdgeFlags edges);
function sgr_box3_v_neon, export=1
        push            {r4-r9,lr}
        ldr             r4,  [sp, #28]
        add             r12, r3,  #2 // Number of output rows to move back
        mov             lr,  r3      // Number of input rows to move back
        add             r2,  r2,  #2 // Actual summed width
        mov             r7,       #(4*SUM_STRIDE) // sumsq stride
        mov             r8,       #(2*SUM_STRIDE) // sum stride
        sub             r0,  r0,  #(4*SUM_STRIDE) // sumsq -= stride
        sub             r1,  r1,  #(2*SUM_STRIDE) // sum   -= stride

        tst             r4,  #4 // LR_HAVE_TOP
        beq             0f
        // If have top, read from row -2.
        sub             r5,  r0,  #(4*SUM_STRIDE)
        sub             r6,  r1,  #(2*SUM_STRIDE)
        add             lr,  lr,  #2
        b               1f
0:
        // !LR_HAVE_TOP
        // If we don't have top, read from row 0 even if
        // we start writing to row -1.
        add             r5,  r0,  #(4*SUM_STRIDE)
        add             r6,  r1,  #(2*SUM_STRIDE)
1:

        tst             r4,  #8 // LR_HAVE_BOTTOM
        beq             1f
        // LR_HAVE_BOTTOM
        add             r3,  r3,  #2  // Sum all h+2 lines with the main loop
        add             lr,  lr,  #2
1:
        mov             r9,  r3       // Backup of h for next loops

1:
        // Start of horizontal loop; start one vertical filter slice.
        // Start loading rows into q8-q13 and q0-q2 taking top
        // padding into consideration.
        tst             r4,  #4 // LR_HAVE_TOP
        vld1.32         {q8,  q9},  [r5, :128], r7
        vld1.16         {q0},       [r6, :128], r8
        beq             2f
        // LR_HAVE_TOP
        vld1.32         {q10, q11}, [r5, :128], r7
        vld1.16         {q1},       [r6, :128], r8
        vld1.32         {q12, q13}, [r5, :128], r7
        vld1.16         {q2},       [r6, :128], r8
        b               3f
2:      // !LR_HAVE_TOP
        vmov            q10, q8
        vmov            q11, q9
        vmov            q1,  q0
        vmov            q12, q8
        vmov            q13, q9
        vmov            q2,  q0

3:
        subs            r3,  r3,  #1
.macro add3
        vadd.i32        q8,  q8,  q10
        vadd.i32        q9,  q9,  q11
        vadd.i16        q0,  q0,  q1
        vadd.i32        q8,  q8,  q12
        vadd.i32        q9,  q9,  q13
        vadd.i16        q0,  q0,  q2
        vst1.32         {q8, q9}, [r0, :128], r7
        vst1.16         {q0},     [r1, :128], r8
.endm
        add3
        vmov            q8,  q10
        vmov            q9,  q11
        vmov            q0,  q1
        vmov            q10, q12
        vmov            q11, q13
        vmov            q1,  q2
        ble             4f
        vld1.32         {q12, q13}, [r5, :128], r7
        vld1.16         {q2},       [r6, :128], r8
        b               3b

4:
        tst             r4,  #8 // LR_HAVE_BOTTOM
        bne             5f
        // !LR_HAVE_BOTTOM
        // Produce two more rows, extending the already loaded rows.
        add3
        vmov            q8,  q10
        vmov            q9,  q11
        vmov            q0,  q1
        add3

5:      // End of one vertical slice.
        subs            r2,  r2,  #8
        ble             0f
        // Move pointers back up to the top and loop horizontally.
        // Input pointers
        mls             r5,  r7,  lr,  r5
        mls             r6,  r8,  lr,  r6
        // Output pointers
        mls             r0,  r7,  r12, r0
        mls             r1,  r8,  r12, r1
        add             r0,  r0,  #32
        add             r1,  r1,  #16
        add             r5,  r5,  #32
        add             r6,  r6,  #16
        mov             r3,  r9
        b               1b

0:
        pop             {r4-r9,pc}
.purgem add3
endfunc

// void dav1d_sgr_box5_v_neon(int32_t *sumsq, int16_t *sum,
//                            const int w, const int h,
//                            const enum LrEdgeFlags edges);
function sgr_box5_v_neon, export=1
        push            {r4-r9,lr}
        vpush           {q5-q7}
        ldr             r4,  [sp, #76]
        add             r12, r3,  #2 // Number of output rows to move back
        mov             lr,  r3      // Number of input rows to move back
        add             r2,  r2,  #8 // Actual summed width
        mov             r7,       #(4*SUM_STRIDE) // sumsq stride
        mov             r8,       #(2*SUM_STRIDE) // sum stride
        sub             r0,  r0,  #(4*SUM_STRIDE) // sumsq -= stride
        sub             r1,  r1,  #(2*SUM_STRIDE) // sum   -= stride

        tst             r4,  #4 // LR_HAVE_TOP
        beq             0f
        // If have top, read from row -2.
        sub             r5,  r0,  #(4*SUM_STRIDE)
        sub             r6,  r1,  #(2*SUM_STRIDE)
        add             lr,  lr,  #2
        b               1f
0:
        // !LR_HAVE_TOP
        // If we don't have top, read from row 0 even if
        // we start writing to row -1.
        add             r5,  r0,  #(4*SUM_STRIDE)
        add             r6,  r1,  #(2*SUM_STRIDE)
1:

        tst             r4,  #8 // LR_HAVE_BOTTOM
        beq             0f
        // LR_HAVE_BOTTOM
        add             r3,  r3,  #2  // Handle h+2 lines with the main loop
        add             lr,  lr,  #2
        b               1f
0:
        // !LR_HAVE_BOTTOM
        sub             r3,  r3,  #1  // Handle h-1 lines with the main loop
1:
        mov             r9,  r3       // Backup of h for next loops

1:
        // Start of horizontal loop; start one vertical filter slice.
        // Start loading rows into q6-q15 and q0-q3,q5 taking top
        // padding into consideration.
        tst             r4,  #4 // LR_HAVE_TOP
        vld1.32         {q6,  q7},  [r5, :128], r7
        vld1.16         {q0},       [r6, :128], r8
        beq             2f
        // LR_HAVE_TOP
        vld1.32         {q10, q11}, [r5, :128], r7
        vld1.16         {q2},       [r6, :128], r8
        vmov            q8,  q6
        vmov            q9,  q7
        vmov            q1,  q0
        vld1.32         {q12, q13}, [r5, :128], r7
        vld1.16         {q3},       [r6, :128], r8
        b               3f
2:      // !LR_HAVE_TOP
        vmov            q8,  q6
        vmov            q9,  q7
        vmov            q1,  q0
        vmov            q10, q6
        vmov            q11, q7
        vmov            q2,  q0
        vmov            q12, q6
        vmov            q13, q7
        vmov            q3,  q0

3:
        cmp             r3,  #0
        beq             4f
        vld1.32         {q14, q15}, [r5, :128], r7
        vld1.16         {q5},       [r6, :128], r8

3:
        // Start of vertical loop
        subs            r3,  r3,  #2
.macro add5
        vadd.i32        q6,  q6,  q8
        vadd.i32        q7,  q7,  q9
        vadd.i16        q0,  q0,  q1
        vadd.i32        q6,  q6,  q10
        vadd.i32        q7,  q7,  q11
        vadd.i16        q0,  q0,  q2
        vadd.i32        q6,  q6,  q12
        vadd.i32        q7,  q7,  q13
        vadd.i16        q0,  q0,  q3
        vadd.i32        q6,  q6,  q14
        vadd.i32        q7,  q7,  q15
        vadd.i16        q0,  q0,  q5
        vst1.32         {q6, q7}, [r0, :128], r7
        vst1.16         {q0},     [r1, :128], r8
.endm
        add5
.macro shift2
        vmov            q6,  q10
        vmov            q7,  q11
        vmov            q0,  q2
        vmov            q8,  q12
        vmov            q9,  q13
        vmov            q1,  q3
        vmov            q10, q14
        vmov            q11, q15
        vmov            q2,  q5
.endm
        shift2
        add             r0,  r0,  r7
        add             r1,  r1,  r8
        ble             5f
        vld1.32         {q12, q13}, [r5, :128], r7
        vld1.16         {q3},       [r6, :128], r8
        vld1.32         {q14, q15}, [r5, :128], r7
        vld1.16         {q5},       [r6, :128], r8
        b               3b

4:
        // h == 1, !LR_HAVE_BOTTOM.
        // Pad the last row with the only content row, and add.
        vmov            q14, q12
        vmov            q15, q13
        vmov            q5,  q3
        add5
        shift2
        add             r0,  r0,  r7
        add             r1,  r1,  r8
        add5
        b               6f

5:
        tst             r4,  #8 // LR_HAVE_BOTTOM
        bne             6f
        // !LR_HAVE_BOTTOM
        cmp             r3,  #0
        bne             5f
        // The intended three edge rows left; output the one at h-2 and
        // the past edge one at h.
        vld1.32         {q12, q13}, [r5, :128], r7
        vld1.16         {q3},       [r6, :128], r8
        // Pad the past-edge row from the last content row.
        vmov            q14, q12
        vmov            q15, q13
        vmov            q5,  q3
        add5
        shift2
        add             r0,  r0,  r7
        add             r1,  r1,  r8
        // The last two rows are already padded properly here.
        add5
        b               6f

5:
        // r3 == -1, two rows left, output one.
        // Pad the last two rows from the mid one.
        vmov            q12, q10
        vmov            q13, q11
        vmov            q3,  q2
        vmov            q14, q10
        vmov            q15, q11
        vmov            q5,  q2
        add5
        add             r0,  r0,  r7
        add             r1,  r1,  r8
        b               6f

6:      // End of one vertical slice.
        subs            r2,  r2,  #8
        ble             0f
        // Move pointers back up to the top and loop horizontally.
        // Input pointers
        mls             r5,  r7,  lr,  r5
        mls             r6,  r8,  lr,  r6
        // Output pointers
        mls             r0,  r7,  r12, r0
        mls             r1,  r8,  r12, r1
        add             r0,  r0,  #32
        add             r1,  r1,  #16
        add             r5,  r5,  #32
        add             r6,  r6,  #16
        mov             r3,  r9
        b               1b

0:
        vpop            {q5-q7}
        pop             {r4-r9,pc}
.purgem add5
endfunc

// void dav1d_sgr_calc_ab1_neon(int32_t *a, int16_t *b,
//                              const int w, const int h, const int strength,
//                              const int bitdepth_max);
// void dav1d_sgr_calc_ab2_neon(int32_t *a, int16_t *b,
//                              const int w, const int h, const int strength,
//                              const int bitdepth_max);
function sgr_calc_ab1_neon, export=1
        push            {r4-r7,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #84]
        add             r3,  r3,  #2   // h += 2
        clz             r6,  r5
        vmov.i32        q15, #9        // n
        movw            r5,  #455
        mov             lr,  #SUM_STRIDE
        b               sgr_calc_ab_neon
endfunc

function sgr_calc_ab2_neon, export=1
        push            {r4-r7,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #84]
        add             r3,  r3,  #3   // h += 3
        clz             r6,  r5
        asr             r3,  r3,  #1   // h /= 2
        vmov.i32        q15, #25       // n
        mov             r5,  #164
        mov             lr,  #(2*SUM_STRIDE)
endfunc

function sgr_calc_ab_neon
        movrel          r12, X(sgr_x_by_x)
        sub             r6,  r6,  #24  // -bitdepth_min_8
        vld1.8          {q8, q9}, [r12, :128]!
        add             r7,  r6,  r6   // -2*bitdepth_min_8
        vmov.i8         q11, #5
        vmov.i8         d10, #55       // idx of last 5
        vld1.8          {q10},    [r12, :128]
        vmov.i8         d11, #72       // idx of last 4
        vmov.i8         d12, #101      // idx of last 3
        vmov.i8         d13, #169      // idx of last 2
        vmov.i8         d14, #254      // idx of last 1
        vmov.i8         d15, #32       // elements consumed in first vtbl
        add             r2,  r2,  #2   // w += 2
        add             r12, r2,  #7
        bic             r12, r12, #7   // aligned w
        sub             r12, lr,  r12  // increment between rows
        vdup.32         q12, r4
        sub             r0,  r0,  #(4*(SUM_STRIDE))
        sub             r1,  r1,  #(2*(SUM_STRIDE))
        mov             r4,  r2        // backup of w
        vsub.i8         q8,  q8,  q11
        vsub.i8         q9,  q9,  q11
        vsub.i8         q10, q10, q11
1:
        vld1.32         {q0, q1}, [r0, :128] // a
        vld1.16         {q2},     [r1, :128] // b
        vdup.32         q13, r7        // -2*bitdepth_min_8
        vdup.16         q14, r6        // -bitdepth_min_8
        subs            r2,  r2,  #8
        vrshl.s32       q0,  q0,  q13
        vrshl.s32       q1,  q1,  q13
        vrshl.s16       q4,  q2,  q14
        vmul.i32        q0,  q0,  q15  // a * n
        vmul.i32        q1,  q1,  q15  // a * n
        vmull.u16       q3,  d8,  d8   // b * b
        vmull.u16       q4,  d9,  d9   // b * b
        vqsub.u32       q0,  q0,  q3   // imax(a * n - b * b, 0)
        vqsub.u32       q1,  q1,  q4   // imax(a * n - b * b, 0)
        vmul.i32        q0,  q0,  q12  // p * s
        vmul.i32        q1,  q1,  q12  // p * s
        vqshrn.u32      d0,  q0,  #16
        vqshrn.u32      d1,  q1,  #16
        vqrshrn.u16     d0,  q0,  #4   // imin(z, 255)

        vcgt.u8         d2,  d0,  d10  // = -1 if sgr_x_by_x[d0] < 5
        vcgt.u8         d3,  d0,  d11  // = -1 if sgr_x_by_x[d0] < 4
        vtbl.8          d1,  {q8, q9}, d0
        vcgt.u8         d6,  d0,  d12  // = -1 if sgr_x_by_x[d0] < 3
        vsub.i8         d9,  d0,  d15  // indices for vtbx
        vcgt.u8         d7,  d0,  d13  // = -1 if sgr_x_by_x[d0] < 2
        vadd.i8         d2,  d2,  d3
        vtbx.8          d1,  {q10}, d9
        vcgt.u8         d8,  d0,  d14  // = -1 if sgr_x_by_x[d0] < 1
        vadd.i8         d6,  d6,  d7
        vadd.i8         d8,  d8,  d22
        vadd.i8         d2,  d2,  d6
        vadd.i8         d1,  d1,  d8
        vadd.i8         d1,  d1,  d2
        vmovl.u8        q0,  d1        // x

        vmov.i16        q13, #256
        vdup.32         q14, r5        // one_by_x

        vmull.u16       q1,  d0,  d4   // x * BB[i]
        vmull.u16       q2,  d1,  d5   // x * BB[i]
        vmul.i32        q1,  q1,  q14  // x * BB[i] * sgr_one_by_x
        vmul.i32        q2,  q2,  q14  // x * BB[i] * sgr_one_by_x
        vrshr.s32       q1,  q1,  #12  // AA[i]
        vrshr.s32       q2,  q2,  #12  // AA[i]
        vsub.i16        q0,  q13, q0   // 256 - x

        vst1.32         {q1, q2}, [r0, :128]!
        vst1.16         {q0},     [r1, :128]!
        bgt             1b

        subs            r3,  r3,  #1
        ble             0f
        add             r0,  r0,  r12, lsl #2
        add             r1,  r1,  r12, lsl #1
        mov             r2,  r4
        b               1b
0:
        vpop            {q4-q7}
        pop             {r4-r7,pc}
endfunc
