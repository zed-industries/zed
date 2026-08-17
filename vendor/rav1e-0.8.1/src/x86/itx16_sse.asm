; Copyright © 2021, VideoLAN and dav1d authors
; Copyright © 2021, Two Orioles, LLC
; Copyright © 2017-2021, The rav1e contributors
; Copyright © 2020, Nathan Egge
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

SECTION_RODATA
%macro COEF 1-2
pd_%1: times 4 dd %1
%if %0 == 2
pd_m%1: times 4 dd -%1
%endif
%endmacro

COEF  201
COEF  401
COEF  601, 1
COEF  799
COEF  995
COEF 1189, 1
COEF 1380, 1
COEF 1567
COEF 1751
COEF 1931
COEF 2106, 1
COEF 2276, 1
COEF 2440
COEF 2598, 1
COEF 2751, 1
COEF 2896
COEF 3035
COEF 3166
COEF 3290
COEF 3406
COEF 3513
COEF 3612
COEF 3703
COEF 3784
COEF 3857
COEF 3920
COEF 3973
COEF 4017
COEF 4052
COEF 4076
COEF 4091

deint_shuf:  db  0,  1,  4,  5,  8,  9, 12, 13,  2,  3,  6,  7, 10, 11, 14, 15

%if ARCH_X86_32
pd_1:            times 4 dd     1
%endif
pd_2:            times 4 dd     2
pw_5:            times 8 dw     5
pd_1321:         times 4 dd  1321
pd_2482:         times 4 dd  2482
pd_m3344:        times 4 dd -3344
pd_2048:         times 4 dd  2048
pw_4x2048_4xm2048: times 4 dw 2048
                   times 4 dw -2048
pw_4xm2048_4x2048: times 4 dw -2048
                   times 4 dw 2048
pw_2048:         times 8 dw  2048
pw_m2048:        times 8 dw  -2048
pd_3803:         times 4 dd  3803
pw_4096:         times 8 dw  4096
pd_5793:         times 4 dd  5793
pd_6144:         times 4 dd  6144
pw_8192:         times 8 dw  8192
pd_10240:        times 4 dd 10240
pd_11586:        times 4 dd 11586
pw_1697x8:       times 8 dw  1697*8
pw_2896x8:       times 8 dw  2896*8
pw_1697x16:      times 8 dw  1697*16
pw_16384:        times 8 dw 16384
pixel_10bpc_max: times 8 dw  0x03ff

pw_1567_3784:    times 4 dw  1567,  3784
pw_m3784_1567:   times 4 dw -3784,  1567
pw_2896_2896:    times 4 dw  2896,  2896
pw_m2896_2896:   times 4 dw -2896,  2896

clip_18b_min: times 4 dd -0x20000
clip_18b_max: times 4 dd  0x1ffff

idct64_mul_16bpc:
dd 4095,  101, 2967, -2824,  3745, 1660, 3822, -1474,   401,  4076,   799,  4017
dd -700, 4036, 2359,  3349, -2191, 3461,  897,  3996, -2598, -3166, -4017,  -799
dd 4065,  501, 3229, -2520,  3564, 2019, 3948, -1092,  1931,  3612,  3406,  2276
dd -301, 4085, 2675,  3102, -1842, 3659, 1285,  3889, -1189, -3920, -2276, -3406

cextern inv_txfm_add_dct_dct_4x4_8bpc_ssse3
cextern iadst_4x4_internal_8bpc_ssse3.main
cextern idct_4x8_internal_8bpc_ssse3.main
cextern iadst_4x8_internal_8bpc_ssse3.main
cextern idct_16x4_internal_8bpc_ssse3.main
cextern iadst_16x4_internal_8bpc_ssse3.main
cextern iadst_16x4_internal_8bpc_ssse3.main_pass2_end
cextern idct_8x4_internal_8bpc_ssse3.main
cextern iadst_8x4_internal_8bpc_ssse3.main
cextern idct_8x8_internal_8bpc_ssse3.main
cextern idct_8x8_internal_8bpc_ssse3.pass1_end3
cextern iadst_8x8_internal_8bpc_ssse3.main
cextern iadst_8x8_internal_8bpc_ssse3.main_pass2_end
cextern idct_16x8_internal_8bpc_ssse3.main
cextern iadst_16x8_internal_8bpc_ssse3.main
cextern iadst_16x8_internal_8bpc_ssse3.main_pass2_end
cextern idct_8x32_internal_8bpc_ssse3.main
cextern idct_8x32_internal_8bpc_ssse3.main_fast
cextern idct_8x32_internal_8bpc_ssse3.main_veryfast
cextern idct_16x64_internal_8bpc_ssse3.main
cextern idct_16x64_internal_8bpc_ssse3.main_fast

tbl_4x16_2d: db 0, 13, 29, 45
tbl_4x16_h: db 0, 16, 32, 48
tbl_4x16_v: db 0, 4, 8, 12

tbl_8x16_2d: db 0, 14, 30, 46
tbl_8x16_v: db 0, 4, 8, 12
tbl_8x16_h: db 0, 32, 64, 96

tbl_16x16_2d: db 0, 10, 36, 78
tbl_16x16_v: db 0, 4, 8, 12
tbl_16x16_h: db 0, 64, 128, 192

tbl_8x32_2d: dw 0, 14, 43, 75, 107, 139, 171, 203

tbl_16x32_2d: dw 0, 14, 44, 90, 151, 215, 279, 343

tbl_32x16_2d: ; first 4 entries of 32x32 are identical to this one
tbl_32x32_2d: dw 0, 10, 36, 78, 136, 210, 300, 406

tbl_Nx32_odd_offset: db 2*16, 2*23
                     db 2*20, 2*19
                     db 2*18, 2*21
                     db 2*22, 2*17
                     db 2*30, 2*25
                     db 2*26, 2*29
                     db 2*28, 2*27
                     db 2*24, 2*31

tbl_Nx64_offset: db 2* 0, 2*32, 2*16, 2*46
                 db 2* 8, 2*40, 2*23, 2*38
                 db 2* 1, 2*36, 2*20, 2*42
                 db 2* 9, 2*44, 2*19, 2*34
                 db 2* 2, 2*60, 2*18, 2*50
                 db 2*10, 2*52, 2*21, 2*58
                 db 2* 3, 2*56, 2*22, 2*54
                 db 2*11, 2*48, 2*17, 2*62

SECTION .text

%define m_suffix(x, sfx) mangle(private_prefix %+ _ %+ x %+ sfx)
%define m(x) m_suffix(x, SUFFIX)

; This refers to the first function in itx_sse i.e. the start of the text section
; which is needed as a base pointer for constants.
%define itx8_start m_suffix(inv_txfm_add_dct_dct_4x4_8bpc, _ssse3)

%if ARCH_X86_64
%define o(x) x
%else
%define o(x) r6-$$+x ; PIC
%endif

%macro IWHT4_1D 0
    ; m0 = in0,  m1 = in1,  m2 = in2,  m3 = in3
    paddd                m0, m1      ; in0 += in1
    psubd                m4, m2, m3  ; tmp0 = in2 - in3
    psubd                m5, m0, m4  ; tmp1 = (in0 - tmp0) >> 1
    psrad                m5, 1
    psubd                m2, m5, m1  ; in2 = tmp1 - in1
    psubd                m5, m3      ; in1 = tmp1 - in3
    psubd                m0, m5      ; in0 -= in1
    paddd                m4, m2      ; in3 = tmp0 + in2
    ; m0 = out0,  m1 = in1,  m2 = out2,  m3 = in3
    ; m4 = out3,  m5 = out1
%endmacro

INIT_XMM sse2
cglobal inv_txfm_add_wht_wht_4x4_16bpc, 3, 3, 6, dst, stride, c, eob, bdmax
    mova                 m0, [cq+16*0]
    mova                 m1, [cq+16*1]
    mova                 m2, [cq+16*2]
    mova                 m3, [cq+16*3]
    REPX       {psrad x, 2}, m0, m1, m2, m3
    IWHT4_1D
    punpckldq            m1, m0, m5
    punpckhdq            m3, m0, m5
    punpckldq            m5, m2, m4
    punpckhdq            m2, m4
    punpcklqdq           m0, m1, m5
    punpckhqdq           m1, m5
    punpcklqdq           m4, m3, m2
    punpckhqdq           m3, m2
    mova                 m2, m4
    IWHT4_1D
    packssdw             m0, m4 ; low: out3,  high: out0
    packssdw             m2, m5 ; low: out2,  high: out1
    pxor                 m4, m4
    mova          [cq+16*0], m4
    mova          [cq+16*1], m4
    mova          [cq+16*2], m4
    mova          [cq+16*3], m4
    lea                  r2, [dstq+strideq*2]
    movq                 m1, [dstq+strideq*0]
    movhps               m1, [r2  +strideq*1]
    movq                 m3, [r2  +strideq*0]
    movhps               m3, [dstq+strideq*1]
    movd                 m5, bdmaxm
    pshuflw              m5, m5, q0000  ; broadcast
    punpcklqdq           m5, m5         ; broadcast
    paddsw               m0, m1
    paddsw               m2, m3
    pmaxsw               m0, m4
    pmaxsw               m2, m4
    pminsw               m0, m5
    pminsw               m2, m5
    movhps [r2  +strideq*1], m0 ; write out0
    movhps [dstq+strideq*1], m2 ; write out1
    movq   [r2  +strideq*0], m2 ; write out2
    movq   [dstq+strideq*0], m0 ; write out3
    RET

; dst1 = (src1 * coef1 - src2 * coef2 + rnd) >> 12
; dst2 = (src1 * coef2 + src2 * coef1 + rnd) >> 12
; flags: 2 = inv_dst1, 4 = inv_dst2
; skip round/shift if rnd is not a number
%macro ITX_MULSUB_2D 8-9 0 ; dst/src[1-2], tmp[1-3], rnd, coef[1-2], flags
; %1 dst/src[1]
; %2 dst/src[2]
; %3 tmp[1]
; %4 tmp[2]
; %5 tmp[3]
; %6 rnd
; %7 coef[1]
; %8 coef[2]
; %9 flags
%ifnidn %7,%8   ; optimize when coef1 == coef2
%if %8 < 32
    pmulld              m%4, m%1, m%8
    pmulld              m%3, m%2, m%8
%else
    mova                m%3, [o(pd_%8)]
    pmulld              m%4, m%1, m%3
    pmulld              m%3, m%2
%endif
%endif
%if %7 < 32
    pmulld              m%1, m%7
    pmulld              m%2, m%7
%else
    mova                m%5, [o(pd_%7)]
    pmulld              m%1, m%5
    pmulld              m%2, m%5
%endif
%if %9 & 4  ; invert dst2
    paddd               m%4, m%2
    psubd               m%2, m%6, m%4
%else
%ifnum %6
%ifnidn %7,%8
    paddd               m%4, m%6
%else
    paddd               m%1, m%6
%endif
%endif
%ifnidn %7,%8
    paddd               m%2, m%4
%else
    mova                m%3, m%2
    paddd               m%2, m%1
%endif
%endif
%if %9 & 2  ; invert dst1
    psubd               m%3, m%1
    paddd               m%1, m%3, m%6
%else
%ifnum %6
%ifnidn %7,%8
    paddd               m%1, m%6
%endif
%endif
    psubd               m%1, m%3
%endif
%ifnum %6
    psrad               m%2, 12
    psrad               m%1, 12
%endif
%endmacro

%macro INV_TXFM_FN 4-5+ 8 ; type1, type2, eob_offset, size, mmsize/stack
cglobal inv_txfm_add_%1_%2_%4_16bpc, 4, 7, %5, dst, stride, c, eob, tx2
    %define %%p1 m(i%1_%4_internal_16bpc)
%if ARCH_X86_32
    LEA                  r6, $$
%endif
%if has_epilogue
%ifidn %1_%2, dct_dct
    test               eobd, eobd
    jz %%end
%endif
    lea                tx2q, [o(m(i%2_%4_internal_16bpc).pass2)]
%ifnum %3
%if %3
    add                eobd, %3
%endif
%else
    lea                  r5, [o(%3)]
%endif
    call %%p1
    RET
%%end:
%else
    ; Jump to the 1st txfm function if we're not taking the fast path, which
    ; in turn performs an indirect jump to the 2nd txfm function.
    lea                tx2q, [o(m(i%2_%4_internal_16bpc).pass2)]
%ifnum %3
%if %3
    add                eobd, %3
%endif
%else
    lea                  r5, [o(%3)]
%endif
%ifidn %1_%2, dct_dct
    test               eobd, eobd
    jnz %%p1
%else
    ; jump to the 1st txfm function unless it's located directly after this
    times ((%%end - %%p1) >> 31) & 1 jmp %%p1
ALIGN function_align
%%end:
%endif
%endif
%endmacro

%macro INV_TXFM_4X4_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 0, 4x4
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 4
.dconly:
    add                 r5d, 128
    sar                 r5d, 8
.dconly2:
    imul                r5d, 2896
    mova                 m2, [o(pixel_10bpc_max)]
    add                 r5d, 34816
    movd                 m0, r5d
    pshuflw              m0, m0, q1111
    pxor                 m3, m3
    punpcklqdq           m0, m0
.dconly_loop:
    movq                 m1, [dstq+strideq*0]
    movhps               m1, [dstq+strideq*1]
    paddw                m1, m0
    pminsw               m1, m2
    pmaxsw               m1, m3
    movq   [dstq+strideq*0], m1
    movhps [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    sub                 r3d, 2
    jg .dconly_loop
    RET
%endif
%endmacro

%macro IDCT4_1D 8 ; src[1-4], tmp[1-3], rnd
    ; butterfly rotation
    ITX_MULSUB_2D        %1, %3, %5, %6, %7, %8, 2896, 2896 ; %1 out1  %3 out0
    ITX_MULSUB_2D        %2, %4, %5, %6, %7, %8, 1567, 3784 ; %2 out2  %4 out3
    ; Hadamard rotation
    psubd               m%5, m%1, m%2
    paddd               m%2, m%1
    paddd               m%1, m%3, m%4
    psubd               m%3, m%4
    ; %1 (src1) = out0
    ; %2 (src2) = out1
    ; %3 (src3) = out3
    ; $5 (tmp1) = out2
%endmacro

INIT_XMM sse4

INV_TXFM_4X4_FN dct, dct
INV_TXFM_4X4_FN dct, identity
INV_TXFM_4X4_FN dct, adst
INV_TXFM_4X4_FN dct, flipadst

cglobal idct_4x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+16*0]
    mova                 m1, [cq+16*1]
    mova                 m2, [cq+16*2]
    mova                 m3, [cq+16*3]
    mova                 m5, [o(pd_2048)]
    call .pass1_main
    packssdw             m0, m1     ; out0 out1
    packssdw             m4, m2     ; out2 out3
    ; transpose
    punpckhwd            m2, m0, m4
    punpcklwd            m0, m4
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    ; m0 = out0 out1
    ; m1 = out2 out3
    ; m5 = pd_2048
    jmp                tx2q
.pass1_main:
    IDCT4_1D              0, 1, 2, 3, 4, 6, 7, 5
    ret
.pass2:
    ; m0 = in0 in1
    ; m1 = in2 in3
    ; m5 = pd_2048
    punpckhwd            m2, m1, m0
    punpcklwd            m1, m0
    pmaddwd              m4, m2, [o(pw_m3784_1567)]
    pmaddwd              m2, [o(pw_1567_3784)]
    pmaddwd              m0, m1, [o(pw_m2896_2896)]
    pmaddwd              m1, [o(pw_2896_2896)]
    REPX      {paddd x, m5}, m4, m2, m0, m1
    packssdw             m5, m5     ; pw_2048
    REPX      {psrad x, 12}, m4, m2, m0, m1
    packssdw             m2, m4     ; t3 t2
    packssdw             m1, m0     ; t0 t1
    paddsw               m0, m1, m2 ; out0 out1
    psubsw               m1, m2     ; out3 out2
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    movq                 m2, [dstq+strideq*0]
    movhps               m2, [dstq+strideq*1]
    lea                  r5, [dstq+strideq*2]
    movq                 m3, [r5  +strideq*1]
    movhps               m3, [r5  +strideq*0]
    mova                 m5, [o(pixel_10bpc_max)]
    pxor                 m4, m4
    mova          [cq+16*0], m4
    mova          [cq+16*1], m4
    mova          [cq+16*2], m4
    mova          [cq+16*3], m4
    paddw                m0, m2
    paddw                m1, m3
    pmaxsw               m0, m4
    pmaxsw               m1, m4
    pminsw               m0, m5
    pminsw               m1, m5
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    movhps [r5  +strideq*0], m1
    movq   [r5  +strideq*1], m1
    RET

INV_TXFM_4X4_FN adst, dct
INV_TXFM_4X4_FN adst, adst
INV_TXFM_4X4_FN adst, flipadst
INV_TXFM_4X4_FN adst, identity

cglobal iadst_4x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    call .main
    packssdw             m0, m2            ; out0 out1
    packssdw             m1, m4            ; out2 out3
    ; transpose
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    ; m0 = out0 out1
    ; m1 = out2 out3
    ; m5 = pd_2048
    jmp                tx2q
.pass2:
    ; m0 = in0 in1
    ; m1 = in2 in3
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_4x4_internal_8bpc, _ssse3).main
.end:
    mova                 m4, [o(pw_2048)]
    movq                 m2, [dstq+strideq*0]
    movhps               m2, [dstq+strideq*1]
    lea                  r5, [dstq+strideq*2]
    movq                 m3, [r5  +strideq*0]
    movhps               m3, [r5  +strideq*1]
    mova                 m5, [o(pixel_10bpc_max)]
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    pxor                 m4, m4
    mova          [cq+16*0], m4
    mova          [cq+16*1], m4
    mova          [cq+16*2], m4
    mova          [cq+16*3], m4
    paddw                m0, m2
    paddw                m1, m3
    pmaxsw               m0, m4
    pmaxsw               m1, m4
    pminsw               m0, m5
    pminsw               m1, m5
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    movq   [r5  +strideq*0], m1
    movhps [r5  +strideq*1], m1
    RET
ALIGN function_align
.main:
    mova                 m1, [cq+16*2]
    mova                 m3, [cq+16*3]
    mova                 m5, [cq+16*0]
    lea                  r3, [cq+16*1]
.main2:
    mova                 m0, [o(pd_1321)]  ; SINPI_1_9
    mova                 m2, [o(pd_2482)]  ; SINPI_2_9
    mova                 m6, [o(pd_3803)]  ; SINPI_4_9
    pmulld               m4, m0, m1        ; s[4] = SINPI_1_9 * T[2]
    pmulld               m7, m3, m6        ; s[6] = SINPI_4_9 * T[3]
    pmulld               m6, m1            ; s[3] = SINPI_4_9 * T[2]
    pmulld               m0, m5            ; s[0] = SINPI_1_9 * T[0]
    psubd                m1, m3            ; T[2] - T[3]
    pmulld               m3, m2            ; s[5] = SINPI_2_9 * T[3]
    pmulld               m2, m5            ; s[1] = SINPI_2_9 * T[0]
    paddd                m0, m6            ; s[0] += s[3]
    paddd                m0, m3            ; s[0] += s[5]
    mova                 m3, [o(pd_m3344)] ; -SINPI_3_9
    psubd                m2, m4            ; s[1] -= s[4]
    psubd                m2, m7            ; s[1] -= s[6]
    psubd                m1, m5            ; -b7 = (T[2] -T[3]) - T[0]
    pmulld               m1, m3            ; s[2]  = -SINPI_3_9 * -b7
    pmulld               m3, [r3]          ; -s[3] = -SINPI_3_9 * T[1]
    mova                 m5, [o(pd_2048)]
    REPX      {paddd x, m5}, m0, m1        ; {s[0], s[2]} + 2048
    paddd                m4, m0, m2        ; x[3]  = s[0] + s[1]
    psubd                m2, m3            ; x[1]  = s[1] + s[3]
    psubd                m0, m3            ; x[0]  = s[0] + s[3]
    paddd                m4, m3            ; x[3] -= s[3]
    paddd                m2, m5            ; x[1] + 2048
    REPX      {psrad x, 12}, m0, m2, m1, m4
    ret


INV_TXFM_4X4_FN flipadst, dct
INV_TXFM_4X4_FN flipadst, adst
INV_TXFM_4X4_FN flipadst, flipadst
INV_TXFM_4X4_FN flipadst, identity

cglobal iflipadst_4x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    call m(iadst_4x4_internal_16bpc).main
    packssdw             m0, m2            ; out0 out1
    packssdw             m1, m4            ; out2 out3
    ; transpose
    punpcklwd            m2, m1, m0
    punpckhwd            m1, m0
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    ; m0 = out0 out1
    ; m1 = out2 out3
    ; m5 = pd_2048
    jmp                tx2q
.pass2:
    ; m0 = in0 in1
    ; m1 = in2 in3
%if ARCH_X86_32
    lea                 r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_4x4_internal_8bpc, _ssse3).main
    mova                 m4, [o(pw_2048)]
    movq                 m3, [dstq+strideq*1]
    movhps               m3, [dstq+strideq*0]
    lea                  r5, [dstq+strideq*2]
    movq                 m2, [r5  +strideq*1]
    movhps               m2, [r5  +strideq*0]
    mova                 m5, [o(pixel_10bpc_max)]
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    pxor                 m4, m4
    mova          [cq+16*0], m4
    mova          [cq+16*1], m4
    mova          [cq+16*2], m4
    mova          [cq+16*3], m4
    paddw                m0, m2
    paddw                m1, m3
    pmaxsw               m0, m4
    pmaxsw               m1, m4
    pminsw               m0, m5
    pminsw               m1, m5
    movhps [dstq+strideq*0], m1
    movq   [dstq+strideq*1], m1
    movhps [r5  +strideq*0], m0
    movq   [r5  +strideq*1], m0
    RET

INV_TXFM_4X4_FN identity, dct
INV_TXFM_4X4_FN identity, adst
INV_TXFM_4X4_FN identity, flipadst
INV_TXFM_4X4_FN identity, identity

cglobal iidentity_4x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    mova                 m3, [o(pd_5793)]
    pmulld               m0, m3, [cq+16*0]
    pmulld               m1, m3, [cq+16*1]
    pmulld               m2, m3, [cq+16*2]
    pmulld               m3,     [cq+16*3]
    mova                 m5, [o(pd_2048)]
    REPX      {paddd x, m5}, m0, m1, m2, m3
    REPX      {psrad x, 12}, m0, m1, m2, m3
    packssdw             m0, m1
    packssdw             m2, m3
    ; transpose
    punpckhwd            m3, m0, m2
    punpcklwd            m0, m2
    punpckhwd            m1, m0, m3
    punpcklwd            m0, m3
    ; m0 = out0 out1
    ; m1 = out2 out3
    ; m5 = pd_2048
    jmp                tx2q
.pass2:
    ; m0 = in0 in1
    ; m1 = in2 in3
    ; m5 = pd_2048
    mova                 m4, [o(pw_1697x8)]
    movq                 m2, [dstq+strideq*0]
    movhps               m2, [dstq+strideq*1]
    lea                  r5, [dstq+strideq*2]
    pmulhrsw             m3, m4, m0
    pmulhrsw             m4, m1
    paddsw               m0, m3
    paddsw               m1, m4
    movq                 m3, [r5  +strideq*0]
    movhps               m3, [r5  +strideq*1]
    mova                 m4, [o(pixel_10bpc_max)]
    packssdw             m5, m5 ; pw_2048
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    pxor                 m5, m5
    mova          [cq+16*0], m5
    mova          [cq+16*1], m5
    mova          [cq+16*2], m5
    mova          [cq+16*3], m5
    paddw                m0, m2
    paddw                m1, m3
    pmaxsw               m0, m5
    pmaxsw               m1, m5
    pminsw               m0, m4
    pminsw               m1, m4
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    movq   [r5  +strideq*0], m1
    movhps [r5  +strideq*1], m1
    RET

%macro INV_TXFM_4X8_FN 2-3 0 ; type1, type2, eob_offset
    INV_TXFM_FN          %1, %2, %3, 4x8
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 8
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
    jmp m(inv_txfm_add_dct_dct_4x4_16bpc).dconly
%endif
%endmacro

INV_TXFM_4X8_FN dct, dct
INV_TXFM_4X8_FN dct, identity, 9
INV_TXFM_4X8_FN dct, adst
INV_TXFM_4X8_FN dct, flipadst

cglobal idct_4x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%undef cmp
    mova                 m5, [o(pd_2048)]
%if ARCH_X86_64
    xor                 r5d, r5d
    cmp                eobd, 13
    setge               r5b
%else
    mov                 r5d, 1
    cmp                eobd, 13
    sbb                 r5d, 0
%endif
    shl                 r5d, 4
.loop_pass1:
    mova                 m3, [o(pd_2896)]
    pmulld               m0, m3, [cq+32*0+r5]
    pmulld               m1, m3, [cq+32*1+r5]
    pmulld               m2, m3, [cq+32*2+r5]
    pmulld               m3, [cq+32*3+r5]
    REPX      {paddd x, m5}, m0, m1, m2, m3
    REPX      {psrad x, 12}, m0, m1, m2, m3
    call m(idct_4x4_internal_16bpc).pass1_main
    packssdw             m0, m1     ; out0 out1
    packssdw             m4, m2     ; out2 out3
    test                r5d, r5d
    jz .end_pass1
    mova       [cq+32*0+16], m0
    mova       [cq+32*1+16], m4
    xor                 r5d, r5d
    jmp .loop_pass1
.end_pass1:
    punpckhwd            m2, m0, m4
    punpcklwd            m0, m4
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    mova                 m2, [cq+32*0+16]
    mova                 m6, [cq+32*1+16]
    punpckhwd            m4, m2, m6
    punpcklwd            m2, m6
    punpckhwd            m3, m2, m4
    punpcklwd            m2, m4
    ; m0-3 = packed & transposed output
    jmp                tx2q
.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(idct_4x8_internal_8bpc, _ssse3).main
    ; m0-3 is now out0/1,3/2,4/5,7/6
    mova                 m4, [o(pw_2048)]
    shufps               m1, m1, q1032
    shufps               m3, m3, q1032
.end:
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
    pxor                 m4, m4
    REPX {mova [cq+16*x], m4}, 0, 1, 2, 3, 4, 5, 6, 7
    mova                 m7, [o(pixel_10bpc_max)]
    lea                  r2, [strideq*3]
    movq                 m5, [dstq+strideq*0]
    movq                 m6, [dstq+strideq*2]
    movhps               m5, [dstq+strideq*1]
    movhps               m6, [dstq+r2]
    lea                  r4, [dstq+strideq*4]
    paddw                m0, m5
    paddw                m1, m6
    movq                 m5, [r4+strideq*0]
    movq                 m6, [r4+strideq*2]
    movhps               m5, [r4+strideq*1]
    movhps               m6, [r4+r2]
    paddw                m2, m5
    paddw                m3, m6
    REPX     {pminsw x, m7}, m0, m1, m2, m3
    REPX     {pmaxsw x, m4}, m0, m1, m2, m3
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    movq   [dstq+strideq*2], m1
    movhps [dstq+r2       ], m1
    movq   [r4  +strideq*0], m2
    movhps [r4  +strideq*1], m2
    movq   [r4  +strideq*2], m3
    movhps [r4  +r2       ], m3
    RET

INV_TXFM_4X8_FN adst, dct
INV_TXFM_4X8_FN adst, adst
INV_TXFM_4X8_FN adst, flipadst
INV_TXFM_4X8_FN adst, identity, 9

cglobal iadst_4x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    call .pass1_main
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    mova                 m2, [cq+32*2+16]
    mova                 m6, [cq+32*3+16]
    punpckhwd            m4, m2, m6
    punpcklwd            m2, m6
    punpckhwd            m3, m2, m4
    punpcklwd            m2, m4
    ; m0-3 = packed & transposed output
    jmp                tx2q
.pass1_main:
%undef cmp
%if ARCH_X86_64
    xor                 r5d, r5d
    cmp                eobd, 13
    setge               r5b
%else
    mov                 r5d, 1
    cmp                eobd, 13
    sbb                 r5d, 0
%endif
    shl                 r5d, 4
    lea                  r3, [cq+32*1+16]
.loop_pass1:
    mova                 m0, [o(pd_2048)]
    mova                 m3, [o(pd_2896)]
    pmulld               m5, m3, [cq+32*0+r5]
    pmulld               m2, m3, [cq+32*1+r5]
    pmulld               m1, m3, [cq+32*2+r5]
    pmulld               m3, [cq+32*3+r5]
    REPX      {paddd x, m0}, m5, m2, m1, m3
    REPX      {psrad x, 12}, m5, m2, m1, m3
    mova               [r3], m2
    call m(iadst_4x4_internal_16bpc).main2
    packssdw             m0, m2            ; out0 out1
    packssdw             m1, m4            ; out2 out3
    test                r5d, r5d
    jz .end_pass1
    mova       [cq+32*2+16], m0
    mova       [cq+32*3+16], m1
    xor                 r5d, r5d
    jmp .loop_pass1
.end_pass1:
    ret
.pass2:
    shufps               m0, m0, q1032
    shufps               m1, m1, q1032
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_4x8_internal_8bpc, _ssse3).main
    mova                 m4, [o(pw_4x2048_4xm2048)]
    jmp m(idct_4x8_internal_16bpc).end

INV_TXFM_4X8_FN flipadst, dct
INV_TXFM_4X8_FN flipadst, adst
INV_TXFM_4X8_FN flipadst, flipadst
INV_TXFM_4X8_FN flipadst, identity, 9

cglobal iflipadst_4x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    call m(iadst_4x8_internal_16bpc).pass1_main
    punpcklwd            m2, m1, m0
    punpckhwd            m1, m0
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    mova                 m6, [cq+32*2+16]
    mova                 m2, [cq+32*3+16]
    punpcklwd            m4, m2, m6
    punpckhwd            m2, m6
    punpckhwd            m3, m2, m4
    punpcklwd            m2, m4
    ; m0-3 = packed & transposed output
    jmp                tx2q
.pass2:
    shufps               m0, m0, q1032
    shufps               m1, m1, q1032
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_4x8_internal_8bpc, _ssse3).main
    mova                 m4, m0
    mova                 m5, m1
    pshufd               m0, m3, q1032
    pshufd               m1, m2, q1032
    pshufd               m2, m5, q1032
    pshufd               m3, m4, q1032
    mova                 m4, [o(pw_4xm2048_4x2048)]
    jmp m(idct_4x8_internal_16bpc).end

INV_TXFM_4X8_FN identity, dct
INV_TXFM_4X8_FN identity, adst
INV_TXFM_4X8_FN identity, flipadst
INV_TXFM_4X8_FN identity, identity, 3

cglobal iidentity_4x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%undef cmp
    mova                 m5, [o(pd_2048)]
    mova                 m4, [o(pd_2896)]
    mova                 m6, [o(pd_5793)]
    ; clear m7 in case we skip the bottom square
    pxor                 m7, m7
%if ARCH_X86_64
    xor                 r5d, r5d
    cmp                eobd, 16
    setge               r5b
%else
    mov                 r5d, 1
    cmp                eobd, 16
    sbb                 r5d, 0
%endif
    shl                 r5d, 4
.loop_pass1:
    pmulld               m0, m4, [cq+32*0+r5]
    pmulld               m1, m4, [cq+32*1+r5]
    pmulld               m2, m4, [cq+32*2+r5]
    pmulld               m3, m4, [cq+32*3+r5]
    REPX      {paddd x, m5}, m0, m1, m2, m3
    REPX      {psrad x, 12}, m0, m1, m2, m3
    REPX     {pmulld x, m6}, m0, m1, m2, m3
    REPX      {paddd x, m5}, m0, m1, m2, m3
    REPX      {psrad x, 12}, m0, m1, m2, m3
    packssdw             m0, m1
    packssdw             m2, m3
    test                r5d, r5d
    jz .end_pass1
    mova       [cq+32*0+16], m0
    mova                 m7, m2
    xor                 r5d, r5d
    jmp .loop_pass1
.end_pass1:
    punpckhwd            m4, m0, m2
    punpcklwd            m0, m2
    punpckhwd            m1, m0, m4
    punpcklwd            m0, m4
    mova                 m2, [cq+32*0+16]
    punpckhwd            m4, m2, m7
    punpcklwd            m2, m7
    punpckhwd            m3, m2, m4
    punpcklwd            m2, m4
    ; m0-3 = packed & transposed output
    jmp                tx2q
.pass2:
    mova                 m4, [o(pw_4096)]
    jmp m(idct_4x8_internal_16bpc).end

%macro INV_TXFM_4X16_FN 2-3 2d ; type1, type2, eob_tbl_suffix
    INV_TXFM_FN          %1, %2, tbl_4x16_%3, 4x16
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 16
    add                 r5d, 384
    sar                 r5d, 9
    jmp m(inv_txfm_add_dct_dct_4x4_16bpc).dconly2
%endif
%endmacro

INV_TXFM_4X16_FN dct, dct
INV_TXFM_4X16_FN dct, identity, v
INV_TXFM_4X16_FN dct, adst
INV_TXFM_4X16_FN dct, flipadst

cglobal idct_4x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%undef cmp
%if ARCH_X86_32
    mov                 r5m, r6d
%endif
    mov                 r6d, 4
.zero_loop:
    dec                 r6d
    cmp                eobb, byte [r5+r6]
    jl .zero_loop
    mov                 r5d, r6d
    shl                 r5d, 4
%if ARCH_X86_32
    ; restore pic-ptr
    mov                  r6, r5m
%endif
    mova                 m5, [o(pd_2048)]
.loop_pass1:
    mova                 m0, [cq+64*0+r5]
    mova                 m1, [cq+64*1+r5]
    mova                 m2, [cq+64*2+r5]
    mova                 m3, [cq+64*3+r5]
    call m(idct_4x4_internal_16bpc).pass1_main
    pcmpeqd              m3, m3
    REPX      {psubd x, m3}, m0, m1, m4, m2
    REPX       {psrad x, 1}, m0, m1, m4, m2
    packssdw             m0, m1     ; out0 out1
    packssdw             m4, m2     ; out2 out3
    punpckhwd            m2, m0, m4
    punpcklwd            m0, m4
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    test                r5d, r5d
    jz .end_pass1
    mova       [cq+64*0+r5], m0
    mova       [cq+64*1+r5], m1
    sub                 r5d, 16
    jmp .loop_pass1
.end_pass1:
    mova                 m2, [cq+64*0+16]
    mova                 m3, [cq+64*1+16]
    mova                 m4, [cq+64*0+32]
    mova                 m5, [cq+64*1+32]
    mova                 m6, [cq+64*0+48]
    mova                 m7, [cq+64*1+48]
    ; m0-7 = packed & transposed output
    jmp                tx2q
.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(idct_16x4_internal_8bpc, _ssse3).main
    ; m0-6 is out0-13 [with odd registers having inversed output]
    ; [coeffq+16*7] has out15/14
    mova                 m7, [o(pw_2048)]
    REPX   {pmulhrsw x, m7}, m0, m1, m2, m3, m4, m5, m6
    pmulhrsw             m7, [cq+16*7]
    REPX {shufps x, x, q1032}, m1, m3, m5, m7
    mova          [cq+16*0], m4
    mova          [cq+16*1], m5
    mova          [cq+16*2], m6
    mova          [cq+16*3], m7
.end:
    pxor                 m4, m4
    REPX {mova [cq+16*x], m4}, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    mova                 m7, [o(pixel_10bpc_max)]
    mov                 r5d, 2
    lea                  r3, [strideq*3]
.loop:
    movq                 m5, [dstq+strideq*0]
    movq                 m6, [dstq+strideq*2]
    movhps               m5, [dstq+strideq*1]
    movhps               m6, [dstq+r3]
    lea                  r4, [dstq+strideq*4]
    paddw                m0, m5
    paddw                m1, m6
    movq                 m5, [r4+strideq*0]
    movq                 m6, [r4+strideq*2]
    movhps               m5, [r4+strideq*1]
    movhps               m6, [r4+r3]
    paddw                m2, m5
    paddw                m3, m6
    REPX     {pminsw x, m7}, m0, m1, m2, m3
    REPX     {pmaxsw x, m4}, m0, m1, m2, m3
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    movq   [dstq+strideq*2], m1
    movhps [dstq+r3       ], m1
    movq   [r4  +strideq*0], m2
    movhps [r4  +strideq*1], m2
    movq   [r4  +strideq*2], m3
    movhps [r4  +r3       ], m3
    dec                 r5d
    jz .end2
    lea                dstq, [dstq+strideq*8]
    mova                 m0, [cq+0*16]
    mova                 m1, [cq+1*16]
    mova                 m2, [cq+2*16]
    mova                 m3, [cq+3*16]
    REPX {mova [cq+x*16], m4}, 0, 1, 2, 3
    jmp .loop
.end2:
    RET

INV_TXFM_4X16_FN adst, dct
INV_TXFM_4X16_FN adst, adst
INV_TXFM_4X16_FN adst, flipadst
INV_TXFM_4X16_FN adst, identity, v

