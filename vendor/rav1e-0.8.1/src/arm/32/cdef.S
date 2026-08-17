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
#include "cdef_tmpl.S"

// n1 = s0/d0
// w1 = d0/q0
// n2 = s4/d2
// w2 = d2/q1
.macro pad_top_bottom s1, s2, w, stride, n1, w1, n2, w2, align, ret
        tst             r7,  #1 // CDEF_HAVE_LEFT
        beq             2f
        // CDEF_HAVE_LEFT
        tst             r7,  #2 // CDEF_HAVE_RIGHT
        beq             1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        ldrh            r12, [\s1, #-2]
        vldr            \n1, [\s1]
        vdup.16         d4,  r12
        ldrh            r12, [\s1, #\w]
        vmov.16         d4[1], r12
        ldrh            r12, [\s2, #-2]
        vldr            \n2, [\s2]
        vmov.16         d4[2], r12
        ldrh            r12, [\s2, #\w]
        vmovl.u8        q0,  d0
        vmov.16         d4[3], r12
        vmovl.u8        q1,  d2
        vmovl.u8        q2,  d4
        vstr            s8,  [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s9,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s10, [r0, #-4]
        vst1.16         {\w2}, [r0, :\align]
        vstr            s11, [r0, #2*\w]
.if \ret
        pop             {r4-r8,pc}
.else
        add             r0,  r0,  #2*\stride
        b               3f
.endif

1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        ldrh            r12, [\s1, #-2]
        vldr            \n1, [\s1]
        vdup.16         d4,  r12
        ldrh            r12, [\s2, #-2]
        vldr            \n2, [\s2]
        vmovl.u8        q0,  d0
        vmov.16         d4[1], r12
        vmovl.u8        q1,  d2
        vmovl.u8        q2,  d4
        vstr            s8,  [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s9,  [r0, #-4]
        vst1.16         {\w2}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
.if \ret
        pop             {r4-r8,pc}
.else
        add             r0,  r0,  #2*\stride
        b               3f
.endif

2:
        // !CDEF_HAVE_LEFT
        tst             r7,  #2 // CDEF_HAVE_RIGHT
        beq             1f
        // !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        vldr            \n1, [\s1]
        ldrh            r12, [\s1, #\w]
        vldr            \n2, [\s2]
        vdup.16         d4,  r12
        ldrh            r12, [\s2, #\w]
        vmovl.u8        q0,  d0
        vmov.16         d4[1], r12
        vmovl.u8        q1,  d2
        vmovl.u8        q2,  d4
        vstr            s12, [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s8,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s12, [r0, #-4]
        vst1.16         {\w2}, [r0, :\align]
        vstr            s9,  [r0, #2*\w]
.if \ret
        pop             {r4-r8,pc}
.else
        add             r0,  r0,  #2*\stride
        b               3f
.endif

1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        vldr            \n1, [\s1]
        vldr            \n2, [\s2]
        vmovl.u8        q0,  d0
        vmovl.u8        q1,  d2
        vstr            s12, [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s12, [r0, #-4]
        vst1.16         {\w2}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
.if \ret
        pop             {r4-r8,pc}
.else
        add             r0,  r0,  #2*\stride
.endif
3:
.endm

.macro load_n_incr dst, src, incr, w
.if \w == 4
        vld1.32         {\dst\()[0]}, [\src, :32], \incr
.else
        vld1.8          {\dst\()},    [\src, :64], \incr
.endif
.endm

// void dav1d_cdef_paddingX_8bpc_neon(uint16_t *tmp, const pixel *src,
//                                    ptrdiff_t src_stride, const pixel (*left)[2],
//                                    const pixel *const top,
//                                    const pixel *const bottom, int h,
//                                    enum CdefEdgeFlags edges);

// n1 = s0/d0
// w1 = d0/q0
// n2 = s4/d2
// w2 = d2/q1
.macro padding_func w, stride, n1, w1, n2, w2, align
function cdef_padding\w\()_8bpc_neon, export=1
        push            {r4-r8,lr}
        ldrd            r4,  r5,  [sp, #24]
        ldrd            r6,  r7,  [sp, #32]
        cmp             r7,  #0xf // fully edged
        beq             cdef_padding\w\()_edged_8bpc_neon
        vmov.i16        q3,  #0x8000
        tst             r7,  #4 // CDEF_HAVE_TOP
        bne             1f
        // !CDEF_HAVE_TOP
        sub             r12, r0,  #2*(2*\stride+2)
        vmov.i16        q2,  #0x8000
        vst1.16         {q2,q3}, [r12]!
.if \w == 8
        vst1.16         {q2,q3}, [r12]!
.endif
        b               3f
1:
        // CDEF_HAVE_TOP
        add             r8,  r4,  r2
        sub             r0,  r0,  #2*(2*\stride)
        pad_top_bottom  r4,  r8,  \w, \stride, \n1, \w1, \n2, \w2, \align, 0

        // Middle section
3:
        tst             r7,  #1 // CDEF_HAVE_LEFT
        beq             2f
        // CDEF_HAVE_LEFT
        tst             r7,  #2 // CDEF_HAVE_RIGHT
        beq             1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        vld1.16         {d2[]}, [r3, :16]!
        ldrh            r12, [r1, #\w]
        load_n_incr     d0,  r1,  r2,  \w
        subs            r6,  r6,  #1
        vmov.16         d2[1], r12
        vmovl.u8        q0,  d0
        vmovl.u8        q1,  d2
        vstr            s4,  [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s5,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        bgt             0b
        b               3f
1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        vld1.16         {d2[]}, [r3, :16]!
        load_n_incr     d0,  r1,  r2,  \w
        subs            r6,  r6,  #1
        vmovl.u8        q0,  d0
        vmovl.u8        q1,  d2
        vstr            s4,  [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        bgt             1b
        b               3f
2:
        tst             r7,  #2 // CDEF_HAVE_RIGHT
        beq             1f
        // !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        ldrh            r12, [r1, #\w]
        load_n_incr     d0,  r1,  r2,  \w
        vdup.16         d2,  r12
        subs            r6,  r6,  #1
        vmovl.u8        q0,  d0
        vmovl.u8        q1,  d2
        vstr            s12, [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s4,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        bgt             0b
        b               3f
1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        load_n_incr     d0,  r1,  r2,  \w
        subs            r6,  r6,  #1
        vmovl.u8        q0,  d0
        vstr            s12, [r0, #-4]
        vst1.16         {\w1}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        bgt             1b

3:
        tst             r7,  #8 // CDEF_HAVE_BOTTOM
        bne             1f
        // !CDEF_HAVE_BOTTOM
        sub             r12, r0,  #4
        vmov.i16        q2,  #0x8000
        vst1.16         {q2,q3}, [r12]!
.if \w == 8
        vst1.16         {q2,q3}, [r12]!
.endif
        pop             {r4-r8,pc}
1:
        // CDEF_HAVE_BOTTOM
        add             r8,  r5,  r2
        pad_top_bottom  r5,  r8,  \w, \stride, \n1, \w1, \n2, \w2, \align, 1
endfunc
.endm

padding_func 8, 16, d0, q0, d2, q1, 128
padding_func 4, 8,  s0, d0, s4, d2, 64

// void cdef_paddingX_edged_8bpc_neon(uint16_t *tmp, const pixel *src,
//                                    ptrdiff_t src_stride, const pixel (*left)[2],
//                                    const pixel *const top,
//                                    const pixel *const bottom, int h,
//                                    enum CdefEdgeFlags edges);

.macro padding_func_edged w, stride, reg, align
function cdef_padding\w\()_edged_8bpc_neon
        sub             r0,  r0,  #(2*\stride)

        ldrh            r12, [r4, #-2]
        vldr            \reg, [r4]
        add             r8,  r4,  r2
        strh            r12, [r0, #-2]
        ldrh            r12, [r4, #\w]
        vstr            \reg, [r0]
        strh            r12, [r0, #\w]

        ldrh            r12, [r8, #-2]
        vldr            \reg, [r8]
        strh            r12, [r0, #\stride-2]
        ldrh            r12, [r8, #\w]
        vstr            \reg, [r0, #\stride]
        strh            r12, [r0, #\stride+\w]
        add             r0,  r0,  #2*\stride

0:
        ldrh            r12, [r3], #2
        vldr            \reg, [r1]
        str             r12, [r0, #-2]
        ldrh            r12, [r1, #\w]
        add             r1,  r1,  r2
        subs            r6,  r6,  #1
        vstr            \reg, [r0]
        str             r12, [r0, #\w]
        add             r0,  r0,  #\stride
        bgt             0b

        ldrh            r12, [r5, #-2]
        vldr            \reg, [r5]
        add             r8,  r5,  r2
        strh            r12, [r0, #-2]
        ldrh            r12, [r5, #\w]
        vstr            \reg, [r0]
        strh            r12, [r0, #\w]

        ldrh            r12, [r8, #-2]
        vldr            \reg, [r8]
        strh            r12, [r0, #\stride-2]
        ldrh            r12, [r8, #\w]
        vstr            \reg, [r0, #\stride]
        strh            r12, [r0, #\stride+\w]

        pop             {r4-r8,pc}
endfunc
.endm

padding_func_edged 8, 16, d0, 64
padding_func_edged 4, 8,  s0, 32

tables

filter 8, 8
filter 4, 8

find_dir 8

.macro load_px_8 d11, d12, d21, d22, w
.if \w == 8
        add             r6,  r2,  r9         // x + off
        sub             r9,  r2,  r9         // x - off
        vld1.8          {\d11}, [r6]         // p0
        add             r6,  r6,  #16        // += stride
        vld1.8          {\d21}, [r9]         // p1
        add             r9,  r9,  #16        // += stride
        vld1.8          {\d12}, [r6]         // p0
        vld1.8          {\d22}, [r9]         // p1
.else
        add             r6,  r2,  r9         // x + off
        sub             r9,  r2,  r9         // x - off
        vld1.32         {\d11[0]}, [r6]      // p0
        add             r6,  r6,  #8         // += stride
        vld1.32         {\d21[0]}, [r9]      // p1
        add             r9,  r9,  #8         // += stride
        vld1.32         {\d11[1]}, [r6]      // p0
        add             r6,  r6,  #8         // += stride
        vld1.32         {\d21[1]}, [r9]      // p1
        add             r9,  r9,  #8         // += stride
        vld1.32         {\d12[0]}, [r6]      // p0
        add             r6,  r6,  #8         // += stride
        vld1.32         {\d22[0]}, [r9]      // p1
        add             r9,  r9,  #8         // += stride
        vld1.32         {\d12[1]}, [r6]      // p0
        vld1.32         {\d22[1]}, [r9]      // p1
.endif
.endm
.macro handle_pixel_8 s1, s2, thresh_vec, shift, tap, min
.if \min
        vmin.u8         q3,  q3,  \s1
        vmax.u8         q4,  q4,  \s1
        vmin.u8         q3,  q3,  \s2
        vmax.u8         q4,  q4,  \s2
.endif
        vabd.u8         q8,  q0,  \s1        // abs(diff)
        vabd.u8         q11, q0,  \s2        // abs(diff)
        vshl.u8         q9,  q8,  \shift     // abs(diff) >> shift
        vshl.u8         q12, q11, \shift     // abs(diff) >> shift
        vqsub.u8        q9,  \thresh_vec, q9 // clip = imax(0, threshold - (abs(diff) >> shift))
        vqsub.u8        q12, \thresh_vec, q12// clip = imax(0, threshold - (abs(diff) >> shift))
        vcgt.u8         q10, q0,  \s1        // px > p0
        vcgt.u8         q13, q0,  \s2        // px > p1
        vmin.u8         q9,  q9,  q8         // imin(abs(diff), clip)
        vmin.u8         q12, q12, q11        // imin(abs(diff), clip)
        vneg.s8         q8,  q9              // -imin()
        vneg.s8         q11, q12             // -imin()
        vbsl            q10, q8,  q9         // constrain() = imax(imin(diff, clip), -clip)
        vdup.8          d18, \tap            // taps[k]
        vbsl            q13, q11, q12        // constrain() = imax(imin(diff, clip), -clip)
        vmlal.s8        q1,  d20, d18        // sum += taps[k] * constrain()
        vmlal.s8        q1,  d26, d18        // sum += taps[k] * constrain()
        vmlal.s8        q2,  d21, d18        // sum += taps[k] * constrain()
        vmlal.s8        q2,  d27, d18        // sum += taps[k] * constrain()
.endm

// void cdef_filterX_edged_neon(pixel *dst, ptrdiff_t dst_stride,
//                              const uint16_t *tmp, int pri_strength,
//                              int sec_strength, int dir, int damping,
//                              int h, size_t edges);
.macro filter_func_8 w, pri, sec, min, suffix
function cdef_filter\w\suffix\()_edged_neon
.if \pri
        movrel_local    r8,  pri_taps
        and             r9,  r3,  #1
        add             r8,  r8,  r9, lsl #1
.endif
        movrel_local    r9,  directions\w
        add             r5,  r9,  r5, lsl #1
        vmov.u8         d17, #7
        vdup.8          d16, r6              // damping

        vmov.8          d8[0], r3
        vmov.8          d8[1], r4
        vclz.i8         d8,  d8              // clz(threshold)
        vsub.i8         d8,  d17, d8         // ulog2(threshold)
        vqsub.u8        d8,  d16, d8         // shift = imax(0, damping - ulog2(threshold))
        vneg.s8         d8,  d8              // -shift
.if \sec
        vdup.8          q6,  d8[1]
.endif
.if \pri
        vdup.8          q5,  d8[0]
.endif

1:
.if \w == 8
        add             r12, r2,  #16
        vld1.8          {d0},  [r2,  :64]    // px
        vld1.8          {d1},  [r12, :64]    // px
.else
        add             r12, r2,  #8
        vld1.32         {d0[0]},  [r2,  :32] // px
        add             r9,  r2,  #2*8
        vld1.32         {d0[1]},  [r12, :32] // px
        add             r12, r12, #2*8
        vld1.32         {d1[0]},  [r9,  :32] // px
        vld1.32         {d1[1]},  [r12, :32] // px
.endif

        vmov.u8         q1,  #0              // sum
        vmov.u8         q2,  #0              // sum
.if \min
        vmov.u16        q3,  q0              // min
        vmov.u16        q4,  q0              // max
.endif

        // Instead of loading sec_taps 2, 1 from memory, just set it
        // to 2 initially and decrease for the second round.
        // This is also used as loop counter.
        mov             lr,  #2              // sec_taps[0]

2:
.if \pri
        ldrsb           r9,  [r5]            // off1

        load_px_8       d28, d29, d30, d31, \w
.endif

.if \sec
        add             r5,  r5,  #4         // +2*2
        ldrsb           r9,  [r5]            // off2
.endif

.if \pri
        ldrb            r12, [r8]            // *pri_taps
        vdup.8          q7,  r3              // threshold

        handle_pixel_8  q14, q15, q7,  q5,  r12, \min
.endif

.if \sec
        load_px_8       d28, d29, d30, d31, \w

        add             r5,  r5,  #8         // +2*4
        ldrsb           r9,  [r5]            // off3

        vdup.8          q7,  r4              // threshold

        handle_pixel_8  q14, q15, q7,  q6,  lr, \min

        load_px_8       d28, d29, d30, d31, \w

        handle_pixel_8  q14, q15, q7,  q6,  lr, \min

        sub             r5,  r5,  #11        // r5 -= 2*(2+4); r5 += 1;
.else
        add             r5,  r5,  #1         // r5 += 1
.endif
        subs            lr,  lr,  #1         // sec_tap-- (value)
.if \pri
        add             r8,  r8,  #1         // pri_taps++ (pointer)
.endif
        bne             2b

        vshr.s16        q14, q1,  #15        // -(sum < 0)
        vshr.s16        q15, q2,  #15        // -(sum < 0)
        vadd.i16        q1,  q1,  q14        // sum - (sum < 0)
        vadd.i16        q2,  q2,  q15        // sum - (sum < 0)
        vrshr.s16       q1,  q1,  #4         // (8 + sum - (sum < 0)) >> 4
        vrshr.s16       q2,  q2,  #4         // (8 + sum - (sum < 0)) >> 4
        vaddw.u8        q1,  q1,  d0         // px + (8 + sum ...) >> 4
        vaddw.u8        q2,  q2,  d1         // px + (8 + sum ...) >> 4
        vqmovun.s16     d0,  q1
        vqmovun.s16     d1,  q2
.if \min
        vmin.u8         q0,  q0,  q4
        vmax.u8         q0,  q0,  q3         // iclip(px + .., min, max)
.endif
.if \w == 8
        vst1.8          {d0}, [r0, :64], r1
        add             r2,  r2,  #2*16      // tmp += 2*tmp_stride
        subs            r7,  r7,  #2         // h -= 2
        vst1.8          {d1}, [r0, :64], r1
.else
        vst1.32         {d0[0]}, [r0, :32], r1
        add             r2,  r2,  #4*8       // tmp += 4*tmp_stride
        vst1.32         {d0[1]}, [r0, :32], r1
        subs            r7,  r7,  #4         // h -= 4
        vst1.32         {d1[0]}, [r0, :32], r1
        vst1.32         {d1[1]}, [r0, :32], r1
.endif

        // Reset pri_taps and directions back to the original point
        sub             r5,  r5,  #2
.if \pri
        sub             r8,  r8,  #2
.endif

        bgt             1b
        vpop            {q4-q7}
        pop             {r4-r9,pc}
endfunc
.endm

.macro filter_8 w
filter_func_8 \w, pri=1, sec=0, min=0, suffix=_pri
filter_func_8 \w, pri=0, sec=1, min=0, suffix=_sec
filter_func_8 \w, pri=1, sec=1, min=1, suffix=_pri_sec
.endm

filter_8 8
filter_8 4
