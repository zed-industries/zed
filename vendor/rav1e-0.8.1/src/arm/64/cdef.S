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

.macro pad_top_bottom s1, s2, w, stride, rn, rw, ret
        tst             w7,  #1 // CDEF_HAVE_LEFT
        b.eq            2f
        // CDEF_HAVE_LEFT
        sub             \s1,  \s1,  #2
        sub             \s2,  \s2,  #2
        tst             w7,  #2 // CDEF_HAVE_RIGHT
        b.eq            1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        ldr             \rn\()0, [\s1]
        ldr             s1,      [\s1, #\w]
        ldr             \rn\()2, [\s2]
        ldr             s3,      [\s2, #\w]
        uxtl            v0.8h,   v0.8b
        uxtl            v1.8h,   v1.8b
        uxtl            v2.8h,   v2.8b
        uxtl            v3.8h,   v3.8b
        str             \rw\()0, [x0]
        str             d1,      [x0, #2*\w]
        add             x0,  x0,  #2*\stride
        str             \rw\()2, [x0]
        str             d3,      [x0, #2*\w]
.if \ret
        ret
.else
        add             x0,  x0,  #2*\stride
        b               3f
.endif

1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        ldr             \rn\()0, [\s1]
        ldr             h1,      [\s1, #\w]
        ldr             \rn\()2, [\s2]
        ldr             h3,      [\s2, #\w]
        uxtl            v0.8h,   v0.8b
        uxtl            v1.8h,   v1.8b
        uxtl            v2.8h,   v2.8b
        uxtl            v3.8h,   v3.8b
        str             \rw\()0, [x0]
        str             s1,      [x0, #2*\w]
        str             s31,     [x0, #2*\w+4]
        add             x0,  x0,  #2*\stride
        str             \rw\()2, [x0]
        str             s3,      [x0, #2*\w]
        str             s31,     [x0, #2*\w+4]
.if \ret
        ret
.else
        add             x0,  x0,  #2*\stride
        b               3f
.endif

2:
        // !CDEF_HAVE_LEFT
        tst             w7,  #2 // CDEF_HAVE_RIGHT
        b.eq            1f
        // !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        ldr             \rn\()0, [\s1]
        ldr             h1,      [\s1, #\w]
        ldr             \rn\()2, [\s2]
        ldr             h3,      [\s2, #\w]
        uxtl            v0.8h,  v0.8b
        uxtl            v1.8h,  v1.8b
        uxtl            v2.8h,  v2.8b
        uxtl            v3.8h,  v3.8b
        str             s31, [x0]
        stur            \rw\()0, [x0, #4]
        str             s1,      [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        str             s31, [x0]
        stur            \rw\()2, [x0, #4]
        str             s3,      [x0, #4+2*\w]
.if \ret
        ret
.else
        add             x0,  x0,  #2*\stride
        b               3f
.endif

1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        ldr             \rn\()0, [\s1]
        ldr             \rn\()1, [\s2]
        uxtl            v0.8h,  v0.8b
        uxtl            v1.8h,  v1.8b
        str             s31,     [x0]
        stur            \rw\()0, [x0, #4]
        str             s31,     [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        str             s31,     [x0]
        stur            \rw\()1, [x0, #4]
        str             s31,     [x0, #4+2*\w]
.if \ret
        ret
.else
        add             x0,  x0,  #2*\stride
.endif
3:
.endm

.macro load_n_incr dst, src, incr, w
.if \w == 4
        ld1             {\dst\().s}[0], [\src], \incr
.else
        ld1             {\dst\().8b},   [\src], \incr
.endif
.endm

// void dav1d_cdef_paddingX_8bpc_neon(uint16_t *tmp, const pixel *src,
//                                    ptrdiff_t src_stride, const pixel (*left)[2],
//                                    const pixel *const top,
//                                    const pixel *const bottom, int h,
//                                    enum CdefEdgeFlags edges);

.macro padding_func w, stride, rn, rw
function cdef_padding\w\()_8bpc_neon, export=1
        cmp             w7,  #0xf // fully edged
        b.eq            cdef_padding\w\()_edged_8bpc_neon
        movi            v30.8h,  #0x80, lsl #8
        mov             v31.16b, v30.16b
        sub             x0,  x0,  #2*(2*\stride+2)
        tst             w7,  #4 // CDEF_HAVE_TOP
        b.ne            1f
        // !CDEF_HAVE_TOP
        st1             {v30.8h, v31.8h}, [x0], #32
.if \w == 8
        st1             {v30.8h, v31.8h}, [x0], #32
.endif
        b               3f
1:
        // CDEF_HAVE_TOP
        add             x9,  x4,  x2
        pad_top_bottom  x4,  x9, \w, \stride, \rn, \rw, 0

        // Middle section
3:
        tst             w7,  #1 // CDEF_HAVE_LEFT
        b.eq            2f
        // CDEF_HAVE_LEFT
        tst             w7,  #2 // CDEF_HAVE_RIGHT
        b.eq            1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        ld1             {v0.h}[0], [x3], #2
        ldr             h2,      [x1, #\w]
        load_n_incr     v1,  x1,  x2,  \w
        subs            w6,  w6,  #1
        uxtl            v0.8h,  v0.8b
        uxtl            v1.8h,  v1.8b
        uxtl            v2.8h,  v2.8b
        str             s0,      [x0]
        stur            \rw\()1, [x0, #4]
        str             s2,      [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        b.gt            0b
        b               3f
1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        ld1             {v0.h}[0], [x3], #2
        load_n_incr     v1,  x1,  x2,  \w
        subs            w6,  w6,  #1
        uxtl            v0.8h,  v0.8b
        uxtl            v1.8h,  v1.8b
        str             s0,      [x0]
        stur            \rw\()1, [x0, #4]
        str             s31,     [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        b.gt            1b
        b               3f
2:
        tst             w7,  #2 // CDEF_HAVE_RIGHT
        b.eq            1f
        // !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        ldr             h1,      [x1, #\w]
        load_n_incr     v0,  x1,  x2,  \w
        subs            w6,  w6,  #1
        uxtl            v0.8h,  v0.8b
        uxtl            v1.8h,  v1.8b
        str             s31,     [x0]
        stur            \rw\()0, [x0, #4]
        str             s1,      [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        b.gt            0b
        b               3f
1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        load_n_incr     v0,  x1,  x2,  \w
        subs            w6,  w6,  #1
        uxtl            v0.8h,  v0.8b
        str             s31,     [x0]
        stur            \rw\()0, [x0, #4]
        str             s31,     [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        b.gt            1b

3:
        tst             w7,  #8 // CDEF_HAVE_BOTTOM
        b.ne            1f
        // !CDEF_HAVE_BOTTOM
        st1             {v30.8h, v31.8h}, [x0], #32
.if \w == 8
        st1             {v30.8h, v31.8h}, [x0], #32
.endif
        ret
1:
        // CDEF_HAVE_BOTTOM
        add             x9,  x5,  x2
        pad_top_bottom  x5,  x9, \w, \stride, \rn, \rw, 1
endfunc
.endm

padding_func 8, 16, d, q
padding_func 4, 8,  s, d

// void cdef_paddingX_edged_8bpc_neon(uint8_t *tmp, const pixel *src,
//                                    ptrdiff_t src_stride, const pixel (*left)[2],
//                                    const pixel *const top,
//                                    const pixel *const bottom, int h,
//                                    enum CdefEdgeFlags edges);

.macro padding_func_edged w, stride, reg
function cdef_padding\w\()_edged_8bpc_neon, export=1
        sub             x4,  x4,  #2
        sub             x5,  x5,  #2
        sub             x0,  x0,  #(2*\stride+2)

.if \w == 4
        ldr             d0, [x4]
        ldr             d1, [x4, x2]
        st1             {v0.8b, v1.8b}, [x0], #16
.else
        add             x9,  x4,  x2
        ldr             d0, [x4]
        ldr             s1, [x4, #8]
        ldr             d2, [x9]
        ldr             s3, [x9, #8]
        str             d0, [x0]
        str             s1, [x0, #8]
        str             d2, [x0, #\stride]
        str             s3, [x0, #\stride+8]
        add             x0,  x0,  #2*\stride
.endif

0:
        ld1             {v0.h}[0], [x3], #2
        ldr             h2,      [x1, #\w]
        load_n_incr     v1,  x1,  x2,  \w
        subs            w6,  w6,  #1
        str             h0,      [x0]
        stur            \reg\()1, [x0, #2]
        str             h2,      [x0, #2+\w]
        add             x0,  x0,  #\stride
        b.gt            0b

.if \w == 4
        ldr             d0, [x5]
        ldr             d1, [x5, x2]
        st1             {v0.8b, v1.8b}, [x0], #16
.else
        add             x9,  x5,  x2
        ldr             d0, [x5]
        ldr             s1, [x5, #8]
        ldr             d2, [x9]
        ldr             s3, [x9, #8]
        str             d0, [x0]
        str             s1, [x0, #8]
        str             d2, [x0, #\stride]
        str             s3, [x0, #\stride+8]
.endif
        ret
endfunc
.endm

padding_func_edged 8, 16, d
padding_func_edged 4, 8,  s

tables

filter 8, 8
filter 4, 8

find_dir 8

.macro load_px_8 d1, d2, w
.if \w == 8
        add             x6,  x2,  w9, sxtb          // x + off
        sub             x9,  x2,  w9, sxtb          // x - off
        ld1             {\d1\().d}[0], [x6]         // p0
        add             x6,  x6,  #16               // += stride
        ld1             {\d2\().d}[0], [x9]         // p1
        add             x9,  x9,  #16               // += stride
        ld1             {\d1\().d}[1], [x6]         // p0
        ld1             {\d2\().d}[1], [x9]         // p0
.else
        add             x6,  x2,  w9, sxtb          // x + off
        sub             x9,  x2,  w9, sxtb          // x - off
        ld1             {\d1\().s}[0], [x6]         // p0
        add             x6,  x6,  #8                // += stride
        ld1             {\d2\().s}[0], [x9]         // p1
        add             x9,  x9,  #8                // += stride
        ld1             {\d1\().s}[1], [x6]         // p0
        add             x6,  x6,  #8                // += stride
        ld1             {\d2\().s}[1], [x9]         // p1
        add             x9,  x9,  #8                // += stride
        ld1             {\d1\().s}[2], [x6]         // p0
        add             x6,  x6,  #8                // += stride
        ld1             {\d2\().s}[2], [x9]         // p1
        add             x9,  x9,  #8                // += stride
        ld1             {\d1\().s}[3], [x6]         // p0
        ld1             {\d2\().s}[3], [x9]         // p1
.endif
.endm
.macro handle_pixel_8 s1, s2, thresh_vec, shift, tap, min
.if \min
        umin            v3.16b,  v3.16b,  \s1\().16b
        umax            v4.16b,  v4.16b,  \s1\().16b
        umin            v3.16b,  v3.16b,  \s2\().16b
        umax            v4.16b,  v4.16b,  \s2\().16b
.endif
        uabd            v16.16b, v0.16b,  \s1\().16b  // abs(diff)
        uabd            v20.16b, v0.16b,  \s2\().16b  // abs(diff)
        ushl            v17.16b, v16.16b, \shift      // abs(diff) >> shift
        ushl            v21.16b, v20.16b, \shift      // abs(diff) >> shift
        uqsub           v17.16b, \thresh_vec, v17.16b // clip = imax(0, threshold - (abs(diff) >> shift))
        uqsub           v21.16b, \thresh_vec, v21.16b // clip = imax(0, threshold - (abs(diff) >> shift))
        cmhi            v18.16b, v0.16b,  \s1\().16b  // px > p0
        cmhi            v22.16b, v0.16b,  \s2\().16b  // px > p1
        umin            v17.16b, v17.16b, v16.16b     // imin(abs(diff), clip)
        umin            v21.16b, v21.16b, v20.16b     // imin(abs(diff), clip)
        dup             v19.16b, \tap                 // taps[k]
        neg             v16.16b, v17.16b              // -imin()
        neg             v20.16b, v21.16b              // -imin()
        bsl             v18.16b, v16.16b, v17.16b     // constrain() = apply_sign()
        bsl             v22.16b, v20.16b, v21.16b     // constrain() = apply_sign()
        mla             v1.16b,  v18.16b, v19.16b     // sum += taps[k] * constrain()
        mla             v2.16b,  v22.16b, v19.16b     // sum += taps[k] * constrain()
.endm

// void cdef_filterX_edged_8bpc_neon(pixel *dst, ptrdiff_t dst_stride,
//                                   const uint8_t *tmp, int pri_strength,
//                                   int sec_strength, int dir, int damping,
//                                   int h);
.macro filter_func_8 w, pri, sec, min, suffix
function cdef_filter\w\suffix\()_edged_8bpc_neon
.if \pri
        movrel          x8,  pri_taps
        and             w9,  w3,  #1
        add             x8,  x8,  w9, uxtw #1
.endif
        movrel          x9,  directions\w
        add             x5,  x9,  w5, uxtw #1
        movi            v30.8b,  #7
        dup             v28.8b,  w6                 // damping

.if \pri
        dup             v25.16b, w3                 // threshold
.endif
.if \sec
        dup             v27.16b, w4                 // threshold
.endif
        trn1            v24.8b,  v25.8b, v27.8b
        clz             v24.8b,  v24.8b             // clz(threshold)
        sub             v24.8b,  v30.8b, v24.8b     // ulog2(threshold)
        uqsub           v24.8b,  v28.8b, v24.8b     // shift = imax(0, damping - ulog2(threshold))
        neg             v24.8b,  v24.8b             // -shift
.if \sec
        dup             v26.16b, v24.b[1]
.endif
.if \pri
        dup             v24.16b, v24.b[0]
.endif

1:
.if \w == 8
        add             x12, x2,  #16
        ld1             {v0.d}[0], [x2]             // px
        ld1             {v0.d}[1], [x12]            // px
.else
        add             x12, x2,  #1*8
        add             x13, x2,  #2*8
        add             x14, x2,  #3*8
        ld1             {v0.s}[0], [x2]             // px
        ld1             {v0.s}[1], [x12]            // px
        ld1             {v0.s}[2], [x13]            // px
        ld1             {v0.s}[3], [x14]            // px
.endif

        // We need 9-bits or two 8-bit accululators to fit the sum.
        // Max of |sum| > 15*2*6(pri) + 4*4*3(sec) = 228.
        // Start sum at -1 instead of 0 to help handle rounding later.
        movi            v1.16b, #255                // sum
        movi            v2.16b, #0                  // sum
.if \min
        mov             v3.16b, v0.16b              // min
        mov             v4.16b, v0.16b              // max
.endif

        // Instead of loading sec_taps 2, 1 from memory, just set it
        // to 2 initially and decrease for the second round.
        // This is also used as loop counter.
        mov             w11, #2                     // sec_taps[0]

2:
.if \pri
        ldrb            w9,  [x5]                   // off1

        load_px_8       v5,  v6, \w
.endif

.if \sec
        add             x5,  x5,  #4                // +2*2
        ldrb            w9,  [x5]                   // off2
        load_px_8       v28, v29, \w
.endif

.if \pri
        ldrb            w10, [x8]                   // *pri_taps

        handle_pixel_8  v5,  v6,  v25.16b, v24.16b, w10, \min
.endif

.if \sec
        add             x5,  x5,  #8                // +2*4
        ldrb            w9,  [x5]                   // off3
        load_px_8       v5,  v6,  \w

        handle_pixel_8  v28, v29, v27.16b, v26.16b, w11, \min

        handle_pixel_8  v5,  v6,  v27.16b, v26.16b, w11, \min

        sub             x5,  x5,  #11               // x5 -= 2*(2+4); x5 += 1;
.else
        add             x5,  x5,  #1                // x5 += 1
.endif
        subs            w11, w11, #1                // sec_tap-- (value)
.if \pri
        add             x8,  x8,  #1                // pri_taps++ (pointer)
.endif
        b.ne            2b

        // Perform halving adds since the value won't fit otherwise.
        // To handle the offset for negative values, use both halving w/ and w/o rounding.
        srhadd          v5.16b,  v1.16b,  v2.16b    // sum >> 1
        shadd           v6.16b,  v1.16b,  v2.16b    // (sum - 1) >> 1
        cmlt            v1.16b,  v5.16b,  #0        // sum < 0
        bsl             v1.16b,  v6.16b,  v5.16b    // (sum - (sum < 0)) >> 1

        srshr           v1.16b,  v1.16b,  #3        // (8 + sum - (sum < 0)) >> 4

        usqadd          v0.16b,  v1.16b             // px + (8 + sum ...) >> 4
.if \min
        umin            v0.16b,  v0.16b,  v4.16b
        umax            v0.16b,  v0.16b,  v3.16b    // iclip(px + .., min, max)
.endif
.if \w == 8
        st1             {v0.d}[0], [x0], x1
        add             x2,  x2,  #2*16             // tmp += 2*tmp_stride
        subs            w7,  w7,  #2                // h -= 2
        st1             {v0.d}[1], [x0], x1
.else
        st1             {v0.s}[0], [x0], x1
        add             x2,  x2,  #4*8              // tmp += 4*tmp_stride
        st1             {v0.s}[1], [x0], x1
        subs            w7,  w7,  #4                // h -= 4
        st1             {v0.s}[2], [x0], x1
        st1             {v0.s}[3], [x0], x1
.endif

        // Reset pri_taps and directions back to the original point
        sub             x5,  x5,  #2
.if \pri
        sub             x8,  x8,  #2
.endif

        b.gt            1b
        ret
endfunc
.endm

.macro filter_8 w
filter_func_8 \w, pri=1, sec=0, min=0, suffix=_pri
filter_func_8 \w, pri=0, sec=1, min=0, suffix=_sec
filter_func_8 \w, pri=1, sec=1, min=1, suffix=_pri_sec
.endm

filter_8 8
filter_8 4
