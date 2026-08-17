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

%if ARCH_X86_64

SECTION_RODATA 32

pb_4x1_4x5_4x9_4x13: times 2 db 0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12
pb_7_1: times 16 db 7, 1
pb_3_1: times 16 db 3, 1
pb_2_1: times 16 db 2, 1
pb_m1_0: times 16 db -1, 0
pb_m1_1: times 16 db -1, 1
pb_m1_2: times 16 db -1, 2
pb_1: times 32 db 1
pb_2: times 32 db 2
pb_3: times 32 db 3
pb_4: times 32 db 4
pb_16: times 32 db 16
pb_63: times 32 db 63
pb_64: times 32 db 64
pb_128: times 32 db 0x80
pb_129: times 32 db 0x81
pb_240: times 32 db 0xf0
pb_248: times 32 db 0xf8
pb_254: times 32 db 0xfe

pw_2048: times 16 dw 2048
pw_4096: times 16 dw 4096

pb_mask: dd 1, 2, 4, 8, 16, 32, 64, 128

SECTION .text

%macro ABSSUB 4 ; dst, a, b, tmp
    psubusb       %1, %2, %3
    psubusb       %4, %3, %2
    por           %1, %4
%endmacro

%macro TRANSPOSE_16x4_AND_WRITE_4x32 5
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
    movd [dstq+strideq*0-2], xm%4
    pextrd [dstq+strideq*1-2], xm%4, 1
    pextrd [dstq+strideq*2-2], xm%4, 2
    pextrd [dstq+stride3q-2], xm%4, 3
    lea         dstq, [dstq+strideq*4]
    movd [dstq+strideq*0-2], xm%5
    pextrd [dstq+strideq*1-2], xm%5, 1
    pextrd [dstq+strideq*2-2], xm%5, 2
    pextrd [dstq+stride3q-2], xm%5, 3
    lea         dstq, [dstq+strideq*4]
    movd [dstq+strideq*0-2], xm%2
    pextrd [dstq+strideq*1-2], xm%2, 1
    pextrd [dstq+strideq*2-2], xm%2, 2
    pextrd [dstq+stride3q-2], xm%2, 3
    lea         dstq, [dstq+strideq*4]
    movd [dstq+strideq*0-2], xm%1
    pextrd [dstq+strideq*1-2], xm%1, 1
    pextrd [dstq+strideq*2-2], xm%1, 2
    pextrd [dstq+stride3q-2], xm%1, 3
    lea         dstq, [dstq+strideq*4]

    vextracti128 xm%4, m%4, 1
    vextracti128 xm%5, m%5, 1
    vextracti128 xm%2, m%2, 1
    vextracti128 xm%1, m%1, 1

    movd [dstq+strideq*0-2], xm%4
    pextrd [dstq+strideq*1-2], xm%4, 1
    pextrd [dstq+strideq*2-2], xm%4, 2
    pextrd [dstq+stride3q-2], xm%4, 3
    lea         dstq, [dstq+strideq*4]
    movd [dstq+strideq*0-2], xm%5
    pextrd [dstq+strideq*1-2], xm%5, 1
    pextrd [dstq+strideq*2-2], xm%5, 2
    pextrd [dstq+stride3q-2], xm%5, 3
    lea         dstq, [dstq+strideq*4]
    movd [dstq+strideq*0-2], xm%2
    pextrd [dstq+strideq*1-2], xm%2, 1
    pextrd [dstq+strideq*2-2], xm%2, 2
    pextrd [dstq+stride3q-2], xm%2, 3
    lea         dstq, [dstq+strideq*4]
    movd [dstq+strideq*0-2], xm%1
    pextrd [dstq+strideq*1-2], xm%1, 1
    pextrd [dstq+strideq*2-2], xm%1, 2
    pextrd [dstq+stride3q-2], xm%1, 3
    lea         dstq, [dstq+strideq*4]
%endmacro

%macro TRANSPOSE_16X16B 3 ; in_load_15_from_mem, out_store_0_in_mem, mem
%if %1 == 0
    mova          %3, m15
%endif

    ; input in m0-15
    punpcklbw    m15, m0, m1
    punpckhbw     m0, m1
    punpcklbw     m1, m2, m3
    punpckhbw     m2, m3
    punpcklbw     m3, m4, m5
    punpckhbw     m4, m5
    punpcklbw     m5, m6, m7
    punpckhbw     m6, m7
    punpcklbw     m7, m8, m9
    punpckhbw     m8, m9
    punpcklbw     m9, m10, m11
    punpckhbw    m10, m11
    punpcklbw    m11, m12, m13
    punpckhbw    m12, m13
    mova         m13, %3
    mova          %3, m12
    punpcklbw    m12, m14, m13
    punpckhbw    m13, m14, m13

    ; interleaved in m15,0,1,2,3,4,5,6,7,8,9,10,11,rsp%3,12,13
    punpcklwd    m14, m15, m1
    punpckhwd    m15, m1
    punpcklwd     m1, m0, m2
    punpckhwd     m0, m2
    punpcklwd     m2, m3, m5
    punpckhwd     m3, m5
    punpcklwd     m5, m4, m6
    punpckhwd     m4, m6
    punpcklwd     m6, m7, m9
    punpckhwd     m7, m9
    punpcklwd     m9, m8, m10
    punpckhwd     m8, m10
    punpcklwd    m10, m11, m12
    punpckhwd    m11, m12
    mova         m12, %3
    mova          %3, m11
    punpcklwd    m11, m12, m13
    punpckhwd    m12, m13

    ; interleaved in m14,15,1,0,2,3,5,4,6,7,9,8,10,rsp%3,11,12
    punpckldq    m13, m14, m2
    punpckhdq    m14, m2
    punpckldq     m2, m15, m3
    punpckhdq    m15, m3
    punpckldq     m3, m1, m5
    punpckhdq     m1, m5
    punpckldq     m5, m0, m4
    punpckhdq     m0, m4
    punpckldq     m4, m6, m10
    punpckhdq     m6, m10
    punpckldq    m10, m9, m11
    punpckhdq     m9, m11
    punpckldq    m11, m8, m12
    punpckhdq     m8, m12
    mova         m12, %3
    mova          %3, m8
    punpckldq     m8, m7, m12
    punpckhdq     m7, m12

    ; interleaved in m13,14,2,15,3,1,5,0,4,6,8,7,10,9,11,rsp%3
    punpcklqdq   m12, m13, m4
    punpckhqdq   m13, m4
    punpcklqdq    m4, m14, m6
    punpckhqdq   m14, m6
    punpcklqdq    m6, m2, m8
    punpckhqdq    m2, m8
    punpcklqdq    m8, m15, m7
    punpckhqdq   m15, m7
    punpcklqdq    m7, m3, m10
    punpckhqdq    m3, m10
    punpcklqdq   m10, m1, m9
    punpckhqdq    m1, m9
    punpcklqdq    m9, m5, m11
    punpckhqdq    m5, m11
    mova         m11, %3
    mova          %3, m12
    punpcklqdq   m12, m0, m11
    punpckhqdq    m0, m11
%if %2 == 0
    mova         m11, %3
