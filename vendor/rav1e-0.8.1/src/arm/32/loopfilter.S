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

.macro loop_filter wd
function lpf_8_wd\wd\()_neon
        vabd.u8         d0,  d22, d23 // abs(p1 - p0)
        vabd.u8         d1,  d25, d24 // abs(q1 - q0)
        vabd.u8         d2,  d23, d24 // abs(p0 - q0)
        vabd.u8         d3,  d22, d25 // abs(p1 - q1)
.if \wd >= 6
        vabd.u8         d4,  d21, d22 // abs(p2 - p1)
        vabd.u8         d5,  d26, d25 // abs(q2 - q1)
.endif
.if \wd >= 8
        vabd.u8         d6,  d20, d21 // abs(p3 - p2)
        vabd.u8         d7,  d27, d26 // abs(q3 - q3)
.endif
.if \wd >= 6
        vmax.u8         d4,  d4,  d5
.endif
        vqadd.u8        d2,  d2,  d2  // abs(p0 - q0) * 2
.if \wd >= 8
        vmax.u8         d6,  d6,  d7
.endif
        vshr.u8         d3,  d3,  #1
.if \wd >= 8
        vmax.u8         d4,  d4,  d6
.endif
.if \wd >= 6
        vand            d4,  d4,  d14
.endif
        vmax.u8         d0,  d0,  d1  // max(abs(p1 - p0), abs(q1 - q0))
        vqadd.u8        d2,  d2,  d3  // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1
.if \wd >= 6
        vmax.u8         d4,  d0,  d4
        vcge.u8         d1,  d11, d4  // max(abs(p1 - p0), abs(q1 - q0), abs(), abs(), ...) <= I
.else
        vcge.u8         d1,  d11, d0  // max(abs(p1 - p0), abs(q1 - q0)) <= I
.endif
        vcge.u8         d2,  d10, d2  // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1 <= E
        vand            d1,  d1,  d2  // fm
        vand            d1,  d1,  d13 // fm && wd >= 4
.if \wd >= 6
        vand            d14, d14, d1  // fm && wd > 4
.endif
.if \wd >= 16
        vand            d15, d15, d1  // fm && wd == 16
.endif

        vmov            r10, r11, d1
        orrs            r10, r10, r11
        beq             9f            // if (!fm || wd < 4) return;

.if \wd >= 6
        vmov.i8         d10, #1
        vabd.u8         d2,  d21, d23 // abs(p2 - p0)
        vabd.u8         d3,  d22, d23 // abs(p1 - p0)
        vabd.u8         d4,  d25, d24 // abs(q1 - q0)
        vabd.u8         d5,  d26, d24 // abs(q2 - q0)
.if \wd >= 8
        vabd.u8         d6,  d20, d23 // abs(p3 - p0)
        vabd.u8         d7,  d27, d24 // abs(q3 - q0)
.endif
        vmax.u8         d2,  d2,  d3
        vmax.u8         d4,  d4,  d5
.if \wd >= 8
        vmax.u8         d6,  d6,  d7
.endif
        vmax.u8         d2,  d2,  d4
.if \wd >= 8
        vmax.u8         d2,  d2,  d6
.endif

.if \wd == 16
        vabd.u8         d3,  d17, d23 // abs(p6 - p0)
        vabd.u8         d4,  d18, d23 // abs(p5 - p0)
        vabd.u8         d5,  d19, d23 // abs(p4 - p0)
.endif
        vcge.u8         d2,  d10, d2  // flat8in
.if \wd == 16
        vabd.u8         d6,  d28, d24 // abs(q4 - q0)
        vabd.u8         d7,  d29, d24 // abs(q5 - q0)
        vabd.u8         d8,  d30, d24 // abs(q6 - q0)
.endif
        vand            d14, d2,  d14 // flat8in && fm && wd > 4
        vbic            d1,  d1,  d14 // fm && wd >= 4 && !flat8in
.if \wd == 16
        vmax.u8         d3,  d3,  d4
        vmax.u8         d5,  d5,  d6
.endif
        vmov            r10, r11, d1
.if \wd == 16
        vmax.u8         d7,  d7,  d8
        vmax.u8         d3,  d3,  d5
        vmax.u8         d3,  d3,  d7
        vcge.u8         d3,  d10, d3  // flat8out
.endif
        orrs            r10, r10, r11
.if \wd == 16
        vand            d15, d15, d3  // flat8out && fm && wd == 16
        vand            d15, d15, d14 // flat8out && flat8in && fm && wd == 16
        vbic            d14, d14, d15 // flat8in && fm && wd >= 4 && !flat8out
.endif
        beq             1f            // skip wd == 4 case
