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

%if ARCH_X86_64

SECTION_RODATA 32

pb_mask: dd 1, 1, 2, 2, 4, 4, 8, 8
pb_4x1_4x5_4x9_4x13: times 4 db 0, 1
                     times 4 db 8, 9
                     times 4 db 0, 1
                     times 4 db 8, 9

pw_1:     times 16 dw 1
pw_2:     times 16 dw 2
pw_3:     times 16 dw 3
pw_4096:  times 2 dw 4096

; 10bpc/12bpc:
pw_4:     times 2 dw 4
          times 2 dw 16
clip_max: times 2 dw 511
          times 2 dw 2047
clip_min: times 2 dw -512
          times 2 dw -2048

SECTION .text

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
; xmm%1   a b c d e f g h      a i q y 6 E M U
; xmm%2   i j k l m n o p      b j r z 7 F N V
; xmm%3   q r s t u v w x      c k s 0 8 G O W
; xmm%4   y z 0 1 2 3 4 5      d l t 1 9 H P X
; xmm%5   6 7 8 9 A B C D  ->  e m u 2 A I Q Y
; xmm%6   E F G H I J K L      f n v 3 B J R Z
; xmm%7   M N O P Q R S T      g o w 4 C K S +
; xmm%8   U V W X Y Z + =      h p x 5 D L T =
%macro TRANSPOSE8X8W 9
    ; xmm%1   a b c d e f g h      a i q y b j r z
    ; xmm%2   i j k l m n o p      c k s 0 d l t 1
    ; xmm%3   q r s t u v w x  ->  e m u 2 f n v 3
    ; xmm%4   y z 0 1 2 3 4 5      g o w 4 h p x 5
    TRANSPOSE4X4W     %1, %2, %3, %4, %9

    ; xmm%5   6 7 8 9 A B C D      6 E M U 7 F N V
    ; xmm%6   E F G H I J K L      8 G O W 9 H P X
    ; xmm%7   M N O P Q R S T  ->  A I Q Y B J R Z
    ; xmm%8   U V W X Y Z + =      C K S + D L T =
    TRANSPOSE4X4W     %5, %6, %7, %8, %9

    ; xmm%1   a i q y b j r z      a i q y 6 E M U
    ; xmm%2   c k s 0 d l t 1      b j r z 7 F N V
    ; xmm%3   e m u 2 f n v 3      c k s 0 8 G O W
    ; xmm%4   g o w 4 h p x 5      d l t 1 9 H P X
    ; xmm%5   6 E M U 7 F N V  ->  e m u 2 A I Q Y
    ; xmm%6   8 G O W 9 H P X      f n v 3 B J R Z
    ; xmm%7   A I Q Y B J R Z      g o w 4 C K S +
    ; xmm%8   C K S + D L T =      h p x 5 D L T =
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

; transpose and write m3-6, everything else is scratch
%macro TRANSPOSE_8x4_AND_WRITE_4x16 0
    ; transpose 8x4
    punpcklwd     m0, m3, m4
    punpckhwd     m3, m4
    punpcklwd     m4, m5, m6
    punpckhwd     m5, m6
    punpckldq     m6, m0, m4
    punpckhdq     m0, m4
    punpckldq     m4, m3, m5
    punpckhdq     m3, m5

    ; write out
    movq   [dstq+strideq*0-4], xm6
    movhps [dstq+strideq*1-4], xm6
    movq   [dstq+strideq*2-4], xm0
    movhps [dstq+stride3q -4], xm0
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm4
    movhps [dstq+strideq*1-4], xm4
    movq   [dstq+strideq*2-4], xm3
    movhps [dstq+stride3q -4], xm3
    lea         dstq, [dstq+strideq*4]

    vextracti128 xm6, m6, 1
    vextracti128 xm0, m0, 1
    vextracti128 xm4, m4, 1
    vextracti128 xm3, m3, 1

    movq   [dstq+strideq*0-4], xm6
    movhps [dstq+strideq*1-4], xm6
    movq   [dstq+strideq*2-4], xm0
    movhps [dstq+stride3q -4], xm0
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm4
    movhps [dstq+strideq*1-4], xm4
    movq   [dstq+strideq*2-4], xm3
    movhps [dstq+stride3q -4], xm3
    lea         dstq, [dstq+strideq*4]
%endmacro

%macro FILTER 2 ; width [4/6/8/16], dir [h/v]
    ; load data
%ifidn %2, v
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
    mova         m13, [tmpq+strideq*1]
    mova          m3, [tmpq+strideq*2]
    mova          m4, [tmpq+stride3q]
    mova          m5, [dstq+strideq*0]
    mova          m6, [dstq+strideq*1]
    mova         m14, [dstq+strideq*2]
%if %1 != 6
    mova         m15, [dstq+stride3q]
%endif
%endif
%else
    ; load lines
