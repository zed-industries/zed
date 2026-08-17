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

// void ipred_dc_128_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                             const pixel *const topleft,
//                             const int width, const int height, const int a,
//                             const int max_width, const int max_height);
function ipred_dc_128_8bpc_neon, export=1
        clz             w3,  w3
        adr             x5,  L(ipred_dc_128_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        movi            v0.16b,  #128
        sub             x5,  x5,  w3, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
4:
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        b.gt            4b
        ret
8:
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        b.gt            8b
        ret
16:
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        movi            v1.16b,  #128
32:
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        b.gt            32b
        ret
640:
        AARCH64_VALID_JUMP_TARGET
        movi            v1.16b,  #128
        movi            v2.16b,  #128
        movi            v3.16b,  #128
64:
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        b.gt            64b
        ret

L(ipred_dc_128_tbl):
        .hword L(ipred_dc_128_tbl) - 640b
        .hword L(ipred_dc_128_tbl) - 320b
        .hword L(ipred_dc_128_tbl) -  16b
        .hword L(ipred_dc_128_tbl) -   8b
        .hword L(ipred_dc_128_tbl) -   4b
endfunc

// void ipred_v_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                        const pixel *const topleft,
//                        const int width, const int height, const int a,
//                        const int max_width, const int max_height);
function ipred_v_8bpc_neon, export=1
        clz             w3,  w3
        adr             x5,  L(ipred_v_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        add             x2,  x2,  #1
        sub             x5,  x5,  w3, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.s}[0],  [x2]
4:
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [x2]
8:
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b}, [x2]
16:
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b}, [x2]
32:
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        b.gt            32b
        ret
640:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x2]
64:
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        b.gt            64b
        ret

L(ipred_v_tbl):
        .hword L(ipred_v_tbl) - 640b
        .hword L(ipred_v_tbl) - 320b
        .hword L(ipred_v_tbl) - 160b
        .hword L(ipred_v_tbl) -  80b
        .hword L(ipred_v_tbl) -  40b
endfunc

// void ipred_h_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                        const pixel *const topleft,
//                        const int width, const int height, const int a,
//                        const int max_width, const int max_height);
function ipred_h_8bpc_neon, export=1
        clz             w3,  w3
        adr             x5,  L(ipred_h_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        sub             x2,  x2,  #4
        sub             x5,  x5,  w3, uxtw
        mov             x7,  #-4
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
4:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7
        st1             {v3.s}[0],  [x0], x1
        st1             {v2.s}[0],  [x6], x1
        subs            w4,  w4,  #4
        st1             {v1.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        b.gt            4b
        ret
8:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7
        st1             {v3.8b},  [x0], x1
        st1             {v2.8b},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v1.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        b.gt            8b
        ret
16:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.16b, v1.16b, v2.16b, v3.16b},  [x2], x7
        st1             {v3.16b}, [x0], x1
        st1             {v2.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v1.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            16b
        ret
32:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.16b, v1.16b, v2.16b, v3.16b},  [x2], x7
        str             q3,  [x0, #16]
        str             q2,  [x6, #16]
        st1             {v3.16b}, [x0], x1
        st1             {v2.16b}, [x6], x1
        subs            w4,  w4,  #4
        str             q1,  [x0, #16]
        str             q0,  [x6, #16]
        st1             {v1.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            32b
        ret
64:
        AARCH64_VALID_JUMP_TARGET
        ld4r            {v0.16b, v1.16b, v2.16b, v3.16b},  [x2], x7
        str             q3,  [x0, #16]
        str             q2,  [x6, #16]
        stp             q3,  q3,  [x0, #32]
        stp             q2,  q2,  [x6, #32]
        st1             {v3.16b}, [x0], x1
        st1             {v2.16b}, [x6], x1
        subs            w4,  w4,  #4
        str             q1,  [x0, #16]
        str             q0,  [x6, #16]
        stp             q1,  q1,  [x0, #32]
        stp             q0,  q0,  [x6, #32]
        st1             {v1.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            64b
        ret

L(ipred_h_tbl):
        .hword L(ipred_h_tbl) - 64b
        .hword L(ipred_h_tbl) - 32b
        .hword L(ipred_h_tbl) - 16b
        .hword L(ipred_h_tbl) -  8b
        .hword L(ipred_h_tbl) -  4b
endfunc

// void ipred_dc_top_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                             const pixel *const topleft,
//                             const int width, const int height, const int a,
//                             const int max_width, const int max_height);
function ipred_dc_top_8bpc_neon, export=1
        clz             w3,  w3
        adr             x5,  L(ipred_dc_top_tbl)
        sub             w3,  w3,  #25
        ldrh            w3,  [x5, w3, uxtw #1]
        add             x2,  x2,  #1
        sub             x5,  x5,  w3, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v0.2s},  [x2]
        uaddlv          h0,      v0.8b
        rshrn           v0.8b,   v0.8h,   #3
        dup             v0.8b,   v0.b[0]
4:
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [x2]
        uaddlv          h0,      v0.8b
        rshrn           v0.8b,   v0.8h,   #3
        dup             v0.8b,   v0.b[0]
8:
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b}, [x2]
        uaddlv          h0,      v0.16b
        rshrn           v0.8b,   v0.8h,   #4
        dup             v0.16b,  v0.b[0]
16:
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b}, [x2]
        uaddlv          h0,      v0.16b
        uaddlv          h1,      v1.16b
        add             v2.4h,   v0.4h,   v1.4h
        rshrn           v2.8b,   v2.8h,   #5
        dup             v0.16b,  v2.b[0]
        dup             v1.16b,  v2.b[0]
32:
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        b.gt            32b
        ret
640:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x2]
        uaddlv          h0,      v0.16b
        uaddlv          h1,      v1.16b
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        add             v4.4h,   v0.4h,   v1.4h
        add             v5.4h,   v2.4h,   v3.4h
        add             v4.4h,   v4.4h,   v5.4h
        rshrn           v4.8b,   v4.8h,   #6
        dup             v0.16b,  v4.b[0]
        dup             v1.16b,  v4.b[0]
        dup             v2.16b,  v4.b[0]
        dup             v3.16b,  v4.b[0]
64:
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        b.gt            64b
        ret

L(ipred_dc_top_tbl):
        .hword L(ipred_dc_top_tbl) - 640b
        .hword L(ipred_dc_top_tbl) - 320b
        .hword L(ipred_dc_top_tbl) - 160b
        .hword L(ipred_dc_top_tbl) -  80b
        .hword L(ipred_dc_top_tbl) -  40b
endfunc

// void ipred_dc_left_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height, const int a,
//                              const int max_width, const int max_height);
function ipred_dc_left_8bpc_neon, export=1
        sub             x2,  x2,  w4, uxtw
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
        ld1r            {v0.2s},  [x2]
        uaddlv          h0,      v0.8b
        rshrn           v0.8b,   v0.8h,   #3
        dup             v0.16b,  v0.b[0]
        br              x3
L(ipred_dc_left_w4):
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        b.gt            L(ipred_dc_left_w4)
        ret

L(ipred_dc_left_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [x2]
        uaddlv          h0,      v0.8b
        rshrn           v0.8b,   v0.8h,   #3
        dup             v0.16b,  v0.b[0]
        br              x3
L(ipred_dc_left_w8):
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        b.gt            L(ipred_dc_left_w8)
        ret

L(ipred_dc_left_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b}, [x2]
        uaddlv          h0,      v0.16b
        rshrn           v0.8b,   v0.8h,   #4
        dup             v0.16b,  v0.b[0]
        br              x3
L(ipred_dc_left_w16):
        AARCH64_VALID_JUMP_TARGET
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            L(ipred_dc_left_w16)
        ret

L(ipred_dc_left_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b}, [x2]
        uaddlv          h0,      v0.16b
        uaddlv          h1,      v1.16b
        add             v0.4h,   v0.4h,   v1.4h
        rshrn           v0.8b,   v0.8h,   #5
        dup             v0.16b,  v0.b[0]
        br              x3
L(ipred_dc_left_w32):
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
1:
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        b.gt            1b
        ret

L(ipred_dc_left_h64):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x2]
        uaddlv          h0,      v0.16b
        uaddlv          h1,      v1.16b
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        add             v0.4h,   v0.4h,   v1.4h
        add             v2.4h,   v2.4h,   v3.4h
        add             v0.4h,   v0.4h,   v2.4h
        rshrn           v0.8b,   v0.8h,   #6
        dup             v0.16b,  v0.b[0]
        br              x3
L(ipred_dc_left_w64):
        AARCH64_VALID_JUMP_TARGET
        mov             v1.16b,  v0.16b
        mov             v2.16b,  v0.16b
        mov             v3.16b,  v0.16b
1:
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
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

// void ipred_dc_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                         const pixel *const topleft,
//                         const int width, const int height, const int a,
//                         const int max_width, const int max_height);
function ipred_dc_8bpc_neon, export=1
        sub             x2,  x2,  w4, uxtw
        add             w7,  w3,  w4             // width + height
        clz             w3,  w3
        clz             w6,  w4
        dup             v16.8h, w7               // width + height
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
        ushr            v16.8h,  v16.8h,  #1     // (width + height) >> 1
        dup             v17.8h,  w7              // -ctz(width + height)
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5

L(ipred_dc_h4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.s}[0],  [x2], #4
        ins             v0.s[1], wzr
        uaddlv          h0,      v0.8b
        add             x2,  x2,  #1
        br              x3
L(ipred_dc_w4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.s}[0],  [x2]
        ins             v1.s[1], wzr
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h1,      v1.8b
        cmp             w4,  #4
        add             v0.4h,   v0.4h,   v1.4h
        ushl            v0.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 8/16
        mov             w16, #(0x3334/2)
        movk            w16, #(0x5556/2), lsl #16
        add             w17, w4,  w4  // w17 = 2*h = 16 or 32
        lsr             w16, w16, w17
        dup             v16.4h,  w16
        sqdmulh         v0.4h,   v0.4h,   v16.4h
1:
        dup             v0.8b,   v0.b[0]
2:
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.s}[0],  [x0], x1
        st1             {v0.s}[0],  [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [x2], #8
        uaddlv          h0,      v0.8b
        add             x2,  x2,  #1
        br              x3
L(ipred_dc_w8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.8b},  [x2]
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h1,      v1.8b
        cmp             w4,  #8
        add             v0.4h,   v0.4h,   v1.4h
        ushl            v0.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 4/16/32
        cmp             w4,  #32
        mov             w16, #(0x3334/2)
        mov             w17, #(0x5556/2)
        csel            w16, w16, w17, eq
        dup             v16.4h,  w16
        sqdmulh         v0.4h,   v0.4h,   v16.4h
1:
        dup             v0.8b,   v0.b[0]
2:
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.8b},  [x0], x1
        st1             {v0.8b},  [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b}, [x2], #16
        uaddlv          h0,      v0.16b
        add             x2,  x2,  #1
        br              x3
L(ipred_dc_w16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.16b}, [x2]
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h1,      v1.16b
        cmp             w4,  #16
        add             v0.4h,   v0.4h,   v1.4h
        ushl            v0.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 4/8/32/64
        tst             w4,  #(32+16+8) // 16 added to make a consecutive bitmask
        mov             w16, #(0x3334/2)
        mov             w17, #(0x5556/2)
        csel            w16, w16, w17, eq
        dup             v16.4h,  w16
        sqdmulh         v0.4h,   v0.4h,   v16.4h
1:
        dup             v0.16b,  v0.b[0]
2:
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b}, [x0], x1
        st1             {v0.16b}, [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b}, [x2], #32
        uaddlv          h0,      v0.16b
        uaddlv          h1,      v1.16b
        add             x2,  x2,  #1
        add             v0.4h,   v0.4h,   v1.4h
        br              x3
L(ipred_dc_w32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.16b, v2.16b}, [x2]
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h1,      v1.16b
        uaddlv          h2,      v2.16b
        cmp             w4,  #32
        add             v0.4h,   v0.4h,   v1.4h
        add             v0.4h,   v0.4h,   v2.4h
        ushl            v4.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 8/16/64
        cmp             w4,  #8
        mov             w16, #(0x3334/2)
        mov             w17, #(0x5556/2)
        csel            w16, w16, w17, eq
        dup             v16.4h,  w16
        sqdmulh         v4.4h,   v4.4h,   v16.4h
1:
        dup             v0.16b,  v4.b[0]
        dup             v1.16b,  v4.b[0]
2:
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b}, [x0], x1
        st1             {v0.16b, v1.16b}, [x6], x1
        b.gt            2b
        ret

L(ipred_dc_h64):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x2], #64
        uaddlv          h0,      v0.16b
        uaddlv          h1,      v1.16b
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        add             v0.4h,   v0.4h,   v1.4h
        add             v2.4h,   v2.4h,   v3.4h
        add             x2,  x2,  #1
        add             v0.4h,   v0.4h,   v2.4h
        br              x3
L(ipred_dc_w64):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.16b, v2.16b, v3.16b, v4.16b}, [x2]
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h1,      v1.16b
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        uaddlv          h4,      v4.16b
        add             v1.4h,   v1.4h,   v2.4h
        add             v3.4h,   v3.4h,   v4.4h
        cmp             w4,  #64
        add             v0.4h,   v0.4h,   v1.4h
        add             v0.4h,   v0.4h,   v3.4h
        ushl            v4.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 16/32
        mov             w16, #(0x5556/2)
        movk            w16, #(0x3334/2), lsl #16
        lsr             w16, w16, w4
        dup             v16.4h,  w16
        sqdmulh         v4.4h,   v4.4h,   v16.4h
1:
        dup             v0.16b,  v4.b[0]
        dup             v1.16b,  v4.b[0]
        dup             v2.16b,  v4.b[0]
        dup             v3.16b,  v4.b[0]
2:
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        st1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
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

// void ipred_paeth_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                            const pixel *const topleft,
//                            const int width, const int height, const int a,
//                            const int max_width, const int max_height);
function ipred_paeth_8bpc_neon, export=1
        clz             w9,  w3
        adr             x5,  L(ipred_paeth_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v4.16b},  [x2]
        add             x8,  x2,  #1
        sub             x2,  x2,  #4
        sub             x5,  x5,  w9, uxtw
        mov             x7,  #-4
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v5.4s},  [x8]
        usubl           v6.8h,   v5.8b,   v4.8b   // top - topleft
4:
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7
        zip1            v0.2s,   v0.2s,   v1.2s
        zip1            v2.2s,   v2.2s,   v3.2s
        uaddw           v16.8h,  v6.8h,   v0.8b
        uaddw           v17.8h,  v6.8h,   v2.8b
        sqxtun          v16.8b,  v16.8h           // base
        sqxtun2         v16.16b, v17.8h
        zip1            v0.2d,   v0.2d,   v2.2d
        uabd            v20.16b, v5.16b,  v16.16b // tdiff
        uabd            v22.16b, v4.16b,  v16.16b // tldiff
        uabd            v16.16b, v0.16b,  v16.16b // ldiff
        umin            v18.16b, v20.16b, v22.16b // min(tdiff, tldiff)
        cmhs            v20.16b, v22.16b, v20.16b // tldiff >= tdiff
        cmhs            v16.16b, v18.16b, v16.16b // min(tdiff, tldiff) >= ldiff
        bsl             v20.16b, v5.16b,  v4.16b  // tdiff <= tldiff ? top : topleft
        bit             v20.16b, v0.16b,  v16.16b // ldiff <= min ? left : ...
        st1             {v20.s}[3], [x0], x1
        st1             {v20.s}[2], [x6], x1
        subs            w4,  w4,  #4
        st1             {v20.s}[1], [x0], x1
        st1             {v20.s}[0], [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v5.2d},  [x8]
        usubl           v6.8h,   v5.8b,   v4.8b   // top - topleft
8:
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7
        uaddw           v16.8h,  v6.8h,   v0.8b
        uaddw           v17.8h,  v6.8h,   v1.8b
        uaddw           v18.8h,  v6.8h,   v2.8b
        uaddw           v19.8h,  v6.8h,   v3.8b
        sqxtun          v16.8b,  v16.8h           // base
        sqxtun2         v16.16b, v17.8h
        sqxtun          v18.8b,  v18.8h
        sqxtun2         v18.16b, v19.8h
        zip1            v2.2d,   v2.2d,   v3.2d
        zip1            v0.2d,   v0.2d,   v1.2d
        uabd            v21.16b, v5.16b,  v18.16b // tdiff
        uabd            v20.16b, v5.16b,  v16.16b
        uabd            v23.16b, v4.16b,  v18.16b // tldiff
        uabd            v22.16b, v4.16b,  v16.16b
        uabd            v17.16b, v2.16b,  v18.16b // ldiff
        uabd            v16.16b, v0.16b,  v16.16b
        umin            v19.16b, v21.16b, v23.16b // min(tdiff, tldiff)
        umin            v18.16b, v20.16b, v22.16b
        cmhs            v21.16b, v23.16b, v21.16b // tldiff >= tdiff
        cmhs            v20.16b, v22.16b, v20.16b
        cmhs            v17.16b, v19.16b, v17.16b // min(tdiff, tldiff) >= ldiff
        cmhs            v16.16b, v18.16b, v16.16b
        bsl             v21.16b, v5.16b,  v4.16b  // tdiff <= tldiff ? top : topleft
        bsl             v20.16b, v5.16b,  v4.16b
        bit             v21.16b, v2.16b,  v17.16b // ldiff <= min ? left : ...
        bit             v20.16b, v0.16b,  v16.16b
        st1             {v21.d}[1], [x0], x1
        st1             {v21.d}[0], [x6], x1
        subs            w4,  w4,  #4
        st1             {v20.d}[1], [x0], x1
        st1             {v20.d}[0], [x6], x1
        b.gt            8b
        ret
160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v5.16b},  [x8], #16
        mov             w9,  w3
        // Set up pointers for four rows in parallel; x0, x6, x5, x10
        add             x5,  x0,  x1
        add             x10, x6,  x1
        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3, uxtw
1:
        ld4r            {v0.16b, v1.16b, v2.16b, v3.16b},  [x2], x7
2:
        usubl           v6.8h,   v5.8b,   v4.8b   // top - topleft
        usubl2          v7.8h,   v5.16b,  v4.16b
        uaddw           v24.8h,  v6.8h,   v0.8b
        uaddw           v25.8h,  v7.8h,   v0.8b
        uaddw           v26.8h,  v6.8h,   v1.8b
        uaddw           v27.8h,  v7.8h,   v1.8b
        uaddw           v28.8h,  v6.8h,   v2.8b
        uaddw           v29.8h,  v7.8h,   v2.8b
        uaddw           v30.8h,  v6.8h,   v3.8b
        uaddw           v31.8h,  v7.8h,   v3.8b
        sqxtun          v17.8b,  v26.8h           // base
        sqxtun2         v17.16b, v27.8h
        sqxtun          v16.8b,  v24.8h
        sqxtun2         v16.16b, v25.8h
        sqxtun          v19.8b,  v30.8h
        sqxtun2         v19.16b, v31.8h
        sqxtun          v18.8b,  v28.8h
        sqxtun2         v18.16b, v29.8h
        uabd            v23.16b, v5.16b,  v19.16b // tdiff
        uabd            v22.16b, v5.16b,  v18.16b
        uabd            v21.16b, v5.16b,  v17.16b
        uabd            v20.16b, v5.16b,  v16.16b
        uabd            v27.16b, v4.16b,  v19.16b // tldiff
        uabd            v26.16b, v4.16b,  v18.16b
        uabd            v25.16b, v4.16b,  v17.16b
        uabd            v24.16b, v4.16b,  v16.16b
        uabd            v19.16b, v3.16b,  v19.16b // ldiff
        uabd            v18.16b, v2.16b,  v18.16b
        uabd            v17.16b, v1.16b,  v17.16b
        uabd            v16.16b, v0.16b,  v16.16b
        umin            v31.16b, v23.16b, v27.16b // min(tdiff, tldiff)
        umin            v30.16b, v22.16b, v26.16b
        umin            v29.16b, v21.16b, v25.16b
        umin            v28.16b, v20.16b, v24.16b
        cmhs            v23.16b, v27.16b, v23.16b // tldiff >= tdiff
        cmhs            v22.16b, v26.16b, v22.16b
        cmhs            v21.16b, v25.16b, v21.16b
        cmhs            v20.16b, v24.16b, v20.16b
        cmhs            v19.16b, v31.16b, v19.16b // min(tdiff, tldiff) >= ldiff
        cmhs            v18.16b, v30.16b, v18.16b
        cmhs            v17.16b, v29.16b, v17.16b
        cmhs            v16.16b, v28.16b, v16.16b
        bsl             v23.16b, v5.16b,  v4.16b  // tdiff <= tldiff ? top : topleft
        bsl             v22.16b, v5.16b,  v4.16b
        bsl             v21.16b, v5.16b,  v4.16b
        bsl             v20.16b, v5.16b,  v4.16b
        bit             v23.16b, v3.16b,  v19.16b // ldiff <= min ? left : ...
        bit             v22.16b, v2.16b,  v18.16b
        bit             v21.16b, v1.16b,  v17.16b
        bit             v20.16b, v0.16b,  v16.16b
        subs            w3,  w3,  #16
        st1             {v23.16b}, [x0],  #16
        st1             {v22.16b}, [x6],  #16
        st1             {v21.16b}, [x5],  #16
        st1             {v20.16b}, [x10], #16
        b.le            8f
        ld1             {v5.16b},  [x8], #16
        b               2b
8:
        subs            w4,  w4,  #4
        b.le            9f
        // End of horizontal loop, move pointers to next four rows
        sub             x8,  x8,  w9, uxtw
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        // Load the top row as early as possible
        ld1             {v5.16b},  [x8], #16
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

// void ipred_smooth_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                             const pixel *const topleft,
//                             const int width, const int height, const int a,
//                             const int max_width, const int max_height);
function ipred_smooth_8bpc_neon, export=1
        movrel          x10, X(sm_weights)
        add             x11, x10, w4, uxtw
        add             x10, x10, w3, uxtw
        clz             w9,  w3
        adr             x5,  L(ipred_smooth_tbl)
        sub             x12, x2,  w4, uxtw
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v4.16b},  [x12] // bottom
        add             x8,  x2,  #1
        sub             x5,  x5,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v6.2s}, [x8]             // top
        ld1r            {v7.2s}, [x10]            // weights_hor
        sub             x2,  x2,  #4
        mov             x7,  #-4
        dup             v5.16b,  v6.b[3]          // right
        usubl           v6.8h,   v6.8b,   v4.8b   // top-bottom
        uxtl            v7.8h,   v7.8b            // weights_hor
4:
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7 // left
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x11], #4 // weights_ver
        shll            v20.8h,  v5.8b,   #8      // right*256
        shll            v21.8h,  v5.8b,   #8
        zip1            v1.2s,   v1.2s,   v0.2s   // left, flipped
        zip1            v0.2s,   v3.2s,   v2.2s
        zip1            v16.2s,  v16.2s,  v17.2s  // weights_ver
        zip1            v18.2s,  v18.2s,  v19.2s
        shll            v22.8h,  v4.8b,   #8      // bottom*256
        shll            v23.8h,  v4.8b,   #8
        usubl           v0.8h,   v0.8b,   v5.8b   // left-right
        usubl           v1.8h,   v1.8b,   v5.8b
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v18.8h,  v18.8b
        mla             v20.8h,  v0.8h,   v7.8h   // right*256  + (left-right)*weights_hor
        mla             v21.8h,  v1.8h,   v7.8h
        mla             v22.8h,  v6.8h,   v16.8h  // bottom*256 + (top-bottom)*weights_ver
        mla             v23.8h,  v6.8h,   v18.8h
        uhadd           v20.8h,  v20.8h,  v22.8h
        uhadd           v21.8h,  v21.8h,  v23.8h
        rshrn           v20.8b,  v20.8h,  #8
        rshrn           v21.8b,  v21.8h,  #8
        st1             {v20.s}[0], [x0], x1
        st1             {v20.s}[1], [x6], x1
        subs            w4,  w4,  #4
        st1             {v21.s}[0], [x0], x1
        st1             {v21.s}[1], [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v6.8b}, [x8]             // top
        ld1             {v7.8b}, [x10]            // weights_hor
        sub             x2,  x2,  #4
        mov             x7,  #-4
        dup             v5.16b,  v6.b[7]          // right
        usubl           v6.8h,   v6.8b,   v4.8b   // top-bottom
        uxtl            v7.8h,   v7.8b            // weights_hor
8:
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7 // left
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x11], #4 // weights_ver
        shll            v20.8h,  v5.8b,   #8      // right*256
        shll            v21.8h,  v5.8b,   #8
        shll            v22.8h,  v5.8b,   #8
        shll            v23.8h,  v5.8b,   #8
        usubl           v0.8h,   v0.8b,   v5.8b   // left-right
        usubl           v1.8h,   v1.8b,   v5.8b
        usubl           v2.8h,   v2.8b,   v5.8b
        usubl           v3.8h,   v3.8b,   v5.8b
        shll            v24.8h,  v4.8b,   #8      // bottom*256
        shll            v25.8h,  v4.8b,   #8
        shll            v26.8h,  v4.8b,   #8
        shll            v27.8h,  v4.8b,   #8
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v17.8h,  v17.8b
        uxtl            v18.8h,  v18.8b
        uxtl            v19.8h,  v19.8b
        mla             v20.8h,  v3.8h,   v7.8h   // right*256  + (left-right)*weights_hor
        mla             v21.8h,  v2.8h,   v7.8h   // (left flipped)
        mla             v22.8h,  v1.8h,   v7.8h
        mla             v23.8h,  v0.8h,   v7.8h
        mla             v24.8h,  v6.8h,   v16.8h  // bottom*256 + (top-bottom)*weights_ver
        mla             v25.8h,  v6.8h,   v17.8h
        mla             v26.8h,  v6.8h,   v18.8h
        mla             v27.8h,  v6.8h,   v19.8h
        uhadd           v20.8h,  v20.8h,  v24.8h
        uhadd           v21.8h,  v21.8h,  v25.8h
        uhadd           v22.8h,  v22.8h,  v26.8h
        uhadd           v23.8h,  v23.8h,  v27.8h
        rshrn           v20.8b,  v20.8h,  #8
        rshrn           v21.8b,  v21.8h,  #8
        rshrn           v22.8b,  v22.8h,  #8
        rshrn           v23.8b,  v23.8h,  #8
        st1             {v20.8b}, [x0], x1
        st1             {v21.8b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v22.8b}, [x0], x1
        st1             {v23.8b}, [x6], x1
        b.gt            8b
        ret
160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        add             x12, x2,  w3, uxtw
        sub             x2,  x2,  #2
        mov             x7,  #-2
        ld1r            {v5.16b}, [x12]           // right
        sub             x1,  x1,  w3, uxtw
        mov             w9,  w3

1:
        ld2r            {v0.8b, v1.8b},   [x2],  x7 // left
        ld2r            {v16.8b, v17.8b}, [x11], #2 // weights_ver
        usubl           v0.8h,   v0.8b,   v5.8b   // left-right
        usubl           v1.8h,   v1.8b,   v5.8b
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v17.8h,  v17.8b
2:
        ld1             {v7.16b}, [x10],  #16     // weights_hor
        ld1             {v3.16b}, [x8],   #16     // top
        shll            v20.8h,  v5.8b,   #8      // right*256
        shll            v21.8h,  v5.8b,   #8
        shll            v22.8h,  v5.8b,   #8
        shll            v23.8h,  v5.8b,   #8
        uxtl            v6.8h,   v7.8b            // weights_hor
        uxtl2           v7.8h,   v7.16b
        usubl           v2.8h,   v3.8b,   v4.8b   // top-bottom
        usubl2          v3.8h,   v3.16b,  v4.16b
        mla             v20.8h,  v1.8h,   v6.8h   // right*256  + (left-right)*weights_hor
        mla             v21.8h,  v1.8h,   v7.8h   // (left flipped)
        mla             v22.8h,  v0.8h,   v6.8h
        mla             v23.8h,  v0.8h,   v7.8h
        shll            v24.8h,  v4.8b,   #8      // bottom*256
        shll            v25.8h,  v4.8b,   #8
        shll            v26.8h,  v4.8b,   #8
        shll            v27.8h,  v4.8b,   #8
        mla             v24.8h,  v2.8h,   v16.8h  // bottom*256 + (top-bottom)*weights_ver
        mla             v25.8h,  v3.8h,   v16.8h
        mla             v26.8h,  v2.8h,   v17.8h
        mla             v27.8h,  v3.8h,   v17.8h
        uhadd           v20.8h,  v20.8h,  v24.8h
        uhadd           v21.8h,  v21.8h,  v25.8h
        uhadd           v22.8h,  v22.8h,  v26.8h
        uhadd           v23.8h,  v23.8h,  v27.8h
        rshrn           v20.8b,  v20.8h,  #8
        rshrn2          v20.16b, v21.8h,  #8
        rshrn           v22.8b,  v22.8h,  #8
        rshrn2          v22.16b, v23.8h,  #8
        subs            w3,  w3,  #16
        st1             {v20.16b}, [x0],  #16
        st1             {v22.16b}, [x6],  #16
        b.gt            2b
        subs            w4,  w4,  #2
        b.le            9f
        sub             x8,  x8,  w9, uxtw
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

// void ipred_smooth_v_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height, const int a,
//                               const int max_width, const int max_height);
function ipred_smooth_v_8bpc_neon, export=1
        movrel          x7,  X(sm_weights)
        add             x7,  x7,  w4, uxtw
        clz             w9,  w3
        adr             x5,  L(ipred_smooth_v_tbl)
        sub             x8,  x2,  w4, uxtw
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v4.16b},  [x8] // bottom
        add             x2,  x2,  #1
        sub             x5,  x5,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v6.2s}, [x2]             // top
        usubl           v6.8h,   v6.8b,   v4.8b   // top-bottom
