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

SECTION_RODATA 64

; dav1d_obmc_masks[] * -512
const obmc_masks_avx2
            dw      0,      0,  -9728,      0, -12800,  -7168,  -2560,      0
            dw -14336, -11264,  -8192,  -5632,  -3584,  -1536,      0,      0
            dw -15360, -13824, -12288, -10752,  -9216,  -7680,  -6144,  -5120
            dw  -4096,  -3072,  -2048,  -1536,      0,      0,      0,      0
            dw -15872, -14848, -14336, -13312, -12288, -11776, -10752, -10240
            dw  -9728,  -8704,  -8192,  -7168,  -6656,  -6144,  -5632,  -4608
            dw  -4096,  -3584,  -3072,  -2560,  -2048,  -2048,  -1536,  -1024
            dw      0,      0,      0,      0,      0,      0,      0,      0

deint_shuf:     dd 0,  4,  1,  5,  2,  6,  3,  7
subpel_h_shufA: db 0,  1,  2,  3,  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9
subpel_h_shufB: db 4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13
subpel_h_shuf2: db 0,  1,  2,  3,  4,  5,  6,  7,  2,  3,  4,  5,  6,  7,  8,  9
subpel_s_shuf2: db 0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7
subpel_s_shuf8: db 0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
rescale_mul:    dd 0,  1,  2,  3,  4,  5,  6,  7
rescale_mul2:   dd 0,  1,  4,  5,  2,  3,  6,  7
resize_shuf:    db 0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7
                db 8,  9, 10, 11, 12, 13, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15
blend_shuf:     db 0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  2,  3,  2,  3,  2,  3
wswap:          db 2,  3,  0,  1,  6,  7,  4,  5, 10, 11,  8,  9, 14, 15, 12, 13
bdct_lb_q: times 8 db 0
           times 8 db 4
           times 8 db 8
           times 8 db 12

prep_mul:         dw 16, 16, 4, 4
put_bilin_h_rnd:  dw 8, 8, 10, 10
put_8tap_h_rnd:   dd 34, 40
s_8tap_h_rnd:     dd 2, 8
s_8tap_h_sh:      dd 2, 4
put_s_8tap_v_rnd: dd 512, 128
put_s_8tap_v_sh:  dd 10, 8
prep_8tap_1d_rnd: dd     8 - (8192 <<  4)
prep_8tap_2d_rnd: dd    32 - (8192 <<  5)
warp8x8t_rnd:     dd 16384 - (8192 << 15)
warp8x8_shift:    dd  5,  3
warp8x8_rnd:      dw   4096,   4096,  16384,  16384
bidir_rnd:        dw -16400, -16400, -16388, -16388
bidir_mul:        dw   2048,   2048,   8192,   8192

%define pw_16 prep_mul
%define pd_512 put_s_8tap_v_rnd

pw_2:          times 2 dw 2
pw_64:         times 2 dw 64
pw_2048:       times 2 dw 2048
pw_8192:       times 2 dw 8192
pw_27615:      times 2 dw 27615
pw_32766:      times 2 dw 32766
pw_m512:       times 2 dw -512
pd_32:         dd 32
pd_63:         dd 63
pd_64:         dd 64
pd_32768:      dd 32768
pd_65538:      dd 65538
pd_m524256:    dd -524256 ; -8192 << 6 + 32
pd_0x3ff:      dd 0x3ff
pq_0x40000000: dq 0x40000000
               dd 0

%macro BIDIR_JMP_TABLE 2-*
    %xdefine %1_%2_table (%%table - 2*%3)
    %xdefine %%base %1_%2_table
    %xdefine %%prefix mangle(private_prefix %+ _%1_16bpc_%2)
    %%table:
    %rep %0 - 2
        dd %%prefix %+ .w%3 - %%base
        %rotate 1
    %endrep
%endmacro

BIDIR_JMP_TABLE avg,        avx2,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_avg,      avx2,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE mask,       avx2,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_420, avx2,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_422, avx2,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_444, avx2,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE blend,      avx2,    4, 8, 16, 32
BIDIR_JMP_TABLE blend_v,    avx2, 2, 4, 8, 16, 32
BIDIR_JMP_TABLE blend_h,    avx2, 2, 4, 8, 16, 32, 64, 128

%macro BASE_JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base %1_%2
    %%table:
    %rep %0 - 2
        dw %%base %+ _w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%xdefine put_avx2 mangle(private_prefix %+ _put_bilin_16bpc_avx2.put)
%xdefine prep_avx2 mangle(private_prefix %+ _prep_bilin_16bpc_avx2.prep)

BASE_JMP_TABLE put,  avx2, 2, 4, 8, 16, 32, 64, 128
BASE_JMP_TABLE prep, avx2,    4, 8, 16, 32, 64, 128

%macro HV_JMP_TABLE 5-*
    %xdefine %%prefix mangle(private_prefix %+ _%1_%2_16bpc_%3)
    %xdefine %%base %1_%3
    %assign %%types %4
    %if %%types & 1
        %xdefine %1_%2_h_%3_table  (%%h  - %5)
        %%h:
        %rep %0 - 4
            dw %%prefix %+ .h_w%5 - %%base
            %rotate 1
        %endrep
        %rotate 4
    %endif
    %if %%types & 2
        %xdefine %1_%2_v_%3_table  (%%v  - %5)
        %%v:
        %rep %0 - 4
            dw %%prefix %+ .v_w%5 - %%base
            %rotate 1
        %endrep
        %rotate 4
    %endif
    %if %%types & 4
        %xdefine %1_%2_hv_%3_table (%%hv - %5)
        %%hv:
        %rep %0 - 4
            dw %%prefix %+ .hv_w%5 - %%base
            %rotate 1
        %endrep
    %endif
%endmacro

HV_JMP_TABLE put,  bilin, avx2, 7, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE prep, bilin, avx2, 7,    4, 8, 16, 32, 64, 128

%macro SCALED_JMP_TABLE 2-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base mangle(private_prefix %+ _%1_16bpc_%2)
%%table:
    %rep %0 - 2
        dw %%base %+ .w%3 - %%base
        %rotate 1
    %endrep
    %rotate 2
 %%dy_1024:
    %xdefine %1_%2_dy1_table (%%dy_1024 - %3)
    %rep %0 - 2
        dw %%base %+ .dy1_w%3 - %%base
        %rotate 1
    %endrep
    %rotate 2
 %%dy_2048:
    %xdefine %1_%2_dy2_table (%%dy_2048 - %3)
    %rep %0 - 2
        dw %%base %+ .dy2_w%3 - %%base
        %rotate 1
    %endrep
%endmacro

SCALED_JMP_TABLE put_8tap_scaled, avx2, 2, 4, 8, 16, 32, 64, 128
SCALED_JMP_TABLE prep_8tap_scaled, avx2,   4, 8, 16, 32, 64, 128

%define table_offset(type, fn) type %+ fn %+ SUFFIX %+ _table - type %+ SUFFIX

cextern mc_subpel_filters
%define subpel_filters (mangle(private_prefix %+ _mc_subpel_filters)-8)

cextern mc_warp_filter
cextern resize_filter

SECTION .text

INIT_XMM avx2
cglobal put_bilin_16bpc, 4, 8, 0, dst, ds, src, ss, w, h, mxy
    mov                mxyd, r6m ; mx
    lea                  r7, [put_avx2]
%if UNIX64
    DECLARE_REG_TMP 8
    %define org_w r8d
    mov                 r8d, wd
%else
    DECLARE_REG_TMP 7
    %define org_w wm
%endif
    tzcnt                wd, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r7m ; my
    test               mxyd, mxyd
    jnz .v
.put:
    movzx                wd, word [r7+wq*2+table_offset(put,)]
    add                  wq, r7
    jmp                  wq
.put_w2:
    mov                 r6d, [srcq+ssq*0]
    mov                 r7d, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6d
    mov        [dstq+dsq*1], r7d
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w2
    RET
.put_w4:
    mov                  r6, [srcq+ssq*0]
    mov                  r7, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6
    mov        [dstq+dsq*1], r7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w4
    RET
.put_w8:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w8
    RET
INIT_YMM avx2
.put_w16:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w16
    RET
.put_w32:
    movu                 m0, [srcq+ssq*0+32*0]
    movu                 m1, [srcq+ssq*0+32*1]
    movu                 m2, [srcq+ssq*1+32*0]
    movu                 m3, [srcq+ssq*1+32*1]
    lea                srcq, [srcq+ssq*2]
    mova  [dstq+dsq*0+32*0], m0
    mova  [dstq+dsq*0+32*1], m1
    mova  [dstq+dsq*1+32*0], m2
    mova  [dstq+dsq*1+32*1], m3
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w32
    RET
.put_w64:
    movu                 m0, [srcq+32*0]
    movu                 m1, [srcq+32*1]
    movu                 m2, [srcq+32*2]
    movu                 m3, [srcq+32*3]
    add                srcq, ssq
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    mova        [dstq+32*2], m2
    mova        [dstq+32*3], m3
    add                dstq, dsq
    dec                  hd
    jg .put_w64
    RET
.put_w128:
    movu                 m0, [srcq+32*0]
    movu                 m1, [srcq+32*1]
    movu                 m2, [srcq+32*2]
    movu                 m3, [srcq+32*3]
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    mova        [dstq+32*2], m2
    mova        [dstq+32*3], m3
    movu                 m0, [srcq+32*4]
    movu                 m1, [srcq+32*5]
    movu                 m2, [srcq+32*6]
    movu                 m3, [srcq+32*7]
    add                srcq, ssq
    mova        [dstq+32*4], m0
    mova        [dstq+32*5], m1
    mova        [dstq+32*6], m2
    mova        [dstq+32*7], m3
    add                dstq, dsq
    dec                  hd
    jg .put_w128
    RET
.h:
    movd                xm5, mxyd
    mov                mxyd, r7m ; my
    vpbroadcastd         m4, [pw_16]
    vpbroadcastw         m5, xm5
    psubw                m4, m5
    test               mxyd, mxyd
    jnz .hv
    ; 12-bit is rounded twice so we can't use the same pmulhrsw approach as .v
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_h)]
    mov                 r6d, r8m ; bitdepth_max
    add                  wq, r7
    shr                 r6d, 11
    vpbroadcastd         m3, [r7-put_avx2+put_bilin_h_rnd+r6*4]
    jmp                  wq
.h_w2:
    movq                xm1, [srcq+ssq*0]
    movhps              xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmullw              xm0, xm4, xm1
    psrlq               xm1, 16
    pmullw              xm1, xm5
    paddw               xm0, xm3
    paddw               xm0, xm1
    psrlw               xm0, 4
    movd       [dstq+dsq*0], xm0
    pextrd     [dstq+dsq*1], xm0, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2
    RET
.h_w4:
    movq                xm0, [srcq+ssq*0]
    movhps              xm0, [srcq+ssq*1]
    movq                xm1, [srcq+ssq*0+2]
    movhps              xm1, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    pmullw              xm0, xm4
    pmullw              xm1, xm5
    paddw               xm0, xm3
    paddw               xm0, xm1
    psrlw               xm0, 4
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4
    RET
.h_w8:
    movu                xm0, [srcq+ssq*0]
    vinserti128          m0, [srcq+ssq*1], 1
    movu                xm1, [srcq+ssq*0+2]
    vinserti128          m1, [srcq+ssq*1+2], 1
    lea                srcq, [srcq+ssq*2]
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    psrlw                m0, 4
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    pmullw               m0, m4, [srcq+ssq*0]
    pmullw               m1, m5, [srcq+ssq*0+2]
    paddw                m0, m3
    paddw                m0, m1
    pmullw               m1, m4, [srcq+ssq*1]
    pmullw               m2, m5, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m0, 4
    psrlw                m1, 4
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w16
    RET
.h_w32:
    pmullw               m0, m4, [srcq+32*0]
    pmullw               m1, m5, [srcq+32*0+2]
    paddw                m0, m3
    paddw                m0, m1
    pmullw               m1, m4, [srcq+32*1]
    pmullw               m2, m5, [srcq+32*1+2]
    add                srcq, ssq
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m0, 4
    psrlw                m1, 4
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    add                dstq, dsq
    dec                  hd
    jg .h_w32
    RET
.h_w64:
.h_w128:
    movifnidn           t0d, org_w
.h_w64_loop0:
    mov                 r6d, t0d
.h_w64_loop:
    pmullw               m0, m4, [srcq+r6*2-32*1]
    pmullw               m1, m5, [srcq+r6*2-32*1+2]
    paddw                m0, m3
    paddw                m0, m1
    pmullw               m1, m4, [srcq+r6*2-32*2]
    pmullw               m2, m5, [srcq+r6*2-32*2+2]
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m0, 4
    psrlw                m1, 4
    mova   [dstq+r6*2-32*1], m0
    mova   [dstq+r6*2-32*2], m1
    sub                 r6d, 32
    jg .h_w64_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w64_loop0
    RET
.v:
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_v)]
    shl                mxyd, 11
    movd                xm5, mxyd
    add                  wq, r7
    vpbroadcastw         m5, xm5
    jmp                  wq
.v_w2:
    movd                xm0, [srcq+ssq*0]
.v_w2_loop:
    movd                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpckldq           xm2, xm0, xm1
    movd                xm0, [srcq+ssq*0]
    punpckldq           xm1, xm0
    psubw               xm1, xm2
    pmulhrsw            xm1, xm5
    paddw               xm1, xm2
    movd       [dstq+dsq*0], xm1
    pextrd     [dstq+dsq*1], xm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movq                xm0, [srcq+ssq*0]
.v_w4_loop:
    movq                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklqdq          xm2, xm0, xm1
    movq                xm0, [srcq+ssq*0]
    punpcklqdq          xm1, xm0
    psubw               xm1, xm2
    pmulhrsw            xm1, xm5
    paddw               xm1, xm2
    movq       [dstq+dsq*0], xm1
    movhps     [dstq+dsq*1], xm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movu                xm0, [srcq+ssq*0]
.v_w8_loop:
    vbroadcasti128       m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd             m2, m0, m1, 0xf0
    vbroadcasti128       m0, [srcq+ssq*0]
    vpblendd             m1, m0, 0xf0
    psubw                m1, m2
    pmulhrsw             m1, m5
    paddw                m1, m2
    mova         [dstq+dsq*0], xm1
    vextracti128 [dstq+dsq*1], m1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w32:
    movu                 m0, [srcq+ssq*0+32*0]
    movu                 m1, [srcq+ssq*0+32*1]
.v_w32_loop:
    movu                 m2, [srcq+ssq*1+32*0]
    movu                 m3, [srcq+ssq*1+32*1]
    lea                srcq, [srcq+ssq*2]
    psubw                m4, m2, m0
    pmulhrsw             m4, m5
    paddw                m4, m0
    movu                 m0, [srcq+ssq*0+32*0]
    mova  [dstq+dsq*0+32*0], m4
    psubw                m4, m3, m1
    pmulhrsw             m4, m5
    paddw                m4, m1
    movu                 m1, [srcq+ssq*0+32*1]
    mova  [dstq+dsq*0+32*1], m4
    psubw                m4, m0, m2
    pmulhrsw             m4, m5
    paddw                m4, m2
    mova  [dstq+dsq*1+32*0], m4
    psubw                m4, m1, m3
    pmulhrsw             m4, m5
    paddw                m4, m3
    mova  [dstq+dsq*1+32*1], m4
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w32_loop
    RET
.v_w16:
.v_w64:
.v_w128:
    movifnidn           t0d, org_w
    add                 t0d, t0d
    mov                  r4, srcq
    lea                 r6d, [hq+t0*8-256]
    mov                  r7, dstq
.v_w16_loop0:
    movu                 m0, [srcq+ssq*0]
.v_w16_loop:
    movu                 m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    psubw                m1, m3, m0
    pmulhrsw             m1, m5
    paddw                m1, m0
    movu                 m0, [srcq+ssq*0]
    psubw                m2, m0, m3
    pmulhrsw             m2, m5
    paddw                m2, m3
    mova       [dstq+dsq*0], m1
    mova       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    add                  r4, 32
    add                  r7, 32
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
    jg .v_w16_loop0
    RET
.hv:
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_hv)]
    WIN64_SPILL_XMM       8
    shl                mxyd, 11
    vpbroadcastd         m3, [pw_2]
    movd                xm6, mxyd
    vpbroadcastd         m7, [pw_8192]
    add                  wq, r7
    vpbroadcastw         m6, xm6
    test          dword r8m, 0x800
    jnz .hv_12bpc
    psllw                m4, 2
    psllw                m5, 2
    vpbroadcastd         m7, [pw_2048]
.hv_12bpc:
    jmp                  wq
.hv_w2:
    vpbroadcastq        xm1, [srcq+ssq*0]
    pmullw              xm0, xm4, xm1
    psrlq               xm1, 16
    pmullw              xm1, xm5
    paddw               xm0, xm3
    paddw               xm0, xm1
    psrlw               xm0, 2
.hv_w2_loop:
    movq                xm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps              xm2, [srcq+ssq*0]
    pmullw              xm1, xm4, xm2
    psrlq               xm2, 16
    pmullw              xm2, xm5
    paddw               xm1, xm3
    paddw               xm1, xm2
    psrlw               xm1, 2              ; 1 _ 2 _
    shufpd              xm2, xm0, xm1, 0x01 ; 0 _ 1 _
    mova                xm0, xm1
    psubw               xm1, xm2
    paddw               xm1, xm1
    pmulhw              xm1, xm6
    paddw               xm1, xm2
    pmulhrsw            xm1, xm7
    movd       [dstq+dsq*0], xm1
    pextrd     [dstq+dsq*1], xm1, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    pmullw              xm0, xm4, [srcq+ssq*0-8]
    pmullw              xm1, xm5, [srcq+ssq*0-6]
    paddw               xm0, xm3
    paddw               xm0, xm1
    psrlw               xm0, 2
.hv_w4_loop:
    movq                xm1, [srcq+ssq*1]
    movq                xm2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    movhps              xm1, [srcq+ssq*0]
    movhps              xm2, [srcq+ssq*0+2]
    pmullw              xm1, xm4
    pmullw              xm2, xm5
    paddw               xm1, xm3
    paddw               xm1, xm2
    psrlw               xm1, 2              ; 1 2
    shufpd              xm2, xm0, xm1, 0x01 ; 0 1
    mova                xm0, xm1
    psubw               xm1, xm2
    paddw               xm1, xm1
    pmulhw              xm1, xm6
    paddw               xm1, xm2
    pmulhrsw            xm1, xm7
    movq       [dstq+dsq*0], xm1
    movhps     [dstq+dsq*1], xm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    pmullw              xm0, xm4, [srcq+ssq*0]
    pmullw              xm1, xm5, [srcq+ssq*0+2]
    paddw               xm0, xm3
    paddw               xm0, xm1
    psrlw               xm0, 2
    vinserti128          m0, xm0, 1
.hv_w8_loop:
    movu                xm1, [srcq+ssq*1]
    movu                xm2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    vinserti128          m1, [srcq+ssq*0], 1
    vinserti128          m2, [srcq+ssq*0+2], 1
    pmullw               m1, m4
    pmullw               m2, m5
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m1, 2            ; 1 2
    vperm2i128           m2, m0, m1, 0x21 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    paddw                m1, m1
    pmulhw               m1, m6
    paddw                m1, m2
    pmulhrsw             m1, m7
    mova         [dstq+dsq*0], xm1
    vextracti128 [dstq+dsq*1], m1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
.hv_w32:
.hv_w64:
.hv_w128:
%if UNIX64
    lea                 r6d, [r8*2-32]
%else
    mov                 r6d, wm
    lea                 r6d, [r6*2-32]
%endif
    mov                  r4, srcq
    lea                 r6d, [hq+r6*8]
    mov                  r7, dstq
.hv_w16_loop0:
    pmullw               m0, m4, [srcq+ssq*0]
    pmullw               m1, m5, [srcq+ssq*0+2]
    paddw                m0, m3
    paddw                m0, m1
    psrlw                m0, 2
.hv_w16_loop:
    pmullw               m1, m4, [srcq+ssq*1]
    pmullw               m2, m5, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m1, 2
    psubw                m2, m1, m0
    paddw                m2, m2
    pmulhw               m2, m6
    paddw                m2, m0
    pmulhrsw             m2, m7
    mova       [dstq+dsq*0], m2
    pmullw               m0, m4, [srcq+ssq*0]
    pmullw               m2, m5, [srcq+ssq*0+2]
    paddw                m0, m3
    paddw                m0, m2
    psrlw                m0, 2
    psubw                m2, m0, m1
    paddw                m2, m2
    pmulhw               m2, m6
    paddw                m2, m1
    pmulhrsw             m2, m7
    mova       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    add                  r4, 32
    add                  r7, 32
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
    jg .hv_w16_loop0
    RET

cglobal prep_bilin_16bpc, 3, 7, 0, tmp, src, stride, w, h, mxy, stride3
    movifnidn          mxyd, r5m ; mx
    lea                  r6, [prep_avx2]
%if UNIX64
    DECLARE_REG_TMP 7
    %define org_w r7d
%else
    DECLARE_REG_TMP 6
    %define org_w r5m
%endif
    mov               org_w, wd
    tzcnt                wd, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r6m ; my
    test               mxyd, mxyd
    jnz .v
.prep:
    movzx                wd, word [r6+wq*2+table_offset(prep,)]
    mov                 r5d, r7m ; bitdepth_max
    vpbroadcastd         m5, [r6-prep_avx2+pw_8192]
    add                  wq, r6
    shr                 r5d, 11
    vpbroadcastd         m4, [r6-prep_avx2+prep_mul+r5*4]
    lea            stride3q, [strideq*3]
    jmp                  wq
.prep_w4:
    movq                xm0, [srcq+strideq*0]
    movhps              xm0, [srcq+strideq*1]
    vpbroadcastq         m1, [srcq+strideq*2]
    vpbroadcastq         m2, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd             m0, m1, 0x30
    vpblendd             m0, m2, 0xc0
    pmullw               m0, m4
    psubw                m0, m5
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 4
    jg .prep_w4
    RET
.prep_w8:
    movu                xm0, [srcq+strideq*0]
    vinserti128          m0, [srcq+strideq*1], 1
    movu                xm1, [srcq+strideq*2]
    vinserti128          m1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    pmullw               m0, m4
    pmullw               m1, m4
    psubw                m0, m5
    psubw                m1, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    add                tmpq, 32*2
    sub                  hd, 4
    jg .prep_w8
    RET
.prep_w16:
    pmullw               m0, m4, [srcq+strideq*0]
    pmullw               m1, m4, [srcq+strideq*1]
    pmullw               m2, m4, [srcq+strideq*2]
    pmullw               m3, m4, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    psubw                m0, m5
    psubw                m1, m5
    psubw                m2, m5
    psubw                m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    sub                  hd, 4
    jg .prep_w16
    RET
.prep_w32:
    pmullw               m0, m4, [srcq+strideq*0+32*0]
    pmullw               m1, m4, [srcq+strideq*0+32*1]
    pmullw               m2, m4, [srcq+strideq*1+32*0]
    pmullw               m3, m4, [srcq+strideq*1+32*1]
    lea                srcq, [srcq+strideq*2]
    psubw                m0, m5
    psubw                m1, m5
    psubw                m2, m5
    psubw                m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    sub                  hd, 2
    jg .prep_w32
    RET
.prep_w64:
    pmullw               m0, m4, [srcq+32*0]
    pmullw               m1, m4, [srcq+32*1]
    pmullw               m2, m4, [srcq+32*2]
    pmullw               m3, m4, [srcq+32*3]
    add                srcq, strideq
    psubw                m0, m5
    psubw                m1, m5
    psubw                m2, m5
    psubw                m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    dec                  hd
    jg .prep_w64
    RET
.prep_w128:
    pmullw               m0, m4, [srcq+32*0]
    pmullw               m1, m4, [srcq+32*1]
    pmullw               m2, m4, [srcq+32*2]
    pmullw               m3, m4, [srcq+32*3]
    psubw                m0, m5
    psubw                m1, m5
    psubw                m2, m5
    psubw                m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    pmullw               m0, m4, [srcq+32*4]
    pmullw               m1, m4, [srcq+32*5]
    pmullw               m2, m4, [srcq+32*6]
    pmullw               m3, m4, [srcq+32*7]
    add                tmpq, 32*8
    add                srcq, strideq
    psubw                m0, m5
    psubw                m1, m5
    psubw                m2, m5
    psubw                m3, m5
    mova        [tmpq-32*4], m0
    mova        [tmpq-32*3], m1
    mova        [tmpq-32*2], m2
    mova        [tmpq-32*1], m3
    dec                  hd
    jg .prep_w128
    RET
