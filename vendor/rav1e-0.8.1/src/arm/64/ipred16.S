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

// void ipred_dc_128_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int a,
//                              const int max_width, const int max_height,
//                              const int bitdepth_max);
function ipred_dc_128_16bpc_neon, export=1
        ldr             w8,  [sp]
        clz             w3,  w3
        adr             x5,  L(ipred_dc_128_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        dup             v0.8h,   w8
        sub             x5,  x5,  w3, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        urshr           v0.8h,   v0.8h,  #1
        br              x5
4:
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        b.gt            4b
        ret
8:
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
16:
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
        mov             v2.16b,  v0.16b
        mov             v3.16b,  v0.16b
32:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            32b
        ret
640:
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
        mov             v2.16b,  v0.16b
        mov             v3.16b,  v0.16b
        sub             x1,  x1,  #64
64:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            64b
        ret

L(ipred_dc_128_tbl):
        .hword L(ipred_dc_128_tbl) - 640b
        .hword L(ipred_dc_128_tbl) - 320b
        .hword L(ipred_dc_128_tbl) - 160b
        .hword L(ipred_dc_128_tbl) -   8b
        .hword L(ipred_dc_128_tbl) -   4b
endfunc

// void ipred_v_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                         const pixel *const topleft,
//                         const int width, const int height, const int a,
//                         const int max_width, const int max_height);
function ipred_v_16bpc_neon, export=1
        clz             w3,  w3
        adr             x5,  L(ipred_v_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        add             x2,  x2,  #2
        sub             x5,  x5,  w3, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h},  [x2]
4:
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h},  [x2]
8:
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h}, [x2]
16:
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2]
32:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            32b
        ret
640:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
        sub             x1,  x1,  #64
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2]
64:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], x1
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], x1
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], x1
        b.gt            64b
        ret

L(ipred_v_tbl):
        .hword L(ipred_v_tbl) - 640b
        .hword L(ipred_v_tbl) - 320b
        .hword L(ipred_v_tbl) - 160b
        .hword L(ipred_v_tbl) -  80b
        .hword L(ipred_v_tbl) -  40b
endfunc

// void ipred_h_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                         const pixel *const topleft,
//                         const int width, const int height, const int a,
//                         const int max_width, const int max_height);
function ipred_h_16bpc_neon, export=1
        clz             w3,  w3
        adr             x5,  L(ipred_h_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        sub             x2,  x2,  #8
        sub             x5,  x5,  w3, uxtw
        mov             x7,  #-8
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
4:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7
        st1             {v3.4h},  [x0], x1
        st1             {v2.4h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v1.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        b.gt            4b
        ret
8:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7
        st1             {v3.8h},  [x0], x1
        st1             {v2.8h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v1.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        b.gt            8b
        ret
16:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7
        str             q3,  [x0, #16]
        str             q2,  [x6, #16]
        st1             {v3.8h}, [x0], x1
        st1             {v2.8h}, [x6], x1
        subs            w4,  w4,  #4
        str             q1,  [x0, #16]
        str             q0,  [x6, #16]
        st1             {v1.8h}, [x0], x1
        st1             {v0.8h}, [x6], x1
        b.gt            16b
        ret
32:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7
        str             q3,  [x0, #16]
        str             q2,  [x6, #16]
        stp             q3,  q3,  [x0, #32]
        stp             q2,  q2,  [x6, #32]
        st1             {v3.8h}, [x0], x1
        st1             {v2.8h}, [x6], x1
        subs            w4,  w4,  #4
        str             q1,  [x0, #16]
        str             q0,  [x6, #16]
        stp             q1,  q1,  [x0, #32]
        stp             q0,  q0,  [x6, #32]
        st1             {v1.8h}, [x0], x1
        st1             {v0.8h}, [x6], x1
        b.gt            32b
        ret
64:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7
        str             q3,  [x0, #16]
        str             q2,  [x6, #16]
        stp             q3,  q3,  [x0, #32]
        stp             q2,  q2,  [x6, #32]
        stp             q3,  q3,  [x0, #64]
        stp             q2,  q2,  [x6, #64]
        stp             q3,  q3,  [x0, #96]
        stp             q2,  q2,  [x6, #96]
        st1             {v3.8h}, [x0], x1
        st1             {v2.8h}, [x6], x1
        subs            w4,  w4,  #4
        str             q1,  [x0, #16]
        str             q0,  [x6, #16]
        stp             q1,  q1,  [x0, #32]
        stp             q0,  q0,  [x6, #32]
        stp             q1,  q1,  [x0, #64]
        stp             q0,  q0,  [x6, #64]
        stp             q1,  q1,  [x0, #96]
        stp             q0,  q0,  [x6, #96]
        st1             {v1.8h}, [x0], x1
        st1             {v0.8h}, [x6], x1
        b.gt            64b
        ret

L(ipred_h_tbl):
        .hword L(ipred_h_tbl) - 64b
        .hword L(ipred_h_tbl) - 32b
        .hword L(ipred_h_tbl) - 16b
        .hword L(ipred_h_tbl) -  8b
        .hword L(ipred_h_tbl) -  4b
endfunc

// void ipred_dc_top_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int a,
//                              const int max_width, const int max_height);
function ipred_dc_top_16bpc_neon, export=1
        clz             w3,  w3
        adr             x5,  L(ipred_dc_top_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        add             x2,  x2,  #2
        sub             x5,  x5,  w3, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h},  [x2]
        addv            h0,      v0.4h
        urshr           v0.4h,   v0.4h,   #2
        dup             v0.4h,   v0.h[0]
4:
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h},  [x2]
        addv            h0,      v0.8h
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
8:
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h}, [x2]
        addp            v0.8h,   v0.8h,   v1.8h
        addv            h0,      v0.8h
        urshr           v2.4h,   v0.4h,   #4
        dup             v0.8h,   v2.h[0]
        dup             v1.8h,   v2.h[0]
16:
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2]
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v0.8h,   v0.8h,   v2.8h
        uaddlv          s0,      v0.8h
        rshrn           v4.4h,   v0.4s,   #5
        dup             v0.8h,   v4.h[0]
        dup             v1.8h,   v4.h[0]
        dup             v2.8h,   v4.h[0]
        dup             v3.8h,   v4.h[0]
32:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            32b
        ret
640:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
        addp            v0.8h,   v0.8h,   v1.8h
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2]
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        addp            v0.8h,   v0.8h,   v2.8h
        addp            v4.8h,   v4.8h,   v6.8h
        addp            v0.8h,   v0.8h,   v4.8h
        uaddlv          s0,      v0.8h
        rshrn           v4.4h,   v0.4s,   #6
        sub             x1,  x1,  #64
        dup             v0.8h,   v4.h[0]
        dup             v1.8h,   v4.h[0]
        dup             v2.8h,   v4.h[0]
        dup             v3.8h,   v4.h[0]
64:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            64b
        ret

L(ipred_dc_top_tbl):
        .hword L(ipred_dc_top_tbl) - 640b
        .hword L(ipred_dc_top_tbl) - 320b
        .hword L(ipred_dc_top_tbl) - 160b
        .hword L(ipred_dc_top_tbl) -  80b
        .hword L(ipred_dc_top_tbl) -  40b
endfunc

// void ipred_dc_left_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height, const int a,
//                               const int max_width, const int max_height);
function ipred_dc_left_16bpc_neon, export=1
        sub             x2,  x2,  w4, uxtw #1
        clz             w3,  w3
        clz             w7,  w4
        adr             x5,  L(ipred_dc_left_tbl)
        sub             w3,  w3,  #20 // 25 leading bits, minus table offset 5
        sub             w7,  w7,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        ldrh            w7,  [x5, w7, uxtw #1]
        sub             x3,  x5,  w3, uxtw
        sub             x5,  x5,  w7, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5

L(ipred_dc_left_h4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h},  [x2]
        addv            h0,      v0.4h
        urshr           v0.4h,   v0.4h,   #2
        dup             v0.8h,   v0.h[0]
        br              x3
L(ipred_dc_left_w4):
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        b.gt            L(ipred_dc_left_w4)
        ret

L(ipred_dc_left_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h},  [x2]
        addv            h0,      v0.8h
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
        br              x3
L(ipred_dc_left_w8):
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        b.gt            L(ipred_dc_left_w8)
        ret

L(ipred_dc_left_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h}, [x2]
        addp            v0.8h,   v0.8h,   v1.8h
        addv            h0,      v0.8h
        urshr           v2.4h,   v0.4h,   #4
        dup             v0.8h,   v2.h[0]
        dup             v1.8h,   v2.h[0]
        br              x3
L(ipred_dc_left_w16):
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
1:
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        b.gt            1b
        ret

L(ipred_dc_left_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2]
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v0.8h,   v0.8h,   v2.8h
        uaddlp          v0.4s,   v0.8h
        addv            s0,      v0.4s
        rshrn           v4.4h,   v0.4s,   #5
        dup             v0.8h,   v4.h[0]
        br              x3
L(ipred_dc_left_w32):
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
        mov             v2.16b,  v0.16b
        mov             v3.16b,  v0.16b
1:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            1b
        ret

L(ipred_dc_left_h64):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
        addp            v0.8h,   v0.8h,   v1.8h
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2]
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        addp            v0.8h,   v0.8h,   v2.8h
        addp            v4.8h,   v4.8h,   v6.8h
        addp            v0.8h,   v0.8h,   v4.8h
        uaddlv          s0,      v0.8h
        rshrn           v4.4h,   v0.4s,   #6
        dup             v0.8h,   v4.h[0]
        br              x3
L(ipred_dc_left_w64):
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
        mov             v2.16b,  v0.16b
        mov             v3.16b,  v0.16b
        sub             x1,  x1,  #64
1:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            1b
        ret

L(ipred_dc_left_tbl):
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_h64)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_h32)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_h16)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_h8)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_h4)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_w64)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_w32)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_w16)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_w8)
        .hword L(ipred_dc_left_tbl) - L(ipred_dc_left_w4)
endfunc

// void ipred_dc_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                          const pixel *const topleft,
//                          const int width, const int height, const int a,
//                          const int max_width, const int max_height);
function ipred_dc_16bpc_neon, export=1
        sub             x2,  x2,  w4, uxtw #1
        add             w7,  w3,  w4             // width + height
        clz             w3,  w3
        clz             w6,  w4
        dup             v16.4s, w7               // width + height
        adr             x5,  L(ipred_dc_tbl)
        rbit            w7,  w7                  // rbit(width + height)
        sub             w3,  w3,  #20            // 25 leading bits, minus table offset 5
        sub             w6,  w6,  #25
        clz             w7,  w7                  // ctz(width + height)
        ldrh            w3,  [x5, w3, uxtw #1]
        ldrh            w6,  [x5, w6, uxtw #1]
        neg             w7,  w7                  // -ctz(width + height)
        sub             x3,  x5,  w3, uxtw
        sub             x5,  x5,  w6, uxtw
        ushr            v16.4s,  v16.4s,  #1     // (width + height) >> 1
        dup             v17.4s,  w7              // -ctz(width + height)
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5

L(ipred_dc_h4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h},  [x2], #8
        uaddlv          s0,      v0.4h
        add             x2,  x2,  #2
        br              x3
L(ipred_dc_w4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.4h},  [x2]
        add             v0.2s,   v0.2s,   v16.2s
        uaddlv          s1,      v1.4h
        cmp             w4,  #4
        add             v0.2s,   v0.2s,   v1.2s
        ushl            v0.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 8/16
        cmp             w4,  #16
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v0.2s,   v0.2s,   v16.2s
        ushr            v0.2s,   v0.2s,   #17
1:
        dup             v0.4h,   v0.h[0]
2:
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.4h},  [x0], x1
        st1             {v0.4h},  [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h},  [x2], #16
        uaddlv          s0,      v0.8h
        add             x2,  x2,  #2
        br              x3
L(ipred_dc_w8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.8h},  [x2]
        add             v0.2s,   v0.2s,   v16.2s
        uaddlv          s1,      v1.8h
        cmp             w4,  #8
        add             v0.2s,   v0.2s,   v1.2s
        ushl            v0.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 4/16/32
        cmp             w4,  #32
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v0.2s,   v0.2s,   v16.2s
        ushr            v0.2s,   v0.2s,   #17
1:
        dup             v0.8h,   v0.h[0]
2:
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h},  [x0], x1
        st1             {v0.8h},  [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h}, [x2], #32
        addp            v0.8h,   v0.8h,   v1.8h
        add             x2,  x2,  #2
        uaddlv          s0,      v0.8h
        br              x3
L(ipred_dc_w16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.8h, v2.8h}, [x2]
        add             v0.2s,   v0.2s,   v16.2s
        addp            v1.8h,   v1.8h,   v2.8h
        uaddlv          s1,      v1.8h
        cmp             w4,  #16
        add             v0.2s,   v0.2s,   v1.2s
        ushl            v4.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 4/8/32/64
        tst             w4,  #(32+16+8) // 16 added to make a consecutive bitmask
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v4.2s,   v4.2s,   v16.2s
        ushr            v4.2s,   v4.2s,   #17
1:
        dup             v0.8h,   v4.h[0]
        dup             v1.8h,   v4.h[0]
2:
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v0.8h, v1.8h}, [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v0.8h,   v0.8h,   v2.8h
        add             x2,  x2,  #2
        uaddlv          s0,      v0.8h
        br              x3
L(ipred_dc_w32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.8h, v2.8h, v3.8h, v4.8h}, [x2]
        add             v0.2s,   v0.2s,   v16.2s
        addp            v1.8h,   v1.8h,   v2.8h
        addp            v3.8h,   v3.8h,   v4.8h
        addp            v1.8h,   v1.8h,   v3.8h
        uaddlv          s1,      v1.8h
        cmp             w4,  #32
        add             v0.2s,   v0.2s,   v1.2s
        ushl            v4.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 8/16/64
        cmp             w4,  #8
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v4.2s,   v4.2s,   v16.2s
        ushr            v4.2s,   v4.2s,   #17
1:
        dup             v0.8h,   v4.h[0]
        dup             v1.8h,   v4.h[0]
        dup             v2.8h,   v4.h[0]
        dup             v3.8h,   v4.h[0]
2:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h64):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
        addp            v0.8h,   v0.8h,   v1.8h
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], #64
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        addp            v0.8h,   v0.8h,   v2.8h
        addp            v4.8h,   v4.8h,   v6.8h
        addp            v0.8h,   v0.8h,   v4.8h
        add             x2,  x2,  #2
        uaddlv          s0,      v0.8h
        br              x3
L(ipred_dc_w64):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.8h, v2.8h, v3.8h, v4.8h}, [x2], #64
        add             v0.2s,   v0.2s,   v16.2s
        addp            v1.8h,   v1.8h,   v2.8h
        ld1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x2]
        addp            v3.8h,   v3.8h,   v4.8h
        addp            v20.8h,  v20.8h,  v21.8h
        addp            v22.8h,  v22.8h,  v23.8h
        addp            v1.8h,   v1.8h,   v3.8h
        addp            v20.8h,  v20.8h,  v22.8h
        addp            v1.8h,   v1.8h,   v20.8h
        uaddlv          s1,      v1.8h
        cmp             w4,  #64
        add             v0.2s,   v0.2s,   v1.2s
        ushl            v4.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 16/32
        cmp             w4,  #16
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v4.2s,   v4.2s,   v16.2s
        ushr            v4.2s,   v4.2s,   #17
1:
        sub             x1,  x1,  #64
        dup             v0.8h,   v4.h[0]
        dup             v1.8h,   v4.h[0]
        dup             v2.8h,   v4.h[0]
        dup             v3.8h,   v4.h[0]
2:
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        b.gt            2b
        ret

L(ipred_dc_tbl):
        .hword L(ipred_dc_tbl) - L(ipred_dc_h64)
        .hword L(ipred_dc_tbl) - L(ipred_dc_h32)
        .hword L(ipred_dc_tbl) - L(ipred_dc_h16)
        .hword L(ipred_dc_tbl) - L(ipred_dc_h8)
        .hword L(ipred_dc_tbl) - L(ipred_dc_h4)
        .hword L(ipred_dc_tbl) - L(ipred_dc_w64)
        .hword L(ipred_dc_tbl) - L(ipred_dc_w32)
        .hword L(ipred_dc_tbl) - L(ipred_dc_w16)
        .hword L(ipred_dc_tbl) - L(ipred_dc_w8)
        .hword L(ipred_dc_tbl) - L(ipred_dc_w4)
endfunc

// void ipred_paeth_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                             const pixel *const topleft,
//                             const int width, const int height, const int a,
//                             const int max_width, const int max_height);
function ipred_paeth_16bpc_neon, export=1
        clz             w9,  w3
        adr             x5,  L(ipred_paeth_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v4.8h},  [x2]
        add             x8,  x2,  #2
        sub             x2,  x2,  #8
        sub             x5,  x5,  w9, uxtw
        mov             x7,  #-8
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v5.2d},  [x8]
        sub             v6.8h,   v5.8h,   v4.8h   // top - topleft
4:
        ld4r            {v0.4h, v1.4h, v2.4h, v3.4h},  [x2], x7
        zip1            v0.2d,   v0.2d,   v1.2d
        zip1            v2.2d,   v2.2d,   v3.2d
        add             v16.8h,  v6.8h,   v0.8h   // base
        add             v17.8h,  v6.8h,   v2.8h
        sabd            v20.8h,  v5.8h,   v16.8h  // tdiff
        sabd            v21.8h,  v5.8h,   v17.8h
        sabd            v22.8h,  v4.8h,   v16.8h  // tldiff
        sabd            v23.8h,  v4.8h,   v17.8h
        sabd            v16.8h,  v0.8h,   v16.8h  // ldiff
        sabd            v17.8h,  v2.8h,   v17.8h
        umin            v18.8h,  v20.8h,  v22.8h  // min(tdiff, tldiff)
        umin            v19.8h,  v21.8h,  v23.8h
        cmge            v20.8h,  v22.8h,  v20.8h  // tldiff >= tdiff
        cmge            v21.8h,  v23.8h,  v21.8h
        cmge            v16.8h,  v18.8h,  v16.8h  // min(tdiff, tldiff) >= ldiff
        cmge            v17.8h,  v19.8h,  v17.8h
        bsl             v21.16b, v5.16b,  v4.16b  // tdiff <= tldiff ? top : topleft
        bsl             v20.16b, v5.16b,  v4.16b
        bit             v21.16b, v2.16b,  v17.16b // ldiff <= min ? left : ...
        bit             v20.16b, v0.16b,  v16.16b
        st1             {v21.d}[1], [x0], x1
        st1             {v21.d}[0], [x6], x1
        subs            w4,  w4,  #4
        st1             {v20.d}[1], [x0], x1
        st1             {v20.d}[0], [x6], x1
        b.gt            4b
        ret
80:
160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v5.8h},  [x8], #16
        mov             w9,  w3
        // Set up pointers for four rows in parallel; x0, x6, x5, x10
        add             x5,  x0,  x1
        add             x10, x6,  x1
        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3, uxtw #1
1:
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7
2:
        sub             v6.8h,   v5.8h,   v4.8h   // top - topleft
        add             v16.8h,  v6.8h,   v0.8h   // base
        add             v17.8h,  v6.8h,   v1.8h
        add             v18.8h,  v6.8h,   v2.8h
        add             v19.8h,  v6.8h,   v3.8h
        sabd            v20.8h,  v5.8h,   v16.8h  // tdiff
        sabd            v21.8h,  v5.8h,   v17.8h
        sabd            v22.8h,  v5.8h,   v18.8h
        sabd            v23.8h,  v5.8h,   v19.8h
        sabd            v24.8h,  v4.8h,   v16.8h  // tldiff
        sabd            v25.8h,  v4.8h,   v17.8h
        sabd            v26.8h,  v4.8h,   v18.8h
        sabd            v27.8h,  v4.8h,   v19.8h
        sabd            v16.8h,  v0.8h,   v16.8h  // ldiff
        sabd            v17.8h,  v1.8h,   v17.8h
        sabd            v18.8h,  v2.8h,   v18.8h
        sabd            v19.8h,  v3.8h,   v19.8h
        umin            v28.8h,  v20.8h,  v24.8h  // min(tdiff, tldiff)
        umin            v29.8h,  v21.8h,  v25.8h
        umin            v30.8h,  v22.8h,  v26.8h
        umin            v31.8h,  v23.8h,  v27.8h
        cmge            v20.8h,  v24.8h,  v20.8h  // tldiff >= tdiff
        cmge            v21.8h,  v25.8h,  v21.8h
        cmge            v22.8h,  v26.8h,  v22.8h
        cmge            v23.8h,  v27.8h,  v23.8h
        cmge            v16.8h,  v28.8h,  v16.8h  // min(tdiff, tldiff) >= ldiff
        cmge            v17.8h,  v29.8h,  v17.8h
        cmge            v18.8h,  v30.8h,  v18.8h
        cmge            v19.8h,  v31.8h,  v19.8h
        bsl             v23.16b, v5.16b,  v4.16b  // tdiff <= tldiff ? top : topleft
        bsl             v22.16b, v5.16b,  v4.16b
        bsl             v21.16b, v5.16b,  v4.16b
        bsl             v20.16b, v5.16b,  v4.16b
        bit             v23.16b, v3.16b,  v19.16b // ldiff <= min ? left : ...
        bit             v22.16b, v2.16b,  v18.16b
        bit             v21.16b, v1.16b,  v17.16b
        bit             v20.16b, v0.16b,  v16.16b
        st1             {v23.8h}, [x0], #16
        st1             {v22.8h}, [x6], #16
        subs            w3,  w3,  #8
        st1             {v21.8h}, [x5], #16
        st1             {v20.8h}, [x10], #16
        b.le            8f
        ld1             {v5.8h},  [x8], #16
        b               2b
8:
        subs            w4,  w4,  #4
        b.le            9f
        // End of horizontal loop, move pointers to next four rows
        sub             x8,  x8,  w9, uxtw #1
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        // Load the top row as early as possible
        ld1             {v5.8h},  [x8], #16
        add             x5,  x5,  x1
        add             x10, x10, x1
        mov             w3,  w9
        b               1b
9:
        ret

L(ipred_paeth_tbl):
        .hword L(ipred_paeth_tbl) - 640b
        .hword L(ipred_paeth_tbl) - 320b
        .hword L(ipred_paeth_tbl) - 160b
        .hword L(ipred_paeth_tbl) -  80b
        .hword L(ipred_paeth_tbl) -  40b
endfunc

// void ipred_smooth_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int a,
//                              const int max_width, const int max_height);
function ipred_smooth_16bpc_neon, export=1
        movrel          x10, X(sm_weights)
        add             x11, x10, w4, uxtw
        add             x10, x10, w3, uxtw
        clz             w9,  w3
        adr             x5,  L(ipred_smooth_tbl)
        sub             x12, x2,  w4, uxtw #1
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v4.8h},  [x12] // bottom
        add             x8,  x2,  #2
        sub             x5,  x5,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v6.2d}, [x8]             // top
        ld1r            {v7.2s}, [x10]            // weights_hor
        sub             x2,  x2,  #8
        mov             x7,  #-8
        dup             v5.8h,   v6.h[3]          // right
        sub             v6.8h,   v6.8h,   v4.8h   // top-bottom
        uxtl            v7.8h,   v7.8b            // weights_hor
        add             v31.4h,  v4.4h,   v5.4h   // bottom+right
4:
        ld4r            {v0.4h, v1.4h, v2.4h, v3.4h},  [x2], x7 // left
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x11], #4 // weights_ver
        ushll           v20.4s,  v31.4h,  #8      // (bottom+right)*256
        ushll           v21.4s,  v31.4h,  #8
        ushll           v22.4s,  v31.4h,  #8
        ushll           v23.4s,  v31.4h,  #8
        zip1            v1.2d,   v1.2d,   v0.2d   // left, flipped
        zip1            v0.2d,   v3.2d,   v2.2d
        zip1            v16.2s,  v16.2s,  v17.2s  // weights_ver
        zip1            v18.2s,  v18.2s,  v19.2s
        sub             v0.8h,   v0.8h,   v5.8h   // left-right
        sub             v1.8h,   v1.8h,   v5.8h
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v18.8h,  v18.8b
        smlal           v20.4s,  v0.4h,   v7.4h   // += (left-right)*weights_hor
        smlal2          v21.4s,  v0.8h,   v7.8h
        smlal           v22.4s,  v1.4h,   v7.4h
        smlal2          v23.4s,  v1.8h,   v7.8h
        smlal           v20.4s,  v6.4h,   v16.4h  // += (top-bottom)*weights_ver
        smlal2          v21.4s,  v6.8h,   v16.8h
        smlal           v22.4s,  v6.4h,   v18.4h
        smlal2          v23.4s,  v6.8h,   v18.8h
        rshrn           v20.4h,  v20.4s,  #9
        rshrn           v21.4h,  v21.4s,  #9
        rshrn           v22.4h,  v22.4s,  #9
        rshrn           v23.4h,  v23.4s,  #9
        st1             {v20.4h}, [x0], x1
        st1             {v21.4h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v22.4h}, [x0], x1
        st1             {v23.4h}, [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v6.8h}, [x8]             // top
        ld1             {v7.8b}, [x10]            // weights_hor
        sub             x2,  x2,  #8
        mov             x7,  #-8
        dup             v5.8h,   v6.h[7]          // right
        sub             v6.8h,   v6.8h,   v4.8h   // top-bottom
        uxtl            v7.8h,   v7.8b            // weights_hor
        add             v31.4h,  v4.4h,   v5.4h   // bottom+right
8:
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7 // left
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x11], #4 // weights_ver
        ushll           v20.4s,  v31.4h,  #8      // (bottom+right)*256
        ushll           v21.4s,  v31.4h,  #8
        ushll           v22.4s,  v31.4h,  #8
        ushll           v23.4s,  v31.4h,  #8
        ushll           v24.4s,  v31.4h,  #8
        ushll           v25.4s,  v31.4h,  #8
        ushll           v26.4s,  v31.4h,  #8
        ushll           v27.4s,  v31.4h,  #8
        sub             v0.8h,   v0.8h,   v5.8h   // left-right
        sub             v1.8h,   v1.8h,   v5.8h
        sub             v2.8h,   v2.8h,   v5.8h
        sub             v3.8h,   v3.8h,   v5.8h
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v17.8h,  v17.8b
        uxtl            v18.8h,  v18.8b
        uxtl            v19.8h,  v19.8b
        smlal           v20.4s,  v3.4h,   v7.4h   // += (left-right)*weights_hor
        smlal2          v21.4s,  v3.8h,   v7.8h   // (left flipped)
        smlal           v22.4s,  v2.4h,   v7.4h
        smlal2          v23.4s,  v2.8h,   v7.8h
        smlal           v24.4s,  v1.4h,   v7.4h
        smlal2          v25.4s,  v1.8h,   v7.8h
        smlal           v26.4s,  v0.4h,   v7.4h
        smlal2          v27.4s,  v0.8h,   v7.8h
        smlal           v20.4s,  v6.4h,   v16.4h  // += (top-bottom)*weights_ver
        smlal2          v21.4s,  v6.8h,   v16.8h
        smlal           v22.4s,  v6.4h,   v17.4h
        smlal2          v23.4s,  v6.8h,   v17.8h
        smlal           v24.4s,  v6.4h,   v18.4h
        smlal2          v25.4s,  v6.8h,   v18.8h
        smlal           v26.4s,  v6.4h,   v19.4h
        smlal2          v27.4s,  v6.8h,   v19.8h
        rshrn           v20.4h,  v20.4s,  #9
        rshrn2          v20.8h,  v21.4s,  #9
        rshrn           v21.4h,  v22.4s,  #9
        rshrn2          v21.8h,  v23.4s,  #9
        rshrn           v22.4h,  v24.4s,  #9
        rshrn2          v22.8h,  v25.4s,  #9
        rshrn           v23.4h,  v26.4s,  #9
        rshrn2          v23.8h,  v27.4s,  #9
        st1             {v20.8h}, [x0], x1
        st1             {v21.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v22.8h}, [x0], x1
        st1             {v23.8h}, [x6], x1
        b.gt            8b
        ret
160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        add             x12, x2,  w3, uxtw #1
        sub             x1,  x1,  w3, uxtw #1
        ld1r            {v5.8h}, [x12]            // right
        sub             x2,  x2,  #4
        mov             x7,  #-4
        mov             w9,  w3
        add             v31.4h,  v4.4h,   v5.4h   // bottom+right

1:
        ld2r            {v0.8h, v1.8h},   [x2],  x7 // left
        ld2r            {v16.8b, v17.8b}, [x11], #2 // weights_ver
        sub             v0.8h,   v0.8h,   v5.8h   // left-right
        sub             v1.8h,   v1.8h,   v5.8h
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v17.8h,  v17.8b
2:
        ld1             {v7.16b}, [x10],  #16     // weights_hor
        ld1             {v2.8h, v3.8h}, [x8], #32 // top
        ushll           v20.4s,  v31.4h,  #8      // (bottom+right)*256
        ushll           v21.4s,  v31.4h,  #8
        ushll           v22.4s,  v31.4h,  #8
        ushll           v23.4s,  v31.4h,  #8
        ushll           v24.4s,  v31.4h,  #8
        ushll           v25.4s,  v31.4h,  #8
        ushll           v26.4s,  v31.4h,  #8
        ushll           v27.4s,  v31.4h,  #8
        uxtl            v6.8h,   v7.8b            // weights_hor
        uxtl2           v7.8h,   v7.16b
        sub             v2.8h,   v2.8h,   v4.8h   // top-bottom
        sub             v3.8h,   v3.8h,   v4.8h
        smlal           v20.4s,  v1.4h,   v6.4h   // += (left-right)*weights_hor
        smlal2          v21.4s,  v1.8h,   v6.8h   // (left flipped)
        smlal           v22.4s,  v1.4h,   v7.4h
        smlal2          v23.4s,  v1.8h,   v7.8h
        smlal           v24.4s,  v0.4h,   v6.4h
        smlal2          v25.4s,  v0.8h,   v6.8h
        smlal           v26.4s,  v0.4h,   v7.4h
        smlal2          v27.4s,  v0.8h,   v7.8h
        smlal           v20.4s,  v2.4h,   v16.4h  // += (top-bottom)*weights_ver
        smlal2          v21.4s,  v2.8h,   v16.8h
        smlal           v22.4s,  v3.4h,   v16.4h
        smlal2          v23.4s,  v3.8h,   v16.8h
        smlal           v24.4s,  v2.4h,   v17.4h
        smlal2          v25.4s,  v2.8h,   v17.8h
        smlal           v26.4s,  v3.4h,   v17.4h
        smlal2          v27.4s,  v3.8h,   v17.8h
        rshrn           v20.4h,  v20.4s,  #9
        rshrn2          v20.8h,  v21.4s,  #9
        rshrn           v21.4h,  v22.4s,  #9
        rshrn2          v21.8h,  v23.4s,  #9
        rshrn           v22.4h,  v24.4s,  #9
        rshrn2          v22.8h,  v25.4s,  #9
        rshrn           v23.4h,  v26.4s,  #9
        rshrn2          v23.8h,  v27.4s,  #9
        subs            w3,  w3,  #16
        st1             {v20.8h, v21.8h}, [x0], #32
        st1             {v22.8h, v23.8h}, [x6], #32
        b.gt            2b
        subs            w4,  w4,  #2
        b.le            9f
        sub             x8,  x8,  w9, uxtw #1
        sub             x10, x10, w9, uxtw
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        mov             w3,  w9
        b               1b
9:
        ret

L(ipred_smooth_tbl):
        .hword L(ipred_smooth_tbl) - 640b
        .hword L(ipred_smooth_tbl) - 320b
        .hword L(ipred_smooth_tbl) - 160b
        .hword L(ipred_smooth_tbl) -  80b
        .hword L(ipred_smooth_tbl) -  40b
endfunc

// void ipred_smooth_v_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const topleft,
//                                const int width, const int height, const int a,
//                                const int max_width, const int max_height);
function ipred_smooth_v_16bpc_neon, export=1
        movrel          x7,  X(sm_weights)
        add             x7,  x7,  w4, uxtw
        clz             w9,  w3
        adr             x5,  L(ipred_smooth_v_tbl)
        sub             x8,  x2,  w4, uxtw #1
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v4.8h},  [x8] // bottom
        add             x2,  x2,  #2
        sub             x5,  x5,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v6.2d}, [x2]             // top
        sub             v6.8h,   v6.8h,   v4.8h   // top-bottom
4:
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x7], #4 // weights_ver
        zip1            v16.2s,  v16.2s,  v17.2s  // weights_ver
        zip1            v18.2s,  v18.2s,  v19.2s
        ushll           v16.8h,  v16.8b,  #7      // weights_ver << 7
        ushll           v18.8h,  v18.8b,  #7
        sqrdmulh        v20.8h,  v6.8h,   v16.8h  // ((top-bottom)*weights_ver + 128) >> 8
        sqrdmulh        v21.8h,  v6.8h,   v18.8h
        add             v20.8h,  v20.8h,  v4.8h
        add             v21.8h,  v21.8h,  v4.8h
        st1             {v20.d}[0], [x0], x1
        st1             {v20.d}[1], [x6], x1
        subs            w4,  w4,  #4
        st1             {v21.d}[0], [x0], x1
        st1             {v21.d}[1], [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v6.8h}, [x2]             // top
        sub             v6.8h,   v6.8h,   v4.8h   // top-bottom
8:
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x7], #4 // weights_ver
        ushll           v16.8h,  v16.8b,  #7      // weights_ver << 7
        ushll           v17.8h,  v17.8b,  #7
        ushll           v18.8h,  v18.8b,  #7
        ushll           v19.8h,  v19.8b,  #7
        sqrdmulh        v20.8h,  v6.8h,   v16.8h  // ((top-bottom)*weights_ver + 128) >> 8
        sqrdmulh        v21.8h,  v6.8h,   v17.8h
        sqrdmulh        v22.8h,  v6.8h,   v18.8h
        sqrdmulh        v23.8h,  v6.8h,   v19.8h
        add             v20.8h,  v20.8h,  v4.8h
        add             v21.8h,  v21.8h,  v4.8h
        add             v22.8h,  v22.8h,  v4.8h
        add             v23.8h,  v23.8h,  v4.8h
        st1             {v20.8h}, [x0], x1
        st1             {v21.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v22.8h}, [x0], x1
        st1             {v23.8h}, [x6], x1
        b.gt            8b
        ret
160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        // Set up pointers for four rows in parallel; x0, x6, x5, x8
        add             x5,  x0,  x1
        add             x8,  x6,  x1
        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3, uxtw #1
        mov             w9,  w3

1:
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b}, [x7], #4 // weights_ver
        ushll           v16.8h,  v16.8b,  #7      // weights_ver << 7
        ushll           v17.8h,  v17.8b,  #7
        ushll           v18.8h,  v18.8b,  #7
        ushll           v19.8h,  v19.8b,  #7
2:
        ld1             {v2.8h, v3.8h}, [x2], #32 // top
        sub             v2.8h,   v2.8h,   v4.8h   // top-bottom
        sub             v3.8h,   v3.8h,   v4.8h
        sqrdmulh        v20.8h,  v2.8h,   v16.8h  // ((top-bottom)*weights_ver + 128) >> 8
        sqrdmulh        v21.8h,  v3.8h,   v16.8h
        sqrdmulh        v22.8h,  v2.8h,   v17.8h
        sqrdmulh        v23.8h,  v3.8h,   v17.8h
        sqrdmulh        v24.8h,  v2.8h,   v18.8h
        sqrdmulh        v25.8h,  v3.8h,   v18.8h
        sqrdmulh        v26.8h,  v2.8h,   v19.8h
        sqrdmulh        v27.8h,  v3.8h,   v19.8h
        add             v20.8h,  v20.8h,  v4.8h
        add             v21.8h,  v21.8h,  v4.8h
        add             v22.8h,  v22.8h,  v4.8h
        add             v23.8h,  v23.8h,  v4.8h
        add             v24.8h,  v24.8h,  v4.8h
        add             v25.8h,  v25.8h,  v4.8h
        add             v26.8h,  v26.8h,  v4.8h
        add             v27.8h,  v27.8h,  v4.8h
        subs            w3,  w3,  #16
        st1             {v20.8h, v21.8h}, [x0], #32
        st1             {v22.8h, v23.8h}, [x6], #32
        st1             {v24.8h, v25.8h}, [x5], #32
        st1             {v26.8h, v27.8h}, [x8], #32
        b.gt            2b
        subs            w4,  w4,  #4
        b.le            9f
        sub             x2,  x2,  w9, uxtw #1
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        add             x5,  x5,  x1
        add             x8,  x8,  x1
        mov             w3,  w9
        b               1b
9:
        ret

L(ipred_smooth_v_tbl):
        .hword L(ipred_smooth_v_tbl) - 640b
        .hword L(ipred_smooth_v_tbl) - 320b
        .hword L(ipred_smooth_v_tbl) - 160b
        .hword L(ipred_smooth_v_tbl) -  80b
        .hword L(ipred_smooth_v_tbl) -  40b
endfunc

// void ipred_smooth_h_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const topleft,
//                                const int width, const int height, const int a,
//                                const int max_width, const int max_height);
function ipred_smooth_h_16bpc_neon, export=1
        movrel          x8,  X(sm_weights)
        add             x8,  x8,  w3, uxtw
        clz             w9,  w3
        adr             x5,  L(ipred_smooth_h_tbl)
        add             x12, x2,  w3, uxtw #1
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v5.8h},  [x12] // right
        sub             x5,  x5,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v7.2s}, [x8]             // weights_hor
        sub             x2,  x2,  #8
        mov             x7,  #-8
        ushll           v7.8h,   v7.8b,   #7      // weights_hor << 7
4:
        ld4r            {v0.4h, v1.4h, v2.4h, v3.4h},  [x2], x7 // left
        zip1            v1.2d,   v1.2d,   v0.2d   // left, flipped
        zip1            v0.2d,   v3.2d,   v2.2d
        sub             v0.8h,   v0.8h,   v5.8h   // left-right
        sub             v1.8h,   v1.8h,   v5.8h
        sqrdmulh        v20.8h,  v0.8h,   v7.8h   // ((left-right)*weights_hor + 128) >> 8
        sqrdmulh        v21.8h,  v1.8h,   v7.8h
        add             v20.8h,  v20.8h,  v5.8h
        add             v21.8h,  v21.8h,  v5.8h
        st1             {v20.d}[0], [x0], x1
        st1             {v20.d}[1], [x6], x1
        subs            w4,  w4,  #4
        st1             {v21.d}[0], [x0], x1
        st1             {v21.d}[1], [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v7.8b}, [x8]             // weights_hor
        sub             x2,  x2,  #8
        mov             x7,  #-8
        ushll           v7.8h,   v7.8b,   #7      // weights_hor << 7
8:
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},  [x2], x7 // left
        sub             v3.8h,   v3.8h,   v5.8h   // left-right
        sub             v2.8h,   v2.8h,   v5.8h
        sub             v1.8h,   v1.8h,   v5.8h
        sub             v0.8h,   v0.8h,   v5.8h
        sqrdmulh        v20.8h,  v3.8h,   v7.8h   // ((left-right)*weights_hor + 128) >> 8
        sqrdmulh        v21.8h,  v2.8h,   v7.8h   // (left flipped)
        sqrdmulh        v22.8h,  v1.8h,   v7.8h
        sqrdmulh        v23.8h,  v0.8h,   v7.8h
        add             v20.8h,  v20.8h,  v5.8h
        add             v21.8h,  v21.8h,  v5.8h
        add             v22.8h,  v22.8h,  v5.8h
        add             v23.8h,  v23.8h,  v5.8h
        st1             {v20.8h}, [x0], x1
        st1             {v21.8h}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v22.8h}, [x0], x1
        st1             {v23.8h}, [x6], x1
        b.gt            8b
        ret
