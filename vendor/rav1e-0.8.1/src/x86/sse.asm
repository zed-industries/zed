; Copyright (c) 2020-2022, The rav1e contributors. All rights reserved
;
; This source code is subject to the terms of the BSD 2 Clause License and
; the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
; was not distributed with this source code in the LICENSE file, you can
; obtain it at www.aomedia.org/license/software. If the Alliance for Open
; Media Patent License 1.0 was not distributed with this source code in the
; PATENTS file, you can obtain it at www.aomedia.org/license/patent.

%include "config.asm"
%include "ext/x86/x86inc.asm"

; Must match crate::dist::rust::GET_WEIGHTED_SSE_SHIFT
%define get_weighted_sse_shift 8
%define get_weighted_sse_round (1 << get_weighted_sse_shift >> 1)

SECTION_RODATA 32
addsub: times 16 db 1, -1
rounding: times 4 dq get_weighted_sse_round

SECTION .text

%define m(x) mangle(private_prefix %+ _ %+ x %+ SUFFIX)

; Consolidate scaling and rounding to one place so that it is easier to change.

%macro SSE_SCALE_4X4 0
    ; Multiply and shift using scalar code
    mov             scaled, [scaleq]
    imul               rax, scaleq
    add                rax, get_weighted_sse_round
    shr                rax, get_weighted_sse_shift
%endmacro

; 1 is the input and output register.
; 2-3 are tmp registers.
%macro SSE_SCALE 2-3
    ; Reduce 32-bit sums to 64-bits sums.
    pshufd             m%2, m%1, q3311
    paddd              m%1, m%2

    LOAD_SCALES %2, %3

    ; Multiply and shift with rounding.
    pmuludq            m%1, m%2
    mova               m%2, [rounding]
    paddq              m%1, m%2
    psrlq              m%1, get_weighted_sse_shift
%endmacro

%macro LOAD_SCALES_4X8 2
    ; Load 1 scale from each of the 2 rows.
    movd               m%1, [scaleq]
    movd               m%2, [scaleq+scale_strideq]
    ; 64-bit unpack since our loads have only one value each.
    punpcklqdq         m%1, m%2
%endmacro

; 2 is unused
%macro LOAD_SCALES_8X4 2
    ; Convert to 64-bits.
    ; It doesn't matter that the upper halves are full of garbage.
    movq               m%1, [scaleq]
    pshufd             m%1, m%1, q1100
%endmacro

; 2 is unused
%macro LOAD_SCALES_16X4 2
    pmovzxdq           m%1, [scaleq]
%endmacro

; Separate from other scale macros, since it uses 2 inputs.
; 1-2 are inputs regs and 1 is the output reg.
; 3-4 are tmp registers
%macro SSE_SCALE_32X4 4
    pshufd             m%3, m%1, q3311
    paddd              m%1, m%3
    pshufd             m%3, m%2, q3311
    paddd              m%2, m%3

    ; Load scale for 4x4 blocks and convert to 64-bits.
    ; It doesn't matter if the upper halves are full of garbage.
    ; raw load:    0, 1, 2, 3 | 4, 5, 6, 7
    ; unpack low:  0,    1    | 4,    5
    ; unpack high: 2,    3,   | 6,    7
    mova               m%4, [scaleq]
    punpckldq          m%3, m%4, m%4
    punpckhdq          m%4, m%4

    pmuludq            m%1, m%3
    pmuludq            m%2, m%4
    mova               m%3, [rounding]
    paddq              m%1, m%3
    paddq              m%2, m%3
    psrlq              m%1, get_weighted_sse_shift
    psrlq              m%2, get_weighted_sse_shift
    paddq              m%1, m%2
%endmacro

INIT_XMM ssse3
; Use scale_stride's register to store src_stride3
cglobal weighted_sse_4x4, 6, 7, 5, \
        src, src_stride, dst, dst_stride, scale, \
        src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    movq                m0, [addsub]
    movd                m1, [srcq]
    movd                m2, [dstq]
    punpcklbw           m1, m2
    movd                m2, [srcq+src_strideq]
    movd                m3, [dstq+dst_strideq]
    punpcklbw           m2, m3
    pmaddubsw           m1, m0
    pmaddubsw           m2, m0
    pmaddwd             m1, m1
    pmaddwd             m2, m2
    paddd               m1, m2
    movd                m2, [srcq+src_strideq*2]
    movd                m3, [dstq+dst_strideq*2]
    punpcklbw           m2, m3
    movd                m3, [srcq+src_stride3q]
    movd                m4, [dstq+dst_stride3q]
    punpcklbw           m3, m4
    pmaddubsw           m2, m0
    pmaddubsw           m3, m0
    pmaddwd             m2, m2
    pmaddwd             m3, m3
    paddd               m2, m3
    paddd               m1, m2

    pshuflw             m0, m1, q3232
    paddd               m0, m1
    movd               eax, m0

    ; Multiply and shift using scalar code.
    SSE_SCALE_4X4
    RET