cglobal iadst_4x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%undef cmp
%if ARCH_X86_32
    mov                 r5m, r6d
%endif
    mov                 r6d, 4
.zero_loop:
    dec                 r6d
    cmp                eobb, byte [r6+r5]
    jl .zero_loop
    mov                 r5d, r6d
    shl                 r5d, 4
%if ARCH_X86_32
    ; restore pic-ptr
    mov                  r6, r5m
%endif
.loop_pass1:
    mova                 m5, [cq+64*0+r5]
    lea                  r3, [cq+64*1+r5]
    mova                 m1, [cq+64*2+r5]
    mova                 m3, [cq+64*3+r5]
    call m(iadst_4x4_internal_16bpc).main2
    pcmpeqd              m3, m3
    REPX      {psubd x, m3}, m0, m2, m1, m4
    REPX       {psrad x, 1}, m0, m2, m1, m4
    packssdw             m0, m2            ; out0 out1
    packssdw             m1, m4            ; out2 out3
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    test                r5d, r5d
    jz m(idct_4x16_internal_16bpc).end_pass1
    mova       [cq+64*0+r5], m0
    mova       [cq+64*1+r5], m1
    sub                 r5d, 16
    jmp .loop_pass1
.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_16x4_internal_8bpc, _ssse3).main
    call m_suffix(iadst_16x4_internal_8bpc, _ssse3).main_pass2_end
    ; m7/5/2/4 = out4/-11,-5/10,6/-9,-7/8
    ; m0/3 & cq6/7 = out0/-15,-3/12,-1/14,2/-13
    mova                 m1, [o(pw_4x2048_4xm2048)]
    REPX   {pmulhrsw x, m1}, m7, m2, m0
    pshufd               m6, m1, q1032  ; 4x-2048,4x2048
    pmulhrsw             m1, [cq+16*7]
    REPX   {pmulhrsw x, m6}, m5, m4, m3
    pmulhrsw             m6, [cq+16*6]
    ; m7/5/2/4 = out4/11,5/10,6/9,7/8
    ; m0/3/6/1 = out0/15,3/12,1/14,2/13
    ; output should be as 0-3 for out0-7, and cq+0-3*16 for out8-15
    movhps         [cq+0*8], m4
    movhps         [cq+1*8], m2
    movhps         [cq+2*8], m5
    movhps         [cq+3*8], m7
    movhps         [cq+4*8], m3
    movhps         [cq+5*8], m1
    movhps         [cq+6*8], m6
    movhps         [cq+7*8], m0
    punpcklqdq           m0, m6
    punpcklqdq           m1, m3
    punpcklqdq           m3, m2, m4
    punpcklqdq           m2, m7, m5
    jmp m(idct_4x16_internal_16bpc).end

INV_TXFM_4X16_FN flipadst, dct
INV_TXFM_4X16_FN flipadst, adst
INV_TXFM_4X16_FN flipadst, flipadst
INV_TXFM_4X16_FN flipadst, identity, v

cglobal iflipadst_4x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%undef cmp
%if ARCH_X86_32
    mov                 r5m, r6d
%endif
    mov                 r6d, 4
.zero_loop:
    dec                 r6d
    cmp                eobb, byte [r5+r6]
    jl .zero_loop
    mov                 r5d, r6d
    shl                 r5d, 4
%if ARCH_X86_32
    ; restore pic-ptr
    mov                  r6, r5m
%endif
.loop_pass1:
    mova                 m5, [cq+64*0+r5]
    lea                  r3, [cq+64*1+r5]
    mova                 m1, [cq+64*2+r5]
    mova                 m3, [cq+64*3+r5]
    call m(iadst_4x4_internal_16bpc).main2
    pcmpeqd              m3, m3
    REPX      {psubd x, m3}, m0, m2, m1, m4
    REPX       {psrad x, 1}, m0, m2, m1, m4
    packssdw             m0, m2            ; out3 out2
    packssdw             m1, m4            ; out1 out0
    punpcklwd            m2, m1, m0
    punpckhwd            m1, m0
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    test                r5d, r5d
    jz m(idct_4x16_internal_16bpc).end_pass1
    mova       [cq+64*0+r5], m0
    mova       [cq+64*1+r5], m1
    sub                 r5d, 16
    jmp .loop_pass1
.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_16x4_internal_8bpc, _ssse3).main
    call m_suffix(iadst_16x4_internal_8bpc, _ssse3).main_pass2_end
    ; m7/5/2/4 = out11/-4,-10/5,9/-6,-8/7
    ; m0/3 & cq6/7 = out15/-0,-12/3,-14/1,13/-2
    mova                 m1, [o(pw_4x2048_4xm2048)]
    REPX   {pmulhrsw x, m1}, m7, m2, m0
    pshufd               m6, m1, q1032  ; 4x-2048,4x2048
    pmulhrsw             m1, [cq+16*7]
    REPX   {pmulhrsw x, m6}, m5, m4, m3
    pmulhrsw             m6, [cq+16*6]
    ; m7/5/2/4 = out11/4,10/5,9/6,8/7
    ; m0/3/6/1 = out15/0,12/3,14/1,13/2
    ; output should be as 0-3 for out0-7, and cq+0-3*16 for out8-15
    movq           [cq+0*8], m4
    movq           [cq+1*8], m2
    movq           [cq+2*8], m5
    movq           [cq+3*8], m7
    movq           [cq+4*8], m3
    movq           [cq+5*8], m1
    movq           [cq+6*8], m6
    movq           [cq+7*8], m0
    punpckhqdq           m0, m6
    punpckhqdq           m1, m3
    punpckhqdq           m3, m2, m4
    punpckhqdq           m2, m7, m5
    jmp m(idct_4x16_internal_16bpc).end

INV_TXFM_4X16_FN identity, dct, h
INV_TXFM_4X16_FN identity, adst, h
INV_TXFM_4X16_FN identity, flipadst, h
INV_TXFM_4X16_FN identity, identity

cglobal iidentity_4x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%undef cmp
%if ARCH_X86_32
    mov                 r5m, r6d
%endif
    mov                 r6d, 4
.zero_loop:
    dec                 r6d
    cmp                eobb, byte [r5+r6]
    jl .zero_loop
    mov                 r5d, r6d
    shl                 r5d, 4
%if ARCH_X86_32
    ; restore pic-ptr
    mov                  r6, r5m
%endif
    mova                 m5, [o(pd_6144)]
    mova                 m4, [o(pd_5793)]
.loop_pass1:
    pmulld               m0, m4, [cq+64*0+r5]
    pmulld               m1, m4, [cq+64*1+r5]
    pmulld               m2, m4, [cq+64*2+r5]
    pmulld               m3, m4, [cq+64*3+r5]
    REPX      {paddd x, m5}, m0, m1, m2, m3
    REPX      {psrad x, 13}, m0, m1, m2, m3
    packssdw             m0, m1
    packssdw             m2, m3
    punpckhwd            m3, m0, m2
    punpcklwd            m0, m2
    punpckhwd            m1, m0, m3
    punpcklwd            m0, m3
    test                r5d, r5d
    jz m(idct_4x16_internal_16bpc).end_pass1
    mova       [cq+64*0+r5], m0
    mova       [cq+64*1+r5], m1
    sub                 r5d, 16
    jmp .loop_pass1
.pass2:
    mova          [cq+16*4], m0
    mova          [cq+16*5], m1
    mova          [cq+16*6], m2
    mova          [cq+16*7], m7
    mova                 m0, [o(pw_1697x16)]
    mova                 m7, [o(pw_2048)]
    pmulhrsw             m1, m0, m4
    pmulhrsw             m2, m0, m5
    REPX      {paddsw x, x}, m4, m5
    paddsw               m4, m1
    paddsw               m5, m2
    REPX   {pmulhrsw x, m7}, m4, m5
    mova          [cq+16*0], m4
    mova          [cq+16*1], m5
    mova                 m4, [cq+16*7]
    pmulhrsw             m1, m0, m6
    pmulhrsw             m2, m0, m4
    REPX      {paddsw x, x}, m6, m4
    paddsw               m6, m1
    paddsw               m4, m2
    REPX   {pmulhrsw x, m7}, m6, m4
    mova          [cq+16*2], m6
    mova          [cq+16*3], m4
    mova                 m4, [cq+16*4]
    mova                 m1, [cq+16*5]
    mova                 m2, [cq+16*6]
    pmulhrsw             m5, m0, m2
    pmulhrsw             m6, m0, m3
    REPX      {paddsw x, x}, m2, m3
    paddsw               m2, m5
    paddsw               m3, m6
    pmulhrsw             m6, m0, m1
    pmulhrsw             m0, m4
    REPX      {paddsw x, x}, m1, m4
    paddsw               m1, m6
    paddsw               m0, m4
    REPX   {pmulhrsw x, m7}, m2, m3, m1, m0
    jmp m(idct_4x16_internal_16bpc).end

%macro INV_TXFM_8X4_FN 2 ; type1, type2
%if ARCH_X86_64
    INV_TXFM_FN          %1, %2, 0, 8x4, 15
%else
    INV_TXFM_FN          %1, %2, 0, 8x4, 8, 0-4*16
%endif
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 2896
    add                 r5d, 34816
    movd                 m0, r5d
    pshuflw              m0, m0, q1111
    punpcklqdq           m0, m0
    mova                 m6, [o(pixel_10bpc_max)]
    pxor                 m5, m5
    lea                  r2, [strideq*3]
    mova                 m1, [dstq+strideq*0]
    mova                 m2, [dstq+strideq*1]
    mova                 m3, [dstq+strideq*2]
    mova                 m4, [dstq+r2]
    REPX      {paddw x, m0}, m1, m2, m3, m4
    REPX     {pmaxsw x, m5}, m1, m2, m3, m4
    REPX     {pminsw x, m6}, m1, m2, m3, m4
    mova   [dstq+strideq*0], m1
    mova   [dstq+strideq*1], m2
    mova   [dstq+strideq*2], m3
    mova   [dstq+r2       ], m4
    RET
%endif
%endmacro

INV_TXFM_8X4_FN dct, dct
INV_TXFM_8X4_FN dct, identity
INV_TXFM_8X4_FN dct, adst
INV_TXFM_8X4_FN dct, flipadst

cglobal idct_8x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    lea                  r5, [o(.main)]
.pass1_entry:
%if ARCH_X86_32
    lea                  r3, [rsp+gprsize]
%else
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+0*16]
    mova                 m1, [cq+1*16]
    mova                 m2, [cq+2*16]
    mova                 m3, [cq+3*16]
    mova                 m4, [cq+4*16]
    mova                 m5, [cq+5*16]
    mova                 m6, [cq+6*16]
    mova                 m7, [cq+7*16]
    call .rect2_mul
    call                 r5
    call .transpose4x8packed
    ; m0-3 = packed & transposed output
    jmp                tx2q
.transpose4x8packed:
    ; transpose
    punpcklwd            m1, m2, m6
    punpckhwd            m2, m6
    punpckhwd            m6, m0, m4
    punpcklwd            m0, m4

    punpckhwd            m3, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m4, m6, m2
    punpcklwd            m6, m2

    punpcklwd            m2, m3, m4
    punpckhwd            m3, m4
    punpckhwd            m1, m0, m6
    punpcklwd            m0, m6
    ret
.main:
    call .main_pass1
    call .round
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    ret
.rect2_mul:
%if ARCH_X86_64
    REPX    {pmulld x, m14}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {paddd  x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
%else
    mova               [r3], m7
    mova                 m7, [o(pd_2896)]
    REPX     {pmulld x, m7}, m0, m1, m2, m3, m4, m5, m6
    pmulld               m7, [r3]
    mova               [r3], m7
    mova                 m7, [o(pd_2048)]
    REPX      {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6
    paddd                m7, [r3]
%endif
    REPX      {psrad x, 12}, m0, m1, m2, m3, m4, m5, m6, m7
    ret
%if ARCH_X86_64
.main_pass1_fast:
    pmulld               m5, m3, [o(pd_m2276)]
    pmulld               m3, [o(pd_3406)]
    pmulld               m7, m1, [o(pd_4017)]
    pmulld               m1, [o(pd_799)]
    pmulld               m6, m2, [o(pd_3784)]
    pmulld               m2, [o(pd_1567)]
    pmulld               m0, m14
    pxor                 m4, m4
    jmp .main_pass1_fast2
.main_pass1:
    ITX_MULSUB_2D         5, 3, 8, 9, 10, _, 3406, 2276 ; t5a t6a
    ITX_MULSUB_2D         1, 7, 8, 9, 10, _,  799, 4017 ; t4a t7a
    ITX_MULSUB_2D         2, 6, 8, 9, 10, _, 1567, 3784 ; t2  t3
    REPX    {pmulld x, m14}, m0, m4
.main_pass1_fast2:
    REPX     {paddd x, m11}, m1, m2, m3, m5, m6, m7
    REPX     {psrad x, 12 }, m1, m2, m3, m5, m6, m7
    paddd                m8, m1, m5 ; t4
    psubd                m1, m5     ; t5a
    paddd                m9, m7, m3 ; t7
    psubd                m7, m3     ; t6a
    REPX    {pmaxsd x, m12}, m1, m8, m7, m9
    REPX    {pminsd x, m13}, m1, m8, m7, m9
    REPX    {pmulld x, m14}, m7, m1
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
.round:
    paddd                m1, m6, m7 ; out1
    psubd                m6, m7     ; out6
    psubd                m7, m0, m9 ; out7
    paddd                m0, m9     ; out0
    paddd                m2, m5, m4 ; out2
    psubd                m5, m4     ; out5
    psubd                m4, m3, m8 ; out4
    paddd                m3, m8     ; out3
%else
.main_pass1_fast:
    pmulld               m5, m3, [o(pd_m2276)]
    pmulld               m3, [o(pd_3406)]
    pmulld               m7, m1, [o(pd_4017)]
    pmulld               m1, [o(pd_799)]
    pmulld               m6, m2, [o(pd_3784)]
    pmulld               m2, [o(pd_1567)]
    mova                 m4, [o(pd_2048)]
    mova          [r3+0*16], m2
    REPX      {paddd x, m4}, m5, m3, m7, m1
    REPX      {psrad x, 12}, m5, m3, m7, m1
    paddd                m2, m1, m5 ; t4
    psubd                m1, m5     ; t5a
    pmulld               m5, m0, [o(pd_2896)]
    mova                 m0, m4
    paddd                m4, m7, m3 ; t7
    psubd                m7, m3     ; t6a
    mova                 m3, [o(clip_18b_min)]
    REPX    {pmaxsd x, m3 }, m1, m2, m7, m4
    mova                 m3, [o(clip_18b_max)]
    REPX    {pminsd x, m3 }, m1, m2, m7, m4
    mova          [r3+3*16], m2
    mova          [r3+1*16], m4
    pxor                 m4, m4
    mova                 m2, [r3+0*16]
    mova                 m3, [o(pd_2896)]
    jmp .main_pass1_fast2
.main_pass1:
    mova          [r3+0*16], m0
    mova          [r3+1*16], m2
    mova          [r3+2*16], m4
    mova          [r3+3*16], m6
    mova                 m0, [o(pd_2048)]
    ITX_MULSUB_2D         5, 3, 2, 4, 6, 0, 3406, 2276 ; t5a t6a
    ITX_MULSUB_2D         1, 7, 2, 4, 6, 0,  799, 4017 ; t4a t7a
    paddd                m2, m1, m5 ; t4
    psubd                m1, m5     ; t5a
    paddd                m4, m7, m3 ; t7
    psubd                m7, m3     ; t6a
    mova                 m6, [o(clip_18b_min)]
    REPX    {pmaxsd x, m6 }, m1, m2, m7, m4
    mova                 m6, [o(clip_18b_max)]
    REPX    {pminsd x, m6 }, m1, m2, m7, m4
    mova                 m6, [r3+3*16]
    mova          [r3+3*16], m2
    mova                 m2, [r3+1*16]
    mova          [r3+1*16], m4

    ITX_MULSUB_2D         2, 6, 4, 3, 5, _, 1567, 3784 ; t2  t3
    mova                 m3, [o(pd_2896)]
    mova                 m5, [r3+0*16]
    mova                 m4, [r3+2*16]
    REPX    {pmulld x, m3 }, m5, m4
.main_pass1_fast2:
    REPX    {paddd  x, m0 }, m2, m6
    REPX    {psrad  x, 12 }, m2, m6
    REPX    {pmulld x, m3 }, m7, m1
    paddd                m7, m0
    paddd                m0, m5

    psubd                m5, m0, m4
    paddd                m0, m4
    psubd                m4, m7, m1
    paddd                m7, m1
    REPX    {psrad  x, 12 }, m5, m0, m4, m7
    psubd                m3, m0, m6 ; dct4 out3
    paddd                m0, m6     ; dct4 out0
    paddd                m6, m5, m2 ; dct4 out1
    psubd                m5, m2     ; dct4 out2

    mova                 m1, [o(clip_18b_min)]
    REPX    {pmaxsd x, m1 }, m0, m6, m5, m3
    mova                 m1, [o(clip_18b_max)]
    REPX    {pminsd x, m1 }, m0, m6, m5, m3
    ret
.round:
    paddd                m1, m6, m7 ; out1
    psubd                m6, m7     ; out6
    mova          [r3+0*16], m6
    mova                 m6, [r3+1*16]
    psubd                m7, m0, m6 ; out7
    paddd                m0, m6     ; out0
    paddd                m2, m5, m4 ; out2
    psubd                m5, m4     ; out5
    mova                 m6, [r3+3*16]
    psubd                m4, m3, m6 ; out4
    paddd                m3, m6     ; out3
    mova                 m6, [r3+0*16]
%endif
    ret

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(idct_8x4_internal_8bpc, _ssse3).main
.end:
    lea                  r3, [strideq*3]
    call .round2_and_write_8x4
    REPX {mova [cq+16*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
    RET
.round2_and_write_8x4:
    pxor                 m6, m6
    mova                 m5, [o(pixel_10bpc_max)]
    mova                 m4, [o(pw_2048)]
.round1_and_write_8x4:
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
.write_8x4:
    paddw                m0, [dstq+strideq*0]
    paddw                m1, [dstq+strideq*1]
    paddw                m2, [dstq+strideq*2]
    paddw                m3, [dstq+r3]
    REPX     {pminsw x, m5}, m0, m1, m2, m3
    REPX     {pmaxsw x, m6}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+r3       ], m3
    ret

INV_TXFM_8X4_FN adst, dct
INV_TXFM_8X4_FN adst, adst
INV_TXFM_8X4_FN adst, flipadst
INV_TXFM_8X4_FN adst, identity

cglobal iadst_8x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    lea                  r5, [o(.main)]
    jmp m(idct_8x4_internal_16bpc).pass1_entry
.main:
    call .main_pass1
    call .round
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    ret
.main_pass1:
%if ARCH_X86_64
    ITX_MULSUB_2D         7, 0, 8, 9, 10, 11,  401, 4076 ; t1a, t0a
    ITX_MULSUB_2D         1, 6, 8, 9, 10, 11, 3920, 1189 ; t7a, t6a
    ITX_MULSUB_2D         5, 2, 8, 9, 10, 11, 1931, 3612 ; t3a, t2a
    ITX_MULSUB_2D         3, 4, 8, 9, 10, 11, 3166, 2598 ; t5a, t4a
    psubd                m8, m2, m6 ; t6
    paddd                m2, m6     ; t2
    psubd                m6, m0, m4 ; t4
    paddd                m0, m4     ; t0
    psubd                m4, m5, m1 ; t7
    paddd                m5, m1     ; t3
    psubd                m1, m7, m3 ; t5
    paddd                m7, m3     ; t1
    REPX    {pmaxsd x, m12}, m6, m1, m8, m4, m2, m0, m5, m7
    REPX    {pminsd x, m13}, m6, m1, m8, m4, m2, m0, m5, m7
    ITX_MULSUB_2D         6, 1, 3, 9, 10, 11, 1567, 3784 ; t5a, t4a
    ITX_MULSUB_2D         4, 8, 3, 9, 10, 11, 3784, 10   ; t6a, t7a
    psubd                m9, m6, m8 ;  t7
    paddd                m6, m8     ;  out6
    mova                 m8, [o(pd_2896)]
    psubd                m3, m7, m5 ;  t3
    paddd                m7, m5     ; -out7
    psubd                m5, m0, m2 ;  t2
    paddd                m0, m2     ;  out0
    psubd                m2, m1, m4 ;  t6
    paddd                m1, m4     ; -out1
    REPX    {pmaxsd x, m12}, m5, m3, m2, m9
    REPX    {pminsd x, m13}, m5, m3, m2, m9
    REPX    {pmulld x, m14}, m5, m3, m2, m9
    psubd               m4, m5, m3 ; (t2 - t3) * 2896
    paddd               m3, m5     ; (t2 + t3) * 2896
    psubd               m5, m2, m9 ; (t6 - t7) * 2896
    paddd               m2, m9     ; (t6 + t7) * 2896
    ret
.round:

    ; m0=out0,m1=-out1,m6=out6,m7=-out7

    pcmpeqd              m8, m8
    REPX     {pxor  x, m8 }, m1, m7, m3, m5
    REPX     {psubd x, m8 }, m1, m7
    REPX     {paddd x, m11}, m2, m3, m4, m5
    REPX     {psrad x, 12 }, m2, m3, m4, m5
%else
    mova          [r3+0*16], m2
    mova          [r3+1*16], m3
    mova          [r3+2*16], m4
    mova          [r3+3*16], m5
    mova                 m5, [o(pd_2048)]

    ITX_MULSUB_2D         7, 0, 2, 3, 4, 5,  401, 4076 ; t1a, t0a
    ITX_MULSUB_2D         1, 6, 2, 3, 4, 5, 3920, 1189 ; t7a, t6a
    mova                 m2, [r3+0*16]
    mova                 m3, [r3+1*16]
    mova                 m4, [r3+2*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m1
    mova          [r3+2*16], m6
    mova                 m1, [r3+3*16]
    mova          [r3+3*16], m7
    ITX_MULSUB_2D         1, 2, 0, 6, 7, 5, 1931, 3612 ; t3a, t2a
    ITX_MULSUB_2D         3, 4, 0, 6, 7, 5, 3166, 2598 ; t5a, t4a
    mova                 m0, [r3+0*16]
    mova                 m6, [r3+2*16]
    psubd                m7, m2, m6 ; t6
    paddd                m2, m6     ; t2
    psubd                m6, m0, m4 ; t4
    paddd                m0, m4     ; t0
    mova          [r3+0*16], m7
    mova                 m5, [r3+1*16]
    mova                 m7, [r3+3*16]
    psubd                m4, m1, m5 ; t7
    paddd                m5, m1     ; t3
    psubd                m1, m7, m3 ; t5
    paddd                m7, m3     ; t1
    mova                 m3, [o(clip_18b_min)]
    REPX    {pmaxsd x, m3 }, m6, m1, m4, m2, m0, m5, m7
    mova          [r3+1*16], m7
    mova                 m7, [o(clip_18b_max)]
    pmaxsd               m3, [r3+0*16]
    REPX    {pminsd x, m7 }, m6, m1, m3, m4, m2, m0, m5
    pminsd               m7, [r3+1*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m2
    mova          [r3+2*16], m5
    mova          [r3+3*16], m7
    mova                 m0, [o(pd_2048)]
    ITX_MULSUB_2D         6, 1, 2, 5, 7, 0, 1567, 3784 ; t5a, t4a
    ITX_MULSUB_2D         4, 3, 2, 5, 7, 0, 3784, 7    ; t6a, t7a
    mova                 m5, [r3+2*16]
    mova                 m7, [r3+3*16]
    psubd                m2, m6, m3 ;  t7
    paddd                m6, m3     ;  out6
    mova          [r3+3*16], m6
    mova                 m0, [r3+0*16]
    mova                 m6, [r3+1*16]
    psubd                m3, m7, m5 ;  t3
    paddd                m7, m5     ; -out7
    psubd                m5, m0, m6 ;  t2
    paddd                m0, m6     ;  out0
    psubd                m6, m1, m4 ;  t6
    paddd                m1, m4     ; -out1
    mova                 m4, [o(clip_18b_min)]
    REPX    {pmaxsd x, m4 }, m5, m3, m6, m2
    mova                 m4, [o(clip_18b_max)]
    REPX    {pminsd x, m4 }, m5, m3, m6, m2
    mova                 m4, [o(pd_2896)]
    REPX    {pmulld x, m4 }, m5, m3, m6, m2
    psubd               m4, m5, m3 ; (t2 - t3) * 2896
    paddd               m3, m5     ; (t2 + t3) * 2896
    psubd               m5, m6, m2 ; (t6 - t7) * 2896
    paddd               m2, m6     ; (t6 + t7) * 2896
    ret
.round:
    mova          [r3+2*16], m0

    pcmpeqd              m0, m0
    mova                 m6, [o(pd_2048)]
    REPX     {pxor  x, m0 }, m1, m7, m3, m5
    REPX     {psubd x, m0 }, m1, m7
    REPX     {paddd x, m6 }, m2, m3, m4, m5
    REPX     {psrad x, 12 }, m2, m3, m4, m5

    mova                 m6, [r3+3*16]
    mova                 m0, [r3+2*16]
%endif
    ret

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_8x4_internal_8bpc, _ssse3).main
    jmp m(idct_8x4_internal_16bpc).end

INV_TXFM_8X4_FN flipadst, dct
INV_TXFM_8X4_FN flipadst, adst
INV_TXFM_8X4_FN flipadst, flipadst
INV_TXFM_8X4_FN flipadst, identity

cglobal iflipadst_8x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    lea                  r5, [o(.main)]
    jmp m(idct_8x4_internal_16bpc).pass1_entry
.main:
    call m(iadst_8x4_internal_16bpc).main_pass1
    call m(iadst_8x4_internal_16bpc).round
    packssdw             m7, m6
    packssdw             m5, m4
    packssdw             m3, m2
    packssdw             m1, m0
    mova                 m0, m7
    mova                 m2, m5
    mova                 m4, m3
    mova                 m6, m1
    ret
.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_8x4_internal_8bpc, _ssse3).main
    lea                  r3, [strideq*3]
    add                dstq, r3
    neg             strideq
    jmp m(idct_8x4_internal_16bpc).end

INV_TXFM_8X4_FN identity, dct
INV_TXFM_8X4_FN identity, adst
INV_TXFM_8X4_FN identity, flipadst
INV_TXFM_8X4_FN identity, identity

cglobal iidentity_8x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    lea                  r5, [o(.main)]
    jmp m(idct_8x4_internal_16bpc).pass1_entry
.main:
    REPX       {paddd x, x}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    ret
.pass2:
    mova                 m7, [o(pw_1697x8)]
    pmulhrsw             m4, m7, m0
    pmulhrsw             m5, m7, m1
    pmulhrsw             m6, m7, m2
    pmulhrsw             m7, m3
    paddsw               m0, m4
    paddsw               m1, m5
    paddsw               m2, m6
    paddsw               m3, m7
    jmp m(idct_8x4_internal_16bpc).end

%macro INV_TXFM_8X8_FN 2-3 0 ; type1, type2, eob_offset
%if ARCH_X86_64
    INV_TXFM_FN          %1, %2, %3, 8x8, 15, 0-3*16
%else
    INV_TXFM_FN          %1, %2, %3, 8x8, 8, 0-5*16
%endif
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 2
.end:
    add                 r5d, 384
    sar                 r5d, 9
.end2:
    imul                r5d, 2896
    add                 r5d, 34816
    movd                 m0, r5d
    pshuflw              m0, m0, q1111
    punpcklqdq           m0, m0
    mova                 m6, [o(pixel_10bpc_max)]
    pxor                 m5, m5
    lea                  r2, [strideq*3]
.loop:
    mova                 m1, [dstq+strideq*0]
    mova                 m2, [dstq+strideq*1]
    mova                 m3, [dstq+strideq*2]
    mova                 m4, [dstq+r2]
    REPX      {paddw x, m0}, m1, m2, m3, m4
    REPX     {pmaxsw x, m5}, m1, m2, m3, m4
    REPX     {pminsw x, m6}, m1, m2, m3, m4
    mova   [dstq+strideq*0], m1
    mova   [dstq+strideq*1], m2
    mova   [dstq+strideq*2], m3
    mova   [dstq+r2       ], m4
    lea                dstq, [dstq+strideq*4]
    dec                 r3d
    jg .loop
    RET
%endif
%endmacro

INV_TXFM_8X8_FN dct, dct
INV_TXFM_8X8_FN dct, identity, 6
INV_TXFM_8X8_FN dct, adst
INV_TXFM_8X8_FN dct, flipadst

cglobal idct_8x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_32
    DECLARE_REG_TMP 1
    mov [rsp+4*16+1*gprsize], r1
%else
    DECLARE_REG_TMP 6
%endif
    lea                  t0, [o(.pass1_main)]

.pass1_full:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
%undef cmp
%if ARCH_X86_64
    xor                 r5d, r5d
    cmp                eobd, 10
    setge               r5b
%else
    mov                 r5d, 1
    cmp                eobd, 10
    sbb                 r5d, 0
%endif
    shl                 r5d, 4
%if ARCH_X86_32
    lea                  r3, [rsp+gprsize]
%endif
.loop_pass1:
    mova                 m0, [cq+0*32+r5]
    mova                 m1, [cq+1*32+r5]
    mova                 m2, [cq+2*32+r5]
    mova                 m3, [cq+3*32+r5]
    mova                 m4, [cq+4*32+r5]
    mova                 m5, [cq+5*32+r5]
    mova                 m6, [cq+6*32+r5]
    mova                 m7, [cq+7*32+r5]
    call                 t0

    test                r5d, r5d
    jz .end_pass1

    mova       [cq+0*32+16], m0
    mova       [cq+1*32+16], m1
    mova       [cq+2*32+16], m2
    mova       [cq+3*32+16], m3

    sub                 r5d, 16
    jmp .loop_pass1
.end_pass1:
    mova                 m4, [cq+0*32+16]
    mova                 m5, [cq+1*32+16]
    mova                 m6, [cq+2*32+16]
    mova                 m7, [cq+3*32+16]
%if ARCH_X86_32
    mov                  r1, [rsp+4*16+1*gprsize]
%endif
    jmp                tx2q
.pass1_main:
    call m(idct_8x4_internal_16bpc).main_pass1
    pcmpeqd              m1, m1
    REPX      {psubd x, m1}, m0, m6, m5, m3
    call m(idct_8x4_internal_16bpc).round
    REPX      {psrad x, 1 }, m0, m1, m2, m3, m4, m5, m6, m7
.pack_and_transpose:
    packssdw             m2, m3
    packssdw             m6, m7
    packssdw             m0, m1
    packssdw             m4, m5
    jmp m(idct_8x4_internal_16bpc).transpose4x8packed

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(idct_8x8_internal_8bpc, _ssse3).main
    lea                  r3, [strideq*3]
%if ARCH_X86_64
    mova                m10, [o(pixel_10bpc_max)]
    pxor                 m9, m9
%endif
    call .round3_and_write_8x8
.zero:
%if ARCH_X86_64
%define mzero m9
%else
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+16*x], mzero}, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
%undef mzero
    RET

    ; round (rounded right-shift by 5) before writing
    ; data in m0-7
    ; on x86-64, pw_2048 is in m8
    ; .round1 is for m0-7
    ; .round2 is for m0-6 & [rsp+gprsize*2]
    ; .round3 is same, but without using m8 on x86-64 (.round2/3 are identical on x86-32)
    ; .round4 is x86-32-only, it is similar to .round2 but with constant already in m7
%if ARCH_X86_32
.round1_and_write_8x8:
    mova    [rsp+gprsize*2], m7
.round2_and_write_8x8:
%endif
.round3_and_write_8x8:
    mova                 m7, [o(pw_2048)]
%if ARCH_X86_32
.round4_and_write_8x8:
%endif
    REPX   {pmulhrsw x, m7}, m0, m1, m2, m3, m4, m5, m6
    pmulhrsw             m7, [rsp+gprsize*2]
%if ARCH_X86_64
    jmp .write_8x8
.round2_and_write_8x8:
    mova                 m7, [rsp+gprsize*2]
.round1_and_write_8x8:
    REPX   {pmulhrsw x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
%endif

    ; m0-7 have to-be-written data [pre-rounded]
    ; on x86-64, m9-10 contain a zero/pixel_max
    ; on x86-32, these are runtime-generated, and [rsp+gprsize*2] is scratch
    ; r0,1,3 contain dstq/strideq/stride3q
    ; r5 is a scratch register
.write_8x8:
    lea                  r5, [dstq+strideq*4]
    paddw                m0, [dstq+strideq*0]
    paddw                m1, [dstq+strideq*1]
    paddw                m2, [dstq+strideq*2]
    paddw                m3, [dstq+r3]
    paddw                m4, [r5  +strideq*0]
    paddw                m5, [r5  +strideq*1]
    paddw                m6, [r5  +strideq*2]
    paddw                m7, [r5  +r3]
%if ARCH_X86_64
    REPX    {pmaxsw x, m9 }, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsw x, m10}, m0, m1, m2, m3, m4, m5, m6, m7
%else
    mova    [rsp+gprsize*2], m7
    pxor                 m7, m7
    REPX     {pmaxsw x, m7}, m0, m1, m2, m3, m4, m5, m6
    pmaxsw               m7, [rsp+gprsize*2]
    mova    [rsp+gprsize*2], m7
    mova                 m7, [o(pixel_10bpc_max)]
    REPX     {pminsw x, m7}, m0, m1, m2, m3, m4, m5, m6
    pminsw               m7, [rsp+gprsize*2]
%endif
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+r3       ], m3
    mova   [r5  +strideq*0], m4
    mova   [r5  +strideq*1], m5
    mova   [r5  +strideq*2], m6
    mova   [r5  +r3       ], m7
    ret

INV_TXFM_8X8_FN adst, dct
INV_TXFM_8X8_FN adst, adst
INV_TXFM_8X8_FN adst, flipadst
INV_TXFM_8X8_FN adst, identity, 6

cglobal iadst_8x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_32
    mov [rsp+4*16+1*gprsize], r1
%endif
    lea                  t0, [o(.pass1_main)]
    jmp m(idct_8x8_internal_16bpc).pass1_full
.pass1_main:
    call m(iadst_8x4_internal_16bpc).main_pass1
    call .round
    jmp m(idct_8x8_internal_16bpc).pack_and_transpose
.round:
%if ARCH_X86_64
    pcmpeqd              m8, m8         ; -1
    REPX     {psubd x, m8 }, m0, m6
    REPX     {pxor  x, m8 }, m1, m7, m3, m5
    REPX     {psrad x, 1  }, m0, m1, m6, m7
    REPX     {psubd x, m8 }, m1, m7
    mova                 m8, [o(pd_6144)]
    REPX     {paddd x, m8 }, m2, m3, m4, m5
    REPX     {psrad x, 13 }, m2, m3, m4, m5
%else
    mova          [r3+2*16], m0

    pcmpeqd              m0, m0         ; -1
    mova                 m6, [o(pd_6144)]
    REPX     {pxor  x, m0 }, m1, m7, m3, m5
    REPX     {psrad x, 1  }, m1, m7
    REPX     {psubd x, m0 }, m1, m7
    REPX     {paddd x, m6 }, m2, m3, m4, m5
    REPX     {psrad x, 13 }, m2, m3, m4, m5

    mova                 m0, [r3+2*16]
    psrld                m6, 12         ; +1
    paddd                m0, m6
    paddd                m6, [r3+3*16]
    REPX     {psrad x, 1  }, m0, m6
%endif
    ret

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_8x8_internal_8bpc, _ssse3).main
    call m_suffix(iadst_8x8_internal_8bpc, _ssse3).main_pass2_end
    lea                  r3, [strideq*3]
%if ARCH_X86_64
    mova                m10, [o(pixel_10bpc_max)]
    pxor                 m9, m9
%endif
    call .round3_and_write_8x8
    jmp m(idct_8x8_internal_16bpc).zero

    ; round (rounded right-shift by 5) before writing; odd registers are negated
    ; data in m0-7
    ; on x86-64, pw_2048 is in m8 and pw_m2048 is in m11
    ; .round1 is for m0-7
    ; .round2 is for m0-6 & [rsp+gprsize*2]
    ; .round3 is same, but without using m8 on x86-64 (.round2/3 are identical on x86-32)
%if ARCH_X86_64
.round2_and_write_8x8:
    mova                 m7, [rsp+gprsize*2]
.round1_and_write_8x8:
    REPX  {pmulhrsw x, m8 }, m0, m2, m4, m6
    REPX  {pmulhrsw x, m11}, m1, m3, m5, m7
    jmp m(idct_8x8_internal_16bpc).write_8x8
%else
.round1_and_write_8x8:
    mova    [rsp+gprsize*2], m7
.round2_and_write_8x8:
%endif
.round3_and_write_8x8:
    mova                 m7, [o(pw_2048)]
    REPX   {pmulhrsw x, m7}, m0, m2, m4, m6
    mova                 m7, [o(pw_m2048)]
    REPX   {pmulhrsw x, m7}, m1, m3, m5
    pmulhrsw             m7, [rsp+gprsize*2]
    jmp m(idct_8x8_internal_16bpc).write_8x8

