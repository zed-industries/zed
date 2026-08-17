/*
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
 */

#include "src/arm/asm.S"
#include "util.S"

.macro loop_filter wd
function lpf_4_wd\wd\()_neon
        vabd.u16        d0,  d22, d23 // abs(p1 - p0)
        vabd.u16        d1,  d25, d24 // abs(q1 - q0)
        vabd.u16        d2,  d23, d24 // abs(p0 - q0)
        vabd.u16        d3,  d22, d25 // abs(p1 - q1)
.if \wd >= 6
        vabd.u16        d4,  d21, d22 // abs(p2 - p1)
        vabd.u16        d5,  d26, d25 // abs(q2 - q1)
.endif
.if \wd >= 8
        vabd.u16        d6,  d20, d21 // abs(p3 - p2)
        vabd.u16        d7,  d27, d26 // abs(q3 - q3)
.endif
.if \wd >= 6
        vmax.u16        d4,  d4,  d5
.endif
        vqadd.u16       d2,  d2,  d2  // abs(p0 - q0) * 2
.if \wd >= 8
        vmax.u16        d6,  d6,  d7
.endif
        vshr.u16        d3,  d3,  #1
.if \wd >= 8
        vmax.u16        d4,  d4,  d6
.endif
        vmax.u16        d0,  d0,  d1  // max(abs(p1 - p0), abs(q1 - q0))
        vqadd.u16       d2,  d2,  d3  // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1
.if \wd >= 6
        vmax.u16        d4,  d0,  d4
        vcge.u16        d1,  d11, d4  // max(abs(p1 - p0), abs(q1 - q0), abs(), abs(), ...) <= I
.else
        vcge.u16        d1,  d11, d0  // max(abs(p1 - p0), abs(q1 - q0)) <= I
.endif
        vcge.u16        d2,  d10, d2  // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1 <= E
        vand            d1,  d1,  d2  // fm && wd >= 4 (implicit)
.if \wd >= 6
        vmov            d14, d1       // fm && wd > 4 (implicit)
.endif
.if \wd >= 16
        vmov            d15, d1       // fm && wd == 16 (implicit)
.endif

        vmov            r10, r11, d1
        orrs            r10, r10, r11
        beq             9f            // if (!fm || wd < 4) return;

.if \wd >= 6
        vmov.i16        d10, #1
        vabd.u16        d2,  d21, d23 // abs(p2 - p0)
        vabd.u16        d3,  d22, d23 // abs(p1 - p0)
        vabd.u16        d4,  d25, d24 // abs(q1 - q0)
        vabd.u16        d5,  d26, d24 // abs(q2 - q0)
        vdup.16         d9,  r9       // bitdepth_min_8
.if \wd >= 8
        vabd.u16        d6,  d20, d23 // abs(p3 - p0)
        vabd.u16        d7,  d27, d24 // abs(q3 - q0)
.endif
        vmax.u16        d2,  d2,  d3
        vmax.u16        d4,  d4,  d5
.if \wd >= 8
        vmax.u16        d6,  d6,  d7
.endif
        vmax.u16        d2,  d2,  d4
        vshl.u16        d10, d10, d9  // F = 1 << bitdepth_min_8
.if \wd >= 8
        vmax.u16        d2,  d2,  d6
.endif

.if \wd == 16
        vabd.u16        d3,  d17, d23 // abs(p6 - p0)
        vabd.u16        d4,  d18, d23 // abs(p5 - p0)
        vabd.u16        d5,  d19, d23 // abs(p4 - p0)
.endif
        vcge.u16        d2,  d10, d2  // flat8in
.if \wd == 16
        vabd.u16        d6,  d28, d24 // abs(q4 - q0)
        vabd.u16        d7,  d29, d24 // abs(q5 - q0)
        vabd.u16        d8,  d30, d24 // abs(q6 - q0)
.endif
        vand            d14, d2,  d14 // flat8in && fm && wd > 4
        vbic            d1,  d1,  d14 // fm && wd >= 4 && !flat8in
.if \wd == 16
        vmax.u16        d3,  d3,  d4
        vmax.u16        d5,  d5,  d6
.endif
        vmov            r10, r11, d1
.if \wd == 16
        vmax.u16        d7,  d7,  d8
        vmax.u16        d3,  d3,  d5
        vmax.u16        d3,  d3,  d7
        vcge.u16        d3,  d10, d3  // flat8out
