/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Janne Grunau
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

#define PREP_BIAS 8192

.macro avg d0, d1, t0, t1, t2, t3
        ld1             {\t0\().8h,\t1\().8h},  [x2],  32
        ld1             {\t2\().8h,\t3\().8h},  [x3],  32
        sqadd           \t0\().8h,  \t0\().8h,  \t2\().8h
        sqadd           \t1\().8h,  \t1\().8h,  \t3\().8h
        smax            \t0\().8h,  \t0\().8h,  v28.8h // -2*PREP_BIAS - 1 << intermediate_bits
        smax            \t1\().8h,  \t1\().8h,  v28.8h // -2*PREP_BIAS - 1 << intermediate_bits
        sqsub           \t0\().8h,  \t0\().8h,  v28.8h // -2*PREP_BIAS - 1 << intermediate_bits
        sqsub           \t1\().8h,  \t1\().8h,  v28.8h // -2*PREP_BIAS - 1 << intermediate_bits
        sshl            \d0\().8h,  \t0\().8h,  v29.8h // -(intermediate_bits+1)
        sshl            \d1\().8h,  \t1\().8h,  v29.8h // -(intermediate_bits+1)
.endm

.macro w_avg d0, d1, t0, t1, t2, t3
        ld1             {\t0\().8h,\t1\().8h},  [x2],  32
        ld1             {\t2\().8h,\t3\().8h},  [x3],  32
        // This difference requires a 17 bit range, and all bits are
        // significant for the following multiplication.
        ssubl           \d0\().4s,  \t2\().4h,  \t0\().4h
        ssubl2          \t0\().4s,  \t2\().8h,  \t0\().8h
        ssubl           \d1\().4s,  \t3\().4h,  \t1\().4h
        ssubl2          \t1\().4s,  \t3\().8h,  \t1\().8h
        mul             \d0\().4s,  \d0\().4s,  v27.4s
        mul             \t0\().4s,  \t0\().4s,  v27.4s
        mul             \d1\().4s,  \d1\().4s,  v27.4s
        mul             \t1\().4s,  \t1\().4s,  v27.4s
        sshr            \d0\().4s,  \d0\().4s,  #4
        sshr            \t0\().4s,  \t0\().4s,  #4
        sshr            \d1\().4s,  \d1\().4s,  #4
        sshr            \t1\().4s,  \t1\().4s,  #4
        saddw           \d0\().4s,  \d0\().4s,  \t2\().4h
        saddw2          \t0\().4s,  \t0\().4s,  \t2\().8h
        saddw           \d1\().4s,  \d1\().4s,  \t3\().4h
        saddw2          \t1\().4s,  \t1\().4s,  \t3\().8h
        uzp1            \d0\().8h,  \d0\().8h,  \t0\().8h // Same as xtn, xtn2
        uzp1            \d1\().8h,  \d1\().8h,  \t1\().8h // Ditto
        srshl           \d0\().8h,  \d0\().8h,  v29.8h // -intermediate_bits
        srshl           \d1\().8h,  \d1\().8h,  v29.8h // -intermediate_bits
        add             \d0\().8h,  \d0\().8h,  v28.8h // PREP_BIAS >> intermediate_bits
        add             \d1\().8h,  \d1\().8h,  v28.8h // PREP_BIAS >> intermediate_bits
        smin            \d0\().8h,  \d0\().8h,  v31.8h // bitdepth_max
        smin            \d1\().8h,  \d1\().8h,  v31.8h // bitdepth_max
        smax            \d0\().8h,  \d0\().8h,  v30.8h // 0
        smax            \d1\().8h,  \d1\().8h,  v30.8h // 0
.endm

.macro mask d0, d1, t0, t1, t2, t3
        ld1             {v27.16b}, [x6],  16
        ld1             {\t0\().8h,\t1\().8h},  [x2],  32
        neg             v27.16b, v27.16b
        ld1             {\t2\().8h,\t3\().8h},  [x3],  32
        sxtl            v26.8h,  v27.8b
        sxtl2           v27.8h,  v27.16b
        sxtl            v24.4s,  v26.4h
        sxtl2           v25.4s,  v26.8h
        sxtl            v26.4s,  v27.4h
        sxtl2           v27.4s,  v27.8h
        ssubl           \d0\().4s,  \t2\().4h,  \t0\().4h
        ssubl2          \t0\().4s,  \t2\().8h,  \t0\().8h
        ssubl           \d1\().4s,  \t3\().4h,  \t1\().4h
        ssubl2          \t1\().4s,  \t3\().8h,  \t1\().8h
        mul             \d0\().4s,  \d0\().4s,  v24.4s
        mul             \t0\().4s,  \t0\().4s,  v25.4s
        mul             \d1\().4s,  \d1\().4s,  v26.4s
        mul             \t1\().4s,  \t1\().4s,  v27.4s
        sshr            \d0\().4s,  \d0\().4s,  #6
        sshr            \t0\().4s,  \t0\().4s,  #6
        sshr            \d1\().4s,  \d1\().4s,  #6
        sshr            \t1\().4s,  \t1\().4s,  #6
        saddw           \d0\().4s,  \d0\().4s,  \t2\().4h
        saddw2          \t0\().4s,  \t0\().4s,  \t2\().8h
        saddw           \d1\().4s,  \d1\().4s,  \t3\().4h
        saddw2          \t1\().4s,  \t1\().4s,  \t3\().8h
        uzp1            \d0\().8h,  \d0\().8h,  \t0\().8h  // Same as xtn, xtn2
        uzp1            \d1\().8h,  \d1\().8h,  \t1\().8h  // Ditto
        srshl           \d0\().8h,  \d0\().8h,  v29.8h // -intermediate_bits
        srshl           \d1\().8h,  \d1\().8h,  v29.8h // -intermediate_bits
        add             \d0\().8h,  \d0\().8h,  v28.8h // PREP_BIAS >> intermediate_bits
        add             \d1\().8h,  \d1\().8h,  v28.8h // PREP_BIAS >> intermediate_bits
        smin            \d0\().8h,  \d0\().8h,  v31.8h // bitdepth_max
        smin            \d1\().8h,  \d1\().8h,  v31.8h // bitdepth_max
        smax            \d0\().8h,  \d0\().8h,  v30.8h // 0
        smax            \d1\().8h,  \d1\().8h,  v30.8h // 0
.endm

.macro bidir_fn type, bdmax
function \type\()_16bpc_neon, export=1
        clz             w4,  w4
.ifnc \type, avg
        dup             v31.8h,  \bdmax // bitdepth_max
        movi            v30.8h,  #0
.endif
        clz             w7,  \bdmax
        sub             w7,  w7,  #18   // intermediate_bits = clz(bitdepth_max) - 18
.ifc \type, avg
        mov             w9,  #1
        mov             w8,  #-2*PREP_BIAS
        lsl             w9,  w9,  w7    // 1 << intermediate_bits
        add             w7,  w7,  #1
        sub             w8,  w8,  w9    // -2*PREP_BIAS - 1 << intermediate_bits
        neg             w7,  w7         // -(intermediate_bits+1)
        dup             v28.8h,   w8    // -2*PREP_BIAS - 1 << intermediate_bits
        dup             v29.8h,   w7    // -(intermediate_bits+1)
.else
        mov             w8,  #PREP_BIAS
        lsr             w8,  w8,  w7    // PREP_BIAS >> intermediate_bits
        neg             w7,  w7         // -intermediate_bits
        dup             v28.8h,  w8     // PREP_BIAS >> intermediate_bits
        dup             v29.8h,  w7     // -intermediate_bits
.endif
.ifc \type, w_avg
        dup             v27.4s,  w6
        neg             v27.4s,  v27.4s
.endif
        adr             x7,  L(\type\()_tbl)
        sub             w4,  w4,  #24
        \type           v4,  v5,  v0,  v1,  v2,  v3
        ldrh            w4,  [x7, x4, lsl #1]
        sub             x7,  x7,  w4, uxtw
        br              x7
40:
        AARCH64_VALID_JUMP_TARGET
        add             x7,  x0,  x1
        lsl             x1,  x1,  #1
4:
        subs            w5,  w5,  #4
        st1             {v4.d}[0],  [x0], x1
        st1             {v4.d}[1],  [x7], x1
        st1             {v5.d}[0],  [x0], x1
        st1             {v5.d}[1],  [x7], x1
        b.le            0f
        \type           v4,  v5,  v0,  v1,  v2,  v3
        b               4b
80:
        AARCH64_VALID_JUMP_TARGET
        add             x7,  x0,  x1
        lsl             x1,  x1,  #1
8:
        st1             {v4.8h},  [x0], x1
        subs            w5,  w5,  #2
        st1             {v5.8h},  [x7], x1
        b.le            0f
        \type           v4,  v5,  v0,  v1,  v2,  v3
        b               8b
16:
        AARCH64_VALID_JUMP_TARGET
        \type           v6,  v7,  v0,  v1,  v2,  v3
        st1             {v4.8h, v5.8h}, [x0], x1
        subs            w5,  w5,  #2
        st1             {v6.8h, v7.8h}, [x0], x1
        b.le            0f
        \type           v4,  v5,  v0,  v1,  v2,  v3
        b               16b
32:
        AARCH64_VALID_JUMP_TARGET
        \type           v6,  v7,  v0,  v1,  v2,  v3
        subs            w5,  w5,  #1
        st1             {v4.8h, v5.8h, v6.8h, v7.8h},  [x0], x1
        b.le            0f
        \type           v4,  v5,  v0,  v1,  v2,  v3
        b               32b
640:
        AARCH64_VALID_JUMP_TARGET
        add             x7,  x0,  #64
64:
        \type           v6,  v7,  v0,  v1,  v2,  v3
        \type           v16, v17, v0,  v1,  v2,  v3
        st1             {v4.8h, v5.8h, v6.8h, v7.8h},  [x0], x1
        \type           v18, v19, v0,  v1,  v2,  v3
        subs            w5,  w5,  #1
        st1             {v16.8h,v17.8h,v18.8h,v19.8h}, [x7], x1
        b.le            0f
        \type           v4,  v5,  v0,  v1,  v2,  v3
        b               64b
1280:
        AARCH64_VALID_JUMP_TARGET
        add             x7,  x0,  #64
        mov             x8,  #128
        sub             x1,  x1,  #128
128:
        \type           v6,  v7,  v0,  v1,  v2,  v3
        \type           v16, v17, v0,  v1,  v2,  v3
        st1             {v4.8h, v5.8h, v6.8h, v7.8h},  [x0], x8
        \type           v18, v19, v0,  v1,  v2,  v3
        st1             {v16.8h,v17.8h,v18.8h,v19.8h}, [x7], x8
        \type           v4,  v5,  v0,  v1,  v2,  v3
        \type           v6,  v7,  v0,  v1,  v2,  v3
        \type           v16, v17, v0,  v1,  v2,  v3
        subs            w5,  w5,  #1
        st1             {v4.8h, v5.8h, v6.8h, v7.8h},  [x0], x1
        \type           v18, v19, v0,  v1,  v2,  v3
        st1             {v16.8h,v17.8h,v18.8h,v19.8h}, [x7], x1
        b.le            0f
        \type           v4,  v5,  v0,  v1,  v2,  v3
        b               128b
0:
        ret
L(\type\()_tbl):
        .hword L(\type\()_tbl) - 1280b
        .hword L(\type\()_tbl) -  640b
        .hword L(\type\()_tbl) -   32b
        .hword L(\type\()_tbl) -   16b
        .hword L(\type\()_tbl) -   80b
        .hword L(\type\()_tbl) -   40b
endfunc
.endm

bidir_fn avg, w6
bidir_fn w_avg, w7
bidir_fn mask, w7


.macro w_mask_fn type
function w_mask_\type\()_16bpc_neon, export=1
        ldr             w8,  [sp]
        clz             w9,  w4
        adr             x10, L(w_mask_\type\()_tbl)
        dup             v31.8h,  w8   // bitdepth_max
        sub             w9,  w9,  #24
        clz             w8,  w8       // clz(bitdepth_max)
        ldrh            w9,  [x10,  x9,  lsl #1]
        sub             x10, x10, w9,  uxtw
        sub             w8,  w8,  #12 // sh = intermediate_bits + 6 = clz(bitdepth_max) - 12
        mov             w9,  #PREP_BIAS*64
        neg             w8,  w8       // -sh
        mov             w11, #27615   // (64 + 1 - 38)<<mask_sh - 1 - mask_rnd
        dup             v30.4s,  w9   // PREP_BIAS*64
        dup             v29.4s,  w8   // -sh
        dup             v0.8h,   w11
.if \type == 444
        movi            v1.16b,  #64
.elseif \type == 422
        dup             v2.8b,   w7
        movi            v3.8b,   #129
        sub             v3.8b,   v3.8b,   v2.8b
.elseif \type == 420
        dup             v2.8h,   w7
        movi            v3.8h,   #1, lsl #8
        sub             v3.8h,   v3.8h,   v2.8h
.endif
        add             x12,  x0,  x1
        lsl             x1,   x1,  #1
        br              x10
4:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v4.8h, v5.8h}, [x2], #32 // tmp1 (four rows at once)
        ld1             {v6.8h, v7.8h}, [x3], #32 // tmp2 (four rows at once)
        subs            w5,  w5,  #4
        sabd            v20.8h,  v4.8h,   v6.8h   // abs(tmp1 - tmp2)
        sabd            v21.8h,  v5.8h,   v7.8h
        ssubl           v16.4s,  v6.4h,   v4.4h   // tmp2 - tmp1 (requires 17 bit)
        ssubl2          v17.4s,  v6.8h,   v4.8h
        ssubl           v18.4s,  v7.4h,   v5.4h
        ssubl2          v19.4s,  v7.8h,   v5.8h
        uqsub           v20.8h,  v0.8h,   v20.8h  // 27615 - abs()
        uqsub           v21.8h,  v0.8h,   v21.8h
        sshll2          v7.4s,   v5.8h,   #6      // tmp1 << 6
        sshll           v6.4s,   v5.4h,   #6
        sshll2          v5.4s,   v4.8h,   #6
        sshll           v4.4s,   v4.4h,   #6
        ushr            v20.8h,  v20.8h,  #10     // 64-m = (27615 - abs()) >> mask_sh
        ushr            v21.8h,  v21.8h,  #10
        add             v4.4s,   v4.4s,   v30.4s  // += PREP_BIAS*64
        add             v5.4s,   v5.4s,   v30.4s
        add             v6.4s,   v6.4s,   v30.4s
        add             v7.4s,   v7.4s,   v30.4s
        uxtl            v22.4s,  v20.4h
        uxtl2           v23.4s,  v20.8h
        uxtl            v24.4s,  v21.4h
        uxtl2           v25.4s,  v21.8h
        mla             v4.4s,   v16.4s,  v22.4s  // (tmp2-tmp1)*(64-m)
        mla             v5.4s,   v17.4s,  v23.4s
        mla             v6.4s,   v18.4s,  v24.4s
        mla             v7.4s,   v19.4s,  v25.4s
        srshl           v4.4s,   v4.4s,   v29.4s  // (tmp1<<6 + (tmp2-tmp1)*(64-m) + (1 << (sh-1)) + PREP_BIAS*64) >> sh
        srshl           v5.4s,   v5.4s,   v29.4s
        srshl           v6.4s,   v6.4s,   v29.4s
        srshl           v7.4s,   v7.4s,   v29.4s
        sqxtun          v4.4h,   v4.4s            // iclip_pixel
        sqxtun2         v4.8h,   v5.4s
        sqxtun          v5.4h,   v6.4s
        sqxtun2         v5.8h,   v7.4s
        umin            v4.8h,   v4.8h,   v31.8h  // iclip_pixel
        umin            v5.8h,   v5.8h,   v31.8h
.if \type == 444
        uzp1            v20.16b, v20.16b, v21.16b // 64 - m
        sub             v20.16b, v1.16b,  v20.16b // m
        st1             {v20.16b}, [x6], #16
.elseif \type == 422
        addp            v20.8h,  v20.8h,  v21.8h  // (64 - m) + (64 - n) (column wise addition)
        xtn             v20.8b,  v20.8h
        uhsub           v20.8b,  v3.8b,   v20.8b  // ((129 - sign) - ((64 - m) + (64 - n)) >> 1
        st1             {v20.8b}, [x6], #8
.elseif \type == 420
        trn1            v24.2d,  v20.2d,  v21.2d
        trn2            v25.2d,  v20.2d,  v21.2d
        add             v24.8h,  v24.8h,  v25.8h  // (64 - my1) + (64 - my2) (row wise addition)
        addp            v20.8h,  v24.8h,  v24.8h  // (128 - m) + (128 - n) (column wise addition)
        sub             v20.4h,  v3.4h,   v20.4h  // (256 - sign) - ((128 - m) + (128 - n))
        rshrn           v20.8b,  v20.8h,  #2      // ((256 - sign) - ((128 - m) + (128 - n)) + 2) >> 2
        st1             {v20.s}[0], [x6], #4
.endif
        st1             {v4.d}[0],  [x0],  x1
        st1             {v4.d}[1],  [x12], x1
        st1             {v5.d}[0],  [x0],  x1
        st1             {v5.d}[1],  [x12], x1
        b.gt            4b
        ret
8:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v4.8h, v5.8h}, [x2], #32 // tmp1
        ld1             {v6.8h, v7.8h}, [x3], #32 // tmp2
        subs            w5,  w5,  #2
        sabd            v20.8h,  v4.8h,   v6.8h   // abs(tmp1 - tmp2)
        sabd            v21.8h,  v5.8h,   v7.8h
        ssubl           v16.4s,  v6.4h,   v4.4h   // tmp2 - tmp1 (requires 17 bit)
        ssubl2          v17.4s,  v6.8h,   v4.8h
        ssubl           v18.4s,  v7.4h,   v5.4h
        ssubl2          v19.4s,  v7.8h,   v5.8h
        uqsub           v20.8h,  v0.8h,   v20.8h  // 27615 - abs()
        uqsub           v21.8h,  v0.8h,   v21.8h
        sshll2          v7.4s,   v5.8h,   #6      // tmp1 << 6
        sshll           v6.4s,   v5.4h,   #6
        sshll2          v5.4s,   v4.8h,   #6
        sshll           v4.4s,   v4.4h,   #6
        ushr            v20.8h,  v20.8h,  #10     // 64-m = (27615 - abs()) >> mask_sh
        ushr            v21.8h,  v21.8h,  #10
        add             v4.4s,   v4.4s,   v30.4s  // += PREP_BIAS*64
        add             v5.4s,   v5.4s,   v30.4s
        add             v6.4s,   v6.4s,   v30.4s
        add             v7.4s,   v7.4s,   v30.4s
        uxtl            v22.4s,  v20.4h
        uxtl2           v23.4s,  v20.8h
        uxtl            v24.4s,  v21.4h
        uxtl2           v25.4s,  v21.8h
        mla             v4.4s,   v16.4s,  v22.4s  // (tmp2-tmp1)*(64-m)
        mla             v5.4s,   v17.4s,  v23.4s
        mla             v6.4s,   v18.4s,  v24.4s
        mla             v7.4s,   v19.4s,  v25.4s
        srshl           v4.4s,   v4.4s,   v29.4s  // (tmp1<<6 + (tmp2-tmp1)*(64-m) + (1 << (sh-1)) + PREP_BIAS*64) >> sh
        srshl           v5.4s,   v5.4s,   v29.4s
        srshl           v6.4s,   v6.4s,   v29.4s
        srshl           v7.4s,   v7.4s,   v29.4s
        sqxtun          v4.4h,   v4.4s            // iclip_pixel
        sqxtun2         v4.8h,   v5.4s
        sqxtun          v5.4h,   v6.4s
        sqxtun2         v5.8h,   v7.4s
        umin            v4.8h,   v4.8h,   v31.8h  // iclip_pixel
        umin            v5.8h,   v5.8h,   v31.8h
.if \type == 444
        uzp1            v20.16b, v20.16b, v21.16b // 64 - m
        sub             v20.16b, v1.16b,  v20.16b // m
        st1             {v20.16b}, [x6], #16