INV_TXFM_8X8_FN flipadst, dct
INV_TXFM_8X8_FN flipadst, adst
INV_TXFM_8X8_FN flipadst, flipadst
INV_TXFM_8X8_FN flipadst, identity, 6

cglobal iflipadst_8x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_32
    mov [rsp+4*16+1*gprsize], r1
%endif
    lea                  t0, [o(.pass1_main)]
    jmp m(idct_8x8_internal_16bpc).pass1_full
.pass1_main:
    call m(iadst_8x4_internal_16bpc).main_pass1
    call m(iadst_8x8_internal_16bpc).round
    ; invert registers
    packssdw             m7, m6
    packssdw             m5, m4
    packssdw             m3, m2
    packssdw             m1, m0
    mova                 m0, m7
    mova                 m2, m5
    mova                 m4, m3
    mova                 m6, m1
    jmp m(idct_8x4_internal_16bpc).transpose4x8packed

.pass2:
    lea                dstq, [dstq+strideq*8]
    sub                dstq, strideq
    neg             strideq
    jmp m(iadst_8x8_internal_16bpc).pass2

INV_TXFM_8X8_FN identity, dct
INV_TXFM_8X8_FN identity, adst
INV_TXFM_8X8_FN identity, flipadst
INV_TXFM_8X8_FN identity, identity

cglobal iidentity_8x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+0*32]
    mova                 m1, [cq+1*32]
    mova                 m2, [cq+2*32]
    mova                 m3, [cq+3*32]
    mova                 m4, [cq+4*32]
    mova                 m5, [cq+5*32]
    mova                 m6, [cq+6*32]
    mova                 m7, [cq+7*32]
    packssdw             m0, [cq+0*32+16]
    packssdw             m1, [cq+1*32+16]
    packssdw             m2, [cq+2*32+16]
    packssdw             m3, [cq+3*32+16]
    packssdw             m4, [cq+4*32+16]
    packssdw             m5, [cq+5*32+16]
    packssdw             m6, [cq+6*32+16]
    packssdw             m7, [cq+7*32+16]
    mova [rsp+gprsize+16*1], m6
    jmp m_suffix(idct_8x8_internal_8bpc, _ssse3).pass1_end3

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    lea                  r3, [strideq*3]
%if ARCH_X86_64
    mova                m10, [o(pixel_10bpc_max)]
    pxor                 m9, m9
    mova                 m8, [o(pw_4096)]
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
%else
    mova      [rsp+gprsize], m7
    mova                 m7, [o(pw_4096)]
    call m(idct_8x8_internal_16bpc).round4_and_write_8x8
%endif
    jmp m(idct_8x8_internal_16bpc).zero

%macro INV_TXFM_8X16_FN 2-3 2d ; type1, type2, eob_tbl_suffix
%if ARCH_X86_64
    INV_TXFM_FN          %1, %2, tbl_8x16_%3, 8x16, 15, 0-16*16
%else
    INV_TXFM_FN          %1, %2, tbl_8x16_%3, 8x16, 8, 0-17*16
%endif
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
    mov                 r3d, 4
%if stack_size_padded > 0
    ; adjust to caller's stack allocation
    add                 rsp, (12+ARCH_X86_64)*16
%endif
    jmp m(inv_txfm_add_dct_dct_8x8_16bpc).end
%endif
%endmacro

INV_TXFM_8X16_FN dct, dct
INV_TXFM_8X16_FN dct, identity, v
INV_TXFM_8X16_FN dct, adst
INV_TXFM_8X16_FN dct, flipadst

%if ARCH_X86_64
DECLARE_REG_TMP 7
%endif

cglobal idct_8x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if WIN64
    PUSH                 r7
%elif ARCH_X86_32
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(m(idct_8x8_internal_16bpc).pass1_main)]
.pass1_full:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
%undef cmp
    mov                 r6d, 4
.zero_loop:
    dec                 r6d
    cmp                eobb, byte [r5+r6]
    jl .zero_loop
    mov                 r5d, r6d
    shl                 r5d, 4
%if ARCH_X86_32
    ; restore pic-ptr
    mov                  r6, [rsp+16*16+2*gprsize]
    ; setup stack pointer
    lea                  r3, [rsp+gprsize]
%endif
.loop_pass1:
    mova                 m0, [cq+0*64+r5]
    mova                 m1, [cq+1*64+r5]
    mova                 m2, [cq+2*64+r5]
    mova                 m3, [cq+3*64+r5]
    mova                 m4, [cq+4*64+r5]
    mova                 m5, [cq+5*64+r5]
    mova                 m6, [cq+6*64+r5]
    mova                 m7, [cq+7*64+r5]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call                 t0

    mova       [cq+0*64+r5], m0
    mova       [cq+1*64+r5], m1
    mova       [cq+2*64+r5], m2
    mova       [cq+3*64+r5], m3
    sub                 r5d, 16
    jge .loop_pass1
%if WIN64
    POP                  r7
%elif ARCH_X86_32
    mov                  r1, [rsp+16*16+1*gprsize]
%endif
    jmp                tx2q

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif

    ; input is in cqN*16, where N=0/4/8/12/1/5/9/13/2/6/10/14/3/7/11/15
    ; some are still pre-loaded from the final loop iteration in pass=1

    mova                 m1, m2
    mova                 m2, [cq+ 1*16]
    mova                 m3, [cq+ 9*16]
    mova                 m4, [cq+ 2*16]
    mova                 m5, [cq+10*16]
    mova                 m6, [cq+ 3*16]
    mova                 m7, [cq+11*16]
    call m_suffix(idct_8x8_internal_8bpc, _ssse3).main
    mova [rsp+gprsize+3*16], m0
    mova [rsp+gprsize+4*16], m1
    mova [rsp+gprsize+5*16], m2
    mova [rsp+gprsize+6*16], m3
    mova [rsp+gprsize+7*16], m4
    mova [rsp+gprsize+8*16], m5
    mova [rsp+gprsize+9*16], m6
    ; m7 is already stored in [rsp+gprsize+0*16]
    mova                 m0, [cq+ 4*16]
    mova                 m1, [cq+12*16]
    mova                 m2, [cq+ 5*16]
    mova                 m3, [cq+13*16]
    mova                 m4, [cq+ 6*16]
    mova                 m5, [cq+14*16]
    mova                 m6, [cq+ 7*16]
    mova                 m7, [cq+15*16]
    call m_suffix(idct_16x8_internal_8bpc, _ssse3).main

    ; out0-7 is in rsp+gprsize+3-10*mmsize
    ; out8-14 is in m0-6, and out15 is in m7 as well as rsp+gprsize+0*mmsize

%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    mova                m10, [o(pixel_10bpc_max)]
    pxor                 m9, m9
    mov                  r6, dstq
%else
    mov [rsp+16*16+gprsize*1], dstq
%endif
    lea                  r3, [strideq*3]
    lea                dstq, [dstq+strideq*8]
    call m(idct_8x8_internal_16bpc).round2_and_write_8x8
%if ARCH_X86_64
%define mzero m9
%else
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+x*16], mzero}, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, \
                     16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
%undef mzero
    mova                 m0, [rsp+gprsize+ 3*16]
    mova                 m1, [rsp+gprsize+ 4*16]
    mova                 m2, [rsp+gprsize+ 5*16]
    mova                 m3, [rsp+gprsize+ 6*16]
    mova                 m4, [rsp+gprsize+ 7*16]
    mova                 m5, [rsp+gprsize+ 8*16]
    mova                 m6, [rsp+gprsize+ 9*16]
    mova                 m7, [rsp+gprsize+10*16]
%if ARCH_X86_64
    mov                dstq, r6
%else
    mov                dstq, [rsp+16*16+gprsize*1]
%endif
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
    RET

INV_TXFM_8X16_FN adst, dct
INV_TXFM_8X16_FN adst, adst
INV_TXFM_8X16_FN adst, flipadst
INV_TXFM_8X16_FN adst, identity, v

cglobal iadst_8x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if WIN64
    PUSH                 r7
%elif ARCH_X86_32
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(m(iadst_8x8_internal_16bpc).pass1_main)]
    jmp m(idct_8x16_internal_16bpc).pass1_full

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    mova                 m4, [cq+ 9*16]
    mova                 m5, [cq+13*16]
    mova [rsp+gprsize+7*16], m0
    mova [rsp+gprsize+8*16], m1
    mova [rsp+gprsize+5*16], m4
    mova [rsp+gprsize+6*16], m5
    mova                 m0, m2
    mova                 m1, m3
    mova                 m2, [cq+ 1*16]
    mova                 m3, [cq+ 5*16]
    mova                 m4, [cq+ 2*16]
    mova                 m5, [cq+ 6*16]
    mova                 m6, [cq+11*16]
    mova                 m7, [cq+15*16]
    mova [rsp+gprsize+ 3*16], m4
    mova [rsp+gprsize+ 4*16], m5
    mova [rsp+gprsize+ 9*16], m6
    mova [rsp+gprsize+10*16], m7
    mova                 m4, [cq+10*16]
    mova                 m5, [cq+14*16]
    mova                 m6, [cq+ 3*16]
    mova                 m7, [cq+ 7*16]
    call m_suffix(iadst_16x8_internal_8bpc, _ssse3).main
    call m_suffix(iadst_16x8_internal_8bpc, _ssse3).main_pass2_end

%if ARCH_X86_64
    mova                m11, [o(pw_m2048)]
    mova                 m8, [o(pw_2048)]
    mova                m10, [o(pixel_10bpc_max)]
    pxor                 m9, m9
    mov                  r6, dstq
%else
    mov [rsp+16*16+gprsize*1], dstq
%endif
    lea                  r3, [strideq*3]
    lea                dstq, [dstq+strideq*8]
    call m(iadst_8x8_internal_16bpc).round2_and_write_8x8
%if ARCH_X86_64
%define mzero m9
%else
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+x*16], mzero}, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, \
                     16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
%undef mzero
    mova                 m0, [rsp+gprsize+ 3*16]
    mova                 m1, [rsp+gprsize+ 4*16]
    mova                 m2, [rsp+gprsize+ 5*16]
    mova                 m3, [rsp+gprsize+ 6*16]
    mova                 m4, [rsp+gprsize+ 7*16]
    mova                 m5, [rsp+gprsize+ 8*16]
    mova                 m6, [rsp+gprsize+ 9*16]
    mova                 m7, [rsp+gprsize+10*16]
%if ARCH_X86_64
    mov                dstq, r6
%else
    mov                dstq, [rsp+16*16+gprsize*1]
%endif
    call m(iadst_8x8_internal_16bpc).round1_and_write_8x8
    RET

INV_TXFM_8X16_FN flipadst, dct
INV_TXFM_8X16_FN flipadst, adst
INV_TXFM_8X16_FN flipadst, flipadst
INV_TXFM_8X16_FN flipadst, identity, v

cglobal iflipadst_8x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if WIN64
    PUSH                 r7
%elif ARCH_X86_32
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(m(iflipadst_8x8_internal_16bpc).pass1_main)]
    jmp m(idct_8x16_internal_16bpc).pass1_full

.pass2:
    lea                  r3, [strideq*3]
    lea                  r3, [r3*5]
    add                dstq, r3
    neg             strideq
    jmp m(iadst_8x16_internal_16bpc).pass2

INV_TXFM_8X16_FN identity, dct, h
INV_TXFM_8X16_FN identity, adst, h
INV_TXFM_8X16_FN identity, flipadst, h
INV_TXFM_8X16_FN identity, identity

cglobal iidentity_8x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if WIN64
    PUSH                 r7
%elif ARCH_X86_32
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(m(idct_8x8_internal_16bpc).pack_and_transpose)]
    jmp m(idct_8x16_internal_16bpc).pass1_full

.pass2:
%if ARCH_X86_64
    mova                 m4, [o(pw_2048)]
    mova                 m5, [o(pixel_10bpc_max)]
    pxor                 m6, m6
    mova                 m7, [o(pw_1697x16)]
%endif
    mov                 r5d, 4
    lea                  r3, [strideq*3]
.pass2_loop:
    call .main
%if ARCH_X86_64
    call m(idct_8x4_internal_16bpc).round1_and_write_8x4
%else
    call m(idct_8x4_internal_16bpc).round2_and_write_8x4
%endif
    REPX {mova [cq+x*16], m6}, 0, 4, 8, 12, 16, 20, 24, 28
    dec                 r5d
    jle .end
    add                  cq, 16
    lea                dstq, [dstq+strideq*4]
    mova                 m0, [cq+ 0*16]
    mova                 m1, [cq+ 4*16]
    mova                 m2, [cq+ 8*16]
    mova                 m3, [cq+12*16]
    jmp .pass2_loop
.end:
    RET
.main:
    ; y = pmulhrsw(x, pw_1697x16); x = paddsw(x, x); x = paddsw(x, y)
%if ARCH_X86_32
    mova                 m7, [o(pw_1697x16)]
    pmulhrsw             m4, m7, m0
    pmulhrsw             m5, m7, m1
    pmulhrsw             m6, m7, m2
    pmulhrsw             m7, m3
%else
    pmulhrsw             m8, m7, m0
    pmulhrsw             m9, m7, m1
    pmulhrsw            m10, m7, m2
    pmulhrsw            m11, m7, m3
%endif
    REPX      {paddsw x, x}, m0, m1, m2, m3
%if ARCH_X86_64
    paddsw               m0, m8
    paddsw               m1, m9
    paddsw               m2, m10
    paddsw               m3, m11
%else
    paddsw               m0, m4
    paddsw               m1, m5
    paddsw               m2, m6
    paddsw               m3, m7
%endif
    ret

%macro INV_TXFM_16X4_FN 2 ; type1, type2
%if ARCH_X86_64
    INV_TXFM_FN          %1, %2, 0, 16x4, 16, 0-8*16
%else
    INV_TXFM_FN          %1, %2, 0, 16x4, 8, 0-12*16
%endif
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 4
.dconly:
    add                 r5d, 384
    sar                 r5d, 9
.dconly2:
    imul                r5d, 2896
    add                 r5d, 34816
    movd                 m0, r5d
    pshuflw              m0, m0, q1111
    punpcklqdq           m0, m0
    mova                 m3, [o(pixel_10bpc_max)]
    pxor                 m4, m4
.loop:
    mova                 m1, [dstq+ 0]
    mova                 m2, [dstq+16]
    REPX     {paddw  x, m0}, m1, m2
    REPX     {pminsw x, m3}, m1, m2
    REPX     {pmaxsw x, m4}, m1, m2
    mova          [dstq+ 0], m1
    mova          [dstq+16], m2
    add                dstq, strideq
    dec                 r3d
    jg .loop
    RET
%endif
%endmacro

INV_TXFM_16X4_FN dct, dct
INV_TXFM_16X4_FN dct, identity
INV_TXFM_16X4_FN dct, adst
INV_TXFM_16X4_FN dct, flipadst

cglobal idct_16x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    ; setup stack pointer
    lea                  r3, [rsp+gprsize]

    mova                 m0, [cq+ 1*16]
    mova                 m1, [cq+ 3*16]
    mova                 m2, [cq+ 5*16]
    mova                 m3, [cq+ 7*16]
    mova                 m4, [cq+ 9*16]
    mova                 m5, [cq+11*16]
    mova                 m6, [cq+13*16]
    mova                 m7, [cq+15*16]
    call .main_oddhalf
    mova                 m0, [cq+ 0*16]
    mova                 m1, [cq+ 2*16]
    mova                 m2, [cq+ 4*16]
    mova                 m3, [cq+ 6*16]
    mova                 m4, [cq+ 8*16]
    mova                 m5, [cq+10*16]
    mova                 m6, [cq+12*16]
    mova                 m7, [cq+14*16]
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    ; t0-7 is in m0-7

    call .round