%if %1 == 4
    movq         xm3, [dstq+strideq*0-4]
    movq         xm4, [dstq+strideq*1-4]
    movq         xm5, [dstq+strideq*2-4]
    movq         xm6, [dstq+stride3q -4]
    lea         tmpq, [dstq+strideq*4]
    movq        xm11, [tmpq+strideq*0-4]
    movq        xm13, [tmpq+strideq*1-4]
    movq        xm14, [tmpq+strideq*2-4]
    movq        xm15, [tmpq+stride3q -4]
    lea         tmpq, [tmpq+strideq*4]
    ; this overreads by 8 bytes but the buffers are padded
    ; so that should be ok
    vinserti128   m3, [tmpq+strideq*0-4], 1
    vinserti128   m4, [tmpq+strideq*1-4], 1
    vinserti128   m5, [tmpq+strideq*2-4], 1
    vinserti128   m6, [tmpq+stride3q -4], 1
    lea         tmpq, [tmpq+strideq*4]
    vinserti128  m11, [tmpq+strideq*0-4], 1
    vinserti128  m13, [tmpq+strideq*1-4], 1
    vinserti128  m14, [tmpq+strideq*2-4], 1
    vinserti128  m15, [tmpq+stride3q -4], 1

    ; transpose 4x8
    ; xm3: A-D0,A-D4
    ; xm4: A-D1,A-D5
    ; xm5: A-D2,A-D6
    ; xm6: A-D3,A-D7
    punpcklwd     m7, m3, m4
    punpcklwd     m3, m11, m13
    punpcklwd     m4, m5, m6
    punpcklwd     m5, m14, m15
    ; xm7: A0-1,B0-1,C0-1,D0-1
    ; xm3: A4-5,B4-5,C4-5,D4-5
    ; xm4: A2-3,B2-3,C2-3,D2-3
    ; xm5: A6-7,B6-7,C6-7,D6-7
    punpckldq     m6, m7, m4
    punpckhdq     m7, m4
    punpckldq     m8, m3, m5
    punpckhdq     m5, m3, m5
    ; xm6: A0-3,B0-3
    ; xm7: C0-3,D0-3
    ; xm8: A4-7,B4-7
    ; xm5: C4-7,D4-7
    punpcklqdq    m3, m6, m8
    punpckhqdq    m4, m6, m8
    punpckhqdq    m6, m7, m5
    punpcklqdq    m5, m7, m5
    ; xm3: A0-7
    ; xm4: B0-7
    ; xm5: C0-7
    ; xm6: D0-7
%elif %1 == 6 || %1 == 8
    movu         xm3, [dstq+strideq*0-8]
    movu         xm4, [dstq+strideq*1-8]
    movu         xm5, [dstq+strideq*2-8]
    movu         xm6, [dstq+stride3q -8]
    lea         tmpq, [dstq+strideq*4]
    movu        xm11, [tmpq+strideq*0-8]
    movu        xm13, [tmpq+strideq*1-8]
    movu        xm14, [tmpq+strideq*2-8]
    movu        xm15, [tmpq+stride3q -8]
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m3, [tmpq+strideq*0-8], 1
    vinserti128   m4, [tmpq+strideq*1-8], 1
    vinserti128   m5, [tmpq+strideq*2-8], 1
    vinserti128   m6, [tmpq+stride3q -8], 1
    lea         tmpq, [tmpq+strideq*4]
    vinserti128  m11, [tmpq+strideq*0-8], 1
    vinserti128  m13, [tmpq+strideq*1-8], 1
    vinserti128  m14, [tmpq+strideq*2-8], 1
    vinserti128  m15, [tmpq+stride3q -8], 1

    ; transpose 8x16
    ; xm3: A-H0,A-H8
    ; xm4: A-H1,A-H9
    ; xm5: A-H2,A-H10
    ; xm6: A-H3,A-H11
    ; xm11: A-H4,A-H12
    ; xm13: A-H5,A-H13
    ; xm14: A-H6,A-H14
    ; xm15: A-H7,A-H15
    punpcklwd    m7, m3, m4
    punpckhwd    m3, m4
    punpcklwd    m4, m5, m6
    punpckhwd    m5, m6
    punpcklwd    m6, m11, m13
    punpckhwd   m11, m13
    punpcklwd   m13, m14, m15
    punpckhwd   m14, m15
    ; xm7: A0-1,B0-1,C0-1,D0-1
    ; xm3: E0-1,F0-1,G0-1,H0-1
    ; xm4: A2-3,B2-3,C2-3,D2-3
    ; xm5: E2-3,F2-3,G2-3,H2-3
    ; xm6: A4-5,B4-5,C4-5,D4-5
    ; xm11: E4-5,F4-5,G4-5,H4-5
    ; xm13: A6-7,B6-7,C6-7,D6-7
    ; xm14: E6-7,F6-7,G6-7,H6-7
    punpckldq   m15, m7, m4
    punpckhdq    m7, m4
    punpckldq    m9, m3, m5
    punpckhdq    m8, m3, m5
    punpckldq    m3, m6, m13
    punpckhdq    m6, m13
    punpckldq   m10, m11, m14
    punpckhdq   m11, m14
    ; xm15: A0-3,B0-3
    ; xm7: C0-3,D0-3
    ; xm9: E0-3,F0-3
    ; xm8: G0-3,H0-3
    ; xm3: A4-7,B4-7
    ; xm6: C4-7,D4-7
    ; xm10: E4-7,F4-7
    ; xm11: G4-7,H4-7