.h:
    movd                xm5, mxyd
    mov                mxyd, r6m ; my
    vpbroadcastd         m4, [pw_16]
    vpbroadcastw         m5, xm5
    vpbroadcastd         m3, [pw_32766]
    psubw                m4, m5
    test          dword r7m, 0x800
    jnz .h_12bpc
    psllw                m4, 2
    psllw                m5, 2
.h_12bpc:
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_h)]
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.h_w4:
    movu                xm1, [srcq+strideq*0]
    vinserti128          m1, [srcq+strideq*2], 1
    movu                xm2, [srcq+strideq*1]
    vinserti128          m2, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    punpcklqdq           m0, m1, m2
    psrldq               m1, 2
    pslldq               m2, 6
    pmullw               m0, m4
    vpblendd             m1, m2, 0xcc
    pmullw               m1, m5
    psubw                m0, m3
    paddw                m0, m1
    psraw                m0, 2
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4
    RET
.h_w8:
    movu                xm0, [srcq+strideq*0]
    vinserti128          m0, [srcq+strideq*1], 1
    movu                xm1, [srcq+strideq*0+2]
    vinserti128          m1, [srcq+strideq*1+2], 1
    lea                srcq, [srcq+strideq*2]
    pmullw               m0, m4
    pmullw               m1, m5
    psubw                m0, m3
    paddw                m0, m1
    psraw                m0, 2
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    pmullw               m0, m4, [srcq+strideq*0]
    pmullw               m1, m5, [srcq+strideq*0+2]
    psubw                m0, m3
    paddw                m0, m1
    pmullw               m1, m4, [srcq+strideq*1]
    pmullw               m2, m5, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    psubw                m1, m3
    paddw                m1, m2
    psraw                m0, 2
    psraw                m1, 2
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    add                tmpq, 32*2
    sub                  hd, 2
    jg .h_w16
    RET
.h_w32:
.h_w64:
.h_w128:
    movifnidn           t0d, org_w
.h_w32_loop0:
    mov                 r3d, t0d
.h_w32_loop:
    pmullw               m0, m4, [srcq+r3*2-32*1]
    pmullw               m1, m5, [srcq+r3*2-32*1+2]
    psubw                m0, m3
    paddw                m0, m1
    pmullw               m1, m4, [srcq+r3*2-32*2]
    pmullw               m2, m5, [srcq+r3*2-32*2+2]
    psubw                m1, m3
    paddw                m1, m2
    psraw                m0, 2
    psraw                m1, 2
    mova   [tmpq+r3*2-32*1], m0
    mova   [tmpq+r3*2-32*2], m1
    sub                 r3d, 32
    jg .h_w32_loop
    add                srcq, strideq
    lea                tmpq, [tmpq+t0*2]
    dec                  hd
    jg .h_w32_loop0
    RET
.v:
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_v)]
    movd                xm5, mxyd
    vpbroadcastd         m4, [pw_16]
    vpbroadcastw         m5, xm5
    vpbroadcastd         m3, [pw_32766]
    add                  wq, r6
    lea            stride3q, [strideq*3]
    psubw                m4, m5
    test          dword r7m, 0x800
    jnz .v_12bpc
    psllw                m4, 2
    psllw                m5, 2
.v_12bpc:
    jmp                  wq
.v_w4:
    movq                xm0, [srcq+strideq*0]
.v_w4_loop:
    vpbroadcastq         m2, [srcq+strideq*2]
    vpbroadcastq        xm1, [srcq+strideq*1]
    vpblendd             m2, m0, 0x03 ; 0 2 2 2
    vpbroadcastq         m0, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd             m1, m0, 0xf0 ; 1 1 3 3
    vpbroadcastq         m0, [srcq+strideq*0]
    vpblendd             m1, m2, 0x33 ; 0 1 2 3
    vpblendd             m0, m2, 0x0c ; 4 2 4 4
    punpckhqdq           m2, m1, m0   ; 1 2 3 4
    pmullw               m1, m4
    pmullw               m2, m5
    psubw                m1, m3
    paddw                m1, m2
    psraw                m1, 2
    mova             [tmpq], m1
    add                tmpq, 32
    sub                  hd, 4
    jg .v_w4_loop
    RET
.v_w8:
    movu                xm0, [srcq+strideq*0]
.v_w8_loop:
    vbroadcasti128       m2, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vpblendd             m1, m0, m2, 0xf0 ; 0 1
    vbroadcasti128       m0, [srcq+strideq*0]
    vpblendd             m2, m0, 0xf0     ; 1 2
    pmullw               m1, m4
    pmullw               m2, m5
    psubw                m1, m3
    paddw                m1, m2
    psraw                m1, 2
    mova             [tmpq], m1
    add                tmpq, 32
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w16:
    movu                 m0, [srcq+strideq*0]
.v_w16_loop:
    movu                 m2, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    pmullw               m0, m4
    pmullw               m1, m5, m2
    psubw                m0, m3
    paddw                m1, m0
    movu                 m0, [srcq+strideq*0]
    psraw                m1, 2
    pmullw               m2, m4
    mova        [tmpq+32*0], m1
    pmullw               m1, m5, m0
    psubw                m2, m3
    paddw                m1, m2
    psraw                m1, 2
    mova        [tmpq+32*1], m1
    add                tmpq, 32*2
    sub                  hd, 2
    jg .v_w16_loop
    RET
.v_w32:
.v_w64:
.v_w128:
%if WIN64
    PUSH                 r7
%endif
    movifnidn           r7d, org_w
    add                 r7d, r7d
    mov                  r3, srcq
    lea                 r6d, [hq+r7*8-256]
    mov                  r5, tmpq
.v_w32_loop0:
    movu                 m0, [srcq+strideq*0]
.v_w32_loop:
    movu                 m2, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    pmullw               m0, m4
    pmullw               m1, m5, m2
    psubw                m0, m3
    paddw                m1, m0
    movu                 m0, [srcq+strideq*0]
    psraw                m1, 2
    pmullw               m2, m4
    mova        [tmpq+r7*0], m1
    pmullw               m1, m5, m0
    psubw                m2, m3
    paddw                m1, m2
    psraw                m1, 2
    mova        [tmpq+r7*1], m1
    lea                tmpq, [tmpq+r7*2]
    sub                  hd, 2
    jg .v_w32_loop
    add                  r3, 32
    add                  r5, 32
    movzx                hd, r6b
    mov                srcq, r3
    mov                tmpq, r5
    sub                 r6d, 1<<8
    jg .v_w32_loop0
%if WIN64
    POP                  r7
%endif
    RET
.hv:
    WIN64_SPILL_XMM       7
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_hv)]
    shl                mxyd, 11
    movd                xm6, mxyd
    add                  wq, r6
    lea            stride3q, [strideq*3]
    vpbroadcastw         m6, xm6
    jmp                  wq
.hv_w4:
    movu                xm1, [srcq+strideq*0]
%if WIN64
    movaps         [rsp+24], xmm7
%endif
    pmullw              xm0, xm4, xm1
    psrldq              xm1, 2
    pmullw              xm1, xm5
    psubw               xm0, xm3
    paddw               xm0, xm1
    psraw               xm0, 2
    vpbroadcastq         m0, xm0
.hv_w4_loop:
    movu                xm1, [srcq+strideq*1]
    vinserti128          m1, [srcq+stride3q ], 1
    movu                xm2, [srcq+strideq*2]
    lea                srcq, [srcq+strideq*4]
    vinserti128          m2, [srcq+strideq*0], 1
    punpcklqdq           m7, m1, m2
    psrldq               m1, 2
    pslldq               m2, 6
    pmullw               m7, m4
    vpblendd             m1, m2, 0xcc
    pmullw               m1, m5
    psubw                m7, m3
    paddw                m1, m7
    psraw                m1, 2         ; 1 2 3 4
    vpblendd             m0, m1, 0x3f
    vpermq               m2, m0, q2103 ; 0 1 2 3
    mova                 m0, m1
    psubw                m1, m2
    pmulhrsw             m1, m6
    paddw                m1, m2
    mova             [tmpq], m1
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
%if WIN64
    movaps             xmm7, [rsp+24]
%endif
    RET
.hv_w8:
    pmullw              xm0, xm4, [srcq+strideq*0]
    pmullw              xm1, xm5, [srcq+strideq*0+2]
    psubw               xm0, xm3
    paddw               xm0, xm1
    psraw               xm0, 2
    vinserti128          m0, xm0, 1
.hv_w8_loop:
    movu                xm1, [srcq+strideq*1]
    movu                xm2, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    vinserti128          m1, [srcq+strideq*0], 1
    vinserti128          m2, [srcq+strideq*0+2], 1
    pmullw               m1, m4
    pmullw               m2, m5
    psubw                m1, m3
    paddw                m1, m2
    psraw                m1, 2            ; 1 2
    vperm2i128           m2, m0, m1, 0x21 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    pmulhrsw             m1, m6
    paddw                m1, m2
    mova             [tmpq], m1
    add                tmpq, 32
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
.hv_w32:
.hv_w64:
.hv_w128:
%if WIN64
    PUSH                 r7
%endif
    movifnidn           r7d, org_w
    add                 r7d, r7d
    mov                  r3, srcq
    lea                 r6d, [hq+r7*8-256]
    mov                  r5, tmpq
.hv_w16_loop0:
    pmullw               m0, m4, [srcq]
    pmullw               m1, m5, [srcq+2]
    psubw                m0, m3
    paddw                m0, m1
    psraw                m0, 2
.hv_w16_loop:
    pmullw               m1, m4, [srcq+strideq*1]
    pmullw               m2, m5, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    psubw                m1, m3
    paddw                m1, m2
    psraw                m1, 2
    psubw                m2, m1, m0
    pmulhrsw             m2, m6
    paddw                m2, m0
    mova        [tmpq+r7*0], m2
    pmullw               m0, m4, [srcq+strideq*0]
    pmullw               m2, m5, [srcq+strideq*0+2]
    psubw                m0, m3
    paddw                m0, m2
    psraw                m0, 2
    psubw                m2, m0, m1
    pmulhrsw             m2, m6
    paddw                m2, m1
    mova        [tmpq+r7*1], m2
    lea                tmpq, [tmpq+r7*2]
    sub                  hd, 2
    jg .hv_w16_loop
    add                  r3, 32
    add                  r5, 32
    movzx                hd, r6b
    mov                srcq, r3
    mov                tmpq, r5
    sub                 r6d, 1<<8
    jg .hv_w16_loop0
%if WIN64
    POP                  r7
%endif
    RET

; int8_t subpel_filters[5][15][8]
%assign FILTER_REGULAR (0*15 << 16) | 3*15
%assign FILTER_SMOOTH  (1*15 << 16) | 4*15
%assign FILTER_SHARP   (2*15 << 16) | 3*15

%macro FN 4 ; prefix, type, type_h, type_v
cglobal %1_%2_16bpc
    mov                 t0d, FILTER_%3
%ifidn %3, %4
    mov                 t1d, t0d
%else
    mov                 t1d, FILTER_%4
%endif
%ifnidn %2, regular ; skip the jump in the last filter
    jmp mangle(private_prefix %+ _%1_16bpc %+ SUFFIX)
%endif
%endmacro

%if WIN64
DECLARE_REG_TMP 4, 5
%else
DECLARE_REG_TMP 7, 8
%endif

%define PUT_8TAP_FN FN put_8tap,
PUT_8TAP_FN sharp,          SHARP,   SHARP
PUT_8TAP_FN sharp_smooth,   SHARP,   SMOOTH
PUT_8TAP_FN smooth_sharp,   SMOOTH,  SHARP
PUT_8TAP_FN smooth,         SMOOTH,  SMOOTH
PUT_8TAP_FN sharp_regular,  SHARP,   REGULAR
PUT_8TAP_FN regular_sharp,  REGULAR, SHARP
PUT_8TAP_FN smooth_regular, SMOOTH,  REGULAR
PUT_8TAP_FN regular_smooth, REGULAR, SMOOTH
PUT_8TAP_FN regular,        REGULAR, REGULAR

cglobal put_8tap_16bpc, 4, 9, 0, dst, ds, src, ss, w, h, mx, my
%define base r8-put_avx2
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r8, [put_avx2]
    movifnidn            wd, wm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    movzx                wd, word [r8+wq*2+table_offset(put,)]
    add                  wq, r8
%if WIN64
    pop                  r8
%endif
    jmp                  wq
.h_w2:
    movzx               mxd, mxb
    sub                srcq, 2
    mova                xm2, [subpel_h_shuf2]
    vpbroadcastd        xm3, [base+subpel_filters+mxq*8+2]
    pmovsxbw            xm3, xm3
.h_w2_loop:
    movu                xm0, [srcq+ssq*0]
    movu                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              xm0, xm2
    pshufb              xm1, xm2
    pmaddwd             xm0, xm3
    pmaddwd             xm1, xm3
    phaddd              xm0, xm1
    paddd               xm0, xm4
    psrad               xm0, 6
    packusdw            xm0, xm0
    pminsw              xm0, xm5
    movd       [dstq+dsq*0], xm0
    pextrd     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2_loop
    RET
.h_w4:
    movzx               mxd, mxb
    sub                srcq, 2
    pmovsxbw            xm3, [base+subpel_filters+mxq*8]
    WIN64_SPILL_XMM       8
    vbroadcasti128       m6, [subpel_h_shufA]
    vbroadcasti128       m7, [subpel_h_shufB]
    pshufd              xm3, xm3, q2211
    vpbroadcastq         m2, xm3
    vpermq               m3, m3, q1111
.h_w4_loop:
    movu                xm1, [srcq+ssq*0]
    vinserti128          m1, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pshufb               m0, m1, m6 ; 0 1 1 2 2 3 3 4
    pshufb               m1, m7     ; 2 3 3 4 4 5 5 6
    pmaddwd              m0, m2
    pmaddwd              m1, m3
    paddd                m0, m4
    paddd                m0, m1
    psrad                m0, 6
    vextracti128        xm1, m0, 1
    packusdw            xm0, xm1
    pminsw              xm0, xm5
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4_loop
    RET
.h:
    test                myd, 0xf00
    jnz .hv
    mov                 r7d, r8m
    vpbroadcastw         m5, r8m
    shr                 r7d, 11
    vpbroadcastd         m4, [base+put_8tap_h_rnd+r7*4]
    cmp                  wd, 4
    je .h_w4
    jl .h_w2
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      13
    shr                 mxd, 16
    sub                srcq, 6
    vpbroadcastq         m0, [base+subpel_filters+mxq*8]
    vbroadcasti128       m6, [subpel_h_shufA]
    vbroadcasti128       m7, [subpel_h_shufB]
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    pshufd               m8, m0, q0000
    pshufd               m9, m0, q1111
    pshufd              m10, m0, q2222
    pshufd              m11, m0, q3333
    cmp                  wd, 8
    jg .h_w16
.h_w8:
%macro PUT_8TAP_H 5 ; dst/src+0, src+8, src+16, tmp[1-2]
    pshufb              m%4, m%1, m7   ; 2 3 3 4 4 5 5 6
    pshufb              m%1, m6        ; 0 1 1 2 2 3 3 4
    pmaddwd             m%5, m9, m%4   ; abcd1
    pmaddwd             m%1, m8        ; abcd0
    pshufb              m%2, m7        ; 6 7 7 8 8 9 9 a
    shufpd              m%4, m%2, 0x05 ; 4 5 5 6 6 7 7 8
    paddd               m%5, m4
    paddd               m%1, m%5
    pmaddwd             m%5, m11, m%2  ; abcd3
    paddd               m%1, m%5
    pmaddwd             m%5, m10, m%4  ; abcd2
    pshufb              m%3, m7        ; a b b c c d d e
    pmaddwd             m%4, m8        ; efgh0
    paddd               m%1, m%5
    pmaddwd             m%5, m9, m%2   ; efgh1
    shufpd              m%2, m%3, 0x05 ; 8 9 9 a a b b c
    pmaddwd             m%3, m11       ; efgh3
    pmaddwd             m%2, m10       ; efgh2
    paddd               m%4, m4
    paddd               m%4, m%5
    paddd               m%3, m%4
    paddd               m%2, m%3
    psrad               m%1, 6
    psrad               m%2, 6
    packusdw            m%1, m%2
    pminsw              m%1, m5
%endmacro
    movu                xm0, [srcq+ssq*0+ 0]
    vinserti128          m0, [srcq+ssq*1+ 0], 1
    movu                xm2, [srcq+ssq*0+16]
    vinserti128          m2, [srcq+ssq*1+16], 1
    lea                srcq, [srcq+ssq*2]
    shufpd               m1, m0, m2, 0x05
    PUT_8TAP_H            0, 1, 2, 3, 12
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    mov                 r6d, wd
.h_w16_loop:
    movu                 m0, [srcq+r6*2-32]
    movu                 m1, [srcq+r6*2-24]
    movu                 m2, [srcq+r6*2-16]
    PUT_8TAP_H            0, 1, 2, 3, 12
    mova     [dstq+r6*2-32], m0
    sub                 r6d, 16
    jg .h_w16_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w16
    RET
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmovle              myd, mxd
    vpbroadcastq         m0, [base+subpel_filters+myq*8]
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      15
    vpbroadcastd         m6, [pd_32]
    vpbroadcastw         m7, r8m
    lea                  r6, [ssq*3]
    sub                srcq, r6
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    pshufd               m8, m0, q0000
    pshufd               m9, m0, q1111
    pshufd              m10, m0, q2222
    pshufd              m11, m0, q3333
    cmp                  wd, 4
    jg .v_w8
    je .v_w4
.v_w2:
    movd                xm2, [srcq+ssq*0]
    pinsrd              xm2, [srcq+ssq*1], 1
    pinsrd              xm2, [srcq+ssq*2], 2
    pinsrd              xm2, [srcq+r6   ], 3 ; 0 1 2 3
    lea                srcq, [srcq+ssq*4]
    movd                xm3, [srcq+ssq*0]
    vpbroadcastd        xm1, [srcq+ssq*1]
    vpbroadcastd        xm0, [srcq+ssq*2]
    add                srcq, r6
    vpblendd            xm3, xm1, 0x02       ; 4 5
    vpblendd            xm1, xm0, 0x02       ; 5 6
    palignr             xm4, xm3, xm2, 4     ; 1 2 3 4
    punpcklwd           xm3, xm1             ; 45 56
    punpcklwd           xm1, xm2, xm4        ; 01 12
    punpckhwd           xm2, xm4             ; 23 34
.v_w2_loop:
    vpbroadcastd        xm4, [srcq+ssq*0]
    pmaddwd             xm5, xm8, xm1        ; a0 b0
    mova                xm1, xm2
    pmaddwd             xm2, xm9             ; a1 b1
    paddd               xm5, xm6
    paddd               xm5, xm2
    mova                xm2, xm3
    pmaddwd             xm3, xm10            ; a2 b2
    paddd               xm5, xm3
    vpblendd            xm3, xm0, xm4, 0x02  ; 6 7
    vpbroadcastd        xm0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd            xm4, xm0, 0x02       ; 7 8
    punpcklwd           xm3, xm4             ; 67 78
    pmaddwd             xm4, xm11, xm3       ; a3 b3
    paddd               xm5, xm4
    psrad               xm5, 6
    packusdw            xm5, xm5
    pminsw              xm5, xm7
    movd       [dstq+dsq*0], xm5
    pextrd     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movq                xm1, [srcq+ssq*0]
    vpbroadcastq         m0, [srcq+ssq*1]
    vpbroadcastq         m2, [srcq+ssq*2]
    vpbroadcastq         m4, [srcq+r6   ]
    lea                srcq, [srcq+ssq*4]
    vpbroadcastq         m3, [srcq+ssq*0]
    vpbroadcastq         m5, [srcq+ssq*1]
    vpblendd             m1, m0, 0x30
    vpblendd             m0, m2, 0x30
    punpcklwd            m1, m0      ; 01 12
    vpbroadcastq         m0, [srcq+ssq*2]
    add                srcq, r6
    vpblendd             m2, m4, 0x30
    vpblendd             m4, m3, 0x30
    punpcklwd            m2, m4      ; 23 34
    vpblendd             m3, m5, 0x30
    vpblendd             m5, m0, 0x30
    punpcklwd            m3, m5      ; 45 56
.v_w4_loop:
    vpbroadcastq         m4, [srcq+ssq*0]
    pmaddwd              m5, m8, m1  ; a0 b0
    mova                 m1, m2
    pmaddwd              m2, m9      ; a1 b1
    paddd                m5, m6
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, m10     ; a2 b2
    paddd                m5, m3
    vpblendd             m3, m0, m4, 0x30
    vpbroadcastq         m0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd             m4, m0, 0x30
    punpcklwd            m3, m4      ; 67 78
    pmaddwd              m4, m11, m3 ; a3 b3
    paddd                m5, m4
    psrad                m5, 6
    vextracti128        xm4, m5, 1
    packusdw            xm5, xm4
    pminsw              xm5, xm7
    movq       [dstq+dsq*0], xm5
    movhps     [dstq+dsq*1], xm5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    shl                  wd, 5
    mov                  r7, srcq
    mov                  r8, dstq
    lea                  wd, [hq+wq-256]
.v_w8_loop0:
    vbroadcasti128       m4, [srcq+ssq*0]
    vbroadcasti128       m5, [srcq+ssq*1]
    vbroadcasti128       m0, [srcq+r6   ]
    vbroadcasti128       m6, [srcq+ssq*2]
    lea                srcq, [srcq+ssq*4]
    vbroadcasti128       m1, [srcq+ssq*0]
    vbroadcasti128       m2, [srcq+ssq*1]
    vbroadcasti128       m3, [srcq+ssq*2]
    add                srcq, r6
    shufpd               m4, m0, 0x0c
    shufpd               m5, m1, 0x0c
    punpcklwd            m1, m4, m5 ; 01
    punpckhwd            m4, m5     ; 34
    shufpd               m6, m2, 0x0c
    punpcklwd            m2, m5, m6 ; 12
    punpckhwd            m5, m6     ; 45
    shufpd               m0, m3, 0x0c
    punpcklwd            m3, m6, m0 ; 23
    punpckhwd            m6, m0     ; 56
.v_w8_loop:
    vbroadcasti128      m14, [srcq+ssq*0]
    pmaddwd             m12, m8, m1  ; a0
    pmaddwd             m13, m8, m2  ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddwd              m3, m9      ; a1
    pmaddwd              m4, m9      ; b1
    paddd               m12, m3
    paddd               m13, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddwd              m5, m10     ; a2
    pmaddwd              m6, m10     ; b2
    paddd               m12, m5
    vbroadcasti128       m5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    paddd               m13, m6
    shufpd               m6, m0, m14, 0x0d
    shufpd               m0, m14, m5, 0x0c
    punpcklwd            m5, m6, m0  ; 67
    punpckhwd            m6, m0      ; 78
    pmaddwd             m14, m11, m5 ; a3
    paddd               m12, m14
    pmaddwd             m14, m11, m6 ; b3
    paddd               m13, m14
    psrad               m12, 5
    psrad               m13, 5
    packusdw            m12, m13
    pxor                m13, m13
    pavgw               m12, m13
    pminsw              m12, m7
    vpermq              m12, m12, q3120
    mova         [dstq+dsq*0], xm12
    vextracti128 [dstq+dsq*1], m12, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    add                  r7, 16
    add                  r8, 16
    movzx                hd, wb
    mov                srcq, r7
    mov                dstq, r8
    sub                  wd, 1<<8
    jg .v_w8_loop0
    RET