4:
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x7], #4 // weights_ver
        shll            v22.8h,  v4.8b,   #8      // bottom*256
        shll            v23.8h,  v4.8b,   #8
        zip1            v16.2s,  v16.2s,  v17.2s  // weights_ver
        zip1            v18.2s,  v18.2s,  v19.2s
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v18.8h,  v18.8b
        mla             v22.8h,  v6.8h,   v16.8h  // bottom*256 + (top-bottom)*weights_ver
        mla             v23.8h,  v6.8h,   v18.8h
        rshrn           v22.8b,  v22.8h,  #8
        rshrn           v23.8b,  v23.8h,  #8
        st1             {v22.s}[0], [x0], x1
        st1             {v22.s}[1], [x6], x1
        subs            w4,  w4,  #4
        st1             {v23.s}[0], [x0], x1
        st1             {v23.s}[1], [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v6.8b}, [x2]             // top
        usubl           v6.8h,   v6.8b,   v4.8b   // top-bottom
8:
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b},  [x7], #4 // weights_ver
        shll            v24.8h,  v4.8b,   #8      // bottom*256
        shll            v25.8h,  v4.8b,   #8
        shll            v26.8h,  v4.8b,   #8
        shll            v27.8h,  v4.8b,   #8
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v17.8h,  v17.8b
        uxtl            v18.8h,  v18.8b
        uxtl            v19.8h,  v19.8b
        mla             v24.8h,  v6.8h,   v16.8h  // bottom*256 + (top-bottom)*weights_ver
        mla             v25.8h,  v6.8h,   v17.8h
        mla             v26.8h,  v6.8h,   v18.8h
        mla             v27.8h,  v6.8h,   v19.8h
        rshrn           v24.8b,  v24.8h,  #8
        rshrn           v25.8b,  v25.8h,  #8
        rshrn           v26.8b,  v26.8h,  #8
        rshrn           v27.8b,  v27.8h,  #8
        st1             {v24.8b}, [x0], x1
        st1             {v25.8b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v26.8b}, [x0], x1
        st1             {v27.8b}, [x6], x1
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
        sub             x1,  x1,  w3, uxtw
        mov             w9,  w3

1:
        ld4r            {v16.8b, v17.8b, v18.8b, v19.8b}, [x7], #4 // weights_ver
        uxtl            v16.8h,  v16.8b           // weights_ver
        uxtl            v17.8h,  v17.8b
        uxtl            v18.8h,  v18.8b
        uxtl            v19.8h,  v19.8b
2:
        ld1             {v3.16b}, [x2],   #16     // top
        shll            v20.8h,  v4.8b,   #8      // bottom*256
        shll            v21.8h,  v4.8b,   #8
        shll            v22.8h,  v4.8b,   #8
        shll            v23.8h,  v4.8b,   #8
        shll            v24.8h,  v4.8b,   #8
        shll            v25.8h,  v4.8b,   #8
        shll            v26.8h,  v4.8b,   #8
        shll            v27.8h,  v4.8b,   #8
        usubl           v2.8h,   v3.8b,   v4.8b   // top-bottom
        usubl2          v3.8h,   v3.16b,  v4.16b
        mla             v20.8h,  v2.8h,   v16.8h  // bottom*256 + (top-bottom)*weights_ver
        mla             v21.8h,  v3.8h,   v16.8h
        mla             v22.8h,  v2.8h,   v17.8h
        mla             v23.8h,  v3.8h,   v17.8h
        mla             v24.8h,  v2.8h,   v18.8h
        mla             v25.8h,  v3.8h,   v18.8h
        mla             v26.8h,  v2.8h,   v19.8h
        mla             v27.8h,  v3.8h,   v19.8h
        rshrn           v20.8b,  v20.8h,  #8
        rshrn2          v20.16b, v21.8h,  #8
        rshrn           v22.8b,  v22.8h,  #8
        rshrn2          v22.16b, v23.8h,  #8
        rshrn           v24.8b,  v24.8h,  #8
        rshrn2          v24.16b, v25.8h,  #8
        rshrn           v26.8b,  v26.8h,  #8
        rshrn2          v26.16b, v27.8h,  #8
        subs            w3,  w3,  #16
        st1             {v20.16b}, [x0],  #16
        st1             {v22.16b}, [x6],  #16
        st1             {v24.16b}, [x5],  #16
        st1             {v26.16b}, [x8],  #16
        b.gt            2b
        subs            w4,  w4,  #4
        b.le            9f
        sub             x2,  x2,  w9, uxtw
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

// void ipred_smooth_h_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height, const int a,
//                               const int max_width, const int max_height);
function ipred_smooth_h_8bpc_neon, export=1
        movrel          x8,  X(sm_weights)
        add             x8,  x8,  w3, uxtw
        clz             w9,  w3
        adr             x5,  L(ipred_smooth_h_tbl)
        add             x12, x2,  w3, uxtw
        sub             w9,  w9,  #25
        ldrh            w9,  [x5, w9, uxtw #1]
        ld1r            {v5.16b},  [x12] // right
        sub             x5,  x5,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v7.2s}, [x8]             // weights_hor
        sub             x2,  x2,  #4
        mov             x7,  #-4
        uxtl            v7.8h,   v7.8b            // weights_hor
4:
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7 // left
        shll            v20.8h,  v5.8b,   #8      // right*256
        shll            v21.8h,  v5.8b,   #8
        zip1            v1.2s,   v1.2s,   v0.2s   // left, flipped
        zip1            v0.2s,   v3.2s,   v2.2s
        usubl           v0.8h,   v0.8b,   v5.8b   // left-right
        usubl           v1.8h,   v1.8b,   v5.8b
        mla             v20.8h,  v0.8h,   v7.8h   // right*256  + (left-right)*weights_hor
        mla             v21.8h,  v1.8h,   v7.8h
        rshrn           v20.8b,  v20.8h,  #8
        rshrn           v21.8b,  v21.8h,  #8
        st1             {v20.s}[0], [x0], x1
        st1             {v20.s}[1], [x6], x1
        subs            w4,  w4,  #4
        st1             {v21.s}[0], [x0], x1
        st1             {v21.s}[1], [x6], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v7.8b}, [x8]             // weights_hor
        sub             x2,  x2,  #4
        mov             x7,  #-4
        uxtl            v7.8h,   v7.8b            // weights_hor
8:
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},  [x2], x7 // left
        shll            v20.8h,  v5.8b,   #8      // right*256
        shll            v21.8h,  v5.8b,   #8
        shll            v22.8h,  v5.8b,   #8
        shll            v23.8h,  v5.8b,   #8
        usubl           v3.8h,   v3.8b,   v5.8b   // left-right
        usubl           v2.8h,   v2.8b,   v5.8b
        usubl           v1.8h,   v1.8b,   v5.8b
        usubl           v0.8h,   v0.8b,   v5.8b
        mla             v20.8h,  v3.8h,   v7.8h   // right*256  + (left-right)*weights_hor
        mla             v21.8h,  v2.8h,   v7.8h   // (left flipped)
        mla             v22.8h,  v1.8h,   v7.8h
        mla             v23.8h,  v0.8h,   v7.8h
        rshrn           v20.8b,  v20.8h,  #8
        rshrn           v21.8b,  v21.8h,  #8
        rshrn           v22.8b,  v22.8h,  #8
        rshrn           v23.8b,  v23.8h,  #8
        st1             {v20.8b}, [x0], x1
        st1             {v21.8b}, [x6], x1
        subs            w4,  w4,  #4
        st1             {v22.8b}, [x0], x1
        st1             {v23.8b}, [x6], x1
        b.gt            8b
        ret
160:
320:
640:
        AARCH64_VALID_JUMP_TARGET
        sub             x2,  x2,  #4
        mov             x7,  #-4
        // Set up pointers for four rows in parallel; x0, x6, x5, x10
        add             x5,  x0,  x1
        add             x10, x6,  x1
        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3, uxtw
        mov             w9,  w3

1:
        ld4r            {v0.8b, v1.8b, v2.8b, v3.8b},   [x2],  x7 // left
        usubl           v0.8h,   v0.8b,   v5.8b   // left-right
        usubl           v1.8h,   v1.8b,   v5.8b
        usubl           v2.8h,   v2.8b,   v5.8b
        usubl           v3.8h,   v3.8b,   v5.8b
2:
        ld1             {v7.16b}, [x8],   #16     // weights_hor
        shll            v20.8h,  v5.8b,   #8      // right*256
        shll            v21.8h,  v5.8b,   #8
        shll            v22.8h,  v5.8b,   #8
        shll            v23.8h,  v5.8b,   #8
        shll            v24.8h,  v5.8b,   #8
        shll            v25.8h,  v5.8b,   #8
        shll            v26.8h,  v5.8b,   #8
        shll            v27.8h,  v5.8b,   #8
        uxtl            v6.8h,   v7.8b            // weights_hor
        uxtl2           v7.8h,   v7.16b
        mla             v20.8h,  v3.8h,   v6.8h   // right*256  + (left-right)*weights_hor
        mla             v21.8h,  v3.8h,   v7.8h   // (left flipped)
        mla             v22.8h,  v2.8h,   v6.8h
        mla             v23.8h,  v2.8h,   v7.8h
        mla             v24.8h,  v1.8h,   v6.8h
        mla             v25.8h,  v1.8h,   v7.8h
        mla             v26.8h,  v0.8h,   v6.8h
        mla             v27.8h,  v0.8h,   v7.8h
        rshrn           v20.8b,  v20.8h,  #8
        rshrn2          v20.16b, v21.8h,  #8
        rshrn           v22.8b,  v22.8h,  #8
        rshrn2          v22.16b, v23.8h,  #8
        rshrn           v24.8b,  v24.8h,  #8
        rshrn2          v24.16b, v25.8h,  #8
        rshrn           v26.8b,  v26.8h,  #8
        rshrn2          v26.16b, v27.8h,  #8
        subs            w3,  w3,  #16
        st1             {v20.16b}, [x0],  #16
        st1             {v22.16b}, [x6],  #16
        st1             {v24.16b}, [x5],  #16
        st1             {v26.16b}, [x10], #16
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
padding_mask:
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        .byte 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
endconst

// void ipred_z1_upsample_edge_8bpc_neon(pixel *out, const int hsz,
//                                       const pixel *const in, const int end);
function ipred_z1_upsample_edge_8bpc_neon, export=1
        movrel          x4,  padding_mask
        ld1             {v0.16b},  [x2]           // in[]
        add             x5,  x2,  w3,  uxtw       // in[end]
        sub             x4,  x4,  w3,  uxtw

        ld1r            {v1.16b},  [x5]           // padding
        ld1             {v3.16b},  [x4]           // padding_mask

        movi            v31.8h,  #9

        bit             v0.16b,  v1.16b,  v3.16b  // padded in[]

        ext             v4.16b,  v0.16b,  v1.16b,  #1
        ext             v5.16b,  v0.16b,  v1.16b,  #2
        ext             v6.16b,  v0.16b,  v1.16b,  #3

        uaddl           v16.8h,  v4.8b,   v5.8b   // in[i+1] + in[i+2]
        uaddl2          v17.8h,  v4.16b,  v5.16b
        uaddl           v18.8h,  v0.8b,   v6.8b   // in[i+0] + in[i+3]
        uaddl2          v19.8h,  v0.16b,  v6.16b
        mul             v16.8h,  v16.8h,  v31.8h  // 9*(in[i+1] + in[i+2])
        mul             v17.8h,  v17.8h,  v31.8h
        sub             v16.8h,  v16.8h,  v18.8h
        sub             v17.8h,  v17.8h,  v19.8h

        sqrshrun        v16.8b,  v16.8h,  #4
        sqrshrun2       v16.16b, v17.8h,  #4

        zip1            v0.16b,  v4.16b,  v16.16b
        zip2            v1.16b,  v4.16b,  v16.16b

        st1             {v0.16b, v1.16b}, [x0]

        ret
endfunc

// void ipred_z2_upsample_edge_8bpc_neon(pixel *out, const int sz,
//                                       const pixel *const in);
function ipred_z2_upsample_edge_8bpc_neon, export=1
        // Here, sz is 4 or 8, and we produce 2*sz+1 output elements.
        movrel          x4,  padding_mask
        ld1             {v0.16b},  [x2]           // in[]
        add             x5,  x2,  w1,  uxtw       // in[sz]
        sub             x4,  x4,  w1,  uxtw

        ld1r            {v2.16b},  [x2]           // in[0] for padding
        ld1r            {v1.16b},  [x5]           // padding
        ld1             {v3.16b},  [x4]           // padding_mask

        movi            v31.8h,  #9

        bit             v0.16b,  v1.16b,  v3.16b  // padded in[]

        ext             v4.16b,  v2.16b,  v0.16b,  #15
        ext             v5.16b,  v0.16b,  v1.16b,  #1
        ext             v6.16b,  v0.16b,  v1.16b,  #2

        uaddl           v16.8h,  v0.8b,   v5.8b   // in[i+0] + in[i+1]
        uaddl           v18.8h,  v4.8b,   v6.8b   // in[i-1] + in[i+2]
        mul             v16.8h,  v16.8h,  v31.8h  // 9*(in[i+1] + in[i+2])
        sub             v16.8h,  v16.8h,  v18.8h

        sqrshrun        v16.8b,  v16.8h,  #4

        add             x5,  x0,  #16

        zip1            v2.16b,  v0.16b,  v16.16b

        st1             {v1.b}[0], [x5]
        // In case sz=8, output one single pixel in out[16].
        st1             {v2.16b}, [x0]

        ret
