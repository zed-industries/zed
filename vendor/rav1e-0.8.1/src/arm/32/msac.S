/*
 * Copyright © 2019, VideoLAN and dav1d authors
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

#define BUF_POS 0
#define BUF_END 4
#define DIF 8
#define RNG 12
#define CNT 16
#define ALLOW_UPDATE_CDF 20

const coeffs
        .short 60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 4, 0
        .short 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0, 0
endconst

const bits, align=4
        .short   0x1,   0x2,   0x4,   0x8,   0x10,   0x20,   0x40,   0x80
        .short 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000
endconst

.macro vld1_align_n d0, q0, q1, src, n
.if \n == 4
        vld1.16         {\d0},  [\src, :64]
.elseif \n == 8
        vld1.16         {\q0},  [\src, :128]
.else
        vld1.16         {\q0, \q1},  [\src, :128]
.endif
.endm

.macro vld1_n d0, q0, q1, src, n
.if \n == 4
        vld1.16         {\d0},  [\src]
.elseif \n == 8
        vld1.16         {\q0},  [\src]
.else
        vld1.16         {\q0, \q1},  [\src]
.endif
.endm

.macro vst1_align_n d0, q0, q1, src, n
.if \n == 4
        vst1.16         {\d0},  [\src, :64]
.elseif \n == 8
        vst1.16         {\q0},  [\src, :128]
.else
        vst1.16         {\q0, \q1},  [\src, :128]
.endif
.endm

.macro vst1_n d0, q0, q1, src, n
.if \n == 4
        vst1.16         {\d0},  [\src]
.elseif \n == 8
        vst1.16         {\q0},  [\src]
.else
        vst1.16         {\q0, \q1},  [\src]
.endif
.endm

.macro vshr_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vshr.u16        \d0,  \s0,  \s3
.else
        vshr.u16        \d1,  \s1,  \s4
.if \n == 16
        vshr.u16        \d2,  \s2,  \s5
.endif
.endif
.endm

.macro vadd_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vadd.i16        \d0,  \s0,  \s3
.else
        vadd.i16        \d1,  \s1,  \s4
.if \n == 16
        vadd.i16        \d2,  \s2,  \s5
.endif
.endif
.endm

.macro vsub_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vsub.i16        \d0,  \s0,  \s3
.else
        vsub.i16        \d1,  \s1,  \s4
.if \n == 16
        vsub.i16        \d2,  \s2,  \s5
.endif
.endif
.endm

.macro vand_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vand            \d0,  \s0,  \s3
.else
        vand            \d1,  \s1,  \s4
.if \n == 16
        vand            \d2,  \s2,  \s5
.endif
.endif
.endm

.macro vcge_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vcge.u16        \d0,  \s0,  \s3
.else
        vcge.u16        \d1,  \s1,  \s4
.if \n == 16
        vcge.u16        \d2,  \s2,  \s5
.endif
.endif
.endm

.macro vrhadd_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vrhadd.u16      \d0,  \s0,  \s3
.else
        vrhadd.u16      \d1,  \s1,  \s4
.if \n == 16
        vrhadd.u16      \d2,  \s2,  \s5
.endif
.endif
.endm

.macro vshl_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vshl.s16        \d0,  \s0,  \s3
.else
        vshl.s16        \d1,  \s1,  \s4
.if \n == 16
        vshl.s16        \d2,  \s2,  \s5
.endif
.endif
.endm

.macro vqdmulh_n d0, d1, d2, s0, s1, s2, s3, s4, s5, n
.if \n == 4
        vqdmulh.s16     \d0,  \s0,  \s3
.else
        vqdmulh.s16     \d1,  \s1,  \s4
.if \n == 16
        vqdmulh.s16     \d2,  \s2,  \s5
.endif
.endif
.endm

// unsigned dav1d_msac_decode_symbol_adapt4_neon(MsacContext *s, uint16_t *cdf,
//                                               size_t n_symbols);

function msac_decode_symbol_adapt4_neon, export=1
.macro decode_update n
        push            {r4-r10,lr}
        sub             sp,  sp,  #48
        add             r8,  r0,  #RNG

        vld1_align_n    d0,  q0,  q1,  r1,  \n                         // cdf
        vld1.16         {d16[]}, [r8, :16]                             // rng
        movrel_local    r9,  coeffs, 30
        vmov.i16        d30, #0x7f00                                   // 0x7f00
        sub             r9,  r9,  r2, lsl #1
        vmvn.i16        q14, #0x3f                                     // 0xffc0
        add             r8,  sp,  #14
        vand            d22, d16, d30                                  // rng & 0x7f00
        vst1.16         {d16[0]}, [r8, :16]                            // store original u = s->rng
        vand_n          d4,  q2,  q3,  d0,  q0,  q1, d28, q14, q14, \n // cdf & 0xffc0
.if \n > 4
        vmov            d23, d22
.endif

        vld1_n          d16, q8,  q9,  r9,  \n                          // EC_MIN_PROB * (n_symbols - ret)
        vqdmulh_n       d20, q10, q11, d4,  q2,  q3,  d22, q11, q11, \n // ((cdf >> EC_PROB_SHIFT) * (r - 128)) >> 1
        add             r8,  r0,  #DIF + 2

        vadd_n          d16, q8,  q9,  d4,  q2,  q3,  d16, q8,  q9,  \n // v = cdf + EC_MIN_PROB * (n_symbols - ret)
.if \n == 4
        vmov.i16        d17, #0
.endif
        vadd_n          d16, q8,  q9,  d20, q10, q11, d16, q8,  q9,  \n // v = ((cdf >> EC_PROB_SHIFT) * r) >> 1 + EC_MIN_PROB * (n_symbols - ret)

        add             r9,  sp,  #16
        vld1.16         {d20[]}, [r8, :16]                              // dif >> (EC_WIN_SIZE - 16)
        movrel_local    r8,  bits
        vst1_n          q8,  q8,  q9,  r9,  \n                          // store v values to allow indexed access

        vmov            d21, d20
        vld1_align_n    q12, q12, q13, r8,  \n
.if \n == 16
        vmov            q11, q10
.endif

        vcge_n          q2,  q2,  q3,  q10, q10, q11, q8,  q8,  q9,  \n // c >= v

        vand_n          q10, q10, q11, q2,  q2,  q3,  q12, q12, q13, \n // One bit per halfword set in the mask
.if \n == 16
        vadd.i16        q10, q10, q11
.endif
        vadd.i16        d20, d20, d21                                   // Aggregate mask bits
        ldr             r4,  [r0, #ALLOW_UPDATE_CDF]
        vpadd.i16       d20, d20, d20
        lsl             r10, r2,  #1
        vpadd.i16       d20, d20, d20
        vmov.u16        r3,  d20[0]
        cmp             r4,  #0
        rbit            r3,  r3
        clz             lr,  r3                                         // ret

        beq             L(renorm)
        // update_cdf
        ldrh            r3,  [r1, r10]                                  // count = cdf[n_symbols]
        vmov.i8         q10, #0xff
.if \n == 16
        mov             r4,  #-5
.else
        mvn             r12, r2
        mov             r4,  #-4
        cmn             r12, #3                                         // set C if n_symbols <= 2
.endif
        vrhadd_n        d16, q8,  q9,  d20, q10, q10, d4,  q2,  q3,  \n // i >= val ? -1 : 32768
.if \n == 16
        sub             r4,  r4,  r3, lsr #4                            // -((count >> 4) + 5)
.else
        lsr             r12, r3,  #4                                    // count >> 4
        sbc             r4,  r4,  r12                                   // -((count >> 4) + (n_symbols > 2) + 4)
.endif
        vsub_n          d16, q8,  q9,  d16, q8,  q9,  d0,  q0,  q1,  \n // (32768 - cdf[i]) or (-1 - cdf[i])
.if \n == 4
        vdup.16         d20, r4                                         // -rate
.else
        vdup.16         q10, r4                                         // -rate
.endif

        sub             r3,  r3,  r3, lsr #5                            // count - (count == 32)
        vsub_n          d0,  q0,  q1,  d0,  q0,  q1,  d4,  q2,  q3,  \n // cdf + (i >= val ? 1 : 0)
        vshl_n          d16, q8,  q9,  d16, q8,  q9,  d20, q10, q10, \n // ({32768,-1} - cdf[i]) >> rate
        add             r3,  r3,  #1                                    // count + (count < 32)
        vadd_n          d0,  q0,  q1,  d0,  q0,  q1,  d16, q8,  q9,  \n // cdf + (32768 - cdf[i]) >> rate
        vst1_align_n    d0,  q0,  q1,  r1,  \n
        strh            r3,  [r1, r10]
.endm

        decode_update   4

L(renorm):
        add             r8,  sp,  #16
        add             r8,  r8,  lr, lsl #1
        ldrh            r3,  [r8]              // v
        ldrh            r4,  [r8, #-2]         // u
        ldr             r6,  [r0, #CNT]
        ldr             r7,  [r0, #DIF]
        sub             r4,  r4,  r3           // rng = u - v
        clz             r5,  r4                // clz(rng)
        eor             r5,  r5,  #16          // d = clz(rng) ^ 16
        mvn             r7,  r7                // ~dif
        add             r7,  r7,  r3, lsl #16  // ~dif + (v << 16)
L(renorm2):
        lsl             r4,  r4,  r5           // rng << d
        subs            r6,  r6,  r5           // cnt -= d
        lsl             r7,  r7,  r5           // (~dif + (v << 16)) << d
        str             r4,  [r0, #RNG]
        mvn             r7,  r7                // ~dif
        bhs             9f

        // refill
        ldr             r3,  [r0, #BUF_POS]    // BUF_POS
        ldr             r4,  [r0, #BUF_END]    // BUF_END
        add             r5,  r3,  #4
        cmp             r5,  r4
        bgt             2f

        ldr             r3,  [r3]              // next_bits
        add             r8,  r6,  #23          // shift_bits = cnt + 23
        add             r6,  r6,  #16          // cnt += 16
        rev             r3,  r3                // next_bits = bswap(next_bits)
        sub             r5,  r5,  r8, lsr #3   // buf_pos -= shift_bits >> 3
        and             r8,  r8,  #24          // shift_bits &= 24
        lsr             r3,  r3,  r8           // next_bits >>= shift_bits
        sub             r8,  r8,  r6           // shift_bits -= 16 + cnt
        str             r5,  [r0, #BUF_POS]
        lsl             r3,  r3,  r8           // next_bits <<= shift_bits
        rsb             r6,  r8,  #16          // cnt = cnt + 32 - shift_bits
        eor             r7,  r7,  r3           // dif ^= next_bits
        b               9f

2:      // refill_eob
        rsb             r5,  r6,  #8           // c = 8 - cnt
3:
        cmp             r3,  r4
        bge             4f
        ldrb            r8,  [r3], #1
        lsl             r8,  r8,  r5
        eor             r7,  r7,  r8
        subs            r5,  r5,  #8
        bge             3b

4:      // refill_eob_end
        str             r3,  [r0, #BUF_POS]
        rsb             r6,  r5,  #8           // cnt = 8 - c

9:
        str             r6,  [r0, #CNT]
        str             r7,  [r0, #DIF]

        mov             r0,  lr
        add             sp,  sp,  #48

        pop             {r4-r10,pc}
endfunc

function msac_decode_symbol_adapt8_neon, export=1
        decode_update   8
        b               L(renorm)
endfunc

function msac_decode_symbol_adapt16_neon, export=1
        decode_update   16
        b               L(renorm)
endfunc

function msac_decode_hi_tok_neon, export=1
        push            {r4-r10,lr}
        vld1.16         {d0},  [r1, :64]       // cdf
        add             r4,  r0,  #RNG
        vmov.i16        d31, #0x7f00           // 0x7f00
        movrel_local    r5,  coeffs, 30-2*3
        vmvn.i16        d30, #0x3f             // 0xffc0
        ldrh            r9,  [r1, #6]          // count = cdf[n_symbols]
        vld1.16         {d1[]},  [r4, :16]     // rng
        movrel_local    r4,  bits
        vld1.16         {d29}, [r5]            // EC_MIN_PROB * (n_symbols - ret)
        add             r5,  r0,  #DIF + 2
        vld1.16         {q8}, [r4, :128]
        mov             r2,  #-24
        vand            d20, d0, d30           // cdf & 0xffc0
        ldr             r10, [r0, #ALLOW_UPDATE_CDF]
        vld1.16         {d2[]}, [r5, :16]      // dif >> (EC_WIN_SIZE - 16)
        sub             sp,  sp,  #48
        ldr             r6,  [r0, #CNT]
        ldr             r7,  [r0, #DIF]
        vmov            d3,  d2
1:
        vand            d23, d1,  d31          // rng & 0x7f00
        vqdmulh.s16     d18, d20, d23          // ((cdf >> EC_PROB_SHIFT) * (r - 128)) >> 1
        add             r12, sp,  #14
        vadd.i16        d6,  d20, d29          // v = cdf + EC_MIN_PROB * (n_symbols - ret)
        vadd.i16        d6,  d18, d6           // v = ((cdf >> EC_PROB_SHIFT) * r) >> 1 + EC_MIN_PROB * (n_symbols - ret)
        vmov.i16        d7,  #0
        vst1.16         {d1[0]}, [r12, :16]    // store original u = s->rng
        add             r12, sp,  #16
        vcge.u16        q2,  q1,  q3           // c >= v
        vst1.16         {q3},  [r12]           // store v values to allow indexed access
        vand            q9,  q2,  q8           // One bit per halfword set in the mask

        vadd.i16        d18, d18, d19          // Aggregate mask bits
        vpadd.i16       d18, d18, d18
        vpadd.i16       d18, d18, d18
        vmov.u16        r3,  d18[0]
        cmp             r10, #0
        add             r2,  r2,  #5
        rbit            r3,  r3
        add             r8,  sp,  #16
        clz             lr,  r3                // ret

        beq             2f
        // update_cdf
        vmov.i8         d22, #0xff
        mov             r4,  #-5
        vrhadd.u16      d6,  d22, d4           // i >= val ? -1 : 32768
        sub             r4,  r4,  r9, lsr #4   // -((count >> 4) + 5)
        vsub.i16        d6,  d6,  d0           // (32768 - cdf[i]) or (-1 - cdf[i])
        vdup.16         d18, r4                // -rate

        sub             r9,  r9,  r9, lsr #5   // count - (count == 32)
        vsub.i16        d0,  d0,  d4           // cdf + (i >= val ? 1 : 0)
        vshl.s16        d6,  d6,  d18          // ({32768,-1} - cdf[i]) >> rate
        add             r9,  r9,  #1           // count + (count < 32)
        vadd.i16        d0,  d0,  d6           // cdf + (32768 - cdf[i]) >> rate
        vst1.16         {d0},  [r1, :64]
        vand            d20, d0,  d30          // cdf & 0xffc0
        strh            r9,  [r1, #6]

2:
        add             r8,  r8,  lr, lsl #1
        ldrh            r3,  [r8]              // v
        ldrh            r4,  [r8, #-2]         // u
        sub             r4,  r4,  r3           // rng = u - v
        clz             r5,  r4                // clz(rng)
        eor             r5,  r5,  #16          // d = clz(rng) ^ 16
        mvn             r7,  r7                // ~dif
        add             r7,  r7,  r3, lsl #16  // ~dif + (v << 16)
        lsl             r4,  r4,  r5           // rng << d
        subs            r6,  r6,  r5           // cnt -= d
        lsl             r7,  r7,  r5           // (~dif + (v << 16)) << d
        str             r4,  [r0, #RNG]
        vdup.16         d1,  r4
        mvn             r7,  r7                // ~dif
        bhs             9f

        // refill
        ldr             r3,  [r0, #BUF_POS]    // BUF_POS
        ldr             r4,  [r0, #BUF_END]    // BUF_END
        add             r5,  r3,  #4
        cmp             r5,  r4
        bgt             2f

        ldr             r3,  [r3]              // next_bits
        add             r8,  r6,  #23          // shift_bits = cnt + 23
        add             r6,  r6,  #16          // cnt += 16
        rev             r3,  r3                // next_bits = bswap(next_bits)
        sub             r5,  r5,  r8, lsr #3   // buf_pos -= shift_bits >> 3
        and             r8,  r8,  #24          // shift_bits &= 24
        lsr             r3,  r3,  r8           // next_bits >>= shift_bits
        sub             r8,  r8,  r6           // shift_bits -= 16 + cnt
        str             r5,  [r0, #BUF_POS]
        lsl             r3,  r3,  r8           // next_bits <<= shift_bits
        rsb             r6,  r8,  #16          // cnt = cnt + 32 - shift_bits
        eor             r7,  r7,  r3           // dif ^= next_bits
        b               9f

2:      // refill_eob
        rsb             r5,  r6,  #8           // c = 40 - cnt
3:
        cmp             r3,  r4
        bge             4f
        ldrb            r8,  [r3], #1
        lsl             r8,  r8,  r5
        eor             r7,  r7,  r8
        subs            r5,  r5,  #8
        bge             3b

4:      // refill_eob_end
        str             r3,  [r0, #BUF_POS]
        rsb             r6,  r5,  #8           // cnt = 40 - c

9:
        lsl             lr,  lr,  #1
        sub             lr,  lr,  #5
        lsr             r12, r7,  #16
        adds            r2,  r2,  lr           // carry = tok_br < 3 || tok == 15
        vdup.16         q1,  r12
        bcc             1b                     // loop if !carry
        add             r2,  r2,  #30
        str             r6,  [r0, #CNT]
        add             sp,  sp,  #48
        str             r7,  [r0, #DIF]
        lsr             r0,  r2,  #1
        pop             {r4-r10,pc}
endfunc

function msac_decode_bool_equi_neon, export=1
        push            {r4-r10,lr}
        ldr             r5,  [r0, #RNG]
        ldr             r6,  [r0, #CNT]
        sub             sp,  sp,  #48
        ldr             r7,  [r0, #DIF]
        bic             r4,  r5,  #0xff        // r &= 0xff00
        add             r4,  r4,  #8
        mov             r2,  #0
        subs            r8,  r7,  r4, lsl #15  // dif - vw
        lsr             r4,  r4,  #1           // v
        sub             r5,  r5,  r4           // r - v
        itee            lo
        movlo           r2,  #1
        movhs           r4,  r5                // if (ret) v = r - v;
        movhs           r7,  r8                // if (ret) dif = dif - vw;

        clz             r5,  r4                // clz(rng)
        mvn             r7,  r7                // ~dif
        eor             r5,  r5,  #16          // d = clz(rng) ^ 16
        mov             lr,  r2
        b               L(renorm2)
endfunc

function msac_decode_bool_neon, export=1
        push            {r4-r10,lr}
        ldr             r5,  [r0, #RNG]
        ldr             r6,  [r0, #CNT]
        sub             sp,  sp,  #48
        ldr             r7,  [r0, #DIF]
        lsr             r4,  r5,  #8           // r >> 8
        bic             r1,  r1,  #0x3f        // f &= ~63
        mul             r4,  r4,  r1
        mov             r2,  #0
        lsr             r4,  r4,  #7
        add             r4,  r4,  #4           // v
        subs            r8,  r7,  r4, lsl #16  // dif - vw
        sub             r5,  r5,  r4           // r - v
        itee            lo
        movlo           r2,  #1
        movhs           r4,  r5                // if (ret) v = r - v;
        movhs           r7,  r8                // if (ret) dif = dif - vw;

        clz             r5,  r4                // clz(rng)
        mvn             r7,  r7                // ~dif
        eor             r5,  r5,  #16          // d = clz(rng) ^ 16
        mov             lr,  r2
        b               L(renorm2)
endfunc

function msac_decode_bool_adapt_neon, export=1
        push            {r4-r10,lr}
        ldr             r9,  [r1]              // cdf[0-1]
        ldr             r5,  [r0, #RNG]
        movw            lr,  #0xffc0
        ldr             r6,  [r0, #CNT]
        sub             sp,  sp,  #48
        ldr             r7,  [r0, #DIF]
        lsr             r4,  r5,  #8           // r >> 8
        and             r2,  r9,  lr           // f &= ~63
        mul             r4,  r4,  r2
        mov             r2,  #0
        lsr             r4,  r4,  #7
        add             r4,  r4,  #4           // v
        subs            r8,  r7,  r4, lsl #16  // dif - vw
        sub             r5,  r5,  r4           // r - v
        ldr             r10, [r0, #ALLOW_UPDATE_CDF]
        itee            lo
        movlo           r2,  #1
        movhs           r4,  r5                // if (ret) v = r - v;
        movhs           r7,  r8                // if (ret) dif = dif - vw;

        cmp             r10, #0
        clz             r5,  r4                // clz(rng)
        mvn             r7,  r7                // ~dif
        eor             r5,  r5,  #16          // d = clz(rng) ^ 16
        mov             lr,  r2

        beq             L(renorm2)

        lsr             r2,  r9,  #16          // count = cdf[1]
        uxth            r9,  r9                // cdf[0]

        sub             r3,  r2,  r2,  lsr #5  // count - (count >= 32)
        lsr             r2,  r2,  #4           // count >> 4
        add             r10, r3,  #1           // count + (count < 32)
        add             r2,  r2,  #4           // rate = (count >> 4) | 4

        sub             r9,  r9,  lr           // cdf[0] -= bit
        sub             r3,  r9,  lr,  lsl #15 // {cdf[0], cdf[0] - 32769}
        asr             r3,  r3,  r2           // {cdf[0], cdf[0] - 32769} >> rate
        sub             r9,  r9,  r3           // cdf[0]

        strh            r9,  [r1]
        strh            r10, [r1, #2]

        b               L(renorm2)
endfunc