%endif

    ; interleaved m11,13,4,14,6,2,8,15,7,3,10,1,9,5,12,0
    SWAP          0, 11, 1, 13, 5, 2, 4, 6, 8, 7, 15
    SWAP          3, 14, 12, 9
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
%if %1 != 6
    mova         m12, [tmpq+strideq*0]
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
%endif
%else
    ; load lines
%if %1 == 4
    movd         xm3, [dstq+strideq*0-2]
    movd         xm4, [dstq+strideq*1-2]
    movd         xm5, [dstq+strideq*2-2]
    movd         xm6, [dstq+stride3q -2]
    lea         tmpq, [dstq+strideq*4]
    pinsrd       xm3, [tmpq+strideq*0-2], 2
    pinsrd       xm4, [tmpq+strideq*1-2], 2
    pinsrd       xm5, [tmpq+strideq*2-2], 2
    pinsrd       xm6, [tmpq+stride3q -2], 2
    lea         tmpq, [tmpq+strideq*4]
    pinsrd       xm3, [tmpq+strideq*0-2], 1
    pinsrd       xm4, [tmpq+strideq*1-2], 1
    pinsrd       xm5, [tmpq+strideq*2-2], 1
    pinsrd       xm6, [tmpq+stride3q -2], 1
    lea         tmpq, [tmpq+strideq*4]
    pinsrd       xm3, [tmpq+strideq*0-2], 3
    pinsrd       xm4, [tmpq+strideq*1-2], 3
    pinsrd       xm5, [tmpq+strideq*2-2], 3
    pinsrd       xm6, [tmpq+stride3q -2], 3
    lea         tmpq, [tmpq+strideq*4]
    movd        xm12, [tmpq+strideq*0-2]
    movd        xm13, [tmpq+strideq*1-2]
    movd        xm14, [tmpq+strideq*2-2]
    movd        xm15, [tmpq+stride3q -2]
    lea         tmpq, [tmpq+strideq*4]
    pinsrd      xm12, [tmpq+strideq*0-2], 2
    pinsrd      xm13, [tmpq+strideq*1-2], 2
    pinsrd      xm14, [tmpq+strideq*2-2], 2
    pinsrd      xm15, [tmpq+stride3q -2], 2
    lea         tmpq, [tmpq+strideq*4]
    pinsrd      xm12, [tmpq+strideq*0-2], 1
    pinsrd      xm13, [tmpq+strideq*1-2], 1
    pinsrd      xm14, [tmpq+strideq*2-2], 1
    pinsrd      xm15, [tmpq+stride3q -2], 1
    lea         tmpq, [tmpq+strideq*4]
    pinsrd      xm12, [tmpq+strideq*0-2], 3
    pinsrd      xm13, [tmpq+strideq*1-2], 3
    pinsrd      xm14, [tmpq+strideq*2-2], 3
    pinsrd      xm15, [tmpq+stride3q -2], 3
    vinserti128   m3, xm12, 1
    vinserti128   m4, xm13, 1
    vinserti128   m5, xm14, 1
    vinserti128   m6, xm15, 1

    ; transpose 4x16
    ; xm3: A-D0,A-D8,A-D4,A-D12
    ; xm4: A-D1,A-D9,A-D5,A-D13
    ; xm5: A-D2,A-D10,A-D6,A-D14
    ; xm6: A-D3,A-D11,A-D7,A-D15
    punpcklbw     m7, m3, m4
    punpckhbw     m3, m4
    punpcklbw     m4, m5, m6
    punpckhbw     m5, m6
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
    punpckhqdq    m4, m5, m4
    punpcklqdq    m5, m6, m7
    punpckhqdq    m6, m7
    ; xm3: A0-15
    ; xm5: B0-15
    ; xm4: C0-15
    ; xm6: D0-15
%elif %1 == 6 || %1 == 8
    movq         xm3, [dstq+strideq*0-%1/2]
    movq         xm4, [dstq+strideq*1-%1/2]
    movq         xm5, [dstq+strideq*2-%1/2]
    movq         xm6, [dstq+stride3q -%1/2]
    lea         tmpq, [dstq+strideq*8]
    movhps       xm3, [tmpq+strideq*0-%1/2]
    movhps       xm4, [tmpq+strideq*1-%1/2]
    movhps       xm5, [tmpq+strideq*2-%1/2]
    movhps       xm6, [tmpq+stride3q -%1/2]
    lea         tmpq, [tmpq+strideq*8]
    movq         xm7, [tmpq+strideq*0-%1/2]
    movq         xm8, [tmpq+strideq*1-%1/2]
    movq         xm9, [tmpq+strideq*2-%1/2]
    movq        xm11, [tmpq+stride3q -%1/2]
    lea         tmpq, [tmpq+strideq*8]
    movhps       xm7, [tmpq+strideq*0-%1/2]
    movhps       xm8, [tmpq+strideq*1-%1/2]
    movhps       xm9, [tmpq+strideq*2-%1/2]
    movhps      xm11, [tmpq+stride3q -%1/2]
    vinserti128   m3, xm7, 1
    vinserti128   m4, xm8, 1
    vinserti128   m5, xm9, 1
    vinserti128   m6, xm11, 1
    lea         tmpq, [dstq+strideq*4]
    movq        xm12, [tmpq+strideq*0-%1/2]
    movq        xm13, [tmpq+strideq*1-%1/2]
    movq        xm14, [tmpq+strideq*2-%1/2]
    movq        xm15, [tmpq+stride3q -%1/2]
    lea         tmpq, [tmpq+strideq*8]
    movhps      xm12, [tmpq+strideq*0-%1/2]
    movhps      xm13, [tmpq+strideq*1-%1/2]
    movhps      xm14, [tmpq+strideq*2-%1/2]
    movhps      xm15, [tmpq+stride3q -%1/2]
    lea         tmpq, [tmpq+strideq*8]
    movq         xm7, [tmpq+strideq*0-%1/2]
    movq         xm8, [tmpq+strideq*1-%1/2]
    movq         xm9, [tmpq+strideq*2-%1/2]
    movq        xm11, [tmpq+stride3q -%1/2]
    lea         tmpq, [tmpq+strideq*8]
    movhps       xm7, [tmpq+strideq*0-%1/2]
    movhps       xm8, [tmpq+strideq*1-%1/2]
    movhps       xm9, [tmpq+strideq*2-%1/2]
    movhps      xm11, [tmpq+stride3q -%1/2]
    vinserti128  m12, xm7, 1
    vinserti128  m13, xm8, 1
    vinserti128  m14, xm9, 1
    vinserti128  m15, xm11, 1

    ; transpose 8x16
    ; xm3: A-H0,A-H8
    ; xm4: A-H1,A-H9
    ; xm5: A-H2,A-H10
    ; xm6: A-H3,A-H11
    ; xm12: A-H4,A-H12
    ; xm13: A-H5,A-H13
    ; xm14: A-H6,A-H14
    ; xm15: A-H7,A-H15
    punpcklbw    m7, m3, m4
    punpckhbw    m3, m4
    punpcklbw    m4, m5, m6
    punpckhbw    m5, m6
    punpcklbw    m6, m12, m13
    punpckhbw   m12, m13
    punpcklbw   m13, m14, m15
    punpckhbw   m14, m15
    ; xm7: A0-1,B0-1,C0-1,D0-1,E0-1,F0-1,G0-1,H0-1
    ; xm3: A8-9,B8-9,C8-9,D8-9,E8-9,F8-9,G8-9,H8-9
    ; xm4: A2-3,B2-3,C2-3,D2-3,E2-3,F2-3,G2-3,H2-3
    ; xm5: A10-11,B10-11,C10-11,D10-11,E10-11,F10-11,G10-11,H10-11
    ; xm6: A4-5,B4-5,C4-5,D4-5,E4-5,F4-5,G4-5,H4-5
    ; xm12: A12-13,B12-13,C12-13,D12-13,E12-13,F12-13,G12-13,H12-13
    ; xm13: A6-7,B6-7,C6-7,D6-7,E6-7,F6-7,G6-7,H6-7
    ; xm14: A14-15,B14-15,C14-15,D14-15,E14-15,F14-15,G14-15,H14-15
    punpcklwd   m15, m7, m4
    punpckhwd    m7, m4
    punpcklwd    m4, m3, m5
    punpckhwd    m3, m5
    punpcklwd    m5, m6, m13
    punpckhwd    m6, m13
    punpcklwd   m13, m12, m14
    punpckhwd   m12, m14
    ; xm15: A0-3,B0-3,C0-3,D0-3
    ; xm7: E0-3,F0-3,G0-3,H0-3
    ; xm4: A8-11,B8-11,C8-11,D8-11
    ; xm3: E8-11,F8-11,G8-11,H8-11
    ; xm5: A4-7,B4-7,C4-7,D4-7
    ; xm6: E4-7,F4-7,G4-7,H4-7
    ; xm13: A12-15,B12-15,C12-15,D12-15
    ; xm12: E12-15,F12-15,G12-15,H12-15
    punpckldq   m14, m15, m5
    punpckhdq   m15, m5
    punpckldq    m5, m7, m6