endfunc

const edge_filter
        .byte 0, 4, 8, 0
        .byte 0, 5, 6, 0
// Leaving out the coeffs for strength=3
//      .byte 2, 4, 4, 0
endconst

// void ipred_z1_filter_edge_8bpc_neon(pixel *out, const int sz,
//                                     const pixel *const in, const int end,
//                                     const int strength);
function ipred_z1_filter_edge_8bpc_neon, export=1
        cmp             w4, #3
        b.eq            L(fivetap)                // if (strength == 3) goto fivetap

        movrel          x5,  edge_filter, -3
        add             x5,  x5,  w4,  uxtw #2    // edge_filter + (strength - 1)*4 + 1

        ld1             {v31.h}[0], [x5]          // kernel[1-2]

        ld1             {v0.16b}, [x2], #16

        dup             v30.16b, v31.b[0]
        dup             v31.16b, v31.b[1]
1:
        // in[end], is the last valid pixel. We produce 16 pixels out by
        // using 18 pixels in - the last pixel used is [17] of the ones
        // read/buffered.
        cmp             w3,  #17
        ld1             {v1.16b}, [x2], #16
        b.lt            2f
        ext             v2.16b,  v0.16b,  v1.16b,  #1
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        umull           v4.8h,   v0.8b,   v30.8b
        umlal           v4.8h,   v2.8b,   v31.8b
        umlal           v4.8h,   v3.8b,   v30.8b
        umull2          v5.8h,   v0.16b,  v30.16b
        umlal2          v5.8h,   v2.16b,  v31.16b
        umlal2          v5.8h,   v3.16b,  v30.16b
        subs            w1,  w1,  #16
        mov             v0.16b,  v1.16b
        rshrn           v4.8b,   v4.8h,   #4
        rshrn2          v4.16b,  v5.8h,   #4
        sub             w3,  w3,  #16
        st1             {v4.16b}, [x0], #16
        b.gt            1b
        ret
2:
        // Right padding

        // x2[w3-32] is the padding pixel (x2 points 32 bytes ahead)
        movrel          x5,  padding_mask
        sub             w6,  w3,  #32
        sub             x5,  x5,  w3,  uxtw
        add             x6,  x2,  w6,  sxtw

        ld1             {v2.16b}, [x5]            // padding_mask

        ld1r            {v1.16b}, [x6]
        bit             v0.16b,  v1.16b,  v2.16b  // Pad v0-v1

        // Filter one block
        ext             v2.16b,  v0.16b,  v1.16b,  #1
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        umull           v4.8h,   v0.8b,   v30.8b
        umlal           v4.8h,   v2.8b,   v31.8b
        umlal           v4.8h,   v3.8b,   v30.8b
        umull2          v5.8h,   v0.16b,  v30.16b
        umlal2          v5.8h,   v2.16b,  v31.16b
        umlal2          v5.8h,   v3.16b,  v30.16b
        subs            w1,  w1,  #16
        rshrn           v4.8b,   v4.8h,   #4
        rshrn2          v4.16b,  v5.8h,   #4
        st1             {v4.16b}, [x0], #16
        b.le            9f
5:
        // After one block, any remaining output would only be filtering
        // padding - thus just store the padding.
        subs            w1,  w1,  #16
        st1             {v1.16b}, [x0], #16
        b.gt            5b
9:
        ret

L(fivetap):
        sub             x2,  x2,  #1              // topleft -= 1
        movi            v29.16b, #2
        ld1             {v0.16b}, [x2], #16
        movi            v30.16b, #4
        movi            v31.16b, #4
        ins             v0.b[0], v0.b[1]
1:
        // in[end+1], is the last valid pixel. We produce 16 pixels out by
        // using 20 pixels in - the last pixel used is [19] of the ones
        // read/buffered.
        cmp             w3,  #18
        ld1             {v1.16b}, [x2], #16
        b.lt            2f                        // if (end + 1 < 19)
        ext             v2.16b,  v0.16b,  v1.16b,  #1
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        ext             v4.16b,  v0.16b,  v1.16b,  #3
        ext             v5.16b,  v0.16b,  v1.16b,  #4
        umull           v6.8h,   v0.8b,   v29.8b
        umlal           v6.8h,   v2.8b,   v30.8b
        umlal           v6.8h,   v3.8b,   v31.8b
        umlal           v6.8h,   v4.8b,   v30.8b
        umlal           v6.8h,   v5.8b,   v29.8b
        umull2          v7.8h,   v0.16b,  v29.16b
        umlal2          v7.8h,   v2.16b,  v30.16b
        umlal2          v7.8h,   v3.16b,  v31.16b
        umlal2          v7.8h,   v4.16b,  v30.16b
        umlal2          v7.8h,   v5.16b,  v29.16b
        subs            w1,  w1,  #16
        mov             v0.16b,  v1.16b
        rshrn           v6.8b,   v6.8h,   #4
        rshrn2          v6.16b,  v7.8h,   #4
        sub             w3,  w3,  #16
        st1             {v6.16b}, [x0], #16
        b.gt            1b
        ret
2:
        // Right padding

        // x2[w3+1-32] is the padding pixel (x2 points 32 bytes ahead)
        movrel          x5,  padding_mask, -1
        sub             w6,  w3,  #31
        sub             x5,  x5,  w3,  uxtw
        add             x6,  x2,  w6,  sxtw

        ld1             {v2.16b, v3.16b}, [x5]    // padding_mask

        ld1r            {v28.16b}, [x6]
        bit             v0.16b,  v28.16b, v2.16b  // Pad v0-v1
        bit             v1.16b,  v28.16b, v3.16b
4:
        // Filter one block
        ext             v2.16b,  v0.16b,  v1.16b,  #1
        ext             v3.16b,  v0.16b,  v1.16b,  #2
        ext             v4.16b,  v0.16b,  v1.16b,  #3
        ext             v5.16b,  v0.16b,  v1.16b,  #4
        umull           v6.8h,   v0.8b,   v29.8b
        umlal           v6.8h,   v2.8b,   v30.8b
        umlal           v6.8h,   v3.8b,   v31.8b
        umlal           v6.8h,   v4.8b,   v30.8b
        umlal           v6.8h,   v5.8b,   v29.8b
        umull2          v7.8h,   v0.16b,  v29.16b
        umlal2          v7.8h,   v2.16b,  v30.16b
        umlal2          v7.8h,   v3.16b,  v31.16b
        umlal2          v7.8h,   v4.16b,  v30.16b
        umlal2          v7.8h,   v5.16b,  v29.16b
        subs            w1,  w1,  #16
        mov             v0.16b,  v1.16b
        mov             v1.16b,  v28.16b
        rshrn           v6.8b,   v6.8h,   #4
        rshrn2          v6.16b,  v7.8h,   #4
        sub             w3,  w3,  #16
        st1             {v6.16b}, [x0], #16
        b.le            9f
        // v0-v1[w3+1] is the last valid pixel; if (w3 + 1 > 0) we need to
        // filter properly once more - aka (w3 >= 0).
        cmp             w3,  #0
        b.ge            4b
5:
        // When w3 <= 0, all remaining pixels in v0-v1 are equal to the
        // last valid pixel - thus just output that without filtering.
        subs            w1,  w1,  #16
        st1             {v1.16b}, [x0], #16
        b.gt            5b
9:
        ret
endfunc

// void ipred_pixel_set_8bpc_neon(pixel *out, const pixel px,
//                                const int n);
function ipred_pixel_set_8bpc_neon, export=1
        dup             v0.16b,  w1
1:
        subs            w2,  w2,  #16
        st1             {v0.16b}, [x0], #16
        b.gt            1b
        ret
endfunc

// void ipred_z1_fill1_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const top,
//                               const int width, const int height,
//                               const int dx, const int max_base_x);
function ipred_z1_fill1_8bpc_neon, export=1
        clz             w9,  w3
        adr             x8,  L(ipred_z1_fill1_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x8, w9, uxtw #1]
        add             x10, x2,  w6,  uxtw       // top[max_base_x]
        sub             x8,  x8,  w9,  uxtw
        ld1r            {v31.16b}, [x10]          // padding
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
        ldr             d0,  [x2, w8, uxtw]       // top[base]
        ldr             d2,  [x2, w10, uxtw]
        dup             v4.4h,   w9               // frac
        dup             v5.4h,   w11
        ext             v1.8b,   v0.8b,   v0.8b,   #1 // top[base+1]
        ext             v3.8b,   v2.8b,   v2.8b,   #1
        usubl           v6.8h,   v1.8b,   v0.8b   // top[base+1]-top[base]
        usubl           v7.8h,   v3.8b,   v2.8b
        ushll           v16.8h,  v0.8b,   #6      // top[base]*64
        ushll           v17.8h,  v2.8b,   #6
        mla             v16.4h,  v6.4h,   v4.4h   // + top[base+1]*frac
        mla             v17.4h,  v7.4h,   v5.4h
        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v17.8b,  v17.8h,  #6
        st1             {v16.s}[0], [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.s}[0], [x0], x1
        b.gt            4b
        ret

49:
        st1             {v31.s}[0], [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.s}[0], [x0], x1
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
        ldr             q0,  [x2, w8, uxtw]       // top[base]
        ldr             q2,  [x2, w10, uxtw]
        dup             v4.8b,   w9               // frac
        dup             v5.8b,   w11
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v6.8b,   w9               // 64 - frac
        dup             v7.8b,   w11
        ext             v1.16b,  v0.16b,  v0.16b,  #1 // top[base+1]
        ext             v3.16b,  v2.16b,  v2.16b,  #1
        umull           v16.8h,  v0.8b,   v6.8b   // top[base]*(64-frac)
        umlal           v16.8h,  v1.8b,   v4.8b   // + top[base+1]*frac
        umull           v17.8h,  v2.8b,   v7.8b
        umlal           v17.8h,  v3.8b,   v5.8b
        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v17.8b,  v17.8h,  #6
        st1             {v16.8b}, [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.8b}, [x0], x1
        b.gt            8b
        ret

89:
        st1             {v31.8b}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.8b}, [x0], x1
        b.gt            89b
        ret

160:
320:
640:
        AARCH64_VALID_JUMP_TARGET

        mov             w12, w3

        add             x13, x0,  x1
        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3,  uxtw
1:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // xpos += dx
        cmp             w8,  w6                   // base >= max_base_x
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            169f
        add             x8,  x2,  w8,  uxtw
        add             x10, x2,  w10, uxtw
        dup             v4.16b,  w9               // frac
        dup             v5.16b,  w11
        ld1             {v0.16b, v1.16b}, [x8],  #32 // top[base]
        ld1             {v2.16b, v3.16b}, [x10], #32
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v6.16b,  w9               // 64 - frac
        dup             v7.16b,  w11
        add             w7,  w7,  w5              // xpos += dx
2:
        ext             v16.16b, v0.16b,  v1.16b,  #1 // top[base+1]
        ext             v17.16b, v2.16b,  v3.16b,  #1
        subs            w3,  w3,  #16
        umull           v18.8h,  v0.8b,   v6.8b   // top[base]*(64-frac)
        umlal           v18.8h,  v16.8b,  v4.8b   // + top[base+1]*frac
        umull2          v19.8h,  v0.16b,  v6.16b
        umlal2          v19.8h,  v16.16b, v4.16b
        umull           v20.8h,  v2.8b,   v7.8b
        umlal           v20.8h,  v17.8b,  v5.8b
        umull2          v21.8h,  v2.16b,  v7.16b
        umlal2          v21.8h,  v17.16b, v5.16b
        rshrn           v16.8b,  v18.8h,  #6
        rshrn2          v16.16b, v19.8h,  #6
        rshrn           v17.8b,  v20.8h,  #6
        rshrn2          v17.16b, v21.8h,  #6
        st1             {v16.16b}, [x0],  #16
        st1             {v17.16b}, [x13], #16
        b.le            3f
        mov             v0.16b,  v1.16b
        ld1             {v1.16b}, [x8],  #16 // top[base]
        mov             v2.16b,  v3.16b
        ld1             {v3.16b}, [x10], #16
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
        st1             {v31.16b}, [x0],  #16
        subs            w3,  w3,  #16
        st1             {v31.16b}, [x13], #16
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

function ipred_z1_fill2_8bpc_neon, export=1
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
        ldr             d0,  [x2, w8, uxtw]       // top[base]
        ldr             d2,  [x2, w10, uxtw]
        dup             v4.4h,   w9               // frac
        dup             v5.4h,   w11
        uzp2            v1.8b,   v0.8b,   v0.8b   // top[base+1]
        uzp1            v0.8b,   v0.8b,   v0.8b   // top[base]
        uzp2            v3.8b,   v2.8b,   v2.8b
        uzp1            v2.8b,   v2.8b,   v2.8b
        usubl           v6.8h,   v1.8b,   v0.8b   // top[base+1]-top[base]
        usubl           v7.8h,   v3.8b,   v2.8b
        ushll           v16.8h,  v0.8b,   #6      // top[base]*64
        ushll           v17.8h,  v2.8b,   #6
        mla             v16.4h,  v6.4h,   v4.4h   // + top[base+1]*frac
        mla             v17.4h,  v7.4h,   v5.4h
        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v17.8b,  v17.8h,  #6
        st1             {v16.s}[0], [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.s}[0], [x0], x1
        b.gt            4b
        ret

49:
        st1             {v31.s}[0], [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.s}[0], [x0], x1
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
        ldr             q0,  [x2, w8, uxtw]       // top[base]
        ldr             q2,  [x2, w10, uxtw]
        dup             v4.8b,   w9               // frac
        dup             v5.8b,   w11
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v6.8b,   w9               // 64 - frac
        dup             v7.8b,   w11
        uzp2            v1.16b,  v0.16b,  v0.16b  // top[base+1]
        uzp1            v0.16b,  v0.16b,  v0.16b  // top[base]
        uzp2            v3.16b,  v2.16b,  v2.16b
        uzp1            v2.16b,  v2.16b,  v2.16b
        umull           v16.8h,  v1.8b,   v4.8b   // top[base+1]*frac
        umlal           v16.8h,  v0.8b,   v6.8b   // + top[base]*(64-frac)
        umull           v17.8h,  v3.8b,   v5.8b
        umlal           v17.8h,  v2.8b,   v7.8b
        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v17.8b,  v17.8h,  #6
        st1             {v16.8b}, [x0], x1
        add             w7,  w7,  w5              // xpos += dx
        subs            w4,  w4,  #2
        st1             {v17.8b}, [x0], x1
        b.gt            8b
        ret

89:
        st1             {v31.8b}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v31.8b}, [x0], x1
        b.gt            89b
        ret
endfunc

// void ipred_reverse_8bpc_neon(pixel *dst, const pixel *const src,
//                              const int n);
function ipred_reverse_8bpc_neon, export=1
        sub             x1,  x1,  #16
        add             x3,  x0,  #8
        mov             x4,  #16
1:
        ld1             {v0.16b}, [x1]
        subs            w2,  w2,  #16
        rev64           v0.16b,  v0.16b
        sub             x1,  x1,  #16
        st1             {v0.d}[1], [x0], x4
        st1             {v0.d}[0], [x3], x4
        b.gt            1b
        ret
endfunc

const increments
        .short          0,  1,  2,  3,  4,  5,  6,  7
        .short          8,  9,  10, 11, 12, 13, 14, 15
endconst

// void ipred_z2_fill1_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const top,
//                               const pixel *const left,
//                               const int width, const int height,
//                               const int dx, const int dy);
function ipred_z2_fill1_8bpc_neon, export=1
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
        movi            v25.16b, #0x3e
        add             v30.4h,  v16.4h,  v30.4h  // -= dy

        xtn             v31.8b,  v31.8h           // {0,1,2,3}

        // Worst case height for w=4 is 16, but we need at least h+1 elements
        ld1             {v0.16b, v1.16b}, [x3]    // left[]

        movi            v26.16b, #64
        movi            v19.16b, #2

        xtn             v27.8b,  v30.8h           // (uint8_t)ypos
        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v27.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 1

        add             v30.8b,  v29.8b,  v17.8b  // base_y + 1
        add             v28.8b,  v29.8b,  v19.8b  // base_y + 2

        tbl             v16.8b, {v0.16b}, v29.8b  // left[base_y]

        trn1            v30.2s,  v30.2s,  v28.2s  // base_y + 1, base_y + 2

        sub             v28.8b,  v26.8b,  v27.8b  // 64 - frac_y

        trn1            v31.2s,  v31.2s,  v31.2s  // {0,1,2,3,0,1,2,3}

        trn1            v27.2s,  v27.2s,  v27.2s  // frac_y
        trn1            v28.2s,  v28.2s,  v28.2s  // 64 - frac_y

        movi            v29.8b,  #2
4:
        asr             w9,  w8,  #6              // base_x
        dup             v6.4h,   w8               // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-4                  // base_x <= -4
        asr             w11, w8,  #6              // base_x
        b.le            49f

        dup             v7.4h,   w8               // xpos

        ldr             d2,  [x2, w9, sxtw]       // top[base_x]
        ldr             d4,  [x2, w11, sxtw]

        trn1            v6.2d,   v6.2d,   v7.2d   // xpos

        // Cut corners here; only doing tbl over v0 here; we only
        // seem to need the last pixel, from v1, after skipping to the
        // left-only codepath below.
        tbl             v17.8b, {v0.16b}, v30.8b  // left[base_y+1], left[base_y+2]

        shrn            v20.8b,  v6.8h,   #6      // first base_x for each row
        xtn             v6.8b,   v6.8h            // (uint8_t)xpos

        ext             v3.8b,   v2.8b,   v2.8b,   #1 // top[base_x+1]
        ext             v5.8b,   v4.8b,   v4.8b,   #1

        and             v6.8b,   v6.8b,   v25.8b  // frac_x

        trn1            v16.2s,  v16.2s,  v17.2s  // left[base_y], left[base_y+1]

        trn1            v2.2s,   v2.2s,   v4.2s   // top[base_x]
        trn1            v3.2s,   v3.2s,   v5.2s   // top[base_x+1]

        sub             v7.8b,   v26.8b,  v6.8b   // 64 - frac_x

        add             v20.8b,  v20.8b,  v31.8b  // actual base_x

        umull           v16.8h,  v16.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v16.8h,  v17.8b,  v27.8b  // + left[base_y+1]*frac_y

        umull           v22.8h,  v2.8b,   v7.8b   // top[base_x]-*(64-frac_x)
        umlal           v22.8h,  v3.8b,   v6.8b   // + top[base_x+1]*frac_x

        cmge            v20.8b,  v20.8b,  #0

        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v22.8b,  v22.8h,  #6

        bit             v16.8b,  v22.8b,  v20.8b

        st1             {v16.s}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v16.s}[1], [x0], x1
        b.le            9f

        ext             v16.8b,  v17.8b,  v17.8b, #4
        add             v30.8b,  v30.8b,  v29.8b  // base_y += 2
        b               4b

49:
        tbl             v17.8b, {v0.16b, v1.16b}, v30.8b // left[base_y+1], left[base_y+2]

        trn1            v16.2s,  v16.2s,  v17.2s  // left[base_y], left[base_y+1]

        umull           v18.8h,  v16.8b,  v28.8b  // left[base_y]*(64-frac_t)
        umlal           v18.8h,  v17.8b,  v27.8b  // + left[base_y+1]*frac_y
        rshrn           v18.8b,  v18.8h,  #6

        st1             {v18.s}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v18.s}[1], [x0], x1
        b.le            9f

        ext             v16.8b,  v17.8b,  v17.8b, #4
        add             v30.8b,  v30.8b,  v29.8b  // base_y += 2
        b               49b

9:
        ret

80:
        AARCH64_VALID_JUMP_TARGET

        dup             v30.8h,  w7               // -dy
        movi            v17.8b,  #1

        mul             v16.8h,  v31.8h,  v30.8h  // {0,1,2,3,4,5,6,7}* -dy
        movi            v25.16b, #0x3e
        add             v30.8h,  v16.8h,  v30.8h  // -= dy

        xtn             v31.8b,  v31.8h           // {0,1,2,3,4,5,6,7}

        // Worst case height for w=8 is 32, but we need at least h+1 elements
        ld1             {v0.16b, v1.16b, v2.16b}, [x3]    // left[]

        movi            v26.16b, #64
        movi            v19.16b, #2

        xtn             v27.8b,  v30.8h           // (uint8_t)ypos
        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v27.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 1

        // Cut corners here; for the first row we don't expect to need to
        // read outside of v0.
        tbl             v18.8b, {v0.16b}, v29.8b  // left[base_y]

        add             v30.8b,  v29.8b,  v19.8b  // base_y + 2
        add             v29.8b,  v29.8b,  v17.8b  // base_y + 1

        sub             v28.8b,  v26.8b,  v27.8b  // 64 - frac_y

        trn1            v31.2d,  v31.2d,  v31.2d  // {0,1,2,3,4,5,6,7,0,1,2,3,4,5,6,7}

        movi            v24.8b,  #2               // 2