.hv:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      16
    vpbroadcastw        m15, r8m
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    vpbroadcastd         m0, [base+subpel_filters+mxq*8+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmovle              myd, mxd
    vpbroadcastq         m1, [base+subpel_filters+myq*8]
    vpbroadcastd         m6, [pd_512]
    lea                  r6, [ssq*3]
    sub                srcq, 2
    sub                srcq, r6
    pxor                 m7, m7
    punpcklbw            m7, m0
    punpcklbw            m1, m1
    psraw                m1, 8 ; sign-extend
    test          dword r8m, 0x800
    jz .hv_10bit
    psraw                m7, 2
    psllw                m1, 2
.hv_10bit:
    pshufd              m11, m1, q0000
    pshufd              m12, m1, q1111
    pshufd              m13, m1, q2222
    pshufd              m14, m1, q3333
    cmp                  wd, 4
    je .hv_w4
    vbroadcasti128       m9, [subpel_h_shuf2]
    vbroadcasti128       m1, [srcq+r6   ]    ; 3 3
    movu                xm3, [srcq+ssq*2]
    movu                xm0, [srcq+ssq*0]
    movu                xm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*4]
    vinserti128          m3, [srcq+ssq*0], 1 ; 2 4
    vinserti128          m0, [srcq+ssq*1], 1 ; 0 5
    vinserti128          m2, [srcq+ssq*2], 1 ; 1 6
    add                srcq, r6
    pshufb               m1, m9
    pshufb               m3, m9
    pshufb               m0, m9
    pshufb               m2, m9
    pmaddwd              m1, m7
    pmaddwd              m3, m7
    pmaddwd              m0, m7
    pmaddwd              m2, m7
    phaddd               m1, m3
    phaddd               m0, m2
    paddd                m1, m6
    paddd                m0, m6
    psrad                m1, 10
    psrad                m0, 10
    packssdw             m1, m0         ; 3 2 0 1
    vextracti128        xm0, m1, 1      ; 3 4 5 6
    pshufd              xm2, xm1, q1301 ; 2 3 1 2
    pshufd              xm3, xm0, q2121 ; 4 5 4 5
    punpckhwd           xm1, xm2        ; 01 12
    punpcklwd           xm2, xm0        ; 23 34
    punpckhwd           xm3, xm0        ; 45 56
.hv_w2_loop:
    movu                xm4, [srcq+ssq*0]
    movu                xm5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              xm4, xm9
    pshufb              xm5, xm9
    pmaddwd             xm4, xm7
    pmaddwd             xm5, xm7
    phaddd              xm4, xm5
    pmaddwd             xm5, xm11, xm1 ; a0 b0
    mova                xm1, xm2
    pmaddwd             xm2, xm12      ; a1 b1
    paddd               xm5, xm2
    mova                xm2, xm3
    pmaddwd             xm3, xm13      ; a2 b2
    paddd               xm5, xm3
    paddd               xm4, xm6
    psrad               xm4, 10
    packssdw            xm4, xm4
    palignr             xm3, xm4, xm0, 12
    mova                xm0, xm4
    punpcklwd           xm3, xm0       ; 67 78
    pmaddwd             xm4, xm14, xm3 ; a3 b3
    paddd               xm5, xm6
    paddd               xm5, xm4
    psrad               xm5, 10
    packusdw            xm5, xm5
    pminsw              xm5, xm15
    movd       [dstq+dsq*0], xm5
    pextrd     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    vbroadcasti128       m9, [subpel_h_shufA]
    vbroadcasti128      m10, [subpel_h_shufB]
    pshufd               m8, m7, q1111
    pshufd               m7, m7, q0000
    movu                xm1, [srcq+ssq*0]
    vinserti128          m1, [srcq+ssq*1], 1     ; 0 1
    vbroadcasti128       m0, [srcq+r6   ]
    vinserti128          m2, m0, [srcq+ssq*2], 0 ; 2 3
    lea                srcq, [srcq+ssq*4]
    vinserti128          m0, [srcq+ssq*0], 1     ; 3 4
    movu                xm3, [srcq+ssq*1]
    vinserti128          m3, [srcq+ssq*2], 1     ; 5 6
    add                srcq, r6
    pshufb               m4, m1, m9
    pshufb               m1, m10
    pmaddwd              m4, m7
    pmaddwd              m1, m8
    pshufb               m5, m2, m9
    pshufb               m2, m10
    pmaddwd              m5, m7
    pmaddwd              m2, m8
    paddd                m4, m6
    paddd                m1, m4
    pshufb               m4, m0, m9
    pshufb               m0, m10
    pmaddwd              m4, m7
    pmaddwd              m0, m8
    paddd                m5, m6
    paddd                m2, m5
    pshufb               m5, m3, m9
    pshufb               m3, m10
    pmaddwd              m5, m7
    pmaddwd              m3, m8
    paddd                m4, m6
    paddd                m4, m0
    paddd                m5, m6
    paddd                m5, m3
    vperm2i128           m0, m1, m2, 0x21
    psrld                m1, 10
    psrld                m2, 10
    vperm2i128           m3, m4, m5, 0x21
    pslld                m4, 6
    pslld                m5, 6
    pblendw              m2, m4, 0xaa ; 23 34
    pslld                m0, 6
    pblendw              m1, m0, 0xaa ; 01 12
    psrld                m3, 10
    pblendw              m3, m5, 0xaa ; 45 56
    psrad                m0, m5, 16
.hv_w4_loop:
    movu                xm4, [srcq+ssq*0]
    vinserti128          m4, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pmaddwd              m5, m11, m1   ; a0 b0
    mova                 m1, m2
    pmaddwd              m2, m12       ; a1 b1
    paddd                m5, m6
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, m13       ; a2 b2
    paddd                m5, m3
    pshufb               m3, m4, m9
    pshufb               m4, m10
    pmaddwd              m3, m7
    pmaddwd              m4, m8
    paddd                m3, m6
    paddd                m4, m3
    psrad                m4, 10
    packssdw             m0, m4        ; _ 7 6 8
    vpermq               m3, m0, q1122 ; _ 6 _ 7
    punpckhwd            m3, m0        ; 67 78
    mova                 m0, m4
    pmaddwd              m4, m14, m3   ; a3 b3
    paddd                m4, m5
    psrad                m4, 10
    vextracti128        xm5, m4, 1
    packusdw            xm4, xm5
    pminsw              xm4, xm15
    movq       [dstq+dsq*0], xm4
    movhps     [dstq+dsq*1], xm4
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    shr                 mxd, 16
    vpbroadcastq         m2, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmovle              myd, mxd
    pmovsxbw            xm1, [base+subpel_filters+myq*8]
    shl                  wd, 5
    lea                  r6, [ssq*3]
    sub                srcq, 6
    sub                srcq, r6
    pxor                 m0, m0
    punpcklbw            m0, m2
    mov                  r7, srcq
    mov                  r8, dstq
    lea                  wd, [hq+wq-256]
    test          dword r8m, 0x800
    jz .hv_w8_10bit
    psraw                m0, 2
    psllw               xm1, 2
.hv_w8_10bit:
    pshufd              m11, m0, q0000
    pshufd              m12, m0, q1111
    pshufd              m13, m0, q2222
    pshufd              m14, m0, q3333
%if WIN64
    %define v_mul (rsp+stack_offset+40) ; r4m
%else
    %define v_mul (rsp-24) ; red zone
%endif
    mova            [v_mul], xm1
.hv_w8_loop0:
%macro PUT_8TAP_HV_H 3 ; dst/src+0, src+8, src+16
    pshufb               m2, m%1, m9   ; 2 3 3 4 4 5 5 6
    pshufb              m%1, m8        ; 0 1 1 2 2 3 3 4
    pmaddwd              m3, m12, m2
    pmaddwd             m%1, m11
    pshufb              m%2, m9        ; 6 7 7 8 8 9 9 a
    shufpd               m2, m%2, 0x05 ; 4 5 5 6 6 7 7 8
    paddd                m3, m10
    paddd               m%1, m3
    pmaddwd              m3, m14, m%2
    paddd               m%1, m3
    pmaddwd              m3, m13, m2
    pshufb              m%3, m9        ; a b b c c d d e
    pmaddwd              m2, m11
    paddd               m%1, m3
    pmaddwd              m3, m12, m%2
    shufpd              m%2, m%3, 0x05 ; 8 9 9 a a b b c
    pmaddwd             m%3, m14
    pmaddwd             m%2, m13
    paddd                m2, m10
    paddd                m2, m3
    paddd               m%3, m2
    paddd               m%2, m%3
    psrad               m%1, 10
    psrad               m%2, 10
    packssdw            m%1, m%2
%endmacro
    movu                xm4, [srcq+r6 *1+ 0]
    vbroadcasti128       m8, [subpel_h_shufA]
    movu                xm6, [srcq+r6 *1+ 8]
    vbroadcasti128       m9, [subpel_h_shufB]
    movu                xm0, [srcq+r6 *1+16]
    vpbroadcastd        m10, [pd_512]
    movu                xm5, [srcq+ssq*0+ 0]
    vinserti128          m5, [srcq+ssq*4+ 0], 1
    movu                xm1, [srcq+ssq*0+16]
    vinserti128          m1, [srcq+ssq*4+16], 1
    shufpd               m7, m5, m1, 0x05
    INIT_XMM avx2
    PUT_8TAP_HV_H         4, 6, 0    ; 3
    INIT_YMM avx2
    PUT_8TAP_HV_H         5, 7, 1    ; 0 4
    movu                xm0, [srcq+ssq*2+ 0]
    vinserti128          m0, [srcq+r6 *2+ 0], 1
    movu                xm1, [srcq+ssq*2+16]
    vinserti128          m1, [srcq+r6 *2+16], 1
    shufpd               m7, m0, m1, 0x05
    PUT_8TAP_HV_H         0, 7, 1    ; 2 6
    movu                xm6, [srcq+ssq*1+ 0]
    movu                xm1, [srcq+ssq*1+16]
    lea                srcq, [srcq+ssq*4]
    vinserti128          m6, [srcq+ssq*1+ 0], 1
    vinserti128          m1, [srcq+ssq*1+16], 1
    add                srcq, r6
    shufpd               m7, m6, m1, 0x05
    PUT_8TAP_HV_H         6, 7, 1    ; 1 5
    vpermq               m4, m4, q1100
    vpermq               m5, m5, q3120
    vpermq               m6, m6, q3120
    vpermq               m7, m0, q3120
    punpcklwd            m3, m7, m4  ; 23
    punpckhwd            m4, m5      ; 34
    punpcklwd            m1, m5, m6  ; 01
    punpckhwd            m5, m6      ; 45
    punpcklwd            m2, m6, m7  ; 12
    punpckhwd            m6, m7      ; 56
.hv_w8_loop:
    vpbroadcastd         m9, [v_mul+4*0]
    vpbroadcastd         m7, [v_mul+4*1]
    vpbroadcastd        m10, [v_mul+4*2]
    pmaddwd              m8, m9, m1  ; a0
    pmaddwd              m9, m2      ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddwd              m3, m7      ; a1
    pmaddwd              m4, m7      ; b1
    paddd                m8, m3
    paddd                m9, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddwd              m5, m10     ; a2
    pmaddwd              m6, m10     ; b2
    paddd                m8, m5
    paddd                m9, m6
    movu                xm5, [srcq+ssq*0]
    vinserti128          m5, [srcq+ssq*1], 1
    vbroadcasti128       m7, [subpel_h_shufA]
    vbroadcasti128      m10, [subpel_h_shufB]
    movu                xm6, [srcq+ssq*0+16]
    vinserti128          m6, [srcq+ssq*1+16], 1
    vextracti128     [dstq], m0, 1
    pshufb               m0, m5, m7  ; 01
    pshufb               m5, m10     ; 23
    pmaddwd              m0, m11
    pmaddwd              m5, m12
    paddd                m0, m5
    pshufb               m5, m6, m7  ; 89
    pshufb               m6, m10     ; ab
    pmaddwd              m5, m13
    pmaddwd              m6, m14
    paddd                m6, m5
    movu                xm5, [srcq+ssq*0+8]
    vinserti128          m5, [srcq+ssq*1+8], 1
    lea                srcq, [srcq+ssq*2]
    pshufb               m7, m5, m7
    pshufb               m5, m10
    pmaddwd             m10, m13, m7
    pmaddwd              m7, m11
    paddd                m0, m10
    vpbroadcastd        m10, [pd_512]
    paddd                m6, m7
    pmaddwd              m7, m14, m5
    pmaddwd              m5, m12
    paddd                m0, m7
    paddd                m5, m6
    vbroadcasti128       m6, [dstq]
    paddd                m8, m10
    paddd                m9, m10
    paddd                m0, m10
    paddd                m5, m10
    vpbroadcastd        m10, [v_mul+4*3]
    psrad                m0, 10
    psrad                m5, 10
    packssdw             m0, m5
    vpermq               m7, m0, q3120 ; 7 8
    shufpd               m6, m7, 0x04  ; 6 7
    punpcklwd            m5, m6, m7    ; 67
    punpckhwd            m6, m7        ; 78
    pmaddwd              m7, m10, m5   ; a3
    pmaddwd             m10, m6        ; b3
    paddd                m7, m8
    paddd                m9, m10
    psrad                m7, 10
    psrad                m9, 10
    packusdw             m7, m9
    pminsw               m7, m15
    vpermq               m7, m7, q3120
    mova         [dstq+dsq*0], xm7
    vextracti128 [dstq+dsq*1], m7, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    add                  r7, 16
    add                  r8, 16
    movzx                hd, wb
    mov                srcq, r7
    mov                dstq, r8
    sub                  wd, 1<<8
    jg .hv_w8_loop0
    RET

%if WIN64
DECLARE_REG_TMP 6, 4
%else
DECLARE_REG_TMP 6, 7
%endif

%define PREP_8TAP_FN FN prep_8tap,
PREP_8TAP_FN sharp,          SHARP,   SHARP
PREP_8TAP_FN sharp_smooth,   SHARP,   SMOOTH
PREP_8TAP_FN smooth_sharp,   SMOOTH,  SHARP
PREP_8TAP_FN smooth,         SMOOTH,  SMOOTH
PREP_8TAP_FN sharp_regular,  SHARP,   REGULAR
PREP_8TAP_FN regular_sharp,  REGULAR, SHARP
PREP_8TAP_FN smooth_regular, SMOOTH,  REGULAR
PREP_8TAP_FN regular_smooth, REGULAR, SMOOTH
PREP_8TAP_FN regular,        REGULAR, REGULAR

cglobal prep_8tap_16bpc, 4, 8, 0, tmp, src, stride, w, h, mx, my
%define base r7-prep_avx2
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r7, [prep_avx2]
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    mov                 r6d, r7m ; bitdepth_max
    movzx                wd, word [r7+wq*2+table_offset(prep,)]
    vpbroadcastd         m5, [r7-prep_avx2+pw_8192]
    shr                 r6d, 11
    add                  wq, r7
    vpbroadcastd         m4, [base+prep_mul+r6*4]
    lea                  r6, [strideq*3]
%if WIN64
    pop                  r7
%endif
    jmp                  wq
.h_w4:
    movzx               mxd, mxb
    sub                srcq, 2
    pmovsxbw            xm0, [base+subpel_filters+mxq*8]
    vbroadcasti128       m3, [subpel_h_shufA]
    vbroadcasti128       m4, [subpel_h_shufB]
    WIN64_SPILL_XMM       8
    pshufd              xm0, xm0, q2211
    test          dword r7m, 0x800
    jnz .h_w4_12bpc
    psllw               xm0, 2
.h_w4_12bpc:
    vpbroadcastq         m6, xm0
    vpermq               m7, m0, q1111
.h_w4_loop:
    movu                xm1, [srcq+strideq*0]
    vinserti128          m1, [srcq+strideq*2], 1
    movu                xm2, [srcq+strideq*1]
    vinserti128          m2, [srcq+r6       ], 1
    lea                srcq, [srcq+strideq*4]
    pshufb               m0, m1, m3 ; 0 1 1 2 2 3 3 4
    pshufb               m1, m4     ; 2 3 3 4 4 5 5 6
    pmaddwd              m0, m6
    pmaddwd              m1, m7
    paddd                m0, m5
    paddd                m0, m1
    pshufb               m1, m2, m3
    pshufb               m2, m4
    pmaddwd              m1, m6
    pmaddwd              m2, m7
    paddd                m1, m5
    paddd                m1, m2
    psrad                m0, 4
    psrad                m1, 4
    packssdw             m0, m1
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h:
    test                myd, 0xf00
    jnz .hv
    vpbroadcastd         m5, [prep_8tap_1d_rnd] ; 8 - (8192 << 4)
    lea                  r6, [strideq*3]
    cmp                  wd, 4
    je .h_w4
    shr                 mxd, 16
    sub                srcq, 6
    vpbroadcastq         m0, [base+subpel_filters+mxq*8]
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      12
    vbroadcasti128       m6, [subpel_h_shufA]
    vbroadcasti128       m7, [subpel_h_shufB]
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    test          dword r7m, 0x800
    jnz .h_12bpc
    psllw                m0, 2
.h_12bpc:
    pshufd               m8, m0, q0000
    pshufd               m9, m0, q1111
    pshufd              m10, m0, q2222
    pshufd              m11, m0, q3333
    cmp                  wd, 8
    jg .h_w16
.h_w8:
%macro PREP_8TAP_H 5 ; dst/src+0, src+8, src+16, tmp[1-2]
    pshufb              m%4, m%1, m7   ; 2 3 3 4 4 5 5 6
    pshufb              m%1, m6        ; 0 1 1 2 2 3 3 4
    pmaddwd             m%5, m9, m%4   ; abcd1
    pmaddwd             m%1, m8        ; abcd0
    pshufb              m%2, m7        ; 6 7 7 8 8 9 9 a
    shufpd              m%4, m%2, 0x05 ; 4 5 5 6 6 7 7 8
    paddd               m%5, m5
    paddd               m%1, m%5
    pmaddwd             m%5, m11, m%2  ; abcd3
    paddd               m%1, m%5
    pmaddwd             m%5, m10, m%4  ; abcd2
    pshufb              m%3, m7        ; a b b c c d d e
    pmaddwd             m%4, m8        ; efgh0
    paddd               m%1, m%5
    pmaddwd             m%5, m9, m%2   ; efgh1
    shufpd              m%2, m%3, 0x05 ; 8 9 9 a a b b c
    pmaddwd             m%3, m11       ; efgh3
    pmaddwd             m%2, m10       ; efgh2
    paddd               m%4, m5
    paddd               m%4, m%5
    paddd               m%3, m%4
    paddd               m%2, m%3
    psrad               m%1, 4
    psrad               m%2, 4
    packssdw            m%1, m%2
%endmacro
    movu                xm0, [srcq+strideq*0+ 0]
    vinserti128          m0, [srcq+strideq*1+ 0], 1
    movu                xm2, [srcq+strideq*0+16]
    vinserti128          m2, [srcq+strideq*1+16], 1
    lea                srcq, [srcq+strideq*2]
    shufpd               m1, m0, m2, 0x05
    PREP_8TAP_H           0, 1, 2, 3, 4
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    add                  wd, wd
.h_w16_loop0:
    mov                 r6d, wd
.h_w16_loop:
    movu                 m0, [srcq+r6-32]
    movu                 m1, [srcq+r6-24]
    movu                 m2, [srcq+r6-16]
    PREP_8TAP_H           0, 1, 2, 3, 4
    mova       [tmpq+r6-32], m0
    sub                 r6d, 32
    jg .h_w16_loop
    add                srcq, strideq
    add                tmpq, wq
    dec                  hd
    jg .h_w16_loop0
    RET
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmovle              myd, mxd
    vpbroadcastq         m0, [base+subpel_filters+myq*8]
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      15
    vpbroadcastd         m7, [prep_8tap_1d_rnd]
    lea                  r6, [strideq*3]
    sub                srcq, r6
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    test          dword r7m, 0x800
    jnz .v_12bpc
    psllw                m0, 2
.v_12bpc:
    pshufd               m8, m0, q0000
    pshufd               m9, m0, q1111
    pshufd              m10, m0, q2222
    pshufd              m11, m0, q3333
    cmp                  wd, 4
    jg .v_w8
.v_w4:
    movq                xm1, [srcq+strideq*0]
    vpbroadcastq         m0, [srcq+strideq*1]
    vpbroadcastq         m2, [srcq+strideq*2]
    vpbroadcastq         m4, [srcq+r6       ]
    lea                srcq, [srcq+strideq*4]
    vpbroadcastq         m3, [srcq+strideq*0]
    vpbroadcastq         m5, [srcq+strideq*1]
    vpblendd             m1, m0, 0x30
    vpblendd             m0, m2, 0x30
    punpcklwd            m1, m0      ; 01 12
    vpbroadcastq         m0, [srcq+strideq*2]
    add                srcq, r6
    vpblendd             m2, m4, 0x30
    vpblendd             m4, m3, 0x30
    punpcklwd            m2, m4      ; 23 34
    vpblendd             m3, m5, 0x30
    vpblendd             m5, m0, 0x30
    punpcklwd            m3, m5      ; 45 56
.v_w4_loop:
    vpbroadcastq         m4, [srcq+strideq*0]
    pmaddwd              m5, m8, m1  ; a0 b0
    mova                 m1, m2
    pmaddwd              m2, m9      ; a1 b1
    paddd                m5, m7
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, m10     ; a2 b2
    paddd                m5, m3
    vpblendd             m3, m0, m4, 0x30
    vpbroadcastq         m0, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vpblendd             m4, m0, 0x30
    punpcklwd            m3, m4      ; 67 78
    pmaddwd              m4, m11, m3 ; a3 b3
    paddd                m5, m4
    psrad                m5, 4
    vextracti128        xm4, m5, 1
    packssdw            xm5, xm4
    mova             [tmpq], xm5
    add                tmpq, 16
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
%if WIN64
    push                 r8
%endif
    mov                 r8d, wd
    shl                  wd, 5
    mov                  r5, srcq
    mov                  r7, tmpq
    lea                  wd, [hq+wq-256]
.v_w8_loop0:
    vbroadcasti128       m4, [srcq+strideq*0]
    vbroadcasti128       m5, [srcq+strideq*1]
    vbroadcasti128       m0, [srcq+r6       ]
    vbroadcasti128       m6, [srcq+strideq*2]
    lea                srcq, [srcq+strideq*4]
    vbroadcasti128       m1, [srcq+strideq*0]
    vbroadcasti128       m2, [srcq+strideq*1]
    vbroadcasti128       m3, [srcq+strideq*2]
    add                srcq, r6
    shufpd               m4, m0, 0x0c
    shufpd               m5, m1, 0x0c
    punpcklwd            m1, m4, m5 ; 01
    punpckhwd            m4, m5     ; 34
    shufpd               m6, m2, 0x0c
    punpcklwd            m2, m5, m6 ; 12
    punpckhwd            m5, m6     ; 45
    shufpd               m0, m3, 0x0c
    punpcklwd            m3, m6, m0 ; 23
    punpckhwd            m6, m0     ; 56
.v_w8_loop:
    vbroadcasti128      m14, [srcq+strideq*0]
    pmaddwd             m12, m8, m1  ; a0
    pmaddwd             m13, m8, m2  ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddwd              m3, m9      ; a1
    pmaddwd              m4, m9      ; b1
    paddd               m12, m7
    paddd               m13, m7
    paddd               m12, m3
    paddd               m13, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddwd              m5, m10     ; a2
    pmaddwd              m6, m10     ; b2
    paddd               m12, m5
    vbroadcasti128       m5, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    paddd               m13, m6
    shufpd               m6, m0, m14, 0x0d
    shufpd               m0, m14, m5, 0x0c
    punpcklwd            m5, m6, m0  ; 67
    punpckhwd            m6, m0      ; 78
    pmaddwd             m14, m11, m5 ; a3
    paddd               m12, m14
    pmaddwd             m14, m11, m6 ; b3
    paddd               m13, m14
    psrad               m12, 4
    psrad               m13, 4
    packssdw            m12, m13
    vpermq              m12, m12, q3120
    mova         [tmpq+r8*0], xm12
    vextracti128 [tmpq+r8*2], m12, 1
    lea                tmpq, [tmpq+r8*4]
    sub                  hd, 2
    jg .v_w8_loop
    add                  r5, 16
    add                  r7, 16
    movzx                hd, wb
    mov                srcq, r5
    mov                tmpq, r7
    sub                  wd, 1<<8
    jg .v_w8_loop0
%if WIN64
    pop                  r8
%endif
    RET