%if ARCH_X86_64
.pack_transpose:
    ; transpose in two parts
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
.transpose:
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    call .transpose4x8packed_hi
%else
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova          [r3+0*16], m0
    mova          [r3+1*16], m1
    mova          [r3+2*16], m2
    mova          [r3+3*16], m3
    mova                 m0, [r3+ 8*16]
    mova                 m2, [r3+ 9*16]
    mova                 m4, [r3+10*16]
    mova                 m6, [r3+11*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
%endif
    jmp                tx2q
%if ARCH_X86_64
.transpose4x8packed_hi:
    punpcklwd            m9, m10, m14
    punpckhwd           m10, m14
    punpckhwd           m14, m8, m12
    punpcklwd            m8, m12

    punpckhwd           m11, m8, m9
    punpcklwd            m8, m9
    punpckhwd           m12, m14, m10
    punpcklwd           m14, m10

    punpcklwd           m10, m11, m12
    punpckhwd           m11, m12
    punpckhwd            m9, m8, m14
    punpcklwd            m8, m14
    ret
%endif
.main_oddhalf_fast: ; lower half zero
    pmulld               m7, m0, [o(pd_4076)]
    pmulld               m0, [o(pd_401)]
    pmulld               m6, m1, [o(pd_m1189)]
    pmulld               m1, [o(pd_3920)]
%if ARCH_X86_32
    mova                 m4, [o(pd_2048)]
    REPX      {paddd x, m4}, m1, m6
    REPX      {psrad x, 12}, m1, m6
    mova          [r3+1*16], m1
%endif
    pmulld               m5, m2, [o(pd_3612)]
    pmulld               m2, [o(pd_1931)]
%if ARCH_X86_32
    pmulld               m1, m3, [o(pd_m2598)]
%else
    pmulld               m4, m3, [o(pd_m2598)]
%endif
    pmulld               m3, [o(pd_3166)]
    jmp .main_oddhalf_fast2
.main_oddhalf:
%if ARCH_X86_64
    ITX_MULSUB_2D         0, 7, 8, 9, 10, _,  401, 4076 ; t8a,  t15a
    ITX_MULSUB_2D         6, 1, 8, 9, 10, _, 3920, 1189 ; t11a, t12a
    ITX_MULSUB_2D         2, 5, 8, 9, 10, _, 1931, 3612 ; t10a, t13a
    ITX_MULSUB_2D         4, 3, 8, 9, 10, _, 3166, 2598 ; t9a,  t14a
.main_oddhalf_fast2:
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
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
    mova                m15, [o(pd_3784)]
    mova                m10, [o(pd_1567)]
    ITX_MULSUB_2D         1, 8, 3, 9, _, 11, 10, 15
    ITX_MULSUB_2D         6, 4, 3, 9, _, 11, 10, 15, 4
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
    mova          [r3+0*16], m0
    mova          [r3+1*16], m1
    mova          [r3+2*16], m2
    mova          [r3+3*16], m3
    mova          [r3+4*16], m4
    mova          [r3+5*16], m5
    mova          [r3+6*16], m6
    mova          [r3+7*16], m7
%else
    mova          [r3+0*16], m2
    mova          [r3+1*16], m3
    mova          [r3+2*16], m4
    mova          [r3+3*16], m5
    mova                 m4, [o(pd_2048)]

    ITX_MULSUB_2D         0, 7, 2, 3, 5, _,  401, 4076 ; t8a,  t15a
    ITX_MULSUB_2D         6, 1, 2, 3, 5, 4, 3920, 1189 ; t11a, t12a

    mova                 m2, [r3+0*16]
    mova                 m3, [r3+1*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m1
    mova                 m1, [r3+2*16]
    mova                 m5, [r3+3*16]
    mova          [r3+2*16], m6
    mova          [r3+3*16], m7

    ITX_MULSUB_2D         2, 5, 0, 6, 7, _, 1931, 3612 ; t10a, t13a
    ITX_MULSUB_2D         1, 3, 0, 6, 7, _, 3166, 2598 ; t9a,  t14a

    mova                 m0, [r3+0*16]
    mova                 m6, [r3+2*16]
    mova                 m7, [r3+3*16]
.main_oddhalf_fast2:
    REPX      {paddd x, m4}, m0, m7, m2, m5, m1, m3
    REPX      {psrad x, 12}, m0, m7, m2, m5, m1, m3
    psubd                m4, m0, m1 ; t9
    paddd                m0, m1     ; t8
    mova                 m1, [r3+1*16]
    mova          [r3+0*16], m4
    psubd                m4, m6, m2 ; t10
    paddd                m2, m6     ; t11
    psubd                m6, m1, m5 ; t13
    paddd                m5, m1     ; t12
    psubd                m1, m7, m3 ; t14
    paddd                m7, m3     ; t15
    mova                 m3, [o(clip_18b_min)]
    REPX     {pmaxsd x, m3}, m1, m4, m6, m0, m2, m5, m7
    pmaxsd               m3, [r3+0*16]
    mova          [r3+0*16], m3
    mova                 m3, [o(clip_18b_max)]
    REPX     {pminsd x, m3}, m1, m4, m6, m0, m2, m5, m7
    pminsd               m3, [r3+0*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m2
    mova          [r3+2*16], m5
    mova          [r3+3*16], m7
    mova                m7, [o(pd_2048)]
    ITX_MULSUB_2D         1, 3, 0, 2, 5, 7, 1567, 3784
    ITX_MULSUB_2D         6, 4, 0, 2, _, 7,    5, 3784, 4
    mova                 m0, [r3+0*16]
    mova                 m2, [r3+1*16]
    psubd                m5, m1, m4 ; t10
    mova          [r3+1*16], m5
    paddd                m1, m4     ; t9
    psubd                m4, m0, m2 ; t11a
    paddd                m0, m2     ; t8a
    mova                 m5, [r3+2*16]
    mova                 m7, [r3+3*16]
    psubd                m2, m3, m6 ; t13
    paddd                m6, m3     ; t14
    paddd                m3, m7, m5 ; t15a
    psubd                m7, m5     ; t12a
    mova          [r3+0*16], m3
    mova                 m3, [r3+1*16]
    mova                 m5, [o(clip_18b_min)]
    REPX     {pmaxsd x, m5}, m2, m7, m3, m4, m0, m1, m6
    pmaxsd               m5, [r3+0*16]
    mova          [r3+0*16], m5
    mova                 m5, [o(clip_18b_max)]
    REPX     {pminsd x, m5}, m2, m7, m3, m4, m0, m1, m6
    pminsd               m5, [r3+0*16]
    mova          [r3+0*16], m5
    mova                 m5, [o(pd_2896)]
    REPX     {pmulld x, m5}, m2, m7, m3, m4
    mova                 m5, [o(pd_2048)]
    REPX     {paddd  x, m5}, m2, m7
    paddd                m5, m2, m3 ; t13a
    psubd                m2, m3     ; t10a
    psubd                m3, m7, m4 ; t11
    paddd                m4, m7     ; t12
    REPX      {psrad x, 12}, m5, m2, m3, m4
    mova                 m7, [r3+0*16]
    mova         [r3+11*16], m0
    mova         [r3+10*16], m1
    mova          [r3+9*16], m2
    mova          [r3+8*16], m3
    mova          [r3+7*16], m4
    mova          [r3+6*16], m5
    mova          [r3+5*16], m6
    mova          [r3+4*16], m7
%endif
    ret
.round:
%if ARCH_X86_64
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    pcmpeqd              m8, m8
    REPX      {psubd x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    mova                 m8, [r3+1*16]
    mova                 m9, [r3+2*16]
    mova                m10, [r3+3*16]
    mova                m11, [r3+4*16]
    mova                m12, [r3+5*16]
    mova                m13, [r3+6*16]
    mova                m14, [r3+7*16]
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
    psubd                m8, m7, [r3+0*16] ; out8
    paddd                m7, [r3+0*16]     ; out7
    REPX       {psrad x, 1}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    ; and out0-15 is now in m0-15
%else
    mova         [r3+ 0*16], m0
    mova                 m0, [o(clip_18b_min)]
    REPX     {pmaxsd x, m0}, m1, m2, m3, m4, m5, m6, m7
    pmaxsd               m0, [r3+ 0*16]
    mova         [r3+ 0*16], m7
    mova                 m7, [o(clip_18b_max)]
    REPX     {pminsd x, m7}, m0, m1, m2, m3, m4, m5, m6
    pminsd               m7, [r3+ 0*16]
    mova         [r3+ 0*16], m0
    pcmpeqd              m0, m0
    REPX      {psubd x, m0}, m1, m2, m3, m4, m5, m6, m7
    mova         [r3+ 1*16], m1
    mova         [r3+ 2*16], m2
    mova                 m1, [r3+ 0*16]
    psubd                m1, m0
    mova         [r3+ 0*16], m1
    mova                 m1, [r3+11*16]
    mova                 m2, [r3+10*16]
    psubd                m0, m7, m1
    paddd                m7, m1
    psubd                m1, m6, m2
    paddd                m6, m2
    REPX       {psrad x, 1}, m0, m1, m6, m7
    packssdw             m0, m1     ; out8-9
    packssdw             m6, m7     ; out6-7
    mova         [r3+11*16], m6
    mova                 m1, [r3+9*16]
    mova                 m7, [r3+8*16]
    psubd                m2, m5, m1
    paddd                m5, m1
    psubd                m1, m4, m7
    paddd                m4, m7
    REPX       {psrad x, 1}, m2, m1, m4, m5
    packssdw             m2, m1     ; out10-11
    packssdw             m4, m5     ; out4-5
    mova                 m1, [r3+2*16]
    mova         [r3+10*16], m4
    mova                 m6, [r3+7*16]
    mova                 m7, [r3+6*16]
    psubd                m4, m3, m6
    paddd                m3, m6
    psubd                m6, m1, m7
    paddd                m1, m7
    REPX       {psrad x, 1}, m4, m6, m1, m3
    packssdw             m4, m6     ; out12-13
    packssdw             m1, m3     ; out2-3
    mova                 m3, [r3+1*16]
    mova          [r3+9*16], m1
    mova                 m1, [r3+0*16]
    mova                 m5, [r3+5*16]
    mova                 m7, [r3+4*16]
    psubd                m6, m3, m5
    paddd                m3, m5
    psubd                m5, m1, m7
    paddd                m1, m7
    REPX       {psrad x, 1}, m6, m5, m1, m3
    packssdw             m6, m5     ; out14-15
    packssdw             m1, m3     ; out0-1
    mova          [r3+8*16], m1
%endif
    ret

.pass2:
    lea                  r4, [o(m_suffix(idct_8x4_internal_8bpc, _ssse3).main)]
.pass2_loop:
    lea                  r3, [strideq*3]
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call                 r4
    call m(idct_8x4_internal_16bpc).round2_and_write_8x4
    REPX {mova [cq+x*16], m6}, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
%if ARCH_X86_64
    mova                 m0, m8
    mova                 m1, m9
    mova                 m2, m10
    mova                 m3, m11
%else
    mova                 m0, [rsp+gprsize+0*16]
    mova                 m1, [rsp+gprsize+1*16]
    mova                 m2, [rsp+gprsize+2*16]
    mova                 m3, [rsp+gprsize+3*16]
%endif
    add                dstq, 16
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call                 r4
    call m(idct_8x4_internal_16bpc).round2_and_write_8x4
    RET

INV_TXFM_16X4_FN adst, dct
INV_TXFM_16X4_FN adst, adst
INV_TXFM_16X4_FN adst, flipadst
INV_TXFM_16X4_FN adst, identity

cglobal iadst_16x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    ; setup stack pointer
    lea                  r3, [rsp+gprsize]
    call .main
%if ARCH_X86_64
    jmp m(idct_16x4_internal_16bpc).pack_transpose
%else
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova [rsp+gprsize+0*16], m0
    mova [rsp+gprsize+1*16], m1
    mova [rsp+gprsize+2*16], m2
    mova [rsp+gprsize+3*16], m3
    mova                 m0, [rsp+gprsize+ 8*16]
    mova                 m2, [rsp+gprsize+ 9*16]
    mova                 m4, [rsp+gprsize+10*16]
    mova                 m6, [rsp+gprsize+11*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    jmp                tx2q
%endif

.main:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+ 2*16]
    mova                 m1, [cq+13*16]
    mova                 m2, [cq+ 6*16]
    mova                 m3, [cq+ 9*16]
    mova                 m4, [cq+10*16]
    mova                 m5, [cq+ 5*16]
    mova                 m6, [cq+14*16]
    mova                 m7, [cq+ 1*16]
    call .main_part1
    mova                 m0, [cq+ 0*16]
    mova                 m1, [cq+15*16]
    mova                 m2, [cq+ 4*16]
    mova                 m3, [cq+11*16]
    mova                 m4, [cq+ 8*16]
    mova                 m5, [cq+ 7*16]
    mova                 m6, [cq+12*16]
    mova                 m7, [cq+ 3*16]
    call .main_part2
.round:
%if ARCH_X86_64
    mova                m15, [o(pd_6144)]
    psrld               m14, 11       ; pd_1
    pcmpeqd              m8, m8       ; -1
    psubd               m13, m15, m14 ; pd_6143
    REPX     {paddd x, m14}, m0, m2
    REPX     {paddd x, m15}, m4, m6
    REPX     {pxor  x, m8 }, m1, m3, m5, m7
    REPX     {psrad x, 1  }, m1, m3
    REPX     {paddd x, m15}, m5, m7
    REPX     {psubd x, m8 }, m1, m3
    paddd                m8, m15, m9
    psubd                m9, m13, m10
    paddd               m10, m15, m11
    psubd               m11, m13, m12
    paddd               m12, m14, [r3+3*16]
    psubd               m13, m14, [r3+2*16]
    psubd               m15, m14, [r3+0*16]
    paddd               m14, [r3+1*16]
    REPX      {psrad x, 1 }, m0,  m2,  m12, m13, m14, m15
    REPX      {psrad x, 13}, m4,  m5,  m6,  m7,  m8,  m9,  m10, m11
%else
    mova          [r3+8*16], m1
    mova          [r3+9*16], m3
    mova                 m3, [o(pd_6144)]
    pcmpeqd              m1, m1
    REPX      {pxor  x, m1}, m5, m7
    REPX      {paddd x, m3}, m4, m5, m6, m7
    REPX      {psrad x, 13}, m4, m5, m6, m7
    packssdw             m4, m5
    packssdw             m6, m7
    mova         [r3+10*16], m4
    mova         [r3+11*16], m6
    mova                 m4, [r3+4*16]
    mova                 m5, [r3+5*16]
    mova                 m6, [r3+6*16]
    mova                 m7, [r3+7*16]
    REPX      {pxor  x, m1}, m5, m7
    REPX      {psubd x, m1}, m4, m6
    REPX      {psrad x, 1 }, m4, m5, m6, m7
    REPX      {psubd x, m1}, m5, m7
    packssdw             m4, m5
    packssdw             m6, m7
    mova                 m5, [r3+8*16]
    mova                 m7, [r3+9*16]
    mova          [r3+8*16], m4
    mova          [r3+9*16], m6
    REPX      {pxor  x, m1}, m5, m7
    REPX      {paddd x, m3}, m0, m5, m2, m7
    REPX      {psrad x, 13}, m0, m5, m2, m7
    packssdw             m0, m5
    packssdw             m2, m7
    mova                 m4, [r3+0*16]
    mova                 m5, [r3+1*16]
    mova                 m6, [r3+2*16]
    mova                 m7, [r3+3*16]
    REPX      {psubd x, m1}, m4, m6
    REPX      {pxor  x, m1}, m5, m7
    REPX      {psrad x, 1 }, m4, m5, m6, m7
    REPX      {psubd x, m1}, m5, m7
    packssdw             m4, m5
    packssdw             m6, m7
%endif
    ret

.main_part2:
%if ARCH_X86_64
    ITX_MULSUB_2D         1, 0, 8, 9, 10, 11,  201, 4091
    ITX_MULSUB_2D         3, 2, 8, 9, 10, 11, 1751, 3703
    ITX_MULSUB_2D         5, 4, 8, 9, 10, 11, 3035, 2751
    ITX_MULSUB_2D         7, 6, 8, 9, 10, 11, 3857, 1380
    psubd                m8, m0, m4 ; t8a
    paddd                m0, m4     ; t0a
    psubd                m4, m1, m5 ; t9a
    paddd                m1, m5     ; t1a
    psubd                m5, m2, m6 ; t12a
    paddd                m2, m6     ; t4a
    psubd                m6, m3, m7 ; t13a
    paddd                m7, m3     ; t5a
    REPX    {pmaxsd x, m12}, m8, m4, m5, m6, m0, m1, m2, m7
    REPX    {pminsd x, m13}, m8, m4, m5, m6, m0, m1, m2, m7
    mova                m15, [o(pd_4017)]
    mova                m10, [o(pd_799)]
    ITX_MULSUB_2D         8, 4, 3, 9, _, 11, 10, 15
    ITX_MULSUB_2D         6, 5, 3, 9, _, 11, 15, 10
    psubd                m3, m0, m2 ; t4
    paddd                m0, m2     ; t0
    psubd                m2, m1, m7 ; t5
    paddd                m1, m7     ; t1
    psubd                m7, m4, m6 ; t12a
    paddd                m4, m6     ; t8a
    psubd                m6, m8, m5 ; t13a
    paddd                m5, m8     ; t9a
    REPX    {pmaxsd x, m12}, m3, m2, m7, m6, m0, m1, m4, m5
    REPX    {pminsd x, m13}, m3, m2, m7, m6, m0, m1, m4, m5
    mova                m15, [o(pd_3784)]
    mova                m10, [o(pd_1567)]
    ITX_MULSUB_2D         3, 2, 8, 9, _, 11, 10, 15
    ITX_MULSUB_2D         7, 6, 8, 9, _, 11, 10, 15
    mova                m10, [r3+0*16]      ;  t2
    mova                 m8, [r3+1*16]      ;  t3
    psubd                m9, m0, m10        ;  t2a
    paddd                m0, m10            ;  out0
    psubd               m10, m1, m8         ;  t3a
    paddd                m1, m8             ; -out15
    mova          [r3+0*16], m1
    mova                m15, [r3+3*16]      ;  t7a
    mova                 m1, [r3+2*16]      ;  t6a
    psubd                m8, m3, m15        ;  t7
    paddd               m15, m3             ;  out12
    paddd                m3, m2, m1         ; -out3
    psubd                m2, m1             ;  t6
    mova          [r3+3*16], m15
    mova          [r3+1*16], m2
    mova                 m1, [r3+7*16]      ;  t15
    mova                 m2, [r3+6*16]      ;  t14
    paddd               m15, m7, m1         ; -out13
    psubd                m7, m1             ;  t15a
    psubd               m11, m6, m2         ;  t14a
    paddd                m2, m6             ;  out2
    mova          [r3+2*16], m15
    mova                 m1, [r3+4*16]      ;  t10a
    mova                m15, [r3+5*16]      ;  t11a
    psubd                m6, m4, m1         ;  t10
    paddd                m1, m4             ; -out1
    psubd                m4, m5, m15        ;  t11
    paddd                m5, m15            ;  out14
    REPX    {pmaxsd x, m12}, m11, m7, m9, m10, m6, m4, m8
    pmaxsd              m12, [r3+1*16]      ;  t6
    mova          [r3+1*16], m5
    REPX    {pminsd x, m13}, m11, m7, m9, m10, m6, m4, m12, m8
    REPX    {pmulld x, m14}, m11, m7, m9, m10, m6, m4, m12, m8
    paddd                m5, m11, m7        ; -out5  (unshifted)
    psubd               m11, m7             ;  out10 (unshifted)
    paddd                m7, m9, m10        ; -out7  (unshifted)
    psubd                m9, m10            ;  out8  (unshifted)
    psubd               m10, m6, m4         ; -out9  (unshifted)
    paddd                m6, m4             ;  out6  (unshifted)
    paddd                m4, m12, m8        ;  out4  (unshifted)
    psubd               m12, m8             ; -out11 (unshifted)
%else
    mova          [r3+8*16], m0
    mova          [r3+9*16], m1
    mova         [r3+10*16], m2
    mova         [r3+11*16], m3
    mova                 m3, [o(pd_2048)]
    ITX_MULSUB_2D         5, 4, 0, 1, 2, 3, 3035, 2751
    ITX_MULSUB_2D         7, 6, 0, 1, 2, 3, 3857, 1380
    mova                 m0, [r3+8*16]
    mova                 m1, [r3+9*16]
    mova          [r3+8*16], m4
    mova                 m4, [r3+10*16]
    mova          [r3+9*16], m5
    mova         [r3+10*16], m6
    mova                 m5, [r3+11*16]
    mova         [r3+11*16], m7
    ITX_MULSUB_2D         1, 0, 2, 6, 7, 3,  201, 4091
    ITX_MULSUB_2D         5, 4, 2, 6, 7, 3, 1751, 3703
    mova                 m2, [r3+8*16]
    mova                 m6, [r3+9*16]
    psubd                m3, m0, m2 ; t8a
    paddd                m0, m2     ; t0a
    mova          [r3+8*16], m3
    psubd                m2, m1, m6 ; t9a
    paddd                m1, m6     ; t1a
    mova                 m3, [r3+10*16]
    psubd                m6, m4, m3 ; t12a
    paddd                m4, m3     ; t4a
    mova                 m3, [r3+11*16]
    psubd                m7, m5, m3 ; t13a
    paddd                m5, m3     ; t5a
    mova                 m3, [o(clip_18b_min)]
    REPX     {pmaxsd x, m3}, m2, m6, m7, m0, m1, m4, m5
    pmaxsd               m3, [r3+8*16]
    mova          [r3+8*16], m3
    mova                 m3, [o(clip_18b_max)]
    REPX     {pminsd x, m3}, m2, m6, m7, m0, m1, m4, m5
    pminsd               m3, [r3+8*16]
    mova          [r3+8*16], m3
    psubd                m3, m0, m4 ; t4
    paddd                m0, m4     ; t0
    psubd                m4, m1, m5 ; t5
    paddd                m1, m5     ; t1
    mova                 m5, [o(pd_2048)]
    mova          [r3+9*16], m1
    mova         [r3+10*16], m4
    mova         [r3+11*16], m3
    mova                 m3, [r3+8*16]
    mova          [r3+8*16], m0
    ITX_MULSUB_2D         3, 2, 0, 1, 4, 5,  799, 4017
    ITX_MULSUB_2D         7, 6, 0, 1, 4, 5, 4017,    4
    psubd                m5, m2, m7 ; t12a
    paddd                m2, m7     ; t8a
    psubd                m7, m3, m6 ; t13a
    paddd                m6, m3     ; t9a
    mova                 m0, [r3+8*16]
    mova                 m1, [r3+9*16]
    mova                 m4, [r3+10*16]
    mova                 m3, [o(clip_18b_min)]
    REPX     {pmaxsd x, m3}, m4, m5, m7, m0, m1, m2, m6
    pmaxsd               m3, [r3+11*16]
    mova          [r3+8*16], m3
    mova                 m3, [o(clip_18b_max)]
    REPX     {pminsd x, m3}, m4, m5, m7, m0, m1, m2, m6
    pminsd               m3, [r3+8*16]
    mova          [r3+8*16], m0
    mova          [r3+9*16], m1
    mova         [r3+10*16], m2
    mova         [r3+11*16], m6
    mova                 m0, [o(pd_2048)]
    ITX_MULSUB_2D         3, 4, 1, 2, 6, 0, 1567, 3784
    ITX_MULSUB_2D         5, 7, 1, 2, 6, 0,    6, 3784
    mova                 m0, [r3+7*16]      ;  t7a
    mova                 m2, [r3+6*16]      ;  t6a
    psubd                m1, m3, m0         ;  t7
    paddd                m0, m3             ;  out12
    paddd                m3, m4, m2         ; -out3
    psubd                m4, m2             ;  t6
    mova          [r3+7*16], m3
    mova                 m3, [r3+3*16]      ;  t15
    mova                 m2, [r3+2*16]      ;  t14
    paddd                m6, m5, m3         ; -out13
    psubd                m5, m3             ;  t15a
    psubd                m3, m7, m2         ;  t14a
    paddd                m2, m7             ;  out2
    mova          [r3+6*16], m2
    mova                 m7, [r3+0*16]      ;  t10a
    mova                 m2, [r3+1*16]      ;  t11a
    mova          [r3+0*16], m0
    mova          [r3+1*16], m6
    mova                 m6, [r3+11*16]
    psubd                m0, m6, m2         ;  t11
    paddd                m6, m2             ;  out14
    mova          [r3+2*16], m6
    mova                 m2, [r3+10*16]
    psubd                m6, m2, m7         ;  t10
    paddd                m2, m7             ; -out1
    mova                 m7, [r3+5*16]      ;  t3
    mova          [r3+5*16], m2
    mova         [r3+10*16], m1
    mova                 m1, [r3+9*16]
    psubd                m2, m1, m7         ;  t3a
    paddd                m1, m7             ; -out15
    mova          [r3+3*16], m1
    mova                 m1, [r3+4*16]      ;  t2
    mova                 m7, [r3+8*16]
    psubd                m7, m1             ;  t2a
    paddd                m1, [r3+8*16]      ;  out0
    mova          [r3+4*16], m1
    mova                 m1, [o(clip_18b_min)]
    REPX     {pmaxsd x, m1}, m0, m2, m3, m4, m5, m6, m7
    pmaxsd               m1, [r3+10*16]
    mova         [r3+10*16], m1
    mova                 m1, [o(clip_18b_max)]
    REPX     {pminsd x, m1}, m0, m2, m3, m4, m5, m6, m7
    pminsd               m1, [r3+10*16]
    mova         [r3+10*16], m1
    mova                 m1, [o(pd_2896)]
    REPX     {pmulld x, m1}, m0, m2, m3, m4, m5, m6, m7
    pmulld               m1, [r3+10*16]
    mova         [r3+11*16], m3
    psubd                m3, m4, m1         ; -out11 (unshifted)
    paddd                m4, m1             ;  out4  (unshifted)
    psubd                m1, m6, m0         ; -out9  (unshifted)
    paddd                m6, m0             ;  out6  (unshifted)
    psubd                m0, m7, m2         ;  out8  (unshifted)
    paddd                m7, m2             ; -out7  (unshifted)
    mova                 m2, [r3+11*16]
    mova         [r3+11*16], m5
    paddd                m5, m2             ; -out5  (unshifted)
    psubd                m2, [r3+11*16]     ;  out10 (unshifted)
    ; m0-3 contain out8-11 (unshifted), m4-7 contain out4-7 (unshifted)
    ; r[-4,3] contain out0-3 and out12-15
%endif
    ret
.main_part1:
%if ARCH_X86_64
    ITX_MULSUB_2D         1, 0, 8, 9, 10, 11,  995, 3973
    ITX_MULSUB_2D         3, 2, 8, 9, 10, 11, 2440, 3290
    ITX_MULSUB_2D         5, 4, 8, 9, 10, 11, 3513, 2106
    ITX_MULSUB_2D         7, 6, 8, 9, 10, 11, 4052,  601
    psubd                m8, m0, m4 ; t10a
    paddd                m0, m4     ; t2a
    psubd                m4, m1, m5 ; t11a
    paddd                m1, m5     ; t3a
    psubd                m5, m2, m6 ; t14a
    paddd                m2, m6     ; t6a
    psubd                m6, m3, m7 ; t15a
    paddd                m7, m3     ; t7a
    REPX    {pmaxsd x, m12}, m8, m4, m5, m6, m0, m1, m2, m7
    REPX    {pminsd x, m13}, m8, m4, m5, m6, m0, m1, m2, m7
    mova                m15, [o(pd_2276)]
    mova                m10, [o(pd_3406)]
    ITX_MULSUB_2D         8, 4, 3, 9, _, 11, 10, 15
    ITX_MULSUB_2D         6, 5, 3, 9, _, 11, 15, 10
    psubd                m3, m0, m2 ; t6
    paddd                m0, m2     ; t2
    psubd                m2, m1, m7 ; t7
    paddd                m1, m7     ; t3
    psubd                m7, m4, m6 ; t14a
    paddd                m4, m6     ; t10a
    psubd                m6, m8, m5 ; t15a
    paddd                m5, m8     ; t11a
    REPX    {pmaxsd x, m12}, m3, m2, m7, m6, m0, m1, m4, m5
    REPX    {pminsd x, m13}, m3, m2, m7, m6, m0, m1, m4, m5
    mova                m15, [o(pd_1567)]
    mova                m10, [o(pd_3784)]
    ITX_MULSUB_2D         2, 3, 8, 9, _, 11, 10, 15
    ITX_MULSUB_2D         6, 7, 8, 9, _, 11, 10, 15
    mova          [r3+0*16], m0
    mova          [r3+1*16], m1
    mova          [r3+4*16], m4
    mova          [r3+5*16], m5
    mova          [r3+2*16], m2
    mova          [r3+3*16], m3
    mova          [r3+6*16], m6
    mova          [r3+7*16], m7
%else
    mova          [r3+4*16], m0
    mova          [r3+5*16], m1
    mova          [r3+6*16], m2
    mova          [r3+7*16], m3
    mova                 m3, [o(pd_2048)]
    ITX_MULSUB_2D         5, 4, 0, 1, 2, 3, 3513, 2106
    ITX_MULSUB_2D         7, 6, 0, 1, 2, 3, 4052,  601
    mova          [r3+0*16], m4
    mova          [r3+1*16], m5
    mova          [r3+2*16], m6
    mova          [r3+3*16], m7
    mova                 m0, [r3+4*16]
    mova                 m1, [r3+5*16]
    mova                 m2, [r3+6*16]
    mova                 m7, [r3+7*16]
    ITX_MULSUB_2D         1, 0, 4, 5, 6, 3,  995, 3973
    ITX_MULSUB_2D         7, 2, 4, 5, 6, 3, 2440, 3290
    mova                 m4, [r3+0*16]
    mova                 m5, [r3+1*16]
    psubd                m6, m0, m4 ; t10a
    paddd                m0, m4     ; t2a
    mova          [r3+4*16], m6
    mova                 m6, [r3+2*16]
    mova                 m3, [r3+3*16]
    psubd                m4, m1, m5 ; t11a
    paddd                m1, m5     ; t3a
    psubd                m5, m2, m6 ; t14a
    paddd                m2, m6     ; t6a
    psubd                m6, m7, m3 ; t15a
    paddd                m7, m3     ; t7a
    mova                 m3, [o(clip_18b_min)]
    REPX     {pmaxsd x, m3}, m4, m5, m6, m0, m1, m2, m7
    pmaxsd               m3, [r3+4*16]
    mova          [r3+4*16], m3
    mova                 m3, [o(clip_18b_max)]
    REPX     {pminsd x, m3}, m4, m5, m6, m0, m1, m2, m7
    pminsd               m3, [r3+4*16]
    mova          [r3+4*16], m3
    psubd                m3, m0, m2 ; t6
    paddd                m0, m2     ; t2
    psubd                m2, m1, m7 ; t7
    paddd                m1, m7     ; t3
    mova          [r3+5*16], m1
    mova          [r3+6*16], m3
    mova          [r3+7*16], m2
    mova                 m1, [r3+4*16]
    mova          [r3+4*16], m0
    mova                 m3, [o(pd_2048)]
    ITX_MULSUB_2D         1, 4, 0, 7, 2, 3, 3406, 2276
    ITX_MULSUB_2D         6, 5, 0, 7, 2, 3, 2276,    2
    psubd                m7, m4, m6 ; t14a
    paddd                m4, m6     ; t10a
    psubd                m6, m1, m5 ; t15a
    paddd                m5, m1     ; t11a
    mova                 m1, [r3+5*16]
    mova                 m3, [r3+6*16]
    mova                 m2, [r3+7*16]
    mova                 m0, [o(clip_18b_min)]
    REPX     {pmaxsd x, m0}, m3, m2, m7, m6, m1, m4, m5
    pmaxsd               m0, [r3+4*16]
    mova          [r3+4*16], m0
    mova                 m0, [o(clip_18b_max)]
    REPX     {pminsd x, m0}, m3, m2, m7, m6, m1, m4, m5
    pminsd               m0, [r3+4*16]
    mova          [r3+4*16], m0
    mova          [r3+5*16], m1
    mova          [r3+0*16], m4
    mova          [r3+1*16], m5
    mova                 m0, [o(pd_2048)]
    ITX_MULSUB_2D         2, 3, 1, 4, 5, 0, 3784, 1567
    ITX_MULSUB_2D         6, 7, 1, 4, 5, 0,    5, 1567
    mova          [r3+6*16], m2
    mova          [r3+7*16], m3
    mova          [r3+2*16], m6
    mova          [r3+3*16], m7
%endif
    ret

.pass2:
    lea                  r4, [o(m_suffix(iadst_8x4_internal_8bpc, _ssse3).main)]
    jmp m(idct_16x4_internal_16bpc).pass2_loop

INV_TXFM_16X4_FN flipadst, dct
INV_TXFM_16X4_FN flipadst, adst
INV_TXFM_16X4_FN flipadst, flipadst
INV_TXFM_16X4_FN flipadst, identity

cglobal iflipadst_16x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
    lea                  r3, [rsp+gprsize]
    call m(iadst_16x4_internal_16bpc).main
%if ARCH_X86_64
    packssdw             m1, m0
    packssdw             m3, m2
    packssdw             m5, m4
    packssdw             m7, m6
    packssdw             m9, m8
    packssdw            m11, m10
    packssdw            m13, m12
    packssdw            m15, m14
    mova                 m0, m15
    mova                 m2, m13
    mova                 m4, m11
    mova                 m6, m9
    mova                 m8, m7
    mova                m10, m5
    mova                m12, m3
    mova                m14, m1
    jmp m(idct_16x4_internal_16bpc).transpose
%else
    mova [rsp+gprsize+4*16], m0
    mova [rsp+gprsize+5*16], m2
    mova [rsp+gprsize+6*16], m4
    mova [rsp+gprsize+7*16], m6
    pshufd               m6, [rsp+gprsize+ 8*16], q1032
    pshufd               m4, [rsp+gprsize+ 9*16], q1032
    pshufd               m2, [rsp+gprsize+10*16], q1032
    pshufd               m0, [rsp+gprsize+11*16], q1032
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova [rsp+gprsize+0*16], m0
    mova [rsp+gprsize+1*16], m1
    mova [rsp+gprsize+2*16], m2
    mova [rsp+gprsize+3*16], m3
    pshufd               m6, [rsp+gprsize+ 4*16], q1032
    pshufd               m4, [rsp+gprsize+ 5*16], q1032
    pshufd               m2, [rsp+gprsize+ 6*16], q1032
    pshufd               m0, [rsp+gprsize+ 7*16], q1032
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    jmp                tx2q
%endif

.pass2:
    lea                  r3, [strideq*3]
    lea                dstq, [dstq+r3]
    neg             strideq
    lea                  r4, [o(m_suffix(iadst_8x4_internal_8bpc, _ssse3).main)]
    jmp m(idct_16x4_internal_16bpc).pass2_loop

INV_TXFM_16X4_FN identity, dct
INV_TXFM_16X4_FN identity, adst
INV_TXFM_16X4_FN identity, flipadst
INV_TXFM_16X4_FN identity, identity

cglobal iidentity_16x4_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_64
    mova                m15, [o(pd_11586)]
    pmulld               m0, m15, [cq+ 0*16]
    pmulld               m1, m15, [cq+ 1*16]
    pmulld               m2, m15, [cq+ 2*16]
    pmulld               m3, m15, [cq+ 3*16]
    pmulld               m4, m15, [cq+ 4*16]
    pmulld               m5, m15, [cq+ 5*16]
    pmulld               m6, m15, [cq+ 6*16]
    pmulld               m7, m15, [cq+ 7*16]
    pmulld               m8, m15, [cq+ 8*16]
    pmulld               m9, m15, [cq+ 9*16]
    pmulld              m10, m15, [cq+10*16]
    pmulld              m11, m15, [cq+11*16]
    pmulld              m12, m15, [cq+12*16]
    pmulld              m13, m15, [cq+13*16]
    pmulld              m14, m15, [cq+14*16]
    pmulld              m15, [cq+15*16]
    mova         [cq+ 0*16], m15
    mova                m15, [o(pd_6144)]
    REPX     {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14
    paddd               m15, [cq+ 0*16]
    REPX     {psrad x, 13 }, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14, m15
    jmp m(idct_16x4_internal_16bpc).pack_transpose
%else
    add                  cq, 8*16
    mov                 r5d, 2
.loop_pass1:
    mova                 m7, [o(pd_11586)]
    pmulld               m0, m7, [cq+0*16]
    pmulld               m1, m7, [cq+1*16]
    pmulld               m2, m7, [cq+2*16]
    pmulld               m3, m7, [cq+3*16]
    pmulld               m4, m7, [cq+4*16]
    pmulld               m5, m7, [cq+5*16]
    pmulld               m6, m7, [cq+6*16]
    pmulld               m7, [cq+7*16]
    mova          [cq+7*16], m7
    mova                 m7, [o(pd_6144)]
    REPX      {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6
    paddd                m7, [cq+7*16]
    REPX      {psrad x, 13}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    dec                 r5d
    jz .end_pass1
    mova [rsp+gprsize+0*16], m0
    mova [rsp+gprsize+1*16], m1
    mova [rsp+gprsize+2*16], m2
    mova [rsp+gprsize+3*16], m3
    sub                  cq, 8*16
    jmp .loop_pass1
.end_pass1:
    jmp                tx2q
%endif

.pass2:
%if ARCH_X86_64
    mova                m12, [o(pw_1697x8)]
%endif
    lea                  r4, [o(.main)]
    jmp m(idct_16x4_internal_16bpc).pass2_loop
.main:
%if ARCH_X86_64
    pmulhrsw             m4, m0, m12
    pmulhrsw             m5, m1, m12
    pmulhrsw             m6, m2, m12
    pmulhrsw             m7, m3, m12
%else
    mova                 m7, [o(pw_1697x8)]
    pmulhrsw             m4, m0, m7
    pmulhrsw             m5, m1, m7
    pmulhrsw             m6, m2, m7
    pmulhrsw             m7, m3
%endif
    paddsw               m0, m4
    paddsw               m1, m5
    paddsw               m2, m6
    paddsw               m3, m7
    ret

%macro INV_TXFM_16X8_FN 2-3 0 ; type1, type2, eob_offset
%if ARCH_X86_64
    INV_TXFM_FN          %1, %2, %3, 16x8, 16, 0-8*16
%else
    INV_TXFM_FN          %1, %2, %3, 16x8, 8, 0-13*16
%endif
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 8
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
%if ARCH_X86_32
    add                 rsp, 1*16
%endif
    jmp m(inv_txfm_add_dct_dct_16x4_16bpc).dconly
%endif
%endmacro

INV_TXFM_16X8_FN dct, dct
INV_TXFM_16X8_FN dct, identity, 6
INV_TXFM_16X8_FN dct, adst
INV_TXFM_16X8_FN dct, flipadst

cglobal idct_16x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_64
    DECLARE_REG_TMP 6, 4, 6
%else
    mov [rsp+gprsize+12*16], r1
    DECLARE_REG_TMP 1, 4, 3
%endif
    lea                  t0, [o(.main)]
.loop_main:
%undef cmp
%if ARCH_X86_64
    xor                 r5d, r5d
    cmp                eobd, 10
    setge               r5b
%else
    mov                 r5d, 1
    cmp                eobd, 10
    sbb                 r5d, 0
%endif
    shl                 r5d, 4

    lea                  r3, [rsp+gprsize]
.loop_pass1:
    call                 t0
%if ARCH_X86_64
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova       [cq+4*32+r5], m8
    mova       [cq+5*32+r5], m9
    mova       [cq+6*32+r5], m10
    mova       [cq+7*32+r5], m11
%else
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova       [cq+4*32+r5], m0
    mova       [cq+5*32+r5], m1
    mova       [cq+6*32+r5], m2
    mova       [cq+7*32+r5], m3
    mova                 m0, [rsp+gprsize+ 8*16]
    mova                 m2, [rsp+gprsize+ 9*16]
    mova                 m4, [rsp+gprsize+10*16]
    mova                 m6, [rsp+gprsize+11*16]
%endif
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    pxor                 m7, m7
    REPX {mova [cq+x*32+r5], m7}, 8, 9, 10, 11, 12, 13, 14, 15
    test                r5d, r5d
    jz .end
    mova       [cq+0*32+r5], m0
    mova       [cq+1*32+r5], m1
    mova       [cq+2*32+r5], m2
    mova       [cq+3*32+r5], m3
    xor                 r5d, r5d
    jmp .loop_pass1
.end:

    jmp                tx2q
.main:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+ 1*32+r5]
    mova                 m1, [cq+ 3*32+r5]
    mova                 m2, [cq+ 5*32+r5]
    mova                 m3, [cq+ 7*32+r5]
    mova                 m4, [cq+ 9*32+r5]
    mova                 m5, [cq+11*32+r5]
    mova                 m6, [cq+13*32+r5]
    mova                 m7, [cq+15*32+r5]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_16x4_internal_16bpc).main_oddhalf

    mova                 m0, [cq+ 0*32+r5]
    mova                 m1, [cq+ 2*32+r5]
    mova                 m2, [cq+ 4*32+r5]
    mova                 m3, [cq+ 6*32+r5]
    mova                 m4, [cq+ 8*32+r5]
    mova                 m5, [cq+10*32+r5]
    mova                 m6, [cq+12*32+r5]
    mova                 m7, [cq+14*32+r5]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    call m(idct_16x4_internal_16bpc).round
%if ARCH_X86_64
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%endif
    ret

.pass2:
%if ARCH_X86_32
    mov             strideq, [rsp+gprsize+12*16]
%endif
    mov                 r4d, 2
.pass2_main:
%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
%endif
    lea                  r3, [strideq*3]
    jmp .loop_pass2_entry
.loop_pass2:
    mova                 m0, [cq+0*32+ 0]
    mova                 m1, [cq+1*32+ 0]
    mova                 m2, [cq+2*32+ 0]
    mova                 m3, [cq+3*32+ 0]
.loop_pass2_entry:
    mova                 m4, [cq+0*32+16]
    mova                 m5, [cq+1*32+16]
    mova                 m6, [cq+2*32+16]
    mova                 m7, [cq+3*32+16]
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(idct_8x8_internal_8bpc, _ssse3).main
    call m(idct_8x8_internal_16bpc).round2_and_write_8x8
%if ARCH_X86_64
%define mzero m9
%else
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+x*16], mzero}, 0, 1, 2, 3, 4, 5, 6, 7
    add                dstq, 16
    add                  cq, 4*32
    dec                 r4d
    jg .loop_pass2
    RET

INV_TXFM_16X8_FN adst, dct
INV_TXFM_16X8_FN adst, adst
INV_TXFM_16X8_FN adst, flipadst
INV_TXFM_16X8_FN adst, identity, 6

cglobal iadst_16x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_32
    mov [rsp+gprsize+12*16], r1
%endif
    lea                  t0, [o(.main)]
    jmp m(idct_16x8_internal_16bpc).loop_main

.main:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+ 2*32+r5]
    mova                 m1, [cq+13*32+r5]
    mova                 m2, [cq+ 6*32+r5]
    mova                 m3, [cq+ 9*32+r5]
    mova                 m4, [cq+10*32+r5]
    mova                 m5, [cq+ 5*32+r5]
    mova                 m6, [cq+14*32+r5]
    mova                 m7, [cq+ 1*32+r5]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(iadst_16x4_internal_16bpc).main_part1
    mova                 m0, [cq+ 0*32+r5]
    mova                 m1, [cq+15*32+r5]
    mova                 m2, [cq+ 4*32+r5]
    mova                 m3, [cq+11*32+r5]
    mova                 m4, [cq+ 8*32+r5]
    mova                 m5, [cq+ 7*32+r5]
    mova                 m6, [cq+12*32+r5]
    mova                 m7, [cq+ 3*32+r5]
%if ARCH_X86_32
    add                  r3, 8*16
%endif
    call m(idct_8x4_internal_16bpc).rect2_mul
%if ARCH_X86_32
    sub                  r3, 8*16
%endif
    call m(iadst_16x4_internal_16bpc).main_part2
    call m(iadst_16x4_internal_16bpc).round
%if ARCH_X86_64
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%endif
    ret

.pass2:
%if ARCH_X86_32
    mov             strideq, [rsp+gprsize+12*16]
%endif
    mov                 r4d, 2
%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
    mova                m11, [o(pw_m2048)]
%endif
    lea                  r3, [strideq*3]
    jmp .loop_pass2_entry
.loop_pass2:
    mova                 m0, [cq+0*32+ 0]
    mova                 m1, [cq+1*32+ 0]
    mova                 m2, [cq+2*32+ 0]
    mova                 m3, [cq+3*32+ 0]
.loop_pass2_entry:
    mova                 m4, [cq+0*32+16]
    mova                 m5, [cq+1*32+16]
    mova                 m6, [cq+2*32+16]
    mova                 m7, [cq+3*32+16]
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    call m_suffix(iadst_8x8_internal_8bpc, _ssse3).main
    call m_suffix(iadst_8x8_internal_8bpc, _ssse3).main_pass2_end
    call m(iadst_8x8_internal_16bpc).round2_and_write_8x8
%if ARCH_X86_64
%define mzero m9
%else
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+x*16], mzero}, 0, 1, 2, 3, 4, 5, 6, 7
    add                dstq, 16
    add                  cq, 4*32
    dec                 r4d
    jg .loop_pass2
    RET

INV_TXFM_16X8_FN flipadst, dct
INV_TXFM_16X8_FN flipadst, adst
INV_TXFM_16X8_FN flipadst, flipadst
INV_TXFM_16X8_FN flipadst, identity, 6

cglobal iflipadst_16x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_32
    mov [rsp+gprsize+12*16], r1
%endif
    lea                  t0, [o(.main)]
    jmp m(idct_16x8_internal_16bpc).loop_main
.main:
    call m(iadst_16x8_internal_16bpc).main
%if ARCH_X86_64
    pshufd               m1, m0, q1032
    pshufd               m3, m2, q1032
    pshufd               m5, m4, q1032
    pshufd               m7, m6, q1032
    pshufd               m0, m14, q1032
    pshufd               m2, m12, q1032
    pshufd               m4, m10, q1032
    pshufd               m6, m8, q1032
    mova                m14, m1
    mova                m12, m3
    mova                m10, m5
    mova                 m8, m7
%else
    pshufd               m1, m0, q1032
    pshufd               m3, m2, q1032
    pshufd               m5, m4, q1032
    pshufd               m7, m6, q1032
    pshufd               m0, [r3+11*16], q1032
    pshufd               m2, [r3+10*16], q1032
    pshufd               m4, [r3+9*16], q1032
    pshufd               m6, [r3+8*16], q1032
    mova          [r3+8*16], m7
    mova          [r3+9*16], m5
    mova         [r3+10*16], m3
    mova         [r3+11*16], m1
%endif
    ret

.pass2:
%if ARCH_X86_32
    mov             strideq, [rsp+gprsize+12*16]
%endif
    lea                dstq, [dstq+strideq*8]
    neg             strideq
    add                dstq, strideq
%if ARCH_X86_32
    mov [rsp+gprsize+12*16], strideq
%endif
    jmp m(iadst_16x8_internal_16bpc).pass2

INV_TXFM_16X8_FN identity, dct, -54
INV_TXFM_16X8_FN identity, adst, -54
INV_TXFM_16X8_FN identity, flipadst, -54
INV_TXFM_16X8_FN identity, identity

cglobal iidentity_16x8_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_32
    mov [rsp+gprsize+12*16], r1
%endif
    lea                  t0, [o(.main)]
    jmp m(idct_16x8_internal_16bpc).loop_main
