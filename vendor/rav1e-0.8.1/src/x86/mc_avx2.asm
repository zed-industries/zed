; Copyright © 2018-2021, VideoLAN and dav1d authors
; Copyright © 2018-2021, Two Orioles, LLC
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

; dav1d_obmc_masks[] with 64-x interleaved
obmc_masks:     db  0,  0,  0,  0
                ; 2
                db 45, 19, 64,  0
                ; 4
                db 39, 25, 50, 14, 59,  5, 64,  0
                ; 8
                db 36, 28, 42, 22, 48, 16, 53, 11, 57,  7, 61,  3, 64,  0, 64,  0
                ; 16
                db 34, 30, 37, 27, 40, 24, 43, 21, 46, 18, 49, 15, 52, 12, 54, 10
                db 56,  8, 58,  6, 60,  4, 61,  3, 64,  0, 64,  0, 64,  0, 64,  0
                ; 32
                db 33, 31, 35, 29, 36, 28, 38, 26, 40, 24, 41, 23, 43, 21, 44, 20
                db 45, 19, 47, 17, 48, 16, 50, 14, 51, 13, 52, 12, 53, 11, 55,  9
                db 56,  8, 57,  7, 58,  6, 59,  5, 60,  4, 60,  4, 61,  3, 62,  2
                db 64,  0, 64,  0, 64,  0, 64,  0, 64,  0, 64,  0, 64,  0, 64,  0

warp_8x8_shufA: db  0,  2,  4,  6,  1,  3,  5,  7,  1,  3,  5,  7,  2,  4,  6,  8
                db  4,  6,  8, 10,  5,  7,  9, 11,  5,  7,  9, 11,  6,  8, 10, 12
warp_8x8_shufB: db  2,  4,  6,  8,  3,  5,  7,  9,  3,  5,  7,  9,  4,  6,  8, 10
                db  6,  8, 10, 12,  7,  9, 11, 13,  7,  9, 11, 13,  8, 10, 12, 14
subpel_h_shuf4: db  0,  1,  2,  3,  1,  2,  3,  4,  8,  9, 10, 11,  9, 10, 11, 12
                db  2,  3,  4,  5,  3,  4,  5,  6, 10, 11, 12, 13, 11, 12, 13, 14
subpel_h_shufA: db  0,  1,  2,  3,  1,  2,  3,  4,  2,  3,  4,  5,  3,  4,  5,  6
subpel_h_shufB: db  4,  5,  6,  7,  5,  6,  7,  8,  6,  7,  8,  9,  7,  8,  9, 10
subpel_h_shufC: db  8,  9, 10, 11,  9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14
subpel_v_shuf4: db  0,  4,  8, 12,  1,  5,  9, 13,  2,  6, 10, 14,  3,  7, 11, 15
subpel_s_shuf2: db  0,  1,  2,  3,  0,  1,  2,  3,  8,  9, 10, 11,  8,  9, 10, 11
subpel_s_shuf8: db  0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
bilin_h_shuf4:  db  0,  1,  1,  2,  2,  3,  3,  4,  8,  9,  9, 10, 10, 11, 11, 12
bilin_h_shuf8:  db  0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8
bilin_v_shuf4:  db  4,  0,  5,  1,  6,  2,  7,  3,  8,  4,  9,  5, 10,  6, 11,  7
deint_shuf4:    db  0,  4,  1,  5,  2,  6,  3,  7,  4,  8,  5,  9,  6, 10,  7, 11
blend_shuf:     db  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  2,  3,  2,  3,  2,  3
pb_8x0_8x8:     db  0,  0,  0,  0,  0,  0,  0,  0,  8,  8,  8,  8,  8,  8,  8,  8
bdct_lb_dw:     db  0,  0,  0,  0,  4,  4,  4,  4,  8,  8,  8,  8, 12, 12, 12, 12
wswap:          db  2,  3,  0,  1,  6,  7,  4,  5, 10, 11,  8,  9, 14, 15, 12, 13
rescale_mul:    dd  0,  1,  2,  3,  4,  5,  6,  7
resize_shuf:    db  0,  0,  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  7,  7,  7,  7

wm_420_sign:    dd 0x01020102, 0x01010101
wm_422_sign:    dd 0x80808080, 0x7f7f7f7f

pb_64:   times 4 db 64
pw_m256: times 2 dw -256
pw_15:   times 2 dw 15
pw_32:   times 2 dw 32
pw_34:   times 2 dw 34
pw_258:  times 2 dw 258
pw_512:  times 2 dw 512
pw_1024: times 2 dw 1024
pw_2048: times 2 dw 2048
pw_6903: times 2 dw 6903
pw_8192: times 2 dw 8192
pd_32:           dd 32
pd_63:           dd 63
pd_512:          dd 512
pd_32768:        dd 32768
pd_0x3ff:        dd 0x3ff
pd_0x4000:       dd 0x4000
pq_0x40000000:   dq 0x40000000

cextern mc_subpel_filters
cextern mc_warp_filter2
cextern resize_filter

%define subpel_filters (mangle(private_prefix %+ _mc_subpel_filters)-8)

%macro BASE_JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base %1_%2
    %%table:
    %rep %0 - 2
        dw %%base %+ _w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%macro HV_JMP_TABLE 5-*
    %xdefine %%prefix mangle(private_prefix %+ _%1_%2_8bpc_%3)
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

%macro BIDIR_JMP_TABLE 2-*
    %xdefine %1_%2_table (%%table - 2*%3)
    %xdefine %%base %1_%2_table
    %xdefine %%prefix mangle(private_prefix %+ _%1_8bpc_%2)
    %%table:
    %rep %0 - 2
        dd %%prefix %+ .w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%macro SCALED_JMP_TABLE 2-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base mangle(private_prefix %+ _%1_8bpc_%2)
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

%xdefine put_avx2 mangle(private_prefix %+ _put_bilin_8bpc_avx2.put)
%xdefine prep_avx2 mangle(private_prefix %+ _prep_bilin_8bpc_avx2.prep)

%define table_offset(type, fn) type %+ fn %+ SUFFIX %+ _table - type %+ SUFFIX

BASE_JMP_TABLE   put,  avx2,            2, 4, 8, 16, 32, 64, 128
BASE_JMP_TABLE   prep, avx2,               4, 8, 16, 32, 64, 128
HV_JMP_TABLE     put,  bilin, avx2,  7, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE     prep, bilin, avx2,  7,    4, 8, 16, 32, 64, 128
HV_JMP_TABLE     put,  8tap,  avx2,  3, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE     prep, 8tap,  avx2,  1,    4, 8, 16, 32, 64, 128
SCALED_JMP_TABLE put_8tap_scaled, avx2, 2, 4, 8, 16, 32, 64, 128
SCALED_JMP_TABLE prep_8tap_scaled, avx2,   4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE  avg, avx2,                4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE  w_avg, avx2,              4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE  mask, avx2,               4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE  w_mask_420, avx2,         4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE  w_mask_422, avx2,         4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE  w_mask_444, avx2,         4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE  blend, avx2,              4, 8, 16, 32
BIDIR_JMP_TABLE  blend_v, avx2,         2, 4, 8, 16, 32
BIDIR_JMP_TABLE  blend_h, avx2,         2, 4, 8, 16, 32, 32, 32

SECTION .text

INIT_XMM avx2
cglobal put_bilin_8bpc, 4, 8, 0, dst, ds, src, ss, w, h, mxy
    movifnidn          mxyd, r6m ; mx
    lea                  r7, [put_avx2]
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
    movzx               r6d, word [srcq+ssq*0]
    movzx               r7d, word [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6w
    mov        [dstq+dsq*1], r7w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w2
    RET
.put_w4:
    mov                 r6d, [srcq+ssq*0]
    mov                 r7d, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6d
    mov        [dstq+dsq*1], r7d
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w4
    RET
.put_w8:
    mov                  r6, [srcq+ssq*0]
    mov                  r7, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6
    mov        [dstq+dsq*1], r7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w8
    RET
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
INIT_YMM avx2
.put_w32:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w32
    RET
.put_w64:
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
    jg .put_w64
    RET
.put_w128:
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
    jg .put_w128
    RET
.h:
    ; (16 * src[x] + (mx * (src[x + 1] - src[x])) + 8) >> 4
    ; = ((16 - mx) * src[x] + mx * src[x + 1] + 8) >> 4
    imul               mxyd, 255
    vbroadcasti128       m4, [bilin_h_shuf8]
    add                mxyd, 16
    movd                xm5, mxyd
    mov                mxyd, r7m ; my
    vpbroadcastw         m5, xm5
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_h)]
    vpbroadcastd         m3, [pw_2048]
    add                  wq, r7
    jmp                  wq
.h_w2:
    movd                xm0, [srcq+ssq*0]
    pinsrd              xm0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pshufb              xm0, xm4
    pmaddubsw           xm0, xm5
    pmulhrsw            xm0, xm3
    packuswb            xm0, xm0
    pextrw     [dstq+dsq*0], xm0, 0
    pextrw     [dstq+dsq*1], xm0, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2
    RET
.h_w4:
    mova                xm4, [bilin_h_shuf4]
.h_w4_loop:
    movq                xm0, [srcq+ssq*0]
    movhps              xm0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              xm0, xm4
    pmaddubsw           xm0, xm5
    pmulhrsw            xm0, xm3
    packuswb            xm0, xm0
    movd       [dstq+dsq*0], xm0
    pextrd     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4_loop
    RET
.h_w8:
    movu                xm0, [srcq+ssq*0]
    movu                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              xm0, xm4
    pshufb              xm1, xm4
    pmaddubsw           xm0, xm5
    pmaddubsw           xm1, xm5
    pmulhrsw            xm0, xm3
    pmulhrsw            xm1, xm3
    packuswb            xm0, xm1
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    movu                xm0, [srcq+ssq*0+8*0]
    vinserti128          m0, [srcq+ssq*1+8*0], 1
    movu                xm1, [srcq+ssq*0+8*1]
    vinserti128          m1, [srcq+ssq*1+8*1], 1
    lea                srcq, [srcq+ssq*2]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w16
    RET
.h_w32:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    add                srcq, ssq
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, dsq
    dec                  hd
    jg .h_w32
    RET
.h_w64:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    movu                 m1, [srcq+8*4]
    movu                 m2, [srcq+8*5]
    add                srcq, ssq
    pshufb               m1, m4
    pshufb               m2, m4
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmulhrsw             m1, m3
    pmulhrsw             m2, m3
    packuswb             m1, m2
    mova        [dstq+32*0], m0
    mova        [dstq+32*1], m1
    add                dstq, dsq
    dec                  hd
    jg .h_w64
    RET
.h_w128:
    mov                  r6, -32*3
.h_w128_loop:
    movu                 m0, [srcq+r6+32*3+8*0]
    movu                 m1, [srcq+r6+32*3+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    mova     [dstq+r6+32*3], m0
    add                  r6, 32
    jle .h_w128_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w128
    RET
.v:
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_v)]
    imul               mxyd, 255
    vpbroadcastd         m5, [pw_2048]
    add                mxyd, 16
    add                  wq, r7
    movd                xm4, mxyd
    vpbroadcastw         m4, xm4
    jmp                  wq
.v_w2:
    movd                xm0,      [srcq+ssq*0]
.v_w2_loop:
    pinsrw              xm1, xm0, [srcq+ssq*1], 1 ; 0 1
    lea                srcq,      [srcq+ssq*2]
    pinsrw              xm0, xm1, [srcq+ssq*0], 0 ; 2 1
    pshuflw             xm1, xm1, q2301           ; 1 0
    punpcklbw           xm1, xm0
    pmaddubsw           xm1, xm4
    pmulhrsw            xm1, xm5
    packuswb            xm1, xm1
    pextrw     [dstq+dsq*0], xm1, 1
    pextrw     [dstq+dsq*1], xm1, 0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movd                xm0, [srcq+ssq*0]
.v_w4_loop:
    vpbroadcastd        xm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd            xm1, xm2, xm0, 0x01 ; 0 1
    vpbroadcastd        xm0, [srcq+ssq*0]
    vpblendd            xm2, xm0, 0x02      ; 1 2
    punpcklbw           xm1, xm2
    pmaddubsw           xm1, xm4
    pmulhrsw            xm1, xm5
    packuswb            xm1, xm1
    movd       [dstq+dsq*0], xm1
    pextrd     [dstq+dsq*1], xm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movq                xm0, [srcq+ssq*0]
.v_w8_loop:
    movq                xm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklbw           xm1, xm0, xm2
    movq                xm0, [srcq+ssq*0]
    punpcklbw           xm2, xm0
    pmaddubsw           xm1, xm4
    pmaddubsw           xm2, xm4
    pmulhrsw            xm1, xm5
    pmulhrsw            xm2, xm5
    packuswb            xm1, xm2
    movq       [dstq+dsq*0], xm1
    movhps     [dstq+dsq*1], xm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w16:
    movu                xm0, [srcq+ssq*0]
.v_w16_loop:
    vbroadcasti128       m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd             m2, m3, m0, 0x0f ; 0 1
    vbroadcasti128       m0, [srcq+ssq*0]
    vpblendd             m3, m0, 0xf0     ; 1 2
    punpcklbw            m1, m2, m3
    punpckhbw            m2, m3
    pmaddubsw            m1, m4
    pmaddubsw            m2, m4
    pmulhrsw             m1, m5
    pmulhrsw             m2, m5
    packuswb             m1, m2
    mova         [dstq+dsq*0], xm1
    vextracti128 [dstq+dsq*1], m1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    RET
.v_w32:
%macro PUT_BILIN_V_W32 0
    movu                 m0, [srcq+ssq*0]
%%loop:
    movu                 m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklbw            m1, m0, m3
    punpckhbw            m2, m0, m3
    movu                 m0, [srcq+ssq*0]
    pmaddubsw            m1, m4
    pmaddubsw            m2, m4
    pmulhrsw             m1, m5
    pmulhrsw             m2, m5
    packuswb             m1, m2
    punpcklbw            m2, m3, m0
    punpckhbw            m3, m0
    pmaddubsw            m2, m4
    pmaddubsw            m3, m4
    pmulhrsw             m2, m5
    pmulhrsw             m3, m5
    packuswb             m2, m3
    mova       [dstq+dsq*0], m1
    mova       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg %%loop
%endmacro
    PUT_BILIN_V_W32
    RET
.v_w64:
    movu                 m0, [srcq+32*0]
    movu                 m1, [srcq+32*1]
.v_w64_loop:
    add                srcq, ssq
    movu                 m3, [srcq+32*0]
    punpcklbw            m2, m0, m3
    punpckhbw            m0, m3
    pmaddubsw            m2, m4
    pmaddubsw            m0, m4
    pmulhrsw             m2, m5
    pmulhrsw             m0, m5
    packuswb             m2, m0
    mova                 m0, m3
    movu                 m3, [srcq+32*1]
    mova        [dstq+32*0], m2
    punpcklbw            m2, m1, m3
    punpckhbw            m1, m3
    pmaddubsw            m2, m4
    pmaddubsw            m1, m4
    pmulhrsw             m2, m5
    pmulhrsw             m1, m5
    packuswb             m2, m1
    mova                 m1, m3
    mova        [dstq+32*1], m2
    add                dstq, dsq
    dec                  hd
    jg .v_w64_loop
    RET
.v_w128:
    lea                 r6d, [hq+(3<<8)]
    mov                  r4, srcq
    mov                  r7, dstq
.v_w128_loop:
    PUT_BILIN_V_W32
    add                  r4, 32
    add                  r7, 32
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
    jg .v_w128_loop
    RET
.hv:
    ; (16 * src[x] + (my * (src[x + src_stride] - src[x])) + 128) >> 8
    ; = (src[x] + ((my * (src[x + src_stride] - src[x])) >> 4) + 8) >> 4
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_hv)]
    WIN64_SPILL_XMM       8
    shl                mxyd, 11 ; can't shift by 12 due to signed overflow
    vpbroadcastd         m7, [pw_15]
    movd                xm6, mxyd
    add                  wq, r7
    paddb                m5, m5
    vpbroadcastw         m6, xm6
    jmp                  wq
.hv_w2:
    vpbroadcastd        xm0, [srcq+ssq*0]
    pshufb              xm0, xm4
    pmaddubsw           xm0, xm5
.hv_w2_loop:
    movd                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pinsrd              xm1, [srcq+ssq*0], 1
    pshufb              xm1, xm4
    pmaddubsw           xm1, xm5             ; 1 _ 2 _
    shufps              xm2, xm0, xm1, q1032 ; 0 _ 1 _
    mova                xm0, xm1
    psubw               xm1, xm2
    pmulhw              xm1, xm6
    pavgw               xm2, xm7
    paddw               xm1, xm2
    psrlw               xm1, 4
    packuswb            xm1, xm1
    pextrw     [dstq+dsq*0], xm1, 0
    pextrw     [dstq+dsq*1], xm1, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    mova                xm4, [bilin_h_shuf4]
    movddup             xm0, [srcq+ssq*0]
    pshufb              xm0, xm4
    pmaddubsw           xm0, xm5
.hv_w4_loop:
    movq                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps              xm1, [srcq+ssq*0]
    pshufb              xm1, xm4
    pmaddubsw           xm1, xm5             ; 1 2
    shufps              xm2, xm0, xm1, q1032 ; 0 1
    mova                xm0, xm1
    psubw               xm1, xm2
    pmulhw              xm1, xm6
    pavgw               xm2, xm7
    paddw               xm1, xm2
    psrlw               xm1, 4
    packuswb            xm1, xm1
    movd       [dstq+dsq*0], xm1
    pextrd     [dstq+dsq*1], xm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    vbroadcasti128       m0, [srcq+ssq*0]
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w8_loop:
    movu                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti128          m1, [srcq+ssq*0], 1
    pshufb               m1, m4
    pmaddubsw            m1, m5           ; 1 2
    vperm2i128           m2, m0, m1, 0x21 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    pmulhw               m1, m6
    pavgw                m2, m7
    paddw                m1, m2
    psrlw                m1, 4
    vextracti128        xm2, m1, 1
    packuswb            xm1, xm2
    movq       [dstq+dsq*0], xm1
    movhps     [dstq+dsq*1], xm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
    movu                 m0, [srcq+ssq*0+8*0]
    vinserti128          m0, [srcq+ssq*0+8*1], 1
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w16_loop:
    movu                xm2, [srcq+ssq*1+8*0]
    vinserti128          m2, [srcq+ssq*1+8*1], 1
    lea                srcq, [srcq+ssq*2]
    movu                xm3, [srcq+ssq*0+8*0]
    vinserti128          m3, [srcq+ssq*0+8*1], 1
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m2, m5
    psubw                m1, m2, m0
    pmulhw               m1, m6
    pavgw                m0, m7
    paddw                m1, m0
    pmaddubsw            m0, m3, m5
    psubw                m3, m0, m2
    pmulhw               m3, m6
    pavgw                m2, m7
    paddw                m3, m2
    psrlw                m1, 4
    psrlw                m3, 4
    packuswb             m1, m3
    vpermq               m1, m1, q3120
    mova         [dstq+dsq*0], xm1
    vextracti128 [dstq+dsq*1], m1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    RET
.hv_w128:
    lea                 r6d, [hq+(3<<16)]
    jmp .hv_w32_start
.hv_w64:
    lea                 r6d, [hq+(1<<16)]
.hv_w32_start:
    mov                  r4, srcq
    mov                  r7, dstq
.hv_w32:
%if WIN64
    movaps              r4m, xmm8
%endif
.hv_w32_loop0:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
.hv_w32_loop:
    add                srcq, ssq
    movu                 m2, [srcq+8*0]
    movu                 m3, [srcq+8*1]
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    psubw                m8, m2, m0
    pmulhw               m8, m6
    pavgw                m0, m7
    paddw                m8, m0
    mova                 m0, m2
    psubw                m2, m3, m1
    pmulhw               m2, m6
    pavgw                m1, m7
    paddw                m2, m1
    mova                 m1, m3
    psrlw                m8, 4
    psrlw                m2, 4
    packuswb             m8, m2
    mova             [dstq], m8
    add                dstq, dsq
    dec                  hd
    jg .hv_w32_loop
    add                  r4, 32
    add                  r7, 32
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<16
    jg .hv_w32_loop0
%if WIN64
    movaps             xmm8, r4m
%endif
    RET

cglobal prep_bilin_8bpc, 3, 7, 0, tmp, src, stride, w, h, mxy, stride3
    movifnidn          mxyd, r5m ; mx
    lea                  r6, [prep%+SUFFIX]
    tzcnt                wd, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r6m ; my
    test               mxyd, mxyd
    jnz .v
.prep:
    movzx                wd, word [r6+wq*2+table_offset(prep,)]
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.prep_w4:
    movd                xm0, [srcq+strideq*0]
    pinsrd              xm0, [srcq+strideq*1], 1
    pinsrd              xm0, [srcq+strideq*2], 2
    pinsrd              xm0, [srcq+stride3q ], 3
    lea                srcq, [srcq+strideq*4]
    pmovzxbw             m0, xm0
    psllw                m0, 4
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 4
    jg .prep_w4
    RET
.prep_w8:
    movq                xm0, [srcq+strideq*0]
    movhps              xm0, [srcq+strideq*1]
    movq                xm1, [srcq+strideq*2]
    movhps              xm1, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    pmovzxbw             m0, xm0
    pmovzxbw             m1, xm1
    psllw                m0, 4
    psllw                m1, 4
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    add                tmpq, 32*2
    sub                  hd, 4
    jg .prep_w8
    RET
.prep_w16:
    pmovzxbw             m0, [srcq+strideq*0]
    pmovzxbw             m1, [srcq+strideq*1]
    pmovzxbw             m2, [srcq+strideq*2]
    pmovzxbw             m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    sub                  hd, 4
    jg .prep_w16
    RET
