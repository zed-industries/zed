; Copyright (c) 2018, The rav1e contributors. All rights reserved
;
; This source code is subject to the terms of the BSD 2 Clause License and
; the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
; was not distributed with this source code in the LICENSE file, you can
; obtain it at www.aomedia.org/license/software. If the Alliance for Open
; Media Patent License 1.0 was not distributed with this source code in the
; PATENTS file, you can obtain it at www.aomedia.org/license/patent.

%include "config.asm"
%include "ext/x86/x86inc.asm"

SECTION .text

%macro W_ABS_DIFF 8
    psubw               %1, %5
    psubw               %2, %6
    psubw               %3, %7
    psubw               %4, %8
    pabsw               %1, %1
    pabsw               %2, %2
    pabsw               %3, %3
    pabsw               %4, %4
%endmacro

INIT_XMM ssse3
cglobal sad_4x4_hbd, 4, 6, 8, src, src_stride, dst, dst_stride, \
                              src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    movq                m0, [srcq]
    movq                m1, [srcq+src_strideq*1]
    movq                m2, [srcq+src_strideq*2]
    movq                m3, [srcq+src_stride3q]
    movq                m4, [dstq]
    movq                m5, [dstq+dst_strideq*1]
    movq                m6, [dstq+dst_strideq*2]
    movq                m7, [dstq+dst_stride3q]
    W_ABS_DIFF m0, m1, m2, m3, m4, m5, m6, m7
; Don't convert to 32 bit integers: 4*4 abs diffs of 12-bits fits in 16 bits.
; Accumulate onto m0
    %define            sum  m0
    paddw              sum, m1
    paddw               m2, m3
    paddw              sum, m2
; Horizontal reduction
    pshuflw             m1, sum, q2323
    paddw              sum, m1
    pshuflw             m1, sum, q1111
    paddw              sum, m1
    movd               eax, sum
; Convert to 16-bits since the upper half of eax is dirty
    movzx              eax, ax
    RET

%if ARCH_X86_64

INIT_XMM ssse3
cglobal sad_16x16_hbd, 4, 5, 10, src, src_stride, dst, dst_stride, \
                                 cnt
    mov               cntd, 8
    %define            sum  m0
    pxor               sum, sum
    pxor                m9, m9
.loop:
    movu                m1, [srcq]
    movu                m2, [srcq+16]
    movu                m3, [srcq+src_strideq]
    movu                m4, [srcq+src_strideq+16]
    lea               srcq, [srcq+src_strideq*2]
    movu                m5, [dstq]
    movu                m6, [dstq+16]
    movu                m7, [dstq+dst_strideq]
    movu                m8, [dstq+dst_strideq+16]
    lea               dstq, [dstq+dst_strideq*2]
    W_ABS_DIFF m1, m2, m3, m4, m5, m6, m7, m8
    paddw               m1, m2
    paddw               m3, m4
; Convert to 32-bits
    punpcklwd           m2, m1, m9
    punpcklwd           m4, m3, m9
    punpckhwd           m1, m9
    punpckhwd           m3, m9
    paddd               m1, m2
    paddd               m3, m4
    paddd              sum, m1
    paddd              sum, m3
    dec               cntd
    jg .loop
; Horizontal reduction
    movhlps             m1, sum
    paddd              sum, m1
    pshufd              m1, sum, q1111
    paddd              sum, m1
    movd               eax, sum
    RET

%endif