.main:
%if ARCH_X86_64
    mova                m15, [o(pd_2896)]
    pmulld               m0, m15, [cq+ 0*32+r5]
    pmulld               m1, m15, [cq+ 1*32+r5]
    pmulld               m2, m15, [cq+ 2*32+r5]
    pmulld               m3, m15, [cq+ 3*32+r5]
    pmulld               m4, m15, [cq+ 4*32+r5]
    pmulld               m5, m15, [cq+ 5*32+r5]
    pmulld               m6, m15, [cq+ 6*32+r5]
    pmulld               m7, m15, [cq+ 7*32+r5]
    pmulld               m8, m15, [cq+ 8*32+r5]
    pmulld               m9, m15, [cq+ 9*32+r5]
    pmulld              m10, m15, [cq+10*32+r5]
    pmulld              m11, m15, [cq+11*32+r5]
    pmulld              m12, m15, [cq+12*32+r5]
    pmulld              m13, m15, [cq+13*32+r5]
    pmulld              m14, m15, [cq+14*32+r5]
    pmulld              m15, [cq+15*32+r5]
    mova               [r3], m15
    mova                m15, [o(pd_2048)]
    REPX     {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14
    paddd               m15, [r3]
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14, m15
    mova               [r3], m15
    mova                m15, [o(pd_11586)]
    REPX    {pmulld x, m15}, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14
    pmulld              m15, [r3]
    mova               [r3], m15
    mova                m15, [o(pd_6144)]
    REPX     {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14
    paddd               m15, [r3]
    REPX     {psrad x, 13 }, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14, m15
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%else
    mova                 m0, [cq+ 0*32+r5]
    mova                 m1, [cq+ 1*32+r5]
    mova                 m2, [cq+ 2*32+r5]
    mova                 m3, [cq+ 3*32+r5]
    mova                 m4, [cq+ 4*32+r5]
    mova                 m5, [cq+ 5*32+r5]
    mova                 m6, [cq+ 6*32+r5]
    mova                 m7, [cq+ 7*32+r5]
    call m(idct_8x4_internal_16bpc).rect2_mul
    mova               [r3], m7
    mova                 m7, [o(pd_11586)]
    REPX      {pmulld x, m7}, m0, m1, m2, m3, m4, m5, m6
    pmulld               m7, [r3]
    mova               [r3], m7
    mova                 m7, [o(pd_6144)]
    REPX      {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6
    paddd                m7, [r3]
    REPX      {psrad x, 13}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    mova         [r3+ 8*16], m0
    mova         [r3+ 9*16], m2
    mova         [r3+10*16], m4
    mova         [r3+11*16], m6
    mova                 m0, [cq+ 8*32+r5]
    mova                 m1, [cq+ 9*32+r5]
    mova                 m2, [cq+10*32+r5]
    mova                 m3, [cq+11*32+r5]
    mova                 m4, [cq+12*32+r5]
    mova                 m5, [cq+13*32+r5]
    mova                 m6, [cq+14*32+r5]
    mova                 m7, [cq+15*32+r5]
    call m(idct_8x4_internal_16bpc).rect2_mul
    mova               [r3], m7
    mova                 m7, [o(pd_11586)]
    REPX      {pmulld x, m7}, m0, m1, m2, m3, m4, m5, m6
    pmulld               m7, [r3]
    mova               [r3], m7
    mova                 m7, [o(pd_6144)]
    REPX      {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6
    paddd                m7, [r3]
    REPX      {psrad x, 13}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
%endif
    ret
.pass2:
%if ARCH_X86_32
    mov             strideq, [rsp+gprsize+12*16]
%endif
    mov                 r4d, 2
%if ARCH_X86_64
    mova                 m8, [o(pw_4096)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
%endif
    lea                  r3, [strideq*3]
    jmp .loop_pass2_entry
.loop_pass2:
    mova                 m0, [cq+0*32+ 0]
    mova                 m1, [cq+1*32+ 0]
    mova                 m2, [cq+2*32+ 0]
    mova                 m3, [cq+3*32+ 0]
.loop_pass2_entry:
    mova                 m4, [cq+0*32+16]
    mova                 m5, [cq+1*32+16]
    mova                 m6, [cq+2*32+16]
    mova                 m7, [cq+3*32+16]
%if ARCH_X86_64
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
%else
    mova      [rsp+gprsize], m7
    mova                 m7, [o(pw_4096)]
    call m(idct_8x8_internal_16bpc).round4_and_write_8x8
%endif
%if ARCH_X86_64
%define mzero m9
%else
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+x*16], mzero}, 0, 1, 2, 3, 4, 5, 6, 7
    add                dstq, 16
    add                  cq, 4*32
    dec                 r4d
    jg .loop_pass2
    RET

%macro INV_TXFM_16X16_FN 2-3 2d ; type1, type2, eob_tbl_suffix
%if ARCH_X86_64
    INV_TXFM_FN          %1, %2, tbl_16x16_%3, 16x16, 16, 0-(16+WIN64)*16
%else
    INV_TXFM_FN          %1, %2, tbl_16x16_%3, 16x16, 8, 0-17*16
%endif
%ifidn %1_%2, dct_dct
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 16
    add                 r5d, 640
    sar                 r5d, 10
    add                 rsp, (5+ARCH_X86_64*3+WIN64)*16
    jmp m(inv_txfm_add_dct_dct_16x4_16bpc).dconly2
%endif
%endmacro

INV_TXFM_16X16_FN dct, dct
INV_TXFM_16X16_FN dct, identity, v
INV_TXFM_16X16_FN dct, adst
INV_TXFM_16X16_FN dct, flipadst

cglobal idct_16x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if ARCH_X86_64
    DECLARE_REG_TMP       6, 7
%if WIN64
    mov [rsp+16*16+gprsize], r7
%endif
%elif ARCH_X86_32
    DECLARE_REG_TMP       1, 6
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(.main)]
.pass1_full:
%undef cmp
    mov                 t1d, 4
.zero_loop:
    dec                 t1d
    cmp                eobb, byte [r5+t1]
    jb .zero_loop
    mov                 r5d, t1d
    shl                 r5d, 4
%if ARCH_X86_32
    ; restore pic-ptr
    mov                  r6, [rsp+16*16+2*gprsize]
%endif
    ; setup stack pointer
    lea                  r3, [rsp+gprsize]
.loop_pass1:
    call                 t0
%if ARCH_X86_64
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova       [cq+4*64+r5], m8
    mova       [cq+5*64+r5], m9
    mova       [cq+6*64+r5], m10
    mova       [cq+7*64+r5], m11
%else
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova       [cq+4*64+r5], m0
    mova       [cq+5*64+r5], m1
    mova       [cq+6*64+r5], m2
    mova       [cq+7*64+r5], m3
    mova                 m0, [rsp+gprsize+ 8*16]
    mova                 m2, [rsp+gprsize+ 9*16]
    mova                 m4, [rsp+gprsize+10*16]
    mova                 m6, [rsp+gprsize+11*16]
%endif
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova       [cq+0*64+r5], m0
    mova       [cq+1*64+r5], m1
    mova       [cq+2*64+r5], m2
    mova       [cq+3*64+r5], m3
    pxor                 m0, m0
    REPX {mova [cq+x*64+r5], m0}, 8, 9, 10, 11, 12, 13, 14, 15
    sub                 r5d, 16
    jge .loop_pass1

%if ARCH_X86_32
    ; restore pic-ptr
    mov                  r1, [rsp+16*16+1*gprsize]
%endif
    jmp                tx2q
.main:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif

    mova                 m0, [cq+ 1*64+r5]
    mova                 m1, [cq+ 3*64+r5]
    mova                 m2, [cq+ 5*64+r5]
    mova                 m3, [cq+ 7*64+r5]
    mova                 m4, [cq+ 9*64+r5]
    mova                 m5, [cq+11*64+r5]
    mova                 m6, [cq+13*64+r5]
    mova                 m7, [cq+15*64+r5]
    call m(idct_16x4_internal_16bpc).main_oddhalf

    mova                 m0, [cq+ 0*64+r5]
    mova                 m1, [cq+ 2*64+r5]
    mova                 m2, [cq+ 4*64+r5]
    mova                 m3, [cq+ 6*64+r5]
    mova                 m4, [cq+ 8*64+r5]
    mova                 m5, [cq+10*64+r5]
    mova                 m6, [cq+12*64+r5]
    mova                 m7, [cq+14*64+r5]
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    call .round
%if ARCH_X86_64
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%endif
    ret
.round:
%if ARCH_X86_64
    REPX    {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX    {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    psrld                m8, m11, 10        ; 2
    REPX      {paddd x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    mova                 m8, [r3+1*16]
    mova                 m9, [r3+2*16]
    mova                m10, [r3+3*16]
    mova                m11, [r3+4*16]
    mova                m12, [r3+5*16]
    mova                m13, [r3+6*16]
    mova                m14, [r3+7*16]
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
    psubd                m8, m7, [r3+0*16] ; out8
    paddd                m7, [r3+0*16]     ; out7
    REPX       {psrad x, 2}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                             m8,  m9,  m10, m11, m12, m13, m14, m15
    ; and out0-15 is now in m0-15
%else
    mova         [r3+ 0*16], m0
    mova                 m0, [o(clip_18b_min)]
    REPX     {pmaxsd x, m0}, m1, m2, m3, m4, m5, m6, m7
    pmaxsd               m0, [r3+ 0*16]
    mova         [r3+ 0*16], m7
    mova                 m7, [o(clip_18b_max)]
    REPX     {pminsd x, m7}, m0, m1, m2, m3, m4, m5, m6
    pminsd               m7, [r3+ 0*16]
    mova         [r3+ 0*16], m0
    mova                 m0, [o(pd_2)]
    REPX      {paddd x, m0}, m1, m2, m3, m4, m5, m6, m7
    paddd                m0, [r3+ 0*16]
    mova         [r3+ 0*16], m0
    mova         [r3+ 1*16], m1
    mova         [r3+ 2*16], m2
    mova                 m1, [r3+11*16]
    mova                 m2, [r3+10*16]
    psubd                m0, m7, m1
    paddd                m7, m1
    psubd                m1, m6, m2
    paddd                m6, m2
    REPX       {psrad x, 2}, m0, m1, m6, m7
    packssdw             m0, m1     ; out8-9
    packssdw             m6, m7     ; out6-7
    mova         [r3+11*16], m6
    mova                 m1, [r3+9*16]
    mova                 m7, [r3+8*16]
    psubd                m2, m5, m1
    paddd                m5, m1
    psubd                m1, m4, m7
    paddd                m4, m7
    REPX       {psrad x, 2}, m2, m1, m4, m5
    packssdw             m2, m1     ; out10-11
    packssdw             m4, m5     ; out4-5
    mova                 m1, [r3+2*16]
    mova         [r3+10*16], m4
    mova                 m6, [r3+7*16]
    mova                 m7, [r3+6*16]
    psubd                m4, m3, m6
    paddd                m3, m6
    psubd                m6, m1, m7
    paddd                m1, m7
    REPX       {psrad x, 2}, m4, m6, m1, m3
    packssdw             m4, m6     ; out12-13
    packssdw             m1, m3     ; out2-3
    mova                 m3, [r3+1*16]
    mova          [r3+9*16], m1
    mova                 m1, [r3+0*16]
    mova                 m5, [r3+5*16]
    mova                 m7, [r3+4*16]
    psubd                m6, m3, m5
    paddd                m3, m5
    psubd                m5, m1, m7
    paddd                m1, m7
    REPX       {psrad x, 2}, m6, m5, m1, m3
    packssdw             m6, m5     ; out14-15
    packssdw             m1, m3     ; out0-1
    mova          [r3+8*16], m1
%endif
    ret

.pass2:
%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
    mov                  r7, dstq
%else
    mov [rsp+2*gprsize+16*16], dstq
%endif
    lea                  r3, [strideq*3]
    mov                 r4d, 2
.loop_pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    mova                 m0, [cq+0*64+ 0]
    mova                 m1, [cq+2*64+ 0]
    mova                 m2, [cq+0*64+16]
    mova                 m3, [cq+2*64+16]
    mova                 m4, [cq+0*64+32]
    mova                 m5, [cq+2*64+32]
    mova                 m6, [cq+0*64+48]
    mova                 m7, [cq+2*64+48]
    call m_suffix(idct_8x8_internal_8bpc, _ssse3).main
    mova [rsp+gprsize+3*16], m0
    mova [rsp+gprsize+4*16], m1
    mova [rsp+gprsize+5*16], m2
    mova [rsp+gprsize+6*16], m3
    mova [rsp+gprsize+7*16], m4
    mova [rsp+gprsize+8*16], m5
    mova [rsp+gprsize+9*16], m6
    ; m7 is already stored in [rsp+gprsize+0*16]
    mova                 m0, [cq+1*64+ 0]
    mova                 m1, [cq+3*64+ 0]
    mova                 m2, [cq+1*64+16]
    mova                 m3, [cq+3*64+16]
    mova                 m4, [cq+1*64+32]
    mova                 m5, [cq+3*64+32]
    mova                 m6, [cq+1*64+48]
    mova                 m7, [cq+3*64+48]
    call m_suffix(idct_16x8_internal_8bpc, _ssse3).main

    ; out0-7 is in rsp+gprsize+3-10*mmsize
    ; out8-14 is in m0-6, and out15 is in m7 as well as rsp+gprsize+0*mmsize

%if ARCH_X86_64
    lea                dstq, [r7+strideq*8]
%else
    mov                dstq, [rsp+2*gprsize+16*16]
    lea                dstq, [dstq+strideq*8]
%endif
    call m(idct_8x8_internal_16bpc).round2_and_write_8x8
%if ARCH_X86_64
    mov                dstq, r7
%else
    mov                dstq, [rsp+2*gprsize+16*16]
%endif
    mova                 m0, [rsp+gprsize+ 3*16]
    mova                 m1, [rsp+gprsize+ 4*16]
    mova                 m2, [rsp+gprsize+ 5*16]
    mova                 m3, [rsp+gprsize+ 6*16]
    mova                 m4, [rsp+gprsize+ 7*16]
    mova                 m5, [rsp+gprsize+ 8*16]
    mova                 m6, [rsp+gprsize+ 9*16]
    mova                 m7, [rsp+gprsize+10*16]
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
%if ARCH_X86_64
    add                  r7, 16
%define mzero m9
%else
    add dword [rsp+2*gprsize+16*16], 16
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+x*16], mzero}, 0, 1, 2, 3, 4, 5, 6, 7
    add                  cq, 64*4
    REPX {mova [cq+x*16], mzero}, -8, -7, -6, -5, -4, -3, -2, -1
%undef mzero
    dec                 r4d
    jg .loop_pass2
%if WIN64
    mov                  r7, [rsp+16*16+gprsize]
%endif
    RET

INV_TXFM_16X16_FN adst, dct
INV_TXFM_16X16_FN adst, adst
INV_TXFM_16X16_FN adst, flipadst

cglobal iadst_16x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if WIN64
    mov [rsp+16*16+gprsize], r7
%elif ARCH_X86_32
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(.main)]
    jmp m(idct_16x16_internal_16bpc).pass1_full

.main:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+ 2*64+r5]
    mova                 m1, [cq+13*64+r5]
    mova                 m2, [cq+ 6*64+r5]
    mova                 m3, [cq+ 9*64+r5]
    mova                 m4, [cq+10*64+r5]
    mova                 m5, [cq+ 5*64+r5]
    mova                 m6, [cq+14*64+r5]
    mova                 m7, [cq+ 1*64+r5]
    call m(iadst_16x4_internal_16bpc).main_part1
    mova                 m0, [cq+ 0*64+r5]
    mova                 m1, [cq+15*64+r5]
    mova                 m2, [cq+ 4*64+r5]
    mova                 m3, [cq+11*64+r5]
    mova                 m4, [cq+ 8*64+r5]
    mova                 m5, [cq+ 7*64+r5]
    mova                 m6, [cq+12*64+r5]
    mova                 m7, [cq+ 3*64+r5]
    call m(iadst_16x4_internal_16bpc).main_part2
    call .round
%if ARCH_X86_64
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%endif
    ret
.round:
%if ARCH_X86_64
    pcmpeqd              m8, m8         ; -1
    mova                m15, [o(pd_10240)]
    psrld               m14, 10         ; +2
    psubd               m13, m14, m8    ; +3
    REPX     {pxor  x, m8 }, m1, m3, m5, m7
    REPX     {paddd x, m14}, m0, m2
    REPX     {paddd x, m13}, m1, m3
    REPX     {paddd x, m15}, m4, m5, m6, m7
    paddd               m13, m15, m8    ; +10239
    paddd                m8, m15, m9
    psubd                m9, m13, m10
    paddd               m10, m15, m11
    psubd               m11, m13, m12
    paddd               m12, m14, [r3+3*16]
    psubd               m13, m14, [r3+2*16]
    psubd               m15, m14, [r3+0*16]
    paddd               m14, [r3+1*16]
    REPX      {psrad x, 2 }, m0,  m1,  m2,  m3,  m12, m13, m14, m15
    REPX      {psrad x, 14}, m4,  m5,  m6,  m7,  m8,  m9,  m10, m11
%else
    mova          [r3+8*16], m1
    mova          [r3+9*16], m3
    mova                 m3, [o(pd_10240)]
    pcmpeqd              m1, m1
    REPX      {pxor  x, m1}, m5, m7
    REPX      {paddd x, m3}, m4, m5, m6, m7
    REPX      {psrad x, 14}, m4, m5, m6, m7
    packssdw             m4, m5
    packssdw             m6, m7
    mova         [r3+10*16], m4
    mova         [r3+11*16], m6
    mova                 m4, [r3+4*16]
    mova                 m5, [r3+5*16]
    mova                 m6, [r3+6*16]
    mova                 m7, [r3+7*16]
    mova                 m3, [o(pd_2)]
    REPX      {pxor  x, m1}, m5, m7
    REPX      {paddd x, m3}, m4, m6
    psubd                m3, m1
    REPX      {paddd x, m3}, m5, m7
    REPX      {psrad x, 2 }, m4, m5, m6, m7
    packssdw             m4, m5
    packssdw             m6, m7
    mova                 m5, [r3+8*16]
    mova                 m7, [r3+9*16]
    mova          [r3+8*16], m4
    mova          [r3+9*16], m6
    mova                 m3, [o(pd_10240)]
    REPX      {pxor  x, m1}, m5, m7
    REPX      {paddd x, m3}, m0, m5, m2, m7
    REPX      {psrad x, 14}, m0, m5, m2, m7
    packssdw             m0, m5
    packssdw             m2, m7
    mova                 m4, [r3+0*16]
    mova                 m5, [r3+1*16]
    mova                 m6, [r3+2*16]
    mova                 m7, [r3+3*16]
    mova                 m3, [o(pd_2)]
    REPX      {pxor  x, m1}, m5, m7
    REPX      {paddd x, m3}, m4, m6
    psubd                m3, m1
    REPX      {paddd x, m3}, m5, m7
    REPX      {psrad x, 2 }, m4, m5, m6, m7
    packssdw             m4, m5
    packssdw             m6, m7
%endif
    ret
.pass2:
%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    mova                m11, [o(pw_m2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
    mov                  r7, dstq
%else
    mov [rsp+2*gprsize+16*16], dstq
%endif
    lea                  r3, [strideq*3]
    mov                 r4d, 2
.loop_pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    mova                 m0, [cq+0*64+32]
    mova                 m1, [cq+1*64+32]
    mova                 m2, [cq+2*64+16]
    mova                 m3, [cq+3*64+16]
    mova                 m4, [cq+0*64+ 0]
    mova                 m5, [cq+1*64+ 0]
    mova                 m6, [cq+2*64+48]
    mova                 m7, [cq+3*64+48]
    mova [rsp+gprsize+3*16], m0
    mova [rsp+gprsize+4*16], m1
    mova [rsp+gprsize+5*16], m2
    mova [rsp+gprsize+6*16], m3
    mova [rsp+gprsize+7*16], m4
    mova [rsp+gprsize+8*16], m5
    mova [rsp+gprsize+9*16], m6
    mova [rsp+gprsize+10*16], m7
    mova                 m0, [cq+2*64+ 0]
    mova                 m1, [cq+3*64+ 0]
    mova                 m2, [cq+0*64+16]
    mova                 m3, [cq+1*64+16]
    mova                 m4, [cq+2*64+32]
    mova                 m5, [cq+3*64+32]
    mova                 m6, [cq+0*64+48]
    mova                 m7, [cq+1*64+48]
    call m_suffix(iadst_16x8_internal_8bpc, _ssse3).main
    call m_suffix(iadst_16x8_internal_8bpc, _ssse3).main_pass2_end

    ; out0-7 is in rsp+gprsize+3-10*mmsize
    ; out8-14 is in m0-6, and out15 is in m7 as well as rsp+gprsize+0*mmsize

%if ARCH_X86_64
    lea                dstq, [r7+strideq*8]
%else
    mov                dstq, [rsp+2*gprsize+16*16]
    lea                dstq, [dstq+strideq*8]
%endif
    call m(iadst_8x8_internal_16bpc).round2_and_write_8x8
%if ARCH_X86_64
    mov                dstq, r7
%else
    mov                dstq, [rsp+2*gprsize+16*16]
%endif
    mova                 m0, [rsp+gprsize+ 3*16]
    mova                 m1, [rsp+gprsize+ 4*16]
    mova                 m2, [rsp+gprsize+ 5*16]
    mova                 m3, [rsp+gprsize+ 6*16]
    mova                 m4, [rsp+gprsize+ 7*16]
    mova                 m5, [rsp+gprsize+ 8*16]
    mova                 m6, [rsp+gprsize+ 9*16]
    mova                 m7, [rsp+gprsize+10*16]
    call m(iadst_8x8_internal_16bpc).round1_and_write_8x8
%if ARCH_X86_64
    add                  r7, 16
%define mzero m9
%else
    add dword [rsp+2*gprsize+16*16], 16
%define mzero m7
    pxor                 m7, m7
%endif
    REPX {mova [cq+x*16], mzero}, 0, 1, 2, 3, 4, 5, 6, 7
    add                  cq, 64*4
    REPX {mova [cq+x*16], mzero}, -8, -7, -6, -5, -4, -3, -2, -1
%undef mzero
    dec                 r4d
    jg .loop_pass2
%if WIN64
    mov                  r7, [rsp+16*16+gprsize]
%endif
    RET

INV_TXFM_16X16_FN flipadst, dct
INV_TXFM_16X16_FN flipadst, adst
INV_TXFM_16X16_FN flipadst, flipadst

cglobal iflipadst_16x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if WIN64
    mov [rsp+16*16+gprsize], r7
%elif ARCH_X86_32
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(.main)]
    jmp m(idct_16x16_internal_16bpc).pass1_full

.main:
    call m(iadst_16x16_internal_16bpc).main
%if ARCH_X86_64
    mova                 m1, m0
    mova                 m3, m2
    mova                 m5, m4
    mova                 m7, m6
    pshufd               m0, m14, q1032
    pshufd               m2, m12, q1032
    pshufd               m4, m10, q1032
    pshufd               m6, m8, q1032
    pshufd               m8, m7, q1032
    pshufd              m10, m5, q1032
    pshufd              m12, m3, q1032
    pshufd              m14, m1, q1032
%else
    pshufd               m1, m0, q1032
    pshufd               m3, m2, q1032
    pshufd               m5, m4, q1032
    pshufd               m7, m6, q1032
    pshufd               m0, [r3+11*16], q1032
    pshufd               m2, [r3+10*16], q1032
    pshufd               m4, [r3+9*16], q1032
    pshufd               m6, [r3+8*16], q1032
    mova         [r3+11*16], m1
    mova         [r3+10*16], m3
    mova         [r3+ 9*16], m5
    mova         [r3+ 8*16], m7
%endif
    ret

.pass2:
    lea                  r3, [strideq*3]
    lea                  r3, [r3*5]
    add                dstq, r3
    neg             strideq
    jmp m(iadst_16x16_internal_16bpc).pass2

INV_TXFM_16X16_FN identity, dct, h
INV_TXFM_16X16_FN identity, identity

cglobal iidentity_16x16_internal_16bpc, 0, 0, 0, dst, stride, c, eob, tx2
%if WIN64
    mov [rsp+16*16+gprsize], r7
%elif ARCH_X86_32
    mov [rsp+16*16+gprsize*1], r1
    mov [rsp+16*16+gprsize*2], r6
%endif
    lea                  t0, [o(.main)]
    jmp m(idct_16x16_internal_16bpc).pass1_full

.main:
%if ARCH_X86_64
    mova                m15, [o(pd_11586)]
    pmulld               m0, m15, [cq+ 0*64+r5]
    pmulld               m1, m15, [cq+ 1*64+r5]
    pmulld               m2, m15, [cq+ 2*64+r5]
    pmulld               m3, m15, [cq+ 3*64+r5]
    pmulld               m4, m15, [cq+ 4*64+r5]
    pmulld               m5, m15, [cq+ 5*64+r5]
    pmulld               m6, m15, [cq+ 6*64+r5]
    pmulld               m7, m15, [cq+ 7*64+r5]
    pmulld               m8, m15, [cq+ 8*64+r5]
    pmulld               m9, m15, [cq+ 9*64+r5]
    pmulld              m10, m15, [cq+10*64+r5]
    pmulld              m11, m15, [cq+11*64+r5]
    pmulld              m12, m15, [cq+12*64+r5]
    pmulld              m13, m15, [cq+13*64+r5]
    pmulld              m14, m15, [cq+14*64+r5]
    pmulld              m15, [cq+15*64+r5]
    mova               [r3], m15
    mova                m15, [o(pd_10240)]
    REPX     {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14
    paddd               m15, [r3]
    REPX     {psrad x, 14 }, m0, m1, m2, m3, m4, m5, m6, m7, \
                         m8, m9, m10, m11, m12, m13, m14, m15
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%else
    mova                 m7, [o(pd_11586)]
    pmulld               m0, m7, [cq+ 0*64+r5]
    pmulld               m1, m7, [cq+ 1*64+r5]
    pmulld               m2, m7, [cq+ 2*64+r5]
    pmulld               m3, m7, [cq+ 3*64+r5]
    pmulld               m4, m7, [cq+ 4*64+r5]
    pmulld               m5, m7, [cq+ 5*64+r5]
    pmulld               m6, m7, [cq+ 6*64+r5]
    pmulld               m7, [cq+ 7*64+r5]
    mova               [r3], m7
    mova                 m7, [o(pd_10240)]
    REPX      {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6
    paddd                m7, [r3]
    REPX      {psrad x, 14}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    mova          [r3+8*16], m0
    mova          [r3+9*16], m2
    mova         [r3+10*16], m4
    mova         [r3+11*16], m6
    mova                 m7, [o(pd_11586)]
    pmulld               m0, m7, [cq+ 8*64+r5]
    pmulld               m1, m7, [cq+ 9*64+r5]
    pmulld               m2, m7, [cq+10*64+r5]
    pmulld               m3, m7, [cq+11*64+r5]
    pmulld               m4, m7, [cq+12*64+r5]
    pmulld               m5, m7, [cq+13*64+r5]
    pmulld               m6, m7, [cq+14*64+r5]
    pmulld               m7, [cq+15*64+r5]
    mova               [r3], m7
    mova                 m7, [o(pd_10240)]
    REPX      {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6
    paddd                m7, [r3]
    REPX      {psrad x, 14}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
%endif
    ret

.pass2:
%if ARCH_X86_64
    mova                 m4, [o(pw_2048)]
    mova                 m5, [o(pixel_10bpc_max)]
    pxor                 m6, m6
    mova                 m7, [o(pw_1697x16)]
    mov                  r7, dstq
%else
    mov [rsp+2*gprsize+16*16], dstq
%endif
    mov                 r5d, 4
    lea                  r3, [strideq*3]
.pass2_loop:
    mova                 m0, [cq+0*64+0]
    mova                 m1, [cq+1*64+0]
    mova                 m2, [cq+2*64+0]
    mova                 m3, [cq+3*64+0]
    call m(iidentity_8x16_internal_16bpc).main
%if ARCH_X86_64
    call m(idct_8x4_internal_16bpc).round1_and_write_8x4
%else
    call m(idct_8x4_internal_16bpc).round2_and_write_8x4
%endif
    REPX {mova [cq+x*16], m6}, 0, 4, 8, 12
    add                  cq, 16
    lea                dstq, [dstq+strideq*4]
    dec                 r5w
    jg .pass2_loop
    add                  cq, 64*3
    btc                 r5d, 16
    jc .end
%if ARCH_X86_64
    lea                dstq, [r7+16]
%else
    mov                dstq, [rsp+2*gprsize+16*16]
    add                dstq, 16
%endif
    add                 r5d, 4
    jmp .pass2_loop
.end:
%if WIN64
    mov                  r7, [rsp+16*16+gprsize]
%endif
    RET

cglobal inv_txfm_add_identity_identity_8x32_16bpc, 4, 7, 8, dst, stride, c, eob
%if ARCH_X86_32
    LEA                  r6, $$
%endif
    mova                 m5, [o(pw_5)]
    mova                 m7, [o(pixel_10bpc_max)]
    pxor                 m6, m6
    mov                 r5d, eobd
    add                eobb, 21
    cmovc              eobd, r5d ; 43, 107, 171 -> 64, 128, 192
    lea                  r4, [strideq*3]
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
    add                  cq, 16
    lea                dstq, [dstq+strideq*4]
    btc                eobd, 16
    jnc .loop
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
    paddw                m0, [dstq+strideq*0]
    paddw                m1, [dstq+strideq*1]
    paddw                m2, [dstq+strideq*2]
    paddw                m3, [dstq+r4       ]
    REPX     {pmaxsw x, m6}, m0, m1, m2, m3
    REPX     {pminsw x, m7}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+r4       ], m3
    ret

cglobal inv_txfm_add_identity_identity_32x8_16bpc, 4, 7, 8, dst, stride, c, eob
%if ARCH_X86_32
    LEA                  r6, $$
%endif
    mova                 m5, [o(pw_4096)]
    mova                 m7, [o(pixel_10bpc_max)]
    pxor                 m6, m6
    mov                 r4d, eobd
    add                eobb, 21
    cmovc              eobd, r4d
    lea                  r4, [strideq*3]
    mov                  r5, dstq
.loop:
    mova                 m0, [cq+32*0]
    packssdw             m0, [cq+32*1]
    mova                 m1, [cq+32*2]
    packssdw             m1, [cq+32*3]
    mova                 m2, [cq+32*4]
    packssdw             m2, [cq+32*5]
    mova                 m3, [cq+32*6]
    packssdw             m3, [cq+32*7]
    REPX {mova [cq+32*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
    REPX   {pmulhrsw x, m5}, m0, m1, m2, m3
    call m(inv_txfm_add_identity_identity_8x32_16bpc).main
    lea                dstq, [dstq+strideq*4]
    add                  cq, 16
    btc                eobd, 16
    jnc .loop
    add                  cq, 32*8-32
    add                  r5, 16
    mov                dstq, r5
    sub                eobd, 64
    jge .loop
    RET

cglobal inv_txfm_add_identity_identity_16x32_16bpc, 4, 7, 12, dst, stride, c, eob
%if ARCH_X86_32
    LEA                  r6, $$
%else
    mova                 m8, [o(pw_2896x8)]
    mova                 m9, [o(pw_1697x16)]
    mova                m11, [o(pw_8192)]
%endif
    mova                 m7, [o(pixel_10bpc_max)]
    lea                  r4, [strideq*3]
    pxor                 m6, m6
%if ARCH_X86_64
    paddw               m10, m11, m11 ; pw_16384
%endif
    mov                  r5, dstq
    call .main
    sub                eobd, 36
    jl .ret
    add                  cq, 128*8-32
    lea                dstq, [r5+16]
    call .main
    sub                  cq, 128*8
    lea                dstq, [r5+strideq*8]
    mov                  r5, dstq
    call .main
    sub                eobd, 107 ; eob < 143
    jl .ret
    add                  cq, 128*8-32
    lea                dstq, [r5+16]
    call .main
    sub                  cq, 128*8
    lea                dstq, [r5+strideq*8]
    mov                  r5, dstq
    call .main
    sub                eobd, 128 ; eob < 271
    jl .ret
    add                  cq, 128*8-32
    lea                dstq, [r5+16]
    call .main
    sub                  cq, 128*8
    lea                dstq, [r5+strideq*8]
    mov                  r5, dstq
    call .main
    sub                eobd, 128 ; eob < 399
    jl .ret
    add                  cq, 128*8-32
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
%if ARCH_X86_64
    REPX  {pmulhrsw x, m8 }, m0, m1, m2, m3
    pmulhrsw             m4, m9, m0
    pmulhrsw             m5, m9, m1
    REPX  {pmulhrsw x, m10}, m4, m5
%else
    mova                 m6, [o(pw_2896x8)]
    REPX  {pmulhrsw x, m6 }, m0, m1, m2, m3
    mova                 m5, [o(pw_1697x16)]
    pmulhrsw             m4, m5, m0
    pmulhrsw             m5, m1
    mova                 m6, [o(pw_16384)]
    REPX  {pmulhrsw x, m6 }, m4, m5
%endif
    paddsw               m0, m4
    paddsw               m1, m5
%if ARCH_X86_64
    pmulhrsw             m4, m9, m2
    pmulhrsw             m5, m9, m3
    REPX  {pmulhrsw x, m10}, m4, m5
%else
    mova                 m5, [o(pw_1697x16)]
    pmulhrsw             m4, m5, m2
    pmulhrsw             m5, m3
    REPX  {pmulhrsw x, m6 }, m4, m5
%endif
    paddsw               m2, m4
    paddsw               m3, m5
%if ARCH_X86_64
    REPX  {pmulhrsw x, m11}, m0, m1, m2, m3
%else
    psrlw                m6, 1          ; pw_8192
    REPX  {pmulhrsw x, m6 }, m0, m1, m2, m3
    pxor                 m6, m6
%endif
    call m(inv_txfm_add_identity_identity_8x32_16bpc).main_zero
    lea                dstq, [dstq+strideq*4]
    add                  cq, 16
    btc                eobd, 16
    jnc .main
    ret

cglobal inv_txfm_add_identity_identity_32x16_16bpc, 4, 7, 11, dst, stride, c, eob
%if ARCH_X86_32
    LEA                  r6, $$
%else
    mova                 m8, [o(pw_2896x8)]
    mova                 m9, [o(pw_1697x16)]
    mova                m10, [o(pw_2048)]
%endif
    mova                 m7, [o(pixel_10bpc_max)]
    lea                  r4, [strideq*3]
    pxor                 m6, m6
    mov                  r5, dstq
    call .main
    sub                eobd, 36
    jl .ret
    call .main
    add                  cq, 64*8-64
    lea                dstq, [r5+16*1]
    call .main
    sub                eobd, 107 ; eob < 143
    jl .ret
    call .main
    add                  cq, 64*8-64
    lea                dstq, [r5+16*2]
    call .main
    sub                eobd, 128 ; eob < 271
    jl .ret
    call .main
    add                  cq, 64*8-64
    lea                dstq, [r5+16*3]
    call .main
    sub                eobd, 128 ; eob < 399
    jl .ret
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
%if ARCH_X86_64
    REPX  {pmulhrsw x, m8 }, m0, m1, m2, m3
%else
    mova                 m6, [o(pw_2896x8)]
    REPX  {pmulhrsw x, m6 }, m0, m1, m2, m3
%endif
    REPX  {paddsw   x, x  }, m0, m1, m2, m3
%if ARCH_X86_64
    pmulhrsw             m4, m9, m0
    pmulhrsw             m5, m9, m1
%else
    mova                 m6, [o(pw_1697x16)]
    pmulhrsw             m4, m6, m0
    pmulhrsw             m5, m6, m1
%endif
    REPX  {paddsw   x, x  }, m0, m1
    paddsw               m0, m4
    paddsw               m1, m5
%if ARCH_X86_64
    pmulhrsw             m4, m9, m2
    pmulhrsw             m5, m9, m3
%else
    pmulhrsw             m4, m6, m2
    pmulhrsw             m6, m3
%endif
    REPX  {paddsw   x, x  }, m2, m3
    paddsw               m2, m4
%if ARCH_X86_64
    paddsw               m3, m5
    REPX  {pmulhrsw x, m10}, m0, m1, m2, m3
%else
    paddsw               m3, m6
    mova                 m6, [o(pw_2048)]
    REPX  {pmulhrsw x, m6 }, m0, m1, m2, m3
    pxor                 m6, m6
%endif
    REPX {mova [cq+64*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
    call m(inv_txfm_add_identity_identity_8x32_16bpc).main
    lea                dstq, [dstq+strideq*4]
    add                  cq, 16
    btc                eobd, 16
    jnc .main
    ret

cglobal inv_txfm_add_identity_identity_32x32_16bpc, 4, 7, 8, dst, stride, c, eob
%undef cmp
%if ARCH_X86_32
    LEA                  r6, $$
%endif
    mova                 m5, [o(pw_8192)]
    mova                 m7, [o(pixel_10bpc_max)]
    pxor                 m6, m6
    lea                  r4, [strideq*3]
    mov                  r5, dstq
    call .main                              ; 0
    cmp                eobd, 36
    jl .ret
    add                  cq, 128*8-32       ; 0 1
    lea                dstq, [r5+16]        ; 1
    call .main
    call .main2
    cmp                eobd, 136
    jl .ret
    add                  cq, 128*16-64      ; 0 1 2
    lea                dstq, [r5+16*2]      ; 1 2
    call .main                              ; 2
    call .main2
    call .main2
    cmp                eobd, 300
    jl .ret
    add                  cq, 128*24-96      ; 0 1 2 3
    add                  r5, 16*3           ; 1 2 3
    mov                dstq, r5             ; 2 3
    call .main                              ; 3
    call .main2
    call .main2
    call .main2
    cmp                eobd, 535
    jl .ret
    add                  cq, 128*24-96      ; 0 1 2 3
    lea                dstq, [r5+strideq*8] ; 1 2 3 4
    mov                  r5, dstq           ; 2 3 4
    call .main                              ; 3 4
    call .main2
    call .main2
    cmp                eobd, 755
    jl .ret
    add                  cq, 128*16-64      ; 0 1 2 3
    lea                dstq, [r5+strideq*8] ; 1 2 3 4
    mov                  r5, dstq           ; 2 3 4 5
    call .main                              ; 3 4 5
    call .main2
    cmp                eobd, 911
    jl .ret
    add                  cq, 128*8-32       ; 0 1 2 3
    lea                dstq, [r5+strideq*8] ; 1 2 3 4
    call .main                              ; 2 3 4 5
.ret:                                       ; 3 4 5 6
    RET
ALIGN function_align
.main2:
    sub                  cq, 128*8
    sub                dstq, 16
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
    call m(inv_txfm_add_identity_identity_8x32_16bpc).main_zero
    lea                dstq, [dstq+strideq*4]
    add                  cq, 16
    btc                eobd, 16
    jnc .main
    ret

cglobal inv_txfm_add_dct_dct_8x32_16bpc, 4, 7, 15, 0-36*16, \
                                         dst, stride, c, eob
%if ARCH_X86_32
    LEA                  r6, $$
%define base $$
    DECLARE_REG_TMP       0, 4
%else
    lea                  r6, [tbl_Nx32_odd_offset]
%define base tbl_Nx32_odd_offset
    DECLARE_REG_TMP       4, 7
%if WIN64
    mov [rsp+gprsize*1+35*16], r7
%endif
%endif
%define o2(x) r6-base+x
    test               eobd, eobd
    jz .dconly

%if ARCH_X86_32
    mov [rsp+gprsize*1+35*16], r0
%endif
%undef cmp
    ; remove entirely-zero iterations
    mov                 r5d, 7*2
    cmp                eobw, word [o2(tbl_8x32_2d)+r5]
    jge .end_zero_loop
    pxor                 m0, m0
.zero_loop:
    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
    mova   [rsp+ 3*16+r5*8], m0
    mova   [rsp+11*16+r5*8], m0
    mova   [rsp+ 3*16+t0*8], m0
    mova   [rsp+ 3*16+t1*8], m0
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_8x32_2d)+r5]
    jl .zero_loop
.end_zero_loop:
    ; actual first pass after skipping all-zero data
    mov [rsp+gprsize*0+35*16], eobd
    mov                  r3, rsp
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+0*128+r5*8]
    mova                 m1, [cq+1*128+r5*8]
    mova                 m2, [cq+2*128+r5*8]
    mova                 m3, [cq+3*128+r5*8]
    mova                 m4, [cq+4*128+r5*8]
    mova                 m5, [cq+5*128+r5*8]
    mova                 m6, [cq+6*128+r5*8]
    mova                 m7, [cq+7*128+r5*8]
    call m(idct_8x4_internal_16bpc).main_pass1
    mova                 m1, [o(pd_2)]
    REPX      {paddd x, m1}, m0, m6, m5, m3
    call m(idct_8x4_internal_16bpc).round
    REPX      {psrad x, 2 }, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    call m(idct_8x4_internal_16bpc).transpose4x8packed

    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
    mova    [r3+ 3*16+r5*8], m0
    mova    [r3+11*16+r5*8], m2
    mova    [r3+ 3*16+t1*8], m1
    mova    [r3+ 3*16+t0*8], m3
    pxor                 m7, m7
    REPX {mova [cq+x*128+r5*8], m7}, 0, 1, 2, 3, 4, 5, 6, 7
    sub                 r5d, 2
    jge .loop_pass1

    ; pass 2 code starts here
    ; m0 is already loaded from last iteration of first pass
%if ARCH_X86_32
    mov                  r0, [rsp+gprsize*1+35*16]
%endif
    mov                eobd, [rsp+gprsize*0+35*16]
    cmp                eobd, 43
    jl .load_veryfast
    cmp                eobd, 107
    jl .load_fast
    ; load normal
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main)]
    jmp .run
.load_fast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_fast)]
    jmp .run
.load_veryfast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_veryfast)]
    ; fall-through
.run:
    call .pass2
%if WIN64
    mov                  r7, [rsp+gprsize*1+35*16]
%endif
    RET

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    mova                 m1, [rsp+gprsize+16* 4]
    mova                 m2, [rsp+gprsize+16* 5]
    mova                 m3, [rsp+gprsize+16* 6]
    mova                 m4, [rsp+gprsize+16* 7]
    mova                 m5, [rsp+gprsize+16* 8]
    mova                 m6, [rsp+gprsize+16* 9]
    mova                 m7, [rsp+gprsize+16*10]
    call m_suffix(idct_8x8_internal_8bpc, _ssse3).main
    mova [rsp+gprsize+ 3*16], m0
    mova [rsp+gprsize+ 4*16], m1
    mova [rsp+gprsize+ 5*16], m2
    mova [rsp+gprsize+ 6*16], m3
    mova [rsp+gprsize+ 7*16], m4
    mova [rsp+gprsize+ 8*16], m5
    mova [rsp+gprsize+ 9*16], m6
    mova                 m0, [rsp+gprsize+11*16]
    mova                 m1, [rsp+gprsize+12*16]
    mova                 m2, [rsp+gprsize+13*16]
    mova                 m3, [rsp+gprsize+14*16]
    mova                 m4, [rsp+gprsize+15*16]
    mova                 m5, [rsp+gprsize+16*16]
    mova                 m6, [rsp+gprsize+17*16]
    mova                 m7, [rsp+gprsize+18*16]
    call m_suffix(idct_16x8_internal_8bpc, _ssse3).main
    mova                 m7, [rsp+gprsize+ 0*16]
    mova [rsp+gprsize+11*16], m0
    mova [rsp+gprsize+12*16], m1
    mova [rsp+gprsize+13*16], m2
    mova [rsp+gprsize+14*16], m3
    mova [rsp+gprsize+15*16], m4
    mova [rsp+gprsize+16*16], m5
    mova [rsp+gprsize+17*16], m6
    mova [rsp+gprsize+18*16], m7
    call                 r4
%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
%endif
    lea                  r3, [strideq*3]
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
    lea                dstq, [dstq+strideq*8]
    mova                 m0, [rsp+gprsize+11*16]
    mova                 m1, [rsp+gprsize+12*16]
    mova                 m2, [rsp+gprsize+13*16]
    mova                 m3, [rsp+gprsize+14*16]
    mova                 m4, [rsp+gprsize+15*16]
    mova                 m5, [rsp+gprsize+16*16]
    mova                 m6, [rsp+gprsize+17*16]
    mova                 m7, [rsp+gprsize+18*16]
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
    lea                dstq, [dstq+strideq*8]
    mova                 m0, [rsp+gprsize+19*16]
    mova                 m1, [rsp+gprsize+20*16]
    mova                 m2, [rsp+gprsize+21*16]
    mova                 m3, [rsp+gprsize+22*16]
    mova                 m4, [rsp+gprsize+23*16]
    mova                 m5, [rsp+gprsize+24*16]
    mova                 m6, [rsp+gprsize+25*16]
    mova                 m7, [rsp+gprsize+26*16]
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
    lea                dstq, [dstq+strideq*8]
    mova                 m0, [rsp+gprsize+27*16]
    mova                 m1, [rsp+gprsize+28*16]
    mova                 m2, [rsp+gprsize+29*16]
    mova                 m3, [rsp+gprsize+30*16]
    mova                 m4, [rsp+gprsize+31*16]
    mova                 m5, [rsp+gprsize+32*16]
    mova                 m6, [rsp+gprsize+33*16]
    mova                 m7, [rsp+gprsize+34*16]
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
    ret
.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 8
    add                 r5d, 640
    sar                 r5d, 10
    add                 rsp, (31+2*ARCH_X86_64)*16
    jmp m(inv_txfm_add_dct_dct_8x8_16bpc).end2

cglobal inv_txfm_add_dct_dct_16x32_16bpc, 4, 7, 16, 0-77*16, \
                                          dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

%if ARCH_X86_32
    mov [rsp+gprsize*1+76*16], r0
%elif WIN64
    mov [rsp+gprsize*1+76*16], r7
%endif
%undef cmp
    ; remove entirely-zero iterations
    mov                 r5d, 7*2
    cmp                eobw, word [o2(tbl_16x32_2d)+r5]
    jge .end_zero_loop
    pxor                 m0, m0
.zero_loop:
    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
    mova   [rsp+12*16+r5*8], m0
    mova   [rsp+20*16+r5*8], m0
    mova   [rsp+12*16+t0*8], m0
    mova   [rsp+12*16+t1*8], m0
    mova   [rsp+44*16+r5*8], m0
    mova   [rsp+52*16+r5*8], m0
    mova   [rsp+44*16+t0*8], m0
    mova   [rsp+44*16+t1*8], m0
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_16x32_2d)+r5]
    jl .zero_loop
.end_zero_loop:
    ; actual first pass after skipping all-zero data
    mov [rsp+gprsize*0+76*16], eobd
    mov                  r3, rsp
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+ 1*128+r5*8]
    mova                 m1, [cq+ 3*128+r5*8]
    mova                 m2, [cq+ 5*128+r5*8]
    mova                 m3, [cq+ 7*128+r5*8]
    mova                 m4, [cq+ 9*128+r5*8]
    mova                 m5, [cq+11*128+r5*8]
    mova                 m6, [cq+13*128+r5*8]
    mova                 m7, [cq+15*128+r5*8]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_16x4_internal_16bpc).main_oddhalf

    mova                 m0, [cq+ 0*128+r5*8]
    mova                 m1, [cq+ 2*128+r5*8]
    mova                 m2, [cq+ 4*128+r5*8]
    mova                 m3, [cq+ 6*128+r5*8]
    mova                 m4, [cq+ 8*128+r5*8]
    mova                 m5, [cq+10*128+r5*8]
    mova                 m6, [cq+12*128+r5*8]
    mova                 m7, [cq+14*128+r5*8]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    call m(idct_16x4_internal_16bpc).round