160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        sub             x2,  x2,  #8
        mov             x7,  #-8
        // Set up pointers for four rows in parallel; x0, x6, x5, x10
        add             x5,  x0,  x1
        add             x10, x6,  x1
        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3, uxtw #1
        mov             w9,  w3

1:
        ld4r            {v0.8h, v1.8h, v2.8h, v3.8h},   [x2],  x7 // left
        sub             v0.8h,   v0.8h,   v5.8h   // left-right
        sub             v1.8h,   v1.8h,   v5.8h
        sub             v2.8h,   v2.8h,   v5.8h
        sub             v3.8h,   v3.8h,   v5.8h
2:
        ld1             {v7.16b}, [x8],   #16     // weights_hor
        ushll           v6.8h,   v7.8b,   #7      // weights_hor << 7
        ushll2          v7.8h,   v7.16b,  #7
        sqrdmulh        v20.8h,  v3.8h,   v6.8h   // ((left-right)*weights_hor + 128) >> 8
        sqrdmulh        v21.8h,  v3.8h,   v7.8h   // (left flipped)
        sqrdmulh        v22.8h,  v2.8h,   v6.8h
        sqrdmulh        v23.8h,  v2.8h,   v7.8h
        sqrdmulh        v24.8h,  v1.8h,   v6.8h
        sqrdmulh        v25.8h,  v1.8h,   v7.8h
        sqrdmulh        v26.8h,  v0.8h,   v6.8h
        sqrdmulh        v27.8h,  v0.8h,   v7.8h
        add             v20.8h,  v20.8h,  v5.8h
        add             v21.8h,  v21.8h,  v5.8h
        add             v22.8h,  v22.8h,  v5.8h
        add             v23.8h,  v23.8h,  v5.8h
        add             v24.8h,  v24.8h,  v5.8h
        add             v25.8h,  v25.8h,  v5.8h
        add             v26.8h,  v26.8h,  v5.8h
        add             v27.8h,  v27.8h,  v5.8h
        subs            w3,  w3,  #16
        st1             {v20.8h, v21.8h}, [x0],  #32
        st1             {v22.8h, v23.8h}, [x6],  #32
        st1             {v24.8h, v25.8h}, [x5],  #32
        st1             {v26.8h, v27.8h}, [x10], #32
        b.gt            2b
        subs            w4,  w4,  #4
        b.le            9f
        sub             x8,  x8,  w9, uxtw
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        add             x5,  x5,  x1
        add             x10, x10, x1
        mov             w3,  w9
        b               1b
9:
        ret

L(ipred_smooth_h_tbl):
        .hword L(ipred_smooth_h_tbl) - 640b
        .hword L(ipred_smooth_h_tbl) - 320b
        .hword L(ipred_smooth_h_tbl) - 160b
        .hword L(ipred_smooth_h_tbl) -  80b
        .hword L(ipred_smooth_h_tbl) -  40b
endfunc

const padding_mask_buf
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        .byte 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
padding_mask:
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
endconst

// void ipred_z1_upsample_edge_16bpc_neon(pixel *out, const int hsz,
//                                        const pixel *const in, const int end,
//                                        const int bitdepth_max);
function ipred_z1_upsample_edge_16bpc_neon, export=1
        dup             v30.8h,  w4               // bitdepth_max
        movrel          x4,  padding_mask
        ld1             {v0.8h, v1.8h},  [x2]     // in[]
        add             x5,  x2,  w3,  uxtw #1    // in[end]
        sub             x4,  x4,  w3,  uxtw #1

        ld1r            {v2.8h},  [x5]            // padding
        ld1             {v3.8h, v4.8h}, [x4]      // padding_mask

        movi            v31.8h,  #9

        bit             v0.16b,  v2.16b,  v3.16b  // padded in[]
        bit             v1.16b,  v2.16b,  v4.16b

        ext             v4.16b,  v0.16b,  v1.16b,  #2
        ext             v5.16b,  v1.16b,  v2.16b,  #2
        ext             v6.16b,  v0.16b,  v1.16b,  #4
        ext             v7.16b,  v1.16b,  v2.16b,  #4
        ext             v16.16b, v0.16b,  v1.16b,  #6
        ext             v17.16b, v1.16b,  v2.16b,  #6

        add             v18.8h,  v4.8h,   v6.8h   // in[i+1] + in[i+2]
        add             v19.8h,  v5.8h,   v7.8h
        add             v20.8h,  v0.8h,   v16.8h
        add             v21.8h,  v1.8h,   v17.8h
        umull           v22.4s,  v18.4h,  v31.4h  // 9*(in[i+1] + in[i+2])
        umull2          v23.4s,  v18.8h,  v31.8h
        umull           v24.4s,  v19.4h,  v31.4h
        umull2          v25.4s,  v19.8h,  v31.8h
        usubw           v22.4s,  v22.4s,  v20.4h
        usubw2          v23.4s,  v23.4s,  v20.8h
        usubw           v24.4s,  v24.4s,  v21.4h
        usubw2          v25.4s,  v25.4s,  v21.8h

        sqrshrun        v16.4h,  v22.4s,  #4
        sqrshrun2       v16.8h,  v23.4s,  #4
        sqrshrun        v17.4h,  v24.4s,  #4
        sqrshrun2       v17.8h,  v25.4s,  #4

        smin            v16.8h,  v16.8h,  v30.8h
        smin            v17.8h,  v17.8h,  v30.8h

        zip1            v0.8h,   v4.8h,   v16.8h
        zip2            v1.8h,   v4.8h,   v16.8h
        zip1            v2.8h,   v5.8h,   v17.8h
        zip2            v3.8h,   v5.8h,   v17.8h

        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0]

        ret
endfunc

// void ipred_z2_upsample_edge_16bpc_neon(pixel *out, const int sz,
//                                        const pixel *const in,
//                                        const int bitdepth_max);
function ipred_z2_upsample_edge_16bpc_neon, export=1
        dup             v30.8h,  w3               // bitdepth_max
        // Here, sz is 4 or 8, and we produce 2*sz+1 output elements.
        movrel          x4,  padding_mask
        ld1             {v0.8h, v1.8h}, [x2]      // in[]
        add             x5,  x2,  w1,  uxtw #1    // in[sz]
        sub             x4,  x4,  w1,  uxtw #1

        ld1r            {v3.8h},  [x2]            // in[0] for padding
        ld1r            {v2.8h},  [x5]            // padding
        ld1             {v4.8h, v5.8h}, [x4]      // padding_mask

        movi            v31.8h,  #9

        bit             v0.16b,  v2.16b,  v4.16b  // padded in[]
        bit             v1.16b,  v2.16b,  v5.16b

        ext             v4.16b,  v3.16b,  v0.16b,  #14
        ext             v5.16b,  v0.16b,  v1.16b,  #2
        ext             v6.16b,  v0.16b,  v1.16b,  #4

        add             v16.8h,  v0.8h,   v5.8h   // in[i+0] + in[i+1]
        add             v17.8h,  v4.8h,   v6.8h   // in[i-1] + in[i+2]
        umull           v18.4s,  v16.4h,  v31.4h  // 9*(in[i+1] + in[i+2])
        umull2          v19.4s,  v16.8h,  v31.8h
        usubw           v18.4s,  v18.4s,  v17.4h
        usubw2          v19.4s,  v19.4s,  v17.8h

        sqrshrun        v16.4h,  v18.4s,  #4
        sqrshrun2       v16.8h,  v19.4s,  #4

        add             x5,  x0,  #2*16

        smin            v16.8h,  v16.8h,  v30.8h

        zip1            v4.8h,   v0.8h,   v16.8h
        zip2            v5.8h,   v0.8h,   v16.8h

        st1             {v2.h}[0], [x5]
        // In case sz=8, output one single pixel in out[16].
        st1             {v4.8h, v5.8h}, [x0]

        ret
endfunc

const edge_filter
        .short 0, 4, 8, 0
        .short 0, 5, 6, 0
// Leaving out the coeffs for strength=3
//      .byte 2, 4, 4, 0
endconst

// void ipred_z1_filter_edge_16bpc_neon(pixel *out, const int sz,
//                                      const pixel *const in, const int end,
//                                      const int strength);
function ipred_z1_filter_edge_16bpc_neon, export=1
        cmp             w4, #3
        b.eq            L(fivetap)                // if (strength == 3) goto fivetap

        movrel          x5,  edge_filter, -6
        add             x5,  x5,  w4,  uxtw #3    // edge_filter + 2*((strength - 1)*4 + 1)

        ld1             {v31.s}[0], [x5]          // kernel[1-2]

        ld1             {v0.8h}, [x2], #16

        dup             v30.8h, v31.h[0]
        dup             v31.8h, v31.h[1]
1:
        // in[end], is the last valid pixel. We produce 16 pixels out by
        // using 18 pixels in - the last pixel used is [17] of the ones
        // read/buffered.
        cmp             w3,  #17
        ld1             {v1.8h, v2.8h}, [x2], #32
        b.lt            2f
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        ext             v4.16b,  v1.16b,  v2.16b,  #2
        ext             v5.16b,  v0.16b,  v1.16b,  #4
        ext             v6.16b,  v1.16b,  v2.16b,  #4
        mul             v16.8h,  v0.8h,   v30.8h
        mla             v16.8h,  v3.8h,   v31.8h
        mla             v16.8h,  v5.8h,   v30.8h
        mul             v17.8h,  v1.8h,   v30.8h
        mla             v17.8h,  v4.8h,   v31.8h
        mla             v17.8h,  v6.8h,   v30.8h
        subs            w1,  w1,  #16
        mov             v0.16b,  v2.16b
        urshr           v16.8h,  v16.8h,  #4
        urshr           v17.8h,  v17.8h,  #4
        sub             w3,  w3,  #16
        st1             {v16.8h, v17.8h}, [x0], #32
        b.gt            1b
        ret
2:
        // Right padding

        // x2[w3-24] is the padding pixel (x2 points 24 pixels ahead)
        movrel          x5,  padding_mask
        sub             w6,  w3,  #24
        sub             x5,  x5,  w3,  uxtw #1
        add             x6,  x2,  w6,  sxtw #1

        ld1             {v3.8h, v4.8h}, [x5] // padding_mask

        ld1r            {v2.8h}, [x6]
        bit             v0.16b,  v2.16b,  v3.16b  // Pad v0-v1
        bit             v1.16b,  v2.16b,  v4.16b

        // Filter one block
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        ext             v4.16b,  v1.16b,  v2.16b,  #2
        ext             v5.16b,  v0.16b,  v1.16b,  #4
        ext             v6.16b,  v1.16b,  v2.16b,  #4
        mul             v16.8h,  v0.8h,   v30.8h
        mla             v16.8h,  v3.8h,   v31.8h
        mla             v16.8h,  v5.8h,   v30.8h
        mul             v17.8h,  v1.8h,   v30.8h
        mla             v17.8h,  v4.8h,   v31.8h
        mla             v17.8h,  v6.8h,   v30.8h
        subs            w1,  w1,  #16
        urshr           v16.8h,  v16.8h,  #4
        urshr           v17.8h,  v17.8h,  #4
        st1             {v16.8h, v17.8h}, [x0], #32
        b.le            9f
