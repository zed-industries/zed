; Copyright © 2018, VideoLAN and dav1d authors
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

SECTION_RODATA 64

pb_4x0_4x4_4x8_4x12: times 4 db 0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12

pb_mask: dd 0x0001, 0x0002, 0x0004, 0x0008, 0x0010, 0x0020, 0x0040, 0x0080
         dd 0x0100, 0x0200, 0x0400, 0x0800, 0x1000, 0x2000, 0x4000, 0x8000

hmulA: dd  0,  8, 16, 24, 32, 40, 48, 56,  4, 12, 20, 28, 36, 44, 52, 60
hmulB: dd  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
hmulC: dd  0,  1,  2,  3, 16, 17, 18, 19, 32, 33, 34, 35, 48, 49, 50, 51
hmulD: dd  0,  1, 16, 17, 32, 33, 48, 49
hshuf4:db  0,  4,  8, 12,  1,  5,  9, 13,  2,  6, 10, 14,  3,  7, 11, 15

pb_1:    times 4 db 1
pb_2:    times 4 db 2
pb_3:    times 4 db 3
pb_4:    times 4 db 4
pb_16:   times 4 db 16
pb_63:   times 4 db 63
pb_64:   times 4 db 64
pb_128:  times 4 db 0x80
pb_240:  times 4 db 0xf0
pb_248:  times 4 db 0xf8
pb_254:  times 4 db 0xfe
pb_2_1:  times 2 db 2, 1
pb_3_1:  times 2 db 3, 1
pb_7_1:  times 2 db 7, 1
pb_m1_0: times 2 db -1, 0
pb_m1_1: times 2 db -1, 1
pb_m1_2: times 2 db -1, 2
pw_2048: times 2 dw 2048
pw_4096: times 2 dw 4096

SECTION .text

%macro ABSSUB 4 ; dst, a, b, tmp
    psubusb           %1, %2, %3
    psubusb           %4, %3, %2
    por               %1, %4
%endmacro

%macro TRANSPOSE_16x4_AND_WRITE_4x32 5
    punpcklbw        m%5, m%1, m%2
    punpckhbw        m%1, m%2
    punpcklbw        m%2, m%3, m%4
    punpckhbw        m%3, m%4
    punpcklwd        m%4, m%5, m%2
    punpckhwd        m%5, m%2
    punpcklwd        m%2, m%1, m%3
    punpckhwd        m%1, m%3
    kmovw             k1, k6
    lea               t0, [dstq+strideq*4]
    vpscatterdd [dstq+m19-2]{k1}, m%4
    kmovw             k1, k6
    lea               t1, [dstq+strideq*8]
    vpscatterdd [t0  +m19-2]{k1}, m%5
    kmovw             k1, k6
    lea               t2, [t0  +strideq*8]
    vpscatterdd [t1  +m19-2]{k1}, m%2
    kmovw             k1, k6
    vpscatterdd [t2  +m19-2]{k1}, m%1
%endmacro

%macro TRANSPOSE_16X16B 3 ; in_load_15_from_mem, out_store_0_in_mem, mem
%if %1 == 0
    SWAP             m16, m22
%endif
    punpcklbw        m22, m24, m26
    punpckhbw        m24, m26
    punpcklbw        m26, m2, m3
    punpckhbw         m2, m3
    punpcklbw         m3, m4, m5
    punpckhbw         m4, m5
    punpcklbw         m5, m6, m7
    punpckhbw         m6, m7
    punpcklbw         m7, m8, m9
    punpckhbw         m8, m9
    punpcklbw         m9, m10, m11
    punpckhbw        m10, m11
    punpcklbw        m11, m25, m13
    punpckhbw        m25, m13
%if %1 == 0
    SWAP             m13, m16
%else
    mova             m13, %3
%endif
    SWAP             m16, m25
    punpcklbw        m25, m14, m13
    punpckhbw        m13, m14, m13
    ; interleaved in m22,24,26,2,3,4,5,6,7,8,9,10,11,rsp%3,25,13
    punpcklwd        m14, m22, m26
    punpckhwd        m22, m26
    punpcklwd        m26, m24, m2
    punpckhwd        m24, m2
    punpcklwd         m2, m3, m5
    punpckhwd         m3, m5
    punpcklwd         m5, m4, m6
    punpckhwd         m4, m6
    punpcklwd         m6, m7, m9
    punpckhwd         m7, m9
    punpcklwd         m9, m8, m10
    punpckhwd         m8, m10
    punpcklwd        m10, m11, m25
    punpckhwd        m11, m25
    SWAP             m25, m16, m11
    punpcklwd        m11, m25, m13
    punpckhwd        m25, m13
    ; interleaved in m14,15,26,24,2,3,5,4,6,7,9,8,10,rsp%3,11,25
    punpckldq        m13, m14, m2
    punpckhdq        m14, m2
    punpckldq         m2, m22, m3
    punpckhdq        m22, m3
    punpckldq         m3, m26, m5
    punpckhdq        m26, m5
    punpckldq         m5, m24, m4
    punpckhdq        m24, m4
    punpckldq         m4, m6, m10
    punpckhdq         m6, m10
    punpckldq        m10, m9, m11
    punpckhdq         m9, m11
    punpckldq        m11, m8, m25
    punpckhdq         m8, m25
    SWAP             m25, m16, m8
    punpckldq         m8, m7, m25
    punpckhdq         m7, m25
    ; interleaved in m13,14,2,15,3,26,5,24,4,6,8,7,10,9,11,rsp%3
    punpcklqdq       m25, m13, m4
    punpckhqdq       m13, m4
    punpcklqdq        m4, m14, m6
    punpckhqdq       m14, m6
    punpcklqdq        m6, m2, m8
    punpckhqdq        m2, m8
    punpcklqdq        m8, m22, m7
    punpckhqdq       m22, m7
    punpcklqdq        m7, m3, m10
    punpckhqdq        m3, m10
    punpcklqdq       m10, m26, m9
    punpckhqdq       m26, m9
    punpcklqdq        m9, m5, m11
    punpckhqdq        m5, m11
    SWAP             m11, m16
%if %2 == 0
    SWAP             m16, m25
%else
    mova              %3, m25
%endif
    punpcklqdq       m25, m24, m11
    punpckhqdq       m24, m11
%if %2 == 0
    SWAP             m11, m16
%endif
    ; interleaved m11,13,4,14,6,2,8,15,7,3,10,26,9,5,25,24
    SWAP              24, 11, 26, 13, 5, 2, 4, 6, 8, 7, 22
    SWAP               3, 14, 25, 9
%endmacro

%macro FILTER 2 ; width [4/6/8/16], dir [h/v]
    ; load data
%ifidn %2, v
%define is_h 0
%if %1 == 4
    lea               t0, [dstq+mstrideq*2]
    mova              m3, [t0  +strideq*0]    ; p1
    mova              m4, [t0  +strideq*1]    ; p0
    mova              m5, [t0  +strideq*2]    ; q0
    mova              m6, [t0  +stride3q ]    ; q1
%else
    ; load 6-8 pixels, remainder (for wd=16) will be read inline
%if %1 == 16
    lea               t0, [dstq+mstrideq*8]
    mova             m16, [t0  +strideq*1]
    mova             m17, [t0  +strideq*2]
    mova             m18, [t0  +stride3q ]
%endif
    lea               t0, [dstq+mstrideq*4]
%if %1 != 6
    mova             m25, [t0  +strideq*0]