8:
        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-8                  // base_x <= -8
        asr             w11, w8,  #6              // base_x
        b.le            89f

        dup             v17.8h,   w8              // xpos

        ldr             q4,  [x2, w9, sxtw]       // top[base_x]
        ldr             q6,  [x2, w11, sxtw]

        // Cut corners here; only doing tbl over v0-v1 here; we only
        // seem to need the last pixel, from v2, after skipping to the
        // left-only codepath below.
        tbl             v19.8b, {v0.16b, v1.16b}, v29.8b // left[base_y+1]

        shrn            v21.8b,  v16.8h,  #6      // first base_x
        shrn2           v21.16b, v17.8h,  #6
        xtn             v16.8b,  v16.8h           // (uint8_t)xpos
        xtn2            v16.16b, v17.8h

        tbl             v20.8b, {v0.16b, v1.16b}, v30.8b // left[base_y+2]

        ext             v5.16b,  v4.16b,  v4.16b,  #1 // top[base_x+1]
        ext             v7.16b,  v6.16b,  v6.16b,  #1

        and             v16.16b, v16.16b, v25.16b // frac_x

        trn1            v4.2d,   v4.2d,   v6.2d   // top[base_x]
        trn1            v5.2d,   v5.2d,   v7.2d   // top[base_x+1]

        sub             v7.16b,  v26.16b, v16.16b // 64 - frac_x

        add             v21.16b, v21.16b, v31.16b // actual base_x

        umull           v6.8h,   v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v6.8h,   v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull           v17.8h,  v19.8b,  v28.8b
        umlal           v17.8h,  v20.8b,  v27.8b

        umull           v22.8h,  v4.8b,   v7.8b   // top[base_x]-*(64-frac_x)
        umlal           v22.8h,  v5.8b,   v16.8b  // + top[base_x+1]*frac_x
        umull2          v23.8h,  v4.16b,  v7.16b
        umlal2          v23.8h,  v5.16b,  v16.16b

        cmge            v21.16b, v21.16b, #0

        rshrn           v6.8b,   v6.8h,   #6
        rshrn2          v6.16b,  v17.8h,  #6
        rshrn           v22.8b,  v22.8h,  #6
        rshrn2          v22.16b, v23.8h,  #6

        bit             v6.16b,  v22.16b, v21.16b

        st1             {v6.d}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v6.d}[1], [x0], x1
        b.le            9f

        mov             v18.8b,  v20.8b
        add             v29.8b,  v29.8b,  v24.8b  // base_y += 2
        add             v30.8b,  v30.8b,  v24.8b  // base_y += 2
        b               8b

89:
        tbl             v19.8b, {v0.16b, v1.16b, v2.16b}, v29.8b // left[base_y+1]
        tbl             v20.8b, {v0.16b, v1.16b, v2.16b}, v30.8b // left[base_y+2]

        umull           v6.8h,   v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v6.8h,   v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull           v17.8h,  v19.8b,  v28.8b
        umlal           v17.8h,  v20.8b,  v27.8b

        rshrn           v6.8b,   v6.8h,   #6
        rshrn2          v6.16b,  v17.8h,  #6

        st1             {v6.d}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v6.d}[1], [x0], x1
        b.le            9f

        mov             v18.8b,  v20.8b
        add             v29.8b,  v29.8b,  v24.8b  // base_y += 2
        add             v30.8b,  v30.8b,  v24.8b  // base_y += 2
        b               89b

9:
        ret

160:
        AARCH64_VALID_JUMP_TARGET

        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

        add             x11, x11, #16             // increments

        dup             v18.8h,  w7               // -dy
        movi            v17.16b, #1
        add             x3,  x3,  #1              // Skip past left[0]

        ld1             {v14.8h}, [x11]           // {8,9,10,11,12,13,14,15}

        mul             v16.8h,  v31.8h,  v18.8h  // {0,1,2,3,4,5,6,7}* -dy
        mul             v19.8h,  v14.8h,  v18.8h  // {8,9,10,11,12,13,14,15}* -dy
        movi            v25.16b, #0x3e
        add             v16.8h,  v16.8h,  v18.8h  // -= dy
        add             v18.8h,  v19.8h,  v18.8h

        xtn             v31.8b,  v31.8h           // {0,1,2,3,4,5,6,7}
        xtn2            v31.16b, v14.8h           // {8,9,10,11,12,13,14,15}

        // Worst case height is 64.
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x3] // left[]
        ld1r            {v15.16b}, [x2]           // left[0] == top[0]

        movi            v26.16b, #64
        movi            v19.16b, #2

        xtn             v27.8b,  v16.8h           // (uint8_t)ypos
        xtn2            v27.16b, v18.8h
        shrn            v29.8b,  v16.8h,  #6      // ypos >> 6
        shrn2           v29.16b, v18.8h,  #6
        mov             v18.16b, v15.16b          // left[0]
        and             v27.16b, v27.16b, v25.16b // frac_y

        // Cut corners here; for the first row we don't expect to need to
        // read outside of v0.
        tbx             v18.16b, {v0.16b}, v29.16b // left[base_y]

        add             v30.16b, v29.16b, v19.16b // base_y + 2
        add             v29.16b, v29.16b, v17.16b // base_y + 1

        sub             v28.16b, v26.16b, v27.16b // 64 - frac_y

        movi            v24.16b, #2               // 2
16:
        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-16                 // base_x <= -16
        asr             w11, w8,  #6              // base_x
        b.le            169f

        dup             v17.8h,   w8              // xpos

        add             x9,  x2,  w9,  sxtw
        add             x11, x2,  w11, sxtw

        ld1             {v4.16b, v5.16b}, [x9]    // top[base_x]
        mov             v19.16b, v15.16b          // left[0]
        ld1             {v6.16b, v7.16b}, [x11]

        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]

        mov             v20.16b, v15.16b          // left[0]

        shrn            v21.8b,  v16.8h,  #6      // first base_x
        shrn            v22.8b,  v17.8h,  #6
        xtn             v16.8b,  v16.8h           // (uint8_t)xpos
        xtn             v17.8b,  v17.8h

        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b // left[base_y+2]

        trn1            v21.2d,  v21.2d,  v21.2d  // first base_x
        trn1            v22.2d,  v22.2d,  v22.2d
        trn1            v16.2d,  v16.2d,  v16.2d  // (uint8_t)xpos
        trn1            v17.2d,  v17.2d,  v17.2d

        ext             v5.16b,  v4.16b,  v5.16b,  #1 // top[base_x+1]
        ext             v7.16b,  v6.16b,  v7.16b,  #1

        and             v16.16b, v16.16b, v25.16b // frac_x
        and             v17.16b, v17.16b, v25.16b

        umull           v10.8h,  v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v10.8h,  v19.8b,  v27.8b  // + left[base_y+1]*frac_y

        sub             v8.16b,  v26.16b, v16.16b // 64 - frac_x
        sub             v9.16b,  v26.16b, v17.16b

        umull2          v11.8h,  v18.16b, v28.16b
        umlal2          v11.8h,  v19.16b, v27.16b

        add             v21.16b, v21.16b, v31.16b // actual base_x
        add             v22.16b, v22.16b, v31.16b

        umull           v12.8h,  v19.8b,  v28.8b
        umlal           v12.8h,  v20.8b,  v27.8b
        umull2          v13.8h,  v19.16b, v28.16b
        umlal2          v13.8h,  v20.16b, v27.16b

        rshrn           v10.8b,  v10.8h,  #6
        rshrn2          v10.16b, v11.8h,  #6
        rshrn           v11.8b,  v12.8h,  #6
        rshrn2          v11.16b, v13.8h,  #6

        umull           v12.8h,  v4.8b,   v8.8b   // top[base_x]-*(64-frac_x)
        umlal           v12.8h,  v5.8b,   v16.8b  // + top[base_x+1]*frac_x
        umull2          v13.8h,  v4.16b,  v8.16b
        umlal2          v13.8h,  v5.16b,  v16.16b
        umull           v14.8h,  v6.8b,   v9.8b
        umlal           v14.8h,  v7.8b,   v17.8b
        umull2          v18.8h,  v6.16b,  v9.16b
        umlal2          v18.8h,  v7.16b,  v17.16b

        cmge            v21.16b, v21.16b, #0
        cmge            v22.16b, v22.16b, #0

        rshrn           v12.8b,  v12.8h,  #6
        rshrn2          v12.16b, v13.8h,  #6
        rshrn           v13.8b,  v14.8h,  #6
        rshrn2          v13.16b, v18.8h,  #6

        bit             v10.16b, v12.16b, v21.16b
        bit             v11.16b, v13.16b, v22.16b

        st1             {v10.16b}, [x0], x1
        subs            w5,  w5,  #2
        sub             w8,  w8,  w6              // xpos -= dx
        st1             {v11.16b}, [x0], x1
        b.le            9f

        mov             v18.16b, v20.16b
        add             v29.16b, v29.16b, v24.16b // base_y += 2
        add             v30.16b, v30.16b, v24.16b // base_y += 2
        b               16b

169:
        mov             v19.16b, v15.16b
        mov             v20.16b, v15.16b
        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]
        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b // left[base_y+2]

        umull           v4.8h,   v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v4.8h,   v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull2          v5.8h,   v18.16b, v28.16b
        umlal2          v5.8h,   v19.16b, v27.16b
        umull           v6.8h,   v19.8b,  v28.8b
        umlal           v6.8h,   v20.8b,  v27.8b
        umull2          v7.8h,   v19.16b, v28.16b
        umlal2          v7.8h,   v20.16b, v27.16b

        rshrn           v4.8b,   v4.8h,   #6
        rshrn2          v4.16b,  v5.8h,   #6
        rshrn           v5.8b,   v6.8h,   #6
        rshrn2          v5.16b,  v7.8h,   #6

        st1             {v4.16b}, [x0], x1
        subs            w5,  w5,  #2
        st1             {v5.16b}, [x0], x1
        b.le            9f

        mov             v18.16b, v20.16b
        add             v29.16b, v29.16b, v24.16b // base_y += 2
        add             v30.16b, v30.16b, v24.16b // base_y += 2
        b               169b

9:
        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40
        ret

320:
640:
        AARCH64_VALID_JUMP_TARGET

        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

        add             x11, x11, #16             // increments

        dup             v25.8h,  w7               // -dy
        add             x3,  x3,  #1              // Skip past left[0]

        ld1             {v14.8h}, [x11]           // {8,9,10,11,12,13,14,15}

        add             x13, x0,  x1              // alternating row
        lsl             x1,  x1,  #1              // stride *= 2
        sub             x1,  x1,  w4,  uxtw       // stride -= width

        movi            v11.8h,  #8
        mul             v26.8h,  v31.8h,  v25.8h  // {0,1,2,3,4,5,6,7}* -dy
        add             v26.8h,  v26.8h,  v25.8h  // -= dy
        mul             v25.8h,  v25.8h,  v11.8h  // -8*dy

        xtn             v31.8b,  v31.8h           // {0,1,2,3,4,5,6,7}
        xtn2            v31.16b, v14.8h           // {8,9,10,11,12,13,14,15}

        // Worst case height is 64.
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x3] // left[]
        ld1r            {v15.16b}, [x2]           // left[0] == top[0]

        mov             w12, w4                   // orig w
        neg             w14, w4                   // -w

1:
        mov             v23.16b, v26.16b          // reset ypos

        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  w14                  // base_x <= -w
        asr             w11, w8,  #6              // base_x
        b.le            329f

        dup             v17.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx

        add             x9,  x2,  w9,  sxtw
        add             x11, x2,  w11, sxtw

        sqshrn          v21.8b,  v16.8h,  #6      // first base_x
        sqshrn          v22.8b,  v17.8h,  #6
        xtn             v16.8b,  v16.8h           // (uint8_t)xpos
        xtn             v17.8b,  v17.8h

        ld1             {v4.16b}, [x9], #16       // top[base_x]
        ld1             {v6.16b}, [x11], #16

        trn1            v21.2d,  v21.2d,  v21.2d  // first base_x
        trn1            v22.2d,  v22.2d,  v22.2d
        trn1            v16.2d,  v16.2d,  v16.2d  // (uint8_t)xpos
        trn1            v17.2d,  v17.2d,  v17.2d

        movi            v10.16b, #0x3e
        movi            v11.16b, #64

        and             v16.16b, v16.16b, v10.16b // frac_x
        and             v17.16b, v17.16b, v10.16b

        sub             v8.16b,  v11.16b, v16.16b // 64 - frac_x
        sub             v9.16b,  v11.16b, v17.16b

        add             v21.16b, v21.16b, v31.16b // actual base_x
        add             v22.16b, v22.16b, v31.16b

2:
        add             v13.8h,  v23.8h,  v25.8h  // ypos -= 8*dy
        movi            v12.16b, #64
        movi            v20.16b, #2
        movi            v10.16b, #0x3e

        smov            w10,     v22.b[0]

        xtn             v27.8b,  v23.8h           // (uint8_t)ypos
        xtn2            v27.16b, v13.8h
        shrn            v29.8b,  v23.8h,  #6      // ypos >> 6
        shrn2           v29.16b, v13.8h,  #6
        cmp             w10, #0                   // base_x (bottom left) >= 0
        and             v27.16b, v27.16b, v10.16b // frac_y

        mov             v18.16b, v15.16b          // left[0]

        b.ge            4f

        add             v23.8h,  v13.8h,  v25.8h  // ypos -= 8*dy
        movi            v13.16b, #1

        tbx             v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y]
        add             v29.16b, v29.16b, v13.16b // base_y + 1
        mov             v19.16b, v15.16b          // left[0]

        sub             v28.16b, v12.16b, v27.16b // 64 - frac_y

        ld1             {v5.16b}, [x9], #16       // top[base_x]
        ld1             {v7.16b}, [x11], #16

        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]
        add             v29.16b, v29.16b, v13.16b // base_y + 2

        mov             v20.16b, v15.16b          // left[0]
        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+2]

        umull           v10.8h,  v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v10.8h,  v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull2          v11.8h,  v18.16b, v28.16b
        umlal2          v11.8h,  v19.16b, v27.16b
        umull           v12.8h,  v19.8b,  v28.8b
        umlal           v12.8h,  v20.8b,  v27.8b
        umull2          v13.8h,  v19.16b, v28.16b
        umlal2          v13.8h,  v20.16b, v27.16b

        ext             v18.16b, v4.16b,  v5.16b,  #1 // top[base_x+1]
        ext             v19.16b, v6.16b,  v7.16b,  #1

        rshrn           v10.8b,  v10.8h,  #6
        rshrn2          v10.16b, v11.8h,  #6
        rshrn           v11.8b,  v12.8h,  #6
        rshrn2          v11.16b, v13.8h,  #6

        umull           v12.8h,  v4.8b,   v8.8b   // top[base_x]-*(64-frac_x)
        umlal           v12.8h,  v18.8b,  v16.8b  // + top[base_x+1]*frac_x
        umull2          v13.8h,  v4.16b,  v8.16b
        umlal2          v13.8h,  v18.16b, v16.16b
        umull           v14.8h,  v6.8b,   v9.8b
        umlal           v14.8h,  v19.8b,  v17.8b
        umull2          v20.8h,  v6.16b,  v9.16b
        umlal2          v20.8h,  v19.16b, v17.16b

        cmge            v18.16b, v21.16b, #0
        cmge            v19.16b, v22.16b, #0

        rshrn           v12.8b,  v12.8h,  #6
        rshrn2          v12.16b, v13.8h,  #6
        rshrn           v13.8b,  v14.8h,  #6
        rshrn2          v13.16b, v20.8h,  #6

        bit             v10.16b, v12.16b, v18.16b
        bit             v11.16b, v13.16b, v19.16b

        st1             {v10.16b}, [x0], #16
        subs            w4,  w4,  #16
        st1             {v11.16b}, [x13], #16
        b.le            3f

        movi            v10.16b, #16
        mov             v4.16b,  v5.16b
        mov             v6.16b,  v7.16b
        add             v21.16b, v21.16b, v10.16b // base_x += 16
        add             v22.16b, v22.16b, v10.16b
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
        ld1             {v5.16b}, [x9], #16       // top[base_x]
        ld1             {v7.16b}, [x11], #16

        ext             v18.16b, v4.16b,  v5.16b,  #1 // top[base_x+1]
        ext             v19.16b, v6.16b,  v7.16b,  #1

        umull           v12.8h,  v4.8b,   v8.8b   // top[base_x]-*(64-frac_x)
        umlal           v12.8h,  v18.8b,  v16.8b  // + top[base_x+1]*frac_x
        umull2          v13.8h,  v4.16b,  v8.16b
        umlal2          v13.8h,  v18.16b, v16.16b
        umull           v14.8h,  v6.8b,   v9.8b
        umlal           v14.8h,  v19.8b,  v17.8b
        umull2          v20.8h,  v6.16b,  v9.16b
        umlal2          v20.8h,  v19.16b, v17.16b

        rshrn           v12.8b,  v12.8h,  #6
        rshrn2          v12.16b, v13.8h,  #6
        rshrn           v13.8b,  v14.8h,  #6
        rshrn2          v13.16b, v20.8h,  #6

        st1             {v12.16b}, [x0], #16
        subs            w4,  w4,  #16
        st1             {v13.16b}, [x13], #16
        b.le            3b

        mov             v4.16b,  v5.16b
        mov             v6.16b,  v7.16b
        b               4b

329:    // The rest of the block only predicted from left[]
        add             x1,  x1,  w4,  uxtw       // restore stride
        mov             w12, w5                   // orig remaining h
1:
        add             v13.8h,  v23.8h,  v25.8h  // ypos -= 8*dy
        movi            v12.16b, #64
        movi            v10.16b, #0x3e

        xtn             v27.8b,  v23.8h           // (uint8_t)ypos
        xtn2            v27.16b, v13.8h
        shrn            v29.8b,  v23.8h,  #6      // ypos >> 6
        shrn2           v29.16b, v13.8h,  #6
        and             v27.16b, v27.16b, v10.16b // frac_y

        mov             v18.16b, v15.16b          // left[0]
        add             v23.8h,  v13.8h,  v25.8h  // ypos -= 8*dy
        movi            v21.16b, #1

        tbx             v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y]
        add             v29.16b, v29.16b, v21.16b // base_y + 1

        sub             v28.16b, v12.16b, v27.16b // 64 - frac_y
2:
        mov             v19.16b, v15.16b          // left[0]
        tbx             v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+1]
        add             v29.16b, v29.16b, v21.16b // base_y + 2
        mov             v20.16b, v15.16b          // left[0]
        tbx             v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b // left[base_y+2]
        add             v29.16b, v29.16b, v21.16b // next base_y

        umull           v10.8h,  v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v10.8h,  v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull2          v11.8h,  v18.16b, v28.16b
        umlal2          v11.8h,  v19.16b, v27.16b
        umull           v12.8h,  v19.8b,  v28.8b
        umlal           v12.8h,  v20.8b,  v27.8b
        umull2          v13.8h,  v19.16b, v28.16b
        umlal2          v13.8h,  v20.16b, v27.16b

        rshrn           v10.8b,  v10.8h,  #6
        rshrn2          v10.16b, v11.8h,  #6
        rshrn           v11.8b,  v12.8h,  #6
        rshrn2          v11.16b, v13.8h,  #6

        st1             {v10.16b}, [x0], x1
        subs            w5,  w5,  #2
        st1             {v11.16b}, [x13], x1
        b.le            3f
        mov             v18.16b, v20.16b
        b               2b

3:
        subs            w4,  w4,  #16
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

function ipred_z2_fill2_8bpc_neon, export=1
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
        movi            v25.16b, #0x3e
        add             v30.4h,  v16.4h,  v30.4h  // -= dy

        xtn             v31.8b,  v31.8h           // {0,1,2,3}

        // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
        // from left.
        ld1             {v0.16b}, [x3]            // left[]

        movi            v26.16b, #64
        movi            v19.16b, #2

        xtn             v27.8b,  v30.8h           // (uint8_t)ypos
        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v27.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 1

        add             v30.8b,  v29.8b,  v17.8b  // base_y + 1
        add             v28.8b,  v29.8b,  v19.8b  // base_y + 2

        tbl             v16.8b, {v0.16b}, v29.8b  // left[base_y]

        trn1            v30.2s,  v30.2s,  v28.2s  // base_y + 1, base_y + 2

        sub             v28.8b,  v26.8b,  v27.8b  // 64 - frac_y

        trn1            v31.2s,  v31.2s,  v31.2s  // {0,1,2,3,0,1,2,3}

        trn1            v27.2s,  v27.2s,  v27.2s  // frac_y
        trn1            v28.2s,  v28.2s,  v28.2s  // 64 - frac_y

        movi            v29.8b,  #2
        add             v31.8b,  v31.8b,  v31.8b  // {0,2,4,6,0,2,4,6}