%if %1 != 6
    punpckhdq    m7, m6
%endif
    punpckldq    m6, m4, m13
    punpckhdq    m4, m13
    punpckldq   m13, m3, m12
%if %1 != 6
    punpckhdq   m12, m3, m12
%endif
    ; xm14: A0-7,B0-7
    ; xm15: C0-7,D0-7
    ; xm5: E0-7,F0-7
    ; xm7: G0-7,H0-7
    ; xm6: A8-15,B8-15
    ; xm4: C8-15,D8-15
    ; xm13: E8-15,F8-15
    ; xm12: G8-15,H8-15
    punpcklqdq   m3, m14, m6
    punpckhqdq  m14, m6
    punpckhqdq   m6, m15, m4
    punpcklqdq  m15, m4
    punpcklqdq   m4, m5, m13
    punpckhqdq  m13, m5, m13
%if %1 == 8
    punpcklqdq   m5, m7, m12
    punpckhqdq  m12, m7, m12
    ; xm3: A0-15
    ; xm14: B0-15
    ; xm15: C0-15
    ; xm6: D0-15
    ; xm4: E0-15
    ; xm13: F0-15
    ; xm5: G0-15
    ; xm12: H0-15
    SWAP         12, 3, 15
    SWAP         13, 14, 5, 4, 6
    ; 3,14,15,6,4,13,5,12 -> 12,13,3,4,5,6,14,15
%else
    SWAP         13, 3, 14
    SWAP          6, 4, 15, 5
    ; 3,14,15,6,4,13 -> 13,3,4,5,6,14
%endif
%else
    ; load and 16x16 transpose. We only use 14 pixels but we'll need the
    ; remainder at the end for the second transpose
    movu         xm0, [dstq+strideq*0-8]
    movu         xm1, [dstq+strideq*1-8]
    movu         xm2, [dstq+strideq*2-8]
    movu         xm3, [dstq+stride3q -8]
    lea         tmpq, [dstq+strideq*4]
    movu         xm4, [tmpq+strideq*0-8]
    movu         xm5, [tmpq+strideq*1-8]
    movu         xm6, [tmpq+strideq*2-8]
    movu         xm7, [tmpq+stride3q -8]
    lea         tmpq, [tmpq+strideq*4]
    movu         xm8, [tmpq+strideq*0-8]
    movu         xm9, [tmpq+strideq*1-8]
    movu        xm10, [tmpq+strideq*2-8]
    movu        xm11, [tmpq+stride3q -8]
    lea         tmpq, [tmpq+strideq*4]
    movu        xm12, [tmpq+strideq*0-8]
    movu        xm13, [tmpq+strideq*1-8]
    movu        xm14, [tmpq+strideq*2-8]
    movu        xm15, [tmpq+stride3q -8]
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m0, [tmpq+strideq*0-8], 1
    vinserti128   m1, [tmpq+strideq*1-8], 1
    vinserti128   m2, [tmpq+strideq*2-8], 1
    vinserti128   m3, [tmpq+stride3q -8], 1
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m4, [tmpq+strideq*0-8], 1
    vinserti128   m5, [tmpq+strideq*1-8], 1
    vinserti128   m6, [tmpq+strideq*2-8], 1
    vinserti128   m7, [tmpq+stride3q -8], 1
    lea         tmpq, [tmpq+strideq*4]
    vinserti128   m8, [tmpq+strideq*0-8], 1
    vinserti128   m9, [tmpq+strideq*1-8], 1
    vinserti128  m10, [tmpq+strideq*2-8], 1
    vinserti128  m11, [tmpq+stride3q -8], 1
    lea         tmpq, [tmpq+strideq*4]
    vinserti128  m12, [tmpq+strideq*0-8], 1
    vinserti128  m13, [tmpq+strideq*1-8], 1
    vinserti128  m14, [tmpq+strideq*2-8], 1
    vinserti128  m15, [tmpq+stride3q -8], 1

    TRANSPOSE_16X16B 0, 1, [rsp+11*32]
    mova  [rsp+12*32], m1
    mova  [rsp+13*32], m2
    mova  [rsp+14*32], m3
    mova  [rsp+15*32], m12
    mova  [rsp+16*32], m13
    mova  [rsp+17*32], m14
    mova  [rsp+18*32], m15
    ; 4,5,6,7,8,9,10,11 -> 12,13,3,4,5,6,14,15
    SWAP           12, 4, 7
    SWAP           13, 5, 8
    SWAP            3, 6, 9
    SWAP           10, 14
    SWAP           11, 15
%endif
%endif

    ; load L/E/I/H
%ifidn %2, v
    movu          m1, [lq]
    movu          m0, [lq+l_strideq]
%else
    movq         xm1, [lq]
    movq         xm2, [lq+l_strideq*2]
    movhps       xm1, [lq+l_strideq]
    movhps       xm2, [lq+l_stride3q]
    lea           lq, [lq+l_strideq*4]
    movq        xm10, [lq]
    movq         xm0, [lq+l_strideq*2]
    movhps      xm10, [lq+l_strideq]
    movhps       xm0, [lq+l_stride3q]
    lea           lq, [lq+l_strideq*4]
    vinserti128   m1, xm10, 1
    vinserti128   m2, xm0, 1
    shufps        m0, m1, m2, q3131
    shufps        m1, m2, q2020