.prep_w32:
    pmovzxbw             m0, [srcq+strideq*0+16*0]
    pmovzxbw             m1, [srcq+strideq*0+16*1]
    pmovzxbw             m2, [srcq+strideq*1+16*0]
    pmovzxbw             m3, [srcq+strideq*1+16*1]
    lea                srcq, [srcq+strideq*2]
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    sub                  hd, 2
    jg .prep_w32
    RET
.prep_w64:
    pmovzxbw             m0, [srcq+16*0]
    pmovzxbw             m1, [srcq+16*1]
    pmovzxbw             m2, [srcq+16*2]
    pmovzxbw             m3, [srcq+16*3]
    add                srcq, strideq
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    dec                  hd
    jg .prep_w64
    RET
.prep_w128:
    pmovzxbw             m0, [srcq+16*0]
    pmovzxbw             m1, [srcq+16*1]
    pmovzxbw             m2, [srcq+16*2]
    pmovzxbw             m3, [srcq+16*3]
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    pmovzxbw             m0, [srcq+16*4]
    pmovzxbw             m1, [srcq+16*5]
    pmovzxbw             m2, [srcq+16*6]
    pmovzxbw             m3, [srcq+16*7]
    add                tmpq, 32*8
    add                srcq, strideq
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq-32*4], m0
    mova        [tmpq-32*3], m1
    mova        [tmpq-32*2], m2
    mova        [tmpq-32*1], m3
    dec                  hd
    jg .prep_w128
    RET
.h:
    ; 16 * src[x] + (mx * (src[x + 1] - src[x]))
    ; = (16 - mx) * src[x] + mx * src[x + 1]
    imul               mxyd, 255
    vbroadcasti128       m4, [bilin_h_shuf8]
    add                mxyd, 16
    movd                xm5, mxyd
    mov                mxyd, r6m ; my
    vpbroadcastw         m5, xm5
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_h)]
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.h_w4:
    vbroadcasti128       m4, [bilin_h_shuf4]
.h_w4_loop:
    movq                xm0, [srcq+strideq*0]
    movhps              xm0, [srcq+strideq*1]
    movq                xm1, [srcq+strideq*2]
    movhps              xm1, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vinserti128          m0, xm1, 1
    pshufb               m0, m4
    pmaddubsw            m0, m5
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h_w8:
.h_w8_loop:
    movu                xm0, [srcq+strideq*0]
    vinserti128          m0, [srcq+strideq*1], 1
    movu                xm1, [srcq+strideq*2]
    vinserti128          m1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    add                tmpq, 32*2
    sub                  hd, 4
    jg .h_w8_loop
    RET
.h_w16:
.h_w16_loop:
    movu                xm0, [srcq+strideq*0+8*0]
    vinserti128          m0, [srcq+strideq*0+8*1], 1
    movu                xm1, [srcq+strideq*1+8*0]
    vinserti128          m1, [srcq+strideq*1+8*1], 1
    movu                xm2, [srcq+strideq*2+8*0]
    vinserti128          m2, [srcq+strideq*2+8*1], 1
    movu                xm3, [srcq+stride3q +8*0]
    vinserti128          m3, [srcq+stride3q +8*1], 1
    lea                srcq, [srcq+strideq*4]
    pshufb               m0, m4
    pshufb               m1, m4
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    sub                  hd, 4
    jg .h_w16_loop
    RET
.h_w32:
.h_w32_loop:
    movu                xm0, [srcq+strideq*0+8*0]
    vinserti128          m0, [srcq+strideq*0+8*1], 1
    movu                xm1, [srcq+strideq*0+8*2]
    vinserti128          m1, [srcq+strideq*0+8*3], 1
    movu                xm2, [srcq+strideq*1+8*0]
    vinserti128          m2, [srcq+strideq*1+8*1], 1
    movu                xm3, [srcq+strideq*1+8*2]
    vinserti128          m3, [srcq+strideq*1+8*3], 1
    lea                srcq, [srcq+strideq*2]
    pshufb               m0, m4
    pshufb               m1, m4
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    sub                  hd, 2
    jg .h_w32_loop
    RET
.h_w64:
    movu                xm0, [srcq+8*0]
    vinserti128          m0, [srcq+8*1], 1
    movu                xm1, [srcq+8*2]
    vinserti128          m1, [srcq+8*3], 1
    movu                xm2, [srcq+8*4]
    vinserti128          m2, [srcq+8*5], 1
    movu                xm3, [srcq+8*6]
    vinserti128          m3, [srcq+8*7], 1
    add                srcq, strideq
    pshufb               m0, m4
    pshufb               m1, m4
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    add                tmpq, 32*4
    dec                  hd
    jg .h_w64
    RET
.h_w128:
    movu                xm0, [srcq+8*0]
    vinserti128          m0, [srcq+8*1], 1
    movu                xm1, [srcq+8*2]
    vinserti128          m1, [srcq+8*3], 1
    movu                xm2, [srcq+8*4]
    vinserti128          m2, [srcq+8*5], 1
    movu                xm3, [srcq+8*6]
    vinserti128          m3, [srcq+8*7], 1
    pshufb               m0, m4
    pshufb               m1, m4
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq+32*0], m0
    mova        [tmpq+32*1], m1
    mova        [tmpq+32*2], m2
    mova        [tmpq+32*3], m3
    movu                xm0, [srcq+8* 8]
    vinserti128          m0, [srcq+8* 9], 1
    movu                xm1, [srcq+8*10]
    vinserti128          m1, [srcq+8*11], 1
    movu                xm2, [srcq+8*12]
    vinserti128          m2, [srcq+8*13], 1
    movu                xm3, [srcq+8*14]
    vinserti128          m3, [srcq+8*15], 1
    add                tmpq, 32*8
    add                srcq, strideq
    pshufb               m0, m4
    pshufb               m1, m4
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq-32*4], m0
    mova        [tmpq-32*3], m1
    mova        [tmpq-32*2], m2
    mova        [tmpq-32*1], m3
    dec                  hd
    jg .h_w128
    RET
.v:
    WIN64_SPILL_XMM       7
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_v)]
    imul               mxyd, 255
    add                mxyd, 16
    add                  wq, r6
    lea            stride3q, [strideq*3]
    movd                xm6, mxyd
    vpbroadcastw         m6, xm6
    jmp                  wq
.v_w4:
    movd                xm0, [srcq+strideq*0]
.v_w4_loop:
    vpbroadcastd         m1, [srcq+strideq*2]
    vpbroadcastd        xm2, [srcq+strideq*1]
    vpbroadcastd         m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd             m1, m0, 0x05     ; 0 2 2 2
    vpbroadcastd         m0, [srcq+strideq*0]
    vpblendd             m3, m2, 0x0f     ; 1 1 3 3
    vpblendd             m2, m1, m0, 0xa0 ; 0 2 2 4
    vpblendd             m1, m3, 0xaa     ; 0 1 2 3
    vpblendd             m2, m3, 0x55     ; 1 2 3 4
    punpcklbw            m1, m2
    pmaddubsw            m1, m6
    mova             [tmpq], m1
    add                tmpq, 32
    sub                  hd, 4
    jg .v_w4_loop
    RET
.v_w8:
    movq                xm0, [srcq+strideq*0]
.v_w8_loop:
    vpbroadcastq         m1, [srcq+strideq*2]
    vpbroadcastq         m2, [srcq+strideq*1]
    vpbroadcastq         m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd             m1, m0, 0x03     ; 0 2 2 2
    vpbroadcastq         m0, [srcq+strideq*0]
    vpblendd             m2, m3, 0xcc     ; 1 3 1 3
    vpblendd             m3, m2, m1, 0xf0 ; 1 3 2 2
    vpblendd             m2, m1, 0x0f     ; 0 2 1 3
    vpblendd             m3, m0, 0xc0     ; 1 3 2 4
    punpcklbw            m1, m2, m3
    punpckhbw            m2, m3
    pmaddubsw            m1, m6
    pmaddubsw            m2, m6
    mova        [tmpq+32*0], m1
    mova        [tmpq+32*1], m2
    add                tmpq, 32*2
    sub                  hd, 4
    jg .v_w8_loop
    RET
.v_w16:
    vbroadcasti128       m0, [srcq+strideq*0]
.v_w16_loop:
    vbroadcasti128       m1, [srcq+strideq*1]
    vbroadcasti128       m2, [srcq+strideq*2]
    vbroadcasti128       m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    shufpd               m4, m0, m2, 0x0c ; 0 2
    vbroadcasti128       m0, [srcq+strideq*0]
    shufpd               m1, m3, 0x0c     ; 1 3
    shufpd               m2, m0, 0x0c     ; 2 4
    punpcklbw            m3, m4, m1
    punpcklbw            m5, m1, m2
    punpckhbw            m4, m1
    punpckhbw            m1, m2
    pmaddubsw            m3, m6
    pmaddubsw            m5, m6
    pmaddubsw            m4, m6
    pmaddubsw            m1, m6
    mova        [tmpq+32*0], m3
    mova        [tmpq+32*1], m5
    mova        [tmpq+32*2], m4
    mova        [tmpq+32*3], m1
    add                tmpq, 32*4
    sub                  hd, 4
    jg .v_w16_loop
    RET
.v_w32:
    vpermq               m0, [srcq+strideq*0], q3120
.v_w32_loop:
    vpermq               m1, [srcq+strideq*1], q3120
    vpermq               m2, [srcq+strideq*2], q3120
    vpermq               m3, [srcq+stride3q ], q3120
    lea                srcq, [srcq+strideq*4]
    punpcklbw            m4, m0, m1
    punpckhbw            m5, m0, m1
    vpermq               m0, [srcq+strideq*0], q3120
    pmaddubsw            m4, m6
    pmaddubsw            m5, m6
    mova        [tmpq+32*0], m4
    mova        [tmpq+32*1], m5
    punpcklbw            m4, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m4, m6
    pmaddubsw            m1, m6
    punpcklbw            m5, m2, m3
    punpckhbw            m2, m3
    pmaddubsw            m5, m6
    pmaddubsw            m2, m6
    mova        [tmpq+32*2], m4
    mova        [tmpq+32*3], m1
    add                tmpq, 32*8
    punpcklbw            m1, m3, m0
    punpckhbw            m3, m0
    pmaddubsw            m1, m6
    pmaddubsw            m3, m6
    mova        [tmpq-32*4], m5
    mova        [tmpq-32*3], m2
    mova        [tmpq-32*2], m1
    mova        [tmpq-32*1], m3
    sub                  hd, 4
    jg .v_w32_loop
    RET
.v_w64:
    vpermq               m0, [srcq+strideq*0+32*0], q3120
    vpermq               m1, [srcq+strideq*0+32*1], q3120
.v_w64_loop:
    vpermq               m2, [srcq+strideq*1+32*0], q3120
    vpermq               m3, [srcq+strideq*1+32*1], q3120
    lea                srcq, [srcq+strideq*2]
    punpcklbw            m4, m0, m2
    punpckhbw            m0, m2
    pmaddubsw            m4, m6
    pmaddubsw            m0, m6
    mova        [tmpq+32*0], m4
    mova        [tmpq+32*1], m0
    punpcklbw            m4, m1, m3
    punpckhbw            m5, m1, m3
    vpermq               m0, [srcq+strideq*0+32*0], q3120
    vpermq               m1, [srcq+strideq*0+32*1], q3120
    pmaddubsw            m4, m6
    pmaddubsw            m5, m6
    mova        [tmpq+32*2], m4
    mova        [tmpq+32*3], m5
    add                tmpq, 32*8
    punpcklbw            m4, m2, m0
    punpckhbw            m2, m0
    punpcklbw            m5, m3, m1
    punpckhbw            m3, m1
    pmaddubsw            m4, m6
    pmaddubsw            m2, m6
    pmaddubsw            m5, m6
    pmaddubsw            m3, m6
    mova        [tmpq-32*4], m4
    mova        [tmpq-32*3], m2
    mova        [tmpq-32*2], m5
    mova        [tmpq-32*1], m3
    sub                  hd, 2
    jg .v_w64_loop
    RET
.v_w128:
    lea                 r6d, [hq+(3<<8)]
    mov                  r3, srcq
    mov                  r5, tmpq
.v_w128_loop0:
    vpermq               m0, [srcq+strideq*0], q3120
.v_w128_loop:
    vpermq               m1, [srcq+strideq*1], q3120
    lea                srcq, [srcq+strideq*2]
    punpcklbw            m2, m0, m1
    punpckhbw            m3, m0, m1
    vpermq               m0, [srcq+strideq*0], q3120
    pmaddubsw            m2, m6
    pmaddubsw            m3, m6
    punpcklbw            m4, m1, m0
    punpckhbw            m1, m0
    pmaddubsw            m4, m6
    pmaddubsw            m1, m6
    mova        [tmpq+32*0], m2
    mova        [tmpq+32*1], m3
    mova        [tmpq+32*8], m4
    mova        [tmpq+32*9], m1
    add                tmpq, 32*16
    sub                  hd, 2
    jg .v_w128_loop
    add                  r3, 32
    add                  r5, 64
    movzx                hd, r6b
    mov                srcq, r3
    mov                tmpq, r5
    sub                 r6d, 1<<8
    jg .v_w128_loop0
    RET
.hv:
    ; (16 * src[x] + (my * (src[x + src_stride] - src[x])) + 8) >> 4
    ; = src[x] + (((my * (src[x + src_stride] - src[x])) + 8) >> 4)
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM       7
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_hv)]
    shl                mxyd, 11
    movd                xm6, mxyd
    vpbroadcastw         m6, xm6
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.hv_w4:
    vbroadcasti128       m4, [bilin_h_shuf4]
    vpbroadcastq         m0, [srcq+strideq*0]
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w4_loop:
    movq                xm1, [srcq+strideq*1]
    movhps              xm1, [srcq+strideq*2]
    movq                xm2, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    movhps              xm2, [srcq+strideq*0]
    vinserti128          m1, xm2, 1
    pshufb               m1, m4
    pmaddubsw            m1, m5        ; 1 2 3 4
    vpblendd             m2, m1, m0, 0xc0
    vpermq               m2, m2, q2103 ; 0 1 2 3
    mova                 m0, m1
    psubw                m1, m2
    pmulhrsw             m1, m6
    paddw                m1, m2
    mova             [tmpq], m1
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
    RET
.hv_w8:
    vbroadcasti128       m0, [srcq+strideq*0]
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w8_loop:
    movu                xm1, [srcq+strideq*1]
    vinserti128          m1, [srcq+strideq*2], 1
    movu                xm2, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vinserti128          m2, [srcq+strideq*0], 1
    pshufb               m1, m4
    pshufb               m2, m4
    pmaddubsw            m1, m5           ; 1 2
    vperm2i128           m3, m0, m1, 0x21 ; 0 1
    pmaddubsw            m0, m2, m5       ; 3 4
    vperm2i128           m2, m1, m0, 0x21 ; 2 3
    psubw                m1, m3
    pmulhrsw             m1, m6
    paddw                m1, m3
    psubw                m3, m0, m2
    pmulhrsw             m3, m6
    paddw                m3, m2
    mova        [tmpq+32*0], m1
    mova        [tmpq+32*1], m3
    add                tmpq, 32*2
    sub                  hd, 4
    jg .hv_w8_loop
    RET
.hv_w16:
    movu                xm0, [srcq+strideq*0+8*0]
    vinserti128          m0, [srcq+strideq*0+8*1], 1
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w16_loop:
    movu                xm1, [srcq+strideq*1+8*0]
    vinserti128          m1, [srcq+strideq*1+8*1], 1
    lea                srcq, [srcq+strideq*2]
    movu                xm2, [srcq+strideq*0+8*0]
    vinserti128          m2, [srcq+strideq*0+8*1], 1
    pshufb               m1, m4
    pshufb               m2, m4
    pmaddubsw            m1, m5
    psubw                m3, m1, m0
    pmulhrsw             m3, m6
    paddw                m3, m0
    pmaddubsw            m0, m2, m5
    psubw                m2, m0, m1
    pmulhrsw             m2, m6
    paddw                m2, m1
    mova        [tmpq+32*0], m3
    mova        [tmpq+32*1], m2
    add                tmpq, 32*2
    sub                  hd, 2
    jg .hv_w16_loop
    RET
.hv_w32:
    movu                xm0, [srcq+8*0]
    vinserti128          m0, [srcq+8*1], 1
    movu                xm1, [srcq+8*2]
    vinserti128          m1, [srcq+8*3], 1
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
.hv_w32_loop:
    add                srcq, strideq
    movu                xm2, [srcq+8*0]
    vinserti128          m2, [srcq+8*1], 1
    pshufb               m2, m4
    pmaddubsw            m2, m5
    psubw                m3, m2, m0
    pmulhrsw             m3, m6
    paddw                m3, m0
    mova                 m0, m2
    movu                xm2, [srcq+8*2]
    vinserti128          m2, [srcq+8*3], 1
    pshufb               m2, m4
    pmaddubsw            m2, m5
    mova        [tmpq+32*0], m3
    psubw                m3, m2, m1
    pmulhrsw             m3, m6
    paddw                m3, m1
    mova                 m1, m2
    mova        [tmpq+32*1], m3
    add                tmpq, 32*2
    dec                  hd
    jg .hv_w32_loop
    RET
.hv_w128:
    lea                 r3d, [hq+(7<<8)]
    mov                 r6d, 256
    jmp .hv_w64_start
.hv_w64:
    lea                 r3d, [hq+(3<<8)]
    mov                 r6d, 128
.hv_w64_start:
%if WIN64
    PUSH                 r7
%endif
    mov                  r5, srcq
    mov                  r7, tmpq
.hv_w64_loop0:
    movu                xm0, [srcq+strideq*0+8*0]
    vinserti128          m0, [srcq+strideq*0+8*1], 1
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w64_loop:
    movu                xm1, [srcq+strideq*1+8*0]
    vinserti128          m1, [srcq+strideq*1+8*1], 1
    lea                srcq, [srcq+strideq*2]
    movu                xm2, [srcq+strideq*0+8*0]
    vinserti128          m2, [srcq+strideq*0+8*1], 1
    pshufb               m1, m4
    pshufb               m2, m4
    pmaddubsw            m1, m5
    psubw                m3, m1, m0
    pmulhrsw             m3, m6
    paddw                m3, m0
    pmaddubsw            m0, m2, m5
    psubw                m2, m0, m1
    pmulhrsw             m2, m6
    paddw                m2, m1
    mova        [tmpq+r6*0], m3
    mova        [tmpq+r6*1], m2
    lea                tmpq, [tmpq+r6*2]
    sub                  hd, 2
    jg .hv_w64_loop
    add                  r5, 16
    add                  r7, 32
    movzx                hd, r3b
    mov                srcq, r5
    mov                tmpq, r7
    sub                 r3d, 1<<8
    jg .hv_w64_loop0
%if WIN64
    POP                  r7
%endif
    RET

; int8_t subpel_filters[5][15][8]
%assign FILTER_REGULAR (0*15 << 16) | 3*15
%assign FILTER_SMOOTH  (1*15 << 16) | 4*15
%assign FILTER_SHARP   (2*15 << 16) | 3*15

%macro FN 4 ; fn, type, type_h, type_v
cglobal %1_%2_8bpc
    mov                 t0d, FILTER_%3
%ifidn %3, %4
    mov                 t1d, t0d
%else
    mov                 t1d, FILTER_%4
%endif
%ifnidn %2, regular ; skip the jump in the last filter
    jmp mangle(private_prefix %+ _%1_8bpc %+ SUFFIX)
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

cglobal put_8tap_8bpc, 4, 9, 0, dst, ds, src, ss, w, h, mx, my, ss3
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r8, [put_avx2]
    movsxd               wq, wm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    movzx                wd, word [r8+wq*2+table_offset(put,)]
    add                  wq, r8
    lea                  r6, [ssq*3]
    lea                  r7, [dsq*3]
%if WIN64
    pop                  r8
%endif
    jmp                  wq
.h:
    test                myd, 0xf00
    jnz .hv
    vpbroadcastd         m5, [pw_34] ; 2 + (8 << 2)
    WIN64_SPILL_XMM      11
    cmp                  wd, 4
    jl .h_w2
    vbroadcasti128       m6, [subpel_h_shufA]
    je .h_w4
    tzcnt                wd, wd
    vbroadcasti128       m7, [subpel_h_shufB]
    vbroadcasti128       m8, [subpel_h_shufC]
    shr                 mxd, 16
    sub                srcq, 3
    movzx                wd, word [r8+wq*2+table_offset(put, _8tap_h)]
    vpbroadcastd         m9, [r8+mxq*8+subpel_filters-put_avx2+0]
    vpbroadcastd        m10, [r8+mxq*8+subpel_filters-put_avx2+4]
    add                  wq, r8
    jmp                  wq
.h_w2:
    movzx               mxd, mxb
    dec                srcq
    mova                xm4, [subpel_h_shuf4]
    vpbroadcastd        xm3, [r8+mxq*8+subpel_filters-put_avx2+2]
.h_w2_loop:
    movq                xm0, [srcq+ssq*0]
    movhps              xm0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              xm0, xm4
    pmaddubsw           xm0, xm3
    phaddw              xm0, xm0
    paddw               xm0, xm5
    psraw               xm0, 6
    packuswb            xm0, xm0
    pextrw     [dstq+dsq*0], xm0, 0
    pextrw     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2_loop
    RET
.h_w4:
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd        xm3, [r8+mxq*8+subpel_filters-put_avx2+2]
.h_w4_loop:
    movq                xm0, [srcq+ssq*0]
    movq                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              xm0, xm6
    pshufb              xm1, xm6
    pmaddubsw           xm0, xm3
    pmaddubsw           xm1, xm3
    phaddw              xm0, xm1
    paddw               xm0, xm5
    psraw               xm0, 6
    packuswb            xm0, xm0
    movd       [dstq+dsq*0], xm0
    pextrd     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4_loop
    RET