%if %1 != 6
    punpcklqdq   m0, m15, m3
%endif
    punpckhqdq  m13, m15, m3
    punpcklqdq   m3, m7, m6
    punpckhqdq   m4, m7, m6
    punpcklqdq   m5, m9, m10
    punpckhqdq   m6, m9, m10
    punpcklqdq  m14, m8, m11
%if %1 != 6
    punpckhqdq  m15, m8, m11
    mova [rsp+5*32], m0
%endif
%else
    ; We only use 14 pixels but we'll need the remainder at the end for
    ; the second transpose
    mova         xm0, [dstq+strideq*0-16]
    mova         xm1, [dstq+strideq*1-16]
    mova         xm2, [dstq+strideq*2-16]
    mova         xm3, [dstq+stride3q -16]
    lea         tmpq, [dstq+strideq*4]
    mova         xm4, [tmpq+strideq*0-16]
    mova         xm5, [tmpq+strideq*1-16]
    mova         xm6, [tmpq+strideq*2-16]
    mova         xm7, [tmpq+stride3q -16]
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m0, m0, [tmpq+strideq*0-16], 1
    vinserti128   m1, m1, [tmpq+strideq*1-16], 1
    vinserti128   m2, m2, [tmpq+strideq*2-16], 1
    vinserti128   m3, m3, [tmpq+stride3q -16], 1
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m4, m4, [tmpq+strideq*0-16], 1
    vinserti128   m5, m5, [tmpq+strideq*1-16], 1
    vinserti128   m6, m6, [tmpq+strideq*2-16], 1
    vinserti128   m7, m7, [tmpq+stride3q -16], 1

    TRANSPOSE8X8W 0, 1, 2, 3, 4, 5, 6, 7, 8

    mova    [rsp+6*32], m0
    mova    [rsp+7*32], m1
    mova    [rsp+8*32], m2
    mova    [rsp+9*32], m3
    mova    [rsp+5*32], m4

    mova         xm0, [dstq+strideq*0]
    mova         xm1, [dstq+strideq*1]
    mova         xm2, [dstq+strideq*2]
    mova         xm3, [dstq+stride3q ]
    lea         tmpq, [dstq+strideq*4]
    mova         xm8, [tmpq+strideq*0]
    mova         xm9, [tmpq+strideq*1]
    mova        xm10, [tmpq+strideq*2]
    mova        xm11, [tmpq+stride3q ]
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m0, m0, [tmpq+strideq*0], 1
    vinserti128   m1, m1, [tmpq+strideq*1], 1
    vinserti128   m2, m2, [tmpq+strideq*2], 1
    vinserti128   m3, m3, [tmpq+stride3q ], 1
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m8, m8, [tmpq+strideq*0], 1
    vinserti128   m9, m9, [tmpq+strideq*1], 1
    vinserti128  m10, m10, [tmpq+strideq*2], 1
    vinserti128  m11, m11, [tmpq+stride3q ], 1

    TRANSPOSE8X8W 0, 1, 2, 3, 8, 9, 10, 11, 4

    mova   [rsp+10*32], m8
    mova   [rsp+11*32], m9
    mova   [rsp+12*32], m10
    mova   [rsp+13*32], m11

    ; 5,6,7,0,1,2,3 -> 13,3,4,5,6,14,15
    SWAP         13, 5, 0
    SWAP          3, 6, 1, 15
    SWAP          4, 7
    SWAP          2, 14
%endif
%endif

    ; load L/E/I/H
%ifidn %2, v
    pmovzxbw      m1, [lq]
    pmovzxbw      m0, [lq+l_strideq]
    pxor          m2, m2
%else
    vpbroadcastq  m0, [lq]                      ; l0, l1
    vpbroadcastq  m1, [lq+l_strideq]            ; l2, l3
    vpbroadcastq  m2, [lq+l_strideq*2]          ; l4, l5
    vpbroadcastq m10, [lq+l_stride3q]           ; l6, l7
    punpckldq     m0, m1                        ; l0, l2, l1, l3 [2x]
    punpckldq     m2, m10                       ; l4, l6, l5, l7 [2x]
    vpblendd      m0, m0, m2, 11110000b         ; l0, l2, l1, l3, l4, l6, l5, l7
    pxor          m2, m2
    punpcklbw     m1, m0, m2                    ; l0, l2, l4, l6
    punpckhbw     m0, m2                        ; l1, l3, l5, l7
%endif
    pcmpeqw      m10, m2, m0
    pand          m1, m10
    por           m0, m1                        ; l[x][] ? l[x][] : l[x-stride][]
    pshufb        m0, [pb_4x1_4x5_4x9_4x13]     ; l[x][1]
    pcmpeqw      m10, m2, m0                    ; !L
    psrlw        m10, 1
    psrlw         m2, m0, [lutq+128]
    vpbroadcastw  m1, [lutq+136]
    pminuw        m2, m1
    pmaxuw        m2, [pw_1]                    ; I
    psrlw         m1, m0, 4                     ; H
    paddw         m0, [pw_2]
    vpbroadcastd  m8, [r11]
    paddw         m0, m0
    paddw         m0, m2                        ; E
    REPX {pmullw x, m8}, m0, m1, m2

    psubw         m8, m3, m4                    ; p1-p0
    psubw         m9, m5, m6                    ; q1-q0
    REPX {pabsw x, x}, m8, m9
    pmaxuw        m8, m10
    pmaxuw        m8, m9
    pcmpgtw       m7, m8, m1                    ; hev