%endif
    mova             m13, [t0  +strideq*1]
    mova              m3, [t0  +strideq*2]
    mova              m4, [t0  +stride3q ]
    mova              m5, [dstq+strideq*0]
    mova              m6, [dstq+strideq*1]
    mova             m14, [dstq+strideq*2]
%if %1 != 6
    mova             m22, [dstq+stride3q ]
%endif
%if %1 == 16
    lea               t0, [dstq+strideq*4]
    mova             m29, [t0  +strideq*0]
    mova             m30, [t0  +strideq*1]
    mova             m31, [t0  +strideq*2]
%endif
%endif
%else ; h
%define is_h 1
    ; load lines
%if %1 == 4
    vbroadcasti32x4   m0, [hshuf4]
    kmovw             k1, k6
    lea               t0, [dstq+strideq*4]
    vpgatherdd    m3{k1}, [dstq+m19-2]
    kmovw             k1, k6
    lea               t1, [dstq+strideq*8]
    vpgatherdd    m4{k1}, [t0  +m19-2]
    kmovw             k1, k6
    lea               t2, [t0  +strideq*8]
    vpgatherdd    m5{k1}, [t1  +m19-2]
    kmovw             k1, k6
    vpgatherdd    m6{k1}, [t2  +m19-2]
    pshufb            m3, m0
    pshufb            m4, m0
    pshufb            m5, m0
    pshufb            m6, m0
    punpckldq         m7, m3, m4
    punpckhdq         m3, m4
    punpckldq         m4, m5, m6
    punpckhdq         m5, m6
    punpcklqdq        m6, m7, m4
    punpckhqdq        m7, m4
    punpcklqdq        m4, m3, m5
    punpckhqdq        m3, m5
    SWAP               3, 6
    SWAP               5, 4, 7
    ; 6,7,4,3 -> 3,4,5,6
%elif %1 == 6 || %1 == 8
    kmovb             k1, k7
    lea               t0, [dstq+strideq*1]
    vpgatherdq    m3{k1}, [dstq+ym21-%1/2]
    kmovb             k1, k7
    lea               t1, [dstq+strideq*2]
    vpgatherdq    m4{k1}, [t0  +ym21-%1/2]
    kmovb             k1, k7
    lea               t2, [dstq+stride3q ]
    vpgatherdq    m5{k1}, [t1  +ym21-%1/2]
    kmovb             k1, k7
    vextracti32x8    ym0, m21, 1
    vpgatherdq    m6{k1}, [t2  +ym21-%1/2]
    kmovb             k1, k7
    vpgatherdq   m12{k1}, [dstq+ym0 -%1/2]
    kmovb             k1, k7
    vpgatherdq   m13{k1}, [t0  +ym0 -%1/2]
    kmovb             k1, k7
    vpgatherdq   m14{k1}, [t1  +ym0 -%1/2]
    kmovb             k1, k7
    vpgatherdq   m15{k1}, [t2  +ym0 -%1/2]
    ; transpose 8x16
    ; xm3: A-H0,A-H8
    ; xm4: A-H1,A-H9
    ; xm5: A-H2,A-H10
    ; xm6: A-H3,A-H11
    ; xm12: A-H4,A-H12
    ; xm13: A-H5,A-H13
    ; xm14: A-H6,A-H14
    ; xm15: A-H7,A-H15
    punpcklbw         m7, m3, m4
    punpckhbw         m3, m4
    punpcklbw         m4, m5, m6
    punpckhbw         m5, m6
    punpcklbw         m6, m12, m13
    punpckhbw        m12, m13
    punpcklbw        m13, m14, m15
    punpckhbw        m14, m15
    ; xm7: A0-1,B0-1,C0-1,D0-1,E0-1,F0-1,G0-1,H0-1
    ; xm3: A8-9,B8-9,C8-9,D8-9,E8-9,F8-9,G8-9,H8-9
    ; xm4: A2-3,B2-3,C2-3,D2-3,E2-3,F2-3,G2-3,H2-3
    ; xm5: A10-11,B10-11,C10-11,D10-11,E10-11,F10-11,G10-11,H10-11
    ; xm6: A4-5,B4-5,C4-5,D4-5,E4-5,F4-5,G4-5,H4-5
    ; xm12: A12-13,B12-13,C12-13,D12-13,E12-13,F12-13,G12-13,H12-13
    ; xm13: A6-7,B6-7,C6-7,D6-7,E6-7,F6-7,G6-7,H6-7
    ; xm14: A14-15,B14-15,C14-15,D14-15,E14-15,F14-15,G14-15,H14-15
    punpcklwd        m15, m7, m4
    punpckhwd         m7, m4
    punpcklwd         m4, m3, m5
    punpckhwd         m3, m5
    punpcklwd         m5, m6, m13
    punpckhwd         m6, m13
    punpcklwd        m13, m12, m14
    punpckhwd        m12, m14
    ; xm15: A0-3,B0-3,C0-3,D0-3
    ; xm7: E0-3,F0-3,G0-3,H0-3
    ; xm4: A8-11,B8-11,C8-11,D8-11
    ; xm3: E8-11,F8-11,G8-11,H8-11
    ; xm5: A4-7,B4-7,C4-7,D4-7
    ; xm6: E4-7,F4-7,G4-7,H4-7
    ; xm13: A12-15,B12-15,C12-15,D12-15
    ; xm12: E12-15,F12-15,G12-15,H12-15
    punpckldq        m14, m15, m5
    punpckhdq        m15, m5
    punpckldq         m5, m7, m6
 %if %1 != 6
    punpckhdq         m7, m6
 %endif
    punpckldq         m6, m4, m13
    punpckhdq         m4, m13
    punpckldq        m13, m3, m12
 %if %1 != 6
    punpckhdq        m12, m3, m12
 %endif
    ; xm14: A0-7,B0-7
    ; xm15: C0-7,D0-7
    ; xm5: E0-7,F0-7
    ; xm7: G0-7,H0-7
    ; xm6: A8-15,B8-15
    ; xm4: C8-15,D8-15
    ; xm13: E8-15,F8-15
    ; xm12: G8-15,H8-15
    punpcklqdq        m3, m14, m6
    punpckhqdq       m14, m6
    punpckhqdq        m6, m15, m4
    punpcklqdq       m15, m4
    punpcklqdq        m4, m5, m13
    punpckhqdq       m13, m5, m13
 %if %1 == 8
    punpcklqdq        m5, m7, m12
    punpckhqdq       m25, m7, m12
    ; xm3: A0-15
    ; xm14: B0-15
    ; xm15: C0-15
    ; xm6: D0-15
    ; xm4: E0-15
    ; xm13: F0-15
    ; xm5: G0-15
    ; xm25: H0-15
    SWAP              25, 3, 15
    SWAP              13, 14, 5, 4, 6
    SWAP              15, 22
    ; 3,14,15,6,4,13,5,12 -> 12,13,3,4,5,6,14,22
 %else
    SWAP              13, 3, 14
    SWAP               6, 4, 15, 5
    ; 3,14,15,6,4,13 -> 13,3,4,5,6,14
 %endif
