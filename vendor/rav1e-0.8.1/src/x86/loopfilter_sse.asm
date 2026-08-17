; Copyright © 2018-2021, VideoLAN and dav1d authors
; Copyright © 2018, Two Orioles, LLC
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

pb_4x0_4x4_4x8_4x12: db 0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12
pb_7_1: times 8 db 7, 1
pb_3_1: times 8 db 3, 1
pb_2_1: times 8 db 2, 1
pb_m1_0: times 8 db -1, 0
pb_m1_1: times 8 db -1, 1
pb_m1_2: times 8 db -1, 2
pb_1: times 16 db 1
pb_2: times 16 db 2
pb_3: times 16 db 3
pb_4: times 16 db 4
pb_16: times 16 db 16
pb_63: times 16 db 63
pb_64: times 16 db 64
pb_128: times 16 db 0x80
pb_129: times 16 db 0x81
pb_240: times 16 db 0xf0
pb_248: times 16 db 0xf8
pb_254: times 16 db 0xfe

pw_2048: times 8 dw 2048
pw_4096: times 8 dw 4096

pd_mask: dd 1, 2, 4, 8

SECTION .text

%macro ABSSUB 4 ; dst, a, b, tmp
    psubusb       %1, %2, %3
    psubusb       %4, %3, %2
    por           %1, %4
%endmacro

%macro TRANSPOSE_16x4_AND_WRITE_4x16 5
    ; transpose 16x4
    punpcklbw    m%5, m%1, m%2
    punpckhbw    m%1, m%2
    punpcklbw    m%2, m%3, m%4
    punpckhbw    m%3, m%4
    punpcklwd    m%4, m%5, m%2
    punpckhwd    m%5, m%2
    punpcklwd    m%2, m%1, m%3
    punpckhwd    m%1, m%3

    ; write out
%assign %%n 0
%rep 4
    movd [dstq+strideq *0-2], xm%4
    movd [dstq+strideq *4-2], xm%5
    movd [dstq+strideq *8-2], xm%2
    movd [dstq+stride3q*4-2], xm%1
    add         dstq, strideq
%if %%n < 3
    psrldq      xm%4, 4
    psrldq      xm%5, 4
    psrldq      xm%2, 4
    psrldq      xm%1, 4
%endif
%assign %%n (%%n+1)
%endrep
    lea         dstq, [dstq+stride3q*4]
%endmacro

%macro TRANSPOSE_16X16B 2 ; output_transpose, mem
%if %1 == 0
    mova          %2, m15 ; m7 in 32-bit
%endif

    ; input in m0-7
    punpcklbw    m15, m0, m1
    punpckhbw     m0, m1
    punpcklbw     m1, m2, m3
    punpckhbw     m2, m3
    punpcklbw     m3, m4, m5
    punpckhbw     m4, m5
%if ARCH_X86_64
    SWAP           4, 5, 7
%else
 %if %1 == 0
    mova          m5, %2
 %else
    mova          m5, [esp+1*16]
 %endif
    mova          %2, m4
%endif
    punpcklbw     m4, m6, m5
    punpckhbw     m6, m5

    ; interleaved in m15,0,1,2,3,7,4,6
    punpcklwd     m5, m15, m1
    punpckhwd    m15, m1
    punpcklwd     m1, m0, m2
    punpckhwd     m0, m2
    punpcklwd     m2, m3, m4
    punpckhwd     m3, m4
%if ARCH_X86_64
    SWAP           3, 4, 7
%else
    mova          m4, %2
    mova          %2, m3
%endif
    punpcklwd     m3, m4, m6
    punpckhwd     m4, m6

    ; interleaved in m5,15,1,0,2,7,3,4
    punpckldq     m6, m5, m2
    punpckhdq     m5, m2
%if ARCH_X86_64
    SWAP           2, 7, 5
%else
    mova          m2, %2
    mova  [esp+1*16], m5
%endif
    punpckldq     m5, m15, m2
    punpckhdq    m15, m2
    punpckldq     m2, m1, m3
    punpckhdq     m1, m3
    punpckldq     m3, m0, m4
    punpckhdq     m0, m4

%if ARCH_X86_32
    mova  [esp+0*16], m6
    mova  [esp+2*16], m5
    mova  [esp+3*16], m15
    mova  [esp+4*16], m2
    mova  [esp+5*16], m1
    mova  [esp+6*16], m3
    mova  [esp+7*16], m0
    mova          m8, [esp+ 8*16]
    mova          m9, [esp+ 9*16]
    mova         m10, [esp+10*16]
 %if %1 == 0
    mova         m11, [esp+11*16]
    mova         m12, [esp+12*16]
    mova         m13, [esp+13*16]
    mova         m14, [esp+14*16]
 %else
    mova         m11, [esp+20*16]
    mova         m12, [esp+15*16]
    mova         m13, [esp+16*16]
    mova         m14, [esp+17*16]
 %endif
%endif

    ; input in m8-m15
%if ARCH_X86_64
    SWAP           7, 4
%endif
    punpcklbw     m7, m8, m9
    punpckhbw     m8, m9
    punpcklbw     m9, m10, m11
    punpckhbw    m10, m11
    punpcklbw    m11, m12, m13
    punpckhbw    m12, m13
%if ARCH_X86_64
    mova         m13, %2
%else
 %if %1 == 0
    mova         m13, [esp+15*16]
 %else
    mova         m13, [esp+18*16]
 %endif
%endif
    mova          %2, m12
    punpcklbw    m12, m14, m13
    punpckhbw    m14, m14, m13

    ; interleaved in m7,8,9,10,11,rsp%2,12,14
    punpcklwd    m13, m7, m9
    punpckhwd     m7, m9
    punpcklwd     m9, m8, m10
    punpckhwd     m8, m10
    punpcklwd    m10, m11, m12
    punpckhwd    m11, m12
    mova         m12, %2
    mova          %2, m11
    punpcklwd    m11, m12, m14
    punpckhwd    m12, m14

    ; interleaved in m13,7,9,8,10,rsp%2,11,12
    punpckldq    m14, m13, m10
    punpckhdq    m13, m10
    punpckldq    m10, m9, m11
    punpckhdq     m9, m11
    punpckldq    m11, m8, m12
    punpckhdq     m8, m12
    mova         m12, %2
    mova          %2, m8
    punpckldq     m8, m7, m12
    punpckhdq     m7, m12

%if ARCH_X86_32
    mova [esp+ 8*16], m10
    mova [esp+ 9*16], m9
    mova [esp+10*16], m11
    SWAP           6, 1
    SWAP           4, 2
    SWAP           5, 3
    mova          m6, [esp+0*16]
    mova          m4, [esp+1*16]
    mova          m5, [esp+2*16]
%endif

    ; interleaved in m6,7,5,15,2,1,3,0,14,13,10,9,11,rsp%2,8,7
    punpcklqdq   m12, m6, m14
    punpckhqdq    m6, m14
    punpcklqdq   m14, m4, m13
    punpckhqdq    m4, m13
    punpcklqdq   m13, m5, m8
    punpckhqdq    m5, m8
%if ARCH_X86_64
    SWAP           8, 5
%else
    mova          m8, [esp+3*16]
    mova [esp+27*16], m5
 %define m15 m8
%endif
    punpcklqdq    m5, m15, m7
    punpckhqdq   m15, m7

%if ARCH_X86_32
    mova [esp+11*16], m12
    mova [esp+12*16], m6
    mova [esp+13*16], m14
    mova [esp+14*16], m4
    mova [esp+26*16], m13
    mova [esp+ 0*16], m5
    mova [esp+ 1*16], m15
    mova          m2, [esp+ 4*16]
    mova         m10, [esp+ 8*16]
    mova          m1, [esp+ 5*16]
    mova          m9, [esp+ 9*16]
    mova          m3, [esp+ 6*16]
    mova         m11, [esp+10*16]
    mova          m0, [esp+ 7*16]
%endif

    punpcklqdq    m7, m2, m10
    punpckhqdq    m2, m10
    punpcklqdq   m10, m1, m9
    punpckhqdq    m1, m9
    punpcklqdq    m9, m3, m11
    punpckhqdq    m3, m11
    mova         m11, %2
%if ARCH_X86_32
 %define m12 m3
%endif
    mova          %2, m12
    punpcklqdq   m12, m0, m11
    punpckhqdq    m0, m11
%if %1 == 1
    mova         m11, %2
%endif

%if ARCH_X86_64
    ; interleaved m11,6,14,4,13,8,5,15,7,2,10,1,9,3,12,0
    SWAP           0, 11, 1, 6, 5, 8, 7, 15
    SWAP           2, 14, 12, 9
    SWAP           3, 4, 13
%else
 %if %1 == 0
    mova [esp+15*16], m9
    mova [esp+17*16], m12
    mova [esp+18*16], m0
    mova [esp+28*16], m10
    mova [esp+29*16], m1
    mova          m3, [esp+0*16]
    mova          m4, [esp+1*16]
    SWAP          m5, m7
    SWAP          m6, m2
 %else
    SWAP           0, 7
    SWAP           3, 1, 2, 4, 6
 %endif
%endif
%endmacro

%macro FILTER 2 ; width [4/6/8/16], dir [h/v]
%if ARCH_X86_64
 %define %%flat8mem [rsp+0*16]
 %define %%q2mem    [rsp+1*16]
 %define %%q3mem    [rsp+2*16]