5:
        // After one block, any remaining output would only be filtering
        // padding - thus just store the padding.
        subs            w1,  w1,  #16
        st1             {v2.16b}, [x0], #16
        b.gt            5b
9:
        ret

L(fivetap):
        sub             x2,  x2,  #2              // topleft -= 1 pixel
        movi            v29.8h, #2
        ld1             {v0.8h}, [x2], #16
        movi            v30.8h, #4
        movi            v31.8h, #4
        ins             v0.h[0], v0.h[1]
1:
        // in[end+1], is the last valid pixel. We produce 16 pixels out by
        // using 20 pixels in - the last pixel used is [19] of the ones
        // read/buffered.
        cmp             w3,  #18
        ld1             {v1.8h, v2.8h}, [x2], #32
        b.lt            2f                        // if (end + 1 < 19)
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        ext             v4.16b,  v1.16b,  v2.16b,  #2
        ext             v5.16b,  v0.16b,  v1.16b,  #4
        ext             v6.16b,  v1.16b,  v2.16b,  #4
        ext             v16.16b, v0.16b,  v1.16b,  #6
        ext             v17.16b, v1.16b,  v2.16b,  #6
        ext             v18.16b, v0.16b,  v1.16b,  #8
        ext             v19.16b, v1.16b,  v2.16b,  #8
        mul             v20.8h,  v0.8h,   v29.8h
        mla             v20.8h,  v3.8h,   v30.8h
        mla             v20.8h,  v5.8h,   v31.8h
        mla             v20.8h,  v16.8h,  v30.8h
        mla             v20.8h,  v18.8h,  v29.8h
        mul             v21.8h,  v1.8h,   v29.8h
        mla             v21.8h,  v4.8h,   v30.8h
        mla             v21.8h,  v6.8h,   v31.8h
        mla             v21.8h,  v17.8h,  v30.8h
        mla             v21.8h,  v19.8h,  v29.8h
        subs            w1,  w1,  #16
        mov             v0.16b,  v2.16b
        urshr           v20.8h,  v20.8h,  #4
        urshr           v21.8h,  v21.8h,  #4
        sub             w3,  w3,  #16
        st1             {v20.8h, v21.8h}, [x0], #32
        b.gt            1b
        ret
2:
        // Right padding

        // x2[w3+1-24] is the padding pixel (x2 points 24 pixels ahead)
        movrel          x5,  padding_mask, -2
        sub             w6,  w3,  #23
        sub             x5,  x5,  w3,  uxtw #1
        add             x6,  x2,  w6,  sxtw #1

        ld1             {v3.8h, v4.8h, v5.8h}, [x5] // padding_mask

        ld1r            {v28.8h}, [x6]
        bit             v0.16b,  v28.16b, v3.16b  // Pad v0-v2
        bit             v1.16b,  v28.16b, v4.16b
        bit             v2.16b,  v28.16b, v5.16b
4:
        // Filter one block
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        ext             v4.16b,  v1.16b,  v2.16b,  #2
        ext             v5.16b,  v0.16b,  v1.16b,  #4
        ext             v6.16b,  v1.16b,  v2.16b,  #4
        ext             v16.16b, v0.16b,  v1.16b,  #6
        ext             v17.16b, v1.16b,  v2.16b,  #6
        ext             v18.16b, v0.16b,  v1.16b,  #8
        ext             v19.16b, v1.16b,  v2.16b,  #8
        mul             v20.8h,  v0.8h,   v29.8h
        mla             v20.8h,  v3.8h,   v30.8h
        mla             v20.8h,  v5.8h,   v31.8h
        mla             v20.8h,  v16.8h,  v30.8h
        mla             v20.8h,  v18.8h,  v29.8h
        mul             v21.8h,  v1.8h,   v29.8h
        mla             v21.8h,  v4.8h,   v30.8h
        mla             v21.8h,  v6.8h,   v31.8h
        mla             v21.8h,  v17.8h,  v30.8h
        mla             v21.8h,  v19.8h,  v29.8h
        subs            w1,  w1,  #16
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v28.16b
        mov             v2.16b,  v28.16b
        urshr           v20.8h,  v20.8h,  #4
        urshr           v21.8h,  v21.8h,  #4
        sub             w3,  w3,  #16
        st1             {v20.8h, v21.8h}, [x0], #32
        b.le            9f
        // v0-v1[w3+1] is the last valid pixel; if (w3 + 1 > 0) we need to
        // filter properly once more - aka (w3 >= 0).
        cmp             w3,  #0
        b.ge            4b
5:
        // When w3 <= 0, all remaining pixels in v0-v1 are equal to the
        // last valid pixel - thus just output that without filtering.
        subs            w1,  w1,  #8
        st1             {v28.8h}, [x0], #16
        b.gt            5b
9:
        ret
endfunc

// void ipred_pixel_set_16bpc_neon(pixel *out, const pixel px,
//                                 const int n);
function ipred_pixel_set_16bpc_neon, export=1
        dup             v0.8h,   w1
1:
        subs            w2,  w2,  #8
        st1             {v0.8h}, [x0], #16
        b.gt            1b
        ret
endfunc

// void ipred_z1_fill1_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const top,
//                                const int width, const int height,
//                                const int dx, const int max_base_x);
function ipred_z1_fill1_16bpc_neon, export=1
        clz             w9,  w3
        adr             x8,  L(ipred_z1_fill1_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x8, w9, uxtw #1]
        add             x10, x2,  w6,  uxtw #1    // top[max_base_x]
        sub             x8,  x8,  w9,  uxtw
        ld1r            {v31.8h}, [x10]           // padding
        mov             w7,  w5
        mov             w15, #64
        br              x8
40:
        AARCH64_VALID_JUMP_TARGET
4:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            49f
        lsl             w8,  w8,  #1
        lsl             w10, w10, #1
        ldr             q0,  [x2, w8, uxtw]       // top[base]
        ldr             q2,  [x2, w10, uxtw]
        dup             v4.4h,   w9               // frac
        dup             v5.4h,   w11
        ext             v1.16b,  v0.16b,  v0.16b,  #2 // top[base+1]
        ext             v3.16b,  v2.16b,  v2.16b,  #2
        sub             v6.4h,   v1.4h,   v0.4h   // top[base+1]-top[base]
        sub             v7.4h,   v3.4h,   v2.4h
        ushll           v16.4s,  v0.4h,   #6      // top[base]*64
        ushll           v17.4s,  v2.4h,   #6
        smlal           v16.4s,  v6.4h,   v4.4h   // + top[base+1]*frac
        smlal           v17.4s,  v7.4h,   v5.4h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn           v17.4h,  v17.4s,  #6
        st1             {v16.4h}, [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.4h}, [x0], x1
        b.gt            4b
        ret

49:
        st1             {v31.4h}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.4h}, [x0], x1
        b.gt            49b
        ret

80:
        AARCH64_VALID_JUMP_TARGET
8:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            89f
        add             x8,  x2,  w8,  uxtw #1
        add             x10, x2,  w10, uxtw #1
        dup             v4.8h,   w9               // frac
        dup             v5.8h,   w11
        ld1             {v0.8h},  [x8]            // top[base]
        ld1             {v2.8h},  [x10]
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        ldr             h1, [x8, #16]
        ldr             h3, [x10, #16]
        dup             v6.8h,   w9               // 64 - frac
        dup             v7.8h,   w11
        ext             v1.16b,  v0.16b,  v1.16b,  #2 // top[base+1]
        ext             v3.16b,  v2.16b,  v3.16b,  #2
        umull           v16.4s,  v0.4h,   v6.4h   // top[base]*(64-frac)
        umlal           v16.4s,  v1.4h,   v4.4h   // + top[base+1]*frac
        umull2          v17.4s,  v0.8h,   v6.8h
        umlal2          v17.4s,  v1.8h,   v4.8h
        umull           v18.4s,  v2.4h,   v7.4h
        umlal           v18.4s,  v3.4h,   v5.4h
        umull2          v19.4s,  v2.8h,   v7.8h
        umlal2          v19.4s,  v3.8h,   v5.8h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn2          v16.8h,  v17.4s,  #6
        rshrn           v17.4h,  v18.4s,  #6
        rshrn2          v17.8h,  v19.4s,  #6
        st1             {v16.8h}, [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.8h}, [x0], x1
        b.gt            8b
        ret

89:
        st1             {v31.8h}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.8h}, [x0], x1
        b.gt            89b
        ret

160:
320:
640:
        AARCH64_VALID_JUMP_TARGET

        mov             w12, w3

        add             x13, x0,  x1
        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3,  uxtw #1
1:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            169f
        add             x8,  x2,  w8,  uxtw #1
        add             x10, x2,  w10, uxtw #1
        dup             v6.8h,   w9               // frac
        dup             v7.8h,   w11
        ld1             {v0.8h, v1.8h, v2.8h}, [x8],  #48 // top[base]
        ld1             {v3.8h, v4.8h, v5.8h}, [x10], #48
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v16.8h,  w9               // 64 - frac
        dup             v17.8h,  w11
        add             w7,  w7,  w5              // xpos += dx
2:
        ext             v18.16b, v0.16b,  v1.16b,  #2 // top[base+1]
        ext             v19.16b, v1.16b,  v2.16b,  #2
        ext             v20.16b, v3.16b,  v4.16b,  #2
        ext             v21.16b, v4.16b,  v5.16b,  #2
        subs            w3,  w3,  #16
        umull           v22.4s,  v0.4h,   v16.4h  // top[base]*(64-frac)
        umlal           v22.4s,  v18.4h,  v6.4h   // + top[base+1]*frac
        umull2          v23.4s,  v0.8h,   v16.8h
        umlal2          v23.4s,  v18.8h,  v6.8h
        umull           v24.4s,  v1.4h,   v16.4h
        umlal           v24.4s,  v19.4h,  v6.4h
        umull2          v25.4s,  v1.8h,   v16.8h
        umlal2          v25.4s,  v19.8h,  v6.8h
        umull           v26.4s,  v3.4h,   v17.4h
        umlal           v26.4s,  v20.4h,  v7.4h
        umull2          v27.4s,  v3.8h,   v17.8h
        umlal2          v27.4s,  v20.8h,  v7.8h
        umull           v28.4s,  v4.4h,   v17.4h
        umlal           v28.4s,  v21.4h,  v7.4h
        umull2          v29.4s,  v4.8h,   v17.8h
        umlal2          v29.4s,  v21.8h,  v7.8h
        rshrn           v22.4h,  v22.4s,  #6
        rshrn2          v22.8h,  v23.4s,  #6
        rshrn           v23.4h,  v24.4s,  #6
        rshrn2          v23.8h,  v25.4s,  #6
        rshrn           v24.4h,  v26.4s,  #6
        rshrn2          v24.8h,  v27.4s,  #6
        rshrn           v25.4h,  v28.4s,  #6
        rshrn2          v25.8h,  v29.4s,  #6
        st1             {v22.8h, v23.8h}, [x0],  #32
        st1             {v24.8h, v25.8h}, [x13], #32
        b.le            3f
        mov             v0.16b,  v2.16b
        ld1             {v1.8h, v2.8h}, [x8],  #32 // top[base]
        mov             v3.16b,  v5.16b
        ld1             {v4.8h, v5.8h}, [x10], #32
        b               2b

3:
        subs            w4,  w4,  #2
        b.le            9f
        add             x0,  x0,  x1
        add             x13, x13, x1
        mov             w3,  w12
        b               1b
9:
        ret

169:
        st1             {v31.8h}, [x0],  #16
        subs            w3,  w3,  #8
        st1             {v31.8h}, [x13], #16
        b.gt            169b
        subs            w4,  w4,  #2
        b.le            9b
        add             x0,  x0,  x1
        add             x13, x13, x1
        mov             w3,  w12
        b               169b

L(ipred_z1_fill1_tbl):
        .hword L(ipred_z1_fill1_tbl) - 640b
        .hword L(ipred_z1_fill1_tbl) - 320b
        .hword L(ipred_z1_fill1_tbl) - 160b
        .hword L(ipred_z1_fill1_tbl) -  80b
        .hword L(ipred_z1_fill1_tbl) -  40b
endfunc

function ipred_z1_fill2_16bpc_neon, export=1
        cmp             w3,  #8
        add             x10, x2,  w6,  uxtw       // top[max_base_x]
        ld1r            {v31.16b}, [x10]          // padding
        mov             w7,  w5
        mov             w15, #64
        b.eq            8f

4:      // w == 4
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            49f
        lsl             w8,  w8,  #1
        lsl             w10, w10, #1
        ldr             q0,  [x2, w8, uxtw]       // top[base]
        ldr             q2,  [x2, w10, uxtw]
        dup             v4.4h,   w9               // frac
        dup             v5.4h,   w11
        uzp2            v1.8h,   v0.8h,   v0.8h   // top[base+1]
        uzp1            v0.8h,   v0.8h,   v0.8h   // top[base]
        uzp2            v3.8h,   v2.8h,   v2.8h
        uzp1            v2.8h,   v2.8h,   v2.8h
        sub             v6.4h,   v1.4h,   v0.4h   // top[base+1]-top[base]
        sub             v7.4h,   v3.4h,   v2.4h
        ushll           v16.4s,  v0.4h,   #6      // top[base]*64
        ushll           v17.4s,  v2.4h,   #6
        smlal           v16.4s,  v6.4h,   v4.4h   // + top[base+1]*frac
        smlal           v17.4s,  v7.4h,   v5.4h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn           v17.4h,  v17.4s,  #6
        st1             {v16.4h}, [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.4h}, [x0], x1
        b.gt            4b
        ret

49:
        st1             {v31.4h}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.4h}, [x0], x1
        b.gt            49b
        ret

8:      // w == 8
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            89f
        add             x8,  x2,  w8,  uxtw #1
        add             x10, x2,  w10, uxtw #1
        dup             v4.8h,   w9               // frac
        dup             v5.8h,   w11
        ld1             {v0.8h, v1.8h},  [x8]     // top[base]
        ld1             {v2.8h, v3.8h},  [x10]
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v6.8h,   w9               // 64 - frac
        dup             v7.8h,   w11
        uzp2            v20.8h,  v0.8h,   v1.8h   // top[base+1]
        uzp1            v0.8h,   v0.8h,   v1.8h   // top[base]
        uzp2            v21.8h,  v2.8h,   v3.8h
        uzp1            v2.8h,   v2.8h,   v3.8h
        umull           v16.4s,  v0.4h,   v6.4h   // top[base]*(64-frac)
        umlal           v16.4s,  v20.4h,  v4.4h   // + top[base+1]*frac
        umull2          v17.4s,  v0.8h,   v6.8h
        umlal2          v17.4s,  v20.8h,  v4.8h
        umull           v18.4s,  v2.4h,   v7.4h
        umlal           v18.4s,  v21.4h,  v5.4h
        umull2          v19.4s,  v2.8h,   v7.8h
        umlal2          v19.4s,  v21.8h,  v5.8h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn2          v16.8h,  v17.4s,  #6
        rshrn           v17.4h,  v18.4s,  #6
        rshrn2          v17.8h,  v19.4s,  #6
        st1             {v16.8h}, [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.8h}, [x0], x1
        b.gt            8b
        ret

89:
        st1             {v31.8h}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.8h}, [x0], x1
        b.gt            89b
        ret
endfunc

// void ipred_reverse_16bpc_neon(pixel *dst, const pixel *const src,
//                               const int n);
function ipred_reverse_16bpc_neon, export=1
        sub             x1,  x1,  #16
        add             x3,  x0,  #8
        mov             x4,  #16
1:
        ld1             {v0.8h}, [x1]
        subs            w2,  w2,  #8
        rev64           v0.8h,  v0.8h
        sub             x1,  x1,  #16
        st1             {v0.d}[1], [x0], x4
        st1             {v0.d}[0], [x3], x4
        b.gt            1b
        ret
endfunc

const increments
        .short          0,  1,  2,  3,  4,  5,  6,  7
endconst

// void ipred_z2_fill1_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const top,
//                                const pixel *const left,
//                                const int width, const int height,
//                                const int dx, const int dy);
function ipred_z2_fill1_16bpc_neon, export=1
        clz             w10, w4
        adr             x9,  L(ipred_z2_fill1_tbl)
        sub             w10, w10, #25
        ldrh            w10, [x9, w10, uxtw #1]
        mov             w8,  #(1 << 6)            // xpos = 1 << 6
        sub             x9,  x9,  w10, uxtw
        sub             w8,  w8,  w6              // xpos -= dx

        movrel          x11, increments
        ld1             {v31.8h},  [x11]          // increments
        neg             w7,  w7                   // -dy

        br              x9
40:
        AARCH64_VALID_JUMP_TARGET

        dup             v30.4h,  w7               // -dy
        movi            v17.8b,  #1

        mul             v16.4h,  v31.4h,  v30.4h  // {0,1,2,3}* -dy
        movi            v25.8h,  #0x3e
        add             v30.4h,  v16.4h,  v30.4h  // -= dy

        // Worst case height for w=4 is 16, but we need at least h+1 elements
        ld1             {v0.8h, v1.8h, v2.8h}, [x3]    // left[]

        movi            v26.8h,  #64
        movi            v19.16b, #4

        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v30.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 1

        movi            v23.4h,  #1, lsl #8
        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        zip1            v29.8b,  v29.8b,  v29.8b  // duplicate elements
        movi            v17.8b,  #2
        add             v29.8b,  v29.8b,  v23.8b  // 2*base, 2*base+1, ...

        add             v30.8b,  v29.8b,  v17.8b  // base_y + 1 (*2)
        add             v28.8b,  v29.8b,  v19.8b  // base_y + 2 (*2)

        tbl             v18.8b, {v0.16b}, v29.8b  // left[base_y]

        trn1            v30.2d,  v30.2d,  v28.2d  // base_y + 1, base_y + 2

        sub             v28.4h,  v26.4h,  v27.4h  // 64 - frac_y

        trn1            v31.2d,  v31.2d,  v31.2d  // {0,1,2,3,0,1,2,3}

        trn1            v27.2d,  v27.2d,  v27.2d  // frac_y
        trn1            v28.2d,  v28.2d,  v28.2d  // 64 - frac_y

        movi            v29.16b, #4
4:
        asr             w9,  w8,  #6              // base_x
        dup             v16.4h,  w8               // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-4                  // base_x <= -4
        asr             w11, w8,  #6              // base_x
        b.le            49f

        lsl             w9,  w9,  #1
        lsl             w11, w11, #1

        dup             v17.4h,  w8               // xpos

        ldr             q4,  [x2, w9, sxtw]       // top[base_x]
        ldr             q6,  [x2, w11, sxtw]

        trn1            v16.2d,  v16.2d,  v17.2d  // xpos

        // Cut corners here; only doing tbl over v0-v1 here; we only
        // seem to need the last pixel, from v2, after skipping to the
        // left-only codepath below.
        tbl             v19.16b, {v0.16b, v1.16b}, v30.16b // left[base_y+1], left[base_y+2]

        sshr            v20.8h,  v16.8h,  #6      // first base_x for each row

        ext             v5.16b,  v4.16b,  v4.16b,  #2 // top[base_x+1]
        ext             v7.16b,  v6.16b,  v6.16b,  #2

        and             v16.16b, v16.16b, v25.16b // frac_x

        trn1            v18.2d,  v18.2d,  v19.2d  // left[base_y], left[base_y+1]

        trn1            v4.2d,   v4.2d,   v6.2d   // top[base_x]
        trn1            v5.2d,   v5.2d,   v7.2d   // top[base_x+1]

        sub             v17.8h,  v26.8h,  v16.8h  // 64 - frac_x

        add             v20.8h,  v20.8h,  v31.8h  // actual base_x

        umull           v21.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v21.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v22.4s,  v18.8h,  v28.8h
        umlal2          v22.4s,  v19.8h,  v27.8h

        umull           v23.4s,  v4.4h,   v17.4h  // top[base_x]-*(64-frac_x)
        umlal           v23.4s,  v5.4h,   v16.4h  // + top[base_x+1]*frac_x
        umull2          v24.4s,  v4.8h,   v17.8h
        umlal2          v24.4s,  v5.8h,   v16.8h

        cmge            v20.8h,  v20.8h,  #0

        rshrn           v21.4h,  v21.4s,  #6
        rshrn2          v21.8h,  v22.4s,  #6
        rshrn           v22.4h,  v23.4s,  #6
        rshrn2          v22.8h,  v24.4s,  #6

        bit             v21.16b, v22.16b, v20.16b

        st1             {v21.d}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v21.d}[1], [x0], x1
        b.le            9f

        ext             v18.16b, v19.16b, v19.16b, #8
        add             v30.16b, v30.16b, v29.16b // base_y += 2 (*2)
        b               4b

49:
        tbl             v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+1], left[base_y+2]

        trn1            v18.2d,  v18.2d,  v19.2d  // left[base_y], left[base_y+1]

        umull           v20.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v20.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v21.4s,  v18.8h,  v28.8h
        umlal2          v21.4s,  v19.8h,  v27.8h

        rshrn           v20.4h,  v20.4s,  #6
        rshrn2          v20.8h,  v21.4s,  #6

        st1             {v20.d}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v20.d}[1], [x0], x1
        b.le            9f

        ext             v18.16b, v19.16b, v19.16b, #8
        add             v30.16b, v30.16b, v29.16b // base_y += 2 (*2)
        b               49b

9:
        ret

80:
        AARCH64_VALID_JUMP_TARGET

        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

        dup             v18.8h,  w7               // -dy
        add             x3,  x3,  #2              // Skip past left[0]

        mul             v16.8h,  v31.8h,  v18.8h  // {0,1,2,3,4,5,6,7}* -dy
        movi            v25.8h,  #0x3e
        add             v16.8h,  v16.8h,  v18.8h  // -= dy

        // Worst case height for w=8 is 32.
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x3] // left[]
        ld1r            {v15.8h}, [x2]            // left[0] == top[0]

        movi            v26.8h,  #64
        movi            v19.16b, #4

        shrn            v29.8b,  v16.8h,  #6      // ypos >> 6
        and             v27.16b, v16.16b, v25.16b // frac_y

        movi            v23.8h,  #1, lsl #8
        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        mov             v18.16b, v15.16b          // left[0]
        zip1            v29.16b, v29.16b, v29.16b // duplicate elements
        movi            v17.16b, #2
        add             v29.16b, v29.16b, v23.16b // 2*base, 2*base+1, ...

        // Cut corners here; for the first row we don't expect to need to
        // read outside of v0.
        tbx             v18.16b, {v0.16b}, v29.16b // left[base_y]

        add             v30.16b, v29.16b, v19.16b // base_y + 2 (*2)
        add             v29.16b, v29.16b, v17.16b // base_y + 1 (*2)

        sub             v28.8h,  v26.8h,  v27.8h  // 64 - frac_y

        movi            v24.16b, #4
8:
        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-16                 // base_x <= -16
        asr             w11, w8,  #6              // base_x
        b.le            89f

        dup             v17.8h,   w8              // xpos

        add             x9,  x2,  w9,  sxtw #1
        add             x11, x2,  w11, sxtw #1

        ld1             {v4.8h, v5.8h}, [x9]      // top[base_x]
        mov             v19.16b, v15.16b          // left[0]
        ld1             {v6.8h, v7.8h}, [x11]

        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]

        mov             v20.16b, v15.16b          // left[0]

        sshr            v21.8h,  v16.8h,  #6      // first base_x
        sshr            v22.8h,  v17.8h,  #6

        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b // left[base_y+2]

        ext             v5.16b,  v4.16b,  v5.16b,  #2 // top[base_x+1]
        ext             v7.16b,  v6.16b,  v7.16b,  #2

        and             v16.16b, v16.16b, v25.16b // frac_x
        and             v17.16b, v17.16b, v25.16b

        umull           v10.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v10.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y

        sub             v8.8h,   v26.8h,  v16.8h  // 64 - frac_x
        sub             v9.8h,   v26.8h,  v17.8h

        umull2          v11.4s,  v18.8h,  v28.8h
        umlal2          v11.4s,  v19.8h,  v27.8h

        add             v21.8h,  v21.8h,  v31.8h  // actual base_x
        add             v22.8h,  v22.8h,  v31.8h

        umull           v12.4s,  v19.4h,  v28.4h
        umlal           v12.4s,  v20.4h,  v27.4h
        umull2          v13.4s,  v19.8h,  v28.8h
        umlal2          v13.4s,  v20.8h,  v27.8h

        rshrn           v10.4h,  v10.4s,  #6
        rshrn2          v10.8h,  v11.4s,  #6
        rshrn           v11.4h,  v12.4s,  #6
        rshrn2          v11.8h,  v13.4s,  #6

        umull           v12.4s,  v4.4h,   v8.4h   // top[base_x]-*(64-frac_x)
        umlal           v12.4s,  v5.4h,   v16.4h  // + top[base_x+1]*frac_x
        umull2          v13.4s,  v4.8h,   v8.8h
        umlal2          v13.4s,  v5.8h,   v16.8h
        umull           v14.4s,  v6.4h,   v9.4h
        umlal           v14.4s,  v7.4h,   v17.4h
        umull2          v18.4s,  v6.8h,   v9.8h
        umlal2          v18.4s,  v7.8h,   v17.8h

        cmge            v21.8h,  v21.8h,  #0
        cmge            v22.8h,  v22.8h,  #0

        rshrn           v12.4h,  v12.4s,  #6
        rshrn2          v12.8h,  v13.4s,  #6
        rshrn           v13.4h,  v14.4s,  #6
        rshrn2          v13.8h,  v18.4s,  #6

        bit             v10.16b, v12.16b, v21.16b
        bit             v11.16b, v13.16b, v22.16b

        st1             {v10.8h}, [x0], x1
        subs            w5,  w5,  #2
        sub             w8,  w8,  w6              // xpos -= dx
        st1             {v11.8h}, [x0], x1
        b.le            9f

        mov             v18.16b, v20.16b
        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        add             v30.16b, v30.16b, v24.16b // base_y += 2 (*2)
        b               8b