4:
        asr             w9,  w8,  #6              // base_x
        dup             v6.4h,   w8               // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-8                  // base_x <= -8
        asr             w11, w8,  #6              // base_x
        b.le            49f

        dup             v7.4h,   w8               // xpos

        ldr             d2,  [x2, w9, sxtw]       // top[base_x]
        ldr             d4,  [x2, w11, sxtw]

        trn1            v6.2d,   v6.2d,   v7.2d   // xpos

        tbl             v17.8b, {v0.16b}, v30.8b  // left[base_y+1], left[base_y+2]

        shrn            v20.8b,  v6.8h,   #6      // first base_x for each row
        xtn             v6.8b,   v6.8h            // (uint8_t)xpos

        uzp2            v3.8b,   v2.8b,   v4.8b   // top[base_x+1]
        uzp1            v2.8b,   v2.8b,   v4.8b   // top[base_x]

        and             v6.8b,   v6.8b,   v25.8b  // frac_x

        trn1            v16.2s,  v16.2s,  v17.2s  // left[base_y], left[base_y+1]

        sub             v7.8b,   v26.8b,  v6.8b   // 64 - frac_x

        add             v20.8b,  v20.8b,  v31.8b  // actual base_x

        umull           v16.8h,  v16.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v16.8h,  v17.8b,  v27.8b  // + left[base_y+1]*frac_y

        umull           v22.8h,  v2.8b,   v7.8b   // top[base_x]-*(64-frac_x)
        umlal           v22.8h,  v3.8b,   v6.8b   // + top[base_x+1]*frac_x

        cmge            v20.8b,  v20.8b,  #0

        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v22.8b,  v22.8h,  #6

        bit             v16.8b,  v22.8b,  v20.8b

        st1             {v16.s}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v16.s}[1], [x0], x1
        b.le            9f

        ext             v16.8b,  v17.8b,  v17.8b, #4
        add             v30.8b,  v30.8b,  v29.8b  // base_y += 2
        b               4b

49:
        tbl             v17.8b, {v0.16b}, v30.8b  // left[base_y+1], left[base_y+2]

        trn1            v16.2s,  v16.2s,  v17.2s  // left[base_y], left[base_y+1]

        umull           v18.8h,  v16.8b,  v28.8b  // left[base_y]*(64-frac_t)
        umlal           v18.8h,  v17.8b,  v27.8b  // + left[base_y+1]*frac_y
        rshrn           v18.8b,  v18.8h,  #6

        st1             {v18.s}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v18.s}[1], [x0], x1
        b.le            9f

        ext             v16.8b,  v17.8b,  v17.8b, #4
        add             v30.8b,  v30.8b,  v29.8b  // base_y += 2
        b               49b

9:
        ret

80:
        dup             v30.8h,  w7               // -dy
        movi            v17.8b,  #1

        mul             v16.8h,  v31.8h,  v30.8h  // {0,1,2,3,4,5,6,7}* -dy
        movi            v25.16b, #0x3e
        add             v30.8h,  v16.8h,  v30.8h  // -= dy

        xtn             v31.8b,  v31.8h           // {0,1,2,3,4,5,6,7}

        // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
        // from left.
        ld1             {v0.16b}, [x3]    // left[]

        movi            v26.16b, #64
        movi            v19.16b, #2

        xtn             v27.8b,  v30.8h           // (uint8_t)ypos
        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v27.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v17.8b  // base_y = (ypos >> 6) + 1

        tbl             v18.8b, {v0.16b}, v29.8b  // left[base_y]

        add             v30.8b,  v29.8b,  v19.8b  // base_y + 2
        add             v29.8b,  v29.8b,  v17.8b  // base_y + 1

        sub             v28.8b,  v26.8b,  v27.8b  // 64 - frac_y

        trn1            v31.2d,  v31.2d,  v31.2d  // {0,1,2,3,4,5,6,7,0,1,2,3,4,5,6,7}

        movi            v24.8b,  #2               // 2
        add             v31.16b, v31.16b, v31.16b // {0,2,4,6,8,10,12,14,0,2,4,6,8,10,12,14}
8:
        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-16                 // base_x <= -16
        asr             w11, w8,  #6              // base_x
        b.le            89f

        dup             v17.8h,   w8              // xpos

        ldr             q4,  [x2, w9, sxtw]       // top[base_x]
        ldr             q6,  [x2, w11, sxtw]

        tbl             v19.8b, {v0.16b}, v29.8b  // left[base_y+1]

        shrn            v21.8b,  v16.8h,  #6      // first base_x
        shrn2           v21.16b, v17.8h,  #6
        xtn             v16.8b,  v16.8h           // (uint8_t)xpos
        xtn2            v16.16b, v17.8h

        tbl             v20.8b, {v0.16b}, v30.8b  // left[base_y+2]

        uzp2            v5.16b,  v4.16b,  v6.16b  // top[base_x+1]
        uzp1            v4.16b,  v4.16b,  v6.16b  // top[base_x]

        and             v16.16b, v16.16b, v25.16b // frac_x

        sub             v7.16b,  v26.16b, v16.16b // 64 - frac_x

        add             v21.16b, v21.16b, v31.16b // actual base_x

        umull           v6.8h,   v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v6.8h,   v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull           v17.8h,  v19.8b,  v28.8b
        umlal           v17.8h,  v20.8b,  v27.8b

        umull           v22.8h,  v4.8b,   v7.8b   // top[base_x]-*(64-frac_x)
        umlal           v22.8h,  v5.8b,   v16.8b  // + top[base_x+1]*frac_x
        umull2          v23.8h,  v4.16b,  v7.16b
        umlal2          v23.8h,  v5.16b,  v16.16b

        cmge            v21.16b, v21.16b, #0

        rshrn           v6.8b,   v6.8h,   #6
        rshrn2          v6.16b,  v17.8h,  #6
        rshrn           v22.8b,  v22.8h,  #6
        rshrn2          v22.16b, v23.8h,  #6

        bit             v6.16b,  v22.16b, v21.16b

        st1             {v6.d}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v6.d}[1], [x0], x1
        b.le            9f

        mov             v18.8b,  v20.8b
        add             v29.8b,  v29.8b,  v24.8b  // base_y += 2
        add             v30.8b,  v30.8b,  v24.8b  // base_y += 2
        b               8b

89:
        tbl             v19.8b, {v0.16b}, v29.8b  // left[base_y+1]
        tbl             v20.8b, {v0.16b}, v30.8b  // left[base_y+2]

        umull           v6.8h,   v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v6.8h,   v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull           v17.8h,  v19.8b,  v28.8b
        umlal           v17.8h,  v20.8b,  v27.8b

        rshrn           v6.8b,   v6.8h,   #6
        rshrn2          v6.16b,  v17.8h,  #6

        st1             {v6.d}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v6.d}[1], [x0], x1
        b.le            9f

        mov             v18.8b,  v20.8b
        add             v29.8b,  v29.8b,  v24.8b  // base_y += 2
        add             v30.8b,  v30.8b,  v24.8b  // base_y += 2
        b               89b

9:
        ret
endfunc

function ipred_z2_fill3_8bpc_neon, export=1
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
        movi            v25.16b, #0x3e
        add             v30.4h,  v16.4h,  v30.4h  // -= dy

        xtn             v31.8b,  v31.8h           // {0,1,2,3}

        // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
        ld1             {v0.16b, v1.16b}, [x3]    // left[]

        movi            v26.16b, #64
        movi            v19.16b, #2

        xtn             v27.8b,  v30.8h           // (uint8_t)ypos
        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v27.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v19.8b  // base_y = (ypos >> 6) + 2

        add             v30.8b,  v29.8b,  v17.8b  // base_y + 1
        add             v28.8b,  v29.8b,  v19.8b  // base_y + 2

        trn1            v31.2s,  v31.2s,  v31.2s  // {0,1,2,3,0,1,2,3}

        add             v24.8b,  v30.8b,  v19.8b  // base_y + 3

        trn1            v29.2s,  v29.2s,  v28.2s  // base_y + 0, base_y + 2
        trn1            v30.2s,  v30.2s,  v24.2s  // base_y + 1, base_y + 3

        sub             v28.8b,  v26.8b,  v27.8b  // 64 - frac_y

        trn1            v27.2s,  v27.2s,  v27.2s  // frac_y
        trn1            v28.2s,  v28.2s,  v28.2s  // 64 - frac_y

        movi            v24.8b,  #4
4:
        asr             w9,  w8,  #6              // base_x
        dup             v6.4h,   w8               // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-4                  // base_x <= -4
        asr             w11, w8,  #6              // base_x
        b.le            49f

        dup             v7.4h,   w8               // xpos

        ldr             d2,  [x2, w9, sxtw]       // top[base_x]
        ldr             d4,  [x2, w11, sxtw]

        trn1            v6.2d,   v6.2d,   v7.2d   // xpos

        tbl             v16.8b, {v0.16b, v1.16b}, v29.8b // left[base_y+0], left[base_y+2]
        tbl             v17.8b, {v0.16b, v1.16b}, v30.8b // left[base_y+1], left[base_y+3]

        shrn            v20.8b,  v6.8h,   #6      // first base_x for each row
        xtn             v6.8b,   v6.8h            // (uint8_t)xpos

        ext             v3.8b,   v2.8b,   v2.8b,   #1 // top[base_x+1]
        ext             v5.8b,   v4.8b,   v4.8b,   #1

        and             v6.8b,   v6.8b,   v25.8b  // frac_x

        trn1            v2.2s,   v2.2s,   v4.2s   // top[base_x]
        trn1            v3.2s,   v3.2s,   v5.2s   // top[base_x+1]

        sub             v7.8b,   v26.8b,  v6.8b   // 64 - frac_x

        add             v20.8b,  v20.8b,  v31.8b  // actual base_x

        umull           v16.8h,  v16.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v16.8h,  v17.8b,  v27.8b  // + left[base_y+1]*frac_y

        umull           v22.8h,  v2.8b,   v7.8b   // top[base_x]-*(64-frac_x)
        umlal           v22.8h,  v3.8b,   v6.8b   // + top[base_x+1]*frac_x

        cmge            v20.8b,  v20.8b,  #0

        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v22.8b,  v22.8h,  #6

        bit             v16.8b,  v22.8b,  v20.8b

        st1             {v16.s}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v16.s}[1], [x0], x1
        b.le            9f

        add             v29.8b,  v29.8b,  v24.8b  // base_y += 4
        add             v30.8b,  v30.8b,  v24.8b  // base_y += 4
        b               4b

49:
        tbl             v16.8b, {v0.16b, v1.16b}, v29.8b // left[base_y+0], left[base_y+2]
        tbl             v17.8b, {v0.16b, v1.16b}, v30.8b // left[base_y+1], left[base_y+3]

        umull           v18.8h,  v16.8b,  v28.8b  // left[base_y]*(64-frac_t)
        umlal           v18.8h,  v17.8b,  v27.8b  // + left[base_y+1]*frac_y
        rshrn           v18.8b,  v18.8h,  #6

        st1             {v18.s}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v18.s}[1], [x0], x1
        b.le            9f

        add             v29.8b,  v29.8b,  v24.8b  // base_y += 4
        add             v30.8b,  v30.8b,  v24.8b  // base_y += 4
        b               49b

9:
        ret

80:
        dup             v30.8h,  w7               // -dy
        movi            v17.8b,  #1

        mul             v16.8h,  v31.8h,  v30.8h  // {0,1,2,3,4,5,6,7}* -dy
        movi            v25.16b, #0x3e
        add             v30.8h,  v16.8h,  v30.8h  // -= dy

        xtn             v31.8b,  v31.8h           // {0,1,2,3,4,5,6,7}

        // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
        ld1             {v0.16b, v1.16b, v2.16b}, [x3]    // left[]

        movi            v26.16b, #64
        movi            v19.16b, #2

        xtn             v27.8b,  v30.8h           // (uint8_t)ypos
        shrn            v29.8b,  v30.8h,  #6      // ypos >> 6
        and             v27.8b,  v27.8b,  v25.8b  // frac_y

        add             v29.8b,  v29.8b,  v19.8b  // base_y = (ypos >> 6) + 2

        add             v28.8b,  v29.8b,  v17.8b  // base_y + 1
        add             v30.8b,  v29.8b,  v19.8b  // base_y + 2

        trn1            v31.2d,  v31.2d,  v31.2d  // {0,1,2,3,4,5,6,7,0,1,2,3,4,5,6,7}
        add             v24.8b,  v28.8b,  v19.8b  // base_y + 3

        trn1            v29.2d,  v29.2d,  v30.2d  // base_y + 0, base_y + 2
        trn1            v30.2d,  v28.2d,  v24.2d  // base_y + 1, base_y + 3

        sub             v28.8b,  v26.8b,  v27.8b  // 64 - frac_y

        movi            v24.16b, #4

        trn1            v27.2d,  v27.2d,  v27.2d  // frac_y
        trn1            v28.2d,  v28.2d,  v28.2d  // 64 - frac_y
8:
        asr             w9,  w8,  #6              // base_x
        dup             v16.8h,   w8              // xpos
        sub             w8,  w8,  w6              // xpos -= dx
        cmp             w9,  #-8                  // base_x <= -8
        asr             w11, w8,  #6              // base_x
        b.le            89f

        dup             v17.8h,   w8              // xpos

        ldr             q4,  [x2, w9, sxtw]       // top[base_x]
        ldr             q6,  [x2, w11, sxtw]

        tbl             v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+0], left[base_y+2]
        tbl             v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+1], left[base_y+3]

        shrn            v21.8b,  v16.8h,  #6      // first base_x
        shrn2           v21.16b, v17.8h,  #6
        xtn             v16.8b,  v16.8h           // (uint8_t)xpos
        xtn2            v16.16b, v17.8h

        ext             v5.16b,  v4.16b,  v4.16b,  #1 // top[base_x+1]
        ext             v7.16b,  v6.16b,  v6.16b,  #1

        and             v16.16b, v16.16b, v25.16b // frac_x

        trn1            v4.2d,   v4.2d,   v6.2d   // top[base_x]
        trn1            v5.2d,   v5.2d,   v7.2d   // top[base_x+1]

        sub             v7.16b,  v26.16b, v16.16b // 64 - frac_x

        add             v21.16b, v21.16b, v31.16b // actual base_x

        umull           v6.8h,   v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v6.8h,   v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull2          v17.8h,  v18.16b, v28.16b
        umlal2          v17.8h,  v19.16b, v27.16b

        umull           v22.8h,  v4.8b,   v7.8b   // top[base_x]-*(64-frac_x)
        umlal           v22.8h,  v5.8b,   v16.8b  // + top[base_x+1]*frac_x
        umull2          v23.8h,  v4.16b,  v7.16b
        umlal2          v23.8h,  v5.16b,  v16.16b

        cmge            v21.16b, v21.16b, #0

        rshrn           v6.8b,   v6.8h,   #6
        rshrn2          v6.16b,  v17.8h,  #6
        rshrn           v22.8b,  v22.8h,  #6
        rshrn2          v22.16b, v23.8h,  #6

        bit             v6.16b,  v22.16b, v21.16b

        st1             {v6.d}[0], [x0], x1
        sub             w8,  w8,  w6              // xpos -= dx
        subs            w5,  w5,  #2
        st1             {v6.d}[1], [x0], x1
        b.le            9f

        add             v29.16b, v29.16b, v24.16b // base_y += 4
        add             v30.16b, v30.16b, v24.16b // base_y += 4
        b               8b

89:
        tbl             v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b // left[base_y+0], left[base_y+2]
        tbl             v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b // left[base_y+1], left[base_y+3]

        umull           v6.8h,   v18.8b,  v28.8b  // left[base_y]*(64-frac_y)
        umlal           v6.8h,   v19.8b,  v27.8b  // + left[base_y+1]*frac_y
        umull2          v17.8h,  v18.16b, v28.16b
        umlal2          v17.8h,  v19.16b, v27.16b

        rshrn           v6.8b,   v6.8h,   #6
        rshrn2          v6.16b,  v17.8h,  #6

        st1             {v6.d}[0], [x0], x1
        subs            w5,  w5,  #2
        st1             {v6.d}[1], [x0], x1
        b.le            9f

        add             v29.16b, v29.16b, v24.16b // base_y += 4
        add             v30.16b, v30.16b, v24.16b // base_y += 4
        b               89b

9:
        ret
endfunc


// void ipred_z3_fill1_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const left,
//                               const int width, const int height,
//                               const int dy, const int max_base_y);
function ipred_z3_fill1_8bpc_neon, export=1
        cmp             w6,  #64
        clz             w9,  w3
        adr             x8,  L(ipred_z3_fill1_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x8, w9, uxtw #1]
        add             x10, x2,  w6,  uxtw       // left[max_base_y]
        sub             x8,  x8,  w9,  uxtw
        movrel          x11, increments
        ld1r            {v31.16b}, [x10]          // padding
        ld1             {v30.8h},  [x11]          // increments
        mov             w7,  w5
        b.gt            L(ipred_z3_fill1_large_h16)
        br              x8

40:
        AARCH64_VALID_JUMP_TARGET
        dup             v29.4h,  w5               // dy

        mul             v30.4h,  v30.4h,  v29.4h  // {0,1,2,3,4,5,6,7}*dy
        movi            v23.16b, #0x3e

        // Worst case max_base_y is width+height-1, for w=4, h=16, <= 32
        ld1             {v0.16b, v1.16b}, [x2] // left[]
        add             v30.4h,  v29.4h,  v30.4h  // ypos

        movi            v22.16b, #64
        movi            v20.16b, #1
        movi            v21.16b, #2

        xtn             v24.8b,  v30.8h           // (uint8_t)ypos
        uqshrn          v26.8b,  v30.8h,  #6      // base
        and             v24.8b,  v24.8b,  v23.8b  // frac

        mov             v4.8b,   v31.8b
        uqadd           v27.8b,  v26.8b,  v20.8b  // base + 1
        uqadd           v28.8b,  v26.8b,  v21.8b  // base + 2
        sub             v25.8b,  v22.8b,  v24.8b  // 64 - frac

        tbx             v4.8b, {v0.16b, v1.16b}, v26.8b // left[base]

        trn1            v27.2s,  v27.2s,  v28.2s  // base + 1, base + 2
        trn1            v24.2s,  v24.2s,  v24.2s  // frac
        trn1            v25.2s,  v25.2s,  v25.2s  // 64 - frac
1:
        mov             v5.8b,   v31.8b
        tbx             v5.8b, {v0.16b, v1.16b}, v27.8b // left[base+1], left[base+2]

        trn1            v4.2s,   v4.2s,   v5.2s   // left[base], left[base+1]

        umull           v16.8h,  v4.8b,   v25.8b  // left[base]*(64-frac)
        umlal           v16.8h,  v5.8b,   v24.8b  // + left[base+1]*frac
        rshrn           v16.8b,  v16.8h,  #6
        st1             {v16.s}[0], [x0], x1
        subs            w4,  w4,  #2
        st1             {v16.s}[1], [x0], x1
        b.le            9f

        ext             v4.8b,   v5.8b,   v5.8b,  #4
        uqadd           v27.8b,  v27.8b,  v21.8b  // base += 2
        b               1b

9:
        ret

80:
        AARCH64_VALID_JUMP_TARGET
        dup             v29.8h,  w5               // dy

        mul             v30.8h,  v30.8h,  v29.8h  // {0,1,2,3,4,5,6,7}*dy
        movi            v23.16b, #0x3e

        // Worst case max_base_y is width+height-1, for w=8, h=32, <= 48
        ld1             {v0.16b, v1.16b, v2.16b}, [x2] // left[]
        add             v30.8h,  v29.8h,  v30.8h  // ypos

        movi            v22.16b, #64
        movi            v20.16b, #1
        movi            v21.16b, #2

        xtn             v24.8b,  v30.8h           // (uint8_t)ypos
        uqshrn          v26.8b,  v30.8h,  #6      // base
        and             v24.8b,  v24.8b,  v23.8b  // frac

        mov             v4.8b,   v31.8b
        uqadd           v27.8b,  v26.8b,  v20.8b  // base + 1
        uqadd           v28.8b,  v26.8b,  v21.8b  // base + 2
        sub             v25.8b,  v22.8b,  v24.8b  // 64 - frac

        tbx             v4.8b, {v0.16b, v1.16b, v2.16b}, v26.8b // left[base]
1:
        mov             v5.8b,   v31.8b
        mov             v6.8b,   v31.8b
        tbx             v5.8b, {v0.16b, v1.16b, v2.16b}, v27.8b // left[base+1]
        tbx             v6.8b, {v0.16b, v1.16b, v2.16b}, v28.8b // left[base+2]

        umull           v16.8h,  v4.8b,   v25.8b  // left[base]*(64-frac)
        umlal           v16.8h,  v5.8b,   v24.8b  // + left[base+1]*frac
        umull           v17.8h,  v5.8b,   v25.8b
        umlal           v17.8h,  v6.8b,   v24.8b
        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v17.8b,  v17.8h,  #6
        st1             {v16.8b}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v17.8b}, [x0], x1
        b.le            9f

        mov             v4.8b,   v6.8b
        uqadd           v27.8b,  v27.8b,  v21.8b  // base += 2
        uqadd           v28.8b,  v28.8b,  v21.8b  // base += 2
        b               1b

9:
        ret

160:
        AARCH64_VALID_JUMP_TARGET
        dup             v28.8h,  w5               // dy

        shl             v29.8h,  v28.8h,  #3      // 8*dy
        mul             v30.8h,  v30.8h,  v28.8h  // {0,1,2,3,4,5,6,7}*dy
        movi            v23.16b, #0x3e

        // This is only executed if we've checked that max_base_y <= 64.
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x2] // left[]
        add             v28.8h,  v28.8h,  v30.8h  // ypos

        movi            v22.16b, #64
        movi            v20.16b, #1
        movi            v21.16b, #2

        add             v29.8h,  v28.8h,  v29.8h  // ypos + 8*dy

        xtn             v24.8b,  v28.8h           // (uint8_t)ypos
        xtn2            v24.16b, v29.8h
        uqshrn          v26.8b,  v28.8h,  #6      // base
        uqshrn2         v26.16b, v29.8h,  #6
        and             v24.16b, v24.16b, v23.16b // frac

        mov             v4.16b,  v31.16b
        uqadd           v27.16b, v26.16b, v20.16b // base + 1
        uqadd           v28.16b, v26.16b, v21.16b // base + 2
        sub             v25.16b, v22.16b, v24.16b // 64 - frac

        tbx             v4.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v26.16b // left[base]
