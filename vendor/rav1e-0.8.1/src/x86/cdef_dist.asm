; Copyright (c) 2022, The rav1e contributors. All rights reserved
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

%if ARCH_X86_64

; m0: zero register
; m1: src input
; m2: dst input
; m3 = sum(src_{i,j})
; m4 = sum(src_{i,j}^2)
; m5 = sum(dst_{i,j})
; m6 = sum(dst_{i,j}^2)
; m7 = sum(src_{i,j} * dst_{i,j})
; m8: tmp register
%macro CDEF_DIST_W8_SSE2 0
    psadbw              m8, m1, m0 ; sum pixel values
    paddd               m3, m8     ; accumulate
    punpcklbw           m1, m0     ; convert to 16-bits
    pmaddwd             m8, m1, m1 ; square and horz add
    paddd               m4, m8     ; accumulate
    psadbw              m8, m2, m0 ; same as above, but for dst
    paddd               m5, m8
    punpcklbw           m2, m0
    pmaddwd             m8, m2, m2
    paddd               m6, m8
    pmaddwd             m8, m1, m2 ; src_{i,j} * dst_{i,j} (and horz add)
    paddd               m7, m8
%endmacro

; Refine sums into variances and sse
; parameter: scale log2 relative to 8x8
%macro CDEF_DIST_REFINE_SSE2 1
    ; Compute [sum(src)^2, sum(dst)^2]
    punpckldq           m3, m5 ; store sums in a single vector
    pcmpeqd             m0, m0 ; -1 (for rounding)
    pmaddwd             m3, m3

    ; Divide by area and round
    pslld               m0, 5 - %1
    psubd               m3, m0 ; + (1 << (5 - %1))
    psrld               m3, 6 - %1

    pshufd              m0, m7, q3232 ; reduce sum(src * dst)
    punpckldq           m1, m4, m6    ; reduce [sum(src^2), sum(dst^2)]
    paddd               m7, m0
    punpckhdq           m4, m6
    paddd               m1, m4
    paddd               m7, m7 ; 2 * sum(src * dst) [Partially reduced; len 2]
    pshufd              m0, m1, q3232

    ; Equivelent to:
    ; paddd m1, m0
    ; psubd m0, m1, m7 ; sse = sum(src^2) + sum(dst^2) - sum(src * dst)
    ; but with fewer dependancies
    psubd               m7, m1
    paddd               m1, m0 ; [sum(src^2), sum(dst^2)]
    psubd               m0, m7 ; sse (Needs reducing; len 2)

    psubd               m1, m3 ; [src variance, dst variance]
    ; Scale up the variances up to 8x8
    ; TODO: this can be handled inside ssim boost in the future
%if %1 != 0
    pslld               m1, %1
%endif
    movq        [ret_ptrq], m1

    ; Final reduce for sse
    pshuflw             m2, m0, q3232
    paddd               m0, m2
    movd      [ret_ptrq+8], m0
%endmacro

INIT_XMM sse2
cglobal cdef_dist_kernel_4x4, 5, 5, 9, \
        src, src_stride, dst, dst_stride, ret_ptr
    pxor                m0, m0
    movd                m1, [srcq]
    movd                m2, [srcq+src_strideq]
    punpckldq           m1, m2
    movd                m2, [dstq]
    movd                m8, [dstq+dst_strideq]
    lea               srcq, [srcq+2*src_strideq]
    lea               dstq, [dstq+2*dst_strideq]
    punpckldq           m2, m8
    psadbw              m3, m1, m0
    punpcklbw           m1, m0
    pmaddwd             m4, m1, m1
    psadbw              m5, m2, m0
    punpcklbw           m2, m0
    pmaddwd             m6, m2, m2
    pmaddwd             m7, m1, m2

    movd                m1, [srcq]
    movd                m2, [srcq+src_strideq]
    punpckldq           m1, m2
    movd                m2, [dstq]
    movd                m8, [dstq+dst_strideq]
    punpckldq           m2, m8
    CDEF_DIST_W8_SSE2

    CDEF_DIST_REFINE_SSE2 2
    RET