.endif
        orrs            r10, r10, r11
.if \wd == 16
        vand            d15, d15, d3  // flat8out && fm && wd == 16
        vand            d15, d15, d14 // flat8out && flat8in && fm && wd == 16
        vbic            d14, d14, d15 // flat8in && fm && wd >= 4 && !flat8out
.endif
        beq             1f            // skip wd == 4 case
.endif

        vdup.16         d3,  r8       // bitdepth_max
        vsub.u16        d2,  d22, d25 // p1 - q1
        vshr.u16        d3,  d3,  #1  // 128 << bitdepth_min_8 - 1
        vcgt.u16        d0,  d0,  d12 // hev
        vmvn            d9,  d3       // - 128 * (1 << bitdepth_min_8)
        vmin.s16        d2,  d2,  d3  // iclip_diff(p1 - q1)
        vmax.s16        d2,  d2,  d9  // iclip_diff(p1 - q1)
        vand            d4,  d2,  d0  // if (hev) iclip_diff(p1 - q1)
        vsub.u16        d2,  d24, d23
        vmov.i16        d6,  #3
        vbic            d0,  d1,  d0  // (fm && wd >= 4 && !hev)
        vmul.i16        d2,  d2,  d6
        vmov.i16        d7,  #4
        vadd.i16        d2,  d2,  d4
        vmin.s16        d2,  d2,  d3  // f = iclip_diff()
        vmax.s16        d2,  d2,  d9  // f = iclip_diff()
        vqadd.s16       d4,  d7,  d2  // f + 4
        vqadd.s16       d5,  d6,  d2  // f + 3
        vmin.s16        d4,  d4,  d3  // imin(f + 4, 128 << bitdepth_min_8 - 1)
        vmin.s16        d5,  d5,  d3  // imin(f + 3, 128 << bitdepth_min_8 - 1)
        vshr.s16        d4,  d4,  #3  // f1
        vshr.s16        d5,  d5,  #3  // f2
        vmov.i16        d9,  #0
        vdup.16         d3,  r8       // bitdepth_max
        vqadd.s16       d2,  d23, d5  // p0 + f2
        vqsub.s16       d6,  d24, d4  // q0 - f1
        vrshr.s16       d4,  d4,  #1  // (f1 + 1) >> 1
        vmin.s16        d2,  d2,  d3  // out p0 = iclip_pixel()
        vmin.s16        d6,  d6,  d3  // out q0 = iclip_pixel()
        vmax.s16        d2,  d2,  d9  // out p0 = iclip_pixel()
        vmax.s16        d6,  d6,  d9  // out q0 = iclip_pixel()
        vbit            d23, d2,  d1  // if (fm && wd >= 4)
        vbit            d24, d6,  d1  // if (fm && wd >= 4)
        vqadd.s16       d2,  d22, d4  // p1 + f
        vqsub.s16       d6,  d25, d4  // q1 - f
        vmin.s16        d2,  d2,  d3  // out p1 = iclip_pixel()
        vmin.s16        d6,  d6,  d3  // out q1 = iclip_pixel()
        vmax.s16        d2,  d2,  d9  // out p1 = iclip_pixel()
        vmax.s16        d6,  d6,  d9  // out q1 = iclip_pixel()
        vbit            d22, d2,  d0  // if (fm && wd >= 4 && !hev)
        vbit            d25, d6,  d0  // if (fm && wd >= 4 && !hev)
1:

