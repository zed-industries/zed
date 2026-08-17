/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Janne Grunau
 * Copyright © 2018, Martin Storsjo
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

.macro avg dst0, dst1, t0, t1, t2, t3
        vld1.16         {\t0,\t1},   [r2, :128]!
        vld1.16         {\t2,\t3},   [r3, :128]!
        vadd.i16        \t0,   \t0,  \t2
        vadd.i16        \t1,   \t1,  \t3
        vqrshrun.s16    \dst0, \t0,  #5
        vqrshrun.s16    \dst1, \t1,  #5
.endm

.macro w_avg dst0, dst1, t0, t1, t2, t3
        vld1.16         {\t0,\t1},   [r2, :128]!
        vld1.16         {\t2,\t3},   [r3, :128]!
        vsub.i16        \t0,   \t2,  \t0
        vsub.i16        \t1,   \t3,  \t1
        vqdmulh.s16     \t0,   \t0,  q15
        vqdmulh.s16     \t1,   \t1,  q15
        vadd.i16        \t0,   \t2,  \t0
        vadd.i16        \t1,   \t3,  \t1
        vqrshrun.s16    \dst0, \t0,  #4
        vqrshrun.s16    \dst1, \t1,  #4
.endm

.macro mask dst0, dst1, t0, t1, t2, t3
        vld1.8          {q14}, [lr, :128]!
        vld1.16         {\t0,\t1},   [r2, :128]!
        vmul.i8         q14,   q14,  q15
        vld1.16         {\t2,\t3},   [r3, :128]!
        vshll.i8        q13,   d28,  #8
        vshll.i8        q14,   d29,  #8
        vsub.i16        \t0,   \t2,  \t0
        vsub.i16        \t1,   \t3,  \t1
        vqdmulh.s16     \t0,   \t0,  q13
        vqdmulh.s16     \t1,   \t1,  q14
        vadd.i16        \t0,   \t2,  \t0
        vadd.i16        \t1,   \t3,  \t1
        vqrshrun.s16    \dst0, \t0,  #4
        vqrshrun.s16    \dst1, \t1,  #4
.endm

