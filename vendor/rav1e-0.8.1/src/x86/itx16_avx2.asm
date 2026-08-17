; Copyright © 2021, VideoLAN and dav1d authors
; Copyright © 2021, Two Orioles, LLC
; Copyright © 2021, Matthias Dressel
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
itx4_shuf:       dd 0x50401600, 0xd0c09284, 0x70603422, 0xf0e0b0a6
                 dd 0x50401701, 0xd0c09385, 0x70603523, 0xf0e0b1a7
idct4_12_shuf:   dd 0, 2, 4, 6, 1, 3, 5, 7
idct4_12_shuf2:  dd 2, 0, 6, 4, 3, 1, 7, 5
iadst8_12_shuf:  dd 0, 4, 1, 5, 2, 6, 3, 7
idct16_12_shuf:  dd 0, 4, 1, 5, 3, 7, 2, 6
iadst16_12_shuf: dd 3, 7, 0, 4, 2, 6, 1, 5
pw_2048_m2048:   dw  2048,  2048,  2048,  2048, -2048, -2048, -2048, -2048
idct4_shuf:   db  0,  1,  4,  5, 12, 13,  8,  9,  2,  3,  6,  7, 14, 15, 10, 11
idct32_shuf:  db  0,  1,  8,  9,  4,  5, 12, 13,  2,  3, 10, 11,  6,  7, 14, 15

%macro COEF_PAIR 2-3 0
pd_%1_%2: dd %1, %1, %2, %2
%define pd_%1 (pd_%1_%2 + 4*0)
%define pd_%2 (pd_%1_%2 + 4*2)
%if %3
dd -%2, -%2
%define pd_%2_m%2 pd_%2
%endif
%endmacro

COEF_PAIR  201,  995
COEF_PAIR  401, 1931
COEF_PAIR  799, 3406
COEF_PAIR 1380,  601
COEF_PAIR 1751, 2440
COEF_PAIR 2598, 1189
COEF_PAIR 2751, 2106
COEF_PAIR 2896, 1567, 1
COEF_PAIR 2896, 3784, 1
COEF_PAIR 3035, 3513
COEF_PAIR 3166, 3920
COEF_PAIR 3703, 3290
COEF_PAIR 3857, 4052
COEF_PAIR 4017, 2276
COEF_PAIR 4076, 3612
COEF_PAIR 4091, 3973

pd_8:      dd     8
pd_m601:   dd  -601
pd_m1189:  dd -1189
pd_m1380:  dd -1380
pd_m2106:  dd -2106
pd_m2598:  dd -2598
pd_m2751:  dd -2751
pd_m3344:  dd -3344
pd_1024:   dd  1024
pd_1321:   dd  1321
pd_1448:   dd  1448
pd_1697:   dd  1697
pd_2482:   dd  2482
pd_3072:   dd  3072 ; 1024 + 2048
pd_3803:   dd  3803
pd_5119:   dd  5119 ; 1024 + 4096 - 1
pd_5120:   dd  5120 ; 1024 + 4096
pd_5793:   dd  5793
pd_6144:   dd  6144 ; 2048 + 4096
pd_17408:  dd 17408 ; 1024 + 16384

pixel_10bpc_max: times 2 dw 0x03ff
pixel_12bpc_max: times 2 dw 0x0fff
dconly_10bpc:    times 2 dw 0x7c00
dconly_12bpc:    times 2 dw 0x7000
clip_18b_min:  dd -0x20000
clip_18b_max:  dd  0x1ffff
clip_20b_min:  dd -0x80000
clip_20b_max:  dd  0x7ffff

const idct64_mul_16bpc
dd 4095,  101, 2967, -2824,  3745, 1660, 3822, -1474,   401,  4076,   799,  4017
dd -700, 4036, 2359,  3349, -2191, 3461,  897,  3996, -2598, -3166, -4017,  -799
dd 4065,  501, 3229, -2520,  3564, 2019, 3948, -1092,  1931,  3612,  3406,  2276
dd -301, 4085, 2675,  3102, -1842, 3659, 1285,  3889, -1189, -3920, -2276, -3406

cextern deint_shuf
cextern idct64_mul
cextern pw_1697x8
cextern pw_1697x16
cextern pw_1567_3784
cextern pw_m1567_m3784
cextern pw_m3784_1567
cextern pw_2896_2896
cextern pw_m2896_2896
cextern pw_5
cextern pw_2048
cextern pw_4096
cextern pw_8192
cextern pw_16384
cextern pw_2896x8
cextern pd_2048

cextern idct_4x8_internal_8bpc_avx2.main
cextern idct_4x16_internal_8bpc_avx2.main
cextern idct_8x8_internal_8bpc_avx2.main
cextern idct_8x16_internal_8bpc_avx2.main
cextern idct_16x4_internal_8bpc_avx2.main
cextern idct_16x8_internal_8bpc_avx2.main
cextern idct_16x16_internal_8bpc_avx2.main
cextern inv_txfm_add_dct_dct_8x32_8bpc_avx2.main
cextern inv_txfm_add_dct_dct_8x32_8bpc_avx2.main_fast
cextern inv_txfm_add_dct_dct_16x32_8bpc_avx2.main_oddhalf
cextern inv_txfm_add_dct_dct_16x32_8bpc_avx2.main_oddhalf_fast
cextern inv_txfm_add_dct_dct_16x64_8bpc_avx2.main_part1
cextern inv_txfm_add_dct_dct_16x64_8bpc_avx2.main_part2_internal

cextern iadst_4x4_internal_8bpc_avx2.main
cextern iadst_4x8_internal_8bpc_avx2.main_pass2
cextern iadst_4x16_internal_8bpc_avx2.main2
cextern iadst_8x4_internal_8bpc_avx2.main
cextern iadst_8x8_internal_8bpc_avx2.main_pass2
cextern iadst_8x16_internal_8bpc_avx2.main
cextern iadst_8x16_internal_8bpc_avx2.main_pass2_end
cextern iadst_16x4_internal_8bpc_avx2.main
cextern iadst_16x8_internal_8bpc_avx2.main
cextern iadst_16x8_internal_8bpc_avx2.main_pass2_end
cextern iadst_16x16_internal_8bpc_avx2.main
cextern iadst_16x16_internal_8bpc_avx2.main_pass2_end

SECTION .text

%define m(x) mangle(private_prefix %+ _ %+ x %+ SUFFIX)

%macro WRAP_XMM 1+
    INIT_XMM cpuname
    %1
    INIT_YMM cpuname
%endmacro

%macro IWHT4_1D_PACKED 0
    ; m0 = in0 in2, m1 = in1 in3
    psubd                m2, m0, m1 ; t2
    paddd               xm0, xm1    ; t0
    vpermq               m2, m2, q3322
    vpermq               m0, m0, q1100
    vpermq               m1, m1, q3120
    psubd                m3, m0, m2
    psrad                m3, 1
    psubd                m3, m1     ; t1 t3
    psubd                m0, m3     ; ____ out0
    paddd                m2, m3     ; out3 ____
%endmacro

INIT_YMM avx2
cglobal inv_txfm_add_wht_wht_4x4_16bpc, 3, 7, 6, dst, stride, c, eob, bdmax
    mova                xm0, [cq+16*0]
    vinserti128          m0, [cq+16*2], 1
    mova                xm1, [cq+16*1]
    vinserti128          m1, [cq+16*3], 1
    pxor                 m4, m4
    mova          [cq+32*0], m4
    mova          [cq+32*1], m4
    lea                  r6, [dstq+strideq*2]
    psrad                m0, 2
    psrad                m1, 2
    IWHT4_1D_PACKED
    punpckhdq            m0, m3
    punpckldq            m3, m2
    punpckhqdq           m1, m0, m3
    punpcklqdq           m0, m3
    IWHT4_1D_PACKED
    vpblendd             m0, m2, 0x33
    packssdw             m0, m3
    vextracti128        xm2, m0, 1
    punpckhdq           xm1, xm0, xm2 ; out2 out1
    punpckldq           xm0, xm2      ; out3 out0
    movq                xm2, [r6  +strideq*1]
    movhps              xm2, [dstq+strideq*0]
    movq                xm3, [r6  +strideq*0]
    movhps              xm3, [dstq+strideq*1]
%ifidn bdmaxd, bdmaxm
    movd                xm5, bdmaxd
    vpbroadcastw        xm5, xm5
%else   ; win64: load from stack
    vpbroadcastw        xm5, bdmaxm
%endif
    paddsw              xm0, xm2
    paddsw              xm1, xm3
    pmaxsw              xm0, xm4
    pmaxsw              xm1, xm4
    pminsw              xm0, xm5
    pminsw              xm1, xm5
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movq   [r6  +strideq*0], xm1
    movq   [r6  +strideq*1], xm0
    RET

; dst1 = (src1 * coef1 - src2 * coef2 + rnd) >> 12
; dst2 = (src1 * coef2 + src2 * coef1 + rnd) >> 12
; flags: 1 = packed, 2 = inv_dst2
; skip round/shift if rnd is not a number
%macro ITX_MULSUB_2D 8-9 0 ; dst/src[1-2], tmp[1-3], rnd, coef[1-2], flags
%if %8 < 32
    pmulld              m%4, m%1, m%8
    pmulld              m%3, m%2, m%8
%else
%if %9 & 1
    vbroadcasti128      m%3, [pd_%8]
%else
    vpbroadcastd        m%3, [pd_%8]
%endif
    pmulld              m%4, m%1, m%3
    pmulld              m%3, m%2
%endif
%if %7 < 32
    pmulld              m%1, m%7
    pmulld              m%2, m%7
%else
%if %9 & 1
    vbroadcasti128      m%5, [pd_%7]
%else
    vpbroadcastd        m%5, [pd_%7]
%endif
    pmulld              m%1, m%5
    pmulld              m%2, m%5
%endif
%if %9 & 2
    psubd               m%4, m%6, m%4
    psubd               m%2, m%4, m%2
%else
%ifnum %6
    paddd               m%4, m%6
%endif
    paddd               m%2, m%4
%endif
%ifnum %6
    paddd               m%1, m%6
%endif
    psubd               m%1, m%3
%ifnum %6
    psrad               m%2, 12
    psrad               m%1, 12
%endif
%endmacro

%macro INV_TXFM_FN 4-5 10 ; type1, type2, eob_offset, size, bitdepth
cglobal inv_txfm_add_%1_%2_%4_%5bpc, 4, 5, 0, dst, stride, c, eob, tx2
    %define %%p1 m(i%1_%4_internal_%5bpc)
    ; Jump to the 1st txfm function if we're not taking the fast path, which
    ; in turn performs an indirect jump to the 2nd txfm function.
    lea tx2q, [m(i%2_%4_internal_%5bpc).pass2]
%ifidn %1_%2, dct_dct
    test               eobd, eobd
    jnz %%p1
%else
%if %3
    add                eobd, %3
%endif
    ; jump to the 1st txfm function unless it's located directly after this
    times ((%%end - %%p1) >> 31) & 1 jmp %%p1
ALIGN function_align
%%end:
%endif
%endmacro

%macro INV_TXFM_4X4_FN 2-3 10 ; type1, type2, bitdepth
    INV_TXFM_FN          %1, %2, 0, 4x4, %3
%ifidn %1_%2, dct_dct
    vpbroadcastd        xm2, [dconly_%3bpc]
%if %3 = 10
.dconly:
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 4
.dconly2:
    add                 r6d, 128
    sar                 r6d, 8
.dconly3:
    imul                r6d, 181
    add                 r6d, 2176
    sar                 r6d, 12
    movd                xm0, r6d
    paddsw              xm0, xm2
    vpbroadcastw        xm0, xm0
.dconly_loop:
    movq                xm1, [dstq+strideq*0]
    movhps              xm1, [dstq+strideq*1]
    paddsw              xm1, xm0
    psubusw             xm1, xm2
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    lea                dstq, [dstq+strideq*2]
    sub                 r3d, 2
    jg .dconly_loop
    WRAP_XMM RET
%else
    jmp m(inv_txfm_add_dct_dct_4x4_10bpc).dconly
%endif
%endif
%endmacro

%macro IDCT4_1D_PACKED 6 ; dst/src[1-2], tmp[1-3], rnd
    ITX_MULSUB_2D        %1, %2, %3, %4, %5, %6, 2896_1567, 2896_3784, 1
    punpckhqdq          m%3, m%2, m%1 ; t3 t2
    punpcklqdq          m%2, m%1      ; t0 t1
    paddd               m%1, m%2, m%3 ; out0 out1
    psubd               m%2, m%3      ; out3 out2
%endmacro

%macro IDCT4_1D_PACKED_WORD 6 ; dst/src[1-2], tmp[1-3], rnd
    vpbroadcastd        m%5, [pw_m3784_1567]
    punpckhwd           m%3, m%2, m%1
    vpbroadcastd        m%4, [pw_1567_3784]
    punpcklwd           m%2, m%1
    vpbroadcastd        m%1, [pw_m2896_2896]
    pmaddwd             m%5, m%3
    pmaddwd             m%3, m%4
    vpbroadcastd        m%4, [pw_2896_2896]
    pmaddwd             m%1, m%2
    pmaddwd             m%2, m%4
    REPX     {paddd x, m%6}, m%5, m%3, m%1, m%2
    REPX     {psrad x, 12 }, m%5, m%3, m%1, m%2
    packssdw            m%3, m%5      ; t3 t2
    packssdw            m%2, m%1      ; t0 t1
    paddsw              m%1, m%2, m%3 ; out0 out1
    psubsw              m%2, m%3      ; out3 out2
%endmacro

INV_TXFM_4X4_FN dct, dct
INV_TXFM_4X4_FN dct, identity
INV_TXFM_4X4_FN dct, adst
INV_TXFM_4X4_FN dct, flipadst

cglobal idct_4x4_internal_10bpc, 0, 7, 6, dst, stride, c, eob, tx2
    call .main
    vbroadcasti128       m2, [idct4_shuf]
    packssdw             m0, m1
    pshufb               m0, m2
    jmp                tx2q
.pass2:
    vextracti128        xm1, m0, 1
    WRAP_XMM IDCT4_1D_PACKED_WORD 0, 1, 2, 3, 4, 5
    packssdw            xm5, xm5 ; pw_2048
    pmulhrsw            xm0, xm5
    pmulhrsw            xm1, xm5
    movq                xm2, [dstq+strideq*0]
    movhps              xm2, [dstq+strideq*1]
    lea                  r6, [dstq+strideq*2]
    movq                xm3, [r6  +strideq*1]
    movhps              xm3, [r6  +strideq*0]
    vpbroadcastd        xm5, [pixel_10bpc_max]
    pxor                 m4, m4
    mova          [cq+32*0], m4
    mova          [cq+32*1], m4
    paddw               xm0, xm2
    paddw               xm1, xm3
    pmaxsw              xm0, xm4
    pmaxsw              xm1, xm4
    pminsw              xm0, xm5
    pminsw              xm1, xm5
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movhps [r6  +strideq*0], xm1
    movq   [r6  +strideq*1], xm1
    RET
ALIGN function_align
.main:
    vpermq               m0, [cq+32*0], q3120
    vpermq               m1, [cq+32*1], q3120
    vpbroadcastd         m5, [pd_2048]
.main2:
    IDCT4_1D_PACKED       0, 1, 2, 3, 4, 5
    ret

INV_TXFM_4X4_FN adst, dct
INV_TXFM_4X4_FN adst, adst
INV_TXFM_4X4_FN adst, flipadst
INV_TXFM_4X4_FN adst, identity

%macro IADST4_1D 0
    vpbroadcastd         m5, [pd_1321]
    vpbroadcastd         m7, [pd_2482]
    pmulld               m4, m0, m5    ; 1321*in0
    pmulld               m6, m3, m7    ; 2482*in3
    paddd                m4, m6        ; 1321*in0 + 2482*in3
    pmulld               m6, m0, m7    ; 2482*in0
    paddd                m0, m3        ; in0 + in3
    paddd                m7, m5        ; pd_3803
    pmulld               m5, m2        ; 1321*in2
    pmulld               m3, m7        ; 3803*in3
    pmulld               m7, m2        ; 3803*in2
    psubd                m2, m0        ; in2 - in0 - in3
    vpbroadcastd         m0, [pd_m3344]
    pmulld               m1, m0        ; -t3
    pmulld               m2, m0        ; out2 (unrounded)
    psubd                m6, m5        ; 2482*in0 - 1321*in2
    paddd                m4, m7        ;  t0
    psubd                m6, m3        ;  t1
    paddd                m3, m4, m6
    psubd                m4, m1        ; out0 (unrounded)
    psubd                m6, m1        ; out1 (unrounded)
    paddd                m3, m1        ; out3 (unrounded)
%endmacro

cglobal iadst_4x4_internal_10bpc, 0, 7, 6, dst, stride, c, eob, tx2
    call .main
    vinserti128          m0, m4, xm6, 1
    vinserti128          m1, m2, xm3, 1
.pass1_end:
    vpbroadcastd         m5, [pd_2048]
    mova                 m2, [itx4_shuf]
    paddd                m0, m5
    paddd                m1, m5
    psrad                m0, 12
    psrad                m1, 12
    packssdw             m0, m1
    vpermd               m0, m2, m0
    psrld                m2, 4
    pshufb               m0, m2
%if WIN64
    movaps             xmm6, [rsp+ 8]
    movaps             xmm7, [rsp+24]
%endif
    jmp                tx2q
.pass2:
    lea                  r6, [deint_shuf+128]
    vextracti128        xm1, m0, 1
    call m(iadst_4x4_internal_8bpc).main
.end:
    vpbroadcastd        xm4, [pw_2048]
    movq                xm2, [dstq+strideq*0]
    movhps              xm2, [dstq+strideq*1]
    lea                  r6, [dstq+strideq*2]
    movq                xm3, [r6  +strideq*0]
    movhps              xm3, [r6  +strideq*1]
    vpbroadcastd        xm5, [pixel_10bpc_max]
    pmulhrsw            xm0, xm4
    pmulhrsw            xm1, xm4
    pxor                 m4, m4
    mova          [cq+32*0], m4
    mova          [cq+32*1], m4
    paddw               xm0, xm2
    paddw               xm1, xm3
    pmaxsw              xm0, xm4
    pmaxsw              xm1, xm4
    pminsw              xm0, xm5
    pminsw              xm1, xm5
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [r6  +strideq*0], xm1
    movhps [r6  +strideq*1], xm1
    RET
ALIGN function_align
.main:
    mova                xm0, [cq+16*0]
    mova                xm1, [cq+16*1]
    mova                xm2, [cq+16*2]
    mova                xm3, [cq+16*3]
%if WIN64
    movaps         [rsp+16], xmm6
    movaps         [rsp+32], xmm7
%endif
.main2:
    WRAP_XMM IADST4_1D
    ret

INV_TXFM_4X4_FN flipadst, dct
INV_TXFM_4X4_FN flipadst, adst
INV_TXFM_4X4_FN flipadst, flipadst
INV_TXFM_4X4_FN flipadst, identity

cglobal iflipadst_4x4_internal_10bpc, 0, 7, 6, dst, stride, c, eob, tx2
    call m(iadst_4x4_internal_10bpc).main
    vinserti128          m0, m3, xm2, 1
    vinserti128          m1, m6, xm4, 1
    jmp m(iadst_4x4_internal_10bpc).pass1_end
.pass2:
    lea                  r6, [deint_shuf+128]
    vextracti128        xm1, m0, 1
    call m(iadst_4x4_internal_8bpc).main
    vpbroadcastd        xm4, [pw_2048]
    movq                xm3, [dstq+strideq*1]
    movhps              xm3, [dstq+strideq*0]
    lea                  r6, [dstq+strideq*2]
    movq                xm2, [r6  +strideq*1]
    movhps              xm2, [r6  +strideq*0]
    vpbroadcastd        xm5, [pixel_10bpc_max]
    pmulhrsw            xm0, xm4
    pmulhrsw            xm1, xm4
    pxor                 m4, m4
    mova          [cq+32*0], m4
    mova          [cq+32*1], m4
    paddw               xm0, xm2
    paddw               xm1, xm3
    pmaxsw              xm0, xm4
    pmaxsw              xm1, xm4
    pminsw              xm0, xm5
    pminsw              xm1, xm5
    movhps [dstq+strideq*0], xm1
    movq   [dstq+strideq*1], xm1
    movhps [r6  +strideq*0], xm0
    movq   [r6  +strideq*1], xm0
    RET

INV_TXFM_4X4_FN identity, dct
INV_TXFM_4X4_FN identity, adst
INV_TXFM_4X4_FN identity, flipadst
INV_TXFM_4X4_FN identity, identity

cglobal iidentity_4x4_internal_10bpc, 0, 7, 6, dst, stride, c, eob, tx2
    vpbroadcastd         m1, [pd_5793]
    pmulld               m0, m1, [cq+32*0]
    pmulld               m1,     [cq+32*1]
    vpbroadcastd         m5, [pd_2048]
    mova                 m3, [itx4_shuf]
    paddd                m0, m5
    paddd                m1, m5
    psrad                m0, 12
    psrad                m1, 12
    packssdw             m0, m1
    vpermd               m0, m3, m0
    psrld                m3, 4
    pshufb               m0, m3
    jmp                tx2q
.pass2:
    vpbroadcastd         m1, [pw_1697x8]
    movq                xm2, [dstq+strideq*0]
    movhps              xm2, [dstq+strideq*1]
    lea                  r6, [dstq+strideq*2]
    pmulhrsw             m1, m0
    paddsw               m0, m1
    movq                xm3, [r6  +strideq*0]
    movhps              xm3, [r6  +strideq*1]
    vpbroadcastd        xm4, [pixel_10bpc_max]
    packssdw             m5, m5 ; pw_2048
    pmulhrsw             m0, m5
    pxor                 m5, m5
    mova          [cq+32*0], m5
    mova          [cq+32*1], m5
    vextracti128        xm1, m0, 1
    paddw               xm0, xm2
    paddw               xm1, xm3
    pmaxsw              xm0, xm5
    pmaxsw              xm1, xm5
    pminsw              xm0, xm4
    pminsw              xm1, xm4
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [r6  +strideq*0], xm1
    movhps [r6  +strideq*1], xm1
    RET

INV_TXFM_4X4_FN dct, dct,      12
INV_TXFM_4X4_FN dct, identity, 12
INV_TXFM_4X4_FN dct, adst,     12
INV_TXFM_4X4_FN dct, flipadst, 12

cglobal idct_4x4_internal_12bpc, 0, 7, 8, dst, stride, c, eob, tx2
    call m(idct_4x4_internal_10bpc).main
    mova                 m3, [idct4_12_shuf]
    mova                 m4, [idct4_12_shuf2]
    vpermd               m2, m4, m1
    vpermd               m1, m3, m0
    jmp m(iadst_4x4_internal_12bpc).pass1_end2
.pass2:
    vpbroadcastd         m5, [pd_2048]
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q3120
    call m(idct_4x4_internal_10bpc).main2
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    jmp m(iadst_4x4_internal_12bpc).end

INV_TXFM_4X4_FN adst, dct,      12
INV_TXFM_4X4_FN adst, adst,     12
INV_TXFM_4X4_FN adst, flipadst, 12
INV_TXFM_4X4_FN adst, identity, 12

cglobal iadst_4x4_internal_12bpc, 0, 7, 8, dst, stride, c, eob, tx2
    call m(iadst_4x4_internal_10bpc).main
    vinserti128          m1, m4, xm6, 1
    vinserti128          m2, xm3, 1
.pass1_end:
    mova                 m3, [itx4_shuf]
    vpbroadcastd         m5, [pd_1024]
    psrad                m1, 1
    psrad                m2, 1
    vpermd               m1, m3, m1
    vpermd               m2, m3, m2
    paddd                m1, m5
    paddd                m2, m5
    psrad                m1, 11
    psrad                m2, 11
.pass1_end2:
    vpbroadcastd         m3, [clip_18b_min]
    vpbroadcastd         m4, [clip_18b_max]
    punpcklqdq           m0, m1, m2
    punpckhqdq           m1, m2
    pmaxsd               m0, m3
    pmaxsd               m1, m3
    pminsd               m0, m4
    pminsd               m1, m4
    jmp                tx2q
.pass2:
    call .main_pass2
    vinserti128          m0, m4, xm6, 1
    vinserti128          m1, m2, xm3, 1
.pass2_end:
    vpbroadcastd         m5, [pd_2048]
    paddd                m0, m5
    paddd                m1, m5
    psrad                m0, 12
    psrad                m1, 12
.end:
%if WIN64
    WIN64_RESTORE_XMM_INTERNAL
    %assign xmm_regs_used 6
%endif
.end2:
    vpbroadcastd         m4, [pw_16384]
    movq                xm2, [dstq+strideq*0]
    movq                xm3, [dstq+strideq*1]
    lea                  r6, [dstq+strideq*2]
    movhps              xm2, [r6  +strideq*0]   ; dst0 dst2
    movhps              xm3, [r6  +strideq*1]   ; dst1 dst3
    vpbroadcastd         m5, [pixel_12bpc_max]
    vinserti128          m2, xm3, 1
    psrad                m0, 3
    psrad                m1, 3
    packssdw             m0, m1     ; t0 t2 t1 t3
    pmulhrsw             m0, m4
    pxor                 m4, m4
    mova          [cq+32*0], m4
    mova          [cq+32*1], m4
    paddw                m0, m2     ; out0 out2 out1 out3
    pmaxsw               m0, m4
    pminsw               m0, m5
    vextracti128        xm1, m0, 1  ; out1 out3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [r6  +strideq*0], xm0
    movhps [r6  +strideq*1], xm1
    RET
.main_pass2:
    vextracti128        xm3, m1, 1
    mova                xm2, xm1
    vextracti128        xm1, m0, 1
    jmp m(iadst_4x4_internal_10bpc).main2

INV_TXFM_4X4_FN flipadst, dct,      12
INV_TXFM_4X4_FN flipadst, adst,     12
INV_TXFM_4X4_FN flipadst, flipadst, 12
INV_TXFM_4X4_FN flipadst, identity, 12

cglobal iflipadst_4x4_internal_12bpc, 0, 7, 8, dst, stride, c, eob, tx2
    call m(iadst_4x4_internal_10bpc).main
    vinserti128          m1, m3, xm2, 1
    vinserti128          m2, m6, xm4, 1
    jmp m(iadst_4x4_internal_12bpc).pass1_end
.pass2:
    call m(iadst_4x4_internal_12bpc).main_pass2
    vinserti128          m0, m3, xm2, 1
    vinserti128          m1, m6, xm4, 1
    jmp m(iadst_4x4_internal_12bpc).pass2_end

INV_TXFM_4X4_FN identity, dct,      12
INV_TXFM_4X4_FN identity, adst,     12
INV_TXFM_4X4_FN identity, flipadst, 12
INV_TXFM_4X4_FN identity, identity, 12

cglobal iidentity_4x4_internal_12bpc, 0, 7, 8, dst, stride, c, eob, tx2
    mova                 m2, [itx4_shuf]
    vpbroadcastd         m3, [pd_1697]
    vpermd               m0, m2, [cq+32*0]
    vpermd               m2, m2, [cq+32*1]
    vpbroadcastd         m5, [pd_2048]
    pmulld               m1, m3, m0
    pmulld               m3, m2
    paddd                m1, m5
    paddd                m3, m5
    psrad                m1, 12
    psrad                m3, 12
    paddd                m1, m0
    paddd                m2, m3
    jmp m(iadst_4x4_internal_12bpc).pass1_end2
.pass2:
    ; m0 = in0 in1
    ; m1 = in2 in3
    vpbroadcastd         m3, [pd_5793]
    vpbroadcastd         m5, [pd_2048]
    pmulld               m0, m3
    pmulld               m1, m3
    paddd                m0, m5 ; 2048
    paddd                m1, m5
    psrad                m0, 12
    psrad                m1, 12
    jmp m(iadst_4x4_internal_12bpc).end

%macro INV_TXFM_4X8_FN 2-3 10 ; type1, type2, bitdepth
    INV_TXFM_FN          %1, %2, 0, 4x8, %3
%ifidn %1_%2, dct_dct
    vpbroadcastd        xm2, [dconly_%3bpc]
%if %3 = 10
.dconly:
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 8
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    jmp m(inv_txfm_add_dct_dct_4x4_10bpc).dconly2
%else
    jmp m(inv_txfm_add_dct_dct_4x8_10bpc).dconly
%endif
%endif
%endmacro

%macro IDCT4_1D 8 ; src[1-4], tmp[1-3], rnd
    ITX_MULSUB_2D        %2, %4, %5, %6, %7, %8, 1567, 3784 ; t2, t3
    vpbroadcastd        m%5, [pd_2896]
    pmulld              m%1, m%5
    pmulld              m%3, m%5
    paddd               m%1, m%8
    paddd               m%5, m%1, m%3
    psubd               m%1, m%3
    psrad               m%5, 12 ; t0
    psrad               m%1, 12 ; t1
    psubd               m%3, m%1, m%2
    paddd               m%2, m%1
    paddd               m%1, m%5, m%4
    psubd               m%4, m%5, m%4
%endmacro

INV_TXFM_4X8_FN dct, dct
INV_TXFM_4X8_FN dct, identity
INV_TXFM_4X8_FN dct, adst
INV_TXFM_4X8_FN dct, flipadst

cglobal idct_4x8_internal_10bpc, 0, 7, 8, dst, stride, c, eob, tx2
.pass1:
    vpbroadcastd         m3, [pd_2896]
    pmulld               m0, m3, [cq+32*0]
    pmulld               m1, m3, [cq+32*1]
    pmulld               m2, m3, [cq+32*2]
    pmulld               m3, m3, [cq+32*3]
    vpbroadcastd         m7, [pd_2048]
    REPX      {paddd x, m7}, m0, m1, m2, m3
    REPX      {psrad x, 12}, m0, m1, m2, m3
    IDCT4_1D              0, 1, 2, 3, 4, 5, 6, 7
    jmp                tx2q
.pass2:
    packssdw             m0, m2
    packssdw             m1, m3
    lea                  r6, [deint_shuf+128]
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    punpckhdq            m1, m0, m2 ; 2 3
    punpckldq            m0, m2     ; 0 1
    vextracti128        xm2, m0, 1  ; 4 5
    vextracti128        xm3, m1, 1  ; 6 7
    call m(idct_4x8_internal_8bpc).main
    vpbroadcastd        xm4, [pw_2048]
    REPX  {pmulhrsw x, xm4}, xm0, xm1, xm2, xm3
    lea                  r3, [strideq*3]
    lea                  r6, [dstq+strideq*4]
    movq                xm4, [dstq+strideq*0]
    movhps              xm4, [dstq+strideq*1]
    movq                xm5, [dstq+r3       ]
    movhps              xm5, [dstq+strideq*2]
    movq                xm6, [r6  +strideq*0]
    movhps              xm6, [r6  +strideq*1]
    movq                xm7, [r6  +r3       ]
    movhps              xm7, [r6  +strideq*2]
    paddw               xm0, xm4 ; 0 1
    paddw               xm1, xm5 ; 3 2
    paddw               xm2, xm6 ; 4 5
    paddw               xm3, xm7 ; 7 6
    vpbroadcastd        xm5, [pixel_10bpc_max]
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    REPX    {pmaxsw x, xm4}, xm0, xm1, xm2, xm3
    REPX    {pminsw x, xm5}, xm0, xm1, xm2, xm3
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movhps [dstq+strideq*2], xm1
    movq   [dstq+r3       ], xm1
    movq   [r6  +strideq*0], xm2
    movhps [r6  +strideq*1], xm2
    movhps [r6  +strideq*2], xm3
    movq   [r6  +r3       ], xm3
    RET

INV_TXFM_4X8_FN adst, dct
INV_TXFM_4X8_FN adst, adst
INV_TXFM_4X8_FN adst, flipadst
INV_TXFM_4X8_FN adst, identity

cglobal iadst_4x8_internal_10bpc, 0, 7, 8, dst, stride, c, eob, tx2
    call m(iadst_8x4_internal_10bpc).main
    vpbroadcastd         m5, [pd_2048]
    paddd                m0, m5, m4
    paddd                m1, m5, m6
    paddd                m2, m5
    paddd                m3, m5
.pass1_end:
    REPX      {psrad x, 12}, m0, m1, m2, m3
    jmp                tx2q
.pass2:
    call .pass2_main
    mova                xm4, [pw_2048_m2048]
    REPX  {pmulhrsw x, xm4}, xm0, xm1, xm2, xm3
.end:
    lea                  r3, [strideq*3]
    lea                  r6, [dstq+strideq*4]
    movq                xm4, [dstq+strideq*0]
    movhps              xm4, [dstq+strideq*1]
    movq                xm5, [dstq+strideq*2]
    movhps              xm5, [dstq+r3       ]
    movq                xm6, [r6  +strideq*0]
    movhps              xm6, [r6  +strideq*1]
    movq                xm7, [r6  +strideq*2]
    movhps              xm7, [r6  +r3       ]
    paddw               xm0, xm4 ; 0 1
    paddw               xm1, xm5 ; 2 3
    paddw               xm2, xm6 ; 4 5
    paddw               xm3, xm7 ; 6 7
    vpbroadcastd        xm5, [pixel_10bpc_max]
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    REPX    {pmaxsw x, xm4}, xm0, xm1, xm2, xm3
    REPX    {pminsw x, xm5}, xm0, xm1, xm2, xm3
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+r3       ], xm1
    movq   [r6  +strideq*0], xm2
    movhps [r6  +strideq*1], xm2
    movq   [r6  +strideq*2], xm3
    movhps [r6  +r3       ], xm3
    RET
ALIGN function_align
.pass2_main:
    packssdw             m0, m2
    packssdw             m1, m3
    lea                  r6, [deint_shuf+128]
    punpcklwd            m4, m0, m1
    punpckhwd            m0, m1
    punpckhdq            m5, m4, m0
    punpckldq            m4, m0
    vextracti128        xm2, m4, 1      ; 4 5
    vextracti128        xm3, m5, 1      ; 6 7
    pshufd              xm4, xm4, q1032 ; 1 0
    pshufd              xm5, xm5, q1032 ; 3 2
    jmp m(iadst_4x8_internal_8bpc).main_pass2
ALIGN function_align
.main:
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
.main2:
    vbroadcasti128       m0, [cq+16*0]
    vbroadcasti128       m2, [cq+16*2]
    vbroadcasti128       m3, [cq+16*5]
    vbroadcasti128       m1, [cq+16*7]
    vpbroadcastd         m6, [pd_2896]
    shufpd               m0, m2, 0x0c ; 0 2
    shufpd               m1, m3, 0x0c ; 7 5
    vbroadcasti128       m2, [cq+16*4]
    vbroadcasti128       m4, [cq+16*6]
    vbroadcasti128       m5, [cq+16*1]
    vbroadcasti128       m3, [cq+16*3]
    vpbroadcastd         m7, [pd_2048]
    shufpd               m2, m4, 0x0c ; 4 6
    shufpd               m3, m5, 0x0c ; 3 1
    REPX {pmulld x, m6}, m0, m1, m2, m3
    REPX {paddd  x, m7}, m0, m1, m2, m3
    REPX {psrad  x, 12}, m0, m1, m2, m3
.main3:
    ITX_MULSUB_2D         1, 0, 4, 5, 6, 7,  401_1931, 4076_3612, 1
    ITX_MULSUB_2D         3, 2, 4, 5, 6, 7, 3166_3920, 2598_1189, 1
    psubd                m4, m0, m2   ; t4  t6
    paddd                m0, m2       ; t0  t2
    psubd                m2, m1, m3   ; t5  t7
    paddd                m1, m3       ; t1  t3
    REPX     {pmaxsd x, m8}, m4, m2, m0, m1
    REPX     {pminsd x, m9}, m4, m2, m0, m1
    pxor                 m5, m5
    psubd                m5, m4
    vpblendd             m4, m2, 0xcc ; t4  t7
    vpblendd             m2, m5, 0xcc ; t5 -t6
    ITX_MULSUB_2D         4, 2, 3, 5, 6, 7, 1567, 3784
    vpbroadcastd         m5, [pd_2896]
    vbroadcasti128       m6, [pw_2048_m2048] ; + + - -
    punpckhqdq           m3, m0, m1
    punpcklqdq           m0, m1
    psubd                m1, m0, m3   ; t2  t3
    paddd                m0, m3       ;  out0 -out7
    punpckhqdq           m3, m4, m2   ; t7a t6a
    punpcklqdq           m4, m2       ; t5a t4a
    psubd                m2, m4, m3   ; t7  t6
    paddd                m4, m3       ;  out6 -out1
    REPX     {pmaxsd x, m8}, m1, m2
    REPX     {pminsd x, m9}, m1, m2
    vpblendd             m3, m1, m2, 0xcc
    shufpd               m1, m2, 0x05
    pmulld               m3, m5
    pmulld               m5, m1
    psignd               m0, m6       ;  out0  out7
    psignd               m4, m6       ;  out6  out1
    paddd                m3, m7
    psubd                m2, m3, m5
    paddd                m5, m3
    psrad                m2, 12       ;  out4 -out5
    psrad                m5, 12       ; -out3  out2
    ret

INV_TXFM_4X8_FN flipadst, dct
INV_TXFM_4X8_FN flipadst, adst
INV_TXFM_4X8_FN flipadst, flipadst
INV_TXFM_4X8_FN flipadst, identity

cglobal iflipadst_4x8_internal_10bpc, 0, 7, 8, dst, stride, c, eob, tx2
    call m(iadst_8x4_internal_10bpc).main
    vpbroadcastd         m5, [pd_2048]
    paddd                m0, m5, m3
    paddd                m1, m5, m2
    paddd                m2, m5, m6
    paddd                m3, m5, m4
    jmp m(iadst_4x8_internal_10bpc).pass1_end
.pass2:
    call m(iadst_4x8_internal_10bpc).pass2_main
    mova                xm4, [pw_2048_m2048]
    REPX  {pmulhrsw x, xm4}, xm3, xm2, xm1, xm0
    lea                  r3, [strideq*3]
    lea                  r6, [dstq+strideq*4]
    movq                xm4, [dstq+strideq*1]
    movhps              xm4, [dstq+strideq*0]
    movq                xm5, [dstq+r3       ]
    movhps              xm5, [dstq+strideq*2]
    movq                xm6, [r6  +strideq*1]
    movhps              xm6, [r6  +strideq*0]
    movq                xm7, [r6  +r3       ]
    movhps              xm7, [r6  +strideq*2]
    paddw               xm3, xm4 ; 1 0
    paddw               xm2, xm5 ; 3 2
    paddw               xm1, xm6 ; 5 4
    paddw               xm0, xm7 ; 7 6
    vpbroadcastd        xm5, [pixel_10bpc_max]
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    REPX    {pmaxsw x, xm4}, xm3, xm2, xm1, xm0
    REPX    {pminsw x, xm5}, xm3, xm2, xm1, xm0
    movhps [dstq+strideq*0], xm3
    movq   [dstq+strideq*1], xm3
    movhps [dstq+strideq*2], xm2
    movq   [dstq+r3       ], xm2
    movhps [r6  +strideq*0], xm1
    movq   [r6  +strideq*1], xm1
    movhps [r6  +strideq*2], xm0
    movq   [r6  +r3       ], xm0
    RET

INV_TXFM_4X8_FN identity, dct
INV_TXFM_4X8_FN identity, adst
INV_TXFM_4X8_FN identity, flipadst
INV_TXFM_4X8_FN identity, identity

cglobal iidentity_4x8_internal_10bpc, 0, 7, 8, dst, stride, c, eob, tx2
.pass1:
    vpbroadcastd         m3, [pd_2896]
    pmulld               m0, m3, [cq+32*0]
    pmulld               m1, m3, [cq+32*1]
    pmulld               m2, m3, [cq+32*2]
    pmulld               m3,     [cq+32*3]
    vpbroadcastd         m5, [pd_2048]
    vpbroadcastd         m4, [pd_5793]
    REPX     {paddd  x, m5}, m0, m1, m2, m3
    REPX     {psrad  x, 12}, m0, m1, m2, m3
    REPX     {pmulld x, m4}, m0, m1, m2, m3
    REPX     {paddd  x, m5}, m0, m1, m2, m3
    REPX     {psrad  x, 12}, m0, m1, m2, m3
    jmp                tx2q
.pass2:
    vpbroadcastd         m6, [pixel_10bpc_max]
    call .pass2_end
    RET
ALIGN function_align
.pass2_end:
    vpbroadcastd         m4, [pw_4096]
    packssdw             m0, m2
    packssdw             m1, m3
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    pmulhrsw             m2, m4
    pmulhrsw             m0, m4
    punpckhdq            m1, m0, m2 ; 2 3 6 7
    punpckldq            m0, m2     ; 0 1 4 5
    lea                  r3, [strideq*3]
    lea                  r6, [dstq+strideq*4]
    movq                xm2, [dstq+strideq*0]
    movhps              xm2, [dstq+strideq*1]
    vpbroadcastq         m4, [r6  +strideq*0]
    vpbroadcastq         m5, [r6  +strideq*1]
    movq                xm3, [dstq+strideq*2]
    movhps              xm3, [dstq+r3       ]
    vpblendd             m2, m4, 0x30
    vpblendd             m2, m5, 0xc0
    vpbroadcastq         m4, [r6  +strideq*2]
    vpbroadcastq         m5, [r6  +r3       ]
    vpblendd             m3, m4, 0x30
    vpblendd             m3, m5, 0xc0
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    paddw                m0, m2 ; out0 out1 out4 out5
    paddw                m1, m3 ; out2 out3 out6 out7
    pmaxsw               m0, m4
    pmaxsw               m1, m4
    pminsw               m0, m6
    pminsw               m1, m6
    vextracti128        xm2, m0, 1  ; out4 out5
    vextracti128        xm3, m1, 1  ; out6 out7
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+r3       ], xm1
    movq   [r6  +strideq*0], xm2
    movhps [r6  +strideq*1], xm2
    movq   [r6  +strideq*2], xm3
    movhps [r6  +r3       ], xm3
    ret

INV_TXFM_4X8_FN dct, dct,      12
INV_TXFM_4X8_FN dct, identity, 12
INV_TXFM_4X8_FN dct, adst,     12
INV_TXFM_4X8_FN dct, flipadst, 12

cglobal idct_4x8_internal_12bpc, 0, 7, 10, dst, stride, c, eob, tx2
    jmp m(idct_4x8_internal_10bpc).pass1
.pass2:
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3
    REPX     {pminsd x, m9}, m0, m1, m2, m3
    ; transpose & interleave
    pshufd               m0, m0, q1320
    pshufd               m1, m1, q1320
    pshufd               m2, m2, q1320
    pshufd               m3, m3, q1320
    punpckldq            m4, m0, m1
    punpckhdq            m0, m1
    punpckldq            m5, m2, m3
    punpckhdq            m2, m3
    vpermq               m0, m0, q3102
    vpermq               m2, m2, q3102
    vperm2i128           m1, m0, m2, 0x31   ; 1 5 (interleaved)
    vperm2i128           m3, m0, m2, 0x20   ; 7 3 (interleaved)
    vperm2i128           m0, m4, m5, 0x20   ; 0 2 (interleaved)
    vperm2i128           m2, m4, m5, 0x31   ; 4 6 (interleaved)
    vpbroadcastd         m7, [pd_2048]
    call m(idct_8x4_internal_10bpc).main
    psubd                m3, m0, m4  ; out7 out6
    paddd                m0, m4      ; out0 out1
    paddd                m1, m2, m5  ; out3 out2
    psubd                m2, m5      ; out4 out5
    pshufd               m1, m1, q1032
    pshufd               m3, m3, q1032
    jmp m(iadst_4x8_internal_12bpc).end

INV_TXFM_4X8_FN adst, dct,      12
INV_TXFM_4X8_FN adst, adst,     12
INV_TXFM_4X8_FN adst, flipadst, 12
INV_TXFM_4X8_FN adst, identity, 12

cglobal iadst_4x8_internal_12bpc, 0, 7, 10, dst, stride, c, eob, tx2
    call m(iadst_8x4_internal_10bpc).main
    psrad                m0, m4, 1
    psrad                m1, m6, 1
    psrad                m2, 1
    psrad                m3, 1
.pass1_end:
    vpbroadcastd         m5, [pd_1024]
    REPX      {paddd x, m5}, m0, m1, m2, m3
    REPX      {psrad x, 11}, m0, m1, m2, m3
    jmp                tx2q
.pass2:
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3
    REPX     {pminsd x, m9}, m0, m1, m2, m3
    call .pass2_main
    vpblendd             m3, m0, m4, 0x33 ; out6 out7
    vpblendd             m0, m4, 0xcc     ; out0 out1
    pshufd               m1, m5, q1032
    psignd               m2, m6           ; out4 out5
    psignd               m1, m6           ; out2 out3
.end:
    vpbroadcastd         m4, [pw_16384]
    REPX       {psrad x, 3}, m0, m1, m2, m3
    packssdw             m0, m2     ; 0 1 4 5 (interleaved)
    packssdw             m1, m3     ; 2 3 6 7 (interleaved)
    mova                 m2, [iadst8_12_shuf]
    vpermd               m0, m2, m0 ; 0 1 4 5
    vpermd               m1, m2, m1 ; 2 3 6 7
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    lea                  r3, [strideq*3]
    lea                  r6, [dstq+strideq*4]
    movq                xm4, [dstq+strideq*0]
    movhps              xm4, [dstq+strideq*1]
    movq                xm5, [dstq+strideq*2]
    movhps              xm5, [dstq+r3       ]
    movq                xm6, [r6  +strideq*0]
    movhps              xm6, [r6  +strideq*1]
    vinserti128          m4, xm6, 1
    movq                xm7, [r6  +strideq*2]
    movhps              xm7, [r6  +r3       ]
    vinserti128          m5, xm7, 1
    paddw                m0, m4 ; 0 1 4 5
    paddw                m1, m5 ; 2 3 6 7
    vpbroadcastd         m5, [pixel_12bpc_max]
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    REPX    {pmaxsw x,  m4}, m0, m1
    REPX    {pminsw x,  m5}, m0, m1
    vextracti128        xm2, m0, 1  ; out4 out5
    vextracti128        xm3, m1, 1  ; out6 out7
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+r3       ], xm1
    movq   [r6  +strideq*0], xm2
    movhps [r6  +strideq*1], xm2
    movq   [r6  +strideq*2], xm3
    movhps [r6  +r3       ], xm3
    RET
ALIGN function_align
.pass2_main:
    ; transpose & interleave
    pshufd               m0, m0, q1320
    pshufd               m1, m1, q1320
    pshufd               m2, m2, q1320
    pshufd               m3, m3, q1320
    punpckldq            m4, m0, m1
    punpckhdq            m0, m1
    punpckldq            m5, m2, m3
    punpckhdq            m2, m3
    vperm2i128           m1, m0, m2, 0x31   ; 7 5 (interleaved)
    vperm2i128           m3, m0, m2, 0x20   ; 3 1 (interleaved)
    vperm2i128           m0, m4, m5, 0x20   ; 0 2 (interleaved)
    vperm2i128           m2, m4, m5, 0x31   ; 4 6 (interleaved)
    vpbroadcastd         m7, [pd_2048]
    jmp m(iadst_4x8_internal_10bpc).main3

INV_TXFM_4X8_FN flipadst, dct,      12
INV_TXFM_4X8_FN flipadst, adst,     12
INV_TXFM_4X8_FN flipadst, flipadst, 12
INV_TXFM_4X8_FN flipadst, identity, 12

cglobal iflipadst_4x8_internal_12bpc, 0, 7, 10, dst, stride, c, eob, tx2
    call m(iadst_8x4_internal_10bpc).main
    psrad                m0, m3, 1
    psrad                m1, m2, 1
    psrad                m2, m6, 1
    psrad                m3, m4, 1
    jmp m(iadst_4x8_internal_12bpc).pass1_end
.pass2:
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3
    REPX     {pminsd x, m9}, m0, m1, m2, m3
    call m(iadst_4x8_internal_12bpc).pass2_main
    shufpd               m3, m4, m0, 0x05 ; out1 out0
    shufpd               m0, m4, 0x05     ; out7 out6
    psignd               m2, m6
    pshufd               m6, m6, q1032
    pshufd               m1, m2, q1032    ; out5 out4
    psignd               m2, m5, m6       ; out3 out2
    jmp m(iadst_4x8_internal_12bpc).end

INV_TXFM_4X8_FN identity, dct,      12
INV_TXFM_4X8_FN identity, adst,     12
INV_TXFM_4X8_FN identity, flipadst, 12
INV_TXFM_4X8_FN identity, identity, 12

cglobal iidentity_4x8_internal_12bpc, 0, 7, 10, dst, stride, c, eob, tx2
    jmp m(iidentity_4x8_internal_10bpc).pass1
.pass2:
    ; m0 = in0 in1
    ; m1 = in2 in3
    ; m2 = in4 in5
    ; m3 = in6 in7
    vpbroadcastd         m6, [pixel_12bpc_max]
    call m(iidentity_4x8_internal_10bpc).pass2_end
    RET

%macro INV_TXFM_4X16_FN 2-3 10 ; type1, type2, bitdepth
    INV_TXFM_FN          %1, %2, 0, 4x16, %3
%ifidn %1_%2, dct_dct
    imul                r6d, [cq], 181
    vpbroadcastd        xm2, [dconly_%3bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 16
    add                 r6d, 384
    sar                 r6d, 9
    jmp m(inv_txfm_add_dct_dct_4x4_10bpc).dconly3
%endif
%endmacro

INV_TXFM_4X16_FN dct, dct
INV_TXFM_4X16_FN dct, identity
INV_TXFM_4X16_FN dct, adst
INV_TXFM_4X16_FN dct, flipadst

cglobal idct_4x16_internal_10bpc, 0, 7, 11, dst, stride, c, eob, tx2
.pass1:
    vpbroadcastd        m10, [pd_3072]
    mova                 m1, [cq+32*2]
    mova                 m3, [cq+32*6]
    mova                 m5, [cq+32*3]
    mova                 m7, [cq+32*7]
    call .pass1_main
    pmulld               m0, m6, [cq+32*0]
    pmulld               m2, m6, [cq+32*4]
    pmulld               m4, m6, [cq+32*1]
    pmulld               m6,     [cq+32*5]
    call .pass1_main2
    REPX       {psrad x, 1}, m0, m1, m2, m3, m4, m5, m6, m7
    jmp                tx2q
.pass2:
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    lea                  r6, [deint_shuf+128]
    punpcklwd            m4, m2, m3
    punpckhwd            m2, m3
    punpckhwd            m5, m0, m1
    punpcklwd            m0, m1
    punpckhdq            m1, m0, m4     ; 2 3
    punpckldq            m0, m4         ; 0 1
    punpckldq            m4, m5, m2     ; 8 9
    punpckhdq            m5, m2         ; a b
    vextracti128        xm2, m0, 1      ; 4 5
    vextracti128        xm3, m1, 1      ; 6 7
    vextracti128        xm6, m4, 1      ; c d
    vextracti128        xm7, m5, 1      ; e f
    call m(idct_4x16_internal_8bpc).main
    vpbroadcastd         m9, [pw_2048]
    vinserti128          m0, m0, xm1, 1 ; 0 1   3 2
    vinserti128          m1, m2, xm3, 1 ; 4 5   7 6
    vinserti128          m2, m4, xm5, 1 ; 8 9   b a
    vinserti128          m3, m6, xm7, 1 ; c d   f e
    vpbroadcastd         m8, [pixel_10bpc_max]
    call .pass2_end
    RET
ALIGN function_align
.pass1_main:
    vpbroadcastd         m4, [pd_3784]
    vpbroadcastd         m8, [pd_1567]
    vpbroadcastd         m9, [pd_2048]
    vpbroadcastd         m6, [pd_1448]
    ITX_MULSUB_2D         1, 3, 0, 2, _, 9, 8, 4 ; t2l, t3l
    ITX_MULSUB_2D         5, 7, 4, 2, _, 9, 8, 4 ; t2h, t3h
    ret
ALIGN function_align
.pass1_main2:
    paddd                m0, m10
    paddd                m4, m10
    paddd                m8, m0, m2
    psubd                m0, m2
    paddd                m9, m4, m6
    psubd                m4, m6
    REPX      {psrad x, 11}, m8, m0, m9, m4 ; t0l, t1l, t0h, t1h
    psubd                m2, m0, m1
    paddd                m1, m0
    psubd                m6, m4, m5
    paddd                m5, m4
    paddd                m0, m8, m3
    psubd                m3, m8, m3
    paddd                m4, m9, m7
    psubd                m7, m9, m7
    ret
ALIGN function_align
.pass2_end:
    lea                  r6, [strideq*3]
    pxor                 m7, m7
    pmulhrsw             m0, m9
    call .write_4x4
    pmulhrsw             m0, m1, m9
    call .write_4x4
    pmulhrsw             m0, m2, m9
    call .write_4x4
    pmulhrsw             m0, m3, m9
    call .write_4x4
    ret
ALIGN function_align
.write_4x4:
    movq                xm4, [dstq+strideq*0]
    movhps              xm4, [dstq+strideq*1]
    vpbroadcastq         m5, [dstq+strideq*2]
    vpbroadcastq         m6, [dstq+r6       ]
    mova          [cq+32*0], m7
    mova          [cq+32*1], m7
    add                  cq, 32*2
    vpblendd             m4, m5, 0xc0
    vpblendd             m4, m6, 0x30
    paddw                m4, m0
    pmaxsw               m4, m7
    pminsw               m4, m8
    vextracti128        xm5, m4, 1
    movq   [dstq+strideq*0], xm4
    movhps [dstq+strideq*1], xm4
    movhps [dstq+strideq*2], xm5
    movq   [dstq+r6       ], xm5
    lea                dstq, [dstq+strideq*4]
    ret

INV_TXFM_4X16_FN adst, dct
INV_TXFM_4X16_FN adst, adst
INV_TXFM_4X16_FN adst, flipadst
INV_TXFM_4X16_FN adst, identity

cglobal iadst_4x16_internal_10bpc, 0, 7, 11, dst, stride, c, eob, tx2
    call m(iadst_16x4_internal_10bpc).main
    vpbroadcastd         m6, [pd_6144]
    call m(iadst_16x4_internal_10bpc).main_end
    psrad                m0, m4, 13
    psrad                m1, m5, 13
    psrad                m2, 13
    psrad                m3, 13
    psrad                m4, m8, 13
    psrad                m5, m9, 13
    psrad                m6, 13
    psrad                m7, 13
    jmp                tx2q
.pass2:
    call .pass2_main
    vpbroadcastd         m5, [pw_2048]
    vpbroadcastd         m8, [pixel_10bpc_max]
    lea                  r6, [strideq*3]
    vpblendd             m4, m3, m0, 0xcc ; -out3   out0   out2  -out1
    pshufd               m2, m2, q1032    ; -out11  out8   out10 -out9
    vpblendd             m3, m0, 0x33     ; -out15  out12  out14 -out13
    pxor                 m7, m7
    psubw                m9, m7, m5
    vpblendd             m9, m5, 0x3c     ; -2048   2048   2048  -2048
    pmulhrsw             m0, m4, m9
    call .write_4x4
    pmulhrsw             m0, m1, m9
    call .write_4x4
    pmulhrsw             m0, m2, m9
    call .write_4x4
    pmulhrsw             m0, m3, m9
    call .write_4x4
    RET
ALIGN function_align
.write_4x4:
    movq                xm4, [dstq+r6       ]
    movhps              xm4, [dstq+strideq*0]
    vpbroadcastq         m5, [dstq+strideq*1]
    vpbroadcastq         m6, [dstq+strideq*2]
    mova          [cq+32*0], m7
    mova          [cq+32*1], m7
    add                  cq, 32*2
    vpblendd             m4, m5, 0xc0
    vpblendd             m4, m6, 0x30
    paddw                m4, m0
    pmaxsw               m4, m7
    pminsw               m4, m8
    vextracti128        xm5, m4, 1
    movhps [dstq+strideq*0], xm4
    movhps [dstq+strideq*1], xm5
    movq   [dstq+strideq*2], xm5
    movq   [dstq+r6       ], xm4
    lea                dstq, [dstq+strideq*4]
    ret
ALIGN function_align
.pass2_main:
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    lea                  r6, [deint_shuf+128]
    punpcklwd            m4, m2, m3
    punpckhwd            m2, m3
    punpckhwd            m5, m0, m1
    punpcklwd            m0, m1
    punpckhdq            m1, m0, m4
    punpckldq            m0, m4
    punpckldq            m4, m5, m2
    punpckhdq            m5, m2
    vpblendd             m3, m0, m1, 0x33
    vpblendd             m0, m1, 0xcc
    shufpd               m2, m5, m4, 0x05
    shufpd               m4, m5, 0x05
    vperm2i128           m1, m0, m3, 0x31 ; 4 7   6 5
    vinserti128          m0, xm3, 1       ; 0 3   2 1
    vperm2i128           m3, m2, m4, 0x31 ; c f   e d ; ????
    vinserti128          m2, xm4, 1       ; b 8   9 a
    call m(iadst_4x16_internal_8bpc).main2
    vpbroadcastd         m5, [pw_2896x8]
    paddsw               m1, m2, m4
    psubsw               m2, m4
    pmulhrsw             m1, m5           ; -out7   out4   out6  -out5
    pmulhrsw             m2, m5           ;  out8  -out11 -out9   out10
    ret
ALIGN function_align
.main:
    vbroadcasti128       m0, [cq+16* 0]
    vbroadcasti128       m4, [cq+16* 2]
    vbroadcasti128       m1, [cq+16*15]
    vbroadcasti128       m5, [cq+16*13]
    vbroadcasti128       m2, [cq+16* 4]
    vbroadcasti128       m6, [cq+16* 6]
    vbroadcasti128       m3, [cq+16*11]
    vbroadcasti128       m7, [cq+16* 9]
    shufpd               m0, m4, 0x0c ;  0  2
    shufpd               m1, m5, 0x0c ; 15 13
    shufpd               m2, m6, 0x0c ;  4  6
    shufpd               m3, m7, 0x0c ; 11  9
    vbroadcasti128       m4, [cq+16* 8]
    vbroadcasti128       m6, [cq+16*10]
    vbroadcasti128       m5, [cq+16* 7]
    vbroadcasti128       m7, [cq+16* 5]
    shufpd               m4, m6, 0x0c ;  8 10
    shufpd               m5, m7, 0x0c ;  7  5
    vbroadcasti128       m6, [cq+16*12]
    vbroadcasti128       m7, [cq+16*14]
    shufpd               m6, m7, 0x0c ; 12 14
    vbroadcasti128       m7, [cq+16* 3]
    vbroadcasti128       m8, [cq+16* 1]
    shufpd               m7, m8, 0x0c ;  3  1
.main2:
    ; expects: m12 = clip_min   m13 = clip_max
    vpbroadcastd        m11, [pd_2048]
    ITX_MULSUB_2D         1, 0, 8, 9, 10, 11,  201_995,  4091_3973, 1
    ITX_MULSUB_2D         3, 2, 8, 9, 10, 11, 1751_2440, 3703_3290, 1
    ITX_MULSUB_2D         5, 4, 8, 9, 10, 11, 3035_3513, 2751_2106, 1
    ITX_MULSUB_2D         7, 6, 8, 9, 10, 11, 3857_4052, 1380_601,  1
    psubd                m8, m0, m4 ; t8a  t10a
    paddd                m0, m4     ; t0a  t2a
    psubd                m4, m1, m5 ; t9a  t11a
    paddd                m1, m5     ; t1a  t3a
    psubd                m5, m2, m6 ; t12a t14a
    paddd                m2, m6     ; t4a  t6a
    psubd                m6, m3, m7 ; t13a t15a
    paddd                m3, m7     ; t5a  t7a
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m8
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m8
    ITX_MULSUB_2D         8, 4, 7, 9, 10, 11,  799_3406, 4017_2276, 1
    ITX_MULSUB_2D         6, 5, 7, 9, 10, 11, 4017_2276, 10,        1
    psubd                m7, m0, m2 ; t4   t6
    paddd                m0, m2     ; t0   t2
    psubd                m2, m1, m3 ; t5   t7
    paddd                m1, m3     ; t1   t3
    psubd                m3, m4, m6 ; t12a t14a
    paddd                m4, m6     ; t8a  t10a
    psubd                m6, m8, m5 ; t13a t15a
    paddd                m8, m5     ; t9a  t11a
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m6, m7, m8
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m6, m7, m8
    punpcklqdq           m5, m3, m7 ; t12a t4
    punpckhqdq           m3, m7     ; t14a t6
    punpckhqdq           m7, m6, m2 ; t15a t7
    punpcklqdq           m6, m2     ; t13a t5
    ITX_MULSUB_2D         7, 3, 2, 9, 10, 11, 3784, 1567
    ITX_MULSUB_2D         5, 6, 2, 9, 10, 11, 1567, 10
    vpbroadcastd        m10, [pd_2896]
    vbroadcasti128       m9, [pw_2048_m2048] ; + + - -
    punpckhqdq           m2, m4, m0 ; t10a t2
    punpcklqdq           m4, m0     ; t8a  t0
    punpckhqdq           m0, m8, m1 ; t11a t3
    punpcklqdq           m8, m1     ; t9a  t1
    paddd                m1, m6, m7 ; out2   -out3
    psubd                m6, m7     ; t14a t6
    paddd                m7, m5, m3 ; -out13  out12
    psubd                m5, m3     ; t15a t7
    psubd                m3, m8, m0 ; t11  t3a
    paddd                m8, m0     ; out14  -out15
    paddd                m0, m4, m2 ; -out1   out0
    psubd                m4, m2     ; t10  t2a
    REPX    {pmaxsd x, m12}, m6, m5, m3, m4
    REPX    {pminsd x, m13}, m6, m5, m3, m4
    REPX    {pmulld x, m10}, m6, m5, m3, m4
    paddd                m6, m11
    paddd                m4, m11
    paddd                m2, m6, m5 ; -out5   out4
    psubd                m6, m5     ;  out10 -out11
    psubd                m5, m4, m3 ; -out9   out8
    paddd                m3, m4     ;  out6  -out7
    REPX     {psrad  x, 12}, m2, m3, m5, m6
    REPX     {psignd x, m9}, m1, m8, m3, m6
    pshufd               m9, m9, q1032
    REPX     {psignd x, m9}, m0, m7, m2, m5
    ret

INV_TXFM_4X16_FN flipadst, dct
INV_TXFM_4X16_FN flipadst, adst
INV_TXFM_4X16_FN flipadst, flipadst
INV_TXFM_4X16_FN flipadst, identity

cglobal iflipadst_4x16_internal_10bpc, 0, 7, 11, dst, stride, c, eob, tx2
.pass1:
    call m(iadst_16x4_internal_10bpc).main
    vpbroadcastd         m6, [pd_6144]
    call m(iadst_16x4_internal_10bpc).main_end
    psrad                m0, m3, 13
    psrad                m1, m2, 13
    psrad                m2, m5, 13
    psrad                m3, m4, 13
    psrad                m4, m7, 13
    psrad                m5, m6, 13
    psrad                m6, m9, 13
    psrad                m7, m8, 13
    jmp                tx2q
.pass2:
    call m(iadst_4x16_internal_10bpc).pass2_main
    vpbroadcastd         m5, [pw_2048]
    vpbroadcastd         m8, [pixel_10bpc_max]
    lea                  r6, [strideq*3]
    vpblendd             m4, m3, m0, 0x33 ; -out0   out3   out1  -out2
    pshufd               m2, m2, q1032    ; -out11  out8   out10 -out9
    vpblendd             m3, m0, 0xcc     ; -out12  out15  out13 -out14
    pxor                 m7, m7
    psubw                m9, m7, m5
    vpblendd             m9, m5, 0x3c     ; -2048   2048   2048  -2048
    pmulhrsw             m0, m4, m9
    call .write_4x4
    pmulhrsw             m0, m2, m9
    call .write_4x4
    pmulhrsw             m0, m1, m9
    call .write_4x4
    pmulhrsw             m0, m3, m9
    call .write_4x4
    RET
ALIGN function_align
.write_4x4:
    movq                xm4, [dstq+strideq*0]
    movhps              xm4, [dstq+r6       ]
    vpbroadcastq         m5, [dstq+strideq*1]
    vpbroadcastq         m6, [dstq+strideq*2]
    mova          [cq+32*0], m7
    mova          [cq+32*1], m7
    add                  cq, 32*2
    vpblendd             m4, m5, 0x30
    vpblendd             m4, m6, 0xc0
    paddw                m4, m0
    pmaxsw               m4, m7
    pminsw               m4, m8
    vextracti128        xm5, m4, 1
    movq   [dstq+strideq*0], xm4
    movq   [dstq+strideq*1], xm5
    movhps [dstq+strideq*2], xm5
    movhps [dstq+r6       ], xm4
    lea                dstq, [dstq+strideq*4]
    ret

INV_TXFM_4X16_FN identity, dct
INV_TXFM_4X16_FN identity, adst
INV_TXFM_4X16_FN identity, flipadst
INV_TXFM_4X16_FN identity, identity

cglobal iidentity_4x16_internal_10bpc, 0, 7, 11, dst, stride, c, eob, tx2
    vpbroadcastd         m7, [pd_5793]
    pmulld               m0, m7, [cq+32*0]
    pmulld               m4, m7, [cq+32*1]
    pmulld               m1, m7, [cq+32*2]
    pmulld               m5, m7, [cq+32*3]
    pmulld               m2, m7, [cq+32*4]
    pmulld               m6, m7, [cq+32*5]
    pmulld               m3, m7, [cq+32*6]
    pmulld               m7,     [cq+32*7]
    vpbroadcastd         m8, [pd_6144]
    REPX      {paddd x, m8}, m0, m4, m1, m5, m2, m6, m3, m7
    REPX      {psrad x, 13}, m0, m4, m1, m5, m2, m6, m3, m7
    jmp                tx2q
.pass2:
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    vpbroadcastd         m7, [pw_1697x16]
    vpbroadcastd         m8, [pw_2048]
    pmulhrsw             m4, m7, m0
    pmulhrsw             m5, m7, m1
    pmulhrsw             m6, m7, m2
    pmulhrsw             m7, m3
    REPX      {paddsw x, x}, m0, m1, m2, m3
    paddsw               m0, m4
    paddsw               m1, m5
    paddsw               m2, m6
    paddsw               m3, m7
    vpbroadcastd         m4, [pixel_10bpc_max]
    call .pass2_end
    RET
ALIGN function_align
.pass2_end:
    punpckhwd            m7, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m2, m3
    punpcklwd            m2, m3
    lea                  r6, [strideq*5]
    pxor                 m3, m3
    punpckhdq            m5, m0, m2 ; 2 3   6 7
    punpckldq            m0, m2     ; 0 1   4 5
    punpckldq            m6, m7, m1 ; 8 9   c d
    punpckhdq            m7, m1     ; a b   e f
    pmulhrsw             m0, m8
    call .write_2x4x2
    pmulhrsw             m0, m5, m8
    call .write_2x4x2
    pmulhrsw             m0, m6, m8
    lea                dstq, [dstq+strideq*4]
    call .write_2x4x2
    pmulhrsw             m0, m7, m8
    call .write_2x4x2
    ret
ALIGN function_align
.write_2x4x2:
    movq                xm1, [dstq+strideq*0]
    movhps              xm1, [dstq+strideq*1]
    vpbroadcastq         m2, [dstq+strideq*4]
    vpblendd             m1, m2, 0x30
    vpbroadcastq         m2, [dstq+r6       ]
    vpblendd             m1, m2, 0xc0
    mova          [cq+32*0], m3
    mova          [cq+32*1], m3
    add                  cq, 32*2
    paddw                m1, m0
    pmaxsw               m1, m3
    pminsw               m1, m4
    vextracti128        xm2, m1, 1
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    movq   [dstq+strideq*4], xm2
    movhps [dstq+r6       ], xm2
    lea                dstq, [dstq+strideq*2]
    ret

INV_TXFM_4X16_FN dct, dct,      12
INV_TXFM_4X16_FN dct, identity, 12
INV_TXFM_4X16_FN dct, adst,     12
INV_TXFM_4X16_FN dct, flipadst, 12

cglobal idct_4x16_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    jmp m(idct_4x16_internal_10bpc).pass1
.pass2:
    punpckldq            m8, m0, m1
    punpckhdq            m0, m1
    punpckldq            m9, m2, m3
    punpckhdq            m2, m3
    punpckldq            m1, m4, m5
    punpckhdq            m4, m5
    punpckldq            m3, m6, m7
    punpckhdq            m6, m7
    punpcklqdq           m5, m0, m2         ;  2  6
    punpckhqdq          m12, m0, m2         ;  3  7
    punpcklqdq           m0, m8, m9         ;  0  4
    punpckhqdq          m10, m8, m9         ;  1  5
    punpcklqdq           m2, m1, m3         ;  8 12
    punpckhqdq          m13, m1, m3         ;  9 13
    punpcklqdq           m9, m4, m6         ; 10 14
    punpckhqdq           m4, m6             ; 11 15
    vperm2i128           m1,  m5,  m9, 0x20 ;  2 10
    vperm2i128           m3,  m9,  m5, 0x31 ; 14  6
    vpermq              m11,  m4, q1302     ; 15 11
    ; interleave
    REPX {vpermq x, x, q3120}, m0, m1, m2, m3, m10
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3, m10, m11, m12, m13
    REPX     {pminsd x, m9}, m0, m1, m2, m3, m10, m11, m12, m13
    call m(idct_16x4_internal_10bpc).pass1_main
    vpermq               m6, m12, q1302 ;  7  3
    vpermq               m5, m13, q3120 ;  9 13
    call m(idct_16x4_internal_10bpc).pass1_main2
    call m(idct_16x4_internal_10bpc).pass1_main3
    REPX       {psrad x, 3}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m1, m2, m3
    packssdw             m2, m4, m5
    packssdw             m3, m6, m7
    mova                 m4, [idct16_12_shuf]
    REPX  {vpermd x, m4, x}, m0, m1, m2, m3
    vpbroadcastd         m9, [pw_16384]
    vpbroadcastd         m8, [pixel_12bpc_max]
    call m(idct_4x16_internal_10bpc).pass2_end
    RET

INV_TXFM_4X16_FN adst, dct,      12
INV_TXFM_4X16_FN adst, adst,     12
INV_TXFM_4X16_FN adst, flipadst, 12
INV_TXFM_4X16_FN adst, identity, 12

cglobal iadst_4x16_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    call .main_pass1
    psrad                m0, m4, 12
    psrad                m1, m5, 12
    psrad                m2, 12
    psrad                m3, 12
    psrad                m4, m8, 12
    psrad                m5, m9, 12
    psrad                m6, 12
    psrad                m7, 12
    jmp                tx2q
.pass2:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call .transpose_16x4
    call m(iadst_4x16_internal_10bpc).main2
    pshufd               m4, m5, q1032
    psrad                m5, m6, 3
    pshufd               m6, m7, q1032
    psrad                m7, m8, 3
    REPX {pshufd x, x, q1032}, m0, m2
    REPX       {psrad x, 3}, m0, m1, m2, m3, m4, m6
.pass2_end:
    packssdw             m0, m1
    packssdw             m1, m2, m3
    packssdw             m2, m4, m5
    packssdw             m3, m6, m7
    mova                 m4, [iadst16_12_shuf]
    REPX  {vpermd x, m4, x}, m0, m1, m2, m3
    vpbroadcastd         m9, [pw_16384]
    vpbroadcastd         m8, [pixel_12bpc_max]
    lea                  r6, [strideq*3]
    pxor                 m7, m7
    pmulhrsw             m0, m9
    call m(iadst_4x16_internal_10bpc).write_4x4
    pmulhrsw             m0, m9, m1
    call m(iadst_4x16_internal_10bpc).write_4x4
    pmulhrsw             m0, m9, m2
    call m(iadst_4x16_internal_10bpc).write_4x4
    pmulhrsw             m0, m9, m3
    call m(iadst_4x16_internal_10bpc).write_4x4
    RET
ALIGN function_align
.transpose_16x4:
    ; transpose & interleave
    punpckldq            m8, m0, m1
    punpckhdq            m0, m1
    punpckldq            m9, m2, m3
    punpckhdq            m2, m3
    punpckldq            m1, m4, m5
    punpckhdq            m4, m5
    punpckldq            m3, m6, m7
    punpckhdq            m6, m7
    punpcklqdq          m10, m8, m0
    punpckhqdq           m0, m8
    punpcklqdq          m11, m9, m2
    punpckhqdq           m2, m9
    punpcklqdq           m8, m1, m4
    punpckhqdq           m4, m1
    punpcklqdq           m9, m3, m6
    punpckhqdq           m6, m3
    vperm2i128           m5,  m0,  m2, 0x31   ;  7  5
    vperm2i128           m7,  m0,  m2, 0x20   ;  3  1
    vperm2i128           m0, m10, m11, 0x20   ;  0  2
    vperm2i128           m2, m10, m11, 0x31   ;  4  6
    vperm2i128           m1,  m4,  m6, 0x31   ; 15 13
    vperm2i128           m3,  m4,  m6, 0x20   ; 11  9
    vperm2i128           m4,  m8,  m9, 0x20   ;  8 10
    vperm2i128           m6,  m8,  m9, 0x31   ; 12 14
    ret
ALIGN function_align
.main_pass1:
    call m(iadst_16x4_internal_10bpc).main
    vpbroadcastd         m6, [pd_3072]
    paddd               m10, m4, m5
    psubd                m4, m3
    psubd                m5, m3
    paddd                m3, m10
    psubd                m8, m7, m1
    paddd                m7, m9
    psubd                m9, m1
    paddd                m7, m1
    REPX      {psrad x, 1 }, m4, m5, m2, m3, m8, m9, m0, m7
    REPX      {paddd x, m6}, m4, m5, m2, m3, m8, m9, m7
    paddd                m6, m0
    ret

INV_TXFM_4X16_FN flipadst, dct,      12
INV_TXFM_4X16_FN flipadst, adst,     12
INV_TXFM_4X16_FN flipadst, flipadst, 12
INV_TXFM_4X16_FN flipadst, identity, 12

cglobal iflipadst_4x16_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    call m(iadst_4x16_internal_12bpc).main_pass1
    psrad                m0, m3, 12
    psrad                m1, m2, 12
    psrad                m2, m5, 12
    psrad                m3, m4, 12
    psrad                m4, m7, 12
    psrad                m5, m6, 12
    psrad                m6, m9, 12
    psrad                m7, m8, 12
    jmp                tx2q
.pass2:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(iadst_4x16_internal_12bpc).transpose_16x4
    call m(iadst_4x16_internal_10bpc).main2
    pshufd               m4, m3, q1032
    psrad                m3, m5, 3
    psrad                m5, m2, 3
    pshufd               m2, m6, q1032
    pshufd               m6, m1, q1032
    psrad                m1, m7, 3
    psrad                m7, m0, 3
    pshufd               m0, m8, q1032
    REPX       {psrad x, 3}, m0, m2, m4, m6
    jmp m(iadst_4x16_internal_12bpc).pass2_end

INV_TXFM_4X16_FN identity, dct,      12
INV_TXFM_4X16_FN identity, adst,     12
INV_TXFM_4X16_FN identity, flipadst, 12
INV_TXFM_4X16_FN identity, identity, 12

cglobal iidentity_4x16_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [pd_1697]
    mova                 m0, [cq+32*0]
    mova                 m4, [cq+32*1]
    mova                 m1, [cq+32*2]
    mova                 m5, [cq+32*3]
    vpbroadcastd         m9, [pd_6144]
    pmulld               m2, m8, m0
    pmulld               m6, m8, m4
    pmulld               m3, m8, m1
    pmulld               m7, m8, m5
    mova                m10, [cq+32*4]
    mova                m11, [cq+32*5]
    mova                m12, [cq+32*6]
    mova                m13, [cq+32*7]
    REPX     {paddd  x, m9}, m2, m6, m3, m7
    REPX     {psrad  x, 12}, m2, m6, m3, m7
    paddd                m0, m2
    pmulld               m2, m8, m10
    paddd                m4, m6
    pmulld               m6, m8, m11
    paddd                m1, m3
    pmulld               m3, m8, m12
    paddd                m5, m7
    pmulld               m7, m8, m13
    REPX     {psrad  x, 1 }, m0, m4, m1, m5
    REPX     {paddd  x, m9}, m2, m6, m3, m7
    REPX     {psrad  x, 12}, m2, m6, m3, m7
    paddd                m2, m10
    paddd                m6, m11
    paddd                m3, m12
    paddd                m7, m13
    REPX     {psrad  x, 1 }, m2, m6, m3, m7
    jmp                tx2q
.pass2:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    vpbroadcastd         m8, [pd_5793]
    vpbroadcastd         m9, [pd_1024]
    REPX     {pmulld x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {paddd  x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad  x, 14}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    vpbroadcastd         m8, [pw_16384]
    vpbroadcastd         m4, [pixel_12bpc_max]
    call m(iidentity_4x16_internal_10bpc).pass2_end
    RET

%macro INV_TXFM_8X4_FN 2-3 10 ; type1, type2, bitdepth
    INV_TXFM_FN          %1, %2, 0, 8x4, %3
%ifidn %1_%2, dct_dct
    vpbroadcastd         m2, [dconly_%3bpc]
%if %3 = 10
.dconly:
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 4
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    add                 r6d, 128
    sar                 r6d, 8
    jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly3
%else
    jmp m(inv_txfm_add_dct_dct_8x4_10bpc).dconly
%endif
%endif
%endmacro

INV_TXFM_8X4_FN dct, dct
INV_TXFM_8X4_FN dct, identity
INV_TXFM_8X4_FN dct, adst
INV_TXFM_8X4_FN dct, flipadst

cglobal idct_8x4_internal_10bpc, 0, 7, 10, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
.pass1:
    vbroadcasti128       m1, [cq+16*1]
    vbroadcasti128       m0, [cq+16*5]
    vbroadcasti128       m2, [cq+16*3]
    vbroadcasti128       m3, [cq+16*7]
    vpbroadcastd         m6, [pd_2896]
    shufpd               m1, m0, 0x0c ; 1 5
    shufpd               m3, m2, 0x0c ; 7 3
    vbroadcasti128       m0, [cq+16*0]
    vbroadcasti128       m4, [cq+16*2]
    vbroadcasti128       m2, [cq+16*4]
    vbroadcasti128       m5, [cq+16*6]
    vpbroadcastd         m7, [pd_2048]
    shufpd               m0, m4, 0x0c ; 0 2
    shufpd               m2, m5, 0x0c ; 4 6
    REPX {pmulld x, m6}, m1, m3, m0, m2
    REPX {paddd  x, m7}, m1, m3, m0, m2
    REPX {psrad  x, 12}, m1, m3, m0, m2
    call .main
    psubd                m3, m0, m4  ; out7 out6 (interleaved)
    paddd                m0, m4      ; out0 out1 (interleaved)
    paddd                m1, m2, m5  ; out3 out2 (interleaved)
    psubd                m2, m5      ; out4 out5 (interleaved)
    pshufd               m1, m1, q1032
    pshufd               m3, m3, q1032
    jmp                tx2q
.pass2:
    vbroadcasti128       m4, [deint_shuf]
    packssdw             m0, m1
    packssdw             m2, m3
    vperm2i128           m1, m0, m2, 0x31
    vinserti128          m0, xm2, 1
    pshufb               m0, m4
    pshufb               m1, m4
    IDCT4_1D_PACKED_WORD  0, 1, 2, 3, 4, 7
    vpermq               m0, m0, q3120 ; out0 out1
    vpermq               m2, m1, q2031 ; out2 out3
    jmp m(iadst_8x4_internal_10bpc).end
ALIGN function_align
.main:
    ITX_MULSUB_2D         1, 3, 4, 5, 6, 7, 799_3406, 4017_2276, 1
    IDCT4_1D_PACKED       0, 2, 4, 5, 6, 7
    vpbroadcastd         m6, [pd_2896]
    punpcklqdq           m4, m1, m3   ; t4a  t7a
    punpckhqdq           m1, m3       ; t5a  t6a
    psubd                m3, m4, m1   ; t5a  t6a
    paddd                m4, m1       ; t4   t7
    REPX     {pmaxsd x, m8}, m3, m4, m0, m2
    REPX     {pminsd x, m9}, m3, m4, m0, m2
    pmulld               m3, m6
    pshufd               m1, m3, q1032
    paddd                m3, m7
    psubd                m5, m3, m1
    paddd                m1, m3
    psrad                m5, 12
    psrad                m1, 12
    vpblendd             m5, m4, 0x33 ; t4   t5
    punpckhqdq           m4, m1       ; t7   t6
    ret

INV_TXFM_8X4_FN adst, dct
INV_TXFM_8X4_FN adst, adst
INV_TXFM_8X4_FN adst, flipadst
INV_TXFM_8X4_FN adst, identity

cglobal iadst_8x4_internal_10bpc, 0, 7, 10, dst, stride, c, eob, tx2
    call m(iadst_4x8_internal_10bpc).main
    vpblendd             m3, m0, m4, 0x33 ; out6 out7
    vpblendd             m0, m4, 0xcc     ; out0 out1
    pshufd               m1, m5, q1032
    psignd               m2, m6           ; out4 out5
    psignd               m1, m6           ; out2 out3
    jmp                tx2q
.pass2:
    call .pass2_main
    vpermq               m0, m0, q3120 ; out0 out1
    vpermq               m2, m1, q3120 ; out2 out3
.end:
    vpbroadcastd         m1, [pw_2048]
    pmulhrsw             m0, m1
    pmulhrsw             m1, m2
    vpbroadcastd         m5, [pixel_10bpc_max]
.end2:
    mova                xm2, [dstq+strideq*0]
    vinserti128          m2, [dstq+strideq*1], 1
    lea                  r6, [dstq+strideq*2]
    mova                xm3, [r6  +strideq*0]
    vinserti128          m3, [r6  +strideq*1], 1
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    paddw                m0, m2
    paddw                m1, m3
    pmaxsw               m0, m4
    pmaxsw               m1, m4
    pminsw               m0, m5
    pminsw               m1, m5
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [r6  +strideq*0], xm1
    vextracti128 [r6  +strideq*1], m1, 1
    RET
ALIGN function_align
.pass2_main:
    vbroadcasti128       m4, [deint_shuf]
    packssdw             m0, m1
    packssdw             m2, m3
    lea                  r6, [deint_shuf+128]
    vperm2i128           m1, m0, m2, 0x31
    vinserti128          m0, xm2, 1
    pshufb               m0, m4
    pshufb               m1, m4
    jmp m(iadst_8x4_internal_8bpc).main
ALIGN function_align
.main:
    vpbroadcastd         m1, [pd_2896]
    pmulld               m0, m1, [cq+32*0]
    pmulld               m3, m1, [cq+32*3]
    pmulld               m2, m1, [cq+32*2]
    pmulld               m1,     [cq+32*1]
    vpbroadcastd         m4, [pd_2048]
    REPX      {paddd x, m4}, m0, m3, m2, m1
    REPX      {psrad x, 12}, m0, m3, m2, m1
.main2:
    IADST4_1D
    ret

INV_TXFM_8X4_FN flipadst, dct
INV_TXFM_8X4_FN flipadst, adst
INV_TXFM_8X4_FN flipadst, flipadst
INV_TXFM_8X4_FN flipadst, identity

cglobal iflipadst_8x4_internal_10bpc, 0, 5, 10, dst, stride, c, eob, tx2
    call m(iadst_4x8_internal_10bpc).main
    shufpd               m3, m4, m0, 0x05
    shufpd               m0, m4, 0x05
    psignd               m2, m6
    pshufd               m6, m6, q1032
    pshufd               m1, m2, q1032
    psignd               m2, m5, m6
    jmp                tx2q
.pass2:
    call m(iadst_8x4_internal_10bpc).pass2_main
    vpermq               m2, m0, q2031
    vpermq               m0, m1, q2031
    jmp m(iadst_8x4_internal_10bpc).end

INV_TXFM_8X4_FN identity, dct
INV_TXFM_8X4_FN identity, adst
INV_TXFM_8X4_FN identity, flipadst
INV_TXFM_8X4_FN identity, identity

cglobal iidentity_8x4_internal_10bpc, 0, 7, 10, dst, stride, c, eob, tx2
.pass1:
    vpbroadcastd         m4, [pd_2896]
    vpermq               m0, [cq+32*0], q3120
    vpermq               m1, [cq+32*1], q3120
    vpermq               m2, [cq+32*2], q3120
    vpermq               m3, [cq+32*3], q3120
    vpbroadcastd         m7, [pd_2048]
    REPX     {pmulld x, m4}, m0, m1, m2, m3
    REPX     {paddd  x, m7}, m0, m1, m2, m3
    REPX     {psrad  x, 12}, m0, m1, m2, m3
    REPX     {paddd  x, x }, m0, m1, m2, m3
    jmp                tx2q
.pass2:
    vpbroadcastd         m5, [pixel_10bpc_max]
    vpbroadcastd         m4, [pw_1697x8]
    packssdw             m0, m1
    packssdw             m2, m3
    pmulhrsw             m1, m4, m0
    pmulhrsw             m4, m2
    paddsw               m0, m1
    paddsw               m2, m4
    packssdw             m7, m7 ; pw_2048
.pass2_end:
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    lea                  r6, [dstq+strideq*2]
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    pmulhrsw             m2, m7
    pmulhrsw             m0, m7
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    mova                xm2, [dstq+strideq*0]
    vinserti128          m2, [r6  +strideq*0], 1
    mova                xm3, [dstq+strideq*1]
    vinserti128          m3, [r6  +strideq*1], 1
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    paddw                m0, m2
    paddw                m1, m3
    pmaxsw               m0, m4
    pmaxsw               m1, m4
    pminsw               m0, m5
    pminsw               m1, m5
    mova         [dstq+strideq*0], xm0
    mova         [dstq+strideq*1], xm1
    vextracti128 [r6  +strideq*0], m0, 1
    vextracti128 [r6  +strideq*1], m1, 1
    RET

INV_TXFM_8X4_FN dct, dct,      12
INV_TXFM_8X4_FN dct, identity, 12
INV_TXFM_8X4_FN dct, adst,     12
INV_TXFM_8X4_FN dct, flipadst, 12

cglobal idct_8x4_internal_12bpc, 0, 7, 10, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [clip_20b_min]
    vpbroadcastd         m9, [clip_20b_max]
    jmp m(idct_8x4_internal_10bpc).pass1
.pass2:
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3
    REPX     {pminsd x, m9}, m0, m1, m2, m3
    call m(iadst_8x4_internal_12bpc).transpose_4x8
    IDCT4_1D              0, 1, 2, 3, 4, 5, 6, 7
    jmp m(iadst_8x4_internal_12bpc).end

INV_TXFM_8X4_FN adst, dct,      12
INV_TXFM_8X4_FN adst, adst,     12
INV_TXFM_8X4_FN adst, flipadst, 12
INV_TXFM_8X4_FN adst, identity, 12

cglobal iadst_8x4_internal_12bpc, 0, 7, 10, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [clip_20b_min]
    vpbroadcastd         m9, [clip_20b_max]
    call m(iadst_4x8_internal_10bpc).main2
    vpblendd             m3, m0, m4, 0x33 ; out6 out7
    vpblendd             m0, m4, 0xcc     ; out0 out1
    pshufd               m1, m5, q1032
    psignd               m2, m6           ; out4 out5
    psignd               m1, m6           ; out2 out3
    jmp                tx2q
.pass2:
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3
    REPX     {pminsd x, m9}, m0, m1, m2, m3
    call .pass2_main
    vpbroadcastd         m5, [pd_2048]
    paddd                m0, m5, m4
    paddd                m1, m5, m6
    paddd                m2, m5
    paddd                m3, m5
.pass2_end:
    REPX      {psrad x, 12}, m0, m1, m2, m3
.end:
    vpbroadcastd         m4, [pw_16384]
    REPX       {psrad x, 3}, m0, m1, m2, m3
    packssdw             m0, m1
    packssdw             m2, m3
    pmulhrsw             m0, m4
    pmulhrsw             m1, m2, m4
    vpermq               m0, m0, q3120 ; out0 out1
    vpermq               m1, m1, q3120 ; out2 out3
    vpbroadcastd         m5, [pixel_12bpc_max]
    jmp m(iadst_8x4_internal_10bpc).end2
ALIGN function_align
.pass2_main:
    call .transpose_4x8
    jmp m(iadst_8x4_internal_10bpc).main2
ALIGN function_align
.transpose_4x8:
    ; deinterleave
    pshufd               m0, m0, q3120
    pshufd               m1, m1, q3120
    pshufd               m2, m2, q3120
    pshufd               m3, m3, q3120
    ; transpose
    punpcklqdq           m4, m0, m1
    punpckhqdq           m0, m1
    punpcklqdq           m5, m2, m3
    punpckhqdq           m2, m3
    vperm2i128           m1, m0, m2, 0x20   ; out1
    vperm2i128           m3, m0, m2, 0x31   ; out3
    vperm2i128           m2, m4, m5, 0x31   ; out2
    vperm2i128           m0, m4, m5, 0x20   ; out0
    ret

INV_TXFM_8X4_FN flipadst, dct,      12
INV_TXFM_8X4_FN flipadst, adst,     12
INV_TXFM_8X4_FN flipadst, flipadst, 12
INV_TXFM_8X4_FN flipadst, identity, 12

cglobal iflipadst_8x4_internal_12bpc, 0, 5, 10, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [clip_20b_min]
    vpbroadcastd         m9, [clip_20b_max]
    call m(iadst_4x8_internal_10bpc).main2
    shufpd               m3, m4, m0, 0x05
    shufpd               m0, m4, 0x05
    psignd               m2, m6
    pshufd               m6, m6, q1032
    pshufd               m1, m2, q1032
    psignd               m2, m5, m6
    jmp                tx2q
.pass2:
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3
    REPX     {pminsd x, m9}, m0, m1, m2, m3
    call m(iadst_8x4_internal_12bpc).pass2_main
    vpbroadcastd         m5, [pd_2048]
    paddd                m0, m5, m3
    paddd                m1, m5, m2
    paddd                m3, m5, m4
    paddd                m2, m5, m6
    jmp m(iadst_8x4_internal_12bpc).pass2_end

INV_TXFM_8X4_FN identity, dct,      12
INV_TXFM_8X4_FN identity, adst,     12
INV_TXFM_8X4_FN identity, flipadst, 12
INV_TXFM_8X4_FN identity, identity, 12

cglobal iidentity_8x4_internal_12bpc, 0, 7, 10, dst, stride, c, eob, tx2
    jmp m(iidentity_8x4_internal_10bpc).pass1
.pass2:
    ; m0 = in0 in1 (interleaved)
    ; m1 = in2 in3 (interleaved)
    ; m2 = in4 in5 (interleaved)
    ; m3 = in6 in7 (interleaved)
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
    REPX     {pmaxsd x, m8}, m0, m1, m2, m3
    REPX     {pminsd x, m9}, m0, m1, m2, m3
    vpbroadcastd         m4, [pd_5793]
    REPX     {pmulld x, m4}, m0, m1, m2, m3
    REPX     {paddd  x, m7}, m0, m1, m2, m3
    REPX     {psrad  x, 15}, m0, m1, m2, m3
    vpbroadcastd         m5, [pixel_12bpc_max]
    vpbroadcastd         m7, [pw_16384]
    packssdw             m0, m1
    packssdw             m2, m3
    jmp m(iidentity_8x4_internal_10bpc).pass2_end

%macro INV_TXFM_8X8_FN 2-3 10 ; type1, type2, bitdepth
    INV_TXFM_FN          %1, %2, 0, 8x8, %3
%ifidn %1_%2, dct_dct
    vpbroadcastd         m2, [dconly_%3bpc]
%if %3 = 10
.dconly:
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 8
.dconly2:
    add                 r6d, 384
    sar                 r6d, 9
.dconly3:
    imul                r6d, 181
    add                 r6d, 2176
    sar                 r6d, 12
    movd                xm0, r6d
    paddsw              xm0, xm2
    vpbroadcastw         m0, xm0
.dconly_loop:
    mova                xm1, [dstq+strideq*0]
    vinserti128          m1, [dstq+strideq*1], 1
    paddsw               m1, m0
    psubusw              m1, m2
    mova         [dstq+strideq*0], xm1
    vextracti128 [dstq+strideq*1], m1, 1
    lea                dstq, [dstq+strideq*2]
    sub                 r3d, 2
    jg .dconly_loop
    RET
%else
    jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly
%endif
%endif
%endmacro

%macro IADST8_1D 14 ; src[1-8], tmp[1-3], pd_2048, clip[1-2]
    ITX_MULSUB_2D        %8, %1, %9, %10, %11, %12,  401, 4076 ; t1a, t0a
    ITX_MULSUB_2D        %2, %7, %9, %10, %11, %12, 3920, 1189 ; t7a, t6a
    ITX_MULSUB_2D        %6, %3, %9, %10, %11, %12, 1931, 3612 ; t3a, t2a
    ITX_MULSUB_2D        %4, %5, %9, %10, %11, %12, 3166, 2598 ; t5a, t4a
    psubd               m%9, m%3, m%7 ; t6
    paddd               m%3, m%7      ; t2
    psubd               m%7, m%1, m%5 ; t4
    paddd               m%1, m%5      ; t0
    psubd               m%5, m%6, m%2 ; t7
    paddd               m%6, m%2      ; t3
    psubd               m%2, m%8, m%4 ; t5
    paddd               m%8, m%4      ; t1
    REPX   {pmaxsd x, m%13}, m%7, m%2, m%9, m%5, m%3, m%1, m%6, m%8
    REPX   {pminsd x, m%14}, m%7, m%2, m%9, m%5, m%3, m%1, m%6, m%8
    ITX_MULSUB_2D        %7, %2, %4, %10, %11, %12, 1567, 3784 ; t5a, t4a
    ITX_MULSUB_2D        %5, %9, %4, %10, %11, %12, 3784, %11  ; t6a, t7a
    psubd              m%10, m%7, m%9 ;  t7
    paddd               m%7, m%9      ;  out6
    vpbroadcastd        m%9, [pd_1448]
    psubd               m%4, m%8, m%6 ;  t3
    paddd               m%8, m%6      ; -out7
    psubd               m%6, m%1, m%3 ;  t2
    paddd               m%1, m%3      ;  out0
    psubd               m%3, m%2, m%5 ;  t6
    paddd               m%2, m%5      ; -out1
    REPX   {pmaxsd x, m%13}, m%6, m%4, m%3, m%10
    REPX   {pminsd x, m%14}, m%6, m%4, m%3, m%10
    REPX   {pmulld x, m%9 }, m%6, m%4, m%3, m%10
    psubd               m%5, m%6, m%4  ; (t2 - t3) * 1448
    paddd               m%4, m%6       ; (t2 + t3) * 1448
    psubd               m%6, m%3, m%10 ; (t6 - t7) * 1448
    paddd               m%3, m%10      ; (t6 + t7) * 1448
%endmacro

INV_TXFM_8X8_FN dct, dct
INV_TXFM_8X8_FN dct, identity
INV_TXFM_8X8_FN dct, adst
INV_TXFM_8X8_FN dct, flipadst

cglobal idct_8x8_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    mova                 m0, [cq+32*0]
    mova                 m1, [cq+32*1]
    mova                 m2, [cq+32*2]
    mova                 m3, [cq+32*3]
    mova                 m4, [cq+32*4]
    mova                 m5, [cq+32*5]
    mova                 m6, [cq+32*6]
    mova                 m7, [cq+32*7]
    vpbroadcastd        m11, [pd_2048]
    call .main
    call .round_shift1
    jmp                tx2q
.pass2:
    call .transpose_8x8_packed
    call m(idct_8x8_internal_8bpc).main
    vpbroadcastd        m12, [pw_2048]
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    vpermq               m2, m2, q3120
    vpermq               m3, m3, q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call .write_8x4_start
    pmulhrsw             m0, m2, m12
    pmulhrsw             m1, m3, m12
    call .write_8x4
    RET
ALIGN function_align
.write_8x4_start:
    vpbroadcastd        m11, [pixel_10bpc_max]
    lea                  r6, [strideq*3]
    pxor                m10, m10
.write_8x4:
    mova                xm8, [dstq+strideq*0]
    vinserti128          m8, [dstq+strideq*1], 1
    mova                xm9, [dstq+strideq*2]
    vinserti128          m9, [dstq+r6       ], 1
    mova          [cq+32*0], m10
    mova          [cq+32*1], m10
    mova          [cq+32*2], m10
    mova          [cq+32*3], m10
    add                  cq, 32*4
    paddw                m0, m8
    paddw                m1, m9
    pmaxsw               m0, m10
    pmaxsw               m1, m10
    pminsw               m0, m11
    pminsw               m1, m11
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+r6       ], m1, 1
    lea                dstq, [dstq+strideq*4]
    ret
ALIGN function_align
.transpose_8x8_packed:
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    lea                  r6, [deint_shuf+128]
    punpckhwd            m4, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m2, m3
    punpcklwd            m2, m3
    punpckhdq            m3, m0, m2
    punpckldq            m0, m2
    punpckhdq            m2, m4, m1
    punpckldq            m4, m1
    vinserti128          m1, m3, xm2, 1
    vperm2i128           m3, m2, 0x31
    vperm2i128           m2, m0, m4, 0x31
    vinserti128          m0, xm4, 1
    ret
ALIGN function_align
.main_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
.main:
    ITX_MULSUB_2D         5, 3, 8, 9, 10, 11, 3406, 2276 ; t5a t6a
    ITX_MULSUB_2D         1, 7, 8, 9, 10, 11,  799, 4017 ; t4a t7a
    ITX_MULSUB_2D         2, 6, 8, 9, 10, 11, 1567, 3784 ; t2  t3
    paddd                m8, m1, m5 ; t4
    psubd                m1, m5     ; t5a
    paddd                m9, m7, m3 ; t7
    psubd                m7, m3     ; t6a
    vpbroadcastd         m3, [pd_2896]
    REPX    {pmaxsd x, m12}, m1, m8, m7, m9
    REPX    {pminsd x, m13}, m1, m8, m7, m9
    REPX    {pmulld x, m3 }, m0, m4, m7, m1
    paddd                m0, m11
    paddd                m7, m11
    psubd                m5, m0, m4
    paddd                m0, m4
    psubd                m4, m7, m1
    paddd                m7, m1
    REPX    {psrad  x, 12 }, m5, m0, m4, m7
    psubd                m3, m0, m6 ; dct4 out3
    paddd                m0, m6     ; dct4 out0
    paddd                m6, m5, m2 ; dct4 out1
    psubd                m5, m2     ; dct4 out2
    REPX    {pmaxsd x, m12}, m0, m6, m5, m3
    REPX    {pminsd x, m13}, m0, m6, m5, m3
    ret
ALIGN function_align
.round_shift1:
    pcmpeqd              m1, m1
    REPX      {psubd x, m1}, m0, m6, m5, m3
    paddd                m1, m6, m7 ; out1
    psubd                m6, m7     ; out6
    psubd                m7, m0, m9 ; out7
    paddd                m0, m9     ; out0
    paddd                m2, m5, m4 ; out2
    psubd                m5, m4     ; out5
    psubd                m4, m3, m8 ; out4
    paddd                m3, m8     ; out3
    REPX      {psrad x, 1 }, m0, m1, m2, m3, m4, m5, m6, m7
    ret

INV_TXFM_8X8_FN adst, dct
INV_TXFM_8X8_FN adst, adst
INV_TXFM_8X8_FN adst, flipadst
INV_TXFM_8X8_FN adst, identity

cglobal iadst_8x8_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    call .main
    call .main_end
    jmp                tx2q
.pass2:
    call m(idct_8x8_internal_10bpc).transpose_8x8_packed
    pshufd               m4, m0, q1032
    pshufd               m5, m1, q1032
    call m(iadst_8x8_internal_8bpc).main_pass2
    vpbroadcastd         m5, [pw_2048]
    vpbroadcastd       xm12, [pw_4096]
    psubw               m12, m5
    REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4_start
    pmulhrsw             m0, m2, m12
    pmulhrsw             m1, m3, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    RET
ALIGN function_align
.main:
    mova                 m0, [cq+32*0]
    mova                 m7, [cq+32*7]
    mova                 m1, [cq+32*1]
    mova                 m6, [cq+32*6]
    mova                 m2, [cq+32*2]
    mova                 m5, [cq+32*5]
    mova                 m3, [cq+32*3]
    mova                 m4, [cq+32*4]
    vpbroadcastd        m11, [pd_2048]
.main2:
    IADST8_1D             0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13
    psrld                m8, 10 ; pd_1
    vpbroadcastd         m9, [pd_3072]
    ret
ALIGN function_align
.main_end:
    paddd                m0, m8
    psubd                m1, m8, m1
    paddd                m6, m8
    psubd                m7, m8, m7
    REPX      {psrad x, 1 }, m0, m1, m6, m7
    ; (1 + ((x + 1024) >> 11)) >> 1 = (3072 + x) >> 12
    ; (1 - ((x + 1024) >> 11)) >> 1 = (3071 - x) >> 12
    psubd                m8, m9, m8 ; pd_3071
    paddd                m2, m9
    psubd                m3, m8, m3
    paddd                m4, m9
    psubd                m5, m8, m5
    REPX      {psrad x, 12}, m2, m3, m4, m5
    ret

INV_TXFM_8X8_FN flipadst, dct
INV_TXFM_8X8_FN flipadst, adst
INV_TXFM_8X8_FN flipadst, flipadst
INV_TXFM_8X8_FN flipadst, identity

cglobal iflipadst_8x8_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    call m(iadst_8x8_internal_10bpc).main
    call .main_end
    jmp                tx2q
.pass2:
    call m(idct_8x8_internal_10bpc).transpose_8x8_packed
    pshufd               m4, m0, q1032
    pshufd               m5, m1, q1032
    call m(iadst_8x8_internal_8bpc).main_pass2
    vpbroadcastd        m12, [pw_2048]
    vpbroadcastd        xm5, [pw_4096]
    psubw               m12, m5
    vpermq               m8, m3, q2031
    vpermq               m9, m2, q2031
    vpermq               m2, m1, q2031
    vpermq               m3, m0, q2031
    pmulhrsw             m0, m8, m12
    pmulhrsw             m1, m9, m12
    call m(idct_8x8_internal_10bpc).write_8x4_start
    pmulhrsw             m0, m2, m12
    pmulhrsw             m1, m3, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    RET
ALIGN function_align
.main_end:
    paddd               m10, m8, m0
    psubd                m0, m8, m7
    psubd                m7, m8, m1
    paddd                m1, m8, m6
    psrad                m0, 1
    psrad                m1, 1
    psrad                m6, m7, 1
    psrad                m7, m10, 1
    psubd                m8, m9, m8 ; pd_6143
    psubd               m10, m8, m5
    paddd                m5, m9, m2
    psubd                m2, m8, m3
    paddd                m3, m9, m4
    psrad                m4, m2, 12
    psrad                m2, m10, 12
    psrad                m3, 12
    psrad                m5, 12
    ret

INV_TXFM_8X8_FN identity, dct
INV_TXFM_8X8_FN identity, adst
INV_TXFM_8X8_FN identity, flipadst
INV_TXFM_8X8_FN identity, identity

cglobal iidentity_8x8_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
.pass1:
    mova                 m0, [cq+32*0]
    mova                 m1, [cq+32*1]
    mova                 m2, [cq+32*2]
    mova                 m3, [cq+32*3]
    mova                 m4, [cq+32*4]
    mova                 m5, [cq+32*5]
    mova                 m6, [cq+32*6]
    mova                 m7, [cq+32*7]
    jmp                tx2q
.pass2:
    packssdw             m3, m7
    vpbroadcastd         m7, [pixel_10bpc_max]
.pass2_main:
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    vpbroadcastd        m12, [pw_4096]
    punpckhwd            m4, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m2, m3
    punpcklwd            m2, m3
    punpckhdq            m3, m0, m2
    punpckldq            m0, m2
    punpckldq            m2, m4, m1
    punpckhdq            m4, m1
    punpckhqdq           m1, m0, m2 ; 1 5
    punpcklqdq           m0, m2     ; 0 4
    punpcklqdq           m2, m3, m4 ; 2 6
    punpckhqdq           m3, m4     ; 3 7
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call .write_2x8x2_start
    pmulhrsw             m0, m2, m12
    pmulhrsw             m1, m3, m12
    call .write_2x8x2_zero
    RET
.write_2x8x2_start:
    lea                  r6, [strideq*5]
    pxor                 m6, m6
.write_2x8x2_zero:
    mova          [cq+32*0], m6
    mova          [cq+32*1], m6
    mova          [cq+32*2], m6
    mova          [cq+32*3], m6
    add                  cq, 32*4
.write_2x8x2:
    mova                xm4, [dstq+strideq*0]
    vinserti128          m4, [dstq+strideq*4], 1
    mova                xm5, [dstq+strideq*1]
    vinserti128          m5, [dstq+r6       ], 1
    paddw                m0, m4
    paddw                m1, m5
    pmaxsw               m0, m6
    pmaxsw               m1, m6
    pminsw               m0, m7
    pminsw               m1, m7
    mova         [dstq+strideq*0], xm0
    mova         [dstq+strideq*1], xm1
    vextracti128 [dstq+strideq*4], m0, 1
    vextracti128 [dstq+r6       ], m1, 1
    lea                dstq, [dstq+strideq*2]
    ret

%macro TRANSPOSE_8X8_DWORD 12 ; src/dst[1-8], tmp[1-4]
    punpckldq            m%9,  m%1,  m%2 ; aibj emfn
    punpckhdq            m%1,  m%2       ; ckdl gohp
    punpckldq           m%10,  m%3,  m%4 ; qyrz uCvD
    punpckhdq            m%3,  m%4       ; sAtB wExF
    punpckldq           m%11,  m%5,  m%6 ; GOHP KSLT
    punpckhdq            m%5,  m%6       ; IQJR MUNV
    punpckldq           m%12,  m%7,  m%8 ; WeXf aibj
    punpckhdq            m%7,  m%8       ; YgZh ckdl
    punpcklqdq           m%2,  m%9, m%10 ; aiqy emuC
    punpckhqdq           m%9, m%10       ; bjrz fnvD
    punpcklqdq           m%4,  m%1,  m%3 ; cksA gowE
    punpckhqdq          m%10,  m%1,  m%3 ; dltB hpxF
    punpcklqdq           m%6, m%11, m%12 ; GOWe KSai
    punpckhqdq          m%11, m%12       ; HPXf LTbj
    punpcklqdq           m%8,  m%5,  m%7 ; IQYg MUck
    punpckhqdq          m%12,  m%5,  m%7 ; JRZh NVdl
    vperm2i128           m%1,  m%2,  m%6, 0x20   ; out0
    vperm2i128           m%5,  m%2,  m%6, 0x31   ; out4
    vperm2i128           m%2,  m%9, m%11, 0x20   ; out1
    vperm2i128           m%6,  m%9, m%11, 0x31   ; out5
    vperm2i128           m%3,  m%4,  m%8, 0x20   ; out2
    vperm2i128           m%7,  m%4,  m%8, 0x31   ; out6
    vperm2i128           m%4, m%10, m%12, 0x20   ; out3
    vperm2i128           m%8, m%10, m%12, 0x31   ; out7
%endmacro

INV_TXFM_8X8_FN dct, dct,      12
INV_TXFM_8X8_FN dct, identity, 12
INV_TXFM_8X8_FN dct, adst,     12
INV_TXFM_8X8_FN dct, flipadst, 12

cglobal idct_8x8_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(idct_8x8_internal_10bpc).pass1
.pass2:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call .transpose_8x8
    vpbroadcastd        m11, [pd_2048]
    call m(idct_8x8_internal_10bpc).main
    call .round_shift4
    jmp m(iadst_8x8_internal_12bpc).pass2_end
ALIGN function_align
.write_8x4_start:
    vpbroadcastd        m11, [pixel_12bpc_max]
    lea                  r6, [strideq*3]
    pxor                m10, m10
    ret
ALIGN function_align
.transpose_8x8:
    TRANSPOSE_8X8_DWORD 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
    ret
ALIGN function_align
.round_shift4:
    vpbroadcastd         m1, [pd_8]
    REPX      {paddd x, m1}, m0, m6, m5, m3
    paddd                m1, m6, m7 ; out1
    psubd                m6, m7     ; out6
    psubd                m7, m0, m9 ; out7
    paddd                m0, m9     ; out0
    paddd                m2, m5, m4 ; out2
    psubd                m5, m4     ; out5
    psubd                m4, m3, m8 ; out4
    paddd                m3, m8     ; out3
    REPX       {psrad x, 4}, m0, m1, m2, m3, m4, m5, m6, m7
    ret

INV_TXFM_8X8_FN adst, dct,      12
INV_TXFM_8X8_FN adst, adst,     12
INV_TXFM_8X8_FN adst, flipadst, 12
INV_TXFM_8X8_FN adst, identity, 12

cglobal iadst_8x8_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(iadst_8x8_internal_10bpc).pass1
.pass2:
    call .pass2_main
.pass2_end:
    packssdw             m0, m1
    packssdw             m1, m2, m3
    REPX {vpermq x, x, q3120}, m0, m1
    call m(idct_8x8_internal_12bpc).write_8x4_start
    call m(idct_8x8_internal_10bpc).write_8x4
    packssdw             m0, m4, m5
    packssdw             m1, m6, m7
    REPX {vpermq x, x, q3120}, m0, m1
    call m(idct_8x8_internal_10bpc).write_8x4
    RET
ALIGN function_align
.pass2_main:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_8x8_internal_12bpc).transpose_8x8
    vpbroadcastd        m11, [pd_2048]
.pass2_main2:
    call m(iadst_8x8_internal_10bpc).main2
    pslld                m9, m8, 3  ; pd_8
    paddd                m0, m9
    psubd                m1, m9, m1 ; 8+x
    paddd                m6, m9
    psubd                m7, m9, m7
    REPX       {psrad x, 4}, m0, m1, m6, m7
    vpbroadcastd         m9, [pd_17408]
    psubd                m8, m9, m8 ; 17407
    paddd                m2, m9
    psubd                m3, m8, m3
    paddd                m4, m9
    psubd                m5, m8, m5
    REPX      {psrad x, 15}, m2, m3, m4, m5
    ret

INV_TXFM_8X8_FN flipadst, dct,      12
INV_TXFM_8X8_FN flipadst, adst,     12
INV_TXFM_8X8_FN flipadst, flipadst, 12
INV_TXFM_8X8_FN flipadst, identity, 12

cglobal iflipadst_8x8_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(iflipadst_8x8_internal_10bpc).pass1
.pass2:
    call m(iadst_8x8_internal_12bpc).pass2_main
    packssdw             m7, m7, m6
    packssdw             m6, m1, m0
    packssdw             m1, m5, m4
    vpermq               m0, m7, q3120
    vpermq               m1, m1, q3120
    call m(idct_8x8_internal_12bpc).write_8x4_start
    call m(idct_8x8_internal_10bpc).write_8x4
    packssdw             m0, m3, m2
    vpermq               m0, m0, q3120
    vpermq               m1, m6, q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    RET

INV_TXFM_8X8_FN identity, dct,      12
INV_TXFM_8X8_FN identity, adst,     12
INV_TXFM_8X8_FN identity, flipadst, 12
INV_TXFM_8X8_FN identity, identity, 12

cglobal iidentity_8x8_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    jmp m(iidentity_8x8_internal_10bpc).pass1
.pass2:
    packssdw             m3, m7
    vpbroadcastd         m7, [pixel_12bpc_max]
    jmp m(iidentity_8x8_internal_10bpc).pass2_main

%macro INV_TXFM_8X16_FN 2-4 0,10 ; type1, type2, eob_offset, bitdepth
    INV_TXFM_FN          %1, %2, %3, 8x16, %4
%ifidn %1_%2, dct_dct
    imul                r6d, [cq], 181
    vpbroadcastd         m2, [dconly_%4bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 16
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly2
%endif
%endmacro

INV_TXFM_8X16_FN dct, dct
INV_TXFM_8X16_FN dct, identity, 35
INV_TXFM_8X16_FN dct, adst
INV_TXFM_8X16_FN dct, flipadst

cglobal idct_8x16_internal_10bpc, 0, 7, 16, dst, stride, c, eob, tx2
%undef cmp
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    vpbroadcastd        m14, [pd_2896]
    vpbroadcastd        m11, [pd_2048]
    cmp                eobd, 43
    jl .fast
    add                  cq, 32
    call .pass1_main
    sub                  cq, 32
    mova         [cq+32* 1], m0
    mova         [cq+32* 3], m1
    mova         [cq+32* 5], m2
    mova         [cq+32* 7], m3
    mova         [cq+32* 9], m4
    mova         [cq+32*11], m5
    mova         [cq+32*13], m6
    mova                m15, m7
    call .pass1_main
    mova                 m8, [cq+32* 1]
    mova                 m9, [cq+32* 3]
    mova                m10, [cq+32* 5]
    mova                m11, [cq+32* 7]
    mova                m12, [cq+32* 9]
    mova                m13, [cq+32*11]
    mova                m14, [cq+32*13]
    jmp                tx2q
.fast:
    call .pass1_main
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    call .transpose
    call m(idct_8x16_internal_8bpc).main
    vpbroadcastd        m12, [pw_2048]
    REPX {vpermq x, x, q3120}, m0, m2, m4, m6
    REPX {vpermq x, x, q2031}, m1, m3, m5, m7
.end:
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4_start
    pmulhrsw             m0, m2, m12
    pmulhrsw             m1, m3, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    pmulhrsw             m0, m4, m12
    pmulhrsw             m1, m5, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    pmulhrsw             m0, m6, m12
    pmulhrsw             m1, m7, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    RET
ALIGN function_align
.transpose:
    packssdw             m0, m8
    packssdw             m1, m9
    packssdw             m2, m10
    packssdw             m3, m11
    packssdw             m4, m12
    packssdw             m5, m13
    packssdw             m6, m14
    packssdw             m7, m15
    lea                  r6, [deint_shuf+128]
    punpckhwd            m8, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m2, m3
    punpcklwd            m2, m3
    punpcklwd            m3, m4, m5
    punpckhwd            m4, m5
    punpckhwd            m5, m6, m7
    punpcklwd            m6, m7
    punpckhdq            m7, m3, m6
    punpckldq            m3, m6
    punpckhdq            m6, m4, m5
    punpckldq            m4, m5
    punpckhdq            m5, m8, m1
    punpckldq            m8, m1
    punpckhdq            m1, m0, m2
    punpckldq            m0, m2
    vperm2i128           m2, m0, m3, 0x31
    vinserti128          m0, xm3, 1
    vperm2i128           m3, m1, m7, 0x31
    vinserti128          m1, xm7, 1
    vperm2i128           m7, m5, m6, 0x31
    vinserti128          m5, xm6, 1
    vperm2i128           m6, m8, m4, 0x31
    vinserti128          m4, m8, xm4, 1
    ret
ALIGN function_align
.pass1_main:
    pmulld               m0, m14, [cq+32* 0]
    pmulld               m1, m14, [cq+32* 2]
    pmulld               m2, m14, [cq+32* 4]
    pmulld               m3, m14, [cq+32* 6]
    pmulld               m4, m14, [cq+32* 8]
    pmulld               m5, m14, [cq+32*10]
    pmulld               m6, m14, [cq+32*12]
    pmulld               m7, m14, [cq+32*14]
    call m(idct_8x8_internal_10bpc).main_rect2
    jmp  m(idct_8x8_internal_10bpc).round_shift1
ALIGN function_align
.main_evenhalf:
    paddd                m1, m6, m7  ; idct8 out1
    psubd                m6, m7      ; idct8 out6
    psubd                m7, m0, m9  ; idct8 out7
    paddd                m0, m9      ; idct8 out0
    paddd                m2, m5, m4  ; idct8 out2
    psubd                m5, m4      ; idct8 out5
    psubd                m4, m3, m8  ; idct8 out4
    paddd                m3, m8      ; idct8 out3
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    ret
.main_oddhalf_fast_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3
    REPX     {psrad x, 12 }, m0, m1, m2, m3
.main_oddhalf_fast: ; lower half zero
    vpbroadcastd         m7, [pd_4076]
    vpbroadcastd         m8, [pd_401]
    vpbroadcastd         m6, [pd_m1189]
    vpbroadcastd         m9, [pd_3920]
    vpbroadcastd         m5, [pd_3612]
    vpbroadcastd        m10, [pd_1931]
    vpbroadcastd         m4, [pd_m2598]
    vpbroadcastd        m15, [pd_3166]
    pmulld               m7, m0
    pmulld               m0, m8
    pmulld               m6, m1
    pmulld               m1, m9
    pmulld               m5, m2
    pmulld               m2, m10
    pmulld               m4, m3
    pmulld               m3, m15
    jmp .main_oddhalf_fast2
.main_oddhalf_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
.main_oddhalf:
    ITX_MULSUB_2D         0, 7, 8, 9, 10, _,  401, 4076 ; t8a,  t15a
    ITX_MULSUB_2D         6, 1, 8, 9, 10, _, 3920, 1189 ; t11a, t12a
    ITX_MULSUB_2D         2, 5, 8, 9, 10, _, 1931, 3612 ; t10a, t13a
    ITX_MULSUB_2D         4, 3, 8, 9, 10, _, 3166, 2598 ; t9a,  t14a
.main_oddhalf_fast2:
    REPX     {paddd x, m11}, m0, m7, m6, m1, m2, m5, m4, m3
    REPX     {psrad x, 12 }, m0, m4, m6, m2, m1, m5, m7, m3
    psubd                m8, m0, m4 ; t9
    paddd                m0, m4     ; t8
    psubd                m4, m6, m2 ; t10
    paddd                m2, m6     ; t11
    psubd                m6, m1, m5 ; t13
    paddd                m5, m1     ; t12
    psubd                m1, m7, m3 ; t14
    paddd                m7, m3     ; t15
    REPX    {pmaxsd x, m12}, m8, m1, m4, m6, m0, m2, m5, m7
    REPX    {pminsd x, m13}, m8, m1, m4, m6, m0, m2, m5, m7
    vpbroadcastd        m15, [pd_3784]
    vpbroadcastd        m10, [pd_1567]
    ITX_MULSUB_2D         1, 8, 3, 9, _, 11, 10, 15
    ITX_MULSUB_2D         6, 4, 3, 9, _, 11, 10, 15, 2
    psubd                m3, m1, m4 ; t10
    paddd                m1, m4     ; t9
    psubd                m4, m0, m2 ; t11a
    paddd                m0, m2     ; t8a
    psubd                m2, m8, m6 ; t13
    paddd                m6, m8     ; t14
    psubd                m8, m7, m5 ; t12a
    paddd                m7, m5     ; t15a
    REPX    {pmaxsd x, m12}, m2, m8, m3, m4, m0, m1, m6, m7
    REPX    {pminsd x, m13}, m2, m8, m3, m4, m0, m1, m6, m7
    REPX    {pmulld x, m14}, m2, m8, m3, m4
    paddd                m2, m11
    paddd                m8, m11
    paddd                m5, m2, m3 ; t13a
    psubd                m2, m3     ; t10a
    psubd                m3, m8, m4 ; t11
    paddd                m4, m8     ; t12
    REPX      {psrad x, 12}, m5, m2, m3, m4
    mova          [r6-32*4], m7
    mova          [r6-32*3], m6
    mova          [r6-32*2], m5
    mova          [r6-32*1], m4
    mova          [r6+32*0], m3
    mova          [r6+32*1], m2
    mova          [r6+32*2], m1
    mova          [r6+32*3], m0
    ret

INV_TXFM_8X16_FN adst, dct
INV_TXFM_8X16_FN adst, adst
INV_TXFM_8X16_FN adst, flipadst
INV_TXFM_8X16_FN adst, identity, 35

cglobal iadst_8x16_internal_10bpc, 0, 7, 16, dst, stride, c, eob, tx2
%undef cmp
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    vpbroadcastd        m14, [pd_2896]
    vpbroadcastd        m11, [pd_2048]
    cmp                eobd, 43
    jl .fast
    add                  cq, 32
    call .pass1_main
    call m(iadst_8x8_internal_10bpc).main_end
    sub                  cq, 32
    mova         [cq+32* 1], m0
    mova         [cq+32* 3], m1
    mova         [cq+32* 5], m2
    mova         [cq+32* 7], m3
    mova         [cq+32* 9], m4
    mova         [cq+32*11], m5
    mova         [cq+32*13], m6
    mova                m15, m7
    call .pass1_main
    call m(iadst_8x8_internal_10bpc).main_end
    mova                 m8, [cq+32* 1]
    mova                 m9, [cq+32* 3]
    mova                m10, [cq+32* 5]
    mova                m11, [cq+32* 7]
    mova                m12, [cq+32* 9]
    mova                m13, [cq+32*11]
    mova                m14, [cq+32*13]
    jmp                tx2q
.fast:
    call .pass1_main
    call m(iadst_8x8_internal_10bpc).main_end
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    call m(idct_8x16_internal_10bpc).transpose
    call m(iadst_8x16_internal_8bpc).main
    call m(iadst_8x16_internal_8bpc).main_pass2_end
    vpbroadcastd         m8, [pw_2048]
    vpbroadcastd       xm12, [pw_4096]
    REPX {vpermq x, x, q2031}, m0, m1, m2, m3
    REPX {vpermq x, x, q3120}, m4, m5, m6, m7
    psubw               m12, m8
    jmp m(idct_8x16_internal_10bpc).end
ALIGN function_align
.pass1_main:
    pmulld               m0, m14, [cq+32* 0]
    pmulld               m7, m14, [cq+32*14]
    pmulld               m1, m14, [cq+32* 2]
    pmulld               m6, m14, [cq+32*12]
    pmulld               m2, m14, [cq+32* 4]
    pmulld               m5, m14, [cq+32*10]
    pmulld               m3, m14, [cq+32* 6]
    pmulld               m4, m14, [cq+32* 8]
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    jmp m(iadst_8x8_internal_10bpc).main2

INV_TXFM_8X16_FN flipadst, dct
INV_TXFM_8X16_FN flipadst, adst
INV_TXFM_8X16_FN flipadst, flipadst
INV_TXFM_8X16_FN flipadst, identity, 35

cglobal iflipadst_8x16_internal_10bpc, 0, 7, 16, dst, stride, c, eob, tx2
%undef cmp
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    vpbroadcastd        m14, [pd_2896]
    vpbroadcastd        m11, [pd_2048]
    cmp                eobd, 43
    jl .fast
    add                  cq, 32
    call m(iadst_8x16_internal_10bpc).pass1_main
    call m(iflipadst_8x8_internal_10bpc).main_end
    sub                  cq, 32
    mova         [cq+32* 1], m0
    mova         [cq+32* 3], m1
    mova         [cq+32* 5], m2
    mova         [cq+32* 7], m3
    mova         [cq+32* 9], m4
    mova         [cq+32*11], m5
    mova         [cq+32*13], m6
    mova                m15, m7
    call m(iadst_8x16_internal_10bpc).pass1_main
    call m(iflipadst_8x8_internal_10bpc).main_end
    mova                 m8, [cq+32* 1]
    mova                 m9, [cq+32* 3]
    mova                m10, [cq+32* 5]
    mova                m11, [cq+32* 7]
    mova                m12, [cq+32* 9]
    mova                m13, [cq+32*11]
    mova                m14, [cq+32*13]
    jmp                tx2q
.fast:
    call m(iadst_8x16_internal_10bpc).pass1_main
    call m(iflipadst_8x8_internal_10bpc).main_end
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    call m(idct_8x16_internal_10bpc).transpose
    call m(iadst_8x16_internal_8bpc).main
    call m(iadst_8x16_internal_8bpc).main_pass2_end
    vpbroadcastd        m12, [pw_2048]
    vpbroadcastd       xm13, [pw_4096]
    mova                m11, m0
    vpermq               m0, m7, q2031
    mova                m10, m1
    vpermq               m1, m6, q2031
    mova                 m9, m2
    vpermq               m2, m5, q2031
    mova                 m8, m3
    vpermq               m3, m4, q2031
    vpermq               m4, m8, q3120
    vpermq               m5, m9, q3120
    vpermq               m6, m10, q3120
    vpermq               m7, m11, q3120
    psubw               m12, m13
    jmp m(idct_8x16_internal_10bpc).end

INV_TXFM_8X16_FN identity, dct
INV_TXFM_8X16_FN identity, adst
INV_TXFM_8X16_FN identity, flipadst
INV_TXFM_8X16_FN identity, identity

%macro IDTX16 3-4 ; src/dst, tmp, pw_1697x16, [pw_16384]
    pmulhrsw            m%2, m%3, m%1
%if %0 == 4 ; if downshifting by 1
%ifnum %4
    pmulhrsw            m%2, m%4
%else ; without rounding
    psraw               m%2, 1
%endif
%else
    paddsw              m%1, m%1
%endif
    paddsw              m%1, m%2
%endmacro

cglobal iidentity_8x16_internal_10bpc, 0, 7, 16, dst, stride, c, eob, tx2
.pass1:
    vpbroadcastd        m15, [pd_2896]
    pmulld               m0, m15, [cq+32* 0]
    pmulld               m8, m15, [cq+32* 1]
    pmulld               m1, m15, [cq+32* 2]
    pmulld               m9, m15, [cq+32* 3]
    pmulld               m2, m15, [cq+32* 4]
    pmulld              m10, m15, [cq+32* 5]
    pmulld               m3, m15, [cq+32* 6]
    pmulld              m11, m15, [cq+32* 7]
    pmulld               m4, m15, [cq+32* 8]
    pmulld              m12, m15, [cq+32* 9]
    pmulld               m5, m15, [cq+32*10]
    pmulld              m13, m15, [cq+32*11]
    pmulld               m6, m15, [cq+32*12]
    pmulld              m14, m15, [cq+32*13]
    pmulld               m7, m15, [cq+32*14]
    pmulld              m15,      [cq+32*15]
    mova               [cq], m7
    vpbroadcastd         m7, [pd_2048]
    REPX     {paddd  x, m7}, m0,  m1,  m2,  m3,  m4,  m5,  m6, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    paddd                m7, [cq]
    REPX     {psrad  x, 12}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    packssdw             m0, m8
    packssdw             m1, m9
    packssdw             m2, m10
    packssdw             m3, m11
    packssdw             m4, m12
    packssdw             m5, m13
    packssdw             m6, m14
    packssdw            m13, m7, m15
    vpbroadcastd         m8, [pw_1697x16]
    REPX {IDTX16   x, 9, 8}, 0, 1, 2, 3, 4, 5, 6, 13
    vpbroadcastd         m7, [pixel_10bpc_max]
    vpbroadcastd        m12, [pw_2048]
    call .pass2_end
    RET
ALIGN function_align
.pass2_end:
    punpckhwd            m9, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m6, m13
    punpcklwd            m6, m13
    punpckhwd           m13, m4, m5
    punpcklwd            m4, m5
    punpcklwd            m5, m2, m3
    punpckhwd            m2, m3
    punpckhdq            m3, m0, m5
    punpckldq            m0, m5
    punpckhdq           m11, m9, m2
    punpckldq            m9, m2
    punpckldq            m2, m4, m6
    punpckhdq            m4, m6
    punpckldq            m6, m13, m1
    punpckhdq           m13, m1
    punpckhqdq           m1, m0, m2
    punpcklqdq           m0, m2
    punpcklqdq           m2, m3, m4
    punpckhqdq           m3, m4
    punpcklqdq           m8, m9, m6
    punpckhqdq           m9, m6
    punpcklqdq          m10, m11, m13
    punpckhqdq          m11, m13
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(iidentity_8x8_internal_10bpc).write_2x8x2_start
    pmulhrsw             m0, m12, m2
    pmulhrsw             m1, m12, m3
    call m(iidentity_8x8_internal_10bpc).write_2x8x2_zero
    pmulhrsw             m0, m12, m8
    pmulhrsw             m1, m12, m9
    lea                dstq, [dstq+strideq*4]
    call m(iidentity_8x8_internal_10bpc).write_2x8x2_zero
    pmulhrsw             m0, m12, m10
    pmulhrsw             m1, m12, m11
    call m(iidentity_8x8_internal_10bpc).write_2x8x2_zero
    ret

INV_TXFM_8X16_FN dct, dct,       0, 12
INV_TXFM_8X16_FN dct, identity, 35, 12
INV_TXFM_8X16_FN dct, adst,      0, 12
INV_TXFM_8X16_FN dct, flipadst,  0, 12

cglobal idct_8x16_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(idct_8x16_internal_10bpc).pass1
.pass2:
    lea                  r6, [rsp+32*4]
    call .transpose
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    mova         [cq+32* 8], m0
    mova         [cq+32*10], m2
    mova         [cq+32*12], m4
    mova         [cq+32*14], m6
    pmaxsd               m0, m12, [cq+32* 1]
    pmaxsd               m4, m12, m1
    pmaxsd               m1, m12, [cq+32* 3]
    pmaxsd               m2, m12, [cq+32* 5]
    pmaxsd               m6, m12, m5
    pmaxsd               m5, m12, m3
    pmaxsd               m3, m12, [cq+32* 7]
    pmaxsd               m7, m12
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m14, [pd_2896]
    call m(idct_8x16_internal_10bpc).main_oddhalf
    pmaxsd               m0, m12, [cq+32* 0]
    pmaxsd               m1, m12, [cq+32* 2]
    pmaxsd               m2, m12, [cq+32* 4]
    pmaxsd               m3, m12, [cq+32* 6]
    pmaxsd               m4, m12, [cq+32* 8]
    pmaxsd               m5, m12, [cq+32*10]
    pmaxsd               m6, m12, [cq+32*12]
    pmaxsd               m7, m12, [cq+32*14]
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
    vpbroadcastd        m11, [pd_8]
    REPX    {paddd  x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_16x8_internal_10bpc).pass1_rotations
    REPX       {psrad x, 4}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
.end:
    packssdw             m0, m1
    packssdw             m1, m2, m3
    packssdw             m2, m4, m5
    packssdw             m3, m6, m7
    packssdw             m4, m8, m9
    packssdw             m5, m10, m11
    packssdw             m6, m12, m13
    packssdw             m7, m14, m15
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q3120
    call m(idct_8x8_internal_12bpc).write_8x4_start
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m2, q3120
    vpermq               m1, m3, q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m4, q3120
    vpermq               m1, m5, q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m6, q3120
    vpermq               m1, m7, q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    RET
ALIGN function_align
.transpose:
    mova         [cq+32* 8], m8
    mova         [cq+32* 9], m9
    mova         [cq+32*10], m10
    mova         [cq+32*11], m11
    call m(idct_8x8_internal_12bpc).transpose_8x8
    mova         [cq+32* 0], m0
    mova         [cq+32* 1], m1
    mova         [cq+32* 2], m2
    mova         [cq+32* 3], m3
    mova         [cq+32* 4], m4
    mova         [cq+32* 5], m5
    mova         [cq+32* 6], m6
    mova         [cq+32* 7], m7
    mova                 m0, [cq+32* 8]
    mova                 m1, [cq+32* 9]
    mova                 m2, [cq+32*10]
    mova                 m3, [cq+32*11]
    mova                 m4, m12
    mova                 m5, m13
    mova                 m6, m14
    mova                 m7, m15
    jmp m(idct_8x8_internal_12bpc).transpose_8x8

INV_TXFM_8X16_FN adst, dct,       0, 12
INV_TXFM_8X16_FN adst, adst,      0, 12
INV_TXFM_8X16_FN adst, flipadst,  0, 12
INV_TXFM_8X16_FN adst, identity, 35, 12

cglobal iadst_8x16_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(iadst_8x16_internal_10bpc).pass1
.pass2:
    lea                  r6, [rsp+32*4]
    call .pass2_main
    call m(iadst_16x8_internal_10bpc).pass1_rotations
.pass2_end:
    REPX      {psrad x, 4 }, m0,  m1,  m2,  m3,  m12, m13, m14, m15
    REPX      {psrad x, 15}, m4,  m5,  m6,  m7,  m8,  m9,  m10, m11
    jmp m(idct_8x16_internal_12bpc).end
ALIGN function_align
.pass2_main:
    call m(idct_8x16_internal_12bpc).transpose
    vpbroadcastd        m13, [clip_18b_min]
    vpbroadcastd        m14, [clip_18b_max]
    mova         [cq+32* 8], m0
    mova         [cq+32*11], m3
    mova         [cq+32*12], m4
    mova         [cq+32*15], m7
    pmaxsd               m0, m13, [cq+32* 2] ;  2
    pmaxsd               m3, m13, m1         ;  9
    pmaxsd               m1, m13, m5         ; 13
    pmaxsd               m4, m13, m2         ; 10
    pmaxsd               m2, m13, [cq+32* 6] ;  6
    pmaxsd               m5, m13, [cq+32* 5] ;  5
    pmaxsd               m6, m13, m6         ; 14
    pmaxsd               m7, m13, [cq+32* 1] ;  1
    REPX    {pminsd x, m14}, m0, m1, m2, m3, m4, m5, m6, m7
    vpbroadcastd        m12, [pd_2048]
    vpbroadcastd        m15, [pd_2896]
    call m(iadst_16x8_internal_10bpc).main_part1
    pmaxsd               m0, m13, [cq+32* 0] ;  0
    pmaxsd               m1, m13, [cq+32*15] ; 15
    pmaxsd               m2, m13, [cq+32* 4] ;  4
    pmaxsd               m3, m13, [cq+32*11] ; 11
    pmaxsd               m4, m13, [cq+32* 8] ;  8
    pmaxsd               m5, m13, [cq+32* 7] ;  7
    pmaxsd               m6, m13, [cq+32*12] ; 12
    pmaxsd               m7, m13, [cq+32* 3] ;  3
    REPX    {pminsd x, m14}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(iadst_16x8_internal_10bpc).main_part2
    vpbroadcastd        m14, [pd_17408]
    psrld               m15, 11              ; pd_1
    psubd               m13, m14, m15        ; pd_17407
    pslld               m15, 3               ; pd_8
    ret

INV_TXFM_8X16_FN flipadst, dct,       0, 12
INV_TXFM_8X16_FN flipadst, adst,      0, 12
INV_TXFM_8X16_FN flipadst, flipadst,  0, 12
INV_TXFM_8X16_FN flipadst, identity, 35, 12

cglobal iflipadst_8x16_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(iflipadst_8x16_internal_10bpc).pass1
.pass2:
    lea                  r6, [rsp+32*4]
    call m(iadst_8x16_internal_12bpc).pass2_main
    call m(iflipadst_16x8_internal_10bpc).pass1_rotations
    jmp m(iadst_8x16_internal_12bpc).pass2_end

INV_TXFM_8X16_FN identity, dct,      0, 12
INV_TXFM_8X16_FN identity, adst,     0, 12
INV_TXFM_8X16_FN identity, flipadst, 0, 12
INV_TXFM_8X16_FN identity, identity, 0, 12

cglobal iidentity_8x16_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    jmp m(iidentity_8x16_internal_10bpc).pass1
.pass2:
    call .pass2_main
    packssdw             m0, m8
    packssdw             m1, m9
    packssdw             m2, m10
    packssdw             m3, m11
    packssdw             m4, m12
    packssdw             m5, m13
    packssdw             m6, m14
    packssdw            m13, m7, m15
    vpbroadcastd         m7, [pixel_12bpc_max]
    vpbroadcastd        m12, [pw_16384]
    call m(iidentity_8x16_internal_10bpc).pass2_end
    RET
ALIGN function_align
.pass2_main:
    mova               [cq], m7
    vpbroadcastd         m7, [clip_18b_min]
    REPX     {pmaxsd x, m7}, m0,  m1,  m2,  m3,  m4,  m5,  m6, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    pmaxsd               m7, [cq]
    mova               [cq], m15
    vpbroadcastd        m15, [clip_18b_max]
    REPX    {pminsd x, m15}, m0,  m1,  m2,  m3,  m4,  m5,  m6, m7, \
                             m8,  m9,  m10, m11, m12, m13, m14
    pminsd              m15, [cq]
    mova               [cq], m7
    vpbroadcastd         m7, [pd_5793]
    REPX     {pmulld x, m7}, m0,  m1,  m2,  m3,  m4,  m5,  m6, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    pmulld               m7, [cq]
    mova               [cq], m15
    vpbroadcastd        m15, [pd_1024]
    REPX    {paddd  x, m15}, m0,  m1,  m2,  m3,  m4,  m5,  m6, m7, \
                             m8,  m9,  m10, m11, m12, m13, m14
    paddd               m15, [cq]
    REPX     {psrad  x, 14}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    ret

%macro INV_TXFM_16X4_FN 2-3 10 ; type1, type2, bitdepth
    INV_TXFM_FN          %1, %2, 0, 16x4, %3
%ifidn %1_%2, dct_dct
    vpbroadcastd         m3, [dconly_%3bpc]
%if %3 = 10
.dconly:
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 4
.dconly2:
    add                 r6d, 384
    sar                 r6d, 9
.dconly3:
    imul                r6d, 181
    add                 r6d, 2176
    sar                 r6d, 12
    movd                xm0, r6d
    paddsw              xm0, xm3
    vpbroadcastw         m0, xm0
.dconly_loop:
    paddsw               m1, m0, [dstq+strideq*0]
    paddsw               m2, m0, [dstq+strideq*1]
    psubusw              m1, m3
    psubusw              m2, m3
    mova   [dstq+strideq*0], m1
    mova   [dstq+strideq*1], m2
    lea                dstq, [dstq+strideq*2]
    sub                 r3d, 2
    jg .dconly_loop
    RET
%else
    jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly
%endif
%endif
%endmacro

INV_TXFM_16X4_FN dct, dct
INV_TXFM_16X4_FN dct, identity
INV_TXFM_16X4_FN dct, adst
INV_TXFM_16X4_FN dct, flipadst

cglobal idct_16x4_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [clip_18b_min]
    vpbroadcastd         m9, [clip_18b_max]
.pass1:
    vbroadcasti128       m0, [cq+16* 0]
    vbroadcasti128       m4, [cq+16* 4]
    vbroadcasti128       m1, [cq+16* 2]
    vbroadcasti128       m7, [cq+16* 6]
    vbroadcasti128       m5, [cq+16*10]
    vbroadcasti128       m2, [cq+16* 8]
    vbroadcasti128       m6, [cq+16*12]
    vbroadcasti128       m3, [cq+16*14]
    shufpd               m0, m4, 0x0c ;  0  4
    shufpd               m1, m5, 0x0c ;  2 10
    shufpd               m2, m6, 0x0c ;  8 12
    shufpd               m3, m7, 0x0c ; 14  6
    call .pass1_main
    vbroadcasti128      m10, [cq+16* 1]
    vbroadcasti128       m4, [cq+16* 5]
    vbroadcasti128      m11, [cq+16*15]
    vbroadcasti128       m5, [cq+16*11]
    shufpd              m10, m4, 0x0c ;  1  5
    shufpd              m11, m5, 0x0c ; 15 11
    vbroadcasti128       m5, [cq+16* 9]
    vbroadcasti128       m4, [cq+16*13]
    shufpd               m5, m4, 0x0c ;  9 13
    vbroadcasti128       m6, [cq+16* 7]
    vbroadcasti128       m4, [cq+16* 3]
    shufpd               m6, m4, 0x0c ;  7  3
    call .pass1_main2
    pcmpeqd              m4, m4
    REPX      {psubd x, m4}, m0, m1, m2, m3
    call .pass1_main3
    REPX      {psrad x, 1 }, m0, m1, m2, m3, m4, m5, m6, m7
    jmp                tx2q
.pass2:
    call .transpose_4x16_packed
    lea                  r6, [deint_shuf+128]
    call m(idct_16x4_internal_8bpc).main
.end:
    vpbroadcastd         m4, [pw_2048]
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
    vpbroadcastd         m5, [pixel_10bpc_max]
.end2:
    paddw                m0, [dstq+strideq*0]
    paddw                m1, [dstq+strideq*1]
.end3:
    lea                  r6, [dstq+strideq*2]
    paddw                m2, [r6  +strideq*0]
    paddw                m3, [r6  +strideq*1]
    pxor                 m4, m4
    REPX {mova [cq+32*x], m4}, 0, 1, 2, 3, 4, 5, 6, 7
    REPX     {pmaxsw x, m4}, m0, m1, m2, m3
    REPX     {pminsw x, m5}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [r6  +strideq*0], m2
    mova   [r6  +strideq*1], m3
    RET
ALIGN function_align
.pass1_main:
    vpbroadcastd         m7, [pd_2048]
    call m(idct_8x4_internal_10bpc).main
    psubd                m3, m0, m4   ; idct8 out7 out6
    paddd                m0, m4       ; idct8 out0 out1
    paddd                m1, m2, m5   ; idct8 out3 out2
    psubd                m2, m5       ; idct8 out4 out5
    ret
ALIGN function_align
.pass1_main2:
    ITX_MULSUB_2D        10, 11, 4, 12, 13, 7,  401_1931, 4076_3612, 1
    ITX_MULSUB_2D         5,  6, 4, 12, 13, 7, 3166_3920, 2598_1189, 1
    vbroadcasti128      m12, [pd_3784_m3784]
    psubd                m4, m10, m5
    paddd               m10, m5       ;  t8  t11
    psignd               m4, m12      ;  t9  t10
    psubd                m5, m11, m6
    paddd               m11, m6       ; t15  t12
    psignd               m5, m12      ; t14  t13
    vpbroadcastd         m6, [pd_1567]
    vpbroadcastd        m13, [pd_3784]
    REPX     {pmaxsd x, m8}, m5, m4
    REPX     {pminsd x, m9}, m5, m4
    pmulld              m12, m5
    pmulld               m5, m6
    vbroadcasti128       m6, [pd_1567_m1567]
    pmulld              m13, m4
    pmulld               m4, m6
    REPX     {pmaxsd x, m8}, m10, m11, m0, m1
    REPX     {pminsd x, m9}, m10, m11, m0, m1
    paddd               m12, m7
    paddd                m5, m7
    paddd                m4, m12
    psubd                m5, m13
    psrad                m4, 12       ; t14a t10a
    psrad                m5, 12       ; t9a  t13a
    vpbroadcastd        m12, [pd_2896]
    punpckhqdq           m6, m11, m5
    punpcklqdq          m11, m4
    punpckhqdq           m4, m10, m4
    punpcklqdq          m10, m5
    psubd                m5, m11, m6  ; t12a t13
    paddd               m11, m6       ; t15a t14
    psubd                m6, m10, m4  ; t11a t10
    paddd               m10, m4       ; t8a  t9
    REPX     {pmaxsd x, m8}, m5, m6
    REPX     {pminsd x, m9}, m5, m6
    pmulld               m5, m12
    pmulld               m6, m12
    REPX     {pmaxsd x, m8}, m2, m3, m11, m10
    REPX     {pminsd x, m9}, m2, m3, m11, m10
    ret
ALIGN function_align
.pass1_main3:
    paddd                m5, m7
    psubd                m4, m5, m6
    paddd                m5, m6
    psrad                m4, 12      ; t11 t10a
    psrad                m5, 12      ; t12 t13a
    psubd                m7, m0, m11 ; out15 out14
    paddd                m0, m11     ; out0  out1
    psubd                m6, m1, m5  ; out12 out13
    paddd                m1, m5      ; out3  out2
    psubd                m5, m2, m4  ; out11 out10
    paddd                m2, m4      ; out4  out5
    psubd                m4, m3, m10 ; out8  out9
    paddd                m3, m10     ; out7  out6
    REPX {pshufd x, x, q1032}, m1, m3, m5, m7
    ret
ALIGN function_align
.transpose_4x16_packed:
    vbroadcasti128       m8, [deint_shuf]
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    REPX     {pshufb x, m8}, m0, m2, m4, m6
    punpckhqdq           m1, m0, m2
    punpcklqdq           m0, m2
    punpckhqdq           m2, m4, m6
    punpcklqdq           m4, m6
    vperm2i128           m3, m1, m2, 0x31
    vinserti128          m1, xm2, 1
    vperm2i128           m2, m0, m4, 0x31
    vinserti128          m0, xm4, 1
    ret

INV_TXFM_16X4_FN adst, dct
INV_TXFM_16X4_FN adst, adst
INV_TXFM_16X4_FN adst, flipadst
INV_TXFM_16X4_FN adst, identity

cglobal iadst_16x4_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    call m(iadst_4x16_internal_10bpc).main
    psrad               m11, 11 ; pd_1
    REPX     {paddd x, m11}, m0, m1, m2, m3
    paddd                m4, m5, m11
    paddd                m5, m6, m11
    paddd                m6, m7, m11
    paddd                m7, m8, m11
.pass1_end:
    REPX {pshufd x, x, q1032}, m0, m2, m4, m6
    REPX       {psrad x, 1}, m0, m1, m2, m3, m4, m5, m6, m7
    jmp                tx2q
.pass2:
    call m(idct_16x4_internal_10bpc).transpose_4x16_packed
    lea                  r6, [deint_shuf+128]
    call m(iadst_16x4_internal_8bpc).main
    jmp m(idct_16x4_internal_10bpc).end
ALIGN function_align
.main:
    vpbroadcastd         m6, [pd_1321]
    mova                 m0, [cq+32*0]
    mova                 m1, [cq+32*1]
    vpbroadcastd         m7, [pd_2482]
    mova                 m2, [cq+32*6]
    mova                 m3, [cq+32*7]
    pmulld               m4, m0, m6
    pmulld               m5, m1, m6    ; 1321*in0
    pmulld               m9, m2, m7
    pmulld               m8, m3, m7    ; 2482*in3
    paddd                m4, m9
    paddd                m8, m5        ; 1321*in0 + 2482*in3
    pmulld               m5, m0, m7
    pmulld               m9, m1, m7    ; 2482*in0
    paddd                m0, m2
    paddd                m1, m3        ; in0 + in3
    paddd                m7, m6        ; pd_3803
    pmulld               m2, m7
    pmulld               m3, m7        ; 3803*in3
    psubd                m5, m2
    psubd                m9, m3        ; 2482*in0 - 3803*in3
    mova                 m2, [cq+32*4]
    pmulld              m10, m7, m2
    pmulld               m3, m6, m2
    psubd                m2, m0
    mova                 m0, [cq+32*5]
    pmulld               m7, m0        ; 3803*in2
    pmulld               m6, m0        ; 1321*in2
    psubd                m0, m1        ; in2 - in0 - in3
    vpbroadcastd         m1, [pd_m3344]
    paddd                m4, m10
    paddd                m7, m8        ; t0
    psubd                m5, m3
    psubd                m9, m6        ; t1
    pmulld               m2, m1
    pmulld               m0, m1        ; t2
    pmulld               m3, m1, [cq+32*2]
    pmulld               m1, [cq+32*3] ; -t3
    ret
ALIGN function_align
.main_end:
    ; expects: m6 = rnd
    paddd                m5, m6
    paddd                m9, m6
    paddd               m10, m4, m5
    paddd                m4, m6
    paddd                m8, m7, m6
    paddd                m7, m9
    psubd                m4, m3        ; out0 (unshifted)
    psubd                m5, m3        ; out1 (unshifted)
    paddd                m2, m6        ; out2 (unshifted)
    paddd                m3, m10       ; out3 (unshifted)
    psubd                m8, m1        ; out4 (unshifted)
    psubd                m9, m1        ; out5 (unshifted)
    paddd                m6, m0        ; out6 (unshifted)
    paddd                m7, m1        ; out7 (unshifted)
    ret

INV_TXFM_16X4_FN flipadst, dct
INV_TXFM_16X4_FN flipadst, adst
INV_TXFM_16X4_FN flipadst, flipadst
INV_TXFM_16X4_FN flipadst, identity

cglobal iflipadst_16x4_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    call m(iadst_4x16_internal_10bpc).main
    psrad               m11, 11 ; pd_1
    paddd                m4, m3, m11
    paddd                m3, m5, m11
    paddd                m5, m2, m11
    paddd                m2, m6, m11
    paddd                m6, m1, m11
    paddd                m1, m7, m11
    paddd                m7, m0, m11
    paddd                m0, m8, m11
    jmp m(iadst_16x4_internal_10bpc).pass1_end
.pass2:
    call m(idct_16x4_internal_10bpc).transpose_4x16_packed
    lea                  r6, [deint_shuf+128]
    call m(iadst_16x4_internal_8bpc).main
    vpbroadcastd         m4, [pw_2048]
    pmulhrsw             m5, m3, m4
    pmulhrsw             m6, m2, m4
    pmulhrsw             m2, m1, m4
    pmulhrsw             m3, m0, m4
    paddw                m0, m5, [dstq+strideq*0]
    paddw                m1, m6, [dstq+strideq*1]
    vpbroadcastd         m5, [pixel_10bpc_max]
    jmp m(idct_16x4_internal_10bpc).end3

INV_TXFM_16X4_FN identity, dct
INV_TXFM_16X4_FN identity, adst
INV_TXFM_16X4_FN identity, flipadst
INV_TXFM_16X4_FN identity, identity

cglobal iidentity_16x4_internal_10bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [pd_5793]
    vpermq               m0, [cq+32*0], q3120 ; 0 1
    vpermq               m1, [cq+32*1], q3120 ; 2 3
    vpermq               m2, [cq+32*2], q3120 ; 4 5
    vpermq               m3, [cq+32*3], q3120 ; 6 7
    vpermq               m4, [cq+32*4], q3120 ; 8 9
    vpermq               m5, [cq+32*5], q3120 ; a b
    vpermq               m6, [cq+32*6], q3120 ; c d
    vpermq               m7, [cq+32*7], q3120 ; e f
    vpbroadcastd         m9, [pd_3072]
    REPX     {pmulld x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {paddd  x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad  x, 12}, m0, m1, m2, m3, m4, m5, m6, m7
    jmp                tx2q
.pass2:
    call m(idct_16x4_internal_10bpc).transpose_4x16_packed
    vpbroadcastd         m7, [pw_1697x8]
    pmulhrsw             m4, m7, m0
    pmulhrsw             m5, m7, m1
    pmulhrsw             m6, m7, m2
    pmulhrsw             m7, m3
    paddsw               m0, m4
    paddsw               m1, m5
    paddsw               m2, m6
    paddsw               m3, m7
    jmp m(idct_16x4_internal_10bpc).end

INV_TXFM_16X4_FN dct, dct,      12
INV_TXFM_16X4_FN dct, identity, 12
INV_TXFM_16X4_FN dct, adst,     12
INV_TXFM_16X4_FN dct, flipadst, 12

cglobal idct_16x4_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [clip_20b_min]
    vpbroadcastd         m9, [clip_20b_max]
    jmp m(idct_16x4_internal_10bpc).pass1
.pass2:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    ; deinterleave
    REPX {pshufd x, x, q3120}, m0, m1, m2, m3, m4, m5, m6, m7
    ; transpose
    punpcklqdq           m8, m0, m1
    punpckhqdq           m0, m1
    punpcklqdq           m9, m2, m3
    punpckhqdq           m2, m3
    punpcklqdq          m10, m4, m5
    punpckhqdq           m4, m5
    punpcklqdq          m11, m6, m7
    punpckhqdq           m6, m7
    vperm2i128           m3,  m0,  m2, 0x31   ; out6
    vperm2i128           m1,  m0,  m2, 0x20   ; out2
    vperm2i128           m7,  m4,  m6, 0x31   ; out7
    vperm2i128           m5,  m4,  m6, 0x20   ; out3
    vperm2i128          m13, m10, m11, 0x31   ; out5
    vperm2i128          m12, m10, m11, 0x20   ; out1
    vperm2i128          m11,  m8,  m9, 0x31   ; out4
    vperm2i128          m10,  m8,  m9, 0x20   ; out0
    call m(idct_4x16_internal_10bpc).pass1_main
    pmulld               m0, m6, m10
    pmulld               m2, m6, m11
    pmulld               m4, m6, m12
    pmulld               m6, m13
    vpbroadcastd        m10, [pd_17408]
    call m(idct_4x16_internal_10bpc).pass1_main2
    REPX       {psrad x, 4}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    vpbroadcastd         m5, [pixel_12bpc_max]
    REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    jmp m(idct_16x4_internal_10bpc).end2

INV_TXFM_16X4_FN adst, dct,      12
INV_TXFM_16X4_FN adst, adst,     12
INV_TXFM_16X4_FN adst, flipadst, 12
INV_TXFM_16X4_FN adst, identity, 12

cglobal iadst_16x4_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(iadst_16x4_internal_10bpc).pass1
.pass2:
    call .pass2_main
    REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
    jmp m(idct_16x4_internal_10bpc).end2
ALIGN function_align
.pass2_main:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m6, m7
    pmaxsd               m8, m4, m12
    pmaxsd               m9, m5, m12
    REPX    {pminsd x, m13}, m0, m1, m2, m3
    call m(iadst_8x4_internal_12bpc).transpose_4x8
    mova          [cq+32*0], m0
    mova          [cq+32*2], m1
    mova          [cq+32*4], m2
    mova          [cq+32*6], m3
    pminsd               m0, m8, m13
    pminsd               m1, m9, m13
    pminsd               m2, m6, m13
    pminsd               m3, m7, m13
    call m(iadst_8x4_internal_12bpc).transpose_4x8
    mova          [cq+32*1], m0
    mova          [cq+32*3], m1
    mova          [cq+32*5], m2
    mova          [cq+32*7], m3
    call m(iadst_16x4_internal_10bpc).main
    vpbroadcastd         m6, [pd_2048]
    call m(iadst_16x4_internal_10bpc).main_end
    psrad                m0, m4, 15
    psrad                m1, m5, 15
    psrad                m2, 15
    psrad                m3, 15
    psrad                m4, m8, 15
    psrad                m5, m9, 15
    psrad                m6, 15
    psrad                m7, 15
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    vpbroadcastd         m4, [pw_16384]
    vpbroadcastd         m5, [pixel_12bpc_max]
    ret

INV_TXFM_16X4_FN flipadst, dct,      12
INV_TXFM_16X4_FN flipadst, adst,     12
INV_TXFM_16X4_FN flipadst, flipadst, 12
INV_TXFM_16X4_FN flipadst, identity, 12

cglobal iflipadst_16x4_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(iflipadst_16x4_internal_10bpc).pass1
.pass2:
    call m(iadst_16x4_internal_12bpc).pass2_main
    vpermq               m7, m0, q3120
    vpermq               m6, m1, q3120
    vpermq               m1, m2, q3120
    vpermq               m0, m3, q3120
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    pmulhrsw             m2, m6, m4
    pmulhrsw             m3, m7, m4
    jmp m(idct_16x4_internal_10bpc).end2

INV_TXFM_16X4_FN identity, dct,      12
INV_TXFM_16X4_FN identity, adst,     12
INV_TXFM_16X4_FN identity, flipadst, 12
INV_TXFM_16X4_FN identity, identity, 12

cglobal iidentity_16x4_internal_12bpc, 0, 7, 14, dst, stride, c, eob, tx2
    vpbroadcastd         m8, [pd_1697]
    vpermq               m0, [cq+32*0], q3120 ; 0 1
    vpermq               m1, [cq+32*1], q3120 ; 2 3
    vpermq               m2, [cq+32*2], q3120 ; 4 5
    vpermq               m3, [cq+32*3], q3120 ; 6 7
    vpbroadcastd         m9, [pd_3072]
    pmulld               m4, m8, m0
    pmulld               m5, m8, m1
    pmulld               m6, m8, m2
    pmulld               m7, m8, m3
    vpermq              m10, [cq+32*4], q3120 ; 8 9
    vpermq              m11, [cq+32*5], q3120 ; a b
    vpermq              m12, [cq+32*6], q3120 ; c d
    vpermq              m13, [cq+32*7], q3120 ; e f
    REPX     {paddd  x, m9}, m4, m5, m6, m7
    REPX     {psrad  x, 12}, m4, m5, m6, m7
    paddd                m0, m4
    pmulld               m4, m8, m10
    paddd                m1, m5
    pmulld               m5, m8, m11
    paddd                m2, m6
    pmulld               m6, m8, m12
    paddd                m3, m7
    pmulld               m7, m8, m13
    REPX     {paddd  x, m9}, m4, m5, m6, m7
    REPX     {psrad  x, 12}, m4, m5, m6, m7
    paddd                m4, m10
    paddd                m5, m11
    paddd                m6, m12
    paddd                m7, m13
    jmp                tx2q
.pass2:
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    vpbroadcastd         m8, [pd_5793]
    vpbroadcastd         m9, [pd_2048]
    REPX     {pmulld x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {paddd  x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad  x, 15}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_16x4_internal_10bpc).transpose_4x16_packed
    vpbroadcastd         m4, [pw_16384]
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
    vpbroadcastd         m5, [pixel_12bpc_max]
    jmp m(idct_16x4_internal_10bpc).end2

%macro INV_TXFM_16X8_FN 2-3 10 ; type1, type2, bitdepth
    INV_TXFM_FN          %1, %2, 0, 16x8, %3
%ifidn %1_%2, dct_dct
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_%3bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 8
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly2
%endif
%endmacro

INV_TXFM_16X8_FN dct, dct
INV_TXFM_16X8_FN dct, identity
INV_TXFM_16X8_FN dct, adst
INV_TXFM_16X8_FN dct, flipadst

cglobal idct_16x8_internal_10bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    vpbroadcastd        m14, [pd_2896]
    pmulld               m0, m14, [cq+32* 1]
    pmulld               m1, m14, [cq+32* 3]
    pmulld               m2, m14, [cq+32* 5]
    pmulld               m3, m14, [cq+32* 7]
    pmulld               m4, m14, [cq+32* 9]
    pmulld               m5, m14, [cq+32*11]
    pmulld               m6, m14, [cq+32*13]
    pmulld               m7, m14, [cq+32*15]
    vpbroadcastd        m11, [pd_2048]
    lea                  r6, [rsp+32*4]
    call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    pmulld               m0, m14, [cq+32* 0]
    pmulld               m1, m14, [cq+32* 2]
    pmulld               m2, m14, [cq+32* 4]
    pmulld               m3, m14, [cq+32* 6]
    pmulld               m4, m14, [cq+32* 8]
    pmulld               m5, m14, [cq+32*10]
    pmulld               m6, m14, [cq+32*12]
    pmulld               m7, m14, [cq+32*14]
    call m(idct_8x8_internal_10bpc).main_rect2
    call m(idct_8x16_internal_10bpc).main_evenhalf
    psrld               m11, 11 ; pd_1
    REPX    {paddd  x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    call .pass1_rotations
    REPX       {psrad x, 1}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    call .transpose
    call m(idct_16x8_internal_8bpc).main
    vpbroadcastd        m10, [pw_2048]
.end:
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    pmulhrsw             m2, m10
    pmulhrsw             m3, m10
    call .write_16x4_start
.end2:
    pmulhrsw             m0, m4, m10
    pmulhrsw             m1, m5, m10
    pmulhrsw             m2, m6, m10
    pmulhrsw             m3, m7, m10
    call .write_16x4_zero
    RET
ALIGN function_align
.pass1_rotations:
    mova                m14, [r6-32*4]
    mova                m13, [r6-32*3]
    mova                m12, [r6-32*2]
    mova                m11, [r6-32*1]
    mova                m10, [r6+32*0]
    mova                 m9, [r6+32*1]
    mova                 m8, [r6+32*2]
    psubd               m15, m0, m14       ; out15
    paddd                m0, m14           ; out0
    psubd               m14, m1, m13       ; out14
    paddd                m1, m13           ; out1
    psubd               m13, m2, m12       ; out13
    paddd                m2, m12           ; out2
    psubd               m12, m3, m11       ; out12
    paddd                m3, m11           ; out3
    psubd               m11, m4, m10       ; out11
    paddd                m4, m10           ; out4
    psubd               m10, m5, m9        ; out10
    paddd                m5, m9            ; out5
    psubd                m9, m6, m8        ; out9
    paddd                m6, m8            ; out6
    psubd                m8, m7, [r6+32*3] ; out8
    paddd                m7, [r6+32*3]     ; out7
    ret
ALIGN function_align
.transpose:
    lea                  r6, [deint_shuf+128]
.transpose2:
    packssdw             m0, m8
    packssdw             m1, m9
    packssdw             m2, m10
    packssdw             m3, m11
    packssdw             m4, m12
    packssdw             m5, m13
    packssdw             m6, m14
    packssdw             m7, m15
.transpose3:
    punpckhwd            m8, m0, m1
    punpcklwd            m0, m1
    punpcklwd            m1, m2, m3
    punpckhwd            m2, m3
    punpckhwd            m3, m4, m5
    punpcklwd            m4, m5
    punpckhwd            m5, m6, m7
    punpcklwd            m6, m7
    punpckhdq            m7, m4, m6
    punpckldq            m4, m6
    punpckldq            m6, m8, m2
    punpckhdq            m8, m2
    punpckhdq            m2, m0, m1
    punpckldq            m0, m1
    punpckhdq            m1, m3, m5
    punpckldq            m3, m5
    punpcklqdq           m5, m6, m3
    punpckhqdq           m6, m3
    punpckhqdq           m3, m2, m7
    punpcklqdq           m2, m7
    punpcklqdq           m7, m8, m1
    punpckhqdq           m8, m1
    punpckhqdq           m1, m0, m4
    punpcklqdq           m0, m4
    vperm2i128           m4, m0, m5, 0x31
    vinserti128          m0, xm5, 1
    vperm2i128           m5, m1, m6, 0x31
    vinserti128          m1, xm6, 1
    vperm2i128           m6, m2, m7, 0x31
    vinserti128          m2, xm7, 1
    vperm2i128           m7, m3, m8, 0x31
    vinserti128          m3, xm8, 1
    ret
ALIGN function_align
.write_16x4_start:
    vpbroadcastd         m9, [pixel_10bpc_max]
    lea                  r3, [strideq*3]
    pxor                 m8, m8
.write_16x4_zero:
    REPX {mova [cq+32*x], m8}, 0, 1, 2, 3, 4, 5, 6, 7
    add                  cq, 32*8
.write_16x4:
    paddw                m0, [dstq+strideq*0]
    paddw                m1, [dstq+strideq*1]
    paddw                m2, [dstq+strideq*2]
    paddw                m3, [dstq+r3       ]
    REPX     {pmaxsw x, m8}, m0, m1, m2, m3
    REPX     {pminsw x, m9}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+r3       ], m3
    lea                dstq, [dstq+strideq*4]
    ret

INV_TXFM_16X8_FN adst, dct
INV_TXFM_16X8_FN adst, adst
INV_TXFM_16X8_FN adst, flipadst
INV_TXFM_16X8_FN adst, identity

cglobal iadst_16x8_internal_10bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_18b_min]
    vpbroadcastd        m14, [clip_18b_max]
.pass1:
    lea                  r6, [rsp+32*4]
    call .main
    vpbroadcastd        m14, [pd_3072]
    psrld               m15, 11       ; pd_1
    psubd               m13, m14, m15 ; pd_3071
    call .pass1_rotations
.pass1_end:
    REPX      {psrad x, 1 }, m0,  m1,  m2,  m3,  m12, m13, m14, m15
    REPX      {psrad x, 12}, m4,  m5,  m6,  m7,  m8,  m9,  m10, m11
    jmp                tx2q
.pass2:
    call m(idct_16x8_internal_10bpc).transpose
    call m(iadst_16x8_internal_8bpc).main
    call m(iadst_16x8_internal_8bpc).main_pass2_end
    vpbroadcastd        m10, [pw_2048]
    pxor                m11, m11
    psubw               m11, m10
    pmulhrsw             m0, m10
    pmulhrsw             m1, m11
    pmulhrsw             m2, m10
    pmulhrsw             m3, m11
    call m(idct_16x8_internal_10bpc).write_16x4_start
    pmulhrsw             m0, m4, m10
    pmulhrsw             m1, m5, m11
    pmulhrsw             m2, m6, m10
    pmulhrsw             m3, m7, m11
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    RET
ALIGN function_align
.pass1_rotations:
    paddd                m0, m15
    psubd                m1, m15, m1
    paddd                m2, m15
    psubd                m3, m15, m3
    paddd                m4, m14
    psubd                m5, m13, m5
    paddd                m6, m14
    psubd                m7, m13, m7
    paddd                m8, m14, m9
    psubd                m9, m13, m10
    paddd               m10, m14, m11
    psubd               m11, m13, m12
    paddd               m12, m15, [r6-32*1]
    psubd               m13, m15, [r6-32*2]
    paddd               m14, m15, [r6-32*3]
    psubd               m15,      [r6-32*4]
    ret
ALIGN function_align
.main:
    ; expects: m13 = clip_min   m14 = clip_max
    vpbroadcastd        m15, [pd_2896]
    pmulld               m0, m15, [cq+32* 2]
    pmulld               m1, m15, [cq+32*13]
    pmulld               m2, m15, [cq+32* 6]
    pmulld               m3, m15, [cq+32* 9]
    pmulld               m4, m15, [cq+32*10]
    pmulld               m5, m15, [cq+32* 5]
    pmulld               m6, m15, [cq+32*14]
    pmulld               m7, m15, [cq+32* 1]
    vpbroadcastd        m12, [pd_2048]
    REPX     {paddd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    call .main_part1
    pmulld               m0, m15, [cq+32* 0]
    pmulld               m1, m15, [cq+32*15]
    pmulld               m2, m15, [cq+32* 4]
    pmulld               m3, m15, [cq+32*11]
    pmulld               m4, m15, [cq+32* 8]
    pmulld               m5, m15, [cq+32* 7]
    pmulld               m6, m15, [cq+32*12]
    pmulld               m7, m15, [cq+32* 3]
    REPX     {paddd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
.main_part2:
    ITX_MULSUB_2D         1, 0, 8, 9, 10, 12,  201, 4091
    ITX_MULSUB_2D         3, 2, 8, 9, 10, 12, 1751, 3703
    ITX_MULSUB_2D         5, 4, 8, 9, 10, 12, 3035, 2751
    ITX_MULSUB_2D         7, 6, 8, 9, 10, 12, 3857, 1380
    psubd                m8, m0, m4 ; t8a
    paddd                m0, m4     ; t0a
    psubd                m4, m1, m5 ; t9a
    paddd                m1, m5     ; t1a
    psubd                m5, m2, m6 ; t12a
    paddd                m2, m6     ; t4a
    psubd                m6, m3, m7 ; t13a
    paddd                m7, m3     ; t5a
    REPX    {pmaxsd x, m13}, m8, m4, m5, m6, m0, m1, m2, m7
    REPX    {pminsd x, m14}, m8, m4, m5, m6, m0, m1, m2, m7
    vpbroadcastd        m11, [pd_4017]
    vpbroadcastd        m10, [pd_799]
    ITX_MULSUB_2D         8, 4, 3, 9, _, 12, 10, 11
    ITX_MULSUB_2D         6, 5, 3, 9, _, 12, 11, 10
    psubd                m3, m0, m2 ; t4
    paddd                m0, m2     ; t0
    psubd                m2, m1, m7 ; t5
    paddd                m1, m7     ; t1
    psubd                m7, m4, m6 ; t12a
    paddd                m4, m6     ; t8a
    psubd                m6, m8, m5 ; t13a
    paddd                m5, m8     ; t9a
    REPX    {pmaxsd x, m13}, m3, m2, m7, m6, m0, m1, m4, m5
    REPX    {pminsd x, m14}, m3, m2, m7, m6, m0, m1, m4, m5
    vpbroadcastd        m11, [pd_3784]
    vpbroadcastd        m10, [pd_1567]
    ITX_MULSUB_2D         3, 2, 8, 9, _, 12, 10, 11
    ITX_MULSUB_2D         7, 6, 8, 9, _, 12, 10, 11
    pminsd              m10, m14, [r6-32*4] ;  t2
    pminsd               m8, m14, [r6-32*3] ;  t3
    psubd                m9, m0, m10        ;  t2a
    paddd                m0, m10            ;  out0
    psubd               m10, m1, m8         ;  t3a
    paddd                m1, m8             ; -out15
    pmaxsd               m9, m13
    pmaxsd              m10, m13
    pminsd               m9, m14
    pminsd              m10, m14
    mova          [r6-32*4], m1
    mova                m11, [r6-32*1]      ;  t7a
    mova                 m1, [r6-32*2]      ;  t6a
    psubd                m8, m3, m11        ;  t7
    paddd               m11, m3             ;  out12
    paddd                m3, m2, m1         ; -out3
    psubd                m2, m1             ;  t6
    pmaxsd               m8, m13
    pmaxsd               m2, m13
    pminsd               m8, m14
    pminsd               m2, m14
    mova          [r6-32*1], m11
    mova          [r6-32*3], m2
    mova                 m1, [r6+32*3]      ;  t15
    mova                 m2, [r6+32*2]      ;  t14
    paddd               m12, m7, m1         ; -out13
    psubd                m7, m1             ;  t15a
    psubd               m11, m6, m2         ;  t14a
    paddd                m2, m6             ;  out2
    pmaxsd               m7, m13
    pmaxsd              m11, m13
    pminsd               m7, m14
    pminsd              m11, m14
    mova          [r6-32*2], m12
    pminsd               m1, m14, [r6+32*0] ;  t10a
    pminsd              m12, m14, [r6+32*1] ;  t11a
    psubd                m6, m4, m1         ;  t10
    paddd                m1, m4             ; -out1
    psubd                m4, m5, m12        ;  t11
    paddd                m5, m12            ;  out14
    vpbroadcastd        m12, [pd_1448]
    pmaxsd               m6, m13
    pmaxsd               m4, m13
    pminsd               m6, m14
    pminsd               m4, m14
    REPX    {pmulld x, m12}, m9, m10, m8, m7, m11, m6, m4
    pmulld              m12, [r6-32*3]      ;  t6
    mova          [r6-32*3], m5
    paddd                m5, m11, m7        ; -out5  (unshifted)
    psubd               m11, m7             ;  out10 (unshifted)
    paddd                m7, m9, m10        ; -out7  (unshifted)
    psubd                m9, m10            ;  out8  (unshifted)
    psubd               m10, m6, m4         ; -out9  (unshifted)
    paddd                m6, m4             ;  out6  (unshifted)
    paddd                m4, m12, m8        ;  out4  (unshifted)
    psubd               m12, m8             ; -out11 (unshifted)
    ret
.main_part1:
    ITX_MULSUB_2D         1, 0, 8, 9, 10, 12,  995, 3973
    ITX_MULSUB_2D         3, 2, 8, 9, 10, 12, 2440, 3290
    ITX_MULSUB_2D         5, 4, 8, 9, 10, 12, 3513, 2106
    ITX_MULSUB_2D         7, 6, 8, 9, 10, 12, 4052,  601
    psubd                m8, m0, m4 ; t10a
    paddd                m0, m4     ; t2a
    psubd                m4, m1, m5 ; t11a
    paddd                m1, m5     ; t3a
    psubd                m5, m2, m6 ; t14a
    paddd                m2, m6     ; t6a
    psubd                m6, m3, m7 ; t15a
    paddd                m7, m3     ; t7a
    REPX    {pmaxsd x, m13}, m8, m4, m5, m6, m0, m1, m2, m7
    REPX    {pminsd x, m14}, m8, m4, m5, m6, m0, m1, m2, m7
    vpbroadcastd        m11, [pd_2276]
    vpbroadcastd        m10, [pd_3406]
    ITX_MULSUB_2D         8, 4, 3, 9, _, 12, 10, 11
    ITX_MULSUB_2D         6, 5, 3, 9, _, 12, 11, 10
    psubd                m3, m0, m2 ; t6
    paddd                m0, m2     ; t2
    psubd                m2, m1, m7 ; t7
    paddd                m1, m7     ; t3
    psubd                m7, m4, m6 ; t14a
    paddd                m4, m6     ; t10a
    psubd                m6, m8, m5 ; t15a
    paddd                m5, m8     ; t11a
    REPX    {pmaxsd x, m13}, m3, m2, m7, m6, m0, m1, m4, m5
    REPX    {pminsd x, m14}, m3, m2, m7, m6 ; clip the rest later
    vpbroadcastd        m11, [pd_1567]
    vpbroadcastd        m10, [pd_3784]
    ITX_MULSUB_2D         2, 3, 8, 9, _, 12, 10, 11
    ITX_MULSUB_2D         6, 7, 8, 9, _, 12, 10, 11
    mova          [r6-32*4], m0
    mova          [r6-32*3], m1
    mova          [r6+32*0], m4
    mova          [r6+32*1], m5
    mova          [r6-32*2], m2
    mova          [r6-32*1], m3
    mova          [r6+32*2], m6
    mova          [r6+32*3], m7
    ret

INV_TXFM_16X8_FN flipadst, dct
INV_TXFM_16X8_FN flipadst, adst
INV_TXFM_16X8_FN flipadst, flipadst
INV_TXFM_16X8_FN flipadst, identity

cglobal iflipadst_16x8_internal_10bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_18b_min]
    vpbroadcastd        m14, [clip_18b_max]
.pass1:
    lea                  r6, [rsp+32*4]
    call m(iadst_16x8_internal_10bpc).main
    vpbroadcastd        m14, [pd_3072]
    psrld               m15, 11
    psubd               m13, m14, m15
    call .pass1_rotations
    jmp m(iadst_16x8_internal_10bpc).pass1_end
.pass2:
    call m(idct_16x8_internal_10bpc).transpose
    call m(iadst_16x8_internal_8bpc).main
    call m(iadst_16x8_internal_8bpc).main_pass2_end
    vpbroadcastd        m10, [pw_2048]
    pxor                m11, m11
    psubw               m11, m10
    mova                m12, m0
    pmulhrsw             m0, m7, m11
    mova                 m7, m1
    pmulhrsw             m1, m6, m10
    mova                 m6, m2
    pmulhrsw             m2, m5, m11
    mova                 m5, m3
    pmulhrsw             m3, m4, m10
    call m(idct_16x8_internal_10bpc).write_16x4_start
    pmulhrsw             m0, m5, m11
    pmulhrsw             m1, m6, m10
    pmulhrsw             m2, m7, m11
    pmulhrsw             m3, m12, m10
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    RET
ALIGN function_align
.pass1_rotations:
    psubd                m8, m13, m7
    paddd                m7, m14, m9
    paddd                m9, m14, m6
    psubd                m6, m13, m10
    psubd               m10, m13, m5
    paddd                m5, m14, m11
    paddd               m11, m14, m4
    psubd                m4, m13, m12
    psubd               m12, m15, m3
    paddd                m3, m15, [r6-32*1]
    paddd               m13, m15, m2
    psubd                m2, m15, [r6-32*2]
    psubd               m14, m15, m1
    mova                 m1, m15
    paddd               m15, m0
    psubd                m0, m1, [r6-32*4]
    paddd                m1,     [r6-32*3]
    ret

INV_TXFM_16X8_FN identity, dct
INV_TXFM_16X8_FN identity, adst
INV_TXFM_16X8_FN identity, flipadst
INV_TXFM_16X8_FN identity, identity

cglobal iidentity_16x8_internal_10bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
.pass1:
    vpbroadcastd        m15, [pd_2896]
    pmulld               m0, m15, [cq+32* 0]
    pmulld               m1, m15, [cq+32* 1]
    pmulld               m2, m15, [cq+32* 2]
    pmulld               m3, m15, [cq+32* 3]
    pmulld               m4, m15, [cq+32* 4]
    pmulld               m5, m15, [cq+32* 5]
    pmulld               m6, m15, [cq+32* 6]
    pmulld               m7, m15, [cq+32* 7]
    pmulld               m8, m15, [cq+32* 8]
    pmulld               m9, m15, [cq+32* 9]
    pmulld              m10, m15, [cq+32*10]
    pmulld              m11, m15, [cq+32*11]
    pmulld              m12, m15, [cq+32*12]
    pmulld              m13, m15, [cq+32*13]
    pmulld              m14, m15, [cq+32*14]
    pmulld              m15,      [cq+32*15]
    mova              [rsp], m7
    vpbroadcastd         m7, [pd_2048]
    REPX    {paddd  x, m7 }, m0,  m1,  m2,  m3,  m4,  m5,  m6, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    paddd                m7, [rsp]
    REPX    {psrad  x, 12 }, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    mova              [rsp], m15
    vpbroadcastd        m15, [pd_5793]
    REPX    {pmulld x, m15}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14
    pmulld              m15, [rsp]
    mova              [rsp], m7
    vpbroadcastd         m7, [pd_3072]
    REPX    {paddd  x, m7 }, m0,  m1,  m2,  m3,  m4,  m5,  m6, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    paddd                m7, [rsp]
    REPX    {psrad  x, 12 }, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    call m(idct_16x8_internal_10bpc).transpose
    vpbroadcastd        m10, [pw_4096]
    jmp m(idct_16x8_internal_10bpc).end

INV_TXFM_16X8_FN dct, dct,      12
INV_TXFM_16X8_FN dct, identity, 12
INV_TXFM_16X8_FN dct, adst,     12
INV_TXFM_16X8_FN dct, flipadst, 12

cglobal idct_16x8_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(idct_16x8_internal_10bpc).pass1
.pass2:
    call .pass2_main
    RET
ALIGN function_align
.pass2_main:
    call m(idct_8x16_internal_12bpc).transpose
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vpbroadcastd        m11, [pd_2048]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x8_internal_12bpc).round_shift4
    mova         [cq+32* 8], m0
    mova         [cq+32* 9], m1
    mova         [cq+32*10], m2
    mova         [cq+32*11], m3
    mova         [cq+32*12], m4
    mova         [cq+32*13], m5
    mova         [cq+32*14], m6
    mova         [cq+32*15], m7
    pmaxsd               m0, m12, [cq+32*0]
    pmaxsd               m1, m12, [cq+32*1]
    pmaxsd               m2, m12, [cq+32*2]
    pmaxsd               m3, m12, [cq+32*3]
    pmaxsd               m4, m12, [cq+32*4]
    pmaxsd               m5, m12, [cq+32*5]
    pmaxsd               m6, m12, [cq+32*6]
    pmaxsd               m7, m12, [cq+32*7]
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x8_internal_12bpc).round_shift4
.end:
    packssdw             m0, [cq+32* 8]
    packssdw             m1, [cq+32* 9]
    packssdw             m2, [cq+32*10]
    packssdw             m3, [cq+32*11]
    packssdw             m4, [cq+32*12]
    packssdw             m5, [cq+32*13]
    packssdw             m6, [cq+32*14]
    packssdw             m7, [cq+32*15]
    REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    call .write_16x4_start
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    vpermq               m0, m4, q3120
    vpermq               m1, m5, q3120
    vpermq               m2, m6, q3120
    vpermq               m3, m7, q3120
    jmp m(idct_16x8_internal_10bpc).write_16x4_zero
ALIGN function_align
.write_16x4_start:
    vpbroadcastd         m9, [pixel_12bpc_max]
    lea                  r3, [strideq*3]
    pxor                 m8, m8
    ret

INV_TXFM_16X8_FN adst, dct,      12
INV_TXFM_16X8_FN adst, adst,     12
INV_TXFM_16X8_FN adst, flipadst, 12
INV_TXFM_16X8_FN adst, identity, 12

cglobal iadst_16x8_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_20b_min]
    vpbroadcastd        m14, [clip_20b_max]
    jmp m(iadst_16x8_internal_10bpc).pass1
.pass2:
    call .pass2_main
    call m(idct_16x8_internal_12bpc).end
    RET
ALIGN function_align
.pass2_main:
    call m(idct_8x16_internal_12bpc).transpose
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vpbroadcastd        m11, [pd_2048]
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(iadst_8x8_internal_12bpc).pass2_main2
    mova         [cq+32* 8], m0
    mova         [cq+32* 9], m1
    mova         [cq+32*10], m2
    mova         [cq+32*11], m3
    mova         [cq+32*12], m4
    mova         [cq+32*13], m5
    mova         [cq+32*14], m6
    mova         [cq+32*15], m7
    pmaxsd               m0, m12, [cq+32*0]
    pmaxsd               m1, m12, [cq+32*1]
    pmaxsd               m2, m12, [cq+32*2]
    pmaxsd               m3, m12, [cq+32*3]
    pmaxsd               m4, m12, [cq+32*4]
    pmaxsd               m5, m12, [cq+32*5]
    pmaxsd               m6, m12, [cq+32*6]
    pmaxsd               m7, m12, [cq+32*7]
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(iadst_8x8_internal_12bpc).pass2_main2
    ret

INV_TXFM_16X8_FN flipadst, dct,      12
INV_TXFM_16X8_FN flipadst, adst,     12
INV_TXFM_16X8_FN flipadst, flipadst, 12
INV_TXFM_16X8_FN flipadst, identity, 12

cglobal iflipadst_16x8_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_20b_min]
    vpbroadcastd        m14, [clip_20b_max]
    jmp m(iflipadst_16x8_internal_10bpc).pass1
.pass2:
    call m(iadst_16x8_internal_12bpc).pass2_main
    packssdw            m13, m0, [cq+32* 8]
    packssdw            m12, m1, [cq+32* 9]
    packssdw            m11, m2, [cq+32*10]
    packssdw            m10, m3, [cq+32*11]
    packssdw             m3, m4, [cq+32*12]
    packssdw             m2, m5, [cq+32*13]
    packssdw             m1, m6, [cq+32*14]
    packssdw             m0, m7, [cq+32*15]
    REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    call m(idct_16x8_internal_12bpc).write_16x4_start
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    vpermq               m0, m10, q3120
    vpermq               m1, m11, q3120
    vpermq               m2, m12, q3120
    vpermq               m3, m13, q3120
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    RET

INV_TXFM_16X8_FN identity, dct,      12
INV_TXFM_16X8_FN identity, adst,     12
INV_TXFM_16X8_FN identity, flipadst, 12
INV_TXFM_16X8_FN identity, identity, 12

cglobal iidentity_16x8_internal_12bpc, 0, 7, 16, 32*8, dst, stride, c, eob, tx2
    jmp m(iidentity_16x8_internal_10bpc).pass1
.pass2:
    call m(idct_16x8_internal_10bpc).transpose2
    vpbroadcastd        m10, [pw_4096]
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    pmulhrsw             m2, m10
    pmulhrsw             m3, m10
    call m(idct_16x8_internal_12bpc).write_16x4_start
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    jmp m(idct_16x8_internal_10bpc).end2

%macro INV_TXFM_16X16_FN 2-4 0,10 ; type1, type2, eob_offset, bitdepth
    INV_TXFM_FN          %1, %2, %3, 16x16, %4
%ifidn %1_%2, dct_dct
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_%4bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 16
    add                 r6d, 640
    sar                 r6d, 10
    jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly3
%endif
%endmacro

INV_TXFM_16X16_FN dct, dct
INV_TXFM_16X16_FN dct, identity, 28
INV_TXFM_16X16_FN dct, adst
INV_TXFM_16X16_FN dct, flipadst

cglobal idct_16x16_internal_10bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
.pass1:
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m14, [pd_2896]
    lea                  r6, [rsp+32*4]
    sub                eobd, 36
    jl .fast
    add                  cq, 32
    call .main
    sub                  cq, 32
    mova                m10, [r6-32*4]
    mova                 m9, [r6-32*3]
    mova                 m8, [r6-32*2]
    psubd               m15, m0, m10 ; out15
    paddd                m0, m10     ; out0
    psubd               m10, m1, m9  ; out14
    paddd                m1, m9      ; out1
    psubd                m9, m2, m8  ; out13
    paddd                m2, m8      ; out2
    REPX       {psrad x, 2}, m0, m1, m2
    mova          [r6-32*4], m0
    mova          [r6-32*3], m1
    mova          [r6-32*2], m2
    mova                 m2, [r6-32*1]
    mova                 m1, [r6+32*0]
    mova                 m0, [r6+32*1]
    REPX       {psrad x, 2}, m9, m10, m15
    psubd                m8, m3, m2 ; out12
    paddd                m3, m2     ; out3
    psubd                m2, m4, m1 ; out11
    paddd                m4, m1     ; out4
    psubd                m1, m5, m0 ; out10
    paddd                m5, m0     ; out5
    REPX       {psrad x, 2}, m3, m4, m5
    mova          [r6-32*1], m3
    mova          [r6+32*0], m4
    mova          [r6+32*1], m5
    mova                 m4, [r6+32*2]
    mova                 m3, [r6+32*3]
    REPX       {psrad x, 2}, m1, m2, m8
    psubd                m5, m6, m4 ; out9
    paddd                m6, m4     ; out6
    psubd                m4, m7, m3 ; out8
    paddd                m7, m3     ; out7
    REPX       {psrad x, 2}, m6, m7, m4, m5
    mova          [r6+32*2], m6
    mova          [r6+32*3], m7
    add                  r6, 32*8
    mova          [r6-32*4], m4
    mova          [r6-32*3], m5
    mova          [r6-32*2], m1
    mova          [r6-32*1], m2
    mova          [r6+32*0], m8
    mova          [r6+32*1], m9
    mova          [r6+32*2], m10
    mova          [r6+32*3], m15
.fast:
    add                  r6, 32*8
    call .main
    mova                m14, [r6-32*4]
    mova                m13, [r6-32*3]
    mova                m12, [r6-32*2]
    mova                m11, [r6-32*1]
    mova                m10, [r6+32*0]
    mova                 m9, [r6+32*1]
    mova                 m8, [r6+32*2]
    psubd               m15, m0, m14       ; out15
    paddd                m0, m14           ; out0
    psubd               m14, m1, m13       ; out14
    paddd                m1, m13           ; out1
    psubd               m13, m2, m12       ; out13
    paddd                m2, m12           ; out2
    psubd               m12, m3, m11       ; out12
    paddd                m3, m11           ; out3
    psubd               m11, m4, m10       ; out11
    paddd                m4, m10           ; out4
    psubd               m10, m5, m9        ; out10
    paddd                m5, m9            ; out5
    psubd                m9, m6, m8        ; out9
    paddd                m6, m8            ; out6
    psubd                m8, m7, [r6+32*3] ; out8
    paddd                m7, [r6+32*3]     ; out7
    sub                  r6, 32*8
    REPX       {psrad x, 2}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    call .transpose
    lea                  r6, [pw_5+128]
    mova              [rsp], m15
    call m(idct_16x16_internal_8bpc).main
    mova                 m1, [rsp+32*1]
.end:
    call .write_16x16
    RET
ALIGN function_align
.write_16x16:
    mova [rsp+gprsize+32*0], m8
    mova [rsp+gprsize+32*1], m9
    mova [rsp+gprsize+32*2], m12
    vpbroadcastd        m12, [pw_2048]
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    pmulhrsw             m2, m12
    pmulhrsw             m3, m12
    call m(idct_16x8_internal_10bpc).write_16x4_start
.write_16x16_2:
    pmulhrsw             m0, m12, m4
    pmulhrsw             m1, m12, m5
    pmulhrsw             m2, m12, m6
    pmulhrsw             m3, m12, m7
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    pmulhrsw             m0, m12, [rsp+gprsize+32*0]
    pmulhrsw             m1, m12, [rsp+gprsize+32*1]
    pmulhrsw             m2, m12, m10
    pmulhrsw             m3, m12, m11
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    pmulhrsw             m0, m12, [rsp+gprsize+32*2]
    pmulhrsw             m1, m12, m13
    pmulhrsw             m2, m12, m14
    pmulhrsw             m3, m12, m15
    jmp m(idct_16x8_internal_10bpc).write_16x4_zero
ALIGN function_align
.transpose:
    test               eobd, eobd
    jl .transpose_fast
    packssdw             m8, [r6-32*4]
    packssdw             m9, [r6-32*3]
    packssdw            m10, [r6-32*2]
    packssdw            m11, [r6-32*1]
    packssdw            m12, [r6+32*0]
    packssdw            m13, [r6+32*1]
    packssdw            m14, [r6+32*2]
    packssdw            m15, [r6+32*3]
    sub                  r6, 32*8
    packssdw             m0, [r6-32*4]
    packssdw             m1, [r6-32*3]
    packssdw             m2, [r6-32*2]
    packssdw             m3, [r6-32*1]
    packssdw             m4, [r6+32*0]
    packssdw             m5, [r6+32*1]
    packssdw             m6, [r6+32*2]
    packssdw             m7, [r6+32*3]
    mova               [r6], m8
    punpckhwd            m8, m0, m1
    punpcklwd            m0, m1
    punpcklwd            m1, m2, m3
    punpckhwd            m2, m3
    punpckhwd            m3, m6, m7
    punpcklwd            m6, m7
    punpcklwd            m7, m4, m5
    punpckhwd            m4, m5
    punpckldq            m5, m8, m2
    punpckhdq            m8, m2
    punpckhdq            m2, m0, m1
    punpckldq            m0, m1
    punpckhdq            m1, m7, m6
    punpckldq            m7, m6
    punpckhdq            m6, m4, m3
    punpckldq            m4, m3
    punpckhqdq           m3, m2, m1
    punpcklqdq           m2, m1
    punpckhqdq           m1, m0, m7
    punpcklqdq           m0, m7
    punpcklqdq           m7, m8, m6
    punpckhqdq           m8, m6
    punpckhqdq           m6, m5, m4
    punpcklqdq           m5, m4
    mova                 m4, [r6]
    mova               [r6], m8
    punpcklwd            m8, m4, m9
    punpckhwd            m4, m9
    punpcklwd            m9, m10, m11
    punpckhwd           m10, m11
    punpckhwd           m11, m14, m15
    punpcklwd           m14, m15
    punpckhwd           m15, m12, m13
    punpcklwd           m12, m13
    punpckldq           m13, m4, m10
    punpckhdq            m4, m10
    punpckhdq           m10, m8, m9
    punpckldq            m8, m9
    punpckhdq            m9, m12, m14
    punpckldq           m12, m14
    punpckhdq           m14, m15, m11
    punpckldq           m15, m11
    punpckhqdq          m11, m10, m9
    punpcklqdq          m10, m9
    punpckhqdq           m9, m8, m12
    punpcklqdq           m8, m12
    punpcklqdq          m12, m13, m15
    punpckhqdq          m13, m15
    punpckhqdq          m15, m4, m14
    punpcklqdq          m14, m4, m14
    vperm2i128           m4, m0, m8, 0x31
    vinserti128          m0, xm8, 1
    vinserti128          m8, m5, xm12, 1
    vperm2i128          m12, m5, 0x13
    vperm2i128           m5, m1, m9, 0x31
    vinserti128          m1, xm9, 1
    vinserti128          m9, m6, xm13, 1
    vperm2i128          m13, m6, 0x13
    vperm2i128           m6, m2, m10, 0x31
    vinserti128          m2, xm10, 1
    vinserti128         m10, m7, xm14, 1
    vperm2i128          m14, m7, 0x13
    vperm2i128           m7, m3, m11, 0x31
    vinserti128          m3, xm11, 1
    mova               xm11, [r6]
    vinserti128         m11, xm15, 1
    vinserti128         m15, [r6+16], 0
    ret
.transpose_fast:
    call m(idct_16x8_internal_10bpc).transpose2
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
    ret
ALIGN function_align
.main:
    mova                 m0, [cq+64* 1]
    mova                 m1, [cq+64* 3]
    mova                 m2, [cq+64* 5]
    mova                 m3, [cq+64* 7]
    mova                 m4, [cq+64* 9]
    mova                 m5, [cq+64*11]
    mova                 m6, [cq+64*13]
    mova                 m7, [cq+64*15]
    call m(idct_8x16_internal_10bpc).main_oddhalf
    mova                 m0, [cq+64* 0]
    mova                 m1, [cq+64* 2]
    mova                 m2, [cq+64* 4]
    mova                 m3, [cq+64* 6]
    mova                 m4, [cq+64* 8]
    mova                 m5, [cq+64*10]
    mova                 m6, [cq+64*12]
    mova                 m7, [cq+64*14]
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
    psrld               m10, m11, 10 ; pd_2
    REPX    {paddd  x, m10}, m0, m1, m2, m3, m4, m5, m6, m7
    ret

INV_TXFM_16X16_FN adst, dct
INV_TXFM_16X16_FN adst, adst
INV_TXFM_16X16_FN adst, flipadst

cglobal iadst_16x16_internal_10bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_18b_min]
    vpbroadcastd        m14, [clip_18b_max]
.pass1:
    vpbroadcastd        m15, [pd_2896]
    lea                  r6, [rsp+32*4]
    sub                eobd, 36
    jl .fast
    add                  cq, 32
    call .main
    sub                  cq, 32
    vpbroadcastd         m8, [pd_5120]
    paddd                m4, m8
    paddd                m6, m8
    paddd                m9, m8
    paddd               m11, m8
    vpbroadcastd         m8, [pd_5119]
    psubd                m5, m8, m5
    psubd                m7, m8, m7
    psubd               m10, m8, m10
    psubd               m12, m8, m12
    REPX      {psrad x, 13}, m4, m5, m6, m7, m9, m10, m11, m12
    mova          [r6+32*0], m4
    mova          [r6+32*1], m5
    mova          [r6+32*2], m6
    mova          [r6+32*3], m7
    psrld                m4, m15, 10 ; pd_2
    paddd                m0, m4
    psubd                m1, m4, m1
    paddd                m2, m4
    psubd                m3, m4, m3
    psubd                m7, m4, [r6-32*4]
    paddd                m6, m4, [r6-32*3]
    psubd                m5, m4, [r6-32*2]
    paddd                m4,     [r6-32*1]
    REPX      {psrad x, 2 }, m0, m1, m2, m3, m4, m5, m6, m7
    mova          [r6-32*4], m0
    mova          [r6-32*3], m1
    mova          [r6-32*2], m2
    mova          [r6-32*1], m3
    add                  r6, 32*8
    mova          [r6-32*4], m9
    mova          [r6-32*3], m10
    mova          [r6-32*2], m11
    mova          [r6-32*1], m12
    mova          [r6+32*0], m4
    mova          [r6+32*1], m5
    mova          [r6+32*2], m6
    mova          [r6+32*3], m7
.fast:
    add                  r6, 32*8
    call .main
    vpbroadcastd        m14, [pd_5120]
    vpbroadcastd        m13, [pd_5119]
    psrld               m15, 10 ; pd_2
    paddd                m0, m15
    psubd                m1, m15, m1
    paddd                m2, m15
    psubd                m3, m15, m3
    paddd                m4, m14
    psubd                m5, m13, m5
    paddd                m6, m14
    psubd                m7, m13, m7
    paddd                m8, m14, m9
    psubd                m9, m13, m10
    paddd               m10, m14, m11
    psubd               m11, m13, m12
    paddd               m12, m15, [r6-32*1]
    psubd               m13, m15, [r6-32*2]
    paddd               m14, m15, [r6-32*3]
    psubd               m15,      [r6-32*4]
.pass1_end:
    REPX      {psrad x, 2 }, m0,  m1,  m2,  m3,  m12, m13, m14, m15
    REPX      {psrad x, 13}, m4,  m5,  m6,  m7,  m8,  m9,  m10, m11
    sub                  r6, 32*8
    jmp                tx2q
.pass2:
    call m(idct_16x16_internal_10bpc).transpose
    lea                  r6, [pw_5+128]
    mova              [rsp], m15
    call m(iadst_16x16_internal_8bpc).main
    call m(iadst_16x16_internal_8bpc).main_pass2_end
    mova         [rsp+32*0], m8
    mova         [rsp+32*2], m12
    mova         [rsp+32*3], m13
    vpbroadcastd        m12, [pw_2048]
    pxor                m13, m13
    psubw               m13, m12
    pmulhrsw             m0, m12
    pmulhrsw             m1, m13, [rsp+32*1]
    mova         [rsp+32*1], m9
    pmulhrsw             m2, m12
    pmulhrsw             m3, m13
    call m(idct_16x8_internal_10bpc).write_16x4_start
    pmulhrsw             m0, m12, m4
    pmulhrsw             m1, m13, m5
    pmulhrsw             m2, m12, m6
    pmulhrsw             m3, m13, m7
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    pmulhrsw             m0, m12, [rsp+32*0]
    pmulhrsw             m1, m13, [rsp+32*1]
    pmulhrsw             m2, m12, m10
    pmulhrsw             m3, m13, m11
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    pmulhrsw             m0, m12, [rsp+32*2]
    pmulhrsw             m1, m13, [rsp+32*3]
    pmulhrsw             m2, m12, m14
    pmulhrsw             m3, m13, m15
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    RET
ALIGN function_align
.main:
    mova                 m0, [cq+64* 2]
    mova                 m1, [cq+64*13]
    mova                 m2, [cq+64* 6]
    mova                 m3, [cq+64* 9]
    mova                 m4, [cq+64*10]
    mova                 m5, [cq+64* 5]
    mova                 m6, [cq+64*14]
    mova                 m7, [cq+64* 1]
    vpbroadcastd        m12, [pd_2048]
    call m(iadst_16x8_internal_10bpc).main_part1
    mova                 m0, [cq+64* 0]
    mova                 m1, [cq+64*15]
    mova                 m2, [cq+64* 4]
    mova                 m3, [cq+64*11]
    mova                 m4, [cq+64* 8]
    mova                 m5, [cq+64* 7]
    mova                 m6, [cq+64*12]
    mova                 m7, [cq+64* 3]
    jmp m(iadst_16x8_internal_10bpc).main_part2

INV_TXFM_16X16_FN flipadst, dct
INV_TXFM_16X16_FN flipadst, adst
INV_TXFM_16X16_FN flipadst, flipadst

cglobal iflipadst_16x16_internal_10bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_18b_min]
    vpbroadcastd        m14, [clip_18b_max]
.pass1:
    vpbroadcastd        m15, [pd_2896]
    lea                  r6, [rsp+32*4]
    sub                eobd, 36
    jl .fast
    add                  cq, 32
    call m(iadst_16x16_internal_10bpc).main
    sub                  cq, 32
    vpbroadcastd         m8, [pd_5120]
    paddd               m11, m8
    paddd                m9, m8
    paddd                m6, m8
    paddd                m4, m8
    vpbroadcastd         m8, [pd_5119]
    psubd               m12, m8, m12
    psubd               m10, m8, m10
    psubd                m7, m8, m7
    psubd                m5, m8, m5
    REPX      {psrad x, 13}, m12, m11, m10, m9, m7, m6, m5, m4
    mova          [r6+32*0], m12
    mova          [r6+32*1], m11
    mova          [r6+32*2], m10
    mova          [r6+32*3], m9
    psrld                m9, m15, 10 ; pd_2
    psubd                m3, m9, m3
    paddd                m2, m9
    psubd                m1, m9, m1
    paddd                m0, m9
    psubd               m12, m9, [r6-32*4]
    paddd               m11, m9, [r6-32*3]
    psubd               m10, m9, [r6-32*2]
    paddd                m9,     [r6-32*1]
    REPX      {psrad x, 2 }, m12, m11, m10, m9, m3, m2, m1, m0
    mova          [r6-32*4], m12
    mova          [r6-32*3], m11
    mova          [r6-32*2], m10
    mova          [r6-32*1], m9
    add                  r6, 32*8
    mova          [r6-32*4], m7
    mova          [r6-32*3], m6
    mova          [r6-32*2], m5
    mova          [r6-32*1], m4
    mova          [r6+32*0], m3
    mova          [r6+32*1], m2
    mova          [r6+32*2], m1
    mova          [r6+32*3], m0
.fast:
    add                  r6, 32*8
    call m(iadst_16x16_internal_10bpc).main
    vpbroadcastd        m14, [pd_5120]
    vpbroadcastd        m13, [pd_5119]
    psrld               m15, 10 ; pd_2
    psubd                m8, m13, m7
    paddd                m7, m14, m9
    paddd                m9, m14, m6
    psubd                m6, m13, m10
    psubd               m10, m13, m5
    paddd                m5, m14, m11
    paddd               m11, m14, m4
    psubd                m4, m13, m12
    psubd               m12, m15, m3
    paddd                m3, m15, [r6-32*1]
    paddd               m13, m15, m2
    psubd                m2, m15, [r6-32*2]
    psubd               m14, m15, m1
    mova                 m1, m15
    paddd               m15, m0
    psubd                m0, m1, [r6-32*4]
    paddd                m1,     [r6-32*3]
    jmp m(iadst_16x16_internal_10bpc).pass1_end
.pass2:
    call m(idct_16x16_internal_10bpc).transpose
    lea                  r6, [pw_5+128]
    mova              [rsp], m15
    call m(iadst_16x16_internal_8bpc).main
    call m(iadst_16x16_internal_8bpc).main_pass2_end
    mova         [rsp+32*3], m3
    mova         [rsp+32*2], m2
    mova         [rsp+32*0], m0
    mova                 m2, m13
    mova                 m3, m12
    vpbroadcastd        m12, [pw_2048]
    pxor                m13, m13
    psubw               m13, m12
    pmulhrsw             m0, m13, m15
    pmulhrsw             m1, m12, m14
    pmulhrsw             m2, m13
    pmulhrsw             m3, m12
    mova                m14, m8
    mova                m15, m9
    call m(idct_16x8_internal_10bpc).write_16x4_start
    pmulhrsw             m0, m13, m11
    pmulhrsw             m1, m12, m10
    pmulhrsw             m2, m13, m15
    pmulhrsw             m3, m12, m14
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    pmulhrsw             m0, m13, m7
    pmulhrsw             m1, m12, m6
    pmulhrsw             m2, m13, m5
    pmulhrsw             m3, m12, m4
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    pmulhrsw             m0, m13, [rsp+32*3]
    pmulhrsw             m1, m12, [rsp+32*2]
    pmulhrsw             m2, m13, [rsp+32*1]
    pmulhrsw             m3, m12, [rsp+32*0]
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    RET

INV_TXFM_16X16_FN identity, dct, -92
INV_TXFM_16X16_FN identity, identity

cglobal iidentity_16x16_internal_10bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd        m15, [pd_5793]
    vpbroadcastd         m7, [pd_5120]
    lea                  r6, [rsp+32*4]
    sub                eobd, 36
    jl .fast
    mov                  r3, -32*8*4
.righthalf:
    pmulld               m0, m15, [cq+r3+32*33]
    pmulld               m1, m15, [cq+r3+32*35]
    pmulld               m2, m15, [cq+r3+32*37]
    pmulld               m3, m15, [cq+r3+32*39]
    add                  r6, 32*4
    REPX      {paddd x, m7}, m0, m1, m2, m3
    REPX      {psrad x, 13}, m0, m1, m2, m3
    mova          [r6+32*0], m0
    mova          [r6+32*1], m1
    mova          [r6+32*2], m2
    mova          [r6+32*3], m3
    add                  r3, 32*8
    jl .righthalf
.fast:
    pmulld               m0, m15, [cq+64* 0]
    pmulld               m1, m15, [cq+64* 1]
    pmulld               m2, m15, [cq+64* 2]
    pmulld               m3, m15, [cq+64* 3]
    pmulld               m4, m15, [cq+64* 4]
    pmulld               m5, m15, [cq+64* 5]
    pmulld               m6, m15, [cq+64* 6]
    pmulld               m8, m15, [cq+64* 7]
    mova               [cq], m8
    pmulld               m8, m15, [cq+64* 8]
    pmulld               m9, m15, [cq+64* 9]
    pmulld              m10, m15, [cq+64*10]
    pmulld              m11, m15, [cq+64*11]
    pmulld              m12, m15, [cq+64*12]
    pmulld              m13, m15, [cq+64*13]
    pmulld              m14, m15, [cq+64*14]
    pmulld              m15,      [cq+64*15]
    REPX      {paddd x, m7}, m0,  m1,  m2,  m3,  m4,  m5,  m6, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    paddd                m7, [cq]
    REPX      {psrad x, 13}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    jmp                tx2q
.pass2:
    call m(idct_16x16_internal_10bpc).transpose

    mova          [cq+32*0], m15
    mova          [cq+32*1], m0
    vpbroadcastd        m15, [pw_1697x16]

    REPX  {IDTX16 x, 0, 15},  1,  2,  3,  4,  5,  6,  7, \
                              8,  9, 10, 11, 12, 13, 14
    mova                 m0, [cq+32*1]
    mova          [cq+32*1], m1
    IDTX16                0, 1, 15
    mova                 m1, [cq+32*0]
    pmulhrsw            m15, m1
    paddsw               m1, m1
    paddsw              m15, m1
    mova                 m1, [cq+32*1]
    jmp m(idct_16x16_internal_10bpc).end

INV_TXFM_16X16_FN dct, dct,       0, 12
INV_TXFM_16X16_FN dct, identity, 28, 12
INV_TXFM_16X16_FN dct, adst,      0, 12
INV_TXFM_16X16_FN dct, flipadst,  0, 12

cglobal idct_16x16_internal_12bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    jmp m(idct_16x16_internal_10bpc).pass1
.pass2:
    mova         [cq+32* 8], m8
    mova         [cq+32* 9], m9
    mova         [cq+32*10], m10
    mova         [cq+32*11], m11
    mova         [cq+32*12], m12
    mova         [cq+32*13], m13
    mova         [cq+32*14], m14
    mova         [cq+32*15], m15
    call .pass2_main
    packssdw             m0,  m1
    packssdw             m1,  m2,  m3
    packssdw             m2,  m4,  m5
    packssdw             m3,  m6,  m7
    packssdw             m4,  m8,  m9
    packssdw             m5, m10, m11
    packssdw             m6, m12, m13
    packssdw             m7, m14, m15
    mova          [r6-32*4], m0
    mova          [r6-32*3], m1
    mova          [r6-32*2], m2
    mova          [r6-32*1], m3
    mova          [r6+32*0], m4
    mova          [r6+32*1], m5
    mova          [r6+32*2], m6
    mova          [r6+32*3], m7
    mova                 m0, [cq+32* 8]
    mova                 m1, [cq+32* 9]
    mova                 m2, [cq+32*10]
    mova                 m3, [cq+32*11]
    mova                 m4, [cq+32*12]
    mova                 m5, [cq+32*13]
    mova                 m6, [cq+32*14]
    mova                 m7, [cq+32*15]
    mov                  r5, r6
    add                  r6, 32*16
    call .pass2_main
    jmp m(iadst_16x16_internal_12bpc).end
ALIGN function_align
.write_16x16:
    mova [rsp+gprsize+32*0], m8
    mova [rsp+gprsize+32*1], m9
    mova [rsp+gprsize+32*2], m12
    vpbroadcastd        m12, [pw_16384]
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    pmulhrsw             m2, m12
    pmulhrsw             m3, m12
    call m(idct_16x8_internal_12bpc).write_16x4_start
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    jmp m(idct_16x16_internal_10bpc).write_16x16_2
ALIGN function_align
.pass2_main:
    call m(idct_8x8_internal_12bpc).transpose_8x8
    mova         [cq+32* 0], m0
    mova         [cq+32* 1], m2
    mova         [cq+32* 2], m4
    mova         [cq+32* 3], m6
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    pmaxsd               m0, m12, m1
    pmaxsd               m1, m12, m3
    pmaxsd               m2, m12, m5
    pmaxsd               m3, m12, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3
    test               eobd, eobd
    jge .pass2_slow
    pxor                 m4, m4
    REPX       {mova x, m4}, m5, m6, m7
    jmp .pass2_fast
.pass2_slow:
    sub                  r6, 32*8
    mova                 m8, [r6-32*4]
    mova                 m4, [r6-32*3]
    mova                m10, [r6-32*2]
    mova                 m5, [r6-32*1]
    mova                m12, [r6+32*0]
    mova                 m6, [r6+32*1]
    mova                m14, [r6+32*2]
    mova                 m7, [r6+32*3]
    TRANSPOSE_8X8_DWORD 8, 4, 10, 5, 12, 6, 14, 7, 9, 11, 13, 15
    mova         [cq+32* 4], m8
    mova         [cq+32* 5], m10
    mova         [cq+32* 6], m12
    mova         [cq+32* 7], m14
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    REPX    {pmaxsd x, m12}, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m4, m5, m6, m7
.pass2_fast:
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m14, [pd_2896]
    call m(idct_8x16_internal_10bpc).main_oddhalf
    pmaxsd               m0, m12, [cq+32* 0]
    pmaxsd               m1, m12, [cq+32* 1]
    pmaxsd               m2, m12, [cq+32* 2]
    pmaxsd               m3, m12, [cq+32* 3]
    REPX    {pminsd x, m13}, m0, m1, m2, m3
    test               eobd, eobd
    jge .pass2_slow2
    pxor                 m4, m4
    REPX       {mova x, m4}, m5, m6, m7
    jmp .pass2_fast2
.pass2_slow2:
    pmaxsd               m4, m12, [cq+32* 4]
    pmaxsd               m5, m12, [cq+32* 5]
    pmaxsd               m6, m12, [cq+32* 6]
    pmaxsd               m7, m12, [cq+32* 7]
    REPX    {pminsd x, m13}, m4, m5, m6, m7
.pass2_fast2:
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
    psrad               m11, 8  ; pd_8
    REPX    {paddd  x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_16x8_internal_10bpc).pass1_rotations
    REPX       {psrad x, 4}, m0, m1, m2,  m3,  m4,  m5,  m6,  m7, \
                             m8, m9, m10, m11, m12, m13, m14, m15
    ret

INV_TXFM_16X16_FN adst, dct,      0, 12
INV_TXFM_16X16_FN adst, adst,     0, 12
INV_TXFM_16X16_FN adst, flipadst, 0, 12

cglobal iadst_16x16_internal_12bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_20b_min]
    vpbroadcastd        m14, [clip_20b_max]
    jmp m(iadst_16x16_internal_10bpc).pass1
.pass2:
    call .pass2_part1
    call m(iadst_16x8_internal_10bpc).pass1_rotations
    call .pass2_part2
    call m(iadst_16x8_internal_10bpc).pass1_rotations
.pass2_part3:
    REPX      {psrad x, 4 }, m0, m1, m2, m3, m12, m13, m14, m15
    REPX      {psrad x, 15}, m4, m5, m6, m7, m8,  m9,  m10, m11
.end:
    packssdw            m15, m14
    packssdw            m14, m13, m12
    packssdw            m13, m11, m10
    packssdw            m12,  m9,  m8
    packssdw            m11,  m7,  m6
    packssdw            m10,  m5,  m4
    packssdw             m7,  m3,  m2
    packssdw             m6,  m1,  m0
    vpblendd             m0, m6, [r5-32*4], 0x33
    vpblendd             m1, m6, [r5-32*4], 0xcc
    vpblendd             m2, m7, [r5-32*3], 0x33
    vpblendd             m3, m7, [r5-32*3], 0xcc
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    vpermq               m2, m2, q3120
    vpermq               m3, m3, q2031
    call m(idct_16x8_internal_12bpc).write_16x4_start
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    vpblendd             m0, m10, [r5-32*2], 0x33
    vpblendd             m1, m10, [r5-32*2], 0xcc
    vpblendd             m2, m11, [r5-32*1], 0x33
    vpblendd             m3, m11, [r5-32*1], 0xcc
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    vpermq               m2, m2, q3120
    vpermq               m3, m3, q2031
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    vpblendd             m0, m12, [r5+32*0], 0x33
    vpblendd             m1, m12, [r5+32*0], 0xcc
    vpblendd             m2, m13, [r5+32*1], 0x33
    vpblendd             m3, m13, [r5+32*1], 0xcc
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    vpermq               m2, m2, q3120
    vpermq               m3, m3, q2031
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    vpblendd             m0, m14, [r5+32*2], 0x33
    vpblendd             m1, m14, [r5+32*2], 0xcc
    vpblendd             m2, m15, [r5+32*3], 0x33
    vpblendd             m3, m15, [r5+32*3], 0xcc
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    vpermq               m2, m2, q3120
    vpermq               m3, m3, q2031
    call m(idct_16x8_internal_10bpc).write_16x4_zero
    RET
ALIGN function_align
.pass2_part1:
    mova         [cq+32* 8], m8
    mova         [cq+32* 9], m9
    mova         [cq+32*10], m10
    mova         [cq+32*11], m11
    mova         [cq+32*12], m12
    mova         [cq+32*13], m13
    mova         [cq+32*14], m14
    mova         [cq+32*15], m15
.pass2_main:
    call m(idct_8x8_internal_12bpc).transpose_8x8
    mova         [cq+32* 0], m0
    mova         [cq+32* 1], m3
    mova         [cq+32* 2], m4
    mova         [cq+32* 3], m7
    vpbroadcastd        m13, [clip_18b_min]
    vpbroadcastd        m14, [clip_18b_max]
    pmaxsd               m0, m13, m2
    pmaxsd               m2, m13, m6
    pmaxsd               m5, m13, m5
    pmaxsd               m7, m13, m1
    REPX    {pminsd x, m14}, m0, m2, m5, m7
    test               eobd, eobd
    jge .pass2_slow
    pxor                 m1, m1
    REPX       {mova x, m1}, m3, m4, m6
    jmp .pass2_fast
.pass2_slow:
    sub                  r6, 32*8
    mova                 m8, [r6-32*4]
    mova                 m3, [r6-32*3]
    mova                 m4, [r6-32*2]
    mova                m11, [r6-32*1]
    mova                m12, [r6+32*0]
    mova                 m1, [r6+32*1]
    mova                 m6, [r6+32*2]
    mova                m15, [r6+32*3]
    TRANSPOSE_8X8_DWORD 8, 3, 4, 11, 12, 1, 6, 15, 13, 9, 10, 14
    mova         [cq+32* 4], m8
    mova         [cq+32* 5], m11
    mova         [cq+32* 6], m12
    mova         [cq+32* 7], m15
    vpbroadcastd        m13, [clip_18b_min]
    vpbroadcastd        m14, [clip_18b_max]
    REPX    {pmaxsd x, m13}, m1, m3, m4, m6
    REPX    {pminsd x, m14}, m1, m3, m4, m6
.pass2_fast:
    vpbroadcastd        m12, [pd_2048]
    vpbroadcastd        m15, [pd_2896]
    call m(iadst_16x8_internal_10bpc).main_part1
    pmaxsd               m0, m13, [cq+32* 0] ;  0
    pmaxsd               m7, m13, [cq+32* 1] ;  3
    pmaxsd               m2, m13, [cq+32* 2] ;  4
    pmaxsd               m5, m13, [cq+32* 3] ;  7
    REPX    {pminsd x, m14}, m0, m2, m5, m7
    test               eobd, eobd
    jge .pass2_slow2
    pxor                 m1, m1
    REPX       {mova x, m1}, m3, m4, m6
    jmp .pass2_fast2
.pass2_slow2:
    pmaxsd               m4, m13, [cq+32* 4] ;  8
    pmaxsd               m3, m13, [cq+32* 5] ; 11
    pmaxsd               m6, m13, [cq+32* 6] ; 12
    pmaxsd               m1, m13, [cq+32* 7] ; 15
    REPX    {pminsd x, m14}, m1, m3, m4, m6
.pass2_fast2:
    call m(iadst_16x8_internal_10bpc).main_part2
    vpbroadcastd        m14, [pd_17408]
    psrld               m15, 11              ; pd_1
    psubd               m13, m14, m15        ; pd_17407
    pslld               m15, 3               ; pd_8
    ret
ALIGN function_align
.pass2_part2:
    REPX      {psrad x, 4 }, m0, m1, m2, m3, m12, m13, m14, m15
    REPX      {psrad x, 15}, m4, m5, m6, m7, m8,  m9,  m10, m11
    packssdw             m0,  m1
    packssdw             m1,  m2,  m3
    packssdw             m2,  m4,  m5
    packssdw             m3,  m6,  m7
    packssdw             m4,  m8,  m9
    packssdw             m5, m10, m11
    packssdw             m6, m12, m13
    packssdw             m7, m14, m15
    mova          [r6-32*4], m0
    mova          [r6-32*3], m1
    mova          [r6-32*2], m2
    mova          [r6-32*1], m3
    mova          [r6+32*0], m4
    mova          [r6+32*1], m5
    mova          [r6+32*2], m6
    mova          [r6+32*3], m7
    mova                 m0, [cq+32* 8]
    mova                 m1, [cq+32* 9]
    mova                 m2, [cq+32*10]
    mova                 m3, [cq+32*11]
    mova                 m4, [cq+32*12]
    mova                 m5, [cq+32*13]
    mova                 m6, [cq+32*14]
    mova                 m7, [cq+32*15]
    mov                  r5, r6
    add                  r6, 32*16
    jmp .pass2_main

INV_TXFM_16X16_FN flipadst, dct,      0, 12
INV_TXFM_16X16_FN flipadst, adst,     0, 12
INV_TXFM_16X16_FN flipadst, flipadst, 0, 12

cglobal iflipadst_16x16_internal_12bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd        m13, [clip_20b_min]
    vpbroadcastd        m14, [clip_20b_max]
    jmp m(iflipadst_16x16_internal_10bpc).pass1
.pass2:
    call m(iadst_16x16_internal_12bpc).pass2_part1
    call m(iflipadst_16x8_internal_10bpc).pass1_rotations
    call m(iadst_16x16_internal_12bpc).pass2_part2
    call m(iflipadst_16x8_internal_10bpc).pass1_rotations
    jmp m(iadst_16x16_internal_12bpc).pass2_part3

INV_TXFM_16X16_FN identity, dct,    -92, 12
INV_TXFM_16X16_FN identity, identity, 0, 12

%macro IDTX16_12BPC 1 ; src
    pmulld               m6, m7, m%1
    paddd                m6, m15
    psrad                m6, 12
    paddd                m6, m%1
    psrad               m%1, m6, 1
%endmacro

cglobal iidentity_16x16_internal_12bpc, 0, 7, 16, 32*24, dst, stride, c, eob, tx2
    vpbroadcastd         m7, [pd_1697]
    vpbroadcastd        m15, [pd_5120]
    lea                  r6, [rsp+32*4]
    sub                eobd, 36
    jl .fast
    mov                  r3, -32*8*4
.righthalf:
    mova                m10, [cq+r3+32*33]
    mova                m11, [cq+r3+32*35]
    mova                m12, [cq+r3+32*37]
    mova                m13, [cq+r3+32*39]
    add                  r6, 32*4
    pmulld               m0, m7, m10
    pmulld               m1, m7, m11
    pmulld               m2, m7, m12
    pmulld               m3, m7, m13
    REPX     {paddd x, m15}, m0, m1, m2, m3
    REPX     {psrad x, 12 }, m0, m1, m2, m3
    paddd                m0, m10
    paddd                m1, m11
    paddd                m2, m12
    paddd                m3, m13
    REPX     {psrad x, 1  }, m0, m1, m2, m3
    mova          [r6+32*0], m0
    mova          [r6+32*1], m1
    mova          [r6+32*2], m2
    mova          [r6+32*3], m3
    add                  r3, 32*8
    jl .righthalf
.fast:
    mova                 m0, [cq+64* 0]
    mova                 m1, [cq+64* 1]
    mova                 m2, [cq+64* 2]
    mova                 m3, [cq+64* 3]
    mova                 m4, [cq+64* 4]
    mova                 m5, [cq+64* 5]
    mova                 m8, [cq+64* 6]
    mova                 m9, [cq+64* 7]
    REPX   {IDTX16_12BPC x}, 0, 1, 2, 3, 4, 5, 8, 9
    mova          [cq+64*0], m8
    mova          [cq+64*1], m9
    mova                 m8, [cq+64* 8]
    mova                 m9, [cq+64* 9]
    mova                m10, [cq+64*10]
    mova                m11, [cq+64*11]
    mova                m12, [cq+64*12]
    mova                m13, [cq+64*13]
    mova                m14, [cq+64*14]
    REPX   {IDTX16_12BPC x}, 8, 9, 10, 11, 12, 13, 14
    mova                 m6, [cq+64*15]
    pmulld               m7, m6
    paddd                m7, m15
    psrad                m7, 12
    paddd                m7, m6
    mova                 m6, [cq+64*0]
    psrad               m15, m7, 1
    mova                 m7, [cq+64*1]
    jmp                tx2q
.pass2:
    call m(iidentity_8x16_internal_12bpc).pass2_main
    call m(idct_16x16_internal_10bpc).transpose_fast
    test               eobd, eobd
    jl .pass2_fast
    mova         [cq+32* 8], m0
    mova         [cq+32* 9], m1
    mova         [cq+32*10], m2
    mova         [cq+32*11], m3
    mova         [cq+32*12], m4
    mova         [cq+32*13], m5
    mova         [cq+32*14], m6
    mova         [cq+32*15], m7
    mova                 m8, [r6-32*4]
    mova                 m9, [r6-32*3]
    mova                m10, [r6-32*2]
    mova                m11, [r6-32*1]
    mova                m12, [r6+32*0]
    mova                m13, [r6+32*1]
    mova                m14, [r6+32*2]
    mova                m15, [r6+32*3]
    sub                  r6, 32*8
    mova                 m0, [r6-32*4]
    mova                 m1, [r6-32*3]
    mova                 m2, [r6-32*2]
    mova                 m3, [r6-32*1]
    mova                 m4, [r6+32*0]
    mova                 m5, [r6+32*1]
    mova                 m6, [r6+32*2]
    mova                 m7, [r6+32*3]
    call m(iidentity_8x16_internal_12bpc).pass2_main
    call m(idct_16x8_internal_10bpc).transpose2
    mova                 m8, m0
    mova                 m9, m1
    mova                m10, m2
    mova                m11, m3
    mova                m12, m4
    mova                m13, m5
    mova                m14, m6
    mova                m15, m7
    mova                 m0, [cq+32* 8]
    mova                 m1, [cq+32* 9]
    mova                 m2, [cq+32*10]
    mova                 m3, [cq+32*11]
    mova                 m4, [cq+32*12]
    mova                 m5, [cq+32*13]
    mova                 m6, [cq+32*14]
    mova                 m7, [cq+32*15]
.pass2_fast:
    call m(idct_16x16_internal_12bpc).write_16x16
    RET

%macro IDCT32_END 6-7 1 ; in/out1, out2, tmp[1-3], shift, pack
    mova                m%4, [r6+32*(%1-4)]
    mova                m%2, [r5+32*(3-%1)]
    mova                m%5, [r4+32*(%1-4)]
    psubd               m%3, m%1, m%4 ; idct16 out15 - n
    paddd               m%1, m%4      ; idct16 out0  + n
    pmaxsd              m%1, m12
    pmaxsd              m%3, m12
    pminsd              m%1, m13
    pminsd              m%3, m13
    paddd               m%1, m11
    paddd               m%3, m11
    psubd               m%4, m%1, m%2 ; out31 - n
    paddd               m%1, m%2      ; out0  + n
    paddd               m%2, m%3, m%5 ; out15 - n
    psubd               m%3, m%5      ; out16 + n
    REPX      {psrad x, %6}, m%1, m%3, m%2, m%4
%if %7 & 1
    packssdw            m%1, m%3      ; out0  + n, out16 + n
    packssdw            m%2, m%4      ; out15 - n, out31 - n
%endif
%endmacro

cglobal inv_txfm_add_dct_dct_8x32_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 7, 16, 32*12, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vbroadcasti128      m14, [idct32_shuf]
    mov                  r4, cq
    call .pass1_main
    mova         [rsp+32*0], m2
    mova         [rsp+32*1], m3
    cmp                eobd, 43
    jge .eob43
    pxor                 m4, m4
    REPX       {mova x, m4}, [rsp+32*2], m2, m3, m11
    jmp .pass1_end_fast
.eob43:
    lea                  r6, [rsp+32*8]
    mova          [r6-32*4], m0
    mova          [r6-32*3], m1
    call .pass1_main
    mova         [rsp+32*2], m2
    cmp                eobd, 107
    jge .eob107
    mova                m11, m3
    mova                 m2, m0
    mova                 m3, m1
    mova                 m0, [r6-32*4]
    mova                 m1, [r6-32*3]
    pxor                 m4, m4
.pass1_end_fast:
    vpbroadcastd        m10, [pw_2048]
    lea                  r6, [deint_shuf+128]
    REPX       {mova x, m4}, m5, m6, m7
    call m(inv_txfm_add_dct_dct_8x32_8bpc).main_fast
    jmp .end
.eob107:
    mova         [rsp+32*3], m3
    mova          [r6-32*2], m0
    mova          [r6-32*1], m1
    call .pass1_main
    cmp                eobd, 171
    jge .eob171
    pshufd              m12, m2, q1032
    pshufd              m13, m3, q1032
    mova                 m4, m0
    mova                 m5, m1
    pxor                 m6, m6
    REPX       {mova x, m6}, m7, m14, m15
    jmp .pass1_end
.eob171:
    mova          [r6+32*0], m0
    mova          [r6+32*1], m1
    mova          [r6+32*2], m2
    mova          [r6+32*3], m3
    call .pass1_main
    pshufd              m12, [r6+32*2], q1032 ; out19 out17
    pshufd              m13, [r6+32*3], q1032 ; out23 out21
    mova                 m4, [r6+32*0]        ; out16 out18
    mova                 m5, [r6+32*1]        ; out20 out22
    pshufd              m14, m2, q1032        ; out27 out25
    pshufd              m15, m3, q1032        ; out31 out29
    mova                 m6, m0               ; out24 out26
    mova                 m7, m1               ; out28 out30
.pass1_end:
    mova                 m0, [r6-32*4]        ; out0  out2
    mova                 m1, [r6-32*3]        ; out4  out6
    mova                 m2, [r6-32*2]        ; out8  out10
    mova                 m3, [r6-32*1]        ; out12 out14
    lea                  r6, [deint_shuf+128]
    mova                m11, [rsp+32*3]       ; out13 out15
    vpbroadcastd        m10, [pw_2048]
    call m(inv_txfm_add_dct_dct_8x32_8bpc).main
.end: ; [rsp+0*32] = m12
    vpbroadcastd        m12, [pw_2048]
    mov                  cq, r4
    mova         [rsp+32*1], m8
    mova         [rsp+32*2], m9
    mova         [rsp+32*3], m10
    mova         [rsp+32*4], m11
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4_start
    vpermq               m0, m2, q3120
    vpermq               m1, m3, q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m4, q3120
    vpermq               m1, m5, q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m6, q3120
    vpermq               m1, m7, q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, [rsp+32*1], q3120
    vpermq               m1, [rsp+32*2], q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, [rsp+32*3], q3120
    vpermq               m1, [rsp+32*4], q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, [rsp+32*0], q3120
    vpermq               m1, m13, q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m14, q3120
    vpermq               m1, m15, q2031
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    call m(idct_8x8_internal_10bpc).write_8x4
    RET
.dconly:
    imul                r6d, [cq], 181
    vpbroadcastd         m2, [dconly_10bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 32
    add                 r6d, 640
    sar                 r6d, 10
    jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly3
ALIGN function_align
.pass1_main_part1:
    mova                 m0, [cq+128*0]
    mova                 m1, [cq+128*1]
    mova                 m2, [cq+128*2]
    mova                 m3, [cq+128*3]
    mova                 m4, [cq+128*4]
    mova                 m5, [cq+128*5]
    mova                 m6, [cq+128*6]
    mova                 m7, [cq+128*7]
    call m(idct_8x8_internal_10bpc).main
    psrld                m1, m11, 10 ; pd_2
    REPX      {paddd x, m1}, m0, m6, m5, m3
    paddd                m1, m6, m7  ; out1
    psubd                m6, m7      ; out6
    psubd                m7, m0, m9  ; out7
    paddd                m0, m9      ; out0
    paddd                m2, m5, m4  ; out2
    psubd                m5, m4      ; out5
    psubd                m4, m3, m8  ; out4
    paddd                m3, m8      ; out3
    REPX      {psrad x, 2 }, m0, m1, m2, m3, m4, m5, m6, m7
    ret
ALIGN function_align
.pass1_main:
    call .pass1_main_part1
    add                  cq, 32
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    pshufb               m0, m14
    pshufb               m2, m14
    pshufb               m4, m14
    pshufb               m6, m14
    punpckhdq            m3, m0, m2
    punpckldq            m0, m2
    punpckldq            m2, m4, m6
    punpckhdq            m4, m6
    vperm2i128           m1, m0, m2, 0x31 ; 4 6
    vinserti128          m0, xm2, 1       ; 0 2
    vinserti128          m2, m3, xm4, 1   ; 1 3
    vperm2i128           m3, m4, 0x31     ; 5 7
    ret
.main_oddhalf_part1_fast_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3
    REPX     {psrad x, 12 }, m0, m1, m2, m3
.main_oddhalf_part1_fast: ; lower half zero
    vpbroadcastd         m7, [pd_4091]
    vpbroadcastd         m8, [pd_201]
    vpbroadcastd         m6, [pd_m1380]
    vpbroadcastd         m9, [pd_3857]
    vpbroadcastd         m5, [pd_3703]
    vpbroadcastd        m10, [pd_1751]
    vpbroadcastd         m4, [pd_m2751]
    vpbroadcastd        m15, [pd_3035]
    pmulld               m7, m0
    pmulld               m0, m8
    pmulld               m6, m1
    pmulld               m1, m9
    pmulld               m5, m2
    pmulld               m2, m10
    pmulld               m4, m3
    pmulld               m3, m15
    jmp .main_oddhalf_part1_fast2
.main_oddhalf_part1_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
.main_oddhalf_part1: ; in1, in7, in9, in15, in17, in23, in25, in31
    ITX_MULSUB_2D         0, 7, 8, 9, 10, _,  201, 4091 ; t16a, t31a
    ITX_MULSUB_2D         6, 1, 8, 9, 10, _, 3857, 1380 ; t19a, t28a
    ITX_MULSUB_2D         2, 5, 8, 9, 10, _, 1751, 3703 ; t18a, t29a
    ITX_MULSUB_2D         4, 3, 8, 9, 10, _, 3035, 2751 ; t17a, t30a
.main_oddhalf_part1_fast2:
    REPX     {paddd x, m11}, m0, m7, m6, m1, m2, m5, m4, m3
    REPX     {psrad x, 12 }, m0, m4, m6, m2, m1, m5, m7, m3
    psubd                m8, m0, m4 ; t17
    paddd                m0, m4     ; t16
    psubd                m4, m6, m2 ; t18
    paddd                m6, m2     ; t19
    psubd                m2, m1, m5 ; t29
    paddd                m1, m5     ; t28
    psubd                m5, m7, m3 ; t30
    paddd                m7, m3     ; t31
    REPX    {pmaxsd x, m12}, m8, m5, m4, m2, m0, m6, m1, m7
    REPX    {pminsd x, m13}, m8, m5, m4, m2, m0, m6, m1, m7
    vpbroadcastd        m15, [pd_4017]
    vpbroadcastd        m10, [pd_799]
    ITX_MULSUB_2D         5, 8, 3, 9, _, 11, 10, 15    ; t17a, t30a
    ITX_MULSUB_2D         2, 4, 3, 9, _, 11, 10, 15, 2 ; t29a, t18a
    psubd                m3, m0, m6 ; t19a
    paddd                m0, m6     ; t16a
    psubd                m6, m7, m1 ; t28a
    paddd                m7, m1     ; t31a
    psubd                m1, m5, m4 ; t18
    paddd                m5, m4     ; t17
    psubd                m4, m8, m2 ; t29
    paddd                m8, m2     ; t30
    REPX    {pmaxsd x, m12}, m3, m6, m1, m4, m0, m7, m5, m8
    REPX    {pminsd x, m13}, m3, m6, m1, m4, m0, m7, m5, m8
    vpbroadcastd        m15, [pd_3784]
    vpbroadcastd        m10, [pd_1567]
    ITX_MULSUB_2D         4, 1, 2, 9, _, 11, 10, 15 ; t18a, t29a
    ITX_MULSUB_2D         6, 3, 2, 9, _, 11, 10, 15 ; t19,  t28
    mova          [r6-32*4], m0
    mova          [r6-32*3], m5
    mova          [r6-32*2], m4
    mova          [r6-32*1], m6
    mova          [r6+32*0], m3
    mova          [r6+32*1], m1
    mova          [r6+32*2], m8
    mova          [r6+32*3], m7
    ret
.main_oddhalf_part2_fast_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3
    REPX     {psrad x, 12 }, m0, m1, m2, m3
.main_oddhalf_part2_fast: ; lower half zero
    vpbroadcastd         m7, [pd_m601]
    vpbroadcastd         m8, [pd_4052]
    vpbroadcastd         m6, [pd_3973]
    vpbroadcastd         m9, [pd_995]
    vpbroadcastd         m5, [pd_m2106]
    vpbroadcastd        m10, [pd_3513]
    vpbroadcastd         m4, [pd_3290]
    vpbroadcastd        m15, [pd_2440]
    pmulld               m7, m0
    pmulld               m0, m8
    pmulld               m6, m1
    pmulld               m1, m9
    pmulld               m5, m2
    pmulld               m2, m10
    pmulld               m4, m3
    pmulld               m3, m15
    jmp .main_oddhalf_part2_fast2
.main_oddhalf_part2_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
.main_oddhalf_part2: ; in3, in5, in11, in13, in19, in21, in27, in29
    ITX_MULSUB_2D         7, 0, 8, 9, 10, _, 4052,  601 ; t23a, t24a
    ITX_MULSUB_2D         1, 6, 8, 9, 10, _,  995, 3973 ; t20a, t27a
    ITX_MULSUB_2D         5, 2, 8, 9, 10, _, 3513, 2106 ; t21a, t26a
    ITX_MULSUB_2D         3, 4, 8, 9, 10, _, 2440, 3290 ; t22a, t25a
.main_oddhalf_part2_fast2:
    REPX     {paddd x, m11}, m0, m7, m6, m1, m2, m5, m4, m3
    REPX     {psrad x, 12 }, m0, m4, m6, m2, m1, m5, m7, m3
    psubd                m8, m0, m4 ; t25
    paddd                m0, m4     ; t24
    psubd                m4, m6, m2 ; t26
    paddd                m6, m2     ; t27
    psubd                m2, m1, m5 ; t21
    paddd                m1, m5     ; t20
    psubd                m5, m7, m3 ; t22
    paddd                m7, m3     ; t23
    REPX    {pmaxsd x, m12}, m8, m5, m4, m2, m0, m6, m1, m7
    REPX    {pminsd x, m13}, m8, m5, m4, m2, m0, m6, m1, m7
    vpbroadcastd        m15, [pd_2276]
    vpbroadcastd        m10, [pd_3406]
    ITX_MULSUB_2D         4, 2, 3, 9, _, 11, 10, 15    ; t21a, t26a
    ITX_MULSUB_2D         8, 5, 3, 9, _, 11, 10, 15, 2 ; t25a, t22a
    psubd                m3, m0, m6 ; t27a
    paddd                m0, m6     ; t24a
    psubd                m6, m7, m1 ; t20a
    paddd                m7, m1     ; t23a
    psubd                m1, m5, m4 ; t21
    paddd                m5, m4     ; t22
    psubd                m4, m8, m2 ; t26
    paddd                m8, m2     ; t25
    REPX    {pmaxsd x, m12}, m3, m6, m1, m4, m0, m7, m5, m8
    REPX    {pminsd x, m13}, m3, m6, m1, m4, m0, m7, m5, m8
    vpbroadcastd        m15, [pd_3784]
    vpbroadcastd        m10, [pd_1567]
    ITX_MULSUB_2D         4, 1, 2, 9, _, 11, 10, 15, 2 ; t26a, t21a
    ITX_MULSUB_2D         3, 6, 2, 9, _, 11, 10, 15, 2 ; t27,  t20
    mova                 m9, [r6-32*4] ; t16a
    mova                m10, [r6-32*3] ; t17
    psubd                m2, m9, m7    ; t23
    paddd                m9, m7        ; t16
    psubd                m7, m10, m5   ; t22a
    paddd               m10, m5        ; t17a
    REPX    {pmaxsd x, m12}, m9, m10, m2, m7
    REPX    {pminsd x, m13}, m9, m10, m2, m7
    mova          [r6-32*4], m9
    mova          [r6-32*3], m10
    mova                 m9, [r6-32*2] ; t18a
    mova                m10, [r6-32*1] ; t19
    psubd                m5, m9, m1    ; t21
    paddd                m9, m1        ; t18
    psubd                m1, m10, m6   ; t20a
    paddd               m10, m6        ; t19a
    REPX    {pmaxsd x, m12}, m9, m10, m5, m1
    REPX    {pminsd x, m13}, m9, m10, m5, m1
    mova          [r6-32*2], m9
    mova          [r6-32*1], m10
    mova                 m9, [r6+32*0] ; t28
    mova                m10, [r6+32*1] ; t29a
    psubd                m6, m9, m3    ; t27a
    paddd                m9, m3        ; t28a
    psubd                m3, m10, m4   ; t26
    paddd               m10, m4        ; t29
    REPX    {pmaxsd x, m12}, m9, m10, m6, m3
    REPX    {pminsd x, m13}, m9, m10, m6, m3
    REPX    {pmulld x, m14}, m6, m3, m1, m5
    paddd                m6, m11
    paddd                m3, m11
    psubd                m4, m6, m1    ; t20
    paddd                m6, m1        ; t27
    psubd                m1, m3, m5    ; t21a
    paddd                m3, m5        ; t26a
    REPX    {psrad  x, 12 }, m4, m1, m3, m6
    mova          [r6+32*0], m4
    mova          [r6+32*1], m1
    mova                 m4, [r6+32*2] ; t30
    mova                 m1, [r6+32*3] ; t31a
    psubd                m5, m4, m8    ; t25a
    paddd                m4, m8        ; t30a
    psubd                m8, m1, m0    ; t24
    paddd                m1, m0        ; t31
    REPX    {pmaxsd x, m12}, m8, m5, m4, m1
    REPX    {pminsd x, m13}, m8, m5, m4, m1
    REPX    {pmulld x, m14}, m5, m8, m7, m2
    paddd                m5, m11
    paddd                m8, m11
    psubd                m0, m5, m7    ; t22
    paddd                m5, m7        ; t25
    psubd                m7, m8, m2    ; t23a
    paddd                m2, m8        ; t24a
    REPX    {psrad  x, 12 }, m0, m7, m2, m5
    mova          [r6+32*2], m0
    mova          [r6+32*3], m7
    mov                  r4, r6
    add                  r6, 32*8
    mova          [r6-32*4], m2
    mova          [r6-32*3], m5
    mova          [r6-32*2], m3
    mova          [r6-32*1], m6
    mova          [r6+32*0], m9
    mova          [r6+32*1], m10
    mova          [r6+32*2], m4
    mova          [r6+32*3], m1
    mov                  r5, r6
    add                  r6, 32*8
    ret
ALIGN function_align
.main_end:
    psrld               m11, 10 ; pd_2
    IDCT32_END            0, 15, 8, 9, 10, 2
    IDCT32_END            1, 14, 8, 9, 10, 2
    punpckhwd            m8, m0, m1   ; 16 17
    punpcklwd            m0, m1       ;  0  1
    punpcklwd            m1, m14, m15 ; 14 15
    punpckhwd           m14, m15      ; 30 31
    mova          [r5+32*3], m8
    mova          [r5+32*2], m14
    IDCT32_END            2, 15, 8, 9, 10, 2
    IDCT32_END            3, 14, 8, 9, 10, 2
    punpckhwd            m8, m2, m3   ; 18 19
    punpcklwd            m2, m3       ;  2  3
    punpcklwd            m3, m14, m15 ; 12 13
    punpckhwd           m14, m15      ; 28 29
    mova          [r5+32*1], m8
    mova          [r5+32*0], m14
    IDCT32_END            4, 15, 8, 9, 10, 2
    IDCT32_END            5, 14, 8, 9, 10, 2
    punpckhwd            m8, m4, m5   ; 20 21
    punpcklwd            m4, m5       ;  4  5
    punpcklwd            m5, m14, m15 ; 10 11
    punpckhwd           m14, m15      ; 26 27
    mova          [r5-32*1], m8
    mova          [r5-32*2], m14
    IDCT32_END            6, 15, 8, 9, 10, 2
    IDCT32_END            7, 14, 8, 9, 10, 2
    punpckhwd            m8, m6, m7   ; 22 23
    punpcklwd            m6, m7       ;  6  7
    punpcklwd            m7, m14, m15 ;  8  9
    punpckhwd           m14, m15      ; 24 25
    mova          [r5-32*3], m8
    mova          [r5-32*4], m14
.transpose:
    punpckhdq           m15, m3, m1
    punpckldq            m3, m1
    punpckhdq            m1, m4, m6
    punpckldq            m4, m6
    punpckhdq            m6, m0, m2
    punpckldq            m0, m2
    punpckhdq            m2, m7, m5
    punpckldq            m7, m5
    punpcklqdq           m5, m2, m15
    punpckhqdq           m2, m15
    punpckhqdq          m15, m7, m3
    punpcklqdq           m7, m3
    punpckhqdq           m3, m6, m1
    punpcklqdq           m6, m1
    punpckhqdq           m1, m0, m4
    punpcklqdq           m0, m4
    vperm2i128           m4, m0, m7, 0x31
    vinserti128          m0, xm7, 1
    vperm2i128           m7, m3, m2, 0x31
    vinserti128          m3, xm2, 1
    vinserti128          m2, m6, xm5, 1
    vperm2i128           m6, m5, 0x31
    vperm2i128           m5, m1, m15, 0x31
    vinserti128          m1, xm15, 1
    ret

cglobal inv_txfm_add_identity_identity_8x32_10bpc, 4, 7, 8, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_10bpc_max]
.pass1:
    vpbroadcastd         m5, [pw_5]
    pxor                 m6, m6
    mov                 r6d, eobd
    add                eobb, 21
    cmovc              eobd, r6d ; 43, 107, 171 -> 64, 128, 192
    lea                  r6, [strideq*3]
    lea                  r5, [strideq*5]
    lea                  r4, [strideq+r6*2] ; strideq*7
.loop:
    mova                 m0, [cq+128*0]
    packssdw             m0, [cq+128*1]
    mova                 m1, [cq+128*2]
    packssdw             m1, [cq+128*3]
    mova                 m2, [cq+128*4]
    packssdw             m2, [cq+128*5]
    mova                 m3, [cq+128*6]
    packssdw             m3, [cq+128*7]
    REPX     {paddsw x, m5}, m0, m1, m2, m3
    REPX     {psraw  x, 3 }, m0, m1, m2, m3
    call .main_zero
    add                  cq, 32
    lea                dstq, [dstq+strideq*8]
    sub                eobd, 64
    jge .loop
    RET
ALIGN function_align
.main_zero:
    REPX {mova [cq+128*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
.main:
    punpckhwd            m4, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m2, m3
    punpcklwd            m2, m3
    punpckhwd            m3, m0, m4
    punpcklwd            m0, m4
    punpckhwd            m4, m2, m1
    punpcklwd            m2, m1
    punpckhqdq           m1, m0, m2
    punpcklqdq           m0, m2
    punpcklqdq           m2, m3, m4
    punpckhqdq           m3, m4
    mova                xm4, [dstq+strideq*0]
    vinserti128          m4, [dstq+strideq*4], 1
    paddw                m0, m4
    mova                xm4, [dstq+strideq*1]
    vinserti128          m4, [dstq+r5       ], 1
    paddw                m1, m4
    mova                xm4, [dstq+strideq*2]
    vinserti128          m4, [dstq+r6*2     ], 1
    paddw                m2, m4
    mova                xm4, [dstq+r6       ]
    vinserti128          m4, [dstq+r4       ], 1
    paddw                m3, m4
    REPX     {pmaxsw x, m6}, m0, m1, m2, m3
    REPX     {pminsw x, m7}, m0, m1, m2, m3
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*4], m0, 1
    mova         [dstq+strideq*1], xm1
    vextracti128 [dstq+r5       ], m1, 1
    mova         [dstq+strideq*2], xm2
    vextracti128 [dstq+r6*2     ], m2, 1
    mova         [dstq+r6       ], xm3
    vextracti128 [dstq+r4       ], m3, 1
    ret

cglobal inv_txfm_add_dct_dct_8x32_12bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 7, 16, 32*24, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    mov                  r4, cq
    lea                  r6, [rsp+32*4]
    call .pass1_main
    cmp                eobd, 43
    jge .eob43
    jmp .pass2_fast
.eob43:
    call .pass1_main
    cmp                eobd, 107
    jge .eob107
.pass2_fast:
    mov                  cq, r4
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    pmaxsd               m0, m12, [cq+128*1+ 0]
    pmaxsd               m1, m12, [cq+128*7+ 0]
    pmaxsd               m2, m12, [cq+128*1+32]
    pmaxsd               m3, m12, [cq+128*7+32]
    REPX    {pminsd x, m13}, m0, m1, m2, m3
    vpbroadcastd        m14, [pd_2896]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast
    pmaxsd               m0, m12, [cq+128*3+ 0]
    pmaxsd               m1, m12, [cq+128*5+ 0]
    pmaxsd               m2, m12, [cq+128*3+32]
    pmaxsd               m3, m12, [cq+128*5+32]
    REPX    {pminsd x, m13}, m0, m1, m2, m3
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast
    pmaxsd               m0, m12, [cq+128*2+ 0]
    pmaxsd               m1, m12, [cq+128*6+ 0]
    pmaxsd               m2, m12, [cq+128*2+32]
    pmaxsd               m3, m12, [cq+128*6+32]
    REPX    {pminsd x, m13}, m0, m1, m2, m3
    call m(idct_8x16_internal_10bpc).main_oddhalf_fast
    pmaxsd               m0, m12, [cq+128*0+ 0]
    pmaxsd               m1, m12, [cq+128*4+ 0]
    pmaxsd               m2, m12, [cq+128*0+32]
    pmaxsd               m3, m12, [cq+128*4+32]
    REPX    {pminsd x, m13}, m0, m1, m2, m3
    pxor                 m4, m4
    REPX       {mova x, m4}, m5, m6, m7
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
    jmp .pass2_end
.eob107:
    call .pass1_main
    cmp                eobd, 171
    jge .eob171
    jmp .pass2
.eob171:
    call .pass1_main
.pass2:
    mov                  cq, r4
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    pmaxsd               m0, m12, [cq+128*1+ 0]
    pmaxsd               m1, m12, [cq+128*7+ 0]
    pmaxsd               m2, m12, [cq+128*1+32]
    pmaxsd               m3, m12, [cq+128*7+32]
    pmaxsd               m4, m12, [cq+128*1+64]
    pmaxsd               m5, m12, [cq+128*7+64]
    pmaxsd               m6, m12, [cq+128*1+96]
    pmaxsd               m7, m12, [cq+128*7+96]
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    vpbroadcastd        m14, [pd_2896]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1
    pmaxsd               m0, m12, [cq+128*3+ 0]
    pmaxsd               m1, m12, [cq+128*5+ 0]
    pmaxsd               m2, m12, [cq+128*3+32]
    pmaxsd               m3, m12, [cq+128*5+32]
    pmaxsd               m4, m12, [cq+128*3+64]
    pmaxsd               m5, m12, [cq+128*5+64]
    pmaxsd               m6, m12, [cq+128*3+96]
    pmaxsd               m7, m12, [cq+128*5+96]
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2
    pmaxsd               m0, m12, [cq+128*2+ 0]
    pmaxsd               m1, m12, [cq+128*6+ 0]
    pmaxsd               m2, m12, [cq+128*2+32]
    pmaxsd               m3, m12, [cq+128*6+32]
    pmaxsd               m4, m12, [cq+128*2+64]
    pmaxsd               m5, m12, [cq+128*6+64]
    pmaxsd               m6, m12, [cq+128*2+96]
    pmaxsd               m7, m12, [cq+128*6+96]
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_8x16_internal_10bpc).main_oddhalf
    pmaxsd               m0, m12, [cq+128*0+ 0]
    pmaxsd               m1, m12, [cq+128*4+ 0]
    pmaxsd               m2, m12, [cq+128*0+32]
    pmaxsd               m3, m12, [cq+128*4+32]
    pmaxsd               m4, m12, [cq+128*0+64]
    pmaxsd               m5, m12, [cq+128*4+64]
    pmaxsd               m6, m12, [cq+128*0+96]
    pmaxsd               m7, m12, [cq+128*4+96]
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
.pass2_end:
    psrld               m11, 8 ; pd_8
    IDCT32_END            0, 15, 8, 9, 10, 4
    IDCT32_END            1, 14, 8, 9, 10, 4
    punpckhqdq           m8, m0, m1   ; 16 17 (interleaved)
    punpcklqdq           m0, m1       ;  0  1 (interleaved)
    punpcklqdq           m1, m14, m15 ; 14 15 (interleaved)
    punpckhqdq          m14, m15      ; 30 31 (interleaved)
    mova          [r5+32*3], m8
    mova          [r5+32*2], m14
    IDCT32_END            2, 15, 8, 9, 10, 4
    IDCT32_END            3, 14, 8, 9, 10, 4
    punpckhqdq            m8, m2, m3   ; 18 19 (interleaved)
    punpcklqdq            m2, m3       ;  2  3 (interleaved)
    punpcklqdq            m3, m14, m15 ; 12 13 (interleaved)
    punpckhqdq           m14, m15      ; 28 29 (interleaved)
    mova          [r5+32*1], m8
    mova          [r5+32*0], m14
    IDCT32_END            4, 15, 8, 9, 10, 4
    IDCT32_END            5, 14, 8, 9, 10, 4
    punpckhqdq            m8, m4, m5   ; 20 21 (interleaved)
    punpcklqdq            m4, m5       ;  4  5 (interleaved)
    punpcklqdq            m5, m14, m15 ; 10 11 (interleaved)
    punpckhqdq           m14, m15      ; 26 27 (interleaved)
    mova          [r5-32*1], m8
    mova          [r5-32*2], m14
    IDCT32_END            6, 15, 8, 9, 10, 4
    IDCT32_END            7, 14, 8, 9, 10, 4
    punpckhqdq            m8, m6, m7   ; 22 23 (interleaved)
    punpcklqdq            m6, m7       ;  6  7 (interleaved)
    punpcklqdq            m7, m14, m15 ;  8  9 (interleaved)
    punpckhqdq           m14, m15      ; 24 25 (interleaved)
    mova          [r5-32*3], m8
    mova          [r5-32*4], m14
    mova                m15, m1
.end:
    vpermq               m0, m0, q3120
    vpermq               m1, m2, q3120
    call m(idct_8x8_internal_12bpc).write_8x4_start
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m4, q3120
    vpermq               m1, m6, q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m7, q3120
    vpermq               m1, m5, q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, m3, q3120
    vpermq               m1, m15, q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, [r5+32*3], q3120
    vpermq               m1, [r5+32*1], q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, [r5-32*1], q3120
    vpermq               m1, [r5-32*3], q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, [r5-32*4], q3120
    vpermq               m1, [r5-32*2], q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    vpermq               m0, [r5+32*0], q3120
    vpermq               m1, [r5+32*2], q3120
    call m(idct_8x8_internal_10bpc).write_8x4
    RET
.dconly:
    imul                r6d, [cq], 181
    vpbroadcastd         m2, [dconly_12bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 32
    add                 r6d, 640
    sar                 r6d, 10
    jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly3
ALIGN function_align
.pass1_main:
    call m(inv_txfm_add_dct_dct_8x32_10bpc).pass1_main_part1
    TRANSPOSE_8X8_DWORD   0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15
    mova         [cq+128*0], m0
    mova         [cq+128*1], m1
    mova         [cq+128*2], m2
    mova         [cq+128*3], m3
    mova         [cq+128*4], m4
    mova         [cq+128*5], m5
    mova         [cq+128*6], m6
    mova         [cq+128*7], m7
    add                  cq, 32
    ret
ALIGN function_align
.main_end:
    psrld               m11, 10 ; pd_2
    IDCT32_END            0, 15, 8, 9, 10, 2, 0
    mova         [cq+32*16], m8
    mova         [cq+32*31], m9
    IDCT32_END            1, 14, 8, 9, 10, 2, 0
    mova         [cq+32*17], m8
    mova         [cq+32*30], m9
    mova         [cq+32*14], m14
    IDCT32_END            2, 14, 8, 9, 10, 2, 0
    mova         [cq+32*18], m8
    mova         [cq+32*29], m9
    mova         [cq+32*13], m14
    IDCT32_END            3, 14, 8, 9, 10, 2, 0
    mova         [cq+32*19], m8
    mova         [cq+32*28], m9
    mova         [cq+32*12], m14
    IDCT32_END            4, 14, 8, 9, 10, 2, 0
    mova         [cq+32*20], m8
    mova         [cq+32*27], m9
    mova         [cq+32* 0], m0
    mova         [cq+32* 1], m1
    mova         [cq+32* 2], m2
    IDCT32_END            5, 10, 0, 1, 2, 2, 0
    mova         [cq+32*21], m0
    mova         [cq+32*26], m1
    IDCT32_END            6, 9, 0, 1, 2, 2, 0
    mova         [cq+32*22], m0
    mova         [cq+32*25], m1
    IDCT32_END            7, 8, 0, 1, 2, 2, 0
    mova         [cq+32*23], m0
    mova         [cq+32*24], m1
    mova                 m0, [cq+32* 0]
    mova                 m1, [cq+32* 1]
    mova                 m2, [cq+32* 2]
    mova                m11, m14
    mova                m12, [cq+32*12]
    mova                m13, [cq+32*13]
    mova                m14, [cq+32*14]
    ret

cglobal inv_txfm_add_identity_identity_8x32_12bpc, 4, 7, 8, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_12bpc_max]
    jmp m(inv_txfm_add_identity_identity_8x32_10bpc).pass1

cglobal inv_txfm_add_dct_dct_32x8_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jnz .full
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_10bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 8
.dconly:
    add                 r6d, 640
    sar                 r6d, 10
.dconly2:
    imul                r6d, 181
    add                 r6d, 2176
    sar                 r6d, 12
    movd                xm0, r6d
    paddsw              xm0, xm3
    vpbroadcastw         m0, xm0
.dconly_loop:
    paddsw               m1, m0, [dstq+32*0]
    paddsw               m2, m0, [dstq+32*1]
    psubusw              m1, m3
    psubusw              m2, m3
    mova        [dstq+32*0], m1
    mova        [dstq+32*1], m2
    add                dstq, strideq
    dec                 r3d
    jg .dconly_loop
    RET
.full:
    PROLOGUE              0, 7, 16, 32*24, dst, stride, c, eob
    lea                  r6, [rsp+32*4]
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    call .pass1
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_end
    lea                  r6, [deint_shuf+128]
    vpbroadcastd        m11, [pw_2048]
    mov                  r4, dstq
    call .pass2
    mova                 m0, [r5+32*3] ; 16 17
    mova                 m1, [r5+32*2] ; 30 31
    mova                 m2, [r5+32*1] ; 18 19
    mova                 m3, [r5+32*0] ; 28 29
    mova                 m4, [r5-32*1] ; 20 21
    mova                 m5, [r5-32*2] ; 26 27
    mova                 m6, [r5-32*3] ; 22 23
    mova                 m7, [r5-32*4] ; 24 25
    call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    lea                dstq, [r4+32]
    call .pass2
    RET
ALIGN function_align
.pass2:
    call m(idct_16x8_internal_8bpc).main
    REPX  {pmulhrsw x, m11}, m0, m1, m2, m3
    call m(idct_16x8_internal_10bpc).write_16x4_start
    pmulhrsw             m0, m11, m4
    pmulhrsw             m1, m11, m5
    pmulhrsw             m2, m11, m6
    pmulhrsw             m3, m11, m7
    jmp m(idct_16x8_internal_10bpc).write_16x4_zero
ALIGN function_align
.pass1:
    mova                 m0, [cq+32* 1]
    mova                 m1, [cq+32* 7]
    mova                 m2, [cq+32* 9]
    mova                 m3, [cq+32*15]
    mova                 m4, [cq+32*17]
    mova                 m5, [cq+32*23]
    mova                 m6, [cq+32*25]
    mova                 m7, [cq+32*31]
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m14, [pd_2896]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1
    mova                 m0, [cq+32* 3]
    mova                 m1, [cq+32* 5]
    mova                 m2, [cq+32*11]
    mova                 m3, [cq+32*13]
    mova                 m4, [cq+32*19]
    mova                 m5, [cq+32*21]
    mova                 m6, [cq+32*27]
    mova                 m7, [cq+32*29]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2
    mova                 m0, [cq+32* 2]
    mova                 m1, [cq+32* 6]
    mova                 m2, [cq+32*10]
    mova                 m3, [cq+32*14]
    mova                 m4, [cq+32*18]
    mova                 m5, [cq+32*22]
    mova                 m6, [cq+32*26]
    mova                 m7, [cq+32*30]
    call m(idct_8x16_internal_10bpc).main_oddhalf
    mova                 m0, [cq+32* 0]
    mova                 m1, [cq+32* 4]
    mova                 m2, [cq+32* 8]
    mova                 m3, [cq+32*12]
    mova                 m4, [cq+32*16]
    mova                 m5, [cq+32*20]
    mova                 m6, [cq+32*24]
    mova                 m7, [cq+32*28]
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
    ret

cglobal inv_txfm_add_identity_identity_32x8_10bpc, 4, 7, 8, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_10bpc_max]
.pass1:
    vpbroadcastd         m5, [pw_4096]
    pxor                 m6, m6
    mov                 r6d, eobd
    add                eobb, 21
    cmovc              eobd, r6d
    lea                  r6, [strideq*3]
    lea                  r5, [strideq*5]
    lea                  r4, [strideq+r6*2] ; strideq*7
.loop:
    mova                 m0, [cq+32*0]
    packssdw             m0, [cq+32*1]
    mova                 m1, [cq+32*2]
    packssdw             m1, [cq+32*3]
    REPX {mova [cq+32*x], m6}, 0, 1, 2, 3
    add                  cq, 32*8
    mova                 m2, [cq-32*4]
    packssdw             m2, [cq-32*3]
    mova                 m3, [cq-32*2]
    packssdw             m3, [cq-32*1]
    REPX   {pmulhrsw x, m5}, m0, m1, m2, m3
    REPX {mova [cq+32*x], m6}, -4, -3, -2, -1
    call m(inv_txfm_add_identity_identity_8x32_10bpc).main
    add                dstq, 16
    sub                eobd, 64
    jge .loop
    RET

cglobal inv_txfm_add_dct_dct_32x8_12bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jnz .full
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_12bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 8
    jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly
.full:
    PROLOGUE              0, 7, 16, 32*24, dst, stride, c, eob
    lea                  r6, [rsp+32*4]
    vpbroadcastd        m12, [clip_20b_min]
    vpbroadcastd        m13, [clip_20b_max]
    call m(inv_txfm_add_dct_dct_32x8_10bpc).pass1
    call m(inv_txfm_add_dct_dct_8x32_12bpc).main_end
    mov                  r4, dstq
    call m(idct_16x8_internal_12bpc).pass2_main
    mova                 m0, [cq+32* 0] ; 16
    mova                 m1, [cq+32* 1] ; 17
    mova                 m2, [cq+32* 2] ; 18
    mova                 m3, [cq+32* 3] ; 19
    mova                 m4, [cq+32* 4] ; 20
    mova                 m5, [cq+32* 5] ; 21
    mova                 m6, [cq+32* 6] ; 22
    mova                 m7, [cq+32* 7] ; 23
    mova                 m8, [cq+32* 8] ; 24
    mova                 m9, [cq+32* 9] ; 25
    mova                m10, [cq+32*10] ; 26
    mova                m11, [cq+32*11] ; 27
    mova                m12, [cq+32*12] ; 28
    mova                m13, [cq+32*13] ; 29
    mova                m14, [cq+32*14] ; 30
    mova                m15, [cq+32*15] ; 31
    lea                dstq, [r4+32]
    call m(idct_16x8_internal_12bpc).pass2_main
    RET

cglobal inv_txfm_add_identity_identity_32x8_12bpc, 4, 7, 8, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_12bpc_max]
    jmp m(inv_txfm_add_identity_identity_32x8_10bpc).pass1

%macro IDCT32_PASS2_END 6 ; coefs[1-2], tmp[1-2], offset[1-2]
    mova                m%4, [%2]
    paddsw              m%3, m%1, m%4
    psubsw              m%1, m%4
%if %1 == 0
    pxor                 m6, m6
%endif
    pmulhrsw            m%3, m15
    pmulhrsw            m%1, m15
    paddw               m%3, [dstq+%5]
    paddw               m%1, [r2+%6]
    pmaxsw              m%3, m6
    pmaxsw              m%1, m6
    pminsw              m%3, m7
    pminsw              m%1, m7
    mova          [dstq+%5], m%3
    mova            [r2+%6], m%1
%endmacro

cglobal inv_txfm_add_dct_dct_16x32_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 8, 16, 32*36, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vpbroadcastd        m14, [pd_2896]
    lea                  r6, [rsp+32*16]
    lea                  r4, [r6+32*8]
    lea                  r5, [r6+32*16]
    call .main
    sub                eobd, 44
    jge .eob44
    vperm2i128           m2, m0, m3, 0x31 ;  5
    vinserti128          m0, xm3, 1       ;  1
    vperm2i128           m3, m1, m4, 0x31 ;  7
    vinserti128          m1, xm4, 1       ;  3
    pxor                 m4, m4
    REPX       {mova x, m4}, m5, m6, m7
    REPX {mova [r6+32*x], m4}, 0, 1, 2, 3
    jmp .fast
.dconly:
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_10bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 32
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly2
.eob44:
    mova          [r4+16*0], xm0
    mova          [r4+16*1], xm3
    mova          [r4+16*2], xm1
    mova          [r4+16*3], xm4
    vextracti128  [r4+16*4], m0, 1
    vextracti128  [r4+16*5], m3, 1
    vextracti128  [r4+16*6], m1, 1
    vextracti128  [r4+16*7], m4, 1
    call .main
    sub                eobd, 107
    jge .eob151
    vperm2i128           m7, m1, m4, 0x31 ; 15
    vinserti128          m5, m1, xm4, 1   ; 11
    vperm2i128           m6, m0, m3, 0x31 ; 13
    vinserti128          m4, m0, xm3, 1   ;  9
    mova                 m0, [r4+32*0]
    mova                 m1, [r4+32*1]
    mova                 m2, [r4+32*2]
    mova                 m3, [r4+32*3]
.fast:
    lea                  r6, [pw_5+128]
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
    jmp .idct16
.eob151:
    mova          [r4-16*8], xm0
    mova          [r4-16*7], xm3
    mova          [r4-16*6], xm1
    mova          [r4-16*5], xm4
    vextracti128  [r4-16*4], m0, 1
    vextracti128  [r4-16*3], m3, 1
    vextracti128  [r4-16*2], m1, 1
    vextracti128  [r4-16*1], m4, 1
    call .main
    sub                eobd, 128
    jge .eob279
    vperm2i128          m10, m0, m3, 0x31 ; 21
    vinserti128          m8, m0, xm3, 1   ; 17
    vperm2i128          m11, m1, m4, 0x31 ; 23
    vinserti128          m9, m1, xm4, 1   ; 19
    pxor                m12, m12
    REPX      {mova x, m12}, m13, m14, m15
    REPX {mova [r6+32*x], m12}, 0, 1, 2, 3
    jmp .full
.eob279:
    mova          [r5+16*0], xm0
    mova          [r5+16*1], xm3
    mova          [r5+16*2], xm1
    mova          [r5+16*3], xm4
    vextracti128  [r5+16*4], m0, 1
    vextracti128  [r5+16*5], m3, 1
    vextracti128  [r5+16*6], m1, 1
    vextracti128  [r5+16*7], m4, 1
    call .main
    vperm2i128          m14, m0, m3, 0x31 ; 29
    vinserti128         m12, m0, xm3, 1   ; 25
    vperm2i128          m15, m1, m4, 0x31 ; 31
    vinserti128         m13, m1, xm4, 1   ; 27
    mova                 m8, [r5+32*0]
    mova                 m9, [r5+32*1]
    mova                m10, [r5+32*2]
    mova                m11, [r5+32*3]
.full:
    mova                 m0, [r4+32*0]
    mova                 m1, [r4+32*1]
    mova                 m2, [r4+32*2]
    mova                 m3, [r4+32*3]
    mova                 m4, [r4-32*4]
    mova                 m5, [r4-32*3]
    mova                 m6, [r4-32*2]
    mova                 m7, [r4-32*1]
    lea                  r6, [pw_5 + 128]
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
    lea                  r3, [rsp+32*8]
    mova                 m8, [r3+32*0]
    mova                 m9, [r3+32*1]
    mova                m10, [r3+32*2]
    mova                m11, [r3+32*3]
    mova                m12, [r3-32*4]
    mova                m13, [r3-32*3]
    mova                m14, [r3-32*2]
    mova                m15, [r3-32*1]
.idct16:
    lea                  r3, [rsp+32*16]
    mova                 m0, [r3+32*0]
    mova                 m1, [r3+32*1]
    mova                 m2, [r3+32*2]
    mova                 m3, [r3+32*3]
    mova                 m4, [r3-32*4]
    mova                 m5, [r3-32*3]
    mova                 m6, [r3-32*2]
    mova                 m7, [r3-32*1]
    mova              [rsp], m15
    call m(idct_16x16_internal_8bpc).main
    imul                 r2, strideq, 19
    lea                  r3, [strideq*3]
    add                  r2, dstq
    call .pass2_end
    RET
ALIGN function_align
.main:
    pmulld               m0, m14, [cq+128* 1]
    pmulld               m1, m14, [cq+128* 3]
    pmulld               m2, m14, [cq+128* 5]
    pmulld               m3, m14, [cq+128* 7]
    pmulld               m4, m14, [cq+128* 9]
    pmulld               m5, m14, [cq+128*11]
    pmulld               m6, m14, [cq+128*13]
    pmulld               m7, m14, [cq+128*15]
    call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    pmulld               m0, m14, [cq+128* 0]
    pmulld               m1, m14, [cq+128* 2]
    pmulld               m2, m14, [cq+128* 4]
    pmulld               m3, m14, [cq+128* 6]
    pmulld               m4, m14, [cq+128* 8]
    pmulld               m5, m14, [cq+128*10]
    pmulld               m6, m14, [cq+128*12]
    pmulld               m7, m14, [cq+128*14]
    call m(idct_8x8_internal_10bpc).main_rect2
    call m(idct_8x16_internal_10bpc).main_evenhalf
    psrld               m15, m11, 11 ; pd_1
    mova                 m8, [r6-32*4]
    mova                 m9, [r6-32*3]
    REPX     {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7
    psubd               m10, m0, m8 ; out15
    paddd                m0, m8     ; out0
    mova                 m8, [r6-32*2]
    paddd               m15, m1, m9 ; out1
    psubd                m1, m9     ; out14
    mova                 m9, [r6-32*1]
    REPX       {psrad x, 1}, m0, m15, m10, m1
    packssdw             m0, m15
    packssdw             m1, m10
    psubd               m10, m2, m8 ; out13
    paddd                m2, m8     ; out2
    mova                 m8, [r6+32*0]
    paddd               m15, m3, m9 ; out3
    psubd                m3, m9     ; out12
    mova                 m9, [r6+32*1]
    REPX       {psrad x, 1}, m2, m15, m10, m3
    packssdw             m2, m15
    packssdw             m3, m10
    psubd               m10, m4, m8 ; out11
    paddd                m4, m8     ; out4
    mova                 m8, [r6+32*2]
    paddd               m15, m5, m9 ; out5
    psubd                m5, m9     ; out10
    mova                 m9, [r6+32*3]
    REPX       {psrad x, 1}, m4, m10, m15, m5
    packssdw             m4, m15
    packssdw             m5, m10
    psubd               m10, m6, m8 ; out9
    paddd                m6, m8     ; out6
    paddd               m15, m7, m9 ; out7
    psubd                m7, m9     ; out8
    REPX       {psrad x, 1}, m6, m10, m15, m7
    packssdw             m6, m15
    packssdw             m7, m10
    punpckhwd            m8, m0, m2
    punpcklwd            m0, m2
    punpckhwd            m2, m3, m1
    punpcklwd            m3, m1
    punpckhwd            m1, m4, m6
    punpcklwd            m4, m6
    punpcklwd            m6, m7, m5
    punpckhwd            m7, m5
    pxor                 m5, m5
    mov                 r7d, 128*13
.main_zero_loop:
    mova      [cq+r7-128*1], m5
    mova      [cq+r7+128*0], m5
    mova      [cq+r7+128*1], m5
    mova      [cq+r7+128*2], m5
    sub                 r7d, 128*4
    jg .main_zero_loop
    add                  cq, 32
    punpcklwd            m5, m3, m2
    punpckhwd            m3, m2
    punpcklwd            m2, m4, m1
    punpckhwd            m4, m1
    punpckhwd            m1, m0, m8
    punpcklwd            m0, m8
    punpckhwd            m8, m6, m7
    punpcklwd            m6, m7
    punpcklqdq           m7, m1, m4
    punpckhqdq           m1, m4
    punpckhqdq           m4, m8, m3
    punpcklqdq           m8, m3
    punpckhqdq           m3, m6, m5
    punpcklqdq           m6, m5
    punpcklqdq           m5, m0, m2
    punpckhqdq           m0, m2
    mova          [r6+16*0], xm5
    mova          [r6+16*1], xm6
    mova          [r6+16*2], xm7
    mova          [r6+16*3], xm8
    vextracti128  [r6+16*4], m5, 1
    vextracti128  [r6+16*5], m6, 1
    vextracti128  [r6+16*6], m7, 1
    vextracti128  [r6+16*7], m8, 1
    sub                  r6, 32*4
    ret
ALIGN function_align
.pass2_end:
    mova [rsp+gprsize+32*0], m6
    mova [rsp+gprsize+32*2], m7
    mova [rsp+gprsize+32*3], m15
    vpbroadcastd        m15, [pw_2048]
    vpbroadcastd         m7, [pixel_10bpc_max]
    IDCT32_PASS2_END      0, r5+32*3, 1, 6, strideq*0, r3*4
    IDCT32_PASS2_END      4, r5-32*1, 0, 1, strideq*4, strideq*8
    IDCT32_PASS2_END      8, r4+32*3, 0, 4, strideq*8, strideq*4
    IDCT32_PASS2_END     12, r4-32*1, 0, 4, r3*4,      strideq*0
    add                dstq, strideq
    sub                  r2, strideq
    mova                 m1, [rsp+gprsize+32*1]
    IDCT32_PASS2_END      1, r5+32*2, 0, 4, strideq*0, r3*4
    IDCT32_PASS2_END      5, r5-32*2, 0, 4, strideq*4, strideq*8
    IDCT32_PASS2_END      9, r4+32*2, 0, 4, strideq*8, strideq*4
    IDCT32_PASS2_END     13, r4-32*2, 0, 4, r3*4,      strideq*0
    add                dstq, strideq
    sub                  r2, strideq
    mova                 m1, [rsp+gprsize+32*0]
    IDCT32_PASS2_END      2, r5+32*1, 0, 4, strideq*0, r3*4
    IDCT32_PASS2_END      1, r5-32*3, 0, 4, strideq*4, strideq*8
    IDCT32_PASS2_END     10, r4+32*1, 0, 4, strideq*8, strideq*4
    IDCT32_PASS2_END     14, r4-32*3, 0, 4, r3*4,      strideq*0
    add                dstq, strideq
    sub                  r2, strideq
    mova                 m1, [rsp+gprsize+32*2]
    mova                 m2, [rsp+gprsize+32*3]
    IDCT32_PASS2_END      3, r5+32*0, 0, 4, strideq*0, r3*4
    IDCT32_PASS2_END      1, r5-32*4, 0, 4, strideq*4, strideq*8
    IDCT32_PASS2_END     11, r4+32*0, 0, 4, strideq*8, strideq*4
    IDCT32_PASS2_END      2, r4-32*4, 0, 4, r3*4,      strideq*0
    ret

cglobal inv_txfm_add_identity_identity_16x32_10bpc, 4, 7, 12, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_10bpc_max]
.pass1:
    vpbroadcastd         m8, [pw_2896x8]
    vpbroadcastd         m9, [pw_1697x16]
    vpbroadcastd        m11, [pw_8192]
    lea                  r6, [strideq*5]
    pxor                 m6, m6
    paddw               m10, m11, m11 ; pw_16384
    mov                  r5, dstq
    call .main
    sub                eobd, 36
    jl .ret
    add                  cq, 128*8
    lea                dstq, [r5+16]
    call .main
    sub                  cq, 128*8-32
    lea                dstq, [r5+strideq*8]
    mov                  r5, dstq
    call .main
    sub                eobd, 107 ; eob < 143
    jl .ret
    add                  cq, 128*8
    lea                dstq, [r5+16]
    call .main
    sub                  cq, 128*8-32
    lea                dstq, [r5+strideq*8]
    mov                  r5, dstq
    call .main
    sub                eobd, 128 ; eob < 271
    jl .ret
    add                  cq, 128*8
    lea                dstq, [r5+16]
    call .main
    sub                  cq, 128*8-32
    lea                dstq, [r5+strideq*8]
    mov                  r5, dstq
    call .main
    sub                eobd, 128 ; eob < 399
    jl .ret
    add                  cq, 128*8
    lea                dstq, [r5+16]
    call .main
.ret:
    RET
ALIGN function_align
.main:
    mova                 m0, [cq+128*0]
    packssdw             m0, [cq+128*1]
    mova                 m1, [cq+128*2]
    packssdw             m1, [cq+128*3]
    mova                 m2, [cq+128*4]
    packssdw             m2, [cq+128*5]
    mova                 m3, [cq+128*6]
    packssdw             m3, [cq+128*7]
    REPX  {pmulhrsw x, m8 }, m0, m1, m2, m3
    REPX {IDTX16 x, 4, 9, 10}, 0, 1, 2, 3
    REPX  {pmulhrsw x, m11}, m0, m1, m2, m3
    REPX {mova [cq+128*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
.main2:
    punpckhwd            m4, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m2, m3
    punpcklwd            m2, m3
    punpckhwd            m3, m0, m4
    punpcklwd            m0, m4
    punpcklwd            m4, m2, m1
    punpckhwd            m2, m1
    punpckhqdq           m1, m0, m4
    punpcklqdq           m0, m4
    call m(iidentity_8x8_internal_10bpc).write_2x8x2
    punpcklqdq           m0, m3, m2
    punpckhqdq           m1, m3, m2
    jmp m(iidentity_8x8_internal_10bpc).write_2x8x2

cglobal inv_txfm_add_identity_identity_16x32_12bpc, 4, 7, 12, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_12bpc_max]
    jmp m(inv_txfm_add_identity_identity_16x32_10bpc).pass1

cglobal inv_txfm_add_dct_dct_32x16_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 8, 16, 32*40, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    lea                  r6, [rsp+32*4]
    call .main
    cmp                eobd, 36
    jge .full
    call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14, [rsp]
    lea                  r6, [pw_5+128]
    mov                  r7, dstq
    call m(idct_16x16_internal_8bpc).main
    call .write_16x16
    mova                 m0, [r5+32*3]
    mova                 m1, [r5+32*2]
    mova                 m2, [r5+32*1]
    mova                 m3, [r5+32*0]
    mova                 m4, [r5-32*1]
    mova                 m5, [r5-32*2]
    mova                 m6, [r5-32*3]
    mova                 m7, [r5-32*4]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14, [rsp]
    jmp .end
.dconly:
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_10bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 16
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    add                 r6d, 384
    sar                 r6d, 9
    jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly2
.full:
    add                  cq, 32
    mova          [r4+32*3], m0
    mova          [r4+32*2], m1
    mova          [r4+32*1], m2
    mova          [r4+32*0], m3
    mova          [r4-32*1], m4
    mova          [r4-32*2], m5
    mova          [r4-32*3], m6
    mova          [r4-32*4], m7
    call .main
    sub                  r4, 32*16 ; topleft 16x8
    call .transpose_16x16
    lea                  r6, [pw_5+128]
    mov                  r7, dstq
    call m(idct_16x16_internal_8bpc).main
    call .write_16x16
    mova                 m0, [r5+32*3]
    mova                 m1, [r5+32*2]
    mova                 m2, [r5+32*1]
    mova                 m3, [r5+32*0]
    mova                 m4, [r5-32*1]
    mova                 m5, [r5-32*2]
    mova                 m6, [r5-32*3]
    mova                 m7, [r5-32*4]
    add                  r4, 32*8 ; bottomleft 16x8
    call .transpose_16x16
.end:
    lea                dstq, [r7+32]
    call m(idct_16x16_internal_8bpc).main
    call .write_16x16
    RET
ALIGN function_align
.transpose_16x16:
    punpckhdq            m8, m3, m1
    punpckldq            m3, m1
    punpckhdq            m1, m0, m2
    punpckldq            m0, m2
    punpckhdq            m2, m7, m5
    punpckldq            m7, m5
    punpckhdq            m5, m4, m6
    punpckldq            m4, m6
    punpckhqdq           m6, m0, m4
    punpcklqdq           m0, m4
    punpckhqdq           m4, m1, m5
    punpcklqdq           m1, m5
    punpckhqdq           m5, m7, m3
    punpcklqdq           m7, m3
    punpckhqdq           m3, m2, m8
    punpcklqdq           m2, m8
    vinserti128          m8, m0, xm7, 1
    vperm2i128          m12, m0, m7, 0x31
    vinserti128          m9, m6, xm5, 1
    vperm2i128          m13, m6, m5, 0x31
    vinserti128         m10, m1, xm2, 1
    vperm2i128          m14, m1, m2, 0x31
    vinserti128         m11, m4, xm3, 1
    vperm2i128          m15, m4, m3, 0x31
    mova                 m0, [r4+32*3]
    mova                 m1, [r4+32*2]
    mova                 m2, [r4+32*1]
    mova                 m3, [r4+32*0]
    mova                 m4, [r4-32*1]
    mova                 m5, [r4-32*2]
    mova                 m6, [r4-32*3]
    mova                 m7, [r4-32*4]
    mova      [rsp+gprsize], m15
    jmp m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
ALIGN function_align
.main:
    vpbroadcastd        m14, [pd_2896]
    vpbroadcastd        m11, [pd_2048]
    pmulld               m0, m14, [cq+64* 1]
    pmulld               m1, m14, [cq+64* 7]
    pmulld               m2, m14, [cq+64* 9]
    pmulld               m3, m14, [cq+64*15]
    pmulld               m4, m14, [cq+64*17]
    pmulld               m5, m14, [cq+64*23]
    pmulld               m6, m14, [cq+64*25]
    pmulld               m7, m14, [cq+64*31]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_rect2
    pmulld               m0, m14, [cq+64* 3]
    pmulld               m1, m14, [cq+64* 5]
    pmulld               m2, m14, [cq+64*11]
    pmulld               m3, m14, [cq+64*13]
    pmulld               m4, m14, [cq+64*19]
    pmulld               m5, m14, [cq+64*21]
    pmulld               m6, m14, [cq+64*27]
    pmulld               m7, m14, [cq+64*29]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_rect2
    pmulld               m0, m14, [cq+64* 2]
    pmulld               m1, m14, [cq+64* 6]
    pmulld               m2, m14, [cq+64*10]
    pmulld               m3, m14, [cq+64*14]
    pmulld               m4, m14, [cq+64*18]
    pmulld               m5, m14, [cq+64*22]
    pmulld               m6, m14, [cq+64*26]
    pmulld               m7, m14, [cq+64*30]
    call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    pmulld               m0, m14, [cq+64* 0]
    pmulld               m1, m14, [cq+64* 4]
    pmulld               m2, m14, [cq+64* 8]
    pmulld               m3, m14, [cq+64*12]
    pmulld               m4, m14, [cq+64*16]
    pmulld               m5, m14, [cq+64*20]
    pmulld               m6, m14, [cq+64*24]
    pmulld               m7, m14, [cq+64*28]
    call m(idct_8x8_internal_10bpc).main_rect2
    call m(idct_8x16_internal_10bpc).main_evenhalf
    pxor                 m8, m8
    mov                 r7d, 64*30
.main_zero_loop:
    mova       [cq+r7-64*2], m8
    mova       [cq+r7-64*1], m8
    mova       [cq+r7+64*0], m8
    mova       [cq+r7+64*1], m8
    sub                 r7d, 64*4
    jg .main_zero_loop
.main_end:
    psrld               m11, 11 ; pd_1
    IDCT32_END            0, 15, 8, 9, 10, 1
    IDCT32_END            1, 14, 8, 9, 10, 1
    punpckhwd            m8, m0, m1   ; 16 17
    punpcklwd            m0, m1       ;  0  1
    punpcklwd            m1, m14, m15 ; 14 15
    punpckhwd           m14, m15      ; 30 31
    mova          [r5+32*3], m8
    mova          [r5+32*2], m14
    IDCT32_END            2, 15, 8, 9, 10, 1
    IDCT32_END            3, 14, 8, 9, 10, 1
    punpckhwd            m8, m2, m3   ; 18 19
    punpcklwd            m2, m3       ;  2  3
    punpcklwd            m3, m14, m15 ; 12 13
    punpckhwd           m14, m15      ; 28 29
    mova          [r5+32*1], m8
    mova          [r5+32*0], m14
    IDCT32_END            4, 15, 8, 9, 10, 1
    IDCT32_END            5, 14, 8, 9, 10, 1
    punpckhwd            m8, m4, m5   ; 20 21
    punpcklwd            m4, m5       ;  4  5
    punpcklwd            m5, m14, m15 ; 10 11
    punpckhwd           m14, m15      ; 26 27
    mova          [r5-32*1], m8
    mova          [r5-32*2], m14
    IDCT32_END            6, 15, 8, 9, 10, 1
    IDCT32_END            7, 14, 8, 9, 10, 1
    punpckhwd            m8, m6, m7   ; 22 23
    punpcklwd            m6, m7       ;  6  7
    punpcklwd            m7, m14, m15 ;  8  9
    punpckhwd           m14, m15      ; 24 25
    mova          [r5-32*3], m8
    mova          [r5-32*4], m14
    ret
ALIGN function_align
.write_16x16:
    mova                 m1, [rsp+gprsize+32*1]
    mova [rsp+gprsize+32*0], m8
    mova [rsp+gprsize+32*1], m9
    mova [rsp+gprsize+32*2], m12
    vpbroadcastd        m12, [pw_2048]
    vpbroadcastd         m9, [pixel_10bpc_max]
    lea                  r3, [strideq*3]
    pxor                 m8, m8
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    pmulhrsw             m2, m12
    pmulhrsw             m3, m12
    call m(idct_16x8_internal_10bpc).write_16x4
    pmulhrsw             m0, m12, m4
    pmulhrsw             m1, m12, m5
    pmulhrsw             m2, m12, m6
    pmulhrsw             m3, m12, m7
    call m(idct_16x8_internal_10bpc).write_16x4
    pmulhrsw             m0, m12, [rsp+gprsize+32*0]
    pmulhrsw             m1, m12, [rsp+gprsize+32*1]
    pmulhrsw             m2, m12, m10
    pmulhrsw             m3, m12, m11
    call m(idct_16x8_internal_10bpc).write_16x4
    pmulhrsw             m0, m12, [rsp+gprsize+32*2]
    pmulhrsw             m1, m12, m13
    pmulhrsw             m2, m12, m14
    pmulhrsw             m3, m12, m15
    jmp m(idct_16x8_internal_10bpc).write_16x4

cglobal inv_txfm_add_identity_identity_32x16_10bpc, 4, 7, 11, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_10bpc_max]
.pass1:
    vpbroadcastd         m8, [pw_2896x8]
    vpbroadcastd         m9, [pw_1697x16]
    vpbroadcastd        m10, [pw_4096]
    lea                  r6, [strideq*5]
    pxor                 m6, m6
    mov                  r5, dstq
    call .main
    sub                eobd, 36
    jl .ret
    add                  cq, 32
    lea                dstq, [dstq+strideq*4]
    call .main
    add                  cq, 64*8-32
    lea                dstq, [r5+16*1]
    call .main
    sub                eobd, 107 ; eob < 143
    jl .ret
    add                  cq, 32
    lea                dstq, [dstq+strideq*4]
    call .main
    add                  cq, 64*8-32
    lea                dstq, [r5+16*2]
    call .main
    sub                eobd, 128 ; eob < 271
    jl .ret
    add                  cq, 32
    lea                dstq, [dstq+strideq*4]
    call .main
    add                  cq, 64*8-32
    lea                dstq, [r5+16*3]
    call .main
    sub                eobd, 128 ; eob < 399
    jl .ret
    add                  cq, 32
    lea                dstq, [dstq+strideq*4]
    call .main
.ret:
    RET
ALIGN function_align
.main:
    mova                 m0, [cq+64*0]
    packssdw             m0, [cq+64*1]
    mova                 m1, [cq+64*2]
    packssdw             m1, [cq+64*3]
    mova                 m2, [cq+64*4]
    packssdw             m2, [cq+64*5]
    mova                 m3, [cq+64*6]
    packssdw             m3, [cq+64*7]
    REPX  {pmulhrsw x, m8 }, m0, m1, m2, m3
    REPX  {paddsw   x, x  }, m0, m1, m2, m3
    REPX  {IDTX16 x, 4, 9, _ }, 0, 1, 2, 3
    REPX  {pmulhrsw x, m10}, m0, m1, m2, m3
    REPX {mova [cq+64*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
    jmp m(inv_txfm_add_identity_identity_16x32_10bpc).main2

cglobal inv_txfm_add_identity_identity_32x16_12bpc, 4, 7, 11, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_12bpc_max]
    jmp m(inv_txfm_add_identity_identity_32x16_10bpc).pass1

cglobal inv_txfm_add_dct_dct_32x32_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 8, 16, 32*83, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    lea                  r6, [rsp+32*7]
    call .main
    cmp                eobd, 36
    jl .fast
    call .main
    cmp                eobd, 136
    jl .fast
    call .main
    cmp                eobd, 300
    jl .fast
    call .main
    jmp .pass2
.dconly:
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_10bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 32
    jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly
.fast:
    lea                  r4, [rsp+32*71]
    pxor                 m0, m0
.fast_loop:
    REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
    add                  r6, 32*8
    cmp                  r6, r4
    jl .fast_loop
.pass2:
    lea                  r3, [rsp+32*3]
    mov                  r4, r6
    lea                  r5, [r6+32*8]
    lea                  r6, [pw_5+128]
    call .pass2_oddhalf
    call .pass2_evenhalf
    imul                 r2, strideq, 19
    lea                  r3, [strideq*3]
    add                  r2, dstq
    call m(inv_txfm_add_dct_dct_16x32_10bpc).pass2_end
    sub                dstq, r3
    lea                  r2, [r2+r3+32]
    add                dstq, 32
    lea                  r3, [rsp+32*11]
    call .pass2_oddhalf
    call .pass2_evenhalf
    lea                  r3, [strideq*3]
    call m(inv_txfm_add_dct_dct_16x32_10bpc).pass2_end
    RET
ALIGN function_align
.main:
    mova                 m0, [cq+128* 1]
    mova                 m1, [cq+128* 7]
    mova                 m2, [cq+128* 9]
    mova                 m3, [cq+128*15]
    mova                 m4, [cq+128*17]
    mova                 m5, [cq+128*23]
    mova                 m6, [cq+128*25]
    mova                 m7, [cq+128*31]
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m14, [pd_2896]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1
    mova                 m0, [cq+128* 3]
    mova                 m1, [cq+128* 5]
    mova                 m2, [cq+128*11]
    mova                 m3, [cq+128*13]
    mova                 m4, [cq+128*19]
    mova                 m5, [cq+128*21]
    mova                 m6, [cq+128*27]
    mova                 m7, [cq+128*29]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2
    mova                 m0, [cq+128* 2]
    mova                 m1, [cq+128* 6]
    mova                 m2, [cq+128*10]
    mova                 m3, [cq+128*14]
    mova                 m4, [cq+128*18]
    mova                 m5, [cq+128*22]
    mova                 m6, [cq+128*26]
    mova                 m7, [cq+128*30]
    call m(idct_8x16_internal_10bpc).main_oddhalf
    mova                 m0, [cq+128* 0]
    mova                 m1, [cq+128* 4]
    mova                 m2, [cq+128* 8]
    mova                 m3, [cq+128*12]
    mova                 m4, [cq+128*16]
    mova                 m5, [cq+128*20]
    mova                 m6, [cq+128*24]
    mova                 m7, [cq+128*28]
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_end
    pxor                m15, m15
    mov                 r7d, 128*29
.main_zero_loop:
    mova      [cq+r7-128*1], m15
    mova      [cq+r7+128*0], m15
    mova      [cq+r7+128*1], m15
    mova      [cq+r7+128*2], m15
    sub                 r7d, 128*4
    jg .main_zero_loop
    add                  cq, 32
    mova          [r4-32*4], m0
    mova          [r4-32*3], m1
    mova          [r4-32*2], m2
    mova          [r4-32*1], m3
    mova          [r4+32*0], m4
    mova          [r4+32*1], m5
    mova          [r4+32*2], m6
    mova          [r4+32*3], m7
    mova                 m0, [r5+32*3]
    mova                 m1, [r5+32*2]
    mova                 m2, [r5+32*1]
    mova                 m3, [r5+32*0]
    mova                 m4, [r5-32*1]
    mova                 m5, [r5-32*2]
    mova                 m6, [r5-32*3]
    mova                 m7, [r5-32*4]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    mova          [r5-32*4], m0
    mova          [r5-32*3], m1
    mova          [r5-32*2], m2
    mova          [r5-32*1], m3
    mova          [r5+32*0], m4
    mova          [r5+32*1], m5
    mova          [r5+32*2], m6
    mova          [r5+32*3], m7
    ret
ALIGN function_align
.pass2_oddhalf:
    mova                 m0, [r3+32* 1] ;  1
    mova                 m1, [r3+32* 3] ;  3
    mova                 m2, [r3+32* 5] ;  5
    mova                 m3, [r3+32* 7] ;  7
    mova                 m4, [r3+32*17] ;  9
    mova                 m5, [r3+32*19] ; 11
    mova                 m6, [r3+32*21] ; 13
    mova                 m7, [r3+32*23] ; 15
    mova                 m8, [r3+32*33] ; 17
    mova                 m9, [r3+32*35] ; 19
    mova                m10, [r3+32*37] ; 21
    mova                m11, [r3+32*39] ; 23
    mova                m12, [r3+32*49] ; 25
    mova                m13, [r3+32*51] ; 27
    mova                m14, [r3+32*53] ; 29
    mova                m15, [r3+32*55] ; 31
    jmp m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
ALIGN function_align
.pass2_evenhalf:
    mova                 m0, [r3+32* 0] ;  0
    mova                 m1, [r3+32* 2] ;  2
    mova                 m2, [r3+32* 4] ;  4
    mova                 m3, [r3+32* 6] ;  6
    mova                 m4, [r3+32*16] ;  8
    mova                 m5, [r3+32*18] ; 10
    mova                 m6, [r3+32*20] ; 12
    mova                 m7, [r3+32*22] ; 14
    mova                 m8, [r3+32*32] ; 16
    mova                 m9, [r3+32*34] ; 18
    mova                m10, [r3+32*36] ; 20
    mova                m11, [r3+32*38] ; 22
    mova                m12, [r3+32*48] ; 24
    mova                m13, [r3+32*50] ; 26
    mova                m14, [r3+32*52] ; 28
    mova                m15, [r3+32*54] ; 30
    mova      [rsp+gprsize], m15
    jmp m(idct_16x16_internal_8bpc).main

cglobal inv_txfm_add_identity_identity_32x32_10bpc, 4, 8, 8, dst, stride, c, eob
%undef cmp
    vpbroadcastd         m7, [pixel_10bpc_max]
.pass1:
    vpbroadcastd         m5, [pw_8192]
    pxor                 m6, m6
    lea                  r6, [strideq*3]
    lea                  r5, [strideq*5]
    lea                  r4, [strideq+r6*2] ; strideq*7
    call .main                              ; 0
    cmp                eobd, 36
    jl .ret
    add                  cq, 128*8          ; 0 1
    mov                  r7, dstq           ; 1
    add                dstq, 16
    call .main
    call .main2
    cmp                eobd, 136
    jl .ret
    add                  cq, 128*16-32      ; 0 1 2
    lea                dstq, [r7+16*2]      ; 1 2
    call .main                              ; 2
    call .main2
    call .main2
    cmp                eobd, 300
    jl .ret
    add                  cq, 128*24-64      ; 0 1 2 3
    add                  r7, 16*3           ; 1 2 3
    mov                dstq, r7             ; 2 3
    call .main                              ; 3
    call .main2
    call .main2
    call .main2
    cmp                eobd, 535
    jl .ret
    add                  cq, 128*24-64      ; 0 1 2 3
    lea                dstq, [r7+strideq*8] ; 1 2 3 4
    mov                  r7, dstq           ; 2 3 4
    call .main                              ; 3 4
    call .main2
    call .main2
    cmp                eobd, 755
    jl .ret
    add                  cq, 128*16-32      ; 0 1 2 3
    lea                dstq, [r7+strideq*8] ; 1 2 3 4
    call .main                              ; 2 3 4 5
    call .main2                             ; 3 4 5
    cmp                eobd, 911
    jl .ret
    add                  cq, 128*8          ; 0 1 2 3
    add                dstq, 16             ; 1 2 3 4
    call .main                              ; 2 3 4 5
.ret:                                       ; 3 4 5 6
    RET
ALIGN function_align
.main2:
    sub                  cq, 128*8-32
    lea                dstq, [dstq+strideq*8-16]
.main:
    mova                 m0, [cq+128*0]
    packssdw             m0, [cq+128*1]
    mova                 m1, [cq+128*2]
    packssdw             m1, [cq+128*3]
    mova                 m2, [cq+128*4]
    packssdw             m2, [cq+128*5]
    mova                 m3, [cq+128*6]
    packssdw             m3, [cq+128*7]
    REPX   {pmulhrsw x, m5}, m0, m1, m2, m3
    jmp m(inv_txfm_add_identity_identity_8x32_10bpc).main_zero

cglobal inv_txfm_add_identity_identity_32x32_12bpc, 4, 8, 8, dst, stride, c, eob
    vpbroadcastd         m7, [pixel_12bpc_max]
    jmp m(inv_txfm_add_identity_identity_32x32_10bpc).pass1

%macro IDCT64_PART2_END 6-10 ; out, src[1-2], tmp[1-3], (offset[1-4])
%if %1 & 1
    mova                m%5, [r5-32*(51-%1)] ; idct16 out 0+n
    mova                m%4, [r4-32*(14+%1)] ; idct32 out31-n
%else
    mova                m%5, [r4-32*(45-%1)]
    mova                m%4, [r5-32*(20+%1)]
%endif
    paddsw              m%6, m%5, m%4 ; idct32 out 0+n
    psubsw              m%5, m%4      ; idct32 out31-n
    paddsw              m%4, m%5, m%3 ; out31-n
    psubsw              m%5, m%3      ; out32+n
    paddsw              m%3, m%6, m%2 ; out 0+n
    psubsw              m%6, m%2      ; out63-n
    REPX  {pmulhrsw x, m14}, m%5, m%6, m%4, m%3
%if %1 & 1
    %define %%d0 r2
    %define %%d1 dstq
%else
    %define %%d0 dstq
    %define %%d1 r2
%endif
    paddw               m%3, [%%d0+%7 ]
    paddw               m%4, [%%d1+%8 ]
    paddw               m%5, [%%d0+%9 ]
    paddw               m%6, [%%d1+%10]
    pxor                m%2, m%2
    REPX    {pmaxsw x, m%2}, m%3, m%4, m%5, m%6
    vpbroadcastd        m%2, [pixel_10bpc_max]
    REPX    {pminsw x, m%2}, m%3, m%4, m%5, m%6
    mova         [%%d0+%7 ], m%3
    mova         [%%d1+%8 ], m%4
    mova         [%%d0+%9 ], m%5
    mova         [%%d1+%10], m%6
%endmacro

cglobal inv_txfm_add_dct_dct_16x64_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 10, 16, 32*98, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vpbroadcastd        m14, [pd_2896]
    lea                  r6, [rsp+32*6]
    call .main
    sub                eobd, 44
    jl .fast
    call .main
    sub                eobd, 107
    jl .fast
    call .main
    sub                eobd, 128
    jl .fast
    call .main
    jmp .pass2
.dconly:
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_10bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 64
    add                 r6d, 640
    sar                 r6d, 10
    jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly3
.fast:
    lea                  r4, [rsp+32*38]
    pxor                 m0, m0
.fast_loop:
    REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
    add                  r6, 32*8
    cmp                  r6, r4
    jl .fast_loop
.pass2:
    lea                  r6, [pw_5+128]
    mova                 m0, [rsp+32* 2] ; in0
    mova                 m1, [rsp+32* 6] ; in4
    mova                 m2, [rsp+32*10] ; in8
    mova                 m3, [rsp+32*14] ; in12
    mova                 m4, [rsp+32*18] ; in16
    mova                 m5, [rsp+32*22] ; in20
    mova                 m6, [rsp+32*26] ; in24
    mova                 m7, [rsp+32*30] ; in28
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14
    mova              [rsp], m8
    call m(idct_16x16_internal_8bpc).main
    mova                 m1, [rsp+32*1]
    lea                  r4, [rsp+32*38]
    mova          [r4-32*4], m0
    mova          [r4-32*3], m1
    mova          [r4-32*2], m2
    mova          [r4-32*1], m3
    mova          [r4+32*0], m4
    mova          [r4+32*1], m5
    mova          [r4+32*2], m6
    mova          [r4+32*3], m7
    add                  r4, 32*8
    mova          [r4-32*4], m8
    mova          [r4-32*3], m9
    mova          [r4-32*2], m10
    mova          [r4-32*1], m11
    mova          [r4+32*0], m12
    mova          [r4+32*1], m13
    mova          [r4+32*2], m14
    mova          [r4+32*3], m15
    mova                 m0, [rsp+32* 4] ; in2
    mova                 m1, [rsp+32* 8] ; in6
    mova                 m2, [rsp+32*12] ; in10
    mova                 m3, [rsp+32*16] ; in14
    mova                 m4, [rsp+32*20] ; in18
    mova                 m5, [rsp+32*24] ; in22
    mova                 m6, [rsp+32*28] ; in26
    mova                 m7, [rsp+32*32] ; in30
    lea                  r5, [r4+32*16]
    add                  r4, 32*8
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    mova                 m0, [rsp+32* 3] ; in1
    mova                 m1, [rsp+32*33] ; in31
    mova                 m2, [rsp+32*19] ; in17
    mova                 m3, [rsp+32*17] ; in15
    mova                 m4, [rsp+32*11] ; in9
    mova                 m5, [rsp+32*25] ; in23
    mova                 m6, [rsp+32*27] ; in25
    mova                 m7, [rsp+32* 9] ; in7
    lea                  r6, [idct64_mul - 8]
    add                  r4, 32*16
    add                  r5, 32*32
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    mova                 m0, [rsp+32* 7] ; in5
    mova                 m1, [rsp+32*29] ; in27
    mova                 m2, [rsp+32*23] ; in21
    mova                 m3, [rsp+32*13] ; in11
    mova                 m4, [rsp+32*15] ; in13
    mova                 m5, [rsp+32*21] ; in19
    mova                 m6, [rsp+32*31] ; in29
    mova                 m7, [rsp+32* 5] ; in3
    add                  r6, 8
    add                  r4, 32*8
    sub                  r5, 32*8
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    lea                  r8, [strideq*4]
    lea                  r9, [strideq*5]
    lea                  r3, [r9+strideq*1] ; stride*6
    lea                  r7, [r9+strideq*2] ; stride*7
    call .main_part2_pass2
    RET
ALIGN function_align
.main:
    mova                 m0, [cq+128* 1]
    mova                 m1, [cq+128* 3]
    mova                 m2, [cq+128* 5]
    mova                 m3, [cq+128* 7]
    mova                 m4, [cq+128* 9]
    mova                 m5, [cq+128*11]
    mova                 m6, [cq+128*13]
    mova                 m7, [cq+128*15]
    call m(idct_8x16_internal_10bpc).main_oddhalf
    mova                 m0, [cq+128* 0]
    mova                 m1, [cq+128* 2]
    mova                 m2, [cq+128* 4]
    mova                 m3, [cq+128* 6]
    mova                 m4, [cq+128* 8]
    mova                 m5, [cq+128*10]
    mova                 m6, [cq+128*12]
    mova                 m7, [cq+128*14]
    call m(idct_8x8_internal_10bpc).main
    call m(idct_8x16_internal_10bpc).main_evenhalf
    pxor                m15, m15
    mov                 r7d, 128*13
.main_zero_loop:
    mova      [cq+r7-128*1], m15
    mova      [cq+r7+128*0], m15
    mova      [cq+r7+128*1], m15
    mova      [cq+r7+128*2], m15
    sub                 r7d, 128*4
    jg .main_zero_loop
    add                  cq, 32
    psrld               m15, m11, 10 ; pd_2
    mova                 m8, [r6-32*4]
    mova                 m9, [r6+32*3]
    REPX     {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7
    psubd               m10, m0, m8 ; out15
    paddd                m0, m8     ; out0
    mova                 m8, [r6-32*3]
    psubd               m15, m7, m9 ; out8
    paddd                m7, m9     ; out7
    mova                 m9, [r6+32*2]
    REPX       {psrad x, 2}, m0, m15, m10, m7
    packssdw             m0, m15
    packssdw             m7, m10
    psubd               m10, m1, m8 ; out14
    paddd                m1, m8     ; out1
    mova                 m8, [r6-32*2]
    psubd               m15, m6, m9 ; out9
    paddd                m6, m9     ; out6
    mova                 m9, [r6+32*1]
    REPX       {psrad x, 2}, m1, m15, m10, m6
    packssdw             m1, m15
    packssdw             m6, m10
    psubd               m10, m2, m8 ; out13
    paddd                m2, m8     ; out2
    mova                 m8, [r6-32*1]
    psubd               m15, m5, m9 ; out10
    paddd                m5, m9     ; out5
    mova                 m9, [r6+32*0]
    REPX       {psrad x, 2}, m2, m15, m10, m5
    packssdw             m2, m15
    packssdw             m5, m10
    psubd               m10, m3, m8 ; out12
    paddd                m3, m8     ; out3
    psubd               m15, m4, m9 ; out11
    paddd                m4, m9     ; out4
    REPX       {psrad x, 2}, m3, m15, m10, m4
    packssdw             m3, m15
    packssdw             m4, m10
    call m(idct_16x8_internal_10bpc).transpose3
    mova          [r6-32*4], m0
    mova          [r6-32*3], m1
    mova          [r6-32*2], m2
    mova          [r6-32*1], m3
    mova          [r6+32*0], m4
    mova          [r6+32*1], m5
    mova          [r6+32*2], m6
    mova          [r6+32*3], m7
    add                  r6, 32*8
    ret
.main_part2_pass2:
    vpbroadcastd        m11, [pw_1567_3784]
    vpbroadcastd        m12, [pw_m3784_1567]
    vpbroadcastd        m13, [pw_2896_2896]
    lea                  r6, [pw_5+128]
    lea                  r2, [dstq+r7]
.main_part2_pass2_loop:
    vpbroadcastd        m14, [pw_m2896_2896]
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_internal
    vpbroadcastd        m14, [pw_2048]
    IDCT64_PART2_END      0,  7,  0,  6,  9, 10, strideq*0, r3*4, r8*8, r7*8
    IDCT64_PART2_END      7,  8,  5,  0,  6,  7, strideq*0, r3*4, r8*8, r7*8
    IDCT64_PART2_END      8,  2,  1,  0,  6,  7, strideq*8, r8*4, r9*8, r3*8
    IDCT64_PART2_END     15,  3,  4,  0,  6,  7, strideq*8, r8*4, r9*8, r3*8
    add                dstq, strideq
    sub                  r2, strideq
    cmp                  r4, r5
    jne .main_part2_pass2_loop
    ret
ALIGN function_align
.main_part1_rect2:
    REPX     {paddd x, m11}, m0, m1, m2, m3
    REPX     {psrad x, 12 }, m0, m1, m2, m3
.main_part1: ; idct64 steps 1-5
    ; in1/31/17/15 -> t32a/33/34a/35/60/61a/62/63a
    ; in7/25/23/ 9 -> t56a/57/58a/59/36/37a/38/39a
    ; in5/27/21/11 -> t40a/41/42a/43/52/53a/54/55a
    ; in3/29/19/13 -> t48a/49/50a/51/44/45a/46/47a
    vpbroadcastd         m7, [r5+4*0]
    vpbroadcastd         m8, [r5+4*1]
    vpbroadcastd         m6, [r5+4*2]
    vpbroadcastd         m9, [r5+4*3]
    vpbroadcastd         m5, [r5+4*4]
    vpbroadcastd        m10, [r5+4*5]
    vpbroadcastd         m4, [r5+4*6]
    vpbroadcastd        m15, [r5+4*7]
    pmulld               m7, m0     ; t63a
    pmulld               m0, m8     ; t32a
    pmulld               m6, m1     ; t62a
    pmulld               m1, m9     ; t33a
    pmulld               m5, m2     ; t61a
    pmulld               m2, m10    ; t34a
    pmulld               m4, m3     ; t60a
    pmulld               m3, m15    ; t35a
    vpbroadcastd        m10, [r5+4*8]
    vpbroadcastd        m15, [r5+4*9]
    REPX     {paddd x, m11}, m7, m0, m6, m1, m5, m2, m4, m3
    REPX     {psrad x, 12 }, m0, m1, m7, m6, m2, m3, m5, m4
    psubd                m8, m0, m1 ; t33
    paddd                m0, m1     ; t32
    psubd                m1, m7, m6 ; t62
    paddd                m7, m6     ; t63
    psubd                m6, m3, m2 ; t34
    paddd                m3, m2     ; t35
    psubd                m2, m4, m5 ; t61
    paddd                m4, m5     ; t60
    REPX    {pmaxsd x, m12}, m8, m1, m6, m2
    REPX    {pminsd x, m13}, m8, m1, m6, m2
    ITX_MULSUB_2D         1, 8, 5, 9, _, 11, 10, 15    ; t33a, t62a
    ITX_MULSUB_2D         2, 6, 5, 9, _, 11, 10, 15, 2 ; t61a, t34a
    REPX    {pmaxsd x, m12}, m0, m3, m7, m4
    REPX    {pminsd x, m13}, m0, m3, m7, m4
    vpbroadcastd        m10, [r5+4*10]
    vpbroadcastd        m15, [r5+4*11]
    psubd                m5, m0, m3 ; t35a
    paddd                m0, m3     ; t32a
    psubd                m3, m7, m4 ; t60a
    paddd                m7, m4     ; t63a
    psubd                m4, m1, m6 ; t34
    paddd                m1, m6     ; t33
    psubd                m6, m8, m2 ; t61
    paddd                m8, m2     ; t62
    REPX    {pmaxsd x, m12}, m5, m3, m4, m6
    REPX    {pminsd x, m13}, m5, m3, m4, m6
    ITX_MULSUB_2D         3, 5, 2, 9, _, 11, 10, 15 ; t35,  t60
    ITX_MULSUB_2D         6, 4, 2, 9, _, 11, 10, 15 ; t34a, t61a
    REPX    {pmaxsd x, m12}, m0, m7, m1, m8
    REPX    {pminsd x, m13}, m0, m7, m1, m8
    add                  r5, 4*12
    mova          [r6-32*4], m0
    mova          [r6+32*3], m7
    mova          [r6-32*3], m1
    mova          [r6+32*2], m8
    mova          [r6-32*2], m6
    mova          [r6+32*1], m4
    mova          [r6-32*1], m3
    mova          [r6+32*0], m5
    add                  r6, 32*8
    ret
.main_part2: ; idct64 steps 6-9
    lea                  r5, [r6+32*3]
    sub                  r6, 32*4
    vpbroadcastd        m10, [pd_1567]
    vpbroadcastd        m15, [pd_3784]
.main_part2_loop:
    mova                 m0, [r6-32*32] ; t32a
    mova                 m1, [r5-32*24] ; t39a
    mova                 m2, [r5-32*32] ; t63a
    mova                 m3, [r6-32*24] ; t56a
    mova                 m4, [r6-32*16] ; t40a
    mova                 m5, [r5-32* 8] ; t47a
    mova                 m6, [r5-32*16] ; t55a
    mova                 m7, [r6-32* 8] ; t48a
    psubd                m8, m0, m1 ; t39
    paddd                m0, m1     ; t32
    psubd                m1, m2, m3 ; t56
    paddd                m2, m3     ; t63
    psubd                m3, m5, m4 ; t40
    paddd                m5, m4     ; t47
    psubd                m4, m7, m6 ; t55
    paddd                m7, m6     ; t48
    REPX    {pmaxsd x, m12}, m8, m1, m3, m4
    REPX    {pminsd x, m13}, m8, m1, m3, m4
    ITX_MULSUB_2D         1, 8, 6, 9, _, 11, 10, 15    ; t39a, t56a
    ITX_MULSUB_2D         4, 3, 6, 9, _, 11, 10, 15, 2 ; t55a, t40a
    REPX    {pmaxsd x, m12}, m0, m2, m5, m7
    REPX    {pminsd x, m13}, m0, m5, m2, m7
    psubd                m6, m2, m7 ; t48a
    paddd                m2, m7     ; t63a
    psubd                m7, m0, m5 ; t47a
    paddd                m0, m5     ; t32a
    psubd                m5, m8, m4 ; t55
    paddd                m8, m4     ; t56
    psubd                m4, m1, m3 ; t40
    paddd                m1, m3     ; t39
    REPX    {pmaxsd x, m12}, m6, m7, m5, m4
    REPX    {pminsd x, m13}, m6, m7, m5, m4
    REPX    {pmulld x, m14}, m6, m7, m5, m4
    REPX    {pmaxsd x, m12}, m2, m0, m8, m1
    REPX    {pminsd x, m13}, m2, m0, m8, m1
    paddd                m6, m11
    paddd                m5, m11
    psubd                m3, m6, m7 ; t47
    paddd                m6, m7     ; t48
    psubd                m7, m5, m4 ; t40a
    paddd                m5, m4     ; t55a
    REPX      {psrad x, 12}, m3, m6, m7, m5
    mova         [r5-32* 8], m2
    mova         [r6-32*32], m0
    mova         [r6-32* 8], m8
    mova         [r5-32*32], m1
    mova         [r5-32*24], m3
    mova         [r6-32*16], m6
    mova         [r6-32*24], m7
    mova         [r5-32*16], m5
    add                  r6, 32
    sub                  r5, 32
    cmp                  r6, r5
    jl .main_part2_loop
    ret

cglobal inv_txfm_add_dct_dct_32x64_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 11, 16, 32*134, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    lea                  r6, [rsp+32*6]
    call .main
    cmp                eobd, 36
    jl .fast
    call .main
    cmp                eobd, 136
    jl .fast
    call .main
    cmp                eobd, 300
    jl .fast
    call .main
    jmp .pass2
.dconly:
    imul                r6d, [cq], 181
    vpbroadcastd         m3, [dconly_10bpc]
    mov                [cq], eobd ; 0
    or                  r3d, 64
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    add                 r6d, 384
    sar                 r6d, 9
    jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly2
.fast:
    lea                  r4, [rsp+32*70]
    pxor                 m0, m0
.fast_loop:
    REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
    add                  r6, 32*8
    cmp                  r6, r4
    jl .fast_loop
.pass2:
    lea                  r6, [pw_5 + 128]
    mov                 r10, rsp
    lea                  r8, [strideq*4]
    lea                  r9, [strideq*5]
    lea                  r3, [r9+strideq*1] ; stride*6
    lea                  r7, [r9+strideq*2] ; stride*7
.pass2_loop:
    mova                 m0, [r10+32* 2] ; in0
    mova                 m1, [r10+32* 6] ; in4
    mova                 m2, [r10+32*18] ; in8
    mova                 m3, [r10+32*22] ; in12
    mova                 m4, [r10+32*34] ; in16
    mova                 m5, [r10+32*38] ; in20
    mova                 m6, [r10+32*50] ; in24
    mova                 m7, [r10+32*54] ; in28
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14
    mova              [rsp], m8
    call m(idct_16x16_internal_8bpc).main
    mova                 m1, [rsp+32*1]
    lea                  r4, [rsp+32*70]
    mova          [r4-32*4], m0
    mova          [r4-32*3], m1
    mova          [r4-32*2], m2
    mova          [r4-32*1], m3
    mova          [r4+32*0], m4
    mova          [r4+32*1], m5
    mova          [r4+32*2], m6
    mova          [r4+32*3], m7
    add                  r4, 32*8
    mova          [r4-32*4], m8
    mova          [r4-32*3], m9
    mova          [r4-32*2], m10
    mova          [r4-32*1], m11
    mova          [r4+32*0], m12
    mova          [r4+32*1], m13
    mova          [r4+32*2], m14
    mova          [r4+32*3], m15
    mova                 m0, [r10+32* 4] ; in2
    mova                 m1, [r10+32* 8] ; in6
    mova                 m2, [r10+32*20] ; in10
    mova                 m3, [r10+32*24] ; in14
    mova                 m4, [r10+32*36] ; in18
    mova                 m5, [r10+32*40] ; in22
    mova                 m6, [r10+32*52] ; in26
    mova                 m7, [r10+32*56] ; in30
    lea                  r5, [r4+32*16]
    add                  r4, 32*8
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    mova                 m0, [r10+32* 3] ; in1
    mova                 m1, [r10+32*57] ; in31
    mova                 m2, [r10+32*35] ; in17
    mova                 m3, [r10+32*25] ; in15
    mova                 m4, [r10+32*19] ; in9
    mova                 m5, [r10+32*41] ; in23
    mova                 m6, [r10+32*51] ; in25
    mova                 m7, [r10+32* 9] ; in7
    lea                  r6, [idct64_mul - 8]
    add                  r4, 32*16
    add                  r5, 32*32
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    mova                 m0, [r10+32* 7] ; in5
    mova                 m1, [r10+32*53] ; in27
    mova                 m2, [r10+32*39] ; in21
    mova                 m3, [r10+32*21] ; in11
    mova                 m4, [r10+32*23] ; in13
    mova                 m5, [r10+32*37] ; in19
    mova                 m6, [r10+32*55] ; in29
    mova                 m7, [r10+32* 5] ; in3
    add                  r6, 8
    add                  r4, 32*8
    sub                  r5, 32*8
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2_pass2
    add                 r10, 32*8
    sub                  r4, 32*98 ; rsp+32*16
    sub                dstq, r8
    add                dstq, 32
    cmp                 r10, r4
    jl .pass2_loop
    RET
ALIGN function_align
.main:
    vpbroadcastd        m14, [pd_2896]
    vpbroadcastd        m11, [pd_2048]
    pmulld               m0, m14, [cq+128* 1]
    pmulld               m1, m14, [cq+128* 7]
    pmulld               m2, m14, [cq+128* 9]
    pmulld               m3, m14, [cq+128*15]
    pmulld               m4, m14, [cq+128*17]
    pmulld               m5, m14, [cq+128*23]
    pmulld               m6, m14, [cq+128*25]
    pmulld               m7, m14, [cq+128*31]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_rect2
    pmulld               m0, m14, [cq+128* 3]
    pmulld               m1, m14, [cq+128* 5]
    pmulld               m2, m14, [cq+128*11]
    pmulld               m3, m14, [cq+128*13]
    pmulld               m4, m14, [cq+128*19]
    pmulld               m5, m14, [cq+128*21]
    pmulld               m6, m14, [cq+128*27]
    pmulld               m7, m14, [cq+128*29]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_rect2
    pmulld               m0, m14, [cq+128* 2]
    pmulld               m1, m14, [cq+128* 6]
    pmulld               m2, m14, [cq+128*10]
    pmulld               m3, m14, [cq+128*14]
    pmulld               m4, m14, [cq+128*18]
    pmulld               m5, m14, [cq+128*22]
    pmulld               m6, m14, [cq+128*26]
    pmulld               m7, m14, [cq+128*30]
    call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    pmulld               m0, m14, [cq+128* 0]
    pmulld               m1, m14, [cq+128* 4]
    pmulld               m2, m14, [cq+128* 8]
    pmulld               m3, m14, [cq+128*12]
    pmulld               m4, m14, [cq+128*16]
    pmulld               m5, m14, [cq+128*20]
    pmulld               m6, m14, [cq+128*24]
    pmulld               m7, m14, [cq+128*28]
    pxor                m15, m15
    mov                 r7d, 128*29
.main_zero_loop:
    mova      [cq+r7-128*1], m15
    mova      [cq+r7+128*0], m15
    mova      [cq+r7+128*1], m15
    mova      [cq+r7+128*2], m15
    sub                 r7d, 128*4
    jg .main_zero_loop
    add                  cq, 32
    call m(idct_8x8_internal_10bpc).main_rect2
    call m(idct_8x16_internal_10bpc).main_evenhalf
    call m(inv_txfm_add_dct_dct_32x16_10bpc).main_end
    call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    mova          [r4-32*4], m0
    mova          [r4-32*3], m1
    mova          [r4-32*2], m2
    mova          [r4-32*1], m3
    mova          [r4+32*0], m4
    mova          [r4+32*1], m5
    mova          [r4+32*2], m6
    mova          [r4+32*3], m7
    mova                 m0, [r5+32*3]
    mova                 m1, [r5+32*2]
    mova                 m2, [r5+32*1]
    mova                 m3, [r5+32*0]
    mova                 m4, [r5-32*1]
    mova                 m5, [r5-32*2]
    mova                 m6, [r5-32*3]
    mova                 m7, [r5-32*4]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    mova          [r5-32*4], m0
    mova          [r5-32*3], m1
    mova          [r5-32*2], m2
    mova          [r5-32*1], m3
    mova          [r5+32*0], m4
    mova          [r5+32*1], m5
    mova          [r5+32*2], m6
    mova          [r5+32*3], m7
    ret

cglobal inv_txfm_add_dct_dct_64x16_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jnz .normal
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 16
.dconly:
    add                 r6d, 640
    sar                 r6d, 10
.dconly2:
    vpbroadcastd         m5, [dconly_10bpc]
    imul                r6d, 181
    add                 r6d, 2176
    sar                 r6d, 12
    movd                xm0, r6d
    paddsw              xm0, xm5
    vpbroadcastw         m0, xm0
.dconly_loop:
    paddsw               m1, m0, [dstq+32*0]
    paddsw               m2, m0, [dstq+32*1]
    paddsw               m3, m0, [dstq+32*2]
    paddsw               m4, m0, [dstq+32*3]
    REPX    {psubusw x, m5}, m1, m2, m3, m4
    mova        [dstq+32*0], m1
    mova        [dstq+32*1], m2
    mova        [dstq+32*2], m3
    mova        [dstq+32*3], m4
    add                dstq, strideq
    dec                 r3d
    jg .dconly_loop
    RET
.normal:
    PROLOGUE              0, 8, 16, 32*96, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vpbroadcastd        m14, [pd_2896]
    lea                  r6, [rsp+32*4]
    call .main
    call .shift_transpose
    cmp                eobd, 36
    jl .fast
    call .main
    call .shift_transpose
    jmp .pass2
.fast:
    pxor                 m0, m0
    mov                 r3d, 4
.fast_loop:
    REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
    add                  r6, 32*8
    dec                 r3d
    jg .fast_loop
.pass2:
    lea                  r7, [r6-32*64]
    lea                  r4, [r6-32*32]
    lea                  r6, [pw_5+128]
    mov                  r5, dstq
.pass2_loop:
    mova                 m0, [r7-32*4]
    mova                 m1, [r7-32*3]
    mova                 m2, [r7-32*2]
    mova                 m3, [r7-32*1]
    mova                 m4, [r7+32*0]
    mova                 m5, [r7+32*1]
    mova                 m6, [r7+32*2]
    mova                 m7, [r7+32*3]
    add                  r7, 32*32
    mova                 m8, [r7-32*4]
    mova                 m9, [r7-32*3]
    mova                m10, [r7-32*2]
    mova                m11, [r7-32*1]
    mova                m12, [r7+32*0]
    mova                m13, [r7+32*1]
    mova                m14, [r7+32*2]
    mova                m15, [r7+32*3]
    sub                  r7, 32*24
    mova              [rsp], m15
    call m(idct_16x16_internal_8bpc).main
    mova                 m1, [rsp+32*1]
    call m(inv_txfm_add_dct_dct_32x16_10bpc).write_16x16
    add                  r5, 32
    mov                dstq, r5
    cmp                  r7, r4
    jl .pass2_loop
    RET
ALIGN function_align
.main:
    lea                  r5, [idct64_mul_16bpc]
    mova                 m0, [cq+64* 1]
    mova                 m1, [cq+64*31]
    mova                 m2, [cq+64*17]
    mova                 m3, [cq+64*15]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    mova                 m0, [cq+64* 7]
    mova                 m1, [cq+64*25]
    mova                 m2, [cq+64*23]
    mova                 m3, [cq+64* 9]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    mova                 m0, [cq+64* 5]
    mova                 m1, [cq+64*27]
    mova                 m2, [cq+64*21]
    mova                 m3, [cq+64*11]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    mova                 m0, [cq+64* 3]
    mova                 m1, [cq+64*29]
    mova                 m2, [cq+64*19]
    mova                 m3, [cq+64*13]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2
    mova                 m0, [cq+64* 2]
    mova                 m1, [cq+64*14]
    mova                 m2, [cq+64*18]
    mova                 m3, [cq+64*30]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast
    mova                 m0, [cq+64* 6]
    mova                 m1, [cq+64*10]
    mova                 m2, [cq+64*22]
    mova                 m3, [cq+64*26]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast
    mova                 m0, [cq+64* 4]
    mova                 m1, [cq+64*12]
    mova                 m2, [cq+64*20]
    mova                 m3, [cq+64*28]
    call m(idct_8x16_internal_10bpc).main_oddhalf_fast
    mova                 m0, [cq+64* 0]
    mova                 m1, [cq+64* 8]
    mova                 m2, [cq+64*16]
    mova                 m3, [cq+64*24]
    pxor                m15, m15
    mov                 r7d, 64*30
.main_zero_loop:
    mova       [cq+r7-64*2], m15
    mova       [cq+r7-64*1], m15
    mova       [cq+r7+64*0], m15
    mova       [cq+r7+64*1], m15
    sub                 r7d, 64*4
    jg .main_zero_loop
.main_end:
    psrld               m15, m11, 10 ; pd_2
.main_end2:
    add                  cq, 32
    pxor                 m4, m4
    REPX       {mova x, m4}, m5, m6, m7
    call m(idct_8x8_internal_10bpc).main
    add                  r6, 32*8
    call m(idct_8x16_internal_10bpc).main_evenhalf
    mova          [r6+32*2], m1
    mova          [r6+32*1], m2
    mova          [r6+32*0], m3
    mova          [r6-32*1], m4
    mova          [r6-32*2], m5
    mova          [r6-32*3], m6
    mova          [r6-32*4], m7
    jmp .main_end_loop_start
.main_end_loop:
    mova                 m0, [r6+32* 3] ; idct8  0  + n
.main_end_loop_start:
    mova                 m1, [r5+32* 4] ; idct16 15 - n
    mova                 m2, [r5-32*12] ; idct32 16 + n
    mova                 m3, [r6-32*13] ; idct32 31 - n
    mova                 m4, [r6-32*29] ; idct64 63 - n
    mova                 m5, [r5-32*28] ; idct64 48 + n
    mova                 m6, [r6-32*45] ; idct64 47 - n
    mova                 m7, [r5-32*44] ; idct64 32 + n
    paddd                m8, m0, m1     ; idct16 out0  + n
    psubd                m0, m1         ; idct16 out15 - n
    REPX    {pmaxsd x, m12}, m8, m0
    REPX    {pminsd x, m13}, m8, m0
    paddd                m1, m8, m3     ; idct32 out0  + n
    psubd                m8, m3         ; idct32 out31 - n
    paddd                m3, m0, m2     ; idct32 out15 - n
    psubd                m0, m2         ; idct32 out16 + n
    REPX    {pmaxsd x, m12}, m1, m8, m3, m0
    REPX    {pminsd x, m13}, m1, m3, m8, m0
    REPX    {paddd  x, m15}, m1, m3, m0, m8
    paddd                m2, m1, m4     ; idct64 out0  + n (unshifted)
    psubd                m1, m4         ; idct64 out63 - n (unshifted)
    paddd                m4, m3, m5     ; idct64 out15 - n (unshifted)
    psubd                m3, m5         ; idct64 out48 + n (unshifted)
    paddd                m5, m0, m6     ; idct64 out16 + n (unshifted)
    psubd                m0, m6         ; idct64 out47 - n (unshifted)
    paddd                m6, m8, m7     ; idct64 out31 - n (unshifted)
    psubd                m8, m7         ; idct64 out32 + n (unshifted)
    mova         [r5-32*44], m2
    mova         [r6+32* 3], m1
    mova         [r6-32*45], m4
    mova         [r5+32* 4], m3
    mova         [r5-32*28], m5
    mova         [r6-32*13], m0
    mova         [r6-32*29], m6
    mova         [r5-32*12], m8
    add                  r5, 32
    sub                  r6, 32
    cmp                  r5, r6
    jl .main_end_loop
    ret
.shift_transpose:
%macro IDCT64_SHIFT_TRANSPOSE 1 ; shift
    sub                  r6, 32*48
    mov                  r5, r6
%%loop:
    mova                 m0, [r6-32* 4]
    mova                 m4, [r6+32* 4]
    mova                 m1, [r6-32* 3]
    mova                 m5, [r6+32* 5]
    mova                 m2, [r6-32* 2]
    mova                 m6, [r6+32* 6]
    mova                 m3, [r6-32* 1]
    mova                 m7, [r6+32* 7]
    REPX      {psrad x, %1}, m0, m4, m1, m5, m2, m6, m3, m7
    packssdw             m0, m4
    packssdw             m1, m5
    packssdw             m2, m6
    packssdw             m3, m7
    mova                 m4, [r6+32* 0]
    mova                 m6, [r6+32* 8]
    mova                 m5, [r6+32* 1]
    mova                 m7, [r6+32* 9]
    REPX      {psrad x, %1}, m4, m6, m5, m7
    packssdw             m4, m6
    packssdw             m5, m7
    mova                 m6, [r6+32* 2]
    mova                 m8, [r6+32*10]
    mova                 m7, [r6+32* 3]
    mova                 m9, [r6+32*11]
    REPX      {psrad x, %1}, m6, m8, m7, m9
    packssdw             m6, m8
    packssdw             m7, m9
    call m(idct_16x8_internal_10bpc).transpose3
    mova          [r5-32*4], m0
    mova          [r5-32*3], m1
    mova          [r5-32*2], m2
    mova          [r5-32*1], m3
    mova          [r5+32*0], m4
    mova          [r5+32*1], m5
    mova          [r5+32*2], m6
    mova          [r5+32*3], m7
    add                  r6, 32*16
    add                  r5, 32*8
    cmp                  r5, r4
    jl %%loop
    mov                  r6, r4
%endmacro
    IDCT64_SHIFT_TRANSPOSE 2
    ret

cglobal inv_txfm_add_dct_dct_64x32_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 8, 16, 32*163, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vpbroadcastd        m14, [pd_2896]
    lea                  r6, [rsp+32*7]
    call .main
    cmp                eobd, 36
    jl .fast
    call .main
    cmp                eobd, 136
    jl .fast
    call .main
    cmp                eobd, 300
    jl .fast
    call .main
    jmp .pass2
.dconly:
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 32
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    add                 r6d, 384
    sar                 r6d, 9
    jmp m(inv_txfm_add_dct_dct_64x16_10bpc).dconly2
.fast:
    pxor                 m0, m0
    lea                  r4, [rsp+32*135]
.fast_loop:
    REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
    add                  r6, 32*8
    cmp                  r6, r4
    jl .fast_loop
.pass2:
    lea                  r7, [r6-32*32]
    lea                  r5, [r6+32*8]
    lea                  r6, [pw_5+128]
    imul                 r2, strideq, 19
    lea                  r3, [strideq*3]
    add                  r2, dstq
.pass2_loop:
    mova                 m0, [r7-32*99]
    mova                 m1, [r7-32*97]
    mova                 m2, [r7-32*95]
    mova                 m3, [r7-32*93]
    mova                 m4, [r7-32*67]
    mova                 m5, [r7-32*65]
    mova                 m6, [r7-32*63]
    mova                 m7, [r7-32*61]
    mova                 m8, [r7-32*35]
    mova                 m9, [r7-32*33]
    mova                m10, [r7-32*31]
    mova                m11, [r7-32*29]
    mova                m12, [r7-32* 3]
    mova                m13, [r7-32* 1]
    mova                m14, [r7+32* 1]
    mova                m15, [r7+32* 3]
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
    mova                 m0, [r7-32*100]
    mova                 m1, [r7-32*98]
    mova                 m2, [r7-32*96]
    mova                 m3, [r7-32*94]
    mova                 m4, [r7-32*68]
    mova                 m5, [r7-32*66]
    mova                 m6, [r7-32*64]
    mova                 m7, [r7-32*62]
    mova                 m8, [r7-32*36]
    mova                 m9, [r7-32*34]
    mova                m10, [r7-32*32]
    mova                m11, [r7-32*30]
    mova                m12, [r7-32* 4]
    mova                m13, [r7-32* 2]
    mova                m14, [r7+32* 0]
    mova                m15, [r7+32* 2]
    add                  r7, 32*8
    mova              [rsp], m15
    call m(idct_16x16_internal_8bpc).main
    call m(inv_txfm_add_dct_dct_16x32_10bpc).pass2_end
    sub                dstq, r3
    lea                  r2, [r2+r3+32]
    add                dstq, 32
    cmp                  r7, r4
    jl .pass2_loop
    RET
ALIGN function_align
.main:
    lea                  r5, [idct64_mul_16bpc]
    pmulld               m0, m14, [cq+128* 1]
    pmulld               m1, m14, [cq+128*31]
    pmulld               m2, m14, [cq+128*17]
    pmulld               m3, m14, [cq+128*15]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    pmulld               m0, m14, [cq+128* 7]
    pmulld               m1, m14, [cq+128*25]
    pmulld               m2, m14, [cq+128*23]
    pmulld               m3, m14, [cq+128* 9]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    pmulld               m0, m14, [cq+128* 5]
    pmulld               m1, m14, [cq+128*27]
    pmulld               m2, m14, [cq+128*21]
    pmulld               m3, m14, [cq+128*11]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    pmulld               m0, m14, [cq+128* 3]
    pmulld               m1, m14, [cq+128*29]
    pmulld               m2, m14, [cq+128*19]
    pmulld               m3, m14, [cq+128*13]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2
    pmulld               m0, m14, [cq+128* 2]
    pmulld               m1, m14, [cq+128*14]
    pmulld               m2, m14, [cq+128*18]
    pmulld               m3, m14, [cq+128*30]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast_rect2
    pmulld               m0, m14, [cq+128* 6]
    pmulld               m1, m14, [cq+128*10]
    pmulld               m2, m14, [cq+128*22]
    pmulld               m3, m14, [cq+128*26]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast_rect2
    pmulld               m0, m14, [cq+128* 4]
    pmulld               m1, m14, [cq+128*12]
    pmulld               m2, m14, [cq+128*20]
    pmulld               m3, m14, [cq+128*28]
    call m(idct_8x16_internal_10bpc).main_oddhalf_fast_rect2
    pmulld               m0, m14, [cq+128* 0]
    pmulld               m1, m14, [cq+128* 8]
    pmulld               m2, m14, [cq+128*16]
    pmulld               m3, m14, [cq+128*24]
    pxor                m15, m15
    mov                 r7d, 128*29
.main_zero_loop:
    mova      [cq+r7-128*1], m15
    mova      [cq+r7+128*0], m15
    mova      [cq+r7+128*1], m15
    mova      [cq+r7+128*2], m15
    sub                 r7d, 128*4
    jg .main_zero_loop
    psrld               m15, m11, 11 ; pd_1
    REPX     {paddd x, m11}, m0, m1, m2, m3
    REPX     {psrad x, 12 }, m0, m1, m2, m3
    call m(inv_txfm_add_dct_dct_64x16_10bpc).main_end2
    IDCT64_SHIFT_TRANSPOSE 1
    ret

cglobal inv_txfm_add_dct_dct_64x64_10bpc, 4, 7, 0, dst, stride, c, eob
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 11, 16, 32*195, dst, stride, c, eob
%undef cmp
    vpbroadcastd        m11, [pd_2048]
    vpbroadcastd        m12, [clip_18b_min]
    vpbroadcastd        m13, [clip_18b_max]
    vpbroadcastd        m14, [pd_2896]
    lea                  r6, [rsp+32*7]
    call .main
    cmp                eobd, 36
    jl .fast
    call .main
    cmp                eobd, 136
    jl .fast
    call .main
    cmp                eobd, 300
    jl .fast
    call .main
    jmp .pass2
.dconly:
    imul                r6d, [cq], 181
    mov                [cq], eobd ; 0
    or                  r3d, 64
    jmp m(inv_txfm_add_dct_dct_64x16_10bpc).dconly
.fast:
    pxor                 m0, m0
    lea                  r4, [rsp+32*135]
.fast_loop:
    REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
    add                  r6, 32*8
    cmp                  r6, r4
    jl .fast_loop
.pass2:
    lea                 r10, [r6-32*32]
    lea                  r6, [pw_5+128]
    lea                  r8, [strideq*4]
    lea                  r9, [strideq*5]
    lea                  r3, [r9+strideq*1] ; stride*6
    lea                  r7, [r9+strideq*2] ; stride*7
.pass2_loop:
    mova                 m0, [r10-32*100] ; in0
    mova                 m1, [r10-32*96]  ; in4
    mova                 m2, [r10-32*68]  ; in8
    mova                 m3, [r10-32*64]  ; in12
    mova                 m4, [r10-32*36]  ; in16
    mova                 m5, [r10-32*32]  ; in20
    mova                 m6, [r10-32* 4]  ; in24
    mova                 m7, [r10+32* 0]  ; in28
    pxor                 m8, m8
    REPX       {mova x, m8}, m9, m10, m11, m12, m13, m14
    mova              [rsp], m8
    call m(idct_16x16_internal_8bpc).main
    mova                 m1, [rsp+32*1]
    mova          [r4-32*4], m0
    mova          [r4-32*3], m1
    mova          [r4-32*2], m2
    mova          [r4-32*1], m3
    mova          [r4+32*0], m4
    mova          [r4+32*1], m5
    mova          [r4+32*2], m6
    mova          [r4+32*3], m7
    add                  r4, 32*8
    mova          [r4-32*4], m8
    mova          [r4-32*3], m9
    mova          [r4-32*2], m10
    mova          [r4-32*1], m11
    mova          [r4+32*0], m12
    mova          [r4+32*1], m13
    mova          [r4+32*2], m14
    mova          [r4+32*3], m15
    mova                 m0, [r10-32*98] ; in2
    mova                 m1, [r10-32*94] ; in6
    mova                 m2, [r10-32*66] ; in10
    mova                 m3, [r10-32*62] ; in14
    mova                 m4, [r10-32*34] ; in18
    mova                 m5, [r10-32*30] ; in22
    mova                 m6, [r10-32* 2] ; in26
    mova                 m7, [r10+32* 2] ; in30
    lea                  r5, [r4+32*16]
    add                  r4, 32*8
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    mova                 m0, [r10-32*99] ; in1
    mova                 m1, [r10+32* 3] ; in31
    mova                 m2, [r10-32*35] ; in17
    mova                 m3, [r10-32*61] ; in15
    mova                 m4, [r10-32*67] ; in9
    mova                 m5, [r10-32*29] ; in23
    mova                 m6, [r10-32* 3] ; in25
    mova                 m7, [r10-32*93] ; in7
    lea                  r6, [idct64_mul - 8]
    add                  r4, 32*16
    add                  r5, 32*32
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    mova                 m0, [r10-32*95] ; in5
    mova                 m1, [r10-32* 1] ; in27
    mova                 m2, [r10-32*31] ; in21
    mova                 m3, [r10-32*65] ; in11
    mova                 m4, [r10-32*63] ; in13
    mova                 m5, [r10-32*33] ; in19
    mova                 m6, [r10+32* 1] ; in29
    mova                 m7, [r10-32*97] ; in3
    add                  r6, 8
    add                  r4, 32*8
    sub                  r5, 32*8
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2_pass2
    add                 r10, 32*8
    sub                dstq, r8
    sub                  r4, 32*44
    add                dstq, 32
    cmp                 r10, r4
    jl .pass2_loop
    RET
ALIGN function_align
.main:
    lea                  r5, [idct64_mul_16bpc]
    mova                 m0, [cq+128* 1]
    mova                 m1, [cq+128*31]
    mova                 m2, [cq+128*17]
    mova                 m3, [cq+128*15]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    mova                 m0, [cq+128* 7]
    mova                 m1, [cq+128*25]
    mova                 m2, [cq+128*23]
    mova                 m3, [cq+128* 9]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    mova                 m0, [cq+128* 5]
    mova                 m1, [cq+128*27]
    mova                 m2, [cq+128*21]
    mova                 m3, [cq+128*11]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    mova                 m0, [cq+128* 3]
    mova                 m1, [cq+128*29]
    mova                 m2, [cq+128*19]
    mova                 m3, [cq+128*13]
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2
    mova                 m0, [cq+128* 2]
    mova                 m1, [cq+128*14]
    mova                 m2, [cq+128*18]
    mova                 m3, [cq+128*30]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast
    mova                 m0, [cq+128* 6]
    mova                 m1, [cq+128*10]
    mova                 m2, [cq+128*22]
    mova                 m3, [cq+128*26]
    call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast
    mova                 m0, [cq+128* 4]
    mova                 m1, [cq+128*12]
    mova                 m2, [cq+128*20]
    mova                 m3, [cq+128*28]
    call m(idct_8x16_internal_10bpc).main_oddhalf_fast
    mova                 m0, [cq+128* 0]
    mova                 m1, [cq+128* 8]
    mova                 m2, [cq+128*16]
    mova                 m3, [cq+128*24]
    pxor                m15, m15
    mov                 r7d, 128*29
.main_zero_loop:
    mova      [cq+r7-128*1], m15
    mova      [cq+r7+128*0], m15
    mova      [cq+r7+128*1], m15
    mova      [cq+r7+128*2], m15
    sub                 r7d, 128*4
    jg .main_zero_loop
    call m(inv_txfm_add_dct_dct_64x16_10bpc).main_end
    jmp m(inv_txfm_add_dct_dct_64x16_10bpc).shift_transpose

%endif ; ARCH_X86_64
