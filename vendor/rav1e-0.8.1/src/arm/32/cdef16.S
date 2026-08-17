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

// r1 = d0/q0
// r2 = d2/q1
.macro pad_top_bot_16 s1, s2, w, stride, r1, r2, align, ret
        tst             r7,  #1 // CDEF_HAVE_LEFT
        beq             2f
        // CDEF_HAVE_LEFT
        tst             r7,  #2 // CDEF_HAVE_RIGHT
        beq             1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        vldr            s8,  [\s1, #-4]
        vld1.16         {\r1}, [\s1, :\align]
        vldr            s9,  [\s1, #2*\w]
        vldr            s10, [\s2, #-4]
        vld1.16         {\r2}, [\s2, :\align]
        vldr            s11, [\s2, #2*\w]
        vstr            s8,  [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
        vstr            s9,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s10, [r0, #-4]
        vst1.16         {\r2}, [r0, :\align]
        vstr            s11, [r0, #2*\w]
.if \ret
        pop             {r4-r8,pc}
.else
        add             r0,  r0,  #2*\stride
        b               3f
.endif

1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        vldr            s8,  [\s1, #-4]
        vld1.16         {\r1}, [\s1, :\align]
        vldr            s9,  [\s2, #-4]
        vld1.16         {\r2}, [\s2, :\align]
        vstr            s8,  [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s9,  [r0, #-4]
        vst1.16         {\r2}, [r0, :\align]
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
        vld1.16         {\r1}, [\s1, :\align]
        vldr            s8,  [\s1, #2*\w]
        vld1.16         {\r2}, [\s2, :\align]
        vldr            s9,  [\s2, #2*\w]
        vstr            s12, [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
        vstr            s8,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s12, [r0, #-4]
        vst1.16         {\r2}, [r0, :\align]
        vstr            s9,  [r0, #2*\w]
.if \ret
        pop             {r4-r8,pc}
.else
        add             r0,  r0,  #2*\stride
        b               3f
.endif

1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        vld1.16         {\r1}, [\s1, :\align]
        vld1.16         {\r2}, [\s2, :\align]
        vstr            s12, [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        vstr            s12, [r0, #-4]
        vst1.16         {\r2}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
.if \ret
        pop             {r4-r8,pc}
.else
        add             r0,  r0,  #2*\stride
.endif
3:
.endm

// void dav1d_cdef_paddingX_16bpc_neon(uint16_t *tmp, const pixel *src,
//                                     ptrdiff_t src_stride, const pixel (*left)[2],
//                                     const pixel *const top,
//                                     const pixel *const bottom, int h,
//                                     enum CdefEdgeFlags edges);

// r1 = d0/q0
// r2 = d2/q1
.macro padding_func_16 w, stride, r1, r2, align
function cdef_padding\w\()_16bpc_neon, export=1
        push            {r4-r8,lr}
        ldrd            r4,  r5,  [sp, #24]
        ldrd            r6,  r7,  [sp, #32]
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
        pad_top_bot_16  r4,  r8,  \w, \stride, \r1, \r2, \align, 0

        // Middle section
3:
        tst             r7,  #1 // CDEF_HAVE_LEFT
        beq             2f
        // CDEF_HAVE_LEFT
        tst             r7,  #2 // CDEF_HAVE_RIGHT
        beq             1f
        // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        vld1.32         {d2[]}, [r3, :32]!
        vldr            s5,  [r1, #2*\w]
        vld1.16         {\r1}, [r1, :\align], r2
        subs            r6,  r6,  #1
        vstr            s4,  [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
        vstr            s5,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        bgt             0b
        b               3f
1:
        // CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        vld1.32         {d2[]}, [r3, :32]!
        vld1.16         {\r1}, [r1, :\align], r2
        subs            r6,  r6,  #1
        vstr            s4,  [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
        vstr            s12, [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        bgt             1b
        b               3f
2:
        tst             r7,  #2 // CDEF_HAVE_RIGHT
        beq             1f
        // !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
0:
        vldr            s4,  [r1, #2*\w]
        vld1.16         {\r1}, [r1, :\align], r2
        subs            r6,  r6,  #1
        vstr            s12, [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
        vstr            s4,  [r0, #2*\w]
        add             r0,  r0,  #2*\stride
        bgt             0b
        b               3f
1:
        // !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        vld1.16         {\r1}, [r1, :\align], r2
        subs            r6,  r6,  #1
        vstr            s12, [r0, #-4]
        vst1.16         {\r1}, [r0, :\align]
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
        pad_top_bot_16  r5,  r8,  \w, \stride, \r1, \r2, \align, 1
endfunc
.endm

padding_func_16 8, 16, q0, q1, 128
padding_func_16 4, 8,  d0, d2, 64

tables

filter 8, 16
filter 4, 16

find_dir 16