.elseif \type == 422
        addp            v20.8h,  v20.8h,  v21.8h  // (64 - m) + (64 - n) (column wise addition)
        xtn             v20.8b,  v20.8h
        uhsub           v20.8b,  v3.8b,   v20.8b  // ((129 - sign) - ((64 - m) + (64 - n)) >> 1
        st1             {v20.8b}, [x6], #8
.elseif \type == 420
        add             v20.8h,  v20.8h,  v21.8h  // (64 - my1) + (64 - my2) (row wise addition)
        addp            v20.8h,  v20.8h,  v20.8h  // (128 - m) + (128 - n) (column wise addition)
        sub             v20.4h,  v3.4h,   v20.4h  // (256 - sign) - ((128 - m) + (128 - n))
        rshrn           v20.8b,  v20.8h,  #2      // ((256 - sign) - ((128 - m) + (128 - n)) + 2) >> 2
        st1             {v20.s}[0], [x6], #4
.endif
        st1             {v4.8h}, [x0],  x1
        st1             {v5.8h}, [x12], x1
        b.gt            8b
        ret
1280:
640:
320:
160:
        AARCH64_VALID_JUMP_TARGET
        mov             w11, w4
        sub             x1,  x1,  w4,  uxtw #1
.if \type == 444
        add             x10, x6,  w4,  uxtw
.elseif \type == 422
        add             x10, x6,  x11, lsr #1
.endif
        add             x9,  x3,  w4,  uxtw #1
        add             x7,  x2,  w4,  uxtw #1
161:
        mov             w8,  w4
16:
        ld1             {v4.8h,   v5.8h},  [x2], #32 // tmp1
        ld1             {v16.8h,  v17.8h}, [x3], #32 // tmp2
        ld1             {v6.8h,   v7.8h},  [x7], #32
        ld1             {v18.8h,  v19.8h}, [x9], #32
        subs            w8,  w8,  #16
        sabd            v20.8h,  v4.8h,   v16.8h  // abs(tmp1 - tmp2)
        sabd            v21.8h,  v5.8h,   v17.8h
        ssubl           v22.4s,  v16.4h,  v4.4h   // tmp2 - tmp1 (requires 17 bit)
        ssubl2          v23.4s,  v16.8h,  v4.8h
        ssubl           v24.4s,  v17.4h,  v5.4h
        ssubl2          v25.4s,  v17.8h,  v5.8h
        uqsub           v20.8h,  v0.8h,   v20.8h  // 27615 - abs()
        uqsub           v21.8h,  v0.8h,   v21.8h
        sshll2          v27.4s,  v5.8h,   #6      // tmp1 << 6
        sshll           v26.4s,  v5.4h,   #6
        sshll2          v5.4s,   v4.8h,   #6
        sshll           v4.4s,   v4.4h,   #6
        ushr            v20.8h,  v20.8h,  #10     // 64-m = (27615 - abs()) >> mask_sh
        ushr            v21.8h,  v21.8h,  #10
        add             v4.4s,   v4.4s,   v30.4s  // += PREP_BIAS*64
        add             v5.4s,   v5.4s,   v30.4s
        add             v26.4s,  v26.4s,  v30.4s
        add             v27.4s,  v27.4s,  v30.4s
        uxtl            v16.4s,  v20.4h
        uxtl2           v17.4s,  v20.8h
        uxtl            v28.4s,  v21.4h
        mla             v4.4s,   v22.4s,  v16.4s  // (tmp2-tmp1)*(64-m)
        uxtl2           v16.4s,  v21.8h
        mla             v5.4s,   v23.4s,  v17.4s
        mla             v26.4s,  v24.4s,  v28.4s
        mla             v27.4s,  v25.4s,  v16.4s
        srshl           v4.4s,   v4.4s,   v29.4s  // (tmp1<<6 + (tmp2-tmp1)*(64-m) + (1 << (sh-1)) + PREP_BIAS*64) >> sh
        srshl           v5.4s,   v5.4s,   v29.4s
        srshl           v26.4s,  v26.4s,  v29.4s
        srshl           v27.4s,  v27.4s,  v29.4s
        sqxtun          v4.4h,   v4.4s            // iclip_pixel
        sqxtun2         v4.8h,   v5.4s
        sqxtun          v5.4h,   v26.4s
        sqxtun2         v5.8h,   v27.4s

        // Start of other half
        sabd            v22.8h,  v6.8h,   v18.8h  // abs(tmp1 - tmp2)
        sabd            v23.8h,  v7.8h,   v19.8h

        umin            v4.8h,   v4.8h,   v31.8h  // iclip_pixel
        umin            v5.8h,   v5.8h,   v31.8h

        ssubl           v16.4s,  v18.4h,  v6.4h   // tmp2 - tmp1 (requires 17 bit)
        ssubl2          v17.4s,  v18.8h,  v6.8h
        ssubl           v18.4s,  v19.4h,  v7.4h
        ssubl2          v19.4s,  v19.8h,  v7.8h
        uqsub           v22.8h,  v0.8h,   v22.8h  // 27615 - abs()
        uqsub           v23.8h,  v0.8h,   v23.8h
        sshll           v24.4s,  v6.4h,   #6      // tmp1 << 6
        sshll2          v25.4s,  v6.8h,   #6
        sshll           v26.4s,  v7.4h,   #6
        sshll2          v27.4s,  v7.8h,   #6
        ushr            v22.8h,  v22.8h,  #10     // 64-m = (27615 - abs()) >> mask_sh
        ushr            v23.8h,  v23.8h,  #10
        add             v24.4s,  v24.4s,  v30.4s  // += PREP_BIAS*64
        add             v25.4s,  v25.4s,  v30.4s
        add             v26.4s,  v26.4s,  v30.4s
        add             v27.4s,  v27.4s,  v30.4s
        uxtl            v6.4s,   v22.4h
        uxtl2           v7.4s,   v22.8h
        uxtl            v28.4s,  v23.4h
        mla             v24.4s,  v16.4s,  v6.4s   // (tmp2-tmp1)*(64-m)
        uxtl2           v6.4s,   v23.8h
        mla             v25.4s,  v17.4s,  v7.4s
        mla             v26.4s,  v18.4s,  v28.4s
        mla             v27.4s,  v19.4s,  v6.4s
        srshl           v24.4s,  v24.4s,  v29.4s  // (tmp1<<6 + (tmp2-tmp1)*(64-m) + (1 << (sh-1)) + PREP_BIAS*64) >> sh
        srshl           v25.4s,  v25.4s,  v29.4s
        srshl           v26.4s,  v26.4s,  v29.4s
        srshl           v27.4s,  v27.4s,  v29.4s
        sqxtun          v6.4h,   v24.4s           // iclip_pixel
        sqxtun2         v6.8h,   v25.4s
        sqxtun          v7.4h,   v26.4s
        sqxtun2         v7.8h,   v27.4s
        umin            v6.8h,   v6.8h,   v31.8h  // iclip_pixel
        umin            v7.8h,   v7.8h,   v31.8h
.if \type == 444
        uzp1            v20.16b, v20.16b, v21.16b // 64 - m
        uzp1            v21.16b, v22.16b, v23.16b
        sub             v20.16b, v1.16b,  v20.16b // m
        sub             v21.16b, v1.16b,  v21.16b
        st1             {v20.16b}, [x6],  #16
        st1             {v21.16b}, [x10], #16
.elseif \type == 422
        addp            v20.8h,  v20.8h,  v21.8h  // (64 - m) + (64 - n) (column wise addition)
        addp            v21.8h,  v22.8h,  v23.8h
        xtn             v20.8b,  v20.8h
        xtn             v21.8b,  v21.8h
        uhsub           v20.8b,  v3.8b,   v20.8b  // ((129 - sign) - ((64 - m) + (64 - n)) >> 1
        uhsub           v21.8b,  v3.8b,   v21.8b
        st1             {v20.8b}, [x6],  #8
        st1             {v21.8b}, [x10], #8
.elseif \type == 420
        add             v20.8h,  v20.8h,  v22.8h  // (64 - my1) + (64 - my2) (row wise addition)
        add             v21.8h,  v21.8h,  v23.8h
        addp            v20.8h,  v20.8h,  v21.8h  // (128 - m) + (128 - n) (column wise addition)
        sub             v20.8h,  v3.8h,   v20.8h  // (256 - sign) - ((128 - m) + (128 - n))
        rshrn           v20.8b,  v20.8h,  #2      // ((256 - sign) - ((128 - m) + (128 - n)) + 2) >> 2
        st1             {v20.8b}, [x6], #8
.endif
        st1             {v4.8h, v5.8h}, [x0],  #32
        st1             {v6.8h, v7.8h}, [x12], #32
        b.gt            16b
        subs            w5,  w5,  #2
        add             x2,  x2,  w4,  uxtw #1
        add             x3,  x3,  w4,  uxtw #1
        add             x7,  x7,  w4,  uxtw #1
        add             x9,  x9,  w4,  uxtw #1
.if \type == 444
        add             x6,  x6,  w4,  uxtw
        add             x10, x10, w4,  uxtw
.elseif \type == 422
        add             x6,  x6,  x11, lsr #1
        add             x10, x10, x11, lsr #1
.endif
        add             x0,  x0,  x1
        add             x12, x12, x1
        b.gt            161b
        ret
L(w_mask_\type\()_tbl):
        .hword L(w_mask_\type\()_tbl) - 1280b
        .hword L(w_mask_\type\()_tbl) -  640b
        .hword L(w_mask_\type\()_tbl) -  320b
        .hword L(w_mask_\type\()_tbl) -  160b
        .hword L(w_mask_\type\()_tbl) -    8b
        .hword L(w_mask_\type\()_tbl) -    4b
endfunc
.endm

w_mask_fn 444
w_mask_fn 422
w_mask_fn 420


function blend_16bpc_neon, export=1
        adr             x6,  L(blend_tbl)
        clz             w3,  w3
        sub             w3,  w3,  #26
        ldrh            w3,  [x6,  x3,  lsl #1]
        sub             x6,  x6,  w3,  uxtw
        add             x8,  x0,  x1
        br              x6
40:
        AARCH64_VALID_JUMP_TARGET
        lsl             x1,  x1,  #1
4:
        ld1             {v2.8b},   [x5], #8
        ld1             {v1.8h},   [x2], #16
        ld1             {v0.d}[0], [x0]
        neg             v2.8b,   v2.8b            // -m
        subs            w4,  w4,  #2
        ld1             {v0.d}[1], [x8]
        sxtl            v2.8h,   v2.8b
        shl             v2.8h,   v2.8h,   #9      // -m << 9
        sub             v1.8h,   v0.8h,   v1.8h   // a - b
        sqrdmulh        v1.8h,   v1.8h,   v2.8h   // ((a-b)*-m + 32) >> 6
        add             v0.8h,   v0.8h,   v1.8h
        st1             {v0.d}[0], [x0], x1
        st1             {v0.d}[1], [x8], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        lsl             x1,  x1,  #1
8:
        ld1             {v4.16b},       [x5], #16
        ld1             {v2.8h, v3.8h}, [x2], #32
        neg             v5.16b,  v4.16b           // -m
        ld1             {v0.8h},   [x0]
        ld1             {v1.8h},   [x8]
        sxtl            v4.8h,   v5.8b
        sxtl2           v5.8h,   v5.16b
        shl             v4.8h,   v4.8h,   #9      // -m << 9
        shl             v5.8h,   v5.8h,   #9
        sub             v2.8h,   v0.8h,   v2.8h   // a - b
        sub             v3.8h,   v1.8h,   v3.8h
        subs            w4,  w4,  #2
        sqrdmulh        v2.8h,   v2.8h,   v4.8h   // ((a-b)*-m + 32) >> 6
        sqrdmulh        v3.8h,   v3.8h,   v5.8h
        add             v0.8h,   v0.8h,   v2.8h
        add             v1.8h,   v1.8h,   v3.8h
        st1             {v0.8h}, [x0], x1
        st1             {v1.8h}, [x8], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        lsl             x1,  x1,  #1
16:
        ld1             {v16.16b, v17.16b},           [x5], #32
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], #64
        subs            w4,  w4,  #2
        neg             v18.16b, v16.16b          // -m
        neg             v19.16b, v17.16b
        ld1             {v0.8h, v1.8h}, [x0]
        sxtl            v16.8h,  v18.8b
        sxtl2           v17.8h,  v18.16b
        sxtl            v18.8h,  v19.8b
        sxtl2           v19.8h,  v19.16b
        ld1             {v2.8h, v3.8h}, [x8]
        shl             v16.8h,  v16.8h,  #9      // -m << 9
        shl             v17.8h,  v17.8h,  #9
        shl             v18.8h,  v18.8h,  #9
        shl             v19.8h,  v19.8h,  #9
        sub             v4.8h,   v0.8h,   v4.8h   // a - b
        sub             v5.8h,   v1.8h,   v5.8h
        sub             v6.8h,   v2.8h,   v6.8h
        sub             v7.8h,   v3.8h,   v7.8h
        sqrdmulh        v4.8h,   v4.8h,   v16.8h  // ((a-b)*-m + 32) >> 6
        sqrdmulh        v5.8h,   v5.8h,   v17.8h
        sqrdmulh        v6.8h,   v6.8h,   v18.8h
        sqrdmulh        v7.8h,   v7.8h,   v19.8h
        add             v0.8h,   v0.8h,   v4.8h
        add             v1.8h,   v1.8h,   v5.8h
        add             v2.8h,   v2.8h,   v6.8h
        add             v3.8h,   v3.8h,   v7.8h
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v2.8h, v3.8h}, [x8], x1
        b.gt            16b
        ret
32:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v16.16b, v17.16b},           [x5], #32
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], #64
        subs            w4,  w4,  #1
        neg             v18.16b, v16.16b          // -m
        neg             v19.16b, v17.16b
        sxtl            v16.8h,  v18.8b
        sxtl2           v17.8h,  v18.16b
        sxtl            v18.8h,  v19.8b
        sxtl2           v19.8h,  v19.16b
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0]
        shl             v16.8h,  v16.8h,  #9      // -m << 9
        shl             v17.8h,  v17.8h,  #9
        shl             v18.8h,  v18.8h,  #9
        shl             v19.8h,  v19.8h,  #9
        sub             v4.8h,   v0.8h,   v4.8h   // a - b
        sub             v5.8h,   v1.8h,   v5.8h
        sub             v6.8h,   v2.8h,   v6.8h
        sub             v7.8h,   v3.8h,   v7.8h
        sqrdmulh        v4.8h,   v4.8h,   v16.8h  // ((a-b)*-m + 32) >> 6
        sqrdmulh        v5.8h,   v5.8h,   v17.8h
        sqrdmulh        v6.8h,   v6.8h,   v18.8h
        sqrdmulh        v7.8h,   v7.8h,   v19.8h
        add             v0.8h,   v0.8h,   v4.8h
        add             v1.8h,   v1.8h,   v5.8h
        add             v2.8h,   v2.8h,   v6.8h
        add             v3.8h,   v3.8h,   v7.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        b.gt            32b
        ret
L(blend_tbl):
        .hword L(blend_tbl) -  32b
        .hword L(blend_tbl) - 160b
        .hword L(blend_tbl) -  80b
        .hword L(blend_tbl) -  40b
endfunc

function blend_h_16bpc_neon, export=1
        adr             x6,  L(blend_h_tbl)
        movrel          x5,  X(obmc_masks)
        add             x5,  x5,  w4,  uxtw
        sub             w4,  w4,  w4,  lsr #2
        clz             w7,  w3
        add             x8,  x0,  x1
        lsl             x1,  x1,  #1
        sub             w7,  w7,  #24
        ldrh            w7,  [x6,  x7,  lsl #1]
        sub             x6,  x6,  w7, uxtw
        br              x6
2:
        AARCH64_VALID_JUMP_TARGET
        ld2r            {v2.8b, v3.8b}, [x5], #2
        ld1             {v1.4h},        [x2], #8
        ext             v2.8b,   v2.8b,   v3.8b,   #6
        subs            w4,  w4,  #2
        neg             v2.8b,   v2.8b            // -m
        ld1             {v0.s}[0], [x0]
        ld1             {v0.s}[1], [x8]
        sxtl            v2.8h,   v2.8b
        shl             v2.4h,   v2.4h,   #9      // -m << 9
        sub             v1.4h,   v0.4h,   v1.4h   // a - b
        sqrdmulh        v1.4h,   v1.4h,   v2.4h   // ((a-b)*-m + 32) >> 6
        add             v0.4h,   v0.4h,   v1.4h
        st1             {v0.s}[0], [x0], x1
        st1             {v0.s}[1], [x8], x1
        b.gt            2b
        ret
4:
        AARCH64_VALID_JUMP_TARGET
        ld2r            {v2.8b, v3.8b}, [x5], #2
        ld1             {v1.8h},        [x2], #16
        ext             v2.8b,   v2.8b,   v3.8b,   #4
        subs            w4,  w4,  #2
        neg             v2.8b,   v2.8b            // -m
        ld1             {v0.d}[0],   [x0]
        ld1             {v0.d}[1],   [x8]
        sxtl            v2.8h,   v2.8b
        shl             v2.8h,   v2.8h,   #9      // -m << 9
        sub             v1.8h,   v0.8h,   v1.8h   // a - b
        sqrdmulh        v1.8h,   v1.8h,   v2.8h   // ((a-b)*-m + 32) >> 6
        add             v0.8h,   v0.8h,   v1.8h
        st1             {v0.d}[0], [x0], x1
        st1             {v0.d}[1], [x8], x1
        b.gt            4b
        ret
8:
        AARCH64_VALID_JUMP_TARGET
        ld2r            {v4.8b, v5.8b}, [x5], #2
        ld1             {v2.8h, v3.8h}, [x2], #32
        neg             v4.8b,   v4.8b            // -m
        neg             v5.8b,   v5.8b
        ld1             {v0.8h}, [x0]
        subs            w4,  w4,  #2
        sxtl            v4.8h,   v4.8b
        sxtl            v5.8h,   v5.8b
        ld1             {v1.8h}, [x8]
        shl             v4.8h,   v4.8h,   #9      // -m << 9
        shl             v5.8h,   v5.8h,   #9
        sub             v2.8h,   v0.8h,   v2.8h   // a - b
        sub             v3.8h,   v1.8h,   v3.8h
        sqrdmulh        v2.8h,   v2.8h,   v4.8h   // ((a-b)*-m + 32) >> 6
        sqrdmulh        v3.8h,   v3.8h,   v5.8h
        add             v0.8h,   v0.8h,   v2.8h
        add             v1.8h,   v1.8h,   v3.8h
        st1             {v0.8h}, [x0], x1
        st1             {v1.8h}, [x8], x1
        b.gt            8b
        ret
16:
        AARCH64_VALID_JUMP_TARGET
        ld2r            {v16.8b, v17.8b}, [x5], #2
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], #64
        neg             v16.8b,  v16.8b           // -m
        neg             v17.8b,  v17.8b
        ld1             {v0.8h, v1.8h},  [x0]
        ld1             {v2.8h, v3.8h},  [x8]
        subs            w4,  w4,  #2
        sxtl            v16.8h,  v16.8b
        sxtl            v17.8h,  v17.8b
        shl             v16.8h,  v16.8h,  #9      // -m << 9
        shl             v17.8h,  v17.8h,  #9
        sub             v4.8h,   v0.8h,   v4.8h   // a - b
        sub             v5.8h,   v1.8h,   v5.8h
        sub             v6.8h,   v2.8h,   v6.8h
        sub             v7.8h,   v3.8h,   v7.8h
        sqrdmulh        v4.8h,   v4.8h,   v16.8h  // ((a-b)*-m + 32) >> 6
        sqrdmulh        v5.8h,   v5.8h,   v16.8h
        sqrdmulh        v6.8h,   v6.8h,   v17.8h
        sqrdmulh        v7.8h,   v7.8h,   v17.8h
        add             v0.8h,   v0.8h,   v4.8h
        add             v1.8h,   v1.8h,   v5.8h
        add             v2.8h,   v2.8h,   v6.8h
        add             v3.8h,   v3.8h,   v7.8h
        st1             {v0.8h, v1.8h}, [x0], x1
        st1             {v2.8h, v3.8h}, [x8], x1
        b.gt            16b
        ret