.h_w8:
%macro PUT_8TAP_H 4 ; dst/src, tmp[1-3]
    pshufb              m%2, m%1, m7
    pshufb              m%3, m%1, m8
    pshufb              m%1, m6
    pmaddubsw           m%4, m%2, m9
    pmaddubsw           m%2, m10
    pmaddubsw           m%3, m10
    pmaddubsw           m%1, m9
    paddw               m%3, m%4
    paddw               m%1, m%2
    phaddw              m%1, m%3
    paddw               m%1, m5
    psraw               m%1, 6
%endmacro
    movu                xm0, [srcq+ssq*0]
    vinserti128          m0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_H            0, 1, 2, 3
    vextracti128        xm1, m0, 1
    packuswb            xm0, xm1
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    movu                xm0, [srcq+ssq*0+8*0]
    vinserti128          m0, [srcq+ssq*1+8*0], 1
    movu                xm1, [srcq+ssq*0+8*1]
    vinserti128          m1, [srcq+ssq*1+8*1], 1
    PUT_8TAP_H            0, 2, 3, 4
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_H            1, 2, 3, 4
    packuswb             m0, m1
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w16
    RET
.h_w32:
    xor                 r6d, r6d
    jmp .h_start
.h_w64:
    mov                  r6, -32*1
    jmp .h_start
.h_w128:
    mov                  r6, -32*3
.h_start:
    sub                srcq, r6
    sub                dstq, r6
    mov                  r4, r6
.h_loop:
    movu                 m0, [srcq+r6+8*0]
    movu                 m1, [srcq+r6+8*1]
    PUT_8TAP_H            0, 2, 3, 4
    PUT_8TAP_H            1, 2, 3, 4
    packuswb             m0, m1
    mova          [dstq+r6], m0
    add                  r6, 32
    jle .h_loop
    add                srcq, ssq
    add                dstq, dsq
    mov                  r6, r4
    dec                  hd
    jg .h_loop
    RET
.v:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      16
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    tzcnt               r6d, wd
    movzx               r6d, word [r8+r6*2+table_offset(put, _8tap_v)]
    vpbroadcastd         m7, [pw_512]
    lea                 myq, [r8+myq*8+subpel_filters-put_avx2]
    vpbroadcastw         m8, [myq+0]
    vpbroadcastw         m9, [myq+2]
    vpbroadcastw        m10, [myq+4]
    vpbroadcastw        m11, [myq+6]
    add                  r6, r8
    lea                ss3q, [ssq*3]
    sub                srcq, ss3q
    jmp                  r6
.v_w2:
    movd                xm2, [srcq+ssq*0]
    pinsrw              xm2, [srcq+ssq*1], 2
    pinsrw              xm2, [srcq+ssq*2], 4
    add                srcq, ss3q
    pinsrw              xm2, [srcq+ssq*0], 6 ; 0 1 2 3
    movd                xm3, [srcq+ssq*1]
    vpbroadcastd        xm1, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastd        xm0, [srcq+ssq*0]
    vpblendd            xm3, xm1, 0x02       ; 4 5
    vpblendd            xm1, xm0, 0x02       ; 5 6
    palignr             xm4, xm3, xm2, 4     ; 1 2 3 4
    punpcklbw           xm3, xm1             ; 45 56
    punpcklbw           xm1, xm2, xm4        ; 01 12
    punpckhbw           xm2, xm4             ; 23 34
.v_w2_loop:
    pmaddubsw           xm5, xm1, xm8        ; a0 b0
    mova                xm1, xm2
    pmaddubsw           xm2, xm9             ; a1 b1
    paddw               xm5, xm2
    mova                xm2, xm3
    pmaddubsw           xm3, xm10            ; a2 b2
    paddw               xm5, xm3
    vpbroadcastd        xm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd            xm3, xm0, xm4, 0x02  ; 6 7
    vpbroadcastd        xm0, [srcq+ssq*0]
    vpblendd            xm4, xm0, 0x02       ; 7 8
    punpcklbw           xm3, xm4             ; 67 78
    pmaddubsw           xm4, xm3, xm11       ; a3 b3
    paddw               xm5, xm4
    pmulhrsw            xm5, xm7
    packuswb            xm5, xm5
    pextrw     [dstq+dsq*0], xm5, 0
    pextrw     [dstq+dsq*1], xm5, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movd                xm2, [srcq+ssq*0]
    pinsrd              xm2, [srcq+ssq*1], 1
    pinsrd              xm2, [srcq+ssq*2], 2
    add                srcq, ss3q
    pinsrd              xm2, [srcq+ssq*0], 3 ; 0 1 2 3
    movd                xm3, [srcq+ssq*1]
    vpbroadcastd        xm1, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastd        xm0, [srcq+ssq*0]
    vpblendd            xm3, xm1, 0x02       ; 4 5
    vpblendd            xm1, xm0, 0x02       ; 5 6
    palignr             xm4, xm3, xm2, 4     ; 1 2 3 4
    punpcklbw           xm3, xm1             ; 45 56
    punpcklbw           xm1, xm2, xm4        ; 01 12
    punpckhbw           xm2, xm4             ; 23 34
.v_w4_loop:
    pmaddubsw           xm5, xm1, xm8        ; a0 b0
    mova                xm1, xm2
    pmaddubsw           xm2, xm9             ; a1 b1
    paddw               xm5, xm2
    mova                xm2, xm3
    pmaddubsw           xm3, xm10            ; a2 b2
    paddw               xm5, xm3
    vpbroadcastd        xm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd            xm3, xm0, xm4, 0x02  ; 6 7
    vpbroadcastd        xm0, [srcq+ssq*0]
    vpblendd            xm4, xm0, 0x02       ; 7 8
    punpcklbw           xm3, xm4             ; 67 78
    pmaddubsw           xm4, xm3, xm11       ; a3 b3
    paddw               xm5, xm4
    pmulhrsw            xm5, xm7
    packuswb            xm5, xm5
    movd       [dstq+dsq*0], xm5
    pextrd     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movq                xm1, [srcq+ssq*0]
    vpbroadcastq         m4, [srcq+ssq*1]
    vpbroadcastq         m2, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq         m5, [srcq+ssq*0]
    vpbroadcastq         m3, [srcq+ssq*1]
    vpbroadcastq         m6, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq         m0, [srcq+ssq*0]
    vpblendd             m1, m4, 0x30
    vpblendd             m4, m2, 0x30
    punpcklbw            m1, m4      ; 01 12
    vpblendd             m2, m5, 0x30
    vpblendd             m5, m3, 0x30
    punpcklbw            m2, m5      ; 23 34
    vpblendd             m3, m6, 0x30
    vpblendd             m6, m0, 0x30
    punpcklbw            m3, m6      ; 45 56
.v_w8_loop:
    vpbroadcastq         m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw            m5, m1, m8  ; a0 b0
    mova                 m1, m2
    pmaddubsw            m2, m9      ; a1 b1
    paddw                m5, m2
    mova                 m2, m3
    pmaddubsw            m3, m10     ; a2 b2
    paddw                m5, m3
    vpblendd             m3, m0, m4, 0x30
    vpbroadcastq         m0, [srcq+ssq*0]
    vpblendd             m4, m0, 0x30
    punpcklbw            m3, m4      ; 67 78
    pmaddubsw            m4, m3, m11 ; a3 b3
    paddw                m5, m4
    pmulhrsw             m5, m7
    vextracti128        xm4, m5, 1
    packuswb            xm5, xm4
    movq       [dstq+dsq*0], xm5
    movhps     [dstq+dsq*1], xm5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w16:
.v_w32:
.v_w64:
.v_w128:
    lea                 r6d, [wq*8-128]
    mov                  r4, srcq
    mov                  r7, dstq
    lea                 r6d, [hq+r6*2]
.v_w16_loop0:
    vbroadcasti128       m4, [srcq+ssq*0]
    vbroadcasti128       m5, [srcq+ssq*1]
    vbroadcasti128       m6, [srcq+ssq*2]
    add                srcq, ss3q
    vbroadcasti128       m0, [srcq+ssq*0]
    vbroadcasti128       m1, [srcq+ssq*1]
    vbroadcasti128       m2, [srcq+ssq*2]
    add                srcq, ss3q
    vbroadcasti128       m3, [srcq+ssq*0]
    shufpd               m4, m0, 0x0c
    shufpd               m5, m1, 0x0c
    punpcklbw            m1, m4, m5 ; 01
    punpckhbw            m4, m5     ; 34
    shufpd               m6, m2, 0x0c
    punpcklbw            m2, m5, m6 ; 12
    punpckhbw            m5, m6     ; 45
    shufpd               m0, m3, 0x0c
    punpcklbw            m3, m6, m0 ; 23
    punpckhbw            m6, m0     ; 56
.v_w16_loop:
    vbroadcasti128      m12, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vbroadcasti128      m13, [srcq+ssq*0]
    pmaddubsw           m14, m1, m8  ; a0
    pmaddubsw           m15, m2, m8  ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddubsw            m3, m9      ; a1
    pmaddubsw            m4, m9      ; b1
    paddw               m14, m3
    paddw               m15, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddubsw            m5, m10     ; a2
    pmaddubsw            m6, m10     ; b2
    paddw               m14, m5
    paddw               m15, m6
    shufpd               m6, m0, m12, 0x0d
    shufpd               m0, m12, m13, 0x0c
    punpcklbw            m5, m6, m0  ; 67
    punpckhbw            m6, m0      ; 78
    pmaddubsw           m12, m5, m11 ; a3
    pmaddubsw           m13, m6, m11 ; b3
    paddw               m14, m12
    paddw               m15, m13
    pmulhrsw            m14, m7
    pmulhrsw            m15, m7
    packuswb            m14, m15
    vpermq              m14, m14, q3120
    mova         [dstq+dsq*0], xm14
    vextracti128 [dstq+dsq*1], m14, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    add                  r4, 16
    add                  r7, 16
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
    jg .v_w16_loop0
    RET
