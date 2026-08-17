; Copyright (c) 2019, The rav1e contributors. All rights reserved
;
; This source code is subject to the terms of the BSD 2 Clause License and
; the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
; was not distributed with this source code in the LICENSE file, you can
; obtain it at www.aomedia.org/license/software. If the Alliance for Open
; Media Patent License 1.0 was not distributed with this source code in the
; PATENTS file, you can obtain it at www.aomedia.org/license/patent.

%include "config.asm"
%include "ext/x86/x86inc.asm"

%if ARCH_X86_64

SECTION_RODATA 32
maddubsw_hsub: times 16 db 1, -1

SECTION .text

%define m(x) mangle(private_prefix %+ _ %+ x %+ SUFFIX)

; Perform 4x4 hadamard transform on input with 2 rows per register.
; Rows 0 and 2 are in m0 and rows 1 and 3 are in m1.
; A second set of packed input can also be taken in m2 and m3.
; Ends with sums in every other entry (i.e. already reduced horizontally).
%macro HADAMARD_4x4_PACKED 1
%if %1 == 1
    %define tmp m2
    ; 2->0, 1->2, 0->2
    %define ROTATE SWAP 2, 1, 0
%elif %1 == 2
    %define tmp m4
    ; 4->0, 3->2, 2->3, 1->2, 0->1
    %define ROTATE SWAP 4, 3, 2, 1, 0
%endif
    ; m0  d2 c2 b2 a2 d0 c0 b0 a0
    ; m1  d3 c3 b3 a3 d1 c1 b1 a1

    ; Stage 1
    ; m0  d2+d3 c2+c3 b2+b3 a2+a3 d0+d1 c0+c1 b0+b1 a0+a1
    ; m1  d2-d3 c2-c3 b2-b3 a2-a3 d0-d1 c0-c1 b0-b1 a0-a1
    paddw              tmp, m0, m1
    psubw               m0, m1
%if %1 == 2
    paddw               m1, m2, m3
    psubw               m2, m3
%endif
    ROTATE

    ; Stage 2
    ; m0  d0-d1 d0+d1 c0-c1 c0+c1 b0-b1 b0+b1 a0-a1 a0+a1
    ; m1  d2-d3 d2+d3 c2-c3 c2+c3 b2-b3 b2+b3 a2-a3 a2+a3
    punpcklwd          tmp, m0, m1
    punpckhwd           m0, m1
%if %1 == 2
    punpcklwd           m1, m2, m3
    punpckhwd           m2, m3
%endif
    ROTATE

    ; m0  d0-d1+d2-d3 d0+d1+d2+d3 c0-c1+c2-c3 c0+c1+c2+c3
    ;     b0-b1+b2-b3 b0+b1+b2+b3 a0-a1+a2-a3 a0+a1+a2+a3
    ; m1  d0-d2-d2+d3 d0+d1-d2-d3 c0-c1-c2+c3 c0+c1-c2-c3
    ;     b0-b1-b2+b3 b0+b1-b2-b3 a0-a1-a2-a3 a0+a1-a2-a3
    paddw              tmp, m0, m1
    psubw               m0, m1
%if %1 == 2
    paddw               m1, m2, m3
    psubw               m2, m3
%endif
    ROTATE

    ; m0  s2 s0 r2 r0 q2 q0 p2 p0
    ; m1  s3 s1 r3 r1 q3 q1 p3 p1

    ; Stage 1
    ; m0  q3 q1 q2 q0 p3 p1 p2 p0
    ; m1  s3 s1 s2 s0 r3 r1 r2 r0
    punpckldq          tmp, m0, m1
    punpckhdq           m0, m1
%if %1 == 2
    punpckldq           m1, m2, m3
    punpckhdq           m2, m3
%endif
    ROTATE

    ; m0  q3+s3 q1+s1 q2+s2 q0+s0 p3+r3 p1+r1 p2+r2 p0+r0
    ; m1  q3-s3 q1-s1 q2-s2 q0-s0 p3-r3 p1-r1 p2-r2 p0-r0
    paddw              tmp, m0, m1
    psubw               m0, m1
%if %1 == 2
    paddw               m1, m2, m3
    psubw               m2, m3
%endif
    ROTATE

    ; Stage 2
    ; m0  p3-r3 p1-r1 p2-r2 p0-r0 p3+r3 p1+r1 p2+r2 p0+r0
    ; m1  q3-s3 q1-s1 q2-s2 q0-s0 q3+s3 q1+s1 q2+s2 q0+s0
    punpcklqdq         tmp, m0, m1
    punpckhqdq          m0, m1
%if %1 == 2
    punpcklqdq          m1, m2, m3
    punpckhqdq          m2, m3
