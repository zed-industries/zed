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
#include "cdef_tmpl.S"

.macro pad_top_bot_16 s1, s2, w, stride, reg, ret
        tst             w7,  #1 // CDEF_HAVE_LEFT
        b.eq            2f
        // CDEF_HAVE_LEFT
        sub             \s1,  \s1,  #4
        sub             \s2,  \s2,  #4
        tst             w7,  #2 // CDEF_HAVE_RIGHT
        b.eq            1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        ldr             \reg\()0, [\s1]
        ldr             d1,       [\s1, #2*\w]
        ldr             \reg\()2, [\s2]
        ldr             d3,       [\s2, #2*\w]
        str             \reg\()0, [x0]
        str             d1,       [x0, #2*\w]
        add             x0,  x0,  #2*\stride
        str             \reg\()2, [x0]
        str             d3,       [x0, #2*\w]
.if \ret
        ret
.else
        add             x0,  x0,  #2*\stride
        b               3f
.endif

1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        ldr             \reg\()0, [\s1]
        ldr             s1,       [\s1, #2*\w]
        ldr             \reg\()2, [\s2]
        ldr             s3,       [\s2, #2*\w]
        str             \reg\()0, [x0]
        str             s1,       [x0, #2*\w]
        str             s31,      [x0, #2*\w+4]
        add             x0,  x0,  #2*\stride
        str             \reg\()2, [x0]
        str             s3,       [x0, #2*\w]
        str             s31,      [x0, #2*\w+4]
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
        ldr             \reg\()0, [\s1]
        ldr             s1,       [\s1, #2*\w]
        ldr             \reg\()2, [\s2]
        ldr             s3,       [\s2, #2*\w]
        str             s31, [x0]
        stur            \reg\()0, [x0, #4]
        str             s1,       [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        str             s31, [x0]
        stur            \reg\()2, [x0, #4]
        str             s3,       [x0, #4+2*\w]
.if \ret
        ret
.else
        add             x0,  x0,  #2*\stride
        b               3f
.endif

1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        ldr             \reg\()0, [\s1]
        ldr             \reg\()1, [\s2]
        str             s31,      [x0]
        stur            \reg\()0, [x0, #4]
        str             s31,      [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        str             s31,      [x0]
        stur            \reg\()1, [x0, #4]
        str             s31,      [x0, #4+2*\w]
.if \ret
        ret
.else
        add             x0,  x0,  #2*\stride
.endif
3:
.endm

.macro load_n_incr_16 dst, src, incr, w
.if \w == 4
        ld1             {\dst\().4h}, [\src], \incr
.else
        ld1             {\dst\().8h}, [\src], \incr
.endif
.endm

// void dav1d_cdef_paddingX_16bpc_neon(uint16_t *tmp, const pixel *src,
//                                     ptrdiff_t src_stride, const pixel (*left)[2],
//                                     const pixel *const top,
//                                     const pixel *const bottom, int h,
//                                     enum CdefEdgeFlags edges);

.macro padding_func_16 w, stride, reg
function cdef_padding\w\()_16bpc_neon, export=1
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
        pad_top_bot_16  x4,  x9, \w, \stride, \reg, 0

        // Middle section
3:
        tst             w7,  #1 // CDEF_HAVE_LEFT
        b.eq            2f
        // CDEF_HAVE_LEFT
        tst             w7,  #2 // CDEF_HAVE_RIGHT
        b.eq            1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        ld1             {v0.s}[0], [x3], #4
        ldr             s2,       [x1, #2*\w]
        load_n_incr_16  v1,  x1,  x2,  \w
        subs            w6,  w6,  #1
        str             s0,       [x0]
        stur            \reg\()1, [x0, #4]
        str             s2,       [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        b.gt            0b
        b               3f
1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        ld1             {v0.s}[0], [x3], #4
        load_n_incr_16  v1,  x1,  x2,  \w
        subs            w6,  w6,  #1
        str             s0,       [x0]
        stur            \reg\()1, [x0, #4]
        str             s31,      [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        b.gt            1b
        b               3f
2:
        tst             w7,  #2 // CDEF_HAVE_RIGHT
        b.eq            1f
        // !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        ldr             s1,       [x1, #2*\w]
        load_n_incr_16  v0,  x1,  x2,  \w
        subs            w6,  w6,  #1
        str             s31,      [x0]
        stur            \reg\()0, [x0, #4]
        str             s1,       [x0, #4+2*\w]
        add             x0,  x0,  #2*\stride
        b.gt            0b
        b               3f
1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        load_n_incr_16  v0,  x1,  x2,  \w
        subs            w6,  w6,  #1
        str             s31,      [x0]
        stur            \reg\()0, [x0, #4]
        str             s31,      [x0, #4+2*\w]
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
        pad_top_bot_16  x5,  x9, \w, \stride, \reg, 1
endfunc
.endm

padding_func_16 8, 16, q
padding_func_16 4, 8,  d

tables

filter 8, 16
filter 4, 16

find_dir 16