%else
 %if %1 == 4 || %1 == 6
  %define %%p2mem      [esp+ 8*16]
  %define %%q2mem      [esp+ 9*16]
  %define %%flat8mem   [esp+10*16]
 %else
  %ifidn %2, v
   %define %%p2mem      [esp+16*16]
   %define %%q2mem      [esp+ 1*16]
   %define %%q3mem      [esp+18*16]
   %define %%flat8mem   [esp+ 0*16]
   %define %%flat16mem  [esp+20*16]
  %else
   %define %%p2mem     [esp+27*16]
   %define %%q2mem     [esp+28*16]
   %define %%q3mem     [esp+29*16]
   %define %%flat8mem  [esp+21*16]
   %define %%flat16mem [esp+30*16]
  %endif
 %endif
 %xdefine m12reg m12
%endif

%if ARCH_X86_32
    lea     stride3q, [strideq*3]
%endif
    ; load data
%ifidn %2, v
%if ARCH_X86_32
    mov     mstrideq, strideq
    neg     mstrideq
%endif
%if %1 == 4
    lea         tmpq, [dstq+mstrideq*2]
    mova          m3, [tmpq+strideq*0]          ; p1
    mova          m4, [tmpq+strideq*1]          ; p0
    mova          m5, [tmpq+strideq*2]          ; q0
    mova          m6, [tmpq+stride3q]           ; q1
%else
    ; load 6-8 pixels, remainder (for wd=16) will be read inline
    lea         tmpq, [dstq+mstrideq*4]
    ; we load p3 later
%define %%p3mem [dstq+mstrideq*4]
 %if ARCH_X86_32
  %define m13 m0
  %define m14 m1
  %define m15 m2
 %endif
    mova         m13, [tmpq+strideq*1]
    mova          m3, [tmpq+strideq*2]
    mova          m4, [tmpq+stride3q]
    mova          m5, [dstq+strideq*0]
    mova          m6, [dstq+strideq*1]
    mova         m14, [dstq+strideq*2]
%if %1 != 6
    mova         m15, [dstq+stride3q]
%endif
 %if ARCH_X86_32
    mova     %%p2mem, m13
    mova     %%q2mem, m14
  %define m13 %%p2mem
  %define m14 %%q2mem
  %if %1 != 6
    mova     %%q3mem, m15
   %define m15 %%q3mem
  %endif
 %endif
%endif
%else ; %2 == h
    ; load lines
%if %1 == 4
    ; transpose 4x16
    movd          m7, [dstq+strideq*0-2]
    movd          m3, [dstq+strideq*1-2]
    movd          m4, [dstq+strideq*2-2]
    movd          m5, [dstq+stride3q -2]
    lea         tmpq, [dstq+strideq*4]
    punpcklbw     m7, m3
    punpcklbw     m4, m5
    movd          m3, [tmpq+strideq*0-2]
    movd          m1, [tmpq+strideq*1-2]
    movd          m5, [tmpq+strideq*2-2]
    movd          m6, [tmpq+stride3q -2]
    lea         tmpq, [tmpq+strideq*4]
    punpcklbw     m3, m1
    punpcklbw     m5, m6
    movd          m0, [tmpq+strideq*0-2]
    movd          m1, [tmpq+strideq*1-2]
    punpcklbw     m0, m1
    movd          m1, [tmpq+strideq*2-2]
    movd          m2, [tmpq+stride3q -2]
    punpcklbw     m1, m2
    punpcklqdq    m7, m0
    punpcklqdq    m4, m1
    lea         tmpq, [tmpq+strideq*4]
    movd          m0, [tmpq+strideq*0-2]
    movd          m1, [tmpq+strideq*1-2]
    punpcklbw     m0, m1
    movd          m1, [tmpq+strideq*2-2]
    movd          m2, [tmpq+stride3q -2]
    punpcklbw     m1, m2
    punpcklqdq    m3, m0
    punpcklqdq    m5, m1
    ; xm7: A0-1,B0-1,C0-1,D0-1,A8-9,B8-9,C8-9,D8-9
    ; xm3: A4-5,B4-5,C4-5,D4-5,A12-13,B12-13,C12-13,D12-13
    ; xm4: A2-3,B2-3,C2-3,D2-3,A10-11,B10-11,C10-11,D10-11
    ; xm5: A6-7,B6-7,C6-7,D6-7,A14-15,B14-15,C14-15,D14-15
    punpcklwd     m6, m7, m4
    punpckhwd     m7, m4
    punpcklwd     m4, m3, m5
    punpckhwd     m3, m5
    ; xm6: A0-3,B0-3,C0-3,D0-3
    ; xm7: A8-11,B8-11,C8-11,D8-11
    ; xm4: A4-7,B4-7,C4-7,D4-7
    ; xm3: A12-15,B12-15,C12-15,D12-15
    punpckldq     m5, m6, m4
    punpckhdq     m6, m4
    punpckldq     m4, m7, m3
    punpckhdq     m7, m3
    ; xm5: A0-7,B0-7
    ; xm6: C0-7,D0-7
    ; xm4: A8-15,B8-15
    ; xm7: C8-15,D8-15
    punpcklqdq    m3, m5, m4
    punpckhqdq    m5, m5, m4
    punpcklqdq    m4, m6, m7
    punpckhqdq    m6, m7
    ; xm3: A0-15
    ; xm5: B0-15
    ; xm4: C0-15
    ; xm6: D0-15
    SWAP           4, 5
%elif %1 == 6 || %1 == 8
    ; transpose 8x16
    movq          m7, [dstq+strideq*0-%1/2]
    movq          m3, [dstq+strideq*1-%1/2]
    movq          m4, [dstq+strideq*2-%1/2]
    movq          m5, [dstq+stride3q -%1/2]
    lea         tmpq, [dstq+strideq*8]
    punpcklbw     m7, m3
    punpcklbw     m4, m5
    movq          m3, [tmpq+strideq*0-%1/2]
    movq          m1, [tmpq+strideq*1-%1/2]
    movq          m5, [tmpq+strideq*2-%1/2]
    movq          m6, [tmpq+stride3q -%1/2]
    lea         tmpq, [dstq+strideq*4]
    punpcklbw     m3, m1
    punpcklbw     m5, m6
    movq          m6, [tmpq+strideq*0-%1/2]
    movq          m0, [tmpq+strideq*1-%1/2]
    movq          m1, [tmpq+strideq*2-%1/2]
    movq          m2, [tmpq+stride3q -%1/2]
    lea         tmpq, [tmpq+strideq*8]
    punpcklbw     m6, m0
    punpcklbw     m1, m2
    movq          m2, [tmpq+strideq*2-%1/2]
    movq          m0, [tmpq+stride3q -%1/2]
    punpcklbw     m2, m0
%if ARCH_X86_64
    SWAP         m15, m2
%else
 %define m15 [esp+3*16]
    mova         m15, m2
%endif
    movq          m0, [tmpq+strideq*0-%1/2]
    movq          m2, [tmpq+strideq*1-%1/2]
    punpcklbw     m0, m2
    ; xm7: A0-1,B0-1,C0-1,D0-1,E0-1,F0-1,G0-1,H0-1
    ; xm3: A8-9,B8-9,C8-9,D8-9,E8-9,F8-9,G8-9,H8-9
    ; xm4: A2-3,B2-3,C2-3,D2-3,E2-3,F2-3,G2-3,H2-3
    ; xm5: A10-11,B10-11,C10-11,D10-11,E10-11,F10-11,G10-11,H10-11
    ; xm6: A4-5,B4-5,C4-5,D4-5,E4-5,F4-5,G4-5,H4-5
    ; xm0: A12-13,B12-13,C12-13,D12-13,E12-13,F12-13,G12-13,H12-13
    ; xm1: A6-7,B6-7,C6-7,D6-7,E6-7,F6-7,G6-7,H6-7
    ; xm2: A14-15,B14-15,C14-15,D14-15,E14-15,F14-15,G14-15,H14-15
    punpcklwd     m2, m7, m4
    punpckhwd     m7, m4
    punpcklwd     m4, m3, m5
    punpckhwd     m3, m5
    punpcklwd     m5, m6, m1
    punpckhwd     m6, m1
    punpcklwd     m1, m0, m15
    punpckhwd     m0, m15
%if ARCH_X86_64
    SWAP         m15, m0
%else
    mova         m15, m0
%endif
    ; xm2: A0-3,B0-3,C0-3,D0-3
    ; xm7: E0-3,F0-3,G0-3,H0-3
    ; xm4: A8-11,B8-11,C8-11,D8-11
    ; xm3: E8-11,F8-11,G8-11,H8-11
    ; xm5: A4-7,B4-7,C4-7,D4-7
    ; xm6: E4-7,F4-7,G4-7,H4-7
    ; xm1: A12-15,B12-15,C12-15,D12-15
    ; xm0: E12-15,F12-15,G12-15,H12-15
    punpckldq     m0, m2, m5
    punpckhdq     m2, m5
    punpckldq     m5, m7, m6
%if %1 != 6
    punpckhdq     m7, m6
%endif
    punpckldq     m6, m4, m1
    punpckhdq     m4, m1
    punpckldq     m1, m3, m15
%if %1 != 6
    punpckhdq     m3, m15
 %if ARCH_X86_64
    SWAP         m15, m3
 %else
    mova         m15, m3
 %endif
%endif
    ; xm0: A0-7,B0-7
    ; xm2: C0-7,D0-7
    ; xm5: E0-7,F0-7
    ; xm7: G0-7,H0-7
    ; xm6: A8-15,B8-15
    ; xm4: C8-15,D8-15
    ; xm1: E8-15,F8-15
    ; xm3: G8-15,H8-15
    punpcklqdq    m3, m0, m6
    punpckhqdq    m0, m6
    punpckhqdq    m6, m2, m4
    punpcklqdq    m2, m4
    punpcklqdq    m4, m5, m1
    punpckhqdq    m5, m1