.if \wd == 6
        vmov            r10, r11, d14
        orrs            r10, r10, r11
        beq             2f            // skip if there's no flat8in

        vadd.i16        d0,  d21, d21 // p2 * 2
        vadd.i16        d2,  d21, d22 // p2 + p1
        vadd.i16        d4,  d22, d23 // p1 + p0
        vadd.i16        d6,  d23, d24 // p0 + q0
        vadd.i16        d8,  d0,  d2
        vadd.i16        d10, d4,  d6
        vadd.i16        d12, d24, d25 // q0 + q1
        vadd.i16        d8,  d8,  d10
        vsub.i16        d12, d12, d0
        vadd.i16        d10, d25, d26 // q1 + q2
        vrshr.u16       d0,  d8,  #3  // out p1

        vadd.i16        d8,  d8,  d12
        vsub.i16        d10, d10, d2
        vadd.i16        d12, d26, d26 // q2 + q2
        vrshr.u16       d1,  d8,  #3  // out p0

        vadd.i16        d8,  d8,  d10
        vsub.i16        d12, d12, d4
        vrshr.u16       d2,  d8,  #3  // out q0

        vbit            d22, d0,  d14 // p1 if (flat8in)
        vadd.i16        d8,  d8,  d12
        vbit            d23, d1,  d14 // p0 if (flat8in)
        vrshr.u16       d3,  d8,  #3  // out q1
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

        vadd.i16        d0,  d20, d21 // p3 + p2
        vadd.i16        d2,  d22, d25 // p1 + q1
        vadd.i16        d4,  d20, d22 // p3 + p1
        vadd.i16        d6,  d23, d26 // p0 + q2
        vadd.i16        d8,  d0,  d0  // 2 * (p3 + p2)
        vadd.i16        d9,  d23, d24 // p0 + q0
        vadd.i16        d8,  d8,  d4  // + p3 + p1
        vsub.i16        d2,  d2,  d0  // p1 + q1 - p3 - p2
        vadd.i16        d8,  d8,  d9  // + p0 + q0
        vsub.i16        d6,  d6,  d4  // p0 + q2 - p3 - p1
        vrshr.u16       d10, d8,  #3  // out p2

        vadd.i16        d8,  d8,  d2
        vadd.i16        d0,  d20, d23 // p3 + p0
        vadd.i16        d2,  d24, d27 // q0 + q3
        vrshr.u16       d11, d8,  #3  // out p1

        vadd.i16        d8,  d8,  d6
        vsub.i16        d2,  d2,  d0  // q0 + q3 - p3 - p0
        vadd.i16        d4,  d21, d24 // p2 + q0
        vadd.i16        d6,  d25, d27 // q1 + q3
        vrshr.u16       d12, d8,  #3  // out p0

        vadd.i16        d8,  d8,  d2
        vsub.i16        d6,  d6,  d4  // q1 + q3 - p2 - q0
        vadd.i16        d0,  d22, d25 // p1 + q1
        vadd.i16        d2,  d26, d27 // q2 + q3
        vrshr.u16       d13, d8,  #3  // out q0

        vadd.i16        d8,  d8,  d6
        vsub.i16        d2,  d2,  d0  // q2 + q3 - p1 - q1
        vrshr.u16       d0,  d8,  #3  // out q1

        vadd.i16        d8,  d8,  d2

        vbit            d21, d10, d14
        vbit            d22, d11, d14
        vbit            d23, d12, d14
        vrshr.u16       d1,  d8,  #3  // out q2
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

        vadd.i16        d2,  d17, d17 // p6 + p6
        vadd.i16        d4,  d17, d18 // p6 + p5
        vadd.i16        d6,  d17, d19 // p6 + p4
        vadd.i16        d8,  d17, d20 // p6 + p3
        vadd.i16        d12, d2,  d4
        vadd.i16        d10, d6,  d8
        vadd.i16        d6,  d17, d21 // p6 + p2
        vadd.i16        d12, d12, d10
        vadd.i16        d8,  d17, d22 // p6 + p1
        vadd.i16        d10, d18, d23 // p5 + p0
        vadd.i16        d6,  d6,  d8
        vadd.i16        d8,  d19, d24 // p4 + q0
        vadd.i16        d12, d12, d6
        vadd.i16        d10, d10, d8
        vadd.i16        d6,  d20, d25 // p3 + q1
        vadd.i16        d12, d12, d10
        vsub.i16        d6,  d6,  d2
        vadd.i16        d2,  d21, d26 // p2 + q2
        vrshr.u16       d0,  d12, #4  // out p5
        vadd.i16        d12, d12, d6  // - (p6 + p6) + (p3 + q1)
        vsub.i16        d2,  d2,  d4
        vadd.i16        d4,  d22, d27 // p1 + q3
        vadd.i16        d6,  d17, d19 // p6 + p4
        vrshr.u16       d1,  d12, #4  // out p4
        vadd.i16        d12, d12, d2  // - (p6 + p5) + (p2 + q2)
        vsub.i16        d4,  d4,  d6
        vadd.i16        d6,  d23, d28 // p0 + q4
        vadd.i16        d8,  d17, d20 // p6 + p3
        vrshr.u16       d2,  d12, #4  // out p3
        vadd.i16        d12, d12, d4  // - (p6 + p4) + (p1 + q3)
        vsub.i16        d6,  d6,  d8
        vadd.i16        d8,  d24, d29 // q0 + q5
        vadd.i16        d4,  d17, d21 // p6 + p2
        vrshr.u16       d3,  d12, #4  // out p2
        vadd.i16        d12, d12, d6  // - (p6 + p3) + (p0 + q4)
        vsub.i16        d8,  d8,  d4
        vadd.i16        d6,  d25, d30 // q1 + q6
        vadd.i16        d10, d17, d22 // p6 + p1
        vrshr.u16       d4,  d12, #4  // out p1
        vadd.i16        d12, d12, d8  // - (p6 + p2) + (q0 + q5)
        vsub.i16        d6,  d6,  d10
        vadd.i16        d8,  d26, d30 // q2 + q6
        vbif            d0,  d18, d15 // out p5
        vadd.i16        d10, d18, d23 // p5 + p0
        vrshr.u16       d5,  d12, #4  // out p0
        vadd.i16        d12, d12, d6  // - (p6 + p1) + (q1 + q6)
        vsub.i16        d8,  d8,  d10
        vadd.i16        d10, d27, d30 // q3 + q6
        vbif            d1,  d19, d15 // out p4
        vadd.i16        d18, d19, d24 // p4 + q0
        vrshr.u16       d6,  d12, #4  // out q0
        vadd.i16        d12, d12, d8  // - (p5 + p0) + (q2 + q6)
        vsub.i16        d10, d10, d18
        vadd.i16        d8,  d28, d30 // q4 + q6
        vbif            d2,  d20, d15 // out p3
        vadd.i16        d18, d20, d25 // p3 + q1
        vrshr.u16       d7,  d12, #4  // out q1
        vadd.i16        d12, d12, d10 // - (p4 + q0) + (q3 + q6)
        vsub.i16        d18, d8,  d18
        vadd.i16        d10, d29, d30 // q5 + q6
        vbif            d3,  d21, d15 // out p2
        vadd.i16        d20, d21, d26 // p2 + q2
        vrshr.u16       d8,  d12, #4  // out q2
        vadd.i16        d12, d12, d18 // - (p3 + q1) + (q4 + q6)
        vsub.i16        d10, d10, d20
        vadd.i16        d18, d30, d30 // q6 + q6
        vbif            d4,  d22, d15 // out p1
        vadd.i16        d20, d22, d27 // p1 + q3
        vrshr.u16       d9,  d12, #4  // out q3
        vadd.i16        d12, d12, d10 // - (p2 + q2) + (q5 + q6)
        vsub.i16        d18, d18, d20
        vbif            d5,  d23, d15 // out p0
        vrshr.u16       d10, d12, #4  // out q4
        vadd.i16        d12, d12, d18 // - (p1 + q3) + (q6 + q6)
        vrshr.u16       d11, d12, #4  // out q5
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
        bx              r6