%if %1 != 4
    psubw         m9, m13, m4                   ; p2-p0
    pabsw         m9, m9
    pmaxuw        m9, m8
%if %1 != 6
%ifidn %2, v
    mova         m11, [tmpq+strideq*0]          ; p3
%else
    mova         m11, [rsp+5*32]                ; p3
%endif
    psubw        m10, m11, m4                   ; p3-p0
    pabsw        m10, m10
    pmaxuw        m9, m10
%endif
    psubw        m10, m5, m14                   ; q2-q0
    pabsw        m10, m10
    pmaxuw        m9, m10
%if %1 != 6
    psubw        m10, m5, m15                   ; q3-q0
    pabsw        m10, m10
    pmaxuw        m9, m10
%endif
    vpbroadcastd m10, [r11]
    pcmpgtw       m9, m10                       ; !flat8in

    psubw        m10, m13, m3                   ; p2-p1
    pabsw        m10, m10
%if %1 != 6
    psubw        m11, m13                       ; p3-p2
    pabsw        m11, m11
    pmaxuw       m10, m11
    psubw        m11, m14, m15                  ; q3-q2
    pabsw        m11, m11
    pmaxuw       m10, m11
%endif
    psubw        m11, m14, m6                   ; q2-q1
    pabsw        m11, m11
    pmaxuw       m10, m11

%if %1 == 16
    vpbroadcastd m11, [maskq+8]
    vpbroadcastd  m1, [maskq+4]
    por          m11, m1
    pand         m11, m12
    pcmpeqd      m11, m12
    pand         m10, m11
%else
    vpbroadcastd m11, [maskq+4]
    pand         m11, m12
    pcmpeqd      m11, m12
    pand         m10, m11                       ; only apply fm-wide to wd>4 blocks
%endif
    pmaxuw        m8, m10
%endif
    pcmpgtw       m8, m2

    psubw        m10, m3, m6                    ; p1-q1
    psubw        m11, m4, m5                    ; p0-q0
    REPX {pabsw x, x}, m10, m11
    paddw        m11, m11
    psrlw        m10, 1
    paddw        m10, m11                       ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    pcmpgtw      m10, m0                        ; abs(p0-q0)*2+(abs(p1-q1)>>1) > E
    por           m8, m10

%if %1 == 16

%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]
    mova          m1, [tmpq+strideq*2]
    mova          m2, [tmpq+stride3q]
%else
    mova          m0, [rsp+7*32]
    mova          m1, [rsp+8*32]
    mova          m2, [rsp+9*32]
%endif
    REPX {psubw x, m4}, m0, m1, m2
    REPX {pabsw x, x}, m0, m1, m2
    pmaxuw        m1, m0
    pmaxuw        m1, m2
%ifidn %2, v
    lea         tmpq, [dstq+strideq*4]
    mova          m0, [tmpq+strideq*0]
    mova          m2, [tmpq+strideq*1]
    mova         m10, [tmpq+strideq*2]
%else
    mova          m0, [rsp+10*32]
    mova          m2, [rsp+11*32]
    mova         m10, [rsp+12*32]
%endif
    REPX {psubw x, m5}, m0, m2, m10
    REPX {pabsw x, x}, m0, m2, m10
    pmaxuw        m0, m2
    pmaxuw        m1, m10
    pmaxuw        m1, m0
    vpbroadcastd  m0, [r11]
    pcmpgtw       m1, m0                        ; !flat8out
    por           m1, m9                        ; !flat8in | !flat8out
    vpbroadcastd  m2, [maskq+8]
    pand         m10, m2, m12
    pcmpeqd      m10, m12
    pandn         m1, m10                       ; flat16
    pandn         m1, m8, m1                    ; flat16 & fm

    vpbroadcastd m10, [maskq+4]
    por          m10, m2
    pand          m2, m10, m12
    pcmpeqd       m2, m12
    pandn         m9, m2                        ; flat8in
    pandn         m9, m8, m9
    vpbroadcastd  m2, [maskq+0]
    por           m2, m10
    pand          m2, m12
    pcmpeqd       m2, m12
    pandn         m8, m2
    pandn         m8, m9, m8                    ; fm & !flat8 & !flat16
    pandn         m9, m1, m9                    ; flat8 & !flat16
%elif %1 != 4
    vpbroadcastd  m0, [maskq+4]
    pand          m2, m0, m12
    pcmpeqd       m2, m12
    pandn         m9, m2
    pandn         m9, m8, m9                    ; flat8 & fm
    vpbroadcastd  m2, [maskq+0]
    por           m0, m2
    pand          m0, m12
    pcmpeqd       m0, m12
    pandn         m8, m0
    pandn         m8, m9, m8                    ; fm & !flat8