cglobal cdef_dist_kernel_4x8, 5, 7, 9, \
        src, src_stride, dst, dst_stride, ret_ptr, src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]

    pxor                m0, m0
    movd                m1, [srcq]
    movd                m2, [srcq+src_strideq]
    punpckldq           m1, m2
    movd                m2, [dstq]
    movd                m8, [dstq+dst_strideq]
    punpckldq           m2, m8
    psadbw              m3, m1, m0
    punpcklbw           m1, m0
    pmaddwd             m4, m1, m1
    psadbw              m5, m2, m0
    punpcklbw           m2, m0
    pmaddwd             m6, m2, m2
    pmaddwd             m7, m1, m2

    movd                m1, [srcq+2*src_strideq]
    movd                m2, [srcq+src_stride3q]
    punpckldq           m1, m2
    movd                m2, [dstq+2*dst_strideq]
    movd                m8, [dstq+dst_stride3q]
    lea               srcq, [srcq+4*src_strideq]
    lea               dstq, [dstq+4*dst_strideq]
    punpckldq           m2, m8
    CDEF_DIST_W8_SSE2
    movd                m1, [srcq]
    movd                m2, [srcq+src_strideq]
    punpckldq           m1, m2
    movd                m2, [dstq]
    movd                m8, [dstq+dst_strideq]
    punpckldq           m2, m8
    CDEF_DIST_W8_SSE2
    movd                m1, [srcq+2*src_strideq]
    movd                m2, [srcq+src_stride3q]
    punpckldq           m1, m2
    movd                m2, [dstq+2*dst_strideq]
    movd                m8, [dstq+dst_stride3q]
    punpckldq           m2, m8
    CDEF_DIST_W8_SSE2

    CDEF_DIST_REFINE_SSE2 1
    RET

cglobal cdef_dist_kernel_8x4, 5, 5, 9, \
        src, src_stride, dst, dst_stride, ret_ptr
    pxor                m0, m0
    movq                m1, [srcq]
    psadbw              m3, m1, m0
    punpcklbw           m1, m0
    pmaddwd             m4, m1, m1
    movq                m2, [dstq]
    psadbw              m5, m2, m0
    punpcklbw           m2, m0
    pmaddwd             m6, m2, m2
    pmaddwd             m7, m1, m2

    movq                m1, [srcq+src_strideq]
    movq                m2, [dstq+dst_strideq]
    lea               srcq, [srcq+2*src_strideq]
    lea               dstq, [dstq+2*dst_strideq]
    CDEF_DIST_W8_SSE2
    movq                m1, [srcq]
    movq                m2, [dstq]
    CDEF_DIST_W8_SSE2
    movq                m1, [srcq+src_strideq]
    movq                m2, [dstq+dst_strideq]
    CDEF_DIST_W8_SSE2

    CDEF_DIST_REFINE_SSE2 1
    RET

cglobal cdef_dist_kernel_8x8, 5, 7, 9, \
        src, src_stride, dst, dst_stride, ret_ptr, src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]

    pxor                m0, m0
    movq                m1, [srcq]
    psadbw              m3, m1, m0
    punpcklbw           m1, m0
    pmaddwd             m4, m1, m1
    movq                m2, [dstq]
    psadbw              m5, m2, m0
    punpcklbw           m2, m0
    pmaddwd             m6, m2, m2
    pmaddwd             m7, m1, m2

    movq                m1, [srcq+src_strideq]
    movq                m2, [dstq+dst_strideq]
    CDEF_DIST_W8_SSE2
    movq                m1, [srcq+2*src_strideq]
    movq                m2, [dstq+2*dst_strideq]
    CDEF_DIST_W8_SSE2
    movq                m1, [srcq+src_stride3q]
    movq                m2, [dstq+dst_stride3q]
    CDEF_DIST_W8_SSE2
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]

    movq                m1, [srcq]
    movq                m2, [dstq]
    CDEF_DIST_W8_SSE2
    movq                m1, [srcq+src_strideq]
    movq                m2, [dstq+dst_strideq]
    CDEF_DIST_W8_SSE2
    movq                m1, [srcq+2*src_strideq]
    movq                m2, [dstq+2*dst_strideq]
    CDEF_DIST_W8_SSE2
    movq                m1, [srcq+src_stride3q]
    movq                m2, [dstq+dst_stride3q]
    CDEF_DIST_W8_SSE2

    CDEF_DIST_REFINE_SSE2 0
    RET
%endif ; ARCH_X86_64