%endif
    pxor          m2, m2
    pcmpeqb      m10, m2, m0
    pand          m1, m10
    por           m0, m1                        ; l[x][] ? l[x][] : l[x-stride][]
    pshufb        m0, [pb_4x1_4x5_4x9_4x13]     ; l[x][1]
    pcmpeqb      m10, m2, m0                    ; !L
    psrlq         m2, m0, [lutq+128]
    pand          m2, [pb_63]
    vpbroadcastb  m1, [lutq+136]
    pminub        m2, m1
    pmaxub        m2, [pb_1]                    ; I
    pand          m1, m0, [pb_240]
    psrlq         m1, 4                         ; H
    paddb         m0, [pb_2]
    paddb         m0, m0
    paddb         m0, m2                        ; E
    pxor          m1, [pb_128]
    pxor          m2, [pb_128]
    pxor          m0, [pb_128]

    ABSSUB        m8, m3, m4, m9                ; abs(p1-p0)
    pmaxub        m8, m10
    ABSSUB        m9, m5, m6, m10               ; abs(q1-q0)
    pmaxub        m8, m9
%if %1 == 4
    pxor          m8, [pb_128]
    pcmpgtb       m7, m8, m1                    ; hev
%else
    pxor          m7, m8, [pb_128]
    pcmpgtb       m7, m1                        ; hev

%if %1 == 6
    ABSSUB        m9, m13, m4, m10              ; abs(p2-p0)
    pmaxub        m9, m8
%else
    ABSSUB        m9, m12, m4, m10              ; abs(p3-p0)
    pmaxub        m9, m8
    ABSSUB       m10, m13, m4, m11              ; abs(p2-p0)
    pmaxub        m9, m10
%endif
    ABSSUB       m10, m5,  m14, m11             ; abs(q2-q0)
    pmaxub        m9, m10
%if %1 != 6
    ABSSUB       m10, m5,  m15, m11             ; abs(q3-q0)
    pmaxub        m9, m10
%endif
    pxor          m9, [pb_128]
    pcmpgtb       m9, [pb_129]                  ; !flat8in

%if %1 == 6
    ABSSUB       m10, m13, m3,  m1              ; abs(p2-p1)
%else
    ABSSUB       m10, m12, m13, m11             ; abs(p3-p2)
    ABSSUB       m11, m13, m3,  m1              ; abs(p2-p1)
    pmaxub       m10, m11
    ABSSUB       m11, m14, m15, m1              ; abs(q3-q2)
    pmaxub       m10, m11
%endif
    ABSSUB       m11, m14, m6,  m1              ; abs(q2-q1)
    pmaxub       m10, m11
%if %1 == 16
    vpbroadcastd m11, [maskq+8]
    vpbroadcastd  m1, [maskq+4]
    por          m11, m1
    pand         m11, [pb_mask]
    pcmpeqd      m11, [pb_mask]
    pand         m10, m11
%else
    vpbroadcastd m11, [maskq+4]
    pand         m11, [pb_mask]
    pcmpeqd      m11, [pb_mask]
    pand         m10, m11                       ; only apply fm-wide to wd>4 blocks
%endif
    pmaxub        m8, m10

    pxor          m8, [pb_128]
%endif
    pcmpgtb       m8, m2

    ABSSUB       m10, m3, m6, m11               ; abs(p1-q1)
    ABSSUB       m11, m4, m5, m2                ; abs(p0-q0)
    paddusb      m11, m11
    pand         m10, [pb_254]
    psrlq        m10, 1
    paddusb      m10, m11                       ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    pxor         m10, [pb_128]
    pcmpgtb      m10, m0                        ; abs(p0-q0)*2+(abs(p1-q1)>>1) > E
    por           m8, m10

%if %1 == 16
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]
%else
    mova          m0, [rsp+12*32]
%endif
    ABSSUB        m1, m0, m4, m2
%ifidn %2, v
    mova          m0, [tmpq+strideq*2]
%else
    mova          m0, [rsp+13*32]
%endif
    ABSSUB        m2, m0, m4, m10
    pmaxub        m1, m2
%ifidn %2, v
    mova          m0, [tmpq+stride3q]
%else
    mova          m0, [rsp+14*32]
%endif
    ABSSUB        m2, m0, m4, m10
    pmaxub        m1, m2
%ifidn %2, v
    lea         tmpq, [dstq+strideq*4]
    mova          m0, [tmpq+strideq*0]
%else
    mova          m0, [rsp+15*32]
%endif
    ABSSUB        m2, m0, m5, m10
    pmaxub        m1, m2
%ifidn %2, v
    mova          m0, [tmpq+strideq*1]
%else
    mova          m0, [rsp+16*32]
%endif
    ABSSUB        m2, m0, m5, m10
    pmaxub        m1, m2
%ifidn %2, v
    mova          m0, [tmpq+strideq*2]
%else
    mova          m0, [rsp+17*32]
%endif
    ABSSUB        m2, m0, m5, m10
    pmaxub        m1, m2
    pxor          m1, [pb_128]
    pcmpgtb       m1, [pb_129]                  ; !flat8out
    por           m1, m9                        ; !flat8in | !flat8out
    vpbroadcastd  m2, [maskq+8]
    pand         m10, m2, [pb_mask]
    pcmpeqd      m10, [pb_mask]
    pandn         m1, m10                       ; flat16
    pandn         m1, m8, m1                    ; flat16 & fm

    vpbroadcastd m10, [maskq+4]
    por          m10, m2
    pand          m2, m10, [pb_mask]
    pcmpeqd       m2, [pb_mask]
    pandn         m9, m2                        ; flat8in
    pandn         m9, m8, m9
    vpbroadcastd  m2, [maskq+0]
    por           m2, m10
    pand          m2, [pb_mask]
    pcmpeqd       m2, [pb_mask]
    pandn         m8, m2
    pandn         m8, m9, m8                    ; fm & !flat8 & !flat16
    pandn         m9, m1, m9                    ; flat8 & !flat16
%elif %1 != 4
    vpbroadcastd  m0, [maskq+4]
    pand          m2, m0, [pb_mask]
    pcmpeqd       m2, [pb_mask]
    pandn         m9, m2
    pandn         m9, m8, m9                    ; flat8 & fm
    vpbroadcastd  m2, [maskq+0]
    por           m0, m2
    pand          m0, [pb_mask]
    pcmpeqd       m0, [pb_mask]
    pandn         m8, m0
    pandn         m8, m9, m8                    ; fm & !flat8
%else
    vpbroadcastd  m0, [maskq+0]
    pand          m0, [pb_mask]
    pcmpeqd       m0, [pb_mask]
    pandn         m8, m0                        ; fm
%endif

    ; short filter

    pxor          m3, [pb_128]
    pxor          m6, [pb_128]
    psubsb       m10, m3, m6                    ; iclip_diff(p1-q1)
    pand         m10, m7                        ; f=iclip_diff(p1-q1)&hev
    pxor          m4, [pb_128]
    pxor          m5, [pb_128]
    psubsb       m11, m5, m4
    paddsb       m10, m11
    paddsb       m10, m11
    paddsb       m10, m11                       ; f=iclip_diff(3*(q0-p0)+f)
    pand          m8, m10                       ; f&=fm
    paddsb       m10, m8, [pb_3]
    paddsb        m8, [pb_4]
    pand         m10, [pb_248]
    pand          m8, [pb_248]
    psrlq        m10, 3
    psrlq         m8, 3
    pxor         m10, [pb_16]
    pxor          m8, [pb_16]
    psubb        m10, [pb_16]                   ; f2
    psubb         m8, [pb_16]                   ; f1
    paddsb        m4, m10
    psubsb        m5, m8
    pxor          m4, [pb_128]
    pxor          m5, [pb_128]

    pxor          m8, [pb_128]
    pxor         m10, m10
    pavgb         m8, m10                       ; f=(f1+1)>>1
    psubb         m8, [pb_64]
    pandn         m8, m7, m8                    ; f&=!hev
    paddsb        m3, m8
    psubsb        m6, m8
    pxor          m3, [pb_128]
    pxor          m6, [pb_128]