.hv:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      16
    vpbroadcastd        m15, [prep_8tap_2d_rnd]
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    vpbroadcastd         m0, [base+subpel_filters+mxq*8+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmovle              myd, mxd
    vpbroadcastq         m1, [base+subpel_filters+myq*8]
    lea                  r6, [strideq*3]
    sub                srcq, 2
    sub                srcq, r6
    pxor                 m7, m7
    punpcklbw            m7, m0
    punpcklbw            m1, m1
    psraw                m7, 4
    psraw                m1, 8
    test          dword r7m, 0x800
    jz .hv_w4_10bit
    psraw                m7, 2
.hv_w4_10bit:
    pshufd              m11, m1, q0000
    pshufd              m12, m1, q1111
    pshufd              m13, m1, q2222
    pshufd              m14, m1, q3333
.hv_w4:
    vbroadcasti128       m9, [subpel_h_shufA]
    vbroadcasti128      m10, [subpel_h_shufB]
    pshufd               m8, m7, q1111
    pshufd               m7, m7, q0000
    movu                xm1, [srcq+strideq*0]
    vinserti128          m1, [srcq+strideq*1], 1     ; 0 1
    vbroadcasti128       m0, [srcq+r6       ]
    vinserti128          m2, m0, [srcq+strideq*2], 0 ; 2 3
    lea                srcq, [srcq+strideq*4]
    vinserti128          m0, [srcq+strideq*0], 1     ; 3 4
    movu                xm3, [srcq+strideq*1]
    vinserti128          m3, [srcq+strideq*2], 1     ; 5 6
    add                srcq, r6
    pshufb               m4, m1, m9
    pshufb               m1, m10
    pmaddwd              m4, m7
    pmaddwd              m1, m8
    pshufb               m5, m2, m9
    pshufb               m2, m10
    pmaddwd              m5, m7
    pmaddwd              m2, m8
    paddd                m4, m15
    paddd                m1, m4
    pshufb               m4, m0, m9
    pshufb               m0, m10
    pmaddwd              m4, m7
    pmaddwd              m0, m8
    paddd                m5, m15
    paddd                m2, m5
    pshufb               m5, m3, m9
    pshufb               m3, m10
    pmaddwd              m5, m7
    pmaddwd              m3, m8
    paddd                m4, m15
    paddd                m4, m0
    paddd                m5, m15
    paddd                m5, m3
    vperm2i128           m0, m1, m2, 0x21
    psrld                m1, 6
    psrld                m2, 6
    vperm2i128           m3, m4, m5, 0x21
    pslld                m4, 10
    pslld                m5, 10
    pblendw              m2, m4, 0xaa ; 23 34
    pslld                m0, 10
    pblendw              m1, m0, 0xaa ; 01 12
    psrld                m3, 6
    pblendw              m3, m5, 0xaa ; 45 56
    psrad                m0, m5, 16
.hv_w4_loop:
    movu                xm4, [srcq+strideq*0]
    vinserti128          m4, [srcq+strideq*1], 1
    lea                srcq, [srcq+strideq*2]
    pmaddwd              m5, m11, m1   ; a0 b0
    mova                 m1, m2
    pmaddwd              m2, m12       ; a1 b1
    paddd                m5, m15
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, m13       ; a2 b2
    paddd                m5, m3
    pshufb               m3, m4, m9
    pshufb               m4, m10
    pmaddwd              m3, m7
    pmaddwd              m4, m8
    paddd                m3, m15
    paddd                m4, m3
    psrad                m4, 6
    packssdw             m0, m4        ; _ 7 6 8
    vpermq               m3, m0, q1122 ; _ 6 _ 7
    punpckhwd            m3, m0        ; 67 78
    mova                 m0, m4
    pmaddwd              m4, m14, m3   ; a3 b3
    paddd                m4, m5
    psrad                m4, 6
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
    mova             [tmpq], xm4
    add                tmpq, 16
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    shr                 mxd, 16
    vpbroadcastq         m2, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmovle              myd, mxd
    pmovsxbw            xm1, [base+subpel_filters+myq*8]
%if WIN64
    PUSH                 r8
%endif
    mov                 r8d, wd
    shl                  wd, 5
    lea                  r6, [strideq*3]
    sub                srcq, 6
    sub                srcq, r6
    mov                  r5, srcq
    mov                  r7, tmpq
    lea                  wd, [hq+wq-256]
    pxor                 m0, m0
    punpcklbw            m0, m2
    mova            [v_mul], xm1
    psraw                m0, 4
    test          dword r7m, 0x800
    jz .hv_w8_10bit
    psraw                m0, 2
.hv_w8_10bit:
    pshufd              m11, m0, q0000
    pshufd              m12, m0, q1111
    pshufd              m13, m0, q2222
    pshufd              m14, m0, q3333
.hv_w8_loop0:
%macro PREP_8TAP_HV_H 3 ; dst/src+0, src+8, src+16
    pshufb               m2, m%1, m9   ; 2 3 3 4 4 5 5 6
    pshufb              m%1, m8        ; 0 1 1 2 2 3 3 4
    pmaddwd              m3, m12, m2
    pmaddwd             m%1, m11
    pshufb              m%2, m9        ; 6 7 7 8 8 9 9 a
    shufpd               m2, m%2, 0x05 ; 4 5 5 6 6 7 7 8
    paddd                m3, m15
    paddd               m%1, m3
    pmaddwd              m3, m14, m%2
    paddd               m%1, m3
    pmaddwd              m3, m13, m2
    pshufb              m%3, m9        ; a b b c c d d e
    pmaddwd              m2, m11
    paddd               m%1, m3
    pmaddwd              m3, m12, m%2
    shufpd              m%2, m%3, 0x05 ; 8 9 9 a a b b c
    pmaddwd             m%3, m14
    pmaddwd             m%2, m13
    paddd                m2, m15
    paddd                m2, m3
    paddd                m2, m%3
    paddd                m2, m%2
    psrad               m%1, 6
    psrad                m2, 6
    packssdw            m%1, m2
%endmacro
    movu                xm4, [srcq+r6       + 0]
    vbroadcasti128       m8, [subpel_h_shufA]
    movu                xm6, [srcq+r6       + 8]
    vbroadcasti128       m9, [subpel_h_shufB]
    movu                xm0, [srcq+r6       +16]
    movu                xm5, [srcq+strideq*0+ 0]
    vinserti128          m5, [srcq+strideq*4+ 0], 1
    movu                xm1, [srcq+strideq*0+16]
    vinserti128          m1, [srcq+strideq*4+16], 1
    shufpd               m7, m5, m1, 0x05
    INIT_XMM avx2
    PREP_8TAP_HV_H        4, 6, 0    ; 3
    INIT_YMM avx2
    PREP_8TAP_HV_H        5, 7, 1    ; 0 4
    movu                xm0, [srcq+strideq*2+ 0]
    vinserti128          m0, [srcq+r6     *2+ 0], 1
    movu                xm1, [srcq+strideq*2+16]
    vinserti128          m1, [srcq+r6     *2+16], 1
    shufpd               m7, m0, m1, 0x05
    PREP_8TAP_HV_H        0, 7, 1    ; 2 6
    movu                xm6, [srcq+strideq*1+ 0]
    movu                xm1, [srcq+strideq*1+16]
    lea                srcq, [srcq+strideq*4]
    vinserti128          m6, [srcq+strideq*1+ 0], 1
    vinserti128          m1, [srcq+strideq*1+16], 1
    add                srcq, r6
    shufpd               m7, m6, m1, 0x05
    PREP_8TAP_HV_H        6, 7, 1    ; 1 5
    vpermq               m4, m4, q1100
    vpermq               m5, m5, q3120
    vpermq               m6, m6, q3120
    vpermq               m7, m0, q3120
    punpcklwd            m3, m7, m4  ; 23
    punpckhwd            m4, m5      ; 34
    punpcklwd            m1, m5, m6  ; 01
    punpckhwd            m5, m6      ; 45
    punpcklwd            m2, m6, m7  ; 12
    punpckhwd            m6, m7      ; 56
.hv_w8_loop:
    vpbroadcastd         m9, [v_mul+4*0]
    vpbroadcastd         m7, [v_mul+4*1]
    vpbroadcastd        m10, [v_mul+4*2]
    pmaddwd              m8, m9, m1  ; a0
    pmaddwd              m9, m2      ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddwd              m3, m7      ; a1
    pmaddwd              m4, m7      ; b1
    paddd                m8, m15
    paddd                m9, m15
    paddd                m8, m3
    paddd                m9, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddwd              m5, m10     ; a2
    pmaddwd              m6, m10     ; b2
    paddd                m8, m5
    paddd                m9, m6
    movu                xm5, [srcq+strideq*0]
    vinserti128          m5, [srcq+strideq*1], 1
    vbroadcasti128       m7, [subpel_h_shufA]
    vbroadcasti128      m10, [subpel_h_shufB]
    movu                xm6, [srcq+strideq*0+16]
    vinserti128          m6, [srcq+strideq*1+16], 1
    vextracti128     [tmpq], m0, 1
    pshufb               m0, m5, m7  ; 01
    pshufb               m5, m10     ; 23
    pmaddwd              m0, m11
    pmaddwd              m5, m12
    paddd                m0, m15
    paddd                m0, m5
    pshufb               m5, m6, m7  ; 89
    pshufb               m6, m10     ; ab
    pmaddwd              m5, m13
    pmaddwd              m6, m14
    paddd                m5, m15
    paddd                m6, m5
    movu                xm5, [srcq+strideq*0+8]
    vinserti128          m5, [srcq+strideq*1+8], 1
    lea                srcq, [srcq+strideq*2]
    pshufb               m7, m5, m7
    pshufb               m5, m10
    pmaddwd             m10, m13, m7
    pmaddwd              m7, m11
    paddd                m0, m10
    paddd                m6, m7
    pmaddwd              m7, m14, m5
    pmaddwd              m5, m12
    paddd                m0, m7
    paddd                m5, m6
    vbroadcasti128       m6, [tmpq]
    vpbroadcastd        m10, [v_mul+4*3]
    psrad                m0, 6
    psrad                m5, 6
    packssdw             m0, m5
    vpermq               m7, m0, q3120 ; 7 8
    shufpd               m6, m7, 0x04  ; 6 7
    punpcklwd            m5, m6, m7    ; 67
    punpckhwd            m6, m7        ; 78
    pmaddwd              m7, m10, m5   ; a3
    pmaddwd             m10, m6        ; b3
    paddd                m7, m8
    paddd                m9, m10
    psrad                m7, 6
    psrad                m9, 6
    packssdw             m7, m9
    vpermq               m7, m7, q3120
    mova         [tmpq+r8*0], xm7
    vextracti128 [tmpq+r8*2], m7, 1
    lea                tmpq, [tmpq+r8*4]
    sub                  hd, 2
    jg .hv_w8_loop
    add                  r5, 16
    add                  r7, 16
    movzx                hd, wb
    mov                srcq, r5
    mov                tmpq, r7
    sub                  wd, 1<<8
    jg .hv_w8_loop0
%if WIN64
    POP                  r8
%endif
    RET

%macro movifprep 2
 %if isprep
    mov %1, %2
 %endif
%endmacro

%macro REMAP_REG 2
 %xdefine r%1  r%2
 %xdefine r%1q r%2q
 %xdefine r%1d r%2d
%endmacro

%macro MCT_8TAP_SCALED_REMAP_REGS_TO_PREV 0
 %if isprep
  %xdefine r14_save r14
  %assign %%i 14
  %rep 14
   %assign %%j %%i-1
   REMAP_REG %%i, %%j
   %assign %%i %%i-1
  %endrep
 %endif
%endmacro

%macro MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT 0
 %if isprep
  %assign %%i 1
  %rep 13
   %assign %%j %%i+1
   REMAP_REG %%i, %%j
   %assign %%i %%i+1
  %endrep
  %xdefine r14 r14_save
  %undef r14_save
 %endif
%endmacro

%macro MC_8TAP_SCALED_RET 0-1 1 ; leave_mapping_unchanged
    MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT
    RET
 %if %1
    MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %endif
%endmacro

%macro MC_8TAP_SCALED_H 8-9 0 ; dst, tmp[0-6], load_hrnd
    movu               xm%1, [srcq+ r4*2]
    movu               xm%2, [srcq+ r6*2]
    movu               xm%3, [srcq+ r7*2]
    movu               xm%4, [srcq+ r9*2]
    vinserti128         m%1, [srcq+r10*2], 1
    vinserti128         m%2, [srcq+r11*2], 1
    vinserti128         m%3, [srcq+r13*2], 1
    vinserti128         m%4, [srcq+ rX*2], 1
    add                srcq, ssq
    movu               xm%5, [srcq+ r4*2]
    movu               xm%6, [srcq+ r6*2]
    movu               xm%7, [srcq+ r7*2]
    movu               xm%8, [srcq+ r9*2]
    vinserti128         m%5, [srcq+r10*2], 1
    vinserti128         m%6, [srcq+r11*2], 1
    vinserti128         m%7, [srcq+r13*2], 1
    vinserti128         m%8, [srcq+ rX*2], 1
    add                srcq, ssq
    pmaddwd             m%1, m12
    pmaddwd             m%2, m13
    pmaddwd             m%3, m14
    pmaddwd             m%4, m15
    pmaddwd             m%5, m12
    pmaddwd             m%6, m13
    pmaddwd             m%7, m14
    pmaddwd             m%8, m15
    phaddd              m%1, m%2
 %if %9
    mova                m10, [rsp+0x00]
 %endif
    phaddd              m%3, m%4
    phaddd              m%5, m%6
    phaddd              m%7, m%8
    phaddd              m%1, m%3
    phaddd              m%5, m%7
    paddd               m%1, m10
    paddd               m%5, m10
    psrad               m%1, xm11
    psrad               m%5, xm11
    packssdw            m%1, m%5
%endmacro

%macro MC_8TAP_SCALED 1
%ifidn %1, put
 %assign isput  1
 %assign isprep 0
cglobal put_8tap_scaled_16bpc, 4, 14, 16, 0xe0, dst, ds, src, ss, w, h, mx, my, dx, dy, pxmax
 %xdefine base_reg r12
    mov                 r7d, pxmaxm
%else
 %assign isput  0
 %assign isprep 1
cglobal prep_8tap_scaled_16bpc, 4, 14, 16, 0xe0, tmp, src, ss, w, h, mx, my, dx, dy, pxmax
  %define tmp_stridem qword [rsp+0xd0]
 %xdefine base_reg r11
%endif
    lea            base_reg, [%1_8tap_scaled_16bpc_avx2]
%define base base_reg-%1_8tap_scaled_16bpc_avx2
    tzcnt                wd, wm
    vpbroadcastd         m8, dxm
%if isprep && UNIX64
    movd               xm10, mxd
    vpbroadcastd        m10, xm10
    mov                 r5d, t0d
 DECLARE_REG_TMP 5, 7
    mov                 r6d, pxmaxm
%else
    vpbroadcastd        m10, mxm
 %if isput
    vpbroadcastw        m11, pxmaxm
 %else
    mov                 r6d, pxmaxm
 %endif
%endif
    mov                 dyd, dym
%if isput
 %if WIN64
    mov                 r8d, hm
  DEFINE_ARGS dst, ds, src, ss, w, _, _, my, h, dy, ss3
  %define hm r5m
  %define dxm r8m
 %else
  DEFINE_ARGS dst, ds, src, ss, w, h, _, my, dx, dy, ss3
  %define hm r6m
 %endif
 %define dsm [rsp+0x98]
 %define rX r1
 %define rXd r1d
%else ; prep
 %if WIN64
    mov                 r7d, hm
  DEFINE_ARGS tmp, src, ss, w, _, _, my, h, dy, ss3
  %define hm r4m
  %define dxm r7m
 %else
  DEFINE_ARGS tmp, src, ss, w, h, _, my, dx, dy, ss3
  %define hm [rsp+0x98]
 %endif
 MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %define rX r14
 %define rXd r14d
%endif
    shr                 r7d, 11
    vpbroadcastd         m6, [base+pd_0x3ff]
    vpbroadcastd        m12, [base+s_8tap_h_rnd+r7*4]
    movd                xm7, [base+s_8tap_h_sh+r7*4]
%if isput
    vpbroadcastd        m13, [base+put_s_8tap_v_rnd+r7*4]
    pinsrd              xm7, [base+put_s_8tap_v_sh+r7*4], 2
%else
    vpbroadcastd        m13, [base+pd_m524256]
%endif
    pxor                 m9, m9
    lea                ss3q, [ssq*3]
    movzx               r7d, t1b
    shr                 t1d, 16
    cmp                  hd, 6
    cmovs               t1d, r7d
    sub                srcq, ss3q
    cmp                 dyd, 1024
    je .dy1
    cmp                 dyd, 2048
    je .dy2
    movzx                wd, word [base+%1_8tap_scaled_avx2_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%if isput
.w2:
    mov                 myd, mym
    movzx               t0d, t0b
    sub                srcq, 2
    movd               xm15, t0d
    punpckldq            m8, m9, m8
    paddd               m10, m8 ; mx+dx*[0,1]
    vpbroadcastd       xm14, [base+pq_0x40000000+2]
    vpbroadcastd       xm15, xm15
    pand                xm8, xm10, xm6
    psrld               xm8, 6
    paddd              xm15, xm8
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    vbroadcasti128       m5, [base+bdct_lb_q]
    vbroadcasti128       m6, [base+subpel_s_shuf2]
    vpbroadcastd       xm15, [base+subpel_filters+r4*8+2]
    vpbroadcastd        xm4, [base+subpel_filters+r6*8+2]
    pcmpeqd             xm8, xm9
    psrld               m10, 10
    paddd               m10, m10
    movu                xm0, [srcq+ssq*0]
    movu                xm1, [srcq+ssq*1]
    movu                xm2, [srcq+ssq*2]
    movu                xm3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pshufb              m10, m5
    paddb               m10, m6
    vpblendd           xm15, xm4, 0xa
    pblendvb           xm15, xm14, xm8
    pmovsxbw            m15, xm15
    vinserti128          m0, [srcq+ssq*0], 1 ; 0 4
    vinserti128          m1, [srcq+ssq*1], 1 ; 1 5
    vinserti128          m2, [srcq+ssq*2], 1 ; 2 6
    vinserti128          m3, [srcq+ss3q ], 1 ; 3 7
    lea                srcq, [srcq+ssq*4]
    REPX    {pshufb x, m10}, m0, m1, m2, m3
    REPX   {pmaddwd x, m15}, m0, m1, m2, m3
    phaddd               m0, m1
    phaddd               m2, m3
    paddd                m0, m12
    paddd                m2, m12
    psrad                m0, xm7
    psrad                m2, xm7
    packssdw             m0, m2             ; 0 1 2 3  4 5 6 7
    vextracti128        xm1, m0, 1
    palignr             xm2, xm1, xm0, 4    ; 1 2 3 4
    punpcklwd           xm3, xm0, xm2       ; 01 12
    punpckhwd           xm0, xm2            ; 23 34
    pshufd              xm4, xm1, q0321     ; 5 6 7 _
    punpcklwd           xm2, xm1, xm4       ; 45 56
    punpckhwd           xm4, xm1, xm4       ; 67 __
.w2_loop:
    and                 myd, 0x3ff
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq               xm14, r6q
    pmovsxbw           xm14, xm14
    pshufd              xm8, xm14, q0000
    pshufd              xm9, xm14, q1111
    pmaddwd             xm5, xm3, xm8
    pmaddwd             xm6, xm0, xm9
    pshufd              xm8, xm14, q2222
    pshufd             xm14, xm14, q3333
    paddd               xm5, xm6
    pmaddwd             xm6, xm2, xm8
    pmaddwd             xm8, xm4, xm14
    psrldq              xm9, xm7, 8
    paddd               xm5, xm6
    paddd               xm5, xm13
    paddd               xm5, xm8
    psrad               xm5, xm9
    packusdw            xm5, xm5
    pminsw              xm5, xm11
    movd             [dstq], xm5
    add                dstq, dsq
    dec                  hd
    jz .ret
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .w2_loop
    movu                xm5, [srcq]
    test                myd, 0x400
    jz .w2_skip_line
    add                srcq, ssq
    shufps              xm3, xm0, q1032     ; 01 12
    shufps              xm0, xm2, q1032     ; 23 34
    shufps              xm2, xm4, q1032     ; 45 56
    pshufb              xm5, xm10
    pmaddwd             xm5, xm15
    phaddd              xm5, xm5
    paddd               xm5, xm12
    psrad               xm5, xm7
    packssdw            xm5, xm5
    palignr             xm1, xm5, xm1, 12
    punpcklqdq          xm1, xm1            ; 6 7 6 7
    punpcklwd           xm4, xm1, xm5       ; 67 __
    jmp .w2_loop
.w2_skip_line:
    movu                xm6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                xm3, xm0            ; 01 12
    mova                xm0, xm2            ; 23 34
    pshufb              xm5, xm10
    pshufb              xm6, xm10
    pmaddwd             xm5, xm15
    pmaddwd             xm6, xm15
    phaddd              xm5, xm6
    paddd               xm5, xm12
    psrad               xm5, xm7
    packssdw            xm5, xm5            ; 6 7 6 7
    palignr             xm1, xm5, xm1, 8    ; 4 5 6 7
    pshufd              xm5, xm1, q0321     ; 5 6 7 _
    punpcklwd           xm2, xm1, xm5       ; 45 56
    punpckhwd           xm4, xm1, xm5       ; 67 __
    jmp .w2_loop
%endif
.w4:
    mov                 myd, mym
    mova         [rsp+0x00], m12
%if isput
    mova         [rsp+0x20], xm13
%else
    SWAP                m11, m13
%endif
    mova         [rsp+0x30], xm7
    vbroadcasti128       m7, [base+rescale_mul]
    movzx               t0d, t0b
    sub                srcq, 2
    movd               xm15, t0d
    pmaddwd              m8, m7
    vpbroadcastq         m2, [base+pq_0x40000000+1]
    vpbroadcastd       xm15, xm15
    SWAP                m13, m10
    paddd               m13, m8 ; mx+dx*[0-3]
    pand                 m6, m13
    psrld                m6, 6
    paddd              xm15, xm6
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    pextrd             r11d, xm15, 2
    pextrd             r13d, xm15, 3
    vbroadcasti128       m5, [base+bdct_lb_q+ 0]
    vbroadcasti128       m1, [base+bdct_lb_q+16]
    vbroadcasti128       m0, [base+subpel_s_shuf2]
    vpbroadcastd       xm14, [base+subpel_filters+r4*8+2]
    vpbroadcastd        xm7, [base+subpel_filters+r6*8+2]
    vpbroadcastd       xm15, [base+subpel_filters+r11*8+2]
    vpbroadcastd        xm8, [base+subpel_filters+r13*8+2]
    pcmpeqd              m6, m9
    punpckldq           m10, m6, m6
    punpckhdq            m6, m6
    psrld               m13, 10
    paddd               m13, m13
    vpblendd           xm14, xm7, 0xa
    vpblendd           xm15, xm8, 0xa
    pmovsxbw            m14, xm14
    pmovsxbw            m15, xm15
    pblendvb            m14, m2, m10
    pblendvb            m15, m2, m6
    pextrd               r4, xm13, 2
    pshufb              m12, m13, m5
    pshufb              m13, m1
    lea                  r6, [r4+ssq*1]
    lea                 r11, [r4+ssq*2]
    lea                 r13, [r4+ss3q ]
    movu                xm7, [srcq+ssq*0]
    movu                xm9, [srcq+ssq*1]
    movu                xm8, [srcq+ssq*2]
    movu               xm10, [srcq+ss3q ]
    movu                xm1, [srcq+r4   ]
    movu                xm3, [srcq+r6   ]
    movu                xm2, [srcq+r11  ]
    movu                xm4, [srcq+r13  ]
    lea                srcq, [srcq+ssq*4]
    vinserti128          m7, [srcq+ssq*0], 1
    vinserti128          m9, [srcq+ssq*1], 1
    vinserti128          m8, [srcq+ssq*2], 1
    vinserti128         m10, [srcq+ss3q ], 1
    vinserti128          m1, [srcq+r4   ], 1
    vinserti128          m3, [srcq+r6   ], 1
    vinserti128          m2, [srcq+r11  ], 1
    vinserti128          m4, [srcq+r13  ], 1
    lea                srcq, [srcq+ssq*4]
    vpbroadcastb         m5, xm13
    psubb               m13, m5
    paddb               m12, m0
    paddb               m13, m0
    REPX    {pshufb x, m12}, m7, m9, m8, m10
    REPX   {pmaddwd x, m14}, m7, m9, m8, m10
    REPX    {pshufb x, m13}, m1, m2, m3, m4
    REPX   {pmaddwd x, m15}, m1, m2, m3, m4
    mova                 m5, [rsp+0x00]
    movd                xm6, [rsp+0x30]
    phaddd               m7, m1
    phaddd               m9, m3
    phaddd               m8, m2
    phaddd              m10, m4
    REPX      {paddd x, m5}, m7, m9, m8, m10
    REPX     {psrad x, xm6}, m7, m9, m8, m10
    packssdw             m7, m9                 ; 0 1  4 5
    packssdw             m8, m10                ; 2 3  6 7
    vextracti128        xm9, m7, 1              ; 4 5
    vextracti128        xm3, m8, 1              ; 6 7
    shufps              xm4, xm7, xm8, q1032    ; 1 2
    shufps              xm5, xm8, xm9, q1032    ; 3 4
    shufps              xm6, xm9, xm3, q1032    ; 5 6
    psrldq             xm10, xm3, 8             ; 7 _
    punpcklwd           xm0, xm7, xm4   ; 01
    punpckhwd           xm7, xm4        ; 12
    punpcklwd           xm1, xm8, xm5   ; 23
    punpckhwd           xm8, xm5        ; 34
    punpcklwd           xm2, xm9, xm6   ; 45
    punpckhwd           xm9, xm6        ; 56
    punpcklwd           xm3, xm10       ; 67
    mova         [rsp+0x40], xm7
    mova         [rsp+0x50], xm8
    mova         [rsp+0x60], xm9
.w4_loop:
    and                 myd, 0x3ff
    mov                r11d, 64 << 24
    mov                r13d, myd
    shr                r13d, 6
    lea                r13d, [t1+r13]
    cmovnz             r11q, [base+subpel_filters+r13*8]
    movq                xm9, r11q
    pmovsxbw            xm9, xm9
    pshufd              xm7, xm9, q0000
    pshufd              xm8, xm9, q1111
    pmaddwd             xm4, xm0, xm7
    pmaddwd             xm5, xm1, xm8
    pshufd              xm7, xm9, q2222
    pshufd              xm9, xm9, q3333
    pmaddwd             xm6, xm2, xm7
    pmaddwd             xm8, xm3, xm9
%if isput
    mova                xm7, [rsp+0x20]
    movd                xm9, [rsp+0x38]
%else
    SWAP                 m7, m11
%endif
    paddd               xm4, xm5
    paddd               xm6, xm8
    paddd               xm4, xm6
    paddd               xm4, xm7
%if isput
    psrad               xm4, xm9
    packusdw            xm4, xm4
    pminuw              xm4, xm11
    movq             [dstq], xm4
    add                dstq, dsq
%else
    SWAP                m11, m7
    psrad               xm4, 6
    packssdw            xm4, xm4
    movq             [tmpq], xm4
    add                tmpq, 8
%endif
    dec                  hd
    jz .ret
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .w4_loop
    mova                xm8, [rsp+0x00]
    movd                xm9, [rsp+0x30]
    movu                xm4, [srcq]
    movu                xm5, [srcq+r4]
    test                myd, 0x400
    jz .w4_skip_line
    mova                xm0, [rsp+0x40]
    mova         [rsp+0x40], xm1
    mova                xm1, [rsp+0x50]
    mova         [rsp+0x50], xm2
    mova                xm2, [rsp+0x60]
    mova         [rsp+0x60], xm3
    pshufb              xm4, xm12
    pshufb              xm5, xm13
    pmaddwd             xm4, xm14
    pmaddwd             xm5, xm15
    phaddd              xm4, xm5
    paddd               xm4, xm8
    psrad               xm4, xm9
    packssdw            xm4, xm4
    punpcklwd           xm3, xm10, xm4
    mova               xm10, xm4
    add                srcq, ssq
    jmp .w4_loop
.w4_skip_line:
    movu                xm6, [srcq+ssq*1]
    movu                xm7, [srcq+r6]
    movu                 m0, [rsp+0x50]
    pshufb              xm4, xm12
    pshufb              xm6, xm12
    pshufb              xm5, xm13
    pshufb              xm7, xm13
    pmaddwd             xm4, xm14
    pmaddwd             xm6, xm14
    pmaddwd             xm5, xm15
    pmaddwd             xm7, xm15
    mova         [rsp+0x40], m0
    phaddd              xm4, xm5
    phaddd              xm6, xm7
    paddd               xm4, xm8
    paddd               xm6, xm8
    psrad               xm4, xm9
    psrad               xm6, xm9
    packssdw            xm4, xm6
    punpcklwd           xm9, xm10, xm4
    mova         [rsp+0x60], xm9
    psrldq             xm10, xm4, 8
    mova                xm0, xm1
    mova                xm1, xm2
    mova                xm2, xm3
    punpcklwd           xm3, xm4, xm10
    lea                srcq, [srcq+ssq*2]
    jmp .w4_loop
    SWAP                m10, m13
%if isprep
    SWAP                m13, m11
%endif
.w8:
    mov    dword [rsp+0x80], 1
    movifprep   tmp_stridem, 16
    jmp .w_start
.w16:
    mov    dword [rsp+0x80], 2
    movifprep   tmp_stridem, 32
    jmp .w_start
.w32:
    mov    dword [rsp+0x80], 4
    movifprep   tmp_stridem, 64
    jmp .w_start
.w64:
    mov    dword [rsp+0x80], 8
    movifprep   tmp_stridem, 128
    jmp .w_start
.w128:
    mov    dword [rsp+0x80], 16
    movifprep   tmp_stridem, 256
.w_start:
    SWAP                m10, m12, m1
    SWAP                m11, m7
    ; m1=mx, m7=pxmax, m10=h_rnd, m11=h_sh, m12=free
%if isput
    movifnidn           dsm, dsq
    mova         [rsp+0xb0], xm7
%endif
    mova         [rsp+0x00], m10
    mova         [rsp+0x20], m13
    shr                 t0d, 16
    sub                srcq, 6
    pmaddwd              m8, [base+rescale_mul2]
    movd               xm15, t0d
    mov          [rsp+0x84], t0d
    mov          [rsp+0x88], srcq
    mov          [rsp+0x90], r0q ; dstq / tmpq
%if UNIX64
    mov                  hm, hd
%endif
    shl           dword dxm, 3 ; dx*8
    vpbroadcastd        m15, xm15
    paddd                m1, m8 ; mx+dx*[0-7]
    jmp .hloop
.hloop_prep:
    dec    dword [rsp+0x80]
    jz .ret
    add    qword [rsp+0x90], 16
    mov                  hd, hm
    vpbroadcastd         m8, dxm
    vpbroadcastd         m6, [base+pd_0x3ff]
    paddd                m1, m8, [rsp+0x40]
    vpbroadcastd        m15, [rsp+0x84]
    pxor                 m9, m9
    mov                srcq, [rsp+0x88]
    mov                 r0q, [rsp+0x90] ; dstq / tmpq
.hloop:
    vpbroadcastq        xm2, [base+pq_0x40000000]
    pand                 m5, m1, m6
    psrld                m5, 6
    paddd               m15, m5
    pcmpeqd              m5, m9
    vextracti128        xm7, m15, 1
    movq                 r6, xm15
    pextrq               r9, xm15, 1
    movq                r11, xm7
    pextrq               rX, xm7, 1
    mov                 r4d, r6d
    shr                  r6, 32
    mov                 r7d, r9d
    shr                  r9, 32
    mov                r10d, r11d
    shr                 r11, 32
    mov                r13d, rXd
    shr                  rX, 32
    mova         [rsp+0x40], m1
    movq               xm12, [base+subpel_filters+ r4*8]
    movq               xm13, [base+subpel_filters+ r6*8]
    movhps             xm12, [base+subpel_filters+ r7*8]
    movhps             xm13, [base+subpel_filters+ r9*8]
    movq               xm14, [base+subpel_filters+r10*8]
    movq               xm15, [base+subpel_filters+r11*8]
    movhps             xm14, [base+subpel_filters+r13*8]
    movhps             xm15, [base+subpel_filters+ rX*8]
    psrld                m1, 10
    vextracti128        xm7, m1, 1
    vextracti128        xm6, m5, 1
    movq         [rsp+0xa0], xm1
    movq         [rsp+0xa8], xm7
    movq                 r6, xm1
    pextrq              r11, xm1, 1
    movq                 r9, xm7
    pextrq               rX, xm7, 1
    mov                 r4d, r6d
    shr                  r6, 32
    mov                r10d, r11d
    shr                 r11, 32
    mov                 r7d, r9d
    shr                  r9, 32
    mov                r13d, rXd
    shr                  rX, 32
    pshufd              xm4, xm5, q2200
    pshufd              xm5, xm5, q3311
    pshufd              xm7, xm6, q2200
    pshufd              xm6, xm6, q3311
    pblendvb           xm12, xm2, xm4
    pblendvb           xm13, xm2, xm5
    pblendvb           xm14, xm2, xm7
    pblendvb           xm15, xm2, xm6
    pmovsxbw            m12, xm12
    pmovsxbw            m13, xm13
    pmovsxbw            m14, xm14
    pmovsxbw            m15, xm15
    MC_8TAP_SCALED_H 0, 1, 2, 3, 4, 5, 6, 7 ; 0a 1a 0b 1b
    mova        [rsp+0x60], m0
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 8 ; 2a 3a 2b 3b
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 7, 8, 9 ; 4a 5a 4b 5b
    MC_8TAP_SCALED_H 3, 4, 5, 6, 7, 8, 9, 0 ; 6a 7a 6b 7b
    mova                 m0, [rsp+0x60]
    vbroadcasti128       m9, [base+subpel_s_shuf8]
    mov                 myd, mym
    mov                 dyd, dym
    pshufb               m0, m9     ; 01a 01b
    pshufb               m1, m9     ; 23a 23b
    pshufb               m2, m9     ; 45a 45b
    pshufb               m3, m9     ; 67a 67b
.vloop:
    and                 myd, 0x3ff
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq                xm9, r6q
    punpcklqdq          xm9, xm9
    pmovsxbw             m9, xm9
    pshufd               m8, m9, q0000
    pshufd               m7, m9, q1111
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m7
    pshufd               m8, m9, q2222
    pshufd               m9, m9, q3333
    pmaddwd              m6, m2, m8
    pmaddwd              m7, m3, m9
%if isput
    psrldq              xm8, xm11, 8
%endif
    paddd                m4, [rsp+0x20]
    paddd                m6, m7
    paddd                m4, m5
    paddd                m4, m6
%if isput
    psrad                m4, xm8
    vextracti128        xm5, m4, 1
    packusdw            xm4, xm5
    pminsw              xm4, [rsp+0xb0]
    mova             [dstq], xm4
    add                dstq, dsm
%else
    psrad                m4, 6
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
    mova             [tmpq], xm4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .hloop_prep
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .vloop
    test                myd, 0x400
    mov          [rsp+0x60], myd
    mov                 r4d, [rsp+0xa0]
    mov                 r6d, [rsp+0xa4]
    mov                 r7d, [rsp+0xa8]
    mov                 r9d, [rsp+0xac]
    jz .skip_line
    vbroadcasti128       m9, [base+wswap]
    movu                xm4, [srcq+ r4*2]
    movu                xm5, [srcq+ r6*2]
    movu                xm6, [srcq+ r7*2]
    movu                xm7, [srcq+ r9*2]
    vinserti128          m4, [srcq+r10*2], 1
    vinserti128          m5, [srcq+r11*2], 1
    vinserti128          m6, [srcq+r13*2], 1
    vinserti128          m7, [srcq+ rX*2], 1
    add                srcq, ssq
    mov                 myd, [rsp+0x60]
    mov                 dyd, dym
    pshufb               m0, m9
    pshufb               m1, m9
    pshufb               m2, m9
    pshufb               m3, m9
    pmaddwd              m4, m12
    pmaddwd              m5, m13
    pmaddwd              m6, m14
    pmaddwd              m7, m15
    phaddd               m4, m5
    phaddd               m6, m7
    phaddd               m4, m6
    paddd                m4, m10
    psrad                m4, xm11
    pslld                m4, 16
    pblendw              m0, m1, 0xaa
    pblendw              m1, m2, 0xaa
    pblendw              m2, m3, 0xaa
    pblendw              m3, m4, 0xaa
    jmp .vloop
.skip_line:
    mova                 m0, m1
    mova                 m1, m2
    mova                 m2, m3
    MC_8TAP_SCALED_H      3, 10, 4, 5, 6, 7, 8, 9, 1
    vbroadcasti128       m9, [base+subpel_s_shuf8]
    mov                 myd, [rsp+0x60]
    mov                 dyd, dym
    pshufb               m3, m9
    jmp .vloop
    SWAP                 m1, m12, m10
    SWAP                 m7, m11
.dy1:
    movzx                wd, word [base+%1_8tap_scaled_avx2_dy1_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%if isput
.dy1_w2:
    mov                 myd, mym
    movzx               t0d, t0b
    sub                srcq, 2
    movd               xm15, t0d
    punpckldq            m8, m9, m8
    paddd               m10, m8 ; mx+dx*[0-1]
    vpbroadcastd       xm14, [base+pq_0x40000000+2]
    vpbroadcastd       xm15, xm15
    pand                xm8, xm10, xm6
    psrld               xm8, 6
    paddd              xm15, xm8
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    vbroadcasti128       m5, [base+bdct_lb_q]
    vbroadcasti128       m6, [base+subpel_s_shuf2]
    vpbroadcastd        m15, [base+subpel_filters+r4*8+2]
    vpbroadcastd         m4, [base+subpel_filters+r6*8+2]
    pcmpeqd             xm8, xm9
    psrld               m10, 10
    paddd               m10, m10
    movu                xm0, [srcq+ssq*0]
    movu                xm1, [srcq+ssq*1]
    movu                xm2, [srcq+ssq*2]
    movu                xm3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pshufb              m10, m5
    paddb               m10, m6
    vpblendd           xm15, xm4, 0xa
    pblendvb           xm15, xm14, xm8
    pmovsxbw            m15, xm15
    vinserti128          m0, [srcq+ssq*0], 1
    vinserti128          m1, [srcq+ssq*1], 1
    vinserti128          m2, [srcq+ssq*2], 1
    add                srcq, ss3q
    movq                xm6, r4q
    pmovsxbw            xm6, xm6
    pshufd              xm8, xm6, q0000
    pshufd              xm9, xm6, q1111
    pshufd             xm14, xm6, q2222
    pshufd              xm6, xm6, q3333
    REPX    {pshufb x, m10}, m0, m1, m2
    pshufb              xm3, xm10
    REPX   {pmaddwd x, m15}, m0, m1, m2
    pmaddwd             xm3, xm15
    phaddd               m0, m1
    phaddd               m2, m3
    paddd                m0, m12
    paddd                m2, m12
    psrad                m0, xm7
    psrad                m2, xm7
    packssdw             m0, m2
    vextracti128        xm1, m0, 1
    palignr             xm2, xm1, xm0, 4
    pshufd              xm4, xm1, q2121
    punpcklwd           xm3, xm0, xm2       ; 01 12
    punpckhwd           xm0, xm2            ; 23 34
    punpcklwd           xm2, xm1, xm4       ; 45 56
.dy1_w2_loop:
    movu                xm1, [srcq+ssq*0]
    movu                xm5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              xm1, xm10
    pshufb              xm5, xm10
    pmaddwd             xm1, xm15
    pmaddwd             xm5, xm15
    phaddd              xm1, xm5
    pmaddwd             xm5, xm3, xm8
    mova                xm3, xm0
    pmaddwd             xm0, xm9
    paddd               xm1, xm12
    psrad               xm1, xm7
    packssdw            xm1, xm1
    paddd               xm5, xm0
    mova                xm0, xm2
    pmaddwd             xm2, xm14
    paddd               xm5, xm2
    palignr             xm2, xm1, xm4, 12
    punpcklwd           xm2, xm1            ; 67 78
    pmaddwd             xm4, xm2, xm6
    paddd               xm5, xm13
    paddd               xm5, xm4
    mova                xm4, xm1
    psrldq              xm1, xm7, 8
    psrad               xm5, xm1
    packusdw            xm5, xm5
    pminsw              xm5, xm11
    movd       [dstq+dsq*0], xm5
    pextrd     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy1_w2_loop
    RET
%endif
.dy1_w4:
    mov                 myd, mym
%if isput
    mova         [rsp+0x50], xm11
%endif
    mova         [rsp+0x00], m12
    mova         [rsp+0x20], m13
    mova         [rsp+0x40], xm7
    vbroadcasti128       m7, [base+rescale_mul]
    movzx               t0d, t0b
    sub                srcq, 2
    movd               xm15, t0d
    pmaddwd              m8, m7
    vpbroadcastq         m2, [base+pq_0x40000000+1]
    vpbroadcastd       xm15, xm15
    SWAP                m13, m10
    paddd               m13, m8 ; mx+dx*[0-3]
    pand                 m6, m13
    psrld                m6, 6
    paddd              xm15, xm6
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    pextrd             r11d, xm15, 2
    pextrd             r13d, xm15, 3
    vbroadcasti128       m5, [base+bdct_lb_q+ 0]
    vbroadcasti128       m1, [base+bdct_lb_q+16]
    vbroadcasti128       m4, [base+subpel_s_shuf2]
    vpbroadcastd       xm14, [base+subpel_filters+r4*8+2]
    vpbroadcastd        xm7, [base+subpel_filters+r6*8+2]
    vpbroadcastd       xm15, [base+subpel_filters+r11*8+2]
    vpbroadcastd        xm8, [base+subpel_filters+r13*8+2]
    pcmpeqd              m6, m9
    punpckldq           m10, m6, m6
    punpckhdq            m6, m6
    psrld               m13, 10
    paddd               m13, m13
    vpblendd           xm14, xm7, 0xa
    vpblendd           xm15, xm8, 0xa
    pmovsxbw            m14, xm14
    pmovsxbw            m15, xm15
    pblendvb            m14, m2, m10
    pblendvb            m15, m2, m6
    pextrd               r4, xm13, 2
    pshufb              m12, m13, m5
    pshufb              m13, m1
    lea                  r6, [r4+ssq*2]
    lea                 r11, [r4+ssq*1]
    lea                 r13, [r4+ss3q ]
    movu                xm0, [srcq+ssq*0]
    movu                xm7, [srcq+r4   ]
    movu                xm1, [srcq+ssq*2]
    movu                xm8, [srcq+r6   ]
    vinserti128          m0, [srcq+ssq*1], 1 ; 0 1
    vinserti128          m7, [srcq+r11  ], 1
    vinserti128          m1, [srcq+ss3q ], 1 ; 2 3
    vinserti128          m8, [srcq+r13  ], 1
    lea                srcq, [srcq+ssq*4]
    movu                xm2, [srcq+ssq*0]
    movu                xm9, [srcq+r4   ]
    movu                xm3, [srcq+ssq*2]    ; 6 _
    movu               xm10, [srcq+r6   ]
    vinserti128          m2, [srcq+ssq*1], 1 ; 4 5
    vinserti128          m9, [srcq+r11  ], 1
    lea                srcq, [srcq+ss3q ]
    vpbroadcastb         m5, xm13
    psubb               m13, m5
    paddb               m12, m4
    paddb               m13, m4
    mova                 m5, [rsp+0x00]
    movd                xm6, [rsp+0x40]
    pshufb               m0, m12
    pshufb               m1, m12
    pmaddwd              m0, m14
    pmaddwd              m1, m14
    pshufb               m7, m13
    pshufb               m8, m13
    pmaddwd              m7, m15
    pmaddwd              m8, m15
    pshufb               m2, m12
    pshufb              xm3, xm12
    pmaddwd              m2, m14
    pmaddwd             xm3, xm14
    pshufb               m9, m13
    pshufb             xm10, xm13
    pmaddwd              m9, m15
    pmaddwd            xm10, xm15
    phaddd               m0, m7
    phaddd               m1, m8
    phaddd               m2, m9
    phaddd              xm3, xm10
    paddd                m0, m5
    paddd                m1, m5
    paddd                m2, m5
    paddd               xm3, xm5
    psrad                m0, xm6
    psrad                m1, xm6
    psrad                m2, xm6
    psrad               xm3, xm6
    vperm2i128           m4, m0, m1, 0x21 ; 1 2
    vperm2i128           m5, m1, m2, 0x21 ; 3 4
    vperm2i128           m6, m2, m3, 0x21 ; 5 6
    shr                 myd, 6
    mov                r13d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz             r13q, [base+subpel_filters+myq*8]
    pslld                m4, 16
    pslld                m5, 16
    pslld                m6, 16
    pblendw              m0, m4, 0xaa ; 01 12
    pblendw              m1, m5, 0xaa ; 23 34
    pblendw              m2, m6, 0xaa ; 45 56
    movq               xm10, r13q
    punpcklqdq         xm10, xm10
    pmovsxbw            m10, xm10
    pshufd               m7, m10, q0000
    pshufd               m8, m10, q1111
    pshufd               m9, m10, q2222
    pshufd              m10, m10, q3333
.dy1_w4_loop:
    movu               xm11, [srcq+ssq*0]
    movu                xm6, [srcq+r4   ]
    vinserti128         m11, [srcq+ssq*1], 1
    vinserti128          m6, [srcq+r11  ], 1
    lea                srcq, [srcq+ssq*2]
    pmaddwd              m4, m0, m7
    pmaddwd              m5, m1, m8
    pshufb              m11, m12
    pshufb               m6, m13
    pmaddwd             m11, m14
    pmaddwd              m6, m15
    paddd                m4, [rsp+0x20]
    phaddd              m11, m6
    pmaddwd              m6, m2, m9
    paddd               m11, [rsp+0x00]
    psrad               m11, [rsp+0x40]
    mova                 m0, m1
    mova                 m1, m2
    paddd                m5, m6
    paddd                m4, m5
    vinserti128          m2, m3, xm11, 1
    pslld                m3, m11, 16
    pblendw              m2, m3, 0xaa   ; 67 78
    pmaddwd              m5, m2, m10
    vextracti128        xm3, m11, 1
    paddd                m4, m5
%if isput
    psrad                m4, [rsp+0x48]
    vextracti128        xm5, m4, 1
    packusdw            xm4, xm5
    pminsw              xm4, [rsp+0x50]
    movq       [dstq+dsq*0], xm4
    movhps     [dstq+dsq*1], xm4
    lea                dstq, [dstq+dsq*2]
%else
    psrad                m4, 6
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
    mova             [tmpq], xm4
    add                tmpq, 16
%endif
    sub                  hd, 2
    jg .dy1_w4_loop
    MC_8TAP_SCALED_RET
    SWAP                 m10, m13
.dy1_w8:
    mov    dword [rsp+0xa0], 1
    movifprep   tmp_stridem, 16
    jmp .dy1_w_start
.dy1_w16:
    mov    dword [rsp+0xa0], 2
    movifprep   tmp_stridem, 32
    jmp .dy1_w_start
.dy1_w32:
    mov    dword [rsp+0xa0], 4
    movifprep   tmp_stridem, 64
    jmp .dy1_w_start
.dy1_w64:
    mov    dword [rsp+0xa0], 8
    movifprep   tmp_stridem, 128
    jmp .dy1_w_start
.dy1_w128:
    mov    dword [rsp+0xa0], 16
    movifprep   tmp_stridem, 256
.dy1_w_start:
    SWAP                m10, m12, m1
    SWAP                m11, m7
    ; m1=mx, m7=pxmax, m10=h_rnd, m11=h_sh, m12=free
    mov                 myd, mym
%if isput
 %define dsm [rsp+0xb8]
    movifnidn           dsm, dsq
    mova         [rsp+0xc0], xm7
%else
 %if UNIX64
  %define hm [rsp+0xb8]
 %endif
%endif
    mova         [rsp+0x00], m10
    mova         [rsp+0x20], m13
    mova         [rsp+0x40], xm11
    shr                 t0d, 16
    sub                srcq, 6
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pmaddwd              m8, [base+rescale_mul2]
    movd               xm15, t0d
    mov          [rsp+0xa4], t0d
    mov          [rsp+0xa8], srcq
    mov          [rsp+0xb0], r0q ; dstq / tmpq
%if UNIX64
    mov                  hm, hd
%endif
    shl           dword dxm, 3 ; dx*8
    vpbroadcastd        m15, xm15
    paddd                m1, m8 ; mx+dx*[0-7]
    movq                xm0, r4q
    pmovsxbw            xm0, xm0
    mova         [rsp+0x50], xm0
    jmp .dy1_hloop
.dy1_hloop_prep:
    dec    dword [rsp+0xa0]
    jz .ret
    add    qword [rsp+0xb0], 16
    mov                  hd, hm
    vpbroadcastd         m8, dxm
    vpbroadcastd         m6, [base+pd_0x3ff]
    paddd                m1, m8, [rsp+0x60]
    vpbroadcastd        m15, [rsp+0xa4]
    pxor                 m9, m9
    mov                srcq, [rsp+0xa8]
    mov                 r0q, [rsp+0xb0] ; dstq / tmpq
    mova                m10, [rsp+0x00]
    mova               xm11, [rsp+0x40]
.dy1_hloop:
    vpbroadcastq        xm2, [base+pq_0x40000000]
    pand                 m5, m1, m6
    psrld                m5, 6
    paddd               m15, m5
    pcmpeqd              m5, m9
    vextracti128        xm7, m15, 1
    movq                 r6, xm15
    pextrq               r9, xm15, 1
    movq                r11, xm7
    pextrq               rX, xm7, 1
    mov                 r4d, r6d
    shr                  r6, 32
    mov                 r7d, r9d
    shr                  r9, 32
    mov                r10d, r11d
    shr                 r11, 32
    mov                r13d, rXd
    shr                  rX, 32
    mova         [rsp+0x60], m1
    movq               xm12, [base+subpel_filters+ r4*8]
    movq               xm13, [base+subpel_filters+ r6*8]
    movhps             xm12, [base+subpel_filters+ r7*8]
    movhps             xm13, [base+subpel_filters+ r9*8]
    movq               xm14, [base+subpel_filters+r10*8]
    movq               xm15, [base+subpel_filters+r11*8]
    movhps             xm14, [base+subpel_filters+r13*8]
    movhps             xm15, [base+subpel_filters+ rX*8]
    psrld                m1, 10
    vextracti128        xm7, m1, 1
    vextracti128        xm6, m5, 1
    movq                 r6, xm1
    pextrq              r11, xm1, 1
    movq                 r9, xm7
    pextrq               rX, xm7, 1
    mov                 r4d, r6d
    shr                  r6, 32
    mov                r10d, r11d
    shr                 r11, 32
    mov                 r7d, r9d
    shr                  r9, 32
    mov                r13d, rXd
    shr                  rX, 32
    pshufd              xm4, xm5, q2200
    pshufd              xm5, xm5, q3311
    pshufd              xm7, xm6, q2200
    pshufd              xm6, xm6, q3311
    pblendvb           xm12, xm2, xm4
    pblendvb           xm13, xm2, xm5
    pblendvb           xm14, xm2, xm7
    pblendvb           xm15, xm2, xm6
    pmovsxbw            m12, xm12
    pmovsxbw            m13, xm13
    pmovsxbw            m14, xm14
    pmovsxbw            m15, xm15
    MC_8TAP_SCALED_H 0, 1, 2, 3, 4, 5, 6, 7 ; 0a 1a 0b 1b
    mova         [rsp+0x80], m0
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 8 ; 2a 3a 2b 3b
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 7, 8, 9 ; 4a 5a 4b 5b
    MC_8TAP_SCALED_H 3, 4, 5, 6, 7, 8, 9, 0 ; 6a 7a 6b 7b
    mova                 m0, [rsp+0x80]
    vbroadcasti128       m7, [base+subpel_s_shuf8]
    vpbroadcastd         m8, [rsp+0x50]
    vpbroadcastd         m9, [rsp+0x54]
    vpbroadcastd        m10, [rsp+0x58]
    vpbroadcastd        m11, [rsp+0x5c]
    pshufb               m0, m7     ; 01a 01b
    pshufb               m1, m7     ; 23a 23b
    pshufb               m2, m7     ; 45a 45b
    pshufb               m3, m7     ; 67a 67b
.dy1_vloop:
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m9
    pmaddwd              m6, m2, m10
    pmaddwd              m7, m3, m11
    paddd                m4, [rsp+0x20]
    paddd                m6, m7
    paddd                m4, m5
    paddd                m4, m6
%if isput
    psrad                m4, [rsp+0x48]
    vextracti128        xm5, m4, 1
    packusdw            xm4, xm5
    pminsw              xm4, [rsp+0xc0]
    mova             [dstq], xm4
    add                dstq, dsm
%else
    psrad                m4, 6
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
    mova             [tmpq], xm4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .dy1_hloop_prep
    vbroadcasti128       m7, [base+wswap]
    pshufb               m0, m7
    pshufb               m1, m7
    pshufb               m2, m7
    pshufb               m3, m7
    movu                xm4, [srcq+ r4*2]
    movu                xm5, [srcq+ r6*2]
    movu                xm6, [srcq+ r7*2]
    movu                xm7, [srcq+ r9*2]
    vinserti128          m4, [srcq+r10*2], 1
    vinserti128          m5, [srcq+r11*2], 1
    vinserti128          m6, [srcq+r13*2], 1
    vinserti128          m7, [srcq+ rX*2], 1
    add                srcq, ssq
    pmaddwd              m4, m12
    pmaddwd              m5, m13
    pmaddwd              m6, m14
    pmaddwd              m7, m15
    phaddd               m4, m5
    phaddd               m6, m7
    phaddd               m4, m6
    paddd                m4, [rsp+0x00]
    psrad                m4, [rsp+0x40]
    pslld                m4, 16
    pblendw              m0, m1, 0xaa
    pblendw              m1, m2, 0xaa
    pblendw              m2, m3, 0xaa
    pblendw              m3, m4, 0xaa
    jmp .dy1_vloop
    SWAP                 m1, m12, m10
    SWAP                 m7, m11
.dy2:
    movzx                wd, word [base+%1_8tap_scaled_avx2_dy2_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%if isput
.dy2_w2:
    mov                 myd, mym
    movzx               t0d, t0b
    sub                srcq, 2
    movd               xm15, t0d
    punpckldq            m8, m9, m8
    paddd               m10, m8 ; mx+dx*[0-1]
    vpbroadcastd       xm14, [base+pq_0x40000000+2]
    vpbroadcastd       xm15, xm15
    pand                xm8, xm10, xm6
    psrld               xm8, 6
    paddd              xm15, xm8
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    vbroadcasti128       m5, [base+bdct_lb_q]
    vbroadcasti128       m6, [base+subpel_s_shuf2]
    vpbroadcastd       xm15, [base+subpel_filters+r4*8+2]
    vpbroadcastd        xm4, [base+subpel_filters+r6*8+2]
    pcmpeqd             xm8, xm9
    psrld               m10, 10
    paddd               m10, m10
    movu                xm0, [srcq+ssq*0]
    movu                xm1, [srcq+ssq*2]
    movu                xm2, [srcq+ssq*4]
    pshufb              m10, m5
    paddb               m10, m6
    vpblendd           xm15, xm4, 0xa
    pblendvb           xm15, xm14, xm8
    pmovsxbw            m15, xm15
    vinserti128          m0, [srcq+ssq*1], 1 ; 0 1
    vinserti128          m1, [srcq+ss3q ], 1 ; 2 3
    lea                srcq, [srcq+ssq*4]
    vinserti128          m2, [srcq+ssq*1], 1 ; 4 5
    lea                srcq, [srcq+ssq*2]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pshufb               m0, m10
    pshufb               m1, m10
    pshufb               m2, m10
    pmaddwd              m0, m15
    pmaddwd              m1, m15
    pmaddwd              m2, m15
    movq                xm6, r4q
    pmovsxbw            xm6, xm6
    phaddd               m0, m1
    phaddd               m1, m2
    paddd                m0, m12
    paddd                m1, m12
    psrad                m0, xm7
    psrad                m1, xm7
    packssdw             m0, m1             ; 0 2 2 4  1 3 3 5
    vextracti128        xm1, m0, 1
    pshufd              xm8, xm6, q0000
    pshufd              xm9, xm6, q1111
    pshufd             xm14, xm6, q2222
    pshufd              xm6, xm6, q3333
    punpcklwd           xm2, xm0, xm1       ; 01 23
    punpckhwd           xm1, xm0, xm1       ; 23 45
.dy2_w2_loop:
    movu                xm3, [srcq+ssq*0]
    movu                xm5, [srcq+ssq*2]
    vinserti128          m3, [srcq+ssq*1], 1 ; 6 7
    vinserti128          m5, [srcq+ss3q ], 1 ; 8 9
    lea                srcq, [srcq+ssq*4]
    pmaddwd             xm4, xm2, xm8
    pmaddwd             xm1, xm9
    pshufb               m3, m10
    pshufb               m5, m10
    pmaddwd              m3, m15
    pmaddwd              m5, m15
    phaddd               m3, m5
    paddd               xm4, xm1
    paddd                m3, m12
    psrad                m3, xm7
    packssdw             m3, m3
    pshufd               m3, m3, q2100
    palignr              m0, m3, m0, 12     ; 4 6 6 8  5 7 7 9
    vextracti128        xm1, m0, 1
    punpcklwd           xm2, xm0, xm1       ; 45 67
    punpckhwd           xm1, xm0, xm1       ; 67 89
    pmaddwd             xm3, xm2, xm14
    pmaddwd             xm5, xm1, xm6
    paddd               xm4, xm13
    paddd               xm4, xm3
    psrldq              xm3, xm7, 8
    paddd               xm4, xm5
    psrad               xm4, xm3
    packusdw            xm4, xm4
    pminsw              xm4, xm11
    movd       [dstq+dsq*0], xm4
    pextrd     [dstq+dsq*1], xm4, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy2_w2_loop
    RET
%endif
.dy2_w4:
    mov                 myd, mym
%if isput
    mova         [rsp+0x50], xm11
%endif
    mova         [rsp+0x00], m12
    mova         [rsp+0x20], m13
    mova         [rsp+0x40], xm7
    vbroadcasti128       m7, [base+rescale_mul]
    movzx               t0d, t0b
    sub                srcq, 2
    movd               xm15, t0d
    pmaddwd              m8, m7
    vpbroadcastq         m2, [base+pq_0x40000000+1]
    vpbroadcastd       xm15, xm15
    SWAP                m13, m10
    paddd               m13, m8 ; mx+dx*[0-3]
    pand                 m6, m13
    psrld                m6, 6
    paddd              xm15, xm6
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    pextrd             r11d, xm15, 2
    pextrd             r13d, xm15, 3
    vbroadcasti128       m5, [base+bdct_lb_q+ 0]
    vbroadcasti128       m1, [base+bdct_lb_q+16]
    vbroadcasti128       m4, [base+subpel_s_shuf2]
    vpbroadcastd       xm14, [base+subpel_filters+r4*8+2]
    vpbroadcastd        xm7, [base+subpel_filters+r6*8+2]
    vpbroadcastd       xm15, [base+subpel_filters+r11*8+2]
    vpbroadcastd        xm8, [base+subpel_filters+r13*8+2]
    shr                 myd, 6
    mov                r13d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz             r13q, [base+subpel_filters+myq*8]
    pcmpeqd              m6, m9
    punpckldq           m11, m6, m6
    punpckhdq            m6, m6
    psrld               m13, 10
    paddd               m13, m13
    vpblendd           xm14, xm7, 0xa
    vpblendd           xm15, xm8, 0xa
    pmovsxbw            m14, xm14
    pmovsxbw            m15, xm15
    movq               xm10, r13q
    pblendvb            m14, m2, m11
    pblendvb            m15, m2, m6
    pextrd               r4, xm13, 2
    pshufb              m12, m13, m5
    pshufb              m13, m1
    lea                  r6, [r4+ssq*1]
    lea                 r11, [r4+ssq*2]
    lea                 r13, [r4+ss3q ]
    movu                xm0, [srcq+ssq*0]
    movu                xm7, [srcq+r4   ]
    movu                xm1, [srcq+ssq*1]
    movu                xm8, [srcq+r6   ]
    vinserti128          m0, [srcq+ssq*2], 1 ; 0 2
    vinserti128          m7, [srcq+r11  ], 1
    vinserti128          m1, [srcq+ss3q ], 1 ; 1 3
    vinserti128          m8, [srcq+r13  ], 1
    lea                srcq, [srcq+ssq*4]
    movu                xm2, [srcq+ssq*0]
    movu                xm9, [srcq+r4   ]
    vinserti128          m2, [srcq+ssq*1], 1 ; 4 5
    vinserti128          m9, [srcq+r6   ], 1
    lea                srcq, [srcq+ssq*2]
    vpbroadcastb         m5, xm13
    psubb               m13, m5
    paddb               m12, m4
    paddb               m13, m4
    mova                 m5, [rsp+0x00]
    movd                xm6, [rsp+0x40]
    pshufb               m0, m12
    pshufb               m1, m12
    pshufb               m2, m12
    pmaddwd              m0, m14
    pmaddwd              m1, m14
    pmaddwd              m2, m14
    pshufb               m7, m13
    pshufb               m8, m13
    pshufb               m9, m13
    pmaddwd              m7, m15
    pmaddwd              m8, m15
    pmaddwd              m9, m15
    punpcklqdq         xm10, xm10
    pmovsxbw            m10, xm10
    phaddd               m0, m7
    phaddd               m1, m8
    phaddd               m2, m9
    paddd                m0, m5
    paddd                m1, m5
    paddd                m2, m5
    psrad                m0, xm6
    psrad                m1, xm6
    psrad                m2, xm6
    vperm2i128           m3, m0, m2, 0x21 ; 2 4
    vperm2i128           m2, m1, 0x13     ; 3 5
    pshufd               m7, m10, q0000
    pshufd               m8, m10, q1111
    pshufd               m9, m10, q2222
    pshufd              m10, m10, q3333
    packssdw             m0, m3 ; 0 2  2 4
    packssdw             m1, m2 ; 1 3  3 5
    punpckhwd            m2, m0, m1 ; 23 45
    punpcklwd            m0, m1     ; 01 23
.dy2_w4_loop:
    movu                xm1, [srcq+ssq*0]
    movu                xm6, [srcq+r4   ]
    movu                xm3, [srcq+ssq*1]
    movu               xm11, [srcq+r6   ]
    vinserti128          m1, [srcq+ssq*2], 1 ; 6 8
    vinserti128          m6, [srcq+r11  ], 1
    vinserti128          m3, [srcq+ss3q ], 1 ; 7 9
    vinserti128         m11, [srcq+r13  ], 1
    lea                srcq, [srcq+ssq*4]
    pmaddwd              m4, m0, m7
    pmaddwd              m5, m2, m8
    pshufb               m1, m12
    pshufb               m3, m12
    pmaddwd              m1, m14
    pmaddwd              m3, m14
    mova                 m0, [rsp+0x00]
    pshufb               m6, m13
    pshufb              m11, m13
    pmaddwd              m6, m15
    pmaddwd             m11, m15
    paddd                m4, m5
    movd                xm5, [rsp+0x40]
    phaddd               m1, m6
    phaddd               m3, m11
    paddd                m1, m0
    paddd                m3, m0
    psrad                m1, xm5
    psrad                m3, xm5
    pslld                m3, 16
    pblendw              m1, m3, 0xaa     ; 67 89
    vperm2i128           m0, m2, m1, 0x21 ; 45 67
    paddd                m4, [rsp+0x20]
    mova                 m2, m1
    pmaddwd              m5, m0, m9
    pmaddwd              m6, m2, m10
    paddd                m4, m5
    paddd                m4, m6
%if isput
    psrad                m4, [rsp+0x48]
    vextracti128        xm5, m4, 1
    packusdw            xm4, xm5
    pminsw              xm4, [rsp+0x50]
    movq       [dstq+dsq*0], xm4
    movhps     [dstq+dsq*1], xm4
    lea                dstq, [dstq+dsq*2]
%else
    psrad                m4, 6
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
    mova             [tmpq], xm4
    add                tmpq, 16
%endif
    sub                  hd, 2
    jg .dy2_w4_loop
    MC_8TAP_SCALED_RET
    SWAP                m10, m13
.dy2_w8:
    mov    dword [rsp+0xa0], 1
    movifprep   tmp_stridem, 16
    jmp .dy2_w_start
.dy2_w16:
    mov    dword [rsp+0xa0], 2
    movifprep   tmp_stridem, 32
    jmp .dy2_w_start
.dy2_w32:
    mov    dword [rsp+0xa0], 4
    movifprep   tmp_stridem, 64
    jmp .dy2_w_start
.dy2_w64:
    mov    dword [rsp+0xa0], 8
    movifprep   tmp_stridem, 128
    jmp .dy2_w_start
.dy2_w128:
    mov    dword [rsp+0xa0], 16
    movifprep   tmp_stridem, 256
.dy2_w_start:
    SWAP                m10, m12, m1
    SWAP                m11, m7
    ; m1=mx, m7=pxmax, m10=h_rnd, m11=h_sh, m12=free
    mov                 myd, mym
%if isput
    movifnidn           dsm, dsq
    mova         [rsp+0xc0], xm7
%endif
    mova         [rsp+0x00], m10
    mova         [rsp+0x20], m13
    mova         [rsp+0x40], xm11
    shr                 t0d, 16
    sub                srcq, 6
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pmaddwd              m8, [base+rescale_mul2]
    movd               xm15, t0d
    mov          [rsp+0xa4], t0d
    mov          [rsp+0xa8], srcq
    mov          [rsp+0xb0], r0q ; dstq / tmpq
%if UNIX64
    mov                  hm, hd
%endif
    shl           dword dxm, 3 ; dx*8
    vpbroadcastd        m15, xm15
    paddd                m1, m8 ; mx+dx*[0-7]
    movq                xm0, r4q
    pmovsxbw            xm0, xm0
    mova         [rsp+0x50], xm0
    jmp .dy2_hloop
.dy2_hloop_prep:
    dec    dword [rsp+0xa0]
    jz .ret
    add    qword [rsp+0xb0], 16
    mov                  hd, hm
    vpbroadcastd         m8, dxm
    vpbroadcastd         m6, [base+pd_0x3ff]
    paddd                m1, m8, [rsp+0x60]
    vpbroadcastd        m15, [rsp+0xa4]
    pxor                 m9, m9
    mov                srcq, [rsp+0xa8]
    mov                 r0q, [rsp+0xb0] ; dstq / tmpq
    mova                m10, [rsp+0x00]
    mova               xm11, [rsp+0x40]
.dy2_hloop:
    vpbroadcastq        xm2, [base+pq_0x40000000]
    pand                 m5, m1, m6
    psrld                m5, 6
    paddd               m15, m5
    pcmpeqd              m5, m9
    vextracti128        xm7, m15, 1
    movq                 r6, xm15
    pextrq               r9, xm15, 1
    movq                r11, xm7
    pextrq               rX, xm7, 1
    mov                 r4d, r6d
    shr                  r6, 32
    mov                 r7d, r9d
    shr                  r9, 32
    mov                r10d, r11d
    shr                 r11, 32
    mov                r13d, rXd
    shr                  rX, 32
    mova         [rsp+0x60], m1
    movq               xm12, [base+subpel_filters+ r4*8]
    movq               xm13, [base+subpel_filters+ r6*8]
    movhps             xm12, [base+subpel_filters+ r7*8]
    movhps             xm13, [base+subpel_filters+ r9*8]
    movq               xm14, [base+subpel_filters+r10*8]
    movq               xm15, [base+subpel_filters+r11*8]
    movhps             xm14, [base+subpel_filters+r13*8]
    movhps             xm15, [base+subpel_filters+ rX*8]
    psrld                m1, 10
    vextracti128        xm7, m1, 1
    vextracti128        xm6, m5, 1
    movq                 r6, xm1
    pextrq              r11, xm1, 1
    movq                 r9, xm7
    pextrq               rX, xm7, 1
    mov                 r4d, r6d
    shr                  r6, 32
    mov                r10d, r11d
    shr                 r11, 32
    mov                 r7d, r9d
    shr                  r9, 32
    mov                r13d, rXd
    shr                  rX, 32
    pshufd              xm4, xm5, q2200
    pshufd              xm5, xm5, q3311
    pshufd              xm7, xm6, q2200
    pshufd              xm6, xm6, q3311
    pblendvb           xm12, xm2, xm4
    pblendvb           xm13, xm2, xm5
    pblendvb           xm14, xm2, xm7
    pblendvb           xm15, xm2, xm6
    pmovsxbw            m12, xm12
    pmovsxbw            m13, xm13
    pmovsxbw            m14, xm14
    pmovsxbw            m15, xm15
    MC_8TAP_SCALED_H 0, 1, 2, 3, 4, 5, 6, 7 ; 0a 1a 0b 1b
    mova         [rsp+0x80], m0
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 8 ; 2a 3a 2b 3b
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 7, 8, 9 ; 4a 5a 4b 5b
    MC_8TAP_SCALED_H 3, 4, 5, 6, 7, 8, 9, 0 ; 6a 7a 6b 7b
    mova                 m0, [rsp+0x80]
    vbroadcasti128       m7, [base+subpel_s_shuf8]
    vpbroadcastd         m8, [rsp+0x50]
    vpbroadcastd         m9, [rsp+0x54]
    vpbroadcastd        m10, [rsp+0x58]
    vpbroadcastd        m11, [rsp+0x5c]
    pshufb               m0, m7     ; 01a 01b
    pshufb               m1, m7     ; 23a 23b
    pshufb               m2, m7     ; 45a 45b
    pshufb               m3, m7     ; 67a 67b
.dy2_vloop:
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m9
    pmaddwd              m6, m2, m10
    pmaddwd              m7, m3, m11
    paddd                m4, [rsp+0x20]
    paddd                m6, m7
    paddd                m4, m5
    paddd                m4, m6
%if isput
    psrad                m4, [rsp+0x48]
    vextracti128        xm5, m4, 1
    packusdw            xm4, xm5
    pminsw              xm4, [rsp+0xc0]
    mova             [dstq], xm4
    add                dstq, dsm
%else
    psrad                m4, 6
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
    mova             [tmpq], xm4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .dy2_hloop_prep
    mova                 m0, m1
    mova                 m1, m2
    mova                 m2, m3
    movu                xm3, [srcq+ r4*2]
    movu                xm4, [srcq+ r6*2]
    movu                xm5, [srcq+ r7*2]
    movu                xm6, [srcq+ r9*2]
    vinserti128          m3, [srcq+r10*2], 1
    vinserti128          m4, [srcq+r11*2], 1
    vinserti128          m5, [srcq+r13*2], 1
    vinserti128          m6, [srcq+ rX*2], 1
    add                srcq, ssq
    pmaddwd              m3, m12
    pmaddwd              m4, m13
    pmaddwd              m5, m14
    pmaddwd              m6, m15
    phaddd               m3, m4
    phaddd               m5, m6
    phaddd               m3, m5
    movu                xm4, [srcq+ r4*2]
    movu                xm5, [srcq+ r6*2]
    movu                xm6, [srcq+ r7*2]
    movu                xm7, [srcq+ r9*2]
    vinserti128          m4, [srcq+r10*2], 1
    vinserti128          m5, [srcq+r11*2], 1
    vinserti128          m6, [srcq+r13*2], 1
    vinserti128          m7, [srcq+ rX*2], 1
    add                srcq, ssq
    pmaddwd              m4, m12
    pmaddwd              m5, m13
    pmaddwd              m6, m14
    pmaddwd              m7, m15
    phaddd               m4, m5
    phaddd               m6, m7
    mova                 m5, [rsp+0x00]
    movd                xm7, [rsp+0x40]
    phaddd               m4, m6
    paddd                m3, m5
    paddd                m4, m5
    psrad                m3, xm7
    psrad                m4, xm7
    pslld                m4, 16
    pblendw              m3, m4, 0xaa
    jmp .dy2_vloop
.ret:
    MC_8TAP_SCALED_RET 0
%undef isput
%undef isprep
%endmacro

%macro BILIN_SCALED_FN 1
cglobal %1_bilin_scaled_16bpc
    mov                 t0d, (5*15 << 16) | 5*15
    mov                 t1d, t0d
    jmp mangle(private_prefix %+ _%1_8tap_scaled_16bpc %+ SUFFIX)
%endmacro

%if WIN64
DECLARE_REG_TMP 6, 5
%else
DECLARE_REG_TMP 6, 8
%endif

%define PUT_8TAP_SCALED_FN FN put_8tap_scaled,
BILIN_SCALED_FN put
PUT_8TAP_SCALED_FN sharp,          SHARP,   SHARP
PUT_8TAP_SCALED_FN sharp_smooth,   SHARP,   SMOOTH
PUT_8TAP_SCALED_FN smooth_sharp,   SMOOTH,  SHARP
PUT_8TAP_SCALED_FN smooth,         SMOOTH,  SMOOTH
PUT_8TAP_SCALED_FN sharp_regular,  SHARP,   REGULAR
PUT_8TAP_SCALED_FN regular_sharp,  REGULAR, SHARP
PUT_8TAP_SCALED_FN smooth_regular, SMOOTH,  REGULAR
PUT_8TAP_SCALED_FN regular_smooth, REGULAR, SMOOTH
PUT_8TAP_SCALED_FN regular,        REGULAR, REGULAR
MC_8TAP_SCALED put

%if WIN64
DECLARE_REG_TMP 5, 4
%else
DECLARE_REG_TMP 6, 7
%endif

%define PREP_8TAP_SCALED_FN FN prep_8tap_scaled,
BILIN_SCALED_FN prep
PREP_8TAP_SCALED_FN sharp,          SHARP,   SHARP
PREP_8TAP_SCALED_FN sharp_smooth,   SHARP,   SMOOTH
PREP_8TAP_SCALED_FN smooth_sharp,   SMOOTH,  SHARP
PREP_8TAP_SCALED_FN smooth,         SMOOTH,  SMOOTH
PREP_8TAP_SCALED_FN sharp_regular,  SHARP,   REGULAR
PREP_8TAP_SCALED_FN regular_sharp,  REGULAR, SHARP
PREP_8TAP_SCALED_FN smooth_regular, SMOOTH,  REGULAR
PREP_8TAP_SCALED_FN regular_smooth, REGULAR, SMOOTH
PREP_8TAP_SCALED_FN regular,        REGULAR, REGULAR
MC_8TAP_SCALED prep

%macro WARP_V 5 ; dst, 01, 23, 45, 67
    lea               tmp1d, [myq+deltaq*4]
    lea               tmp2d, [myq+deltaq*1]
    shr                 myd, 10
    shr               tmp1d, 10
    movq                xm8, [filterq+myq  *8]
    vinserti128          m8, [filterq+tmp1q*8], 1 ; a e
    lea               tmp1d, [tmp2q+deltaq*4]
    lea                 myd, [tmp2q+deltaq*1]
    shr               tmp2d, 10
    shr               tmp1d, 10
    movq                xm0, [filterq+tmp2q*8]
    vinserti128          m0, [filterq+tmp1q*8], 1 ; b f
    lea               tmp1d, [myq+deltaq*4]
    lea               tmp2d, [myq+deltaq*1]
    shr                 myd, 10
    shr               tmp1d, 10
    movq                xm9, [filterq+myq  *8]
    vinserti128          m9, [filterq+tmp1q*8], 1 ; c g
    lea               tmp1d, [tmp2q+deltaq*4]
    lea                 myd, [tmp2q+gammaq]       ; my += gamma
    punpcklwd            m8, m0
    shr               tmp2d, 10
    shr               tmp1d, 10
    movq                xm0, [filterq+tmp2q*8]
    vinserti128          m0, [filterq+tmp1q*8], 1 ; d h
    punpcklwd            m0, m9, m0
    punpckldq            m9, m8, m0
    punpckhdq            m0, m8, m0
    punpcklbw            m8, m11, m9 ; a0 a1 b0 b1 c0 c1 d0 d1 << 8
    punpckhbw            m9, m11, m9 ; a2 a3 b2 b3 c2 c3 d2 d3 << 8
    pmaddwd             m%2, m8
    pmaddwd              m9, m%3
    punpcklbw            m8, m11, m0 ; a4 a5 b4 b5 c4 c5 d4 d5 << 8
    punpckhbw            m0, m11, m0 ; a6 a7 b6 b7 c6 c7 d6 d7 << 8
    pmaddwd              m8, m%4
    pmaddwd              m0, m%5
    paddd                m9, m%2
    mova                m%2, m%3
    paddd                m0, m8
    mova                m%3, m%4
    mova                m%4, m%5
    paddd               m%1, m0, m9
%endmacro

cglobal warp_affine_8x8t_16bpc, 4, 14, 16, tmp, ts
    mov                 r6d, r7m
    lea                  r9, [$$]
    shr                 r6d, 11
    vpbroadcastd        m13, [r9-$$+warp8x8_shift+r6*4]
    vpbroadcastd        m14, [warp8x8t_rnd]
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_avx2).main
    jmp .start
.loop:
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_avx2).main2
    lea                tmpq, [tmpq+tsq*4]
.start:
    paddd                m7, m14
    paddd                m0, m14
    psrad                m7, 15
    psrad                m0, 15
    packssdw             m7, m0
    vpermq               m7, m7, q3120
    mova         [tmpq+tsq*0], xm7
    vextracti128 [tmpq+tsq*2], m7, 1
    dec                 r4d
    jg .loop
.end:
    RET

cglobal warp_affine_8x8_16bpc, 4, 14, 16, dst, ds, src, ss, abcd, mx, tmp2, \
                                          alpha, beta, filter, tmp1, delta, \
                                          my, gamma
    mov                 r6d, r7m
    lea             filterq, [$$]
    shr                 r6d, 11
    vpbroadcastd        m13, [filterq-$$+warp8x8_shift+r6*4]
    vpbroadcastd        m14, [filterq-$$+warp8x8_rnd  +r6*4]
    vpbroadcastw        m15, r7m ; pixel_max
    call .main
    jmp .start
.loop:
    call .main2
    lea                dstq, [dstq+dsq*2]
.start:
    psrad                m7, 16
    psrad                m0, 16
    packusdw             m7, m0
    pmulhrsw             m7, m14
    pminsw               m7, m15
    vpermq               m7, m7, q3120
    mova         [dstq+dsq*0], xm7
    vextracti128 [dstq+dsq*1], m7, 1
    dec                 r4d
    jg .loop
.end:
    RET
ALIGN function_align
.main:
    ; Stack args offset by one (r4m -> r5m etc.) due to call
%if WIN64
    mov               abcdq, r5m
    mov                 mxd, r6m
%endif
    movsx            alphad, word [abcdq+2*0]
    movsx             betad, word [abcdq+2*1]
    vpbroadcastd        m12, [pd_32768]
    pxor                m11, m11
    add             filterq, mc_warp_filter-$$
    lea               tmp1q, [ssq*3]
    add                 mxd, 512+(64<<10)
    lea               tmp2d, [alphaq*3]
    sub                srcq, tmp1q    ; src -= src_stride*3
    sub               betad, tmp2d    ; beta -= alpha*3
    mov                 myd, r7m
    call .h
    psrld                m1, m0, 16
    call .h
    pblendw              m1, m0, 0xaa ; 01
    psrld                m2, m0, 16
    call .h
    pblendw              m2, m0, 0xaa ; 12
    psrld                m3, m0, 16
    call .h
    pblendw              m3, m0, 0xaa ; 23
    psrld                m4, m0, 16
    call .h
    pblendw              m4, m0, 0xaa ; 34
    psrld                m5, m0, 16
    call .h
    pblendw              m5, m0, 0xaa ; 45
    psrld                m6, m0, 16
    call .h
    pblendw              m6, m0, 0xaa ; 56
    movsx            deltad, word [abcdq+2*2]
    movsx            gammad, word [abcdq+2*3]
    add                 myd, 512+(64<<10)
    mov                 r4d, 4
    lea               tmp1d, [deltaq*3]
    sub              gammad, tmp1d    ; gamma -= delta*3
.main2:
    call .h
    psrld                m7, m6, 16
    pblendw              m7, m0, 0xaa ; 67
    WARP_V                7, 1, 3, 5, 7
    call .h
    psrld               m10, m5, 16
    pblendw             m10, m0, 0xaa ; 78
    WARP_V                0, 2, 4, 6, 10
    ret
ALIGN function_align
.h:
    lea               tmp1d, [mxq+alphaq*4]
    lea               tmp2d, [mxq+alphaq*1]
    movu               xm10, [srcq-6]
    vinserti128         m10, [srcq+2], 1
    shr                 mxd, 10 ; 0
    shr               tmp1d, 10 ; 4
    movq                xm0, [filterq+mxq  *8]
    vinserti128          m0, [filterq+tmp1q*8], 1
    lea               tmp1d, [tmp2q+alphaq*4]
    lea                 mxd, [tmp2q+alphaq*1]
    movu                xm8, [srcq-4]
    vinserti128          m8, [srcq+4], 1
    shr               tmp2d, 10 ; 1
    shr               tmp1d, 10 ; 5
    movq                xm9, [filterq+tmp2q*8]
    vinserti128          m9, [filterq+tmp1q*8], 1
    lea               tmp1d, [mxq+alphaq*4]
    lea               tmp2d, [mxq+alphaq*1]
    shr                 mxd, 10 ; 2
    shr               tmp1d, 10 ; 6
    punpcklbw            m0, m11, m0
    pmaddwd              m0, m10
    movu               xm10, [srcq-2]
    vinserti128         m10, [srcq+6], 1
    punpcklbw            m9, m11, m9
    pmaddwd              m9, m8
    movq                xm8, [filterq+mxq  *8]
    vinserti128          m8, [filterq+tmp1q*8], 1
    lea               tmp1d, [tmp2q+alphaq*4]
    lea                 mxd, [tmp2q+betaq] ; mx += beta
    phaddd               m0, m9 ; 0 1   4 5
    movu                xm9, [srcq+0]
    vinserti128          m9, [srcq+8], 1
    shr               tmp2d, 10 ; 3
    shr               tmp1d, 10 ; 7
    punpcklbw            m8, m11, m8
    pmaddwd              m8, m10
    movq               xm10, [filterq+tmp2q*8]
    vinserti128         m10, [filterq+tmp1q*8], 1
    punpcklbw           m10, m11, m10
    pmaddwd              m9, m10
    add                srcq, ssq
    phaddd               m8, m9 ; 2 3   6 7
    phaddd               m0, m8 ; 0 1 2 3   4 5 6 7
    vpsllvd              m0, m13
    paddd                m0, m12 ; rounded 14-bit result in upper 16 bits of dword
    ret

%macro BIDIR_FN 0
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    movq   [dstq          ], xm0
    movhps [dstq+strideq*1], xm0
    vextracti128        xm0, m0, 1
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    cmp                  hd, 4
    je .ret
    lea                dstq, [dstq+strideq*4]
    movq   [dstq          ], xm1
    movhps [dstq+strideq*1], xm1
    vextracti128        xm1, m1, 1
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
    cmp                  hd, 8
    je .ret
    lea                dstq, [dstq+strideq*4]
    movq   [dstq          ], xm2
    movhps [dstq+strideq*1], xm2
    vextracti128        xm2, m2, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq          ], xm3
    movhps [dstq+strideq*1], xm3
    vextracti128        xm3, m3, 1
    movq   [dstq+strideq*2], xm3
    movhps [dstq+stride3q ], xm3
.ret:
    RET
.w8:
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+stride3q ], m1, 1
    cmp                  hd, 4
    jne .w8_loop_start
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+stride3q ], m1, 1
.w8_loop_start:
    lea                dstq, [dstq+strideq*4]
    mova         [dstq+strideq*0], xm2
    vextracti128 [dstq+strideq*1], m2, 1
    mova         [dstq+strideq*2], xm3
    vextracti128 [dstq+stride3q ], m3, 1
    sub                  hd, 8
    jg .w8_loop
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w16:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w32:
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*1+32*0], m2
    mova [dstq+strideq*1+32*1], m3
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    mova        [dstq+32*2], m2
    mova        [dstq+32*3], m3
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    mova        [dstq+32*2], m2
    mova        [dstq+32*3], m3
    call .main
    mova        [dstq+32*4], m0
    mova        [dstq+32*5], m1
    mova        [dstq+32*6], m2
    mova        [dstq+32*7], m3
    dec                  hd
    jg .w128_loop
    RET