1:
        mov             v5.16b,  v31.16b
        mov             v6.16b,  v31.16b
        tbx             v5.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v27.16b // left[base+1]
        tbx             v6.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v28.16b // left[base+2]

        umull           v16.8h,  v4.8b,   v25.8b  // left[base]*(64-frac)
        umlal           v16.8h,  v5.8b,   v24.8b  // + left[base+1]*frac
        umull2          v17.8h,  v4.16b,  v25.16b
        umlal2          v17.8h,  v5.16b,  v24.16b
        umull           v18.8h,  v5.8b,   v25.8b
        umlal           v18.8h,  v6.8b,   v24.8b
        umull2          v19.8h,  v5.16b,  v25.16b
        umlal2          v19.8h,  v6.16b,  v24.16b
        rshrn           v16.8b,  v16.8h,  #6
        rshrn2          v16.16b, v17.8h,  #6
        rshrn           v17.8b,  v18.8h,  #6
        rshrn2          v17.16b, v19.8h,  #6
        st1             {v16.16b}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v17.16b}, [x0], x1
        b.le            9f

        mov             v4.16b,  v6.16b
        uqadd           v27.16b, v27.16b, v21.16b // base += 2
        uqadd           v28.16b, v28.16b, v21.16b // base += 2
        b               1b

9:
        ret
320:
640:
        AARCH64_VALID_JUMP_TARGET
        dup             v28.8h,  w5               // dy
        mov             w12, w3

        add             x13, x0,  x1

        shl             v29.8h,  v28.8h,  #3      // 8*dy
        mul             v30.8h,  v30.8h,  v28.8h  // {0,1,2,3,4,5,6,7}*dy
        movi            v23.16b, #0x3e

        lsl             x1,  x1,  #1
        sub             x1,  x1,  w3,  uxtw
        add             v30.8h,  v28.8h,  v30.8h  // ypos

        // This is only executed if we've checked that max_base_y <= 64.
        ld1             {v0.16b, v1.16b, v2.16b, v3.16b}, [x2] // left[]

        movi            v22.16b, #64
        movi            v20.16b, #1
        movi            v21.16b, #2

1:
        mov             v26.16b,  v30.16b         // reset ypos

2:
        add             v27.8h,  v26.8h,  v29.8h  // ypos + 8*dy
        uqshrn          v16.8b,  v26.8h,  #6      // base
        uqshrn2         v16.16b, v27.8h,  #6
        xtn             v24.8b,  v26.8h           // (uint8_t)ypos
        xtn2            v24.16b, v27.8h
        umov            w14,     v16.b[0]
        and             v24.16b, v24.16b, v23.16b // frac

        uqadd           v17.16b, v16.16b, v20.16b // base + 1
        cmp             w14, w6                   // base >= max_base_y
        uqadd           v18.16b, v16.16b, v21.16b // base + 2
        sub             v25.16b, v22.16b, v24.16b // 64 - frac

        b.ge            4f

        mov             v4.16b,  v31.16b
        mov             v5.16b,  v31.16b
        mov             v6.16b,  v31.16b
        tbx             v4.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v16.16b // left[base]
        tbx             v5.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v17.16b // left[base+1]
        tbx             v6.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v18.16b // left[base+2]

        subs            w3,  w3,  #16
        umull           v16.8h,  v4.8b,   v25.8b  // left[base]*(64-frac)
        umlal           v16.8h,  v5.8b,   v24.8b  // + left[base+1]*frac
        umull2          v17.8h,  v4.16b,  v25.16b
        umlal2          v17.8h,  v5.16b,  v24.16b
        umull           v18.8h,  v5.8b,   v25.8b
        umlal           v18.8h,  v6.8b,   v24.8b
        umull2          v19.8h,  v5.16b,  v25.16b
        umlal2          v19.8h,  v6.16b,  v24.16b
        rshrn           v16.8b,  v16.8h,  #6
        rshrn2          v16.16b, v17.8h,  #6
        rshrn           v17.8b,  v18.8h,  #6
        rshrn2          v17.16b, v19.8h,  #6
        st1             {v16.16b}, [x0],  #16
        st1             {v17.16b}, [x13], #16
        b.le            3f
        add             v26.8h,  v27.8h,  v29.8h  // ypos += 16*dy
        b               2b

3:
        subs            w4,  w4,  #2
        b.le            9f
        movi            v16.8h,  #128
        add             x0,  x0,  x1
        add             x13, x13, x1
        add             v30.8h,  v30.8h,  v16.8h  // ypos = dy + y*(1<<6)*2
        mov             w3,  w12
        b               1b

4:
        subs            w3,  w3,  #16
        st1             {v31.16b}, [x0],  #16
        st1             {v31.16b}, [x13], #16
        b.gt            4b
        b               3b

9:
        ret

L(ipred_z3_fill1_large_h16):
        // Fallback case for max_base_y > 64; similar to the z1
        // implementation. This does the filtering vertically, filling out
        // a 2x pixel column at a time.
        mov             w15, #64
        add             x13, x0,  x1
        lsl             x1,  x1,  #1

        mov             w12, w4
1:
        lsr             w8,  w7,  #6              // base
        and             w9,  w7,  #0x3e           // frac
        add             w7,  w7,  w5              // ypos += dy
        cmp             w8,  w6                   // base >= max_base_y
        lsr             w10, w7,  #6              // base
        and             w11, w7,  #0x3e           // frac
        b.ge            ipred_z3_fill_padding_neon
        add             x8,  x2,  w8,  uxtw
        add             x10, x2,  w10, uxtw
        dup             v4.16b,  w9               // frac
        dup             v5.16b,  w11
        ld1             {v0.16b, v1.16b}, [x8],  #32 // left[base]
        ld1             {v2.16b, v3.16b}, [x10], #32
        sub             w9,  w15, w9              // 64 - frac
        sub             w11, w15, w11
        dup             v6.16b,  w9               // 64 - frac
        dup             v7.16b,  w11
        add             w7,  w7,  w5              // ypos += dy
2:
        ext             v16.16b, v0.16b,  v1.16b,  #1 // left[base+1]
        ext             v17.16b, v2.16b,  v3.16b,  #1
        subs            w4,  w4,  #16
        umull           v18.8h,  v16.8b,  v4.8b   // left[base+1]*frac
        umlal           v18.8h,  v0.8b,   v6.8b   // + left[base]*(64-frac)
        umull2          v19.8h,  v16.16b, v4.16b
        umlal2          v19.8h,  v0.16b,  v6.16b
        umull           v20.8h,  v17.8b,  v5.8b
        umlal           v20.8h,  v2.8b,   v7.8b
        umull2          v21.8h,  v17.16b, v5.16b
        umlal2          v21.8h,  v2.16b,  v7.16b
        rshrn           v16.8b,  v18.8h,  #6
        rshrn2          v16.16b, v19.8h,  #6
        rshrn           v17.8b,  v20.8h,  #6
        rshrn2          v17.16b, v21.8h,  #6
        zip1            v18.16b, v16.16b, v17.16b
        zip2            v19.16b, v16.16b, v17.16b
        st1             {v18.h}[0], [x0],  x1
        st1             {v18.h}[1], [x13], x1
        st1             {v18.h}[2], [x0],  x1
        st1             {v18.h}[3], [x13], x1
        st1             {v18.h}[4], [x0],  x1
        st1             {v18.h}[5], [x13], x1
        st1             {v18.h}[6], [x0],  x1
        st1             {v18.h}[7], [x13], x1
        st1             {v19.h}[0], [x0],  x1
        st1             {v19.h}[1], [x13], x1
        st1             {v19.h}[2], [x0],  x1
        st1             {v19.h}[3], [x13], x1
        st1             {v19.h}[4], [x0],  x1
        st1             {v19.h}[5], [x13], x1
        st1             {v19.h}[6], [x0],  x1
        st1             {v19.h}[7], [x13], x1
        b.le            3f
        mov             v0.16b,  v1.16b
        ld1             {v1.16b}, [x8],  #16      // left[base]
        mov             v2.16b,  v3.16b
        ld1             {v3.16b}, [x10], #16
        b               2b

3:
        subs            w3,  w3,  #2
        b.le            9f
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        lsl             x1,  x1,  #1
        add             x0,  x0,  #2
        add             x13, x13, #2
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
        cmp             w3,  #16
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
        st1             {v31.h}[0], [x0],  x1
        subs            w4,  w4,  #4
        st1             {v31.h}[0], [x13], x1
        st1             {v31.h}[0], [x0],  x1
        st1             {v31.h}[0], [x13], x1
        b.gt            2b
        subs            w3,  w3,  #2
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        b.le            9f
        lsl             x1,  x1,  #1
        add             x0,  x0,  #2
        add             x13, x13, #2
        mov             w4,  w12
        b               1b

4:
        st1             {v31.s}[0], [x0],  x1
        subs            w4,  w4,  #4
        st1             {v31.s}[0], [x13], x1
        st1             {v31.s}[0], [x0],  x1
        st1             {v31.s}[0], [x13], x1
        b.gt            4b
        subs            w3,  w3,  #4
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        b.le            9f
        lsl             x1,  x1,  #1
        add             x0,  x0,  #4
        add             x13, x13, #4
        mov             w4,  w12
        b               1b

8:
        st1             {v31.8b}, [x0],  x1
        subs            w4,  w4,  #4
        st1             {v31.8b}, [x13], x1
        st1             {v31.8b}, [x0],  x1
        st1             {v31.8b}, [x13], x1
        b.gt            4b
        subs            w3,  w3,  #8
        lsr             x1,  x1,  #1
        msub            x0,  x1,  x12, x0         // ptr -= h * stride
        msub            x13, x1,  x12, x13
        b.le            9f
        lsl             x1,  x1,  #1
        add             x0,  x0,  #8
        add             x13, x13, #8
        mov             w4,  w12
        b               1b

16:
32:
64:
        st1             {v31.16b}, [x0],  x1
        subs            w4,  w4,  #4
        st1             {v31.16b}, [x13], x1
        st1             {v31.16b}, [x0],  x1
        st1             {v31.16b}, [x13], x1
        b.gt            4b
        subs            w3,  w3,  #16
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
        // Fill a WxH rectangle with padding, with W > 16.
        lsr             x1,  x1,  #1
        mov             w12, w3
        sub             x1,  x1,  w3,  uxtw
1:
        ands            w5,  w3,  #15
        b.eq            2f
        // If the width isn't aligned to 16, first do one 16 byte write
        // and align the start pointer.
        sub             w3,  w3,  w5
        st1             {v31.16b}, [x0]
        add             x0,  x0,  w5,  uxtw
2:
        // Fill the rest of the line with aligned 16 byte writes.
        subs            w3,  w3,  #16
        st1             {v31.16b}, [x0], #16
        b.gt            2b
        subs            w4,  w4,  #1
        add             x0,  x0,  x1
        b.le            9f
        mov             w3,  w12
        b               1b
9:
        ret
endfunc

function ipred_z3_fill2_8bpc_neon, export=1
        cmp             w3,  #8
        add             x10, x2,  w6,  uxtw       // left[max_base_y]
        movrel          x11, increments
        ld1r            {v31.16b}, [x10]          // padding
        ld1             {v30.8h},  [x11]          // increments
        b.eq            80f

40:     // w == 4
        dup             v29.4h,  w5               // dy

        mul             v30.4h,  v30.4h,  v29.4h  // {0,1,2,3,4,5,6,7}*dy
        movi            v23.16b, #0x3e

        // Worst case max_base_y is 2*(width+height)-2, but width+height <= 16,
        // so max_base_y <= 32.
        ld1             {v0.16b, v1.16b}, [x2] // left[]
        add             v30.4h,  v29.4h,  v30.4h  // ypos

        movi            v22.16b, #64
        movi            v20.16b, #1
        movi            v21.16b, #2

        xtn             v24.8b,  v30.8h           // (uint8_t)ypos
        uqshrn          v26.8b,  v30.8h,  #6      // base
        and             v24.8b,  v24.8b,  v23.8b  // frac

        uqadd           v27.8b,  v26.8b,  v20.8b  // base + 1
        uqadd           v28.8b,  v26.8b,  v21.8b  // base + 2
        sub             v25.8b,  v22.8b,  v24.8b  // 64 - frac
        uqadd           v29.8b,  v27.8b,  v21.8b  // base + 3

        trn1            v24.2s,  v24.2s,  v24.2s  // frac
        trn1            v26.2s,  v26.2s,  v28.2s  // base + 0, base + 2
        trn1            v27.2s,  v27.2s,  v29.2s  // base + 1, base + 3
        trn1            v25.2s,  v25.2s,  v25.2s  // 64 - frac

        movi            v21.16b, #4
1:
        mov             v4.8b,   v31.8b
        mov             v5.8b,   v31.8b
        tbx             v4.8b, {v0.16b, v1.16b}, v26.8b // left[base], left[base+2]
        tbx             v5.8b, {v0.16b, v1.16b}, v27.8b // left[base+1], left[base+3]

        umull           v16.8h,  v4.8b,   v25.8b  // left[base]*(64-frac)
        umlal           v16.8h,  v5.8b,   v24.8b  // + left[base+1]*frac
        rshrn           v16.8b,  v16.8h,  #6
        st1             {v16.s}[0], [x0], x1
        subs            w4,  w4,  #2
        st1             {v16.s}[1], [x0], x1
        b.le            9f

        uqadd           v26.8b,  v26.8b,  v21.8b  // base += 4
        uqadd           v27.8b,  v27.8b,  v21.8b  // base += 4
        b               1b

9:
        ret

80:     // w == 8
        dup             v29.8h,  w5               // dy

        mul             v30.8h,  v30.8h,  v29.8h  // {0,1,2,3,4,5,6,7}*dy
        movi            v23.16b, #0x3e

        // Worst case max_base_y is 2*(width+height)-2, but width+height <= 16,
        // so max_base_y <= 32.
        ld1             {v0.16b, v1.16b}, [x2] // left[]
        add             v30.8h,  v29.8h,  v30.8h  // ypos

        movi            v22.16b, #64
        movi            v20.16b, #1
        movi            v21.16b, #2

        xtn             v24.8b,  v30.8h           // (uint8_t)ypos
        uqshrn          v26.8b,  v30.8h,  #6      // base
        and             v24.8b,  v24.8b,  v23.8b  // frac

        uqadd           v27.8b,  v26.8b,  v20.8b  // base + 1
        uqadd           v28.8b,  v26.8b,  v21.8b  // base + 2
        sub             v25.8b,  v22.8b,  v24.8b  // 64 - frac
        uqadd           v29.8b,  v27.8b,  v21.8b  // base + 3

        trn1            v24.2d,  v24.2d,  v24.2d  // frac
        trn1            v26.2d,  v26.2d,  v28.2d  // base + 0, base + 2
        trn1            v27.2d,  v27.2d,  v29.2d  // base + 1, base + 3
        trn1            v25.2d,  v25.2d,  v25.2d  // 64 - frac

        movi            v21.16b, #4
1:
        mov             v4.16b,  v31.16b
        mov             v5.16b,  v31.16b
        tbx             v4.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v26.16b // left[base], left[base+2]
        tbx             v5.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v27.16b // left[base+1], left[base+3]

        umull           v16.8h,  v4.8b,   v25.8b  // left[base]*(64-frac)
        umlal           v16.8h,  v5.8b,   v24.8b  // + left[base+1]*frac
        umull2          v17.8h,  v4.16b,  v25.16b
        umlal2          v17.8h,  v5.16b,  v24.16b
        rshrn           v16.8b,  v16.8h,  #6
        rshrn           v17.8b,  v17.8h,  #6
        st1             {v16.8b}, [x0], x1
        subs            w4,  w4,  #2
        st1             {v17.8b}, [x0], x1
        b.le            9f

        uqadd           v26.16b, v26.16b, v21.16b // base += 4
        uqadd           v27.16b, v27.16b, v21.16b // base += 4
        b               1b

9:
        ret
endfunc


// void ipred_filter_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                             const pixel *const topleft,
//                             const int width, const int height, const int filt_idx,
//                             const int max_width, const int max_height);
function ipred_filter_8bpc_neon, export=1
        and             w5,  w5,  #511
        movrel          x6,  X(filter_intra_taps)
        lsl             w5,  w5,  #6
        add             x6,  x6,  w5, uxtw
        ld1             {v16.8b, v17.8b, v18.8b, v19.8b}, [x6], #32
        clz             w9,  w3
        adr             x5,  L(ipred_filter_tbl)
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
        br              x5
