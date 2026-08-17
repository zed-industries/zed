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

SECTION_RODATA

; dav1d_obmc_masks[] << 9
obmc_masks:     dw     0,     0,  9728,     0, 12800,  7168,  2560,     0
                dw 14336, 11264,  8192,  5632,  3584,  1536,     0,     0
                dw 15360, 13824, 12288, 10752,  9216,  7680,  6144,  5120
                dw  4096,  3072,  2048,  1536,     0,     0,     0,     0
                dw 15872, 14848, 14336, 13312, 12288, 11776, 10752, 10240
                dw  9728,  8704,  8192,  7168,  6656,  6144,  5632,  4608
                dw  4096,  3584,  3072,  2560,  2048,  2048,  1536,  1024

blend_shuf:     db 0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  2,  3,  2,  3,  2,  3
spel_h_shufA:   db 0,  1,  2,  3,  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9
spel_h_shufB:   db 4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13
spel_h_shuf2:   db 0,  1,  2,  3,  4,  5,  6,  7,  2,  3,  4,  5,  6,  7,  8,  9
spel_s_shuf2:   db 0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7
spel_s_shuf8:   db 0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
unpckw:         db 0,  1,  4,  5,  8,  9, 12, 13,  2,  3,  6,  7, 10, 11, 14, 15
rescale_mul:    dd 0,  1,  2,  3
resize_shuf:    db 0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7
                db 8,  9, 10, 11, 12, 13, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15
bdct_lb_q: times 8 db 0
           times 8 db 4
           times 8 db 8
           times 8 db 12

pw_2:             times 8 dw 2
pw_16:            times 4 dw 16
prep_mul:         times 4 dw 16
                  times 8 dw 4
pw_64:            times 8 dw 64
pw_256:           times 8 dw 256
pw_2048:          times 4 dw 2048
bidir_mul:        times 4 dw 2048
pw_8192:          times 8 dw 8192
pw_27615:         times 8 dw 27615
pw_32766:         times 8 dw 32766
pw_m512:          times 8 dw -512
pd_63:            times 4 dd 63
pd_64:            times 4 dd 64
pd_512:           times 4 dd 512
pd_m524256:       times 4 dd -524256 ; -8192 << 6 + 32
pd_0x3ff:         times 4 dd 0x3ff
pd_0x4000:        times 4 dd 0x4000
pq_0x400000:      times 2 dq 0x400000
pq_0x40000000:    times 2 dq 0x40000000
pd_65538:         times 2 dd 65538

put_bilin_h_rnd:  times 4 dw 8
                  times 4 dw 10
s_8tap_h_rnd:     times 2 dd 2
                  times 2 dd 8
put_s_8tap_v_rnd: times 2 dd 512
                  times 2 dd 128
s_8tap_h_sh:      dd 2, 4
put_s_8tap_v_sh:  dd 10, 8
bidir_rnd:        times 4 dw -16400
                  times 4 dw -16388
put_8tap_h_rnd:   dd 34, 34, 40, 40
prep_8tap_1d_rnd: times 2 dd     8 - (8192 <<  4)
prep_8tap_2d_rnd: times 4 dd    32 - (8192 <<  5)

warp8x8_shift:    dd 11, 13
warp8x8_rnd1:     dd 1024, 1024, 4096, 4096
warp8x8_rnd2:     times 4 dw 4096
                  times 4 dw 16384
warp8x8t_rnd:     times 2 dd 16384 - (8192 << 15)

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

BIDIR_JMP_TABLE avg,        ssse3,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_avg,      ssse3,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE mask,       ssse3,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_420, ssse3,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_422, ssse3,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_444, ssse3,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE blend,      ssse3,    4, 8, 16, 32
BIDIR_JMP_TABLE blend_v,    ssse3, 2, 4, 8, 16, 32
BIDIR_JMP_TABLE blend_h,    ssse3, 2, 4, 8, 16, 32, 64, 128

%macro BASE_JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base %1_%2
    %%table:
    %rep %0 - 2
        dw %%base %+ _w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%xdefine put_ssse3 mangle(private_prefix %+ _put_bilin_16bpc_ssse3.put)
%xdefine prep_ssse3 mangle(private_prefix %+ _prep_bilin_16bpc_ssse3.prep)

BASE_JMP_TABLE put,  ssse3, 2, 4, 8, 16, 32, 64, 128
BASE_JMP_TABLE prep, ssse3,    4, 8, 16, 32, 64, 128

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

SCALED_JMP_TABLE put_8tap_scaled, ssse3, 2, 4, 8, 16, 32, 64, 128
SCALED_JMP_TABLE prep_8tap_scaled, ssse3,   4, 8, 16, 32, 64, 128

cextern mc_subpel_filters
%define subpel_filters (mangle(private_prefix %+ _mc_subpel_filters)-8)

cextern mc_warp_filter
cextern resize_filter

SECTION .text

%if UNIX64
DECLARE_REG_TMP 7
%else
DECLARE_REG_TMP 5
%endif

INIT_XMM ssse3
cglobal put_bilin_16bpc, 4, 7, 0, dst, ds, src, ss, w, h, mxy
%define base t0-put_ssse3
    mov                mxyd, r6m ; mx
    LEA                  t0, put_ssse3
    movifnidn            wd, wm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r7m ; my
    test               mxyd, mxyd
    jnz .v
.put:
    tzcnt                wd, wd
    movzx                wd, word [base+put_ssse3_table+wq*2]
    add                  wq, t0
    movifnidn            hd, hm
    jmp                  wq
.put_w2:
    mov                 r4d, [srcq+ssq*0]
    mov                 r6d, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r4d
    mov        [dstq+dsq*1], r6d
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w2
    RET
.put_w4:
    movq                 m0, [srcq+ssq*0]
    movq                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movq       [dstq+dsq*0], m0
    movq       [dstq+dsq*1], m1
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
.put_w16:
    movu                 m0, [srcq+ssq*0+16*0]
    movu                 m1, [srcq+ssq*0+16*1]
    movu                 m2, [srcq+ssq*1+16*0]
    movu                 m3, [srcq+ssq*1+16*1]
    lea                srcq, [srcq+ssq*2]
    mova  [dstq+dsq*0+16*0], m0
    mova  [dstq+dsq*0+16*1], m1
    mova  [dstq+dsq*1+16*0], m2
    mova  [dstq+dsq*1+16*1], m3
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w16
    RET
.put_w32:
    movu                 m0, [srcq+16*0]
    movu                 m1, [srcq+16*1]
    movu                 m2, [srcq+16*2]
    movu                 m3, [srcq+16*3]
    add                srcq, ssq
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    mova        [dstq+16*2], m2
    mova        [dstq+16*3], m3
    add                dstq, dsq
    dec                  hd
    jg .put_w32
    RET
.put_w64:
    movu                 m0, [srcq+16*0]
    movu                 m1, [srcq+16*1]
    movu                 m2, [srcq+16*2]
    movu                 m3, [srcq+16*3]
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    mova        [dstq+16*2], m2
    mova        [dstq+16*3], m3
    movu                 m0, [srcq+16*4]
    movu                 m1, [srcq+16*5]
    movu                 m2, [srcq+16*6]
    movu                 m3, [srcq+16*7]
    add                srcq, ssq
    mova        [dstq+16*4], m0
    mova        [dstq+16*5], m1
    mova        [dstq+16*6], m2
    mova        [dstq+16*7], m3
    add                dstq, dsq
    dec                  hd
    jg .put_w64
    RET
.put_w128:
    add                srcq, 16*8
    add                dstq, 16*8
.put_w128_loop:
    movu                 m0, [srcq-16*8]
    movu                 m1, [srcq-16*7]
    movu                 m2, [srcq-16*6]
    movu                 m3, [srcq-16*5]
    mova        [dstq-16*8], m0
    mova        [dstq-16*7], m1
    mova        [dstq-16*6], m2
    mova        [dstq-16*5], m3
    movu                 m0, [srcq-16*4]
    movu                 m1, [srcq-16*3]
    movu                 m2, [srcq-16*2]
    movu                 m3, [srcq-16*1]
    mova        [dstq-16*4], m0
    mova        [dstq-16*3], m1
    mova        [dstq-16*2], m2
    mova        [dstq-16*1], m3
    movu                 m0, [srcq+16*0]
    movu                 m1, [srcq+16*1]
    movu                 m2, [srcq+16*2]
    movu                 m3, [srcq+16*3]
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    mova        [dstq+16*2], m2
    mova        [dstq+16*3], m3
    movu                 m0, [srcq+16*4]
    movu                 m1, [srcq+16*5]
    movu                 m2, [srcq+16*6]
    movu                 m3, [srcq+16*7]
    add                srcq, ssq
    mova        [dstq+16*4], m0
    mova        [dstq+16*5], m1
    mova        [dstq+16*6], m2
    mova        [dstq+16*7], m3
    add                dstq, dsq
    dec                  hd
    jg .put_w128_loop
    RET
.h:
    movd                 m5, mxyd
    mov                mxyd, r7m ; my
    mova                 m4, [base+pw_16]
    pshufb               m5, [base+pw_256]
    psubw                m4, m5
    test               mxyd, mxyd
    jnz .hv
    ; 12-bit is rounded twice so we can't use the same pmulhrsw approach as .v
    mov                 r6d, r8m ; bitdepth_max
    shr                 r6d, 11
    movddup              m3, [base+put_bilin_h_rnd+r6*8]
    movifnidn            hd, hm
    sub                  wd, 8
    jg .h_w16
    je .h_w8
    cmp                  wd, -4
    je .h_w4
.h_w2:
    movq                 m1, [srcq+ssq*0]
    movhps               m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmullw               m0, m4, m1
    psrlq                m1, 16
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    psrlw                m0, 4
    movd       [dstq+dsq*0], m0
    punpckhqdq           m0, m0
    movd       [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2
    RET
.h_w4:
    movq                 m0, [srcq+ssq*0]
    movhps               m0, [srcq+ssq*1]
    movq                 m1, [srcq+ssq*0+2]
    movhps               m1, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    psrlw                m0, 4
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4
    RET
.h_w8:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*0+2]
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    movu                 m1, [srcq+ssq*1]
    movu                 m2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    pmullw               m1, m4
    pmullw               m2, m5
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m0, 4
    psrlw                m1, 4
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    lea                srcq, [srcq+wq*2]
    lea                dstq, [dstq+wq*2]
    neg                  wq
.h_w16_loop0:
    mov                  r6, wq
.h_w16_loop:
    movu                 m0, [srcq+r6*2+ 0]
    movu                 m1, [srcq+r6*2+ 2]
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    movu                 m1, [srcq+r6*2+16]
    movu                 m2, [srcq+r6*2+18]
    pmullw               m1, m4
    pmullw               m2, m5
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m0, 4
    psrlw                m1, 4
    mova   [dstq+r6*2+16*0], m0
    mova   [dstq+r6*2+16*1], m1
    add                  r6, 16
    jl .h_w16_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w16_loop0
    RET
.v:
    shl                mxyd, 11
    movd                 m5, mxyd
    pshufb               m5, [base+pw_256]
    movifnidn            hd, hm
    cmp                  wd, 4
    jg .v_w8
    je .v_w4
.v_w2:
    movd                 m0, [srcq+ssq*0]
.v_w2_loop:
    movd                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklqdq           m2, m0, m1
    movd                 m0, [srcq+ssq*0]
    punpcklqdq           m1, m0
    psubw                m1, m2
    pmulhrsw             m1, m5
    paddw                m1, m2
    movd       [dstq+dsq*0], m1
    punpckhqdq           m1, m1
    movd       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movq                 m0, [srcq+ssq*0]
.v_w4_loop:
    movq                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklqdq           m2, m0, m1
    movq                 m0, [srcq+ssq*0]
    punpcklqdq           m1, m0
    psubw                m1, m2
    pmulhrsw             m1, m5
    paddw                m1, m2
    movq       [dstq+dsq*0], m1
    movhps     [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
%if ARCH_X86_64
%if WIN64
    push                 r7
%endif
    shl                  wd, 5
    mov                  r7, srcq
    lea                 r6d, [wq+hq-256]
    mov                  r4, dstq
%else
    mov                  r6, srcq
%endif
.v_w8_loop0:
    movu                 m0, [srcq+ssq*0]
.v_w8_loop:
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
    jg .v_w8_loop
%if ARCH_X86_64
    add                  r7, 16
    add                  r4, 16
    movzx                hd, r6b
    mov                srcq, r7
    mov                dstq, r4
    sub                 r6d, 1<<8
%else
    mov                dstq, dstmp
    add                  r6, 16
    mov                  hd, hm
    add                dstq, 16
    mov                srcq, r6
    mov               dstmp, dstq
    sub                  wd, 8
%endif
    jg .v_w8_loop0
%if WIN64
    pop                 r7
%endif
    RET
.hv:
    WIN64_SPILL_XMM       8
    shl                mxyd, 11
    mova                 m3, [base+pw_2]
    movd                 m6, mxyd
    mova                 m7, [base+pw_8192]
    pshufb               m6, [base+pw_256]
    test          dword r8m, 0x800
    jnz .hv_12bpc
    psllw                m4, 2
    psllw                m5, 2
    mova                 m7, [base+pw_2048]
.hv_12bpc:
    movifnidn            hd, hm
    cmp                  wd, 4
    jg .hv_w8
    je .hv_w4
.hv_w2:
    movddup              m0, [srcq+ssq*0]
    pshufhw              m1, m0, q0321
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    psrlw                m0, 2
.hv_w2_loop:
    movq                 m2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps               m2, [srcq+ssq*0]
    pmullw               m1, m4, m2
    psrlq                m2, 16
    pmullw               m2, m5
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m1, 2            ; 1 _ 2 _
    shufpd               m2, m0, m1, 0x01 ; 0 _ 1 _
    mova                 m0, m1
    psubw                m1, m2
    paddw                m1, m1
    pmulhw               m1, m6
    paddw                m1, m2
    pmulhrsw             m1, m7
    movd       [dstq+dsq*0], m1
    punpckhqdq           m1, m1
    movd       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    movddup              m0, [srcq+ssq*0]
    movddup              m1, [srcq+ssq*0+2]
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    psrlw                m0, 2
.hv_w4_loop:
    movq                 m1, [srcq+ssq*1]
    movq                 m2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    movhps               m1, [srcq+ssq*0]
    movhps               m2, [srcq+ssq*0+2]
    pmullw               m1, m4
    pmullw               m2, m5
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m1, 2            ; 1 2
    shufpd               m2, m0, m1, 0x01 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    paddw                m1, m1
    pmulhw               m1, m6
    paddw                m1, m2
    pmulhrsw             m1, m7
    movq       [dstq+dsq*0], m1
    movhps     [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
%if ARCH_X86_64
%if WIN64
    push                 r7
%endif
    shl                  wd, 5
    lea                 r6d, [wq+hq-256]
    mov                  r4, srcq
    mov                  r7, dstq
%else
    mov                  r6, srcq
%endif
.hv_w8_loop0:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*0+2]
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m3
    paddw                m0, m1
    psrlw                m0, 2
.hv_w8_loop:
    movu                 m1, [srcq+ssq*1]
    movu                 m2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    pmullw               m1, m4
    pmullw               m2, m5
    paddw                m1, m3
    paddw                m1, m2
    psrlw                m1, 2
    psubw                m2, m1, m0
    paddw                m2, m2
    pmulhw               m2, m6
    paddw                m2, m0
    pmulhrsw             m2, m7
    mova       [dstq+dsq*0], m2
    movu                 m0, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*0+2]
    pmullw               m0, m4
    pmullw               m2, m5
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
    jg .hv_w8_loop
%if ARCH_X86_64
    add                  r4, 16
    add                  r7, 16
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
%else
    mov                dstq, dstmp
    add                  r6, 16
    mov                  hd, hm
    add                dstq, 16
    mov                srcq, r6
    mov               dstmp, dstq
    sub                  wd, 8
%endif
    jg .hv_w8_loop0
%if WIN64
    pop                  r7
%endif
    RET

cglobal prep_bilin_16bpc, 4, 7, 0, tmp, src, stride, w, h, mxy, stride3
%define base r6-prep_ssse3
    movifnidn          mxyd, r5m ; mx
    LEA                  r6, prep_ssse3
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r6m ; my
    test               mxyd, mxyd
    jnz .v
.prep:
    tzcnt                wd, wd
    movzx                wd, word [base+prep_ssse3_table+wq*2]
    mov                 r5d, r7m ; bitdepth_max
    mova                 m5, [base+pw_8192]
    add                  wq, r6
    shr                 r5d, 11
    movddup              m4, [base+prep_mul+r5*8]
    lea            stride3q, [strideq*3]
    jmp                  wq
.prep_w4:
    movq                 m0, [srcq+strideq*0]
    movhps               m0, [srcq+strideq*1]
    movq                 m1, [srcq+strideq*2]
    movhps               m1, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    pmullw               m0, m4
    pmullw               m1, m4
    psubw                m0, m5
    psubw                m1, m5
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    add                tmpq, 16*2
    sub                  hd, 4
    jg .prep_w4
    RET
.prep_w8:
    movu                 m0, [srcq+strideq*0]
    movu                 m1, [srcq+strideq*1]
    movu                 m2, [srcq+strideq*2]
    movu                 m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    sub                  hd, 4
    jg .prep_w8
    RET
.prep_w16:
    movu                 m0, [srcq+strideq*0+16*0]
    movu                 m1, [srcq+strideq*0+16*1]
    movu                 m2, [srcq+strideq*1+16*0]
    movu                 m3, [srcq+strideq*1+16*1]
    lea                srcq, [srcq+strideq*2]
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    sub                  hd, 2
    jg .prep_w16
    RET
.prep_w32:
    movu                 m0, [srcq+16*0]
    movu                 m1, [srcq+16*1]
    movu                 m2, [srcq+16*2]
    movu                 m3, [srcq+16*3]
    add                srcq, strideq
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    dec                  hd
    jg .prep_w32
    RET
.prep_w64:
    movu                 m0, [srcq+16*0]
    movu                 m1, [srcq+16*1]
    movu                 m2, [srcq+16*2]
    movu                 m3, [srcq+16*3]
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    movu                 m0, [srcq+16*4]
    movu                 m1, [srcq+16*5]
    movu                 m2, [srcq+16*6]
    movu                 m3, [srcq+16*7]
    add                srcq, strideq
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq+16*4], m0
    mova        [tmpq+16*5], m1
    mova        [tmpq+16*6], m2
    mova        [tmpq+16*7], m3
    add                tmpq, 16*8
    dec                  hd
    jg .prep_w64
    RET
.prep_w128:
    movu                 m0, [srcq+16* 0]
    movu                 m1, [srcq+16* 1]
    movu                 m2, [srcq+16* 2]
    movu                 m3, [srcq+16* 3]
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    movu                 m0, [srcq+16* 4]
    movu                 m1, [srcq+16* 5]
    movu                 m2, [srcq+16* 6]
    movu                 m3, [srcq+16* 7]
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq+16*4], m0
    mova        [tmpq+16*5], m1
    mova        [tmpq+16*6], m2
    mova        [tmpq+16*7], m3
    movu                 m0, [srcq+16* 8]
    movu                 m1, [srcq+16* 9]
    movu                 m2, [srcq+16*10]
    movu                 m3, [srcq+16*11]
    add                tmpq, 16*16
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq-16*8], m0
    mova        [tmpq-16*7], m1
    mova        [tmpq-16*6], m2
    mova        [tmpq-16*5], m3
    movu                 m0, [srcq+16*12]
    movu                 m1, [srcq+16*13]
    movu                 m2, [srcq+16*14]
    movu                 m3, [srcq+16*15]
    add                srcq, strideq
    REPX     {pmullw x, m4}, m0, m1, m2, m3
    REPX     {psubw  x, m5}, m0, m1, m2, m3
    mova        [tmpq-16*4], m0
    mova        [tmpq-16*3], m1
    mova        [tmpq-16*2], m2
    mova        [tmpq-16*1], m3
    dec                  hd
    jg .prep_w128
    RET
.h:
    movd                 m4, mxyd
    mov                mxyd, r6m ; my
    mova                 m3, [base+pw_16]
    pshufb               m4, [base+pw_256]
    mova                 m5, [base+pw_32766]
    psubw                m3, m4
    test          dword r7m, 0x800
    jnz .h_12bpc
    psllw                m3, 2
    psllw                m4, 2
.h_12bpc:
    test               mxyd, mxyd
    jnz .hv
    sub                  wd, 8
    je .h_w8
    jg .h_w16
.h_w4:
    movq                 m0, [srcq+strideq*0]
    movhps               m0, [srcq+strideq*1]
    movq                 m1, [srcq+strideq*0+2]
    movhps               m1, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    pmullw               m0, m3
    pmullw               m1, m4
    psubw                m0, m5
    paddw                m0, m1
    psraw                m0, 2
    mova             [tmpq], m0
    add                tmpq, 16
    sub                  hd, 2
    jg .h_w4
    RET
.h_w8:
    movu                 m0, [srcq+strideq*0]
    movu                 m1, [srcq+strideq*0+2]
    pmullw               m0, m3
    pmullw               m1, m4
    psubw                m0, m5
    paddw                m0, m1
    movu                 m1, [srcq+strideq*1]
    movu                 m2, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    pmullw               m1, m3
    pmullw               m2, m4
    psubw                m1, m5
    paddw                m1, m2
    psraw                m0, 2
    psraw                m1, 2
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    add                tmpq, 16*2
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    lea                srcq, [srcq+wq*2]
    neg                  wq
.h_w16_loop0:
    mov                  r6, wq
.h_w16_loop:
    movu                 m0, [srcq+r6*2+ 0]
    movu                 m1, [srcq+r6*2+ 2]
    pmullw               m0, m3
    pmullw               m1, m4
    psubw                m0, m5
    paddw                m0, m1
    movu                 m1, [srcq+r6*2+16]
    movu                 m2, [srcq+r6*2+18]
    pmullw               m1, m3
    pmullw               m2, m4
    psubw                m1, m5
    paddw                m1, m2
    psraw                m0, 2
    psraw                m1, 2
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    add                tmpq, 16*2
    add                  r6, 16
    jl .h_w16_loop
    add                srcq, strideq
    dec                  hd
    jg .h_w16_loop0
    RET
.v:
    movd                 m4, mxyd
    mova                 m3, [base+pw_16]
    pshufb               m4, [base+pw_256]
    mova                 m5, [base+pw_32766]
    psubw                m3, m4
    test          dword r7m, 0x800
    jnz .v_12bpc
    psllw                m3, 2
    psllw                m4, 2
.v_12bpc:
    cmp                  wd, 8
    je .v_w8
    jg .v_w16
.v_w4:
    movq                 m0, [srcq+strideq*0]
.v_w4_loop:
    movq                 m2, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    punpcklqdq           m1, m0, m2 ; 0 1
    movq                 m0, [srcq+strideq*0]
    punpcklqdq           m2, m0     ; 1 2
    pmullw               m1, m3
    pmullw               m2, m4
    psubw                m1, m5
    paddw                m1, m2
    psraw                m1, 2
    mova             [tmpq], m1
    add                tmpq, 16
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movu                 m0, [srcq+strideq*0]
.v_w8_loop:
    movu                 m2, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    pmullw               m0, m3
    pmullw               m1, m4, m2
    psubw                m0, m5
    paddw                m1, m0
    movu                 m0, [srcq+strideq*0]
    psraw                m1, 2
    pmullw               m2, m3
    mova        [tmpq+16*0], m1
    pmullw               m1, m4, m0
    psubw                m2, m5
    paddw                m1, m2
    psraw                m1, 2
    mova        [tmpq+16*1], m1
    add                tmpq, 16*2
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w16:
%if WIN64
    push                 r7
%endif
    mov                  r5, srcq
%if ARCH_X86_64
    lea                 r6d, [wq*4-32]
    mov                  wd, wd
    lea                 r6d, [hq+r6*8]
    mov                  r7, tmpq
%else
    mov                 r6d, wd
%endif
.v_w16_loop0:
    movu                 m0, [srcq+strideq*0]
.v_w16_loop:
    movu                 m2, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    pmullw               m0, m3
    pmullw               m1, m4, m2
    psubw                m0, m5
    paddw                m1, m0
    movu                 m0, [srcq+strideq*0]
    psraw                m1, 2
    pmullw               m2, m3
    mova        [tmpq+wq*0], m1
    pmullw               m1, m4, m0
    psubw                m2, m5
    paddw                m1, m2
    psraw                m1, 2
    mova        [tmpq+wq*2], m1
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jg .v_w16_loop
%if ARCH_X86_64
    add                  r5, 16
    add                  r7, 16
    movzx                hd, r6b
    mov                srcq, r5
    mov                tmpq, r7
    sub                 r6d, 1<<8
%else
    mov                tmpq, tmpmp
    add                  r5, 16
    mov                  hd, hm
    add                tmpq, 16
    mov                srcq, r5
    mov               tmpmp, tmpq
    sub                 r6d, 8
%endif
    jg .v_w16_loop0
%if WIN64
    pop                  r7
%endif
    RET
.hv:
    WIN64_SPILL_XMM       7
    shl                mxyd, 11
    movd                 m6, mxyd
    pshufb               m6, [base+pw_256]
    cmp                  wd, 8
    je .hv_w8
    jg .hv_w16
.hv_w4:
    movddup              m0, [srcq+strideq*0]
    movddup              m1, [srcq+strideq*0+2]
    pmullw               m0, m3
    pmullw               m1, m4
    psubw                m0, m5
    paddw                m0, m1
    psraw                m0, 2
.hv_w4_loop:
    movq                 m1, [srcq+strideq*1]
    movq                 m2, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    movhps               m1, [srcq+strideq*0]
    movhps               m2, [srcq+strideq*0+2]
    pmullw               m1, m3
    pmullw               m2, m4
    psubw                m1, m5
    paddw                m1, m2
    psraw                m1, 2            ; 1 2
    shufpd               m2, m0, m1, 0x01 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    pmulhrsw             m1, m6
    paddw                m1, m2
    mova             [tmpq], m1
    add                tmpq, 16
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    movu                 m0, [srcq+strideq*0]
    movu                 m1, [srcq+strideq*0+2]
    pmullw               m0, m3
    pmullw               m1, m4
    psubw                m0, m5
    paddw                m0, m1
    psraw                m0, 2
.hv_w8_loop:
    movu                 m1, [srcq+strideq*1]
    movu                 m2, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    pmullw               m1, m3
    pmullw               m2, m4
    psubw                m1, m5
    paddw                m1, m2
    psraw                m1, 2
    psubw                m2, m1, m0
    pmulhrsw             m2, m6
    paddw                m2, m0
    mova        [tmpq+16*0], m2
    movu                 m0, [srcq+strideq*0]
    movu                 m2, [srcq+strideq*0+2]
    pmullw               m0, m3
    pmullw               m2, m4
    psubw                m0, m5
    paddw                m0, m2
    psraw                m0, 2
    psubw                m2, m0, m1
    pmulhrsw             m2, m6
    paddw                m2, m1
    mova        [tmpq+16*1], m2
    add                tmpq, 16*2
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
%if WIN64
    push                 r7
%endif
    mov                  r5, srcq
%if ARCH_X86_64
    lea                 r6d, [wq*4-32]
    mov                  wd, wd
    lea                 r6d, [hq+r6*8]
    mov                  r7, tmpq
%else
    mov                 r6d, wd
%endif
.hv_w16_loop0:
    movu                 m0, [srcq+strideq*0]
    movu                 m1, [srcq+strideq*0+2]
    pmullw               m0, m3
    pmullw               m1, m4
    psubw                m0, m5
    paddw                m0, m1
    psraw                m0, 2
.hv_w16_loop:
    movu                 m1, [srcq+strideq*1]
    movu                 m2, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    pmullw               m1, m3
    pmullw               m2, m4
    psubw                m1, m5
    paddw                m1, m2
    psraw                m1, 2
    psubw                m2, m1, m0
    pmulhrsw             m2, m6
    paddw                m2, m0
    mova        [tmpq+wq*0], m2
    movu                 m0, [srcq+strideq*0]
    movu                 m2, [srcq+strideq*0+2]
    pmullw               m0, m3
    pmullw               m2, m4
    psubw                m0, m5
    paddw                m0, m2
    psraw                m0, 2
    psubw                m2, m0, m1
    pmulhrsw             m2, m6
    paddw                m2, m1
    mova        [tmpq+wq*2], m2
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jg .hv_w16_loop
%if ARCH_X86_64
    add                  r5, 16
    add                  r7, 16
    movzx                hd, r6b
    mov                srcq, r5
    mov                tmpq, r7
    sub                 r6d, 1<<8
%else
    mov                tmpq, tmpmp
    add                  r5, 16
    mov                  hd, hm
    add                tmpq, 16
    mov                srcq, r5
    mov               tmpmp, tmpq
    sub                 r6d, 8
%endif
    jg .hv_w16_loop0
%if WIN64
    pop                  r7
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

%if ARCH_X86_32
DECLARE_REG_TMP 1, 2, 6
%elif WIN64
DECLARE_REG_TMP 4, 5, 8
%else
DECLARE_REG_TMP 7, 8, 8
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

%if ARCH_X86_32
cglobal put_8tap_16bpc, 0, 7, 8, dst, ds, src, ss, w, h, mx, my
%define mxb r0b
%define mxd r0
%define mxq r0
%define myb r1b
%define myd r1
%define myq r1
%define  m8 [esp+16*0]
%define  m9 [esp+16*1]
%define m10 [esp+16*2]
%define m11 [esp+16*3]
%define m12 [esp+16*4]
%define m13 [esp+16*5]
%define m14 [esp+16*6]
%define m15 [esp+16*7]
%else
cglobal put_8tap_16bpc, 4, 9, 0, dst, ds, src, ss, w, h, mx, my
%endif
%define base t2-put_ssse3
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    LEA                  t2, put_ssse3
    movifnidn            wd, wm
    movifnidn          srcq, srcmp
    movifnidn           ssq, ssmp
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    movzx                wd, word [base+put_ssse3_table+wq*2]
    movifnidn          dstq, dstmp
    movifnidn           dsq, dsmp
    add                  wq, t2
%if WIN64
    pop                  r8
    pop                  r7
%endif
    jmp                  wq
.h:
    test                myd, 0xf00
    jnz .hv
    mov                 myd, r8m
    movd                 m5, r8m
    shr                 myd, 11
    movddup              m4, [base+put_8tap_h_rnd+myq*8]
    movifnidn           dsq, dsmp
    pshufb               m5, [base+pw_256]
    cmp                  wd, 4
    jg .h_w8
    movzx               mxd, mxb
    lea                srcq, [srcq-2]
    movq                 m3, [base+subpel_filters+mxq*8]
    movifnidn          dstq, dstmp
    punpcklbw            m3, m3
    psraw                m3, 8 ; sign-extend
    je .h_w4
.h_w2:
    mova                 m2, [base+spel_h_shuf2]
    pshufd               m3, m3, q2121