%if %1 == 16
    ; flat16 filter
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*8]
    mova          m0, [tmpq+strideq*1]          ; p6
    mova          m2, [tmpq+strideq*2]          ; p5
    mova          m7, [tmpq+stride3q]           ; p4
%else
    mova          m0, [rsp+12*32]
    mova          m2, [rsp+13*32]
    mova          m7, [rsp+14*32]
%endif

    mova  [rsp+0*32], m9
    mova  [rsp+1*32], m14
    mova  [rsp+2*32], m15

    ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0 [p5/p4/p2/p1/p0/q0][p6/p3] A
    ; write -6
    punpcklbw    m14, m0, m12
    punpckhbw    m15, m0, m12
    pmaddubsw    m10, m14, [pb_7_1]
    pmaddubsw    m11, m15, [pb_7_1]             ; p6*7+p3
    punpcklbw     m8, m2, m7
    punpckhbw     m9, m2, m7
    pmaddubsw     m8, [pb_2]
    pmaddubsw     m9, [pb_2]
    paddw        m10, m8
    paddw        m11, m9                        ; p6*7+p5*2+p4*2+p3
    punpcklbw     m8, m13, m3
    punpckhbw     m9, m13, m3
    pmaddubsw     m8, [pb_1]
    pmaddubsw     m9, [pb_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p6*7+p5*2+p4*2+p3+p2+p1
    punpcklbw     m8, m4, m5
    punpckhbw     m9, m4, m5
    pmaddubsw     m8, [pb_1]
    pmaddubsw     m9, [pb_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
    pand          m8, m1
    pandn         m9, m1, m2
    por           m8, m9
%ifidn %2, v
    mova [tmpq+strideq*2], m8                   ; p5
%else
    mova [rsp+13*32], m8
%endif

    ; sub p6*2, add p3/q1 [reuse p6/p3 from A][-p6,+q1|save] B
    ; write -5
    pmaddubsw    m14, [pb_m1_1]
    pmaddubsw    m15, [pb_m1_1]
    paddw        m10, m14
    paddw        m11, m15                       ; p6*6+p5*2+p4*2+p3*2+p2+p1+p0+q0
    punpcklbw     m8, m0, m6
    punpckhbw     m9, m0, m6
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    mova  [rsp+3*32], m8
    mova  [rsp+4*32], m9
    paddw        m10, m8
    paddw        m11, m9                        ; p6*5+p5*2+p4*2+p3*2+p2+p1+p0+q0+q1
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
    vpblendvb     m8, m7, m8, m1
%ifidn %2, v
    mova [tmpq+stride3q], m8                    ; p4
%else
    mova [rsp+14*32], m8
%endif

    ; sub p6/p5, add p2/q2 [-p6,+p2][-p5,+q2|save] C
    ; write -4
    mova         m14, [rsp+1*32]
    punpcklbw     m8, m0, m13
    punpckhbw     m9, m0, m13
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p6*4+p5*2+p4*2+p3*2+p2*2+p1+p0+q0+q1
    punpcklbw     m8, m2, m14
    punpckhbw     m2, m14
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m2, [pb_m1_1]
    mova  [rsp+1*32], m8
    paddw        m10, m8
    paddw        m11, m2                        ; p6*4+p5+p4*2+p3*2+p2*2+p1+p0+q0+q1+q2
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
    vpblendvb     m8, m12, m8, m1
%ifidn %2, v
    mova [tmpq+strideq*4], m8                   ; p3
%else
    mova [rsp+19*32], m8
%endif

    ; sub p6/p4, add p1/q3 [-p6,+p1][-p4,+q3|save] D
    ; write -3
    mova         m15, [rsp+2*32]
    punpcklbw     m8, m0, m3
    punpckhbw     m9, m0, m3
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p6*3+p5+p4*2+p3*2+p2*2+p1*2+p0+q0+q1+q2
    punpcklbw     m8, m7, m15
    punpckhbw     m7, m15
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m7, [pb_m1_1]
    mova  [rsp+2*32], m8
    paddw        m10, m8
    paddw        m11, m7                        ; p6*3+p5+p4+p3*2+p2*2+p1*2+p0+q0+q1+q2+q3
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
    vpblendvb     m8, m13, m8, m1
    mova  [rsp+6*32], m8                        ; don't clobber p2/m13 since we need it in F

    ; sub p6/p3, add p0/q4 [-p6,+p0][-p3,+q4|save] E
    ; write -2
%ifidn %2, v
    lea         tmpq, [dstq+strideq*4]
%endif
    punpcklbw     m8, m0, m4
    punpckhbw     m9, m0, m4
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p6*2+p5+p4+p3*2+p2*2+p1*2+p0*2+q0+q1+q2+q3
%ifidn %2, v
    mova          m9, [tmpq+strideq*0]          ; q4
%else
    mova          m9, [rsp+15*32]
%endif
    punpcklbw     m8, m12, m9
    punpckhbw     m9, m12, m9
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    mova  [rsp+7*32], m8
    mova  [rsp+5*32], m9
    paddw        m10, m8
    paddw        m11, m9                        ; p6*2+p5+p4+p3+p2*2+p1*2+p0*2+q0+q1+q2+q3+q4
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
    vpblendvb     m8, m3, m8, m1
    mova  [rsp+8*32], m8                        ; don't clobber p1/m3 since we need it in G

    ; sub p6/p2, add q0/q5 [-p6,+q0][-p2,+q5|save] F
    ; write -1
%ifidn %2, v
    mova          m9, [tmpq+strideq*1]          ; q5
%else
    mova          m9, [rsp+16*32]
%endif
    punpcklbw     m8, m0, m5
    punpckhbw     m0, m5
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m0, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m0                        ; p6+p5+p4+p3+p2*2+p1*2+p0*2+q0*2+q1+q2+q3+q4
    punpcklbw     m0, m13, m9
    punpckhbw     m9, m13, m9
    mova         m13, [rsp+6*32]
    pmaddubsw     m0, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    mova [rsp+ 9*32], m0
    mova [rsp+10*32], m9
    paddw        m10, m0
    paddw        m11, m9                        ; p6+p5+p4+p3+p2+p1*2+p0*2+q0*2+q1+q2+q3+q4+q5
    pmulhrsw      m0, m10, [pw_2048]
    pmulhrsw      m8, m11, [pw_2048]
    packuswb      m0, m8
    vpblendvb     m0, m4, m0, m1
    mova  [rsp+6*32], m0                        ; don't clobber p0/m4 since we need it in H

    ; sub p6/p1, add q1/q6 [reuse -p6,+q1 from B][-p1,+q6|save] G
    ; write +0
%ifidn %2, v
    mova          m0, [tmpq+strideq*2]          ; q6
%else
    mova          m0, [rsp+17*32]
%endif
    paddw        m10, [rsp+3*32]
    paddw        m11, [rsp+4*32]                ; p5+p4+p3+p2+p1*2+p0*2+q0*2+q1*2+q2+q3+q4+q5
    punpcklbw     m8, m3, m0
    punpckhbw     m9, m3, m0
    mova          m3, [rsp+8*32]
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    mova  [rsp+3*32], m8
    mova  [rsp+4*32], m9
    paddw        m10, m8
    paddw        m11, m9                        ; p5+p4+p3+p2+p1+p0*2+q0*2+q1*2+q2+q3+q4+q5+q6
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
    vpblendvb     m8, m5, m8, m1
    mova  [rsp+8*32], m8                        ; don't clobber q0/m5 since we need it in I

    ; sub p5/p0, add q2/q6 [reuse -p5,+q2 from C][-p0,+q6] H
    ; write +1
    paddw        m10, [rsp+1*32]
    paddw        m11, m2                        ; p4+p3+p2+p1+p0*2+q0*2+q1*2+q2*2+q3+q4+q5+q6
    punpcklbw     m8, m4, m0
    punpckhbw     m2, m4, m0
    mova          m4, [rsp+6*32]
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m2, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m2                        ; p4+p3+p2+p1+p0+q0*2+q1*2+q2*2+q3+q4+q5+q6*2
    pmulhrsw      m2, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m2, m9
    vpblendvb     m2, m6, m2, m1                ; don't clobber q1/m6 since we need it in K

    ; sub p4/q0, add q3/q6 [reuse -p4,+q3 from D][-q0,+q6] I
    ; write +2
    paddw        m10, [rsp+2*32]
    paddw        m11, m7                        ; p3+p2+p1+p0+q0*2+q1*2+q2*2+q3*2+q4+q5+q6*2
    punpcklbw     m8, m5, m0
    punpckhbw     m9, m5, m0
    mova          m5, [rsp+8*32]
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p3+p2+p1+p0+q0+q1*2+q2*2+q3*2+q4+q5+q6*3
    pmulhrsw      m7, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m7, m9
    vpblendvb     m7, m14, m7, m1               ; don't clobber q2/m14 since we need it in K

    ; sub p3/q1, add q4/q6 [reuse -p3,+q4 from E][-q1,+q6] J
    ; write +3
    paddw        m10, [rsp+7*32]
    paddw        m11, [rsp+5*32]                ; p2+p1+p0+q0+q1*2+q2*2+q3*2+q4*2+q5+q6*3
    punpcklbw     m8, m6, m0
    punpckhbw     m9, m6, m0
    SWAP           2, 6
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p2+p1+p0+q0+q1+q2*2+q3*2+q4*2+q5+q6*4
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
    vpblendvb     m8, m15, m8, m1
%ifidn %2, v
    mova [tmpq+mstrideq], m8                    ; q3
%else
    mova [rsp+20*32], m8
%endif

    ; sub p2/q2, add q5/q6 [reuse -p2,+q5 from F][-q2,+q6] K
    ; write +4
    paddw        m10, [rsp+ 9*32]
    paddw        m11, [rsp+10*32]               ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
    punpcklbw     m8, m14, m0
    punpckhbw     m9, m14, m0
    SWAP          14, 7
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
    pmulhrsw      m8, m10, [pw_2048]
    pmulhrsw      m9, m11, [pw_2048]
    packuswb      m8, m9
%ifidn %2, v
    mova          m9, [tmpq+strideq*0]
%else
    mova          m9, [rsp+15*32]
%endif
    vpblendvb     m8, m9, m8, m1
%ifidn %2, v
    mova [tmpq+strideq*0], m8                    ; q4
%else
    mova [rsp+15*32], m8
%endif

    ; sub p1/q3, add q6*2 [reuse -p1,+q6 from G][-q3,+q6] L
    ; write +5
    paddw        m10, [rsp+3*32]
    paddw        m11, [rsp+4*32]                ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
    punpcklbw     m8, m15, m0
    punpckhbw     m9, m15, m0
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw     m9, [pb_m1_1]
    paddw        m10, m8
    paddw        m11, m9                        ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
    pmulhrsw     m10, [pw_2048]
    pmulhrsw     m11, [pw_2048]
    packuswb     m10, m11
%ifidn %2, v
    mova         m11, [tmpq+strideq*1]
%else
    mova         m11, [rsp+16*32]
%endif
    vpblendvb    m10, m11, m10, m1
%ifidn %2, v
    mova [tmpq+strideq*1], m10                  ; q5
%else
    mova [rsp+16*32], m10
%endif

    mova          m9, [rsp+0*32]
%ifidn %2, v
    lea         tmpq, [dstq+mstrideq*4]
%endif
%endif
%if %1 >= 8
    ; flat8 filter
    punpcklbw     m0, m12, m3
    punpckhbw     m1, m12, m3
    pmaddubsw     m2, m0, [pb_3_1]
    pmaddubsw     m7, m1, [pb_3_1]              ; 3 * p3 + p1
    punpcklbw     m8, m13, m4
    punpckhbw    m11, m13, m4
    pmaddubsw     m8, [pb_2_1]
    pmaddubsw    m11, [pb_2_1]
    paddw         m2, m8
    paddw         m7, m11                       ; 3 * p3 + 2 * p2 + p1 + p0
    punpcklbw     m8, m5, [pb_4]
    punpckhbw    m11, m5, [pb_4]
    pmaddubsw     m8, [pb_1]
    pmaddubsw    m11, [pb_1]
    paddw         m2, m8
    paddw         m7, m11                       ; 3 * p3 + 2 * p2 + p1 + p0 + q0 + 4
    psrlw         m8, m2, 3
    psrlw        m11, m7, 3
    packuswb      m8, m11
    vpblendvb    m10, m13, m8, m9              ; p2
%ifidn %2, v
    mova [tmpq+strideq*1], m10                 ; p2
%endif

    pmaddubsw     m8, m0, [pb_m1_1]
    pmaddubsw    m11, m1, [pb_m1_1]
    paddw         m2, m8
    paddw         m7, m11
    punpcklbw     m8, m13, m6
    punpckhbw    m11, m13, m6
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw    m11, [pb_m1_1]
    paddw         m2, m8
    paddw         m7, m11                       ; 2 * p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4
    psrlw         m8, m2, 3
    psrlw        m11, m7, 3
    packuswb      m8, m11
    vpblendvb     m8, m3, m8, m9                ; p1
%ifidn %2, v
    mova [tmpq+strideq*2], m8                   ; p1
%else
    mova  [rsp+0*32], m8
%endif

    pmaddubsw     m0, [pb_1]
    pmaddubsw     m1, [pb_1]
    psubw         m2, m0
    psubw         m7, m1
    punpcklbw     m8, m4, m14
    punpckhbw    m11, m4, m14
    pmaddubsw     m8, [pb_1]
    pmaddubsw    m11, [pb_1]
    paddw         m2, m8
    paddw         m7, m11                       ; p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4
    psrlw         m8, m2, 3
    psrlw        m11, m7, 3
    packuswb      m8, m11
    vpblendvb     m8, m4, m8, m9                ; p0
%ifidn %2, v
    mova [tmpq+stride3q ], m8                   ; p0
%else
    mova  [rsp+1*32], m8
%endif

    punpcklbw     m0, m5, m15
    punpckhbw     m1, m5, m15
    pmaddubsw     m8, m0, [pb_1]
    pmaddubsw    m11, m1, [pb_1]
    paddw         m2, m8
    paddw         m7, m11
    punpcklbw     m8, m4, m12
    punpckhbw    m11, m4, m12
    pmaddubsw     m8, [pb_1]
    pmaddubsw    m11, [pb_1]
    psubw         m2, m8
    psubw         m7, m11                       ; p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4
    psrlw         m8, m2, 3
    psrlw        m11, m7, 3
    packuswb      m8, m11
    vpblendvb    m11, m5, m8, m9                ; q0
%ifidn %2, v
    mova [dstq+strideq*0], m11                  ; q0
%endif

    pmaddubsw     m0, [pb_m1_1]
    pmaddubsw     m1, [pb_m1_1]
    paddw         m2, m0
    paddw         m7, m1
    punpcklbw     m8, m13, m6
    punpckhbw    m13, m6
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw    m13, [pb_m1_1]
    paddw         m2, m8
    paddw         m7, m13                       ; p1 + p0 + q0 + 2 * q1 + q2 + 2 * q3 + 4
    psrlw         m8, m2, 3
    psrlw        m13, m7, 3
    packuswb      m8, m13
    vpblendvb    m13, m6, m8, m9                ; q1
%ifidn %2, v
    mova [dstq+strideq*1], m13                  ; q1
%endif

    punpcklbw     m0, m3, m6
    punpckhbw     m1, m3, m6
    pmaddubsw     m0, [pb_1]
    pmaddubsw     m1, [pb_1]
    psubw         m2, m0
    psubw         m7, m1
    punpcklbw     m0, m14, m15
    punpckhbw     m1, m14, m15
    pmaddubsw     m0, [pb_1]
    pmaddubsw     m1, [pb_1]
    paddw         m2, m0
    paddw         m7, m1                        ; p0 + q0 + q1 + q2 + 2 * q2 + 3 * q3 + 4
    psrlw         m2, 3
    psrlw         m7, 3
    packuswb      m2, m7
    vpblendvb     m2, m14, m2, m9               ; q2
%ifidn %2, v
    mova [dstq+strideq*2], m2                   ; q2
%else
    mova          m0, [rsp+0*32]
    mova          m1, [rsp+1*32]
%if %1 == 8
    ; 16x8 transpose
    punpcklbw     m3, m12, m10
    punpckhbw    m12, m10
    punpcklbw    m10, m0, m1
    punpckhbw     m0, m1
    punpcklbw     m1, m11, m13
    punpckhbw    m11, m13
    punpcklbw    m13, m2, m15
    punpckhbw     m2, m15

    punpcklwd    m15, m3, m10
    punpckhwd     m3, m10
    punpcklwd    m10, m12, m0
    punpckhwd    m12, m0
    punpcklwd     m0, m1, m13
    punpckhwd     m1, m13
    punpcklwd    m13, m11, m2
    punpckhwd    m11, m2

    punpckldq     m2, m15, m0
    punpckhdq    m15, m0
    punpckldq     m0, m3, m1
    punpckhdq     m3, m1
    punpckldq     m1, m10, m13
    punpckhdq    m10, m13
    punpckldq    m13, m12, m11
    punpckhdq    m12, m11

    ; write 8x32
    movq   [dstq+strideq*0-4], xm2
    movhps [dstq+strideq*1-4], xm2
    movq   [dstq+strideq*2-4], xm15
    movhps [dstq+stride3q -4], xm15
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm0
    movhps [dstq+strideq*1-4], xm0
    movq   [dstq+strideq*2-4], xm3
    movhps [dstq+stride3q -4], xm3
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm1
    movhps [dstq+strideq*1-4], xm1
    movq   [dstq+strideq*2-4], xm10
    movhps [dstq+stride3q -4], xm10
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm13
    movhps [dstq+strideq*1-4], xm13
    movq   [dstq+strideq*2-4], xm12
    movhps [dstq+stride3q -4], xm12
    lea         dstq, [dstq+strideq*4]

    vextracti128  xm2,  m2, 1
    vextracti128 xm15, m15, 1
    vextracti128  xm0,  m0, 1
    vextracti128  xm3,  m3, 1
    vextracti128  xm1,  m1, 1
    vextracti128 xm10, m10, 1
    vextracti128 xm13, m13, 1
    vextracti128 xm12, m12, 1

    movq   [dstq+strideq*0-4], xm2
    movhps [dstq+strideq*1-4], xm2
    movq   [dstq+strideq*2-4], xm15
    movhps [dstq+stride3q -4], xm15
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm0
    movhps [dstq+strideq*1-4], xm0
    movq   [dstq+strideq*2-4], xm3
    movhps [dstq+stride3q -4], xm3
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm1
    movhps [dstq+strideq*1-4], xm1
    movq   [dstq+strideq*2-4], xm10
    movhps [dstq+stride3q -4], xm10
    lea         dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0-4], xm13
    movhps [dstq+strideq*1-4], xm13
    movq   [dstq+strideq*2-4], xm12
    movhps [dstq+stride3q -4], xm12
    lea         dstq, [dstq+strideq*4]
%else
    ; 16x16 transpose and store
    SWAP           5, 10, 2
    SWAP           6, 0
    SWAP           7, 1
    SWAP           8, 11
    SWAP           9, 13
    mova          m0, [rsp+11*32]
    mova          m1, [rsp+12*32]
    mova          m2, [rsp+13*32]
    mova          m3, [rsp+14*32]
    mova          m4, [rsp+19*32]
    mova         m11, [rsp+20*32]
    mova         m12, [rsp+15*32]
    mova         m13, [rsp+16*32]
    mova         m14, [rsp+17*32]
    TRANSPOSE_16X16B 1, 0, [rsp+18*32]
    movu [dstq+strideq*0-8], xm0
    movu [dstq+strideq*1-8], xm1
    movu [dstq+strideq*2-8], xm2
    movu [dstq+stride3q -8], xm3
    lea         dstq, [dstq+strideq*4]
    movu [dstq+strideq*0-8], xm4
    movu [dstq+strideq*1-8], xm5
    movu [dstq+strideq*2-8], xm6
    movu [dstq+stride3q -8], xm7
    lea         dstq, [dstq+strideq*4]
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
    vextracti128 [dstq+strideq*0-8], m0, 1
    vextracti128 [dstq+strideq*1-8], m1, 1
    vextracti128 [dstq+strideq*2-8], m2, 1
    vextracti128 [dstq+stride3q -8], m3, 1
    lea         dstq, [dstq+strideq*4]
    vextracti128 [dstq+strideq*0-8], m4, 1
    vextracti128 [dstq+strideq*1-8], m5, 1
    vextracti128 [dstq+strideq*2-8], m6, 1
    vextracti128 [dstq+stride3q -8], m7, 1
    lea         dstq, [dstq+strideq*4]
    vextracti128 [dstq+strideq*0-8], m8, 1
    vextracti128 [dstq+strideq*1-8], m9, 1
    vextracti128 [dstq+strideq*2-8], m10, 1
    vextracti128 [dstq+stride3q -8], m11, 1
    lea         dstq, [dstq+strideq*4]
    vextracti128 [dstq+strideq*0-8], m12, 1
    vextracti128 [dstq+strideq*1-8], m13, 1
    vextracti128 [dstq+strideq*2-8], m14, 1
    vextracti128 [dstq+stride3q -8], m15, 1
    lea         dstq, [dstq+strideq*4]
%endif
%endif
%elif %1 == 6
    ; flat6 filter

    punpcklbw     m8, m13, m5
    punpckhbw    m11, m13, m5
    pmaddubsw     m0, m8, [pb_3_1]
    pmaddubsw     m1, m11, [pb_3_1]
    punpcklbw     m7, m4, m3
    punpckhbw    m10, m4, m3
    pmaddubsw     m2, m7, [pb_2]
    pmaddubsw    m12, m10, [pb_2]
    paddw         m0, m2
    paddw         m1, m12
    pmulhrsw      m2, m0, [pw_4096]
    pmulhrsw     m12, m1, [pw_4096]
    packuswb      m2, m12
    vpblendvb     m2, m3, m2, m9
%ifidn %2, v
    mova [tmpq+strideq*2], m2                   ; p1
%endif

    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw    m11, [pb_m1_1]
    paddw         m0, m8
    paddw         m1, m11
    punpcklbw     m8, m13, m6
    punpckhbw    m11, m13, m6
    pmaddubsw     m8, [pb_m1_1]
    pmaddubsw    m11, [pb_m1_1]
    paddw         m0, m8
    paddw         m1, m11
    pmulhrsw     m12, m0, [pw_4096]
    pmulhrsw     m13, m1, [pw_4096]
    packuswb     m12, m13
    vpblendvb    m12, m4, m12, m9
%ifidn %2, v
    mova [tmpq+stride3q], m12                   ; p0
%endif

    paddw         m0, m8
    paddw         m1, m11
    punpcklbw     m8, m3, m14
    punpckhbw    m11, m3, m14
    pmaddubsw    m14, m8, [pb_m1_1]
    pmaddubsw    m13, m11, [pb_m1_1]
    paddw         m0, m14
    paddw         m1, m13
    pmulhrsw     m14, m0, [pw_4096]
    pmulhrsw     m13, m1, [pw_4096]
    packuswb     m14, m13
    vpblendvb    m14, m5, m14, m9
%ifidn %2, v
    mova [dstq+strideq*0], m14                  ; q0
%endif

    pmaddubsw     m8, [pb_m1_2]
    pmaddubsw    m11, [pb_m1_2]
    paddw         m0, m8
    paddw         m1, m11
    pmaddubsw     m7, [pb_m1_0]
    pmaddubsw    m10, [pb_m1_0]
    paddw         m0, m7
    paddw         m1, m10
    pmulhrsw      m0, [pw_4096]
    pmulhrsw      m1, [pw_4096]
    packuswb      m0, m1
    vpblendvb     m0, m6, m0, m9
%ifidn %2, v
    mova [dstq+strideq*1], m0                   ; q1
%else
    TRANSPOSE_16x4_AND_WRITE_4x32 2, 12, 14, 0, 1
%endif
%else
%ifidn %2, v
    mova [tmpq+strideq*0], m3                   ; p1
    mova [tmpq+strideq*1], m4                   ; p0
    mova [tmpq+strideq*2], m5                   ; q0
    mova [tmpq+stride3q ], m6                   ; q1
%else
    TRANSPOSE_16x4_AND_WRITE_4x32 3, 4, 5, 6, 7
%endif
%endif
%endmacro

INIT_YMM avx2
cglobal lpf_v_sb_y_8bpc, 7, 10, 16, 32 * 11, \
                    dst, stride, mask, l, l_stride, lut, \
                    w, stride3, mstride, tmp
    shl    l_strideq, 2
    sub           lq, l_strideq
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]