%else
    vpbroadcastd  m0, [maskq+0]
    pand          m0, m12
    pcmpeqd       m0, m12
    pandn         m8, m0                        ; fm
%endif

    ; short filter
    vpbroadcastd  m0, [r11+8*1]                 ; 511 or 2047
    vpbroadcastd  m2, [r11+8*2]                 ; -512 or -2048
    psubw        m10, m5, m4
    paddw        m11, m10, m10
    paddw        m11, m10
    psubw        m10, m3, m6                    ; iclip_diff(p1-q1)
    pminsw       m10, m0
    pmaxsw       m10, m2
    pand         m10, m7                        ; f=iclip_diff(p1-q1)&hev
    paddw        m10, m11                       ; f=iclip_diff(3*(q0-p0)+f)
    pminsw       m10, m0
    pmaxsw       m10, m2
    pand          m8, m10                       ; f&=fm
    vpbroadcastd m10, [pw_4]
    paddw        m10, m8
    paddw         m8, [pw_3]
    REPX {pminsw x, m0}, m10, m8
    psraw        m10, 3                         ; f2
    psraw         m8, 3                         ; f1
    psubw         m5, m10
    paddw         m4, m8

    paddw        m10, [pw_1]
    psraw        m10, 1                         ; f=(f1+1)>>1
    pandn         m8, m7, m10                   ; f&=!hev
    paddw         m3, m8
    psubw         m6, m8
    pxor          m8, m8
    psubw         m0, m2                        ; 1023 or 4095
    REPX {pminsw x, m0}, m3, m4, m5, m6
    REPX {pmaxsw x, m8}, m3, m4, m5, m6

%if %1 == 16

; m3-6 = p1/p0/q0/q1, m9=flat8, m1=flat16
; m12=filter bits mask
; m13-15=p2/q2/q3
; m0,2,7-8,10-11 = free

    ; flat16 filter
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]          ; p6
    mova          m2, [tmpq+strideq*2]          ; p5
    mova          m7, [tmpq+stride3q]           ; p4
    mova         m11, [tmpq+strideq*4]          ; p3
%else
    mova          m0, [rsp+7*32]
    mova          m2, [rsp+8*32]
    mova          m7, [rsp+9*32]
    mova         m11, [rsp+5*32]
%endif

    mova [rsp+ 0*32], m9

    ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
    paddw         m8, m0, [pw_1]
    psllw         m8, 3                         ; p6*8+8
    paddw        m10, m2, m7                    ; p5+p4
    psubw         m8, m0
    paddw        m10, m10                       ; (p5+p4)*2
    paddw         m8, m11                       ; p6*7+p3
    paddw        m10, m13                       ; (p5+p4)*2+p2
    paddw         m8, m3                        ; p6*7+p3+p1
    paddw        m10, m4                        ; (p5+p4)*2+p2+p0
    paddw         m8, m5                        ; p6*7+p3+p1+q0
    paddw         m8, m10                       ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
    psrlw        m10, m8, 4
    vpblendvb    m10, m2, m10, m1
%ifidn %2, v
    mova [tmpq+strideq*2], m10                  ; p5
%else
    mova [rsp+8*32], m10
%endif

    ; sub p6*2, add p3/q1
    paddw         m8, m11
    paddw        m10, m0, m0
    paddw         m8, m6
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb    m10, m7, m10, m1
%ifidn %2, v
    mova [tmpq+stride3q], m10                   ; p4
%else
    mova [rsp+9*32], m10
%endif

    ; sub p6/p5, add p2/q2
    psubw         m8, m0
    paddw        m10, m13, m14
    psubw         m8, m2
    paddw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb    m10, m11, m10, m1
%ifidn %2, v
    mova [tmpq+strideq*4], m10                  ; p3
    lea         tmpq, [dstq+strideq*4]
%else
    mova [rsp+5*32], m10
%endif

    ; sub p6/p4, add p1/q3
    paddw         m8, m3
    paddw        m10, m0, m7
    paddw         m8, m15
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb    m10, m13, m10, m1
    mova  [rsp+1*32], m10                       ; don't clobber p2/m13

    ; sub p6/p3, add p0/q4
    paddw         m8, m4
    paddw        m10, m0, m11
%ifidn %2, v
    paddw         m8, [tmpq+strideq*0]
%else
    paddw         m8, [rsp+10*32]
%endif
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb    m10, m3, m10, m1
    mova  [rsp+2*32], m10                       ; don't clobber p1/m3

    ; sub p6/p2, add q0/q5
    paddw         m8, m5
    paddw        m10, m0, m13
%ifidn %2, v
    paddw         m8, [tmpq+strideq*1]
%else
    paddw         m8, [rsp+11*32]
%endif
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb    m10, m4, m10, m1
    mova  [rsp+3*32], m10                       ; don't clobber p0/m4

    ; sub p6/p1, add q1/q6
    paddw         m8, m6
    paddw        m10, m0, m3
%ifidn %2, v
    mova          m0, [tmpq+strideq*2]          ; q6
%else
    mova          m0, [rsp+12*32]               ; q6