.h_w2_loop:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb               m0, m2
    pshufb               m1, m2
    pmaddwd              m0, m3
    pmaddwd              m1, m3
    phaddd               m0, m1
    paddd                m0, m4
    psrad                m0, 6
    packssdw             m0, m0
    pxor                 m1, m1
    pminsw               m0, m5
    pmaxsw               m0, m1
    movd       [dstq+dsq*0], m0
    pshuflw              m0, m0, q3232
    movd       [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2_loop
    RET
.h_w4:
    WIN64_SPILL_XMM       8
    mova                 m6, [base+spel_h_shufA]
    mova                 m7, [base+spel_h_shufB]
    pshufd               m2, m3, q1111
    pshufd               m3, m3, q2222
.h_w4_loop:
    movu                 m1, [srcq]
    add                srcq, ssq
    pshufb               m0, m1, m6 ; 0 1 1 2 2 3 3 4
    pshufb               m1, m7     ; 2 3 3 4 4 5 5 6
    pmaddwd              m0, m2
    pmaddwd              m1, m3
    paddd                m0, m4
    paddd                m0, m1
    psrad                m0, 6
    packssdw             m0, m0
    pxor                 m1, m1
    pminsw               m0, m5
    pmaxsw               m0, m1
    movq             [dstq], m0
    add                dstq, dsq
    dec                  hd
    jg .h_w4_loop
    RET
.h_w8:
%if WIN64
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      12
%endif
    shr                 mxd, 16
    movq                 m3, [base+subpel_filters+mxq*8]
    movifnidn          dstq, dstmp
    mova                 m6, [base+spel_h_shufA]
    mova                 m7, [base+spel_h_shufB]
%if UNIX64
    mov                  wd, wd
%endif
    lea                srcq, [srcq+wq*2]
    punpcklbw            m3, m3
    lea                dstq, [dstq+wq*2]
    psraw                m3, 8
    neg                  wq
%if ARCH_X86_32
    ALLOC_STACK       -16*4
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova                 m8, m0
    mova                 m9, m1
    mova                m10, m2
    mova                m11, m3
%else
    pshufd               m8, m3, q0000
    pshufd               m9, m3, q1111
    pshufd              m10, m3, q2222
    pshufd              m11, m3, q3333
%endif
.h_w8_loop0:
    mov                  r6, wq
.h_w8_loop:
    movu                 m0, [srcq+r6*2- 6]
    movu                 m1, [srcq+r6*2+ 2]
    pshufb               m2, m0, m6   ; 0 1 1 2 2 3 3 4
    pshufb               m0, m7       ; 2 3 3 4 4 5 5 6
    pmaddwd              m2, m8       ; abcd0
    pmaddwd              m0, m9       ; abcd1
    pshufb               m3, m1, m6   ; 4 5 5 6 6 7 7 8
    pshufb               m1, m7       ; 6 7 7 8 8 9 9 a
    paddd                m2, m4
    paddd                m0, m2
    pmaddwd              m2, m10, m3  ; abcd2
    pmaddwd              m3, m8       ; efgh0
    paddd                m0, m2
    pmaddwd              m2, m11, m1  ; abcd3
    pmaddwd              m1, m9       ; efgh1
    paddd                m0, m2
    movu                 m2, [srcq+r6*2+10]
    paddd                m3, m4
    paddd                m1, m3
    pshufb               m3, m2, m6   ; 8 9 9 a a b b c
    pshufb               m2, m7       ; a b b c c d d e
    pmaddwd              m3, m10      ; efgh2
    pmaddwd              m2, m11      ; efgh3
    paddd                m1, m3
    paddd                m1, m2
    psrad                m0, 6
    psrad                m1, 6
    packssdw             m0, m1
    pxor                 m1, m1
    pminsw               m0, m5
    pmaxsw               m0, m1
    mova        [dstq+r6*2], m0
    add                  r6, 8
    jl .h_w8_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w8_loop0
    RET
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovb               myd, mxd
    movq                 m3, [base+subpel_filters+myq*8]
%if STACK_ALIGNMENT < 16
    %xdefine           rstk  rsp
%else
    %assign stack_offset stack_offset - stack_size_padded
%endif
%if WIN64
    WIN64_SPILL_XMM      15
%endif
    movd                 m7, r8m
    movifnidn          dstq, dstmp
    movifnidn           dsq, dsmp
    punpcklbw            m3, m3
    pshufb               m7, [base+pw_256]
    psraw                m3, 8 ; sign-extend
%if ARCH_X86_32
    ALLOC_STACK       -16*7
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova                 m8, m0
    mova                 m9, m1
    mova                m10, m2
    mova                m11, m3
%else
    pshufd               m8, m3, q0000
    pshufd               m9, m3, q1111
    pshufd              m10, m3, q2222
    pshufd              m11, m3, q3333
%endif
    lea                  r6, [ssq*3]
    sub                srcq, r6
    cmp                  wd, 2
    jne .v_w4
.v_w2:
    movd                 m1, [srcq+ssq*0]
    movd                 m4, [srcq+ssq*1]
    movd                 m2, [srcq+ssq*2]
    add                srcq, r6
    movd                 m5, [srcq+ssq*0]
    movd                 m3, [srcq+ssq*1]
    movd                 m6, [srcq+ssq*2]
    add                srcq, r6
    movd                 m0, [srcq+ssq*0]
    punpckldq            m1, m4      ; 0 1
    punpckldq            m4, m2      ; 1 2
    punpckldq            m2, m5      ; 2 3
    punpckldq            m5, m3      ; 3 4
    punpckldq            m3, m6      ; 4 5
    punpckldq            m6, m0      ; 5 6
    punpcklwd            m1, m4      ; 01 12
    punpcklwd            m2, m5      ; 23 34
    punpcklwd            m3, m6      ; 45 56
    pxor                 m6, m6
.v_w2_loop:
    movd                 m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddwd              m5, m8, m1  ; a0 b0
    mova                 m1, m2
    pmaddwd              m2, m9      ; a1 b1
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, m10     ; a2 b2
    paddd                m5, m3
    punpckldq            m3, m0, m4  ; 6 7
    movd                 m0, [srcq+ssq*0]
    punpckldq            m4, m0      ; 7 8
    punpcklwd            m3, m4      ; 67 78
    pmaddwd              m4, m11, m3 ; a3 b3
    paddd                m5, m4
    psrad                m5, 5
    packssdw             m5, m5
    pmaxsw               m5, m6
    pavgw                m5, m6
    pminsw               m5, m7
    movd       [dstq+dsq*0], m5
    pshuflw              m5, m5, q3232
    movd       [dstq+dsq*1], m5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
%if ARCH_X86_32
    shl                  wd, 14
%if STACK_ALIGNMENT < 16
    mov          [esp+4*29], srcq
    mov          [esp+4*30], dstq
%else
    mov               srcmp, srcq
%endif
    lea                  wd, [wq+hq-(1<<16)]
%else
    shl                  wd, 6
    mov                  r7, srcq
    mov                  r8, dstq
    lea                  wd, [wq+hq-(1<<8)]
%endif
.v_w4_loop0:
    movq                 m1, [srcq+ssq*0]
    movq                 m2, [srcq+ssq*1]
    movq                 m3, [srcq+ssq*2]
    add                srcq, r6
    movq                 m4, [srcq+ssq*0]
    movq                 m5, [srcq+ssq*1]
    movq                 m6, [srcq+ssq*2]
    add                srcq, r6
    movq                 m0, [srcq+ssq*0]
    punpcklwd            m1, m2      ; 01
    punpcklwd            m2, m3      ; 12
    punpcklwd            m3, m4      ; 23
    punpcklwd            m4, m5      ; 34
    punpcklwd            m5, m6      ; 45
    punpcklwd            m6, m0      ; 56
%if ARCH_X86_32
    jmp .v_w4_loop_start
.v_w4_loop:
    mova                 m1, m12
    mova                 m2, m13
    mova                 m3, m14
.v_w4_loop_start:
    pmaddwd              m1, m8      ; a0
    pmaddwd              m2, m8      ; b0
    mova                m12, m3
    mova                m13, m4
    pmaddwd              m3, m9      ; a1
    pmaddwd              m4, m9      ; b1
    paddd                m1, m3
    paddd                m2, m4
    mova                m14, m5
    mova                 m4, m6
    pmaddwd              m5, m10     ; a2
    pmaddwd              m6, m10     ; b2
    paddd                m1, m5
    paddd                m2, m6
    movq                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklwd            m5, m0, m6  ; 67
    movq                 m0, [srcq+ssq*0]
    pmaddwd              m3, m11, m5 ; a3
    punpcklwd            m6, m0      ; 78
    paddd                m1, m3
    pmaddwd              m3, m11, m6 ; b3
    paddd                m2, m3
    psrad                m1, 5
    psrad                m2, 5
    packssdw             m1, m2
    pxor                 m2, m2
    pmaxsw               m1, m2
    pavgw                m1, m2
    pminsw               m1, m7
    movq       [dstq+dsq*0], m1
    movhps     [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
%if STACK_ALIGNMENT < 16
    mov                srcq, [esp+4*29]
    mov                dstq, [esp+4*30]
    movzx                hd, ww
    add                srcq, 8
    add                dstq, 8
    mov          [esp+4*29], srcq
    mov          [esp+4*30], dstq
%else
    mov                srcq, srcmp
    mov                dstq, dstmp
    movzx                hd, ww
    add                srcq, 8
    add                dstq, 8
    mov               srcmp, srcq
    mov               dstmp, dstq
%endif
    sub                  wd, 1<<16
%else
.v_w4_loop:
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
    paddd               m13, m6
    movq                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklwd            m5, m0, m6  ; 67
    movq                 m0, [srcq+ssq*0]
    pmaddwd             m14, m11, m5 ; a3
    punpcklwd            m6, m0      ; 78
    paddd               m12, m14
    pmaddwd             m14, m11, m6 ; b3
    paddd               m13, m14
    psrad               m12, 5
    psrad               m13, 5
    packssdw            m12, m13
    pxor                m13, m13
    pmaxsw              m12, m13
    pavgw               m12, m13
    pminsw              m12, m7
    movq       [dstq+dsq*0], m12
    movhps     [dstq+dsq*1], m12
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    add                  r7, 8
    add                  r8, 8
    movzx                hd, wb
    mov                srcq, r7
    mov                dstq, r8
    sub                  wd, 1<<8
%endif
    jg .v_w4_loop0
    RET
.hv:
%if STACK_ALIGNMENT < 16
    %xdefine           rstk  rsp
%else
    %assign stack_offset stack_offset - stack_size_padded
%endif
%if ARCH_X86_32
    movd                 m4, r8m
    mova                 m6, [base+pd_512]
    pshufb               m4, [base+pw_256]
%else
%if WIN64
    ALLOC_STACK        16*6, 16
%endif
    movd                m15, r8m
    pshufb              m15, [base+pw_256]
%endif
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    je .hv_w4
    movq                 m0, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovb               myd, mxd
    movq                 m3, [base+subpel_filters+myq*8]
%if ARCH_X86_32
    mov                dstq, dstmp
    mov                 dsq, dsmp
    mova                 m5, [base+spel_h_shuf2]
    ALLOC_STACK       -16*8
%else
    mova                 m6, [base+pd_512]
    mova                 m9, [base+spel_h_shuf2]
%endif
    pshuflw              m0, m0, q2121
    pxor                 m7, m7
    punpcklbw            m7, m0
    punpcklbw            m3, m3
    psraw                m3, 8 ; sign-extend
    test          dword r8m, 0x800
    jz .hv_w2_10bpc
    psraw                m7, 2
    psllw                m3, 2
.hv_w2_10bpc:
    lea                  r6, [ssq*3]
    sub                srcq, 2
    sub                srcq, r6
%if ARCH_X86_32
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova                 m9, m5
    mova                m11, m0
    mova                m12, m1
    mova                m13, m2
    mova                m14, m3
    mova                m15, m4
%else
    pshufd              m11, m3, q0000
    pshufd              m12, m3, q1111
    pshufd              m13, m3, q2222
    pshufd              m14, m3, q3333
%endif
    movu                 m2, [srcq+ssq*0]
    movu                 m3, [srcq+ssq*1]
    movu                 m1, [srcq+ssq*2]
    add                srcq, r6
    movu                 m4, [srcq+ssq*0]
%if ARCH_X86_32
    REPX    {pshufb  x, m5}, m2, m3, m1, m4
%else
    REPX    {pshufb  x, m9}, m2, m3, m1, m4
%endif
    REPX    {pmaddwd x, m7}, m2, m3, m1, m4
    phaddd               m2, m3        ; 0 1
    phaddd               m1, m4        ; 2 3
    movu                 m3, [srcq+ssq*1]
    movu                 m4, [srcq+ssq*2]
    add                srcq, r6
    movu                 m0, [srcq+ssq*0]
%if ARCH_X86_32
    REPX    {pshufb  x, m5}, m3, m4, m0
%else
    REPX    {pshufb  x, m9}, m3, m4, m0
%endif
    REPX    {pmaddwd x, m7}, m3, m4, m0
    phaddd               m3, m4        ; 4 5
    phaddd               m0, m0        ; 6 6
    REPX    {paddd   x, m6}, m2, m1, m3, m0
    REPX    {psrad   x, 10}, m2, m1, m3, m0
    packssdw             m2, m1        ; 0 1 2 3
    packssdw             m3, m0        ; 4 5 6 _
    palignr              m4, m3, m2, 4 ; 1 2 3 4
    pshufd               m5, m3, q0321 ; 5 6 _ _
    punpcklwd            m1, m2, m4    ; 01 12
    punpckhwd            m2, m4        ; 23 34
    punpcklwd            m3, m5        ; 45 56
.hv_w2_loop:
    movu                 m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movu                 m5, [srcq+ssq*0]
    pshufb               m4, m9
    pshufb               m5, m9
    pmaddwd              m4, m7
    pmaddwd              m5, m7
    phaddd               m4, m5
    pmaddwd              m5, m11, m1   ; a0 b0
    mova                 m1, m2
    pmaddwd              m2, m12       ; a1 b1
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, m13       ; a2 b2
    paddd                m5, m3
    paddd                m4, m6
    psrad                m4, 10        ; 7 8
    packssdw             m0, m4
    pshufd               m3, m0, q2103
    punpckhwd            m3, m0        ; 67 78
    mova                 m0, m4
    pmaddwd              m4, m14, m3   ; a3 b3
    paddd                m5, m6
    paddd                m5, m4
    psrad                m5, 10
    packssdw             m5, m5
    pxor                 m4, m4
    pminsw               m5, m15
    pmaxsw               m5, m4
    movd       [dstq+dsq*0], m5
    pshuflw              m5, m5, q3232
    movd       [dstq+dsq*1], m5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w8:
    shr                 mxd, 16
.hv_w4:
    movq                 m2, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovb               myd, mxd
    movq                 m3, [base+subpel_filters+myq*8]
%if ARCH_X86_32
%if STACK_ALIGNMENT < 16
    %xdefine           rstk  rsp
%else
    %assign stack_offset stack_offset - stack_size_padded
%endif
    mov                dstq, dstmp
    mov                 dsq, dsmp
    mova                 m0, [base+spel_h_shufA]
    mova                 m1, [base+spel_h_shufB]
    ALLOC_STACK      -16*15
    mova                 m8, m0
    mova                 m9, m1
    mova                m14, m6
%else
    mova                 m8, [base+spel_h_shufA]
    mova                 m9, [base+spel_h_shufB]
%endif
    pxor                 m0, m0
    punpcklbw            m0, m2
    punpcklbw            m3, m3
    psraw                m3, 8
    test          dword r8m, 0x800
    jz .hv_w4_10bpc
    psraw                m0, 2
    psllw                m3, 2
.hv_w4_10bpc:
    lea                  r6, [ssq*3]
    sub                srcq, 6
    sub                srcq, r6
%if ARCH_X86_32
    %define tmp esp+16*8
    shl                  wd, 14
%if STACK_ALIGNMENT < 16
    mov          [esp+4*61], srcq
    mov          [esp+4*62], dstq
%else
    mov               srcmp, srcq
%endif
    mova         [tmp+16*5], m4
    lea                  wd, [wq+hq-(1<<16)]
    pshufd               m1, m0, q0000
    pshufd               m2, m0, q1111
    pshufd               m5, m0, q2222
    pshufd               m0, m0, q3333
    mova                m10, m1
    mova                m11, m2
    mova                m12, m5
    mova                m13, m0
%else
%if WIN64
    %define tmp rsp
%else
    %define tmp rsp-104 ; red zone
%endif
    shl                  wd, 6
    mov                  r7, srcq
    mov                  r8, dstq
    lea                  wd, [wq+hq-(1<<8)]
    pshufd              m10, m0, q0000
    pshufd              m11, m0, q1111
    pshufd              m12, m0, q2222
    pshufd              m13, m0, q3333
    mova         [tmp+16*5], m15
%endif
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova         [tmp+16*1], m0
    mova         [tmp+16*2], m1
    mova         [tmp+16*3], m2
    mova         [tmp+16*4], m3
%macro PUT_8TAP_HV_H 4-5 m14 ; dst/src+0, src+8, tmp, shift, [pd_512]
    pshufb              m%3, m%1, m8 ; 0 1 1 2 2 3 3 4
    pshufb              m%1, m9      ; 2 3 3 4 4 5 5 6
    pmaddwd             m%3, m10
    pmaddwd             m%1, m11
    paddd               m%3, %5
    paddd               m%1, m%3
    pshufb              m%3, m%2, m8 ; 4 5 5 6 6 7 7 8
    pshufb              m%2, m9      ; 6 7 7 8 8 9 9 a
    pmaddwd             m%3, m12
    pmaddwd             m%2, m13
    paddd               m%1, m%3
    paddd               m%1, m%2
    psrad               m%1, %4
%endmacro
.hv_w4_loop0:
%if ARCH_X86_64
    mova                m14, [pd_512]
%endif
    movu                 m4, [srcq+ssq*0+0]
    movu                 m1, [srcq+ssq*0+8]
    movu                 m5, [srcq+ssq*1+0]
    movu                 m2, [srcq+ssq*1+8]
    movu                 m6, [srcq+ssq*2+0]
    movu                 m3, [srcq+ssq*2+8]
    add                srcq, r6
    PUT_8TAP_HV_H         4, 1, 0, 10
    PUT_8TAP_HV_H         5, 2, 0, 10
    PUT_8TAP_HV_H         6, 3, 0, 10
    movu                 m7, [srcq+ssq*0+0]
    movu                 m2, [srcq+ssq*0+8]
    movu                 m1, [srcq+ssq*1+0]
    movu                 m3, [srcq+ssq*1+8]
    PUT_8TAP_HV_H         7, 2, 0, 10
    PUT_8TAP_HV_H         1, 3, 0, 10
    movu                 m2, [srcq+ssq*2+0]
    movu                 m3, [srcq+ssq*2+8]
    add                srcq, r6
    PUT_8TAP_HV_H         2, 3, 0, 10
    packssdw             m4, m7      ; 0 3
    packssdw             m5, m1      ; 1 4
    movu                 m0, [srcq+ssq*0+0]
    movu                 m1, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         0, 1, 3, 10
    packssdw             m6, m2      ; 2 5
    packssdw             m7, m0      ; 3 6
    punpcklwd            m1, m4, m5  ; 01
    punpckhwd            m4, m5      ; 34
    punpcklwd            m2, m5, m6  ; 12
    punpckhwd            m5, m6      ; 45
    punpcklwd            m3, m6, m7  ; 23
    punpckhwd            m6, m7      ; 56
%if ARCH_X86_32
    jmp .hv_w4_loop_start
.hv_w4_loop:
    mova                 m1, [tmp+16*6]
    mova                 m2, m15
.hv_w4_loop_start:
    mova                 m7, [tmp+16*1]
    pmaddwd              m1, m7      ; a0
    pmaddwd              m2, m7      ; b0
    mova                 m7, [tmp+16*2]
    mova         [tmp+16*6], m3
    pmaddwd              m3, m7      ; a1
    mova                m15, m4
    pmaddwd              m4, m7      ; b1
    mova                 m7, [tmp+16*3]
    paddd                m1, m3
    paddd                m2, m4
    mova                 m3, m5
    pmaddwd              m5, m7      ; a2
    mova                 m4, m6
    pmaddwd              m6, m7      ; b2
    paddd                m1, m5
    paddd                m2, m6
    movu                 m7, [srcq+ssq*1+0]
    movu                 m5, [srcq+ssq*1+8]
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_HV_H         7, 5, 6, 10
    packssdw             m0, m7      ; 6 7
    mova         [tmp+16*0], m0
    movu                 m0, [srcq+ssq*0+0]
    movu                 m5, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         0, 5, 6, 10
    mova                 m6, [tmp+16*0]
    packssdw             m7, m0      ; 7 8
    punpcklwd            m5, m6, m7  ; 67
    punpckhwd            m6, m7      ; 78
    pmaddwd              m7, m5, [tmp+16*4]
    paddd                m1, m7      ; a3
    pmaddwd              m7, m6, [tmp+16*4]
    paddd                m2, m7      ; b3
    psrad                m1, 9
    psrad                m2, 9
    packssdw             m1, m2
    pxor                 m7, m7
    pmaxsw               m1, m7
    pavgw                m7, m1
    pminsw               m7, [tmp+16*5]
    movq       [dstq+dsq*0], m7
    movhps     [dstq+dsq*1], m7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
%if STACK_ALIGNMENT < 16
    mov                srcq, [esp+4*61]
    mov                dstq, [esp+4*62]
    add                srcq, 8
    add                dstq, 8
    mov          [esp+4*61], srcq
    mov          [esp+4*62], dstq
%else
    mov                srcq, srcmp
    mov                dstq, dstmp
    add                srcq, 8
    add                dstq, 8
    mov               srcmp, srcq
    mov               dstmp, dstq
%endif
    movzx                hd, ww
    sub                  wd, 1<<16
%else
.hv_w4_loop:
    mova                m15, [tmp+16*1]
    pmaddwd             m14, m15, m1 ; a0
    pmaddwd             m15, m2      ; b0
    mova                 m7, [tmp+16*2]
    mova                 m1, m3
    pmaddwd              m3, m7      ; a1
    mova                 m2, m4
    pmaddwd              m4, m7      ; b1
    mova                 m7, [tmp+16*3]
    paddd               m14, m3
    paddd               m15, m4
    mova                 m3, m5
    pmaddwd              m5, m7      ; a2
    mova                 m4, m6
    pmaddwd              m6, m7      ; b2
    paddd               m14, m5
    paddd               m15, m6
    movu                 m7, [srcq+ssq*1+0]
    movu                 m5, [srcq+ssq*1+8]
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_HV_H         7, 5, 6, 10, [pd_512]
    packssdw             m0, m7      ; 6 7
    mova         [tmp+16*0], m0
    movu                 m0, [srcq+ssq*0+0]
    movu                 m5, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         0, 5, 6, 10, [pd_512]
    mova                 m6, [tmp+16*0]
    packssdw             m7, m0      ; 7 8
    punpcklwd            m5, m6, m7  ; 67
    punpckhwd            m6, m7      ; 78
    pmaddwd              m7, m5, [tmp+16*4]
    paddd               m14, m7      ; a3
    pmaddwd              m7, m6, [tmp+16*4]
    paddd               m15, m7      ; b3
    psrad               m14, 9
    psrad               m15, 9
    packssdw            m14, m15
    pxor                 m7, m7
    pmaxsw              m14, m7
    pavgw                m7, m14
    pminsw               m7, [tmp+16*5]
    movq       [dstq+dsq*0], m7
    movhps     [dstq+dsq*1], m7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    add                  r7, 8
    add                  r8, 8
    movzx                hd, wb
    mov                srcq, r7
    mov                dstq, r8
    sub                  wd, 1<<8
%endif
    jg .hv_w4_loop0
    RET
%undef tmp

%if ARCH_X86_32
DECLARE_REG_TMP 2, 1, 6, 4
%elif WIN64
DECLARE_REG_TMP 6, 4, 7, 4
%else
DECLARE_REG_TMP 6, 7, 7, 8
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

%if ARCH_X86_32
cglobal prep_8tap_16bpc, 0, 7, 8, tmp, src, ss, w, h, mx, my
%define mxb r0b
%define mxd r0
%define mxq r0
%define myb r2b
%define myd r2
%define myq r2
%else
cglobal prep_8tap_16bpc, 4, 8, 0, tmp, src, ss, w, h, mx, my
%endif
%define base t2-prep_ssse3
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    LEA                  t2, prep_ssse3
    movifnidn            wd, wm
    movifnidn          srcq, srcmp
    test                mxd, 0xf00
    jnz .h
    movifnidn            hd, hm
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    mov                 myd, r7m ; bitdepth_max
    movzx                wd, word [base+prep_ssse3_table+wq*2]
    mova                 m5, [base+pw_8192]
    shr                 myd, 11
    add                  wq, t2
    movddup              m4, [base+prep_mul+myq*8]
    movifnidn           ssq, ssmp
    movifnidn          tmpq, tmpmp
    lea                  r6, [ssq*3]
%if WIN64
    pop                  r7
%endif
    jmp                  wq
.h:
    test                myd, 0xf00
    jnz .hv
    movifnidn           ssq, r2mp
    movifnidn            hd, r4m
    movddup              m5, [base+prep_8tap_1d_rnd]
    cmp                  wd, 4
    jne .h_w8
    movzx               mxd, mxb
    movq                 m0, [base+subpel_filters+mxq*8]
    mova                 m3, [base+spel_h_shufA]
    mova                 m4, [base+spel_h_shufB]
    movifnidn          tmpq, tmpmp
    sub                srcq, 2
    WIN64_SPILL_XMM       8
    punpcklbw            m0, m0
    psraw                m0, 8
    test          dword r7m, 0x800
    jnz .h_w4_12bpc
    psllw                m0, 2
.h_w4_12bpc:
    pshufd               m6, m0, q1111
    pshufd               m7, m0, q2222
.h_w4_loop:
    movu                 m1, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
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
    add                tmpq, 16
    sub                  hd, 2
    jg .h_w4_loop
    RET
.h_w8:
    WIN64_SPILL_XMM      11
    shr                 mxd, 16
    movq                 m2, [base+subpel_filters+mxq*8]
    mova                 m4, [base+spel_h_shufA]
    mova                 m6, [base+spel_h_shufB]
    movifnidn          tmpq, r0mp
    add                  wd, wd
    punpcklbw            m2, m2
    add                srcq, wq
    psraw                m2, 8
    add                tmpq, wq
    neg                  wq
    test          dword r7m, 0x800
    jnz .h_w8_12bpc
    psllw                m2, 2
.h_w8_12bpc:
    pshufd               m7, m2, q0000
%if ARCH_X86_32
    ALLOC_STACK       -16*3
    pshufd               m0, m2, q1111
    pshufd               m1, m2, q2222
    pshufd               m2, m2, q3333
    mova                 m8, m0
    mova                 m9, m1
    mova                m10, m2
%else
    pshufd               m8, m2, q1111
    pshufd               m9, m2, q2222
    pshufd              m10, m2, q3333
%endif
.h_w8_loop0:
    mov                  r6, wq
.h_w8_loop:
    movu                 m0, [srcq+r6- 6]
    movu                 m1, [srcq+r6+ 2]
    pshufb               m2, m0, m4  ; 0 1 1 2 2 3 3 4
    pshufb               m0, m6      ; 2 3 3 4 4 5 5 6
    pmaddwd              m2, m7      ; abcd0
    pmaddwd              m0, m8      ; abcd1
    pshufb               m3, m1, m4  ; 4 5 5 6 6 7 7 8
    pshufb               m1, m6      ; 6 7 7 8 8 9 9 a
    paddd                m2, m5
    paddd                m0, m2
    pmaddwd              m2, m9, m3  ; abcd2
    pmaddwd              m3, m7      ; efgh0
    paddd                m0, m2
    pmaddwd              m2, m10, m1 ; abcd3
    pmaddwd              m1, m8      ; efgh1
    paddd                m0, m2
    movu                 m2, [srcq+r6+10]
    paddd                m3, m5
    paddd                m1, m3
    pshufb               m3, m2, m4  ; a b b c c d d e
    pshufb               m2, m6      ; 8 9 9 a a b b c
    pmaddwd              m3, m9      ; efgh2
    pmaddwd              m2, m10     ; efgh3
    paddd                m1, m3
    paddd                m1, m2
    psrad                m0, 4
    psrad                m1, 4
    packssdw             m0, m1
    mova          [tmpq+r6], m0
    add                  r6, 16
    jl .h_w8_loop
    add                srcq, ssq
    sub                tmpq, wq
    dec                  hd
    jg .h_w8_loop0
    RET
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    movq                 m3, [base+subpel_filters+myq*8]
%if STACK_ALIGNMENT < 16
    %xdefine           rstk  rsp
%else
    %assign stack_offset stack_offset - stack_size_padded
%endif
    WIN64_SPILL_XMM      15
    movddup              m7, [base+prep_8tap_1d_rnd]
    movifnidn           ssq, r2mp
    movifnidn          tmpq, r0mp
    punpcklbw            m3, m3
    psraw                m3, 8 ; sign-extend
    test          dword r7m, 0x800
    jnz .v_12bpc
    psllw                m3, 2
.v_12bpc:
%if ARCH_X86_32
    ALLOC_STACK       -16*7
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova                 m8, m0
    mova                 m9, m1
    mova                m10, m2
    mova                m11, m3
%else
    pshufd               m8, m3, q0000
    pshufd               m9, m3, q1111
    pshufd              m10, m3, q2222
    pshufd              m11, m3, q3333
%endif
    lea                  r6, [ssq*3]
    sub                srcq, r6
    mov                 r6d, wd
    shl                  wd, 6
    mov                  r5, srcq
%if ARCH_X86_64
    mov                  r7, tmpq
%elif STACK_ALIGNMENT < 16
    mov          [esp+4*29], tmpq
%endif
    lea                  wd, [wq+hq-(1<<8)]
.v_loop0:
    movq                 m1, [srcq+ssq*0]
    movq                 m2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movq                 m3, [srcq+ssq*0]
    movq                 m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movq                 m5, [srcq+ssq*0]
    movq                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movq                 m0, [srcq+ssq*0]
    punpcklwd            m1, m2      ; 01
    punpcklwd            m2, m3      ; 12
    punpcklwd            m3, m4      ; 23
    punpcklwd            m4, m5      ; 34
    punpcklwd            m5, m6      ; 45
    punpcklwd            m6, m0      ; 56
%if ARCH_X86_32
    jmp .v_loop_start
.v_loop:
    mova                 m1, m12
    mova                 m2, m13
    mova                 m3, m14
.v_loop_start:
    pmaddwd              m1, m8      ; a0
    pmaddwd              m2, m8      ; b0
    mova                m12, m3
    mova                m13, m4
    pmaddwd              m3, m9      ; a1
    pmaddwd              m4, m9      ; b1
    paddd                m1, m3
    paddd                m2, m4
    mova                m14, m5
    mova                 m4, m6
    pmaddwd              m5, m10     ; a2
    pmaddwd              m6, m10     ; b2
    paddd                m1, m5
    paddd                m2, m6
    movq                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklwd            m5, m0, m6  ; 67
    movq                 m0, [srcq+ssq*0]
    pmaddwd              m3, m11, m5 ; a3
    punpcklwd            m6, m0      ; 78
    paddd                m1, m7
    paddd                m1, m3
    pmaddwd              m3, m11, m6 ; b3
    paddd                m2, m7
    paddd                m2, m3
    psrad                m1, 4
    psrad                m2, 4
    packssdw             m1, m2
    movq        [tmpq+r6*0], m1
    movhps      [tmpq+r6*2], m1
    lea                tmpq, [tmpq+r6*4]
    sub                  hd, 2
    jg .v_loop
%if STACK_ALIGNMENT < 16
    mov                tmpq, [esp+4*29]
    add                  r5, 8
    add                tmpq, 8
    mov                srcq, r5
    mov          [esp+4*29], tmpq
%else
    mov                tmpq, tmpmp
    add                  r5, 8
    add                tmpq, 8
    mov                srcq, r5
    mov               tmpmp, tmpq
%endif
%else
.v_loop:
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
    paddd               m13, m6
    movq                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklwd            m5, m0, m6  ; 67
    movq                 m0, [srcq+ssq*0]
    pmaddwd             m14, m11, m5 ; a3
    punpcklwd            m6, m0      ; 78
    paddd               m12, m7
    paddd               m12, m14
    pmaddwd             m14, m11, m6 ; b3
    paddd               m13, m7
    paddd               m13, m14
    psrad               m12, 4
    psrad               m13, 4
    packssdw            m12, m13
    movq        [tmpq+r6*0], m12
    movhps      [tmpq+r6*2], m12
    lea                tmpq, [tmpq+r6*4]
    sub                  hd, 2
    jg .v_loop
    add                  r5, 8
    add                  r7, 8
    mov                srcq, r5
    mov                tmpq, r7
%endif
    movzx                hd, wb
    sub                  wd, 1<<8
    jg .v_loop0
    RET
.hv:
%if STACK_ALIGNMENT < 16
    %xdefine           rstk  rsp
%else
    %assign stack_offset stack_offset - stack_size_padded
%endif
    movzx               t3d, mxb
    shr                 mxd, 16
    cmp                  wd, 4
    cmove               mxd, t3d
    movifnidn            hd, r4m
    movq                 m2, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    movq                 m3, [base+subpel_filters+myq*8]
%if ARCH_X86_32
    mov                 ssq, r2mp
    mov                tmpq, r0mp
    mova                 m0, [base+spel_h_shufA]
    mova                 m1, [base+spel_h_shufB]
    mova                 m4, [base+prep_8tap_2d_rnd]
    ALLOC_STACK      -16*14
    mova                 m8, m0
    mova                 m9, m1
    mova                m14, m4
%else
%if WIN64
    ALLOC_STACK        16*6, 16
%endif
    mova                 m8, [base+spel_h_shufA]
    mova                 m9, [base+spel_h_shufB]
%endif
    pxor                 m0, m0
    punpcklbw            m0, m2
    punpcklbw            m3, m3
    psraw                m0, 4
    psraw                m3, 8
    test          dword r7m, 0x800
    jz .hv_10bpc
    psraw                m0, 2
.hv_10bpc:
    lea                  r6, [ssq*3]
    sub                srcq, 6
    sub                srcq, r6
    mov                 r6d, wd
    shl                  wd, 6
    mov                  r5, srcq
%if ARCH_X86_32
    %define             tmp  esp+16*8
%if STACK_ALIGNMENT < 16
    mov          [esp+4*61], tmpq
%endif
    pshufd               m1, m0, q0000
    pshufd               m2, m0, q1111
    pshufd               m5, m0, q2222
    pshufd               m0, m0, q3333
    mova                m10, m1
    mova                m11, m2
    mova                m12, m5
    mova                m13, m0
%else
%if WIN64
    %define             tmp  rsp
%else
    %define             tmp  rsp-88 ; red zone
%endif
    mov                  r7, tmpq
    pshufd              m10, m0, q0000
    pshufd              m11, m0, q1111
    pshufd              m12, m0, q2222
    pshufd              m13, m0, q3333
%endif
    lea                  wd, [wq+hq-(1<<8)]
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova         [tmp+16*1], m0
    mova         [tmp+16*2], m1
    mova         [tmp+16*3], m2
    mova         [tmp+16*4], m3
.hv_loop0:
%if ARCH_X86_64
    mova                m14, [prep_8tap_2d_rnd]
%endif
    movu                 m4, [srcq+ssq*0+0]
    movu                 m1, [srcq+ssq*0+8]
    movu                 m5, [srcq+ssq*1+0]
    movu                 m2, [srcq+ssq*1+8]
    lea                srcq, [srcq+ssq*2]
    movu                 m6, [srcq+ssq*0+0]
    movu                 m3, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         4, 1, 0, 6
    PUT_8TAP_HV_H         5, 2, 0, 6
    PUT_8TAP_HV_H         6, 3, 0, 6
    movu                 m7, [srcq+ssq*1+0]
    movu                 m2, [srcq+ssq*1+8]
    lea                srcq, [srcq+ssq*2]
    movu                 m1, [srcq+ssq*0+0]
    movu                 m3, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         7, 2, 0, 6
    PUT_8TAP_HV_H         1, 3, 0, 6
    movu                 m2, [srcq+ssq*1+0]
    movu                 m3, [srcq+ssq*1+8]
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_HV_H         2, 3, 0, 6
    packssdw             m4, m7      ; 0 3
    packssdw             m5, m1      ; 1 4
    movu                 m0, [srcq+ssq*0+0]
    movu                 m1, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         0, 1, 3, 6
    packssdw             m6, m2      ; 2 5
    packssdw             m7, m0      ; 3 6
    punpcklwd            m1, m4, m5  ; 01
    punpckhwd            m4, m5      ; 34
    punpcklwd            m2, m5, m6  ; 12
    punpckhwd            m5, m6      ; 45
    punpcklwd            m3, m6, m7  ; 23
    punpckhwd            m6, m7      ; 56
%if ARCH_X86_32
    jmp .hv_loop_start
.hv_loop:
    mova                 m1, [tmp+16*5]
    mova                 m2, m15
.hv_loop_start:
    mova                 m7, [tmp+16*1]
    pmaddwd              m1, m7      ; a0
    pmaddwd              m2, m7      ; b0
    mova                 m7, [tmp+16*2]
    mova         [tmp+16*5], m3
    pmaddwd              m3, m7      ; a1
    mova                m15, m4
    pmaddwd              m4, m7      ; b1
    mova                 m7, [tmp+16*3]
    paddd                m1, m14
    paddd                m2, m14
    paddd                m1, m3
    paddd                m2, m4
    mova                 m3, m5
    pmaddwd              m5, m7      ; a2
    mova                 m4, m6
    pmaddwd              m6, m7      ; b2
    paddd                m1, m5
    paddd                m2, m6
    movu                 m7, [srcq+ssq*1+0]
    movu                 m5, [srcq+ssq*1+8]
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_HV_H         7, 5, 6, 6
    packssdw             m0, m7      ; 6 7
    mova         [tmp+16*0], m0
    movu                 m0, [srcq+ssq*0+0]
    movu                 m5, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         0, 5, 6, 6
    mova                 m6, [tmp+16*0]
    packssdw             m7, m0      ; 7 8
    punpcklwd            m5, m6, m7  ; 67
    punpckhwd            m6, m7      ; 78
    pmaddwd              m7, m5, [tmp+16*4]
    paddd                m1, m7      ; a3
    pmaddwd              m7, m6, [tmp+16*4]
    paddd                m2, m7      ; b3
    psrad                m1, 6
    psrad                m2, 6
    packssdw             m1, m2
    movq        [tmpq+r6*0], m1
    movhps      [tmpq+r6*2], m1
    lea                tmpq, [tmpq+r6*4]
    sub                  hd, 2
    jg .hv_loop
%if STACK_ALIGNMENT < 16
    mov                tmpq, [esp+4*61]
    add                  r5, 8
    add                tmpq, 8
    mov                srcq, r5
    mov          [esp+4*61], tmpq
%else
    mov                tmpq, tmpmp
    add                  r5, 8
    add                tmpq, 8
    mov                srcq, r5
    mov               tmpmp, tmpq
%endif
%else
.hv_loop:
    mova                m15, [tmp+16*1]
    mova                 m7, [prep_8tap_2d_rnd]
    pmaddwd             m14, m15, m1 ; a0
    pmaddwd             m15, m2      ; b0
    paddd               m14, m7
    paddd               m15, m7
    mova                 m7, [tmp+16*2]
    mova                 m1, m3
    pmaddwd              m3, m7      ; a1
    mova                 m2, m4
    pmaddwd              m4, m7      ; b1
    mova                 m7, [tmp+16*3]
    paddd               m14, m3
    paddd               m15, m4
    mova                 m3, m5
    pmaddwd              m5, m7      ; a2
    mova                 m4, m6
    pmaddwd              m6, m7      ; b2
    paddd               m14, m5
    paddd               m15, m6
    movu                 m7, [srcq+ssq*1+0]
    movu                 m5, [srcq+ssq*1+8]
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_HV_H         7, 5, 6, 6, [prep_8tap_2d_rnd]
    packssdw             m0, m7      ; 6 7
    mova         [tmp+16*0], m0
    movu                 m0, [srcq+ssq*0+0]
    movu                 m5, [srcq+ssq*0+8]
    PUT_8TAP_HV_H         0, 5, 6, 6, [prep_8tap_2d_rnd]
    mova                 m6, [tmp+16*0]
    packssdw             m7, m0      ; 7 8
    punpcklwd            m5, m6, m7  ; 67
    punpckhwd            m6, m7      ; 78
    pmaddwd              m7, m5, [tmp+16*4]
    paddd               m14, m7      ; a3
    pmaddwd              m7, m6, [tmp+16*4]
    paddd               m15, m7      ; b3
    psrad               m14, 6
    psrad               m15, 6
    packssdw            m14, m15
    movq        [tmpq+r6*0], m14
    movhps      [tmpq+r6*2], m14
    lea                tmpq, [tmpq+r6*4]
    sub                  hd, 2
    jg .hv_loop
    add                  r5, 8
    add                  r7, 8
    mov                srcq, r5
    mov                tmpq, r7
%endif
    movzx                hd, wb
    sub                  wd, 1<<8
    jg .hv_loop0
    RET
%undef tmp

%macro movifprep 2
 %if isprep
    mov %1, %2
 %endif
%endmacro

%macro SAVE_REG 1
 %xdefine r%1_save  r%1
 %xdefine r%1q_save r%1q
 %xdefine r%1d_save r%1d
 %if ARCH_X86_32
  %define r%1m_save [rstk+stack_offset+(%1+1)*4]
 %endif
%endmacro

%macro LOAD_REG 1
 %xdefine r%1  r%1_save
 %xdefine r%1q r%1q_save
 %xdefine r%1d r%1d_save
 %if ARCH_X86_32
  %define r%1m r%1m_save
 %endif
 %undef r%1d_save
 %undef r%1q_save
 %undef r%1_save
%endmacro

%macro REMAP_REG 2-3
 %xdefine r%1  r%2
 %xdefine r%1q r%2q
 %xdefine r%1d r%2d
 %if ARCH_X86_32
  %if %3 == 0
   %xdefine r%1m r%2m
  %else
   %define r%1m [rstk+stack_offset+(%1+1)*4]
  %endif
 %endif
%endmacro

%macro MCT_8TAP_SCALED_REMAP_REGS_TO_PREV 0
 %if isprep
  %if ARCH_X86_64
   SAVE_REG 14
   %assign %%i 14
   %rep 14
    %assign %%j %%i-1
    REMAP_REG %%i, %%j
    %assign %%i %%i-1
   %endrep
  %else
   SAVE_REG 5
   %assign %%i 5
   %rep 5
    %assign %%j %%i-1
    REMAP_REG %%i, %%j, 0
    %assign %%i %%i-1
   %endrep
  %endif
 %endif
%endmacro

%macro MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT 0
 %if isprep
  %assign %%i 1
  %if ARCH_X86_64
   %rep 13
    %assign %%j %%i+1
    REMAP_REG %%i, %%j
    %assign %%i %%i+1
   %endrep
   LOAD_REG 14
  %else
   %rep 4
    %assign %%j %%i+1
    REMAP_REG %%i, %%j, 1
    %assign %%i %%i+1
   %endrep
   LOAD_REG 5
  %endif
 %endif
%endmacro

%macro MC_8TAP_SCALED_RET 0-1 1 ; leave_mapping_unchanged
    MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT
    RET
 %if %1
    MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %endif
%endmacro

%if ARCH_X86_32
 %macro MC_4TAP_SCALED_H 1 ; dst_mem
    movu                 m7, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*1]
    movu                 m5, [r4  +ssq*0]
    movu                 m6, [r4  +ssq*1]
    lea                srcq, [srcq+ssq*2]
    lea                  r4, [r4  +ssq*2]
    REPX    {pshufb x, m12}, m7, m2
    REPX   {pmaddwd x, m13}, m7, m2
    REPX    {pshufb x, m14}, m5, m6
    REPX   {pmaddwd x, m15}, m5, m6
    phaddd               m7, m5
    phaddd               m2, m6
    mova                 m5, [esp+0x00]
    movd                 m6, [esp+0x10]
    paddd                m7, m5
    paddd                m2, m5
    psrad                m7, m6
    psrad                m2, m6
    packssdw             m7, m2
    mova           [stk+%1], m7
 %endmacro
%endif

%if ARCH_X86_64
 %macro MC_8TAP_SCALED_H 8 ; dst, tmp[0-6]
    movu                m%1, [srcq+ r4*2]
    movu                m%2, [srcq+ r6*2]
    movu                m%3, [srcq+ r7*2]
    movu                m%4, [srcq+ r9*2]
    movu                m%5, [srcq+r10*2]
    movu                m%6, [srcq+r11*2]
    movu                m%7, [srcq+r13*2]
    movu                m%8, [srcq+ rX*2]
    add                srcq, ssq
    pmaddwd             m%1, [stk+0x10]
    pmaddwd             m%2, [stk+0x20]
    pmaddwd             m%3, [stk+0x30]
    pmaddwd             m%4, [stk+0x40]
    pmaddwd             m%5, [stk+0x50]
    pmaddwd             m%6, [stk+0x60]
    pmaddwd             m%7, [stk+0x70]
    pmaddwd             m%8, [stk+0x80]
    phaddd              m%1, m%2
    phaddd              m%3, m%4
    phaddd              m%5, m%6
    phaddd              m%7, m%8
    phaddd              m%1, m%3
    phaddd              m%5, m%7
    paddd               m%1, hround
    paddd               m%5, hround
    psrad               m%1, m12
    psrad               m%5, m12
    packssdw            m%1, m%5
 %endmacro
%else
 %macro MC_8TAP_SCALED_H 2-3 1 ; weights_mem_start, h_mem, load_fh_offsets
  %if %3 == 1
    mov                  r0, [stk+ 0]
    mov                  rX, [stk+ 4]
    mov                  r4, [stk+ 8]
    mov                  r5, [stk+12]
  %endif
    movu                 m0, [srcq+r0*2]
    movu                 m1, [srcq+rX*2]
    movu                 m2, [srcq+r4*2]
    movu                 m3, [srcq+r5*2]
    mov                  r0, [stk+16]
    mov                  rX, [stk+20]
    mov                  r4, [stk+24]
    mov                  r5, [stk+28]
    pmaddwd              m0, [stk+%1+0x00]
    pmaddwd              m1, [stk+%1+0x10]
    pmaddwd              m2, [stk+%1+0x20]
    pmaddwd              m3, [stk+%1+0x30]
    phaddd               m0, m1
    phaddd               m2, m3
    movu                 m4, [srcq+r0*2]
    movu                 m5, [srcq+rX*2]
    movu                 m6, [srcq+r4*2]
    movu                 m7, [srcq+r5*2]
    add                srcq, ssq
    pmaddwd              m4, [stk+%1+0xa0]
    pmaddwd              m5, [stk+%1+0xb0]
    pmaddwd              m6, [stk+%1+0xc0]
    pmaddwd              m7, [stk+%1+0xd0]
    phaddd               m4, m5
    phaddd               m6, m7
    phaddd               m0, m2
    phaddd               m4, m6
    paddd                m0, hround
    paddd                m4, hround
    psrad                m0, m12
    psrad                m4, m12
    packssdw             m0, m4
  %if %2 != 0
    mova           [stk+%2], m0
  %endif
 %endmacro
%endif

%macro MC_8TAP_SCALED 1
%ifidn %1, put
 %assign isput  1
 %assign isprep 0
 %if ARCH_X86_64
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal put_8tap_scaled_16bpc, 2, 15, 16, 0x1c0, dst, ds, src, ss, w, h, mx, my, dx, dy, pxmax
  %else
cglobal put_8tap_scaled_16bpc, 2, 14, 16, 0x1c0, dst, ds, src, ss, w, h, mx, my, dx, dy, pxmax
  %endif
 %else ; ARCH_X86_32
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal put_8tap_scaled_16bpc, 0, 7, 8, 0x200, dst, ds, src, ss, w, h, mx, my, dx, dy, pxmax
  %else
cglobal put_8tap_scaled_16bpc, 0, 7, 8, -0x200-0x30, dst, ds, src, ss, w, h, mx, my, dx, dy, pxmax
  %endif
 %endif
 %xdefine base_reg r12
%else ; prep
 %assign isput  0
 %assign isprep 1
 %if ARCH_X86_64
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal prep_8tap_scaled_16bpc, 2, 15, 16, 0x1c0, tmp, src, ss, w, h, mx, my, dx, dy, pxmax
   %xdefine tmp_stridem r14q
  %else
cglobal prep_8tap_scaled_16bpc, 2, 14, 16, 0x1c0, tmp, src, ss, w, h, mx, my, dx, dy, pxmax
   %define tmp_stridem qword [stk+0x138]
  %endif
  %xdefine base_reg r11
 %else ; ARCH_X86_32
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal prep_8tap_scaled_16bpc, 0, 7, 8, 0x200, tmp, src, ss, w, h, mx, my, dx, dy, pxmax
  %else
cglobal prep_8tap_scaled_16bpc, 0, 6, 8, 0x200, tmp, src, ss, w, h, mx, my, dx, dy, pxmax
  %endif
  %define tmp_stridem dword [stk+0x138]
 %endif
%endif
%if ARCH_X86_32
    mov         [esp+0x1f0], t0d
    mov         [esp+0x1f4], t1d
 %if isput && required_stack_alignment > STACK_ALIGNMENT
    mov                dstd, dstm
    mov                 dsd, dsm
    mov                srcd, srcm
    mov                 ssd, ssm
    mov                  hd, hm
    mov                  r4, mxm
  %define r0m  [esp+0x200]
  %define dsm  [esp+0x204]
  %define dsmp dsm
  %define r1m  dsm
  %define r2m  [esp+0x208]
  %define ssm  [esp+0x20c]
  %define r3m  ssm
  %define hm   [esp+0x210]
  %define mxm  [esp+0x214]
    mov                 r0m, dstd
    mov                 dsm, dsd
    mov                 r2m, srcd
    mov                 ssm, ssd
    mov                  hm, hd
    mov                  r0, mym
    mov                  r1, dxm
    mov                  r2, dym
  %define mym    [esp+0x218]
  %define dxm    [esp+0x21c]
  %define dym    [esp+0x220]
    mov                 mxm, r4
    mov                 mym, r0
    mov                 dxm, r1
    mov                 dym, r2
    tzcnt                wd, wm
 %endif
 %if isput
    mov                  r3, pxmaxm
  %define pxmaxm r3
 %else
    mov                  r2, pxmaxm
 %endif
 %if isprep && required_stack_alignment > STACK_ALIGNMENT
  %xdefine base_reg r5
 %else
  %xdefine base_reg r6
 %endif
%endif
    LEA            base_reg, %1_8tap_scaled_16bpc_ssse3
%xdefine base base_reg-%1_8tap_scaled_16bpc_ssse3
%if ARCH_X86_64 || isprep || required_stack_alignment <= STACK_ALIGNMENT
    tzcnt                wd, wm
%endif
%if ARCH_X86_64
 %if isput
    mov                 r7d, pxmaxm
 %endif
%else
 %define m8  m0
 %define m9  m1
 %define m14 m4
 %define m15 m3
%endif
    movd                 m8, dxm
    movd                m14, mxm
%if isput
    movd                m15, pxmaxm
%endif
    pshufd               m8, m8, q0000
    pshufd              m14, m14, q0000
%if isput
    pshuflw             m15, m15, q0000
    punpcklqdq          m15, m15
%endif
%if isprep
 %if UNIX64
    mov                 r5d, t0d
  DECLARE_REG_TMP 5, 7
 %endif
 %if ARCH_X86_64
    mov                 r6d, pxmaxm
 %endif
%endif
%if ARCH_X86_64
    mov                 dyd, dym
%endif
%if isput
 %if WIN64
    mov                 r8d, hm
  DEFINE_ARGS dst, ds, src, ss, w, _, _, my, h, dy, ss3
  %define hm r5m
  %define dxm r8m
 %elif ARCH_X86_64
  DEFINE_ARGS dst, ds, src, ss, w, h, _, my, dx, dy, ss3
  %define hm r6m
 %else
 %endif
 %if ARCH_X86_64
  %if required_stack_alignment > STACK_ALIGNMENT
   %define dsm [rsp+0x138]
   %define rX r1
   %define rXd r1d
  %else
   %define dsm dsq
   %define rX r14
   %define rXd r14d
  %endif
 %else
  %define rX r1
 %endif
%else ; prep
 %if WIN64
    mov                 r7d, hm
  DEFINE_ARGS tmp, src, ss, w, _, _, my, h, dy, ss3
  %define hm r4m
  %define dxm r7m
 %elif ARCH_X86_64
  DEFINE_ARGS tmp, src, ss, w, h, _, my, dx, dy, ss3
  %xdefine hm r7m
 %endif
 MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %if ARCH_X86_64
  %define rX r14
  %define rXd r14d
 %else
  %define rX r3
 %endif
%endif
%if ARCH_X86_64
    shr                 r7d, 11
    mova                m10, [base+pd_0x3ff]
    movddup             m11, [base+s_8tap_h_rnd+r7*8]
    movd                m12, [base+s_8tap_h_sh+r7*4]
 %if isput
    movddup             m13, [base+put_s_8tap_v_rnd+r7*8]
    movd                 m7, [base+put_s_8tap_v_sh+r7*4]
  %define pxmaxm [rsp]
    mova             pxmaxm, m15
    punpcklqdq          m12, m7
 %endif
    lea                ss3q, [ssq*3]
    movzx               r7d, t1b
    shr                 t1d, 16
    cmp                  hd, 6
    cmovs               t1d, r7d
    sub                srcq, ss3q
%else
 %define m10    [base+pd_0x3ff]
 %define m11    [esp+0x00]
 %define m12    [esp+0x10]
    shr                  r3, 11
    movddup              m1, [base+s_8tap_h_rnd+r3*8]
    movd                 m2, [base+s_8tap_h_sh+r3*4]
 %if isput
  %define m13    [esp+0x20]
  %define pxmaxm [esp+0x30]
  %define stk esp+0x40
    movddup              m5, [base+put_s_8tap_v_rnd+r3*8]
    movd                 m6, [base+put_s_8tap_v_sh+r3*4]
    mova             pxmaxm, m15
    punpcklqdq           m2, m6
    mova                m13, m5
 %else
  %define m13 [base+pd_m524256]
 %endif
    mov                 ssd, ssm
    mova                m11, m1
    mova                m12, m2
 MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT
    mov                  r1, [esp+0x1f4]
    lea                  r0, [ssd*3]
    movzx                r2, r1b
    shr                  r1, 16
    cmp            dword hm, 6
    cmovs                r1, r2
    mov         [esp+0x1f4], r1
 %if isprep
    mov                  r1, r1m
 %endif
    mov                  r2, r2m
    sub                srcq, r0
 MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %define ss3q r0
 %define myd r4
 %define dyd dword dym
 %define hd  dword hm
%endif
    cmp                 dyd, 1024
    je .dy1
    cmp                 dyd, 2048
    je .dy2
    movzx                wd, word [base+%1_8tap_scaled_ssse3_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%if isput
.w2:
 %if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    sub                srcq, 2
    movd                m15, t0d
 %else
    movzx                r4, byte [esp+0x1f0]
    sub                srcq, 2
    movd                m15, r4
 %endif
    pxor                 m9, m9
    punpckldq            m9, m8
    paddd               m14, m9 ; mx+dx*[0-1]
 %if ARCH_X86_64
    mova                 m9, [base+pd_0x4000]
 %endif
    pshufd              m15, m15, q0000
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    movd                r4d, m15
    pshufd              m15, m15, q0321
 %if ARCH_X86_64
    movd                r6d, m15
 %else
    movd                r3d, m15
 %endif
    mova                 m5, [base+bdct_lb_q]
    mova                 m6, [base+spel_s_shuf2]
    movd                m15, [base+subpel_filters+r4*8+2]
 %if ARCH_X86_64
    movd                 m7, [base+subpel_filters+r6*8+2]
 %else
    movd                 m7, [base+subpel_filters+r3*8+2]
 %endif
    pxor                 m2, m2
    pcmpeqd              m8, m2
    psrld               m14, 10
    paddd               m14, m14
 %if ARCH_X86_32
    mov                  r3, r3m
    pshufb              m14, m5
    paddb               m14, m6
    mova              [stk], m14
    SWAP                 m5, m0
    SWAP                 m6, m3
  %define m15 m6
 %endif
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    movu                 m2, [srcq+ssq*2]
    movu                 m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    punpckldq           m15, m7
 %if ARCH_X86_64
    pshufb              m14, m5
    paddb               m14, m6
    pand                 m9, m8
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m9
    movu                 m4, [srcq+ssq*0]
    movu                 m5, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    movu                 m7, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
 %else
    pand                 m7, m5, [base+pd_0x4000]
    pandn                m5, m15
    por                  m5, m7
  %define m15 m5
 %endif
    punpcklbw           m15, m15
    psraw               m15, 8
    REPX    {pshufb x, m14}, m0, m1, m2, m3
    REPX   {pmaddwd x, m15}, m0, m1, m2, m3
 %if ARCH_X86_64
    REPX    {pshufb x, m14}, m4, m5, m6, m7
    REPX   {pmaddwd x, m15}, m4, m5, m6, m7
    phaddd               m0, m1
    phaddd               m2, m3
    phaddd               m4, m5
    phaddd               m6, m7
    REPX     {paddd x, m11}, m0, m2, m4, m6
    REPX     {psrad x, m12}, m0, m2, m4, m6
    packssdw             m0, m2 ; 0 1 2 3
    packssdw             m4, m6 ; 4 5 6 7
    SWAP                 m1, m4
 %else
    mova         [stk+0x10], m15
    phaddd               m0, m1
    phaddd               m2, m3
    movu                 m1, [srcq+ssq*0]
    movu                 m7, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    movu                 m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    REPX    {pshufb x, m14}, m1, m7, m6, m3
    REPX   {pmaddwd x, m15}, m1, m7, m6, m3
    phaddd               m1, m7
    phaddd               m6, m3
    REPX     {paddd x, m11}, m0, m2, m1, m6
    REPX     {psrad x, m12}, m0, m2, m1, m6
    packssdw             m0, m2
    packssdw             m1, m6
  %define m14 [stk+0x00]
  %define m15 [stk+0x10]
 %endif
    palignr              m2, m1, m0, 4 ; 1 2 3 4
    punpcklwd            m3, m0, m2    ; 01 12
    punpckhwd            m0, m2        ; 23 34
    pshufd               m5, m1, q0321 ; 5 6 7 _
    punpcklwd            m2, m1, m5    ; 45 56
    punpckhwd            m4, m1, m5    ; 67 __
 %if ARCH_X86_32
    mov                 myd, mym
    mov                  r0, r0m
    mova         [stk+0x20], m3
    mova         [stk+0x30], m0
    mova         [stk+0x40], m2
    mova         [stk+0x50], m4
 %endif
.w2_loop:
    and                 myd, 0x3ff
 %if ARCH_X86_64
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq                m10, r6q
    punpcklbw           m10, m10
    psraw               m10, 8
    pshufd               m7, m10, q0000
    pshufd               m8, m10, q1111
    pmaddwd              m5, m3, m7
    pmaddwd              m6, m0, m8
    pshufd               m9, m10, q2222
    pshufd              m10, m10, q3333
    pmaddwd              m7, m2, m9
    pmaddwd              m8, m4, m10
    paddd                m5, m6
    paddd                m7, m8
 %else
    mov                  r1, [esp+0x1f4]
    xor                  r3, r3
    mov                  r5, myd
    shr                  r5, 6
    lea                  r1, [r1+r5]
    mov                  r5, 64 << 24
    cmovnz               r3, [base+subpel_filters+r1*8+4]
    cmovnz               r5, [base+subpel_filters+r1*8+0]
    movd                 m6, r3
    movd                 m7, r5
    punpckldq            m7, m6
    punpcklbw            m7, m7
    psraw                m7, 8
    pshufd               m5, m7, q0000
    pshufd               m6, m7, q1111
    pmaddwd              m3, m5
    pmaddwd              m0, m6
    pshufd               m5, m7, q2222
    pshufd               m7, m7, q3333
    pmaddwd              m2, m5
    pmaddwd              m4, m7
    paddd                m3, m0
    paddd                m2, m4
    SWAP                 m5, m3
    SWAP                 m7, m2
  %define m8 m3
 %endif
    paddd                m5, m13
    pshufd               m6, m12, q1032
    pxor                 m8, m8
    paddd                m5, m7
    psrad                m5, m6
    packssdw             m5, m5
    pmaxsw               m5, m8
    pminsw               m5, pxmaxm
    movd             [dstq], m5
    add                dstq, dsmp
    dec                  hd
    jz .ret
 %if ARCH_X86_64
    add                 myd, dyd
 %else
    add                 myd, dym
 %endif
    test                myd, ~0x3ff
 %if ARCH_X86_32
    SWAP                 m3, m5
    SWAP                 m2, m7
    mova                 m3, [stk+0x20]
    mova                 m0, [stk+0x30]
    mova                 m2, [stk+0x40]
    mova                 m4, [stk+0x50]
 %endif
    jz .w2_loop
 %if ARCH_X86_32
    mov                  r3, r3m
 %endif
    movu                 m5, [srcq]
    test                myd, 0x400
    jz .w2_skip_line
    add                srcq, ssq
    shufps               m3, m0, q1032      ; 01 12
    shufps               m0, m2, q1032      ; 23 34
    shufps               m2, m4, q1032      ; 45 56
    pshufb               m5, m14
    pmaddwd              m5, m15
    phaddd               m5, m5
    paddd                m5, m11
    psrad                m5, m12
    packssdw             m5, m5
    palignr              m4, m5, m1, 12
    punpcklqdq           m1, m4, m4         ; 6 7 6 7
    punpcklwd            m4, m1, m5         ; 67 __
 %if ARCH_X86_32
    mova         [stk+0x20], m3
    mova         [stk+0x30], m0
    mova         [stk+0x40], m2
    mova         [stk+0x50], m4
 %endif
    jmp .w2_loop
.w2_skip_line:
    movu                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                 m3, m0             ; 01 12
    mova                 m0, m2             ; 23 34
    pshufb               m5, m14
    pshufb               m6, m14
    pmaddwd              m5, m15
    pmaddwd              m6, m15
    phaddd               m5, m6
    paddd                m5, m11
    psrad                m5, m12
    packssdw             m5, m5             ; 6 7 6 7
    punpckhqdq           m1, m5             ; 4 5 6 7
    pshufd               m5, m1, q0321      ; 5 6 7 _
    punpcklwd            m2, m1, m5         ; 45 56
    punpckhwd            m4, m1, m5         ; 67 __
 %if ARCH_X86_32
    mova         [stk+0x20], m3
    mova         [stk+0x30], m0
    mova         [stk+0x40], m2
    mova         [stk+0x50], m4
 %endif
    jmp .w2_loop
%endif
INIT_XMM ssse3
.w4:
%if ARCH_X86_64
    mov                 myd, mym
    mova         [rsp+0x10], m11
    mova         [rsp+0x20], m12
 %if isput
    mova         [rsp+0x30], m13
 %endif
    movzx               t0d, t0b
    sub                srcq, 2
    movd                m15, t0d
%else
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
    movzx                r4, byte [esp+0x1f0]
    sub                srcq, 2
    movd                m15, r4
%endif
    pmaddwd              m8, [base+rescale_mul]
%if ARCH_X86_64
    mova                 m9, [base+pd_0x4000]
%else
 %define m9 [base+pd_0x4000]
%endif
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m0, m14, m10
    psrld                m0, 6
    paddd               m15, m0
    pshufd               m7, m15, q1032
%if ARCH_X86_64
    movd                r4d, m15
    movd               r11d, m7
    pshufd              m15, m15, q0321
    pshufd               m7, m7, q0321
    movd                r6d, m15
    movd               r13d, m7
    mova                m10, [base+bdct_lb_q+ 0]
    mova                m11, [base+bdct_lb_q+16]
    movd                m13, [base+subpel_filters+ r4*8+2]
    movd                 m2, [base+subpel_filters+ r6*8+2]
    movd                m15, [base+subpel_filters+r11*8+2]
    movd                 m4, [base+subpel_filters+r13*8+2]
%else
    movd                 r0, m15
    movd                 r4, m7
    pshufd              m15, m15, q0321
    pshufd               m7, m7, q0321
    movd                 rX, m15
    movd                 r5, m7
    mova                 m5, [base+bdct_lb_q+ 0]
    mova                 m6, [base+bdct_lb_q+16]
    movd                 m1, [base+subpel_filters+r0*8+2]
    movd                 m2, [base+subpel_filters+rX*8+2]
    movd                 m3, [base+subpel_filters+r4*8+2]
    movd                 m7, [base+subpel_filters+r5*8+2]
    movifprep            r3, r3m
    SWAP                 m4, m7
 %define m10 m5
 %define m11 m6
 %define m12 m1
 %define m13 m1
%endif
    psrld               m14, 10
    paddd               m14, m14
    punpckldq           m13, m2
    punpckldq           m15, m4
    punpcklqdq          m13, m15
    pxor                 m2, m2
    pcmpeqd              m0, m2
%if ARCH_X86_64
    pand                 m9, m0
%else
    pand                 m2, m9, m0
 %define m9 m2
    SWAP                 m7, m4
%endif
    pandn                m0, m13
%if ARCH_X86_64
    SWAP                m13, m0
%else
 %define m13 m0
%endif
    por                 m13, m9
    punpckhbw           m15, m13, m13
    punpcklbw           m13, m13
    psraw               m15, 8
    psraw               m13, 8
    pshufb              m12, m14, m10
    pshufb              m14, m11
    mova                m10, [base+spel_s_shuf2]
    movd                r4d, m14
    shr                 r4d, 24
%if ARCH_X86_32
    mova         [stk+0x20], m13
    mova         [stk+0x30], m15
    pxor                 m2, m2
%endif
    pshufb               m7, m14, m2
    psubb               m14, m7
    paddb               m12, m10
    paddb               m14, m10
%if ARCH_X86_64
    lea                  r6, [r4+ssq*1]
    lea                 r11, [r4+ssq*2]
    lea                 r13, [r4+ss3q ]
    movu                 m7, [srcq+ssq*0]
    movu                 m9, [srcq+ssq*1]
    movu                 m8, [srcq+ssq*2]
    movu                m10, [srcq+ss3q ]
    movu                 m1, [srcq+r4   ]
    movu                 m3, [srcq+r6   ]
    movu                 m2, [srcq+r11  ]
    movu                 m4, [srcq+r13  ]
    lea                srcq, [srcq+ssq*4]
    REPX    {pshufb x, m12}, m7, m9, m8, m10
    REPX   {pmaddwd x, m13}, m7, m9, m8, m10
    REPX    {pshufb x, m14}, m1, m2, m3, m4
    REPX   {pmaddwd x, m15}, m1, m2, m3, m4
    mova                 m5, [rsp+0x10]
    movd                xm6, [rsp+0x20]
    phaddd               m7, m1
    phaddd               m9, m3
    phaddd               m8, m2
    phaddd              m10, m4
    movu                 m1, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*1]
    movu                 m3, [srcq+ssq*2]
    movu                 m4, [srcq+ss3q ]
    REPX      {paddd x, m5}, m7, m9, m8, m10
    REPX     {psrad x, xm6}, m7, m9, m8, m10
    packssdw             m7, m9  ; 0 1
    packssdw             m8, m10 ; 2 3
    movu                 m0, [srcq+r4   ]
    movu                 m9, [srcq+r6   ]
    movu                m10, [srcq+r11  ]
    movu                m11, [srcq+r13  ]
    lea                srcq, [srcq+ssq*4]
    REPX    {pshufb x, m12}, m1, m2, m3, m4
    REPX   {pmaddwd x, m13}, m1, m2, m3, m4
    REPX    {pshufb x, m14}, m0, m9, m10, m11
    REPX   {pmaddwd x, m15}, m0, m9, m10, m11
    phaddd               m1, m0
    phaddd               m2, m9
    phaddd               m3, m10
    phaddd               m4, m11
    REPX      {paddd x, m5}, m1, m2, m3, m4
    REPX     {psrad x, xm6}, m1, m2, m3, m4
    packssdw             m1, m2 ; 4 5
    packssdw             m3, m4 ; 6 7
    SWAP                 m9, m1
    shufps               m4, m7, m8, q1032  ; 1 2
    shufps               m5, m8, m9, q1032  ; 3 4
    shufps               m6, m9, m3, q1032  ; 5 6
    pshufd              m10, m3, q1032      ; 7 _
    punpcklwd            m0, m7, m4 ; 01
    punpckhwd            m7, m4     ; 12
    punpcklwd            m1, m8, m5 ; 23
    punpckhwd            m8, m5     ; 34
    punpcklwd            m2, m9, m6 ; 45
    punpckhwd            m9, m6     ; 56
    punpcklwd            m3, m10    ; 67
    mova         [rsp+0x40], m7
    mova         [rsp+0x50], m8
    mova         [rsp+0x60], m9
%else
    mova         [stk+0x00], m12
    mova         [stk+0x10], m14
    add                  r4, srcq
    MC_4TAP_SCALED_H   0x40 ; 0 1
    MC_4TAP_SCALED_H   0x50 ; 2 3
    MC_4TAP_SCALED_H   0x60 ; 4 5
    MC_4TAP_SCALED_H   0x70 ; 6 7
    mova                 m4, [stk+0x40]
    mova                 m5, [stk+0x50]
    mova                 m6, [stk+0x60]
    mova                 m7, [stk+0x70]
    mov          [stk+0xc0], r4
    shufps               m1, m4, m5, q1032 ; 1 2
    shufps               m2, m5, m6, q1032 ; 3 4
    shufps               m3, m6, m7, q1032 ; 5 6
    pshufd               m0, m7, q1032     ; 7 _
    mova         [stk+0xb0], m0
    punpcklwd            m0, m4, m1         ; 01
    punpckhwd            m4, m1             ; 12
    punpcklwd            m1, m5, m2         ; 23
    punpckhwd            m5, m2             ; 34
    punpcklwd            m2, m6, m3         ; 45
    punpckhwd            m6, m3             ; 56
    punpcklwd            m3, m7, [stk+0xb0] ; 67
    mov                 myd, mym
    mov                  r0, r0m
    mova         [stk+0x40], m0 ; 01
    mova         [stk+0x50], m1 ; 23
    mova         [stk+0x60], m2 ; 45
    mova         [stk+0x70], m3 ; 67
    mova         [stk+0x80], m4 ; 12
    mova         [stk+0x90], m5 ; 34
    mova         [stk+0xa0], m6 ; 56
 %define m12 [stk+0x00]
 %define m14 [stk+0x10]
 %define m13 [stk+0x20]
 %define m15 [stk+0x30]
 %define hrnd_mem [esp+0x00]
 %define hsh_mem  [esp+0x10]
 %if isput
  %define vrnd_mem [esp+0x20]
 %else
  %define vrnd_mem [base+pd_m524256]
 %endif
%endif
.w4_loop:
    and                 myd, 0x3ff
%if ARCH_X86_64
    mov                r11d, 64 << 24
    mov                r13d, myd
    shr                r13d, 6
    lea                r13d, [t1+r13]
    cmovnz             r11q, [base+subpel_filters+r13*8]
    movq                 m9, r11q
    punpcklbw            m9, m9
    psraw                m9, 8
    pshufd               m7, m9, q0000
    pshufd               m8, m9, q1111
    pmaddwd              m4, m0, m7
    pmaddwd              m5, m1, m8
    pshufd               m7, m9, q2222
    pshufd               m9, m9, q3333
    pmaddwd              m6, m2, m7
    pmaddwd              m8, m3, m9
 %if isput
    movd                 m9, [rsp+0x28]
  %define vrnd_mem [rsp+0x30]
 %else
  %define vrnd_mem [base+pd_m524256]
 %endif
    paddd                m4, m5
    paddd                m6, m8
    paddd                m4, m6
    paddd                m4, vrnd_mem
%else
    mov                 mym, myd
    mov                  r5, [esp+0x1f4]
    xor                  r3, r3
    shr                  r4, 6
    lea                  r5, [r5+r4]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r5*8+0]
    cmovnz               r3, [base+subpel_filters+r5*8+4]
    movd                 m7, r4
    movd                 m6, r3
    punpckldq            m7, m6
    punpcklbw            m7, m7
    psraw                m7, 8
    pshufd               m4, m7, q0000
    pshufd               m5, m7, q1111
    pshufd               m6, m7, q2222
    pshufd               m7, m7, q3333
    pmaddwd              m0, m4
    pmaddwd              m1, m5
    pmaddwd              m2, m6
    pmaddwd              m3, m7
 %if isput
    movd                 m4, [esp+0x18]
 %endif
    paddd                m0, m1
    paddd                m2, m3
    paddd                m0, vrnd_mem
    paddd                m0, m2
    SWAP                 m4, m0
 %define m9 m0