%else ; 16, h
    ; load and 16x16 transpose. We only use 14 pixels but we'll need the
    ; remainder at the end for the second transpose
    movu            xm24, [dstq+strideq*0-8]
    movu            xm26, [dstq+strideq*1-8]
    movu             xm2, [dstq+strideq*2-8]
    movu             xm3, [dstq+stride3q -8]
    lea               t0, [dstq+strideq*4]
    movu             xm4, [t0  +strideq*0-8]
    movu             xm5, [t0  +strideq*1-8]
    movu             xm6, [t0  +strideq*2-8]
    movu             xm7, [t0  +stride3q -8]
    lea               t0, [t0  +strideq*4]
    movu             xm8, [t0  +strideq*0-8]
    movu             xm9, [t0  +strideq*1-8]
    movu            xm10, [t0  +strideq*2-8]
    movu            xm11, [t0  +stride3q -8]
    lea               t0, [t0  +strideq*4]
    movu            xm25, [t0  +strideq*0-8]
    movu            xm13, [t0  +strideq*1-8]
    movu            xm14, [t0  +strideq*2-8]
    movu            xm22, [t0  +stride3q -8]
    lea               t0, [t0  +strideq*4]
    vinserti32x4    ym24, [t0  +strideq*0-8], 1
    vinserti32x4    ym26, [t0  +strideq*1-8], 1
    vinserti32x4     ym2, [t0  +strideq*2-8], 1
    vinserti32x4     ym3, [t0  +stride3q -8], 1
    lea               t0, [t0  +strideq*4]
    vinserti32x4     ym4, [t0  +strideq*0-8], 1
    vinserti32x4     ym5, [t0  +strideq*1-8], 1
    vinserti32x4     ym6, [t0  +strideq*2-8], 1
    vinserti32x4     ym7, [t0  +stride3q -8], 1
    lea               t0, [t0  +strideq*4]
    vinserti32x4     ym8, [t0  +strideq*0-8], 1
    vinserti32x4     ym9, [t0  +strideq*1-8], 1
    vinserti32x4    ym10, [t0  +strideq*2-8], 1
    vinserti32x4    ym11, [t0  +stride3q -8], 1
    lea               t0, [t0  +strideq*4]
    vinserti32x4    ym25, [t0  +strideq*0-8], 1
    vinserti32x4    ym13, [t0  +strideq*1-8], 1
    vinserti32x4    ym14, [t0  +strideq*2-8], 1
    vinserti32x4    ym22, [t0  +stride3q -8], 1
    lea               t0, [t0  +strideq*4]
    vinserti32x4     m24, [t0  +strideq*0-8], 2
    vinserti32x4     m26, [t0  +strideq*1-8], 2
    vinserti32x4      m2, [t0  +strideq*2-8], 2
    vinserti32x4      m3, [t0  +stride3q -8], 2
    lea               t0, [t0  +strideq*4]
    vinserti32x4      m4, [t0  +strideq*0-8], 2
    vinserti32x4      m5, [t0  +strideq*1-8], 2
    vinserti32x4      m6, [t0  +strideq*2-8], 2
    vinserti32x4      m7, [t0  +stride3q -8], 2
    lea               t0, [t0  +strideq*4]
    vinserti32x4      m8, [t0  +strideq*0-8], 2
    vinserti32x4      m9, [t0  +strideq*1-8], 2
    vinserti32x4     m10, [t0  +strideq*2-8], 2
    vinserti32x4     m11, [t0  +stride3q -8], 2
    lea               t0, [t0  +strideq*4]
    vinserti32x4     m25, [t0  +strideq*0-8], 2
    vinserti32x4     m13, [t0  +strideq*1-8], 2
    vinserti32x4     m14, [t0  +strideq*2-8], 2
    vinserti32x4     m22, [t0  +stride3q -8], 2
    lea               t0, [t0  +strideq*4]
    vinserti32x4     m24, [t0  +strideq*0-8], 3
    vinserti32x4     m26, [t0  +strideq*1-8], 3
    vinserti32x4      m2, [t0  +strideq*2-8], 3
    vinserti32x4      m3, [t0  +stride3q -8], 3
    lea               t0, [t0  +strideq*4]
    vinserti32x4      m4, [t0  +strideq*0-8], 3
    vinserti32x4      m5, [t0  +strideq*1-8], 3
    vinserti32x4      m6, [t0  +strideq*2-8], 3
    vinserti32x4      m7, [t0  +stride3q -8], 3
    lea               t0, [t0  +strideq*4]
    vinserti32x4      m8, [t0  +strideq*0-8], 3
    vinserti32x4      m9, [t0  +strideq*1-8], 3
    vinserti32x4     m10, [t0  +strideq*2-8], 3
    vinserti32x4     m11, [t0  +stride3q -8], 3
    lea               t0, [t0  +strideq*4]
    vinserti32x4     m25, [t0  +strideq*0-8], 3
    vinserti32x4     m13, [t0  +strideq*1-8], 3
    vinserti32x4     m14, [t0  +strideq*2-8], 3
    vinserti32x4     m22, [t0  +stride3q -8], 3
    ;
    TRANSPOSE_16X16B 0, 1, [rsp+0*64]
    SWAP             m16, m26
    SWAP             m17, m2
    SWAP             m18, m3
    SWAP             m29, m25
    SWAP             m30, m13
    SWAP             m31, m14
    mova      [rsp+4*64], m22
    ; 4,5,6,7,8,9,10,11 -> 25,13,3,4,5,6,14,22
    SWAP              25, 4, 7
    SWAP              13, 5, 8
    SWAP               3, 6, 9
    SWAP              10, 14
    SWAP              11, 22
%endif
%endif

    ; load L/E/I/H
    vpbroadcastd     m15, [pb_1]
%ifidn %2, v
    movu              m1, [lq]
    movu              m0, [lq+l_strideq]
%else
    kmovw             k1, k6
    vpgatherdd    m0{k1}, [lq+m20+4]
    kmovw             k1, k6
    vpgatherdd    m1{k1}, [lq+m20+0]
%endif
    pxor              m2, m2
    pcmpeqb           k1, m0, m2
    vmovdqu8      m0{k1}, m1                ; l[x][] ? l[x][] : l[x-stride][]
    pshufb            m0, pbshuf            ; l[x][0]
    vpcmpub           k3, m0, m2, 4 ; neq   ; L
    psrlq             m2, m0, [lutq+128]
    pand              m2, [pb_63]{bcstd}
    vpbroadcastb      m1, [lutq+136]
    pminub            m2, m1
    pmaxub            m2, m15               ; I
    pand              m1, m0, [pb_240]{bcstd}
    psrlq             m1, 4                 ; H
    paddd             m0, [pb_2]{bcstd}
    paddb             m0, m0
    paddb             m0, m2                ; E

    ABSSUB            m8, m3, m4, m9        ; abs(p1-p0)
    ABSSUB            m9, m5, m6, m10       ; abs(q1-q0)
    pmaxub            m8, m9
    vpcmpub           k1, m8, m1, 6 ; gt    ; hev
%if %1 != 4
 %if %1 == 6
    ABSSUB            m9, m13, m4, m10      ; abs(p2-p0)
    pmaxub            m9, m8
 %else
    ABSSUB            m9, m25, m4, m10      ; abs(p3-p0)
    pmaxub            m9, m8
    ABSSUB           m10, m13, m4, m11      ; abs(p2-p0)
    pmaxub            m9, m10
 %endif
    ABSSUB           m10, m5,  m14, m11     ; abs(q2-q0)
    pmaxub            m9, m10
 %if %1 != 6
    ABSSUB           m10, m5,  m22, m11     ; abs(q3-q0)
    pmaxub            m9, m10
 %endif
    vpcmpub       k2{k3}, m9, m15, 2 ; le   ; flat8in
 %if %1 == 6
    ABSSUB           m10, m13, m3,  m1      ; abs(p2-p1)
 %else
    ABSSUB           m10, m25, m13, m11     ; abs(p3-p2)
    ABSSUB           m11, m13, m3,  m1      ; abs(p2-p1)
    pmaxub           m10, m11
    ABSSUB           m11, m14, m22, m1      ; abs(q3-q2)
    pmaxub           m10, m11
 %endif
    ABSSUB           m11, m14, m6,  m1      ; abs(q2-q1)
    pmaxub           m10, m11
 %if %1 == 16
    vpbroadcastd     m11, [maskq+8]
    por              m11, [maskq+4]{bcstd}
 %else
    vpbroadcastd     m11, [maskq+4]
 %endif
    vptestmd          k4, m11, pbmask
    vmovdqa32 m10{k4}{z}, m10               ; only apply fm-wide to wd>4 blocks
    pmaxub            m8, m10