%endmacro

%if WIN64
DECLARE_REG_TMP 5
%else
DECLARE_REG_TMP 7
%endif

cglobal avg_16bpc, 4, 7, 6, dst, stride, tmp1, tmp2, w, h, stride3
%define base r6-avg_avx2_table
    lea                  r6, [avg_avx2_table]
    tzcnt                wd, wm
    mov                 t0d, r6m ; pixel_max
    movsxd               wq, [r6+wq*4]
    shr                 t0d, 11
    vpbroadcastd         m4, [base+bidir_rnd+t0*4]
    vpbroadcastd         m5, [base+bidir_mul+t0*4]
    movifnidn            hd, hm
    add                  wq, r6
    BIDIR_FN
ALIGN function_align
.main:
    mova                 m0, [tmp1q+32*0]
    paddsw               m0, [tmp2q+32*0]
    mova                 m1, [tmp1q+32*1]
    paddsw               m1, [tmp2q+32*1]
    mova                 m2, [tmp1q+32*2]
    paddsw               m2, [tmp2q+32*2]
    mova                 m3, [tmp1q+32*3]
    paddsw               m3, [tmp2q+32*3]
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    pmaxsw               m0, m4
    pmaxsw               m1, m4
    pmaxsw               m2, m4
    pmaxsw               m3, m4
    psubsw               m0, m4
    psubsw               m1, m4
    psubsw               m2, m4
    psubsw               m3, m4
    pmulhw               m0, m5
    pmulhw               m1, m5
    pmulhw               m2, m5
    pmulhw               m3, m5
    ret

