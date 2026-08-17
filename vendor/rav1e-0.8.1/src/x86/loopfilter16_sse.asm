; Copyright © 2021, VideoLAN and dav1d authors
; Copyright © 2021, Two Orioles, LLC
; All rights reserved.
;
; Redistribution and use in source and binary forms, with or without
; modification, are permitted provided that the following conditions are met:
;
; 1. Redistributions of source code must retain the above copyright notice, this
;    list of conditions and the following disclaimer.
;
; 2. Redistributions in binary form must reproduce the above copyright notice,
;    this list of conditions and the following disclaimer in the documentation
;    and/or other materials provided with the distribution.
;
; THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
; ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
; WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
; DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
; ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
; (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
; ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
; (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
; SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

%include "config.asm"
%include "ext/x86/x86inc.asm"

SECTION_RODATA 16

%if ARCH_X86_64
%define PIC_sym(a) a
%else
%define PIC_base $$
%define PIC_sym(a) pic_regq+a-PIC_base
%endif

pb_4x1_4x5_4x9_4x13: times 4 db 0, 1
                     times 4 db 8, 9

pw_1: times 8 dw 1
pw_2: times 8 dw 2
pw_3: times 8 dw 3
; 4 and 16 need to be next to each other since they are used as alternates
; depending on whether bitdepth is 10 or 12
pw_4: times 8 dw 4
pw_16: times 8 dw 16
pw_8: times 8 dw 8
pw_4096: times 8 dw 4096

pb_mask: dd 1, 1, 2, 2

SECTION .text

%if ARCH_X86_32
%if STACK_ALIGNMENT < 16
%define extra_stack 2
%else
%define extra_stack 0
%endif
%endif

%macro RELOC_ARGS 2 ; h/v, off
ASSERT ARCH_X86_32
%if STACK_ALIGNMENT < 16
    mov          r5d, [rstk + stack_offset + 4*4 + 4]
%define lstridem [esp+%2+0*gprsize]
    mov     lstridem, r5d
    mov          r5d, [rstk + stack_offset + 4*5 + 4]
%define lutm [esp+%2+1*gprsize]
    mov         lutm, r5d
    mov          r5d, [rstk + stack_offset + 4*6 + 4]
%ifidn %1, v
%define wm [esp+%2+2*gprsize]
    mov           wm, r5d
    mov          r5d, [rstk + stack_offset + 4*3 + 4]
%define lm [esp+%2+3*gprsize]
    mov           lm, r5d
%else ; %1 == h
%define hm [esp+%2+2*gprsize]
    mov           hm, r5d
%endif ; %1==v
   mov           r5d, r7m
%define bdmulm [esp+%2+4*gprsize]
    mov       bdmulm, r5d
%else
%define lstridem r4m
%define lutm r5m
%ifidn %1, v
%define wm r6m
%define lm r3m
%else
%define hm r6m
%endif
%define bdmulm r7m
%endif ; STACK_ALIGNMENT
%endmacro

%macro UNRELOC_ARGS 0
%if ARCH_X86_32
%undef lm
%undef lstridem
%undef wm
%undef hm
%undef lutm
%endif
%endmacro

%macro SPLATD 2
    movd %1, %2
    pshufd %1, %1, q0000
%endmacro

%macro SPLATW 2
    movd %1, %2
    pshuflw %1, %1, q0000
    punpcklqdq %1, %1
%endmacro

;        in:            out:
; mm%1   a b c d        a e i m
; mm%2   e f g h        b f j n
; mm%3   i j k l   ->   c g k o
; mm%4   m n o p        d h l p
%macro TRANSPOSE4X4W 5
    punpcklwd        m%5, m%1, m%2
    punpckhwd        m%1, m%2
    punpcklwd        m%2, m%3, m%4
    punpckhwd        m%3, m%4
    punpckldq        m%4, m%5, m%2
    punpckhdq        m%5, m%2
    punpckldq        m%2, m%1, m%3
    punpckhdq        m%1, m%3

    SWAP              %1, %4
    SWAP              %2, %5, %3
%endmacro

;         in:                  out:
; m%1   a b c d e f g h      a i q y 6 E M U
; m%2   i j k l m n o p      b j r z 7 F N V
; m%3   q r s t u v w x      c k s 0 8 G O W
; m%4   y z 0 1 2 3 4 5      d l t 1 9 H P X
; m%5   6 7 8 9 A B C D  ->  e m u 2 A I Q Y
; m%6   E F G H I J K L      f n v 3 B J R Z
; m%7   M N O P Q R S T      g o w 4 C K S +
; m%8   U V W X Y Z + =      h p x 5 D L T =
%if ARCH_X86_64
%macro TRANSPOSE8X8W 9
    ; m%1   a b c d e f g h      a i q y b j r z
    ; m%2   i j k l m n o p      c k s 0 d l t 1
    ; m%3   q r s t u v w x  ->  e m u 2 f n v 3
    ; m%4   y z 0 1 2 3 4 5      g o w 4 h p x 5
    TRANSPOSE4X4W     %1, %2, %3, %4, %9

    ; m%5   6 7 8 9 A B C D      6 E M U 7 F N V
    ; m%6   E F G H I J K L      8 G O W 9 H P X
    ; m%7   M N O P Q R S T  ->  A I Q Y B J R Z
    ; m%8   U V W X Y Z + =      C K S + D L T =
    TRANSPOSE4X4W     %5, %6, %7, %8, %9

    ; m%1   a i q y b j r z      a i q y 6 E M U
    ; m%2   c k s 0 d l t 1      b j r z 7 F N V
    ; m%3   e m u 2 f n v 3      c k s 0 8 G O W
    ; m%4   g o w 4 h p x 5      d l t 1 9 H P X
    ; m%5   6 E M U 7 F N V  ->  e m u 2 A I Q Y
    ; m%6   8 G O W 9 H P X      f n v 3 B J R Z
    ; m%7   A I Q Y B J R Z      g o w 4 C K S +
    ; m%8   C K S + D L T =      h p x 5 D L T =
    punpckhqdq       m%9, m%1, m%5
    punpcklqdq       m%1, m%5
    punpckhqdq       m%5, m%2, m%6
    punpcklqdq       m%2, m%6
    punpckhqdq       m%6, m%3, m%7
    punpcklqdq       m%3, m%7
    punpckhqdq       m%7, m%4, m%8
    punpcklqdq       m%4, m%8

    SWAP %8, %7, %4, %5, %3, %2, %9
%endmacro
%else ; x86-32
; input: 1-7 in registers, 8 in first memory [read-only]
; second memory is scratch, and may overlap with first or third memory
; output: 1-5,7-8 in registers, 6 in third memory [write-only]
%macro TRANSPOSE8X8W 13 ; regs [8x], mem [3x], a/u [in/out alignment [2x]
    TRANSPOSE4X4W     %1, %2, %3, %4, %8
%ifnidn %9, ""
    mov%12           m%8, %9
%else
    mova             m%8, %10
%endif
    mova             %10, m%4
    TRANSPOSE4X4W     %5, %6, %7, %8, %4
    punpckhqdq       m%4, m%1, m%5
    punpcklqdq       m%1, m%5
    punpckhqdq       m%5, m%2, m%6
    punpcklqdq       m%2, m%6
    punpckhqdq       m%6, m%3, m%7
    punpcklqdq       m%3, m%7
    mova             m%7, %10
%ifnidn %11, ""
    mov%13           %11, m%6
%else
    mova             %10, m%6
%endif
    punpckhqdq       m%6, m%7, m%8
    punpcklqdq       m%7, m%8

    ; 1,4,2,5,3,8,7,6 -> 1,2,3,4,5,6,7,8
    SWAP              %2, %4, %5, %3
    SWAP              %6, %8
%endmacro
%endif ; x86-32/64

; transpose and write m8-11, everything else is scratch
%macro TRANSPOSE_8x4_AND_WRITE_4x8 5 ; p1, p0, q0, q1, tmp
    ; transpose 8x4
    punpcklwd     %5, %1, %2
    punpckhwd     %1, %2
    punpcklwd     %2, %3, %4
    punpckhwd     %3, %4
    punpckldq     %4, %5, %2
    punpckhdq     %5, %2
    punpckldq     %2, %1, %3
    punpckhdq     %1, %3

    ; write out
    movq   [dstq+strideq*0-4], %4
    movhps [dstq+strideq*1-4], %4
    movq   [dstq+strideq*2-4], %5
    movhps [dstq+stride3q -4], %5
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], %2
    movhps [dstq+strideq*1-4], %2
    movq   [dstq+strideq*2-4], %1
    movhps [dstq+stride3q -4], %1
    lea         dstq, [dstq+strideq*4]
%endmacro

%macro FILTER 2 ; width [4/6/8/16], dir [h/v]
    ; load data
%ifidn %2, v
%if %1 == 4
%if ARCH_X86_64
%define P1 m8
%define P0 m9
%define Q0 m10
%define Q1 m11
    mova          P1, [dstq+mstrideq*2]         ; p1
    mova          P0, [dstq+mstrideq*1]         ; p0
    mova          Q0, [dstq+strideq*0]          ; q0
    mova          Q1, [dstq+strideq*1]          ; q1
%else ; x86-32
%define P1 [dstq+mstrideq*2]
%define P0 [dstq+mstrideq*1]
%define Q0 [dstq+strideq*0]
%define Q1 [dstq+strideq*1]
%endif ; x86-32/64
%else ; %1 != 4
    ; load 6-8 pixels, remainder (for wd=16) will be read inline
    lea         tmpq, [dstq+mstrideq*4]
%if ARCH_X86_64
    ; we load p3 later
%define P2 m13
%define P1 m8
%define P0 m9
%define Q0 m10
%define Q1 m11
%define Q2 m14
    mova          P2, [tmpq+strideq*1]
    mova          P1, [tmpq+strideq*2]
    mova          P0, [tmpq+stride3q]
    mova          Q0, [dstq+strideq*0]
    mova          Q1, [dstq+strideq*1]
    mova          Q2, [dstq+strideq*2]
%if %1 != 6
%define P3 [tmpq+strideq*0]
%define Q3 m15
    mova          Q3, [dstq+stride3q]
%endif ; %1 != 6
%else ; x86-32
%define P2 [tmpq+strideq*1]
%define P1 [dstq+mstrideq*2]
%define P0 [dstq+mstrideq*1]
%define Q0 [dstq+strideq*0]
%define Q1 [dstq+strideq*1]
%define Q2 [dstq+strideq*2]
%if %1 != 6
%define P3 [dstq+mstrideq*4]
%define Q3 [dstq+stride3q]
%endif ; %1 != 6
%endif ; x86-32/64
%endif ; %1 ==/!= 4
%else ; %2 != v
    ; load lines
%if %1 == 4
    movq          m0, [dstq+strideq*0-4]
    movq          m2, [dstq+strideq*1-4]
    movq          m4, [dstq+strideq*2-4]
    movq          m5, [dstq+stride3q -4]
    lea         tmpq, [dstq+strideq*4]
    movq          m3, [tmpq+strideq*0-4]
    movq          m6, [tmpq+strideq*1-4]
    movq          m1, [tmpq+strideq*2-4]
    movq          m7, [tmpq+stride3q -4]

    ; transpose 4x8
    ; m0: A-D0
    ; m2: A-D1
    ; m4: A-D2
    ; m5: A-D3
    ; m3: A-D4
    ; m6: A-D5
    ; m1: A-D6
    ; m7: A-D7
    punpcklwd     m0, m2
    punpcklwd     m4, m5
    punpcklwd     m3, m6
    punpcklwd     m1, m7
    ; m0: A0-1,B0-1,C0-1,D0-1
    ; m4: A2-3,B2-3,C2-3,D2-3
    ; m3: A4-5,B4-5,C4-5,D4-5
    ; m1: A6-7,B6-7,C6-7,D6-7
    punpckhdq     m2, m0, m4
    punpckldq     m0, m4
    punpckhdq     m4, m3, m1
    punpckldq     m3, m1
    ; m0: A0-3,B0-3
    ; m2: C0-3,D0-3
    ; m3: A4-7,B4-7
    ; m4: C4-7,D4-7
    punpckhqdq    m1, m0, m3
    punpcklqdq    m0, m3
    punpckhqdq    m3, m2, m4
    punpcklqdq    m2, m4
    ; m0: A0-7
    ; m1: B0-7
    ; m2: C0-7
    ; m3: D0-7
%if ARCH_X86_64
    SWAP           0, 8
    SWAP           1, 9
    SWAP           2, 10
    SWAP           3, 11
%define P1 m8
%define P0 m9
%define Q0 m10
%define Q1 m11
%else
%define P1 [esp+3*mmsize]
%define P0 [esp+4*mmsize]
%define Q0 [esp+5*mmsize]
%define Q1 [esp+6*mmsize]
    mova          P1, m0
    mova          P0, m1
    mova          Q0, m2
    mova          Q1, m3
%endif
%elif %1 == 6 || %1 == 8
    movu          m0, [dstq+strideq*0-8]
    movu          m1, [dstq+strideq*1-8]
    movu          m2, [dstq+strideq*2-8]
    movu          m3, [dstq+stride3q -8]
    lea         tmpq, [dstq+strideq*4]
    movu          m4, [tmpq+strideq*0-8]
    movu          m5, [tmpq+strideq*1-8]
    movu          m6, [tmpq+strideq*2-8]
%if ARCH_X86_64
    movu          m7, [tmpq+stride3q -8]
%endif

    ; transpose 8x16
    ; m0: A-H0,A-H8
    ; m1: A-H1,A-H9
    ; m2: A-H2,A-H10
    ; m3: A-H3,A-H11
    ; m4: A-H4,A-H12
    ; m5: A-H5,A-H13
    ; m6: A-H6,A-H14
    ; m7: A-H7,A-H15
%if ARCH_X86_64
    punpcklwd     m8, m0, m1
%else
    punpcklwd     m7, m0, m1
%endif
    punpckhwd     m0, m1
    punpcklwd     m1, m2, m3
    punpckhwd     m2, m3
    punpcklwd     m3, m4, m5
    punpckhwd     m4, m5
%if ARCH_X86_64
    punpcklwd     m5, m6, m7
    punpckhwd     m6, m7
%else
    mova  [rsp+3*16], m4
    movu          m4, [tmpq+stride3q -8]
    punpcklwd     m5, m6, m4
    punpckhwd     m6, m4
%endif
    ; m8: A0-1,B0-1,C0-1,D0-1 [m7 on x86-32]
    ; m0: E0-1,F0-1,G0-1,H0-1
    ; m1: A2-3,B2-3,C2-3,D2-3
    ; m2: E2-3,F2-3,G2-3,H2-3
    ; m3: A4-5,B4-5,C4-5,D4-5
    ; m4: E4-5,F4-5,G4-5,H4-5 [r3 on x86-32]
    ; m5: A6-7,B6-7,C6-7,D6-7
    ; m6: E6-7,F6-7,G6-7,H6-7
%if ARCH_X86_64
    punpckldq     m7, m8, m1
    punpckhdq     m8, m1
%else
    punpckldq     m4, m7, m1
    punpckhdq     m7, m1
%endif
    punpckldq     m1, m0, m2
    punpckhdq     m0, m2
    punpckldq     m2, m3, m5
    punpckhdq     m3, m5
%if ARCH_X86_64
    punpckldq     m5, m4, m6
    punpckhdq     m4, m6
%else
    mova  [rsp+4*16], m3
    mova          m3, [rsp+3*16]
    punpckldq     m5, m3, m6
    punpckhdq     m3, m6
%endif
    ; m7: A0-3,B0-3 [m4 on x86-32]
    ; m8: C0-3,D0-3 [m7 on x86-32]
    ; m1: E0-3,F0-3
    ; m0: G0-3,H0-3
    ; m2: A4-7,B4-7
    ; m3: C4-7,D4-7 [r4 on x86-32]
    ; m5: E4-7,F4-7
    ; m4: G4-7,H4-7 [m3 on x86-32]
%if ARCH_X86_64
%if %1 != 6
    punpcklqdq    m6, m7, m2
%endif
    punpckhqdq    m7, m2
    punpcklqdq    m2, m8, m3
    punpckhqdq    m8, m3
    punpcklqdq    m3, m1, m5
    punpckhqdq    m1, m5
%if %1 != 6
    punpckhqdq    m5, m0, m4
%endif
    punpcklqdq    m0, m4
%if %1 == 8
    mova  [rsp+1*16], m6
%define P3 [rsp+1*16]
%endif
    ; 7,2,8,3,1,0,5 -> 13,8,9,10,11,14,15
    SWAP           7, 13
    SWAP           8, 2, 9
    SWAP           3, 10
    SWAP           1, 11
    SWAP           0, 14
    SWAP           5, 15
%define P2 m13
%define P1 m8
%define P0 m9
%define Q0 m10
%define Q1 m11
%define Q2 m14
%if %1 == 8
%define Q3 m15
%endif
%else ; x86-32
%if %1 == 8
%define P3 [rsp+ 6*16]
    punpcklqdq    m6, m4, m2
    mova          P3, m6
%endif
    mova          m6, [rsp+4*16]
    punpckhqdq    m4, m2
    punpcklqdq    m2, m7, m6
    punpckhqdq    m7, m6
    punpcklqdq    m6, m1, m5
    punpckhqdq    m1, m5
%if %1 == 8
%define Q3 [rsp+24*16]
    punpckhqdq    m5, m0, m3
    mova          Q3, m5
%endif
    punpcklqdq    m0, m3
%if %1 == 8
%define P2 [rsp+18*16]
%define P1 [rsp+19*16]
%define P0 [rsp+20*16]
%define Q0 [rsp+21*16]
%define Q1 [rsp+22*16]
%define Q2 [rsp+23*16]
%else
%define P2 [rsp+3*16]
%define P1 [rsp+4*16]
%define P0 [rsp+5*16]
%define Q0 [rsp+6*16]
%define Q1 [rsp+7*16]
%define Q2 [rsp+8*16]
%endif
    mova          P2, m4
    mova          P1, m2
    mova          P0, m7
    mova          Q0, m6
    mova          Q1, m1
    mova          Q2, m0
%endif ; x86-32/64
%else ; %1 == 16
    ; We only use 14 pixels but we'll need the remainder at the end for
    ; the second transpose
    mova          m0, [dstq+strideq*0-16]
    mova          m1, [dstq+strideq*1-16]
    mova          m2, [dstq+strideq*2-16]
    mova          m3, [dstq+stride3q -16]
    lea         tmpq, [dstq+strideq*4]
    mova          m4, [tmpq+strideq*0-16]
    mova          m5, [tmpq+strideq*1-16]
    mova          m6, [tmpq+strideq*2-16]
%if ARCH_X86_64
    mova          m7, [tmpq+stride3q -16]

    TRANSPOSE8X8W 0, 1, 2, 3, 4, 5, 6, 7, 8
    SWAP           5, 13
    SWAP           6, 8
    SWAP           7, 9
%define P2 m13
%define P1 m8
%define P0 m9
%else ; x86-32
%define P2 [esp+18*16]
%define P1 [esp+19*16]
%define P0 [esp+20*16]
    TRANSPOSE8X8W 0, 1, 2, 3, 4, 5, 6, 7, \
                     [tmpq+stride3q -16], P2, "", a, a
    mova          P1, m6
    mova          P0, m7
%endif ; x86-32/64
    mova [rsp+ 7*16], m0
    mova [rsp+ 8*16], m1
    mova [rsp+ 9*16], m2
    mova [rsp+10*16], m3
%define P3 [rsp+6*16]
    mova          P3, m4

    mova          m0, [dstq+strideq*0]
    mova          m1, [dstq+strideq*1]
    mova          m2, [dstq+strideq*2]
    mova          m3, [dstq+stride3q ]
    lea         tmpq, [dstq+strideq*4]
    mova          m4, [tmpq+strideq*0]
    mova          m5, [tmpq+strideq*1]
    mova          m6, [tmpq+strideq*2]
%if ARCH_X86_64
    mova          m7, [tmpq+stride3q ]

    TRANSPOSE8X8W 0, 1, 2, 3, 4, 5, 6, 7, 10
    SWAP          0, 10
    SWAP          1, 11
    SWAP          2, 14
    SWAP          3, 15
%define Q0 m10
%define Q1 m11
%define Q2 m14
%define Q3 m15
%else ; x86-32
    TRANSPOSE8X8W 0, 1, 2, 3, 4, 5, 6, 7, \
                     [tmpq+stride3q ], [rsp+12*16], "", a, a
%define Q0 [esp+21*16]
%define Q1 [esp+22*16]
%define Q2 [esp+23*16]
%define Q3 [esp+24*16]
    mova         Q0, m0
    mova         Q1, m1
    mova         Q2, m2
    mova         Q3, m3
%endif ; x86-32/64

    mova [rsp+11*16], m4
%if ARCH_X86_64
    mova [rsp+12*16], m5
%endif
    mova [rsp+13*16], m6
    mova [rsp+14*16], m7
%endif ; %1 == 4/6/8/16
%endif ; %2 ==/!= v

    ; load L/E/I/H
%if ARCH_X86_32
%define l_strideq r5
    mov    l_strideq, dword lstridem
%ifidn %2, v
%define lq r3
    mov           lq, dword lm
%endif
%endif
%ifidn %2, v
%if cpuflag(sse4)
    pmovzxbw      m1, [lq]
    pmovzxbw      m0, [lq+l_strideq]
    pxor          m2, m2
%else ; ssse3
    movq          m1, [lq]
    movq          m0, [lq+l_strideq]
    pxor          m2, m2
    REPX {punpcklbw x, m2}, m1, m0
%endif ; ssse3/sse4
%else ; %2 != v
    movq          m0, [lq]                      ; l0, l1
    movq          m1, [lq+l_strideq]            ; l2, l3
    punpckldq     m0, m1                        ; l0, l2, l1, l3
    pxor          m2, m2
    punpcklbw     m1, m0, m2                    ; l0, l2
    punpckhbw     m0, m2                        ; l1, l3
%endif ; %2==/!=v
%if ARCH_X86_32
%ifidn %2, v
%undef lq
    mov     mstrideq, mstridem
%endif
%endif
    pcmpeqw       m5, m2, m0
    pand          m1, m5
    por           m0, m1                        ; l[x][] ? l[x][] : l[x-stride][]
    pshufb        m0, [PIC_sym(pb_4x1_4x5_4x9_4x13)] ; l[x][1]
    pcmpeqw       m5, m2, m0                    ; !L
    psrlw         m5, 1
%if ARCH_X86_64
    psrlw         m2, m0, [lutq+128]
    SPLATW        m1, [lutq+136]
%else ; x86-32
    mov           r5, lutm
    psrlw         m2, m0, [r5+128]
    SPLATW        m1, [r5+136]
%endif ; x86-32/64
    pminsw        m2, m1
    pmaxsw        m2, [PIC_sym(pw_1)]           ; I
    psrlw         m1, m0, 4                     ; H
    paddw         m0, [PIC_sym(pw_2)]
    paddw         m0, m0
    paddw         m0, m2                        ; E
    REPX {pmullw x, [bdmulq]}, m0, m1, m2
%if ARCH_X86_32
%undef l_strideq
    lea    stride3q, [strideq*3]
%endif

    psubw         m3, P1, P0                    ; p1-p0
    psubw         m4, Q0, Q1                    ; q0-q1
    REPX {pabsw x, x}, m3, m4
    pmaxsw        m3, m5
    pmaxsw        m3, m4
    pcmpgtw       m7, m3, m1                    ; hev
%if %1 != 4
    psubw         m4, P2, P0                    ; p2-p0
    pabsw         m4, m4
    pmaxsw        m4, m3
%if %1 != 6
    mova          m6, P3                        ; p3
    psubw         m5, m6, P0                    ; p3-p0
    pabsw         m5, m5
    pmaxsw        m4, m5
%endif ; %1 != 6
    psubw         m5, Q0, Q2                    ; q0-q2
    pabsw         m5, m5
    pmaxsw        m4, m5
%if %1 != 6
    psubw         m5, Q0, Q3                    ; q0-q3
    pabsw         m5, m5
    pmaxsw        m4, m5
%endif ; %1 != 6
    pcmpgtw       m4, [bdmulq]                     ; !flat8in

    psubw         m5, P2, P1                    ; p2-p1
    pabsw         m5, m5
%if %1 != 6
    psubw         m6, P2                        ; p3-p2
    pabsw         m6, m6
    pmaxsw        m5, m6
    psubw         m6, Q2, Q3                    ; q2-q3
    pabsw         m6, m6
    pmaxsw        m5, m6
%endif ; %1 != 6
    psubw         m6, Q2, Q1                    ; q2-q1
    pabsw         m6, m6
    pmaxsw        m5, m6

%if %1 == 16
    SPLATD        m6, [maskq+8]
    SPLATD        m1, [maskq+4]
    por           m6, m1
    pand          m6, m12
    pcmpeqd       m6, m12
    pand          m5, m6
%else ; %1 != 16
    SPLATD        m6, [maskq+4]
    pand          m6, m12
    pcmpeqd       m6, m12
    pand          m5, m6                        ; only apply fm-wide to wd>4 blocks
%endif ; %1==/!=16
    pmaxsw        m3, m5
%endif ; %1 != 4
    pcmpgtw       m3, m2

    psubw         m5, P1, Q1                    ; p1-q1
    psubw         m6, P0, Q0                    ; p0-q0
    REPX {pabsw x, x}, m5, m6
    paddw         m6, m6
    psrlw         m5, 1
    paddw         m5, m6                        ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    pcmpgtw       m5, m0                        ; abs(p0-q0)*2+(abs(p1-q1)>>1) > E
    por           m3, m5

%if %1 == 16

%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]
    mova          m1, [tmpq+strideq*2]
    mova          m2, [tmpq+stride3q]
%else ; %2 != v
    mova          m0, [rsp+ 8*16]
    mova          m1, [rsp+ 9*16]
    mova          m2, [rsp+10*16]
%endif ; %2==/!=v
    REPX {psubw x, P0}, m0, m1, m2
    REPX {pabsw x, x}, m0, m1, m2
    pmaxsw        m1, m0
    pmaxsw        m1, m2
%ifidn %2, v
    lea         tmpq, [dstq+strideq*4]
    mova          m0, [tmpq+strideq*0]
    mova          m2, [tmpq+strideq*1]
    mova          m5, [tmpq+strideq*2]
%else ; %2 != v
    mova          m0, [rsp+11*16]
    mova          m2, [rsp+12*16]
    mova          m5, [rsp+13*16]
%endif ; %2==/!=v
    REPX {psubw x, Q0}, m0, m2, m5
    REPX {pabsw x, x}, m0, m2, m5
    pmaxsw        m0, m2
    pmaxsw        m1, m5
    pmaxsw        m1, m0
    pcmpgtw       m1, [bdmulq]                  ; !flat8out
    por           m1, m4                        ; !flat8in | !flat8out
    SPLATD        m2, [maskq+8]
    pand          m5, m2, m12
    pcmpeqd       m5, m12
    pandn         m1, m5                        ; flat16
    pandn         m5, m3, m1                    ; flat16 & fm
    SWAP           1, 5

    SPLATD        m5, [maskq+4]
    por           m5, m2
    pand          m2, m5, m12
    pcmpeqd       m2, m12
    pandn         m4, m2                        ; flat8in
    pandn         m2, m3, m4
    SWAP           2, 4
    SPLATD        m2, [maskq+0]
    por           m2, m5
    pand          m2, m12
    pcmpeqd       m2, m12
    pandn         m3, m2
    pandn         m0, m4, m3                    ; fm & !flat8 & !flat16
    SWAP           0, 3
    pandn         m0, m1, m4                    ; flat8 & !flat16
    SWAP           0, 4
%elif %1 != 4
    SPLATD        m0, [maskq+4]
    pand          m2, m0, m12
    pcmpeqd       m2, m12
    pandn         m4, m2
    pandn         m2, m3, m4                    ; flat8 & fm
    SWAP           2, 4
    SPLATD        m2, [maskq+0]
    por           m0, m2
    pand          m0, m12
    pcmpeqd       m0, m12
    pandn         m3, m0
    pandn         m0, m4, m3                    ; fm & !flat8
    SWAP           0, 3
%else ; %1 == 4
    SPLATD        m0, [maskq+0]
    pand          m0, m12
    pcmpeqd       m0, m12
    pandn         m3, m0                        ; fm
%endif ; %1==/!=4

    ; short filter
%if ARCH_X86_64
    SPLATW        m0, r7m
%else
    SPLATW        m0, bdmulm
%endif
    pcmpeqw       m2, m2
    psrlw         m0, 1                         ; 511 or 2047
    pxor          m2, m0                        ; -512 or -2048

    psubw         m5, Q0, P0                    ; q0-p0
    paddw         m6, m5, m5
    paddw         m6, m5                        ; 3*(q0-p0)
    psubw         m5, P1, Q1                    ; iclip_diff(p1-q1)
    pminsw        m5, m0
    pmaxsw        m5, m2
    pand          m5, m7                        ; f=iclip_diff(p1-q1)&hev
    paddw         m5, m6                        ; f=iclip_diff(3*(q0-p0)+f)
    pminsw        m5, m0
    pmaxsw        m5, m2
    pand          m3, m5                        ; f&=fm
    paddw         m5, m3, [PIC_sym(pw_3)]
    paddw         m3, [PIC_sym(pw_4)]
    REPX {pminsw x, m0}, m5, m3
    psraw         m5, 3                         ; f2
    psraw         m3, 3                         ; f1
    psubw         m0, m2                        ; 1023 or 4095
    pxor          m2, m2
%if ARCH_X86_64
    paddw         P0, m5
    psubw         Q0, m3
%else
    paddw          m5, P0
    psubw          m6, Q0, m3
    REPX {pminsw x, m0}, m5, m6
    REPX {pmaxsw x, m2}, m5, m6
%endif

    paddw         m3, [PIC_sym(pw_1)]
    psraw         m3, 1                         ; f=(f1+1)>>1
    pandn         m7, m3                        ; f&=!hev
    SWAP           7, 3
%if ARCH_X86_64
    paddw         P1, m3
    psubw         Q1, m3
    REPX {pminsw x, m0}, P1, P0, Q0, Q1
    REPX {pmaxsw x, m2}, P1, P0, Q0, Q1
%else
    psubw         m7, Q1, m3
    paddw         m3, P1
    REPX {pminsw x, m0}, m7, m3
    REPX {pmaxsw x, m2}, m7, m3
%if %1 > 4
    mova          P1, m3
    mova          P0, m5
    mova          Q0, m6
    mova          Q1, m7
%endif
%endif

%if %1 == 16

; m8-11 = p1/p0/q0/q1, m4=flat8, m1=flat16
; m12=filter bits mask
; m13-15=p2/q2/q3
; m0,2-3,5-7 = free

    ; flat16 filter
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]          ; p6
    mova          m2, [tmpq+strideq*2]          ; p5
    mova          m7, [tmpq+stride3q]           ; p4
    mova          m6, [tmpq+strideq*4]          ; p3
    lea         tmpq, [dstq+mstrideq*4]