%endif
    paddw         m8, m0
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb    m10, m5, m10, m1
    mova  [rsp+4*32], m10                       ; don't clobber q0/m5

    ; sub p5/p0, add q2/q6
    paddw         m8, m14
    paddw        m10, m2, m4
    paddw         m8, m0
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb     m2, m6, m10, m1               ; don't clobber q1/m6

    ; sub p4/q0, add q3/q6
    paddw         m8, m15
    paddw        m10, m7, m5
    paddw         m8, m0
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb     m7, m14, m10, m1              ; don't clobber q2/m14

    ; sub p3/q1, add q4/q6
%ifidn %2, v
    paddw         m8, [tmpq+strideq*0]
%else
    paddw         m8, [rsp+10*32]
%endif
    paddw        m10, m11, m6
    paddw         m8, m0
    psubw         m8, m10
    psrlw        m10, m8, 4
    vpblendvb    m10, m15, m10, m1
%ifidn %2, v
    mova [tmpq+mstrideq], m10                   ; q3
%else
    mova [rsp+14*32], m10
%endif

    ; sub p2/q2, add q5/q6
%ifidn %2, v
    paddw         m8, [tmpq+strideq*1]
%else
    paddw         m8, [rsp+11*32]
%endif
    paddw        m10, m13, m14
    paddw         m8, m0
    psubw         m8, m10
    psrlw        m10, m8, 4
%ifidn %2, v
    mova          m9, [tmpq+strideq*0]
%else
    mova          m9, [rsp+10*32]
%endif
    vpblendvb    m10, m9, m10, m1
%ifidn %2, v
    mova [tmpq+strideq*0], m10                   ; q4
%else
    mova [rsp+10*32], m10
%endif

    ; sub p1/q3, add q6*2
    psubw         m8, m3
    paddw         m0, m0
    psubw         m8, m15
    paddw         m8, m0
    psrlw        m10, m8, 4
%ifidn %2, v
    mova          m9, [tmpq+strideq*1]
%else
    mova          m9, [rsp+11*32]
%endif
    vpblendvb    m10, m9, m10, m1
%ifidn %2, v
    mova [tmpq+strideq*1], m10                  ; q5
%else
    mova [rsp+11*32], m10
%endif

    mova          m9, [rsp+0*32]
    mova         m13, [rsp+1*32]
    mova          m3, [rsp+2*32]
    mova          m4, [rsp+3*32]
    mova          m5, [rsp+4*32]
    SWAP           2, 6
    SWAP           7, 14
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*4]
%else
    mova         m15, [rsp+14*32]
%endif
%endif

%if %1 >= 8
    ; flat8 filter
    vpbroadcastd  m7, [pw_4096]
%ifidn %2, v
    mova          m0, [tmpq+strideq*0]          ; p3
%else
    mova          m0, [rsp+5*32]                ; p3
%endif
    paddw         m1, m0, m13                   ; p3+p2
    paddw         m2, m3, m4                    ; p1+p0
    paddw         m8, m1, m1                    ; 2*(p3+p2)
    paddw         m2, m0                        ; p1+p0+p3
    paddw         m8, m5                        ; 2*(p3+p2)+q0
    paddw         m2, m8                        ; 3*p3+2*p2+p1+p0+q0
    pmulhrsw     m10, m2, m7

    paddw         m8, m3, m6
    psubw         m2, m1
    paddw         m2, m8
    pmulhrsw      m8, m2, m7

    paddw        m11, m0, m3
    paddw         m1, m4, m14
    psubw         m2, m11
    paddw         m2, m1
    pmulhrsw      m1, m2, m7

    paddw        m11, m0, m4
    pblendvb      m4, m1, m9
    paddw         m1, m5, m15
    psubw         m2, m11
    paddw         m2, m1
    pmulhrsw     m11, m2, m7

    paddw         m2, m6
    paddw         m2, m15
    paddw         m1, m13, m5
    pblendvb      m5, m11, m9
    pblendvb     m13, m10, m9
    psubw         m2, m1
    pmulhrsw      m1, m2, m7

    psubw         m2, m3
    pblendvb      m3, m8, m9
    psubw         m2, m6
    pblendvb      m6, m1, m9
    paddw         m1, m15, m14
    paddw         m2, m1
    pmulhrsw      m2, m7

    pblendvb     m14, m2, m9

%ifidn %2, v
    mova [tmpq+strideq*1], m13                  ; p2
    mova [tmpq+strideq*2], m3                   ; p1
    mova [tmpq+stride3q ], m4                   ; p0
    mova [dstq+strideq*0], m5                   ; q0
    mova [dstq+strideq*1], m6                   ; q1
    mova [dstq+strideq*2], m14                  ; q2