%endif
    vpcmpub       k3{k3}, m8, m2, 2 ; le
    ABSSUB           m10, m3, m6, m11       ; abs(p1-q1)
    ABSSUB           m11, m4, m5, m2        ; abs(p0-q0)
    paddusb          m11, m11
    pand             m10, [pb_254]{bcstd}
    psrlq            m10, 1
    paddusb          m10, m11               ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    vpcmpub       k3{k3}, m10, m0, 2        ; abs(p0-q0)*2+(abs(p1-q1)>>1) <= E

%if %1 == 16
    ABSSUB            m1, m16, m4, m2
    ABSSUB            m2, m17, m4, m10
    pmaxub            m1, m2
    ABSSUB            m2, m18, m4, m10
    pmaxub            m1, m2
    ABSSUB            m2, m29, m5, m10
    pmaxub            m1, m2
    ABSSUB            m2, m30, m5, m10
    pmaxub            m1, m2
    ABSSUB            m2, m31, m5, m10
    pmaxub            m1, m2
    kandq             k2, k2, k3
    vpcmpub       k4{k2}, m1, m15, 2        ; flat8in & flat8out
    vpbroadcastd      m2, [maskq+8]
    vptestmd          k5, m2, pbmask
    vpmovm2d          m7, k5
    vptestmb      k4{k4}, m7, m7            ; flat16 & fm
    por              m10, m2, [maskq+4]{bcstd}
    vptestmd          k5, m10, pbmask
    vpmovm2d          m7, k5
    vptestmb      k2{k2}, m7, m7            ; flat8in
    por               m2, m10, [maskq+0]{bcstd}
    vptestmd          k5, m2, pbmask
    vpmovm2d          m7, k5
    vptestmb      k3{k3}, m7, m7
    kandnq            k3, k2, k3            ; fm & !flat8 & !flat16
    kandnq            k2, k4, k2            ; flat8 & !flat16
%elif %1 != 4
    vpbroadcastd      m0, [maskq+4]
    vptestmd          k4, m0, pbmask
    vpmovm2d          m7, k4
    vptestmb      k2{k2}, m7, m7
    kandq             k2, k2, k3            ; flat8 & fm
    por               m0, [maskq+0]{bcstd}
    vptestmd          k4, m0, pbmask
    vpmovm2d          m7, k4
    vptestmb      k3{k3}, m7, m7
    kandnq            k3, k2, k3            ; fm & !flat8
%else
 %ifidn %2, v
    vptestmd          k4, pbmask, [maskq+0]{bcstd}
 %else
    vpbroadcastd      m0, [maskq+0]
    vptestmd          k4, m0, pbmask
 %endif
    vpmovm2d          m7, k4
    vptestmb      k3{k3}, m7, m7            ; fm
%endif

    ; short filter
%if %1 >= 8
    SWAP             m23, m15
%endif
    vpbroadcastd     m15, [pb_3]
    vpbroadcastd      m0, [pb_4]
    vpbroadcastd     m12, [pb_16]
    vpbroadcastd      m1, [pb_64]
    pxor              m3, pb128
    pxor              m6, pb128
    psubsb    m10{k1}{z}, m3, m6            ; f=iclip_diff(p1-q1)&hev
    pxor              m4, pb128
    pxor              m5, pb128
    psubsb           m11, m5, m4
    paddsb           m10, m11
    paddsb           m10, m11
    paddsb    m10{k3}{z}, m10, m11          ; f=iclip_diff(3*(q0-p0)+f)&fm
    paddsb            m8, m10, m15
    paddsb           m10, m0
    pand              m8, [pb_248]{bcstd}
    pand             m10, [pb_248]{bcstd}
    psrlq             m8, 3
    psrlq            m10, 3
    pxor              m8, m12
    pxor             m10, m12
    psubb             m8, m12               ; f2
    psubb            m10, m12               ; f1
    paddsb            m4, m8
    psubsb            m5, m10
    pxor              m4, pb128
    pxor              m5, pb128
    ;
    pxor             m10, pb128
    pxor              m8, m8
    pavgb             m8, m10               ; f=(f1+1)>>1
    psubb             m8, m1
    knotq             k1, k1
    paddsb        m3{k1}, m3, m8
    psubsb        m6{k1}, m6, m8
    pxor              m3, pb128
    pxor              m6, pb128

%if %1 == 16
    ; flat16 filter
%ifidn %2, v
    lea               t0, [dstq+mstrideq*8]
%endif
    SWAP             m24, m16, m14
    SWAP              m2, m17, m22
    SWAP              m7, m18

    ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0 [p5/p4/p2/p1/p0/q0][p6/p3] A
    ; write -6
    vpbroadcastd      m1, [pb_7_1]
    vpbroadcastd     m12, [pb_2]
    punpcklbw        m14, m24, m25
    punpckhbw        m22, m24, m25
    pmaddubsw        m10, m14, m1
    pmaddubsw        m11, m22, m1          ; p6*7+p3
    punpcklbw         m8, m2, m7
    punpckhbw         m9, m2, m7
    pmaddubsw         m8, m12
    pmaddubsw         m9, m12
    paddw            m10, m8
    paddw            m11, m9                ; p6*7+p5*2+p4*2+p3
%ifidn %2, h
    vpbroadcastd     m27, [pw_2048]
    vpbroadcastd      m1, [pb_m1_1]
 %define pw2048 m27
 %define pbm1_1 m1
%endif
    punpcklbw         m8, m13, m3
    punpckhbw         m9, m13, m3
    pmaddubsw         m8, m23
    pmaddubsw         m9, m23
    paddw            m10, m8
    paddw            m11, m9                ; p6*7+p5*2+p4*2+p3+p2+p1
    punpcklbw         m8, m4, m5
    punpckhbw         m9, m4, m5
    pmaddubsw         m8, m23
    pmaddubsw         m9, m23
    paddw            m10, m8
    paddw            m11, m9                ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
    pmulhrsw          m8, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m8, m9
%ifidn %2, v
    vmovdqu8 [t0+strideq*2]{k4}, m8         ; p5
%else
    vpblendmb     m8{k4}, m2, m8
    mova      [rsp+1*64], m8
%endif

    ; sub p6*2, add p3/q1 [reuse p6/p3 from A][-p6,+q1|save] B
    ; write -5
    pmaddubsw        m14, pbm1_1
    pmaddubsw        m22, pbm1_1
    paddw            m10, m14
    paddw            m11, m22               ; p6*6+p5*2+p4*2+p3*2+p2+p1+p0+q0
    punpcklbw         m8, m24, m6
    punpckhbw         m9, m24, m6
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p6*5+p5*2+p4*2+p3*2+p2+p1+p0+q0+q1
    SWAP             m18, m8
    SWAP             m23, m9
    pmulhrsw          m8, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m8, m9
%ifidn %2, v
    vmovdqu8 [t0+stride3q]{k4}, m8          ; p4
%else
    vpblendmb     m8{k4}, m7, m8
    mova      [rsp+2*64], m8