%endif
    ROTATE

    ; Use the fact that
    ;   (abs(a+b)+abs(a-b))/2 = max(abs(a),abs(b))
    ;  to merge the final butterfly with the abs and the first stage of
    ;  accumulation.
    ; Avoid pabsw by using max(a, b) + max(a + b + 0x7FFF, 0x7FFF) instead.
    ; Actually calculates (abs(a+b)+abs(a-b))/2-0x7FFF.
    ; The final sum must be offset to compensate for subtracting 0x7FFF.
    paddw              tmp, m0, m1
    pmaxsw              m0, m1
    ; m1 is free
    ; 0x7FFF
    pcmpeqb             m1, m1
    psrlw               m1, 1

    paddsw             tmp, m1
    psubw               m0, tmp
%if %1 == 2
    paddw              tmp, m2, m3
    pmaxsw              m2, m3
    paddsw             tmp, m1
    psubw               m2, tmp

    paddw               m0, m2
%endif
%endmacro

; Load diffs of 4 entries for 2 rows
%macro LOAD_PACK_DIFF_Dx2 7
    movd               m%1, %2
    movd               m%6, %4
    punpckldq          m%1, m%6
    pmovzxbw           m%1, m%1
    movd               m%6, %3
    movd               m%7, %5
    punpckldq          m%6, m%7
    pmovzxbw           m%6, m%6
    psubw              m%1, m%6
%endmacro