%macro WEIGHTED_SSE_4X8_KERNEL 0
    movd                m1, [srcq]
    movd                m2, [srcq+src_strideq*4]
    punpckldq           m1, m2
    movd                m2, [dstq]
    movd                m3, [dstq+dst_strideq*4]
    add               srcq, src_strideq
    add               dstq, dst_strideq
    punpckldq           m2, m3
    punpcklbw           m1, m2
    movd                m2, [srcq]
    movd                m3, [srcq+src_strideq*4]
    punpckldq           m2, m3
    movd                m3, [dstq]
    movd                m4, [dstq+dst_strideq*4]
    add               srcq, src_strideq
    add               dstq, dst_strideq
    punpckldq           m3, m4
    punpcklbw           m2, m3
    pmaddubsw           m1, m0
    pmaddubsw           m2, m0
    pmaddwd             m1, m1
    pmaddwd             m2, m2
    paddd               m1, m2
    movd                m2, [srcq]
    movd                m3, [srcq+src_strideq*4]
    punpckldq           m2, m3
    movd                m3, [dstq]
    movd                m4, [dstq+dst_strideq*4]
    add               srcq, src_strideq
    add               dstq, dst_strideq
    punpckldq           m3, m4
    punpcklbw           m2, m3
    movd                m3, [srcq]
    movd                m4, [srcq+src_strideq*4]
    punpckldq           m3, m4
    movd                m4, [dstq]
    movd                m5, [dstq+dst_strideq*4]
    punpckldq           m4, m5
    punpcklbw           m3, m4
    pmaddubsw           m2, m0
    pmaddubsw           m3, m0
    pmaddwd             m2, m2
    pmaddwd             m3, m3
    paddd               m2, m3
    paddd               m1, m2

    %define LOAD_SCALES LOAD_SCALES_4X8
    SSE_SCALE 1, 2, 3
%endmacro

INIT_XMM ssse3
cglobal weighted_sse_4x8, 6, 6, 6, \
        src, src_stride, dst, dst_stride, scale, scale_stride
    mova                m0, [addsub]
    WEIGHTED_SSE_4X8_KERNEL

    pshufd              m0, m1, q3232
    paddq               m1, m0
    movq               rax, m1
    RET

INIT_XMM ssse3
cglobal weighted_sse_4x16, 6, 6, 7, \
        src, src_stride, dst, dst_stride, scale, scale_stride
    mova                m0, [addsub]

    WEIGHTED_SSE_4X8_KERNEL
    ; Swap so the use of this macro will use m6 as the result
    SWAP 1, 6

    lea             scaleq, [scaleq+scale_strideq*2]
    ; Already incremented by stride 3 times, but must go up 5 more to get to 8
    add               srcq, src_strideq
    add               dstq, dst_strideq
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]
    WEIGHTED_SSE_4X8_KERNEL
    paddq               m6, m1

    pshufd              m0, m6, q3232
    paddq               m6, m0
    movq               rax, m6
    RET

%macro WEIGHTED_SSE_8X4_KERNEL 0
    movq                m1, [srcq]
    movq                m2, [dstq]
    punpcklbw           m1, m2
    movq                m2, [srcq+src_strideq]
    movq                m3, [dstq+dst_strideq]
    punpcklbw           m2, m3
    pmaddubsw           m1, m0
    pmaddubsw           m2, m0
    pmaddwd             m1, m1
    pmaddwd             m2, m2
    paddd               m1, m2
    movq                m2, [srcq+src_strideq*2]
    movq                m3, [dstq+dst_strideq*2]
    punpcklbw           m2, m3
    movq                m3, [srcq+src_stride3q]
    movq                m4, [dstq+dst_stride3q]
    punpcklbw           m3, m4
    pmaddubsw           m2, m0
    pmaddubsw           m3, m0
    pmaddwd             m2, m2
    pmaddwd             m3, m3
    paddd               m2, m3
    paddd               m1, m2

    %define LOAD_SCALES LOAD_SCALES_8X4
    SSE_SCALE 1, 2