.endif

        vsubl.u8        q1,  d22, d25 // p1 - q1
        vcgt.u8         d0,  d0,  d12 // hev
        vqmovn.s16      d2,  q1
        vand            d4,  d2,  d0  // if (hev) iclip_diff(p1 - q1)
        vbic            d0,  d1,  d0  // (fm && wd >= 4 && !hev)
        vsubl.u8        q1,  d24, d23
        vmov.i16        q3,  #3
        vmul.i16        q1,  q1,  q3
        vmov.i8         d6,  #4
        vaddw.s8        q1,  q1,  d4
        vmov.i8         d7,  #3
        vqmovn.s16      d2,  q1       // f
        vqadd.s8        d4,  d6,  d2  // imin(f + 4, 127)
        vqadd.s8        d5,  d7,  d2  // imin(f + 3, 127)
        vshr.s8         d4,  d4,  #3  // f1
        vshr.s8         d5,  d5,  #3  // f2
        vmovl.u8        q1,  d23      // p0
        vmovl.u8        q3,  d24      // q0
        vaddw.s8        q1,  q1,  d5
        vsubw.s8        q3,  q3,  d4
        vrshr.s8        d4,  d4,  #1  // (f1 + 1) >> 1
        vqmovun.s16     d2,  q1       // out p0
        vqmovun.s16     d6,  q3       // out q0
        vbit            d23, d2,  d1  // if (fm && wd >= 4)
        vmovl.u8        q1,  d22      // p1
        vbit            d24, d6,  d1  // if (fm && wd >= 4)
        vmovl.u8        q3,  d25      // q1
        vaddw.s8        q1,  q1,  d4
        vsubw.s8        q3,  q3,  d4
        vqmovun.s16     d2,  q1       // out p1
        vqmovun.s16     d6,  q3       // out q1
        vbit            d22, d2,  d0  // if (fm && wd >= 4 && !hev)
        vbit            d25, d6,  d0  // if (fm && wd >= 4 && !hev)
1:

.if \wd == 6
        vmov            r10, r11, d14
        orrs            r10, r10, r11
        beq             2f            // skip if there's no flat8in

        vaddl.u8        q0,  d21, d21 // p2 * 2
        vaddl.u8        q1,  d21, d22 // p2 + p1
        vaddl.u8        q2,  d22, d23 // p1 + p0
        vaddl.u8        q3,  d23, d24 // p0 + q0
        vadd.i16        q4,  q0,  q1
        vadd.i16        q5,  q2,  q3
        vaddl.u8        q6,  d24, d25 // q0 + q1
        vadd.i16        q4,  q4,  q5
        vsub.i16        q6,  q6,  q0
        vaddl.u8        q5,  d25, d26 // q1 + q2
        vrshrn.i16      d0,  q4,  #3  // out p1

        vadd.i16        q4,  q4,  q6
        vsub.i16        q5,  q5,  q1
        vaddl.u8        q6,  d26, d26 // q2 + q2
        vrshrn.i16      d1,  q4,  #3  // out p0

        vadd.i16        q4,  q4,  q5
        vsub.i16        q6,  q6,  q2
        vrshrn.i16      d2,  q4,  #3  // out q0

        vbit            d22, d0,  d14 // p1 if (flat8in)
        vadd.i16        q4,  q4,  q6
        vbit            d23, d1,  d14 // p0 if (flat8in)
        vrshrn.i16      d3,  q4,  #3  // out q1
        vbit            d24, d2,  d14 // q0 if (flat8in)
        vbit            d25, d3,  d14 // q1 if (flat8in)
.elseif \wd >= 8
        vmov            r10, r11, d14
        orrs            r10, r10, r11
.if \wd == 8
        beq             8f            // skip if there's no flat8in
.else
        beq             2f            // skip if there's no flat8in