%if %1 == 8
    punpcklqdq    m1, m7, m15
    punpckhqdq    m7, m15
    ; xm3: A0-15
    ; xm0: B0-15
    ; xm2: C0-15
    ; xm6: D0-15
    ; xm4: E0-15
    ; xm5: F0-15
    ; xm1: G0-15
    ; xm7: H0-15
%if ARCH_X86_64
    SWAP          11, 3, 2
    SWAP          13, 0
    SWAP           6, 5, 4
    SWAP          14, 1
    SWAP          15, 7
    ; 3,0,2,6,4,5,1,7 -> 11,13,3,4,5,6,14,15
    mova [rsp+21*16], m11
 %define %%p3mem [rsp+21*16]
%else
 %define m11 [esp+26*16]
 %define m13 [esp+27*16]
 %define m14 [esp+28*16]
 %define m15 [esp+29*16]
    mova         m11, m3
    mova         m13, m0
    SWAP           3, 2
    SWAP           6, 5, 4
    mova         m14, m1
    mova         m15, m7
 %define %%p3mem [esp+26*16]
%endif
%else
 %if ARCH_X86_64
    SWAP          13, 3, 0
    SWAP          14, 5, 6, 4, 2
    ; 3,0,2,6,4,5 -> 13,3,4,5,6,14
 %else
  %define m13 %%p2mem
  %define m14 %%q2mem
    mova         m13, m3
    mova         m14, m5
    SWAP           3, 0
    SWAP           5, 6, 4, 2
    ; 0,2,6,4 -> 3,4,5,6
 %endif
%endif
%else
%if ARCH_X86_64
    mova [rsp+20*16], m12
%endif
    ; load and 16x16 transpose. We only use 14 pixels but we'll need the
    ; remainder at the end for the second transpose
%if ARCH_X86_32
 %xdefine m8  m0
 %xdefine m9  m1
 %xdefine m10 m2
 %xdefine m11 m3
 %xdefine m12 m4
 %xdefine m13 m5
 %xdefine m14 m6
 %xdefine m15 m7
    lea         tmpq, [dstq+strideq*8]
    movu          m8, [tmpq+strideq*0-8]
    movu          m9, [tmpq+strideq*1-8]
    movu         m10, [tmpq+strideq*2-8]
    movu         m11, [tmpq+stride3q -8]
    lea         tmpq, [tmpq+strideq*4]
    movu         m12, [tmpq+strideq*0-8]
    movu         m13, [tmpq+strideq*1-8]
    movu         m14, [tmpq+strideq*2-8]
    movu         m15, [tmpq+stride3q -8]
    mova [esp+ 8*16], m8
    mova [esp+ 9*16], m9
    mova [esp+10*16], m10
    mova [esp+11*16], m11
    mova [esp+12*16], m12
    mova [esp+13*16], m13
    mova [esp+14*16], m14
    mova [esp+15*16], m15
%endif
    movu          m0, [dstq+strideq*0-8]
    movu          m1, [dstq+strideq*1-8]
    movu          m2, [dstq+strideq*2-8]
    movu          m3, [dstq+stride3q -8]
    lea         tmpq, [dstq+strideq*4]
    movu          m4, [tmpq+strideq*0-8]
    movu          m5, [tmpq+strideq*1-8]
    movu          m6, [tmpq+strideq*2-8]
    movu          m7, [tmpq+stride3q -8]
    lea         tmpq, [tmpq+strideq*4]
%if ARCH_X86_64
    movu          m8, [tmpq+strideq*0-8]
    movu          m9, [tmpq+strideq*1-8]
    movu         m10, [tmpq+strideq*2-8]
    movu         m11, [tmpq+stride3q -8]
    lea         tmpq, [tmpq+strideq*4]
    movu         m12, [tmpq+strideq*0-8]
    movu         m13, [tmpq+strideq*1-8]
    movu         m14, [tmpq+strideq*2-8]
    movu         m15, [tmpq+stride3q -8]
%endif

%if ARCH_X86_64
    TRANSPOSE_16X16B 0, [rsp+11*16]
    mova [rsp+12*16], m1
    mova [rsp+13*16], m2
    mova [rsp+14*16], m3
    mova [rsp+15*16], m12
    mova [rsp+16*16], m13
    mova [rsp+17*16], m14
    mova [rsp+18*16], m15
    ; 4,5,6,7,8,9,10,11 -> 12,13,3,4,5,6,14,15
    SWAP          12, 4, 7
    SWAP          13, 5, 8
    SWAP           3, 6, 9
    SWAP          10, 14
    SWAP          11, 15
    mova [rsp+21*16], m12
 %define %%p3mem [rsp+21*16]
    mova         m12, [rsp+20*16]
%else
    TRANSPOSE_16X16B 0, [esp+16*16]
 %define %%p3mem [esp+26*16]
 %define m11 %%p3mem
 %define m13 %%p2mem
 %define m14 %%q2mem
 %define m15 %%q3mem
%endif
%endif ; if 4 elif 6 or 8 else 16
%endif ; if v else h

    ; load L/E/I/H
%if ARCH_X86_32
    mov    l_strideq, l_stridem
%endif
%ifidn %2, v
    movu          m1, [lq]
    movu          m0, [lq+l_strideq]
%else
 %if ARCH_X86_32
    lea   l_stride3q, [l_strideq*3]
 %endif
    movq         xm1, [lq]
    movq         xm2, [lq+l_strideq*2]
    movhps       xm1, [lq+l_strideq]
    movhps       xm2, [lq+l_stride3q]
    shufps        m0, m1, m2, q3131
    shufps        m1, m2, q2020
 %if ARCH_X86_32
    lea     stride3q, [strideq*3]
 %endif
%endif

%if ARCH_X86_32
 %ifidn %2, v
    mov         lutd, lutm
 %endif
%endif
    pxor          m2, m2
    pcmpeqb       m7, m2, m0
    pand          m1, m7
    por           m0, m1                        ; l[x][] ? l[x][] : l[x-stride][]
    pshufb        m0, [PIC_sym(pb_4x0_4x4_4x8_4x12)] ; l[x][1]
    pcmpeqb       m2, m0                        ; !L
    psrlq         m7, m0, [lutq+128]
    pand          m7, [PIC_sym(pb_63)]
    pminub        m7, minlvl
    pmaxub        m7, [PIC_sym(pb_1)]           ; I
    pand          m1, m0, [PIC_sym(pb_240)]
    psrlq         m1, 4                         ; H
    paddb         m0, [PIC_sym(pb_2)]
    paddb         m0, m0
    paddb         m0, m7                        ; E
    pxor          m1, [PIC_sym(pb_128)]
    pxor          m7, [PIC_sym(pb_128)]
    pxor          m0, [PIC_sym(pb_128)]
    SWAP           2, 7

%if ARCH_X86_64
    SWAP           0, 8
    SWAP           2, 10
%else
 %ifidn %2, v
    mov     mstrideq, strideq
    neg     mstrideq
  %if %1 == 4
    lea         tmpq, [dstq+mstrideq*2]
  %elif %1 == 6 || %1 == 8
    lea         tmpq, [dstq+mstrideq*4]
  %endif
 %endif
    mova  [esp+3*16], m0
    mova  [esp+4*16], m2
%endif

    ABSSUB        m0, m3, m4, m2                ; abs(p1-p0)
    pmaxub        m0, m7
    ABSSUB        m2, m5, m6, m7                ; abs(q1-q0)
    pmaxub        m0, m2
%if %1 == 4
    pxor          m0, [PIC_sym(pb_128)]
    pcmpgtb       m7, m0, m1                    ; hev
 %if ARCH_X86_64
    SWAP           7, 11
 %else
    mova  [esp+5*16], m7
 %endif
%else
    pxor          m7, m0, [PIC_sym(pb_128)]
    pcmpgtb       m7, m1                        ; hev
%if ARCH_X86_64
    SWAP           7, 11
%else
    mova  [esp+5*16], m7
%endif

%if %1 == 6
    ABSSUB        m1, m13, m4, m7               ; abs(p2-p0)
    pmaxub        m1, m0
%else
    mova          m2, %%p3mem
    ABSSUB        m1, m2, m4, m7                ; abs(p3-p0)
    pmaxub        m1, m0
    ABSSUB        m7, m13, m4, m2               ; abs(p2-p0)
    pmaxub        m1, m7
%endif
    ABSSUB        m7, m5, m14, m2               ; abs(p2-p0)
    pmaxub        m1, m7
%if %1 != 6
    ABSSUB        m7, m5, m15, m2               ; abs(q3-q0)
    pmaxub        m1, m7
%endif
    pxor          m1, [PIC_sym(pb_128)]
    pcmpgtb       m1, [PIC_sym(pb_129)]         ; !flat8in
%if ARCH_X86_64
    SWAP           1, 9
%else
    mova  [esp+6*16], m1
%endif

%if %1 == 6
    ABSSUB        m7, m13, m3, m1               ; abs(p2-p1)
%else
    mova          m2, %%p3mem
    ABSSUB        m7, m2, m13, m1               ; abs(p3-p2)
    ABSSUB        m2, m13, m3, m1               ; abs(p2-p1)
    pmaxub        m7, m2
    ABSSUB        m2, m14, m15, m1              ; abs(q3-q2)
    pmaxub        m7, m2
%endif
    ABSSUB        m2, m14, m6,  m1              ; abs(q2-q1)
    pmaxub        m7, m2
%if ARCH_X86_32
 %define m12 m1
    mova         m12, maskmem
%endif
    pand          m2, m12, mask1
    pcmpeqd       m2, m12
    pand          m7, m2                        ; only apply fm-wide to wd>4 blocks
    pmaxub        m0, m7

    pxor          m0, [PIC_sym(pb_128)]
%endif ; %if %1 == 4 else
%if ARCH_X86_64
    SWAP           2, 10
    pcmpgtb       m0, m2