%elif %1 == 8
    TRANSPOSE8X8W  0, 13, 3, 4, 5, 6, 14, 15, 1

    ; write 8x16
    movu   [dstq+strideq*0-8], xm0
    movu   [dstq+strideq*1-8], xm13
    movu   [dstq+strideq*2-8], xm3
    movu   [dstq+stride3q -8], xm4
    lea         dstq, [dstq+strideq*4]
    movu   [dstq+strideq*0-8], xm5
    movu   [dstq+strideq*1-8], xm6
    movu   [dstq+strideq*2-8], xm14
    movu   [dstq+stride3q -8], xm15
    lea         dstq, [dstq+strideq*4]
    vextracti128 [dstq+strideq*0-8], m0, 1
    vextracti128 [dstq+strideq*1-8], m13, 1
    vextracti128 [dstq+strideq*2-8], m3, 1
    vextracti128 [dstq+stride3q -8], m4, 1
    lea         dstq, [dstq+strideq*4]
    vextracti128 [dstq+strideq*0-8], m5, 1
    vextracti128 [dstq+strideq*1-8], m6, 1
    vextracti128 [dstq+strideq*2-8], m14, 1
    vextracti128 [dstq+stride3q -8], m15, 1
    lea         dstq, [dstq+strideq*4]
%else
    mova          m8, [rsp+6*32]
    mova          m1, [rsp+7*32]
    mova          m2, [rsp+8*32]
    mova          m7, [rsp+9*32]
    TRANSPOSE8X8W  8, 1, 2, 7, 0, 13, 3, 4, 9

    mova [dstq+strideq*0-16], xm8
    mova [dstq+strideq*1-16], xm1
    mova [dstq+strideq*2-16], xm2
    mova [dstq+stride3q -16], xm7
    lea         tmpq, [dstq+strideq*4]
    mova [tmpq+strideq*0-16], xm0
    mova [tmpq+strideq*1-16], xm13
    mova [tmpq+strideq*2-16], xm3
    mova [tmpq+stride3q -16], xm4
    lea         tmpq, [tmpq+strideq*4]
    vextracti128 [tmpq+strideq*0-16], m8, 1
    vextracti128 [tmpq+strideq*1-16], m1, 1
    vextracti128 [tmpq+strideq*2-16], m2, 1
    vextracti128 [tmpq+stride3q -16], m7, 1
    lea         tmpq, [tmpq+strideq*4]
    vextracti128 [tmpq+strideq*0-16], m0, 1
    vextracti128 [tmpq+strideq*1-16], m13, 1
    vextracti128 [tmpq+strideq*2-16], m3, 1
    vextracti128 [tmpq+stride3q -16], m4, 1

    mova          m0, [rsp+10*32]
    mova          m1, [rsp+11*32]
    mova          m2, [rsp+12*32]
    mova          m3, [rsp+13*32]
    TRANSPOSE8X8W  5, 6, 14, 15, 0, 1, 2, 3, 4
    mova [dstq+strideq*0], xm5
    mova [dstq+strideq*1], xm6
    mova [dstq+strideq*2], xm14
    mova [dstq+stride3q ], xm15
    lea         dstq, [dstq+strideq*4]
    mova [dstq+strideq*0], xm0
    mova [dstq+strideq*1], xm1
    mova [dstq+strideq*2], xm2
    mova [dstq+stride3q ], xm3
    lea         dstq, [dstq+strideq*4]
    vextracti128 [dstq+strideq*0], m5, 1
    vextracti128 [dstq+strideq*1], m6, 1
    vextracti128 [dstq+strideq*2], m14, 1
    vextracti128 [dstq+stride3q ], m15, 1
    lea         dstq, [dstq+strideq*4]
    vextracti128 [dstq+strideq*0], m0, 1
    vextracti128 [dstq+strideq*1], m1, 1
    vextracti128 [dstq+strideq*2], m2, 1
    vextracti128 [dstq+stride3q ], m3, 1
    lea         dstq, [dstq+strideq*4]
%endif
%elif %1 == 6
    ; flat6 filter
    vpbroadcastd  m7, [pw_4096]
    paddw         m8, m3, m4
    paddw         m8, m13                       ; p2+p1+p0
    paddw        m11, m13, m5
    paddw         m8, m8
    paddw         m8, m11                       ; p2+2*(p2+p1+p0)+q0
    pmulhrsw      m2, m8, m7

    paddw         m8, m5
    paddw        m11, m13, m13
    paddw         m8, m6
    psubw         m8, m11
    pmulhrsw     m10, m8, m7

    paddw         m8, m6
    paddw        m11, m13, m3
    paddw         m8, m14
    psubw         m8, m11
    pmulhrsw     m11, m8, m7

    psubw         m8, m3
    paddw        m14, m14
    psubw         m8, m4
    paddw         m8, m14
    pmulhrsw      m8, m7

    pblendvb      m3, m2, m9
    pblendvb      m4, m10, m9
    pblendvb      m5, m11, m9
    pblendvb      m6, m8, m9

%ifidn %2, v
    mova [tmpq+strideq*2], m3                   ; p1
    mova [tmpq+stride3q ], m4                   ; p0
    mova [dstq+strideq*0], m5                   ; q0
    mova [dstq+strideq*1], m6                   ; q1
%else
    TRANSPOSE_8x4_AND_WRITE_4x16
%endif
%else
%ifidn %2, v
    mova [tmpq+strideq*0], m3                   ; p1
    mova [tmpq+strideq*1], m4                   ; p0
    mova [tmpq+strideq*2], m5                   ; q0
    mova [tmpq+stride3q ], m6                   ; q1