%else ; %2 != v
    mova          m0, [rsp+ 8*16]
    mova          m2, [rsp+ 9*16]
    mova          m7, [rsp+10*16]
    mova          m6, [rsp+ 6*16]
%endif ; %2==/!=v

    mova [rsp+ 0*16], m4

    ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
    psllw         m3, m0, 3                     ; p6*8
    paddw         m3, [PIC_sym(pw_8)]
    paddw         m5, m2, m7                    ; p5+p4
    psubw         m3, m0
    paddw         m5, m5                        ; (p5+p4)*2
    paddw         m3, m6                        ; p6*7+p3
    paddw         m5, P2                        ; (p5+p4)*2+p2
    paddw         m3, P1                        ; p6*7+p3+p1
    paddw         m5, P0                        ; (p5+p4)*2+p2+p0
    paddw         m3, Q0                        ; p6*7+p3+p1+q0
    paddw         m3, m5                        ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, m2
    por           m5, m4
%ifidn %2, v
    mova [tmpq+mstrideq*2], m5                   ; p5
%else ; %2 != v
    mova  [rsp+9*16], m5
%endif ; %2==/!=v

    ; sub p6*2, add p3/q1
    paddw         m3, m6
    paddw         m5, m0, m0
    paddw         m3, Q1
    psubw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, m7
    por           m5, m4