%endif
%if isput
    pxor                 m5, m5
    psrad                m4, m9
    packssdw             m4, m4
    pmaxsw               m4, m5
    pminsw               m4, pxmaxm
    movq             [dstq], m4
    add                dstq, dsmp
%else
    psrad                m4, 6
    packssdw             m4, m4
    movq             [tmpq], m4
    add                tmpq, 8
%endif
    dec                  hd
    jz .ret
%if ARCH_X86_64
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .w4_loop
    mova                 m8, [rsp+0x10]
    movd                 m9, [rsp+0x20]
    movu                 m4, [srcq]
    movu                 m5, [srcq+r4]
    test                myd, 0x400
    jz .w4_skip_line
    mova                 m0, [rsp+0x40]
    mova         [rsp+0x40], m1
    mova                 m1, [rsp+0x50]
    mova         [rsp+0x50], m2
    mova                 m2, [rsp+0x60]
    mova         [rsp+0x60], m3
    pshufb               m4, m12
    pshufb               m5, m14
    pmaddwd              m4, m13
    pmaddwd              m5, m15
    phaddd               m4, m5
    paddd                m4, m8
    psrad                m4, m9
    packssdw             m4, m4
    punpcklwd            m3, m10, m4
    mova                m10, m4
    add                srcq, ssq
    jmp .w4_loop