%if ARCH_X86_64
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%endif
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
%if ARCH_X86_64
    mova   [rsp+12*16+r5*8], m0
    mova   [rsp+20*16+r5*8], m2
    mova   [rsp+12*16+t1*8], m1
    mova   [rsp+12*16+t0*8], m3
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova   [rsp+44*16+r5*8], m8
    mova   [rsp+52*16+r5*8], m10
    mova   [rsp+44*16+t1*8], m9
    mova   [rsp+44*16+t0*8], m11
%else
    mova   [rsp+44*16+r5*8], m0
    mova   [rsp+52*16+r5*8], m2
    mova   [rsp+44*16+t1*8], m1
    mova   [rsp+44*16+t0*8], m3
    mova                 m0, [r3+ 8*16]
    mova                 m2, [r3+ 9*16]
    mova                 m4, [r3+10*16]
    mova                 m6, [r3+11*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova   [rsp+12*16+r5*8], m0
    mova   [rsp+20*16+r5*8], m2
    mova   [rsp+12*16+t1*8], m1
    mova   [rsp+12*16+t0*8], m3
%endif
    pxor                 m7, m7
    REPX {mova [cq+x*128+r5*8], m7}, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2
    add                 rsp, 9*16
%if ARCH_X86_64
    mov                  r6, dstq
%else
    mov                dstq, [rsp+gprsize*1+67*16]
%endif
    mov                eobd, [rsp+gprsize*0+67*16]
    cmp                eobd, 44
    jl .load_veryfast
    cmp                eobd, 151
    jl .load_fast
    ; load normal
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main)]
    jmp .run
.load_fast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_fast)]
    jmp .run
.load_veryfast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_veryfast)]
    ; fall-through
.run:
%if ARCH_X86_64
    lea                  r2, [dstq+32]
    mov                  r7, -4
%else
    lea                  r2, [rsp+67*16]
    mov dword [r2+0*gprsize], 2
%endif
    jmp .loop_pass2_entry
.loop_pass2:
    mova                 m0, [rsp+16* 3]
.loop_pass2_entry:
%if ARCH_X86_32
    mov                dstq, [r2+1*gprsize]
%endif
    call m(inv_txfm_add_dct_dct_8x32_16bpc).pass2
    add                 rsp, 32*16
%if ARCH_X86_64
    add                  r7, 2
    lea                dstq, [r2+r7*8]
    jl .loop_pass2
%if WIN64
    mov                  r7, [rsp+gprsize*1+3*16]
%endif
%else
    add dword [r2+1*gprsize], 16
    dec dword [r2+0*gprsize]
    jg .loop_pass2
%endif
%assign stack_size (stack_size-73*16)
%if STACK_ALIGNMENT >= 16
%assign stack_size_padded (stack_size_padded-73*16)
%assign stack_offset (stack_offset-73*16)
%else
%xdefine rstkm [rsp + stack_size]
%endif
    RET
.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 32
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
    add                 rsp, (65+4*ARCH_X86_64)*16
    jmp m(inv_txfm_add_dct_dct_16x4_16bpc).dconly

cglobal inv_txfm_add_dct_dct_32x8_16bpc, 4, 7, 16, 0-(24+8*ARCH_X86_32)*16, \
                                         dst, stride, c, eob
%if ARCH_X86_32
    LEA                  r6, $$
%endif
    test               eobd, eobd
    jz .dconly

    ; remove entirely-zero iterations
%undef cmp
%if ARCH_X86_64
    xor                 r5d, r5d
    cmp                eobd, 10
    setge               r5b
%else
    mov                 r5d, 1
    cmp                eobd, 10
    sbb                 r5d, 0
%endif
    add                 r5d, r5d

    ; actual first pass after skipping all-zero data
.loop_pass1:
    mova                 m0, [cq+32* 1+r5*8]
    mova                 m1, [cq+32* 7+r5*8]
    mova                 m2, [cq+32* 9+r5*8]
    mova                 m3, [cq+32*15+r5*8]
    mova                 m4, [cq+32*17+r5*8]
    mova                 m5, [cq+32*23+r5*8]
    mova                 m6, [cq+32*25+r5*8]
    mova                 m7, [cq+32*31+r5*8]
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mov                  r3, rsp
    call .main_oddhalf_part1
    mova                 m0, [cq+32* 3+r5*8]
    mova                 m1, [cq+32* 5+r5*8]
    mova                 m2, [cq+32*11+r5*8]
    mova                 m3, [cq+32*13+r5*8]
    mova                 m4, [cq+32*19+r5*8]
    mova                 m5, [cq+32*21+r5*8]
    mova                 m6, [cq+32*27+r5*8]
    mova                 m7, [cq+32*29+r5*8]
    call .main_oddhalf_part2
    mova                 m0, [cq+32* 2+r5*8]
    mova                 m1, [cq+32* 6+r5*8]
    mova                 m2, [cq+32*10+r5*8]
    mova                 m3, [cq+32*14+r5*8]
    mova                 m4, [cq+32*18+r5*8]
    mova                 m5, [cq+32*22+r5*8]
    mova                 m6, [cq+32*26+r5*8]
    mova                 m7, [cq+32*30+r5*8]
    add                  r3, 16*(16+4*ARCH_X86_32)
    call m(idct_16x4_internal_16bpc).main_oddhalf
    mova                 m0, [cq+32* 0+r5*8]
    mova                 m1, [cq+32* 4+r5*8]
    mova                 m2, [cq+32* 8+r5*8]
    mova                 m3, [cq+32*12+r5*8]
    mova                 m4, [cq+32*16+r5*8]
    mova                 m5, [cq+32*20+r5*8]
    mova                 m6, [cq+32*24+r5*8]
    mova                 m7, [cq+32*28+r5*8]
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    sub                  r3, 16*(16+4*ARCH_X86_32)
    call .round_dct32
%if ARCH_X86_64
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova    [cq+32* 8+r5*8], m8
    mova    [cq+32* 9+r5*8], m9
    mova    [cq+32*10+r5*8], m10
    mova    [cq+32*11+r5*8], m11
    mova                 m8, [r3+16* 9] ;  8  9
    mova                m10, [r3+16*11] ; 10 11
    mova                m12, [r3+16*13] ; 12 13
    mova                m14, [r3+16*15] ; 14 15
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova    [cq+32* 4+r5*8], m8
    mova    [cq+32* 5+r5*8], m9
    mova    [cq+32* 6+r5*8], m10
    mova    [cq+32* 7+r5*8], m11
    mova                 m8, [r3+16* 8] ; 24 25
    mova                m10, [r3+16*10] ; 26 27
    mova                m12, [r3+16*12] ; 28 29
    mova                m14, [r3+16*14] ; 30 31
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova    [cq+32*12+r5*8], m8
    mova    [cq+32*13+r5*8], m9
    mova    [cq+32*14+r5*8], m10
    mova    [cq+32*15+r5*8], m11
%else
    sub                  r3, 8*16
    mova                 m0, [r3+ 8*16]
    mova                 m2, [r3+10*16]
    mova                 m4, [r3+12*16]
    mova                 m6, [r3+14*16]
    packssdw             m0, [r3+ 9*16]
    packssdw             m2, [r3+11*16]
    packssdw             m4, [r3+13*16]
    packssdw             m6, [r3+15*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova    [cq+32* 4+r5*8], m0
    mova    [cq+32* 5+r5*8], m1
    mova    [cq+32* 6+r5*8], m2
    mova    [cq+32* 7+r5*8], m3
    mova                 m0, [r3+16*16]
    mova                 m2, [r3+18*16]
    mova                 m4, [r3+20*16]
    mova                 m6, [r3+22*16]
    packssdw             m0, [r3+17*16]
    packssdw             m2, [r3+19*16]
    packssdw             m4, [r3+21*16]
    packssdw             m6, [r3+23*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova    [cq+32* 8+r5*8], m0
    mova    [cq+32* 9+r5*8], m1
    mova    [cq+32*10+r5*8], m2
    mova    [cq+32*11+r5*8], m3
    mova                 m0, [r3+31*16]
    mova                 m2, [r3+29*16]
    mova                 m4, [r3+27*16]
    mova                 m6, [r3+25*16]
    packssdw             m0, [r3+30*16]
    packssdw             m2, [r3+28*16]
    packssdw             m4, [r3+26*16]
    packssdw             m6, [r3+24*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova    [cq+32*12+r5*8], m0
    mova    [cq+32*13+r5*8], m1
    mova    [cq+32*14+r5*8], m2
    mova    [cq+32*15+r5*8], m3
    mova                 m0, [r3+ 0*16]
    mova                 m2, [r3+ 2*16]
    mova                 m4, [r3+ 4*16]
    mova                 m6, [r3+ 6*16]
    packssdw             m0, [r3+ 1*16]
    packssdw             m2, [r3+ 3*16]
    packssdw             m4, [r3+ 5*16]
    packssdw             m6, [r3+ 7*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
%endif
    pxor                 m7, m7
    ; clear lower half of [cq]
    REPX {mova [cq+x*32+r5*8], m7}, 16, 17, 18, 19, 20, 21, 22, 23, \
                                    24, 25, 26, 27, 28, 29, 30, 31
    test                r5d, r5d
    jz .end_pass1
    mova    [cq+32* 0+r5*8], m0
    mova    [cq+32* 1+r5*8], m1
    mova    [cq+32* 2+r5*8], m2
    mova    [cq+32* 3+r5*8], m3
    sub                 r5d, 2
    jmp .loop_pass1
.end_pass1:

    ; pass=2, we need to call this otherwise the stack pointer has
    ; the wrong offset in the 8-bit code
    mov                 r4d, 4
    call m(idct_16x8_internal_16bpc).pass2_main
    RET

.main_oddhalf_part1_fast: ; lower half zero
    pmulld               m7, m0, [o(pd_4091)]
    pmulld               m0, [o(pd_201)]
    pmulld               m4, m3, [o(pd_m2751)]
%if ARCH_X86_32
    pmulld               m3, [o(pd_3035)]
    mova                 m5, [o(pd_2048)]
    REPX      {paddd x, m5}, m0, m7
    REPX      {psrad x, 12}, m0, m7
    mova          [r3+3*16], m7
    mova                 m7, m3
    mova                 m3, m5
%else
    pmulld               m3, [o(pd_3035)]
%endif
    pmulld               m6, m1, [o(pd_m1380)]
    pmulld               m1, [o(pd_3857)]
    pmulld               m5, m2, [o(pd_3703)]
    pmulld               m2, [o(pd_1751)]
    jmp .main_oddhalf_part1_fast2
.main_oddhalf_part1: ; in1, in7, in9, in15, in17, in23, in25, in31
%if ARCH_X86_64
    ITX_MULSUB_2D         0, 7, 8, 9, 10, _,  201, 4091 ; t16a, t31a
    ITX_MULSUB_2D         6, 1, 8, 9, 10, _, 3857, 1380 ; t19a, t28a
    ITX_MULSUB_2D         2, 5, 8, 9, 10, _, 1751, 3703 ; t18a, t29a
    ITX_MULSUB_2D         4, 3, 8, 9, 10, _, 3035, 2751 ; t17a, t30a
.main_oddhalf_part1_fast2:
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
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
    mova                m15, [o(pd_4017)]
    mova                m10, [o(pd_799)]
    ITX_MULSUB_2D         5, 8, 3, 9, _, 11, 10, 15    ; t17a, t30a
    ITX_MULSUB_2D         2, 4, 3, 9, _, 11, 10, 15, 4 ; t29a, t18a
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
    mova                m15, [o(pd_3784)]
    mova                m10, [o(pd_1567)]
    ITX_MULSUB_2D         4, 1, 2, 9, _, 11, 10, 15 ; t18a, t29a
    ITX_MULSUB_2D         6, 3, 2, 9, _, 11, 10, 15 ; t19,  t28
    mova          [r3+16*0], m0
    mova          [r3+16*1], m5
    mova          [r3+16*2], m4
    mova          [r3+16*3], m6
    mova          [r3+16*4], m3
    mova          [r3+16*5], m1
    mova          [r3+16*6], m8
    mova          [r3+16*7], m7
%else
    mova          [r3+0*16], m2
    mova          [r3+1*16], m3
    mova          [r3+2*16], m4
    mova          [r3+3*16], m5
    mova                  m3, [o(pd_2048)]
    ITX_MULSUB_2D         0, 7, 2, 4, 5, 3,  201, 4091 ; t16a, t31a
    ITX_MULSUB_2D         6, 1, 2, 4, 5, _, 3857, 1380 ; t19a, t28a
    mova                 m4, [r3+2*16]
    mova                 m5, [r3+3*16]
    mova          [r3+2*16], m6
    mova          [r3+3*16], m7
    mova                 m2, [r3+0*16]
    mova                 m7, [r3+1*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m1
    ITX_MULSUB_2D         2, 5, 0, 1, 6, _, 1751, 3703 ; t18a, t29a
    ITX_MULSUB_2D         4, 7, 0, 1, 6, _, 3035, 2751 ; t17a, t30a
    mova                 m0, [r3+0*16]
    mova                 m1, [r3+1*16]
    mova                 m6, [r3+2*16]
.main_oddhalf_part1_fast2:
    REPX      {paddd x, m3}, m1, m2, m4, m5, m6, m7
    REPX      {psrad x, 12}, m1, m2, m4, m5, m6, m7
    psubd                m3, m0, m4 ; t17
    mova          [r3+0*16], m3
    mova                 m3, [r3+3*16]
    paddd                m0, m4     ; t16
    psubd                m4, m6, m2 ; t18
    paddd                m6, m2     ; t19
    psubd                m2, m1, m5 ; t29
    paddd                m1, m5     ; t28
    psubd                m5, m3, m7 ; t30
    paddd                m7, m3     ; t31
    mova                 m3, [o(clip_18b_min)]
    REPX     {pmaxsd x, m3}, m5, m4, m2, m0, m6, m1, m7
    pmaxsd               m3, [r3+0*16]
    mova          [r3+0*16], m3
    mova                 m3, [o(clip_18b_max)]
    REPX     {pminsd x, m3}, m5, m4, m2, m0, m6, m1, m7
    pminsd               m3, [r3+0*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m1
    mova          [r3+2*16], m6
    mova          [r3+3*16], m7
    mova                 m0, [o(pd_2048)]
    ITX_MULSUB_2D         5, 3, 1, 6, 7, 0,  799, 4017    ; t17a, t30a
    ITX_MULSUB_2D         2, 4, 1, 6, _, 0,    7, 4017, 4 ; t29a, t18a
    psubd                m1, m5, m4 ; t18
    paddd                m5, m4     ; t17
    psubd                m4, m3, m2 ; t29
    paddd                m3, m2     ; t30
    mova                 m0, [r3+0*16]
    mova                 m2, [r3+1*16]
    mova                 m6, [r3+2*16]
    mova                 m7, [r3+3*16]
    mova          [r3+0*16], m3
    psubd                m3, m0, m6 ; t19a
    paddd                m0, m6     ; t16a
    psubd                m6, m7, m2 ; t28a
    paddd                m7, m2     ; t31a
    mova                 m2, [o(clip_18b_min)]
    REPX     {pmaxsd x, m2}, m3, m6, m1, m4, m0, m7, m5
    pmaxsd               m2, [r3+0*16]
    mova          [r3+0*16], m2
    mova                 m2, [o(clip_18b_max)]
    REPX     {pminsd x, m2}, m3, m6, m1, m4, m0, m7, m5
    pminsd               m2, [r3+0*16]
    mova          [r3+16*0], m0
    mova          [r3+16*1], m5
    mova          [r3+16*6], m2
    mova          [r3+16*7], m7
    mova                 m7, [o(pd_2048)]
    ITX_MULSUB_2D         4, 1, 0, 5, 2, 7, 1567, 3784 ; t18a, t29a
    ITX_MULSUB_2D         6, 3, 0, 5, 2, 7,    2, 3784 ; t19,  t28
    mova          [r3+16*2], m4
    mova          [r3+16*3], m6
    mova          [r3+16*4], m3
    mova          [r3+16*5], m1
%endif
    ret
.main_oddhalf_part2_fast: ; lower half zero
    pmulld               m7, m0, [o(pd_m601)]
    pmulld               m0, [o(pd_4052)]
    pmulld               m4, m3, [o(pd_3290)]
%if ARCH_X86_32
    pmulld               m3, [o(pd_2440)]
    mova                 m5, [o(pd_2048)]
    REPX      {paddd x, m5}, m0, m7
    REPX      {psrad x, 12}, m0, m7
    mova         [r3+11*16], m7
    mova                 m7, m3
    mova                 m3, m5
%else
    pmulld               m3, [o(pd_2440)]
%endif
    pmulld               m6, m1, [o(pd_3973)]
    pmulld               m1, [o(pd_995)]
    pmulld               m5, m2, [o(pd_m2106)]
    pmulld               m2, [o(pd_3513)]
    jmp .main_oddhalf_part2_fast2
.main_oddhalf_part2: ; in3, in5, in11, in13, in19, in21, in27, in29
%if ARCH_X86_64
    ITX_MULSUB_2D         7, 0, 8, 9, 10, _, 4052,  601 ; t23a, t24a
    ITX_MULSUB_2D         1, 6, 8, 9, 10, _,  995, 3973 ; t20a, t27a
    ITX_MULSUB_2D         5, 2, 8, 9, 10, _, 3513, 2106 ; t21a, t26a
    ITX_MULSUB_2D         3, 4, 8, 9, 10, _, 2440, 3290 ; t22a, t25a
.main_oddhalf_part2_fast2:
    REPX     {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX     {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
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
    mova                m15, [o(pd_2276)]
    mova                m10, [o(pd_3406)]
    ITX_MULSUB_2D         4, 2, 3, 9, _, 11, 10, 15    ; t21a, t26a
    ITX_MULSUB_2D         8, 5, 3, 9, _, 11, 10, 15, 4 ; t25a, t22a
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
    mova                m15, [o(pd_3784)]
    mova                m10, [o(pd_1567)]
    ITX_MULSUB_2D         4, 1, 2, 9, _, 11, 10, 15, 4 ; t26a, t21a
    ITX_MULSUB_2D         3, 6, 2, 9, _, 11, 10, 15, 4 ; t27,  t20
    mova                 m9, [r3+16*0] ; t16a
    mova                m10, [r3+16*1] ; t17
    psubd                m2, m9, m7    ; t23
    paddd                m9, m7        ; t16
    psubd                m7, m10, m5   ; t22a
    paddd               m10, m5        ; t17a
    REPX    {pmaxsd x, m12}, m9, m10, m2, m7
    REPX    {pminsd x, m13}, m9, m10, m2, m7
    mova          [r3+16*0], m9
    mova          [r3+16*1], m10
    mova                 m9, [r3+16*2] ; t18a
    mova                m10, [r3+16*3] ; t19
    psubd                m5, m9, m1    ; t21
    paddd                m9, m1        ; t18
    psubd                m1, m10, m6   ; t20a
    paddd               m10, m6        ; t19a
    REPX    {pmaxsd x, m12}, m9, m10, m5, m1
    REPX    {pminsd x, m13}, m9, m10, m5, m1
    mova          [r3+16*2], m9
    mova          [r3+16*3], m10
    mova                 m9, [r3+16*4] ; t28
    mova                m10, [r3+16*5] ; t29a
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
    mova          [r3+16*4], m4
    mova          [r3+16*5], m1
    mova                 m4, [r3+16*6] ; t30
    mova                 m1, [r3+16*7] ; t31a
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
    mova          [r3+16*6], m0
    mova          [r3+16*7], m7
    mova          [r3+16*8], m2
    mova          [r3+16*9], m5
    mova         [r3+16*10], m3
    mova         [r3+16*11], m6
    mova         [r3+16*12], m9
    mova         [r3+16*13], m10
    mova         [r3+16*14], m4
    mova         [r3+16*15], m1
%else
    mova         [r3+ 8*16], m2
    mova         [r3+ 9*16], m3
    mova         [r3+10*16], m4
    mova         [r3+11*16], m5
    mova                 m3, [o(pd_2048)]
    ITX_MULSUB_2D         7, 0, 2, 4, 5, 3, 4052,  601 ; t23a, t24a
    ITX_MULSUB_2D         1, 6, 2, 4, 5, _,  995, 3973 ; t20a, t27a
    mova                 m2, [r3+ 8*16]
    mova                 m4, [r3+10*16]
    mova                 m5, [r3+11*16]
    mova         [r3+ 8*16], m0
    mova         [r3+10*16], m6
    mova         [r3+11*16], m7
    mova                 m7, [r3+ 9*16]
    mova         [r3+ 9*16], m1
    ITX_MULSUB_2D         5, 2, 0, 6, 1, _, 3513, 2106 ; t21a, t26a
    ITX_MULSUB_2D         7, 4, 0, 6, 1, _, 2440, 3290 ; t22a, t25a
    mova                 m0, [r3+ 8*16]
    mova                 m1, [r3+ 9*16]
    mova                 m6, [r3+10*16]
.main_oddhalf_part2_fast2:
    REPX      {paddd x, m3}, m1, m2, m7, m4, m5, m6
    REPX      {psrad x, 12}, m1, m2, m7, m4, m5, m6
    psubd                m3, m0, m4 ; t25
    mova         [r3+ 8*16], m3
    mova                 m3, [r3+11*16]
    paddd                m0, m4     ; t24
    psubd                m4, m6, m2 ; t26
    paddd                m6, m2     ; t27
    psubd                m2, m1, m5 ; t21
    paddd                m1, m5     ; t20
    psubd                m5, m3, m7 ; t22
    paddd                m7, m3     ; t23
    mova                 m3, [o(clip_18b_min)]
    REPX     {pmaxsd x, m3}, m5, m4, m2, m0, m6, m1, m7
    pmaxsd               m3, [r3+ 8*16]
    mova         [r3+ 8*16], m3
    mova                 m3, [o(clip_18b_max)]
    REPX     {pminsd x, m3}, m5, m4, m2, m0, m6, m1, m7
    pminsd               m3, [r3+ 8*16]
    mova         [r3+ 8*16], m0
    mova         [r3+ 9*16], m1
    mova         [r3+10*16], m6
    mova         [r3+11*16], m7
    mova                 m7, [o(pd_2048)]
    ITX_MULSUB_2D         4, 2, 0, 1, 6, 7, 3406, 2276    ; t21a, t26a
    ITX_MULSUB_2D         3, 5, 0, 1, _, 7,    6, 2276, 4 ; t25a, t22a
    psubd                m1, m5, m4 ; t21
    paddd                m5, m4     ; t22
    psubd                m4, m3, m2 ; t26
    paddd                m3, m2     ; t25
    mova                 m0, [r3+ 8*16]
    mova                 m2, [r3+ 9*16]
    mova                 m6, [r3+10*16]
    mova                 m7, [r3+11*16]
    mova         [r3+ 8*16], m3
    psubd                m3, m0, m6 ; t27a
    paddd                m0, m6     ; t24a
    psubd                m6, m7, m2 ; t20a
    paddd                m7, m2     ; t23a
    mova                 m2, [o(clip_18b_min)]
    REPX     {pmaxsd x, m2}, m3, m6, m1, m4, m0, m7, m5
    pmaxsd               m2, [r3+ 8*16]
    mova         [r3+ 8*16], m2
    mova                 m2, [o(clip_18b_max)]
    REPX     {pminsd x, m2}, m3, m6, m1, m4, m0, m7, m5
    pminsd               m2, [r3+ 8*16]
    mova         [r3+ 8*16], m0
    mova         [r3+ 9*16], m2
    mova         [r3+14*16], m5
    mova         [r3+15*16], m7
    mova                 m0, [o(pd_2048)]
    ITX_MULSUB_2D         4, 1, 2, 5, 7, 0, 1567, 3784, 4 ; t26a, t21a
    ITX_MULSUB_2D         3, 6, 2, 5, _, 0,    7, 3784, 4 ; t27,  t20
    mova         [r3+10*16], m3
    mova                 m0, [o(clip_18b_min)]
    mova                 m2, [o(clip_18b_max)]
    mova                 m5, [r3+16*2] ; t18a
    mova                 m7, [r3+16*3] ; t19
    psubd                m3, m5, m1    ; t21
    paddd                m5, m1        ; t18
    psubd                m1, m7, m6    ; t20a
    paddd                m7, m6        ; t19a
    REPX     {pmaxsd x, m0}, m5, m7, m3, m1
    REPX     {pminsd x, m2}, m5, m7, m3, m1
    mova          [r3+16*2], m5
    mova          [r3+16*3], m7
    mova         [r3+11*16], m3
    mova                 m3, [r3+10*16]
    mova                 m5, [r3+16*4] ; t28
    mova                 m7, [r3+16*5] ; t29a
    psubd                m6, m5, m3    ; t27a
    paddd                m5, m3        ; t28a
    psubd                m3, m7, m4    ; t26
    paddd                m7, m4        ; t29
    REPX     {pmaxsd x, m0}, m5, m7, m6, m3
    REPX     {pminsd x, m2}, m5, m7, m6, m3
    mova         [r3+16*12], m5
    mova         [r3+16*13], m7
    mova                 m5, [o(pd_2048)]
    mova                 m7, [o(pd_2896)]
    mova                 m4, [r3+11*16]
    REPX     {pmulld x, m7}, m6, m3, m1, m4
    paddd                m6, m5
    paddd                m3, m5
    psubd                m5, m6, m1    ; t20
    paddd                m6, m1        ; t27
    psubd                m1, m3, m4    ; t21a
    paddd                m3, m4        ; t26a
    REPX     {psrad  x, 12}, m5, m1, m3, m6
    mova          [r3+16*4], m5
    mova          [r3+16*5], m1
    mova         [r3+16*10], m3
    mova         [r3+16*11], m6

    mova                 m5, [r3+14*16]
    mova                 m6, [r3+15*16]
    mova                 m3, [r3+16*0] ; t16a
    mova                 m4, [r3+16*1] ; t17
    psubd                m1, m3, m6    ; t23
    paddd                m3, m6        ; t16
    psubd                m6, m4, m5    ; t22a
    paddd                m4, m5        ; t17a
    REPX     {pmaxsd x, m0}, m3, m4, m1, m6
    REPX     {pminsd x, m2}, m3, m4, m1, m6
    mova          [r3+16*0], m3
    mova          [r3+16*1], m4
    mova                 m5, [r3+ 8*16]
    mova                 m3, [r3+ 9*16]
    mova         [r3+ 8*16], m1
    mova         [r3+ 9*16], m6
    mova                 m4, [r3+16*6] ; t30
    mova                 m1, [r3+16*7] ; t31a
    psubd                m6, m1, m5    ; t24
    paddd                m1, m5        ; t31
    psubd                m5, m4, m3    ; t25a
    paddd                m4, m3        ; t30a
    REPX     {pmaxsd x, m0}, m6, m5, m4, m1
    REPX     {pminsd x, m2}, m6, m5, m4, m1
    mova         [r3+16*14], m4
    mova         [r3+16*15], m1
    mova                 m4, [o(pd_2048)]
    mova                 m1, [r3+ 9*16]
    mova                 m2, [r3+ 8*16]
    REPX     {pmulld x, m7}, m5, m6, m1, m2
    paddd                m5, m4
    paddd                m6, m4
    psubd                m0, m5, m1    ; t22
    paddd                m5, m1        ; t25
    psubd                m1, m6, m2    ; t23a
    paddd                m2, m6        ; t24a
    REPX     {psrad  x, 12}, m0, m1, m2, m5
    mova          [r3+16*6], m0
    mova          [r3+16*7], m1
    mova          [r3+16*8], m2
    mova          [r3+16*9], m5
%endif
    ret

    ; final sumsub for idct16 as well as idct32, plus final downshift
%macro IDCT32_END 6 ; in/out1, out2-4, tmp, shift, idx
    mova                m%4, [r3+16*(23-%1)]
    pmaxsd              m%1, m12
    pminsd              m%1, m13
    psubd               m%3, m%1, m%4 ; idct16 out15 - n
    paddd               m%1, m%4      ; idct16 out0  + n
    pmaxsd              m%1, m12
    pmaxsd              m%3, m12
    pminsd              m%1, m13
    pminsd              m%3, m13
    paddd               m%1, m11
    paddd               m%3, m11
    mova                m%5, [r3+16*( 0+%1)]
    mova                m%2, [r3+16*(15-%1)]
    psubd               m%4, m%1, m%2 ; out31 - n
    paddd               m%1, m%2      ; out0  + n
    paddd               m%2, m%3, m%5 ; out15 - n
    psubd               m%3, m%5      ; out16 + n
    REPX      {psrad x, %6}, m%1, m%3, m%2, m%4
%endmacro

.round_dct32:
%if ARCH_X86_64
    psrld               m11, 10 ; pd_2
    IDCT32_END            0, 15, 8, 9, 10, 2    ; 0 15 16 31
    mova         [r3+ 0*16], m6
    mova         [r3+23*16], m7
    IDCT32_END            1, 14, 6, 7, 10, 2    ; 1 14 17 30
    packssdw             m0, m1       ;  0  1
    packssdw            m14, m15      ; 14 15
    packssdw             m8, m6       ; 16 17
    packssdw             m7, m9       ; 30 31
    mova         [r3+16*15], m14
    mova         [r3+16*14], m7
    IDCT32_END            2, 15, 10, 7, 6, 2    ; 2 13 18 29
    IDCT32_END            3, 14,  1, 9, 6, 2    ; 3 12 19 28
    packssdw             m2, m3       ;  2  3
    packssdw            m14, m15      ; 12 13
    packssdw            m10, m1       ; 18 19
    packssdw             m9, m7       ; 28 29
    mova         [r3+16*13], m14
    mova         [r3+16*12], m9
    IDCT32_END            4, 15, 1, 7, 6, 2     ; 4 11 20 27
    IDCT32_END            5, 14, 3, 9, 6, 2     ; 5 10 21 26
    packssdw             m4, m5       ;  4  5
    packssdw            m14, m15      ; 10 11
    packssdw             m1, m3       ; 20 21
    packssdw             m9, m7       ; 26 27
    mova         [r3+16*11], m14
    mova         [r3+16*10], m9
    mova                 m6, [r3+ 0*16]
    mova                 m7, [r3+23*16]
    IDCT32_END            6, 15, 14, 5,  3, 2   ; 6 9 22 25
    IDCT32_END            7, 11,  3, 9, 13, 2   ; 7 8 23 24
    packssdw             m6, m7       ;  6  7
    packssdw            m11, m15      ;  8  9
    packssdw            m14, m3       ; 22 23
    packssdw             m9, m5       ; 24 25
    mova          [r3+16*9], m11
    mova          [r3+16*8], m9
    mova                m12, m1
    ret
%else
    mova         [r3+16*16], m0
    mova         [r3+17*16], m1
    mova         [r3+18*16], m2
    mova         [r3+19*16], m3
    mova         [r3+20*16], m4
    mova         [r3+21*16], m5
    mova         [r3+22*16], m6
    mova         [r3+23*16], m7
    mova                 m1, [o(pd_2)]
    mova                 m2, [o(clip_18b_min)]
    mova                 m3, [o(clip_18b_max)]

    mov                  r4, 15*16
.loop_dct32_end:
    mova                 m0, [r3+16*16]
    mova                 m6, [r3+16*24]
    pmaxsd               m0, m2
    pminsd               m0, m3
    psubd                m5, m0, m6 ; idct16 out15 - n
    paddd                m0, m6     ; idct16 out0  + n
    pmaxsd               m0, m2
    pmaxsd               m5, m2
    pminsd               m0, m3
    pminsd               m5, m3
    paddd                m0, m1
    paddd                m5, m1
    mova                 m7, [r3]
    mova                 m4, [r3+r4]
    psubd                m6, m0, m4 ; out31 - n
    paddd                m0, m4     ; out0  + n
    paddd                m4, m5, m7 ; out15 - n
    psubd                m5, m7     ; out16 + n
    REPX       {psrad x, 2}, m0, m5, m4, m6
    mova               [r3], m0
    mova            [r3+r4], m4
    mova         [r3+16*16], m5
    mova         [r3+24*16], m6
    add                  r3, 16
    sub                  r4, 32
    jg .loop_dct32_end
    ret
%endif

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 8
.dconly1:
    add                 r5d, 640
    sar                 r5d, 10
.dconly2:
    imul                r5d, 2896
    add                 r5d, 34816
    movd                 m0, r5d
    pshuflw              m0, m0, q1111
    punpcklqdq           m0, m0
    mova                 m6, [o(pixel_10bpc_max)]
    pxor                 m5, m5
.dconly_loop:
    mova                 m1, [dstq+16*0]
    mova                 m2, [dstq+16*1]
    mova                 m3, [dstq+16*2]
    mova                 m4, [dstq+16*3]
    REPX     {paddw  x, m0}, m1, m2, m3, m4
    REPX     {pminsw x, m6}, m1, m2, m3, m4
    REPX     {pmaxsw x, m5}, m1, m2, m3, m4
    mova        [dstq+16*0], m1
    mova        [dstq+16*1], m2
    mova        [dstq+16*2], m3
    mova        [dstq+16*3], m4
    add                dstq, strideq
    dec                 r3d
    jg .dconly_loop
    RET

cglobal inv_txfm_add_dct_dct_32x16_16bpc, 4, 7, 16, 0-(24+8*ARCH_X86_32)*16, \
                                         dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

    ; remove entirely-zero iterations
%undef cmp
    mov                 r5d, 8
.zero_loop:
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_32x16_2d)+r5]
    jl .zero_loop

    ; actual first pass after skipping all-zero data
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+64* 1+r5*8]
    mova                 m1, [cq+64* 7+r5*8]
    mova                 m2, [cq+64* 9+r5*8]
    mova                 m3, [cq+64*15+r5*8]
    mova                 m4, [cq+64*17+r5*8]
    mova                 m5, [cq+64*23+r5*8]
    mova                 m6, [cq+64*25+r5*8]
    mova                 m7, [cq+64*31+r5*8]
    mov                  r3, rsp
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part1

    mova                 m0, [cq+64* 3+r5*8]
    mova                 m1, [cq+64* 5+r5*8]
    mova                 m2, [cq+64*11+r5*8]
    mova                 m3, [cq+64*13+r5*8]
    mova                 m4, [cq+64*19+r5*8]
    mova                 m5, [cq+64*21+r5*8]
    mova                 m6, [cq+64*27+r5*8]
    mova                 m7, [cq+64*29+r5*8]
%if ARCH_X86_32
    add                  r3, 16*8
%endif
    call m(idct_8x4_internal_16bpc).rect2_mul
%if ARCH_X86_32
    sub                  r3, 16*8
%endif
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part2
    add                  r3, 16*(16+4*ARCH_X86_32)

    mova                 m0, [cq+64* 2+r5*8]
    mova                 m1, [cq+64* 6+r5*8]
    mova                 m2, [cq+64*10+r5*8]
    mova                 m3, [cq+64*14+r5*8]
    mova                 m4, [cq+64*18+r5*8]
    mova                 m5, [cq+64*22+r5*8]
    mova                 m6, [cq+64*26+r5*8]
    mova                 m7, [cq+64*30+r5*8]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_16x4_internal_16bpc).main_oddhalf

    mova                 m0, [cq+64* 0+r5*8]
    mova                 m1, [cq+64* 4+r5*8]
    mova                 m2, [cq+64* 8+r5*8]
    mova                 m3, [cq+64*12+r5*8]
    mova                 m4, [cq+64*16+r5*8]
    mova                 m5, [cq+64*20+r5*8]
    mova                 m6, [cq+64*24+r5*8]
    mova                 m7, [cq+64*28+r5*8]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    sub                  r3, 16*(16+4*ARCH_X86_32)
    call .round_dct32

%if ARCH_X86_64
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova    [cq+64* 8+r5*8], m8
    mova    [cq+64* 9+r5*8], m9
    mova    [cq+64*10+r5*8], m10
    mova    [cq+64*11+r5*8], m11
    mova                 m8, [r3+16* 9] ;  8  9
    mova                m10, [r3+16*11] ; 10 11
    mova                m12, [r3+16*13] ; 12 13
    mova                m14, [r3+16*15] ; 14 15
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova    [cq+64* 4+r5*8], m8
    mova    [cq+64* 5+r5*8], m9
    mova    [cq+64* 6+r5*8], m10
    mova    [cq+64* 7+r5*8], m11
    mova                 m8, [r3+16* 8] ; 24 25
    mova                m10, [r3+16*10] ; 26 27
    mova                m12, [r3+16*12] ; 28 29
    mova                m14, [r3+16*14] ; 30 31
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova    [cq+64*12+r5*8], m8
    mova    [cq+64*13+r5*8], m9
    mova    [cq+64*14+r5*8], m10
    mova    [cq+64*15+r5*8], m11
%else
    sub                  r3, 8*16
    mova                 m0, [r3+ 8*16]
    mova                 m2, [r3+10*16]
    mova                 m4, [r3+12*16]
    mova                 m6, [r3+14*16]
    packssdw             m0, [r3+ 9*16]
    packssdw             m2, [r3+11*16]
    packssdw             m4, [r3+13*16]
    packssdw             m6, [r3+15*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova    [cq+64* 4+r5*8], m0
    mova    [cq+64* 5+r5*8], m1
    mova    [cq+64* 6+r5*8], m2
    mova    [cq+64* 7+r5*8], m3
    mova                 m0, [r3+16*16]
    mova                 m2, [r3+18*16]
    mova                 m4, [r3+20*16]
    mova                 m6, [r3+22*16]
    packssdw             m0, [r3+17*16]
    packssdw             m2, [r3+19*16]
    packssdw             m4, [r3+21*16]
    packssdw             m6, [r3+23*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova    [cq+64* 8+r5*8], m0
    mova    [cq+64* 9+r5*8], m1
    mova    [cq+64*10+r5*8], m2
    mova    [cq+64*11+r5*8], m3
    mova                 m0, [r3+31*16]
    mova                 m2, [r3+29*16]
    mova                 m4, [r3+27*16]
    mova                 m6, [r3+25*16]
    packssdw             m0, [r3+30*16]
    packssdw             m2, [r3+28*16]
    packssdw             m4, [r3+26*16]
    packssdw             m6, [r3+24*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova    [cq+64*12+r5*8], m0
    mova    [cq+64*13+r5*8], m1
    mova    [cq+64*14+r5*8], m2
    mova    [cq+64*15+r5*8], m3
    mova                 m0, [r3+ 0*16]
    mova                 m2, [r3+ 2*16]
    mova                 m4, [r3+ 4*16]
    mova                 m6, [r3+ 6*16]
    packssdw             m0, [r3+ 1*16]
    packssdw             m2, [r3+ 3*16]
    packssdw             m4, [r3+ 5*16]
    packssdw             m6, [r3+ 7*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
%endif
    mova    [cq+64* 0+r5*8], m0
    mova    [cq+64* 1+r5*8], m1
    mova    [cq+64* 2+r5*8], m2
    mova    [cq+64* 3+r5*8], m3
    pxor                 m0, m0
    REPX {mova [cq+x*64+r5*8], m0}, 16, 17, 18, 19, 20, 21, 22, 23, \
                                    24, 25, 26, 27, 28, 29, 30, 31
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2, we need to call this otherwise the stack pointer has
    ; the wrong offset in the 8-bit code
    call .pass2
    RET

.pass2:
%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
%if WIN64
    mov [rsp+16*16+gprsize], r7
%endif
    mov                  r7, dstq
%else
    mov [rsp+2*gprsize+16*16], dstq
%endif
    lea                  r3, [strideq*3]
    mov                 r4d, 4
    jmp m(idct_16x16_internal_16bpc).loop_pass2

.round_dct32:
%if ARCH_X86_64
    psrld               m11, 11 ; pd_1
    IDCT32_END            0, 15, 8, 9, 10, 1    ; 0 15 16 31
    mova         [r3+ 0*16], m6
    mova         [r3+23*16], m7
    IDCT32_END            1, 14, 6, 7, 10, 1    ; 1 14 17 30
    packssdw             m0, m1       ;  0  1
    packssdw            m14, m15      ; 14 15
    packssdw             m8, m6       ; 16 17
    packssdw             m7, m9       ; 30 31
    mova         [r3+16*15], m14
    mova         [r3+16*14], m7
    IDCT32_END            2, 15, 10, 7, 6, 1    ; 2 13 18 29
    IDCT32_END            3, 14,  1, 9, 6, 1    ; 3 12 19 28
    packssdw             m2, m3       ;  2  3
    packssdw            m14, m15      ; 12 13
    packssdw            m10, m1       ; 18 19
    packssdw             m9, m7       ; 28 29
    mova         [r3+16*13], m14
    mova         [r3+16*12], m9
    IDCT32_END            4, 15, 1, 7, 6, 1     ; 4 11 20 27
    IDCT32_END            5, 14, 3, 9, 6, 1     ; 5 10 21 26
    packssdw             m4, m5       ;  4  5
    packssdw            m14, m15      ; 10 11
    packssdw             m1, m3       ; 20 21
    packssdw             m9, m7       ; 26 27
    mova         [r3+16*11], m14
    mova         [r3+16*10], m9
    mova                 m6, [r3+ 0*16]
    mova                 m7, [r3+23*16]
    IDCT32_END            6, 15, 14, 5,  3, 1   ; 6 9 22 25
    IDCT32_END            7, 11,  3, 9, 13, 1   ; 7 8 23 24
    packssdw             m6, m7       ;  6  7
    packssdw            m11, m15      ;  8  9
    packssdw            m14, m3       ; 22 23
    packssdw             m9, m5       ; 24 25
    mova          [r3+16*9], m11
    mova          [r3+16*8], m9
    mova                m12, m1
    ret
%else
    mova         [r3+16*16], m0
    mova         [r3+17*16], m1
    mova         [r3+18*16], m2
    mova         [r3+19*16], m3
    mova         [r3+20*16], m4
    mova         [r3+21*16], m5
    mova         [r3+22*16], m6
    mova         [r3+23*16], m7
    pcmpeqd              m1, m1     ; -1
    mova                 m2, [o(clip_18b_min)]
    mova                 m3, [o(clip_18b_max)]

    mov                  r4, 15*16
.loop_dct32_end:
    mova                 m0, [r3+16*16]
    mova                 m6, [r3+16*24]
    psubd                m5, m0, m6 ; idct16 out15 - n
    paddd                m0, m6     ; idct16 out0  + n
    pmaxsd               m0, m2
    pmaxsd               m5, m2
    pminsd               m0, m3
    pminsd               m5, m3
    psubd                m0, m1
    psubd                m5, m1
    mova                 m7, [r3]
    mova                 m4, [r3+r4]
    psubd                m6, m0, m4 ; out31 - n
    paddd                m0, m4     ; out0  + n
    paddd                m4, m5, m7 ; out15 - n
    psubd                m5, m7     ; out16 + n
    REPX       {psrad x, 1}, m0, m5, m4, m6
    mova               [r3], m0
    mova            [r3+r4], m4
    mova         [r3+16*16], m5
    mova         [r3+24*16], m6
    add                  r3, 16
    sub                  r4, 32
    jg .loop_dct32_end
    ret
%endif

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 16
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
    add                 r5d, 384
    sar                 r5d, 9
    jmp m(inv_txfm_add_dct_dct_32x8_16bpc).dconly2

cglobal inv_txfm_add_dct_dct_32x32_16bpc, 4, 7, 16, 0-(5*32+1)*16, \
                                          dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

    ; remove entirely-zero iterations
%if ARCH_X86_32
    mov [rsp+5*32*16+1*gprsize], dstq
%elif WIN64
    mov [rsp+5*32*16+1*gprsize], r7
%endif
%undef cmp
    mov                 r5d, 14
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jge .end_zero_loop
    pxor                 m0, m0
.zero_loop:
    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
    mova   [rsp+32*16+r5*8+0*32*16], m0
    mova   [rsp+40*16+r5*8+0*32*16], m0
    mova   [rsp+32*16+t0*8+0*32*16], m0
    mova   [rsp+32*16+t1*8+0*32*16], m0
    mova   [rsp+32*16+r5*8+1*32*16], m0
    mova   [rsp+40*16+r5*8+1*32*16], m0
    mova   [rsp+32*16+t0*8+1*32*16], m0
    mova   [rsp+32*16+t1*8+1*32*16], m0
    mova   [rsp+32*16+r5*8+2*32*16], m0
    mova   [rsp+40*16+r5*8+2*32*16], m0
    mova   [rsp+32*16+t0*8+2*32*16], m0
    mova   [rsp+32*16+t1*8+2*32*16], m0
    mova   [rsp+32*16+r5*8+3*32*16], m0
    mova   [rsp+40*16+r5*8+3*32*16], m0
    mova   [rsp+32*16+t0*8+3*32*16], m0
    mova   [rsp+32*16+t1*8+3*32*16], m0
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jl .zero_loop
.end_zero_loop:

    ; actual first pass after skipping all-zero data
    mov [rsp+gprsize*0+5*32*16], eobd
.loop_pass1:
    mova                 m0, [cq+128* 1+r5*8]
    mova                 m1, [cq+128* 7+r5*8]
    mova                 m2, [cq+128* 9+r5*8]
    mova                 m3, [cq+128*15+r5*8]
    mova                 m4, [cq+128*17+r5*8]
    mova                 m5, [cq+128*23+r5*8]
    mova                 m6, [cq+128*25+r5*8]
    mova                 m7, [cq+128*31+r5*8]
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mov                  r3, rsp
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part1
    mova                 m0, [cq+128* 3+r5*8]
    mova                 m1, [cq+128* 5+r5*8]
    mova                 m2, [cq+128*11+r5*8]
    mova                 m3, [cq+128*13+r5*8]
    mova                 m4, [cq+128*19+r5*8]
    mova                 m5, [cq+128*21+r5*8]
    mova                 m6, [cq+128*27+r5*8]
    mova                 m7, [cq+128*29+r5*8]
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part2
    mova                 m0, [cq+128* 2+r5*8]
    mova                 m1, [cq+128* 6+r5*8]
    mova                 m2, [cq+128*10+r5*8]
    mova                 m3, [cq+128*14+r5*8]
    mova                 m4, [cq+128*18+r5*8]
    mova                 m5, [cq+128*22+r5*8]
    mova                 m6, [cq+128*26+r5*8]
    mova                 m7, [cq+128*30+r5*8]
    add                  r3, 16*(16+4*ARCH_X86_32)
    call m(idct_16x4_internal_16bpc).main_oddhalf
    mova                 m0, [cq+128* 0+r5*8]
    mova                 m1, [cq+128* 4+r5*8]
    mova                 m2, [cq+128* 8+r5*8]
    mova                 m3, [cq+128*12+r5*8]
    mova                 m4, [cq+128*16+r5*8]
    mova                 m5, [cq+128*20+r5*8]
    mova                 m6, [cq+128*24+r5*8]
    mova                 m7, [cq+128*28+r5*8]
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    sub                  r3, 16*(16+4*ARCH_X86_32)
    call m(inv_txfm_add_dct_dct_32x8_16bpc).round_dct32
    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
%if ARCH_X86_64
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova   [rsp+32*16+r5*8+2*32*16], m8
    mova   [rsp+40*16+r5*8+2*32*16], m10
    mova   [rsp+32*16+t1*8+2*32*16], m9
    mova   [rsp+32*16+t0*8+2*32*16], m11
    mova                 m8, [r3+16* 9] ;  8  9
    mova                m10, [r3+16*11] ; 10 11
    mova                m12, [r3+16*13] ; 12 13
    mova                m14, [r3+16*15] ; 14 15
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova   [rsp+32*16+r5*8+1*32*16], m8
    mova   [rsp+40*16+r5*8+1*32*16], m10
    mova   [rsp+32*16+t1*8+1*32*16], m9
    mova   [rsp+32*16+t0*8+1*32*16], m11
    mova                 m8, [r3+16* 8] ; 24 25
    mova                m10, [r3+16*10] ; 26 27
    mova                m12, [r3+16*12] ; 28 29
    mova                m14, [r3+16*14] ; 30 31
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova   [rsp+32*16+r5*8+3*32*16], m8
    mova   [rsp+40*16+r5*8+3*32*16], m10
    mova   [rsp+32*16+t1*8+3*32*16], m9
    mova   [rsp+32*16+t0*8+3*32*16], m11
%else
    sub                  r3, 8*16
    mova                 m0, [r3+ 8*16]
    mova                 m2, [r3+10*16]
    mova                 m4, [r3+12*16]
    mova                 m6, [r3+14*16]
    packssdw             m0, [r3+ 9*16]
    packssdw             m2, [r3+11*16]
    packssdw             m4, [r3+13*16]
    packssdw             m6, [r3+15*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova   [rsp+32*16+r5*8+1*32*16], m0
    mova   [rsp+40*16+r5*8+1*32*16], m2
    mova   [rsp+32*16+t1*8+1*32*16], m1
    mova   [rsp+32*16+t0*8+1*32*16], m3
    mova                 m0, [r3+16*16]
    mova                 m2, [r3+18*16]
    mova                 m4, [r3+20*16]
    mova                 m6, [r3+22*16]
    packssdw             m0, [r3+17*16]
    packssdw             m2, [r3+19*16]
    packssdw             m4, [r3+21*16]
    packssdw             m6, [r3+23*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova   [rsp+32*16+r5*8+2*32*16], m0
    mova   [rsp+40*16+r5*8+2*32*16], m2
    mova   [rsp+32*16+t1*8+2*32*16], m1
    mova   [rsp+32*16+t0*8+2*32*16], m3
    mova                 m0, [r3+31*16]
    mova                 m2, [r3+29*16]
    mova                 m4, [r3+27*16]
    mova                 m6, [r3+25*16]
    packssdw             m0, [r3+30*16]
    packssdw             m2, [r3+28*16]
    packssdw             m4, [r3+26*16]
    packssdw             m6, [r3+24*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova   [rsp+32*16+r5*8+3*32*16], m0
    mova   [rsp+40*16+r5*8+3*32*16], m2
    mova   [rsp+32*16+t1*8+3*32*16], m1
    mova   [rsp+32*16+t0*8+3*32*16], m3
    mova                 m0, [r3+ 0*16]
    mova                 m2, [r3+ 2*16]
    mova                 m4, [r3+ 4*16]
    mova                 m6, [r3+ 6*16]
    packssdw             m0, [r3+ 1*16]
    packssdw             m2, [r3+ 3*16]
    packssdw             m4, [r3+ 5*16]
    packssdw             m6, [r3+ 7*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
%endif
    pxor                 m7, m7
    ; clear lower half of [cq]
    REPX {mova [cq+x*128+r5*8], m7}, 0, 1, 2, 3, 4, 5, 6, 7, \
                                     8, 9, 10, 11, 12, 13, 14, 15, \
                                     16, 17, 18, 19, 20, 21, 22, 23, \
                                     24, 25, 26, 27, 28, 29, 30, 31
    mova   [rsp+32*16+r5*8+0*32*16], m0
    mova   [rsp+40*16+r5*8+0*32*16], m2
    mova   [rsp+32*16+t1*8+0*32*16], m1
    mova   [rsp+32*16+t0*8+0*32*16], m3
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2 code starts here
    mov                eobd, [rsp+gprsize*0+5*32*16]
    add                 rsp, 29*16
    cmp                eobd, 36
    jl .load_veryfast
    cmp                eobd, 136
    jl .load_fast
    ; load normal
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main)]
    jmp .run
.load_fast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_fast)]
    jmp .run
.load_veryfast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_veryfast)]
    ; fall-through
.run:
%if ARCH_X86_64
    lea                  r2, [dstq+64]
    mov                  r7, -8
%else
    lea                  r2, [rsp+(4*32+3)*16]
    mov dword [r2+0*gprsize], 4
%endif
    jmp m(inv_txfm_add_dct_dct_16x32_16bpc).loop_pass2_entry

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 32
    add                 rsp, (5*32+1-(24+8*ARCH_X86_32))*16
    jmp m(inv_txfm_add_dct_dct_32x8_16bpc).dconly1

cglobal inv_txfm_add_dct_dct_16x64_16bpc, 4, 7, 16, \
                                          0-(12+2*64)*16-(4+4*ARCH_X86_32)*gprsize, \
                                          dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

%if ARCH_X86_32
    DECLARE_REG_TMP 4, 1, 2, 0
    mov [rsp+gprsize*1+(64*2+12)*16], r0
    mov [rsp+gprsize*2+(64*2+12)*16], r1
    mov [rsp+gprsize*3+(64*2+12)*16], r2
%else
    DECLARE_REG_TMP 8, 9, 4, 7
    mov [rsp+gprsize*1+(64*2+12)*16], r9
%if WIN64
    mov [rsp+gprsize*2+(64*2+12)*16], r7
    mov [rsp+gprsize*3+(64*2+12)*16], r8
%endif
%endif
%undef cmp
    ; remove entirely-zero iterations
    mov                 r5d, 7*2
    cmp                eobw, word [o2(tbl_16x32_2d)+r5]
    jge .end_zero_loop
    pxor                 m0, m0
.zero_loop:
    movzx               t1d, word [o2(tbl_Nx64_offset)+r5*2+0]
    movzx               t3d, word [o2(tbl_Nx64_offset)+r5*2+2]
    movzx               t0d, t1b
    movzx               t2d, t3b
    shr                 t1d, 8
    shr                 t3d, 8
    mova   [rsp+12*16+t0*8], m0
    mova   [rsp+12*16+t1*8], m0
    mova   [rsp+12*16+t2*8], m0
    mova   [rsp+12*16+t3*8], m0
    mova   [rsp+76*16+t0*8], m0
    mova   [rsp+76*16+t1*8], m0
    mova   [rsp+76*16+t2*8], m0
    mova   [rsp+76*16+t3*8], m0
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_16x32_2d)+r5]
    jl .zero_loop
.end_zero_loop:
    ; actual first pass after skipping all-zero data
    mov [rsp+gprsize*0+(64*2+12)*16], eobd
    mov                  r3, rsp
%if ARCH_X86_32
    DECLARE_REG_TMP 4, 1, 6, 0
    mov                  r2, [rsp+gprsize*3+(64*2+12)*16]
    mov [rsp+gprsize*3+(64*2+12)*16], r6
%endif
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+ 1*128+r5*8]
    mova                 m1, [cq+ 3*128+r5*8]
    mova                 m2, [cq+ 5*128+r5*8]
    mova                 m3, [cq+ 7*128+r5*8]
    mova                 m4, [cq+ 9*128+r5*8]
    mova                 m5, [cq+11*128+r5*8]
    mova                 m6, [cq+13*128+r5*8]
    mova                 m7, [cq+15*128+r5*8]
    call m(idct_16x4_internal_16bpc).main_oddhalf

    mova                 m0, [cq+ 0*128+r5*8]
    mova                 m1, [cq+ 2*128+r5*8]
    mova                 m2, [cq+ 4*128+r5*8]
    mova                 m3, [cq+ 6*128+r5*8]
    mova                 m4, [cq+ 8*128+r5*8]
    mova                 m5, [cq+10*128+r5*8]
    mova                 m6, [cq+12*128+r5*8]
    mova                 m7, [cq+14*128+r5*8]
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    call m(idct_16x16_internal_16bpc).round
%if ARCH_X86_64
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    packssdw             m8, m9
    packssdw            m10, m11
    packssdw            m12, m13
    packssdw            m14, m15
%endif
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    movzx               t1d, word [o2(tbl_Nx64_offset)+r5*2+0]
    movzx               t3d, word [o2(tbl_Nx64_offset)+r5*2+2]
    movzx               t0d, t1b
    movzx               t2d, t3b
    shr                 t1d, 8
    shr                 t3d, 8
%if ARCH_X86_64
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova   [rsp+76*16+t0*8], m8
    mova   [rsp+76*16+t1*8], m9
    mova   [rsp+76*16+t2*8], m10
    mova   [rsp+76*16+t3*8], m11
%else
    mova   [rsp+76*16+t0*8], m0
    mova   [rsp+76*16+t1*8], m1
    mova   [rsp+76*16+t2*8], m2
    mova   [rsp+76*16+t3*8], m3
    mova                 m0, [rsp+ 8*16]
    mova                 m2, [rsp+ 9*16]
    mova                 m4, [rsp+10*16]
    mova                 m6, [rsp+11*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
%endif
    mova   [rsp+12*16+t0*8], m0
    mova   [rsp+12*16+t1*8], m1
    mova   [rsp+12*16+t2*8], m2
    mova   [rsp+12*16+t3*8], m3
%if ARCH_X86_32
    mov                  r6, [rsp+gprsize*3+(64*2+12)*16]
%endif
    pxor                 m7, m7
    REPX {mova [cq+x*128+r5*8], m7}, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2
    mov                eobd, [rsp+gprsize*0+(64*2+12)*16]
    cmp                eobd, 151
    jl .fast
    ; fall-through
%if ARCH_X86_64
    DECLARE_REG_TMP 8, 9
%else
    DECLARE_REG_TMP 1, 5
%endif
    lea                  t0, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_fast)]
    lea                  t1, [o(m_suffix(idct_16x64_internal_8bpc, _ssse3).main)]
    jmp .run
.fast:
    lea                  t0, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_veryfast)]
    lea                  t1, [o(m_suffix(idct_16x64_internal_8bpc, _ssse3).main_fast)]
