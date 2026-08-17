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

SECTION_RODATA 64

%macro SMOOTH_WEIGHTS 1-*
const smooth_weights_1d_16bpc ; sm_weights[] << 7
    %rep %0
        dw %1*128
        %rotate 1
    %endrep
const smooth_weights_2d_16bpc ; sm_weights[], 256 - sm_weights[]
    %rep %0
        dw %1, 256-%1
        %rotate 1
    %endrep
%endmacro

SMOOTH_WEIGHTS   0,   0, 255, 128, 255, 149,  85,  64, \
               255, 197, 146, 105,  73,  50,  37,  32, \
               255, 225, 196, 170, 145, 123, 102,  84, \
                68,  54,  43,  33,  26,  20,  17,  16, \
               255, 240, 225, 210, 196, 182, 169, 157, \
               145, 133, 122, 111, 101,  92,  83,  74, \
                66,  59,  52,  45,  39,  34,  29,  25, \
                21,  17,  14,  12,  10,   9,   8,   8, \
               255, 248, 240, 233, 225, 218, 210, 203, \
               196, 189, 182, 176, 169, 163, 156, 150, \
               144, 138, 133, 127, 121, 116, 111, 106, \
               101,  96,  91,  86,  82,  77,  73,  69, \
                65,  61,  57,  54,  50,  47,  44,  41, \
                38,  35,  32,  29,  27,  25,  22,  20, \
                18,  16,  15,  13,  12,  10,   9,   8, \
                 7,   6,   6,   5,   5,   4,   4,   4

%if ARCH_X86_64

ipred_hv_shuf: db  6,  7,  6,  7,  0,  1,  2,  3,  2,  3,  2,  3,  8,  9, 10, 11
               db  4,  5,  4,  5,  4,  5,  6,  7,  0,  1,  0,  1, 12, 13, 14, 15
filter_shuf1:  db  8,  9,  0,  1,  2,  3,  4,  5,  6,  7, 14, 15, 12, 13, -1, -1
filter_shuf2:  db  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,  4,  5,  2,  3, -1, -1
filter_shuf3:  db 12, 13,  0,  1,  2,  3,  4,  5,  6,  7, 10, 11,  8,  9, -1, -1
pal_pred_shuf: db  0,  2,  4,  6,  8, 10, 12, 14,  1,  3,  5,  7,  9, 11, 13, 15
z_base_inc:    dw   0*64,   1*64,   2*64,   3*64,   4*64,   5*64,   6*64,   7*64
               dw   8*64,   9*64,  10*64,  11*64,  12*64,  13*64,  14*64,  15*64
z_filter_t0:   db 55,127, 39,127, 39,127,  7, 15, 31,  7, 15, 31,  0,  3, 31,  0
z_filter_t1:   db 39, 63, 19, 47, 19, 47,  3,  3,  3,  3,  3,  3,  0,  0,  0,  0
z_filter_wh:   db  7,  7, 11, 11, 15, 15, 19, 19, 19, 23, 23, 23, 31, 31, 31, 39
               db 39, 39, 47, 47, 47, 63, 63, 63, 79, 79, 79, -1
pw_m1024:      times 2 dw -1024
pw_1to16:      dw  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16
pw_16to1:      dw 16, 15, 14, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1
z2_ymul:       dw  1,  2,  1,  2,  1,  2,  1,  2,  3,  4,  3,  4,  3,  4,  3,  4
z2_ymul8:      dw  1,  2,  5,  6,  3,  4,  7,  8,  5,  6, 16, 16,  7,  8
pb_90:         times 4 db 90
z2_y_shuf_h4:  dd  3,  7,  2,  6,  1,  5,  0,  4
z_upsample:    db  0,  1,  4,  5,  8,  9, 12, 13,  2,  3,  6,  7, 10, 11, 14, 15
z2_x_shuf:     db  0,  1,  2,  3,  4,  5,  6,  7,  2,  3,  4,  5,  6,  7,  8,  9
z2_y_shuf:     db  6,  7, 14, 15,  4,  5, 12, 13,  4,  5, 12, 13,  2,  3, 10, 11
z2_y_shuf_us:  db  6,  7, 14, 15,  2,  3, 10, 11,  4,  5, 12, 13,  0,  1,  8,  9
z_filter_k:    dw  4,  4,  5,  5,  4,  4
               dw  8,  8,  6,  6,  4,  4
               dw  0,  0,  0,  0,  2,  2

%define pw_2  (z_filter_k+32)
%define pw_4  (z_filter_k+ 0)
%define pw_16 (z2_ymul8  +20)

pw_1:    times 2 dw 1
pw_3:    times 2 dw 3
pw_62:   times 2 dw 62
pw_512:  times 2 dw 512
pw_2048: times 2 dw 2048
pd_8:    dd 8

%macro JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - 2*4)
    %xdefine %%base mangle(private_prefix %+ _%1_%2)
    %%table:
    %rep %0 - 2
        dd %%base %+ .%3 - (%%table - 2*4)
        %rotate 1
    %endrep
%endmacro

%define ipred_dc_splat_16bpc_avx2_table (ipred_dc_16bpc_avx2_table + 10*4)
%define ipred_cfl_splat_16bpc_avx2_table (ipred_cfl_16bpc_avx2_table + 8*4)

JMP_TABLE ipred_dc_16bpc,         avx2, h4, h8, h16, h32, h64, w4, w8, w16, w32, w64, \
                                        s4-10*4, s8-10*4, s16-10*4, s32-10*4, s64-10*4
JMP_TABLE ipred_dc_left_16bpc,    avx2, h4, h8, h16, h32, h64
JMP_TABLE ipred_h_16bpc,          avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_paeth_16bpc,      avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_16bpc,     avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_h_16bpc,   avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_v_16bpc,   avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_z1_16bpc,         avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_z2_16bpc,         avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_z3_16bpc,         avx2, h4, h8, h16, h32, h64
JMP_TABLE ipred_filter_16bpc,     avx2, w4, w8, w16, w32
JMP_TABLE ipred_cfl_16bpc,        avx2, h4, h8, h16, h32, w4, w8, w16, w32, \
                                        s4-8*4, s8-8*4, s16-8*4, s32-8*4
JMP_TABLE ipred_cfl_left_16bpc,   avx2, h4, h8, h16, h32
JMP_TABLE ipred_cfl_ac_444_16bpc, avx2, w4, w8, w16, w32
JMP_TABLE pal_pred_16bpc,         avx2, w4, w8, w16, w32, w64

cextern dr_intra_derivative
cextern filter_intra_taps

SECTION .text

INIT_YMM avx2
cglobal ipred_dc_top_16bpc, 3, 7, 6, dst, stride, tl, w, h
    movifnidn            hd, hm
    add                 tlq, 2
    movd                xm4, wd
    pxor                xm3, xm3
    pavgw               xm4, xm3
    tzcnt                wd, wd
    movd                xm5, wd
    movu                 m0, [tlq]
    lea                  r5, [ipred_dc_left_16bpc_avx2_table]
    movsxd               r6, [r5+wq*4]
    add                  r6, r5
    add                  r5, ipred_dc_splat_16bpc_avx2_table-ipred_dc_left_16bpc_avx2_table
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6

cglobal ipred_dc_left_16bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    mov                  hd, hm
    sub                 tlq, hq
    movd                xm4, hd
    sub                 tlq, hq
    pxor                xm3, xm3
    pavgw               xm4, xm3
    tzcnt               r6d, hd
    movd                xm5, r6d
    movu                 m0, [tlq]
    lea                  r5, [ipred_dc_left_16bpc_avx2_table]
    movsxd               r6, [r5+r6*4]
    add                  r6, r5
    add                  r5, ipred_dc_splat_16bpc_avx2_table-ipred_dc_left_16bpc_avx2_table
    tzcnt                wd, wd
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6
.h64:
    paddw                m0, [tlq+96]
    paddw                m0, [tlq+64]
.h32:
    paddw                m0, [tlq+32]
.h16:
    vextracti128        xm1, m0, 1
    paddw               xm0, xm1
.h8:
    psrldq              xm1, xm0, 8
    paddw               xm0, xm1
.h4:
    punpcklwd           xm0, xm3
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    paddd               xm0, xm4
    psrld               xm0, xm5
    lea            stride3q, [strideq*3]
    vpbroadcastw         m0, xm0
    mova                 m1, m0
    mova                 m2, m0
    mova                 m3, m0
    jmp                  wq

cglobal ipred_dc_16bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    movifnidn            hd, hm
    tzcnt               r6d, hd
    lea                 r5d, [wq+hq]
    movd                xm4, r5d
    tzcnt               r5d, r5d
    movd                xm5, r5d
    lea                  r5, [ipred_dc_16bpc_avx2_table]
    tzcnt                wd, wd
    movsxd               r6, [r5+r6*4]
    movsxd               wq, [r5+wq*4+5*4]
    pxor                 m3, m3
    psrlw               xm4, 1
    add                  r6, r5
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  r6
.h4:
    movq                xm0, [tlq-8]
    jmp                  wq
.w4:
    movq                xm1, [tlq+2]
    paddw                m0, m4
    paddw                m0, m1
    psrlq                m1, m0, 32
    paddw                m0, m1
    psrld                m1, m0, 16
    paddw                m0, m1
    cmp                  hd, 4
    jg .w4_mul
    psrlw               xm0, 3
    jmp .w4_end
.w4_mul:
    vextracti128        xm1, m0, 1
    paddw               xm0, xm1
    lea                 r2d, [hq*2]
    mov                 r6d, 0xAAAB6667
    shrx                r6d, r6d, r2d
    punpckhwd           xm1, xm0, xm3
    punpcklwd           xm0, xm3
    paddd               xm0, xm1
    movd                xm1, r6d
    psrld               xm0, 2
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w4_end:
    vpbroadcastw        xm0, xm0
.s4:
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm0
    movq   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s4
    RET
ALIGN function_align
.h8:
    mova                xm0, [tlq-16]
    jmp                  wq
.w8:
    vextracti128        xm1, m0, 1
    paddw               xm0, [tlq+2]
    paddw               xm0, xm4
    paddw               xm0, xm1
    psrld               xm1, xm0, 16
    paddw               xm0, xm1
    pblendw             xm0, xm3, 0xAA
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    psrld               xm0, xm5
    cmp                  hd, 8
    je .w8_end
    mov                 r6d, 0xAAAB
    mov                 r2d, 0x6667
    cmp                  hd, 32
    cmovz               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w8_end:
    vpbroadcastw        xm0, xm0
.s8:
    mova   [dstq+strideq*0], xm0
    mova   [dstq+strideq*1], xm0
    mova   [dstq+strideq*2], xm0
    mova   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s8
    RET
ALIGN function_align
.h16:
    mova                 m0, [tlq-32]
    jmp                  wq
.w16:
    paddw                m0, [tlq+2]
    vextracti128        xm1, m0, 1
    paddw               xm0, xm4
    paddw               xm0, xm1
    punpckhwd           xm1, xm0, xm3
    punpcklwd           xm0, xm3
    paddd               xm0, xm1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    psrld               xm0, xm5
    cmp                  hd, 16
    je .w16_end
    mov                 r6d, 0xAAAB
    mov                 r2d, 0x6667
    test                 hb, 8|32
    cmovz               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w16_end:
    vpbroadcastw         m0, xm0
.s16:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m0
    mova   [dstq+strideq*2], m0
    mova   [dstq+stride3q ], m0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s16
    RET
ALIGN function_align
.h32:
    mova                 m0, [tlq-64]
    paddw                m0, [tlq-32]
    jmp                  wq
.w32:
    paddw                m0, [tlq+ 2]
    paddw                m0, [tlq+34]
    vextracti128        xm1, m0, 1
    paddw               xm0, xm4
    paddw               xm0, xm1
    punpcklwd           xm1, xm0, xm3
    punpckhwd           xm0, xm3
    paddd               xm0, xm1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    psrld               xm0, xm5
    cmp                  hd, 32
    je .w32_end
    lea                 r2d, [hq*2]
    mov                 r6d, 0x6667AAAB
    shrx                r6d, r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w32_end:
    vpbroadcastw         m0, xm0
    mova                 m1, m0
.s32:
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*1+32*0], m0
    mova [dstq+strideq*1+32*1], m1
    mova [dstq+strideq*2+32*0], m0
    mova [dstq+strideq*2+32*1], m1
    mova [dstq+stride3q +32*0], m0
    mova [dstq+stride3q +32*1], m1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s32
    RET
ALIGN function_align
.h64:
    mova                 m0, [tlq-128]
    mova                 m1, [tlq- 96]
    paddw                m0, [tlq- 64]
    paddw                m1, [tlq- 32]
    paddw                m0, m1
    jmp                  wq
.w64:
    movu                 m1, [tlq+ 2]
    paddw                m0, [tlq+34]
    paddw                m1, [tlq+66]
    paddw                m0, [tlq+98]
    paddw                m0, m1
    vextracti128        xm1, m0, 1
    paddw               xm0, xm1
    punpcklwd           xm1, xm0, xm3
    punpckhwd           xm0, xm3
    paddd               xm1, xm4
    paddd               xm0, xm1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    psrld               xm0, xm5
    cmp                  hd, 64
    je .w64_end
    mov                 r6d, 0x6667AAAB
    shrx                r6d, r6d, hd
    movd                xm1, r6d
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w64_end:
    vpbroadcastw         m0, xm0
    mova                 m1, m0
    mova                 m2, m0
    mova                 m3, m0
.s64:
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*0+32*2], m2
    mova [dstq+strideq*0+32*3], m3
    mova [dstq+strideq*1+32*0], m0
    mova [dstq+strideq*1+32*1], m1
    mova [dstq+strideq*1+32*2], m2
    mova [dstq+strideq*1+32*3], m3
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .s64
    RET

cglobal ipred_dc_128_16bpc, 2, 7, 6, dst, stride, tl, w, h, stride3
    mov                 r6d, r8m
    shr                 r6d, 11
    lea                  r5, [ipred_dc_splat_16bpc_avx2_table]
    tzcnt                wd, wd
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    vpbroadcastd         m0, [r5-ipred_dc_splat_16bpc_avx2_table+pw_512+r6*4]
    mova                 m1, m0
    mova                 m2, m0
    mova                 m3, m0
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

cglobal ipred_v_16bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    movifnidn            hd, hm
    movu                 m0, [tlq+ 2]
    movu                 m1, [tlq+34]
    movu                 m2, [tlq+66]
    movu                 m3, [tlq+98]
    lea                  r5, [ipred_dc_splat_16bpc_avx2_table]
    tzcnt                wd, wd
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

%macro IPRED_H 2 ; w, store_type
    vpbroadcastw         m0, [tlq-2]
    vpbroadcastw         m1, [tlq-4]
    vpbroadcastw         m2, [tlq-6]
    vpbroadcastw         m3, [tlq-8]
    sub                 tlq, 8
    mov%2  [dstq+strideq*0], m0
    mov%2  [dstq+strideq*1], m1
    mov%2  [dstq+strideq*2], m2
    mov%2  [dstq+stride3q ], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w%1
    RET
ALIGN function_align
%endmacro

cglobal ipred_h_16bpc, 3, 6, 4, dst, stride, tl, w, h, stride3
    movifnidn            hd, hm
    lea                  r5, [ipred_h_16bpc_avx2_table]
    tzcnt                wd, wd
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq
INIT_XMM avx2
.w4:
    IPRED_H               4, q
.w8:
    IPRED_H               8, a
INIT_YMM avx2
.w16:
    IPRED_H              16, a
.w32:
    vpbroadcastw         m0, [tlq-2]
    vpbroadcastw         m1, [tlq-4]
    vpbroadcastw         m2, [tlq-6]
    vpbroadcastw         m3, [tlq-8]
    sub                 tlq, 8
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m0
    mova [dstq+strideq*1+32*0], m1
    mova [dstq+strideq*1+32*1], m1
    mova [dstq+strideq*2+32*0], m2
    mova [dstq+strideq*2+32*1], m2
    mova [dstq+stride3q +32*0], m3
    mova [dstq+stride3q +32*1], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w32
    RET
.w64:
    vpbroadcastw         m0, [tlq-2]
    vpbroadcastw         m1, [tlq-4]
    sub                 tlq, 4
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m0
    mova [dstq+strideq*0+32*2], m0
    mova [dstq+strideq*0+32*3], m0
    mova [dstq+strideq*1+32*0], m1
    mova [dstq+strideq*1+32*1], m1
    mova [dstq+strideq*1+32*2], m1
    mova [dstq+strideq*1+32*3], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w64
    RET

%macro PAETH 3 ; top, signed_ldiff, ldiff
    paddw               m0, m%2, m1
    psubw               m7, m3, m0  ; tldiff
    psubw               m0, m%1     ; tdiff
    pabsw               m7, m7
    pabsw               m0, m0
    pminsw              m7, m0
    pcmpeqw             m0, m7
    pcmpgtw             m7, m%3, m7
    vpblendvb           m0, m3, m%1, m0
    vpblendvb           m0, m1, m0, m7
%endmacro

cglobal ipred_paeth_16bpc, 3, 6, 8, dst, stride, tl, w, h
%define base r5-ipred_paeth_16bpc_avx2_table
    movifnidn           hd, hm
    lea                 r5, [ipred_paeth_16bpc_avx2_table]
    tzcnt               wd, wd
    movsxd              wq, [r5+wq*4]
    vpbroadcastw        m3, [tlq]   ; topleft
    add                 wq, r5
    jmp                 wq
.w4:
    vpbroadcastq        m2, [tlq+2] ; top
    movsldup            m6, [base+ipred_hv_shuf]
    lea                 r3, [strideq*3]
    psubw               m4, m2, m3
    pabsw               m5, m4
.w4_loop:
    sub                tlq, 8
    vpbroadcastq        m1, [tlq]
    pshufb              m1, m6      ; left
    PAETH                2, 4, 5
    vextracti128       xm1, m0, 1
    movq  [dstq+strideq*0], xm0
    movq  [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r3       ], xm1
    lea               dstq, [dstq+strideq*4]
    sub                 hd, 4
    jg .w4_loop
    RET
ALIGN function_align
.w8:
    vbroadcasti128      m2, [tlq+2]
    movsldup            m6, [base+ipred_hv_shuf]
    psubw               m4, m2, m3
    pabsw               m5, m4
.w8_loop:
    sub                tlq, 4
    vpbroadcastd        m1, [tlq]
    pshufb              m1, m6
    PAETH                2, 4, 5
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea               dstq, [dstq+strideq*2]
    sub                 hd, 2
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    movu                m2, [tlq+2]
    psubw               m4, m2, m3
    pabsw               m5, m4
.w16_loop:
    sub                tlq, 2
    vpbroadcastw        m1, [tlq]
    PAETH                2, 4, 5
    mova            [dstq], m0
    add               dstq, strideq
    dec                 hd
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    movu                m2, [tlq+2]
    movu                m6, [tlq+34]
%if WIN64
    movaps             r4m, xmm8
    movaps             r6m, xmm9
%endif
    psubw               m4, m2, m3
    psubw               m8, m6, m3
    pabsw               m5, m4
    pabsw               m9, m8
.w32_loop:
    sub                tlq, 2
    vpbroadcastw        m1, [tlq]
    PAETH                2, 4, 5
    mova       [dstq+32*0], m0
    PAETH                6, 8, 9
    mova       [dstq+32*1], m0
    add               dstq, strideq
    dec                 hd
    jg .w32_loop
%if WIN64
    movaps            xmm8, r4m
    movaps            xmm9, r6m
%endif
    RET
ALIGN function_align
.w64:
    WIN64_SPILL_XMM 16
    movu                m2, [tlq+ 2]
    movu                m6, [tlq+34]
    movu               m10, [tlq+66]
    movu               m13, [tlq+98]
    psubw               m4, m2, m3
    psubw               m8, m6, m3
    psubw              m11, m10, m3
    psubw              m14, m13, m3
    pabsw               m5, m4
    pabsw               m9, m8
    pabsw              m12, m11
    pabsw              m15, m14
.w64_loop:
    sub                tlq, 2
    vpbroadcastw        m1, [tlq]
    PAETH                2, 4, 5
    mova       [dstq+32*0], m0
    PAETH                6, 8, 9
    mova       [dstq+32*1], m0
    PAETH               10, 11, 12
    mova       [dstq+32*2], m0
    PAETH               13, 14, 15
    mova       [dstq+32*3], m0
    add               dstq, strideq
    dec                 hd
    jg .w64_loop
    RET

cglobal ipred_smooth_v_16bpc, 3, 7, 6, dst, stride, tl, w, h, weights
%define base r6-ipred_smooth_v_16bpc_avx2_table
    lea                  r6, [ipred_smooth_v_16bpc_avx2_table]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    lea            weightsq, [base+smooth_weights_1d_16bpc+hq*4]
    neg                  hq
    vpbroadcastw         m5, [tlq+hq*2] ; bottom
    add                  wq, r6
    jmp                  wq
.w4:
    vpbroadcastq         m4, [tlq+2]    ; top
    movsldup             m3, [base+ipred_hv_shuf]
    lea                  r6, [strideq*3]
    psubw                m4, m5         ; top - bottom
.w4_loop:
    vpbroadcastq         m0, [weightsq+hq*2]
    pshufb               m0, m3
    pmulhrsw             m0, m4
    paddw                m0, m5
    vextracti128        xm1, m0, 1
    movhps [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movq   [dstq+r6       ], xm0
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w4_loop
.ret:
    RET
.w8:
    vbroadcasti128       m4, [tlq+2]
    movsldup             m3, [base+ipred_hv_shuf]
    lea                  r6, [strideq*3]
    psubw                m4, m5
.w8_loop:
    vpbroadcastd         m0, [weightsq+hq*2+0]
    vpbroadcastd         m1, [weightsq+hq*2+4]
    pshufb               m0, m3
    pshufb               m1, m3
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    paddw                m0, m5
    paddw                m1, m5
    vextracti128 [dstq+strideq*0], m0, 1
    mova         [dstq+strideq*1], xm0
    vextracti128 [dstq+strideq*2], m1, 1
    mova         [dstq+r6       ], xm1
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w8_loop
    RET
.w16:
    movu                 m4, [tlq+2]
    lea                  r6, [strideq*3]
    psubw                m4, m5
.w16_loop:
    vpbroadcastw         m0, [weightsq+hq*2+0]
    vpbroadcastw         m1, [weightsq+hq*2+2]
    vpbroadcastw         m2, [weightsq+hq*2+4]
    vpbroadcastw         m3, [weightsq+hq*2+6]
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
    REPX   {paddw    x, m5}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+r6       ], m3
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w16_loop
    RET
.w32:
    WIN64_SPILL_XMM       7
    movu                 m4, [tlq+ 2]
    movu                 m6, [tlq+34]
    psubw                m4, m5
    psubw                m6, m5