.w4_skip_line:
    movu                 m6, [srcq+ssq*1]
    movu                 m7, [srcq+r6]
    mova                 m0, [rsp+0x50]
    mova                m11, [rsp+0x60]
    pshufb               m4, m12
    pshufb               m6, m12
    pshufb               m5, m14
    pshufb               m7, m14
    pmaddwd              m4, m13
    pmaddwd              m6, m13
    pmaddwd              m5, m15
    pmaddwd              m7, m15
    mova         [rsp+0x40], m0
    mova         [rsp+0x50], m11
    phaddd               m4, m5
    phaddd               m6, m7
    paddd                m4, m8
    paddd                m6, m8
    psrad                m4, m9
    psrad                m6, m9
    packssdw             m4, m6
    punpcklwd            m9, m10, m4
    mova         [rsp+0x60], m9
    pshufd              m10, m4, q1032
    mova                 m0, m1
    mova                 m1, m2
    mova                 m2, m3
    punpcklwd            m3, m4, m10
    lea                srcq, [srcq+ssq*2]
    jmp .w4_loop
%else
    SWAP                 m0, m4
    mov                 myd, mym
    mov                  r3, r3m
    add                 myd, dym
    test                myd, ~0x3ff
    jnz .w4_next_line
    mova                 m0, [stk+0x40]
    mova                 m1, [stk+0x50]
    mova                 m2, [stk+0x60]
    mova                 m3, [stk+0x70]
    jmp .w4_loop
.w4_next_line:
    mov                  r5, [stk+0xc0]
    movu                 m4, [srcq]
    movu                 m5, [r5]
    test                myd, 0x400
    jz .w4_skip_line
    add          [stk+0xc0], ssq
    mova                 m0, [stk+0x80]
    mova                 m3, [stk+0x50]
    mova         [stk+0x40], m0
    mova         [stk+0x80], m3
    mova                 m1, [stk+0x90]
    mova                 m6, [stk+0x60]
    mova         [stk+0x50], m1
    mova         [stk+0x90], m6
    mova                 m2, [stk+0xa0]
    mova                 m7, [stk+0x70]
    mova         [stk+0x60], m2
    mova         [stk+0xa0], m7
    pshufb               m4, m12
    pshufb               m5, m14
    pmaddwd              m4, m13
    pmaddwd              m5, m15
    phaddd               m4, m5
    paddd                m4, hrnd_mem
    psrad                m4, hsh_mem
    packssdw             m4, m4
    punpcklwd            m3, [stk+0xb0], m4
    mova         [stk+0xb0], m4
    mova         [stk+0x70], m3
    add                srcq, ssq
    jmp .w4_loop
.w4_skip_line:
    movu                 m6, [srcq+ssq*1]
    movu                 m7, [r5  +ssq*1]
    lea                  r5, [r5  +ssq*2]
    mov          [stk+0xc0], r5
    mova                 m0, [stk+0x50]
    mova                 m1, [stk+0x60]
    mova                 m2, [stk+0x70]
    mova                 m3, [stk+0x90]
    pshufb               m4, m12
    pshufb               m6, m12
    pshufb               m5, m14
    pshufb               m7, m14
    pmaddwd              m4, m13
    pmaddwd              m6, m13
    pmaddwd              m5, m15
    pmaddwd              m7, m15
    mova         [stk+0x40], m0
    mova         [stk+0x50], m1
    mova         [stk+0x60], m2
    mova         [stk+0x80], m3
    phaddd               m4, m5
    phaddd               m6, m7
    mova                 m5, [stk+0xa0]
    mova                 m7, [stk+0xb0]
    paddd                m4, hrnd_mem
    paddd                m6, hrnd_mem
    psrad                m4, hsh_mem
    psrad                m6, hsh_mem
    packssdw             m4, m6
    punpcklwd            m7, m4
    pshufd               m6, m4, q1032
    mova         [stk+0x90], m5
    mova         [stk+0xa0], m7
    mova         [stk+0xb0], m6
    punpcklwd            m3, m4, m6
    mova         [stk+0x70], m3
    lea                srcq, [srcq+ssq*2]
    jmp .w4_loop
%endif
INIT_XMM ssse3
%if ARCH_X86_64
 %define stk rsp+0x20
%endif
.w8:
    mov    dword [stk+0xf0], 1
    movifprep   tmp_stridem, 16
    jmp .w_start
.w16:
    mov    dword [stk+0xf0], 2
    movifprep   tmp_stridem, 32
    jmp .w_start
.w32:
    mov    dword [stk+0xf0], 4
    movifprep   tmp_stridem, 64
    jmp .w_start
.w64:
    mov    dword [stk+0xf0], 8
    movifprep   tmp_stridem, 128
    jmp .w_start
.w128:
    mov    dword [stk+0xf0], 16
    movifprep   tmp_stridem, 256
.w_start:
%if ARCH_X86_64
 %ifidn %1, put
    movifnidn           dsm, dsq
 %endif
    mova         [rsp+0x10], m11
 %define hround m11
    shr                 t0d, 16
    movd                m15, t0d
 %if isprep
    mova                m13, [base+pd_m524256]
 %endif
%else
 %define hround [esp+0x00]
 %define m12    [esp+0x10]
 %define m10    [base+pd_0x3ff]
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
 %if isprep
  %define ssq ssm
 %endif
    mov                  r4, [esp+0x1f0]
    shr                  r4, 16
    movd                m15, r4
    mov                  r0, r0m
    mov                 myd, mym
%endif
    sub                srcq, 6
    pslld                m7, m8, 2 ; dx*4
    pmaddwd              m8, [base+rescale_mul] ; dx*[0-3]
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    mova        [stk+0x100], m7
    mova        [stk+0x120], m15
    mov         [stk+0x0f8], srcq
    mov         [stk+0x130], r0q ; dstq / tmpq
%if ARCH_X86_64 && UNIX64
    mov                  hm, hd
%elif ARCH_X86_32
    mov                  r5, hm
    mov         [stk+0x0f4], myd
    mov         [stk+0x134], r5
%endif
    jmp .hloop
.hloop_prep:
    dec   dword [stk+0x0f0]
    jz .ret
%if ARCH_X86_64
    add   qword [stk+0x130], 16
    mov                  hd, hm
%else
    add   dword [stk+0x130], 16
    mov                 myd, [stk+0x0f4]
    mov                  r5, [stk+0x134]
    mov                  r0, [stk+0x130]
%endif
    mova                 m7, [stk+0x100]
    mova                m14, [stk+0x110]
%if ARCH_X86_64
    mova                m10, [base+pd_0x3ff]
    mova                m11, [rsp+0x10]
%endif
    mova                m15, [stk+0x120]
    mov                srcq, [stk+0x0f8]
%if ARCH_X86_64
    mov                 r0q, [stk+0x130] ; dstq / tmpq
%else
    mov                 mym, myd
    mov                  hm, r5
    mov                 r0m, r0
    mov                  r3, r3m
%endif
    paddd               m14, m7
.hloop:
%if ARCH_X86_64
    mova                 m9, [base+pq_0x40000000]
%else
 %define m9 [base+pq_0x40000000]
%endif
    pxor                 m1, m1
    psrld                m2, m14, 10
    mova              [stk], m2
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd                m5, m15, m6
    pcmpeqd              m6, m1
    pshufd               m2, m5, q1032
%if ARCH_X86_64
    movd                r4d, m5
    movd                r6d, m2
    pshufd               m5, m5, q0321
    pshufd               m2, m2, q0321
    movd                r7d, m5
    movd                r9d, m2
    movq                 m0, [base+subpel_filters+r4*8]
    movq                 m1, [base+subpel_filters+r6*8]
    movhps               m0, [base+subpel_filters+r7*8]
    movhps               m1, [base+subpel_filters+r9*8]
%else
    movd                 r0, m5
    movd                 rX, m2
    pshufd               m5, m5, q0321
    pshufd               m2, m2, q0321
    movd                 r4, m5
    movd                 r5, m2
    movq                 m0, [base+subpel_filters+r0*8]
    movq                 m1, [base+subpel_filters+rX*8]
    movhps               m0, [base+subpel_filters+r4*8]
    movhps               m1, [base+subpel_filters+r5*8]
%endif
    paddd               m14, m7 ; mx+dx*[4-7]
    pand                 m5, m14, m10
    psrld                m5, 6
    paddd               m15, m5
    pxor                 m2, m2
    pcmpeqd              m5, m2
    mova        [stk+0x110], m14
    pshufd               m4, m15, q1032
%if ARCH_X86_64
    movd               r10d, m15
    movd               r11d, m4
    pshufd              m15, m15, q0321
    pshufd               m4, m4, q0321
    movd               r13d, m15
    movd                rXd, m4
    movq                 m2, [base+subpel_filters+r10*8]
    movq                 m3, [base+subpel_filters+r11*8]
    movhps               m2, [base+subpel_filters+r13*8]
    movhps               m3, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    movq                r11, m14
    punpckhqdq          m14, m14
    movq                 rX, m14
    mov                r10d, r11d
    shr                 r11, 32
    mov                r13d, rXd
    shr                  rX, 32
    mov                 r4d, [stk+ 0]
    mov                 r6d, [stk+ 4]
    mov                 r7d, [stk+ 8]
    mov                 r9d, [stk+12]
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd              m14, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m7, m9, m4
    pand                 m8, m9, m6
    pand                m15, m9, m14
    pand                 m9, m9, m5
    pandn                m4, m0
    pandn                m6, m1
    pandn               m14, m2
    pandn                m5, m3
    por                  m7, m4
    por                  m8, m6
    por                 m15, m14
    por                  m9, m5
    punpcklbw            m0, m7, m7
    punpckhbw            m7, m7
    punpcklbw            m1, m8, m8
    punpckhbw            m8, m8
    psraw                m0, 8
    psraw                m7, 8
    psraw                m1, 8
    psraw                m8, 8
    punpcklbw            m2, m15, m15
    punpckhbw           m15, m15
    punpcklbw            m3, m9, m9
    punpckhbw            m9, m9
    psraw                m2, 8
    psraw               m15, 8
    psraw                m3, 8
    psraw                m9, 8
    mova         [stk+0x10], m0
    mova         [stk+0x20], m7
    mova         [stk+0x30], m1
    mova         [stk+0x40], m8
    mova         [stk+0x50], m2
    mova         [stk+0x60], m15
    mova         [stk+0x70], m3
    mova         [stk+0x80], m9
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 9, 10 ; 0
    mova         [stk+0x90], m1
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 1, 9, 10 ; 1
    mova         [stk+0xa0], m2
    MC_8TAP_SCALED_H 3, 4, 5, 6, 1, 2, 9, 10 ; 2
    mova         [stk+0xb0], m3
    MC_8TAP_SCALED_H 4, 5, 6, 1, 2, 3, 9, 10 ; 3
    mova         [stk+0xc0], m4
    MC_8TAP_SCALED_H 5, 6, 1, 2, 3, 4, 9, 10 ; 4
    mova         [stk+0xd0], m5
    MC_8TAP_SCALED_H 6, 1, 2, 3, 4, 5, 9, 10 ; 5
    MC_8TAP_SCALED_H 7, 1, 2, 3, 4, 5, 9, 10 ; 6
    MC_8TAP_SCALED_H 8, 1, 2, 3, 4, 5, 9, 10 ; 7
    mova                 m5, [stk+0xd0]
    mova                 m1, [stk+0x90]
    mova                 m2, [stk+0xa0]
    mova                 m3, [stk+0xb0]
    mova                 m9, [stk+0xc0]
    mov                 myd, mym
    mov                 dyd, dym
    punpcklwd            m4, m5, m6 ; 45a
    punpckhwd            m5, m6     ; 45b
    punpcklwd            m6, m7, m8 ; 67a
    punpckhwd            m7, m8     ; 67b
    punpcklwd            m0, m1, m2 ; 01a
    punpckhwd            m1, m2     ; 01b
    punpcklwd            m2, m3, m9 ; 23a
    punpckhwd            m3, m9     ; 23b
    mova         [stk+0x90], m4
    mova         [stk+0xa0], m5
    mova         [stk+0xb0], m6
    mova         [stk+0xc0], m7
 %define hround [rsp+0x10]
.vloop:
    and                 myd, 0x3ff
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq                m11, r6q
    punpcklbw           m11, m11
    psraw               m11, 8
    pshufd               m5, m11, q0000
    pshufd               m7, m11, q1111
    pshufd              m10, m11, q2222
    pshufd              m11, m11, q3333
    pmaddwd              m4, m5, m0
    pmaddwd              m5, m5, m1
    pmaddwd              m6, m7, m2
    pmaddwd              m7, m7, m3
    paddd                m4, m13
    paddd                m5, m13
    paddd                m4, m6
    paddd                m5, m7
    pmaddwd              m6, [stk+0x90], m10
    pmaddwd              m7, [stk+0xa0], m10
    pmaddwd              m8, [stk+0xb0], m11
    pmaddwd              m9, [stk+0xc0], m11
    paddd                m4, m6
    paddd                m5, m7
 %if isput
    pshufd               m6, m12, q1032
 %endif
    paddd                m4, m8
    paddd                m5, m9
%else
    movd                 r0, m15
    movd                 rX, m4
    pshufd              m15, m15, q0321
    pshufd               m4, m4, q0321
    movd                 r4, m15
    movd                 r5, m4
    mova                m14, [stk+0x110]
    movq                 m2, [base+subpel_filters+r0*8]
    movq                 m3, [base+subpel_filters+rX*8]
    movhps               m2, [base+subpel_filters+r4*8]
    movhps               m3, [base+subpel_filters+r5*8]
    psrld               m14, 10
    mova           [stk+16], m14
    mov                  r0, [stk+ 0]
    mov                  rX, [stk+ 4]
    mov                  r4, [stk+ 8]
    mov                  r5, [stk+12]
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m0, m9, m4
    pand                 m1, m9, m6
    pand                 m2, m9, m7
    pand                 m3, m9, m5
    pandn                m4, [stk+0x20]
    pandn                m6, [stk+0x30]
    pandn                m7, [stk+0x40]
    pandn                m5, [stk+0x50]
    por                  m0, m4
    por                  m1, m6
    por                  m2, m7
    por                  m3, m5
    punpcklbw            m4, m0, m0
    punpckhbw            m0, m0
    punpcklbw            m5, m1, m1
    punpckhbw            m1, m1
    psraw                m4, 8
    psraw                m0, 8
    psraw                m5, 8
    psraw                m1, 8
    punpcklbw            m6, m2, m2
    punpckhbw            m2, m2
    punpcklbw            m7, m3, m3
    punpckhbw            m3, m3
    psraw                m6, 8
    psraw                m2, 8
    psraw                m7, 8
    psraw                m3, 8
    mova        [stk+0x0a0], m4
    mova        [stk+0x0b0], m0
    mova        [stk+0x0c0], m5
    mova        [stk+0x0d0], m1
    mova        [stk+0x140], m6
    mova        [stk+0x150], m2
    mova        [stk+0x160], m7
    mova        [stk+0x170], m3
    MC_8TAP_SCALED_H   0xa0, 0x20, 0 ; 0
    MC_8TAP_SCALED_H   0xa0, 0x30    ; 1
    MC_8TAP_SCALED_H   0xa0, 0x40    ; 2
    MC_8TAP_SCALED_H   0xa0, 0x50    ; 3
    MC_8TAP_SCALED_H   0xa0, 0x60    ; 4
    MC_8TAP_SCALED_H   0xa0, 0x70    ; 5
    MC_8TAP_SCALED_H   0xa0, 0x80    ; 6
    MC_8TAP_SCALED_H   0xa0, 0x90    ; 7
    mova                 m5, [stk+0x60]
    mova                 m6, [stk+0x70]
    mova                 m7, [stk+0x80]
    mova                 m0, [stk+0x90]
    mov                 myd, mym
    punpcklwd            m4, m5, m6      ; 45a
    punpckhwd            m5, m6          ; 45b
    punpcklwd            m6, m7, m0      ; 67a
    punpckhwd            m7, m0          ; 67b
    mova         [stk+0x60], m4
    mova         [stk+0x70], m5
    mova         [stk+0x80], m6
    mova         [stk+0x90], m7
    mova                 m1, [stk+0x20]
    mova                 m2, [stk+0x30]
    mova                 m3, [stk+0x40]
    mova                 m4, [stk+0x50]
    punpcklwd            m0, m1, m2      ; 01a
    punpckhwd            m1, m2          ; 01b
    punpcklwd            m2, m3, m4      ; 23a
    punpckhwd            m3, m4          ; 23b
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
.vloop:
    mov                  r0, r0m
    mov                  r5, [esp+0x1f4]
    and                 myd, 0x3ff
    mov                 mym, myd
    xor                  r3, r3
    shr                  r4, 6
    lea                  r5, [r5+r4]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r5*8+0]
    cmovnz               r3, [base+subpel_filters+r5*8+4]
    movd                 m7, r4
    movd                 m6, r3
    punpckldq            m7, m6
    punpcklbw            m7, m7
    psraw                m7, 8
    pshufd               m4, m7, q0000
    pshufd               m5, m7, q1111
    pmaddwd              m0, m4
    pmaddwd              m1, m4
    pmaddwd              m2, m5
    pmaddwd              m3, m5
    pshufd               m6, m7, q2222
    pshufd               m7, m7, q3333
    paddd                m0, m2
    paddd                m1, m3
    pmaddwd              m2, [stk+0x60], m6
    pmaddwd              m3, [stk+0x70], m6
    pmaddwd              m4, [stk+0x80], m7
    pmaddwd              m5, [stk+0x90], m7
 %if isput
    movd                 m6, [esp+0x18]
 %endif
    paddd                m0, m2
    paddd                m1, m3
    paddd                m0, vrnd_mem
    paddd                m1, vrnd_mem
    paddd                m4, m0
    paddd                m5, m1
%endif
%ifidn %1, put
    psrad                m4, m6
    psrad                m5, m6
    packssdw             m4, m5
    pxor                 m7, m7
    pmaxsw               m4, m7
    pminsw               m4, pxmaxm
    mova             [dstq], m4
    add                dstq, dsm
%else
    psrad                m4, 6
    psrad                m5, 6
    packssdw             m4, m5
    mova             [tmpq], m4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .hloop_prep
%if ARCH_X86_64
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .vloop
    test                myd, 0x400
    mov         [stk+0x140], myd
    mov                 r4d, [stk+ 0]
    mov                 r6d, [stk+ 4]
    mov                 r7d, [stk+ 8]
    mov                 r9d, [stk+12]
    jz .skip_line
    mova                m14, [base+unpckw]
    movu                 m8, [srcq+r10*2]
    movu                 m9, [srcq+r11*2]
    movu                m10, [srcq+r13*2]
    movu                m11, [srcq+ rX*2]
    movu                 m4, [srcq+ r4*2]
    movu                 m5, [srcq+ r6*2]
    movu                 m6, [srcq+ r7*2]
    movu                 m7, [srcq+ r9*2]
    add                srcq, ssq
    mov                 myd, [stk+0x140]
    mov                 dyd, dym
    pshufd              m15, m14, q1032
    pshufb               m0, m14                ; 0a 1a
    pshufb               m1, m14                ; 0b 1b
    pshufb               m2, m15                ; 3a 2a
    pshufb               m3, m15                ; 3b 2b
    pmaddwd              m8, [stk+0x50]
    pmaddwd              m9, [stk+0x60]
    pmaddwd             m10, [stk+0x70]
    pmaddwd             m11, [stk+0x80]
    pmaddwd              m4, [stk+0x10]
    pmaddwd              m5, [stk+0x20]
    pmaddwd              m6, [stk+0x30]
    pmaddwd              m7, [stk+0x40]
    phaddd               m8, m9
    phaddd              m10, m11
    mova                m11, hround
    phaddd               m4, m5
    phaddd               m6, m7
    phaddd               m8, m10
    phaddd               m4, m6
    paddd                m4, m11
    paddd                m8, m11
    psrad                m4, m12
    psrad                m8, m12
    packssdw             m4, m8
    pshufb               m5, [stk+0x90], m14    ; 4a 5a
    pshufb               m6, [stk+0xa0], m14    ; 4b 5b
    pshufb               m7, [stk+0xb0], m15    ; 7a 6a
    pshufb               m8, [stk+0xc0], m15    ; 7b 6b
    punpckhwd            m0, m2 ; 12a
    punpckhwd            m1, m3 ; 12b
    punpcklwd            m2, m5 ; 34a
    punpcklwd            m3, m6 ; 34b
    punpckhwd            m5, m7 ; 56a
    punpckhwd            m6, m8 ; 56b
    punpcklwd            m7, m4 ; 78a
    punpckhqdq           m4, m4
    punpcklwd            m8, m4 ; 78b
    mova         [stk+0x90], m5
    mova         [stk+0xa0], m6
    mova         [stk+0xb0], m7
    mova         [stk+0xc0], m8
    jmp .vloop
.skip_line:
    MC_8TAP_SCALED_H 4, 8, 5, 6, 7, 9, 10, 11
    MC_8TAP_SCALED_H 8, 5, 6, 7, 9, 0, 10, 11
    mov                 myd, [stk+0x140]
    mov                 dyd, dym
    mova                 m0, m2         ; 01a
    mova                 m1, m3         ; 01b
    mova                 m2, [stk+0x90] ; 23a
    mova                 m3, [stk+0xa0] ; 23b
    mova                 m5, [stk+0xb0] ; 45a
    mova                 m6, [stk+0xc0] ; 45b
    punpcklwd            m7, m4, m8     ; 67a
    punpckhwd            m4, m8         ; 67b
    mova         [stk+0x90], m5
    mova         [stk+0xa0], m6
    mova         [stk+0xb0], m7
    mova         [stk+0xc0], m4
%else
    mov                 r0m, r0
    mov                 myd, mym
    mov                  r3, r3m
    add                 myd, dym
    test                myd, ~0x3ff
    mov                 mym, myd
    jnz .next_line
    mova                 m0, [stk+0x20]
    mova                 m1, [stk+0x30]
    mova                 m2, [stk+0x40]
    mova                 m3, [stk+0x50]
    jmp .vloop
.next_line:
    test                myd, 0x400
    mov                  r0, [stk+ 0]
    mov                  rX, [stk+ 4]
    mov                  r4, [stk+ 8]
    mov                  r5, [stk+12]
    jz .skip_line
    MC_8TAP_SCALED_H 0xa0, 0xe0, 0 ; 8
    mova                 m7, [base+unpckw]
    pshufd               m4, m7, q1032
    pshufb               m0, [stk+0x20], m7 ; 0a 1a
    pshufb               m1, [stk+0x30], m7 ; 0b 1b
    pshufb               m2, [stk+0x40], m4 ; 3a 2a
    pshufb               m3, [stk+0x50], m4 ; 3b 2b
    pshufb               m5, [stk+0x60], m7 ; 4a 5a
    pshufb               m6, [stk+0x70], m7 ; 4b 5b
    pshufb               m7, [stk+0x80], m4 ; 7a 6a
    punpckhwd            m0, m2 ; 12a
    punpckhwd            m1, m3 ; 12b
    punpcklwd            m2, m5 ; 34a
    punpcklwd            m3, m6 ; 34b
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
    punpckhwd            m5, m7 ; 56a
    mova         [stk+0x60], m5
    pshufb               m5, [stk+0x90], m4 ; 7b 6b
    punpcklwd            m7, [stk+0xe0] ; 78a
    punpckhwd            m6, m5 ; 56b
    mova         [stk+0x70], m6
    movq                 m6, [stk+0xe8]
    mova         [stk+0x80], m7
    punpcklwd            m5, m6
    mov                 myd, mym
    mova         [stk+0x90], m5
    jmp .vloop
.skip_line:
    MC_8TAP_SCALED_H 0xa0, 0xe0, 0 ; 8
    MC_8TAP_SCALED_H 0xa0, 0       ; 9
    mova                 m7, [stk+0xe0]
    mova                 m2, [stk+0x60] ; 23a
    mova                 m3, [stk+0x70] ; 23b
    mova                 m4, [stk+0x80] ; 45a
    mova                 m5, [stk+0x90] ; 45b
    punpcklwd            m6, m7, m0     ; 67a
    punpckhwd            m7, m0         ; 67b
    mova                 m0, [stk+0x40] ; 01a
    mova                 m1, [stk+0x50] ; 01b
    mov                 myd, mym
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
    mova         [stk+0x60], m4
    mova         [stk+0x70], m5
    mova         [stk+0x80], m6
    mova         [stk+0x90], m7
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
%endif
    jmp .vloop
INIT_XMM ssse3
.dy1:
    movzx                wd, word [base+%1_8tap_scaled_ssse3_dy1_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%if isput
.dy1_w2:
 %if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    sub                srcq, 2
    movd                m15, t0d
 %else
  %define m8  m0
  %define m9  m1
  %define m14 m4
  %define m15 m3
  %define m11 [esp+0x00]
  %define m12 [esp+0x10]
  %define m13 [esp+0x20]
    movzx                r5, byte [esp+0x1f0]
    sub                srcq, 2
    movd                m15, r5
    mov                  r1, r1m
 %endif
    pxor                 m9, m9
    punpckldq            m9, m8
    paddd               m14, m9 ; mx+dx*[0-1]
 %if ARCH_X86_64
    mova                 m9, [base+pd_0x4000]
 %endif
    pshufd              m15, m15, q0000
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    movd                r4d, m15
    pshufd              m15, m15, q0321
 %if ARCH_X86_64
    movd                r6d, m15
 %else
    movd                r3d, m15
 %endif
    mova                 m5, [base+bdct_lb_q]
    mova                 m6, [base+spel_s_shuf2]
    movd                m15, [base+subpel_filters+r4*8+2]
 %if ARCH_X86_64
    movd                 m7, [base+subpel_filters+r6*8+2]
 %else
    movd                 m7, [base+subpel_filters+r3*8+2]
 %endif
    pxor                 m2, m2
    pcmpeqd              m8, m2
    psrld               m14, 10
    paddd               m14, m14
 %if ARCH_X86_32
    mov                  r3, r3m
    pshufb              m14, m5
    paddb               m14, m6
    mova              [stk], m14
    SWAP                 m5, m0
    SWAP                 m6, m3
  %define m15 m6
 %endif
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    movu                 m2, [srcq+ssq*2]
    movu                 m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    punpckldq           m15, m7
 %if ARCH_X86_64
    pshufb              m14, m5
    paddb               m14, m6
    pand                 m9, m8
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m9
    movu                 m4, [srcq+ssq*0]
    movu                 m5, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    add                srcq, ss3q
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
 %else
    pand                 m7, m5, [base+pd_0x4000]
    pandn                m5, m15
    por                  m5, m7
  %define m15 m5
    mov                 myd, mym
    mov                  r5, [esp+0x1f4]
    xor                  r3, r3
    shr                 myd, 6
    lea                  r5, [r5+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r5*8+0]
    cmovnz               r3, [base+subpel_filters+r5*8+4]
    mov          [stk+0x20], r3
    mov                  r3, r3m
 %endif
    punpcklbw           m15, m15
    psraw               m15, 8
    REPX    {pshufb x, m14}, m0, m1, m2, m3
    REPX   {pmaddwd x, m15}, m0, m1, m2, m3
 %if ARCH_X86_64
    REPX    {pshufb x, m14}, m4, m5, m6
    REPX   {pmaddwd x, m15}, m4, m5, m6
    phaddd               m0, m1
    phaddd               m2, m3
    phaddd               m4, m5
    phaddd               m6, m6
    REPX     {paddd x, m11}, m0, m2, m4, m6
    REPX     {psrad x, m12}, m0, m2, m4, m6
    packssdw             m0, m2 ; 0 1 2 3
    packssdw             m4, m6 ; 4 5 6
    SWAP                 m1, m4
    movq                m10, r4
 %else
    mova         [stk+0x10], m15
    phaddd               m0, m1
    phaddd               m2, m3
    movu                 m1, [srcq+ssq*0]
    movu                 m7, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    add                srcq, ss3q
    REPX    {pshufb x, m14}, m1, m7, m6
    REPX   {pmaddwd x, m15}, m1, m7, m6
  %define m14 [stk+0x00]
  %define m15 [stk+0x10]
    phaddd               m1, m7
    phaddd               m6, m6
    REPX     {paddd x, m11}, m0, m2, m1, m6
    REPX     {psrad x, m12}, m0, m2, m1, m6
    packssdw             m0, m2
    packssdw             m1, m6
  %define m8  m6
  %define m9  m4
  %define m10 m5
    movd                m10, r4
    movd                 m9, [stk+0x20]
    punpckldq           m10, m9
 %endif
    punpcklbw           m10, m10
    psraw               m10, 8
    pshufd               m7, m10, q0000
    pshufd               m8, m10, q1111
    pshufd               m9, m10, q2222
    pshufd              m10, m10, q3333
 %if ARCH_X86_32
    mova         [stk+0x50], m7
    mova         [stk+0x60], m8
    mova         [stk+0x70], m9
    mova         [stk+0x80], m10
  %define m7  [stk+0x50]
  %define m8  [stk+0x60]
  %define m9  [stk+0x70]
  %define m10 [stk+0x80]
 %endif
    palignr              m2, m1, m0, 4 ; 1 2 3 4
    punpcklwd            m3, m0, m2    ; 01 12
    punpckhwd            m0, m2        ; 23 34
    pshufd               m4, m1, q2121 ; 5 6 5 6
    punpcklwd            m2, m1, m4    ; 45 56
 %if ARCH_X86_32
    mov                  r0, r0m
 %endif
.dy1_w2_loop:
    movu                 m1, [srcq+ssq*0]
    movu                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddwd              m5, m3, m7
    mova                 m3, m0
    pmaddwd              m0, m8
    pshufb               m1, m14
    pshufb               m6, m14
    pmaddwd              m1, m15
    pmaddwd              m6, m15
    phaddd               m1, m6
    paddd                m1, m11
    psrad                m1, m12
    packssdw             m1, m1
    paddd                m5, m0
    mova                 m0, m2
    pmaddwd              m2, m9
    paddd                m5, m2
    palignr              m2, m1, m4, 12
    punpcklwd            m2, m1        ; 67 78
    pmaddwd              m4, m2, m10
    paddd                m5, m13
    paddd                m5, m4
    pxor                 m6, m6
    mova                 m4, m1
    pshufd               m1, m12, q1032
    psrad                m5, m1
    packssdw             m5, m5
    pmaxsw               m5, m6
    pminsw               m5, pxmaxm
    movd       [dstq+dsq*0], m5
    pshuflw              m5, m5, q1032
    movd       [dstq+dsq*1], m5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy1_w2_loop
    RET
%endif
INIT_XMM ssse3
.dy1_w4:
%if ARCH_X86_64
    mov                 myd, mym
    mova         [rsp+0x10], m11
    mova         [rsp+0x20], m12
 %if isput
    mova         [rsp+0x30], m13
  %define vrnd_mem [rsp+0x30]
  %define stk rsp+0x40
 %else
  %define vrnd_mem [base+pd_m524256]
  %define stk rsp+0x30
 %endif
    movzx               t0d, t0b
    sub                srcq, 2
    movd                m15, t0d
%else
 %define m10 [base+pd_0x3ff]
 %define m9  [base+pd_0x4000]
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
 %if isprep
  %define ssq r3
 %endif
    movzx                r5, byte [esp+0x1f0]
    sub                srcq, 2
    movd                m15, r5
%endif
    pmaddwd              m8, [base+rescale_mul]
%if ARCH_X86_64
    mova                 m9, [base+pd_0x4000]
%endif
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m0, m14, m10
    psrld                m0, 6
    paddd               m15, m0
    pshufd               m7, m15, q1032
%if ARCH_X86_64
    movd                r4d, m15
    movd               r11d, m7
    pshufd              m15, m15, q0321
    pshufd               m7, m7, q0321
    movd                r6d, m15
    movd               r13d, m7
    mova                m10, [base+bdct_lb_q+ 0]
    mova                m11, [base+bdct_lb_q+16]
    movd                m13, [base+subpel_filters+ r4*8+2]
    movd                 m2, [base+subpel_filters+ r6*8+2]
    movd                m15, [base+subpel_filters+r11*8+2]
    movd                 m4, [base+subpel_filters+r13*8+2]
%else
    movd                 r0, m15
    movd                 r4, m7
    pshufd              m15, m15, q0321
    pshufd               m7, m7, q0321
    movd                 rX, m15
    movd                 r5, m7
    mova                 m5, [base+bdct_lb_q+ 0]
    mova                 m6, [base+bdct_lb_q+16]
    movd                 m1, [base+subpel_filters+r0*8+2]
    movd                 m2, [base+subpel_filters+rX*8+2]
    movd                 m3, [base+subpel_filters+r4*8+2]
    movd                 m7, [base+subpel_filters+r5*8+2]
    SWAP                 m4, m7
 %if isprep
    mov                  r3, r3m
 %endif
 %define m10 m5
 %define m11 m6
 %define m12 m1
 %define m13 m1
%endif
    psrld               m14, 10
    paddd               m14, m14
    punpckldq           m13, m2
    punpckldq           m15, m4
    punpcklqdq          m13, m15
    pxor                 m2, m2
    pcmpeqd              m0, m2
%if ARCH_X86_64
    pand                 m9, m0
%else
    pand                 m2, m9, m0
 %define m9 m2
    SWAP                 m7, m4
%endif
    pandn                m0, m13
%if ARCH_X86_64
    SWAP                m13, m0
%else
 %define m13 m0
%endif
    por                 m13, m9
    punpckhbw           m15, m13, m13
    punpcklbw           m13, m13
    psraw               m15, 8
    psraw               m13, 8
    pshufb              m12, m14, m10
    pshufb              m14, m11
    mova                m10, [base+spel_s_shuf2]
    movd                r4d, m14
    shr                 r4d, 24
%if ARCH_X86_32
    mova         [stk+0x40], m13
    mova         [stk+0x50], m15
    pxor                 m2, m2
%endif
    pshufb               m7, m14, m2
    psubb               m14, m7
    paddb               m12, m10
    paddb               m14, m10
%if ARCH_X86_64
    lea                  r6, [r4+ssq*1]
    lea                 r11, [r4+ssq*2]
    lea                 r13, [r4+ss3q ]
    movu                 m7, [srcq+ssq*0]
    movu                 m9, [srcq+ssq*1]
    movu                 m8, [srcq+ssq*2]
    movu                m10, [srcq+ss3q ]
    movu                 m1, [srcq+r4   ]
    movu                 m3, [srcq+r6   ]
    movu                 m2, [srcq+r11  ]
    movu                 m4, [srcq+r13  ]
    lea                srcq, [srcq+ssq*4]
    REPX    {pshufb x, m12}, m7, m9, m8, m10
    REPX   {pmaddwd x, m13}, m7, m9, m8, m10
    REPX    {pshufb x, m14}, m1, m3, m2, m4
    REPX   {pmaddwd x, m15}, m1, m3, m2, m4
    mova                 m5, [rsp+0x10]
    movd                xm6, [rsp+0x20]
    phaddd               m7, m1
    phaddd               m9, m3
    phaddd               m8, m2
    phaddd              m10, m4
    movu                 m1, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*1]
    movu                 m3, [srcq+ssq*2]
    REPX      {paddd x, m5}, m7, m9, m8, m10
    REPX     {psrad x, xm6}, m7, m9, m8, m10
    packssdw             m7, m9  ; 0 1
    packssdw             m8, m10 ; 2 3
    movu                 m0, [srcq+r4   ]
    movu                 m9, [srcq+r6   ]
    movu                m10, [srcq+r11  ]
    add                srcq, ss3q
    REPX    {pshufb x, m12}, m1, m2, m3
    REPX   {pmaddwd x, m13}, m1, m2, m3
    REPX    {pshufb x, m14}, m0, m9, m10
    REPX   {pmaddwd x, m15}, m0, m9, m10
    phaddd               m1, m0
    phaddd               m2, m9
    phaddd               m3, m10
    shr                 myd, 6
    mov                r13d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz             r13q, [base+subpel_filters+myq*8]
    REPX      {paddd x, m5}, m1, m2, m3
    REPX     {psrad x, xm6}, m1, m2, m3
    packssdw             m1, m2 ; 4 5
    packssdw             m3, m3 ; 6 6
    SWAP                 m9, m1
    shufps               m4, m7, m8, q1032  ; 1 2
    shufps               m5, m8, m9, q1032  ; 3 4
    shufps               m6, m9, m3, q1032  ; 5 6
    punpcklwd            m0, m7, m4 ; 01
    punpckhwd            m7, m4     ; 12
    punpcklwd            m1, m8, m5 ; 23
    punpckhwd            m8, m5     ; 34
    punpcklwd            m2, m9, m6 ; 45
    punpckhwd            m9, m6     ; 56
    movq                m10, r13
    mova         [stk+0x00], m1
    mova         [stk+0x10], m8
    mova         [stk+0x20], m2
    mova         [stk+0x30], m9
    mova         [stk+0x40], m3
 %define hrnd_mem [rsp+0x10]
 %define hsh_mem  [rsp+0x20]
 %define vsh_mem  [rsp+0x28]
 %if isput
  %define vrnd_mem [rsp+0x30]
 %else
  %define vrnd_mem [base+pd_m524256]
 %endif