%ifidn %2, v
    mova [tmpq+mstrideq*1], m5                   ; p4
%else ; %2 != v
    mova [rsp+10*16], m5
%endif ; %2==/!=v

    ; sub p6/p5, add p2/q2
    psubw         m3, m0
    paddw         m5, P2, Q2
    psubw         m3, m2
    paddw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, m6
    por           m5, m4
%ifidn %2, v
    mova [tmpq+strideq*0], m5                  ; p3
%else ; %2 != v
    mova  [rsp+6*16], m5
%endif ; %2==/!=v

%define WRITE_IN_PLACE 0
%ifidn %2, v
%if ARCH_X86_64
%define WRITE_IN_PLACE 1
%endif
%endif

    ; sub p6/p4, add p1/q3
    paddw         m3, P1
    paddw         m5, m0, m7
    paddw         m3, Q3
    psubw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, P2
    por           m5, m4
%if WRITE_IN_PLACE
    mova [tmpq+strideq*1], m5
%else
    mova  [rsp+1*16], m5                        ; don't clobber p2/m13
%endif

    ; sub p6/p3, add p0/q4
    paddw         m3, P0
    paddw         m5, m0, m6
%ifidn %2, v
    paddw         m3, [dstq+strideq*4]
%else ; %2 != v
    paddw         m3, [rsp+11*16]