.hv:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      16
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd         m7, [r8+mxq*8+subpel_filters-put_avx2+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    vpbroadcastq         m0, [r8+myq*8+subpel_filters-put_avx2]
    lea                ss3q, [ssq*3]
    sub                srcq, ss3q
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    vpbroadcastd         m8, [pw_8192]
    vpbroadcastd         m9, [pd_512]
    pshufd              m10, m0, q0000
    pshufd              m11, m0, q1111
    pshufd              m12, m0, q2222
    pshufd              m13, m0, q3333
    cmp                  wd, 4
    je .hv_w4
    vbroadcasti128       m6, [subpel_h_shuf4]
    movq                xm2, [srcq+ssq*0]
    movhps              xm2, [srcq+ssq*1]
    movq                xm0, [srcq+ssq*2]
    add                srcq, ss3q
    movhps              xm0, [srcq+ssq*0]
    vpbroadcastq         m3, [srcq+ssq*1]
    vpbroadcastq         m4, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq         m1, [srcq+ssq*0]
    vpblendd             m2, m3, 0x30
    vpblendd             m0, m1, 0x30
    vpblendd             m2, m4, 0xc0
    pshufb               m2, m6
    pshufb               m0, m6
    pmaddubsw            m2, m7
    pmaddubsw            m0, m7
    phaddw               m2, m0
    pmulhrsw             m2, m8
    vextracti128        xm3, m2, 1
    palignr             xm4, xm3, xm2, 4
    punpcklwd           xm1, xm2, xm4  ; 01 12
    punpckhwd           xm2, xm4       ; 23 34
    pshufd              xm0, xm3, q2121
    punpcklwd           xm3, xm0       ; 45 56
.hv_w2_loop:
    movq                xm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps              xm4, [srcq+ssq*0]
    pshufb              xm4, xm6
    pmaddubsw           xm4, xm7
    pmaddwd             xm5, xm1, xm10 ; a0 b0
    mova                xm1, xm2
    pmaddwd             xm2, xm11      ; a1 b1
    paddd               xm5, xm2
    mova                xm2, xm3
    pmaddwd             xm3, xm12      ; a2 b2
    phaddw              xm4, xm4
    pmulhrsw            xm4, xm8
    paddd               xm5, xm3
    palignr             xm3, xm4, xm0, 12
    mova                xm0, xm4
    punpcklwd           xm3, xm0       ; 67 78
    pmaddwd             xm4, xm3, xm13 ; a3 b3
    paddd               xm5, xm9
    paddd               xm5, xm4
    psrad               xm5, 10
    packssdw            xm5, xm5
    packuswb            xm5, xm5
    pextrw     [dstq+dsq*0], xm5, 0
    pextrw     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    mova                 m6, [subpel_h_shuf4]
    vpbroadcastq         m2, [srcq+ssq*0]
    vpbroadcastq         m4, [srcq+ssq*1]
    vpbroadcastq         m0, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq         m5, [srcq+ssq*0]
    vpbroadcastq         m3, [srcq+ssq*1]
    vpblendd             m2, m4, 0xcc ; 0 1
    vpbroadcastq         m4, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq         m1, [srcq+ssq*0]
    vpblendd             m0, m5, 0xcc ; 2 3
    vpblendd             m3, m4, 0xcc ; 4 5
    pshufb               m2, m6
    pshufb               m0, m6
    pshufb               m3, m6
    pshufb               m1, m6
    pmaddubsw            m2, m7
    pmaddubsw            m0, m7
    pmaddubsw            m3, m7
    pmaddubsw            m1, m7
    phaddw               m2, m0
    phaddw               m3, m1
    pmulhrsw             m2, m8
    pmulhrsw             m3, m8
    palignr              m4, m3, m2, 4
    punpcklwd            m1, m2, m4   ; 01 12
    punpckhwd            m2, m4       ; 23 34
    pshufd               m0, m3, q2121
    punpcklwd            m3, m0       ; 45 56
.hv_w4_loop:
    vpbroadcastq         m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddwd              m5, m1, m10  ; a0 b0
    mova                 m1, m2
    pmaddwd              m2, m11      ; a1 b1
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, m12      ; a2 b2
    paddd                m5, m3
    vpbroadcastq         m3, [srcq+ssq*0]
    vpblendd             m4, m3, 0xcc ; 7 8
    pshufb               m4, m6
    pmaddubsw            m4, m7
    phaddw               m4, m4
    pmulhrsw             m4, m8
    palignr              m3, m4, m0, 12
    mova                 m0, m4
    punpcklwd            m3, m0       ; 67 78
    pmaddwd              m4, m3, m13  ; a3 b3
    paddd                m5, m9
    paddd                m5, m4
    psrad                m5, 10
    vextracti128        xm4, m5, 1
    packssdw            xm5, xm4
    packuswb            xm5, xm5
    pshuflw             xm5, xm5, q3120
    movd       [dstq+dsq*0], xm5
    pextrd     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    shr                 mxd, 16
    sub                srcq, 3
    vpbroadcastd        m10, [r8+mxq*8+subpel_filters-put_avx2+0]
    vpbroadcastd        m11, [r8+mxq*8+subpel_filters-put_avx2+4]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    vpbroadcastq         m0, [r8+myq*8+subpel_filters-put_avx2]
    lea                ss3q, [ssq*3]
    sub                srcq, ss3q
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    pshufd              m12, m0, q0000
    pshufd              m13, m0, q1111
    pshufd              m14, m0, q2222
    pshufd              m15, m0, q3333
    lea                 r6d, [wq*8-64]
    mov                  r4, srcq
    mov                  r7, dstq
    lea                 r6d, [hq+r6*4]
.hv_w8_loop0:
    vbroadcasti128       m7, [subpel_h_shufA]
    movu                xm4, [srcq+ssq*0]
    vbroadcasti128       m8, [subpel_h_shufB]
    movu                xm5, [srcq+ssq*1]
    vbroadcasti128       m9, [subpel_h_shufC]
    movu                xm6, [srcq+ssq*2]
    add                srcq, ss3q
    vbroadcasti128       m0, [srcq+ssq*0]
    vpblendd             m4, m0, 0xf0        ; 0 3
    vinserti128          m5, [srcq+ssq*1], 1 ; 1 4
    vinserti128          m6, [srcq+ssq*2], 1 ; 2 5
    add                srcq, ss3q
    vinserti128          m0, [srcq+ssq*0], 1 ; 3 6
%macro HV_H_W8 4-7 ; src/dst, tmp[1-3], shuf[1-3]
    pshufb               %3, %1, %6
    pshufb               %4, %1, %7
    pshufb               %1, %5
    pmaddubsw            %2, %3, m10
    pmaddubsw            %4, m11
    pmaddubsw            %3, m11
    pmaddubsw            %1, m10
    paddw                %2, %4
    paddw                %1, %3
    phaddw               %1, %2
%endmacro
    HV_H_W8              m4, m1, m2, m3, m7, m8, m9
    HV_H_W8              m5, m1, m2, m3, m7, m8, m9
    HV_H_W8              m6, m1, m2, m3, m7, m8, m9
    HV_H_W8              m0, m1, m2, m3, m7, m8, m9
    vpbroadcastd         m7, [pw_8192]
    vpermq               m4, m4, q3120
    vpermq               m5, m5, q3120
    vpermq               m6, m6, q3120
    pmulhrsw             m0, m7
    pmulhrsw             m4, m7
    pmulhrsw             m5, m7
    pmulhrsw             m6, m7
    vpermq               m7, m0, q3120
    punpcklwd            m1, m4, m5  ; 01
    punpckhwd            m4, m5      ; 34
    punpcklwd            m2, m5, m6  ; 12
    punpckhwd            m5, m6      ; 45
    punpcklwd            m3, m6, m7  ; 23
    punpckhwd            m6, m7      ; 56
.hv_w8_loop:
    vextracti128        r6m, m0, 1 ; not enough registers
    movu                xm0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti128          m0, [srcq+ssq*0], 1 ; 7 8
    pmaddwd              m8, m1, m12 ; a0
    pmaddwd              m9, m2, m12 ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddwd              m3, m13     ; a1
    pmaddwd              m4, m13     ; b1
    paddd                m8, m3
    paddd                m9, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddwd              m5, m14     ; a2
    pmaddwd              m6, m14     ; b2
    paddd                m8, m5
    paddd                m9, m6
    vbroadcasti128       m6, [subpel_h_shufB]
    vbroadcasti128       m7, [subpel_h_shufC]
    vbroadcasti128       m5, [subpel_h_shufA]
    HV_H_W8              m0, m5, m6, m7, m5, m6, m7
    vpbroadcastd         m5, [pw_8192]
    vpbroadcastd         m7, [pd_512]
    vbroadcasti128       m6, r6m
    pmulhrsw             m0, m5
    paddd                m8, m7
    paddd                m9, m7
    vpermq               m7, m0, q3120    ; 7 8
    shufpd               m6, m6, m7, 0x04 ; 6 7
    punpcklwd            m5, m6, m7  ; 67
    punpckhwd            m6, m7      ; 78
    pmaddwd              m7, m5, m15 ; a3
    paddd                m8, m7
    pmaddwd              m7, m6, m15 ; b3
    paddd                m7, m9
    psrad                m8, 10
    psrad                m7, 10
    packssdw             m8, m7
    vextracti128        xm7, m8, 1
    packuswb            xm8, xm7
    pshufd              xm7, xm8, q3120
    movq       [dstq+dsq*0], xm7
    movhps     [dstq+dsq*1], xm7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    add                  r4, 8
    add                  r7, 8
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
    jg .hv_w8_loop0
    RET

%macro PREP_8TAP_H 0
    pshufb               m1, m0, m5
    pshufb               m2, m0, m6
    pshufb               m3, m0, m7
    pmaddubsw            m1, m8
    pmaddubsw            m0, m2, m8
    pmaddubsw            m2, m9
    pmaddubsw            m3, m9
    paddw                m1, m2
    paddw                m0, m3
    phaddw               m0, m1, m0
    pmulhrsw             m0, m4
%endmacro

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

cglobal prep_8tap_8bpc, 3, 8, 0, tmp, src, stride, w, h, mx, my, stride3
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r7, [prep%+SUFFIX]
    movsxd               wq, wm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    movzx                wd, word [r7+wq*2+table_offset(prep,)]
    add                  wq, r7
    lea                  r6, [strideq*3]
%if WIN64
    pop                  r7
%endif
    jmp                  wq
.h:
    test                myd, 0xf00
    jnz .hv
    vpbroadcastd         m4, [pw_8192]
    vbroadcasti128       m5, [subpel_h_shufA]
    WIN64_SPILL_XMM      10
    cmp                  wd, 4
    je .h_w4
    tzcnt                wd, wd
    vbroadcasti128       m6, [subpel_h_shufB]
    vbroadcasti128       m7, [subpel_h_shufC]
    shr                 mxd, 16
    sub                srcq, 3
    movzx                wd, word [r7+wq*2+table_offset(prep, _8tap_h)]
    vpbroadcastd         m8, [r7+mxq*8+subpel_filters-prep%+SUFFIX+0]
    vpbroadcastd         m9, [r7+mxq*8+subpel_filters-prep%+SUFFIX+4]
    add                  wq, r7
    jmp                  wq
.h_w4:
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd         m6, [r7+mxq*8+subpel_filters-prep%+SUFFIX+2]
    lea            stride3q, [strideq*3]
.h_w4_loop:
    movq                xm0, [srcq+strideq*0]
    vpbroadcastq         m2, [srcq+strideq*2]
    movq                xm1, [srcq+strideq*1]
    vpblendd             m0, m2, 0xf0
    vpbroadcastq         m2, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd             m1, m2, 0xf0
    pshufb               m0, m5
    pshufb               m1, m5
    pmaddubsw            m0, m6
    pmaddubsw            m1, m6
    phaddw               m0, m1
    pmulhrsw             m0, m4
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h_w8:
    movu                xm0, [srcq+strideq*0]
    vinserti128          m0, [srcq+strideq*1], 1
    lea                srcq, [srcq+strideq*2]
    PREP_8TAP_H
    mova             [tmpq], m0
    add                tmpq, 32
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    movu                xm0, [srcq+strideq*0+8*0]
    vinserti128          m0, [srcq+strideq*0+8*1], 1
    PREP_8TAP_H
    mova        [tmpq+32*0], m0
    movu                xm0, [srcq+strideq*1+8*0]
    vinserti128          m0, [srcq+strideq*1+8*1], 1
    lea                srcq, [srcq+strideq*2]
    PREP_8TAP_H
    mova        [tmpq+32*1], m0
    add                tmpq, 32*2
    sub                  hd, 2
    jg .h_w16
    RET
.h_w32:
    xor                 r6d, r6d
    jmp .h_start
.h_w64:
    mov                  r6, -32*1
    jmp .h_start
.h_w128:
    mov                  r6, -32*3
.h_start:
    sub                srcq, r6
    mov                  r5, r6
.h_loop:
    movu                xm0, [srcq+r6+8*0]
    vinserti128          m0, [srcq+r6+8*1], 1
    PREP_8TAP_H
    mova        [tmpq+32*0], m0
    movu                xm0, [srcq+r6+8*2]
    vinserti128          m0, [srcq+r6+8*3], 1
    PREP_8TAP_H
    mova        [tmpq+32*1], m0
    add                tmpq, 32*2
    add                  r6, 32
    jle .h_loop
    add                srcq, strideq
    mov                  r6, r5
    dec                  hd
    jg .h_loop
    RET
.v:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      16
    movzx               mxd, myb ; Select 4-tap/8-tap filter multipliers.
    shr                 myd, 16  ; Note that the code is 8-tap only, having
    cmp                  hd, 4   ; a separate 4-tap code path for (4|8|16)x4
    cmove               myd, mxd ; had a negligible effect on performance.
    ; TODO: Would a 6-tap code path be worth it?
    lea                 myq, [r7+myq*8+subpel_filters-prep%+SUFFIX]
    lea            stride3q, [strideq*3]
    sub                srcq, stride3q
    vpbroadcastd         m7, [pw_8192]
    vpbroadcastw         m8, [myq+0]
    vpbroadcastw         m9, [myq+2]
    vpbroadcastw        m10, [myq+4]
    vpbroadcastw        m11, [myq+6]
    cmp                  wd, 8
    jg .v_w16
    je .v_w8
.v_w4:
    movd                xm0, [srcq+strideq*0]
    vpbroadcastd         m1, [srcq+strideq*2]
    vpbroadcastd        xm2, [srcq+strideq*1]
    add                srcq, stride3q
    vpbroadcastd         m3, [srcq+strideq*0]
    vpblendd             m1, m0, 0x01     ; 0 2 2 _   2 _ _ _
    vpblendd             m3, m2, 0x03     ; 1 1 3 3   3 3 _ _
    vpbroadcastd         m0, [srcq+strideq*1]
    vpbroadcastd         m2, [srcq+strideq*2]
    vpblendd             m1, m0, 0x68     ; 0 2 2 4   2 4 4 _
    vpbroadcastd         m0, [srcq+stride3q ]
    vbroadcasti128       m5, [deint_shuf4]
    vpblendd             m3, m2, 0xc0     ; 1 1 3 3   3 3 5 5
    vpblendd             m2, m3, m1, 0x55 ; 0 1 2 3   2 3 4 5
    vpblendd             m3, m1, 0xaa     ; 1 2 3 4   3 4 5 _
    punpcklbw            m1, m2, m3       ; 01  12    23  34
    vpblendd             m3, m0, 0x80     ; 1 2 3 4   3 4 5 6
    punpckhbw            m2, m3           ; 23  34    45  56
.v_w4_loop:
    lea                srcq, [srcq+strideq*4]
    pinsrd              xm0, [srcq+strideq*0], 1
    vpbroadcastd         m3, [srcq+strideq*1]
    vpbroadcastd         m4, [srcq+strideq*2]
    vpblendd             m3, m0, 0x03     ; 6 7 8 _   8 _ _ _
    vpbroadcastd         m0, [srcq+stride3q ]
    vpblendd             m3, m4, 0x20     ; 6 7 8 _   8 9 _ _
    vpblendd             m3, m0, 0x40     ; 6 7 8 _   8 9 a _
    pshufb               m3, m5           ; 67  78    89  9a
    pmaddubsw            m4, m1, m8
    vperm2i128           m1, m2, m3, 0x21 ; 45  56    67  78
    pmaddubsw            m2, m9
    paddw                m4, m2
    mova                 m2, m3
    pmaddubsw            m3, m11
    paddw                m3, m4
    pmaddubsw            m4, m1, m10
    paddw                m3, m4
    pmulhrsw             m3, m7
    mova             [tmpq], m3
    add                tmpq, 32
    sub                  hd, 4
    jg .v_w4_loop
    RET
.v_w8:
    movq                xm1, [srcq+strideq*0]
    vpbroadcastq         m4, [srcq+strideq*1]
    vpbroadcastq         m2, [srcq+strideq*2]
    vpbroadcastq         m5, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpbroadcastq         m3, [srcq+strideq*0]
    vpbroadcastq         m6, [srcq+strideq*1]
    vpbroadcastq         m0, [srcq+strideq*2]
    vpblendd             m1, m4, 0x30
    vpblendd             m4, m2, 0x30
    punpcklbw            m1, m4 ; 01 12
    vpblendd             m2, m5, 0x30
    vpblendd             m5, m3, 0x30
    punpcklbw            m2, m5 ; 23 34
    vpblendd             m3, m6, 0x30
    vpblendd             m6, m0, 0x30
    punpcklbw            m3, m6 ; 45 56
.v_w8_loop:
    vpbroadcastq         m4, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    pmaddubsw            m5, m2, m9  ; a1
    pmaddubsw            m6, m2, m8  ; b0
    vpblendd             m2, m0, m4, 0x30
    vpbroadcastq         m0, [srcq+strideq*0]
    vpblendd             m4, m0, 0x30
    punpcklbw            m2, m4      ; 67 78
    pmaddubsw            m1, m8      ; a0
    pmaddubsw            m4, m3, m9  ; b1
    paddw                m5, m1
    mova                 m1, m3
    pmaddubsw            m3, m10     ; a2
    paddw                m6, m4
    paddw                m5, m3
    vpbroadcastq         m4, [srcq+strideq*1]
    vpblendd             m3, m0, m4, 0x30
    vpbroadcastq         m0, [srcq+strideq*2]
    vpblendd             m4, m0, 0x30
    punpcklbw            m3, m4      ; 89 9a
    pmaddubsw            m4, m2, m11 ; a3
    paddw                m5, m4
    pmaddubsw            m4, m2, m10 ; b2
    paddw                m6, m4
    pmaddubsw            m4, m3, m11 ; b3
    paddw                m6, m4
    pmulhrsw             m5, m7
    pmulhrsw             m6, m7
    mova        [tmpq+32*0], m5
    mova        [tmpq+32*1], m6
    add                tmpq, 32*2
    sub                  hd, 4
    jg .v_w8_loop
    RET
.v_w16:
    add                  wd, wd
    mov                  r5, srcq
    mov                  r7, tmpq
    lea                 r6d, [hq+wq*8-256]
.v_w16_loop0:
    vbroadcasti128       m4, [srcq+strideq*0]
    vbroadcasti128       m5, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vbroadcasti128       m0, [srcq+strideq*1]
    vbroadcasti128       m6, [srcq+strideq*0]
    lea                srcq, [srcq+strideq*2]
    vbroadcasti128       m1, [srcq+strideq*0]
    vbroadcasti128       m2, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vbroadcasti128       m3, [srcq+strideq*0]
    shufpd               m4, m4, m0, 0x0c
    shufpd               m5, m5, m1, 0x0c
    punpcklbw            m1, m4, m5 ; 01
    punpckhbw            m4, m5     ; 34
    shufpd               m6, m6, m2, 0x0c
    punpcklbw            m2, m5, m6 ; 12
    punpckhbw            m5, m6     ; 45
    shufpd               m0, m0, m3, 0x0c
    punpcklbw            m3, m6, m0 ; 23
    punpckhbw            m6, m0     ; 56
.v_w16_loop:
    vbroadcasti128      m12, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vbroadcasti128      m13, [srcq+strideq*0]
    pmaddubsw           m14, m1, m8  ; a0
    pmaddubsw           m15, m2, m8  ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddubsw            m3, m9      ; a1
    pmaddubsw            m4, m9      ; b1
    paddw               m14, m3
    paddw               m15, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddubsw            m5, m10     ; a2
    pmaddubsw            m6, m10     ; b2
    paddw               m14, m5
    paddw               m15, m6
    shufpd               m6, m0, m12, 0x0d
    shufpd               m0, m12, m13, 0x0c
    punpcklbw            m5, m6, m0  ; 67
    punpckhbw            m6, m0      ; 78
    pmaddubsw           m12, m5, m11 ; a3
    pmaddubsw           m13, m6, m11 ; b3
    paddw               m14, m12
    paddw               m15, m13
    pmulhrsw            m14, m7
    pmulhrsw            m15, m7
    mova        [tmpq+wq*0], m14
    mova        [tmpq+wq*1], m15
    lea                tmpq, [tmpq+wq*2]
    sub                  hd, 2
    jg .v_w16_loop
    add                  r5, 16
    add                  r7, 32
    movzx                hd, r6b
    mov                srcq, r5
    mov                tmpq, r7
    sub                 r6d, 1<<8
    jg .v_w16_loop0
    RET
.hv:
    %assign stack_offset stack_offset - stack_size_padded
    %assign stack_size_padded 0
    WIN64_SPILL_XMM      16
    cmp                  wd, 4
    je .hv_w4
    shr                 mxd, 16
    sub                srcq, 3
    vpbroadcastd        m10, [r7+mxq*8+subpel_filters-prep%+SUFFIX+0]
    vpbroadcastd        m11, [r7+mxq*8+subpel_filters-prep%+SUFFIX+4]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    vpbroadcastq         m0, [r7+myq*8+subpel_filters-prep%+SUFFIX]
    lea            stride3q, [strideq*3]
    sub                srcq, stride3q
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    pshufd              m12, m0, q0000
    pshufd              m13, m0, q1111
    pshufd              m14, m0, q2222
    pshufd              m15, m0, q3333
    jmp .hv_w8
.hv_w4:
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd         m8, [r7+mxq*8+subpel_filters-prep%+SUFFIX+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    vpbroadcastq         m0, [r7+myq*8+subpel_filters-prep%+SUFFIX]
    lea            stride3q, [strideq*3]
    sub                srcq, stride3q
    mova                 m7, [subpel_h_shuf4]
    pmovzxbd             m9, [deint_shuf4]
    vpbroadcastd        m10, [pw_8192]
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    vpbroadcastd        m11, [pd_32]
    pshufd              m12, m0, q0000
    pshufd              m13, m0, q1111
    pshufd              m14, m0, q2222
    pshufd              m15, m0, q3333
    vpbroadcastq         m2, [srcq+strideq*0]
    vpbroadcastq         m4, [srcq+strideq*1]
    vpbroadcastq         m0, [srcq+strideq*2]
    vpbroadcastq         m5, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpbroadcastq         m3, [srcq+strideq*0]
    vpbroadcastq         m6, [srcq+strideq*1]
    vpbroadcastq         m1, [srcq+strideq*2]
    vpblendd             m2, m4, 0xcc ; 0 1
    vpblendd             m0, m5, 0xcc ; 2 3
    vpblendd             m3, m6, 0xcc ; 4 5
    pshufb               m2, m7 ; 00 01 10 11  02 03 12 13
    pshufb               m0, m7 ; 20 21 30 31  22 23 32 33
    pshufb               m3, m7 ; 40 41 50 51  42 43 52 53
    pshufb               m1, m7 ; 60 61 60 61  62 63 62 63
    pmaddubsw            m2, m8
    pmaddubsw            m0, m8
    pmaddubsw            m3, m8
    pmaddubsw            m1, m8
    phaddw               m2, m0 ; 0a 1a 2a 3a  0b 1b 2b 3b
    phaddw               m3, m1 ; 4a 5a 6a __  4b 5b 6b __
    pmulhrsw             m2, m10
    pmulhrsw             m3, m10
    palignr              m4, m3, m2, 4 ; 1a 2a 3a 4a  1b 2b 3b 4b
    punpcklwd            m1, m2, m4  ; 01 12
    punpckhwd            m2, m4      ; 23 34
    pshufd               m0, m3, q2121
    punpcklwd            m3, m0      ; 45 56
.hv_w4_loop:
    pmaddwd              m5, m1, m12 ; a0 b0
    pmaddwd              m6, m2, m12 ; c0 d0
    pmaddwd              m2, m13     ; a1 b1
    pmaddwd              m4, m3, m13 ; c1 d1
    mova                 m1, m3
    pmaddwd              m3, m14     ; a2 b2
    paddd                m5, m2
    vpbroadcastq         m2, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    paddd                m6, m4
    vpbroadcastq         m4, [srcq+strideq*0]
    paddd                m5, m3
    vpbroadcastq         m3, [srcq+strideq*1]
    vpblendd             m2, m4, 0xcc
    vpbroadcastq         m4, [srcq+strideq*2]
    vpblendd             m3, m4, 0xcc
    pshufb               m2, m7
    pshufb               m3, m7
    pmaddubsw            m2, m8
    pmaddubsw            m3, m8
    phaddw               m2, m3
    pmulhrsw             m2, m10
    palignr              m3, m2, m0, 12
    mova                 m0, m2
    punpcklwd            m2, m3, m0  ; 67 78
    punpckhwd            m3, m0      ; 89 9a
    pmaddwd              m4, m2, m14 ; c2 d2
    paddd                m6, m11
    paddd                m5, m11
    paddd                m6, m4
    pmaddwd              m4, m2, m15 ; a3 b3
    paddd                m5, m4
    pmaddwd              m4, m3, m15 ; c3 d3
    paddd                m6, m4
    psrad                m5, 6
    psrad                m6, 6
    packssdw             m5, m6
    vpermd               m5, m9, m5
    mova             [tmpq], m5
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
    RET
.hv_w8:
    lea                 r6d, [wq*8-64]
    mov                  r5, srcq
    mov                  r7, tmpq
    lea                 r6d, [hq+r6*4]
.hv_w8_loop0:
    vbroadcasti128       m7, [subpel_h_shufA]
    movu                xm4, [srcq+strideq*0]
    vbroadcasti128       m8, [subpel_h_shufB]
    movu                xm5, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vbroadcasti128       m9, [subpel_h_shufC]
    movu                xm6, [srcq+strideq*0]
    vbroadcasti128       m0, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vpblendd             m4, m0, 0xf0            ; 0 3
    vinserti128          m5, [srcq+strideq*0], 1 ; 1 4
    vinserti128          m6, [srcq+strideq*1], 1 ; 2 5
    lea                srcq, [srcq+strideq*2]
    vinserti128          m0, [srcq+strideq*0], 1 ; 3 6
    HV_H_W8              m4, m1, m2, m3, m7, m8, m9
    HV_H_W8              m5, m1, m2, m3, m7, m8, m9
    HV_H_W8              m6, m1, m2, m3, m7, m8, m9
    HV_H_W8              m0, m1, m2, m3, m7, m8, m9
    vpbroadcastd         m7, [pw_8192]
    vpermq               m4, m4, q3120
    vpermq               m5, m5, q3120
    vpermq               m6, m6, q3120
    pmulhrsw             m0, m7
    pmulhrsw             m4, m7
    pmulhrsw             m5, m7
    pmulhrsw             m6, m7
    vpermq               m7, m0, q3120
    punpcklwd            m1, m4, m5  ; 01
    punpckhwd            m4, m5      ; 34
    punpcklwd            m2, m5, m6  ; 12
    punpckhwd            m5, m6      ; 45
    punpcklwd            m3, m6, m7  ; 23
    punpckhwd            m6, m7      ; 56
.hv_w8_loop:
    vextracti128     [tmpq], m0, 1 ; not enough registers
    movu                xm0, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vinserti128          m0, [srcq+strideq*0], 1 ; 7 8
    pmaddwd              m8, m1, m12 ; a0
    pmaddwd              m9, m2, m12 ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddwd              m3, m13     ; a1
    pmaddwd              m4, m13     ; b1
    paddd                m8, m3
    paddd                m9, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddwd              m5, m14     ; a2
    pmaddwd              m6, m14     ; b2
    paddd                m8, m5
    paddd                m9, m6
    vbroadcasti128       m6, [subpel_h_shufB]
    vbroadcasti128       m7, [subpel_h_shufC]
    vbroadcasti128       m5, [subpel_h_shufA]
    HV_H_W8              m0, m5, m6, m7, m5, m6, m7
    vpbroadcastd         m5, [pw_8192]
    vpbroadcastd         m7, [pd_32]
    vbroadcasti128       m6, [tmpq]
    pmulhrsw             m0, m5
    paddd                m8, m7
    paddd                m9, m7
    vpermq               m7, m0, q3120    ; 7 8
    shufpd               m6, m6, m7, 0x04 ; 6 7
    punpcklwd            m5, m6, m7  ; 67
    punpckhwd            m6, m7      ; 78
    pmaddwd              m7, m5, m15 ; a3
    paddd                m8, m7
    pmaddwd              m7, m6, m15 ; b3
    paddd                m7, m9
    psrad                m8, 6
    psrad                m7, 6
    packssdw             m8, m7
    vpermq               m7, m8, q3120
    mova         [tmpq+wq*0], xm7
    vextracti128 [tmpq+wq*2], m7, 1
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jg .hv_w8_loop
    add                  r5, 8
    add                  r7, 16
    movzx                hd, r6b
    mov                srcq, r5
    mov                tmpq, r7
    sub                 r6d, 1<<8
    jg .hv_w8_loop0
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

%macro MC_8TAP_SCALED_H 8 ; dst, tmp[0-6]
    movq               xm%1, [srcq+ r4]
    movq               xm%2, [srcq+ r6]
    movhps             xm%1, [srcq+ r7]
    movhps             xm%2, [srcq+ r9]
    vinserti128         m%1, [srcq+r10], 1
    vinserti128         m%2, [srcq+r11], 1
    vpbroadcastq        m%5, [srcq+r13]
    vpbroadcastq        m%6, [srcq+ rX]
    add                srcq, ssq
    movq               xm%3, [srcq+ r4]
    movq               xm%4, [srcq+ r6]
    movhps             xm%3, [srcq+ r7]
    movhps             xm%4, [srcq+ r9]
    vinserti128         m%3, [srcq+r10], 1
    vinserti128         m%4, [srcq+r11], 1
    vpbroadcastq        m%7, [srcq+r13]
    vpbroadcastq        m%8, [srcq+ rX]
    add                srcq, ssq
    vpblendd            m%1, m%5, 0xc0
    vpblendd            m%2, m%6, 0xc0
    vpblendd            m%3, m%7, 0xc0
    vpblendd            m%4, m%8, 0xc0
    pmaddubsw           m%1, m15
    pmaddubsw           m%2, m10
    pmaddubsw           m%3, m15
    pmaddubsw           m%4, m10
    phaddw              m%1, m%2
    phaddw              m%3, m%4
    phaddw              m%1, m%3
    pmulhrsw            m%1, m12
%endmacro

%macro MC_8TAP_SCALED 1
%ifidn %1, put
 %assign isprep 0
cglobal put_8tap_scaled_8bpc, 4, 14, 16, 128, dst, ds, src, ss, w, h, mx, my, dx, dy
 %xdefine base_reg r12
 %define rndshift 10
%else
 %assign isprep 1
cglobal prep_8tap_scaled_8bpc, 4, 14, 16, 128, tmp, src, ss, w, h, mx, my, dx, dy
 %define tmp_stridem qword [rsp+120]
 %xdefine base_reg r11
 %define rndshift 6
%endif
    lea            base_reg, [%1_8tap_scaled_8bpc_avx2]
%define base base_reg-%1_8tap_scaled_8bpc_avx2
    tzcnt                wd, wm
    vpbroadcastd         m8, dxm
%if isprep && UNIX64
    movd               xm14, mxd
    vpbroadcastd        m14, xm14
    mov                 r5d, t0d
 DECLARE_REG_TMP 5, 7
%else
    vpbroadcastd        m14, mxm
%endif
    mov                 dyd, dym
%ifidn %1, put
 %if WIN64
    mov                 r8d, hm
  DEFINE_ARGS dst, ds, src, ss, w, _, _, my, h, dy, ss3
  %define hm r5m
  %define dxm r8m
 %else
  DEFINE_ARGS dst, ds, src, ss, w, h, _, my, dx, dy, ss3
  %define hm r6m
 %endif
 %define dsm [rsp+112]
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
  %define hm [rsp+112]
 %endif
 MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %define rX r14
 %define rXd r14d
%endif
    vpbroadcastd        m10, [base+pd_0x3ff]
    vpbroadcastd        m12, [base+pw_8192]
%ifidn %1, put
    vpbroadcastd        m13, [base+pd_512]
%else
    vpbroadcastd        m13, [base+pd_32]
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
%ifidn %1, put
.w2:
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd               xm15, t0d
    punpckldq            m8, m9, m8
    paddd               m14, m8 ; mx+dx*[0,1]
    vpbroadcastd        m11, [base+pd_0x4000]
    vpbroadcastd       xm15, xm15
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd              xm15, xm8
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    vbroadcasti128       m5, [base+bdct_lb_dw]
    vbroadcasti128       m6, [base+subpel_s_shuf2]
    vpbroadcastd        m15, [base+subpel_filters+r4*8+2]
    vpbroadcastd         m7, [base+subpel_filters+r6*8+2]
    pcmpeqd              m8, m9
    psrld               m14, 10
    movq                xm0, [srcq+ssq*0]
    movq                xm1, [srcq+ssq*2]
    movhps              xm0, [srcq+ssq*1]
    movhps              xm1, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pshufb              m14, m5
    paddb               m14, m6
    vinserti128          m0, [srcq+ssq*0], 1
    vinserti128          m1, [srcq+ssq*2], 1
    vpbroadcastq         m2, [srcq+ssq*1]
    vpbroadcastq         m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    vpblendd            m15, m7, 0xaa
    vpblendd             m0, m2, 0xc0       ; 0 1  4 5
    vpblendd             m1, m3, 0xc0       ; 2 3  6 7
    pblendvb            m15, m11, m8
    pshufb               m0, m14
    pshufb               m1, m14
    pmaddubsw            m0, m15
    pmaddubsw            m1, m15
    phaddw               m0, m1
    pmulhrsw             m0, m12            ; 0 1 2 3  4 5 6 7
    vextracti128        xm1, m0, 1          ; 4 5 6 7
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
    movq               xm11, r6q
    pmovsxbw           xm11, xm11
    pshufd              xm8, xm11, q0000
    pshufd              xm9, xm11, q1111
    pshufd             xm10, xm11, q2222
    pshufd             xm11, xm11, q3333
    pmaddwd             xm5, xm3, xm8
    pmaddwd             xm6, xm0, xm9
    pmaddwd             xm7, xm2, xm10
    pmaddwd             xm8, xm4, xm11
    paddd               xm5, xm6
    paddd               xm7, xm8
    paddd               xm5, xm13
    paddd               xm5, xm7
    psrad               xm5, 10
    packssdw            xm5, xm5
    packuswb            xm5, xm5
    pextrw           [dstq], xm5, 0
    add                dstq, dsq
    dec                  hd
    jz .ret
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .w2_loop
    movq                xm5, [srcq]
    test                myd, 0x400
    jz .w2_skip_line
    add                srcq, ssq
    shufps              xm3, xm0, q1032     ; 01 12
    shufps              xm0, xm2, q1032     ; 23 34
    shufps              xm2, xm4, q1032     ; 45 56
    pshufb              xm5, xm14
    pmaddubsw           xm5, xm15
    phaddw              xm5, xm5
    pmulhrsw            xm5, xm12
    palignr             xm1, xm5, xm1, 12
    punpcklqdq          xm1, xm1            ; 6 7 6 7
    punpcklwd           xm4, xm1, xm5       ; 67 __
    jmp .w2_loop
.w2_skip_line:
    movhps              xm5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                xm3, xm0            ; 01 12
    mova                xm0, xm2            ; 23 34
    pshufb              xm5, xm14
    pmaddubsw           xm5, xm15
    phaddw              xm5, xm5
    pmulhrsw            xm5, xm12           ; 6 7 6 7
    palignr             xm1, xm5, xm1, 8    ; 4 5 6 7
    pshufd              xm5, xm1, q0321     ; 5 6 7 _
    punpcklwd           xm2, xm1, xm5       ; 45 56
    punpckhwd           xm4, xm1, xm5       ; 67 __
    jmp .w2_loop
%endif
.w4:
    mov                 myd, mym
    vbroadcasti128       m7, [base+rescale_mul]
    movzx               t0d, t0b
    dec                srcq
    movd               xm15, t0d
    pmaddwd              m8, m7
    vpbroadcastd        m11, [base+pd_0x4000]
    vpbroadcastd       xm15, xm15
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m0, m14, m10
    psrld                m0, 6
    paddd              xm15, xm0
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    pextrd             r11d, xm15, 2
    pextrd             r13d, xm15, 3
    movd               xm15, [base+subpel_filters+r4*8+2]
    vbroadcasti128       m5, [base+bdct_lb_dw]
    vpbroadcastq         m6, [base+subpel_s_shuf2]
    pinsrd             xm15, [base+subpel_filters+r6*8+2], 1
    pcmpeqd              m0, m9
    psrld               m14, 10
    movu                xm7, [srcq+ssq*0]
    movu                xm9, [srcq+ssq*1]
    pinsrd             xm15, [base+subpel_filters+r11*8+2], 2
    movu                xm8, [srcq+ssq*2]
    movu               xm10, [srcq+ss3q ]
    pinsrd             xm15, [base+subpel_filters+r13*8+2], 3
    lea                srcq, [srcq+ssq*4]
    pshufb              m14, m5
    paddb               m14, m6
    vinserti128          m7, [srcq+ssq*0], 1
    vinserti128          m9, [srcq+ssq*1], 1
    vinserti128         m15, xm15, 1
    vinserti128          m8, [srcq+ssq*2], 1
    vinserti128         m10, [srcq+ss3q ], 1
    lea                srcq, [srcq+ssq*4]
    pblendvb            m15, m11, m0
    pshufb               m7, m14
    pshufb               m9, m14
    pshufb               m8, m14
    pshufb              m10, m14
    pmaddubsw            m7, m15
    pmaddubsw            m9, m15
    pmaddubsw            m8, m15
    pmaddubsw           m10, m15
    phaddw               m7, m9
    phaddw               m8, m10
    pmulhrsw             m7, m12                ; 0 1  4 5
    pmulhrsw             m8, m12                ; 2 3  6 7
    vextracti128        xm9, m7, 1              ; 4 5
    vextracti128        xm3, m8, 1              ; 6 7
    shufps              xm4, xm7, xm8, q1032    ; 1 2
    shufps              xm5, xm8, xm9, q1032    ; 3 4
    shufps              xm6, xm9, xm3, q1032    ; 5 6
    psrldq             xm11, xm3, 8             ; 7 _
    punpcklwd           xm0, xm7, xm4   ; 01
    punpckhwd           xm7, xm4        ; 12
    punpcklwd           xm1, xm8, xm5   ; 23
    punpckhwd           xm8, xm5        ; 34
    punpcklwd           xm2, xm9, xm6   ; 45
    punpckhwd           xm9, xm6        ; 56
    punpcklwd           xm3, xm11       ; 67
    mova         [rsp+0x00], xm7
    mova         [rsp+0x10], xm8
    mova         [rsp+0x20], xm9
.w4_loop:
    and                 myd, 0x3ff
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq               xm10, r6q
    pmovsxbw           xm10, xm10
    pshufd              xm7, xm10, q0000
    pshufd              xm8, xm10, q1111
    pshufd              xm9, xm10, q2222
    pshufd             xm10, xm10, q3333
    pmaddwd             xm4, xm0, xm7
    pmaddwd             xm5, xm1, xm8
    pmaddwd             xm6, xm2, xm9
    pmaddwd             xm7, xm3, xm10
    paddd               xm4, xm5
    paddd               xm6, xm7
    paddd               xm4, xm13
    paddd               xm4, xm6
    psrad               xm4, rndshift
    packssdw            xm4, xm4
%ifidn %1, put
    packuswb            xm4, xm4
    movd             [dstq], xm4
    add                dstq, dsq
%else
    movq             [tmpq], xm4
    add                tmpq, 8
%endif
    dec                  hd
    jz .ret
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .w4_loop
    movu                xm4, [srcq]
    test                myd, 0x400
    jz .w4_skip_line
    mova                xm0, [rsp+0x00]
    mova         [rsp+0x00], xm1
    mova                xm1, [rsp+0x10]
    mova         [rsp+0x10], xm2
    mova                xm2, [rsp+0x20]
    mova         [rsp+0x20], xm3
    pshufb              xm4, xm14
    pmaddubsw           xm4, xm15
    phaddw              xm4, xm4
    pmulhrsw            xm4, xm12
    punpcklwd           xm3, xm11, xm4
    mova               xm11, xm4
    add                srcq, ssq
    jmp .w4_loop
.w4_skip_line:
    movu                xm5, [srcq+ssq*1]
    movu                 m6, [rsp+0x10]
    pshufb              xm4, xm14
    pshufb              xm5, xm14
    pmaddubsw           xm4, xm15
    pmaddubsw           xm5, xm15
    movu         [rsp+0x00], m6
    phaddw              xm4, xm5
    pmulhrsw            xm4, xm12
    punpcklwd           xm9, xm11, xm4
    mova         [rsp+0x20], xm9
    psrldq             xm11, xm4, 8
    mova                xm0, xm1
    mova                xm1, xm2
    mova                xm2, xm3
    punpcklwd           xm3, xm4, xm11
    lea                srcq, [srcq+ssq*2]
    jmp .w4_loop
.w8:
    mov      dword [rsp+48], 1
    movifprep   tmp_stridem, 16
    jmp .w_start
.w16:
    mov      dword [rsp+48], 2
    movifprep   tmp_stridem, 32
    jmp .w_start
.w32:
    mov      dword [rsp+48], 4
    movifprep   tmp_stridem, 64
    jmp .w_start
.w64:
    mov      dword [rsp+48], 8
    movifprep   tmp_stridem, 128
    jmp .w_start
.w128:
    mov      dword [rsp+48], 16
    movifprep   tmp_stridem, 256
.w_start:
%ifidn %1, put
    movifnidn           dsm, dsq
%endif
    shr                 t0d, 16
    sub                srcq, 3
    pmaddwd              m8, [base+rescale_mul]
    movd               xm15, t0d
    mov            [rsp+72], t0d
    mov            [rsp+56], srcq
    mov            [rsp+64], r0q ; dstq / tmpq
%if UNIX64
    mov                  hm, hd
%endif
    shl           dword dxm, 3 ; dx*8
    vpbroadcastd        m15, xm15
    paddd               m14, m8 ; mx+dx*[0-7]
    jmp .hloop
.hloop_prep:
    dec      dword [rsp+48]
    jz .ret
    add      qword [rsp+64], 8*(isprep+1)
    mov                  hd, hm
    vpbroadcastd         m8, dxm
    vpbroadcastd        m10, [base+pd_0x3ff]
    paddd               m14, m8, [rsp+16]
    vpbroadcastd        m15, [rsp+72]
    pxor                 m9, m9
    mov                srcq, [rsp+56]
    mov                 r0q, [rsp+64] ; dstq / tmpq
.hloop:
    vpbroadcastq        m11, [base+pq_0x40000000]
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd               m15, m6
    pcmpeqd              m6, m9
    vextracti128        xm7, m15, 1
    movd                r4d, xm15
    pextrd              r6d, xm15, 2
    pextrd              r7d, xm15, 1
    pextrd              r9d, xm15, 3
    movd               r10d, xm7
    pextrd             r11d, xm7, 2
    pextrd             r13d, xm7, 1
    pextrd              rXd, xm7, 3
    movu           [rsp+16], m14
    movq               xm15, [base+subpel_filters+ r4*8]
    movq               xm10, [base+subpel_filters+ r6*8]
    movhps             xm15, [base+subpel_filters+ r7*8]
    movhps             xm10, [base+subpel_filters+ r9*8]
    vinserti128         m15, [base+subpel_filters+r10*8], 1
    vinserti128         m10, [base+subpel_filters+r11*8], 1
    vpbroadcastq         m9, [base+subpel_filters+r13*8]
    vpbroadcastq         m8, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    vextracti128        xm7, m14, 1
    mova              [rsp], xm14
    movd                r4d, xm14
    pextrd              r6d, xm14, 2
    pextrd              r7d, xm14, 1
    pextrd              r9d, xm14, 3
    movd               r10d, xm7
    pextrd             r11d, xm7, 2
    pextrd             r13d, xm7, 1
    pextrd              rXd, xm7, 3
    pshufd               m5, m6, q1100
    pshufd               m6, m6, q3322
    vpblendd            m15, m9, 0xc0
    vpblendd            m10, m8, 0xc0
    pblendvb            m15, m11, m5
    pblendvb            m10, m11, m6
    vbroadcasti128      m14, [base+subpel_s_shuf8]
    MC_8TAP_SCALED_H 0, 1, 2, 3, 4, 5, 6, 7  ; 0a 1a 0b 1b
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 8  ; 2a 3a 2b 3b
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 7, 8, 9  ; 4a 5a 4b 5b
    MC_8TAP_SCALED_H 3, 4, 5, 6, 7, 8, 9, 11 ; 6a 7a 6b 7b
    mov                 myd, mym
    mov                 dyd, dym
    pshufb               m0, m14    ; 01a 01b
    pshufb               m1, m14    ; 23a 23b
    pshufb               m2, m14    ; 45a 45b
    pshufb               m3, m14    ; 67a 67b
    vbroadcasti128      m14, [base+wswap]
.vloop:
    and                 myd, 0x3ff
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq               xm11, r6q
    punpcklqdq         xm11, xm11
    pmovsxbw            m11, xm11
    pshufd               m8, m11, q0000
    pshufd               m9, m11, q1111
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m9
    pshufd               m8, m11, q2222
    pshufd              m11, m11, q3333
    pmaddwd              m6, m2, m8
    pmaddwd              m7, m3, m11
    paddd                m4, m5
    paddd                m6, m7
    paddd                m4, m13
    paddd                m4, m6
    psrad                m4, rndshift
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
%ifidn %1, put
    packuswb            xm4, xm4
    movq             [dstq], xm4
    add                dstq, dsm
%else
    mova             [tmpq], xm4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .hloop_prep
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .vloop
    test                myd, 0x400
    mov            [rsp+52], myd
    mov                 r4d, [rsp+ 0]
    mov                 r6d, [rsp+ 8]
    mov                 r7d, [rsp+ 4]
    mov                 r9d, [rsp+12]
    jz .skip_line
    vpbroadcastq         m6, [srcq+r13]
    vpbroadcastq         m7, [srcq+ rX]
    movq                xm4, [srcq+ r4]
    movq                xm5, [srcq+ r6]
    movhps              xm4, [srcq+ r7]
    movhps              xm5, [srcq+ r9]
    vinserti128          m4, [srcq+r10], 1
    vinserti128          m5, [srcq+r11], 1
    add                srcq, ssq
    mov                 myd, [rsp+52]
    mov                 dyd, dym
    pshufb               m0, m14
    pshufb               m1, m14
    pshufb               m2, m14
    pshufb               m3, m14
    vpblendd             m4, m6, 0xc0
    vpblendd             m5, m7, 0xc0
    pmaddubsw            m4, m15
    pmaddubsw            m5, m10
    phaddw               m4, m5
    pslld                m5, m4, 16
    paddw                m4, m5
    pmulhrsw             m4, m12
    pblendw              m0, m1, 0xaa
    pblendw              m1, m2, 0xaa
    pblendw              m2, m3, 0xaa
    pblendw              m3, m4, 0xaa
    jmp .vloop
.skip_line:
    mova                 m0, m1
    mova                 m1, m2
    mova                 m2, m3
    vpbroadcastq         m7, [srcq+r13]
    vpbroadcastq         m8, [srcq+ rX]
    movq                xm3, [srcq+ r4]
    movq                xm4, [srcq+ r6]
    movhps              xm3, [srcq+ r7]
    movhps              xm4, [srcq+ r9]
    vinserti128          m3, [srcq+r10], 1
    vinserti128          m4, [srcq+r11], 1
    add                srcq, ssq
    movq                xm5, [srcq+ r4]
    movq                xm6, [srcq+ r6]
    movhps              xm5, [srcq+ r7]
    movhps              xm6, [srcq+ r9]
    vinserti128          m5, [srcq+r10], 1
    vinserti128          m6, [srcq+r11], 1
    vpbroadcastq         m9, [srcq+r13]
    vpbroadcastq        m11, [srcq+ rX]
    add                srcq, ssq
    mov                 myd, [rsp+52]
    mov                 dyd, dym
    vpblendd             m3, m7, 0xc0
    vpblendd             m4, m8, 0xc0
    vpblendd             m5, m9, 0xc0
    vpblendd             m6, m11, 0xc0
    pmaddubsw            m3, m15
    pmaddubsw            m4, m10
    pmaddubsw            m5, m15
    pmaddubsw            m6, m10
    phaddw               m3, m4
    phaddw               m5, m6
    psrld                m4, m3, 16
    pslld                m6, m5, 16
    paddw                m3, m4
    paddw                m5, m6
    pblendw              m3, m5, 0xaa
    pmulhrsw             m3, m12
    jmp .vloop
.dy1:
    movzx                wd, word [base+%1_8tap_scaled_avx2_dy1_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%ifidn %1, put
.dy1_w2:
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd               xm15, t0d
    punpckldq            m8, m9, m8
    paddd               m14, m8 ; mx+dx*[0-1]
    vpbroadcastd        m11, [base+pd_0x4000]
    vpbroadcastd       xm15, xm15
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd              xm15, xm8
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    vbroadcasti128       m5, [base+bdct_lb_dw]
    vbroadcasti128       m6, [base+subpel_s_shuf2]
    vpbroadcastd        m15, [base+subpel_filters+r4*8+2]
    vpbroadcastd         m7, [base+subpel_filters+r6*8+2]
    pcmpeqd              m8, m9
    psrld               m14, 10
    movq                xm0, [srcq+ssq*0]
    movq                xm1, [srcq+ssq*2]
    movhps              xm0, [srcq+ssq*1]
    movhps              xm1, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pshufb              m14, m5
    paddb               m14, m6
    vinserti128          m0, [srcq+ssq*0], 1
    vinserti128          m1, [srcq+ssq*2], 1
    vpbroadcastq         m2, [srcq+ssq*1]
    add                srcq, ss3q
    movq               xm10, r4q
    pmovsxbw           xm10, xm10
    vpblendd            m15, m7, 0xaa
    pblendvb            m15, m11, m8
    pshufd              xm8, xm10, q0000
    pshufd              xm9, xm10, q1111
    pshufd             xm11, xm10, q3333
    pshufd             xm10, xm10, q2222
    vpblendd             m0, m2, 0xc0
    pshufb               m1, m14
    pshufb               m0, m14
    pmaddubsw            m1, m15
    pmaddubsw            m0, m15
    phaddw               m0, m1
    pmulhrsw             m0, m12
    vextracti128        xm1, m0, 1
    palignr             xm2, xm1, xm0, 4
    pshufd              xm4, xm1, q2121
    punpcklwd           xm3, xm0, xm2       ; 01 12
    punpckhwd           xm0, xm2            ; 23 34
    punpcklwd           xm2, xm1, xm4       ; 45 56
.dy1_w2_loop:
    movq                xm1, [srcq+ssq*0]
    movhps              xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddwd             xm5, xm3, xm8
    pmaddwd             xm6, xm0, xm9
    pmaddwd             xm7, xm2, xm10
    mova                xm3, xm0
    mova                xm0, xm2
    paddd               xm5, xm13
    paddd               xm6, xm7
    pshufb              xm1, xm14
    pmaddubsw           xm1, xm15
    phaddw              xm1, xm1
    pmulhrsw            xm1, xm12
    palignr             xm7, xm1, xm4, 12
    punpcklwd           xm2, xm7, xm1     ; 67 78
    pmaddwd             xm7, xm2, xm11
    mova                xm4, xm1
    paddd               xm5, xm6
    paddd               xm5, xm7
    psrad               xm5, rndshift
    packssdw            xm5, xm5
    packuswb            xm5, xm5
    pextrw     [dstq+dsq*0], xm5, 0
    pextrw     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy1_w2_loop
    RET
%endif
.dy1_w4:
    mov                 myd, mym
    vbroadcasti128       m7, [base+rescale_mul]
    movzx               t0d, t0b
    dec                srcq
    movd               xm15, t0d
    pmaddwd              m8, m7
    vpbroadcastd        m11, [base+pd_0x4000]
    vpbroadcastd       xm15, xm15
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd              xm15, xm8
    vpermq               m8, m8, q3120
    movd                r4d, xm15
    pextrd              r6d, xm15, 2
    pextrd             r11d, xm15, 1
    pextrd             r13d, xm15, 3
    movd               xm15, [base+subpel_filters+r4*8+2]
    vpbroadcastd         m7, [base+subpel_filters+r6*8+2]
    movu                xm2, [srcq+ssq*0]
    movu                xm3, [srcq+ssq*2]
    vbroadcasti128       m5, [base+bdct_lb_dw]
    vpbroadcastq         m6, [base+subpel_s_shuf2]
    pcmpeqd              m8, m9
    psrld               m14, 10
    pinsrd             xm15, [base+subpel_filters+r11*8+2], 1
    vpblendd             m7, [base+subpel_filters+r13*8+2-20], 0x20
    vinserti128          m2, [srcq+ssq*1], 1
    vinserti128          m3, [srcq+ss3q ], 1
    lea                srcq, [srcq+ssq*4]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pshufb              m14, m5
    paddb               m14, m6
    movu                xm4, [srcq+ssq*0]
    movu                xm5, [srcq+ssq*2]
    vinserti128          m4, [srcq+ssq*1], 1
    add                srcq, ss3q
    vpblendd            m15, m7, 0x30
    punpcklqdq          m15, m15
    pblendvb            m15, m11, m8
    movq               xm10, r4q
    punpcklqdq         xm10, xm10
    pmovsxbw            m10, xm10
    pshufb               m2, m14
    pshufb               m3, m14
    pshufb               m4, m14
    pshufb              xm5, xm14
    vpermq               m2, m2, q3120
    vpermq               m3, m3, q3120
    vpermq               m4, m4, q3120
    vpermq               m5, m5, q3120
    pshufd               m7, m10, q0000
    pshufd               m8, m10, q1111
    pshufd               m9, m10, q2222
    pshufd              m10, m10, q3333
    pmaddubsw            m2, m15
    pmaddubsw            m3, m15
    pmaddubsw            m4, m15
    pmaddubsw            m5, m15
    phaddw               m2, m3
    phaddw               m4, m5
    pmulhrsw             m2, m12
    pmulhrsw             m4, m12
    palignr              m5, m4, m2, 4
    pshufd               m3, m4, q2121
    punpcklwd            m0, m2, m5     ; 01 12
    punpckhwd            m1, m2, m5     ; 23 34
    punpcklwd            m2, m4, m3     ; 45 56
.dy1_w4_loop:
    movu               xm11, [srcq+ssq*0]
    vinserti128         m11, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pmaddwd              m4, m0, m7
    pmaddwd              m5, m1, m8
    pmaddwd              m6, m2, m9
    mova                 m0, m1
    mova                 m1, m2
    paddd                m4, m13
    paddd                m5, m6
    pshufb              m11, m14
    vpermq              m11, m11, q3120
    pmaddubsw           m11, m15
    phaddw              m11, m11
    pmulhrsw            m11, m12
    palignr              m6, m11, m3, 12
    punpcklwd            m2, m6, m11    ; 67 78
    mova                 m3, m11
    pmaddwd              m6, m2, m10
    paddd                m4, m5
    paddd                m4, m6
    psrad                m4, rndshift
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
%ifidn %1, put
    packuswb            xm4, xm4
    pshuflw             xm4, xm4, q3120
    movd       [dstq+dsq*0], xm4
    pextrd     [dstq+dsq*1], xm4, 1
    lea                dstq, [dstq+dsq*2]
%else
    pshufd              xm4, xm4, q3120
    mova             [tmpq], xm4
    add                tmpq, 16
%endif
    sub                  hd, 2
    jg .dy1_w4_loop
    MC_8TAP_SCALED_RET
.dy1_w8:
    mov      dword [rsp+72], 1
    movifprep   tmp_stridem, 16
    jmp .dy1_w_start
.dy1_w16:
    mov      dword [rsp+72], 2
    movifprep   tmp_stridem, 32
    jmp .dy1_w_start
.dy1_w32:
    mov      dword [rsp+72], 4
    movifprep   tmp_stridem, 64
    jmp .dy1_w_start
.dy1_w64:
    mov      dword [rsp+72], 8
    movifprep   tmp_stridem, 128
    jmp .dy1_w_start
.dy1_w128:
    mov      dword [rsp+72], 16
    movifprep   tmp_stridem, 256
.dy1_w_start:
    mov                 myd, mym
%ifidn %1, put
    movifnidn           dsm, dsq
%endif
    shr                 t0d, 16
    sub                srcq, 3
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pmaddwd              m8, [base+rescale_mul]
    movd               xm15, t0d
    mov            [rsp+76], t0d
    mov            [rsp+80], srcq
    mov            [rsp+88], r0q ; dstq / tmpq
%if UNIX64
    mov                  hm, hd
%endif
    shl           dword dxm, 3 ; dx*8
    vpbroadcastd        m15, xm15
    paddd               m14, m8 ; mx+dx*[0-7]
    movq                xm0, r4q
    pmovsxbw            xm0, xm0
    mova           [rsp+96], xm0
    jmp .dy1_hloop
.dy1_hloop_prep:
    dec      dword [rsp+72]
    jz .ret
    add      qword [rsp+88], 8*(isprep+1)
    mov                  hd, hm
    vpbroadcastd         m8, dxm
    vpbroadcastd        m10, [base+pd_0x3ff]
    paddd               m14, m8, [rsp+32]
    vpbroadcastd        m15, [rsp+76]
    pxor                 m9, m9
    mov                srcq, [rsp+80]
    mov                 r0q, [rsp+88] ; dstq / tmpq
.dy1_hloop:
    vpbroadcastq        m11, [base+pq_0x40000000]
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd               m15, m6
    pcmpeqd              m6, m9
    vextracti128        xm7, m15, 1
    movd                r4d, xm15
    pextrd              r6d, xm15, 2
    pextrd              r7d, xm15, 1
    pextrd              r9d, xm15, 3
    movd               r10d, xm7
    pextrd             r11d, xm7, 2
    pextrd             r13d, xm7, 1
    pextrd              rXd, xm7, 3
    movu           [rsp+32], m14
    movq               xm15, [base+subpel_filters+ r4*8]
    movq               xm10, [base+subpel_filters+ r6*8]
    movhps             xm15, [base+subpel_filters+ r7*8]
    movhps             xm10, [base+subpel_filters+ r9*8]
    vinserti128         m15, [base+subpel_filters+r10*8], 1
    vinserti128         m10, [base+subpel_filters+r11*8], 1
    vpbroadcastq         m9, [base+subpel_filters+r13*8]
    vpbroadcastq         m8, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    vextracti128        xm7, m14, 1
    movq           [rsp+64], xm14
    movd                r4d, xm14
    pextrd              r6d, xm14, 2
    pextrd              r7d, xm14, 1
    pextrd              r9d, xm14, 3
    movd               r10d, xm7
    pextrd             r11d, xm7, 2
    pextrd             r13d, xm7, 1
    pextrd              rXd, xm7, 3
    pshufd               m5, m6, q1100
    pshufd               m6, m6, q3322
    vpblendd            m15, m9, 0xc0
    vpblendd            m10, m8, 0xc0
    pblendvb            m15, m11, m5
    pblendvb            m10, m11, m6
    vbroadcasti128      m14, [base+subpel_s_shuf8]
    MC_8TAP_SCALED_H 0, 1, 2, 3, 4, 5, 6, 7  ; 0a 1a 0b 1b
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 8  ; 2a 3a 2b 3b
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 7, 8, 9  ; 4a 5a 4b 5b
    MC_8TAP_SCALED_H 3, 4, 5, 6, 7, 8, 9, 11 ; 6a 7a 6b 7b
    movu              [rsp], m10
    vpbroadcastd         m8, [rsp+0x60]
    vpbroadcastd         m9, [rsp+0x64]
    vpbroadcastd        m10, [rsp+0x68]
    vpbroadcastd        m11, [rsp+0x6c]
    pshufb               m0, m14    ; 01a 01b
    pshufb               m1, m14    ; 23a 23b
    pshufb               m2, m14    ; 45a 45b
    pshufb               m3, m14    ; 67a 67b
    vbroadcasti128      m14, [base+wswap]
.dy1_vloop:
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m9
    pmaddwd              m6, m2, m10
    pmaddwd              m7, m3, m11
    paddd                m4, m5
    paddd                m6, m7
    paddd                m4, m13
    paddd                m4, m6
    psrad                m4, rndshift
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
%ifidn %1, put
    packuswb            xm4, xm4
    movq             [dstq], xm4
    add                dstq, dsm
%else
    mova             [tmpq], xm4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .dy1_hloop_prep
    movq                xm4, [srcq+ r4]
    movq                xm5, [srcq+ r6]
    movhps              xm4, [srcq+ r7]
    movhps              xm5, [srcq+ r9]
    vinserti128          m4, [srcq+r10], 1
    vinserti128          m5, [srcq+r11], 1
    vpbroadcastq         m6, [srcq+r13]
    vpbroadcastq         m7, [srcq+ rX]
    add                srcq, ssq
    pshufb               m0, m14
    pshufb               m1, m14
    pshufb               m2, m14
    pshufb               m3, m14
    vpblendd             m4, m6, 0xc0
    vpblendd             m5, m7, 0xc0
    pmaddubsw            m4, m15
    pmaddubsw            m5, [rsp]
    phaddw               m4, m5
    pslld                m5, m4, 16
    paddw                m4, m5
    pmulhrsw             m4, m12
    pblendw              m0, m1, 0xaa
    pblendw              m1, m2, 0xaa
    pblendw              m2, m3, 0xaa
    pblendw              m3, m4, 0xaa
    jmp .dy1_vloop
.dy2:
    movzx                wd, word [base+%1_8tap_scaled_avx2_dy2_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%ifidn %1, put
.dy2_w2:
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd               xm15, t0d
    punpckldq            m8, m9, m8
    paddd               m14, m8 ; mx+dx*[0-1]
    vpbroadcastd        m11, [base+pd_0x4000]
    vpbroadcastd       xm15, xm15
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd              xm15, xm8
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    vbroadcasti128       m5, [base+bdct_lb_dw]
    vbroadcasti128       m6, [base+subpel_s_shuf2]
    vpbroadcastd        m15, [base+subpel_filters+r4*8+2]
    vpbroadcastd         m7, [base+subpel_filters+r6*8+2]
    pcmpeqd              m8, m9
    psrld               m14, 10
    movq                xm0, [srcq+ssq*0]
    vpbroadcastq         m2, [srcq+ssq*1]
    movhps              xm0, [srcq+ssq*2]
    vpbroadcastq         m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pshufb              m14, m5
    paddb               m14, m6
    vpblendd            m15, m7, 0xaa
    pblendvb            m15, m11, m8
    movhps              xm1, [srcq+ssq*0]
    vpbroadcastq         m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    vpblendd             m0, m2, 0x30
    vpblendd             m1, m4, 0xc0
    vpblendd             m0, m3, 0xc0
    pshufb               m0, m14
    pshufb               m1, m14
    pmaddubsw            m0, m15
    pmaddubsw            m1, m15
    movq               xm11, r4q
    pmovsxbw           xm11, xm11
    phaddw               m0, m1
    pmulhrsw             m0, m12            ; 0 2 _ 4  1 3 _ 5
    pshufd              xm8, xm11, q0000
    pshufd              xm9, xm11, q1111
    pshufd             xm10, xm11, q2222
    pshufd             xm11, xm11, q3333
    pshufd               m2, m0, q3110      ; 0 2 2 4  1 3 3 5
    vextracti128        xm1, m2, 1
    punpcklwd           xm3, xm2, xm1       ; 01 23
    punpckhwd           xm2, xm1            ; 23 45
.dy2_w2_loop:
    movq                xm6, [srcq+ssq*0]
    vpbroadcastq         m7, [srcq+ssq*1]
    movhps              xm6, [srcq+ssq*2]
    vpbroadcastq         m1, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pmaddwd             xm4, xm3, xm8
    pmaddwd             xm5, xm2, xm9
    vpblendd             m6, m7, 0x30
    vpblendd             m6, m1, 0xc0
    pshufb               m6, m14
    pmaddubsw            m6, m15
    phaddw               m6, m6
    pmulhrsw             m6, m12
    palignr              m0, m6, m0, 8
    pshufd               m2, m0, q3221
    vextracti128        xm1, m2, 1
    punpcklwd           xm3, xm2, xm1       ; 45 67
    punpckhwd           xm2, xm1            ; 67 89
    pmaddwd             xm6, xm3, xm10
    pmaddwd             xm7, xm2, xm11
    paddd               xm4, xm5
    paddd               xm4, xm13
    paddd               xm6, xm7
    paddd               xm4, xm6
    psrad               xm4, rndshift
    packssdw            xm4, xm4
    packuswb            xm4, xm4
    pextrw     [dstq+dsq*0], xm4, 0
    pextrw     [dstq+dsq*1], xm4, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy2_w2_loop
    RET
%endif
.dy2_w4:
    mov                 myd, mym
    vbroadcasti128       m7, [base+rescale_mul]
    movzx               t0d, t0b
    dec                srcq
    movd               xm15, t0d
    pmaddwd              m8, m7
    vpbroadcastd        m11, [base+pd_0x4000]
    vpbroadcastd       xm15, xm15
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd              xm15, xm8
    movd                r4d, xm15
    pextrd              r6d, xm15, 1
    pextrd             r11d, xm15, 2
    pextrd             r13d, xm15, 3
    movd               xm15, [base+subpel_filters+r4*8+2]
    vbroadcasti128       m5, [base+bdct_lb_dw]
    vpbroadcastq         m6, [base+subpel_s_shuf2]
    pinsrd             xm15, [base+subpel_filters+r6*8+2], 1
    pcmpeqd              m8, m9
    psrld               m14, 10
    movu                xm0, [srcq+ssq*0]
    movu                xm2, [srcq+ssq*2]
    pinsrd             xm15, [base+subpel_filters+r11*8+2], 2
    movu                xm1, [srcq+ssq*1]
    movu                xm3, [srcq+ss3q ]
    pinsrd             xm15, [base+subpel_filters+r13*8+2], 3
    lea                srcq, [srcq+ssq*4]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    vinserti128         m15, xm15, 1
    pshufb              m14, m5
    paddb               m14, m6
    vinserti128          m2, [srcq+ssq*0], 1
    vinserti128          m3, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pblendvb            m15, m11, m8
    pshufb              xm0, xm14
    pshufb               m2, m14
    pshufb              xm1, xm14
    pshufb               m3, m14
    pmaddubsw           xm0, xm15
    pmaddubsw            m2, m15
    pmaddubsw           xm1, xm15
    pmaddubsw            m3, m15
    movq               xm11, r4q
    punpcklqdq         xm11, xm11
    pmovsxbw            m11, xm11
    phaddw               m0, m2
    phaddw               m1, m3
    pmulhrsw             m0, m12    ; 0 2  _ 4
    pmulhrsw             m1, m12    ; 1 3  _ 5
    pshufd               m8, m11, q0000
    pshufd               m9, m11, q1111
    pshufd              m10, m11, q2222
    pshufd              m11, m11, q3333
    punpcklwd           xm2, xm0, xm1
    punpckhwd            m1, m0, m1     ; 23 45
    vinserti128          m0, m2, xm1, 1 ; 01 23
.dy2_w4_loop:
    movu                xm6, [srcq+ssq*0]
    movu                xm7, [srcq+ssq*1]
    vinserti128          m6, [srcq+ssq*2], 1
    vinserti128          m7, [srcq+ss3q ], 1
    lea                srcq, [srcq+ssq*4]
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m9
    pshufb               m6, m14
    pshufb               m7, m14
    pmaddubsw            m6, m15
    pmaddubsw            m7, m15
    psrld                m2, m6, 16
    pslld                m3, m7, 16
    paddw                m6, m2
    paddw                m7, m3
    pblendw              m6, m7, 0xaa   ; 67 89
    pmulhrsw             m6, m12
    paddd                m4, m5
    vperm2i128           m0, m1, m6, 0x21 ; 45 67
    mova                 m1, m6
    pmaddwd              m6, m0, m10
    pmaddwd              m7, m1, m11
    paddd                m4, m13
    paddd                m6, m7
    paddd                m4, m6
    psrad                m4, rndshift
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
%ifidn %1, put
    packuswb            xm4, xm4
    movd       [dstq+dsq*0], xm4
    pextrd     [dstq+dsq*1], xm4, 1
    lea                dstq, [dstq+dsq*2]
%else
    mova             [tmpq], xm4
    add                tmpq, 16
%endif
    sub                  hd, 2
    jg .dy2_w4_loop
    MC_8TAP_SCALED_RET
.dy2_w8:
    mov      dword [rsp+40], 1
    movifprep   tmp_stridem, 16
    jmp .dy2_w_start
.dy2_w16:
    mov      dword [rsp+40], 2
    movifprep   tmp_stridem, 32
    jmp .dy2_w_start
.dy2_w32:
    mov      dword [rsp+40], 4
    movifprep   tmp_stridem, 64
    jmp .dy2_w_start
.dy2_w64:
    mov      dword [rsp+40], 8
    movifprep   tmp_stridem, 128
    jmp .dy2_w_start
.dy2_w128:
    mov      dword [rsp+40], 16
    movifprep   tmp_stridem, 256
.dy2_w_start:
    mov                 myd, mym
%ifidn %1, put
    movifnidn           dsm, dsq
%endif
    shr                 t0d, 16
    sub                srcq, 3
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pmaddwd              m8, [base+rescale_mul]
    movd               xm15, t0d
    mov            [rsp+64], t0d
    mov            [rsp+48], srcq
    mov            [rsp+56], r0q ; dstq / tmpq
%if UNIX64
    mov                  hm, hd
%endif
    shl           dword dxm, 3 ; dx*8
    vpbroadcastd        m15, xm15
    paddd               m14, m8 ; mx+dx*[0-7]
    movq                xm0, r4q
    pmovsxbw            xm0, xm0
    mova         [rsp+0x50], xm0
    jmp .dy2_hloop
.dy2_hloop_prep:
    dec      dword [rsp+40]
    jz .ret
    add      qword [rsp+56], 8*(isprep+1)
    mov                  hd, hm
    vpbroadcastd         m8, dxm
    vpbroadcastd        m10, [base+pd_0x3ff]
    paddd               m14, m8, [rsp]
    vpbroadcastd        m15, [rsp+64]
    pxor                 m9, m9
    mov                srcq, [rsp+48]
    mov                 r0q, [rsp+56] ; dstq / tmpq
.dy2_hloop:
    vpbroadcastq        m11, [base+pq_0x40000000]
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd               m15, m6
    pcmpeqd              m6, m9
    vextracti128        xm7, m15, 1
    movd                r4d, xm15
    pextrd              r6d, xm15, 2
    pextrd              r7d, xm15, 1
    pextrd              r9d, xm15, 3
    movd               r10d, xm7
    pextrd             r11d, xm7, 2
    pextrd             r13d, xm7, 1
    pextrd              rXd, xm7, 3
    movu              [rsp], m14
    movq               xm15, [base+subpel_filters+ r4*8]
    movq               xm10, [base+subpel_filters+ r6*8]
    movhps             xm15, [base+subpel_filters+ r7*8]
    movhps             xm10, [base+subpel_filters+ r9*8]
    vinserti128         m15, [base+subpel_filters+r10*8], 1
    vinserti128         m10, [base+subpel_filters+r11*8], 1
    vpbroadcastq         m9, [base+subpel_filters+r13*8]
    vpbroadcastq         m8, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    vextracti128        xm7, m14, 1
    movd                r4d, xm14
    pextrd              r6d, xm14, 2
    pextrd              r7d, xm14, 1
    pextrd              r9d, xm14, 3
    movd               r10d, xm7
    pextrd             r11d, xm7, 2
    pextrd             r13d, xm7, 1
    pextrd              rXd, xm7, 3
    pshufd               m5, m6, q1100
    pshufd               m6, m6, q3322
    vpblendd            m15, m9, 0xc0
    vpblendd            m10, m8, 0xc0
    pblendvb            m15, m11, m5
    pblendvb            m10, m11, m6
    vbroadcasti128      m14, [base+subpel_s_shuf8]
    MC_8TAP_SCALED_H 0, 1, 2, 3, 4, 5, 6, 7  ; 0a 1a 0b 1b
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 8  ; 2a 3a 2b 3b
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 7, 8, 9  ; 4a 5a 4b 5b
    MC_8TAP_SCALED_H 3, 4, 5, 6, 7, 8, 9, 11 ; 6a 7a 6b 7b
    vpbroadcastd         m8, [rsp+0x50]
    vpbroadcastd         m9, [rsp+0x54]
    vpbroadcastd        m11, [rsp+0x58]
    vpbroadcastd         m4, [rsp+0x5c]
    pshufb               m0, m14    ; 01a 01b
    pshufb               m1, m14    ; 23a 23b
    pshufb               m2, m14    ; 45a 45b
    pshufb               m3, m14    ; 67a 67b
    SWAP                m14, m4
.dy2_vloop:
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m9
    pmaddwd              m6, m2, m11
    pmaddwd              m7, m3, m14
    paddd                m4, m5
    paddd                m6, m7
    paddd                m4, m13
    paddd                m4, m6
    psrad                m4, rndshift
    vextracti128        xm5, m4, 1
    packssdw            xm4, xm5
%ifidn %1, put
    packuswb            xm4, xm4
    movq             [dstq], xm4
    add                dstq, dsm
%else
    mova             [tmpq], xm4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .dy2_hloop_prep
    mova                 m0, m1
    mova                 m1, m2
    mova                 m2, m3
    movq                xm3, [srcq+ r4]
    movq                xm4, [srcq+ r6]
    movhps              xm3, [srcq+ r7]
    movhps              xm4, [srcq+ r9]
    vinserti128          m3, [srcq+r10], 1
    vinserti128          m4, [srcq+r11], 1
    vpbroadcastq         m5, [srcq+r13]
    vpbroadcastq         m6, [srcq+ rX]
    add                srcq, ssq
    vpblendd             m3, m5, 0xc0
    vpblendd             m4, m6, 0xc0
    pmaddubsw            m3, m15
    pmaddubsw            m4, m10
    phaddw               m3, m4
    movq                xm4, [srcq+ r4]
    movq                xm5, [srcq+ r6]
    movhps              xm4, [srcq+ r7]
    movhps              xm5, [srcq+ r9]
    vinserti128          m4, [srcq+r10], 1
    vinserti128          m5, [srcq+r11], 1
    vpbroadcastq         m6, [srcq+r13]
    vpbroadcastq         m7, [srcq+ rX]
    add                srcq, ssq
    vpblendd             m4, m6, 0xc0
    vpblendd             m5, m7, 0xc0
    pmaddubsw            m4, m15
    pmaddubsw            m5, m10
    phaddw               m4, m5
    psrld                m5, m3, 16
    pslld                m6, m4, 16
    paddw                m3, m5
    paddw                m4, m6
    pblendw              m3, m4, 0xaa
    pmulhrsw             m3, m12
    jmp .dy2_vloop
.ret:
    MC_8TAP_SCALED_RET 0
%undef isprep
%endmacro

%macro BILIN_SCALED_FN 1
cglobal %1_bilin_scaled_8bpc
    mov                 t0d, (5*15 << 16) | 5*15
    mov                 t1d, t0d
    jmp mangle(private_prefix %+ _%1_8tap_scaled_8bpc %+ SUFFIX)
%endmacro

%if WIN64
DECLARE_REG_TMP 6, 5
%else
DECLARE_REG_TMP 6, 8
%endif

%define PUT_8TAP_SCALED_FN FN put_8tap_scaled,
%define PREP_8TAP_SCALED_FN FN prep_8tap_scaled,

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

%macro WARP_V 5 ; dst, 02, 46, 13, 57
    ; Can be done using gathers, but that's terribly slow on many CPU:s
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
    shr               tmp2d, 10
    shr               tmp1d, 10
    punpcklwd            m8, m0
    movq                xm0, [filterq+tmp2q*8]
    vinserti128          m0, [filterq+tmp1q*8], 1 ; d h
    punpcklwd            m0, m9, m0
    punpckldq            m9, m8, m0
    punpckhdq            m0, m8, m0
    punpcklbw            m8, m11, m9 ; a0 a2 b0 b2 c0 c2 d0 d2 << 8
    punpckhbw            m9, m11, m9 ; a4 a6 b4 b6 c4 c6 d4 d6 << 8
    pmaddwd             m%2, m8
    pmaddwd              m9, m%3
    punpcklbw            m8, m11, m0 ; a1 a3 b1 b3 c1 c3 d1 d3 << 8
    punpckhbw            m0, m11, m0 ; a5 a7 b5 b7 c5 c7 d5 d7 << 8
    pmaddwd              m8, m%4
    pmaddwd              m0, m%5
    paddd               m%2, m9
    paddd                m0, m8
    paddd               m%1, m0, m%2
%endmacro

cglobal warp_affine_8x8t_8bpc, 0, 14, 0, tmp, ts
%if WIN64
    sub                 rsp, 0xa0
%endif
    call mangle(private_prefix %+ _warp_affine_8x8_8bpc_avx2).main
.loop:
    psrad                m7, 13
    psrad                m0, 13
    packssdw             m7, m0
    pmulhrsw             m7, m14 ; (x + (1 << 6)) >> 7
    vpermq               m7, m7, q3120
    mova         [tmpq+tsq*0], xm7
    vextracti128 [tmpq+tsq*2], m7, 1
    dec                 r4d
    jz   mangle(private_prefix %+ _warp_affine_8x8_8bpc_avx2).end
    call mangle(private_prefix %+ _warp_affine_8x8_8bpc_avx2).main2
    lea                tmpq, [tmpq+tsq*4]
    jmp .loop

cglobal warp_affine_8x8_8bpc, 0, 14, 0, dst, ds, src, ss, abcd, mx, tmp2, alpha, \
                                        beta, filter, tmp1, delta, my, gamma
%if WIN64
    sub                 rsp, 0xa0
    %assign xmm_regs_used 16
    %assign stack_size_padded 0xa0
    %assign stack_offset stack_offset+stack_size_padded
%endif
    call .main
    jmp .start
.loop:
    call .main2
    lea                dstq, [dstq+dsq*2]
.start:
    psrad                m7, 18
    psrad                m0, 18
    packusdw             m7, m0
    pavgw                m7, m11 ; (x + (1 << 10)) >> 11
    vextracti128        xm0, m7, 1
    packuswb            xm7, xm0
    pshufd              xm7, xm7, q3120
    movq       [dstq+dsq*0], xm7
    movhps     [dstq+dsq*1], xm7
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
    movaps [rsp+stack_offset+0x10], xmm6
    movaps [rsp+stack_offset+0x20], xmm7
    movaps       [rsp+0x28], xmm8
    movaps       [rsp+0x38], xmm9
    movaps       [rsp+0x48], xmm10
    movaps       [rsp+0x58], xmm11
    movaps       [rsp+0x68], xmm12
    movaps       [rsp+0x78], xmm13
    movaps       [rsp+0x88], xmm14
    movaps       [rsp+0x98], xmm15
%endif
    movsx            alphad, word [abcdq+2*0]
    movsx             betad, word [abcdq+2*1]
    mova                m12, [warp_8x8_shufA]
    mova                m13, [warp_8x8_shufB]
    vpbroadcastd        m14, [pw_8192]
    vpbroadcastd        m15, [pd_32768]
    pxor                m11, m11
    lea             filterq, [mc_warp_filter2]
    lea               tmp1q, [ssq*3+3]
    add                 mxd, 512+(64<<10)
    lea               tmp2d, [alphaq*3]
    sub                srcq, tmp1q    ; src -= src_stride*3 + 3
    sub               betad, tmp2d    ; beta -= alpha*3
    mov                 myd, r7m
    call .h
    psrld                m1, m0, 16
    call .h
    psrld                m4, m0, 16
    call .h
    pblendw              m1, m0, 0xaa ; 02
    call .h
    pblendw              m4, m0, 0xaa ; 13
    call .h
    psrld                m2, m1, 16
    pblendw              m2, m0, 0xaa ; 24
    call .h
    psrld                m5, m4, 16
    pblendw              m5, m0, 0xaa ; 35
    call .h
    psrld                m3, m2, 16
    pblendw              m3, m0, 0xaa ; 46
    movsx            deltad, word [abcdq+2*2]
    movsx            gammad, word [abcdq+2*3]
    add                 myd, 512+(64<<10)
    mov                 r4d, 4
    lea               tmp1d, [deltaq*3]
    sub              gammad, tmp1d    ; gamma -= delta*3
.main2:
    call .h
    psrld                m6, m5, 16
    pblendw              m6, m0, 0xaa ; 57
    WARP_V                7, 1, 3, 4, 6
    call .h
    mova                 m1, m2
    mova                 m2, m3
    psrld                m3, 16
    pblendw              m3, m0, 0xaa ; 68
    WARP_V                0, 4, 6, 1, 3
    mova                 m4, m5
    mova                 m5, m6
    ret
ALIGN function_align
.h:
    lea               tmp1d, [mxq+alphaq*4]
    lea               tmp2d, [mxq+alphaq*1]
    vbroadcasti128      m10, [srcq]
    shr                 mxd, 10
    shr               tmp1d, 10
    movq                xm8, [filterq+mxq  *8]
    vinserti128          m8, [filterq+tmp1q*8], 1
    lea               tmp1d, [tmp2q+alphaq*4]
    lea                 mxd, [tmp2q+alphaq*1]
    shr               tmp2d, 10
    shr               tmp1d, 10
    movq                xm0, [filterq+tmp2q*8]
    vinserti128          m0, [filterq+tmp1q*8], 1
    lea               tmp1d, [mxq+alphaq*4]
    lea               tmp2d, [mxq+alphaq*1]
    shr                 mxd, 10
    shr               tmp1d, 10
    movq                xm9, [filterq+mxq  *8]
    vinserti128          m9, [filterq+tmp1q*8], 1
    lea               tmp1d, [tmp2q+alphaq*4]
    lea                 mxd, [tmp2q+betaq] ; mx += beta
    shr               tmp2d, 10
    shr               tmp1d, 10
    punpcklqdq           m8, m0  ; 0 1   4 5
    movq                xm0, [filterq+tmp2q*8]
    vinserti128          m0, [filterq+tmp1q*8], 1
    punpcklqdq           m9, m0  ; 2 3   6 7
    pshufb               m0, m10, m12
    pmaddubsw            m0, m8
    pshufb              m10, m13
    pmaddubsw           m10, m9
    add                srcq, ssq
    phaddw               m0, m10
    pmaddwd              m0, m14 ; 17-bit intermediate, upshifted by 13
    paddd                m0, m15 ; rounded 14-bit result in upper 16 bits of dword
    ret

%macro BIDIR_FN 1 ; op
    %1                    0
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    vextracti128        xm1, m0, 1
    movd   [dstq          ], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    cmp                  hd, 4
    je .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq          ], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
    cmp                  hd, 8
    je .ret
    %1                    2
    lea                dstq, [dstq+strideq*4]
    vextracti128        xm1, m0, 1
    movd   [dstq          ], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq          ], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
.ret:
    RET
.w8_loop:
    %1_INC_PTR            2
    %1                    0
    lea                dstq, [dstq+strideq*4]
.w8:
    vextracti128        xm1, m0, 1
    movq   [dstq          ], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    sub                  hd, 4
    jg .w8_loop
    RET
.w16_loop:
    %1_INC_PTR            4
    %1                    0
    lea                dstq, [dstq+strideq*4]
.w16:
    vpermq               m0, m0, q3120
    mova         [dstq          ], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    %1                    2
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*2], xm0
    vextracti128 [dstq+stride3q ], m0, 1
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    %1_INC_PTR            4
    %1                    0
    lea                dstq, [dstq+strideq*2]
.w32:
    vpermq               m0, m0, q3120
    mova   [dstq+strideq*0], m0
    %1                    2
    vpermq               m0, m0, q3120
    mova   [dstq+strideq*1], m0
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    %1_INC_PTR            4
    %1                    0
    add                dstq, strideq
.w64:
    vpermq               m0, m0, q3120
    mova             [dstq], m0
    %1                    2
    vpermq               m0, m0, q3120
    mova          [dstq+32], m0
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    %1                    0
    add                dstq, strideq
.w128:
    vpermq               m0, m0, q3120
    mova        [dstq+0*32], m0
    %1                    2
    vpermq               m0, m0, q3120
    mova        [dstq+1*32], m0
    %1_INC_PTR            8
    %1                   -4
    vpermq               m0, m0, q3120
    mova        [dstq+2*32], m0
    %1                   -2
    vpermq               m0, m0, q3120
    mova        [dstq+3*32], m0
    dec                  hd
    jg .w128_loop
    RET
%endmacro

%macro AVG 1 ; src_offset
    mova                 m0, [tmp1q+(%1+0)*32]
    paddw                m0, [tmp2q+(%1+0)*32]
    mova                 m1, [tmp1q+(%1+1)*32]
    paddw                m1, [tmp2q+(%1+1)*32]
    pmulhrsw             m0, m2
    pmulhrsw             m1, m2
    packuswb             m0, m1
%endmacro

%macro AVG_INC_PTR 1
    add               tmp1q, %1*32
    add               tmp2q, %1*32
%endmacro

cglobal avg_8bpc, 4, 7, 3, dst, stride, tmp1, tmp2, w, h, stride3
%define base r6-avg %+ SUFFIX %+ _table
    lea                  r6, [avg %+ SUFFIX %+ _table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, dword [r6+wq*4]
    vpbroadcastd         m2, [base+pw_1024]
    add                  wq, r6
    BIDIR_FN            AVG

%macro W_AVG 1 ; src_offset
    ; (a * weight + b * (16 - weight) + 128) >> 8
    ; = ((a - b) * weight + (b << 4) + 128) >> 8
    ; = ((((a - b) * ((weight-16) << 12)) >> 16) + a + 8) >> 4
    ; = ((((b - a) * (-weight     << 12)) >> 16) + b + 8) >> 4
    mova                 m0,     [tmp1q+(%1+0)*32]
    psubw                m2, m0, [tmp2q+(%1+0)*32]
    mova                 m1,     [tmp1q+(%1+1)*32]
    psubw                m3, m1, [tmp2q+(%1+1)*32]
    pmulhw               m2, m4
    pmulhw               m3, m4
    paddw                m0, m2
    paddw                m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
%endmacro

%define W_AVG_INC_PTR AVG_INC_PTR

cglobal w_avg_8bpc, 4, 7, 6, dst, stride, tmp1, tmp2, w, h, stride3
%define base r6-w_avg %+ SUFFIX %+ _table
    lea                  r6, [w_avg %+ SUFFIX %+ _table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    vpbroadcastw         m4, r6m ; weight
    movsxd               wq, dword [r6+wq*4]
    vpbroadcastd         m5, [base+pw_2048]
    psllw                m4, 12 ; (weight-16) << 12 when interpreted as signed
    add                  wq, r6
    cmp           dword r6m, 7
    jg .weight_gt7
    mov                  r6, tmp1q
    pxor                 m0, m0
    mov               tmp1q, tmp2q
    psubw                m4, m0, m4 ; -weight
    mov               tmp2q, r6
.weight_gt7:
    BIDIR_FN          W_AVG

%macro MASK 1 ; src_offset
    ; (a * m + b * (64 - m) + 512) >> 10
    ; = ((a - b) * m + (b << 6) + 512) >> 10
    ; = ((((b - a) * (-m << 10)) >> 16) + b + 8) >> 4
    vpermq               m3,     [maskq+%1*16], q3120
    mova                 m0,     [tmp2q+(%1+0)*32]
    psubw                m1, m0, [tmp1q+(%1+0)*32]
    psubb                m3, m4, m3
    paddw                m1, m1     ; (b - a) << 1
    paddb                m3, m3
    punpcklbw            m2, m4, m3 ; -m << 9
    pmulhw               m1, m2
    paddw                m0, m1
    mova                 m1,     [tmp2q+(%1+1)*32]
    psubw                m2, m1, [tmp1q+(%1+1)*32]
    paddw                m2, m2
    punpckhbw            m3, m4, m3
    pmulhw               m2, m3
    paddw                m1, m2
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
%endmacro

%macro MASK_INC_PTR 1
    add               maskq, %1*16
    add               tmp2q, %1*32
    add               tmp1q, %1*32
%endmacro

cglobal mask_8bpc, 4, 8, 6, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-mask %+ SUFFIX %+ _table
    lea                  r7, [mask %+ SUFFIX %+ _table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    mov               maskq, maskmp
    movsxd               wq, dword [r7+wq*4]
    vpbroadcastd         m5, [base+pw_2048]
    pxor                 m4, m4
    add                  wq, r7
    BIDIR_FN           MASK

%macro W_MASK 4-5 0 ; dst, mask, tmp_offset[1-2], 4:4:4
    mova                m%1, [tmp1q+32*%3]
    mova                 m1, [tmp2q+32*%3]
    psubw                m1, m%1
    pabsw               m%2, m1
    psubusw             m%2, m6, m%2
    psrlw               m%2, 8 ; 64 - m
    psllw                m2, m%2, 10
    pmulhw               m1, m2
    paddw               m%1, m1
    mova                 m1, [tmp1q+32*%4]
    mova                 m2, [tmp2q+32*%4]
    psubw                m2, m1
    pabsw                m3, m2
    psubusw              m3, m6, m3
    psrlw                m3, 8
%if %5
    packuswb            m%2, m3
    psubb               m%2, m5, m%2
    vpermq              m%2, m%2, q3120
%else
    phaddw              m%2, m3
%endif
    psllw                m3, 10
    pmulhw               m2, m3
    paddw                m1, m2
    pmulhrsw            m%1, m7
    pmulhrsw             m1, m7
    packuswb            m%1, m1
%endmacro

cglobal blend_8bpc, 3, 7, 7, dst, ds, tmp, w, h, mask
%define base r6-blend_avx2_table
    lea                  r6, [blend_avx2_table]
    tzcnt                wd, wm
    movifnidn         maskq, maskmp
    movifnidn            hd, hm
    movsxd               wq, dword [r6+wq*4]
    vpbroadcastd         m4, [base+pb_64]
    vpbroadcastd         m5, [base+pw_512]
    sub                tmpq, maskq
    add                  wq, r6
    lea                  r6, [dsq*3]
    jmp                  wq
.w4:
    movd                xm0, [dstq+dsq*0]
    pinsrd              xm0, [dstq+dsq*1], 1
    vpbroadcastd        xm1, [dstq+dsq*2]
    pinsrd              xm1, [dstq+r6   ], 3
    mova                xm6, [maskq]
    psubb               xm3, xm4, xm6
    punpcklbw           xm2, xm3, xm6
    punpckhbw           xm3, xm6
    mova                xm6, [maskq+tmpq]
    add               maskq, 4*4
    punpcklbw           xm0, xm6
    punpckhbw           xm1, xm6
    pmaddubsw           xm0, xm2
    pmaddubsw           xm1, xm3
    pmulhrsw            xm0, xm5
    pmulhrsw            xm1, xm5
    packuswb            xm0, xm1
    movd       [dstq+dsq*0], xm0
    pextrd     [dstq+dsq*1], xm0, 1
    pextrd     [dstq+dsq*2], xm0, 2
    pextrd     [dstq+r6   ], xm0, 3
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w4
    RET
ALIGN function_align
.w8:
    movq                xm1, [dstq+dsq*0]
    movhps              xm1, [dstq+dsq*1]
    vpbroadcastq         m2, [dstq+dsq*2]
    vpbroadcastq         m3, [dstq+r6   ]
    mova                 m0, [maskq]
    mova                 m6, [maskq+tmpq]
    add               maskq, 8*4
    vpblendd             m1, m2, 0x30
    vpblendd             m1, m3, 0xc0
    psubb                m3, m4, m0
    punpcklbw            m2, m3, m0
    punpckhbw            m3, m0
    punpcklbw            m0, m1, m6
    punpckhbw            m1, m6
    pmaddubsw            m0, m2
    pmaddubsw            m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    vextracti128        xm1, m0, 1
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    movq       [dstq+dsq*2], xm1
    movhps     [dstq+r6   ], xm1
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w8
    RET
ALIGN function_align
.w16:
    mova                 m0, [maskq]
    mova                xm1, [dstq+dsq*0]
    vinserti128          m1, [dstq+dsq*1], 1
    psubb                m3, m4, m0
    punpcklbw            m2, m3, m0
    punpckhbw            m3, m0
    mova                 m6, [maskq+tmpq]
    add               maskq, 16*2
    punpcklbw            m0, m1, m6
    punpckhbw            m1, m6
    pmaddubsw            m0, m2
    pmaddubsw            m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w16
    RET
ALIGN function_align
.w32:
    mova                 m0, [maskq]
    mova                 m1, [dstq]
    mova                 m6, [maskq+tmpq]
    add               maskq, 32
    psubb                m3, m4, m0
    punpcklbw            m2, m3, m0
    punpckhbw            m3, m0
    punpcklbw            m0, m1, m6
    punpckhbw            m1, m6
    pmaddubsw            m0, m2
    pmaddubsw            m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, dsq
    dec                  hd
    jg .w32
    RET

cglobal blend_v_8bpc, 3, 6, 6, dst, ds, tmp, w, h, mask
%define base r5-blend_v_avx2_table
    lea                  r5, [blend_v_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, dword [r5+wq*4]
    vpbroadcastd         m5, [base+pw_512]
    add                  wq, r5
    add               maskq, obmc_masks-blend_v_avx2_table
    jmp                  wq
.w2:
    vpbroadcastd        xm2, [maskq+2*2]
.w2_s0_loop:
    movd                xm0, [dstq+dsq*0]
    pinsrw              xm0, [dstq+dsq*1], 1
    movd                xm1, [tmpq]
    add                tmpq, 2*2
    punpcklbw           xm0, xm1
    pmaddubsw           xm0, xm2
    pmulhrsw            xm0, xm5
    packuswb            xm0, xm0
    pextrw     [dstq+dsq*0], xm0, 0
    pextrw     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w2_s0_loop
    RET
ALIGN function_align
.w4:
    vpbroadcastq        xm2, [maskq+4*2]
.w4_loop:
    movd                xm0, [dstq+dsq*0]
    pinsrd              xm0, [dstq+dsq*1], 1
    movq                xm1, [tmpq]
    add                tmpq, 4*2
    punpcklbw           xm0, xm1
    pmaddubsw           xm0, xm2
    pmulhrsw            xm0, xm5
    packuswb            xm0, xm0
    movd       [dstq+dsq*0], xm0
    pextrd     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w4_loop
    RET
ALIGN function_align
.w8:
    mova                xm3, [maskq+8*2]
.w8_loop:
    movq                xm0, [dstq+dsq*0]
    vpbroadcastq        xm1, [dstq+dsq*1]
    mova                xm2, [tmpq]
    add                tmpq, 8*2
    punpcklbw           xm0, xm2
    punpckhbw           xm1, xm2
    pmaddubsw           xm0, xm3
    pmaddubsw           xm1, xm3
    pmulhrsw            xm0, xm5
    pmulhrsw            xm1, xm5
    packuswb            xm0, xm1
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    vbroadcasti128       m3, [maskq+16*2]
    vbroadcasti128       m4, [maskq+16*3]
.w16_loop:
    mova                xm1, [dstq+dsq*0]
    vinserti128          m1, [dstq+dsq*1], 1
    mova                 m2, [tmpq]
    add                tmpq, 16*2
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m0, m3
    pmaddubsw            m1, m4
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    mova                xm3, [maskq+16*4]
    vinserti128          m3, [maskq+16*6], 1
    mova                xm4, [maskq+16*5]
    vinserti128          m4, [maskq+16*7], 1
.w32_loop:
    mova                 m1, [dstq]
    mova                 m2, [tmpq]
    add                tmpq, 32
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m0, m3
    pmaddubsw            m1, m4
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, dsq
    dec                  hd
    jg .w32_loop
    RET

cglobal blend_h_8bpc, 4, 7, 6, dst, ds, tmp, w, h, mask
%define base r5-blend_h_avx2_table
    lea                  r5, [blend_h_avx2_table]
    mov                 r6d, wd
    tzcnt                wd, wd
    mov                  hd, hm
    movsxd               wq, dword [r5+wq*4]
    vpbroadcastd         m5, [base+pw_512]
    add                  wq, r5
    lea               maskq, [base+obmc_masks+hq*2]
    lea                  hd, [hq*3]
    shr                  hd, 2 ; h * 3/4
    lea               maskq, [maskq+hq*2]
    neg                  hq
    jmp                  wq
.w2:
    movd                xm0, [dstq+dsq*0]
    pinsrw              xm0, [dstq+dsq*1], 1
    movd                xm2, [maskq+hq*2]
    movd                xm1, [tmpq]
    add                tmpq, 2*2
    punpcklwd           xm2, xm2
    punpcklbw           xm0, xm1
    pmaddubsw           xm0, xm2
    pmulhrsw            xm0, xm5
    packuswb            xm0, xm0
    pextrw     [dstq+dsq*0], xm0, 0
    pextrw     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w2
    RET
ALIGN function_align
.w4:
    mova                xm3, [blend_shuf]
.w4_loop:
    movd                xm0, [dstq+dsq*0]
    pinsrd              xm0, [dstq+dsq*1], 1
    movd                xm2, [maskq+hq*2]
    movq                xm1, [tmpq]
    add                tmpq, 4*2
    pshufb              xm2, xm3
    punpcklbw           xm0, xm1
    pmaddubsw           xm0, xm2
    pmulhrsw            xm0, xm5
    packuswb            xm0, xm0
    movd       [dstq+dsq*0], xm0
    pextrd     [dstq+dsq*1], xm0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w4_loop
    RET
ALIGN function_align
.w8:
    vbroadcasti128       m4, [blend_shuf]
    shufpd               m4, m4, 0x03
.w8_loop:
    vpbroadcastq         m1, [dstq+dsq*0]
    movq                xm0, [dstq+dsq*1]
    vpblendd             m0, m1, 0x30
    vpbroadcastd         m3, [maskq+hq*2]
    movq                xm1, [tmpq+8*1]
    vinserti128          m1, [tmpq+8*0], 1
    add                tmpq, 8*2
    pshufb               m3, m4
    punpcklbw            m0, m1
    pmaddubsw            m0, m3
    pmulhrsw             m0, m5
    vextracti128        xm1, m0, 1
    packuswb            xm0, xm1
    movhps     [dstq+dsq*0], xm0
    movq       [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w8_loop
    RET
ALIGN function_align
.w16:
    vbroadcasti128       m4, [blend_shuf]
    shufpd               m4, m4, 0x0c
.w16_loop:
    mova                xm1, [dstq+dsq*0]
    vinserti128          m1, [dstq+dsq*1], 1
    vpbroadcastd         m3, [maskq+hq*2]
    mova                 m2, [tmpq]
    add                tmpq, 16*2
    pshufb               m3, m4
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m0, m3
    pmaddubsw            m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w16_loop
    RET
ALIGN function_align
.w32: ; w32/w64/w128
    sub                 dsq, r6
.w32_loop0:
    vpbroadcastw         m3, [maskq+hq*2]
    mov                  wd, r6d
.w32_loop:
    mova                 m1, [dstq]
    mova                 m2, [tmpq]
    add                tmpq, 32
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m0, m3
    pmaddubsw            m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, 32
    sub                  wd, 32
    jg .w32_loop
    add                dstq, dsq
    inc                  hq
    jl .w32_loop0
    RET

cglobal emu_edge_8bpc, 10, 13, 1, bw, bh, iw, ih, x, y, dst, dstride, src, sstride, \
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
    add                srcq, r10

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
    vpbroadcastb         m0, [srcq]
.left_loop_%3:
    mova          [dstq+r3], m0
    add                  r3, 32
    cmp                  r3, leftextq
    jl .left_loop_%3

    ; body
    lea                 r12, [dstq+leftextq]
%endif
    xor                  r3, r3
.body_loop_%3:
    movu                 m0, [srcq+r3]
%if %1
    movu           [r12+r3], m0
%else
    movu          [dstq+r3], m0
%endif
    add                  r3, 32
    cmp                  r3, centerwq
    jl .body_loop_%3

%if %2
    ; right extension
%if %1
    add                 r12, centerwq
%else
    lea                 r12, [dstq+centerwq]
%endif
    xor                  r3, r3
    vpbroadcastb         m0, [srcq+centerwq-1]
.right_loop_%3:
    movu           [r12+r3], m0
    add                  r3, 32
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
    mova                 m0, [srcq+r1]
    lea                  r3, [dstq+r1]
    mov                  r4, bottomextq
.bottom_y_loop:
    mova               [r3], m0
    add                  r3, dstrideq
    dec                  r4
    jg .bottom_y_loop
    add                  r1, 32
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
    mova                 m0, [srcq+r1]
    lea                  r3, [dstq+r1]
    mov                  r4, topextq
.top_y_loop:
    mova               [r3], m0
    add                  r3, dstrideq
    dec                  r4
    jg .top_y_loop
    add                  r1, 32
    cmp                  r1, bwq
    jl .top_x_loop

.end:
    RET

cglobal resize_8bpc, 6, 12, 16, dst, dst_stride, src, src_stride, \
                                dst_w, h, src_w, dx, mx0
    sub          dword mx0m, 4<<14
    sub        dword src_wm, 8
    vpbroadcastd         m5, dxm
    vpbroadcastd         m8, mx0m
    vpbroadcastd         m6, src_wm

    DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, h, x
    LEA                  r7, $$
%define base r7-$$

    vpbroadcastd        xm3, [base+pw_m256]
    vpbroadcastd         m7, [base+pd_63]
    vbroadcasti128      m15, [base+pb_8x0_8x8]
    pmaddwd              m2, m5, [base+rescale_mul] ; dx*[0,1,2,3,4,5,6,7]
    pslld                m5, 3                      ; dx*8
    pslld                m6, 14
    paddd                m8, m2                     ; mx+[0..7]*dx
    pxor                 m2, m2

    ; m2 = 0, m3 = pmulhrsw constant for x=(x+64)>>7
    ; m8 = mx+[0..7]*dx, m5 = dx*8, m6 = src_w, m7 = 0x3f, m15=0,8

.loop_y:
    xor                  xd, xd
    mova                 m4, m8                     ; per-line working version of mx

.loop_x:
    pmaxsd               m0, m4, m2
    psrad                m9, m4, 8                  ; filter offset (unmasked)
    pminsd               m0, m6                     ; iclip(mx, 0, src_w-8)
    psubd                m1, m4, m0                 ; pshufb offset
    psrad                m0, 14                     ; clipped src_x offset
    psrad                m1, 14                     ; pshufb edge_emu offset
    pand                 m9, m7                     ; filter offset (masked)

    ; load source pixels - this ugly code is vpgatherdq emulation since
    ; directly using vpgatherdq on Haswell is quite a bit slower :(
    movd                r8d, xm0
    pextrd              r9d, xm0, 1
    pextrd             r10d, xm0, 2
    pextrd             r11d, xm0, 3
    vextracti128        xm0, m0, 1
    movq               xm12, [srcq+r8]
    movq               xm13, [srcq+r10]
    movhps             xm12, [srcq+r9]
    movhps             xm13, [srcq+r11]
    movd                r8d, xm0
    pextrd              r9d, xm0, 1
    pextrd             r10d, xm0, 2
    pextrd             r11d, xm0, 3
    vinserti128         m12, [srcq+r8], 1
    vinserti128         m13, [srcq+r10], 1
    vpbroadcastq        m10, [srcq+r9]
    vpbroadcastq        m11, [srcq+r11]
    vpblendd            m12, m10, 11000000b
    vpblendd            m13, m11, 11000000b

    ; if no emulation is required, we don't need to shuffle or emulate edges
    ; this also saves 2 quasi-vpgatherdqs
    vptest               m1, m1
    jz .filter

    movq                 r9, xm1
    pextrq              r11, xm1, 1
    movsxd               r8, r9d
    sar                  r9, 32
    movsxd              r10, r11d
    sar                 r11, 32
    vextracti128        xm1, m1, 1
    movq               xm14, [base+resize_shuf+4+r8]
    movq                xm0, [base+resize_shuf+4+r10]
    movhps             xm14, [base+resize_shuf+4+r9]
    movhps              xm0, [base+resize_shuf+4+r11]
    movq                 r9, xm1
    pextrq              r11, xm1, 1
    movsxd               r8, r9d
    sar                  r9, 32
    movsxd              r10, r11d
    sar                 r11, 32
    vinserti128         m14, [base+resize_shuf+4+r8], 1
    vinserti128          m0, [base+resize_shuf+4+r10], 1
    vpbroadcastq        m10, [base+resize_shuf+4+r9]
    vpbroadcastq        m11, [base+resize_shuf+4+r11]
    vpblendd            m14, m10, 11000000b
    vpblendd             m0, m11, 11000000b

    paddb               m14, m15
    paddb                m0, m15
    pshufb              m12, m14
    pshufb              m13, m0

.filter:
    movd                r8d, xm9
    pextrd              r9d, xm9, 1
    pextrd             r10d, xm9, 2
    pextrd             r11d, xm9, 3
    vextracti128        xm9, m9, 1
    movq               xm10, [base+resize_filter+r8*8]
    movq               xm11, [base+resize_filter+r10*8]
    movhps             xm10, [base+resize_filter+r9*8]
    movhps             xm11, [base+resize_filter+r11*8]
    movd                r8d, xm9
    pextrd              r9d, xm9, 1
    pextrd             r10d, xm9, 2
    pextrd             r11d, xm9, 3
    vinserti128         m10, [base+resize_filter+r8*8], 1
    vinserti128         m11, [base+resize_filter+r10*8], 1
    vpbroadcastq        m14, [base+resize_filter+r9*8]
    vpbroadcastq         m1, [base+resize_filter+r11*8]
    vpblendd            m10, m14, 11000000b
    vpblendd            m11, m1, 11000000b

    pmaddubsw           m12, m10
    pmaddubsw           m13, m11
    phaddw              m12, m13
    vextracti128       xm13, m12, 1
    phaddsw            xm12, xm13
    pmulhrsw           xm12, xm3                    ; x=(x+64)>>7
    packuswb           xm12, xm12
    movq          [dstq+xq], xm12

    paddd                m4, m5
    add                  xd, 8
    cmp                  xd, dst_wd
    jl .loop_x

    add                dstq, dst_strideq
    add                srcq, src_strideq
    dec                  hd
    jg .loop_y
    RET

cglobal w_mask_420_8bpc, 4, 8, 14, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_420_avx2_table
    lea                  r7, [w_mask_420_avx2_table]
    tzcnt                wd, wm
    mov                 r6d, r7m ; sign
    movifnidn            hd, hm
    movsxd               wq, [r7+wq*4]
    vpbroadcastd         m6, [base+pw_6903] ; ((64 - 38) << 8) + 255 - 8
    vpbroadcastd         m7, [base+pw_2048]
    pmovzxbd             m9, [base+deint_shuf4]
    vpbroadcastd         m8, [base+wm_420_sign+r6*4] ; 258 - sign
    add                  wq, r7
    W_MASK                0, 4, 0, 1
    mov               maskq, maskmp
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    cmp                  hd, 8
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
    jg .w4_h16
.w4_end:
    vextracti128        xm0, m4, 1
    vpblendd            xm1, xm4, xm0, 0x05
    vpblendd            xm4, xm0, 0x0a
    pshufd              xm1, xm1, q2301
    psubw               xm4, xm8, xm4
    psubw               xm4, xm1
    psrlw               xm4, 2
    packuswb            xm4, xm4
    movq            [maskq], xm4
    RET
.w4_h16:
    W_MASK                0, 5, 2, 3
    lea                dstq, [dstq+strideq*4]
    phaddd               m4, m5
    vextracti128        xm1, m0, 1
    psubw                m4, m8, m4
    psrlw                m4, 2
    vpermd               m4, m9, m4
    vextracti128        xm5, m4, 1
    packuswb            xm4, xm5
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q], xm1, 1
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
    mova            [maskq], xm4
    RET
.w8_loop:
    add               tmp1q, 2*32
    add               tmp2q, 2*32
    W_MASK                0, 4, 0, 1
    lea                dstq, [dstq+strideq*4]
    add               maskq, 8
.w8:
    vextracti128        xm2, m4, 1
    vextracti128        xm1, m0, 1
    psubw               xm4, xm8, xm4
    psubw               xm4, xm2
    psrlw               xm4, 2
    packuswb            xm4, xm4
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    movq            [maskq], xm4
    sub                  hd, 4
    jg .w8_loop
    RET
.w16_loop:
    add               tmp1q, 4*32
    add               tmp2q, 4*32
    W_MASK                0, 4, 0, 1
    lea                dstq, [dstq+strideq*4]
    add               maskq, 16
.w16:
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    W_MASK                0, 5, 2, 3
    punpckhqdq           m1, m4, m5
    punpcklqdq           m4, m5
    psubw                m1, m8, m1
    psubw                m1, m4
    psrlw                m1, 2
    vpermq               m0, m0, q3120
    packuswb             m1, m1
    vpermd               m1, m9, m1
    mova         [dstq+strideq*2], xm0
    vextracti128 [dstq+stride3q ], m0, 1
    mova            [maskq], xm1
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    add               tmp1q, 4*32
    add               tmp2q, 4*32
    W_MASK                0, 4, 0, 1
    lea                dstq, [dstq+strideq*2]
    add               maskq, 16
.w32:
    vpermq               m0, m0, q3120
    mova   [dstq+strideq*0], m0
    W_MASK                0, 5, 2, 3
    psubw                m4, m8, m4
    psubw                m4, m5
    psrlw                m4, 2
    vpermq               m0, m0, q3120
    packuswb             m4, m4
    vpermd               m4, m9, m4
    mova   [dstq+strideq*1], m0
    mova            [maskq], xm4
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop_even:
    psubw               m10, m8, m4
    psubw               m11, m8, m5
    dec                  hd
.w64_loop:
    add               tmp1q, 4*32
    add               tmp2q, 4*32
    W_MASK                0, 4, 0, 1
    add                dstq, strideq
.w64:
    vpermq               m0, m0, q3120
    mova        [dstq+32*0], m0
    W_MASK                0, 5, 2, 3
    vpermq               m0, m0, q3120
    mova        [dstq+32*1], m0
    test                 hd, 1
    jz .w64_loop_even
    psubw                m4, m10, m4
    psubw                m5, m11, m5
    psrlw                m4, 2
    psrlw                m5, 2
    packuswb             m4, m5
    vpermd               m4, m9, m4
    mova            [maskq], m4
    add               maskq, 32
    dec                  hd
    jg .w64_loop
    RET
.w128_loop_even:
    psubw               m12, m8, m4
    psubw               m13, m8, m5
    dec                  hd
.w128_loop:
    W_MASK                0, 4, 0, 1
    add                dstq, strideq
.w128:
    vpermq               m0, m0, q3120
    mova        [dstq+32*0], m0
    W_MASK                0, 5, 2, 3
    vpermq               m0, m0, q3120
    mova        [dstq+32*1], m0
    add               tmp1q, 8*32
    add               tmp2q, 8*32
    test                 hd, 1
    jz .w128_even
    psubw                m4, m10, m4
    psubw                m5, m11, m5
    psrlw                m4, 2
    psrlw                m5, 2
    packuswb             m4, m5
    vpermd               m4, m9, m4
    mova       [maskq+32*0], m4
    jmp .w128_odd
.w128_even:
    psubw               m10, m8, m4
    psubw               m11, m8, m5
.w128_odd:
    W_MASK                0, 4, -4, -3
    vpermq               m0, m0, q3120
    mova        [dstq+32*2], m0
    W_MASK                0, 5, -2, -1
    vpermq               m0, m0, q3120
    mova        [dstq+32*3], m0
    test                 hd, 1
    jz .w128_loop_even
    psubw                m4, m12, m4
    psubw                m5, m13, m5
    psrlw                m4, 2
    psrlw                m5, 2
    packuswb             m4, m5
    vpermd               m4, m9, m4
    mova       [maskq+32*1], m4
    add               maskq, 64
    dec                  hd
    jg .w128_loop
    RET

cglobal w_mask_422_8bpc, 4, 8, 11, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_422_avx2_table
    lea                  r7, [w_mask_422_avx2_table]
    tzcnt                wd, wm
    mov                 r6d, r7m ; sign
    movifnidn            hd, hm
    pxor                 m9, m9
    movsxd               wq, dword [r7+wq*4]
    vpbroadcastd         m6, [base+pw_6903] ; ((64 - 38) << 8) + 255 - 8
    vpbroadcastd         m7, [base+pw_2048]
    pmovzxbd            m10, [base+deint_shuf4]
    vpbroadcastd         m8, [base+wm_422_sign+r6*4] ; 128 - sign
    add                  wq, r7
    mov               maskq, maskmp
    W_MASK                0, 4, 0, 1
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    cmp                  hd, 8
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
    jg .w4_h16
.w4_end:
    vextracti128        xm5, m4, 1
    packuswb            xm4, xm5
    psubb               xm5, xm8, xm4
    pavgb               xm5, xm9
    pshufd              xm5, xm5, q3120
    mova            [maskq], xm5
    RET
.w4_h16:
    W_MASK                0, 5, 2, 3
    lea                dstq, [dstq+strideq*4]
    packuswb             m4, m5
    psubb                m5, m8, m4
    pavgb                m5, m9
    vpermd               m5, m10, m5
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
    mova            [maskq], m5
    RET
.w8_loop:
    add               tmp1q, 32*2
    add               tmp2q, 32*2
    W_MASK                0, 4, 0, 1
    lea                dstq, [dstq+strideq*4]
    add               maskq, 16
.w8:
    vextracti128        xm5, m4, 1
    vextracti128        xm1, m0, 1
    packuswb            xm4, xm5
    psubb               xm5, xm8, xm4
    pavgb               xm5, xm9
    pshufd              xm5, xm5, q3120
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    mova            [maskq], xm5
    sub                  hd, 4
    jg .w8_loop
    RET
.w16_loop:
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    W_MASK                0, 4, 0, 1
    lea                dstq, [dstq+strideq*4]
    add               maskq, 32
.w16:
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    W_MASK                0, 5, 2, 3
    packuswb             m4, m5
    psubb                m5, m8, m4
    pavgb                m5, m9
    vpermq               m0, m0, q3120
    vpermd               m5, m10, m5
    mova         [dstq+strideq*2], xm0
    vextracti128 [dstq+stride3q ], m0, 1
    mova            [maskq], m5
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    W_MASK                0, 4, 0, 1
    lea                dstq, [dstq+strideq*2]
    add               maskq, 32
.w32:
    vpermq               m0, m0, q3120
    mova   [dstq+strideq*0], m0
    W_MASK                0, 5, 2, 3
    packuswb             m4, m5
    psubb                m5, m8, m4
    pavgb                m5, m9
    vpermq               m0, m0, q3120
    vpermd               m5, m10, m5
    mova   [dstq+strideq*1], m0
    mova            [maskq], m5
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    W_MASK                0, 4, 0, 1
    add                dstq, strideq
    add               maskq, 32
.w64:
    vpermq               m0, m0, q3120
    mova        [dstq+32*0], m0
    W_MASK                0, 5, 2, 3
    packuswb             m4, m5
    psubb                m5, m8, m4
    pavgb                m5, m9
    vpermq               m0, m0, q3120
    vpermd               m5, m10, m5
    mova        [dstq+32*1], m0
    mova            [maskq], m5
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    add               tmp1q, 32*8
    add               tmp2q, 32*8
    W_MASK                0, 4, 0, 1
    add                dstq, strideq
    add               maskq, 32*2
.w128:
    vpermq               m0, m0, q3120
    mova        [dstq+32*0], m0
    W_MASK                0, 5, 2, 3
    packuswb             m4, m5
    psubb                m5, m8, m4
    pavgb                m5, m9
    vpermq               m0, m0, q3120
    vpermd               m5, m10, m5
    mova        [dstq+32*1], m0
    mova       [maskq+32*0], m5
    W_MASK                0, 4, 4, 5
    vpermq               m0, m0, q3120
    mova        [dstq+32*2], m0
    W_MASK                0, 5, 6, 7
    packuswb             m4, m5
    psubb                m5, m8, m4
    pavgb                m5, m9
    vpermq               m0, m0, q3120
    vpermd               m5, m10, m5
    mova        [dstq+32*3], m0
    mova       [maskq+32*1], m5
    dec                  hd
    jg .w128_loop
    RET

cglobal w_mask_444_8bpc, 4, 8, 8, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_444_avx2_table
    lea                  r7, [w_mask_444_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    mov               maskq, maskmp
    movsxd               wq, dword [r7+wq*4]
    vpbroadcastd         m6, [base+pw_6903] ; ((64 - 38) << 8) + 255 - 8
    vpbroadcastd         m5, [base+pb_64]
    vpbroadcastd         m7, [base+pw_2048]
    add                  wq, r7
    W_MASK                0, 4, 0, 1, 1
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    mova       [maskq+32*0], m4
    cmp                  hd, 8
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
    je .w4_end
    W_MASK                0, 4, 2, 3, 1
    lea                dstq, [dstq+strideq*4]
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
    mova       [maskq+32*1], m4
.w4_end:
    RET
.w8_loop:
    add               tmp1q, 32*2
    add               tmp2q, 32*2
    W_MASK                0, 4, 0, 1, 1
    lea                dstq, [dstq+strideq*4]
    add               maskq, 32
.w8:
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    mova            [maskq], m4
    sub                  hd, 4
    jg .w8_loop
    RET
.w16_loop:
    add               tmp1q, 32*2
    add               tmp2q, 32*2
    W_MASK                0, 4, 0, 1, 1
    lea                dstq, [dstq+strideq*2]
    add               maskq, 32
.w16:
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova            [maskq], m4
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    add               tmp1q, 32*2
    add               tmp2q, 32*2
    W_MASK                0, 4, 0, 1, 1
    add                dstq, strideq
    add               maskq, 32
.w32:
    vpermq               m0, m0, q3120
    mova             [dstq], m0
    mova            [maskq], m4
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    add               tmp1q, 32*4
    add               tmp2q, 32*4
    W_MASK                0, 4, 0, 1, 1
    add                dstq, strideq
    add               maskq, 32*2
.w64:
    vpermq               m0, m0, q3120
    mova        [dstq+32*0], m0
    mova       [maskq+32*0], m4
    W_MASK                0, 4, 2, 3, 1
    vpermq               m0, m0, q3120
    mova        [dstq+32*1], m0
    mova       [maskq+32*1], m4
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    add               tmp1q, 32*8
    add               tmp2q, 32*8
    W_MASK                0, 4, 0, 1, 1
    add                dstq, strideq
    add               maskq, 32*4
.w128:
    vpermq               m0, m0, q3120
    mova        [dstq+32*0], m0
    mova       [maskq+32*0], m4
    W_MASK                0, 4, 2, 3, 1
    vpermq               m0, m0, q3120
    mova        [dstq+32*1], m0
    mova       [maskq+32*1], m4
    W_MASK                0, 4, 4, 5, 1
    vpermq               m0, m0, q3120
    mova        [dstq+32*2], m0
    mova       [maskq+32*2], m4
    W_MASK                0, 4, 6, 7, 1
    vpermq               m0, m0, q3120
    mova        [dstq+32*3], m0
    mova       [maskq+32*3], m4
    dec                  hd
    jg .w128_loop
    RET

%endif ; ARCH_X86_64