89:
        mov             v19.16b, v15.16b
        mov             v20.16b, v15.16b
        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]
        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b // left[base_y+2]

        umull           v4.4s,   v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v4.4s,   v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v5.4s,   v18.8h,  v28.8h
        umlal2          v5.4s,   v19.8h,  v27.8h
        umull           v6.4s,   v19.4h,  v28.4h
        umlal           v6.4s,   v20.4h,  v27.4h
        umull2          v7.4s,   v19.8h,  v28.8h
        umlal2          v7.4s,   v20.8h,  v27.8h

        rshrn           v4.4h,   v4.4s,   #6
        rshrn2          v4.8h,   v5.4s,   #6
        rshrn           v5.4h,   v6.4s,   #6
        rshrn2          v5.8h,   v7.4s,   #6

        st1             {v4.8h}, [x0], x1
        subs            w5,  w5,  #2
        st1             {v5.8h}, [x0], x1
        b.le            9f

        mov             v18.16b, v20.16b
        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        add             v30.16b, v30.16b, v24.16b // base_y += 2 (*2)
        b               89b

9:
        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40
        ret

160:
320:
640:
        AARCH64_VALID_JUMP_TARGET

        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

        dup             v25.8h,  w7               // -dy
        add             x3,  x3,  #2              // Skip past left[0]

        add             x13, x0,  x1              // alternating row
        lsl             x1,  x1,  #1              // stride *= 2
        sub             x1,  x1,  w4,  uxtw #1    // stride -= width

        movi            v11.8h,  #8
        mul             v26.8h,  v31.8h,  v25.8h  // {0,1,2,3,4,5,6,7}* -dy
        add             v26.8h,  v26.8h,  v25.8h  // -= dy
        mul             v25.8h,  v25.8h,  v11.8h  // -8*dy

        // Worst case height is 64, but we can only fit 32 pixels into
        // v0-v3 usable within one tbx instruction. As long as base_y is
        // up to 32, we use tbx.
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x3] // left[]
        ld1r            {v15.8h}, [x2]            // left[0] == top[0]

        mov             w12, w4                   // orig w
        neg             w14, w4                   // -w

1:
        mov             v23.16b, v26.16b          // reset ypos

        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  w14                  // base_x <= -2*w
        asr             w11, w8,  #6              // base_x
        b.le            169f

        dup             v17.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx

        add             x9,  x2,  w9,  sxtw #1
        add             x11, x2,  w11, sxtw #1

        sshr            v21.8h,  v16.8h,  #6      // first base_x
        sshr            v22.8h,  v17.8h,  #6

        ld1             {v4.8h}, [x9], #16        // top[base_x]
        ld1             {v6.8h}, [x11], #16

        movi            v10.8h,  #0x3e
        movi            v11.8h,  #64

        and             v16.16b, v16.16b, v10.16b // frac_x
        and             v17.16b, v17.16b, v10.16b

        sub             v8.8h,   v11.8h,  v16.8h  // 64 - frac_x
        sub             v9.8h,   v11.8h,  v17.8h

        add             v21.8h,  v21.8h,  v31.8h  // actual base_x
        add             v22.8h,  v22.8h,  v31.8h

2:
        smov            w10,     v22.h[0]

        shrn            v29.8b,  v23.8h,  #6      // ypos >> 6
        movi            v12.8h,  #64
        cmp             w10, #0                   // base_x (bottom left) >= 0
        smov            w10,     v29.b[0]         // base_y[0]
        movi            v10.8h,  #0x3e

        b.ge            4f
        and             v27.16b, v23.16b, v10.16b // frac_y
        cmp             w10,     #(32-3)

        mov             v18.16b, v15.16b          // left[0]
        sub             v28.8h,  v12.8h,  v27.8h  // 64 - frac_y
        b.gt            22f

21:
        // base_y < 32, using tbx
        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        movi            v11.8h,  #1, lsl #8
        zip1            v29.16b, v29.16b, v29.16b // duplicate elements
        add             v29.16b, v29.16b, v11.16b // 2*base, 2*base+1, ...

        movi            v13.16b, #2

        tbx             v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y]

        add             v29.16b, v29.16b, v13.16b // base_y + 1 (*2)
        mov             v19.16b, v15.16b          // left[0]

        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]

        add             v29.16b, v29.16b, v13.16b // base_y + 2 (*2)
        mov             v20.16b, v15.16b          // left[0]

        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+2]

        b               23f

22:
        // base_y >= 32, using separate loads.
        smov            w15,     v29.b[1]
        smov            w16,     v29.b[2]
        add             x10, x3,  w10, sxtw #1
        smov            w17,     v29.b[3]
        add             x15, x3,  w15, sxtw #1
        ld3             {v18.h, v19.h, v20.h}[0], [x10]
        smov            w10,     v29.b[4]
        add             x16, x3,  w16, sxtw #1
        ld3             {v18.h, v19.h, v20.h}[1], [x15]
        smov            w15,     v29.b[5]
        add             x17, x3,  w17, sxtw #1
        ld3             {v18.h, v19.h, v20.h}[2], [x16]
        smov            w16,     v29.b[6]
        add             x10, x3,  w10, sxtw #1
        ld3             {v18.h, v19.h, v20.h}[3], [x17]
        smov            w17,     v29.b[7]
        add             x15, x3,  w15, sxtw #1
        add             x16, x3,  w16, sxtw #1
        ld3             {v18.h, v19.h, v20.h}[4], [x10]
        add             x17, x3,  w17, sxtw #1
        ld3             {v18.h, v19.h, v20.h}[5], [x15]
        ld3             {v18.h, v19.h, v20.h}[6], [x16]
        ld3             {v18.h, v19.h, v20.h}[7], [x17]

23:

        ld1             {v5.8h}, [x9], #16        // top[base_x]
        ld1             {v7.8h}, [x11], #16

        add             v23.8h,  v23.8h,  v25.8h  // ypos -= 8*dy

        umull           v10.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v10.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v11.4s,  v18.8h,  v28.8h
        umlal2          v11.4s,  v19.8h,  v27.8h
        umull           v12.4s,  v19.4h,  v28.4h
        umlal           v12.4s,  v20.4h,  v27.4h
        umull2          v13.4s,  v19.8h,  v28.8h
        umlal2          v13.4s,  v20.8h,  v27.8h

        ext             v18.16b, v4.16b,  v5.16b,  #2 // top[base_x+1]
        ext             v19.16b, v6.16b,  v7.16b,  #2

        rshrn           v10.4h,  v10.4s,  #6
        rshrn2          v10.8h,  v11.4s,  #6
        rshrn           v11.4h,  v12.4s,  #6
        rshrn2          v11.8h,  v13.4s,  #6

        umull           v12.4s,  v4.4h,   v8.4h   // top[base_x]-*(64-frac_x)
        umlal           v12.4s,  v18.4h,  v16.4h  // + top[base_x+1]*frac_x
        umull2          v13.4s,  v4.8h,   v8.8h
        umlal2          v13.4s,  v18.8h,  v16.8h
        umull           v14.4s,  v6.4h,   v9.4h
        umlal           v14.4s,  v19.4h,  v17.4h
        umull2          v20.4s,  v6.8h,   v9.8h
        umlal2          v20.4s,  v19.8h,  v17.8h

        cmge            v18.8h,  v21.8h,  #0
        cmge            v19.8h,  v22.8h,  #0

        rshrn           v12.4h,  v12.4s,  #6
        rshrn2          v12.8h,  v13.4s,  #6
        rshrn           v13.4h,  v14.4s,  #6
        rshrn2          v13.8h,  v20.4s,  #6

        bit             v10.16b, v12.16b, v18.16b
        bit             v11.16b, v13.16b, v19.16b

        st1             {v10.8h}, [x0], #16
        subs            w4,  w4,  #8
        st1             {v11.8h}, [x13], #16
        b.le            3f

        movi            v10.8h,  #8
        mov             v4.16b,  v5.16b
        mov             v6.16b,  v7.16b
        add             v21.8h,  v21.8h,  v10.8h  // base_x += 8
        add             v22.8h,  v22.8h,  v10.8h
        b               2b

3:
        subs            w5,  w5,  #2
        b.le            9f
        movi            v10.8h, #128
        add             x0,  x0,  x1
        add             x13, x13, x1
        mov             w4,  w12                  // reset w
        add             v26.8h,  v26.8h,  v10.8h  // ypos += 2*(1<<6)
        b               1b

4:      // The rest of the row only predicted from top[]
        ld1             {v5.8h}, [x9], #16        // top[base_x]
        ld1             {v7.8h}, [x11], #16

        ext             v18.16b, v4.16b,  v5.16b,  #2 // top[base_x+1]
        ext             v19.16b, v6.16b,  v7.16b,  #2

        umull           v12.4s,  v4.4h,   v8.4h   // top[base_x]-*(64-frac_x)
        umlal           v12.4s,  v18.4h,  v16.4h  // + top[base_x+1]*frac_x
        umull2          v13.4s,  v4.8h,   v8.8h
        umlal2          v13.4s,  v18.8h,  v16.8h
        umull           v14.4s,  v6.4h,   v9.4h
        umlal           v14.4s,  v19.4h,  v17.4h
        umull2          v20.4s,  v6.8h,   v9.8h
        umlal2          v20.4s,  v19.8h,  v17.8h

        rshrn           v12.4h,  v12.4s,  #6
        rshrn2          v12.8h,  v13.4s,  #6
        rshrn           v13.4h,  v14.4s,  #6
        rshrn2          v13.8h,  v20.4s,  #6

        st1             {v12.8h}, [x0], #16
        subs            w4,  w4,  #8
        st1             {v13.8h}, [x13], #16
        b.le            3b

        mov             v4.16b,  v5.16b
        mov             v6.16b,  v7.16b
        b               4b

169:    // The rest of the block only predicted from left[]
        add             x1,  x1,  w4,  uxtw #1    // restore stride
        mov             w12, w5                   // orig remaining h
1:
        movi            v12.8h,  #64
        movi            v10.8h,  #0x3e

        shrn            v29.8b,  v23.8h,  #6      // ypos >> 6
        and             v27.16b, v23.16b, v10.16b // frac_y

        smov            w10,     v29.b[0]         // base_y[0]

        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        movi            v11.8h,  #1, lsl #8
        zip1            v29.16b, v29.16b, v29.16b // duplicate elements
        add             v23.8h,  v23.8h,  v25.8h  // ypos -= 8*dy
        add             v29.16b, v29.16b, v11.16b // 2*base, 2*base+1, ...

        cmp             w10,     #(32-1)

        mov             v18.16b, v15.16b          // left[0]
        movi            v21.16b, #2

        sub             v28.8h,  v12.8h,  v27.8h  // 64 - frac_y

        b.gt            31f

        tbx             v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y]
        add             v29.16b, v29.16b, v21.16b // base_y + 1 (*2)

2:
        // base_y < 32, using tbx.
        smov            w10,     v29.b[0]         // base_y[0]
        mov             v19.16b, v15.16b          // left[0]
        cmp             w10,     #(64-4)
        b.gt            32f
        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]
        add             v29.16b, v29.16b, v21.16b // base_y + 2 (*2)
        mov             v20.16b, v15.16b          // left[0]
        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+2]
        add             v29.16b, v29.16b, v21.16b // next base_y

        umull           v10.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v10.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v11.4s,  v18.8h,  v28.8h
        umlal2          v11.4s,  v19.8h,  v27.8h
        umull           v12.4s,  v19.4h,  v28.4h
        umlal           v12.4s,  v20.4h,  v27.4h
        umull2          v13.4s,  v19.8h,  v28.8h
        umlal2          v13.4s,  v20.8h,  v27.8h

        rshrn           v10.4h,  v10.4s,  #6
        rshrn2          v10.8h,  v11.4s,  #6
        rshrn           v11.4h,  v12.4s,  #6
        rshrn2          v11.8h,  v13.4s,  #6

        st1             {v10.8h}, [x0], x1
        subs            w5,  w5,  #2
        st1             {v11.8h}, [x13], x1
        b.le            4f
        mov             v18.16b, v20.16b
        b               2b

31:     // base_y >= 32, using separate loads, loading v18 if we had to bail
        // in the prologue.
        smov            w10,     v29.b[0]
        smov            w15,     v29.b[2]
        movi            v21.16b, #2
        smov            w16,     v29.b[4]
        add             x10, x3,  w10, sxtw
        smov            w17,     v29.b[6]
        add             x15, x3,  w15, sxtw
        ld1             {v18.h}[0], [x10]
        smov            w10,     v29.b[8]
        add             x16, x3,  w16, sxtw
        ld1             {v18.h}[1], [x15]
        smov            w15,     v29.b[10]
        add             x17, x3,  w17, sxtw
        ld1             {v18.h}[2], [x16]
        smov            w16,     v29.b[12]
        add             x10, x3,  w10, sxtw
        ld1             {v18.h}[3], [x17]
        smov            w17,     v29.b[14]
        add             x15, x3,  w15, sxtw
        add             x16, x3,  w16, sxtw
        ld1             {v18.h}[4], [x10]
        add             x17, x3,  w17, sxtw
        ld1             {v18.h}[5], [x15]
        add             v29.16b, v29.16b, v21.16b // next base_y
        ld1             {v18.h}[6], [x16]
        ld1             {v18.h}[7], [x17]

32:     // base_y >= 32, using separate loads.
        cmp             w5,  #4
        b.lt            34f
33:     // h >= 4, preserving v18 from the previous round, loading v19-v22.
        smov            w10,     v29.b[0]
        subs            w5,  w5,  #4
        smov            w15,     v29.b[2]
        movi            v10.16b, #8
        smov            w16,     v29.b[4]
        add             x10, x3,  w10, sxtw
        smov            w17,     v29.b[6]
        add             x15, x3,  w15, sxtw
        ld4             {v19.h, v20.h, v21.h, v22.h}[0], [x10]
        smov            w10,     v29.b[8]
        add             x16, x3,  w16, sxtw
        ld4             {v19.h, v20.h, v21.h, v22.h}[1], [x15]
        smov            w15,     v29.b[10]
        add             x17, x3,  w17, sxtw
        ld4             {v19.h, v20.h, v21.h, v22.h}[2], [x16]
        smov            w16,     v29.b[12]
        add             x10, x3,  w10, sxtw
        ld4             {v19.h, v20.h, v21.h, v22.h}[3], [x17]
        smov            w17,     v29.b[14]
        add             x15, x3,  w15, sxtw
        add             x16, x3,  w16, sxtw
        ld4             {v19.h, v20.h, v21.h, v22.h}[4], [x10]
        add             x17, x3,  w17, sxtw
        ld4             {v19.h, v20.h, v21.h, v22.h}[5], [x15]
        ld4             {v19.h, v20.h, v21.h, v22.h}[6], [x16]
        add             v29.16b, v29.16b, v10.16b // next base_y
        ld4             {v19.h, v20.h, v21.h, v22.h}[7], [x17]

        umull           v10.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v10.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v11.4s,  v18.8h,  v28.8h
        umlal2          v11.4s,  v19.8h,  v27.8h
        umull           v12.4s,  v19.4h,  v28.4h
        umlal           v12.4s,  v20.4h,  v27.4h
        umull2          v13.4s,  v19.8h,  v28.8h
        umlal2          v13.4s,  v20.8h,  v27.8h

        rshrn           v10.4h,  v10.4s,  #6
        rshrn2          v10.8h,  v11.4s,  #6
        rshrn           v11.4h,  v12.4s,  #6
        rshrn2          v11.8h,  v13.4s,  #6

        umull           v12.4s,  v20.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v12.4s,  v21.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v13.4s,  v20.8h,  v28.8h
        umlal2          v13.4s,  v21.8h,  v27.8h
        umull           v14.4s,  v21.4h,  v28.4h
        umlal           v14.4s,  v22.4h,  v27.4h
        umull2          v18.4s,  v21.8h,  v28.8h
        umlal2          v18.4s,  v22.8h,  v27.8h

        rshrn           v12.4h,  v12.4s,  #6
        rshrn2          v12.8h,  v13.4s,  #6
        rshrn           v13.4h,  v14.4s,  #6
        rshrn2          v13.8h,  v18.4s,  #6

        st1             {v10.8h}, [x0],  x1
        cmp             w5,  #2
        st1             {v11.8h}, [x13], x1
        st1             {v12.8h}, [x0],  x1
        st1             {v13.8h}, [x13], x1
        b.lt            4f
        mov             v18.16b, v22.16b
        b.gt            33b

34:     // h == 2, preserving v18 from the previous round, loading v19-v20.
        smov            w10,     v29.b[0]
        smov            w15,     v29.b[2]
        movi            v21.16b, #4
        smov            w16,     v29.b[4]
        add             x10, x3,  w10, sxtw
        smov            w17,     v29.b[6]
        add             x15, x3,  w15, sxtw
        ld2             {v19.h, v20.h}[0], [x10]
        smov            w10,     v29.b[8]
        add             x16, x3,  w16, sxtw
        ld2             {v19.h, v20.h}[1], [x15]
        smov            w15,     v29.b[10]
        add             x17, x3,  w17, sxtw
        ld2             {v19.h, v20.h}[2], [x16]
        smov            w16,     v29.b[12]
        add             x10, x3,  w10, sxtw
        ld2             {v19.h, v20.h}[3], [x17]
        smov            w17,     v29.b[14]
        add             x15, x3,  w15, sxtw
        add             x16, x3,  w16, sxtw
        ld2             {v19.h, v20.h}[4], [x10]
        add             x17, x3,  w17, sxtw
        ld2             {v19.h, v20.h}[5], [x15]
        ld2             {v19.h, v20.h}[6], [x16]
        add             v29.16b, v29.16b, v21.16b // next base_y
        ld2             {v19.h, v20.h}[7], [x17]

        umull           v10.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v10.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v11.4s,  v18.8h,  v28.8h
        umlal2          v11.4s,  v19.8h,  v27.8h
        umull           v12.4s,  v19.4h,  v28.4h
        umlal           v12.4s,  v20.4h,  v27.4h
        umull2          v13.4s,  v19.8h,  v28.8h
        umlal2          v13.4s,  v20.8h,  v27.8h

        rshrn           v10.4h,  v10.4s,  #6
        rshrn2          v10.8h,  v11.4s,  #6
        rshrn           v11.4h,  v12.4s,  #6
        rshrn2          v11.8h,  v13.4s,  #6

        st1             {v10.8h}, [x0], x1
        st1             {v11.8h}, [x13], x1
        // The h==2 case only happens once at the end, if at all.

4:
        subs            w4,  w4,  #8
        b.le            9f

        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        lsl             x1,  x1,  #1
        add             x0,  x0,  #16
        add             x13, x13, #16
        mov             w5,  w12                  // reset h
        b               1b

9:
        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40
        ret

L(ipred_z2_fill1_tbl):
        .hword L(ipred_z2_fill1_tbl) - 640b
        .hword L(ipred_z2_fill1_tbl) - 320b
        .hword L(ipred_z2_fill1_tbl) - 160b
        .hword L(ipred_z2_fill1_tbl) -  80b
        .hword L(ipred_z2_fill1_tbl) -  40b
endfunc

function ipred_z2_fill2_16bpc_neon, export=1
        cmp             w4,  #8
        mov             w8,  #(2 << 6)            // xpos = 2 << 6
        sub             w8,  w8,  w6              // xpos -= dx

        movrel          x11, increments
        ld1             {v31.8h},  [x11]          // increments
        neg             w7,  w7                   // -dy
        b.eq            80f

40:
        dup             v30.4h,  w7               // -dy
        movi            v17.8b,  #1

        mul             v16.4h,  v31.4h,  v30.4h  // {0,1,2,3}* -dy
        movi            v25.8h,  #0x3e
        add             v30.4h,  v16.4h,  v30.4h  // -= dy

        // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
        // from left.
        ld1             {v0.8h, v1.8h}, [x3]      // left[]

        movi            v26.8h,  #64
        movi            v19.16b, #4

        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v30.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 1

        movi            v23.4h,  #1, lsl #8
        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        zip1            v29.8b,  v29.8b,  v29.8b  // duplicate elements
        movi            v17.8b,  #2
        add             v29.8b,  v29.8b,  v23.8b  // 2*base, 2*base+1, ...

        add             v30.8b,  v29.8b,  v17.8b  // base_y + 1 (*2)
        add             v28.8b,  v29.8b,  v19.8b  // base_y + 2 (*2)

        tbl             v18.8b, {v0.16b}, v29.8b  // left[base_y]

        trn1            v30.2d,  v30.2d,  v28.2d  // base_y + 1, base_y + 2

        sub             v28.4h,  v26.4h,  v27.4h  // 64 - frac_y

        trn1            v31.2d,  v31.2d,  v31.2d  // {0,1,2,3,0,1,2,3}

        trn1            v27.2d,  v27.2d,  v27.2d  // frac_y
        trn1            v28.2d,  v28.2d,  v28.2d  // 64 - frac_y

        movi            v29.16b, #4
        add             v31.8h,  v31.8h,  v31.8h  // {0,2,4,6,0,2,4,6}
4:
        asr             w9,  w8,  #6              // base_x
        dup             v16.4h,  w8               // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-8                  // base_x <= -8
        asr             w11, w8,  #6              // base_x
        b.le            49f

        lsl             w9,  w9,  #1
        lsl             w11, w11, #1

        dup             v17.4h,  w8               // xpos

        ldr             q4,  [x2, w9, sxtw]       // top[base_x]
        ldr             q6,  [x2, w11, sxtw]

        trn1            v16.2d,  v16.2d,  v17.2d  // xpos

        tbl             v19.16b, {v0.16b, v1.16b}, v30.16b // left[base_y+1], left[base_y+2]

        sshr            v20.8h,  v16.8h,  #6      // first base_x for each row

        uzp2            v5.8h,   v4.8h,   v6.8h   // top[base_x+1]
        uzp1            v4.8h,   v4.8h,   v6.8h   // top[base_x]

        and             v16.16b, v16.16b, v25.16b // frac_x

        trn1            v18.2d,  v18.2d,  v19.2d  // left[base_y], left[base_y+1]

        sub             v17.8h,  v26.8h,  v16.8h  // 64 - frac_x

        add             v20.8h,  v20.8h,  v31.8h  // actual base_x

        umull           v21.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v21.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v22.4s,  v18.8h,  v28.8h
        umlal2          v22.4s,  v19.8h,  v27.8h

        umull           v23.4s,  v4.4h,   v17.4h  // top[base_x]-*(64-frac_x)
        umlal           v23.4s,  v5.4h,   v16.4h  // + top[base_x+1]*frac_x
        umull2          v24.4s,  v4.8h,   v17.8h
        umlal2          v24.4s,  v5.8h,   v16.8h

        cmge            v20.8h,  v20.8h,  #0

        rshrn           v21.4h,  v21.4s,  #6
        rshrn2          v21.8h,  v22.4s,  #6
        rshrn           v22.4h,  v23.4s,  #6
        rshrn2          v22.8h,  v24.4s,  #6

        bit             v21.16b, v22.16b, v20.16b

        st1             {v21.d}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v21.d}[1], [x0], x1
        b.le            9f

        ext             v18.16b, v19.16b, v19.16b, #8
        add             v30.16b, v30.16b, v29.16b // base_y += 2 (*2)
        b               4b

49:
        tbl             v19.16b, {v0.16b, v1.16b}, v30.16b // left[base_y+1], left[base_y+2]

        trn1            v18.2d,  v18.2d,  v19.2d  // left[base_y], left[base_y+1]

        umull           v20.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v20.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v21.4s,  v18.8h,  v28.8h
        umlal2          v21.4s,  v19.8h,  v27.8h

        rshrn           v20.4h,  v20.4s,  #6
        rshrn2          v20.8h,  v21.4s,  #6

        st1             {v20.d}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v20.d}[1], [x0], x1
        b.le            9f

        ext             v18.16b, v19.16b, v19.16b, #8
        add             v30.16b, v30.16b, v29.16b // base_y += 2 (*2)
        b               49b

9:
        ret