%endmacro

%macro WEIGHTED_SSE_16X4_KERNEL 0
    pmovzxbw            m0, [srcq]
    pmovzxbw            m1, [dstq]
    psubw               m0, m1
    pmaddwd             m0, m0
    pmovzxbw            m1, [srcq+src_strideq]
    pmovzxbw            m2, [dstq+dst_strideq]
    psubw               m1, m2
    pmaddwd             m1, m1
    paddd               m0, m1
    pmovzxbw            m1, [srcq+src_strideq*2]
    pmovzxbw            m2, [dstq+dst_strideq*2]
    psubw               m1, m2
    pmaddwd             m1, m1
    pmovzxbw            m2, [srcq+src_stride3q]
    pmovzxbw            m3, [dstq+dst_stride3q]
    psubw               m2, m3
    pmaddwd             m2, m2
    paddd               m1, m2
    paddd               m1, m0

    %define LOAD_SCALES LOAD_SCALES_16X4
    SSE_SCALE 1, 2
%endmacro

%macro WEIGHTED_SSE_32X4_KERNEL 0
    ; Unpacking high and low results in sums that are 8 samples apart. To
    ; correctly apply weights, two separate registers are needed to accumulate.
    mova                m2, [srcq]
    mova                m3, [dstq]
    punpcklbw           m1, m2, m3
    punpckhbw           m2, m3
    mova                m4, [srcq+src_strideq]
    mova                m5, [dstq+dst_strideq]
    punpcklbw           m3, m4, m5
    punpckhbw           m4, m5
    pmaddubsw           m1, m0
    pmaddubsw           m2, m0
    pmaddubsw           m3, m0
    pmaddubsw           m4, m0
    pmaddwd             m1, m1
    pmaddwd             m2, m2
    pmaddwd             m3, m3
    pmaddwd             m4, m4
    ; Accumulate
    paddd               m1, m3
    paddd               m2, m4
    mova                m4, [srcq+src_strideq*2]
    mova                m5, [dstq+dst_strideq*2]
    punpcklbw           m3, m4, m5
    punpckhbw           m4, m5
    mova                m6, [srcq+src_stride3q]
    mova                m7, [dstq+dst_stride3q]
    punpcklbw           m5, m6, m7
    punpckhbw           m6, m7
    pmaddubsw           m3, m0
    pmaddubsw           m4, m0
    pmaddubsw           m5, m0
    pmaddubsw           m6, m0
    pmaddwd             m3, m3
    pmaddwd             m4, m4
    pmaddwd             m5, m5
    pmaddwd             m6, m6
    paddd               m3, m5
    paddd               m4, m6
    paddd               m1, m3
    paddd               m2, m4

    SSE_SCALE_32X4 1, 2, 3, 4
%endmacro

%macro WEIGHTED_SSE 2 ; w, h
%if %1 == 8
%if %2 == 4
; Use scale_stride's register to store src_stride3
cglobal weighted_sse_%1x%2, 6, 7, 5, \
        src, src_stride, dst, dst_stride, scale, \
        src_stride3, dst_stride3
%else
cglobal weighted_sse_%1x%2, 6, 9, 6, \
        src, src_stride, dst, dst_stride, scale, scale_stride, \
        src_stride3, dst_stride3, h
%endif
%elif %1 == 16
%if %2 == 4
; Use scale_stride's register to store src_stride3
cglobal weighted_sse_%1x%2, 6, 7, 4, \
        src, src_stride, dst, dst_stride, scale, \
        src_stride3, dst_stride3
%else
cglobal weighted_sse_%1x%2, 6, 9, 5, \
        src, src_stride, dst, dst_stride, scale, scale_stride, \
        src_stride3, dst_stride3, h
%endif
%elif %1 == 32
cglobal weighted_sse_%1x%2, 6, 9, 9, \
        src, src_stride, dst, dst_stride, scale, scale_stride, \
        src_stride3, dst_stride3, h
%else ; > 32
cglobal weighted_sse_%1x%2, 6, 10, 9, \
        src, src_stride, dst, dst_stride, scale, scale_stride, \
        src_stride3, dst_stride3, h, w
%endif
; === Setup ===
; kernel_width/kernel_height: number of elements that the kernel processes.
; m0: except for when w == 16, m0 is used to hold a constant 1, -1... vector
;     register for diffing the two sources.
; sum: The kernel stores it's results on m1. The last vector register is used
;      unless only one iteration is done.

; Default the kernel width to the width of this function.
%define kernel_width %1
%define kernel_height 4
%if %1 == 8
    mova                m0, [addsub]