%else
    mova         [stk+0x20], m12
    mova         [stk+0x30], m14
    add                  r4, srcq
    MC_4TAP_SCALED_H   0x60 ; 0 1
    MC_4TAP_SCALED_H   0x70 ; 2 3
    MC_4TAP_SCALED_H   0x80 ; 4 5
    movu                 m7, [srcq]
    movu                 m2, [r4]
    add                srcq, ssq
    add                  r4, ssq
    mov          [stk+0xb0], r4
    pshufb               m7, m12
    pshufb               m2, m14
    pmaddwd              m7, m13
    pmaddwd              m2, m15
    phaddd               m7, m2
    paddd                m7, [esp+0x00]
    psrad                m7, [esp+0x10]
    packssdw             m7, m7 ; 6 6
    mova                 m4, [stk+0x60]
    mova                 m5, [stk+0x70]
    mova                 m6, [stk+0x80]
    mov                 myd, mym
    mov                  rX, [esp+0x1f4]
    xor                  r5, r5
    shr                 myd, 6
    lea                  rX, [rX+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+rX*8+0]
    cmovnz               r5, [base+subpel_filters+rX*8+4]
    mov                  r3, r3m
    shufps               m1, m4, m5, q1032 ; 1 2
    shufps               m2, m5, m6, q1032 ; 3 4
    shufps               m3, m6, m7, q1032 ; 5 6
    mova         [stk+0xa0], m7
    punpcklwd            m0, m4, m1         ; 01
    punpckhwd            m4, m1             ; 12
    punpcklwd            m1, m5, m2         ; 23
    punpckhwd            m5, m2             ; 34
    punpcklwd            m2, m6, m3         ; 45
    punpckhwd            m6, m3             ; 56
    movd                 m7, r4
    movd                 m3, r5
    mov                  r0, r0m
 %if isput
    mov                  r1, r1m
 %endif
    mov                  r4, [stk+0xb0]
    mova         [stk+0xc0], m4 ; 12
    mova         [stk+0x60], m1 ; 23
    mova         [stk+0x70], m2 ; 45
    mova         [stk+0x80], m5 ; 34
    mova         [stk+0x90], m6 ; 56
 %define m12 [stk+0x20]
 %define m14 [stk+0x30]
 %define m13 [stk+0x40]
 %define m15 [stk+0x50]
 %define hrnd_mem [esp+0x00]
 %define hsh_mem  [esp+0x10]
 %define vsh_mem  [esp+0x18]
 %if isput
  %define vrnd_mem [esp+0x20]
 %else
  %define vrnd_mem [base+pd_m524256]
 %endif
 %define m10 m7
    punpckldq           m10, m3
%endif
    punpcklbw           m10, m10
    psraw               m10, 8
    pshufd               m3, m10, q0000
    pshufd               m4, m10, q1111
    pshufd               m5, m10, q2222
    pshufd              m10, m10, q3333
%if ARCH_X86_32
 %xdefine m8  m3
 %xdefine m9  m6
 %xdefine m11 m5
 %xdefine m6  m4
    mova         [stk+0x100], m3
    mova         [stk+0x110], m4
    mova         [stk+0x120], m5
    mova         [stk+0x130], m10
 %define m3  [stk+0x100]
 %define m4  [stk+0x110]
 %define m5  [stk+0x120]
 %define m10 [stk+0x130]
    mova                 m7, [stk+0xc0]
    mova                 m8, [stk+0x80]
%endif
.dy1_w4_loop:
    movu                m11, [srcq+ssq*0]
    movu                 m6, [srcq+ssq*1]
    pmaddwd              m0, m3
    pmaddwd              m7, m3
    pmaddwd              m1, m4
    pmaddwd              m8, m4
    pmaddwd              m2, m5
    pmaddwd              m9, m5
    paddd                m1, m0
    paddd                m8, m7
%if ARCH_X86_64
    movu                 m0, [srcq+r4]
    movu                 m7, [srcq+r6]
%else
    movu                 m0, [r4+ssq*0]
    movu                 m7, [r4+ssq*1]
    lea                  r4, [r4+ssq*2]
%endif
    lea                srcq, [srcq+ssq*2]
    paddd                m1, m2
    paddd                m8, m9
    pshufb              m11, m12
    pshufb               m6, m12
    pmaddwd             m11, m13
    pmaddwd              m6, m13
    pshufb               m0, m14
    pshufb               m7, m14
    pmaddwd              m0, m15
    pmaddwd              m7, m15
    phaddd              m11, m0
    phaddd               m6, m7
    paddd               m11, hrnd_mem
    paddd                m6, hrnd_mem
    psrad               m11, hsh_mem
    psrad                m6, hsh_mem
    packssdw            m11, m6                     ; 7 8
%if ARCH_X86_64
    shufps               m9, [stk+0x40], m11, q1032 ; 6 7
    mova                 m0, [stk+0x00]
    mova         [stk+0x40], m11
%else
    shufps               m9, [stk+0xa0], m11, q1032 ; 6 7
    mova                 m0, [stk+0x60]
    mova         [stk+0xa0], m11
%endif
    punpcklwd            m2, m9, m11 ; 67
    punpckhwd            m9, m11     ; 78
    pmaddwd              m6, m2, m10
    pmaddwd              m7, m9, m10
%if isput
    movd                m11, vsh_mem
%endif
    paddd                m1, vrnd_mem
    paddd                m8, vrnd_mem
    paddd                m1, m6
    paddd                m8, m7
%if ARCH_X86_64
    mova                 m7, [stk+0x10]
%else
    mova                 m7, [stk+0x80]
%endif
%if isput
    psrad                m1, m11
    psrad                m8, m11
%else
    psrad                m1, 6
    psrad                m8, 6
%endif
    packssdw             m1, m8
%if ARCH_X86_64
    mova                 m8, [stk+0x30]
%else
    mova                 m8, [stk+0x90]
%endif
%if isput
    pxor                 m6, m6
    pmaxsw               m1, m6
    pminsw               m1, pxmaxm
    movq       [dstq+dsq*0], m1
    movhps     [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
%else
    mova             [tmpq], m1
    add                tmpq, 16
%endif
%if ARCH_X86_64
    mova                 m1, [stk+0x20]
    mova         [stk+0x10], m8
    mova         [stk+0x00], m1
    mova         [stk+0x20], m2
    mova         [stk+0x30], m9
%else
    mova                 m1, [stk+0x70]
    mova         [stk+0x80], m8
    mova         [stk+0x60], m1
    mova         [stk+0x70], m2
    mova         [stk+0x90], m9
%endif
    sub                  hd, 2
    jg .dy1_w4_loop
    MC_8TAP_SCALED_RET ; why not jz .ret?
INIT_XMM ssse3
.dy1_w8:
    mov    dword [stk+0xf0], 1
    movifprep   tmp_stridem, 16
    jmp .dy1_w_start
.dy1_w16:
    mov    dword [stk+0xf0], 2
    movifprep   tmp_stridem, 32
    jmp .dy1_w_start
.dy1_w32:
    mov    dword [stk+0xf0], 4
    movifprep   tmp_stridem, 64
    jmp .dy1_w_start
.dy1_w64:
    mov    dword [stk+0xf0], 8
    movifprep   tmp_stridem, 128
    jmp .dy1_w_start
.dy1_w128:
    mov    dword [stk+0xf0], 16
    movifprep   tmp_stridem, 256
.dy1_w_start:
    mov                 myd, mym
%if ARCH_X86_64
 %ifidn %1, put
    movifnidn           dsm, dsq
 %endif
    mova         [rsp+0x10], m11
    mova         [rsp+0x20], m12
 %define hround m11
 %if isput
    mova         [rsp+0x30], m13
 %else
    mova                m13, [base+pd_m524256]
 %endif
    shr                 t0d, 16
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    movd                m15, t0d
%else
 %define hround [esp+0x00]
 %define m12    [esp+0x10]
 %define m10    [base+pd_0x3ff]
 %define m8  m0
 %xdefine m14 m4
 %xdefine m15 m3
 %if isprep
  %define ssq ssm
 %endif
    mov                  r5, [esp+0x1f0]
    mov                  r3, [esp+0x1f4]
    shr                  r5, 16
    movd                m15, r5
    xor                  r5, r5
    shr                 myd, 6
    lea                  r3, [r3+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r3*8+0]
    cmovnz               r5, [base+subpel_filters+r3*8+4]
    mov                  r0, r0m
    mov                  r3, r3m
%endif
    sub                srcq, 6
    pslld                m7, m8, 2 ; dx*4
    pmaddwd              m8, [base+rescale_mul] ; dx*[0-3]
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
%if ARCH_X86_64
    movq                 m3, r4q
%else
    movd                 m5, r4
    movd                 m6, r5
    punpckldq            m5, m6
    SWAP                 m3, m5
%endif
    punpcklbw            m3, m3
    psraw                m3, 8
    mova        [stk+0x100], m7
    mova        [stk+0x120], m15
    mov         [stk+0x0f8], srcq
    mov         [stk+0x130], r0q ; dstq / tmpq
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
%if ARCH_X86_64
    mova        [stk+0x140], m0
    mova        [stk+0x150], m1
    mova        [stk+0x160], m2
    mova        [stk+0x170], m3
 %if UNIX64
    mov                  hm, hd
 %endif
%else
    mova        [stk+0x180], m0
    mova        [stk+0x190], m1
    mova        [stk+0x1a0], m2
    mova        [stk+0x1b0], m3
    SWAP                 m5, m3
    mov                  r5, hm
    mov         [stk+0x134], r5
%endif
    jmp .dy1_hloop
.dy1_hloop_prep:
    dec   dword [stk+0x0f0]
    jz .ret
%if ARCH_X86_64
    add   qword [stk+0x130], 16
    mov                  hd, hm
%else
    add   dword [stk+0x130], 16
    mov                  r5, [stk+0x134]
    mov                  r0, [stk+0x130]
%endif
    mova                 m7, [stk+0x100]
    mova                m14, [stk+0x110]
%if ARCH_X86_64
    mova                m10, [base+pd_0x3ff]
    mova                m11, [rsp+0x10]
%endif
    mova                m15, [stk+0x120]
    mov                srcq, [stk+0x0f8]
%if ARCH_X86_64
    mov                 r0q, [stk+0x130] ; dstq / tmpq
%else
    mov                  hm, r5
    mov                 r0m, r0
    mov                  r3, r3m
%endif
    paddd               m14, m7
.dy1_hloop:
%if ARCH_X86_64
    mova                 m9, [base+pq_0x40000000]
%else
 %define m9 [base+pq_0x40000000]
%endif
    pxor                 m1, m1
    psrld                m2, m14, 10
    mova              [stk], m2
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd                m5, m15, m6
    pcmpeqd              m6, m1
    pshufd               m2, m5, q1032
%if ARCH_X86_64
    movd                r4d, m5
    movd                r6d, m2
    pshufd               m5, m5, q0321
    pshufd               m2, m2, q0321
    movd                r7d, m5
    movd                r9d, m2
    movq                 m0, [base+subpel_filters+r4*8]
    movq                 m1, [base+subpel_filters+r6*8]
    movhps               m0, [base+subpel_filters+r7*8]
    movhps               m1, [base+subpel_filters+r9*8]
%else
    movd                 r0, m5
    movd                 rX, m2
    pshufd               m5, m5, q0321
    pshufd               m2, m2, q0321
    movd                 r4, m5
    movd                 r5, m2
    movq                 m0, [base+subpel_filters+r0*8]
    movq                 m1, [base+subpel_filters+rX*8]
    movhps               m0, [base+subpel_filters+r4*8]
    movhps               m1, [base+subpel_filters+r5*8]
%endif
    paddd               m14, m7 ; mx+dx*[4-7]
    pand                 m5, m14, m10
    psrld                m5, 6
    paddd               m15, m5
    pxor                 m2, m2
    pcmpeqd              m5, m2
    mova        [stk+0x110], m14
    pshufd               m4, m15, q1032
%if ARCH_X86_64
    movd               r10d, m15
    movd               r11d, m4
    pshufd              m15, m15, q0321
    pshufd               m4, m4, q0321
    movd               r13d, m15
    movd                rXd, m4
    movq                 m2, [base+subpel_filters+r10*8]
    movq                 m3, [base+subpel_filters+r11*8]
    movhps               m2, [base+subpel_filters+r13*8]
    movhps               m3, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    movq                r11, m14
    punpckhqdq          m14, m14
    movq                 rX, m14
    mov                r10d, r11d
    shr                 r11, 32
    mov                r13d, rXd
    shr                  rX, 32
    mov                 r4d, [stk+ 0]
    mov                 r6d, [stk+ 4]
    mov                 r7d, [stk+ 8]
    mov                 r9d, [stk+12]
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd              m14, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m7, m9, m4
    pand                 m8, m9, m6
    pand                m15, m9, m14
    pand                 m9, m9, m5
    pandn                m4, m0
    pandn                m6, m1
    pandn               m14, m2
    pandn                m5, m3
    por                  m7, m4
    por                  m8, m6
    por                 m15, m14
    por                  m9, m5
    punpcklbw            m0, m7, m7
    punpckhbw            m7, m7
    punpcklbw            m1, m8, m8
    punpckhbw            m8, m8
    psraw                m0, 8
    psraw                m7, 8
    psraw                m1, 8
    psraw                m8, 8
    punpcklbw            m2, m15, m15
    punpckhbw           m15, m15
    punpcklbw            m3, m9, m9
    punpckhbw            m9, m9
    psraw                m2, 8
    psraw               m15, 8
    psraw                m3, 8
    psraw                m9, 8
    mova         [stk+0x10], m0
    mova         [stk+0x20], m7
    mova         [stk+0x30], m1
    mova         [stk+0x40], m8
    mova         [stk+0x50], m2
    mova         [stk+0x60], m15
    mova         [stk+0x70], m3
    mova         [stk+0x80], m9
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 9, 10 ; 0
    mova         [stk+0x90], m1
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 1, 9, 10 ; 1
    mova         [stk+0xa0], m2
    MC_8TAP_SCALED_H 3, 4, 5, 6, 1, 2, 9, 10 ; 2
    mova         [stk+0xb0], m3
    MC_8TAP_SCALED_H 4, 5, 6, 1, 2, 3, 9, 10 ; 3
    mova         [stk+0xc0], m4
    MC_8TAP_SCALED_H 5, 6, 1, 2, 3, 4, 9, 10 ; 4
    mova         [stk+0xd0], m5
    MC_8TAP_SCALED_H 6, 1, 2, 3, 4, 5, 9, 10 ; 5
    MC_8TAP_SCALED_H 7, 1, 2, 3, 4, 5, 9, 10 ; 6
    MC_8TAP_SCALED_H 8, 1, 2, 3, 4, 5, 9, 10 ; 7
    mova                 m5, [stk+0xd0]
    mova                 m1, [stk+0x90]
    mova                 m2, [stk+0xa0]
    mova                 m3, [stk+0xb0]
    mova                 m9, [stk+0xc0]
    punpcklwd            m4, m5, m6 ; 45a
    punpckhwd            m5, m6     ; 45b
    punpcklwd            m6, m7, m8 ; 67a
    punpckhwd            m7, m8     ; 67b
    punpcklwd            m0, m1, m2 ; 01a
    punpckhwd            m1, m2     ; 01b
    punpcklwd            m2, m3, m9 ; 23a
    punpckhwd            m3, m9     ; 23b
    mova                m10, [stk+0x140]
    mova                m11, [stk+0x150]
    mova                m14, [stk+0x160]
    mova                m15, [stk+0x170]
    mova         [stk+0x90], m4
    mova         [stk+0xa0], m5
    mova         [stk+0xb0], m6
    mova         [stk+0xc0], m7
 %define hround [rsp+0x10]
 %define shift  [rsp+0x20]
 %if isput
  %define vround [rsp+0x30]
 %else
  %define vround [base+pd_m524256]
 %endif
.dy1_vloop:
    pmaddwd              m4, m0, m10
    pmaddwd              m5, m1, m10
    pmaddwd              m6, m2, m11
    pmaddwd              m7, m3, m11
    paddd                m4, m13
    paddd                m5, m13
    paddd                m4, m6
    paddd                m5, m7
    pmaddwd              m6, [stk+0x90], m14
    pmaddwd              m7, [stk+0xa0], m14
    pmaddwd              m8, [stk+0xb0], m15
    pmaddwd              m9, [stk+0xc0], m15
    paddd                m4, m6
    paddd                m5, m7
 %if isput
    pshufd               m6, m12, q1032
 %endif
    paddd                m4, m8
    paddd                m5, m9
%else
    movd                 r0, m15
    movd                 rX, m4
    pshufd              m15, m15, q0321
    pshufd               m4, m4, q0321
    movd                 r4, m15
    movd                 r5, m4
    mova                m14, [stk+0x110]
    movq                 m2, [base+subpel_filters+r0*8]
    movq                 m3, [base+subpel_filters+rX*8]
    movhps               m2, [base+subpel_filters+r4*8]
    movhps               m3, [base+subpel_filters+r5*8]
    psrld               m14, 10
    mova           [stk+16], m14
    mov                  r0, [stk+ 0]
    mov                  rX, [stk+ 4]
    mov                  r4, [stk+ 8]
    mov                  r5, [stk+12]
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m0, m9, m4
    pand                 m1, m9, m6
    pand                 m2, m9, m7
    pand                 m3, m9, m5
    pandn                m4, [stk+0x20]
    pandn                m6, [stk+0x30]
    pandn                m7, [stk+0x40]
    pandn                m5, [stk+0x50]
    por                  m0, m4
    por                  m1, m6
    por                  m2, m7
    por                  m3, m5
    punpcklbw            m4, m0, m0
    punpckhbw            m0, m0
    punpcklbw            m5, m1, m1
    punpckhbw            m1, m1
    psraw                m4, 8
    psraw                m0, 8
    psraw                m5, 8
    psraw                m1, 8
    punpcklbw            m6, m2, m2
    punpckhbw            m2, m2
    punpcklbw            m7, m3, m3
    punpckhbw            m3, m3
    psraw                m6, 8
    psraw                m2, 8
    psraw                m7, 8
    psraw                m3, 8
    mova        [stk+0x0a0], m4
    mova        [stk+0x0b0], m0
    mova        [stk+0x0c0], m5
    mova        [stk+0x0d0], m1
    mova        [stk+0x140], m6
    mova        [stk+0x150], m2
    mova        [stk+0x160], m7
    mova        [stk+0x170], m3
    MC_8TAP_SCALED_H   0xa0, 0x20, 0 ; 0
    MC_8TAP_SCALED_H   0xa0, 0x30    ; 1
    MC_8TAP_SCALED_H   0xa0, 0x40    ; 2
    MC_8TAP_SCALED_H   0xa0, 0x50    ; 3
    MC_8TAP_SCALED_H   0xa0, 0x60    ; 4
    MC_8TAP_SCALED_H   0xa0, 0x70    ; 5
    MC_8TAP_SCALED_H   0xa0, 0x80    ; 6
    MC_8TAP_SCALED_H   0xa0, 0x90    ; 7
    mova                 m5, [stk+0x60]
    mova                 m6, [stk+0x70]
    mova                 m7, [stk+0x80]
    mova                 m0, [stk+0x90]
    mov                  r0, r0m
    punpcklwd            m4, m5, m6      ; 45a
    punpckhwd            m5, m6          ; 45b
    punpcklwd            m6, m7, m0      ; 67a
    punpckhwd            m7, m0          ; 67b
    mova         [stk+0x60], m4
    mova         [stk+0x70], m5
    mova         [stk+0x80], m6
    mova         [stk+0x90], m7
    mova                 m1, [stk+0x20]
    mova                 m2, [stk+0x30]
    mova                 m3, [stk+0x40]
    mova                 m4, [stk+0x50]
    punpcklwd            m0, m1, m2      ; 01a
    punpckhwd            m1, m2          ; 01b
    punpcklwd            m2, m3, m4      ; 23a
    punpckhwd            m3, m4          ; 23b
    mova                 m4, [stk+0x180]
    mova                 m5, [stk+0x190]
    mova                 m6, [stk+0x1a0]
    mova                 m7, [stk+0x1b0]
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
.dy1_vloop:
    pmaddwd              m0, m4
    pmaddwd              m1, m4
    pmaddwd              m2, m5
    pmaddwd              m3, m5
    paddd                m0, m2
    paddd                m1, m3
    pmaddwd              m2, [stk+0x60], m6
    pmaddwd              m3, [stk+0x70], m6
    pmaddwd              m4, [stk+0x80], m7
    pmaddwd              m5, [stk+0x90], m7
 %if isput
    movd                 m6, [esp+0x18]
 %endif
    paddd                m0, m2
    paddd                m1, m3
    paddd                m0, vrnd_mem
    paddd                m1, vrnd_mem
    paddd                m4, m0
    paddd                m5, m1
%endif
%ifidn %1, put
    psrad                m4, m6
    psrad                m5, m6
    packssdw             m4, m5
    pxor                 m7, m7
    pmaxsw               m4, m7
    pminsw               m4, pxmaxm
    mova             [dstq], m4
    add                dstq, dsm
%else
    psrad                m4, 6
    psrad                m5, 6
    packssdw             m4, m5
    mova             [tmpq], m4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .dy1_hloop_prep
%if ARCH_X86_64
    movu                 m8, [srcq+r10*2]
    movu                 m9, [srcq+r11*2]
    movu                m12, [srcq+r13*2]
    movu                m13, [srcq+ rX*2]
    movu                 m4, [srcq+ r4*2]
    movu                 m5, [srcq+ r6*2]
    movu                 m6, [srcq+ r7*2]
    movu                 m7, [srcq+ r9*2]
    add                srcq, ssq
    pmaddwd              m8, [stk+0x50]
    pmaddwd              m9, [stk+0x60]
    pmaddwd             m12, [stk+0x70]
    pmaddwd             m13, [stk+0x80]
    pmaddwd              m4, [stk+0x10]
    pmaddwd              m5, [stk+0x20]
    pmaddwd              m6, [stk+0x30]
    pmaddwd              m7, [stk+0x40]
    phaddd               m8, m9
    phaddd              m12, m13
    mova                 m9, [base+unpckw]
    mova                m13, hround
    phaddd               m4, m5
    phaddd               m6, m7
    phaddd               m8, m12
    phaddd               m4, m6
    pshufd               m5, m9, q1032
    pshufb               m0, m9             ; 0a 1a
    pshufb               m1, m9             ; 0b 1b
    pshufb               m2, m5             ; 3a 2a
    pshufb               m3, m5             ; 3b 2b
    mova                m12, shift
    paddd                m4, m13
    paddd                m8, m13
    psrad                m4, m12
    psrad                m8, m12
    packssdw             m4, m8
    pshufb               m6, [stk+0x90], m9 ; 4a 5a
    pshufb               m7, [stk+0xa0], m9 ; 4b 5b
    pshufb               m8, [stk+0xb0], m5 ; 7a 6a
    pshufb              m13, [stk+0xc0], m5 ; 7b 6b
    punpckhwd            m0, m2  ; 12a
    punpckhwd            m1, m3  ; 12b
    punpcklwd            m2, m6  ; 34a
    punpcklwd            m3, m7  ; 34b
    punpckhwd            m6, m8  ; 56a
    punpckhwd            m7, m13 ; 56b
    punpcklwd            m8, m4  ; 78a
    punpckhqdq           m4, m4
    punpcklwd           m13, m4  ; 78b
    mova         [stk+0x90], m6
    mova         [stk+0xa0], m7
    mova         [stk+0xb0], m8
    mova         [stk+0xc0], m13
    mova                m13, vround
%else
    mov                 r0m, r0
    mov                  r3, r3m
    mov                  r0, [stk+ 0]
    mov                  rX, [stk+ 4]
    mov                  r4, [stk+ 8]
    mov                  r5, [stk+12]
    MC_8TAP_SCALED_H 0xa0, 0xe0, 0 ; 8
    mova                 m7, [base+unpckw]
    pshufd               m4, m7, q1032
    pshufb               m0, [stk+0x20], m7 ; 0a 1a
    pshufb               m1, [stk+0x30], m7 ; 0b 1b
    pshufb               m2, [stk+0x40], m4 ; 3a 2a
    pshufb               m3, [stk+0x50], m4 ; 3b 2b
    pshufb               m5, [stk+0x60], m7 ; 4a 5a
    pshufb               m6, [stk+0x70], m7 ; 4b 5b
    pshufb               m7, [stk+0x80], m4 ; 7a 6a
    punpckhwd            m0, m2 ; 12a
    punpckhwd            m1, m3 ; 12b
    punpcklwd            m2, m5 ; 34a
    punpcklwd            m3, m6 ; 34b
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
    punpckhwd            m5, m7 ; 56a
    mova         [stk+0x60], m5
    pshufb               m5, [stk+0x90], m4 ; 7b 6b
    punpcklwd            m7, [stk+0xe0] ; 78a
    mova                 m4, [stk+0x180]
    punpckhwd            m6, m5 ; 56b
    mova         [stk+0x70], m6
    movq                 m6, [stk+0xe8]
    mova         [stk+0x80], m7
    mova                 m7, [stk+0x1b0]
    punpcklwd            m5, m6
    mova                 m6, [stk+0x1a0]
    mova         [stk+0x90], m5
    mova                 m5, [stk+0x190]
    mov                  r0, r0m
%endif
    jmp .dy1_vloop
INIT_XMM ssse3
%if ARCH_X86_64
 %define stk rsp+0x20
%endif
.dy2:
    movzx                wd, word [base+%1_8tap_scaled_ssse3_dy2_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%if isput
.dy2_w2:
 %if ARCH_X86_64
    mov                 myd, mym
    mova         [rsp+0x10], m13
  %define vrnd_mem [rsp+0x10]
    movzx               t0d, t0b
    sub                srcq, 2
    movd                m15, t0d
 %else
  %define m8  m0
  %define m9  m1
  %define m14 m4
  %define m15 m3
  %define m11 [esp+0x00]
  %define m12 [esp+0x10]
  %define vrnd_mem [esp+0x20]
    mov                  r1, r1m
    movzx                r5, byte [esp+0x1f0]
    sub                srcq, 2
    movd                m15, r5
 %endif
    pxor                 m9, m9
    punpckldq            m9, m8
    paddd               m14, m9 ; mx+dx*[0-1]
 %if ARCH_X86_64
    mova                 m9, [base+pd_0x4000]
 %endif
    pshufd              m15, m15, q0000
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    movd                r4d, m15
    pshufd              m15, m15, q0321
 %if ARCH_X86_64
    movd                r6d, m15
 %else
    movd                r3d, m15
 %endif
    mova                 m5, [base+bdct_lb_q]
    mova                 m6, [base+spel_s_shuf2]
    movd                m15, [base+subpel_filters+r4*8+2]
 %if ARCH_X86_64
    movd                 m7, [base+subpel_filters+r6*8+2]
 %else
    movd                 m7, [base+subpel_filters+r3*8+2]
 %endif
    pxor                 m2, m2
    pcmpeqd              m8, m2
    psrld               m14, 10
    paddd               m14, m14
 %if ARCH_X86_32
    mov                  r3, r3m
    pshufb              m14, m5
    paddb               m14, m6
    mova              [stk], m14
    SWAP                 m5, m0
    SWAP                 m6, m3
  %define m15 m6
 %endif
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*2]
    movu                 m2, [srcq+ssq*4]
    punpckldq           m15, m7
 %if ARCH_X86_64
    pshufb              m14, m5
    paddb               m14, m6
    pand                 m9, m8
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m9
    movu                 m4, [srcq+ssq*1]
    movu                 m5, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    movu                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
 %else
    pand                 m7, m5, [base+pd_0x4000]
    pandn                m5, m15
    por                  m5, m7
  %define m15 m5
    mov                 myd, mym
    mov                  r5, [esp+0x1f4]
    xor                  r3, r3
    shr                 myd, 6
    lea                  r5, [r5+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r5*8+0]
    cmovnz               r3, [base+subpel_filters+r5*8+4]
    mov          [stk+0x20], r3
    mov                  r3, r3m
 %endif
    punpcklbw           m15, m15
    psraw               m15, 8
    REPX    {pshufb x, m14}, m0, m1, m2
    REPX   {pmaddwd x, m15}, m0, m1, m2
 %if ARCH_X86_64
    REPX    {pshufb x, m14}, m4, m5, m6
    REPX   {pmaddwd x, m15}, m4, m5, m6
    phaddd               m0, m1
    phaddd               m1, m2
    phaddd               m4, m5
    phaddd               m5, m6
    REPX     {paddd x, m11}, m0, m1, m4, m5
    REPX     {psrad x, m12}, m0, m1, m4, m5
    packssdw             m0, m1 ; 0 2 2 4
    packssdw             m4, m5 ; 1 3 3 5
    SWAP                 m2, m4
    movq                m10, r4
 %else
    mova         [stk+0x10], m15
    phaddd               m0, m1
    phaddd               m1, m2
    movu                 m2, [srcq+ssq*1]
    movu                 m7, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    movu                 m6, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    REPX    {pshufb x, m14}, m2, m7, m6
    REPX   {pmaddwd x, m15}, m2, m7, m6
  %define m14 [stk+0x00]
  %define m15 [stk+0x10]
    phaddd               m2, m7
    phaddd               m7, m6
    REPX     {paddd x, m11}, m0, m1, m2, m7
    REPX     {psrad x, m12}, m0, m1, m2, m7
    packssdw             m0, m1
    packssdw             m2, m7
  %define m8  m6
  %define m9  m4
  %define m10 m5
    movd                m10, r4
    movd                 m9, [stk+0x20]
    punpckldq           m10, m9
 %endif
    punpcklbw           m10, m10
    psraw               m10, 8
    pshufd               m7, m10, q0000
    pshufd               m8, m10, q1111
    pshufd               m9, m10, q2222
    pshufd              m10, m10, q3333
 %if ARCH_X86_32
    mova         [stk+0x50], m7
    mova         [stk+0x60], m8
    mova         [stk+0x70], m9
    mova         [stk+0x80], m10
  %xdefine m13 m7
  %define m7  [stk+0x50]
  %define m8  [stk+0x60]
  %define m9  [stk+0x70]
  %define m10 [stk+0x80]
 %endif
    punpcklwd            m1, m0, m2    ; 01 23
    punpckhwd            m3, m0, m2    ; 23 45
 %if ARCH_X86_32
    mov                  r4, r0m
  %define dstq r4
    mova         [stk+0x20], m3
    mova         [stk+0x30], m0
 %endif
.dy2_w2_loop:
    movu                 m4, [srcq+ssq*0]
    movu                 m5, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    movu                m13, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pmaddwd              m3, m8
    REPX    {pshufb x, m14}, m4, m5, m6, m13
    REPX   {pmaddwd x, m15}, m4, m5, m6, m13
    phaddd               m4, m5
    phaddd               m6, m13
    pmaddwd              m5, m1, m7
    paddd                m4, m11
    paddd                m6, m11
    psrad                m4, m12
    psrad                m6, m12
    packssdw             m4, m6 ; 6 7 8 9
    paddd                m5, m3
    pshufd               m3, m4, q2200
    pshufd               m4, m4, q3311
    palignr              m3, m0, 12 ; 4 6 6 8
    palignr              m4, m2, 12 ; 5 7 7 9
    mova                 m0, m3
    mova                 m2, m4
    punpcklwd            m1, m3, m4
    punpckhwd            m3, m4
    pmaddwd              m6, m1, m9
    pmaddwd              m4, m3, m10
    paddd                m5, vrnd_mem
    paddd                m6, m4
    paddd                m5, m6
    pshufd               m4, m12, q1032
    pxor                 m6, m6
    psrad                m5, m4
    packssdw             m5, m5
    pmaxsw               m5, m6
    pminsw               m5, pxmaxm
    movd       [dstq+dsq*0], m5
    pshuflw              m5, m5, q1032
    movd       [dstq+dsq*1], m5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy2_w2_loop
    RET
%endif
INIT_XMM ssse3
.dy2_w4:
%if ARCH_X86_64
    mov                 myd, mym
    mova         [rsp+0x10], m11
    mova         [rsp+0x20], m12
 %if isput
    mova         [rsp+0x30], m13
  %define vrnd_mem [rsp+0x30]
  %define stk rsp+0x40
 %else
  %define vrnd_mem [base+pd_m524256]
  %define stk rsp+0x30
 %endif
    movzx               t0d, t0b
    sub                srcq, 2
    movd                m15, t0d
%else
 %define m10 [base+pd_0x3ff]
 %define m9  [base+pd_0x4000]
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
 %if isprep
  %define ssq r3
 %endif
    movzx                r5, byte [esp+0x1f0]
    sub                srcq, 2
    movd                m15, r5
%endif
    pmaddwd              m8, [base+rescale_mul]
%if ARCH_X86_64
    mova                 m9, [base+pd_0x4000]
%endif
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m0, m14, m10
    psrld                m0, 6
    paddd               m15, m0
    pshufd               m7, m15, q1032
%if ARCH_X86_64
    movd                r4d, m15
    movd               r11d, m7
    pshufd              m15, m15, q0321
    pshufd               m7, m7, q0321
    movd                r6d, m15
    movd               r13d, m7
    mova                m10, [base+bdct_lb_q+ 0]
    mova                m11, [base+bdct_lb_q+16]
    movd                m13, [base+subpel_filters+ r4*8+2]
    movd                 m2, [base+subpel_filters+ r6*8+2]
    movd                m15, [base+subpel_filters+r11*8+2]
    movd                 m4, [base+subpel_filters+r13*8+2]
%else
    movd                 r1, m15
    movd                 r4, m7
    pshufd              m15, m15, q0321
    pshufd               m7, m7, q0321
    movd                 r3, m15
    movd                 r5, m7
    mova                 m5, [base+bdct_lb_q+ 0]
    mova                 m6, [base+bdct_lb_q+16]
    movd                 m1, [base+subpel_filters+r1*8+2]
    movd                 m2, [base+subpel_filters+r3*8+2]
    movd                 m3, [base+subpel_filters+r4*8+2]
    movd                 m7, [base+subpel_filters+r5*8+2]
    SWAP                 m4, m7
    mov                  r3, r3m
 %if isprep
    lea                ss3q, [ssq*3]
 %endif
 %define m10 m5
 %define m11 m6
 %define m12 m1
 %define m13 m1
%endif
    psrld               m14, 10
    paddd               m14, m14
    punpckldq           m13, m2
    punpckldq           m15, m4
    punpcklqdq          m13, m15
    pxor                 m2, m2
    pcmpeqd              m0, m2
%if ARCH_X86_64
    pand                 m9, m0
%else
    pand                 m2, m9, m0
 %define m9 m2
    SWAP                 m7, m4
%endif
    pandn                m0, m13
%if ARCH_X86_64
    SWAP                m13, m0
%else
 %define m13 m0
%endif
    por                 m13, m9
    punpckhbw           m15, m13, m13
    punpcklbw           m13, m13
    psraw               m15, 8
    psraw               m13, 8
    pshufb              m12, m14, m10
    pshufb              m14, m11
    mova                m10, [base+spel_s_shuf2]
    movd                r4d, m14
    shr                 r4d, 24
%if ARCH_X86_32
    mova         [stk+0x40], m13
    mova         [stk+0x50], m15
    pxor                 m2, m2
%endif
    pshufb               m7, m14, m2
    psubb               m14, m7
    paddb               m12, m10
    paddb               m14, m10
%if ARCH_X86_64
    lea                  r6, [r4+ssq*1]
    lea                 r11, [r4+ssq*2]
    lea                 r13, [r4+ss3q ]
    movu                 m1, [srcq+ssq*0]
    movu                 m8, [srcq+ssq*2]
    movu                 m9, [srcq+ssq*1]
    movu                m10, [srcq+ss3q ]
    movu                 m7, [srcq+r4   ]
    movu                 m2, [srcq+r11  ]
    movu                 m3, [srcq+r6   ]
    movu                 m4, [srcq+r13  ]
    lea                srcq, [srcq+ssq*4]
    REPX    {pshufb x, m12}, m1, m9, m8, m10
    REPX   {pmaddwd x, m13}, m1, m9, m8, m10
    REPX    {pshufb x, m14}, m7, m3, m2, m4
    REPX   {pmaddwd x, m15}, m7, m3, m2, m4
    mova                 m5, [rsp+0x10]
    movd                xm6, [rsp+0x20]
    phaddd               m1, m7
    phaddd               m8, m2
    phaddd               m9, m3
    phaddd              m10, m4
    movu                 m2, [srcq+ssq*0]
    movu                 m3, [srcq+ssq*1]
    REPX      {paddd x, m5}, m1, m9, m8, m10
    REPX     {psrad x, xm6}, m1, m9, m8, m10
    packssdw             m1, m8     ; 0 2
    packssdw             m9, m10    ; 1 3
    movu                 m0, [srcq+r4   ]
    movu                 m8, [srcq+r6   ]
    lea                srcq, [srcq+ssq*2]
    REPX    {pshufb x, m12}, m2, m3
    REPX   {pmaddwd x, m13}, m2, m3
    REPX    {pshufb x, m14}, m0, m8
    REPX   {pmaddwd x, m15}, m0, m8
    phaddd               m2, m0
    phaddd               m3, m8
    shr                 myd, 6
    mov                 r9d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r9q, [base+subpel_filters+myq*8]
    REPX      {paddd x, m5}, m2, m3
    REPX     {psrad x, xm6}, m2, m3
    packssdw             m2, m3        ; 4 5
    pshufd               m3, m2, q1032 ; 5 _
    punpcklwd            m0, m1, m9    ; 01
    punpckhwd            m1, m9        ; 23
    punpcklwd            m2, m3        ; 45
    movq                m10, r9
 %define hrnd_mem [rsp+0x10]
 %define hsh_mem  [rsp+0x20]
 %define vsh_mem  [rsp+0x28]
 %if isput
  %define vrnd_mem [rsp+0x30]
 %else
  %define vrnd_mem [base+pd_m524256]
 %endif
%else
    mova         [stk+0x20], m12
    mova         [stk+0x30], m14
    add                  r4, srcq
    MC_4TAP_SCALED_H   0x60 ; 0 1
    MC_4TAP_SCALED_H   0x70 ; 2 3
    MC_4TAP_SCALED_H   0x80 ; 4 5
    mov          [stk+0xe0], r4
    mova                 m3, [base+spel_s_shuf8]
    mova                 m0, [stk+0x60]
    mova                 m1, [stk+0x70]
    mova                 m2, [stk+0x80]
    mov                 myd, mym
    mov                  rX, [esp+0x1f4]
    xor                  r5, r5
    shr                 myd, 6
    lea                  rX, [rX+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+rX*8+0]
    cmovnz               r5, [base+subpel_filters+rX*8+4]
    mov                  r3, r3m
    pshufb               m0, m3 ; 01
    pshufb               m1, m3 ; 23
    pshufb               m2, m3 ; 45
    movd                 m7, r4
    movd                 m4, r5
    mov                  r5, r0m
 %if isput
    mov                  r1, r1m
 %endif
    mov                  r4, [stk+0xe0]
 %define dstq r5
 %define tmpq r5
 %define m12 [stk+0x20]
 %define m14 [stk+0x30]
 %define m13 [stk+0x40]
 %define m15 [stk+0x50]
 %define hrnd_mem [esp+0x00]
 %define hsh_mem  [esp+0x10]
 %define vsh_mem  [esp+0x18]
 %if isput
  %define vrnd_mem [esp+0x20]
 %else
  %define vrnd_mem [base+pd_m524256]
 %endif
 %define m10 m7
    punpckldq           m10, m4
%endif
    punpcklbw           m10, m10
    psraw               m10, 8
    pshufd               m3, m10, q0000
    pshufd               m4, m10, q1111
    pshufd               m5, m10, q2222
    pshufd              m10, m10, q3333
%if ARCH_X86_32
 %xdefine m8  m3
 %xdefine m9  m6
 %xdefine m11 m5
 %xdefine m6  m4
    mova         [stk+0x100], m3
    mova         [stk+0x110], m4
    mova         [stk+0x120], m5
    mova         [stk+0x130], m10
 %define m3  [stk+0x100]
 %define m4  [stk+0x110]
 %define m5  [stk+0x120]
 %define m10 [stk+0x130]
%endif
.dy2_w4_loop:
    pmaddwd              m8, m0, m3
    pmaddwd              m9, m1, m3
    mova                 m0, m2
    pmaddwd              m1, m4
    pmaddwd             m11, m2, m4
    paddd                m8, vrnd_mem
    paddd                m9, vrnd_mem
    pmaddwd              m2, m5
    paddd                m8, m1
    paddd                m9, m11
    paddd                m8, m2
    movu                 m6, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*2]