80:
        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

        dup             v18.8h,  w7               // -dy
        movi            v17.8b,  #1

        mul             v16.8h,  v31.8h,  v18.8h  // {0,1,2,3,4,5,6,7}* -dy
        movi            v25.8h,  #0x3e
        add             v16.8h,  v16.8h,  v18.8h  // -= dy

        // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
        // from left.
        ld1             {v0.8h, v1.8h}, [x3]      // left[]

        movi            v26.8h,  #64
        movi            v19.16b, #4

        shrn            v29.8b,  v16.8h,  #6      // ypos >> 6
        and             v27.16b, v16.16b, v25.16b // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 1

        movi            v23.8h,  #1, lsl #8
        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        zip1            v29.16b, v29.16b, v29.16b // duplicate elements
        movi            v17.16b, #2
        add             v29.16b, v29.16b, v23.16b // 2*base, 2*base+1, ...

        // Cut corners here; for the first row we don't expect to need to
        // read outside of v0.
        tbl             v18.16b, {v0.16b}, v29.16b // left[base_y]

        add             v30.16b, v29.16b, v19.16b // base_y + 2 (*2)
        add             v29.16b, v29.16b, v17.16b // base_y + 1 (*2)

        sub             v28.8h,  v26.8h,  v27.8h  // 64 - frac_y

        movi            v24.16b, #4
        add             v31.16b, v31.16b, v31.16b // {0,2,4,6,8,10,12,14,0,2,4,6,8,10,12,14}
8:
        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-16                 // base_x <= -16
        asr             w11, w8,  #6              // base_x
        b.le            89f

        dup             v17.8h,   w8              // xpos

        add             x9,  x2,  w9,  sxtw #1
        add             x11, x2,  w11, sxtw #1

        ld1             {v4.8h, v5.8h}, [x9]      // top[base_x]
        ld1             {v6.8h, v7.8h}, [x11]

        tbl             v19.16b, {v0.16b, v1.16b}, v29.16b // left[base_y+1]

        sshr            v21.8h,  v16.8h,  #6      // first base_x
        sshr            v22.8h,  v17.8h,  #6

        tbl             v20.16b, {v0.16b, v1.16b}, v30.16b // left[base_y+2]

        uzp2            v2.8h,   v4.8h,   v5.8h   // top[base_x+1]
        uzp1            v4.8h,   v4.8h,   v5.8h   // top[base_x]
        uzp2            v3.8h,   v6.8h,   v7.8h
        uzp1            v6.8h,   v6.8h,   v7.8h
        mov             v5.16b,  v2.16b
        mov             v7.16b,  v3.16b

        and             v16.16b, v16.16b, v25.16b // frac_x
        and             v17.16b, v17.16b, v25.16b

        umull           v10.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v10.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y

        sub             v8.8h,   v26.8h,  v16.8h  // 64 - frac_x
        sub             v9.8h,   v26.8h,  v17.8h

        umull2          v11.4s,  v18.8h,  v28.8h
        umlal2          v11.4s,  v19.8h,  v27.8h

        add             v21.8h,  v21.8h,  v31.8h  // actual base_x
        add             v22.8h,  v22.8h,  v31.8h

        umull           v12.4s,  v19.4h,  v28.4h
        umlal           v12.4s,  v20.4h,  v27.4h
        umull2          v13.4s,  v19.8h,  v28.8h
        umlal2          v13.4s,  v20.8h,  v27.8h

        rshrn           v10.4h,  v10.4s,  #6
        rshrn2          v10.8h,  v11.4s,  #6
        rshrn           v11.4h,  v12.4s,  #6
        rshrn2          v11.8h,  v13.4s,  #6

        umull           v12.4s,  v4.4h,   v8.4h   // top[base_x]-*(64-frac_x)
        umlal           v12.4s,  v5.4h,   v16.4h  // + top[base_x+1]*frac_x
        umull2          v13.4s,  v4.8h,   v8.8h
        umlal2          v13.4s,  v5.8h,   v16.8h
        umull           v14.4s,  v6.4h,   v9.4h
        umlal           v14.4s,  v7.4h,   v17.4h
        umull2          v18.4s,  v6.8h,   v9.8h
        umlal2          v18.4s,  v7.8h,   v17.8h

        cmge            v21.8h,  v21.8h,  #0
        cmge            v22.8h,  v22.8h,  #0

        rshrn           v12.4h,  v12.4s,  #6
        rshrn2          v12.8h,  v13.4s,  #6
        rshrn           v13.4h,  v14.4s,  #6
        rshrn2          v13.8h,  v18.4s,  #6

        bit             v10.16b, v12.16b, v21.16b
        bit             v11.16b, v13.16b, v22.16b

        st1             {v10.8h}, [x0], x1
        subs            w5,  w5,  #2
        sub             w8,  w8,  w6              // xpos -= dx
        st1             {v11.8h}, [x0], x1
        b.le            9f

        mov             v18.16b, v20.16b
        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        add             v30.16b, v30.16b, v24.16b // base_y += 2 (*2)
        b               8b

89:
        tbl             v19.16b, {v0.16b, v1.16b}, v29.16b // left[base_y+1]
        tbl             v20.16b, {v0.16b, v1.16b}, v30.16b // left[base_y+2]

        umull           v4.4s,   v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v4.4s,   v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v5.4s,   v18.8h,  v28.8h
        umlal2          v5.4s,   v19.8h,  v27.8h
        umull           v6.4s,   v19.4h,  v28.4h
        umlal           v6.4s,   v20.4h,  v27.4h
        umull2          v7.4s,   v19.8h,  v28.8h
        umlal2          v7.4s,   v20.8h,  v27.8h

        rshrn           v4.4h,   v4.4s,   #6
        rshrn2          v4.8h,   v5.4s,   #6
        rshrn           v5.4h,   v6.4s,   #6
        rshrn2          v5.8h,   v7.4s,   #6

        st1             {v4.8h}, [x0], x1
        subs            w5,  w5,  #2
        st1             {v5.8h}, [x0], x1
        b.le            9f

        mov             v18.16b, v20.16b
        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        add             v30.16b, v30.16b, v24.16b // base_y += 2 (*2)
        b               89b

9:
        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40
        ret
endfunc

function ipred_z2_fill3_16bpc_neon, export=1
        cmp             w4,  #8
        mov             w8,  #(1 << 6)            // xpos = 1 << 6
        sub             w8,  w8,  w6              // xpos -= dx

        movrel          x11, increments
        ld1             {v31.8h},  [x11]          // increments
        neg             w7,  w7                   // -dy
        b.eq            80f

40:
        dup             v30.4h,  w7               // -dy
        movi            v17.8b,  #1

        mul             v16.4h,  v31.4h,  v30.4h  // {0,1,2,3}* -dy
        movi            v25.8h,  #0x3e
        add             v30.4h,  v16.4h,  v30.4h  // -= dy

        // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
        ld1             {v0.8h, v1.8h, v2.8h}, [x3]    // left[]

        movi            v26.8h,  #64
        movi            v19.16b, #2

        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v30.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v19.8b  // base_y = (ypos >> 6) + 2

        movi            v23.4h,  #1, lsl #8
        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        movi            v19.16b, #4
        zip1            v29.8b,  v29.8b,  v29.8b  // duplicate elements
        movi            v17.8b,  #2
        add             v29.8b,  v29.8b,  v23.8b  // 2*base, 2*base+1, ...

        add             v30.8b,  v29.8b,  v17.8b  // base_y + 1 (*2)
        add             v28.8b,  v29.8b,  v19.8b  // base_y + 2 (*2)

        trn1            v31.2d,  v31.2d,  v31.2d  // {0,1,2,3,0,1,2,3}

        add             v24.8b,  v30.8b,  v19.8b  // base_y + 3 (*2)

        trn1            v29.2d,  v29.2d,  v28.2d  // base_y + 0, base_y + 2
        trn1            v30.2d,  v30.2d,  v24.2d  // base_y + 1, base_y + 3

        sub             v28.4h,  v26.4h,  v27.4h  // 64 - frac_y

        trn1            v27.2d,  v27.2d,  v27.2d  // frac_y
        trn1            v28.2d,  v28.2d,  v28.2d  // 64 - frac_y

        movi            v24.16b, #8
4:
        asr             w9,  w8,  #6              // base_x
        dup             v16.4h,  w8               // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-4                  // base_x <= -4
        asr             w11, w8,  #6              // base_x
        b.le            49f

        lsl             w9,  w9,  #1
        lsl             w11, w11, #1

        dup             v17.4h,  w8               // xpos

        ldr             q4,  [x2, w9, sxtw]       // top[base_x]
        ldr             q6,  [x2, w11, sxtw]

        trn1            v16.2d,  v16.2d,  v17.2d  // xpos

        tbl             v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+0], left[base_y+2]
        tbl             v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+1], left[base_y+3]

        sshr            v20.8h,  v16.8h,  #6      // first base_x for each row

        ext             v5.16b,  v4.16b,  v4.16b,  #2 // top[base_x+1]
        ext             v7.16b,  v6.16b,  v6.16b,  #2

        and             v16.16b, v16.16b, v25.16b // frac_x

        trn1            v4.2d,   v4.2d,   v6.2d   // top[base_x]
        trn1            v5.2d,   v5.2d,   v7.2d   // top[base_x+1]

        sub             v17.8h,  v26.8h,  v16.8h  // 64 - frac_x

        add             v20.8h,  v20.8h,  v31.8h  // actual base_x

        umull           v21.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v21.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v22.4s,  v18.8h,  v28.8h
        umlal2          v22.4s,  v19.8h,  v27.8h

        umull           v23.4s,  v4.4h,   v17.4h  // top[base_x]-*(64-frac_x)
        umlal           v23.4s,  v5.4h,   v16.4h  // + top[base_x+1]*frac_x
        umull2          v24.4s,  v4.8h,   v17.8h
        umlal2          v24.4s,  v5.8h,   v16.8h

        cmge            v20.8h,  v20.8h,  #0

        rshrn           v21.4h,  v21.4s,  #6
        rshrn2          v21.8h,  v22.4s,  #6
        rshrn           v22.4h,  v23.4s,  #6
        rshrn2          v22.8h,  v24.4s,  #6

        movi            v24.16b, #8

        bit             v21.16b, v22.16b, v20.16b

        st1             {v21.d}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v21.d}[1], [x0], x1
        b.le            9f

        add             v29.16b, v29.16b, v24.16b // base_y += 4 (*2)
        add             v30.16b, v30.16b, v24.16b // base_y += 4 (*2)
        b               4b

49:
        tbl             v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+0], left[base_y+2]
        tbl             v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+1], left[base_y+3]

        umull           v20.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v20.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v21.4s,  v18.8h,  v28.8h
        umlal2          v21.4s,  v19.8h,  v27.8h

        rshrn           v20.4h,  v20.4s,  #6
        rshrn2          v20.8h,  v21.4s,  #6

        st1             {v20.d}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v20.d}[1], [x0], x1
        b.le            9f

        add             v29.16b, v29.16b, v24.16b // base_y += 4 (*2)
        add             v30.16b, v30.16b, v24.16b // base_y += 4 (*2)
        b               49b

9:
        ret

80:
        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

        dup             v18.8h,  w7               // -dy
        movi            v17.16b, #2

        mul             v16.8h,  v31.8h,  v18.8h  // {0,1,2,3,4,5,6,7}* -dy
        movi            v25.8h,  #0x3e
        add             v16.8h,  v16.8h,  v18.8h  // -= dy

        // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
        ld1             {v0.8h, v1.8h, v2.8h}, [x3]    // left[]

        movi            v26.8h,  #64
        movi            v19.16b, #4

        shrn            v29.8b,  v16.8h,  #6      // ypos >> 6
        and             v27.16b, v16.16b, v25.16b // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 2

        movi            v23.8h,  #1, lsl #8
        shl             v29.8b,  v29.8b,  #1      // 2*base_y
        mov             v18.16b, v15.16b          // left[0]
        zip1            v29.16b, v29.16b, v29.16b // duplicate elements
        add             v29.16b, v29.16b, v23.16b // 2*base, 2*base+1, ...

        add             v30.16b, v29.16b, v17.16b // base_y + 1 (*2)

        sub             v28.8h,  v26.8h,  v27.8h  // 64 - frac_y

        movi            v24.16b, #4
8:
        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-16                 // base_x <= -16
        asr             w11, w8,  #6              // base_x
        b.le            89f

        dup             v17.8h,   w8              // xpos

        add             x9,  x2,  w9,  sxtw #1
        add             x11, x2,  w11, sxtw #1

        ld1             {v4.8h, v5.8h}, [x9]      // top[base_x]
        ld1             {v6.8h, v7.8h}, [x11]

        tbl             v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+0]
        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        tbl             v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+1]
        add             v30.16b, v30.16b, v24.16b

        sshr            v22.8h,  v16.8h,  #6      // first base_x
        tbl             v20.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+2]
        sshr            v23.8h,  v17.8h,  #6
        tbl             v21.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+3]

        ext             v5.16b,  v4.16b,  v5.16b,  #2 // top[base_x+1]
        ext             v7.16b,  v6.16b,  v7.16b,  #2

        and             v16.16b, v16.16b, v25.16b // frac_x
        and             v17.16b, v17.16b, v25.16b

        umull           v10.4s,  v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v10.4s,  v19.4h,  v27.4h  // + left[base_y+1]*frac_y

        sub             v8.8h,   v26.8h,  v16.8h  // 64 - frac_x
        sub             v9.8h,   v26.8h,  v17.8h

        umull2          v11.4s,  v18.8h,  v28.8h
        umlal2          v11.4s,  v19.8h,  v27.8h

        add             v22.8h,  v22.8h,  v31.8h  // actual base_x
        add             v23.8h,  v23.8h,  v31.8h

        umull           v12.4s,  v20.4h,  v28.4h
        umlal           v12.4s,  v21.4h,  v27.4h
        umull2          v13.4s,  v20.8h,  v28.8h
        umlal2          v13.4s,  v21.8h,  v27.8h

        rshrn           v10.4h,  v10.4s,  #6
        rshrn2          v10.8h,  v11.4s,  #6
        rshrn           v11.4h,  v12.4s,  #6
        rshrn2          v11.8h,  v13.4s,  #6

        umull           v12.4s,  v4.4h,   v8.4h   // top[base_x]-*(64-frac_x)
        umlal           v12.4s,  v5.4h,   v16.4h  // + top[base_x+1]*frac_x
        umull2          v13.4s,  v4.8h,   v8.8h
        umlal2          v13.4s,  v5.8h,   v16.8h
        umull           v14.4s,  v6.4h,   v9.4h
        umlal           v14.4s,  v7.4h,   v17.4h
        umull2          v18.4s,  v6.8h,   v9.8h
        umlal2          v18.4s,  v7.8h,   v17.8h

        cmge            v22.8h,  v22.8h,  #0
        cmge            v23.8h,  v23.8h,  #0

        rshrn           v12.4h,  v12.4s,  #6
        rshrn2          v12.8h,  v13.4s,  #6
        rshrn           v13.4h,  v14.4s,  #6
        rshrn2          v13.8h,  v18.4s,  #6

        bit             v10.16b, v12.16b, v22.16b
        bit             v11.16b, v13.16b, v23.16b

        st1             {v10.8h}, [x0], x1
        subs            w5,  w5,  #2
        sub             w8,  w8,  w6              // xpos -= dx
        st1             {v11.8h}, [x0], x1
        b.le            9f

        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        add             v30.16b, v30.16b, v24.16b
        b               8b

89:
        tbl             v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+0]
        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        tbl             v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+1]
        add             v30.16b, v30.16b, v24.16b
        tbl             v20.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+2]
        tbl             v21.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+3]

        umull           v4.4s,   v18.4h,  v28.4h  // left[base_y]*(64-frac_y)
        umlal           v4.4s,   v19.4h,  v27.4h  // + left[base_y+1]*frac_y
        umull2          v5.4s,   v18.8h,  v28.8h
        umlal2          v5.4s,   v19.8h,  v27.8h
        umull           v6.4s,   v20.4h,  v28.4h
        umlal           v6.4s,   v21.4h,  v27.4h
        umull2          v7.4s,   v20.8h,  v28.8h
        umlal2          v7.4s,   v21.8h,  v27.8h

        rshrn           v4.4h,   v4.4s,   #6
        rshrn2          v4.8h,   v5.4s,   #6
        rshrn           v5.4h,   v6.4s,   #6
        rshrn2          v5.8h,   v7.4s,   #6

        st1             {v4.8h}, [x0], x1
        subs            w5,  w5,  #2
        st1             {v5.8h}, [x0], x1
        b.le            9f

        add             v29.16b, v29.16b, v24.16b // base_y += 2 (*2)
        add             v30.16b, v30.16b, v24.16b
        b               89b

9:
        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40
        ret
endfunc