.loop:
    cmp byte [maskq+8], 0                       ; vmask[2]
    je .no_flat16

    FILTER        16, v
    jmp .end

.no_flat16:
    cmp byte [maskq+4], 0                       ; vmask[1]
    je .no_flat

    FILTER         8, v
    jmp .end

.no_flat:
    cmp byte [maskq+0], 0                       ; vmask[0]
    je .end

    call .v4

.end:
    add           lq, 32
    add         dstq, 32
    add        maskq, 1
    sub           wd, 8
    jg .loop
    RET
ALIGN function_align
.v4:
    FILTER         4, v
    ret

INIT_YMM avx2
cglobal lpf_h_sb_y_8bpc, 7, 10, 16, 32 * 21, \
                    dst, stride, mask, l, l_stride, lut, \
                    h, stride3, l_stride3, tmp
    shl    l_strideq, 2
    sub           lq, 4
    lea     stride3q, [strideq*3]
    lea   l_stride3q, [l_strideq*3]

.loop:
    cmp byte [maskq+8], 0                       ; vmask[2]
    je .no_flat16

    FILTER        16, h
    jmp .end

.no_flat16:
    cmp byte [maskq+4], 0                       ; vmask[1]
    je .no_flat

    FILTER         8, h
    jmp .end

.no_flat:
    cmp byte [maskq+0], 0                       ; vmask[0]
    je .no_filter

    call .h4
    jmp .end