1280:
640:
320:
        AARCH64_VALID_JUMP_TARGET
        sub             x1,  x1,  w3,  uxtw #1
        add             x7,  x2,  w3,  uxtw #1
321:
        ld2r            {v24.8b, v25.8b}, [x5], #2
        mov             w6,  w3
        neg             v24.8b,  v24.8b           // -m
        neg             v25.8b,  v25.8b
        sxtl            v24.8h,  v24.8b
        sxtl            v25.8h,  v25.8b
        shl             v24.8h,  v24.8h,  #9      // -m << 9
        shl             v25.8h,  v25.8h,  #9
32:
        ld1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x2], #64
        ld1             {v0.8h,  v1.8h,  v2.8h,  v3.8h},  [x0]
        subs            w6,  w6,  #32
        sub             v16.8h,  v0.8h,   v16.8h  // a - b
        sub             v17.8h,  v1.8h,   v17.8h
        sub             v18.8h,  v2.8h,   v18.8h
        sub             v19.8h,  v3.8h,   v19.8h
        ld1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x7], #64
        ld1             {v4.8h,  v5.8h,  v6.8h,  v7.8h},  [x8]
        sqrdmulh        v16.8h,  v16.8h,  v24.8h  // ((a-b)*-m + 32) >> 6
        sqrdmulh        v17.8h,  v17.8h,  v24.8h
        sqrdmulh        v18.8h,  v18.8h,  v24.8h
        sqrdmulh        v19.8h,  v19.8h,  v24.8h
        sub             v20.8h,  v4.8h,   v20.8h  // a - b
        sub             v21.8h,  v5.8h,   v21.8h
        sub             v22.8h,  v6.8h,   v22.8h
        sub             v23.8h,  v7.8h,   v23.8h
        add             v0.8h,   v0.8h,   v16.8h
        add             v1.8h,   v1.8h,   v17.8h
        add             v2.8h,   v2.8h,   v18.8h
        add             v3.8h,   v3.8h,   v19.8h
        sqrdmulh        v20.8h,  v20.8h,  v25.8h  // ((a-b)*-m + 32) >> 6
        sqrdmulh        v21.8h,  v21.8h,  v25.8h
        sqrdmulh        v22.8h,  v22.8h,  v25.8h
        sqrdmulh        v23.8h,  v23.8h,  v25.8h
        st1             {v0.8h,  v1.8h,  v2.8h,  v3.8h},  [x0], #64
        add             v4.8h,   v4.8h,   v20.8h
        add             v5.8h,   v5.8h,   v21.8h
        add             v6.8h,   v6.8h,   v22.8h
        add             v7.8h,   v7.8h,   v23.8h
        st1             {v4.8h,  v5.8h,  v6.8h,  v7.8h},  [x8], #64
        b.gt            32b
        subs            w4,  w4,  #2
        add             x0,  x0,  x1
        add             x8,  x8,  x1
        add             x2,  x2,  w3,  uxtw #1
        add             x7,  x7,  w3,  uxtw #1
        b.gt            321b
        ret
L(blend_h_tbl):
        .hword L(blend_h_tbl) - 1280b
        .hword L(blend_h_tbl) -  640b
        .hword L(blend_h_tbl) -  320b
        .hword L(blend_h_tbl) -   16b
        .hword L(blend_h_tbl) -    8b
        .hword L(blend_h_tbl) -    4b
        .hword L(blend_h_tbl) -    2b
endfunc

function blend_v_16bpc_neon, export=1
        adr             x6,  L(blend_v_tbl)
        movrel          x5,  X(obmc_masks)
        add             x5,  x5,  w3,  uxtw
        clz             w3,  w3
        add             x8,  x0,  x1
        lsl             x1,  x1,  #1
        sub             w3,  w3,  #26
        ldrh            w3,  [x6,  x3,  lsl #1]
        sub             x6,  x6,  w3,  uxtw
        br              x6
20:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v2.8b}, [x5]
        neg             v2.8b,   v2.8b            // -m
        sxtl            v2.8h,   v2.8b
        shl             v2.4h,   v2.4h,   #9      // -m << 9
2:
        ld1             {v1.s}[0], [x2], #4
        ld1             {v0.h}[0], [x0]
        subs            w4,  w4,  #2
        ld1             {v1.h}[1], [x2]
        ld1             {v0.h}[1], [x8]
        add             x2,  x2,  #4
        sub             v1.4h,   v0.4h,   v1.4h   // a - b
        sqrdmulh        v1.4h,   v1.4h,   v2.4h   // ((a-b)*-m + 32) >> 6
        add             v0.4h,   v0.4h,   v1.4h
        st1             {v0.h}[0], [x0],  x1
        st1             {v0.h}[1], [x8],  x1
        b.gt            2b
        ret
40:
        AARCH64_VALID_JUMP_TARGET
        ld1r            {v2.2s}, [x5]
        sub             x1,  x1,  #4
        neg             v2.8b,   v2.8b            // -m
        sxtl            v2.8h,   v2.8b
        shl             v2.8h,   v2.8h,   #9      // -m << 9
4:
        ld1             {v1.8h},   [x2], #16
        ld1             {v0.d}[0], [x0]
        ld1             {v0.d}[1], [x8]
        subs            w4,  w4,  #2
        sub             v1.8h,   v0.8h,   v1.8h   // a - b
        sqrdmulh        v1.8h,   v1.8h,   v2.8h   // ((a-b)*-m + 32) >> 6
        add             v0.8h,   v0.8h,   v1.8h
        st1             {v0.s}[0], [x0], #4
        st1             {v0.s}[2], [x8], #4
        st1             {v0.h}[2], [x0], x1
        st1             {v0.h}[6], [x8], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v4.8b}, [x5]
        sub             x1,  x1,  #8
        neg             v4.8b,   v4.8b            // -m
        sxtl            v4.8h,   v4.8b
        shl             v4.8h,   v4.8h,   #9      // -m << 9
8:
        ld1             {v2.8h, v3.8h}, [x2], #32
        ld1             {v0.8h}, [x0]
        ld1             {v1.8h}, [x8]
        subs            w4,  w4,  #2
        sub             v2.8h,   v0.8h,   v2.8h   // a - b
        sub             v3.8h,   v1.8h,   v3.8h
        sqrdmulh        v2.8h,   v2.8h,   v4.8h   // ((a-b)*-m + 32) >> 6
        sqrdmulh        v3.8h,   v3.8h,   v4.8h
        add             v0.8h,   v0.8h,   v2.8h
        add             v1.8h,   v1.8h,   v3.8h
        st1             {v0.d}[0], [x0], #8
        st1             {v1.d}[0], [x8], #8
        st1             {v0.s}[2], [x0], x1
        st1             {v1.s}[2], [x8], x1
        b.gt            8b
        ret
160:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v16.16b}, [x5]
        sub             x1,  x1,  #16
        neg             v17.16b, v16.16b          // -m
        sxtl            v16.8h,  v17.8b
        sxtl2           v17.8h,  v17.16b
        shl             v16.8h,  v16.8h,  #9      // -m << 9
        shl             v17.4h,  v17.4h,  #9
16:
        ld1             {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], #64
        ld1             {v0.8h, v1.8h}, [x0]
        subs            w4,  w4,  #2
        ld1             {v2.8h, v3.8h}, [x8]
        sub             v4.8h,   v0.8h,   v4.8h   // a - b
        sub             v5.4h,   v1.4h,   v5.4h
        sub             v6.8h,   v2.8h,   v6.8h
        sub             v7.4h,   v3.4h,   v7.4h
        sqrdmulh        v4.8h,   v4.8h,   v16.8h  // ((a-b)*-m + 32) >> 6
        sqrdmulh        v5.4h,   v5.4h,   v17.4h
        sqrdmulh        v6.8h,   v6.8h,   v16.8h
        sqrdmulh        v7.4h,   v7.4h,   v17.4h
        add             v0.8h,   v0.8h,   v4.8h
        add             v1.4h,   v1.4h,   v5.4h
        add             v2.8h,   v2.8h,   v6.8h
        add             v3.4h,   v3.4h,   v7.4h
        st1             {v0.8h}, [x0], #16
        st1             {v2.8h}, [x8], #16
        st1             {v1.4h}, [x0], x1
        st1             {v3.4h}, [x8], x1
        b.gt            16b
        ret
320:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v24.16b, v25.16b},  [x5]
        neg             v26.16b, v24.16b          // -m
        neg             v27.8b,  v25.8b
        sxtl            v24.8h,  v26.8b
        sxtl2           v25.8h,  v26.16b
        sxtl            v26.8h,  v27.8b
        shl             v24.8h,  v24.8h,  #9      // -m << 9
        shl             v25.8h,  v25.8h,  #9
        shl             v26.8h,  v26.8h,  #9
32:
        ld1             {v16.8h, v17.8h, v18.8h, v19.8h}, [x2], #64
        ld1             {v0.8h, v1.8h, v2.8h}, [x0]
        ld1             {v20.8h, v21.8h, v22.8h, v23.8h}, [x2], #64
        ld1             {v4.8h, v5.8h, v6.8h}, [x8]
        subs            w4,  w4,  #2
        sub             v16.8h,  v0.8h,   v16.8h  // a - b
        sub             v17.8h,  v1.8h,   v17.8h
        sub             v18.8h,  v2.8h,   v18.8h
        sub             v20.8h,  v4.8h,   v20.8h
        sub             v21.8h,  v5.8h,   v21.8h
        sub             v22.8h,  v6.8h,   v22.8h
        sqrdmulh        v16.8h,  v16.8h,  v24.8h  // ((a-b)*-m + 32) >> 6
        sqrdmulh        v17.8h,  v17.8h,  v25.8h
        sqrdmulh        v18.8h,  v18.8h,  v26.8h
        sqrdmulh        v20.8h,  v20.8h,  v24.8h
        sqrdmulh        v21.8h,  v21.8h,  v25.8h
        sqrdmulh        v22.8h,  v22.8h,  v26.8h
        add             v0.8h,   v0.8h,   v16.8h
        add             v1.8h,   v1.8h,   v17.8h
        add             v2.8h,   v2.8h,   v18.8h
        add             v4.8h,   v4.8h,   v20.8h
        add             v5.8h,   v5.8h,   v21.8h
        add             v6.8h,   v6.8h,   v22.8h
        st1             {v0.8h, v1.8h, v2.8h}, [x0], x1
        st1             {v4.8h, v5.8h, v6.8h}, [x8], x1
        b.gt            32b
        ret
L(blend_v_tbl):
        .hword L(blend_v_tbl) - 320b
        .hword L(blend_v_tbl) - 160b
        .hword L(blend_v_tbl) -  80b
        .hword L(blend_v_tbl) -  40b
        .hword L(blend_v_tbl) -  20b
endfunc


// This has got the same signature as the put_8tap functions,
// and assumes that x9 is set to (clz(w)-24).
function put_neon
        adr             x10, L(put_tbl)
        ldrh            w9, [x10, x9, lsl #1]
        sub             x10, x10, w9, uxtw
        br              x10

2:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.s}[0], [x2], x3
        ld1             {v1.s}[0], [x2], x3
        subs            w5,  w5,  #2
        st1             {v0.s}[0], [x0], x1
        st1             {v1.s}[0], [x0], x1
        b.gt            2b
        ret
4:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.4h}, [x2], x3
        ld1             {v1.4h}, [x2], x3
        subs            w5,  w5,  #2
        st1             {v0.4h}, [x0], x1
        st1             {v1.4h}, [x0], x1
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        add             x8,  x0,  x1
        lsl             x1,  x1,  #1
        add             x9,  x2,  x3
        lsl             x3,  x3,  #1
8:
        ld1             {v0.8h}, [x2], x3
        ld1             {v1.8h}, [x9], x3
        subs            w5,  w5,  #2
        st1             {v0.8h}, [x0], x1
        st1             {v1.8h}, [x8], x1
        b.gt            8b
        ret
