; Copyright © 2020-2023, VideoLAN and dav1d authors
; Copyright © 2020-2023, Two Orioles, LLC
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
const \
dup16_perm,  db  0,  1,  0,  1,  2,  3,  2,  3,  4,  5,  4,  5,  6,  7,  6,  7
             db  8,  9,  8,  9, 10, 11, 10, 11, 12, 13, 12, 13, 14, 15, 14, 15
             db 16, 17, 16, 17, 18, 19, 18, 19, 20, 21, 20, 21, 22, 23, 22, 23
             db 24, 25, 24, 25, 26, 27, 26, 27, 28, 29, 28, 29, 30, 31, 30, 31
const \
int8_permA,  db  0,  1, 16, 17, 32, 33, 48, 49,  2,  3, 18, 19, 34, 35, 50, 51
             db  4,  5, 20, 21, 36, 37, 52, 53,  6,  7, 22, 23, 38, 39, 54, 55
             db  8,  9, 24, 25, 40, 41, 56, 57, 10, 11, 26, 27, 42, 43, 58, 59
             db 12, 13, 28, 29, 44, 45, 60, 61, 14, 15, 30, 31, 46, 47, 62, 63
int8_permB:  db  0,  1, 16, 17, 32, 33, 48, 49,  2,  3, 18, 19, 34, 35, 50, 51
             db  8,  9, 24, 25, 40, 41, 56, 57, 10, 11, 26, 27, 42, 43, 58, 59
             db  4,  5, 20, 21, 36, 37, 52, 53,  6,  7, 22, 23, 38, 39, 54, 55
             db 12, 13, 28, 29, 44, 45, 60, 61, 14, 15, 30, 31, 46, 47, 62, 63
int16_perm:  db  0,  1, 32, 33,  2,  3, 34, 35,  4,  5, 36, 37,  6,  7, 38, 39
             db  8,  9, 40, 41, 10, 11, 42, 43, 12, 13, 44, 45, 14, 15, 46, 47
             db 16, 17, 48, 49, 18, 19, 50, 51, 20, 21, 52, 53, 22, 23, 54, 55
             db 24, 25, 56, 57, 26, 27, 58, 59, 28, 29, 60, 61, 30, 31, 62, 63
idtx_16x4p:  db  0,  1,  4,  5, 16, 17, 20, 21,  2,  3,  6,  7, 18, 19, 22, 23
             db 32, 33, 36, 37, 48, 49, 52, 53, 34, 35, 38, 39, 50, 51, 54, 55
             db  8,  9, 12, 13, 24, 25, 28, 29, 10, 11, 14, 15, 26, 27, 30, 31
             db 40, 41, 44, 45, 56, 57, 60, 61, 42, 43, 46, 47, 58, 59, 62, 63
idct_8x32p:  db 60, 61,  4,  5, 32, 33,  0,  1, 28, 29, 36, 37, 56, 57,  8,  9
             db 12, 13, 52, 53, 24, 25, 40, 41, 44, 45, 20, 21, 48, 49, 16, 17
             db 62, 63,  2,  3,  6,  7, 58, 59, 54, 55, 10, 11, 14, 15, 50, 51
             db 46, 47, 18, 19, 22, 23, 42, 43, 38, 39, 26, 27, 30, 31, 34, 35
idct_16x32p: db  6,  7, 58, 59, 38, 39, 26, 27, 32, 33,  0,  1, 30, 31, 34, 35
             db 46, 47, 18, 19, 22, 23, 42, 43, 24, 25, 40, 41, 44, 45, 20, 21
             db 62, 63,  2,  3, 48, 49, 16, 17, 56, 57,  8,  9, 14, 15, 50, 51
             db 54, 55, 10, 11, 60, 61,  4,  5, 12, 13, 52, 53, 28, 29, 36, 37
end_16x32p:  db  0, 32,  1, 48,  2, 36,  3, 52, 16, 40, 17, 56, 18, 44, 19, 60
             db  4, 33,  5, 49,  6, 37,  7, 53, 20, 41, 21, 57, 22, 45, 23, 61
             db  8, 35,  9, 51, 10, 39, 11, 55, 24, 43, 25, 59, 26, 47, 27, 63
             db 12, 34, 13, 50, 14, 38, 15, 54, 28, 42, 29, 58, 30, 46, 31, 62

; packed 4-bit qword shuffle indices
permA:       dq 0x1c0d0d1ce0d94040, 0x5849495868fb6262
             dq 0x3e2f2f3ef1c85151, 0x7a6b6b7a79ea7373
             dq 0x94858594a451c8d9, 0xd0c1c1d02c73eafb
             dq 0xb6a7a7b6b540d9c8, 0xf2e3e3f23d62fbea
permB:       dq 0x40acbd0fcadb0f40, 0x518e9f3ce8f99604
             dq 0xc824352d56128751, 0xd906171e74301e15
             dq 0x6271604b03472d62, 0x735342782165b426
             dq 0xeaf9e8699f8ea573, 0xfbdbca5abdac3c37
permC:       dq 0x9d409d041551c2e0, 0xbf62bf263773a486
             dq 0xc88c8c15409dd3f1, 0xeaaeae3762bfb597
             dq 0x04d9158c8cc84a68, 0x26fb37aeaeea2c0e
             dq 0x5115049dd9045b79, 0x733726bffb263d1f
permD:       dq 0x0cda098800041504, 0x0edb09b2028c3726
             dq 0x0f11fa9c01150415, 0x0988f326039d2637
             dq 0x05640f1108269d8c, 0x05290edb0aaebfae
             dq 0x0005000509378c9d, 0xffffffff0bbfaebf

pd_0to15:    dd  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
gather8a:    dd  0,  2,  1,  3,  8, 10,  9, 11
gather8b:    dd  0,  1,  4,  5,  8,  9, 12, 13
gather8c:    dd  0,  4,  2,  6, 12,  8, 14, 10
gather8d:    dd  0, 19,  1, 18,  2, 17,  3, 16

int_shuf1:   db  0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
int_shuf2:   db  8,  9,  0,  1, 10, 11,  2,  3, 12, 13,  4,  5, 14, 15,  6,  7
int_shuf3:   db  0,  1,  8,  9,  4,  5, 12, 13,  2,  3, 10, 11,  6,  7, 14, 15
int_shuf4:   db  8,  9,  0,  1, 12, 13,  4,  5, 10, 11,  2,  3, 14, 15,  6,  7
deint_shuf:  db  0,  1,  4,  5,  8,  9, 12, 13,  2,  3,  6,  7, 10, 11, 14, 15
int_mshift:  db 12, 20,  0,  0, 44, 52,  0,  0

pb_32:           times 4 db 32
pw_2048:         times 2 dw 2048
pw_4096:         times 2 dw 4096
pw_8192:         times 2 dw 8192
pw_16384:        times 2 dw 16384
pw_1697x16:      times 2 dw 1697*16
pw_1697x8:       times 2 dw 1697*8
pw_2896x8:       times 2 dw 2896*8
pd_2048:         dd  2048

%define pw_5          (permD+52)
%define pd_m1         (permD+60)
%define pw_3803_1321  (permD+44)
%define pw_2482_3803  (permD+12)
%define pw_2440_3290  (permD+ 4)
%define pw_m3290_2440 (permD+28)
%define pw_3857_1380  (permD+36)
%define pw_m1380_3857 (permD+20)

pw_8192_m8192:   dw   8192,  -8192
pw_m8192_8192:   dw  -8192,   8192
pw_16384_m16384: dw  16384, -16384
pw_m16384_16384: dw -16384,  16384

pw_m1321_2482:   dw  -1321,  2482
pw_m3344_3344:   dw  -3344,  3344
pw_2482_3344:    dw   2482,  3344
pw_m3803_3344:   dw  -3803,  3344
pd_3344:         dd   3344
pw_m1321_m3344:  dw  -1321, -3344
pw_2896_m2896:   dw   2896, -2896

pw_1567_m3784:   dw   1567, -3784
pw_3784_m1567:   dw   3784, -1567
pw_4017_m799:    dw   4017,  -799
pw_2276_m3406:   dw   2276, -3406
pw_m799_m4017:   dw   -799, -4017
pw_m3406_m2276:  dw  -3406, -2276

%macro COEF_PAIR 2-3 0
pw_%1_%2:   dw  %1,  %2
pw_m%2_%1:  dw -%2,  %1
%if %3
pw_m%1_m%2: dw -%1, -%2
%endif
%endmacro

COEF_PAIR 2896, 2896
COEF_PAIR 1567, 3784, 1
COEF_PAIR 3784, 1567
COEF_PAIR  201, 4091
COEF_PAIR  995, 3973
COEF_PAIR 1751, 3703
COEF_PAIR 3035, 2751
COEF_PAIR 3513, 2106
COEF_PAIR 4052,  601
COEF_PAIR 3166, 2598, 1
COEF_PAIR 3920, 1189, 1
COEF_PAIR 2276, 3406
COEF_PAIR 4017,  799

%macro COEF_X8 1-*
%rep %0
    dw %1*8, %1*8
    %rotate 1
%endrep
%endmacro

pw_m2276x8: COEF_X8 -2276
pw_3406x8:  COEF_X8  3406
pw_4017x8:  COEF_X8  4017
pw_799x8:   COEF_X8   799
pw_3784x8:  COEF_X8  3784
pw_1567x8:  COEF_X8  1567

pw_4076x8:  COEF_X8  4076
pw_401x8:   COEF_X8   401
pw_m2598x8: COEF_X8 -2598
pw_3166x8:  COEF_X8  3166
pw_3612x8:  COEF_X8  3612
pw_1931x8:  COEF_X8  1931
pw_m1189x8: COEF_X8 -1189
pw_3920x8:  COEF_X8  3920

pw_4091x8:  COEF_X8  4091
pw_201x8:   COEF_X8   201
pw_m2751x8: COEF_X8 -2751
pw_3035x8:  COEF_X8  3035
pw_3703x8:  COEF_X8  3703
pw_1751x8:  COEF_X8  1751
pw_m1380x8: COEF_X8 -1380
pw_3857x8:  COEF_X8  3857
pw_3973x8:  COEF_X8  3973
pw_995x8:   COEF_X8   995
pw_m2106x8: COEF_X8 -2106
pw_3513x8:  COEF_X8  3513
pw_3290x8:  COEF_X8  3290
pw_2440x8:  COEF_X8  2440
pw_m601x8:  COEF_X8  -601
pw_4052x8:  COEF_X8  4052

pw_401_4076x8:   dw   401*8, 4076*8
pw_m2598_3166x8: dw -2598*8, 3166*8
pw_1931_3612x8:  dw  1931*8, 3612*8
pw_m1189_3920x8: dw -1189*8, 3920*8
pw_799_4017x8:   dw   799*8, 4017*8
pw_m2276_3406x8: dw -2276*8, 3406*8

pw_201_4091x8:   dw   201*8, 4091*8
pw_m601_4052x8:  dw  -601*8, 4052*8
pw_995_3973x8:   dw   995*8, 3973*8
pw_m1380_3857x8: dw -1380*8, 3857*8
pw_1751_3703x8:  dw  1751*8, 3703*8
pw_m2106_3513x8: dw -2106*8, 3513*8
pw_2440_3290x8:  dw  2440*8, 3290*8
pw_m2751_3035x8: dw -2751*8, 3035*8

pw_101_4095x8:   dw   101*8, 4095*8
pw_m2824_2967x8: dw -2824*8, 2967*8
pw_1660_3745x8:  dw  1660*8, 3745*8
pw_m1474_3822x8: dw -1474*8, 3822*8
pw_897_3996x8:   dw   897*8, 3996*8
pw_m2191_3461x8: dw -2191*8, 3461*8
pw_2359_3349x8:  dw  2359*8, 3349*8
pw_m700_4036x8:  dw  -700*8, 4036*8
pw_501_4065x8:   dw   501*8, 4065*8
pw_m2520_3229x8: dw -2520*8, 3229*8
pw_2019_3564x8:  dw  2019*8, 3564*8
pw_m1092_3948x8: dw -1092*8, 3948*8
pw_1285_3889x8:  dw  1285*8, 3889*8
pw_m1842_3659x8: dw -1842*8, 3659*8
pw_2675_3102x8:  dw  2675*8, 3102*8
pw_m301_4085x8:  dw  -301*8, 4085*8

idct64_mul: COEF_X8  4095,   101,  2967, -2824,  3745,  1660,  3822, -1474
COEF_PAIR  401, 4076, 1
COEF_PAIR  799, 4017
            COEF_X8  -700,  4036,  2359,  3349, -2191,  3461,   897,  3996
dw    -2598, -3166,  3166, -2598,  2598,  3166, -4017,  -799,   799, -4017
            COEF_X8  4065,   501,  3229, -2520,  3564,  2019,  3948, -1092
COEF_PAIR 1931, 3612, 1
COEF_PAIR 3406, 2276
            COEF_X8  -301,  4085,  2675,  3102, -1842,  3659,  1285,  3889
dw    -1189, -3920,  3920, -1189,  1189,  3920, -2276, -3406,  3406, -2276

SECTION .text

%define o_base int8_permA+64*18
%define o(x) (r5 - (o_base) + (x))
%define m(x) mangle(private_prefix %+ _ %+ x %+ SUFFIX)

; flags: 1 = swap, 2 = interleave (l), 4 = interleave (t), 8 = no_pack,
;        16 = special_mul1, 32 = special_mul2
%macro ITX_MUL2X_PACK 6-7 0 ; dst/src, tmp[1-2], rnd, coef[1-2], flags
    mova                m%2, m%4
%if %7 & 16
    vpdpwssd            m%2, m%1, [o(pw_%5)] {bcstd}
    mova                m%3, m%4
%if %7 & 32
    vpdpwssd            m%3, m%1, [o(pw_%6)] {bcstd}
%else
    vpdpwssd            m%3, m%1, m%6
%endif
%elif %7 & 32
    vpdpwssd            m%2, m%1, m%5
    mova                m%3, m%4
    vpdpwssd            m%3, m%1, [o(pw_%6)] {bcstd}
%elif %6 < 32
    vpdpwssd            m%2, m%1, m%5
    mova                m%3, m%4
    vpdpwssd            m%3, m%1, m%6
%elif %7 & 1
    vpdpwssd            m%2, m%1, [o(pw_%5_%6)] {bcstd}
    mova                m%3, m%4
    vpdpwssd            m%3, m%1, [o(pw_m%6_%5)] {bcstd}
%else
    vpdpwssd            m%2, m%1, [o(pw_m%6_%5)] {bcstd}
    mova                m%3, m%4
    vpdpwssd            m%3, m%1, [o(pw_%5_%6)] {bcstd}
%endif
%if %7 & 2
    psrld               m%2, 12
    pslld               m%3, 4
    vpshrdd             m%1, m%3, m%2, 16
%elif %7 & 4
    ; compared to using shifts (as above) this has better throughput,
    ; but worse latency and requires setting up the opmask/index
    ; registers, so only use this method for the larger transforms
    pslld               m%1, m%2, 4
    vpmultishiftqb  m%1{k7}, m13, m%3
%else
    psrad               m%2, 12
    psrad               m%3, 12
%if %7 & 8 == 0
    packssdw            m%1, m%3, m%2
%endif
%endif
%endmacro

; flags: same as ITX_MUL2X_PACK
%macro ITX_MUL4X_PACK 10-11 0 ; dst/src, tmp[1-2], coef_tmp[1-2], rnd, coef[1-4], flags
%if %11 & 1
    vpbroadcastd        m%4, [o(pw_%9_%10)]
    vpbroadcastd    m%4{k1}, [o(pw_%7_%8)]
    vpbroadcastd        m%5, [o(pw_m%10_%9)]
    vpbroadcastd    m%5{k1}, [o(pw_m%8_%7)]
%else
    vpbroadcastd        m%4, [o(pw_m%10_%9)]
    vpbroadcastd    m%4{k1}, [o(pw_m%8_%7)]
    vpbroadcastd        m%5, [o(pw_%9_%10)]
    vpbroadcastd    m%5{k1}, [o(pw_%7_%8)]
%endif
    ITX_MUL2X_PACK       %1, %2, %3, %6, %4, %5, %11
%endmacro

; dst1 = (src1 * coef1 - src2 * coef2 + rnd) >> 12
; dst2 = (src1 * coef2 + src2 * coef1 + rnd) >> 12
%macro ITX_MULSUB_2W 7-8 ; dst/src[1-2], tmp[1-2], rnd, coef[1-2], dst2
    punpcklwd           m%3, m%2, m%1
    punpckhwd           m%2, m%1
%if %7 < 32
    mova                m%1, m%5
    vpdpwssd            m%1, m%3, m%7
    mova                m%4, m%5
    vpdpwssd            m%4, m%2, m%7
%else
    mova                m%1, m%5
    vpdpwssd            m%1, m%3, [o(pw_m%7_%6)] {bcstd}
    mova                m%4, m%5
    vpdpwssd            m%4, m%2, [o(pw_m%7_%6)] {bcstd}
%endif
    psrad               m%1, 12
    psrad               m%4, 12
    packssdw            m%1, m%4
    mova                m%4, m%5
%if %7 < 32
    vpdpwssd            m%4, m%2, m%6
    mova                m%2, m%5
    vpdpwssd            m%2, m%3, m%6
%else
    vpdpwssd            m%4, m%2, [o(pw_%6_%7)] {bcstd}
    mova                m%2, m%5
    vpdpwssd            m%2, m%3, [o(pw_%6_%7)] {bcstd}
%endif
    psrad               m%4, 12
    psrad               m%2, 12
%if %0 == 8
    packssdw            m%8, m%2, m%4
%else
    packssdw            m%2, m%4
%endif
%endmacro

%macro WRAP_XMM 1+
    %xdefine %%reset RESET_MM_PERMUTATION
    INIT_XMM cpuname
    DEFINE_MMREGS xmm
    AVX512_MM_PERMUTATION
    %1
    %%reset
%endmacro

%macro WRAP_YMM 1+
    INIT_YMM cpuname
    %1
    INIT_ZMM cpuname
%endmacro

%macro ITX4_END 4-5 2048 ; row[1-4], rnd
%if %5
    vpbroadcastd         m2, [o(pw_%5)]
    pmulhrsw             m0, m2
    pmulhrsw             m1, m2
%endif
    lea                  r2, [dstq+strideq*2]
%assign %%i 1
%rep 4
    %if %1 & 2
        CAT_XDEFINE %%row_adr, %%i, r2   + strideq*(%1&1)
    %else
        CAT_XDEFINE %%row_adr, %%i, dstq + strideq*(%1&1)
    %endif
    %assign %%i %%i + 1
    %rotate 1
%endrep
    movd                 m2, [%%row_adr1]
    pinsrd               m2, [%%row_adr2], 1
    movd                 m3, [%%row_adr3]
    pinsrd               m3, [%%row_adr4], 1
    pmovzxbw             m2, m2
    pmovzxbw             m3, m3
    paddw                m0, m2
    paddw                m1, m3
    packuswb             m0, m1
    movd       [%%row_adr1], m0
    pextrd     [%%row_adr2], m0, 1
    pextrd     [%%row_adr3], m0, 2
    pextrd     [%%row_adr4], m0, 3
    ret
%endmacro

%macro INV_TXFM_FN 3 ; type1, type2, size
cglobal inv_txfm_add_%1_%2_%3_8bpc, 4, 6, 0, dst, stride, c, eob, tx2, base
    %define %%p1 m(i%1_%3_internal_8bpc)
    lea               baseq, [o_base]
    ; Jump to the 1st txfm function if we're not taking the fast path, which
    ; in turn performs an indirect jump to the 2nd txfm function.
    lea tx2q, [m(i%2_%3_internal_8bpc).pass2]
%ifidn %1_%2, dct_dct
    test               eobd, eobd
    jnz %%p1
%else
    ; jump to the 1st txfm function unless it's located directly after this
    times ((%%end - %%p1) >> 31) & 1 jmp %%p1
ALIGN function_align
%%end:
%endif
%endmacro

%macro INV_TXFM_4X4_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 4x4
%ifidn %1_%2, dct_dct
    vpbroadcastw         m0, [cq]
    vpbroadcastd         m1, [o(pw_2896x8)]
    pmulhrsw             m0, m1
    mov                [cq], eobd
    pmulhrsw             m0, m1
    mova                 m1, m0
    jmp m(iadst_4x4_internal_8bpc).end2
%endif
%endmacro

%macro IDCT4_1D_PACKED 0
    vpbroadcastd         m4, [o(pd_2048)]
    punpckhwd            m2, m1, m0
    punpcklwd            m1, m0
    ITX_MUL2X_PACK        2, 0, 3, 4, 1567, 3784
    ITX_MUL2X_PACK        1, 0, 3, 4, 2896, 2896
    paddsw               m0, m1, m2 ; out0 out1
    psubsw               m1, m2     ; out3 out2
%endmacro

%macro IADST4_1D_PACKED 0
    punpcklwd            m4, m1, m0 ; in2 in0
    punpckhwd            m5, m1, m0 ; in3 in1
.main2:
    vpbroadcastd         m3, [o(pd_2048)]
    mova                 m0, m3
    vpdpwssd             m0, m4, [o(pw_3803_1321)] {bcstd}
    mova                 m2, m3
    vpdpwssd             m2, m4, [o(pw_m1321_2482)] {bcstd}
    mova                 m1, m3
    vpdpwssd             m1, m4, [o(pw_m3344_3344)] {bcstd}
    vpdpwssd             m3, m4, [o(pw_2482_3803)] {bcstd}
    vpdpwssd             m0, m5, [o(pw_2482_3344)] {bcstd}
    vpdpwssd             m2, m5, [o(pw_m3803_3344)] {bcstd}
    vpdpwssd             m1, m5, [o(pd_3344)] {bcstd}
    vpdpwssd             m3, m5, [o(pw_m1321_m3344)] {bcstd}
    REPX      {psrad x, 12}, m0, m2, m1, m3
    packssdw             m0, m2 ; out0 out1
    packssdw             m1, m3 ; out2 out3
%endmacro

INIT_XMM avx512icl
INV_TXFM_4X4_FN dct, dct
INV_TXFM_4X4_FN dct, adst
INV_TXFM_4X4_FN dct, flipadst
INV_TXFM_4X4_FN dct, identity

cglobal idct_4x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+16*0]
    mova                 m1, [cq+16*1]
    IDCT4_1D_PACKED
    mova                 m2, [o(deint_shuf)]
    shufps               m3, m0, m1, q1331
    shufps               m0, m0, m1, q0220
    pshufb               m0, m2
    pshufb               m1, m3, m2
    jmp                tx2q
.pass2:
    IDCT4_1D_PACKED
    pxor              ymm16, ymm16
    mova               [cq], ymm16
    ITX4_END              0, 1, 3, 2

INV_TXFM_4X4_FN adst, dct
INV_TXFM_4X4_FN adst, adst
INV_TXFM_4X4_FN adst, flipadst
INV_TXFM_4X4_FN adst, identity

cglobal iadst_4x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+16*0]
    mova                 m1, [cq+16*1]
    call .main
    punpckhwd            m3, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m3
    punpcklwd            m0, m3
    jmp                tx2q
.pass2:
    call .main
.end:
    pxor              ymm16, ymm16
    mova               [cq], ymm16
.end2:
    ITX4_END              0, 1, 2, 3
ALIGN function_align
.main:
    IADST4_1D_PACKED
    ret

INV_TXFM_4X4_FN flipadst, dct
INV_TXFM_4X4_FN flipadst, adst
INV_TXFM_4X4_FN flipadst, flipadst
INV_TXFM_4X4_FN flipadst, identity

cglobal iflipadst_4x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+16*0]
    mova                 m1, [cq+16*1]
    call m(iadst_4x4_internal_8bpc).main
    punpcklwd            m2, m1, m0
    punpckhwd            m1, m0
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    jmp                tx2q
.pass2:
    call m(iadst_4x4_internal_8bpc).main
.end:
    pxor              ymm16, ymm16
    mova               [cq], ymm16
.end2:
    ITX4_END              3, 2, 1, 0

INV_TXFM_4X4_FN identity, dct
INV_TXFM_4X4_FN identity, adst
INV_TXFM_4X4_FN identity, flipadst
INV_TXFM_4X4_FN identity, identity

cglobal iidentity_4x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+16*0]
    mova                 m1, [cq+16*1]
    vpbroadcastd         m3, [o(pw_1697x8)]
    pmulhrsw             m2, m3, m0
    pmulhrsw             m3, m1
    paddsw               m0, m2
    paddsw               m1, m3
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    jmp                tx2q
.pass2:
    vpbroadcastd         m3, [o(pw_1697x8)]
    pmulhrsw             m2, m3, m0
    pmulhrsw             m3, m1
    paddsw               m0, m2
    paddsw               m1, m3
    jmp m(iadst_4x4_internal_8bpc).end

%macro INV_TXFM_4X8_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 4x8
%ifidn %1_%2, dct_dct
    movd               xmm1, [o(pw_2896x8)]
    pmulhrsw           xmm0, xmm1, [cq]
    movd               xmm2, [o(pw_2048)]
    pmulhrsw           xmm0, xmm1
    pmulhrsw           xmm0, xmm1
    pmulhrsw           xmm0, xmm2
    vpbroadcastw        ym0, xmm0
    mova                ym1, ym0
    jmp m(iadst_4x8_internal_8bpc).end3
%endif
%endmacro

%macro IDCT8_1D_PACKED 0
    punpckhwd            m5, m3, m0 ; in7 in1
    punpckhwd            m4, m1, m2 ; in3 in5
    punpcklwd            m3, m1     ; in6 in2
    punpcklwd            m2, m0     ; in4 in0
.main2:
    vpbroadcastd         m6, [o(pd_2048)]
    ITX_MUL2X_PACK        5, 0, 1, 6,  799, 4017, 3 ; t4a t7a
    ITX_MUL2X_PACK        4, 0, 1, 6, 3406, 2276, 3 ; t5a t6a
    ITX_MUL2X_PACK        3, 0, 1, 6, 1567, 3784    ; t3 t2
    psubsw               m0, m5, m4 ; t5a t6a (interleaved)
    paddsw               m4, m5     ; t4  t7  (interleaved)
    ITX_MUL2X_PACK        2, 1, 5, 6, 2896, 2896    ; t0 t1
    ITX_MUL2X_PACK        0, 1, 5, 6, 2896, 2896, 1 ; t6 t5
%if mmsize > 16
    vbroadcasti32x4      m1, [o(deint_shuf)]
    pshufb               m4, m1
%else
    pshufb               m4, [o(deint_shuf)]
%endif
    psubsw               m1, m2, m3 ; tmp3 tmp2
    paddsw               m3, m2     ; tmp0 tmp1
    punpckhqdq           m2, m4, m0 ; t7 t6
    punpcklqdq           m4, m0     ; t4 t5
    paddsw               m0, m3, m2 ; out0 out1
    psubsw               m3, m2     ; out7 out6
    psubsw               m2, m1, m4 ; out4 out5
    paddsw               m1, m4     ; out3 out2
%endmacro

%macro IADST8_1D_PACKED 1 ; pass
    vpbroadcastd         m6, [o(pd_2048)]
%if %1 == 1
    ITX_MUL2X_PACK        0, 4, 5, 6,  401, 4076, 3 ; t1a t0a
    ITX_MUL2X_PACK        1, 4, 5, 6, 1931, 3612, 2 ; t2a t3a
    ITX_MUL2X_PACK        2, 4, 5, 6, 3166, 2598, 3 ; t5a t4a
    ITX_MUL2X_PACK        3, 4, 5, 6, 3920, 1189, 2 ; t6a t7a
    psubsw               m4, m0, m2 ; t5 t4
    paddsw               m0, m2     ; t1 t0
    psubsw               m5, m1, m3 ; t6 t7
    paddsw               m1, m3     ; t2 t3
    ITX_MUL2X_PACK        4, 2, 3, 6, 1567, 3784, 3 ; t5a t4a
    ITX_MUL2X_PACK        5, 2, 3, 6, 3784, 1567, 2 ; t7a t6a
%if mmsize > 16
    vbroadcasti32x4      m2, [o(deint_shuf)]
%else
    mova                 m2, [o(deint_shuf)]
%endif
    vprord               m1, 16
    psubsw               m3, m0, m1 ; t3 t2
    paddsw               m0, m1     ; -out7  out0
    psubsw               m1, m4, m5 ; t7 t6
    paddsw               m4, m5     ;  out6 -out1
    pshufb               m0, m2
    pshufb               m4, m2
    mova                 m2, m6
    vpdpwssd             m2, m3, [o(pw_m2896_2896)] {bcstd}
    mova                 m5, m6
    vpdpwssd             m5, m1, [o(pw_m2896_2896)] {bcstd}
    psrad                m2, 12
    psrad                m5, 12
    packssdw             m2, m5     ; out4 -out5
    mova                 m5, m6
    vpdpwssd             m5, m3, [o(pw_2896_2896)] {bcstd}
    mova                 m3, m6
    vpdpwssd             m3, m1, [o(pw_2896_2896)] {bcstd}
    psrad                m5, 12
    psrad                m3, 12
    packssdw             m1, m3, m5 ; out2 -out3
%else
    punpckhwd            m0, m4, m3 ; 0 7
    punpckhwd            m1, m5, m2 ; 2 5
    punpcklwd            m2, m5     ; 4 3
    punpcklwd            m3, m4     ; 6 1
    ITX_MUL2X_PACK        0, 4, 5, 6,  401, 4076 ; t0a t1a
    ITX_MUL2X_PACK        1, 4, 5, 6, 1931, 3612 ; t2a t3a
    ITX_MUL2X_PACK        2, 4, 5, 6, 3166, 2598 ; t4a t5a
    ITX_MUL2X_PACK        3, 4, 5, 6, 3920, 1189 ; t6a t7a
    psubsw               m4, m0, m2 ; t4 t5
    paddsw               m0, m2     ; t0 t1
    psubsw               m5, m1, m3 ; t6 t7
    paddsw               m1, m3     ; t2 t3
    shufps               m2, m5, m4, q1032
    punpckhwd            m4, m2
    punpcklwd            m5, m2
    ITX_MUL2X_PACK        4, 2, 3, 6, 1567, 3784    ; t4a t5a
    ITX_MUL2X_PACK        5, 2, 3, 6, 3784, 1567, 1 ; t6a t7a
    psubsw               m2, m0, m1 ; t2 t3
    paddsw               m0, m1     ; out0 -out7
    psubsw               m1, m4, m5 ; t6 t7
    paddsw               m4, m5     ; -out1 out6
    vpbroadcastd         m5, [o(pw_2896x8)]
    punpckhqdq           m3, m2, m1 ; t3 t7
    punpcklqdq           m2, m1     ; t2 t6
    paddsw               m1, m2, m3 ; t2+t3 t6+t7
    psubsw               m2, m3     ; t2-t3 t6-t7
    punpckhqdq           m3, m4, m0 ; out6 -out7
    punpcklqdq           m0, m4     ; out0 -out1
    pmulhrsw             m2, m5     ; out4 -out5
    pshufd               m1, m1, q1032
    pmulhrsw             m1, m5     ; out2 -out3
%endif
%endmacro

INIT_YMM avx512icl
INV_TXFM_4X8_FN dct, dct
INV_TXFM_4X8_FN dct, identity
INV_TXFM_4X8_FN dct, adst
INV_TXFM_4X8_FN dct, flipadst

cglobal idct_4x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpermq               m0, [cq+32*0], q3120
    vpermq               m1, [cq+32*1], q3120
    vpbroadcastd         m2, [o(pw_2896x8)]
    pmulhrsw             m0, m2
    pmulhrsw             m1, m2
    IDCT4_1D_PACKED
    vbroadcasti32x4      m2, [o(deint_shuf)]
    shufps               m3, m0, m1, q1331
    shufps               m0, m0, m1, q0220
    pshufb               m0, m2
    pshufb               m1, m3, m2
    jmp                tx2q
.pass2:
    vextracti32x4       xm2, m0, 1
    vextracti32x4       xm3, m1, 1
    call .main
    vpbroadcastd         m4, [o(pw_2048)]
    vinserti32x4         m0, m0, xm2, 1
    vinserti32x4         m1, m1, xm3, 1
    pshufd               m1, m1, q1032
    jmp m(iadst_4x8_internal_8bpc).end2
ALIGN function_align
.main:
    WRAP_XMM IDCT8_1D_PACKED
    ret

INV_TXFM_4X8_FN adst, dct
INV_TXFM_4X8_FN adst, adst
INV_TXFM_4X8_FN adst, flipadst
INV_TXFM_4X8_FN adst, identity

cglobal iadst_4x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpermq               m0, [cq+32*0], q3120
    vpermq               m1, [cq+32*1], q3120
    vpbroadcastd         m2, [o(pw_2896x8)]
    pmulhrsw             m0, m2
    pmulhrsw             m1, m2
    call m(iadst_8x4_internal_8bpc).main
    punpckhwd            m3, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m3
    punpcklwd            m0, m3
    jmp                tx2q
.pass2:
    vextracti32x4       xm2, m0, 1
    vextracti32x4       xm3, m1, 1
    pshufd              xm4, xm0, q1032
    pshufd              xm5, xm1, q1032
    call .main_pass2
    vpbroadcastd         m4, [o(pw_2048)]
    vinserti32x4         m0, xm2, 1
    vinserti32x4         m1, xm3, 1
    pxor                 m5, m5
    psubw                m5, m4
.end:
    punpcklqdq           m4, m5
.end2:
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
.end3:
    vpbroadcastd         m3, strided
    pmulld               m5, m3, [o(pd_0to15)]
    kxnorb               k1, k1, k1
    kmovb                k2, k1
    vpgatherdd       m3{k1}, [dstq+m5]
    pxor                 m4, m4
    mova               [cq], zmm20
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    paddw                m0, m2
    paddw                m1, m3
    packuswb             m0, m1
    vpscatterdd [dstq+m5]{k2}, m0
    RET
ALIGN function_align
.main_pass1:
    punpckhwd           xm0, xm4, xm3 ; 0 7
    punpckhwd           xm1, xm5, xm2 ; 2 5
    punpcklwd           xm2, xm5      ; 4 3
    punpcklwd           xm3, xm4      ; 6 1
    WRAP_XMM IADST8_1D_PACKED 1
    punpcklqdq          xm3, xm4, xm0 ; out6 -out7
    punpckhqdq          xm0, xm4      ; out0 -out1
    ret
ALIGN function_align
.main_pass2:
    WRAP_XMM IADST8_1D_PACKED 2
    ret

INV_TXFM_4X8_FN flipadst, dct
INV_TXFM_4X8_FN flipadst, adst
INV_TXFM_4X8_FN flipadst, flipadst
INV_TXFM_4X8_FN flipadst, identity

cglobal iflipadst_4x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpermq               m0, [cq+32*0], q3120
    vpermq               m1, [cq+32*1], q3120
    vpbroadcastd         m2, [o(pw_2896x8)]
    pmulhrsw             m0, m2
    pmulhrsw             m1, m2
    call m(iadst_8x4_internal_8bpc).main
    punpcklwd            m3, m1, m0
    punpckhwd            m1, m0
    punpcklwd            m0, m1, m3
    punpckhwd            m1, m3
    jmp                tx2q
.pass2:
    vextracti32x4       xm2, m0, 1
    vextracti32x4       xm3, m1, 1
    pshufd              xm4, xm0, q1032
    pshufd              xm5, xm1, q1032
    call m(iadst_4x8_internal_8bpc).main_pass2
    vpbroadcastd         m5, [o(pw_2048)]
    vinserti32x4         m3, xm1, 1
    vinserti32x4         m2, xm0, 1
    pxor                 m4, m4
    psubw                m4, m5
    pshufd               m0, m3, q1032
    pshufd               m1, m2, q1032
    jmp m(iadst_4x8_internal_8bpc).end

INIT_ZMM avx512icl
INV_TXFM_4X8_FN identity, dct
INV_TXFM_4X8_FN identity, adst
INV_TXFM_4X8_FN identity, flipadst
INV_TXFM_4X8_FN identity, identity

cglobal iidentity_4x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpbroadcastd         m0, [o(pw_2896x8)]
    pmulhrsw             m0, [cq]
    mova                 m1, [o(int8_permB)]
    vpbroadcastd         m2, [o(pw_1697x8)]
    vpermb               m0, m1, m0
    pmulhrsw             m2, m0
    paddsw               m0, m2
    vextracti32x8       ym1, m0, 1
    jmp                tx2q
.pass2:
    vpbroadcastd        ym4, [o(pw_4096)]
    jmp m(iadst_4x8_internal_8bpc).end2

%macro INV_TXFM_4X16_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 4x16
%ifidn %1_%2, dct_dct
    movsx               r6d, word [cq]
    mov                [cq], eobd
    imul                r6d, 181
    add                 r6d, 128+256
    sar                 r6d, 8+1
    imul                r6d, 181
    add                 r6d, 128+2048
    sar                 r6d, 8+4
    vpbroadcastw         m0, r6d
    mova                 m1, m0
    jmp m(iadst_4x16_internal_8bpc).end3
%endif
%endmacro

%macro IDCT16_1D_PACKED 0
    punpckhwd            m8, m7, m0 ; dct16 in15 in1
    punpcklwd            m9, m4, m0 ; dct4  in2  in0
    punpckhwd            m0, m3, m4 ; dct16 in7  in9
    punpcklwd            m7, m1     ; dct8  in7  in1
    punpckhwd            m1, m6     ; dct16 in3  in13
    punpcklwd            m3, m5     ; dct8  in3  in5
    punpckhwd            m5, m2     ; dct16 in11 in5
    punpcklwd            m6, m2     ; dct4  in3  in1
cglobal_label .main2
    vpbroadcastd        m10, [o(pd_2048)]
.main3:
    vpbroadcastq        m13, [o(int_mshift)]
    vpcmpub              k7, m13, m10, 6 ; 0x33...
    ITX_MUL2X_PACK        8, 2, 4, 10,  401, 4076, 5 ; t8a  t15a
    ITX_MUL2X_PACK        0, 2, 4, 10, 3166, 2598, 5 ; t9a  t14a
    ITX_MUL2X_PACK        5, 2, 4, 10, 1931, 3612, 5 ; t10a t13a
    ITX_MUL2X_PACK        1, 2, 4, 10, 3920, 1189, 5 ; t11a t12a
    ITX_MUL2X_PACK        7, 2, 4, 10,  799, 4017, 5 ; t4a  t7a
    ITX_MUL2X_PACK        3, 2, 4, 10, 3406, 2276, 5 ; t5a  t6a
.main4:
    psubsw               m2, m8, m0 ; t9  t14
    paddsw               m8, m0     ; t8  t15
    psubsw               m4, m1, m5 ; t10 t13
    paddsw               m1, m5     ; t11 t12
    ITX_MUL2X_PACK        6, 0, 5, 10, 1567,  3784    ; t3   t2
    psubsw               m0, m8, m1 ; t11a t12a
    paddsw               m8, m1     ; t8a  t15a
    psubsw               m1, m7, m3 ; t5a  t6a
    paddsw               m7, m3     ; t4   t7
.main5:
    ITX_MUL2X_PACK        2, 3, 5, 10, 1567,  3784, 5 ; t9a  t14a
    ITX_MUL2X_PACK        4, 3, 5, 10, m3784, 1567, 5 ; t10a t13a
%if mmsize > 16
    vbroadcasti32x4      m5, [o(deint_shuf)]
%else
    mova                 m5, [o(deint_shuf)]
%endif
    vpbroadcastd        m11, [o(pw_m2896_2896)]
    vpbroadcastd        m12, [o(pw_2896_2896)]
    paddsw               m3, m2, m4 ; t9   t14
    psubsw               m2, m4     ; t10  t13
    pshufb               m8, m5
    pshufb               m7, m5
    pshufb               m3, m5
    ITX_MUL2X_PACK        9, 4,  5, 10, 11, 12    ; t0   t1
    ITX_MUL2X_PACK        1, 4,  5, 10, 12, 11    ; t5   t6
    ITX_MUL2X_PACK        0, 4,  5, 10, 11, 12, 8 ; t11  t12
    ITX_MUL2X_PACK        2, 0, 11, 10, 11, 12, 8 ; t10a t13a
    punpckhqdq           m2, m7, m1 ; t7 t6
    punpcklqdq           m7, m1     ; t4 t5
    psubsw               m1, m9, m6 ; dct4 out3 out2
    paddsw               m9, m6     ; dct4 out0 out1
    packssdw             m5, m11    ; t12  t13a
    packssdw             m4, m0     ; t11  t10a
    punpckhqdq           m0, m8, m3 ; t15a t14
    punpcklqdq           m8, m3     ; t8a  t9
    psubsw               m3, m9, m2 ; dct8 out7 out6
    paddsw               m9, m2     ; dct8 out0 out1
    psubsw               m2, m1, m7 ; dct8 out4 out5
    paddsw               m1, m7     ; dct8 out3 out2
    psubsw               m7, m9, m0 ; out15 out14
    paddsw               m0, m9     ; out0  out1
    psubsw               m6, m1, m5 ; out12 out13
    paddsw               m1, m5     ; out3  out2
    psubsw               m5, m2, m4 ; out11 out10
    paddsw               m2, m4     ; out4  out5
    psubsw               m4, m3, m8 ; out8  out9
    paddsw               m3, m8     ; out7  out6
%endmacro

INV_TXFM_4X16_FN dct, dct
INV_TXFM_4X16_FN dct, identity
INV_TXFM_4X16_FN dct, adst
INV_TXFM_4X16_FN dct, flipadst

cglobal idct_4x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                ym1, [cq+32*2]
    vinserti32x8         m1, [cq+32*0], 1
    mova                 m0, [o(int16_perm)]
    mova                ym2, [cq+32*3]
    vinserti32x8         m2, [cq+32*1], 1
    vpbroadcastd         m4, [o(pd_2048)]
    vpermb               m1, m0, m1 ; c0 a0 c1 a1 c2 a2 c3 a3
    vpermb               m2, m0, m2 ; d0 b0 d1 b1 d2 b2 d3 b3
    ITX_MUL2X_PACK        1, 0, 3, 4, 2896, 2896, 2
    ITX_MUL2X_PACK        2, 0, 3, 4, 1567, 3784, 2
    vpbroadcastd         m4, [o(pw_16384)]
    psubsw               m3, m1, m2
    paddsw               m1, m2     ; out0 out1
    vprord               m3, 16     ; out2 out3
    punpckldq            m0, m1, m3
    punpckhdq            m1, m3
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    jmp                tx2q
.pass2:
    vextracti32x4       xm2, ym0, 1
    vextracti32x4       xm3, ym1, 1
    vextracti32x4       xm4, m0, 2
    vextracti32x4       xm5, m1, 2
    vextracti32x4       xm6, m0, 3
    vextracti32x4       xm7, m1, 3
    call .main
    vinserti32x4        ym0, xm2, 1
    vinserti32x4        ym1, xm3, 1
    vinserti32x4        ym4, xm6, 1
    vinserti32x4        ym5, xm7, 1
    vinserti32x8         m0, ym4, 1
    vinserti32x8         m1, ym5, 1
    vpbroadcastd         m5, [o(pw_2048)]
    pshufd               m1, m1, q1032
    jmp m(iadst_4x16_internal_8bpc).end2
ALIGN function_align
.main:
    WRAP_XMM IDCT16_1D_PACKED
    ret

INV_TXFM_4X16_FN adst, dct
INV_TXFM_4X16_FN adst, adst
INV_TXFM_4X16_FN adst, flipadst
INV_TXFM_4X16_FN adst, identity

cglobal iadst_4x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m1, [o(permB)]
    vpermq               m0, m1, [cq+64*0]
    vpermq               m1, m1, [cq+64*1]
    call m(iadst_16x4_internal_8bpc).main
    vpbroadcastd         m3, [o(pw_16384)]
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    pmulhrsw             m2, m3
    pmulhrsw             m0, m3
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    jmp                tx2q
.pass2:
    call .main
    vpbroadcastd         m5, [o(pw_2048)]
    psrlq               m10, 4
    psubw                m6, m8, m5
.end:
    vpbroadcastd         m7, [o(pw_2896x8)]
    paddsw              ym1, ym2, ym4
    psubsw              ym2, ym4
    vinserti32x8         m1, ym2, 1
    pmulhrsw             m1, m7      ; -out7   out4   out6  -out5   out8  -out11 -out9   out10
    psrlq                m0, m10, 4
    vpermi2q             m0, m1, m3  ; 0 1   4 5   8 9   c d
    vpermt2q             m1, m10, m3 ; 2 3   6 7   a b   e f
    punpcklqdq           m5, m6
.end2:
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
.end3:
    vpbroadcastd         m3, strided
    pmulld               m5, m3, [o(pd_0to15)]
    kxnorw               k1, k1, k1
    kmovw                k2, k1
    vpgatherdd       m3{k1}, [dstq+m5]
    pxor                 m4, m4
    mova          [cq+64*0], m4
    mova          [cq+64*1], m4
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    paddw                m0, m2
    paddw                m1, m3
    packuswb             m0, m1
    vpscatterdd [dstq+m5]{k2}, m0
    RET
ALIGN function_align
.main:
    movu                 m3, [o(permB+1)]
    psrlq               m10, m3, 4
.main2:
    vpermi2q             m3, m0, m1  ; in15 in12 in13 in14 in11 in8  in9  in10
    vpermt2q             m0, m10, m1 ; in0  in3  in2  in1  in4  in7  in6  in5
    vpbroadcastd         m9, [o(pd_2048)]
    vpbroadcastq       ym13, [o(int_mshift)]
    kxnorb               k1, k1, k1
    punpckhwd            m4, m3, m0  ; in12 in3  in14 in1
    punpcklwd            m0, m3      ; in0  in15 in2  in13
    kshiftrb             k1, k1, 4
    vextracti32x8       ym3, m4, 1   ; in8  in7  in10 in5
    vextracti32x8       ym1, m0, 1   ; in4  in11 in6  in9
INIT_YMM avx512icl
    vpcmpub              k7, m13, m9, 6 ; 0x33...
    pxor                 m8, m8
    ITX_MUL4X_PACK        0, 2, 5, 6, 7, 9,  201, 4091,  995, 3973, 5
    ITX_MUL4X_PACK        1, 2, 5, 6, 7, 9, 1751, 3703, 2440, 3290, 5
    ITX_MUL4X_PACK        3, 2, 5, 6, 7, 9, 3035, 2751, 3513, 2106, 5
    ITX_MUL4X_PACK        4, 2, 5, 6, 7, 9, 3857, 1380, 4052,  601, 5
    psubsw               m2, m0, m3 ; t9a  t8a  t11a t10a
    paddsw               m0, m3     ; t1a  t0a  t3a  t2a
    psubsw               m3, m1, m4 ; t13a t12a t15a t14a
    paddsw               m4, m1     ; t5a  t4a  t7a  t6a
    ITX_MUL4X_PACK        2, 1, 5, 6, 7, 9,  799, 4017, 3406, 2276, 5
    psubw                m7, m8, m7
    ITX_MUL2X_PACK        3, 1, 5, 9, 7, 6, 4
    vpbroadcastd         m6, [o(pw_3784_m1567)]
    vpbroadcastd     m6{k1}, [o(pw_m3784_1567)]
    psubsw               m1, m0, m4 ; t5   t4   t7   t6
    paddsw               m0, m4     ; t1   t0   t3   t2
    psubsw               m4, m2, m3 ; t13a t12a t15a t14a
    paddsw               m2, m3     ; t9a  t8a  t11a t10a
    ITX_MUL2X_PACK        1, 3, 5, 9, 1567_3784, 6, 16 ; t4a t5a t7a t6a
    ITX_MUL2X_PACK        4, 3, 5, 9, 1567_3784, 6, 16 ; t12 t13 t15 t14
    vbroadcasti32x4      m5, [o(deint_shuf)]
    pshufb               m0, m5
    pshufb               m2, m5
    vshufi32x4           m3, m0, m2, 0x03  ; t3   t2   t11a t10a
    vinserti32x4         m0, xm2, 1        ; t1   t0   t9a  t8a
    vshufi32x4           m2, m1, m4, 0x03  ; t7a  t6a  t15  t14
    vinserti32x4         m1, xm4, 1        ; t4a  t5a  t12  t13
    pshufd               m2, m2, q1032     ; t6a  t7a  t14  t15
    psubsw               m4, m0, m3        ; t3a t2a t11 t10
    paddsw               m0, m3            ; -out15  out0   out14 -out1
    paddsw               m3, m1, m2        ;  out12 -out3  -out13  out2
    psubsw               m1, m2            ; t7 t6 t15a t14a
    punpckhqdq           m2, m4, m1        ; t2a t6  t10 t14a
    punpcklqdq           m4, m1            ; t3a t7  t11 t15a
INIT_ZMM avx512icl
    vinserti32x8         m3, ym0, 1        ; out12 -out3  -out13  out2  -out15  out0   out14 -out1
    ret

INV_TXFM_4X16_FN flipadst, dct
INV_TXFM_4X16_FN flipadst, adst
INV_TXFM_4X16_FN flipadst, flipadst
INV_TXFM_4X16_FN flipadst, identity

cglobal iflipadst_4x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m1, [o(permB)]
    vpermq               m0, m1, [cq+64*0]
    vpermq               m1, m1, [cq+64*1]
    call m(iadst_16x4_internal_8bpc).main
    vpbroadcastd         m3, [o(pw_16384)]
    punpcklwd            m2, m1, m0
    punpckhwd            m1, m0
    pmulhrsw             m2, m3
    pmulhrsw             m1, m3
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    jmp                tx2q
.pass2:
    call m(iadst_4x16_internal_8bpc).main
    vpbroadcastd         m6, [o(pw_2048)]
    psrlq               m10, 12
    psubw                m5, m8, m6
    jmp m(iadst_4x16_internal_8bpc).end

INV_TXFM_4X16_FN identity, dct
INV_TXFM_4X16_FN identity, adst
INV_TXFM_4X16_FN identity, flipadst
INV_TXFM_4X16_FN identity, identity

cglobal iidentity_4x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m2, [o(int16_perm)]
    vpermb               m1, m2, [cq+64*0]
    vpermb               m2, m2, [cq+64*1]
    vpbroadcastd         m4, [o(pw_1697x8)]
    vpbroadcastd         m0, [o(pd_m1)]
    pmulhrsw             m3, m4, m1    ; we want to do a signed avg, but pavgw is
    vpcmpw               k1, m1, m0, 4 ; unsigned. as long as both signs are equal
    pmulhrsw             m4, m2        ; it still works, but if the input is -1 the
    vpcmpw               k2, m2, m0, 4 ; pmulhrsw result will become 0 which causes
    vpavgw        m1{k1}{z}, m3        ; pavgw to output -32768 instead of 0 unless
    vpavgw        m2{k2}{z}, m4        ; we explicitly deal with that case here.
    punpckldq            m0, m1, m2
    punpckhdq            m1, m2
    jmp                tx2q
.pass2:
    vpbroadcastd         m3, [o(pw_1697x16)]
    vpbroadcastd         m5, [o(pw_2048)]
    pmulhrsw             m2, m3, m0
    pmulhrsw             m3, m1
    paddsw               m0, m0
    paddsw               m1, m1
    paddsw               m0, m2
    paddsw               m1, m3
    jmp m(iadst_4x16_internal_8bpc).end2

%macro WRITE_8X4 4-7 strideq*1, strideq*2, r6 ; coefs[1-2], tmp[1-2], off[1-3]
    movq               xm%3, [dstq   ]
    movhps             xm%3, [dstq+%5]
    movq               xm%4, [dstq+%6]
    movhps             xm%4, [dstq+%7]
    pmovzxbw            m%3, xm%3
    pmovzxbw            m%4, xm%4
%ifnum %1
    paddw               m%3, m%1
%else
    paddw               m%3, %1
%endif
%ifnum %2
    paddw               m%4, m%2
%else
    paddw               m%4, %2
%endif
    packuswb            m%3, m%4
    vextracti32x4      xm%4, m%3, 1
    movq          [dstq   ], xm%3
    movhps        [dstq+%6], xm%3
    movq          [dstq+%5], xm%4
    movhps        [dstq+%7], xm%4
%endmacro

%macro INV_TXFM_8X4_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 8x4
%ifidn %1_%2, dct_dct
    movd                xm1, [o(pw_2896x8)]
    pmulhrsw            xm0, xm1, [cq]
    movd                xm2, [o(pw_2048)]
    pmulhrsw            xm0, xm1
    pmulhrsw            xm0, xm1
    pmulhrsw            xm0, xm2
    vpbroadcastw         m0, xm0
    mova                 m1, m0
    jmp m(iadst_8x4_internal_8bpc).end3
%endif
%endmacro

INIT_YMM avx512icl
INV_TXFM_8X4_FN dct, dct
INV_TXFM_8X4_FN dct, adst
INV_TXFM_8X4_FN dct, flipadst
INV_TXFM_8X4_FN dct, identity

cglobal idct_8x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpbroadcastd        xm3, [o(pw_2896x8)]
    pmulhrsw            xm0, xm3, [cq+16*0]
    pmulhrsw            xm1, xm3, [cq+16*1]
    pmulhrsw            xm2, xm3, [cq+16*2]
    pmulhrsw            xm3,      [cq+16*3]
    call m(idct_4x8_internal_8bpc).main
    vbroadcasti32x4      m4, [o(deint_shuf)]
    vinserti32x4         m3, m1, xm3, 1
    vinserti32x4         m1, m0, xm2, 1
    shufps               m0, m1, m3, q0220
    shufps               m1, m3, q1331
    pshufb               m0, m4
    pshufb               m1, m4
    jmp                tx2q
.pass2:
    IDCT4_1D_PACKED
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    jmp m(iadst_8x4_internal_8bpc).end2

INV_TXFM_8X4_FN adst, dct
INV_TXFM_8X4_FN adst, adst
INV_TXFM_8X4_FN adst, flipadst
INV_TXFM_8X4_FN adst, identity

cglobal iadst_8x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpbroadcastd        xm0, [o(pw_2896x8)]
    pshufd              xm4,      [cq+16*0], q1032
    pmulhrsw            xm3, xm0, [cq+16*3]
    pshufd              xm5,      [cq+16*1], q1032
    pmulhrsw            xm2, xm0, [cq+16*2]
    pmulhrsw            xm4, xm0
    pmulhrsw            xm5, xm0
    call m(iadst_4x8_internal_8bpc).main_pass1
    vinserti32x4         m0, xm2, 1
    vinserti32x4         m1, xm3, 1
    pxor                 m3, m3
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    psubsw               m3, m2
    punpckhwd            m1, m0, m3
    punpcklwd            m0, m3
    jmp                tx2q
.pass2:
    call .main
.end:
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q3120
.end2:
    vpbroadcastd         m2, [o(pw_2048)]
    pmulhrsw             m0, m2
    pmulhrsw             m1, m2
.end3:
    pxor                 m2, m2
    mova               [cq], zmm18
    lea                  r6, [strideq*3]
    WRITE_8X4             0, 1, 4, 5
    RET
ALIGN function_align
.main:
    IADST4_1D_PACKED
    ret

INV_TXFM_8X4_FN flipadst, dct
INV_TXFM_8X4_FN flipadst, adst
INV_TXFM_8X4_FN flipadst, flipadst
INV_TXFM_8X4_FN flipadst, identity

cglobal iflipadst_8x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpbroadcastd        xm0, [o(pw_2896x8)]
    pshufd              xm4,      [cq+16*0], q1032
    pmulhrsw            xm3, xm0, [cq+16*3]
    pshufd              xm5,      [cq+16*1], q1032
    pmulhrsw            xm2, xm0, [cq+16*2]
    pmulhrsw            xm4, xm0
    pmulhrsw            xm5, xm0
    call m(iadst_4x8_internal_8bpc).main_pass1
    vinserti32x4         m3, m3, xm1, 1
    vinserti32x4         m2, m2, xm0, 1
    punpckhwd            m1, m3, m2
    punpcklwd            m3, m2
    pxor                 m0, m0
    psubsw               m0, m1
    punpckhwd            m1, m0, m3
    punpcklwd            m0, m3
    jmp                tx2q
.pass2:
    call m(iadst_8x4_internal_8bpc).main
    mova                 m2, m1
    vpermq               m1, m0, q2031
    vpermq               m0, m2, q2031
    jmp m(iadst_8x4_internal_8bpc).end2

INV_TXFM_8X4_FN identity, dct
INV_TXFM_8X4_FN identity, adst
INV_TXFM_8X4_FN identity, flipadst
INV_TXFM_8X4_FN identity, identity

cglobal iidentity_8x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                xm2, [cq+16*0]
    mova                xm0, [cq+16*1]
    vinserti32x4         m2, [cq+16*2], 1
    vinserti32x4         m0, [cq+16*3], 1
    vpbroadcastd         m3, [o(pw_2896x8)]
    punpcklwd            m1, m2, m0
    punpckhwd            m2, m0
    pmulhrsw             m1, m3
    pmulhrsw             m2, m3
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    paddsw               m0, m0
    paddsw               m1, m1
    jmp                tx2q
.pass2:
    vpbroadcastd         m3, [o(pw_1697x8)]
    pmulhrsw             m2, m3, m0
    pmulhrsw             m3, m1
    paddsw               m0, m2
    paddsw               m1, m3
    jmp m(iadst_8x4_internal_8bpc).end

%macro INV_TXFM_8X8_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 8x8
%ifidn %1_%2, dct_dct
INIT_ZMM avx512icl
    movsx               r6d, word [cq]
    mov                [cq], eobd
.dconly:
    imul                r6d, 181
    add                 r6d, 128+256
    sar                 r6d, 8+1
.dconly2:
    vpbroadcastd        ym2, strided
    imul                r6d, 181
    pmulld              ym5, ym2, [o(pd_0to15)]
    kxnorb               k1, k1, k1
    add                 r6d, 128+2048
    sar                 r6d, 8+4
    pxor                 m3, m3
    vpbroadcastw         m4, r6d
.dconly_loop:
    kmovb                k2, k1
    vpgatherdq       m2{k1}, [dstq+ym5]
    punpcklbw            m0, m2, m3
    punpckhbw            m1, m2, m3
    paddw                m0, m4
    paddw                m1, m4
    packuswb             m0, m1
    kmovb                k1, k2
    vpscatterdq [dstq+ym5]{k2}, m0
    lea                dstq, [dstq+strideq*8]
    sub                 r3d, 8
    jg .dconly_loop
    RET
INIT_YMM avx512icl
%endif
%endmacro

INV_TXFM_8X8_FN dct, dct
INV_TXFM_8X8_FN dct, identity
INV_TXFM_8X8_FN dct, adst
INV_TXFM_8X8_FN dct, flipadst

cglobal idct_8x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpermq               m0, [cq+32*0], q3120 ; 0 1
    vpermq               m3, [cq+32*3], q3120 ; 6 7
    vpermq               m2, [cq+32*2], q3120 ; 4 5
    vpermq               m1, [cq+32*1], q3120 ; 2 3
    call .main
    shufps               m4, m0, m1, q0220
    shufps               m5, m0, m1, q1331
    shufps               m1, m2, m3, q0220
    shufps               m3, m2, m3, q1331
    vbroadcasti32x4      m0, [o(deint_shuf)]
    vpbroadcastd         m2, [o(pw_16384)]
    REPX   {pshufb   x, m0}, m4, m5, m1, m3
    REPX   {pmulhrsw x, m2}, m4, m5, m1, m3
    vinserti32x4         m0, m4, xm1, 1
    vshufi32x4           m2, m4, m1, 0x03
    vinserti32x4         m1, m5, xm3, 1
    vshufi32x4           m3, m5, m3, 0x03
    jmp                tx2q
.pass2:
    call .main
    vpbroadcastd         m4, [o(pw_2048)]
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q2031
    vpermq               m2, m2, q3120
    vpermq               m3, m3, q2031
    jmp m(iadst_8x8_internal_8bpc).end2
ALIGN function_align
cglobal_label .main
    IDCT8_1D_PACKED
    ret

INV_TXFM_8X8_FN adst, dct
INV_TXFM_8X8_FN adst, adst
INV_TXFM_8X8_FN adst, flipadst
INV_TXFM_8X8_FN adst, identity

cglobal iadst_8x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpermq               m4, [cq+32*0], q1302 ; 1 0
    vpermq               m3, [cq+32*3], q3120 ; 6 7
    vpermq               m5, [cq+32*1], q1302 ; 3 2
    vpermq               m2, [cq+32*2], q3120 ; 4 5
    call .main_pass1
    vpbroadcastd         m5, [o(pw_16384_m16384)]
    punpcklwd            m4, m0, m1
    punpckhwd            m0, m1
    punpcklwd            m1, m2, m3
    punpckhwd            m2, m3
    punpcklwd            m3, m4, m0
    punpckhwd            m4, m0
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    REPX   {pmulhrsw x, m5}, m3, m4, m0, m1
    vshufi32x4           m2, m3, m0, 0x03
    vinserti32x4         m0, m3, xm0, 1
    vshufi32x4           m3, m4, m1, 0x03
    vinserti32x4         m1, m4, xm1, 1
    jmp                tx2q
.pass2:
    pshufd               m4, m0, q1032
    pshufd               m5, m1, q1032
    call .main_pass2
    vpbroadcastd         m5, [o(pw_2048)]
    vpbroadcastd        xm4, [o(pw_4096)]
    psubw                m4, m5 ; lower half = 2048, upper half = -2048
.end:
    REPX {vpermq x, x, q3120}, m0, m1, m2, m3
.end2:
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
.end3:
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
.end4:
    pxor                 m4, m4
    mova          [cq+32*0], m4
    mova          [cq+32*1], m4
    mova          [cq+32*2], m4
    mova          [cq+32*3], m4
    lea                  r6, [strideq*3]
    WRITE_8X4             0, 1, 4, 5
    lea                dstq, [dstq+strideq*4]
    WRITE_8X4             2, 3, 4, 5
    RET
ALIGN function_align
.main_pass1:
    punpckhwd            m0, m4, m3 ; 0 7
    punpckhwd            m1, m5, m2 ; 2 5
    punpcklwd            m2, m5     ; 4 3
    punpcklwd            m3, m4     ; 6 1
    IADST8_1D_PACKED 1
    punpcklqdq           m3, m4, m0        ; out6 -out7
    punpckhqdq           m0, m4            ; out0 -out1
    ret
ALIGN function_align
cglobal_label .main_pass2
    IADST8_1D_PACKED 2
    ret

INV_TXFM_8X8_FN flipadst, dct
INV_TXFM_8X8_FN flipadst, adst
INV_TXFM_8X8_FN flipadst, flipadst
INV_TXFM_8X8_FN flipadst, identity

cglobal iflipadst_8x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpermq               m4, [cq+32*0], q1302 ; 1 0
    vpermq               m3, [cq+32*3], q3120 ; 6 7
    vpermq               m5, [cq+32*1], q1302 ; 3 2
    vpermq               m2, [cq+32*2], q3120 ; 4 5
    call m(iadst_8x8_internal_8bpc).main_pass1
    vpbroadcastd         m5, [o(pw_m16384_16384)]
    punpckhwd            m4, m3, m2
    punpcklwd            m3, m2
    punpckhwd            m2, m1, m0
    punpcklwd            m1, m0
    punpckhwd            m0, m4, m3
    punpcklwd            m4, m3
    punpckhwd            m3, m2, m1
    punpcklwd            m2, m1
    REPX   {pmulhrsw x, m5}, m0, m4, m3, m2
    vinserti32x4         m1, m0, xm3, 1
    vshufi32x4           m3, m0, m3, 0x03
    vinserti32x4         m0, m4, xm2, 1
    vshufi32x4           m2, m4, m2, 0x03
    jmp                tx2q
.pass2:
    pshufd               m4, m0, q1032
    pshufd               m5, m1, q1032
    call m(iadst_8x8_internal_8bpc).main_pass2
    vpbroadcastd         m4, [o(pw_2048)]
    vpbroadcastd        xm5, [o(pw_4096)]
    psubw                m4, m5 ; lower half = -2048, upper half = 2048
    vpermq               m5, m3, q2031
    vpermq               m3, m0, q2031
    vpermq               m0, m2, q2031
    vpermq               m2, m1, q2031
    pmulhrsw             m1, m0, m4
    pmulhrsw             m0, m5, m4
    jmp m(iadst_8x8_internal_8bpc).end3

INV_TXFM_8X8_FN identity, dct
INV_TXFM_8X8_FN identity, adst
INV_TXFM_8X8_FN identity, flipadst
INV_TXFM_8X8_FN identity, identity

cglobal iidentity_8x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                xm3, [cq+16*0]
    mova                xm2, [cq+16*1]
    vinserti32x4         m3, [cq+16*4], 1
    vinserti32x4         m2, [cq+16*5], 1
    mova                xm4, [cq+16*2]
    mova                xm0, [cq+16*3]
    vinserti32x4         m4, [cq+16*6], 1
    vinserti32x4         m0, [cq+16*7], 1
    punpcklwd            m1, m3, m2
    punpckhwd            m3, m2
    punpcklwd            m2, m4, m0
    punpckhwd            m4, m0
    punpckldq            m0, m1, m2
    punpckhdq            m1, m2
    punpckldq            m2, m3, m4
    punpckhdq            m3, m4
    jmp                tx2q
.pass2:
    vpbroadcastd         m4, [o(pw_4096)]
    jmp m(iadst_8x8_internal_8bpc).end

%macro INV_TXFM_8X16_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 8x16
%ifidn %1_%2, dct_dct
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 16
    imul                r6d, 181
    add                 r6d, 128
    sar                 r6d, 8
    jmp m(inv_txfm_add_dct_dct_8x8_8bpc).dconly
%endif
%endmacro

%macro ITX_8X16_LOAD_COEFS 0
    vpbroadcastd         m4, [o(pw_2896x8)]
    pmulhrsw             m0, m4, [cq+32*0]
    add                  cq, 32*4
    pmulhrsw             m7, m4, [cq+32*3]
    pmulhrsw             m1, m4, [cq-32*3]
    pmulhrsw             m6, m4, [cq+32*2]
    pmulhrsw             m2, m4, [cq-32*2]
    pmulhrsw             m5, m4, [cq+32*1]
    pmulhrsw             m3, m4, [cq-32*1]
    pmulhrsw             m4,     [cq+32*0]
%endmacro

INIT_ZMM avx512icl
INV_TXFM_8X16_FN dct, dct
INV_TXFM_8X16_FN dct, identity
INV_TXFM_8X16_FN dct, adst
INV_TXFM_8X16_FN dct, flipadst

cglobal idct_8x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m3, [o(permB)]
    vpermq               m0, m3, [cq+64*0]
    vpbroadcastd         m4, [o(pw_2896x8)]
    vpermq               m1, m3, [cq+64*1]
    vpermq               m2, m3, [cq+64*2]
    vpermq               m3, m3, [cq+64*3]
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
    call m(idct_16x8_internal_8bpc).main
    vpbroadcastd         m5, [o(pw_16384)]
    punpckhwd            m4, m0, m2 ; b0 f0 b1 f1 b2 f2 b3 f3
    punpcklwd            m0, m2     ; a0 e0 a1 e1 a2 e2 a3 e3
    punpckhwd            m2, m1, m3 ; c0 g0 c1 g1 c2 g2 c3 g3
    punpcklwd            m1, m3     ; d0 h0 d1 h1 d2 h2 d3 h3
    REPX   {pmulhrsw x, m5}, m4, m0, m2, m1
    punpckhwd            m3, m0, m4 ; a2 b2 e2 f2 a3 b3 e3 f3
    punpcklwd            m0, m4     ; a0 b0 e0 f0 a1 b1 e1 f1
    punpckhwd            m4, m2, m1 ; c2 d2 g2 h2 c3 d3 g3 h3
    punpcklwd            m2, m1     ; c0 d0 g0 h0 c1 d1 g1 h1
    punpckhdq            m1, m0, m2 ;  1  5  9 13
    punpckldq            m0, m2     ;  0  4  8 12
    punpckldq            m2, m3, m4 ;  2  6 10 14
    punpckhdq            m3, m4     ;  3  7 11 15
    jmp                tx2q
.pass2:
    vprord               m5, [o(int16_perm)], 16
    vshufi32x4           m2, m2, q1320     ;  2 10 14  6
    vshufi32x4           m4, m1, m3, q2310 ;  1  5 15 11
    vshufi32x4           m1, m3, q0132     ;  9 13  7  3
    vpermb               m9, m5, m0
    vpermb               m7, m5, m2
    vpermb               m8, m5, m4
    vpermb               m0, m5, m1
    vextracti32x8       ym6, m9, 1
    vextracti32x8       ym3, m7, 1
    vextracti32x8       ym5, m8, 1
    vextracti32x8       ym1, m0, 1
    call .main2
    mova                ym8, [o(gather8a)]
    lea                  r3, [dstq+strideq*4]
    pmovzxdq             m9, ym8
    pshufd              ym8, ym8, q1230
    vpermt2q             m0, m9, m4
    vpermt2q             m1, m9, m5
    vpermt2q             m2, m9, m6
    vpermt2q             m3, m9, m7
.end:
    vpbroadcastd         m7, [o(pw_2048)]
.end2:
    pmulhrsw             m0, m7
    pmulhrsw             m1, m7
.end3:
    pmulhrsw             m2, m7
    pmulhrsw             m3, m7
.end4:
    vpbroadcastd        ym6, strided
    kxnorb               k1, k1, k1
    pxor                 m4, m4
    pmulld              ym8, ym6
    kmovb                k2, k1
    vpgatherdq       m6{k1}, [dstq+ym8]
    kmovb                k1, k2
    vpgatherdq       m7{k2}, [r3+ym8]
    mova          [cq+64*0], m4
    mova          [cq+64*1], m4
    kmovb                k2, k1
    mova          [cq+64*2], m4
    mova          [cq+64*3], m4
    punpcklbw            m5, m6, m4
    punpckhbw            m6, m4
    paddw                m0, m5
    paddw                m1, m6
    packuswb             m0, m1
    vpscatterdq [dstq+ym8]{k1}, m0
    punpcklbw            m6, m7, m4
    punpckhbw            m7, m4
    paddw                m2, m6
    paddw                m3, m7
    packuswb             m2, m3
    vpscatterdq [r3+ym8]{k2}, m2
    RET
ALIGN function_align
cglobal_label .main_fast2 ; bottom three-quarters are zero
    vpbroadcastd       ym10, [o(pd_2048)]
    vpbroadcastq       ym13, [o(int_mshift)]
    vpbroadcastd        ym3, [o(pw_401_4076x8)]
    vpbroadcastd        ym5, [o(pw_799_4017x8)]
    vpbroadcastd        ym4, [o(pw_m1189_3920x8)]
    pxor                ym6, ym6
    punpckhwd           ym2, ym0, ym0
    pmulhrsw            ym2, ym3      ; t8a  t15a
    punpcklwd           ym7, ym1, ym1
    pmulhrsw            ym7, ym5      ; t4a  t7a
    punpckhwd           ym1, ym1
    pmulhrsw            ym4, ym1      ; t11a t12a
    vpcmpub              k7, ym13, ym10, 6
    punpcklwd           ym9, ym6, ym0
    psubsw              ym0, ym2, ym4 ; t11a t12a
    paddsw              ym8, ym2, ym4 ; t8a  t15a
    mova                ym1, ym7
    jmp .main5
ALIGN function_align
cglobal_label .main_fast ; bottom half is zero
    vpbroadcastd       ym10, [o(pd_2048)]
    vpbroadcastq       ym13, [o(int_mshift)]
    pxor                ym6, ym6
    punpckhwd           ym8, ym0, ym0
    punpckhwd           ym4, ym3, ym3
    punpckhwd           ym5, ym2, ym2
    punpcklwd           ym7, ym1, ym1
    punpckhwd           ym1, ym1
    punpcklwd           ym3, ym3
    punpcklwd           ym9, ym6, ym0
    punpcklwd           ym6, ym2
    vpbroadcastd        ym2, [o(pw_401_4076x8)]
    vpbroadcastd        ym0, [o(pw_m2598_3166x8)]
    vpbroadcastd       ym11, [o(pw_1931_3612x8)]
    vpbroadcastd       ym12, [o(pw_m1189_3920x8)]
    pmulhrsw            ym8, ym2  ; t8a  t15a
    vpbroadcastd        ym2, [o(pw_799_4017x8)]
    pmulhrsw            ym0, ym4  ; t9a  t14a
    vpbroadcastd        ym4, [o(pw_m2276_3406x8)]
    pmulhrsw            ym5, ym11 ; t10a t13a
    pmulhrsw            ym1, ym12 ; t11a t12a
    pmulhrsw            ym7, ym2  ; t4a  t7a
    pmulhrsw            ym3, ym4  ; t5a  t6a
    vpcmpub              k7, ym13, ym10, 6
    jmp .main4
ALIGN function_align
cglobal_label .main
    WRAP_YMM IDCT16_1D_PACKED
    ret

INV_TXFM_8X16_FN adst, dct
INV_TXFM_8X16_FN adst, adst
INV_TXFM_8X16_FN adst, flipadst
INV_TXFM_8X16_FN adst, identity

cglobal iadst_8x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    call m(iadst_16x8_internal_8bpc).main_pass1
    vbroadcasti32x4      m6, [o(int_shuf1)]
    vpbroadcastd         m7, [o(pw_16384_m16384)]
    punpckhwd            m3, m0, m4 ; a0 b0 a1 b1 a2 b2 a3 b3
    punpcklwd            m4, m0     ; g0 h0 g1 h1 g2 h2 g3 h3
    pshufb               m5, m1, m6 ; c0 d0 c1 d1 c2 d2 c3 d3
    pshufb               m2, m6     ; e0 f0 e1 f1 e2 f2 e3 f3
.pass1_end:
    REPX   {pmulhrsw x, m7}, m3, m5, m4, m2
    punpckldq            m0, m3, m5 ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhdq            m3, m5     ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckhdq            m5, m2, m4 ; e2 f2 g2 h2 e3 f3 g3 h3
    punpckldq            m2, m4     ; e0 f0 g0 h0 e1 f1 g1 h1
    punpckhqdq           m1, m0, m2
    punpcklqdq           m0, m2
    punpcklqdq           m2, m3, m5
    punpckhqdq           m3, m5
    jmp                tx2q
.pass2:
    call .main_pass2
    vpbroadcastd         m6, [o(pw_2048)]
    psrlq               m10, 4
    psubw                m7, m8, m6
.pass2_end:
    vpbroadcastd         m5, [o(pw_2896x8)]
    paddsw               m1, m2, m4
    psubsw               m2, m4
    pmulhrsw             m1, m5      ; -out7   out4   out6  -out5
    pmulhrsw             m5, m2      ;  out8  -out11 -out9   out10
    mova                ym8, [o(gather8c)]
    lea                  r3, [dstq+strideq]
    psrlq                m2, m10, 4
    vpermi2q             m2, m0, m3  ;  1  3 13 15
    vpermt2q             m0, m10, m3 ;  0  2 12 14
    psrlq                m3, m10, 8
    vpermi2q             m3, m1, m5  ;  5  7  9 11
    psrlq               m10, 12
    vpermt2q             m1, m10, m5 ;  4  6  8 10
    pmulhrsw             m0, m6
    pmulhrsw             m1, m6
    jmp m(idct_8x16_internal_8bpc).end3
ALIGN function_align
.main_pass1:
    vpbroadcastd         m2, [o(pw_2896x8)]
    pmulhrsw             m5, m2, [cq+64*0]
    pmulhrsw             m3, m2, [cq+64*3]
    pmulhrsw             m1, m2, [cq+64*1]
    pmulhrsw             m2,     [cq+64*2]
    movu                 m4, [o(permA+3)]
    psrlq               m10, m4, 4
    mova                 m6, m4
    vpermi2q             m4, m5, m3  ; in0  in12 in2  in14
    vpermt2q             m5, m10, m3 ; in15 in3  in13 in1
    vpermi2q             m6, m1, m2  ; in4  in8  in6  in10
    vpermt2q             m1, m10, m2 ; in11 in7  in9  in5
    jmp .main
ALIGN function_align
.main_pass2:
    mova                 m4, [o(permC)]
    psrlq                m5, m4, 4
    vpermi2q             m4, m0, m2  ; in0  in12 in2  in14
    psrlq                m6, m5, 4
    vpermi2q             m5, m1, m3  ; in15 in3  in13 in1
    psrlq               m10, m6, 4
    vpermi2q             m6, m0, m2  ; in4  in8  in6  in10
    vpermt2q             m1, m10, m3 ; in11 in7  in9  in5
.main:
    punpcklwd            m0, m4, m5  ; in0  in15 in2  in13
    punpckhwd            m4, m5      ; in12 in3  in14 in1
    punpcklwd            m5, m6, m1  ; in4  in11 in6  in9
    punpckhwd            m6, m1      ; in8  in7  in10 in5
cglobal_label .main2
    vpbroadcastd         m9, [o(pd_2048)]
    vpbroadcastq        m13, [o(int_mshift)]
    kxnorb               k1, k1, k1
    vpcmpub              k7, m13, m9, 6 ; 0x33...
    pxor                 m8, m8
    ITX_MUL4X_PACK        0, 1, 2, 3, 7, 9,  201, 4091,  995, 3973, 5
    ITX_MUL4X_PACK        6, 1, 2, 3, 7, 9, 3035, 2751, 3513, 2106, 5
    ITX_MUL4X_PACK        4, 1, 2, 3, 7, 9, 3857, 1380, 4052,  601, 5
    ITX_MUL4X_PACK        5, 1, 2, 3, 7, 9, 1751, 3703, 2440, 3290, 5
    psubsw               m2, m0, m6 ; t9a  t8a  t11a t10a
    paddsw               m0, m6     ; t1a  t0a  t3a  t2a
    psubsw               m3, m5, m4 ; t13a t12a t15a t14a
    paddsw               m5, m4     ; t5a  t4a  t7a  t6a
    ITX_MUL4X_PACK        2, 4, 1, 6, 7, 9,  799, 4017, 3406, 2276, 5
    psubw                m7, m8, m7
    ITX_MUL2X_PACK        3, 4, 1, 9, 7, 6, 4
    vpbroadcastd         m6, [o(pw_3784_m1567)]
    vpbroadcastd     m6{k1}, [o(pw_m3784_1567)]
    psubsw               m1, m0, m5 ; t5   t4   t7   t6
    paddsw               m0, m5     ; t1   t0   t3   t2
    psubsw               m4, m2, m3 ; t13a t12a t15a t14a
    paddsw               m2, m3     ; t9a  t8a  t11a t10a
    ITX_MUL2X_PACK        1, 3, 5, 9, 1567_3784, 6, 16 ; t5a t4a t6a t7a
    ITX_MUL2X_PACK        4, 3, 5, 9, 1567_3784, 6, 16 ; t13 t12 t14 t15
    vbroadcasti32x4      m5, [o(deint_shuf)]
    pshufb               m0, m5
    pshufb               m2, m5
    vshufi32x4           m3, m0, m2, q3232 ; t3   t2   t11a t10a
    vinserti32x8         m0, ym2, 1        ; t1   t0   t9a  t8a
    vshufi32x4           m2, m1, m4, q3232 ; t6a  t7a  t14  t15
    vinserti32x8         m1, ym4, 1        ; t5a  t4a  t13  t12
    pshufd               m2, m2, q1032     ; t7a  t6a  t15  t14
    psubsw               m4, m0, m3        ; t3a t2a t11 t10
    paddsw               m0, m3            ; -out15  out0   out14 -out1
    paddsw               m3, m1, m2        ;  out12 -out3  -out13  out2
    psubsw               m1, m2            ; t7 t6 t15a t14a
    punpckhqdq           m2, m4, m1        ; t2a t6  t10 t14a
    punpcklqdq           m4, m1            ; t3a t7  t11 t15a
    ret

INV_TXFM_8X16_FN flipadst, dct
INV_TXFM_8X16_FN flipadst, adst
INV_TXFM_8X16_FN flipadst, flipadst
INV_TXFM_8X16_FN flipadst, identity

cglobal iflipadst_8x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    call m(iadst_16x8_internal_8bpc).main_pass1
    vbroadcasti32x4      m6, [o(int_shuf2)]
    vpbroadcastd         m7, [o(pw_m16384_16384)]
    punpcklwd            m3, m0, m4 ; a0 b0 a1 b1 a2 b2 a3 b3
    punpckhwd            m4, m0     ; g0 h0 g1 h1 g2 h2 g3 h3
    pshufb               m5, m2, m6 ; c0 d0 c1 d1 c2 d2 c3 d3
    pshufb               m2, m1, m6 ; e0 f0 e1 f1 e2 f2 e3 f3
    jmp m(iadst_8x16_internal_8bpc).pass1_end
.pass2:
    call m(iadst_8x16_internal_8bpc).main_pass2
    vpbroadcastd         m7, [o(pw_2048)]
    psrlq               m10, 36
    psubw                m6, m8, m7
    jmp m(iadst_8x16_internal_8bpc).pass2_end

INV_TXFM_8X16_FN identity, dct
INV_TXFM_8X16_FN identity, adst
INV_TXFM_8X16_FN identity, flipadst
INV_TXFM_8X16_FN identity, identity

cglobal iidentity_8x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m0, [o(int16_perm)]
    vpermb               m3, m0, [cq+64*0] ; a0 b0 a1 b1 a2 b2 a3 b3
    vpermb               m2, m0, [cq+64*1] ; c0 d0 c1 d1 c2 d2 c3 d3
    vpermb               m4, m0, [cq+64*2] ; e0 f0 e1 f1 e2 f2 e3 f3
    vpermb               m0, m0, [cq+64*3] ; g0 h0 g1 h1 g2 h2 g3 h3
    vpbroadcastd         m5, [o(pw_2896x8)]
    punpckldq            m1, m3, m2        ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhdq            m3, m2            ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckldq            m2, m4, m0        ; e0 f0 g0 h0 a1 f1 g1 h1
    punpckhdq            m4, m0            ; e2 f2 g2 h2 e3 f3 g3 h3
    REPX   {pmulhrsw x, m5}, m1, m2, m3, m4
    punpcklqdq           m0, m1, m2        ; a0 b0 c0 d0 e0 f0 g0 h0
    punpckhqdq           m1, m2            ; a1 b1 c1 d1 e1 f1 g1 h1
    punpcklqdq           m2, m3, m4        ; a2 b2 c2 d2 e2 f2 g2 h2
    punpckhqdq           m3, m4            ; a3 b3 c3 d3 e3 f3 g3 h3
    jmp                tx2q
.pass2:
    vpbroadcastd         m7, [o(pw_1697x16)]
    mova                ym8, [o(gather8b)]
    lea                  r3, [dstq+strideq*2]
    pmulhrsw             m4, m7, m0
    pmulhrsw             m5, m7, m1
    pmulhrsw             m6, m7, m2
    pmulhrsw             m7, m3
    REPX      {paddsw x, x}, m0, m1, m2, m3
    paddsw               m0, m4
    paddsw               m1, m5
    paddsw               m2, m6
    paddsw               m3, m7
    jmp m(idct_8x16_internal_8bpc).end

%macro WRITE_16X2 6 ; coefs[1-2], tmp[1-2], offset[1-2]
    pmovzxbw            m%3, [dstq+%5]
%ifnum %1
    paddw               m%3, m%1
%else
    paddw               m%3, %1
%endif
    pmovzxbw            m%4, [dstq+%6]
%ifnum %2
    paddw               m%4, m%2
%else
    paddw               m%4, %2
%endif
    packuswb            m%3, m%4
    vpermq              m%3, m%3, q3120
    mova          [dstq+%5], xm%3
    vextracti32x4 [dstq+%6], m%3, 1
%endmacro

%macro INV_TXFM_16X4_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 16x4
%ifidn %1_%2, dct_dct
    movsx               r6d, word [cq]
    mov                [cq], eobd
    jmp m(inv_txfm_add_dct_dct_16x8_8bpc).dconly2
%endif
%endmacro

INIT_ZMM avx512icl
INV_TXFM_16X4_FN dct, dct
INV_TXFM_16X4_FN dct, adst
INV_TXFM_16X4_FN dct, flipadst
INV_TXFM_16X4_FN dct, identity

cglobal idct_16x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                xm0, [cq+16*0]
    mova                xm1, [cq+16*1]
    mova                xm2, [cq+16*2]
    mova                xm3, [cq+16*3]
    mova                xm4, [cq+16*4]
    mova                xm5, [cq+16*5]
    mova                xm6, [cq+16*6]
    mova                xm7, [cq+16*7]
    call m(idct_4x16_internal_8bpc).main
    vpbroadcastd         m8, [o(pw_16384)]
    vinserti32x4        ym1, xm3, 1 ; 3 2   7 6
    vinserti32x4        ym5, xm7, 1 ; b a   f e
    vinserti32x4        ym0, xm2, 1 ; 0 1   4 5
    vinserti32x4        ym4, xm6, 1 ; 8 9   c d
    vinserti32x8         m1, ym5, 1 ; 3 2   7 6   b a   f e
    vinserti32x8         m0, ym4, 1 ; 0 1   4 5   8 9   c d
    pmulhrsw             m1, m8
    pmulhrsw             m0, m8
    pshufd               m1, m1, q1032
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    jmp                tx2q
.pass2:
    IDCT4_1D_PACKED
    mova                 m2, [o(permA)]
    jmp m(iadst_16x4_internal_8bpc).end

INV_TXFM_16X4_FN adst, dct
INV_TXFM_16X4_FN adst, adst
INV_TXFM_16X4_FN adst, flipadst
INV_TXFM_16X4_FN adst, identity

cglobal iadst_16x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+64*0]
    mova                 m1, [cq+64*1]
    movshdup             m3, [o(permB)]
    psrlq               m10, m3, 4
    call m(iadst_4x16_internal_8bpc).main2
    vpbroadcastd         m6, [o(pw_16384_m16384)]
    psrlq                m0, m10, 4
    psrlq               m10, 8
.pass1_end:
    punpcklwd           ym5, ym4, ym2
    punpckhwd           ym4, ym2
    vinserti32x8         m5, ym4, 1
    mova                 m1, m9
    vpdpwssd             m1, m5, [o(pw_m2896_2896)] {1to16}
    mova                 m4, m9
    vpdpwssd             m4, m5, [o(pw_2896_2896)] {1to16}
    psrad                m1, 12
    psrad                m4, 12
    packssdw             m1, m4 ;  out8  -out7  -out9   out6  -out11  out4   out10 -out5
    vpermi2q             m0, m1, m3  ; 0 1   4 5   8 9   c d
    vpermt2q             m1, m10, m3 ; 2 3   6 7   a b   e f
    punpckhwd            m2, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m0, m2
    punpcklwd            m0, m2
    pmulhrsw             m0, m6
    pmulhrsw             m1, m6
    jmp                tx2q
.pass2:
    call .main
    movu                 m2, [o(permA+1)]
.end:
    vpbroadcastd         m3, [o(pw_2048)]
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
.end2:
    psrlq                m3, m2, 4
    vpermi2q             m2, m0, m1
    vpermi2q             m3, m0, m1
.end3:
    lea                  r3, [dstq+strideq*2]
    mova                xm1, [dstq+strideq*0]
    vinserti32x4        ym1, [dstq+strideq*1], 1
    vinserti32x4         m1, [r3  +strideq*0], 2
    vinserti32x4         m1, [r3  +strideq*1], 3
    pxor                 m4, m4
    mova          [cq+64*0], m4
    mova          [cq+64*1], m4
    punpcklbw            m0, m1, m4
    punpckhbw            m1, m4
    paddw                m0, m2
    paddw                m1, m3
    packuswb             m0, m1
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [r3  +strideq*0], m0, 2
    vextracti32x4 [r3  +strideq*1], m0, 3
    RET
ALIGN function_align
.main:
    IADST4_1D_PACKED
    ret

INV_TXFM_16X4_FN flipadst, dct
INV_TXFM_16X4_FN flipadst, adst
INV_TXFM_16X4_FN flipadst, flipadst
INV_TXFM_16X4_FN flipadst, identity

cglobal iflipadst_16x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m0, [cq+64*0]
    mova                 m1, [cq+64*1]
    movshdup             m3, [o(permB)]
    psrlq               m10, m3, 4
    call m(iadst_4x16_internal_8bpc).main2
    vpbroadcastd         m6, [o(pw_m16384_16384)]
    psrlq                m0, m10, 12
    psrlq               m10, 16
    jmp m(iadst_16x4_internal_8bpc).pass1_end
.pass2:
    call m(iadst_16x4_internal_8bpc).main
    movu                m2, [o(permA+2)]
    jmp m(iadst_16x4_internal_8bpc).end

INV_TXFM_16X4_FN identity, dct
INV_TXFM_16X4_FN identity, adst
INV_TXFM_16X4_FN identity, flipadst
INV_TXFM_16X4_FN identity, identity

cglobal iidentity_16x4_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m1, [cq+64*0]
    mova                 m2, [cq+64*1]
    vpbroadcastd         m3, [o(pw_1697x16)]
    vpbroadcastd         m4, [o(pw_16384)]
    mova                 m5, [o(idtx_16x4p)]
    shufps               m0, m1, m2, q2020
    shufps               m1, m2, q3131
    pmulhrsw             m2, m3, m0
    pmulhrsw             m3, m1
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
    paddsw               m0, m2
    paddsw               m1, m3
    vpermb               m0, m5, m0
    vpermb               m1, m5, m1
    jmp                tx2q
.pass2:
    vpbroadcastd         m3, [o(pw_1697x8)]
    pmulhrsw             m2, m3, m0
    pmulhrsw             m3, m1
    paddsw               m0, m2
    paddsw               m1, m3
    movu                 m2, [o(permA+1)]
    jmp m(iadst_16x4_internal_8bpc).end

%macro INV_TXFM_16X8_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 16x8
%ifidn %1_%2, dct_dct
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 8
.dconly:
    imul                r6d, 181
    add                 r6d, 128
    sar                 r6d, 8
.dconly2:
    imul                r6d, 181
    add                 r6d, 128+256
    sar                 r6d, 8+1
.dconly3:
    imul                r6d, 181
    lea                  r2, [strideq*3]
    add                 r6d, 128+2048
    sar                 r6d, 8+4
    pxor                 m2, m2
    vpbroadcastw         m3, r6d
.dconly_loop:
    mova                xm1, [dstq+strideq*0]
    vinserti32x4        ym1, [dstq+strideq*1], 1
    vinserti32x4         m1, [dstq+strideq*2], 2
    vinserti32x4         m1, [dstq+r2       ], 3
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    paddw                m0, m3
    paddw                m1, m3
    packuswb             m0, m1
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+r2       ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                 r3d, 4
    jg .dconly_loop
    RET
%endif
%endmacro

%macro ITX_16X8_LOAD_COEFS 1 ; shuf_odd
    vpbroadcastd         m8, [o(pw_2896x8)]
    vpermq               m0, [cq+32*0], q3120
    add                  cq, 32*4
    vpermq               m7, [cq+32*3], q%1
    vpermq               m1, [cq-32*3], q%1
    vpermq               m6, [cq+32*2], q3120
    vpermq               m2, [cq-32*2], q3120
    vpermq               m5, [cq+32*1], q%1
    vpermq               m3, [cq-32*1], q%1
    vpermq               m4, [cq+32*0], q3120
    REPX   {pmulhrsw x, m8}, m0, m7, m1, m6, m2, m5, m3, m4
%endmacro

INV_TXFM_16X8_FN dct, dct
INV_TXFM_16X8_FN dct, identity
INV_TXFM_16X8_FN dct, adst
INV_TXFM_16X8_FN dct, flipadst

cglobal idct_16x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpbroadcastd         m1, [o(pw_2896x8)]
    vpermq               m0, [cq+64*0], q3120
    vpermq               m2, [cq+64*1], q3120
    vpermq               m4, [cq+64*2], q3120
    vpermq               m6, [cq+64*3], q3120
    REPX   {pmulhrsw x, m1}, m0, m2, m4, m6
    vextracti32x8       ym1, m0, 1
    vextracti32x8       ym3, m2, 1
    vextracti32x8       ym5, m4, 1
    vextracti32x8       ym7, m6, 1
    call m(idct_8x16_internal_8bpc).main
    vbroadcasti32x4      m8, [o(int_shuf1)]
    vbroadcasti32x4      m9, [o(int_shuf2)]
    vinserti32x8         m0, ym2, 1 ; a0 a1 a2 a3 b0 b1 b2 b3
    vinserti32x8         m1, ym3, 1 ; d0 d1 d2 d3 c0 c1 c2 c3
    vinserti32x8         m4, ym6, 1 ; i0 i1 i2 i3 j0 j1 j2 j3
    vinserti32x8         m5, ym7, 1 ; l0 l1 l2 l3 k0 k1 k2 k3
    vpbroadcastd         m2, [o(pw_16384)]
    pshufb               m0, m8     ; a0 b0 a1 b1 a2 b2 a3 b3
    pshufb               m1, m9     ; c0 d0 c1 d1 c2 d2 c3 d3
    pshufb               m6, m4, m8 ; i0 j0 i1 j1 i2 j2 i3 j3
    pshufb               m7, m5, m9 ; m0 n0 m1 n1 m2 n2 m3 n3
    REPX   {pmulhrsw x, m2}, m0, m1, m6, m7
    punpckldq            m2, m0, m1 ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhdq            m3, m0, m1 ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckldq            m4, m6, m7 ; i0 j0 k0 l0 i1 j1 k1 l1
    punpckhdq            m5, m6, m7 ; i2 j2 k2 l2 i3 j3 k3 l3
    jmp                tx2q
.pass2:
    vshufi32x4           m0, m2, m4, q2020 ; 0 1
    vshufi32x4           m2, m4, q3131     ; 4 5
    vshufi32x4           m1, m3, m5, q2020 ; 2 3
    vshufi32x4           m3, m5, q3131     ; 6 7
    call .main
    movshdup             m4, [o(permC)]
    psrlq                m6, m4, 4
    vpermq               m5, m4, q1032
    vpermi2q             m4, m0, m2 ; a2 a3   b2 b3   e2 e3   f2 f3
    vpermt2q             m0, m6, m2 ; a0 a1   b0 b1   e0 e1   f0 f1
    psrlq                m6, m5, 4
    vpermi2q             m5, m1, m3 ; c2 c3   d2 d3   g2 g3   h2 h3
    vpermt2q             m1, m6, m3 ; c0 c1   d0 d1   g0 g1   h0 h1
    vpbroadcastd         m6, [o(pw_2048)]
.end:
    REPX   {pmulhrsw x, m6}, m0, m4, m1, m5
.end2:
    lea                  r3, [dstq+strideq*4]
    lea                  r4, [strideq*3]
    mova                xm3, [dstq+strideq*0]
    mova                xm6, [dstq+strideq*2]
    vinserti32x4        ym3, [dstq+strideq*1], 1
    vinserti32x4        ym6, [dstq+r4       ], 1
    vinserti32x4         m3, [r3  +strideq*0], 2
    vinserti32x4         m6, [r3  +strideq*2], 2
    vinserti32x4         m3, [r3  +strideq*1], 3
    vinserti32x4         m6, [r3  +r4       ], 3
    pxor                 m7, m7
    mova          [cq+64*0], m7
    mova          [cq+64*1], m7
    mova          [cq+64*2], m7
    mova          [cq+64*3], m7
    punpcklbw            m2, m3, m7
    punpckhbw            m3, m7
    paddw                m0, m2
    paddw                m4, m3
    packuswb             m0, m4
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [r3  +strideq*0], m0, 2
    vextracti32x4 [r3  +strideq*1], m0, 3
    punpcklbw            m3, m6, m7
    punpckhbw            m6, m7
    paddw                m1, m3
    paddw                m5, m6
    packuswb             m1, m5
    mova          [dstq+strideq*2], xm1
    vextracti32x4 [dstq+r4       ], ym1, 1
    vextracti32x4 [r3  +strideq*2], m1, 2
    vextracti32x4 [r3  +r4       ], m1, 3
    RET
ALIGN function_align
cglobal_label .main
    IDCT8_1D_PACKED
    ret

INV_TXFM_16X8_FN adst, dct
INV_TXFM_16X8_FN adst, adst
INV_TXFM_16X8_FN adst, flipadst
INV_TXFM_16X8_FN adst, identity

cglobal iadst_16x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    call m(iadst_8x16_internal_8bpc).main_pass1
    vpbroadcastd         m7, [o(pw_16384_m16384)]
    psrlq               m10, 4
.pass1_end:
    punpcklwd            m5, m4, m2
    punpckhwd            m4, m2
    mova                 m1, m9
    vpdpwssd             m1, m5, [o(pw_m2896_2896)] {1to16}
    mova                 m6, m9
    vpdpwssd             m6, m5, [o(pw_2896_2896)] {1to16}
    mova                 m2, m9
    vpdpwssd             m2, m4, [o(pw_m2896_2896)] {1to16}
    vpdpwssd             m9, m4, [o(pw_2896_2896)] {1to16}
    psrad                m1, 12
    psrad                m6, 12
    packssdw             m1, m6 ;  out8  -out7  -out9   out6
    psrad                m2, 12
    psrad                m9, 12
    packssdw             m2, m9 ; -out11  out4   out10 -out5
    psrlq                m4, m10, 4
    vpermi2q             m4, m0, m2
    vpermt2q             m0, m10, m2
    psrlq                m5, m10, 8
    vpermi2q             m5, m1, m3
    psrlq               m10, 12
    vpermt2q             m1, m10, m3
    punpcklwd            m3, m4, m5 ; a0 c0 a1 c1 a2 c2 a3 c3
    punpckhwd            m4, m5     ; b0 d0 b1 d1 b2 d2 b3 d3
    punpcklwd            m5, m1, m0 ; i0 k0 i1 k1 2i k2 i3 k3
    punpckhwd            m1, m0     ; j0 l0 j1 l1 j2 l2 j3 l3
    punpcklwd            m2, m3, m4 ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhwd            m3, m4     ; a2 b2 c2 d2 a3 b3 c3 d3
    punpcklwd            m4, m5, m1 ; i0 j0 k0 l0 i1 j1 k1 l1
    punpckhwd            m5, m1     ; i2 j2 k2 l2 i3 j3 k3 l3
    REPX   {pmulhrsw x, m7}, m2, m3, m4, m5
    jmp                tx2q
.pass2:
    vshufi32x4           m0, m2, m4, q2020
    vshufi32x4           m2, m4, q3131     ; 4 5
    vshufi32x4           m1, m3, m5, q2020
    vshufi32x4           m3, m5, q3131     ; 6 7
    pshufd               m4, m0, q1032     ; 1 0
    pshufd               m5, m1, q1032     ; 3 2
    call .main_pass2
    movshdup             m4, [o(permC)]
    pmulhrsw             m0, m6
    pmulhrsw             m1, m6
    psrlq                m6, m4, 4
    mova                 m5, m4
    vpermi2q             m4, m0, m2
    vpermt2q             m0, m6, m2
    vpermi2q             m5, m1, m3
    vpermt2q             m1, m6, m3
    jmp m(idct_16x8_internal_8bpc).end2
ALIGN function_align
.main_pass1:
    vpbroadcastd         m4, [o(pw_2896x8)]
    pmulhrsw             m3, m4, [cq+64*0]
    pmulhrsw             m1, m4, [cq+64*3]
    pmulhrsw             m2, m4, [cq+64*1]
    pmulhrsw             m4, [cq+64*2]
    mova                 m5, [o(int16_perm)]
    kxnorb               k1, k1, k1
    vpblendmd        m0{k1}, m1, m3 ; 0 7
    vmovdqa32        m3{k1}, m1     ; 6 1
    vpblendmd        m1{k1}, m4, m2 ; 2 5
    vmovdqa32        m2{k1}, m4     ; 4 3
    REPX  {vpermb x, m5, x}, m0, m1, m2, m3
    IADST8_1D_PACKED 1
    ret
ALIGN function_align
cglobal_label .main_pass2
    IADST8_1D_PACKED 2
    pxor                 m5, m5
    psubd                m5, m6
    packssdw             m6, m5
    pmulhrsw             m2, m6
    pmulhrsw             m3, m6
    ret

INV_TXFM_16X8_FN flipadst, dct
INV_TXFM_16X8_FN flipadst, adst
INV_TXFM_16X8_FN flipadst, flipadst
INV_TXFM_16X8_FN flipadst, identity

cglobal iflipadst_16x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    call m(iadst_8x16_internal_8bpc).main_pass1
    vpbroadcastd         m7, [o(pw_m16384_16384)]
    psrlq               m10, 20
    jmp m(iadst_16x8_internal_8bpc).pass1_end
.pass2:
    vshufi32x4           m0, m2, m4, q2020
    vshufi32x4           m2, m4, q3131     ; 4 5
    vshufi32x4           m1, m3, m5, q2020
    vshufi32x4           m3, m5, q3131     ; 6 7
    pshufd               m4, m0, q1032     ; 1 0
    pshufd               m5, m1, q1032     ; 3 2
    call m(iadst_16x8_internal_8bpc).main_pass2
    movshdup             m4, [o(permC)]
    pmulhrsw             m5, m6, m0
    pmulhrsw             m0, m6, m1
    psrlq                m1, m4, 12
    psrlq                m4, 8
    mova                 m7, m4
    vpermi2q             m4, m0, m3
    vpermt2q             m0, m1, m3
    vpermi2q             m1, m5, m2
    vpermt2q             m5, m7, m2
    jmp m(idct_16x8_internal_8bpc).end2

INV_TXFM_16X8_FN identity, dct
INV_TXFM_16X8_FN identity, adst
INV_TXFM_16X8_FN identity, flipadst
INV_TXFM_16X8_FN identity, identity

cglobal iidentity_16x8_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    vpbroadcastd         m0, [o(pw_2896x8)]
    pmulhrsw             m3, m0, [cq+64*0]
    pmulhrsw             m4, m0, [cq+64*1]
    pmulhrsw             m5, m0, [cq+64*2]
    pmulhrsw             m0,     [cq+64*3]
    vpbroadcastd         m7, [o(pw_1697x16)]
    vpbroadcastd         m8, [o(pw_16384)]
    shufps               m2, m3, m4, q2020 ; a0 a1 a4 a5 e0 e1 e4 e5
    shufps               m3, m4, q3131     ; a2 a3 a6 a7 e2 e3 e6 e7
    shufps               m4, m5, m0, q2020 ; i0 i1 i4 i5 m0 m1 m4 m5
    shufps               m5, m0, q3131     ; i2 i3 i6 i7 m2 m3 m6 m7
    mova                 m9, [o(int8_permA)]
    pmulhrsw             m0, m7, m2
    pmulhrsw             m1, m7, m3
    pmulhrsw             m6, m7, m4
    pmulhrsw             m7, m5
    REPX   {pmulhrsw x, m8}, m0, m1, m6, m7
    paddsw               m2, m0
    paddsw               m3, m1
    paddsw               m4, m6
    paddsw               m5, m7
    REPX  {vpermb x, m9, x}, m2, m3, m4, m5
    jmp                tx2q
.pass2:
    mova                 m7, [o(permB)]
    vpbroadcastd         m6, [o(pw_4096)]
    vpermq               m0, m7, m2
    vpermq               m4, m7, m4
    vpermq               m1, m7, m3
    vpermq               m5, m7, m5
    jmp m(idct_16x8_internal_8bpc).end

%macro INV_TXFM_16X16_FN 2 ; type1, type2
    INV_TXFM_FN          %1, %2, 16x16
%ifidn %1_%2, dct_dct
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 16
    imul                r6d, 181
    add                 r6d, 128+512
    sar                 r6d, 8+2
    jmp m(inv_txfm_add_dct_dct_16x8_8bpc).dconly3
%endif
%endmacro

INV_TXFM_16X16_FN dct, dct
INV_TXFM_16X16_FN dct, identity
INV_TXFM_16X16_FN dct, adst
INV_TXFM_16X16_FN dct, flipadst

cglobal idct_16x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m7, [o(permB)]
    vpermq               m0, m7, [cq+64*0]
    vpermq               m1, m7, [cq+64*1]
    vpermq               m2, m7, [cq+64*2]
    vpermq               m3, m7, [cq+64*3]
    vpermq               m4, m7, [cq+64*4]
    vpermq               m5, m7, [cq+64*5]
    vpermq               m6, m7, [cq+64*6]
    vpermq               m7, m7, [cq+64*7]
    call .main
    vbroadcasti32x4     m12, [o(int_shuf1)]
    vbroadcasti32x4     m11, [o(int_shuf2)]
    vpbroadcastd        m13, [o(pw_8192)]
    pshufb               m0, m12
    pshufb               m8, m1, m11
    pshufb               m2, m12
    pshufb               m9, m3, m11
    pshufb               m4, m12
    pshufb              m10, m5, m11
    pshufb               m6, m12
    pshufb              m11, m7, m11
    REPX  {pmulhrsw x, m13}, m0, m8, m2, m9, m4, m10, m6, m11
    punpckhdq            m1, m0, m8
    punpckldq            m0, m8
    punpckhdq            m3, m2, m9
    punpckldq            m2, m9
    punpckhdq            m5, m4, m10
    punpckldq            m4, m10
    punpckhdq            m7, m6, m11
    punpckldq            m6, m11
    jmp                tx2q
.pass2:
    vshufi32x4           m8, m4, m6, q3232 ; i8 ic m8 mc
    vinserti32x8         m4, ym6, 1        ; i0 i4 m0 m4
    vshufi32x4           m6, m0, m2, q3232 ; a8 ac e8 ec
    vinserti32x8         m0, ym2, 1        ; a0 a4 e0 e4
    vshufi32x4           m9, m5, m7, q3232 ; ia ie ma me
    vinserti32x8         m5, ym7, 1        ; i2 i6 m2 m6
    vshufi32x4           m7, m1, m3, q3232 ; aa ae ea ee
    vinserti32x8         m1, ym3, 1        ; a2 a6 e2 e6
    vshufi32x4           m2, m0, m4, q3131 ;  4  5
    vshufi32x4           m0, m4, q2020     ;  0  1
    vshufi32x4           m4, m6, m8, q2020 ;  8  9
    vshufi32x4           m6, m8, q3131     ; 12 13
    vshufi32x4           m3, m1, m5, q3131 ;  6  7
    vshufi32x4           m1, m5, q2020     ;  2  3
    vshufi32x4           m5, m7, m9, q2020 ; 10 11
    vshufi32x4           m7, m9, q3131     ; 14 15
    call .main
    mova                  m8, [o(permD)]
    psrlq                m12, m8, 4
    psrlq                 m9, m8, 8
    psrlq                m13, m8, 12
    mova                 m10, m8
    vpermi2q              m8, m0, m2 ;  0  1  4  5
    vpermt2q              m0, m12, m2
    mova                 m11, m9
    vpermi2q              m9, m1, m3 ;  2  3  6  7
    vpermt2q              m1, m13, m3
    vpermi2q             m10, m4, m6 ;  8  9 12 13
    vpermt2q              m4, m12, m6
    vpermi2q             m11, m5, m7 ; 10 11 14 15
    vpermt2q              m5, m13, m7
.end:
    vpbroadcastd        m12, [o(pw_2048)]
.end2:
    REPX  {pmulhrsw x, m12}, m0, m1, m4, m5
.end3:
    REPX  {pmulhrsw x, m12}, m8, m9, m10, m11
    lea                  r3, [strideq*3]
    lea                  r4, [dstq+strideq*4]
    lea                  r5, [dstq+strideq*8]
    lea                  r6, [r4  +strideq*8]
    mova                xm3, [dstq+strideq*0]
    mova                xm6, [dstq+strideq*2]
    vinserti32x4        ym3, [dstq+strideq*1], 1
    vinserti32x4        ym6, [dstq+r3       ], 1
    vinserti32x4         m3, [r4+strideq*0], 2
    vinserti32x4         m6, [r4+strideq*2], 2
    vinserti32x4         m3, [r4+strideq*1], 3
    vinserti32x4         m6, [r4+r3       ], 3
    mova               xm12, [r5+strideq*0]
    mova               xm13, [r5+strideq*2]
    vinserti32x4       ym12, [r5+strideq*1], 1
    vinserti32x4       ym13, [r5+r3       ], 1
    vinserti32x4        m12, [r6+strideq*0], 2
    vinserti32x4        m13, [r6+strideq*2], 2
    vinserti32x4        m12, [r6+strideq*1], 3
    vinserti32x4        m13, [r6+r3       ], 3
    pxor                 m7, m7
    REPX {mova [cq+64*x], m7}, 0, 1, 2, 3, 4, 5, 6, 7
    punpcklbw            m2, m3, m7
    punpckhbw            m3, m7
    paddw                m0, m2
    paddw                m8, m3
    packuswb             m0, m8
    punpcklbw            m2, m6, m7
    punpckhbw            m6, m7
    paddw                m1, m2
    paddw                m9, m6
    packuswb             m1, m9
    punpcklbw            m2, m12, m7
    punpckhbw           m12, m7
    paddw                m2, m4
    paddw               m10, m12
    packuswb             m2, m10
    punpcklbw            m3, m13, m7
    punpckhbw           m13, m7
    paddw                m3, m5
    paddw               m11, m13
    packuswb             m3, m11
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    mova          [dstq+strideq*2], xm1
    vextracti32x4 [dstq+r3       ], ym1, 1
    vextracti32x4 [r4+strideq*0], m0, 2
    vextracti32x4 [r4+strideq*1], m0, 3
    vextracti32x4 [r4+strideq*2], m1, 2
    vextracti32x4 [r4+r3       ], m1, 3
    mova          [r5+strideq*0], xm2
    vextracti32x4 [r5+strideq*1], ym2, 1
    mova          [r5+strideq*2], xm3
    vextracti32x4 [r5+r3       ], ym3, 1
    vextracti32x4 [r6+strideq*0], m2, 2
    vextracti32x4 [r6+strideq*1], m2, 3
    vextracti32x4 [r6+strideq*2], m3, 2
    vextracti32x4 [r6+r3       ], m3, 3
    RET
ALIGN function_align
cglobal_label .main_fast2 ; bottom three-quarters are zero
    vpbroadcastd        m10, [o(pd_2048)]
    vpbroadcastq        m13, [o(int_mshift)]
    vpcmpub              k7, m13, m10, 6
.main_fast4:
    vpbroadcastd         m2, [o(pw_401_4076x8)]
    vpbroadcastd         m4, [o(pw_m1189_3920x8)]
    vpbroadcastd         m3, [o(pw_799_4017x8)]
    pmulhrsw             m2, m8     ; t8a  t15a
    pmulhrsw             m4, m1     ; t11a t12a
    pmulhrsw             m7, m3     ; t4a  t7a
    pxor                 m6, m6
    psubsw               m0, m2, m4 ; t11a t12a
    paddsw               m8, m2, m4 ; t8a  t15a
    mova                 m1, m7
    jmp .main5
ALIGN function_align
cglobal_label .main_fast ; bottom half is zero
    vpbroadcastd        m10, [o(pd_2048)]
.main_fast3:
    vpbroadcastq        m13, [o(int_mshift)]
    vpcmpub              k7, m13, m10, 6
.main_fast5:
    vpbroadcastd         m2, [o(pw_401_4076x8)]
    vpbroadcastd         m4, [o(pw_m2598_3166x8)]
    vpbroadcastd        m11, [o(pw_1931_3612x8)]
    vpbroadcastd        m12, [o(pw_m1189_3920x8)]
    pmulhrsw             m8, m2  ; t8a  t15a
    vpbroadcastd         m2, [o(pw_799_4017x8)]
    pmulhrsw             m0, m4  ; t9a  t14a
    vpbroadcastd         m4, [o(pw_m2276_3406x8)]
    pmulhrsw             m5, m11 ; t10a t13a
    pmulhrsw             m1, m12 ; t11a t12a
    pmulhrsw             m7, m2  ; t4a  t7a
    pmulhrsw             m3, m4  ; t5a  t6a
    jmp .main4
ALIGN function_align
cglobal_label .main
    IDCT16_1D_PACKED
    ret

INV_TXFM_16X16_FN adst, dct
INV_TXFM_16X16_FN adst, adst
INV_TXFM_16X16_FN adst, flipadst

cglobal iadst_16x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    call .main_pass1
    vpbroadcastd        m10, [o(pw_8192_m8192)]
    punpcklwd            m8, m0, m1 ; b0 d0 b1 d1 b2 d2 b3 d3
    punpckhwd            m0, m1     ; a0 c0 a1 c1 a2 c2 a3 c3
    punpckhwd            m1, m0, m8 ; a2 b2 c2 d2 a3 b3 c3 d3
    punpcklwd            m0, m8     ; a0 b0 c0 d0 a1 b1 c1 d1
    punpcklwd            m8, m2, m3 ; f0 h0 f1 h1 f2 h2 f3 h3
    punpckhwd            m2, m3     ; e0 g0 e1 g1 e2 g2 e3 g3
    punpckhwd            m3, m2, m8 ; e2 f2 g2 h2 e3 f3 g3 h3
    punpcklwd            m2, m8     ; e0 f0 g0 h0 e1 f1 g1 h1
    punpckhwd            m8, m4, m5 ; i0 k0 i1 k1 i2 k2 i3 k3
    punpcklwd            m4, m5     ; j0 l0 j1 l1 j2 l2 j3 l3
    punpckhwd            m5, m4, m8 ; i2 j2 k2 l2 i3 j3 k3 l3
    punpcklwd            m4, m8     ; i0 j0 k0 l0 i1 j1 k1 l1
    punpckhwd            m8, m6, m7 ; m0 o0 m1 o1 m2 o2 m3 o3
    punpcklwd            m6, m7     ; n0 p0 n1 p1 n2 p2 n3 p3
    punpckhwd            m7, m6, m8 ; m2 n2 o2 p2 m3 n3 o3 p3
    punpcklwd            m6, m8     ; m0 n0 o0 p0 m1 n1 o1 p1
.pass1_end:
    REPX  {pmulhrsw x, m10}, m0, m1, m2, m3, m4, m5, m6, m7
    jmp                tx2q
.pass2:
    call .main_pass2
    mova                m10, [o(permD)]
    psrlq                m8, m10, 8
    psrlq               m12, m10, 12
    psrlq               m13, m10, 4
    mova                 m9, m8
    vpermi2q             m8, m0, m2 ;  0  1  4  5
    vpermt2q             m0, m12, m2
    vpermi2q             m9, m1, m3 ;  2  3  6  7
    vpermt2q             m1, m12, m3
    vpbroadcastd        m12, [o(pw_2048)]
    mov                 r3d, 0xff00ff00
    mova                m11, m10
    vpermi2q            m10, m4, m6 ;  8  9 12 13
    vpermt2q             m4, m13, m6
    kmovd                k1, r3d
    vpermi2q            m11, m5, m7 ; 10 11 14 15
    vpermt2q             m5, m13, m7
    pxor                 m7, m7
    vpsubw          m12{k1}, m7, m12
    jmp m(idct_16x16_internal_8bpc).end2
ALIGN function_align
.main_pass1:
    mova                 m4, [o(permB)]
    psrlq                m3, m4, 4
    vpermq               m0, m4, [cq+64*0]
    vpermq               m7, m3, [cq+64*7]
    vpermq               m6, m4, [cq+64*6]
    vpermq               m1, m3, [cq+64*1]
    vpermq               m2, m4, [cq+64*2]
    vpermq               m5, m3, [cq+64*5]
    vpermq               m4, m4, [cq+64*4]
    vpermq               m3, m3, [cq+64*3]
    call .main
    vpbroadcastd        m13, [o(pw_2896_2896)]
    vpbroadcastd        m12, [o(pw_m2896_2896)]
    mova                 m2, m10
    vpdpwssd             m2, m5, m13       ; -out5
    mova                 m8, m10
    vpdpwssd             m8, m11, m13      ;  out4
    mova                 m9, m10
    vpdpwssd             m9, m5, m12       ;  out10
    mova                 m5, m10
    vpdpwssd             m5, m11, m12      ; -out11
    mova                m11, m10
    vpdpwssd            m11, m3, m13       ; -out7
    mova                m14, m10
    vpdpwssd            m14, m4, m13       ;  out6
    mova                m13, m10
    vpdpwssd            m13, m3, m12       ;  out8
    vpdpwssd            m10, m4, [o(pw_2896_m2896)] {1to16} ; -out9
    REPX      {psrad x, 12}, m2, m8, m9, m5, m11, m14, m13, m10
    packssdw             m2, m8            ; -out5   out4
    packssdw             m5, m9, m5        ;  out10 -out11
    packssdw             m3, m11, m14      ; -out7   out6
    packssdw             m4, m13, m10      ;  out8  -out9
    ret
ALIGN function_align
.main_pass2:
    vshufi32x4           m8, m4, m6, q3232 ; i8 ic m8 mc
    vinserti32x8         m4, ym6, 1        ; i0 i4 m0 m4
    vshufi32x4           m6, m0, m2, q3232 ; a8 ac e8 ec
    vinserti32x8         m0, ym2, 1        ; a0 a4 e0 e4
    vshufi32x4           m9, m5, m7, q3232 ; ia ie ma me
    vinserti32x8         m5, ym7, 1        ; i2 i6 m2 m6
    vshufi32x4           m7, m1, m3, q3232 ; aa ae ea ee
    vinserti32x8         m1, ym3, 1        ; a2 a6 e2 e6
    vshufi32x4           m2, m0, m4, q3131 ;  4  5
    vshufi32x4           m0, m4, q2020     ;  0  1
    vshufi32x4           m4, m6, m8, q2020 ;  8  9
    vshufi32x4           m6, m8, q3131     ; 12 13
    vshufi32x4           m3, m1, m5, q3131 ;  6  7
    vshufi32x4           m1, m5, q2020     ;  2  3
    vshufi32x4           m5, m7, m9, q2020 ; 10 11
    vshufi32x4           m7, m9, q3131     ; 14 15
cglobal_label .main_pass2b
    REPX {pshufd x, x, q1032}, m1, m3, m5, m7
    call .main
    vpbroadcastd         m8, [o(pw_2896x8)]
    pshufb               m2, m11, m12
    pshufb               m5, m12
    pshufb               m3, m12
    pshufb               m4, m12
    punpcklqdq           m9, m5, m2        ;  t15a   t7
    punpckhqdq           m5, m2            ;  t14a   t6
    shufps               m2, m3, m4, q1032 ;  t2a    t10
    shufps               m3, m4, q3210     ;  t3a    t11
    psubsw               m4, m2, m3        ;  out8  -out9
    paddsw               m3, m2            ; -out7   out6
    paddsw               m2, m5, m9        ; -out5   out4
    psubsw               m5, m9            ;  out10 -out11
    REPX   {pmulhrsw x, m8}, m2, m3, m4, m5
    ret
ALIGN function_align
.main:
    vpbroadcastd        m10, [o(pd_2048)]
    vpbroadcastq        m13, [o(int_mshift)]
    punpckhwd            m8, m7, m0 ; in14 in1
    punpcklwd            m0, m7     ; in0  in15
    punpcklwd            m7, m6, m1 ; in12 in3
    punpckhwd            m1, m6     ; in2  in13
    punpckhwd            m6, m5, m2 ; in10 in5
    punpcklwd            m2, m5     ; in4  in11
    punpcklwd            m5, m4, m3 ; in8  in7
    punpckhwd            m3, m4     ; in6  in9
    vpcmpub              k7, m13, m10, 6 ; 0x33...
    ITX_MUL2X_PACK        0, 4, 9, 10,  201, 4091, 5 ; t0  t1
    ITX_MUL2X_PACK        1, 4, 9, 10,  995, 3973, 5 ; t2  t3
    ITX_MUL2X_PACK        2, 4, 9, 10, 1751, 3703, 5 ; t4  t5
    ITX_MUL2X_PACK        3, 4, 9, 10, 2440, 3290, 5 ; t6  t7
    ITX_MUL2X_PACK        5, 4, 9, 10, 3035, 2751, 5 ; t8  t9
    ITX_MUL2X_PACK        6, 4, 9, 10, 3513, 2106, 5 ; t10 t11
    ITX_MUL2X_PACK        7, 4, 9, 10, 3857, 1380, 5 ; t12 t13
    ITX_MUL2X_PACK        8, 4, 9, 10, 4052,  601, 5 ; t14 t15
    psubsw               m4, m0, m5 ; t9a  t8a
    paddsw               m0, m5     ; t1a  t0a
    psubsw               m5, m1, m6 ; t11a t10a
    paddsw               m1, m6     ; t3a  t2a
    psubsw               m6, m2, m7 ; t13a t12a
    paddsw               m2, m7     ; t5a  t4a
    psubsw               m7, m3, m8 ; t15a t14a
    paddsw               m3, m8     ; t7a  t6a
    ITX_MUL2X_PACK        4, 8, 9, 10, 799,       4017,        4 ; t8  t9
    ITX_MUL2X_PACK        6, 8, 9, 10, 799_4017,  4017_m799,  52 ; t12 t13
    ITX_MUL2X_PACK        5, 8, 9, 10, 3406,      2276,        4 ; t10 t11
    ITX_MUL2X_PACK        7, 8, 9, 10, 3406_2276, 2276_m3406, 52 ; t14 t15
    psubsw               m8, m1, m3 ; t7   t6
    paddsw               m1, m3     ; t3   t2
    psubsw               m3, m0, m2 ; t5   t4
    paddsw               m0, m2     ; t1   t0
    psubsw               m2, m5, m7 ; t14a t15a
    paddsw               m7, m5     ; t10a t11a
    psubsw               m5, m4, m6 ; t12a t13a
    paddsw               m4, m6     ; t8a  t9a
    ITX_MUL2X_PACK        3, 6, 9, 10, 1567,       3784,        5 ; t5a t4a
    ITX_MUL2X_PACK        8, 6, 9, 10, 3784_m1567, 1567_3784,  52 ; t7a t6a
    ITX_MUL2X_PACK        2, 6, 9, 10, 3784,       1567,        4 ; t15 t14
    ITX_MUL2X_PACK        5, 6, 9, 10, 3784_1567,  1567_m3784, 52 ; t13 t12
    vbroadcasti32x4     m12, [o(deint_shuf)]
    paddsw               m6, m4, m7        ; -out1  out14
    psubsw               m4, m7            ;  t10    t11
    psubsw              m11, m3, m8        ;  t7     t6
    paddsw               m8, m3            ;  out12 -out3
    psubsw               m3, m0, m1        ;  t3a    t2a
    paddsw               m0, m1            ; -out15  out0
    paddsw               m1, m2, m5        ; -out13  out2
    psubsw               m5, m2            ;  t15a   t14a
    pshufb               m0, m12
    pshufb               m6, m12
    pshufb               m8, m12
    pshufb               m1, m12
    shufps               m7, m6, m0, q1032 ;  out14 -out15
    shufps               m0, m6, m0, q3210 ; -out1   out0
    punpcklqdq           m6, m8, m1        ;  out12 -out13
    punpckhqdq           m1, m8, m1        ; -out3   out2
    ret

INV_TXFM_16X16_FN flipadst, dct
INV_TXFM_16X16_FN flipadst, adst
INV_TXFM_16X16_FN flipadst, flipadst

cglobal iflipadst_16x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    call m(iadst_16x16_internal_8bpc).main_pass1
    vpbroadcastd        m10, [o(pw_m8192_8192)]
    punpcklwd            m8, m1, m0 ; m0 o0 m1 o1 m2 o2 m3 o3
    punpckhwd            m9, m1, m0 ; n0 p0 n1 p1 n2 p2 n3 p3
    punpckhwd            m1, m7, m6 ; a0 c0 a1 c1 a2 c2 a3 c3
    punpcklwd            m7, m6     ; b0 d0 b1 d1 b2 d2 b3 d3
    punpcklwd            m0, m1, m7 ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhwd            m1, m7     ; a2 b2 c2 d2 a3 b3 c3 d3
    punpcklwd            m6, m8, m9 ; m0 n0 o0 p0 m1 n1 o1 p1
    punpckhwd            m7, m8, m9 ; m2 n2 o2 p2 m3 n3 o3 p3
    punpcklwd            m8, m3, m2 ; i0 k0 i1 k1 i2 k2 i3 k3
    punpckhwd            m9, m3, m2 ; j0 l0 j1 l1 j2 l2 j3 l3
    punpckhwd            m3, m5, m4 ; e0 g0 e1 g1 e2 g2 e3 g3
    punpcklwd            m5, m4     ; f0 h0 f1 h1 f2 h2 f3 h3
    punpcklwd            m2, m3, m5 ; e0 f0 g0 h0 e1 f1 g1 h1
    punpckhwd            m3, m5     ; e2 f2 g2 h2 e3 f3 g3 h3
    punpcklwd            m4, m8, m9 ; i0 j0 k0 l0 i1 j1 k1 l1
    punpckhwd            m5, m8, m9 ; i2 j2 k2 l2 i3 j3 k3 l3
    jmp m(iadst_16x16_internal_8bpc).pass1_end
.pass2:
    call m(iadst_16x16_internal_8bpc).main_pass2
    mova                m10, [o(permD)]
    psrlq                m8, m10, 8
    psrlq               m12, m10, 12
    psrlq               m13, m10, 4
    mova                 m9, m8
    vpermi2q             m8, m7, m5 ;  0  1  4  5
    vpermt2q             m7, m12, m5
    vpermi2q             m9, m6, m4 ;  2  3  6  7
    vpermt2q             m6, m12, m4
    vpbroadcastd        m12, [o(pw_2048)]
    mov                 r3d, 0x00ff00ff
    mova                m11, m10
    vpermi2q            m10, m3, m1 ;  8  9 12 13
    vpermt2q             m3, m13, m1
    kmovd                k1, r3d
    vpermi2q            m11, m2, m0 ; 10 11 14 15
    vpermt2q             m2, m13, m0
    pxor                 m0, m0
    vpsubw          m12{k1}, m0, m12
    pmulhrsw             m0, m7, m12
    pmulhrsw             m1, m6, m12
    pmulhrsw             m4, m3, m12
    pmulhrsw             m5, m2, m12
    jmp m(idct_16x16_internal_8bpc).end3

INV_TXFM_16X16_FN identity, dct
INV_TXFM_16X16_FN identity, identity

cglobal iidentity_16x16_internal_8bpc, 0, 6, 0, dst, stride, c, eob, tx2
    mova                 m8, [o(int16_perm)]
    vpermb               m1, m8, [cq+64*0] ; a0 b0 a1 b1 a2 b2 a3 b3
    vpermb               m2, m8, [cq+64*1] ; c0 d0 c1 d1 c2 d2 c3 d3
    vpbroadcastd         m0, [o(pw_1697x16)]
    vpermb               m3, m8, [cq+64*2] ; e0 f0 e1 f1 e2 f2 e3 f3
    vpermb               m4, m8, [cq+64*3] ; g0 h0 g1 h1 g2 h2 g3 h3
    vpermb               m5, m8, [cq+64*4] ; i0 j0 i1 j1 i2 j2 i3 j3
    vpermb               m6, m8, [cq+64*5] ; k0 l0 k1 l1 k2 l2 k3 l3
    vpermb               m7, m8, [cq+64*6] ; m0 n0 m1 n1 m2 n2 m3 n3
    vpermb               m8, m8, [cq+64*7] ; o0 p0 o1 p1 o2 p2 o3 p3
    pmulhrsw             m9, m0, m1
    pmulhrsw            m10, m0, m2
    pmulhrsw            m11, m0, m3
    pmulhrsw            m12, m0, m4
    pmulhrsw            m13, m0, m5
    pmulhrsw            m14, m0, m6
    pmulhrsw            m15, m0, m7
    pmulhrsw             m0, m8
    REPX       {psraw x, 1}, m9, m10, m11, m12
    pavgw                m1, m9
    pavgw                m2, m10
    pavgw                m3, m11
    pavgw                m4, m12
    REPX       {psraw x, 1}, m13, m14, m15, m0
    pavgw                m5, m13
    pavgw                m6, m14
    pavgw                m7, m15
    pavgw                m8, m0
    punpckldq            m0, m1, m2 ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhdq            m1, m2     ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckldq            m2, m3, m4 ; e0 f0 g0 h0 e1 f1 g1 h1
    punpckhdq            m3, m4     ; e2 f2 g2 h2 e3 f3 g3 h3
    punpckldq            m4, m5, m6 ; i0 j0 k0 l0 i1 j1 k1 l1
    punpckhdq            m5, m6     ; i2 j2 k2 l2 i3 j3 k3 l3
    punpckldq            m6, m7, m8 ; m0 n0 o0 p0 m1 n1 o1 p1
    punpckhdq            m7, m8     ; m2 n2 o2 p2 m3 n3 o3 p3
    jmp                tx2q
ALIGN function_align
.pass2:
    vpbroadcastd        m11, [o(pw_1697x16)]
    pmulhrsw            m12, m11, m0
    pmulhrsw            m13, m11, m1
    pmulhrsw            m14, m11, m2
    pmulhrsw            m15, m11, m3
    pmulhrsw             m8, m11, m4
    pmulhrsw             m9, m11, m5
    pmulhrsw            m10, m11, m6
    pmulhrsw            m11, m7
    REPX      {paddsw x, x}, m0, m1, m2, m3, m4, m5, m6, m7
    paddsw               m0, m12
    paddsw               m1, m13
    paddsw               m2, m14
    paddsw               m3, m15
    paddsw               m8, m4
    movu                 m4, [o(permD+2)]
    paddsw               m9, m5
    paddsw               m6, m10
    paddsw               m7, m11
    psrlq               m12, m4, 4
    mova                 m5, m4
    mova                m10, m4
    mova                m11, m4
    vpermi2q             m4, m0, m2  ;  8  9 12 13
    vpermt2q             m0, m12, m2 ;  0  1  4  5
    vpermi2q             m5, m1, m3  ; 10 11 14 15
    vpermt2q             m1, m12, m3 ;  2  3  6  7
    vpermi2q            m10, m8, m6
    vpermt2q             m8, m12, m6
    vpermi2q            m11, m9, m7
    vpermt2q             m9, m12, m7
    jmp m(idct_16x16_internal_8bpc).end

%macro ITX_UNPACK_MULHRSW 8 ; dst[1-2], src, tmp, coef[1-4]
    vpbroadcastd        m%4, [o(pw_%5_%6x8)]
    punpcklwd           m%1, m%3, m%3
    pmulhrsw            m%1, m%4
    vpbroadcastd        m%4, [o(pw_%7_%8x8)]
    punpckhwd           m%2, m%3, m%3
    pmulhrsw            m%2, m%4
%endmacro

cglobal inv_txfm_add_dct_dct_8x32_8bpc, 4, 4, 0, dst, stride, c, eob
%undef cmp
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    cmp                eobd, 107
    jb .fast
    mova                 m5, [cq+64*5]
    mova                 m3, [cq+64*3]
    mova                 m1, [cq+64*1]
    mova                 m7, [cq+64*7]
    mova                 m2, [cq+64*2]
    mova                 m6, [cq+64*6]
    mova                 m0, [cq+64*0]
    mova                 m4, [cq+64*4]
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    mova                 m8, [o(idct_8x32p)]
    vpbroadcastd         m9, [o(pw_8192)]
    REPX  {vpermb x, m8, x}, m0, m1, m2, m3, m4, m5, m6, m7
    punpckldq            m8, m0, m1 ; ab
    punpckhdq            m0, m1
    punpckldq            m1, m2, m3 ; cd
    punpckhdq            m2, m3
    punpckldq            m3, m4, m5 ; ef
    punpckhdq            m4, m5
    punpckldq            m5, m6, m7 ; gh
    punpckhdq            m6, m7
    REPX   {pmulhrsw x, m9}, m8, m0, m1, m2, m3, m4, m5, m6
    punpcklqdq          m18, m8, m1 ; 30  2    6 26   31  1   23  9
    punpckhqdq          m14, m8, m1 ; 16  0   12 20    3 29   11 21
    punpcklqdq          m21, m0, m2 ; 14 18   22 10   27  5   19 13
    punpckhqdq          m15, m0, m2 ; 18  4   24  8    7 25   15 17
    punpcklqdq          m20, m3, m5
    punpckhqdq          m16, m3, m5
    punpcklqdq          m19, m4, m6
    punpckhqdq          m17, m4, m6
    vinserti32x4        ym8, ym18, xm20, 1
    vshufi32x4          ym1, ym18, ym20, 0x03
    vinserti32x4        ym9, ym14, xm16, 1
    vshufi32x4          ym3, ym14, ym16, 0x03
    vinserti32x4        ym0, ym21, xm19, 1
    vshufi32x4          ym5, ym21, ym19, 0x03
    vinserti32x4        ym7, ym15, xm17, 1
    vshufi32x4          ym6, ym15, ym17, 0x03
    call m(idct_8x16_internal_8bpc).main2
    psrlq               m12, [o(permB)], 60
    vpermt2q            m14, m12, m16
    vpermt2q            m21, m12, m19
    vpermt2q            m15, m12, m17
    vpermi2q            m12, m18, m20
    vextracti32x8      ym16, m14, 1
    vextracti32x8      ym19, m21, 1
    vextracti32x8      ym17, m15, 1
    vextracti32x8      ym20, m12, 1
    call .main2
    jmp .end
.fast: ; right half is zero
    mova                 m0, [o(int16_perm)]
    mova                ym2, [cq+64*4]
    vinserti32x8         m2, [cq+64*0], 1
    mova                ym3, [cq+64*6]
    vinserti32x8         m3, [cq+64*2], 1
    mova                ym4, [cq+64*3]
    vinserti32x8         m4, [cq+64*5], 1
    mova                ym5, [cq+64*7]
    vinserti32x8         m5, [cq+64*1], 1
    REPX  {vpermb x, m0, x}, m2, m3, m4, m5
    call m(idct_16x8_internal_8bpc).main2
    vbroadcasti32x4      m4, [o(int_shuf3)]
    vbroadcasti32x4      m5, [o(int_shuf4)]
    pshufb               m2, m4     ; e0 f0 e2 f2 e1 f1 e3 f3
    pshufb               m3, m5     ; g0 h0 g2 h2 g1 h1 g3 h3
    pshufb               m0, m4     ; a0 b0 a2 b2 a1 b1 a3 b3
    pshufb               m1, m5     ; c0 d0 c2 d2 c1 d1 c3 d3
    vpbroadcastd         m4, [o(pw_8192)]
    psrlq                m5, [o(permB)], 60
    punpckldq            m6, m2, m3 ; e0 f0 g0 h0 e2 f2 g2 h2
    punpckhdq           m17, m2, m3 ; e1 f1 g1 h1 e3 f3 g3 h3
    punpckldq            m2, m0, m1 ; a0 b0 c0 d0 a2 b2 c2 d2
    punpckhdq           m16, m0, m1 ; a1 b1 c1 d1 a3 b3 c3 d3
    REPX   {pmulhrsw x, m4}, m6, m17, m2, m16
    vinserti32x4        ym0, ym2, xm6, 1      ;  0  2
    vshufi32x4          ym1, ym2, ym6, 0x03   ;  4  6
    vinserti32x4       ym14, ym16, xm17, 1    ;  1  3
    vshufi32x4         ym15, ym16, ym17, 0x03 ;  5  7
    vpermt2q             m2, m5, m6           ;  8 10
    vpermt2q            m16, m5, m17          ;  9 11
    vextracti32x8       ym3, m2, 1            ; 12 14
    vextracti32x8      ym17, m16, 1           ; 13 15
    call m(idct_8x16_internal_8bpc).main_fast
    call .main_fast
.end:
    vpbroadcastd        ym8, strided
    pmulld              ym8, [o(gather8d)]
    call .main_end
    lea                  r3, [dstq+strideq*4]
    kxnorb               k1, k1, k1
    lea                  r4, [dstq+strideq*8]
    pxor                 m9, m9
    lea                  r1, [r3+strideq*8]
    kmovb                k2, k1
    vpgatherdq      m12{k1}, [r0+ym8]
    kmovb                k1, k2
    vpgatherdq      m13{k2}, [r3+ym8]
    kmovb                k2, k1
    vpgatherdq      m14{k1}, [r4+ym8]
    kmovb                k1, k2
    vpgatherdq      m15{k2}, [r1+ym8]
    REPX  {pmulhrsw x, m10}, m0, m1, m2, m3, m4, m5, m6, m7
    REPX {mova [cq+64*x], m9}, 0, 1, 2, 3, 4, 5, 6, 7
    punpcklbw           m11, m12, m9
    punpckhbw           m12, m9
    paddw                m0, m11
    paddw                m1, m12
    packuswb             m0, m1
    kmovb                k2, k1
    vpscatterdq [r0+ym8]{k1}, m0
    punpcklbw           m12, m13, m9
    punpckhbw           m13, m9
    paddw                m2, m12
    paddw                m3, m13
    packuswb             m2, m3
    kmovb                k1, k2
    vpscatterdq [r3+ym8]{k2}, m2
    punpcklbw           m13, m14, m9
    punpckhbw           m14, m9
    paddw                m4, m13
    paddw                m5, m14
    packuswb             m4, m5
    kmovb                k2, k1
    vpscatterdq [r4+ym8]{k1}, m4
    punpcklbw           m14, m15, m9
    punpckhbw           m15, m9
    paddw                m6, m14
    paddw                m7, m15
    packuswb             m6, m7
    vpscatterdq [r1+ym8]{k2}, m6
    RET
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 32
    imul                r6d, 181
    add                 r6d, 128+512
    sar                 r6d, 8+2
    jmp m(inv_txfm_add_dct_dct_8x8_8bpc).dconly2
INIT_YMM avx512icl
ALIGN function_align
cglobal_label .main_fast2 ; bottom three-quarters are zero
    ITX_UNPACK_MULHRSW   12, 14, 14, 8,  201, 4091,  m601, 4052 ; t16a, t31a, t23a, t24a
    ITX_UNPACK_MULHRSW   21, 20, 15, 8,  995, 3973, m1380, 3857 ; t20a, t27a, t19a, t28a
    mova                m11, m12
    mova                m17, m20
    mova                m15, m21
    mova                m16, m14
    jmp .main4
ALIGN function_align
cglobal_label .main_fast ; bottom half is zero
    ITX_UNPACK_MULHRSW   12, 14, 14, 8,  201, 4091,  m601, 4052 ; t16a, t31a, t23a, t24a
    ITX_UNPACK_MULHRSW   21, 15, 15, 8,  995, 3973, m1380, 3857 ; t20a, t27a, t19a, t28a
    ITX_UNPACK_MULHRSW   20, 16, 16, 8, 1751, 3703, m2106, 3513 ; t18a, t29a, t21a, t26a
    ITX_UNPACK_MULHRSW   19, 17, 17, 8, 2440, 3290, m2751, 3035 ; t22a, t25a, t17a, t30a
    jmp .main3
ALIGN function_align
cglobal_label .main
    punpcklwd           m12, m21, m14 ; in31 in1
    punpckhwd           m14, m21      ; in3  in29
    punpcklwd           m21, m20, m15 ; in27 in5
    punpckhwd           m15, m20      ; in7  in25
    punpcklwd           m20, m19, m16 ; in23 in9
    punpckhwd           m16, m19      ; in11 in21
    punpcklwd           m19, m18, m17 ; in19 in13
    punpckhwd           m17, m18      ; in15 in17
.main2:
    ITX_MUL2X_PACK       12, 8, 9, 10,  201, 4091, 5 ; t16a, t31a
    ITX_MUL2X_PACK       14, 8, 9, 10, 4052,  601, 5 ; t23a, t24a
    ITX_MUL2X_PACK       21, 8, 9, 10,  995, 3973, 5 ; t20a, t27a
    ITX_MUL2X_PACK       15, 8, 9, 10, 3857, 1380, 5 ; t19a, t28a
    ITX_MUL2X_PACK       20, 8, 9, 10, 1751, 3703, 5 ; t18a, t29a
    ITX_MUL2X_PACK       16, 8, 9, 10, 3513, 2106, 5 ; t21a, t26a
    ITX_MUL2X_PACK       19, 8, 9, 10, 2440, 3290, 5 ; t22a, t25a
    ITX_MUL2X_PACK       17, 8, 9, 10, 3035, 2751, 5 ; t17a, t30a
.main3:
    psubsw              m11, m12, m17 ; t17 t30
    paddsw              m12, m17      ; t16 t31
    psubsw              m17, m15, m20 ; t18 t29
    paddsw              m20, m15      ; t19 t28
    psubsw              m15, m21, m16 ; t21 t26
    paddsw              m21, m16      ; t20 t27
    psubsw              m16, m14, m19 ; t22 t25
    paddsw              m14, m19      ; t23 t24
.main4:
    ITX_MUL2X_PACK       11, 18, 19, 10,   799, 4017, 5 ; t17a t30a
    ITX_MUL2X_PACK       17, 18, 19, 10, m4017,  799, 5 ; t18a t29a
    ITX_MUL2X_PACK       15, 18, 19, 10,  3406, 2276, 5 ; t21a t26a
    ITX_MUL2X_PACK       16, 18, 19, 10, m2276, 3406, 5 ; t22a t25a
    vpbroadcastd         m8, [o(pw_m3784_1567)]
    psubsw              m19, m12, m20 ; t19a t28a
    paddsw              m20, m12      ; t16a t31a
    psubsw              m12, m14, m21 ; t20a t27a
    paddsw              m14, m21      ; t23a t24a
    psubsw              m21, m11, m17 ; t18  t29
    paddsw              m11, m17      ; t17  t30
    psubsw              m17, m16, m15 ; t21  t26
    paddsw              m16, m15      ; t22  t25
    ITX_MUL2X_PACK       21, 18, 15, 10, 1567_3784, 8,   20 ; t18a t29a
    ITX_MUL2X_PACK       19, 18, 15, 10, 1567_3784, 8,   20 ; t19  t28
    ITX_MUL2X_PACK       12, 18, 15, 10, 8, m1567_m3784, 36 ; t20  t27
    ITX_MUL2X_PACK       17, 18, 15, 10, 8, m1567_m3784, 36 ; t21a t26a
    vbroadcasti32x4     m18, [o(deint_shuf)]
    vpbroadcastd         m8, [o(pw_m2896_2896)]
    vpbroadcastd         m9, [o(pw_2896_2896)]
    psubsw              m15, m20, m14 ; t23  t24
    paddsw              m20, m14      ; t16  t31
    psubsw              m14, m11, m16 ; t22a t25a
    paddsw              m11, m16      ; t17a t30a
    psubsw              m16, m21, m17 ; t21  t26
    paddsw              m21, m17      ; t18  t29
    psubsw              m17, m19, m12 ; t20a t27a
    paddsw              m19, m12      ; t19a t28a
    REPX    {pshufb x, m18}, m20, m11, m21, m19
    ITX_MUL2X_PACK       15, 18, 12, 10, 8, 9, 8 ; t23a t22a
    ITX_MUL2X_PACK       14, 13, 15, 10, 8, 9, 8 ; t22  t25
    packssdw            m18, m13      ; t23a t22
    packssdw            m12, m15      ; t24a t25
    ITX_MUL2X_PACK       16, 13, 15, 10, 8, 9, 8 ; t21a t26a
    ITX_MUL2X_PACK       17, 16, 14, 10, 8, 9, 8 ; t20  t27
    packssdw            m16, m13      ; t20  t21a
    packssdw            m14, m15      ; t27  t26a
    punpcklqdq          m13, m19, m21 ; t19a t18
    punpckhqdq          m19, m21      ; t28a t29
    punpcklqdq          m21, m20, m11 ; t16  t17a
    punpckhqdq          m20, m11      ; t31  t30a
INIT_ZMM avx512icl
    mova                m15, [o(permA)]
    ret
cglobal_label .main_end
    vpbroadcastd        m10, [o(pw_2048)]
    vpermt2q             m0, m15, m1  ; t0   t1   t2   t3
    vpermt2q            m20, m15, m19 ; t31  t30a t29  t28a
    vpermt2q             m2, m15, m3  ; t4   t5   t6   t7
    vpermt2q            m14, m15, m12 ; t27  t26a t25  t24a
    vpermt2q             m4, m15, m5  ; t8   t9   t10  t11
    vpermt2q            m18, m15, m16 ; t23a t22  t21a t20
    vpermt2q             m6, m15, m7  ; t12  t13  t14  t15
    vpermt2q            m13, m15, m21 ; t19a t18  t17a t16
    psubsw               m7, m0, m20  ; out31 out30 out29 out28
    paddsw               m0, m20      ; out0  out1  out2  out3
    psubsw               m5, m2, m14  ; out27 out26 out25 out24
    paddsw               m2, m14      ; out4  out5  out6  out7
    psubsw               m3, m4, m18  ; out23 out22 out21 out20
    paddsw               m4, m18      ; out8  out9  out10 out11
    psubsw               m1, m6, m13  ; out19 out18 out17 out16
    paddsw               m6, m13      ; out12 out13 out14 out15
    vzeroupper
    ret

%macro LOAD_PACKED_16X2 3 ; dst, row[1-2]
    vbroadcasti32x4    ym%1, [cq+16*%2]
    vbroadcasti32x4     ym8, [cq+16*%3]
    shufpd             ym%1, ym8, 0x0c
%endmacro

cglobal inv_txfm_add_dct_dct_32x8_8bpc, 4, 4, 0, dst, stride, c, eob
%undef cmp
    test               eobd, eobd
    jz .dconly
    lea                  r5, [o_base]
    LOAD_PACKED_16X2      0,  0,  2 ; in0  in2
    LOAD_PACKED_16X2      1,  4,  6 ; in4  in6
    LOAD_PACKED_16X2      2,  8, 10 ; in8  in10
    LOAD_PACKED_16X2      3, 12, 14 ; in12 in14
    LOAD_PACKED_16X2     14,  1,  3 ; in1  in3
    LOAD_PACKED_16X2     15,  5,  7 ; in5  in7
    LOAD_PACKED_16X2     16,  9, 11 ; in9  in11
    LOAD_PACKED_16X2     17, 13, 15 ; in13 in15
    pxor                 m4, m4
    REPX {mova [cq+64*x], m4}, 0, 1, 2, 3
    cmp                eobd, 107
    jb .fast
    LOAD_PACKED_16X2      4, 16, 18 ; in16 in18
    LOAD_PACKED_16X2      5, 20, 22 ; in20 in22
    LOAD_PACKED_16X2      6, 24, 26 ; in24 in26
    LOAD_PACKED_16X2      7, 28, 30 ; in28 in30
    call m(idct_8x16_internal_8bpc).main
    LOAD_PACKED_16X2     18, 19, 17 ; in19 in17
    LOAD_PACKED_16X2     19, 23, 21 ; in23 in21
    LOAD_PACKED_16X2     20, 27, 25 ; in27 in25
    LOAD_PACKED_16X2     21, 31, 29 ; in31 in29
    pxor                 m8, m8
    REPX {mova [cq+64*x], m8}, 4, 5, 6, 7
    call m(inv_txfm_add_dct_dct_8x32_8bpc).main
    jmp .pass2
.fast: ; bottom half is zero
    mova                ym5, ym4
    mova                ym6, ym4
    mova                ym7, ym4
    call m(idct_8x16_internal_8bpc).main
    call m(inv_txfm_add_dct_dct_8x32_8bpc).main_fast
.pass2:
    vpbroadcastd        m10, [o(pw_8192)]
    vpermt2q             m0, m15, m4       ; t0   t1   t9   t8
    vpermt2q            m20, m15, m18      ; t31  t30a t23a t22
    vpermt2q             m3, m15, m7       ; t7   t6   t14  t15
    vpermt2q            m12, m15, m21      ; t25  t24a t17a t16
    vpermt2q             m2, m15, m6       ; t4   t5   t13  t12
    vpermt2q            m14, m15, m13      ; t23a t22  t21a t20
    vpermt2q             m1, m15, m5       ; t3   t2   t10  t11
    vpermt2q            m19, m15, m16      ; t27  t26a t19a t18
    psubsw               m8, m0, m20       ; out31 out30 out22 out23
    paddsw               m0, m20           ; out0  out1  out9  out8
    paddsw               m6, m3, m12       ; out7  out6  out14 out15
    psubsw               m3, m12           ; out24 out25 out17 out16
    psubsw               m5, m2, m14       ; out27 out26 out18 out19
    paddsw               m4, m2, m14       ; out4  out5  out13 out12
    psubsw               m7, m1, m19       ; out28 out29 out21 out20
    paddsw               m2, m1, m19       ; out3  out2  out10 out11
    vzeroupper
    vshufi32x4           m1, m0, m3, q1221 ; out1  out9  out17 out25
    vshufi32x4           m0, m3, q0330     ; out0  out8  out16 out24
    vshufi32x4           m3, m2, m5, q0330 ; out3  out11 out19 out27
    vshufi32x4           m2, m5, q1221     ; out2  out10 out18 out26
    vshufi32x4           m5, m4, m7, q1221 ; out5  out13 out21 out29
    vshufi32x4           m4, m7, q0330     ; out4  out12 out20 out28
    vshufi32x4           m7, m6, m8, q0330 ; out7  out15 out23 out31
    vshufi32x4           m6, m8, q1221     ; out6  out14 out22 out30
    REPX  {pmulhrsw x, m10}, m0, m1, m2, m3, m4, m5, m6, m7
    call m(inv_txfm_add_dct_dct_64x32_8bpc).transpose_8x8
    call .main
    vpbroadcastd         m8, [o(pw_2048)]
    REPX   {pmulhrsw x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    lea                  r2, [strideq*3]
    lea                  r3, [dstq+strideq*4]
    movshdup            m12, [o(permD)]
    pmovzxbw             m8, [dstq+strideq*0]
    pmovzxbw             m9, [dstq+strideq*1]
    pmovzxbw            m10, [dstq+strideq*2]
    pmovzxbw            m11, [dstq+r2       ]
    paddw                m0, m8
    paddw                m1, m9
    paddw                m2, m10
    paddw                m3, m11
    pmovzxbw             m8, [r3+strideq*0]
    pmovzxbw             m9, [r3+strideq*1]
    pmovzxbw            m10, [r3+strideq*2]
    pmovzxbw            m11, [r3+r2       ]
    paddw                m4, m8
    paddw                m5, m9
    paddw                m6, m10
    paddw                m7, m11
    packuswb             m0, m1
    packuswb             m2, m3
    vpermq               m0, m12, m0
    vpermq               m2, m12, m2
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym2
    vextracti32x8 [dstq+r2       ], m2, 1
    packuswb             m4, m5
    packuswb             m6, m7
    vpermq               m4, m12, m4
    vpermq               m6, m12, m6
    mova          [r3+strideq*0], ym4
    vextracti32x8 [r3+strideq*1], m4, 1
    mova          [r3+strideq*2], ym6
    vextracti32x8 [r3+r2       ], m6, 1
    RET
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 8
.dconly2:
    imul                r6d, 181
    add                 r6d, 128+512
    sar                 r6d, 8+2
.dconly3:
    imul                r6d, 181
    add                 r6d, 128+2048
    sar                 r6d, 8+4
    pxor                 m2, m2
    vpbroadcastw         m3, r6d
.dconly_loop:
    mova                ym1, [dstq+strideq*0]
    vinserti32x8         m1, [dstq+strideq*1], 1
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    paddw                m0, m3
    paddw                m1, m3
    packuswb             m0, m1
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                 r3d, 2
    jg .dconly_loop
    RET
ALIGN function_align
cglobal_label .main
    vpbroadcastd       m10, [o(pd_2048)]
.main2:
    ITX_MULSUB_2W        5, 3, 8, 9, 10, 3406, 2276 ; t5a, t6a
    ITX_MULSUB_2W        1, 7, 8, 9, 10,  799, 4017 ; t4a, t7a
    ITX_MULSUB_2W        2, 6, 8, 9, 10, 1567, 3784 ; t2, t3
    vpbroadcastd       m11, [o(pw_2896_2896)]
    vpbroadcastd       m12, [o(pw_m2896_2896)]
    ITX_MULSUB_2W        0, 4, 8, 9, 10, 11, 12 ; t1, t0
.main3:
    paddsw              m8, m1, m5 ; t4
    psubsw              m1, m5     ; t5a
    paddsw              m9, m7, m3 ; t7
    psubsw              m7, m3     ; t6a
    ITX_MULSUB_2W        7, 1, 3, 5, 10, 11, 12 ; t5, t6
    psubsw              m5, m0, m2 ; dct4 out2
    paddsw              m2, m0     ; dct4 out1
    paddsw              m0, m4, m6 ; dct4 out0
    psubsw              m4, m6     ; dct4 out3
    psubsw              m6, m2, m1 ; out6
    paddsw              m1, m2     ; out1
    paddsw              m2, m5, m7 ; out2
    psubsw              m5, m7     ; out5
    psubsw              m7, m0, m9 ; out7
    paddsw              m0, m9     ; out0
    paddsw              m3, m4, m8 ; out3
    psubsw              m4, m8     ; out4
    ret

cglobal inv_txfm_add_identity_identity_8x32_8bpc, 3, 5, 0, dst, stride, c
    vpbroadcastd         m7, [pw_5]
    paddsw               m0, m7, [cq+64*0]
    paddsw               m1, m7, [cq+64*1]
    vpbroadcastd        ym9, strided
    paddsw               m2, m7, [cq+64*2]
    paddsw               m3, m7, [cq+64*3]
    paddsw               m4, m7, [cq+64*4]
    paddsw               m5, m7, [cq+64*5]
    paddsw               m6, m7, [cq+64*6]
    paddsw               m7,     [cq+64*7]
    pmulld             ym14, ym9, [pd_0to15]
    lea                  r3, [dstq+strideq*1]
    lea                  r4, [dstq+strideq*2]
    kxnorb               k1, k1, k1
    pxor                m13, m13
    add                  r1, r4 ; dstq+strideq*3
    kmovb                k2, k1
    vpgatherdq       m9{k1}, [r0+ym14*4]
    kmovb                k1, k2
    vpgatherdq      m10{k2}, [r3+ym14*4]
    kmovb                k2, k1
    call m(inv_txfm_add_dct_dct_64x32_8bpc).transpose_8x8
    REPX       {psraw x, 3}, m0, m1, m2, m3, m4, m5, m6, m7
    vpgatherdq      m11{k1}, [r4+ym14*4]
    kmovb                k1, k2
    vpgatherdq      m12{k2}, [r1+ym14*4]
    REPX {mova [cq+64*x], m13}, 0, 1, 2, 3, 4, 5, 6, 7
    punpcklbw            m8, m9, m13  ;  0  8 16 24
    punpckhbw            m9, m13      ;  4 12 20 28
    paddw                m0, m8
    paddw                m4, m9
    packuswb             m0, m4
    kmovb                k2, k1
    vpscatterdq [r0+ym14*4]{k1}, m0
    punpcklbw            m8, m10, m13 ;  1  9 17 25
    punpckhbw           m10, m13      ;  5 13 21 29
    paddw                m1, m8
    paddw                m5, m10
    packuswb             m1, m5
    kmovb                k1, k2
    vpscatterdq [r3+ym14*4]{k2}, m1
    punpcklbw            m8, m11, m13 ;  2 10 18 26
    punpckhbw           m11, m13      ;  6 14 22 30
    paddw                m2, m8
    paddw                m6, m11
    packuswb             m2, m6
    kmovb                k2, k1
    vpscatterdq [r4+ym14*4]{k1}, m2
    punpcklbw            m8, m12, m13 ;  3 11 19 27
    punpckhbw           m12, m13      ;  7 15 23 31
    paddw                m3, m8
    paddw                m7, m12
    packuswb             m3, m7
    vpscatterdq [r1+ym14*4]{k2}, m3
    RET

cglobal inv_txfm_add_identity_identity_32x8_8bpc, 3, 5, 0, dst, stride, c
    vpbroadcastd         m0, [pw_4096]
    pmulhrsw             m3, m0, [cq+64*0]
    pmulhrsw             m4, m0, [cq+64*4]
    pmulhrsw             m6, m0, [cq+64*1]
    pmulhrsw             m5, m0, [cq+64*5]
    pmulhrsw             m7, m0, [cq+64*2]
    pmulhrsw             m2, m0, [cq+64*6]
    pmulhrsw             m8, m0, [cq+64*3]
    pmulhrsw             m0,     [cq+64*7]
    mova                m13, [int8_permA]
    lea                  r3, [strideq*3]
    lea                  r4, [dstq+strideq*4]
    punpckldq            m1, m3, m4
    punpckhdq            m3, m4
    punpckldq            m4, m6, m5
    punpckhdq            m6, m5
    punpckldq            m5, m7, m2
    punpckhdq            m7, m2
    punpckldq            m2, m8, m0
    punpckhdq            m8, m0
    mova                ym9, [dstq+strideq*0]
    vinserti32x8         m9, [dstq+strideq*2], 1
    mova               ym10, [dstq+strideq*1]
    vinserti32x8        m10, [dstq+r3       ], 1
    mova               ym11, [r4+strideq*0]
    vinserti32x8        m11, [r4+strideq*2], 1
    mova               ym12, [r4+strideq*1]
    vinserti32x8        m12, [r4+r3       ], 1
    REPX {vpermb x, m13, x}, m1, m4, m5, m2, m3, m6, m7, m8
    pxor                m13, m13
    REPX {mova [cq+64*x], m13}, 0, 1, 2, 3, 4, 5, 6, 7
    punpcklqdq           m0, m1, m4 ; a0 a2   c0 c2
    punpckhqdq           m1, m4     ; b0 b2   d0 d2
    punpcklqdq           m4, m5, m2 ; a1 a3   c1 c3
    punpckhqdq           m5, m2     ; b1 b3   d1 d3
    punpcklqdq           m2, m3, m6 ; e0 e2   g0 g2
    punpckhqdq           m3, m6     ; f0 f2   h0 h2
    punpcklqdq           m6, m7, m8 ; e1 e3   g1 g3
    punpckhqdq           m7, m8     ; f1 f3   h1 h3
    punpcklbw            m8, m9, m13
    punpckhbw            m9, m13
    paddw                m0, m8
    paddw                m4, m9
    packuswb             m0, m4
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*2], m0, 1
    punpcklbw            m8, m10, m13
    punpckhbw           m10, m13
    paddw                m1, m8
    paddw                m5, m10
    packuswb             m1, m5
    mova          [dstq+strideq*1], ym1
    vextracti32x8 [dstq+r3       ], m1, 1
    punpcklbw            m8, m11, m13
    punpckhbw           m11, m13
    paddw                m2, m8
    paddw                m6, m11
    packuswb             m2, m6
    mova          [r4+strideq*0], ym2
    vextracti32x8 [r4+strideq*2], m2, 1
    punpcklbw            m8, m12, m13
    punpckhbw           m12, m13
    paddw                m3, m8
    paddw                m7, m12
    packuswb             m3, m7
    mova          [r4+strideq*1], ym3
    vextracti32x8 [r4+r3       ], m3, 1
    RET

%macro IDCT_16x32_END 3 ; src[1-2], row
    mova                xm8, [dstq+strideq*0]
    vinserti32x4        ym8, [dstq+strideq*1], 1
    mova                xm9, [dstq+r3       ]
    vinserti32x4        ym9, [dstq+strideq*2], 1
    pmulhrsw            m%1, m10
    pmulhrsw            m%2, m10
    vpermb               m8, m11, m8
    vpermb               m9, m11, m9
    mova   [cq+64*(%3*2+0)], m13
    mova   [cq+64*(%3*2+1)], m13
    paddw                m8, m%1
    paddw                m9, m%2
    packuswb             m8, m9
    vpermd               m8, m12, m8
    mova          [dstq+strideq*0], xm8
    vextracti32x4 [dstq+strideq*1], ym8, 1
    vextracti32x4 [dstq+strideq*2], m8, 2
    vextracti32x4 [dstq+r3       ], m8, 3
%if %1 != 20
    lea                dstq, [dstq+strideq*4]
%endif
%endmacro

cglobal inv_txfm_add_dct_dct_16x32_8bpc, 4, 4, 22, dst, stride, c, eob
%undef cmp
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    vpbroadcastd        m15, [o(pw_2896x8)]
    cmp                eobd, 151
    jb .fast
    pmulhrsw             m5, m15, [cq+64*10]
    pmulhrsw             m3, m15, [cq+64* 6]
    pmulhrsw             m1, m15, [cq+64* 2]
    pmulhrsw             m7, m15, [cq+64*14]
    pmulhrsw             m2, m15, [cq+64* 4]
    pmulhrsw             m6, m15, [cq+64*12]
    pmulhrsw             m0, m15, [cq+64* 0]
    pmulhrsw             m4, m15, [cq+64* 8]
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    pmulhrsw            m14, m15, [cq+64* 1]
    pmulhrsw            m21, m15, [cq+64*15]
    pmulhrsw            m18, m15, [cq+64* 9]
    pmulhrsw            m17, m15, [cq+64* 7]
    pmulhrsw            m16, m15, [cq+64* 5]
    pmulhrsw            m19, m15, [cq+64*11]
    pmulhrsw            m20, m15, [cq+64*13]
    pmulhrsw            m15,      [cq+64* 3]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf
    mova                 m8, [o(idct_16x32p)]
    vpbroadcastd         m9, [o(pw_16384)]
    REPX {vpermb x, m8, x}, m0,  m1,  m2,  m3,  m4,  m5,  m6,  m7, \
                            m14, m15, m16, m17, m18, m19, m20, m21
    punpckldq            m8, m0, m1
    punpckhdq            m0, m1
    punpckldq            m1, m2, m3
    punpckhdq            m2, m3
    REPX   {pmulhrsw x, m9}, m8, m0, m1, m2
    punpckldq            m3, m4, m5
    punpckhdq            m4, m5
    punpckldq            m5, m6, m7
    punpckhdq            m6, m7
    REPX   {pmulhrsw x, m9}, m3, m4, m5, m6
    punpckldq            m7, m14, m15
    punpckhdq           m14, m15
    punpckldq           m15, m16, m17
    punpckhdq           m16, m17
    REPX   {pmulhrsw x, m9}, m7, m14, m15, m16
    punpckldq           m17, m18, m19
    punpckhdq           m18, m19
    punpckldq           m19, m20, m21
    punpckhdq           m20, m21
    REPX   {pmulhrsw x, m9}, m17, m18, m19, m20
    punpcklqdq          m21, m8, m1
    punpckhqdq           m8, m1
    punpcklqdq           m1, m0, m2
    punpckhqdq           m0, m2
    punpcklqdq           m2, m3, m5
    punpckhqdq           m3, m5
    punpcklqdq           m5, m4, m6
    punpckhqdq           m4, m6
    punpcklqdq           m6, m7, m15
    punpckhqdq           m7, m15
    punpcklqdq          m15, m14, m16
    punpckhqdq          m14, m16
    punpcklqdq          m16, m17, m19
    punpckhqdq          m17, m19
    punpcklqdq          m19, m18, m20
    punpckhqdq          m18, m20
    vinserti32x8        m20, m21, ym2, 1
    vshufi32x4          m21, m2, q3232
    vinserti32x8         m2, m8, ym3, 1
    vshufi32x4           m8, m3, q3232
    vinserti32x8         m3, m1, ym5, 1
    vshufi32x4           m1, m5, q3232
    vinserti32x8         m5, m0, ym4, 1
    vshufi32x4           m0, m4, q3232
    vinserti32x8         m4, m6, ym16, 1
    vshufi32x4           m6, m16, q3232
    vinserti32x8        m16, m7, ym17, 1
    vshufi32x4           m7, m17, q3232
    vinserti32x8        m17, m15, ym19, 1
    vshufi32x4          m15, m19, q3232
    vinserti32x8        m19, m14, ym18, 1
    vshufi32x4          m14, m18, q3232
    vshufi32x4          m18, m21, m6, q3131 ; 27  5
    vshufi32x4          m21, m6, q2020      ; 31  1
    vshufi32x4           m6, m8, m7, q2020  ; 24  8
    vshufi32x4           m8, m7, q3131      ; 30  2
    vshufi32x4           m7, m1, m15, q2020 ; 28  4
    vshufi32x4           m1, m15, q3131     ;  6 26
    vshufi32x4          m15, m0, m14, q2020 ;  7 25
    vshufi32x4           m0, m14, q3131     ; 14 18
    vshufi32x4          m14, m20, m4, q2020 ;  3 29
    vshufi32x4          m20, m4, q3131      ; 23  9
    vshufi32x4           m9, m3, m17, q2020 ; 16  0
    vshufi32x4           m3, m17, q3131     ; 12 20
    vshufi32x4          m17, m5, m19, q2020 ; 15 17
    vshufi32x4           m5, m19, q3131     ; 22 10
    vshufi32x4          m19, m2, m16, q2020 ; 19 13
    vshufi32x4          m16, m2, m16, q3131 ; 11 21
    call m(idct_16x16_internal_8bpc).main3
    call .main_oddhalf
    jmp .pass2
.fast: ; right half is zero
    mova                ym8, [cq+64*15]
    vinserti32x8         m8, [cq+64* 1], 1
    mova                 m2, [o(int16_perm)]
    mova                ym9, [cq+64* 8]
    vinserti32x8         m9, [cq+64* 0], 1
    mova                ym0, [cq+64* 7]
    vinserti32x8         m0, [cq+64* 9], 1
    mova                ym7, [cq+64*14]
    vinserti32x8         m7, [cq+64* 2], 1
    mova                ym1, [cq+64* 3]
    vinserti32x8         m1, [cq+64*13], 1
    mova                ym3, [cq+64* 6]
    vinserti32x8         m3, [cq+64*10], 1
    mova                ym5, [cq+64*11]
    vinserti32x8         m5, [cq+64* 5], 1
    mova                ym6, [cq+64*12]
    vinserti32x8         m6, [cq+64* 4], 1
    REPX  {pmulhrsw x, m15}, m8, m9, m0, m7, m1, m3, m5, m6
    REPX  {vpermb x, m2, x}, m8, m9, m0, m7, m1, m3, m5, m6
    call m(idct_16x16_internal_8bpc).main2
    vbroadcasti32x4      m8, [o(int_shuf3)]
    vbroadcasti32x4      m9, [o(int_shuf4)]
    vpbroadcastd        m11, [o(pw_16384)]
    pshufb               m0, m8
    pshufb               m1, m9
    pshufb               m2, m8
    pshufb               m3, m9
    REPX  {pmulhrsw x, m11}, m0, m1, m2, m3
    pshufb               m4, m8
    pshufb               m5, m9
    pshufb               m6, m8
    pshufb               m7, m9
    REPX  {pmulhrsw x, m11}, m4, m5, m6, m7
    punpckhdq           m17, m0, m1
    punpckldq            m0, m1
    punpckhdq           m16, m2, m3
    punpckldq            m2, m3
    punpckhdq           m18, m4, m5
    punpckldq            m4, m5
    punpckhdq            m5, m6, m7
    punpckldq            m6, m7
    vinserti32x8         m1, m0, ym2, 1
    vshufi32x4           m3, m0, m2, q3232
    vinserti32x8         m2, m4, ym6, 1
    vshufi32x4           m4, m6, q3232
    vinserti32x8        m15, m17, ym16, 1
    vshufi32x4          m17, m16, q3232
    vinserti32x8        m16, m18, ym5, 1
    vshufi32x4          m18, m5, q3232
    vshufi32x4           m0, m1, m2, q2020   ;  0  2
    vshufi32x4           m1, m2, q3131       ;  4  6
    vshufi32x4           m2, m3, m4, q2020   ;  8 10
    vshufi32x4           m3, m4, q3131       ; 12 14
    vshufi32x4          m14, m15, m16, q2020 ;  1  3
    vshufi32x4          m15, m16, q3131      ;  5  7
    vshufi32x4          m16, m17, m18, q2020 ;  9 11
    vshufi32x4          m17, m18, q3131      ; 13 15
    pxor                 m6, m6
    punpckhwd            m8, m0, m0
    punpcklwd            m9, m6, m0
    punpckhwd            m0, m3, m3
    punpckhwd            m5, m2, m2
    punpcklwd            m7, m1, m1
    punpckhwd            m1, m1
    punpcklwd            m3, m3
    punpcklwd            m6, m2
    call m(idct_16x16_internal_8bpc).main_fast5
    punpcklwd           m21, m14, m14
    punpckhwd           m14, m14
    punpcklwd           m18, m15, m15
    punpckhwd           m15, m15
    punpcklwd           m20, m16, m16
    punpckhwd           m16, m16
    punpcklwd           m19, m17, m17
    punpckhwd           m17, m17
    call .main_oddhalf_fast
.pass2:
    vpbroadcastd        m10, [o(pw_2048)]
    mova                m11, [o(end_16x32p)]
    lea                  r3, [strideq*3]
    pxor                m13, m13
    psrld               m12, m11, 8
    IDCT_16x32_END        0,  1,  0
    IDCT_16x32_END        2,  3,  1
    IDCT_16x32_END        4,  5,  2
    IDCT_16x32_END        6,  7,  3
    IDCT_16x32_END       14, 15,  4
    IDCT_16x32_END       16, 17,  5
    IDCT_16x32_END       18, 19,  6
    IDCT_16x32_END       20, 21,  7
    RET
ALIGN function_align
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 32
    jmp m(inv_txfm_add_dct_dct_16x8_8bpc).dconly
ALIGN function_align
cglobal_label .main_oddhalf_fast2 ; bottom three-quarters are zero
    vpbroadcastd         m8, [o(pw_201_4091x8)]
    vpbroadcastd        m20, [o(pw_m1380_3857x8)]
    vpbroadcastd         m9, [o(pw_995_3973x8)]
    vpbroadcastd        m16, [o(pw_m601_4052x8)]
    pmulhrsw            m21, m8  ; t16a, t31a
    pmulhrsw            m20, m15 ; t19a, t28a
    pmulhrsw            m18, m9  ; t20a, t27a
    pmulhrsw            m14, m16 ; t23a, t24a
    mova                 m8, m21
    mova                m17, m20
    mova                m15, m18
    mova                m16, m14
    jmp .main3
ALIGN function_align
cglobal_label .main_oddhalf_fast ; bottom half is zero
    vpbroadcastd         m8, [o(pw_201_4091x8)]
    vpbroadcastd         m9, [o(pw_m2751_3035x8)]
    vpbroadcastd        m11, [o(pw_1751_3703x8)]
    vpbroadcastd        m12, [o(pw_m1380_3857x8)]
    pmulhrsw            m21, m8  ; t16a, t31a
    vpbroadcastd         m8, [o(pw_995_3973x8)]
    pmulhrsw            m17, m9  ; t17a, t30a
    vpbroadcastd         m9, [o(pw_m2106_3513x8)]
    pmulhrsw            m20, m11 ; t18a, t29a
    vpbroadcastd        m11, [o(pw_2440_3290x8)]
    pmulhrsw            m15, m12 ; t19a, t28a
    vpbroadcastd        m12, [o(pw_m601_4052x8)]
    pmulhrsw            m18, m8  ; t20a, t27a
    pmulhrsw            m16, m9  ; t21a, t26a
    pmulhrsw            m19, m11 ; t22a, t25a
    pmulhrsw            m14, m12 ; t23a, t24a
    jmp .main2
ALIGN function_align
cglobal_label .main_oddhalf
    ITX_MUL2X_PACK       21, 8, 9, 10,  201, 4091, 5 ; t16a, t31a
    ITX_MUL2X_PACK       17, 8, 9, 10, 3035, 2751, 5 ; t17a, t30a
    ITX_MUL2X_PACK       20, 8, 9, 10, 1751, 3703, 5 ; t18a, t29a
    ITX_MUL2X_PACK       15, 8, 9, 10, 3857, 1380, 5 ; t19a, t28a
    ITX_MUL2X_PACK       18, 8, 9, 10,  995, 3973, 5 ; t20a, t27a
    ITX_MUL2X_PACK       16, 8, 9, 10, 3513, 2106, 5 ; t21a, t26a
    ITX_MUL2X_PACK       19, 8, 9, 10, 2440, 3290, 5 ; t22a, t25a
    ITX_MUL2X_PACK       14, 8, 9, 10, 4052,  601, 5 ; t23a, t24a
.main2:
    psubsw               m8, m21, m17 ; t17 t30
    paddsw              m21, m17      ; t16 t31
    psubsw              m17, m15, m20 ; t18 t29
    paddsw              m20, m15      ; t19 t28
    psubsw              m15, m18, m16 ; t21 t26
    paddsw              m18, m16      ; t20 t27
    psubsw              m16, m14, m19 ; t22 t25
    paddsw              m14, m19      ; t23 t24
.main3:
    ITX_MUL2X_PACK        8, 9, 19, 10,   799, 4017, 5 ; t17a t30a
    ITX_MUL2X_PACK       17, 9, 19, 10, m4017,  799, 5 ; t18a t29a
    ITX_MUL2X_PACK       15, 9, 19, 10,  3406, 2276, 5 ; t21a t26a
    ITX_MUL2X_PACK       16, 9, 19, 10, m2276, 3406, 5 ; t22a t25a
    vpbroadcastd        m11, [o(pw_m3784_1567)]
    psubsw              m19, m21, m20 ; t19a t28a
    paddsw              m21, m20      ; t16a t31a
    psubsw              m20, m14, m18 ; t20a t27a
    paddsw              m14, m18      ; t23a t24a
    psubsw              m18, m8, m17  ; t18  t29
    paddsw               m8, m17      ; t17  t30
    psubsw              m17, m16, m15 ; t21  t26
    paddsw              m15, m16      ; t22  t25
    ITX_MUL2X_PACK       18, 9, 16, 10, 1567_3784, 11,   20 ; t18a t29a
    ITX_MUL2X_PACK       19, 9, 16, 10, 1567_3784, 11,   20 ; t19  t28
    ITX_MUL2X_PACK       20, 9, 16, 10, 11, m1567_m3784, 36 ; t20  t27
    ITX_MUL2X_PACK       17, 9, 16, 10, 11, m1567_m3784, 36 ; t21a t26a
    vbroadcasti32x4      m9, [o(deint_shuf)]
    psubsw              m16, m21, m14 ; t23  t24
    paddsw              m14, m21      ; t16  t31
    psubsw              m21, m8, m15  ; t22a t25a
    paddsw              m15, m8       ; t17a t30a
    psubsw               m8, m18, m17 ; t21  t26
    paddsw              m18, m17      ; t18  t29
    paddsw              m17, m19, m20 ; t19a t28a
    psubsw              m19, m20      ; t20a t27a
    vpbroadcastd        m11, [o(pw_m2896_2896)]
    vpbroadcastd        m12, [o(pw_2896_2896)]
    REPX     {pshufb x, m9}, m14, m15, m18, m17
    mova                 m9, m10
    vpdpwssd             m9, m16, m11
    mova                m20, m10
    vpdpwssd            m20, m21, m11
    psrad                m9, 12
    psrad               m20, 12
    packssdw             m9, m20      ; t23a t22
    mova                m20, m10
    vpdpwssd            m20, m16, m12
    mova                m16, m10
    vpdpwssd            m16, m21, m12
    psrad               m20, 12
    psrad               m16, 12
    packssdw            m16, m20, m16 ; t24a t25
    ITX_MUL2X_PACK        8, 21, 20, 10, 11, 12, 8 ; t21a t26a
    ITX_MUL2X_PACK       19,  8, 11, 10, 11, 12, 8 ; t20  t27
    packssdw            m11, m20      ; t27  t26a
    packssdw             m8, m21      ; t20  t21a
    punpcklqdq          m20, m14, m15 ; t16  t17a
    punpckhqdq          m14, m15      ; t31  t30a
    punpckhqdq          m15, m17, m18 ; t28a t29
    punpcklqdq          m17, m18      ; t19a t18
    psubsw              m21, m0, m14  ; out31 out30
    paddsw               m0, m14      ; out0  out1
    psubsw              m14, m7, m20  ; out16 out17
    paddsw               m7, m20      ; out15 out14
    psubsw              m20, m1, m15  ; out28 out29
    paddsw               m1, m15      ; out3  out2
    psubsw              m15, m6, m17  ; out19 out18
    paddsw               m6, m17      ; out12 out13
    psubsw              m17, m4, m9   ; out23 out22
    paddsw               m4, m9       ; out8  out9
    psubsw              m18, m3, m16  ; out24 out25
    paddsw               m3, m16      ; out7  out6
    psubsw              m16, m5, m8   ; out20 out21
    paddsw               m5, m8       ; out11 out10
    psubsw              m19, m2, m11  ; out27 out26
    paddsw               m2, m11      ; out4  out5
    ret

cglobal inv_txfm_add_dct_dct_32x16_8bpc, 4, 6, 22, dst, stride, c, eob
%undef cmp
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    mova                m21, [o(permB)]
    vpermq               m1, m21, [cq+64* 0] ;  0  1
    vpermq              m14, m21, [cq+64* 1] ;  2  3
    vpermq              m20, m21, [cq+64* 2] ;  4  5
    vpermq              m15, m21, [cq+64* 3] ;  6  7
    vpbroadcastd         m8, [o(pw_2896x8)]
    vpermq               m2, m21, [cq+64* 4] ;  8  9
    vpermq              m16, m21, [cq+64* 5] ; 10 11
    vpermq               m3, m21, [cq+64* 6] ; 12 13
    vpermq              m17, m21, [cq+64* 7] ; 14 15
    REPX   {pmulhrsw x, m8}, m1, m14, m20, m15, m2, m16, m3, m17
    pxor                m12, m12
    REPX {mova [cq+64*x], m12}, 0, 1, 2, 3, 4, 5, 6, 7
    cmp                eobd, 151
    jb .fast
    vpermq               m9, m21, [cq+64* 8] ; 16 17
    vpermq              m19, m21, [cq+64* 9] ; 18 19
    vpermq               m4, m21, [cq+64*10] ; 20 21
    vpermq               m5, m21, [cq+64*11] ; 22 23
    vpermq               m6, m21, [cq+64*12] ; 24 25
    vpermq              m18, m21, [cq+64*13] ; 26 27
    vpermq               m7, m21, [cq+64*14] ; 28 29
    vpermq              m21, m21, [cq+64*15] ; 30 31
    REPX   {pmulhrsw x, m8}, m9, m19, m4, m5, m6, m18, m7, m21
    REPX {mova [cq+64*x], m12}, 8, 9, 10, 11, 12, 13, 14, 15
    punpcklwd            m8, m21, m14 ; 30  2
    punpckhwd           m21, m1       ; 31  1
    punpcklwd            m0, m17, m19 ; 14 18
    punpckhwd           m17, m9       ; 15 17
    punpcklwd            m9, m1       ; 16  0
    punpckhwd           m14, m7       ;  3 29
    punpcklwd            m1, m15, m18 ;  6 26
    punpckhwd           m15, m6       ;  7 25
    punpcklwd            m6, m2       ; 24  8
    punpckhwd           m19, m3       ; 19 13
    punpcklwd            m3, m4       ; 12 20
    punpckhwd           m18, m20      ; 27  5
    punpcklwd            m7, m20      ; 28  4
    punpckhwd           m20, m5, m2   ; 23  9
    punpcklwd            m5, m16      ; 22 10
    punpckhwd           m16, m4       ; 11 21
    call m(idct_16x16_internal_8bpc).main2
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
    jmp .pass2
.fast: ; bottom half zero
    punpcklwd            m8, m14, m14 ;  2
    punpcklwd            m0, m17, m17 ; 14
    punpcklwd            m5, m16, m16 ; 10
    punpcklwd            m9, m12, m1  ; __  0
    punpckhwd           m21, m1, m1   ;  1
    punpcklwd            m1, m15, m15 ;  6
    punpcklwd            m7, m20, m20 ;  4
    punpckhwd           m19, m3, m3   ; 13
    punpcklwd            m3, m3       ; 12
    punpcklwd            m6, m12, m2  ; __  8
    punpckhwd           m18, m20, m20 ;  5
    punpckhwd           m20, m2, m2   ;  9
    call m(idct_16x16_internal_8bpc).main_fast
    punpckhwd           m15, m15      ;  7
    punpckhwd           m14, m14      ;  3
    punpckhwd           m16, m16      ; 11
    punpckhwd           m17, m17      ; 15
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
.pass2:
    vpbroadcastd         m9, [o(pw_16384)]
    call .transpose_round
    vshufi32x4          m16, m14, m2, q3131 ;  5
    vshufi32x4          m14, m2, q2020      ;  1
    vshufi32x4           m2, m0, m3, q3131  ;  4
    vshufi32x4           m0, m3, q2020      ;  0
    vshufi32x4           m3, m1, m18, q3131 ;  6
    vshufi32x4           m1, m18, q2020     ;  2
    vshufi32x4          m18, m20, m6, q2020 ;  9
    vshufi32x4          m20, m6, q3131      ; 13
    vshufi32x4           m6, m21, m4, q3131 ; 12
    vshufi32x4           m4, m21, m4, q2020 ;  8
    vshufi32x4          m21, m19, m7, q3131 ; 15
    vshufi32x4          m19, m7, q2020      ; 11
    vshufi32x4           m7, m5, m15, q3131 ; 14
    vshufi32x4           m5, m15, q2020     ; 10
    vshufi32x4          m15, m17, m9, q2020 ;  3
    vshufi32x4          m17, m9, q3131      ;  7
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main2
    call .main_oddhalf
    vpbroadcastd        m12, [o(pw_2048)]
    movshdup            m13, [o(permD)]
    lea                  r2, [strideq*3]
    pmovzxbw             m8, [dstq+strideq*0]
    pmovzxbw             m9, [dstq+strideq*1]
    pmovzxbw            m10, [dstq+strideq*2]
    pmovzxbw            m11, [dstq+r2       ]
    REPX  {pmulhrsw x, m12}, m0, m1, m2, m3
    lea                  r3, [dstq+strideq*4]
    paddw                m0, m8
    paddw                m1, m9
    paddw                m2, m10
    paddw                m3, m11
    pmovzxbw             m8, [r3+strideq*0]
    pmovzxbw             m9, [r3+strideq*1]
    pmovzxbw            m10, [r3+strideq*2]
    pmovzxbw            m11, [r3+r2       ]
    REPX  {pmulhrsw x, m12}, m4, m5, m6, m7
    lea                  r4, [dstq+strideq*8]
    packuswb             m0, m1
    paddw                m4, m8
    paddw                m5, m9
    packuswb             m2, m3
    paddw                m6, m10
    paddw                m7, m11
    pmovzxbw             m8, [r4+strideq*0]
    pmovzxbw             m9, [r4+strideq*1]
    pmovzxbw            m10, [r4+strideq*2]
    pmovzxbw            m11, [r4+r2       ]
    REPX  {pmulhrsw x, m12}, m14, m15, m16, m17
    lea                  r5, [r3+strideq*8]
    packuswb             m4, m5
    paddw               m14, m8
    paddw               m15, m9
    packuswb             m6, m7
    paddw               m16, m10
    paddw               m17, m11
    pmovzxbw             m8, [r5+strideq*0]
    pmovzxbw             m9, [r5+strideq*1]
    pmovzxbw            m10, [r5+strideq*2]
    pmovzxbw            m11, [r5+r2       ]
    REPX  {pmulhrsw x, m12}, m18, m19, m20, m21
    packuswb            m14, m15
    paddw               m18, m8
    paddw               m19, m9
    packuswb            m16, m17
    paddw               m20, m10
    paddw               m21, m11
    packuswb            m18, m19
    packuswb            m20, m21
    REPX {vpermq x, m13, x}, m0, m2, m4, m6, m14, m16, m18, m20
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym2
    vextracti32x8 [dstq+r2       ], m2, 1
    mova          [r3+strideq*0], ym4
    vextracti32x8 [r3+strideq*1], m4, 1
    mova          [r3+strideq*2], ym6
    vextracti32x8 [r3+r2       ], m6, 1
    mova          [r4+strideq*0], ym14
    vextracti32x8 [r4+strideq*1], m14, 1
    mova          [r4+strideq*2], ym16
    vextracti32x8 [r4+r2       ], m16, 1
    mova          [r5+strideq*0], ym18
    vextracti32x8 [r5+strideq*1], m18, 1
    mova          [r5+strideq*2], ym20
    vextracti32x8 [r5+r2       ], m20, 1
    RET
ALIGN function_align
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 16
    imul                r6d, 181
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    add                 r6d, 128+256
    sar                 r6d, 8+1
    jmp m(inv_txfm_add_dct_dct_32x8_8bpc).dconly3
ALIGN function_align
cglobal_label .main_oddhalf_fast3 ; bottom seven-eights are zero
    vpbroadcastd         m8, [o(pw_2896x8)]
    vpbroadcastd         m4, [o(pw_4076x8)]
    vpbroadcastd         m3, [o(pw_401x8)]
    pmulhrsw             m8, m0  ; t0
    pmulhrsw             m4, m14 ; t15a
    pmulhrsw             m3, m14 ; t8a
    punpcklwd            m9, m3, m4
    punpckhwd            m5, m3, m4
    mova                 m2, m10
    vpdpwssd             m2, m9, [o(pw_m3784_1567)] {bcstd}
    mova                 m1, m10
    vpdpwssd             m1, m5, [o(pw_m3784_1567)] {bcstd}
    mova                 m6, m10
    vpdpwssd             m6, m5, [o(pw_1567_3784)] {bcstd}
    mova                 m5, m10
    vpdpwssd             m5, m9, [o(pw_1567_3784)] {bcstd}
    vpbroadcastd        m11, [o(pw_2896_2896)]
    vpbroadcastd        m12, [o(pw_m2896_2896)]
    psubsw              m21, m8, m4 ; out15
    paddsw               m0, m8, m4 ; out0
    psubsw              m14, m8, m3 ; out8
    paddsw               m7, m8, m3 ; out7
    REPX      {psrad x, 12}, m2, m1, m6, m5
    packssdw             m2, m1     ; t9a
    packssdw             m5, m6     ; t14a
    ITX_MULSUB_2W         4, 3, 16, 17, 10, 11, 12 ; t11,  t12
    psubsw              m20, m8, m5 ; out14
    paddsw               m1, m8, m5 ; out1
    psubsw              m15, m8, m2 ; out9
    paddsw               m6, m8, m2 ; out6
    ITX_MULSUB_2W         5, 2, 16, 17, 10, 11, 12 ; t10a, t13a
    psubsw              m18, m8, m3 ; out12
    paddsw               m3, m8     ; out3
    psubsw              m17, m8, m4 ; out11
    paddsw               m4, m8     ; out4
    psubsw              m19, m8, m2 ; out13
    paddsw               m2, m8     ; out2
    psubsw              m16, m8, m5 ; out10
    paddsw               m5, m8     ; out5
    ret
cglobal_label .main_oddhalf_fast2 ; bottom three-quarters are zero
    vpbroadcastd         m9, [o(pw_2896x8)]
    vpbroadcastd         m2, [o(pw_4017x8)]
    vpbroadcastd         m3, [o(pw_799x8)]
    vpbroadcastd        m18, [o(pw_4076x8)]
    vpbroadcastd        m19, [o(pw_401x8)]
    vpbroadcastd        m20, [o(pw_m1189x8)]
    vpbroadcastd        m16, [o(pw_3920x8)]
    pmulhrsw             m9, m0  ; t0
    pmulhrsw             m2, m1  ; t7a
    pmulhrsw             m1, m3  ; t4a
    pmulhrsw            m18, m14 ; t15a
    pmulhrsw            m14, m19 ; t8a
    pmulhrsw            m20, m15 ; t11a
    pmulhrsw            m15, m16 ; t12a
    psubsw               m7, m9, m2 ; idct8 out7
    paddsw               m0, m9, m2 ; idct8 out0
    psubsw               m4, m9, m1 ; idct8 out4
    paddsw               m3, m9, m1 ; idct8 out3
    ITX_MULSUB_2W         2, 1, 5, 6, 10, 2896, 2896 ; t5, t6
    mova                m21, m18
    mova                m19, m14
    mova                m16, m15
    mova                 m8, m20
    psubsw               m6, m9, m1 ; idct8 out6
    paddsw               m1, m9     ; idct8 out1
    psubsw               m5, m9, m2 ; idct8 out5
    paddsw               m2, m9     ; idct8 out2
    jmp .main3
ALIGN function_align
cglobal_label .main_oddhalf_fast ; bottom half is zero
    vpbroadcastd         m5, [o(pw_m2276x8)]
    vpbroadcastd        m11, [o(pw_3406x8)]
    vpbroadcastd         m7, [o(pw_4017x8)]
    vpbroadcastd        m12, [o(pw_799x8)]
    vpbroadcastd         m6, [o(pw_3784x8)]
    vpbroadcastd        m10, [o(pw_1567x8)]
    vpbroadcastd         m4, [o(pw_2896x8)]
    pmulhrsw             m5, m3  ; t5a
    pmulhrsw             m3, m11 ; t6a
    pmulhrsw             m7, m1  ; t7a
    pmulhrsw             m1, m12 ; t4a
    pmulhrsw             m6, m2  ; t3
    pmulhrsw             m2, m10 ; t2
    pmulhrsw             m4, m0  ; t0
    vpbroadcastd        m11, [o(pw_2896_2896)]
    vpbroadcastd        m12, [o(pw_m2896_2896)]
    vpbroadcastd        m10, [o(pd_2048)]
    mova                 m0, m4  ; t1
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main3
    vpbroadcastd        m21, [o(pw_4076x8)]
    vpbroadcastd         m8, [o(pw_401x8)]
    vpbroadcastd        m18, [o(pw_m2598x8)]
    vpbroadcastd         m9, [o(pw_3166x8)]
    vpbroadcastd        m19, [o(pw_3612x8)]
    vpbroadcastd        m11, [o(pw_1931x8)]
    vpbroadcastd        m20, [o(pw_m1189x8)]
    vpbroadcastd        m12, [o(pw_3920x8)]
    pmulhrsw            m21, m14 ; t15a
    pmulhrsw            m14, m8  ; t8a
    pmulhrsw            m18, m17 ; t9a
    pmulhrsw            m17, m9  ; t14a
    pmulhrsw            m19, m16 ; t13a
    pmulhrsw            m16, m11 ; t10a
    pmulhrsw            m20, m15 ; t11a
    pmulhrsw            m15, m12 ; t12a
    jmp .main2
ALIGN function_align
cglobal_label .main_oddhalf
    ITX_MULSUB_2W        14, 21, 8, 9, 10,  401, 4076 ; t8a,  t15a
    ITX_MULSUB_2W        18, 17, 8, 9, 10, 3166, 2598 ; t9a,  t14a
    ITX_MULSUB_2W        16, 19, 8, 9, 10, 1931, 3612 ; t10a, t13a
    ITX_MULSUB_2W        20, 15, 8, 9, 10, 3920, 1189 ; t11a, t12a
.main2:
    paddsw               m8, m20, m16 ; t11
    psubsw              m20, m16      ; t10
    paddsw              m16, m15, m19 ; t12
    psubsw              m15, m19      ; t13
    psubsw              m19, m14, m18 ; t9
    paddsw              m14, m18      ; t8
    psubsw              m18, m21, m17 ; t14
    paddsw              m21, m17      ; t15
.main3:
    vpbroadcastd        m11, [o(pw_1567_3784)]
    vpbroadcastd        m12, [o(pw_m3784_1567)]
    ITX_MULSUB_2W        18, 19, 9, 17, 10, 11, 12 ; t9a,  t14a
    vpbroadcastd        m11, [o(pw_m1567_m3784)]
    ITX_MULSUB_2W        15, 20, 9, 17, 10, 12, 11 ; t10a, t13a
    vpbroadcastd        m11, [o(pw_2896_2896)]
    vpbroadcastd        m12, [o(pw_m2896_2896)]
    psubsw              m17, m14, m8  ; t11a
    paddsw               m8, m14      ; t8a
    paddsw              m14, m18, m15 ; t9
    psubsw              m18, m15      ; t10
    psubsw              m15, m19, m20 ; t13
    paddsw              m19, m20      ; t14
    paddsw              m20, m21, m16 ; t15a
    psubsw              m16, m21, m16 ; t12a
    ITX_MULSUB_2W        15, 18, 9, 21, 10, 11, 12 ; t10a, t13a
    ITX_MULSUB_2W        16, 17, 9, 21, 10, 11, 12 ; t11,  t12
    psubsw              m21, m0, m20 ; out15
    paddsw               m0, m20     ; out0
    psubsw              m20, m1, m19 ; out14
    paddsw               m1, m19     ; out1
    psubsw              m19, m2, m18 ; out13
    paddsw               m2, m18     ; out2
    psubsw              m18, m3, m17 ; out12
    paddsw               m3, m17     ; out3
    psubsw              m17, m4, m16 ; out11
    paddsw               m4, m16     ; out4
    psubsw              m16, m5, m15 ; out10
    paddsw               m5, m15     ; out5
    psubsw              m15, m6, m14 ; out9
    paddsw               m6, m14     ; out6
    psubsw              m14, m7, m8  ; out8
    paddsw               m7, m8      ; out7
    ret
.transpose_round:
    punpcklwd            m8, m0, m2
    punpckhwd            m0, m2
    punpcklwd            m2, m1, m3
    punpckhwd            m1, m3
    punpcklwd            m3, m4, m6
    punpckhwd            m4, m6
    punpcklwd            m6, m5, m7
    punpckhwd            m5, m7
    punpcklwd            m7, m14, m16
    punpckhwd           m14, m16
    punpcklwd           m16, m15, m17
    punpckhwd           m15, m17
    punpcklwd           m17, m19, m21
    punpckhwd           m19, m21
    punpckhwd           m21, m18, m20
    punpcklwd           m18, m20
    punpcklwd           m20, m8, m1
    punpckhwd            m8, m1
    punpcklwd            m1, m0, m2
    punpckhwd            m0, m2
    punpcklwd            m2, m3, m5
    punpckhwd            m3, m5
    punpcklwd            m5, m4, m6
    punpckhwd            m4, m6
    REPX   {pmulhrsw x, m9}, m20, m8, m1, m0
    punpcklwd            m6, m7, m15
    punpckhwd            m7, m15
    punpcklwd           m15, m14, m16
    punpckhwd           m14, m16
    REPX   {pmulhrsw x, m9}, m2, m3, m5, m4
    punpckhwd           m16, m18, m19
    punpcklwd           m18, m19
    punpcklwd           m19, m21, m17
    punpckhwd           m21, m17
    REPX   {pmulhrsw x, m9}, m6, m7, m15, m14
    punpcklwd           m17, m8, m0         ; a2   a6   aa   ae
    punpckhwd            m8, m0             ; a3   a7   ab   af
    punpcklwd            m0, m20, m1        ; a0   a4   a8   ac
    punpckhwd           m20, m1             ; a1   a5   a9   ad
    REPX   {pmulhrsw x, m9}, m16, m18, m19, m21
    punpcklwd            m1, m2, m5         ; b0   b4   b8   bc
    punpckhwd            m2, m5             ; b1   b5   b9   bd
    punpcklwd            m5, m3, m4         ; b2   b6   ba   be
    punpckhwd            m3, m4             ; b3   b7   bb   bf
    punpcklwd            m4, m6, m15        ; c0   c4   c8   cc
    punpckhwd            m6, m15            ; c1   c5   c9   cd
    punpcklwd           m15, m7, m14        ; c2   c6   ca   ce
    punpckhwd            m7, m14            ; c3   c7   cb   cf
    punpcklwd           m14, m18, m19       ; d0   d4   d8   dc
    punpckhwd           m18, m19            ; d1   d5   d9   dd
    punpcklwd            m9, m16, m21       ; d2   d6   da   de
    punpckhwd           m16, m21            ; d3   d7   db   df
    vshufi32x4          m21, m0, m1, q3232  ; a8   ac   b8   bc
    vinserti32x8         m0, ym1, 1         ; a0   a4   b0   b4
    vinserti32x8         m1, m17, ym5, 1    ; a2   a6   b2   b6
    vshufi32x4           m5, m17, m5, q3232 ; aa   ae   ba   be
    vinserti32x8        m17, m8, ym3, 1     ; a3   a7   b3   b7
    vshufi32x4          m19, m8, m3, q3232  ; ab   af   bb   bf
    vinserti32x8         m3, m4, ym14, 1    ; c0   c4   d0   d4
    vshufi32x4           m4, m14, q3232     ; c8   cc   d8   dc
    vinserti32x8        m14, m20, ym2, 1    ; a1   a5   b1   b5
    vshufi32x4          m20, m2, q3232      ; a9   ad   b9   bd
    vinserti32x8         m2, m6, ym18, 1    ; c1   c5   d1   d5
    vshufi32x4           m6, m18, q3232     ; c9   cd   d9   dd
    vinserti32x8        m18, m15, ym9, 1    ; c2   c6   d2   d6
    vshufi32x4          m15, m9, q3232      ; ca   ce   da   de
    vinserti32x8         m9, m7, ym16, 1    ; c3   c7   d3   d7
    vshufi32x4           m7, m16, q3232     ; cb   cf   db   df
    ret

%macro IDTX_16x32 4 ; src/dst[1-4]
    pmulhrsw            m%1, m15, [cq+64*%1]
    pmulhrsw            m%2, m15, [cq+64*%2]
    pmulhrsw            m%3, m15, [cq+64*%3]
    pmulhrsw            m%4, m15, [cq+64*%4]
    pmulhrsw            m18, m16, m%1
    pmulhrsw            m19, m16, m%2
    pmulhrsw            m20, m16, m%3
    pmulhrsw            m21, m16, m%4
    REPX  {pmulhrsw x, m17}, m18, m19, m20, m21
    paddsw              m%1, m18
    paddsw              m%2, m19
    paddsw              m%3, m20
    paddsw              m%4, m21
%endmacro

%macro IDTX_16x32_STORE 2 ; src[1-2]
    mova               xm17, [dstq+r3*0]
    vinserti128        ym17, [dstq+r3*4], 1
    vinserti32x4        m17, [dstq+r3*8], 2
    vinserti32x4        m17, [dstq+r4*8], 3
    mova   [cq+64*(%1*2+0)], m18
    mova   [cq+64*(%1*2+1)], m18
    punpcklbw           m16, m17, m18
    punpckhbw           m17, m18
    paddw               m16, m%1
    paddw               m17, m%2
    packuswb            m16, m17
    mova          [dstq+r3*0], xm16
    vextracti128  [dstq+r3*4], ym16, 1
    vextracti32x4 [dstq+r3*8], m16, 2
    vextracti32x4 [dstq+r4*8], m16, 3
%if %1 != 7
    add                dstq, strideq
%endif
%endmacro

cglobal inv_txfm_add_identity_identity_16x32_8bpc, 3, 5, 22, dst, stride, c
    vpbroadcastd        m15, [pw_2896x8]
    vpbroadcastd        m16, [pw_1697x16]
    vpbroadcastd        m17, [pw_16384]
    IDTX_16x32            0,  1,  2,  3
    IDTX_16x32            4,  5,  6,  7
    IDTX_16x32            8,  9, 10, 11
    IDTX_16x32           12, 13, 14, 15
    vpbroadcastd        m16, [pw_8192]
    call .transpose_2x8x8_round
    lea                  r3, [strideq*2]
    lea                  r4, [strideq*3]
    pxor                m18, m18
    IDTX_16x32_STORE      0,  8
    IDTX_16x32_STORE      1,  9
    IDTX_16x32_STORE      2, 10
    IDTX_16x32_STORE      3, 11
    IDTX_16x32_STORE      4, 12
    IDTX_16x32_STORE      5, 13
    IDTX_16x32_STORE      6, 14
    IDTX_16x32_STORE      7, 15
    RET
ALIGN function_align
.transpose_2x8x8_round:
    punpckhwd           m17, m4, m5
    punpcklwd            m4, m5
    punpckhwd            m5, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m6, m7
    punpcklwd            m6, m7
    punpckhwd            m7, m2, m3
    punpcklwd            m2, m3
    punpckhdq            m3, m0, m2
    punpckldq            m0, m2
    punpckldq            m2, m4, m6
    punpckhdq            m4, m6
    punpckhdq            m6, m5, m7
    punpckldq            m5, m7
    punpckldq            m7, m17, m1
    punpckhdq           m17, m1
    REPX  {pmulhrsw x, m16}, m0, m2, m3, m4, m5, m7, m6, m17
    punpckhqdq           m1, m0, m2
    punpcklqdq           m0, m2
    punpcklqdq           m2, m3, m4
    punpckhqdq           m3, m4
    punpcklqdq           m4, m5, m7
    punpckhqdq           m5, m7
    punpckhqdq           m7, m6, m17
    punpcklqdq           m6, m17
    punpckhwd           m17, m12, m13
    punpcklwd           m12, m13
    punpckhwd           m13, m8, m9
    punpcklwd            m8, m9
    punpckhwd            m9, m14, m15
    punpcklwd           m14, m15
    punpckhwd           m15, m10, m11
    punpcklwd           m10, m11
    punpckhdq           m11, m8, m10
    punpckldq            m8, m10
    punpckldq           m10, m12, m14
    punpckhdq           m12, m14
    punpckhdq           m14, m13, m15
    punpckldq           m13, m15
    punpckldq           m15, m17, m9
    punpckhdq           m17, m9
    REPX  {pmulhrsw x, m16}, m8, m10, m11, m12, m13, m15, m14, m17
    punpckhqdq           m9, m8, m10
    punpcklqdq           m8, m10
    punpcklqdq          m10, m11, m12
    punpckhqdq          m11, m12
    punpcklqdq          m12, m13, m15
    punpckhqdq          m13, m15
    punpckhqdq          m15, m14, m17
    punpcklqdq          m14, m17
    ret

%macro IDTX_32x16 4 ; dst[1-4]
    pmulhrsw            m%2, m12, [cq+32*(%1+ 0)]
    pmulhrsw            m18, m12, [cq+32*(%1+16)]
    pmulhrsw            m%4, m12, [cq+32*(%3+ 0)]
    pmulhrsw            m19, m12, [cq+32*(%3+16)]
    REPX      {paddsw x, x}, m%2, m18, m%4, m19
    mova                m%1, m14
    vpermi2q            m%1, m%2, m18
    vpermt2q            m%2, m16, m18
%if %3 != 14
    mova                m%3, m14
%endif
    vpermi2q            m%3, m%4, m19
    vpermt2q            m%4, m16, m19
    pmulhrsw            m18, m17, m%1
    pmulhrsw            m19, m17, m%2
    pmulhrsw            m20, m17, m%3
    pmulhrsw            m21, m17, m%4
    REPX      {paddsw x, x}, m%1, m%2, m%3, m%4
    paddsw              m%1, m18
    paddsw              m%2, m19
    paddsw              m%3, m20
    paddsw              m%4, m21
%endmacro

%macro IDTX_32x16_STORE 2-3 0 ; src[1-2], 32x32
    mova               ym19, [dstq+strideq*0]
    vinserti32x8        m19, [dstq+strideq*8], 1
%if %3 == 0
    mova   [cq+64*(%1*2+0)], m20
    mova   [cq+64*(%1*2+1)], m20
%endif
    punpcklbw           m18, m19, m20
    punpckhbw           m19, m20
    paddw               m18, m%1
    paddw               m19, m%2
    packuswb            m18, m19
    mova          [dstq+strideq*0], ym18
    vextracti32x8 [dstq+strideq*8], m18, 1
%if %3 || %1 != 7
    add                dstq, strideq
%endif
%endmacro

cglobal inv_txfm_add_identity_identity_32x16_8bpc, 3, 3, 22, dst, stride, c
    vpbroadcastd        m12, [pw_2896x8]
    movu                m14, [permB+7]
    vpbroadcastd        m17, [pw_1697x16]
    psrlq               m16, m14, 4
    IDTX_32x16            0,  1,  2,  3
    IDTX_32x16            4,  5,  6,  7
    IDTX_32x16            8,  9, 10, 11
    IDTX_32x16           12, 13, 14, 15
    vpbroadcastd        m16, [pw_2048]
    call m(inv_txfm_add_identity_identity_16x32_8bpc).transpose_2x8x8_round
    pxor                m20, m20
    IDTX_32x16_STORE      0,  8
    IDTX_32x16_STORE      1,  9
    IDTX_32x16_STORE      2, 10
    IDTX_32x16_STORE      3, 11
    IDTX_32x16_STORE      4, 12
    IDTX_32x16_STORE      5, 13
    IDTX_32x16_STORE      6, 14
    IDTX_32x16_STORE      7, 15
    RET

%macro IDCT_32x32_END 4 ; src, mem, stride[1-2]
    pmovzxbw            m10, [dstq+%3]
    pmovzxbw            m11, [r3  +%4]
%if %2 < 8
    paddsw               m8, m%2, m%1
    psubsw               m9, m%2, m%1
%else
    mova                 m9, [cq+64*(%2*2-16)]
    paddsw               m8, m9, m%1
    psubsw               m9, m%1
%endif
    pmulhrsw             m8, m12
    pmulhrsw             m9, m12
%if %2 >= 8
%if %2 == 8
    pxor                 m0, m0
%endif
    mova  [cq+64*(%2*2-16)], m0
    mova  [cq+64*(%2*2-15)], m0
%endif
    paddw                m8, m10
    paddw                m9, m11
    packuswb             m8, m9
    vpermq               m8, m13, m8
    mova          [dstq+%3], ym8
    vextracti32x8 [r3  +%4], m8, 1
%if %2 == 3 || %2 == 7 || %2 == 11
    add                dstq, r5
    sub                  r3, r5
%endif
%endmacro

cglobal inv_txfm_add_dct_dct_32x32_8bpc, 4, 6, 0, dst, stride, c, eob
%undef cmp
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    WIN64_SPILL_XMM      30
    cmp                eobd, 136
    jb .fast
    mova                 m5, [cq+64*20]
    mova                 m3, [cq+64*12]
    mova                 m1, [cq+64* 4]
    mova                 m7, [cq+64*28]
    mova                 m2, [cq+64* 8]
    mova                 m6, [cq+64*24]
    mova                 m0, [cq+64* 0]
    mova                 m4, [cq+64*16]
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    mova                m14, [cq+64* 2]
    mova                m21, [cq+64*30]
    mova                m18, [cq+64*18]
    mova                m17, [cq+64*14]
    mova                m16, [cq+64*10]
    mova                m19, [cq+64*22]
    mova                m20, [cq+64*26]
    mova                m15, [cq+64* 6]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf
    mova         [cq+64* 0], m14
    mova         [cq+64* 2], m15
    mova         [cq+64* 4], m16
    mova         [cq+64* 6], m17
    mova         [cq+64* 8], m18
    mova         [cq+64*10], m19
    mova         [cq+64*12], m20
    mova         [cq+64*14], m21
    mova                m22, [cq+64* 1]
    mova                m21, [cq+64*31]
    mova                m14, [cq+64*17]
    mova                m29, [cq+64*15]
    mova                m26, [cq+64* 9]
    mova                m17, [cq+64*23]
    mova                m18, [cq+64*25]
    mova                m25, [cq+64* 7]
    mova                m24, [cq+64* 5]
    mova                m19, [cq+64*27]
    mova                m16, [cq+64*21]
    mova                m27, [cq+64*11]
    mova                m28, [cq+64*13]
    mova                m15, [cq+64*19]
    mova                m20, [cq+64*29]
    mova                m23, [cq+64* 3]
    call .main_oddhalf
    vpbroadcastd        m10, [o(pw_8192)]
    psubsw              m13, m0, m29 ; 31
    paddsw               m0, m29     ;  0
    psubsw              m29, m1, m28 ; 30
    paddsw               m1, m28     ;  1
    psubsw              m28, m2, m27 ; 29
    paddsw               m2, m27     ;  2
    psubsw              m27, m3, m26 ; 28
    paddsw               m3, m26     ;  3
    psubsw              m26, m4, m25 ; 27
    paddsw               m4, m25     ;  4
    psubsw              m25, m5, m24 ; 26
    paddsw               m5, m24     ;  5
    psubsw              m24, m6, m23 ; 25
    paddsw               m6, m23     ;  6
    psubsw              m23, m7, m22 ; 24
    paddsw               m7, m22     ;  7
    pxor                 m9, m9
    punpckhwd            m8, m0, m1 ; a4 b4 a5 b5 a6 b6 a7 b7
    punpcklwd            m0, m1     ; a0 b0 a1 b1 a2 b2 a3 b3
    punpckhwd            m1, m2, m3 ; c4 d4 c5 d5 c6 d6 c7 d7
    punpcklwd            m2, m3     ; c0 d0 c1 d1 c2 d2 c3 d3
    REPX {mova [cq+64*x], m9}, 16, 17, 18, 19
    punpckhwd           m22, m4, m5 ; e4 f4 e5 f5 e6 f6 e7 f7
    punpcklwd            m4, m5     ; e0 f0 e1 f1 e2 f2 e3 f3
    punpckhwd            m5, m6, m7 ; g4 h4 g5 h5 g6 h6 g7 h7
    punpcklwd            m6, m7     ; g0 h0 g1 h1 g2 h2 g3 h3
    REPX {mova [cq+64*x], m9}, 20, 21, 22, 23
    punpckhwd            m3, m23, m24
    punpcklwd           m23, m24
    punpckhwd           m24, m25, m26
    punpcklwd           m25, m26
    REPX {mova [cq+64*x], m9}, 24, 25, 26, 27
    punpckhwd           m26, m27, m28
    punpcklwd           m27, m28
    punpckhwd           m28, m29, m13
    punpcklwd           m29, m13
    REPX {mova [cq+64*x], m9}, 28, 29, 30, 31
    punpckhdq            m7, m0, m2  ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckldq            m0, m2      ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhdq            m2, m4, m6  ; e2 f2 g2 h2 e3 f3 g3 h3
    punpckldq            m4, m6      ; e0 f0 g0 h0 e1 f1 g1 h1
    punpckhdq            m6, m8, m1  ; a6 b6 c6 d6 a7 b7 c7 d7
    punpckldq            m8, m1      ; a4 b4 c4 d4 a5 b5 c5 d5
    punpckhdq            m1, m22, m5 ; e6 f6 g6 h6 e7 f7 g7 h7
    punpckldq           m22, m5      ; e4 f4 g4 h5 e5 f5 g5 h5
    REPX  {pmulhrsw x, m10}, m0, m4, m8, m22
    punpckhdq           m13, m23, m25
    punpckldq           m23, m25
    punpckhdq           m25, m27, m29
    punpckldq           m27, m29
    REPX  {pmulhrsw x, m10}, m13, m23, m25, m27
    punpckhdq            m9, m3, m24
    punpckldq            m3, m24
    punpckhdq           m24, m26, m28
    punpckldq           m26, m28
    punpcklqdq           m5, m23, m27 ; d00 d08 d16 d24
    punpckhqdq          m23, m27      ; d01 d09 d17 d25
    punpckhqdq          m27, m13, m25 ; d03 d11 d19 d27
    punpcklqdq          m13, m25      ; d02 d10 d18 d26
    punpckhqdq          m25, m3, m26  ; d05 d13 d21 d29
    punpcklqdq           m3, m26      ; d04 d12 d20 d28
    punpckhqdq          m26, m9, m24  ; d07 d15 d23 d31
    punpcklqdq           m9, m24      ; d06 d14 d22 d30
    REPX  {pmulhrsw x, m10}, m25, m3, m26
    mova         [cq+64* 9], m23
    mova         [cq+64*11], m27
    mova         [cq+64*13], m25
    mova         [cq+64*15], m26
    punpckhqdq          m24, m8, m22  ; a05 a13 a21 a29
    punpcklqdq           m8, m22      ; a04 a12 a20 a28
    punpckhqdq          m22, m0, m4   ; a01 a09 a17 a25
    punpcklqdq           m0, m4       ; a00 a08 a16 a24
    punpckhqdq          m23, m7, m2   ; a03 a11 a19 a27
    punpcklqdq           m7, m2       ; a02 a10 a18 a26
    punpckhqdq          m25, m6, m1   ; a07 a15 a23 a31
    punpcklqdq           m6, m1       ; a06 a14 a22 a30
    mova                 m2, [cq+64* 0]
    mova                m11, [cq+64* 2]
    mova                m12, [cq+64* 4]
    mova                m29, [cq+64* 6]
    mova                m27, [cq+64* 8]
    mova                m26, [cq+64*10]
    mova                 m4, [cq+64*12]
    mova                m28, [cq+64*14]
    psubsw               m1, m2, m21  ; 23
    paddsw               m2, m21      ;  8
    psubsw              m21, m11, m20 ; 22
    paddsw              m11, m20      ;  9
    psubsw              m20, m12, m19 ; 21
    paddsw              m12, m19      ; 10
    psubsw              m19, m29, m18 ; 20
    paddsw              m29, m18      ; 11
    psubsw              m18, m27, m17 ; 19
    paddsw              m27, m17      ; 12
    psubsw              m17, m26, m16 ; 18
    paddsw              m26, m16      ; 13
    paddsw              m16, m4, m15  ; 14
    psubsw               m4, m15      ; 17
    pmulhrsw            m15, m6, m10
    psubsw               m6, m28, m14 ; 16
    paddsw              m28, m14      ; 15
    pmulhrsw            m14, m7, m10
    punpcklwd            m7, m6, m4
    punpckhwd            m6, m4
    punpckhwd            m4, m17, m18
    punpcklwd           m17, m18
    punpckhwd           m18, m19, m20
    punpcklwd           m19, m20
    punpckhwd           m20, m21, m1
    punpcklwd           m21, m1
    punpckhwd            m1, m2, m11  ; i4 j4 i5 j5 i6 j6 i7 j7
    punpcklwd            m2, m11      ; i0 j1 i1 j1 i2 j2 i3 j3
    punpckhwd           m11, m12, m29 ; k4 l4 k5 l5 k6 l6 k7 l7
    punpcklwd           m12, m29      ; k0 l0 k1 l1 k2 l2 k3 l3
    punpckhwd           m29, m27, m26 ; m4 n4 m5 n5 m6 n6 m7 n7
    punpcklwd           m27, m26      ; m0 n0 m1 n1 m2 n2 m3 n3
    punpckhwd           m26, m16, m28 ; o4 p4 o5 p5 o6 p6 o7 p7
    punpcklwd           m16, m28      ; o0 p0 o1 p1 o2 p2 o3 p3
    pmulhrsw            m23, m10
    pmulhrsw            m25, m10
    punpckhdq           m28, m2, m12  ; i2 j2 k2 l2 i3 j3 k3 l3
    punpckldq            m2, m12      ; i0 j0 k0 l0 i1 j1 k1 l1
    punpckhdq           m12, m27, m16 ; m2 n2 o2 p2 m3 n3 o3 p3
    punpckldq           m27, m16      ; m0 n0 o0 p0 m1 n1 o1 p1
    REPX  {pmulhrsw x, m10}, m28, m2, m12, m27
    punpckhdq           m16, m1, m11  ; i6 j6 k6 l6 i7 j7 k7 l7
    punpckldq            m1, m11      ; i4 j4 k4 l4 i5 j5 k5 l5
    punpckhdq           m11, m29, m26 ; m6 n6 o6 p6 m7 n7 o7 p7
    punpckldq           m29, m26      ; m4 n4 o4 p4 m5 n5 o5 p5
    REPX  {pmulhrsw x, m10}, m16, m1, m11, m29
    punpckhdq           m26, m19, m21
    punpckldq           m19, m21
    punpckhdq           m21, m6, m4
    punpckldq            m6, m4
    REPX  {pmulhrsw x, m10}, m26, m19, m21, m6
    punpckhdq            m4, m18, m20
    punpckldq           m18, m20
    punpckhdq           m20, m7, m17
    punpckldq            m7, m17
    REPX  {pmulhrsw x, m10}, m4, m18, m20, m7
    punpcklqdq          m17, m28, m12 ; b02 b10 b18 b26
    punpckhqdq          m28, m12      ; b03 b11 b19 b27
    punpckhqdq          m12, m2, m27  ; b01 b09 b17 b25
    punpcklqdq           m2, m27      ; b00 b08 b16 b24
    punpckhqdq          m27, m1, m29  ; b05 b13 b21 b29
    punpcklqdq           m1, m29      ; b04 b12 b20 b28
    punpckhqdq          m29, m16, m11 ; b07 b15 b23 b31
    punpcklqdq          m16, m11      ; b06 b14 b22 b30
    mova         [cq+64* 1], m12
    mova         [cq+64* 3], m28
    mova         [cq+64* 5], m27
    mova         [cq+64* 7], m29
    punpckhqdq          m27, m20, m26 ; c03 c11 c19 c27
    punpcklqdq          m20, m26      ; c02 c10 c18 c26
    punpckhqdq          m26, m7, m19  ; c01 c09 c17 c25
    punpcklqdq           m7, m19      ; c00 c08 c16 c24
    punpckhqdq          m28, m6, m18  ; c05 c13 c21 c29
    punpcklqdq           m6, m18      ; c04 c12 c20 c28
    punpckhqdq          m29, m21, m4  ; c07 c15 c23 c31
    punpcklqdq          m21, m4       ; c06 c14 c22 c30
    pmulhrsw            m19, m9, m10
    vshufi32x4           m4, m0, m2, q3232   ; a16 a24 b16 b24
    vinserti32x8         m0, ym2, 1          ; a00 a08 b00 b08
    vshufi32x4           m2, m7, m5, q3232   ; c16 c24 d16 d24
    vinserti32x8         m7, ym5, 1          ; c00 c08 d00 d08
    vshufi32x4           m5, m8, m1, q3232   ; a20 a28 b20 b28
    vinserti32x8         m1, m8, ym1, 1      ; a04 a12 b04 b12
    vshufi32x4           m8, m6, m3, q3232   ; c20 c28 d20 d28
    vinserti32x8         m6, ym3, 1          ; c04 c12 d04 d12
    vshufi32x4           m3, m1, m6, q3131   ; 12
    vshufi32x4           m1, m6, q2020       ;  4
    vshufi32x4           m6, m4, m2, q3131   ; 24
    vshufi32x4           m4, m2, q2020       ; 16
    vshufi32x4           m2, m0, m7, q3131   ;  8
    vshufi32x4           m0, m7, q2020       ;  0
    vshufi32x4           m7, m5, m8, q3131   ; 28
    vshufi32x4           m5, m8, q2020       ; 20
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    vshufi32x4          m18, m14, m17, q3232 ; a18 a26 b18 b26
    vinserti32x8        m14, ym17, 1         ; a02 a10 b02 b10
    vshufi32x4          m17, m20, m13, q3232 ; c18 c26 d18 d26
    vinserti32x8        m20, ym13, 1         ; c02 c10 d02 d10
    vshufi32x4          m13, m21, m19, q3232 ; c22 c30 d22 d30
    vinserti32x8        m21, ym19, 1         ; c06 c14 d06 d14
    vshufi32x4          m19, m15, m16, q3232 ; a22 a30 b22 b30
    vinserti32x8        m15, ym16, 1         ; a06 a14 b06 b14
    vshufi32x4          m16, m14, m20, q3131 ; 10
    vshufi32x4          m14, m20, q2020      ;  2
    vshufi32x4          m20, m18, m17, q3131 ; 26
    vshufi32x4          m18, m17, q2020      ; 18
    vshufi32x4          m17, m15, m21, q3131 ; 14
    vshufi32x4          m15, m21, q2020      ;  6
    vshufi32x4          m21, m19, m13, q3131 ; 30
    vshufi32x4          m19, m13, q2020      ; 22
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf
    mova         [cq+64* 0], m14
    mova         [cq+64* 2], m15
    mova         [cq+64* 4], m16
    mova         [cq+64* 6], m17
    mova         [cq+64* 8], m18
    mova         [cq+64*10], m19
    mova         [cq+64*12], m20
    mova         [cq+64*14], m21
    mova                m15, [cq+64* 1]
    mova                m16, [cq+64* 3]
    mova                m17, [cq+64* 5]
    mova                m19, [cq+64* 7]
    mova                m20, [cq+64* 9]
    mova                m21, [cq+64*11]
    mova                m13, [cq+64*13]
    mova                m18, [cq+64*15]
    vshufi32x4          m14, m22, m15, q3232 ; a17 a25 b17 b25
    vinserti32x8        m22, ym15, 1         ; a01 a09 b01 b09
    vshufi32x4          m15, m23, m16, q3232 ; a19 a27 b19 b27
    vinserti32x8        m23, ym16, 1         ; a03 a11 b03 b11
    vshufi32x4          m16, m24, m17, q3232 ; a21 a29 b21 b29
    vinserti32x8        m24, ym17, 1         ; a05 a13 b05 b13
    vshufi32x4          m17, m25, m19, q3232 ; a23 a31 b23 b31
    vinserti32x8        m25, ym19, 1         ; a07 a15 b07 b15
    vinserti32x8         m8, m26, ym20, 1    ; c01 c09 d01 d09
    vshufi32x4          m26, m20, q3232      ; c17 c25 d17 d25
    vinserti32x8         m9, m27, ym21, 1    ; c03 c11 d03 d11
    vshufi32x4          m27, m21, q3232      ; c19 c27 d19 d27
    vinserti32x8        m11, m28, ym13, 1    ; c05 c13 d05 d13
    vshufi32x4          m28, m13, q3232      ; c21 c29 d21 d29
    vinserti32x8        m12, m29, ym18, 1    ; c07 c15 d07 d15
    vshufi32x4          m29, m18, q3232      ; c23 c31 d23 d31
    vshufi32x4          m18, m14, m26, q3131 ; 25
    vshufi32x4          m14, m26, q2020      ; 17
    vshufi32x4          m19, m15, m27, q3131 ; 27
    vshufi32x4          m15, m27, q2020      ; 19
    vshufi32x4          m20, m16, m28, q3131 ; 29
    vshufi32x4          m16, m28, q2020      ; 21
    vshufi32x4          m21, m17, m29, q3131 ; 31
    vshufi32x4          m17, m29, q2020      ; 23
    vshufi32x4          m26, m22, m8, q3131  ;  9
    vshufi32x4          m22, m8, q2020       ;  1
    vshufi32x4          m27, m23, m9, q3131  ; 11
    vshufi32x4          m23, m9, q2020       ;  3
    vshufi32x4          m28, m24, m11, q3131 ; 13
    vshufi32x4          m24, m11, q2020      ;  5
    vshufi32x4          m29, m25, m12, q3131 ; 15
    vshufi32x4          m25, m12, q2020      ;  7
    call .main_oddhalf
    jmp .end
.fast: ; bottom/right halves are zero
    mova                m14, [o(dup16_perm)]
    pmovzxwd             m9,       [cq+64* 0]
    pmovzxwd             m6,       [cq+64* 8]
    vpermb               m8, m14,  [cq+64* 2]
    vpermb              ym0, ym14, [cq+64*14]
    vpermb              ym5, ym14, [cq+64*10]
    vpermb               m1, m14,  [cq+64* 6]
    vpermb               m7, m14,  [cq+64* 4]
    vpermb              ym3, ym14, [cq+64*12]
    pslld                m9, 16
    pslld                m6, 16
    call m(idct_16x16_internal_8bpc).main_fast
    vpermb              m21, m14,  [cq+64* 1]
    vpermb             ym17, ym14, [cq+64*15]
    vpermb             ym20, ym14, [cq+64* 9]
    vpermb              m15, m14,  [cq+64* 7]
    vpermb              m18, m14,  [cq+64* 5]
    vpermb             ym16, ym14, [cq+64*11]
    vpermb             ym19, ym14, [cq+64*13]
    vpermb              m14, m14,  [cq+64* 3]
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    vpbroadcastd         m9, [o(pw_8192)]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).transpose_round
    vshufi32x4          m22, m14, m2, q2020 ;  1
    vshufi32x4          m24, m14, m2, q3131 ;  5
    vshufi32x4          m23, m17, m9, q2020 ;  3
    vshufi32x4          m25, m17, m9, q3131 ;  7
    vshufi32x4          m16, m5, m15, q2020 ; 10
    vshufi32x4          m17, m5, m15, q3131 ; 14
    vshufi32x4          m14, m1, m18, q2020 ;  2
    vshufi32x4          m15, m1, m18, q3131 ;  6
    vshufi32x4           m1, m0, m3, q3131  ;  4
    vshufi32x4           m0, m3, q2020      ;  0
    vshufi32x4           m3, m21, m4, q3131 ; 12
    vshufi32x4           m2, m21, m4, q2020 ;  8
    vshufi32x4          m26, m20, m6, q2020 ;  9
    vshufi32x4          m28, m20, m6, q3131 ; 13
    vshufi32x4          m27, m19, m7, q2020 ; 11
    vshufi32x4          m29, m19, m7, q3131 ; 15
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast
    mova         [cq+64* 0], m14
    mova         [cq+64* 2], m15
    mova         [cq+64* 4], m16
    mova         [cq+64* 6], m17
    mova         [cq+64* 8], m18
    mova         [cq+64*10], m19
    mova         [cq+64*12], m20
    mova         [cq+64*14], m21
    call .main_oddhalf_fast
.end:
    lea                  r4, [strideq*3]
    vpbroadcastd        m12, [o(pw_2048)]
    movshdup            m13, [o(permD)]
    lea                  r3, [dstq+r4*8]
    lea                  r5, [strideq+r4] ; stride*4
    add                  r3, r5           ; dst+stride*28
    IDCT_32x32_END       29,  0, strideq*0, r4
    IDCT_32x32_END       28,  1, strideq*1, strideq*2
    IDCT_32x32_END       27,  2, strideq*2, strideq*1
    IDCT_32x32_END       26,  3, r4       , strideq*0
    IDCT_32x32_END       25,  4, strideq*0, r4
    IDCT_32x32_END       24,  5, strideq*1, strideq*2
    IDCT_32x32_END       23,  6, strideq*2, strideq*1
    IDCT_32x32_END       22,  7, r4       , strideq*0
    IDCT_32x32_END       21,  8, strideq*0, r4
    IDCT_32x32_END       20,  9, strideq*1, strideq*2
    IDCT_32x32_END       19, 10, strideq*2, strideq*1
    IDCT_32x32_END       18, 11, r4       , strideq*0
    IDCT_32x32_END       17, 12, strideq*0, r4
    IDCT_32x32_END       16, 13, strideq*1, strideq*2
    IDCT_32x32_END       15, 14, strideq*2, strideq*1
    IDCT_32x32_END       14, 15, r4       , strideq*0
    RET
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 32
    jmp m(inv_txfm_add_dct_dct_32x8_8bpc).dconly2
ALIGN function_align
cglobal_label .main_oddhalf_fast3 ; bottom seven-eights are zero
    vpbroadcastd        m21, [o(pw_4091x8)]
    vpbroadcastd         m8, [o(pw_201x8)]
    vpbroadcastd        m24, [o(pw_m601x8)]
    vpbroadcastd        m12, [o(pw_4052x8)]
    pmulhrsw            m21, m22 ; t31a
    pmulhrsw            m22, m8  ; t16a
    pmulhrsw            m24, m23 ; t23a
    pmulhrsw            m23, m12 ; t24a

    punpcklwd            m9, m22, m21
    punpckhwd            m8, m22, m21
    mova                m15, m10
    vpdpwssd            m15, m9, [o(pw_m4017_799)] {bcstd}
    mova                m17, m10
    vpdpwssd            m17, m8, [o(pw_m4017_799)] {bcstd}
    REPX      {psrad x, 12}, m15, m17
    packssdw            m15, m17
    mova                m17, m10
    vpdpwssd            m17, m8, [o(pw_799_4017)] {bcstd}
    mova                 m8, m10
    vpdpwssd             m8, m9, [o(pw_799_4017)] {bcstd}
    REPX      {psrad x, 12}, m17, m8
    packssdw             m8, m17

    punpcklwd            m9, m24, m23
    punpckhwd           m16, m24, m23
    mova                m20, m10
    vpdpwssd            m20, m9, [o(pw_m3406_m2276)] {bcstd}
    mova                m17, m10
    vpdpwssd            m17, m16, [o(pw_m3406_m2276)] {bcstd}
    REPX      {psrad x, 12}, m20, m17
    packssdw            m20, m17
    mova                m17, m10
    vpdpwssd            m17, m16, [o(pw_m2276_3406)] {bcstd}
    mova                m16, m10
    vpdpwssd            m16, m9, [o(pw_m2276_3406)] {bcstd}
    REPX      {psrad x, 12}, m17, m16
    packssdw            m16, m17

    mova                m17, m21
    mova                m27, m15
    mova                m25, m20
    mova                m29, m8
    mova                m18, m22
    mova                m14, m24
    mova                m28, m16
    mova                m26, m23
    jmp .main4
cglobal_label .main_oddhalf_fast2 ; bottom three-quarters are zero
    vpbroadcastd        m21, [o(pw_4091x8)]
    vpbroadcastd         m8, [o(pw_201x8)]
    vpbroadcastd        m18, [o(pw_m1380x8)]
    vpbroadcastd         m9, [o(pw_3857x8)]
    vpbroadcastd        m19, [o(pw_3973x8)]
    vpbroadcastd        m11, [o(pw_995x8)]
    vpbroadcastd        m28, [o(pw_m601x8)]
    vpbroadcastd        m12, [o(pw_4052x8)]
    pmulhrsw            m21, m22 ; t31a
    pmulhrsw            m22, m8  ; t16a
    pmulhrsw            m18, m25 ; t19a
    pmulhrsw            m25, m9 ; t28a
    pmulhrsw            m19, m24 ; t27a
    pmulhrsw            m24, m11 ; t20a
    pmulhrsw            m28, m23 ; t23a
    pmulhrsw            m23, m12 ; t24a
    mova                m15, m21
    mova                 m8, m22
    mova                m14, m18
    mova                m27, m25
    mova                m29, m19
    mova                m26, m24
    mova                m16, m28
    mova                m20, m23
    jmp .main3
ALIGN function_align
cglobal_label .main_oddhalf_fast ; bottom half is zero
    vpbroadcastd        m21, [o(pw_4091x8)]
    vpbroadcastd         m8, [o(pw_201x8)]
    vpbroadcastd        m14, [o(pw_m2751x8)]
    vpbroadcastd         m9, [o(pw_3035x8)]
    vpbroadcastd        m17, [o(pw_3703x8)]
    vpbroadcastd        m11, [o(pw_1751x8)]
    vpbroadcastd        m18, [o(pw_m1380x8)]
    vpbroadcastd        m12, [o(pw_3857x8)]
    pmulhrsw            m21, m22 ; t31a
    vpbroadcastd        m19, [o(pw_3973x8)]
    pmulhrsw            m22, m8  ; t16a
    vpbroadcastd         m8, [o(pw_995x8)]
    pmulhrsw            m14, m29 ; t30a
    vpbroadcastd        m16, [o(pw_m2106x8)]
    pmulhrsw            m29, m9  ; t17a
    vpbroadcastd         m9, [o(pw_3513x8)]
    pmulhrsw            m17, m26 ; t29a
    vpbroadcastd        m15, [o(pw_3290x8)]
    pmulhrsw            m26, m11 ; t18a
    vpbroadcastd        m11, [o(pw_2440x8)]
    pmulhrsw            m18, m25 ; t19a
    vpbroadcastd        m20, [o(pw_m601x8)]
    pmulhrsw            m25, m12 ; t28a
    vpbroadcastd        m12, [o(pw_4052x8)]
    pmulhrsw            m19, m24 ; t27a
    pmulhrsw            m24, m8  ; t20a
    pmulhrsw            m16, m27 ; t21a
    pmulhrsw            m27, m9  ; t26a
    pmulhrsw            m15, m28 ; t25a
    pmulhrsw            m28, m11 ; t22a
    pmulhrsw            m20, m23 ; t23a
    pmulhrsw            m23, m12 ; t24a
    jmp .main2
ALIGN function_align
cglobal_label .main_oddhalf
    ITX_MULSUB_2W        22, 21,  8,  9, 10,  201, 4091 ; t16a, t31a
    ITX_MULSUB_2W        14, 29,  8,  9, 10, 3035, 2751 ; t17a, t30a
    ITX_MULSUB_2W        26, 17,  8,  9, 10, 1751, 3703 ; t18a, t29a
    ITX_MULSUB_2W        18, 25,  8,  9, 10, 3857, 1380 ; t19a, t28a
    ITX_MULSUB_2W        24, 19,  8,  9, 10,  995, 3973 ; t20a, t27a
    ITX_MULSUB_2W        16, 27,  8,  9, 10, 3513, 2106 ; t21a, t26a
    ITX_MULSUB_2W        28, 15,  8,  9, 10, 2440, 3290 ; t22a, t25a
    ITX_MULSUB_2W        20, 23,  8,  9, 10, 4052,  601 ; t23a, t24a
.main2:
    psubsw               m8, m22, m14 ; t17
    paddsw              m22, m14      ; t16
    paddsw              m14, m18, m26 ; t19
    psubsw              m18, m26      ; t18
    psubsw              m26, m24, m16 ; t21
    paddsw              m24, m16      ; t20
    psubsw              m16, m20, m28 ; t22
    paddsw              m28, m20      ; t23
    psubsw              m20, m23, m15 ; t25
    paddsw              m23, m15      ; t24
    psubsw              m15, m21, m29 ; t30
    paddsw              m21, m29      ; t31
    psubsw              m29, m19, m27 ; t26
    paddsw              m19, m27      ; t27
    paddsw              m27, m25, m17 ; t28
    psubsw              m25, m17      ; t29
.main3:
    ITX_MULSUB_2W        15,  8,  9, 17, 10,   799, 4017 ; t17a, t30a
    ITX_MULSUB_2W        25, 18,  9, 17, 10, m4017,  799 ; t18a, t29a
    ITX_MULSUB_2W        29, 26,  9, 17, 10,  3406, 2276 ; t21a, t26a
    ITX_MULSUB_2W        20, 16,  9, 17, 10, m2276, 3406 ; t22a, t25a
    psubsw              m17, m21, m27 ; t28a
    paddsw              m21, m27      ; t31a
    psubsw              m27, m15, m25 ; t18
    paddsw              m15, m25      ; t17
    psubsw              m25, m20, m29 ; t21
    paddsw              m20, m29      ; t22
    psubsw              m29, m8, m18  ; t29
    paddsw               m8, m18      ; t30
    psubsw              m18, m22, m14 ; t19a
    paddsw              m22, m14      ; t16a
    psubsw              m14, m28, m24 ; t20a
    paddsw              m24, m28      ; t23a
    paddsw              m28, m16, m26 ; t25
    psubsw              m16, m26      ; t26
    psubsw              m26, m23, m19 ; t27a
    paddsw              m23, m19      ; t24a
.main4:
    vpbroadcastd        m12, [o(pw_m3784_1567)]
    vpbroadcastd        m11, [o(pw_1567_3784)]
    ITX_MULSUB_2W        29, 27,  9, 19, 10, 11, 12 ; t18a, t29a
    ITX_MULSUB_2W        17, 18,  9, 19, 10, 11, 12 ; t19,  t28
    vpbroadcastd        m11, [o(pw_m1567_m3784)]
    ITX_MULSUB_2W        16, 25,  9, 19, 10, 12, 11 ; t21a, t26a
    ITX_MULSUB_2W        26, 14,  9, 19, 10, 12, 11 ; t20,  t27
    vpbroadcastd        m12, [o(pw_m2896_2896)]
    vpbroadcastd        m11, [o(pw_2896_2896)]
    psubsw              m19, m27, m25 ; t26
    paddsw              m27, m25      ; t29
    psubsw              m25, m17, m26 ; t20a
    paddsw              m17, m26      ; t19a
    paddsw              m26, m18, m14 ; t28a
    psubsw              m18, m14      ; t27a
    paddsw              m14, m22, m24 ; t16
    psubsw              m22, m24      ; t23
    psubsw              m24, m29, m16 ; t21
    paddsw              m16, m29      ; t18
    paddsw              m29, m21, m23 ; t31
    psubsw              m21, m23      ; t24
    psubsw              m23, m15, m20 ; t22a
    paddsw              m15, m20      ; t17a
    psubsw              m20, m8, m28  ; t25a
    paddsw              m28, m8       ; t30a
    ITX_MULSUB_2W        18, 25,  8,  9, 10, 11, 12 ; t20,  t27
    ITX_MULSUB_2W        19, 24,  8,  9, 10, 11, 12 ; t21a, t26a
    ITX_MULSUB_2W        21, 22,  8,  9, 10, 11, 12 ; t23a, t24a
    ITX_MULSUB_2W        20, 23,  8,  9, 10, 11, 12 ; t22,  t25
    ret

%macro IDTX_32x32 2 ; dst[1-2]
    vmovdqa32           ym%1, [cq+64*(%1+ 0)] ; force EVEX encoding, which
    vmovdqa32           ym17, [cq+64*(%1+16)] ; reduces code size due to
    vmovdqa32           ym%2, [cq+64*(%2+ 0)] ; compressed displacements
    vmovdqa32           ym18, [cq+64*(%2+16)]
    vpermt2q             m%1, m21, m17
    vpermt2q             m%2, m21, m18
%endmacro

cglobal inv_txfm_add_identity_identity_32x32_8bpc, 3, 3, 22, dst, stride, c
    movu                 m21, [permB+7]
    vpbroadcastd         m16, [pw_8192]
    pxor                 m20, m20
.loop:
    IDTX_32x32            0,  1
    IDTX_32x32            2,  3
    IDTX_32x32            4,  5
    IDTX_32x32            6,  7
    IDTX_32x32            8,  9
    IDTX_32x32           10, 11
    IDTX_32x32           12, 13
    IDTX_32x32           14, 15
    call m(inv_txfm_add_identity_identity_16x32_8bpc).transpose_2x8x8_round
    IDTX_32x16_STORE      0,  8, 1
    IDTX_32x16_STORE      1,  9, 1
    IDTX_32x16_STORE      2, 10, 1
    IDTX_32x16_STORE      3, 11, 1
    IDTX_32x16_STORE      4, 12, 1
    IDTX_32x16_STORE      5, 13, 1
    IDTX_32x16_STORE      6, 14, 1
    IDTX_32x16_STORE      7, 15, 1
    lea                dstq, [dstq+strideq*8]
    btc                  cq, 5
    jnc .loop
    mov                 r0d, 8
.zero_loop:
    mova          [cq+64*0], m20
    mova          [cq+64*1], m20
    mova          [cq+64*2], m20
    mova          [cq+64*3], m20
    add                  cq, 64*4
    dec                 r0d
    jg .zero_loop
    RET

cglobal inv_txfm_add_dct_dct_16x64_8bpc, 4, 7, 0, dst, stride, c, eob
%undef cmp
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    WIN64_SPILL_XMM      30
    cmp                eobd, 151
    jb .fast
    mova                 m5, [cq+64*10]
    mova                 m3, [cq+64* 6]
    mova                 m1, [cq+64* 2]
    mova                 m7, [cq+64*14]
    mova                 m2, [cq+64* 4]
    mova                 m6, [cq+64*12]
    mova                 m0, [cq+64* 0]
    mova                 m4, [cq+64* 8]
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    mova                m14, [cq+64* 1]
    mova                m21, [cq+64*15]
    mova                m18, [cq+64* 9]
    mova                m17, [cq+64* 7]
    mova                m16, [cq+64* 5]
    mova                m19, [cq+64*11]
    mova                m20, [cq+64*13]
    mova                m15, [cq+64* 3]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf
    vpbroadcastd         m9, [o(pw_8192)]
%macro TRANSPOSE_8x4_ROUND 4
    punpckhwd            m8, m%3, m%4 ; c4 d4 c5 d5 c6 d6 c7 d7
    punpcklwd           m%3, m%4      ; c0 d0 c1 d1 c2 d2 c3 d3
    punpckhwd           m%4, m%1, m%2 ; a4 b4 a5 b5 a6 b6 a7 b7
    punpcklwd           m%1, m%2      ; a0 b0 a1 b1 a2 b2 a3 b3
    punpckhdq           m%2, m%1, m%3 ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckldq           m%1, m%3      ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckldq           m%3, m%4, m8  ; a4 b4 c4 d4 a5 b5 c5 d5
    punpckhdq           m%4, m8       ; a6 b6 c6 d6 a7 b7 c7 d7
    REPX   {pmulhrsw x, m9}, m%2, m%1, m%3, m%4
%endmacro
    TRANSPOSE_8x4_ROUND   0,  1,  2,  3
    TRANSPOSE_8x4_ROUND   4,  5,  6,  7
    TRANSPOSE_8x4_ROUND  14, 15, 16, 17
    TRANSPOSE_8x4_ROUND  18, 19, 20, 21
    vinserti32x8        m26, m0, ym4, 1     ; a0  a4  b0  b4
    vshufi32x4           m0, m4, q3232      ; a8  a12 b8  b12
    vinserti32x8        m27, m1, ym5, 1     ; a1  a5  b1  b5
    vshufi32x4           m1, m5, q3232      ; a9  a13 b9  b13
    vinserti32x8        m28, m2, ym6, 1     ; a2  a6  b2  b6
    vshufi32x4           m2, m6, q3232      ; a10 a14 b10 b14
    vinserti32x8        m29, m3, ym7, 1     ; a3  a7  b3  b7
    vshufi32x4           m8, m3, m7, q3232  ; a11 a15 b11 b15
    vinserti32x8         m4, m14, ym18, 1   ; c0  c4  d0  d4
    vshufi32x4          m14, m18, q3232     ; c8  c12 d8  d12
    vinserti32x8         m5, m15, ym19, 1   ; c1  c5  d1  d5
    vshufi32x4          m15, m19, q3232     ; c9  c13 d9  d13
    vinserti32x8         m6, m16, ym20, 1   ; c2  c6  d2  d6
    vshufi32x4          m16, m20, q3232     ; c10 c14 d10 d14
    vinserti32x8         m7, m17, ym21, 1   ; c3  c7  d3  d7
    vshufi32x4          m17, m21, q3232     ; c11 c15 d11 d15
    vshufi32x4          m22, m26, m4, q2020 ;  0  1
    vshufi32x4          m26, m4, q3131      ;  8  9
    vshufi32x4          m23, m27, m5, q2020 ;  2  3
    vshufi32x4          m27, m5, q3131      ; 10 11
    vshufi32x4          m24, m28, m6, q2020 ;  4  5
    vshufi32x4          m28, m6, q3131      ; 12 13
    vshufi32x4          m25, m29, m7, q2020 ;  6  7
    vshufi32x4          m29, m7, q3131      ; 14 15
    vshufi32x4           m4, m0, m14, q2020 ; 16 17
    vshufi32x4           m3, m0, m14, q3131 ; 24 25
    vshufi32x4          m20, m1, m15, q2020 ; 18 19
    vshufi32x4          m19, m1, m15, q3131 ; 26 27
    vshufi32x4           m5, m2, m16, q2020 ; 20 21
    vshufi32x4           m0, m2, m16, q3131 ; 28 29
    vshufi32x4          m16, m8, m17, q2020 ; 22 23
    vshufi32x4          m17, m8, m17, q3131 ; 30 31
    pxor                 m6, m6
    mova         [cq+64* 0], m4
    mova         [cq+64* 2], m5
    mova         [cq+64* 4], m3
    mova         [cq+64* 6], m0
    punpcklwd            m8, m24, m24 ;  4
    punpcklwd            m0, m0       ; 28
    punpcklwd            m5, m5       ; 20
    punpcklwd            m1, m28, m28 ; 12
    punpcklwd            m7, m26, m26 ;  8
    punpcklwd            m3, m3       ; 24
    punpcklwd            m9, m6, m22  ; __  0
    punpcklwd            m6, m4       ; __ 16
    call m(idct_16x16_internal_8bpc).main_fast3
    mova         [cq+64* 1], m20
    mova         [cq+64* 3], m16
    mova         [cq+64* 5], m19
    mova         [cq+64* 7], m17
    punpcklwd           m21, m23, m23 ;  2
    punpcklwd           m17, m17      ; 30
    punpcklwd           m20, m20      ; 18
    punpcklwd           m15, m29, m29 ; 14
    punpcklwd           m18, m27, m27 ; 10
    punpcklwd           m16, m16      ; 22
    punpcklwd           m19, m19      ; 26
    punpcklwd           m14, m25, m25 ;  6
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    mova         [cq+64* 8], m14
    mova         [cq+64* 9], m15
    mova         [cq+64*10], m16
    mova         [cq+64*11], m17
    mova         [cq+64*12], m18
    mova         [cq+64*13], m19
    mova         [cq+64*14], m20
    mova         [cq+64*15], m21
    mova                m21, [cq+64* 7]
    mova                m14, [cq+64* 0]
    mova                m17, [cq+64* 3]
    mova                m18, [cq+64* 4]
    mova                m19, [cq+64* 5]
    mova                m16, [cq+64* 2]
    mova                m15, [cq+64* 1]
    mova                m20, [cq+64* 6]
    REPX   {punpckhwd x, x}, m22, m21, m14, m29, m26, m17, m18, m25, \
                             m24, m19, m16, m27, m28, m15, m20, m23
    call .main_oddhalf
    jmp .end
.fast: ; right half is zero
    mova                ym8, [cq+64*15]
    vinserti32x8         m8, [cq+64* 1], 1
    mova                 m2, [o(int16_perm)]
    mova                ym9, [cq+64* 8]
    vinserti32x8         m9, [cq+64* 0], 1
    mova                ym0, [cq+64* 7]
    vinserti32x8         m0, [cq+64* 9], 1
    mova                ym7, [cq+64*14]
    vinserti32x8         m7, [cq+64* 2], 1
    mova                ym1, [cq+64* 3]
    vinserti32x8         m1, [cq+64*13], 1
    mova                ym3, [cq+64* 6]
    vinserti32x8         m3, [cq+64*10], 1
    mova                ym5, [cq+64*11]
    vinserti32x8         m5, [cq+64* 5], 1
    mova                ym6, [cq+64*12]
    vinserti32x8         m6, [cq+64* 4], 1
    REPX  {vpermb x, m2, x}, m8, m9, m0, m7, m1, m3, m5, m6
    call m(idct_16x16_internal_8bpc).main2
    vbroadcasti32x4      m8, [o(int_shuf3)]
    vbroadcasti32x4      m9, [o(int_shuf4)]
    vpbroadcastd        m11, [o(pw_8192)]
    pshufb               m0, m8
    pshufb               m1, m9
    pshufb               m2, m8
    pshufb               m3, m9
    REPX  {pmulhrsw x, m11}, m0, m1, m2, m3
    pshufb               m4, m8
    pshufb               m5, m9
    pshufb               m6, m8
    pshufb               m7, m9
    REPX  {pmulhrsw x, m11}, m4, m5, m6, m7
    punpckhdq           m28, m0, m1
    punpckldq            m0, m1
    punpckhdq           m27, m2, m3
    punpckldq            m2, m3
    punpckhdq           m22, m4, m5
    punpckldq            m4, m5
    punpckhdq           m23, m6, m7
    punpckldq            m6, m7
    vinserti32x8        m14, m0, ym2, 1
    vshufi32x4          m15, m0, m2, q3232
    vinserti32x8         m2, m4, ym6, 1
    vshufi32x4           m4, m6, q3232
    vshufi32x4          m21, m14, m2, q2020 ;  0  2
    vshufi32x4          m14, m2, q3131      ;  4  6
    vshufi32x4          m18, m15, m4, q2020 ;  8 10
    vshufi32x4          m15, m4, q3131      ; 12 14
    pxor                 m9, m9
    punpcklwd            m8, m14, m14 ;  4
    punpcklwd            m1, m15, m15 ; 12
    punpcklwd            m7, m18, m18 ;  8
    punpcklwd            m9, m21      ; __  0
    call m(idct_16x16_internal_8bpc).main_fast4
    punpckhwd           m21, m21      ;  2
    punpckhwd           m15, m15      ; 14
    punpckhwd           m18, m18      ; 10
    punpckhwd           m14, m14      ;  6
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast2
    vinserti32x8        m24, m28, ym27, 1
    vshufi32x4          m28, m27, q3232
    vinserti32x8        m27, m22, ym23, 1
    vshufi32x4          m22, m23, q3232
    vshufi32x4          m23, m24, m27, q2020 ;  1  3
    vshufi32x4          m24, m27, q3131      ;  5  7
    vshufi32x4          m27, m28, m22, q2020 ;  9 11
    vshufi32x4          m28, m22, q3131      ; 13 15
    punpcklwd           m22, m23, m23 ;  1
    punpckhwd           m29, m28, m28 ; 15
    punpcklwd           m26, m27, m27 ;  9
    punpckhwd           m25, m24, m24 ;  7
    mova         [cq+64* 8], m14
    mova         [cq+64* 9], m15
    mova         [cq+64*10], m16
    mova         [cq+64*11], m17
    punpcklwd           m24, m24      ;  5
    punpckhwd           m27, m27      ; 11
    punpcklwd           m28, m28      ; 13
    punpckhwd           m23, m23      ;  3
    mova         [cq+64*12], m18
    mova         [cq+64*13], m19
    mova         [cq+64*14], m20
    mova         [cq+64*15], m21
    call .main_oddhalf_fast
.end:
    imul                 r6, strideq, 60
    mova                m10, [o(end_16x32p)]
    vpbroadcastd        m11, [o(pw_2048)]
    lea                  r3, [strideq*3]
    pxor                m12, m12
    add                  r6, dstq         ; dst+stride*60
    psrldq              m13, m10, 1
    lea                  r4, [strideq+r3] ; stride*4
%macro IDCT_16x64_END 3 ; idct32, idct64, tmp
%if %1 & 1
    %define %%s0 r3
    %define %%s1 strideq*2
    %define %%s2 strideq*1
    %define %%s3 strideq*0
%else
    %define %%s0 strideq*0
    %define %%s1 strideq*1
    %define %%s2 strideq*2
    %define %%s3 r3
%if %1
    add                dstq, r4
    sub                  r6, r4
%endif
%endif
%if %1 < 8
    pmulhrsw             m8, m11, m%1
    pmulhrsw             m9, m11, m%2
%else
    mova                 m9, [cq+64*%1]
    paddsw               m8, m9, m%2 ; out  0+n,  1+n
    psubsw               m9, m%2     ; out 63-n, 62-n
    pmulhrsw             m8, m11
    pmulhrsw             m9, m11
%endif
    mova               xm29, [dstq+%%s0]
    vinserti128        ym29, [dstq+%%s1], 1
    mova               xm%3, [r6  +%%s3]
    vinserti128        ym%3, [r6  +%%s2], 1
    vpermb              m29, m10, m29
    vpermb              m%3, m10, m%3
    mova         [cq+64*%1], m12
    paddw               m29, m8
    paddw               m%3, m9
    packuswb            m29, m%3
    vpermd              m29, m13, m29
    mova          [dstq+%%s0], xm29
    vextracti128  [dstq+%%s1], ym29, 1
    vextracti32x4 [r6  +%%s2], m29, 2
    vextracti32x4 [r6  +%%s3], m29, 3
%endmacro
    IDCT_16x64_END        0, 29,  0
    IDCT_16x64_END        1, 28, 28
    IDCT_16x64_END        2, 27, 28
    IDCT_16x64_END        3, 26, 28
    IDCT_16x64_END        4, 25, 28
    IDCT_16x64_END        5, 24, 28
    IDCT_16x64_END        6, 23, 28
    IDCT_16x64_END        7, 22, 28
    IDCT_16x64_END        8, 21, 28
    IDCT_16x64_END        9, 20, 28
    IDCT_16x64_END       10, 19, 28
    IDCT_16x64_END       11, 18, 28
    IDCT_16x64_END       12, 17, 28
    IDCT_16x64_END       13, 16, 28
    IDCT_16x64_END       14, 15, 28
    IDCT_16x64_END       15, 14, 28
    RET
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 64
    imul                r6d, 181
    add                 r6d, 128+512
    sar                 r6d, 8+2
    jmp m(inv_txfm_add_dct_dct_16x8_8bpc).dconly3
ALIGN function_align
cglobal_label .main_oddhalf_fast ; bottom three-quarters are zero
    vpbroadcastd         m8, [o(pw_101_4095x8)]
    vpbroadcastd        m21, [o(pw_m1474_3822x8)]
    vpbroadcastd        m14, [o(pw_897_3996x8)]
    vpbroadcastd        m17, [o(pw_m700_4036x8)]
    vpbroadcastd        m18, [o(pw_501_4065x8)]
    vpbroadcastd        m19, [o(pw_m1092_3948x8)]
    vpbroadcastd        m16, [o(pw_1285_3889x8)]
    vpbroadcastd        m15, [o(pw_m301_4085x8)]
    pmulhrsw             m8, m22 ; t32a t63a
    pmulhrsw            m21, m29 ; t35a t60a
    pmulhrsw            m14, m26 ; t36a t59a
    pmulhrsw            m17, m25 ; t39a t56
    pmulhrsw            m18, m24 ; t40a t55a
    pmulhrsw            m19, m27 ; t43a t52a
    pmulhrsw            m16, m28 ; t44a t51a
    pmulhrsw            m15, m23 ; t47a t48a
    mova                m22, m8
    mova                m29, m21
    mova                m26, m14
    mova                m25, m17
    mova                m24, m18
    mova                m27, m19
    mova                m28, m16
    mova                m20, m15
    jmp .main_oddhalf2
ALIGN function_align
cglobal_label .main_oddhalf
    vpbroadcastd         m8, [o(pw_101_4095x8)]
    vpbroadcastd         m9, [o(pw_m2824_2967x8)]
    vpbroadcastd        m11, [o(pw_1660_3745x8)]
    vpbroadcastd        m12, [o(pw_m1474_3822x8)]
    pmulhrsw            m22, m8       ; t32a t63a
    vpbroadcastd         m8, [o(pw_897_3996x8)]
    pmulhrsw            m21, m9       ; t33a t62a
    vpbroadcastd         m9, [o(pw_m2191_3461x8)]
    pmulhrsw            m14, m11      ; t34a t61a
    vpbroadcastd        m11, [o(pw_2359_3349x8)]
    pmulhrsw            m29, m12      ; t35a t60a
    vpbroadcastd        m12, [o(pw_m700_4036x8)]
    pmulhrsw            m26, m8       ; t36a t59a
    vpbroadcastd         m8, [o(pw_501_4065x8)]
    pmulhrsw            m17, m9       ; t37a t58a
    vpbroadcastd         m9, [o(pw_m2520_3229x8)]
    pmulhrsw            m18, m11      ; t38a t57a
    vpbroadcastd        m11, [o(pw_2019_3564x8)]
    pmulhrsw            m25, m12      ; t39a t56a
    vpbroadcastd        m12, [o(pw_m1092_3948x8)]
    pmulhrsw            m24, m8       ; t40a t55a
    vpbroadcastd         m8, [o(pw_1285_3889x8)]
    pmulhrsw            m19, m9       ; t41a t54a
    vpbroadcastd         m9, [o(pw_m1842_3659x8)]
    pmulhrsw            m16, m11      ; t42a t53a
    vpbroadcastd        m11, [o(pw_2675_3102x8)]
    pmulhrsw            m27, m12      ; t43a t52a
    vpbroadcastd        m12, [o(pw_m301_4085x8)]
    pmulhrsw            m28, m8       ; t44a t51a
    pmulhrsw            m15, m9       ; t45a t50a
    pmulhrsw            m20, m11      ; t46a t49a
    pmulhrsw            m23, m12      ; t47a t48a
    psubsw               m8, m22, m21 ; t33  t62
    paddsw              m22, m21      ; t32  t63
    psubsw              m21, m29, m14 ; t34  t61
    paddsw              m29, m14      ; t35  t60
    psubsw              m14, m26, m17 ; t37  t58
    paddsw              m26, m17      ; t36  t59
    psubsw              m17, m25, m18 ; t38  t57
    paddsw              m25, m18      ; t39  t56
    psubsw              m18, m24, m19 ; t41  t54
    paddsw              m24, m19      ; t40  t55
    psubsw              m19, m27, m16 ; t42  t53
    paddsw              m27, m16      ; t43  t52
    psubsw              m16, m28, m15 ; t45  t50
    paddsw              m28, m15      ; t44  t51
    psubsw              m15, m23, m20 ; t46  t49
    paddsw              m20, m23      ; t47  t48
.main_oddhalf2:
    ITX_MUL2X_PACK        8, 9, 23, 10,   401, 4076, 5 ; t33a t62a
    ITX_MUL2X_PACK       21, 9, 23, 10, m4076,  401, 5 ; t34a t61a
    ITX_MUL2X_PACK       14, 9, 23, 10,  3166, 2598, 5 ; t37a t58a
    ITX_MUL2X_PACK       17, 9, 23, 10, m2598, 3166, 5 ; t38a t57a
    ITX_MUL2X_PACK       18, 9, 23, 10,  1931, 3612, 5 ; t41a t54a
    ITX_MUL2X_PACK       19, 9, 23, 10, m3612, 1931, 5 ; t42a t53a
    ITX_MUL2X_PACK       16, 9, 23, 10,  3920, 1189, 5 ; t45a t50a
    ITX_MUL2X_PACK       15, 9, 23, 10, m1189, 3920, 5 ; t46a t49a
    vpbroadcastd        m11, [o(pw_m4017_799)]
    psubsw              m23, m25, m26 ; t36a t59a
    paddsw              m25, m26      ; t39a t56a
    psubsw              m26, m24, m27 ; t43a t52a
    paddsw              m27, m24      ; t40a t55a
    psubsw              m24, m20, m28 ; t44a t51a
    paddsw              m20, m28      ; t47a t48a
    psubsw              m28, m8, m21  ; t34  t61
    paddsw               m8, m21      ; t33  t62
    psubsw              m21, m17, m14 ; t37  t58
    paddsw              m17, m14      ; t38  t57
    psubsw              m14, m18, m19 ; t42  t53
    paddsw              m18, m19      ; t41  t54
    psubsw              m19, m15, m16 ; t45  t50
    paddsw              m15, m16      ; t46  t49
    psubsw              m16, m22, m29 ; t35a t60a
    paddsw              m22, m29      ; t32a t63a
    ITX_MUL2X_PACK       16, 9, 29, 10, 799_4017, 11,    20 ; t35  t60
    ITX_MUL2X_PACK       28, 9, 29, 10, 799_4017, 11,    20 ; t34a t61a
    ITX_MUL2X_PACK       23, 9, 29, 10, 11, m799_m4017,  36 ; t36  t59
    ITX_MUL2X_PACK       21, 9, 29, 10, 11, m799_m4017,  36 ; t37a t58a
    vpbroadcastd        m11, [o(pw_m2276_3406)]
    ITX_MUL2X_PACK       26, 9, 29, 10, 3406_2276, 11,   20 ; t43  t52
    ITX_MUL2X_PACK       14, 9, 29, 10, 3406_2276, 11,   20 ; t42a t53a
    ITX_MUL2X_PACK       24, 9, 29, 10, 11, m3406_m2276, 36 ; t44  t51
    ITX_MUL2X_PACK       19, 9, 29, 10, 11, m3406_m2276, 36 ; t45a t50a
    vpbroadcastd        m11, [o(pw_1567_3784)]
    vpbroadcastd        m12, [o(pw_m3784_1567)]
    psubsw              m29, m22, m25 ; t39  t56
    paddsw              m22, m25      ; t32  t63
    psubsw              m25, m20, m27 ; t40  t55
    paddsw              m20, m27      ; t47  t48
    psubsw              m27, m8, m17  ; t38a t57a
    paddsw               m8, m17      ; t33a t62a
    psubsw              m17, m15, m18 ; t41a t54a
    paddsw              m15, m18      ; t46a t49a
    paddsw              m18, m16, m23 ; t35a t60a
    psubsw              m16, m23      ; t36a t59a
    psubsw              m23, m24, m26 ; t43a t52a
    paddsw              m24, m26      ; t44a t51a
    paddsw              m26, m28, m21 ; t34  t61
    psubsw              m28, m21      ; t37  t58
    psubsw              m21, m19, m14 ; t42  t53
    paddsw              m19, m14      ; t45  t50
    ITX_MUL2X_PACK       29, 9, 14, 10, 11, 12, 4 ; t39a t56a
    ITX_MUL2X_PACK       27, 9, 14, 10, 11, 12, 4 ; t38  t57
    ITX_MUL2X_PACK       16, 9, 14, 10, 11, 12, 4 ; t36  t59
    ITX_MUL2X_PACK       28, 9, 14, 10, 11, 12, 4 ; t37a t58a
    vpbroadcastd        m11, [o(pw_m1567_m3784)]
    ITX_MUL2X_PACK       25, 9, 14, 10, 12, 11, 4 ; t40a t55a
    ITX_MUL2X_PACK       17, 9, 14, 10, 12, 11, 4 ; t41  t54
    ITX_MUL2X_PACK       23, 9, 14, 10, 12, 11, 4 ; t43  t52
    ITX_MUL2X_PACK       21, 9, 14, 10, 12, 11, 4 ; t42a t53a
    vbroadcasti32x4     m13, [o(deint_shuf)]
    vpbroadcastd        m11, [o(pw_2896_2896)]
    vpbroadcastd        m12, [o(pw_m2896_2896)]
    paddsw              m14, m22, m20 ; t32a t63a
    psubsw              m22, m20      ; t47a t48a
    psubsw              m20, m8, m15  ; t46  t49
    paddsw               m8, m15      ; t33  t62
    paddsw              m15, m18, m24 ; t35  t60
    psubsw              m18, m24      ; t44  t51
    psubsw              m24, m26, m19 ; t45a t50a
    paddsw              m26, m19      ; t34a t61a
    REPX    {pshufb x, m13}, m14, m8, m15, m26
    psubsw              m19, m29, m25 ; t40  t55
    paddsw              m25, m29      ; t39  t56
    psubsw              m29, m27, m17 ; t41a t54a
    paddsw              m27, m17      ; t38a t57a
    psubsw              m17, m16, m23 ; t43a t52a
    paddsw              m16, m23      ; t36a t59a
    psubsw               m9, m28, m21 ; t42  t53
    paddsw              m28, m21      ; t37  t58
    REPX    {pshufb x, m13}, m25, m27, m16, m28
    ITX_MUL2X_PACK       22, 13, 21, 10, 11, 12, 8 ; t47  t48
    ITX_MUL2X_PACK       20, 23, 22, 10, 11, 12, 8 ; t46a t49a
    packssdw            m21, m22      ; t47  t46a
    packssdw            m13, m23      ; t48  t49a
    ITX_MUL2X_PACK       18, 22, 20, 10, 11, 12, 8 ; t44a t51a
    ITX_MUL2X_PACK       24, 23, 18, 10, 11, 12, 8 ; t45  t50
    packssdw            m20, m18      ; t44a t45
    packssdw            m22, m23      ; t51a t50
    ITX_MUL2X_PACK       19, 24, 18, 10, 11, 12, 8 ; t40a t55a
    ITX_MUL2X_PACK       29, 23, 19, 10, 11, 12, 8 ; t41  t54
    packssdw            m18, m19      ; t40a t41
    packssdw            m24, m23      ; t55a t54
    ITX_MUL2X_PACK       17, 23, 19, 10, 11, 12, 8 ; t43  t52
    ITX_MUL2X_PACK        9, 29, 17, 10, 11, 12, 8 ; t42a t53a
    packssdw            m19, m17      ; t43  t42a
    packssdw            m23, m29      ; t52  t53a
    punpcklqdq          m17, m25, m27 ; t39  t38a
    punpckhqdq          m25, m27      ; t56  t57a
    punpckhqdq          m27, m15, m26 ; t60  t61a
    punpcklqdq          m15, m26      ; t35  t34a
    punpckhqdq          m26, m16, m28 ; t59a t58
    punpcklqdq          m16, m28      ; t36a t37
    punpckhqdq          m28, m14, m8  ; t63a t62
    punpcklqdq          m14, m8       ; t32a t33
    psubsw              m29, m0, m28  ; out63 out62
    paddsw               m0, m28      ; out0  out1
    psubsw              m28, m1, m27  ; out60 out61
    paddsw               m1, m27      ; out3  out2
    psubsw              m27, m2, m26  ; out59 out58
    paddsw               m2, m26      ; out4  out5
    psubsw              m26, m3, m25  ; out56 out57
    paddsw               m3, m25      ; out7  out6
    psubsw              m25, m4, m24  ; out55 out54
    paddsw               m4, m24      ; out8  out9
    psubsw              m24, m5, m23  ; out52 out53
    paddsw               m5, m23      ; out11 out10
    psubsw              m23, m6, m22  ; out51 out50
    paddsw               m6, m22      ; out12 out13
    psubsw              m22, m7, m13  ; out48 out49
    paddsw               m7, m13      ; out15 out14
    ret

cglobal inv_txfm_add_dct_dct_64x16_8bpc, 4, 7, 0, dst, stride, c, eob
%undef cmp
    lea                  r5, [o_base]
    test               eobd, eobd
    jnz .normal
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 16
.dconly:
    imul                r6d, 181
    add                 r6d, 128+512
    sar                 r6d, 8+2
.dconly2:
    imul                r6d, 181
    add                 r6d, 128+2048
    sar                 r6d, 8+4
    pxor                 m2, m2
    vpbroadcastw         m3, r6d
.dconly_loop:
    mova                 m1, [dstq]
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    paddw                m0, m3
    paddw                m1, m3
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, strideq
    dec                 r3d
    jg .dconly_loop
    RET
.normal:
    WIN64_SPILL_XMM      31
    mova                m19, [o(dup16_perm)]
    mova                m24, [cq+64* 2]
    mova                m28, [cq+64* 6]
    mova                m26, [cq+64* 4]
    mova                m22, [cq+64* 0]
    mova                m23, [cq+64* 1]
    mova                m29, [cq+64* 7]
    mova                m27, [cq+64* 5]
    mova                m25, [cq+64* 3]
    vpermb               m8, m19, m24        ;  4
    vpermb               m1, m19, m28        ; 12
    vpermb               m7, m19, m26        ;  8
    vpermb               m9, m19, m22        ; __  0
    vpermb              m21, m19, m23        ;  2
    vpermb              m15, m19, m29        ; 14
    vpermb              m18, m19, m27        ; 10
    vpermb              m14, m19, m25        ;  6
    pslld                m9, 16
    vpord               m30, m19, [o(pb_32)] {1to16}
    REPX {vpermb x, m30, x}, m22, m29, m26, m25, m24, m27, m28, m23
    cmp                eobd, 151
    jb .fast
    vpermb               m0, m19, [cq+64*14] ; 28
    vpermb               m5, m19, [cq+64*10] ; 20
    vpermb               m3, m19, [cq+64*12] ; 24
    vpermb               m6, m19, [cq+64* 8] ; __ 16
    pslld                m6, 16
    call m(idct_16x16_internal_8bpc).main_fast
    vpermb              m17, m19, [cq+64*15] ; 30
    vpermb              m20, m19, [cq+64* 9] ; 18
    vpermb              m16, m19, [cq+64*11] ; 22
    vpermb              m19, m19, [cq+64*13] ; 26
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    mova         [cq+64* 0], m14
    mova         [cq+64* 1], m15
    mova         [cq+64* 2], m16
    mova         [cq+64* 3], m17
    mova         [cq+64* 4], m18
    mova         [cq+64* 5], m19
    mova         [cq+64* 6], m20
    mova         [cq+64* 7], m21
    vpermb              m21, m30, [cq+64*15]
    vpermb              m14, m30, [cq+64* 8]
    vpermb              m17, m30, [cq+64*11]
    vpermb              m18, m30, [cq+64*12]
    vpermb              m19, m30, [cq+64*13]
    vpermb              m16, m30, [cq+64*10]
    vpermb              m15, m30, [cq+64* 9]
    vpermb              m20, m30, [cq+64*14]
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_oddhalf
    jmp .end
.fast: ; bottom half is zero
    call m(idct_16x16_internal_8bpc).main_fast2
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast2
    mova         [cq+64* 0], m14
    mova         [cq+64* 1], m15
    mova         [cq+64* 2], m16
    mova         [cq+64* 3], m17
    mova         [cq+64* 4], m18
    mova         [cq+64* 5], m19
    mova         [cq+64* 6], m20
    mova         [cq+64* 7], m21
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_oddhalf_fast
.end:
    mova         [cq+64* 8], m4
    mova         [cq+64* 9], m5
    mova         [cq+64*10], m6
    mova         [cq+64*11], m7
    mova         [cq+64*12], m26
    mova         [cq+64*13], m27
    mova         [cq+64*14], m28
    mova         [cq+64*15], m29
    vpbroadcastd        m13, [o(pw_8192)]
    call .pass1_end
    call .pass2
    mova         [cq+64* 0], m0
    mova         [cq+64* 1], m1
    mova         [cq+64* 2], m2
    mova         [cq+64* 3], m3
    mova         [cq+64* 4], m4
    mova         [cq+64* 5], m5
    mova         [cq+64* 6], m6
    mova         [cq+64* 7], m7
    pmulhrsw             m0, m13, [cq+64* 8]
    pmulhrsw             m1, m13, [cq+64* 9]
    pmulhrsw             m2, m13, [cq+64*10]
    pmulhrsw             m3, m13, [cq+64*11]
    vpbroadcastd        m30, [o(pw_2048)]
    pmulhrsw             m4, m13, m22
    pmulhrsw             m5, m13, m23
    pmulhrsw             m6, m13, m24
    pmulhrsw             m7, m13, m25
    pmulhrsw            m22, m30, m14
    pmulhrsw            m14, m13, m26
    pmulhrsw            m23, m30, m15
    pmulhrsw            m15, m13, m27
    pmulhrsw            m24, m30, m16
    pmulhrsw            m16, m13, m28
    pmulhrsw            m25, m30, m17
    pmulhrsw            m17, m13, m29
    pmulhrsw            m26, m30, m18
    pmulhrsw            m18, m13, [cq+64*12]
    pmulhrsw            m27, m30, m19
    pmulhrsw            m19, m13, [cq+64*13]
    pmulhrsw            m28, m30, m20
    pmulhrsw            m20, m13, [cq+64*14]
    pmulhrsw            m29, m30, m21
    pmulhrsw            m21, m13, [cq+64*15]
    call .transpose_round
    call .pass2
    pxor                m10, m10
    lea                  r3, [strideq*3]
%macro IDCT_64x16_END 4
    mova                 m9, [dstq+%4]
%if %1 < 8
    pmulhrsw            m%3, m30, [cq+64*%1]
%endif
    pmulhrsw            m%2, m30
    mova         [cq+64*%1], m10
    punpcklbw            m8, m9, m10
    punpckhbw            m9, m10
    paddw                m8, m%3
    paddw                m9, m%2
    packuswb             m8, m9
    mova          [dstq+%4], m8
%if %1 == 3 || %1 == 7 || %1 == 11
    lea                dstq, [dstq+strideq*4]
%endif
%endmacro
    IDCT_64x16_END        0,  0, 11, strideq*0
    IDCT_64x16_END        1,  1, 11, strideq*1
    IDCT_64x16_END        2,  2, 11, strideq*2
    IDCT_64x16_END        3,  3, 11, r3
    IDCT_64x16_END        4,  4, 11, strideq*0
    IDCT_64x16_END        5,  5, 11, strideq*1
    IDCT_64x16_END        6,  6, 11, strideq*2
    IDCT_64x16_END        7,  7, 11, r3
    IDCT_64x16_END        8, 14, 22, strideq*0
    IDCT_64x16_END        9, 15, 23, strideq*1
    IDCT_64x16_END       10, 16, 24, strideq*2
    IDCT_64x16_END       11, 17, 25, r3
    IDCT_64x16_END       12, 18, 26, strideq*0
    IDCT_64x16_END       13, 19, 27, strideq*1
    IDCT_64x16_END       14, 20, 28, strideq*2
    IDCT_64x16_END       15, 21, 29, r3
    RET
ALIGN function_align
.pass1_end:
    mova                 m4, [cq+64* 0]
    mova                 m5, [cq+64* 1]
    mova                 m6, [cq+64* 2]
    mova                 m7, [cq+64* 3]
    mova                 m8, [cq+64* 4]
    mova                 m9, [cq+64* 5]
    mova                m11, [cq+64* 6]
    mova                m12, [cq+64* 7]
    psubsw              m29, m4, m21  ; out47 out46
    paddsw               m4, m21      ; out16 out17
    psubsw              m28, m5, m20  ; out44 out45
    paddsw               m5, m20      ; out19 out18
    REPX  {pmulhrsw x, m13}, m0, m1, m2, m3
    psubsw              m27, m6, m19  ; out43 out42
    paddsw               m6, m19      ; out20 out21
    psubsw              m26, m7, m18  ; out40 out41
    paddsw               m7, m18      ; out23 out22
    pmulhrsw            m18, m13, m22
    pmulhrsw            m19, m13, m23
    pmulhrsw            m20, m13, m24
    pmulhrsw            m21, m13, m25
    paddsw              m25, m12, m14 ; out31 out30
    psubsw              m14, m12, m14 ; out32 out33
    paddsw              m24, m11, m15 ; out28 out29
    psubsw              m15, m11, m15 ; out35 out34
    REPX  {pmulhrsw x, m13}, m4, m5, m6, m7
    paddsw              m23, m9, m16  ; out27 out26
    psubsw              m16, m9, m16  ; out36 out37
    paddsw              m22, m8, m17  ; out24 out25
    psubsw              m17, m8, m17  ; out39 out38
    REPX  {pmulhrsw x, m13}, m14, m15, m16, m17
.transpose_round:
%macro TRANSPOSE_8x4_PACKED 4
    punpckhwd            m8, m%1, m%3 ; b0 f0 b1 f1 b2 f2 b3 f3
    punpcklwd           m%1, m%3      ; a0 e0 a1 e1 a2 e2 a3 e3
    punpcklwd           m%3, m%2, m%4 ; d0 h0 d1 h1 d2 h2 d3 h3
    punpckhwd           m%2, m%4      ; c0 g0 c1 g1 c2 g2 c3 g3
    punpckhwd           m%4, m%1, m%2 ; a2 c2 e2 g2 a3 c3 e3 g3
    punpcklwd           m%1, m%2      ; a0 c0 e0 g0 a1 c1 e1 g1
    punpckhwd           m%2, m8, m%3  ; b2 d2 f2 h2 b3 d3 f3 h3
    punpcklwd            m8, m%3      ; b0 d0 f0 h0 b1 d1 f1 h1
    punpcklwd           m%3, m%4, m%2 ; 2
    punpckhwd           m%4, m%2      ; 3
    punpckhwd           m%2, m%1, m8  ; 1
    punpcklwd           m%1, m8       ; 0
%endmacro
    TRANSPOSE_8x4_PACKED  0,  1,  2,  3
    TRANSPOSE_8x4_PACKED 18, 19, 20, 21
    TRANSPOSE_8x4_PACKED  4,  5,  6,  7
    TRANSPOSE_8x4_PACKED 14, 15, 16, 17
    vshufi32x4           m8, m0, m4, q3232   ; a02 a03 b02 b03
    vinserti32x8         m0, ym4, 1          ; a00 a01 b00 b01
    vshufi32x4           m4, m1, m5, q3232   ; a12 a13 b12 b13
    vinserti32x8         m9, m1, ym5, 1      ; a10 a11 b10 b11
    vshufi32x4           m5, m2, m6, q3232   ; a22 a23 b22 b23
    vinserti32x8         m1, m2, ym6, 1      ; a20 a21 b20 b21
    vshufi32x4           m6, m3, m7, q3232   ; a32 a33 b32 b33
    vinserti32x8        m11, m3, ym7, 1      ; a30 a31 b30 b31
    vshufi32x4           m2, m14, m18, q3232 ; c02 c03 d02 d03
    vinserti32x8         m3, m14, ym18, 1    ; c00 c01 d00 d01
    vshufi32x4          m18, m15, m19, q3232 ; c12 c13 d12 d13
    vinserti32x8        m15, ym19, 1         ; c10 c11 d10 d11
    vshufi32x4          m19, m16, m20, q3232 ; c22 c23 d22 d23
    vinserti32x8        m16, ym20, 1         ; c20 c21 d20 d21
    vshufi32x4          m20, m17, m21, q3232 ; c32 c33 d32 d33
    vinserti32x8        m17, ym21, 1         ; c30 c31 d30 d31
    ret
.pass2:
    vshufi32x4           m7, m5, m19, q3131  ; 14
    vshufi32x4           m5, m19, q2020      ; 10
    vshufi32x4          m21, m6, m20, q3131  ; 15
    vshufi32x4          m19, m6, m20, q2020  ; 11
    vshufi32x4          m20, m4, m18, q3131  ; 13
    vshufi32x4          m18, m4, m18, q2020  ;  9
    vshufi32x4           m6, m8, m2, q3131   ; 12
    vshufi32x4           m4, m8, m2, q2020   ;  8
    vshufi32x4           m2, m0, m3, q3131   ;  4
    vshufi32x4           m0, m3, q2020       ;  0
    vshufi32x4           m3, m1, m16, q3131  ;  6
    vshufi32x4           m1, m16, q2020      ;  2
    vshufi32x4          m16, m9, m15, q3131  ;  5
    vshufi32x4          m14, m9, m15, q2020  ;  1
    vshufi32x4          m15, m11, m17, q2020 ;  3
    vshufi32x4          m17, m11, m17, q3131 ;  7
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main2
    jmp m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf

cglobal inv_txfm_add_dct_dct_32x64_8bpc, 4, 7, 0, dst, stride, c, eob
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 9, 30, 64*32, dst, stride, c, eob
    vpbroadcastd        m23, [o(pw_2896x8)]
%undef cmp
    cmp                eobd, 136
    jb .fast
    pmulhrsw             m5, m23, [cq+64*20]
    pmulhrsw             m3, m23, [cq+64*12]
    pmulhrsw             m1, m23, [cq+64* 4]
    pmulhrsw             m7, m23, [cq+64*28]
    pmulhrsw             m2, m23, [cq+64* 8]
    pmulhrsw             m6, m23, [cq+64*24]
    pmulhrsw             m0, m23, [cq+64* 0]
    pmulhrsw             m4, m23, [cq+64*16]
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    pmulhrsw            m14, m23, [cq+64* 2]
    pmulhrsw            m21, m23, [cq+64*30]
    pmulhrsw            m18, m23, [cq+64*18]
    pmulhrsw            m17, m23, [cq+64*14]
    pmulhrsw            m16, m23, [cq+64*10]
    pmulhrsw            m19, m23, [cq+64*22]
    pmulhrsw            m20, m23, [cq+64*26]
    pmulhrsw            m15, m23, [cq+64* 6]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf
    mova         [cq+64* 0], m14
    mova         [cq+64* 2], m15
    mova         [cq+64* 4], m16
    mova         [cq+64* 6], m17
    mova         [cq+64* 8], m18
    mova         [cq+64*10], m19
    mova         [cq+64*12], m20
    mova         [cq+64*14], m21
    pmulhrsw            m22, m23, [cq+64* 1]
    pmulhrsw            m21, m23, [cq+64*31]
    pmulhrsw            m14, m23, [cq+64*17]
    pmulhrsw            m29, m23, [cq+64*15]
    pmulhrsw            m26, m23, [cq+64* 9]
    pmulhrsw            m17, m23, [cq+64*23]
    pmulhrsw            m18, m23, [cq+64*25]
    pmulhrsw            m25, m23, [cq+64* 7]
    pmulhrsw            m24, m23, [cq+64* 5]
    pmulhrsw            m19, m23, [cq+64*27]
    pmulhrsw            m16, m23, [cq+64*21]
    pmulhrsw            m27, m23, [cq+64*11]
    pmulhrsw            m28, m23, [cq+64*13]
    pmulhrsw            m15, m23, [cq+64*19]
    pmulhrsw            m20, m23, [cq+64*29]
    pmulhrsw            m23,      [cq+64* 3]
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf
    vpbroadcastd        m12, [o(pw_16384)]
    psubsw              m13, m0, m29 ; 31
    paddsw               m0, m29     ;  0
    psubsw              m29, m1, m28 ; 30
    paddsw               m1, m28     ;  1
    psubsw              m28, m2, m27 ; 29
    paddsw               m2, m27     ;  2
    psubsw              m27, m3, m26 ; 28
    paddsw               m3, m26     ;  3
    psubsw              m26, m4, m25 ; 27
    paddsw               m4, m25     ;  4
    psubsw              m25, m5, m24 ; 26
    paddsw               m5, m24     ;  5
    psubsw              m24, m6, m23 ; 25
    paddsw               m6, m23     ;  6
    psubsw              m23, m7, m22 ; 24
    paddsw               m7, m22     ;  7
    pxor                 m9, m9
    punpckhwd            m8, m0, m1 ; a4 b4 a5 b5 a6 b6 a7 b7
    punpcklwd            m0, m1     ; a0 b0 a1 b1 a2 b2 a3 b3
    punpckhwd            m1, m2, m3 ; c4 d4 c5 d5 c6 d6 c7 d7
    punpcklwd            m2, m3     ; c0 d0 c1 d1 c2 d2 c3 d3
    REPX {mova [cq+64*x], m9}, 16, 17, 18, 19
    punpckhwd           m22, m4, m5 ; e4 f4 e5 f5 e6 f6 e7 f7
    punpcklwd            m4, m5     ; e0 f0 e1 f1 e2 f2 e3 f3
    punpckhwd            m5, m6, m7 ; g4 h4 g5 h5 g6 h6 g7 h7
    punpcklwd            m6, m7     ; g0 h0 g1 h1 g2 h2 g3 h3
    REPX {mova [cq+64*x], m9}, 20, 21, 22, 23
    punpckhwd            m3, m23, m24
    punpcklwd           m23, m24
    punpckhwd           m24, m25, m26
    punpcklwd           m25, m26
    REPX {mova [cq+64*x], m9}, 24, 25, 26, 27
    punpckhwd           m26, m27, m28
    punpcklwd           m27, m28
    punpckhwd           m28, m29, m13
    punpcklwd           m29, m13
    REPX {mova [cq+64*x], m9}, 28, 29, 30, 31
    punpckhdq            m7, m0, m2  ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckldq            m0, m2      ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckhdq            m2, m4, m6  ; e2 f2 g2 h2 e3 f3 g3 h3
    punpckldq            m4, m6      ; e0 f0 g0 h0 e1 f1 g1 h1
    REPX  {pmulhrsw x, m12}, m7, m0, m2, m4
    punpckhdq            m6, m8, m1  ; a6 b6 c6 d6 a7 b7 c7 d7
    punpckldq            m8, m1      ; a4 b4 c4 d4 a5 b5 c5 d5
    punpckhdq            m1, m22, m5 ; e6 f6 g6 h6 e7 f7 g7 h7
    punpckldq           m22, m5      ; e4 f4 g4 h5 e5 f5 g5 h5
    REPX  {pmulhrsw x, m12}, m6, m8, m1, m22
    punpckhdq           m13, m23, m25
    punpckldq           m23, m25
    punpckhdq           m25, m27, m29
    punpckldq           m27, m29
    REPX  {pmulhrsw x, m12}, m13, m23, m25, m27
    punpckhdq            m9, m3, m24
    punpckldq            m3, m24
    punpckhdq           m24, m26, m28
    punpckldq           m26, m28
    REPX  {pmulhrsw x, m12}, m9, m3, m24, m26
    punpckhqdq           m5, m23, m27 ; d01 d09 d17 d25
    punpcklqdq          m23, m27      ; d00 d08 d16 d24
    punpcklqdq          m27, m13, m25 ; d02 d10 d18 d26
    punpckhqdq          m13, m25      ; d03 d11 d19 d27
    punpcklqdq          m25, m3, m26  ; d04 d12 d20 d28
    punpckhqdq           m3, m26      ; d05 d13 d21 d29
    punpcklqdq          m26, m9, m24  ; d06 d14 d22 d30
    punpckhqdq           m9, m24      ; d07 d15 d23 d31
    mova         [cq+64* 3], m23
    mova         [cq+64*13], m27
    mova         [cq+64* 7], m25
    mova         [cq+64*15], m26
    punpckhqdq          m24, m8, m22  ; a05 a13 a21 a29
    punpcklqdq           m8, m22      ; a04 a12 a20 a28
    punpckhqdq          m22, m0, m4   ; a01 a09 a17 a25
    punpcklqdq           m0, m4       ; a00 a08 a16 a24
    punpckhqdq          m23, m7, m2   ; a03 a11 a19 a27
    punpcklqdq           m7, m2       ; a02 a10 a18 a26
    punpckhqdq          m25, m6, m1   ; a07 a15 a23 a31
    punpcklqdq           m6, m1       ; a06 a14 a22 a30
    mova         [cq+64* 1], m0
    mova         [cq+64* 9], m7
    mova         [cq+64* 5], m8
    mova         [cq+64*11], m6
    mova                 m2, [cq+64* 0]
    mova                m11, [cq+64* 2]
    mova                 m8, [cq+64* 4]
    mova                m29, [cq+64* 6]
    mova                m27, [cq+64* 8]
    mova                m26, [cq+64*10]
    mova                 m4, [cq+64*12]
    mova                m28, [cq+64*14]
    psubsw               m1, m2, m21  ; 23
    paddsw               m2, m21      ;  8
    psubsw              m21, m11, m20 ; 22
    paddsw              m11, m20      ;  9
    psubsw              m20, m8, m19  ; 21
    paddsw               m8, m19      ; 10
    psubsw              m19, m29, m18 ; 20
    paddsw              m29, m18      ; 11
    psubsw              m18, m27, m17 ; 19
    paddsw              m27, m17      ; 12
    psubsw              m17, m26, m16 ; 18
    paddsw              m26, m16      ; 13
    psubsw              m16, m4, m15  ; 17
    paddsw               m4, m15      ; 14
    psubsw              m15, m28, m14 ; 16
    paddsw              m28, m14      ; 15
    punpcklwd           m14, m15, m16
    punpckhwd           m15, m16
    punpckhwd           m16, m17, m18
    punpcklwd           m17, m18
    punpckhwd           m18, m19, m20
    punpcklwd           m19, m20
    punpckhwd           m20, m21, m1
    punpcklwd           m21, m1
    punpckhwd            m1, m2, m11  ; i4 j4 i5 j5 i6 j6 i7 j7
    punpcklwd            m2, m11      ; i0 j1 i1 j1 i2 j2 i3 j3
    punpckhwd           m11, m8, m29  ; k4 l4 k5 l5 k6 l6 k7 l7
    punpcklwd            m8, m29      ; k0 l0 k1 l1 k2 l2 k3 l3
    punpckhwd           m29, m27, m26 ; m4 n4 m5 n5 m6 n6 m7 n7
    punpcklwd           m27, m26      ; m0 n0 m1 n1 m2 n2 m3 n3
    punpckhwd           m26, m4, m28  ; o4 p4 o5 p5 o6 p6 o7 p7
    punpcklwd            m4, m28      ; o0 p0 o1 p1 o2 p2 o3 p3
    punpckhdq           m28, m2, m8   ; i2 j2 k2 l2 i3 j3 k3 l3
    punpckldq            m2, m8       ; i0 j0 k0 l0 i1 j1 k1 l1
    punpckhdq            m8, m27, m4  ; m2 n2 o2 p2 m3 n3 o3 p3
    punpckldq           m27, m4       ; m0 n0 o0 p0 m1 n1 o1 p1
    REPX  {pmulhrsw x, m12}, m28, m2, m8, m27
    punpckhdq            m4, m1, m11  ; i6 j6 k6 l6 i7 j7 k7 l7
    punpckldq            m1, m11      ; i4 j4 k4 l4 i5 j5 k5 l5
    punpckhdq           m11, m29, m26 ; m6 n6 o6 p6 m7 n7 o7 p7
    punpckldq           m29, m26      ; m4 n4 o4 p4 m5 n5 o5 p5
    REPX  {pmulhrsw x, m12}, m4, m1, m11, m29
    punpckhdq           m26, m19, m21
    punpckldq           m19, m21
    punpckhdq           m21, m15, m16
    punpckldq           m15, m16
    REPX  {pmulhrsw x, m12}, m26, m19, m21, m15
    punpckhdq           m16, m18, m20
    punpckldq           m18, m20
    punpckhdq           m20, m14, m17
    punpckldq           m14, m17
    REPX  {pmulhrsw x, m12}, m16, m18, m20, m14
    punpckhqdq          m17, m28, m8  ; b03 b11 b19 b27
    punpcklqdq          m28, m8       ; b02 b10 b18 b26
    punpckhqdq           m8, m2, m27  ; b01 b09 b17 b25
    punpcklqdq           m2, m27      ; b00 b08 b16 b24
    punpcklqdq          m27, m1, m29  ; b04 b12 b20 b28
    punpckhqdq           m1, m29      ; b05 b13 b21 b29
    punpcklqdq          m29, m4, m11  ; b06 b14 b22 b30
    punpckhqdq           m4, m11      ; b07 b15 b23 b31
    mova         [cq+64* 0], m2
    mova         [cq+64* 8], m28
    mova         [cq+64* 4], m27
    mova         [cq+64*10], m29
    punpckhqdq          m27, m20, m26 ; c03 c11 c19 c27
    punpcklqdq          m20, m26      ; c02 c10 c18 c26
    punpckhqdq          m26, m14, m19 ; c01 c09 c17 c25
    punpcklqdq          m14, m19      ; c00 c08 c16 c24
    punpckhqdq          m28, m15, m18 ; c05 c13 c21 c29
    punpcklqdq          m15, m18      ; c04 c12 c20 c28
    punpckhqdq          m29, m21, m16 ; c07 c15 c23 c31
    punpcklqdq          m21, m16      ; c06 c14 c22 c30
    mova         [cq+64* 2], m14
    mova         [cq+64*12], m20
    mova         [cq+64* 6], m15
    mova         [cq+64*14], m21
    vshufi32x4          m14, m22, m8, q3232  ; a17 a25 b17 b25
    vinserti32x8        m22, ym8, 1          ; a01 a09 b01 b09
    vshufi32x4          m15, m23, m17, q3232 ; a19 a27 b19 b27
    vinserti32x8        m23, ym17, 1         ; a03 a11 b03 b11
    vshufi32x4          m16, m24, m1, q3232  ; a21 a29 b21 b29
    vinserti32x8        m24, ym1, 1          ; a05 a13 b05 b13
    vshufi32x4          m17, m25, m4, q3232  ; a23 a31 b23 b31
    vinserti32x8        m25, ym4, 1          ; a07 a15 b07 b15
    vinserti32x8        m19, m26, ym5, 1     ; c01 c09 d01 d09
    vshufi32x4          m26, m5, q3232       ; c17 c25 d17 d25
    vinserti32x8        m20, m27, ym13, 1    ; c03 c11 d03 d11
    vshufi32x4          m27, m13, q3232      ; c19 c27 d19 d27
    vinserti32x8        m21, m28, ym3, 1     ; c05 c13 d05 d13
    vshufi32x4          m28, m3, q3232       ; c21 c29 d21 d29
    vinserti32x8        m18, m29, ym9, 1     ; c07 c15 d07 d15
    vshufi32x4          m29, m9, q3232       ; c23 c31 d23 d31
    mov                  r4, rsp
    vshufi32x4           m0, m22, m19, q2020 ;  1
    vshufi32x4           m1, m17, m29, q3131 ; 31
    vshufi32x4           m2, m14, m26, q2020 ; 17
    vshufi32x4           m3, m25, m18, q3131 ; 15
    call .main_part1
    vshufi32x4           m0, m25, m18, q2020 ;  7
    vshufi32x4           m1, m14, m26, q3131 ; 25
    vshufi32x4           m2, m17, m29, q2020 ; 23
    vshufi32x4           m3, m22, m19, q3131 ;  9
    call .main_part1
    vshufi32x4           m0, m24, m21, q2020 ;  5
    vshufi32x4           m1, m15, m27, q3131 ; 27
    vshufi32x4           m2, m16, m28, q2020 ; 21
    vshufi32x4           m3, m23, m20, q3131 ; 11
    call .main_part1
    vshufi32x4           m0, m23, m20, q2020 ;  3
    vshufi32x4           m1, m16, m28, q3131 ; 29
    vshufi32x4           m2, m15, m27, q2020 ; 19
    vshufi32x4           m3, m24, m21, q3131 ; 13
    call .main_part1
    call .main_part2
    mova                 m0, [cq+64* 1] ; a0
    mova                m15, [cq+64* 0] ; b0
    mova                 m3, [cq+64* 2] ; c0
    mova                m16, [cq+64* 3] ; d0
    mova                m14, [cq+64* 5] ; a4
    mova                 m8, [cq+64* 4] ; b4
    mova                m17, [cq+64* 6] ; c4
    mova                 m1, [cq+64* 7] ; d4
    vshufi32x4           m2, m0, m15, q3232  ; a16 a24 b16 b24
    vinserti32x8         m0, ym15, 1         ; a00 a08 b00 b08
    vshufi32x4          m15, m3, m16, q3232  ; c16 c24 d16 d24
    vinserti32x8         m3, ym16, 1         ; c00 c08 d00 d08
    vshufi32x4          m16, m14, m8, q3232  ; a20 a28 b20 b28
    vinserti32x8        m14, ym8, 1          ; a04 a12 b04 b12
    vshufi32x4           m8, m17, m1, q3232  ; c20 c28 d20 d28
    vinserti32x8        m17, ym1, 1          ; c04 c12 d04 d12
    vshufi32x4           m1, m0, m3, q3131   ;  8
    vshufi32x4           m0, m3, q2020       ;  0
    vshufi32x4           m3, m2, m15, q3131  ; 24
    vshufi32x4           m2, m15, q2020      ; 16
    vshufi32x4          m15, m14, m17, q3131 ; 12
    vshufi32x4          m14, m17, q2020      ;  4
    vshufi32x4          m17, m16, m8, q3131  ; 28
    vshufi32x4          m16, m8, q2020       ; 20
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast
    mova                 m8, [cq+64* 8]
    mova                 m9, [cq+64*12]
    mova                m11, [cq+64*10]
    mova                m12, [cq+64*14]
    mova         [cq+64* 0], m14
    mova         [cq+64* 2], m15
    mova         [cq+64* 4], m16
    mova         [cq+64* 6], m17
    mova         [cq+64* 8], m18
    mova         [cq+64*10], m19
    mova         [cq+64*12], m20
    mova         [cq+64*14], m21
    mova                m22, [cq+64* 9]
    mova                m27, [cq+64*13]
    mova                m23, [cq+64*11]
    mova                m24, [cq+64*15]
    vshufi32x4          m26, m22, m8, q3232  ; a18 a26 b18 b26
    vinserti32x8        m22, ym8, 1          ; a02 a10 b02 b10
    vshufi32x4           m8, m9, m27, q3232  ; c18 c26 d18 d26
    vinserti32x8         m9, ym27, 1         ; c02 c10 d02 d10
    vshufi32x4          m27, m23, m11, q3232 ; a22 a30 b22 b30
    vinserti32x8        m23, ym11, 1         ; a06 a14 b06 b14
    vshufi32x4          m11, m12, m24, q3232 ; c22 c30 d22 d30
    vinserti32x8        m12, ym24, 1         ; c06 c14 d06 d14
    vshufi32x4          m28, m26, m8, q3131  ; 26
    vshufi32x4          m26, m8, q2020       ; 18
    vshufi32x4          m24, m22, m9, q3131  ; 10
    vshufi32x4          m22, m9, q2020       ;  2
    vshufi32x4          m29, m27, m11, q3131 ; 30
    vshufi32x4          m27, m11, q2020      ; 22
    vshufi32x4          m25, m23, m12, q3131 ; 14
    vshufi32x4          m23, m12, q2020      ;  6
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast
    jmp .end
.fast: ; bottom/right halves are zero
    pmulhrsw            ym9, ym23, [cq+64* 0]
    pmulhrsw            ym6, ym23, [cq+64* 8]
    mova                m14, [o(dup16_perm)]
    pmulhrsw            ym8, ym23, [cq+64* 2]
    pmulhrsw            xm0, xm23, [cq+64*14]
    pmulhrsw            xm5, xm23, [cq+64*10]
    pmulhrsw            ym1, ym23, [cq+64* 6]
    pmulhrsw            ym7, ym23, [cq+64* 4]
    pmulhrsw            xm3, xm23, [cq+64*12]
    pmovzxwd             m9, ym9
    pmovzxwd             m6, ym6
    vpermb               m8, m14, m8
    punpcklwd           xm0, xm0
    vpermb              ym5, ym14, ym5
    vpermb               m1, m14, m1
    vpermb               m7, m14, m7
    punpcklwd           xm3, xm3
    pslld                m9, 16
    pslld                m6, 16
    call m(idct_16x16_internal_8bpc).main_fast
          vpmulhrsw    ym21, ym23, [cq+64* 1]
    {evex}vpmulhrsw    xm17, xm23, [cq+64*15] ; force EVEX encoding, which
    {evex}vpmulhrsw    xm20, xm23, [cq+64* 9] ; reduces code size due to
    {evex}vpmulhrsw    ym15, ym23, [cq+64* 7] ; compressed displacements
    {evex}vpmulhrsw    ym18, ym23, [cq+64* 5]
    {evex}vpmulhrsw    xm16, xm23, [cq+64*11]
    {evex}vpmulhrsw    xm19, xm23, [cq+64*13]
    {evex}vpmulhrsw    ym23,       [cq+64* 3]
    vpermb              m21, m14, m21
    punpcklwd          xm17, xm17
    vpermb             ym20, ym14, ym20
    vpermb              m15, m14, m15
    vpermb              m18, m14, m18
    vpermb             ym16, ym14, ym16
    punpcklwd          xm19, xm19
    vpermb              m14, m14, m23
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    vpbroadcastd         m9, [o(pw_16384)]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).transpose_round
    vshufi32x4          m16, m0, m3, q2020  ;  0
    vshufi32x4          m26, m0, m3, q3131  ;  4
    vshufi32x4           m0, m14, m2, q2020 ;  1
    vshufi32x4          m14, m2, q3131      ;  5
    vshufi32x4           m3, m19, m7, q3131 ; 15
    vshufi32x4          m19, m7, q2020      ; 11
    vshufi32x4          m27, m17, m9, q2020 ;  3
    vshufi32x4          m17, m9, q3131      ;  7
    vshufi32x4          m28, m20, m6, q2020 ;  9
    vshufi32x4          m20, m6, q3131      ; 13
    vshufi32x4          m22, m1, m18, q2020 ;  2
    vshufi32x4          m23, m1, m18, q3131 ;  6
    vshufi32x4          m24, m5, m15, q2020 ; 10
    vshufi32x4          m25, m5, m15, q3131 ; 14
    vshufi32x4          m15, m21, m4, q3131 ; 12
    vshufi32x4          m21, m21, m4, q2020 ;  8
    mov                  r4, rsp
    call .main_part1_fast
    mova                 m0, m17
    mova                 m3, m28
    call .main_part1_fast
    mova                 m0, m14
    mova                 m3, m19
    call .main_part1_fast
    mova                 m0, m27
    mova                 m3, m20
    call .main_part1_fast
    call .main_part2
    mova                 m0, m16
    mova                 m1, m21
    mova                m14, m26
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast2
    mova         [cq+64*14], m21
    mova         [cq+64* 0], m14
    mova         [cq+64* 6], m17
    mova         [cq+64* 8], m18
    mova         [cq+64*10], m19
    mova         [cq+64* 4], m16
    mova         [cq+64* 2], m15
    mova         [cq+64*12], m20
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast2
.end:
    lea                  r4, [strideq*3]
    vpbroadcastd        m12, [o(pw_2048)]
    movshdup            m13, [o(permD)]
    lea                  r5, [r4+strideq]   ; stride*4
    lea                  r3, [dstq+r4*8]
    lea                  r6, [strideq+r5*8] ; stride*33
    lea                  r8, [r4+r5*8]      ; stride*35
    add                  r3, r5             ; dst+stride*28
    lea                  r7, [r6+strideq]   ; stride*34
%macro IDCT_32x64_END 6 ; src, mem, stride[1-4]
%if %2 < 8
    paddsw              m10, m%2, m%1
    psubsw              m11, m%2, m%1
%else
    mova                m11, [cq+64*(%2*2-16)]
    paddsw              m10, m11, m%1
    psubsw              m11, m%1
%endif
    mova                 m9, [rsp+64*(31-%2)]
    mova                m%1, [rsp+64*%2]
    paddsw               m8, m10, m9
    psubsw              m10, m9
    paddsw               m9, m11, m%1
    pmovzxbw             m0, [dstq+%3]
    psubsw              m11, m%1
    pmovzxbw            m%1, [r3  +%4]
    REPX  {pmulhrsw x, m12}, m8, m10, m9, m11
    paddw                m8, m0
    pmovzxbw             m0, [r3  +%5]
    paddw               m10, m%1
    pmovzxbw            m%1, [dstq+%6]
    paddw                m9, m0
    paddw               m11, m%1
%if %2 >= 8
%if %2 == 8
    pxor                 m1, m1
%endif
    mova  [cq+64*(%2*2-16)], m1
    mova  [cq+64*(%2*2-15)], m1
%endif
    packuswb             m8, m10
    packuswb             m9, m11
    vpermq               m8, m13, m8
    vpermq               m9, m13, m9
    mova          [dstq+%3], ym8
    vextracti32x8 [r3  +%4], m8, 1
    mova          [r3  +%5], ym9
    vextracti32x8 [dstq+%6], m9, 1
%if %2 == 3 || %2 == 7 || %2 == 11
    add                dstq, r5
    sub                  r3, r5
%endif
%endmacro
    IDCT_32x64_END       29,  0, strideq*0, r8,   r4       , r5*8
    IDCT_32x64_END       28,  1, strideq*1, r7,   strideq*2, r6
    IDCT_32x64_END       27,  2, strideq*2, r6,   strideq*1, r7
    IDCT_32x64_END       26,  3, r4       , r5*8, strideq*0, r8
    IDCT_32x64_END       25,  4, strideq*0, r8,   r4       , r5*8
    IDCT_32x64_END       24,  5, strideq*1, r7,   strideq*2, r6
    IDCT_32x64_END       23,  6, strideq*2, r6,   strideq*1, r7
    IDCT_32x64_END       22,  7, r4       , r5*8, strideq*0, r8
    IDCT_32x64_END       21,  8, strideq*0, r8,   r4       , r5*8
    IDCT_32x64_END       20,  9, strideq*1, r7,   strideq*2, r6
    IDCT_32x64_END       19, 10, strideq*2, r6,   strideq*1, r7
    IDCT_32x64_END       18, 11, r4       , r5*8, strideq*0, r8
    IDCT_32x64_END       17, 12, strideq*0, r8,   r4       , r5*8
    IDCT_32x64_END       16, 13, strideq*1, r7,   strideq*2, r6
    IDCT_32x64_END       15, 14, strideq*2, r6,   strideq*1, r7
    IDCT_32x64_END       14, 15, r4       , r5*8, strideq*0, r8
    RET
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 64
    imul                r6d, 181
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    add                 r6d, 128+256
    sar                 r6d, 8+1
    jmp m(inv_txfm_add_dct_dct_32x8_8bpc).dconly3
ALIGN function_align ; bottom three-quarters are zero
cglobal_label .main_part1_fast2
    vpbroadcastd         m7, [o(idct64_mul+4*0)]
    vpbroadcastd         m8, [o(idct64_mul+4*1)]
    pmulhrsw             m7, m0     ; t63a
    pmulhrsw             m0, m8     ; t32a

    punpcklwd            m4, m0, m7
    punpckhwd            m6, m0, m7
    mova                 m1, m10
    vpdpwssd             m1, m4, [o(idct64_mul+4*9)] {bcstd}
    mova                 m9, m10
    vpdpwssd             m9, m6, [o(idct64_mul+4*9)] {bcstd}
    REPX      {psrad x, 12}, m1, m9
    packssdw             m1, m9
    mova                 m9, m10
    vpdpwssd             m9, m6, [o(idct64_mul+4*8)] {bcstd}
    mova                 m6, m10
    vpdpwssd             m6, m4, [o(idct64_mul+4*8)] {bcstd}
    REPX      {psrad x, 12}, m9, m6
    packssdw             m6, m9

    mova                 m4, m0
    mova                 m3, m7
    mova                 m5, m1
    mova                 m2, m6
    jmp .main_part1c
cglobal_label .main_part1_fast
    vpbroadcastd         m1, [o(idct64_mul+4*0)]
    vpbroadcastd         m8, [o(idct64_mul+4*1)]
    vpbroadcastd         m2, [o(idct64_mul+4*6)]
    vpbroadcastd         m9, [o(idct64_mul+4*7)]
    pmulhrsw             m1, m0     ; t63a
    pmulhrsw             m0, m8     ; t32a
    pmulhrsw             m2, m3     ; t60a
    pmulhrsw             m3, m9     ; t35a
    mova                 m8, m0
    mova                 m7, m1
    mova                 m6, m3
    mova                 m5, m2
    jmp .main_part1b
cglobal_label .main_part1
    ; idct64 steps 1-5:
    ; in1/31/17/15 -> t32a/33/34a/35/60/61a/62/63a
    ; in7/25/23/ 9 -> t56a/57/58a/59/36/37a/38/39a
    ; in5/27/21/11 -> t40a/41/42a/43/52/53a/54/55a
    ; in3/29/19/13 -> t48a/49/50a/51/44/45a/46/47a
    vpbroadcastd         m7, [o(idct64_mul+4*0)]
    vpbroadcastd         m8, [o(idct64_mul+4*1)]
    vpbroadcastd         m6, [o(idct64_mul+4*2)]
    vpbroadcastd         m9, [o(idct64_mul+4*3)]
    pmulhrsw             m7, m0     ; t63a
    vpbroadcastd         m5, [o(idct64_mul+4*4)]
    pmulhrsw             m0, m8     ; t32a
    vpbroadcastd         m8, [o(idct64_mul+4*5)]
    pmulhrsw             m6, m1     ; t62a
    vpbroadcastd         m4, [o(idct64_mul+4*6)]
    pmulhrsw             m1, m9     ; t33a
    vpbroadcastd         m9, [o(idct64_mul+4*7)]
    pmulhrsw             m5, m2     ; t61a
    pmulhrsw             m2, m8     ; t34a
    pmulhrsw             m4, m3     ; t60a
    pmulhrsw             m3, m9     ; t35a
    psubsw               m8, m0, m1 ; t33
    paddsw               m0, m1     ; t32
    psubsw               m1, m7, m6 ; t62
    paddsw               m7, m6     ; t63
    psubsw               m6, m3, m2 ; t34
    paddsw               m3, m2     ; t35
    psubsw               m2, m4, m5 ; t61
    paddsw               m5, m4     ; t60
.main_part1b:
    vpbroadcastd        m11, [o(idct64_mul+4*8)]
    vpbroadcastd        m12, [o(idct64_mul+4*9)]
    ITX_MULSUB_2W         1, 8, 4, 9, 10, 11, 12 ; t33a, t62a
    vpbroadcastd        m11, [o(idct64_mul+4*10)]
    ITX_MULSUB_2W         2, 6, 4, 9, 10, 12, 11 ; t34a, t61a
    psubsw               m4, m0, m3 ; t35a
    paddsw               m0, m3     ; t32a
    psubsw               m3, m7, m5 ; t60a
    paddsw               m7, m5     ; t63a
    psubsw               m5, m1, m2 ; t34
    paddsw               m1, m2     ; t33
    psubsw               m2, m8, m6 ; t61
    paddsw               m6, m8     ; t62
.main_part1c:
    vpbroadcastd        m11, [o(idct64_mul+4*11)]
    vpbroadcastd        m12, [o(idct64_mul+4*12)]
    add                  r5, 4*13
    ITX_MULSUB_2W         3, 4, 8, 9, 10, 11, 12 ; t35,  t60
    ITX_MULSUB_2W         2, 5, 8, 9, 10, 11, 12 ; t34a, t61a
    mova          [r4+64*0], m0
    mova          [r4+64*7], m7
    mova          [r4+64*1], m1
    mova          [r4+64*6], m6
    mova          [r4+64*3], m3
    mova          [r4+64*4], m4
    mova          [r4+64*2], m2
    mova          [r4+64*5], m5
    add                  r4, 64*8
    ret
cglobal_label .main_part2
    vpbroadcastd        m11, [o(pw_1567_3784  -16*13)]
    vpbroadcastd        m12, [o(pw_m3784_1567 -16*13)]
    lea                  r6, [r4+64*7]
    vpbroadcastd        m17, [o(pw_m1567_m3784-16*13)]
    vpbroadcastd        m18, [o(pw_2896_2896  -16*13)]
    vpbroadcastd        m19, [o(pw_m2896_2896 -16*13)]
    sub                  r5, 16*13
.main_part2_loop:
    mova                 m0, [r4-64*32] ; t32a
    mova                 m1, [r6-64*24] ; t39a
    mova                 m2, [r6-64*32] ; t63a
    mova                 m3, [r4-64*24] ; t56a
    mova                 m4, [r4-64*16] ; t40a
    mova                 m5, [r6-64* 8] ; t47a
    mova                 m6, [r6-64*16] ; t55a
    mova                 m7, [r4-64* 8] ; t48a
    psubsw               m8, m0, m1 ; t39
    paddsw               m0, m1     ; t32
    psubsw               m1, m2, m3 ; t56
    paddsw               m2, m3     ; t63
    psubsw               m3, m5, m4 ; t40
    paddsw               m5, m4     ; t47
    psubsw               m4, m7, m6 ; t55
    paddsw               m7, m6     ; t48
    ITX_MULSUB_2W         1, 8, 6, 9, 10, 11, 12 ; t39a, t56a
    ITX_MULSUB_2W         4, 3, 6, 9, 10, 12, 17 ; t40a, t55a
    psubsw               m6, m2, m7 ; t48a
    paddsw               m2, m7     ; t63a
    psubsw               m7, m0, m5 ; t47a
    paddsw               m0, m5     ; t32a
    psubsw               m5, m8, m3 ; t55
    paddsw               m8, m3     ; t56
    psubsw               m3, m1, m4 ; t40
    paddsw               m1, m4     ; t39
    ITX_MULSUB_2W         6, 7, 4, 9, 10, 18, 19 ; t47,  t48
    ITX_MULSUB_2W         5, 3, 4, 9, 10, 18, 19 ; t40a, t55a
    mova         [r6-64* 8], m2
    mova         [r4-64*32], m0
    mova         [r4-64* 8], m8
    mova         [r6-64*32], m1
    mova         [r6-64*24], m6
    mova         [r4-64*16], m7
    mova         [r4-64*24], m5
    mova         [r6-64*16], m3
    add                  r4, 64
    sub                  r6, 64
    cmp                  r4, r6
    jb .main_part2_loop
    ret

cglobal inv_txfm_add_dct_dct_64x32_8bpc, 4, 7, 0, dst, stride, c, eob
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 7, 30, 64*32, dst, stride, c, eob
    vpbroadcastd        m23, [o(pw_2896x8)]
%undef cmp
    cmp                eobd, 136
    jb .fast
    pmulhrsw             m0, m23, [cq+64* 1]
    pmulhrsw             m1, m23, [cq+64*31]
    pmulhrsw             m2, m23, [cq+64*17]
    pmulhrsw             m3, m23, [cq+64*15]
    vpbroadcastd        m10, [o(pd_2048)]
    mov                  r4, rsp
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    pmulhrsw             m0, m23, [cq+64* 7]
    pmulhrsw             m1, m23, [cq+64*25]
    pmulhrsw             m2, m23, [cq+64*23]
    pmulhrsw             m3, m23, [cq+64* 9]
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    pmulhrsw             m0, m23, [cq+64* 5]
    pmulhrsw             m1, m23, [cq+64*27]
    pmulhrsw             m2, m23, [cq+64*21]
    pmulhrsw             m3, m23, [cq+64*11]
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    pmulhrsw             m0, m23, [cq+64* 3]
    pmulhrsw             m1, m23, [cq+64*29]
    pmulhrsw             m2, m23, [cq+64*19]
    pmulhrsw             m3, m23, [cq+64*13]
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part2
    pmulhrsw             m3, m23, [cq+64*24]
    pmulhrsw             m1, m23, [cq+64* 8]
    pmulhrsw             m2, m23, [cq+64*16]
    pmulhrsw             m0, m23, [cq+64* 0]
    pmulhrsw            m14, m23, [cq+64* 4]
    pmulhrsw            m17, m23, [cq+64*28]
    pmulhrsw            m16, m23, [cq+64*20]
    pmulhrsw            m15, m23, [cq+64*12]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast
    pmulhrsw            m22, m23, [cq+64* 2]
    pmulhrsw            m29, m23, [cq+64*30]
    pmulhrsw            m26, m23, [cq+64*18]
    pmulhrsw            m25, m23, [cq+64*14]
    pmulhrsw            m24, m23, [cq+64*10]
    pmulhrsw            m27, m23, [cq+64*22]
    pmulhrsw            m28, m23, [cq+64*26]
    pmulhrsw            m23,      [cq+64* 6]
    mova         [cq+64* 0], m14
    mova         [cq+64* 1], m15
    mova         [cq+64* 2], m16
    mova         [cq+64* 3], m17
    mova         [cq+64* 4], m18
    mova         [cq+64* 5], m19
    mova         [cq+64* 6], m20
    mova         [cq+64* 7], m21
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast
    vpbroadcastd        m13, [o(pw_16384)]
    call .pass1_end_part1
    mova         [cq+64*16], m1
    mova         [cq+64*17], m3
    mova         [cq+64*18], m5
    mova         [cq+64*19], m7
    mova         [cq+64*24], m23
    mova         [cq+64*25], m25
    mova         [cq+64*26], m27
    mova         [cq+64*27], m29
    pmulhrsw            m23, m13, m0 ; a0
    pmulhrsw            m25, m13, m2 ; a2
    pmulhrsw            m27, m13, m4 ; a4
    pmulhrsw            m29, m13, m6 ; a6
    REPX {pmulhrsw x, m13}, m22, m24, m26, m28 ; e0 e2 e4 e6
    call .pass1_end_part2
    mova         [cq+64*20], m15
    mova         [cq+64*21], m17
    mova         [cq+64*22], m19
    mova         [cq+64*23], m21
    mova         [cq+64*28], m1
    mova         [cq+64*29], m3
    mova         [cq+64*30], m5
    mova         [cq+64*31], m7
    REPX {pmulhrsw x, m13}, m14, m16, m18, m20 ; c0 c2 c4 c6
    REPX {pmulhrsw x, m13}, m0, m2, m4, m6     ; g0 g2 g4 g6
    vinserti32x8        m3, m23, ym14, 1 ; a00 a01 c00 c01
    vshufi32x4         m23, m14, q3232   ; a02 a03 c02 c03
    vinserti32x8       m15, m22, ym0, 1  ; e00 e01 g00 g01
    vshufi32x4         m22, m0, q3232    ; e02 e03 g02 g03
    vinserti32x8        m1, m27, ym18, 1 ; a40 a41 c40 c41
    vshufi32x4         m27, m18, q3232   ; a42 a43 c42 c43
    vinserti32x8       m18, m26, ym4, 1  ; e40 e41 g40 g41
    vshufi32x4         m26, m4, q3232    ; e42 e43 g42 g43
    vinserti32x8       m14, m25, ym16, 1 ; a20 a21 c20 c21
    vshufi32x4         m25, m16, q3232   ; a22 a23 c22 c23
    vinserti32x8       m17, m24, ym2, 1  ; e20 e21 g20 g21
    vshufi32x4         m24, m2, q3232    ; e22 e23 g22 g23
    vinserti32x8       m19, m29, ym20, 1 ; a60 a61 c60 c61
    vshufi32x4         m29, m20, q3232   ; a62 a63 c62 c63
    vinserti32x8       m20, m28, ym6, 1  ; e60 e61 g60 g61
    vshufi32x4         m28, m6, q3232    ; e62 e63 g62 g63
    vshufi32x4          m2, m3, m15, q3131  ;  8
    vshufi32x4          m0, m3, m15, q2020  ;  0
    vshufi32x4          m6, m23, m22, q3131 ; 24
    vshufi32x4          m4, m23, m22, q2020 ; 16
    vshufi32x4          m3, m1, m18, q3131  ; 12
    vshufi32x4          m1, m18, q2020      ;  4
    vshufi32x4          m7, m27, m26, q3131 ; 28
    vshufi32x4          m5, m27, m26, q2020 ; 20
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    vshufi32x4         m16, m14, m17, q3131 ; 10
    vshufi32x4         m14, m17, q2020      ;  2
    vshufi32x4         m17, m19, m20, q3131 ; 14
    vshufi32x4         m15, m19, m20, q2020 ;  6
    vshufi32x4         m20, m25, m24, q3131 ; 26
    vshufi32x4         m18, m25, m24, q2020 ; 18
    vshufi32x4         m21, m29, m28, q3131 ; 30
    vshufi32x4         m19, m29, m28, q2020 ; 22
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf
    pmulhrsw           m22, m13, [cq+64*16] ; a1
    pmulhrsw           m23, m13, [cq+64*20] ; c1
    pmulhrsw           m24, m13, [cq+64*24] ; e1
    pmulhrsw           m25, m13, [cq+64*28] ; g1
    pmulhrsw           m26, m13, [cq+64*17] ; a3
    pmulhrsw           m27, m13, [cq+64*21] ; c3
    pmulhrsw           m28, m13, [cq+64*25] ; e3
    pmulhrsw           m29, m13, [cq+64*29] ; g3
    mova        [cq+64* 8], m14
    mova        [cq+64* 9], m15
    mova        [cq+64*10], m16
    mova        [cq+64*11], m17
    mova        [cq+64*12], m18
    mova        [cq+64*13], m19
    mova        [cq+64*14], m20
    mova        [cq+64*15], m21
    pmulhrsw           m14, m13, [cq+64*18] ; a5
    pmulhrsw           m15, m13, [cq+64*22] ; c5
    pmulhrsw           m16, m13, [cq+64*26] ; e5
    pmulhrsw           m17, m13, [cq+64*30] ; g5
    pmulhrsw           m18, m13, [cq+64*19] ; a7
    pmulhrsw           m19, m13, [cq+64*23] ; c7
    pmulhrsw           m20, m13, [cq+64*27] ; e7
    pmulhrsw           m21, m13, [cq+64*31] ; g7
    vinserti32x8        m8, m22, ym23, 1 ; a10 a11 c10 c11
    vshufi32x4         m22, m23, q3232   ; a12 a13 c12 c13
    vinserti32x8        m9, m24, ym25, 1 ; e10 e11 g10 g11
    vshufi32x4         m24, m25, q3232   ; e12 e13 g12 g13
    vinserti32x8       m23, m26, ym27, 1 ; a30 a31 c30 c31
    vshufi32x4         m26, m27, q3232   ; a32 a33 c32 c33
    vinserti32x8       m11, m28, ym29, 1 ; e30 e31 g30 g31
    vshufi32x4         m28, m29, q3232   ; e32 e33 g32 g33
    mova        [cq+64* 0], m0
    mova        [cq+64* 1], m1
    mova        [cq+64* 2], m2
    mova        [cq+64* 3], m3
    mova        [cq+64* 4], m4
    mova        [cq+64* 5], m5
    mova        [cq+64* 6], m6
    mova        [cq+64* 7], m7
    vinserti32x8       m12, m14, ym15, 1 ; a50 a51 c50 c51
    vshufi32x4         m14, m15, q3232   ; a52 a53 c52 c53
    vinserti32x8       m13, m16, ym17, 1 ; e50 e51 g50 g51
    vshufi32x4         m16, m17, q3232   ; e52 e53 g52 g53
    vinserti32x8       m25, m18, ym19, 1 ; a70 a71 c70 c71
    vshufi32x4         m18, m19, q3232   ; a72 a73 c72 c73
    vinserti32x8       m17, m20, ym21, 1 ; e70 e71 g70 g71
    vshufi32x4         m20, m21, q3232   ; e72 e73 g72 g73
    vshufi32x4         m27, m23, m11, q3131 ; 11 m27
    vshufi32x4         m23, m11, q2020      ;  3 m23
    vshufi32x4         m19, m26, m28, q3131 ; 27 m19
    vshufi32x4         m15, m26, m28, q2020 ; 19 m15
    vshufi32x4         m29, m25, m17, q3131 ; 15 m29
    vshufi32x4         m25, m17, q2020      ;  7 m25
    vshufi32x4         m21, m18, m20, q3131 ; 31 m21
    vshufi32x4         m17, m18, m20, q2020 ; 23 m17
    vshufi32x4         m20, m14, m16, q3131 ; 29 m20
    vshufi32x4         m16, m14, m16, q2020 ; 21 m16
    vshufi32x4         m18, m22, m24, q3131 ; 25 m18
    vshufi32x4         m14, m22, m24, q2020 ; 17 m14
    vshufi32x4         m26, m8, m9, q3131   ;  9 m26
    vshufi32x4         m22, m8, m9, q2020   ;  1 m22
    vshufi32x4         m28, m12, m13, q3131 ; 13 m28
    vshufi32x4         m24, m12, m13, q2020 ;  5 m24
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf
    vpbroadcastd       m13, [o(pw_16384)]
    pmulhrsw            m0, m13, [r4-64*21]
    pmulhrsw            m1, m13, [r4-64*22]
    pmulhrsw            m2, m13, [r4-64*23]
    pmulhrsw            m3, m13, [r4-64*24]
    pmulhrsw            m4, m13, [r4-64*25]
    pmulhrsw            m5, m13, [r4-64*26]
    pmulhrsw            m6, m13, [r4-64*27]
    pmulhrsw            m7, m13, [r4-64*28]
    mova        [cq+64*16], m14
    mova        [cq+64*17], m15
    mova        [cq+64*18], m16
    mova        [cq+64*19], m17
    mova        [cq+64*20], m18
    mova        [cq+64*21], m19
    mova        [cq+64*22], m20
    mova        [cq+64*23], m21
    pmulhrsw           m14, m13, [r4-64*12]
    pmulhrsw           m15, m13, [r4-64*11]
    pmulhrsw           m16, m13, [r4-64*10]
    pmulhrsw           m17, m13, [r4-64* 9]
    pmulhrsw           m18, m13, [r4-64* 8]
    pmulhrsw           m19, m13, [r4-64* 7]
    pmulhrsw           m20, m13, [r4-64* 6]
    pmulhrsw           m21, m13, [r4-64* 5]
    mova        [cq+64*24], m22
    mova        [cq+64*25], m23
    mova        [cq+64*26], m24
    mova        [cq+64*27], m25
    mova        [cq+64*28], m26
    mova        [cq+64*29], m27
    mova        [cq+64*30], m28
    mova        [cq+64*31], m29
    call .transpose_2x8x8_lo
    mova        [r4-64*12], m1
    mova        [r4-64*11], m3
    mova        [r4-64*10], m5
    mova        [r4-64* 9], m7
    mova        [r4-64* 8], m15
    mova        [r4-64* 7], m17
    mova        [r4-64* 6], m19
    mova        [r4-64* 5], m21
    vinserti32x8       m22, m0, ym14, 1     ; f00 f01 h00 h01
    vshufi32x4         m23, m0, m14, q3232  ; f02 f03 h02 h03
    vinserti32x8       m24, m2, ym16, 1     ; f20 f21 h20 h21
    vshufi32x4         m25, m2, m16, q3232  ; f22 f23 h22 h23
    vinserti32x8       m26, m4, ym18, 1     ; f40 f41 h40 h41
    vshufi32x4         m27, m4, m18, q3232  ; f42 f43 h42 h43
    vinserti32x8       m28, m6, ym20, 1     ; f60 f61 h60 h61
    vshufi32x4         m29, m6, m20, q3232  ; f62 f63 h62 h63
    pmulhrsw            m0, m13, [r4-64*20]
    pmulhrsw            m1, m13, [r4-64*19]
    pmulhrsw            m2, m13, [r4-64*18]
    pmulhrsw            m3, m13, [r4-64*17]
    pmulhrsw            m4, m13, [r4-64*16]
    pmulhrsw            m5, m13, [r4-64*15]
    pmulhrsw            m6, m13, [r4-64*14]
    pmulhrsw            m7, m13, [r4-64*13]
    pmulhrsw           m14, m13, [r4-64*29]
    pmulhrsw           m15, m13, [r4-64*30]
    pmulhrsw           m16, m13, [r4-64*31]
    pmulhrsw           m17, m13, [r4-64*32]
    pmulhrsw           m18, m13, [r4-64*33]
    pmulhrsw           m19, m13, [r4-64*34]
    pmulhrsw           m20, m13, [r4-64*35]
    pmulhrsw           m21, m13, [r4-64*36]
    call .transpose_2x8x8_lo
    mova       [r4-64*20], m1
    mova       [r4-64*19], m3
    mova       [r4-64*18], m5
    mova       [r4-64*17], m7
    mova       [r4-64*16], m15
    mova       [r4-64*15], m17
    mova       [r4-64*14], m19
    mova       [r4-64*13], m21
    vinserti32x8        m1, m4, ym18, 1     ; b40 b41 d40 d41
    vshufi32x4          m5, m4, m18, q3232  ; b42 b43 d42 d43
    vshufi32x4          m4, m0, m14, q3232  ; b02 b03 d02 d03
    vinserti32x8        m0, ym14, 1         ; b00 b01 d00 d01
    vinserti32x8       m14, m2, ym16, 1     ; b20 b21 d20 d21
    vshufi32x4         m18, m2, m16, q3232  ; b22 b23 d22 d23
    vinserti32x8       m15, m6, ym20, 1     ; b60 b61 d60 d61
    vshufi32x4         m19, m6, m20, q3232  ; b62 b63 d62 d63
    vshufi32x4          m2, m0, m22, q3131  ;  8
    vshufi32x4          m0, m22, q2020      ;  0
    vshufi32x4          m3, m1, m26, q3131  ; 12
    vshufi32x4          m1, m26, q2020      ;  4
    vshufi32x4          m6, m4, m23, q3131  ; 24
    vshufi32x4          m4, m23, q2020      ; 16
    vshufi32x4          m7, m5, m27, q3131  ; 28
    vshufi32x4          m5, m27, q2020      ; 20
    call m(inv_txfm_add_dct_dct_32x8_8bpc).main
    vshufi32x4         m16, m14, m24, q3131 ; 10
    vshufi32x4         m14, m24, q2020      ;  2
    vshufi32x4         m17, m15, m28, q3131 ; 14
    vshufi32x4         m15, m28, q2020      ;  6
    vshufi32x4         m20, m18, m25, q3131 ; 26
    vshufi32x4         m18, m25, q2020      ; 18
    vshufi32x4         m21, m19, m29, q3131 ; 30
    vshufi32x4         m19, m29, q2020      ; 22
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf
    mova               m22, [r4-64*20]
    mova               m26, [r4-64*16]
    mova               m23, [r4-64*19]
    mova               m27, [r4-64*15]
    mova               m24, [r4-64*18]
    mova               m28, [r4-64*14]
    mova               m25, [r4-64*17]
    mova               m29, [r4-64*13]
    mova        [r4-64*20], m14
    mova        [r4-64*19], m15
    mova        [r4-64*18], m16
    mova        [r4-64*17], m17
    mova        [r4-64*16], m18
    mova        [r4-64*15], m19
    mova        [r4-64*14], m20
    mova        [r4-64*13], m21
    mova               m19, [r4-64*12]
    mova               m11, [r4-64* 8]
    mova               m20, [r4-64*11]
    mova               m12, [r4-64* 7]
    mova               m21, [r4-64*10]
    mova                m8, [r4-64* 6]
    mova                m9, [r4-64* 9]
    mova               m18, [r4-64* 5]
    vshufi32x4         m14, m22, m26, q3232 ; b12 b13 d12 d13
    vinserti32x8       m22, ym26, 1         ; b10 b11 d10 d11
    vshufi32x4         m15, m23, m27, q3232 ; b32 b33 d32 d33
    vinserti32x8       m23, ym27, 1         ; b30 b31 d30 d31
    vshufi32x4         m16, m24, m28, q3232 ; b52 b53 d52 d53
    vinserti32x8       m24, ym28, 1         ; b50 b51 d50 d51
    vshufi32x4         m17, m25, m29, q3232 ; b72 b73 d72 d73
    vinserti32x8       m25, ym29, 1         ; b70 b71 d70 d71
    vinserti32x8       m27, m19, ym11, 1    ; f10 f11 h10 h11
    vshufi32x4         m19, m11, q3232      ; f12 f13 h12 h13
    vinserti32x8       m28, m20, ym12, 1    ; f30 f31 h30 h31
    vshufi32x4         m20, m12, q3232      ; f32 f33 h32 h33
    vinserti32x8       m29, m21, ym8, 1     ; f50 f51 h50 h51
    vshufi32x4         m21, m8, q3232       ; f52 f53 h52 h53
    vinserti32x8        m8, m9, ym18, 1     ; f70 f71 h70 h71
    vshufi32x4          m9, m18, q3232      ; f72 f73 h72 h73
    vshufi32x4         m26, m22, m27, q3131 ;  9
    vshufi32x4         m22, m27, q2020      ;  1
    vshufi32x4         m27, m23, m28, q3131 ; 11
    vshufi32x4         m23, m28, q2020      ;  3
    vshufi32x4         m28, m24, m29, q3131 ; 13
    vshufi32x4         m24, m29, q2020      ;  5
    vshufi32x4         m29, m25, m8, q3131  ; 15
    vshufi32x4         m25, m8, q2020       ;  7
    vshufi32x4         m18, m14, m19, q3131 ; 25
    vshufi32x4         m14, m19, q2020      ; 17
    vshufi32x4         m19, m15, m20, q3131 ; 27
    vshufi32x4         m15, m20, q2020      ; 19
    vshufi32x4         m20, m16, m21, q3131 ; 29
    vshufi32x4         m16, m21, q2020      ; 21
    vshufi32x4         m21, m17, m9, q3131  ; 31
    vshufi32x4         m17, m9, q2020       ; 23
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf
    jmp .end
.fast: ; bottom/right halves are zero
    {evex}vpmulhrsw     ym8, ym23, [cq+64* 4]
    {evex}vpmulhrsw     xm1, xm23, [cq+64*12]
    mova                m28, [o(dup16_perm)]
    {evex}vpmulhrsw     ym7, ym23, [cq+64* 8]
          vpmulhrsw    ym22, ym23, [cq+64* 0]
    vpermb               m8, m28, m8
    vpermb              ym1, ym28, ym1
    vpermb               m7, m28, m7
    pmovzxwd             m9, ym22
    pslld                m9, 16
    call m(idct_16x16_internal_8bpc).main_fast2
    {evex}vpmulhrsw    ym21, ym23, [cq+64* 2]
    {evex}vpmulhrsw    xm15, xm23, [cq+64*14]
    {evex}vpmulhrsw    xm18, xm23, [cq+64*10]
    {evex}vpmulhrsw    ym14, ym23, [cq+64* 6]
    vpermb              m21, m28, m21
    punpcklwd          xm15, xm15
    vpermb             ym18, ym28, ym18
    vpermb              m14, m28, m14
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast2
          vpmulhrsw    ym22, ym23, [cq+64* 1]
    {evex}vpmulhrsw    xm29, xm23, [cq+64*15]
    {evex}vpmulhrsw    xm26, xm23, [cq+64* 9]
    {evex}vpmulhrsw    ym25, ym23, [cq+64* 7]
    {evex}vpmulhrsw    ym24, ym23, [cq+64* 5]
    {evex}vpmulhrsw    xm27, xm23, [cq+64*11]
    {evex}vpmulhrsw     xm8, xm23, [cq+64*13]
    {evex}vpmulhrsw    ym23,       [cq+64* 3]
    vpermb              m22, m28, m22
    punpcklwd          xm29, xm29
    vpermb             ym26, ym28, ym26
    vpermb              m25, m28, m25
    mova         [cq+64* 0], m14
    mova         [cq+64* 1], m15
    mova         [cq+64* 2], m16
    mova         [cq+64* 3], m17
    REPX {vpermb x, m28, x}, m24, m27, m23
    punpcklwd          xm28, xm8, xm8
    mova         [cq+64* 4], m18
    mova         [cq+64* 5], m19
    mova         [cq+64* 6], m20
    mova         [cq+64* 7], m21
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_oddhalf_fast
    mov                  r4, rsp
    vpbroadcastd        m13, [o(pw_16384)]
    mova         [r4+64*16], m4
    mova         [r4+64*17], m5
    mova         [r4+64*18], m6
    mova         [r4+64*19], m7
    mova         [r4+64*28], m26
    mova         [r4+64*29], m27
    mova         [r4+64*30], m28
    mova         [r4+64*31], m29
    call m(inv_txfm_add_dct_dct_64x16_8bpc).pass1_end
    mova         [r4+64*20], m22
    mova         [r4+64*21], m23
    mova         [r4+64*22], m24
    mova         [r4+64*23], m25
    mova         [r4+64*24], m26
    mova         [r4+64*25], m27
    mova         [r4+64*26], m28
    mova         [r4+64*27], m29
    call .pass2_fast
    mova         [cq+64* 8], m14
    mova         [cq+64* 9], m15
    mova         [cq+64*10], m16
    mova         [cq+64*11], m17
    mova         [cq+64*12], m18
    mova         [cq+64*13], m19
    mova         [cq+64*14], m20
    mova         [cq+64*15], m21
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast
    mova         [cq+64* 0], m0
    mova         [cq+64* 1], m1
    mova         [cq+64* 2], m2
    mova         [cq+64* 3], m3
    mova         [cq+64* 4], m4
    mova         [cq+64* 5], m5
    mova         [cq+64* 6], m6
    mova         [cq+64* 7], m7
    pmulhrsw             m0, m13, [r4+64*16]
    pmulhrsw             m1, m13, [r4+64*17]
    pmulhrsw             m2, m13, [r4+64*18]
    pmulhrsw             m3, m13, [r4+64*19]
    pmulhrsw             m4, m13, [r4+64*20]
    pmulhrsw             m5, m13, [r4+64*21]
    pmulhrsw             m6, m13, [r4+64*22]
    pmulhrsw             m7, m13, [r4+64*23]
    mova         [cq+64*16], m14
    mova         [cq+64*17], m15
    mova         [cq+64*18], m16
    mova         [cq+64*19], m17
    mova         [cq+64*20], m18
    mova         [cq+64*21], m19
    mova         [cq+64*22], m20
    mova         [cq+64*23], m21
    pmulhrsw            m14, m13, [r4+64*24]
    pmulhrsw            m15, m13, [r4+64*25]
    pmulhrsw            m16, m13, [r4+64*26]
    pmulhrsw            m17, m13, [r4+64*27]
    pmulhrsw            m18, m13, [r4+64*28]
    pmulhrsw            m19, m13, [r4+64*29]
    pmulhrsw            m20, m13, [r4+64*30]
    pmulhrsw            m21, m13, [r4+64*31]
    mova         [cq+64*24], m22
    mova         [cq+64*25], m23
    mova         [cq+64*26], m24
    mova         [cq+64*27], m25
    mova         [cq+64*28], m26
    mova         [cq+64*29], m27
    mova         [cq+64*30], m28
    mova         [cq+64*31], m29
    call m(inv_txfm_add_dct_dct_64x16_8bpc).transpose_round
    call .pass2_fast
    mova         [r4+64*16], m14
    mova         [r4+64*17], m15
    mova         [r4+64*18], m16
    mova         [r4+64*19], m17
    mova         [r4+64*20], m18
    mova         [r4+64*21], m19
    mova         [r4+64*22], m20
    mova         [r4+64*23], m21
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast
.end:
    vpbroadcastd        m13, [o(pw_2048)]
    lea                  r5, [strideq*3]
    pxor                m12, m12
    lea                  r3, [dstq+r5*8]
    lea                  r6, [strideq+r5] ; stride*4
    add                  r3, r6           ; dst+stride*28
%macro IDCT_64x32_END 5 ; src16, src32, mem, off_lo, off_hi
    mova                m11, [cq+64*(   %3)] ;  0
    mova                 m9, [cq+64*(31-%3)] ; 31
%if %3 >= 8
    mova                m%1, [rsp+64*(%1+16)]
%endif
    mova                m10, [dstq+%4]
    paddsw               m8, m11, m9
    psubsw              m11, m9
    paddsw               m9, m%1, m%2
    psubsw              m%1, m%2
    punpcklbw           m%2, m10, m12
    punpckhbw           m10, m12
    pmulhrsw             m8, m13
    pmulhrsw             m9, m13
    paddw                m8, m%2
    paddw                m9, m10
    mova                m10, [r3+%5]
    pmulhrsw            m11, m13
    pmulhrsw            m%1, m13
    mova    [cq+64*(   %3)], m12
    mova    [cq+64*(31-%3)], m12
    punpcklbw           m%2, m10, m12
    punpckhbw           m10, m12
    packuswb             m8, m9
    paddw               m11, m%2
    paddw               m%1, m10
    packuswb            m11, m%1
    mova          [dstq+%4], m8
    mova          [r3  +%5], m11
%if %3 == 3 || %3 == 7 || %3 == 11
    add                dstq, r6
    sub                  r3, r6
%endif
%endmacro
    IDCT_64x32_END        0, 29,  0, strideq*0, r5
    IDCT_64x32_END        1, 28,  1, strideq*1, strideq*2
    IDCT_64x32_END        2, 27,  2, strideq*2, strideq*1
    IDCT_64x32_END        3, 26,  3, r5       , strideq*0
    IDCT_64x32_END        4, 25,  4, strideq*0, r5
    IDCT_64x32_END        5, 24,  5, strideq*1, strideq*2
    IDCT_64x32_END        6, 23,  6, strideq*2, strideq*1
    IDCT_64x32_END        7, 22,  7, r5       , strideq*0
    IDCT_64x32_END        0, 21,  8, strideq*0, r5
    IDCT_64x32_END        1, 20,  9, strideq*1, strideq*2
    IDCT_64x32_END        2, 19, 10, strideq*2, strideq*1
    IDCT_64x32_END        3, 18, 11, r5       , strideq*0
    IDCT_64x32_END        4, 17, 12, strideq*0, r5
    IDCT_64x32_END        5, 16, 13, strideq*1, strideq*2
    IDCT_64x32_END        6, 15, 14, strideq*2, strideq*1
    IDCT_64x32_END        7, 14, 15, r5       , strideq*0
    RET
ALIGN function_align
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 32
    imul                r6d, 181
    add                 r6d, 128
    sar                 r6d, 8
    imul                r6d, 181
    add                 r6d, 128+256
    sar                 r6d, 8+1
    jmp m(inv_txfm_add_dct_dct_64x16_8bpc).dconly2
ALIGN function_align
.pass1_end_part1:
%macro IDCT_64x32_PASS1_END 3 ; src16, src32, src64
%if %1 != %3
    mova                m%1, [cq+64*%1]
%endif
    mova                 m9, [r4+64*(%3-36)] ; idct64 32+n
    mova                m11, [r4+64*(-5-%3)] ; idct64 63-n
    psubsw               m8, m%1, m%2        ; idct32 31-n
    paddsw              m%1, m%2             ; idct32  0+n
%if %1 == %3
    psubsw              m%2, m8, m9   ; out 32+n e
    paddsw               m8, m9       ; out 31-n d
    psubsw               m9, m%1, m11 ; out 63-n h
    paddsw              m%1, m11      ; out  0+n a
%else
    paddsw              m%2, m8, m9   ; out 23-n c
    psubsw               m8, m9       ; out 40+n f
    paddsw               m9, m%1, m11 ; out  8+n b
    psubsw              m%1, m11      ; out 55-n g
%endif
    mova   [r4+64*(%3-36)], m8
    mova   [r4+64*(-5-%3)], m9
%endmacro
    IDCT_64x32_PASS1_END  0, 29,  0
    IDCT_64x32_PASS1_END  1, 28,  1
    IDCT_64x32_PASS1_END  2, 27,  2
    IDCT_64x32_PASS1_END  3, 26,  3
    IDCT_64x32_PASS1_END  4, 25,  4
    IDCT_64x32_PASS1_END  5, 24,  5
    IDCT_64x32_PASS1_END  6, 23,  6
    IDCT_64x32_PASS1_END  7, 22,  7
.transpose_2x8x8_hi: ; m0-m7 + m22-m29 (inverted)
    punpcklwd            m8, m25, m24 ; e0 f0 e1 f1 e2 f2 e3 f3
    punpckhwd           m25, m24      ; e4 f4 e5 f5 e6 f6 e7 f7
    punpcklwd           m24, m23, m22 ; g0 h0 g1 h1 g2 h2 g3 h3
    punpckhwd           m23, m22      ; g4 h4 g5 h5 g6 h6 g7 h7
    punpcklwd           m22, m29, m28 ; a0 b0 a1 b1 a2 b2 a3 b3
    punpckhwd           m29, m28      ; a4 b4 a5 b5 a6 b6 a7 b7
    punpcklwd           m28, m27, m26 ; c0 d0 c1 d1 c2 d2 c3 d3
    punpckhwd           m27, m26      ; c4 d4 c5 d5 c6 d6 c7 d7
    punpckldq           m26, m29, m27 ; a4 b4 c4 d4 a5 b5 c5 d5
    punpckhdq           m29, m27      ; a6 b6 c6 d6 a7 b7 c7 d7
    punpckldq           m27, m8, m24  ; e0 f0 g0 h0 e1 f1 g1 h1
    punpckhdq            m8, m24      ; e2 f2 g2 h2 e3 f3 g3 h3
    punpckhdq           m24, m22, m28 ; a2 b2 c2 d2 a3 b3 c3 d3
    punpckldq           m22, m28      ; a0 b0 c0 d0 a1 b1 c1 d1
    punpckldq           m28, m25, m23 ; e4 f4 g4 h4 e5 f5 g5 h5
    punpckhdq           m25, m23      ; e6 f6 g6 h6 e7 f7 g7 h7
    punpckhqdq          m23, m22, m27 ;  1 23
    punpcklqdq          m22, m27      ;  0 22
    punpckhqdq          m27, m26, m28 ;  5 27
    punpcklqdq          m26, m28      ;  4 26
    punpcklqdq          m28, m29, m25 ;  6 28
    punpckhqdq          m29, m25      ;  7 29
    punpckhqdq          m25, m24, m8  ;  3 25
    punpcklqdq          m24, m8       ;  2 24
.transpose_8x8:
    punpckhwd            m8, m4, m5
    punpcklwd            m4, m5
    punpckhwd            m5, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m6, m7
    punpcklwd            m6, m7
    punpckhwd            m7, m2, m3
    punpcklwd            m2, m3
    punpckhdq            m3, m0, m2
    punpckldq            m0, m2
    punpckldq            m2, m4, m6
    punpckhdq            m4, m6
    punpckhdq            m6, m5, m7
    punpckldq            m5, m7
    punpckldq            m7, m8, m1
    punpckhdq            m8, m1
    punpckhqdq           m1, m0, m2
    punpcklqdq           m0, m2
    punpcklqdq           m2, m3, m4
    punpckhqdq           m3, m4
    punpcklqdq           m4, m5, m7
    punpckhqdq           m5, m7
    punpckhqdq           m7, m6, m8
    punpcklqdq           m6, m8
    ret
.pass1_end_part2:
    IDCT_64x32_PASS1_END  0, 21,  8
    IDCT_64x32_PASS1_END  1, 20,  9
    IDCT_64x32_PASS1_END  2, 19, 10
    IDCT_64x32_PASS1_END  3, 18, 11
    IDCT_64x32_PASS1_END  4, 17, 12
    IDCT_64x32_PASS1_END  5, 16, 13
    IDCT_64x32_PASS1_END  6, 15, 14
    IDCT_64x32_PASS1_END  7, 14, 15
.transpose_2x8x8_lo: ; m0-m7 (inverted) + m14-m21
    punpcklwd            m8, m3, m2
    punpckhwd            m3, m2
    punpcklwd            m2, m1, m0
    punpckhwd            m1, m0
    punpcklwd            m0, m7, m6
    punpckhwd            m7, m6
    punpcklwd            m6, m5, m4
    punpckhwd            m5, m4
    punpckldq            m4, m7, m5
    punpckhdq            m7, m5
    punpckldq            m5, m8, m2
    punpckhdq            m8, m2
    punpckhdq            m2, m0, m6
    punpckldq            m0, m6
    punpckldq            m6, m3, m1
    punpckhdq            m3, m1
    punpckhqdq           m1, m0, m5
    punpcklqdq           m0, m5
    punpckhqdq           m5, m4, m6
    punpcklqdq           m4, m6
    punpcklqdq           m6, m7, m3
    punpckhqdq           m7, m3
    punpckhqdq           m3, m2, m8
    punpcklqdq           m2, m8
    punpckhwd            m8, m18, m19
    punpcklwd           m18, m19
    punpckhwd           m19, m14, m15
    punpcklwd           m14, m15
    punpckhwd           m15, m20, m21
    punpcklwd           m20, m21
    punpckhwd           m21, m16, m17
    punpcklwd           m16, m17
    punpckhdq           m17, m14, m16
    punpckldq           m14, m16
    punpckldq           m16, m18, m20
    punpckhdq           m18, m20
    punpckhdq           m20, m19, m21
    punpckldq           m19, m21
    punpckldq           m21, m8, m15
    punpckhdq            m8, m15
    punpckhqdq          m15, m14, m16
    punpcklqdq          m14, m16
    punpcklqdq          m16, m17, m18
    punpckhqdq          m17, m18
    punpcklqdq          m18, m19, m21
    punpckhqdq          m19, m21
    punpckhqdq          m21, m20, m8
    punpcklqdq          m20, m8
    ret
.pass2_fast:
    vshufi32x4          m24, m9, m15, q3131  ;  5
    vshufi32x4          m22, m9, m15, q2020  ;  1
    vshufi32x4          m15, m1, m16, q3131  ;  6
    vshufi32x4          m14, m1, m16, q2020  ;  2
    vshufi32x4           m1, m0, m3, q3131   ;  4
    vshufi32x4           m0, m3, q2020       ;  0
    vshufi32x4           m3, m8, m2, q3131   ; 12
    vshufi32x4           m2, m8, m2, q2020   ;  8
    vshufi32x4          m25, m11, m17, q3131 ;  7
    vshufi32x4          m23, m11, m17, q2020 ;  3
    vshufi32x4          m17, m5, m19, q3131  ; 14
    vshufi32x4          m16, m5, m19, q2020  ; 10
    vshufi32x4          m29, m6, m20, q3131  ; 15
    vshufi32x4          m27, m6, m20, q2020  ; 11
    vshufi32x4          m28, m4, m18, q3131  ; 13
    vshufi32x4          m26, m4, m18, q2020  ;  9
    jmp m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast

cglobal inv_txfm_add_dct_dct_64x64_8bpc, 4, 7, 0, dst, stride, c, eob
    lea                  r5, [o_base]
    test               eobd, eobd
    jz .dconly
    PROLOGUE              0, 7, 30, 64*96, dst, stride, c, eob
%undef cmp
    cmp                eobd, 136
    jb .fast
    mova                 m0, [cq+64* 1]
    mova                 m1, [cq+64*31]
    mova                 m2, [cq+64*17]
    mova                 m3, [cq+64*15]
    vpbroadcastd        m10, [o(pd_2048)]
    mov                  r4, rsp
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    mova                 m0, [cq+64* 7]
    mova                 m1, [cq+64*25]
    mova                 m2, [cq+64*23]
    mova                 m3, [cq+64* 9]
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    mova                 m0, [cq+64* 5]
    mova                 m1, [cq+64*27]
    mova                 m2, [cq+64*21]
    mova                 m3, [cq+64*11]
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    mova                 m0, [cq+64* 3]
    mova                 m1, [cq+64*29]
    mova                 m2, [cq+64*19]
    mova                 m3, [cq+64*13]
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part2
    mova                 m0, [cq+64* 0]
    mova                 m1, [cq+64* 8]
    mova                 m2, [cq+64*16]
    mova                 m3, [cq+64*24]
    mova                m14, [cq+64* 4]
    mova                m15, [cq+64*12]
    mova                m16, [cq+64*20]
    mova                m17, [cq+64*28]
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast
    mova                m22, [cq+64* 2]
    mova                m29, [cq+64*30]
    mova                m26, [cq+64*18]
    mova                m25, [cq+64*14]
    mova                m24, [cq+64*10]
    mova                m27, [cq+64*22]
    mova                m28, [cq+64*26]
    mova                m23, [cq+64* 6]
    mova         [cq+64* 0], m14
    mova         [cq+64* 1], m15
    mova         [cq+64* 2], m16
    mova         [cq+64* 3], m17
    mova         [cq+64* 4], m18
    mova         [cq+64* 5], m19
    mova         [cq+64* 6], m20
    mova         [cq+64* 7], m21
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast
    vpbroadcastd        m13, [o(pw_8192)]
    call m(inv_txfm_add_dct_dct_64x32_8bpc).pass1_end_part1
    mova         [r4+64*36], m1
    mova         [r4+64*37], m3
    mova         [r4+64*38], m5
    mova         [r4+64*39], m7
    mova         [r4+64*44], m23
    mova         [r4+64*45], m25
    mova         [r4+64*46], m27
    mova         [r4+64*47], m29
    pmulhrsw            m23, m13, m0 ; a0
    pmulhrsw            m25, m13, m2 ; a2
    pmulhrsw            m27, m13, m4 ; a4
    pmulhrsw            m29, m13, m6 ; a6
    call m(inv_txfm_add_dct_dct_64x32_8bpc).pass1_end_part2
    lea                  r6, [r4-64*4]
    add                  r4, 64*28
    call .pass2_end
    mov                  r4, rsp
    mova                 m0, [r4+64*23]
    mova                 m1, [r4+64*22]
    mova                 m2, [r4+64*21]
    mova                 m3, [r4+64*20]
    mova                 m4, [r4+64*19]
    mova                 m5, [r4+64*18]
    mova                 m6, [r4+64*17]
    mova                 m7, [r4+64*16]
    mova                m22, [r4+64*15]
    mova                m23, [r4+64*14]
    mova                m24, [r4+64*13]
    mova                m25, [r4+64*12]
    mova                m26, [r4+64*11]
    mova                m27, [r4+64*10]
    mova                m28, [r4+64* 9]
    mova                m29, [r4+64* 8]
    call m(inv_txfm_add_dct_dct_64x32_8bpc).transpose_2x8x8_hi
    vpbroadcastd        m13, [o(pw_8192)]
    mova         [r4+64* 8], m1
    mova         [r4+64* 9], m3
    mova         [r4+64*10], m5
    mova         [r4+64*11], m7
    mova         [r4+64*16], m23
    mova         [r4+64*17], m25
    mova         [r4+64*18], m27
    mova         [r4+64*19], m29
    pmulhrsw            m23, m13, m0 ; b0
    pmulhrsw            m25, m13, m2 ; b2
    pmulhrsw            m27, m13, m4 ; b4
    pmulhrsw            m29, m13, m6 ; b6
    mova                 m0, [r4+64*31]
    mova                 m1, [r4+64*30]
    mova                 m2, [r4+64*29]
    mova                 m3, [r4+64*28]
    mova                 m4, [r4+64*27]
    mova                 m5, [r4+64*26]
    mova                 m6, [r4+64*25]
    mova                 m7, [r4+64*24]
    mova                m14, [r4+64* 7]
    mova                m15, [r4+64* 6]
    mova                m16, [r4+64* 5]
    mova                m17, [r4+64* 4]
    mova                m18, [r4+64* 3]
    mova                m19, [r4+64* 2]
    mova                m20, [r4+64* 1]
    mova                m21, [r4+64* 0]
    call m(inv_txfm_add_dct_dct_64x32_8bpc).transpose_2x8x8_lo
    mov                  r6, cq
    call .pass2_end
    jmp .end
.fast: ; bottom/right halves are zero
    mova                m28, [o(dup16_perm)]
    pmovzxwd             m9,       [cq+64* 0]
    vpermb               m8, m28,  [cq+64* 4]
    vpermb              ym1, ym28, [cq+64*12]
    vpermb               m7, m28,  [cq+64* 8]
    pslld                m9, 16
    call m(idct_16x16_internal_8bpc).main_fast2
    vpermb              m21, m28,  [cq+64* 2]
    vpermb             ym15, ym28, [cq+64*14]
    vpermb             ym18, ym28, [cq+64*10]
    vpermb              m14, m28,  [cq+64* 6]
    call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast2
    vpermb              m22, m28,  [cq+64* 1]
    vpermb             ym29, ym28, [cq+64*15]
    vpermb             ym26, ym28, [cq+64* 9]
    vpermb              m25, m28,  [cq+64* 7]
    vpermb              m24, m28,  [cq+64* 5]
    vpermb             ym27, ym28, [cq+64*11]
    vpermb              m23, m28,  [cq+64* 3]
    vpermb             ym28, ym28, [cq+64*13]
    mova         [cq+64* 0], m14
    mova         [cq+64* 1], m15
    mova         [cq+64* 2], m16
    mova         [cq+64* 3], m17
    mova         [cq+64* 4], m18
    mova         [cq+64* 5], m19
    mova         [cq+64* 6], m20
    mova         [cq+64* 7], m21
    call m(inv_txfm_add_dct_dct_16x64_8bpc).main_oddhalf_fast
    vpbroadcastd        m13, [o(pw_8192)]
    mova         [cq+64*16], m4
    mova         [cq+64*17], m5
    mova         [cq+64*18], m6
    mova         [cq+64*19], m7
    mova         [cq+64*28], m26
    mova         [cq+64*29], m27
    mova         [cq+64*30], m28
    mova         [cq+64*31], m29
    call m(inv_txfm_add_dct_dct_64x16_8bpc).pass1_end
    mova         [cq+64*20], m22
    mova         [cq+64*21], m23
    mova         [cq+64*22], m24
    mova         [cq+64*23], m25
    mova         [cq+64*24], m26
    mova         [cq+64*25], m27
    mova         [cq+64*26], m28
    mova         [cq+64*27], m29
    lea                  r4, [rsp+64*64]
    lea                  r3, [rsp+64*32]
    call .pass2_fast
    pmulhrsw             m0, m13, [cq+64*16]
    pmulhrsw             m1, m13, [cq+64*17]
    pmulhrsw             m2, m13, [cq+64*18]
    pmulhrsw             m3, m13, [cq+64*19]
    pmulhrsw             m4, m13, [cq+64*20]
    pmulhrsw             m5, m13, [cq+64*21]
    pmulhrsw             m6, m13, [cq+64*22]
    pmulhrsw             m7, m13, [cq+64*23]
    pmulhrsw            m14, m13, [cq+64*24]
    pmulhrsw            m15, m13, [cq+64*25]
    pmulhrsw            m16, m13, [cq+64*26]
    pmulhrsw            m17, m13, [cq+64*27]
    pmulhrsw            m18, m13, [cq+64*28]
    pmulhrsw            m19, m13, [cq+64*29]
    pmulhrsw            m20, m13, [cq+64*30]
    pmulhrsw            m21, m13, [cq+64*31]
    call m(inv_txfm_add_dct_dct_64x16_8bpc).transpose_round
    mov                  r4, rsp
    mov                  r3, cq
    call .pass2_fast
.end:
    vpbroadcastd        m17, [o(pw_2048)]
    lea                  r5, [strideq*8]
    mov                  r3, dstq
    pxor                m16, m16
    sub                  r4, 64*5 ; rsp+64*31
    mov                  r6, rsp
.end_loop:
    mova                 m2, [r6+64*32] ; idct16 0+n  lo
    mova                 m7, [r6+64*48] ; idct32 31-n lo
    mova                 m6, [cq+64* 0] ; idct16 0+n  hi
    mova                 m0, [cq+64*16] ; idct32 31-n hi
    mova                 m4, [r4+64*64] ; idct64 63-n lo
    mova                 m1, [r4+64* 0] ; idct64 63-n hi
    mova                 m5, [r6+64*64] ; idct64 32+n lo
    mova                 m8, [r6+64* 0] ; idct64 32+n hi
    sub                  r3, strideq
    paddsw               m3, m2, m7     ; idct32  0+n lo
    mova                m12, [dstq+r5*0]
    psubsw               m2, m7         ; idct32 31-n lo
    mova                m15, [r3  +r5*8]
    paddsw               m7, m6, m0     ; idct32  0+n hi
    mova                m13, [r3  +r5*4]
    psubsw               m6, m0         ; idct32 31-n hi
    mova                m14, [dstq+r5*4]
    paddsw               m0, m3, m4     ; out  0+n lo
    add                  r6, 64
    psubsw               m3, m4         ; out 63-n lo
    sub                  r4, 64
    paddsw               m4, m7, m1     ; out  0+n hi
    mova         [cq+64* 0], m16
    psubsw               m7, m1         ; out 63-n hi
    mova         [cq+64*16], m16
    paddsw               m1, m2, m5     ; out 31-n lo
    add                  cq, 64
    psubsw               m2, m5         ; out 32+n lo
    paddsw               m5, m6, m8     ; out 31-n hi
    psubsw               m6, m8         ; out 32+n hi
    pmulhrsw             m0, m17
    punpcklbw            m8, m12, m16
    pmulhrsw             m4, m17
    punpckhbw           m12, m16
    pmulhrsw             m3, m17
    punpcklbw           m11, m15, m16
    pmulhrsw             m7, m17
    punpckhbw           m15, m16
    pmulhrsw             m1, m17
    punpcklbw            m9, m13, m16
    pmulhrsw             m5, m17
    punpckhbw           m13, m16
    pmulhrsw             m2, m17
    punpcklbw           m10, m14, m16
    pmulhrsw             m6, m17
    punpckhbw           m14, m16
    paddw                m0, m8
    paddw                m4, m12
    packuswb             m0, m4
    paddw                m3, m11
    paddw                m7, m15
    packuswb             m3, m7
    paddw                m1, m9
    paddw                m5, m13
    packuswb             m1, m5
    paddw                m2, m10
    paddw                m6, m14
    packuswb             m2, m6
    mova        [dstq+r5*0], m0
    mova        [r3  +r5*8], m3
    mova        [r3  +r5*4], m1
    mova        [dstq+r5*4], m2
    add                dstq, strideq
    cmp                  r6, r4
    jb .end_loop
    RET
.dconly:
    movsx               r6d, word [cq]
    mov                [cq], eobd
    or                  r3d, 64
    jmp m(inv_txfm_add_dct_dct_64x16_8bpc).dconly
ALIGN function_align
.pass2_end:
    REPX  {pmulhrsw x, m13}, m22, m24, m26, m28, m14, m16, m18, m20, m0, m2, m4, m6
    mova         [r4+64*20], m1
    mova         [r4+64*21], m3
    mova         [r4+64*22], m5
    mova         [r4+64*23], m7
    vinserti32x8         m1, m23, ym14, 1    ; a00 a01 c00 c01
    vshufi32x4           m3, m23, m14, q3232 ; a02 a03 c02 c03
    vinserti32x8         m5, m22, ym0, 1     ; e00 e01 g00 g01
    vshufi32x4          m14, m22, m0, q3232  ; e02 e03 g02 g03
    mova         [r4+64*12], m15
    mova         [r4+64*13], m17
    mova         [r4+64*14], m19
    mova         [r4+64*15], m21
    vinserti32x8        m15, m27, ym18, 1    ; a40 a41 c40 c41
    vshufi32x4          m17, m27, m18, q3232 ; a42 a43 c42 c43
    vinserti32x8        m18, m26, ym4, 1     ; e40 e41 g40 g41
    vshufi32x4          m19, m26, m4, q3232  ; e42 e43 g42 g43
    vinserti32x8        m22, m25, ym16, 1    ; a20 a21 c20 c21
    vshufi32x4          m26, m25, m16, q3232 ; a22 a23 c22 c23
    vinserti32x8        m25, m24, ym2, 1     ; e20 e21 g20 g21
    vshufi32x4          m27, m24, m2, q3232  ; e22 e23 g22 g23
    vinserti32x8        m23, m29, ym20, 1    ; a60 a61 c60 c61
    vshufi32x4          m29, m20, q3232      ; a62 a63 c62 c63
    vshufi32x4          m13, m28, m6, q3232  ; e62 e63 g62 g63
    vinserti32x8        m28, ym6, 1          ; e60 e61 g60 g61
    vshufi32x4           m0, m1, m5, q2020   ;  0
    vshufi32x4           m1, m5, q3131       ;  8
    vshufi32x4           m2, m3, m14, q2020  ; 16
    vshufi32x4           m3, m14, q3131      ; 24
    vshufi32x4          m14, m15, m18, q2020 ;  4
    vshufi32x4          m15, m18, q3131      ; 12
    vshufi32x4          m16, m17, m19, q2020 ; 20
    vshufi32x4          m17, m19, q3131      ; 28
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast
    vshufi32x4          m24, m22, m25, q3131 ; 10
    vshufi32x4          m22, m25, q2020      ;  2
    vshufi32x4          m25, m23, m28, q3131 ; 14
    vshufi32x4          m23, m28, q2020      ;  6
    vshufi32x4          m28, m26, m27, q3131 ; 26
    vshufi32x4          m26, m27, q2020      ; 18
    vshufi32x4          m27, m29, m13, q2020 ; 22
    vshufi32x4          m29, m13, q3131      ; 30
    mova         [r6+64* 0], m0
    mova         [r6+64* 1], m1
    mova         [r6+64* 2], m2
    mova         [r6+64* 3], m3
    mova         [r6+64* 4], m4
    mova         [r6+64* 5], m5
    mova         [r6+64* 6], m6
    mova         [r6+64* 7], m7
    mova         [r6+64* 8], m14
    mova         [r6+64* 9], m15
    mova         [r6+64*10], m16
    mova         [r6+64*11], m17
    mova         [r6+64*12], m18
    mova         [r6+64*13], m19
    mova         [r6+64*14], m20
    mova         [r6+64*15], m21
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast
    vpbroadcastd        m13, [o(pw_8192)]
    mova         [r6+64*16], m29
    mova         [r6+64*17], m28
    mova         [r6+64*18], m27
    mova         [r6+64*19], m26
    mova         [r6+64*20], m25
    mova         [r6+64*21], m24
    mova         [r6+64*22], m23
    mova         [r6+64*23], m22
    mova         [r6+64*24], m21
    mova         [r6+64*25], m20
    mova         [r6+64*26], m19
    mova         [r6+64*27], m18
    mova         [r6+64*28], m17
    mova         [r6+64*29], m16
    mova         [r6+64*30], m15
    mova         [r6+64*31], m14
    pmulhrsw            m15, m13, [r4+64* 8] ;  1  9 17 25
    pmulhrsw            m16, m13, [r4+64*12]
    pmulhrsw            m17, m13, [r4+64*16]
    pmulhrsw            m18, m13, [r4+64*20]
    pmulhrsw            m19, m13, [r4+64*11] ;  7 15 23 31
    pmulhrsw            m20, m13, [r4+64*15]
    pmulhrsw            m21, m13, [r4+64*19]
    pmulhrsw            m22, m13, [r4+64*23]
    vinserti32x8        m14, m15, ym16, 1 ; a1  a9  c1  c9
    vshufi32x4          m15, m16, q3232   ; a17 a25 c17 c25
    vinserti32x8        m16, m17, ym18, 1 ; e1  e9  g1  g9
    vshufi32x4          m17, m18, q3232   ; e17 e25 g17 g25
    pmulhrsw            m23, m13, [r4+64*10] ;  5 13 21 29
    pmulhrsw            m24, m13, [r4+64*14]
    pmulhrsw            m25, m13, [r4+64*18]
    pmulhrsw            m26, m13, [r4+64*22]
    vinserti32x8        m18, m19, ym20, 1 ; a7  a15 c7  c15
    vshufi32x4          m19, m20, q3232   ; a23 a31 c23 c31
    vinserti32x8        m20, m21, ym22, 1 ; e7  e15 g7  g15
    vshufi32x4          m21, m22, q3232   ; e23 e31 g23 g31
    pmulhrsw            m27, m13, [r4+64* 9] ;  3 11 19 27
    pmulhrsw            m28, m13, [r4+64*13]
    pmulhrsw            m29, m13, [r4+64*17]
    pmulhrsw            m13,      [r4+64*21]
    vshufi32x4           m0, m14, m16, q2020 ;  1
    vshufi32x4           m1, m19, m21, q3131 ; 31
    vshufi32x4           m2, m15, m17, q2020 ; 17
    vshufi32x4           m3, m18, m20, q3131 ; 15
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    vshufi32x4           m0, m18, m20, q2020 ;  7
    vshufi32x4           m1, m15, m17, q3131 ; 25
    vshufi32x4           m2, m19, m21, q2020 ; 23
    vshufi32x4           m3, m14, m16, q3131 ;  9
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    vinserti32x8        m22, m23, ym24, 1 ; a5  a13 c5  c13
    vshufi32x4          m23, m24, q3232   ; a21 a29 c21 c29
    vinserti32x8        m24, m25, ym26, 1 ; e5  e13 g5  g13
    vshufi32x4          m25, m26, q3232   ; e21 e29 g21 g29
    vinserti32x8        m26, m27, ym28, 1 ; a3  a11 c3  c11
    vshufi32x4          m27, m28, q3232   ; a19 a27 c19 c27
    vinserti32x8        m28, m29, ym13, 1 ; e3  e11 g3  g11
    vshufi32x4          m29, m13, q3232   ; e19 e17 g19 g27
    vshufi32x4           m0, m22, m24, q2020 ;  5
    vshufi32x4           m1, m27, m29, q3131 ; 27
    vshufi32x4           m2, m23, m25, q2020 ; 21
    vshufi32x4           m3, m26, m28, q3131 ; 11
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    vshufi32x4           m0, m26, m28, q2020 ;  3
    vshufi32x4           m1, m23, m25, q3131 ; 29
    vshufi32x4           m2, m27, m29, q2020 ; 19
    vshufi32x4           m3, m22, m24, q3131 ; 13
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1
    jmp m(inv_txfm_add_dct_dct_32x64_8bpc).main_part2
ALIGN function_align
.pass2_fast:
    vshufi32x4          m23, m1, m16, q3131  ;  6
    vshufi32x4          m22, m1, m16, q2020  ;  2
    vshufi32x4          m14, m0, m3, q3131   ;  4
    vshufi32x4          m26, m0, m3, q2020   ;  0
    vshufi32x4          m28, m9, m15, q3131  ;  5
    vshufi32x4           m0, m9, m15, q2020  ;  1
    vshufi32x4          m16, m11, m17, q3131 ;  7
    vshufi32x4          m29, m11, m17, q2020 ;  3
    vshufi32x4          m15, m8, m2, q3131   ; 12
    vshufi32x4          m27, m8, m2, q2020   ;  8
    vshufi32x4          m25, m5, m19, q3131  ; 14
    vshufi32x4          m24, m5, m19, q2020  ; 10
    vshufi32x4           m3, m6, m20, q3131  ; 15
    vshufi32x4          m19, m6, m20, q2020  ; 11
    vshufi32x4          m17, m4, m18, q3131  ; 13
    vshufi32x4          m18, m4, m18, q2020  ;  9
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1_fast
    mova                 m0, m16
    mova                 m3, m18
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1_fast
    mova                 m0, m28
    mova                 m3, m19
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1_fast
    mova                 m0, m29
    mova                 m3, m17
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part1_fast
    call m(inv_txfm_add_dct_dct_32x64_8bpc).main_part2
    mova                 m0, m26
    mova                 m1, m27
    call m(inv_txfm_add_dct_dct_32x16_8bpc).main_oddhalf_fast2
    mova         [r3+64* 0], m0
    mova         [r3+64* 1], m1
    mova         [r3+64* 2], m2
    mova         [r3+64* 3], m3
    mova         [r3+64* 4], m4
    mova         [r3+64* 5], m5
    mova         [r3+64* 6], m6
    mova         [r3+64* 7], m7
    mova         [r3+64* 8], m14
    mova         [r3+64* 9], m15
    mova         [r3+64*10], m16
    mova         [r3+64*11], m17
    mova         [r3+64*12], m18
    mova         [r3+64*13], m19
    mova         [r3+64*14], m20
    mova         [r3+64*15], m21
    call m(inv_txfm_add_dct_dct_32x32_8bpc).main_oddhalf_fast2
    mova         [r3+64*16], m29
    mova         [r3+64*17], m28
    mova         [r3+64*18], m27
    mova         [r3+64*19], m26
    mova         [r3+64*20], m25
    mova         [r3+64*21], m24
    mova         [r3+64*22], m23
    mova         [r3+64*23], m22
    mova         [r3+64*24], m21
    mova         [r3+64*25], m20
    mova         [r3+64*26], m19
    mova         [r3+64*27], m18
    mova         [r3+64*28], m17
    mova         [r3+64*29], m16
    mova         [r3+64*30], m15
    mova         [r3+64*31], m14
    ret

%endif ; ARCH_X86_64