%else
    pcmpgtb       m0, [esp+4*16]
%endif

    ABSSUB        m1, m3, m6, m7                ; abs(p1-q1)
    ABSSUB        m7, m4, m5, m2                ; abs(p0-q0)
    paddusb       m7, m7
    pand          m1, [PIC_sym(pb_254)]
    psrlq         m1, 1
    paddusb       m1, m7                        ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    pxor          m1, [PIC_sym(pb_128)]
%if ARCH_X86_64
    pcmpgtb       m1, m8                        ; abs(p0-q0)*2+(abs(p1-q1)>>1) > E
%else
    pcmpgtb       m1, [esp+3*16]
%endif
    por           m0, m1

%if %1 == 16
%if ARCH_X86_64
    SWAP           0, 8
%else
    mova  [esp+3*16], m0
%endif
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]
%else
    mova          m0, [rsp+12*16]
%endif
    ABSSUB        m1, m0, m4, m2
%ifidn %2, v
    mova          m0, [tmpq+strideq*2]
%else
    mova          m0, [rsp+13*16]
%endif
    ABSSUB        m2, m0, m4, m7
    pmaxub        m1, m2
%ifidn %2, v
    mova          m0, [tmpq+stride3q]
%else
    mova          m0, [rsp+14*16]
%endif
    ABSSUB        m2, m0, m4, m7
    pmaxub        m1, m2
%ifidn %2, v
    lea         tmpq, [dstq+strideq*4]
    mova          m0, [tmpq+strideq*0]
%else
    mova          m0, [rsp+15*16]
%endif
    ABSSUB        m2, m0, m5, m7
    pmaxub        m1, m2
%ifidn %2, v
    mova          m0, [tmpq+strideq*1]
%else
    mova          m0, [rsp+16*16]
%endif
    ABSSUB        m2, m0, m5, m7
    pmaxub        m1, m2
%ifidn %2, v
    mova          m0, [tmpq+strideq*2]
%else
    mova          m0, [rsp+17*16]
%endif
    ABSSUB        m2, m0, m5, m7
    pmaxub        m1, m2
    pxor          m1, [PIC_sym(pb_128)]
    pcmpgtb       m1, [PIC_sym(pb_129)]         ; !flat8out
%if ARCH_X86_64
    por           m1, m9                        ; !flat8in | !flat8out
%else
    por           m1, [esp+6*16]
 %define m12 m7
    mova         m12, maskmem
%endif
    pand          m2, m12, mask2
    pcmpeqd       m2, m12
    pandn         m1, m2                        ; flat16
%if ARCH_X86_64
    pandn         m2, m8, m1                    ; flat16 & fm
%else
    pandn         m2, [esp+3*16], m1            ; flat16 & fm
    mova %%flat16mem, m2
%endif
    SWAP           1, 2

    pand          m2, m12, mask1
    pcmpeqd       m2, m12
%if ARCH_X86_64
    pandn         m9, m2                    ; flat8in
    pandn         m2, m8, m9
    SWAP           2, 9
%else
    pandn         m0, [esp+6*16], m2
    pandn         m2, [esp+3*16], m0
    mova  [esp+6*16], m2
%endif
    pand          m2, m12, mask0
    pcmpeqd       m2, m12
%if ARCH_X86_64
    pandn         m8, m2
    pandn         m2, m9, m8                    ; fm & !flat8 & !flat16
    SWAP           2, 8
    pandn         m2, m1, m9                    ; flat8 & !flat16
    SWAP           2, 9
    SWAP           0, 8
    SWAP           1, 10
%else
    pandn         m0, [esp+3*16], m2
    pandn         m2, [esp+6*16], m0
    SWAP           2, 0
    pandn         m2, m1, [esp+6*16]
    mova  %%flat8mem, m2
%endif
%elif %1 != 4
 %if ARCH_X86_64
    SWAP           1, 9
 %else
  %define m12 m7
    mova         m12, maskmem
    mova          m1, [esp+6*16]
 %endif
    pand          m2, m12, mask1
    pcmpeqd       m2, m12
    pandn         m1, m2
    pandn         m2, m0, m1                    ; flat8 & fm
    pand          m1, m12, mask0
    pcmpeqd       m1, m12
    pandn         m0, m1
    pandn         m1, m2, m0                    ; fm & !flat8
    SWAP           1, 2, 0
 %if ARCH_X86_64
    SWAP           1, 9
 %else
    mova  %%flat8mem, m1
 %endif
%else
%if ARCH_X86_32
 %define m12 m1
    mova         m12, maskmem
%endif
    pand          m2, m12, mask0
    pcmpeqd       m2, m12
    pandn         m0, m2                        ; fm
%endif

    ; short filter

    mova          m1, [PIC_sym(pb_128)]
%if ARCH_X86_64
    SWAP           7, 11
%else
    mova          m7, [esp+5*16]
%endif
    pxor          m3, m1
    pxor          m6, m1
    pxor          m4, m1
    pxor          m5, m1
    psubsb        m1, m3, m6                    ; iclip_diff(p1-q1)
    pand          m1, m7                        ; f=iclip_diff(p1-q1)&hev
    psubsb        m2, m5, m4
    paddsb        m1, m2
    paddsb        m1, m2
    paddsb        m1, m2                        ; f=iclip_diff(3*(q0-p0)+f)
    mova          m2, [PIC_sym(pb_16)]
    pand          m0, m1                        ; f&=fm
    paddsb        m1, m0, [PIC_sym(pb_3)]
    paddsb        m0, [PIC_sym(pb_4)]
    pand          m1, [PIC_sym(pb_248)]
    pand          m0, [PIC_sym(pb_248)]
    psrlq         m1, 3
    psrlq         m0, 3
    pxor          m1, m2
    pxor          m0, m2
    psubb         m1, m2                        ; f2
    psubb         m0, m2                        ; f1
    mova          m2, [PIC_sym(pb_128)]
    paddsb        m4, m1
    psubsb        m5, m0
    pxor          m4, m2
    pxor          m5, m2

    pxor          m0, m2
    pxor          m1, m1
    pavgb         m0, m1                        ; f=(f1+1)>>1
    psubb         m0, [PIC_sym(pb_64)]
    pandn         m7, m0                        ; f&=!hev
    paddsb        m3, m7
    psubsb        m6, m7
    pxor          m3, m2
    pxor          m6, m2

%if %1 == 16
    ; flat16 filter
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]          ; p6
    mova          m2, [tmpq+strideq*2]          ; p5
    mova          m7, [tmpq+stride3q]           ; p4
%else
    mova          m0, [rsp+12*16]
    mova          m2, [rsp+13*16]
    mova          m7, [rsp+14*16]
%endif

%if ARCH_X86_64
    SWAP           1, 10
    mova  %%flat8mem, m9
    mova     %%q2mem, m14
    mova     %%q3mem, m15
    SWAP           0, 8
    SWAP           1, 9
%else
 %ifidn %2, v
    mova [esp+17*16], m0
    mova [esp+19*16], m3
    mova [esp+21*16], m4
    mova [esp+22*16], m5
    mova [esp+23*16], m6
  %xdefine m11 m3
  %xdefine m14 m4
  %xdefine m15 m5
  %xdefine m10 m6
  %define m13 %%p2mem
  %define m8  [esp+17*16]
  %define m9  %%flat16mem
  %define m3  [esp+19*16]
  %define m4  [esp+21*16]
  %define m5  [esp+22*16]
  %define m6  [esp+23*16]
 %else
    mova [esp+31*16], m0
    mova [esp+32*16], m3
    mova [esp+33*16], m4
    mova [esp+34*16], m5
    mova [esp+35*16], m6
  %xdefine m11 m3
  %xdefine m14 m4
  %xdefine m15 m5
  %xdefine m10 m6
  %define m13 %%p2mem
  %define m8  [esp+31*16]
  %define m9  %%flat16mem
  %define m3  [esp+32*16]
  %define m4  [esp+33*16]
  %define m5  [esp+34*16]
  %define m6  [esp+35*16]
 %endif
%endif

    ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0 [p5/p4/p2/p1/p0/q0][p6/p3] A
    ; write -6
    mova         m11, %%p3mem
%if ARCH_X86_64
    punpcklbw    m14, m8, m11
    punpckhbw    m15, m8, m11
%else
    punpcklbw    m14, m0, m11
    punpckhbw    m15, m0, m11
%endif
%ifidn %2, v
    mova  [rsp+5*16], m11