%endif ; %2==/!=v
    psubw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, P1
    por           m5, m4
%if WRITE_IN_PLACE
    mova [dstq+mstrideq*2], m5
%else
    mova  [rsp+2*16], m5                        ; don't clobber p1/m3
%endif

    ; sub p6/p2, add q0/q5
    paddw         m3, Q0
    paddw         m5, m0, P2
%ifidn %2, v
%if ARCH_X86_32
    lea           r4, P2
%endif
    lea         tmpq, [dstq+strideq*4]
    paddw         m3, [tmpq+strideq*1]
%else ; %2 != v
    paddw         m3, [rsp+12*16]
%endif ; %2==/!=v
    psubw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, P0
    por           m5, m4
%if WRITE_IN_PLACE
    mova [dstq+mstrideq*1], m5
%else
    mova  [rsp+3*16], m5                        ; don't clobber p0/m4
%endif

    ; sub p6/p1, add q1/q6
    paddw         m3, Q1
    paddw         m5, m0, P1
%ifidn %2, v
    mova          m0, [tmpq+strideq*2]          ; q6
%else ; %2 != v
    mova          m0, [rsp+13*16]               ; q6
%endif ; %2==/!=v
    paddw         m3, m0
    psubw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, Q0
    por           m5, m4
%if WRITE_IN_PLACE
    mova      [dstq], m5