%endif

    ; sub p6/p5, add p2/q2 [-p6,+p2][-p5,+q2|save] C
    ; write -4
    SWAP             m14, m16
    punpcklbw         m8, m24, m13
    punpckhbw         m9, m24, m13
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p6*4+p5*2+p4*2+p3*2+p2*2+p1+p0+q0+q1
    punpcklbw         m8, m2, m14
    punpckhbw         m2, m14
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m2, pbm1_1
    paddw            m10, m8
    paddw            m11, m2                ; p6*4+p5+p4*2+p3*2+p2*2+p1+p0+q0+q1+q2
    SWAP             m16, m8
    pmulhrsw          m8, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m8, m9
%ifidn %2, v
    vmovdqu8 [t0+strideq*4]{k4}, m8         ; p3
%else
    vpblendmb     m8{k4}, m25, m8
    mova      [rsp+3*64], m8
%endif

    ; sub p6/p4, add p1/q3 [-p6,+p1][-p4,+q3|save] D
    ; write -3
    SWAP             m22, m17
    punpcklbw         m8, m24, m3
    punpckhbw         m9, m24, m3
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p6*3+p5+p4*2+p3*2+p2*2+p1*2+p0+q0+q1+q2
    punpcklbw         m8, m7, m22
    punpckhbw         m7, m22
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m7, pbm1_1
    paddw            m10, m8
    paddw            m11, m7                ; p6*3+p5+p4+p3*2+p2*2+p1*2+p0+q0+q1+q2+q3
    SWAP             m17, m8
    pmulhrsw          m8, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m8, m9
    vpblendmb    m15{k4}, m13, m8           ; don't clobber p2/m13 since we need it in F

    ; sub p6/p3, add p0/q4 [-p6,+p0][-p3,+q4|save] E
    ; write -2
%ifidn %2, v
    lea               t0, [dstq+strideq*4]
%endif
    punpcklbw         m8, m24, m4
    punpckhbw         m9, m24, m4
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p6*2+p5+p4+p3*2+p2*2+p1*2+p0*2+q0+q1+q2+q3
    punpcklbw         m8, m25, m29
    punpckhbw         m9, m25, m29
    SWAP             m26, m29
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p6*2+p5+p4+p3+p2*2+p1*2+p0*2+q0+q1+q2+q3+q4
    SWAP             m29, m8
    SWAP              m0, m9
    pmulhrsw          m8, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m8, m9
    vpblendmb    m12{k4}, m3, m8            ; don't clobber p1/m3 since we need it in G

    ; sub p6/p2, add q0/q5 [-p6,+q0][-p2,+q5|save] F
    ; write -1
%ifidn %2, h
    SWAP             m28, m24
    punpcklbw         m8, m28, m5
    punpckhbw        m24, m28, m5
%else
    punpcklbw         m8, m24, m5
    punpckhbw        m24, m5
%endif
    pmaddubsw         m8, pbm1_1
    pmaddubsw        m24, pbm1_1
    paddw            m10, m8
    paddw            m11, m24               ; p6+p5+p4+p3+p2*2+p1*2+p0*2+q0*2+q1+q2+q3+q4
    punpcklbw        m24, m13, m30
    punpckhbw         m9, m13, m30
%ifidn %2, h
    SWAP             m27, m30
%endif
    SWAP             m13, m15
    pmaddubsw        m24, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m24
    paddw            m11, m9                ; p6+p5+p4+p3+p2+p1*2+p0*2+q0*2+q1+q2+q3+q4+q5
    SWAP             m30, m24
    SWAP             m15, m9
%ifidn %2, h
    SWAP              m9, m24
 %define pw2048 m9
%endif
    pmulhrsw         m24, m10, pw2048
    pmulhrsw          m8, m11, pw2048
    paddw            m10, m18               ; p5+p4+p3+p2+p1*2+p0*2+q0*2+q1*2+q2+q3+q4+q5
    paddw            m11, m23
    packuswb         m24, m8
    punpcklbw         m8, m3, m31
    pmaddubsw         m8, pbm1_1
    paddw            m10, m8                ; p5+p4+p3+p2+p1+p0*2+q0*2+q1*2+q2+q3+q4+q5+q6
    SWAP             m18, m8
    pmulhrsw          m8, m10, pw2048
    paddw            m10, m16               ; p4+p3+p2+p1+p0*2+q0*2+q1*2+q2*2+q3+q4+q5+q6
%ifidn %2, h
    SWAP             m16, m9
 %define pw2048 m16
%endif
    punpckhbw         m9, m3, m31
    SWAP              m3, m12
    pmaddubsw         m9, pbm1_1
    paddw            m11, m9                ; p5+p4+p3+p2+p1+p0*2+q0*2+q1*2+q2+q3+q4+q5+q6
    SWAP             m23, m9
    pmulhrsw          m9, m11, pw2048
    paddw            m11, m2                ; p4+p3+p2+p1+p0*2+q0*2+q1*2+q2*2+q3+q4+q5+q6
%ifidn %2, h
    SWAP              m2, m1
 %define pbm1_1 m2
%endif
    vpblendmb     m1{k4}, m4, m24           ; don't clobber p0/m4 since we need it in H

    ; sub p6/p1, add q1/q6 [reuse -p6,+q1 from B][-p1,+q6|save] G
    ; write +0
    SWAP             m24, m31               ; q6
    packuswb          m8, m9
%ifidn %2, h
    SWAP             m31, m2
 %define pbm1_1 m31
%endif
    vpblendmb    m12{k4}, m5, m8            ; don't clobber q0/m5 since we need it in I

    ; sub p5/p0, add q2/q6 [reuse -p5,+q2 from C][-p0,+q6] H
    ; write +1
    punpcklbw         m8, m4, m24
    punpckhbw         m2, m4, m24
    SWAP              m4, m1
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m2, pbm1_1
    paddw            m10, m8
    paddw            m11, m2                ; p4+p3+p2+p1+p0+q0*2+q1*2+q2*2+q3+q4+q5+q6*2
    pmulhrsw          m2, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m2, m9
    vpblendmb     m2{k4}, m6, m2            ; don't clobber q1/m6 since we need it in K

    ; sub p4/q0, add q3/q6 [reuse -p4,+q3 from D][-q0,+q6] I
    ; write +2
    paddw            m10, m17               ; p3+p2+p1+p0+q0*2+q1*2+q2*2+q3*2+q4+q5+q6*2
    paddw            m11, m7
    punpcklbw         m8, m5, m24
    punpckhbw         m9, m5, m24
    SWAP              m5, m12
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p3+p2+p1+p0+q0+q1*2+q2*2+q3*2+q4+q5+q6*3
    pmulhrsw          m7, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m7, m9
    vpblendmb     m7{k4}, m14, m7           ; don't clobber q2/m14 since we need it in K

    ; sub p3/q1, add q4/q6 [reuse -p3,+q4 from E][-q1,+q6] J
    ; write +3
    paddw            m10, m29               ; p2+p1+p0+q0+q1*2+q2*2+q3*2+q4*2+q5+q6*3
    paddw            m11, m0
    punpcklbw         m8, m6, m24
    punpckhbw         m9, m6, m24
    SWAP               2, 6
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p2+p1+p0+q0+q1+q2*2+q3*2+q4*2+q5+q6*4
    pmulhrsw          m8, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m8, m9
%ifidn %2, v
    vmovdqu8 [t0+mstrideq]{k4}, m8
%else
    SWAP             m29, m16
 %define pw2048 m29
    vpblendmb    m16{k4}, m22, m8