.run:
    add                 rsp, 9*16

%if ARCH_X86_64
    lea                  r2, [dstq+32]
    mov                  r7, -4
%else
    lea                  r2, [rsp+(64*2+3)*16]
    mov      [r2+4*gprsize], t0
    mov      [r2+5*gprsize], t1
    mov                  r1, [r2+2*gprsize]
    mov dword [r2+0*gprsize], 2
%endif
.loop_pass2:
%if ARCH_X86_32
    mov                dstq, [r2+1*gprsize]
%endif
    call .pass2
    add                 rsp, 64*16
%if ARCH_X86_64
    add                  r7, 2
    lea                dstq, [r2+r7*8]
    jl .loop_pass2
%else
    add dword [r2+1*gprsize], 16
    dec dword [r2+0*gprsize]
    jg .loop_pass2
%endif
%assign stack_size (stack_size-(64*2+9)*16)
%if STACK_ALIGNMENT >= 16
%assign stack_size_padded (stack_size_padded-(64*2+9)*16)
%assign stack_offset (stack_offset-(64*2+9)*16)
%else
%xdefine rstkm [rsp + stack_size]
%endif
%if ARCH_X86_64
    mov                  r9, [rsp+gprsize*1+3*16]
%if WIN64
    mov                  r7, [rsp+gprsize*2+3*16]
    mov                  r8, [rsp+gprsize*3+3*16]
%endif
%endif
    RET

.pass2:
%if ARCH_X86_32
    lea                  r5, [o(itx8_start)]
%endif
    mova                 m0, [rsp+gprsize+16* 3]
    mova                 m1, [rsp+gprsize+16* 4]
    mova                 m2, [rsp+gprsize+16* 5]
    mova                 m3, [rsp+gprsize+16* 6]
    pxor                 m4, m4
    REPX       {mova x, m4}, m5, m6, m7
    call m_suffix(idct_8x8_internal_8bpc, _ssse3).main
    mova [rsp+gprsize+ 3*16], m0
    mova [rsp+gprsize+ 4*16], m1
    mova [rsp+gprsize+ 5*16], m2
    mova [rsp+gprsize+ 6*16], m3
    mova [rsp+gprsize+ 7*16], m4
    mova [rsp+gprsize+ 8*16], m5
    mova [rsp+gprsize+ 9*16], m6
    mova [rsp+gprsize+10*16], m7
    mova                 m0, [rsp+gprsize+16*11]
    mova                 m1, [rsp+gprsize+16*12]
    mova                 m2, [rsp+gprsize+16*13]
    mova                 m3, [rsp+gprsize+16*14]
    pxor                 m4, m4
    REPX       {mova x, m4}, m5, m6, m7
    call m_suffix(idct_16x8_internal_8bpc, _ssse3).main
    mova                 m7, [rsp+gprsize+ 0*16]
    mova [rsp+gprsize+11*16], m0
    mova [rsp+gprsize+12*16], m1
    mova [rsp+gprsize+13*16], m2
    mova [rsp+gprsize+14*16], m3
    mova [rsp+gprsize+15*16], m4
    mova [rsp+gprsize+16*16], m5
    mova [rsp+gprsize+17*16], m6
    mova [rsp+gprsize+18*16], m7
%if ARCH_X86_64
    call                  r8
%else
    call      [r2+4*gprsize]
%endif
    mova [rsp+gprsize+ 3*16], m0
    mova [rsp+gprsize+ 5*16], m2
    mova [rsp+gprsize+ 8*16], m5
    mova [rsp+gprsize+10*16], m7
%if ARCH_X86_64
    call                 r9
    mova                 m8, [o(pw_2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
%else
    call     [r2+5*gprsize]
%endif
    lea                  r3, [strideq*3]
    lea                  r4, [rsp+gprsize+ 3*16]
%if ARCH_X86_64
    mov                 r6d, 8
%else
    mov dword [r2+2*gprsize], 8
%endif
.loop_write:
    mova                 m0, [r4+0*16]
    mova                 m1, [r4+1*16]
    mova                 m2, [r4+2*16]
    mova                 m3, [r4+3*16]
    mova                 m4, [r4+4*16]
    mova                 m5, [r4+5*16]
    mova                 m6, [r4+6*16]
    mova                 m7, [r4+7*16]
    call m(idct_8x8_internal_16bpc).round1_and_write_8x8
    lea                dstq, [dstq+strideq*8]
    add                  r4, 8*16
%if ARCH_X86_64
    dec                 r6d
%else
    dec dword [r2+2*gprsize]
%endif
    jg .loop_write
    ret

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 64
    add                 r5d, 640
    sar                 r5d, 10
    add                 rsp, (12+2*64)*16+(4+4*ARCH_X86_32)*gprsize-(8+4*ARCH_X86_32)*16
    jmp m(inv_txfm_add_dct_dct_16x4_16bpc).dconly2

cglobal inv_txfm_add_dct_dct_32x64_16bpc, 4, 7, 16, \
                                          0-(32+4*64)*16-(4+4*ARCH_X86_32)*gprsize, \
                                          dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

%if ARCH_X86_32
    DECLARE_REG_TMP 4, 1, 2, 0
    mov [rsp+gprsize*1+(64*4+32)*16], r0
    mov [rsp+gprsize*2+(64*4+32)*16], r1
    mov [rsp+gprsize*3+(64*4+32)*16], r2
%else
    DECLARE_REG_TMP 8, 9, 4, 7
    mov [rsp+gprsize*1+(64*4+32)*16], r9
%if WIN64
    mov [rsp+gprsize*2+(64*4+32)*16], r7
    mov [rsp+gprsize*3+(64*4+32)*16], r8
%endif
%endif
%undef cmp
    ; remove entirely-zero iterations
    mov                 r5d, 7*2
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jge .end_zero_loop
    pxor                 m0, m0
.zero_loop:
    movzx               t1d, word [o2(tbl_Nx64_offset)+r5*2+0]
    movzx               t3d, word [o2(tbl_Nx64_offset)+r5*2+2]
    movzx               t0d, t1b
    movzx               t2d, t3b
    shr                 t1d, 8
    shr                 t3d, 8
    mova  [rsp+ 32*16+t0*8], m0
    mova  [rsp+ 32*16+t1*8], m0
    mova  [rsp+ 32*16+t2*8], m0
    mova  [rsp+ 32*16+t3*8], m0
    mova  [rsp+ 96*16+t0*8], m0
    mova  [rsp+ 96*16+t1*8], m0
    mova  [rsp+ 96*16+t2*8], m0
    mova  [rsp+ 96*16+t3*8], m0
    mova  [rsp+160*16+t0*8], m0
    mova  [rsp+160*16+t1*8], m0
    mova  [rsp+160*16+t2*8], m0
    mova  [rsp+160*16+t3*8], m0
    mova  [rsp+224*16+t0*8], m0
    mova  [rsp+224*16+t1*8], m0
    mova  [rsp+224*16+t2*8], m0
    mova  [rsp+224*16+t3*8], m0
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jl .zero_loop
.end_zero_loop:
    ; actual first pass after skipping all-zero data
    mov [rsp+gprsize*0+(64*4+32)*16], eobd
    mov                  r3, rsp
%if ARCH_X86_32
    DECLARE_REG_TMP 4, 1, 6, 0
    mov                  r2, [rsp+gprsize*3+(64*4+32)*16]
    mov [rsp+gprsize*3+(64*4+32)*16], r6
%endif
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif
    mova                 m0, [cq+128* 1+r5*8]
    mova                 m1, [cq+128* 7+r5*8]
    mova                 m2, [cq+128* 9+r5*8]
    mova                 m3, [cq+128*15+r5*8]
    mova                 m4, [cq+128*17+r5*8]
    mova                 m5, [cq+128*23+r5*8]
    mova                 m6, [cq+128*25+r5*8]
    mova                 m7, [cq+128*31+r5*8]
    mov                  r3, rsp
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part1

    mova                 m0, [cq+128* 3+r5*8]
    mova                 m1, [cq+128* 5+r5*8]
    mova                 m2, [cq+128*11+r5*8]
    mova                 m3, [cq+128*13+r5*8]
    mova                 m4, [cq+128*19+r5*8]
    mova                 m5, [cq+128*21+r5*8]
    mova                 m6, [cq+128*27+r5*8]
    mova                 m7, [cq+128*29+r5*8]
%if ARCH_X86_32
    add                  r3, 16*8
%endif
    call m(idct_8x4_internal_16bpc).rect2_mul
%if ARCH_X86_32
    sub                  r3, 16*8
%endif
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part2
    add                  r3, 16*(16+4*ARCH_X86_32)

    mova                 m0, [cq+128* 2+r5*8]
    mova                 m1, [cq+128* 6+r5*8]
    mova                 m2, [cq+128*10+r5*8]
    mova                 m3, [cq+128*14+r5*8]
    mova                 m4, [cq+128*18+r5*8]
    mova                 m5, [cq+128*22+r5*8]
    mova                 m6, [cq+128*26+r5*8]
    mova                 m7, [cq+128*30+r5*8]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_16x4_internal_16bpc).main_oddhalf

    mova                 m0, [cq+128* 0+r5*8]
    mova                 m1, [cq+128* 4+r5*8]
    mova                 m2, [cq+128* 8+r5*8]
    mova                 m3, [cq+128*12+r5*8]
    mova                 m4, [cq+128*16+r5*8]
    mova                 m5, [cq+128*20+r5*8]
    mova                 m6, [cq+128*24+r5*8]
    mova                 m7, [cq+128*28+r5*8]
    call m(idct_8x4_internal_16bpc).rect2_mul
    call m(idct_8x4_internal_16bpc).main_pass1
    call m(idct_8x4_internal_16bpc).round
    sub                  r3, 16*(16+4*ARCH_X86_32)
    call m(inv_txfm_add_dct_dct_32x16_16bpc).round_dct32

    movzx               t1d, word [o2(tbl_Nx64_offset)+r5*2+0]
    movzx               t3d, word [o2(tbl_Nx64_offset)+r5*2+2]
    movzx               t0d, t1b
    movzx               t2d, t3b
    shr                 t1d, 8
    shr                 t3d, 8
%if ARCH_X86_64
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova  [rsp+160*16+t0*8], m8
    mova  [rsp+160*16+t1*8], m9
    mova  [rsp+160*16+t2*8], m10
    mova  [rsp+160*16+t3*8], m11
    mova                 m8, [r3+16* 9] ;  8  9
    mova                m10, [r3+16*11] ; 10 11
    mova                m12, [r3+16*13] ; 12 13
    mova                m14, [r3+16*15] ; 14 15
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova  [rsp+ 96*16+t0*8], m8
    mova  [rsp+ 96*16+t1*8], m9
    mova  [rsp+ 96*16+t2*8], m10
    mova  [rsp+ 96*16+t3*8], m11
    mova                 m8, [r3+16* 8] ; 24 25
    mova                m10, [r3+16*10] ; 26 27
    mova                m12, [r3+16*12] ; 28 29
    mova                m14, [r3+16*14] ; 30 31
    call m(idct_16x4_internal_16bpc).transpose4x8packed_hi
    mova  [rsp+224*16+t0*8], m8
    mova  [rsp+224*16+t1*8], m9
    mova  [rsp+224*16+t2*8], m10
    mova  [rsp+224*16+t3*8], m11
%else
    sub                  r3, 8*16
    mova                 m0, [r3+ 8*16]
    mova                 m2, [r3+10*16]
    mova                 m4, [r3+12*16]
    mova                 m6, [r3+14*16]
    packssdw             m0, [r3+ 9*16]
    packssdw             m2, [r3+11*16]
    packssdw             m4, [r3+13*16]
    packssdw             m6, [r3+15*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova  [rsp+ 96*16+t0*8], m0
    mova  [rsp+ 96*16+t1*8], m1
    mova  [rsp+ 96*16+t2*8], m2
    mova  [rsp+ 96*16+t3*8], m3
    mova                 m0, [r3+16*16]
    mova                 m2, [r3+18*16]
    mova                 m4, [r3+20*16]
    mova                 m6, [r3+22*16]
    packssdw             m0, [r3+17*16]
    packssdw             m2, [r3+19*16]
    packssdw             m4, [r3+21*16]
    packssdw             m6, [r3+23*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova  [rsp+160*16+t0*8], m0
    mova  [rsp+160*16+t1*8], m1
    mova  [rsp+160*16+t2*8], m2
    mova  [rsp+160*16+t3*8], m3
    mova                 m0, [r3+31*16]
    mova                 m2, [r3+29*16]
    mova                 m4, [r3+27*16]
    mova                 m6, [r3+25*16]
    packssdw             m0, [r3+30*16]
    packssdw             m2, [r3+28*16]
    packssdw             m4, [r3+26*16]
    packssdw             m6, [r3+24*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova  [rsp+224*16+t0*8], m0
    mova  [rsp+224*16+t1*8], m1
    mova  [rsp+224*16+t2*8], m2
    mova  [rsp+224*16+t3*8], m3
    mova                 m0, [r3+ 0*16]
    mova                 m2, [r3+ 2*16]
    mova                 m4, [r3+ 4*16]
    mova                 m6, [r3+ 6*16]
    packssdw             m0, [r3+ 1*16]
    packssdw             m2, [r3+ 3*16]
    packssdw             m4, [r3+ 5*16]
    packssdw             m6, [r3+ 7*16]
    call m(idct_8x4_internal_16bpc).transpose4x8packed
%endif
    mova  [rsp+ 32*16+t0*8], m0
    mova  [rsp+ 32*16+t1*8], m1
    mova  [rsp+ 32*16+t2*8], m2
    mova  [rsp+ 32*16+t3*8], m3
    pxor                 m0, m0
    REPX {mova [cq+x*128+r5*8], m0}, 0, 1, 2, 3, 4, 5, 6, 7, \
                                     8, 9, 10, 11, 12, 13, 14, 15, \
                                     16, 17, 18, 19, 20, 21, 22, 23, \
                                     24, 25, 26, 27, 28, 29, 30, 31
%if ARCH_X86_32
    mov                  r6, [rsp+gprsize*3+(64*4+32)*16]
%endif
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2
    mov                eobd, [rsp+gprsize*0+(64*4+32)*16]
    cmp                eobd, 136
    jl .fast
    ; fall-through
%if ARCH_X86_64
    DECLARE_REG_TMP 8, 9
%else
    DECLARE_REG_TMP 1, 5
%endif
    lea                  t0, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_fast)]
    lea                  t1, [o(m_suffix(idct_16x64_internal_8bpc, _ssse3).main)]
    jmp .run
.fast:
    lea                  t0, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_veryfast)]
    lea                  t1, [o(m_suffix(idct_16x64_internal_8bpc, _ssse3).main_fast)]
.run:
    add                 rsp, 29*16

%if ARCH_X86_64
    lea                  r2, [dstq+64]
    mov                  r7, -8
%else
    lea                  r2, [rsp+(64*4+3)*16]
    mov      [r2+4*gprsize], t0
    mov      [r2+5*gprsize], t1
    mov                  r1, [r2+2*gprsize]
    mov dword [r2+0*gprsize], 4
%endif
    jmp m(inv_txfm_add_dct_dct_16x64_16bpc).loop_pass2

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 64
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
    add                 r5d, 384
    sar                 r5d, 9
    add                 rsp, (32+4*64)*16+(4+4*ARCH_X86_32)*gprsize-(24+8*ARCH_X86_32)*16
    jmp m(inv_txfm_add_dct_dct_32x8_16bpc).dconly2

cglobal inv_txfm_add_dct_dct_64x16_16bpc, 4, 7, 16, 0-(64+8*ARCH_X86_32)*16, \
                                         dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

    ; remove entirely-zero iterations
%undef cmp
    mov                 r5d, 8