%else
    mova  [rsp+4*16], m5                        ; don't clobber q0/m5
%endif

    ; sub p5/p0, add q2/q6
    paddw         m3, Q2
    paddw         m5, m2, P0
    paddw         m3, m0
    psubw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
    pandn         m4, m1, Q1
    por           m2, m5, m4                    ; don't clobber q1/m6

    ; sub p4/q0, add q3/q6
    paddw         m3, Q3
    paddw         m7, Q0
    paddw         m3, m0
    psubw         m3, m7
    psrlw         m7, m3, 4
    pand          m7, m1
    pandn         m4, m1, Q2
    por           m7, m4                        ; don't clobber q2/m14

    ; sub p3/q1, add q4/q6
%ifidn %2, v
    paddw         m3, [tmpq+strideq*0]
%else ; %2 != v
    paddw         m3, [rsp+11*16]
%endif ; %2==/!=v
    paddw         m6, Q1
    paddw         m3, m0
    psubw         m3, m6
    psrlw         m6, m3, 4
    pand          m6, m1
    pandn         m4, m1, Q3
    por           m6, m4
%if WRITE_IN_PLACE
    mova [tmpq+mstrideq], m6                    ; q3
%else ; %2 != v
    mova  [rsp+5*16], m6
%endif ; %2==/!=v

    ; sub p2/q2, add q5/q6
%ifidn %2, v
    paddw         m3, [tmpq+strideq*1]
%if ARCH_X86_64
    paddw         m5, P2, Q2
%else
    ; because tmpq is clobbered, so we use a backup pointer for P2 instead
    paddw         m5, [r4], Q2
    mov     pic_regq, pic_regm
%endif
%else ; %2 != v
    paddw         m3, [rsp+12*16]
    paddw         m5, P2, Q2
%endif ; %2==/!=v
    paddw         m3, m0
    psubw         m3, m5
    psrlw         m5, m3, 4
    pand          m5, m1
%ifidn %2, v
    pandn         m4, m1, [tmpq+strideq*0]
%else ; %2 != v
    pandn         m4, m1, [rsp+11*16]
%endif ; %2==/!=v
    por           m5, m4
%ifidn %2, v
    mova [tmpq+strideq*0], m5                   ; q4
%else ; %2 != v
    mova [rsp+11*16], m5
%endif ; %2==/!=v

    ; sub p1/q3, add q6*2
    psubw         m3, P1
    paddw         m0, m0
    psubw         m3, Q3
    paddw         m3, m0
    psrlw         m5, m3, 4
    pand          m5, m1
%ifidn %2, v
    pandn         m4, m1, [tmpq+strideq*1]
%else ; %2 != v
    pandn         m4, m1, [rsp+12*16]
%endif ; %2==/!=v
    por           m5, m4
%ifidn %2, v
    mova [tmpq+strideq*1], m5                   ; q5
%else ; %2 != v
    mova [rsp+12*16], m5
%endif ; %2==/!=v

    mova          m4, [rsp+0*16]
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*4]
%endif
%if ARCH_X86_64
    SWAP           2, 11
    SWAP           7, 14
    SWAP           6, 15
%else ; x86-32
    mova          Q1, m2
    mova          Q2, m7
%endif ; x86-32/64
%if WRITE_IN_PLACE
    mova          P2, [tmpq+strideq*1]
    mova          P1, [tmpq+strideq*2]
    mova          P0, [tmpq+stride3q]
    mova          Q0, [dstq]