// void ipred_z3_fill1_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const left,
//                                const int width, const int height,
//                                const int dy, const int max_base_y);
function ipred_z3_fill1_16bpc_neon, export=1
        clz             w9,  w4
        adr             x8,  L(ipred_z3_fill1_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x8, w9, uxtw #1]
        add             x10, x2,  w6,  uxtw #1    // left[max_base_y]
        sub             x8,  x8,  w9,  uxtw
        ld1r            {v31.8h}, [x10]           // padding
        mov             w7,  w5
        mov             w15, #64
        add             x13, x0,  x1
        lsl             x1,  x1,  #1
        br              x8

40:
        AARCH64_VALID_JUMP_TARGET
4:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            ipred_z3_fill_padding_neon
        lsl             w8,  w8,  #1
        lsl             w10, w10, #1
        ldr             q0,  [x2, w8, uxtw]       // left[base]
        ldr             q2,  [x2, w10, uxtw]
        dup             v4.8h,   w9               // frac
        dup             v5.8h,   w11
        ext             v1.16b,  v0.16b,  v0.16b,  #2 // left[base+1]
        ext             v3.16b,  v2.16b,  v2.16b,  #2
        sub             v6.4h,   v1.4h,   v0.4h   // top[base+1]-top[base]
        sub             v7.4h,   v3.4h,   v2.4h
        ushll           v16.4s,  v0.4h,   #6      // top[base]*64
        ushll           v17.4s,  v2.4h,   #6
        smlal           v16.4s,  v6.4h,   v4.4h   // + top[base+1]*frac
        smlal           v17.4s,  v7.4h,   v5.4h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn           v17.4h,  v17.4s,  #6
        subs            w3,  w3,  #2
        zip1            v18.8h,  v16.8h,  v17.8h
        st1             {v18.s}[0], [x0],  x1
        st1             {v18.s}[1], [x13], x1
        add             w7,  w7,  w5              // xpos += dx
        st1             {v18.s}[2], [x0]
        st1             {v18.s}[3], [x13]
        b.le            9f
        sub             x0,  x0,  x1              // ptr -= 4 * (2*stride)
        sub             x13, x13, x1
        add             x0,  x0,  #4
        add             x13, x13, #4
        b               4b
9:
        ret

80:
        AARCH64_VALID_JUMP_TARGET
8:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            ipred_z3_fill_padding_neon
        add             x8,  x2,  w8,  uxtw #1
        add             x10, x2,  w10, uxtw #1
        dup             v4.8h,   w9               // frac
        dup             v5.8h,   w11
        ld1             {v0.8h},  [x8]            // left[base]
        ld1             {v2.8h},  [x10]
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        ldr             h1, [x8, #16]
        ldr             h3, [x10, #16]
        dup             v6.8h,   w9               // 64 - frac
        dup             v7.8h,   w11
        ext             v1.16b,  v0.16b,  v1.16b,  #2 // left[base+1]
        ext             v3.16b,  v2.16b,  v3.16b,  #2
        umull           v16.4s,  v0.4h,   v6.4h   // left[base]*(64-frac)
        umlal           v16.4s,  v1.4h,   v4.4h   // + left[base+1]*frac
        umull2          v17.4s,  v0.8h,   v6.8h
        umlal2          v17.4s,  v1.8h,   v4.8h
        umull           v18.4s,  v2.4h,   v7.4h
        umlal           v18.4s,  v3.4h,   v5.4h
        umull2          v19.4s,  v2.8h,   v7.8h
        umlal2          v19.4s,  v3.8h,   v5.8h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn2          v16.8h,  v17.4s,  #6
        rshrn           v17.4h,  v18.4s,  #6
        rshrn2          v17.8h,  v19.4s,  #6
        subs            w3,  w3,  #2
        zip1            v18.8h,  v16.8h,  v17.8h
        zip2            v19.8h,  v16.8h,  v17.8h
        add             w7,  w7,  w5              // xpos += dx
        st1             {v18.s}[0], [x0],  x1
        st1             {v18.s}[1], [x13], x1
        st1             {v18.s}[2], [x0],  x1
        st1             {v18.s}[3], [x13], x1
        st1             {v19.s}[0], [x0],  x1
        st1             {v19.s}[1], [x13], x1
        st1             {v19.s}[2], [x0],  x1
        st1             {v19.s}[3], [x13], x1
        b.le            9f
        sub             x0,  x0,  x1, lsl #2      // ptr -= 4 * (2*stride)
        sub             x13, x13, x1, lsl #2
        add             x0,  x0,  #4
        add             x13, x13, #4
        b               8b
9:
        ret

160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        mov             w12, w4
1:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // ypos += dy
        cmp             w8,  w6                   // base >= max_base_y
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            ipred_z3_fill_padding_neon
        add             x8,  x2,  w8,  uxtw #1
        add             x10, x2,  w10, uxtw #1
        dup             v6.8h,   w9               // frac
        dup             v7.8h,   w11
        ld1             {v0.8h, v1.8h, v2.8h}, [x8],  #48 // left[base]
        ld1             {v3.8h, v4.8h, v5.8h}, [x10], #48
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v16.8h,  w9               // 64 - frac
        dup             v17.8h,  w11
        add             w7,  w7,  w5              // ypos += dy
2:
        ext             v18.16b, v0.16b,  v1.16b,  #2 // left[base+1]
        ext             v19.16b, v1.16b,  v2.16b,  #2
        ext             v20.16b, v3.16b,  v4.16b,  #2
        ext             v21.16b, v4.16b,  v5.16b,  #2
        subs            w4,  w4,  #16
        umull           v22.4s,  v0.4h,   v16.4h  // left[base]*(64-frac)
        umlal           v22.4s,  v18.4h,  v6.4h   // + left[base+1]*frac
        umull2          v23.4s,  v0.8h,   v16.8h
        umlal2          v23.4s,  v18.8h,  v6.8h
        umull           v24.4s,  v1.4h,   v16.4h
        umlal           v24.4s,  v19.4h,  v6.4h
        umull2          v25.4s,  v1.8h,   v16.8h
        umlal2          v25.4s,  v19.8h,  v6.8h
        umull           v26.4s,  v3.4h,   v17.4h
        umlal           v26.4s,  v20.4h,  v7.4h
        umull2          v27.4s,  v3.8h,   v17.8h
        umlal2          v27.4s,  v20.8h,  v7.8h
        umull           v28.4s,  v4.4h,   v17.4h
        umlal           v28.4s,  v21.4h,  v7.4h
        umull2          v29.4s,  v4.8h,   v17.8h
        umlal2          v29.4s,  v21.8h,  v7.8h
        rshrn           v22.4h,  v22.4s,  #6
        rshrn2          v22.8h,  v23.4s,  #6
        rshrn           v23.4h,  v24.4s,  #6
        rshrn2          v23.8h,  v25.4s,  #6
        rshrn           v24.4h,  v26.4s,  #6
        rshrn2          v24.8h,  v27.4s,  #6
        rshrn           v25.4h,  v28.4s,  #6
        rshrn2          v25.8h,  v29.4s,  #6
        zip1            v18.8h,  v22.8h,  v24.8h
        zip2            v19.8h,  v22.8h,  v24.8h
        zip1            v20.8h,  v23.8h,  v25.8h
        zip2            v21.8h,  v23.8h,  v25.8h
        st1             {v18.s}[0], [x0],  x1
        st1             {v18.s}[1], [x13], x1
        st1             {v18.s}[2], [x0],  x1
        st1             {v18.s}[3], [x13], x1
        st1             {v19.s}[0], [x0],  x1
        st1             {v19.s}[1], [x13], x1
        st1             {v19.s}[2], [x0],  x1
        st1             {v19.s}[3], [x13], x1
        st1             {v20.s}[0], [x0],  x1
        st1             {v20.s}[1], [x13], x1
        st1             {v20.s}[2], [x0],  x1
        st1             {v20.s}[3], [x13], x1
        st1             {v21.s}[0], [x0],  x1
        st1             {v21.s}[1], [x13], x1
        st1             {v21.s}[2], [x0],  x1
        st1             {v21.s}[3], [x13], x1
        b.le            3f
        mov             v0.16b,  v2.16b
        ld1             {v1.8h, v2.8h}, [x8],  #32      // left[base]
        mov             v3.16b,  v5.16b
        ld1             {v4.8h, v5.8h}, [x10], #32
        b               2b

3:
        subs            w3,  w3,  #2
        b.le            9f
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        lsl             x1,  x1,  #1
        add             x0,  x0,  #4
        add             x13, x13, #4
        mov             w4,  w12
        b               1b
9:
        ret

L(ipred_z3_fill1_tbl):
        .hword L(ipred_z3_fill1_tbl) - 640b
        .hword L(ipred_z3_fill1_tbl) - 320b
        .hword L(ipred_z3_fill1_tbl) - 160b
        .hword L(ipred_z3_fill1_tbl) -  80b
        .hword L(ipred_z3_fill1_tbl) -  40b
endfunc

function ipred_z3_fill_padding_neon, export=0
        cmp             w3,  #8
        adr             x8,  L(ipred_z3_fill_padding_tbl)
        b.gt            L(ipred_z3_fill_padding_wide)
        // w3 = remaining width, w4 = constant height
        mov             w12, w4

1:
        // Fill a WxH rectangle with padding. W can be any number;
        // this fills the exact width by filling in the largest
        // power of two in the remaining width, and repeating.
        clz             w9,  w3
        sub             w9,  w9,  #25
        ldrh            w9,  [x8, w9, uxtw #1]
        sub             x9,  x8,  w9,  uxtw
        br              x9

2:
        st1             {v31.s}[0], [x0],  x1
        subs            w4,  w4,  #4
        st1             {v31.s}[0], [x13], x1
        st1             {v31.s}[0], [x0],  x1
        st1             {v31.s}[0], [x13], x1
        b.gt            2b
        subs            w3,  w3,  #2
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        b.le            9f
        lsl             x1,  x1,  #1
        add             x0,  x0,  #4
        add             x13, x13, #4
        mov             w4,  w12
        b               1b

4:
        st1             {v31.4h}, [x0],  x1
        subs            w4,  w4,  #4
        st1             {v31.4h}, [x13], x1
        st1             {v31.4h}, [x0],  x1
        st1             {v31.4h}, [x13], x1
        b.gt            4b
        subs            w3,  w3,  #4
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        b.le            9f
        lsl             x1,  x1,  #1
        add             x0,  x0,  #8
        add             x13, x13, #8
        mov             w4,  w12
        b               1b

8:
16:
32:
64:
        st1             {v31.8h}, [x0],  x1
        subs            w4,  w4,  #4
        st1             {v31.8h}, [x13], x1
        st1             {v31.8h}, [x0],  x1
        st1             {v31.8h}, [x13], x1
        b.gt            4b
        subs            w3,  w3,  #8
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        b.le            9f
        lsl             x1,  x1,  #1
        add             x0,  x0,  #16
        add             x13, x13, #16
        mov             w4,  w12
        b               1b

9:
        ret

L(ipred_z3_fill_padding_tbl):
        .hword L(ipred_z3_fill_padding_tbl) - 64b
        .hword L(ipred_z3_fill_padding_tbl) - 32b
        .hword L(ipred_z3_fill_padding_tbl) - 16b
        .hword L(ipred_z3_fill_padding_tbl) -  8b
        .hword L(ipred_z3_fill_padding_tbl) -  4b
        .hword L(ipred_z3_fill_padding_tbl) -  2b

L(ipred_z3_fill_padding_wide):
        // Fill a WxH rectangle with padding, with W > 8.
        lsr             x1,  x1,  #1
        mov             w12, w3
        sub             x1,  x1,  w3,  uxtw #1
1:
        ands            w5,  w3,  #7
        b.eq            2f
        // If the width isn't aligned to 8, first do one 8 pixel write
        // and align the start pointer.
        sub             w3,  w3,  w5
        st1             {v31.8h}, [x0]
        add             x0,  x0,  w5,  uxtw #1
2:
        // Fill the rest of the line with aligned 8 pixel writes.
        subs            w3,  w3,  #8
        st1             {v31.8h}, [x0], #16
        b.gt            2b
        subs            w4,  w4,  #1
        add             x0,  x0,  x1
        b.le            9f
        mov             w3,  w12
        b               1b
9:
        ret
endfunc

function ipred_z3_fill2_16bpc_neon, export=1
        cmp             w4,  #8
        add             x10, x2,  w6,  uxtw       // left[max_base_y]
        ld1r            {v31.16b}, [x10]          // padding
        mov             w7,  w5
        mov             w15, #64
        add             x13, x0,  x1
        lsl             x1,  x1,  #1
        b.eq            8f

4:      // h == 4
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            ipred_z3_fill_padding_neon
        lsl             w8,  w8,  #1
        lsl             w10, w10, #1
        ldr             q0,  [x2, w8, uxtw]       // top[base]
        ldr             q2,  [x2, w10, uxtw]
        dup             v4.4h,   w9               // frac
        dup             v5.4h,   w11
        uzp2            v1.8h,   v0.8h,   v0.8h   // top[base+1]
        uzp1            v0.8h,   v0.8h,   v0.8h   // top[base]
        uzp2            v3.8h,   v2.8h,   v2.8h
        uzp1            v2.8h,   v2.8h,   v2.8h
        sub             v6.4h,   v1.4h,   v0.4h   // top[base+1]-top[base]
        sub             v7.4h,   v3.4h,   v2.4h
        ushll           v16.4s,  v0.4h,   #6      // top[base]*64
        ushll           v17.4s,  v2.4h,   #6
        smlal           v16.4s,  v6.4h,   v4.4h   // + top[base+1]*frac
        smlal           v17.4s,  v7.4h,   v5.4h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn           v17.4h,  v17.4s,  #6
        subs            w3,  w3,  #2
        zip1            v18.8h,  v16.8h,  v17.8h
        st1             {v18.s}[0], [x0],  x1
        st1             {v18.s}[1], [x13], x1
        add             w7,  w7,  w5              // xpos += dx
        st1             {v18.s}[2], [x0]
        st1             {v18.s}[3], [x13]
        b.le            9f
        sub             x0,  x0,  x1              // ptr -= 4 * (2*stride)
        sub             x13, x13, x1
        add             x0,  x0,  #4
        add             x13, x13, #4
        b               4b
9:
        ret

8:      // h == 8
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            ipred_z3_fill_padding_neon
        add             x8,  x2,  w8,  uxtw #1
        add             x10, x2,  w10, uxtw #1
        dup             v4.8h,   w9               // frac
        dup             v5.8h,   w11
        ld1             {v0.8h, v1.8h},  [x8]     // top[base]
        ld1             {v2.8h, v3.8h},  [x10]
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v6.8h,   w9               // 64 - frac
        dup             v7.8h,   w11
        uzp2            v20.8h,  v0.8h,   v1.8h   // top[base+1]
        uzp1            v0.8h,   v0.8h,   v1.8h   // top[base]
        uzp2            v21.8h,  v2.8h,   v3.8h
        uzp1            v2.8h,   v2.8h,   v3.8h
        umull           v16.4s,  v0.4h,   v6.4h   // top[base]*(64-frac)
        umlal           v16.4s,  v20.4h,  v4.4h   // + top[base+1]*frac
        umull2          v17.4s,  v0.8h,   v6.8h
        umlal2          v17.4s,  v20.8h,  v4.8h
        umull           v18.4s,  v2.4h,   v7.4h
        umlal           v18.4s,  v21.4h,  v5.4h
        umull2          v19.4s,  v2.8h,   v7.8h
        umlal2          v19.4s,  v21.8h,  v5.8h
        rshrn           v16.4h,  v16.4s,  #6
        rshrn2          v16.8h,  v17.4s,  #6
        rshrn           v17.4h,  v18.4s,  #6
        rshrn2          v17.8h,  v19.4s,  #6
        subs            w3,  w3,  #2
        zip1            v18.8h,  v16.8h,  v17.8h
        zip2            v19.8h,  v16.8h,  v17.8h
        add             w7,  w7,  w5              // xpos += dx
        st1             {v18.s}[0], [x0],  x1
        st1             {v18.s}[1], [x13], x1
        st1             {v18.s}[2], [x0],  x1
        st1             {v18.s}[3], [x13], x1
        st1             {v19.s}[0], [x0],  x1
        st1             {v19.s}[1], [x13], x1
        st1             {v19.s}[2], [x0],  x1
        st1             {v19.s}[3], [x13], x1
        b.le            9f
        sub             x0,  x0,  x1, lsl #2      // ptr -= 4 * (2*stride)
        sub             x13, x13, x1, lsl #2
        add             x0,  x0,  #4
        add             x13, x13, #4
        b               8b
9:
        ret
endfunc


// void ipred_filter_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int filt_idx,
//                              const int max_width, const int max_height,
//                              const int bitdepth_max);
.macro filter_fn bpc
function ipred_filter_\bpc\()bpc_neon
        and             w5,  w5,  #511
        movrel          x6,  X(filter_intra_taps)
        lsl             w5,  w5,  #6
        add             x6,  x6,  w5, uxtw
        ld1             {v16.8b, v17.8b, v18.8b, v19.8b}, [x6], #32
        clz             w9,  w3
        adr             x5,  L(ipred_filter\bpc\()_tbl)
        ld1             {v20.8b, v21.8b, v22.8b}, [x6]
        sub             w9,  w9,  #26
        ldrh            w9,  [x5, w9, uxtw #1]
        sxtl            v16.8h,  v16.8b
        sxtl            v17.8h,  v17.8b
        sub             x5,  x5,  w9, uxtw
        sxtl            v18.8h,  v18.8b
        sxtl            v19.8h,  v19.8b
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        sxtl            v20.8h,  v20.8b
        sxtl            v21.8h,  v21.8b
        sxtl            v22.8h,  v22.8b
        dup             v31.8h,  w8
.if \bpc == 10
        movi            v30.8h,  #0
.endif
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ldur            d0,  [x2, #2]             // top (0-3)
        sub             x2,  x2,  #4
        mov             x7,  #-4
4:
        ld1             {v1.4h}, [x2], x7         // left (0-1) + topleft (2)
.if \bpc == 10
        mul             v2.8h,   v17.8h,  v0.h[0] // p1(top[0]) * filter(1)
        mla             v2.8h,   v18.8h,  v0.h[1] // p2(top[1]) * filter(2)
        mla             v2.8h,   v19.8h,  v0.h[2] // p3(top[2]) * filter(3)
        mla             v2.8h,   v20.8h,  v0.h[3] // p4(top[3]) * filter(4)
        mla             v2.8h,   v16.8h,  v1.h[2] // p0(topleft) * filter(0)
        mla             v2.8h,   v21.8h,  v1.h[1] // p5(left[0]) * filter(5)
        mla             v2.8h,   v22.8h,  v1.h[0] // p6(left[1]) * filter(6)
        srshr           v2.8h,   v2.8h,   #4
        smax            v2.8h,   v2.8h,   v30.8h
.else
        smull           v2.4s,   v17.4h,  v0.h[0] // p1(top[0]) * filter(1)
        smlal           v2.4s,   v18.4h,  v0.h[1] // p2(top[1]) * filter(2)
        smlal           v2.4s,   v19.4h,  v0.h[2] // p3(top[2]) * filter(3)
        smlal           v2.4s,   v20.4h,  v0.h[3] // p4(top[3]) * filter(4)
        smlal           v2.4s,   v16.4h,  v1.h[2] // p0(topleft) * filter(0)
        smlal           v2.4s,   v21.4h,  v1.h[1] // p5(left[0]) * filter(5)
        smlal           v2.4s,   v22.4h,  v1.h[0] // p6(left[1]) * filter(6)
        smull2          v3.4s,   v17.8h,  v0.h[0] // p1(top[0]) * filter(1)
        smlal2          v3.4s,   v18.8h,  v0.h[1] // p2(top[1]) * filter(2)
        smlal2          v3.4s,   v19.8h,  v0.h[2] // p3(top[2]) * filter(3)
        smlal2          v3.4s,   v20.8h,  v0.h[3] // p4(top[3]) * filter(4)
        smlal2          v3.4s,   v16.8h,  v1.h[2] // p0(topleft) * filter(0)
        smlal2          v3.4s,   v21.8h,  v1.h[1] // p5(left[0]) * filter(5)
        smlal2          v3.4s,   v22.8h,  v1.h[0] // p6(left[1]) * filter(6)
        sqrshrun        v2.4h,   v2.4s,   #4
        sqrshrun2       v2.8h,   v3.4s,   #4
.endif
        smin            v2.8h,   v2.8h,   v31.8h
        subs            w4,  w4,  #2
        st1             {v2.d}[0], [x0], x1
        ext             v0.16b,  v2.16b,  v2.16b, #8 // move top from [4-7] to [0-3]
        st1             {v2.d}[1], [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ldur            q0,  [x2, #2]             // top (0-7)
        sub             x2,  x2,  #4
        mov             x7,  #-4
8:
        ld1             {v1.4h}, [x2], x7         // left (0-1) + topleft (2)
.if \bpc == 10
        mul             v2.8h,   v17.8h,  v0.h[0] // p1(top[0]) * filter(1)
        mla             v2.8h,   v18.8h,  v0.h[1] // p2(top[1]) * filter(2)
        mla             v2.8h,   v19.8h,  v0.h[2] // p3(top[2]) * filter(3)
        mla             v2.8h,   v20.8h,  v0.h[3] // p4(top[3]) * filter(4)
        mla             v2.8h,   v16.8h,  v1.h[2] // p0(topleft) * filter(0)
        mla             v2.8h,   v21.8h,  v1.h[1] // p5(left[0]) * filter(5)
        mla             v2.8h,   v22.8h,  v1.h[0] // p6(left[1]) * filter(6)
        mul             v3.8h,   v17.8h,  v0.h[4] // p1(top[0]) * filter(1)
        mla             v3.8h,   v18.8h,  v0.h[5] // p2(top[1]) * filter(2)
        mla             v3.8h,   v19.8h,  v0.h[6] // p3(top[2]) * filter(3)
        srshr           v2.8h,   v2.8h,   #4
        smax            v2.8h,   v2.8h,   v30.8h
        smin            v2.8h,   v2.8h,   v31.8h
        mla             v3.8h,   v20.8h,  v0.h[7] // p4(top[3]) * filter(4)
        mla             v3.8h,   v16.8h,  v0.h[3] // p0(topleft) * filter(0)
        mla             v3.8h,   v21.8h,  v2.h[3] // p5(left[0]) * filter(5)
        mla             v3.8h,   v22.8h,  v2.h[7] // p6(left[1]) * filter(6)
        srshr           v3.8h,   v3.8h,   #4
        smax            v3.8h,   v3.8h,   v30.8h
.else
        smull           v2.4s,   v17.4h,  v0.h[0] // p1(top[0]) * filter(1)
        smlal           v2.4s,   v18.4h,  v0.h[1] // p2(top[1]) * filter(2)
        smlal           v2.4s,   v19.4h,  v0.h[2] // p3(top[2]) * filter(3)
        smlal           v2.4s,   v20.4h,  v0.h[3] // p4(top[3]) * filter(4)
        smlal           v2.4s,   v16.4h,  v1.h[2] // p0(topleft) * filter(0)
        smlal           v2.4s,   v21.4h,  v1.h[1] // p5(left[0]) * filter(5)
        smlal           v2.4s,   v22.4h,  v1.h[0] // p6(left[1]) * filter(6)
        smull2          v3.4s,   v17.8h,  v0.h[0] // p1(top[0]) * filter(1)
        smlal2          v3.4s,   v18.8h,  v0.h[1] // p2(top[1]) * filter(2)
        smlal2          v3.4s,   v19.8h,  v0.h[2] // p3(top[2]) * filter(3)
        smlal2          v3.4s,   v20.8h,  v0.h[3] // p4(top[3]) * filter(4)
        smlal2          v3.4s,   v16.8h,  v1.h[2] // p0(topleft) * filter(0)
        smlal2          v3.4s,   v21.8h,  v1.h[1] // p5(left[0]) * filter(5)
        smlal2          v3.4s,   v22.8h,  v1.h[0] // p6(left[1]) * filter(6)
        smull           v4.4s,   v17.4h,  v0.h[4] // p1(top[0]) * filter(1)
        smlal           v4.4s,   v18.4h,  v0.h[5] // p2(top[1]) * filter(2)
        smlal           v4.4s,   v19.4h,  v0.h[6] // p3(top[2]) * filter(3)
        sqrshrun        v2.4h,   v2.4s,   #4
        sqrshrun2       v2.8h,   v3.4s,   #4
        smin            v2.8h,   v2.8h,   v31.8h
        smlal           v4.4s,   v20.4h,  v0.h[7] // p4(top[3]) * filter(4)
        smlal           v4.4s,   v16.4h,  v0.h[3] // p0(topleft) * filter(0)
        smlal           v4.4s,   v21.4h,  v2.h[3] // p5(left[0]) * filter(5)
        smlal           v4.4s,   v22.4h,  v2.h[7] // p6(left[1]) * filter(6)
        smull2          v5.4s,   v17.8h,  v0.h[4] // p1(top[0]) * filter(1)
        smlal2          v5.4s,   v18.8h,  v0.h[5] // p2(top[1]) * filter(2)
        smlal2          v5.4s,   v19.8h,  v0.h[6] // p3(top[2]) * filter(3)
        smlal2          v5.4s,   v20.8h,  v0.h[7] // p4(top[3]) * filter(4)
        smlal2          v5.4s,   v16.8h,  v0.h[3] // p0(topleft) * filter(0)
        smlal2          v5.4s,   v21.8h,  v2.h[3] // p5(left[0]) * filter(5)
        smlal2          v5.4s,   v22.8h,  v2.h[7] // p6(left[1]) * filter(6)
        sqrshrun        v3.4h,   v4.4s,   #4
        sqrshrun2       v3.8h,   v5.4s,   #4
.endif
        smin            v3.8h,   v3.8h,   v31.8h
        subs            w4,  w4,  #2
        st2             {v2.d, v3.d}[0], [x0], x1
        zip2            v0.2d,   v2.2d,   v3.2d
        st2             {v2.d, v3.d}[1], [x6], x1
        b.gt            8b
        ret
160:
320:
        AARCH64_VALID_JUMP_TARGET
        add             x8,  x2,  #2
        sub             x2,  x2,  #4
        mov             x7,  #-4
        sub             x1,  x1,  w3, uxtw #1
        mov             w9,  w3

1:
        ld1             {v0.4h}, [x2], x7         // left (0-1) + topleft (2)
2:
        ld1             {v1.8h, v2.8h}, [x8], #32 // top(0-15)
.if \bpc == 10
        mul             v3.8h,   v16.8h,  v0.h[2] // p0(topleft) * filter(0)
        mla             v3.8h,   v21.8h,  v0.h[1] // p5(left[0]) * filter(5)
        mla             v3.8h,   v22.8h,  v0.h[0] // p6(left[1]) * filter(6)
        mla             v3.8h,   v17.8h,  v1.h[0] // p1(top[0]) * filter(1)
        mla             v3.8h,   v18.8h,  v1.h[1] // p2(top[1]) * filter(2)
        mla             v3.8h,   v19.8h,  v1.h[2] // p3(top[2]) * filter(3)
        mla             v3.8h,   v20.8h,  v1.h[3] // p4(top[3]) * filter(4)

        mul             v4.8h,   v17.8h,  v1.h[4] // p1(top[0]) * filter(1)
        mla             v4.8h,   v18.8h,  v1.h[5] // p2(top[1]) * filter(2)
        mla             v4.8h,   v19.8h,  v1.h[6] // p3(top[2]) * filter(3)
        srshr           v3.8h,   v3.8h,   #4
        smax            v3.8h,   v3.8h,   v30.8h
        smin            v3.8h,   v3.8h,   v31.8h
        mla             v4.8h,   v20.8h,  v1.h[7] // p4(top[3]) * filter(4)
        mla             v4.8h,   v16.8h,  v1.h[3] // p0(topleft) * filter(0)
        mla             v4.8h,   v21.8h,  v3.h[3] // p5(left[0]) * filter(5)
        mla             v4.8h,   v22.8h,  v3.h[7] // p6(left[1]) * filter(6)

        mul             v5.8h,   v17.8h,  v2.h[0] // p1(top[0]) * filter(1)
        mla             v5.8h,   v18.8h,  v2.h[1] // p2(top[1]) * filter(2)
        mla             v5.8h,   v19.8h,  v2.h[2] // p3(top[2]) * filter(3)
        srshr           v4.8h,   v4.8h,   #4
        smax            v4.8h,   v4.8h,   v30.8h
        smin            v4.8h,   v4.8h,   v31.8h
        mla             v5.8h,   v20.8h,  v2.h[3] // p4(top[3]) * filter(4)
        mla             v5.8h,   v16.8h,  v1.h[7] // p0(topleft) * filter(0)
        mla             v5.8h,   v21.8h,  v4.h[3] // p5(left[0]) * filter(5)
        mla             v5.8h,   v22.8h,  v4.h[7] // p6(left[1]) * filter(6)

        mul             v6.8h,   v17.8h,  v2.h[4] // p1(top[0]) * filter(1)
        mla             v6.8h,   v18.8h,  v2.h[5] // p2(top[1]) * filter(2)
        mla             v6.8h,   v19.8h,  v2.h[6] // p3(top[2]) * filter(3)
        srshr           v5.8h,   v5.8h,   #4
        smax            v5.8h,   v5.8h,   v30.8h
        smin            v5.8h,   v5.8h,   v31.8h
        mla             v6.8h,   v20.8h,  v2.h[7] // p4(top[3]) * filter(4)
        mla             v6.8h,   v16.8h,  v2.h[3] // p0(topleft) * filter(0)
        mla             v6.8h,   v21.8h,  v5.h[3] // p5(left[0]) * filter(5)
        mla             v6.8h,   v22.8h,  v5.h[7] // p6(left[1]) * filter(6)

        subs            w3,  w3,  #16
        srshr           v6.8h,   v6.8h,   #4
        smax            v6.8h,   v6.8h,   v30.8h
.else
        smull           v3.4s,   v16.4h,  v0.h[2] // p0(topleft) * filter(0)
        smlal           v3.4s,   v21.4h,  v0.h[1] // p5(left[0]) * filter(5)
        smlal           v3.4s,   v22.4h,  v0.h[0] // p6(left[1]) * filter(6)
        smlal           v3.4s,   v17.4h,  v1.h[0] // p1(top[0]) * filter(1)
        smlal           v3.4s,   v18.4h,  v1.h[1] // p2(top[1]) * filter(2)
        smlal           v3.4s,   v19.4h,  v1.h[2] // p3(top[2]) * filter(3)
        smlal           v3.4s,   v20.4h,  v1.h[3] // p4(top[3]) * filter(4)
        smull2          v4.4s,   v16.8h,  v0.h[2] // p0(topleft) * filter(0)
        smlal2          v4.4s,   v21.8h,  v0.h[1] // p5(left[0]) * filter(5)
        smlal2          v4.4s,   v22.8h,  v0.h[0] // p6(left[1]) * filter(6)
        smlal2          v4.4s,   v17.8h,  v1.h[0] // p1(top[0]) * filter(1)
        smlal2          v4.4s,   v18.8h,  v1.h[1] // p2(top[1]) * filter(2)
        smlal2          v4.4s,   v19.8h,  v1.h[2] // p3(top[2]) * filter(3)
        smlal2          v4.4s,   v20.8h,  v1.h[3] // p4(top[3]) * filter(4)

        smull           v5.4s,   v17.4h,  v1.h[4] // p1(top[0]) * filter(1)
        smlal           v5.4s,   v18.4h,  v1.h[5] // p2(top[1]) * filter(2)
        smlal           v5.4s,   v19.4h,  v1.h[6] // p3(top[2]) * filter(3)
        sqrshrun        v3.4h,   v3.4s,   #4
        sqrshrun2       v3.8h,   v4.4s,   #4
        smin            v3.8h,   v3.8h,   v31.8h
        smlal           v5.4s,   v20.4h,  v1.h[7] // p4(top[3]) * filter(4)
        smlal           v5.4s,   v16.4h,  v1.h[3] // p0(topleft) * filter(0)
        smlal           v5.4s,   v21.4h,  v3.h[3] // p5(left[0]) * filter(5)
        smlal           v5.4s,   v22.4h,  v3.h[7] // p6(left[1]) * filter(6)
        smull2          v6.4s,   v17.8h,  v1.h[4] // p1(top[0]) * filter(1)
        smlal2          v6.4s,   v18.8h,  v1.h[5] // p2(top[1]) * filter(2)
        smlal2          v6.4s,   v19.8h,  v1.h[6] // p3(top[2]) * filter(3)
        smlal2          v6.4s,   v20.8h,  v1.h[7] // p4(top[3]) * filter(4)
        smlal2          v6.4s,   v16.8h,  v1.h[3] // p0(topleft) * filter(0)
        smlal2          v6.4s,   v21.8h,  v3.h[3] // p5(left[0]) * filter(5)
        smlal2          v6.4s,   v22.8h,  v3.h[7] // p6(left[1]) * filter(6)

        smull           v24.4s,  v17.4h,  v2.h[0] // p1(top[0]) * filter(1)
        smlal           v24.4s,  v18.4h,  v2.h[1] // p2(top[1]) * filter(2)
        smlal           v24.4s,  v19.4h,  v2.h[2] // p3(top[2]) * filter(3)
        sqrshrun        v4.4h,   v5.4s,   #4
        sqrshrun2       v4.8h,   v6.4s,   #4
        smin            v4.8h,   v4.8h,   v31.8h
        smlal           v24.4s,  v20.4h,  v2.h[3] // p4(top[3]) * filter(4)
        smlal           v24.4s,  v16.4h,  v1.h[7] // p0(topleft) * filter(0)
        smlal           v24.4s,  v21.4h,  v4.h[3] // p5(left[0]) * filter(5)
        smlal           v24.4s,  v22.4h,  v4.h[7] // p6(left[1]) * filter(6)
        smull2          v25.4s,  v17.8h,  v2.h[0] // p1(top[0]) * filter(1)
        smlal2          v25.4s,  v18.8h,  v2.h[1] // p2(top[1]) * filter(2)
        smlal2          v25.4s,  v19.8h,  v2.h[2] // p3(top[2]) * filter(3)
        smlal2          v25.4s,  v20.8h,  v2.h[3] // p4(top[3]) * filter(4)
        smlal2          v25.4s,  v16.8h,  v1.h[7] // p0(topleft) * filter(0)
        smlal2          v25.4s,  v21.8h,  v4.h[3] // p5(left[0]) * filter(5)
        smlal2          v25.4s,  v22.8h,  v4.h[7] // p6(left[1]) * filter(6)

        smull           v26.4s,  v17.4h,  v2.h[4] // p1(top[0]) * filter(1)
        smlal           v26.4s,  v18.4h,  v2.h[5] // p2(top[1]) * filter(2)
        smlal           v26.4s,  v19.4h,  v2.h[6] // p3(top[2]) * filter(3)
        sqrshrun        v5.4h,   v24.4s,  #4
        sqrshrun2       v5.8h,   v25.4s,  #4
        smin            v5.8h,   v5.8h,   v31.8h
        smlal           v26.4s,  v20.4h,  v2.h[7] // p4(top[3]) * filter(4)
        smlal           v26.4s,  v16.4h,  v2.h[3] // p0(topleft) * filter(0)
        smlal           v26.4s,  v21.4h,  v5.h[3] // p5(left[0]) * filter(5)
        smlal           v26.4s,  v22.4h,  v5.h[7] // p6(left[1]) * filter(6)
        smull2          v27.4s,  v17.8h,  v2.h[4] // p1(top[0]) * filter(1)
        smlal2          v27.4s,  v18.8h,  v2.h[5] // p2(top[1]) * filter(2)
        smlal2          v27.4s,  v19.8h,  v2.h[6] // p3(top[2]) * filter(3)
        smlal2          v27.4s,  v20.8h,  v2.h[7] // p4(top[3]) * filter(4)
        smlal2          v27.4s,  v16.8h,  v2.h[3] // p0(topleft) * filter(0)
        smlal2          v27.4s,  v21.8h,  v5.h[3] // p5(left[0]) * filter(5)
        smlal2          v27.4s,  v22.8h,  v5.h[7] // p6(left[1]) * filter(6)

        subs            w3,  w3,  #16
        sqrshrun        v6.4h,   v26.4s,  #4
        sqrshrun2       v6.8h,   v27.4s,  #4
.endif
        smin            v6.8h,   v6.8h,   v31.8h

        ins             v0.h[2], v2.h[7]
        st4             {v3.d, v4.d, v5.d, v6.d}[0], [x0], #32
        ins             v0.h[0], v6.h[7]
        st4             {v3.d, v4.d, v5.d, v6.d}[1], [x6], #32
        ins             v0.h[1], v6.h[3]
        b.gt            2b
        subs            w4,  w4,  #2
        b.le            9f
        sub             x8,  x6,  w9, uxtw #1
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        mov             w3,  w9
        b               1b
9:
        ret

L(ipred_filter\bpc\()_tbl):
        .hword L(ipred_filter\bpc\()_tbl) - 320b
        .hword L(ipred_filter\bpc\()_tbl) - 160b
        .hword L(ipred_filter\bpc\()_tbl) -  80b
        .hword L(ipred_filter\bpc\()_tbl) -  40b
endfunc
.endm

filter_fn 10
filter_fn 12

function ipred_filter_16bpc_neon, export=1
        ldr             w8,  [sp]
        cmp             w8,  0x3ff
        b.le            ipred_filter_10bpc_neon
        b               ipred_filter_12bpc_neon
endfunc

// void pal_pred_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                          const uint16_t *const pal, const uint8_t *idx,
//                          const int w, const int h);
function pal_pred_16bpc_neon, export=1
        ld1             {v30.8h}, [x2]
        clz             w9,  w4
        adr             x6,  L(pal_pred_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x6, w9, uxtw #1]
        movi            v31.8h,  #1, lsl #8
        sub             x6,  x6,  w9, uxtw
        br              x6
40:
        AARCH64_VALID_JUMP_TARGET
        add             x2,  x0,  x1
        lsl             x1,  x1,  #1
4:
        ld1             {v1.16b}, [x3], #16
        subs            w5,  w5,  #4
        // Restructure v1 from a, b, c, ... into 2*a, 2*a+1, 2*b, 2*b+1, 2*c, 2*c+1, ...
        add             v1.16b,  v1.16b,  v1.16b
        zip1            v0.16b,  v1.16b,  v1.16b
        zip2            v1.16b,  v1.16b,  v1.16b
        add             v0.8h,   v0.8h,   v31.8h
        add             v1.8h,   v1.8h,   v31.8h
        tbl             v0.16b, {v30.16b}, v0.16b
        st1             {v0.d}[0], [x0], x1
        tbl             v1.16b, {v30.16b}, v1.16b
        st1             {v0.d}[1], [x2], x1
        st1             {v1.d}[0], [x0], x1
        st1             {v1.d}[1], [x2], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        add             x2,  x0,  x1
        lsl             x1,  x1,  #1
8:
        ld1             {v2.16b, v3.16b}, [x3], #32
        subs            w5,  w5,  #4
        add             v2.16b,  v2.16b,  v2.16b
        add             v3.16b,  v3.16b,  v3.16b
        zip1            v0.16b,  v2.16b,  v2.16b
        zip2            v1.16b,  v2.16b,  v2.16b
        zip1            v2.16b,  v3.16b,  v3.16b
        zip2            v3.16b,  v3.16b,  v3.16b
        add             v0.8h,   v0.8h,   v31.8h
        add             v1.8h,   v1.8h,   v31.8h
        add             v2.8h,   v2.8h,   v31.8h
        add             v3.8h,   v3.8h,   v31.8h
        tbl             v0.16b, {v30.16b}, v0.16b
        tbl             v1.16b, {v30.16b}, v1.16b
        st1             {v0.8h}, [x0], x1
        tbl             v2.16b, {v30.16b}, v2.16b
        st1             {v1.8h}, [x2], x1
        tbl             v3.16b, {v30.16b}, v3.16b
        st1             {v2.8h}, [x0], x1
        st1             {v3.8h}, [x2], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        add             x2,  x0,  x1
        lsl             x1,  x1,  #1
16:
        ld1             {v4.16b, v5.16b, v6.16b, v7.16b}, [x3], #64
        subs            w5,  w5,  #4
        add             v4.16b,  v4.16b,  v4.16b
        add             v5.16b,  v5.16b,  v5.16b
        add             v6.16b,  v6.16b,  v6.16b
        add             v7.16b,  v7.16b,  v7.16b
        zip1            v0.16b,  v4.16b,  v4.16b
        zip2            v1.16b,  v4.16b,  v4.16b
        zip1            v2.16b,  v5.16b,  v5.16b
        zip2            v3.16b,  v5.16b,  v5.16b
        zip1            v4.16b,  v6.16b,  v6.16b
        zip2            v5.16b,  v6.16b,  v6.16b
        zip1            v6.16b,  v7.16b,  v7.16b
        zip2            v7.16b,  v7.16b,  v7.16b
        add             v0.8h,   v0.8h,   v31.8h
        add             v1.8h,   v1.8h,   v31.8h
        add             v2.8h,   v2.8h,   v31.8h
        add             v3.8h,   v3.8h,   v31.8h
        add             v4.8h,   v4.8h,   v31.8h
        tbl             v0.16b, {v30.16b}, v0.16b
        add             v5.8h,   v5.8h,   v31.8h
        tbl             v1.16b, {v30.16b}, v1.16b
        add             v6.8h,   v6.8h,   v31.8h
        tbl             v2.16b, {v30.16b}, v2.16b
        add             v7.8h,   v7.8h,   v31.8h
        tbl             v3.16b, {v30.16b}, v3.16b
        tbl             v4.16b, {v30.16b}, v4.16b
        tbl             v5.16b, {v30.16b}, v5.16b
        st1             {v0.8h, v1.8h}, [x0], x1
        tbl             v6.16b, {v30.16b}, v6.16b
        st1             {v2.8h, v3.8h}, [x2], x1
        tbl             v7.16b, {v30.16b}, v7.16b
        st1             {v4.8h, v5.8h}, [x0], x1
        st1             {v6.8h, v7.8h}, [x2], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        add             x2,  x0,  x1
        lsl             x1,  x1,  #1
32:
        ld1             {v4.16b, v5.16b, v6.16b, v7.16b}, [x3], #64
        subs            w5,  w5,  #2
        add             v4.16b,  v4.16b,  v4.16b
        add             v5.16b,  v5.16b,  v5.16b
        add             v6.16b,  v6.16b,  v6.16b
        add             v7.16b,  v7.16b,  v7.16b
        zip1            v0.16b,  v4.16b,  v4.16b
        zip2            v1.16b,  v4.16b,  v4.16b
        zip1            v2.16b,  v5.16b,  v5.16b
        zip2            v3.16b,  v5.16b,  v5.16b
        zip1            v4.16b,  v6.16b,  v6.16b
        zip2            v5.16b,  v6.16b,  v6.16b
        zip1            v6.16b,  v7.16b,  v7.16b
        zip2            v7.16b,  v7.16b,  v7.16b
        add             v0.8h,   v0.8h,   v31.8h
        add             v1.8h,   v1.8h,   v31.8h
        add             v2.8h,   v2.8h,   v31.8h
        add             v3.8h,   v3.8h,   v31.8h
        add             v4.8h,   v4.8h,   v31.8h
        tbl             v0.16b, {v30.16b}, v0.16b
        add             v5.8h,   v5.8h,   v31.8h
        tbl             v1.16b, {v30.16b}, v1.16b
        add             v6.8h,   v6.8h,   v31.8h
        tbl             v2.16b, {v30.16b}, v2.16b
        add             v7.8h,   v7.8h,   v31.8h
        tbl             v3.16b, {v30.16b}, v3.16b
        tbl             v4.16b, {v30.16b}, v4.16b
        tbl             v5.16b, {v30.16b}, v5.16b
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        tbl             v6.16b, {v30.16b}, v6.16b
        tbl             v7.16b, {v30.16b}, v7.16b
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], x1
        b.gt            32b
        ret
640:
        AARCH64_VALID_JUMP_TARGET
        add             x2,  x0,  #64
64:
        ld1             {v4.16b, v5.16b, v6.16b, v7.16b}, [x3], #64
        subs            w5,  w5,  #1
        add             v4.16b,  v4.16b,  v4.16b
        add             v5.16b,  v5.16b,  v5.16b
        add             v6.16b,  v6.16b,  v6.16b
        add             v7.16b,  v7.16b,  v7.16b
        zip1            v0.16b,  v4.16b,  v4.16b
        zip2            v1.16b,  v4.16b,  v4.16b
        zip1            v2.16b,  v5.16b,  v5.16b
        zip2            v3.16b,  v5.16b,  v5.16b
        zip1            v4.16b,  v6.16b,  v6.16b
        zip2            v5.16b,  v6.16b,  v6.16b
        zip1            v6.16b,  v7.16b,  v7.16b
        zip2            v7.16b,  v7.16b,  v7.16b
        add             v0.8h,   v0.8h,   v31.8h
        add             v1.8h,   v1.8h,   v31.8h
        add             v2.8h,   v2.8h,   v31.8h
        add             v3.8h,   v3.8h,   v31.8h
        add             v4.8h,   v4.8h,   v31.8h
        tbl             v0.16b, {v30.16b}, v0.16b
        add             v5.8h,   v5.8h,   v31.8h
        tbl             v1.16b, {v30.16b}, v1.16b
        add             v6.8h,   v6.8h,   v31.8h
        tbl             v2.16b, {v30.16b}, v2.16b
        add             v7.8h,   v7.8h,   v31.8h
        tbl             v3.16b, {v30.16b}, v3.16b
        tbl             v4.16b, {v30.16b}, v4.16b
        tbl             v5.16b, {v30.16b}, v5.16b
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        tbl             v6.16b, {v30.16b}, v6.16b
        tbl             v7.16b, {v30.16b}, v7.16b
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], x1
        b.gt            64b
        ret

L(pal_pred_tbl):
        .hword L(pal_pred_tbl) - 640b
        .hword L(pal_pred_tbl) - 320b
        .hword L(pal_pred_tbl) - 160b
        .hword L(pal_pred_tbl) -  80b
        .hword L(pal_pred_tbl) -  40b
endfunc

// void ipred_cfl_128_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height,
//                               const int16_t *ac, const int alpha,
//                               const int bitdepth_max);
function ipred_cfl_128_16bpc_neon, export=1
        dup             v31.8h,  w7   // bitdepth_max
        clz             w9,  w3
        adr             x7,  L(ipred_cfl_128_tbl)
        sub             w9,  w9,  #26
        ldrh            w9,  [x7, w9, uxtw #1]
        urshr           v0.8h,   v31.8h,  #1
        dup             v1.8h,   w6   // alpha
        sub             x7,  x7,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        movi            v30.8h,  #0
        br              x7
L(ipred_cfl_splat_w4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v4.8h, v5.8h}, [x5], #32
        subs            w4,  w4,  #4
        smull           v2.4s,   v4.4h,   v1.4h  // diff = ac * alpha
        smull2          v3.4s,   v4.8h,   v1.8h
        smull           v4.4s,   v5.4h,   v1.4h
        smull2          v5.4s,   v5.8h,   v1.8h
        cmlt            v16.4s,  v2.4s,   #0     // sign
        cmlt            v17.4s,  v3.4s,   #0
        cmlt            v18.4s,  v4.4s,   #0
        cmlt            v19.4s,  v5.4s,   #0
        add             v2.4s,   v2.4s,   v16.4s // diff + sign
        add             v3.4s,   v3.4s,   v17.4s
        add             v4.4s,   v4.4s,   v18.4s
        add             v5.4s,   v5.4s,   v19.4s
        rshrn           v2.4h,   v2.4s,   #6     // (diff + sign + 32) >> 6 = apply_sign()
        rshrn2          v2.8h,   v3.4s,   #6
        rshrn           v3.4h,   v4.4s,   #6
        rshrn2          v3.8h,   v5.4s,   #6
        add             v2.8h,   v2.8h,   v0.8h  // dc + apply_sign()
        add             v3.8h,   v3.8h,   v0.8h
        smax            v2.8h,   v2.8h,   v30.8h
        smax            v3.8h,   v3.8h,   v30.8h
        smin            v2.8h,   v2.8h,   v31.8h
        smin            v3.8h,   v3.8h,   v31.8h
        st1             {v2.d}[0],  [x0], x1
        st1             {v2.d}[1],  [x6], x1
        st1             {v3.d}[0],  [x0], x1
        st1             {v3.d}[1],  [x6], x1
        b.gt            L(ipred_cfl_splat_w4)
        ret
L(ipred_cfl_splat_w8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v4.8h, v5.8h}, [x5], #32
        subs            w4,  w4,  #2
        smull           v2.4s,   v4.4h,   v1.4h  // diff = ac * alpha
        smull2          v3.4s,   v4.8h,   v1.8h
        smull           v4.4s,   v5.4h,   v1.4h
        smull2          v5.4s,   v5.8h,   v1.8h
        cmlt            v16.4s,  v2.4s,   #0     // sign
        cmlt            v17.4s,  v3.4s,   #0
        cmlt            v18.4s,  v4.4s,   #0
        cmlt            v19.4s,  v5.4s,   #0
        add             v2.4s,   v2.4s,   v16.4s // diff + sign
        add             v3.4s,   v3.4s,   v17.4s
        add             v4.4s,   v4.4s,   v18.4s
        add             v5.4s,   v5.4s,   v19.4s
        rshrn           v2.4h,   v2.4s,   #6     // (diff + sign + 32) >> 6 = apply_sign()
        rshrn2          v2.8h,   v3.4s,   #6
        rshrn           v3.4h,   v4.4s,   #6
        rshrn2          v3.8h,   v5.4s,   #6
        add             v2.8h,   v2.8h,   v0.8h  // dc + apply_sign()
        add             v3.8h,   v3.8h,   v0.8h
        smax            v2.8h,   v2.8h,   v30.8h
        smax            v3.8h,   v3.8h,   v30.8h
        smin            v2.8h,   v2.8h,   v31.8h
        smin            v3.8h,   v3.8h,   v31.8h
        st1             {v2.8h},  [x0], x1
        st1             {v3.8h},  [x6], x1
        b.gt            L(ipred_cfl_splat_w8)
        ret
L(ipred_cfl_splat_w16):
        AARCH64_VALID_JUMP_TARGET
        add             x7,  x5,  w3, uxtw #1
        sub             x1,  x1,  w3, uxtw #1
        mov             w9,  w3
1:
        ld1             {v2.8h, v3.8h}, [x5], #32
        ld1             {v4.8h, v5.8h}, [x7], #32
        subs            w3,  w3,  #16
        smull           v16.4s,  v2.4h,   v1.4h  // diff = ac * alpha
        smull2          v17.4s,  v2.8h,   v1.8h
        smull           v18.4s,  v3.4h,   v1.4h
        smull2          v19.4s,  v3.8h,   v1.8h
        smull           v2.4s,   v4.4h,   v1.4h
        smull2          v3.4s,   v4.8h,   v1.8h
        smull           v4.4s,   v5.4h,   v1.4h
        smull2          v5.4s,   v5.8h,   v1.8h
        cmlt            v20.4s,  v16.4s,  #0     // sign
        cmlt            v21.4s,  v17.4s,  #0
        cmlt            v22.4s,  v18.4s,  #0
        cmlt            v23.4s,  v19.4s,  #0
        cmlt            v24.4s,  v2.4s,   #0
        cmlt            v25.4s,  v3.4s,   #0
        cmlt            v26.4s,  v4.4s,   #0
        cmlt            v27.4s,  v5.4s,   #0
        add             v16.4s,  v16.4s,  v20.4s // diff + sign
        add             v17.4s,  v17.4s,  v21.4s
        add             v18.4s,  v18.4s,  v22.4s
        add             v19.4s,  v19.4s,  v23.4s
        add             v2.4s,   v2.4s,   v24.4s
        add             v3.4s,   v3.4s,   v25.4s
        add             v4.4s,   v4.4s,   v26.4s
        add             v5.4s,   v5.4s,   v27.4s
        rshrn           v16.4h,  v16.4s,  #6     // (diff + sign + 32) >> 6 = apply_sign()
        rshrn2          v16.8h,  v17.4s,  #6
        rshrn           v17.4h,  v18.4s,  #6
        rshrn2          v17.8h,  v19.4s,  #6
        rshrn           v6.4h,   v2.4s,   #6
        rshrn2          v6.8h,   v3.4s,   #6
        rshrn           v7.4h,   v4.4s,   #6
        rshrn2          v7.8h,   v5.4s,   #6
        add             v2.8h,   v16.8h,  v0.8h  // dc + apply_sign()
        add             v3.8h,   v17.8h,  v0.8h
        add             v4.8h,   v6.8h,   v0.8h
        add             v5.8h,   v7.8h,   v0.8h
        smax            v2.8h,   v2.8h,   v30.8h
        smax            v3.8h,   v3.8h,   v30.8h
        smax            v4.8h,   v4.8h,   v30.8h
        smax            v5.8h,   v5.8h,   v30.8h
        smin            v2.8h,   v2.8h,   v31.8h
        smin            v3.8h,   v3.8h,   v31.8h
        smin            v4.8h,   v4.8h,   v31.8h
        smin            v5.8h,   v5.8h,   v31.8h
        st1             {v2.8h, v3.8h},  [x0], #32
        st1             {v4.8h, v5.8h},  [x6], #32
        b.gt            1b
        subs            w4,  w4,  #2
        add             x5,  x5,  w9, uxtw #1
        add             x7,  x7,  w9, uxtw #1
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        mov             w3,  w9
        b.gt            1b
        ret

L(ipred_cfl_128_tbl):
L(ipred_cfl_splat_tbl):
        .hword L(ipred_cfl_128_tbl) - L(ipred_cfl_splat_w16)
        .hword L(ipred_cfl_128_tbl) - L(ipred_cfl_splat_w16)
        .hword L(ipred_cfl_128_tbl) - L(ipred_cfl_splat_w8)
        .hword L(ipred_cfl_128_tbl) - L(ipred_cfl_splat_w4)
endfunc

// void ipred_cfl_top_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height,
//                               const int16_t *ac, const int alpha,
//                               const int bitdepth_max);
function ipred_cfl_top_16bpc_neon, export=1
        dup             v31.8h,  w7   // bitdepth_max
        clz             w9,  w3
        adr             x7,  L(ipred_cfl_top_tbl)
        sub             w9,  w9,  #26
        ldrh            w9,  [x7, w9, uxtw #1]
        dup             v1.8h,   w6   // alpha
        add             x2,  x2,  #2
        sub             x7,  x7,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        movi            v30.8h,  #0
        br              x7
4:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h},  [x2]
        addv            h0,      v0.4h
        urshr           v0.4h,   v0.4h,   #2
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w4)
8:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h},  [x2]
        addv            h0,      v0.8h
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w8)
16:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h}, [x2]
        addp            v0.8h,   v2.8h,   v3.8h
        addv            h0,      v0.8h
        urshr           v0.4h,   v0.4h,   #4
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w16)
32:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h, v4.8h, v5.8h}, [x2]
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v0.8h,   v2.8h,   v4.8h
        uaddlv          s0,      v0.8h
        rshrn           v0.4h,   v0.4s,   #5
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w16)

L(ipred_cfl_top_tbl):
        .hword L(ipred_cfl_top_tbl) - 32b
        .hword L(ipred_cfl_top_tbl) - 16b
        .hword L(ipred_cfl_top_tbl) -  8b
        .hword L(ipred_cfl_top_tbl) -  4b
endfunc

// void ipred_cfl_left_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                                const pixel *const topleft,
//                                const int width, const int height,
//                                const int16_t *ac, const int alpha,
//                                const int bitdepth_max);
function ipred_cfl_left_16bpc_neon, export=1
        dup             v31.8h,  w7   // bitdepth_max
        sub             x2,  x2,  w4, uxtw #1
        clz             w9,  w3
        clz             w8,  w4
        adr             x10, L(ipred_cfl_splat_tbl)
        adr             x7,  L(ipred_cfl_left_tbl)
        sub             w9,  w9,  #26
        sub             w8,  w8,  #26
        ldrh            w9,  [x10, w9, uxtw #1]
        ldrh            w8,  [x7,  w8, uxtw #1]
        dup             v1.8h,   w6   // alpha
        sub             x9,  x10, w9, uxtw
        sub             x7,  x7,  w8, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        movi            v30.8h,  #0
        br              x7

L(ipred_cfl_left_h4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h},  [x2]
        addv            h0,      v0.4h
        urshr           v0.4h,   v0.4h,   #2
        dup             v0.8h,   v0.h[0]
        br              x9

L(ipred_cfl_left_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h},  [x2]
        addv            h0,      v0.8h
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
        br              x9

L(ipred_cfl_left_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h}, [x2]
        addp            v0.8h,   v2.8h,   v3.8h
        addv            h0,      v0.8h
        urshr           v0.4h,   v0.4h,   #4
        dup             v0.8h,   v0.h[0]
        br              x9

L(ipred_cfl_left_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h, v4.8h, v5.8h}, [x2]
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v0.8h,   v2.8h,   v4.8h
        uaddlv          s0,      v0.8h
        rshrn           v0.4h,   v0.4s,   #5
        dup             v0.8h,   v0.h[0]
        br              x9

L(ipred_cfl_left_tbl):
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h32)
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h16)
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h8)
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h4)
endfunc

