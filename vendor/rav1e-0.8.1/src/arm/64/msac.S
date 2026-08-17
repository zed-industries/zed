/*
 * Copyright © 2019, VideoLAN and dav1d authors
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

#define BUF_POS 0
#define BUF_END 8
#define DIF 16
#define RNG 24
#define CNT 28
#define ALLOW_UPDATE_CDF 32

const coeffs
        .short 60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 4, 0
        .short 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0, 0
endconst

const bits
        .short   0x1,   0x2,   0x4,   0x8,   0x10,   0x20,   0x40,   0x80
        .short 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000
endconst

.macro ld1_n d0, d1, src, sz, n
.if \n <= 8
        ld1             {\d0\sz},  [\src]
.else
        ld1             {\d0\sz, \d1\sz},  [\src]
.endif
.endm

.macro st1_n s0, s1, dst, sz, n
.if \n <= 8
        st1             {\s0\sz},  [\dst]
.else
        st1             {\s0\sz, \s1\sz},  [\dst]
.endif
.endm

.macro ushr_n d0, d1, s0, s1, shift, sz, n
        ushr            \d0\sz,  \s0\sz,  \shift
.if \n == 16
        ushr            \d1\sz,  \s1\sz,  \shift
.endif
.endm

.macro add_n d0, d1, s0, s1, s2, s3, sz, n
        add             \d0\sz,  \s0\sz,  \s2\sz
.if \n == 16
        add             \d1\sz,  \s1\sz,  \s3\sz
.endif
.endm

.macro sub_n d0, d1, s0, s1, s2, s3, sz, n
        sub             \d0\sz,  \s0\sz,  \s2\sz
.if \n == 16
        sub             \d1\sz,  \s1\sz,  \s3\sz
.endif
.endm

.macro and_n d0, d1, s0, s1, s2, s3, sz, n
        and             \d0\sz,  \s0\sz,  \s2\sz
.if \n == 16
        and             \d1\sz,  \s1\sz,  \s3\sz
.endif
.endm

.macro cmhs_n d0, d1, s0, s1, s2, s3, sz, n
        cmhs            \d0\sz,  \s0\sz,  \s2\sz
.if \n == 16
        cmhs            \d1\sz,  \s1\sz,  \s3\sz
.endif
.endm

.macro urhadd_n d0, d1, s0, s1, s2, s3, sz, n
        urhadd          \d0\sz,  \s0\sz,  \s2\sz
.if \n == 16
        urhadd          \d1\sz,  \s1\sz,  \s3\sz
.endif
.endm

.macro sshl_n d0, d1, s0, s1, s2, s3, sz, n
        sshl            \d0\sz,  \s0\sz,  \s2\sz
.if \n == 16
        sshl            \d1\sz,  \s1\sz,  \s3\sz
.endif
.endm

.macro sqdmulh_n d0, d1, s0, s1, s2, s3, sz, n
        sqdmulh         \d0\sz,  \s0\sz,  \s2\sz
.if \n == 16
        sqdmulh         \d1\sz,  \s1\sz,  \s3\sz
.endif
.endm

.macro str_n            idx0, idx1, dstreg, dstoff, n
        str             \idx0,  [\dstreg, \dstoff]
.if \n == 16
        str             \idx1,  [\dstreg, \dstoff + 16]
.endif
.endm

// unsigned dav1d_msac_decode_symbol_adapt4_neon(MsacContext *s, uint16_t *cdf,
//                                               size_t n_symbols);

function msac_decode_symbol_adapt4_neon, export=1
.macro decode_update sz, szb, n
        sub             sp,  sp,  #48
        add             x8,  x0,  #RNG
        ld1_n           v0,  v1,  x1,  \sz, \n                    // cdf
        ld1r            {v4\sz},  [x8]                            // rng
        movrel          x9,  coeffs, 30
        movi            v31\sz, #0x7f, lsl #8                     // 0x7f00
        sub             x9,  x9,  x2, lsl #1
        mvni            v30\sz, #0x3f                             // 0xffc0
        and             v7\szb, v4\szb, v31\szb                   // rng & 0x7f00
        str             h4,  [sp, #14]                            // store original u = s->rng
        and_n           v2,  v3,  v0,  v1,  v30, v30, \szb, \n    // cdf & 0xffc0

        ld1_n           v4,  v5,  x9,  \sz, \n                    // EC_MIN_PROB * (n_symbols - ret)
        sqdmulh_n       v6,  v7,  v2,  v3,  v7,  v7,  \sz, \n     // ((cdf >> EC_PROB_SHIFT) * (r - 128)) >> 1
        add             x8,  x0,  #DIF + 6

        add_n           v4,  v5,  v2,  v3,  v4,  v5,  \sz, \n     // v = cdf + EC_MIN_PROB * (n_symbols - ret)
        add_n           v4,  v5,  v6,  v7,  v4,  v5,  \sz, \n     // v = ((cdf >> EC_PROB_SHIFT) * r) >> 1 + EC_MIN_PROB * (n_symbols - ret)

        ld1r            {v6.8h},  [x8]                            // dif >> (EC_WIN_SIZE - 16)
        movrel          x8,  bits
        str_n           q4,  q5,  sp, #16, \n                     // store v values to allow indexed access

        ld1_n           v16, v17, x8,  .8h, \n

        cmhs_n          v2,  v3,  v6,  v6,  v4,  v5,  .8h,  \n    // c >= v

        and_n           v6,  v7,  v2,  v3,  v16, v17, .16b, \n    // One bit per halfword set in the mask
.if \n == 16
        add             v6.8h,  v6.8h,  v7.8h
.endif
        addv            h6,  v6.8h                                // Aggregate mask bits
        ldr             w4,  [x0, #ALLOW_UPDATE_CDF]
        umov            w3,  v6.h[0]
        rbit            w3,  w3
        clz             w15, w3                                   // ret

        cbz             w4,  L(renorm)
        // update_cdf
        ldrh            w3,  [x1, x2, lsl #1]                     // count = cdf[n_symbols]
        movi            v5\szb, #0xff
.if \n == 16
        mov             w4,  #-5
.else
        mvn             w14, w2
        mov             w4,  #-4
        cmn             w14, #3                                   // set C if n_symbols <= 2
.endif
        urhadd_n        v4,  v5,  v5,  v5,  v2,  v3,  \sz, \n     // i >= val ? -1 : 32768
.if \n == 16
        sub             w4,  w4,  w3, lsr #4                      // -((count >> 4) + 5)
.else
        lsr             w14, w3,  #4                              // count >> 4
        sbc             w4,  w4,  w14                             // -((count >> 4) + (n_symbols > 2) + 4)
.endif
        sub_n           v4,  v5,  v4,  v5,  v0,  v1,  \sz, \n     // (32768 - cdf[i]) or (-1 - cdf[i])
        dup             v6\sz,    w4                              // -rate

        sub             w3,  w3,  w3, lsr #5                      // count - (count == 32)
        sub_n           v0,  v1,  v0,  v1,  v2,  v3,  \sz, \n     // cdf + (i >= val ? 1 : 0)
        sshl_n          v4,  v5,  v4,  v5,  v6,  v6,  \sz, \n     // ({32768,-1} - cdf[i]) >> rate
        add             w3,  w3,  #1                              // count + (count < 32)
        add_n           v0,  v1,  v0,  v1,  v4,  v5,  \sz, \n     // cdf + (32768 - cdf[i]) >> rate
        st1_n           v0,  v1,  x1,  \sz, \n
        strh            w3,  [x1, x2, lsl #1]
.endm

        decode_update   .4h, .8b, 4

L(renorm):
        add             x8,  sp,  #16
        add             x8,  x8,  w15, uxtw #1
        ldrh            w3,  [x8]              // v
        ldurh           w4,  [x8, #-2]         // u
        ldr             w6,  [x0, #CNT]
        ldr             x7,  [x0, #DIF]
        sub             w4,  w4,  w3           // rng = u - v
        clz             w5,  w4                // clz(rng)
        eor             w5,  w5,  #16          // d = clz(rng) ^ 16
        mvn             x7,  x7                // ~dif
        add             x7,  x7,  x3, lsl #48  // ~dif + (v << 48)
L(renorm2):
        lsl             w4,  w4,  w5           // rng << d
        subs            w6,  w6,  w5           // cnt -= d
        lsl             x7,  x7,  x5           // (~dif + (v << 48)) << d
        str             w4,  [x0, #RNG]
        mvn             x7,  x7                // ~dif
        b.hs            9f

        // refill
        ldp             x3,  x4,  [x0]         // BUF_POS, BUF_END
        add             x5,  x3,  #8
        cmp             x5,  x4
        b.gt            2f

        ldr             x3,  [x3]              // next_bits
        add             w8,  w6,  #23          // shift_bits = cnt + 23
        add             w6,  w6,  #16          // cnt += 16
        rev             x3,  x3                // next_bits = bswap(next_bits)
        sub             x5,  x5,  x8, lsr #3   // buf_pos -= shift_bits >> 3
        and             w8,  w8,  #24          // shift_bits &= 24
        lsr             x3,  x3,  x8           // next_bits >>= shift_bits
        sub             w8,  w8,  w6           // shift_bits -= 16 + cnt
        str             x5,  [x0, #BUF_POS]
        lsl             x3,  x3,  x8           // next_bits <<= shift_bits
        mov             w4,  #48
        sub             w6,  w4,  w8           // cnt = cnt + 64 - shift_bits
        eor             x7,  x7,  x3           // dif ^= next_bits
        b               9f

2:      // refill_eob
        mov             w14, #40
        sub             w5,  w14, w6           // c = 40 - cnt
3:
        cmp             x3,  x4
        b.ge            4f
        ldrb            w8,  [x3], #1
        lsl             x8,  x8,  x5
        eor             x7,  x7,  x8
        subs            w5,  w5,  #8
        b.ge            3b

4:      // refill_eob_end
        str             x3,  [x0, #BUF_POS]
        sub             w6,  w14, w5           // cnt = 40 - c

9:
        str             w6,  [x0, #CNT]
        str             x7,  [x0, #DIF]

        mov             w0,  w15
        add             sp,  sp,  #48
        ret
endfunc

function msac_decode_symbol_adapt8_neon, export=1
        decode_update   .8h, .16b, 8
        b               L(renorm)
endfunc

function msac_decode_symbol_adapt16_neon, export=1
        decode_update   .8h, .16b, 16
        b               L(renorm)
endfunc

function msac_decode_hi_tok_neon, export=1
        ld1             {v0.4h},  [x1]            // cdf
        add             x16, x0,  #RNG
        movi            v31.4h, #0x7f, lsl #8     // 0x7f00
        movrel          x17, coeffs, 30-2*3
        mvni            v30.4h, #0x3f             // 0xffc0
        ldrh            w9,  [x1, #6]             // count = cdf[n_symbols]
        ld1r            {v3.4h},  [x16]           // rng
        movrel          x16, bits
        ld1             {v29.4h}, [x17]           // EC_MIN_PROB * (n_symbols - ret)
        add             x17, x0,  #DIF + 6
        ld1             {v16.8h}, [x16]
        mov             w13, #-24
        and             v17.8b,  v0.8b,   v30.8b  // cdf & 0xffc0
        ldr             w10, [x0, #ALLOW_UPDATE_CDF]
        ld1r            {v1.8h},  [x17]           // dif >> (EC_WIN_SIZE - 16)
        sub             sp,  sp,  #48
        ldr             w6,  [x0, #CNT]
        ldr             x7,  [x0, #DIF]
1:
        and             v7.8b,   v3.8b,   v31.8b  // rng & 0x7f00
        sqdmulh         v6.4h,   v17.4h,  v7.4h   // ((cdf >> EC_PROB_SHIFT) * (r - 128)) >> 1
        add             v4.4h,   v17.4h,  v29.4h  // v = cdf + EC_MIN_PROB * (n_symbols - ret)
        add             v4.4h,   v6.4h,   v4.4h   // v = ((cdf >> EC_PROB_SHIFT) * r) >> 1 + EC_MIN_PROB * (n_symbols - ret)
        str             h3,  [sp, #14]            // store original u = s->rng
        cmhs            v2.8h,   v1.8h,   v4.8h   // c >= v
        str             q4,  [sp, #16]            // store v values to allow indexed access
        and             v6.16b,  v2.16b,  v16.16b // One bit per halfword set in the mask
        addv            h6,  v6.8h                // Aggregate mask bits
        umov            w3,  v6.h[0]
        add             w13, w13, #5
        rbit            w3,  w3
        add             x8,  sp,  #16
        clz             w15, w3                   // ret

        cbz             w10, 2f
        // update_cdf
        movi            v5.8b, #0xff
        mov             w4,  #-5
        urhadd          v4.4h,   v5.4h,   v2.4h   // i >= val ? -1 : 32768
        sub             w4,  w4,  w9, lsr #4      // -((count >> 4) + 5)
        sub             v4.4h,   v4.4h,   v0.4h   // (32768 - cdf[i]) or (-1 - cdf[i])
        dup             v6.4h,    w4              // -rate

        sub             w9,  w9,  w9, lsr #5      // count - (count == 32)
        sub             v0.4h,   v0.4h,   v2.4h   // cdf + (i >= val ? 1 : 0)
        sshl            v4.4h,   v4.4h,   v6.4h   // ({32768,-1} - cdf[i]) >> rate
        add             w9,  w9,  #1              // count + (count < 32)
        add             v0.4h,   v0.4h,   v4.4h   // cdf + (32768 - cdf[i]) >> rate
        st1             {v0.4h},  [x1]
        and             v17.8b,  v0.8b,   v30.8b  // cdf & 0xffc0
        strh            w9,  [x1, #6]

2:
        add             x8,  x8,  w15, uxtw #1
        ldrh            w3,  [x8]              // v
        ldurh           w4,  [x8, #-2]         // u
        sub             w4,  w4,  w3           // rng = u - v
        clz             w5,  w4                // clz(rng)
        eor             w5,  w5,  #16          // d = clz(rng) ^ 16
        mvn             x7,  x7                // ~dif
        add             x7,  x7,  x3, lsl #48  // ~dif + (v << 48)
        lsl             w4,  w4,  w5           // rng << d
        subs            w6,  w6,  w5           // cnt -= d
        lsl             x7,  x7,  x5           // (~dif + (v << 48)) << d
        str             w4,  [x0, #RNG]
        dup             v3.4h,   w4
        mvn             x7,  x7                // ~dif
        b.hs            9f

        // refill
        ldp             x3,  x4,  [x0]         // BUF_POS, BUF_END
        add             x5,  x3,  #8
        cmp             x5,  x4
        b.gt            2f

        ldr             x3,  [x3]              // next_bits
        add             w8,  w6,  #23          // shift_bits = cnt + 23
        add             w6,  w6,  #16          // cnt += 16
        rev             x3,  x3                // next_bits = bswap(next_bits)
        sub             x5,  x5,  x8, lsr #3   // buf_pos -= shift_bits >> 3
        and             w8,  w8,  #24          // shift_bits &= 24
        lsr             x3,  x3,  x8           // next_bits >>= shift_bits
        sub             w8,  w8,  w6           // shift_bits -= 16 + cnt
        str             x5,  [x0, #BUF_POS]
        lsl             x3,  x3,  x8           // next_bits <<= shift_bits
        mov             w4,  #48
        sub             w6,  w4,  w8           // cnt = cnt + 64 - shift_bits
        eor             x7,  x7,  x3           // dif ^= next_bits
        b               9f

2:      // refill_eob
        mov             w14, #40
        sub             w5,  w14, w6           // c = 40 - cnt
3:
        cmp             x3,  x4
        b.ge            4f
        ldrb            w8,  [x3], #1
        lsl             x8,  x8,  x5
        eor             x7,  x7,  x8
        subs            w5,  w5,  #8
        b.ge            3b

4:      // refill_eob_end
        str             x3,  [x0, #BUF_POS]
        sub             w6,  w14, w5           // cnt = 40 - c

9:
        lsl             w15, w15, #1
        sub             w15, w15, #5
        lsr             x12, x7,  #48
        adds            w13, w13, w15          // carry = tok_br < 3 || tok == 15
        dup             v1.8h,   w12
        b.cc            1b                     // loop if !carry
        add             w13, w13, #30
        str             w6,  [x0, #CNT]
        add             sp,  sp,  #48
        str             x7,  [x0, #DIF]
        lsr             w0,  w13, #1
        ret
endfunc

function msac_decode_bool_equi_neon, export=1
        ldp             w5,  w6,  [x0, #RNG]   // + CNT
        sub             sp,  sp,  #48
        ldr             x7,  [x0, #DIF]
        bic             w4,  w5,  #0xff        // r &= 0xff00
        add             w4,  w4,  #8
        subs            x8,  x7,  x4, lsl #47  // dif - vw
        lsr             w4,  w4,  #1           // v
        sub             w5,  w5,  w4           // r - v
        cset            w15, lo
        csel            w4,  w5,  w4,  hs      // if (ret) v = r - v;
        csel            x7,  x8,  x7,  hs      // if (ret) dif = dif - vw;

        clz             w5,  w4                // clz(rng)
        mvn             x7,  x7                // ~dif
        eor             w5,  w5,  #16          // d = clz(rng) ^ 16
        b               L(renorm2)
endfunc

function msac_decode_bool_neon, export=1
        ldp             w5,  w6,  [x0, #RNG]   // + CNT
        sub             sp,  sp,  #48
        ldr             x7,  [x0, #DIF]
        lsr             w4,  w5,  #8           // r >> 8
        bic             w1,  w1,  #0x3f        // f &= ~63
        mul             w4,  w4,  w1
        lsr             w4,  w4,  #7
        add             w4,  w4,  #4           // v
        subs            x8,  x7,  x4, lsl #48  // dif - vw
        sub             w5,  w5,  w4           // r - v
        cset            w15, lo
        csel            w4,  w5,  w4,  hs      // if (ret) v = r - v;
        csel            x7,  x8,  x7,  hs      // if (ret) dif = dif - vw;

        clz             w5,  w4                // clz(rng)
        mvn             x7,  x7                // ~dif
        eor             w5,  w5,  #16          // d = clz(rng) ^ 16
        b               L(renorm2)
endfunc

function msac_decode_bool_adapt_neon, export=1
        ldr             w9,  [x1]              // cdf[0-1]
        ldp             w5,  w6,  [x0, #RNG]   // + CNT
        sub             sp,  sp,  #48
        ldr             x7,  [x0, #DIF]
        lsr             w4,  w5,  #8           // r >> 8
        and             w2,  w9,  #0xffc0      // f &= ~63
        mul             w4,  w4,  w2
        lsr             w4,  w4,  #7
        add             w4,  w4,  #4           // v
        subs            x8,  x7,  x4, lsl #48  // dif - vw
        sub             w5,  w5,  w4           // r - v
        cset            w15, lo
        csel            w4,  w5,  w4,  hs      // if (ret) v = r - v;
        csel            x7,  x8,  x7,  hs      // if (ret) dif = dif - vw;

        ldr             w10, [x0, #ALLOW_UPDATE_CDF]

        clz             w5,  w4                // clz(rng)
        mvn             x7,  x7                // ~dif
        eor             w5,  w5,  #16          // d = clz(rng) ^ 16

        cbz             w10, L(renorm2)

        lsr             w2,  w9,  #16          // count = cdf[1]
        and             w9,  w9,  #0xffff      // cdf[0]

        sub             w3,  w2,  w2, lsr #5   // count - (count >= 32)
        lsr             w2,  w2,  #4           // count >> 4
        add             w10, w3,  #1           // count + (count < 32)
        add             w2,  w2,  #4           // rate = (count >> 4) | 4

        sub             w9,  w9,  w15          // cdf[0] -= bit
        sub             w11, w9,  w15, lsl #15 // {cdf[0], cdf[0] - 32769}
        asr             w11, w11, w2           // {cdf[0], cdf[0] - 32769} >> rate
        sub             w9,  w9,  w11          // cdf[0]

        strh            w9,  [x1]
        strh            w10, [x1, #2]

        b               L(renorm2)
endfunc