%endif

    ; sub p2/q2, add q5/q6 [reuse -p2,+q5 from F][-q2,+q6] K
    ; write +4
    paddw            m10, m30               ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
    paddw            m11, m15
%ifidn %2, h
    SWAP             m15, m8
%endif
    punpcklbw         m8, m14, m24
    punpckhbw         m9, m14, m24
    SWAP              14, 7
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
    pmulhrsw          m8, m10, pw2048
    pmulhrsw          m9, m11, pw2048
    packuswb          m8, m9
%ifidn %2, v
    vmovdqu8 [t0+strideq*0]{k4}, m8         ; q4
%else
    vpblendmb    m17{k4}, m26, m8
%endif

    ; sub p1/q3, add q6*2 [reuse -p1,+q6 from G][-q3,+q6] L
    ; write +5
    paddw            m10, m18               ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
    paddw            m11, m23
    punpcklbw         m8, m22, m24
    punpckhbw         m9, m22, m24
    SWAP             m30, m24
    pmaddubsw         m8, pbm1_1
    pmaddubsw         m9, pbm1_1
    paddw            m10, m8
    paddw            m11, m9                ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
    pmulhrsw         m10, pw2048
    pmulhrsw         m11, pw2048
    packuswb         m10, m11
%ifidn %2, v
    vmovdqu8 [t0+strideq*1]{k4}, m10        ; q5
%else
    vmovdqu8     m27{k4}, m10
%endif

%ifidn %2, v
    lea               t0, [dstq+mstrideq*4]
%endif
%endif

%if %1 >= 8
    ; flat8 filter
    vpbroadcastd      m9, [pb_3_1]
    vpbroadcastd     m10, [pb_2_1]
%if %1 == 16
    vpbroadcastd     m23, [pb_1]
    vpbroadcastd      m0, [pb_4]
%elifidn %2, h
    vpbroadcastd     m31, [pb_m1_1]
 %define pbm1_1 m31
%endif
    punpcklbw        m24, m25, m3
    punpckhbw        m26, m25, m3
    pmaddubsw         m2, m24, m9
    pmaddubsw         m7, m26, m9           ; 3 * p3 + p1
    punpcklbw         m8, m13, m4
    punpckhbw        m11, m13, m4
    pmaddubsw         m8, m10
    pmaddubsw        m11, m10
    paddw             m2, m8
    paddw             m7, m11               ; 3 * p3 + 2 * p2 + p1 + p0
    punpcklbw         m8, m5, m0
    punpckhbw        m11, m5, m0
    pmaddubsw         m8, m23
    pmaddubsw        m11, m23
    paddw             m2, m8
    paddw             m7, m11               ; 3 * p3 + 2 * p2 + p1 + p0 + q0 + 4
    psrlw             m8, m2, 3
    psrlw            m11, m7, 3
    packuswb          m8, m11
%if is_h || %1 == 16
    vpblendmb    m10{k2}, m13, m8           ; p2
%endif
%ifidn %2, v
 %if %1 == 8
    vmovdqu8 [t0+strideq*1]{k2}, m8
 %else
    mova  [t0+strideq*1], m10
 %endif
%endif

    pmaddubsw         m8, m24, pbm1_1
    pmaddubsw        m11, m26, pbm1_1
    paddw             m2, m8
    paddw             m7, m11
    punpcklbw         m8, m13, m6
    punpckhbw        m11, m13, m6
    pmaddubsw         m8, pbm1_1
    pmaddubsw        m11, pbm1_1
    paddw             m2, m8
    paddw             m7, m11               ; 2 * p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4
    psrlw             m8, m2, 3
    psrlw            m11, m7, 3
    packuswb          m8, m11
    vpblendmb     m8{k2}, m3, m8            ; p1
%ifidn %2, v
    mova  [t0+strideq*2], m8
%else
    SWAP             m18, m8
%endif

    pmaddubsw        m24, m23
    pmaddubsw        m26, m23
    psubw             m2, m24
    psubw             m7, m26
    punpcklbw         m8, m4, m14
    punpckhbw        m11, m4, m14
    pmaddubsw         m8, m23
    pmaddubsw        m11, m23
    paddw             m2, m8
    paddw             m7, m11               ; p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4
    psrlw             m8, m2, 3
    psrlw            m11, m7, 3
    packuswb          m8, m11
    vpblendmb     m8{k2}, m4, m8            ; p0
%ifidn %2, v
    mova   [t0+stride3q], m8
%else
    SWAP             m29, m8
%endif

    punpcklbw        m24, m5, m22
    punpckhbw        m26, m5, m22
    pmaddubsw         m8, m24, m23
    pmaddubsw        m11, m26, m23
    paddw             m2, m8
    paddw             m7, m11
    punpcklbw         m8, m4, m25
    punpckhbw        m11, m4, m25
    pmaddubsw         m8, m23
    pmaddubsw        m11, m23
    psubw             m2, m8
    psubw             m7, m11               ; p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4
    psrlw             m8, m2, 3
    psrlw            m11, m7, 3
    packuswb          m8, m11
    vpblendmb    m11{k2}, m5, m8            ; q0
%ifidn %2, v
    mova [dstq+strideq*0], m11
%endif

    pmaddubsw        m24, pbm1_1
    pmaddubsw        m26, pbm1_1
    paddw             m2, m24
    paddw             m7, m26
    punpcklbw         m8, m13, m6
    punpckhbw        m13, m6
    pmaddubsw         m8, pbm1_1
    pmaddubsw        m13, pbm1_1
    paddw             m2, m8
    paddw             m7, m13               ; p1 + p0 + q0 + 2 * q1 + q2 + 2 * q3 + 4
    psrlw             m8, m2, 3
    psrlw            m13, m7, 3
    packuswb          m8, m13
    vpblendmb    m13{k2}, m6, m8            ; q1
%ifidn %2, v
    mova [dstq+strideq*1], m13
%endif

    punpcklbw        m24, m3, m6
    punpckhbw        m26, m3, m6
    pmaddubsw        m24, m23
    pmaddubsw        m26, m23
    psubw             m2, m24
    psubw             m7, m26
    punpcklbw        m24, m14, m22
    punpckhbw        m26, m14, m22
    pmaddubsw        m24, m23
    pmaddubsw        m26, m23
    paddw             m2, m24
    paddw             m7, m26               ; p0 + q0 + q1 + q2 + 2 * q2 + 3 * q3 + 4
    psrlw             m2, 3
    psrlw             m7, 3
    packuswb          m2, m7
%if is_h || %1 == 16
    vpblendmb     m2{k2}, m14, m2           ; q2
%endif
%ifidn %2, v
 %if %1 == 8
    vmovdqu8 [dstq+strideq*2]{k2}, m2
 %else
    mova [dstq+strideq*2], m2
 %endif
%endif

%ifidn %2, h
    SWAP             m24, m18
    SWAP             m26, m29