// void ipred_cfl_16bpc_neon(pixel *dst, const ptrdiff_t stride,
//                           const pixel *const topleft,
//                           const int width, const int height,
//                           const int16_t *ac, const int alpha,
//                           const int bitdepth_max);
function ipred_cfl_16bpc_neon, export=1
        dup             v31.8h,  w7              // bitdepth_max
        sub             x2,  x2,  w4, uxtw #1
        add             w8,  w3,  w4             // width + height
        dup             v1.8h,   w6              // alpha
        clz             w9,  w3
        clz             w6,  w4
        dup             v16.4s, w8               // width + height
        adr             x7,  L(ipred_cfl_tbl)
        rbit            w8,  w8                  // rbit(width + height)
        sub             w9,  w9,  #22            // 26 leading bits, minus table offset 4
        sub             w6,  w6,  #26
        clz             w8,  w8                  // ctz(width + height)
        ldrh            w9,  [x7, w9, uxtw #1]
        ldrh            w6,  [x7, w6, uxtw #1]
        neg             w8,  w8                  // -ctz(width + height)
        sub             x9,  x7,  w9, uxtw
        sub             x7,  x7,  w6, uxtw
        ushr            v16.4s,  v16.4s,  #1     // (width + height) >> 1
        dup             v17.4s,  w8              // -ctz(width + height)
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        movi            v30.8h,  #0
        br              x7

L(ipred_cfl_h4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h},  [x2], #8
        uaddlv          s0,      v0.4h
        add             x2,  x2,  #2
        br              x9
L(ipred_cfl_w4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.4h},  [x2]
        add             v0.2s,   v0.2s,   v16.2s
        uaddlv          s2,      v2.4h
        cmp             w4,  #4
        add             v0.2s,   v0.2s,   v2.2s
        ushl            v0.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 8/16
        cmp             w4,  #16
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v0.2s,   v0.2s,   v16.2s
        ushr            v0.2s,   v0.2s,   #17
1:
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w4)

L(ipred_cfl_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8h},  [x2], #16
        uaddlv          s0,      v0.8h
        add             x2,  x2,  #2
        br              x9
L(ipred_cfl_w8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h},  [x2]
        add             v0.2s,   v0.2s,   v16.2s
        uaddlv          s2,      v2.8h
        cmp             w4,  #8
        add             v0.2s,   v0.2s,   v2.2s
        ushl            v0.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 4/16/32
        cmp             w4,  #32
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v0.2s,   v0.2s,   v16.2s
        ushr            v0.2s,   v0.2s,   #17
1:
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w8)