.w32_loop:
    vpbroadcastw         m1, [weightsq+hq*2+0]
    vpbroadcastw         m3, [weightsq+hq*2+2]
    pmulhrsw             m0, m4, m1
    pmulhrsw             m1, m6
    pmulhrsw             m2, m4, m3
    pmulhrsw             m3, m6
    REPX      {paddw x, m5}, m0, m1, m2, m3
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*1+32*0], m2
    mova [dstq+strideq*1+32*1], m3
    lea                dstq, [dstq+strideq*2]
    add                  hq, 2
    jl .w32_loop
    RET
.w64:
    WIN64_SPILL_XMM       8
    movu                 m3, [tlq+ 2]
    movu                 m4, [tlq+34]
    movu                 m6, [tlq+66]
    movu                 m7, [tlq+98]
    REPX      {psubw x, m5}, m3, m4, m6, m7
.w64_loop:
    vpbroadcastw         m2, [weightsq+hq*2]
    pmulhrsw             m0, m3, m2
    pmulhrsw             m1, m4, m2
    paddw                m0, m5
    paddw                m1, m5
    mova        [dstq+32*0], m0
    pmulhrsw             m0, m6, m2
    mova        [dstq+32*1], m1
    pmulhrsw             m1, m7, m2
    paddw                m0, m5
    paddw                m1, m5
    mova        [dstq+32*2], m0
    mova        [dstq+32*3], m1
    add                dstq, strideq
    inc                  hq
    jl .w64_loop
    RET

cglobal ipred_smooth_h_16bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
%define base r6-ipred_smooth_h_16bpc_avx2_table
    lea                  r6, [ipred_smooth_h_16bpc_avx2_table]
    mov                  wd, wm
    movifnidn            hd, hm
    vpbroadcastw         m5, [tlq+wq*2] ; right
    tzcnt                wd, wd
    add                  hd, hd
    movsxd               wq, [r6+wq*4]
    sub                 tlq, hq
    lea            stride3q, [strideq*3]
    add                  wq, r6
    jmp                  wq
.w4:
    vpbroadcastq         m4, [base+smooth_weights_1d_16bpc+4*2]
    movsldup             m3, [base+ipred_hv_shuf]
.w4_loop:
    vpbroadcastq         m0, [tlq+hq-8] ; left
    pshufb               m0, m3
    psubw                m0, m5         ; left - right
    pmulhrsw             m0, m4
    paddw                m0, m5
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4*2
    jg .w4_loop
    RET
.w8:
    vbroadcasti128       m4, [base+smooth_weights_1d_16bpc+8*2]
    movsldup             m3, [base+ipred_hv_shuf]
.w8_loop:
    vpbroadcastd         m0, [tlq+hq-4]
    vpbroadcastd         m1, [tlq+hq-8]
    pshufb               m0, m3
    pshufb               m1, m3
    psubw                m0, m5
    psubw                m1, m5
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    paddw                m0, m5
    paddw                m1, m5
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+stride3q ], m1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hq, 4*2
    jg .w8_loop
    RET
.w16:
    movu                 m4, [base+smooth_weights_1d_16bpc+16*2]
.w16_loop:
    vpbroadcastq         m3, [tlq+hq-8]
    punpcklwd            m3, m3
    psubw                m3, m5
    pshufd               m0, m3, q3333
    pshufd               m1, m3, q2222
    pshufd               m2, m3, q1111
    pshufd               m3, m3, q0000
    REPX   {pmulhrsw x, m4}, m0, m1, m2, m3
    REPX   {paddw    x, m5}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hq, 4*2
    jg .w16_loop
    RET
.w32:
    WIN64_SPILL_XMM       7
    movu                 m4, [base+smooth_weights_1d_16bpc+32*2]
    movu                 m6, [base+smooth_weights_1d_16bpc+32*3]
.w32_loop:
    vpbroadcastw         m1, [tlq+hq-2]
    vpbroadcastw         m3, [tlq+hq-4]
    psubw                m1, m5
    psubw                m3, m5
    pmulhrsw             m0, m4, m1
    pmulhrsw             m1, m6
    pmulhrsw             m2, m4, m3
    pmulhrsw             m3, m6
    REPX      {paddw x, m5}, m0, m1, m2, m3
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*1+32*0], m2
    mova [dstq+strideq*1+32*1], m3
    lea                dstq, [dstq+strideq*2]
    sub                  hq, 2*2
    jg .w32_loop
    RET
.w64:
    WIN64_SPILL_XMM       8
    movu                 m3, [base+smooth_weights_1d_16bpc+32*4]
    movu                 m4, [base+smooth_weights_1d_16bpc+32*5]
    movu                 m6, [base+smooth_weights_1d_16bpc+32*6]
    movu                 m7, [base+smooth_weights_1d_16bpc+32*7]
.w64_loop:
    vpbroadcastw         m2, [tlq+hq-2]
    psubw                m2, m5
    pmulhrsw             m0, m3, m2
    pmulhrsw             m1, m4, m2
    paddw                m0, m5
    paddw                m1, m5
    mova        [dstq+32*0], m0
    pmulhrsw             m0, m6, m2
    mova        [dstq+32*1], m1
    pmulhrsw             m1, m7, m2
    paddw                m0, m5
    paddw                m1, m5
    mova        [dstq+32*2], m0
    mova        [dstq+32*3], m1
    add                dstq, strideq
    sub                  hq, 1*2
    jg .w64_loop
    RET

%macro SMOOTH_2D_END 6 ; src[1-2], mul[1-2], add[1-2]
    pmaddwd             m0, m%1, m%3
    pmaddwd             m1, m%2, m%4
    paddd               m0, m%5
    paddd               m1, m%6
    psrld               m0, 8
    psrld               m1, 8
    packssdw            m0, m1
    pavgw               m0, m5
%endmacro

cglobal ipred_smooth_16bpc, 3, 7, 6, dst, stride, tl, w, h, v_weights
%define base r6-ipred_smooth_16bpc_avx2_table
    lea                 r6, [ipred_smooth_16bpc_avx2_table]
    mov                 wd, wm
    vpbroadcastw        m4, [tlq+wq*2] ; right
    tzcnt               wd, wd
    mov                 hd, hm
    sub                tlq, hq
    sub                tlq, hq
    movsxd              wq, [r6+wq*4]
    pxor                m5, m5
    add                 wq, r6
    lea         v_weightsq, [base+smooth_weights_2d_16bpc+hq*4]
    jmp                 wq
.w4:
    WIN64_SPILL_XMM     11
    vpbroadcastw        m0, [tlq] ; bottom
    vpbroadcastq        m6, [tlq+hq*2+2]
    movsldup            m7, [base+ipred_hv_shuf]
    movshdup            m9, [base+ipred_hv_shuf]
    vbroadcasti128     m10, [base+smooth_weights_2d_16bpc+4*4]
    punpcklwd           m6, m0 ; top, bottom
    punpcklqdq          m8, m9, m9
    punpckhqdq          m9, m9
    lea                 r3, [strideq*3]
.w4_loop:
    vpbroadcastq        m3, [tlq+hq*2-8]
    vbroadcasti128      m1, [v_weightsq]
    pshufb              m3, m7
    punpcklwd           m2, m3, m4 ; left, right
    punpckhwd           m3, m4
    pmaddwd             m2, m10
    pmaddwd             m3, m10
    pshufb              m0, m1, m8
    pshufb              m1, m9
    SMOOTH_2D_END        0, 1, 6, 6, 2, 3
    vextracti128       xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r3       ], xm1
    lea               dstq, [dstq+strideq*4]
    add         v_weightsq, 16
    sub                 hd, 4
    jg .w4_loop
    RET
.w8:
%assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM     12
    vpbroadcastw        m0, [tlq] ; bottom
    vbroadcasti128      m7, [tlq+hq*2+2]
    movsldup            m8, [base+ipred_hv_shuf]
    movshdup            m9, [base+ipred_hv_shuf]
    vbroadcasti128     m10, [base+smooth_weights_2d_16bpc+8*4+16*0]
    vbroadcasti128     m11, [base+smooth_weights_2d_16bpc+8*4+16*1]
    punpcklwd           m6, m7, m0 ; top, bottom
    punpckhwd           m7, m0
.w8_loop:
    vpbroadcastd        m3, [tlq+hq*2-4]
    vpbroadcastq        m1, [v_weightsq]
    pshufb              m3, m8
    punpcklwd           m2, m3, m4 ; left, right
    punpckhwd           m3, m4
    pmaddwd             m2, m10
    pmaddwd             m3, m11
    pshufb              m1, m9
    SMOOTH_2D_END        1, 1, 6, 7, 2, 3
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea               dstq, [dstq+strideq*2]
    add         v_weightsq, 8
    sub                 hd, 2
    jg .w8_loop
    RET
.w16:
%assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM     11
    vpbroadcastw        m0, [tlq] ; bottom
    movu                m7, [tlq+hq*2+2]
    mova               xm8, [base+smooth_weights_2d_16bpc+16*4+16*0]
    mova               xm9, [base+smooth_weights_2d_16bpc+16*4+16*1]
    vinserti128         m8, [base+smooth_weights_2d_16bpc+16*4+16*2], 1
    vinserti128         m9, [base+smooth_weights_2d_16bpc+16*4+16*3], 1
    punpcklwd           m6, m7, m0 ; top, bottom
    punpckhwd           m7, m0
.w16_loop:
    vpbroadcastd        m3, [tlq+hq*2-4]
    vpbroadcastd        m1, [v_weightsq+0]
    punpcklwd           m3, m4     ; left, right
    pshufd              m2, m3, q1111
    pmaddwd            m10, m8, m2
    pmaddwd             m2, m9
    pshufd              m3, m3, q0000
    SMOOTH_2D_END        1, 1, 6, 7, 10, 2
    vpbroadcastd        m1, [v_weightsq+4]
    pmaddwd             m2, m8, m3
    pmaddwd             m3, m9
    mova  [dstq+strideq*0], m0
    SMOOTH_2D_END        1, 1, 6, 7, 2, 3
    mova  [dstq+strideq*1], m0
    lea               dstq, [dstq+strideq*2]
    add         v_weightsq, 8
    sub                 hq, 2
    jg .w16_loop
    RET
.w32:
%assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM     15
    vpbroadcastw        m0, [tlq] ; bottom
    movu                m7, [tlq+hq*2+ 2]
    movu                m9, [tlq+hq*2+34]
    mova              xm10, [base+smooth_weights_2d_16bpc+32*4+16*0]
    mova              xm11, [base+smooth_weights_2d_16bpc+32*4+16*1]
    vinserti128        m10, [base+smooth_weights_2d_16bpc+32*4+16*2], 1
    vinserti128        m11, [base+smooth_weights_2d_16bpc+32*4+16*3], 1
    mova              xm12, [base+smooth_weights_2d_16bpc+32*4+16*4]
    mova              xm13, [base+smooth_weights_2d_16bpc+32*4+16*5]
    vinserti128        m12, [base+smooth_weights_2d_16bpc+32*4+16*6], 1
    vinserti128        m13, [base+smooth_weights_2d_16bpc+32*4+16*7], 1
    punpcklwd           m6, m7, m0
    punpckhwd           m7, m0
    punpcklwd           m8, m9, m0
    punpckhwd           m9, m0
.w32_loop:
    vpbroadcastw        m3, [tlq+hq*2-2]
    vpbroadcastd       m14, [v_weightsq]
    punpcklwd           m3, m4
    pmaddwd             m1, m10, m3
    pmaddwd             m2, m11, m3
    pmaddwd             m0, m6, m14
    paddd               m0, m1
    pmaddwd             m1, m7, m14
    paddd               m1, m2
    pmaddwd             m2, m12, m3
    pmaddwd             m3, m13
    psrld               m0, 8
    psrld               m1, 8
    packssdw            m0, m1
    pavgw               m0, m5
    mova       [dstq+32*0], m0
    SMOOTH_2D_END       14, 14, 8, 9, 2, 3
    mova       [dstq+32*1], m0
    add               dstq, strideq
    add         v_weightsq, 4
    dec                 hd
    jg .w32_loop
    RET
.w64:
%assign stack_offset stack_offset - stack_size_padded
    PROLOGUE 0, 11, 16, dst, stride, tl, tl_base, h, v_weights, dummy, v_weights_base, x, y, dst_base
    mov          dst_baseq, dstq
    mov           tl_baseq, tlq
    mov    v_weights_baseq, v_weightsq
    xor                 xq, xq
.w64_loop_x:
    mov                 yq, hq
    lea                tlq, [tl_baseq+hq*2]
    vpbroadcastw        m0, [tl_baseq] ; bottom
    movu                m7, [tlq+xq*2+ 2]
    movu                m9, [tlq+xq*2+34]
    mova              xm10, [base+smooth_weights_2d_16bpc+64*4+16*0]
    mova              xm11, [base+smooth_weights_2d_16bpc+64*4+16*1]
    vinserti128        m10, [base+smooth_weights_2d_16bpc+64*4+16*2], 1
    vinserti128        m11, [base+smooth_weights_2d_16bpc+64*4+16*3], 1
    mova              xm12, [base+smooth_weights_2d_16bpc+64*4+16*4]
    mova              xm13, [base+smooth_weights_2d_16bpc+64*4+16*5]
    vinserti128        m12, [base+smooth_weights_2d_16bpc+64*4+16*6], 1
    vinserti128        m13, [base+smooth_weights_2d_16bpc+64*4+16*7], 1
    punpcklwd           m6, m7, m0
    punpckhwd           m7, m0
    punpcklwd           m8, m9, m0
    punpckhwd           m9, m0
    lea                tlq, [tl_baseq-2]
.w64_loop_y:
    vpbroadcastw        m3, [tlq+yq*2]
    vpbroadcastd        m1, [v_weightsq]
    punpcklwd           m3, m4
    pmaddwd            m14, m10, m3
    pmaddwd            m15, m11, m3
    pmaddwd             m2, m12, m3
    pmaddwd             m3, m13
    pmaddwd             m0, m6, m1
    paddd               m0, m14
    pmaddwd            m14, m7, m1
    paddd              m14, m15
    psrld               m0, 8
    psrld              m14, 8
    packssdw            m0, m14
    pavgw               m0, m5
    mova       [dstq+32*0], m0
    SMOOTH_2D_END        8, 9, 1, 1, 2, 3
    mova       [dstq+32*1], m0
    add               dstq, strideq
    add         v_weightsq, 4
    dec                 yq
    jg .w64_loop_y
    lea               dstq, [dst_baseq+32*2]
    add                 r6, 16*8
    mov         v_weightsq, v_weights_baseq
    add                 xq, 32
    test                xb, 64
    jz .w64_loop_x
    RET

cglobal ipred_z1_16bpc, 3, 8, 0, dst, stride, tl, w, h, angle, dx, maxbase
    %assign org_stack_offset stack_offset
    lea                  r6, [ipred_z1_16bpc_avx2_table]
    tzcnt                wd, wm
    movifnidn        angled, anglem
    movifnidn            hd, hm
    lea                  r7, [dr_intra_derivative]
    movsxd               wq, [r6+wq*4]
    add                 tlq, 2
    add                  wq, r6
    mov                 dxd, angled
    and                 dxd, 0x7e
    add              angled, 165 ; ~90
    movzx               dxd, word [r7+dxq]
    xor              angled, 0x4ff ; d = 90 - angle
    vpbroadcastd         m5, [pw_62]
    jmp                  wq
.w4:
    ALLOC_STACK         -64, 7
    cmp              angleb, 40
    jae .w4_no_upsample
    lea                 r3d, [angleq-1024]
    sar                 r3d, 7
    add                 r3d, hd
    jg .w4_no_upsample ; !enable_intra_edge_filter || h > 8 || (h == 8 && is_sm)
    vpbroadcastw        xm3, [tlq+14]
    movu                xm1, [tlq+ 0]    ; 1 2 3 4 5 6 7 8
    palignr             xm0, xm3, xm1, 4 ; 3 4 5 6 7 8 8 8
    paddw               xm0, [tlq- 2]    ; 0 1 2 3 4 5 6 7
    add                 dxd, dxd
    palignr             xm2, xm3, xm1, 2 ; 2 3 4 5 6 7 8 8
    paddw               xm2, xm1         ; -1 * a + 9 * b + 9 * c + -1 * d
    psubw               xm0, xm2, xm0    ; = (b + c - a - d + (b + c) << 3 + 8) >> 4
    psraw               xm0, 3           ; = ((b + c - a - d) >> 3 + b + c + 1) >> 1
    pxor                xm4, xm4
    paddw               xm2, xm0
    vpbroadcastw        xm0, r8m         ; pixel_max
    mova           [rsp+32], xm3
    movd                xm3, dxd
    pmaxsw              xm2, xm4
    mov                 r3d, dxd
    pavgw               xm2, xm4
    vpbroadcastw         m3, xm3
    pminsw              xm2, xm0
    punpcklwd           xm0, xm1, xm2
    punpckhwd           xm1, xm2
    lea                  r5, [strideq*3]
    pslldq               m2, m3, 8
    mova           [rsp+ 0], xm0
    mova           [rsp+16], xm1
    paddw                m6, m3, m3
    paddw                m3, m2
    vpblendd             m4, m6, 0xf0
    paddw                m6, m6
    paddw                m3, m4 ; xpos0 xpos1 xpos2 xpos3
    vbroadcasti128       m4, [z_upsample]