.macro bidir_fn type
function \type\()_8bpc_neon, export=1
        push            {r4-r6,lr}
        ldrd            r4,  r5,  [sp, #16]
        clz             r4,  r4
.ifnc \type, avg
        ldr             lr, [sp, #24]
.endif
.ifc \type, w_avg
        vdup.s16        q15, lr
        vneg.s16        q15, q15
        vshl.i16        q15, q15, #11
.endif
.ifc \type, mask
        vmov.i8         q15, #256-2
.endif
        adr             r12, L(\type\()_tbl)
        sub             r4,  r4,  #24
        ldr             r4,  [r12, r4, lsl #2]
        \type           d16, d17, q0,  q1,  q2,  q3
        add             r12, r12, r4
        bx              r12

        .align 2
L(\type\()_tbl):
        .word 1280f - L(\type\()_tbl) + CONFIG_THUMB
        .word 640f  - L(\type\()_tbl) + CONFIG_THUMB
        .word 320f  - L(\type\()_tbl) + CONFIG_THUMB
        .word 160f  - L(\type\()_tbl) + CONFIG_THUMB
        .word 80f   - L(\type\()_tbl) + CONFIG_THUMB
        .word 4f    - L(\type\()_tbl) + CONFIG_THUMB

4:
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        cmp             r5,  #4
        vst1.32         {d16[0]},  [r0, :32], r1
        vst1.32         {d16[1]},  [r6, :32], r1
        vst1.32         {d17[0]},  [r0, :32], r1
        vst1.32         {d17[1]},  [r6, :32], r1
        beq             0f
        \type           d18, d19,  q0,  q1,  q2,  q3
        cmp             r5,  #8
        vst1.32         {d18[0]},  [r0, :32], r1
        vst1.32         {d18[1]},  [r6, :32], r1
        vst1.32         {d19[0]},  [r0, :32], r1
        vst1.32         {d19[1]},  [r6, :32], r1
        beq             0f
        \type           d16, d17, q0,  q1,  q2,  q3
        vst1.32         {d16[0]},  [r0, :32], r1
        vst1.32         {d16[1]},  [r6, :32], r1
        \type           d18, d19,  q0,  q1,  q2,  q3
        vst1.32         {d17[0]},  [r0, :32], r1
        vst1.32         {d17[1]},  [r6, :32], r1
        vst1.32         {d18[0]},  [r0, :32], r1
        vst1.32         {d18[1]},  [r6, :32], r1
        vst1.32         {d19[0]},  [r0, :32], r1
        vst1.32         {d19[1]},  [r6, :32], r1
        pop             {r4-r6,pc}
80:
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
8:
        vst1.8          {d16},  [r0, :64], r1
        \type           d18, d19, q0,  q1,  q2,  q3
        vst1.8          {d17},  [r6, :64], r1
        vst1.8          {d18},  [r0, :64], r1
        subs            r5,  r5,  #4
        vst1.8          {d19},  [r6, :64], r1
        ble             0f
        \type           d16, d17, q0,  q1,  q2,  q3
        b               8b
160:
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
16:
        \type           d18, d19, q0, q1, q2, q3
        vst1.8          {q8},  [r0, :128], r1
        \type           d20, d21, q0, q1, q2, q3
        vst1.8          {q9},  [r6, :128], r1
        \type           d22, d23, q0, q1, q2, q3
        vst1.8          {q10}, [r0, :128], r1
        subs            r5,  r5,  #4
        vst1.8          {q11}, [r6, :128], r1
        ble             0f
        \type           d16, d17, q0, q1, q2, q3
        b               16b
320:
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
32:
        \type           d18, d19, q0, q1, q2, q3
        \type           d20, d21, q0, q1, q2, q3
        vst1.8          {q8,  q9},  [r0, :128], r1
        \type           d22, d23, q0, q1, q2, q3
        subs            r5,  r5,  #2
        vst1.8          {q10, q11}, [r6, :128], r1
        ble             0f
        \type           d16, d17, q0, q1, q2, q3
        b               32b
640:
        add             r6,  r0,  #32
64:
        \type           d18, d19, q0, q1, q2, q3
        \type           d20, d21, q0, q1, q2, q3
        \type           d22, d23, q0, q1, q2, q3
        vst1.8          {q8,  q9},  [r0, :128], r1
        \type           d16, d17, q0, q1, q2, q3
        vst1.8          {q10, q11}, [r6, :128], r1
        \type           d18, d19, q0, q1, q2, q3
        \type           d20, d21, q0, q1, q2, q3
        vst1.8          {q8,  q9},  [r0, :128], r1
        \type           d22, d23, q0, q1, q2, q3
        subs            r5,  r5,  #2
        vst1.8          {q10, q11}, [r6, :128], r1
        ble             0f
        \type           d16, d17, q0, q1, q2, q3
        b               64b
1280:
        sub             r1,  r1,  #32
        add             r6,  r0,  #64
128:
        \type           d18, d19, q0, q1, q2, q3
        \type           d20, d21, q0, q1, q2, q3
        \type           d22, d23, q0, q1, q2, q3
        vst1.8          {q8,  q9},  [r0, :128]!
        \type           d16, d17, q0, q1, q2, q3
        vst1.8          {q10, q11}, [r0, :128], r1
        \type           d18, d19, q0, q1, q2, q3
        \type           d20, d21, q0, q1, q2, q3
        vst1.8          {q8,  q9},  [r6, :128]!
        \type           d22, d23, q0, q1, q2, q3
        subs            r5,  r5,  #1
        vst1.8          {q10, q11}, [r6, :128], r1
        ble             0f
        \type           d16, d17, q0, q1, q2, q3
        b               128b

0:
        pop             {r4-r6,pc}
endfunc
.endm

bidir_fn avg
bidir_fn w_avg
bidir_fn mask


.macro w_mask_fn type
function w_mask_\type\()_8bpc_neon, export=1
        push            {r4-r9,lr}
        ldrd            r4,  r5,  [sp, #28]
        ldrd            r6,  r7,  [sp, #36]
        clz             r8,  r4
        adr             r9,  L(w_mask_\type\()_tbl)
        sub             r8,  r8,  #24
        ldr             r8,  [r9,  r8,  lsl #2]
        add             r9,  r9,  r8
        movw            r12, #6903
        vdup.16         q14, r12
.if \type == 444
        vmov.i8         q15, #64
.elseif \type == 422
        vdup.8          d0,  r7         // d0[] <- sign
        vmov.i8         d30, #129
        vsub.i8         d30, d30, d0    // 129 - sign
.elseif \type == 420
        vdup.16         q0,  r7         // d0[] <- sign
        vmov.i16        q15, #256
        vsub.i16        q15, q15, q0    // 256 - sign
.endif
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        bx              r9

        .align 2
L(w_mask_\type\()_tbl):
        .word 1280f  - L(w_mask_\type\()_tbl) + CONFIG_THUMB
        .word 640f   - L(w_mask_\type\()_tbl) + CONFIG_THUMB
        .word 320f   - L(w_mask_\type\()_tbl) + CONFIG_THUMB
        .word 160f   - L(w_mask_\type\()_tbl) + CONFIG_THUMB
        .word 8f     - L(w_mask_\type\()_tbl) + CONFIG_THUMB
        .word 4f     - L(w_mask_\type\()_tbl) + CONFIG_THUMB

4:
        vld1.16         {d0,  d1,  d2,  d3},  [r2,  :128]! // tmp1 (four rows at once)
        vld1.16         {d4,  d5,  d6,  d7},  [r3,  :128]! // tmp2 (four rows at once)
        subs            r5,  r5,  #4
        vsub.i16        q8,  q2,  q0    // tmp2-tmp1
        vsub.i16        q9,  q3,  q1
        vabd.s16        q10, q0,  q2    // (abs(tmp1[x] - tmp2[x]))
        vabd.s16        q11, q1,  q3
        vqsub.u16       q10, q14, q10   // 6903 - abs ()
        vqsub.u16       q11, q14, q11
        vshr.s16        q10, q10, #8    // 64-m = (6903 - abs()) >> 8
        vshr.s16        q11, q11, #8
        vshl.s16        q12, q10, #9    // (64-m)<<9
        vshl.s16        q13, q11, #9
        vqdmulh.s16     q12, q12, q8    // ((tmp2-tmp1)*(64-m)<<9)>>15
        vqdmulh.s16     q13, q13, q9
        vadd.i16        q12, q12, q0    // (((tmp2-tmp1)*(64-m)<<9)>>15) + tmp1
        vadd.i16        q13, q13, q1
        vqrshrun.s16    d24, q12, #4    // (((((tmp2-tmp1)*(64-m)<<9)>>15) + tmp1) + 8) >> 4
        vqrshrun.s16    d25, q13, #4
.if \type == 444
        vmovn.u16       d20, q10        // 64 - m
        vmovn.u16       d21, q11
        vsub.i8         q10, q15, q10   // m
        vst1.8          {d20, d21}, [r6,  :128]!
.elseif \type == 422
        vpadd.s16       d20, d20, d21   // (64 - m) + (64 - n) (column wise addition)
        vpadd.s16       d21, d22, d23
        vmovn.s16       d6,  q10
        vhsub.u8        d6,  d30, d6    // ((129 - sign) - ((64 - m) + (64 - n))) >> 1
        vst1.8          {d6},  [r6,  :64]!
.elseif \type == 420
        vadd.s16        d20, d20, d21   // (64 - my1) + (64 - my2) (row wise addition)
        vadd.s16        d21, d22, d23
        vpadd.s16       d20, d20, d21   // (128 - m) + (128 - n) (column wise addition)
        vsub.s16        d20, d30, d20   // (256 - sign) - ((128 - m) + (128 - n))
        vrshrn.u16      d20, q10,  #2   // ((256 - sign) - ((128 - m) + (128 - n)) + 2) >> 2
        vst1.32         {d20[0]}, [r6,  :32]!
.endif
        vst1.32         {d24[0]}, [r0,  :32], r1
        vst1.32         {d24[1]}, [r12, :32], r1
        vst1.32         {d25[0]}, [r0,  :32], r1
        vst1.32         {d25[1]}, [r12, :32], r1
        bgt             4b
        pop             {r4-r9,pc}
8:
        vld1.16         {d0,  d1,  d2,  d3},  [r2,  :128]! // tmp1y1, tmp1y2
        vld1.16         {d4,  d5,  d6,  d7},  [r3,  :128]! // tmp2y1, tmp2y2
        subs            r5,  r5,  #2
        vsub.i16        q8,  q2,  q0    // tmp2y1 - tmp1y1
        vsub.i16        q9,  q3,  q1    // tmp2y2 - tmp1y2
        vabd.s16        q10, q0,  q2    // abs(tmp1y1 - tmp2y1)
        vabd.s16        q11, q1,  q3    // abs(tmp1y2 - tmp2y2)
        vqsub.u16       q10, q14, q10   // 6903 - abs(tmp1y1 - tmp2y1)
        vqsub.u16       q11, q14, q11   // 6903 - abs(tmp1y2 - tmp2y2)
        vshr.s16        q10, q10, #8    // 64 - my1 = 6903 - abs(tmp1y1 - tmp2y1) >> 8
        vshr.s16        q11, q11, #8    // 64 - my2 = 6903 - abs(tmp1y2 - tmp2y2) >> 8
        vshl.s16        q12, q10, #9    // (64 - my1) << 9
        vshl.s16        q13, q11, #9    // (64 - my2) << 9
        vqdmulh.s16     q12, q12, q8    // ((tmp2y1 - tmp1y1) * (64 - my1) << 9) >> 15
        vqdmulh.s16     q13, q13, q9    // ((tmp2y2 - tmp1y2) * (64 - my2) << 9) >> 15
        vadd.s16        q12, q12, q0    // (((tmp2y1 - tmp1y1) * (64 - my1) << 9) >> 15) + tmp1y1
        vadd.s16        q13, q13, q1    // (((tmp2y2 - tmp1y2) * (64 - my2) << 9) >> 15) + tmp1y2
        vqrshrun.s16    d24, q12, #4    // (((((tmp2y1 - tmp1y1) * (64 - my1) << 9) >> 15) + tmp1y1) + 8) >> 4
        vqrshrun.s16    d25, q13, #4    // (((((tmp2y2 - tmp1y2) * (64 - my2) << 9) >> 15) + tmp1y2) + 8) >> 4
.if \type == 444
        vmovn.u16       d20, q10        // 64 - m
        vmovn.u16       d21, q11
        vsub.i8         q10, q15, q10   // m
        vst1.8          {d20, d21}, [r6,  :128]!
.elseif \type == 422
        vpadd.s16       d20, d20, d21   // (64 - my1) + (64 - ny1) (column wise addition)
        vpadd.s16       d21, d22, d23   // (64 - my2) + (64 - ny2)
        vmovn.s16       d20, q10
        vhsub.u8        d20, d30, d20   // ((129 - sign) - ((64 - my1/y2) + (64 - ny1/y2))) >> 1
        vst1.8          {d20}, [r6,  :64]!
.elseif \type == 420
        vadd.s16        q10, q10, q11   // (64 - my1) + (64 - my2) (row wise addition)
        vpadd.s16       d20, d20, d21   // (128 - m) + (128 - n) (column wise addition)
        vsub.s16        d20, d30, d20   // (256 - sign) - ((128 - m) + (128 - n))
        vrshrn.u16      d20, q10, #2    // ((256 - sign) - ((128 - m) + (128 - n)) + 2) >> 2
        vst1.32         {d20[0]}, [r6,  :32]!
.endif
        vst1.16         {d24}, [r0,  :64], r1
        vst1.16         {d25}, [r12, :64], r1
        bgt             8b
        pop             {r4-r9,pc}
1280:
640:
320:
160:
        sub             r1,  r1,  r4
.if \type == 444
        add             lr,  r6,  r4
.elseif \type == 422
        add             lr,  r6,  r4,  lsr #1
.endif
        add             r9,  r3,  r4,  lsl #1
        add             r7,  r2,  r4,  lsl #1
161:
        mov             r8,  r4
16:
        vld1.16         {d0,  d1,  d2,  d3},  [r2,  :128]! // tmp1y1
        vld1.16         {d4,  d5,  d6,  d7},  [r3,  :128]! // tmp2y1
        vld1.16         {d16, d17, d18, d19}, [r7,  :128]! // tmp1y2
        subs            r8,  r8,  #16
        vsub.i16        q2,  q2,  q0    // tmp2y1 - tmp1y1
        vsub.i16        q3,  q3,  q1
        vabs.s16        q10, q2         // abs(tm2y1 - tmp1y1)
        vabs.s16        q11, q3
        vqsub.u16       q10, q14, q10   // 6903 - abs(tmp1y1 - tmp2y1)
        vqsub.u16       q11, q14, q11
        vshr.s16        q10, q10, #8    // 64 - my1 = 6903 - abs(tmp1y1 - tmp2y1) >> 8
        vshr.s16        q11, q11, #8
        vshl.s16        q12, q10, #9    // (64 - my1) << 9
        vshl.s16        q13, q11, #9
        vqdmulh.s16     q12, q12, q2    // ((tmp2y1 - tmp1y1) * (64 - my1) << 9) >> 15
        vqdmulh.s16     q13, q13, q3
        vadd.i16        q12, q12, q0    // (((tmp2y1 - tmp1y1) * (64 - my1) << 9) >> 15) + tmp1y1
        vadd.i16        q13, q13, q1
        vld1.16         {d0,  d1,  d2,  d3},  [r9,  :128]! // tmp2h2
.if \type == 444
        vmovn.u16       d20, q10        // 64 - my1
        vmovn.u16       d21, q11
        vsub.i8         q10, q15, q10   // my1
        vst1.8          {d20, d21}, [r6,  :128]!
.elseif \type == 422
        vpadd.s16       d20, d20, d21   // (64 - my1) + (64 - ny1) (column wise addition)
        vpadd.s16       d21, d22, d23
        vmovn.s16       d20, q10
        vhsub.u8        d20, d30, d20   // ((129 - sign) - ((64 - my1) + (64 - ny1))) >> 1
        vst1.8          {d20}, [r6,  :64]!
.endif
        vqrshrun.s16    d24, q12, #4    // (((((tmp2y1 - tmp1y1)*(64 - my1) << 9) >> 15) + tmp1y1) + 8) >> 4
        vqrshrun.s16    d25, q13, #4
        vsub.i16        q0,  q0,  q8    // tmp2y2 - tmp1y2
        vsub.i16        q1,  q1,  q9
        vst1.16         {d24, d25}, [r0,  :128]!    // store dsty1
        vabs.s16        q2,  q0         // abs(tmp2y2 - tmp1y2)
        vabs.s16        q3,  q1
        vqsub.u16       q2,  q14, q2    // 6903 - abs(tmp2y2 - tmp1y2)
        vqsub.u16       q3,  q14, q3
        vshr.s16        q2,  q2,  #8    // (6903 - abs(tmp2y2 - tmp1y2)) >> 8
        vshr.s16        q3,  q3,  #8
        vshl.s16        q12, q2,  #9    // (64 - my2) << 9
        vshl.s16        q13, q3,  #9
.if \type == 444
        vmovn.u16       d4,  q2         // 64 - my2
        vmovn.u16       d5,  q3
        vsub.i8         q2,  q15, q2    // my2
        vst1.8          {d4,  d5},  [lr,  :128]!
.elseif \type == 422
        vpadd.s16       d4,  d4,  d5    // (64 - my2) + (64 - ny2) (column wise addition)
        vpadd.s16       d5,  d6,  d7
        vmovn.s16       d4,  q2
        vhsub.u8        d4,  d30, d4    // ((129 - sign) - ((64 - my2) + (64 - ny2))) >> 1
        vst1.8          {d4},  [lr,  :64]!
.elseif \type == 420
        vadd.s16        q10, q10, q2    // (64 - my1) + (64 - my2) (row wise addition)
        vadd.s16        q11, q11, q3
        vpadd.s16       d20, d20, d21   // (128 - m) + (128 - n) (column wise addition)
        vpadd.s16       d21, d22, d23
        vsub.s16        q10, q15, q10   // (256 - sign) - ((128 - m) + (128 - n))
        vrshrn.u16      d20, q10, #2    // ((256 - sign) - ((128 - m) + (128 - n)) + 2) >> 2
        vst1.8          {d20}, [r6,  :64]!
.endif
        vqdmulh.s16     q12, q12, q0    // ((tmp2y2 - tmp1y2) * (64 - my2) << 9) >> 15
        vqdmulh.s16     q13, q13, q1
        vadd.i16        q12, q12, q8    // (((tmp2y2 - tmp1y2) * (64 - my2) << 9) >> 15) + tmp1y2
        vadd.i16        q13, q13, q9
        vqrshrun.s16    d24, q12, #4    // (((((tmp2y2 - tmp1y2)*(64 - my2) << 9) >> 15) + tmp1y2) + 8) >> 4
        vqrshrun.s16    d25, q13, #4
        vst1.16         {d24, d25}, [r12, :128]!   // store dsty2
        bgt             16b
        subs            r5,  r5,  #2
        add             r2,  r2,  r4,  lsl #1
        add             r3,  r3,  r4,  lsl #1
        add             r7,  r7,  r4,  lsl #1
        add             r9,  r9,  r4,  lsl #1
.if \type == 444
        add             r6,  r6,  r4
        add             lr,  lr,  r4
.elseif \type == 422
        add             r6,  r6,  r4,  lsr #1
        add             lr,  lr,  r4,  lsr #1
.endif
        add             r0,  r0,  r1
        add             r12, r12, r1
        bgt             161b
        pop             {r4-r9,pc}
endfunc
.endm

w_mask_fn 444
w_mask_fn 422
w_mask_fn 420


function blend_8bpc_neon, export=1
        push            {r4-r5,lr}
        ldrd            r4,  r5,  [sp, #12]
        clz             lr,  r3
        adr             r3,  L(blend_tbl)
        sub             lr,  lr,  #26
        ldr             lr,  [r3, lr, lsl #2]
        add             r3,  r3,  lr
        bx              r3

        .align 2
L(blend_tbl):
        .word 320f  - L(blend_tbl) + CONFIG_THUMB
        .word 160f  - L(blend_tbl) + CONFIG_THUMB
        .word 80f   - L(blend_tbl) + CONFIG_THUMB
        .word 40f   - L(blend_tbl) + CONFIG_THUMB

40:
        vmov.i8         d22, #64
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
4:
        vld1.u8         {d2},     [r5,  :64]!
        vld1.u8         {d1},     [r2,  :64]!
        vld1.32         {d0[]},   [r0,  :32]
        subs            r4,  r4,  #2
        vld1.32         {d0[1]},  [r12, :32]
        vsub.i8         d3,  d22, d2
        vmull.u8        q8,  d1,  d2
        vmlal.u8        q8,  d0,  d3
        vrshrn.i16      d20, q8,  #6
        vst1.32         {d20[0]}, [r0,  :32], r1
        vst1.32         {d20[1]}, [r12, :32], r1
        bgt             4b
        pop             {r4-r5,pc}
80:
        vmov.i8         d16, #64
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
8:
        vld1.u8         {q1},  [r5,  :128]!
        vld1.u8         {q2},  [r2,  :128]!
        vld1.u8         {d0},  [r0,  :64]
        vsub.i8         d17, d16, d2
        vld1.u8         {d1},  [r12, :64]
        subs            r4,  r4,  #2
        vsub.i8         d18, d16, d3
        vmull.u8        q3,  d2,  d4
        vmlal.u8        q3,  d0,  d17
        vmull.u8        q10, d3,  d5
        vmlal.u8        q10, d1,  d18
        vrshrn.i16      d22, q3,  #6
        vrshrn.i16      d23, q10, #6
        vst1.u8         {d22}, [r0,  :64], r1
        vst1.u8         {d23}, [r12, :64], r1
        bgt             8b
        pop             {r4-r5,pc}
160:
        vmov.i8         q12, #64
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
16:
        vld1.u8         {q1,  q2},  [r5,  :128]!
        vld1.u8         {q8,  q9},  [r2,  :128]!
        vld1.u8         {q0},  [r0,  :128]
        subs            r4,  r4,  #2
        vsub.i8         q15, q12, q1
        vld1.u8         {q13}, [r12, :128]
        vmull.u8        q3,  d16, d2
        vmlal.u8        q3,  d0,  d30
        vmull.u8        q14, d17, d3
        vmlal.u8        q14, d1,  d31
        vsub.i8         q15, q12, q2
        vrshrn.i16      d20, q3,  #6
        vrshrn.i16      d21, q14, #6
        vmull.u8        q3,  d18, d4
        vmlal.u8        q3,  d26, d30
        vmull.u8        q14, d19, d5
        vmlal.u8        q14, d27, d31
        vrshrn.i16      d22, q3,  #6
        vrshrn.i16      d23, q14, #6
        vst1.u8         {q10}, [r0,  :128], r1
        vst1.u8         {q11}, [r12, :128], r1
        bgt             16b
        pop             {r4-r5,pc}
320:
        vmov.i8         q10, #64
32:
        vld1.u8         {q2,  q3},  [r5,  :128]!
        vld1.u8         {q8,  q9},  [r2,  :128]!
        vld1.u8         {q0,  q1},  [r0,  :128]
        subs            r4,  r4,  #1
        vsub.i8         q11, q10, q2
        vmull.u8        q15, d16, d4
        vmlal.u8        q15, d0,  d22
        vmull.u8        q14, d17, d5
        vmlal.u8        q14, d1,  d23
        vsub.i8         q11, q10, q3
        vrshrn.i16      d24, q15, #6
        vrshrn.i16      d25, q14, #6
        vmull.u8        q15, d18, d6
        vmlal.u8        q15, d2,  d22
        vmull.u8        q14, d19, d7
        vmlal.u8        q14, d3,  d23
        vrshrn.i16      d26, q15, #6
        vrshrn.i16      d27, q14, #6
        vst1.u8         {q12, q13}, [r0,  :128],  r1
        bgt             32b
        pop             {r4-r5,pc}
endfunc

function blend_h_8bpc_neon, export=1
        push            {r4-r5,lr}
        ldr             r4,  [sp, #12]
        movrel          r5,  X(obmc_masks)
        add             r5,  r5,  r4
        sub             r4,  r4,  r4,  lsr #2
        clz             lr,  r3
        adr             r12, L(blend_h_tbl)
        sub             lr,  lr,  #24
        ldr             lr,  [r12, lr, lsl #2]
        add             r12, r12, lr
        bx              r12

        .align 2
L(blend_h_tbl):
        .word 1280f  - L(blend_h_tbl) + CONFIG_THUMB
        .word 640f   - L(blend_h_tbl) + CONFIG_THUMB
        .word 320f   - L(blend_h_tbl) + CONFIG_THUMB
        .word 160f   - L(blend_h_tbl) + CONFIG_THUMB
        .word 80f    - L(blend_h_tbl) + CONFIG_THUMB
        .word 40f    - L(blend_h_tbl) + CONFIG_THUMB
        .word 20f    - L(blend_h_tbl) + CONFIG_THUMB

20:
        vmov.i8         d22, #64
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
2:
        vld1.16         {d2[], d3[]},  [r5,  :16]!
        vld1.32         {d1[]},  [r2,  :32]!
        subs            r4,  r4,  #2
        vld1.16         {d0[]},   [r0,  :16]
        vzip.8          d2,  d3
        vsub.i8         d4,  d22, d2
        vld1.16         {d0[1]},  [r12, :16]
        vmull.u8        q8,  d1,  d2
        vmlal.u8        q8,  d0,  d4
        vrshrn.i16      d20, q8,  #6
        vst1.16         {d20[0]}, [r0,  :16], r1
        vst1.16         {d20[1]}, [r12, :16], r1
        bgt             2b
        pop             {r4-r5,pc}
40:
        vmov.i8         d22, #64
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
4:
        vld2.u8         {d2[],  d3[]},   [r5,  :16]!
        vld1.u8         {d1},     [r2,  :64]!
        subs            r4,  r4,  #2
        vext.u8         d2,  d2,  d3,   #4
        vld1.32         {d0[]},   [r0,  :32]
        vsub.i8         d6,  d22, d2
        vld1.32         {d0[1]},  [r12, :32]
        vmull.u8        q8,  d1,  d2
        vmlal.u8        q8,  d0,  d6
        vrshrn.i16      d20, q8,  #6
        vst1.32         {d20[0]}, [r0,  :32], r1
        vst1.32         {d20[1]}, [r12, :32], r1
        bgt             4b
        pop             {r4-r5,pc}
80:
        vmov.i8         q8, #64
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
8:
        vld2.u8         {d2[],  d3[]},  [r5,  :16]!
        vld1.u8         {d4,  d5},  [r2,  :128]!
        vld1.u8         {d0},   [r0,  :64]
        vsub.i8         q9,  q8,  q1
        vld1.u8         {d1},   [r12, :64]
        subs            r4,  r4,  #2
        vmull.u8        q3,  d2,  d4
        vmlal.u8        q3,  d0,  d18
        vmull.u8        q10, d3,  d5
        vmlal.u8        q10, d1,  d19
        vrshrn.i16      d22, q3,  #6
        vrshrn.i16      d23, q10, #6
        vst1.u8         {d22}, [r0,  :64], r1
        vst1.u8         {d23}, [r12, :64], r1
        bgt             8b
        pop             {r4-r5,pc}
160:
        vmov.i8         q12, #64
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
16:
        vld2.u8         {d28[], d29[]}, [r5,  :16]!
        vld1.u8         {d2,  d3,  d4,  d5},  [r2,  :128]!
        vsub.i8         q15, q12, q14
        vld1.u8         {q0},  [r0,  :128]
        subs            r4,  r4,  #2
        vld1.u8         {q13}, [r12, :128]
        vmull.u8        q3,  d2,  d28
        vmlal.u8        q3,  d0,  d30
        vmull.u8        q8,  d3,  d28
        vmlal.u8        q8,  d1,  d30
        vrshrn.i16      d18, q3,  #6
        vrshrn.i16      d19, q8,  #6
        vmull.u8        q3,  d4,  d29
        vmlal.u8        q3,  d26, d31
        vmull.u8        q8,  d5,  d29
        vmlal.u8        q8,  d27, d31
        vrshrn.i16      d20, q3,  #6
        vrshrn.i16      d21, q8,  #6
        vst1.u8         {q9},  [r0,  :128], r1
        vst1.u8         {q10}, [r12, :128], r1
        bgt             16b
        pop             {r4-r5,pc}
320:
640:
1280:
        vmov.i8         d20, #64
        sub             r1,  r1,  r3
321:
        vld1.u8         {d6[]},  [r5]!
        vsub.i8         d7,  d20, d6
        mov             r12, r3
32:
        vld1.u8         {q8,  q9},  [r2,  :128]!
        vld1.u8         {q0,  q1},  [r0,  :128]
        vmull.u8        q15, d16, d6
        vmlal.u8        q15, d0,  d7
        vmull.u8        q14, d17, d6
        vmlal.u8        q14, d1,  d7
        vrshrn.i16      d0,  q15, #6
        vrshrn.i16      d1,  q14, #6
        vmull.u8        q15, d18, d6
        vmlal.u8        q15, d2,  d7
        vmull.u8        q14, d19, d6
        vmlal.u8        q14, d3,  d7
        vrshrn.i16      d2,  q15, #6
        vrshrn.i16      d3,  q14, #6
        subs            r12, r12, #32
        vst1.u8         {q0,  q1},  [r0,  :128]!
        bgt             32b
        add             r0,  r0,  r1
        subs            r4,  r4,  #1
        bgt             321b
        pop             {r4-r5,pc}
endfunc

function blend_v_8bpc_neon, export=1
        push            {r4,lr}
        ldr             r4,  [sp, #8]
        movrel          lr,  X(obmc_masks)
        add             lr,  lr,  r3
        clz             r12, r3
        adr             r3,  L(blend_v_tbl)
        sub             r12, r12, #26
        ldr             r12, [r3, r12, lsl #2]
        add             r3,  r3,  r12
        bx              r3

        .align 2
L(blend_v_tbl):
        .word 320f  - L(blend_v_tbl) + CONFIG_THUMB
        .word 160f  - L(blend_v_tbl) + CONFIG_THUMB
        .word 80f   - L(blend_v_tbl) + CONFIG_THUMB
        .word 40f   - L(blend_v_tbl) + CONFIG_THUMB
        .word 20f   - L(blend_v_tbl) + CONFIG_THUMB

20:
        vmov.i8         d22, #64
        vld1.8          {d2[]},   [lr]
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        vsub.i8         d3,  d22, d2
2:
        vld1.16         {d1[0]},  [r2,  :16]!
        vld1.8          {d0[]},   [r0]
        subs            r4,  r4,  #2
        vld1.8          {d1[1]},  [r2]
        vld1.8          {d0[1]},  [r12]
        vmull.u8        q2,  d1,  d2
        vmlal.u8        q2,  d0,  d3
        vrshrn.i16      d6,  q2,  #6
        add             r2,  r2,  #2
        vst1.8          {d6[0]},  [r0],  r1
        vst1.8          {d6[1]},  [r12], r1
        bgt             2b
        pop             {r4,pc}
40:
        vmov.i8         d22, #64
        vld1.32         {d4[]},   [lr,  :32]
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        vsub.i8         d5,  d22, d4
        sub             r1,  r1,  #2
4:
        vld1.u8         {d2},     [r2,  :64]!
        vld1.32         {d0[]},   [r0,  :32]
        vld1.32         {d0[1]},  [r12, :32]
        subs            r4,  r4,  #2
        vmull.u8        q3,  d2,  d4
        vmlal.u8        q3,  d0,  d5
        vrshrn.i16      d20, q3,  #6
        vst1.16         {d20[0]}, [r0,  :16]!
        vst1.16         {d20[2]}, [r12, :16]!
        vst1.8          {d20[2]}, [r0],  r1
        vst1.8          {d20[6]}, [r12], r1
        bgt             4b
        pop             {r4,pc}
80:
        vmov.i8         d16, #64
        vld1.u8         {d2},  [lr,  :64]
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        vsub.i8         d17, d16, d2
        sub             r1,  r1,  #4
8:
        vld1.u8         {d4,  d5},  [r2,  :128]!
        vld1.u8         {d0},  [r0,  :64]
        vld1.u8         {d1},  [r12, :64]
        subs            r4,  r4,  #2
        vmull.u8        q3,  d2,  d4
        vmlal.u8        q3,  d0,  d17
        vmull.u8        q10, d2,  d5
        vmlal.u8        q10, d1,  d17
        vrshrn.i16      d22, q3,  #6
        vrshrn.i16      d23, q10, #6
        vst1.32         {d22[0]}, [r0,  :32]!
        vst1.32         {d23[0]}, [r12, :32]!
        vst1.16         {d22[2]}, [r0,  :16], r1
        vst1.16         {d23[2]}, [r12, :16], r1
        bgt             8b
        pop             {r4,pc}
160:
        vmov.i8         q12, #64
        vld1.u8         {q14}, [lr,  :128]
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        vsub.i8         q11, q12, q14
        sub             r1,  r1,  #8
16:
        vld1.u8         {q1,  q2},  [r2,  :128]!
        vld1.u8         {q0},  [r0,  :128]
        subs            r4,  r4,  #2
        vld1.u8         {q13}, [r12, :128]
        vmull.u8        q3,  d2,  d28
        vmlal.u8        q3,  d0,  d22
        vmull.u8        q8,  d3,  d29
        vmlal.u8        q8,  d1,  d23
        vrshrn.i16      d18, q3,  #6
        vrshrn.i16      d19, q8,  #6
        vmull.u8        q3,  d4,  d28
        vmlal.u8        q3,  d26, d22
        vmull.u8        q8,  d5,  d29
        vmlal.u8        q8,  d27, d23
        vrshrn.i16      d20, q3,  #6
        vrshrn.i16      d21, q8,  #6
        vst1.u8         {d18},    [r0,  :64]!
        vst1.u8         {d20},    [r12, :64]!
        vst1.32         {d19[0]}, [r0,  :32], r1
        vst1.32         {d21[0]}, [r12, :32], r1
        bgt             16b
        pop             {r4,pc}
320:
        vmov.i8         q10, #64
        vld1.u8         {q2,  q3},  [lr,  :128]
        vsub.i8         q11, q10, q2
        vsub.i8         d24, d20, d6
32:
        vld1.u8         {q8,  q9},  [r2,  :128]!
        vld1.u8         {d0,  d1,  d2},  [r0,  :64]
        subs            r4,  r4,  #1
        vmull.u8        q15, d16, d4
        vmlal.u8        q15, d0,  d22
        vmull.u8        q14, d17, d5
        vmlal.u8        q14, d1,  d23
        vrshrn.i16      d0,  q15, #6
        vrshrn.i16      d1,  q14, #6
        vmull.u8        q15, d18, d6
        vmlal.u8        q15, d2,  d24
        vrshrn.i16      d2,  q15, #6
        vst1.u8         {d0,  d1,  d2},  [r0,  :64],  r1
        bgt             32b
        pop             {r4,pc}
endfunc


// This has got the same signature as the put_8tap functions,
// assumes that the caller has loaded the h argument into r5,
// and assumes that r8 is set to (clz(w)-24).
function put_neon
        adr             r9,  L(put_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(put_tbl):
        .word 1280f - L(put_tbl) + CONFIG_THUMB
        .word 640f  - L(put_tbl) + CONFIG_THUMB
        .word 32f   - L(put_tbl) + CONFIG_THUMB
        .word 160f  - L(put_tbl) + CONFIG_THUMB
        .word 8f    - L(put_tbl) + CONFIG_THUMB
        .word 4f    - L(put_tbl) + CONFIG_THUMB
        .word 2f    - L(put_tbl) + CONFIG_THUMB

2:
        vld1.16         {d0[]}, [r2], r3
        vld1.16         {d1[]}, [r2], r3
        subs            r5,  r5,  #2
        vst1.16         {d0[0]}, [r0, :16], r1
        vst1.16         {d1[0]}, [r0, :16], r1
        bgt             2b
        pop             {r4-r11,pc}
4:
        vld1.32         {d0[]}, [r2], r3
        vld1.32         {d1[]}, [r2], r3
        subs            r5,  r5,  #2
        vst1.32         {d0[0]}, [r0, :32], r1
        vst1.32         {d1[0]}, [r0, :32], r1
        bgt             4b
        pop             {r4-r11,pc}
8:
        vld1.8          {d0}, [r2], r3
        vld1.8          {d1}, [r2], r3
        subs            r5,  r5,  #2
        vst1.8          {d0}, [r0, :64], r1
        vst1.8          {d1}, [r0, :64], r1
        bgt             8b
        pop             {r4-r11,pc}
160:
        add             r8,  r0,  r1
        lsl             r1,  r1,  #1
        add             r9,  r2,  r3
        lsl             r3,  r3,  #1
16:
        vld1.8          {q0}, [r2], r3
        vld1.8          {q1}, [r9], r3
        subs            r5,  r5,  #2
        vst1.8          {q0}, [r0, :128], r1
        vst1.8          {q1}, [r8, :128], r1
        bgt             16b
        pop             {r4-r11,pc}
32:
        vld1.8          {q0,  q1},  [r2], r3
        subs            r5,  r5,  #1
        vst1.8          {q0,  q1},  [r0, :128], r1
        bgt             32b
        pop             {r4-r11,pc}
640:
        sub             r1,  r1,  #32
        sub             r3,  r3,  #32
64:
        vld1.8          {q0,  q1},  [r2]!
        vst1.8          {q0,  q1},  [r0, :128]!
        vld1.8          {q2,  q3},  [r2], r3
        subs            r5,  r5,  #1
        vst1.8          {q2,  q3},  [r0, :128], r1
        bgt             64b
        pop             {r4-r11,pc}
1280:
        sub             r1,  r1,  #96
        sub             r3,  r3,  #96
128:
        vld1.8          {q8,  q9},  [r2]!
        vst1.8          {q8,  q9},  [r0, :128]!
        vld1.8          {q10, q11}, [r2]!
        vst1.8          {q10, q11}, [r0, :128]!
        vld1.8          {q12, q13}, [r2]!
        vst1.8          {q12, q13}, [r0, :128]!
        vld1.8          {q14, q15}, [r2], r3
        subs            r5,  r5,  #1
        vst1.8          {q14, q15}, [r0, :128], r1
        bgt             128b
        pop             {r4-r11,pc}
endfunc


// This has got the same signature as the put_8tap functions,
// assumes that the caller has loaded the h argument into r4,
// and assumes that r8 is set to (clz(w)-24), and r7 to w*2.
function prep_neon
        adr             r9,  L(prep_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(prep_tbl):
        .word 1280f - L(prep_tbl) + CONFIG_THUMB
        .word 640f  - L(prep_tbl) + CONFIG_THUMB
        .word 320f  - L(prep_tbl) + CONFIG_THUMB
        .word 160f  - L(prep_tbl) + CONFIG_THUMB
        .word 8f    - L(prep_tbl) + CONFIG_THUMB
        .word 4f    - L(prep_tbl) + CONFIG_THUMB

4:
        vld1.32         {d0[]}, [r1], r2
        vld1.32         {d2[]}, [r1], r2
        subs            r4,  r4,  #2
        vshll.u8        q0,  d0,  #4
        vshll.u8        q1,  d2,  #4
        vst1.16         {d1, d2}, [r0, :64]!
        bgt             4b
        pop             {r4-r11,pc}
8:
        vld1.8          {d0}, [r1], r2
        vld1.8          {d2}, [r1], r2
        subs            r4,  r4,  #2
        vshll.u8        q0,  d0,  #4
        vshll.u8        q1,  d2,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        bgt             8b
        pop             {r4-r11,pc}
160:
        add             r9,  r1,  r2
        lsl             r2,  r2,  #1
        add             r8,  r0,  r7
        lsl             r7,  r7,  #1
16:
        vld1.8          {q2}, [r1], r2
        vld1.8          {q3}, [r9], r2
        subs            r4,  r4,  #2
        vshll.u8        q0,  d4,  #4
        vshll.u8        q1,  d5,  #4
        vshll.u8        q2,  d6,  #4
        vshll.u8        q3,  d7,  #4
        vst1.16         {q0, q1}, [r0, :128], r7
        vst1.16         {q2, q3}, [r8, :128], r7
        bgt             16b
        pop             {r4-r11,pc}
320:
        add             r8,  r0,  r3
32:
        vld1.8          {q0,  q1},  [r1], r2
        subs            r4,  r4,  #2
        vshll.u8        q8,  d0,  #4
        vshll.u8        q9,  d1,  #4
        vld1.8          {q2,  q3},  [r1], r2
        vshll.u8        q10, d2,  #4
        vshll.u8        q11, d3,  #4
        vshll.u8        q12, d4,  #4
        vst1.16         {q8,  q9},  [r0, :128], r7
        vshll.u8        q13, d5,  #4
        vst1.16         {q10, q11}, [r8, :128], r7
        vshll.u8        q14, d6,  #4
        vst1.16         {q12, q13}, [r0, :128], r7
        vshll.u8        q15, d7,  #4
        vst1.16         {q14, q15}, [r8, :128], r7
        bgt             32b
        pop             {r4-r11,pc}
640:
        sub             r2,  r2,  #32
        add             r8,  r0,  #32
        mov             r6,  #64
64:
        vld1.8          {q0,  q1},  [r1]!
        subs            r4,  r4,  #1
        vshll.u8        q8,  d0,  #4
        vshll.u8        q9,  d1,  #4
        vld1.8          {q2,  q3},  [r1], r2
        vshll.u8        q10, d2,  #4
        vshll.u8        q11, d3,  #4
        vshll.u8        q12, d4,  #4
        vst1.16         {q8,  q9},  [r0, :128], r6
        vshll.u8        q13, d5,  #4
        vshll.u8        q14, d6,  #4
        vst1.16         {q10, q11}, [r8, :128], r6
        vshll.u8        q15, d7,  #4
        vst1.16         {q12, q13}, [r0, :128], r6
        vst1.16         {q14, q15}, [r8, :128], r6
        bgt             64b
        pop             {r4-r11,pc}
1280:
        sub             r2,  r2,  #96
        add             r8,  r0,  #32
        mov             r6,  #64
128:
        vld1.8          {q0,  q1},  [r1]!
        vld1.8          {q2,  q3},  [r1]!
        vshll.u8        q10, d0,  #4
        vshll.u8        q11, d1,  #4
        vshll.u8        q12, d2,  #4
        vshll.u8        q13, d3,  #4
        vshll.u8        q14, d4,  #4
        vshll.u8        q15, d5,  #4
        vld1.8          {q8,  q9},  [r1]!
        vst1.16         {q10, q11}, [r0, :128], r6
        vst1.16         {q12, q13}, [r8, :128], r6
        vshll.u8        q0,  d6,  #4
        vshll.u8        q1,  d7,  #4
        vshll.u8        q2,  d16, #4
        vshll.u8        q3,  d17, #4
        vshll.u8        q8,  d18, #4
        vshll.u8        q9,  d19, #4
        vld1.8          {q10, q11}, [r1], r2
        vst1.16         {q14, q15}, [r0, :128], r6
        vst1.16         {q0,  q1},  [r8, :128], r6
        vshll.u8        q12, d20, #4
        vshll.u8        q13, d21, #4
        vshll.u8        q14, d22, #4
        vshll.u8        q15, d23, #4
        subs            r4,  r4,  #1
        vst1.16         {q2,  q3},  [r0, :128], r6
        vst1.16         {q8,  q9},  [r8, :128], r6
        vst1.16         {q12, q13}, [r0, :128], r6
        vst1.16         {q14, q15}, [r8, :128], r6
        bgt             128b
        pop             {r4-r11,pc}
endfunc


.macro load_slice s0, s1, strd, wd, d0, d1, d2, d3, d4, d5, d6
        vld1.\wd        {\d0[]}, [\s0], \strd
        vld1.\wd        {\d1[]}, [\s1], \strd
.ifnb \d2
        vld1.\wd        {\d2[]}, [\s0], \strd
        vld1.\wd        {\d3[]}, [\s1], \strd
.endif
.ifnb \d4
        vld1.\wd        {\d4[]}, [\s0], \strd
.endif
.ifnb \d5
        vld1.\wd        {\d5[]}, [\s1], \strd
.endif
.ifnb \d6
        vld1.\wd        {\d6[]}, [\s0], \strd
.endif
.endm
.macro load_reg s0, s1, strd, d0, d1, d2, d3, d4, d5, d6
        vld1.8          {\d0}, [\s0], \strd
        vld1.8          {\d1}, [\s1], \strd
.ifnb \d2
        vld1.8          {\d2}, [\s0], \strd
        vld1.8          {\d3}, [\s1], \strd
.endif
.ifnb \d4
        vld1.8          {\d4}, [\s0], \strd
.endif
.ifnb \d5
        vld1.8          {\d5}, [\s1], \strd
.endif
.ifnb \d6
        vld1.8          {\d6}, [\s0], \strd
.endif
.endm
.macro load_16 s0, s1, strd, d0, d1, d2, d3, d4, d5, d6
        load_slice      \s0, \s1, \strd, 16, \d0, \d1, \d2, \d3, \d4, \d5, \d6
.endm
.macro load_32 s0, s1, strd, d0, d1, d2, d3, d4, d5, d6
        load_slice      \s0, \s1, \strd, 32, \d0, \d1, \d2, \d3, \d4, \d5, \d6
.endm
.macro interleave_1_16 r0, r1, r2, r3, r4
        vext.8          \r0, \r0, \r1, #6
        vext.8          \r1, \r1, \r2, #6
.ifnb \r3
        vext.8          \r2, \r2, \r3, #6
        vext.8          \r3, \r3, \r4, #6
.endif
.endm
.macro interleave_1_32 r0, r1, r2, r3, r4
        vext.8          \r0, \r0, \r1, #4
        vext.8          \r1, \r1, \r2, #4
.ifnb \r3
        vext.8          \r2, \r2, \r3, #4
        vext.8          \r3, \r3, \r4, #4
.endif
.endm
.macro vmovl_u8 q0, d0, q1, d1, q2, d2, q3, d3, q4, d4, q5, d5, q6, d6
        vmovl.u8        \q0, \d0
        vmovl.u8        \q1, \d1
.ifnb \q2
        vmovl.u8        \q2, \d2
        vmovl.u8        \q3, \d3
.endif
.ifnb \q4
        vmovl.u8        \q4, \d4
.endif
.ifnb \q5
        vmovl.u8        \q5, \d5
.endif
.ifnb \q6
        vmovl.u8        \q6, \d6
.endif
.endm
.macro mul_mla_4 d, s0, s1, s2, s3
        vmul.s16        \d,  \s0,  d0[0]
        vmla.s16        \d,  \s1,  d0[1]
        vmla.s16        \d,  \s2,  d0[2]
        vmla.s16        \d,  \s3,  d0[3]
.endm
.macro mul_mla_8_0 d0, s0, s1, s2, s3, s4, s5, s6, s7
        vmul.s16        \d0, \s0,  d0[0]
        vmla.s16        \d0, \s1,  d0[1]
        vmla.s16        \d0, \s2,  d0[2]
        vmla.s16        \d0, \s3,  d0[3]
        vmla.s16        \d0, \s4,  d1[0]
        vmla.s16        \d0, \s5,  d1[1]
        vmla.s16        \d0, \s6,  d1[2]
        vmla.s16        \d0, \s7,  d1[3]
.endm
.macro mul_mla_8_1 d0, d1, s0, s1, s2, s3, s4, s5, s6, s7, s8
        vmul.s16        \d0, \s0, d0[0]
        vmla.s16        \d0, \s1, d0[1]
        vmla.s16        \d0, \s2, d0[2]
        vmla.s16        \d0, \s3, d0[3]
        vmla.s16        \d0, \s4, d1[0]
        vmla.s16        \d0, \s5, d1[1]
        vmla.s16        \d0, \s6, d1[2]
        vmla.s16        \d0, \s7, d1[3]
        vmul.s16        \d1, \s1, d0[0]
        vmla.s16        \d1, \s2, d0[1]
        vmla.s16        \d1, \s3, d0[2]
        vmla.s16        \d1, \s4, d0[3]
        vmla.s16        \d1, \s5, d1[0]
        vmla.s16        \d1, \s6, d1[1]
        vmla.s16        \d1, \s7, d1[2]
        vmla.s16        \d1, \s8, d1[3]
.endm
.macro mul_mla_8_2 d0, d1, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9
        vmul.s16        \d0, \s0, d0[0]
        vmla.s16        \d0, \s1, d0[1]
        vmla.s16        \d0, \s2, d0[2]
        vmla.s16        \d0, \s3, d0[3]
        vmla.s16        \d0, \s4, d1[0]
        vmla.s16        \d0, \s5, d1[1]
        vmla.s16        \d0, \s6, d1[2]
        vmla.s16        \d0, \s7, d1[3]
        vmul.s16        \d1, \s2, d0[0]
        vmla.s16        \d1, \s3, d0[1]
        vmla.s16        \d1, \s4, d0[2]
        vmla.s16        \d1, \s5, d0[3]
        vmla.s16        \d1, \s6, d1[0]
        vmla.s16        \d1, \s7, d1[1]
        vmla.s16        \d1, \s8, d1[2]
        vmla.s16        \d1, \s9, d1[3]
.endm
.macro vqrshrun_s16 shift, q0, d0, q1, d1, q2, d2, q3, d3
        vqrshrun.s16    \d0, \q0, #\shift
.ifnb \q1
        vqrshrun.s16    \d1, \q1, #\shift
.endif
.ifnb \q2
        vqrshrun.s16    \d2, \q2, #\shift
        vqrshrun.s16    \d3, \q3, #\shift
.endif
.endm
.macro vrshr_s16 shift, r0, r1, r2, r3
        vrshr.s16       \r0, \r0, #\shift
.ifnb \r1
        vrshr.s16       \r1, \r1, #\shift
.endif
.ifnb \r2
        vrshr.s16       \r2, \r2, #\shift
        vrshr.s16       \r3, \r3, #\shift
.endif
.endm
.macro st_16 strd, reg, lanes
        vst1.16         {\reg[0]}, [r0, :16], \strd
        vst1.16         {\reg[1]}, [r8, :16], \strd
.if \lanes > 2
        vst1.16         {\reg[2]}, [r0, :16], \strd
        vst1.16         {\reg[3]}, [r8, :16], \strd
.endif
.endm
.macro st_32 strd, r0, r1
        vst1.32         {\r0[0]}, [r0, :32], \strd
        vst1.32         {\r0[1]}, [r8, :32], \strd
.ifnb \r1
        vst1.32         {\r1[0]}, [r0, :32], \strd
        vst1.32         {\r1[1]}, [r8, :32], \strd
.endif
.endm
.macro st_reg strd, align, r0, r1, r2, r3, r4, r5, r6, r7
        vst1.8          {\r0}, [r0, \align], \strd
        vst1.8          {\r1}, [r8, \align], \strd
.ifnb \r2
        vst1.8          {\r2}, [r0, \align], \strd
        vst1.8          {\r3}, [r8, \align], \strd
.endif
.ifnb \r4
        vst1.8          {\r4}, [r0, \align], \strd
        vst1.8          {\r5}, [r8, \align], \strd
        vst1.8          {\r6}, [r0, \align], \strd
        vst1.8          {\r7}, [r8, \align], \strd
.endif
.endm
.macro shift_store_4 type, strd, q0, d0, d1, q1, d2, d3
.ifc \type, put
        vqrshrun_s16    6,     \q0, \d0, \q1, \d2
        st_32           \strd, \d0, \d2
.else
        vrshr_s16       2,          \q0, \q1
        st_reg          \strd, :64, \d0, \d1, \d2, \d3
.endif
.endm
.macro shift_store_8 type, strd, q0, d0, q1, d1, q2, d2, q3, d3
.ifc \type, put
        vqrshrun_s16    6,          \q0, \d0, \q1, \d1, \q2, \d2, \q3, \d3
        st_reg          \strd, :64, \d0, \d1, \d2, \d3
.else
        vrshr_s16       2,          \q0, \q1, \q2, \q3
        st_reg          \strd, :128,\q0, \q1, \q2, \q3
.endif
.endm
.macro shift_store_16 type, strd, q0, d0, d1, q1, q2, d4, d5, q3
.ifc \type, put
        vqrshrun.s16    \d0,   \q0, #6
        vqrshrun.s16    \d1,   \q1, #6
        vqrshrun.s16    \d4,   \q2, #6
        vqrshrun.s16    \d5,   \q3, #6
        st_reg          \strd, :128, \q0, \q2
.else
        vrshr_s16       2,     \q0, \q1, \q2, \q3
        vst1.16         {\q0, \q1}, [r0, :128], \strd
        vst1.16         {\q2, \q3}, [r8, :128], \strd
.endif
.endm

.macro make_8tap_fn op, type, type_h, type_v
function \op\()_8tap_\type\()_8bpc_neon, export=1
        push            {r4-r11,lr}
        movw            r8,  \type_h
        movw            r9,  \type_v
        b               \op\()_8tap_neon
endfunc
.endm

// No spaces in these expressions, due to gas-preprocessor.
#define REGULAR ((0*15<<7)|3*15)
#define SMOOTH  ((1*15<<7)|4*15)
#define SHARP   ((2*15<<7)|3*15)

.macro filter_fn type, dst, d_strd, src, s_strd, w, h, mx, my, ds2, sr2, shift_hv
make_8tap_fn \type, regular,        REGULAR, REGULAR
make_8tap_fn \type, regular_smooth, REGULAR, SMOOTH
make_8tap_fn \type, regular_sharp,  REGULAR, SHARP
make_8tap_fn \type, smooth,         SMOOTH,  SMOOTH
make_8tap_fn \type, smooth_regular, SMOOTH,  REGULAR
make_8tap_fn \type, smooth_sharp,   SMOOTH,  SHARP
make_8tap_fn \type, sharp,          SHARP,   SHARP
make_8tap_fn \type, sharp_regular,  SHARP,   REGULAR
make_8tap_fn \type, sharp_smooth,   SHARP,   SMOOTH

function \type\()_8tap_neon
        ldrd            r4,  r5,  [sp, #36]
        ldrd            r6,  r7,  [sp, #44]
        movw            r10,  #0x4081  // (1 << 14) | (1 << 7) | (1 << 0)
        mul             \mx,  \mx, r10
        mul             \my,  \my, r10
        add             \mx,  \mx, r8 // mx, 8tap_h, 4tap_h
        add             \my,  \my, r9 // my, 8tap_v, 4tap_v
.ifc \type, prep
        lsl             \d_strd, \w, #1
.endif

        clz             r8,  \w
        tst             \mx, #(0x7f << 14)
        sub             r8,  r8,  #24
        movrel          r10, X(mc_subpel_filters), -8
        bne             L(\type\()_8tap_h)
        tst             \my, #(0x7f << 14)
        bne             L(\type\()_8tap_v)
        b               \type\()_neon

L(\type\()_8tap_h):
        cmp             \w,  #4
        ubfx            r9,  \mx, #7, #7
        and             \mx, \mx, #0x7f
        it              gt
        movgt           \mx,  r9
        tst             \my,  #(0x7f << 14)
        add             \mx, r10, \mx, lsl #3
        bne             L(\type\()_8tap_hv)

        adr             r9,  L(\type\()_8tap_h_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(\type\()_8tap_h_tbl):
        .word 1280f - L(\type\()_8tap_h_tbl) + CONFIG_THUMB
        .word 640f  - L(\type\()_8tap_h_tbl) + CONFIG_THUMB
        .word 320f  - L(\type\()_8tap_h_tbl) + CONFIG_THUMB
        .word 160f  - L(\type\()_8tap_h_tbl) + CONFIG_THUMB
        .word 80f   - L(\type\()_8tap_h_tbl) + CONFIG_THUMB
        .word 40f   - L(\type\()_8tap_h_tbl) + CONFIG_THUMB
        .word 20f   - L(\type\()_8tap_h_tbl) + CONFIG_THUMB

20:     // 2xN h
.ifc \type, put
        add             \mx,  \mx,  #2
        vld1.32         {d0[]}, [\mx]
        sub             \src,  \src,  #1
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
        vmovl.s8        q0,  d0
2:
        vld1.8          {d4},  [\src], \s_strd
        vld1.8          {d6},  [\sr2], \s_strd
        vmovl.u8        q2,  d4
        vmovl.u8        q3,  d6
        vext.8          d5,  d4,  d5,  #2
        vext.8          d7,  d6,  d7,  #2
        subs            \h,  \h,  #2
        vtrn.32         d4,  d6
        vtrn.32         d5,  d7
        vmul.s16        d2,  d4,  d0[0]
        vmla.s16        d2,  d5,  d0[1]
        vmla.s16        d2,  d6,  d0[2]
        vmla.s16        d2,  d7,  d0[3]
        vrshr.s16       d2,  d2,  #2
        vqrshrun.s16    d2,  q1,  #4
        vst1.16         {d2[0]}, [\dst, :16], \d_strd
        vst1.16         {d2[1]}, [\ds2, :16], \d_strd
        bgt             2b
        pop             {r4-r11,pc}
.endif

40:     // 4xN h
        add             \mx,  \mx,  #2
        vld1.32         {d0[]}, [\mx]
        sub             \src,  \src,  #1
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
        vmovl.s8        q0,  d0
4:
        vld1.8          {d16}, [\src], \s_strd
        vld1.8          {d24}, [\sr2], \s_strd
        vmovl.u8        q8,  d16
        vmovl.u8        q12, d24
        vext.8          d18, d16, d17, #2
        vext.8          d20, d16, d17, #4
        vext.8          d22, d16, d17, #6
        vext.8          d26, d24, d25, #2
        vext.8          d28, d24, d25, #4
        vext.8          d30, d24, d25, #6
        subs            \h,  \h,  #2
        vmul.s16        d4,  d16, d0[0]
        vmla.s16        d4,  d18, d0[1]
        vmla.s16        d4,  d20, d0[2]
        vmla.s16        d4,  d22, d0[3]
        vmul.s16        d5,  d24, d0[0]
        vmla.s16        d5,  d26, d0[1]
        vmla.s16        d5,  d28, d0[2]
        vmla.s16        d5,  d30, d0[3]
        vrshr.s16       q2,  q2,  #2
.ifc \type, put
        vqrshrun.s16    d4,  q2,  #4
        vst1.32         {d4[0]}, [\dst, :32], \d_strd
        vst1.32         {d4[1]}, [\ds2, :32], \d_strd
.else
        vst1.16         {d4}, [\dst, :64], \d_strd
        vst1.16         {d5}, [\ds2, :64], \d_strd
.endif
        bgt             4b
        pop             {r4-r11,pc}

80:     // 8xN h
        vld1.8          {d0}, [\mx, :64]
        sub             \src,  \src,  #3
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
        vmovl.s8        q0,  d0
8:
        vld1.8          {q8},  [\src], \s_strd
        vld1.8          {q12}, [\sr2], \s_strd
        vmovl.u8        q9,  d17
        vmovl.u8        q8,  d16
        vmovl.u8        q13, d25
        vmovl.u8        q12, d24

        vmul.s16        q10, q8,  d0[0]
        vmul.s16        q14, q12, d0[0]
.irpc i, 1234567
        vext.8          q11, q8,  q9,  #(2*\i)
        vext.8          q15, q12, q13, #(2*\i)
.if \i < 4
        vmla.s16        q10, q11, d0[\i]
        vmla.s16        q14, q15, d0[\i]
.else
        vmla.s16        q10, q11, d1[\i-4]
        vmla.s16        q14, q15, d1[\i-4]
.endif
.endr
        subs            \h,  \h,  #2
        vrshr.s16       q10, q10, #2
        vrshr.s16       q14, q14, #2
.ifc \type, put
        vqrshrun.s16    d20, q10, #4
        vqrshrun.s16    d28, q14, #4
        vst1.8          {d20}, [\dst, :64], \d_strd
        vst1.8          {d28}, [\ds2, :64], \d_strd
.else
        vst1.16         {q10}, [\dst, :128], \d_strd
        vst1.16         {q14}, [\ds2, :128], \d_strd
.endif
        bgt             8b
        pop             {r4-r11,pc}

160:
320:
640:
1280:   // 16xN, 32xN, ... h
        // This could be done without touching q4-q6, by using only
        // one temporary for vext in the loop. That's slower on A7 and A53,
        // (but surprisingly, marginally faster on A8 and A73).
        vpush           {q4-q6}
        vld1.8          {d0}, [\mx, :64]
        sub             \src,  \src,  #3
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        vmovl.s8        q0,  d0

        sub             \s_strd,  \s_strd,  \w
        sub             \s_strd,  \s_strd,  #8
.ifc \type, put
        lsl             \d_strd,  \d_strd,  #1
        sub             \d_strd,  \d_strd,  \w
.endif
161:
        vld1.8          {d16, d17, d18},  [\src]!
        vld1.8          {d24, d25, d26},  [\sr2]!
        mov             \mx, \w
        vmovl.u8        q10, d18
        vmovl.u8        q9,  d17
        vmovl.u8        q8,  d16
        vmovl.u8        q14, d26
        vmovl.u8        q13, d25
        vmovl.u8        q12, d24

16:
        vmul.s16        q1,  q8,  d0[0]
        vmul.s16        q2,  q9,  d0[0]
        vmul.s16        q3,  q12, d0[0]
        vmul.s16        q4,  q13, d0[0]
.irpc i, 1234567
        vext.8          q5,  q8,  q9,  #(2*\i)
        vext.8          q6,  q9,  q10, #(2*\i)
        vext.8          q11, q12, q13, #(2*\i)
        vext.8          q15, q13, q14, #(2*\i)
.if \i < 4
        vmla.s16        q1,  q5,  d0[\i]
        vmla.s16        q2,  q6,  d0[\i]
        vmla.s16        q3,  q11, d0[\i]
        vmla.s16        q4,  q15, d0[\i]
.else
        vmla.s16        q1,  q5,  d1[\i-4]
        vmla.s16        q2,  q6,  d1[\i-4]
        vmla.s16        q3,  q11, d1[\i-4]
        vmla.s16        q4,  q15, d1[\i-4]
.endif
.endr
        vrshr.s16       q1,  q1,  #2
        vrshr.s16       q2,  q2,  #2
        vrshr.s16       q3,  q3,  #2
        vrshr.s16       q4,  q4,  #2
        subs            \mx, \mx, #16
.ifc \type, put
        vqrshrun.s16    d2,  q1,  #4
        vqrshrun.s16    d3,  q2,  #4
        vqrshrun.s16    d4,  q3,  #4
        vqrshrun.s16    d5,  q4,  #4
        vst1.8          {q1}, [\dst, :128]!
        vst1.8          {q2}, [\ds2, :128]!
.else
        vst1.16         {q1, q2}, [\dst, :128]!
        vst1.16         {q3, q4}, [\ds2, :128]!
.endif
        ble             9f

        vmov            q8,  q10
        vmov            q12, q14
        vld1.8          {d18, d19}, [\src]!
        vld1.8          {d26, d27}, [\sr2]!
        vmovl.u8        q10, d19
        vmovl.u8        q9,  d18
        vmovl.u8        q14, d27
        vmovl.u8        q13, d26
        b               16b

9:
        add             \dst,  \dst,  \d_strd
        add             \ds2,  \ds2,  \d_strd
        add             \src,  \src,  \s_strd
        add             \sr2,  \sr2,  \s_strd

        subs            \h,  \h,  #2
        bgt             161b
        vpop            {q4-q6}
        pop             {r4-r11,pc}

L(\type\()_8tap_v):
        cmp             \h,  #4
        ubfx            r9,  \my, #7, #7
        and             \my, \my, #0x7f
        it              gt
        movgt           \my, r9
        add             \my, r10, \my, lsl #3

        adr             r9,  L(\type\()_8tap_v_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(\type\()_8tap_v_tbl):
        .word 1280f - L(\type\()_8tap_v_tbl) + CONFIG_THUMB
        .word 640f  - L(\type\()_8tap_v_tbl) + CONFIG_THUMB
        .word 320f  - L(\type\()_8tap_v_tbl) + CONFIG_THUMB
        .word 160f  - L(\type\()_8tap_v_tbl) + CONFIG_THUMB
        .word 80f   - L(\type\()_8tap_v_tbl) + CONFIG_THUMB
        .word 40f   - L(\type\()_8tap_v_tbl) + CONFIG_THUMB
        .word 20f   - L(\type\()_8tap_v_tbl) + CONFIG_THUMB

20:     // 2xN v
.ifc \type, put
        bgt             28f

        cmp             \h,  #2
        add             \my, \my, #2
        vld1.32         {d0[]}, [\my]
        sub             \src,  \src,  \s_strd
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1
        vmovl.s8        q0,  d0

        // 2x2 v
        load_16         \src, \sr2, \s_strd, d1, d2, d3, d4, d5
        interleave_1_16 d1, d2, d3, d4, d5
        bgt             24f
        vmovl_u8        q8, d1, q9, d2, q10, d3, q11, d4
        mul_mla_4       d6, d16, d18, d20, d22
        vqrshrun_s16    6,   q3,  d6
        st_16           \d_strd, d6, 2
        pop             {r4-r11,pc}

24:     // 2x4 v
        load_16         \sr2, \src, \s_strd, d6, d7
        interleave_1_16 d5, d6, d7
        vmovl_u8        q8, d1, q9, d2, q10, d3, q11, d4, q12, d5, q13, d6
        vmov            d17, d20
        vmov            d19, d22
        vmov            d21, d24
        vmov            d23, d26
        mul_mla_4       q3, q8, q9, q10, q11
        vqrshrun_s16    6,   q3,  d6
        st_16           \d_strd, d6, 4
        pop             {r4-r11,pc}

28:     // 2x6, 2x8, 2x12, 2x16 v
        vpush           {q4-q7}
        vld1.8          {d0}, [\my, :64]
        sub             \sr2,  \src,  \s_strd, lsl #1
        add             \ds2,  \dst,  \d_strd
        sub             \src,  \sr2,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
        vmovl.s8        q0,  d0

        load_16         \src, \sr2, \s_strd, d2,  d4,  d6,  d8,  d10, d12, d14
        interleave_1_16 d2,  d4,  d6,  d8,  d10
        interleave_1_16 d10, d12, d14
        vmovl_u8        q1,  d2,  q2,  d4,  q3,  d6,  q4,  d8,  q5,  d10, q6,  d12
        vmov            d3,  d6
        vmov            d5,  d8
        vmov            d7,  d10
        vmov            d9,  d12
216:
        subs            \h,  \h,  #4
        load_16         \sr2, \src, \s_strd, d16, d18, d20, d22
        interleave_1_16 d14, d16, d18, d20, d22
        vmovl_u8        q7,  d14, q8,  d16, q9,  d18, q10, d20
        vmov            d11, d14
        vmov            d13, d16
        vmov            d15, d18
        vmov            d17, d20
        mul_mla_8_0     q1,  q1,  q2,  q3,  q4,  q5,  q6,  q7, q8
        vqrshrun_s16    6,   q1,  d2
        st_16           \d_strd, d2, 4
        ble             0f
        cmp             \h,  #2
        vmov            q1,  q5
        vmov            q2,  q6
        vmov            q3,  q7
        vmov            q4,  q8
        vmov            q5,  q9
        vmov            q6,  q10
        vmov            d14, d22
        beq             26f
        b               216b
26:
        load_16         \sr2, \src, \s_strd, d16, d18
        interleave_1_16 d14, d16, d18
        vmovl_u8        q7,  d14, q8,  d16
        vmov            d11, d14
        vmov            d13, d16
        mul_mla_8_0     d2,  d2,  d4,  d6,  d8,  d10, d12, d14, d16
        vqrshrun_s16    6,   q1,  d2
        st_16           \d_strd, d2, 2
0:
        vpop            {q4-q7}
        pop             {r4-r11,pc}
.endif

40:
        bgt            480f

        // 4x2, 4x4 v
        cmp             \h,  #2
        add             \my, \my, #2
        vld1.32         {d0[]}, [\my]
        sub             \src, \src, \s_strd
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0

        load_32         \src, \sr2, \s_strd, d1, d2, d3, d4, d5
        interleave_1_32 d1,  d2,  d3,  d4,  d5
        vmovl_u8        q8,  d1,  q9,  d2,  q10, d3,  q11, d4
        mul_mla_4       q3,  q8,  q9,  q10, q11
        shift_store_4   \type, \d_strd, q3, d6, d7
        ble             0f
        load_32         \sr2, \src, \s_strd, d6, d7
        interleave_1_32 d5,  d6,  d7
        vmovl_u8        q12, d5,  q13, d6
        mul_mla_4       q3,  q10, q11, q12, q13
        shift_store_4   \type, \d_strd, q3, d6, d7
0:
        pop             {r4-r11,pc}

480:    // 4x6, 4x8, 4x12, 4x16 v
        vpush           {q4}
        vld1.8          {d0}, [\my, :64]
        sub             \sr2, \src, \s_strd, lsl #1
        add             \ds2, \dst, \d_strd
        sub             \src, \sr2, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0

        load_32         \src, \sr2, \s_strd, d2,  d4,  d6,  d8,  d16, d18, d20
        interleave_1_32 d2,  d4,  d6
        interleave_1_32 d6,  d8,  d16, d18, d20
        vmovl_u8        q1,  d2,  q2,  d4,  q3,  d6,  q4,  d8,  q8,  d16, q9,  d18

48:
        subs            \h,  \h,  #4
        load_32         \sr2, \src, \s_strd, d22, d24, d26, d28
        interleave_1_32 d20, d22, d24, d26, d28
        vmovl_u8        q10, d20, q11, d22, q12, d24, q13, d26
        mul_mla_8_2     q1,  q2,  q1,  q2,  q3,  q4,  q8,  q9,  q10, q11, q12, q13
        shift_store_4   \type, \d_strd, q1,  d2,  d3,  q2,  d4,  d5
        ble             0f
        load_32         \sr2,  \src, \s_strd, d30, d2
        subs            \h,  \h,  #2
        interleave_1_32 d28, d30, d2
        vmovl_u8        q14, d28, q15, d30
        mul_mla_8_0     q8,  q8,  q9,  q10, q11, q12, q13, q14, q15
        shift_store_4   \type, \d_strd, q8,  d16, d17
        ble             0f
        load_32         \sr2,  \src, \s_strd, d4,  d6
        subs            \h,  \h,  #2
        interleave_1_32 d2,  d4,  d6
        vmovl_u8        q1,  d2,  q2,  d4
        mul_mla_8_0     q9,  q10, q11, q12, q13, q14, q15, q1,  q2
        shift_store_4   \type, \d_strd, q9,  d18, d19
        ble             0f
        subs            \h,  \h,  #4
        load_32         \sr2, \src, \s_strd, d8,  d16, d18, d20
        interleave_1_32 d6,  d8,  d16, d18, d20
        vmovl_u8        q3,  d6,  q4,  d8,  q8,  d16, q9, d18
        mul_mla_8_2     q12, q13, q12, q13, q14, q15, q1,  q2,  q3,  q4,  q8,  q9
        shift_store_4   \type, \d_strd, q12, d24, d25, q13, d26, d27
        bgt             48b
0:
        vpop            {q4}
        pop             {r4-r11,pc}

80:
        bgt             880f

        // 8x2, 8x4 v
        cmp             \h,  #2
        add             \my, \my, #2
        vld1.32         {d0[]}, [\my]
        sub             \src, \src, \s_strd
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0

        load_reg        \src, \sr2, \s_strd, d1, d2, d3, d4, d5
        vmovl_u8        q8,  d1,  q9,  d2,  q10, d3,  q11, d4,  q12, d5
        mul_mla_4       q1,  q8,  q9,  q10, q11
        mul_mla_4       q2,  q9,  q10, q11, q12
        shift_store_8   \type, \d_strd, q1, d2, q2, d4
        ble             0f
        load_reg        \sr2, \src, \s_strd, d6, d7
        vmovl_u8        q13, d6,  q14, d7
        mul_mla_4       q1,  q10, q11, q12, q13
        mul_mla_4       q2,  q11, q12, q13, q14
        shift_store_8   \type, \d_strd, q1, d2, q2, d4
0:
        pop             {r4-r11,pc}

880:    // 8x6, 8x8, 8x16, 8x32 v
1680:   // 16x8, 16x16, ...
320:    // 32x8, 32x16, ...
640:
1280:
        vpush           {q4}
        vld1.8          {d0}, [\my, :64]
        sub             \src, \src, \s_strd
        sub             \src, \src, \s_strd, lsl #1
        vmovl.s8        q0,  d0
        mov             \my, \h
168:
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        load_reg        \src, \sr2, \s_strd, d2,  d4,  d6,  d8,  d16, d18, d20
        vmovl_u8        q1,  d2,  q2,  d4,  q3,  d6,  q4,  d8,  q8,  d16, q9,  d18, q10, d20

88:
        subs            \h,  \h,  #2
        load_reg        \sr2, \src, \s_strd, d22, d24
        vmovl_u8        q11, d22, q12, d24
        mul_mla_8_1     q1,  q2,  q1,  q2,  q3,  q4,  q8,  q9,  q10,  q11, q12
        shift_store_8   \type, \d_strd, q1,  d2,  q2,  d4
        ble             9f
        subs            \h,  \h,  #2
        load_reg        \sr2, \src, \s_strd, d26, d28
        vmovl_u8        q13, d26, q14, d28
        mul_mla_8_1     q3,  q4,  q3,  q4,  q8,  q9,  q10, q11, q12, q13, q14
        shift_store_8   \type, \d_strd, q3,  d6,  q4,  d8
        ble             9f
        subs            \h,  \h,  #2
        load_reg        \sr2, \src, \s_strd, d30, d2
        vmovl_u8        q15, d30, q1,  d2
        mul_mla_8_1     q8,  q9,  q8,  q9,  q10, q11, q12, q13, q14, q15, q1
        shift_store_8   \type, \d_strd, q8,  d16, q9,  d18
        ble             9f
        subs            \h,  \h,  #2
        load_reg        \sr2, \src, \s_strd, d4,  d6
        vmovl_u8        q2,  d4,  q3,  d6
        mul_mla_8_1     q10, q11, q10, q11, q12, q13, q14, q15, q1,  q2,  q3
        shift_store_8   \type, \d_strd, q10, d20, q11, d22
        ble             9f
        subs            \h,  \h,  #4
        load_reg        \sr2, \src, \s_strd, d8,  d16, d18, d20
        vmovl_u8        q4,  d8,  q8,  d16, q9,  d18, q10, d20
        mul_mla_8_1     q12, q13, q12, q13, q14, q15, q1,  q2,  q3,  q4,  q8
        mul_mla_8_1     q14, q15, q14, q15, q1,  q2,  q3,  q4,  q8,  q9,  q10
        shift_store_8   \type, \d_strd, q12, d24, q13, d26, q14, d28, q15, d30
        bgt             88b
9:
        subs            \w,  \w,  #8
        ble             0f
        asr             \s_strd, \s_strd, #1
        asr             \d_strd, \d_strd, #1
        mls             \src, \s_strd, \my, \src
        mls             \dst, \d_strd, \my, \dst
        sub             \src, \src, \s_strd, lsl #3
        mov             \h,  \my
        add             \src, \src, #8
.ifc \type, put
        add             \dst, \dst, #8
.else
        add             \dst, \dst, #16
.endif
        b               168b
0:
        vpop            {q4}
        pop             {r4-r11,pc}

160:
        bgt             1680b

        // 16x2, 16x4 v
        add             \my, \my, #2
        vld1.32         {d0[]}, [\my]
        sub             \src, \src, \s_strd
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0

        cmp             \h,  #2
        load_reg        \src, \sr2, \s_strd, q11, q12, q13, q14, q15
        vmovl.u8        q1,  d22
        vmovl.u8        q2,  d24
        vmovl.u8        q3,  d26
        vmovl.u8        q8,  d28
        vmovl.u8        q9,  d30
        vmovl.u8        q11, d23
        vmovl.u8        q12, d25
        vmovl.u8        q13, d27
        vmovl.u8        q14, d29
        vmovl.u8        q15, d31
        mul_mla_4       q1,  q1,  q2,  q3,  q8
        mul_mla_4       q10, q2,  q3,  q8,  q9
        mul_mla_4       q2,  q11, q12, q13, q14
        mul_mla_4       q11, q12, q13, q14, q15
        shift_store_16  \type, \d_strd, q1, d2, d3, q2, q10, d20, d21, q11
        ble             0f
        load_reg        \sr2, \src, \s_strd, q10, q11
        vmovl.u8        q1,  d20
        vmovl.u8        q10, d21
        vmovl.u8        q12, d22
        vmovl.u8        q11, d23
        mul_mla_4       q2,  q3,  q8,  q9,  q1
        mul_mla_4       q3,  q13, q14, q15, q10
        mul_mla_4       q13, q8,  q9,  q1,  q12
        mul_mla_4       q14, q14, q15, q10, q11
        shift_store_16  \type, \d_strd, q2, d4, d5, q3, q13, d26, d27, q14
0:
        pop             {r4-r11,pc}

L(\type\()_8tap_hv):
        cmp             \h,  #4
        ubfx            r9,  \my, #7, #7
        and             \my, \my, #0x7f
        it              gt
        movgt           \my, r9
        add             \my,  r10, \my, lsl #3

        adr             r9,  L(\type\()_8tap_hv_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(\type\()_8tap_hv_tbl):
        .word 1280f - L(\type\()_8tap_hv_tbl) + CONFIG_THUMB
        .word 640f  - L(\type\()_8tap_hv_tbl) + CONFIG_THUMB
        .word 320f  - L(\type\()_8tap_hv_tbl) + CONFIG_THUMB
        .word 160f  - L(\type\()_8tap_hv_tbl) + CONFIG_THUMB
        .word 80f   - L(\type\()_8tap_hv_tbl) + CONFIG_THUMB
        .word 40f   - L(\type\()_8tap_hv_tbl) + CONFIG_THUMB
        .word 20f   - L(\type\()_8tap_hv_tbl) + CONFIG_THUMB

20:
.ifc \type, put
        add             \mx,  \mx,  #2
        vld1.32         {d0[]},  [\mx]
        bgt             280f
        add             \my,  \my,  #2
        vld1.32         {d2[]},  [\my]

        // 2x2, 2x4 hv
        sub             \sr2, \src, #1
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0
        vmovl.s8        q1,  d2


        vld1.8          {d26}, [\src], \s_strd
        vmovl.u8        q13, d26
        vext.8          q14, q13, q13, #2
        vmul.s16        d26, d26, d0
        vmul.s16        d28, d28, d0
        vpadd.s16       d26, d26, d28
        vpadd.s16       d26, d26, d26
        vrshr.s16       d16, d26, #2
        bl              L(\type\()_8tap_filter_2)

        vext.8          d16, d16, d16, #4
        vmov            d17, d26
        vext.8          d16, d16, d26, #4

2:
        bl              L(\type\()_8tap_filter_2)

        vext.8          d18, d17, d26, #4
        vmull.s16       q2,  d16, d2[0]
        vmlal.s16       q2,  d17, d2[1]
        vmlal.s16       q2,  d18, d2[2]
        vmlal.s16       q2,  d26, d2[3]

        vqrshrn.s32     d4,  q2,  #\shift_hv
        vqmovun.s16     d4,  q2
        subs            \h,  \h,  #2
        vst1.16         {d4[0]}, [\dst, :16], \d_strd
        vst1.16         {d4[1]}, [\ds2, :16], \d_strd
        ble             0f
        vmov            d16, d18
        vmov            d17, d26
        b               2b

280:    // 2x8, 2x16, 2x32 hv
        vld1.8          {d2},  [\my, :64]
        sub             \src, \src, #1
        sub             \sr2, \src, \s_strd, lsl #1
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0
        vmovl.s8        q1,  d2

        vld1.8          {d26}, [\src], \s_strd
        vmovl.u8        q13, d26
        vext.8          q14, q13, q13, #2
        vmul.s16        d26, d26, d0
        vmul.s16        d28, d28, d0
        vpadd.s16       d26, d26, d28
        vpadd.s16       d26, d26, d26
        vrshr.s16       d16, d26, #2

        bl              L(\type\()_8tap_filter_2)
        vext.8          d16, d16, d16, #4
        vmov            d17, d26
        vext.8          d16, d16, d26, #4
        bl              L(\type\()_8tap_filter_2)
        vext.8          d18, d17, d26, #4
        vmov            d19, d26
        bl              L(\type\()_8tap_filter_2)
        vext.8          d20, d19, d26, #4
        vmov            d21, d26

28:
        bl              L(\type\()_8tap_filter_2)
        vext.8          d22, d21, d26, #4
        vmull.s16       q2,  d16, d2[0]
        vmlal.s16       q2,  d17, d2[1]
        vmlal.s16       q2,  d18, d2[2]
        vmlal.s16       q2,  d19, d2[3]
        vmlal.s16       q2,  d20, d3[0]
        vmlal.s16       q2,  d21, d3[1]
        vmlal.s16       q2,  d22, d3[2]
        vmlal.s16       q2,  d26, d3[3]

        vqrshrn.s32     d4,  q2,  #\shift_hv
        vqmovun.s16     d4,  q2
        subs            \h,  \h,  #2
        vst1.16         {d4[0]}, [\dst, :16], \d_strd
        vst1.16         {d4[1]}, [\ds2, :16], \d_strd
        ble             0f
        vmov            d16, d18
        vmov            d17, d19
        vmov            d18, d20
        vmov            d19, d21
        vmov            d20, d22
        vmov            d21, d26
        b               28b

0:
        pop             {r4-r11,pc}

L(\type\()_8tap_filter_2):
        vld1.8          {d28},  [\sr2], \s_strd
        vld1.8          {d30},  [\src], \s_strd
        vext.8          d29, d28, d28, #1
        vext.8          d31, d30, d30, #1
        vmovl.u8        q13, d28
        vmovl.u8        q14, d29
        vmov            d27, d28
        vmovl.u8        q14, d30
        vmovl.u8        q15, d31
        vtrn.32         d26, d28
        vtrn.32         d27, d30
        vmul.s16        d26, d26, d0[0]
        vmla.s16        d26, d27, d0[1]
        vmla.s16        d26, d28, d0[2]
        vmla.s16        d26, d30, d0[3]
        vrshr.s16       d26, d26, #2
        vext.8          d27, d26, d26, #4
        bx              lr
.endif

40:
        add             \mx, \mx, #2
        vld1.32         {d0[]},  [\mx]
        bgt             480f
        add             \my, \my,  #2
        vld1.32         {d2[]},  [\my]
        sub             \sr2, \src, #1
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0
        vmovl.s8        q1,  d2

        // 4x2, 4x4 hv
        vld1.8          {d30}, [\src], \s_strd
        vmovl.u8        q14, d30
        vext.8          d27, d28, d29, #2
        vext.8          d30, d28, d29, #4
        vext.8          d31, d28, d29, #6
        vmul.s16        d26, d28, d0[0]
        vmla.s16        d26, d27, d0[1]
        vmla.s16        d26, d30, d0[2]
        vmla.s16        d26, d31, d0[3]
        vrshr.s16       d16, d26, #2

        bl              L(\type\()_8tap_filter_4)
        vmov            d17, d26
        vmov            d18, d27

4:
        bl              L(\type\()_8tap_filter_4)
        vmull.s16       q2,  d16, d2[0]
        vmlal.s16       q2,  d17, d2[1]
        vmlal.s16       q2,  d18, d2[2]
        vmlal.s16       q2,  d26, d2[3]
        vmull.s16       q3,  d17, d2[0]
        vmlal.s16       q3,  d18, d2[1]
        vmlal.s16       q3,  d26, d2[2]
        vmlal.s16       q3,  d27, d2[3]
        vqrshrn.s32     d4,  q2,  #\shift_hv
        vqrshrn.s32     d6,  q3,  #\shift_hv
        subs            \h,  \h,  #2
.ifc \type, put
        vqmovun.s16     d4,  q2
        vqmovun.s16     d6,  q3
        vst1.32         {d4[0]}, [\dst, :32], \d_strd
        vst1.32         {d6[0]}, [\ds2, :32], \d_strd
.else
        vst1.16         {d4}, [\dst, :64], \d_strd
        vst1.16         {d6}, [\ds2, :64], \d_strd
.endif
        ble             0f
        vmov            d16, d18
        vmov            d17, d26
        vmov            d18, d27
        b               4b

480:    // 4x8, 4x16, 4x32 hv
        vld1.8          {d2},  [\my, :64]
        sub             \src, \src, #1
        sub             \sr2, \src, \s_strd, lsl #1
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        vmovl.s8        q0,  d0
        vmovl.s8        q1,  d2

        vld1.8          {d30}, [\src], \s_strd
        vmovl.u8        q14, d30
        vext.8          d27, d28, d29, #2
        vext.8          d30, d28, d29, #4
        vext.8          d31, d28, d29, #6
        vmul.s16        d26, d28, d0[0]
        vmla.s16        d26, d27, d0[1]
        vmla.s16        d26, d30, d0[2]
        vmla.s16        d26, d31, d0[3]
        vrshr.s16       d16, d26, #2

        bl              L(\type\()_8tap_filter_4)
        vmov            d17, d26
        vmov            d18, d27
        bl              L(\type\()_8tap_filter_4)
        vmov            d19, d26
        vmov            d20, d27
        bl              L(\type\()_8tap_filter_4)
        vmov            d21, d26
        vmov            d22, d27

48:
        bl              L(\type\()_8tap_filter_4)
        vmull.s16       q2,  d16, d2[0]
        vmlal.s16       q2,  d17, d2[1]
        vmlal.s16       q2,  d18, d2[2]
        vmlal.s16       q2,  d19, d2[3]
        vmlal.s16       q2,  d20, d3[0]
        vmlal.s16       q2,  d21, d3[1]
        vmlal.s16       q2,  d22, d3[2]
        vmlal.s16       q2,  d26, d3[3]
        vmull.s16       q3,  d17, d2[0]
        vmlal.s16       q3,  d18, d2[1]
        vmlal.s16       q3,  d19, d2[2]
        vmlal.s16       q3,  d20, d2[3]
        vmlal.s16       q3,  d21, d3[0]
        vmlal.s16       q3,  d22, d3[1]
        vmlal.s16       q3,  d26, d3[2]
        vmlal.s16       q3,  d27, d3[3]
        vqrshrn.s32     d4,  q2,  #\shift_hv
        vqrshrn.s32     d6,  q3,  #\shift_hv
        subs            \h,  \h,  #2
.ifc \type, put
        vqmovun.s16     d4,  q2
        vqmovun.s16     d6,  q3
        vst1.32         {d4[0]}, [\dst, :32], \d_strd
        vst1.32         {d6[0]}, [\ds2, :32], \d_strd
.else
        vst1.16         {d4}, [\dst, :64], \d_strd
        vst1.16         {d6}, [\ds2, :64], \d_strd
.endif
        ble             0f
        vmov            d16, d18
        vmov            d17, d19
        vmov            d18, d20
        vmov            d19, d21
        vmov            d20, d22
        vmov            d21, d26
        vmov            d22, d27
        b               48b
0:
        pop             {r4-r11,pc}

L(\type\()_8tap_filter_4):
        vld1.8          {d30}, [\sr2], \s_strd
        vld1.8          {d31}, [\src], \s_strd
        vmovl.u8        q14, d30
        vext.8          d27, d28, d29, #2
        vext.8          d30, d28, d29, #4
        vext.8          d1,  d28, d29, #6
        vmul.s16        d26, d28, d0[0]
        vmla.s16        d26, d27, d0[1]
        vmla.s16        d26, d30, d0[2]
        vmla.s16        d26, d1,  d0[3]

        vmovl.u8        q14, d31
        vext.8          d30, d28, d29, #2
        vext.8          d31, d28, d29, #4
        vext.8          d1,  d28, d29, #6
        vmul.s16        d27, d28, d0[0]
        vmla.s16        d27, d30, d0[1]
        vmla.s16        d27, d31, d0[2]
        vmla.s16        d27, d1,  d0[3]
        vrshr.s16       d26, d26, #2
        vrshr.s16       d27, d27, #2
        bx              lr

80:
160:
320:
        bgt             880f
        vpush           {q4-q7}
        add             \my,  \my,  #2
        vld1.8          {d0},  [\mx, :64]
        vld1.32         {d2[]},  [\my]
        sub             \src,  \src,  #3
        sub             \src,  \src,  \s_strd
        vmovl.s8        q0,  d0
        vmovl.s8        q1,  d2
        mov             \my, \h

164:    // 8x2, 8x4, 16x2, 16x4, 32x2, 32x4 hv
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd, \d_strd, #1
        lsl             \s_strd, \s_strd, #1

        vld1.8          {q14},  [\src], \s_strd
        vmovl.u8        q12, d28
        vmovl.u8        q13, d29
        vmul.s16        q10, q12, d0[0]
.irpc i, 123
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q10, q14, d0[\i]
.endr
.irpc i, 4567
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q10, q14, d1[\i-4]
.endr
        vrshr.s16       q3,  q10, #2

        bl              L(\type\()_8tap_filter_8)
        vmov            q4,  q10
        vmov            q5,  q11

8:
        bl              L(\type\()_8tap_filter_8)
        vmull.s16       q12, d6,  d2[0]
        vmull.s16       q13, d7,  d2[0]
        vmull.s16       q14, d8,  d2[0]
        vmull.s16       q15, d9,  d2[0]
        vmlal.s16       q12, d8,  d2[1]
        vmlal.s16       q13, d9,  d2[1]
        vmlal.s16       q14, d10, d2[1]
        vmlal.s16       q15, d11, d2[1]
        vmlal.s16       q12, d10, d2[2]
        vmlal.s16       q13, d11, d2[2]
        vmlal.s16       q14, d20, d2[2]
        vmlal.s16       q15, d21, d2[2]
        vmlal.s16       q12, d20, d2[3]
        vmlal.s16       q13, d21, d2[3]
        vmlal.s16       q14, d22, d2[3]
        vmlal.s16       q15, d23, d2[3]
        vqrshrn.s32     d24, q12, #\shift_hv
        vqrshrn.s32     d25, q13, #\shift_hv
        vqrshrn.s32     d28, q14, #\shift_hv
        vqrshrn.s32     d29, q15, #\shift_hv
        subs            \h,  \h,  #2
.ifc \type, put
        vqmovun.s16     d24, q12
        vqmovun.s16     d28, q14
        vst1.8          {d24}, [\dst, :64], \d_strd
        vst1.8          {d28}, [\ds2, :64], \d_strd
.else
        vst1.16         {q12}, [\dst, :128], \d_strd
        vst1.16         {q14}, [\ds2, :128], \d_strd
.endif
        ble             9f
        vmov            q3,  q5
        vmov            q4,  q10
        vmov            q5,  q11
        b               8b
9:
        subs            \w,  \w,  #8
        ble             0f
        asr             \s_strd,  \s_strd,  #1
        asr             \d_strd,  \d_strd,  #1
        mls             \src,  \s_strd,  \my,  \src
        mls             \dst,  \d_strd,  \my,  \dst
        sub             \src,  \src,  \s_strd,  lsl #2
        mov             \h,  \my
        add             \src,  \src,  #8
.ifc \type, put
        add             \dst,  \dst,  #8
.else
        add             \dst,  \dst,  #16
.endif
        b               164b

880:    // 8x8, 8x16, ..., 16x8, ..., 32x8, ... hv
640:
1280:
        vpush           {q4-q7}
        vld1.8          {d0},  [\mx, :64]
        vld1.8          {d2},  [\my, :64]
        sub             \src,  \src,  #3
        sub             \src,  \src,  \s_strd
        sub             \src,  \src,  \s_strd, lsl #1
        vmovl.s8        q0,  d0
        vmovl.s8        q1,  d2
        mov             \my, \h

168:
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd, \d_strd, #1
        lsl             \s_strd, \s_strd, #1

        vld1.8          {q14},  [\src], \s_strd
        vmovl.u8        q12, d28
        vmovl.u8        q13, d29
        vmul.s16        q10, q12, d0[0]
.irpc i, 123
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q10, q14, d0[\i]
.endr
.irpc i, 4567
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q10, q14, d1[\i-4]
.endr
        vrshr.s16       q3,  q10, #2

        bl              L(\type\()_8tap_filter_8)
        vmov            q4,  q10
        vmov            q5,  q11
        bl              L(\type\()_8tap_filter_8)
        vmov            q6,  q10
        vmov            q7,  q11
        bl              L(\type\()_8tap_filter_8)
        vmov            q8,  q10
        vmov            q9,  q11

88:
        bl              L(\type\()_8tap_filter_8)
        vmull.s16       q12, d6,  d2[0]
        vmull.s16       q13, d7,  d2[0]
        vmull.s16       q14, d8,  d2[0]
        vmull.s16       q15, d9,  d2[0]
        vmlal.s16       q12, d8,  d2[1]
        vmlal.s16       q13, d9,  d2[1]
        vmlal.s16       q14, d10, d2[1]
        vmlal.s16       q15, d11, d2[1]
        vmlal.s16       q12, d10, d2[2]
        vmlal.s16       q13, d11, d2[2]
        vmlal.s16       q14, d12, d2[2]
        vmlal.s16       q15, d13, d2[2]
        vmlal.s16       q12, d12, d2[3]
        vmlal.s16       q13, d13, d2[3]
        vmlal.s16       q14, d14, d2[3]
        vmlal.s16       q15, d15, d2[3]
        vmlal.s16       q12, d14, d3[0]
        vmlal.s16       q13, d15, d3[0]
        vmlal.s16       q14, d16, d3[0]
        vmlal.s16       q15, d17, d3[0]
        vmlal.s16       q12, d16, d3[1]
        vmlal.s16       q13, d17, d3[1]
        vmlal.s16       q14, d18, d3[1]
        vmlal.s16       q15, d19, d3[1]
        vmlal.s16       q12, d18, d3[2]
        vmlal.s16       q13, d19, d3[2]
        vmlal.s16       q14, d20, d3[2]
        vmlal.s16       q15, d21, d3[2]
        vmlal.s16       q12, d20, d3[3]
        vmlal.s16       q13, d21, d3[3]
        vmlal.s16       q14, d22, d3[3]
        vmlal.s16       q15, d23, d3[3]
        vqrshrn.s32     d24, q12, #\shift_hv
        vqrshrn.s32     d25, q13, #\shift_hv
        vqrshrn.s32     d28, q14, #\shift_hv
        vqrshrn.s32     d29, q15, #\shift_hv
        subs            \h,  \h,  #2
.ifc \type, put
        vqmovun.s16     d24, q12
        vqmovun.s16     d28, q14
        vst1.8          {d24}, [\dst, :64], \d_strd
        vst1.8          {d28}, [\ds2, :64], \d_strd
.else
        vst1.16         {q12}, [\dst, :128], \d_strd
        vst1.16         {q14}, [\ds2, :128], \d_strd
.endif
        ble             9f
        vmov            q3,  q5
        vmov            q4,  q6
        vmov            q5,  q7
        vmov            q6,  q8
        vmov            q7,  q9
        vmov            q8,  q10
        vmov            q9,  q11
        b               88b
9:
        subs            \w,  \w,  #8
        ble             0f
        asr             \s_strd,  \s_strd,  #1
        asr             \d_strd,  \d_strd,  #1
        mls             \src,  \s_strd,  \my,  \src
        mls             \dst,  \d_strd,  \my,  \dst
        sub             \src,  \src,  \s_strd,  lsl #3
        mov             \h,  \my
        add             \src,  \src,  #8
.ifc \type, put
        add             \dst,  \dst,  #8
.else
        add             \dst,  \dst,  #16
.endif
        b               168b
0:
        vpop            {q4-q7}
        pop             {r4-r11,pc}

L(\type\()_8tap_filter_8):
        vld1.8          {q14},  [\sr2], \s_strd
        vld1.8          {q15},  [\src], \s_strd
        vmovl.u8        q12, d28
        vmovl.u8        q13, d29
        vmul.s16        q10, q12, d0[0]
.irpc i, 123
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q10, q14, d0[\i]
.endr
.irpc i, 4567
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q10, q14, d1[\i-4]
.endr
        vmovl.u8        q12, d30
        vmovl.u8        q13, d31
        vmul.s16        q11, q12, d0[0]
.irpc i, 123
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q11, q14, d0[\i]
.endr
.irpc i, 4567
        vext.8          q14, q12, q13, #(2*\i)
        vmla.s16        q11, q14, d1[\i-4]
.endr
        vrshr.s16       q10, q10, #2
        vrshr.s16       q11, q11, #2
        bx              lr
endfunc


function \type\()_bilin_8bpc_neon, export=1
        push            {r4-r11,lr}
        ldrd            r4,  r5,  [sp, #36]
        ldrd            r6,  r7,  [sp, #44]
        vdup.8          d1,  \mx
        vdup.8          d3,  \my
        rsb             r8,  \mx, #16
        rsb             r9,  \my, #16
        vdup.8          d0,  r8
        vdup.8          d2,  r9
.ifc \type, prep
        lsl             \d_strd, \w, #1
.endif
        clz             r8,  \w
        cmp             \mx, #0
        sub             r8,  r8,  #24
        bne             L(\type\()_bilin_h)
        cmp             \my, #0
        bne             L(\type\()_bilin_v)
        b               \type\()_neon

L(\type\()_bilin_h):
        cmp             \my, #0
        bne             L(\type\()_bilin_hv)

        adr             r9,  L(\type\()_bilin_h_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(\type\()_bilin_h_tbl):
        .word 1280f - L(\type\()_bilin_h_tbl) + CONFIG_THUMB
        .word 640f  - L(\type\()_bilin_h_tbl) + CONFIG_THUMB
        .word 320f  - L(\type\()_bilin_h_tbl) + CONFIG_THUMB
        .word 160f  - L(\type\()_bilin_h_tbl) + CONFIG_THUMB
        .word 80f   - L(\type\()_bilin_h_tbl) + CONFIG_THUMB
        .word 40f   - L(\type\()_bilin_h_tbl) + CONFIG_THUMB
        .word 20f   - L(\type\()_bilin_h_tbl) + CONFIG_THUMB

20:     // 2xN h
.ifc \type, put
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
2:
        vld1.32         {d4[]},  [\src], \s_strd
        vld1.32         {d6[]},  [\sr2], \s_strd
        vext.8          d5,  d4,  d4, #1
        vext.8          d7,  d6,  d6, #1
        vtrn.16         q2,  q3
        subs            \h,  \h,  #2
        vmull.u8        q3,  d4,  d0
        vmlal.u8        q3,  d5,  d1
        vqrshrn.u16     d4,  q3,  #4
        vst1.16         {d4[0]}, [\dst, :16], \d_strd
        vst1.16         {d4[1]}, [\ds2, :16], \d_strd
        bgt             2b
        pop             {r4-r11,pc}
.endif

40:     // 4xN h
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
4:
        vld1.8          {d4}, [\src], \s_strd
        vld1.8          {d6}, [\sr2], \s_strd
        vext.8          d5,  d4,  d4, #1
        vext.8          d7,  d6,  d6, #1
        vtrn.32         q2,  q3
        subs            \h,  \h,  #2
        vmull.u8        q3,  d4,  d0
        vmlal.u8        q3,  d5,  d1
.ifc \type, put
        vqrshrn.u16     d4,  q3,  #4
        vst1.32         {d4[0]}, [\dst, :32], \d_strd
        vst1.32         {d4[1]}, [\ds2, :32], \d_strd
.else
        vst1.16         {d6}, [\dst, :64], \d_strd
        vst1.16         {d7}, [\ds2, :64], \d_strd
.endif
        bgt             4b
        pop             {r4-r11,pc}

80:     // 8xN h
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
8:
        vld1.8          {q8},  [\src], \s_strd
        vld1.8          {q10}, [\sr2], \s_strd
        vext.8          q9,  q8,  q8,  #1
        vext.8          q11, q10, q10, #1
        subs            \h,  \h,  #2
        vmull.u8        q8,  d16, d0
        vmull.u8        q10, d20, d0
        vmlal.u8        q8,  d18, d1
        vmlal.u8        q10, d22, d1
.ifc \type, put
        vqrshrn.u16     d16,  q8,  #4
        vqrshrn.u16     d18,  q10, #4
        vst1.8          {d16}, [\dst, :64], \d_strd
        vst1.8          {d18}, [\ds2, :64], \d_strd
.else
        vst1.16         {q8},  [\dst, :128], \d_strd
        vst1.16         {q10}, [\ds2, :128], \d_strd
.endif
        bgt             8b
        pop             {r4-r11,pc}
160:
320:
640:
1280:   // 16xN, 32xN, ... h
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1

        sub             \s_strd,  \s_strd,  \w
        sub             \s_strd,  \s_strd,  #8
.ifc \type, put
        lsl             \d_strd,  \d_strd,  #1
        sub             \d_strd,  \d_strd,  \w
.endif
161:
        vld1.8          {d16},  [\src]!
        vld1.8          {d22},  [\sr2]!
        mov             \mx, \w

16:
        vld1.8          {d17,d18},  [\src]!
        vld1.8          {d23,d24},  [\sr2]!
        vext.8          q10, q8,  q9,  #1
        vext.8          q13, q11, q12, #1
        vmull.u8        q2,  d16, d0
        vmull.u8        q3,  d17, d0
        vmull.u8        q14, d22, d0
        vmull.u8        q15, d23, d0
        vmlal.u8        q2,  d20, d1
        vmlal.u8        q3,  d21, d1
        vmlal.u8        q14, d26, d1
        vmlal.u8        q15, d27, d1
        subs            \mx, \mx, #16
.ifc \type, put
        vqrshrn.u16     d4,  q2,  #4
        vqrshrn.u16     d5,  q3,  #4
        vqrshrn.u16     d28, q14, #4
        vqrshrn.u16     d29, q15, #4
        vst1.8          {q2},  [\dst, :128]!
        vst1.8          {q14}, [\ds2, :128]!
.else
        vst1.16         {q2,  q3},  [\dst, :128]!
        vst1.16         {q14, q15}, [\ds2, :128]!
.endif
        ble             9f

        vmov            d16, d18
        vmov            d22, d24
        b               16b

9:
        add             \dst,  \dst,  \d_strd
        add             \ds2,  \ds2,  \d_strd
        add             \src,  \src,  \s_strd
        add             \sr2,  \sr2,  \s_strd

        subs            \h,  \h,  #2
        bgt             161b
        pop             {r4-r11,pc}

L(\type\()_bilin_v):
        cmp             \h,  #4
        adr             r9,  L(\type\()_bilin_v_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(\type\()_bilin_v_tbl):
        .word 1280f - L(\type\()_bilin_v_tbl) + CONFIG_THUMB
        .word 640f  - L(\type\()_bilin_v_tbl) + CONFIG_THUMB
        .word 320f  - L(\type\()_bilin_v_tbl) + CONFIG_THUMB
        .word 160f  - L(\type\()_bilin_v_tbl) + CONFIG_THUMB
        .word 80f   - L(\type\()_bilin_v_tbl) + CONFIG_THUMB
        .word 40f   - L(\type\()_bilin_v_tbl) + CONFIG_THUMB
        .word 20f   - L(\type\()_bilin_v_tbl) + CONFIG_THUMB

20:     // 2xN v
.ifc \type, put
        cmp             \h,  #2
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1

        // 2x2 v
        vld1.16         {d16[]}, [\src], \s_strd
        bgt             24f
22:
        vld1.16         {d17[]}, [\sr2], \s_strd
        vld1.16         {d18[]}, [\src], \s_strd
        vext.8          d16, d16, d17, #6
        vext.8          d17, d17, d18, #6
        vmull.u8        q2,  d16, d2
        vmlal.u8        q2,  d17, d3
        vqrshrn.u16     d4,  q2,  #4
        vst1.16         {d4[0]}, [\dst, :16]
        vst1.16         {d4[1]}, [\ds2, :16]
        pop             {r4-r11,pc}
24:     // 2x4, 2x6, 2x8, ... v
        vld1.16         {d17[]}, [\sr2], \s_strd
        vld1.16         {d18[]}, [\src], \s_strd
        vld1.16         {d19[]}, [\sr2], \s_strd
        vld1.16         {d20[]}, [\src], \s_strd
        sub             \h,  \h,  #4
        vext.8          d16, d16, d17, #6
        vext.8          d17, d17, d18, #6
        vext.8          d18, d18, d19, #6
        vext.8          d19, d19, d20, #6
        vtrn.32         d16, d18
        vtrn.32         d17, d19
        vmull.u8        q2,  d16, d2
        vmlal.u8        q2,  d17, d3
        cmp             \h,  #2
        vqrshrn.u16     d4,  q2,  #4
        vst1.16         {d4[0]}, [\dst, :16], \d_strd
        vst1.16         {d4[1]}, [\ds2, :16], \d_strd
        vst1.16         {d4[2]}, [\dst, :16], \d_strd
        vst1.16         {d4[3]}, [\ds2, :16], \d_strd
        blt             0f
        vmov            d16, d20
        beq             22b
        b               24b
0:
        pop             {r4-r11,pc}
.endif

40:     // 4xN v
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1
        vld1.32         {d16[]}, [\src], \s_strd
4:
        vld1.32         {d17[]}, [\sr2], \s_strd
        vld1.32         {d18[]}, [\src], \s_strd
        vext.8          d16, d16, d17, #4
        vext.8          d17, d17, d18, #4
        vmull.u8        q2,  d16, d2
        vmlal.u8        q2,  d17, d3
        subs            \h,  \h,  #2
.ifc \type, put
        vqrshrn.u16     d4,  q2,  #4
        vst1.32         {d4[0]}, [\dst, :32], \d_strd
        vst1.32         {d4[1]}, [\ds2, :32], \d_strd
.else
        vst1.16         {d4}, [\dst, :64], \d_strd
        vst1.16         {d5}, [\ds2, :64], \d_strd
.endif
        ble             0f
        vmov            d16,  d18
        b               4b
0:
        pop             {r4-r11,pc}

80:     // 8xN v
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1
        vld1.8          {d16}, [\src], \s_strd
8:
        vld1.8          {d17}, [\sr2], \s_strd
        vld1.8          {d18}, [\src], \s_strd
        vmull.u8        q2,  d16, d2
        vmull.u8        q3,  d17, d2
        vmlal.u8        q2,  d17, d3
        vmlal.u8        q3,  d18, d3
        subs            \h,  \h,  #2
.ifc \type, put
        vqrshrn.u16     d4,  q2,  #4
        vqrshrn.u16     d6,  q3,  #4
        vst1.8          {d4}, [\dst, :64], \d_strd
        vst1.8          {d6}, [\ds2, :64], \d_strd
.else
        vst1.16         {q2}, [\dst, :128], \d_strd
        vst1.16         {q3}, [\ds2, :128], \d_strd
.endif
        ble             0f
        vmov            d16, d18
        b               8b
0:
        pop             {r4-r11,pc}

160:    // 16xN, 32xN, ...
320:
640:
1280:
        mov             \my, \h
1:
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        vld1.8          {q8},  [\src], \s_strd
2:
        vld1.8          {q9},  [\sr2], \s_strd
        vld1.8          {q10}, [\src], \s_strd
        vmull.u8        q12, d16, d2
        vmull.u8        q13, d17, d2
        vmull.u8        q14, d18, d2
        vmull.u8        q15, d19, d2
        vmlal.u8        q12, d18, d3
        vmlal.u8        q13, d19, d3
        vmlal.u8        q14, d20, d3
        vmlal.u8        q15, d21, d3
        subs            \h,  \h,  #2
.ifc \type, put
        vqrshrn.u16     d24, q12, #4
        vqrshrn.u16     d25, q13, #4
        vqrshrn.u16     d28, q14, #4
        vqrshrn.u16     d29, q15, #4
        vst1.8          {q12}, [\dst, :128], \d_strd
        vst1.8          {q14}, [\ds2, :128], \d_strd
.else
        vst1.16         {q12, q13}, [\dst, :128], \d_strd
        vst1.16         {q14, q15}, [\ds2, :128], \d_strd
.endif
        ble             9f
        vmov            q8,  q10
        b               2b
9:
        subs            \w,  \w,  #16
        ble             0f
        asr             \s_strd, \s_strd, #1
        asr             \d_strd, \d_strd, #1
        mls             \src, \s_strd, \my, \src
        mls             \dst, \d_strd, \my, \dst
        sub             \src, \src, \s_strd, lsl #1
        mov             \h,  \my
        add             \src, \src, #16
.ifc \type, put
        add             \dst, \dst, #16
.else
        add             \dst, \dst, #32
.endif
        b               1b
0:
        pop             {r4-r11,pc}

L(\type\()_bilin_hv):
        vmovl.u8        q2,  d2
        vmovl.u8        q3,  d3
        adr             r9,  L(\type\()_bilin_hv_tbl)
        ldr             r8,  [r9, r8, lsl #2]
        add             r9,  r9,  r8
        bx              r9

        .align 2
L(\type\()_bilin_hv_tbl):
        .word 1280f - L(\type\()_bilin_hv_tbl) + CONFIG_THUMB
        .word 640f  - L(\type\()_bilin_hv_tbl) + CONFIG_THUMB
        .word 320f  - L(\type\()_bilin_hv_tbl) + CONFIG_THUMB
        .word 160f  - L(\type\()_bilin_hv_tbl) + CONFIG_THUMB
        .word 80f   - L(\type\()_bilin_hv_tbl) + CONFIG_THUMB
        .word 40f   - L(\type\()_bilin_hv_tbl) + CONFIG_THUMB
        .word 20f   - L(\type\()_bilin_hv_tbl) + CONFIG_THUMB

20:     // 2xN hv
.ifc \type, put
        add             \sr2, \src, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        vld1.32         {d28[]},  [\src], \s_strd
        vext.8          d29, d28, d28, #1
        vmull.u8        q8,  d28, d0
        vmlal.u8        q8,  d29, d1

2:
        vld1.32         {d28[]},  [\sr2], \s_strd
        vld1.32         {d30[]},  [\src], \s_strd
        vext.8          d29, d28, d28, #1
        vext.8          d31, d30, d30, #1
        vtrn.16         d28, d30
        vtrn.16         d29, d31
        vmull.u8        q9,  d28, d0
        vmlal.u8        q9,  d29, d1

        vtrn.32         d16, d18

        vmul.u16        d20, d16, d4
        vmla.u16        d20, d19, d6
        vqrshrn.u16     d20, q10, #8
        subs            \h,  \h,  #2
        vst1.16         {d20[0]}, [\dst, :16], \d_strd
        vst1.16         {d20[1]}, [\ds2, :16], \d_strd
        ble             0f
        vtrn.32         d19, d16
        b               2b
0:
        pop             {r4-r11,pc}
.endif

40:     // 4xN hv
        add             \sr2, \src, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        vld1.8          {d28},  [\src], \s_strd
        vext.8          d29, d28, d28, #1
        vmull.u8        q8,  d28, d0
        vmlal.u8        q8,  d29, d1

4:
        vld1.8          {d28},  [\sr2], \s_strd
        vld1.8          {d30},  [\src], \s_strd
        vext.8          d29, d28, d28, #1
        vext.8          d31, d30, d30, #1
        vtrn.32         d28, d30
        vtrn.32         d29, d31
        vmull.u8        q9,  d28, d0
        vmlal.u8        q9,  d29, d1

        vmov            d17, d18

        vmul.u16        q10, q8, q2
        vmla.u16        q10, q9, q3
        subs            \h,  \h,  #2
.ifc \type, put
        vqrshrn.u16     d20, q10, #8
        vst1.32         {d20[0]}, [\dst, :32], \d_strd
        vst1.32         {d20[1]}, [\ds2, :32], \d_strd
.else
        vrshr.u16       q10, q10, #4
        vst1.16         {d20}, [\dst, :64], \d_strd
        vst1.16         {d21}, [\ds2, :64], \d_strd
.endif
        ble             0f
        vmov            d16, d19
        b               4b
0:
        pop             {r4-r11,pc}

80:     // 8xN, 16xN, ... hv
160:
320:
640:
1280:
        mov             \my, \h

1:
        add             \sr2, \src, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        vld1.8          {q12},  [\src], \s_strd
        vext.8          q13, q12, q12, #1
        vmull.u8        q8,  d24, d0
        vmlal.u8        q8,  d26, d1

2:
        vld1.8          {q12},  [\sr2], \s_strd
        vld1.8          {q14},  [\src], \s_strd
        vext.8          q13, q12, q12, #1
        vext.8          q15, q14, q14, #1
        vmull.u8        q9,  d24, d0
        vmlal.u8        q9,  d26, d1
        vmull.u8        q10, d28, d0
        vmlal.u8        q10, d30, d1

        vmul.u16        q8,  q8,  q2
        vmla.u16        q8,  q9,  q3
        vmul.u16        q9,  q9,  q2
        vmla.u16        q9,  q10, q3
        subs            \h,  \h,  #2
.ifc \type, put
        vqrshrn.u16     d16, q8,  #8
        vqrshrn.u16     d18, q9,  #8
        vst1.8          {d16}, [\dst, :64], \d_strd
        vst1.8          {d18}, [\ds2, :64], \d_strd
.else
        vrshr.u16       q8,  q8,  #4
        vrshr.u16       q9,  q9,  #4
        vst1.16         {q8}, [\dst, :128], \d_strd
        vst1.16         {q9}, [\ds2, :128], \d_strd
.endif
        ble             9f
        vmov            q8,  q10
        b               2b
9:
        subs            \w,  \w,  #8
        ble             0f
        asr             \s_strd,  \s_strd,  #1
        asr             \d_strd,  \d_strd,  #1
        mls             \src,  \s_strd,  \my,  \src
        mls             \dst,  \d_strd,  \my,  \dst
        sub             \src,  \src,  \s_strd,  lsl #1
        mov             \h,  \my
        add             \src,  \src,  #8
.ifc \type, put
        add             \dst,  \dst,  #8
.else
        add             \dst,  \dst,  #16
.endif
        b               1b
0:
        pop             {r4-r11,pc}
endfunc
.endm

filter_fn put,  r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, 10
filter_fn prep, r0, r7, r1, r2, r3, r4, r5, r6, r8, r9, 6

.macro load_filter_ptr src
        asr             r12, \src, #10
        add             r12, r11, r12, lsl #3
.endm

.macro load_filter_coef dst, src, inc
        add             \src, \src, \inc
        vld1.8          {\dst}, [r12, :64]
.endm

.macro load_filter_row dst, src, inc
        load_filter_ptr \src
        load_filter_coef \dst, \src, \inc
.endm

function warp_filter_horz_neon
        load_filter_ptr r5                  // filter 0
        vld1.16         {q7}, [r2], r3
        vmov.i8         q6,  #128

        load_filter_coef d0, r5,  r7        // filter 0
        load_filter_row d1,  r5,  r7        // filter 1
        load_filter_row d2,  r5,  r7        // filter 2
        load_filter_ptr r5                  // filter 3
        veor            q7,  q7,  q6        // subtract by 128 to allow using vmull
        load_filter_coef d3, r5,  r7        // filter 3
        vext.8          d12, d14, d15, #1   // filter 1 pixels
        vext.8          d13, d14, d15, #2   // filter 2 pixels
        load_filter_ptr r5                  // filter 4
        vmull.s8        q2,  d14, d0        // filter 0 output
        vmull.s8        q3,  d12, d1        // filter 1 output
        load_filter_coef d0, r5,  r7        // filter 4
        load_filter_ptr r5                  // filter 5
        vext.8          d12, d14, d15, #3   // filter 3 pixels
        vmull.s8        q4,  d13, d2        // filter 2 output
        vext.8          d13, d14, d15, #4   // filter 4 pixels
        vpadd.i16       d4,  d4,  d5        // pixel 0 (4x16)
        vpadd.i16       d5,  d6,  d7        // pixel 1 (4x16)
        load_filter_coef d1, r5,  r7        // filter 5
        load_filter_ptr r5                  // filter 6
        vmull.s8        q5,  d12, d3        // filter 3 output
        vext.8          d12, d14, d15, #5   // filter 5 pixels
        vmull.s8        q3,  d13, d0        // filter 4 output
        load_filter_coef d0, r5,  r7        // filter 6
        vext.8          d13, d14, d15, #6   // filter 6 pixels
        load_filter_ptr r5                  // filter 7
        vpadd.i16       d8,  d8,  d9        // pixel 2 (4x16)
        vpadd.i16       d9,  d10, d11       // pixel 3 (4x16)
        vmull.s8        q5,  d12, d1        // filter 5 output
        load_filter_coef d1, r5,  r7        // filter 7
        vext.8          d14, d14, d15, #7   // filter 7 pixels
        vpadd.i16       d6,  d6,  d7        // pixel 4 (4x16)
        vpadd.i16       d10, d10, d11       // pixel 5 (4x16)
        vmull.s8        q6,  d13, d0        // filter 6 output
        vmull.s8        q7,  d14, d1        // filter 7 output

        sub             r5,  r5,  r7, lsl #3

        vpadd.i16       d4,  d4,  d5        // pixel 0,1 (2x16)
        vpadd.i16       d5,  d8,  d9        // pixel 2,3 (2x16)
        vpadd.i16       d12, d12, d13       // pixel 6 (4x16)
        vpadd.i16       d14, d14, d15       // pixel 7 (4x16)
        vpadd.i16       d6,  d6,  d10       // pixel 4,5 (2x16)
        vpadd.i16       d10, d12, d14       // pixel 6,7 (2x16)
        vpadd.i16       d4,  d4,  d5        // pixel 0-3
        vpadd.i16       d5,  d6,  d10       // pixel 4-7

        add             r5,  r5,  r8

        bx              lr
endfunc

// void dav1d_warp_affine_8x8_8bpc_neon(
//         pixel *dst, const ptrdiff_t dst_stride,
//         const pixel *src, const ptrdiff_t src_stride,
//         const int16_t *const abcd, int mx, int my)
.macro warp t, shift
function warp_affine_8x8\t\()_8bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldr             r6,  [sp, #108]
        ldrd            r8,  r9,  [r4]
        sxth            r7,  r8
        asr             r8,  r8, #16
        asr             r4,  r9, #16
        sxth            r9,  r9
        mov             r10, #8
        sub             r2,  r2,  r3, lsl #1
        sub             r2,  r2,  r3
        sub             r2,  r2,  #3
        movrel          r11, X(mc_warp_filter), 64*8
.ifnb \t
        lsl             r1,  r1,  #1
.endif
        add             r5,  r5,  #512
        add             r6,  r6,  #512

        bl              warp_filter_horz_neon
        vrshr.s16       q8,  q2,  #3
        bl              warp_filter_horz_neon
        vrshr.s16       q9,  q2,  #3
        bl              warp_filter_horz_neon
        vrshr.s16       q10, q2,  #3
        bl              warp_filter_horz_neon
        vrshr.s16       q11, q2,  #3
        bl              warp_filter_horz_neon
        vrshr.s16       q12, q2,  #3
        bl              warp_filter_horz_neon
        vrshr.s16       q13, q2,  #3
        bl              warp_filter_horz_neon
        vrshr.s16       q14, q2,  #3

1:
        bl              warp_filter_horz_neon
        vrshr.s16       q15, q2,  #3

        load_filter_row d8,  r6,  r9
        load_filter_row d9,  r6,  r9
        load_filter_row d10, r6,  r9
        load_filter_row d11, r6,  r9
        load_filter_row d12, r6,  r9
        load_filter_row d13, r6,  r9
        load_filter_row d14, r6,  r9
        load_filter_row d15, r6,  r9
        transpose_8x8b  q4,  q5,  q6,  q7,  d8,  d9,  d10, d11, d12, d13, d14, d15
        vmovl.s8        q1,  d8
        vmovl.s8        q2,  d9
        vmovl.s8        q3,  d10
        vmovl.s8        q4,  d11
        vmovl.s8        q5,  d12
        vmovl.s8        q6,  d13

        sub             r6,  r6,  r9, lsl #3

        // This ordering of vmull/vmlal is highly beneficial for
        // Cortex A8/A9/A53 here, but harmful for Cortex A7.
        vmull.s16       q0,  d16,  d2
        vmlal.s16       q0,  d18,  d4
        vmlal.s16       q0,  d20,  d6
        vmlal.s16       q0,  d22,  d8
        vmlal.s16       q0,  d24,  d10
        vmlal.s16       q0,  d26,  d12
        vmull.s16       q1,  d17,  d3
        vmlal.s16       q1,  d19,  d5
        vmlal.s16       q1,  d21,  d7
        vmlal.s16       q1,  d23,  d9
        vmlal.s16       q1,  d25,  d11
        vmlal.s16       q1,  d27,  d13

        vmovl.s8        q2,  d14
        vmovl.s8        q3,  d15

        vmlal.s16       q0,  d28,  d4
        vmlal.s16       q0,  d30,  d6
        vmlal.s16       q1,  d29,  d5
        vmlal.s16       q1,  d31,  d7

.ifb \t
        vmov.i16        q7,  #128
.else
        vmov.i16        q7,  #0x800
.endif

        vmov            q8,  q9
        vmov            q9,  q10
        vqrshrn.s32     d0,  q0,  #\shift
        vmov            q10, q11
        vqrshrn.s32     d1,  q1,  #\shift
        vmov            q11, q12
        vadd.i16        q0,  q0,  q7
        vmov            q12, q13
.ifb \t
        vqmovun.s16     d0,  q0
.endif
        vmov            q13, q14
        vmov            q14, q15
        subs            r10, r10, #1
.ifnb \t
        vst1.16         {q0}, [r0, :128], r1
.else
        vst1.8          {d0}, [r0, :64], r1
.endif

        add             r6,  r6,  r4
        bgt             1b

        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc
.endm

warp  , 11
warp t, 7

// void dav1d_emu_edge_8bpc_neon(
//         const intptr_t bw, const intptr_t bh,
//         const intptr_t iw, const intptr_t ih,
//         const intptr_t x, const intptr_t y,
//         pixel *dst, const ptrdiff_t dst_stride,
//         const pixel *ref, const ptrdiff_t ref_stride)
function emu_edge_8bpc_neon, export=1
        push            {r4-r11,lr}
        ldrd            r4,  r5,  [sp, #36]
        ldrd            r6,  r7,  [sp, #44]
        ldrd            r8,  r9,  [sp, #52]

        // ref += iclip(y, 0, ih - 1) * PXSTRIDE(ref_stride)
        // ref += iclip(x, 0, iw - 1)
        sub             r12, r3,  #1           // ih - 1
        cmp             r5,  r3
        sub             lr,  r2,  #1           // iw - 1
        it              lt
        movlt           r12, r5                // min(y, ih - 1)
        cmp             r4,  r2
        bic             r12, r12, r12, asr #31 // max(min(y, ih - 1), 0)
        it              lt
        movlt           lr,  r4                // min(x, iw - 1)
        bic             lr,  lr,  lr,  asr #31 // max(min(x, iw - 1), 0)
        mla             r8,  r12, r9,  r8      // ref += iclip() * stride
        add             r8,  r8,  lr           // ref += iclip()

        // bottom_ext = iclip(y + bh - ih, 0, bh - 1)
        // top_ext = iclip(-y, 0, bh - 1)
        add             r10, r5,  r1           // y + bh
        neg             r5,  r5                // -y
        sub             r10, r10, r3           // y + bh - ih
        sub             r12, r1,  #1           // bh - 1
        cmp             r10, r1
        bic             r5,  r5,  r5,  asr #31 // max(-y, 0)
        it              ge
        movge           r10, r12               // min(y + bh - ih, bh-1)
        cmp             r5,  r1
        bic             r10, r10, r10, asr #31 // max(min(y + bh - ih, bh-1), 0)
        it              ge
        movge           r5,  r12               // min(max(-y, 0), bh-1)

        // right_ext = iclip(x + bw - iw, 0, bw - 1)
        // left_ext = iclip(-x, 0, bw - 1)
        add             r11, r4,  r0           // x + bw
        neg             r4,  r4                // -x
        sub             r11, r11, r2           // x + bw - iw
        sub             lr,  r0,  #1           // bw - 1
        cmp             r11, r0
        bic             r4,  r4,  r4,  asr #31 // max(-x, 0)
        it              ge
        movge           r11, lr                // min(x + bw - iw, bw-1)
        cmp             r4,  r0
        bic             r11, r11, r11, asr #31 // max(min(x + bw - iw, bw-1), 0)
        it              ge
        movge           r4,  lr                // min(max(-x, 0), bw - 1)

        // center_h = bh - top_ext - bottom_ext
        // dst += top_ext * PXSTRIDE(dst_stride)
        // center_w = bw - left_ext - right_ext
        sub             r1,  r1,  r5           // bh - top_ext
        mla             r6,  r5,  r7,  r6
        sub             r2,  r0,  r4           // bw - left_ext
        sub             r1,  r1,  r10          // center_h = bh - top_ext - bottom_ext
        sub             r2,  r2,  r11          // center_w = bw - left_ext - right_ext

        mov             r0,  r6                // backup of dst

.macro v_loop need_left, need_right
0:
.if \need_left
        vld1.8          {d0[], d1[]}, [r8]
        mov             r12, r6                // out = dst
        mov             r3,  r4
1:
        subs            r3,  r3,  #16
        vst1.8          {q0}, [r12, :128]!
        bgt             1b
.endif
        mov             lr,  r8
        add             r12, r6,  r4           // out = dst + left_ext
        mov             r3,  r2
1:
        vld1.8          {q0, q1}, [lr]!
        subs            r3,  r3,  #32
.if \need_left
        vst1.8          {q0, q1}, [r12]!
.else
        vst1.8          {q0, q1}, [r12, :128]!
.endif
        bgt             1b
.if \need_right
        add             r3,  r8,  r2           // in + center_w
        sub             r3,  r3,  #1           // in + center_w - 1
        add             r12, r6,  r4           // dst + left_ext
        vld1.8          {d0[], d1[]}, [r3]
        add             r12, r12, r2           // out = dst + left_ext + center_w
        mov             r3,  r11
1:
        subs            r3,  r3,  #16
        vst1.8          {q0}, [r12]!
        bgt             1b
.endif

        subs            r1,  r1,  #1           // center_h--
        add             r6,  r6,  r7
        add             r8,  r8,  r9
        bgt             0b
.endm

        cmp             r4,  #0
        beq             2f
        // need_left
        cmp             r11, #0
        beq             3f
        // need_left + need_right
        v_loop          1,   1
        b               5f

2:
        // !need_left
        cmp             r11, #0
        beq             4f
        // !need_left + need_right
        v_loop          0,   1
        b               5f

3:
        // need_left + !need_right
        v_loop          1,   0
        b               5f

4:
        // !need_left + !need_right
        v_loop          0,   0

5:
        cmp             r10, #0
        // Storing the original dst in r0 overwrote bw, recalculate it here
        add             r2,  r2,  r4           // center_w + left_ext
        add             r2,  r2,  r11          // bw = center_w + left_ext + right_ext

        beq             3f
        // need_bottom
        sub             r8,  r6,  r7           // ref = dst - stride
        mov             r4,  r2
1:
        vld1.8          {q0, q1}, [r8, :128]!
        mov             r3,  r10
2:
        subs            r3,  r3,  #1
        vst1.8          {q0, q1}, [r6, :128], r7
        bgt             2b
        mls             r6,  r7,  r10,  r6     // dst -= bottom_ext * stride
        subs            r4,  r4,  #32          // bw -= 32
        add             r6,  r6,  #32          // dst += 32
        bgt             1b

3:
        cmp             r5,  #0
        beq             3f
        // need_top
        mls             r6,  r7,  r5,  r0      // dst = stored_dst - top_ext * stride
1:
        vld1.8          {q0, q1}, [r0, :128]!
        mov             r3,  r5
2:
        subs            r3,  r3,  #1
        vst1.8          {q0, q1}, [r6, :128], r7
        bgt             2b
        mls             r6,  r7,  r5,  r6      // dst -= top_ext * stride
        subs            r2,  r2,  #32          // bw -= 32
        add             r6,  r6,  #32          // dst += 32
        bgt             1b

3:
        pop             {r4-r11,pc}
endfunc