cglobal w_avg_16bpc, 4, 7, 9, dst, stride, tmp1, tmp2, w, h, stride3
    lea                  r6, [w_avg_avx2_table]
    tzcnt                wd, wm
    mov                 t0d, r6m ; weight
    vpbroadcastw         m8, r7m ; pixel_max
    vpbroadcastd         m7, [r6-w_avg_avx2_table+pd_65538]
    movsxd               wq, [r6+wq*4]
    paddw                m7, m8
    add                  wq, r6
    lea                 r6d, [t0-16]
    shl                 t0d, 16
    sub                 t0d, r6d ; 16-weight, weight
    pslld                m7, 7
    rorx                r6d, t0d, 30 ; << 2
    test          dword r7m, 0x800
    cmovz               r6d, t0d
    movifnidn            hd, hm
    movd                xm6, r6d
    vpbroadcastd         m6, xm6
    BIDIR_FN
ALIGN function_align
.main:
    mova                 m4, [tmp1q+32*0]
    mova                 m0, [tmp2q+32*0]
    punpckhwd            m5, m0, m4
    punpcklwd            m0, m4
    mova                 m4, [tmp1q+32*1]
    mova                 m1, [tmp2q+32*1]
    pmaddwd              m5, m6
    pmaddwd              m0, m6
    paddd                m5, m7
    paddd                m0, m7
    psrad                m5, 8
    psrad                m0, 8
    packusdw             m0, m5
    punpckhwd            m5, m1, m4
    punpcklwd            m1, m4
    mova                 m4, [tmp1q+32*2]
    mova                 m2, [tmp2q+32*2]
    pmaddwd              m5, m6
    pmaddwd              m1, m6
    paddd                m5, m7
    paddd                m1, m7
    psrad                m5, 8
    psrad                m1, 8
    packusdw             m1, m5
    punpckhwd            m5, m2, m4
    punpcklwd            m2, m4
    mova                 m4, [tmp1q+32*3]
    mova                 m3, [tmp2q+32*3]
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    pmaddwd              m5, m6
    pmaddwd              m2, m6
    paddd                m5, m7
    paddd                m2, m7
    psrad                m5, 8
    psrad                m2, 8
    packusdw             m2, m5
    punpckhwd            m5, m3, m4
    punpcklwd            m3, m4
    pmaddwd              m5, m6
    pmaddwd              m3, m6
    paddd                m5, m7
    paddd                m3, m7
    psrad                m5, 8
    psrad                m3, 8
    packusdw             m3, m5
    pminsw               m0, m8
    pminsw               m1, m8
    pminsw               m2, m8
    pminsw               m3, m8
    ret

