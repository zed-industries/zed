/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2019, B Krishnan Iyer
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

// void ipred_dc_128_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int a,
//                              const int max_width, const int max_height,
//                              const int bitdepth_max);
function ipred_dc_128_16bpc_neon, export=1
        push            {r4, lr}
        ldr             r4,  [sp, #8]
        ldr             r12, [sp, #24]
        clz             r3,  r3
        adr             r2,  L(ipred_dc_128_tbl)
        sub             r3,  r3,  #25
        vdup.16         q0,  r12
        ldr             r3,  [r2,  r3,  lsl #2]
        add             r12, r0,  r1
        vrshr.u16       q0,  q0,  #1
        add             r2,  r2,  r3
        lsl             r1,  r1,  #1
        bx              r2

        .align 2
L(ipred_dc_128_tbl):
        .word 640f - L(ipred_dc_128_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_dc_128_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_dc_128_tbl) + CONFIG_THUMB
        .word 8f   - L(ipred_dc_128_tbl) + CONFIG_THUMB
        .word 4f   - L(ipred_dc_128_tbl) + CONFIG_THUMB
4:
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        bgt             4b
        pop             {r4, pc}
8:
        vst1.16         {d0,  d1}, [r0,  :128], r1
        vst1.16         {d0,  d1}, [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1}, [r0,  :128], r1
        vst1.16         {d0,  d1}, [r12, :128], r1
        bgt             8b
        pop             {r4, pc}
160:
        vmov            q1,  q0
16:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             16b
        pop             {r4, pc}
320:
        vmov            q1,  q0
        sub             r1,  r1,  #32
32:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             32b
        pop             {r4, pc}
640:
        vmov            q1,  q0
        sub             r1,  r1,  #96
64:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        subs            r4,  r4,  #2
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             64b
        pop             {r4, pc}
endfunc

// void ipred_v_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                         const pixel *const topleft,
//                         const int width, const int height, const int a,
//                         const int max_width, const int max_height);
function ipred_v_16bpc_neon, export=1
        push            {r4, lr}
        ldr             lr,  [sp, #8]
        clz             r3,  r3
        adr             r4,  L(ipred_v_tbl)
        sub             r3,  r3,  #25
        ldr             r3,  [r4,  r3,  lsl #2]
        add             r2,  r2,  #2
        add             r4,  r4,  r3
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        bx              r4

        .align 2
L(ipred_v_tbl):
        .word 640f - L(ipred_v_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_v_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_v_tbl) + CONFIG_THUMB
        .word 80f  - L(ipred_v_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_v_tbl) + CONFIG_THUMB

40:
        vld1.16         {d0}, [r2]
4:
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        subs            lr,  lr,  #4
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        bgt             4b
        pop             {r4, pc}
80:
        vld1.16         {q0},  [r2]
8:
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        subs            lr,  lr,  #4
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        bgt             8b
        pop             {r4, pc}
160:
        vld1.16         {q0,  q1},  [r2]
16:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            lr,  lr,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             16b
        pop             {r4, pc}
320:
        vld1.16         {q0,  q1},  [r2]!
        sub             r1,  r1,  #32
        vld1.16         {q2,  q3},  [r2]
32:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d4,  d5,  d6,  d7},  [r0,  :128], r1
        vst1.16         {d4,  d5,  d6,  d7},  [r12, :128], r1
        subs            lr,  lr,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d4,  d5,  d6,  d7},  [r0,  :128], r1
        vst1.16         {d4,  d5,  d6,  d7},  [r12, :128], r1
        bgt             32b
        pop             {r4, pc}
640:
        vld1.16         {q0,  q1},  [r2]!
        sub             r1,  r1,  #96
        vld1.16         {q2,  q3},  [r2]!
        vld1.16         {q8,  q9},  [r2]!
        vld1.16         {q10, q11}, [r2]!
64:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d4,  d5,  d6,  d7},  [r0,  :128]!
        vst1.16         {d4,  d5,  d6,  d7},  [r12, :128]!
        subs            lr,  lr,  #2
        vst1.16         {d16, d17, d18, d19}, [r0,  :128]!
        vst1.16         {d16, d17, d18, d19}, [r12, :128]!
        vst1.16         {d20, d21, d22, d23}, [r0,  :128], r1
        vst1.16         {d20, d21, d22, d23}, [r12, :128], r1
        bgt             64b
        pop             {r4, pc}
endfunc

// void ipred_h_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                         const pixel *const topleft,
//                         const int width, const int height, const int a,
//                         const int max_width, const int max_height);
function ipred_h_16bpc_neon, export=1
        push            {r4-r5, lr}
        ldr             r4,  [sp, #12]
        clz             r3,  r3
        adr             r5,  L(ipred_h_tbl)
        sub             r3,  r3,  #25
        ldr             r3,  [r5,  r3,  lsl #2]
        sub             r2,  r2,  #2
        mov             lr,  #-2
        add             r5,  r5,  r3
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        bx              r5

        .align 2
L(ipred_h_tbl):
        .word 640f - L(ipred_h_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_h_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_h_tbl) + CONFIG_THUMB
        .word 8f   - L(ipred_h_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_h_tbl) + CONFIG_THUMB
40:
        sub             r2,  r2,  #6
        mov             lr,  #-8
4:
        vld4.16         {d0[],  d1[],  d2[],  d3[]},  [r2],  lr
        vst1.16         {d3},  [r0,  :64], r1
        vst1.16         {d2},  [r12, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d1},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        bgt             4b
        pop             {r4-r5, pc}
8:
        vld1.16         {d0[],  d1[]},  [r2],  lr
        subs            r4,  r4,  #4
        vld1.16         {d2[],  d3[]},  [r2],  lr
        vst1.16         {q0},  [r0,    :128],  r1
        vld1.16         {d4[],  d5[]},  [r2],  lr
        vst1.16         {q1},  [r12,   :128],  r1
        vld1.16         {d6[],  d7[]},  [r2],  lr
        vst1.16         {q2},  [r0,    :128],  r1
        vst1.16         {q3},  [r12,   :128],  r1
        bgt             8b
        pop             {r4-r5, pc}
160:
        sub             r1,  r1,  #16
16:
        vld1.16         {d0[],  d1[]}, [r2],  lr
        subs            r4,  r4,  #4
        vld1.16         {d2[],  d3[]}, [r2],  lr
        vst1.16         {q0},  [r0,   :128]!
        vld1.16         {d4[],  d5[]}, [r2],  lr
        vst1.16         {q1},  [r12,  :128]!
        vld1.16         {d6[],  d7[]}, [r2],  lr
        vst1.16         {q0},  [r0,   :128],  r1
        vst1.16         {q1},  [r12,  :128],  r1
        vst1.16         {q2},  [r0,   :128]!
        vst1.16         {q3},  [r12,  :128]!
        vst1.16         {q2},  [r0,   :128],  r1
        vst1.16         {q3},  [r12,  :128],  r1
        bgt             16b
        pop             {r4-r5, pc}
320:
        sub             r1,  r1,  #48
32:
        vld1.16         {d0[],  d1[]},  [r2],  lr
        subs            r4,  r4,  #4
        vld1.16         {d2[],  d3[]},  [r2],  lr
        vst1.16         {q0},  [r0,    :128]!
        vld1.16         {d4[],  d5[]},  [r2],  lr
        vst1.16         {q1},  [r12,   :128]!
        vld1.16         {d6[],  d7[]},  [r2],  lr
        vst1.16         {q0},  [r0,    :128]!
        vst1.16         {q1},  [r12,   :128]!
        vst1.16         {q0},  [r0,    :128]!
        vst1.16         {q1},  [r12,   :128]!
        vst1.16         {q0},  [r0,    :128],  r1
        vst1.16         {q1},  [r12,   :128],  r1
        vst1.16         {q2},  [r0,    :128]!
        vst1.16         {q3},  [r12,   :128]!
        vst1.16         {q2},  [r0,    :128]!
        vst1.16         {q3},  [r12,   :128]!
        vst1.16         {q2},  [r0,    :128]!
        vst1.16         {q3},  [r12,   :128]!
        vst1.16         {q2},  [r0,    :128],  r1
        vst1.16         {q3},  [r12,   :128],  r1
        bgt             32b
        pop             {r4-r5, pc}
640:
        sub             r1,  r1,  #96
64:
        vld1.16         {d0[],  d1[]},  [r2],  lr
        subs            r4,  r4,  #2
        vld1.16         {d4[],  d5[]},  [r2],  lr
        vmov            q1,  q0
        vmov            q3,  q2
        vst1.16         {q0, q1}, [r0,  :128]!
        vst1.16         {q2, q3}, [r12, :128]!
        vst1.16         {q0, q1}, [r0,  :128]!
        vst1.16         {q2, q3}, [r12, :128]!
        vst1.16         {q0, q1}, [r0,  :128]!
        vst1.16         {q2, q3}, [r12, :128]!
        vst1.16         {q0, q1}, [r0,  :128],  r1
        vst1.16         {q2, q3}, [r12, :128],  r1
        bgt             64b
        pop             {r4-r5, pc}
endfunc

// void ipred_dc_top_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int a,
//                              const int max_width, const int max_height);
function ipred_dc_top_16bpc_neon, export=1
        push            {r4-r5, lr}
        ldr             r4,  [sp, #12]
        clz             r3,  r3
        adr             r5,  L(ipred_dc_top_tbl)
        sub             r3,  r3,  #25
        ldr             r3,  [r5,  r3,  lsl #2]
        add             r2,  r2,  #2
        add             r5,  r5,  r3
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        bx              r5

        .align 2
L(ipred_dc_top_tbl):
        .word 640f - L(ipred_dc_top_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_dc_top_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_dc_top_tbl) + CONFIG_THUMB
        .word 80f  - L(ipred_dc_top_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_dc_top_tbl) + CONFIG_THUMB

40:
        vld1.16         {d0},  [r2]
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #2
        vdup.16         d0,  d0[0]
4:
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        bgt             4b
        pop             {r4-r5, pc}
80:
        vld1.16         {d0,  d1},  [r2]
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #3
        vdup.16         q0,  d0[0]
8:
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        bgt             8b
        pop             {r4-r5, pc}
160:
        vld1.16         {d0,  d1,  d2,  d3},  [r2]
        vadd.i16        q0,  q0,  q1
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d4,  d0,  #4
        vdup.16         q0,  d4[0]
        vdup.16         q1,  d4[0]
16:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             16b
        pop             {r4-r5, pc}
320:
        vld1.16         {d0,  d1,  d2,  d3},  [r2]!
        vld1.16         {d4,  d5,  d6,  d7},  [r2]
        vadd.i16        q0,  q0,  q1
        vadd.i16        q2,  q2,  q3
        vadd.i16        q0,  q0,  q2
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpaddl.u16      d0,  d0
        vrshrn.i32      d18, q0,  #5
        vdup.16         q0,  d18[0]
        vdup.16         q1,  d18[0]
        sub             r1,  r1,  #32
32:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             32b
        pop             {r4-r5, pc}
640:
        vld1.16         {d0,  d1,  d2,  d3},  [r2]!
        vld1.16         {d4,  d5,  d6,  d7},  [r2]!
        vadd.i16        q0,  q0,  q1
        vld1.16         {d16, d17, d18, d19}, [r2]!
        vadd.i16        q2,  q2,  q3
        vld1.16         {d20, d21, d22, d23}, [r2]
        vadd.i16        q8,  q8,  q9
        vadd.i16        q10, q10, q11
        vadd.i16        q0,  q0,  q2
        vadd.i16        q8,  q8,  q10
        vadd.i16        q0,  q0,  q8
        vadd.i16        d0,  d0,  d1
        vpaddl.u16      d0,  d0
        vpadd.i32       d0,  d0,  d0
        vrshrn.i32      d18, q0,  #6
        vdup.16         q0,  d18[0]
        vdup.16         q1,  d18[0]
        sub             r1,  r1,  #96
64:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        subs            r4,  r4,  #2
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             64b
        pop             {r4-r5, pc}
endfunc

// void ipred_dc_left_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height, const int a,
//                               const int max_width, const int max_height);
function ipred_dc_left_16bpc_neon, export=1
        push            {r4-r5, lr}
        ldr             r4,  [sp, #12]
        sub             r2,  r2,  r4,  lsl #1
        clz             r3,  r3
        clz             lr,  r4
        sub             lr,  lr,  #25
        adr             r5,  L(ipred_dc_left_tbl)
        sub             r3,  r3,  #20
        ldr             r3,  [r5,  r3,  lsl #2]
        ldr             lr,  [r5,  lr,  lsl #2]
        add             r3,  r5,  r3
        add             r5,  r5,  lr
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        bx              r5

        .align 2
L(ipred_dc_left_tbl):
        .word L(ipred_dc_left_h64) - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_h32) - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_h16) - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_h8)  - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_h4)  - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_w64) - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_w32) - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_w16) - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_w8)  - L(ipred_dc_left_tbl) + CONFIG_THUMB
        .word L(ipred_dc_left_w4)  - L(ipred_dc_left_tbl) + CONFIG_THUMB

L(ipred_dc_left_h4):
        vld1.16         {d0},  [r2, :64]
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #2
        vdup.16         q0,  d0[0]
        bx              r3
L(ipred_dc_left_w4):
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        bgt             L(ipred_dc_left_w4)
        pop             {r4-r5, pc}
L(ipred_dc_left_h8):
        vld1.16         {d0,  d1},  [r2, :128]
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #3
        vdup.16         q0,  d0[0]
        bx              r3
L(ipred_dc_left_w8):
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        bgt             L(ipred_dc_left_w8)
        pop             {r4-r5, pc}
L(ipred_dc_left_h16):
        vld1.16         {d0,  d1,  d2,  d3},  [r2, :128]
        vadd.i16        q0,  q0,  q1
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #4
        vdup.16         q0,  d0[0]
        bx              r3
L(ipred_dc_left_w16):
        vmov            q1,  q0
1:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             1b
        pop             {r4-r5, pc}
L(ipred_dc_left_h32):
        vld1.16         {d0,  d1,  d2,  d3},  [r2, :128]!
        vld1.16         {d4,  d5,  d6,  d7},  [r2, :128]
        vadd.i16        q0,  q0,  q1
        vadd.i16        q2,  q2,  q3
        vadd.i16        q0,  q0,  q2
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpaddl.u16      d0,  d0
        vrshrn.i32      d0,  q0,  #5
        vdup.16         q0,  d0[0]
        bx              r3
L(ipred_dc_left_w32):
        sub             r1,  r1,  #32
        vmov            q1,  q0
1:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             1b
        pop             {r4-r5, pc}
L(ipred_dc_left_h64):
        vld1.16         {d0,  d1,  d2,  d3},  [r2, :128]!
        vld1.16         {d4,  d5,  d6,  d7},  [r2, :128]!
        vadd.i16        q0,  q0,  q1
        vld1.16         {d16, d17, d18, d19}, [r2, :128]!
        vadd.i16        q2,  q2,  q3
        vld1.16         {d20, d21, d22, d23}, [r2, :128]
        vadd.i16        q8,  q8,  q9
        vadd.i16        q10, q10, q11
        vadd.i16        q0,  q0,  q2
        vadd.i16        q8,  q8,  q10
        vadd.i16        q0,  q0,  q8
        vadd.i16        d0,  d0,  d1
        vpaddl.u16      d0,  d0
        vpadd.i32       d0,  d0,  d0
        vrshrn.i32      d0,  q0,  #6
        vdup.16         q0,  d0[0]
        bx              r3
L(ipred_dc_left_w64):
        sub             r1,  r1,  #96
        vmov            q1,  q0
1:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        subs            r4,  r4,  #2
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             1b
        pop             {r4-r5, pc}
endfunc

// void ipred_dc_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                          const pixel *const topleft,
//                          const int width, const int height, const int a,
//                          const int max_width, const int max_height);
function ipred_dc_16bpc_neon, export=1
        push            {r4-r6, lr}
        ldr             r4,  [sp, #16]
        sub             r2,  r2,  r4,  lsl #1
        add             lr,  r3,  r4        // width + height
        clz             r3,  r3
        clz             r12, r4
        vdup.32         q15, lr             // width + height
        adr             r5,  L(ipred_dc_tbl)
        rbit            lr,  lr             // rbit(width + height)
        sub             r3,  r3,  #20       // 25 leading bits, minus table offset 5
        sub             r12, r12, #25
        clz             lr,  lr             // ctz(width + height)
        ldr             r3,  [r5,  r3,  lsl #2]
        ldr             r12, [r5,  r12, lsl #2]
        neg             lr,  lr             // -ctz(width + height)
        add             r3,  r5,  r3
        add             r5,  r5,  r12
        vshr.u32        q15, q15, #1        // (width + height) >> 1
        vdup.32         q14, lr             // -ctz(width + height)
        add             r12, r0,  r1
        lsl             r1,  r1,  #1
        bx              r5

        .align 2
L(ipred_dc_tbl):
        .word L(ipred_dc_h64) - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_h32) - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_h16) - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_h8)  - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_h4)  - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_w64) - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_w32) - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_w16) - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_w8)  - L(ipred_dc_tbl) + CONFIG_THUMB
        .word L(ipred_dc_w4)  - L(ipred_dc_tbl) + CONFIG_THUMB