.endif

        vaddl.u8        q0,  d20, d21 // p3 + p2
        vaddl.u8        q1,  d22, d25 // p1 + q1
        vaddl.u8        q2,  d20, d22 // p3 + p1
        vaddl.u8        q3,  d23, d26 // p0 + q2
        vadd.i16        q4,  q0,  q0  // 2 * (p3 + p2)
        vaddw.u8        q4,  q4,  d23 // + p0
        vaddw.u8        q4,  q4,  d24 // + q0
        vadd.i16        q4,  q4,  q2  // + p3 + p1
        vsub.i16        q1,  q1,  q0  // p1 + q1 - p3 - p2
        vsub.i16        q3,  q3,  q2  // p0 + q2 - p3 - p1
        vrshrn.i16      d10, q4,  #3  // out p2

        vadd.i16        q4,  q4,  q1
        vaddl.u8        q0,  d20, d23 // p3 + p0
        vaddl.u8        q1,  d24, d27 // q0 + q3
        vrshrn.i16      d11, q4,  #3  // out p1

        vadd.i16        q4,  q4,  q3
        vsub.i16        q1,  q1,  q0  // q0 + q3 - p3 - p0
        vaddl.u8        q2,  d21, d24 // p2 + q0
        vaddl.u8        q3,  d25, d27 // q1 + q3
        vrshrn.i16      d12, q4,  #3  // out p0

        vadd.i16        q4,  q4,  q1
        vsub.i16        q3,  q3,  q2  // q1 + q3 - p2 - q0
        vaddl.u8        q0,  d22, d25 // p1 + q1
        vaddl.u8        q1,  d26, d27 // q2 + q3
        vrshrn.i16      d13, q4,  #3  // out q0

        vadd.i16        q4,  q4,  q3
        vsub.i16        q1,  q1,  q0  // q2 + q3 - p1 - q1
        vrshrn.i16      d0,  q4,  #3  // out q1

        vadd.i16        q4,  q4,  q1

        vbit            d21, d10, d14
        vbit            d22, d11, d14
        vbit            d23, d12, d14
        vrshrn.i16      d1,  q4,  #3  // out q2
        vbit            d24, d13, d14
        vbit            d25, d0,  d14
        vbit            d26, d1,  d14
.endif
2:
.if \wd == 16
        vmov            r10, r11, d15
        orrs            r10, r10, r11
        bne             1f            // check if flat8out is needed
        vmov            r10, r11, d14
        orrs            r10, r10, r11
        beq             8f            // if there was no flat8in, just write the inner 4 pixels
        b               7f            // if flat8in was used, write the inner 6 pixels
1:

        vaddl.u8        q1,  d17, d17 // p6 + p6
        vaddl.u8        q2,  d17, d18 // p6 + p5
        vaddl.u8        q3,  d17, d19 // p6 + p4
        vaddl.u8        q4,  d17, d20 // p6 + p3
        vadd.i16        q6,  q1,  q2
        vadd.i16        q5,  q3,  q4
        vaddl.u8        q3,  d17, d21 // p6 + p2
        vadd.i16        q6,  q6,  q5
        vaddl.u8        q4,  d17, d22 // p6 + p1
        vaddl.u8        q5,  d18, d23 // p5 + p0
        vadd.i16        q3,  q3,  q4
        vaddl.u8        q4,  d19, d24 // p4 + q0
        vadd.i16        q6,  q6,  q3
        vadd.i16        q5,  q5,  q4
        vaddl.u8        q3,  d20, d25 // p3 + q1
        vadd.i16        q6,  q6,  q5
        vsub.i16        q3,  q3,  q1
        vaddl.u8        q1,  d21, d26 // p2 + q2
        vrshrn.i16      d0,  q6,  #4  // out p5
        vadd.i16        q6,  q6,  q3  // - (p6 + p6) + (p3 + q1)
        vsub.i16        q1,  q1,  q2
        vaddl.u8        q2,  d22, d27 // p1 + q3
        vaddl.u8        q3,  d17, d19 // p6 + p4
        vrshrn.i16      d1,  q6,  #4  // out p4
        vadd.i16        q6,  q6,  q1  // - (p6 + p5) + (p2 + q2)
        vsub.i16        q2,  q2,  q3
        vaddl.u8        q3,  d23, d28 // p0 + q4
        vaddl.u8        q4,  d17, d20 // p6 + p3
        vrshrn.i16      d2,  q6,  #4  // out p3
        vadd.i16        q6,  q6,  q2  // - (p6 + p4) + (p1 + q3)
        vsub.i16        q3,  q3,  q4
        vaddl.u8        q4,  d24, d29 // q0 + q5
        vaddl.u8        q2,  d17, d21 // p6 + p2
        vrshrn.i16      d3,  q6,  #4  // out p2
        vadd.i16        q6,  q6,  q3  // - (p6 + p3) + (p0 + q4)
        vsub.i16        q4,  q4,  q2
        vaddl.u8        q3,  d25, d30 // q1 + q6
        vaddl.u8        q5,  d17, d22 // p6 + p1
        vrshrn.i16      d4,  q6,  #4  // out p1
        vadd.i16        q6,  q6,  q4  // - (p6 + p2) + (q0 + q5)
        vsub.i16        q3,  q3,  q5
        vaddl.u8        q4,  d26, d30 // q2 + q6
        vbif            d0,  d18, d15 // out p5
        vaddl.u8        q5,  d18, d23 // p5 + p0
        vrshrn.i16      d5,  q6,  #4  // out p0
        vadd.i16        q6,  q6,  q3  // - (p6 + p1) + (q1 + q6)
        vsub.i16        q4,  q4,  q5
        vaddl.u8        q5,  d27, d30 // q3 + q6
        vbif            d1,  d19, d15 // out p4
        vaddl.u8        q9,  d19, d24 // p4 + q0
        vrshrn.i16      d6,  q6,  #4  // out q0
        vadd.i16        q6,  q6,  q4  // - (p5 + p0) + (q2 + q6)
        vsub.i16        q5,  q5,  q9
        vaddl.u8        q4,  d28, d30 // q4 + q6
        vbif            d2,  d20, d15 // out p3
        vaddl.u8        q9,  d20, d25 // p3 + q1
        vrshrn.i16      d7,  q6,  #4  // out q1
        vadd.i16        q6,  q6,  q5  // - (p4 + q0) + (q3 + q6)
        vsub.i16        q9,  q4,  q9
        vaddl.u8        q5,  d29, d30 // q5 + q6
        vbif            d3,  d21, d15 // out p2
        vaddl.u8        q10, d21, d26 // p2 + q2
        vrshrn.i16      d8,  q6,  #4  // out q2
        vadd.i16        q6,  q6,  q9  // - (p3 + q1) + (q4 + q6)
        vsub.i16        q5,  q5,  q10
        vaddl.u8        q9,  d30, d30 // q6 + q6
        vbif            d4,  d22, d15 // out p1
        vaddl.u8        q10, d22, d27 // p1 + q3
        vrshrn.i16      d9,  q6,  #4  // out q3
        vadd.i16        q6,  q6,  q5  // - (p2 + q2) + (q5 + q6)
        vsub.i16        q9,  q9,  q10
        vbif            d5,  d23, d15 // out p0
        vrshrn.i16      d10, q6,  #4  // out q4
        vadd.i16        q6,  q6,  q9  // - (p1 + q3) + (q6 + q6)
        vrshrn.i16      d11, q6,  #4  // out q5
        vbif            d6,  d24, d15 // out q0
        vbif            d7,  d25, d15 // out q1
        vbif            d8,  d26, d15 // out q2
        vbif            d9,  d27, d15 // out q3
        vbif            d10, d28, d15 // out q4
        vbif            d11, d29, d15 // out q5