.no_filter:
    lea         dstq, [dstq+stride3q*8]
    lea           lq, [lq+l_strideq*8]
    lea         dstq, [dstq+strideq*8]
.end:
    add        maskq, 1
    sub           hd, 8
    jg .loop
    RET
ALIGN function_align
.h4:
    FILTER         4, h
    ret

INIT_YMM avx2
cglobal lpf_v_sb_uv_8bpc, 7, 10, 16, \
                     dst, stride, mask, l, l_stride, lut, \
                     w, stride3, mstride, tmp
    shl    l_strideq, 2
    sub           lq, l_strideq
    mov     mstrideq, strideq
    neg     mstrideq
    lea     stride3q, [strideq*3]

.loop:
    cmp byte [maskq+4], 0                       ; vmask[1]
    je .no_flat

    FILTER         6, v
    jmp .end

.no_flat:
    cmp byte [maskq+0], 0                       ; vmask[0]
    je .end

    call mangle(private_prefix %+ _lpf_v_sb_y_8bpc_avx2).v4

.end:
    add           lq, 32
    add         dstq, 32
    add        maskq, 1
    sub           wd, 8
    jg .loop
    RET

INIT_YMM avx2
cglobal lpf_h_sb_uv_8bpc, 7, 10, 16, \
                     dst, stride, mask, l, l_stride, lut, \
                     h, stride3, l_stride3, tmp
    shl    l_strideq, 2
    sub           lq, 4
    lea     stride3q, [strideq*3]
    lea   l_stride3q, [l_strideq*3]

.loop:
    cmp byte [maskq+4], 0                       ; vmask[1]
    je .no_flat

    FILTER         6, h
    jmp .end

.no_flat:
    cmp byte [maskq+0], 0                       ; vmask[0]
    je .no_filter

    call mangle(private_prefix %+ _lpf_h_sb_y_8bpc_avx2).h4
    jmp .end

.no_filter:
    lea         dstq, [dstq+stride3q*8]
    lea           lq, [lq+l_strideq*8]
    lea         dstq, [dstq+strideq*8]
.end:
    add        maskq, 1
    sub           hd, 8
    jg .loop
    RET

%endif ; ARCH_X86_64