.endif
.if \wd >= 8
8:
        // Return to a shorter epilogue, writing only the inner 4 pixels
        bx              r7
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

.macro lpf_4_wd16
        adr             r6,  7f + CONFIG_THUMB
        adr             r7,  8f + CONFIG_THUMB
        bl              lpf_4_wd16_neon
.endm

.macro lpf_4_wd8
        adr             r7,  8f + CONFIG_THUMB
        bl              lpf_4_wd8_neon
.endm

.macro lpf_4_wd6
        bl              lpf_4_wd6_neon
.endm

.macro lpf_4_wd4
        bl              lpf_4_wd4_neon
.endm

function lpf_v_4_4_neon
        mov             r12, lr
        sub             r10, r0,  r1, lsl #1
        vld1.16         {d22}, [r10, :64], r1 // p1
        vld1.16         {d24}, [r0,  :64], r1 // q0
        vld1.16         {d23}, [r10, :64], r1 // p0
        vld1.16         {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1

        lpf_4_wd4

        sub             r10, r0,  r1, lsl #1
        vst1.16         {d22}, [r10, :64], r1 // p1
        vst1.16         {d24}, [r0,  :64], r1 // q0
        vst1.16         {d23}, [r10, :64], r1 // p0
        vst1.16         {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_4_4_neon
        mov             r12, lr
        sub             r10, r0,  #4
        add             r0,  r10, r1, lsl #1
        vld1.16         {d22}, [r10], r1
        vld1.16         {d24}, [r0],  r1
        vld1.16         {d23}, [r10], r1
        vld1.16         {d25}, [r0],  r1
        add             r0,  r0,  #4

        transpose_4x4h  q11, q12, d22, d23, d24, d25

        lpf_4_wd4

        sub             r10, r0,  r1, lsl #2
        sub             r10, r10, #4
        transpose_4x4h  q11, q12, d22, d23, d24, d25
        add             r0,  r10, r1, lsl #1

        vst1.16         {d22}, [r10], r1
        vst1.16         {d24}, [r0],  r1
        vst1.16         {d23}, [r10], r1
        vst1.16         {d25}, [r0],  r1
        add             r0,  r0,  #4
        bx              r12
endfunc

function lpf_v_6_4_neon
        mov             r12, lr
        sub             r10, r0,  r1, lsl #1
        sub             r10, r10, r1
        vld1.16         {d21}, [r10, :64], r1 // p2
        vld1.16         {d24}, [r0,  :64], r1 // q0
        vld1.16         {d22}, [r10, :64], r1 // p1
        vld1.16         {d25}, [r0,  :64], r1 // q1
        vld1.16         {d23}, [r10, :64], r1 // p0
        vld1.16         {d26}, [r0,  :64], r1 // q2
        sub             r0,  r0,  r1, lsl #1
        sub             r0,  r0,  r1

        lpf_4_wd6

        sub             r10, r0,  r1, lsl #1
        vst1.16         {d22}, [r10, :64], r1 // p1
        vst1.16         {d24}, [r0,  :64], r1 // q0
        vst1.16         {d23}, [r10, :64], r1 // p0
        vst1.16         {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_6_4_neon
        mov             r12, lr
        sub             r10, r0,  #8
        vld1.16         {d20}, [r10, :64], r1
        vld1.16         {d24}, [r0,  :64], r1
        vld1.16         {d21}, [r10, :64], r1
        vld1.16         {d25}, [r0,  :64], r1
        vld1.16         {d22}, [r10, :64], r1
        vld1.16         {d26}, [r0,  :64], r1
        vld1.16         {d23}, [r10, :64], r1
        vld1.16         {d27}, [r0,  :64], r1

        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q12, q13, d24, d25, d26, d27

        lpf_4_wd6

        sub             r0,  r0,  #4
        transpose_4x4h  q11, q12, d22, d23, d24, d25
        sub             r10, r0,  r1, lsl #2
        sub             r0,  r0,  r1, lsl #1

        vst1.16         {d22}, [r10], r1
        vst1.16         {d24}, [r0],  r1
        vst1.16         {d23}, [r10], r1
        vst1.16         {d25}, [r0],  r1
        add             r0,  r0,  #4
        bx              r12
endfunc

function lpf_v_8_4_neon
        mov             r12, lr
        sub             r10, r0,  r1, lsl #2
        vld1.16         {d20}, [r10, :64], r1 // p3
        vld1.16         {d24}, [r0,  :64], r1 // q0
        vld1.16         {d21}, [r10, :64], r1 // p2
        vld1.16         {d25}, [r0,  :64], r1 // q1
        vld1.16         {d22}, [r10, :64], r1 // p1
        vld1.16         {d26}, [r0,  :64], r1 // q2
        vld1.16         {d23}, [r10, :64], r1 // p0
        vld1.16         {d27}, [r0,  :64], r1 // q3
        sub             r0,  r0,  r1, lsl #2

        lpf_4_wd8

        sub             r10, r0,  r1, lsl #1
        sub             r10, r10, r1
        vst1.16         {d21}, [r10, :64], r1 // p2
        vst1.16         {d24}, [r0,  :64], r1 // q0
        vst1.16         {d22}, [r10, :64], r1 // p1
        vst1.16         {d25}, [r0,  :64], r1 // q1
        vst1.16         {d23}, [r10, :64], r1 // p0
        vst1.16         {d26}, [r0,  :64], r1 // q2
        sub             r0,  r0,  r1, lsl #1
        sub             r0,  r0,  r1
        bx              r12

8:
        sub             r10, r0,  r1, lsl #1
        vst1.16         {d22}, [r10, :64], r1 // p1
        vst1.16         {d24}, [r0,  :64], r1 // q0
        vst1.16         {d23}, [r10, :64], r1 // p0
        vst1.16         {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_8_4_neon
        mov             r12, lr
        sub             r10, r0,  #8
        vld1.16         {d20}, [r10, :64], r1
        vld1.16         {d24}, [r0,  :64], r1
        vld1.16         {d21}, [r10, :64], r1
        vld1.16         {d25}, [r0,  :64], r1
        vld1.16         {d22}, [r10, :64], r1
        vld1.16         {d26}, [r0,  :64], r1
        vld1.16         {d23}, [r10, :64], r1
        vld1.16         {d27}, [r0,  :64], r1

        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q12, q13, d24, d25, d26, d27

        lpf_4_wd8

        sub             r0,  r0,  r1, lsl #2
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q12, q13, d24, d25, d26, d27
        sub             r10, r0,  #8

        vst1.16         {d20}, [r10, :64], r1
        vst1.16         {d24}, [r0,  :64], r1
        vst1.16         {d21}, [r10, :64], r1
        vst1.16         {d25}, [r0,  :64], r1
        vst1.16         {d22}, [r10, :64], r1
        vst1.16         {d26}, [r0,  :64], r1
        vst1.16         {d23}, [r10, :64], r1
        vst1.16         {d27}, [r0,  :64], r1
        bx              r12
8:
        sub             r0,  r0,  #4
        transpose_4x4h  q11, q12, d22, d23, d24, d25
        sub             r10, r0,  r1, lsl #2
        sub             r0,  r0,  r1, lsl #1

        vst1.16         {d22}, [r10], r1
        vst1.16         {d24}, [r0],  r1
        vst1.16         {d23}, [r10], r1
        vst1.16         {d25}, [r0],  r1
        add             r0,  r0,  #4
        bx              r12
endfunc

function lpf_v_16_4_neon
        mov             r12, lr

        sub             r10, r0,  r1, lsl #3
        add             r10, r10, r1
        vld1.16         {d17}, [r10, :64], r1 // p6
        vld1.16         {d24}, [r0,  :64], r1 // q0
        vld1.16         {d18}, [r10, :64], r1 // p5
        vld1.16         {d25}, [r0,  :64], r1 // q1
        vld1.16         {d19}, [r10, :64], r1 // p4
        vld1.16         {d26}, [r0,  :64], r1 // q2
        vld1.16         {d20}, [r10, :64], r1 // p3
        vld1.16         {d27}, [r0,  :64], r1 // q3
        vld1.16         {d21}, [r10, :64], r1 // p2
        vld1.16         {d28}, [r0,  :64], r1 // q4
        vld1.16         {d22}, [r10, :64], r1 // p1
        vld1.16         {d29}, [r0,  :64], r1 // q5
        vld1.16         {d23}, [r10, :64], r1 // p0
        vld1.16         {d30}, [r0,  :64], r1 // q6
        sub             r0,  r0,  r1, lsl #3
        add             r0,  r0,  r1

        lpf_4_wd16

        sub             r10, r0,  r1, lsl #2
        sub             r10, r10, r1, lsl #1
        vst1.16         {d0},  [r10, :64], r1 // p5
        vst1.16         {d6},  [r0,  :64], r1 // q0
        vst1.16         {d1},  [r10, :64], r1 // p4
        vst1.16         {d7},  [r0,  :64], r1 // q1
        vst1.16         {d2},  [r10, :64], r1 // p3
        vst1.16         {d8},  [r0,  :64], r1 // q2
        vst1.16         {d3},  [r10, :64], r1 // p2
        vst1.16         {d9},  [r0,  :64], r1 // q3
        vst1.16         {d4},  [r10, :64], r1 // p1
        vst1.16         {d10}, [r0,  :64], r1 // q4
        vst1.16         {d5},  [r10, :64], r1 // p0
        vst1.16         {d11}, [r0,  :64], r1 // q5
        sub             r0,  r0,  r1, lsl #2
        sub             r0,  r0,  r1, lsl #1
        bx              r12
7:
        sub             r10, r0,  r1
        sub             r10, r10, r1, lsl #1
        vst1.16         {d21}, [r10, :64], r1 // p2
        vst1.16         {d24}, [r0,  :64], r1 // q0
        vst1.16         {d22}, [r10, :64], r1 // p1
        vst1.16         {d25}, [r0,  :64], r1 // q1
        vst1.16         {d23}, [r10, :64], r1 // p0
        vst1.16         {d26}, [r0,  :64], r1 // q2
        sub             r0,  r0,  r1, lsl #1
        sub             r0,  r0,  r1
        bx              r12

8:
        sub             r10, r0,  r1, lsl #1
        vst1.16         {d22}, [r10, :64], r1 // p1
        vst1.16         {d24}, [r0,  :64], r1 // q0
        vst1.16         {d23}, [r10, :64], r1 // p0
        vst1.16         {d25}, [r0,  :64], r1 // q1
        sub             r0,  r0,  r1, lsl #1
        bx              r12
endfunc

function lpf_h_16_4_neon
        mov             r12, lr
        sub             r10, r0,  #16
        sub             r0,  r0,  #8
        vld1.16         {d16}, [r10, :64], r1
        vld1.16         {d20}, [r0,  :64], r1
        vld1.16         {d17}, [r10, :64], r1
        vld1.16         {d21}, [r0,  :64], r1
        vld1.16         {d18}, [r10, :64], r1
        vld1.16         {d22}, [r0,  :64], r1
        vld1.16         {d19}, [r10, :64], r1
        vld1.16         {d23}, [r0,  :64], r1
        sub             r10, r10, r1, lsl #2
        sub             r0,  r0,  r1, lsl #2
        add             r10, r10, #16
        add             r0,  r0,  #16
        vld1.16         {d24}, [r10, :64], r1
        vld1.16         {d28}, [r0,  :64], r1
        vld1.16         {d25}, [r10, :64], r1
        vld1.16         {d29}, [r0,  :64], r1
        vld1.16         {d26}, [r10, :64], r1
        vld1.16         {d30}, [r0,  :64], r1
        vld1.16         {d27}, [r10, :64], r1
        vld1.16         {d31}, [r0,  :64], r1
        sub             r0,  r0,  #8

        transpose_4x4h  q8,  q9,  d16, d17, d18, d19
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q12, q13, d24, d25, d26, d27
        transpose_4x4h  q14, q15, d28, d29, d30, d31

        lpf_4_wd16

        sub             r0,  r0,  r1, lsl #2
        transpose_4x4h  q8,  q0,  d16, d17, d0,  d1
        transpose_4x4h  q1,  q2,  d2,  d3,  d4,  d5
        transpose_4x4h  q3,  q4,  d6,  d7,  d8,  d9
        transpose_4x4h  q5,  q15, d10, d11, d30, d31
        sub             r10, r0,  #16
        sub             r0,  r0,  #8

        vst1.16         {d16}, [r10, :64], r1
        vst1.16         {d2},  [r0,  :64], r1
        vst1.16         {d17}, [r10, :64], r1
        vst1.16         {d3},  [r0,  :64], r1
        vst1.16         {d0},  [r10, :64], r1
        vst1.16         {d4},  [r0,  :64], r1
        vst1.16         {d1},  [r10, :64], r1
        vst1.16         {d5},  [r0,  :64], r1
        sub             r10, r10, r1, lsl #2
        sub             r0,  r0,  r1, lsl #2
        add             r10, r10, #16
        add             r0,  r0,  #16
        vst1.16         {d6},  [r10, :64], r1
        vst1.16         {d10}, [r0,  :64], r1
        vst1.16         {d7},  [r10, :64], r1
        vst1.16         {d11}, [r0,  :64], r1
        vst1.16         {d8},  [r10, :64], r1
        vst1.16         {d30}, [r0,  :64], r1
        vst1.16         {d9},  [r10, :64], r1
        vst1.16         {d31}, [r0,  :64], r1
        sub             r0,  r0,  #8

        bx              r12

7:
        sub             r0,  r0,  r1, lsl #2
        transpose_4x4h  q10, q11, d20, d21, d22, d23
        transpose_4x4h  q12, q13, d24, d25, d26, d27
        sub             r10, r0,  #8

        vst1.16         {d20}, [r10, :64], r1
        vst1.16         {d24}, [r0,  :64], r1
        vst1.16         {d21}, [r10, :64], r1
        vst1.16         {d25}, [r0,  :64], r1
        vst1.16         {d22}, [r10, :64], r1
        vst1.16         {d26}, [r0,  :64], r1
        vst1.16         {d23}, [r10, :64], r1
        vst1.16         {d27}, [r0,  :64], r1
        bx              r12
8:
        sub             r0,  r0,  #4
        transpose_4x4h  q11, q12, d22, d23, d24, d25
        sub             r10, r0,  r1, lsl #2
        sub             r0,  r0,  r1, lsl #1

        vst1.16         {d22}, [r10], r1
        vst1.16         {d24}, [r0],  r1
        vst1.16         {d23}, [r10], r1
        vst1.16         {d25}, [r0],  r1
        add             r0,  r0,  #4
        bx              r12
endfunc

// void dav1d_lpf_v_sb_y_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                  const uint32_t *const vmask,
//                                  const uint8_t (*l)[4], ptrdiff_t b4_stride,
//                                  const Av1FilterLUT *lut, const int w,
//                                  const int bitdepth_max)

.macro lpf_func dir, type
function lpf_\dir\()_sb_\type\()_16bpc_neon, export=1
        push            {r4-r11,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #100]
        ldr             r8,  [sp,  #112] // bitdepth_max; the 'w' parameter isn't loaded
        sub             sp,  sp,  #8
        clz             r9,  r8
        rsb             r9,  r9,  #24  // bitdepth_min_8
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
        tst             r6,  #0x01
        strd            r6,  r7,  [sp]
.ifc \dir, v
        ldrb            r10, [r4], #4
        ldrb            r11, [r3], #4
.else
        ldrb            r10, [r3]
        ldrb            r11, [r3, #4]
        add             r3,  r3,  r4
.endif
        beq             7f             // if (!(vm & bits)) continue;

        orrs            r12, r10, r11
        vdup.16         d31, r9        // bitdepth_min_8
        beq             7f             // if (!(l[0][0] | l[offset][0])) continue;
        cmp             r11, #0        // Check for nonzero values in l[0][0]
        ldrb            r6,  [r5], #8  // sharp[0]
        it              eq
        moveq           r11, r10       // if (!l[0][0]) L = l[offset][0]
        ldrb            r12, [r5]      // sharp[1]
        lsr             r6,  r11, r6   // L >> sharp[0]
        sub             r5,  r5,  #8
        cmp             r12, r6
        lsr             r10, r11, #4   // H
        add             r11, r11, #2   // L + 2
        it              lt
        movlt           r6,  r12       // imin(L >> sharp[0], sharp[1])
        add             r11, r11, r11  // 2*(L + 2)
        cmp             r6,  #1
        lsl             r10, r10, r9   // H << bitdepth_min_8
        it              lt
        movlt           r6,  #1        // imax(imin(), 1) = limit = I
        vdup.16         d12, r10       // H << bitdepth_min_8
        add             r11, r11, r6   // 2*(L + 2) + limit = E
        lsl             r6,  r6,  r9   // I << bitdepth_min_8
        lsl             r11, r11, r9   // E << bitdepth_min_8
        vdup.16         d11, r6        // I << bitdepth_min_8
        vdup.16         d10, r11       // E << bitdepth_min_8

.ifc \type, y
        tst             r2,  #0x01
        beq             2f
        // wd16
        bl              lpf_\dir\()_16_4_neon
        b               8f
2:
.endif
        tst             r7,  #0x01
        beq             3f
.ifc \type, y
        // wd8
        bl              lpf_\dir\()_8_4_neon
.else
        // wd6
        bl              lpf_\dir\()_6_4_neon
.endif
        b               8f
3:
        // wd4
        bl              lpf_\dir\()_4_4_neon
.ifc \dir, h
        b               8f
7:
        // For dir h, the functions above increment r0.
        // If the whole function is skipped, increment it here instead.
        add             r0,  r0,  r1,  lsl #2
.else
7:
.endif
8:
        ldrd            r6,  r7,  [sp]
.ifc \type, y
        lsr             r2,  r2,  #1   // vmask[2] >>= 1
.endif
.ifc \dir, v
        add             r0,  r0,  #8
.else
        // For dir h, r0 is returned incremented
.endif
        lsrs            r6,  r6,  #1   // vmask[0] >>= 1
        lsr             r7,  r7,  #1   // vmask[1] >>= 1
        bne             1b

        add             sp,  sp,  #8
        vpop            {q4-q7}
        pop             {r4-r11,pc}
endfunc
.endm

lpf_func v, y
lpf_func h, y
lpf_func v, uv
lpf_func h, uv