.w4_upsample_loop:
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6 ; base0
    movu                xm1, [rsp+r3*2]
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6 ; base1
    movu                xm2, [rsp+r2*2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6 ; base2
    vinserti128          m1, [rsp+r3*2], 1 ; 0 2
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6 ; base3
    vinserti128          m2, [rsp+r2*2], 1 ; 1 3
    pshufb               m1, m4
    pshufb               m2, m4
    punpcklqdq           m0, m1, m2
    punpckhqdq           m1, m2
    pand                 m2, m5, m3 ; frac
    psllw                m2, 9      ; (a * (64 - frac) + b * frac + 32) >> 6
    psubw                m1, m0     ; = a + (((b - a) * frac + 32) >> 6)
    pmulhrsw             m1, m2     ; = a + (((b - a) * (frac << 9) + 16384) >> 15)
    paddw                m3, m6     ; xpos += dx
    paddw                m0, m1
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+r5       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_upsample_loop
    RET
ALIGN function_align
.filter_strength: ; w4/w8/w16
%define base r3-z_filter_t0
    movd                xm0, maxbased
    lea                  r3, [z_filter_t0]
    movd                xm1, angled
    shr              angled, 8 ; is_sm << 1
    vpbroadcastb         m0, xm0
    vpbroadcastb         m1, xm1
    pcmpeqb              m0, [base+z_filter_wh]
    mova                xm2, [r3+angleq*8]
    pand                 m0, m1
    pcmpgtb              m0, m2
    pmovmskb            r5d, m0
    ret
.w4_no_upsample:
    mov            maxbased, 7
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .w4_main
    lea            maxbased, [hq+3]
    call .filter_strength
    mov            maxbased, 7
    test                r5d, r5d
    jz .w4_main ; filter_strength == 0
    popcnt              r5d, r5d
    vpbroadcastw        xm3, [tlq+14]
    mova                xm0, [tlq- 2]      ; 0 1 2 3 4 5 6 7
    vpbroadcastd        xm1, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd        xm4, [base+z_filter_k-4+r5*4+12*0]
    palignr             xm2, xm3, xm0, 4   ; 2 3 4 5 6 7 8 8
    pmullw              xm1, [tlq+ 0]      ; 1 2 3 4 5 6 7 8
    paddw               xm2, xm0
    pmullw              xm2, xm4
    movd           [rsp+16], xm3
    cmp                 r5d, 3
    jne .w4_3tap
    paddw               xm1, xm2
    palignr             xm2, xm3, xm0, 6   ; 3 4 5 6 7 8 8 8
    pblendw             xm0, [tlq-4], 0xfe ; 0 0 1 2 3 4 5 6
    movzx               r3d, word [tlq+14]
    movzx               r2d, word [tlq+12]
    inc            maxbased
    paddw               xm2, xm0
    sub                 r2d, r3d
    paddw               xm2, xm2
    lea                 r2d, [r2+r3*8+4]
    shr                 r2d, 3 ; (1 * top[6] + 7 * top[7] + 4) >> 3
    mov            [rsp+16], r2w
.w4_3tap:
    pxor                xm0, xm0
    paddw               xm1, xm2
    mov                 tlq, rsp
    psrlw               xm1, 3
    cmp                  hd, 8
    sbb            maxbased, -1
    pavgw               xm0, xm1
    mova              [tlq], xm0
.w4_main:
    movd                xm3, dxd
    vpbroadcastq         m1, [z_base_inc]
    vpbroadcastw         m6, [tlq+maxbaseq*2] ; top[max_base_x]
    shl            maxbased, 6
    vpbroadcastw         m3, xm3
    movd                xm0, maxbased
    mov                 r3d, dxd      ; xpos
    vpbroadcastw         m0, xm0
    paddw                m4, m3, m3
    psubw                m1, m0       ; -max_base_x
    vpblendd             m3, m4, 0xcc
    paddw                m0, m4, m3
    vpblendd             m3, m0, 0xf0 ; xpos0 xpos1 xpos2 xpos3
    paddw                m4, m4
    paddw                m3, m1
.w4_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base0
    movu                xm1, [tlq+r3*2]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base1
    movu                xm2, [tlq+r5*2]
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base2
    vinserti128          m1, [tlq+r3*2], 1 ; 0 2
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base3
    vinserti128          m2, [tlq+r5*2], 1 ; 1 3
    punpcklqdq           m0, m1, m2
    psrldq               m1, 2
    pslldq               m2, 6
    vpblendd             m1, m2, 0xcc
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    psraw                m2, m3, 15 ; xpos < max_base_x
    paddw                m3, m4
    paddw                m0, m1
    vpblendvb            m0, m6, m0, m2
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    lea                dstq, [dstq+strideq*2]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    sub                  hd, 4
    jz .w4_end
    lea                dstq, [dstq+strideq*2]
    cmp                 r3d, maxbased
    jb .w4_loop
    lea                  r6, [strideq*3]
.w4_end_loop:
    movq   [dstq+strideq*0], xm6
    movq   [dstq+strideq*1], xm6
    movq   [dstq+strideq*2], xm6
    movq   [dstq+r6       ], xm6
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_end_loop
.w4_end:
    RET
.w8:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -64, 7
    lea                 r3d, [angleq+216]
    mov                 r3b, hb
    cmp                 r3d, 8
    ja .w8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || h > 8
    movu                 m2, [tlq+2]    ; 2 3 4 5 6 7 8 9   a b c d e f g _
    movu                 m0, [tlq+4]    ; 3 4 5 6 7 8 9 a   b c d e f g _ _
    movu                 m1, [tlq+0]    ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    cmp                  hd, 4
    jne .w8_upsample_h8 ; awkward single-pixel edge case
    vpblendd             m0, m2, 0x20   ; 3 4 5 6 7 8 9 a   b c c _ _ _ _ _
.w8_upsample_h8:
    paddw                m2, m1
    paddw                m0, [tlq-2]    ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    add                 dxd, dxd
    psubw                m0, m2, m0
    psraw                m0, 3
    pxor                 m4, m4
    paddw                m2, m0
    vpbroadcastw         m0, r8m
    movd                xm3, dxd
    pmaxsw               m2, m4
    mov                 r3d, dxd
    pavgw                m2, m4
    vpbroadcastw         m3, xm3
    pminsw               m2, m0
    punpcklwd            m0, m1, m2
    punpckhwd            m1, m2
    vbroadcasti128       m4, [z_upsample]
    mova           [rsp+ 0], xm0
    mova           [rsp+16], xm1
    paddw                m6, m3, m3
    vextracti128   [rsp+32], m0, 1
    vextracti128   [rsp+48], m1, 1
    vpblendd             m3, m6, 0xf0 ; xpos0 xpos1
.w8_upsample_loop:
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6 ; base0
    movu                xm1, [rsp+r3*2]
    movu                xm2, [rsp+r3*2+16]
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6 ; base1
    vinserti128          m1, [rsp+r2*2], 1
    vinserti128          m2, [rsp+r2*2+16], 1
    pshufb               m1, m4
    pshufb               m2, m4
    punpcklqdq           m0, m1, m2
    punpckhqdq           m1, m2
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m3, m6
    paddw                m0, m1
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_upsample_loop
    RET
.w8_no_intra_edge_filter:
    and            maxbased, 7
    or             maxbased, 8 ; imin(h+7, 15)
    jmp .w8_main
.w8_no_upsample:
    lea            maxbased, [hq+7]
    test             angled, 0x400
    jnz .w8_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .w8_main
    popcnt              r5d, r5d
    vpbroadcastd         m1, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd         m4, [base+z_filter_k-4+r5*4+12*0]
    mova                 m0, [tlq-2]           ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    movu                 m2, [tlq+0]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pmullw               m1, m2
    cmp                  hd, 8
    jl .w8_filter_h4
    punpckhwd            m2, m2
    vpblendd             m3, m2, [tlq+2], 0x7f ; 2 3 4 5 6 7 8 9   a b c d e f g g
    je .w8_filter_end ; 8x4 and 8x8 are always 3-tap
    movzx               r3d, word [tlq+30]
    mov            maxbased, 16
    mov            [rsp+32], r3d
    cmp                 r5d, 3
    jne .w8_filter_end
    punpcklwd           xm6, xm0, xm0
    vpblendd             m2, [tlq+4], 0x7f     ; 3 4 5 6 7 8 9 a   b c d e f g g g
    vpblendd             m6, [tlq-4], 0xfe     ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    movzx               r5d, word [tlq+28]
    mov            [rsp+34], r3w
    paddw                m2, m6
    sub                 r5d, r3d
    inc            maxbased
    paddw                m2, m2
    lea                 r3d, [r5+r3*8+4]
    paddw                m1, m2
    shr                 r3d, 3
    mov            [rsp+32], r3w
    jmp .w8_filter_end
.w8_filter_h4:
    pshuflw              m3, m2, q3321
    vinserti128          m3, [tlq+2], 0        ; 2 3 4 5 6 7 8 9   a b c c _ _ _ _
.w8_filter_end:
    paddw                m0, m3
    pmullw               m0, m4
    mov                 tlq, rsp
    pxor                 m2, m2
    paddw                m0, m1
    psrlw                m0, 3
    pavgw                m0, m2
    mova              [tlq], m0
.w8_main:
    movd                xm3, dxd
    vbroadcasti128       m1, [z_base_inc]
    vpbroadcastw         m6, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m3, xm3
    movd                xm0, maxbased
    mov                 r3d, dxd
    vpbroadcastw         m0, xm0
    paddw                m4, m3, m3
    psubw                m1, m0
    vpblendd             m3, m4, 0xf0 ; xpos0 xpos1
    paddw                m3, m1
.w8_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6
    movu                xm0, [tlq+r3*2]
    movu                xm1, [tlq+r3*2+2]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6
    vinserti128          m0, [tlq+r5*2], 1
    vinserti128          m1, [tlq+r5*2+2], 1
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    psraw                m2, m3, 15
    paddw                m3, m4
    paddw                m0, m1
    vpblendvb            m0, m6, m0, m2
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    sub                  hd, 2
    jz .w8_end
    lea                dstq, [dstq+strideq*2]
    cmp                 r3d, maxbased
    jb .w8_loop
.w8_end_loop:
    mova   [dstq+strideq*0], xm6
    mova   [dstq+strideq*1], xm6
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_end_loop
.w8_end:
    RET
.w16_no_intra_edge_filter:
    and            maxbased, 15
    or             maxbased, 16 ; imin(h+15, 31)
    jmp .w16_main
.w16:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -96, 7
    lea            maxbased, [hq+15]
    test             angled, 0x400
    jnz .w16_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .w16_main
    popcnt              r5d, r5d
    mova                 m0, [tlq-2]            ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m1, m0, [tlq+2]        ; 2 3 4 5 6 7 8 9   a b c d e f g h
    cmp                 r5d, 3
    jne .w16_filter_3tap
    vpbroadcastd         m2, [base+pw_3]
    punpcklwd           xm0, xm0
    vpblendd             m0, [tlq-4], 0xfe      ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    paddw                m1, [tlq+0]            ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    paddw                m0, m2
    pavgw                m0, [tlq+4]            ; 3 4 5 6 7 8 9 a   b c d e f g h i
    paddw                m0, m1
    psrlw                m0, 2
    movu                 m3, [tlq+32]           ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m2, [tlq+28]           ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m1, m3, [tlq+30]       ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    cmp                  hd, 8
    jl .w16_filter_5tap_h4
    punpckhwd            m3, m3
    je .w16_filter_5tap_h8
    vpblendd             m4, m3, [tlq+36], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
    vpblendd             m3, [tlq+34], 0x7f     ; 3 4 5 6 7 8 9 a   b c d e f g h h
    movzx               r3d, word [tlq+62]
    movzx               r2d, word [tlq+60]
    pavgw                m2, m4
    sub                 r2d, r3d
    paddw                m1, m3
    lea                 r2d, [r2+r3*8+4]
    paddw                m1, m2
    shr                 r2d, 3
    psrlw                m1, 2
    mov            [rsp+66], r3w
    mov            [rsp+64], r2w
    mov                 tlq, rsp
    mov                 r3d, 33
    cmp                  hd, 16
    cmovg          maxbased, r3d
    jmp .w16_filter_end2
.w16_filter_5tap_h8:
    vpblendd            xm4, xm3, [tlq+36], 0x07 ; 4 5 6 7 8 9 9 9
    vpblendd            xm3, [tlq+34], 0x07      ; 3 4 5 6 7 8 9 9
    pavgw               xm2, xm4
    paddw               xm1, xm3
    paddw               xm1, xm2
    psrlw               xm1, 2
    jmp .w16_filter_end2
.w16_filter_5tap_h4:
    pshuflw             xm4, xm3, q3332          ; 4 5 5 5
    pshuflw             xm3, xm3, q3321          ; 3 4 5 5
    pavgw               xm2, xm4
    paddw               xm1, xm3
    paddw               xm1, xm2
    psrlw               xm1, 2
    jmp .w16_filter_end2
.w16_filter_3tap:
    vpbroadcastd         m3, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd         m4, [base+z_filter_k-4+r5*4+12*0]
    pmullw               m0, m3, [tlq+0]    ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    movu                 m2, [tlq+32]       ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pmullw               m1, m4
    pmullw               m3, m2
    paddw                m0, m1
    cmp                  hd, 8
    je .w16_filter_3tap_h8
    jl .w16_filter_3tap_h4
    punpckhwd            m2, m2
    vpblendd             m2, [tlq+34], 0x7f ; 2 3 4 5 6 7 8 9   a b c d e f g g
    jmp .w16_filter_end
.w16_filter_3tap_h4:
    pshuflw             xm2, xm2, q3321     ; 2 3 4 4 _ _ _ _
    jmp .w16_filter_end
.w16_filter_3tap_h8:
    psrldq              xm2, 2
    pshufhw             xm2, xm2, q2210     ; 2 3 4 5 6 7 8 8
.w16_filter_end:
    paddw                m2, [tlq+30]       ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    pmullw               m2, m4
    psrlw                m0, 3
    pxor                 m1, m1
    paddw                m2, m3
    psrlw                m2, 3
    pavgw                m0, m1
    pavgw                m1, m2
.w16_filter_end2:
    mov                 tlq, rsp
    mova           [tlq+ 0], m0
    mova           [tlq+32], m1
.w16_main:
    movd                xm4, dxd
    vpbroadcastw         m6, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m4, xm4
    movd                xm0, maxbased
    mov                 r3d, dxd
    vpbroadcastw         m0, xm0
    paddw                m3, m4, [z_base_inc]
    psubw                m3, m0
.w16_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6
    movu                 m0, [tlq+r3*2]
    movu                 m1, [tlq+r3*2+2]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    psraw                m2, m3, 15
    paddw                m3, m4
    paddw                m1, m0
    movu                 m0, [tlq+r5*2]
    vpblendvb            m2, m6, m1, m2
    movu                 m1, [tlq+r5*2+2]
    mova   [dstq+strideq*0], m2
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    psraw                m2, m3, 15
    paddw                m3, m4
    paddw                m0, m1
    vpblendvb            m0, m6, m0, m2
    mova   [dstq+strideq*1], m0
    sub                  hd, 2
    jz .w16_end
    lea                dstq, [dstq+strideq*2]
    cmp                 r3d, maxbased
    jb .w16_loop
.w16_end_loop:
    mova   [dstq+strideq*0], m6
    mova   [dstq+strideq*1], m6
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_end_loop
.w16_end:
    RET
.w32:
    %assign stack_offset org_stack_offset
    ALLOC_STACK        -160, 8
    lea            maxbased, [hq+31]
    mov                 r3d, 63
    cmp                  hd, 32
    cmova          maxbased, r3d
    test             angled, 0x400
    jnz .w32_main
    vpbroadcastd         m2, [pw_3]
    mova                 m0, [tlq-2]       ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    punpcklwd           xm1, xm0, xm0
    vpblendd             m1, [tlq-4], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    paddw                m0, [tlq+0]       ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    paddw                m1, m2
    paddw                m0, [tlq+2]       ; 2 3 4 5 6 7 8 9   a b c d e f g h
    pavgw                m1, [tlq+4]       ; 3 4 5 6 7 8 9 a   b c d e f g h i
    mov                  r3, rsp
    paddw                m0, m1
    lea                 r5d, [maxbaseq-31]
    psrlw                m0, 2
    mova               [r3], m0
.w32_filter_loop:
    mova                 m0, [tlq+30]
    paddw                m1, m2, [tlq+28]
    add                 tlq, 32
    paddw                m0, [tlq+0]
    pavgw                m1, [tlq+4]
    paddw                m0, [tlq+2]
    add                  r3, 32
    paddw                m0, m1
    psrlw                m0, 2
    mova               [r3], m0
    sub                 r5d, 16
    jg .w32_filter_loop
    movu                 m0, [tlq+32]           ; 2 3 4 5 6 7 8 9   a b c d e f g h
    punpckhwd            m1, m0, m0
    paddw                m2, [tlq+28]           ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m0, [tlq+30]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    jl .w32_filter_h8
    vpblendd             m3, m1, [tlq+36], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
    vpblendd             m1, [tlq+34], 0x7f     ; 3 4 5 6 7 8 9 a   b c d e f g h h
    movzx               r5d, word [tlq+62]
    movzx               r2d, word [tlq+60]
    pavgw                m2, m3
    sub                 r2d, r5d
    paddw                m0, m1
    lea                 r2d, [r2+r5*8+4]
    paddw                m0, m2
    shr                 r2d, 3
    psrlw                m0, 2
    mova            [r3+32], m0
    mov             [r3+66], r5w
    mov             [r3+64], r2w
    mov                 tlq, rsp
    mov                 r3d, 65
    cmp                  hd, 64
    cmove          maxbased, r3d
    jmp .w32_main
.w32_filter_h8:
    vpblendd            xm3, xm1, [tlq+36], 0x07 ; 4 5 6 7 8 9 9 9
    vpblendd            xm1, [tlq+34], 0x07      ; 3 4 5 6 7 8 9 9
    pavgw               xm2, xm3
    paddw               xm0, xm1
    mov                 tlq, rsp
    paddw               xm0, xm2
    psrlw               xm0, 2
    mova            [r3+32], xm0
.w32_main:
    movd                xm4, dxd
    vpbroadcastw         m6, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m4, xm4
    movd                xm0, maxbased
    mov                 r5d, dxd
    vpbroadcastd         m7, [pw_m1024] ; -16 * 64
    vpbroadcastw         m0, xm0
    paddw                m3, m4, [z_base_inc]
    psubw                m3, m0
.w32_loop:
    mov                 r3d, r5d
    shr                 r3d, 6
    movu                 m0, [tlq+r3*2]
    movu                 m1, [tlq+r3*2+2]
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    psraw                m1, m3, 15
    vpblendvb            m0, m6, m0, m1
    mova        [dstq+32*0], m0
    movu                 m0, [tlq+r3*2+32]
    movu                 m1, [tlq+r3*2+34]
    add                 r5d, dxd
    psubw                m1, m0
    pmulhrsw             m1, m2
    pcmpgtw              m2, m7, m3
    paddw                m3, m4
    paddw                m0, m1
    vpblendvb            m0, m6, m0, m2
    mova        [dstq+32*1], m0
    dec                  hd
    jz .w32_end
    add                dstq, strideq
    cmp                 r5d, maxbased
    jb .w32_loop
.w32_end_loop:
    mova        [dstq+32*0], m6
    mova        [dstq+32*1], m6
    add                dstq, strideq
    dec                  hd
    jg .w32_end_loop
.w32_end:
    RET
.w64:
    %assign stack_offset org_stack_offset
    ALLOC_STACK        -256, 10
    lea            maxbased, [hq+63]
    test             angled, 0x400
    jnz .w64_main
    vpbroadcastd         m2, [pw_3]
    mova                 m0, [tlq-2]       ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    punpcklwd           xm1, xm0, xm0
    vpblendd             m1, [tlq-4], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    paddw                m0, [tlq+0]       ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    paddw                m1, m2
    paddw                m0, [tlq+2]       ; 2 3 4 5 6 7 8 9   a b c d e f g h
    pavgw                m1, [tlq+4]       ; 3 4 5 6 7 8 9 a   b c d e f g h i
    mov                  r3, rsp
    paddw                m0, m1
    lea                 r5d, [hq+32]
    psrlw                m0, 2
    mova               [r3], m0
.w64_filter_loop:
    mova                 m0, [tlq+30]
    paddw                m1, m2, [tlq+28]
    add                 tlq, 32
    paddw                m0, [tlq+0]
    pavgw                m1, [tlq+4]
    paddw                m0, [tlq+2]
    add                  r3, 32
    paddw                m0, m1
    psrlw                m0, 2
    mova               [r3], m0
    sub                 r5d, 16
    jg .w64_filter_loop
    movu                 m0, [tlq+32]           ; 2 3 4 5 6 7 8 9   a b c d e f g h
    punpckhwd            m1, m0, m0
    paddw                m2, [tlq+28]           ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m0, [tlq+30]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    vpblendd             m3, m1, [tlq+36], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
    vpblendd             m1, [tlq+34], 0x7f     ; 3 4 5 6 7 8 9 a   b c d e f g h h
    pavgw                m2, m3
    paddw                m0, m1
    paddw                m0, m2
    mov                 tlq, rsp
    psrlw                m0, 2
    mova            [r3+32], m0
.w64_main:
    movd                xm4, dxd
    vpbroadcastw         m6, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m4, xm4
    movd                xm0, maxbased
    mov                 r5d, dxd
    vpbroadcastd         m7, [pw_m1024] ; -16 * 64
    vpbroadcastw         m0, xm0
    paddw                m3, m4, [z_base_inc]
    paddw                m8, m7, m7     ; -32 * 64
    psubw                m3, m0
    paddw                m9, m8, m7     ; -48 * 64
.w64_loop:
    mov                 r3d, r5d
    shr                 r3d, 6
    movu                 m0, [tlq+r3*2]
    movu                 m1, [tlq+r3*2+2]
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    psraw                m1, m3, 15
    vpblendvb            m0, m6, m0, m1
    mova        [dstq+32*0], m0
    movu                 m0, [tlq+r3*2+32]
    movu                 m1, [tlq+r3*2+34]
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    pcmpgtw              m1, m7, m3
    vpblendvb            m0, m6, m0, m1
    mova        [dstq+32*1], m0
    movu                 m0, [tlq+r3*2+64]
    movu                 m1, [tlq+r3*2+66]
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    pcmpgtw              m1, m8, m3
    vpblendvb            m0, m6, m0, m1
    mova        [dstq+32*2], m0
    movu                 m0, [tlq+r3*2+96]
    movu                 m1, [tlq+r3*2+98]
    add                 r5d, dxd
    psubw                m1, m0
    pmulhrsw             m1, m2
    pcmpgtw              m2, m9, m3
    paddw                m3, m4
    paddw                m0, m1
    vpblendvb            m0, m6, m0, m2
    mova        [dstq+32*3], m0
    dec                  hd
    jz .w64_end
    add                dstq, strideq
    cmp                 r5d, maxbased
    jb .w64_loop
.w64_end_loop:
    mova        [dstq+32*0], m6
    mova        [dstq+32*1], m6
    mova        [dstq+32*2], m6
    mova        [dstq+32*3], m6
    add                dstq, strideq
    dec                  hd
    jg .w64_end_loop
.w64_end:
    RET

cglobal ipred_z2_16bpc, 3, 12, 12, 352, dst, stride, tl, w, h, angle, dx, dy
%define base r9-z_filter_t0
    lea                  r9, [ipred_z2_16bpc_avx2_table]
    tzcnt                wd, wm
    movifnidn        angled, anglem
    movifnidn            hd, hm
    lea                 dxq, [dr_intra_derivative-90]
    movsxd               wq, [r9+wq*4]
    mova                 m1, [tlq-  0]
    movzx               dyd, angleb
    xor              angled, 0x400
    mova                 m2, [tlq- 32]
    mov                  r8, dxq
    sub                 dxq, dyq
    mova                 m3, [tlq- 64]
    add                  wq, r9
    add                  r9, z_filter_t0-ipred_z2_16bpc_avx2_table
    mova                 m4, [tlq- 96]
    and                 dyd, ~1
    mova                 m5, [tlq-128]
    and                 dxq, ~1
    movzx               dyd, word [r8+dyq]  ; angle - 90
    movzx               dxd, word [dxq+270] ; 180 - angle
    vpbroadcastd        m11, [base+pw_62]
    mova          [rsp+128], m1
    mova          [rsp+ 96], m2
    mova          [rsp+ 64], m3
    neg                 dxd
    mova          [rsp+ 32], m4
    neg                 dyq
    mova          [rsp+  0], m5
    jmp                  wq
.w4:
    vbroadcasti128      m10, [base+z2_x_shuf]
    vpbroadcastq         m6, [base+z_base_inc+2]
    lea                 r8d, [dxq+(65<<6)] ; xpos
    mov                r10d, (63-4)<<6
    test             angled, 0x400
    jnz .w4_main ; !enable_intra_edge_filter
    lea                 r3d, [hq+2]
    add              angled, 1022
    shl                 r3d, 6
    test                r3d, angled
    jnz .w4_no_upsample_above ; angle >= 130 || h > 8 || (is_sm && h == 8)
    movq                xm0, [tlq+2]    ; 1 2 3 4
    movq                xm1, [tlq+0]    ; 0 1 2 3
    pshuflw             xm2, xm0, q3321 ; 2 3 4 4
    pshuflw             xm3, xm1, q2100 ; 0 0 1 2
    vpbroadcastw        xm4, r8m        ; pixel_max
    vbroadcasti128      m10, [base+z_upsample]
    paddw               xm1, xm0
    paddw               xm2, xm3
    lea                 r8d, [r8+dxq+(1<<6)]
    psubw               xm2, xm1, xm2
    add                 dxd, dxd
    psraw               xm2, 3
    pxor                xm3, xm3
    sub                r10d, 3<<6
    paddw               xm1, xm2
    paddw                m6, m6
    pmaxsw              xm1, xm3
    sub              angled, 1075 ; angle - 53
    pavgw               xm1, xm3
    lea                 r3d, [hq+3]
    pminsw              xm1, xm4
    xor              angled, 0x7f ; 180 - angle
    punpcklwd           xm1, xm0
    movu          [rsp+130], xm1
    call .filter_strength
    jmp .w4_filter_left
ALIGN function_align
.filter_strength:
    movd                xm8, r3d
    mov                 r3d, angled
    movd                xm7, angled
    vpbroadcastb         m8, xm8
    shr                 r3d, 8 ; is_sm << 1
    vpbroadcastb         m7, xm7
    pcmpeqb              m8, [base+z_filter_wh]
    mova                xm9, [r9+r3*8]
    pand                 m0, m8, m7
    pcmpgtb              m0, m9
    pmovmskb            r3d, m0
    ret
ALIGN function_align
.upsample_left: ; h4/h8
    mova                xm0, [tlq-16]            ; 8 7 6 5 4 3 2 1
    movu                xm1, [tlq-14]            ; 7 6 5 4 3 2 1 0
    vpbroadcastw        xm4, r8m ; pixel_max
    cmp                  hd, 8
    je .upsample_left_h8
    pshufhw             xm2, xm0, q2100          ; _ _ _ _ 4 4 3 2
    pshufhw             xm3, xm1, q3321          ; _ _ _ _ 2 1 0 0
    jmp .upsample_left_end
.upsample_left_h8:
    pblendw             xm2, xm0, [tlq-18], 0xfe ; 8 8 7 6 5 4 3 2
    pblendw             xm3, xm1, [tlq-12], 0x7f ; 6 5 4 3 2 1 0 0
.upsample_left_end:
    paddw               xm1, xm0
    paddw               xm2, xm3
    psubw               xm2, xm1, xm2
    add                 dyq, dyq
    psraw               xm2, 3
    pxor                xm3, xm3
    paddw               xm1, xm2
    pmaxsw              xm1, xm3
    pavgw               xm1, xm3
    pminsw              xm1, xm4
    punpcklwd           xm2, xm0, xm1
    punpckhwd           xm0, xm1
    mova  [rsp+ 96+gprsize], xm2
    mova  [rsp+112+gprsize], xm0
    ret
.w4_no_upsample_above:
    lea                 r3d, [hq+3]
    sub              angled, 1112 ; angle - 90
    call .filter_strength
    test                r3d, r3d
    jz .w4_no_filter_above
    popcnt              r3d, r3d
    vpbroadcastd        xm4, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd        xm5, [base+z_filter_k-4+r3*4+12*0]
    psrldq              xm0, xm1, 2     ; 1 2 3 4
    pshuflw             xm2, xm1, q2100 ; 0 0 1 2
    pmullw              xm4, xm0
    pshuflw             xm3, xm0, q3321 ; 2 3 4 4
    paddw               xm1, xm3
    pshuflw             xm3, xm0, q3332 ; 3 4 4 4
    pmullw              xm1, xm5
    vpbroadcastd        xm5, [base+z_filter_k-4+r3*4+12*2]
    paddw               xm2, xm3
    vpbroadcastd        xm3, r6m ; max_width
    pmullw              xm2, xm5
    packssdw            xm3, xm3
    paddw               xm1, xm4
    paddw               xm1, xm2
    psubw               xm3, [base+pw_1to16]
    pxor                xm4, xm4
    psrlw               xm1, 3
    pminsw              xm3, xm11 ; clip to byte range since there's no variable word blend
    pavgw               xm1, xm4
    vpblendvb           xm1, xm0, xm3
    movq          [rsp+130], xm1
.w4_no_filter_above:
    lea                 r3d, [hq+2]
    add              angled, 973 ; angle + 883
    shl                 r3d, 6
    test                r3d, angled
    jz .w4_upsample_left ; angle <= 140 || h > 8 || (is_sm && h == 8)
    vpbroadcastd        xm0, [base+pb_90]
    psubb               xm0, xm7 ; 180 - angle
    pand                xm0, xm8 ; reuse from previous filter_strength call
    pcmpgtb             xm0, xm9
    pmovmskb            r3d, xm0
.w4_filter_left:
    test                r3d, r3d
    jz .w4_main
    popcnt              r3d, r3d
    mova                 m0, [tlq-32]  ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    vpbroadcastd         m5, r7m ; max_height
    cmp                 r3d, 3
    je .w4_filter_left_s3
    vpbroadcastd         m2, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd         m3, [base+z_filter_k-4+r3*4+12*0]
    pmullw               m2, m0
    cmp                  hd, 8
    jl .w4_filter_left_h4
    movu                 m4, [tlq-34]
    punpcklwd            m1, m0, m0
    vpblendd             m1, m4, 0xee  ; 0 0 1 2 3 4 5 6   8 8 9 a b c d e
    je .w4_filter_left_end
    vpblendd             m1, m4, 0x10  ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    jmp .w4_filter_left_end
.w4_upsample_left:
    call .upsample_left
    mov                 r11, -16
    vbroadcasti128       m9, [base+z_upsample]
    jmp .w4_main_upsample_left
.w4_filter_left_s3: ; can only be h16
    movu                 m2, [tlq-30]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    vpbroadcastd         m4, [base+pw_3]
    paddw                m1, m0, m2
    punpckhwd            m2, m2
    vpblendd             m2, [tlq-28], 0x7f     ; 2 3 4 5 6 7 8 9   a b c d e f g g
    punpcklwd           xm3, xm0, xm0
    paddw                m2, m4
    vpblendd             m4, m3, [tlq-34], 0xfe ; 0 0 1 2 3 4 5 6   8 8 9 a b c d e
    vpblendd             m3, [tlq-36], 0xfe     ; 0 0 0 1 2 3 4 5   6 8 8 9 a b c d
    paddw                m1, m4
    pavgw                m2, m3
    paddw                m1, m2
    psrlw                m1, 2
    jmp .w4_filter_left_end2
.w4_filter_left_h4:
    pshufhw              m1, m0, q2100 ; _ _ _ _ _ _ _ _   _ _ _ _ c c d e
.w4_filter_left_end:
    paddw                m1, [tlq-30]  ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pmullw               m1, m3
    paddw                m1, m2
    pxor                 m2, m2
    psrlw                m1, 3
    pavgw                m1, m2
.w4_filter_left_end2:
    packssdw             m5, m5
    psubw                m5, [base+pw_16to1]
    pminsw               m5, m11
    vpblendvb            m1, m0, m5
    mova           [rsp+96], m1
.w4_main:
    vbroadcasti128       m9, [base+z2_x_shuf]
    mov                 r11, -8
.w4_main_upsample_left:
    movd                xm5, dyd
    mova                 m4, [base+z2_y_shuf_h4]
    mov                 r2d, r8d
    movd                xm0, dxd
    vpbroadcastw         m5, xm5
    rorx                 r5, dyq, 5
    lea                 r8d, [dyq*3]
    pmullw               m5, [base+z2_ymul]
    rorx                 r9, dyq, 4
    sar                 dyd, 6
    vpbroadcastw         m0, xm0
    sar                 r8d, 6
    pand                 m5, m11       ; frac_y
    neg                 dyd
    psllw                m5, 9
    add                 r5d, dyd
    add                 r8d, dyd
    add                 r9d, dyd
    paddw                m7, m0, m0
    lea                 dyq, [rsp+dyq*2+126]
    vpblendd             m0, m7, 0xcc
    add                 dyq, r11
    neg                 r5d
    paddw                m1, m0, m7
    neg                 r8d
    vpblendd             m0, m1, 0xf0  ; xpos0 xpos1 xpos2 xpos3
    neg                 r9d
    paddw                m7, m7
    paddw                m6, m0
.w4_loop:
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6         ; base_x0
    movu                xm1, [rsp+r2*2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6         ; base_x1
    movu                xm3, [rsp+r3*2]
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6         ; base_x2
    vinserti128          m1, [rsp+r2*2], 1
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6         ; base_x3
    vinserti128          m3, [rsp+r3*2], 1
    pshufb               m1, m10 ; a0 a1 a2 a3 A0 A1 A2 A3
    pshufb               m3, m10 ; b0 b1 b2 b3 B0 B1 B2 B3
    pand                 m2, m11, m6
    punpcklqdq           m0, m1, m3
    punpckhqdq           m1, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    cmp                 r3d, 64
    jge .w4_toponly
    movu                xm2, [dyq]
    vinserti128          m2, [dyq+r8*2], 1
    movu                xm3, [dyq+r5*2]
    vinserti128          m3, [dyq+r9*2], 1
    pshufb               m2, m9
    pshufb               m3, m9
    punpckhwd            m1, m2, m3 ; a3 b3 a2 b2 a1 b1 a0 b0
    punpcklwd            m2, m3
    psubw                m2, m1
    pmulhrsw             m2, m5
    psraw                m3, m6, 15 ; base_x < topleft
    paddw                m1, m2
    vpermd               m1, m4, m1 ; a0 b0 c0 d0 a1 b1 c1 d1   a2 b2 c2 d2 a3 b3 c3 d3
    vpblendvb            m0, m1, m3
.w4_toponly:
    paddw                m6, m7     ; xpos += dx
    lea                  r3, [strideq*3]
    add                 dyq, r11
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+r3       ], xm1
    sub                  hd, 4
    jz .w4_end
    lea                dstq, [dstq+strideq*4]
    cmp                 r2d, r10d
    jge .w4_loop
.w4_leftonly_loop:
    movu                xm1, [dyq]
    vinserti128          m1, [dyq+r8*2], 1
    movu                xm2, [dyq+r5*2]
    vinserti128          m2, [dyq+r9*2], 1
    add                 dyq, r11
    pshufb               m1, m9
    pshufb               m2, m9
    punpckhwd            m0, m1, m2
    punpcklwd            m1, m2
    psubw                m1, m0
    pmulhrsw             m1, m5
    paddw                m0, m1
    vpermd               m0, m4, m0
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+r3       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_leftonly_loop
.w4_end:
    RET
.w8:
    mov                r10d, hd
    test             angled, 0x400
    jnz .w8_main
    lea                 r3d, [angleq+126]
    xor                 r8d, r8d
    mov                 r3b, hb
    cmp                 r3d, 8
    ja .w8_no_upsample_above ; angle >= 130 || h > 8 || is_sm
    movu                xm0, [tlq+2]            ; 1 2 3 4 5 6 7 8
    mova                xm1, [tlq+0]            ; 0 1 2 3 4 5 6 7
    pblendw             xm2, xm0, [tlq+4], 0x7f ; 2 3 4 5 6 7 8 8
    pblendw             xm3, xm1, [tlq-2], 0xfe ; 0 0 1 2 3 4 5 6
    vpbroadcastw        xm4, r8m ; pixel_max
    paddw               xm1, xm0
    paddw               xm2, xm3
    not                 r8d
    psubw               xm2, xm1, xm2
    add                 dxd, dxd
    psraw               xm2, 3
    sub              angled, 53 ; angle - 53
    pxor                xm3, xm3
    paddw               xm2, xm1
    lea                 r3d, [hq+7]
    pmaxsw              xm2, xm3
    xor              angled, 0x7f ; 180 - angle
    pavgw               xm2, xm3
    pminsw              xm2, xm4
    punpcklwd           xm1, xm2, xm0
    punpckhwd           xm2, xm0
    movu          [rsp+130], xm1
    movu          [rsp+146], xm2
    call .filter_strength
    jmp .w8_filter_left
.w8_no_upsample_above:
    lea                 r3d, [hq+7]
    sub              angled, 90 ; angle - 90
    call .filter_strength
    test                r3d, r3d
    jz .w8_no_filter_above
    popcnt              r3d, r3d
    vpbroadcastd        xm4, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd        xm5, [base+z_filter_k-4+r3*4+12*0]
    vpbroadcastd        xm6, [base+z_filter_k-4+r3*4+12*2]
    movu                xm0, [tlq+2]            ; 1 2 3 4 5 6 7 8 x
    pblendw             xm2, xm1, [tlq-2], 0xfe ; 0 0 1 2 3 4 5 6 x
    pmullw              xm4, xm0
    pblendw             xm3, xm0, [tlq+4], 0x7f ; 2 3 4 5 6 7 8 8 x
    paddw               xm1, xm3
    vpblendd            xm3, [tlq+6], 0x07      ; 3 4 5 6 7 8 8 8 x
    paddw               xm2, xm3
    vpbroadcastd        xm3, r6m ; max_width
    pmullw              xm1, xm5
    pmullw              xm2, xm6
    packssdw            xm3, xm3
    paddw               xm1, xm4
    paddw               xm1, xm2
    psubw               xm3, [base+pw_1to16]
    pxor                xm4, xm4
    psrlw               xm1, 3
    pminsw              xm3, xm11
    pavgw               xm1, xm4
    vpblendvb           xm1, xm0, xm3
    movu          [rsp+130], xm1
.w8_no_filter_above:
    lea                 r3d, [angleq-51]
    mov                 r3b, hb
    cmp                 r3d, 8
    jbe .w8_upsample_left ; angle > 140 && h <= 8 && !is_sm
    vpbroadcastd         m0, [base+pb_90]
    psubb                m0, m7
    pand                 m0, m8
    pcmpgtb              m0, m9
    pmovmskb            r3d, m0
.w8_filter_left:
    test                r3d, r3d
    jz .w8_main
    popcnt              r3d, r3d
    cmp                 r3d, 3
    jne .w8_filter_left_s12
    vpbroadcastd         m6, [base+pw_3]
    vpbroadcastd         m7, [base+pw_16]
    cmp                  hd, 16 ; flags needed for later
    jmp .filter_left_s3b
.w8_upsample_left:
    call .upsample_left
    vbroadcasti128       m7, [base+z2_y_shuf_us]
    lea                 r11, [rsp+118]
    mov                  r8, -8
    jmp .w8_main_upsample_left
.w16_filter_left_s12:
    xor                 r8d, r8d
.w8_filter_left_s12:
    mova                 m0, [tlq-32]  ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    vpbroadcastd         m5, r7m ; max_height
    vpbroadcastd         m2, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd         m3, [base+z_filter_k-4+r3*4+12*0]
    pmullw               m2, m0
    cmp                  hd, 8
    jl .w8_filter_left_h4
    movu                 m4, [tlq-34]
    punpcklwd            m1, m0, m0
    vpblendd             m1, m4, 0xee  ; 0 0 1 2 3 4 5 6   8 8 9 a b c d e
    je .w8_filter_left_end
    vpblendd             m1, m4, 0x10  ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    jmp .w8_filter_left_end
.w8_filter_left_h4:
    pshufhw              m1, m0, q2100 ; _ _ _ _ _ _ _ _   _ _ _ _ c c d e
.w8_filter_left_end:
    paddw                m1, [tlq-30]  ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pmullw               m1, m3
    paddw                m1, m2
    pxor                 m2, m2
    psrlw                m1, 3
    pavgw                m1, m2
    packssdw             m5, m5
    psubw                m5, [base+pw_16to1]
    pminsw               m5, m11
    vpblendvb            m1, m0, m5
    mova           [rsp+96], m1
    test                r8d, r8d
    jz .w8_main
; upsample_main
    vbroadcasti128      m10, [base+z_upsample]
    vbroadcasti128       m7, [base+z2_y_shuf]
    lea                  r5, [rsp+120]
    movd                xm1, dyd
    vbroadcasti128       m4, [base+z_base_inc+2]
    movd                xm2, dxd
    vpbroadcastw         m1, xm1
    vpbroadcastw         m2, xm2
    mov                  r7, dstq
    paddw                m4, m4
    pmullw               m0, m1, [base+z2_ymul8]
    paddw                m5, m2, m2
    psllw               xm1, 3
    vpblendd             m2, m5, 0xf0
    lea                 r2d, [dxq+(66<<6)] ; xpos
    paddw                m4, m2
    pshufd               m6, m0, q2020
    psraw               xm0, 6
    pxor                xm1, xm1
    psubw               xm8, xm1, xm0
    pand                 m6, m11
    punpckhwd           xm9, xm8, xm1
    psllw                m6, 9
    punpcklwd           xm8, xm1
.w8_upsample_above_loop:
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6
    movu                xm1, [rsp+r2*2]
    movu                xm2, [rsp+r2*2+16]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6
    vinserti128          m1, [rsp+r3*2], 1
    vinserti128          m2, [rsp+r3*2+16], 1
    pshufb               m1, m10
    pshufb               m2, m10
    punpcklqdq           m0, m1, m2   ; a0 b0 c0 d0 e0 f0 g0 h0
    punpckhqdq           m1, m2
    pand                 m2, m11, m4
    psubw                m1, m0
    psllw                m2, 9
    pmulhrsw             m1, m2
    paddw                m0, m1
    cmp                 r3d, 64
    jge .w8_upsample_above_toponly
    mova                 m1, m5
    vpgatherdq           m3, [r5+xm9*2], m5
    mova                 m5, m1
    vpgatherdq           m2, [r5+xm8*2], m1
    pshufb               m3, m7
    pshufb               m2, m7
    punpckldq            m1, m2, m3
    punpckhdq            m2, m3
    psubw                m2, m1
    pmulhrsw             m2, m6
    paddw                m1, m2
    vpermq               m1, m1, q3120
    psraw                m2, m4, 15
    vpblendvb            m0, m1, m2
.w8_upsample_above_toponly:
    paddw                m4, m5
    sub                  r5, 4
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    sub                  hd, 2
    jz .w8_ret
    lea                dstq, [dstq+strideq*2]
    jmp .w8_upsample_above_loop
.w8_main:
    vbroadcasti128       m7, [base+z2_y_shuf]
    lea                 r11, [rsp+120]
    mov                  r8, -4
.w8_main_upsample_left:
    movd                xm1, dyd
    vbroadcasti128       m4, [base+z_base_inc+2]
    movd                xm2, dxd
    vpbroadcastw         m1, xm1
    vpbroadcastw         m2, xm2
    mov                  r7, dstq
    pmullw               m0, m1, [base+z2_ymul8]
    paddw                m5, m2, m2
    psllw               xm1, 3
    vpblendd             m2, m5, 0xf0 ; xpos0 xpos1
    lea                 r9d, [dxq+(65<<6)] ; xpos
    paddw                m4, m2
    movd          [rsp+284], xm1
.w8_loop0:
    mov                 r2d, r9d
    mova          [rsp+288], m0
    mov                  r5, r11
    mova          [rsp+320], m4
    pshufd               m6, m0, q2020
    psraw               xm0, 6
    pxor                xm1, xm1
    psubw               xm8, xm1, xm0 ; base_y
    pand                 m6, m11      ; frac_y
    punpckhwd           xm9, xm8, xm1 ; base_y 2 3 6 7
    psllw                m6, 9
    punpcklwd           xm8, xm1      ; base_y 0 1 4 5
.w8_loop:
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6        ; base_x0
    movu                xm0, [rsp+r2*2]
    movu                xm1, [rsp+r2*2+2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6        ; base_x1
    vinserti128          m0, [rsp+r3*2], 1
    vinserti128          m1, [rsp+r3*2+2], 1
    pand                 m2, m11, m4
    psubw                m1, m0
    psllw                m2, 9
    pmulhrsw             m1, m2
    paddw                m0, m1
    cmp                 r3d, 64
    jge .w8_toponly
    mova                 m1, m5
    vpgatherdq           m3, [r5+xm9*2], m5
    mova                 m5, m1
    vpgatherdq           m2, [r5+xm8*2], m1
    pshufb               m3, m7       ; c0 d0 c1 d1               g0 h0 g1 h1
    pshufb               m2, m7       ; a0 b0 a1 b1               e0 f0 e1 f1
    punpckldq            m1, m2, m3   ; a0 b0 c0 d0 a1 b1 c1 d1   e0 f0 g0 h0 e1 f1 g1 h1
    punpckhdq            m2, m3
    psubw                m2, m1
    pmulhrsw             m2, m6
    paddw                m1, m2
    vpermq               m1, m1, q3120
    psraw                m2, m4, 15   ; base_x < topleft
    vpblendvb            m0, m1, m2
.w8_toponly:
    paddw                m4, m5       ; xpos += dx
    add                  r5, r8
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    sub                  hd, 2
    jz .w8_end
    lea                dstq, [dstq+strideq*2]
    cmp                 r2d, (63-8)<<6
    jge .w8_loop
.w8_leftonly_loop:
    mova                 m0, m5
    vpgatherdq           m4, [r5+xm9*2], m5
    mova                 m5, m0
    vpgatherdq           m3, [r5+xm8*2], m0
    add                  r5, r8
    pshufb               m2, m4, m7
    pshufb               m1, m3, m7
    punpckldq            m0, m1, m2
    punpckhdq            m1, m2
    psubw                m1, m0
    pmulhrsw             m1, m6
    paddw                m0, m1
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_leftonly_loop
.w8_end:
    sub                r10d, 1<<8
    jl .w8_ret
    vpbroadcastd         m0, [rsp+284]
    add                  r7, 16
    paddw                m0, [rsp+288] ; base_y += 8*dy
    add                 r9d, 8<<6
    vpbroadcastd         m4, [pw_512]
    movzx                hd, r10b
    paddw                m4, [rsp+320] ; base_x += 8*64
    mov                dstq, r7
    jmp .w8_loop0
.w8_ret:
    RET
.w16:
    movd                xm0, [tlq+32]
    lea                r10d, [hq+(1<<8)]
    movd          [rsp+160], xm0
    test             angled, 0x400
    jnz .w8_main
    lea                 r3d, [hq+15]
    sub              angled, 90
    call .filter_strength
    test                r3d, r3d
    jz .w16_no_filter_above
    popcnt              r3d, r3d
    vpbroadcastd         m4, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd         m5, [base+z_filter_k-4+r3*4+12*0]
    vpbroadcastd         m6, [base+z_filter_k-4+r3*4+12*2]
    movu                 m0, [tlq+2]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    punpcklwd           xm2, xm1, xm1
    vpblendd             m2, [tlq-2], 0xfe     ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    punpckhwd            m3, m0, m0
    pmullw               m4, m0
    vpblendd             m3, [tlq+4], 0x7f     ; 2 3 4 5 6 7 8 9   a b c d e f g g
    paddw                m1, m3
    vpblendd             m3, [tlq+6], 0x7f     ; 3 4 5 6 7 8 9 a   b c d e f g g g
    paddw                m2, m3
    vpbroadcastd         m3, r6m ; max_width
    pmullw               m1, m5
    pmullw               m2, m6
    packssdw             m3, m3
    paddw                m1, m4
    paddw                m1, m2
    psubw                m3, [base+pw_1to16]
    pxor                 m4, m4
    psrlw                m1, 3
    pminsw               m3, m11
    pavgw                m1, m4
    vpblendvb            m1, m0, m3
    movu          [rsp+130], m1
.w16_no_filter_above:
    vpbroadcastd         m0, [base+pb_90]
    psubb                m0, m7
    pand                 m0, m8
    pcmpgtb              m0, m9
    pmovmskb            r3d, m0
    test                r3d, r3d
    jz .w8_main
    popcnt              r3d, r3d
    cmp                 r3d, 3
    jne .w16_filter_left_s12
    vpbroadcastd         m6, [base+pw_3]
    vpbroadcastd         m7, [base+pw_16]
    cmp                  hd, 4
    jne .filter_left_s3
    movq                xm0, [tlq-8]    ; 0 1 2 3
    movq                xm1, [tlq-6]    ; 1 2 3 4
    vpbroadcastd        xm5, r7m ; max_height
    movq                xm4, [base+pw_16to1+24] ; 4to1
    pshuflw             xm2, xm0, q2100 ; 0 0 1 2
    pshuflw             xm3, xm1, q3321 ; 2 3 4 4
    paddw               xm1, xm0
    paddw               xm1, xm2
    pshuflw             xm2, xm0, q1000 ; 0 0 0 1
    paddw               xm3, xm6
    packssdw            xm5, xm5
    pavgw               xm2, xm3
    psubw               xm5, xm4
    paddw               xm1, xm2
    pminsw              xm5, xm11
    psrlw               xm1, 2
    vpblendvb           xm1, xm0, xm5
    movq          [rsp+120], xm1
    jmp .w8_main
.w32:
    mova                 m2, [tlq+32]
    movd                xm0, [tlq+64]
    lea                r10d, [hq+(3<<8)]
    mova          [rsp+160], m2
    movd          [rsp+192], xm0
    test             angled, 0x400
    jnz .w8_main
    vpbroadcastd         m6, [base+pw_3]
    vpbroadcastd         m0, r6m ; max_width
    vpbroadcastd         m7, [base+pw_16]
    mov                 r3d, 32
    packssdw             m0, m0
    psubw                m0, [base+pw_1to16]
    pminsw               m8, m0, m11
    psubw                m9, m8, m7
.w32_filter_above:
    movu                 m0, [tlq+2]
    punpcklwd           xm4, xm1, xm1
    paddw                m2, m6, [tlq+6]
    paddw                m1, m0
    vpblendd             m4, [tlq-2], 0xfe        ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    paddw                m1, [tlq+4]
    movu                 m3, [tlq+r3+2]
    paddw                m5, m6, [tlq+r3-2]
    pavgw                m2, m4
    punpckhwd            m4, m3, m3
    paddw                m1, m2
    vpblendd             m2, m4, [tlq+r3+6], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
    vpblendd             m4, [tlq+r3+4], 0x7f     ; 3 4 5 6 7 8 9 a   b c d e f g h h
    pavgw                m2, m5
    paddw                m5, m3, [tlq+r3]
    paddw                m4, m5
    psrlw                m1, 2
    paddw                m2, m4
    vpblendvb            m1, m0, m8
    psrlw                m2, 2
    vpblendvb            m2, m3, m9
    movu          [rsp+130], m1
    movu       [rsp+r3+130], m2
.filter_left_s3:
    cmp                  hd, 16
    jl .filter_left_s3_h8 ; h8
.filter_left_s3b:
    mova                 m0, [tlq-32]       ; 2 3 4 5 6 7 8 9   a b c d e f g h
    movu                 m2, [tlq-30]       ; 3 4 5 6 7 8 9 a   b c d e f g h i
    vpbroadcastd         m5, r7m ; max_height
    paddw                m1, m0, m2
    punpckhwd            m2, m2
    mov                 r3d, hd
    vpblendd             m2, [tlq-28], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
    packssdw             m5, m5
    not                  r3
    psubw                m5, [base+pw_16to1]
    paddw                m2, m6
    pminsw               m8, m11, m5
    je .filter_left_s3_end ; h16
    paddw                m1, [tlq-34]       ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pavgw                m2, [tlq-36]       ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m1, m2
    psrlw                m1, 2
    vpblendvb            m3, m1, m0, m8
    mova                 m0, [tlq-64]       ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m1, m0, [tlq-62]   ; 3 4 5 6 7 8 9 a   b c d e f g h i
    paddw                m2, m6, [tlq-60]   ; 4 5 6 7 8 9 a b   c d e f g h i j
    psubw                m8, m7
    mova           [rsp+96], m3
    jnp .filter_left_s3_end ; h32
    mova                 m5, [tlq-96]
    paddw                m1, [tlq-66]
    pavgw                m2, [tlq-68]
    paddw                m1, m2
    paddw                m4, m5, [tlq-94]
    paddw                m2, m6, [tlq-92]
    psrlw                m1, 2
    paddw                m4, [tlq- 98]
    pavgw                m2, [tlq-100]
    vpblendvb            m3, m1, m0, m8
    mova                 m0, [tlq-128]
    psubw                m8, m7
    paddw                m4, m2
    paddw                m1, m0, [tlq-126]
    paddw                m2, m6, [tlq-124]
    psrlw                m4, 2
    mova           [rsp+64], m3
    vpblendvb            m4, m5, m8
    psubw                m8, m7
    mova           [rsp+32], m4
.filter_left_s3_end:
    punpcklwd           xm3, xm0, xm0
    vpblendd             m4, m3, [tlq+r3*2], 0xfe ; 2 2 3 4 5 6 7 8   9 a b c d e f g
    vpblendd             m3, [tlq+r3*2-2], 0xfe   ; 2 2 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m1, m4
    pavgw                m2, m3
    paddw                m1, m2
    psrlw                m1, 2
    vpblendvb            m1, m0, m8
    mova     [rsp+r3*2+130], m1
    jmp .w8_main
.filter_left_s3_h8:
    mova                xm0, [tlq-16]            ; 0 1 2 3 4 5 6 7
    movu                xm3, [tlq-14]            ; 1 2 3 4 5 6 7 8
    pblendw             xm2, xm0, [tlq-18], 0xfe ; 0 0 1 2 3 4 5 6
    vpbroadcastd        xm5, r7m ; max_height
    paddw               xm1, xm0, xm3
    pblendw             xm3, [tlq-12], 0x7f      ; 2 3 4 5 6 7 8 8
    paddw               xm1, xm2
    vpblendd            xm2, [tlq-20], 0x0e      ; 0 0 0 1 2 3 4 5
    paddw               xm3, xm6
    packssdw            xm5, xm5
    pavgw               xm2, xm3
    psubw               xm5, [base+pw_16to1+16] ; 8to1
    paddw               xm1, xm2
    pminsw              xm5, xm11
    psrlw               xm1, 2
    vpblendvb           xm1, xm0, xm5
    mova          [rsp+112], xm1
    jmp .w8_main
.w64:
    mova                 m2, [tlq+ 32]
    mova                 m3, [tlq+ 64]
    mova                 m4, [tlq+ 96]
    movd                xm0, [tlq+128]
    lea                r10d, [hq+(7<<8)]
    mova          [rsp+160], m2
    mova          [rsp+192], m3
    mova          [rsp+224], m4
    movd          [rsp+256], xm0
    test             angled, 0x400
    jnz .w8_main
    vpbroadcastd         m6, [base+pw_3]
    movu                 m0, [tlq+34]     ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m2, m6, [tlq+30] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m5, m0, [tlq+32] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pavgw                m2, [tlq+38]     ; 4 5 6 7 8 9 a b   c d e f g h h h
    paddw                m5, [tlq+36]     ; 3 4 5 6 7 8 9 a   b c d e f g h h
    movu                 m4, [tlq+66]
    paddw                m3, m6, [tlq+62]
    paddw                m7, m4, [tlq+64]
    pavgw                m3, [tlq+70]
    paddw                m7, [tlq+68]
    paddw                m2, m5
    vpbroadcastd         m5, r6m ; max_width
    mov                 r3d, 96
    packssdw             m5, m5
    paddw                m3, m7
    psubw                m5, [base+pw_1to16]
    psrlw                m2, 2
    vpbroadcastd         m7, [base+pw_16]
    psrlw                m3, 2
    pminsw               m8, m11, m5
    psubw                m9, m8, m7
    vpblendvb            m2, m0, m9
    psubw                m9, m7
    vpblendvb            m3, m4, m9
    psubw                m9, m7
    movu          [rsp+162], m2
    movu          [rsp+194], m3
    jmp .w32_filter_above

cglobal ipred_z3_16bpc, 4, 9, 0, dst, stride, tl, w, h, angle, dy, org_w, maxbase
    %assign org_stack_offset stack_offset
    lea                  r6, [ipred_z3_16bpc_avx2_table]
    tzcnt                hd, hm
    movifnidn        angled, anglem
    lea                  r7, [dr_intra_derivative+45*2-1]
    sub                 tlq, 2
    movsxd               hq, [r6+hq*4]
    sub              angled, 180
    add                  hq, r6
    mov                 dyd, angled
    neg                 dyd
    xor              angled, 0x400
    or                  dyq, ~0x7e
    movzx               dyd, word [r7+dyq]
    vpbroadcastd         m5, [pw_62]
    mov              org_wd, wd
    jmp                  hq
.h4:
    ALLOC_STACK         -64, 7
    lea                  r7, [strideq*3]
    cmp              angleb, 40
    jae .h4_no_upsample
    lea                 r4d, [angleq-1024]
    sar                 r4d, 7
    add                 r4d, wd
    jg .h4_no_upsample ; !enable_intra_edge_filter || w > 8 || (w == 8 && is_sm)
    mova                xm2, [tlq-14]            ; 0 1 2 3 4 5 6 7
    pblendw             xm1, xm2, [tlq-16], 0xfe ; 0 0 1 2 3 4 5 6
    vpblendd            xm0, xm1, [tlq-18], 0x0e ; 0 0 0 1 2 3 4 5
    pshufd              xm3, xm1, q0000
    paddw               xm1, xm2
    paddw               xm0, [tlq-12]            ; 1 2 3 4 5 6 7 8
    vpbroadcastw        xm4, r8m ; pixel_max
    add                 dyd, dyd
    psubw               xm0, xm1, xm0
    mova           [rsp+ 0], xm3
    movd                xm3, dyd
    psraw               xm0, 3
    neg                 dyd
    paddw               xm1, xm0
    pxor                xm0, xm0
    lea                 r2d, [dyq+(16<<6)+63] ; ypos
    pmaxsw              xm1, xm0
    pavgw               xm1, xm0
    vpbroadcastw         m3, xm3
    pminsw              xm1, xm4
    punpckhwd           xm0, xm1, xm2
    punpcklwd           xm1, xm2
    paddw                m2, m3, m3
    mova           [rsp+32], xm0
    punpcklwd            m3, m2
    mova           [rsp+16], xm1
    paddw                m4, m2, m2
    paddw                m2, m3
    vpblendd             m3, m2, 0xf0 ; ypos0 ypos1   ypos2 ypos3
.h4_upsample_loop:
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6
    movu                xm1, [rsp+r2*2]
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6
    movu                xm2, [rsp+r4*2]
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6
    vinserti128          m1, [rsp+r2*2], 1
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6
    vinserti128          m2, [rsp+r4*2], 1
    psrld                m0, m1, 16
    pblendw              m0, m2, 0xaa ; a3 b3 a2 b2 a1 b1 a0 b0   c3 d3 c2 d2 c1 d1 c0 d0
    pslld                m2, 16
    pblendw              m1, m2, 0xaa
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m3, m4
    paddw                m1, m0
    vextracti128        xm2, m1, 1
    punpckhdq           xm0, xm1, xm2 ; a1 b1 c1 d1 a0 b0 c0 d0
    punpckldq           xm1, xm2      ; a3 b3 c3 d3 a2 b2 c2 d2
    movhps [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm0
    movhps [dstq+strideq*2], xm1
    movq   [dstq+r7       ], xm1
    add                dstq, 8
    sub                  wd, 4
    jg .h4_upsample_loop
    RET
ALIGN function_align
.filter_strength: ; h4/h8/h16
%define base r4-z_filter_t0
    lea                  r4, [z_filter_t0]
    movd                xm0, maxbased
    movd                xm1, angled
    shr              angled, 8 ; is_sm << 1
    vpbroadcastb         m0, xm0
    vpbroadcastb         m1, xm1
    pcmpeqb              m0, [base+z_filter_wh]
    pand                 m0, m1
    mova                xm1, [r4+angleq*8]
    pcmpgtb              m0, m1
    pmovmskb            r5d, m0
    ret
.h4_no_upsample:
    mov            maxbased, 7
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .h4_main
    lea            maxbased, [wq+3]
    call .filter_strength
    mov            maxbased, 7
    test                r5d, r5d
    jz .h4_main ; filter_strength == 0
    popcnt              r5d, r5d
    mova                xm0, [tlq-14]       ; 0 1 2 3 4 5 6 7
    movu                xm3, [tlq-12]       ; 1 2 3 4 5 6 7 8
    vpbroadcastd        xm2, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd        xm4, [base+z_filter_k-4+r5*4+12*0]
    pmullw              xm2, xm0
    pblendw             xm0, [tlq-16], 0xfe ; 0 0 1 2 3 4 5 6
    paddw               xm1, xm0, xm3
    movd           [rsp+12], xm0
    pmullw              xm1, xm4
    cmp                 r5d, 3
    jne .h4_filter_3tap
    pblendw             xm3, [tlq-10], 0x7f ; 2 3 4 5 6 7 8 8
    vpblendd            xm0, [tlq-18], 0x0e ; 0 0 0 1 2 3 4 5
    movzx               r4d, word [tlq-14]
    movzx               r2d, word [tlq-12]
    inc            maxbased
    paddw               xm1, xm2
    paddw               xm0, xm3
    sub                 r2d, r4d
    paddw               xm2, xm0, xm0
    lea                 r2d, [r2+r4*8+4]
    shr                 r2d, 3
    mov            [rsp+14], r2w
.h4_filter_3tap:
    pxor                xm0, xm0
    paddw               xm1, xm2
    lea                 tlq, [rsp+30]
    psrlw               xm1, 3
    cmp                  wd, 8
    sbb            maxbased, -1
    pavgw               xm0, xm1
    mova           [rsp+16], xm0
.h4_main:
    movd                xm3, dyd
    neg            maxbaseq
    vbroadcasti128       m1, [z_base_inc]
    vpbroadcastw         m6, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m3, xm3
    lea                 r4d, [maxbaseq+3*64]
    neg                 dyq
    movd                xm2, r4d
    sub                 tlq, 8
    lea                  r4, [dyq+63] ; ypos
    punpcklwd            m1, m1
    paddw                m0, m3, m3
    vpbroadcastw         m2, xm2
    punpcklwd            m3, m0
    paddw                m4, m0, m0
    paddw                m0, m3
    psubw                m2, m1
    vpblendd             m3, m0, 0xf0 ; ypos0 ypos1   ypos2 ypos3
    or             maxbased, 63
    paddw                m3, m2
.h4_loop:
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base0
    movu                xm1, [tlq+r4*2]
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base1
    movu                xm2, [tlq+r5*2]
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base2
    vinserti128          m1, [tlq+r4*2], 1
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base3
    vinserti128          m2, [tlq+r5*2], 1
    punpckhwd            m0, m1, m2
    punpcklwd            m1, m2
    pand                 m2, m5, m3
    palignr              m0, m1, 4    ; a3 b3 a2 b2 a1 b1 a0 b0   c3 d3 c2 d2 c1 d1 c0 d0
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    psraw                m2, m3, 15   ; ypos < max_base_y
    paddw                m3, m4
    paddw                m1, m0
    vpblendvb            m1, m6, m1, m2
    vextracti128        xm2, m1, 1
    punpckhdq           xm0, xm1, xm2 ; a1 b1 c1 d1 a0 b0 c0 d0
    punpckldq           xm1, xm2      ; a3 b3 c3 d3 a2 b2 c2 d2
    movhps [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm0
    movhps [dstq+strideq*2], xm1
    movq   [dstq+r7       ], xm1
    sub                  wd, 4
    jz .h4_end
    add                dstq, 8
    cmp                 r4d, maxbased
    jg .h4_loop
.h4_end_loop:
    movq   [dstq+strideq*0], xm6
    movq   [dstq+strideq*1], xm6
    movq   [dstq+strideq*2], xm6
    movq   [dstq+r7       ], xm6
    add                dstq, 8
    sub                  wd, 4
    jg .h4_end_loop
.h4_end:
    RET
.h8:
    lea                 r4d, [angleq+216]
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -64, 8
    mov                 r4b, wb
    lea                  r7, [strideq*3]
    cmp                 r4d, 8
    ja .h8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || w > 8
    mova                 m2, [tlq-30]     ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw                m1, m2, [tlq-32] ; _ 0 1 2 3 4 5 6   7 8 9 a b c d e
    movu                 m0, [tlq-34]     ; _ _ 0 1 2 3 4 5   6 7 8 9 a b c d
    cmp                  wd, 8
    je .h8_upsample_w8
    pshufhw             xm3, xm2, q1000
    vpblendd             m0, m3, 0x0f     ; _ _ _ _ 4 4 4 5   6 7 8 9 a b c d
.h8_upsample_w8:
    paddw                m0, [tlq-28]     ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    vpbroadcastw         m4, r8m ; pixel_max
    add                 dyd, dyd
    psubw                m0, m1, m0
    movd                xm6, dyd
    psraw                m0, 3
    neg                 dyd
    paddw                m1, m0
    pxor                 m0, m0
    pmaxsw               m1, m0
    lea                 r4d, [dyq+(16<<6)+63] ; ypos
    pavgw                m1, m0
    vpbroadcastw         m6, xm6
    pminsw               m1, m4
    punpckhwd            m0, m1, m2
    punpcklwd            m1, m2
    vextracti128   [rsp+48], m0, 1
    vextracti128   [rsp+32], m1, 1
    paddw                m7, m6, m6
    mova           [rsp+16], xm0
    mova           [rsp+ 0], xm1
    punpcklwd            m6, m7 ; ypos0 ypos1
.h8_upsample_loop:
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6 ; base0
    movu                 m1, [rsp+r4*2]
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6 ; base1
    movu                 m2, [rsp+r2*2]
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6 ; base2
    movu                 m3, [rsp+r4*2]
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6 ; base3
    movu                 m4, [rsp+r2*2]
    psrld                m0, m1, 16
    pblendw              m0, m2, 0xaa ; a7 b7 a6 b6 a5 b5 a4 b4   a3 b3 a2 b2 a1 b1 a0 b0
    pslld                m2, 16
    pblendw              m1, m2, 0xaa
    psrld                m2, m3, 16
    pblendw              m2, m4, 0xaa ; c7 d7 c6 d6 c5 d5 c4 d4   c3 d3 c2 d2 c1 d1 c0 d0
    pslld                m4, 16
    pblendw              m3, m4, 0xaa
    pand                 m4, m5, m6
    paddw                m6, m7
    psllw                m4, 9
    psubw                m1, m0
    pmulhrsw             m1, m4
    pand                 m4, m5, m6
    psllw                m4, 9
    psubw                m3, m2
    pmulhrsw             m3, m4
    paddw                m6, m7
    lea                  r2, [dstq+strideq*4]
    paddw                m1, m0
    paddw                m3, m2
    punpckhdq            m0, m1, m3   ; a5 b5 c5 d5 a4 b4 c4 d4   a1 b1 c1 d1 a0 b0 c0 d0
    punpckldq            m1, m3       ; a7 b7 c7 d7 a6 b6 c6 d6   a3 b3 c3 d3 a2 b2 c2 d2
    vextracti128        xm2, m0, 1
    vextracti128        xm3, m1, 1
    movhps [r2  +strideq*0], xm0
    movq   [r2  +strideq*1], xm0
    movhps [r2  +strideq*2], xm1
    movq   [r2  +r7       ], xm1
    movhps [dstq+strideq*0], xm2
    movq   [dstq+strideq*1], xm2
    movhps [dstq+strideq*2], xm3
    movq   [dstq+r7       ], xm3
    add                dstq, 8
    sub                  wd, 4
    jg .h8_upsample_loop
    RET
.h8_no_intra_edge_filter:
    and            maxbased, 7
    or             maxbased, 8 ; imin(w+7, 15)
    jmp .h8_main
.h8_no_upsample:
    lea            maxbased, [wq+7]
    test             angled, 0x400
    jnz .h8_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .h8_main
    popcnt              r5d, r5d
    mova                 m0, [tlq-30]           ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    movu                 m3, [tlq-28]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    vpbroadcastd         m2, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd         m4, [base+z_filter_k-4+r5*4+12*0]
    pmullw               m2, m0
    cmp                  wd, 8
    jl .h8_filter_w4
    punpcklwd           xm0, xm0
    vpblendd             m1, m0, [tlq-32], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    movd           [rsp+28], xm0
    paddw                m1, m3
    mov                 r4d, 16
    pmullw               m1, m4
    cmovg          maxbased, r4d
    cmp                 r5d, 3
    jne .h8_filter_3tap
    punpckhwd            m3, m3
    vpblendd             m0, [tlq-34], 0xfe     ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
    vpblendd             m3, [tlq-26], 0x7f     ; 2 3 4 5 6 7 8 9   a b c d e f g g
    movzx               r4d, word [tlq-30]
    movzx               r2d, word [tlq-28]
    inc            maxbased
    paddw                m1, m2
    paddw                m0, m3
    sub                 r2d, r4d
    paddw                m2, m0, m0
    lea                 r2d, [r2+r4*8+4]
    shr                 r2d, 3
    mov            [rsp+30], r2w
    jmp .h8_filter_3tap
.h8_filter_w4:
    pshufhw             xm1, xm0, q2100
    vinserti128          m1, [tlq-16], 1        ; _ _ _ _ 4 4 5 6   7 8 9 a b c d e
    paddw                m1, m3
    pmullw               m1, m4
.h8_filter_3tap:
    pxor                 m0, m0
    paddw                m1, m2
    lea                 tlq, [rsp+62]
    psrlw                m1, 3
    pavgw                m0, m1
    mova           [rsp+32], m0
.h8_main:
    movd                xm4, dyd
    neg            maxbaseq
    vbroadcasti128       m1, [z_base_inc]
    vpbroadcastw         m7, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m4, xm4
    lea                 r4d, [maxbaseq+7*64]
    neg                 dyq
    movd                xm2, r4d
    sub                 tlq, 16
    lea                  r4, [dyq+63]
    paddw                m6, m4, m4
    vpbroadcastw         m2, xm2
    vpblendd             m4, m6, 0xf0 ; ypos0 ypos1
    psubw                m2, m1
    or             maxbased, 63
    paddw                m4, m2
.h8_loop:
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base0
    movu                xm0, [tlq+r4*2+2]
    movu                xm1, [tlq+r4*2]
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base1
    vinserti128          m0, [tlq+r5*2+2], 1
    vinserti128          m1, [tlq+r5*2], 1
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base2
    pand                 m3, m5, m4
    psllw                m3, 9
    psubw                m1, m0
    pmulhrsw             m1, m3
    psraw                m3, m4, 15
    paddw                m4, m6
    paddw                m0, m1
    movu                xm1, [tlq+r4*2+2]
    movu                xm2, [tlq+r4*2]
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base3
    vpblendvb            m0, m7, m0, m3
    vinserti128          m1, [tlq+r5*2+2], 1
    vinserti128          m2, [tlq+r5*2], 1
    pand                 m3, m5, m4
    psllw                m3, 9
    psubw                m2, m1
    pmulhrsw             m2, m3
    psraw                m3, m4, 15
    paddw                m4, m6
    lea                  r5, [dstq+strideq*4]
    paddw                m1, m2
    vpblendvb            m1, m7, m1, m3
    punpckhwd            m2, m0, m1   ; a3 c3 a2 c2 a1 c1 a0 c0   b3 d3 b2 d2 b1 d1 b0 d0
    vextracti128        xm3, m2, 1
    punpcklwd            m0, m1       ; a7 c7 a6 c6 a5 c5 a4 c5   b7 d7 b6 d6 b5 d5 b4 d4
    punpckhwd           xm1, xm2, xm3 ; a1 b1 c1 d1 a0 b0 c0 d0
    punpcklwd           xm2, xm3      ; a3 b3 c3 d3 a2 b2 c2 d2
    vextracti128        xm3, m0, 1
    movhps [dstq+strideq*0], xm1
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movq   [dstq+r7       ], xm2
    punpckhwd           xm1, xm0, xm3 ; a5 b5 c5 d5 a4 b4 c4 d4
    punpcklwd           xm0, xm3      ; a7 b7 c7 d7 a6 b6 c6 d6
    movhps [r5  +strideq*0], xm1
    movq   [r5  +strideq*1], xm1
    movhps [r5  +strideq*2], xm0
    movq   [r5  +r7       ], xm0
    sub                  wd, 4
    jz .h8_end
    add                dstq, 8
    cmp                 r4d, maxbased
    jg .h8_loop
    lea                  r6, [strideq*5]
    lea                  r2, [strideq+r7*2] ; stride*7
    test                 wd, 4
    jz .h8_end_loop
    movq   [dstq+strideq*0], xm7
    movq   [dstq+strideq*1], xm7
    movq   [dstq+strideq*2], xm7
    movq   [dstq+r7       ], xm7
    movq   [dstq+strideq*4], xm7
    movq   [dstq+r6       ], xm7
    movq   [dstq+r7*2     ], xm7
    movq   [dstq+r2       ], xm7
    add                dstq, 8
    sub                  wd, 4
    jz .h8_end
.h8_end_loop:
    mova   [dstq+strideq*0], xm7
    mova   [dstq+strideq*1], xm7
    mova   [dstq+strideq*2], xm7
    mova   [dstq+r7       ], xm7
    mova   [dstq+strideq*4], xm7
    mova   [dstq+r6       ], xm7
    mova   [dstq+r7*2     ], xm7
    mova   [dstq+r2       ], xm7
    add                dstq, 16
    sub                  wd, 8
    jg .h8_end_loop
.h8_end:
    RET
.h16_no_intra_edge_filter:
    and            maxbased, 15
    or             maxbased, 16 ; imin(w+15, 31)
    jmp .h16_main
ALIGN function_align
.h16:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -96, 10
    lea            maxbased, [wq+15]
    lea                  r7, [strideq*3]
    test             angled, 0x400
    jnz .h16_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .h16_main ; filter_strength == 0
    popcnt              r5d, r5d
    movu                 m0, [tlq-28]            ; 3 4 5 6 7 8 9 a   b c d e f g h i
    paddw                m1, m0, [tlq-32]        ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    vpbroadcastd         m6, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd         m7, [base+z_filter_k-4+r5*4+12*0]
    pmullw               m2, m6, [tlq-30]        ; 2 3 4 5 6 7 8 9   a b c d e f g h
    pmullw               m1, m7
    paddw                m1, m2
    cmp                  wd, 8
    jg .h16_filter_w16
    mova                xm3, [tlq-46]            ; 0 1 2 3 4 5 6 7
    pmullw              xm6, xm3
    jl .h16_filter_w4
    pblendw             xm3, [tlq-48], 0xfe      ; 0 0 1 2 3 4 5 6
    cmp                 r5d, 3
    jne .h16_filter_w8_3tap
    vpblendd            xm4, xm3, [tlq-50], 0x0e ; 0 0 0 1 2 3 4 5
.h16_filter_w8_5tap:
    punpckhwd            m0, m0
    vpblendd             m0, [tlq-26], 0x7f      ; 4 5 6 7 8 9 a b   c d e f g h i i
    paddw               xm4, [tlq-42]            ; 2 3 4 5 6 7 8 9
    paddw                m0, [tlq-34]            ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    paddw               xm4, xm4
    paddw                m0, m0
    paddw               xm6, xm4
    paddw                m1, m0
.h16_filter_w8_3tap:
    paddw               xm3, [tlq-44]            ; 1 2 3 4 5 6 7 8
    pmullw              xm3, xm7
    pxor                 m0, m0
    paddw               xm3, xm6
    psrlw               xm3, 3
    pavgw               xm3, xm0
    mova           [rsp+48], xm3
    jmp .h16_filter_end
.h16_filter_w4:
    pshufhw             xm3, xm3, q2100          ; _ _ _ _ 4 4 5 6
    cmp                 r5d, 3
    jne .h16_filter_w8_3tap
    pshufhw             xm4, xm3, q2100          ; _ _ _ _ 4 4 4 5
    jmp .h16_filter_w8_5tap
.h16_filter_w16:
    mova                 m3, [tlq-62]            ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    pmullw               m6, m3
    punpcklwd           xm3, xm3
    vpblendd             m4, m3, [tlq-64], 0xfe  ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    paddw                m4, [tlq-60]            ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    mov                 r4d, 32
    cmp                  wd, 16
    cmovg          maxbased, r4d
    movd           [rsp+28], xm3
    pmullw               m4, m7
    cmp                 r5d, 3
    jne .h16_filter_w16_3tap
    punpckhwd            m0, m0
    vpblendd             m3, [tlq-66], 0xfe      ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
    vpblendd             m0, [tlq-26], 0x7f      ; 4 5 6 7 8 9 a b   c d e f g h i i
    paddw                m3, [tlq-58]            ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m0, [tlq-34]            ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    movzx               r4d, word [tlq-62]
    movzx               r2d, word [tlq-60]
    or             maxbased, 1
    paddw                m3, m3
    sub                 r2d, r4d
    paddw                m0, m0
    lea                 r2d, [r2+r4*8+4]
    paddw                m4, m3
    shr                 r2d, 3
    paddw                m1, m0
    mov            [rsp+30], r2w
.h16_filter_w16_3tap:
    pxor                 m0, m0
    paddw                m4, m6
    psrlw                m4, 3
    pavgw                m4, m0
    mova           [rsp+32], m4
.h16_filter_end:
    psrlw                m1, 3
    lea                 tlq, [rsp+94]
    pavgw                m1, m0
    mova           [rsp+64], m1
.h16_main:
    movd                xm8, dyd
    neg            maxbaseq
    vpbroadcastw         m9, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m8, xm8
    lea                 r4d, [maxbaseq+dyq+15*64]
    neg                 dyq
    movd                xm7, r4d
    sub                 tlq, 32
    lea                  r4, [dyq+63]
    vpbroadcastw         m7, xm7
    or             maxbased, 63
    psubw                m7, [z_base_inc]
.h16_loop:
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base0
    movu                 m0, [tlq+r4*2+2]
    movu                 m2, [tlq+r4*2]
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base1
    movu                 m1, [tlq+r5*2+2]
    movu                 m3, [tlq+r5*2]
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base3
    pand                 m6, m5, m7
    psllw                m6, 9
    psubw                m2, m0
    pmulhrsw             m2, m6
    psraw                m6, m7, 15
    paddw                m7, m8
    paddw                m0, m2
    movu                 m2, [tlq+r4*2+2]
    movu                 m4, [tlq+r4*2]
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base3
    vpblendvb            m0, m9, m0, m6
    pand                 m6, m5, m7
    psllw                m6, 9
    psubw                m3, m1
    pmulhrsw             m3, m6
    psraw                m6, m7, 15
    paddw                m7, m8
    paddw                m1, m3
    vpblendvb            m1, m9, m1, m6
    pand                 m6, m5, m7
    psllw                m6, 9
    psubw                m4, m2
    pmulhrsw             m4, m6
    psraw                m6, m7, 15
    paddw                m7, m8
    paddw                m2, m4
    movu                 m3, [tlq+r5*2+2]
    movu                 m4, [tlq+r5*2]
    vpblendvb            m2, m9, m2, m6
    pand                 m6, m5, m7
    psllw                m6, 9
    psubw                m4, m3
    pmulhrsw             m4, m6
    psraw                m6, m7, 15
    paddw                m7, m8
    lea                  r5, [dstq+strideq*4]
    paddw                m3, m4
    vpblendvb            m3, m9, m3, m6
    punpckhwd            m4, m0, m1 ; ab bb aa ba a9 b9 a8 b8   a3 b3 a2 b2 a1 b1 a0 b0
    punpcklwd            m0, m1     ; af bf ae be ad bd ac bc   a7 b7 a6 b6 a5 b5 a4 b4
    punpckhwd            m1, m2, m3 ; cb db ca da c9 d9 c8 d8   c3 d3 c2 d2 c1 d1 c0 d0
    punpcklwd            m2, m3     ; cf df ce de cd dd cc dc   c7 d7 c6 d6 c5 d5 c4 d4
    punpckhdq            m3, m4, m1 ; a9 b9 c9 d9 a8 b8 c8 d8   a1 b1 c1 d1 a0 b0 c0 d0
    vextracti128        xm6, m3, 1
    punpckldq            m4, m1     ; ab bb cb db aa ba ca da   a3 b3 c3 d3 a2 b2 c2 d2
    punpckhdq            m1, m0, m2 ; ad bd cd dd ac bc cc dc   a5 b5 c5 d5 a4 b4 c4 d4
    punpckldq            m0, m2     ; af bf cf df ae be ce de   a7 b7 c7 d7 a6 b6 c6 d6
    vextracti128        xm2, m4, 1
    movhps [dstq+strideq*0], xm6
    movq   [dstq+strideq*1], xm6
    vextracti128        xm6, m1, 1
    movhps [dstq+strideq*2], xm2
    movq   [dstq+r7       ], xm2
    vextracti128        xm2, m0, 1
    movhps [r5  +strideq*0], xm6
    movq   [r5  +strideq*1], xm6
    movhps [r5  +strideq*2], xm2
    movq   [r5  +r7       ], xm2
    lea                  r5, [dstq+strideq*8]
    movhps [r5  +strideq*0], xm3
    movq   [r5  +strideq*1], xm3
    movhps [r5  +strideq*2], xm4
    movq   [r5  +r7       ], xm4
    lea                  r5, [r5+strideq*4]
    movhps [r5  +strideq*0], xm1
    movq   [r5  +strideq*1], xm1
    movhps [r5  +strideq*2], xm0
    movq   [r5  +r7       ], xm0
    sub                  wd, 4
    jz .h16_end
    add                dstq, 8
    cmp                 r4d, maxbased
    jg .h16_loop
    mov                  hd, 4
.h16_end_loop0:
    mov                 r6d, wd
    mov                  r2, dstq
    test                 wb, 4
    jz .h16_end_loop
    movq   [dstq+strideq*0], xm9
    movq   [dstq+strideq*1], xm9
    movq   [dstq+strideq*2], xm9
    movq   [dstq+r7       ], xm9
    and                 r6d, 120
    jz .h16_end_w4
    add                dstq, 8
.h16_end_loop:
    mova   [dstq+strideq*0], xm9
    mova   [dstq+strideq*1], xm9
    mova   [dstq+strideq*2], xm9
    mova   [dstq+r7       ], xm9
    add                dstq, 16
    sub                 r6d, 8
    jg .h16_end_loop
.h16_end_w4:
    lea                dstq, [r2+strideq*4]
    dec                  hd
    jg .h16_end_loop0
.h16_end:
    RET
.h32:
    %assign stack_offset org_stack_offset
    ALLOC_STACK        -160, 9
    lea            maxbased, [wq+31]
    and            maxbased, 31
    or             maxbased, 32 ; imin(w+31, 63)
    test             angled, 0x400
    jnz .h32_main
    vpbroadcastd         m2, [pw_3]
    movu                 m0, [tlq-28]       ; 3 4 5 6 7 8 9 a   b c d e f g h i
    punpckhwd            m1, m0, m0
    vpblendd             m1, [tlq-26], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
    paddw                m0, [tlq-30]       ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m1, m2
    paddw                m0, [tlq-32]       ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pavgw                m1, [tlq-34]       ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    lea                  r4, [rsp+128]
    paddw                m0, m1
    lea                 r5d, [maxbaseq-31]
    psrlw                m0, 2
    mova               [r4], m0
.h32_filter_loop:
    mova                 m0, [tlq-62]
    paddw                m1, m2, [tlq-66]
    paddw                m0, [tlq-64]
    pavgw                m1, [tlq-58]
    paddw                m0, [tlq-60]
    sub                 tlq, 32
    sub                  r4, 32
    paddw                m0, m1
    psrlw                m0, 2
    mova               [r4], m0
    sub                 r5d, 16
    jg .h32_filter_loop
    jl .h32_filter_h8
    mova                 m0, [tlq-62]           ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    punpcklwd           xm1, xm0, xm0
    paddw                m2, [tlq-58]           ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m0, [tlq-60]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    vpblendd             m3, m1, [tlq-66], 0xfe ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
    vpblendd             m1, [tlq-64], 0xfe     ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    movzx               r5d, word [tlq-62]
    movzx               r2d, word [tlq-60]
    pavgw                m2, m3
    sub                 r2d, r5d
    paddw                m0, m1
    lea                 r2d, [r2+r5*8+4]
    paddw                m0, m2
    shr                 r2d, 3
    psrlw                m0, 2
    mova            [r4-32], m0
    mov             [r4-36], r5w
    mov             [r4-34], r2w
    lea                 tlq, [rsp+158]
    mov                 r4d, 65
    cmp                  wd, 64
    cmove          maxbased, r4d
    jmp .h32_main
.h32_filter_h8:
    mova                xm0, [tlq-46]            ; 0 1 2 3 4 5 6 7
    pblendw             xm1, xm0, [tlq-48], 0xfe ; 0 0 1 2 3 4 5 6
    paddw               xm2, [tlq-42]            ; 2 3 4 5 6 7 8 9
    paddw               xm0, [tlq-44]            ; 1 2 3 4 5 6 7 8
    vpblendd            xm3, xm1, [tlq-50], 0x0e ; 0 0 0 1 2 3 4 5
    lea                 tlq, [rsp+158]
    pavgw               xm2, xm3
    paddw               xm0, xm1
    paddw               xm0, xm2
    psrlw               xm0, 2
    mova            [r4-16], xm0
.h32_main:
    movd                xm6, dyd
    neg            maxbaseq
    vpbroadcastw         m7, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m6, xm6
    lea                 r4d, [maxbaseq+dyq+15*64]
    neg                 dyq
    movd                xm4, r4d
    vpbroadcastd         m8, [pw_m1024]
    lea                  r4, [dyq+63]
    vpbroadcastw         m4, xm4
    or             maxbased, 63
    psubw                m4, [z_base_inc]
.h32_loop:
    mov                  r5, r4
    sar                  r5, 6
    movu                 m1, [tlq+r5*2-64]
    movu                 m0, [tlq+r5*2-62]
    pand                 m3, m5, m4
    psllw                m3, 9
    psubw                m1, m0
    pmulhrsw             m1, m3
    pcmpgtw              m2, m8, m4
    paddw                m0, m1
    vpblendvb            m0, m7, m0, m2
    movu                 m2, [tlq+r5*2-32]
    movu                 m1, [tlq+r5*2-30]
    add                  r4, dyq
    sub                 rsp, 64
    psubw                m2, m1
    pmulhrsw             m2, m3
    psraw                m3, m4, 15
    paddw                m4, m6
    mova         [rsp+32*0], m0
    paddw                m1, m2
    vpblendvb            m1, m7, m1, m3
    mova         [rsp+32*1], m1
    dec                  wd
    jz .h32_transpose
    cmp                 r4d, maxbased
    jg .h32_loop
.h32_end_loop:
    sub                 rsp, 64
    mova         [rsp+32*0], m7
    mova         [rsp+32*1], m7
    dec                  wd
    jg .h32_end_loop
.h32_transpose:
    lea                  r3, [strideq*3]
    lea                  r4, [strideq*5]
    mov                  r8, dstq
    lea                  r5, [strideq+r3*2]
.h32_transpose_loop0:
    lea                  r6, [rsp+32]
    lea                  r2, [r8+org_wq*2-16]
.h32_transpose_loop:
    mova                 m0, [r6+64*7]
    mova                 m1, [r6+64*6]
    mova                 m2, [r6+64*5]
    mova                 m3, [r6+64*4]
    mova                 m4, [r6+64*3]
    mova                 m5, [r6+64*2]
    mova                 m6, [r6+64*1]
    mova                 m7, [r6+64*0]
    punpckhwd            m8, m0, m1 ; a3 b3 a2 b2 a1 b1 a0 b0
    punpcklwd            m0, m1     ; a7 b7 a6 b6 a5 b5 a4 b4
    punpckhwd            m1, m2, m3 ; c3 d3 c2 d2 c1 d1 c0 d0
    punpcklwd            m2, m3     ; c7 d7 c6 d6 c5 d5 c4 d4
    punpckhwd            m3, m4, m5 ; e3 f3 e2 f2 e1 f1 e0 f0
    punpcklwd            m4, m5     ; e7 f7 e6 f6 e5 f5 e4 f4
    punpckhwd            m5, m6, m7 ; g3 h3 g2 h2 g1 h1 g0 h0
    punpcklwd            m6, m7     ; g7 h7 g6 h6 g5 h5 g4 h4
    lea                dstq, [r2+strideq*8]
    sub                  r6, 32
    punpckhdq            m7, m8, m1 ; a1 b1 c1 d1 a0 b0 c0 d0
    punpckldq            m8, m1     ; a3 b3 c3 d3 a2 b2 c2 d2
    punpckhdq            m1, m3, m5 ; e1 f1 g1 h1 e0 f0 g0 h0
    punpckldq            m3, m5     ; e3 f3 g3 h3 e2 f2 g2 h2
    punpckhqdq           m5, m7, m1 ;  8  0
    vextracti128 [r2  +strideq*0], m5, 1
    punpcklqdq           m7, m1     ;  9  1
    mova         [dstq+strideq*0], xm5
    punpckhqdq           m1, m8, m3 ; 10  2
    vextracti128 [r2  +strideq*1], m7, 1
    punpcklqdq           m8, m3     ; 11  3
    mova         [dstq+strideq*1], xm7
    punpckhdq            m3, m0, m2 ; a5 b5 c5 d5 a4 b4 c4 d4
    vextracti128 [r2  +strideq*2], m1, 1
    punpckldq            m0, m2     ; a7 b7 c7 d7 a6 b6 c6 d6
    mova         [dstq+strideq*2], xm1
    punpckhdq            m2, m4, m6 ; e5 f5 g5 h5 e4 f4 g4 h4
    vextracti128 [r2  +r3       ], m8, 1
    punpckldq            m4, m6     ; e7 f7 g7 h7 e6 f6 g6 h6
    mova         [dstq+r3       ], xm8
    punpckhqdq           m6, m3, m2 ; 12  4
    vextracti128 [r2  +strideq*4], m6, 1
    punpcklqdq           m3, m2     ; 13  5
    mova         [dstq+strideq*4], xm6
    punpckhqdq           m2, m0, m4 ; 14  6
    vextracti128 [r2  +r4       ], m3, 1
    punpcklqdq           m0, m4     ; 15  7
    mova         [dstq+r4       ], xm3
    vextracti128 [r2  +r3*2     ], m2, 1
    mova         [dstq+r3*2     ], xm2
    vextracti128 [r2  +r5       ], m0, 1
    mova         [dstq+r5       ], xm0
    lea                  r2, [dstq+strideq*8]
    cmp                  r6, rsp
    jae .h32_transpose_loop
    add                 rsp, 64*8
    sub              org_wd, 8
    jg .h32_transpose_loop0
.h32_end:
    RET
.h64:
    %assign stack_offset org_stack_offset
    ALLOC_STACK        -256, 10
    lea            maxbased, [wq+63]
    test             angled, 0x400
    jnz .h64_main
    vpbroadcastd         m2, [pw_3]
    movu                 m0, [tlq-28]       ; 3 4 5 6 7 8 9 a   b c d e f g h i
    punpckhwd            m1, m0, m0
    vpblendd             m1, [tlq-26], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
    paddw                m0, [tlq-30]       ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m1, m2
    paddw                m0, [tlq-32]       ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    pavgw                m1, [tlq-34]       ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    lea                  r4, [rsp+224]
    paddw                m0, m1
    lea                 r5d, [wq+32]
    psrlw                m0, 2
    mova               [r4], m0
.h64_filter_loop:
    mova                 m0, [tlq-62]
    paddw                m1, m2, [tlq-66]
    paddw                m0, [tlq-64]
    pavgw                m1, [tlq-58]
    paddw                m0, [tlq-60]
    sub                 tlq, 32
    sub                  r4, 32
    paddw                m0, m1
    psrlw                m0, 2
    mova               [r4], m0
    sub                 r5d, 16
    jg .h64_filter_loop
    mova                 m0, [tlq-62]           ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    punpcklwd           xm1, xm0, xm0
    paddw                m2, [tlq-58]           ; 2 3 4 5 6 7 8 9   a b c d e f g h
    paddw                m0, [tlq-60]           ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    vpblendd             m3, m1, [tlq-66], 0xfe ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
    vpblendd             m1, [tlq-64], 0xfe     ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    lea                 tlq, [rsp+254]
    pavgw                m2, m3
    paddw                m0, m1
    paddw                m0, m2
    psrlw                m0, 2
    mova            [r4-32], m0
.h64_main:
    neg            maxbaseq
    movd                xm4, dyd
    vpbroadcastw         m6, [tlq+maxbaseq*2]
    shl            maxbased, 6
    vpbroadcastw         m4, xm4
    lea                 r4d, [maxbaseq+dyq+15*64]
    neg                 dyq
    vpbroadcastd         m7, [pw_m1024]
    movd                xm3, r4d
    lea                  r4, [dyq+63]
    paddw                m8, m7, m7
    vpbroadcastw         m3, xm3
    or             maxbased, 63
    paddw                m9, m8, m7
    psubw                m3, [z_base_inc]
.h64_loop:
    mov                  r5, r4
    sar                  r5, 6
    movu                 m1, [tlq+r5*2-128]
    movu                 m0, [tlq+r5*2-126]
    pand                 m2, m5, m3
    psllw                m2, 9
    psubw                m1, m0
    pmulhrsw             m1, m2
    sub                 rsp, 128
    paddw                m0, m1
    pcmpgtw              m1, m9, m3
    vpblendvb            m0, m6, m0, m1
    mova         [rsp+32*0], m0
    movu                 m1, [tlq+r5*2-96]
    movu                 m0, [tlq+r5*2-94]
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    pcmpgtw              m1, m8, m3
    vpblendvb            m0, m6, m0, m1
    mova         [rsp+32*1], m0
    movu                 m1, [tlq+r5*2-64]
    movu                 m0, [tlq+r5*2-62]
    psubw                m1, m0
    pmulhrsw             m1, m2
    paddw                m0, m1
    pcmpgtw              m1, m7, m3
    vpblendvb            m0, m6, m0, m1
    mova         [rsp+32*2], m0
    movu                 m1, [tlq+r5*2-32]
    movu                 m0, [tlq+r5*2-30]
    psubw                m1, m0
    pmulhrsw             m1, m2
    add                  r4, dyq
    psraw                m2, m3, 15
    paddw                m3, m4
    paddw                m0, m1
    vpblendvb            m0, m6, m0, m2
    mova         [rsp+32*3], m0
    dec                  wd
    jz .h64_transpose
    cmp                 r4d, maxbased
    jg .h64_loop
.h64_end_loop:
    sub                 rsp, 128
    mova         [rsp+32*0], m6
    mova         [rsp+32*1], m6
    mova         [rsp+32*2], m6
    mova         [rsp+32*3], m6
    dec                  wd
    jg .h64_end_loop
.h64_transpose:
    lea                  r2, [strideq*3]
    lea                  r3, [strideq*5]
    mov                  r5, dstq
    lea                  r4, [strideq+r2*2]
.h64_transpose_loop0:
    lea                  r6, [rsp+112]
    lea                dstq, [r5+org_wq*2-32]
.h64_transpose_loop:
    mova                xm0, [r6+128*15]
    vinserti128          m0, [r6+128* 7], 1
    mova                xm1, [r6+128*14]
    vinserti128          m1, [r6+128* 6], 1
    mova                xm2, [r6+128*13]
    vinserti128          m2, [r6+128* 5], 1
    mova                xm3, [r6+128*12]
    vinserti128          m3, [r6+128* 4], 1
    mova                xm4, [r6+128*11]
    vinserti128          m4, [r6+128* 3], 1
    mova                xm5, [r6+128*10]
    vinserti128          m5, [r6+128* 2], 1
    mova                xm6, [r6+128* 9]
    vinserti128          m6, [r6+128* 1], 1
    mova                xm7, [r6+128* 8]
    vinserti128          m7, [r6+128* 0], 1
    punpckhwd            m8, m0, m1
    punpcklwd            m0, m1
    punpckhwd            m1, m2, m3
    punpcklwd            m2, m3
    punpckhwd            m3, m4, m5
    punpcklwd            m4, m5
    punpckhwd            m5, m6, m7
    punpcklwd            m6, m7
    sub                  r6, 16
    punpckhdq            m7, m8, m1
    punpckldq            m8, m1
    punpckhdq            m1, m3, m5
    punpckldq            m3, m5
    punpckhqdq           m5, m7, m1
    punpcklqdq           m7, m1
    punpckhqdq           m1, m8, m3
    punpcklqdq           m8, m3
    punpckhdq            m3, m0, m2
    mova   [dstq+strideq*0], m5
    punpckldq            m0, m2
    mova   [dstq+strideq*1], m7
    punpckhdq            m2, m4, m6
    mova   [dstq+strideq*2], m1
    punpckldq            m4, m6
    mova   [dstq+r2       ], m8
    punpckhqdq           m6, m3, m2
    mova   [dstq+strideq*4], m6
    punpcklqdq           m3, m2
    mova   [dstq+r3       ], m3
    punpckhqdq           m2, m0, m4
    mova   [dstq+r2*2     ], m2
    punpcklqdq           m0, m4
    mova   [dstq+r4       ], m0
    lea                dstq, [dstq+strideq*8]
    cmp                  r6, rsp
    jae .h64_transpose_loop
    add                 rsp, 128*16
    sub              org_wd, 16
    jg .h64_transpose_loop0
.h64_end:
    RET

%macro FILTER_1BLK 5 ; dst, src, tmp, shuf, bdmax
%ifnum %4
    pshufb             xm%2, xm%4
%else
    pshufb             xm%2, %4
%endif
    vinserti128         m%2, xm%2, 1
    pshufd              m%1, m%2, q0000
    pmaddwd             m%1, m2
    pshufd              m%3, m%2, q1111
    pmaddwd             m%3, m3
    paddd               m%1, m1
    paddd               m%1, m%3
    pshufd              m%3, m%2, q2222
    pmaddwd             m%3, m4
    paddd               m%1, m%3
    pshufd              m%3, m%2, q3333
    pmaddwd             m%3, m5
    paddd               m%1, m%3
    psrad               m%1, 4
    packusdw            m%1, m%1
    pminsw              m%1, m%5
%endmacro

%macro FILTER_2BLK 7 ; dst, src, tmp_dst, tmp_src, tmp, shuf, bdmax
    pshufb              m%2, m%6
    vpermq              m%4, m%2, q3232
    vinserti128         m%2, xm%2, 1
    pshufd              m%1, m%2, q0000
    pshufd              m%3, m%4, q0000
    pmaddwd             m%1, m2
    pmaddwd             m%3, m2
    paddd               m%1, m1
    paddd               m%3, m1
    pshufd              m%5, m%2, q1111
    pmaddwd             m%5, m3
    paddd               m%1, m%5
    pshufd              m%5, m%4, q1111
    pmaddwd             m%5, m3
    paddd               m%3, m%5
    pshufd              m%5, m%2, q2222
    pmaddwd             m%5, m4
    paddd               m%1, m%5
    pshufd              m%5, m%4, q2222
    pmaddwd             m%5, m4
    paddd               m%3, m%5
    pshufd              m%5, m%2, q3333
    pmaddwd             m%5, m5
    paddd               m%1, m%5
    pshufd              m%5, m%4, q3333
    pmaddwd             m%5, m5
    paddd               m%3, m%5
    psrad               m%1, 4
    psrad               m%3, 4
    packusdw            m%1, m%3
    pminsw              m%1, m%7
%endmacro

; The ipred_filter SIMD processes 4x2 blocks in the following order which
; increases parallelism compared to doing things row by row. One redundant
; block is calculated for w8 and w16, two for w32.
;     w4     w8       w16             w32
;     1     1 2     1 2 3 5     1 2 3 5 b c d f
;     2     2 3     2 4 5 7     2 4 5 7 c e f h
;     3     3 4     4 6 7 9     4 6 7 9 e g h j
; ___ 4 ___ 4 5 ___ 6 8 9 a ___ 6 8 9 a g i j k ___
;           5       8           8       i

cglobal ipred_filter_16bpc, 3, 9, 0, dst, stride, tl, w, h, filter
%assign org_stack_offset stack_offset
%define base r6-ipred_filter_16bpc_avx2_table
    lea                  r6, [filter_intra_taps]
    tzcnt                wd, wm
%ifidn filterd, filterm
    movzx           filterd, filterb
%else
    movzx           filterd, byte filterm
%endif
    shl             filterd, 6
    add             filterq, r6
    lea                  r6, [ipred_filter_16bpc_avx2_table]
    vbroadcasti128       m0, [tlq-6]
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m1, [base+pd_8]
    pmovsxbw             m2, [filterq+16*0]
    pmovsxbw             m3, [filterq+16*1]
    pmovsxbw             m4, [filterq+16*2]
    pmovsxbw             m5, [filterq+16*3]
    add                  wq, r6
    mov                  hd, hm
    jmp                  wq
.w4:
    WIN64_SPILL_XMM      10
    mova                xm8, [base+filter_shuf2]
    vpbroadcastw         m9, r8m ; bitdepth_max
    lea                  r7, [6+hq*2]
    sub                 tlq, r7
    jmp .w4_loop_start
.w4_loop:
    pinsrq              xm0, [tlq+hq*2], 0
    lea                dstq, [dstq+strideq*2]
.w4_loop_start:
    FILTER_1BLK           6, 0, 7, 8, 9
    vextracti128        xm0, m6, 1
    movq   [dstq+strideq*0], xm6
    movq   [dstq+strideq*1], xm0
    sub                  hd, 2
    jg .w4_loop
    RET
ALIGN function_align
.w8:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      16
    vbroadcasti128      m14, [base+filter_shuf3]
    vpbroadcastw        m15, r8m ; bitdepth_max
    FILTER_1BLK          10, 0, 7, [base+filter_shuf2], 15
    vpermq               m6, m10, q1302         ; ____ ____ | ____ 4321
    pslldq               m8, m0, 4
    psrldq               m7, m6, 2
    psrldq               m0, m6, 10
    punpcklwd            m7, m0
    vpblendd             m8, m6, 0x33           ; _0__ 4321 | ____ 4321
    vpblendd             m8, m7, 0x40           ; _056 4321 | ____ 4321
    vpblendd             m8, [tlq-6], 0x30      ; _056 4321 | ____ 4321
    lea                  r7, [16+hq*2]
    sub                 tlq, r7
    jmp .w8_loop_start
.w8_loop:
    vpermq               m8, m9, q1302          ; ____ 4321 | ____ 4321
    vpermq               m6, m9, q2031
    psrldq               m0, m6, 2
    psrldq               m6, 10
    punpcklwd            m6, m0
    vpblendd             m8, m7, 0x80           ; _0__ 4321 | ____ 4321
    vpblendd             m8, m6, 0x40           ; _056 4321 | ____ 4321
    mova                m10, m9
.w8_loop_start:
    vpblendd             m8, [tlq+hq*2], 0x0C   ; _056 4321 | _056 4321
    call .main
    vpblendd            m10, m9, 0xCC
    mova         [dstq+strideq*0], xm10
    vextracti128 [dstq+strideq*1], m10, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    %assign stack_offset stack_offset - stack_size_padded
    ALLOC_STACK          32, 16
    vpbroadcastw        m15, r8m ; bitdepth_max
    sub                  hd, 2
    TAIL_CALL .w16_main, 0
.w16_main:
    mova               xm10, [base+filter_shuf2]
    FILTER_1BLK          13, 0, 6, 10, 15
    vpermq              m12, m13, q3120
    mova               xm14, [base+filter_shuf3]
    vinserti128         m14, [base+filter_shuf1], 1
    vpbroadcastq         m0, [tlq+10]
    vpblendd             m0, [tlq-16], 0x4C     ; ___0 4321 | _056 ____
    psrldq               m6, m12, 8
    vpblendd             m0, m6, 0x03           ; ___0 4321 | _056 4321
    punpcklwd            m6, m12
    vpblendd             m0, m6, 0x80           ; 56_0 4321 | _056 4321
    FILTER_2BLK          12, 0, 6, 7, 8, 14, 15
    vpblendd            m13, m12, 0xCC
    vpermq              m12, m12, q2031         ; 6___ 5___
    psrldq              xm6, xm12, 2
    psrldq              xm8, xm12, 12
    vpblendd            xm6, xm8, 0x01
    pblendw             xm6, [tlq+10], 0xF8     ; 4321 056_
    FILTER_1BLK          11, 6, 8, 10, 15
    vpermq              m11, m11, q3120
    pshufd               m9, m11, q1032
    movu                 m8, [tlq+6]            ; __43 210_ | ____ ____
    pshufd               m8, m8, q3021          ; __0_ 4321 | ____ ____
    pshufhw              m8, m8, q3201          ; ___0 4321 | ____ ____
    vpblendd             m9, m8, 0x70           ; ___0 4321 | ____ 4321
    mova         [dstq+strideq*0], xm13
    vextracti128 [dstq+strideq*1], m13, 1
    lea                  r7, [20+hq*2]
    sub                 tlq, r7
    vpermq               m6, m12, q0123         ; ____ 4321 | ____ 4321
    jmp .w16_loop_start
.w16_loop:
    vpermq              m13, m13, q3322
    vpermq              m11,  m9, q2020
    vpermq               m9,  m9, q1302
    vpermq               m6, m12, q0123
    psrldq               m7, 4
    vpblendd            m13, m10, 0xCC
    vpblendd             m9, m7, 0x40
    mova                 m0, [rsp+8]
    mova         [dstq+strideq*0], xm13
    vextracti128 [dstq+strideq*1], m13, 1
.w16_loop_start:
    mova                m13, m12
    vpblendd             m0, [tlq+hq*2], 0x0C
    psrldq               m7, m12, 8
    punpcklwd            m7, m12
    vpblendd             m0, m6, 0x33           ; ___0 4321 | _056 4321
    vpblendd             m0, m7, 0x80           ; 56_0 4321 | _056 4321
    FILTER_2BLK          10, 0, 6, 7, 8, 14, 15
    vpermq              m12, m10, q2031
    mova            [rsp+8], m0
    psrldq               m8, m11, 8
    psrldq              xm6, xm12, 2
    psrldq              xm7, xm12, 10
    psrldq              xm0, xm13, 2
    punpcklwd            m8, m11
    punpcklwd           xm7, xm6
    vpblendd             m8, m9, 0x73           ; 56_0 4321 | ____ 4321
    vpblendd             m8, m7, 0x04           ; 56_0 4321 | __56 4321
    vpblendd             m8, m0, 0x08           ; 56_0 4321 | _056 4321
    call .main
    vpermq               m8, m11, q3120
    vpblendd             m6, m8, m9, 0xCC
    mova         [dstq+strideq*0+16], xm6
    vextracti128 [dstq+strideq*1+16], m6, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_loop
    vpermq               m8, m9, q3120
    vextracti128        xm0, m8, 1              ; 4321 ____
    pshufd             xm11, xm11, q1032
    vpblendd            xm0, xm11, 0x02         ; 4321 0___
    psrldq              xm6, xm8, 2
    psrldq              xm7, xm8, 12
    pblendw             xm0, xm6, 0x4           ; 4321 05__
    pblendw             xm0, xm7, 0x2           ; 4321 056_
    FILTER_1BLK           6, 0, 7, [base+filter_shuf2], 15
    vpermq              m12, m13, q1302
    vpblendd            m12, m10, 0xCC
    vpblendd             m9, m6, 0xCC
    mova         [dstq+strideq*0+ 0], xm12
    mova         [dstq+strideq*0+16], xm9
    vextracti128 [dstq+strideq*1+ 0], m12, 1
    vextracti128 [dstq+strideq*1+16], m9, 1
    ret
ALIGN function_align
.w32:
    %assign stack_offset org_stack_offset
    ALLOC_STACK          64, 16
    vpbroadcastw        m15, r8m ; bitdepth_max
    sub                  hd, 2
    lea                  r3, [dstq+32]
    lea                 r5d, [hd*2+20]
    call .w16_main
    mov                dstq, r3
    lea                 tlq, [tlq+r5+32]
    sub                 r5d, 20
    shr                 r5d, 1
    sub                 r5d, 2
    lea                  r4, [dstq+strideq*2-2]
DEFINE_ARGS dst, stride, tl, stride3, left, h
    lea            stride3q, [strideq*3]
    movu                 m8, [tlq-6]                        ; 4321 0___
    mova               xm10, [base+filter_shuf2]
    pinsrw              xm0, xm8, [dstq+strideq*0-2], 2
    pinsrw              xm0, xm0, [dstq+strideq*1-2], 1     ; 4321 056_
    pinsrw              xm9, [leftq+strideq*0], 5
    pinsrw              xm9, [leftq+strideq*1], 4
    FILTER_1BLK          13, 0, 6, 10, 15
    vpermq              m12, m13, q3120
    mova               xm14, [base+filter_shuf3]
    vinserti128         m14, [base+filter_shuf1], 1
    psrldq               m6, m12, 8
    punpcklwd            m7, m6, m12
    vpblendd             m0, m6, 0x03           ; ___0 ____ | _0__ 4321
    vpblendd             m0, m7, 0x80           ; 56_0 ____ | _0__ 4321
    vpblendd             m0, m8, 0x30           ; 56_0 4321 | _0__ 4321
    vpblendd             m0, m9, 0x04           ; 56_0 4321 | _056 4321
    FILTER_2BLK          12, 0, 6, 7, 8, 14, 15
    vpblendd            m13, m12, 0xCC
    pinsrw              xm9, [leftq+strideq*2], 3
    pinsrw              xm9, [leftq+stride3q ], 2
    lea               leftq, [leftq+strideq*4]
    pinsrw              xm9, [leftq+strideq*0], 1
    pinsrw              xm9, [leftq+strideq*1], 0
    movq           [rsp+32], xm9
    mov                 r7d, 1
    pslldq               m8, m9, 4
    vpblendd             m0, m8, 0x0C           ; ___0 ____ | _056 ____
    vpermq              m12, m12, q2031         ; 6___ 5___
    psrldq              xm6, xm12, 2
    psrldq              xm7, xm12, 12
    vpblendd            xm6, xm7, 0x01          ; ____ _56_
    pblendw             xm6, [tlq+10], 0xF8     ; 4321 056_
    FILTER_1BLK          11, 6, 7, 10, 15
    vpermq              m11, m11, q3120
    pshufd               m9, m11, q1032
    vbroadcasti128       m8, [tlq+22]           ; __43 210_ | ____ ____
    pshufd               m8, m8, q3021          ; __0_ 4321 | ____ ____
    pshufhw              m8, m8, q3201          ; ___0 4321 | ____ ____
    vpblendd             m9, m8, 0x70           ; ___0 4321 | ____ 4321
    mova         [dstq+strideq*0], xm13
    vextracti128 [dstq+strideq*1], m13, 1
    vpermq               m6, m12, q0123         ; ____ 4321 | ____ 4321
    jmp .w32_loop_start
.w32_loop_last:
    mova                 m0, [rsp+0]
    jmp .w32_loop
.w32_loop_left:
    mova                 m0, [rsp+0]
    vpblendd             m0, [rsp+32+r7*4-12], 0x0C
    dec                 r7d
    jg .w32_loop
    cmp                  hd, 2
    je .w32_loop
    pinsrw              xm6, [rsp+32], 6
    pinsrw              xm6, [leftq+strideq*2], 5
    pinsrw              xm6, [leftq+stride3q ], 4
    lea               leftq, [leftq+strideq*4]
    pinsrw              xm6, [leftq+strideq*0], 3
    pinsrw              xm6, [leftq+strideq*1], 2
    pinsrw              xm6, [leftq+strideq*2], 1
    pinsrw              xm6, [leftq+stride3q ], 0
    lea               leftq, [leftq+strideq*4]
    movu           [rsp+36], xm6
    pinsrw              xm6, [leftq+strideq*0], 1
    pinsrw              xm6, [leftq+strideq*1], 0
    movd           [rsp+32], xm6
    mov                 r7d, 4
.w32_loop:
    vpermq              m13, m13, q3322
    vpermq              m11,  m9, q2020
    vpermq               m9,  m9, q1302
    vpermq               m6, m12, q0123
    psrldq               m7, 4
    vpblendd            m13, m10, 0xCC
    vpblendd             m9, m7, 0x40           ; ___0 4321 | ____ 4321
    mova         [dstq+strideq*0], xm13
    vextracti128 [dstq+strideq*1], m13, 1
.w32_loop_start:
    mova                m13, m12
    psrldq               m7, m12, 8
    punpcklwd            m7, m12
    vpblendd             m0, m6, 0x33           ; ___0 4321 | _056 4321
    vpblendd             m0, m7, 0x80           ; 56_0 4321 | _056 4321
    FILTER_2BLK          10, 0, 6, 7, 8, 14, 15
    vpermq              m12, m10, q2031
    mova            [rsp+0], m0
    psrldq               m8, m11, 8
    psrldq              xm6, xm12, 2
    psrldq              xm7, xm12, 10
    psrldq              xm0, xm13, 2
    punpcklwd            m8, m11
    punpcklwd           xm7, xm6
    vpblendd             m8, m9, 0x73           ; 56_0 4321 | ____ 4321
    vpblendd             m8, m7, 0x04           ; 56_0 4321 | __56 4321
    vpblendd             m8, m0, 0x08           ; 56_0 4321 | _056 4321
    call .main
    vpermq               m8, m11, q3120
    vpblendd             m6, m8, m9, 0xCC
    mova         [dstq+strideq*0+16], xm6
    vextracti128 [dstq+strideq*1+16], m6, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop_left
    jz .w32_loop_last
    vpermq               m8, m9, q3120
    vextracti128        xm0, m8, 1              ; 4321 ____
    pshufd             xm11, xm11, q1032
    vpblendd            xm0, xm11, 0x02         ; 4321 0___
    psrldq              xm6, xm8, 2
    psrldq              xm7, xm8, 12
    pblendw             xm0, xm6, 0x4           ; 4321 05__
    pblendw             xm0, xm7, 0x2           ; 4321 056_
    FILTER_1BLK           6, 0, 7, [base+filter_shuf2], 15
    vpermq              m12, m13, q1302
    vpblendd            m12, m10, 0xCC
    vpblendd             m9, m6, 0xCC
    mova         [dstq+strideq*0+ 0], xm12
    mova         [dstq+strideq*0+16], xm9
    vextracti128 [dstq+strideq*1+ 0], m12, 1
    vextracti128 [dstq+strideq*1+16], m9, 1
    RET
.main:
    FILTER_2BLK           9, 8, 6, 7, 0, 14, 15
    ret

%if WIN64
DECLARE_REG_TMP 5
%else
DECLARE_REG_TMP 7
%endif

%macro IPRED_CFL 1 ; ac in, unpacked pixels out
    psignw               m3, m%1, m1
    pabsw               m%1, m%1
    pmulhrsw            m%1, m2
    psignw              m%1, m3
    paddw               m%1, m0
%endmacro

cglobal ipred_cfl_top_16bpc, 3, 7, 8, dst, stride, tl, w, h, ac, alpha
    movifnidn            hd, hm
    add                 tlq, 2
    movd                xm4, wd
    pxor                 m6, m6
    vpbroadcastw         m7, r7m
    pavgw               xm4, xm6
    tzcnt                wd, wd
    movd                xm5, wd
    movu                 m0, [tlq]
    lea                  t0, [ipred_cfl_left_16bpc_avx2_table]
    movsxd               r6, [t0+wq*4]
    add                  r6, t0
    add                  t0, ipred_cfl_splat_16bpc_avx2_table-ipred_cfl_left_16bpc_avx2_table
    movsxd               wq, [t0+wq*4]
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6

cglobal ipred_cfl_left_16bpc, 3, 7, 8, dst, stride, tl, w, h, ac, alpha
    mov                  hd, hm ; zero upper half
    sub                 tlq, hq
    movd                xm4, hd
    sub                 tlq, hq
    pxor                 m6, m6
    vpbroadcastw         m7, r7m
    pavgw               xm4, xm6
    tzcnt               r6d, hd
    movd                xm5, r6d
    movu                 m0, [tlq]
    lea                  t0, [ipred_cfl_left_16bpc_avx2_table]
    movsxd               r6, [t0+r6*4]
    add                  r6, t0
    add                  t0, ipred_cfl_splat_16bpc_avx2_table-ipred_cfl_left_16bpc_avx2_table
    tzcnt                wd, wd
    movsxd               wq, [t0+wq*4]
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6
.h32:
    paddw                m0, [tlq+32]
.h16:
    vextracti128        xm1, m0, 1
    paddw               xm0, xm1
.h8:
    psrldq              xm1, xm0, 8
    paddw               xm0, xm1
.h4:
    punpcklwd           xm0, xm6
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    paddd               xm0, xm4
    psrld               xm0, xm5
    vpbroadcastw         m0, xm0
    jmp                  wq

cglobal ipred_cfl_16bpc, 3, 7, 8, dst, stride, tl, w, h, ac, alpha
    movifnidn            hd, hm
    movifnidn            wd, wm
    tzcnt               r6d, hd
    lea                 t0d, [wq+hq]
    movd                xm4, t0d
    tzcnt               t0d, t0d
    movd                xm5, t0d
    lea                  t0, [ipred_cfl_16bpc_avx2_table]
    tzcnt                wd, wd
    movsxd               r6, [t0+r6*4]
    movsxd               wq, [t0+wq*4+4*4]
    psrlw               xm4, 1
    pxor                 m6, m6
    vpbroadcastw         m7, r7m
    add                  r6, t0
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6
.h4:
    movq                xm0, [tlq-8]
    jmp                  wq
.w4:
    movq                xm1, [tlq+2]
    paddw                m0, m4
    paddw                m0, m1
    psrlq                m1, m0, 32
    paddw                m0, m1
    psrld                m1, m0, 16
    paddw                m0, m1
    cmp                  hd, 4
    jg .w4_mul
    psrlw               xm0, 3
    jmp .w4_end
.w4_mul:
    vextracti128        xm1, m0, 1
    paddw               xm0, xm1
    lea                 r2d, [hq*2]
    mov                 r6d, 0xAAAB6667
    shrx                r6d, r6d, r2d
    punpckhwd           xm1, xm0, xm6
    punpcklwd           xm0, xm6
    paddd               xm0, xm1
    movd                xm1, r6d
    psrld               xm0, 2
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w4_end:
    vpbroadcastw         m0, xm0
.s4:
    vpbroadcastw         m1, alpham
    lea                  r6, [strideq*3]
    pabsw                m2, m1
    psllw                m2, 9
.s4_loop:
    mova                 m4, [acq]
    IPRED_CFL             4
    pmaxsw               m4, m6
    pminsw               m4, m7
    vextracti128        xm5, m4, 1
    movq   [dstq+strideq*0], xm4
    movq   [dstq+strideq*2], xm5
    movhps [dstq+strideq*1], xm4
    movhps [dstq+r6       ], xm5
    lea                dstq, [dstq+strideq*4]
    add                 acq, 32
    sub                  hd, 4
    jg .s4_loop
    RET
ALIGN function_align
.h8:
    mova                xm0, [tlq-16]
    jmp                  wq
.w8:
    vextracti128        xm1, m0, 1
    paddw               xm0, [tlq+2]
    paddw               xm0, xm4
    paddw               xm0, xm1
    psrld               xm1, xm0, 16
    paddw               xm0, xm1
    pblendw             xm0, xm6, 0xAA
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    psrld               xm0, xm5
    cmp                  hd, 8
    je .w8_end
    mov                 r6d, 0xAAAB
    mov                 r2d, 0x6667
    cmp                  hd, 32
    cmovz               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w8_end:
    vpbroadcastw         m0, xm0
.s8:
    vpbroadcastw         m1, alpham
    lea                  r6, [strideq*3]
    pabsw                m2, m1
    psllw                m2, 9
.s8_loop:
    mova                 m4, [acq]
    mova                 m5, [acq+32]
    IPRED_CFL             4
    IPRED_CFL             5
    pmaxsw               m4, m6
    pmaxsw               m5, m6
    pminsw               m4, m7
    pminsw               m5, m7
    mova         [dstq+strideq*0], xm4
    mova         [dstq+strideq*2], xm5
    vextracti128 [dstq+strideq*1], m4, 1
    vextracti128 [dstq+r6       ], m5, 1
    lea                dstq, [dstq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .s8_loop
    RET
ALIGN function_align
.h16:
    mova                 m0, [tlq-32]
    jmp                  wq
.w16:
    paddw                m0, [tlq+2]
    vextracti128        xm1, m0, 1
    paddw               xm0, xm4
    paddw               xm0, xm1
    punpckhwd           xm1, xm0, xm6
    punpcklwd           xm0, xm6
    paddd               xm0, xm1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    psrld               xm0, xm5
    cmp                  hd, 16
    je .w16_end
    mov                 r6d, 0xAAAB
    mov                 r2d, 0x6667
    test                 hb, 8|32
    cmovz               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w16_end:
    vpbroadcastw         m0, xm0
.s16:
    vpbroadcastw         m1, alpham
    pabsw                m2, m1
    psllw                m2, 9
.s16_loop:
    mova                 m4, [acq]
    mova                 m5, [acq+32]
    IPRED_CFL             4
    IPRED_CFL             5
    pmaxsw               m4, m6
    pmaxsw               m5, m6
    pminsw               m4, m7
    pminsw               m5, m7
    mova   [dstq+strideq*0], m4
    mova   [dstq+strideq*1], m5
    lea                dstq, [dstq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .s16_loop
    RET
ALIGN function_align
.h32:
    mova                 m0, [tlq-64]
    paddw                m0, [tlq-32]
    jmp                  wq
.w32:
    paddw                m0, [tlq+ 2]
    paddw                m0, [tlq+34]
    vextracti128        xm1, m0, 1
    paddw               xm0, xm4
    paddw               xm0, xm1
    punpcklwd           xm1, xm0, xm6
    punpckhwd           xm0, xm6
    paddd               xm0, xm1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
    psrldq              xm1, xm0, 8
    paddd               xm0, xm1
    psrld               xm0, xm5
    cmp                  hd, 32
    je .w32_end
    lea                 r2d, [hq*2]
    mov                 r6d, 0x6667AAAB
    shrx                r6d, r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
    psrlw               xm0, 1
.w32_end:
    vpbroadcastw         m0, xm0
.s32:
    vpbroadcastw         m1, alpham
    pabsw                m2, m1
    psllw                m2, 9
.s32_loop:
    mova                 m4, [acq]
    mova                 m5, [acq+32]
    IPRED_CFL             4
    IPRED_CFL             5
    pmaxsw               m4, m6
    pmaxsw               m5, m6
    pminsw               m4, m7
    pminsw               m5, m7
    mova        [dstq+32*0], m4
    mova        [dstq+32*1], m5
    add                dstq, strideq
    add                 acq, 64
    dec                  hd
    jg .s32_loop
    RET

cglobal ipred_cfl_128_16bpc, 3, 7, 8, dst, stride, tl, w, h, ac, alpha
    mov                 r6d, r7m
    shr                 r6d, 11
    lea                  t0, [ipred_cfl_splat_16bpc_avx2_table]
    tzcnt                wd, wd
    movifnidn            hd, hm
    movsxd               wq, [t0+wq*4]
    vpbroadcastd         m0, [t0-ipred_cfl_splat_16bpc_avx2_table+pw_512+r6*4]
    pxor                 m6, m6
    vpbroadcastw         m7, r7m
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  wq

cglobal ipred_cfl_ac_420_16bpc, 4, 7, 6, ac, ypx, stride, wpad, hpad, w, h
    movifnidn         hpadd, hpadm
    vpbroadcastd         m5, [pw_2]
    mov                  hd, hm
    shl               hpadd, 2
    pxor                 m4, m4
    sub                  hd, hpadd
    cmp            dword wm, 8
    jg .w16
    je .w8
.w4:
    lea                  r3, [strideq*3]
    mov                  r5, acq
.w4_loop:
    mova                xm0, [ypxq+strideq*2]
    mova                xm1, [ypxq+r3       ]
    vinserti128          m0, [ypxq+strideq*0], 1
    vinserti128          m1, [ypxq+strideq*1], 1
    lea                ypxq, [ypxq+strideq*4]
    pmaddwd              m0, m5
    pmaddwd              m1, m5
    paddd                m0, m1
    vextracti128        xm1, m0, 1
    paddd                m4, m0
    packssdw            xm1, xm0
    mova              [acq], xm1
    add                 acq, 16
    sub                  hd, 2
    jg .w4_loop
    test              hpadd, hpadd
    jz .dc
    vpermq               m1, m1, q1111
    pslld               xm0, 2
.w4_hpad_loop:
    mova              [acq], m1
    paddd                m4, m0
    add                 acq, 32
    sub               hpadd, 4
    jg .w4_hpad_loop
    jmp .dc
.w8:
    mov                  r5, acq
    test              wpadd, wpadd
    jnz .w8_wpad1
.w8_loop:
    pmaddwd              m0, m5, [ypxq+strideq*0]
    pmaddwd              m1, m5, [ypxq+strideq*1]
    lea                ypxq, [ypxq+strideq*2]
    paddd                m0, m1
    vextracti128        xm1, m0, 1
    paddd                m4, m0
    packssdw            xm1, xm0, xm1
    mova              [acq], xm1
    add                 acq, 16
    dec                  hd
    jg .w8_loop
.w8_hpad:
    test              hpadd, hpadd
    jz .dc
    vinserti128          m1, xm1, 1
    pslld                m0, 2
    jmp .hpad
.w8_wpad1:
    pmaddwd             xm0, xm5, [ypxq+strideq*0]
    pmaddwd             xm3, xm5, [ypxq+strideq*1]
    lea                ypxq, [ypxq+strideq*2]
    paddd               xm0, xm3
    pshufd              xm3, xm0, q3333
    packssdw            xm1, xm0, xm3
    paddd               xm0, xm3
    paddd               xm4, xm0
    mova              [acq], xm1
    add                 acq, 16
    dec                  hd
    jg .w8_wpad1
    jmp .w8_hpad
.w16_wpad:
    mova                 m0, [ypxq+strideq*0+ 0]
    mova                 m1, [ypxq+strideq*1+ 0]
    cmp               wpadd, 2
    jl .w16_wpad1
    je .w16_wpad2
    vpbroadcastd         m2, [ypxq+strideq*0+12]
    vpbroadcastd         m3, [ypxq+strideq*1+12]
    vpblendd             m0, m2, 0xf0
    vpblendd             m1, m3, 0xf0
    jmp .w16_wpad_end
.w16_wpad2:
    vpbroadcastd         m2, [ypxq+strideq*0+28]
    vpbroadcastd         m3, [ypxq+strideq*1+28]
    jmp .w16_wpad_end
.w16_wpad1:
    vpbroadcastd         m2, [ypxq+strideq*0+44]
    vpbroadcastd         m3, [ypxq+strideq*1+44]
    vinserti128          m2, [ypxq+strideq*0+32], 0
    vinserti128          m3, [ypxq+strideq*1+32], 0
.w16_wpad_end:
    lea                ypxq, [ypxq+strideq*2]
    REPX    {pmaddwd x, m5}, m0, m1, m2, m3
    paddd                m0, m1
    paddd                m2, m3
    packssdw             m1, m0, m2
    paddd                m0, m2
    vpermq               m1, m1, q3120
    paddd                m4, m0
    mova              [acq], m1
    add                 acq, 32
    dec                  hd
    jg .w16_wpad
    jmp .w16_hpad
.w16:
    mov                  r5, acq
    test              wpadd, wpadd
    jnz .w16_wpad
.w16_loop:
    pmaddwd              m0, m5, [ypxq+strideq*0+ 0]
    pmaddwd              m2, m5, [ypxq+strideq*0+32]
    pmaddwd              m1, m5, [ypxq+strideq*1+ 0]
    pmaddwd              m3, m5, [ypxq+strideq*1+32]
    lea                ypxq, [ypxq+strideq*2]
    paddd                m0, m1
    paddd                m2, m3
    packssdw             m1, m0, m2
    paddd                m0, m2
    vpermq               m1, m1, q3120
    paddd                m4, m0
    mova              [acq], m1
    add                 acq, 32
    dec                  hd
    jg .w16_loop
.w16_hpad:
    add               hpadd, hpadd
    jz .dc
    paddd                m0, m0
.hpad:
    mova         [acq+32*0], m1
    paddd                m4, m0
    mova         [acq+32*1], m1
    add                 acq, 32*2
    sub               hpadd, 4
    jg .hpad
.dc:
    vextracti128        xm1, m4, 1
    sub                  r5, acq ; -w*h*2
    tzcnt               r1d, r5d
    paddd               xm4, xm1
    sub                 r1d, 2
    punpckhqdq          xm1, xm4, xm4
    movd                xm0, r1d
    paddd               xm1, xm4
    pshuflw             xm4, xm1, q1032
    paddd               xm1, xm4
    psrld               xm1, xm0
    pxor                xm0, xm0
    pavgw               xm1, xm0
    vpbroadcastw         m1, xm1
.dc_loop:
    mova                 m0, [acq+r5]
    psubw                m0, m1
    mova           [acq+r5], m0
    add                  r5, 32
    jl .dc_loop
    RET

cglobal ipred_cfl_ac_422_16bpc, 4, 7, 6, ac, ypx, stride, wpad, hpad, w, h
    movifnidn         hpadd, hpadm
    vpbroadcastd         m5, [pw_4]
    mov                  hd, hm
    shl               hpadd, 2
    pxor                 m4, m4
    sub                  hd, hpadd
    cmp            dword wm, 8
    jg .w16
    je .w8
.w4:
    lea                  r3, [strideq*3]
    mov                  r5, acq
.w4_loop:
    mova                xm0, [ypxq+strideq*0]
    mova                xm1, [ypxq+strideq*1]
    vinserti128          m0, [ypxq+strideq*2], 1
    vinserti128          m1, [ypxq+r3       ], 1
    lea                ypxq, [ypxq+strideq*4]
    pmaddwd              m0, m5
    pmaddwd              m1, m5
    paddd                m4, m0
    packssdw             m0, m1
    paddd                m4, m1
    mova              [acq], m0
    add                 acq, 32
    sub                  hd, 4
    jg .w4_loop
    test              hpadd, hpadd
    jz mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
    vextracti128        xm1, m1, 1
    vpermq               m0, m0, q3333
    pslld               xm1, 2
.w4_hpad_loop:
    mova              [acq], m0
    paddd                m4, m1
    add                 acq, 32
    sub               hpadd, 4
    jg .w4_hpad_loop
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
.w8:
    mov                  r5, acq
    test              wpadd, wpadd
    jnz .w8_wpad1
.w8_loop:
    pmaddwd              m1, m5, [ypxq+strideq*0]
    pmaddwd              m0, m5, [ypxq+strideq*1]
    lea                ypxq, [ypxq+strideq*2]
    paddd                m4, m1
    packssdw             m1, m0
    paddd                m4, m0
    vpermq               m2, m1, q3120
    mova              [acq], m2
    add                 acq, 32
    sub                  hd, 2
    jg .w8_loop
.w8_hpad:
    test              hpadd, hpadd
    jz mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
    vpermq               m1, m1, q3131
    pslld                m0, 2
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).hpad
.w8_wpad1:
    vpbroadcastd         m1, [ypxq+strideq*0+12]
    vpbroadcastd         m0, [ypxq+strideq*1+12]
    vinserti128          m1, [ypxq+strideq*0+ 0], 0
    vinserti128          m0, [ypxq+strideq*1+ 0], 0
    lea                ypxq, [ypxq+strideq*2]
    pmaddwd              m1, m5
    pmaddwd              m0, m5
    paddd                m4, m1
    packssdw             m1, m0
    paddd                m4, m0
    vpermq               m2, m1, q3120
    mova              [acq], m2
    add                 acq, 32
    sub                  hd, 2
    jg .w8_wpad1
    jmp .w8_hpad
.w16:
    mov                  r5, acq
    test              wpadd, wpadd
    jnz .w16_wpad
.w16_loop:
    pmaddwd              m2, m5, [ypxq+strideq*0+ 0]
    pmaddwd              m1, m5, [ypxq+strideq*0+32]
    pmaddwd              m0, m5, [ypxq+strideq*1+ 0]
    pmaddwd              m3, m5, [ypxq+strideq*1+32]
    lea                ypxq, [ypxq+strideq*2]
    paddd                m4, m2
    packssdw             m2, m1
    paddd                m4, m1
    packssdw             m1, m0, m3
    paddd                m0, m3
    vpermq               m2, m2, q3120
    paddd                m4, m0
    vpermq               m1, m1, q3120
    mova         [acq+32*0], m2
    mova         [acq+32*1], m1
    add                 acq, 32*2
    sub                  hd, 2
    jg .w16_loop
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).w16_hpad
.w16_wpad:
    mova                 m2, [ypxq+strideq*0+ 0]
    mova                 m0, [ypxq+strideq*1+ 0]
    cmp               wpadd, 2
    jl .w16_wpad1
    je .w16_wpad2
    vpbroadcastd         m1, [ypxq+strideq*0+12]
    vpbroadcastd         m3, [ypxq+strideq*1+12]
    vpblendd             m2, m1, 0xf0
    vpblendd             m0, m3, 0xf0
    jmp .w16_wpad_end
.w16_wpad2:
    vpbroadcastd         m1, [ypxq+strideq*0+28]
    vpbroadcastd         m3, [ypxq+strideq*1+28]
    jmp .w16_wpad_end
.w16_wpad1:
    vpbroadcastd         m1, [ypxq+strideq*0+44]
    vpbroadcastd         m3, [ypxq+strideq*1+44]
    vinserti128          m1, [ypxq+strideq*0+32], 0
    vinserti128          m3, [ypxq+strideq*1+32], 0
.w16_wpad_end:
    lea                ypxq, [ypxq+strideq*2]
    REPX    {pmaddwd x, m5}, m2, m0, m1, m3
    paddd                m4, m2
    packssdw             m2, m1
    paddd                m4, m1
    packssdw             m1, m0, m3
    paddd                m0, m3
    vpermq               m2, m2, q3120
    paddd                m4, m0
    vpermq               m1, m1, q3120
    mova         [acq+32*0], m2
    mova         [acq+32*1], m1
    add                 acq, 32*2
    sub                  hd, 2
    jg .w16_wpad
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).w16_hpad

cglobal ipred_cfl_ac_444_16bpc, 4, 7, 6, ac, ypx, stride, wpad, hpad, w, h
    lea                  r6, [ipred_cfl_ac_444_16bpc_avx2_table]
    tzcnt                wd, wm
    movifnidn         hpadd, hpadm
    vpbroadcastd         m5, [pw_1]
    movsxd               wq, [r6+wq*4]
    shl               hpadd, 2
    add                  wq, r6
    mov                  hd, hm
    pxor                 m4, m4
    sub                  hd, hpadd
    jmp                  wq
.w4:
    lea                  r3, [strideq*3]
    mov                  r5, acq
.w4_loop:
    movq                xm0, [ypxq+strideq*0]
    movhps              xm0, [ypxq+strideq*1]
    vpbroadcastq         m1, [ypxq+strideq*2]
    vpbroadcastq         m2, [ypxq+r3       ]
    lea                ypxq, [ypxq+strideq*4]
    vpblendd             m0, m1, 0x30
    vpblendd             m0, m2, 0xc0
    psllw                m0, 3
    pmaddwd              m1, m0, m5
    mova              [acq], m0
    add                 acq, 32
    paddd                m4, m1
    sub                  hd, 4
    jg .w4_loop
    test              hpadd, hpadd
    jz mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
    vpermq               m0, m0, q3333
    paddd                m1, m1
    mova         [acq+32*0], m0
    vpermq               m1, m1, q3333
    mova         [acq+32*1], m0
    add                 acq, 32*2
    paddd                m4, m1
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
.w8:
    lea                  r3, [strideq*3]
    mov                  r5, acq
.w8_loop:
    mova                xm2, [ypxq+strideq*0]
    vinserti128          m2, [ypxq+strideq*1], 1
    mova                xm1, [ypxq+strideq*2]
    vinserti128          m1, [ypxq+r3       ], 1
    lea                ypxq, [ypxq+strideq*4]
    psllw                m2, 3
    psllw                m1, 3
    mova         [acq+32*0], m2
    pmaddwd              m2, m5
    mova         [acq+32*1], m1
    pmaddwd              m0, m1, m5
    add                 acq, 32*2
    paddd                m4, m2
    paddd                m4, m0
    sub                  hd, 4
    jg .w8_loop
    test              hpadd, hpadd
    jz mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
    vperm2i128           m1, m1, 0x11
    pslld                m0, 2
    pxor                 m2, m2
    vpblendd             m0, m2, 0x0f
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).hpad
.w16_wpad2:
    vpbroadcastw         m3, [ypxq+strideq*0+14]
    vpbroadcastw         m0, [ypxq+strideq*1+14]
    vpblendd             m2, m3, 0xf0
    vpblendd             m1, m0, 0xf0
    jmp .w16_wpad_end
.w16:
    mov                  r5, acq
.w16_loop:
    mova                 m2, [ypxq+strideq*0]
    mova                 m1, [ypxq+strideq*1]
    test              wpadd, wpadd
    jnz .w16_wpad2
.w16_wpad_end:
    lea                ypxq, [ypxq+strideq*2]
    psllw                m2, 3
    psllw                m1, 3
    mova         [acq+32*0], m2
    pmaddwd              m2, m5
    mova         [acq+32*1], m1
    pmaddwd              m0, m1, m5
    add                 acq, 32*2
    paddd                m4, m2
    paddd                m4, m0
    sub                  hd, 2
    jg .w16_loop
    add               hpadd, hpadd
    jz mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
    paddd                m0, m0
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).hpad
.w32:
    mov                  r5, acq
    test              wpadd, wpadd
    jnz .w32_wpad
.w32_loop:
    mova                 m0, [ypxq+ 0]
    mova                 m1, [ypxq+32]
    add                ypxq, strideq
    psllw                m0, 3
    psllw                m1, 3
    pmaddwd              m2, m0, m5
    mova         [acq+32*0], m0
    pmaddwd              m3, m1, m5
    mova         [acq+32*1], m1
    add                 acq, 32*2
    paddd                m2, m3
    paddd                m4, m2
    dec                  hd
    jg .w32_loop
.w32_hpad:
    test              hpadd, hpadd
    jz mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
    paddd                m2, m2
.w32_hpad_loop:
    mova         [acq+32*0], m0
    mova         [acq+32*1], m1
    paddd                m4, m2
    mova         [acq+32*2], m0
    mova         [acq+32*3], m1
    add                 acq, 32*4
    sub               hpadd, 2
    jg .w32_hpad_loop
    jmp mangle(private_prefix %+ _ipred_cfl_ac_420_16bpc_avx2).dc
.w32_wpad:
    mova                 m0, [ypxq+ 0]
    cmp               wpadd, 4
    jl .w32_wpad2
    je .w32_wpad4
    vpbroadcastw         m1, [ypxq+14]
    vpblendd             m0, m1, 0xf0
    jmp .w32_wpad_end
.w32_wpad4:
    vpbroadcastw         m1, [ypxq+30]
    jmp .w32_wpad_end
.w32_wpad2:
    vpbroadcastw         m1, [ypxq+46]
    vinserti128          m1, [ypxq+32], 0
.w32_wpad_end:
    add                ypxq, strideq
    psllw                m0, 3
    psllw                m1, 3
    pmaddwd              m2, m0, m5
    mova         [acq+32*0], m0
    pmaddwd              m3, m1, m5
    mova         [acq+32*1], m1
    add                 acq, 32*2
    paddd                m2, m3
    paddd                m4, m2
    dec                  hd
    jg .w32_wpad
    jmp .w32_hpad

cglobal pal_pred_16bpc, 4, 6, 5, dst, stride, pal, idx, w, h
    vbroadcasti128       m3, [palq]
    lea                  r2, [pal_pred_16bpc_avx2_table]
    tzcnt                wd, wm
    vbroadcasti128       m4, [pal_pred_shuf]
    movifnidn            hd, hm
    movsxd               wq, [r2+wq*4]
    pshufb               m3, m4
    punpckhqdq           m4, m3, m3
    add                  wq, r2
DEFINE_ARGS dst, stride, stride3, idx, w, h
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    mova                xm2, [idxq]
    add                idxq, 16
    pshufb              xm1, xm3, xm2
    pshufb              xm2, xm4, xm2
    punpcklbw           xm0, xm1, xm2
    punpckhbw           xm1, xm2
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+strideq*1], xm0
    movhps [dstq+stride3q ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4
    RET
.w8:
    movu                 m2, [idxq] ; only 16-byte alignment
    add                idxq, 32
    pshufb               m1, m3, m2
    pshufb               m2, m4, m2
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    mova         [dstq+strideq*0], xm0
    mova         [dstq+strideq*1], xm1
    vextracti128 [dstq+strideq*2], m0, 1
    vextracti128 [dstq+stride3q ], m1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8
    RET
.w16:
    vpermq               m2, [idxq+ 0], q3120
    vpermq               m5, [idxq+32], q3120
    add                idxq, 64
    pshufb               m1, m3, m2
    pshufb               m2, m4, m2
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    pshufb               m1, m3, m5
    pshufb               m2, m4, m5
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    mova   [dstq+strideq*2], m0
    mova   [dstq+stride3q ], m1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16
    RET
.w32:
    vpermq               m2, [idxq+ 0], q3120
    vpermq               m5, [idxq+32], q3120
    add                idxq, 64
    pshufb               m1, m3, m2
    pshufb               m2, m4, m2
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    mova [dstq+strideq*0+ 0], m0
    mova [dstq+strideq*0+32], m1
    pshufb               m1, m3, m5
    pshufb               m2, m4, m5
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    mova [dstq+strideq*1+ 0], m0
    mova [dstq+strideq*1+32], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32
    RET
.w64:
    vpermq               m2, [idxq+ 0], q3120
    vpermq               m5, [idxq+32], q3120
    add                idxq, 64
    pshufb               m1, m3, m2
    pshufb               m2, m4, m2
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    mova          [dstq+ 0], m0
    mova          [dstq+32], m1
    pshufb               m1, m3, m5
    pshufb               m2, m4, m5
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    mova          [dstq+64], m0
    mova          [dstq+96], m1
    add                 dstq, strideq
    dec                   hd
    jg .w64
    RET

%endif