%endif

%if %1 >= 32
    mova                m0, [addsub]
    ; Iterate multiple times when w > 32.
    %define kernel_width 32
%endif

%if %1 > kernel_width || %2 > kernel_height
    ; Add onto the last used vector register.
    %assign sum xmm_regs_used-1
%else
    ; Use the result from the kernel
    %define sum 1
%endif

    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
%if %1 > kernel_width || %2 > kernel_height
    pxor           m%[sum], m%[sum]
%endif
%if %2 > kernel_height
    mov                 hd, %2/kernel_height-1
.loop:
%endif

%if %1 > kernel_width
    mov                 wd, %1/kernel_width-1
.loop_horiz:
%endif

    WEIGHTED_SSE_%[kernel_width]X%[kernel_height]_KERNEL
%if %2 > kernel_height || %1 > kernel_width
    paddq          m%[sum], m1
%endif

%if %1 > kernel_width
    add             scaleq, kernel_width*4/4
    add               srcq, kernel_width
    add               dstq, kernel_width
    dec                 wq
    jge .loop_horiz
%endif

%if %2 > kernel_height
    ; Move down 4 rows.
%if %1 > kernel_width
    ; src/dst is incremented by width when processing multi iteration rows.
    ; Reduce the offset by the width of the row.
    lea               srcq, [srcq+src_strideq*4 - %1]
    lea               dstq, [dstq+dst_strideq*4 - %1]
    ; The behaviour for scale is similar
    lea             scaleq, [scaleq+scale_strideq - %1*4/4]
%else
    lea               srcq, [srcq+src_strideq*4]
    lea               dstq, [dstq+dst_strideq*4]
    add             scaleq, scale_strideq
%endif
    dec                 hq
    jge .loop
%endif

%if mmsize == 16
    pshufd              m2, m%[sum], q3232
    paddq          m%[sum], m2
    movq               rax, m%[sum]
%elif mmsize == 32
    vextracti128       xm2, m%[sum], 1
    paddq         xm%[sum], xm2
    pshufd             xm2, xm%[sum], q3232
    paddq         xm%[sum], xm2
    movq               rax, xm%[sum]
%endif
    RET

    %undef sum
    %undef kernel_width
%endmacro

INIT_XMM ssse3
WEIGHTED_SSE 8, 4
%if ARCH_X86_64
WEIGHTED_SSE 8, 8
WEIGHTED_SSE 8, 16
WEIGHTED_SSE 8, 32
%endif ; ARCH_X86_64

INIT_YMM avx2
WEIGHTED_SSE 16, 4
%if ARCH_X86_64
WEIGHTED_SSE 16, 8
WEIGHTED_SSE 16, 16
WEIGHTED_SSE 16, 32
WEIGHTED_SSE 16, 64

WEIGHTED_SSE 32, 8
WEIGHTED_SSE 32, 16
WEIGHTED_SSE 32, 32
WEIGHTED_SSE 32, 64

WEIGHTED_SSE 64, 16
WEIGHTED_SSE 64, 32
WEIGHTED_SSE 64, 64
WEIGHTED_SSE 64, 128

WEIGHTED_SSE 128, 64
WEIGHTED_SSE 128, 128
%endif ; ARCH_X86_64

INIT_XMM sse2

cglobal weighted_sse_4x4_hbd, 6, 8, 4, \
        src, src_stride, dst, dst_stride, scale, scale_stride, \
        src_stride3, dst_stride3
    lea       src_stride3q, [src_strideq*3]
    lea       dst_stride3q, [dst_strideq*3]
    movq                m0, [srcq]
    movq                m1, [dstq]
    psubw               m0, m1
    pmaddwd             m0, m0
    movq                m1, [srcq+src_strideq]
    movq                m2, [dstq+dst_strideq]
    psubw               m1, m2
    pmaddwd             m1, m1
    paddd               m0, m1
    movq                m1, [srcq+src_strideq*2]
    movq                m2, [dstq+dst_strideq*2]
    psubw               m1, m2
    pmaddwd             m1, m1
    movq                m2, [srcq+src_stride3q]
    movq                m3, [dstq+dst_stride3q]
    psubw               m2, m3
    pmaddwd             m2, m2
    paddd               m1, m2
    paddd               m0, m1

    pshuflw             m1, m0, q3232
    paddd               m0, m1
    movd               eax, m0

    ; Multiply and shift using scalar code.
    SSE_SCALE_4X4
    RET