%elif ARCH_X86_64
    mova          P2, [rsp+1*16]
    mova          P1, [rsp+2*16]
    mova          P0, [rsp+3*16]
    mova          Q0, [rsp+4*16]
%else ; !WRITE_IN_PLACE & x86-32
    mova          m0, [rsp+1*16]
    mova          m1, [rsp+2*16]
    mova          m2, [rsp+3*16]
    mova          m3, [rsp+4*16]
    mova          m7, [rsp+5*16]
    mova          P2, m0
    mova          P1, m1
    mova          P0, m2
    mova          Q0, m3
    mova          Q3, m7
%endif ; WRITE_IN_PLACE / x86-32/64
%undef WRITE_IN_PLACE
%endif ; %1 == 16

%if %1 >= 8

    ; flat8 filter
    mova          m0, P3                        ; p3
    paddw         m1, m0, P2                    ; p3+p2
    paddw         m2, P1, P0                    ; p1+p0
    paddw         m3, m1, m1                    ; 2*(p3+p2)
    paddw         m2, m0                        ; p1+p0+p3
    paddw         m3, Q0                        ; 2*(p3+p2)+q0
    paddw         m2, m3                        ; 3*p3+2*p2+p1+p0+q0
    pmulhrsw      m7, m2, [PIC_sym(pw_4096)]
    psubw         m7, P2
    pand          m7, m4

    paddw         m3, P1, Q1                    ; p1+q1
    psubw         m2, m1                        ; 2*p3+p2+p1+p0+q0
    paddw         m2, m3                        ; 2*p3+p2+2*p1+p0+q0+q1
    pmulhrsw      m3, m2, [PIC_sym(pw_4096)]
    psubw         m3, P1
    pand          m3, m4

    paddw         m5, m0, P1                    ; p3+p1
    paddw         m6, P0, Q2                    ; p0+q2
    psubw         m2, m5                        ; p3+p2+p1+p0+q0+q1
    paddw         m2, m6                        ; p3+p2+p1+2*p0+q0+q1+q2
    pmulhrsw      m5, m2, [PIC_sym(pw_4096)]
    psubw         m5, P0
    pand          m5, m4

    paddw         m6, m0, P0                    ; p3+p0
    paddw         m1, Q0, Q3                    ; q0+q3
    psubw         m2, m6                        ; p2+p1+p0+q0+q1+q2
    paddw         m2, m1                        ; p2+p1+p0+2*q0+q1+q2+q3
    pmulhrsw      m6, m2, [PIC_sym(pw_4096)]
    psubw         m6, Q0
    pand          m6, m4

    paddw         m2, Q1                        ; p2+p1+p0+2*q0+2*q1+q2+q3
    paddw         m2, Q3                        ; p2+p1+p0+2*q0+2*q1+q2+2*q3
    paddw         m1, P2, Q0                    ; p2+q0
    psubw         m2, m1                        ; p1+p0+q0+2*q1+q2+2*q3
    pmulhrsw      m1, m2, [PIC_sym(pw_4096)]
    psubw         m1, Q1
    pand          m1, m4

    psubw         m2, P1                        ; p0+q0+2*q1+q2+2*q3
    psubw         m2, Q1                        ; p0+q0+q1+q2+2*q3
    paddw         m0, Q3, Q2                    ; q3+q2
    paddw         m2, m0                        ; p0+q0+q1+2*q2+3*q3
    pmulhrsw      m2, [PIC_sym(pw_4096)]
    psubw         m2, Q2
    pand          m2, m4

    paddw         m7, P2
    paddw         m3, P1
    paddw         m5, P0
    paddw         m6, Q0
    paddw         m1, Q1
    paddw         m2, Q2

%ifidn %2, v
    mova [tmpq+strideq*1], m7                   ; p2
    mova [tmpq+strideq*2], m3                   ; p1
    mova [tmpq+stride3q ], m5                   ; p0
    mova [dstq+strideq*0], m6                   ; q0
    mova [dstq+strideq*1], m1                   ; q1
    mova [dstq+strideq*2], m2                   ; q2
%else ; %2 != v
    mova          m0, P3

%if %1 == 8
    lea         tmpq, [dstq+strideq*4]
%if ARCH_X86_64
    SWAP           4, 15
    TRANSPOSE8X8W  0, 7, 3, 5, 6, 1, 2, 4, 8
%else
    TRANSPOSE8X8W  0, 7, 3, 5, 6, 1, 2, 4, "", \
                      Q3, [tmpq+strideq*1-8], a, u
%endif

    ; write 8x8
    movu   [dstq+strideq*0-8], m0
    movu   [dstq+strideq*1-8], m7
    movu   [dstq+strideq*2-8], m3
    movu   [dstq+stride3q -8], m5
    movu   [tmpq+strideq*0-8], m6
%if ARCH_X86_64
    movu   [tmpq+strideq*1-8], m1
%endif
    movu   [tmpq+strideq*2-8], m2
    movu   [tmpq+stride3q -8], m4
    lea         dstq, [dstq+strideq*8]
%else ; %1 != 8
%if ARCH_X86_64
    SWAP           6, 8
    SWAP           1, 9
    SWAP           2, 10
%else
    mova  [rsp+1*16], m6
    mova  [rsp+2*16], m1
    mova  [rsp+3*16], m2
%endif

    mova          m1, [rsp+ 7*16]
    mova          m2, [rsp+ 8*16]
    mova          m4, [rsp+ 9*16]
    mova          m6, [rsp+10*16]
    lea         tmpq, [dstq+strideq*4]
%if ARCH_X86_64
    TRANSPOSE8X8W  1, 2, 4, 6, 0, 7, 3, 5, 11
%else
    mova  [rsp+7*16],  m5
    TRANSPOSE8X8W  1, 2, 4, 6, 0, 7, 3, 5, "", \
                      [rsp+7*16], [tmpq+strideq*1-16], a, a
%endif

    mova [dstq+strideq*0-16], m1
    mova [dstq+strideq*1-16], m2
    mova [dstq+strideq*2-16], m4
    mova [dstq+stride3q -16], m6
    mova [tmpq+strideq*0-16], m0
%if ARCH_X86_64
    mova [tmpq+strideq*1-16], m7
%endif
    mova [tmpq+strideq*2-16], m3
    mova [tmpq+stride3q -16], m5

%if ARCH_X86_64
    SWAP           6, 8
    SWAP           1, 9
    SWAP           2, 10
    SWAP           4, 15
%else
    mova          m6, [rsp+1*16]
    mova          m1, [rsp+2*16]
    mova          m2, [rsp+3*16]
    mova          m4, Q3
%endif
    mova          m0, [rsp+11*16]
    mova          m3, [rsp+12*16]
    mova          m5, [rsp+13*16]
%if ARCH_X86_64
    mova          m7, [rsp+14*16]
    TRANSPOSE8X8W  6, 1, 2, 4, 0, 3, 5, 7, 8
%else
    TRANSPOSE8X8W  6, 1, 2, 4, 0, 3, 5, 7, "", \
                      [rsp+14*16], [tmpq+strideq*1], a, a
%endif
    mova [dstq+strideq*0], m6
    mova [dstq+strideq*1], m1
    mova [dstq+strideq*2], m2
    mova [dstq+stride3q ], m4
    mova [tmpq+strideq*0], m0
%if ARCH_X86_64
    mova [tmpq+strideq*1], m3
%endif
    mova [tmpq+strideq*2], m5
    mova [tmpq+stride3q ], m7
    lea         dstq, [dstq+strideq*8]