cglobal mask_16bpc, 4, 8, 11, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-mask_avx2_table
    lea                  r7, [mask_avx2_table]
    tzcnt                wd, wm
    mov                 r6d, r7m ; pixel_max
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd         m8, [base+pw_64]
    vpbroadcastd         m9, [base+bidir_rnd+r6*4]
    vpbroadcastd        m10, [base+bidir_mul+r6*4]
    mov               maskq, maskmp
    add                  wq, r7
    BIDIR_FN
ALIGN function_align
.main:
%macro MASK 1
    pmovzxbw             m5, [maskq+16*%1]
    mova                m%1, [tmp1q+32*%1]
    mova                 m6, [tmp2q+32*%1]
    punpckhwd            m4, m%1, m6
    punpcklwd           m%1, m6
    psubw                m7, m8, m5
    punpckhwd            m6, m5, m7 ; m, 64-m
    punpcklwd            m5, m7
    pmaddwd              m4, m6     ; tmp1 * m + tmp2 * (64-m)
    pmaddwd             m%1, m5
    psrad                m4, 5
    psrad               m%1, 5
    packssdw            m%1, m4
    pmaxsw              m%1, m9
    psubsw              m%1, m9
    pmulhw              m%1, m10
%endmacro
    MASK                  0
    MASK                  1
    MASK                  2
    MASK                  3
    add               maskq, 16*4
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    ret

cglobal w_mask_420_16bpc, 4, 8, 16, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_420_avx2_table
    lea                  r7, [w_mask_420_avx2_table]
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    movd                xm0, r7m ; sign
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd        m10, [base+pw_27615] ; ((64 - 38) << 10) + 1023 - 32
    vpbroadcastd        m11, [base+pw_64]
    vpbroadcastd        m12, [base+bidir_rnd+r6*4]
    vpbroadcastd        m13, [base+bidir_mul+r6*4]
    movd               xm14, [base+pw_2]
    mov               maskq, maskmp
    psubw              xm14, xm0
    vpbroadcastw        m14, xm14
    add                  wq, r7
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    phaddd               m4, m5
    paddw                m4, m14
    psrlw                m4, 2
    packuswb             m4, m4
    vextracti128        xm5, m4, 1
    punpcklwd           xm4, xm5
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti128        xm0, m0, 1
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    mova            [maskq], xm4
    cmp                  hd, 8
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    vextracti128        xm1, m1, 1
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
    je .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti128        xm2, m2, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm3
    movhps [dstq+strideq*1], xm3
    vextracti128        xm3, m3, 1
    movq   [dstq+strideq*2], xm3
    movhps [dstq+stride3q ], xm3
.w4_end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
    add               maskq, 16
.w8:
    vperm2i128           m6, m4, m5, 0x21
    vpblendd             m4, m5, 0xf0
    paddw                m4, m14
    paddw                m4, m6
    psrlw                m4, 2
    vextracti128        xm5, m4, 1
    packuswb            xm4, xm5
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+stride3q ], m1, 1
    mova            [maskq], xm4
    sub                  hd, 8
    jl .w8_end
    lea                dstq, [dstq+strideq*4]
    mova         [dstq+strideq*0], xm2
    vextracti128 [dstq+strideq*1], m2, 1
    mova         [dstq+strideq*2], xm3
    vextracti128 [dstq+stride3q ], m3, 1
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
    add               maskq, 16
.w16:
    punpcklqdq           m6, m4, m5
    punpckhqdq           m4, m5
    paddw                m6, m14
    paddw                m4, m6
    psrlw                m4, 2
    vextracti128        xm5, m4, 1
    packuswb            xm4, xm5
    pshufd              xm4, xm4, q3120
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    mova            [maskq], xm4
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
    add               maskq, 32
.w32:
    paddw                m4, m14
    paddw                m4, m5
    psrlw               m15, m4, 2
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*1+32*0], m2
    mova [dstq+strideq*1+32*1], m3
    call .main
    mova                 m6, [deint_shuf]
    paddw                m4, m14
    paddw                m4, m5
    psrlw                m4, 2
    packuswb            m15, m4
    vpermd               m4, m6, m15
    mova [dstq+strideq*2+32*0], m0
    mova [dstq+strideq*2+32*1], m1
    mova [dstq+stride3q +32*0], m2
    mova [dstq+stride3q +32*1], m3
    mova            [maskq], m4
    sub                  hd, 4
    jg .w32_loop
    RET
.w64_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 32
.w64:
    paddw                m4, m14
    paddw               m15, m14, m5
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*0+32*2], m2
    mova [dstq+strideq*0+32*3], m3
    mova            [maskq], m4 ; no available registers
    call .main
    paddw                m4, [maskq]
    mova                 m6, [deint_shuf]
    paddw                m5, m15
    psrlw                m4, 2
    psrlw                m5, 2
    packuswb             m4, m5 ; 0 2 4 6   1 3 5 7
    vpermd               m4, m6, m4
    mova [dstq+strideq*1+32*0], m0
    mova [dstq+strideq*1+32*1], m1
    mova [dstq+strideq*1+32*2], m2
    mova [dstq+strideq*1+32*3], m3
    mova            [maskq], m4
    sub                  hd, 2
    jg .w64_loop
    RET
.w128_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 64
.w128:
    paddw                m4, m14
    paddw                m5, m14
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*0+32*2], m2
    mova [dstq+strideq*0+32*3], m3
    mova       [maskq+32*0], m4
    mova     [dstq+strideq], m5
    call .main
    paddw                m4, m14
    paddw               m15, m14, m5
    mova [dstq+strideq*0+32*4], m0
    mova [dstq+strideq*0+32*5], m1
    mova [dstq+strideq*0+32*6], m2
    mova [dstq+strideq*0+32*7], m3
    mova       [maskq+32*1], m4
    call .main
    paddw                m4, [maskq+32*0]
    paddw                m5, [dstq+strideq]
    mova                 m6, [deint_shuf]
    psrlw                m4, 2
    psrlw                m5, 2
    packuswb             m4, m5
    vpermd               m4, m6, m4
    mova [dstq+strideq*1+32*0], m0
    mova [dstq+strideq*1+32*1], m1
    mova [dstq+strideq*1+32*2], m2
    mova [dstq+strideq*1+32*3], m3
    mova       [maskq+32*0], m4
    call .main
    paddw                m4, [maskq+32*1]
    mova                 m6, [deint_shuf]
    paddw                m5, m15
    psrlw                m4, 2
    psrlw                m5, 2
    packuswb             m4, m5
    vpermd               m4, m6, m4
    mova [dstq+strideq*1+32*4], m0
    mova [dstq+strideq*1+32*5], m1
    mova [dstq+strideq*1+32*6], m2
    mova [dstq+strideq*1+32*7], m3
    mova       [maskq+32*1], m4
    sub                  hd, 2
    jg .w128_loop
    RET
ALIGN function_align
.main:
%macro W_MASK 2-6 11, 12, 13 ; dst/src1, mask/src2, pw_64, rnd, mul
    mova                m%1, [tmp1q+32*%1]
    mova                m%2, [tmp2q+32*%1]
    punpcklwd            m8, m%2, m%1
    punpckhwd            m9, m%2, m%1
    psubsw              m%1, m%2
    pabsw               m%1, m%1
    psubusw              m7, m10, m%1
    psrlw                m7, 10       ; 64-m
    psubw               m%2, m%3, m7  ; m
    punpcklwd           m%1, m7, m%2
    punpckhwd            m7, m%2
    pmaddwd             m%1, m8
    pmaddwd              m7, m9
    psrad               m%1, 5
    psrad                m7, 5
    packssdw            m%1, m7
    pmaxsw              m%1, m%4
    psubsw              m%1, m%4
    pmulhw              m%1, m%5