L(ipred_dc_h4):
        vld1.16         {d0},  [r2, :64]!
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r3
L(ipred_dc_w4):
        vld1.16         {d2},  [r2]
        vadd.i32        d0,  d0,  d30
        vpadd.i16       d2,  d2,  d2
        vpaddl.u16      d2,  d2
        cmp             r4,  #4
        vadd.i32        d0,  d0,  d2
        vshl.u32        d0,  d0,  d28
        beq             1f
        // h = 8/16
        cmp             r4,  #16
        movw            lr,  #0x6667
        movw            r5,  #0xAAAB
        it              ne
        movne           lr,  r5
        vdup.32         d24, lr
        vmul.i32        d0,  d0,  d24
        vshr.u32        d0,  d0,  #17
1:
        vdup.16         d0,  d0[0]
2:
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d0},  [r0,  :64], r1
        vst1.16         {d0},  [r12, :64], r1
        bgt             2b
        pop             {r4-r6, pc}

L(ipred_dc_h8):
        vld1.16         {d0,  d1},  [r2, :128]!
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r3
L(ipred_dc_w8):
        vld1.16         {d2,  d3},  [r2]
        vadd.i32        d0,  d0,  d30
        vadd.i16        d2,  d2,  d3
        vpadd.i16       d2,  d2,  d2
        vpaddl.u16      d2,  d2
        cmp             r4,  #8
        vadd.i32        d0,  d0,  d2
        vshl.u32        d0,  d0,  d28
        beq             1f
        // h = 4/16/32
        cmp             r4,  #32
        movw            lr,  #0x6667
        movw            r5,  #0xAAAB
        it              ne
        movne           lr,  r5
        vdup.32         d24, lr
        vmul.i32        d0,  d0,  d24
        vshr.u32        d0,  d0,  #17
1:
        vdup.16         q0,  d0[0]
2:
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1},  [r0,  :128], r1
        vst1.16         {d0,  d1},  [r12, :128], r1
        bgt             2b
        pop             {r4-r6, pc}

L(ipred_dc_h16):
        vld1.16         {d0,  d1,  d2,  d3},  [r2, :128]!
        vadd.i16        q0,  q0,  q1
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r3
L(ipred_dc_w16):
        vld1.16         {d2,  d3,  d4,  d5},  [r2]
        vadd.i32        d0,  d0,  d30
        vadd.i16        q1,  q1,  q2
        vadd.i16        d2,  d2,  d3
        vpadd.i16       d2,  d2,  d1
        vpaddl.u16      d2,  d2
        cmp             r4,  #16
        vadd.i32        d0,  d0,  d2
        vshl.u32        d4,  d0,  d28
        beq             1f
        // h = 4/8/32/64
        tst             r4,  #(32+16+8)     // 16 added to make a consecutive bitmask
        movw            lr,  #0x6667
        movw            r5,  #0xAAAB
        it              ne
        movne           lr,  r5
        vdup.32         d24, lr
        vmul.i32        d4,  d4,  d24
        vshr.u32        d4,  d4,  #17
1:
        vdup.16         q0,  d4[0]
        vdup.16         q1,  d4[0]
2:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             2b
        pop             {r4-r6, pc}

L(ipred_dc_h32):
        vld1.16         {d0,  d1,  d2,  d3},  [r2, :128]!
        vld1.16         {d4,  d5,  d6,  d7},  [r2, :128]!
        vadd.i16        q0,  q0,  q1
        vadd.i16        q2,  q2,  q3
        vadd.i16        q0,  q0,  q2
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r3
L(ipred_dc_w32):
        vld1.16         {d2,  d3,  d4,  d5},  [r2]!
        vadd.i32        d0,  d0,  d30
        vld1.16         {d16, d17, d18, d19}, [r2]
        vadd.i16        q1,  q1,  q2
        vadd.i16        q8,  q8,  q9
        vadd.i16        q1,  q1,  q8
        vadd.i16        d2,  d2,  d3
        vpadd.i16       d2,  d2,  d2
        vpaddl.u16      d2,  d2
        cmp             r4,  #32
        vadd.i32        d0,  d0,  d2
        vshl.u32        d4,  d0,  d28
        beq             1f
        // h = 8/16/64
        cmp             r4,  #8
        movw            lr,  #0x6667
        movw            r5,  #0xAAAB
        it              ne
        movne           lr,  r5
        vdup.32         d24, lr
        vmul.i32        d4,  d4,  d24
        vshr.u32        d4,  d4,  #17
1:
        sub             r1,  r1,  #32
        vdup.16         q0,  d4[0]
        vdup.16         q1,  d4[0]
2:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             2b
        pop             {r4-r6, pc}
L(ipred_dc_h64):
        vld1.16         {d0,  d1,  d2,  d3},  [r2, :128]!
        vld1.16         {d4,  d5,  d6,  d7},  [r2, :128]!
        vadd.i16        q0,  q0,  q1
        vld1.16         {d16, d17, d18, d19}, [r2, :128]!
        vadd.i16        q2,  q2,  q3
        vld1.16         {d20, d21, d22, d23}, [r2, :128]!
        vadd.i16        q8,  q8,  q9
        vadd.i16        q10, q10, q11
        vadd.i16        q0,  q0,  q2
        vadd.i16        q8,  q8,  q10
        vadd.i16        q0,  q0,  q8
        vadd.i16        d0,  d0,  d1
        vpaddl.u16      d0,  d0
        add             r2,  r2,  #2
        vpadd.i32       d0,  d0,  d0
        bx              r3
L(ipred_dc_w64):
        vld1.16         {d2,  d3,  d4,  d5},  [r2]!
        vadd.i32        d0,  d0,  d30
        vld1.16         {d16, d17, d18, d19}, [r2]!
        vadd.i16        q1,  q1,  q2
        vld1.16         {d20, d21, d22, d23}, [r2]!
        vadd.i16        q8,  q8,  q9
        vld1.16         {d24, d25, d26, d27}, [r2]!
        vadd.i16        q10, q10, q11
        vadd.i16        q12, q12, q13
        vadd.i16        q1,  q1,  q8
        vadd.i16        q10, q10, q12
        vadd.i16        q1,  q1,  q10
        vadd.i16        d2,  d2,  d3
        vpaddl.u16      d2,  d2
        vpadd.i32       d2,  d2,  d2
        cmp             r4,  #64
        vadd.i32        d0,  d0,  d2
        vshl.u32        d4,  d0,  d28
        beq             1f
        // h = 16/32
        cmp             r4,  #16
        movw            lr,  #0x6667
        movw            r5,  #0xAAAB
        it              ne
        movne           lr,  r5
        vdup.32         d24, lr
        vmul.i32        d4,  d4,  d24
        vshr.u32        d4,  d4,  #17
1:
        sub             r1,  r1,  #96
        vdup.16         q0,  d4[0]
        vdup.16         q1,  d4[0]
2:
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        subs            r4,  r4,  #2
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128]!
        vst1.16         {d0,  d1,  d2,  d3},  [r0,  :128], r1
        vst1.16         {d0,  d1,  d2,  d3},  [r12, :128], r1
        bgt             2b
        pop             {r4-r6, pc}
endfunc