%if ARCH_X86_64
    movu                m11, [srcq+r4 ]
    movu                 m2, [srcq+r11]
%else
    movu                m11, [r4+ssq*0]
    movu                 m2, [r4+ssq*2]
%endif
    pshufb               m6, m12
    pshufb               m1, m12
    pmaddwd              m6, m13
    pmaddwd              m1, m13
    pshufb              m11, m14
    pshufb               m2, m14
    pmaddwd             m11, m15
    pmaddwd              m2, m15
    phaddd               m6, m11
    phaddd               m1, m2
    paddd                m6, hrnd_mem
    paddd                m1, hrnd_mem
    psrad                m6, hsh_mem
    psrad                m1, hsh_mem
    movu                 m7, [srcq+ssq*1]
    movu                m11, [srcq+ss3q ]
    packssdw             m6, m1 ; 6 8
%if ARCH_X86_64
    movu                 m2, [srcq+r6 ]
    movu                 m1, [srcq+r13]
%else
    movu                 m2, [r4+ssq*1]
    movu                 m1, [r4+ss3q ]
%endif
    pshufb               m7, m12
    pshufb              m11, m12
    pmaddwd              m7, m13
    pmaddwd             m11, m13
    pshufb               m2, m14
    pshufb               m1, m14
    pmaddwd              m2, m15
    pmaddwd              m1, m15
    phaddd               m7, m2
    phaddd              m11, m1
    paddd                m7, hrnd_mem
    paddd               m11, hrnd_mem
    psrad                m7, hsh_mem
    psrad               m11, hsh_mem
    packssdw             m7, m11 ; 7 9
%if ARCH_X86_32
    lea                  r4, [r4+ssq*4]
%endif
    lea                srcq, [srcq+ssq*4]
    punpcklwd            m1, m6, m7 ; 67
    punpckhwd            m6, m7     ; 89
    mova                 m2, m6
    pmaddwd             m11, m1, m5
    pmaddwd              m7, m1, m10
    pmaddwd              m6, m10
    paddd                m9, m11
%if isput
    movd                m11, vsh_mem
%endif
    paddd                m8, m7
    paddd                m9, m6
%if isput
    psrad                m8, m11
    psrad                m9, m11
    packssdw             m8, m9
    pxor                 m7, m7
    pmaxsw               m8, m7
    pminsw               m8, pxmaxm
    movq       [dstq+dsq*0], m8
    movhps     [dstq+dsq*1], m8
    lea                dstq, [dstq+dsq*2]
%else
    psrad                m8, 6
    psrad                m9, 6
    packssdw             m8, m9
    mova             [tmpq], m8
    add                tmpq, 16
%endif
    sub                  hd, 2
    jg .dy2_w4_loop
    MC_8TAP_SCALED_RET ; why not jz .ret?
INIT_XMM ssse3
.dy2_w8:
    mov    dword [stk+0xf0], 1
    movifprep   tmp_stridem, 16
    jmp .dy2_w_start
.dy2_w16:
    mov    dword [stk+0xf0], 2
    movifprep   tmp_stridem, 32
    jmp .dy2_w_start
.dy2_w32:
    mov    dword [stk+0xf0], 4
    movifprep   tmp_stridem, 64
    jmp .dy2_w_start
.dy2_w64:
    mov    dword [stk+0xf0], 8
    movifprep   tmp_stridem, 128
    jmp .dy2_w_start
.dy2_w128:
    mov    dword [stk+0xf0], 16
    movifprep   tmp_stridem, 256
.dy2_w_start:
    mov                 myd, mym
%if ARCH_X86_64
 %ifidn %1, put
    movifnidn           dsm, dsq
 %endif
    mova         [rsp+0x10], m11
    mova         [rsp+0x20], m12
 %define hround m11
 %if isput
    mova         [rsp+0x30], m13
 %else
    mova                m13, [base+pd_m524256]
 %endif
    shr                 t0d, 16
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    movd                m15, t0d
%else
 %define hround [esp+0x00]
 %define m12    [esp+0x10]
 %define m10    [base+pd_0x3ff]
 %define m8  m0
 %xdefine m14 m4
 %xdefine m15 m3
 %if isput
  %define dstq r0
 %else
  %define tmpq r0
  %define ssq ssm
 %endif
    mov                  r5, [esp+0x1f0]
    mov                  r3, [esp+0x1f4]
    shr                  r5, 16
    movd                m15, r5
    xor                  r5, r5
    shr                 myd, 6
    lea                  r3, [r3+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r3*8+0]
    cmovnz               r5, [base+subpel_filters+r3*8+4]
    mov                  r0, r0m
    mov                  r3, r3m
%endif
    sub                srcq, 6
    pslld                m7, m8, 2 ; dx*4
    pmaddwd              m8, [base+rescale_mul] ; dx*[0-3]
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
%if ARCH_X86_64
    movq                 m3, r4q
%else
    movd                 m5, r4
    movd                 m6, r5
    punpckldq            m5, m6
    SWAP                 m3, m5
%endif
    punpcklbw            m3, m3
    psraw                m3, 8
    mova        [stk+0x100], m7
    mova        [stk+0x120], m15
    mov         [stk+0x0f8], srcq
    mov         [stk+0x130], r0q ; dstq / tmpq
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
%if ARCH_X86_64
    mova        [stk+0x140], m0
    mova        [stk+0x150], m1
    mova        [stk+0x160], m2
    mova        [stk+0x170], m3
 %if UNIX64
    mov                  hm, hd
 %endif
%else
    mova        [stk+0x180], m0
    mova        [stk+0x190], m1
    mova        [stk+0x1a0], m2
    mova        [stk+0x1b0], m3
    SWAP                 m5, m3
    mov                  r5, hm
    mov         [stk+0x134], r5
%endif
    jmp .dy2_hloop
.dy2_hloop_prep:
    dec   dword [stk+0x0f0]
    jz .ret
%if ARCH_X86_64
    add   qword [stk+0x130], 16
    mov                  hd, hm
%else
    add   dword [stk+0x130], 16
    mov                  r5, [stk+0x134]
    mov                  r0, [stk+0x130]
%endif
    mova                 m7, [stk+0x100]
    mova                m14, [stk+0x110]
%if ARCH_X86_64
    mova                m10, [base+pd_0x3ff]
    mova                m11, [rsp+0x10]
%endif
    mova                m15, [stk+0x120]
    mov                srcq, [stk+0x0f8]
%if ARCH_X86_64
    mov                 r0q, [stk+0x130] ; dstq / tmpq
%else
    mov                  hm, r5
    mov                 r0m, r0
    mov                  r3, r3m
%endif
    paddd               m14, m7
.dy2_hloop:
%if ARCH_X86_64
    mova                 m9, [base+pq_0x40000000]
%else
 %define m9 [base+pq_0x40000000]
%endif
    pxor                 m1, m1
    psrld                m2, m14, 10
    mova              [stk], m2
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd                m5, m15, m6
    pcmpeqd              m6, m1
    pshufd               m2, m5, q1032
%if ARCH_X86_64
    movd                r4d, m5
    movd                r6d, m2
    pshufd               m5, m5, q0321
    pshufd               m2, m2, q0321
    movd                r7d, m5
    movd                r9d, m2
    movq                 m0, [base+subpel_filters+r4*8]
    movq                 m1, [base+subpel_filters+r6*8]
    movhps               m0, [base+subpel_filters+r7*8]
    movhps               m1, [base+subpel_filters+r9*8]
%else
    movd                 r0, m5
    movd                 rX, m2
    pshufd               m5, m5, q0321
    pshufd               m2, m2, q0321
    movd                 r4, m5
    movd                 r5, m2
    movq                 m0, [base+subpel_filters+r0*8]
    movq                 m1, [base+subpel_filters+rX*8]
    movhps               m0, [base+subpel_filters+r4*8]
    movhps               m1, [base+subpel_filters+r5*8]
%endif
    paddd               m14, m7 ; mx+dx*[4-7]
    pand                 m5, m14, m10
    psrld                m5, 6
    paddd               m15, m5
    pxor                 m2, m2
    pcmpeqd              m5, m2
    mova        [stk+0x110], m14
    pshufd               m4, m15, q1032
%if ARCH_X86_64
    movd               r10d, m15
    movd               r11d, m4
    pshufd              m15, m15, q0321
    pshufd               m4, m4, q0321
    movd               r13d, m15
    movd                rXd, m4
    movq                 m2, [base+subpel_filters+r10*8]
    movq                 m3, [base+subpel_filters+r11*8]
    movhps               m2, [base+subpel_filters+r13*8]
    movhps               m3, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    movq                r11, m14
    punpckhqdq          m14, m14
    movq                 rX, m14
    mov                r10d, r11d
    shr                 r11, 32
    mov                r13d, rXd
    shr                  rX, 32
    mov                 r4d, [stk+ 0]
    mov                 r6d, [stk+ 4]
    mov                 r7d, [stk+ 8]
    mov                 r9d, [stk+12]
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd              m14, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m7, m9, m4
    pand                 m8, m9, m6
    pand                m15, m9, m14
    pand                 m9, m9, m5
    pandn                m4, m0
    pandn                m6, m1
    pandn               m14, m2
    pandn                m5, m3
    por                  m7, m4
    por                  m8, m6
    por                 m15, m14
    por                  m9, m5
    punpcklbw            m0, m7, m7
    punpckhbw            m7, m7
    punpcklbw            m1, m8, m8
    punpckhbw            m8, m8
    psraw                m0, 8
    psraw                m7, 8
    psraw                m1, 8
    psraw                m8, 8
    punpcklbw            m2, m15, m15
    punpckhbw           m15, m15
    punpcklbw            m3, m9, m9
    punpckhbw            m9, m9
    psraw                m2, 8
    psraw               m15, 8
    psraw                m3, 8
    psraw                m9, 8
    mova         [stk+0x10], m0
    mova         [stk+0x20], m7
    mova         [stk+0x30], m1
    mova         [stk+0x40], m8
    mova         [stk+0x50], m2
    mova         [stk+0x60], m15
    mova         [stk+0x70], m3
    mova         [stk+0x80], m9
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 9, 10 ; 0
    mova         [stk+0x90], m1
    MC_8TAP_SCALED_H 2, 3, 4, 5, 6, 1, 9, 10 ; 1
    mova         [stk+0xa0], m2
    MC_8TAP_SCALED_H 3, 4, 5, 6, 1, 2, 9, 10 ; 2
    mova         [stk+0xb0], m3
    MC_8TAP_SCALED_H 4, 5, 6, 1, 2, 3, 9, 10 ; 3
    mova         [stk+0xc0], m4
    MC_8TAP_SCALED_H 5, 6, 1, 2, 3, 4, 9, 10 ; 4
    mova         [stk+0xd0], m5
    MC_8TAP_SCALED_H 6, 1, 2, 3, 4, 5, 9, 10 ; 5
    MC_8TAP_SCALED_H 7, 1, 2, 3, 4, 5, 9, 10 ; 6
    MC_8TAP_SCALED_H 8, 1, 2, 3, 4, 5, 9, 10 ; 7
    mova                 m5, [stk+0xd0]
    mova                 m1, [stk+0x90]
    mova                 m2, [stk+0xa0]
    mova                 m3, [stk+0xb0]
    mova                 m9, [stk+0xc0]
    punpcklwd            m4, m5, m6 ; 45a
    punpckhwd            m5, m6     ; 45b
    punpcklwd            m6, m7, m8 ; 67a
    punpckhwd            m7, m8     ; 67b
    punpcklwd            m0, m1, m2 ; 01a
    punpckhwd            m1, m2     ; 01b
    punpcklwd            m2, m3, m9 ; 23a
    punpckhwd            m3, m9     ; 23b
    mova                m10, [stk+0x140]
    mova                m11, [stk+0x150]
    mova                m14, [stk+0x160]
    mova                m15, [stk+0x170]
    mova         [stk+0x90], m4
    mova         [stk+0xa0], m5
    mova         [stk+0xb0], m6
    mova         [stk+0xc0], m7
 %define hround [rsp+0x10]
 %define shift  [rsp+0x20]
 %if isput
  %define vround [rsp+0x30]
 %else
  %define vround [base+pd_m524256]
 %endif
.dy2_vloop:
    pmaddwd              m4, m0, m10
    pmaddwd              m5, m1, m10
    pmaddwd              m6, m2, m11
    pmaddwd              m7, m3, m11
    paddd                m4, m13
    paddd                m5, m13
    paddd                m4, m6
    paddd                m5, m7
    pmaddwd              m6, [stk+0x90], m14
    pmaddwd              m7, [stk+0xa0], m14
    pmaddwd              m8, [stk+0xb0], m15
    pmaddwd              m9, [stk+0xc0], m15
    paddd                m4, m6
    paddd                m5, m7
 %if isput
    pshufd               m6, m12, q1032
 %endif
    paddd                m4, m8
    paddd                m5, m9
%else
    movd                 r0, m15
    movd                 rX, m4
    pshufd              m15, m15, q0321
    pshufd               m4, m4, q0321
    movd                 r4, m15
    movd                 r5, m4
    mova                m14, [stk+0x110]
    movq                 m2, [base+subpel_filters+r0*8]
    movq                 m3, [base+subpel_filters+rX*8]
    movhps               m2, [base+subpel_filters+r4*8]
    movhps               m3, [base+subpel_filters+r5*8]
    psrld               m14, 10
    mova           [stk+16], m14
    mov                  r0, [stk+ 0]
    mov                  rX, [stk+ 4]
    mov                  r4, [stk+ 8]
    mov                  r5, [stk+12]
    mova         [stk+0x20], m0
    mova         [stk+0x30], m1
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m0, m9, m4
    pand                 m1, m9, m6
    pand                 m2, m9, m7
    pand                 m3, m9, m5
    pandn                m4, [stk+0x20]
    pandn                m6, [stk+0x30]
    pandn                m7, [stk+0x40]
    pandn                m5, [stk+0x50]
    por                  m0, m4
    por                  m1, m6
    por                  m2, m7
    por                  m3, m5
    punpcklbw            m4, m0, m0
    punpckhbw            m0, m0
    punpcklbw            m5, m1, m1
    punpckhbw            m1, m1
    psraw                m4, 8
    psraw                m0, 8
    psraw                m5, 8
    psraw                m1, 8
    punpcklbw            m6, m2, m2
    punpckhbw            m2, m2
    punpcklbw            m7, m3, m3
    punpckhbw            m3, m3
    psraw                m6, 8
    psraw                m2, 8
    psraw                m7, 8
    psraw                m3, 8
    mova        [stk+0x0a0], m4
    mova        [stk+0x0b0], m0
    mova        [stk+0x0c0], m5
    mova        [stk+0x0d0], m1
    mova        [stk+0x140], m6
    mova        [stk+0x150], m2
    mova        [stk+0x160], m7
    mova        [stk+0x170], m3
    MC_8TAP_SCALED_H   0xa0, 0x20, 0 ; 0
    MC_8TAP_SCALED_H   0xa0, 0x30    ; 1
    MC_8TAP_SCALED_H   0xa0, 0x40    ; 2
    MC_8TAP_SCALED_H   0xa0, 0x50    ; 3
    MC_8TAP_SCALED_H   0xa0, 0x60    ; 4
    MC_8TAP_SCALED_H   0xa0, 0x70    ; 5
    MC_8TAP_SCALED_H   0xa0, 0x80    ; 6
    MC_8TAP_SCALED_H   0xa0, 0x90    ; 7
    mova                 m5, [stk+0x60]
    mova                 m6, [stk+0x70]
    mova                 m7, [stk+0x80]
    mova                 m0, [stk+0x90]
    mov                  r0, r0m
    punpcklwd            m4, m5, m6      ; 45a
    punpckhwd            m5, m6          ; 45b
    punpcklwd            m6, m7, m0      ; 67a
    punpckhwd            m7, m0          ; 67b
    mova         [stk+0x60], m4
    mova         [stk+0x70], m5
    mova         [stk+0x80], m6
    mova         [stk+0x90], m7
    mova                 m1, [stk+0x20]
    mova                 m2, [stk+0x30]
    mova                 m3, [stk+0x40]
    mova                 m4, [stk+0x50]
    punpcklwd            m0, m1, m2      ; 01a
    punpckhwd            m1, m2          ; 01b
    punpcklwd            m2, m3, m4      ; 23a
    punpckhwd            m3, m4          ; 23b
    mova                 m4, [stk+0x180]
    mova                 m5, [stk+0x190]
    mova                 m6, [stk+0x1a0]
    mova                 m7, [stk+0x1b0]
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
.dy2_vloop:
    pmaddwd              m0, m4
    pmaddwd              m1, m4
    pmaddwd              m2, m5
    pmaddwd              m3, m5
    paddd                m0, m2
    paddd                m1, m3
    pmaddwd              m2, [stk+0x60], m6
    pmaddwd              m3, [stk+0x70], m6
    pmaddwd              m4, [stk+0x80], m7
    pmaddwd              m5, [stk+0x90], m7
 %if isput
    movd                 m6, [esp+0x18]
 %endif
    paddd                m0, m2
    paddd                m1, m3
    paddd                m0, vrnd_mem
    paddd                m1, vrnd_mem
    paddd                m4, m0
    paddd                m5, m1
%endif
%ifidn %1, put
    psrad                m4, m6
    psrad                m5, m6
    packssdw             m4, m5
    pxor                 m7, m7
    pmaxsw               m4, m7
    pminsw               m4, pxmaxm
    mova             [dstq], m4
    add                dstq, dsm
%else
    psrad                m4, 6
    psrad                m5, 6
    packssdw             m4, m5
    mova             [tmpq], m4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .dy2_hloop_prep
%if ARCH_X86_64
    MC_8TAP_SCALED_H 4, 8, 5, 6, 7, 9, 0, 1
    mova         [stk+0xd0], m4
    MC_8TAP_SCALED_H 8, 5, 6, 7, 9, 4, 0, 1
    mova                 m4, [stk+0xd0]
    mova                 m0, m2         ; 01a
    mova                 m1, m3         ; 01b
    mova                 m2, [stk+0x90] ; 23a
    mova                 m3, [stk+0xa0] ; 23b
    mova                 m5, [stk+0xb0] ; 45a
    mova                 m6, [stk+0xc0] ; 45b
    punpcklwd            m7, m4, m8     ; 67a
    punpckhwd            m4, m8         ; 67b
    mova         [stk+0x90], m5
    mova         [stk+0xa0], m6
    mova         [stk+0xb0], m7
    mova         [stk+0xc0], m4
%else
    mov                 r0m, r0
    mov                  r3, r3m
    MC_8TAP_SCALED_H 0xa0, 0xe0 ; 8
    MC_8TAP_SCALED_H 0xa0, 0    ; 9
    mova                 m7, [stk+0xe0]
    mova                 m2, [stk+0x60] ; 23a
    mova                 m3, [stk+0x70] ; 23b
    mova                 m4, [stk+0x80] ; 45a
    mova                 m5, [stk+0x90] ; 45b
    punpcklwd            m6, m7, m0     ; 67a
    punpckhwd            m7, m0         ; 67b
    mova                 m0, [stk+0x40] ; 01a
    mova                 m1, [stk+0x50] ; 01b
    mova         [stk+0x40], m2
    mova         [stk+0x50], m3
    mova         [stk+0x60], m4
    mova         [stk+0x70], m5
    mova                 m4, [stk+0x180]
    mova                 m5, [stk+0x190]
    mova         [stk+0x80], m6
    mova         [stk+0x90], m7
    mova                 m6, [stk+0x1a0]
    mova                 m7, [stk+0x1b0]
    mov                  r0, r0m
%endif
    jmp .dy2_vloop
INIT_XMM ssse3
.ret:
    MC_8TAP_SCALED_RET 0
%if ARCH_X86_32 && !isprep && required_stack_alignment > STACK_ALIGNMENT
 %define r0m [rstk+stack_offset+ 4]
 %define r1m [rstk+stack_offset+ 8]
 %define r2m [rstk+stack_offset+12]
 %define r3m [rstk+stack_offset+16]
%endif
%undef isput
%undef isprep
%endmacro

%macro BILIN_SCALED_FN 1
cglobal %1_bilin_scaled_16bpc
    mov                 t0d, (5*15 << 16) | 5*15
    mov                 t1d, (5*15 << 16) | 5*15
    jmp mangle(private_prefix %+ _%1_8tap_scaled_16bpc %+ SUFFIX)
%endmacro

%if WIN64
DECLARE_REG_TMP 6, 5
%elif ARCH_X86_64
DECLARE_REG_TMP 6, 8
%else
DECLARE_REG_TMP 1, 2
%endif
BILIN_SCALED_FN put
FN put_8tap_scaled, sharp,          SHARP,   SHARP
FN put_8tap_scaled, sharp_smooth,   SHARP,   SMOOTH
FN put_8tap_scaled, smooth_sharp,   SMOOTH,  SHARP
FN put_8tap_scaled, smooth,         SMOOTH,  SMOOTH
FN put_8tap_scaled, sharp_regular,  SHARP,   REGULAR
FN put_8tap_scaled, regular_sharp,  REGULAR, SHARP
FN put_8tap_scaled, smooth_regular, SMOOTH,  REGULAR
FN put_8tap_scaled, regular_smooth, REGULAR, SMOOTH
FN put_8tap_scaled, regular,        REGULAR, REGULAR
MC_8TAP_SCALED put

%if WIN64
DECLARE_REG_TMP 5, 4
%elif ARCH_X86_64
DECLARE_REG_TMP 6, 7
%else
DECLARE_REG_TMP 1, 2
%endif
BILIN_SCALED_FN prep
FN prep_8tap_scaled, sharp,          SHARP,   SHARP
FN prep_8tap_scaled, sharp_smooth,   SHARP,   SMOOTH
FN prep_8tap_scaled, smooth_sharp,   SMOOTH,  SHARP
FN prep_8tap_scaled, smooth,         SMOOTH,  SMOOTH
FN prep_8tap_scaled, sharp_regular,  SHARP,   REGULAR
FN prep_8tap_scaled, regular_sharp,  REGULAR, SHARP
FN prep_8tap_scaled, smooth_regular, SMOOTH,  REGULAR
FN prep_8tap_scaled, regular_smooth, REGULAR, SMOOTH
FN prep_8tap_scaled, regular,        REGULAR, REGULAR
MC_8TAP_SCALED prep