%if %1 == 8
    ; 16x8 transpose
    punpcklbw         m3, m25, m10
    punpckhbw        m25, m10
    punpcklbw        m10, m24, m26
    punpckhbw        m24, m26
    punpcklbw        m26, m11, m13
    punpckhbw        m11, m13
    punpcklbw        m13, m2, m22
    punpckhbw         m2, m22
    ;
    punpcklwd        m22, m3, m10
    punpckhwd         m3, m10
    punpcklwd        m10, m25, m24
    punpckhwd        m25, m24
    punpcklwd        m24, m26, m13
    punpckhwd        m26, m13
    punpcklwd        m13, m11, m2
    punpckhwd        m11, m2
    ;
    punpckldq         m2, m22, m24
    punpckhdq        m22, m24
    punpckldq        m24, m3, m26
    punpckhdq         m3, m26
    punpckldq        m26, m10, m13
    punpckhdq        m10, m13
    punpckldq        m13, m25, m11
    punpckhdq        m25, m11
    ; write 8x32
    vpbroadcastd    ym16, strided
    pmulld          ym16, [hmulD]
    lea               t1, [dstq+strideq*2]
    lea               t2, [dstq+strideq*4]
    lea               t3, [t1  +strideq*4]
    lea               t0, [dstq+strideq*8]
    kmovb             k1, k6
    kmovb             k2, k6
    kmovb             k3, k6
    kmovb             k4, k6
    vpscatterdq [dstq+ym16-4]{k1}, m2
    vpscatterdq [t1  +ym16-4]{k2}, m22
    vpscatterdq [t2  +ym16-4]{k3}, m24
    vpscatterdq [t3  +ym16-4]{k4}, m3
    lea               t1, [t0+strideq*2]
    lea               t2, [t0+strideq*4]
    lea               t3, [t1+strideq*4]
    kmovb             k1, k6
    kmovb             k2, k6
    kmovb             k3, k6
    kmovb             k4, k6
    vpscatterdq [t0+ym16-4]{k1}, m26
    vpscatterdq [t1+ym16-4]{k2}, m10
    vpscatterdq [t2+ym16-4]{k3}, m13
    vpscatterdq [t3+ym16-4]{k4}, m25
%else
    ; 16x16 transpose and store
    SWAP               5, 10, 2
    SWAP               6, 24
    SWAP               7, 26
    SWAP               8, 11
    SWAP               9, 13
    mova             m24, [rsp+0*64]
    SWAP             m26, m28
    mova              m2, [rsp+1*64]
    mova              m3, [rsp+2*64]
    mova              m4, [rsp+3*64]
    SWAP             m11, m16
    SWAP             m25, m17
    SWAP             m13, m27
    SWAP             m14, m30
    TRANSPOSE_16X16B 1, 0, [rsp+4*64]
    movu [dstq+strideq*0-8], xm24
    movu [dstq+strideq*1-8], xm26
    movu [dstq+strideq*2-8], xm2
    movu [dstq+stride3q -8], xm3
    lea               t0, [dstq+strideq*4]
    movu [t0+strideq*0-8], xm4
    movu [t0+strideq*1-8], xm5
    movu [t0+strideq*2-8], xm6
    movu [t0+stride3q -8], xm7
    lea               t0, [t0+strideq*4]
    movu [t0+strideq*0-8], xm8
    movu [t0+strideq*1-8], xm9
    movu [t0+strideq*2-8], xm10
    movu [t0+stride3q -8], xm11
    lea               t0, [t0+strideq*4]
    movu [t0+strideq*0-8], xm25
    movu [t0+strideq*1-8], xm13
    movu [t0+strideq*2-8], xm14
    movu [t0+stride3q -8], xm22
    lea               t0, [t0+strideq*4]
    vextracti128 [t0+strideq*0-8], ym24, 1
    vextracti128 [t0+strideq*1-8], ym26, 1
    vextracti128 [t0+strideq*2-8], ym2, 1
    vextracti128 [t0+stride3q -8], ym3, 1
    lea               t0, [t0+strideq*4]
    vextracti128 [t0+strideq*0-8], ym4, 1
    vextracti128 [t0+strideq*1-8], ym5, 1
    vextracti128 [t0+strideq*2-8], ym6, 1
    vextracti128 [t0+stride3q -8], ym7, 1
    lea               t0, [t0+strideq*4]
    vextracti128 [t0+strideq*0-8], ym8, 1
    vextracti128 [t0+strideq*1-8], ym9, 1
    vextracti128 [t0+strideq*2-8], ym10, 1
    vextracti128 [t0+stride3q -8], ym11, 1
    lea               t0, [t0+strideq*4]
    vextracti128 [t0+strideq*0-8], ym25, 1
    vextracti128 [t0+strideq*1-8], ym13, 1
    vextracti128 [t0+strideq*2-8], ym14, 1
    vextracti128 [t0+stride3q -8], ym22, 1
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m24, 2
    vextracti32x4 [t0+strideq*1-8], m26, 2
    vextracti32x4 [t0+strideq*2-8], m2, 2
    vextracti32x4 [t0+stride3q -8], m3, 2
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m4, 2
    vextracti32x4 [t0+strideq*1-8], m5, 2
    vextracti32x4 [t0+strideq*2-8], m6, 2
    vextracti32x4 [t0+stride3q -8], m7, 2
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m8, 2
    vextracti32x4 [t0+strideq*1-8], m9, 2
    vextracti32x4 [t0+strideq*2-8], m10, 2
    vextracti32x4 [t0+stride3q -8], m11, 2
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m25, 2
    vextracti32x4 [t0+strideq*1-8], m13, 2
    vextracti32x4 [t0+strideq*2-8], m14, 2
    vextracti32x4 [t0+stride3q -8], m22, 2
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m24, 3
    vextracti32x4 [t0+strideq*1-8], m26, 3
    vextracti32x4 [t0+strideq*2-8], m2, 3
    vextracti32x4 [t0+stride3q -8], m3, 3
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m4, 3
    vextracti32x4 [t0+strideq*1-8], m5, 3
    vextracti32x4 [t0+strideq*2-8], m6, 3
    vextracti32x4 [t0+stride3q -8], m7, 3
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m8, 3
    vextracti32x4 [t0+strideq*1-8], m9, 3
    vextracti32x4 [t0+strideq*2-8], m10, 3
    vextracti32x4 [t0+stride3q -8], m11, 3
    lea               t0, [t0+strideq*4]
    vextracti32x4 [t0+strideq*0-8], m25, 3
    vextracti32x4 [t0+strideq*1-8], m13, 3
    vextracti32x4 [t0+strideq*2-8], m14, 3
    vextracti32x4 [t0+stride3q -8], m22, 3
%endif
%endif

%elif %1 == 6
    ; flat6 filter
    vpbroadcastd     m15, [pb_3_1]
    vpbroadcastd     m12, [pb_2]
    punpcklbw         m8, m13, m5
    punpckhbw        m11, m13, m5
    pmaddubsw         m0, m8, m15
    pmaddubsw         m1, m11, m15
    punpcklbw         m7, m4, m3
    punpckhbw        m10, m4, m3
    pmaddubsw         m2, m7, m12
    pmaddubsw        m12, m10, m12
%ifidn %2, h
    vpbroadcastd     m15, [pb_m1_1]
 %define pbm1_1 m15
%endif
    paddw             m0, m2
    paddw             m1, m12
    pmulhrsw          m2, m0, m16
    pmulhrsw         m12, m1, m16
    packuswb          m2, m12
    vpblendmb     m2{k2}, m3, m2            ; p1
%ifidn %2, v
    mova  [t0+strideq*2], m2
%endif

    pmaddubsw         m8, pbm1_1
    pmaddubsw        m11, pbm1_1
    paddw             m0, m8
    paddw             m1, m11
    punpcklbw         m8, m13, m6
    punpckhbw        m11, m13, m6
    pmaddubsw         m8, pbm1_1
    pmaddubsw        m11, pbm1_1
    paddw             m0, m8
    paddw             m1, m11
    pmulhrsw         m12, m0, m16
    pmulhrsw         m13, m1, m16
    packuswb         m12, m13
    vpblendmb    m12{k2}, m4, m12           ; p0
%ifidn %2, v
    mova   [t0+stride3q], m12