; Can only use 128-bit vectors
%macro SATD_4x4_FN 0
cglobal satd_4x4, 4, 6, 4, src, src_stride, dst, dst_stride, \
                           src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]

    ; Load rows 0 and 2 to m0 and 1 and 3 to m1
    LOAD_PACK_DIFF_Dx2 0, [srcq], [dstq], \
                          [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                          2, 3
    LOAD_PACK_DIFF_Dx2 1, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                          [srcq+src_stride3q], [dstq+dst_stride3q], \
                          2, 3

    HADAMARD_4x4_PACKED 1

    ; Reduce horizontally
    pshufd              m1, m0, q3232
    paddw               m0, m1
    pshuflw             m1, m0, q3232
    paddw               m0, m1
    pshuflw             m1, m0, q1111

    ; Perform normalization during the final stage of accumulation
    pavgw               m0, m1
    movd               eax, m0
    movzx              eax, ax

    ; Add an offset for how the final butterfly stage and the first stage of
    ;  accumulation was done. Since this offset is an even number, this can
    ;  safely be done after normalization using pavgw.
    sub                 ax, 4
    RET
%endmacro

INIT_XMM sse4
SATD_4x4_FN

INIT_XMM avx2
SATD_4x4_FN

; Load diffs of 8 entries for 2 row
; Each set of 4 columns share an 128-bit lane
%macro LOAD_PACK_DIFF_Qx2 7
    movq              xm%1, %2
    movq              xm%6, %4
    punpckldq         xm%1, xm%6
    pmovzxbw           m%1, xm%1
    movq              xm%6, %3
    movq              xm%7, %5
    punpckldq         xm%6, xm%7
    pmovzxbw           m%6, xm%6
    psubw              m%1, m%6
%endmacro

INIT_YMM avx2
cglobal satd_8x4, 4, 6, 4, src, src_stride, dst, dst_stride, \
                           src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    ; Load rows 0 and 2 to m0 and 1 and 3 to m1
    ; Each set of 4 columns share 128-bit lanes
    LOAD_PACK_DIFF_Qx2 0, [srcq], [dstq], \
                          [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                       2, 3
    LOAD_PACK_DIFF_Qx2 1, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                          [srcq+src_stride3q], [dstq+dst_stride3q], \
                       2, 3

    HADAMARD_4x4_PACKED 1

    ; Reduce horizontally
    vextracti128       xm1, m0, 1
    paddw              xm0, xm1
    pshufd             xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q1111

    ; Perform normalization during the final stage of accumulation
    pavgw              xm0, xm1
    movd               eax, xm0
    movzx              eax, ax

    ; Add an offset for how the final butterfly stage and the first stage of
    ;  accumulation was done. Since this offset is an even number, this can
    ;  safely be done after normalization using pavgw.
    sub                 ax, 8
    RET

; Load diffs of 4 entries for 4 rows
; Each set of two rows share 128-bit lanes
%macro LOAD_PACK_DIFF_Dx4 12
    movd              xm%1, %2
    movd             xm%10, %4
    punpckldq         xm%1, xm%10
    movd             xm%10, %6
    movd             xm%11, %8
    punpckldq        xm%10, xm%11
    punpcklqdq        xm%1, xm%10
    pmovzxbw           m%1, xm%1
    movd             xm%10, %3
    movd             xm%11, %5
    punpckldq        xm%10, xm%11
    movd             xm%11, %7
    movd             xm%12, %9
    punpckldq        xm%11, xm%12
    punpcklqdq       xm%10, xm%11
    pmovzxbw          m%10, xm%10
    psubw              m%1, m%10
%endmacro

INIT_YMM avx2
cglobal satd_4x8, 4, 8, 5, src, src_stride, dst, dst_stride, \
                           src4, dst4, src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    lea              src4q, [srcq+src_strideq*4]
    lea              dst4q, [dstq+dst_strideq*4]
    ; Load rows 0, 2, 4 and 6 to m0 and 1, 3, 5 and 7 to m1.
    ; Lanes split the low and high rows of m0 and m1.
    LOAD_PACK_DIFF_Dx4 0, [srcq], [dstq], \
                          [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                          [src4q], [dst4q], \
                          [src4q+src_strideq*2], [dst4q+dst_strideq*2], \
                       2, 3, 4
    LOAD_PACK_DIFF_Dx4 1, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                          [srcq+src_stride3q], [dstq+dst_stride3q], \
                          [src4q+src_strideq*1], [dst4q+dst_strideq*1], \
                          [src4q+src_stride3q], [dst4q+dst_stride3q], \
                       2, 3, 4

    HADAMARD_4x4_PACKED 1

    ; Reduce horizontally
    vextracti128       xm1, m0, 1
    paddw              xm0, xm1
    pshufd             xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q1111

    ; Perform normalization during the final stage of accumulation.
    pavgw              xm0, xm1
    movd               eax, xm0
    movzx              eax, ax
    sub                 ax, 8
    RET

; Rudimentary fast hadamard transform
; Two Hadamard transforms share an 128-bit lane.
%macro HADAMARD_4x4 0
    ; 4->0, 3->2, 2->3, 1->2, 0->1
    %define ROTATE SWAP 4, 3, 2, 1, 0

    ; Stage 1
    paddw               m0, m1, m2
    psubw               m1, m2
    paddw               m2, m3, m4
    psubw               m3, m4
    ROTATE

    ; Stage 2
    paddw               m0, m1, m3
    psubw               m1, m3
    paddw               m3, m2, m4
    psubw               m2, m4
    SWAP                3, 2, 1
    ROTATE

    ; Transpose
    ; Since two transforms share an 128-bit lane, unpacking results in a single
    ;  transform's values on each register. This has to be resolved later.
    ; A and B indicate different 4x4 transforms.

    ; Start
    ; m1  B (a3 a2 a1 a0) A (a3 a2 a1 a0)
    ; m2  B (b3 b2 b1 b0) A (b3 b2 b1 b0)
    ; m3  B (c3 c2 c1 c0) A (c3 c2 c1 c0)
    ; m4  B (d3 d2 d1 d0) A (d3 d2 d1 d0)

    ; Stage 1
    ; m1  A (b3 a3 b2 a2 b1 a1 b0 a0)
    ; m2  B (b3 a3 b2 a2 b1 a1 b0 a0)
    ; m3  A (d3 c3 d2 c2 d1 c1 d0 c0)
    ; m4  B (d3 c3 d2 c2 d1 c1 d0 c0)
    punpcklwd           m0, m1, m2
    punpckhwd           m1, m2
    punpcklwd           m2, m3, m4
    punpckhwd           m3, m4
    ROTATE

    ; m1  A (d3 c3 b3 a3 d2 c2 b2 a2)
    ; m2  A (d1 c1 b1 a1 d0 c0 b0 a0)
    ; m3  B (d3 c3 b3 a3 d2 c2 b2 a2)
    ; m4  B (d1 c1 b1 a1 d0 c0 b0 a0)
    punpckldq           m0, m1, m3
    punpckhdq           m1, m3
    punpckldq           m3, m2, m4
    punpckhdq           m2, m4
    SWAP                3, 2, 1
    ROTATE

    ; Make the transforms share 128-bit lanes again.
    ; m1  B (d0 c0 b0 a0) A (d0 c0 b0 a0)
    ; m2  B (d1 c1 b1 a1) A (d1 c1 b1 a1)
    ; m3  B (d2 c2 b2 a2) A (d2 c2 b2 a2)
    ; m4  B (d3 c3 b3 a3) A (d3 c3 b3 a3)
    punpcklqdq          m0, m1, m2
    punpckhqdq          m1, m2
    punpcklqdq          m2, m3, m4
    punpckhqdq          m3, m4
    ROTATE

    ; Stage 1
    paddw               m0, m1, m2
    psubw               m1, m2
    paddw               m2, m3, m4
    psubw               m3, m4
    ROTATE

    ; Use the fact that
    ;   (abs(a+b)+abs(a-b))/2 = max(abs(a),abs(b))
    ;  to merge the final butterfly with the abs and the first stage of
    ;  accumulation.
    ; Avoid pabsw by using max(a, b) + max(a + b + 0x7FFF, 0x7FFF) instead.
    ; Actually calculates (abs(a+b)+abs(a-b))/2-0x7FFF.
    ; The final sum must be offset to compensate for subtracting 0x7FFF.
    paddw               m0, m1, m3
    pmaxsw              m1, m3
    ; m2 is free
    ; 0x7FFF
    pcmpeqb             m3, m3
    psrlw               m3, 1

    paddsw              m0, m3
    psubw               m1, m0

    paddw               m0, m2, m4
    pmaxsw              m2, m4
    paddsw              m0, m3
    psubw               m2, m0

    paddw               m1, m2
    SWAP                1, 0
%endmacro

; Load diffs of 16 entries for 1 row
%macro LOAD_DIFF_DQ 4
    movu              xm%1, %2
    movu              xm%4, %3
    vpmovzxbw          m%1, xm%1
    vpmovzxbw          m%4, xm%4
    psubw              m%1, m%4
%endmacro

INIT_YMM avx2
cglobal satd_16x4, 4, 6, 5, src, src_stride, dst, dst_stride, \
                            src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    LOAD_DIFF_DQ 1, [srcq], [dstq], 0
    LOAD_DIFF_DQ 2, [srcq+src_strideq*1], [dstq+dst_strideq*1], 0
    LOAD_DIFF_DQ 3, [srcq+src_strideq*2], [dstq+dst_strideq*2], 0
    LOAD_DIFF_DQ 4, [srcq+src_stride3q], [dstq+dst_stride3q], 0

    HADAMARD_4x4

    ; Reduce horizontally
    vextracti128       xm1, m0, 1
    paddw              xm0, xm1
    pshufd             xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q1111

    ; Perform normalization during the final stage of accumulation
    ; Avoids overflow in this case
    pavgw              xm0, xm1
    movd               eax, xm0
    movzx              eax, ax

    ; Add an offset for how the final butterfly stage and the first stage of
    ;  accumulation was done. Since this offset is an even number, this can
    ;  safely be done after normalization using pavgw.
    sub                 ax, 16
    RET

INIT_YMM avx2
cglobal satd_4x16, 4, 8, 7, src, src_stride, dst, dst_stride, \
                            src4, dst4, src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    lea              src4q, [srcq+src_strideq*4]
    lea              dst4q, [dstq+dst_strideq*4]
    LOAD_PACK_DIFF_Dx4 0, [srcq], [dstq], \
                          [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                          [src4q], [dst4q], \
                          [src4q+src_strideq*2], [dst4q+dst_strideq*2], \
                       4, 5, 6
    LOAD_PACK_DIFF_Dx4 1, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                          [srcq+src_stride3q], [dstq+dst_stride3q], \
                          [src4q+src_strideq*1], [dst4q+dst_strideq*1], \
                          [src4q+src_stride3q], [dst4q+dst_stride3q], \
                       4, 5, 6
    lea               srcq, [srcq+src_strideq*8]
    lea               dstq, [dstq+dst_strideq*8]
    lea              src4q, [src4q+src_strideq*8]
    lea              dst4q, [dst4q+dst_strideq*8]
    LOAD_PACK_DIFF_Dx4 2, [srcq], [dstq], \
                          [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                          [src4q], [dst4q], \
                          [src4q+src_strideq*2], [dst4q+dst_strideq*2], \
                       4, 5, 6
    LOAD_PACK_DIFF_Dx4 3, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                          [srcq+src_stride3q], [dstq+dst_stride3q], \
                          [src4q+src_strideq*1], [dst4q+dst_strideq*1], \
                          [src4q+src_stride3q], [dst4q+dst_stride3q], \
                       4, 5, 6
    HADAMARD_4x4_PACKED 2

    ; Reduce horizontally
    vextracti128       xm1, m0, 1
    paddw              xm0, xm1
    pshufd             xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddw              xm0, xm1
    pshuflw            xm1, xm0, q1111

    ; Perform normalization during the final stage of accumulation
    pavgw              xm0, xm1
    movd               eax, xm0
    movzx              eax, ax

    ; Add an offset for how the final butterfly stage and the first stage of
    ;  accumulation was done. Since this offset is an even number, this can
    ;  safely be done after normalization using pavgw.
    sub                 ax, 16
    RET

; On x86-64 we can transpose in-place without spilling registers.
; By clever choices of the order to apply the butterflies and the order of
;  their outputs, we can take the rows in order and output the columns in order
;  without any extra operations and using just one temporary register.
%macro TRANSPOSE8x8 9
    punpckhwd           m%9, m%5, m%6
    punpcklwd           m%5, m%6
    ; m%6 is free
    punpckhwd           m%6, m%1, m%2
    punpcklwd           m%1, m%2
    ; m%2 is free
    punpckhwd           m%2, m%7, m%8
    punpcklwd           m%7, m%8
    ; m%8 is free
    punpckhwd           m%8, m%3, m%4
    punpcklwd           m%3, m%4
    ; m%4 is free
    punpckhdq           m%4, m%1, m%3
    punpckldq           m%1, m%3
    ; m%3 is free
    punpckldq           m%3, m%5, m%7
    punpckhdq           m%5, m%7
    ; m%7 is free
    punpckhdq           m%7, m%6, m%8
    punpckldq           m%6, m%8
    ; m%8 is free
    punpckldq           m%8, m%9, m%2
    punpckhdq           m%9, m%2
    ; m%2 is free
    punpckhqdq          m%2, m%1, m%3
    punpcklqdq          m%1, m%3
    ; m%3 is free
    punpcklqdq          m%3, m%4, m%5
    punpckhqdq          m%4, m%5
    ; m%5 is free
    punpcklqdq          m%5, m%6, m%8
    punpckhqdq          m%6, m%8
    ; m%8 is free
    punpckhqdq          m%8, m%7, m%9
    punpcklqdq          m%7, m%9
%endmacro

; Load diff of 8 entries for 1 row
%macro LOAD_DIFF_Q 4
    movq                %1, %2
    movq                %4, %3
    punpcklbw           %1, %4
    pmaddubsw           %1, hsub
%endmacro

%macro HADAMARD_8_STAGE_1 9
    paddw              m%9, m%1, m%2
    psubw              m%1, m%2
    paddw              m%2, m%3, m%4
    psubw              m%3, m%4
    paddw              m%4, m%5, m%6
    psubw              m%5, m%6
    paddw              m%6, m%7, m%8
    psubw              m%7, m%8
    ; 8->9, 7->8, 6->7, 5->6, 4->5, 3->4, 2->3, 1->2, 9->1
    SWAP                %8, %7, %6, %5, %4, %3, %2, %1, %9
%endmacro

%macro HADAMARD_8_STAGE_2 9
    paddw              m%9, m%1, m%3 ; 0
    psubw              m%1, m%3      ; 2
    paddw              m%3, m%2, m%4 ; 1
    psubw              m%2, m%4      ; 3
    SWAP                %3, %2, %1
    paddw              m%4, m%5, m%7 ; 4
    psubw              m%5, m%7      ; 6
    paddw              m%7, m%6, m%8 ; 5
    psubw              m%6, m%8      ; 7
    SWAP                %7, %6, %5
    ; 8->9, 7->8, 6->7, 5->6, 4->5, 3->4, 2->3, 1->2, 9->1
    SWAP                %8, %7, %6, %5, %4, %3, %2, %1, %9
%endmacro

%macro HADAMARD_8_STAGE_3 9
    paddw              m%9, m%1, m%5 ; 0
    psubw              m%1, m%5      ; 4
    paddw              m%5, m%2, m%6 ; 1
    psubw              m%2, m%6      ; 5
    paddw              m%6, m%3, m%7 ; 2
    psubw              m%3, m%7      ; 6
    paddw              m%7, m%4, m%8 ; 3
    psubw              m%4, m%8      ; 7
    SWAP                %5, %2, %6, %3, %7, %4, %1
    ; 8->9, 7->8, 6->7, 5->6, 4->5, 3->4, 2->3, 1->2, 9->1
    SWAP                %8, %7, %6, %5, %4, %3, %2, %1, %9
%endmacro

; Rudimentary fast hadamard transform
%macro HADAMARD_8x8 0
    HADAMARD_8_STAGE_1 1, 2, 3, 4, 5, 6, 7, 8, 0
    HADAMARD_8_STAGE_2 1, 2, 3, 4, 5, 6, 7, 8, 0
    HADAMARD_8_STAGE_3 1, 2, 3, 4, 5, 6, 7, 8, 0

    TRANSPOSE8x8 1, 2, 3, 4, 5, 6, 7, 8, 0

    HADAMARD_8_STAGE_1 1, 2, 3, 4, 5, 6, 7, 8, 0
    HADAMARD_8_STAGE_2 1, 2, 3, 4, 5, 6, 7, 8, 0

    ; Stage 3
    ; Use the fact that
    ;   (abs(a+b)+abs(a-b))/2 = max(abs(a),abs(b))
    ;  to merge the final butterfly with the abs and the first stage of
    ;  accumulation.
    ; Avoid pabsw by using max(a, b) + max(a + b + 0x7FFF, 0x7FFF) instead.
    ; Actually calculates (abs(a+b)+abs(a-b))/2-0x7FFF.
    ; The final sum must be offset to compensate for subtracting 0x7FFF.
    paddw               m0, m1, m5
    pmaxsw              m1, m5
    ; m1 is free
    ; 0x7FFF
    pcmpeqb             m5, m5
    psrlw               m5, 1

    paddsw              m0, m5
    psubw               m1, m0

    paddw               m0, m2, m6
    pmaxsw              m2, m6
    paddsw              m0, m5
    psubw               m2, m0

    paddw               m0, m3, m7
    pmaxsw              m3, m7
    paddsw              m0, m5
    psubw               m3, m0

    paddw               m0, m4, m8
    pmaxsw              m4, m8
    paddsw              m0, m5
    psubw               m4, m0

    paddw               m1, m2
    paddw               m3, m4

    paddw               m1, m3
    SWAP                 1, 0
%endmacro

; Only works with 128 bit vectors
%macro SATD_8x8_FN 0
cglobal satd_8x8, 4, 6, 10, src, src_stride, dst, dst_stride, \
                           src_stride3, dst_stride3
    %define           hsub  m0
    mova              hsub, [maddubsw_hsub]
    ; Load rows into m1-m8
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    LOAD_DIFF_Q m1, [srcq], [dstq], m2
    LOAD_DIFF_Q m2, [srcq+src_strideq*1], [dstq+dst_strideq*1], m3
    LOAD_DIFF_Q m3, [srcq+src_strideq*2], [dstq+dst_strideq*2], m4
    LOAD_DIFF_Q m4, [srcq+src_stride3q], [dstq+dst_stride3q], m5
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]
    LOAD_DIFF_Q m5, [srcq], [dstq], m6
    LOAD_DIFF_Q m6, [srcq+src_strideq*1], [dstq+dst_strideq*1], m7
    LOAD_DIFF_Q m7, [srcq+src_strideq*2], [dstq+dst_strideq*2], m8
    LOAD_DIFF_Q m8, [srcq+src_stride3q], [dstq+dst_stride3q], m9

    HADAMARD_8x8

    ; Reduce horizontally and convert to 32 bits
    pxor                m2, m2
    punpcklwd           m1, m0, m2
    punpckhwd           m0, m2
    paddd               m0, m1

    pshufd              m1, m0, q3232
    paddd               m0, m1
    pshuflw             m1, m0, q3232
    paddd               m0, m1
    movd               eax, m0

    ; Normalize
    ; Add rounding offset and an offset for how the final butterfly stage and
    ;  the first stage of accumulation was done.
    sub                eax, 32-2
    shr                eax, 2
    RET
%endmacro

INIT_XMM ssse3
SATD_8x8_FN

INIT_XMM avx2
SATD_8x8_FN

INIT_YMM avx2
cglobal satd_16x8, 4, 6, 9, src, src_stride, dst, dst_stride, \
                            src_stride3, dst_stride3
    ; Load rows into m1-m8
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    LOAD_DIFF_DQ 1, [srcq], [dstq], 0
    LOAD_DIFF_DQ 2, [srcq+src_strideq*1], [dstq+dst_strideq*1], 0
    LOAD_DIFF_DQ 3, [srcq+src_strideq*2], [dstq+dst_strideq*2], 0
    LOAD_DIFF_DQ 4, [srcq+src_stride3q], [dstq+dst_stride3q], 0
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]
    LOAD_DIFF_DQ 5, [srcq], [dstq], 0
    LOAD_DIFF_DQ 6, [srcq+src_strideq*1], [dstq+dst_strideq*1], 0
    LOAD_DIFF_DQ 7, [srcq+src_strideq*2], [dstq+dst_strideq*2], 0
    LOAD_DIFF_DQ 8, [srcq+src_stride3q], [dstq+dst_stride3q], 0

    HADAMARD_8x8

    ; Reduce horizontally and convert to 32 bits
    pxor                m2, m2
    punpcklwd           m1, m0, m2
    punpckhwd           m0, m2
    paddd               m0, m1

    vextracti128       xm1, m0, 1
    paddd              xm0, xm1
    pshufd             xm1, xm0, q3232
    paddd              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddd              xm0, xm1
    movd               eax, xm0

    ; Normalize
    ; Add rounding offset and an offset for how the final butterfly stage and
    ;  the first stage of accumulation was done.
    sub                eax, 64-2
    shr                eax, 2
    RET

%macro LOAD_DIFF_Qx2 7
    movq              xm%1, %2
    movq              xm%6, %3
    punpcklbw         xm%1, xm%6
    movq              xm%6, %4
    movq              xm%7, %5
    punpcklbw         xm%6, xm%7
    vinserti128        m%1, xm%6, 1
    pmaddubsw          m%1, hsub
%endmacro

INIT_YMM avx2
cglobal satd_8x16, 4, 8, 11, src, src_stride, dst, dst_stride, \
                             src8, dst8, src_stride3, dst_stride3
    %define           hsub  m0
    mova              hsub, [maddubsw_hsub]
    ; Load rows into m1-m8
    lea              src8q, [srcq+src_strideq*8]
    lea              dst8q, [dstq+dst_strideq*8]
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    LOAD_DIFF_Qx2 1, [srcq], [dstq], \
                     [src8q], [dst8q], \
                     9, 10
    LOAD_DIFF_Qx2 2, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                     [src8q+src_strideq*1], [dst8q+dst_strideq*1], \
                     9, 10
    LOAD_DIFF_Qx2 3, [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                     [src8q+src_strideq*2], [dst8q+dst_strideq*2], \
                     9, 10
    LOAD_DIFF_Qx2 4, [srcq+src_stride3q], [dstq+dst_stride3q], \
                     [src8q+src_stride3q], [dst8q+dst_stride3q], \
                     9, 10
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]
    lea              src8q, [src8q+src_strideq*4]
    lea              dst8q, [dst8q+dst_strideq*4]
    LOAD_DIFF_Qx2 5, [srcq], [dstq], \
                     [src8q], [dst8q], \
                     9, 10
    LOAD_DIFF_Qx2 6, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                     [src8q+src_strideq*1], [dst8q+dst_strideq*1], \
                     9, 10
    LOAD_DIFF_Qx2 7, [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                     [src8q+src_strideq*2], [dst8q+dst_strideq*2], \
                     9, 10
    LOAD_DIFF_Qx2 8, [srcq+src_stride3q], [dstq+dst_stride3q], \
                     [src8q+src_stride3q], [dst8q+dst_stride3q], \
                     9, 10

    HADAMARD_8x8

    ; Reduce horizontally and convert to 32 bits
    pxor                m2, m2
    punpcklwd           m1, m0, m2
    punpckhwd           m0, m2
    paddd               m0, m1

    vextracti128       xm1, m0, 1
    paddd              xm0, xm1
    pshufd             xm1, xm0, q3232
    paddd              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddd              xm0, xm1
    movd               eax, xm0

    ; Normalize
    ; Add rounding offset and an offset for how the final butterfly stage and
    ;  the first stage of accumulation was done.
    sub                eax, 64-2
    shr                eax, 2
    RET

; Less optimized, boilerplate implementations

INIT_YMM avx2
cglobal satd_8x32, 4, 9, 13, src, src_stride, dst, dst_stride, \
                             src8, dst8, src_stride3, dst_stride3, cnt
    ; ones for converting to 32-bit with pmaddwd
    pcmpeqw            m11, m11
    pabsw              m11, m11
    ; sum
    pxor               m12, m12
    mov               cntd, 1
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    lea              src8q, [srcq+src_strideq*8]
    lea              dst8q, [dstq+dst_strideq*8]
.loop:
    %define           hsub  m0
    mova              hsub, [maddubsw_hsub]
    ; Load rows into m1-m8
    LOAD_DIFF_Qx2 1, [srcq], [dstq], \
                     [src8q], [dst8q], \
                  9, 10
    LOAD_DIFF_Qx2 2, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                     [src8q+src_strideq*1], [dst8q+dst_strideq*1], \
                  9, 10
    LOAD_DIFF_Qx2 3, [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                     [src8q+src_strideq*2], [dst8q+dst_strideq*2], \
                  9, 10
    LOAD_DIFF_Qx2 4, [srcq+src_stride3q], [dstq+dst_stride3q], \
                     [src8q+src_stride3q], [dst8q+dst_stride3q], \
                  9, 10
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]
    lea              src8q, [src8q+src_strideq*4]
    lea              dst8q, [dst8q+dst_strideq*4]
    LOAD_DIFF_Qx2 5, [srcq], [dstq], \
                     [src8q], [dst8q], \
                  9, 10
    LOAD_DIFF_Qx2 6, [srcq+src_strideq*1], [dstq+dst_strideq*1], \
                     [src8q+src_strideq*1], [dst8q+dst_strideq*1], \
                  9, 10
    LOAD_DIFF_Qx2 7, [srcq+src_strideq*2], [dstq+dst_strideq*2], \
                     [src8q+src_strideq*2], [dst8q+dst_strideq*2], \
                  9, 10
    LOAD_DIFF_Qx2 8, [srcq+src_stride3q], [dstq+dst_stride3q], \
                     [src8q+src_stride3q], [dst8q+dst_stride3q], \
                  9, 10

    HADAMARD_8x8

    ; Reduce horizontally and convert to 32 bits
    pmaddwd             m0, m11
    paddd              m12, m0

    lea               srcq, [srcq+src_stride3q*4]
    lea               dstq, [dstq+dst_stride3q*4]
    lea              src8q, [src8q+src_stride3q*4]
    lea              dst8q, [dst8q+dst_stride3q*4]
    dec               cntd
    jge .loop

    vextracti128       xm0, m12, 1
    paddd              xm0, xm12
    pshufd             xm1, xm0, q3232
    paddd              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddd              xm0, xm1
    movd               eax, xm0

    ; Normalize
    ; Add rounding offset and an offset for how the final butterfly stage and
    ;  the first stage of accumulation was done.
    sub                eax, 128-2
    shr                eax, 2
    RET

INIT_YMM avx2
cglobal satd_16x8_internal, 0, 0, 0, \
                            dummy1, src_stride, dummy2, dst_stride, \
                            src_stride3, dst_stride3, src, dst
    %define hadd m9
    %define sum m10
    ; Load rows into m1-m8
    LOAD_DIFF_DQ 1, [srcq], [dstq], 0
    LOAD_DIFF_DQ 2, [srcq+src_strideq*1], [dstq+dst_strideq*1], 0
    LOAD_DIFF_DQ 3, [srcq+src_strideq*2], [dstq+dst_strideq*2], 0
    LOAD_DIFF_DQ 4, [srcq+src_stride3q], [dstq+dst_stride3q], 0
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]
    LOAD_DIFF_DQ 5, [srcq], [dstq], 0
    LOAD_DIFF_DQ 6, [srcq+src_strideq*1], [dstq+dst_strideq*1], 0
    LOAD_DIFF_DQ 7, [srcq+src_strideq*2], [dstq+dst_strideq*2], 0
    LOAD_DIFF_DQ 8, [srcq+src_stride3q], [dstq+dst_stride3q], 0

    HADAMARD_8x8

    pmaddwd             m0, hadd
    paddd              sum, m0
    ret

%macro SATD_NXM 2
%if %1 > 16
%if %2 > 8
cglobal satd_%1x%2, 4, 10, 11, src, src_stride, dst, dst_stride, \
                              src_stride3, dst_stride3, call_src, call_dst, \
                              w, h
%else
cglobal satd_%1x%2, 4, 9, 11, src, src_stride, dst, dst_stride, \
                              src_stride3, dst_stride3, call_src, call_dst, \
                              w
%endif
%else ; %2 > 8
cglobal satd_%1x%2, 4, 9, 11, src, src_stride, dst, dst_stride, \
                              src_stride3, dst_stride3, call_src, call_dst, \
                              h
%endif
    ; ones for converting to 32-bit with pmaddwd
    pcmpeqw             m9, m9
    pabsw               m9, m9
    ; sum
    pxor               m10, m10
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
%if %2 > 8
    mov                 hd, %2/8 - 1
.looph:
%endif
%if %1 > 16
    mov                 wd, %1/16 - 1
.loopv:
%endif
    mov          call_srcq, srcq
    mov          call_dstq, dstq
    call m(satd_16x8_internal)
%if %1 > 16
    add               srcq, 16
    add               dstq, 16
    dec                 wd
    jge .loopv
    sub               srcq, %1
    sub               dstq, %1
%endif
%if %2 > 8
    lea               srcq, [srcq+src_strideq*8]
    lea               dstq, [dstq+dst_strideq*8]
    dec                 hd
    jge .looph
%endif

    ; Reduce horizontally
    vextracti128       xm0, m10, 1
    paddd              xm0, xm10
    pshufd             xm1, xm0, q3232
    paddd              xm0, xm1
    pshuflw            xm1, xm0, q3232
    paddd              xm0, xm1
    movd               eax, xm0

    ; Normalize
    ; Add rounding offset and an offset for how the final butterfly stage and
    ;  the first stage of accumulation was done.
    sub                eax, %1*%2/2 - 2
    shr                eax, 2
    RET
%endmacro

INIT_YMM avx2
SATD_NXM 16, 16
SATD_NXM 32, 32
SATD_NXM 64, 64
SATD_NXM 128, 128

SATD_NXM 16, 32
SATD_NXM 32, 16
SATD_NXM 32, 64
SATD_NXM 64, 32
SATD_NXM 64, 128
SATD_NXM 128, 64

SATD_NXM 32, 8
SATD_NXM 16, 64
SATD_NXM 64, 16

%endif ; ARCH_X86_64