%if ARCH_X86_64
DECLARE_REG_TMP 6
%else
DECLARE_REG_TMP 2
%endif

%if ARCH_X86_64
; warp8x8t spills one less xmm register than warp8x8 on WIN64, compensate that
; by allocating 16 bytes more stack space so that stack offsets match up.
%if WIN64 && STACK_ALIGNMENT == 16
%assign stksz 16*14
%else
%assign stksz 16*13
%endif
cglobal warp_affine_8x8t_16bpc, 4, 13, 9, stksz, dst, ds, src, ss, delta, \
                                                 mx, tmp, alpha, beta, \
                                                 filter, my, gamma, cnt
%assign stack_size_padded_8x8t stack_size_padded
%else
cglobal warp_affine_8x8t_16bpc, 0, 7, 8, -16*17, alpha, gamma, src, tmp, \
                                                 filter, mx, my
%define m8   [esp+16*13]
%define m9   [esp+16*14]
%define cntd dword [esp+4*63]
%define dstq tmpq
%define dsq  0
%if STACK_ALIGNMENT < 16
%define dstm [esp+4*65]
%define dsm  [esp+4*66]
%else
%define dstm r0m
%define dsm  r1m
%endif
%endif
%define base filterq-$$
    mov                 t0d, r7m
    LEA             filterq, $$
    shr                 t0d, 11
%if ARCH_X86_64
    movddup              m8, [base+warp8x8t_rnd]
%else
    movddup              m1, [base+warp8x8t_rnd]
    mov                  r1, r1m
    add                  r1, r1
    mova                 m8, m1
    mov                 r1m, r1 ; ds *= 2
%endif
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_ssse3).main
    jmp .start
.loop:
%if ARCH_X86_64
    lea                dstq, [dstq+dsq*4]
%else
    add                dstq, dsm
    mov                dstm, dstq
%endif
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_ssse3).main2
.start:
%if ARCH_X86_32
    mov                dstq, dstm
%endif
    paddd                m1, m8
    paddd                m2, m8
    psrad                m1, 15
    psrad                m2, 15
    packssdw             m1, m2
    mova       [dstq+dsq*0], m1
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_ssse3).main3
%if ARCH_X86_32
    mov                dstq, dstm
    add                dstq, dsm
%endif
    paddd                m1, m8
    paddd                m2, m8
    psrad                m1, 15
    psrad                m2, 15
    packssdw             m1, m2
    mova       [dstq+dsq*2], m1
    dec                cntd
    jg .loop
    RET

%if ARCH_X86_64
cglobal warp_affine_8x8_16bpc, 4, 13, 10, 16*13, dst, ds, src, ss, delta, \
                                                 mx, tmp, alpha, beta, \
                                                 filter, my, gamma, cnt
ASSERT stack_size_padded == stack_size_padded_8x8t
%else
cglobal warp_affine_8x8_16bpc, 0, 7, 8, -16*17, alpha, gamma, src, tmp, \
                                                filter, mx, my
%endif
    mov                 t0d, r7m
    LEA             filterq, $$
    shr                 t0d, 11
%if ARCH_X86_64
    movddup              m8, [base+warp8x8_rnd2+t0*8]
    movd                 m9, r7m ; pixel_max
    pshufb               m9, [base+pw_256]
%else
    movddup              m1, [base+warp8x8_rnd2+t0*8]
    movd                 m2, r7m ; pixel_max
    pshufb               m2, [base+pw_256]
    mova                 m8, m1
    mova                 m9, m2
%endif
    call .main
    jmp .start
.loop:
%if ARCH_X86_64
    lea                dstq, [dstq+dsq*2]
%else
    add                dstq, dsm
    mov                dstm, dstq
%endif
    call .main2
.start:
%if ARCH_X86_32
    mov                dstq, dstm
%endif
    psrad                m1, 16
    psrad                m2, 16
    packssdw             m1, m2
    pmaxsw               m1, m6
    pmulhrsw             m1, m8
    pminsw               m1, m9
    mova       [dstq+dsq*0], m1
    call .main3
%if ARCH_X86_32
    mov                dstq, dstm
    add                dstq, dsm
%endif
    psrad                m1, 16
    psrad                m2, 16
    packssdw             m1, m2
    pmaxsw               m1, m6
    pmulhrsw             m1, m8
    pminsw               m1, m9
    mova       [dstq+dsq*1], m1
    dec                cntd
    jg .loop
    RET
ALIGN function_align
.main:
    ; Stack args offset by one (r4m -> r5m etc.) due to call
%if WIN64
    mov              deltaq, r5m
    mov                 mxd, r6m
%endif
    movd                 m0, [base+warp8x8_shift+t0*4]
    movddup              m7, [base+warp8x8_rnd1+t0*8]
    add             filterq, mc_warp_filter-$$
%if ARCH_X86_64
    movsx            alphad, word [deltaq+2*0]
    movsx             betad, word [deltaq+2*1]
    movsx            gammad, word [deltaq+2*2]
    movsx            deltad, word [deltaq+2*3]
    lea                tmpq, [ssq*3]
    add                 mxd, 512+(64<<10)
    sub                srcq, tmpq             ; src -= ss*3
    imul               tmpd, alphad, -7
    mov                 myd, r7m
    add               betad, tmpd             ; beta -= alpha*7
    imul               tmpd, gammad, -7
    add                 myd, 512+(64<<10)
    mov                cntd, 4
    add              deltad, tmpd             ; delta -= gamma*7
%else
%if STACK_ALIGNMENT < 16
    %assign stack_offset stack_offset - gprsize
%endif
    mov                 r3d, r5m              ; abcd
%if STACK_ALIGNMENT < 16
    mov                  r0, r1m              ; dst
    mov                  r1, r2m              ; ds
    mov  [esp+gprsize+4*65], r0
    mov  [esp+gprsize+4*66], r1
%endif
    movsx            alphad, word [r3+2*0]
    movsx               r2d, word [r3+2*1]
    movsx            gammad, word [r3+2*2]
    movsx               r3d, word [r3+2*3]
    imul                r5d, alphad, -7
    add                 r2d, r5d              ; beta -= alpha*7
    imul                r5d, gammad, -7
    mov  [esp+gprsize+4*60], r2d
    add                 r3d, r5d              ; delta -= gamma*7
    mov  [esp+gprsize+4*61], r3d
    mov                 r3d, r4m              ; ss
    mov                srcq, r3m
    mov                 mxd, r6m
    mov                 myd, r7m
    mov dword [esp+gprsize+4*63], 4           ; cnt
    mov  [esp+gprsize+4*62], r3
    lea                  r3, [r3*3]
    add                 mxd, 512+(64<<10)
    add                 myd, 512+(64<<10)
    sub                srcq, r3               ; src -= ss*3
%if STACK_ALIGNMENT < 16
    %assign stack_offset stack_offset + gprsize
%endif
%endif
    mova      [rsp+gprsize], m0
    pxor                 m6, m6
    call .h
    mova                 m5, m0
    call .h
    punpcklwd            m1, m5, m0           ; 01
    punpckhwd            m5, m0
    mova [rsp+gprsize+16* 1], m1
    mova [rsp+gprsize+16* 4], m5
    mova                 m5, m0
    call .h
    punpcklwd            m1, m5, m0           ; 12
    punpckhwd            m5, m0
    mova [rsp+gprsize+16* 7], m1
    mova [rsp+gprsize+16*10], m5
    mova                 m5, m0
    call .h
    punpcklwd            m1, m5, m0           ; 23
    punpckhwd            m5, m0
    mova [rsp+gprsize+16* 2], m1
    mova [rsp+gprsize+16* 5], m5
    mova                 m5, m0
    call .h
    punpcklwd            m1, m5, m0           ; 34
    punpckhwd            m5, m0
    mova [rsp+gprsize+16* 8], m1
    mova [rsp+gprsize+16*11], m5
    mova                 m5, m0
    call .h
    punpcklwd            m1, m5, m0           ; 45
    punpckhwd            m5, m0
    mova [rsp+gprsize+16* 3], m1
    mova [rsp+gprsize+16* 6], m5
    mova                 m5, m0
    call .h
    punpcklwd            m1, m5, m0           ; 56
    punpckhwd            m5, m0
    mova [rsp+gprsize+16* 9], m1
    mova [rsp+gprsize+16*12], m5
    mova                 m5, m0
.main2:
    call .h
%macro WARP_V 6 ; 01l, 23l, 45l, 01h, 23h, 45h
    lea                tmpd, [myq+gammaq]
    shr                 myd, 10
    movq                 m4, [filterq+myq*8]  ; a
    lea                 myd, [tmpq+gammaq]
    shr                tmpd, 10
    movq                 m2, [filterq+tmpq*8] ; b
    lea                tmpd, [myq+gammaq]
    shr                 myd, 10
    movq                 m3, [filterq+myq*8]  ; c
    lea                 myd, [tmpq+gammaq]
    shr                tmpd, 10
    movq                 m1, [filterq+tmpq*8] ; d
    lea                tmpd, [myq+gammaq]
    shr                 myd, 10
    punpcklwd            m4, m2
    punpcklwd            m3, m1
    punpckldq            m2, m4, m3
    punpckhdq            m4, m3
    punpcklbw            m1, m6, m2           ; a0 a1 b0 b1 c0 c1 d0 d1 << 8
    pmaddwd              m1, [rsp+gprsize+16*%1]
    punpckhbw            m3, m6, m2           ; a2 a3 b2 b3 c2 c3 d2 d3 << 8
    mova                 m2, [rsp+gprsize+16*%2]
    pmaddwd              m3, m2
    mova [rsp+gprsize+16*%1], m2
    paddd                m1, m3
    punpcklbw            m3, m6, m4           ; a4 a5 b4 b5 c4 c5 d4 d5 << 8
    mova                 m2, [rsp+gprsize+16*%3]
    pmaddwd              m3, m2
    mova [rsp+gprsize+16*%2], m2
    paddd                m1, m3
    punpcklwd            m3, m5, m0           ; 67
    punpckhbw            m2, m6, m4           ; a6 a7 b6 b7 c6 c7 d6 d7 << 8
    pmaddwd              m2, m3
    mova [rsp+gprsize+16*%3], m3
    paddd                m1, m2
    movq                 m4, [filterq+myq*8]  ; e
    lea                 myd, [tmpq+gammaq]
    shr                tmpd, 10
    movq                 m3, [filterq+tmpq*8] ; f
    lea                tmpd, [myq+gammaq]
    shr                 myd, 10
    movq                 m2, [filterq+myq*8]  ; g
%if ARCH_X86_64
    lea                 myd, [tmpq+deltaq]    ; my += delta
%else
    mov                 myd, [esp+gprsize+4*61]
    add                 myd, tmpd
%endif
    shr                tmpd, 10
    punpcklwd            m4, m3
    movq                 m3, [filterq+tmpq*8] ; h
    punpcklwd            m2, m3
    punpckldq            m3, m4, m2
    punpckhdq            m4, m2
    punpcklbw            m2, m6, m3           ; e0 e1 f0 f1 g0 g1 h0 h1 << 8
    pmaddwd              m2, [rsp+gprsize+16*%4]
    punpckhbw            m6, m3               ; e2 e3 f2 f3 g2 g3 h2 h3 << 8
    mova                 m3, [rsp+gprsize+16*%5]
    pmaddwd              m6, m3
    mova [rsp+gprsize+16*%4], m3
    pxor                 m3, m3
    paddd                m2, m6
    punpcklbw            m3, m4               ; e4 e5 f4 f5 g4 g5 h4 h5 << 8
    mova                 m6, [rsp+gprsize+16*%6]
    pmaddwd              m3, m6
    mova [rsp+gprsize+16*%5], m6
    punpckhwd            m5, m0
    pxor                 m6, m6
    paddd                m2, m3
    punpckhbw            m3, m6, m4           ; e6 e7 f6 f7 g6 g7 h6 h7 << 8
    pmaddwd              m3, m5
    mova [rsp+gprsize+16*%6], m5
    mova                 m5, m0
    paddd                m2, m3
%endmacro
    WARP_V                1,  2,  3,  4,  5,  6
    ret
.main3:
    call .h
    WARP_V                7,  8,  9, 10, 11, 12
    ret
ALIGN function_align
.h:
    lea                tmpd, [mxq+alphaq]
    shr                 mxd, 10
    movq                 m3, [filterq+mxq*8]
    punpcklbw            m0, m6, m3
    movu                 m3, [srcq-6]
    pmaddwd              m0, m3               ; 0
    lea                 mxd, [tmpq+alphaq]
    shr                tmpd, 10
    movq                 m3, [filterq+tmpq*8]
    punpcklbw            m2, m6, m3
    movu                 m3, [srcq-4]
    pmaddwd              m2, m3               ; 1
    lea                tmpd, [mxq+alphaq]
    shr                 mxd, 10
    movq                 m3, [filterq+mxq*8]
    phaddd               m0, m2               ; 0 1
    punpcklbw            m2, m6, m3
    movu                 m3, [srcq-2]
    pmaddwd              m2, m3               ; 2
    lea                 mxd, [tmpq+alphaq]
    shr                tmpd, 10
    movq                 m3, [filterq+tmpq*8]
    punpcklbw            m1, m6, m3
    movu                 m3, [srcq+0]
    pmaddwd              m1, m3               ; 3
    lea                tmpd, [mxq+alphaq]
    shr                 mxd, 10
    movq                 m3, [filterq+mxq*8]
    phaddd               m2, m1               ; 2 3
    punpcklbw            m1, m6, m3
    movu                 m3, [srcq+2]
    pmaddwd              m1, m3               ; 4
    lea                 mxd, [tmpq+alphaq]
    shr                tmpd, 10
    movq                 m3, [filterq+tmpq*8]
    phaddd               m0, m2               ; 0 1 2 3
    punpcklbw            m2, m6, m3
    movu                 m3, [srcq+4]
    pmaddwd              m2, m3               ; 5
    lea                tmpd, [mxq+alphaq]
    shr                 mxd, 10
    movq                 m3, [filterq+mxq*8]
    phaddd               m1, m2               ; 4 5
    punpcklbw            m2, m6, m3
    movu                 m3, [srcq+6]
    pmaddwd              m2, m3               ; 6
%if ARCH_X86_64
    lea                 mxd, [tmpq+betaq]     ; mx += beta
%else
    mov                 mxd, [esp+gprsize*2+4*60]
    add                 mxd, tmpd
%endif
    shr                tmpd, 10
    movq                 m3, [filterq+tmpq*8]
    punpcklbw            m4, m6, m3
    movu                 m3, [srcq+8]
%if ARCH_X86_64
    add                srcq, ssq
%else
    add                srcq, [esp+gprsize*2+4*62]
%endif
    pmaddwd              m3, m4               ; 7
    phaddd               m2, m3               ; 6 7
    phaddd               m1, m2               ; 4 5 6 7
    paddd                m0, m7
    paddd                m1, m7
    psrad                m0, [rsp+gprsize*2]
    psrad                m1, [rsp+gprsize*2]
    packssdw             m0, m1
    ret

%macro BIDIR_FN 0
    call .main
    jmp                  wq
.w4_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w4:
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    movq   [dstq+strideq*0], m1
    movhps [dstq+strideq*1], m1
    sub                  hd, 4
    jg .w4_loop
.ret:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w8:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    sub                  hd, 2
    jne .w8_loop
    RET
.w16_loop:
    call .main
    add                dstq, strideq
.w16:
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    dec                  hd
    jg .w16_loop
    RET
.w32_loop:
    call .main
    add                dstq, strideq
.w32:
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    call .main
    mova        [dstq+16*2], m0
    mova        [dstq+16*3], m1
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    call .main
    mova        [dstq+16*2], m0
    mova        [dstq+16*3], m1
    call .main
    mova        [dstq+16*4], m0
    mova        [dstq+16*5], m1
    call .main
    mova        [dstq+16*6], m0
    mova        [dstq+16*7], m1
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova       [dstq+16* 0], m0
    mova       [dstq+16* 1], m1
    call .main
    mova       [dstq+16* 2], m0
    mova       [dstq+16* 3], m1
    call .main
    mova       [dstq+16* 4], m0
    mova       [dstq+16* 5], m1
    call .main
    mova       [dstq+16* 6], m0
    mova       [dstq+16* 7], m1
    call .main
    mova       [dstq+16* 8], m0
    mova       [dstq+16* 9], m1
    call .main
    mova       [dstq+16*10], m0
    mova       [dstq+16*11], m1
    call .main
    mova       [dstq+16*12], m0
    mova       [dstq+16*13], m1
    call .main
    mova       [dstq+16*14], m0
    mova       [dstq+16*15], m1
    dec                  hd
    jg .w128_loop
    RET
%endmacro

%if UNIX64
DECLARE_REG_TMP 7
%else
DECLARE_REG_TMP 5
%endif

cglobal avg_16bpc, 4, 7, 4, dst, stride, tmp1, tmp2, w, h
%define base r6-avg_ssse3_table
    LEA                  r6, avg_ssse3_table
    tzcnt                wd, wm
    mov                 t0d, r6m ; pixel_max
    movsxd               wq, [r6+wq*4]
    shr                 t0d, 11
    movddup              m2, [base+bidir_rnd+t0*8]
    movddup              m3, [base+bidir_mul+t0*8]
    movifnidn            hd, hm
    add                  wq, r6
    BIDIR_FN
ALIGN function_align
.main:
    mova                 m0, [tmp1q+16*0]
    paddsw               m0, [tmp2q+16*0]
    mova                 m1, [tmp1q+16*1]
    paddsw               m1, [tmp2q+16*1]
    add               tmp1q, 16*2
    add               tmp2q, 16*2
    pmaxsw               m0, m2
    pmaxsw               m1, m2
    psubsw               m0, m2
    psubsw               m1, m2
    pmulhw               m0, m3
    pmulhw               m1, m3
    ret

cglobal w_avg_16bpc, 4, 7, 8, dst, stride, tmp1, tmp2, w, h
%define base r6-w_avg_ssse3_table
    LEA                  r6, w_avg_ssse3_table
    tzcnt                wd, wm
    mov                 t0d, r6m ; weight
    movd                 m6, r7m ; pixel_max
    movddup              m5, [base+pd_65538]
    movsxd               wq, [r6+wq*4]
    pshufb               m6, [base+pw_256]
    add                  wq, r6
    lea                 r6d, [t0-16]
    shl                 t0d, 16
    sub                 t0d, r6d ; 16-weight, weight
    paddw                m5, m6
    mov                 r6d, t0d
    shl                 t0d, 2
    test          dword r7m, 0x800
    cmovnz              r6d, t0d
    movifnidn            hd, hm
    movd                 m4, r6d
    pslld                m5, 7
    pxor                 m7, m7
    pshufd               m4, m4, q0000
    BIDIR_FN
ALIGN function_align
.main:
    mova                 m2, [tmp1q+16*0]
    mova                 m0, [tmp2q+16*0]
    punpckhwd            m3, m0, m2
    punpcklwd            m0, m2
    mova                 m2, [tmp1q+16*1]
    mova                 m1, [tmp2q+16*1]
    add               tmp1q, 16*2
    add               tmp2q, 16*2
    pmaddwd              m3, m4
    pmaddwd              m0, m4
    paddd                m3, m5
    paddd                m0, m5
    psrad                m3, 8
    psrad                m0, 8
    packssdw             m0, m3
    punpckhwd            m3, m1, m2
    punpcklwd            m1, m2
    pmaddwd              m3, m4
    pmaddwd              m1, m4
    paddd                m3, m5
    paddd                m1, m5
    psrad                m3, 8
    psrad                m1, 8
    packssdw             m1, m3
    pminsw               m0, m6
    pminsw               m1, m6
    pmaxsw               m0, m7
    pmaxsw               m1, m7
    ret

%if ARCH_X86_64
cglobal mask_16bpc, 4, 7, 9, dst, stride, tmp1, tmp2, w, h, mask
%else
cglobal mask_16bpc, 4, 7, 8, dst, stride, tmp1, tmp2, w, mask
%define hd dword r5m
%define m8 [base+pw_64]
%endif
%define base r6-mask_ssse3_table
    LEA                  r6, mask_ssse3_table
    tzcnt                wd, wm
    mov                 t0d, r7m ; pixel_max
    shr                 t0d, 11
    movsxd               wq, [r6+wq*4]
    movddup              m6, [base+bidir_rnd+t0*8]
    movddup              m7, [base+bidir_mul+t0*8]
%if ARCH_X86_64
    mova                 m8, [base+pw_64]
    movifnidn            hd, hm
%endif
    add                  wq, r6
    mov               maskq, r6mp
    BIDIR_FN
ALIGN function_align
.main:
    movq                 m3, [maskq+8*0]
    mova                 m0, [tmp1q+16*0]
    mova                 m4, [tmp2q+16*0]
    pxor                 m5, m5
    punpcklbw            m3, m5
    punpckhwd            m2, m0, m4
    punpcklwd            m0, m4
    psubw                m1, m8, m3
    punpckhwd            m4, m3, m1 ; m, 64-m
    punpcklwd            m3, m1
    pmaddwd              m2, m4     ; tmp1 * m + tmp2 * (64-m)
    pmaddwd              m0, m3
    movq                 m3, [maskq+8*1]
    mova                 m1, [tmp1q+16*1]
    mova                 m4, [tmp2q+16*1]
    add               maskq, 8*2
    add               tmp1q, 16*2
    add               tmp2q, 16*2
    psrad                m2, 5
    psrad                m0, 5
    packssdw             m0, m2
    punpcklbw            m3, m5
    punpckhwd            m2, m1, m4
    punpcklwd            m1, m4
    psubw                m5, m8, m3
    punpckhwd            m4, m3, m5 ; m, 64-m
    punpcklwd            m3, m5
    pmaddwd              m2, m4     ; tmp1 * m + tmp2 * (64-m)
    pmaddwd              m1, m3
    psrad                m2, 5
    psrad                m1, 5
    packssdw             m1, m2
    pmaxsw               m0, m6
    pmaxsw               m1, m6
    psubsw               m0, m6
    psubsw               m1, m6
    pmulhw               m0, m7
    pmulhw               m1, m7
    ret

cglobal w_mask_420_16bpc, 4, 7, 12, dst, stride, tmp1, tmp2, w, h, mask
%define base t0-w_mask_420_ssse3_table
    LEA                  t0, w_mask_420_ssse3_table
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    movd                 m0, r7m ; sign
    shr                 r6d, 11
    movsxd               wq, [t0+wq*4]
%if ARCH_X86_64
    mova                 m8, [base+pw_27615] ; ((64 - 38) << 10) + 1023 - 32
    mova                 m9, [base+pw_64]
    movddup             m10, [base+bidir_rnd+r6*8]
    movddup             m11, [base+bidir_mul+r6*8]
%else
    mova                 m1, [base+pw_27615] ; ((64 - 38) << 10) + 1023 - 32
    mova                 m2, [base+pw_64]
    movddup              m3, [base+bidir_rnd+r6*8]
    movddup              m4, [base+bidir_mul+r6*8]
    ALLOC_STACK       -16*4
    mova         [rsp+16*0], m1
    mova         [rsp+16*1], m2
    mova         [rsp+16*2], m3
    mova         [rsp+16*3], m4
    %define              m8  [rsp+gprsize+16*0]
    %define              m9  [rsp+gprsize+16*1]
    %define             m10  [rsp+gprsize+16*2]
    %define             m11  [rsp+gprsize+16*3]
%endif
    movd                 m7, [base+pw_2]
    psubw                m7, m0
    pshufb               m7, [base+pw_256]
    add                  wq, t0
    movifnidn            hd, r5m
    mov               maskq, r6mp
    call .main
    jmp                  wq
.w4_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 4
.w4:
    movq   [dstq+strideq*0], m0
    phaddw               m2, m3
    movhps [dstq+strideq*1], m0
    phaddd               m2, m2
    lea                dstq, [dstq+strideq*2]
    paddw                m2, m7
    movq   [dstq+strideq*0], m1
    psrlw                m2, 2
    movhps [dstq+strideq*1], m1
    packuswb             m2, m2
    movd            [maskq], m2
    sub                  hd, 4
    jg .w4_loop
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 4
.w8:
    mova   [dstq+strideq*0], m0
    paddw                m2, m3
    phaddw               m2, m2
    mova   [dstq+strideq*1], m1
    paddw                m2, m7
    psrlw                m2, 2
    packuswb             m2, m2
    movd            [maskq], m2
    sub                  hd, 2
    jg .w8_loop
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 8
.w16:
    mova [dstq+strideq*1+16*0], m2
    mova [dstq+strideq*0+16*0], m0
    mova [dstq+strideq*1+16*1], m3
    mova [dstq+strideq*0+16*1], m1
    call .main
    paddw                m2, [dstq+strideq*1+16*0]
    paddw                m3, [dstq+strideq*1+16*1]
    mova [dstq+strideq*1+16*0], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*1], m1
    paddw                m2, m7
    psrlw                m2, 2
    packuswb             m2, m2
    movq            [maskq], m2
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 16
.w32:
    mova [dstq+strideq*1+16*0], m2
    mova [dstq+strideq*0+16*0], m0
    mova [dstq+strideq*1+16*1], m3
    mova [dstq+strideq*0+16*1], m1
    call .main
    mova [dstq+strideq*0+16*2], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*3], m2
    mova [dstq+strideq*0+16*3], m1
    call .main
    paddw                m2, [dstq+strideq*1+16*0]
    paddw                m3, [dstq+strideq*1+16*1]
    mova [dstq+strideq*1+16*0], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*2], m2
    mova [dstq+strideq*1+16*1], m1
    call .main
    phaddw               m2, m3
    paddw                m3, m7, [dstq+strideq*1+16*2]
    paddw                m2, [dstq+strideq*1+16*3]
    mova [dstq+strideq*1+16*2], m0
    paddw                m2, m7
    psrlw                m3, 2
    psrlw                m2, 2
    mova [dstq+strideq*1+16*3], m1
    packuswb             m3, m2
    mova            [maskq], m3
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 16*2
.w64:
    mova [dstq+strideq*1+16*1], m2
    mova [dstq+strideq*0+16*0], m0
    mova [dstq+strideq*1+16*2], m3
    mova [dstq+strideq*0+16*1], m1
    call .main
    mova [dstq+strideq*1+16*3], m2
    mova [dstq+strideq*0+16*2], m0
    mova [dstq+strideq*1+16*4], m3
    mova [dstq+strideq*0+16*3], m1
    call .main
    mova [dstq+strideq*1+16*5], m2
    mova [dstq+strideq*0+16*4], m0
    mova [dstq+strideq*1+16*6], m3
    mova [dstq+strideq*0+16*5], m1
    call .main
    mova [dstq+strideq*0+16*6], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*7], m2
    mova [dstq+strideq*0+16*7], m1
    call .main
    paddw                m2, [dstq+strideq*1+16*1]
    paddw                m3, [dstq+strideq*1+16*2]
    mova [dstq+strideq*1+16*0], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*2], m2
    mova [dstq+strideq*1+16*1], m1
    call .main
    paddw                m2, [dstq+strideq*1+16*3]
    paddw                m3, [dstq+strideq*1+16*4]
    phaddw               m2, m3
    paddw                m3, m7, [dstq+strideq*1+16*2]
    mova [dstq+strideq*1+16*2], m0
    paddw                m2, m7
    psrlw                m3, 2
    psrlw                m2, 2
    mova [dstq+strideq*1+16*3], m1
    packuswb             m3, m2
    mova       [maskq+16*0], m3
    call .main
    paddw                m2, [dstq+strideq*1+16*5]
    paddw                m3, [dstq+strideq*1+16*6]
    mova [dstq+strideq*1+16*4], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*6], m2
    mova [dstq+strideq*1+16*5], m1
    call .main
    phaddw               m2, m3
    paddw                m3, m7, [dstq+strideq*1+16*6]
    paddw                m2, [dstq+strideq*1+16*7]
    mova [dstq+strideq*1+16*6], m0
    paddw                m2, m7
    psrlw                m3, 2
    psrlw                m2, 2
    mova [dstq+strideq*1+16*7], m1
    packuswb             m3, m2
    mova       [maskq+16*1], m3
    sub                  hd, 2
    jg .w64_loop
    RET
.w128_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 16*4
.w128:
    mova [dstq+strideq*1+16* 1], m2
    mova [dstq+strideq*0+16* 0], m0
    mova [dstq+strideq*1+16* 2], m3
    mova [dstq+strideq*0+16* 1], m1
    call .main
    mova [dstq+strideq*1+16* 3], m2
    mova [dstq+strideq*0+16* 2], m0
    mova [dstq+strideq*1+16* 4], m3
    mova [dstq+strideq*0+16* 3], m1
    call .main
    mova [dstq+strideq*1+16* 5], m2
    mova [dstq+strideq*0+16* 4], m0
    mova [dstq+strideq*1+16* 6], m3
    mova [dstq+strideq*0+16* 5], m1
    call .main
    mova [dstq+strideq*1+16* 7], m2
    mova [dstq+strideq*0+16* 6], m0
    mova [dstq+strideq*1+16* 8], m3
    mova [dstq+strideq*0+16* 7], m1
    call .main
    mova [dstq+strideq*1+16* 9], m2
    mova [dstq+strideq*0+16* 8], m0
    mova [dstq+strideq*1+16*10], m3
    mova [dstq+strideq*0+16* 9], m1
    call .main
    mova [dstq+strideq*1+16*11], m2
    mova [dstq+strideq*0+16*10], m0
    mova [dstq+strideq*1+16*12], m3
    mova [dstq+strideq*0+16*11], m1
    call .main
    mova [dstq+strideq*1+16*13], m2
    mova [dstq+strideq*0+16*12], m0
    mova [dstq+strideq*1+16*14], m3
    mova [dstq+strideq*0+16*13], m1
    call .main
    mova [dstq+strideq*0+16*14], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*15], m2
    mova [dstq+strideq*0+16*15], m1
    call .main
    paddw                m2, [dstq+strideq*1+16* 1]
    paddw                m3, [dstq+strideq*1+16* 2]
    mova [dstq+strideq*1+16* 0], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16* 2], m2
    mova [dstq+strideq*1+16* 1], m1
    call .main
    paddw                m2, [dstq+strideq*1+16* 3]
    paddw                m3, [dstq+strideq*1+16* 4]
    phaddw               m2, m3
    paddw                m3, m7, [dstq+strideq*1+16* 2]
    mova [dstq+strideq*1+16* 2], m0
    paddw                m2, m7
    psrlw                m3, 2
    psrlw                m2, 2
    mova [dstq+strideq*1+16* 3], m1
    packuswb             m3, m2
    mova       [maskq+16*0], m3
    call .main
    paddw                m2, [dstq+strideq*1+16* 5]
    paddw                m3, [dstq+strideq*1+16* 6]
    mova [dstq+strideq*1+16* 4], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16* 6], m2
    mova [dstq+strideq*1+16* 5], m1
    call .main
    paddw                m2, [dstq+strideq*1+16* 7]
    paddw                m3, [dstq+strideq*1+16* 8]
    phaddw               m2, m3
    paddw                m3, m7, [dstq+strideq*1+16* 6]
    mova [dstq+strideq*1+16* 6], m0
    paddw                m2, m7
    psrlw                m3, 2
    psrlw                m2, 2
    mova [dstq+strideq*1+16* 7], m1
    packuswb             m3, m2
    mova       [maskq+16*1], m3
    call .main
    paddw                m2, [dstq+strideq*1+16* 9]
    paddw                m3, [dstq+strideq*1+16*10]
    mova [dstq+strideq*1+16* 8], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*10], m2
    mova [dstq+strideq*1+16* 9], m1
    call .main
    paddw                m2, [dstq+strideq*1+16*11]
    paddw                m3, [dstq+strideq*1+16*12]
    phaddw               m2, m3
    paddw                m3, m7, [dstq+strideq*1+16*10]
    mova [dstq+strideq*1+16*10], m0
    paddw                m2, m7
    psrlw                m3, 2
    psrlw                m2, 2
    mova [dstq+strideq*1+16*11], m1
    packuswb             m3, m2
    mova       [maskq+16*2], m3
    call .main
    paddw                m2, [dstq+strideq*1+16*13]
    paddw                m3, [dstq+strideq*1+16*14]
    mova [dstq+strideq*1+16*12], m0
    phaddw               m2, m3
    mova [dstq+strideq*1+16*14], m2
    mova [dstq+strideq*1+16*13], m1
    call .main
    phaddw               m2, m3
    paddw                m3, m7, [dstq+strideq*1+16*14]
    paddw                m2, [dstq+strideq*1+16*15]
    mova [dstq+strideq*1+16*14], m0
    paddw                m2, m7
    psrlw                m3, 2
    psrlw                m2, 2
    mova [dstq+strideq*1+16*15], m1
    packuswb             m3, m2
    mova       [maskq+16*3], m3
    sub                  hd, 2
    jg .w128_loop
    RET
ALIGN function_align
.main:
%macro W_MASK 2 ; dst/tmp_offset, mask
    mova                m%1, [tmp1q+16*%1]
    mova                m%2, [tmp2q+16*%1]
    punpcklwd            m4, m%2, m%1
    punpckhwd            m5, m%2, m%1
    psubsw              m%1, m%2
    pabsw               m%1, m%1
    psubusw              m6, m8, m%1
    psrlw                m6, 10      ; 64-m
    psubw               m%2, m9, m6  ; m
    punpcklwd           m%1, m6, m%2
    punpckhwd            m6, m%2
    pmaddwd             m%1, m4
    pmaddwd              m6, m5
    psrad               m%1, 5
    psrad                m6, 5
    packssdw            m%1, m6
    pmaxsw              m%1, m10
    psubsw              m%1, m10
    pmulhw              m%1, m11
%endmacro
    W_MASK                0, 2
    W_MASK                1, 3
    add               tmp1q, 16*2
    add               tmp2q, 16*2
    ret

cglobal w_mask_422_16bpc, 4, 7, 12, dst, stride, tmp1, tmp2, w, h, mask
%define base t0-w_mask_422_ssse3_table
    LEA                  t0, w_mask_422_ssse3_table
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    movd                 m7, r7m ; sign
    shr                 r6d, 11
    movsxd               wq, [t0+wq*4]