%else
    TRANSPOSE_8x4_AND_WRITE_4x16
%endif
%endif
%endmacro

INIT_YMM avx2
cglobal lpf_v_sb_y_16bpc, 6, 12, 16, 32 * 5, \
                          dst, stride, mask, l, l_stride, lut, \
                          w, stride3, mstride, tmp, mask_bits
    mov          r6d, r7m
    lea          r11, [pw_4]
    shr          r6d, 11                      ; is_12bpc
    lea          r11, [r11+r6*4]
    mov           wd, wm
    shl    l_strideq, 2
    sub           lq, l_strideq
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]
    mov   mask_bitsd, 0xf
    mova         m12, [pb_mask]

.loop:
    test   [maskq+8], mask_bitsd              ; vmask[2]
    jz .no_flat16

    FILTER        16, v
    jmp .end

.no_flat16:
    test   [maskq+4], mask_bitsd              ; vmask[1]
    jz .no_flat

    FILTER         8, v
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd              ; vmask[0]
    jz .end

    call .v4

.end:
    pslld        m12, 4
    add           lq, 16
    add         dstq, 32
    shl   mask_bitsd, 4
    sub           wd, 4
    jg .loop
    RET
ALIGN function_align
.v4:
    FILTER         4, v
    ret

INIT_YMM avx2
cglobal lpf_h_sb_y_16bpc, 6, 12, 16, 32 * 15, \
                          dst, stride, mask, l, l_stride, lut, \
                          h, stride3, l_stride3, tmp, mask_bits
    mov          r6d, r7m
    lea          r11, [pw_4]
    shr          r6d, 11                      ; is_12bpc
    lea          r11, [r11+r6*4]
    mov           hd, hm
    shl    l_strideq, 2
    sub           lq, 4
    lea     stride3q, [strideq*3]
    lea   l_stride3q, [l_strideq*3]
    mov   mask_bitsd, 0xf
    mova         m12, [pb_mask]

.loop:
    test   [maskq+8], mask_bitsd            ; vmask[2]
    jz .no_flat16

    FILTER        16, h
    jmp .end

.no_flat16:
    test   [maskq+4], mask_bitsd            ; vmask[1]
    jz .no_flat

    FILTER         8, h
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd            ; vmask[0]
    jz .no_filter

    call .h4
    jmp .end

.no_filter:
    lea         dstq, [dstq+strideq*8]
    lea         dstq, [dstq+strideq*8]
.end:
    pslld        m12, 4
    lea           lq, [lq+l_strideq*4]
    shl   mask_bitsd, 4
    sub           hd, 4
    jg .loop
    RET
ALIGN function_align
.h4:
    FILTER         4, h
    ret

INIT_YMM avx2
cglobal lpf_v_sb_uv_16bpc, 6, 12, 16, \
                           dst, stride, mask, l, l_stride, lut, \
                           w, stride3, mstride, tmp, mask_bits
    mov          r6d, r7m
    lea          r11, [pw_4]
    shr          r6d, 11                      ; is_12bpc
    lea          r11, [r11+r6*4]
    mov           wd, wm
    shl    l_strideq, 2
    sub           lq, l_strideq
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]
    mov   mask_bitsd, 0xf
    mova         m12, [pb_mask]

.loop:
    test   [maskq+4], mask_bitsd            ; vmask[1]
    jz .no_flat

    FILTER         6, v
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd            ; vmask[0]
    jz .end

    call mangle(private_prefix %+ _lpf_v_sb_y_16bpc_avx2).v4

.end:
    pslld        m12, 4
    add           lq, 16
    add         dstq, 32
    shl   mask_bitsd, 4
    sub           wd, 4
    jg .loop
    RET

INIT_YMM avx2
cglobal lpf_h_sb_uv_16bpc, 6, 12, 16, \
                           dst, stride, mask, l, l_stride, lut, \
                           h, stride3, l_stride3, tmp, mask_bits
    mov          r6d, r7m
    lea          r11, [pw_4]
    shr          r6d, 11                      ; is_12bpc
    lea          r11, [r11+r6*4]
    mov           hd, hm
    shl    l_strideq, 2
    sub           lq, 4
    lea     stride3q, [strideq*3]
    lea   l_stride3q, [l_strideq*3]
    mov   mask_bitsd, 0xf
    mova         m12, [pb_mask]

.loop:
    test   [maskq+4], mask_bitsd            ; vmask[1]
    jz .no_flat

    FILTER         6, h
    jmp .end

.no_flat:
    test   [maskq+0], mask_bitsd            ; vmask[0]
    jz .no_filter

    call mangle(private_prefix %+ _lpf_h_sb_y_16bpc_avx2).h4
    jmp .end

.no_filter:
    lea         dstq, [dstq+strideq*8]
    lea         dstq, [dstq+strideq*8]
.end:
    pslld        m12, 4
    lea           lq, [lq+l_strideq*4]
    shl   mask_bitsd, 4
    sub           hd, 4
    jg .loop
    RET

%endif ; ARCH_X86_64