%endif ; %1==/!=8
%endif ; %2==/!=v
%elif %1 == 6
    ; flat6 filter
    paddw         m3, P1, P0                    ; p1+p0
    paddw         m3, P2                        ; p2+p1+p0
    paddw         m6, P2, Q0                    ; p2+q0
    paddw         m3, m3                        ; 2*(p2+p1+p0)
    paddw         m3, m6                        ; p2+2*(p2+p1+p0)+q0
    pmulhrsw      m2, m3, [PIC_sym(pw_4096)]
    psubw         m2, P1
    pand          m2, m4

    paddw         m3, Q0                        ; p2+2*(p2+p1+p0+q0)
    paddw         m6, P2, P2                    ; 2*p2
    paddw         m3, Q1                        ; p2+2*(p2+p1+p0+q0)+q1
    psubw         m3, m6                        ; p2+2*(p1+p0+q0)+q1
    pmulhrsw      m5, m3, [PIC_sym(pw_4096)]
    psubw         m5, P0
    pand          m5, m4

    paddw         m3, Q1                        ; p2+2*(p1+p0+q0+q1)
    paddw         m6, P2, P1                    ; p2+p1
    paddw         m3, Q2                        ; p2+2*(p1+p0+q0+q1)+q2
    psubw         m3, m6                        ; p1+2*(p0+q0+q1)+q2
    pmulhrsw      m6, m3, [PIC_sym(pw_4096)]
    psubw         m6, Q0
    pand          m6, m4

    psubw         m3, P1                        ; 2*(p0+q0+q1)+q2
%if ARCH_X86_64
    paddw         Q2, Q2                        ; q2*2
%else
    mova          m0, Q2
    paddw         m0, m0
%endif
    psubw         m3, P0                        ; p0+2*(q0+q1)+q2
%if ARCH_X86_64
    paddw         m3, Q2                        ; p0+q*(q0+q1+q2)+q2
%else
    paddw         m3, m0
%endif
    pmulhrsw      m3, [PIC_sym(pw_4096)]
    psubw         m3, Q1
    pand          m3, m4

    paddw         m2, P1
    paddw         m5, P0
    paddw         m6, Q0
    paddw         m3, Q1

%ifidn %2, v
    mova [dstq+mstrideq*2], m2                   ; p1
    mova [dstq+mstrideq*1], m5                   ; p0
    mova [dstq+strideq*0], m6                   ; q0
    mova [dstq+strideq*1], m3                   ; q1
%else ; %2 != v
    TRANSPOSE_8x4_AND_WRITE_4x8 m2, m5, m6, m3, m0
%endif ; %2==/!=v
%else ; %1 == 4
%if ARCH_X86_64
%ifidn %2, v
    mova [dstq+mstrideq*2], P1                   ; p1
    mova [dstq+mstrideq*1], P0                   ; p0
    mova [dstq+strideq*0], Q0                   ; q0
    mova [dstq+strideq*1], Q1                   ; q1
%else ; %2 != v
    TRANSPOSE_8x4_AND_WRITE_4x8 P1, P0, Q0, Q1, m0
%endif ; %2==/!=v
%else ; x86-32
%ifidn %2, v
    mova [dstq+mstrideq*2], m3
    mova [dstq+mstrideq*1], m5
    mova [dstq+strideq*0], m6
    mova [dstq+strideq*1], m7
%else ; %2 != v
    TRANSPOSE_8x4_AND_WRITE_4x8 m3, m5, m6, m7, m0
%endif ; %2==/!=v
%endif ; x86-32/64
%endif ; %1
%undef P3
%undef P2
%undef P1
%undef P0
%undef Q0
%undef Q1
%undef Q2
%undef Q3
%endmacro

INIT_XMM ssse3
; stack layout:
; r0 - flat8 backup inside flat16 code
%if ARCH_X86_64
cglobal lpf_v_sb_y_16bpc, 6, 12, 16, -16 * 1, \
                          dst, stride, mask, l, l_stride, lut, \
                          w, stride3, mstride, tmp, mask_bits, bdmul
    mov          r6d, r7m
    sar          r6d, 7
    and          r6d, 16                      ; 0 for 10bpc, 16 for 12bpc
    lea       bdmulq, [pw_4]
    add       bdmulq, r6
    mov           wd, wm
    shl    l_strideq, 2
    sub           lq, l_strideq
%else
; stack layout [32bit only]:
; r1-4 - p2-q0 post-filter16
; r5 - p3
; r6 - q3 post-filter16
; r7 - GPRs [mask_bitsm, mstridem]
; r8 - m12/pb_mask
; r9 - bdmulq
cglobal lpf_v_sb_y_16bpc, 4, 7, 8, -16 * (10 + extra_stack), \
                          dst, stride, mask, mstride, pic_reg, stride3, tmp
    RELOC_ARGS     v, 10*16
%if STACK_ALIGNMENT >= 16
    mov          r5d, r7m
%endif
    sar          r5d, 7
    and          r5d, 16                      ; 0 for 10bpc, 16 for 12bpc
    LEA     pic_regq, PIC_base
%define pic_regm dword [esp+7*16+2*gprsize]
    mov     pic_regm, pic_regq
    mova          m0, [PIC_sym(pw_4)+r5]
%define bdmulq esp+9*16
    mova    [bdmulq], m0
    shl dword lstridem, 2
    sub           r3, dword lstridem
    mov     dword lm, r3
%endif
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]
%if ARCH_X86_64
    mov   mask_bitsd, 0x3
    mova         m12, [pb_mask]
%else
%define mstridem dword [esp+7*16+1*gprsize]
    mov     mstridem, mstrideq
%define mask_bitsm dword [esp+7*16+0*gprsize]
    mov   mask_bitsm, 0x3
    mova          m0, [PIC_sym(pb_mask)]
%define m12 [esp+8*16]
    mova         m12, m0
%endif

.loop:
%if ARCH_X86_64
    test   [maskq+8], mask_bitsd              ; vmask[2]
%else
    mov          r6d, mask_bitsm
    test   [maskq+8], r6d
%endif
    jz .no_flat16

    FILTER        16, v
    jmp .end

.no_flat16:
%if ARCH_X86_64
    test   [maskq+4], mask_bitsd              ; vmask[1]
%else
    test   [maskq+4], r6d
%endif
    jz .no_flat

    FILTER         8, v
    jmp .end

.no_flat:
%if ARCH_X86_64
    test   [maskq+0], mask_bitsd              ; vmask[0]
%else
    test   [maskq+0], r6d
%endif
    jz .end

    FILTER         4, v

.end:
%if ARCH_X86_64
    pslld        m12, 2
    add           lq, 8
%else
    mova          m0, m12
    pslld         m0, 2
    mova         m12, m0
    add     dword lm, 8
%endif
    add         dstq, 16
%if ARCH_X86_64
    shl   mask_bitsd, 2
    sub           wd, 2
%else
    shl   mask_bitsm, 2
    sub     dword wm, 2
%endif
    jg .loop
%undef mask_bitsm
%undef bdmulq
    UNRELOC_ARGS
    RET

INIT_XMM ssse3
; stack layout:
; r0 - flat8 backup inside flat16
; r1-4 - p2-q0 post-filter16 backup
; r5 - q3 post-filter16 backup
; r6 - p3
; r7-10 - p7-4
; r11-14 - q4-7
%if ARCH_X86_64
cglobal lpf_h_sb_y_16bpc, 6, 11, 16, -16 * 15, \
                          dst, stride, mask, l, l_stride, lut, \
                          h, stride3, tmp, mask_bits, bdmul
    mov          r6d, r7m
    sar          r6d, 7
    and          r6d, 16                      ; 0 for 10bpc, 16 for 12bpc
    lea       bdmulq, [pw_4]
    add       bdmulq, r6
    mov           hd, hm
    shl    l_strideq, 2
%else
; stack layout [32bit only]:
; r15 - GPRs [mask_bitsm]
; r16 - m12/pb_mask
; r17 - bdmulq
; r18-24 - p2-q3
cglobal lpf_h_sb_y_16bpc, 4, 7, 8, -16 * (25 + extra_stack), \
                          dst, stride, mask, l, pic_reg, stride3, tmp
    RELOC_ARGS     h, 25*16