%endif
    pmaddubsw    m10, m14, [PIC_sym(pb_7_1)]
    pmaddubsw    m11, m15, [PIC_sym(pb_7_1)]    ; p6*7+p3
    punpcklbw     m0, m2, m7
    punpckhbw     m1, m2, m7
    pmaddubsw     m0, [PIC_sym(pb_2)]
    pmaddubsw     m1, [PIC_sym(pb_2)]
    paddw        m10, m0
    paddw        m11, m1                        ; p6*7+p5*2+p4*2+p3
    punpcklbw     m0, m13, m3
    punpckhbw     m1, m13, m3
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw     m1, [PIC_sym(pb_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p6*7+p5*2+p4*2+p3+p2+p1
    punpcklbw     m0, m4, m5
    punpckhbw     m1, m4, m5
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw     m1, [PIC_sym(pb_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m2
    por           m0, m1
%ifidn %2, v
    mova [tmpq+strideq*2], m0                   ; p5
%else
    mova [rsp+13*16], m0
%endif

    ; sub p6*2, add p3/q1 [reuse p6/p3 from A][-p6,+q1|save] B
    ; write -5
    pmaddubsw    m14, [PIC_sym(pb_m1_1)]
    pmaddubsw    m15, [PIC_sym(pb_m1_1)]
    paddw        m10, m14
    paddw        m11, m15                       ; p6*6+p5*2+p4*2+p3*2+p2+p1+p0+q0
    punpcklbw     m0, m8, m6
    punpckhbw     m1, m8, m6
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    mova  [rsp+3*16], m0
    mova  [rsp+4*16], m1
    paddw        m10, m0
    paddw        m11, m1                        ; p6*5+p5*2+p4*2+p3*2+p2+p1+p0+q0+q1
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m7
    por           m0, m1
%ifidn %2, v
    mova [tmpq+stride3q], m0                    ; p4
%else
    mova [rsp+14*16], m0
%endif

    ; sub p6/p5, add p2/q2 [-p6,+p2][-p5,+q2|save] C
    ; write -4
    mova         m14, %%q2mem
    punpcklbw     m0, m8, m13
    punpckhbw     m1, m8, m13
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p6*4+p5*2+p4*2+p3*2+p2*2+p1+p0+q0+q1
    punpcklbw     m0, m2, m14
    punpckhbw     m2, m14
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m2, [PIC_sym(pb_m1_1)]
    mova  [rsp+1*16], m0
    paddw        m10, m0
    paddw        m11, m2                        ; p6*4+p5+p4*2+p3*2+p2*2+p1+p0+q0+q1+q2
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, %%p3mem
    por           m0, m1
%ifidn %2, v
    mova [tmpq+strideq*4], m0                   ; p3
%else
    mova [rsp+19*16], m0
%endif

    ; sub p6/p4, add p1/q3 [-p6,+p1][-p4,+q3|save] D
    ; write -3
    mova         m15, %%q3mem
    punpcklbw     m0, m8, m3
    punpckhbw     m1, m8, m3
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p6*3+p5+p4*2+p3*2+p2*2+p1*2+p0+q0+q1+q2
    punpcklbw     m0, m7, m15
    punpckhbw     m7, m15
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m7, [PIC_sym(pb_m1_1)]
    mova  [rsp+2*16], m0
%if ARCH_X86_32
 %ifidn %2, v
    mova [esp+24*16], m7
 %else
    mova [esp+36*16], m7
 %endif
%endif
    paddw        m10, m0
    paddw        m11, m7                        ; p6*3+p5+p4+p3*2+p2*2+p1*2+p0+q0+q1+q2+q3
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m13
    por           m0, m1
    mova  [rsp+6*16], m0                        ; don't clobber p2/m13 since we need it in F

    ; sub p6/p3, add p0/q4 [-p6,+p0][-p3,+q4|save] E
    ; write -2
    punpcklbw     m0, m8, m4
    punpckhbw     m1, m8, m4
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p6*2+p5+p4+p3*2+p2*2+p1*2+p0*2+q0+q1+q2+q3
%if ARCH_X86_64
    SWAP           7, 8
%endif
%ifidn %2, v
    mova          m1, [dstq+strideq*4]          ; q4
    mova          m7, [rsp+5*16]                ; (pre-filter) p3
%else
    mova          m1, [rsp+15*16]
    mova          m7, %%p3mem                   ; (pre-filter) p3
%endif
    punpcklbw     m0, m1, m7
    punpckhbw     m1, m1, m7
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    mova  [rsp+7*16], m0
    mova  [rsp+5*16], m1
    psubw        m10, m0
    psubw        m11, m1                        ; p6*2+p5+p4+p3+p2*2+p1*2+p0*2+q0+q1+q2+q3+q4
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m3
    por           m0, m1
    mova  [rsp+8*16], m0                        ; don't clobber p1/m3 since we need it in G

    ; sub p6/p2, add q0/q5 [-p6,+q0][-p2,+q5|save] F
    ; write -1
%ifidn %2, v
    mova          m7, [tmpq+strideq*1]          ; p6
    lea         tmpq, [dstq+strideq*4]
    mova          m1, [tmpq+strideq*1]          ; q5
%else
    mova          m7, [rsp+12*16]               ; p6
    mova          m1, [rsp+16*16]
%endif
    punpcklbw     m0, m7, m5
    punpckhbw     m7, m5
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m7, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m7                        ; p6+p5+p4+p3+p2*2+p1*2+p0*2+q0*2+q1+q2+q3+q4
    punpcklbw     m7, m13, m1
    pmaddubsw     m7, [PIC_sym(pb_m1_1)]
    mova  [rsp+9*16], m7
    paddw        m10, m7
%if ARCH_X86_64
    punpckhbw    m13, m1
    mova          m1, [rsp+6*16]
    SWAP           1, 13
%else
    punpckhbw     m7, m13, m1
    mova          m1, [esp+6*16]
    mova         m13, m1
    SWAP           1, 7
%endif
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    mova [rsp+10*16], m1
    paddw        m11, m1                        ; p6+p5+p4+p3+p2+p1*2+p0*2+q0*2+q1+q2+q3+q4+q5
    pmulhrsw      m7, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m0, m11, [PIC_sym(pw_2048)]
    packuswb      m7, m0
    pand          m7, m9
    pandn         m0, m9, m4
    por           m7, m0
    mova  [rsp+6*16], m7                        ; don't clobber p0/m4 since we need it in H

    ; sub p6/p1, add q1/q6 [reuse -p6,+q1 from B][-p1,+q6|save] G
    ; write +0
%ifidn %2, v
    mova          m7, [tmpq+strideq*2]          ; q6
%else
    mova          m7, [rsp+17*16]
%endif
    paddw        m10, [rsp+3*16]
    paddw        m11, [rsp+4*16]                ; p5+p4+p3+p2+p1*2+p0*2+q0*2+q1*2+q2+q3+q4+q5
    punpcklbw     m0, m3, m7
    punpckhbw     m1, m3, m7
%if ARCH_X86_64
    mova          m3, [rsp+8*16]
%endif
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    mova  [rsp+3*16], m0
    mova  [rsp+4*16], m1
    paddw        m10, m0
    paddw        m11, m1                        ; p5+p4+p3+p2+p1+p0*2+q0*2+q1*2+q2+q3+q4+q5+q6
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m5
    por           m0, m1
%if ARCH_X86_32
    mova          m1, [esp+8*16]
    mova          m3, m1
%endif
    mova  [rsp+8*16], m0                        ; don't clobber q0/m5 since we need it in I

    ; sub p5/p0, add q2/q6 [reuse -p5,+q2 from C][-p0,+q6] H
    ; write +1
    paddw        m10, [rsp+1*16]
    paddw        m11, m2                        ; p4+p3+p2+p1+p0*2+q0*2+q1*2+q2*2+q3+q4+q5+q6
    punpcklbw     m0, m4, m7
    punpckhbw     m2, m4, m7
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m2, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m2                        ; p4+p3+p2+p1+p0+q0*2+q1*2+q2*2+q3+q4+q5+q6*2
%if ARCH_X86_64
    mova          m4, [rsp+6*16]
%else
 %define m4 [esp+6*16]
%endif
    pmulhrsw      m2, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m2, m1
    pand          m2, m9
    pandn         m1, m9, m6
    por           m2, m1                        ; don't clobber q1/m6 since we need it in K

    ; sub p4/q0, add q3/q6 [reuse -p4,+q3 from D][-q0,+q6] I
    ; write +2
    paddw        m10, [rsp+2*16]
%if ARCH_X86_64
    SWAP           7, 8
    paddw        m11, m7
%else
    mova          m8, m7
 %ifidn %2, v
    paddw        m11, [esp+24*16]               ; p3+p2+p1+p0+q0*2+q1*2+q2*2+q3*2+q4+q5+q6*2
 %else
    paddw        m11, [esp+36*16]               ; p3+p2+p1+p0+q0*2+q1*2+q2*2+q3*2+q4+q5+q6*2
 %endif
%endif
    punpcklbw     m0, m5, m8
    punpckhbw     m1, m5, m8
%if ARCH_X86_64
    mova          m5, [rsp+8*16]
%else
 %define m5 [esp+8*16]
%endif
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p3+p2+p1+p0+q0+q1*2+q2*2+q3*2+q4+q5+q6*3
    pmulhrsw      m7, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m7, m1
    pand          m7, m9
    pandn         m1, m9, m14
    por           m7, m1                        ; don't clobber q2/m14 since we need it in K

    ; sub p3/q1, add q4/q6 [reuse -p3,+q4 from E][-q1,+q6] J
    ; write +3
    psubw        m10, [rsp+7*16]
    psubw        m11, [rsp+5*16]                ; p2+p1+p0+q0+q1*2+q2*2+q3*2+q4*2+q5+q6*3
    punpcklbw     m0, m6, m8
    punpckhbw     m1, m6, m8
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p2+p1+p0+q0+q1+q2*2+q3*2+q4*2+q5+q6*4
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m15
    por           m0, m1
%ifidn %2, v
    mova [tmpq+mstrideq], m0                    ; q3
%else
    mova [rsp+20*16], m0
%endif

    ; sub p2/q2, add q5/q6 [reuse -p2,+q5 from F][-q2,+q6] K
    ; write +4
    paddw        m10, [rsp+ 9*16]
    paddw        m11, [rsp+10*16]               ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
    punpcklbw     m0, m14, m8
    punpckhbw     m1, m14, m8
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
    pmulhrsw      m0, m10, [PIC_sym(pw_2048)]
    pmulhrsw      m1, m11, [PIC_sym(pw_2048)]
    packuswb      m0, m1
    pand          m0, m9
%ifidn %2, v
    pandn         m1, m9, [tmpq+strideq*0]
%else
    pandn         m1, m9, [rsp+15*16]
%endif
    por           m0, m1
%ifidn %2, v
    mova [tmpq+strideq*0], m0                    ; q4
%else
    mova [rsp+15*16], m0
%endif

    ; sub p1/q3, add q6*2 [reuse -p1,+q6 from G][-q3,+q6] L
    ; write +5
    paddw        m10, [rsp+3*16]
    paddw        m11, [rsp+4*16]                ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
    punpcklbw     m0, m15, m8
    punpckhbw     m1, m15, m8
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw        m10, m0
    paddw        m11, m1                        ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
    pmulhrsw     m10, [PIC_sym(pw_2048)]
    pmulhrsw     m11, [PIC_sym(pw_2048)]
    packuswb     m10, m11
    pand         m10, m9
%ifidn %2, v
    pandn        m11, m9, [tmpq+strideq*1]
%else
    pandn        m11, m9, [rsp+16*16]
%endif
    por          m10, m11
%ifidn %2, v
    mova [tmpq+strideq*1], m10                  ; q5
%else
    mova [rsp+16*16], m10
%endif

%if ARCH_X86_64
    SWAP           0, 8
    SWAP           1, 9
    SWAP          14, 7
%else
 %xdefine m3 m11
 %xdefine m4 m14
 %xdefine m5 m15
 %xdefine m6 m10
    mova     %%q2mem, m7
 %ifidn %2, v
    mova          m3, [esp+19*16]
 %else
    mova          m3, [esp+32*16]
 %endif
    mova          m4, [esp+ 6*16]
    mova          m5, [esp+ 8*16]
%endif
    SWAP          m6, m2

%if ARCH_X86_64
    mova          m9, %%flat8mem
%endif
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*4]
%endif
%endif ; if %1 == 16
%if %1 >= 8
    ; flat8 filter
%if ARCH_X86_32
 %define m9  %%flat8mem
 %define m11 m1
 %define m13 %%p2mem
 %define m14 %%q2mem
 %define m15 %%q3mem
%endif
    mova         m11, %%p3mem
    punpcklbw     m0, m11, m3
    punpcklbw     m7, m13, m4
    pmaddubsw     m2, m0, [PIC_sym(pb_3_1)] ; 3 * p3 + p1
    pmaddubsw     m7, [PIC_sym(pb_2_1)]
    paddw         m2, m7                    ; 3 * p3 + 2 * p2 + p1 + p0
    punpcklbw     m7, m5, [PIC_sym(pb_4)]
    pmaddubsw     m7, [PIC_sym(pb_1)]
    paddw         m2, m7                    ; 3 * p3 + 2 * p2 + p1 + p0 + q0 + 4
    punpckhbw     m1, m11, m3
    pmaddubsw     m7, m1, [PIC_sym(pb_3_1)] ; 3 * p3 + p1
    punpckhbw     m0, m13, m4
    pmaddubsw     m0, [PIC_sym(pb_2_1)]
    paddw         m7, m0                    ; 3 * p3 + 2 * p2 + p1 + p0
    punpckhbw     m0, m5, [PIC_sym(pb_4)]
    pmaddubsw     m0, [PIC_sym(pb_1)]
    paddw         m7, m0                    ; 3 * p3 + 2 * p2 + p1 + p0 + q0 + 4
    psrlw         m0, m2, 3
    psrlw         m1, m7, 3
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m13
    por           m0, m1                    ; p2
%ifidn %2, v
    mova [tmpq+strideq*1], m0
%else
 %if ARCH_X86_64
    SWAP           0, 10
 %else
    mova  [esp+2*16], m0
 %endif
%endif

%if ARCH_X86_32
    mova         m11, %%p3mem
%endif
    punpcklbw     m0, m11, m3
    punpckhbw     m1, m11, m3
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw         m2, m0
    paddw         m7, m1
    punpcklbw     m0, m13, m6
    punpckhbw     m1, m13, m6
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw         m2, m0
    paddw         m7, m1            ; 2 * p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4
    psrlw         m0, m2, 3
    psrlw         m1, m7, 3
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m3
    por           m0, m1            ; p1
%ifidn %2, v
    mova [tmpq+strideq*2], m0
%else
    mova  [rsp+0*16], m0
%endif

%if ARCH_X86_32
    mova         m11, %%p3mem
%endif
    punpcklbw     m0, m11, m3
    punpckhbw     m1, m11, m3
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw     m1, [PIC_sym(pb_1)]
    psubw         m2, m0
    psubw         m7, m1
    punpcklbw     m0, m4, m14
    punpckhbw     m1, m4, m14
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw     m1, [PIC_sym(pb_1)]
    paddw         m2, m0
    paddw         m7, m1            ; p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4
    psrlw         m0, m2, 3
    psrlw         m1, m7, 3
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m4
    por           m0, m1            ; p0
%ifidn %2, v
    mova [tmpq+stride3q], m0
%else
    mova  [rsp+1*16], m0
%endif

    punpcklbw     m0, m5, m15
    punpckhbw     m1, m5, m15
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw     m1, [PIC_sym(pb_1)]
    paddw         m2, m0
    paddw         m7, m1
%if ARCH_X86_32
    mova         m11, %%p3mem
%endif
    punpcklbw     m0, m11, m4
    punpckhbw    m11, m11, m4
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw    m11, [PIC_sym(pb_1)]
    psubw         m2, m0
    psubw         m7, m11           ; p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4
    psrlw         m0, m2, 3
    psrlw        m11, m7, 3
    packuswb      m0, m11
    pand          m0, m9
    pandn        m11, m9, m5
    por          m11, m0            ; q0
%ifidn %2, v
    mova [dstq+strideq*0], m11
%elif ARCH_X86_32
    mova  [esp+8*16], m11
%endif

    punpcklbw     m0, m5, m15
    punpckhbw     m1, m5, m15
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw         m2, m0
    paddw         m7, m1
    punpcklbw     m0, m13, m6
    punpckhbw     m1, m13, m6
    pmaddubsw     m0, [PIC_sym(pb_m1_1)]
    pmaddubsw     m1, [PIC_sym(pb_m1_1)]
    paddw         m2, m0
    paddw         m7, m1            ; p1 + p0 + q0 + 2 * q1 + q2 + 2 * q3 + 4
    psrlw         m0, m2, 3
    psrlw         m1, m7, 3
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m6
    por           m0, m1            ; q1
%ifidn %2, v
    mova [dstq+strideq*1], m0
%else
 %if ARCH_X86_64
    SWAP           0, 13
 %else
    mova  [esp+9*16], m0
 %endif
%endif

    punpcklbw     m0, m3, m6
    punpckhbw     m1, m3, m6
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw     m1, [PIC_sym(pb_1)]
    psubw         m2, m0
    psubw         m7, m1
    punpcklbw     m0, m14, m15
    punpckhbw     m1, m14, m15
    pmaddubsw     m0, [PIC_sym(pb_1)]
    pmaddubsw     m1, [PIC_sym(pb_1)]
    paddw         m2, m0
    paddw         m7, m1            ; p0 + q0 + q1 + q2 + 2 * q2 + 3 * q3 + 4
    psrlw         m2, 3
    psrlw         m7, 3
    packuswb      m2, m7
    pand          m2, m9
    pandn         m7, m9, m14
    por           m2, m7            ; q2
%ifidn %2, v
    mova [dstq+strideq*2], m2
%else
    mova          m0, [rsp+0*16]
%if %1 == 8
    mova          m1, [rsp+1*16]
    mova          m4, %%p3mem

%if ARCH_X86_32
 %define m10 [esp+2*16]
 %define m11 [esp+8*16]
 %define m13 [esp+9*16]
%endif

    ; 16x8 transpose
    punpcklbw     m3, m4, m10
    punpckhbw     m4, m10
    punpcklbw     m5, m0, m1
    punpckhbw     m0, m1
    punpcklbw     m1, m11, m13
    punpckhbw     m6, m11, m13
    punpcklbw     m7, m2, m15
    punpckhbw     m2, m15
%if ARCH_X86_64
    SWAP           2, 15
%else
    mova         m15, m2
%endif

    punpcklwd     m2, m3, m5
    punpckhwd     m3, m5
    punpcklwd     m5, m4, m0
    punpckhwd     m4, m0
    punpcklwd     m0, m1, m7
    punpckhwd     m1, m7
    punpcklwd     m7, m6, m15
    punpckhwd     m6, m15
%if ARCH_X86_64
    SWAP           6, 15
%else
    mova         m15, m6
%endif

    punpckldq     m6, m2, m0
    punpckhdq     m2, m0
    punpckldq     m0, m3, m1
    punpckhdq     m3, m1
    punpckldq     m1, m5, m7
    punpckhdq     m5, m7
    punpckldq     m7, m4, m15
    punpckhdq     m4, m15

    ; write 8x16
    movq   [dstq+strideq*0-4], xm6
    movhps [dstq+strideq*1-4], xm6
    movq   [dstq+strideq*2-4], xm2
    movhps [dstq+stride3q -4], xm2
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm0
    movhps [dstq+strideq*1-4], xm0
    movq   [dstq+strideq*2-4], xm3
    movhps [dstq+stride3q -4], xm3
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm1
    movhps [dstq+strideq*1-4], xm1
    movq   [dstq+strideq*2-4], xm5
    movhps [dstq+stride3q -4], xm5
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm7
    movhps [dstq+strideq*1-4], xm7
    movq   [dstq+strideq*2-4], xm4
    movhps [dstq+stride3q -4], xm4
    lea         dstq, [dstq+strideq*4]
%else
    ; 16x16 transpose and store
    SWAP           6, 0
    SWAP           7, 1
 %if ARCH_X86_64
    SWAP           5, 10, 2
    SWAP           8, 11
    SWAP           9, 13
    mova [rsp+21*16], m12
 %else
    mova [esp+10*16], m2
  %xdefine m8  m0
  %xdefine m9  m1
  %xdefine m10 m2
  %xdefine m11 m3
  %xdefine m12 m4
  %xdefine m13 m5
  %xdefine m14 m6
  %xdefine m15 m7
 %endif
    mova          m0, [rsp+11*16]
    mova          m1, [rsp+12*16]
    mova          m2, [rsp+13*16]
    mova          m3, [rsp+14*16]
    mova          m4, [rsp+19*16]
%if ARCH_X86_64
    mova          m7, [rsp+ 1*16]
    mova         m11, [rsp+20*16]
    mova         m12, [rsp+15*16]
    mova         m13, [rsp+16*16]
    mova         m14, [rsp+17*16]
    TRANSPOSE_16X16B 1, [rsp+18*16]
%else
    mova          m5, [esp+ 2*16]
    TRANSPOSE_16X16B 1, [esp+32*16]
    mov         tmpq, dstq
    lea         dstq, [dstq+strideq*8]
%endif
    movu [dstq+strideq*0-8], xm0
    movu [dstq+strideq*1-8], xm1
    movu [dstq+strideq*2-8], xm2
    movu [dstq+stride3q -8], xm3
    lea         dstq, [dstq+strideq*4]
    movu [dstq+strideq*0-8], xm4
    movu [dstq+strideq*1-8], xm5
    movu [dstq+strideq*2-8], xm6
    movu [dstq+stride3q -8], xm7
%if ARCH_X86_64
    lea         dstq, [dstq+strideq*4]
%else
  %xdefine m8  m0
  %xdefine m9  m1
  %xdefine m10 m2
  %xdefine m11 m3
  %xdefine m12 m4
  %xdefine m13 m5
  %xdefine m14 m6
  %xdefine m15 m7
    mova          m8, [esp+11*16]
    mova          m9, [esp+12*16]
    mova         m10, [esp+13*16]
    mova         m11, [esp+14*16]
    mova         m12, [esp+26*16]
    mova         m13, [esp+27*16]
    mova         m14, [esp+ 0*16]
    mova         m15, [esp+ 1*16]
    mov         dstq, tmpq
%endif
    movu [dstq+strideq*0-8], xm8
    movu [dstq+strideq*1-8], xm9
    movu [dstq+strideq*2-8], xm10
    movu [dstq+stride3q -8], xm11
    lea         dstq, [dstq+strideq*4]
    movu [dstq+strideq*0-8], xm12
    movu [dstq+strideq*1-8], xm13
    movu [dstq+strideq*2-8], xm14
    movu [dstq+stride3q -8], xm15
    lea         dstq, [dstq+strideq*4]
%if ARCH_X86_32
    lea         dstq, [dstq+strideq*8]
%else
    mova         m12, [rsp+21*16]
%endif

%endif ; if %1 == 8
%endif ; ifidn %2, v
%elif %1 == 6
    ; flat6 filter
%if ARCH_X86_32
    mova  [esp+3*16], m3
    mova  [esp+4*16], m4
    mova  [esp+5*16], m5
    mova  [esp+6*16], m6
 %xdefine m8  m3
 %xdefine m10 m4
 %xdefine m11 m5
 %xdefine m15 m6
 %define m3  [esp+3*16]
 %define m4  [esp+4*16]
 %define m5  [esp+5*16]
 %define m6  [esp+6*16]
 %define m9  %%flat8mem
 %define m13 %%p2mem
 %define m14 %%q2mem
%endif

    punpcklbw     m8, m13, m5
    punpckhbw    m11, m13, m5
    pmaddubsw     m0, m8, [PIC_sym(pb_3_1)]
    pmaddubsw     m1, m11, [PIC_sym(pb_3_1)]
    punpcklbw     m7, m4, m3
    punpckhbw    m10, m4, m3
    pmaddubsw     m2, m7, [PIC_sym(pb_2)]
    pmaddubsw    m15, m10, [PIC_sym(pb_2)]
    paddw         m0, m2
    paddw         m1, m15
    pmulhrsw      m2, m0, [PIC_sym(pw_4096)]
    pmulhrsw     m15, m1, [PIC_sym(pw_4096)]
    packuswb      m2, m15
    pand          m2, m9
    pandn        m15, m9, m3
    por           m2, m15
%ifidn %2, v
    mova [tmpq+strideq*2], m2                   ; p1
%elif ARCH_X86_32
    mova [esp+11*16], m2
%endif

    pmaddubsw     m8, [PIC_sym(pb_m1_1)]
    pmaddubsw    m11, [PIC_sym(pb_m1_1)]
    paddw         m0, m8
    paddw         m1, m11
    punpcklbw     m8, m13, m6
    punpckhbw    m11, m13, m6
%if ARCH_X86_64
    SWAP           2, 13
%endif
    pmaddubsw     m8, [PIC_sym(pb_m1_1)]
    pmaddubsw    m11, [PIC_sym(pb_m1_1)]
    paddw         m0, m8
    paddw         m1, m11
    pmulhrsw      m2, m0, [PIC_sym(pw_4096)]
    pmulhrsw     m15, m1, [PIC_sym(pw_4096)]
    packuswb      m2, m15
    pand          m2, m9
    pandn        m15, m9, m4
    por           m2, m15
%ifidn %2, v
    mova [tmpq+stride3q], m2                    ; p0
%elif ARCH_X86_32
    mova  [esp+8*16], m2
%endif

    paddw         m0, m8
    paddw         m1, m11
    punpcklbw     m8, m3, m14
    punpckhbw    m11, m3, m14
%if ARCH_X86_64
    SWAP           2, 14
%endif
    pmaddubsw     m2, m8, [PIC_sym(pb_m1_1)]
    pmaddubsw    m15, m11, [PIC_sym(pb_m1_1)]
    paddw         m0, m2
    paddw         m1, m15
    pmulhrsw      m2, m0, [PIC_sym(pw_4096)]
    pmulhrsw     m15, m1, [PIC_sym(pw_4096)]
    packuswb      m2, m15
    pand          m2, m9
    pandn        m15, m9, m5
    por           m2, m15
%ifidn %2, v
    mova [dstq+strideq*0], m2                   ; q0
%endif

    pmaddubsw     m8, [PIC_sym(pb_m1_2)]
    pmaddubsw    m11, [PIC_sym(pb_m1_2)]
    paddw         m0, m8
    paddw         m1, m11
    pmaddubsw     m7, [PIC_sym(pb_m1_0)]
    pmaddubsw    m10, [PIC_sym(pb_m1_0)]
    paddw         m0, m7
    paddw         m1, m10
    pmulhrsw      m0, [PIC_sym(pw_4096)]
    pmulhrsw      m1, [PIC_sym(pw_4096)]
    packuswb      m0, m1
    pand          m0, m9
    pandn         m1, m9, m6
    por           m0, m1
%if ARCH_X86_32
 %xdefine m3 m8
 %xdefine m4 m10
 %xdefine m5 m11
 %xdefine m6 m15
%endif
%ifidn %2, v
    mova [dstq+strideq*1], m0                   ; q1
%else
 %if ARCH_X86_64
    SWAP           3, 13
    SWAP           4, 14
 %else
    mova          m3, [esp+11*16]
    mova          m4, [esp+ 8*16]
 %endif
    SWAP           5, 2
    SWAP           6, 0
    TRANSPOSE_16x4_AND_WRITE_4x16 3, 4, 5, 6, 7
%endif
%else ; if %1 == 4
%ifidn %2, v
    mova [tmpq+strideq*0], m3                   ; p1
    mova [tmpq+strideq*1], m4                   ; p0
    mova [tmpq+strideq*2], m5                   ; q0
    mova [tmpq+stride3q ], m6                   ; q1
%else
    TRANSPOSE_16x4_AND_WRITE_4x16 3, 4, 5, 6, 7
%endif
%endif
%if ARCH_X86_32
 %define m12 m12reg
%endif
%endmacro

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;          32-bit PIC helpers          ;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

%if ARCH_X86_32
 %define PIC_base_offset $$

 %macro SETUP_PIC 0 ; PIC_reg
  %define PIC_reg r2
  %assign PIC_reg_stk_offset stack_size-gprsize*(1+copy_args*4)
    LEA      PIC_reg, $$
 %endmacro

 %macro XCHG_PIC_REG 1 ; 0=mask 1=PIC_base
  %if %1 == 0
    mov [esp+PIC_reg_stk_offset], PIC_reg
    mov      PIC_reg, maskm
  %else
    mov      PIC_reg, [esp+PIC_reg_stk_offset]
  %endif
 %endmacro

 %define PIC_sym(sym) (PIC_reg+(sym)-PIC_base_offset)

%else
 %macro XCHG_PIC_REG 1
 %endmacro
 %define PIC_sym(sym) (sym)
%endif

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

%if ARCH_X86_32
 %if STACK_ALIGNMENT < required_stack_alignment
  %assign copy_args 1
 %else
  %assign copy_args 0
 %endif
%endif

%macro RELOC_ARGS 1
 %if copy_args
  %define maskm     [esp+stack_size-gprsize*1]
  %define l_stridem [esp+stack_size-gprsize*2]
  %define lutm      [esp+stack_size-gprsize*3]
  %define %1m       [esp+stack_size-gprsize*4]
    mov          r6d, r6m
    mov        maskm, maskd
    mov         lutm, lutd
    mov          %1m, r6d
 %else
  %define %1m       r6m
 %endif
%endmacro

%if ARCH_X86_32
 %define tmpq       r4
 %define mstrideq   r5
 %define stride3q   r6
 %define l_stride3q r6
%endif

INIT_XMM ssse3
%if ARCH_X86_64
cglobal lpf_v_sb_y_8bpc, 7, 11, 16, 16 * 15, \
                    dst, stride, mask, l, l_stride, lut, \
                    w, stride3, mstride, tmp, mask_bits
%else
cglobal lpf_v_sb_y_8bpc, 6, 7, 8, -16 * (26 + copy_args), \
                    dst, stride, mask, l, l_stride, lut, mask_bits
    RELOC_ARGS w
    SETUP_PIC
 %define m12 m5
%endif
    shl    l_strideq, 2
    sub           lq, l_strideq
%if ARCH_X86_64
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]
%else
    mov    l_stridem, l_strided
%endif
    mov   mask_bitsd, 0xf
    mova         m12, [PIC_sym(pd_mask)]
    XCHG_PIC_REG   0
    movu          m0, [maskq]
    pxor          m4, m4
    movd          m3, [lutq+136]
    pshufb        m3, m4
    pshufd        m2, m0, q2222
    pshufd        m1, m0, q1111
    pshufd        m0, m0, q0000
    por           m1, m2
    por           m0, m1
    mova [rsp+11*16], m0
    mova [rsp+12*16], m1
    mova [rsp+13*16], m2
    mova [rsp+14*16], m3

%define maskmem [esp+15*16]
%define mask0   [rsp+11*16]
%define mask1   [rsp+12*16]
%define mask2   [rsp+13*16]
%define minlvl  [rsp+14*16]

.loop:
    test   [maskq+8], mask_bitsd                ; vmask[2]
    je .no_flat16

%if ARCH_X86_32
    XCHG_PIC_REG   1
    mov  [esp+25*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER        16, v
    jmp .end

.no_flat16:
    test   [maskq+4], mask_bitsd                ; vmask[1]
    je .no_flat

%if ARCH_X86_32
    XCHG_PIC_REG   1
    mov  [esp+25*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         8, v
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd                ; vmask[0]
    XCHG_PIC_REG   1
    je .no_filter

%if ARCH_X86_32
    mov  [esp+25*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         4, v

.end:
%if ARCH_X86_32
    mova         m12, maskmem
    mov   mask_bitsd, [esp+25*16]
%endif
.no_filter:
    pslld        m12, 4
    shl   mask_bitsd, 4
    add           lq, 16
    add         dstq, 16
%if ARCH_X86_64
    sub           wd, 4
%else
    sub     dword wm, 4
%endif
    XCHG_PIC_REG   0
    jg .loop
    RET

INIT_XMM ssse3
%if ARCH_X86_64
cglobal lpf_h_sb_y_8bpc, 7, 11, 16, 16 * 26, \
                    dst, stride, mask, l, l_stride, lut, \
                    h, stride3, l_stride3, tmp, mask_bits
%else
cglobal lpf_h_sb_y_8bpc, 6, 7, 8, -16 * (39 + copy_args), \
                    dst, stride, mask, l, l_stride, lut, mask_bits
    RELOC_ARGS h
    SETUP_PIC
 %define m12 m5
%endif
    sub           lq, 4
    shl    l_strideq, 2
%if ARCH_X86_64
    lea     stride3q, [strideq*3]
    lea   l_stride3q, [l_strideq*3]
%else
    mov    l_stridem, l_strided
%endif
    mov   mask_bitsd, 0xf
    mova         m12, [PIC_sym(pd_mask)]
    XCHG_PIC_REG   0
    movu          m0, [maskq]
    pxor          m4, m4
    movd          m3, [lutq+136]
    pshufb        m3, m4
    pshufd        m2, m0, q2222
    pshufd        m1, m0, q1111
    pshufd        m0, m0, q0000
    por           m1, m2
    por           m0, m1
    mova [rsp+22*16], m0
    mova [rsp+23*16], m1
    mova [rsp+24*16], m2
    mova [rsp+25*16], m3

%define maskmem [esp+37*16]
%define mask0   [rsp+22*16]
%define mask1   [rsp+23*16]
%define mask2   [rsp+24*16]
%define minlvl  [rsp+25*16]

.loop:
    test   [maskq+8], mask_bitsd                ; vmask[2]
    je .no_flat16

%if ARCH_X86_32
    XCHG_PIC_REG   1
    mov  [esp+38*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER        16, h
    jmp .end

.no_flat16:
    test   [maskq+4], mask_bitsd                ; vmask[1]
    je .no_flat

%if ARCH_X86_32
    XCHG_PIC_REG   1
    mov  [esp+38*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         8, h
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd                ; vmask[0]
    XCHG_PIC_REG   1
    je .no_filter

%if ARCH_X86_32
    mov  [esp+38*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         4, h
    jmp .end

.no_filter:
    lea         dstq, [dstq+strideq*8]
    lea         dstq, [dstq+strideq*8]
%if ARCH_X86_32
    jmp .end_noload
.end:
    mova         m12, maskmem
    mov    l_strideq, l_stridem
    mov   mask_bitsd, [esp+38*16]
.end_noload:
%else
.end:
%endif
    lea           lq, [lq+l_strideq*4]
    pslld        m12, 4
    shl   mask_bitsd, 4
%if ARCH_X86_64
    sub           hd, 4
%else
    sub     dword hm, 4
%endif
    XCHG_PIC_REG   0
    jg .loop
    RET

INIT_XMM ssse3
%if ARCH_X86_64
cglobal lpf_v_sb_uv_8bpc, 7, 11, 16, 3 * 16, \
                     dst, stride, mask, l, l_stride, lut, \
                     w, stride3, mstride, tmp, mask_bits
%else
cglobal lpf_v_sb_uv_8bpc, 6, 7, 8, -16 * (12 + copy_args), \
                     dst, stride, mask, l, l_stride, lut, mask_bits
    RELOC_ARGS w
    SETUP_PIC
 %define m12 m4
%endif
    shl    l_strideq, 2
    sub           lq, l_strideq
%if ARCH_X86_64
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]
%else
    mov    l_stridem, l_strided
%endif
    mov   mask_bitsd, 0xf
    mova         m12, [PIC_sym(pd_mask)]
    XCHG_PIC_REG   0
    movq          m0, [maskq]
    pxor          m3, m3
    movd          m2, [lutq+136]
    pshufb        m2, m3
    pshufd        m1, m0, q1111
    pshufd        m0, m0, q0000
    por           m0, m1
    mova  [rsp+0*16], m0
    mova  [rsp+1*16], m1
    mova  [rsp+2*16], m2

%define maskmem [esp+7*16]
%define mask0   [rsp+0*16]
%define mask1   [rsp+1*16]
%define minlvl  [rsp+2*16]

.loop:
    test   [maskq+4], mask_bitsd                ; vmask[1]
    je .no_flat

%if ARCH_X86_32
    XCHG_PIC_REG   1
    mov  [esp+11*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         6, v
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd                ; vmask[1]
    XCHG_PIC_REG   1
    je .no_filter

%if ARCH_X86_32
    mov  [esp+11*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         4, v

.end:
%if ARCH_X86_32
    mova         m12, maskmem
    mov   mask_bitsd, [esp+11*16]
%endif
.no_filter:
    pslld        m12, 4
    shl   mask_bitsd, 4
    add           lq, 16
    add         dstq, 16
%if ARCH_X86_64
    sub           wd, 4
%else
    sub     dword wm, 4
%endif
    XCHG_PIC_REG   0
    jg .loop
    RET

INIT_XMM ssse3
%if ARCH_X86_64
cglobal lpf_h_sb_uv_8bpc, 7, 11, 16, 16 * 3, \
                     dst, stride, mask, l, l_stride, lut, \
                     h, stride3, l_stride3, tmp, mask_bits
%else
cglobal lpf_h_sb_uv_8bpc, 6, 7, 8, -16 * (13 + copy_args), \
                     dst, stride, mask, l, l_stride, lut, mask_bits
    RELOC_ARGS h
    SETUP_PIC
 %define m12 m4
%endif
    sub           lq, 4
    shl    l_strideq, 2
%if ARCH_X86_64
    lea     stride3q, [strideq*3]
    lea   l_stride3q, [l_strideq*3]
%else
    mov    l_stridem, l_strided
%endif
    mov   mask_bitsd, 0xf
    mova         m12, [PIC_sym(pd_mask)]
    XCHG_PIC_REG   0
    movq          m0, [maskq]
    pxor          m3, m3
    movd          m2, [lutq+136]
    pshufb        m2, m3
    pshufd        m1, m0, q1111
    pshufd        m0, m0, q0000
    por           m0, m1
    mova  [rsp+0*16], m0
    mova  [rsp+1*16], m1
    mova  [rsp+2*16], m2

%define maskmem [esp+7*16]
%define mask0   [rsp+0*16]
%define mask1   [rsp+1*16]
%define minlvl  [rsp+2*16]

.loop:
    test   [maskq+4], mask_bitsd                ; vmask[1]
    je .no_flat

%if ARCH_X86_32
    XCHG_PIC_REG   1
    mov  [esp+12*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         6, h
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd                ; vmask[1]
    XCHG_PIC_REG   1
    je .no_filter

%if ARCH_X86_32
    mov  [esp+12*16], mask_bitsd
    mova     maskmem, m12
%endif
    FILTER         4, h
    jmp .end

.no_filter:
    lea         dstq, [dstq+strideq*8]
    lea         dstq, [dstq+strideq*8]
%if ARCH_X86_32
    jmp .end_noload
.end:
    mova         m12, maskmem
    mov    l_strided, l_stridem
    mov   mask_bitsd, [esp+12*16]
.end_noload:
%else
.end:
%endif
    lea           lq, [lq+l_strideq*4]
    pslld        m12, 4
    shl   mask_bitsd, 4
%if ARCH_X86_64
    sub           hd, 4
%else
    sub     dword hm, 4
%endif
    XCHG_PIC_REG   0
    jg .loop
    RET