40:
        AARCH64_VALID_JUMP_TARGET
        ldur            s0,  [x2, #1]             // top (0-3)
        sub             x2,  x2,  #2
        mov             x7,  #-2
        uxtl            v0.8h,   v0.8b            // top (0-3)
4:
        ld1             {v1.s}[0], [x2], x7       // left (0-1) + topleft (2)
        mul             v2.8h,   v17.8h,  v0.h[0] // p1(top[0]) * filter(1)
        mla             v2.8h,   v18.8h,  v0.h[1] // p2(top[1]) * filter(2)
        mla             v2.8h,   v19.8h,  v0.h[2] // p3(top[2]) * filter(3)
        uxtl            v1.8h,   v1.8b            // left (0-1) + topleft (2)
        mla             v2.8h,   v20.8h,  v0.h[3] // p4(top[3]) * filter(4)
        mla             v2.8h,   v16.8h,  v1.h[2] // p0(topleft) * filter(0)
        mla             v2.8h,   v21.8h,  v1.h[1] // p5(left[0]) * filter(5)
        mla             v2.8h,   v22.8h,  v1.h[0] // p6(left[1]) * filter(6)
        sqrshrun        v2.8b,   v2.8h,   #4
        subs            w4,  w4,  #2
        st1             {v2.s}[0], [x0], x1
        uxtl            v0.8h,   v2.8b
        st1             {v2.s}[1], [x6], x1
        ext             v0.16b,  v0.16b,  v0.16b, #8 // move top from [4-7] to [0-3]
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ldur            d0,  [x2, #1]             // top (0-7)
        sub             x2,  x2,  #2
        mov             x7,  #-2
        uxtl            v0.8h,   v0.8b            // top (0-7)
8:
        ld1             {v1.s}[0], [x2], x7       // left (0-1) + topleft (2)
        mul             v2.8h,   v17.8h,  v0.h[0] // p1(top[0]) * filter(1)
        mla             v2.8h,   v18.8h,  v0.h[1] // p2(top[1]) * filter(2)
        mla             v2.8h,   v19.8h,  v0.h[2] // p3(top[2]) * filter(3)
        uxtl            v1.8h,   v1.8b            // left (0-1) + topleft (2)
        mla             v2.8h,   v20.8h,  v0.h[3] // p4(top[3]) * filter(4)
        mla             v2.8h,   v16.8h,  v1.h[2] // p0(topleft) * filter(0)
        mla             v2.8h,   v21.8h,  v1.h[1] // p5(left[0]) * filter(5)
        mla             v2.8h,   v22.8h,  v1.h[0] // p6(left[1]) * filter(6)
        mul             v3.8h,   v17.8h,  v0.h[4] // p1(top[0]) * filter(1)
        mla             v3.8h,   v18.8h,  v0.h[5] // p2(top[1]) * filter(2)
        mla             v3.8h,   v19.8h,  v0.h[6] // p3(top[2]) * filter(3)
        sqrshrun        v2.8b,   v2.8h,   #4
        uxtl            v1.8h,   v2.8b            // first block, in 16 bit
        mla             v3.8h,   v20.8h,  v0.h[7] // p4(top[3]) * filter(4)
        mla             v3.8h,   v16.8h,  v0.h[3] // p0(topleft) * filter(0)
        mla             v3.8h,   v21.8h,  v1.h[3] // p5(left[0]) * filter(5)
        mla             v3.8h,   v22.8h,  v1.h[7] // p6(left[1]) * filter(6)
        sqrshrun        v3.8b,   v3.8h,   #4
        subs            w4,  w4,  #2
        st2             {v2.s, v3.s}[0], [x0], x1
        zip2            v0.2s,   v2.2s,   v3.2s
        st2             {v2.s, v3.s}[1], [x6], x1
        uxtl            v0.8h,   v0.8b
        b.gt            8b
        ret
160:
320:
        AARCH64_VALID_JUMP_TARGET
        add             x8,  x2,  #1
        sub             x2,  x2,  #2
        mov             x7,  #-2
        sub             x1,  x1,  w3, uxtw
        mov             w9,  w3

1:
        ld1             {v0.s}[0], [x2], x7       // left (0-1) + topleft (2)
        uxtl            v0.8h,   v0.8b            // left (0-1) + topleft (2)
2:
        ld1             {v2.16b}, [x8],   #16     // top(0-15)
        mul             v3.8h,   v16.8h,  v0.h[2] // p0(topleft) * filter(0)
        mla             v3.8h,   v21.8h,  v0.h[1] // p5(left[0]) * filter(5)
        uxtl            v1.8h,   v2.8b            // top(0-7)
        uxtl2           v2.8h,   v2.16b           // top(8-15)
        mla             v3.8h,   v22.8h,  v0.h[0] // p6(left[1]) * filter(6)
        mla             v3.8h,   v17.8h,  v1.h[0] // p1(top[0]) * filter(1)
        mla             v3.8h,   v18.8h,  v1.h[1] // p2(top[1]) * filter(2)
        mla             v3.8h,   v19.8h,  v1.h[2] // p3(top[2]) * filter(3)
        mla             v3.8h,   v20.8h,  v1.h[3] // p4(top[3]) * filter(4)

        mul             v4.8h,   v17.8h,  v1.h[4] // p1(top[0]) * filter(1)
        mla             v4.8h,   v18.8h,  v1.h[5] // p2(top[1]) * filter(2)
        mla             v4.8h,   v19.8h,  v1.h[6] // p3(top[2]) * filter(3)
        sqrshrun        v3.8b,   v3.8h,   #4
        uxtl            v0.8h,   v3.8b            // first block, in 16 bit
        mla             v4.8h,   v20.8h,  v1.h[7] // p4(top[3]) * filter(4)
        mla             v4.8h,   v16.8h,  v1.h[3] // p0(topleft) * filter(0)
        mla             v4.8h,   v21.8h,  v0.h[3] // p5(left[0]) * filter(5)
        mla             v4.8h,   v22.8h,  v0.h[7] // p6(left[1]) * filter(6)

        mul             v5.8h,   v17.8h,  v2.h[0] // p1(top[0]) * filter(1)
        mla             v5.8h,   v18.8h,  v2.h[1] // p2(top[1]) * filter(2)
        mla             v5.8h,   v19.8h,  v2.h[2] // p3(top[2]) * filter(3)
        sqrshrun        v4.8b,   v4.8h,   #4
        uxtl            v0.8h,   v4.8b            // second block, in 16 bit
        mla             v5.8h,   v20.8h,  v2.h[3] // p4(top[3]) * filter(4)
        mla             v5.8h,   v16.8h,  v1.h[7] // p0(topleft) * filter(0)
        mla             v5.8h,   v21.8h,  v0.h[3] // p5(left[0]) * filter(5)
        mla             v5.8h,   v22.8h,  v0.h[7] // p6(left[1]) * filter(6)

        mul             v6.8h,   v17.8h,  v2.h[4] // p1(top[0]) * filter(1)
        mla             v6.8h,   v18.8h,  v2.h[5] // p2(top[1]) * filter(2)
        mla             v6.8h,   v19.8h,  v2.h[6] // p3(top[2]) * filter(3)
        sqrshrun        v5.8b,   v5.8h,   #4
        uxtl            v0.8h,   v5.8b            // third block, in 16 bit
        mla             v6.8h,   v20.8h,  v2.h[7] // p4(top[3]) * filter(4)
        mla             v6.8h,   v16.8h,  v2.h[3] // p0(topleft) * filter(0)
        mla             v6.8h,   v21.8h,  v0.h[3] // p5(left[0]) * filter(5)
        mla             v6.8h,   v22.8h,  v0.h[7] // p6(left[1]) * filter(6)

        subs            w3,  w3,  #16
        sqrshrun        v6.8b,   v6.8h,   #4

        st4             {v3.s, v4.s, v5.s, v6.s}[0], [x0], #16
        st4             {v3.s, v4.s, v5.s, v6.s}[1], [x6], #16
        b.le            8f
        ins             v0.h[2], v2.h[7]
        ins             v0.b[0], v6.b[7]
        ins             v0.b[2], v6.b[3]
        b               2b
8:
        subs            w4,  w4,  #2
        b.le            9f
        sub             x8,  x6,  w9, uxtw
        add             x0,  x0,  x1
        add             x6,  x6,  x1
        mov             w3,  w9
        b               1b
9:
        ret

L(ipred_filter_tbl):
        .hword L(ipred_filter_tbl) - 320b
        .hword L(ipred_filter_tbl) - 160b
        .hword L(ipred_filter_tbl) -  80b
        .hword L(ipred_filter_tbl) -  40b
endfunc

// void pal_pred_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                         const uint16_t *const pal, const uint8_t *idx,
//                         const int w, const int h);
function pal_pred_8bpc_neon, export=1
        ld1             {v0.8h}, [x2]
        clz             w9,  w4
        adr             x6,  L(pal_pred_tbl)
        sub             w9,  w9,  #25
        ldrh            w9,  [x6, w9, uxtw #1]
        xtn             v0.8b,  v0.8h
        sub             x6,  x6,  w9, uxtw
        add             x2,  x0,  x1
        lsl             x1,  x1,  #1
        br              x6
4:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.16b}, [x3], #16
        subs            w5,  w5,  #4
        tbl             v1.16b, {v0.16b}, v1.16b
        st1             {v1.s}[0], [x0], x1
        st1             {v1.s}[1], [x2], x1
        st1             {v1.s}[2], [x0], x1
        st1             {v1.s}[3], [x2], x1
        b.gt            4b
        ret
8:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.16b, v2.16b}, [x3], #32
        subs            w5,  w5,  #4
        tbl             v1.16b, {v0.16b}, v1.16b
        st1             {v1.d}[0], [x0], x1
        tbl             v2.16b, {v0.16b}, v2.16b
        st1             {v1.d}[1], [x2], x1
        st1             {v2.d}[0], [x0], x1
        st1             {v2.d}[1], [x2], x1
        b.gt            8b
        ret
16:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v1.16b, v2.16b, v3.16b, v4.16b}, [x3], #64
        subs            w5,  w5,  #4
        tbl             v1.16b, {v0.16b}, v1.16b
        tbl             v2.16b, {v0.16b}, v2.16b
        st1             {v1.16b}, [x0], x1
        tbl             v3.16b, {v0.16b}, v3.16b
        st1             {v2.16b}, [x2], x1
        tbl             v4.16b, {v0.16b}, v4.16b
        st1             {v3.16b}, [x0], x1
        st1             {v4.16b}, [x2], x1
        b.gt            16b
        ret
32:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v16.16b, v17.16b, v18.16b, v19.16b}, [x3], #64
        ld1             {v20.16b, v21.16b, v22.16b, v23.16b}, [x3], #64
        subs            w5,  w5,  #4
        tbl             v16.16b, {v0.16b}, v16.16b
        tbl             v17.16b, {v0.16b}, v17.16b
        tbl             v18.16b, {v0.16b}, v18.16b
        tbl             v19.16b, {v0.16b}, v19.16b
        tbl             v20.16b, {v0.16b}, v20.16b
        st1             {v16.16b, v17.16b}, [x0], x1
        tbl             v21.16b, {v0.16b}, v21.16b
        st1             {v18.16b, v19.16b}, [x2], x1
        tbl             v22.16b, {v0.16b}, v22.16b
        st1             {v20.16b, v21.16b}, [x0], x1
        tbl             v23.16b, {v0.16b}, v23.16b
        st1             {v22.16b, v23.16b}, [x2], x1
        b.gt            32b
        ret
64:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v16.16b, v17.16b, v18.16b, v19.16b}, [x3], #64
        ld1             {v20.16b, v21.16b, v22.16b, v23.16b}, [x3], #64
        subs            w5,  w5,  #2
        tbl             v16.16b, {v0.16b}, v16.16b
        tbl             v17.16b, {v0.16b}, v17.16b
        tbl             v18.16b, {v0.16b}, v18.16b
        tbl             v19.16b, {v0.16b}, v19.16b
        st1             {v16.16b, v17.16b, v18.16b, v19.16b}, [x0], x1
        tbl             v20.16b, {v0.16b}, v20.16b
        tbl             v21.16b, {v0.16b}, v21.16b
        tbl             v22.16b, {v0.16b}, v22.16b
        tbl             v23.16b, {v0.16b}, v23.16b
        st1             {v20.16b, v21.16b, v22.16b, v23.16b}, [x2], x1
        b.gt            64b
        ret

L(pal_pred_tbl):
        .hword L(pal_pred_tbl) - 64b
        .hword L(pal_pred_tbl) - 32b
        .hword L(pal_pred_tbl) - 16b
        .hword L(pal_pred_tbl) -  8b
        .hword L(pal_pred_tbl) -  4b
endfunc

// void ipred_cfl_128_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height,
//                              const int16_t *ac, const int alpha);
function ipred_cfl_128_8bpc_neon, export=1
        clz             w9,  w3
        adr             x7,  L(ipred_cfl_128_tbl)
        sub             w9,  w9,  #26
        ldrh            w9,  [x7, w9, uxtw #1]
        movi            v0.8h,   #128 // dc
        dup             v1.8h,   w6   // alpha
        sub             x7,  x7,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x7
L(ipred_cfl_splat_w4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h}, [x5], #32
        mul             v2.8h,   v2.8h,   v1.8h  // diff = ac * alpha
        mul             v3.8h,   v3.8h,   v1.8h
        cmlt            v4.8h,   v2.8h,   #0     // sign
        cmlt            v5.8h,   v3.8h,   #0
        add             v2.8h,   v2.8h,   v4.8h  // diff + sign
        add             v3.8h,   v3.8h,   v5.8h
        srshr           v2.8h,   v2.8h,   #6     // (diff + sign + 32) >> 6 = apply_sign()
        srshr           v3.8h,   v3.8h,   #6
        add             v2.8h,   v2.8h,   v0.8h  // dc + apply_sign()
        add             v3.8h,   v3.8h,   v0.8h
        sqxtun          v2.8b,   v2.8h           // iclip_pixel(dc + apply_sign())
        sqxtun          v3.8b,   v3.8h
        st1             {v2.s}[0],  [x0], x1
        st1             {v2.s}[1],  [x6], x1
        subs            w4,  w4,  #4
        st1             {v3.s}[0],  [x0], x1
        st1             {v3.s}[1],  [x6], x1
        b.gt            L(ipred_cfl_splat_w4)
        ret
L(ipred_cfl_splat_w8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8h, v3.8h, v4.8h, v5.8h}, [x5], #64
        mul             v2.8h,   v2.8h,   v1.8h  // diff = ac * alpha
        mul             v3.8h,   v3.8h,   v1.8h
        mul             v4.8h,   v4.8h,   v1.8h
        mul             v5.8h,   v5.8h,   v1.8h
        cmlt            v16.8h,  v2.8h,   #0     // sign
        cmlt            v17.8h,  v3.8h,   #0
        cmlt            v18.8h,  v4.8h,   #0
        cmlt            v19.8h,  v5.8h,   #0
        add             v2.8h,   v2.8h,   v16.8h // diff + sign
        add             v3.8h,   v3.8h,   v17.8h
        add             v4.8h,   v4.8h,   v18.8h
        add             v5.8h,   v5.8h,   v19.8h
        srshr           v2.8h,   v2.8h,   #6     // (diff + sign + 32) >> 6 = apply_sign()
        srshr           v3.8h,   v3.8h,   #6
        srshr           v4.8h,   v4.8h,   #6
        srshr           v5.8h,   v5.8h,   #6
        add             v2.8h,   v2.8h,   v0.8h  // dc + apply_sign()
        add             v3.8h,   v3.8h,   v0.8h
        add             v4.8h,   v4.8h,   v0.8h
        add             v5.8h,   v5.8h,   v0.8h
        sqxtun          v2.8b,   v2.8h           // iclip_pixel(dc + apply_sign())
        sqxtun          v3.8b,   v3.8h
        sqxtun          v4.8b,   v4.8h
        sqxtun          v5.8b,   v5.8h
        st1             {v2.8b},  [x0], x1
        st1             {v3.8b},  [x6], x1
        subs            w4,  w4,  #4
        st1             {v4.8b},  [x0], x1
        st1             {v5.8b},  [x6], x1
        b.gt            L(ipred_cfl_splat_w8)
        ret
L(ipred_cfl_splat_w16):
        AARCH64_VALID_JUMP_TARGET
        add             x7,  x5,  w3, uxtw #1
        sub             x1,  x1,  w3, uxtw
        mov             w9,  w3
1:
        ld1             {v2.8h, v3.8h}, [x5], #32
        ld1             {v4.8h, v5.8h}, [x7], #32
        mul             v2.8h,   v2.8h,   v1.8h  // diff = ac * alpha
        mul             v3.8h,   v3.8h,   v1.8h
        mul             v4.8h,   v4.8h,   v1.8h
        mul             v5.8h,   v5.8h,   v1.8h
        cmlt            v16.8h,  v2.8h,   #0     // sign
        cmlt            v17.8h,  v3.8h,   #0
        cmlt            v18.8h,  v4.8h,   #0
        cmlt            v19.8h,  v5.8h,   #0
        add             v2.8h,   v2.8h,   v16.8h // diff + sign
        add             v3.8h,   v3.8h,   v17.8h
        add             v4.8h,   v4.8h,   v18.8h
        add             v5.8h,   v5.8h,   v19.8h
        srshr           v2.8h,   v2.8h,   #6     // (diff + sign + 32) >> 6 = apply_sign()
        srshr           v3.8h,   v3.8h,   #6
        srshr           v4.8h,   v4.8h,   #6
        srshr           v5.8h,   v5.8h,   #6
        add             v2.8h,   v2.8h,   v0.8h  // dc + apply_sign()
        add             v3.8h,   v3.8h,   v0.8h
        add             v4.8h,   v4.8h,   v0.8h
        add             v5.8h,   v5.8h,   v0.8h
        sqxtun          v2.8b,   v2.8h           // iclip_pixel(dc + apply_sign())
        sqxtun          v3.8b,   v3.8h
        sqxtun          v4.8b,   v4.8h
        sqxtun          v5.8b,   v5.8h
        subs            w3,  w3,  #16
        st1             {v2.8b, v3.8b},  [x0], #16
        st1             {v4.8b, v5.8b},  [x6], #16
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

// void ipred_cfl_top_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                              const pixel *const topleft,
//                              const int width, const int height,
//                              const int16_t *ac, const int alpha);
function ipred_cfl_top_8bpc_neon, export=1
        clz             w9,  w3
        adr             x7,  L(ipred_cfl_top_tbl)
        sub             w9,  w9,  #26
        ldrh            w9,  [x7, w9, uxtw #1]
        dup             v1.8h,   w6   // alpha
        add             x2,  x2,  #1
        sub             x7,  x7,  w9, uxtw
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x7
4:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v0.2s},  [x2]
        uaddlv          h0,      v0.8b
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w4)
8:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [x2]
        uaddlv          h0,      v0.8b
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w8)
16:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b}, [x2]
        uaddlv          h0,      v0.16b
        urshr           v0.4h,   v0.4h,   #4
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w16)
32:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.16b, v3.16b}, [x2]
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        add             v2.4h,   v2.4h,   v3.4h
        urshr           v2.4h,   v2.4h,   #5
        dup             v0.8h,   v2.h[0]
        b               L(ipred_cfl_splat_w16)

L(ipred_cfl_top_tbl):
        .hword L(ipred_cfl_top_tbl) - 32b
        .hword L(ipred_cfl_top_tbl) - 16b
        .hword L(ipred_cfl_top_tbl) -  8b
        .hword L(ipred_cfl_top_tbl) -  4b
endfunc

// void ipred_cfl_left_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                               const pixel *const topleft,
//                               const int width, const int height,
//                               const int16_t *ac, const int alpha);
function ipred_cfl_left_8bpc_neon, export=1
        sub             x2,  x2,  w4, uxtw
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
        br              x7

L(ipred_cfl_left_h4):
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v0.2s},  [x2]
        uaddlv          h0,      v0.8b
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
        br              x9

L(ipred_cfl_left_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [x2]
        uaddlv          h0,      v0.8b
        urshr           v0.4h,   v0.4h,   #3
        dup             v0.8h,   v0.h[0]
        br              x9

L(ipred_cfl_left_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b}, [x2]
        uaddlv          h0,      v0.16b
        urshr           v0.4h,   v0.4h,   #4
        dup             v0.8h,   v0.h[0]
        br              x9

L(ipred_cfl_left_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.16b, v3.16b}, [x2]
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        add             v2.4h,   v2.4h,   v3.4h
        urshr           v2.4h,   v2.4h,   #5
        dup             v0.8h,   v2.h[0]
        br              x9

L(ipred_cfl_left_tbl):
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h32)
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h16)
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h8)
        .hword L(ipred_cfl_left_tbl) - L(ipred_cfl_left_h4)
endfunc

// void ipred_cfl_8bpc_neon(pixel *dst, const ptrdiff_t stride,
//                          const pixel *const topleft,
//                          const int width, const int height,
//                          const int16_t *ac, const int alpha);
function ipred_cfl_8bpc_neon, export=1
        sub             x2,  x2,  w4, uxtw
        add             w8,  w3,  w4             // width + height
        dup             v1.8h,   w6              // alpha
        clz             w9,  w3
        clz             w6,  w4
        dup             v16.8h, w8               // width + height
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
        ushr            v16.8h,  v16.8h,  #1     // (width + height) >> 1
        dup             v17.8h,  w8              // -ctz(width + height)
        add             x6,  x0,  x1
        lsl             x1,  x1,  #1
        br              x7

L(ipred_cfl_h4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.s}[0],  [x2], #4
        ins             v0.s[1], wzr
        add             x2,  x2,  #1
        uaddlv          h0,      v0.8b
        br              x9
L(ipred_cfl_w4):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.s}[0],  [x2]
        ins             v2.s[1], wzr
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h2,      v2.8b
        cmp             w4,  #4
        add             v0.4h,   v0.4h,   v2.4h
        ushl            v0.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 8/16
        mov             w16, #(0x3334/2)
        movk            w16, #(0x5556/2), lsl #16
        add             w17, w4,  w4  // w17 = 2*h = 16 or 32
        lsr             w16, w16, w17
        dup             v16.4h,  w16
        sqdmulh         v0.4h,   v0.4h,   v16.4h
1:
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w4)

L(ipred_cfl_h8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [x2], #8
        uaddlv          h0,      v0.8b
        add             x2,  x2,  #1
        br              x9
L(ipred_cfl_w8):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.8b},  [x2]
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h2,      v2.8b
        cmp             w4,  #8
        add             v0.4h,   v0.4h,   v2.4h
        ushl            v0.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 4/16/32
        cmp             w4,  #32
        mov             w16, #(0x3334/2)
        mov             w17, #(0x5556/2)
        csel            w16, w16, w17, eq
        dup             v16.4h,  w16
        sqdmulh         v0.4h,   v0.4h,   v16.4h
1:
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w8)

L(ipred_cfl_h16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.16b}, [x2], #16
        uaddlv          h0,      v0.16b
        add             x2,  x2,  #1
        br              x9
L(ipred_cfl_w16):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.16b}, [x2]
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h2,      v2.16b
        cmp             w4,  #16
        add             v0.4h,   v0.4h,   v2.4h
        ushl            v0.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 4/8/32
        cmp             w4,  #4
        mov             w16, #(0x3334/2)
        mov             w17, #(0x5556/2)
        csel            w16, w16, w17, eq
        dup             v16.4h,  w16
        sqdmulh         v0.4h,   v0.4h,   v16.4h
1:
        dup             v0.8h,   v0.h[0]
        b               L(ipred_cfl_splat_w16)

L(ipred_cfl_h32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.16b, v3.16b}, [x2], #32
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        add             x2,  x2,  #1
        add             v0.4h,   v2.4h,   v3.4h
        br              x9
L(ipred_cfl_w32):
        AARCH64_VALID_JUMP_TARGET
        ld1             {v2.16b, v3.16b}, [x2]
        add             v0.4h,   v0.4h,   v16.4h
        uaddlv          h2,      v2.16b
        uaddlv          h3,      v3.16b
        cmp             w4,  #32
        add             v0.4h,   v0.4h,   v2.4h
        add             v0.4h,   v0.4h,   v3.4h
        ushl            v0.4h,   v0.4h,   v17.4h
        b.eq            1f
        // h = 8/16
        mov             w16, #(0x5556/2)
        movk            w16, #(0x3334/2), lsl #16
        add             w17, w4,  w4  // w17 = 2*h = 16 or 32
        lsr             w16, w16, w17
        dup             v16.4h,  w16
        sqdmulh         v0.4h,   v0.4h,   v16.4h
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