%endif

    vpbroadcastd      m9, [pb_m1_2]
    vpbroadcastd      m4, [pb_m1_0]
    paddw             m0, m8
    paddw             m1, m11
    punpcklbw         m8, m3, m14
    punpckhbw        m11, m3, m14
    pmaddubsw        m14, m8, pbm1_1
    pmaddubsw        m13, m11, pbm1_1
    paddw             m0, m14
    paddw             m1, m13
    pmulhrsw         m14, m0, m16
    pmulhrsw         m13, m1, m16
    packuswb         m14, m13
    vpblendmb    m14{k2}, m5, m14           ; q0
%ifidn %2, v
    mova [dstq+strideq*0], m14
%endif

    pmaddubsw         m8, m9
    pmaddubsw        m11, m9
    paddw             m0, m8
    paddw             m1, m11
    pmaddubsw         m7, m4
    pmaddubsw        m10, m4
    paddw             m0, m7
    paddw             m1, m10
    pmulhrsw          m0, m16
    pmulhrsw          m1, m16
    packuswb          m0, m1
    vpblendmb     m0{k2}, m6, m0            ; q1
%ifidn %2, v
    mova [dstq+strideq*1], m0
%else
    TRANSPOSE_16x4_AND_WRITE_4x32 2, 12, 14, 0, 1
%endif
%else ; %1 == 4
%ifidn %2, v
    mova  [t0+strideq*0], m3                ; p1
    mova  [t0+strideq*1], m4                ; p0
    mova  [t0+strideq*2], m5                ; q0
    mova  [t0+stride3q ], m6                ; q1
%else
    TRANSPOSE_16x4_AND_WRITE_4x32 3, 4, 5, 6, 7
%endif
%endif
%endmacro

%define k7 k6

INIT_ZMM avx512icl
cglobal lpf_v_sb_y_8bpc, 7, 10, 32, dst, stride, mask, l, l_stride, \
                                    lut, w, stride3, mstride
 DECLARE_REG_TMP 9
    shl        l_strideq, 2
    sub               lq, l_strideq
    mov         mstrideq, strideq
    neg         mstrideq
    lea         stride3q, [strideq*3]
    mova             m21, [pb_4x0_4x4_4x8_4x12]
    mova             m20, [pb_mask]
    vpbroadcastd     m19, [pb_128]
    vpbroadcastd     m28, [pb_m1_1]
    vpbroadcastd     m27, [pw_2048]
 %define pbshuf m21
 %define pbmask m20
 %define pb128  m19
 %define pbm1_1 m28
 %define pw2048 m27

.loop:
    cmp   word [maskq+8], 0                 ; vmask[2]
    je .no_flat16

    FILTER            16, v
    jmp .end

.no_flat16:
    cmp   word [maskq+4], 0                 ; vmask[1]
    je .no_flat

    FILTER             8, v
    jmp .end

.no_flat:
    cmp   word [maskq+0], 0                 ; vmask[0]
    je .end

    call .v4

.end:
    add               lq, 64
    add             dstq, 64
    add            maskq, 2
    sub               wd, 16
    jg .loop
    RET
ALIGN function_align
RESET_MM_PERMUTATION
.v4:
    FILTER             4, v
    ret

cglobal lpf_h_sb_y_8bpc, 7, 13, 32, 5*64, dst, stride, mask, l, l_stride, \
                                          lut, h, stride3, stride8
 DECLARE_REG_TMP 9, 10, 11, 12
    shl        l_strideq, 2
    sub               lq, 4
    lea         stride3q, [strideq*3]
    lea         stride8q, [strideq*8]
    kxnorw            k6, k6, k6
    vpbroadcastd     m19, strided
    vpbroadcastd     m20, l_strided
    pmulld           m21, m19, [hmulA]
    pmulld           m20, [hmulB]
    pmulld           m19, [hmulC]
 %define pbshuf [pb_4x0_4x4_4x8_4x12]
 %define pbmask [pb_mask]
 %define pb128  [pb_128]{bcstd}
    shl        l_strideq, 1

.loop:
    cmp   word [maskq+8], 0                 ; vmask[2]
    je .no_flat16

    FILTER            16, h
    jmp .end

.no_flat16:
    cmp   word [maskq+4], 0                 ; vmask[1]
    je .no_flat

    FILTER             8, h
    jmp .end

.no_flat:
    cmp   word [maskq+0], 0                 ; vmask[0]
    je .end

    call .h4

.end:
    lea               lq, [lq+l_strideq*8]
    lea             dstq, [dstq+stride8q*8]
    add            maskq, 2
    sub               hd, 16
    jg .loop
    RET
ALIGN function_align
RESET_MM_PERMUTATION
.h4:
    FILTER             4, h
    ret

cglobal lpf_v_sb_uv_8bpc, 7, 10, 22, dst, stride, mask, l, l_stride, \
                                     lut, w, stride3, mstride
 DECLARE_REG_TMP 9
    shl        l_strideq, 2
    sub               lq, l_strideq
    mov         mstrideq, strideq
    neg         mstrideq
    lea         stride3q, [strideq*3]
    mova             m21, [pb_4x0_4x4_4x8_4x12]
    mova             m20, [pb_mask]
    vpbroadcastd     m19, [pb_128]
    vpbroadcastd     m17, [pb_m1_1]
    vpbroadcastd     m16, [pw_4096]
 %define pbshuf m21
 %define pbmask m20
 %define pb128  m19
 %define pbm1_1 m17

.loop:
    cmp   word [maskq+4], 0                 ; vmask[1]
    je .no_flat

    FILTER             6, v
    jmp .end

.no_flat:
    cmp   word [maskq+0], 0                 ; vmask[0]
    je .end

    call mangle(private_prefix %+ _lpf_v_sb_y_8bpc_avx512icl).v4

.end:
    add               lq, 64
    add             dstq, 64
    add            maskq, 2
    sub               wd, 16
    jg .loop
    RET

%undef k7
cglobal lpf_h_sb_uv_8bpc, 7, 12, 22, dst, stride, mask, l, l_stride, \
                                     lut, h, stride3, stride8
 DECLARE_REG_TMP 9, 10, 11
    mov              r7d, 0xffff
    movzx            r8d, r7b
    cmp               hd, 9
    cmovb            r7d, r8d
    kmovw             k6, r7d   ; h > 8 ? 0xffff : 0x00ff
    shl        l_strideq, 2
    sub               lq, 4
    kshiftrw          k7, k6, 4 ; h > 8 ? 0xff   : 0xf0
    lea         stride3q, [strideq*3]
    lea         stride8q, [strideq*8]
    vpbroadcastd     m19, strided
    vpbroadcastd     m20, l_strided
    pmulld           m21, m19, [hmulA]
    pmulld           m20, [hmulB]
    pmulld           m19, [hmulC]
    mova             m18, [pb_mask]
    vpbroadcastd     m17, [pb_128]
    vpbroadcastd     m16, [pw_4096]
 %define pbshuf [pb_4x0_4x4_4x8_4x12]
 %define pbmask m18
 %define pb128  m17
    add        l_strideq, l_strideq

.loop:
    cmp   word [maskq+4], 0                 ; vmask[1]
    je .no_flat

    FILTER             6, h
    jmp .end

.no_flat:
    cmp   word [maskq+0], 0                 ; vmask[0]
    je .end

    call mangle(private_prefix %+ _lpf_h_sb_y_8bpc_avx512icl).h4

.end:
    lea               lq, [lq+l_strideq*8]
    lea             dstq, [dstq+stride8q*8]
    add            maskq, 2
    sub               hd, 16
    jg .loop
    RET

%endif ; ARCH_X86_64