%endmacro
    W_MASK                0, 4
    W_MASK                1, 5
    phaddw               m4, m5
    W_MASK                2, 5
    W_MASK                3, 6
    phaddw               m5, m6
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    ret

cglobal w_mask_422_16bpc, 4, 8, 16, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_422_avx2_table
    lea                  r7, [w_mask_422_avx2_table]
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    vpbroadcastb        m14, r7m ; sign
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd        m10, [base+pw_27615]
    vpbroadcastd        m11, [base+pw_64]
    vpbroadcastd        m12, [base+bidir_rnd+r6*4]
    vpbroadcastd        m13, [base+bidir_mul+r6*4]
    mova                m15, [base+deint_shuf]
    mov               maskq, maskmp
    add                  wq, r7
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti128        xm0, m0, 1
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    cmp                  hd, 8
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    vextracti128        xm1, m1, 1
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
    je .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti128        xm2, m2, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm3
    movhps [dstq+strideq*1], xm3
    vextracti128        xm3, m3, 1
    movq   [dstq+strideq*2], xm3
    movhps [dstq+stride3q ], xm3
.w4_end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w8:
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+stride3q ], m1, 1
    sub                  hd, 8
    jl .w8_end
    lea                dstq, [dstq+strideq*4]
    mova         [dstq+strideq*0], xm2
    vextracti128 [dstq+strideq*1], m2, 1
    mova         [dstq+strideq*2], xm3
    vextracti128 [dstq+stride3q ], m3, 1
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w16:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w32:
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*1+32*0], m2
    mova [dstq+strideq*1+32*1], m3
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    mova        [dstq+32*2], m2
    mova        [dstq+32*3], m3
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    mova        [dstq+32*2], m2
    mova        [dstq+32*3], m3
    call .main
    mova        [dstq+32*4], m0
    mova        [dstq+32*5], m1
    mova        [dstq+32*6], m2
    mova        [dstq+32*7], m3
    dec                  hd
    jg .w128_loop
    RET
ALIGN function_align
.main:
    W_MASK                0, 4
    W_MASK                1, 5
    phaddw               m4, m5
    W_MASK                2, 5
    W_MASK                3, 6
    phaddw               m5, m6
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    packuswb             m4, m5
    pxor                 m5, m5
    psubb                m4, m14
    pavgb                m4, m5
    vpermd               m4, m15, m4
    mova            [maskq], m4
    add               maskq, 32
    ret

cglobal w_mask_444_16bpc, 4, 8, 11, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_444_avx2_table
    lea                  r7, [w_mask_444_avx2_table]
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd        m10, [base+pw_27615]
    vpbroadcastd         m4, [base+pw_64]
    vpbroadcastd         m5, [base+bidir_rnd+r6*4]
    vpbroadcastd         m6, [base+bidir_mul+r6*4]
    mov               maskq, maskmp
    add                  wq, r7
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti128        xm0, m0, 1
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    cmp                  hd, 8
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    vextracti128        xm1, m1, 1
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
    je .w4_end
    call .main
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti128        xm0, m0, 1
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    vextracti128        xm1, m1, 1
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
.w4_end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w8:
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+stride3q ], m1, 1
    sub                  hd, 4
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w16:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    call .main
    add                dstq, strideq
.w32:
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    call .main
    mova        [dstq+32*2], m0
    mova        [dstq+32*3], m1
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    call .main
    mova        [dstq+32*2], m0
    mova        [dstq+32*3], m1
    call .main
    mova        [dstq+32*4], m0
    mova        [dstq+32*5], m1
    call .main
    mova        [dstq+32*6], m0
    mova        [dstq+32*7], m1
    dec                  hd
    jg .w128_loop
    RET
ALIGN function_align
.main:
    W_MASK                0, 2, 4, 5, 6
    W_MASK                1, 3, 4, 5, 6
    packuswb             m2, m3
    vpermq               m2, m2, q3120
    add               tmp1q, 32*2
    add               tmp2q, 32*2
    mova            [maskq], m2
    add               maskq, 32
    ret

; (a * (64 - m) + b * m + 32) >> 6
; = (((b - a) * m + 32) >> 6) + a
; = (((b - a) * (m << 9) + 16384) >> 15) + a
;   except m << 9 overflows int16_t when m == 64 (which is possible),
;   but if we negate m it works out (-64 << 9 == -32768).
; = (((a - b) * (m * -512) + 16384) >> 15) + a
cglobal blend_16bpc, 3, 7, 7, dst, ds, tmp, w, h, mask
%define base r6-blend_avx2_table
    lea                  r6, [blend_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r6+wq*4]
    movifnidn         maskq, maskmp
    vpbroadcastd         m6, [base+pw_m512]
    add                  wq, r6
    lea                  r6, [dsq*3]
    jmp                  wq
.w4:
    pmovzxbw             m3, [maskq]
    movq                xm0, [dstq+dsq*0]
    movhps              xm0, [dstq+dsq*1]
    vpbroadcastq         m1, [dstq+dsq*2]
    vpbroadcastq         m2, [dstq+r6   ]
    vpblendd             m0, m1, 0x30
    vpblendd             m0, m2, 0xc0
    psubw                m1, m0, [tmpq]
    add               maskq, 16
    add                tmpq, 32
    pmullw               m3, m6
    pmulhrsw             m1, m3
    paddw                m0, m1
    vextracti128        xm1, m0, 1
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    movq       [dstq+dsq*2], xm1
    movhps     [dstq+r6   ], xm1
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w4
    RET
.w8:
    pmovzxbw             m4, [maskq+16*0]
    pmovzxbw             m5, [maskq+16*1]
    mova                xm0, [dstq+dsq*0]
    vinserti128          m0, [dstq+dsq*1], 1
    mova                xm1, [dstq+dsq*2]
    vinserti128          m1, [dstq+r6   ], 1
    psubw                m2, m0, [tmpq+32*0]
    psubw                m3, m1, [tmpq+32*1]
    add               maskq, 16*2
    add                tmpq, 32*2
    pmullw               m4, m6
    pmullw               m5, m6
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    mova         [dstq+dsq*2], xm1
    vextracti128 [dstq+r6   ], m1, 1
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w8
    RET
.w16:
    pmovzxbw             m4, [maskq+16*0]
    pmovzxbw             m5, [maskq+16*1]
    mova                 m0,     [dstq+dsq*0]
    psubw                m2, m0, [tmpq+ 32*0]
    mova                 m1,     [dstq+dsq*1]
    psubw                m3, m1, [tmpq+ 32*1]
    add               maskq, 16*2
    add                tmpq, 32*2
    pmullw               m4, m6
    pmullw               m5, m6
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w16
    RET
.w32:
    pmovzxbw             m4, [maskq+16*0]
    pmovzxbw             m5, [maskq+16*1]
    mova                 m0,     [dstq+32*0]
    psubw                m2, m0, [tmpq+32*0]
    mova                 m1,     [dstq+32*1]
    psubw                m3, m1, [tmpq+32*1]
    add               maskq, 16*2
    add                tmpq, 32*2
    pmullw               m4, m6
    pmullw               m5, m6
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    add                dstq, dsq
    dec                  hd
    jg .w32
    RET

INIT_XMM avx2
cglobal blend_v_16bpc, 3, 6, 6, dst, ds, tmp, w, h
%define base r5-blend_v_avx2_table
    lea                  r5, [blend_v_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  wq
.w2:
    vpbroadcastd         m2, [base+obmc_masks_avx2+2*2]
.w2_loop:
    movd                 m0, [dstq+dsq*0]
    pinsrd               m0, [dstq+dsq*1], 1
    movq                 m1, [tmpq]
    add                tmpq, 4*2
    psubw                m1, m0, m1
    pmulhrsw             m1, m2
    paddw                m0, m1
    movd       [dstq+dsq*0], m0
    pextrd     [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w2_loop
    RET
.w4:
    vpbroadcastq         m2, [base+obmc_masks_avx2+4*2]
.w4_loop:
    movq                 m0, [dstq+dsq*0]
    movhps               m0, [dstq+dsq*1]
    psubw                m1, m0, [tmpq]
    add                tmpq, 8*2
    pmulhrsw             m1, m2
    paddw                m0, m1
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w4_loop
    RET
INIT_YMM avx2
.w8:
    vbroadcasti128       m2, [base+obmc_masks_avx2+8*2]
.w8_loop:
    mova                xm0, [dstq+dsq*0]
    vinserti128          m0, [dstq+dsq*1], 1
    psubw                m1, m0, [tmpq]
    add                tmpq, 16*2
    pmulhrsw             m1, m2
    paddw                m0, m1
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
.w16:
    mova                 m4, [base+obmc_masks_avx2+16*2]
.w16_loop:
    mova                 m0,     [dstq+dsq*0]
    psubw                m2, m0, [tmpq+ 32*0]
    mova                 m1,     [dstq+dsq*1]
    psubw                m3, m1, [tmpq+ 32*1]
    add                tmpq, 32*2
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
    paddw                m0, m2
    paddw                m1, m3
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w16_loop
    RET
.w32:
%if WIN64
    movaps         [rsp+ 8], xmm6
    movaps         [rsp+24], xmm7
%endif
    mova                 m6, [base+obmc_masks_avx2+32*2]
    vbroadcasti128       m7, [base+obmc_masks_avx2+32*3]
.w32_loop:
    mova                 m0,     [dstq+dsq*0+32*0]
    psubw                m3, m0, [tmpq      +32*0]
    mova                xm2,     [dstq+dsq*0+32*1]
    mova                xm5,     [tmpq      +32*1]
    mova                 m1,     [dstq+dsq*1+32*0]
    psubw                m4, m1, [tmpq      +32*2]
    vinserti128          m2,     [dstq+dsq*1+32*1], 1
    vinserti128          m5,     [tmpq      +32*3], 1
    add                tmpq, 32*4
    psubw                m5, m2, m5
    pmulhrsw             m3, m6
    pmulhrsw             m4, m6
    pmulhrsw             m5, m7
    paddw                m0, m3
    paddw                m1, m4
    paddw                m2, m5
    mova         [dstq+dsq*0+32*0], m0
    mova         [dstq+dsq*1+32*0], m1
    mova         [dstq+dsq*0+32*1], xm2
    vextracti128 [dstq+dsq*1+32*1], m2, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w32_loop
%if WIN64
    movaps             xmm6, [rsp+ 8]
    movaps             xmm7, [rsp+24]
%endif
    RET

%macro BLEND_H_ROW 2-3 0; dst_off, tmp_off, inc_tmp
    mova                 m0,     [dstq+32*(%1+0)]
    psubw                m2, m0, [tmpq+32*(%2+0)]
    mova                 m1,     [dstq+32*(%1+1)]
    psubw                m3, m1, [tmpq+32*(%2+1)]
%if %3
    add                tmpq, 32*%3
%endif
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
    paddw                m0, m2
    paddw                m1, m3
    mova   [dstq+32*(%1+0)], m0
    mova   [dstq+32*(%1+1)], m1
%endmacro

INIT_XMM avx2
cglobal blend_h_16bpc, 3, 6, 6, dst, ds, tmp, w, h, mask
%define base r5-blend_h_avx2_table
    lea                  r5, [blend_h_avx2_table]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    lea               maskq, [base+obmc_masks_avx2+hq*2]
    lea                  hd, [hq*3]
    shr                  hd, 2 ; h * 3/4
    lea               maskq, [maskq+hq*2]
    neg                  hq
    jmp                  wq
.w2:
    movd                 m0, [dstq+dsq*0]
    pinsrd               m0, [dstq+dsq*1], 1
    movd                 m2, [maskq+hq*2]
    movq                 m1, [tmpq]
    add                tmpq, 4*2
    punpcklwd            m2, m2
    psubw                m1, m0, m1
    pmulhrsw             m1, m2
    paddw                m0, m1
    movd       [dstq+dsq*0], m0
    pextrd     [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w2
    RET
.w4:
    mova                 m3, [blend_shuf]
.w4_loop:
    movq                 m0, [dstq+dsq*0]
    movhps               m0, [dstq+dsq*1]
    movd                 m2, [maskq+hq*2]
    psubw                m1, m0, [tmpq]
    add                tmpq, 8*2
    pshufb               m2, m3
    pmulhrsw             m1, m2
    paddw                m0, m1
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w4_loop
    RET
INIT_YMM avx2
.w8:
    vbroadcasti128       m3, [blend_shuf]
    shufpd               m3, m3, 0x0c
.w8_loop:
    mova                xm0, [dstq+dsq*0]
    vinserti128          m0, [dstq+dsq*1], 1
    vpbroadcastd         m2, [maskq+hq*2]
    psubw                m1, m0, [tmpq]
    add                tmpq, 16*2
    pshufb               m2, m3
    pmulhrsw             m1, m2
    paddw                m0, m1
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w8_loop
    RET
.w16:
    vpbroadcastw         m4, [maskq+hq*2]
    vpbroadcastw         m5, [maskq+hq*2+2]
    mova                 m0,     [dstq+dsq*0]
    psubw                m2, m0, [tmpq+ 32*0]
    mova                 m1,     [dstq+dsq*1]
    psubw                m3, m1, [tmpq+ 32*1]
    add                tmpq, 32*2
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w16
    RET
.w32:
    vpbroadcastw         m4, [maskq+hq*2]
    BLEND_H_ROW           0, 0, 2
    add                dstq, dsq
    inc                  hq
    jl .w32
    RET
.w64:
    vpbroadcastw         m4, [maskq+hq*2]
    BLEND_H_ROW           0, 0
    BLEND_H_ROW           2, 2, 4
    add                dstq, dsq
    inc                  hq
    jl .w64
    RET
.w128:
    vpbroadcastw         m4, [maskq+hq*2]
    BLEND_H_ROW           0,  0
    BLEND_H_ROW           2,  2, 8
    BLEND_H_ROW           4, -4
    BLEND_H_ROW           6, -2
    add                dstq, dsq
    inc                  hq
    jl .w128
    RET

cglobal emu_edge_16bpc, 10, 13, 1, bw, bh, iw, ih, x, y, dst, dstride, src, sstride, \
                                   bottomext, rightext
    ; we assume that the buffer (stride) is larger than width, so we can
    ; safely overwrite by a few bytes

    ; ref += iclip(y, 0, ih - 1) * PXSTRIDE(ref_stride)
    xor                r12d, r12d
    lea                 r10, [ihq-1]
    cmp                  yq, ihq
    cmovs               r10, yq
    test                 yq, yq
    cmovs               r10, r12
    imul                r10, sstrideq
    add                srcq, r10

    ; ref += iclip(x, 0, iw - 1)
    lea                 r10, [iwq-1]
    cmp                  xq, iwq
    cmovs               r10, xq
    test                 xq, xq
    cmovs               r10, r12
    lea                srcq, [srcq+r10*2]

    ; bottom_ext = iclip(y + bh - ih, 0, bh - 1)
    lea          bottomextq, [yq+bhq]
    sub          bottomextq, ihq
    lea                  r3, [bhq-1]
    cmovs        bottomextq, r12

    DEFINE_ARGS bw, bh, iw, ih, x, topext, dst, dstride, src, sstride, \
                bottomext, rightext

    ; top_ext = iclip(-y, 0, bh - 1)
    neg             topextq
    cmovs           topextq, r12
    cmp          bottomextq, bhq
    cmovns       bottomextq, r3
    cmp             topextq, bhq
    cmovg           topextq, r3

    ; right_ext = iclip(x + bw - iw, 0, bw - 1)
    lea           rightextq, [xq+bwq]
    sub           rightextq, iwq
    lea                  r2, [bwq-1]
    cmovs         rightextq, r12

    DEFINE_ARGS bw, bh, iw, ih, leftext, topext, dst, dstride, src, sstride, \
                bottomext, rightext

    ; left_ext = iclip(-x, 0, bw - 1)
    neg            leftextq
    cmovs          leftextq, r12
    cmp           rightextq, bwq
    cmovns        rightextq, r2
    cmp            leftextq, bwq
    cmovns         leftextq, r2

    DEFINE_ARGS bw, centerh, centerw, dummy, leftext, topext, \
                dst, dstride, src, sstride, bottomext, rightext

    ; center_h = bh - top_ext - bottom_ext
    lea                  r3, [bottomextq+topextq]
    sub            centerhq, r3

    ; blk += top_ext * PXSTRIDE(dst_stride)
    mov                  r2, topextq
    imul                 r2, dstrideq
    add                dstq, r2
    mov                 r9m, dstq

    ; center_w = bw - left_ext - right_ext
    mov            centerwq, bwq
    lea                  r3, [rightextq+leftextq]
    sub            centerwq, r3

%macro v_loop 3 ; need_left_ext, need_right_ext, suffix
.v_loop_%3:
%if %1
    ; left extension
    xor                  r3, r3
    vpbroadcastw         m0, [srcq]
.left_loop_%3:
    mova        [dstq+r3*2], m0
    add                  r3, 16
    cmp                  r3, leftextq
    jl .left_loop_%3

    ; body
    lea                 r12, [dstq+leftextq*2]
%endif
    xor                  r3, r3
.body_loop_%3:
    movu                 m0, [srcq+r3*2]
%if %1
    movu         [r12+r3*2], m0
%else
    movu        [dstq+r3*2], m0
%endif
    add                  r3, 16
    cmp                  r3, centerwq
    jl .body_loop_%3

%if %2
    ; right extension
%if %1
    lea                 r12, [r12+centerwq*2]
%else
    lea                 r12, [dstq+centerwq*2]
%endif
    xor                  r3, r3
    vpbroadcastw         m0, [srcq+centerwq*2-2]
.right_loop_%3:
    movu         [r12+r3*2], m0
    add                  r3, 16
    cmp                  r3, rightextq
    jl .right_loop_%3

%endif
    add                dstq, dstrideq
    add                srcq, sstrideq
    dec            centerhq
    jg .v_loop_%3
%endmacro

    test           leftextq, leftextq
    jnz .need_left_ext
    test          rightextq, rightextq
    jnz .need_right_ext
    v_loop                0, 0, 0
    jmp .body_done

.need_left_ext:
    test          rightextq, rightextq
    jnz .need_left_right_ext
    v_loop                1, 0, 1
    jmp .body_done

.need_left_right_ext:
    v_loop                1, 1, 2
    jmp .body_done

.need_right_ext:
    v_loop                0, 1, 3

.body_done:
    ; bottom edge extension
    test         bottomextq, bottomextq
    jz .top
    mov                srcq, dstq
    sub                srcq, dstrideq
    xor                  r1, r1
.bottom_x_loop:
    mova                 m0, [srcq+r1*2]
    lea                  r3, [dstq+r1*2]
    mov                  r4, bottomextq
.bottom_y_loop:
    mova               [r3], m0
    add                  r3, dstrideq
    dec                  r4
    jg .bottom_y_loop
    add                  r1, 16
    cmp                  r1, bwq
    jl .bottom_x_loop

.top:
    ; top edge extension
    test            topextq, topextq
    jz .end
    mov                srcq, r9m
    mov                dstq, dstm
    xor                  r1, r1
.top_x_loop:
    mova                 m0, [srcq+r1*2]
    lea                  r3, [dstq+r1*2]
    mov                  r4, topextq
.top_y_loop:
    mova               [r3], m0
    add                  r3, dstrideq
    dec                  r4
    jg .top_y_loop
    add                  r1, 16
    cmp                  r1, bwq
    jl .top_x_loop

.end:
    RET

cglobal resize_16bpc, 6, 12, 16, dst, dst_stride, src, src_stride, \
                                 dst_w, h, src_w, dx, mx0, pxmax
    sub          dword mx0m, 4<<14
    sub        dword src_wm, 8
    vpbroadcastd         m5, dxm
    vpbroadcastd         m8, mx0m
    vpbroadcastd         m6, src_wm
 DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, h, x, _, _, pxmax
    LEA                  r7, $$
%define base r7-$$
    vpbroadcastd         m3, [base+pd_64]
    vpbroadcastw        xm7, pxmaxm
    pmaddwd              m2, m5, [base+rescale_mul] ; dx*[0,1,2,3,4,5,6,7]
    pslld                m5, 3                      ; dx*8
    pslld                m6, 14
    paddd                m8, m2                     ; mx+[0..7]*dx
.loop_y:
    xor                  xd, xd
    mova                 m4, m8             ; per-line working version of mx
.loop_x:
    vpbroadcastd        m10, [base+pd_63]
    pxor                 m2, m2
    pmaxsd               m0, m4, m2
    psrad                m9, m4, 8          ; filter offset (unmasked)
    pminsd               m0, m6             ; iclip(mx, 0, src_w-8)
    psubd                m1, m4, m0         ; pshufb offset
    psrad                m0, 14             ; clipped src_x offset
    psrad                m1, 14             ; pshufb edge_emu offset
    pand                 m9, m10            ; filter offset (masked)
    ; load source pixels
    movd                r8d, xm0
    pextrd              r9d, xm0, 1
    pextrd             r10d, xm0, 2
    pextrd             r11d, xm0, 3
    vextracti128        xm0, m0, 1
    movu               xm10, [srcq+r8*2]
    movu               xm11, [srcq+r9*2]
    movu               xm12, [srcq+r10*2]
    movu               xm13, [srcq+r11*2]
    movd                r8d, xm0
    pextrd              r9d, xm0, 1
    pextrd             r10d, xm0, 2
    pextrd             r11d, xm0, 3
    vinserti128         m10, [srcq+r8*2], 1
    vinserti128         m11, [srcq+r9*2], 1
    vinserti128         m12, [srcq+r10*2], 1
    vinserti128         m13, [srcq+r11*2], 1
    ptest                m1, m1
    jz .filter
    movq                 r9, xm1
    pextrq              r11, xm1, 1
    movsxd               r8, r9d
    sar                  r9, 32
    movsxd              r10, r11d
    sar                 r11, 32
    vextracti128        xm1, m1, 1
    movu               xm14, [base+resize_shuf+8+r8*2]
    movu               xm15, [base+resize_shuf+8+r9*2]
    movu                xm0, [base+resize_shuf+8+r10*2]
    movu                xm2, [base+resize_shuf+8+r11*2]
    movq                 r9, xm1
    pextrq              r11, xm1, 1
    movsxd               r8, r9d
    sar                  r9, 32
    movsxd              r10, r11d
    sar                 r11, 32
    vinserti128         m14, [base+resize_shuf+8+r8*2], 1
    vinserti128         m15, [base+resize_shuf+8+r9*2], 1
    vinserti128          m0, [base+resize_shuf+8+r10*2], 1
    vinserti128          m2, [base+resize_shuf+8+r11*2], 1
    pshufb              m10, m14
    pshufb              m11, m15
    pshufb              m12, m0
    pshufb              m13, m2
.filter:
    movd                r8d, xm9
    pextrd              r9d, xm9, 1
    pextrd             r10d, xm9, 2
    pextrd             r11d, xm9, 3
    vextracti128        xm9, m9, 1
    movq               xm14, [base+resize_filter+r8*8]
    movq               xm15, [base+resize_filter+r9*8]
    movq                xm0, [base+resize_filter+r10*8]
    movq                xm2, [base+resize_filter+r11*8]
    movd                r8d, xm9
    pextrd              r9d, xm9, 1
    pextrd             r10d, xm9, 2
    pextrd             r11d, xm9, 3
    movhps             xm14, [base+resize_filter+r8*8]
    movhps             xm15, [base+resize_filter+r9*8]
    movhps              xm0, [base+resize_filter+r10*8]
    movhps              xm2, [base+resize_filter+r11*8]
    pmovsxbw            m14, xm14
    pmovsxbw            m15, xm15
    pmovsxbw             m0, xm0
    pmovsxbw             m2, xm2
    pmaddwd             m10, m14
    pmaddwd             m11, m15
    pmaddwd             m12, m0
    pmaddwd             m13, m2
    phaddd              m10, m11
    phaddd              m12, m13
    phaddd              m10, m12
    psubd               m10, m3, m10
    psrad               m10, 7
    vextracti128        xm0, m10, 1
    packusdw           xm10, xm0
    pminsw             xm10, xm7
    mova        [dstq+xq*2], xm10
    paddd                m4, m5
    add                  xd, 8
    cmp                  xd, dst_wd
    jl .loop_x
    add                dstq, dst_strideq
    add                srcq, src_strideq
    dec                  hd
    jg .loop_y
    RET

%endif ; ARCH_X86_64