.zero_loop:
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_32x16_2d)+r5]
    jl .zero_loop

    ; actual first pass after skipping all-zero data
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif

    mov                  r3, rsp
    lea                  r4, [o(idct64_mul_16bpc)]
    mova                 m0, [cq+64* 1+r5*8]
    mova                 m1, [cq+64*31+r5*8]
    mova                 m2, [cq+64*17+r5*8]
    mova                 m3, [cq+64*15+r5*8]
    call .main_part1
    mova                 m0, [cq+64* 7+r5*8]
    mova                 m1, [cq+64*25+r5*8]
    mova                 m2, [cq+64*23+r5*8]
    mova                 m3, [cq+64* 9+r5*8]
    call .main_part1
    mova                 m0, [cq+64* 5+r5*8]
    mova                 m1, [cq+64*27+r5*8]
    mova                 m2, [cq+64*21+r5*8]
    mova                 m3, [cq+64*11+r5*8]
    call .main_part1
    mova                 m0, [cq+64* 3+r5*8]
    mova                 m1, [cq+64*29+r5*8]
    mova                 m2, [cq+64*19+r5*8]
    mova                 m3, [cq+64*13+r5*8]
    call .main_part1
    call .main_part2

    mova                 m0, [cq+64* 2+r5*8]
    mova                 m1, [cq+64*14+r5*8]
    mova                 m2, [cq+64*18+r5*8]
    mova                 m3, [cq+64*30+r5*8]
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part1_fast

    mova                 m0, [cq+64* 6+r5*8]
    mova                 m1, [cq+64*10+r5*8]
    mova                 m2, [cq+64*22+r5*8]
    mova                 m3, [cq+64*26+r5*8]
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part2_fast
    add                  r3, 16*(24+4*ARCH_X86_32)

    mova                 m0, [cq+64* 4+r5*8]
    mova                 m1, [cq+64*12+r5*8]
    mova                 m2, [cq+64*20+r5*8]
    mova                 m3, [cq+64*28+r5*8]
    call m(idct_16x4_internal_16bpc).main_oddhalf_fast

    mova                 m0, [cq+64* 0+r5*8]
    mova                 m1, [cq+64* 8+r5*8]
    mova                 m2, [cq+64*16+r5*8]
    mova                 m3, [cq+64*24+r5*8]
    call m(idct_8x4_internal_16bpc).main_pass1_fast
    call m(idct_8x4_internal_16bpc).round
    mova [r3-(7+4*ARCH_X86_32)*16], m1
    mova [r3-(6+4*ARCH_X86_32)*16], m2
    mova [r3-(5+4*ARCH_X86_32)*16], m3
    mova [r3-(4+4*ARCH_X86_32)*16], m4
    mova [r3-(3+4*ARCH_X86_32)*16], m5
    mova [r3-(2+4*ARCH_X86_32)*16], m6
    mova [r3-(1+4*ARCH_X86_32)*16], m7
    sub                  r3, 16*(40+4*ARCH_X86_32-4)

%if ARCH_X86_64
    psrld               m15, m11, 10 ; pd_2
%else
    mova                 m7, [o(pd_2)]
%endif
    call .main_end_loop_start

    lea                  r3, [rsp+56*16]
    lea                  r4, [cq+r5*8+64*28]
    call .shift_transpose
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2, we need to call this otherwise the stack pointer has
    ; the wrong offset in the 8-bit code
    call .pass2
    RET

.pass2:
%if ARCH_X86_64
    mova                 m8, [o(pw_2048)]
    pxor                 m9, m9
    mova                m10, [o(pixel_10bpc_max)]
%if WIN64
    mov [rsp+16*16+gprsize], r7
%endif
    mov                  r7, dstq
%else
    mov [rsp+2*gprsize+16*16], dstq
%endif
    lea                  r3, [strideq*3]
    mov                 r4d, 8
    jmp m(idct_16x16_internal_16bpc).loop_pass2

.main_part1: ; idct64 steps 1-5
    ; in1/31/17/15 -> t32a/33/34a/35/60/61a/62/63a
    ; in7/25/23/ 9 -> t56a/57/58a/59/36/37a/38/39a
    ; in5/27/21/11 -> t40a/41/42a/43/52/53a/54/55a
    ; in3/29/19/13 -> t48a/49/50a/51/44/45a/46/47a
%if ARCH_X86_64
    movd                 m7, [r4+4*0]
    movd                 m8, [r4+4*1]
    movd                 m6, [r4+4*2]
    movd                 m9, [r4+4*3]
    movd                 m5, [r4+4*4]
    movd                m10, [r4+4*5]
    movd                 m4, [r4+4*6]
    movd                m15, [r4+4*7]
    REPX {pshufd x, x, q0000}, m7, m8, m6, m9, m5, m10, m4, m15
    pmulld               m7, m0     ; t63a
    pmulld               m0, m8     ; t32a
    pmulld               m6, m1     ; t62a
    pmulld               m1, m9     ; t33a
    pmulld               m5, m2     ; t61a
    pmulld               m2, m10    ; t34a
    pmulld               m4, m3     ; t60a
    pmulld               m3, m15    ; t35a
    movd                m10, [r4+4*8]
    movd                m15, [r4+4*9]
    REPX {pshufd x, x, q0000}, m10, m15
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
    ITX_MULSUB_2D         2, 6, 5, 9, _, 11, 10, 15, 4 ; t61a, t34a
    REPX    {pmaxsd x, m12}, m0, m3, m7, m4
    REPX    {pminsd x, m13}, m0, m3, m7, m4
    movd                m10, [r4+4*10]
    movd                m15, [r4+4*11]
    REPX {pshufd x, x, q0000}, m10, m15
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
    add                  r4, 4*12
    mova          [r3+16*0], m0
    mova          [r3+16*7], m7
    mova          [r3+16*1], m1
    mova          [r3+16*6], m8
    mova          [r3+16*2], m6
    mova          [r3+16*5], m4
    mova          [r3+16*3], m3
    mova          [r3+16*4], m5
%else
    movd                 m7, [r4+4*0]
    movd                 m6, [r4+4*2]
    movd                 m5, [r4+4*4]
    movd                 m4, [r4+4*6]
    REPX {pshufd x, x, q0000}, m7, m6, m5, m4
    pmulld               m7, m0     ; t63a
    pmulld               m6, m1     ; t62a
    pmulld               m5, m2     ; t61a
    pmulld               m4, m3     ; t60a
    mova          [r3+0*16], m6
    mova          [r3+1*16], m7
    movd                 m6, [r4+4*1]
    movd                 m7, [r4+4*3]
    REPX {pshufd x, x, q0000}, m7, m6
    pmulld               m0, m6     ; t32a
    pmulld               m1, m7     ; t33a
    movd                 m6, [r4+4*5]
    movd                 m7, [r4+4*7]
    REPX {pshufd x, x, q0000}, m7, m6
    pmulld               m2, m6     ; t34a
    pmulld               m3, m7     ; t35a
    mova                 m6, [r3+0*16]
    mova                 m7, [o(pd_2048)]
    REPX      {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6
    paddd                m7, [r3+1*16]
    REPX      {psrad x, 12}, m0, m1, m7, m6, m2, m3, m5, m4
    mova           [r3+0*16], m5
    psubd                m5, m0, m1 ; t33
    paddd                m0, m1     ; t32
    mova           [r3+1*16], m0
    mova                 m0, [r3+0*16]
    psubd                m1, m7, m6 ; t62
    paddd                m7, m6     ; t63
    psubd                m6, m3, m2 ; t34
    paddd                m3, m2     ; t35
    psubd                m2, m4, m0 ; t61
    paddd                m4, m0     ; t60
    mova                 m0, [o(clip_18b_min)]
    REPX     {pmaxsd x, m0}, m5, m1, m7, m6, m3, m2, m4
    pmaxsd               m0, [r3+1*16]
    mova          [r3+0*16], m0
    mova                 m0, [o(clip_18b_max)]
    REPX     {pminsd x, m0}, m5, m1, m7, m6, m3, m2, m4
    pminsd               m0, [r3+0*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m3
    mova          [r3+2*16], m4
    mova          [r3+3*16], m7
    mova                 m0, [o(pd_2048)]
    movd                 m3, [r4+4*8]
    movd                 m4, [r4+4*9]
    REPX {pshufd x, x, q0000}, m3, m4
    mova          [r3+4*16], m2
    ITX_MULSUB_2D         1, 5, 2, 7, _, 0, 3, 4    ; t33a, t62a
    mova                 m2, [r3+4*16]
    mova          [r3+4*16], m5
    ITX_MULSUB_2D         2, 6, 5, 7, _, 0, 3, 4, 4 ; t61a, t34a
    mova                 m0, [r3+0*16]
    mova                 m3, [r3+1*16]
    mova                 m4, [r3+2*16]
    mova                 m7, [r3+3*16]
    psubd                m5, m0, m3 ; t35a
    paddd                m0, m3     ; t32a
    mova          [r3+0*16], m5
    mova                 m5, [r3+4*16]
    psubd                m3, m7, m4 ; t60a
    paddd                m7, m4     ; t63a
    psubd                m4, m1, m6 ; t34
    paddd                m1, m6     ; t33
    psubd                m6, m5, m2 ; t61
    paddd                m2, m5     ; t62
    mova                 m5, [o(clip_18b_min)]
    REPX     {pmaxsd x, m5}, m0, m3, m7, m4, m1, m6, m2
    pmaxsd               m5, [r3+0*16]
    mova          [r3+0*16], m5
    mova                 m5, [o(clip_18b_max)]
    REPX     {pminsd x, m5}, m0, m3, m7, m4, m1, m6, m2
    pminsd               m5, [r3+0*16]
    mova          [r3+16*0], m0
    mova          [r3+16*7], m7
    mova          [r3+16*1], m1
    mova          [r3+16*6], m2
    mova          [r3+16*2], m4
    mova                 m7, [o(pd_2048)]
    movd                 m0, [r4+4*10]
    movd                 m1, [r4+4*11]
    REPX {pshufd x, x, q0000}, m0, m1
    ITX_MULSUB_2D         3, 5, 2, 4, _, 7, 0, 1 ; t35,  t60
    mova          [r3+16*3], m3
    mova          [r3+16*4], m5
    mova                 m4, [r3+2*16]
    ITX_MULSUB_2D         6, 4, 2, 3, _, 7, 0, 1 ; t34a, t61a
    add                  r4, 4*12
    mova          [r3+16*2], m6
    mova          [r3+16*5], m4
%endif
    add                  r3, 16*8
    ret

.main_part2: ; idct64 steps 6-9
    lea                  r4, [r3+16*7]
%if ARCH_X86_64
    mova                m10, [o(pd_1567)]
    mova                m15, [o(pd_3784)]
.main_part2_loop:
    mova                 m0, [r3-16*32] ; t32a
    mova                 m1, [r4-16*24] ; t39a
    mova                 m2, [r4-16*32] ; t63a
    mova                 m3, [r3-16*24] ; t56a
    mova                 m4, [r3-16*16] ; t40a
    mova                 m5, [r4-16* 8] ; t47a
    mova                 m6, [r4-16*16] ; t55a
    mova                 m7, [r3-16* 8] ; t48a
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
    ITX_MULSUB_2D         4, 3, 6, 9, _, 11, 10, 15, 4 ; t55a, t40a
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
    mova         [r4-16* 8], m2
    mova         [r3-16*32], m0
    mova         [r3-16* 8], m8
    mova         [r4-16*32], m1
    mova         [r4-16*24], m3
    mova         [r3-16*16], m6
    mova         [r3-16*24], m7
    mova         [r4-16*16], m5
%else
.main_part2_loop:
    mova                 m0, [r3-16*32] ; t32a
    mova                 m1, [r4-16*24] ; t39a
    mova                 m2, [r4-16*32] ; t63a
    mova                 m3, [r3-16*24] ; t56a
    mova                 m4, [r3-16*16] ; t40a
    mova                 m5, [r4-16* 8] ; t47a
    mova                 m6, [r4-16*16] ; t55a
    psubd                m7, m0, m1 ; t39
    paddd                m0, m1     ; t32
    mova          [r3+0*16], m7
    mova                 m7, [r3-16* 8] ; t48a
    psubd                m1, m2, m3 ; t56
    paddd                m2, m3     ; t63
    psubd                m3, m5, m4 ; t40
    paddd                m5, m4     ; t47
    psubd                m4, m7, m6 ; t55
    paddd                m7, m6     ; t48
    mova                 m6, [o(clip_18b_min)]
    REPX     {pmaxsd x, m6}, m0, m1, m2, m3, m5, m4, m7
    pmaxsd               m6, [r3+0*16]
    mova          [r3+0*16], m6
    mova                 m6, [o(clip_18b_max)]
    REPX     {pminsd x, m6}, m0, m1, m2, m3, m5, m4, m7
    pminsd               m6, [r3+0*16]
    mova          [r3+0*16], m0
    mova          [r3+1*16], m2
    mova          [r3+2*16], m5
    mova          [r3+3*16], m7
    mova                 m0, [o(pd_2048)]
    ITX_MULSUB_2D         1, 6, 2, 5, 7, 0, 1567, 3784    ; t39a, t56a
    ITX_MULSUB_2D         4, 3, 2, 5, _, 0,    7, 3784, 4 ; t55a, t40a
    mova                 m2, [r3+1*16]
    mova                 m7, [r3+3*16]
    psubd                m5, m2, m7 ; t48a
    paddd                m2, m7     ; t63a
    mova          [r3+1*16], m5
    mova                 m0, [r3+0*16]
    mova                 m5, [r3+2*16]
    psubd                m7, m0, m5 ; t47a
    paddd                m0, m5     ; t32a
    psubd                m5, m6, m4 ; t55
    paddd                m6, m4     ; t56
    psubd                m4, m1, m3 ; t40
    paddd                m1, m3     ; t39
    mova                 m3, [o(clip_18b_min)]
    REPX     {pmaxsd x, m3}, m2, m7, m0, m5, m6, m4, m1
    pmaxsd               m3, [r3+1*16]
    mova          [r3+0*16], m3
    mova                 m3, [o(clip_18b_max)]
    REPX     {pminsd x, m3}, m2, m7, m0, m5, m6, m4, m1
    pminsd               m3, [r3+0*16]
    mova         [r4-16* 8], m2
    mova         [r3-16*32], m0
    mova         [r3-16* 8], m6
    mova         [r4-16*32], m1
    mova                 m0, [o(pd_2896)]
    mova                 m1, [o(pd_2048)]
    REPX     {pmulld x, m0}, m3, m7, m5, m4
    REPX     {paddd  x, m1}, m3, m5
    psubd                m6, m3, m7 ; t47
    paddd                m3, m7     ; t48
    psubd                m7, m5, m4 ; t40a
    paddd                m5, m4     ; t55a
    REPX      {psrad x, 12}, m6, m3, m7, m5
    mova         [r4-16*24], m6
    mova         [r3-16*16], m3
    mova         [r3-16*24], m7
    mova         [r4-16*16], m5
%endif
    add                  r3, 16
    sub                  r4, 16
    cmp                  r3, r4
    jl .main_part2_loop
    sub                  r3, 4*16
    ret

.main_end_loop:
    mova                 m0, [r3+16*28] ; idct8  0  + n
.main_end_loop_start:
    mova                 m2, [r3+16*12] ; idct32 16 + n
    mova                 m3, [r4+16*12] ; idct32 31 - n
%if ARCH_X86_64
    mova                 m1, [r4+16*28] ; idct16 15 - n
    mova                 m4, [r4-16* 4] ; idct64 63 - n
    mova                 m5, [r3-16* 4] ; idct64 48 + n
    mova                 m6, [r4-16*20] ; idct64 47 - n
    mova                 m7, [r3-16*20] ; idct64 32 + n
    pmaxsd               m0, m12
    pminsd               m0, m13
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
    mova         [r3-16*20], m2
    mova         [r4+16*28], m1
    mova         [r4-16*20], m4
    mova         [r3+16*28], m3
    mova         [r3-16* 4], m5
    mova         [r4+16*12], m0
    mova         [r4-16* 4], m6
    mova         [r3+16*12], m8
%else
    mova                 m5, [o(clip_18b_min)]
    mova                 m6, [o(clip_18b_max)]
    mova                 m1, [r3+16*44] ; idct16 15 - n
    pmaxsd               m0, m5
    pminsd               m0, m6
    paddd                m4, m0, m1     ; idct16 out0  + n
    psubd                m0, m1         ; idct16 out15 - n
    REPX     {pmaxsd x, m5}, m4, m0
    REPX     {pminsd x, m6}, m4, m0
    paddd                m1, m4, m3     ; idct32 out0  + n
    psubd                m4, m3         ; idct32 out31 - n
    paddd                m3, m0, m2     ; idct32 out15 - n
    psubd                m0, m2         ; idct32 out16 + n
    REPX     {pmaxsd x, m5}, m1, m4, m3, m0
    REPX     {pminsd x, m6}, m1, m3, m4, m0
    REPX     {paddd  x, m7}, m1, m3, m0, m4
    mova                 m5, [r4-16* 4] ; idct64 63 - n
    mova                 m6, [r3-16* 4] ; idct64 48 + n
    paddd                m2, m1, m5     ; idct64 out0  + n (unshifted)
    psubd                m1, m5         ; idct64 out63 - n (unshifted)
    paddd                m5, m3, m6     ; idct64 out15 - n (unshifted)
    psubd                m3, m6         ; idct64 out48 + n (unshifted)
    mova         [r4+16*28], m1
    mova         [r3+16*28], m3
    mova                 m6, [r4-16*20] ; idct64 47 - n
    mova                 m1, [r3-16*20] ; idct64 32 + n
    mova         [r3-16*20], m2
    mova         [r4-16*20], m5
    paddd                m5, m0, m6     ; idct64 out16 + n (unshifted)
    psubd                m0, m6         ; idct64 out47 - n (unshifted)
    paddd                m6, m4, m1     ; idct64 out31 - n (unshifted)
    psubd                m4, m1         ; idct64 out32 + n (unshifted)
    mova         [r3-16* 4], m5
    mova         [r4+16*12], m0
    mova         [r4-16* 4], m6
    mova         [r3+16*12], m4
%endif
    sub                  r4, 16
    add                  r3, 16
    cmp                  r3, r4
    jl .main_end_loop
    ret

.shift_transpose:
    mova                 m0, [r3+0*16]
    mova                 m1, [r3+1*16]
    mova                 m2, [r3+2*16]
    mova                 m3, [r3+3*16]
    mova                 m4, [r3+4*16]
    mova                 m5, [r3+5*16]
    mova                 m6, [r3+6*16]
    mova                 m7, [r3+7*16]
    REPX       {psrad x, 2}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova          [r4+0*64], m0
    mova          [r4+1*64], m1
    mova          [r4+2*64], m2
    mova          [r4+3*64], m3
    sub                  r4, 4*64
    sub                  r3, 8*16
    cmp                  r3, rsp
    jg .shift_transpose
    ret

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 16
.dconly1:
    add                 r5d, 640
    sar                 r5d, 10
.dconly2:
    imul                r5d, 2896
    add                 r5d, 34816
    movd                 m0, r5d
    pshuflw              m0, m0, q1111
    punpcklqdq           m0, m0
    mova                 m6, [o(pixel_10bpc_max)]
    pxor                 m5, m5
.dconly_loop:
    paddw                m1, m0, [dstq+16*0]
    paddw                m2, m0, [dstq+16*1]
    paddw                m3, m0, [dstq+16*2]
    paddw                m4, m0, [dstq+16*3]
    REPX     {pmaxsw x, m5}, m1, m2, m3, m4
    REPX     {pminsw x, m6}, m1, m2, m3, m4
    mova        [dstq+16*0], m1
    mova        [dstq+16*1], m2
    mova        [dstq+16*2], m3
    mova        [dstq+16*3], m4
    add                dstq, 64
    btc                 r3d, 16
    jnc .dconly_loop
    lea                dstq, [dstq+strideq-128]
    dec                 r3d
    jg .dconly_loop
    RET

cglobal inv_txfm_add_dct_dct_64x32_16bpc, 4, 7, 16, \
                                         0-(1+64+8*ARCH_X86_32+8*32+1*WIN64)*16, \
                                         dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

%if ARCH_X86_32
    DECLARE_REG_TMP 0, 4, 1
    mov [rsp+(8*32+64+8)*16+1*gprsize], dstq
    mov [rsp+(8*32+64+8)*16+2*gprsize], strideq
%else
    DECLARE_REG_TMP 4, 7, 8
%if WIN64
    mov [rsp+(8*32+64+1)*16+1*gprsize], r7
    mov [rsp+64*16+0*gprsize], r8
%endif
%endif
%undef cmp
    ; remove entirely-zero iterations
    mov                 r5d, 14
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jge .end_zero_loop
    pxor                 m0, m0
.zero_loop:
    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
    lea                  t2, [rsp+7*32*16]
.zero_loop_inner:
    mova [t2+(64+8*ARCH_X86_32+1*WIN64)*16+r5*8], m0
    mova [t2+(72+8*ARCH_X86_32+1*WIN64)*16+r5*8], m0
    mova [t2+(64+8*ARCH_X86_32+1*WIN64)*16+t0*8], m0
    mova [t2+(64+8*ARCH_X86_32+1*WIN64)*16+t1*8], m0
    sub                  t2, 32*16
    cmp                  t2, rsp
    jge .zero_loop_inner
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jl .zero_loop
.end_zero_loop:
    mov [rsp+(8*32+64+8*ARCH_X86_32+1*WIN64)*16+0*gprsize], eobd
    ; actual first pass after skipping all-zero data
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif

    mov                  r3, rsp
    lea                  r4, [o(idct64_mul_16bpc)]
    mova                 m0, [cq+128* 1+r5*8]
    mova                 m1, [cq+128*31+r5*8]
    mova                 m2, [cq+128*17+r5*8]
    mova                 m3, [cq+128*15+r5*8]
    call .rect2_mul_fast
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    mova                 m0, [cq+128* 7+r5*8]
    mova                 m1, [cq+128*25+r5*8]
    mova                 m2, [cq+128*23+r5*8]
    mova                 m3, [cq+128* 9+r5*8]
    call .rect2_mul_fast
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    mova                 m0, [cq+128* 5+r5*8]
    mova                 m1, [cq+128*27+r5*8]
    mova                 m2, [cq+128*21+r5*8]
    mova                 m3, [cq+128*11+r5*8]
    call .rect2_mul_fast
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    mova                 m0, [cq+128* 3+r5*8]
    mova                 m1, [cq+128*29+r5*8]
    mova                 m2, [cq+128*19+r5*8]
    mova                 m3, [cq+128*13+r5*8]
    call .rect2_mul_fast
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part2

    mova                 m0, [cq+128* 2+r5*8]
    mova                 m1, [cq+128*14+r5*8]
    mova                 m2, [cq+128*18+r5*8]
    mova                 m3, [cq+128*30+r5*8]
    call .rect2_mul_fast
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part1_fast

    mova                 m0, [cq+128* 6+r5*8]
    mova                 m1, [cq+128*10+r5*8]
    mova                 m2, [cq+128*22+r5*8]
    mova                 m3, [cq+128*26+r5*8]
    call .rect2_mul_fast
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part2_fast
    add                  r3, 16*(24+4*ARCH_X86_32)

    mova                 m0, [cq+128* 4+r5*8]
    mova                 m1, [cq+128*12+r5*8]
    mova                 m2, [cq+128*20+r5*8]
    mova                 m3, [cq+128*28+r5*8]
    call .rect2_mul_fast
    call m(idct_16x4_internal_16bpc).main_oddhalf_fast

    mova                 m0, [cq+128* 0+r5*8]
    mova                 m1, [cq+128* 8+r5*8]
    mova                 m2, [cq+128*16+r5*8]
    mova                 m3, [cq+128*24+r5*8]
    call .rect2_mul_fast
    call m(idct_8x4_internal_16bpc).main_pass1_fast
    call m(idct_8x4_internal_16bpc).round
    mova [r3-(7+4*ARCH_X86_32)*16], m1
    mova [r3-(6+4*ARCH_X86_32)*16], m2
    mova [r3-(5+4*ARCH_X86_32)*16], m3
    mova [r3-(4+4*ARCH_X86_32)*16], m4
    mova [r3-(3+4*ARCH_X86_32)*16], m5
    mova [r3-(2+4*ARCH_X86_32)*16], m6
    mova [r3-(1+4*ARCH_X86_32)*16], m7
    sub                  r3, 16*(40+4*ARCH_X86_32-4)

%if ARCH_X86_64
    psrld               m15, m11, 11 ; pd_1
%else
    mova                 m7, [o(pd_1)]
%endif
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_end_loop_start

    lea                  r3, [rsp+56*16]
    lea                  t2, [rsp+7*32*16+(64+8*ARCH_X86_32+1*WIN64)*16]
    movzx               t0d, word [o2(tbl_Nx32_odd_offset)+r5]
    movzx               t1d, t0b
    shr                 t0d, 8
    call .shift_transpose
    ; zero cq
    pxor                 m7, m7
    lea                  r4, [cq+30*128+r5*8]
.zero_cq_loop:
    REPX {mova [r4+x*128], m7}, -2, -1, 0, 1
    sub                  r4, 4*128
    cmp                  r4, cq
    jg .zero_cq_loop
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2 code starts here
    mov                eobd, [rsp+gprsize*0+(8*32+64+8*ARCH_X86_32+1*WIN64)*16]
%if ARCH_X86_32
    mov             strideq, [rsp+gprsize*2+(8*32+64+8)*16]
%elif WIN64
    mov                  r8, [rsp+gprsize*0+64*16]
%endif
    add                 rsp, (64+8*ARCH_X86_32+1*WIN64-3)*16
    cmp                eobd, 36
    jl .load_veryfast
    cmp                eobd, 136
    jl .load_fast
    ; load normal
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main)]
    jmp .run
.load_fast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_fast)]
    jmp .run
.load_veryfast:
    lea                  r4, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_veryfast)]
    ; fall-through
.run:
%if ARCH_X86_64
    lea                  r2, [dstq+128]
    mov                  r7, -16
%else
    lea                  r2, [rsp+(8*32+3)*16]
    mov dword [r2+0*gprsize], 8
%endif
    jmp m(inv_txfm_add_dct_dct_16x32_16bpc).loop_pass2_entry

.rect2_mul_fast:
%if ARCH_X86_64
    REPX    {pmulld x, m14}, m0, m1, m2, m3
    REPX    {paddd  x, m11}, m0, m1, m2, m3
%else
    mova                 m4, [o(pd_2896)]
    mova                 m5, [o(pd_2048)]
    REPX    {pmulld x, m4 }, m0, m1, m2, m3
    REPX    {paddd  x, m5 }, m0, m1, m2, m3
%endif
    REPX    {psrad  x, 12 }, m0, m1, m2, m3
    ret

.shift_transpose:
    mova                 m0, [r3+0*16]
    mova                 m1, [r3+1*16]
    mova                 m2, [r3+2*16]
    mova                 m3, [r3+3*16]
    mova                 m4, [r3+4*16]
    mova                 m5, [r3+5*16]
    mova                 m6, [r3+6*16]
    mova                 m7, [r3+7*16]
    REPX       {psrad x, 1}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova     [t2+0*16+r5*8], m0
    mova     [t2+8*16+r5*8], m2
    mova     [t2+0*16+t0*8], m3
    mova     [t2+0*16+t1*8], m1
    sub                  t2, 16*32
    sub                  r3, 8*16
    cmp                  r3, rsp
    jg .shift_transpose
    ret

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 32
    add                 r5d, 128
    sar                 r5d, 8
    imul                r5d, 181
    add                 r5d, 384
    sar                 r5d, 9
    add                 rsp, (1+8*32+1*WIN64)*16
    jmp m(inv_txfm_add_dct_dct_64x16_16bpc).dconly2

cglobal inv_txfm_add_dct_dct_64x64_16bpc, 4, 7, 16, \
                                         0-(64+8*ARCH_X86_32+8*64+1*ARCH_X86_64)*16-(4+4*ARCH_X86_32)*gprsize, \
                                         dst, stride, c, eob
    LEA                  r6, base
    test               eobd, eobd
    jz .dconly

%if ARCH_X86_32
    DECLARE_REG_TMP 4, 1, 2, 0, 6
    mov [rsp+gprsize*1+(64*9+8)*16], r0
    mov [rsp+gprsize*2+(64*9+8)*16], r1
    mov [rsp+gprsize*3+(64*9+8)*16], r2
    mov [rsp+gprsize*4+(64*9+8)*16], r6
%else
    DECLARE_REG_TMP 8, 9, 4, 7, 0
    mov [rsp+gprsize*1+(64*9+1)*16], r9
    mov [rsp+gprsize*0+64*16], r0
%if WIN64
    mov [rsp+gprsize*2+(64*9+1)*16], r7
    mov [rsp+gprsize*3+(64*9+1)*16], r8
%endif
%endif
%undef cmp

    ; remove entirely-zero iterations
    mov                 r5d, 14
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jge .end_zero_loop
    pxor                 m0, m0
.zero_loop:
    movzx               t1d, word [o2(tbl_Nx64_offset)+r5*2+0]
    movzx               t3d, word [o2(tbl_Nx64_offset)+r5*2+2]
    movzx               t0d, t1b
    movzx               t2d, t3b
    shr                 t1d, 8
    shr                 t3d, 8
    lea                  t4, [rsp+7*64*16]
.zero_loop_inner:
    mova [t4+(64+8*ARCH_X86_32+1*ARCH_X86_64)*16+t0*8], m0
    mova [t4+(64+8*ARCH_X86_32+1*ARCH_X86_64)*16+t1*8], m0
    mova [t4+(64+8*ARCH_X86_32+1*ARCH_X86_64)*16+t2*8], m0
    mova [t4+(64+8*ARCH_X86_32+1*ARCH_X86_64)*16+t3*8], m0
    sub                  t4, 64*16
    cmp                  t4, rsp
    jge .zero_loop_inner
%if ARCH_X86_32
    mov                  r6, [rsp+gprsize*4+(64*9+8)*16]
%endif
    sub                 r5d, 2
    cmp                eobw, word [o2(tbl_32x32_2d)+r5]
    jl .zero_loop
.end_zero_loop:
    mov [rsp+gprsize*0+(9*64+8*ARCH_X86_32+1*ARCH_X86_64)*16], eobd
%if ARCH_X86_32
    mov                  cq, [rsp+gprsize*3+(64*9+8)*16]
%endif
    ; actual first pass after skipping all-zero data
.loop_pass1:
%if ARCH_X86_64
    mova                m11, [o(pd_2048)]
    mova                m12, [o(clip_18b_min)]
    mova                m13, [o(clip_18b_max)]
    mova                m14, [o(pd_2896)]
%endif

    mov                  r3, rsp
    lea                  r4, [o(idct64_mul_16bpc)]
    mova                 m0, [cq+128* 1+r5*8]
    mova                 m1, [cq+128*31+r5*8]
    mova                 m2, [cq+128*17+r5*8]
    mova                 m3, [cq+128*15+r5*8]
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    mova                 m0, [cq+128* 7+r5*8]
    mova                 m1, [cq+128*25+r5*8]
    mova                 m2, [cq+128*23+r5*8]
    mova                 m3, [cq+128* 9+r5*8]
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    mova                 m0, [cq+128* 5+r5*8]
    mova                 m1, [cq+128*27+r5*8]
    mova                 m2, [cq+128*21+r5*8]
    mova                 m3, [cq+128*11+r5*8]
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    mova                 m0, [cq+128* 3+r5*8]
    mova                 m1, [cq+128*29+r5*8]
    mova                 m2, [cq+128*19+r5*8]
    mova                 m3, [cq+128*13+r5*8]
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part1
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_part2

    mova                 m0, [cq+128* 2+r5*8]
    mova                 m1, [cq+128*14+r5*8]
    mova                 m2, [cq+128*18+r5*8]
    mova                 m3, [cq+128*30+r5*8]
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part1_fast

    mova                 m0, [cq+128* 6+r5*8]
    mova                 m1, [cq+128*10+r5*8]
    mova                 m2, [cq+128*22+r5*8]
    mova                 m3, [cq+128*26+r5*8]
    call m(inv_txfm_add_dct_dct_32x8_16bpc).main_oddhalf_part2_fast
    add                  r3, 16*(24+4*ARCH_X86_32)

    mova                 m0, [cq+128* 4+r5*8]
    mova                 m1, [cq+128*12+r5*8]
    mova                 m2, [cq+128*20+r5*8]
    mova                 m3, [cq+128*28+r5*8]
    call m(idct_16x4_internal_16bpc).main_oddhalf_fast

    mova                 m0, [cq+128* 0+r5*8]
    mova                 m1, [cq+128* 8+r5*8]
    mova                 m2, [cq+128*16+r5*8]
    mova                 m3, [cq+128*24+r5*8]
    call m(idct_8x4_internal_16bpc).main_pass1_fast
    call m(idct_8x4_internal_16bpc).round
    mova [r3-(7+4*ARCH_X86_32)*16], m1
    mova [r3-(6+4*ARCH_X86_32)*16], m2
    mova [r3-(5+4*ARCH_X86_32)*16], m3
    mova [r3-(4+4*ARCH_X86_32)*16], m4
    mova [r3-(3+4*ARCH_X86_32)*16], m5
    mova [r3-(2+4*ARCH_X86_32)*16], m6
    mova [r3-(1+4*ARCH_X86_32)*16], m7
    sub                  r3, 16*(40+4*ARCH_X86_32-4)

%if ARCH_X86_64
    psrld               m15, m11, 10 ; pd_2
%else
    mova                 m7, [o(pd_2)]
%endif
    call m(inv_txfm_add_dct_dct_64x16_16bpc).main_end_loop_start

    lea                  r3, [rsp+56*16]
    movzx               t1d, word [o2(tbl_Nx64_offset)+r5*2+0]
    movzx               t3d, word [o2(tbl_Nx64_offset)+r5*2+2]
    movzx               t0d, t1b
    movzx               t2d, t3b
    shr                 t1d, 8
    shr                 t3d, 8
    lea                  t4, [rsp+7*64*16+(64+8*ARCH_X86_32+1*ARCH_X86_64)*16]
    call .shift_transpose
    ; zero cq
    pxor                 m7, m7
%if ARCH_X86_32
    mov                  cq, [rsp+gprsize*3+(64*9+8)*16]
%endif
    lea                  r4, [cq+30*128+r5*8]
.zero_cq_loop:
    REPX {mova [r4+x*128], m7}, -2, -1, 0, 1
    sub                  r4, 4*128
    cmp                  r4, cq
    jg .zero_cq_loop
%if ARCH_X86_32
    mov                  r6, [rsp+gprsize*4+(64*9+8)*16]
%endif
    sub                 r5d, 2
    jge .loop_pass1

    ; pass=2 code starts here
    mov                eobd, [rsp+gprsize*0+(9*64+8*ARCH_X86_32+1*ARCH_X86_64)*16]
%if ARCH_X86_32
    mov             strideq, [rsp+gprsize*2+(9*64+8)*16]
%else
    mov                  r0, [rsp+gprsize*0+64*16]
%endif
    add                 rsp, (64+8*ARCH_X86_32+1*ARCH_X86_64-3)*16
    cmp                eobd, 151
    jl .fast
    ; fall-through
%if ARCH_X86_64
    DECLARE_REG_TMP 8, 9
%else
    DECLARE_REG_TMP 1, 5
%endif
    lea                  t0, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_fast)]
    lea                  t1, [o(m_suffix(idct_16x64_internal_8bpc, _ssse3).main)]
    jmp .run
.fast:
    lea                  t0, [o(m_suffix(idct_8x32_internal_8bpc, _ssse3).main_veryfast)]
    lea                  t1, [o(m_suffix(idct_16x64_internal_8bpc, _ssse3).main_fast)]
.run:

%if ARCH_X86_64
    lea                  r2, [dstq+128]
    mov                  r7, -16
%else
    lea                  r2, [rsp+(64*8+3)*16]
    mov      [r2+4*gprsize], t0
    mov      [r2+5*gprsize], t1
    mov                  r1, [r2+2*gprsize]
    mov dword [r2+0*gprsize], 8
%endif
    jmp m(inv_txfm_add_dct_dct_16x64_16bpc).loop_pass2

    ; copy of pass=1 tmp-regs
%if ARCH_X86_32
    DECLARE_REG_TMP 4, 1, 2, 0, 6
%else
    DECLARE_REG_TMP 8, 9, 4, 7, 0
%endif

.shift_transpose:
    mova                 m0, [r3+0*16]
    mova                 m1, [r3+1*16]
    mova                 m2, [r3+2*16]
    mova                 m3, [r3+3*16]
    mova                 m4, [r3+4*16]
    mova                 m5, [r3+5*16]
    mova                 m6, [r3+6*16]
    mova                 m7, [r3+7*16]
    REPX       {psrad x, 2}, m0, m1, m2, m3, m4, m5, m6, m7
    packssdw             m0, m1
    packssdw             m2, m3
    packssdw             m4, m5
    packssdw             m6, m7
    call m(idct_8x4_internal_16bpc).transpose4x8packed
    mova          [t4+t0*8], m0
    mova          [t4+t1*8], m1
    mova          [t4+t2*8], m2
    mova          [t4+t3*8], m3
    sub                  t4, 16*64
    sub                  r3, 8*16
    cmp                  r3, rsp
    jg .shift_transpose
    ret

.dconly:
    imul                r5d, [cq], 181
    mov                [cq], eobd ; 0
    mov                 r3d, 64
    add                 rsp, (64+8*ARCH_X86_32+8*64+1*ARCH_X86_64)*16 + \
                             (4+4*ARCH_X86_32)*gprsize - (64+8*ARCH_X86_32)*16
    jmp m(inv_txfm_add_dct_dct_64x16_16bpc).dconly1