16:
        AARCH64_VALID_JUMP_TARGET
        ldp             x6,  x7,  [x2]
        ldp             x8,  x9,  [x2, #16]
        stp             x6,  x7,  [x0]
        subs            w5,  w5,  #1
        stp             x8,  x9,  [x0, #16]
        add             x2,  x2,  x3
        add             x0,  x0,  x1
        b.gt            16b
        ret
32:
        AARCH64_VALID_JUMP_TARGET
        ldp             x6,  x7,  [x2]
        ldp             x8,  x9,  [x2, #16]
        stp             x6,  x7,  [x0]
        ldp             x10, x11, [x2, #32]
        stp             x8,  x9,  [x0, #16]
        subs            w5,  w5,  #1
        ldp             x12, x13, [x2, #48]
        stp             x10, x11, [x0, #32]
        stp             x12, x13, [x0, #48]
        add             x2,  x2,  x3
        add             x0,  x0,  x1
        b.gt            32b
        ret
64:
        AARCH64_VALID_JUMP_TARGET
        ldp             q0,  q1,  [x2]
        ldp             q2,  q3,  [x2, #32]
        stp             q0,  q1,  [x0]
        ldp             q4,  q5,  [x2, #64]
        stp             q2,  q3,  [x0, #32]
        ldp             q6,  q7,  [x2, #96]
        subs            w5,  w5,  #1
        stp             q4,  q5,  [x0, #64]
        stp             q6,  q7,  [x0, #96]
        add             x2,  x2,  x3
        add             x0,  x0,  x1
        b.gt            64b
        ret
128:
        AARCH64_VALID_JUMP_TARGET
        ldp             q0,  q1,  [x2]
        ldp             q2,  q3,  [x2, #32]
        stp             q0,  q1,  [x0]
        ldp             q4,  q5,  [x2, #64]
        stp             q2,  q3,  [x0, #32]
        ldp             q6,  q7,  [x2, #96]
        subs            w5,  w5,  #1
        stp             q4,  q5,  [x0, #64]
        ldp             q16, q17, [x2, #128]
        stp             q6,  q7,  [x0, #96]
        ldp             q18, q19, [x2, #160]
        stp             q16, q17, [x0, #128]
        ldp             q20, q21, [x2, #192]
        stp             q18, q19, [x0, #160]
        ldp             q22, q23, [x2, #224]
        stp             q20, q21, [x0, #192]
        stp             q22, q23, [x0, #224]
        add             x2,  x2,  x3
        add             x0,  x0,  x1
        b.gt            128b
        ret

L(put_tbl):
        .hword L(put_tbl) - 128b
        .hword L(put_tbl) -  64b
        .hword L(put_tbl) -  32b
        .hword L(put_tbl) -  16b
        .hword L(put_tbl) -  80b
        .hword L(put_tbl) -   4b
        .hword L(put_tbl) -   2b
endfunc


// This has got the same signature as the prep_8tap functions,
// and assumes that x9 is set to (clz(w)-24), w7 to intermediate_bits and
// x8 to w*2.
function prep_neon
        adr             x10, L(prep_tbl)
        ldrh            w9, [x10, x9, lsl #1]
        dup             v31.8h,  w7   // intermediate_bits
        movi            v30.8h,  #(PREP_BIAS >> 8), lsl #8
        sub             x10, x10, w9, uxtw
        br              x10

40:
        AARCH64_VALID_JUMP_TARGET
        add             x9,  x1,  x2
        lsl             x2,  x2,  #1
4:
        ld1             {v0.d}[0], [x1], x2
        ld1             {v0.d}[1], [x9], x2
        subs            w4,  w4,  #2
        sshl            v0.8h,   v0.8h,   v31.8h
        sub             v0.8h,   v0.8h,   v30.8h
        st1             {v0.8h}, [x0], #16
        b.gt            4b
        ret
80:
        AARCH64_VALID_JUMP_TARGET
        add             x9,  x1,  x2
        lsl             x2,  x2,  #1
8:
        ld1             {v0.8h}, [x1], x2
        ld1             {v1.8h}, [x9], x2
        subs            w4,  w4,  #2
        sshl            v0.8h,   v0.8h,   v31.8h
        sshl            v1.8h,   v1.8h,   v31.8h
        sub             v0.8h,   v0.8h,   v30.8h
        sub             v1.8h,   v1.8h,   v30.8h
        st1             {v0.8h, v1.8h}, [x0], #32
        b.gt            8b
        ret
16:
        AARCH64_VALID_JUMP_TARGET
        ldp             q0,  q1,  [x1]
        add             x1,  x1,  x2
        sshl            v0.8h,   v0.8h,   v31.8h
        ldp             q2,  q3,  [x1]
        add             x1,  x1,  x2
        subs            w4,  w4,  #2
        sshl            v1.8h,   v1.8h,   v31.8h
        sshl            v2.8h,   v2.8h,   v31.8h
        sshl            v3.8h,   v3.8h,   v31.8h
        sub             v0.8h,   v0.8h,   v30.8h
        sub             v1.8h,   v1.8h,   v30.8h
        sub             v2.8h,   v2.8h,   v30.8h
        sub             v3.8h,   v3.8h,   v30.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        b.gt            16b
        ret
32:
        AARCH64_VALID_JUMP_TARGET
        ldp             q0,  q1,  [x1]
        sshl            v0.8h,   v0.8h,   v31.8h
        ldp             q2,  q3,  [x1, #32]
        add             x1,  x1,  x2
        sshl            v1.8h,   v1.8h,   v31.8h
        sshl            v2.8h,   v2.8h,   v31.8h
        sshl            v3.8h,   v3.8h,   v31.8h
        subs            w4,  w4,  #1
        sub             v0.8h,   v0.8h,   v30.8h
        sub             v1.8h,   v1.8h,   v30.8h
        sub             v2.8h,   v2.8h,   v30.8h
        sub             v3.8h,   v3.8h,   v30.8h
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        b.gt            32b
        ret
64:
        AARCH64_VALID_JUMP_TARGET
        ldp             q0,  q1,  [x1]
        subs            w4,  w4,  #1
        sshl            v0.8h,   v0.8h,   v31.8h
        ldp             q2,  q3,  [x1, #32]
        sshl            v1.8h,   v1.8h,   v31.8h
        ldp             q4,  q5,  [x1, #64]
        sshl            v2.8h,   v2.8h,   v31.8h
        sshl            v3.8h,   v3.8h,   v31.8h
        ldp             q6,  q7,  [x1, #96]
        add             x1,  x1,  x2
        sshl            v4.8h,   v4.8h,   v31.8h
        sshl            v5.8h,   v5.8h,   v31.8h
        sshl            v6.8h,   v6.8h,   v31.8h
        sshl            v7.8h,   v7.8h,   v31.8h
        sub             v0.8h,   v0.8h,   v30.8h
        sub             v1.8h,   v1.8h,   v30.8h
        sub             v2.8h,   v2.8h,   v30.8h
        sub             v3.8h,   v3.8h,   v30.8h
        stp             q0,  q1,  [x0]
        sub             v4.8h,   v4.8h,   v30.8h
        sub             v5.8h,   v5.8h,   v30.8h
        stp             q2,  q3,  [x0, #32]
        sub             v6.8h,   v6.8h,   v30.8h
        sub             v7.8h,   v7.8h,   v30.8h
        stp             q4,  q5,  [x0, #64]
        stp             q6,  q7,  [x0, #96]
        add             x0,  x0,  x8
        b.gt            64b
        ret
128:
        AARCH64_VALID_JUMP_TARGET
        ldp             q0,  q1,  [x1]
        subs            w4,  w4,  #1
        sshl            v0.8h,   v0.8h,   v31.8h
        ldp             q2,  q3,  [x1, #32]
        sshl            v1.8h,   v1.8h,   v31.8h
        ldp             q4,  q5,  [x1, #64]
        sshl            v2.8h,   v2.8h,   v31.8h
        sshl            v3.8h,   v3.8h,   v31.8h
        ldp             q6,  q7,  [x1, #96]
        sshl            v4.8h,   v4.8h,   v31.8h
        sshl            v5.8h,   v5.8h,   v31.8h
        ldp             q16, q17, [x1, #128]
        sshl            v6.8h,   v6.8h,   v31.8h
        sshl            v7.8h,   v7.8h,   v31.8h
        ldp             q18, q19, [x1, #160]
        sshl            v16.8h,  v16.8h,  v31.8h
        sshl            v17.8h,  v17.8h,  v31.8h
        ldp             q20, q21, [x1, #192]
        sshl            v18.8h,  v18.8h,  v31.8h
        sshl            v19.8h,  v19.8h,  v31.8h
        ldp             q22, q23, [x1, #224]
        add             x1,  x1,  x2
        sshl            v20.8h,  v20.8h,  v31.8h
        sshl            v21.8h,  v21.8h,  v31.8h
        sshl            v22.8h,  v22.8h,  v31.8h
        sshl            v23.8h,  v23.8h,  v31.8h
        sub             v0.8h,   v0.8h,   v30.8h
        sub             v1.8h,   v1.8h,   v30.8h
        sub             v2.8h,   v2.8h,   v30.8h
        sub             v3.8h,   v3.8h,   v30.8h
        stp             q0,  q1,  [x0]
        sub             v4.8h,   v4.8h,   v30.8h
        sub             v5.8h,   v5.8h,   v30.8h
        stp             q2,  q3,  [x0, #32]
        sub             v6.8h,   v6.8h,   v30.8h
        sub             v7.8h,   v7.8h,   v30.8h
        stp             q4,  q5,  [x0, #64]
        sub             v16.8h,  v16.8h,  v30.8h
        sub             v17.8h,  v17.8h,  v30.8h
        stp             q6,  q7,  [x0, #96]
        sub             v18.8h,  v18.8h,  v30.8h
        sub             v19.8h,  v19.8h,  v30.8h
        stp             q16, q17, [x0, #128]
        sub             v20.8h,  v20.8h,  v30.8h
        sub             v21.8h,  v21.8h,  v30.8h
        stp             q18, q19, [x0, #160]
        sub             v22.8h,  v22.8h,  v30.8h
        sub             v23.8h,  v23.8h,  v30.8h
        stp             q20, q21, [x0, #192]
        stp             q22, q23, [x0, #224]
        add             x0,  x0,  x8
        b.gt            128b
        ret

L(prep_tbl):
        .hword L(prep_tbl) - 128b
        .hword L(prep_tbl) -  64b
        .hword L(prep_tbl) -  32b
        .hword L(prep_tbl) -  16b
        .hword L(prep_tbl) -  80b
        .hword L(prep_tbl) -  40b
endfunc


.macro load_slice s0, s1, strd, wd, d0, d1, d2, d3, d4, d5, d6
        ld1             {\d0\wd}[0], [\s0], \strd
        ld1             {\d1\wd}[0], [\s1], \strd
.ifnb \d2
        ld1             {\d2\wd}[0], [\s0], \strd
        ld1             {\d3\wd}[0], [\s1], \strd
.endif
.ifnb \d4
        ld1             {\d4\wd}[0], [\s0], \strd
.endif
.ifnb \d5
        ld1             {\d5\wd}[0], [\s1], \strd
.endif
.ifnb \d6
        ld1             {\d6\wd}[0], [\s0], \strd
.endif
.endm
.macro load_reg s0, s1, strd, wd, d0, d1, d2, d3, d4, d5, d6
        ld1             {\d0\wd}, [\s0], \strd
        ld1             {\d1\wd}, [\s1], \strd
.ifnb \d2
        ld1             {\d2\wd}, [\s0], \strd
        ld1             {\d3\wd}, [\s1], \strd
.endif
.ifnb \d4
        ld1             {\d4\wd}, [\s0], \strd
.endif
.ifnb \d5
        ld1             {\d5\wd}, [\s1], \strd
.endif
.ifnb \d6
        ld1             {\d6\wd}, [\s0], \strd
.endif
.endm
.macro load_regpair s0, s1, strd, wd, d0, d1, d2, d3, d4, d5
        ld1             {\d0\wd, \d1\wd}, [\s0], \strd
.ifnb \d2
        ld1             {\d2\wd, \d3\wd}, [\s1], \strd
.endif
.ifnb \d4
        ld1             {\d4\wd, \d5\wd}, [\s0], \strd
.endif
.endm
.macro load_s s0, s1, strd, d0, d1, d2, d3, d4, d5, d6
        load_slice      \s0, \s1, \strd, .s, \d0, \d1, \d2, \d3, \d4, \d5, \d6
.endm
.macro load_4h s0, s1, strd, d0, d1, d2, d3, d4, d5, d6
        load_reg        \s0, \s1, \strd, .4h, \d0, \d1, \d2, \d3, \d4, \d5, \d6
.endm
.macro load_8h s0, s1, strd, d0, d1, d2, d3, d4, d5, d6
        load_reg        \s0, \s1, \strd, .8h, \d0, \d1, \d2, \d3, \d4, \d5, \d6
.endm
.macro load_16h s0, s1, strd, d0, d1, d2, d3, d4, d5
        load_regpair    \s0, \s1, \strd, .8h, \d0, \d1, \d2, \d3, \d4, \d5
.endm
.macro interleave_1 wd, r0, r1, r2, r3, r4
        trn1            \r0\wd, \r0\wd, \r1\wd
        trn1            \r1\wd, \r1\wd, \r2\wd
.ifnb \r3
        trn1            \r2\wd, \r2\wd, \r3\wd
        trn1            \r3\wd, \r3\wd, \r4\wd
.endif
.endm
.macro interleave_1_s r0, r1, r2, r3, r4
        interleave_1    .2s, \r0, \r1, \r2, \r3, \r4
.endm
.macro umin_h c, wd, r0, r1, r2, r3
        umin            \r0\wd,  \r0\wd,  \c\wd
.ifnb \r1
        umin            \r1\wd,  \r1\wd,  \c\wd
.endif
.ifnb \r2
        umin            \r2\wd,  \r2\wd,  \c\wd
        umin            \r3\wd,  \r3\wd,  \c\wd
.endif
.endm
.macro sub_h c, wd, r0, r1, r2, r3
        sub             \r0\wd,  \r0\wd,  \c\wd
.ifnb \r1
        sub             \r1\wd,  \r1\wd,  \c\wd
.endif
.ifnb \r2
        sub             \r2\wd,  \r2\wd,  \c\wd
        sub             \r3\wd,  \r3\wd,  \c\wd
.endif
.endm
.macro smull_smlal_4 d, s0, s1, s2, s3
        smull           \d\().4s,  \s0\().4h,  v0.h[0]
        smlal           \d\().4s,  \s1\().4h,  v0.h[1]
        smlal           \d\().4s,  \s2\().4h,  v0.h[2]
        smlal           \d\().4s,  \s3\().4h,  v0.h[3]
.endm
.macro smull2_smlal2_4 d, s0, s1, s2, s3
        smull2          \d\().4s,  \s0\().8h,  v0.h[0]
        smlal2          \d\().4s,  \s1\().8h,  v0.h[1]
        smlal2          \d\().4s,  \s2\().8h,  v0.h[2]
        smlal2          \d\().4s,  \s3\().8h,  v0.h[3]
.endm
.macro smull_smlal_8 d, s0, s1, s2, s3, s4, s5, s6, s7
        smull           \d\().4s,  \s0\().4h,  v0.h[0]
        smlal           \d\().4s,  \s1\().4h,  v0.h[1]
        smlal           \d\().4s,  \s2\().4h,  v0.h[2]
        smlal           \d\().4s,  \s3\().4h,  v0.h[3]
        smlal           \d\().4s,  \s4\().4h,  v0.h[4]
        smlal           \d\().4s,  \s5\().4h,  v0.h[5]
        smlal           \d\().4s,  \s6\().4h,  v0.h[6]
        smlal           \d\().4s,  \s7\().4h,  v0.h[7]
.endm
.macro smull2_smlal2_8 d, s0, s1, s2, s3, s4, s5, s6, s7
        smull2          \d\().4s,  \s0\().8h,  v0.h[0]
        smlal2          \d\().4s,  \s1\().8h,  v0.h[1]
        smlal2          \d\().4s,  \s2\().8h,  v0.h[2]
        smlal2          \d\().4s,  \s3\().8h,  v0.h[3]
        smlal2          \d\().4s,  \s4\().8h,  v0.h[4]
        smlal2          \d\().4s,  \s5\().8h,  v0.h[5]
        smlal2          \d\().4s,  \s6\().8h,  v0.h[6]
        smlal2          \d\().4s,  \s7\().8h,  v0.h[7]
.endm
.macro sqrshrun_h shift, r0, r1, r2, r3
        sqrshrun        \r0\().4h, \r0\().4s,  #\shift
.ifnb \r1
        sqrshrun2       \r0\().8h, \r1\().4s,  #\shift
.endif
.ifnb \r2
        sqrshrun        \r2\().4h, \r2\().4s,  #\shift
        sqrshrun2       \r2\().8h, \r3\().4s,  #\shift
.endif
.endm
.macro xtn_h r0, r1, r2, r3
        uzp1            \r0\().8h,  \r0\().8h,  \r1\().8h // Same as xtn, xtn2
.ifnb \r2
        uzp1            \r2\().8h,  \r2\().8h,  \r3\().8h // Ditto
.endif
.endm
.macro srshl_s shift, r0, r1, r2, r3
        srshl           \r0\().4s,  \r0\().4s,  \shift\().4s
        srshl           \r1\().4s,  \r1\().4s,  \shift\().4s
.ifnb \r2
        srshl           \r2\().4s,  \r2\().4s,  \shift\().4s
        srshl           \r3\().4s,  \r3\().4s,  \shift\().4s
.endif
.endm
.macro st_s strd, reg, lanes
        st1             {\reg\().s}[0], [x0], \strd
        st1             {\reg\().s}[1], [x9], \strd
.if \lanes > 2
        st1             {\reg\().s}[2], [x0], \strd
        st1             {\reg\().s}[3], [x9], \strd
.endif
.endm
.macro st_d strd, r0, r1
        st1             {\r0\().d}[0], [x0], \strd
        st1             {\r0\().d}[1], [x9], \strd
.ifnb \r1
        st1             {\r1\().d}[0], [x0], \strd
        st1             {\r1\().d}[1], [x9], \strd
.endif
.endm
.macro shift_store_4 type, strd, r0, r1, r2, r3
.ifc \type, put
        sqrshrun_h      6,   \r0, \r1, \r2, \r3
        umin_h          v31, .8h, \r0, \r2
.else
        srshl_s         v30, \r0, \r1, \r2, \r3  // -(6-intermediate_bits)
        xtn_h           \r0, \r1, \r2, \r3
        sub_h           v29, .8h, \r0, \r2       // PREP_BIAS
.endif
        st_d            \strd, \r0, \r2
.endm
.macro st_reg strd, wd, r0, r1, r2, r3, r4, r5, r6, r7
        st1             {\r0\wd}, [x0], \strd
        st1             {\r1\wd}, [x9], \strd
.ifnb \r2
        st1             {\r2\wd}, [x0], \strd
        st1             {\r3\wd}, [x9], \strd
.endif
.ifnb \r4
        st1             {\r4\wd}, [x0], \strd
        st1             {\r5\wd}, [x9], \strd
        st1             {\r6\wd}, [x0], \strd
        st1             {\r7\wd}, [x9], \strd
.endif
.endm
.macro st_8h strd, r0, r1, r2, r3, r4, r5, r6, r7
        st_reg          \strd, .8h, \r0, \r1, \r2, \r3, \r4, \r5, \r6, \r7
.endm
.macro shift_store_8 type, strd, r0, r1, r2, r3
.ifc \type, put
        sqrshrun_h      6,   \r0, \r1, \r2, \r3
        umin_h          v31, .8h, \r0, \r2
.else
        srshl_s         v30, \r0, \r1, \r2, \r3  // -(6-intermediate_bits)
        xtn_h           \r0, \r1, \r2, \r3
        sub_h           v29, .8h, \r0, \r2       // PREP_BIAS
.endif
        st_8h           \strd, \r0, \r2
.endm
.macro shift_store_16 type, strd, dst, r0, r1, r2, r3
.ifc \type, put
        sqrshrun_h      6,   \r0, \r1, \r2, \r3
        umin            \r0\().8h, \r0\().8h, v31.8h
        umin            \r1\().8h, \r2\().8h, v31.8h
.else
        srshl_s         v30, \r0, \r1, \r2, \r3  // -(6-intermediate_bits)
        xtn_h           \r0, \r1, \r2, \r3
        sub             \r0\().8h, \r0\().8h, v29.8h
        sub             \r1\().8h, \r2\().8h, v29.8h
.endif
        st1             {\r0\().8h, \r1\().8h}, [\dst], \strd
.endm

.macro make_8tap_fn op, type, type_h, type_v
function \op\()_8tap_\type\()_16bpc_neon, export=1
        mov             w9,  \type_h
        mov             w10, \type_v
        b               \op\()_8tap_neon
endfunc
.endm

// No spaces in these expressions, due to gas-preprocessor.
#define REGULAR ((0*15<<7)|3*15)
#define SMOOTH  ((1*15<<7)|4*15)
#define SHARP   ((2*15<<7)|3*15)

.macro filter_fn type, dst, d_strd, src, s_strd, w, h, mx, xmx, my, xmy, bdmax, ds2, sr2
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
.ifc \bdmax, w8
        ldr             w8,  [sp]
.endif
        mov             w11,  #0x4081  // (1 << 14) | (1 << 7) | (1 << 0)
        mul             \mx,  \mx, w11
        mul             \my,  \my, w11
        add             \mx,  \mx, w9  // mx, 8tap_h, 4tap_h
        add             \my,  \my, w10 // my, 8tap_v, 4tap_v
.ifc \type, prep
        uxtw            \d_strd, \w
        lsl             \d_strd, \d_strd, #1
.endif

        dup             v31.8h,  \bdmax        // bitdepth_max
        clz             \bdmax,  \bdmax
        clz             w9,  \w
        sub             \bdmax,  \bdmax,  #18  // intermediate_bits = clz(bitdepth_max) - 18
        mov             w12, #6
        tst             \mx, #(0x7f << 14)
        sub             w9,  w9,  #24
        add             w13, w12, \bdmax       // 6 + intermediate_bits
        sub             w12, w12, \bdmax       // 6 - intermediate_bits
        movrel          x11, X(mc_subpel_filters), -8
        b.ne            L(\type\()_8tap_h)
        tst             \my, #(0x7f << 14)
        b.ne            L(\type\()_8tap_v)
        b               \type\()_neon

L(\type\()_8tap_h):
        cmp             \w,   #4
        ubfx            w10,  \mx, #7, #7
        and             \mx,  \mx, #0x7f
        b.le            4f
        mov             \mx,  w10
4:
        tst             \my,  #(0x7f << 14)
        add             \xmx, x11, \mx, uxtw #3
        b.ne            L(\type\()_8tap_hv)

        adr             x10, L(\type\()_8tap_h_tbl)
        dup             v30.4s,  w12           // 6 - intermediate_bits
        ldrh            w9,  [x10, x9, lsl #1]
        neg             v30.4s,  v30.4s        // -(6-intermediate_bits)
.ifc \type, put
        dup             v29.8h,  \bdmax        // intermediate_bits
.else
        movi            v28.8h,  #(PREP_BIAS >> 8), lsl #8
.endif
        sub             x10, x10, w9, uxtw
.ifc \type, put
        neg             v29.8h,  v29.8h        // -intermediate_bits
.endif
        br              x10

20:     // 2xN h
        AARCH64_VALID_JUMP_TARGET
.ifc \type, put
        add             \xmx,  \xmx,  #2
        ld1             {v0.s}[0], [\xmx]
        sub             \src,  \src,  #2
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
        sxtl            v0.8h,   v0.8b
2:
        ld1             {v4.8h},  [\src], \s_strd
        ld1             {v6.8h},  [\sr2], \s_strd
        ext             v5.16b,  v4.16b,  v4.16b,  #2
        ext             v7.16b,  v6.16b,  v6.16b,  #2
        subs            \h,  \h,  #2
        trn1            v3.2s,   v4.2s,   v6.2s
        trn2            v6.2s,   v4.2s,   v6.2s
        trn1            v4.2s,   v5.2s,   v7.2s
        trn2            v7.2s,   v5.2s,   v7.2s
        smull           v3.4s,   v3.4h,   v0.h[0]
        smlal           v3.4s,   v4.4h,   v0.h[1]
        smlal           v3.4s,   v6.4h,   v0.h[2]
        smlal           v3.4s,   v7.4h,   v0.h[3]
        srshl           v3.4s,   v3.4s,   v30.4s // -(6-intermediate_bits)
        sqxtun          v3.4h,   v3.4s
        srshl           v3.4h,   v3.4h,   v29.4h // -intermediate_bits
        umin            v3.4h,   v3.4h,   v31.4h
        st1             {v3.s}[0], [\dst], \d_strd
        st1             {v3.s}[1], [\ds2], \d_strd
        b.gt            2b
        ret
.endif

40:     // 4xN h
        AARCH64_VALID_JUMP_TARGET
        add             \xmx,  \xmx,  #2
        ld1             {v0.s}[0], [\xmx]
        sub             \src,  \src,  #2
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
        sxtl            v0.8h,   v0.8b
4:
        ld1             {v16.8h}, [\src], \s_strd
        ld1             {v20.8h}, [\sr2], \s_strd
        ext             v17.16b, v16.16b, v16.16b, #2
        ext             v18.16b, v16.16b, v16.16b, #4
        ext             v19.16b, v16.16b, v16.16b, #6
        ext             v21.16b, v20.16b, v20.16b, #2
        ext             v22.16b, v20.16b, v20.16b, #4
        ext             v23.16b, v20.16b, v20.16b, #6
        subs            \h,  \h,  #2
        smull           v16.4s,  v16.4h,  v0.h[0]
        smlal           v16.4s,  v17.4h,  v0.h[1]
        smlal           v16.4s,  v18.4h,  v0.h[2]
        smlal           v16.4s,  v19.4h,  v0.h[3]
        smull           v20.4s,  v20.4h,  v0.h[0]
        smlal           v20.4s,  v21.4h,  v0.h[1]
        smlal           v20.4s,  v22.4h,  v0.h[2]
        smlal           v20.4s,  v23.4h,  v0.h[3]
        srshl           v16.4s,  v16.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v20.4s,  v20.4s,  v30.4s // -(6-intermediate_bits)
.ifc \type, put
        sqxtun          v16.4h,  v16.4s
        sqxtun2         v16.8h,  v20.4s
        srshl           v16.8h,  v16.8h,  v29.8h // -intermediate_bits
        umin            v16.8h,  v16.8h,  v31.8h
.else
        uzp1            v16.8h,  v16.8h,  v20.8h // Same as xtn, xtn2
        sub             v16.8h,  v16.8h,  v28.8h // PREP_BIAS
.endif
        st1             {v16.d}[0], [\dst], \d_strd
        st1             {v16.d}[1], [\ds2], \d_strd
        b.gt            4b
        ret

80:
160:
320:
640:
1280:   // 8xN, 16xN, 32xN, ... h
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b}, [\xmx]
        sub             \src,  \src,  #6
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        sxtl            v0.8h,   v0.8b

        sub             \s_strd,  \s_strd,  \w, uxtw #1
        sub             \s_strd,  \s_strd,  #16
.ifc \type, put
        lsl             \d_strd,  \d_strd,  #1
        sub             \d_strd,  \d_strd,  \w, uxtw #1
.endif
81:
        ld1             {v16.8h, v17.8h},  [\src], #32
        ld1             {v20.8h, v21.8h},  [\sr2], #32
        mov             \mx, \w

8:
        smull           v18.4s,  v16.4h,  v0.h[0]
        smull2          v19.4s,  v16.8h,  v0.h[0]
        smull           v22.4s,  v20.4h,  v0.h[0]
        smull2          v23.4s,  v20.8h,  v0.h[0]
.irpc i, 1234567
        ext             v24.16b, v16.16b, v17.16b, #(2*\i)
        ext             v25.16b, v20.16b, v21.16b, #(2*\i)
        smlal           v18.4s,  v24.4h,  v0.h[\i]
        smlal2          v19.4s,  v24.8h,  v0.h[\i]
        smlal           v22.4s,  v25.4h,  v0.h[\i]
        smlal2          v23.4s,  v25.8h,  v0.h[\i]
.endr
        subs            \mx, \mx, #8
        srshl           v18.4s,  v18.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v19.4s,  v19.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v22.4s,  v22.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v23.4s,  v23.4s,  v30.4s // -(6-intermediate_bits)
.ifc \type, put
        sqxtun          v18.4h,  v18.4s
        sqxtun2         v18.8h,  v19.4s
        sqxtun          v22.4h,  v22.4s
        sqxtun2         v22.8h,  v23.4s
        srshl           v18.8h,  v18.8h,  v29.8h // -intermediate_bits
        srshl           v22.8h,  v22.8h,  v29.8h // -intermediate_bits
        umin            v18.8h,  v18.8h,  v31.8h
        umin            v22.8h,  v22.8h,  v31.8h
.else
        uzp1            v18.8h,  v18.8h,  v19.8h // Same as xtn, xtn2
        uzp1            v22.8h,  v22.8h,  v23.8h // Ditto
        sub             v18.8h,  v18.8h,  v28.8h // PREP_BIAS
        sub             v22.8h,  v22.8h,  v28.8h // PREP_BIAS
.endif
        st1             {v18.8h}, [\dst], #16
        st1             {v22.8h}, [\ds2], #16
        b.le            9f

        mov             v16.16b, v17.16b
        mov             v20.16b, v21.16b
        ld1             {v17.8h}, [\src], #16
        ld1             {v21.8h}, [\sr2], #16
        b               8b

9:
        add             \dst,  \dst,  \d_strd
        add             \ds2,  \ds2,  \d_strd
        add             \src,  \src,  \s_strd
        add             \sr2,  \sr2,  \s_strd

        subs            \h,  \h,  #2
        b.gt            81b
        ret

L(\type\()_8tap_h_tbl):
        .hword L(\type\()_8tap_h_tbl) - 1280b
        .hword L(\type\()_8tap_h_tbl) -  640b
        .hword L(\type\()_8tap_h_tbl) -  320b
        .hword L(\type\()_8tap_h_tbl) -  160b
        .hword L(\type\()_8tap_h_tbl) -   80b
        .hword L(\type\()_8tap_h_tbl) -   40b
        .hword L(\type\()_8tap_h_tbl) -   20b
        .hword 0


L(\type\()_8tap_v):
        cmp             \h,  #4
        ubfx            w10, \my, #7, #7
        and             \my, \my, #0x7f
        b.le            4f
        mov             \my, w10
4:
        add             \xmy, x11, \my, uxtw #3

.ifc \type, prep
        dup             v30.4s,  w12           // 6 - intermediate_bits
        movi            v29.8h,  #(PREP_BIAS >> 8), lsl #8
.endif
        adr             x10, L(\type\()_8tap_v_tbl)
        ldrh            w9,  [x10, x9, lsl #1]
.ifc \type, prep
        neg             v30.4s,  v30.4s        // -(6-intermediate_bits)
.endif
        sub             x10, x10, w9, uxtw
        br              x10

20:     // 2xN v
        AARCH64_VALID_JUMP_TARGET
.ifc \type, put
        b.gt            28f

        cmp             \h,  #2
        add             \xmy, \xmy, #2
        ld1             {v0.s}[0], [\xmy]
        sub             \src,  \src,  \s_strd
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1
        sxtl            v0.8h,   v0.8b

        // 2x2 v
        load_s          \src, \sr2, \s_strd, v1, v2, v3, v4, v5
        interleave_1_s  v1,  v2,  v3,  v4,  v5
        b.gt            24f
        smull_smlal_4   v6,  v1,  v2,  v3,  v4
        sqrshrun_h      6,   v6
        umin_h          v31, .8h, v6
        st_s            \d_strd, v6, 2
        ret

24:     // 2x4 v
        load_s          \sr2, \src, \s_strd, v6, v7
        interleave_1_s  v5,  v6,  v7
        smull_smlal_4   v16, v1,  v2,  v3,  v4
        smull_smlal_4   v17, v3,  v4,  v5,  v6
        sqrshrun_h      6,   v16, v17
        umin_h          v31, .8h, v16
        st_s            \d_strd, v16, 4
        ret

28:     // 2x6, 2x8, 2x12, 2x16 v
        ld1             {v0.8b}, [\xmy]
        sub             \sr2,  \src,  \s_strd, lsl #1
        add             \ds2,  \dst,  \d_strd
        sub             \src,  \sr2,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
        sxtl            v0.8h,   v0.8b

        load_s          \src, \sr2, \s_strd, v1,  v2,  v3,  v4, v5, v6, v7
        interleave_1_s  v1,  v2,  v3,  v4,  v5
        interleave_1_s  v5,  v6,  v7
216:
        subs            \h,  \h,  #4
        load_s          \sr2, \src, \s_strd, v16, v17, v18, v19
        interleave_1_s  v7,  v16, v17, v18, v19
        smull_smlal_8   v24, v1,  v2,  v3,  v4,  v5,  v6,  v7,  v16
        smull_smlal_8   v25, v3,  v4,  v5,  v6,  v7,  v16, v17, v18
        sqrshrun_h      6,   v24, v25
        umin_h          v31, .8h, v24
        st_s            \d_strd, v24, 4
        b.le            0f
        cmp             \h,  #2
        mov             v1.16b,  v5.16b
        mov             v2.16b,  v6.16b
        mov             v3.16b,  v7.16b
        mov             v4.16b,  v16.16b
        mov             v5.16b,  v17.16b
        mov             v6.16b,  v18.16b
        mov             v7.16b,  v19.16b
        b.eq            26f
        b               216b
26:
        load_s          \sr2, \src, \s_strd, v16, v17
        interleave_1_s  v7,  v16, v17
        smull_smlal_8   v24, v1,  v2,  v3,  v4,  v5,  v6,  v7,  v16
        sqrshrun_h      6,   v24
        umin_h          v31, .4h, v24
        st_s            \d_strd, v24, 2
0:
        ret
.endif

40:
        AARCH64_VALID_JUMP_TARGET
        b.gt            480f

        // 4x2, 4x4 v
        cmp             \h,  #2
        add             \xmy, \xmy, #2
        ld1             {v0.s}[0], [\xmy]
        sub             \src, \src, \s_strd
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        sxtl            v0.8h,   v0.8b

        load_4h         \src, \sr2, \s_strd, v1, v2, v3, v4, v5
        smull_smlal_4   v6,  v1,  v2,  v3,  v4
        smull_smlal_4   v7,  v2,  v3,  v4,  v5
        shift_store_4   \type, \d_strd, v6, v7
        b.le            0f
        load_4h         \sr2, \src, \s_strd, v6, v7
        smull_smlal_4   v1,  v3,  v4,  v5,  v6
        smull_smlal_4   v2,  v4,  v5,  v6,  v7
        shift_store_4   \type, \d_strd, v1, v2
0:
        ret

480:    // 4x6, 4x8, 4x12, 4x16 v
        ld1             {v0.8b}, [\xmy]
        sub             \sr2, \src, \s_strd, lsl #1
        add             \ds2, \dst, \d_strd
        sub             \src, \sr2, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        sxtl            v0.8h,   v0.8b

        load_4h         \src, \sr2, \s_strd, v16, v17, v18, v19, v20, v21, v22

48:
        subs            \h,  \h,  #4
        load_4h         \sr2, \src, \s_strd, v23, v24, v25, v26
        smull_smlal_8   v1,  v16, v17, v18, v19, v20, v21, v22, v23
        smull_smlal_8   v2,  v17, v18, v19, v20, v21, v22, v23, v24
        smull_smlal_8   v3,  v18, v19, v20, v21, v22, v23, v24, v25
        smull_smlal_8   v4,  v19, v20, v21, v22, v23, v24, v25, v26
        shift_store_4   \type, \d_strd, v1, v2, v3, v4
        b.le            0f
        cmp             \h,  #2
        mov             v16.8b,  v20.8b
        mov             v17.8b,  v21.8b
        mov             v18.8b,  v22.8b
        mov             v19.8b,  v23.8b
        mov             v20.8b,  v24.8b
        mov             v21.8b,  v25.8b
        mov             v22.8b,  v26.8b
        b.eq            46f
        b               48b
46:
        load_4h         \sr2, \src, \s_strd, v23, v24
        smull_smlal_8   v1,  v16, v17, v18, v19, v20, v21, v22, v23
        smull_smlal_8   v2,  v17, v18, v19, v20, v21, v22, v23, v24
        shift_store_4   \type, \d_strd, v1, v2
0:
        ret

80:
        AARCH64_VALID_JUMP_TARGET
        b.gt            880f

        // 8x2, 8x4 v
        cmp             \h,  #2
        add             \xmy, \xmy, #2
        ld1             {v0.s}[0], [\xmy]
        sub             \src, \src, \s_strd
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        sxtl            v0.8h,   v0.8b

        load_8h         \src, \sr2, \s_strd, v1, v2, v3, v4, v5
        smull_smlal_4   v16, v1,  v2,  v3,  v4
        smull2_smlal2_4 v17, v1,  v2,  v3,  v4
        smull_smlal_4   v18, v2,  v3,  v4,  v5
        smull2_smlal2_4 v19, v2,  v3,  v4,  v5
        shift_store_8   \type, \d_strd, v16, v17, v18, v19
        b.le            0f
        load_8h         \sr2, \src, \s_strd, v6, v7
        smull_smlal_4   v16, v3,  v4,  v5,  v6
        smull2_smlal2_4 v17, v3,  v4,  v5,  v6
        smull_smlal_4   v18, v4,  v5,  v6,  v7
        smull2_smlal2_4 v19, v4,  v5,  v6,  v7
        shift_store_8   \type, \d_strd, v16, v17, v18, v19
0:
        ret

880:    // 8x6, 8x8, 8x16, 8x32 v
1680:   // 16x8, 16x16, ...
320:    // 32x8, 32x16, ...
640:
1280:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b}, [\xmy]
        sub             \src, \src, \s_strd
        sub             \src, \src, \s_strd, lsl #1
        sxtl            v0.8h,   v0.8b
        mov             \my,  \h
168:
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        load_8h         \src, \sr2, \s_strd, v16, v17, v18, v19, v20, v21, v22

88:
        subs            \h,  \h,  #2
        load_8h         \sr2, \src, \s_strd, v23, v24
        smull_smlal_8   v1,  v16, v17, v18, v19, v20, v21, v22, v23
        smull2_smlal2_8 v2,  v16, v17, v18, v19, v20, v21, v22, v23
        smull_smlal_8   v3,  v17, v18, v19, v20, v21, v22, v23, v24
        smull2_smlal2_8 v4,  v17, v18, v19, v20, v21, v22, v23, v24
        shift_store_8   \type, \d_strd, v1, v2, v3, v4
        b.le            9f
        subs            \h,  \h,  #2
        load_8h         \sr2, \src, \s_strd, v25, v26
        smull_smlal_8   v1,  v18, v19, v20, v21, v22, v23, v24, v25
        smull2_smlal2_8 v2,  v18, v19, v20, v21, v22, v23, v24, v25
        smull_smlal_8   v3,  v19, v20, v21, v22, v23, v24, v25, v26
        smull2_smlal2_8 v4,  v19, v20, v21, v22, v23, v24, v25, v26
        shift_store_8   \type, \d_strd, v1, v2, v3, v4
        b.le            9f
        mov             v16.16b, v20.16b
        mov             v17.16b, v21.16b
        mov             v18.16b, v22.16b
        mov             v19.16b, v23.16b
        mov             v20.16b, v24.16b
        mov             v21.16b, v25.16b
        mov             v22.16b, v26.16b
        b               88b
9:
        subs            \w,  \w,  #8
        b.le            0f
        asr             \s_strd, \s_strd, #1
        asr             \d_strd, \d_strd, #1
        msub            \src, \s_strd, \xmy, \src
        msub            \dst, \d_strd, \xmy, \dst
        sub             \src, \src, \s_strd, lsl #3
        mov             \h,  \my
        add             \src, \src, #16
        add             \dst, \dst, #16
        b               168b
0:
        ret

160:
        AARCH64_VALID_JUMP_TARGET
        b.gt            1680b

        // 16x2, 16x4 v
        add             \xmy, \xmy, #2
        ld1             {v0.s}[0], [\xmy]
        sub             \src, \src, \s_strd
        sxtl            v0.8h,   v0.8b

        load_16h        \src, \src, \s_strd, v16, v17, v18, v19, v20, v21
16:
        load_16h        \src, \src, \s_strd, v22, v23
        subs            \h,  \h,  #1
        smull_smlal_4   v1,  v16, v18, v20, v22
        smull2_smlal2_4 v2,  v16, v18, v20, v22
        smull_smlal_4   v3,  v17, v19, v21, v23
        smull2_smlal2_4 v4,  v17, v19, v21, v23
        shift_store_16  \type, \d_strd, x0, v1, v2, v3, v4
        b.le            0f
        mov             v16.16b, v18.16b
        mov             v17.16b, v19.16b
        mov             v18.16b, v20.16b
        mov             v19.16b, v21.16b
        mov             v20.16b, v22.16b
        mov             v21.16b, v23.16b
        b               16b
0:
        ret

L(\type\()_8tap_v_tbl):
        .hword L(\type\()_8tap_v_tbl) - 1280b
        .hword L(\type\()_8tap_v_tbl) -  640b
        .hword L(\type\()_8tap_v_tbl) -  320b
        .hword L(\type\()_8tap_v_tbl) -  160b
        .hword L(\type\()_8tap_v_tbl) -   80b
        .hword L(\type\()_8tap_v_tbl) -   40b
        .hword L(\type\()_8tap_v_tbl) -   20b
        .hword 0

L(\type\()_8tap_hv):
        cmp             \h,  #4
        ubfx            w10, \my, #7, #7
        and             \my, \my, #0x7f
        b.le            4f
        mov             \my,  w10
4:
        add             \xmy, x11, \my, uxtw #3

        adr             x10, L(\type\()_8tap_hv_tbl)
        dup             v30.4s,  w12           // 6 - intermediate_bits
        ldrh            w9,  [x10, x9, lsl #1]
        neg             v30.4s,  v30.4s        // -(6-intermediate_bits)
.ifc \type, put
        dup             v29.4s,  w13           // 6 + intermediate_bits
.else
        movi            v29.8h,  #(PREP_BIAS >> 8), lsl #8
.endif
        sub             x10, x10, w9, uxtw
.ifc \type, put
        neg             v29.4s,  v29.4s        // -(6+intermediate_bits)
.endif
        br              x10

20:
        AARCH64_VALID_JUMP_TARGET
.ifc \type, put
        add             \xmx,  \xmx,  #2
        ld1             {v0.s}[0],  [\xmx]
        b.gt            280f
        add             \xmy,  \xmy,  #2
        ld1             {v1.s}[0],  [\xmy]

        // 2x2, 2x4 hv
        sub             \sr2, \src, #2
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        sxtl            v0.8h,   v0.8b
        sxtl            v1.8h,   v1.8b
        mov             x15, x30

        ld1             {v27.8h}, [\src], \s_strd
        ext             v28.16b, v27.16b, v27.16b, #2
        smull           v27.4s,  v27.4h,  v0.4h
        smull           v28.4s,  v28.4h,  v0.4h
        addp            v27.4s,  v27.4s,  v28.4s
        addp            v16.4s,  v27.4s,  v27.4s
        srshl           v16.2s,  v16.2s,  v30.2s // -(6-intermediate_bits)
        bl              L(\type\()_8tap_filter_2)
        // The intermediates from the horizontal pass fit in 16 bit without
        // any bias; we could just as well keep them as .4s, but narrowing
        // them to .4h gives a significant speedup on out of order cores
        // (at the cost of a smaller slowdown on in-order cores such as A53).
        xtn             v16.4h,  v16.4s

        trn1            v16.2s,  v16.2s,  v24.2s
        mov             v17.8b,  v24.8b

2:
        bl              L(\type\()_8tap_filter_2)

        ext             v18.8b,  v17.8b,  v24.8b,  #4
        smull           v2.4s,   v16.4h,  v1.h[0]
        smlal           v2.4s,   v17.4h,  v1.h[1]
        smlal           v2.4s,   v18.4h,  v1.h[2]
        smlal           v2.4s,   v24.4h,  v1.h[3]

        srshl           v2.4s,   v2.4s,   v29.4s // -(6+intermediate_bits)
        sqxtun          v2.4h,   v2.4s
        umin            v2.4h,   v2.4h,   v31.4h
        subs            \h,  \h,  #2
        st1             {v2.s}[0], [\dst], \d_strd
        st1             {v2.s}[1], [\ds2], \d_strd
        b.le            0f
        mov             v16.8b,  v18.8b
        mov             v17.8b,  v24.8b
        b               2b

280:    // 2x8, 2x16, 2x32 hv
        ld1             {v1.8b},  [\xmy]
        sub             \src, \src, #2
        sub             \sr2, \src, \s_strd, lsl #1
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        sxtl            v0.8h,   v0.8b
        sxtl            v1.8h,   v1.8b
        mov             x15, x30

        ld1             {v27.8h}, [\src], \s_strd
        ext             v28.16b, v27.16b, v27.16b, #2
        smull           v27.4s,  v27.4h,  v0.4h
        smull           v28.4s,  v28.4h,  v0.4h
        addp            v27.4s,  v27.4s,  v28.4s
        addp            v16.4s,  v27.4s,  v27.4s
        srshl           v16.2s,  v16.2s,  v30.2s // -(6-intermediate_bits)
        // The intermediates from the horizontal pass fit in 16 bit without
        // any bias; we could just as well keep them as .4s, but narrowing
        // them to .4h gives a significant speedup on out of order cores
        // (at the cost of a smaller slowdown on in-order cores such as A53).

        bl              L(\type\()_8tap_filter_2)
        xtn             v16.4h,  v16.4s
        trn1            v16.2s,  v16.2s,  v24.2s
        mov             v17.8b,  v24.8b
        bl              L(\type\()_8tap_filter_2)
        ext             v18.8b,  v17.8b,  v24.8b,  #4
        mov             v19.8b,  v24.8b
        bl              L(\type\()_8tap_filter_2)
        ext             v20.8b,  v19.8b,  v24.8b,  #4
        mov             v21.8b,  v24.8b

28:
        bl              L(\type\()_8tap_filter_2)
        ext             v22.8b,  v21.8b,  v24.8b,  #4
        smull           v3.4s,   v16.4h,  v1.h[0]
        smlal           v3.4s,   v17.4h,  v1.h[1]
        smlal           v3.4s,   v18.4h,  v1.h[2]
        smlal           v3.4s,   v19.4h,  v1.h[3]
        smlal           v3.4s,   v20.4h,  v1.h[4]
        smlal           v3.4s,   v21.4h,  v1.h[5]
        smlal           v3.4s,   v22.4h,  v1.h[6]
        smlal           v3.4s,   v24.4h,  v1.h[7]

        srshl           v3.4s,   v3.4s,   v29.4s // -(6+intermediate_bits)
        sqxtun          v3.4h,   v3.4s
        umin            v3.4h,   v3.4h,   v31.4h
        subs            \h,  \h,  #2
        st1             {v3.s}[0], [\dst], \d_strd
        st1             {v3.s}[1], [\ds2], \d_strd
        b.le            0f
        mov             v16.8b,  v18.8b
        mov             v17.8b,  v19.8b
        mov             v18.8b,  v20.8b
        mov             v19.8b,  v21.8b
        mov             v20.8b,  v22.8b
        mov             v21.8b,  v24.8b
        b               28b

0:
        ret             x15

L(\type\()_8tap_filter_2):
        ld1             {v25.8h},  [\sr2], \s_strd
        ld1             {v27.8h},  [\src], \s_strd
        ext             v26.16b, v25.16b, v25.16b, #2
        ext             v28.16b, v27.16b, v27.16b, #2
        trn1            v24.2s,  v25.2s,  v27.2s
        trn2            v27.2s,  v25.2s,  v27.2s
        trn1            v25.2s,  v26.2s,  v28.2s
        trn2            v28.2s,  v26.2s,  v28.2s
        smull           v24.4s,  v24.4h,  v0.h[0]
        smlal           v24.4s,  v25.4h,  v0.h[1]
        smlal           v24.4s,  v27.4h,  v0.h[2]
        smlal           v24.4s,  v28.4h,  v0.h[3]
        srshl           v24.4s,  v24.4s,  v30.4s // -(6-intermediate_bits)
        xtn             v24.4h,  v24.4s
        ret
.endif

40:
        AARCH64_VALID_JUMP_TARGET
        add             \xmx, \xmx, #2
        ld1             {v0.s}[0],  [\xmx]
        b.gt            480f
        add             \xmy, \xmy,  #2
        ld1             {v1.s}[0],  [\xmy]
        sub             \sr2, \src, #2
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        sxtl            v0.8h,   v0.8b
        sxtl            v1.8h,   v1.8b
        mov             x15, x30

        // 4x2, 4x4 hv
        ld1             {v25.8h}, [\src], \s_strd
        ext             v26.16b, v25.16b, v25.16b, #2
        ext             v27.16b, v25.16b, v25.16b, #4
        ext             v28.16b, v25.16b, v25.16b, #6
        smull           v25.4s,  v25.4h,  v0.h[0]
        smlal           v25.4s,  v26.4h,  v0.h[1]
        smlal           v25.4s,  v27.4h,  v0.h[2]
        smlal           v25.4s,  v28.4h,  v0.h[3]
        srshl           v16.4s,  v25.4s,  v30.4s // -(6-intermediate_bits)
        // The intermediates from the horizontal pass fit in 16 bit without
        // any bias; we could just as well keep them as .4s, but narrowing
        // them to .4h gives a significant speedup on out of order cores
        // (at the cost of a smaller slowdown on in-order cores such as A53).
        xtn             v16.4h,  v16.4s

        bl              L(\type\()_8tap_filter_4)
        mov             v17.8b,  v24.8b
        mov             v18.8b,  v25.8b

4:
        bl              L(\type\()_8tap_filter_4)
        smull           v2.4s,   v16.4h,  v1.h[0]
        smlal           v2.4s,   v17.4h,  v1.h[1]
        smlal           v2.4s,   v18.4h,  v1.h[2]
        smlal           v2.4s,   v24.4h,  v1.h[3]
        smull           v3.4s,   v17.4h,  v1.h[0]
        smlal           v3.4s,   v18.4h,  v1.h[1]
        smlal           v3.4s,   v24.4h,  v1.h[2]
        smlal           v3.4s,   v25.4h,  v1.h[3]
.ifc \type, put
        srshl           v2.4s,   v2.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v3.4s,   v3.4s,   v29.4s // -(6+intermediate_bits)
        sqxtun          v2.4h,   v2.4s
        sqxtun2         v2.8h,   v3.4s
        umin            v2.8h,   v2.8h,   v31.8h
.else
        rshrn           v2.4h,   v2.4s,   #6
        rshrn2          v2.8h,   v3.4s,   #6
        sub             v2.8h,   v2.8h,   v29.8h // PREP_BIAS
.endif
        subs            \h,  \h,  #2

        st1             {v2.d}[0], [\dst], \d_strd
        st1             {v2.d}[1], [\ds2], \d_strd
        b.le            0f
        mov             v16.8b,  v18.8b
        mov             v17.8b,  v24.8b
        mov             v18.8b,  v25.8b
        b               4b

480:    // 4x8, 4x16, 4x32 hv
        ld1             {v1.8b},  [\xmy]
        sub             \src, \src, #2
        sub             \sr2, \src, \s_strd, lsl #1
        sub             \src, \sr2, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1
        sxtl            v0.8h,   v0.8b
        sxtl            v1.8h,   v1.8b
        mov             x15, x30

        ld1             {v25.8h}, [\src], \s_strd
        ext             v26.16b, v25.16b, v25.16b, #2
        ext             v27.16b, v25.16b, v25.16b, #4
        ext             v28.16b, v25.16b, v25.16b, #6
        smull           v25.4s,  v25.4h,  v0.h[0]
        smlal           v25.4s,  v26.4h,  v0.h[1]
        smlal           v25.4s,  v27.4h,  v0.h[2]
        smlal           v25.4s,  v28.4h,  v0.h[3]
        srshl           v16.4s,  v25.4s,  v30.4s // -(6-intermediate_bits)
        // The intermediates from the horizontal pass fit in 16 bit without
        // any bias; we could just as well keep them as .4s, but narrowing
        // them to .4h gives a significant speedup on out of order cores
        // (at the cost of a smaller slowdown on in-order cores such as A53).
        xtn             v16.4h,  v16.4s

        bl              L(\type\()_8tap_filter_4)
        mov             v17.8b,  v24.8b
        mov             v18.8b,  v25.8b
        bl              L(\type\()_8tap_filter_4)
        mov             v19.8b,  v24.8b
        mov             v20.8b,  v25.8b
        bl              L(\type\()_8tap_filter_4)
        mov             v21.8b,  v24.8b
        mov             v22.8b,  v25.8b

48:
        bl              L(\type\()_8tap_filter_4)
        smull           v3.4s,   v16.4h,  v1.h[0]
        smlal           v3.4s,   v17.4h,  v1.h[1]
        smlal           v3.4s,   v18.4h,  v1.h[2]
        smlal           v3.4s,   v19.4h,  v1.h[3]
        smlal           v3.4s,   v20.4h,  v1.h[4]
        smlal           v3.4s,   v21.4h,  v1.h[5]
        smlal           v3.4s,   v22.4h,  v1.h[6]
        smlal           v3.4s,   v24.4h,  v1.h[7]
        smull           v4.4s,   v17.4h,  v1.h[0]
        smlal           v4.4s,   v18.4h,  v1.h[1]
        smlal           v4.4s,   v19.4h,  v1.h[2]
        smlal           v4.4s,   v20.4h,  v1.h[3]
        smlal           v4.4s,   v21.4h,  v1.h[4]
        smlal           v4.4s,   v22.4h,  v1.h[5]
        smlal           v4.4s,   v24.4h,  v1.h[6]
        smlal           v4.4s,   v25.4h,  v1.h[7]
.ifc \type, put
        srshl           v3.4s,   v3.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v4.4s,   v4.4s,   v29.4s // -(6+intermediate_bits)
        sqxtun          v3.4h,   v3.4s
        sqxtun2         v3.8h,   v4.4s
        umin            v3.8h,   v3.8h,   v31.8h
.else
        rshrn           v3.4h,   v3.4s,   #6
        rshrn2          v3.8h,   v4.4s,   #6
        sub             v3.8h,   v3.8h,   v29.8h // PREP_BIAS
.endif
        subs            \h,  \h,  #2
        st1             {v3.d}[0], [\dst], \d_strd
        st1             {v3.d}[1], [\ds2], \d_strd
        b.le            0f
        mov             v16.8b,  v18.8b
        mov             v17.8b,  v19.8b
        mov             v18.8b,  v20.8b
        mov             v19.8b,  v21.8b
        mov             v20.8b,  v22.8b
        mov             v21.8b,  v24.8b
        mov             v22.8b,  v25.8b
        b               48b
0:
        ret             x15

L(\type\()_8tap_filter_4):
        ld1             {v24.8h}, [\sr2], \s_strd
        ld1             {v25.8h}, [\src], \s_strd
        ext             v26.16b, v24.16b, v24.16b, #2
        ext             v27.16b, v24.16b, v24.16b, #4
        ext             v28.16b, v24.16b, v24.16b, #6
        smull           v24.4s,  v24.4h,  v0.h[0]
        smlal           v24.4s,  v26.4h,  v0.h[1]
        smlal           v24.4s,  v27.4h,  v0.h[2]
        smlal           v24.4s,  v28.4h,  v0.h[3]
        ext             v26.16b, v25.16b, v25.16b, #2
        ext             v27.16b, v25.16b, v25.16b, #4
        ext             v28.16b, v25.16b, v25.16b, #6
        smull           v25.4s,  v25.4h,  v0.h[0]
        smlal           v25.4s,  v26.4h,  v0.h[1]
        smlal           v25.4s,  v27.4h,  v0.h[2]
        smlal           v25.4s,  v28.4h,  v0.h[3]
        srshl           v24.4s,  v24.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v25.4s,  v25.4s,  v30.4s // -(6-intermediate_bits)
        xtn             v24.4h,  v24.4s
        xtn             v25.4h,  v25.4s
        ret

80:
160:
320:
        AARCH64_VALID_JUMP_TARGET
        b.gt            880f
        add             \xmy,  \xmy,  #2
        ld1             {v0.8b},  [\xmx]
        ld1             {v1.s}[0],  [\xmy]
        sub             \src,  \src,  #6
        sub             \src,  \src,  \s_strd
        sxtl            v0.8h,   v0.8b
        sxtl            v1.8h,   v1.8b
        mov             x15, x30
        mov             \my, \h

164:    // 8x2, 8x4, 16x2, 16x4, 32x2, 32x4 hv
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd, \d_strd, #1
        lsl             \s_strd, \s_strd, #1

        ld1             {v27.8h, v28.8h},  [\src], \s_strd
        smull           v24.4s,  v27.4h,  v0.h[0]
        smull2          v25.4s,  v27.8h,  v0.h[0]
.irpc i, 1234567
        ext             v26.16b, v27.16b, v28.16b, #(2*\i)
        smlal           v24.4s,  v26.4h,  v0.h[\i]
        smlal2          v25.4s,  v26.8h,  v0.h[\i]
.endr
        srshl           v24.4s,  v24.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v25.4s,  v25.4s,  v30.4s // -(6-intermediate_bits)
        // The intermediates from the horizontal pass fit in 16 bit without
        // any bias; we could just as well keep them as .4s, but narrowing
        // them to .4h gives a significant speedup on out of order cores
        // (at the cost of a smaller slowdown on in-order cores such as A53),
        // and conserves register space (no need to clobber v8-v15).
        uzp1            v16.8h,  v24.8h,  v25.8h // Same as xtn, xtn2

        bl              L(\type\()_8tap_filter_8)
        mov             v17.16b, v23.16b
        mov             v18.16b, v24.16b

8:
        smull           v2.4s,   v16.4h,  v1.h[0]
        smull2          v3.4s,   v16.8h,  v1.h[0]
        bl              L(\type\()_8tap_filter_8)
        smull           v4.4s,   v17.4h,  v1.h[0]
        smull2          v5.4s,   v17.8h,  v1.h[0]
        smlal           v2.4s,   v17.4h,  v1.h[1]
        smlal2          v3.4s,   v17.8h,  v1.h[1]
        smlal           v4.4s,   v18.4h,  v1.h[1]
        smlal2          v5.4s,   v18.8h,  v1.h[1]
        smlal           v2.4s,   v18.4h,  v1.h[2]
        smlal2          v3.4s,   v18.8h,  v1.h[2]
        smlal           v4.4s,   v23.4h,  v1.h[2]
        smlal2          v5.4s,   v23.8h,  v1.h[2]
        smlal           v2.4s,   v23.4h,  v1.h[3]
        smlal2          v3.4s,   v23.8h,  v1.h[3]
        smlal           v4.4s,   v24.4h,  v1.h[3]
        smlal2          v5.4s,   v24.8h,  v1.h[3]
.ifc \type, put
        srshl           v2.4s,   v2.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v3.4s,   v3.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v4.4s,   v4.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v5.4s,   v5.4s,   v29.4s // -(6+intermediate_bits)
        sqxtun          v2.4h,   v2.4s
        sqxtun2         v2.8h,   v3.4s
        sqxtun          v3.4h,   v4.4s
        sqxtun2         v3.8h,   v5.4s
        umin            v2.8h,   v2.8h,   v31.8h
        umin            v3.8h,   v3.8h,   v31.8h
.else
        rshrn           v2.4h,   v2.4s,   #6
        rshrn2          v2.8h,   v3.4s,   #6
        rshrn           v3.4h,   v4.4s,   #6
        rshrn2          v3.8h,   v5.4s,   #6
        sub             v2.8h,   v2.8h,   v29.8h // PREP_BIAS
        sub             v3.8h,   v3.8h,   v29.8h // PREP_BIAS
.endif
        subs            \h,  \h,  #2
        st1             {v2.8h}, [\dst], \d_strd
        st1             {v3.8h}, [\ds2], \d_strd
        b.le            9f
        mov             v16.16b, v18.16b
        mov             v17.16b, v23.16b
        mov             v18.16b, v24.16b
        b               8b
9:
        subs            \w,  \w,  #8
        b.le            0f
        asr             \s_strd,  \s_strd,  #1
        asr             \d_strd,  \d_strd,  #1
        msub            \src,  \s_strd,  \xmy,  \src
        msub            \dst,  \d_strd,  \xmy,  \dst
        sub             \src,  \src,  \s_strd,  lsl #2
        mov             \h,  \my
        add             \src,  \src,  #16
        add             \dst,  \dst,  #16
        b               164b

880:    // 8x8, 8x16, ..., 16x8, ..., 32x8, ... hv
640:
1280:
        AARCH64_VALID_JUMP_TARGET
        ld1             {v0.8b},  [\xmx]
        ld1             {v1.8b},  [\xmy]
        sub             \src,  \src,  #6
        sub             \src,  \src,  \s_strd
        sub             \src,  \src,  \s_strd, lsl #1
        sxtl            v0.8h,   v0.8b
        sxtl            v1.8h,   v1.8b
        mov             x15, x30
        mov             \my, \h

168:
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd, \d_strd, #1
        lsl             \s_strd, \s_strd, #1

        ld1             {v27.8h, v28.8h},  [\src], \s_strd
        smull           v24.4s,  v27.4h,  v0.h[0]
        smull2          v25.4s,  v27.8h,  v0.h[0]
.irpc i, 1234567
        ext             v26.16b, v27.16b, v28.16b, #(2*\i)
        smlal           v24.4s,  v26.4h,  v0.h[\i]
        smlal2          v25.4s,  v26.8h,  v0.h[\i]
.endr
        srshl           v24.4s,  v24.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v25.4s,  v25.4s,  v30.4s // -(6-intermediate_bits)
        // The intermediates from the horizontal pass fit in 16 bit without
        // any bias; we could just as well keep them as .4s, but narrowing
        // them to .4h gives a significant speedup on out of order cores
        // (at the cost of a smaller slowdown on in-order cores such as A53),
        // and conserves register space (no need to clobber v8-v15).
        uzp1            v16.8h,  v24.8h,  v25.8h // Same as xtn, xtn2

        bl              L(\type\()_8tap_filter_8)
        mov             v17.16b, v23.16b
        mov             v18.16b, v24.16b
        bl              L(\type\()_8tap_filter_8)
        mov             v19.16b, v23.16b
        mov             v20.16b, v24.16b
        bl              L(\type\()_8tap_filter_8)
        mov             v21.16b, v23.16b
        mov             v22.16b, v24.16b

88:
        smull           v2.4s,   v16.4h,  v1.h[0]
        smull2          v3.4s,   v16.8h,  v1.h[0]
        bl              L(\type\()_8tap_filter_8)
        smull           v4.4s,   v17.4h,  v1.h[0]
        smull2          v5.4s,   v17.8h,  v1.h[0]
        smlal           v2.4s,   v17.4h,  v1.h[1]
        smlal2          v3.4s,   v17.8h,  v1.h[1]
        smlal           v4.4s,   v18.4h,  v1.h[1]
        smlal2          v5.4s,   v18.8h,  v1.h[1]
        smlal           v2.4s,   v18.4h,  v1.h[2]
        smlal2          v3.4s,   v18.8h,  v1.h[2]
        smlal           v4.4s,   v19.4h,  v1.h[2]
        smlal2          v5.4s,   v19.8h,  v1.h[2]
        smlal           v2.4s,   v19.4h,  v1.h[3]
        smlal2          v3.4s,   v19.8h,  v1.h[3]
        smlal           v4.4s,   v20.4h,  v1.h[3]
        smlal2          v5.4s,   v20.8h,  v1.h[3]
        smlal           v2.4s,   v20.4h,  v1.h[4]
        smlal2          v3.4s,   v20.8h,  v1.h[4]
        smlal           v4.4s,   v21.4h,  v1.h[4]
        smlal2          v5.4s,   v21.8h,  v1.h[4]
        smlal           v2.4s,   v21.4h,  v1.h[5]
        smlal2          v3.4s,   v21.8h,  v1.h[5]
        smlal           v4.4s,   v22.4h,  v1.h[5]
        smlal2          v5.4s,   v22.8h,  v1.h[5]
        smlal           v2.4s,   v22.4h,  v1.h[6]
        smlal2          v3.4s,   v22.8h,  v1.h[6]
        smlal           v4.4s,   v23.4h,  v1.h[6]
        smlal2          v5.4s,   v23.8h,  v1.h[6]
        smlal           v2.4s,   v23.4h,  v1.h[7]
        smlal2          v3.4s,   v23.8h,  v1.h[7]
        smlal           v4.4s,   v24.4h,  v1.h[7]
        smlal2          v5.4s,   v24.8h,  v1.h[7]
.ifc \type, put
        srshl           v2.4s,   v2.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v3.4s,   v3.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v4.4s,   v4.4s,   v29.4s // -(6+intermediate_bits)
        srshl           v5.4s,   v5.4s,   v29.4s // -(6+intermediate_bits)
        sqxtun          v2.4h,   v2.4s
        sqxtun2         v2.8h,   v3.4s
        sqxtun          v3.4h,   v4.4s
        sqxtun2         v3.8h,   v5.4s
        umin            v2.8h,   v2.8h,   v31.8h
        umin            v3.8h,   v3.8h,   v31.8h
.else
        rshrn           v2.4h,   v2.4s,   #6
        rshrn2          v2.8h,   v3.4s,   #6
        rshrn           v3.4h,   v4.4s,   #6
        rshrn2          v3.8h,   v5.4s,   #6
        sub             v2.8h,   v2.8h,   v29.8h // PREP_BIAS
        sub             v3.8h,   v3.8h,   v29.8h // PREP_BIAS
.endif
        subs            \h,  \h,  #2
        st1             {v2.8h}, [\dst], \d_strd
        st1             {v3.8h}, [\ds2], \d_strd
        b.le            9f
        mov             v16.16b, v18.16b
        mov             v17.16b, v19.16b
        mov             v18.16b, v20.16b
        mov             v19.16b, v21.16b
        mov             v20.16b, v22.16b
        mov             v21.16b, v23.16b
        mov             v22.16b, v24.16b
        b               88b
9:
        subs            \w,  \w,  #8
        b.le            0f
        asr             \s_strd,  \s_strd,  #1
        asr             \d_strd,  \d_strd,  #1
        msub            \src,  \s_strd,  \xmy,  \src
        msub            \dst,  \d_strd,  \xmy,  \dst
        sub             \src,  \src,  \s_strd,  lsl #3
        mov             \h,  \my
        add             \src,  \src,  #16
        add             \dst,  \dst,  #16
        b               168b
0:
        ret             x15

L(\type\()_8tap_filter_8):
        ld1             {v4.8h, v5.8h},  [\sr2], \s_strd
        ld1             {v6.8h, v7.8h},  [\src], \s_strd
        smull           v25.4s,  v4.4h,   v0.h[0]
        smull2          v26.4s,  v4.8h,   v0.h[0]
        smull           v27.4s,  v6.4h,   v0.h[0]
        smull2          v28.4s,  v6.8h,   v0.h[0]
.irpc i, 1234567
        ext             v23.16b, v4.16b,  v5.16b,  #(2*\i)
        ext             v24.16b, v6.16b,  v7.16b,  #(2*\i)
        smlal           v25.4s,  v23.4h,  v0.h[\i]
        smlal2          v26.4s,  v23.8h,  v0.h[\i]
        smlal           v27.4s,  v24.4h,  v0.h[\i]
        smlal2          v28.4s,  v24.8h,  v0.h[\i]
.endr
        srshl           v25.4s,  v25.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v26.4s,  v26.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v27.4s,  v27.4s,  v30.4s // -(6-intermediate_bits)
        srshl           v28.4s,  v28.4s,  v30.4s // -(6-intermediate_bits)
        uzp1            v23.8h,  v25.8h,  v26.8h // Same as xtn, xtn2
        uzp1            v24.8h,  v27.8h,  v28.8h // Ditto
        ret

L(\type\()_8tap_hv_tbl):
        .hword L(\type\()_8tap_hv_tbl) - 1280b
        .hword L(\type\()_8tap_hv_tbl) -  640b
        .hword L(\type\()_8tap_hv_tbl) -  320b
        .hword L(\type\()_8tap_hv_tbl) -  160b
        .hword L(\type\()_8tap_hv_tbl) -   80b
        .hword L(\type\()_8tap_hv_tbl) -   40b
        .hword L(\type\()_8tap_hv_tbl) -   20b
        .hword 0
endfunc


function \type\()_bilin_16bpc_neon, export=1
.ifc \bdmax, w8
        ldr             w8,  [sp]
.endif
        dup             v1.8h,   \mx
        dup             v3.8h,   \my
        mov             w10, #16
        sub             w9,  w10, \mx
        sub             w10, w10, \my
        dup             v0.8h,   w9
        dup             v2.8h,   w10
.ifc \type, prep
        uxtw            \d_strd, \w
        lsl             \d_strd, \d_strd, #1
.endif

        clz             \bdmax,   \bdmax       // bitdepth_max
        clz             w9,  \w
        sub             \bdmax,   \bdmax,  #18 // intermediate_bits = clz(bitdepth_max) - 18
        mov             w11, #4
        sub             w9,  w9,  #24
        sub             w11, w11, \bdmax  // 4 - intermediate_bits
        add             w12, \bdmax, #4   // 4 + intermediate_bits
        cbnz            \mx, L(\type\()_bilin_h)
        cbnz            \my, L(\type\()_bilin_v)
        b               \type\()_neon

L(\type\()_bilin_h):
        cbnz            \my, L(\type\()_bilin_hv)

        adr             x10, L(\type\()_bilin_h_tbl)
        dup             v31.8h,  w11      // 4 - intermediate_bits
        ldrh            w9,  [x10, x9, lsl #1]
        neg             v31.8h,  v31.8h   // -(4-intermediate_bits)
.ifc \type, put
        dup             v30.8h,  \bdmax   // intermediate_bits
.else
        movi            v29.8h,  #(PREP_BIAS >> 8), lsl #8
.endif
        sub             x10, x10, w9, uxtw
.ifc \type, put
        neg             v30.8h,  v30.8h   // -intermediate_bits
.endif
        br              x10

20:     // 2xN h
        AARCH64_VALID_JUMP_TARGET
.ifc \type, put
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
2:
        ld1             {v4.4h},  [\src], \s_strd
        ld1             {v6.4h},  [\sr2], \s_strd
        ext             v5.8b,   v4.8b,   v4.8b,   #2
        ext             v7.8b,   v6.8b,   v6.8b,   #2
        trn1            v4.2s,   v4.2s,   v6.2s
        trn1            v5.2s,   v5.2s,   v7.2s
        subs            \h,  \h,  #2
        mul             v4.4h,   v4.4h,   v0.4h
        mla             v4.4h,   v5.4h,   v1.4h
        urshl           v4.4h,   v4.4h,   v31.4h
        urshl           v4.4h,   v4.4h,   v30.4h
        st1             {v4.s}[0], [\dst], \d_strd
        st1             {v4.s}[1], [\ds2], \d_strd
        b.gt            2b
        ret
.endif

40:     // 4xN h
        AARCH64_VALID_JUMP_TARGET
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
4:
        ld1             {v4.8h}, [\src], \s_strd
        ld1             {v6.8h}, [\sr2], \s_strd
        ext             v5.16b,  v4.16b,  v4.16b,  #2
        ext             v7.16b,  v6.16b,  v6.16b,  #2
        trn1            v4.2d,   v4.2d,   v6.2d
        trn1            v5.2d,   v5.2d,   v7.2d
        subs            \h,  \h,  #2
        mul             v4.8h,   v4.8h,   v0.8h
        mla             v4.8h,   v5.8h,   v1.8h
        urshl           v4.8h,   v4.8h,   v31.8h
.ifc \type, put
        urshl           v4.8h,   v4.8h,   v30.8h
.else
        sub             v4.8h,   v4.8h,   v29.8h
.endif
        st1             {v4.d}[0], [\dst], \d_strd
        st1             {v4.d}[1], [\ds2], \d_strd
        b.gt            4b
        ret

80:     // 8xN h
        AARCH64_VALID_JUMP_TARGET
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \d_strd,  \d_strd,  #1
        lsl             \s_strd,  \s_strd,  #1
8:
        ldr             h5,  [\src, #16]
        ldr             h7,  [\sr2, #16]
        ld1             {v4.8h}, [\src], \s_strd
        ld1             {v6.8h}, [\sr2], \s_strd
        ext             v5.16b,  v4.16b,  v5.16b,  #2
        ext             v7.16b,  v6.16b,  v7.16b,  #2
        subs            \h,  \h,  #2
        mul             v4.8h,   v4.8h,   v0.8h
        mla             v4.8h,   v5.8h,   v1.8h
        mul             v6.8h,   v6.8h,   v0.8h
        mla             v6.8h,   v7.8h,   v1.8h
        urshl           v4.8h,   v4.8h,   v31.8h
        urshl           v6.8h,   v6.8h,   v31.8h
.ifc \type, put
        urshl           v4.8h,   v4.8h,   v30.8h
        urshl           v6.8h,   v6.8h,   v30.8h
.else
        sub             v4.8h,   v4.8h,   v29.8h
        sub             v6.8h,   v6.8h,   v29.8h
.endif
        st1             {v4.8h}, [\dst], \d_strd
        st1             {v6.8h}, [\ds2], \d_strd
        b.gt            8b
        ret
160:
320:
640:
1280:   // 16xN, 32xN, ... h
        AARCH64_VALID_JUMP_TARGET
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1

        sub             \s_strd,  \s_strd,  \w, uxtw #1
        sub             \s_strd,  \s_strd,  #16
.ifc \type, put
        lsl             \d_strd,  \d_strd,  #1
        sub             \d_strd,  \d_strd,  \w, uxtw #1
.endif
161:
        ld1             {v16.8h},  [\src], #16
        ld1             {v21.8h},  [\sr2], #16
        mov             \mx, \w

16:
        ld1             {v17.8h, v18.8h},  [\src], #32
        ld1             {v22.8h, v23.8h},  [\sr2], #32
        ext             v19.16b, v16.16b, v17.16b, #2
        ext             v20.16b, v17.16b, v18.16b, #2
        ext             v24.16b, v21.16b, v22.16b, #2
        ext             v25.16b, v22.16b, v23.16b, #2
        mul             v16.8h,  v16.8h,  v0.8h
        mla             v16.8h,  v19.8h,  v1.8h
        mul             v17.8h,  v17.8h,  v0.8h
        mla             v17.8h,  v20.8h,  v1.8h
        mul             v21.8h,  v21.8h,  v0.8h
        mla             v21.8h,  v24.8h,  v1.8h
        mul             v22.8h,  v22.8h,  v0.8h
        mla             v22.8h,  v25.8h,  v1.8h
        urshl           v16.8h,  v16.8h,  v31.8h
        urshl           v17.8h,  v17.8h,  v31.8h
        urshl           v21.8h,  v21.8h,  v31.8h
        urshl           v22.8h,  v22.8h,  v31.8h
        subs            \mx, \mx, #16
.ifc \type, put
        urshl           v16.8h,  v16.8h,  v30.8h
        urshl           v17.8h,  v17.8h,  v30.8h
        urshl           v21.8h,  v21.8h,  v30.8h
        urshl           v22.8h,  v22.8h,  v30.8h
.else
        sub             v16.8h,  v16.8h,  v29.8h
        sub             v17.8h,  v17.8h,  v29.8h
        sub             v21.8h,  v21.8h,  v29.8h
        sub             v22.8h,  v22.8h,  v29.8h
.endif
        st1             {v16.8h, v17.8h}, [\dst], #32
        st1             {v21.8h, v22.8h}, [\ds2], #32
        b.le            9f

        mov             v16.16b, v18.16b
        mov             v21.16b, v23.16b
        b               16b

9:
        add             \dst,  \dst,  \d_strd
        add             \ds2,  \ds2,  \d_strd
        add             \src,  \src,  \s_strd
        add             \sr2,  \sr2,  \s_strd

        subs            \h,  \h,  #2
        b.gt            161b
        ret

L(\type\()_bilin_h_tbl):
        .hword L(\type\()_bilin_h_tbl) - 1280b
        .hword L(\type\()_bilin_h_tbl) -  640b
        .hword L(\type\()_bilin_h_tbl) -  320b
        .hword L(\type\()_bilin_h_tbl) -  160b
        .hword L(\type\()_bilin_h_tbl) -   80b
        .hword L(\type\()_bilin_h_tbl) -   40b
        .hword L(\type\()_bilin_h_tbl) -   20b
        .hword 0


L(\type\()_bilin_v):
        cmp             \h,  #4
        adr             x10, L(\type\()_bilin_v_tbl)
.ifc \type, prep
        dup             v31.8h,  w11      // 4 - intermediate_bits
.endif
        ldrh            w9,  [x10, x9, lsl #1]
.ifc \type, prep
        movi            v29.8h,  #(PREP_BIAS >> 8), lsl #8
        neg             v31.8h,  v31.8h   // -(4-intermediate_bits)
.endif
        sub             x10, x10, w9, uxtw
        br              x10

20:     // 2xN v
        AARCH64_VALID_JUMP_TARGET
.ifc \type, put
        cmp             \h,  #2
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1

        // 2x2 v
        ld1             {v16.s}[0], [\src], \s_strd
        b.gt            24f
22:
        ld1             {v17.s}[0], [\sr2], \s_strd
        ld1             {v18.s}[0], [\src], \s_strd
        trn1            v16.2s,  v16.2s,  v17.2s
        trn1            v17.2s,  v17.2s,  v18.2s
        mul             v4.4h,   v16.4h,  v2.4h
        mla             v4.4h,   v17.4h,  v3.4h
        urshr           v4.8h,   v4.8h,   #4
        st1             {v4.s}[0], [\dst]
        st1             {v4.s}[1], [\ds2]
        ret
24:     // 2x4, 2x6, 2x8, ... v
        ld1             {v17.s}[0], [\sr2], \s_strd
        ld1             {v18.s}[0], [\src], \s_strd
        ld1             {v19.s}[0], [\sr2], \s_strd
        ld1             {v20.s}[0], [\src], \s_strd
        sub             \h,  \h,  #4
        trn1            v16.2s,  v16.2s,  v17.2s
        trn1            v17.2s,  v17.2s,  v18.2s
        trn1            v18.2s,  v18.2s,  v19.2s
        trn1            v19.2s,  v19.2s,  v20.2s
        trn1            v16.2d,  v16.2d,  v18.2d
        trn1            v17.2d,  v17.2d,  v19.2d
        mul             v4.8h,   v16.8h,  v2.8h
        mla             v4.8h,   v17.8h,  v3.8h
        cmp             \h,  #2
        urshr           v4.8h,   v4.8h,   #4
        st1             {v4.s}[0], [\dst], \d_strd
        st1             {v4.s}[1], [\ds2], \d_strd
        st1             {v4.s}[2], [\dst], \d_strd
        st1             {v4.s}[3], [\ds2], \d_strd
        b.lt            0f
        mov             v16.8b,  v20.8b
        b.eq            22b
        b               24b
0:
        ret
.endif

40:     // 4xN v
        AARCH64_VALID_JUMP_TARGET
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1
        ld1             {v16.4h}, [\src], \s_strd
4:
        ld1             {v17.4h}, [\sr2], \s_strd
        ld1             {v18.4h}, [\src], \s_strd
        trn1            v16.2d,  v16.2d,  v17.2d
        trn1            v17.2d,  v17.2d,  v18.2d
        mul             v4.8h,   v16.8h,  v2.8h
        mla             v4.8h,   v17.8h,  v3.8h
        subs            \h,  \h,  #2
.ifc \type, put
        urshr           v4.8h,   v4.8h,   #4
.else
        urshl           v4.8h,   v4.8h,   v31.8h
        sub             v4.8h,   v4.8h,   v29.8h
.endif
        st1             {v4.d}[0], [\dst], \d_strd
        st1             {v4.d}[1], [\ds2], \d_strd
        b.le            0f
        mov             v16.8b,  v18.8b
        b               4b
0:
        ret

80:     // 8xN v
        AARCH64_VALID_JUMP_TARGET
        add             \ds2,  \dst,  \d_strd
        add             \sr2,  \src,  \s_strd
        lsl             \s_strd,  \s_strd,  #1
        lsl             \d_strd,  \d_strd,  #1
        ld1             {v16.8h}, [\src], \s_strd
8:
        ld1             {v17.8h}, [\sr2], \s_strd
        ld1             {v18.8h}, [\src], \s_strd
        mul             v4.8h,   v16.8h,  v2.8h
        mla             v4.8h,   v17.8h,  v3.8h
        mul             v5.8h,   v17.8h,  v2.8h
        mla             v5.8h,   v18.8h,  v3.8h
        subs            \h,  \h,  #2
.ifc \type, put
        urshr           v4.8h,   v4.8h,   #4
        urshr           v5.8h,   v5.8h,   #4
.else
        urshl           v4.8h,   v4.8h,   v31.8h
        urshl           v5.8h,   v5.8h,   v31.8h
        sub             v4.8h,   v4.8h,   v29.8h
        sub             v5.8h,   v5.8h,   v29.8h
.endif
        st1             {v4.8h}, [\dst], \d_strd
        st1             {v5.8h}, [\ds2], \d_strd
        b.le            0f
        mov             v16.16b, v18.16b
        b               8b
0:
        ret

160:    // 16xN, 32xN, ...
320:
640:
1280:
        AARCH64_VALID_JUMP_TARGET
        mov             \my, \h
1:
        add             \ds2, \dst, \d_strd
        add             \sr2, \src, \s_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        ld1             {v16.8h, v17.8h}, [\src], \s_strd
2:
        ld1             {v18.8h, v19.8h}, [\sr2], \s_strd
        ld1             {v20.8h, v21.8h}, [\src], \s_strd
        mul             v4.8h,   v16.8h,  v2.8h
        mla             v4.8h,   v18.8h,  v3.8h
        mul             v5.8h,   v17.8h,  v2.8h
        mla             v5.8h,   v19.8h,  v3.8h
        mul             v6.8h,   v18.8h,  v2.8h
        mla             v6.8h,   v20.8h,  v3.8h
        mul             v7.8h,   v19.8h,  v2.8h
        mla             v7.8h,   v21.8h,  v3.8h
        subs            \h,  \h,  #2
.ifc \type, put
        urshr           v4.8h,   v4.8h,   #4
        urshr           v5.8h,   v5.8h,   #4
        urshr           v6.8h,   v6.8h,   #4
        urshr           v7.8h,   v7.8h,   #4
.else
        urshl           v4.8h,   v4.8h,   v31.8h
        urshl           v5.8h,   v5.8h,   v31.8h
        urshl           v6.8h,   v6.8h,   v31.8h
        urshl           v7.8h,   v7.8h,   v31.8h
        sub             v4.8h,   v4.8h,   v29.8h
        sub             v5.8h,   v5.8h,   v29.8h
        sub             v6.8h,   v6.8h,   v29.8h
        sub             v7.8h,   v7.8h,   v29.8h
.endif
        st1             {v4.8h, v5.8h}, [\dst], \d_strd
        st1             {v6.8h, v7.8h}, [\ds2], \d_strd
        b.le            9f
        mov             v16.16b, v20.16b
        mov             v17.16b, v21.16b
        b               2b
9:
        subs            \w,  \w,  #16
        b.le            0f
        asr             \s_strd, \s_strd, #1
        asr             \d_strd, \d_strd, #1
        msub            \src, \s_strd, \xmy, \src
        msub            \dst, \d_strd, \xmy, \dst
        sub             \src, \src, \s_strd, lsl #1
        mov             \h,  \my
        add             \src, \src, #32
        add             \dst, \dst, #32
        b               1b
0:
        ret

L(\type\()_bilin_v_tbl):
        .hword L(\type\()_bilin_v_tbl) - 1280b
        .hword L(\type\()_bilin_v_tbl) -  640b
        .hword L(\type\()_bilin_v_tbl) -  320b
        .hword L(\type\()_bilin_v_tbl) -  160b
        .hword L(\type\()_bilin_v_tbl) -   80b
        .hword L(\type\()_bilin_v_tbl) -   40b
        .hword L(\type\()_bilin_v_tbl) -   20b
        .hword 0

L(\type\()_bilin_hv):
        adr             x10, L(\type\()_bilin_hv_tbl)
        dup             v31.8h,  w11      // 4 - intermediate_bits
        ldrh            w9,  [x10, x9, lsl #1]
        neg             v31.8h,  v31.8h   // -(4-intermediate_bits)
.ifc \type, put
        dup             v30.4s,  w12      // 4 + intermediate_bits
.else
        movi            v29.8h,  #(PREP_BIAS >> 8), lsl #8
.endif
        sub             x10, x10, w9, uxtw
.ifc \type, put
        neg             v30.4s,  v30.4s   // -(4+intermediate_bits)
.endif
        br              x10

20:     // 2xN hv
        AARCH64_VALID_JUMP_TARGET
.ifc \type, put
        add             \sr2, \src, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        ld1             {v20.4h},  [\src], \s_strd
        ext             v21.8b,  v20.8b,  v20.8b,  #2
        mul             v16.4h,  v20.4h,  v0.4h
        mla             v16.4h,  v21.4h,  v1.4h
        urshl           v16.4h,  v16.4h,  v31.4h

2:
        ld1             {v22.4h},  [\sr2], \s_strd
        ld1             {v24.4h},  [\src], \s_strd
        ext             v23.8b,  v22.8b,  v22.8b,  #2
        ext             v25.8b,  v24.8b,  v24.8b,  #2
        trn1            v22.2s,  v22.2s,  v24.2s
        trn1            v23.2s,  v23.2s,  v25.2s
        mul             v17.4h,  v22.4h,  v0.4h
        mla             v17.4h,  v23.4h,  v1.4h
        urshl           v17.4h,  v17.4h,  v31.4h

        trn1            v16.2s,  v16.2s,  v17.2s

        umull           v4.4s,   v16.4h,  v2.4h
        umlal           v4.4s,   v17.4h,  v3.4h
        urshl           v4.4s,   v4.4s,   v30.4s
        xtn             v4.4h,   v4.4s
        subs            \h,  \h,  #2
        st1             {v4.s}[0], [\dst], \d_strd
        st1             {v4.s}[1], [\ds2], \d_strd
        b.le            0f
        trn2            v16.2s,  v17.2s,  v17.2s
        b               2b
0:
        ret
.endif

40:     // 4xN hv
        AARCH64_VALID_JUMP_TARGET
        add             \sr2, \src, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        ld1             {v20.8h},  [\src], \s_strd
        ext             v21.16b, v20.16b, v20.16b, #2
        mul             v16.4h,  v20.4h,  v0.4h
        mla             v16.4h,  v21.4h,  v1.4h
        urshl           v16.4h,  v16.4h,  v31.4h

4:
        ld1             {v22.8h},  [\sr2], \s_strd
        ld1             {v24.8h},  [\src], \s_strd
        ext             v23.16b, v22.16b, v22.16b, #2
        ext             v25.16b, v24.16b, v24.16b, #2
        trn1            v22.2d,  v22.2d,  v24.2d
        trn1            v23.2d,  v23.2d,  v25.2d
        mul             v17.8h,  v22.8h,  v0.8h
        mla             v17.8h,  v23.8h,  v1.8h
        urshl           v17.8h,  v17.8h,  v31.8h

        trn1            v16.2d,  v16.2d,  v17.2d

        umull           v4.4s,   v16.4h,  v2.4h
        umlal           v4.4s,   v17.4h,  v3.4h
        umull2          v5.4s,   v16.8h,  v2.8h
        umlal2          v5.4s,   v17.8h,  v3.8h
.ifc \type, put
        urshl           v4.4s,   v4.4s,   v30.4s
        urshl           v5.4s,   v5.4s,   v30.4s
        uzp1            v4.8h,   v4.8h,   v5.8h  // Same as xtn, xtn2
.else
        rshrn           v4.4h,   v4.4s,   #4
        rshrn2          v4.8h,   v5.4s,   #4
        sub             v4.8h,   v4.8h,   v29.8h
.endif
        subs            \h,  \h,  #2
        st1             {v4.d}[0], [\dst], \d_strd
        st1             {v4.d}[1], [\ds2], \d_strd
        b.le            0f
        trn2            v16.2d,  v17.2d,  v17.2d
        b               4b
0:
        ret

80:     // 8xN, 16xN, ... hv
160:
320:
640:
1280:
        AARCH64_VALID_JUMP_TARGET
        mov             \my, \h

1:
        add             \sr2, \src, \s_strd
        add             \ds2, \dst, \d_strd
        lsl             \s_strd, \s_strd, #1
        lsl             \d_strd, \d_strd, #1

        ldr             h21, [\src, #16]
        ld1             {v20.8h},  [\src], \s_strd
        ext             v21.16b, v20.16b, v21.16b, #2
        mul             v16.8h,  v20.8h,  v0.8h
        mla             v16.8h,  v21.8h,  v1.8h
        urshl           v16.8h,  v16.8h,  v31.8h

2:
        ldr             h23, [\sr2, #16]
        ld1             {v22.8h},  [\sr2], \s_strd
        ldr             h25, [\src, #16]
        ld1             {v24.8h},  [\src], \s_strd
        ext             v23.16b, v22.16b, v23.16b, #2
        ext             v25.16b, v24.16b, v25.16b, #2
        mul             v17.8h,  v22.8h,  v0.8h
        mla             v17.8h,  v23.8h,  v1.8h
        mul             v18.8h,  v24.8h,  v0.8h
        mla             v18.8h,  v25.8h,  v1.8h
        urshl           v17.8h,  v17.8h,  v31.8h
        urshl           v18.8h,  v18.8h,  v31.8h

        umull           v4.4s,   v16.4h,  v2.4h
        umlal           v4.4s,   v17.4h,  v3.4h
        umull2          v5.4s,   v16.8h,  v2.8h
        umlal2          v5.4s,   v17.8h,  v3.8h
        umull           v6.4s,   v17.4h,  v2.4h
        umlal           v6.4s,   v18.4h,  v3.4h
        umull2          v7.4s,   v17.8h,  v2.8h
        umlal2          v7.4s,   v18.8h,  v3.8h
.ifc \type, put
        urshl           v4.4s,   v4.4s,   v30.4s
        urshl           v5.4s,   v5.4s,   v30.4s
        urshl           v6.4s,   v6.4s,   v30.4s
        urshl           v7.4s,   v7.4s,   v30.4s
        uzp1            v4.8h,   v4.8h,   v5.8h  // Same as xtn, xtn2
        uzp1            v5.8h,   v6.8h,   v7.8h  // Ditto
.else
        rshrn           v4.4h,   v4.4s,   #4
        rshrn2          v4.8h,   v5.4s,   #4
        rshrn           v5.4h,   v6.4s,   #4
        rshrn2          v5.8h,   v7.4s,   #4
        sub             v4.8h,   v4.8h,   v29.8h
        sub             v5.8h,   v5.8h,   v29.8h
.endif
        subs            \h,  \h,  #2
        st1             {v4.8h}, [\dst], \d_strd
        st1             {v5.8h}, [\ds2], \d_strd
        b.le            9f
        mov             v16.16b, v18.16b
        b               2b
9:
        subs            \w,  \w,  #8
        b.le            0f
        asr             \s_strd,  \s_strd,  #1
        asr             \d_strd,  \d_strd,  #1
        msub            \src,  \s_strd,  \xmy,  \src
        msub            \dst,  \d_strd,  \xmy,  \dst
        sub             \src,  \src,  \s_strd,  lsl #1
        mov             \h,  \my
        add             \src,  \src,  #16
        add             \dst,  \dst,  #16
        b               1b
0:
        ret

L(\type\()_bilin_hv_tbl):
        .hword L(\type\()_bilin_hv_tbl) - 1280b
        .hword L(\type\()_bilin_hv_tbl) -  640b
        .hword L(\type\()_bilin_hv_tbl) -  320b
        .hword L(\type\()_bilin_hv_tbl) -  160b
        .hword L(\type\()_bilin_hv_tbl) -   80b
        .hword L(\type\()_bilin_hv_tbl) -   40b
        .hword L(\type\()_bilin_hv_tbl) -   20b
        .hword 0
endfunc
.endm

filter_fn put,  x0, x1, x2, x3, w4, w5, w6, x6, w7, x7, w8, x9, x10
filter_fn prep, x0, x8, x1, x2, w3, w4, w5, x5, w6, x6, w7, x9, x10

.macro load_filter_row dst, src, inc
        asr             w13, \src, #10
        add             \src, \src, \inc
        ldr             \dst, [x11, w13, sxtw #3]
.endm

function warp_filter_horz_neon
        add             w12, w5,  #512

        ld1             {v16.8h, v17.8h}, [x2], x3

        load_filter_row d0, w12, w7
        load_filter_row d1, w12, w7
        load_filter_row d2, w12, w7
        sxtl            v0.8h,   v0.8b
        load_filter_row d3, w12, w7
        sxtl            v1.8h,   v1.8b
        load_filter_row d4, w12, w7
        sxtl            v2.8h,   v2.8b
        load_filter_row d5, w12, w7
        sxtl            v3.8h,   v3.8b
        load_filter_row d6, w12, w7
        sxtl            v4.8h,   v4.8b
        load_filter_row d7, w12, w7
        sxtl            v5.8h,   v5.8b
        ext             v18.16b, v16.16b, v17.16b, #2*1
        smull           v8.4s,   v16.4h,  v0.4h
        smull2          v9.4s,   v16.8h,  v0.8h
        sxtl            v6.8h,   v6.8b
        ext             v19.16b, v16.16b, v17.16b, #2*2
        smull           v10.4s,  v18.4h,  v1.4h
        smull2          v11.4s,  v18.8h,  v1.8h
        sxtl            v7.8h,   v7.8b
        ext             v20.16b, v16.16b, v17.16b, #2*3
        smull           v0.4s,   v19.4h,  v2.4h
        smull2          v1.4s,   v19.8h,  v2.8h
        ext             v21.16b, v16.16b, v17.16b, #2*4
        addp            v8.4s,   v8.4s,   v9.4s
        smull           v2.4s,   v20.4h,  v3.4h
        smull2          v3.4s,   v20.8h,  v3.8h
        ext             v22.16b, v16.16b, v17.16b, #2*5
        addp            v9.4s,   v10.4s,  v11.4s
        smull           v10.4s,  v21.4h,  v4.4h
        smull2          v11.4s,  v21.8h,  v4.8h
        ext             v23.16b, v16.16b, v17.16b, #2*6
        addp            v0.4s,   v0.4s,   v1.4s
        smull           v18.4s,  v22.4h,  v5.4h
        smull2          v19.4s,  v22.8h,  v5.8h
        ext             v16.16b, v16.16b, v17.16b, #2*7
        addp            v1.4s,   v2.4s,   v3.4s
        addp            v2.4s,   v10.4s,  v11.4s
        smull           v20.4s,  v23.4h,  v6.4h
        smull2          v21.4s,  v23.8h,  v6.8h
        addp            v3.4s,   v18.4s,  v19.4s
        smull           v22.4s,  v16.4h,  v7.4h
        smull2          v23.4s,  v16.8h,  v7.8h
        addp            v4.4s,   v20.4s,  v21.4s
        addp            v5.4s,   v22.4s,  v23.4s

        addp            v8.4s,   v8.4s,   v9.4s
        addp            v0.4s,   v0.4s,   v1.4s
        addp            v2.4s,   v2.4s,   v3.4s
        addp            v4.4s,   v4.4s,   v5.4s

        addp            v16.4s,  v8.4s,   v0.4s
        addp            v17.4s,  v2.4s,   v4.4s

        add             w5,  w5,  w8

        srshl           v16.4s,  v16.4s,  v14.4s // -(7 - intermediate_bits)
        srshl           v17.4s,  v17.4s,  v14.4s // -(7 - intermediate_bits)

        ret
endfunc

// void dav1d_warp_affine_8x8_16bpc_neon(
//         pixel *dst, const ptrdiff_t dst_stride,
//         const pixel *src, const ptrdiff_t src_stride,
//         const int16_t *const abcd, int mx, int my,
//         const int bitdepth_max)
.macro warp t
function warp_affine_8x8\t\()_16bpc_neon, export=1
        stp             d8,  d9,  [sp, #-0x40]!
        stp             d10, d11, [sp, #0x10]
        stp             d12, d13, [sp, #0x20]
        stp             d14, d15, [sp, #0x30]

.ifb \t
        dup             v15.8h,  w7        // bitdepth_max
.else
        movi            v15.8h,  #(PREP_BIAS >> 8), lsl #8
.endif
        clz             w7,  w7
                                           // intermediate_bits = clz(bitdepth_max) - 18
.ifb \t
        sub             w8,  w7,  #11      // 7 + intermediate_bits = clz(bitdepth_max) - 18 + 7
.endif
        sub             w7,  w7,  #25      // -(7 - intermediate_bits)
.ifb \t
        neg             w8,  w8            // -(7 + intermediate_bits)
.endif
        dup             v14.4s,  w7        // -(7 - intermediate_bits)
.ifb \t
        dup             v13.4s,  w8        // -(7 + intermediate_bits)
.endif

        ldr             x4,  [x4]
        sbfx            x7,  x4, #0,  #16
        sbfx            x8,  x4, #16, #16
        sbfx            x9,  x4, #32, #16
        sbfx            x4,  x4, #48, #16
        mov             w10, #8
        sub             x2,  x2,  x3, lsl #1
        sub             x2,  x2,  x3
        sub             x2,  x2,  #6
        movrel          x11, X(mc_warp_filter), 64*8
        mov             x15, x30
.ifnb \t
        lsl             x1,  x1,  #1
.endif

        bl              warp_filter_horz_neon
        uzp1            v24.8h,  v16.8h,  v17.8h // Same as xtn, xtn2
        bl              warp_filter_horz_neon
        uzp1            v25.8h,  v16.8h,  v17.8h // Ditto
        bl              warp_filter_horz_neon
        uzp1            v26.8h,  v16.8h,  v17.8h // Ditto
        bl              warp_filter_horz_neon
        uzp1            v27.8h,  v16.8h,  v17.8h // Ditto
        bl              warp_filter_horz_neon
        uzp1            v28.8h,  v16.8h,  v17.8h // Ditto
        bl              warp_filter_horz_neon
        uzp1            v29.8h,  v16.8h,  v17.8h // Ditto
        bl              warp_filter_horz_neon
        uzp1            v30.8h,  v16.8h,  v17.8h // Ditto

1:
        add             w14, w6,  #512
        bl              warp_filter_horz_neon
        uzp1            v31.8h,  v16.8h,  v17.8h // Same as xtn, xtn2

        load_filter_row d0, w14, w9
        load_filter_row d1, w14, w9
        load_filter_row d2, w14, w9
        load_filter_row d3, w14, w9
        load_filter_row d4, w14, w9
        load_filter_row d5, w14, w9
        load_filter_row d6, w14, w9
        load_filter_row d7, w14, w9
        transpose_8x8b_xtl v0, v1, v2, v3, v4, v5, v6, v7, sxtl

        // This ordering of smull/smlal/smull2/smlal2 is highly
        // beneficial for Cortex A53 here.
        smull           v16.4s,  v24.4h,  v0.4h
        smlal           v16.4s,  v25.4h,  v1.4h
        smlal           v16.4s,  v26.4h,  v2.4h
        smlal           v16.4s,  v27.4h,  v3.4h
        smlal           v16.4s,  v28.4h,  v4.4h
        smlal           v16.4s,  v29.4h,  v5.4h
        smlal           v16.4s,  v30.4h,  v6.4h
        smlal           v16.4s,  v31.4h,  v7.4h
        smull2          v17.4s,  v24.8h,  v0.8h
        smlal2          v17.4s,  v25.8h,  v1.8h
        smlal2          v17.4s,  v26.8h,  v2.8h
        smlal2          v17.4s,  v27.8h,  v3.8h
        smlal2          v17.4s,  v28.8h,  v4.8h
        smlal2          v17.4s,  v29.8h,  v5.8h
        smlal2          v17.4s,  v30.8h,  v6.8h
        smlal2          v17.4s,  v31.8h,  v7.8h

        mov             v24.16b, v25.16b
        mov             v25.16b, v26.16b
.ifb \t
        srshl           v16.4s,  v16.4s,  v13.4s // -(7 + intermediate_bits)
        srshl           v17.4s,  v17.4s,  v13.4s // -(7 + intermediate_bits)
.else
        rshrn           v16.4h,  v16.4s,  #7
        rshrn2          v16.8h,  v17.4s,  #7
.endif
        mov             v26.16b, v27.16b
.ifb \t
        sqxtun          v16.4h,  v16.4s
        sqxtun2         v16.8h,  v17.4s
.else
        sub             v16.8h,  v16.8h,  v15.8h // PREP_BIAS
.endif
        mov             v27.16b, v28.16b
        mov             v28.16b, v29.16b
.ifb \t
        umin            v16.8h,  v16.8h,  v15.8h // bitdepth_max
.endif
        mov             v29.16b, v30.16b
        mov             v30.16b, v31.16b
        subs            w10, w10, #1
        st1             {v16.8h}, [x0], x1

        add             w6,  w6,  w4
        b.gt            1b

        ldp             d14, d15, [sp, #0x30]
        ldp             d12, d13, [sp, #0x20]
        ldp             d10, d11, [sp, #0x10]
        ldp             d8,  d9,  [sp], 0x40

        ret             x15
endfunc
.endm

warp
warp t

// void dav1d_emu_edge_16bpc_neon(
//         const intptr_t bw, const intptr_t bh,
//         const intptr_t iw, const intptr_t ih,
//         const intptr_t x, const intptr_t y,
//         pixel *dst, const ptrdiff_t dst_stride,
//         const pixel *ref, const ptrdiff_t ref_stride)
function emu_edge_16bpc_neon, export=1
        ldp             x8,  x9,  [sp]

        // ref += iclip(y, 0, ih - 1) * PXSTRIDE(ref_stride)
        // ref += iclip(x, 0, iw - 1)
        sub             x12, x3,  #1           // ih - 1
        cmp             x5,  x3
        sub             x13, x2,  #1           // iw - 1
        csel            x12, x12, x5,  ge      // min(y, ih - 1)
        cmp             x4,  x2
        bic             x12, x12, x12, asr #63 // max(min(y, ih - 1), 0)
        csel            x13, x13, x4,  ge      // min(x, iw - 1)
        bic             x13, x13, x13, asr #63 // max(min(x, iw - 1), 0)
        madd            x8,  x12, x9,  x8      // ref += iclip() * stride
        add             x8,  x8,  x13, lsl #1  // ref += iclip()

        // bottom_ext = iclip(y + bh - ih, 0, bh - 1)
        // top_ext = iclip(-y, 0, bh - 1)
        add             x10, x5,  x1           // y + bh
        neg             x5,  x5                // -y
        sub             x10, x10, x3           // y + bh - ih
        sub             x12, x1,  #1           // bh - 1
        cmp             x10, x1
        bic             x5,  x5,  x5,  asr #63 // max(-y, 0)
        csel            x10, x10, x12, lt      // min(y + bh - ih, bh-1)
        cmp             x5,  x1
        bic             x10, x10, x10, asr #63 // max(min(y + bh - ih, bh-1), 0)
        csel            x5,  x5,  x12, lt      // min(max(-y, 0), bh-1)

        // right_ext = iclip(x + bw - iw, 0, bw - 1)
        // left_ext = iclip(-x, 0, bw - 1)
        add             x11, x4,  x0           // x + bw
        neg             x4,  x4                // -x
        sub             x11, x11, x2           // x + bw - iw
        sub             x13, x0,  #1           // bw - 1
        cmp             x11, x0
        bic             x4,  x4,  x4,  asr #63 // max(-x, 0)
        csel            x11, x11, x13, lt      // min(x + bw - iw, bw-1)
        cmp             x4,  x0
        bic             x11, x11, x11, asr #63 // max(min(x + bw - iw, bw-1), 0)
        csel            x4,  x4,  x13, lt      // min(max(-x, 0), bw - 1)

        // center_h = bh - top_ext - bottom_ext
        // dst += top_ext * PXSTRIDE(dst_stride)
        // center_w = bw - left_ext - right_ext
        sub             x1,  x1,  x5           // bh - top_ext
        madd            x6,  x5,  x7,  x6
        sub             x2,  x0,  x4           // bw - left_ext
        sub             x1,  x1,  x10          // center_h = bh - top_ext - bottom_ext
        sub             x2,  x2,  x11          // center_w = bw - left_ext - right_ext

        mov             x14, x6                // backup of dst

.macro v_loop need_left, need_right
0:
.if \need_left
        ld1r            {v0.8h}, [x8]
        mov             x12, x6                // out = dst
        mov             x3,  x4
        mov             v1.16b,  v0.16b
1:
        subs            x3,  x3,  #16
        st1             {v0.8h, v1.8h}, [x12], #32
        b.gt            1b
.endif
        mov             x13, x8
        add             x12, x6,  x4, lsl #1   // out = dst + left_ext
        mov             x3,  x2
1:
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x13], #64
        subs            x3,  x3,  #32
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x12], #64
        b.gt            1b
.if \need_right
        add             x3,  x8,  x2, lsl #1   // in + center_w
        sub             x3,  x3,  #2           // in + center_w - 1
        add             x12, x6,  x4, lsl #1   // dst + left_ext
        ld1r            {v0.8h}, [x3]
        add             x12, x12, x2, lsl #1   // out = dst + left_ext + center_w
        mov             x3,  x11
        mov             v1.16b,  v0.16b
1:
        subs            x3,  x3,  #16
        st1             {v0.8h, v1.8h}, [x12], #32
        b.gt            1b
.endif

        subs            x1,  x1,  #1           // center_h--
        add             x6,  x6,  x7
        add             x8,  x8,  x9
        b.gt            0b
.endm

        cbz             x4,  2f
        // need_left
        cbz             x11, 3f
        // need_left + need_right
        v_loop          1,   1
        b               5f

2:
        // !need_left
        cbz             x11, 4f
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

        cbz             x10, 3f
        // need_bottom
        sub             x8,  x6,  x7           // ref = dst - stride
        mov             x4,  x0
1:
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x8], #64
        mov             x3,  x10
2:
        subs            x3,  x3,  #1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x7
        b.gt            2b
        msub            x6,  x7,  x10,  x6     // dst -= bottom_ext * stride
        subs            x4,  x4,  #32          // bw -= 32
        add             x6,  x6,  #64          // dst += 32
        b.gt            1b

3:
        cbz             x5,  3f
        // need_top
        msub            x6,  x7,  x5,  x14     // dst = stored_dst - top_ext * stride
1:
        ld1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x14], #64
        mov             x3,  x5
2:
        subs            x3,  x3,  #1
        st1             {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x7
        b.gt            2b
        msub            x6,  x7,  x5,  x6      // dst -= top_ext * stride
        subs            x0,  x0,  #32          // bw -= 32
        add             x6,  x6,  #64          // dst += 32
        b.gt            1b

3:
        ret
endfunc