.endif

        bx              lr
.if \wd == 16
7:
        // Return to a shorter epilogue, writing only the inner 6 pixels
        bx              r8
.endif
.if \wd >= 8
8:
        // Return to a shorter epilogue, writing only the inner 4 pixels
        bx              r9
.endif
9:
        // Return directly without writing back any pixels
        bx              r12
endfunc
.endm

loop_filter 16
loop_filter 8
loop_filter 6
loop_filter 4

.macro lpf_8_wd16
        adr             r8,  7f + CONFIG_THUMB
        adr             r9,  8f + CONFIG_THUMB
        bl              lpf_8_wd16_neon
.endm

.macro lpf_8_wd8
        adr             r9,  8f + CONFIG_THUMB
        bl              lpf_8_wd8_neon
.endm

.macro lpf_8_wd6
        bl              lpf_8_wd6_neon
.endm

.macro lpf_8_wd4
        bl              lpf_8_wd4_neon
.endm

function lpf_v_4_8_neon
        mov             r12, lr
        sub             r10, r0,  r1, lsl #1
        vld1.8          {d22}, [r10, :64], r1 // p1
        vld1.8          {d24}, [r0,  :64], r1 // q0
        vld1.8          {d23}, [r10, :64], r1 // p0
        vld1.8          {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1

        lpf_8_wd4

        sub             r10, r0,  r1, lsl #1
        vst1.8          {d22}, [r10, :64], r1 // p1
        vst1.8          {d24}, [r0,  :64], r1 // q0
        vst1.8          {d23}, [r10, :64], r1 // p0
        vst1.8          {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_4_8_neon
        mov             r12, lr
        sub             r10, r0,  #2
        add             r0,  r10, r1, lsl #2
        vld1.32         {d22[0]}, [r10], r1
        vld1.32         {d22[1]}, [r0],  r1
        vld1.32         {d23[0]}, [r10], r1
        vld1.32         {d23[1]}, [r0],  r1
        vld1.32         {d24[0]}, [r10], r1
        vld1.32         {d24[1]}, [r0],  r1
        vld1.32         {d25[0]}, [r10], r1
        vld1.32         {d25[1]}, [r0],  r1
        add             r0,  r0,  #2

        transpose_4x8b  q11, q12, d22, d23, d24, d25

        lpf_8_wd4

        sub             r10, r0,  r1, lsl #3
        sub             r10, r10, #2
        transpose_4x8b  q11, q12, d22, d23, d24, d25
        add             r0,  r10, r1, lsl #2

        vst1.32         {d22[0]}, [r10], r1
        vst1.32         {d22[1]}, [r0],  r1
        vst1.32         {d23[0]}, [r10], r1
        vst1.32         {d23[1]}, [r0],  r1
        vst1.32         {d24[0]}, [r10], r1
        vst1.32         {d24[1]}, [r0],  r1
        vst1.32         {d25[0]}, [r10], r1
        vst1.32         {d25[1]}, [r0],  r1
        add             r0,  r0,  #2
        bx              r12
endfunc

function lpf_v_6_8_neon
        mov             r12, lr
        sub             r10, r0,  r1, lsl #1
        sub             r10, r10, r1
        vld1.8          {d21}, [r10, :64], r1 // p2
        vld1.8          {d24}, [r0,  :64], r1 // q0
        vld1.8          {d22}, [r10, :64], r1 // p1
        vld1.8          {d25}, [r0,  :64], r1 // q1
        vld1.8          {d23}, [r10, :64], r1 // p0
        vld1.8          {d26}, [r0,  :64], r1 // q2
        sub             r0,  r0,  r1, lsl #1
        sub             r0,  r0,  r1

        lpf_8_wd6

        sub             r10, r0,  r1, lsl #1
        vst1.8          {d22}, [r10, :64], r1 // p1
        vst1.8          {d24}, [r0,  :64], r1 // q0
        vst1.8          {d23}, [r10, :64], r1 // p0
        vst1.8          {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_6_8_neon
        mov             r12, lr
        sub             r10, r0,  #4
        add             r0,  r10, r1, lsl #2
        vld1.8          {d20}, [r10], r1
        vld1.8          {d24}, [r0],  r1
        vld1.8          {d21}, [r10], r1
        vld1.8          {d25}, [r0],  r1
        vld1.8          {d22}, [r10], r1
        vld1.8          {d26}, [r0],  r1
        vld1.8          {d23}, [r10], r1
        vld1.8          {d27}, [r0],  r1
        add             r0,  r0,  #4

        transpose_8x8b  q10, q11, q12, q13, d20, d21, d22, d23, d24, d25, d26, d27

        lpf_8_wd6

        sub             r10, r0,  r1, lsl #3
        sub             r10, r10, #2
        transpose_4x8b  q11, q12, d22, d23, d24, d25
        add             r0,  r10, r1, lsl #2

        vst1.32         {d22[0]}, [r10], r1
        vst1.32         {d22[1]}, [r0],  r1
        vst1.32         {d23[0]}, [r10], r1
        vst1.32         {d23[1]}, [r0],  r1
        vst1.32         {d24[0]}, [r10], r1
        vst1.32         {d24[1]}, [r0],  r1
        vst1.32         {d25[0]}, [r10], r1
        vst1.32         {d25[1]}, [r0],  r1
        add             r0,  r0,  #2
        bx              r12
endfunc

function lpf_v_8_8_neon
        mov             r12, lr
        sub             r10, r0,  r1, lsl #2
        vld1.8          {d20}, [r10, :64], r1 // p3
        vld1.8          {d24}, [r0,  :64], r1 // q0
        vld1.8          {d21}, [r10, :64], r1 // p2
        vld1.8          {d25}, [r0,  :64], r1 // q1
        vld1.8          {d22}, [r10, :64], r1 // p1
        vld1.8          {d26}, [r0,  :64], r1 // q2
        vld1.8          {d23}, [r10, :64], r1 // p0
        vld1.8          {d27}, [r0,  :64], r1 // q3
        sub             r0,  r0,  r1, lsl #2

        lpf_8_wd8

        sub             r10, r0,  r1, lsl #1
        sub             r10, r10, r1
        vst1.8          {d21}, [r10, :64], r1 // p2
        vst1.8          {d24}, [r0,  :64], r1 // q0
        vst1.8          {d22}, [r10, :64], r1 // p1
        vst1.8          {d25}, [r0,  :64], r1 // q1
        vst1.8          {d23}, [r10, :64], r1 // p0
        vst1.8          {d26}, [r0,  :64], r1 // q2
        sub             r0,  r0,  r1, lsl #1
        sub             r0,  r0,  r1
        bx              r12

8:
        sub             r10, r0,  r1, lsl #1
        vst1.8          {d22}, [r10, :64], r1 // p1
        vst1.8          {d24}, [r0,  :64], r1 // q0
        vst1.8          {d23}, [r10, :64], r1 // p0
        vst1.8          {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_8_8_neon
        mov             r12, lr
        sub             r10, r0,  #4
        add             r0,  r10, r1, lsl #2
        vld1.8          {d20}, [r10], r1
        vld1.8          {d24}, [r0],  r1
        vld1.8          {d21}, [r10], r1
        vld1.8          {d25}, [r0],  r1
        vld1.8          {d22}, [r10], r1
        vld1.8          {d26}, [r0],  r1
        vld1.8          {d23}, [r10], r1
        vld1.8          {d27}, [r0],  r1
        add             r0,  r0,  #4

        transpose_8x8b  q10, q11, q12, q13, d20, d21, d22, d23, d24, d25, d26, d27

        lpf_8_wd8

        sub             r10, r0,  r1, lsl #3
        sub             r10, r10, #4
        transpose_8x8b  q10, q11, q12, q13, d20, d21, d22, d23, d24, d25, d26, d27
        add             r0,  r10, r1, lsl #2

        vst1.8          {d20}, [r10], r1
        vst1.8          {d24}, [r0],  r1
        vst1.8          {d21}, [r10], r1
        vst1.8          {d25}, [r0],  r1
        vst1.8          {d22}, [r10], r1
        vst1.8          {d26}, [r0],  r1
        vst1.8          {d23}, [r10], r1
        vst1.8          {d27}, [r0],  r1
        add             r0,  r0,  #4
        bx              r12
8:
        sub             r10, r0,  r1, lsl #3
        sub             r10, r10, #2
        transpose_4x8b  q11, q12, d22, d23, d24, d25
        add             r0,  r10, r1, lsl #2

        vst1.32         {d22[0]}, [r10], r1
        vst1.32         {d22[1]}, [r0],  r1
        vst1.32         {d23[0]}, [r10], r1
        vst1.32         {d23[1]}, [r0],  r1
        vst1.32         {d24[0]}, [r10], r1
        vst1.32         {d24[1]}, [r0],  r1
        vst1.32         {d25[0]}, [r10], r1
        vst1.32         {d25[1]}, [r0],  r1
        add             r0,  r0,  #2
        bx              r12
endfunc

function lpf_v_16_8_neon
        mov             r12, lr

        sub             r10, r0,  r1, lsl #3
        add             r10, r10, r1
        vld1.8          {d17}, [r10, :64], r1 // p6
        vld1.8          {d24}, [r0,  :64], r1 // q0
        vld1.8          {d18}, [r10, :64], r1 // p5
        vld1.8          {d25}, [r0,  :64], r1 // q1
        vld1.8          {d19}, [r10, :64], r1 // p4
        vld1.8          {d26}, [r0,  :64], r1 // q2
        vld1.8          {d20}, [r10, :64], r1 // p3
        vld1.8          {d27}, [r0,  :64], r1 // q3
        vld1.8          {d21}, [r10, :64], r1 // p2
        vld1.8          {d28}, [r0,  :64], r1 // q4
        vld1.8          {d22}, [r10, :64], r1 // p1
        vld1.8          {d29}, [r0,  :64], r1 // q5
        vld1.8          {d23}, [r10, :64], r1 // p0
        vld1.8          {d30}, [r0,  :64], r1 // q6
        sub             r0,  r0,  r1, lsl #3
        add             r0,  r0,  r1

        lpf_8_wd16

        sub             r10, r0,  r1, lsl #2
        sub             r10, r10, r1, lsl #1
        vst1.8          {d0},  [r10, :64], r1 // p5
        vst1.8          {d6},  [r0,  :64], r1 // q0
        vst1.8          {d1},  [r10, :64], r1 // p4
        vst1.8          {d7},  [r0,  :64], r1 // q1
        vst1.8          {d2},  [r10, :64], r1 // p3
        vst1.8          {d8},  [r0,  :64], r1 // q2
        vst1.8          {d3},  [r10, :64], r1 // p2
        vst1.8          {d9},  [r0,  :64], r1 // q3
        vst1.8          {d4},  [r10, :64], r1 // p1
        vst1.8          {d10}, [r0,  :64], r1 // q4
        vst1.8          {d5},  [r10, :64], r1 // p0
        vst1.8          {d11}, [r0,  :64], r1 // q5
        sub             r0,  r0,  r1, lsl #2
        sub             r0,  r0,  r1, lsl #1
        bx              r12
7:
        sub             r10, r0,  r1
        sub             r10, r10, r1, lsl #1
        vst1.8          {d21}, [r10, :64], r1 // p2
        vst1.8          {d24}, [r0,  :64], r1 // q0
        vst1.8          {d22}, [r10, :64], r1 // p1
        vst1.8          {d25}, [r0,  :64], r1 // q1
        vst1.8          {d23}, [r10, :64], r1 // p0
        vst1.8          {d26}, [r0,  :64], r1 // q2
        sub             r0,  r0,  r1, lsl #1
        sub             r0,  r0,  r1
        bx              r12

8:
        sub             r10, r0,  r1, lsl #1
        vst1.8          {d22}, [r10, :64], r1 // p1
        vst1.8          {d24}, [r0,  :64], r1 // q0
        vst1.8          {d23}, [r10, :64], r1 // p0
        vst1.8          {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_16_8_neon
        mov             r12, lr
        sub             r10, r0,  #8
        vld1.8          {d16}, [r10, :64], r1
        vld1.8          {d24}, [r0,  :64], r1
        vld1.8          {d17}, [r10, :64], r1
        vld1.8          {d25}, [r0,  :64], r1
        vld1.8          {d18}, [r10, :64], r1
        vld1.8          {d26}, [r0,  :64], r1
        vld1.8          {d19}, [r10, :64], r1
        vld1.8          {d27}, [r0,  :64], r1
        vld1.8          {d20}, [r10, :64], r1
        vld1.8          {d28}, [r0,  :64], r1
        vld1.8          {d21}, [r10, :64], r1
        vld1.8          {d29}, [r0,  :64], r1
        vld1.8          {d22}, [r10, :64], r1
        vld1.8          {d30}, [r0,  :64], r1
        vld1.8          {d23}, [r10, :64], r1
        vld1.8          {d31}, [r0,  :64], r1

        transpose_8x8b  q8,  q9,  q10, q11, d16, d17, d18, d19, d20, d21, d22, d23
        transpose_8x8b  q12, q13, q14, q15, d24, d25, d26, d27, d28, d29, d30, d31

        lpf_8_wd16

        sub             r0,  r0,  r1, lsl #3
        sub             r10, r0,  #8

        transpose_8x8b  q8,  q0,  q1,  q2,  d16, d17, d0,  d1,  d2,  d3,  d4,  d5
        transpose_8x8b  q3,  q4,  q5,  q15, d6,  d7,  d8,  d9,  d10, d11, d30, d31

        vst1.8          {d16}, [r10, :64], r1
        vst1.8          {d6},  [r0,  :64], r1
        vst1.8          {d17}, [r10, :64], r1
        vst1.8          {d7},  [r0,  :64], r1
        vst1.8          {d0},  [r10, :64], r1
        vst1.8          {d8},  [r0,  :64], r1
        vst1.8          {d1},  [r10, :64], r1
        vst1.8          {d9},  [r0,  :64], r1
        vst1.8          {d2},  [r10, :64], r1
        vst1.8          {d10}, [r0,  :64], r1
        vst1.8          {d3},  [r10, :64], r1
        vst1.8          {d11}, [r0,  :64], r1
        vst1.8          {d4},  [r10, :64], r1
        vst1.8          {d30}, [r0,  :64], r1
        vst1.8          {d5},  [r10, :64], r1
        vst1.8          {d31}, [r0,  :64], r1
        bx              r12

7:
        sub             r10, r0,  r1, lsl #3
        sub             r10, r10, #4
        transpose_8x8b  q10, q11, q12, q13, d20, d21, d22, d23, d24, d25, d26, d27
        add             r0,  r10, r1, lsl #2

        vst1.8          {d20}, [r10], r1
        vst1.8          {d24}, [r0],  r1
        vst1.8          {d21}, [r10], r1
        vst1.8          {d25}, [r0],  r1
        vst1.8          {d22}, [r10], r1
        vst1.8          {d26}, [r0],  r1
        vst1.8          {d23}, [r10], r1
        vst1.8          {d27}, [r0],  r1
        add             r0,  r0,  #4
        bx              r12
8:
        sub             r10, r0,  r1, lsl #3
        sub             r10, r10, #2
        transpose_4x8b  q11, q12, d22, d23, d24, d25
        add             r0,  r10, r1, lsl #2

        vst1.32         {d22[0]}, [r10], r1
        vst1.32         {d22[1]}, [r0],  r1
        vst1.32         {d23[0]}, [r10], r1
        vst1.32         {d23[1]}, [r0],  r1
        vst1.32         {d24[0]}, [r10], r1
        vst1.32         {d24[1]}, [r0],  r1
        vst1.32         {d25[0]}, [r10], r1
        vst1.32         {d25[1]}, [r0],  r1
        add             r0,  r0,  #2
        bx              r12
endfunc

// void dav1d_lpf_v_sb_y_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                 const uint32_t *const vmask,
//                                 const uint8_t (*l)[4], ptrdiff_t b4_stride,
//                                 const Av1FilterLUT *lut, const int w)

.macro lpf_func dir, type
function lpf_\dir\()_sb_\type\()_8bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldrd            r6,  r7,  [r2] // vmask[0], vmask[1]
.ifc \type, y
        ldr             r2,  [r2, #8]  // vmask[2]
.endif
        add             r5,  r5,  #128 // Move to sharp part of lut
.ifc \type, y
        orr             r7,  r7,  r2   // vmask[1] |= vmask[2]
.endif
.ifc \dir, v
        sub             r4,  r3,  r4, lsl #2
.else
        sub             r3,  r3,  #4
        lsl             r4,  r4,  #2
.endif
        orr             r6,  r6,  r7   // vmask[0] |= vmask[1]

1:
        tst             r6,  #0x03
.ifc \dir, v
        vld1.8          {d0}, [r4]!
        vld1.8          {d1}, [r3]!
.else
        vld2.32         {d0[0], d1[0]}, [r3], r4
        vld2.32         {d0[1], d1[1]}, [r3], r4
.endif
        beq             7f             // if (!(vm & bits)) continue;

        vld1.8          {d5[]}, [r5]   // sharp[0]
        add             r5,  r5,  #8
        vmov.i32        d2,  #0xff
        vdup.32         d13, r6        // vmask[0]

        vand            d0,  d0,  d2   // Keep only lowest byte in each 32 bit word
        vand            d1,  d1,  d2
        vtst.8          d3,  d1,  d2   // Check for nonzero values in l[0][0]
        vmov.i8         d4,  #1
        vld1.8          {d6[]}, [r5]   // sharp[1]
        sub             r5,  r5,  #8
        vbif            d1,  d0,  d3   // if (!l[0][0]) L = l[offset][0]
        vtst.32         d2,  d1,  d2   // L != 0
        vmul.i32        d1,  d1,  d4   // L
.ifc \type, y
        vdup.32         d15, r2        // vmask[2]
.endif
        vdup.32         d14, r7        // vmask[1]
        vmov            r10, r11, d2
        orrs            r10, r10, r11
        beq             7f             // if (!L) continue;
        vneg.s8         d5,  d5        // -sharp[0]
        movrel_local    r10, word_12
        vshr.u8         d12, d1,  #4   // H
        vld1.32         {d16}, [r10, :64]
        vshl.s8         d3,  d1,  d5   // L >> sharp[0]
.ifc \type, y
        vtst.32         d15, d15, d16  // if (vmask[2] & bits)
.endif
        vmov.i8         d7,  #2
        vmin.u8         d3,  d3,  d6   // imin(L >> sharp[0], sharp[1])
        vadd.i8         d0,  d1,  d7   // L + 2
        vmax.u8         d11, d3,  d4   // imax(imin(), 1) = limit = I
        vadd.u8         d0,  d0,  d0   // 2*(L + 2)
        vtst.32         d14, d14, d16  // if (vmask[1] & bits)
        vadd.i8         d10, d0,  d11  // 2*(L + 2) + limit = E
        vtst.32         d13, d13, d16  // if (vmask[0] & bits)
        vand            d13, d13, d2   // vmask[0] &= L != 0

.ifc \type, y
        tst             r2,  #0x03
        beq             2f
        // wd16
        bl              lpf_\dir\()_16_8_neon
        b               8f
2:
.endif
        tst             r7,  #0x03
        beq             3f
.ifc \type, y
        // wd8
        bl              lpf_\dir\()_8_8_neon
.else
        // wd6
        bl              lpf_\dir\()_6_8_neon
.endif
        b               8f
3:
        // wd4
        bl              lpf_\dir\()_4_8_neon
.ifc \dir, h
        b               8f
7:
        // For dir h, the functions above increment r0.
        // If the whole function is skipped, increment it here instead.
        add             r0,  r0,  r1,  lsl #3
.else
7:
.endif
8:
        lsrs            r6,  r6,  #2   // vmask[0] >>= 2
        lsr             r7,  r7,  #2   // vmask[1] >>= 2
.ifc \type, y
        lsr             r2,  r2,  #2   // vmask[2] >>= 2
.endif
.ifc \dir, v
        add             r0,  r0,  #8
.else
        // For dir h, r0 is returned incremented
.endif
        bne             1b

        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc
.endm

lpf_func v, y
lpf_func h, y
lpf_func v, uv
lpf_func h, uv

const word_12, align=4
        .word 1, 2
endconst
