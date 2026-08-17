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

const right_ext_mask_buf
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
right_ext_mask:
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
endconst

// void dav1d_wiener_filter_h_8bpc_neon(int16_t *dst, const pixel (*left)[4],
//                                      const pixel *src, ptrdiff_t stride,
//                                      const int16_t fh[8], intptr_t w,
//                                      int h, enum LrEdgeFlags edges);
function wiener_filter_h_8bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldrd            r6,  r7,  [sp, #108]
        mov             r8,  r5
        vld1.16         {q0},  [r4, :128]
        movw            r9,  #(1 << 14) - (1 << 2)
        vdup.16         q14, r9
        vmov.s16        q15, #2048
        // Calculate mid_stride
        add             r10, r5,  #7
        bic             r10, r10, #7
        lsl             r10, r10, #1

        // Set up pointers for reading/writing alternate rows
        add             r12, r0,  r10
        lsl             r10, r10, #1
        add             lr,  r2,  r3
        lsl             r3,  r3,  #1

        // Subtract the aligned width from mid_stride
        add             r11, r5,  #7
        bic             r11, r11, #7
        sub             r10, r10, r11, lsl #1

        // Subtract the number of pixels read from the source stride
        add             r11, r11, #8
        sub             r3,  r3,  r11

        // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
        tst             r7,  #1 // LR_HAVE_LEFT
        beq             2f
        // LR_HAVE_LEFT
        cmp             r1,  #0
        bne             0f
        // left == NULL
        sub             r2,  r2,  #3
        sub             lr,  lr,  #3
        b               1f
0:      // LR_HAVE_LEFT, left != NULL
2:      // !LR_HAVE_LEFT, increase the stride.
        // For this case we don't read the left 3 pixels from the src pointer,
        // but shift it as if we had done that.
        add             r3,  r3,  #3


1:      // Loop vertically
        vld1.8          {q2},  [r2]!
        vld1.8          {q9},  [lr]!

        tst             r7,  #1 // LR_HAVE_LEFT
        beq             0f
        cmp             r1,  #0
        beq             2f
        // LR_HAVE_LEFT, left != NULL
        vld1.32         {d3[1]},  [r1]!
        // Move r2/lr back to account for the last 3 bytes we loaded earlier,
        // which we'll shift out.
        sub             r2,  r2,  #3
        sub             lr,  lr,  #3
        vld1.32         {d17[1]},  [r1]!
        vext.8          q2,  q1,  q2,  #13
        vext.8          q9,  q8,  q9,  #13
        b               2f
0:
        // !LR_HAVE_LEFT, fill q1 with the leftmost byte
        // and shift q2 to have 3x the first byte at the front.
        vdup.8          q1,  d4[0]
        vdup.8          q8,  d18[0]
        // Move r2 back to account for the last 3 bytes we loaded before,
        // which we shifted out.
        sub             r2,  r2,  #3
        sub             lr,  lr,  #3
        vext.8          q2,  q1,  q2,  #13
        vext.8          q9,  q8,  q9,  #13

2:
        vmovl.u8        q1,  d4
        vmovl.u8        q2,  d5
        vmovl.u8        q8,  d18
        vmovl.u8        q9,  d19

        tst             r7,  #2 // LR_HAVE_RIGHT
        bne             4f
        // If we'll need to pad the right edge, load that byte to pad with
        // here since we can find it pretty easily from here.
        sub             r9,  r5,  #14
        ldrb            r11, [r2, r9]
        ldrb            r9,  [lr, r9]
        // Fill q12/q13 with the right padding pixel
        vdup.16         q12, r11
        vdup.16         q13, r9
3:      // !LR_HAVE_RIGHT

        // Check whether we need to pad the right edge
        cmp             r5,  #11
        bge             4f   // If w >= 11, all used input pixels are valid

        // 1 <= w < 11, w+3 pixels valid in q1-q2. For w=9 or w=10,
        // this ends up called again; it's not strictly needed in those
        // cases (we pad enough here), but keeping the code as simple as possible.

        // Insert padding in q1/2.h[w+3] onwards; fuse the +3 (*2) into the
        // buffer pointer.
        movrel_local    r4,  right_ext_mask, -6
        sub             r4,  r4,  r5,  lsl #1
        vld1.8          {q10, q11}, [r4]

        vbit            q1,  q12, q10
        vbit            q2,  q12, q11
        vbit            q8,  q13, q10
        vbit            q9,  q13, q11

4:      // Loop horizontally
        vext.8          q11, q1,  q2,  #4
        vext.8          q5,  q1,  q2,  #8
        vext.8          q10, q1,  q2,  #2
        vext.8          q6,  q1,  q2,  #10
        vext.8          q7,  q1,  q2,  #12
        vext.8          q4,  q1,  q2,  #6
        vadd.i16        q5,  q5,  q11
        vadd.i16        q6,  q6,  q10
        vadd.i16        q7,  q7,  q1
        vmul.s16        q3,  q4,  d0[3]
        vmla.s16        q3,  q5,  d1[0]
        vmla.s16        q3,  q6,  d1[1]
        vmla.s16        q3,  q7,  d1[2]

        vext.8          q4,  q8,  q9,  #4
        vext.8          q6,  q8,  q9,  #8
        vext.8          q11, q8,  q9,  #2
        vext.8          q7,  q8,  q9,  #10
        vadd.i16        q6,  q6,  q4
        vext.8          q4,  q8,  q9,  #12
        vext.8          q5,  q8,  q9,  #6
        vadd.i16        q7,  q7,  q11
        vadd.i16        q4,  q4,  q8
        vmul.s16        q10, q5,  d0[3]
        vmla.s16        q10, q6,  d1[0]
        vmla.s16        q10, q7,  d1[1]
        vmla.s16        q10, q4,  d1[2]

        vext.8          q1,  q1,  q2,  #6
        vext.8          q8,  q8,  q9,  #6
        vshl.s16        q1,  q1,  #7
        vshl.s16        q8,  q8,  #7
        vsub.s16        q1,  q1,  q14
        vsub.s16        q8,  q8,  q14
        vqadd.s16       q3,  q3,  q1
        vqadd.s16       q10, q10, q8
        vshr.s16        q3,  q3,  #3
        vshr.s16        q10, q10, #3
        vadd.s16        q3,  q3,  q15
        vadd.s16        q10, q10, q15
        subs            r5,  r5,  #8
        vst1.16         {q3},  [r0,  :128]!
        vst1.16         {q10}, [r12, :128]!

        ble             9f
        tst             r7,  #2 // LR_HAVE_RIGHT
        vmov            q1,  q2
        vmov            q8,  q9
        vld1.8          {d4},  [r2]!
        vld1.8          {d18}, [lr]!
        vmovl.u8        q2,  d4
        vmovl.u8        q9,  d18
        bne             4b // If we don't need to pad, just keep filtering.
        b               3b // If we need to pad, check how many pixels we have left.

9:
        subs            r6,  r6,  #2
        ble             0f
        // Jump to the next row and loop horizontally
        add             r0,  r0,  r10
        add             r12, r12, r10
        add             r2,  r2,  r3
        add             lr,  lr,  r3
        mov             r5,  r8
        b               1b
0:
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

// void dav1d_wiener_filter_v_8bpc_neon(pixel *dst, ptrdiff_t stride,
//                                      const int16_t *mid, int w, int h,
//                                      const int16_t fv[8], enum LrEdgeFlags edges,
//                                      ptrdiff_t mid_stride);
function wiener_filter_v_8bpc_neon, export=1
        push            {r4-r7,lr}
        vpush           {q4-q6}
        ldrd            r4,  r5,  [sp, #68]
        ldrd            r6,  r7,  [sp, #76]
        mov             lr,  r4
        vld1.16         {q0},  [r5, :128]

        // Calculate the number of rows to move back when looping vertically
        mov             r12, r4
        tst             r6,  #4 // LR_HAVE_TOP
        beq             0f
        sub             r2,  r2,  r7,  lsl #1
        add             r12, r12, #2
0:
        tst             r6,  #8 // LR_HAVE_BOTTOM
        beq             1f
        add             r12, r12, #2

1:      // Start of horizontal loop; start one vertical filter slice.
        // Load rows into q8-q11 and pad properly.
        tst             r6,  #4 // LR_HAVE_TOP
        vld1.16         {q8},  [r2, :128], r7
        beq             2f
        // LR_HAVE_TOP
        vld1.16         {q10}, [r2, :128], r7
        vmov            q9,  q8
        vld1.16         {q11}, [r2, :128], r7
        b               3f
2:      // !LR_HAVE_TOP
        vmov            q9,  q8
        vmov            q10, q8
        vmov            q11, q8

3:
        cmp             r4,  #4
        blt             5f
        // Start filtering normally; fill in q12-q14 with unique rows.
        vld1.16         {q12}, [r2, :128], r7
        vld1.16         {q13}, [r2, :128], r7
        vld1.16         {q14}, [r2, :128], r7

4:
.macro filter compare
        subs            r4,  r4,  #1
        // Interleaving the mul/mla chains actually hurts performance
        // significantly on Cortex A53, thus keeping mul/mla tightly
        // chained like this.
        vadd.i16        q4,  q10, q12
        vadd.i16        q5,  q9,  q13
        vadd.i16        q6,  q8,  q14
        vmull.s16       q2,  d22, d0[3]
        vmlal.s16       q2,  d8,  d1[0]
        vmlal.s16       q2,  d10, d1[1]
        vmlal.s16       q2,  d12, d1[2]
        vmull.s16       q3,  d23, d0[3]
        vmlal.s16       q3,  d9,  d1[0]
        vmlal.s16       q3,  d11, d1[1]
        vmlal.s16       q3,  d13, d1[2]
        vqrshrun.s32    d4,  q2,  #11
        vqrshrun.s32    d5,  q3,  #11
        vqmovun.s16     d4,  q2
        vst1.8          {d4}, [r0, :64], r1
.if \compare
        cmp             r4,  #4
.else
        ble             9f
.endif
        vmov            q8,  q9
        vmov            q9,  q10
        vmov            q10, q11
        vmov            q11, q12
        vmov            q12, q13
        vmov            q13, q14
.endm
        filter          1
        blt             7f
        vld1.16         {q14}, [r2, :128], r7
        b               4b

5:      // Less than 4 rows in total; not all of q12-q13 are filled yet.
        tst             r6,  #8 // LR_HAVE_BOTTOM
        beq             6f
        // LR_HAVE_BOTTOM
        cmp             r4,  #2
        // We load at least 2 rows in all cases.
        vld1.16         {q12}, [r2, :128], r7
        vld1.16         {q13}, [r2, :128], r7
        bgt             53f // 3 rows in total
        beq             52f // 2 rows in total
51:     // 1 row in total, q11 already loaded, load edge into q12-q14.
        vmov            q13, q12
        b               8f
52:     // 2 rows in total, q11 already loaded, load q12 with content data
        // and 2 rows of edge.
        vld1.16         {q14}, [r2, :128], r7
        vmov            q15, q14
        b               8f
53:
        // 3 rows in total, q11 already loaded, load q12 and q13 with content
        // and 2 rows of edge.
        vld1.16         {q14}, [r2, :128], r7
        vld1.16         {q15}, [r2, :128], r7
        vmov            q1,  q15
        b               8f

6:
        // !LR_HAVE_BOTTOM
        cmp             r4,  #2
        bgt             63f // 3 rows in total
        beq             62f // 2 rows in total
61:     // 1 row in total, q11 already loaded, pad that into q12-q14.
        vmov            q12, q11
        vmov            q13, q11
        vmov            q14, q11
        b               8f
62:     // 2 rows in total, q11 already loaded, load q12 and pad that into q12-q15.
        vld1.16         {q12}, [r2, :128], r7
        vmov            q13, q12
        vmov            q14, q12
        vmov            q15, q12
        b               8f
63:
        // 3 rows in total, q11 already loaded, load q12 and q13 and pad q13 into q14-q15,q1.
        vld1.16         {q12}, [r2, :128], r7
        vld1.16         {q13}, [r2, :128], r7
        vmov            q14, q13
        vmov            q15, q13
        vmov            q1,  q13
        b               8f

7:
        // All registers up to q13 are filled already, 3 valid rows left.
        // < 4 valid rows left; fill in padding and filter the last
        // few rows.
        tst             r6,  #8 // LR_HAVE_BOTTOM
        beq             71f
        // LR_HAVE_BOTTOM; load 2 rows of edge.
        vld1.16         {q14}, [r2, :128], r7
        vld1.16         {q15}, [r2, :128], r7
        vmov            q1,  q15
        b               8f
71:
        // !LR_HAVE_BOTTOM, pad 3 rows
        vmov            q14, q13
        vmov            q15, q13
        vmov            q1,  q13

8:      // At this point, all registers up to q14-15,q1 are loaded with
        // edge/padding (depending on how many rows are left).
        filter          0 // This branches to 9f when done
        vmov            q14, q15
        vmov            q15, q1
        b               8b

9:      // End of one vertical slice.
        subs            r3,  r3,  #8
        ble             0f
        // Move pointers back up to the top and loop horizontally.
        mls             r0,  r1,  lr,  r0
        mls             r2,  r7,  r12, r2
        add             r0,  r0,  #8
        add             r2,  r2,  #16
        mov             r4,  lr
        b               1b

0:
        vpop            {q4-q6}
        pop             {r4-r7,pc}
.purgem filter
endfunc

#define SUM_STRIDE (384+16)

#include "looprestoration_tmpl.S"

// void dav1d_sgr_box3_h_8bpc_neon(int32_t *sumsq, int16_t *sum,
//                                 const pixel (*left)[4],
//                                 const pixel *src, const ptrdiff_t stride,
//                                 const int w, const int h,
//                                 const enum LrEdgeFlags edges);
function sgr_box3_h_8bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldrd            r6,  r7,  [sp, #108]
        add             r5,  r5,  #2 // w += 2

        // Set up pointers for reading/writing alternate rows
        add             r10, r0,  #(4*SUM_STRIDE)   // sumsq
        add             r11, r1,  #(2*SUM_STRIDE)   // sum
        add             r12, r3,  r4                // src
        lsl             r4,  r4,  #1
        mov             r9,       #(2*2*SUM_STRIDE) // double sum stride

        // Subtract the aligned width from the output stride.
        add             lr,  r5,  #7
        bic             lr,  lr,  #7
        sub             r9,  r9,  lr, lsl #1

        // Store the width for the vertical loop
        mov             r8,  r5

        // Subtract the number of pixels read from the input from the stride
        add             lr,  lr,  #8
        sub             r4,  r4,  lr

        // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
        tst             r7,  #1 // LR_HAVE_LEFT
        beq             2f
        // LR_HAVE_LEFT
        cmp             r2,  #0
        bne             0f
        // left == NULL
        sub             r3,  r3,  #2
        sub             r12, r12, #2
        b               1f
0:      // LR_HAVE_LEFT, left != NULL
2:      // !LR_HAVE_LEFT, increase the stride.
        // For this case we don't read the left 2 pixels from the src pointer,
        // but shift it as if we had done that.
        add             r4,  r4,  #2


1:      // Loop vertically
        vld1.8          {q0}, [r3]!
        vld1.8          {q4}, [r12]!

        tst             r7,  #1 // LR_HAVE_LEFT
        beq             0f
        cmp             r2,  #0
        beq             2f
        // LR_HAVE_LEFT, left != NULL
        vld1.32         {d3[]}, [r2]!
        // Move r3/r12 back to account for the last 2 bytes we loaded earlier,
        // which we'll shift out.
        sub             r3,  r3,  #2
        sub             r12, r12, #2
        vld1.32         {d11[]}, [r2]!
        vext.8          q0,  q1,  q0,  #14
        vext.8          q4,  q5,  q4,  #14
        b               2f
0:
        // !LR_HAVE_LEFT, fill q1 with the leftmost byte
        // and shift q0 to have 2x the first byte at the front.
        vdup.8          q1,  d0[0]
        vdup.8          q5,  d8[0]
        // Move r3 back to account for the last 2 bytes we loaded before,
        // which we shifted out.
        sub             r3,  r3,  #2
        sub             r12, r12, #2
        vext.8          q0,  q1,  q0,  #14
        vext.8          q4,  q5,  q4,  #14

2:
        vmull.u8        q1,  d0,  d0
        vmull.u8        q2,  d1,  d1
        vmull.u8        q5,  d8,  d8
        vmull.u8        q6,  d9,  d9

        tst             r7,  #2 // LR_HAVE_RIGHT
        bne             4f
        // If we'll need to pad the right edge, load that byte to pad with
        // here since we can find it pretty easily from here.
        sub             lr,  r5,  #(2 + 16 - 2 + 1)
        ldrb            r11, [r3,  lr]
        ldrb            lr,  [r12, lr]
        // Fill q14/q15 with the right padding pixel
        vdup.8          q14, r11
        vdup.8          q15, lr
        // Restore r11 after using it for a temporary value
        add             r11, r1,  #(2*SUM_STRIDE)
3:      // !LR_HAVE_RIGHT

        // Check whether we need to pad the right edge
        cmp             r5,  #10
        bge             4f   // If w >= 10, all used input pixels are valid

        // 1 <= w < 10, w pixels valid in q0. For w=9, this ends up called
        // again; it's not strictly needed in those cases (we pad enough here),
        // but keeping the code as simple as possible.

        // Insert padding in q0/4.b[w] onwards
        movrel_local    lr,  right_ext_mask
        sub             lr,  lr,  r5
        vld1.8          {q13}, [lr]

        vbit            q0,  q14, q13
        vbit            q4,  q15, q13

        // Update the precalculated squares
        vmull.u8        q1,  d0,  d0
        vmull.u8        q2,  d1,  d1
        vmull.u8        q5,  d8,  d8
        vmull.u8        q6,  d9,  d9

4:      // Loop horizontally
        vext.8          d16, d0,  d1,  #1
        vext.8          d17, d0,  d1,  #2
        vext.8          d18, d8,  d9,  #1
        vext.8          d19, d8,  d9,  #2
        vaddl.u8        q3,  d0,  d16
        vaddw.u8        q3,  q3,  d17
        vaddl.u8        q7,  d8,  d18
        vaddw.u8        q7,  q7,  d19

        vext.8          q8,  q1,  q2,  #2
        vext.8          q9,  q1,  q2,  #4
        vext.8          q10, q5,  q6,  #2
        vext.8          q11, q5,  q6,  #4

        vaddl.u16       q12, d2,  d16
        vaddl.u16       q13, d3,  d17
        vaddw.u16       q12, q12, d18
        vaddw.u16       q13, q13, d19

        vaddl.u16       q8,  d10, d20
        vaddl.u16       q9,  d11, d21
        vaddw.u16       q8,  q8,  d22
        vaddw.u16       q9,  q9,  d23

        subs            r5,  r5,  #8
        vst1.16         {q3},       [r1,  :128]!
        vst1.16         {q7},       [r11, :128]!
        vst1.32         {q12, q13}, [r0,  :128]!
        vst1.32         {q8,  q9},  [r10, :128]!

        ble             9f
        tst             r7,  #2 // LR_HAVE_RIGHT
        vld1.8          {d6},  [r3]!
        vld1.8          {d14}, [r12]!
        vmov            q1,  q2
        vmov            q5,  q6
        vext.8          q0,  q0,  q3,  #8
        vext.8          q4,  q4,  q7,  #8
        vmull.u8        q2,  d6,  d6
        vmull.u8        q6,  d14, d14

        bne             4b // If we don't need to pad, just keep summing.
        b               3b // If we need to pad, check how many pixels we have left.

9:
        subs            r6,  r6,  #2
        ble             0f
        // Jump to the next row and loop horizontally
        add             r0,  r0,  r9, lsl #1
        add             r10, r10, r9, lsl #1
        add             r1,  r1,  r9
        add             r11, r11, r9
        add             r3,  r3,  r4
        add             r12, r12, r4
        mov             r5,  r8
        b               1b
0:
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

// void dav1d_sgr_box5_h_8bpc_neon(int32_t *sumsq, int16_t *sum,
//                                 const pixel (*left)[4],
//                                 const pixel *src, const ptrdiff_t stride,
//                                 const int w, const int h,
//                                 const enum LrEdgeFlags edges);
function sgr_box5_h_8bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldrd            r6,  r7,  [sp, #108]
        add             r5,  r5,  #2 // w += 2

        // Set up pointers for reading/writing alternate rows
        add             r10, r0,  #(4*SUM_STRIDE)   // sumsq
        add             r11, r1,  #(2*SUM_STRIDE)   // sum
        add             r12, r3,  r4                // src
        lsl             r4,  r4,  #1
        mov             r9,       #(2*2*SUM_STRIDE) // double sum stride

        // Subtract the aligned width from the output stride.
        add             lr,  r5,  #7
        bic             lr,  lr,  #7
        sub             r9,  r9,  lr, lsl #1
        add             lr,  lr,  #8
        sub             r4,  r4,  lr

        // Store the width for the vertical loop
        mov             r8,  r5

        // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
        tst             r7,  #1 // LR_HAVE_LEFT
        beq             2f
        // LR_HAVE_LEFT
        cmp             r2,  #0
        bne             0f
        // left == NULL
        sub             r3,  r3,  #3
        sub             r12, r12, #3
        b               1f
0:      // LR_HAVE_LEFT, left != NULL
2:      // !LR_HAVE_LEFT, increase the stride.
        // For this case we don't read the left 3 pixels from the src pointer,
        // but shift it as if we had done that.
        add             r4,  r4,  #3

1:      // Loop vertically
        vld1.8          {q0}, [r3]!
        vld1.8          {q4}, [r12]!

        tst             r7,  #1 // LR_HAVE_LEFT
        beq             0f
        cmp             r2,  #0
        beq             2f
        // LR_HAVE_LEFT, left != NULL
        vld1.32         {d3[]}, [r2]!
        // Move r3/r12 back to account for the last 3 bytes we loaded earlier,
        // which we'll shift out.
        sub             r3,  r3,  #3
        sub             r12, r12, #3
        vld1.32         {d11[]}, [r2]!
        vext.8          q0,  q1,  q0,  #13
        vext.8          q4,  q5,  q4,  #13
        b               2f
0:
        // !LR_HAVE_LEFT, fill q1 with the leftmost byte
        // and shift q0 to have 3x the first byte at the front.
        vdup.8          q1,  d0[0]
        vdup.8          q5,  d8[0]
        // Move r3 back to account for the last 3 bytes we loaded before,
        // which we shifted out.
        sub             r3,  r3,  #3
        sub             r12, r12, #3
        vext.8          q0,  q1,  q0,  #13
        vext.8          q4,  q5,  q4,  #13

2:
        vmull.u8        q1,  d0,  d0
        vmull.u8        q2,  d1,  d1
        vmull.u8        q5,  d8,  d8
        vmull.u8        q6,  d9,  d9

        tst             r7,  #2 // LR_HAVE_RIGHT
        bne             4f
        // If we'll need to pad the right edge, load that byte to pad with
        // here since we can find it pretty easily from here.
        sub             lr,  r5,  #(2 + 16 - 3 + 1)
        ldrb            r11, [r3,  lr]
        ldrb            lr,  [r12, lr]
        // Fill q14/q15 with the right padding pixel
        vdup.8          q14, r11
        vdup.8          q15, lr
        // Restore r11 after using it for a temporary value
        add             r11, r1,  #(2*SUM_STRIDE)
3:      // !LR_HAVE_RIGHT

        // Check whether we need to pad the right edge
        cmp             r5,  #11
        bge             4f   // If w >= 11, all used input pixels are valid

        // 1 <= w < 11, w+1 pixels valid in q0. For w=9 or w=10,
        // this ends up called again; it's not strictly needed in those
        // cases (we pad enough here), but keeping the code as simple as possible.

        // Insert padding in q0/4.b[w+1] onwards; fuse the +1 into the
        // buffer pointer.
        movrel_local    lr,  right_ext_mask, -1
        sub             lr,  lr,  r5
        vld1.8          {q13}, [lr]

        vbit            q0,  q14, q13
        vbit            q4,  q15, q13

        // Update the precalculated squares
        vmull.u8        q1,  d0,  d0
        vmull.u8        q2,  d1,  d1
        vmull.u8        q5,  d8,  d8
        vmull.u8        q6,  d9,  d9

4:      // Loop horizontally
        vext.8          d16, d0,  d1,  #1
        vext.8          d17, d0,  d1,  #2
        vext.8          d18, d0,  d1,  #3
        vext.8          d19, d0,  d1,  #4
        vext.8          d20, d8,  d9,  #1
        vext.8          d21, d8,  d9,  #2
        vext.8          d22, d8,  d9,  #3
        vext.8          d23, d8,  d9,  #4
        vaddl.u8        q3,  d0,  d16
        vaddl.u8        q12, d17, d18
        vaddl.u8        q7,  d8,  d20
        vaddl.u8        q13, d21, d22
        vaddw.u8        q3,  q3,  d19
        vaddw.u8        q7,  q7,  d23
        vadd.u16        q3,  q3,  q12
        vadd.u16        q7,  q7,  q13

        vext.8          q8,  q1,  q2,  #2
        vext.8          q9,  q1,  q2,  #4
        vext.8          q10, q1,  q2,  #6
        vext.8          q11, q1,  q2,  #8
        vaddl.u16       q12, d2,  d16
        vaddl.u16       q13, d3,  d17
        vaddl.u16       q8,  d18, d20
        vaddl.u16       q9,  d19, d21
        vaddw.u16       q12, q12, d22
        vaddw.u16       q13, q13, d23
        vadd.i32        q12, q12, q8
        vadd.i32        q13, q13, q9
        vext.8          q8,  q5,  q6,  #2
        vext.8          q9,  q5,  q6,  #4
        vext.8          q10, q5,  q6,  #6
        vext.8          q11, q5,  q6,  #8
        vaddl.u16       q1,  d10, d16
        vaddl.u16       q5,  d11, d17
        vaddl.u16       q8,  d18, d20
        vaddl.u16       q9,  d19, d21
        vaddw.u16       q1,  q1,  d22
        vaddw.u16       q5,  q5,  d23
        vadd.i32        q10, q1,  q8
        vadd.i32        q11, q5,  q9

        subs            r5,  r5,  #8
        vst1.16         {q3},       [r1,  :128]!
        vst1.16         {q7},       [r11, :128]!
        vst1.32         {q12, q13}, [r0,  :128]!
        vst1.32         {q10, q11}, [r10, :128]!

        ble             9f
        tst             r7,  #2 // LR_HAVE_RIGHT
        vld1.8          {d6},  [r3]!
        vld1.8          {d14}, [r12]!
        vmov            q1,  q2
        vmov            q5,  q6
        vext.8          q0,  q0,  q3,  #8
        vext.8          q4,  q4,  q7,  #8
        vmull.u8        q2,  d6,  d6
        vmull.u8        q6,  d14, d14
        bne             4b // If we don't need to pad, just keep summing.
        b               3b // If we need to pad, check how many pixels we have left.

9:
        subs            r6,  r6,  #2
        ble             0f
        // Jump to the next row and loop horizontally
        add             r0,  r0,  r9, lsl #1
        add             r10, r10, r9, lsl #1
        add             r1,  r1,  r9
        add             r11, r11, r9
        add             r3,  r3,  r4
        add             r12, r12, r4
        mov             r5,  r8
        b               1b
0:
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc

sgr_funcs 8