%if ARCH_X86_64
    mova                 m8, [base+pw_27615]
    mova                 m9, [base+pw_64]
    movddup             m10, [base+bidir_rnd+r6*8]
    movddup             m11, [base+bidir_mul+r6*8]
%else
    mova                 m1, [base+pw_27615]
    mova                 m2, [base+pw_64]
    movddup              m3, [base+bidir_rnd+r6*8]
    movddup              m4, [base+bidir_mul+r6*8]
    ALLOC_STACK       -16*4
    mova         [rsp+16*0], m1
    mova         [rsp+16*1], m2
    mova         [rsp+16*2], m3
    mova         [rsp+16*3], m4
%endif
    pxor                 m0, m0
    add                  wq, t0
    pshufb               m7, m0
    movifnidn            hd, r5m
    mov               maskq, r6mp
    call .main
    jmp                  wq
.w4_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w4:
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    movq   [dstq+strideq*0], m1
    movhps [dstq+strideq*1], m1
    sub                  hd, 4
    jg .w4_loop
.end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w8:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    sub                  hd, 2
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w16:
    mova [dstq+strideq*0+16*0], m0
    mova [dstq+strideq*0+16*1], m1
    call .main
    mova [dstq+strideq*1+16*0], m0
    mova [dstq+strideq*1+16*1], m1
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    call .main
    add                dstq, strideq
.w32:
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    call .main
    mova        [dstq+16*2], m0
    mova        [dstq+16*3], m1
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    call .main
    mova        [dstq+16*2], m0
    mova        [dstq+16*3], m1
    call .main
    mova        [dstq+16*4], m0
    mova        [dstq+16*5], m1
    call .main
    mova        [dstq+16*6], m0
    mova        [dstq+16*7], m1
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova       [dstq+16* 0], m0
    mova       [dstq+16* 1], m1
    call .main
    mova       [dstq+16* 2], m0
    mova       [dstq+16* 3], m1
    call .main
    mova       [dstq+16* 4], m0
    mova       [dstq+16* 5], m1
    call .main
    mova       [dstq+16* 6], m0
    mova       [dstq+16* 7], m1
    call .main
    mova       [dstq+16* 8], m0
    mova       [dstq+16* 9], m1
    call .main
    mova       [dstq+16*10], m0
    mova       [dstq+16*11], m1
    call .main
    mova       [dstq+16*12], m0
    mova       [dstq+16*13], m1
    call .main
    mova       [dstq+16*14], m0
    mova       [dstq+16*15], m1
    dec                  hd
    jg .w128_loop
    RET
ALIGN function_align
.main:
    W_MASK                0, 2
    W_MASK                1, 3
    phaddw               m2, m3
    add               tmp1q, 16*2
    add               tmp2q, 16*2
    packuswb             m2, m2
    pxor                 m3, m3
    psubb                m2, m7
    pavgb                m2, m3
    movq            [maskq], m2
    add               maskq, 8
    ret

cglobal w_mask_444_16bpc, 4, 7, 12, dst, stride, tmp1, tmp2, w, h, mask
%define base t0-w_mask_444_ssse3_table
    LEA                  t0, w_mask_444_ssse3_table
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    shr                 r6d, 11
    movsxd               wq, [t0+wq*4]
%if ARCH_X86_64
    mova                 m8, [base+pw_27615]
    mova                 m9, [base+pw_64]
    movddup             m10, [base+bidir_rnd+r6*8]
    movddup             m11, [base+bidir_mul+r6*8]
%else
    mova                 m1, [base+pw_27615]
    mova                 m2, [base+pw_64]
    movddup              m3, [base+bidir_rnd+r6*8]
    movddup              m7, [base+bidir_mul+r6*8]
    ALLOC_STACK       -16*3
    mova         [rsp+16*0], m1
    mova         [rsp+16*1], m2
    mova         [rsp+16*2], m3
    %define             m11  m7
%endif
    add                  wq, t0
    movifnidn            hd, r5m
    mov               maskq, r6mp
    call .main
    jmp                  wq
.w4_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w4:
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    movq   [dstq+strideq*0], m1
    movhps [dstq+strideq*1], m1
    sub                  hd, 4
    jg .w4_loop
.end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w8:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    sub                  hd, 2
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w16:
    mova [dstq+strideq*0+16*0], m0
    mova [dstq+strideq*0+16*1], m1
    call .main
    mova [dstq+strideq*1+16*0], m0
    mova [dstq+strideq*1+16*1], m1
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    call .main
    add                dstq, strideq
.w32:
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    call .main
    mova        [dstq+16*2], m0
    mova        [dstq+16*3], m1
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    call .main
    mova        [dstq+16*2], m0
    mova        [dstq+16*3], m1
    call .main
    mova        [dstq+16*4], m0
    mova        [dstq+16*5], m1
    call .main
    mova        [dstq+16*6], m0
    mova        [dstq+16*7], m1
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova       [dstq+16* 0], m0
    mova       [dstq+16* 1], m1
    call .main
    mova       [dstq+16* 2], m0
    mova       [dstq+16* 3], m1
    call .main
    mova       [dstq+16* 4], m0
    mova       [dstq+16* 5], m1
    call .main
    mova       [dstq+16* 6], m0
    mova       [dstq+16* 7], m1
    call .main
    mova       [dstq+16* 8], m0
    mova       [dstq+16* 9], m1
    call .main
    mova       [dstq+16*10], m0
    mova       [dstq+16*11], m1
    call .main
    mova       [dstq+16*12], m0
    mova       [dstq+16*13], m1
    call .main
    mova       [dstq+16*14], m0
    mova       [dstq+16*15], m1
    dec                  hd
    jg .w128_loop
    RET
ALIGN function_align
.main:
    W_MASK                0, 2
    W_MASK                1, 3
    packuswb             m2, m3
    add               tmp1q, 16*2
    add               tmp2q, 16*2
    mova            [maskq], m2
    add               maskq, 16
    ret

; (a * (64 - m) + b * m + 32) >> 6
; = (((b - a) * m + 32) >> 6) + a
; = (((b - a) * (m << 9) + 16384) >> 15) + a
;   except m << 9 overflows int16_t when m == 64 (which is possible),
;   but if we negate m it works out (-64 << 9 == -32768).
; = (((a - b) * (m * -512) + 16384) >> 15) + a
cglobal blend_16bpc, 3, 7, 8, dst, stride, tmp, w, h, mask, stride3
%define base r6-blend_ssse3_table
    LEA                  r6, blend_ssse3_table
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r6+wq*4]
    movifnidn         maskq, maskmp
    mova                 m7, [base+pw_m512]
    add                  wq, r6
    lea            stride3q, [strideq*3]
    pxor                 m6, m6
    jmp                  wq
.w4:
    mova                 m5, [maskq]
    movq                 m0, [dstq+strideq*0]
    movhps               m0, [dstq+strideq*1]
    movq                 m1, [dstq+strideq*2]
    movhps               m1, [dstq+stride3q ]
    psubw                m2, m0, [tmpq+16*0]
    psubw                m3, m1, [tmpq+16*1]
    add               maskq, 16
    add                tmpq, 32
    punpcklbw            m4, m5, m6
    punpckhbw            m5, m6
    pmullw               m4, m7
    pmullw               m5, m7
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    movq   [dstq+strideq*2], m1
    movhps [dstq+stride3q ], m1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4
    RET
.w8:
    mova                 m5, [maskq]
    mova                 m0, [dstq+strideq*0]
    mova                 m1, [dstq+strideq*1]
    psubw                m2, m0, [tmpq+16*0]
    psubw                m3, m1, [tmpq+16*1]
    add               maskq, 16
    add                tmpq, 32
    punpcklbw            m4, m5, m6
    punpckhbw            m5, m6
    pmullw               m4, m7
    pmullw               m5, m7
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8
    RET
.w16:
    mova                 m5, [maskq]
    mova                 m0, [dstq+16*0]
    mova                 m1, [dstq+16*1]
    psubw                m2, m0, [tmpq+16*0]
    psubw                m3, m1, [tmpq+16*1]
    add               maskq, 16
    add                tmpq, 32
    punpcklbw            m4, m5, m6
    punpckhbw            m5, m6
    pmullw               m4, m7
    pmullw               m5, m7
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    add                dstq, strideq
    dec                  hd
    jg .w16
    RET
.w32:
    mova                 m5, [maskq+16*0]
    mova                 m0, [dstq+16*0]
    mova                 m1, [dstq+16*1]
    psubw                m2, m0, [tmpq+16*0]
    psubw                m3, m1, [tmpq+16*1]
    punpcklbw            m4, m5, m6
    punpckhbw            m5, m6
    pmullw               m4, m7
    pmullw               m5, m7
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    mova                 m5, [maskq+16*1]
    mova                 m0, [dstq+16*2]
    mova                 m1, [dstq+16*3]
    psubw                m2, m0, [tmpq+16*2]
    psubw                m3, m1, [tmpq+16*3]
    add               maskq, 32
    add                tmpq, 64
    punpcklbw            m4, m5, m6
    punpckhbw            m5, m6
    pmullw               m4, m7
    pmullw               m5, m7
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova        [dstq+16*2], m0
    mova        [dstq+16*3], m1
    add                dstq, strideq
    dec                  hd
    jg .w32
    RET

cglobal blend_v_16bpc, 3, 6, 6, dst, stride, tmp, w, h
%define base r5-blend_v_ssse3_table
    LEA                  r5, blend_v_ssse3_table
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  wq
.w2:
    movd                 m4, [base+obmc_masks+2*2]
.w2_loop:
    movd                 m0, [dstq+strideq*0]
    movd                 m2, [tmpq+4*0]
    movd                 m1, [dstq+strideq*1]
    movd                 m3, [tmpq+4*1]
    add                tmpq, 4*2
    psubw                m2, m0
    psubw                m3, m1
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
    paddw                m0, m2
    paddw                m1, m3
    movd   [dstq+strideq*0], m0
    movd   [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w2_loop
    RET
.w4:
    movddup              m2, [base+obmc_masks+4*2]
.w4_loop:
    movq                 m0, [dstq+strideq*0]
    movhps               m0, [dstq+strideq*1]
    mova                 m1, [tmpq]
    add                tmpq, 8*2
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w4_loop
    RET
.w8:
    mova                 m4, [base+obmc_masks+8*2]
.w8_loop:
    mova                 m0, [dstq+strideq*0]
    mova                 m2, [tmpq+16*0]
    mova                 m1, [dstq+strideq*1]
    mova                 m3, [tmpq+16*1]
    add                tmpq, 16*2
    psubw                m2, m0
    psubw                m3, m1
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
    paddw                m0, m2
    paddw                m1, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
.w16:
    mova                 m4, [base+obmc_masks+16*2]
    movq                 m5, [base+obmc_masks+16*3]
.w16_loop:
    mova                 m0, [dstq+16*0]
    mova                 m2, [tmpq+16*0]
    mova                 m1, [dstq+16*1]
    mova                 m3, [tmpq+16*1]
    add                tmpq, 16*2
    psubw                m2, m0
    psubw                m3, m1
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    add                dstq, strideq
    dec                  hd
    jg .w16_loop
    RET
.w32:
%if WIN64
    movaps          [rsp+8], m6
%endif
    mova                 m4, [base+obmc_masks+16*4]
    mova                 m5, [base+obmc_masks+16*5]
    mova                 m6, [base+obmc_masks+16*6]
.w32_loop:
    mova                 m0, [dstq+16*0]
    mova                 m2, [tmpq+16*0]
    mova                 m1, [dstq+16*1]
    mova                 m3, [tmpq+16*1]
    psubw                m2, m0
    psubw                m3, m1
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    mova                 m2, [dstq+16*2]
    paddw                m1, m3
    mova                 m3, [tmpq+16*2]
    add                tmpq, 16*4
    psubw                m3, m2
    pmulhrsw             m3, m6
    paddw                m2, m3
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    mova        [dstq+16*2], m2
    add                dstq, strideq
    dec                  hd
    jg .w32_loop
%if WIN64
    movaps               m6, [rsp+8]
%endif
    RET

%macro BLEND_H_ROW 2-3 0; dst_off, tmp_off, inc_tmp
    mova                 m0, [dstq+16*(%1+0)]
    mova                 m2, [tmpq+16*(%2+0)]
    mova                 m1, [dstq+16*(%1+1)]
    mova                 m3, [tmpq+16*(%2+1)]
%if %3
    add                tmpq, 16*%3
%endif
    psubw                m2, m0
    psubw                m3, m1
    pmulhrsw             m2, m5
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova   [dstq+16*(%1+0)], m0
    mova   [dstq+16*(%1+1)], m1
%endmacro

cglobal blend_h_16bpc, 3, 7, 6, dst, ds, tmp, w, h, mask
%define base r6-blend_h_ssse3_table
    LEA                  r6, blend_h_ssse3_table
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    movddup              m4, [base+blend_shuf]
    lea               maskq, [base+obmc_masks+hq*2]
    lea                  hd, [hq*3]
    add                  wq, r6
    shr                  hd, 2 ; h * 3/4
    lea               maskq, [maskq+hq*2]
    neg                  hq
    jmp                  wq
.w2:
    movd                 m0, [dstq+dsq*0]
    movd                 m2, [dstq+dsq*1]
    movd                 m3, [maskq+hq*2]
    movq                 m1, [tmpq]
    add                tmpq, 4*2
    punpckldq            m0, m2
    punpcklwd            m3, m3
    psubw                m1, m0
    pmulhrsw             m1, m3
    paddw                m0, m1
    movd       [dstq+dsq*0], m0
    psrlq                m0, 32
    movd       [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w2
    RET
.w4:
    mova                 m3, [base+blend_shuf]
.w4_loop:
    movq                 m0, [dstq+dsq*0]
    movhps               m0, [dstq+dsq*1]
    movd                 m2, [maskq+hq*2]
    mova                 m1, [tmpq]
    add                tmpq, 8*2
    psubw                m1, m0
    pshufb               m2, m3
    pmulhrsw             m1, m2
    paddw                m0, m1
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w4_loop
    RET
.w8:
    movddup              m5, [base+blend_shuf+8]
%if WIN64
    movaps         [rsp+ 8], m6
    movaps         [rsp+24], m7
%endif
.w8_loop:
    movd                 m7, [maskq+hq*2]
    mova                 m0, [dstq+dsq*0]
    mova                 m2, [tmpq+16*0]
    mova                 m1, [dstq+dsq*1]
    mova                 m3, [tmpq+16*1]
    add                tmpq, 16*2
    pshufb               m6, m7, m4
    psubw                m2, m0
    pshufb               m7, m5
    psubw                m3, m1
    pmulhrsw             m2, m6
    pmulhrsw             m3, m7
    paddw                m0, m2
    paddw                m1, m3
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w8_loop
%if WIN64
    movaps               m6, [rsp+ 8]
    movaps               m7, [rsp+24]
%endif
    RET
.w16:
    movd                 m5, [maskq+hq*2]
    pshufb               m5, m4
    BLEND_H_ROW           0, 0, 2
    add                dstq, dsq
    inc                  hq
    jl .w16
    RET
.w32:
    movd                 m5, [maskq+hq*2]
    pshufb               m5, m4
    BLEND_H_ROW           0, 0
    BLEND_H_ROW           2, 2, 4
    add                dstq, dsq
    inc                  hq
    jl .w32
    RET
.w64:
    movd                 m5, [maskq+hq*2]
    pshufb               m5, m4
    BLEND_H_ROW           0, 0
    BLEND_H_ROW           2, 2
    BLEND_H_ROW           4, 4
    BLEND_H_ROW           6, 6, 8
    add                dstq, dsq
    inc                  hq
    jl .w64
    RET
.w128:
    movd                 m5, [maskq+hq*2]
    pshufb               m5, m4
    BLEND_H_ROW           0,  0
    BLEND_H_ROW           2,  2
    BLEND_H_ROW           4,  4
    BLEND_H_ROW           6,  6, 16
    BLEND_H_ROW           8, -8
    BLEND_H_ROW          10, -6
    BLEND_H_ROW          12, -4
    BLEND_H_ROW          14, -2
    add                dstq, dsq
    inc                  hq
    jl .w128
    RET

; emu_edge args:
; const intptr_t bw, const intptr_t bh, const intptr_t iw, const intptr_t ih,
; const intptr_t x, const intptr_t y, pixel *dst, const ptrdiff_t dst_stride,
; const pixel *ref, const ptrdiff_t ref_stride
;
; bw, bh total filled size
; iw, ih, copied block -> fill bottom, right
; x, y, offset in bw/bh -> fill top, left
cglobal emu_edge_16bpc, 10, 13, 1, bw, bh, iw, ih, x, \
                             y, dst, dstride, src, sstride, \
                             bottomext, rightext, blk
    ; we assume that the buffer (stride) is larger than width, so we can
    ; safely overwrite by a few bytes

%if ARCH_X86_64
 %define reg_zero       r12q
 %define reg_tmp        r10
 %define reg_src        srcq
 %define reg_bottomext  bottomextq
 %define reg_rightext   rightextq
 %define reg_blkm       r9m
%else
 %define reg_zero       r6
 %define reg_tmp        r0
 %define reg_src        r1
 %define reg_bottomext  r0
 %define reg_rightext   r1
 %define reg_blkm       r2m
%endif
    ;
    ; ref += iclip(y, 0, ih - 1) * PXSTRIDE(ref_stride)
    xor            reg_zero, reg_zero
    lea             reg_tmp, [ihq-1]
    cmp                  yq, ihq
    cmovs           reg_tmp, yq
    test                 yq, yq
    cmovs           reg_tmp, reg_zero
%if ARCH_X86_64
    imul            reg_tmp, sstrideq
    add                srcq, reg_tmp
%else
    imul            reg_tmp, sstridem
    mov             reg_src, srcm
    add             reg_src, reg_tmp
%endif
    ;
    ; ref += iclip(x, 0, iw - 1)
    lea             reg_tmp, [iwq-1]
    cmp                  xq, iwq
    cmovs           reg_tmp, xq
    test                 xq, xq
    cmovs           reg_tmp, reg_zero
    lea             reg_src, [reg_src+reg_tmp*2]
%if ARCH_X86_32
    mov                srcm, reg_src
%endif
    ;
    ; bottom_ext = iclip(y + bh - ih, 0, bh - 1)
%if ARCH_X86_32
    mov                  r1, r1m ; restore bh
%endif
    lea       reg_bottomext, [yq+bhq]
    sub       reg_bottomext, ihq
    lea                  r3, [bhq-1]
    cmovs     reg_bottomext, reg_zero
    ;

    DEFINE_ARGS bw, bh, iw, ih, x, \
                topext, dst, dstride, src, sstride, \
                bottomext, rightext, blk

    ; top_ext = iclip(-y, 0, bh - 1)
    neg             topextq
    cmovs           topextq, reg_zero
    cmp       reg_bottomext, bhq
    cmovns    reg_bottomext, r3
    cmp             topextq, bhq
    cmovg           topextq, r3
 %if ARCH_X86_32
    mov                 r4m, reg_bottomext
    ;
    ; right_ext = iclip(x + bw - iw, 0, bw - 1)
    mov                  r0, r0m ; restore bw
 %endif
    lea        reg_rightext, [xq+bwq]
    sub        reg_rightext, iwq
    lea                  r2, [bwq-1]
    cmovs      reg_rightext, reg_zero

    DEFINE_ARGS bw, bh, iw, ih, leftext, \
                topext, dst, dstride, src, sstride, \
                bottomext, rightext, blk

    ; left_ext = iclip(-x, 0, bw - 1)
    neg            leftextq
    cmovs          leftextq, reg_zero
    cmp        reg_rightext, bwq
    cmovns     reg_rightext, r2
 %if ARCH_X86_32
    mov                 r3m, r1
 %endif
    cmp            leftextq, bwq
    cmovns         leftextq, r2

%undef reg_zero
%undef reg_tmp
%undef reg_src
%undef reg_bottomext
%undef reg_rightext

    DEFINE_ARGS bw, centerh, centerw, dummy, leftext, \
                topext, dst, dstride, src, sstride, \
                bottomext, rightext, blk

    ; center_h = bh - top_ext - bottom_ext
%if ARCH_X86_64
    lea                  r3, [bottomextq+topextq]
    sub            centerhq, r3
%else
    mov                   r1, centerhm ; restore r1
    sub             centerhq, topextq
    sub             centerhq, r4m
    mov                  r1m, centerhq
%endif
    ;
    ; blk += top_ext * PXSTRIDE(dst_stride)
    mov                  r2, topextq
%if ARCH_X86_64
    imul                 r2, dstrideq
%else
    mov                  r6, r6m ; restore dstq
    imul                 r2, dstridem
%endif
    add                dstq, r2
    mov            reg_blkm, dstq ; save pointer for ext
    ;
    ; center_w = bw - left_ext - right_ext
    mov            centerwq, bwq
%if ARCH_X86_64
    lea                  r3, [rightextq+leftextq]
    sub            centerwq, r3
%else
    sub            centerwq, r3m
    sub            centerwq, leftextq
%endif

; vloop Macro
%macro v_loop 3 ; need_left_ext, need_right_ext, suffix
  %if ARCH_X86_64
    %define reg_tmp        r12
  %else
    %define reg_tmp        r0
  %endif
.v_loop_%3:
  %if ARCH_X86_32
    mov                  r0, r0m
    mov                  r1, r1m
  %endif
%if %1
    ; left extension
  %if ARCH_X86_64
    movd                 m0, [srcq]
  %else
    mov                  r3, srcm
    movd                 m0, [r3]
  %endif
    pshuflw              m0, m0, q0000
    punpcklqdq           m0, m0
    xor                  r3, r3
.left_loop_%3:
    mova        [dstq+r3*2], m0
    add                  r3, mmsize/2
    cmp                  r3, leftextq
    jl .left_loop_%3
    ; body
    lea             reg_tmp, [dstq+leftextq*2]
%endif
    xor                  r3, r3
.body_loop_%3:
  %if ARCH_X86_64
    movu                 m0, [srcq+r3*2]
  %else
    mov                  r1, srcm
    movu                 m0, [r1+r3*2]
  %endif
%if %1
    movu     [reg_tmp+r3*2], m0
%else
    movu        [dstq+r3*2], m0
%endif
    add                  r3, mmsize/2
    cmp                  r3, centerwq
    jl .body_loop_%3
%if %2
    ; right extension
%if %1
    lea             reg_tmp, [reg_tmp+centerwq*2]
%else
    lea             reg_tmp, [dstq+centerwq*2]
%endif
  %if ARCH_X86_64
    movd                 m0, [srcq+centerwq*2-2]
  %else
    mov                  r3, srcm
    movd                 m0, [r3+centerwq*2-2]
  %endif
    pshuflw              m0, m0, q0000
    punpcklqdq           m0, m0
    xor                  r3, r3
.right_loop_%3:
    movu     [reg_tmp+r3*2], m0
    add                  r3, mmsize/2
  %if ARCH_X86_64
    cmp                  r3, rightextq
  %else
    cmp                  r3, r3m
  %endif
    jl .right_loop_%3
%endif
  %if ARCH_X86_64
    add                dstq, dstrideq
    add                srcq, sstrideq
    dec            centerhq
    jg .v_loop_%3
  %else
    add                dstq, dstridem
    mov                  r0, sstridem
    add                srcm, r0
    sub       dword centerhm, 1
    jg .v_loop_%3
    mov                  r0, r0m ; restore r0
  %endif
%endmacro ; vloop MACRO

    test           leftextq, leftextq
    jnz .need_left_ext
 %if ARCH_X86_64
    test          rightextq, rightextq
    jnz .need_right_ext
 %else
    cmp            leftextq, r3m ; leftextq == 0
    jne .need_right_ext
 %endif
    v_loop                0, 0, 0
    jmp .body_done

    ;left right extensions
.need_left_ext:
 %if ARCH_X86_64
    test          rightextq, rightextq
 %else
    mov                  r3, r3m
    test                 r3, r3
 %endif
    jnz .need_left_right_ext
    v_loop                1, 0, 1
    jmp .body_done

.need_left_right_ext:
    v_loop                1, 1, 2
    jmp .body_done

.need_right_ext:
    v_loop                0, 1, 3

.body_done:
; r0 ; bw
; r1 ;; x loop
; r4 ;; y loop
; r5 ; topextq
; r6 ;dstq
; r7 ;dstrideq
; r8 ; srcq
%if ARCH_X86_64
 %define reg_dstride    dstrideq
%else
 %define reg_dstride    r2
%endif
    ;
    ; bottom edge extension
 %if ARCH_X86_64
    test         bottomextq, bottomextq
    jz .top
 %else
    xor                  r1, r1
    cmp                  r1, r4m
    je .top
 %endif
    ;
 %if ARCH_X86_64
    mov                srcq, dstq
    sub                srcq, dstrideq
    xor                  r1, r1
 %else
    mov                  r3, dstq
    mov         reg_dstride, dstridem
    sub                  r3, reg_dstride
    mov                srcm, r3
 %endif
    ;
.bottom_x_loop:
 %if ARCH_X86_64
    mova                 m0, [srcq+r1*2]
    lea                  r3, [dstq+r1*2]
    mov                  r4, bottomextq
 %else
    mov                  r3, srcm
    mova                 m0, [r3+r1*2]
    lea                  r3, [dstq+r1*2]
    mov                  r4, r4m
 %endif
    ;
.bottom_y_loop:
    mova               [r3], m0
    add                  r3, reg_dstride
    dec                  r4
    jg .bottom_y_loop
    add                  r1, mmsize/2
    cmp                  r1, bwq
    jl .bottom_x_loop

.top:
    ; top edge extension
    test            topextq, topextq
    jz .end
%if ARCH_X86_64
    mov                srcq, reg_blkm
%else
    mov                  r3, reg_blkm
    mov         reg_dstride, dstridem
%endif
    mov                dstq, dstm
    xor                  r1, r1
    ;
.top_x_loop:
%if ARCH_X86_64
    mova                 m0, [srcq+r1*2]
%else
    mov                  r3, reg_blkm
    mova                 m0, [r3+r1*2]
%endif
    lea                  r3, [dstq+r1*2]
    mov                  r4, topextq
    ;
.top_y_loop:
    mova               [r3], m0
    add                  r3, reg_dstride
    dec                  r4
    jg .top_y_loop
    add                  r1, mmsize/2
    cmp                  r1, bwq
    jl .top_x_loop

.end:
    RET

%undef reg_dstride
%undef reg_blkm
%undef reg_tmp

%macro SCRATCH 3
%if ARCH_X86_32
    mova [rsp+%3*mmsize], m%1
%define m%2 [rsp+%3*mmsize]
%else
    SWAP             %1, %2
%endif
%endmacro

%if ARCH_X86_64
cglobal resize_16bpc, 0, 12, 16, 1*16, dst, dst_stride, src, src_stride, \
                                       dst_w, h, src_w, dx, mx0, pxmax
%elif STACK_ALIGNMENT >= 16
cglobal resize_16bpc, 0, 7, 8, 6*16, dst, dst_stride, src, src_stride, \
                                     dst_w, h, src_w, dx, mx0, pxmax
%else
cglobal resize_16bpc, 0, 6, 8, 6*16, dst, dst_stride, src, src_stride, \
                                     dst_w, h, src_w, dx, mx0, pxmax
%endif
    movifnidn         dstq, dstmp
    movifnidn         srcq, srcmp
%if STACK_ALIGNMENT >= 16
    movifnidn       dst_wd, dst_wm
%endif
%if ARCH_X86_64
    movifnidn           hd, hm
%endif
    sub         dword mx0m, 4<<14
    sub       dword src_wm, 8
    movd                m4, pxmaxm
    movd                m7, dxm
    movd                m6, mx0m
    movd                m5, src_wm
    punpcklwd           m4, m4
    pshufd              m4, m4, q0000
    pshufd              m7, m7, q0000
    pshufd              m6, m6, q0000
    pshufd              m5, m5, q0000
    mova [rsp+16*3*ARCH_X86_32], m4
%if ARCH_X86_64
 DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, h, x
    LEA                 r7, $$
 %define base r7-$$
%else
 DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, x
 %define hd dword r5m
 %if STACK_ALIGNMENT >= 16
    LEA                 r6, $$
  %define base r6-$$
 %else
    LEA                 r4, $$
  %define base r4-$$
 %endif
%endif
%if ARCH_X86_64
    mova               m12, [base+pd_64]
    mova               m11, [base+pd_63]
%else
 %define m12 [base+pd_64]
 %define m11 [base+pd_63]
%endif
    pmaddwd             m4, m7, [base+rescale_mul] ; dx*[0,1,2,3]
    pslld               m7, 2                      ; dx*4
    pslld               m5, 14
    paddd               m6, m4                     ; mx+[0..3]*dx
    SCRATCH              7, 15, 0
    SCRATCH              6, 14, 1
    SCRATCH              5, 13, 2
    pxor                m1, m1
.loop_y:
    xor                 xd, xd
    mova                m0, m14            ; per-line working version of mx
.loop_x:
    pcmpgtd             m1, m0
    pandn               m1, m0
    psrad               m2, m0, 8          ; filter offset (unmasked)
    pcmpgtd             m3, m13, m1
    pand                m1, m3
    pandn               m3, m13
    por                 m1, m3
    psubd               m3, m0, m1         ; pshufb offset
    psrad               m1, 14             ; clipped src_x offset
    psrad               m3, 14             ; pshufb edge_emu offset
    pand                m2, m11            ; filter offset (masked)
    ; load source pixels
%if ARCH_X86_64
    movd               r8d, m1
    pshuflw             m1, m1, q3232
    movd               r9d, m1
    punpckhqdq          m1, m1
    movd              r10d, m1
    psrlq               m1, 32
    movd              r11d, m1
    movu                m4, [srcq+r8*2]
    movu                m5, [srcq+r9*2]
    movu                m6, [srcq+r10*2]
    movu                m7, [srcq+r11*2]
    ; if no emulation is required, we don't need to shuffle or emulate edges
    packssdw            m3, m3
    movq               r11, m3
    test               r11, r11
    jz .filter
    movsx               r8, r11w
    sar                r11, 16
    movsx               r9, r11w
    sar                r11, 16
    movsx              r10, r11w
    sar                r11, 16
    movu                m1, [base+resize_shuf+8+r8*2]
    movu                m3, [base+resize_shuf+8+r9*2]
    movu                m8, [base+resize_shuf+8+r10*2]
    movu                m9, [base+resize_shuf+8+r11*2]
    pshufb              m4, m1
    pshufb              m5, m3
    pshufb              m6, m8
    pshufb              m7, m9
.filter:
    movd               r8d, m2
    pshuflw             m2, m2, q3232
    movd               r9d, m2
    punpckhqdq          m2, m2
    movd              r10d, m2
    psrlq               m2, 32
    movd              r11d, m2
    movq                m8, [base+resize_filter+r8*8]
    movq                m2, [base+resize_filter+r9*8]
    pxor                m9, m9
    punpcklbw           m1, m9, m8
    punpcklbw           m3, m9, m2
    psraw               m1, 8
    psraw               m3, 8
    movq               m10, [base+resize_filter+r10*8]
    movq                m2, [base+resize_filter+r11*8]
    punpcklbw           m8, m9, m10
    punpcklbw           m9, m2
    psraw               m8, 8
    psraw               m9, 8
    pmaddwd             m4, m1
    pmaddwd             m5, m3
    pmaddwd             m6, m8
    pmaddwd             m7, m9
    phaddd              m4, m5
%else
    movd                r3, m1
    pshuflw             m1, m1, q3232
    movd                r1, m1
    punpckhqdq          m1, m1
    movu                m4, [srcq+r3*2]
    movu                m5, [srcq+r1*2]
    movd                r3, m1
    psrlq               m1, 32
    movd                r1, m1
    movu                m6, [srcq+r3*2]
    movu                m7, [srcq+r1*2]
    ; if no emulation is required, we don't need to shuffle or emulate edges
    pxor                m1, m1
    pcmpeqb             m1, m3
    pmovmskb           r3d, m1
    cmp                r3d, 0xffff
    je .filter
    movd                r3, m3
    movu                m1, [base+resize_shuf+8+r3*2]
    pshuflw             m3, m3, q3232
    movd                r1, m3
    pshufb              m4, m1
    movu                m1, [base+resize_shuf+8+r1*2]
    punpckhqdq          m3, m3
    movd                r3, m3
    pshufb              m5, m1
    movu                m1, [base+resize_shuf+8+r3*2]
    psrlq               m3, 32
    movd                r1, m3
    pshufb              m6, m1
    movu                m1, [base+resize_shuf+8+r1*2]
    pshufb              m7, m1
.filter:
    mova        [esp+4*16], m6
    mova        [esp+5*16], m7
    movd                r3, m2
    pshuflw             m2, m2, q3232
    movd                r1, m2
    movq                m6, [base+resize_filter+r3*8]
    movq                m7, [base+resize_filter+r1*8]
    pxor                m3, m3
    punpcklbw           m1, m3, m6
    punpcklbw           m3, m7
    psraw               m1, 8
    psraw               m3, 8
    pmaddwd             m4, m1
    pmaddwd             m5, m3
    punpckhqdq          m2, m2
    movd                r3, m2
    psrlq               m2, 32
    movd                r1, m2
    phaddd              m4, m5
    movq                m2, [base+resize_filter+r3*8]
    movq                m5, [base+resize_filter+r1*8]
    mova                m6, [esp+4*16]
    mova                m7, [esp+5*16]
    pxor                m3, m3
    punpcklbw           m1, m3, m2
    punpcklbw           m3, m5
    psraw               m1, 8
    psraw               m3, 8
    pmaddwd             m6, m1
    pmaddwd             m7, m3
%endif
    phaddd              m6, m7
    phaddd              m4, m6
    pxor                m1, m1
    psubd               m2, m12, m4
    psrad               m2, 7
    packssdw            m2, m2
    pmaxsw              m2, m1
    pminsw              m2, [rsp+16*3*ARCH_X86_32]
    movq       [dstq+xq*2], m2
    paddd               m0, m15
    add                 xd, 4
%if STACK_ALIGNMENT >= 16
    cmp                 xd, dst_wd
%else
    cmp                 xd, dst_wm
%endif
    jl .loop_x
    add               dstq, dst_stridemp
    add               srcq, src_stridemp
    dec                 hd
    jg .loop_y
    RET