L(ipred_cfl_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h}, [x2], #32
        addp            v0.8h,   v2.8h,   v3.8h
        add             x2,  x2,  #2
        uaddlv          s0,      v0.8h
        br              x9
L(ipred_cfl_w16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h}, [x2]
        add             v0.2s,   v0.2s,   v16.2s
        addp            v2.8h,   v2.8h,   v3.8h
        uaddlv          s2,      v2.8h
        cmp             w4,  #16
        add             v0.2s,   v0.2s,   v2.2s
        ushl            v0.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 4/8/32
        tst             w4,  #(32+16+8) // 16 added to make a consecutive bitmask
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v0.2s,   v0.2s,   v16.2s
        ushr            v0.2s,   v0.2s,   #17
1:
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w16)

L(ipred_cfl_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h, v4.8h, v5.8h}, [x2], #64
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v0.8h,   v2.8h,   v4.8h
        add             x2,  x2,  #2
        uaddlv          s0,      v0.8h
        br              x9
L(ipred_cfl_w32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h, v4.8h, v5.8h}, [x2]
        add             v0.4s,   v0.4s,   v16.4s
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v2.8h,   v2.8h,   v4.8h
        cmp             w4,  #32
        uaddlv          s2,      v2.8h
        add             v0.2s,   v0.2s,   v2.2s
        ushl            v0.2s,   v0.2s,   v17.2s
        b.eq            1f
        // h = 8/16
        cmp             w4,  #8
        mov             w16, #0x6667
        mov             w17, #0xAAAB
        csel            w16, w16, w17, eq
        dup             v16.2s,  w16
        mul             v0.2s,   v0.2s,   v16.2s
        ushr            v0.2s,   v0.2s,   #17
1:
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w16)

L(ipred_cfl_tbl):
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_h32)
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_h16)
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_h8)
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_h4)
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_w32)
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_w16)
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_w8)
        .hword L(ipred_cfl_tbl) - L(ipred_cfl_w4)
endfunc

// void cfl_ac_420_16bpc_neon(int16_t *const ac, const pixel *const ypx,
//                            const ptrdiff_t stride, const int w_pad,
//                            const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_420_16bpc_neon, export=1
        clz             w8,  w5
        lsl             w4,  w4,  #2
        adr             x7,  L(ipred_cfl_ac_420_tbl)
        sub             w8,  w8,  #27
        ldrh            w8,  [x7, w8, uxtw #1]
        movi            v24.4s,  #0
        movi            v25.4s,  #0
        movi            v26.4s,  #0
        movi            v27.4s,  #0
        sub             x7,  x7,  w8, uxtw
        sub             w8,  w6,  w4         // height - h_pad
        rbit            w9,  w5              // rbit(width)
        rbit            w10, w6              // rbit(height)
        clz             w9,  w9              // ctz(width)
        clz             w10, w10             // ctz(height)
        add             w9,  w9,  w10        // log2sz
        add             x10, x1,  x2
        dup             v31.4s,  w9
        lsl             x2,  x2,  #1
        neg             v31.4s,  v31.4s      // -log2sz
        br              x7

L(ipred_cfl_ac_420_w4):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input
        ld1             {v0.8h}, [x1],  x2
        ld1             {v1.8h}, [x10], x2
        ld1             {v2.8h}, [x1],  x2
        ld1             {v3.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v2.8h
        addp            v1.8h,   v1.8h,   v3.8h
        add             v0.8h,   v0.8h,   v1.8h
        shl             v0.8h,   v0.8h,   #1
        subs            w8,  w8,  #2
        st1             {v0.8h}, [x0], #16
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        b.gt            1b
        trn2            v1.2d,   v0.2d,   v0.2d
        trn2            v0.2d,   v0.2d,   v0.2d
L(ipred_cfl_ac_420_w4_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], #32
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        b.gt            2b
3:
L(ipred_cfl_ac_420_w4_calc_subtract_dc):
        // Aggregate the sums
        add             v24.4s,  v24.4s,  v25.4s
        add             v26.4s,  v26.4s,  v27.4s
        add             v0.4s,   v24.4s,  v26.4s
        addv            s0,  v0.4s                // sum
        sub             x0,  x0,  w6, uxtw #3
        urshl           v4.2s,   v0.2s,   v31.2s  // (sum + (1 << (log2sz - 1)))  >>= log2sz
        dup             v4.8h,   v4.h[0]
6:      // Subtract dc from ac
        ld1             {v0.8h, v1.8h}, [x0]
        subs            w6,  w6,  #4
        sub             v0.8h,   v0.8h,   v4.8h
        sub             v1.8h,   v1.8h,   v4.8h
        st1             {v0.8h, v1.8h}, [x0], #32
        b.gt            6b
        ret

L(ipred_cfl_ac_420_w8):
        AARCH64_VALID_JUMP_TARGET
        cbnz            w3,  L(ipred_cfl_ac_420_w8_wpad)
1:      // Copy and subsample input, without padding
        ld1             {v0.8h, v1.8h}, [x1],  x2
        ld1             {v2.8h, v3.8h}, [x10], x2
        ld1             {v4.8h, v5.8h}, [x1],  x2
        addp            v0.8h,   v0.8h,   v1.8h
        ld1             {v6.8h, v7.8h}, [x10], x2
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        add             v0.8h,   v0.8h,   v2.8h
        add             v4.8h,   v4.8h,   v6.8h
        shl             v0.8h,   v0.8h,   #1
        shl             v1.8h,   v4.8h,   #1
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h}, [x0], #32
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        b.gt            1b
        mov             v0.16b,  v1.16b
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_420_w8_wpad):
1:      // Copy and subsample input, padding 4
        ld1             {v0.8h}, [x1],  x2
        ld1             {v1.8h}, [x10], x2
        ld1             {v2.8h}, [x1],  x2
        ld1             {v3.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v2.8h
        addp            v1.8h,   v1.8h,   v3.8h
        add             v0.8h,   v0.8h,   v1.8h
        shl             v0.8h,   v0.8h,   #1
        dup             v1.4h,   v0.h[3]
        dup             v3.4h,   v0.h[7]
        trn2            v2.2d,   v0.2d,   v0.2d
        subs            w8,  w8,  #2
        st1             {v0.4h, v1.4h, v2.4h, v3.4h}, [x0], #32
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw           v25.4s,  v25.4s,  v1.4h
        uaddw           v26.4s,  v26.4s,  v2.4h
        uaddw           v27.4s,  v27.4s,  v3.4h
        b.gt            1b
        trn1            v0.2d,   v2.2d,   v3.2d
        trn1            v1.2d,   v2.2d,   v3.2d

L(ipred_cfl_ac_420_w8_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], #32
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        st1             {v0.8h, v1.8h}, [x0], #32
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        b.gt            2b
3:

        // Double the height and reuse the w4 summing/subtracting
        lsl             w6,  w6,  #1
        b               L(ipred_cfl_ac_420_w4_calc_subtract_dc)

L(ipred_cfl_ac_420_w16):
        AARCH64_VALID_JUMP_TARGET
        adr             x7,  L(ipred_cfl_ac_420_w16_tbl)
        ldrh            w3,  [x7, w3, uxtw #1]
        sub             x7,  x7,  w3, uxtw
        br              x7

L(ipred_cfl_ac_420_w16_wpad0):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, without padding
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x1],  x2
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        ld1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x1],  x2
        add             v0.8h,   v0.8h,   v4.8h
        ld1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x10], x2
        add             v2.8h,   v2.8h,   v6.8h
        addp            v16.8h,  v16.8h,  v17.8h
        addp            v18.8h,  v18.8h,  v19.8h
        addp            v20.8h,  v20.8h,  v21.8h
        addp            v22.8h,  v22.8h,  v23.8h
        add             v16.8h,  v16.8h,  v20.8h
        add             v18.8h,  v18.8h,  v22.8h
        shl             v0.8h,   v0.8h,   #1
        shl             v1.8h,   v2.8h,   #1
        shl             v2.8h,   v16.8h,  #1
        shl             v3.8h,   v18.8h,  #1
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad1):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 4
        ldr             q2,  [x1,  #32]
        ld1             {v0.8h, v1.8h}, [x1],  x2
        ldr             q5,  [x10, #32]
        ld1             {v3.8h, v4.8h}, [x10], x2
        addp            v2.8h,   v2.8h,   v2.8h
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v5.8h,   v5.8h,   v5.8h
        addp            v3.8h,   v3.8h,   v4.8h
        ldr             q18, [x1,  #32]
        add             v2.4h,   v2.4h,   v5.4h
        ld1             {v16.8h, v17.8h}, [x1],  x2
        add             v0.8h,   v0.8h,   v3.8h
        ldr             q21, [x10, #32]
        ld1             {v19.8h, v20.8h}, [x10], x2
        addp            v18.8h,  v18.8h,  v18.8h
        addp            v16.8h,  v16.8h,  v17.8h
        addp            v21.8h,  v21.8h,  v21.8h
        addp            v19.8h,  v19.8h,  v20.8h
        add             v18.4h,  v18.4h,  v21.4h
        add             v16.8h,  v16.8h,  v19.8h
        shl             v1.4h,   v2.4h,   #1
        shl             v0.8h,   v0.8h,   #1
        shl             v3.4h,   v18.4h,  #1
        shl             v2.8h,   v16.8h,  #1
        dup             v4.4h,   v1.h[3]
        dup             v5.4h,   v3.h[3]
        trn1            v1.2d,   v1.2d,   v4.2d
        trn1            v3.2d,   v3.2d,   v5.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad2):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 8
        ld1             {v0.8h, v1.8h}, [x1],  x2
        ld1             {v2.8h, v3.8h}, [x10], x2
        ld1             {v4.8h, v5.8h}, [x1],  x2
        addp            v0.8h,   v0.8h,   v1.8h
        ld1             {v6.8h, v7.8h}, [x10], x2
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        add             v0.8h,   v0.8h,   v2.8h
        add             v4.8h,   v4.8h,   v6.8h
        shl             v0.8h,   v0.8h,   #1
        shl             v2.8h,   v4.8h,   #1
        dup             v1.8h,   v0.h[7]
        dup             v3.8h,   v2.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad3):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 12
        ld1             {v0.8h}, [x1],  x2
        ld1             {v2.8h}, [x10], x2
        ld1             {v4.8h}, [x1],  x2
        ld1             {v6.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v4.8h
        addp            v2.8h,   v2.8h,   v6.8h
        add             v0.8h,   v0.8h,   v2.8h
        shl             v0.8h,   v0.8h,   #1
        dup             v1.8h,   v0.h[3]
        dup             v3.8h,   v0.h[7]
        trn2            v2.2d,   v0.2d,   v3.2d
        trn1            v0.2d,   v0.2d,   v1.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b

L(ipred_cfl_ac_420_w16_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            2b
3:

        // Quadruple the height and reuse the w4 summing/subtracting
        lsl             w6,  w6,  #2
        b               L(ipred_cfl_ac_420_w4_calc_subtract_dc)

L(ipred_cfl_ac_420_tbl):
        .hword L(ipred_cfl_ac_420_tbl) - L(ipred_cfl_ac_420_w16)
        .hword L(ipred_cfl_ac_420_tbl) - L(ipred_cfl_ac_420_w8)
        .hword L(ipred_cfl_ac_420_tbl) - L(ipred_cfl_ac_420_w4)
        .hword 0

L(ipred_cfl_ac_420_w16_tbl):
        .hword L(ipred_cfl_ac_420_w16_tbl) - L(ipred_cfl_ac_420_w16_wpad0)
        .hword L(ipred_cfl_ac_420_w16_tbl) - L(ipred_cfl_ac_420_w16_wpad1)
        .hword L(ipred_cfl_ac_420_w16_tbl) - L(ipred_cfl_ac_420_w16_wpad2)
        .hword L(ipred_cfl_ac_420_w16_tbl) - L(ipred_cfl_ac_420_w16_wpad3)
endfunc

// void cfl_ac_422_16bpc_neon(int16_t *const ac, const pixel *const ypx,
//                            const ptrdiff_t stride, const int w_pad,
//                            const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_422_16bpc_neon, export=1
        clz             w8,  w5
        lsl             w4,  w4,  #2
        adr             x7,  L(ipred_cfl_ac_422_tbl)
        sub             w8,  w8,  #27
        ldrh            w8,  [x7, w8, uxtw #1]
        movi            v24.4s,  #0
        movi            v25.4s,  #0
        movi            v26.4s,  #0
        movi            v27.4s,  #0
        sub             x7,  x7,  w8, uxtw
        sub             w8,  w6,  w4         // height - h_pad
        rbit            w9,  w5              // rbit(width)
        rbit            w10, w6              // rbit(height)
        clz             w9,  w9              // ctz(width)
        clz             w10, w10             // ctz(height)
        add             w9,  w9,  w10        // log2sz
        add             x10, x1,  x2
        dup             v31.4s,  w9
        lsl             x2,  x2,  #1
        neg             v31.4s,  v31.4s      // -log2sz
        br              x7

L(ipred_cfl_ac_422_w4):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input
        ld1             {v0.8h}, [x1],  x2
        ld1             {v1.8h}, [x10], x2
        ld1             {v2.8h}, [x1],  x2
        ld1             {v3.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        shl             v0.8h,   v0.8h,   #2
        shl             v1.8h,   v2.8h,   #2
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h}, [x0], #32
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        b.gt            1b
        trn2            v0.2d,   v1.2d,   v1.2d
        trn2            v1.2d,   v1.2d,   v1.2d
        b               L(ipred_cfl_ac_420_w4_hpad)

L(ipred_cfl_ac_422_w8):
        AARCH64_VALID_JUMP_TARGET
        cbnz            w3,  L(ipred_cfl_ac_422_w8_wpad)
1:      // Copy and subsample input, without padding
        ld1             {v0.8h, v1.8h}, [x1],  x2
        ld1             {v2.8h, v3.8h}, [x10], x2
        ld1             {v4.8h, v5.8h}, [x1],  x2
        addp            v0.8h,   v0.8h,   v1.8h
        ld1             {v6.8h, v7.8h}, [x10], x2
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        shl             v0.8h,   v0.8h,   #2
        shl             v1.8h,   v2.8h,   #2
        shl             v2.8h,   v4.8h,   #2
        shl             v3.8h,   v6.8h,   #2
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v3.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_422_w8_wpad):
1:      // Copy and subsample input, padding 4
        ld1             {v0.8h}, [x1],  x2
        ld1             {v1.8h}, [x10], x2
        ld1             {v2.8h}, [x1],  x2
        ld1             {v3.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        shl             v0.8h,   v0.8h,   #2
        shl             v2.8h,   v2.8h,   #2
        dup             v4.4h,   v0.h[3]
        dup             v5.8h,   v0.h[7]
        dup             v6.4h,   v2.h[3]
        dup             v7.8h,   v2.h[7]
        trn2            v1.2d,   v0.2d,   v5.2d
        trn1            v0.2d,   v0.2d,   v4.2d
        trn2            v3.2d,   v2.2d,   v7.2d
        trn1            v2.2d,   v2.2d,   v6.2d
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v3.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_422_w16):
        AARCH64_VALID_JUMP_TARGET
        adr             x7,  L(ipred_cfl_ac_422_w16_tbl)
        ldrh            w3,  [x7, w3, uxtw #1]
        sub             x7,  x7,  w3, uxtw
        br              x7

L(ipred_cfl_ac_422_w16_wpad0):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, without padding
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x1],  x2
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        addp            v4.8h,   v4.8h,   v5.8h
        addp            v6.8h,   v6.8h,   v7.8h
        shl             v0.8h,   v0.8h,   #2
        shl             v1.8h,   v2.8h,   #2
        shl             v2.8h,   v4.8h,   #2
        shl             v3.8h,   v6.8h,   #2
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad1):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 4
        ldr             q2,  [x1,  #32]
        ld1             {v0.8h, v1.8h}, [x1],  x2
        ldr             q6,  [x10, #32]
        ld1             {v4.8h, v5.8h}, [x10], x2
        addp            v2.8h,   v2.8h,   v2.8h
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v6.8h,   v6.8h,   v6.8h
        addp            v4.8h,   v4.8h,   v5.8h
        shl             v1.4h,   v2.4h,   #2
        shl             v0.8h,   v0.8h,   #2
        shl             v3.4h,   v6.4h,   #2
        shl             v2.8h,   v4.8h,   #2
        dup             v4.4h,   v1.h[3]
        dup             v5.4h,   v3.h[3]
        trn1            v1.2d,   v1.2d,   v4.2d
        trn1            v3.2d,   v3.2d,   v5.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad2):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 8
        ld1             {v0.8h, v1.8h}, [x1],  x2
        ld1             {v2.8h, v3.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v1.8h
        addp            v2.8h,   v2.8h,   v3.8h
        shl             v0.8h,   v0.8h,   #2
        shl             v2.8h,   v2.8h,   #2
        dup             v1.8h,   v0.h[7]
        dup             v3.8h,   v2.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad3):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 12
        ld1             {v0.8h}, [x1],  x2
        ld1             {v2.8h}, [x10], x2
        addp            v0.8h,   v0.8h,   v0.8h
        addp            v2.8h,   v2.8h,   v2.8h
        shl             v0.4h,   v0.4h,   #2
        shl             v2.4h,   v2.4h,   #2
        dup             v1.8h,   v0.h[3]
        dup             v3.8h,   v2.h[3]
        trn1            v0.2d,   v0.2d,   v1.2d
        trn1            v2.2d,   v2.2d,   v3.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_tbl):
        .hword L(ipred_cfl_ac_422_tbl) - L(ipred_cfl_ac_422_w16)
        .hword L(ipred_cfl_ac_422_tbl) - L(ipred_cfl_ac_422_w8)
        .hword L(ipred_cfl_ac_422_tbl) - L(ipred_cfl_ac_422_w4)
        .hword 0

L(ipred_cfl_ac_422_w16_tbl):
        .hword L(ipred_cfl_ac_422_w16_tbl) - L(ipred_cfl_ac_422_w16_wpad0)
        .hword L(ipred_cfl_ac_422_w16_tbl) - L(ipred_cfl_ac_422_w16_wpad1)
        .hword L(ipred_cfl_ac_422_w16_tbl) - L(ipred_cfl_ac_422_w16_wpad2)
        .hword L(ipred_cfl_ac_422_w16_tbl) - L(ipred_cfl_ac_422_w16_wpad3)
endfunc

// void cfl_ac_444_16bpc_neon(int16_t *const ac, const pixel *const ypx,
//                            const ptrdiff_t stride, const int w_pad,
//                            const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_444_16bpc_neon, export=1
        clz             w8,  w5
        lsl             w4,  w4,  #2
        adr             x7,  L(ipred_cfl_ac_444_tbl)
        sub             w8,  w8,  #26
        ldrh            w8,  [x7, w8, uxtw #1]
        movi            v24.4s,  #0
        movi            v25.4s,  #0
        movi            v26.4s,  #0
        movi            v27.4s,  #0
        sub             x7,  x7,  w8, uxtw
        sub             w8,  w6,  w4         // height - h_pad
        rbit            w9,  w5              // rbit(width)
        rbit            w10, w6              // rbit(height)
        clz             w9,  w9              // ctz(width)
        clz             w10, w10             // ctz(height)
        add             w9,  w9,  w10        // log2sz
        add             x10, x1,  x2
        dup             v31.4s,  w9
        lsl             x2,  x2,  #1
        neg             v31.4s,  v31.4s      // -log2sz
        br              x7

L(ipred_cfl_ac_444_w4):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input
        ld1             {v0.4h},   [x1],  x2
        ld1             {v0.d}[1], [x10], x2
        ld1             {v1.4h},   [x1],  x2
        ld1             {v1.d}[1], [x10], x2
        shl             v0.8h,   v0.8h,   #3
        shl             v1.8h,   v1.8h,   #3
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h}, [x0], #32
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        b.gt            1b
        trn2            v0.2d,   v1.2d,   v1.2d
        trn2            v1.2d,   v1.2d,   v1.2d
        b               L(ipred_cfl_ac_420_w4_hpad)

L(ipred_cfl_ac_444_w8):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input
        ld1             {v0.8h}, [x1],  x2
        ld1             {v1.8h}, [x10], x2
        ld1             {v2.8h}, [x1],  x2
        shl             v0.8h,   v0.8h,   #3
        ld1             {v3.8h}, [x10], x2
        shl             v1.8h,   v1.8h,   #3
        shl             v2.8h,   v2.8h,   #3
        shl             v3.8h,   v3.8h,   #3
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v3.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_444_w16):
        AARCH64_VALID_JUMP_TARGET
        cbnz            w3,  L(ipred_cfl_ac_444_w16_wpad)
1:      // Copy and expand input, without padding
        ld1             {v0.8h, v1.8h}, [x1],  x2
        ld1             {v2.8h, v3.8h}, [x10], x2
        shl             v0.8h,   v0.8h,   #3
        shl             v1.8h,   v1.8h,   #3
        shl             v2.8h,   v2.8h,   #3
        shl             v3.8h,   v3.8h,   #3
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_444_w16_wpad):
1:      // Copy and expand input, padding 8
        ld1             {v0.8h}, [x1],  x2
        ld1             {v2.8h}, [x10], x2
        shl             v0.8h,   v0.8h,   #3
        shl             v2.8h,   v2.8h,   #3
        dup             v1.8h,   v0.h[7]
        dup             v3.8h,   v2.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_444_w32):
        AARCH64_VALID_JUMP_TARGET
        adr             x7,  L(ipred_cfl_ac_444_w32_tbl)
        ldrh            w3,  [x7, w3, uxtw] // (w3>>1) << 1
        lsr             x2,  x2,  #1 // Restore the stride to one line increments
        sub             x7,  x7,  w3, uxtw
        br              x7

L(ipred_cfl_ac_444_w32_wpad0):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, without padding
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x1],  x2
        shl             v0.8h,   v0.8h,   #3
        shl             v1.8h,   v1.8h,   #3
        shl             v2.8h,   v2.8h,   #3
        shl             v3.8h,   v3.8h,   #3
        subs            w8,  w8,  #1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad2):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, padding 8
        ld1             {v0.8h, v1.8h, v2.8h}, [x1],  x2
        shl             v2.8h,   v2.8h,   #3
        shl             v0.8h,   v0.8h,   #3
        shl             v1.8h,   v1.8h,   #3
        dup             v3.8h,   v2.h[7]
        subs            w8,  w8,  #1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad4):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, padding 16
        ld1             {v0.8h, v1.8h}, [x1],  x2
        shl             v1.8h,   v1.8h,   #3
        shl             v0.8h,   v0.8h,   #3
        dup             v2.8h,   v1.h[7]
        dup             v3.8h,   v1.h[7]
        subs            w8,  w8,  #1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad6):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, padding 24
        ld1             {v0.8h}, [x1],  x2
        shl             v0.8h,   v0.8h,   #3
        dup             v1.8h,   v0.h[7]
        dup             v2.8h,   v0.h[7]
        dup             v3.8h,   v0.h[7]
        subs            w8,  w8,  #1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            1b

L(ipred_cfl_ac_444_w32_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        uaddw           v24.4s,  v24.4s,  v0.4h
        uaddw2          v25.4s,  v25.4s,  v0.8h
        uaddw           v26.4s,  v26.4s,  v1.4h
        uaddw2          v27.4s,  v27.4s,  v1.8h
        uaddw           v24.4s,  v24.4s,  v2.4h
        uaddw2          v25.4s,  v25.4s,  v2.8h
        uaddw           v26.4s,  v26.4s,  v3.4h
        uaddw2          v27.4s,  v27.4s,  v3.8h
        b.gt            2b
3:

        //  Multiply the height by eight and reuse the w4 subtracting
        lsl             w6,  w6,  #3
        b               L(ipred_cfl_ac_420_w4_calc_subtract_dc)

L(ipred_cfl_ac_444_tbl):
        .hword L(ipred_cfl_ac_444_tbl) - L(ipred_cfl_ac_444_w32)
        .hword L(ipred_cfl_ac_444_tbl) - L(ipred_cfl_ac_444_w16)
        .hword L(ipred_cfl_ac_444_tbl) - L(ipred_cfl_ac_444_w8)
        .hword L(ipred_cfl_ac_444_tbl) - L(ipred_cfl_ac_444_w4)

L(ipred_cfl_ac_444_w32_tbl):
        .hword L(ipred_cfl_ac_444_w32_tbl) - L(ipred_cfl_ac_444_w32_wpad0)
        .hword L(ipred_cfl_ac_444_w32_tbl) - L(ipred_cfl_ac_444_w32_wpad2)
        .hword L(ipred_cfl_ac_444_w32_tbl) - L(ipred_cfl_ac_444_w32_wpad4)
        .hword L(ipred_cfl_ac_444_w32_tbl) - L(ipred_cfl_ac_444_w32_wpad6)
endfunc