%if STACK_ALIGNMENT >= 16
    mov          r5d, r7m
%endif
    sar          r5d, 7
    and          r5d, 16                      ; 0 for 10bpc, 16 for 12bpc
    LEA     pic_regq, PIC_base
    mova          m0, [PIC_sym(pw_4)+r5]
%define bdmulq esp+17*16
    mova    [bdmulq], m0
    shl dword lstridem, 2
%endif
    sub           lq, 4
    lea     stride3q, [strideq*3]
%if ARCH_X86_64
    mov   mask_bitsd, 0x3
    mova         m12, [pb_mask]
%else
%define mask_bitsm dword [esp+15*16+0*gprsize]
    mov   mask_bitsm, 0x3
    mova          m0, [PIC_sym(pb_mask)]
%define m12 [esp+16*16]
    mova         m12, m0
%endif

.loop:
%if ARCH_X86_64
    test   [maskq+8], mask_bitsd            ; vmask[2]
%else
    mov         r6d, mask_bitsm
    test   [maskq+8], r6d
%endif
    jz .no_flat16

    FILTER        16, h
    jmp .end

.no_flat16:
%if ARCH_X86_64
    test   [maskq+4], mask_bitsd            ; vmask[1]
%else
    test   [maskq+4], r6d
%endif
    jz .no_flat

    FILTER         8, h
    jmp .end

.no_flat:
%if ARCH_X86_64
    test   [maskq+0], mask_bitsd            ; vmask[0]
%else
    test   [maskq+0], r6d
%endif
    jz .no_filter

    FILTER         4, h
    jmp .end

.no_filter:
    lea         dstq, [dstq+strideq*8]
.end:
%if ARCH_X86_64
    pslld        m12, 2
    lea           lq, [lq+l_strideq*2]
    shl   mask_bitsd, 2
    sub           hd, 2
%else
    mova          m0, m12
    pslld         m0, 2
    mova         m12, m0
    add           lq, dword lstridem
    add           lq, dword lstridem
    shl   mask_bitsm, 2
    sub     dword hm, 2
%endif
    jg .loop
%undef mask_bitsm
%undef bdmulq
    UNRELOC_ARGS
    RET

INIT_XMM ssse3
%if ARCH_X86_64
cglobal lpf_v_sb_uv_16bpc, 6, 12, 16, \
                           dst, stride, mask, l, l_stride, lut, \
                           w, stride3, mstride, tmp, mask_bits, bdmul
    mov          r6d, r7m
    sar          r6d, 7
    and          r6d, 16                      ; 0 for 10bpc, 16 for 12bpc
    lea       bdmulq, [pw_4]
    add       bdmulq, r6
    mov           wd, wm
    shl    l_strideq, 2
    sub           lq, l_strideq
%else
; stack layout [32bit only]:
; r0 - GPRs [mask_bitsm, mstridem]
; r1 - m12/pb_mask
; r2 - bdmulq
cglobal lpf_v_sb_uv_16bpc, 4, 7, 8, -16 * (3 + extra_stack), \
                           dst, stride, mask, mstride, pic_reg, stride3, tmp
    RELOC_ARGS     v, 3*16
%if STACK_ALIGNMENT >= 16
    mov          r5d, r7m
%endif
    sar          r5d, 7
    and          r5d, 16                      ; 0 for 10bpc, 16 for 12bpc
    LEA     pic_regq, PIC_base
    mova          m0, [PIC_sym(pw_4)+r5]
%define bdmulq esp+2*16
    mova    [bdmulq], m0
    shl dword lstridem, 2
    sub           r3, dword lstridem
    mov     dword lm, r3
%endif
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]
%if ARCH_X86_64
    mov   mask_bitsd, 0x3
    mova         m12, [pb_mask]
%else
%define mask_bitsm dword [esp+0*gprsize]
%define mstridem dword [esp+1*gprsize]
    mov   mask_bitsm, 0x3
    mov     mstridem, mstrideq
    mova          m0, [PIC_sym(pb_mask)]
%define m12 [esp+1*16]
    mova         m12, m0
%endif

.loop:
%if ARCH_X86_64
    test   [maskq+4], mask_bitsd            ; vmask[1]
%else
    mov          r6d, mask_bitsm
    test   [maskq+4], r6d
%endif
    jz .no_flat

    FILTER         6, v
    jmp .end

.no_flat:
%if ARCH_X86_64
    test   [maskq+0], mask_bitsd            ; vmask[0]
%else
    test   [maskq+0], r6d
%endif
    jz .end

    FILTER         4, v

.end:
%if ARCH_X86_64
    pslld        m12, 2
    add           lq, 8
%else
    mova          m0, m12
    pslld         m0, 2
    mova         m12, m0
    add     dword lm, 8
%endif
    add         dstq, 16
%if ARCH_X86_64
    shl   mask_bitsd, 2
    sub           wd, 2
%else
    shl   mask_bitsm, 2
    sub     dword wm, 2
%endif
    jg .loop
%undef mask_bitsm
%undef bdmulq
    UNRELOC_ARGS
    RET

INIT_XMM ssse3
%if ARCH_X86_64
cglobal lpf_h_sb_uv_16bpc, 6, 11, 16, \
                           dst, stride, mask, l, l_stride, lut, \
                           h, stride3, tmp, mask_bits, bdmul
    mov          r6d, r7m
    sar          r6d, 7
    and          r6d, 16                      ; 0 for 10bpc, 16 for 12bpc
    lea       bdmulq, [pw_4]
    add       bdmulq, r6
    mov           hd, hm
    shl    l_strideq, 2
%else
; stack layout [32bit only]:
; r0 - GPRs [mask_bitsm]
; r1 - m12/pb_mask
; r2 - bdmulq
; r3-8 - p2-q2
cglobal lpf_h_sb_uv_16bpc, 4, 7, 8, -16 * (9 + extra_stack), \
                           dst, stride, mask, l, pic_reg, stride3, tmp
    RELOC_ARGS     h, 9*16
%if STACK_ALIGNMENT >= 16
    mov          r5d, r7m
%endif
    sar          r5d, 7
    and          r5d, 16                      ; 0 for 10bpc, 16 for 12bpc
    LEA     pic_regq, PIC_base
    mova          m0, [PIC_sym(pw_4)+r5]
%define bdmulq esp+2*16
    mova    [bdmulq], m0
    shl dword lstridem, 2
%endif
    sub           lq, 4
    lea     stride3q, [strideq*3]
%if ARCH_X86_64
    mov   mask_bitsd, 0x3
    mova         m12, [pb_mask]
%else
%define mask_bitsm dword [esp+0*gprsize]
    mov   mask_bitsm, 0x3
    mova          m0, [PIC_sym(pb_mask)]
%define m12 [esp+1*16]
    mova         m12, m0
%endif

.loop:
%if ARCH_X86_64
    test   [maskq+4], mask_bitsd            ; vmask[1]
%else
    mov          r6d, mask_bitsm
    test   [maskq+4], r6d
%endif
    jz .no_flat

    FILTER         6, h
    jmp .end

.no_flat:
%if ARCH_X86_64
    test   [maskq+0], mask_bitsd            ; vmask[0]
%else
    test   [maskq+0], r6d
%endif
    jz .no_filter

    FILTER         4, h
    jmp .end

.no_filter:
    lea         dstq, [dstq+strideq*8]
.end:
%if ARCH_X86_64
    pslld        m12, 2
    lea           lq, [lq+l_strideq*2]
    shl   mask_bitsd, 2
    sub           hd, 2
%else
    mova          m0, m12
    pslld         m0, 2
    mova         m12, m0
    add           lq, dword lstridem
    add           lq, dword lstridem
    shl   mask_bitsm, 2
    sub     dword hm, 2
%endif
    jg .loop
%undef mask_bitsm
%undef bdmulq
    UNRELOC_ARGS
    RET