// void cfl_ac_420_8bpc_neon(int16_t *const ac, const pixel *const ypx,
//                           const ptrdiff_t stride, const int w_pad,
//                           const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_420_8bpc_neon, export=1
        clz             w8,  w5
        lsl             w4,  w4,  #2
        adr             x7,  L(ipred_cfl_ac_420_tbl)
        sub             w8,  w8,  #27
        ldrh            w8,  [x7, w8, uxtw #1]
        movi            v16.8h,  #0
        movi            v17.8h,  #0
        movi            v18.8h,  #0
        movi            v19.8h,  #0
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
        ld1             {v0.8b},   [x1],  x2
        ld1             {v1.8b},   [x10], x2
        ld1             {v0.d}[1], [x1],  x2
        ld1             {v1.d}[1], [x10], x2
        uaddlp          v0.8h,   v0.16b
        uaddlp          v1.8h,   v1.16b
        add             v0.8h,   v0.8h,   v1.8h
        shl             v0.8h,   v0.8h,   #1
        subs            w8,  w8,  #2
        st1             {v0.8h}, [x0], #16
        add             v16.8h,  v16.8h,  v0.8h
        b.gt            1b
        trn2            v1.2d,   v0.2d,   v0.2d
        trn2            v0.2d,   v0.2d,   v0.2d
L(ipred_cfl_ac_420_w4_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], #32
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        b.gt            2b
3:
        // Aggregate the sums
        add             v0.8h,   v16.8h,  v17.8h
        uaddlv          s0,  v0.8h                // sum
        sub             x0,  x0,  w6, uxtw #3
        urshl           v4.2s,   v0.2s,   v31.2s  // (sum + (1 << (log2sz - 1))) >>= log2sz
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
        ld1             {v0.16b}, [x1],  x2
        ld1             {v1.16b}, [x10], x2
        ld1             {v2.16b}, [x1],  x2
        uaddlp          v0.8h,   v0.16b
        ld1             {v3.16b}, [x10], x2
        uaddlp          v1.8h,   v1.16b
        uaddlp          v2.8h,   v2.16b
        uaddlp          v3.8h,   v3.16b
        add             v0.8h,   v0.8h,   v1.8h
        add             v2.8h,   v2.8h,   v3.8h
        shl             v0.8h,   v0.8h,   #1
        shl             v1.8h,   v2.8h,   #1
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h}, [x0], #32
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        b.gt            1b
        mov             v0.16b,  v1.16b
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_420_w8_wpad):
1:      // Copy and subsample input, padding 4
        ld1             {v0.8b},   [x1],  x2
        ld1             {v1.8b},   [x10], x2
        ld1             {v0.d}[1], [x1],  x2
        ld1             {v1.d}[1], [x10], x2
        uaddlp          v0.8h,   v0.16b
        uaddlp          v1.8h,   v1.16b
        add             v0.8h,   v0.8h,   v1.8h
        shl             v0.8h,   v0.8h,   #1
        dup             v1.4h,   v0.h[3]
        dup             v3.4h,   v0.h[7]
        trn2            v2.2d,   v0.2d,   v0.2d
        subs            w8,  w8,  #2
        st1             {v0.4h, v1.4h, v2.4h, v3.4h}, [x0], #32
        add             v16.4h,  v16.4h,  v0.4h
        add             v17.4h,  v17.4h,  v1.4h
        add             v18.4h,  v18.4h,  v2.4h
        add             v19.4h,  v19.4h,  v3.4h
        b.gt            1b
        trn1            v0.2d,   v2.2d,   v3.2d
        trn1            v1.2d,   v2.2d,   v3.2d

L(ipred_cfl_ac_420_w8_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h}, [x0], #32
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        st1             {v0.8h, v1.8h}, [x0], #32
        add             v18.8h,  v18.8h,  v0.8h
        add             v19.8h,  v19.8h,  v1.8h
        b.gt            2b
3:

L(ipred_cfl_ac_420_w8_calc_subtract_dc):
        // Aggregate the sums
        add             v0.8h,   v16.8h,  v17.8h
        add             v2.8h,   v18.8h,  v19.8h
        uaddlp          v0.4s,   v0.8h
        uaddlp          v2.4s,   v2.8h
        add             v0.4s,   v0.4s,   v2.4s
        addv            s0,  v0.4s                // sum
        sub             x0,  x0,  w6, uxtw #4
        urshl           v4.2s,   v0.2s,   v31.2s  // (sum + (1 << (log2sz - 1))) >>= log2sz
        dup             v4.8h,   v4.h[0]
L(ipred_cfl_ac_420_w8_subtract_dc):
6:      // Subtract dc from ac
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0]
        subs            w6,  w6,  #4
        sub             v0.8h,   v0.8h,   v4.8h
        sub             v1.8h,   v1.8h,   v4.8h
        sub             v2.8h,   v2.8h,   v4.8h
        sub             v3.8h,   v3.8h,   v4.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        b.gt            6b
        ret

L(ipred_cfl_ac_420_w16):
        AARCH64_VALID_JUMP_TARGET
        adr             x7,  L(ipred_cfl_ac_420_w16_tbl)
        ldrh            w3,  [x7, w3, uxtw #1]
        sub             x7,  x7,  w3, uxtw
        br              x7

L(ipred_cfl_ac_420_w16_wpad0):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, without padding
        ld1             {v0.16b, v1.16b}, [x1],  x2
        ld1             {v2.16b, v3.16b}, [x10], x2
        uaddlp          v0.8h,   v0.16b
        ld1             {v4.16b, v5.16b}, [x1],  x2
        uaddlp          v1.8h,   v1.16b
        ld1             {v6.16b, v7.16b}, [x10], x2
        uaddlp          v2.8h,   v2.16b
        uaddlp          v3.8h,   v3.16b
        uaddlp          v4.8h,   v4.16b
        uaddlp          v5.8h,   v5.16b
        uaddlp          v6.8h,   v6.16b
        uaddlp          v7.8h,   v7.16b
        add             v0.8h,   v0.8h,   v2.8h
        add             v1.8h,   v1.8h,   v3.8h
        add             v4.8h,   v4.8h,   v6.8h
        add             v5.8h,   v5.8h,   v7.8h
        shl             v0.8h,   v0.8h,   #1
        shl             v1.8h,   v1.8h,   #1
        shl             v2.8h,   v4.8h,   #1
        shl             v3.8h,   v5.8h,   #1
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad1):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 4
        ldr             d1,  [x1,  #16]
        ld1             {v0.16b}, [x1],  x2
        ldr             d3,  [x10, #16]
        ld1             {v2.16b}, [x10], x2
        uaddlp          v1.4h,   v1.8b
        ldr             d5,  [x1,  #16]
        uaddlp          v0.8h,   v0.16b
        ld1             {v4.16b}, [x1],  x2
        uaddlp          v3.4h,   v3.8b
        ldr             d7,  [x10, #16]
        uaddlp          v2.8h,   v2.16b
        ld1             {v6.16b}, [x10], x2
        uaddlp          v5.4h,   v5.8b
        uaddlp          v4.8h,   v4.16b
        uaddlp          v7.4h,   v7.8b
        uaddlp          v6.8h,   v6.16b
        add             v1.4h,   v1.4h,   v3.4h
        add             v0.8h,   v0.8h,   v2.8h
        add             v5.4h,   v5.4h,   v7.4h
        add             v4.8h,   v4.8h,   v6.8h
        shl             v1.4h,   v1.4h,   #1
        shl             v0.8h,   v0.8h,   #1
        shl             v3.4h,   v5.4h,   #1
        shl             v2.8h,   v4.8h,   #1
        dup             v4.4h,   v1.h[3]
        dup             v5.4h,   v3.h[3]
        trn1            v1.2d,   v1.2d,   v4.2d
        trn1            v3.2d,   v3.2d,   v5.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad2):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 8
        ld1             {v0.16b}, [x1],  x2
        ld1             {v2.16b}, [x10], x2
        ld1             {v4.16b}, [x1],  x2
        uaddlp          v0.8h,   v0.16b
        ld1             {v6.16b}, [x10], x2
        uaddlp          v2.8h,   v2.16b
        uaddlp          v4.8h,   v4.16b
        uaddlp          v6.8h,   v6.16b
        add             v0.8h,   v0.8h,   v2.8h
        add             v4.8h,   v4.8h,   v6.8h
        shl             v0.8h,   v0.8h,   #1
        shl             v2.8h,   v4.8h,   #1
        dup             v1.8h,   v0.h[7]
        dup             v3.8h,   v2.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_420_w16_wpad3):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 12
        ld1             {v0.8b}, [x1],  x2
        ld1             {v2.8b}, [x10], x2
        ld1             {v4.8b}, [x1],  x2
        uaddlp          v0.4h,   v0.8b
        ld1             {v6.8b}, [x10], x2
        uaddlp          v2.4h,   v2.8b
        uaddlp          v4.4h,   v4.8b
        uaddlp          v6.4h,   v6.8b
        add             v0.4h,   v0.4h,   v2.4h
        add             v4.4h,   v4.4h,   v6.4h
        shl             v0.4h,   v0.4h,   #1
        shl             v2.4h,   v4.4h,   #1
        dup             v1.8h,   v0.h[3]
        dup             v3.8h,   v2.h[3]
        trn1            v0.2d,   v0.2d,   v1.2d
        trn1            v2.2d,   v2.2d,   v3.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b

L(ipred_cfl_ac_420_w16_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            2b
3:

        // Double the height and reuse the w8 summing/subtracting
        lsl             w6,  w6,  #1
        b               L(ipred_cfl_ac_420_w8_calc_subtract_dc)

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

// void cfl_ac_422_8bpc_neon(int16_t *const ac, const pixel *const ypx,
//                           const ptrdiff_t stride, const int w_pad,
//                           const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_422_8bpc_neon, export=1
        clz             w8,  w5
        lsl             w4,  w4,  #2
        adr             x7,  L(ipred_cfl_ac_422_tbl)
        sub             w8,  w8,  #27
        ldrh            w8,  [x7, w8, uxtw #1]
        movi            v16.8h,  #0
        movi            v17.8h,  #0
        movi            v18.8h,  #0
        movi            v19.8h,  #0
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
        ld1             {v0.8b},   [x1],  x2
        ld1             {v0.d}[1], [x10], x2
        ld1             {v1.8b},   [x1],  x2
        ld1             {v1.d}[1], [x10], x2
        uaddlp          v0.8h,   v0.16b
        uaddlp          v1.8h,   v1.16b
        shl             v0.8h,   v0.8h,   #2
        shl             v1.8h,   v1.8h,   #2
        subs            w8,  w8,  #4
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        st1             {v0.8h, v1.8h}, [x0], #32
        b.gt            1b
        trn2            v0.2d,   v1.2d,   v1.2d
        trn2            v1.2d,   v1.2d,   v1.2d
        b               L(ipred_cfl_ac_420_w4_hpad)

L(ipred_cfl_ac_422_w8):
        AARCH64_VALID_JUMP_TARGET
        cbnz            w3,  L(ipred_cfl_ac_422_w8_wpad)
1:      // Copy and subsample input, without padding
        ld1             {v0.16b}, [x1],  x2
        ld1             {v1.16b}, [x10], x2
        ld1             {v2.16b}, [x1],  x2
        uaddlp          v0.8h,   v0.16b
        ld1             {v3.16b}, [x10], x2
        uaddlp          v1.8h,   v1.16b
        uaddlp          v2.8h,   v2.16b
        uaddlp          v3.8h,   v3.16b
        shl             v0.8h,   v0.8h,   #2
        shl             v1.8h,   v1.8h,   #2
        shl             v2.8h,   v2.8h,   #2
        shl             v3.8h,   v3.8h,   #2
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v3.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_422_w8_wpad):
1:      // Copy and subsample input, padding 4
        ld1             {v0.8b},   [x1],  x2
        ld1             {v0.d}[1], [x10], x2
        ld1             {v2.8b},   [x1],  x2
        ld1             {v2.d}[1], [x10], x2
        uaddlp          v0.8h,   v0.16b
        uaddlp          v2.8h,   v2.16b
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
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
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
        ld1             {v0.16b, v1.16b}, [x1],  x2
        ld1             {v2.16b, v3.16b}, [x10], x2
        uaddlp          v0.8h,   v0.16b
        uaddlp          v1.8h,   v1.16b
        uaddlp          v2.8h,   v2.16b
        uaddlp          v3.8h,   v3.16b
        shl             v0.8h,   v0.8h,   #2
        shl             v1.8h,   v1.8h,   #2
        shl             v2.8h,   v2.8h,   #2
        shl             v3.8h,   v3.8h,   #2
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad1):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 4
        ldr             d1,  [x1,  #16]
        ld1             {v0.16b}, [x1],  x2
        ldr             d3,  [x10, #16]
        ld1             {v2.16b}, [x10], x2
        uaddlp          v1.4h,   v1.8b
        uaddlp          v0.8h,   v0.16b
        uaddlp          v3.4h,   v3.8b
        uaddlp          v2.8h,   v2.16b
        shl             v1.4h,   v1.4h,   #2
        shl             v0.8h,   v0.8h,   #2
        shl             v3.4h,   v3.4h,   #2
        shl             v2.8h,   v2.8h,   #2
        dup             v4.4h,   v1.h[3]
        dup             v5.4h,   v3.h[3]
        trn1            v1.2d,   v1.2d,   v4.2d
        trn1            v3.2d,   v3.2d,   v5.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad2):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 8
        ld1             {v0.16b}, [x1],  x2
        ld1             {v2.16b}, [x10], x2
        uaddlp          v0.8h,   v0.16b
        uaddlp          v2.8h,   v2.16b
        shl             v0.8h,   v0.8h,   #2
        shl             v2.8h,   v2.8h,   #2
        dup             v1.8h,   v0.h[7]
        dup             v3.8h,   v2.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v2.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_422_w16_wpad3):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and subsample input, padding 12
        ld1             {v0.8b}, [x1],  x2
        ld1             {v2.8b}, [x10], x2
        uaddlp          v0.4h,   v0.8b
        uaddlp          v2.4h,   v2.8b
        shl             v0.4h,   v0.4h,   #2
        shl             v2.4h,   v2.4h,   #2
        dup             v1.8h,   v0.h[3]
        dup             v3.8h,   v2.h[3]
        trn1            v0.2d,   v0.2d,   v1.2d
        trn1            v2.2d,   v2.2d,   v3.2d
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
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

// void cfl_ac_444_8bpc_neon(int16_t *const ac, const pixel *const ypx,
//                           const ptrdiff_t stride, const int w_pad,
//                           const int h_pad, const int cw, const int ch);
function ipred_cfl_ac_444_8bpc_neon, export=1
        clz             w8,  w5
        lsl             w4,  w4,  #2
        adr             x7,  L(ipred_cfl_ac_444_tbl)
        sub             w8,  w8,  #26
        ldrh            w8,  [x7, w8, uxtw #1]
        movi            v16.8h,  #0
        movi            v17.8h,  #0
        movi            v18.8h,  #0
        movi            v19.8h,  #0
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
        ld1             {v0.s}[0], [x1],  x2
        ld1             {v0.s}[1], [x10], x2
        ld1             {v1.s}[0], [x1],  x2
        ld1             {v1.s}[1], [x10], x2
        ushll           v0.8h,   v0.8b,   #3
        ushll           v1.8h,   v1.8b,   #3
        subs            w8,  w8,  #4
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        st1             {v0.8h, v1.8h}, [x0], #32
        b.gt            1b
        trn2            v0.2d,   v1.2d,   v1.2d
        trn2            v1.2d,   v1.2d,   v1.2d
        b               L(ipred_cfl_ac_420_w4_hpad)

L(ipred_cfl_ac_444_w8):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input
        ld1             {v0.8b}, [x1],  x2
        ld1             {v1.8b}, [x10], x2
        ld1             {v2.8b}, [x1],  x2
        ushll           v0.8h,   v0.8b,   #3
        ld1             {v3.8b}, [x10], x2
        ushll           v1.8h,   v1.8b,   #3
        ushll           v2.8h,   v2.8b,   #3
        ushll           v3.8h,   v3.8b,   #3
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        b.gt            1b
        mov             v0.16b,  v3.16b
        mov             v1.16b,  v3.16b
        b               L(ipred_cfl_ac_420_w8_hpad)

L(ipred_cfl_ac_444_w16):
        AARCH64_VALID_JUMP_TARGET
        cbnz            w3,  L(ipred_cfl_ac_444_w16_wpad)
1:      // Copy and expand input, without padding
        ld1             {v0.16b}, [x1],  x2
        ld1             {v2.16b}, [x10], x2
        ld1             {v4.16b}, [x1],  x2
        ushll2          v1.8h,   v0.16b,  #3
        ushll           v0.8h,   v0.8b,   #3
        ld1             {v6.16b}, [x10], x2
        ushll2          v3.8h,   v2.16b,  #3
        ushll           v2.8h,   v2.8b,   #3
        ushll2          v5.8h,   v4.16b,  #3
        ushll           v4.8h,   v4.8b,   #3
        ushll2          v7.8h,   v6.16b,  #3
        ushll           v6.8h,   v6.8b,   #3
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        b.gt            1b
        mov             v0.16b,  v6.16b
        mov             v1.16b,  v7.16b
        mov             v2.16b,  v6.16b
        mov             v3.16b,  v7.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_444_w16_wpad):
1:      // Copy and expand input, padding 8
        ld1             {v0.8b}, [x1],  x2
        ld1             {v2.8b}, [x10], x2
        ld1             {v4.8b}, [x1],  x2
        ld1             {v6.8b}, [x10], x2
        ushll           v0.8h,   v0.8b,   #3
        ushll           v2.8h,   v2.8b,   #3
        ushll           v4.8h,   v4.8b,   #3
        ushll           v6.8h,   v6.8b,   #3
        dup             v1.8h,   v0.h[7]
        dup             v3.8h,   v2.h[7]
        dup             v5.8h,   v4.h[7]
        dup             v7.8h,   v6.h[7]
        subs            w8,  w8,  #4
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        b.gt            1b
        mov             v0.16b,  v6.16b
        mov             v1.16b,  v7.16b
        mov             v2.16b,  v6.16b
        mov             v3.16b,  v7.16b
        b               L(ipred_cfl_ac_420_w16_hpad)

L(ipred_cfl_ac_444_w32):
        AARCH64_VALID_JUMP_TARGET
        adr             x7,  L(ipred_cfl_ac_444_w32_tbl)
        ldrh            w3,  [x7, w3, uxtw] // (w3>>1) << 1
        sub             x7,  x7,  w3, uxtw
        br              x7

L(ipred_cfl_ac_444_w32_wpad0):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, without padding
        ld1             {v2.16b, v3.16b}, [x1],  x2
        ld1             {v6.16b, v7.16b}, [x10], x2
        ushll           v0.8h,   v2.8b,   #3
        ushll2          v1.8h,   v2.16b,  #3
        ushll           v2.8h,   v3.8b,   #3
        ushll2          v3.8h,   v3.16b,  #3
        ushll           v4.8h,   v6.8b,   #3
        ushll2          v5.8h,   v6.16b,  #3
        ushll           v6.8h,   v7.8b,   #3
        ushll2          v7.8h,   v7.16b,  #3
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        b.gt            1b
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad2):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, padding 8
        ldr             d2,  [x1,  #16]
        ld1             {v1.16b}, [x1],  x2
        ldr             d6,  [x10, #16]
        ld1             {v5.16b}, [x10], x2
        ushll           v2.8h,   v2.8b,   #3
        ushll           v0.8h,   v1.8b,   #3
        ushll2          v1.8h,   v1.16b,  #3
        ushll           v6.8h,   v6.8b,   #3
        ushll           v4.8h,   v5.8b,   #3
        ushll2          v5.8h,   v5.16b,  #3
        dup             v3.8h,   v2.h[7]
        dup             v7.8h,   v6.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        b.gt            1b
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad4):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, padding 16
        ld1             {v1.16b}, [x1],  x2
        ld1             {v5.16b}, [x10], x2
        ushll           v0.8h,   v1.8b,   #3
        ushll2          v1.8h,   v1.16b,  #3
        ushll           v4.8h,   v5.8b,   #3
        ushll2          v5.8h,   v5.16b,  #3
        dup             v2.8h,   v1.h[7]
        dup             v3.8h,   v1.h[7]
        dup             v6.8h,   v5.h[7]
        dup             v7.8h,   v5.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        b.gt            1b
        b               L(ipred_cfl_ac_444_w32_hpad)

L(ipred_cfl_ac_444_w32_wpad6):
        AARCH64_VALID_JUMP_TARGET
1:      // Copy and expand input, padding 24
        ld1             {v0.8b}, [x1],  x2
        ld1             {v4.8b}, [x10], x2
        ushll           v0.8h,   v0.8b,   #3
        ushll           v4.8h,   v4.8b,   #3
        dup             v1.8h,   v0.h[7]
        dup             v2.8h,   v0.h[7]
        dup             v3.8h,   v0.h[7]
        dup             v5.8h,   v4.h[7]
        dup             v6.8h,   v4.h[7]
        dup             v7.8h,   v4.h[7]
        subs            w8,  w8,  #2
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v0.8h
        add             v17.8h,  v17.8h,  v1.8h
        add             v18.8h,  v18.8h,  v2.8h
        add             v19.8h,  v19.8h,  v3.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        b.gt            1b

L(ipred_cfl_ac_444_w32_hpad):
        cbz             w4,  3f
2:      // Vertical padding (h_pad > 0)
        subs            w4,  w4,  #2
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        st1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
        add             v16.8h,  v16.8h,  v4.8h
        add             v17.8h,  v17.8h,  v5.8h
        add             v18.8h,  v18.8h,  v6.8h
        add             v19.8h,  v19.8h,  v7.8h
        b.gt            2b
3:

        // Quadruple the height and reuse the w8 subtracting
        lsl             w6,  w6,  #2
        // Aggregate the sums, with wider intermediates earlier than in
        // ipred_cfl_ac_420_w8_calc_subtract_dc.
        uaddlp          v0.4s,   v16.8h
        uaddlp          v1.4s,   v17.8h
        uaddlp          v2.4s,   v18.8h
        uaddlp          v3.4s,   v19.8h
        add             v0.4s,   v0.4s,   v1.4s
        add             v2.4s,   v2.4s,   v3.4s
        add             v0.4s,   v0.4s,   v2.4s
        addv            s0,  v0.4s                // sum
        sub             x0,  x0,  w6, uxtw #4
        urshl           v4.2s,   v0.2s,   v31.2s  // (sum + (1 << (log2sz - 1))) >>= log2sz
        dup             v4.8h,   v4.h[0]
        b               L(ipred_cfl_ac_420_w8_subtract_dc)

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