// void ipred_paeth_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                             const pixel *const topleft,
//                             const int width, const int height, const int a,
//                             const int max_width, const int max_height);
function ipred_paeth_16bpc_neon, export=1
        push            {r4-r6, lr}
        vpush           {q4}
        ldr             r4,  [sp, #32]
        clz             lr,  r3
        adr             r12, L(ipred_paeth_tbl)
        sub             lr,  lr,  #25
        ldr             lr,  [r12, lr, lsl #2]
        vld1.16         {d4[], d5[]},  [r2]
        add             r6,  r2,  #2
        sub             r2,  r2,  #4
        add             r12, r12, lr
        mov             r5,  #-4
        add             lr,  r0,  r1
        lsl             r1,  r1,  #1
        bx              r12

        .align 2
L(ipred_paeth_tbl):
        .word 640f - L(ipred_paeth_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_paeth_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_paeth_tbl) + CONFIG_THUMB
        .word 80f  - L(ipred_paeth_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_paeth_tbl) + CONFIG_THUMB

40:
        sub             r2,  r2,  #4
        mov             r5,  #-8
        vld1.16         {d6},  [r6]
        vsub.i16        d16, d6,  d4  // top - topleft
        vmov            d7,  d6
        vmov            d17, d16
4:
        vld4.16         {d0[], d1[], d2[], d3[]},  [r2, :64], r5
        vadd.i16        q9,  q8,  q0  // base
        vadd.i16        q10, q8,  q1
        vabd.s16        q11, q3,  q9  // tdiff
        vabd.s16        q12, q3,  q10
        vabd.s16        q13, q2,  q9  // tldiff
        vabd.s16        q14, q2,  q10
        vabd.s16        q9,  q0,  q9  // ldiff
        vabd.s16        q10, q1,  q10
        vmin.u16        q15, q11, q13 // min(tdiff, tldiff)
        vmin.u16        q4,  q12, q14
        vcge.u16        q11, q13, q11 // tldiff >= tdiff
        vcge.u16        q12, q14, q12
        vcge.u16        q9,  q15, q9  // min(tdiff, tldiff) >= ldiff
        vcge.u16        q10, q4,  q10
        vbsl            q12, q3,  q2  // tdiff <= tldiff ? top : topleft
        vbsl            q11, q3,  q2
        vbit            q12, q1,  q10 // ldiff <= min ? left : ...
        vbit            q11, q0,  q9
        vst1.16         {d25}, [r0, :64], r1
        vst1.16         {d24}, [lr, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d23}, [r0, :64], r1
        vst1.16         {d22}, [lr, :64], r1
        bgt             4b
        vpop            {q4}
        pop             {r4-r6, pc}
80:
160:
320:
640:
        vld1.16         {q3},  [r6]!
        mov             r12, r3
        sub             r1,  r1,  r3, lsl #1
1:
        vld2.16         {d0[], d2[]},  [r2, :32], r5
        vmov            d1,  d0
        vmov            d3,  d2
2:
        vsub.i16        q8,  q3,  q2  // top - topleft
        vadd.i16        q9,  q8,  q0  // base
        vadd.i16        q10, q8,  q1
        vabd.s16        q11, q3,  q9  // tdiff
        vabd.s16        q12, q3,  q10
        vabd.s16        q13, q2,  q9  // tldiff
        vabd.s16        q14, q2,  q10
        vabd.s16        q9,  q0,  q9  // ldiff
        vabd.s16        q10, q1,  q10
        vmin.u16        q15, q11, q13 // min(tdiff, tldiff)
        vmin.u16        q4,  q12, q14
        vcge.u16        q11, q13, q11 // tldiff >= tdiff
        vcge.u16        q12, q14, q12
        vcge.u16        q9,  q15, q9  // min(tdiff, tldiff) >= ldiff
        vcge.u16        q10, q4,  q10
        vbsl            q12, q3,  q2  // tdiff <= tldiff ? top : topleft
        vbsl            q11, q3,  q2
        vbit            q12, q1,  q10 // ldiff <= min ? left : ...
        vbit            q11, q0,  q9
        subs            r3,  r3,  #8
        vst1.16         {q12}, [r0, :128]!
        vst1.16         {q11}, [lr, :128]!
        ble             8f
        vld1.16         {q3},  [r6]!
        b               2b
8:
        subs            r4,  r4,  #2
        ble             9f
        // End of horizontal loop, move pointers to next two rows
        sub             r6,  r6,  r12, lsl #1
        add             r0,  r0,  r1
        add             lr,  lr,  r1
        vld1.16         {q3},  [r6]!
        mov             r3,  r12
        b               1b
9:
        vpop            {q4}
        pop             {r4-r6, pc}
endfunc

// void ipred_smooth_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int a,
//                              const int max_width, const int max_height);
function ipred_smooth_16bpc_neon, export=1
        push            {r4-r10, lr}
        ldr             r4,  [sp, #32]
        movrel          r10, X(sm_weights)
        add             r12, r10, r4
        add             r10, r10, r3
        clz             r9,  r3
        adr             r5,  L(ipred_smooth_tbl)
        sub             lr,  r2,  r4, lsl #1
        sub             r9,  r9,  #25
        ldr             r9,  [r5, r9, lsl #2]
        vld1.16         {d4[], d5[]},  [lr] // bottom
        add             r8,  r2,  #2
        add             r5,  r5,  r9
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        bx              r5

        .align 2
L(ipred_smooth_tbl):
        .word 640f - L(ipred_smooth_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_smooth_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_smooth_tbl) + CONFIG_THUMB
        .word 80f  - L(ipred_smooth_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_smooth_tbl) + CONFIG_THUMB

40:
        vld1.16         {d16},   [r8]       // top
        vld1.32         {d18[]}, [r10, :32] // weights_hor
        sub             r2,  r2,  #8
        mov             r7,  #-8
        vdup.16         q3,  d16[3]   // right
        vsub.i16        q8,  q8,  q2  // top-bottom
        vmovl.u8        q9,  d18      // weights_hor
        vadd.i16        d19, d4,  d6  // bottom+right
4:
        vld4.16         {d0[],  d1[],  d2[],  d3[]},  [r2,  :64], r7 // left
        vld4.8          {d20[], d21[], d22[], d23[]}, [r12, :32]!    // weights_ver
        vshll.u16       q12, d19, #8  // (bottom+right)*256
        vshll.u16       q13, d19, #8
        vshll.u16       q14, d19, #8
        vshll.u16       q15, d19, #8
        vzip.32         d20, d21      // weights_ver
        vzip.32         d22, d23
        vsub.i16        q1,  q1,  q3  // left-right
        vsub.i16        q0,  q0,  q3
        vmovl.u8        q10, d20      // weights_ver
        vmovl.u8        q11, d22
        vmlal.s16       q12, d3,  d18 // += (left-right)*weights_hor
        vmlal.s16       q13, d2,  d18 // (left flipped)
        vmlal.s16       q14, d1,  d18
        vmlal.s16       q15, d0,  d18
        vmlal.s16       q12, d16, d20 // += (top-bottom)*weights_ver
        vmlal.s16       q13, d16, d21
        vmlal.s16       q14, d16, d22
        vmlal.s16       q15, d16, d23
        vrshrn.i32      d24, q12, #9
        vrshrn.i32      d25, q13, #9
        vrshrn.i32      d26, q14, #9
        vrshrn.i32      d27, q15, #9
        vst1.16         {d24}, [r0, :64], r1
        vst1.16         {d25}, [r6, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d26}, [r0, :64], r1
        vst1.16         {d27}, [r6, :64], r1
        bgt             4b
        pop             {r4-r10, pc}
80:
        vld1.16         {q8},  [r8]       // top
        vld1.8          {d18}, [r10, :64] // weights_hor
        sub             r2,  r2,  #4
        mov             r7,  #-4
        vdup.16         q3,  d17[3]   // right
        vsub.i16        q8,  q8,  q2  // top-bottom
        vmovl.u8        q9,  d18      // weights_hor
        vadd.i16        d3,  d4,  d6  // bottom+right
8:
        vld2.16         {d0[],  d1[]},  [r2,  :32], r7 // left
        vld2.8          {d20[], d22[]}, [r12, :16]!    // weights_ver
        vshll.u16       q12, d3,  #8  // (bottom+right)*256
        vshll.u16       q13, d3,  #8
        vshll.u16       q14, d3,  #8
        vshll.u16       q15, d3,  #8
        vsub.i16        q0,  q0,  q3  // left-right
        vmovl.u8        q10, d20      // weights_ver
        vmovl.u8        q11, d22
        vmlal.s16       q12, d1,  d18 // += (left-right)*weights_hor
        vmlal.s16       q13, d1,  d19 // (left flipped)
        vmlal.s16       q14, d0,  d18
        vmlal.s16       q15, d0,  d19
        vmlal.s16       q12, d16, d20 // += (top-bottom)*weights_ver
        vmlal.s16       q13, d17, d20
        vmlal.s16       q14, d16, d22
        vmlal.s16       q15, d17, d22
        vrshrn.i32      d24, q12, #9
        vrshrn.i32      d25, q13, #9
        vrshrn.i32      d26, q14, #9
        vrshrn.i32      d27, q15, #9
        subs            r4,  r4,  #2
        vst1.16         {q12}, [r0, :128], r1
        vst1.16         {q13}, [r6, :128], r1
        bgt             8b
        pop             {r4-r10, pc}
160:
320:
640:
        add             lr,  r2,  r3, lsl #1
        sub             r2,  r2,  #4
        mov             r7,  #-4
        vld1.16         {d6[], d7[]}, [lr] // right
        sub             r1,  r1,  r3, lsl #1
        mov             r9,  r3
        vadd.i16        d3,  d4,  d6  // bottom+right

1:
        vld2.16         {d0[],  d1[]},  [r2,  :32], r7 // left
        vld2.8          {d20[], d22[]}, [r12, :16]!    // weights_ver
        vsub.i16        q0,  q0,  q3  // left-right
        vmovl.u8        q10, d20      // weights_ver
        vmovl.u8        q11, d22
2:
        vld1.8          {d18}, [r10, :64]! // weights_hor
        vld1.16         {q8},  [r8]!       // top
        vshll.u16       q12, d3,  #8  // (bottom+right)*256
        vshll.u16       q13, d3,  #8
        vmovl.u8        q9,  d18      // weights_hor
        vshll.u16       q14, d3,  #8
        vshll.u16       q15, d3,  #8
        vsub.i16        q8,  q8,  q2  // top-bottom
        vmlal.s16       q12, d1,  d18 // += (left-right)*weights_hor
        vmlal.s16       q13, d1,  d19 // (left flipped)
        vmlal.s16       q14, d0,  d18
        vmlal.s16       q15, d0,  d19
        vmlal.s16       q12, d16, d20 // += (top-bottom)*weights_ver
        vmlal.s16       q13, d17, d20
        vmlal.s16       q14, d16, d22
        vmlal.s16       q15, d17, d22
        vrshrn.i32      d24, q12, #9
        vrshrn.i32      d25, q13, #9
        vrshrn.i32      d26, q14, #9
        vrshrn.i32      d27, q15, #9
        subs            r3,  r3,  #8
        vst1.16         {q12}, [r0, :128]!
        vst1.16         {q13}, [r6, :128]!
        bgt             2b
        subs            r4,  r4,  #2
        ble             9f
        sub             r8,  r8,  r9, lsl #1
        sub             r10, r10, r9
        add             r0,  r0,  r1
        add             r6,  r6,  r1
        mov             r3,  r9
        b               1b
9:
        pop             {r4-r10, pc}
endfunc

// void ipred_smooth_v_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const topleft,
//                                const int width, const int height, const int a,
//                                const int max_width, const int max_height);
function ipred_smooth_v_16bpc_neon, export=1
        push            {r4-r7, lr}
        ldr             r4,  [sp, #20]
        movrel          r7,  X(sm_weights)
        add             r7,  r7,  r4
        clz             lr,  r3
        adr             r5,  L(ipred_smooth_v_tbl)
        sub             r12, r2,  r4,  lsl #1
        sub             lr,  lr,  #25
        ldr             lr,  [r5, lr, lsl #2]
        vld1.16         {d4[], d5[]},  [r12] // bottom
        add             r2,  r2,  #2
        add             r5,  r5,  lr
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        bx              r5

        .align 2
L(ipred_smooth_v_tbl):
        .word 640f - L(ipred_smooth_v_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_smooth_v_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_smooth_v_tbl) + CONFIG_THUMB
        .word 80f  - L(ipred_smooth_v_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_smooth_v_tbl) + CONFIG_THUMB

40:
        vld1.16         {d6}, [r2]    // top
        vsub.i16        d6,  d6,  d4  // top-bottom
        vmov            d7,  d6
4:
        vld4.8          {d16[], d17[], d18[], d19[]}, [r7, :32]! // weights_ver
        vzip.32         d16, d17      // weights_ver
        vzip.32         d18, d19
        vshll.u8        q8,  d16, #7  // weights_ver << 7
        vshll.u8        q9,  d18, #7
        vqrdmulh.s16    q10, q3,  q8  // ((top-bottom)*weights_ver + 128) >> 8
        vqrdmulh.s16    q11, q3,  q9
        vadd.i16        q10, q10, q2
        vadd.i16        q11, q11, q2
        vst1.16         {d20}, [r0, :64], r1
        vst1.16         {d21}, [r6, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d22}, [r0, :64], r1
        vst1.16         {d23}, [r6, :64], r1
        bgt             4b
        pop             {r4-r7, pc}
80:
        vld1.16         {q3}, [r2]    // top
        vsub.i16        q3,  q3,  q2  // top-bottom
8:
        vld4.8          {d16[], d18[], d20[], d22[]}, [r7, :32]! // weights_ver
        vshll.u8        q8,  d16, #7  // weights_ver << 7
        vshll.u8        q9,  d18, #7
        vshll.u8        q10, d20, #7
        vshll.u8        q11, d22, #7
        vqrdmulh.s16    q8,  q3,  q8  // ((top-bottom)*weights_ver + 128) >> 8
        vqrdmulh.s16    q9,  q3,  q9
        vqrdmulh.s16    q10, q3,  q10
        vqrdmulh.s16    q11, q3,  q11
        vadd.i16        q8,  q8,  q2
        vadd.i16        q9,  q9,  q2
        vadd.i16        q10, q10, q2
        vadd.i16        q11, q11, q2
        vst1.16         {q8},  [r0, :128], r1
        vst1.16         {q9},  [r6, :128], r1
        subs            r4,  r4,  #4
        vst1.16         {q10}, [r0, :128], r1
        vst1.16         {q11}, [r6, :128], r1
        bgt             8b
        pop             {r4-r7, pc}
160:
320:
640:
        vpush           {q4-q7}
        // Set up pointers for four rows in parallel; r0, r6, r5, lr
        add             r5,  r0,  r1
        add             lr,  r6,  r1
        lsl             r1,  r1,  #1
        sub             r1,  r1,  r3, lsl #1
        mov             r12, r3

1:
        vld4.8          {d8[], d10[], d12[], d14[]}, [r7, :32]! // weights_ver
        vshll.u8        q4,  d8,  #7  // weights_ver << 7
        vshll.u8        q5,  d10, #7
        vshll.u8        q6,  d12, #7
        vshll.u8        q7,  d14, #7
2:
        vld1.16         {q0, q1}, [r2]!  // top
        vsub.i16        q0,  q0,  q2  // top-bottom
        vsub.i16        q1,  q1,  q2
        vqrdmulh.s16    q8,  q0,  q4  // ((top-bottom)*weights_ver + 128) >> 8
        vqrdmulh.s16    q9,  q1,  q4
        vqrdmulh.s16    q10, q0,  q5
        vqrdmulh.s16    q11, q1,  q5
        vqrdmulh.s16    q12, q0,  q6
        vqrdmulh.s16    q13, q1,  q6
        vqrdmulh.s16    q14, q0,  q7
        vqrdmulh.s16    q15, q1,  q7
        vadd.i16        q8,  q8,  q2
        vadd.i16        q9,  q9,  q2
        vadd.i16        q10, q10, q2
        vadd.i16        q11, q11, q2
        vadd.i16        q12, q12, q2
        vadd.i16        q13, q13, q2
        vadd.i16        q14, q14, q2
        vadd.i16        q15, q15, q2
        subs            r3,  r3,  #16
        vst1.16         {q8,  q9},  [r0, :128]!
        vst1.16         {q10, q11}, [r6, :128]!
        vst1.16         {q12, q13}, [r5, :128]!
        vst1.16         {q14, q15}, [lr, :128]!
        bgt             2b
        subs            r4,  r4,  #4
        ble             9f
        sub             r2,  r2,  r12, lsl #1
        add             r0,  r0,  r1
        add             r6,  r6,  r1
        add             r5,  r5,  r1
        add             lr,  lr,  r1
        mov             r3,  r12
        b               1b
9:
        vpop            {q4-q7}
        pop             {r4-r7, pc}
endfunc

// void ipred_smooth_h_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const topleft,
//                                const int width, const int height, const int a,
//                                const int max_width, const int max_height);
function ipred_smooth_h_16bpc_neon, export=1
        push            {r4-r8, lr}
        ldr             r4,  [sp, #24]
        movrel          r8,  X(sm_weights)
        add             r8,  r8,  r3
        clz             lr,  r3
        adr             r5,  L(ipred_smooth_h_tbl)
        add             r12, r2,  r3, lsl #1
        sub             lr,  lr,  #25
        ldr             lr,  [r5, lr, lsl #2]
        vld1.16         {d4[], d5[]},  [r12] // right
        add             r5,  r5,  lr
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        bx              r5

        .align 2
L(ipred_smooth_h_tbl):
        .word 640f - L(ipred_smooth_h_tbl) + CONFIG_THUMB
        .word 320f - L(ipred_smooth_h_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_smooth_h_tbl) + CONFIG_THUMB
        .word 80f  - L(ipred_smooth_h_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_smooth_h_tbl) + CONFIG_THUMB

40:
        vld1.32         {d6[]}, [r8, :32] // weights_hor
        sub             r2,  r2,  #8
        mov             r7,  #-8
        vshll.u8        q3,  d6,  #7  // weights_hor << 7
4:
        vld4.16         {d0[], d1[], d2[], d3[]},  [r2, :64], r7 // left
        vsub.i16        q0,  q0,  q2  // left-right
        vsub.i16        q1,  q1,  q2
        subs            r4,  r4,  #4
        vqrdmulh.s16    q8,  q1,  q3  // ((left-right)*weights_hor + 128) >> 8
        vqrdmulh.s16    q9,  q0,  q3  // (left flipped)
        vadd.i16        q8,  q8,  q2
        vadd.i16        q9,  q9,  q2
        vst1.16         {d17}, [r0, :64], r1
        vst1.16         {d16}, [r6, :64], r1
        vst1.16         {d19}, [r0, :64], r1
        vst1.16         {d18}, [r6, :64], r1
        bgt             4b
        pop             {r4-r8, pc}
80:
        vld1.8          {d6}, [r8, :64] // weights_hor
        sub             r2,  r2,  #8
        mov             r7,  #-8
        vshll.u8        q3,  d6,  #7  // weights_hor << 7
8:
        vld1.16         {d23},  [r2, :64], r7 // left
        subs            r4,  r4,  #4
        vsub.i16        d23, d23, d4  // left-right
        vdup.16         q8,  d23[3]   // flip left
        vdup.16         q9,  d23[2]
        vdup.16         q10, d23[1]
        vdup.16         q11, d23[0]
        vqrdmulh.s16    q8,  q8,  q3  // ((left-right)*weights_hor + 128) >> 8
        vqrdmulh.s16    q9,  q9,  q3
        vqrdmulh.s16    q10, q10, q3
        vqrdmulh.s16    q11, q11, q3
        vadd.i16        q8,  q8,  q2
        vadd.i16        q9,  q9,  q2
        vadd.i16        q10, q10, q2
        vadd.i16        q11, q11, q2
        vst1.16         {q8},  [r0, :128], r1
        vst1.16         {q9},  [r6, :128], r1
        vst1.16         {q10}, [r0, :128], r1
        vst1.16         {q11}, [r6, :128], r1
        bgt             8b
        pop             {r4-r8, pc}
160:
320:
640:
        vpush           {q4-q7}
        sub             r2,  r2,  #8
        mov             r7,  #-8
        // Set up pointers for four rows in parallel; r0, r6, r5, lr
        add             r5,  r0,  r1
        add             lr,  r6,  r1
        lsl             r1,  r1,  #1
        sub             r1,  r1,  r3, lsl #1
        mov             r12, r3

1:
        vld1.16         {d15},  [r2, :64], r7 // left
        vsub.i16        d15, d15, d4  // left-right
        vdup.16         q4,  d15[3]   // flip left
        vdup.16         q5,  d15[2]
        vdup.16         q6,  d15[1]
        vdup.16         q7,  d15[0]
2:
        vld1.8          {q1}, [r8, :128]! // weights_hor
        subs            r3,  r3,  #16
        vshll.u8        q0,  d2,  #7  // weights_hor << 7
        vshll.u8        q1,  d3,  #7
        vqrdmulh.s16    q8,  q0,  q4  // ((left-right)*weights_hor + 128) >> 8
        vqrdmulh.s16    q9,  q1,  q4
        vqrdmulh.s16    q10, q0,  q5
        vqrdmulh.s16    q11, q1,  q5
        vqrdmulh.s16    q12, q0,  q6
        vqrdmulh.s16    q13, q1,  q6
        vqrdmulh.s16    q14, q0,  q7
        vqrdmulh.s16    q15, q1,  q7
        vadd.i16        q8,  q8,  q2
        vadd.i16        q9,  q9,  q2
        vadd.i16        q10, q10, q2
        vadd.i16        q11, q11, q2
        vadd.i16        q12, q12, q2
        vadd.i16        q13, q13, q2
        vadd.i16        q14, q14, q2
        vadd.i16        q15, q15, q2
        vst1.16         {q8,  q9},  [r0, :128]!
        vst1.16         {q10, q11}, [r6, :128]!
        vst1.16         {q12, q13}, [r5, :128]!
        vst1.16         {q14, q15}, [lr, :128]!
        bgt             2b
        subs            r4,  r4,  #4
        ble             9f
        sub             r8,  r8,  r12
        add             r0,  r0,  r1
        add             r6,  r6,  r1
        add             r5,  r5,  r1
        add             lr,  lr,  r1
        mov             r3,  r12
        b               1b
9:
        vpop            {q4-q7}
        pop             {r4-r8, pc}
endfunc

// void ipred_filter_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int filt_idx,
//                              const int max_width, const int max_height,
//                              const int bitdepth_max);
.macro filter_fn bpc
function ipred_filter_\bpc\()bpc_neon, export=1
        movw            r12, #511
        ldrd            r4,  r5,   [sp, #88]
        and             r5,  r5,  r12 // 511
        movrel          r6,  X(filter_intra_taps)
        lsl             r5,  r5,  #6
        add             r6,  r6,  r5
        vld1.8          {d20, d21, d22, d23}, [r6, :128]!
        clz             lr,  r3
        adr             r5,  L(ipred_filter\bpc\()_tbl)
        vld1.8          {d27, d28, d29}, [r6, :64]
        sub             lr,  lr,  #26
        ldr             lr,  [r5, lr, lsl #2]
        vmovl.s8        q8,  d20
        vmovl.s8        q9,  d21
        add             r5,  r5,  lr
        vmovl.s8        q10, d22
        vmovl.s8        q11, d23
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        vmovl.s8        q12, d27
        vmovl.s8        q13, d28
        vmovl.s8        q14, d29
        mov             r7,  #-4
        vdup.16         q15, r8
        add             r8,  r2,  #2
        sub             r2,  r2,  #4
.if \bpc == 10
        vmov.i16        q7,  #0
.endif
        bx              r5

        .align 2
L(ipred_filter\bpc\()_tbl):
        .word 320f - L(ipred_filter\bpc\()_tbl) + CONFIG_THUMB
        .word 160f - L(ipred_filter\bpc\()_tbl) + CONFIG_THUMB
        .word 80f  - L(ipred_filter\bpc\()_tbl) + CONFIG_THUMB
        .word 40f  - L(ipred_filter\bpc\()_tbl) + CONFIG_THUMB

40:
        vld1.16         {d0}, [r8]       // top (0-3)
4:
        vld1.16         {d2}, [r2], r7   // left (0-1) + topleft (2)
.if \bpc == 10
        vmul.i16        q2,  q9,  d0[0]  // p1(top[0]) * filter(1)
        vmla.i16        q2,  q10, d0[1]  // p2(top[1]) * filter(2)
        vmla.i16        q2,  q11, d0[2]  // p3(top[2]) * filter(3)
        vmla.i16        q2,  q12, d0[3]  // p4(top[3]) * filter(4)
        vmla.i16        q2,  q8,  d2[2]  // p0(topleft) * filter(0)
        vmla.i16        q2,  q13, d2[1]  // p5(left[0]) * filter(5)
        vmla.i16        q2,  q14, d2[0]  // p6(left[1]) * filter(6)
        vrshr.s16       q2,  q2,  #4
        vmax.s16        q2,  q2,  q7
.else
        vmull.s16       q2,  d18, d0[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q2,  d20, d0[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q2,  d22, d0[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q2,  d24, d0[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q2,  d16, d2[2]  // p0(topleft) * filter(0)
        vmlal.s16       q2,  d26, d2[1]  // p5(left[0]) * filter(5)
        vmlal.s16       q2,  d28, d2[0]  // p6(left[1]) * filter(6)
        vmull.s16       q3,  d19, d0[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q3,  d21, d0[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q3,  d23, d0[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q3,  d25, d0[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q3,  d17, d2[2]  // p0(topleft) * filter(0)
        vmlal.s16       q3,  d27, d2[1]  // p5(left[0]) * filter(5)
        vmlal.s16       q3,  d29, d2[0]  // p6(left[1]) * filter(6)
        vqrshrun.s32    d4,  q2,  #4
        vqrshrun.s32    d5,  q3,  #4
.endif
        vmin.s16        q2,  q2,  q15
        subs            r4,  r4,  #2
        vst1.16         {d4}, [r0, :64], r1
        vst1.16         {d5}, [r6, :64], r1
        vmov            d0,  d5          // move top from [4-7] to [0-3]
        bgt             4b
        vpop            {q4-q7}
        pop             {r4-r8, pc}
80:
        vld1.16         {q0},  [r8]      // top (0-7)
8:
        vld1.16         {d2}, [r2], r7   // left (0-1) + topleft (2)
.if \bpc == 10
        vmul.i16        q2,  q9,  d0[0]  // p1(top[0]) * filter(1)
        vmla.i16        q2,  q10, d0[1]  // p2(top[1]) * filter(2)
        vmla.i16        q2,  q11, d0[2]  // p3(top[2]) * filter(3)
        vmla.i16        q2,  q12, d0[3]  // p4(top[3]) * filter(4)
        vmla.i16        q2,  q8,  d2[2]  // p0(topleft) * filter(0)
        vmla.i16        q2,  q13, d2[1]  // p5(left[0]) * filter(5)
        vmla.i16        q2,  q14, d2[0]  // p6(left[1]) * filter(6)
        vmul.i16        q3,  q9,  d1[0]  // p1(top[0]) * filter(1)
        vmla.i16        q3,  q10, d1[1]  // p2(top[1]) * filter(2)
        vmla.i16        q3,  q11, d1[2]  // p3(top[2]) * filter(3)
        vrshr.s16       q2,  q2,  #4
        vmax.s16        q2,  q2,  q7
        vmin.s16        q2,  q2,  q15
        vmla.i16        q3,  q12, d1[3]  // p4(top[3]) * filter(4)
        vmla.i16        q3,  q8,  d0[3]  // p0(topleft) * filter(0)
        vmla.i16        q3,  q13, d4[3]  // p5(left[0]) * filter(5)
        vmla.i16        q3,  q14, d5[3]  // p6(left[1]) * filter(6)
        vrshr.s16       q3,  q3,  #4
        vmax.s16        q3,  q3,  q7
.else
        vmull.s16       q2,  d18, d0[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q2,  d20, d0[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q2,  d22, d0[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q2,  d24, d0[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q2,  d16, d2[2]  // p0(topleft) * filter(0)
        vmlal.s16       q2,  d26, d2[1]  // p5(left[0]) * filter(5)
        vmlal.s16       q2,  d28, d2[0]  // p6(left[1]) * filter(6)
        vmull.s16       q3,  d19, d0[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q3,  d21, d0[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q3,  d23, d0[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q3,  d25, d0[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q3,  d17, d2[2]  // p0(topleft) * filter(0)
        vmlal.s16       q3,  d27, d2[1]  // p5(left[0]) * filter(5)
        vmlal.s16       q3,  d29, d2[0]  // p6(left[1]) * filter(6)
        vqrshrun.s32    d4,  q2,  #4
        vmull.s16       q4,  d18, d1[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q4,  d20, d1[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q4,  d22, d1[2]  // p3(top[2]) * filter(3)
        vqrshrun.s32    d5,  q3,  #4
        vmin.s16        q2,  q2,  q15
        vmlal.s16       q4,  d24, d1[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q4,  d16, d0[3]  // p0(topleft) * filter(0)
        vmlal.s16       q4,  d26, d4[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q4,  d28, d5[3]  // p6(left[1]) * filter(6)
        vmull.s16       q5,  d19, d1[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q5,  d21, d1[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q5,  d23, d1[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q5,  d25, d1[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q5,  d17, d0[3]  // p0(topleft) * filter(0)
        vmlal.s16       q5,  d27, d4[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q5,  d29, d5[3]  // p6(left[1]) * filter(6)
        vqrshrun.s32    d6,  q4,  #4
        vqrshrun.s32    d7,  q5,  #4
.endif
        vmin.s16        q3,  q3,  q15
        vswp            d5,  d6
        subs            r4,  r4,  #2
        vst1.16         {q2}, [r0, :128], r1
        vmov            q0,  q3
        vst1.16         {q3}, [r6, :128], r1
        bgt             8b
        vpop            {q4-q7}
        pop             {r4-r8, pc}
160:
320:
        sub             r1,  r1,  r3, lsl #1
        mov             lr,  r3

1:
        vld1.16         {d0}, [r2], r7   // left (0-1) + topleft (2)
2:
        vld1.16         {q1, q2}, [r8]!  // top(0-15)
.if \bpc == 10
        vmul.i16        q3,  q8,  d0[2]  // p0(topleft) * filter(0)
        vmla.i16        q3,  q13, d0[1]  // p5(left[0]) * filter(5)
        vmla.i16        q3,  q14, d0[0]  // p6(left[1]) * filter(6)
        vmla.i16        q3,  q9,  d2[0]  // p1(top[0]) * filter(1)
        vmla.i16        q3,  q10, d2[1]  // p2(top[1]) * filter(2)
        vmla.i16        q3,  q11, d2[2]  // p3(top[2]) * filter(3)
        vmla.i16        q3,  q12, d2[3]  // p4(top[3]) * filter(4)

        vmul.i16        q4,  q9,  d3[0]  // p1(top[0]) * filter(1)
        vmla.i16        q4,  q10, d3[1]  // p2(top[1]) * filter(2)
        vmla.i16        q4,  q11, d3[2]  // p3(top[2]) * filter(3)
        vrshr.s16       q3,  q3,  #4
        vmax.s16        q3,  q3,  q7
        vmin.s16        q3,  q3,  q15
        vmla.i16        q4,  q12, d3[3]  // p4(top[3]) * filter(4)
        vmla.i16        q4,  q8,  d2[3]  // p0(topleft) * filter(0)
        vmla.i16        q4,  q13, d6[3]  // p5(left[0]) * filter(5)
        vmla.i16        q4,  q14, d7[3]  // p6(left[1]) * filter(6)

        vmul.i16        q5,  q9,  d4[0]  // p1(top[0]) * filter(1)
        vmla.i16        q5,  q10, d4[1]  // p2(top[1]) * filter(2)
        vmla.i16        q5,  q11, d4[2]  // p3(top[2]) * filter(3)
        vrshr.s16       q4,  q4,  #4
        vmax.s16        q4,  q4,  q7
        vmin.s16        q4,  q4,  q15
        vmov            q0,  q4
        vmla.i16        q5,  q12, d4[3]  // p4(top[3]) * filter(4)
        vmla.i16        q5,  q8,  d3[3]  // p0(topleft) * filter(0)
        vmla.i16        q5,  q13, d0[3]  // p5(left[0]) * filter(5)
        vmla.i16        q5,  q14, d1[3]  // p6(left[1]) * filter(6)

        vmul.i16        q6,  q9,  d5[0]  // p1(top[0]) * filter(1)
        vmla.i16        q6,  q10, d5[1]  // p2(top[1]) * filter(2)
        vmla.i16        q6,  q11, d5[2]  // p3(top[2]) * filter(3)
        vrshr.s16       q5,  q5,  #4
        vmax.s16        q5,  q5,  q7
        vmin.s16        q5,  q5,  q15
        vmov            q0,  q5
        vmov.u16        r12, d5[3]
        vmla.i16        q6,  q12, d5[3]  // p4(top[3]) * filter(4)
        vmla.i16        q6,  q8,  d4[3]  // p0(topleft) * filter(0)
        vmla.i16        q6,  q13, d0[3]  // p5(left[0]) * filter(5)
        vmla.i16        q6,  q14, d1[3]  // p6(left[1]) * filter(6)
        vmov.16         d0[2], r12
        subs            r3,  r3,  #16
        vrshr.s16       q6,  q6,  #4
.else
        vmull.s16       q3,  d16, d0[2]  // p0(topleft) * filter(0)
        vmlal.s16       q3,  d26, d0[1]  // p5(left[0]) * filter(5)
        vmlal.s16       q3,  d28, d0[0]  // p6(left[1]) * filter(6)
        vmlal.s16       q3,  d18, d2[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q3,  d20, d2[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q3,  d22, d2[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q3,  d24, d2[3]  // p4(top[3]) * filter(4)
        vmull.s16       q4,  d17, d0[2]  // p0(topleft) * filter(0)
        vmlal.s16       q4,  d27, d0[1]  // p5(left[0]) * filter(5)
        vmlal.s16       q4,  d29, d0[0]  // p6(left[1]) * filter(6)
        vmlal.s16       q4,  d19, d2[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q4,  d21, d2[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q4,  d23, d2[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q4,  d25, d2[3]  // p4(top[3]) * filter(4)
        vqrshrun.s32    d6,  q3,  #4
        vmull.s16       q5,  d18, d3[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q5,  d20, d3[1]  // p2(top[1]) * filter(2)
        vqrshrun.s32    d7,  q4,  #4
        vmin.s16        q3,  q3,  q15
        vmlal.s16       q5,  d22, d3[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q5,  d24, d3[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q5,  d16, d2[3]  // p0(topleft) * filter(0)
        vmlal.s16       q5,  d26, d6[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q5,  d28, d7[3]  // p6(left[1]) * filter(6)
        vmull.s16       q6,  d19, d3[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q6,  d21, d3[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q6,  d23, d3[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q6,  d25, d3[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q6,  d17, d2[3]  // p0(topleft) * filter(0)
        vmlal.s16       q6,  d27, d6[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q6,  d29, d7[3]  // p6(left[1]) * filter(6)
        vqrshrun.s32    d8,  q5,  #4
        vmull.s16       q7,  d18, d4[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q7,  d20, d4[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q7,  d22, d4[2]  // p3(top[2]) * filter(3)
        vqrshrun.s32    d9,  q6,  #4
        vmin.s16        q0,  q4,  q15
        vmlal.s16       q7,  d24, d4[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q7,  d16, d3[3]  // p0(topleft) * filter(0)
        vmlal.s16       q7,  d26, d0[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q7,  d28, d1[3]  // p6(left[1]) * filter(6)
        vmin.s16        q4,  q4,  q15
        vmull.s16       q6,  d19, d4[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q6,  d21, d4[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q6,  d23, d4[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q6,  d25, d4[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q6,  d17, d3[3]  // p0(topleft) * filter(0)
        vmlal.s16       q6,  d27, d0[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q6,  d29, d1[3]  // p6(left[1]) * filter(6)
        vqrshrun.s32    d10, q7,  #4
        vmull.s16       q1,  d18, d5[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q1,  d20, d5[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q1,  d22, d5[2]  // p3(top[2]) * filter(3)
        vqrshrun.s32    d11, q6,  #4
        vmin.s16        q0,  q5,  q15
        vmlal.s16       q1,  d24, d5[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q1,  d16, d4[3]  // p0(topleft) * filter(0)
        vmlal.s16       q1,  d26, d0[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q1,  d28, d1[3]  // p6(left[1]) * filter(6)
        vmin.s16        q5,  q5,  q15
        vmov.u16        r12, d5[3]
        vmull.s16       q7,  d19, d5[0]  // p1(top[0]) * filter(1)
        vmlal.s16       q7,  d21, d5[1]  // p2(top[1]) * filter(2)
        vmlal.s16       q7,  d23, d5[2]  // p3(top[2]) * filter(3)
        vmlal.s16       q7,  d25, d5[3]  // p4(top[3]) * filter(4)
        vmlal.s16       q7,  d17, d4[3]  // p0(topleft) * filter(0)
        vmlal.s16       q7,  d27, d0[3]  // p5(left[0]) * filter(5)
        vmlal.s16       q7,  d29, d1[3]  // p6(left[1]) * filter(6)
        vmov.16         d0[2], r12
        vqrshrun.s32    d12, q1,  #4
        subs            r3,  r3,  #16
        vqrshrun.s32    d13, q7,  #4
.endif
        vswp            q4,  q5
.if \bpc == 10
        vmax.s16        q6,  q6,  q7
.endif
        vswp            d7,  d10
        vmin.s16        q6,  q6,  q15

        vswp            d9,  d12

        vst1.16         {q3, q4}, [r0, :128]!
        vst1.16         {q5, q6}, [r6, :128]!
        ble             8f
        vmov.u16        r12, d13[3]
        vmov.16         d0[0], r12
        vmov.u16        r12, d9[3]
        vmov.16         d0[1], r12
        b               2b
8:
        subs            r4,  r4,  #2

        ble             9f
        sub             r8,  r6,  lr, lsl #1
        add             r0,  r0,  r1
        add             r6,  r6,  r1
        mov             r3,  lr
        b               1b
9:
        vpop            {q4-q7}
        pop             {r4-r8, pc}
endfunc
.endm

filter_fn 10
filter_fn 12

function ipred_filter_16bpc_neon, export=1
        push            {r4-r8, lr}
        vpush           {q4-q7}
        movw            r12, 0x3ff
        ldr             r8, [sp, #104]
        cmp             r8,  r12
        ble             ipred_filter_10bpc_neon
        b               ipred_filter_12bpc_neon
endfunc

// void pal_pred_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                          const uint16_t *const pal, const uint8_t *idx,
//                          const int w, const int h);
function pal_pred_16bpc_neon, export=1
        push            {r4-r5, lr}
        ldr             r4,  [sp, #12]
        ldr             r5,  [sp, #16]
        vld1.16         {q14}, [r2, :128]
        clz             lr,  r4
        adr             r12, L(pal_pred_tbl)
        sub             lr,  lr,  #25
        ldr             lr,  [r12, lr, lsl #2]
        vmov.i16        q15, #0x100
        add             r12, r12, lr
        add             r2,  r0,  r1
        bx              r12

        .align 2
L(pal_pred_tbl):
        .word 640f - L(pal_pred_tbl) + CONFIG_THUMB
        .word 320f - L(pal_pred_tbl) + CONFIG_THUMB
        .word 160f - L(pal_pred_tbl) + CONFIG_THUMB
        .word 80f  - L(pal_pred_tbl) + CONFIG_THUMB
        .word 40f  - L(pal_pred_tbl) + CONFIG_THUMB

40:
        lsl             r1,  r1,  #1
4:
        vld1.8          {q1}, [r3, :128]!
        subs            r5,  r5,  #4
        // Restructure q1 from a, b, c, ... into 2*a, 2*a+1, 2*b, 2*b+1, 2*c, 2*c+1, ...
        vadd.i8         q0,  q1,  q1
        vadd.i8         q1,  q1,  q1
        vzip.8          q0,  q1
        vadd.i16        q0,  q0,  q15
        vadd.i16        q1,  q1,  q15
        vtbl.8          d0, {q14}, d0
        vtbl.8          d1, {q14}, d1
        vst1.16         {d0}, [r0, :64], r1
        vtbl.8          d2, {q14}, d2
        vst1.16         {d1}, [r2, :64], r1
        vtbl.8          d3, {q14}, d3
        vst1.16         {d2}, [r0, :64], r1
        vst1.16         {d3}, [r2, :64], r1
        bgt             4b
        pop             {r4-r5, pc}
80:
        lsl             r1,  r1,  #1
8:
        vld1.8          {q1, q2}, [r3, :128]!
        subs            r5,  r5,  #4
        // Prefer doing the adds twice, instead of chaining a vmov after
        // the add.
        vadd.i8         q0,  q1,  q1
        vadd.i8         q1,  q1,  q1
        vadd.i8         q3,  q2,  q2
        vadd.i8         q2,  q2,  q2
        vzip.8          q0,  q1
        vzip.8          q2,  q3
        vadd.i16        q0,  q0,  q15
        vadd.i16        q1,  q1,  q15
        vtbl.8          d0, {q14}, d0
        vadd.i16        q2,  q2,  q15
        vtbl.8          d1, {q14}, d1
        vadd.i16        q3,  q3,  q15
        vtbl.8          d2, {q14}, d2
        vtbl.8          d3, {q14}, d3
        vtbl.8          d4, {q14}, d4
        vtbl.8          d5, {q14}, d5
        vst1.16         {q0}, [r0, :128], r1
        vtbl.8          d6, {q14}, d6
        vst1.16         {q1}, [r2, :128], r1
        vtbl.8          d7, {q14}, d7
        vst1.16         {q2}, [r0, :128], r1
        vst1.16         {q3}, [r2, :128], r1
        bgt             8b
        pop             {r4-r5, pc}
160:
        lsl             r1,  r1,  #1
16:
        vld1.8          {q2, q3},   [r3, :128]!
        subs            r5,  r5,  #4
        vld1.8          {q10, q11}, [r3, :128]!
        vadd.i8         q0,  q2,  q2
        vadd.i8         q1,  q2,  q2
        vadd.i8         q2,  q3,  q3
        vadd.i8         q3,  q3,  q3
        vadd.i8         q8,  q10, q10
        vadd.i8         q9,  q10, q10
        vadd.i8         q10, q11, q11
        vzip.8          q0,  q1
        vadd.i8         q11, q11, q11
        vzip.8          q2,  q3
        vzip.8          q8,  q9
        vadd.i16        q0,  q0,  q15
        vzip.8          q10, q11
        vadd.i16        q1,  q1,  q15
        vadd.i16        q2,  q2,  q15
        vadd.i16        q3,  q3,  q15
        vadd.i16        q8,  q8,  q15
        vadd.i16        q9,  q9,  q15
        vadd.i16        q10, q10, q15
        vtbl.8          d0,  {q14}, d0
        vadd.i16        q11, q11, q15
        vtbl.8          d1,  {q14}, d1
        vtbl.8          d2,  {q14}, d2
        vtbl.8          d3,  {q14}, d3
        vtbl.8          d4,  {q14}, d4
        vtbl.8          d5,  {q14}, d5
        vtbl.8          d6,  {q14}, d6
        vtbl.8          d7,  {q14}, d7
        vtbl.8          d16, {q14}, d16
        vtbl.8          d17, {q14}, d17
        vtbl.8          d18, {q14}, d18
        vst1.16         {q0,  q1},  [r0, :128], r1
        vtbl.8          d19, {q14}, d19
        vtbl.8          d20, {q14}, d20
        vst1.16         {q2,  q3},  [r2, :128], r1
        vtbl.8          d21, {q14}, d21
        vtbl.8          d22, {q14}, d22
        vst1.16         {q8,  q9},  [r0, :128], r1
        vtbl.8          d23, {q14}, d23
        vst1.16         {q10, q11}, [r2, :128], r1
        bgt             16b
        pop             {r4-r5, pc}
320:
        lsl             r1,  r1,  #1
        sub             r1,  r1,  #32
32:
        vld1.8          {q2, q3},   [r3, :128]!
        subs            r5,  r5,  #2
        vld1.8          {q10, q11}, [r3, :128]!
        vadd.i8         q0,  q2,  q2
        vadd.i8         q1,  q2,  q2
        vadd.i8         q2,  q3,  q3
        vadd.i8         q3,  q3,  q3
        vadd.i8         q8,  q10, q10
        vadd.i8         q9,  q10, q10
        vadd.i8         q10, q11, q11
        vzip.8          q0,  q1
        vadd.i8         q11, q11, q11
        vzip.8          q2,  q3
        vzip.8          q8,  q9
        vadd.i16        q0,  q0,  q15
        vzip.8          q10, q11
        vadd.i16        q1,  q1,  q15
        vadd.i16        q2,  q2,  q15
        vadd.i16        q3,  q3,  q15
        vadd.i16        q8,  q8,  q15
        vadd.i16        q9,  q9,  q15
        vadd.i16        q10, q10, q15
        vtbl.8          d0,  {q14}, d0
        vadd.i16        q11, q11, q15
        vtbl.8          d1,  {q14}, d1
        vtbl.8          d2,  {q14}, d2
        vtbl.8          d3,  {q14}, d3
        vtbl.8          d4,  {q14}, d4
        vtbl.8          d5,  {q14}, d5
        vtbl.8          d6,  {q14}, d6
        vtbl.8          d7,  {q14}, d7
        vtbl.8          d16, {q14}, d16
        vtbl.8          d17, {q14}, d17
        vtbl.8          d18, {q14}, d18
        vst1.16         {q0,  q1},  [r0, :128]!
        vtbl.8          d19, {q14}, d19
        vtbl.8          d20, {q14}, d20
        vst1.16         {q2,  q3},  [r0, :128], r1
        vtbl.8          d21, {q14}, d21
        vtbl.8          d22, {q14}, d22
        vst1.16         {q8,  q9},  [r2, :128]!
        vtbl.8          d23, {q14}, d23
        vst1.16         {q10, q11}, [r2, :128], r1
        bgt             32b
        pop             {r4-r5, pc}
640:
        sub             r1,  r1,  #96
64:
        vld1.8          {q2, q3},   [r3, :128]!
        subs            r5,  r5,  #1
        vld1.8          {q10, q11}, [r3, :128]!
        vadd.i8         q0,  q2,  q2
        vadd.i8         q1,  q2,  q2
        vadd.i8         q2,  q3,  q3
        vadd.i8         q3,  q3,  q3
        vadd.i8         q8,  q10, q10
        vadd.i8         q9,  q10, q10
        vadd.i8         q10, q11, q11
        vzip.8          q0,  q1
        vadd.i8         q11, q11, q11
        vzip.8          q2,  q3
        vzip.8          q8,  q9
        vadd.i16        q0,  q0,  q15
        vzip.8          q10, q11
        vadd.i16        q1,  q1,  q15
        vadd.i16        q2,  q2,  q15
        vadd.i16        q3,  q3,  q15
        vadd.i16        q8,  q8,  q15
        vadd.i16        q9,  q9,  q15
        vadd.i16        q10, q10, q15
        vtbl.8          d0,  {q14}, d0
        vadd.i16        q11, q11, q15
        vtbl.8          d1,  {q14}, d1
        vtbl.8          d2,  {q14}, d2
        vtbl.8          d3,  {q14}, d3
        vtbl.8          d4,  {q14}, d4
        vtbl.8          d5,  {q14}, d5
        vtbl.8          d6,  {q14}, d6
        vtbl.8          d7,  {q14}, d7
        vtbl.8          d16, {q14}, d16
        vtbl.8          d17, {q14}, d17
        vtbl.8          d18, {q14}, d18
        vst1.16         {q0,  q1},  [r0, :128]!
        vtbl.8          d19, {q14}, d19
        vtbl.8          d20, {q14}, d20
        vst1.16         {q2,  q3},  [r0, :128]!
        vtbl.8          d21, {q14}, d21
        vtbl.8          d22, {q14}, d22
        vst1.16         {q8,  q9},  [r0, :128]!
        vtbl.8          d23, {q14}, d23
        vst1.16         {q10, q11}, [r0, :128], r1
        bgt             64b
        pop             {r4-r5, pc}
endfunc

// void ipred_cfl_128_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height,
//                               const int16_t *ac, const int alpha,
//                               const int bitdepth_max);
function ipred_cfl_128_16bpc_neon, export=1
        push            {r4-r8, lr}
        ldrd            r4,  r5,  [sp, #24]
        ldrd            r6,  r7,  [sp, #32]
        clz             lr,  r3
        vdup.16         q15, r7       // bitdepth_max
        adr             r12, L(ipred_cfl_128_tbl)
        sub             lr,  lr,  #26
        ldr             lr,  [r12, lr, lsl #2]
        vrshr.u16       q0,  q15, #1
        vdup.16         q1,  r6       // alpha
        add             r12, r12, lr
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        vmov.i16        q14, #0
        bx              r12

        .align 2
L(ipred_cfl_128_tbl):
L(ipred_cfl_splat_tbl):
        .word L(ipred_cfl_splat_w16) - L(ipred_cfl_128_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_splat_w16) - L(ipred_cfl_128_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_splat_w8)  - L(ipred_cfl_128_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_splat_w4)  - L(ipred_cfl_128_tbl) + CONFIG_THUMB

L(ipred_cfl_splat_w4):
        vld1.16         {q8, q9}, [r5, :128]!
        vmull.s16       q2,  d16, d2  // diff = ac * alpha
        vmull.s16       q3,  d17, d3
        vmull.s16       q8,  d18, d2
        vmull.s16       q9,  d19, d3
        vshr.s32        q10, q2,  #31 // sign = diff >> 15
        vshr.s32        q11, q3,  #31
        vshr.s32        q12, q8,  #31
        vshr.s32        q13, q9,  #31
        vadd.i32        q2,  q2,  q10 // diff + sign
        vadd.i32        q3,  q3,  q11
        vadd.i32        q8,  q8,  q12
        vadd.i32        q9,  q9,  q13
        vrshrn.i32      d4,  q2,  #6  // (diff + sign + 32) >> 6 = apply_sign()
        vrshrn.i32      d5,  q3,  #6
        vrshrn.i32      d6,  q8,  #6
        vrshrn.i32      d7,  q9,  #6
        vadd.i16        q2,  q2,  q0  // dc + apply_sign()
        vadd.i16        q3,  q3,  q0
        vmax.s16        q2,  q2,  q14
        vmax.s16        q3,  q3,  q14
        vmin.s16        q2,  q2,  q15
        vmin.s16        q3,  q3,  q15
        vst1.16         {d4}, [r0, :64], r1
        vst1.16         {d5}, [r6, :64], r1
        subs            r4,  r4,  #4
        vst1.16         {d6}, [r0, :64], r1
        vst1.16         {d7}, [r6, :64], r1
        bgt             L(ipred_cfl_splat_w4)
        pop             {r4-r8, pc}
L(ipred_cfl_splat_w8):
        vld1.16         {q8, q9}, [r5, :128]!
        subs            r4,  r4,  #2
        vmull.s16       q2,  d16, d2  // diff = ac * alpha
        vmull.s16       q3,  d17, d3
        vmull.s16       q8,  d18, d2
        vmull.s16       q9,  d19, d3
        vshr.s32        q10, q2,  #31 // sign = diff >> 15
        vshr.s32        q11, q3,  #31
        vshr.s32        q12, q8,  #31
        vshr.s32        q13, q9,  #31
        vadd.i32        q2,  q2,  q10 // diff + sign
        vadd.i32        q3,  q3,  q11
        vadd.i32        q8,  q8,  q12
        vadd.i32        q9,  q9,  q13
        vrshrn.i32      d4,  q2,  #6  // (diff + sign + 32) >> 6 = apply_sign()
        vrshrn.i32      d5,  q3,  #6
        vrshrn.i32      d6,  q8,  #6
        vrshrn.i32      d7,  q9,  #6
        vadd.i16        q2,  q2,  q0  // dc + apply_sign()
        vadd.i16        q3,  q3,  q0
        vmax.s16        q2,  q2,  q14
        vmax.s16        q3,  q3,  q14
        vmin.s16        q2,  q2,  q15
        vmin.s16        q3,  q3,  q15
        vst1.16         {q2}, [r0, :128], r1
        vst1.16         {q3}, [r6, :128], r1
        bgt             L(ipred_cfl_splat_w8)
        pop             {r4-r8, pc}
L(ipred_cfl_splat_w16):
        vpush           {q4-q7}
        add             r12, r5,  r3, lsl #1
        sub             r1,  r1,  r3, lsl #1
        mov             lr,  r3
1:
        vld1.16         {q6, q7}, [r5, :128]!
        vmull.s16       q2,  d12, d2  // diff = ac * alpha
        vld1.16         {q8, q9}, [r12, :128]!
        vmull.s16       q3,  d13, d3
        vmull.s16       q4,  d14, d2
        vmull.s16       q5,  d15, d3
        vmull.s16       q6,  d16, d2
        vmull.s16       q7,  d17, d3
        vmull.s16       q8,  d18, d2
        vmull.s16       q9,  d19, d3
        vshr.s32        q10, q2,  #31 // sign = diff >> 15
        vshr.s32        q11, q3,  #31
        vshr.s32        q12, q4,  #31
        vshr.s32        q13, q5,  #31
        vadd.i32        q2,  q2,  q10 // diff + sign
        vshr.s32        q10, q6,  #31
        vadd.i32        q3,  q3,  q11
        vshr.s32        q11, q7,  #31
        vadd.i32        q4,  q4,  q12
        vshr.s32        q12, q8,  #31
        vadd.i32        q5,  q5,  q13
        vshr.s32        q13, q9,  #31
        vadd.i32        q6,  q6,  q10
        vadd.i32        q7,  q7,  q11
        vadd.i32        q8,  q8,  q12
        vadd.i32        q9,  q9,  q13
        vrshrn.i32      d4,  q2,  #6  // (diff + sign + 32) >> 6 = apply_sign()
        vrshrn.i32      d5,  q3,  #6
        vrshrn.i32      d6,  q4,  #6
        vrshrn.i32      d7,  q5,  #6
        vadd.i16        q2,  q2,  q0  // dc + apply_sign()
        vrshrn.i32      d8,  q6,  #6
        vrshrn.i32      d9,  q7,  #6
        vadd.i16        q3,  q3,  q0
        vrshrn.i32      d10, q8,  #6
        vrshrn.i32      d11, q9,  #6
        vadd.i16        q4,  q4,  q0
        vadd.i16        q5,  q5,  q0
        vmax.s16        q2,  q2,  q14
        vmax.s16        q3,  q3,  q14
        vmax.s16        q4,  q4,  q14
        vmax.s16        q5,  q5,  q14
        vmin.s16        q2,  q2,  q15
        vmin.s16        q3,  q3,  q15
        vmin.s16        q4,  q4,  q15
        vmin.s16        q5,  q5,  q15
        subs            r3,  r3,  #16
        vst1.16         {q2, q3}, [r0, :128]!
        vst1.16         {q4, q5}, [r6, :128]!
        bgt             1b
        subs            r4,  r4,  #2
        add             r5,  r5,  lr, lsl #1
        add             r12, r12, lr, lsl #1
        add             r0,  r0,  r1
        add             r6,  r6,  r1
        mov             r3,  lr
        bgt             1b
        vpop            {q4-q7}
        pop             {r4-r8, pc}
endfunc

// void ipred_cfl_top_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height,
//                               const int16_t *ac, const int alpha,
//                               const int bitdepth_max);
function ipred_cfl_top_16bpc_neon, export=1
        push            {r4-r8, lr}
        ldrd            r4,  r5,  [sp, #24]
        ldrd            r6,  r7,  [sp, #32]
        clz             lr,  r3
        vdup.16         q15, r7       // bitdepth_max
        adr             r12, L(ipred_cfl_top_tbl)
        sub             lr,  lr,  #26
        ldr             lr,  [r12, lr, lsl #2]
        vdup.16         q1,  r6   // alpha
        add             r2,  r2,  #2
        add             r12, r12, lr
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        vmov.i16        q14, #0
        bx              r12

        .align 2
L(ipred_cfl_top_tbl):
        .word 32f - L(ipred_cfl_top_tbl) + CONFIG_THUMB
        .word 16f - L(ipred_cfl_top_tbl) + CONFIG_THUMB
        .word 8f  - L(ipred_cfl_top_tbl) + CONFIG_THUMB
        .word 4f  - L(ipred_cfl_top_tbl) + CONFIG_THUMB

4:
        vld1.16         {d0}, [r2]
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #2
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w4)
8:
        vld1.16         {q0}, [r2]
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #3
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w8)
16:
        vld1.16         {q2, q3}, [r2]
        vadd.i16        q0,  q2,  q3
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #4
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w16)
32:
        vld1.16         {q8,  q9},  [r2]!
        vld1.16         {q10, q11}, [r2]
        vadd.i16        q8,  q8,  q9
        vadd.i16        q10, q10, q11
        vadd.i16        q0,  q8,  q10
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpaddl.u16      d0,  d0
        vrshrn.i32      d0,  q0,  #5
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w16)
endfunc

// void ipred_cfl_left_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const topleft,
//                                const int width, const int height,
//                                const int16_t *ac, const int alpha,
//                                const int bitdepth_max);
function ipred_cfl_left_16bpc_neon, export=1
        push            {r4-r8, lr}
        ldrd            r4,  r5,  [sp, #24]
        ldrd            r6,  r7,  [sp, #32]
        sub             r2,  r2,  r4, lsl #1
        clz             lr,  r3
        clz             r8,  r4
        vdup.16         q15, r7       // bitdepth_max
        adr             r12, L(ipred_cfl_splat_tbl)
        adr             r7,  L(ipred_cfl_left_tbl)
        sub             lr,  lr,  #26
        sub             r8,  r8,  #26
        ldr             lr,  [r12, lr, lsl #2]
        ldr             r8,  [r7,  r8, lsl #2]
        vdup.16         q1,  r6   // alpha
        add             r12, r12, lr
        add             r7,  r7,  r8
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        vmov.i16        q14, #0
        bx              r7

        .align 2
L(ipred_cfl_left_tbl):
        .word L(ipred_cfl_left_h32) - L(ipred_cfl_left_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_left_h16) - L(ipred_cfl_left_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_left_h8)  - L(ipred_cfl_left_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_left_h4)  - L(ipred_cfl_left_tbl) + CONFIG_THUMB

L(ipred_cfl_left_h4):
        vld1.16         {d0}, [r2, :64]
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #2
        vdup.16         q0,  d0[0]
        bx              r12

L(ipred_cfl_left_h8):
        vld1.16         {q0}, [r2, :128]
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #3
        vdup.16         q0,  d0[0]
        bx              r12

L(ipred_cfl_left_h16):
        vld1.16         {q2, q3}, [r2, :128]
        vadd.i16        q0,  q2,  q3
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpadd.i16       d0,  d0,  d0
        vrshr.u16       d0,  d0,  #4
        vdup.16         q0,  d0[0]
        bx              r12

L(ipred_cfl_left_h32):
        vld1.16         {q8,  q9},  [r2, :128]!
        vld1.16         {q10, q11}, [r2, :128]
        vadd.i16        q8,  q8,  q9
        vadd.i16        q10, q10, q11
        vadd.i16        q0,  q8,  q10
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        vpaddl.u16      d0,  d0
        vrshrn.i32      d0,  q0,  #5
        vdup.16         q0,  d0[0]
        bx              r12
endfunc

// void ipred_cfl_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                           const pixel *const topleft,
//                           const int width, const int height,
//                           const int16_t *ac, const int alpha,
//                           const int bitdepth_max);
function ipred_cfl_16bpc_neon, export=1
        push            {r4-r8, lr}
        ldrd            r4,  r5,  [sp, #24]
        ldrd            r6,  r7,  [sp, #32]
        sub             r2,  r2,  r4, lsl #1
        add             r8,  r3,  r4  // width + height
        vdup.16         q1,  r6       // alpha
        clz             lr,  r3
        clz             r6,  r4
        vdup.32         d16, r8       // width + height
        vdup.16         q15, r7       // bitdepth_max
        adr             r7,  L(ipred_cfl_tbl)
        rbit            r8,  r8       // rbit(width + height)
        sub             lr,  lr,  #22 // 26 leading bits, minus table offset 4
        sub             r6,  r6,  #26
        clz             r8,  r8       // ctz(width + height)
        ldr             lr,  [r7, lr, lsl #2]
        ldr             r6,  [r7, r6, lsl #2]
        neg             r8,  r8       // -ctz(width + height)
        add             r12, r7,  lr
        add             r7,  r7,  r6
        vshr.u32        d16, d16, #1  // (width + height) >> 1
        vdup.32         d17, r8       // -ctz(width + height)
        add             r6,  r0,  r1
        lsl             r1,  r1,  #1
        vmov.i16        q14, #0
        bx              r7

        .align 2
L(ipred_cfl_tbl):
        .word L(ipred_cfl_h32) - L(ipred_cfl_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_h16) - L(ipred_cfl_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_h8)  - L(ipred_cfl_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_h4)  - L(ipred_cfl_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_w32) - L(ipred_cfl_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_w16) - L(ipred_cfl_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_w8)  - L(ipred_cfl_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_w4)  - L(ipred_cfl_tbl) + CONFIG_THUMB

L(ipred_cfl_h4):
        vld1.16         {d0}, [r2, :64]!
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r12
L(ipred_cfl_w4):
        vld1.16         {d1}, [r2]
        vadd.i32        d0,  d0,  d16
        vpadd.i16       d1,  d1,  d1
        vpaddl.u16      d1,  d1
        cmp             r4,  #4
        vadd.i32        d0,  d0,  d1
        vshl.u32        d0,  d0,  d17
        beq             1f
        // h = 8/16
        cmp             r4,  #16
        movw            lr,  #0x6667
        movw            r8,  #0xAAAB
        it              ne
        movne           lr,  r8
        vdup.32         d18, lr
        vmul.i32        d0,  d0,  d18
        vshr.u32        d0,  d0,  #17
1:
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w4)

L(ipred_cfl_h8):
        vld1.16         {q0}, [r2, :128]!
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r12
L(ipred_cfl_w8):
        vld1.16         {q2}, [r2]
        vadd.i32        d0,  d0,  d16
        vadd.i16        d1,  d4,  d5
        vpadd.i16       d1,  d1,  d1
        vpaddl.u16      d1,  d1
        cmp             r4,  #8
        vadd.i32        d0,  d0,  d1
        vshl.u32        d0,  d0,  d17
        beq             1f
        // h = 4/16/32
        cmp             r4,  #32
        movw            lr,  #0x6667
        movw            r8,  #0xAAAB
        it              ne
        movne           lr,  r8
        vdup.32         d18, lr
        vmul.i32        d0,  d0,  d18
        vshr.u32        d0,  d0,  #17
1:
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w8)

L(ipred_cfl_h16):
        vld1.16         {q2, q3}, [r2, :128]!
        vadd.i16        q0,  q2,  q3
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r12
L(ipred_cfl_w16):
        vld1.16         {q2, q3}, [r2]
        vadd.i32        d0,  d0,  d16
        vadd.i16        q2,  q2,  q3
        vadd.i16        d1,  d4,  d5
        vpadd.i16       d1,  d1,  d1
        vpaddl.u16      d1,  d1
        cmp             r4,  #16
        vadd.i32        d0,  d0,  d1
        vshl.u32        d0,  d0,  d17
        beq             1f
        // h = 4/8/32/64
        tst             r4,  #(32+16+8)  // 16 added to make a consecutive bitmask
        movw            lr,  #0x6667
        movw            r8,  #0xAAAB
        it              ne
        movne           lr,  r8
        vdup.32         d18, lr
        vmul.i32        d0,  d0,  d18
        vshr.u32        d0,  d0,  #17
1:
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w16)

L(ipred_cfl_h32):
        vld1.16         {q2, q3},   [r2, :128]!
        vld1.16         {q10, q11}, [r2, :128]!
        vadd.i16        q2,  q2,  q3
        vadd.i16        q10, q10, q11
        vadd.i16        q0,  q2,  q10
        vadd.i16        d0,  d0,  d1
        vpadd.i16       d0,  d0,  d0
        add             r2,  r2,  #2
        vpaddl.u16      d0,  d0
        bx              r12
L(ipred_cfl_w32):
        vld1.16         {q2, q3},   [r2]!
        vadd.i32        d0,  d0,  d16
        vld1.16         {q10, q11}, [r2]!
        vadd.i16        q2,  q2,  q3
        vadd.i16        q10, q10, q11
        vadd.i16        q2,  q2,  q10
        vadd.i16        d1,  d4,  d5
        vpadd.i16       d1,  d1,  d1
        vpaddl.u16      d1,  d1
        cmp             r4,  #32
        vadd.i32        d0,  d0,  d1
        vshl.u32        d0,  d0,  d17
        beq             1f
        // h = 8/16/64
        cmp             r4,  #8
        movw            lr,  #0x6667
        movw            r8,  #0xAAAB
        it              ne
        movne           lr,  r8
        vdup.32         d18, lr
        vmul.i32        d0,  d0,  d18
        vshr.u32        d0,  d0,  #17
1:
        vdup.16         q0,  d0[0]
        b               L(ipred_cfl_splat_w16)
endfunc

// void cfl_ac_420_16bpc_neon(int16_t *const ac, const pixel *const ypx,
//                            const ptrdiff_t stride, const int w_pad,
//                            const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_420_16bpc_neon, export=1
        push            {r4-r8,lr}
        ldrd            r4,  r5,  [sp, #24]
        ldr             r6,  [sp, #32]
        clz             r8,  r5
        lsl             r4,  r4,  #2
        adr             r7,  L(ipred_cfl_ac_420_tbl)
        sub             r8,  r8,  #27
        ldr             r8,  [r7, r8, lsl #2]
        vmov.i32        q8,  #0
        vmov.i32        q9,  #0
        vmov.i32        q10, #0
        vmov.i32        q11, #0
        add             r7,  r7,  r8
        sub             r8,  r6,  r4  // height - h_pad
        rbit            lr,  r5       // rbit(width)
        rbit            r12, r6       // rbit(height)
        clz             lr,  lr       // ctz(width)
        clz             r12, r12      // ctz(height)
        add             lr,  lr,  r12 // log2sz
        add             r12, r1,  r2
        vdup.32         d31, lr
        lsl             r2,  r2,  #1
        vneg.s32        d31, d31      // -log2sz
        bx              r7

        .align 2
L(ipred_cfl_ac_420_tbl):
        .word L(ipred_cfl_ac_420_w16) - L(ipred_cfl_ac_420_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_420_w8)  - L(ipred_cfl_ac_420_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_420_w4)  - L(ipred_cfl_ac_420_tbl) + CONFIG_THUMB

L(ipred_cfl_ac_420_w4):
1:      // Copy and subsample input
        vld1.16         {q0}, [r1,  :128], r2
        vld1.16         {q1}, [r12, :128], r2
        vld1.16         {q2}, [r1,  :128], r2
        vld1.16         {q3}, [r12, :128], r2
        vadd.i16        q0,  q0,  q1
        vadd.i16        q2,  q2,  q3
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d4,  d5
        vshl.i16        q0,  q0,  #1
        subs            r8,  r8,  #2
        vst1.16         {q0}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        bgt             1b
        cmp             r4,  #0
        vmov            d0,  d1
        vmov            d2,  d1
        vmov            d3,  d1
L(ipred_cfl_ac_420_w4_hpad):
        beq             3f // This assumes that all callers already did "cmp r4, #0"
2:      // Vertical padding (h_pad > 0)
        subs            r4,  r4,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             2b
3:
L(ipred_cfl_ac_420_w4_calc_subtract_dc):
        // Aggregate the sums
        vadd.i32        q8,  q8,  q9
        vadd.i32        q10, q10, q11
        vadd.i32        q0,  q8,  q10
        vadd.i32        d0,  d0,  d1
        vpadd.i32       d0,  d0,  d0  // sum
        sub             r0,  r0,  r6, lsl #3
        vrshl.u32       d16, d0,  d31 // (sum + (1 << (log2sz - 1))) >>= log2sz
        vdup.16         q8,  d16[0]
6:      // Subtract dc from ac
        vld1.16         {q0, q1}, [r0, :128]
        subs            r6,  r6,  #4
        vsub.i16        q0,  q0,  q8
        vsub.i16        q1,  q1,  q8
        vst1.16         {q0, q1}, [r0, :128]!
        bgt             6b
        pop             {r4-r8, pc}

L(ipred_cfl_ac_420_w8):
        cmp             r3,  #0
        bne             L(ipred_cfl_ac_420_w8_wpad)
1:      // Copy and subsample input, without padding
        vld1.16         {q0,  q1},  [r1,  :128], r2
        vld1.16         {q2,  q3},  [r12, :128], r2
        vld1.16         {q12, q13}, [r1,  :128], r2
        vadd.i16        q0,  q0,  q2
        vadd.i16        q1,  q1,  q3
        vld1.16         {q2,  q3},  [r12, :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vadd.i16        q12, q12, q2
        vadd.i16        q13, q13, q3
        vpadd.i16       d2,  d24, d25
        vpadd.i16       d3,  d26, d27
        vshl.i16        q0,  q0,  #1
        vshl.i16        q1,  q1,  #1
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q1
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_420_w8_wpad):
1:      // Copy and subsample input, padding 4
        vld1.16         {q0}, [r1,  :128], r2
        vld1.16         {q1}, [r12, :128], r2
        vld1.16         {q2}, [r1,  :128], r2
        vld1.16         {q3}, [r12, :128], r2
        vadd.i16        q0,  q0,  q1
        vadd.i16        q2,  q2,  q3
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d4,  d5
        vshl.i16        q0,  q0,  #1
        vdup.16         d3,  d1[3]
        vmov            d2,  d1
        vdup.16         d1,  d0[3]
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q1

L(ipred_cfl_ac_420_w8_hpad):
        beq             3f // This assumes that all callers already did "cmp r4, #0"
2:      // Vertical padding (h_pad > 0)
        subs            r4,  r4,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             2b
3:

        // Double the height and reuse the w4 summing/subtracting
        lsl             r6,  r6,  #1
        b               L(ipred_cfl_ac_420_w4_calc_subtract_dc)

L(ipred_cfl_ac_420_w16):
        adr             r7,  L(ipred_cfl_ac_420_w16_tbl)
        ldr             r3,  [r7, r3, lsl #2]
        add             r7,  r7,  r3
        bx              r7

        .align 2
L(ipred_cfl_ac_420_w16_tbl):
        .word L(ipred_cfl_ac_420_w16_wpad0) - L(ipred_cfl_ac_420_w16_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_420_w16_wpad1) - L(ipred_cfl_ac_420_w16_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_420_w16_wpad2) - L(ipred_cfl_ac_420_w16_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_420_w16_wpad3) - L(ipred_cfl_ac_420_w16_tbl) + CONFIG_THUMB

L(ipred_cfl_ac_420_w16_wpad0):
        sub             r2,  r2,  #32
1:      // Copy and subsample input, without padding
        vld1.16         {q0,  q1},  [r1,  :128]!
        vld1.16         {q12, q13}, [r12, :128]!
        vld1.16         {q2,  q3},  [r1,  :128], r2
        vadd.i16        q0,  q0,  q12
        vadd.i16        q1,  q1,  q13
        vld1.16         {q12, q13}, [r12, :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vadd.i16        q2,  q2,  q12
        vadd.i16        q3,  q3,  q13
        vpadd.i16       d2,  d4,  d5
        vpadd.i16       d3,  d6,  d7
        vshl.i16        q0,  q0,  #1
        vshl.i16        q1,  q1,  #1
        subs            r8,  r8,  #1
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad1):
        sub             r2,  r2,  #32
1:      // Copy and subsample input, padding 4
        vld1.16         {q0,  q1},  [r1,  :128]!
        vld1.16         {q12, q13}, [r12, :128]!
        vld1.16         {q2},       [r1,  :128], r2
        vadd.i16        q0,  q0,  q12
        vadd.i16        q1,  q1,  q13
        vld1.16         {q12},     [r12, :128], r2
        vpadd.i16       d0,  d0,  d1
        vadd.i16        q2,  q2,  q12
        vpadd.i16       d1,  d2,  d3
        vpadd.i16       d2,  d4,  d5
        vshl.i16        q0,  q0,  #1
        vshl.i16        d2,  d2,  #1
        subs            r8,  r8,  #1
        vdup.16         d3,  d2[3]
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad2):
1:      // Copy and subsample input, padding 8
        vld1.16         {q0,  q1},  [r1,  :128], r2
        vld1.16         {q12, q13}, [r12, :128], r2
        vadd.i16        q0,  q0,  q12
        vadd.i16        q1,  q1,  q13
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vshl.i16        q0,  q0,  #1
        subs            r8,  r8,  #1
        vdup.16         q1,  d1[3]
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad3):
1:      // Copy and subsample input, padding 12
        vld1.16         {q0},  [r1,  :128], r2
        vld1.16         {q12}, [r12, :128], r2
        vadd.i16        q0,  q0,  q12
        vpadd.i16       d0,  d0,  d1
        vshl.i16        d0,  d0,  #1
        subs            r8,  r8,  #1
        vdup.16         q1,  d0[3]
        vdup.16         d1,  d0[3]
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_hpad):
        beq             3f // This assumes that all callers already did "cmp r4, #0"
2:      // Vertical padding (h_pad > 0)
        subs            r4,  r4,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             2b
3:

        // Quadruple the height and reuse the w4 summing/subtracting
        lsl             r6,  r6,  #2
        b               L(ipred_cfl_ac_420_w4_calc_subtract_dc)
endfunc

// void cfl_ac_422_16bpc_neon(int16_t *const ac, const pixel *const ypx,
//                            const ptrdiff_t stride, const int w_pad,
//                            const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_422_16bpc_neon, export=1
        push            {r4-r8,lr}
        ldrd            r4,  r5,  [sp, #24]
        ldr             r6,  [sp, #32]
        clz             r8,  r5
        lsl             r4,  r4,  #2
        adr             r7,  L(ipred_cfl_ac_422_tbl)
        sub             r8,  r8,  #27
        ldr             r8,  [r7, r8, lsl #2]
        vmov.i16        q8,  #0
        vmov.i16        q9,  #0
        vmov.i16        q10, #0
        vmov.i16        q11, #0
        add             r7,  r7,  r8
        sub             r8,  r6,  r4  // height - h_pad
        rbit            lr,  r5       // rbit(width)
        rbit            r12, r6       // rbit(height)
        clz             lr,  lr       // ctz(width)
        clz             r12, r12      // ctz(height)
        add             lr,  lr,  r12 // log2sz
        add             r12, r1,  r2
        vdup.32         d31, lr
        lsl             r2,  r2,  #1
        vneg.s32        d31, d31      // -log2sz
        bx              r7

        .align 2
L(ipred_cfl_ac_422_tbl):
        .word L(ipred_cfl_ac_422_w16) - L(ipred_cfl_ac_422_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_422_w8) - L(ipred_cfl_ac_422_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_422_w4) - L(ipred_cfl_ac_422_tbl) + CONFIG_THUMB

L(ipred_cfl_ac_422_w4):
1:      // Copy and subsample input
        vld1.16         {q0}, [r1,  :128], r2
        vld1.16         {q1}, [r12, :128], r2
        vld1.16         {q2}, [r1,  :128], r2
        vld1.16         {q3}, [r12, :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vpadd.i16       d2,  d4,  d5
        vpadd.i16       d3,  d6,  d7
        vshl.i16        q0,  q0,  #2
        vshl.i16        q1,  q1,  #2
        subs            r8,  r8,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        vmov            d0,  d3
        vmov            d1,  d3
        vmov            d2,  d3
        b               L(ipred_cfl_ac_420_w4_hpad)

L(ipred_cfl_ac_422_w8):
        cmp             r3,  #0
        bne             L(ipred_cfl_ac_422_w8_wpad)
1:      // Copy and subsample input, without padding
        vld1.16         {q0,  q1},  [r1,  :128], r2
        vld1.16         {q2,  q3},  [r12, :128], r2
        vld1.16         {q12, q13}, [r1,  :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vpadd.i16       d2,  d4,  d5
        vpadd.i16       d3,  d6,  d7
        vld1.16         {q2, q3}, [r12, :128], r2
        vpadd.i16       d24, d24, d25
        vpadd.i16       d25, d26, d27
        vpadd.i16       d26, d4,  d5
        vpadd.i16       d27, d6,  d7
        vshl.i16        q0,  q0,  #2
        vshl.i16        q1,  q1,  #2
        vshl.i16        q2,  q12, #2
        vshl.i16        q3,  q13, #2
        subs            r8,  r8,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q3
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_422_w8_wpad):
1:      // Copy and subsample input, padding 4
        vld1.16         {q0},  [r1,  :128], r2
        vld1.16         {q2},  [r12, :128], r2
        vld1.16         {q12}, [r1,  :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d4,  d5
        vld1.16         {q2, q3}, [r12, :128], r2
        vpadd.i16       d24, d24, d25
        vpadd.i16       d25, d4,  d5
        vshl.i16        q0,  q0,  #2
        vshl.i16        q12, q12, #2
        vdup.16         d7,  d25[3]
        vmov            d6,  d25
        vdup.16         d5,  d24[3]
        vmov            d4,  d24
        vdup.16         d3,  d1[3]
        vmov            d2,  d1
        vdup.16         d1,  d0[3]
        subs            r8,  r8,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q3
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_422_w16):
        adr             r7,  L(ipred_cfl_ac_422_w16_tbl)
        ldr             r3,  [r7, r3, lsl #2]
        add             r7,  r7,  r3
        bx              r7

        .align 2
L(ipred_cfl_ac_422_w16_tbl):
        .word L(ipred_cfl_ac_422_w16_wpad0) - L(ipred_cfl_ac_422_w16_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_422_w16_wpad1) - L(ipred_cfl_ac_422_w16_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_422_w16_wpad2) - L(ipred_cfl_ac_422_w16_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_422_w16_wpad3) - L(ipred_cfl_ac_422_w16_tbl) + CONFIG_THUMB

L(ipred_cfl_ac_422_w16_wpad0):
        sub             r2,  r2,  #32
1:      // Copy and subsample input, without padding
        vld1.16         {q0,  q1},  [r1,  :128]!
        vld1.16         {q2,  q3},  [r12, :128]!
        vld1.16         {q12, q13}, [r1,  :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vpadd.i16       d2,  d24, d25
        vpadd.i16       d3,  d26, d27
        vld1.16         {q12, q13}, [r12, :128], r2
        vpadd.i16       d4,  d4,  d5
        vpadd.i16       d5,  d6,  d7
        vpadd.i16       d6,  d24, d25
        vpadd.i16       d7,  d26, d27
        vshl.i16        q0,  q0,  #2
        vshl.i16        q1,  q1,  #2
        vshl.i16        q2,  q2,  #2
        vshl.i16        q3,  q3,  #2
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q2
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad1):
        sub             r2,  r2,  #32
1:      // Copy and subsample input, padding 4
        vld1.16         {q0,  q1},  [r1,  :128]!
        vld1.16         {q2,  q3},  [r12, :128]!
        vld1.16         {q12},      [r1,  :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vpadd.i16       d2,  d24, d25
        vld1.16         {q12},      [r12, :128], r2
        vpadd.i16       d4,  d4,  d5
        vpadd.i16       d5,  d6,  d7
        vpadd.i16       d6,  d24, d25
        vshl.i16        q0,  q0,  #2
        vshl.i16        d2,  d2,  #2
        vshl.i16        q2,  q2,  #2
        vshl.i16        d6,  d6,  #2
        vdup.16         d3,  d2[3]
        vdup.16         d7,  d6[3]
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q2
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad2):
1:      // Copy and subsample input, padding 8
        vld1.16         {q0,  q1},  [r1,  :128], r2
        vld1.16         {q2,  q3},  [r12, :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d2,  d3
        vpadd.i16       d4,  d4,  d5
        vpadd.i16       d5,  d6,  d7
        vshl.i16        q0,  q0,  #2
        vshl.i16        q2,  q2,  #2
        vdup.16         q1,  d1[3]
        vdup.16         q3,  d5[3]
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q2
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad3):
1:      // Copy and subsample input, padding 12
        vld1.16         {q0}, [r1,  :128], r2
        vld1.16         {q2}, [r12, :128], r2
        vpadd.i16       d0,  d0,  d1
        vpadd.i16       d1,  d4,  d5
        vshl.i16        q0,  q0,  #2
        vdup.16         q3,  d1[3]
        vdup.16         q1,  d0[3]
        vdup.16         d5,  d1[3]
        vmov            d4,  d1
        vdup.16         d1,  d0[3]
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q2
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w16_hpad)
endfunc

// void cfl_ac_444_16bpc_neon(int16_t *const ac, const pixel *const ypx,
//                            const ptrdiff_t stride, const int w_pad,
//                            const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_444_16bpc_neon, export=1
        push            {r4-r8,lr}
        ldrd            r4,  r5,  [sp, #24]
        ldr             r6,  [sp, #32]
        clz             r8,  r5
        lsl             r4,  r4,  #2
        adr             r7,  L(ipred_cfl_ac_444_tbl)
        sub             r8,  r8,  #26
        ldr             r8,  [r7, r8, lsl #2]
        vmov.i16        q8,  #0
        vmov.i16        q9,  #0
        vmov.i16        q10, #0
        vmov.i16        q11, #0
        add             r7,  r7,  r8
        sub             r8,  r6,  r4  // height - h_pad
        rbit            lr,  r5       // rbit(width)
        rbit            r12, r6       // rbit(height)
        clz             lr,  lr       // ctz(width)
        clz             r12, r12      // ctz(height)
        add             lr,  lr,  r12 // log2sz
        add             r12, r1,  r2
        vdup.32         d31, lr
        lsl             r2,  r2,  #1
        vneg.s32        d31, d31      // -log2sz
        bx              r7

        .align 2
L(ipred_cfl_ac_444_tbl):
        .word L(ipred_cfl_ac_444_w32) - L(ipred_cfl_ac_444_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_444_w16) - L(ipred_cfl_ac_444_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_444_w8)  - L(ipred_cfl_ac_444_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_444_w4)  - L(ipred_cfl_ac_444_tbl) + CONFIG_THUMB

L(ipred_cfl_ac_444_w4):
1:      // Copy and expand input
        vld1.16         {d0}, [r1,  :64], r2
        vld1.16         {d1}, [r12, :64], r2
        vld1.16         {d2}, [r1,  :64], r2
        vld1.16         {d3}, [r12, :64], r2
        vshl.i16        q0,  q0,  #3
        vshl.i16        q1,  q1,  #3
        subs            r8,  r8,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        bgt             1b
        cmp             r4,  #0
        vmov            d0,  d3
        vmov            d1,  d3
        vmov            d2,  d3
        b               L(ipred_cfl_ac_420_w4_hpad)

L(ipred_cfl_ac_444_w8):
1:      // Copy and expand input
        vld1.16         {q0}, [r1,  :128], r2
        vld1.16         {q1}, [r12, :128], r2
        vld1.16         {q2}, [r1,  :128], r2
        vld1.16         {q3}, [r12, :128], r2
        vshl.i16        q0,  q0,  #3
        vshl.i16        q1,  q1,  #3
        vshl.i16        q2,  q2,  #3
        vshl.i16        q3,  q3,  #3
        subs            r8,  r8,  #4
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q3
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_444_w16):
        cmp             r3,  #0
        bne             L(ipred_cfl_ac_444_w16_wpad)
1:      // Copy and expand input, without padding
        vld1.16         {q0, q1}, [r1,  :128], r2
        vld1.16         {q2, q3}, [r12, :128], r2
        vshl.i16        q0,  q0,  #3
        vshl.i16        q1,  q1,  #3
        vshl.i16        q2,  q2,  #3
        vshl.i16        q3,  q3,  #3
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q2
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_444_w16_wpad):
1:      // Copy and expand input, padding 8
        vld1.16         {q0}, [r1,  :128], r2
        vld1.16         {q2}, [r12, :128], r2
        vshl.i16        q0,  q0,  #3
        vshl.i16        q2,  q2,  #3
        vdup.16         q1,  d1[3]
        vdup.16         q3,  d5[3]
        subs            r8,  r8,  #2
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        vmov            q0,  q2
        vmov            q1,  q3
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_444_w32):
        adr             r7,  L(ipred_cfl_ac_444_w32_tbl)
        ldr             r3,  [r7, r3, lsl #1] // (w3>>1) << 2
        asr             r2,  r2,  #1
        add             r7,  r7,  r3
        bx              r7

        .align 2
L(ipred_cfl_ac_444_w32_tbl):
        .word L(ipred_cfl_ac_444_w32_wpad0) - L(ipred_cfl_ac_444_w32_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_444_w32_wpad2) - L(ipred_cfl_ac_444_w32_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_444_w32_wpad4) - L(ipred_cfl_ac_444_w32_tbl) + CONFIG_THUMB
        .word L(ipred_cfl_ac_444_w32_wpad6) - L(ipred_cfl_ac_444_w32_tbl) + CONFIG_THUMB

L(ipred_cfl_ac_444_w32_wpad0):
        sub             r2,  r2,  #32
1:      // Copy and expand input, without padding
        vld1.16         {q0, q1}, [r1, :128]!
        vld1.16         {q2, q3}, [r1, :128], r2
        vshl.i16        q0,  q0,  #3
        vshl.i16        q1,  q1,  #3
        vshl.i16        q2,  q2,  #3
        vshl.i16        q3,  q3,  #3
        subs            r8,  r8,  #1
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad2):
        sub             r2,  r2,  #32
1:      // Copy and expand input, padding 8
        vld1.16         {q0, q1}, [r1, :128]!
        vld1.16         {q2},     [r1, :128], r2
        vshl.i16        q0,  q0,  #3
        vshl.i16        q1,  q1,  #3
        vshl.i16        q2,  q2,  #3
        subs            r8,  r8,  #1
        vst1.16         {q0, q1}, [r0, :128]!
        vdup.16         q3,  d5[3]
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad4):
1:      // Copy and expand input, padding 16
        vld1.16         {q0, q1}, [r1, :128], r2
        vshl.i16        q0,  q0,  #3
        vshl.i16        q1,  q1,  #3
        subs            r8,  r8,  #1
        vst1.16         {q0, q1}, [r0, :128]!
        vdup.16         q2,  d3[3]
        vdup.16         q3,  d3[3]
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad6):
1:      // Copy and expand input, padding 24
        vld1.16         {q0}, [r1, :128], r2
        vshl.i16        q0,  q0,  #3
        subs            r8,  r8,  #1
        vdup.16         q1,  d1[3]
        vst1.16         {q0, q1}, [r0, :128]!
        vdup.16         q2,  d1[3]
        vdup.16         q3,  d1[3]
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             1b
        cmp             r4,  #0

L(ipred_cfl_ac_444_w32_hpad):
        beq             3f // This assumes that all callers already did "cmp r4, #0"
2:      // Vertical padding (h_pad > 0)
        subs            r4,  r4,  #1
        vst1.16         {q0, q1}, [r0, :128]!
        vaddw.u16       q8,  q8,  d0
        vaddw.u16       q9,  q9,  d1
        vaddw.u16       q10, q10, d2
        vaddw.u16       q11, q11, d3
        vst1.16         {q2, q3}, [r0, :128]!
        vaddw.u16       q8,  q8,  d4
        vaddw.u16       q9,  q9,  d5
        vaddw.u16       q10, q10, d6
        vaddw.u16       q11, q11, d7
        bgt             2b
3:

        //  Multiply the height by eight and reuse the w4 subtracting
        lsl             r6,  r6,  #3
        b               L(ipred_cfl_ac_420_w4_calc_subtract_dc)
endfunc
