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

SECTION_RODATA 64

%macro SMOOTH_WEIGHT_TABLE 1-*
    %rep %0
        db %1-128, 127-%1
        %rotate 1
    %endrep
%endmacro

; sm_weights[], but modified to precalculate x and 256-x with offsets to
; enable efficient use of pmaddubsw (which requires signed values)
smooth_weights: SMOOTH_WEIGHT_TABLE         \
      0,   0, 255, 128, 255, 149,  85,  64, \
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

pb_1to32:     db  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16
              db 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
pb_32to1:     db 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17
pb_16to1:     db 16, 15, 14, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1
z_filter_wh:  db  7,  7, 11, 11, 15, 15, 19, 19, 19, 23, 23, 23, 31, 31, 31, 39
              db 39, 39, 47, 47, 47, 63, 63, 63, 79, 79, 79, -1
z_filter_k:   db  0, 16,  0, 16,  0, 20,  0, 20,  8, 16,  8, 16
              db 32, 16, 32, 16, 24, 20, 24, 20, 16, 16, 16, 16
              db  0,  0,  0,  0,  0,  0,  0,  0,  8,  0,  8,  0
z_filter_s:   db  0,  0,  0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,  7
              db  7,  8,  8,  9,  9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15
              db 15, 15, 15, 15, 15, 15, 15, 15 ; should be in one cache line
pb_128:       times 4 db 128 ; those are just placed here for alignment.
pb_36_m4:     times 2 db 36, -4
z3_shuf:      db  8,  7,  7,  6,  6,  5,  5,  4,  4,  3,  3,  2,  2,  1,  1,  0
z_filter_t0:  db 55,127, 39,127, 39,127,  7, 15, 31,  7, 15, 31,  0,  3, 31,  0
z_filter_t1:  db 39, 63, 19, 47, 19, 47,  3,  3,  3,  3,  3,  3,  0,  0,  0,  0
z_upsample1:  db  1,  0,  2,  1,  3,  2,  4,  3,  5,  4,  6,  5,  7,  6,  8,  7
z_upsample2:  db  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8,  8,  8,  8,  8
z2_upsample:  db  7,  6, 15, 14,  5,  4, 13, 12,  3,  2, 11, 10,  1,  0,  9,  8
z1_shuf_w4:   db  0,  1,  1,  2,  2,  3,  3,  4,  8,  9,  9, 10, 10, 11, 11, 12
z2_shuf_h2:   db  3,  2,  7,  6, 11, 10, 15, 14,  2,  1,  6,  5, 10,  9, 14, 13
z2_shuf_h4:   db  7,  6, 15, 14,  6,  5, 14, 13,  5,  4, 13, 12,  4,  3, 12, 11
z3_shuf_w4:   db  4,  3,  3,  2,  2,  1,  1,  0, 12, 11, 11, 10, 10,  9,  9,  8
z_transpose4: db  0,  4,  8, 12,  1,  5,  9, 13,  2,  6, 10, 14,  3,  7, 11, 15
z_base_inc:   dw   0*64,   1*64,   2*64,   3*64,   4*64,   5*64,   6*64,   7*64
              dw  16*64,  17*64,  18*64,  19*64,  20*64,  21*64,  22*64,  23*64
z2_base_inc:  dw   1*64,   2*64,   3*64,   4*64,   5*64,   6*64,   7*64,   8*64
              dw   9*64,  10*64,  11*64,  12*64,  13*64,  14*64,  15*64,  16*64
z2_ymul:      dw  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16
z2_y_shuf_h4: db 90, 90, 90, 90, 14, 14, 14, 14, 27, 27, 27, 27, 31, 31, 31, 31 ; 2, 6, 3, 7
              db 32, 32, 32, 32, 12, 12, 12, 12,  1,  0,  1,  0,  5, -1, -1, -1 ; 0, 4, 1, 5
; vpermd indices in bits 4..6 of filter_shuf1: 0, 2, 6, 4, 1, 3, 7, 5
filter_shuf1: db 10,  4, 10,  4, 37,  6,  5,  6,103,  9,  7,  9, 72, -1,  8, -1
              db 16,  4,  0,  4, 53,  6,  5,  6,119, 11,  7, 11, 95, -1, 15, -1
filter_shuf2: db  3,  4,  3,  4,  5,  6,  5,  6,  7,  2,  7,  2,  1, -1,  1, -1
filter_shuf3: db  3,  4,  3,  4,  5,  6,  5,  6,  7, 11,  7, 11; 15, -1, 15, -1
pb_127_m127:  times 2 db 127, -127
ipred_v_shuf: db  0,  1,  0,  1,  4,  5,  4,  5,  8,  9,  8,  9, 12, 13, 12, 13
              db  2,  3,  2,  3,  6,  7,  6,  7, 10, 11, 10, 11, 14, 15, 14, 15
ipred_h_shuf: db  7,  7,  7,  7,  3,  3,  3,  3,  5,  5,  5,  5,  1,  1,  1,  1
              db  6,  6,  6,  6,  2,  2,  2,  2,  4,  4,  4,  4;  0,  0,  0,  0
pw_64:        times 2 dw 64

cfl_ac_444_w16_pad1_shuffle: db 0, -1, 1, -1, 2, -1, 3, -1, 4, -1, 5, -1, 6, -1
                             times 9 db 7, -1
cfl_ac_w16_pad_shuffle: ; w=16, w_pad=1
                        db 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
                        ; w=8, w_pad=1 as well as second half of previous one
cfl_ac_w8_pad1_shuffle: db 0, 1, 2, 3, 4, 5
                        times 5 db 6, 7
                        ; w=16,w_pad=2
                        db 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
                        times 8 db 14, 15
                        ; w=16,w_pad=3
                        db 0, 1, 2, 3, 4, 5
                        times 13 db 6, 7
pb_15to0:               db 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

%define pb_0to15 cfl_ac_w16_pad_shuffle
%define pb_1  (ipred_h_shuf+12)
%define pb_2  (ipred_h_shuf+20)
%define pb_3  (ipred_h_shuf+ 4)
%define pb_4  (ipred_h_shuf+24)
%define pb_5  (ipred_h_shuf+ 8)
%define pb_7  (ipred_h_shuf+ 0)
%define pb_8  (z_upsample2 +12)
%define pb_12 (z2_y_shuf_h4+20)
%define pb_14 (z2_y_shuf_h4+ 4)
%define pb_15 (z_filter_s  +32)
%define pb_27 (z2_y_shuf_h4+ 8)
%define pb_31 (z2_y_shuf_h4+12)
%define pb_32 (z2_y_shuf_h4+16)
%define pb_90 (z2_y_shuf_h4+ 0)
%define pw_1  (z2_y_shuf_h4+24)
%define pw_8  (z_filter_k  +32)

pw_62:    times 2 dw 62
pw_128:   times 2 dw 128
pw_255:   times 2 dw 255
pw_512:   times 2 dw 512

%macro JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - 2*4)
    %xdefine %%base mangle(private_prefix %+ _%1_8bpc_%2)
    %%table:
    %rep %0 - 2
        dd %%base %+ .%3 - (%%table - 2*4)
        %rotate 1
    %endrep
%endmacro

%define ipred_dc_splat_avx2_table (ipred_dc_avx2_table + 10*4)
%define ipred_cfl_splat_avx2_table (ipred_cfl_avx2_table + 8*4)

JMP_TABLE ipred_smooth,     avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_v,   avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_h,   avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_paeth,      avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_filter,     avx2, w4, w8, w16, w32
JMP_TABLE ipred_dc,         avx2, h4, h8, h16, h32, h64, w4, w8, w16, w32, w64, \
                                  s4-10*4, s8-10*4, s16-10*4, s32-10*4, s64-10*4
JMP_TABLE ipred_dc_left,    avx2, h4, h8, h16, h32, h64
JMP_TABLE ipred_h,          avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_z1,         avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_z2,         avx2, w4, w8, w16, w32, w64
JMP_TABLE ipred_z3,         avx2, h4, h8, h16, h32, h64
JMP_TABLE ipred_cfl,        avx2, h4, h8, h16, h32, w4, w8, w16, w32, \
                                  s4-8*4, s8-8*4, s16-8*4, s32-8*4
JMP_TABLE ipred_cfl_left,   avx2, h4, h8, h16, h32
JMP_TABLE ipred_cfl_ac_420, avx2, w16_pad1, w16_pad2, w16_pad3
JMP_TABLE ipred_cfl_ac_422, avx2, w16_pad1, w16_pad2, w16_pad3
JMP_TABLE ipred_cfl_ac_444, avx2, w32_pad1, w32_pad2, w32_pad3, w4, w8, w16, w32
JMP_TABLE pal_pred,         avx2, w4, w8, w16, w32, w64

cextern dr_intra_derivative
cextern filter_intra_taps

SECTION .text

INIT_YMM avx2
cglobal ipred_dc_top_8bpc, 3, 7, 6, dst, stride, tl, w, h
    lea                  r5, [ipred_dc_left_avx2_table]
    tzcnt                wd, wm
    inc                 tlq
    movu                 m0, [tlq]
    movifnidn            hd, hm
    mov                 r6d, 0x8000
    shrx                r6d, r6d, wd
    movd                xm3, r6d
    movsxd               r6, [r5+wq*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, r5
    add                  r5, ipred_dc_splat_avx2_table-ipred_dc_left_avx2_table
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6

cglobal ipred_dc_left_8bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    mov                  hd, hm ; zero upper half
    tzcnt               r6d, hd
    sub                 tlq, hq
    tzcnt                wd, wm
    movu                 m0, [tlq]
    mov                 r5d, 0x8000
    shrx                r5d, r5d, r6d
    movd                xm3, r5d
    lea                  r5, [ipred_dc_left_avx2_table]
    movsxd               r6, [r5+r6*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, r5
    add                  r5, ipred_dc_splat_avx2_table-ipred_dc_left_avx2_table
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6
.h64:
    movu                 m1, [tlq+32] ; unaligned when jumping here from dc_top
    pmaddubsw            m1, m2
    paddw                m0, m1
.h32:
    vextracti128        xm1, m0, 1
    paddw               xm0, xm1
.h16:
    punpckhqdq          xm1, xm0, xm0
    paddw               xm0, xm1
.h8:
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
.h4:
    pmaddwd             xm0, xm2
    pmulhrsw            xm0, xm3
    lea            stride3q, [strideq*3]
    vpbroadcastb         m0, xm0
    mova                 m1, m0
    jmp                  wq

cglobal ipred_dc_8bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    movifnidn            hd, hm
    movifnidn            wd, wm
    tzcnt               r6d, hd
    lea                 r5d, [wq+hq]
    movd                xm4, r5d
    tzcnt               r5d, r5d
    movd                xm5, r5d
    lea                  r5, [ipred_dc_avx2_table]
    tzcnt                wd, wd
    movsxd               r6, [r5+r6*4]
    movsxd               wq, [r5+wq*4+5*4]
    pcmpeqd              m3, m3
    psrlw               xm4, 1
    add                  r6, r5
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  r6
.h4:
    movd                xm0, [tlq-4]
    pmaddubsw           xm0, xm3
    jmp                  wq
.w4:
    movd                xm1, [tlq+1]
    pmaddubsw           xm1, xm3
    psubw               xm0, xm4
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    cmp                  hd, 4
    jg .w4_mul
    psrlw               xm0, 3
    jmp .w4_end
.w4_mul:
    punpckhqdq          xm1, xm0, xm0
    lea                 r2d, [hq*2]
    mov                 r6d, 0x55563334
    paddw               xm0, xm1
    shrx                r6d, r6d, r2d
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    movd                xm1, r6d
    psrlw               xm0, 2
    pmulhuw             xm0, xm1
.w4_end:
    vpbroadcastb        xm0, xm0
.s4:
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm0
    movd   [dstq+strideq*2], xm0
    movd   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s4
    RET
ALIGN function_align
.h8:
    movq                xm0, [tlq-8]
    pmaddubsw           xm0, xm3
    jmp                  wq
.w8:
    movq                xm1, [tlq+1]
    vextracti128        xm2, m0, 1
    pmaddubsw           xm1, xm3
    psubw               xm0, xm4
    paddw               xm0, xm2
    punpckhqdq          xm2, xm0, xm0
    paddw               xm0, xm2
    paddw               xm0, xm1
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    psrlw               xm0, xm5
    cmp                  hd, 8
    je .w8_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    cmp                  hd, 32
    cmove               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
.w8_end:
    vpbroadcastb        xm0, xm0
.s8:
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm0
    movq   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s8
    RET
ALIGN function_align
.h16:
    mova                xm0, [tlq-16]
    pmaddubsw           xm0, xm3
    jmp                  wq
.w16:
    movu                xm1, [tlq+1]
    vextracti128        xm2, m0, 1
    pmaddubsw           xm1, xm3
    psubw               xm0, xm4
    paddw               xm0, xm2
    paddw               xm0, xm1
    punpckhqdq          xm1, xm0, xm0
    paddw               xm0, xm1
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    psrlw               xm0, xm5
    cmp                  hd, 16
    je .w16_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    test                 hb, 8|32
    cmovz               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
.w16_end:
    vpbroadcastb        xm0, xm0
.s16:
    mova   [dstq+strideq*0], xm0
    mova   [dstq+strideq*1], xm0
    mova   [dstq+strideq*2], xm0
    mova   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s16
    RET
ALIGN function_align
.h32:
    mova                 m0, [tlq-32]
    pmaddubsw            m0, m3
    jmp                  wq
.w32:
    movu                 m1, [tlq+1]
    pmaddubsw            m1, m3
    paddw                m0, m1
    vextracti128        xm1, m0, 1
    psubw               xm0, xm4
    paddw               xm0, xm1
    punpckhqdq          xm1, xm0, xm0
    paddw               xm0, xm1
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    psrlw               xm0, xm5
    cmp                  hd, 32
    je .w32_end
    lea                 r2d, [hq*2]
    mov                 r6d, 0x33345556
    shrx                r6d, r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
.w32_end:
    vpbroadcastb         m0, xm0
.s32:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m0
    mova   [dstq+strideq*2], m0
    mova   [dstq+stride3q ], m0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s32
    RET
ALIGN function_align
.h64:
    mova                 m0, [tlq-64]
    mova                 m1, [tlq-32]
    pmaddubsw            m0, m3
    pmaddubsw            m1, m3
    paddw                m0, m1
    jmp                  wq
.w64:
    movu                 m1, [tlq+ 1]
    movu                 m2, [tlq+33]
    pmaddubsw            m1, m3
    pmaddubsw            m2, m3
    paddw                m0, m1
    paddw                m0, m2
    vextracti128        xm1, m0, 1
    psubw               xm0, xm4
    paddw               xm0, xm1
    punpckhqdq          xm1, xm0, xm0
    paddw               xm0, xm1
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    psrlw               xm0, xm5
    cmp                  hd, 64
    je .w64_end
    mov                 r6d, 0x33345556
    shrx                r6d, r6d, hd
    movd                xm1, r6d
    pmulhuw             xm0, xm1
.w64_end:
    vpbroadcastb         m0, xm0
    mova                 m1, m0
.s64:
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
    jg .s64
    RET

cglobal ipred_dc_128_8bpc, 2, 7, 6, dst, stride, tl, w, h, stride3
    lea                  r5, [ipred_dc_splat_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    vpbroadcastd         m0, [r5-ipred_dc_splat_avx2_table+pb_128]
    mova                 m1, m0
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

cglobal ipred_v_8bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    lea                  r5, [ipred_dc_splat_avx2_table]
    tzcnt                wd, wm
    movu                 m0, [tlq+ 1]
    movu                 m1, [tlq+33]
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

%macro IPRED_H 2 ; w, store_type
    vpbroadcastb         m0, [tlq-1]
    vpbroadcastb         m1, [tlq-2]
    vpbroadcastb         m2, [tlq-3]
    sub                 tlq, 4
    vpbroadcastb         m3, [tlq+0]
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

INIT_XMM avx2
cglobal ipred_h_8bpc, 3, 6, 4, dst, stride, tl, w, h, stride3
    lea                  r5, [ipred_h_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    IPRED_H               4, d
.w8:
    IPRED_H               8, q
.w16:
    IPRED_H              16, a
INIT_YMM avx2
.w32:
    IPRED_H              32, a
.w64:
    vpbroadcastb         m0, [tlq-1]
    vpbroadcastb         m1, [tlq-2]
    vpbroadcastb         m2, [tlq-3]
    sub                 tlq, 4
    vpbroadcastb         m3, [tlq+0]
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
    jg .w64
    RET

%macro PAETH 2 ; top, ldiff
    pavgb                m1, m%1, m3 ; Calculating tldiff normally requires
    pxor                 m0, m%1, m3 ; 10-bit intermediates, but we can do it
    pand                 m0, m4      ; in 8-bit with some tricks which avoids
    psubusb              m2, m5, m1  ; having to unpack everything to 16-bit.
    psubb                m1, m0
    psubusb              m1, m5
    por                  m1, m2
    paddusb              m1, m1
    por                  m1, m0      ; min(tldiff, 255)
    psubusb              m2, m5, m3
    psubusb              m0, m3, m5
    por                  m2, m0      ; tdiff
    pminub               m2, m%2
    pcmpeqb              m0, m%2, m2 ; ldiff <= tdiff
    vpblendvb            m0, m%1, m3, m0
    pminub               m1, m2
    pcmpeqb              m1, m2      ; ldiff <= tldiff || tdiff <= tldiff
    vpblendvb            m0, m5, m0, m1
%endmacro

cglobal ipred_paeth_8bpc, 3, 6, 9, dst, stride, tl, w, h
%define base r5-ipred_paeth_avx2_table
    lea                  r5, [ipred_paeth_avx2_table]
    tzcnt                wd, wm
    vpbroadcastb         m5, [tlq]   ; topleft
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    vpbroadcastd         m4, [base+pb_1]
    add                  wq, r5
    jmp                  wq
.w4:
    vpbroadcastd         m6, [tlq+1] ; top
    mova                 m8, [base+ipred_h_shuf]
    lea                  r3, [strideq*3]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0      ; ldiff
.w4_loop:
    sub                 tlq, 8
    vpbroadcastq         m3, [tlq]
    pshufb               m3, m8      ; left
    PAETH                 6, 7
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+r3       ], xm1, 2
    cmp                  hd, 4
    je .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm0, 3
    pextrd [dstq+r3       ], xm1, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 8
    jg .w4_loop
.ret:
    RET
ALIGN function_align
.w8:
    vpbroadcastq         m6, [tlq+1]
    mova                 m8, [base+ipred_h_shuf]
    lea                  r3, [strideq*3]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w8_loop:
    sub                 tlq, 4
    vpbroadcastd         m3, [tlq]
    pshufb               m3, m8
    PAETH                 6, 7
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r3       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    vbroadcasti128       m6, [tlq+1]
    mova                xm8, xm4 ; lower half = 1, upper half = 0
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w16_loop:
    sub                 tlq, 2
    vpbroadcastd         m3, [tlq]
    pshufb               m3, m8
    PAETH                 6, 7
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    movu                 m6, [tlq+1]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w32_loop:
    dec                 tlq
    vpbroadcastb         m3, [tlq]
    PAETH                 6, 7
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w32_loop
    RET
ALIGN function_align
.w64:
    movu                 m6, [tlq+ 1]
    movu                 m7, [tlq+33]
%if WIN64
    movaps              r4m, xmm9
%endif
    psubusb              m8, m5, m6
    psubusb              m0, m6, m5
    psubusb              m9, m5, m7
    psubusb              m1, m7, m5
    por                  m8, m0
    por                  m9, m1
.w64_loop:
    dec                 tlq
    vpbroadcastb         m3, [tlq]
    PAETH                 6, 8
    mova        [dstq+32*0], m0
    PAETH                 7, 9
    mova        [dstq+32*1], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
%if WIN64
    movaps             xmm9, r4m
%endif
    RET

%macro SMOOTH 6 ; src[1-2], mul[1-2], add[1-2]
    ; w * a         = (w - 128) * a + 128 * a
    ; (256 - w) * b = (127 - w) * b + 129 * b
    pmaddubsw            m0, m%3, m%1
    pmaddubsw            m1, m%4, m%2
    paddw                m0, m%5
    paddw                m1, m%6
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
%endmacro

cglobal ipred_smooth_v_8bpc, 3, 7, 0, dst, stride, tl, w, h, weights
%define base r6-ipred_smooth_v_avx2_table
    lea                  r6, [ipred_smooth_v_avx2_table]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m0, [base+pb_127_m127]
    vpbroadcastd         m1, [base+pw_128]
    lea            weightsq, [base+smooth_weights+hq*4]
    neg                  hq
    vpbroadcastb         m5, [tlq+hq] ; bottom
    add                  wq, r6
    jmp                  wq
.w4:
    vpbroadcastd         m2, [tlq+1]
    punpcklbw            m2, m5 ; top, bottom
    mova                 m5, [base+ipred_v_shuf]
    lea                  r3, [strideq*3]
    punpckldq            m4, m5, m5
    punpckhdq            m5, m5
    pmaddubsw            m3, m2, m0
    paddw                m1, m2 ;   1 * top + 256 * bottom + 128, overflow is ok
    paddw                m3, m1 ; 128 * top + 129 * bottom + 128
.w4_loop:
    vbroadcasti128       m1, [weightsq+hq*2]
    pshufb               m0, m1, m4
    pshufb               m1, m5
    SMOOTH                0, 1, 2, 2, 3, 3
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm1
    pextrd [dstq+strideq*2], xm0, 1
    pextrd [dstq+r3       ], xm1, 1
    cmp                  hd, -4
    je .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm1, 2
    pextrd [dstq+strideq*2], xm0, 3
    pextrd [dstq+r3       ], xm1, 3
    lea                dstq, [dstq+strideq*4]
    add                  hq, 8
    jl .w4_loop
.ret:
    RET
ALIGN function_align
.w8:
    vpbroadcastq         m2, [tlq+1]
    punpcklbw            m2, m5
    mova                 m5, [base+ipred_v_shuf]
    lea                  r3, [strideq*3]
    pshufd               m4, m5, q0000
    pshufd               m5, m5, q1111
    pmaddubsw            m3, m2, m0
    paddw                m1, m2
    paddw                m3, m1
.w8_loop:
    vpbroadcastq         m1, [weightsq+hq*2]
    pshufb               m0, m1, m4
    pshufb               m1, m5
    SMOOTH                0, 1, 2, 2, 3, 3
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r3       ], xm1
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w8_loop
    RET
ALIGN function_align
.w16:
    WIN64_SPILL_XMM       7
    vbroadcasti128       m3, [tlq+1]
    mova                 m6, [base+ipred_v_shuf]
    punpcklbw            m2, m3, m5
    punpckhbw            m3, m5
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1
.w16_loop:
    vpbroadcastd         m1, [weightsq+hq*2]
    pshufb               m1, m6
    SMOOTH                1, 1, 2, 3, 4, 5
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    add                  hq, 2
    jl .w16_loop
    RET
ALIGN function_align
.w32:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM       6
    movu                 m3, [tlq+1]
    punpcklbw            m2, m3, m5
    punpckhbw            m3, m5
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1
.w32_loop:
    vpbroadcastw         m1, [weightsq+hq*2]
    SMOOTH                1, 1, 2, 3, 4, 5
    mova             [dstq], m0
    add                dstq, strideq
    inc                  hq
    jl .w32_loop
    RET
ALIGN function_align
.w64:
    WIN64_SPILL_XMM      11
    movu                 m4, [tlq+ 1]
    movu                 m8, [tlq+33]
    punpcklbw            m3, m4, m5
    punpckhbw            m4, m5
    punpcklbw            m7, m8, m5
    punpckhbw            m8, m5
    pmaddubsw            m5, m3, m0
    pmaddubsw            m6, m4, m0
    pmaddubsw            m9, m7, m0
    pmaddubsw           m10, m8, m0
    paddw                m2, m1, m3
    paddw                m5, m2
    paddw                m2, m1, m4
    paddw                m6, m2
    paddw                m0, m1, m7
    paddw                m9, m0
    paddw                m1, m8
    paddw               m10, m1
.w64_loop:
    vpbroadcastw         m2, [weightsq+hq*2]
    SMOOTH                2, 2, 3, 4, 5, 6
    mova        [dstq+32*0], m0
    SMOOTH                2, 2, 7, 8, 9, 10
    mova        [dstq+32*1], m0
    add                dstq, strideq
    inc                  hq
    jl .w64_loop
    RET

%macro SETUP_STACK_FRAME 3 ; stack_size, regs_used, xmm_regs_used
    %assign stack_offset 0
    %assign stack_size_padded 0
    %assign regs_used %2
    %xdefine rstk rsp
    SETUP_STACK_POINTER %1
    %if regs_used != %2 && WIN64
        PUSH r%2
    %endif
    ALLOC_STACK %1, %3
%endmacro

cglobal ipred_smooth_h_8bpc, 3, 7, 0, dst, stride, tl, w, h
%define base r6-ipred_smooth_h_avx2_table
    lea                  r6, [ipred_smooth_h_avx2_table]
    mov                  wd, wm
    vpbroadcastb         m3, [tlq+wq] ; right
    tzcnt                wd, wd
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m4, [base+pb_127_m127]
    vpbroadcastd         m5, [base+pw_128]
    add                  wq, r6
    jmp                  wq
.w4:
    WIN64_SPILL_XMM       8
    vpbroadcastq         m6, [base+smooth_weights+4*2]
    mova                 m7, [base+ipred_h_shuf]
    sub                 tlq, 8
    sub                 tlq, hq
    lea                  r3, [strideq*3]
.w4_loop:
    vpbroadcastq         m2, [tlq+hq]
    pshufb               m2, m7
    punpcklbw            m1, m2, m3 ; left, right
    punpckhbw            m2, m3
    pmaddubsw            m0, m1, m4 ; 127 * left - 127 * right
    paddw                m0, m1     ; 128 * left + 129 * right
    pmaddubsw            m1, m6
    paddw                m1, m5
    paddw                m0, m1
    pmaddubsw            m1, m2, m4
    paddw                m1, m2
    pmaddubsw            m2, m6
    paddw                m2, m5
    paddw                m1, m2
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+r3       ], xm1, 2
    cmp                  hd, 4
    je .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm0, 3
    pextrd [dstq+r3       ], xm1, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 8
    jg .w4_loop
.ret:
    RET
ALIGN function_align
.w8:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM       8
    vbroadcasti128       m6, [base+smooth_weights+8*2]
    mova                 m7, [base+ipred_h_shuf]
    sub                 tlq, 4
    lea                  r3, [strideq*3]
    sub                 tlq, hq
.w8_loop:
    vpbroadcastd         m2, [tlq+hq]
    pshufb               m2, m7
    punpcklbw            m1, m2, m3
    punpckhbw            m2, m3
    pmaddubsw            m0, m1, m4
    paddw                m0, m1
    pmaddubsw            m1, m6
    paddw                m1, m5
    paddw                m0, m1
    pmaddubsw            m1, m2, m4
    paddw                m1, m2
    pmaddubsw            m2, m6
    paddw                m2, m5
    paddw                m1, m2
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r3       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    SETUP_STACK_FRAME  32*4, 7, 8
    lea                  r3, [rsp+64*2-4]
    call .prep ; only worthwhile for for w16 and above
    sub                 tlq, 2
    vpbroadcastd        xm6, [base+pb_1]
    mova                xm7, [base+ipred_v_shuf+16]
    vinserti128          m7, [base+ipred_v_shuf+ 0], 1
    vbroadcasti128       m4, [base+smooth_weights+16*2]
    vbroadcasti128       m5, [base+smooth_weights+16*3]
.w16_loop:
    vpbroadcastd         m1, [tlq+hq]
    vpbroadcastd         m2, [r3+hq*2]
    pshufb               m1, m6
    punpcklbw            m1, m3
    pshufb               m2, m7
    SMOOTH                4, 5, 1, 1, 2, 2
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    SETUP_STACK_FRAME  32*4, 7, 6
    lea                  r3, [rsp+64*2-2]
    call .prep
    dec                 tlq
    mova                xm4, [base+smooth_weights+16*4]
    vinserti128          m4, [base+smooth_weights+16*6], 1
    mova                xm5, [base+smooth_weights+16*5]
    vinserti128          m5, [base+smooth_weights+16*7], 1
.w32_loop:
    vpbroadcastb         m1, [tlq+hq]
    punpcklbw            m1, m3
    vpbroadcastw         m2, [r3+hq*2]
    SMOOTH                4, 5, 1, 1, 2, 2
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w32_loop
    RET
ALIGN function_align
.w64:
    SETUP_STACK_FRAME  32*4, 7, 9
    lea                  r3, [rsp+64*2-2]
    call .prep
    add                  r6, smooth_weights+16*15-ipred_smooth_h_avx2_table
    dec                 tlq
    mova                xm5, [r6-16*7]
    vinserti128          m5, [r6-16*5], 1
    mova                xm6, [r6-16*6]
    vinserti128          m6, [r6-16*4], 1
    mova                xm7, [r6-16*3]
    vinserti128          m7, [r6-16*1], 1
    mova                xm8, [r6-16*2]
    vinserti128          m8, [r6-16*0], 1
.w64_loop:
    vpbroadcastb         m2, [tlq+hq]
    punpcklbw            m2, m3
    vpbroadcastw         m4, [r3+hq*2]
    SMOOTH                5, 6, 2, 2, 4, 4
    mova        [dstq+32*0], m0
    SMOOTH                7, 8, 2, 2, 4, 4
    mova        [dstq+32*1], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET
ALIGN function_align
.prep:
    vpermq               m2, [tlq-32*1], q3120
    punpckhbw            m1, m2, m3
    punpcklbw            m2, m3
    pmaddubsw            m0, m1, m4 ; 127 * left - 127 * right
    paddw                m1, m5     ;   1 * left + 256 * right + 128
    paddw                m0, m1     ; 128 * left + 129 * right + 128
    pmaddubsw            m1, m2, m4
    paddw                m2, m5
    paddw                m1, m2
    vpermq               m2, [tlq-32*2], q3120
    mova [rsp+gprsize+32*3], m0
    mova [rsp+gprsize+32*2], m1
    punpckhbw            m1, m2, m3
    punpcklbw            m2, m3
    pmaddubsw            m0, m1, m4
    paddw                m1, m5
    paddw                m0, m1
    pmaddubsw            m1, m2, m4
    paddw                m2, m5
    paddw                m1, m2
    mova [rsp+gprsize+32*1], m0
    mova [rsp+gprsize+32*0], m1
    sub                  r3, hq
    sub                 tlq, hq
    sub                  r3, hq
    ret

%macro SMOOTH_2D_END 6 ; src[1-2], mul[1-2], add[1-2]
    pmaddubsw            m0, m%3, m%1
    pmaddubsw            m1, m%4, m%2
%ifnum %5
    paddw                m0, m%5
%else
    paddw                m0, %5
%endif
%ifnum %6
    paddw                m1, m%6
%else
    paddw                m1, %6
%endif
    pavgw                m0, m2
    pavgw                m1, m3
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
%endmacro

cglobal ipred_smooth_8bpc, 3, 7, 0, dst, stride, tl, w, h, v_weights
%define base r6-ipred_smooth_avx2_table
    lea                  r6, [ipred_smooth_avx2_table]
    mov                  wd, wm
    vpbroadcastb         m4, [tlq+wq] ; right
    tzcnt                wd, wd
    mov                  hd, hm
    mov                  r5, tlq
    sub                  r5, hq
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m5, [base+pb_127_m127]
    vpbroadcastb         m0, [r5] ; bottom
    vpbroadcastd         m3, [base+pw_255]
    add                  wq, r6
    lea          v_weightsq, [base+smooth_weights+hq*2]
    jmp                  wq
.w4:
    WIN64_SPILL_XMM      12
    mova                m10, [base+ipred_h_shuf]
    vpbroadcastq        m11, [base+smooth_weights+4*2]
    mova                 m7, [base+ipred_v_shuf]
    vpbroadcastd         m8, [tlq+1]
    sub                 tlq, 8
    lea                  r3, [strideq*3]
    sub                 tlq, hq
    punpcklbw            m8, m0 ; top, bottom
    pshufd               m6, m7, q2200
    pshufd               m7, m7, q3311
    pmaddubsw            m9, m8, m5
    paddw                m3, m8 ;   1 * top + 255 * bottom + 255
    paddw                m9, m3 ; 128 * top + 129 * bottom + 255
.w4_loop:
    vpbroadcastq         m1, [tlq+hq]
    pshufb               m1, m10
    punpcklbw            m0, m1, m4 ; left, right
    punpckhbw            m1, m4
    pmaddubsw            m2, m0, m5 ; 127 * left - 127 * right
    pmaddubsw            m3, m1, m5
    paddw                m2, m0     ; 128 * left + 129 * right
    paddw                m3, m1
    pmaddubsw            m0, m11
    pmaddubsw            m1, m11
    paddw                m2, m0
    paddw                m3, m1
    vbroadcasti128       m1, [v_weightsq]
    add          v_weightsq, 16
    pshufb               m0, m1, m6
    pshufb               m1, m7
    SMOOTH_2D_END         0, 1, 8, 8, 9, 9
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+r3       ], xm1, 2
    cmp                  hd, 4
    je .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm0, 3
    pextrd [dstq+r3       ], xm1, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 8
    jg .w4_loop
.ret:
    RET
ALIGN function_align
.w8:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      12
    mova                m10, [base+ipred_h_shuf]
    vbroadcasti128      m11, [base+smooth_weights+8*2]
    mova                 m7, [base+ipred_v_shuf]
    vpbroadcastq         m8, [tlq+1]
    sub                 tlq, 4
    lea                  r3, [strideq*3]
    sub                 tlq, hq
    punpcklbw            m8, m0
    pshufd               m6, m7, q0000
    pshufd               m7, m7, q1111
    pmaddubsw            m9, m8, m5
    paddw                m3, m8
    paddw                m9, m3
.w8_loop:
    vpbroadcastd         m1, [tlq+hq]
    pshufb               m1, m10
    punpcklbw            m0, m1, m4
    punpckhbw            m1, m4
    pmaddubsw            m2, m0, m5
    pmaddubsw            m3, m1, m5
    paddw                m2, m0
    paddw                m3, m1
    pmaddubsw            m0, m11
    pmaddubsw            m1, m11
    paddw                m2, m0
    paddw                m3, m1
    vpbroadcastq         m1, [v_weightsq]
    add          v_weightsq, 8
    pshufb               m0, m1, m6
    pshufb               m1, m7
    SMOOTH_2D_END         0, 1, 8, 8, 9, 9
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r3       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    SETUP_STACK_FRAME  32*4, 7, 14
    vbroadcasti128      m11, [tlq+1]
    lea                  r3, [rsp+64*2-4]
    punpcklbw           m10, m11, m0 ; top, bottom
    punpckhbw           m11, m0
    call .prep_v
    sub                 tlq, 2
    pmaddubsw           m12, m10, m5
    pmaddubsw           m13, m11, m5
    vpbroadcastd        xm5, [base+pb_1]
    mova                 m9, [base+ipred_v_shuf]
    vbroadcasti128       m6, [base+smooth_weights+16*2]
    vbroadcasti128       m7, [base+smooth_weights+16*3]
    vperm2i128           m8, m9, m9, 0x01
    paddw                m0, m10, m3
    paddw                m3, m11
    paddw               m12, m0
    paddw               m13, m3
.w16_loop:
    vpbroadcastd         m3, [tlq+hq]
    vpbroadcastd         m0, [r3+hq*2]
    vpbroadcastd         m1, [v_weightsq]
    add          v_weightsq, 4
    pshufb               m3, m5
    punpcklbw            m3, m4 ; left, right
    pmaddubsw            m2, m3, m6
    pmaddubsw            m3, m7
    pshufb               m0, m8
    pshufb               m1, m9
    paddw                m2, m0
    paddw                m3, m0
    SMOOTH_2D_END         1, 1, 10, 11, 12, 13
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    SETUP_STACK_FRAME  32*4, 7, 11
    movu                 m8, [tlq+1]
    lea                  r3, [rsp+64*2-2]
    punpcklbw            m7, m8, m0
    punpckhbw            m8, m0
    call .prep_v
    dec                 tlq
    pmaddubsw            m9, m7, m5
    pmaddubsw           m10, m8, m5
    mova                xm5, [base+smooth_weights+16*4]
    vinserti128          m5, [base+smooth_weights+16*6], 1
    mova                xm6, [base+smooth_weights+16*5]
    vinserti128          m6, [base+smooth_weights+16*7], 1
    paddw                m0, m7, m3
    paddw                m3, m8
    paddw                m9, m0
    paddw               m10, m3
.w32_loop:
    vpbroadcastb         m3, [tlq+hq]
    punpcklbw            m3, m4
    vpbroadcastw         m0, [r3+hq*2]
    vpbroadcastw         m1, [v_weightsq]
    add          v_weightsq, 2
    pmaddubsw            m2, m3, m5
    pmaddubsw            m3, m6
    paddw                m2, m0
    paddw                m3, m0
    SMOOTH_2D_END         1, 1, 7, 8, 9, 10
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w32_loop
    RET
ALIGN function_align
.w64:
    SETUP_STACK_FRAME  32*8, 7, 16
    movu                m13, [tlq+1 ]
    movu                m15, [tlq+33]
    add                  r6, smooth_weights+16*15-ipred_smooth_avx2_table
    lea                  r3, [rsp+64*2-2]
    punpcklbw           m12, m13, m0
    punpckhbw           m13, m0
    punpcklbw           m14, m15, m0
    punpckhbw           m15, m0
    call .prep_v
    dec                 tlq
    pmaddubsw            m0, m12, m5
    pmaddubsw            m1, m13, m5
    pmaddubsw            m2, m14, m5
    pmaddubsw            m5, m15, m5
    mova                xm8, [r6-16*7]
    vinserti128          m8, [r6-16*5], 1
    mova                xm9, [r6-16*6]
    vinserti128          m9, [r6-16*4], 1
    mova               xm10, [r6-16*3]
    vinserti128         m10, [r6-16*1], 1
    mova               xm11, [r6-16*2]
    vinserti128         m11, [r6-16*0], 1
    lea                  r6, [rsp+32*4]
    paddw                m0, m3
    paddw                m1, m3
    paddw                m2, m3
    paddw                m3, m5
    paddw                m0, m12
    paddw                m1, m13
    paddw                m2, m14
    paddw                m3, m15
    mova          [r6+32*0], m0
    mova          [r6+32*1], m1
    mova          [r6+32*2], m2
    mova          [r6+32*3], m3
.w64_loop:
    vpbroadcastb         m5, [tlq+hq]
    punpcklbw            m5, m4
    vpbroadcastw         m6, [r3+hq*2]
    vpbroadcastw         m7, [v_weightsq]
    add          v_weightsq, 2
    pmaddubsw            m2, m5, m8
    pmaddubsw            m3, m5, m9
    paddw                m2, m6
    paddw                m3, m6
    SMOOTH_2D_END         7, 7, 12, 13, [r6+32*0], [r6+32*1]
    mova        [dstq+32*0], m0
    pmaddubsw            m2, m5, m10
    pmaddubsw            m3, m5, m11
    paddw                m2, m6
    paddw                m3, m6
    SMOOTH_2D_END         7, 7, 14, 15, [r6+32*2], [r6+32*3]
    mova        [dstq+32*1], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET
ALIGN function_align
.prep_v:
    vpermq               m2, [tlq-32*1], q3120
    punpckhbw            m1, m2, m4
    punpcklbw            m2, m4
    pmaddubsw            m0, m1, m5 ; 127 * left - 127 * right
    paddw                m0, m1     ; 128 * left + 129 * right
    pmaddubsw            m1, m2, m5
    paddw                m1, m2
    vpermq               m2, [tlq-32*2], q3120
    mova [rsp+gprsize+32*3], m0
    mova [rsp+gprsize+32*2], m1
    punpckhbw            m1, m2, m4
    punpcklbw            m2, m4
    pmaddubsw            m0, m1, m5
    paddw                m0, m1
    pmaddubsw            m1, m2, m5
    paddw                m1, m2
    mova [rsp+gprsize+32*1], m0
    mova [rsp+gprsize+32*0], m1
    sub                  r3, hq
    sub                 tlq, hq
    sub                  r3, hq
    ret

cglobal ipred_z1_8bpc, 3, 8, 0, dst, stride, tl, w, h, angle, dx, maxbase
    %assign org_stack_offset stack_offset
    lea                  r6, [ipred_z1_avx2_table]
    tzcnt                wd, wm
    movifnidn        angled, anglem
    movifnidn            hd, hm
    lea                  r7, [dr_intra_derivative]
    inc                 tlq
    movsxd               wq, [r6+wq*4]
    add                  wq, r6
    mov                 dxd, angled
    and                 dxd, 0x7e
    add              angled, 165 ; ~90
    movzx               dxd, word [r7+dxq]
    xor              angled, 0x4ff ; d = 90 - angle
    vpbroadcastd         m3, [pw_512]
    vpbroadcastd         m4, [pw_62]
    vpbroadcastd         m5, [pw_64]
    jmp                  wq
.w4:
    cmp              angleb, 40
    jae .w4_no_upsample
    lea                 r3d, [angleq-1024]
    sar                 r3d, 7
    add                 r3d, hd
    jg .w4_no_upsample ; !enable_intra_edge_filter || h > 8 || (h == 8 && is_sm)
    ALLOC_STACK         -32, 8
    mova                xm1, [tlq-1]
    pshufb              xm0, xm1, [z_upsample1]
    pshufb              xm1, [z_upsample2]
    vpbroadcastd        xm2, [pb_36_m4] ; upshifted by 2 to be able to reuse
    add                 dxd, dxd        ; pw_512 (which is already in m3)
    pmaddubsw           xm0, xm2        ; for rounding instead of pw_2048
    pextrd         [rsp+16], xm1, 3 ; top[max_base_x]
    pmaddubsw           xm1, xm2
    movd                xm7, dxd
    mov                 r3d, dxd ; xpos
    vpbroadcastw         m7, xm7
    paddw               xm1, xm0
    movq                xm0, [tlq]
    pmulhrsw            xm1, xm3
    pslldq               m6, m7, 8
    paddw               xm2, xm7, xm7
    lea                  r2, [strideq*3]
    paddw                m6, m7
    packuswb            xm1, xm1
    paddw                m6, m2 ; xpos2 xpos3 xpos0 xpos1
    punpcklbw           xm0, xm1
    psllw                m7, 2
    mova              [rsp], xm0
.w4_upsample_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base0
    vpbroadcastq         m1, [rsp+r3]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base1
    vpbroadcastq         m2, [rsp+r5]
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base2
    movq                xm0, [rsp+r3]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base3
    movhps              xm0, [rsp+r5]
    vpblendd             m1, m2, 0xc0
    pand                 m2, m4, m6 ; frac
    vpblendd             m0, m1, 0xf0
    psubw                m1, m5, m2 ; 64-frac
    psllw                m2, 8
    por                  m1, m2     ; 64-frac, frac
    pmaddubsw            m0, m1
    paddw                m6, m7     ; xpos += dx
    pmulhrsw             m0, m3
    packuswb             m0, m0
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*2], xm0
    pextrd [dstq+r2       ], xm0, 1
    movd   [dstq+strideq*0], xm1
    pextrd [dstq+strideq*1], xm1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_upsample_loop
    RET
ALIGN function_align
.filter_strength: ; w4/w8/w16
    ; The C version uses a lot of branches, but we can do all the comparisons
    ; in parallel and use popcnt to get the final filter strength value.
%define base r3-z_filter_t0
    lea                  r3, [z_filter_t0]
    movd                xm0, maxbased
    movd                xm2, angled
    shr              angled, 8 ; is_sm << 1
    vpbroadcastb         m0, xm0
    vpbroadcastb         m2, xm2
    pcmpeqb              m1, m0, [base+z_filter_wh]
    pand                 m1, m2
    mova                xm2, [r3+angleq*8] ; upper ymm half zero in both cases
    pcmpgtb              m1, m2
    pmovmskb            r5d, m1
    ret
.w4_no_upsample:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -16, 11
    mov            maxbased, 7
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .w4_main
    lea            maxbased, [hq+3]
    call .filter_strength
    mov            maxbased, 7
    test                r5d, r5d
    jz .w4_main ; filter_strength == 0
    popcnt              r5d, r5d
    vpbroadcastd         m7, [base+pb_8]
    vbroadcasti128       m2, [tlq-1]
    pminub               m1, m7, [base+z_filter_s]
    vpbroadcastd         m8, [base+z_filter_k-4+r5*4+12*0]
    pminub               m7, [base+z_filter_s+8]
    vpbroadcastd         m9, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd        m10, [base+z_filter_k-4+r5*4+12*2]
    pshufb               m0, m2, m1
    shufps               m1, m7, q2121
    pmaddubsw            m0, m8
    pshufb               m1, m2, m1
    pmaddubsw            m1, m9
    pshufb               m2, m7
    pmaddubsw            m2, m10
    paddw                m0, m1
    paddw                m0, m2
    pmulhrsw             m0, m3
    mov                 r3d, 9
    mov                 tlq, rsp
    cmp                  hd, 4
    cmovne         maxbased, r3d
    vextracti128        xm1, m0, 1
    packuswb            xm0, xm1
    mova              [tlq], xm0
.w4_main:
    movd                xm6, dxd
    vpbroadcastq         m0, [z_base_inc] ; base_inc << 6
    vpbroadcastb         m7, [tlq+maxbaseq]
    shl            maxbased, 6
    vpbroadcastw         m6, xm6
    mov                 r3d, dxd ; xpos
    movd                xm9, maxbased
    vpbroadcastw         m9, xm9
    vbroadcasti128       m8, [z1_shuf_w4]
    psrlw                m7, 8  ; top[max_base_x]
    paddw               m10, m6, m6
    psubw                m9, m0 ; max_base_x
    vpblendd             m6, m10, 0xcc
    mova                xm0, xm10
    paddw                m6, m0 ; xpos2 xpos3 xpos0 xpos1
    paddw               m10, m10
.w4_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base0
    vpbroadcastq         m1, [tlq+r3]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base1
    vpbroadcastq         m2, [tlq+r5]
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base2
    movq                xm0, [tlq+r3]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base3
    movhps              xm0, [tlq+r5]
    vpblendd             m1, m2, 0xc0
    pand                 m2, m4, m6 ; frac
    vpblendd             m0, m1, 0xf0
    psubw                m1, m5, m2 ; 64-frac
    psllw                m2, 8
    pshufb               m0, m8
    por                  m1, m2     ; 64-frac, frac
    pmaddubsw            m0, m1
    pcmpgtw              m1, m9, m6 ; base < max_base_x
    pmulhrsw             m0, m3
    paddw                m6, m10    ; xpos += dx
    lea                  r5, [dstq+strideq*2]
    vpblendvb            m0, m7, m0, m1
    packuswb             m0, m0
    vextracti128        xm1, m0, 1
    movd   [r5  +strideq*0], xm0
    pextrd [r5  +strideq*1], xm0, 1
    movd   [dstq+strideq*0], xm1
    pextrd [dstq+strideq*1], xm1, 1
    sub                  hd, 4
    jz .w4_end
    lea                dstq, [dstq+strideq*4]
    cmp                 r3d, maxbased
    jb .w4_loop
    packuswb            xm7, xm7
    lea                  r6, [strideq*3]
.w4_end_loop:
    movd   [dstq+strideq*0], xm7
    movd   [dstq+strideq*1], xm7
    movd   [dstq+strideq*2], xm7
    movd   [dstq+r6       ], xm7
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_end_loop
.w4_end:
    RET
ALIGN function_align
.w8:
    lea                 r3d, [angleq+216]
    mov                 r3b, hb
    cmp                 r3d, 8
    ja .w8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || h > 8
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -32, 8
    movu                xm2, [z_filter_s+6]
    mova                xm0, [tlq-1]
    movd                xm6, hd
    vinserti128          m0, [tlq+7], 1
    vpbroadcastb        xm6, xm6
    vbroadcasti128       m1, [z_upsample1]
    pminub              xm6, xm2
    vpbroadcastd         m7, [pb_36_m4]
    vinserti128          m2, xm6, 1
    add                 dxd, dxd
    pshufb               m1, m0, m1
    pshufb               m2, m0, m2
    movd                xm6, dxd
    pmaddubsw            m1, m7
    pmaddubsw            m2, m7
    vpbroadcastw         m6, xm6
    mov                 r3d, dxd
    psrldq               m0, 1
    lea                  r2, [strideq*3]
    paddw                m7, m6, m6
    paddw                m1, m2
    vpblendd             m6, m7, 0xf0
    pmulhrsw             m1, m3
    pslldq               m2, m7, 8
    paddw                m7, m7
    paddw                m6, m2
    packuswb             m1, m1
    punpcklbw            m0, m1
    mova              [rsp], m0
.w8_upsample_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base0
    movu                xm0, [rsp+r3]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base1
    vinserti128          m0, [rsp+r5], 1
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base2
    pand                 m1, m4, m6
    psubw                m2, m5, m1
    psllw                m1, 8
    por                  m2, m1
    punpcklqdq           m1, m2, m2 ; frac0 frac1
    pmaddubsw            m0, m1
    movu                xm1, [rsp+r3]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base3
    vinserti128          m1, [rsp+r5], 1
    punpckhqdq           m2, m2 ; frac2 frac3
    pmaddubsw            m1, m2
    pmulhrsw             m0, m3
    paddw                m6, m7
    pmulhrsw             m1, m3
    packuswb             m0, m1
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*2], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+r2       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_upsample_loop
    RET
.w8_no_intra_edge_filter:
    and            maxbased, 7
    or             maxbased, 8 ; imin(h+7, 15)
    jmp .w8_main
.w8_no_upsample:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -32, 10
    lea            maxbased, [hq+7]
    test             angled, 0x400
    jnz .w8_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .w8_main ; filter_strength == 0
    popcnt              r5d, r5d
    movu                xm2, [tlq]
    pminub              xm1, xm0, [base+z_filter_s+14]
    vinserti128          m2, [tlq-1], 1
    vinserti128          m1, [base+z_filter_s+ 0], 1
    vpbroadcastd         m7, [base+z_filter_k-4+r5*4+12*0]
    pminub              xm0, [base+z_filter_s+22]
    vinserti128          m0, [base+z_filter_s+ 8], 1
    pshufb               m6, m2, m1
    pmaddubsw            m6, m7
    vpbroadcastd         m7, [base+z_filter_k-4+r5*4+12*1]
    movzx               r3d, byte [tlq+15]
    shufps               m1, m0, q2121
    pshufb               m1, m2, m1
    pmaddubsw            m1, m7
    paddw                m1, m6
    sub                 r5d, 3
    jnz .w8_3tap
    ; filter_strength == 3 uses a 5-tap filter instead of a 3-tap one,
    ; which also results in an awkward edge case where out[w*2] is
    ; slightly different from out[max_base_x] when h > w.
    vpbroadcastd         m7, [z_filter_k+4*8]
    movzx               r2d, byte [tlq+14]
    pshufb               m2, m0
    pmaddubsw            m2, m7
    sub                 r2d, r3d
    lea                 r2d, [r2+r3*8+4]
    shr                 r2d, 3 ; (tlq[w*2-2] + tlq[w*2-1]*7 + 4) >> 3
    mov            [rsp+16], r2b
    paddw                m1, m2
.w8_3tap:
    pmulhrsw             m1, m3
    sar                 r5d, 1
    mov                 tlq, rsp
    add                 r5d, 17 ; w*2 + (filter_strength == 3)
    cmp                  hd, 16
    cmovns         maxbased, r5d
    mov            [tlq+r5], r3b
    vextracti128        xm0, m1, 1
    packuswb            xm0, xm1
    mova              [tlq], xm0
.w8_main:
    movd                xm2, dxd
    vbroadcasti128       m0, [z_base_inc]
    vpbroadcastw         m2, xm2
    vpbroadcastb         m7, [tlq+maxbaseq]
    shl            maxbased, 6
    movd                xm9, maxbased
    vbroadcasti128       m8, [z_filter_s+2]
    vpbroadcastw         m9, xm9
    psrlw                m7, 8
    psubw                m9, m0
    mov                 r3d, dxd
    paddw                m6, m2, m2
    vpblendd             m2, m6, 0xf0
.w8_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6
    pand                 m0, m4, m2
    psubw                m1, m5, m0
    psllw                m0, 8
    por                  m1, m0
    movu                xm0, [tlq+r3]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base1
    vinserti128          m0, [tlq+r5], 1
    pshufb               m0, m8
    pmaddubsw            m0, m1
    pcmpgtw              m1, m9, m2
    paddw                m2, m6
    pmulhrsw             m0, m3
    vpblendvb            m0, m7, m0, m1
    vextracti128        xm1, m0, 1
    packuswb            xm0, xm1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    sub                  hd, 2
    jz .w8_end
    lea                dstq, [dstq+strideq*2]
    cmp                 r3d, maxbased
    jb .w8_loop
    packuswb            xm7, xm7
.w8_end_loop:
    movq   [dstq+strideq*0], xm7
    movq   [dstq+strideq*1], xm7
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_end_loop
.w8_end:
    RET
.w16_no_intra_edge_filter:
    and            maxbased, 15
    or             maxbased, 16 ; imin(h+15, 31)
    jmp .w16_main
ALIGN function_align
.w16:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -64, 12
    lea            maxbased, [hq+15]
    test             angled, 0x400
    jnz .w16_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .w16_main ; filter_strength == 0
    popcnt              r5d, r5d
    vpbroadcastd         m1, [base+pb_12]
    vbroadcasti128       m6, [base+z_filter_s+8]
    vinserti128          m2, m6, [base+z_filter_s], 0
    vinserti128          m6, [base+z_filter_s+16], 1
    mova               xm10, [tlq-1]
    vinserti128         m10, [tlq+3], 1
    vpbroadcastd         m9, [base+z_filter_k-4+r5*4+12*0]
    vbroadcasti128       m7, [base+z_filter_s+14]
    vinserti128          m8, m7, [base+z_filter_s+6], 0
    vinserti128          m7, [base+z_filter_s+22], 1
    psubw                m0, m1
    movu               xm11, [tlq+12]
    vinserti128         m11, [tlq+16], 1
    pminub               m8, m0
    pminub               m7, m0
    pshufb               m0, m10, m2
    shufps               m2, m6, q2121
    pmaddubsw            m0, m9
    pshufb               m1, m11, m8
    shufps               m8, m7, q2121
    pmaddubsw            m1, m9
    vpbroadcastd         m9, [base+z_filter_k-4+r5*4+12*1]
    movzx               r3d, byte [tlq+31]
    pshufb               m2, m10, m2
    pmaddubsw            m2, m9
    pshufb               m8, m11, m8
    pmaddubsw            m8, m9
    paddw                m0, m2
    paddw                m1, m8
    sub                 r5d, 3
    jnz .w16_3tap
    vpbroadcastd         m9, [z_filter_k+4*8]
    movzx               r2d, byte [tlq+30]
    pshufb              m10, m6
    pmaddubsw           m10, m9
    pshufb              m11, m7
    pmaddubsw           m11, m9
    sub                 r2d, r3d
    lea                 r2d, [r2+r3*8+4]
    shr                 r2d, 3
    mov            [rsp+32], r2b
    paddw                m0, m10
    paddw                m1, m11
.w16_3tap:
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    sar                 r5d, 1
    mov                 tlq, rsp
    add                 r5d, 33
    cmp                  hd, 32
    cmovns         maxbased, r5d
    mov            [tlq+r5], r3b
    packuswb             m0, m1
    vpermq               m0, m0, q3120
    mova              [tlq], m0
.w16_main:
    movd                xm6, dxd
    vbroadcasti128       m0, [z_base_inc]
    vpbroadcastb         m7, [tlq+maxbaseq]
    shl            maxbased, 6
    vpbroadcastw         m6, xm6
    movd                xm9, maxbased
    vbroadcasti128       m8, [z_filter_s+2]
    vpbroadcastw         m9, xm9
    mov                 r3d, dxd
    psubw                m9, m0
    paddw               m11, m6, m6
    psubw               m10, m9, m3 ; 64*8
    vpblendd             m6, m11, 0xf0
.w16_loop:
    lea                 r5d, [r3+dxq]
    shr                 r3d, 6 ; base0
    pand                 m1, m4, m6
    psubw                m2, m5, m1
    psllw                m1, 8
    por                  m2, m1
    movu                xm0, [tlq+r3+0]
    movu                xm1, [tlq+r3+8]
    lea                 r3d, [r5+dxq]
    shr                 r5d, 6 ; base1
    vinserti128          m0, [tlq+r5+0], 1
    vinserti128          m1, [tlq+r5+8], 1
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    pcmpgtw              m1, m9, m6
    pcmpgtw              m2, m10, m6
    packsswb             m1, m2
    paddw                m6, m11
    vpblendvb            m0, m7, m0, m1
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    sub                  hd, 2
    jz .w16_end
    lea                dstq, [dstq+strideq*2]
    cmp                 r3d, maxbased
    jb .w16_loop
.w16_end_loop:
    mova   [dstq+strideq*0], xm7
    mova   [dstq+strideq*1], xm7
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_end_loop
.w16_end:
    RET
ALIGN function_align
.w32:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -96, 15
    lea                 r3d, [hq+31]
    mov            maxbased, 63
    cmp                  hd, 32
    cmovs          maxbased, r3d
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .w32_main
    vbroadcasti128       m0, [pb_0to15]
    sub                 r3d, 29 ; h+2
    movu               xm13, [tlq+29]    ; 32-39
    movd                xm1, r3d
    movu               xm14, [tlq+37]    ; 40-47
    sub                 r3d, 8 ; h-6
    vinserti128         m14, [tlq+51], 1 ; 56-63
    vpbroadcastb        xm1, xm1
    mova               xm11, [tlq- 1]    ;  0- 7
    vinserti128         m11, [tlq+13], 1 ; 16-23
    movd                xm2, r3d
    movu               xm12, [tlq+ 5]    ;  8-15
    vinserti128         m12, [tlq+19], 1 ; 24-31
    pminub              xm1, xm0 ; clip 32x8
    mova                 m7, [z_filter_s+0]
    pshufb             xm13, xm1
    vpbroadcastd         m1, [pb_12]
    vpbroadcastb        xm2, xm2
    vinserti128         m13, [tlq+43], 1 ; 48-55
    vinserti128          m8, m7, [z_filter_s+4], 1
    vpblendd             m2, m1, 0xf0
    vinserti128          m7, [z_filter_s+12], 0
    pminub               m2, m0 ; clip 32x16 and 32x(32|64)
    vpbroadcastd         m9, [z_filter_k+4*2+12*0]
    pshufb              m14, m2
    pshufb               m0, m11, m8
    shufps               m8, m7, q1021
    pmaddubsw            m0, m9
    pshufb               m2, m12, m8
    pmaddubsw            m2, m9
    pshufb               m1, m13, m8
    pmaddubsw            m1, m9
    pshufb               m6, m14, m8
    pmaddubsw            m6, m9
    vpbroadcastd         m9, [z_filter_k+4*2+12*1]
    pshufb              m10, m11, m8
    shufps               m8, m7, q2121
    pmaddubsw           m10, m9
    paddw                m0, m10
    pshufb              m10, m12, m8
    pmaddubsw           m10, m9
    paddw                m2, m10
    pshufb              m10, m13, m8
    pmaddubsw           m10, m9
    paddw                m1, m10
    pshufb              m10, m14, m8
    pmaddubsw           m10, m9
    paddw                m6, m10
    vpbroadcastd         m9, [z_filter_k+4*2+12*2]
    pshufb              m11, m8
    pmaddubsw           m11, m9
    pshufb              m12, m7
    pmaddubsw           m12, m9
    movzx               r3d, byte [tlq+63]
    movzx               r2d, byte [tlq+62]
    paddw                m0, m11
    paddw                m2, m12
    pshufb              m13, m7
    pmaddubsw           m13, m9
    pshufb              m14, m7
    pmaddubsw           m14, m9
    paddw                m1, m13
    paddw                m6, m14
    sub                 r2d, r3d
    lea                 r2d, [r2+r3*8+4] ; edge case for 32x64
    pmulhrsw             m0, m3
    pmulhrsw             m2, m3
    pmulhrsw             m1, m3
    pmulhrsw             m6, m3
    shr                 r2d, 3
    mov            [rsp+64], r2b
    mov                 tlq, rsp
    mov            [tlq+65], r3b
    mov                 r3d, 65
    cmp                  hd, 64
    cmove          maxbased, r3d
    packuswb             m0, m2
    packuswb             m1, m6
    mova           [tlq+ 0], m0
    mova           [tlq+32], m1
.w32_main:
    movd                xm6, dxd
    vpbroadcastb         m7, [tlq+maxbaseq]
    shl            maxbased, 6
    vpbroadcastw         m6, xm6
    movd                xm9, maxbased
    vbroadcasti128       m8, [z_filter_s+2]
    vpbroadcastw         m9, xm9
    mov                 r5d, dxd
    psubw                m9, [z_base_inc]
    mova                m11, m6
    psubw               m10, m9, m3 ; 64*8
.w32_loop:
    mov                 r3d, r5d
    shr                 r3d, 6
    pand                 m1, m4, m6
    psubw                m2, m5, m1
    psllw                m1, 8
    por                  m2, m1
    movu                 m0, [tlq+r3+0]
    movu                 m1, [tlq+r3+8]
    add                 r5d, dxd
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    pcmpgtw              m1, m9, m6
    pcmpgtw              m2, m10, m6
    packsswb             m1, m2
    paddw                m6, m11
    vpblendvb            m0, m7, m0, m1
    mova             [dstq], m0
    dec                  hd
    jz .w32_end
    add                dstq, strideq
    cmp                 r5d, maxbased
    jb .w32_loop
    test                 hb, 1
    jz .w32_end_loop
    mova             [dstq], m7
    add                dstq, strideq
    dec                  hd
    jz .w32_end
.w32_end_loop:
    mova   [dstq+strideq*0], m7
    mova   [dstq+strideq*1], m7
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_end_loop
.w32_end:
    RET
ALIGN function_align
.w64:
    %assign stack_offset org_stack_offset
    ALLOC_STACK        -128, 16
    lea            maxbased, [hq+63]
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .w64_main
    mova               xm11, [tlq- 1]    ;  0- 7
    vinserti128         m11, [tlq+13], 1 ; 16-23
    movu               xm12, [tlq+ 5]    ;  8-15
    vinserti128         m12, [tlq+19], 1 ; 24-31
    mova                 m7, [z_filter_s+0]
    vinserti128          m8, m7, [z_filter_s+4], 1
    vinserti128          m7, [z_filter_s+12], 0
    vpbroadcastd         m9, [z_filter_k+4*2+12*0]
    movu               xm13, [tlq+29]    ; 32-39
    vinserti128         m13, [tlq+43], 1 ; 48-55
    movu               xm14, [tlq+37]    ; 40-47
    vinserti128         m14, [tlq+51], 1 ; 56-63
    pshufb               m0, m11, m8
    shufps               m8, m7, q1021
    pmaddubsw            m0, m9
    pshufb               m2, m12, m8
    pmaddubsw            m2, m9
    pshufb               m1, m13, m8
    pmaddubsw            m1, m9
    pshufb               m6, m14, m8
    pmaddubsw            m6, m9
    vpbroadcastd         m9, [z_filter_k+4*2+12*1]
    pshufb              m10, m11, m8
    shufps              m15, m8, m7, q2121
    pmaddubsw           m10, m9
    paddw                m0, m10
    pshufb              m10, m12, m15
    pmaddubsw           m10, m9
    paddw                m2, m10
    pshufb              m10, m13, m15
    pmaddubsw           m10, m9
    paddw                m1, m10
    pshufb              m10, m14, m15
    pmaddubsw           m10, m9
    paddw                m6, m10
    vpbroadcastd        m10, [z_filter_k+4*2+12*2]
    pshufb              m11, m15
    pmaddubsw           m11, m10
    pshufb              m12, m7
    pmaddubsw           m12, m10
    pshufb              m13, m7
    pmaddubsw           m13, m10
    pshufb              m14, m7
    pmaddubsw           m14, m10
    paddw                m0, m11
    paddw                m2, m12
    paddw                m1, m13
    paddw                m6, m14
    movu               xm11, [tlq+ 61]    ;  64- 71
    vinserti128         m11, [tlq+ 75], 1 ;  80- 87
    movu               xm12, [tlq+ 69]    ;  72- 79
    vinserti128         m12, [tlq+ 83], 1 ;  88- 95
    movu               xm13, [tlq+ 93]    ;  96-103
    vinserti128         m13, [tlq+107], 1 ; 112-119
    movu               xm14, [tlq+101]    ; 104-111
    vinserti128         m14, [tlq+115], 1 ; 120-127
    pmulhrsw             m0, m3
    pmulhrsw             m2, m3
    pmulhrsw             m1, m3
    pmulhrsw             m6, m3
    lea                 r3d, [hq-20]
    mov                 tlq, rsp
    packuswb             m0, m2
    packuswb             m1, m6
    vpbroadcastd        xm2, [pb_14]
    vbroadcasti128       m6, [pb_0to15]
    mova         [tlq+32*0], m0
    mova         [tlq+32*1], m1
    movd                xm0, r3d
    vpbroadcastd         m1, [pb_12]
    vpbroadcastb         m0, xm0
    paddb                m0, m2
    pminub               m0, m6 ; clip 64x16 and 64x32
    pshufb              m12, m0
    pminub               m1, m6 ; clip 64x64
    pshufb              m14, m1
    pshufb               m0, m11, m7
    pmaddubsw            m0, m10
    pshufb               m2, m12, m7
    pmaddubsw            m2, m10
    pshufb               m1, m13, m7
    pmaddubsw            m1, m10
    pshufb               m6, m14, m7
    pmaddubsw            m6, m10
    pshufb               m7, m11, m15
    pmaddubsw            m7, m9
    pshufb              m10, m12, m15
    pmaddubsw           m10, m9
    paddw                m0, m7
    pshufb               m7, m13, m15
    pmaddubsw            m7, m9
    paddw                m2, m10
    pshufb              m10, m14, m15
    pmaddubsw           m10, m9
    paddw                m1, m7
    paddw                m6, m10
    vpbroadcastd         m9, [z_filter_k+4*2+12*0]
    pshufb              m11, m8
    pmaddubsw           m11, m9
    pshufb              m12, m8
    pmaddubsw           m12, m9
    pshufb              m13, m8
    pmaddubsw           m13, m9
    pshufb              m14, m8
    pmaddubsw           m14, m9
    paddw                m0, m11
    paddw                m2, m12
    paddw                m1, m13
    paddw                m6, m14
    pmulhrsw             m0, m3
    pmulhrsw             m2, m3
    pmulhrsw             m1, m3
    pmulhrsw             m6, m3
    packuswb             m0, m2
    packuswb             m1, m6
    mova         [tlq+32*2], m0
    mova         [tlq+32*3], m1
.w64_main:
    movd               xm12, dxd
    vpbroadcastb         m7, [tlq+maxbaseq]
    lea                 r3d, [dxq-64]
    shl            maxbased, 6
    vpbroadcastw        m12, xm12
    sub                 r3d, maxbased
    vbroadcasti128       m8, [z_filter_s+2]
    movd                xm6, r3d
    mov                 r5d, dxd
    mova                m10, [pb_1to32]
    vpbroadcastd        m11, [pb_32]
    vpbroadcastw         m6, xm6
.w64_loop:
    mov                 r3d, r5d
    shr                 r3d, 6
    movu                 m0, [tlq+r3+ 0]
    movu                 m1, [tlq+r3+ 8]
    pand                 m2, m4, m6
    psubw                m9, m5, m2
    psllw                m2, 8
    por                  m9, m2
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m9
    pmaddubsw            m1, m9
    psraw                m2, m6, 6
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packsswb             m2, m2
    paddb                m2, m10
    packuswb             m0, m1
    vpblendvb            m0, m7, m0, m2
    mova          [dstq+ 0], m0
    movu                 m0, [tlq+r3+32]
    movu                 m1, [tlq+r3+40]
    add                 r5d, dxd
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m9
    pmaddubsw            m1, m9
    paddb                m2, m11
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    paddw                m6, m12
    packuswb             m0, m1
    vpblendvb            m0, m7, m0, m2
    mova          [dstq+32], m0
    dec                  hd
    jz .w64_end
    add                dstq, strideq
    cmp                 r5d, maxbased
    jb .w64_loop
.w64_end_loop:
    mova          [dstq+ 0], m7
    mova          [dstq+32], m7
    add                dstq, strideq
    dec                  hd
    jg .w64_end_loop
.w64_end:
    RET

cglobal ipred_z2_8bpc, 3, 10, 16, 224, dst, stride, tl, w, h, angle, dx, dy
%define base r9-z_filter_t0
    lea                  r9, [ipred_z2_avx2_table]
    tzcnt                wd, wm
    movifnidn        angled, anglem
    movifnidn            hd, hm
    lea                 dxq, [dr_intra_derivative-90]
    movsxd               wq, [r9+wq*4]
    movzx               dyd, angleb
    xor              angled, 0x400
    mov                  r8, dxq
    sub                 dxq, dyq
    add                  wq, r9
    add                  r9, z_filter_t0-ipred_z2_avx2_table
    mova                 m2, [tlq-64]
    mova                 m0, [tlq-32]
    mova                 m1, [tlq]
    and                 dyd, ~1
    and                 dxq, ~1
    movzx               dyd, word [r8+dyq]  ; angle - 90
    movzx               dxd, word [dxq+270] ; 180 - angle
    vpbroadcastd        m13, [base+pw_512]
    vpbroadcastd        m14, [base+pw_62]
    vpbroadcastd        m15, [base+pw_64]
    mova           [rsp+ 0], m2
    mova           [rsp+32], m0
    mova           [rsp+64], m1
    neg                 dxd
    neg                 dyd
    jmp                  wq
.w4:
    vpbroadcastq         m6, [base+z2_base_inc] ; base_inc << 6
    vbroadcasti128      m10, [base+z1_shuf_w4]
    vbroadcasti128      m11, [base+z2_shuf_h4]
    lea                 r2d, [dxq+(65<<6)] ; xpos
    movd                xm5, dyd
    mov                 r8d, (63-4)<<6
    mov                 dyq, -4
    pshuflw             xm5, xm5, q0000
    pmullw              xm5, [base+z2_ymul]
    test             angled, 0x400
    jnz .w4_main ; !enable_intra_edge_filter
    lea                 r3d, [hq+2]
    add              angled, 1022
    shl                 r3d, 6
    test                r3d, angled
    jnz .w4_no_upsample_above ; angle >= 130 || h > 8 || (is_sm && h == 8)
    vpbroadcastd        xm3, [base+pb_4]
    call .upsample_above
    sub              angled, 1075 ; angle - 53
    lea                 r3d, [hq+3]
    xor              angled, 0x7f ; 180 - angle
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
.upsample_above: ; w4/w8
    pshufb              xm2, xm1, [base+z_upsample1-2]
    pminub              xm3, [base+z_filter_s+4]
    vpbroadcastd        xm4, [base+pb_36_m4]
    vbroadcasti128      m10, [base+pb_0to15]
    pshufb              xm3, xm1, xm3
    pmaddubsw           xm2, xm4
    pmaddubsw           xm3, xm4
    lea                 r2d, [r2+dxq+(1<<6)]
    add                 dxd, dxd
    paddw               xm2, xm3
    pmulhrsw            xm2, xm13
    sub                 r8d, 3<<6
    paddw                m6, m6
    packuswb            xm2, xm2
    punpcklbw           xm1, xm2
    mova   [rsp+gprsize+64], xm1
    ret
ALIGN function_align
.upsample_left: ; h4/h8
    mov                 r3d, hd
    and                 r3d, 4
    movd                xm2, [rsp+gprsize+64]
    movddup             xm0, [rsp+gprsize+56]
    movd                xm1, r3d
    palignr             xm2, xm0, 1
    vpbroadcastb        xm1, xm1
    pshufb              xm2, [base+z_filter_s+18]
    vpbroadcastd        xm3, [base+pb_36_m4]
    pmaxub              xm1, [base+z_upsample1-2]
    pshufb              xm1, xm0, xm1
    pmaddubsw           xm2, xm3
    pmaddubsw           xm1, xm3
    paddw               xm5, xm5
    add                 dyq, dyq
    paddw               xm1, xm2
    pmulhrsw            xm1, xm13
    vbroadcasti128      m11, [base+z2_upsample]
    paddw               xm5, xm15
    packuswb            xm1, xm1
    punpcklbw           xm0, xm1
    mova   [rsp+gprsize+48], xm0
    ret
.w4_no_upsample_above:
    lea                 r3d, [hq+3]
    sub              angled, 1112 ; angle - 90
    call .filter_strength
    test                r3d, r3d
    jz .w4_no_filter_above
    popcnt              r3d, r3d
    vpbroadcastd        xm2, [base+pb_4]
    pminub              xm2, [base+z_filter_s]
    vpbroadcastd        xm0, [base+z_filter_k-4+r3*4+12*0]
    vpbroadcastd        xm4, [base+z_filter_k-4+r3*4+12*1]
    pshufb              xm3, xm1, xm2 ; 00 01 12 23
    pshufd              xm2, xm2, q0321
    pmaddubsw           xm0, xm3, xm0
    pshufb              xm2, xm1, xm2 ; 12 23 34 44
    pmaddubsw           xm2, xm4
    vpbroadcastd        xm4, [base+z_filter_k-4+r3*4+12*2]
    punpckhqdq          xm3, xm3      ; 34 44 44 44
    pmaddubsw           xm3, xm4
    movd                xm4, r6m      ; max_width
    pminsw              xm4, xm15
    vpbroadcastb        xm4, xm4
    paddw               xm0, xm2
    paddw               xm0, xm3
    pmulhrsw            xm0, xm13
    psubb               xm4, [base+pb_1to32]
    psrlq               xm1, 8
    packuswb            xm0, xm0
    vpblendvb           xm0, xm1, xm4
    movd           [rsp+65], xm0
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
    mov                 r5d, 10
    cmp                  hd, 16
    movu                xm2, [rsp+49]
    vinserti128          m2, [rsp+43], 1
    cmovs               r5d, hd
    xor                 r5d, 15 ; h == 16 ? 5 : 15 - h
    movd                xm0, r5d
    vbroadcasti128       m1, [base+z_filter_s+12]
    vbroadcasti128       m4, [base+z_filter_s+16]
    vinserti128          m3, m1, [z_filter_s+8], 1   ; 56 67 78 89 9a ab bc cd   55 55 56 67 78 89 9a ab
    vpblendd             m1, m4, 0x0f                ; 78 89 9a ab bc cd de ef   56 67 78 89 9a ab bc cd
    vinserti128          m4, [base+z_filter_s+20], 0 ; 9a ab bc cd de ef ff ff   78 89 9a ab bc cd de ef
    vpbroadcastb         m0, xm0
    pmaxub               m0, m3
    vpbroadcastd         m3, [base+z_filter_k-4+r3*4+12*0]
    pshufb               m0, m2, m0
    pmaddubsw            m0, m3
    vpbroadcastd         m3, [base+z_filter_k-4+r3*4+12*1]
    pshufb               m1, m2, m1
    pmaddubsw            m1, m3
    vpbroadcastd         m3, [base+z_filter_k-4+r3*4+12*2]
    pshufb               m2, m4
    pmaddubsw            m2, m3
    movd                xm4, r7m ; max_height
    pminsw              xm4, xm15
    vpbroadcastb        xm4, xm4
    psubb               xm4, [base+pb_16to1]
    paddw                m1, m0
    paddw                m1, m2
    pmulhrsw             m1, m13
    vextracti128        xm0, m1, 1
    packuswb            xm0, xm1
    vpblendvb           xm0, [rsp+48], xm4
    mova           [rsp+48], xm0
    jmp .w4_main
.w4_upsample_left:
    call .upsample_left
.w4_main:
    movd                xm0, dxd
    mova                m12, [base+z2_y_shuf_h4]
    lea                  r5, [rsp+56]  ; left-7
    vpbroadcastw         m0, xm0
    lea                  r9, [strideq*3]
    psraw               xm1, xm5, 6
    pand                xm5, xm14      ; frac_y
    pxor                xm2, xm2
    paddw                m7, m0, m0
    psubw               xm4, xm2, xm1  ; base_y
    vpblendd             m0, m7, 0xcc
    mova                xm1, xm7
    punpcklwd           xm4, xm2
    paddw                m0, m1        ; xpos2 xpos3 xpos0 xpos1
    psubw               xm1, xm15, xm5 ; 64-frac_y
    psllw               xm5, 8
    paddw                m7, m7
    paddw                m6, m0
    por                 xm5, xm1       ; 64-frac_y, frac_y
    vpbroadcastq         m5, xm5
.w4_loop:
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6         ; base_x0
    vpbroadcastq         m1, [rsp+r2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6         ; base_x1
    vpbroadcastq         m2, [rsp+r3]
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6         ; base_x2
    movq                xm0, [rsp+r2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6         ; base_x3
    movhps              xm0, [rsp+r3]
    vpblendd             m1, m2, 0xc0
    pand                 m2, m14, m6   ; frac_x
    vpblendd             m0, m1, 0xf0
    psubw                m1, m15, m2   ; 64-frac_x
    psllw                m2, 8
    pshufb               m0, m10
    por                  m1, m2        ; 64-frac_x, frac_x
    pmaddubsw            m0, m1
    cmp                 r3d, 64
    jge .w4_toponly
    mova                 m1, m7        ; arbitrary negative value
    vpgatherdq           m3, [r5+xm4], m1
    pshufb               m1, m3, m11
    vpermd               m1, m12, m1
    pmaddubsw            m1, m5
    psraw                m2, m6, 15    ; base_x < topleft
    vpblendvb            m0, m1, m2
.w4_toponly:
    pmulhrsw             m0, m13
    paddw                m6, m7        ; xpos += dx
    add                  r5, dyq
    packuswb             m0, m0
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*2], xm0
    pextrd [dstq+r9       ], xm0, 1
    movd   [dstq+strideq*0], xm1
    pextrd [dstq+strideq*1], xm1, 1
    sub                  hd, 4
    jz .w4_end
    lea                dstq, [dstq+strideq*4]
    cmp                 r2d, r8d
    jge .w4_loop
.w4_leftonly_loop:
    mova                 m1, m7
    vpgatherdq           m2, [r5+xm4], m1
    add                  r5, dyq
    pshufb               m0, m2, m11
    vpermd               m0, m12, m0
    pmaddubsw            m0, m5
    pmulhrsw             m0, m13
    packuswb             m0, m0
    vextracti128        xm1, m0, 1
    movd   [dstq+strideq*2], xm0
    pextrd [dstq+r9       ], xm0, 1
    movd   [dstq+strideq*0], xm1
    pextrd [dstq+strideq*1], xm1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_leftonly_loop
.w4_end:
    RET
.w8:
    vbroadcasti128       m6, [base+z2_base_inc] ; base_inc << 6
    movd                xm5, dyd
    vbroadcasti128      m10, [base+z_filter_s+2]
    vbroadcasti128      m11, [base+z2_shuf_h4]
    lea                 r2d, [dxq+(65<<6)] ; xpos
    vpbroadcastw        xm5, xm5
    mov                 r8d, (63-8)<<6
    mov                 dyq, -4
    pmullw              xm5, [base+z2_ymul]
    test             angled, 0x400
    jnz .w8_main
    lea                 r3d, [angleq+126]
    mov                 r3b, hb
    cmp                 r3d, 8
    ja .w8_no_upsample_above ; angle >= 130 || h > 8 || is_sm
    vpbroadcastd        xm3, [base+pb_8]
    movhps         [rsp+80], xm1
    call .upsample_above
    sub              angled, 53 ; angle - 53
    lea                 r3d, [hq+7]
    xor              angled, 0x7f ; 180 - angle
    call .filter_strength
    jmp .w8_filter_left
.w8_no_upsample_above:
    lea                 r3d, [hq+7]
    sub              angled, 90 ; angle - 90
    call .filter_strength
    test                r3d, r3d
    jz .w8_no_filter_above
    popcnt              r3d, r3d
    vpbroadcastd        xm3, [base+pb_8]
    pminub              xm3, [base+z_filter_s+8]
    vpbroadcastd        xm0, [base+z_filter_k-4+r3*4+12*0]
    vpbroadcastd        xm4, [base+z_filter_k-4+r3*4+12*1]
    pshufb              xm2, xm1, [base+z_filter_s] ; 00 01 12 23 34 45 56 67
    pmaddubsw           xm0, xm2, xm0
    pshufb              xm3, xm1, xm3               ; 34 45 56 67 78 88 88 88
    shufps              xm2, xm3, q2121             ; 12 23 34 45 56 67 78 88
    pmaddubsw           xm2, xm4
    vpbroadcastd        xm4, [base+z_filter_k-4+r3*4+12*2]
    pmaddubsw           xm3, xm4
    movd                xm4, r6m ; max_width
    pminuw              xm4, xm15
    vpbroadcastb        xm4, xm4
    paddw               xm0, xm2
    paddw               xm0, xm3
    pmulhrsw            xm0, xm13
    psubb               xm4, [base+pb_1to32]
    psrldq              xm1, 1
    packuswb            xm0, xm0
    vpblendvb           xm0, xm1, xm4
    movq           [rsp+65], xm0
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
    vpbroadcastd         m7, [base+z_filter_k-4+r3*4+12*0]
    vpbroadcastd         m8, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd         m9, [base+z_filter_k-4+r3*4+12*2]
    cmp                  hd, 32
    jne .w8_filter_left_h16
    movu                xm2, [rsp+27]
    vinserti128          m2, [rsp+35], 1
    vpbroadcastd        xm0, [base+pb_5]
    vbroadcasti128       m3, [base+z_filter_s+ 8]
    vbroadcasti128       m1, [base+z_filter_s+12]
    vbroadcasti128       m4, [base+z_filter_s+16]
    pmaxub               m3, m0
    pshufb               m3, m2, m3
    pmaddubsw            m3, m7
    pshufb               m1, m2, m1
    pmaddubsw            m1, m8
    pshufb               m2, m4
    pmaddubsw            m2, m9
    paddw                m3, m1
    paddw                m3, m2
    pmulhrsw             m3, m13
    jmp .w8_filter_left_top16
.w8_filter_left_h16:
    mov                 r5d, 10
    cmp                  hd, 16
    cmovs               r5d, hd
    xor                 r5d, 15 ; h == 16 ? 5 : 15 - h
    movd                xm0, r5d
    vpbroadcastb         m0, xm0
.w8_filter_left_top16:
    vbroadcasti128       m1, [base+z_filter_s+12]
    vinserti128          m2, m1, [base+z_filter_s+8], 1 ; 56 67 78 89 9a ab bc cd   55 55 56 67 78 89 9a ab
    vbroadcasti128       m4, [base+z_filter_s+16]
    vpblendd             m1, m4, 0x0f                   ; 78 89 9a ab bc cd de ef   56 67 78 89 9a ab bc cd
    vinserti128          m4, [base+z_filter_s+20], 0    ; 9a ab bc cd de ef ff ff   78 89 9a ab bc cd de ef
    pmaxub               m0, m2
    movu                xm2, [rsp+49]
    vinserti128          m2, [rsp+43], 1
    pshufb               m0, m2, m0
    pmaddubsw            m0, m7
    movd                xm7, r7m ; max_height
    pshufb               m1, m2, m1
    pmaddubsw            m1, m8
    pshufb               m2, m4
    pmaddubsw            m2, m9
    pminsw              xm7, xm15
    paddw                m1, m0
    vpbroadcastb         m7, xm7
    paddw                m1, m2
    pmulhrsw             m1, m13
    psubb                m7, [base+pb_32to1]
    packuswb             m3, m1
    vpermq               m3, m3, q1320
    vpblendvb            m3, [rsp+32], m7
    mova           [rsp+32], m3
    jmp .w8_main
.w8_upsample_left:
    call .upsample_left
.w8_main:
    movd                xm3, dxd
    lea                  r5, [rsp+56]  ; left-7
    pshufd              xm1, xm5, q3120
    pand                xm5, xm14
    vpbroadcastw         m3, xm3
    pxor                xm0, xm0
    psubw               xm2, xm15, xm5
    psraw               xm1, 6
    lea                  r9, [strideq*3]
    paddw                m7, m3, m3
    psubw               xm9, xm0, xm1  ; base_y
    psllw               xm5, 8
    punpcklwd           xm8, xm9, xm0  ; base_y 0, 1, 4, 5
    vpblendd             m3, m7, 0xf0  ; xpos0 xpos1
    por                 xm5, xm2       ; 64-frac_y, frac_y
    punpckhwd           xm9, xm0       ; base_y 2, 3, 6, 7
    paddw                m6, m3
    vinserti128         m12, m5, xm5, 1
.w8_loop:
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6         ; base_x0
    movu                xm0, [rsp+r2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6         ; base_x1
    vinserti128          m0, [rsp+r3], 1
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6         ; base_x2
    movu                xm1, [rsp+r2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6         ; base_x3
    vinserti128          m1, [rsp+r3], 1
    pand                 m2, m14, m6
    paddsw               m4, m6, m7
    psubw                m5, m15, m2
    psllw                m2, 8
    pshufb               m0, m10
    por                  m2, m5
    pmaddubsw            m0, m2
    pand                 m2, m14, m4
    psubw                m5, m15, m2
    psllw                m2, 8
    pshufb               m1, m10
    por                  m2, m5
    pmaddubsw            m1, m2
    cmp                 r3d, 64
    jge .w8_toponly
    mova                 m5, m7
    vpgatherdq           m3, [r5+xm9], m7
    mova                 m7, m5
    vpgatherdq           m2, [r5+xm8], m5
    pshufb               m3, m11
    pshufb               m2, m11
    punpckldq            m5, m2, m3    ; a0 b0 c0 d0 a1 b1 c1 d1   e0 f0 g0 h0 e1 f1 g1 h1
    punpckhdq            m2, m3        ; a2 b2 c2 d2 a3 b3 c3 d3   e2 f2 g2 h2 e3 f3 g3 h3
    vpermq               m5, m5, q3120 ; y0 y1
    vpermq               m2, m2, q3120 ; y2 y3
    pmaddubsw            m5, m12
    pmaddubsw            m2, m12
    psraw                m6, 15        ; base_x < topleft
    vpblendvb            m0, m5, m6
    psraw                m3, m4, 15
    vpblendvb            m1, m2, m3
.w8_toponly:
    pmulhrsw             m0, m13
    pmulhrsw             m1, m13
    paddw                m6, m4, m7     ; xpos += dx
    add                  r5, dyq
    packuswb             m0, m1
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*2], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+r9       ], xm1
    sub                  hd, 4
    jz .w8_end
    lea                dstq, [dstq+strideq*4]
    cmp                 r2d, r8d
    jge .w8_loop
.w8_leftonly_loop:
    mova                 m0, m7
    vpgatherdq           m5, [r5+xm9], m7
    mova                 m7, m0
    vpgatherdq           m3, [r5+xm8], m0
    add                  r5, dyq
    pshufb               m2, m5, m11
    pshufb               m1, m3, m11
    punpckldq            m0, m1, m2
    punpckhdq            m1, m2
    vpermq               m0, m0, q3120
    vpermq               m1, m1, q3120
    pmaddubsw            m0, m12
    pmaddubsw            m1, m12
    pmulhrsw             m0, m13
    pmulhrsw             m1, m13
    packuswb             m0, m1
    vextracti128        xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*2], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+r9       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_leftonly_loop
.w8_end:
    RET
.w16:
    mov                 r8d, hd
    test             angled, 0x400
    jnz .w16_main
    lea                 r3d, [hq+15]
    sub              angled, 90
    call .filter_strength
    test                r3d, r3d
    jz .w16_no_filter_above
    popcnt              r3d, r3d
    vbroadcasti128       m6, [tlq+1]
    mova                xm2, [base+z_filter_s]
    vinserti128          m2, [base+z_filter_s+14], 1 ; 00 01 12 23 34 45 56 67   67 78 89 9a ab bc cd de
    movu                xm3, [base+z_filter_s+8]
    vinserti128          m3, [base+z_filter_s+22], 1 ; 34 45 56 67 78 89 9a ab   ab bc cd de ef ff ff ff
    vpblendd             m1, m6, 0xf0
    vpbroadcastd         m0, [base+z_filter_k-4+r3*4+12*0]
    vpbroadcastd         m4, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd         m5, [base+z_filter_k-4+r3*4+12*2]
    pshufb               m2, m1, m2
    pshufb               m1, m3
    pmaddubsw            m0, m2, m0
    shufps               m2, m1, q2121                ; 12 23 34 45 56 67 78 89   89 9a ab bc cd de ef ff
    pmaddubsw            m2, m4
    pmaddubsw            m1, m5
    movd                xm4, r6m ; max_width
    pminsw              xm4, xm15
    vpbroadcastb        xm4, xm4
    paddw                m0, m2
    paddw                m0, m1
    pmulhrsw             m0, m13
    psubb               xm4, [base+pb_1to32]
    vextracti128        xm2, m0, 1
    packuswb            xm0, xm2
    vpblendvb           xm0, xm6, xm4
    movu           [rsp+65], xm0
.w16_no_filter_above:
    vpbroadcastd         m0, [base+pb_90]
    psubb                m0, m7
    pand                 m0, m8
    pcmpgtb              m0, m9
    pmovmskb            r3d, m0
    test                r3d, r3d
    jz .w16_main
    popcnt              r3d, r3d
    vpbroadcastd         m7, [base+z_filter_k-4+r3*4+12*0]
    vpbroadcastd         m8, [base+z_filter_k-4+r3*4+12*1]
    vpbroadcastd         m9, [base+z_filter_k-4+r3*4+12*2]
.w16_filter_left:
    movd                xm6, r7m ; max_height
    pminsw              xm6, xm15
    vpbroadcastb         m6, xm6
    cmp                  hd, 32
    jl .w16_filter_left_h16
    vpbroadcastd        xm0, [base+pb_5]
    vbroadcasti128      m10, [base+z_filter_s+ 8]
    vbroadcasti128      m11, [base+z_filter_s+12]
    vbroadcasti128      m12, [base+z_filter_s+16]
    je .w16_filter_left_h32
    movu                 m3, [tlq-69]
    movu                 m5, [tlq-61]
    pmaxub               m1, m10, m0
    pshufb               m1, m3, m1
    pmaddubsw            m1, m7
    pshufb               m2, m3, m11
    pmaddubsw            m2, m8
    pshufb               m3, m12
    pmaddubsw            m3, m9
    paddw                m1, m2
    pshufb               m2, m5, m10
    pmaddubsw            m2, m7
    pshufb               m4, m5, m11
    pmaddubsw            m4, m8
    pshufb               m5, m12
    pmaddubsw            m5, m9
    paddw                m1, m3
    vpbroadcastd         m3, [base+pb_32]
    paddb                m3, [base+pb_32to1]
    paddw                m2, m4
    paddw                m2, m5
    pmulhrsw             m1, m13
    pmulhrsw             m2, m13
    psubb                m3, m6, m3
    packuswb             m1, m2
    vpblendvb            m1, [tlq-64], m3
    mova              [rsp], m1
    jmp .w16_filter_left_top32
.w16_filter_left_h32:
    pmaxub              m10, m0
.w16_filter_left_top32:
    movu                xm2, [tlq-37]
    vinserti128          m2, [tlq-29], 1
    pshufb               m3, m2, m10
    pshufb               m1, m2, m11
    pshufb               m2, m12
    pmaddubsw            m3, m7
    pmaddubsw            m1, m8
    pmaddubsw            m2, m9
    paddw                m3, m1
    paddw                m3, m2
    pmulhrsw             m3, m13
    jmp .w16_filter_left_top16
.w16_filter_left_h16:
    mov                 r5d, 10
    cmp                  hd, 16
    cmovs               r5d, hd
    xor                 r5d, 15 ; h == 16 ? 5 : 15 - h
    movd                xm0, r5d
    vpbroadcastb         m0, xm0
.w16_filter_left_top16:
    movu                xm2, [tlq-15]
    vinserti128          m2, [tlq-21], 1
    vbroadcasti128       m1, [base+z_filter_s+12]
    vbroadcasti128       m4, [base+z_filter_s+16]
    vinserti128          m5, m1, [base+z_filter_s+8], 1 ; 56 67 78 89 9a ab bc cd   34 45 56 67 78 89 9a ab
    vpblendd             m1, m4, 0x0f                   ; 78 89 9a ab bc cd de ef   56 67 78 89 9a ab bc cd
    vinserti128          m4, [base+z_filter_s+20], 0    ; 9a ab bc cd de ef ff ff   78 89 9a ab bc cd de ef
    pmaxub               m0, m5
    pshufb               m0, m2, m0
    pmaddubsw            m0, m7
    pshufb               m1, m2, m1
    pmaddubsw            m1, m8
    pshufb               m2, m4
    pmaddubsw            m2, m9
    psubb                m6, [base+pb_32to1]
    paddw                m1, m0
    paddw                m1, m2
    pmulhrsw             m1, m13
    packuswb             m3, m1
    vpermq               m3, m3, q1320
    vpblendvb            m3, [tlq-32], m6
    mova           [rsp+32], m3
.w16_main:
    movd                xm1, dyd
    vbroadcasti128      m10, [base+z_filter_s+2]
    movd                xm7, dxd
    vbroadcasti128      m11, [base+z2_shuf_h2]
    vpbroadcastw         m1, xm1
    vpbroadcastw         m7, xm7
    mov                  r7, dstq
    pmullw               m0, m1, [base+z2_ymul]
    psllw               xm1, 4
    paddw                m6, m7, [base+z2_base_inc]
    lea                 r9d, [dxq+(65<<6)] ; xpos
    movd          [rsp+156], xm1
.w16_loop0:
    mov                 r2d, r9d
    mova          [rsp+160], m0
    lea                  r5, [rsp+60] ; left-3
    mova          [rsp+192], m6
    pxor                 m1, m1
    psraw                m2, m0, 6
    pand                 m0, m14
    psubw                m9, m1, m2   ; base_y
    psubw               m12, m15, m0
    punpcklwd            m8, m9, m1   ; base_y  0,  1,  2,  3,     8,  9, 10, 11
    psllw                m0, 8
    punpckhwd            m9, m1       ; base_y  4,  5,  6,  7,    12, 13, 14, 15
    por                 m12, m0       ; 64-frac_y, frac_y
.w16_loop:
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6        ; base_x0
    movu                xm0, [rsp+r2]
    vinserti128          m0, [rsp+r2+8], 1
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6        ; base_x1
    movu                xm1, [rsp+r3]
    vinserti128          m1, [rsp+r3+8], 1
    pand                 m2, m14, m6
    paddsw               m5, m6, m7
    psubw                m3, m15, m2
    psllw                m2, 8
    pshufb               m0, m10
    por                  m2, m3
    pmaddubsw            m0, m2
    pand                 m2, m14, m5
    psubw                m3, m15, m2
    psllw                m2, 8
    pshufb               m1, m10
    por                  m2, m3
    pmaddubsw            m1, m2
    cmp                 r3d, 64
    jge .w16_toponly
    punpckhwd            m2, m5, m5   ; mask out unnecessary loads
    vpgatherdd           m4, [r5+m9], m2
    punpcklwd            m2, m5, m5
    vpgatherdd           m3, [r5+m8], m2
    pshufb               m4, m11      ; e0 f0 g0 h0 e1 f1 g1 h1   m0 n0 o0 p0 m1 n1 o1 p1
    pshufb               m3, m11      ; a0 b0 c0 d0 a1 b1 c1 d1   i0 j0 k0 l0 i1 j1 k1 l1
    punpcklqdq           m2, m3, m4   ; y0
    punpckhqdq           m3, m4       ; y1
    pmaddubsw            m2, m12
    pmaddubsw            m3, m12
    psraw                m6, 15       ; base_x < topleft
    vpblendvb            m0, m2, m6
    psraw                m6, m5, 15
    vpblendvb            m1, m3, m6
.w16_toponly:
    pmulhrsw             m0, m13
    pmulhrsw             m1, m13
    paddw                m6, m5, m7   ; xpos += dx
    sub                  r5, 2
    packuswb             m0, m1
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    sub                  hd, 2
    jz .w16_end
    lea                dstq, [dstq+strideq*2]
    cmp                 r2d, (63-16)<<6
    jge .w16_loop
.w16_leftonly_loop:
    mova                 m0, m7
    vpgatherdd           m4, [r5+m9], m7
    mova                 m7, m0
    vpgatherdd           m3, [r5+m8], m0
    sub                  r5, 2
    pshufb               m2, m4, m11
    pshufb               m1, m3, m11
    punpcklqdq           m0, m1, m2
    punpckhqdq           m1, m2
    pmaddubsw            m0, m12
    pmaddubsw            m1, m12
    pmulhrsw             m0, m13
    pmulhrsw             m1, m13
    packuswb             m0, m1
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_leftonly_loop
.w16_end:
    sub                 r8d, 1<<8
    jl .w16_ret
    vpbroadcastd         m0, [rsp+156]
    paddw                m0, [rsp+160] ; base_y += 16*dy
    paddw                m6, m13, [rsp+192]
    add                  r7, 16
    add                 r9d, 16<<6
    movzx                hd, r8b
    mov                dstq, r7
    paddw                m6, m13 ; base_x += 16*64
    jmp .w16_loop0
.w16_ret:
    RET
.w32:
    mova                 m2, [tlq+32]
    lea                 r8d, [hq+(1<<8)]
    mova           [rsp+96], m2
    test             angled, 0x400
    jnz .w16_main
    vpbroadcastd         m7, [base+z_filter_k+4*2+12*0]
    vpbroadcastd         m8, [base+z_filter_k+4*2+12*1]
    vpbroadcastd         m9, [base+z_filter_k+4*2+12*2]
    mova                xm5, [base+z_filter_s]
    vinserti128          m5, [base+z_filter_s+10], 1 ; 00 01 12 23 34 45 56 67   45 56 67 78 89 9a ab bc
    vinserti128          m1, [tlq+11], 1
    movu                xm6, [base+z_filter_s+12]
    vinserti128          m6, [base+z_filter_s+22], 1 ; 56 67 78 89 9a ab bc cd   ab bc cd de ef ff ff ff
    movu                xm3, [tlq+ 6]
    vinserti128          m3, [tlq+17], 1
    movd                xm0, r6m ; max_width
    pminsw              xm0, xm15
    vpbroadcastb        m10, xm0
.w32_filter_above:
    pshufb               m0, m1, m5
    shufps               m4, m5, m6, q1021           ; 12 23 34 45 56 67 78 89   67 78 89 9a ab bc cd de
    pmaddubsw            m0, m7
    pshufb               m2, m1, m4
    shufps               m5, m6, q2132               ; 34 45 56 67 78 89 9a ab   89 9a ab bc cd de ef ff
    pmaddubsw            m2, m8
    pshufb               m1, m5
    pmaddubsw            m1, m9
    paddw                m0, m2
    paddw                m0, m1
    pshufb               m1, m3, m4
    pmaddubsw            m1, m7
    pshufb               m2, m3, m5
    pmaddubsw            m2, m8
    pshufb               m3, m6
    pmaddubsw            m3, m9
    paddw                m1, m2
    paddw                m1, m3
    pmulhrsw             m0, m13
    pmulhrsw             m1, m13
    psubb               m10, [base+pb_1to32]
    packuswb             m0, m1
    vpblendvb            m0, [tlq+1], m10
    movu           [rsp+65], m0
    jmp .w16_filter_left
.w64:
    mova                 m2, [tlq+32]
    mov                 r3d, [tlq+64]
    lea                 r8d, [hq+(3<<8)]
    mova          [rsp+ 96], m2
    mov           [rsp+128], r3d
    test             angled, 0x400
    jnz .w16_main
    vpbroadcastd         m7, [base+z_filter_k+4*2+12*0]
    vpbroadcastd         m8, [base+z_filter_k+4*2+12*1]
    vpbroadcastd         m9, [base+z_filter_k+4*2+12*2]
    movu                xm6, [base+z_filter_s+ 4]
    vinserti128          m6, [base+z_filter_s+10], 1 ; 12 23 34 45 56 67 78 89   45 56 67 78 89 9a ab bc
    movu                xm3, [tlq+30]
    vinserti128          m3, [tlq+43], 1
    movu                xm5, [base+z_filter_s+16]
    vinserti128          m5, [base+z_filter_s+22], 1 ; 78 89 9a ab bc cd de ef   ab bc cd de ef ff ff ff
    pshufb               m0, m3, m6
    shufps               m4, m6, m5, q1021           ; 34 45 56 67 78 89 9a ab   67 78 89 9a ab bc cd de
    pmaddubsw            m0, m7
    pshufb               m2, m3, m4
    shufps               m6, m5, q2132               ; 56 67 78 89 9a ab bc cd   89 9a ab bc cd de ef ff
    pmaddubsw            m2, m8
    pshufb               m3, m6
    pmaddubsw            m3, m9
    paddw                m0, m2
    paddw                m0, m3
    movu                xm2, [tlq+36]
    vinserti128          m2, [tlq+49], 1
    pshufb               m4, m2, m4
    pmaddubsw            m4, m7
    pshufb               m3, m2, m6
    pmaddubsw            m3, m8
    pshufb               m2, m5
    pmaddubsw            m2, m9
    movd                xm5, r6m ; max_width
    pminsw              xm5, xm15
    vpbroadcastb        m10, xm5
    paddw                m3, m4
    paddw                m2, m3
    vpbroadcastd         m3, [base+pb_32]
    pmulhrsw             m0, m13
    pmulhrsw             m2, m13
    mova                xm5, [base+z_filter_s]
    vinserti128          m5, [base+z_filter_s+6], 1
    psubb                m3, m10, m3
    psubb                m3, [base+pb_1to32]
    vinserti128          m1, [tlq+13], 1
    packuswb             m0, m2
    vpblendvb            m0, [tlq+33], m3
    movu                xm3, [tlq+ 6]
    vinserti128          m3, [tlq+19], 1
    movu           [rsp+97], m0
    jmp .w32_filter_above

cglobal ipred_z3_8bpc, 4, 9, 0, dst, stride, tl, w, h, angle, dy, org_w, maxbase
    %assign org_stack_offset stack_offset
    lea                  r6, [ipred_z3_avx2_table]
    tzcnt                hd, hm
    movifnidn        angled, anglem
    lea                  r7, [dr_intra_derivative+45*2-1]
    dec                 tlq
    movsxd               hq, [r6+hq*4]
    sub              angled, 180
    add                  hq, r6
    mov                 dyd, angled
    neg                 dyd
    xor              angled, 0x400
    or                  dyq, ~0x7e
    movzx               dyd, word [r7+dyq]
    vpbroadcastd         m3, [pw_512]
    vpbroadcastd         m4, [pw_62]
    vpbroadcastd         m5, [pw_64]
    mov              org_wd, wd
    jmp                  hq
.h4:
    lea                  r7, [strideq*3]
    cmp              angleb, 40
    jae .h4_no_upsample
    lea                 r4d, [angleq-1024]
    sar                 r4d, 7
    add                 r4d, wd
    jg .h4_no_upsample ; !enable_intra_edge_filter || w > 8 || (w == 8 && is_sm)
    ALLOC_STACK         -32, 9
    movu                xm8, [tlq-7]
    pshufb              xm0, xm8, [z_upsample1-4]
    vpbroadcastb        xm2, xm8
    pshufb              xm1, xm8, [z_filter_s+2]
    mova           [rsp+16], xm2 ; top[max_base_y]
    vpbroadcastd        xm2, [pb_36_m4]
    add                 dyd, dyd
    pmaddubsw           xm0, xm2
    pmaddubsw           xm1, xm2
    movd                xm7, dyd
    mov                 r2d, dyd
    vpbroadcastw         m7, xm7
    paddw               xm1, xm0
    pmulhrsw            xm1, xm3
    pslldq               m6, m7, 8
    paddw               xm2, xm7, xm7
    paddw                m6, m7
    packuswb            xm1, xm1
    paddw                m6, m2
    punpcklbw           xm1, xm8
    mova                xm8, [z_transpose4]
    psllw                m7, 2
    pshufb              xm1, [pb_15to0]
    mova              [rsp], xm1
.h4_upsample_loop:
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6
    vpbroadcastq         m1, [rsp+r2]
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6
    vpbroadcastq         m2, [rsp+r4]
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6
    movq                xm0, [rsp+r2]
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6
    movhps              xm0, [rsp+r4]
    vpblendd             m1, m2, 0xc0
    pand                 m2, m4, m6
    vpblendd             m0, m1, 0xf0
    psubw                m1, m5, m2
    psllw                m2, 8
    por                  m1, m2
    pmaddubsw            m0, m1
    paddw                m6, m7
    pmulhrsw             m0, m3
    vextracti128        xm1, m0, 1
    packuswb            xm1, xm0
    pshufb              xm1, xm8
    movd   [dstq+strideq*0], xm1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+r7       ], xm1, 3
    add                dstq, 4
    sub                  wd, 4
    jg .h4_upsample_loop
    RET
ALIGN function_align
.filter_strength: ; h4/h8/h16
%define base r4-z_filter_t0
    lea                  r4, [z_filter_t0]
    movd                xm0, maxbased
    movd                xm2, angled
    shr              angled, 8 ; is_sm << 1
    vpbroadcastb         m0, xm0
    vpbroadcastb         m2, xm2
    pcmpeqb              m1, m0, [base+z_filter_wh]
    pand                 m1, m2
    mova                xm2, [r4+angleq*8]
    pcmpgtb              m1, m2
    pmovmskb            r5d, m1
    ret
.h4_no_upsample:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -16, 12
    mov            maxbased, 7
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .h4_main
    lea            maxbased, [wq+3]
    call .filter_strength
    mov            maxbased, 7
    test                r5d, r5d
    jz .h4_main ; filter_strength == 0
    popcnt              r5d, r5d
    vpbroadcastd         m7, [base+pb_7]
    vbroadcasti128       m2, [tlq-14]
    pmaxub               m1, m7, [base+z_filter_s-4]
    vpbroadcastd         m8, [base+z_filter_k-4+r5*4+12*0]
    pmaxub               m7, [base+z_filter_s+4]
    vpbroadcastd         m9, [base+z_filter_k-4+r5*4+12*1]
    vpbroadcastd        m10, [base+z_filter_k-4+r5*4+12*2]
    pshufb               m0, m2, m1
    shufps               m1, m7, q2121
    pmaddubsw            m0, m8
    pshufb               m1, m2, m1
    pmaddubsw            m1, m9
    pshufb               m2, m7
    pmaddubsw            m2, m10
    paddw                m0, m1
    paddw                m0, m2
    pmulhrsw             m0, m3
    mov                 r4d, 9
    lea                 tlq, [rsp+15]
    cmp                  wd, 4
    cmovne         maxbased, r4d
    vextracti128        xm1, m0, 1
    packuswb            xm0, xm1
    mova              [rsp], xm0
.h4_main:
    movd                xm6, dyd
    vpbroadcastq         m0, [z_base_inc] ; base_inc << 6
    mov                  r4, tlq
    sub                 tlq, 4
    neg                 dyq
    vpbroadcastw         m6, xm6
    sub                  r4, maxbaseq
    shl            maxbased, 6
    vpbroadcastb         m7, [r4]
    lea                  r4, [dyq+63] ; ypos
    movd                xm9, maxbased
    not            maxbased
    vbroadcasti128       m8, [z3_shuf_w4]
    add            maxbased, 64
    vpbroadcastw         m9, xm9
    psrlw                m7, 8  ; top[max_base_y]
    paddw               m10, m6, m6
    psubw                m9, m0 ; max_base_y
    vpblendd             m6, m10, 0xcc
    mova                xm0, xm10
    paddw                m6, m0 ; ypos2 ypos3 ypos0 ypos1
    paddw               m10, m10
    mova               xm11, [z_transpose4]
.h4_loop:
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base0
    vpbroadcastq         m1, [tlq+r4]
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base1
    vpbroadcastq         m2, [tlq+r5]
    lea                  r5, [r4+dyq]
    sar                  r4, 6 ; base2
    movq                xm0, [tlq+r4]
    lea                  r4, [r5+dyq]
    sar                  r5, 6 ; base3
    movhps              xm0, [tlq+r5]
    vpblendd             m1, m2, 0xc0
    pand                 m2, m4, m6 ; frac
    vpblendd             m0, m1, 0xf0
    psubw                m1, m5, m2 ; 64-frac
    psllw                m2, 8
    pshufb               m0, m8
    por                  m1, m2     ; 64-frac, frac
    pmaddubsw            m0, m1
    pcmpgtw              m1, m9, m6 ; base < max_base_y
    pmulhrsw             m0, m3
    paddw                m6, m10    ; ypos += dy
    vpblendvb            m0, m7, m0, m1
    vextracti128        xm1, m0, 1
    packuswb            xm1, xm0
    pshufb              xm1, xm11   ; transpose
    movd   [dstq+strideq*0], xm1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+r7       ], xm1, 3
    sub                  wd, 4
    jz .h4_end
    add                dstq, 4
    cmp                 r4d, maxbased
    jg .h4_loop
    packuswb            xm7, xm7
.h4_end_loop:
    movd   [dstq+strideq*0], xm7
    movd   [dstq+strideq*1], xm7
    movd   [dstq+strideq*2], xm7
    movd   [dstq+r7       ], xm7
    add                dstq, 4
    sub                  wd, 4
    jg .h4_end_loop
.h4_end:
    RET
ALIGN function_align
.h8:
    lea                 r4d, [angleq+216]
    mov                 r4b, wb
    cmp                 r4d, 8
    ja .h8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || w > 8
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -32, 8
    and                 r4d, 4
    mova                xm0, [tlq-15]
    vinserti128          m0, [tlq- 9], 1
    movd                xm1, r4d
    movu                xm2, [z_filter_s+2]
    vinserti128          m2, [z_filter_s+6], 1
    vpbroadcastb        xm1, xm1 ; w & 4
    vpbroadcastd         m7, [pb_36_m4]
    pmaxub              xm1, [z_upsample1-4] ; clip 4x8
    vinserti128          m1, [z_upsample1], 1
    add                 dyd, dyd
    pshufb               m1, m0, m1
    pshufb               m2, m0, m2
    vinserti128          m0, [tlq-7], 1
    movd                xm6, dyd
    pmaddubsw            m1, m7
    pmaddubsw            m2, m7
    vpbroadcastw         m6, xm6
    mov                 r2d, dyd
    lea                  r5, [strideq*3]
    paddw                m7, m6, m6
    paddw                m1, m2
    vpblendd             m6, m7, 0xf0
    pmulhrsw             m1, m3
    pslldq               m2, m7, 8
    paddw                m7, m7
    paddw                m6, m2
    vbroadcasti128       m2, [pb_15to0]
    packuswb             m1, m1
    punpcklbw            m1, m0
    pshufb               m1, m2
    vextracti128   [rsp+ 0], m1, 1
    mova           [rsp+16], xm1
.h8_upsample_loop:
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6 ; base0
    movu                xm0, [rsp+r2]
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6 ; base1
    vinserti128          m0, [rsp+r4], 1
    lea                 r4d, [r2+dyq]
    shr                 r2d, 6 ; base2
    pand                 m1, m4, m6
    psubw                m2, m5, m1
    psllw                m1, 8
    por                  m2, m1
    punpcklqdq           m1, m2, m2 ; frac0 frac1
    pmaddubsw            m0, m1
    movu                xm1, [rsp+r2]
    lea                 r2d, [r4+dyq]
    shr                 r4d, 6 ; base3
    vinserti128          m1, [rsp+r4], 1
    punpckhqdq           m2, m2 ; frac2 frac3
    pmaddubsw            m1, m2
    pmulhrsw             m0, m3
    paddw                m6, m7
    pmulhrsw             m1, m3
    lea                  r4, [dstq+strideq*4]
    psllw                m1, 8
    por                  m0, m1
    vextracti128        xm1, m0, 1
    punpcklbw           xm2, xm0, xm1
    punpckhbw           xm0, xm1
    movd   [dstq+strideq*0], xm2
    pextrd [dstq+strideq*1], xm2, 1
    pextrd [dstq+strideq*2], xm2, 2
    pextrd [dstq+r5       ], xm2, 3
    movd   [r4  +strideq*0], xm0
    pextrd [r4  +strideq*1], xm0, 1
    pextrd [r4  +strideq*2], xm0, 2
    pextrd [r4  +r5       ], xm0, 3
    add                dstq, 4
    sub                  wd, 4
    jg .h8_upsample_loop
    RET
.h8_no_intra_edge_filter:
    and            maxbased, 7
    or             maxbased, 8 ; imin(w+7, 15)
    jmp .h8_main
.h8_no_upsample:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -32, 10
    lea            maxbased, [wq+7]
    test             angled, 0x400
    jnz .h8_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .h8_main ; filter_strength == 0
    popcnt              r5d, r5d
    vpbroadcastd        xm6, [base+pb_15]
    pcmpeqb             xm1, xm1
    psubusb             xm6, xm0
    psubb               xm6, xm1 ; w == 4 ? 5 : 1
    movu                xm2, [tlq-16]
    pmaxub              xm1, xm6, [base+z_filter_s]
    vinserti128          m2, [tlq-14], 1
    vinserti128          m1, [base+z_filter_s+12], 1
    vpbroadcastd         m7, [base+z_filter_k-4+r5*4+12*0]
    pmaxub              xm6, [base+z_filter_s+ 8]
    vinserti128          m6, [base+z_filter_s+20], 1
    pshufb               m0, m2, m1
    pmaddubsw            m0, m7
    vpbroadcastd         m7, [base+z_filter_k-4+r5*4+12*1]
    movzx               r4d, byte [tlq-15]
    shufps               m1, m6, q2121
    pshufb               m1, m2, m1
    pmaddubsw            m1, m7
    paddw                m0, m1
    sub                 r5d, 3
    jnz .h8_3tap
    vpbroadcastd         m7, [z_filter_k+4*8]
    movzx               r2d, byte [tlq-14]
    pshufb               m2, m6
    pmaddubsw            m2, m7
    sub                 r2d, r4d
    lea                 r2d, [r2+r4*8+4]
    shr                 r2d, 3
    mov            [rsp+15], r2b
    paddw                m0, m2
.h8_3tap:
    pmulhrsw             m0, m3
    sar                 r5d, 1
    lea                 tlq, [rsp+31]
    add                 r5d, 17
    cmp                  wd, 16
    cmovns         maxbased, r5d
    neg                  r5
    mov            [tlq+r5], r4b
    vextracti128        xm1, m0, 1
    packuswb            xm0, xm1
    mova           [tlq-15], xm0
.h8_main:
    movd                xm2, dyd
    vbroadcasti128       m0, [z_base_inc]
    mov                  r4, tlq
    sub                 tlq, 8
    neg                 dyq
    vpbroadcastw         m2, xm2
    sub                  r4, maxbaseq
    shl            maxbased, 6
    vpbroadcastb         m7, [r4]
    lea                  r4, [dyq+63]
    movd                xm9, maxbased
    not            maxbased
    vbroadcasti128       m8, [z3_shuf]
    add            maxbased, 64
    vpbroadcastw         m9, xm9
    psrlw                m7, 8
    psubw                m9, m0
    paddw                m6, m2, m2
    vpblendd             m2, m6, 0x0f
.h8_loop:
    lea                  r5, [r4+dyq]
    sar                  r4, 6
    pand                 m0, m4, m2
    psubw                m1, m5, m0
    psllw                m0, 8
    por                  m1, m0
    vbroadcasti128       m0, [tlq+r4]
    lea                  r4, [r5+dyq]
    sar                  r5, 6
    vinserti128          m0, [tlq+r5], 0
    sub                 rsp, 8*2
    pshufb               m0, m8
    pmaddubsw            m0, m1
    pcmpgtw              m1, m9, m2
    paddw                m2, m6
    pmulhrsw             m0, m3
    vpblendvb            m0, m7, m0, m1
    vextracti128        xm1, m0, 1
    psllw               xm0, 8
    por                 xm0, xm1 ; interleave rows (partial transpose)
    mova              [rsp], xm0
    sub                  wd, 2
    jz .h8_transpose
    cmp                 r4d, maxbased
    jg .h8_loop
    packuswb            xm0, xm7, xm7
.h8_end_loop:
    sub                 rsp, 8*2
    mova              [rsp], xm0
    sub                  wd, 2
    jg .h8_end_loop
.h8_transpose:
    mova                xm2, [rsp+16*1]
    sub              org_wd, 8
    lea                  r2, [strideq*3]
    lea                  r6, [dstq+org_wq]
    cmovns             dstq, r6
    punpcklwd           xm1, xm2, xm0
    punpckhwd           xm2, xm0
    lea                  r6, [dstq+strideq*4]
    jge .h8_w8
    add                 rsp, 16*2
    movd   [dstq+strideq*0], xm1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+r2       ], xm1, 3
    movd   [r6  +strideq*0], xm2
    pextrd [r6  +strideq*1], xm2, 1
    pextrd [r6  +strideq*2], xm2, 2
    pextrd [r6  +r2       ], xm2, 3
    jmp .h8_end
.h8_w8_loop:
    mova                xm0, [rsp+16*0]
    mova                xm2, [rsp+16*1]
    punpcklwd           xm1, xm2, xm0
    punpckhwd           xm2, xm0
.h8_w8: ; w8/w16/w32
    mova                xm0, [rsp+16*2]
    mova                xm4, [rsp+16*3]
    add                 rsp, 16*4
    punpcklwd           xm3, xm4, xm0
    punpckhwd           xm4, xm0
    punpckldq           xm0, xm3, xm1
    punpckhdq           xm3, xm1
    punpckldq           xm1, xm4, xm2
    punpckhdq           xm4, xm2
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm3
    movhps [dstq+r2       ], xm3
    movq   [r6  +strideq*0], xm1
    movhps [r6  +strideq*1], xm1
    movq   [r6  +strideq*2], xm4
    movhps [r6  +r2       ], xm4
    sub                dstq, 8
    sub                  r6, 8
    sub              org_wd, 8
    jge .h8_w8_loop
.h8_end:
    RET
.h16_no_intra_edge_filter:
    and            maxbased, 15
    or             maxbased, 16 ; imin(w+15, 31)
    jmp .h16_main
ALIGN function_align
.h16:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -64, 12
    lea            maxbased, [wq+15]
    test             angled, 0x400
    jnz .h16_no_intra_edge_filter
    call .filter_strength
    test                r5d, r5d
    jz .h16_main ; filter_strength == 0
    popcnt              r5d, r5d
    vpbroadcastd        m11, [base+pb_27]
    vpbroadcastd         m1, [base+pb_1]
    vbroadcasti128       m6, [base+z_filter_s+12]
    vinserti128          m2, m6, [base+z_filter_s+4], 0
    vinserti128          m6, [base+z_filter_s+20], 1
    movu               xm10, [tlq-18]
    vinserti128         m10, [tlq-14], 1
    vpbroadcastd         m9, [base+z_filter_k-4+r5*4+12*0]
    vbroadcasti128       m7, [base+z_filter_s+8]
    vinserti128          m8, m7, [base+z_filter_s+0], 0
    vinserti128          m7, [base+z_filter_s+16], 1
    psubusb             m11, m0
    por                  m1, m11
    movu               xm11, [tlq-32]
    vinserti128         m11, [tlq-28], 1
    pmaxub               m8, m1
    pmaxub               m7, m1
    pshufb               m0, m10, m2
    shufps               m2, m6, q2121
    pmaddubsw            m0, m9
    pshufb               m1, m11, m8
    shufps               m8, m7, q2121
    pmaddubsw            m1, m9
    vpbroadcastd         m9, [base+z_filter_k-4+r5*4+12*1]
    movzx               r4d, byte [tlq-31]
    pshufb               m2, m10, m2
    pmaddubsw            m2, m9
    pshufb               m8, m11, m8
    pmaddubsw            m8, m9
    paddw                m0, m2
    paddw                m1, m8
    sub                 r5d, 3
    jnz .h16_3tap
    vpbroadcastd         m9, [z_filter_k+4*8]
    movzx               r2d, byte [tlq-30]
    pshufb              m10, m6
    pmaddubsw           m10, m9
    pshufb              m11, m7
    pmaddubsw           m11, m9
    sub                 r2d, r4d
    lea                 r2d, [r2+r4*8+4]
    shr                 r2d, 3
    mov            [rsp+31], r2b
    paddw                m0, m10
    paddw                m1, m11
.h16_3tap:
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    sar                 r5d, 1
    lea                 tlq, [rsp+63]
    add                 r5d, 33
    cmp                  wd, 32
    cmovns         maxbased, r5d
    neg                  r5
    mov            [tlq+r5], r4b
    packuswb             m0, m1
    vpermq               m0, m0, q2031
    mova           [tlq-31], m0
.h16_main:
    movd                xm6, dyd
    vbroadcasti128       m0, [z_base_inc]
    mov                  r4, tlq
    sub                 tlq, 8
    neg                 dyq
    vpbroadcastw         m6, xm6
    sub                  r4, maxbaseq
    shl            maxbased, 6
    vpbroadcastb         m7, [r4]
    lea                  r4, [dyq+63]
    movd                xm9, maxbased
    not            maxbased
    vbroadcasti128       m8, [z3_shuf]
    add            maxbased, 64
    vpbroadcastw         m9, xm9
    psubw                m9, m0
    paddw               m11, m6, m6
    psubw               m10, m9, m3 ; 64*8
    vpblendd             m6, m11, 0xf0
.h16_loop:
    lea                  r5, [r4+dyq]
    sar                  r4, 6
    pand                 m1, m4, m6
    psubw                m2, m5, m1
    psllw                m1, 8
    por                  m2, m1
    movu                xm0, [tlq+r4-0]
    movu                xm1, [tlq+r4-8]
    lea                  r4, [r5+dyq]
    sar                  r5, 6
    vinserti128          m0, [tlq+r5-0], 1
    vinserti128          m1, [tlq+r5-8], 1
    sub                 rsp, 32
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    pcmpgtw              m1, m9, m6
    pcmpgtw              m2, m10, m6
    packsswb             m1, m2
    paddw                m6, m11
    vpblendvb            m0, m7, m0, m1
    vpermq               m0, m0, q3120
    mova              [rsp], m0
    sub                  wd, 2
    jz .h16_transpose
    cmp                 r4d, maxbased
    jg .h16_loop
    mova                 m0, m7
.h16_end_loop:
    sub                 rsp, 32
    mova              [rsp], m7
    sub                  wd, 2
    jg .h16_end_loop
.h16_transpose:
    mova                 m2, [rsp+32*1]
    sub              org_wd, 8
    lea                  r2, [strideq*3]
    lea                  r6, [dstq+org_wq]
    cmovns             dstq, r6
    punpcklbw            m1, m2, m0
    punpckhbw            m2, m0
    lea                  r3, [strideq*5]
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    lea                  r4, [strideq+r2*2] ; stride*7
    jge .h16_w8
    add                 rsp, 32*2
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+r2       ], xm0, 3
    vextracti128        xm0, m0, 1
    movd   [dstq+strideq*4], xm1
    pextrd [dstq+r3       ], xm1, 1
    pextrd [dstq+r2*2     ], xm1, 2
    pextrd [dstq+r4       ], xm1, 3
    lea                dstq, [dstq+strideq*8]
    vextracti128        xm1, m1, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+r2       ], xm0, 3
    movd   [dstq+strideq*4], xm1
    pextrd [dstq+r3       ], xm1, 1
    pextrd [dstq+r2*2     ], xm1, 2
    pextrd [dstq+r4       ], xm1, 3
    jmp .h16_end
.h16_w8_loop:
    mova                 m0, [rsp+32*0]
    mova                 m2, [rsp+32*1]
    punpcklbw            m1, m2, m0
    punpckhbw            m2, m0
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
.h16_w8:
    mova                 m2, [rsp+32*2]
    mova                 m4, [rsp+32*3]
    lea                  r6, [dstq+strideq*8]
    add                 rsp, 32*4
    punpcklbw            m3, m4, m2
    punpckhbw            m4, m2
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    punpckldq            m4, m2, m0
    punpckhdq            m2, m0
    punpckldq            m0, m3, m1
    punpckhdq            m3, m1
    movq   [dstq+strideq*0], xm4
    movhps [dstq+strideq*1], xm4
    vextracti128        xm4, m4, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+r2       ], xm2
    vextracti128        xm2, m2, 1
    movq   [dstq+strideq*4], xm0
    movhps [dstq+r3       ], xm0
    vextracti128        xm0, m0, 1
    movq   [dstq+r2*2     ], xm3
    movhps [dstq+r4       ], xm3
    vextracti128        xm3, m3, 1
    movq     [r6+strideq*0], xm4
    movhps   [r6+strideq*1], xm4
    movq     [r6+strideq*2], xm2
    movhps   [r6+r2       ], xm2
    movq     [r6+strideq*4], xm0
    movhps   [r6+r3       ], xm0
    movq     [r6+r2*2     ], xm3
    movhps   [r6+r4       ], xm3
    sub                dstq, 8
    sub              org_wd, 8
    jge .h16_w8_loop
.h16_end:
    RET
ALIGN function_align
.h32:
    %assign stack_offset org_stack_offset
    ALLOC_STACK         -96, 15
    lea            maxbased, [wq+31]
    and            maxbased, 31
    or             maxbased, 32 ; imin(w+31, 63)
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .h32_main
    vbroadcasti128       m0, [pb_0to15]
    mov                 r4d, 21
    mov                 r5d, 3
    movu               xm11, [tlq-66]    ; 56-63
    vinserti128         m11, [tlq-52], 1 ; 40-47
    sub                 r4d, wd ; 21-w
    cmovns              r5d, r4d
    movu               xm12, [tlq-58]    ; 48-55
    vinserti128         m12, [tlq-44], 1 ; 32-39
    sub                 r4d, 8 ; 13-w
    movd                xm1, r5d
    movu               xm13, [tlq-34]    ; 24-31
    vinserti128         m13, [tlq-20], 1 ;  8-15
    movd                xm2, r4d
    vpbroadcastb         m1, xm1
    movu               xm14, [tlq-28]    ; 16-23
    vinserti128         m14, [tlq-14], 1 ;  0- 7
    vpbroadcastb         m2, xm2
    pmaxsb               m1, m0 ; clip 16x32 and (32|64)x32
    movu                 m7, [z_filter_s+4]
    pshufb              m11, m1
    vinserti128          m8, m7, [z_filter_s+8], 1
    vinserti128          m7, [z_filter_s+16], 0
    pmaxsb               m2, m0 ; clip 8x32
    vpbroadcastd         m9, [z_filter_k+4*2+12*0]
    pshufb              m12, m2
    pshufb               m0, m11, m8
    pmaddubsw            m0, m9
    pshufb               m2, m12, m8
    pmaddubsw            m2, m9
    pshufb               m1, m13, m8
    pmaddubsw            m1, m9
    shufps               m8, m7, q1021
    pshufb               m6, m14, m8
    pmaddubsw            m6, m9
    vpbroadcastd         m9, [z_filter_k+4*2+12*1]
    pshufb              m10, m11, m8
    pmaddubsw           m10, m9
    paddw                m0, m10
    pshufb              m10, m12, m8
    pmaddubsw           m10, m9
    paddw                m2, m10
    pshufb              m10, m13, m8
    pmaddubsw           m10, m9
    shufps               m8, m7, q2121
    paddw                m1, m10
    pshufb              m10, m14, m8
    pmaddubsw           m10, m9
    paddw                m6, m10
    vpbroadcastd         m9, [z_filter_k+4*2+12*2]
    pshufb              m11, m8
    pmaddubsw           m11, m9
    pshufb              m12, m8
    pmaddubsw           m12, m9
    movzx               r4d, byte [tlq-63]
    movzx               r2d, byte [tlq-62]
    paddw                m0, m11
    paddw                m2, m12
    pshufb              m13, m8
    pmaddubsw           m13, m9
    pshufb              m14, m7
    pmaddubsw           m14, m9
    paddw                m1, m13
    paddw                m6, m14
    sub                 r2d, r4d
    lea                 r2d, [r2+r4*8+4] ; edge case for 64x32
    pmulhrsw             m0, m3
    pmulhrsw             m2, m3
    pmulhrsw             m1, m3
    pmulhrsw             m6, m3
    shr                 r2d, 3
    mov            [rsp+31], r2b
    lea                 tlq, [rsp+95]
    mov            [tlq-65], r4b
    mov                 r4d, 65
    cmp                  wd, 64
    cmove          maxbased, r4d
    packuswb             m0, m2
    packuswb             m1, m6
    mova           [tlq-63], m0
    mova           [tlq-31], m1
.h32_main:
    movd                xm6, dyd
    mov                  r4, tlq
    sub                 tlq, 8
    neg                 dyq
    vpbroadcastw         m6, xm6
    sub                  r4, maxbaseq
    shl            maxbased, 6
    vpbroadcastb         m7, [r4]
    lea                  r4, [dyq+63]
    movd                xm9, maxbased
    not            maxbased
    vbroadcasti128       m8, [z3_shuf]
    add            maxbased, 64
    vpbroadcastw         m9, xm9
    psubw                m9, [z_base_inc]
    mova                m11, m6
    psubw               m10, m9, m3 ; 64*8
.h32_loop:
    mov                  r5, r4
    sar                  r5, 6
    pand                 m1, m4, m6
    psubw                m2, m5, m1
    psllw                m1, 8
    por                  m2, m1
    movu                xm0, [tlq+r5- 0]
    vinserti128          m0, [tlq+r5-16], 1
    movu                xm1, [tlq+r5- 8]
    vinserti128          m1, [tlq+r5-24], 1
    sub                 rsp, 32
    add                  r4, dyq
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    pcmpgtw              m1, m9, m6
    pcmpgtw              m2, m10, m6
    packsswb             m1, m2
    paddw                m6, m11
    vpblendvb            m0, m7, m0, m1
    mova              [rsp], m0
    dec                  wd
    jz .h32_transpose
    cmp                 r4d, maxbased
    jg .h32_loop
.h32_end_loop:
    sub                 rsp, 32
    mova              [rsp], m7
    dec                  wd
    jg .h32_end_loop
.h32_transpose:
    lea                dstq, [dstq+org_wq-8]
    lea                  r2, [strideq*3]
    lea                  r3, [strideq*5]
    lea                  r4, [strideq+r2*2] ; stride*7
.h32_w8_loop:
    mova                 m7, [rsp+32*0]
    mova                 m6, [rsp+32*1]
    mova                 m5, [rsp+32*2]
    mova                 m4, [rsp+32*3]
    mova                 m3, [rsp+32*4]
    mova                 m2, [rsp+32*5]
    mova                 m1, [rsp+32*6]
    mova                 m0, [rsp+32*7]
    lea                  r6, [dstq+strideq*8]
    add                 rsp, 32*8
    punpcklbw            m8, m0, m1
    punpckhbw            m0, m1
    punpcklbw            m1, m2, m3
    punpckhbw            m2, m3
    punpcklbw            m3, m4, m5
    punpckhbw            m4, m5
    punpcklbw            m5, m6, m7
    punpckhbw            m6, m7
    punpcklwd            m7, m8, m1
    punpckhwd            m8, m1
    punpcklwd            m1, m0, m2
    punpckhwd            m0, m2
    punpcklwd            m2, m3, m5
    punpckhwd            m3, m5
    punpcklwd            m5, m4, m6
    punpckhwd            m4, m6
    punpckldq            m6, m7, m2
    punpckhdq            m7, m2
    punpckldq            m2, m8, m3
    punpckhdq            m8, m3
    punpckldq            m3, m1, m5
    punpckhdq            m1, m5
    punpckldq            m5, m0, m4
    punpckhdq            m0, m4
    movq   [dstq+strideq*0], xm6
    movhps [dstq+strideq*1], xm6
    vextracti128        xm6, m6, 1
    movq   [dstq+strideq*2], xm7
    movhps [dstq+r2       ], xm7
    vextracti128        xm7, m7, 1
    movq   [dstq+strideq*4], xm2
    movhps [dstq+r3       ], xm2
    vextracti128        xm2, m2, 1
    movq   [dstq+r2*2     ], xm8
    movhps [dstq+r4       ], xm8
    vextracti128        xm8, m8, 1
    movq     [r6+strideq*0], xm3
    movhps   [r6+strideq*1], xm3
    vextracti128        xm3, m3, 1
    movq     [r6+strideq*2], xm1
    movhps   [r6+r2       ], xm1
    vextracti128        xm1, m1, 1
    movq     [r6+strideq*4], xm5
    movhps   [r6+r3       ], xm5
    vextracti128        xm5, m5, 1
    movq     [r6+r2*2     ], xm0
    movhps   [r6+r4       ], xm0
    lea                  r6, [r6+strideq*8]
    vextracti128        xm0, m0, 1
    movq     [r6+strideq*0], xm6
    movhps   [r6+strideq*1], xm6
    movq     [r6+strideq*2], xm7
    movhps   [r6+r2       ], xm7
    movq     [r6+strideq*4], xm2
    movhps   [r6+r3       ], xm2
    movq     [r6+r2*2     ], xm8
    movhps   [r6+r4       ], xm8
    lea                  r6, [r6+strideq*8]
    movq     [r6+strideq*0], xm3
    movhps   [r6+strideq*1], xm3
    movq     [r6+strideq*2], xm1
    movhps   [r6+r2       ], xm1
    movq     [r6+strideq*4], xm5
    movhps   [r6+r3       ], xm5
    movq     [r6+r2*2     ], xm0
    movhps   [r6+r4       ], xm0
    sub                dstq, 8
    sub              org_wd, 8
    jg .h32_w8_loop
    RET
ALIGN function_align
.h64:
    %assign stack_offset org_stack_offset
    ALLOC_STACK        -128, 16
    lea            maxbased, [wq+63]
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .h64_main
    mov                 r4d, 21
    vpbroadcastb       xm11, [tlq-127]
    vpblendd           xm11, [tlq-130], 0x0e ; 120-127
    sub                 r4d, wd ; 21-w
    mov                 r5d, 3
    vinserti128         m11, [tlq-116], 1    ; 104-111
    movu                 m7, [z_filter_s+4]
    cmp                  wd, 32
    cmove               r4d, r5d
    vinserti128          m8, m7, [z_filter_s+8], 1
    vbroadcasti128       m6, [pb_0to15]
    movd                xm1, r4d
    vpbroadcastd         m9, [z_filter_k+4*2+12*0]
    movu               xm12, [tlq-122]       ; 112-119
    vinserti128         m12, [tlq-108], 1    ;  96-103
    vpbroadcastb         m1, xm1
    movu               xm13, [tlq- 98]       ;  88- 95
    vinserti128         m13, [tlq- 84], 1    ;  72- 79
    movu               xm14, [tlq- 90]       ;  80- 87
    vinserti128         m14, [tlq- 76], 1    ;  64- 71
    vinserti128          m7, [z_filter_s+16], 0
    pshufb               m0, m11, m8
    pmaddubsw            m0, m9
    pshufb               m2, m12, m8
    pmaddubsw            m2, m9
    pmaxsb               m1, m6 ; clip (16|32)x64
    pshufb              m13, m1
    pshufb               m1, m13, m8
    pmaddubsw            m1, m9
    pshufb               m6, m14, m8
    pmaddubsw            m6, m9
    vpbroadcastd         m9, [z_filter_k+4*2+12*1]
    shufps              m15, m8, m7, q1021
    pshufb              m10, m11, m15
    pmaddubsw           m10, m9
    paddw                m0, m10
    pshufb              m10, m12, m15
    pmaddubsw           m10, m9
    paddw                m2, m10
    pshufb              m10, m13, m15
    pmaddubsw           m10, m9
    paddw                m1, m10
    pshufb              m10, m14, m15
    pmaddubsw           m10, m9
    paddw                m6, m10
    vpbroadcastd         m9, [z_filter_k+4*2+12*2]
    shufps              m10, m8, m7, q2132
    pshufb              m11, m10
    pmaddubsw           m11, m9
    pshufb              m12, m10
    pmaddubsw           m12, m9
    pshufb              m13, m10
    pmaddubsw           m13, m9
    pshufb              m14, m10
    pmaddubsw           m14, m9
    paddw                m0, m11
    paddw                m2, m12
    paddw                m1, m13
    paddw                m6, m14
    movu               xm11, [tlq-66]    ; 56-63
    vinserti128         m11, [tlq-52], 1 ; 40-47
    movu               xm12, [tlq-58]    ; 48-55
    vinserti128         m12, [tlq-44], 1 ; 32-39
    movu               xm13, [tlq-34]    ; 24-31
    vinserti128         m13, [tlq-20], 1 ;  8-15
    movu               xm14, [tlq-28]    ; 16-23
    vinserti128         m14, [tlq-14], 1 ;  0- 7
    pmulhrsw             m0, m3
    pmulhrsw             m2, m3
    pmulhrsw             m1, m3
    pmulhrsw             m6, m3
    lea                 tlq, [rsp+127]
    packuswb             m0, m2
    packuswb             m1, m6
    mova          [tlq-127], m0
    mova          [tlq- 95], m1
    pshufb               m0, m11, m10
    pmaddubsw            m0, m9
    pshufb               m2, m12, m10
    pmaddubsw            m2, m9
    pshufb               m1, m13, m10
    pmaddubsw            m1, m9
    pshufb               m6, m14, m7
    pmaddubsw            m6, m9
    vpbroadcastd         m9, [z_filter_k+4*2+12*1]
    pshufb               m7, m11, m15
    pmaddubsw            m7, m9
    paddw                m0, m7
    pshufb               m7, m12, m15
    pmaddubsw            m7, m9
    paddw                m2, m7
    pshufb               m7, m13, m15
    pmaddubsw            m7, m9
    paddw                m1, m7
    pshufb               m7, m14, m10
    pmaddubsw            m7, m9
    paddw                m6, m7
    vpbroadcastd         m9, [z_filter_k+4*2+12*0]
    pshufb              m11, m8
    pmaddubsw           m11, m9
    pshufb              m12, m8
    pmaddubsw           m12, m9
    pshufb              m13, m8
    pmaddubsw           m13, m9
    pshufb              m14, m15
    pmaddubsw           m14, m9
    paddw                m0, m11
    paddw                m2, m12
    paddw                m1, m13
    paddw                m6, m14
    pmulhrsw             m0, m3
    pmulhrsw             m2, m3
    pmulhrsw             m1, m3
    pmulhrsw             m6, m3
    packuswb             m0, m2
    packuswb             m1, m6
    mova           [tlq-63], m0
    mova           [tlq-31], m1
.h64_main:
    movd               xm12, dyd
    neg            maxbaseq
    vbroadcasti128       m8, [z3_shuf]
    vpbroadcastb         m7, [tlq+maxbaseq]
    shl            maxbased, 6
    vpbroadcastw        m12, xm12
    lea                 r5d, [dyq+maxbaseq-64]
    neg                 dyq
    or             maxbased, 63
    lea                  r4, [dyq+63]
    movd                xm6, r5d
    mova               xm10, [pb_1to32+16]
    vinserti128         m10, [pb_1to32], 1
    vpbroadcastd        m11, [pb_32]
    vpbroadcastw         m6, xm6
.h64_loop:
    mov                  r5, r4
    sar                  r5, 6
    movu                 m0, [tlq+r5-24]
    movu                 m1, [tlq+r5-32]
    pand                 m2, m4, m6
    psubw                m9, m5, m2
    psllw                m2, 8
    por                  m9, m2
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m9
    pmaddubsw            m1, m9
    psraw                m2, m6, 6
    sub                 rsp, 64
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packsswb             m2, m2
    paddb                m2, m10
    packuswb             m0, m1
    vpblendvb            m0, m7, m0, m2
    mova           [rsp+32], m0
    movu                 m0, [tlq+r5-56]
    movu                 m1, [tlq+r5-64]
    add                  r4, dyq
    pshufb               m0, m8
    pshufb               m1, m8
    pmaddubsw            m0, m9
    pmaddubsw            m1, m9
    paddb                m2, m11
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    paddw                m6, m12
    packuswb             m0, m1
    vpblendvb            m0, m7, m0, m2
    mova              [rsp], m0
    dec                  wd
    jz .h64_transpose
    cmp                 r4d, maxbased
    jg .h64_loop
.h64_end_loop:
    sub                 rsp, 64
    mova           [rsp+32], m7
    mova           [rsp+ 0], m7
    dec                  wd
    jg .h64_end_loop
.h64_transpose:
    lea                  r2, [strideq*3]
    lea                  r3, [strideq*5]
    imul                 r5, strideq, -8
    lea                dstq, [dstq+org_wq-16]
    lea                  r4, [strideq+r2*2] ; stride*7
.h64_transpose_loop0:
    lea                  r6, [rsp+16*3]
.h64_transpose_loop:
    mova                xm0, [r6+64*15]
    vinserti128          m0, [r6+64* 7], 1
    mova                xm1, [r6+64*14]
    vinserti128          m1, [r6+64* 6], 1
    mova                xm2, [r6+64*13]
    vinserti128          m2, [r6+64* 5], 1
    mova                xm3, [r6+64*12]
    vinserti128          m3, [r6+64* 4], 1
    mova                xm4, [r6+64*11]
    vinserti128          m4, [r6+64* 3], 1
    mova                xm5, [r6+64*10]
    vinserti128          m5, [r6+64* 2], 1
    mova                xm6, [r6+64* 9]
    vinserti128          m6, [r6+64* 1], 1
    mova                xm7, [r6+64* 8]
    vinserti128          m7, [r6+64* 0], 1
    sub                  r6, 16
    punpcklbw            m8, m0, m1
    punpckhbw            m0, m1
    punpcklbw            m1, m2, m3
    punpckhbw            m2, m3
    punpcklbw            m3, m4, m5
    punpckhbw            m4, m5
    punpcklbw            m5, m6, m7
    punpckhbw            m6, m7
    punpcklwd            m7, m8, m1
    punpckhwd            m8, m1
    punpcklwd            m1, m0, m2
    punpckhwd            m0, m2
    punpcklwd            m2, m3, m5
    punpckhwd            m3, m5
    punpcklwd            m5, m4, m6
    punpckhwd            m4, m6
    punpckldq            m6, m7, m2
    punpckhdq            m7, m2
    punpckldq            m2, m8, m3
    punpckhdq            m8, m3
    punpckldq            m3, m1, m5
    punpckhdq            m1, m5
    punpckldq            m5, m0, m4
    punpckhdq            m0, m4
    vpermq               m6, m6, q3120
    vpermq               m7, m7, q3120
    vpermq               m2, m2, q3120
    vpermq               m8, m8, q3120
    vpermq               m3, m3, q3120
    vpermq               m1, m1, q3120
    vpermq               m5, m5, q3120
    vpermq               m0, m0, q3120
    mova         [dstq+strideq*0], xm6
    vextracti128 [dstq+strideq*1], m6, 1
    mova         [dstq+strideq*2], xm7
    vextracti128 [dstq+r2       ], m7, 1
    mova         [dstq+strideq*4], xm2
    vextracti128 [dstq+r3       ], m2, 1
    mova         [dstq+r2*2     ], xm8
    vextracti128 [dstq+r4       ], m8, 1
    sub               dstq, r5
    mova         [dstq+strideq*0], xm3
    vextracti128 [dstq+strideq*1], m3, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+r2       ], m1, 1
    mova         [dstq+strideq*4], xm5
    vextracti128 [dstq+r3       ], m5, 1
    mova         [dstq+r2*2     ], xm0
    vextracti128 [dstq+r4       ], m0, 1
    sub                dstq, r5
    cmp                  r6, rsp
    jae .h64_transpose_loop
    add                 rsp, 64*16
    lea                dstq, [dstq+r5*8-16]
    sub              org_wd, 16
    jg .h64_transpose_loop0
.h64_end:
    RET

%macro FILTER_XMM 4 ; dst, src, tmp, shuf
%ifnum %4
    pshufb             xm%2, xm%4
%else
    pshufb             xm%2, %4
%endif
    pshufd             xm%1, xm%2, q0000 ; p0 p1
    pmaddubsw          xm%1, xm2
    pshufd             xm%3, xm%2, q1111 ; p2 p3
    pmaddubsw          xm%3, xm3
    paddw              xm%1, xm1
    paddw              xm%1, xm%3
    pshufd             xm%3, xm%2, q2222 ; p4 p5
    pmaddubsw          xm%3, xm4
    paddw              xm%1, xm%3
    pshufd             xm%3, xm%2, q3333 ; p6 __
    pmaddubsw          xm%3, xm5
    paddw              xm%1, xm%3
    psraw              xm%1, 4
    packuswb           xm%1, xm%1
%endmacro

%macro FILTER_YMM 4 ; dst, src, tmp, shuf
    pshufb              m%2, m%4
    pshufd              m%1, m%2, q0000
    pmaddubsw           m%1, m2
    pshufd              m%3, m%2, q1111
    pmaddubsw           m%3, m3
    paddw               m%1, m1
    paddw               m%1, m%3
    pshufd              m%3, m%2, q2222
    pmaddubsw           m%3, m4
    paddw               m%1, m%3
    pshufd              m%3, m%2, q3333
    pmaddubsw           m%3, m5
    paddw               m%1, m%3
    psraw               m%1, 4
    vperm2i128          m%3, m%1, m%1, 0x01
    packuswb            m%1, m%3
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

cglobal ipred_filter_8bpc, 3, 7, 0, dst, stride, tl, w, h, filter
%define base r6-ipred_filter_avx2_table
    lea                  r6, [filter_intra_taps]
    tzcnt                wd, wm
%ifidn filterd, filterm
    movzx           filterd, filterb
%else
    movzx           filterd, byte filterm
%endif
    shl             filterd, 6
    add             filterq, r6
    lea                  r6, [ipred_filter_avx2_table]
    movq                xm0, [tlq-3] ; _ 6 5 0 1 2 3 4
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m1, [base+pw_8]
    vbroadcasti128       m2, [filterq+16*0]
    vbroadcasti128       m3, [filterq+16*1]
    vbroadcasti128       m4, [filterq+16*2]
    vbroadcasti128       m5, [filterq+16*3]
    add                  wq, r6
    mov                  hd, hm
    jmp                  wq
.w4:
    WIN64_SPILL_XMM       9
    mova                xm8, [base+filter_shuf2]
    sub                 tlq, 3
    sub                 tlq, hq
    jmp .w4_loop_start
.w4_loop:
    pinsrd              xm0, xm6, [tlq+hq], 0
    lea                dstq, [dstq+strideq*2]
.w4_loop_start:
    FILTER_XMM            6, 0, 7, 8
    movd   [dstq+strideq*0], xm6
    pextrd [dstq+strideq*1], xm6, 1
    sub                  hd, 2
    jg .w4_loop
    RET
ALIGN function_align
.w8:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM      10
    mova                 m8, [base+filter_shuf1]
    FILTER_XMM            7, 0, 6, [base+filter_shuf2]
    vpbroadcastd         m0, [tlq+4]
    vpbroadcastd         m6, [tlq+5]
    sub                 tlq, 4
    sub                 tlq, hq
    vpbroadcastq         m7, xm7
    vpblendd             m7, m6, 0x20
.w8_loop:
    vpbroadcastd        xm6, [tlq+hq]
    palignr              m6, m0, 12
    vpblendd             m0, m6, m7, 0xeb     ; _ _ _ _ 1 2 3 4 6 5 0 _ _ _ _ _
                                              ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    mova                xm6, xm7
    call .main
    vpblendd            xm6, xm7, 0x0c
    pshufd              xm6, xm6, q3120
    movq   [dstq+strideq*0], xm6
    movhps [dstq+strideq*1], xm6
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
ALIGN function_align
.w16:
%if WIN64
    %assign stack_offset stack_offset - stack_size_padded
    %assign xmm_regs_used 15
    %assign stack_size_padded 0x98
    SUB                 rsp, stack_size_padded
%endif
    sub                  hd, 2
    TAIL_CALL .w16_main, 0
.w16_main:
%if WIN64
    movaps       [rsp+0xa8], xmm6
    movaps       [rsp+0xb8], xmm7
    movaps       [rsp+0x28], xmm8
    movaps       [rsp+0x38], xmm9
    movaps       [rsp+0x48], xmm10
    movaps       [rsp+0x58], xmm11
    movaps       [rsp+0x68], xmm12
    movaps       [rsp+0x78], xmm13
    movaps       [rsp+0x88], xmm14
%endif
    FILTER_XMM           12, 0, 7, [base+filter_shuf2]
    vpbroadcastd         m0, [tlq+5]
    vpblendd             m0, [tlq-12], 0x14
    mova                 m8, [base+filter_shuf1]
    vpbroadcastq         m7, xm12
    vpblendd             m0, m7, 0xc2         ; _ _ _ _ 1 2 3 4 6 5 0 _ _ _ _ _
                                              ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    call .main                                ; c0 d0 a1 b1   a1 b1 c0 d0
    movlps              xm9, xm7, [tlq+5]     ; _ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    vinserti128         m14, m8, [base+filter_shuf3], 0
    vpblendd           xm12, xm7, 0x0c        ; a0 b0 a1 b1
    FILTER_XMM            6, 9, 10, 14
    vpbroadcastq         m6, xm6              ; a2 b2 __ __ __ __ a2 b2
    vpbroadcastd         m9, [tlq+13]
    vpbroadcastd        m10, [tlq+12]
    psrld               m11, m8, 4
    vpblendd             m6, m9, 0x20         ; top
    sub                 tlq, 6
    sub                 tlq, hq
.w16_loop:
    vpbroadcastd        xm9, [tlq+hq]
    palignr              m9, m0, 12
    vpblendd             m0, m9, m7, 0xe2     ; _ _ _ _ 1 2 3 4 6 5 0 _ _ _ _ _
                                              ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    mova               xm13, xm7
    call .main                                ; e0 f0 c1 d1   c1 d1 e0 f0
    vpblendd             m9, m12, m10, 0xf0
    vpblendd            m12, m6, 0xc0
    pshufd               m9, m9, q3333
    vpblendd             m9, m6, 0xee
    vpblendd            m10, m9, m7, 0x0c     ; _ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
                                              ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    FILTER_YMM            6, 10, 9, 14        ; c2 d2 a3 b3   a3 b3 c2 d2
    vpblendd            m12, m6, 0x30         ; a0 b0 a1 b1   a3 b3 a2 b2
    vpermd               m9, m11, m12         ; a0 a1 a2 a3   b0 b1 b2 b3
    vpblendd           xm12, xm13, xm7, 0x0c  ; c0 d0 c1 d1
    mova         [dstq+strideq*0], xm9
    vextracti128 [dstq+strideq*1], m9, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_loop
    vpblendd            xm7, xm6, xm10, 0x04  ; _ _ _ 5 _ _ _ 6 0 _ _ _ 1 2 3 4
    pshufd              xm7, xm7, q1032       ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    FILTER_XMM            0, 7, 9, [base+filter_shuf1+16]
    vpblendd            xm6, xm0, 0x0c        ; c2 d2 c3 d3
    shufps              xm0, xm12, xm6, q2020 ; c0 c1 c2 c3
    shufps              xm6, xm12, xm6, q3131 ; d0 d1 d2 d3
    mova   [dstq+strideq*0], xm0
    mova   [dstq+strideq*1], xm6
    ret
ALIGN function_align
.w32:
    sub                 rsp, stack_size_padded
    sub                  hd, 2
    lea                  r3, [dstq+16]
    lea                 r5d, [hq-2]
    call .w16_main
    add                 tlq, r5
    mov                dstq, r3
    lea                  r3, [strideq-4]
    lea                  r4, [r3+strideq*2]
    movq                xm0, [tlq+21]
    pinsrd              xm0, [dstq-4], 2
    pinsrd              xm0, [dstq+r3*1], 3
    FILTER_XMM           12, 0, 7, 14         ; a0 b0 a0 b0
    movq                xm7, [dstq+r3*2]
    pinsrd              xm7, [dstq+r4], 2
    palignr             xm7, xm0, 12          ; 0 _ _ _ _ _ _ _ _ _ _ 5 _ _ _ 6
    vpbroadcastd         m0, [tlq+28]
    vpbroadcastd         m9, [tlq+29]
    vbroadcasti128       m8, [base+filter_shuf1+16]
    vpblendd             m0, m9, 0x20
    vpblendd             m0, m7, 0x0f
    vpbroadcastq         m7, xm12
    vpblendd             m0, m7, 0xc2         ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    call .main                                ; c0 d0 a1 b1   a1 b1 c0 d0
    add                  r3, 2
    lea                  r4, [r4+strideq*2]
    movlps              xm9, xm7, [tlq+29]    ; _ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    vpblendd           xm12, xm7, 0x0c        ; a0 b0 a1 b1
    FILTER_XMM            6, 9, 10, 14
    vpbroadcastq         m6, xm6              ; a2 b2 __ __ __ __ a2 b2
    vpbroadcastd         m9, [tlq+37]
    vpbroadcastd        m10, [tlq+36]
    vpblendd             m6, m9, 0x20         ; top
.w32_loop:
    movq                xm9, [dstq+r3*4]
    pinsrd              xm9, [dstq+r4], 2
.w32_loop_last:
    palignr              m9, m0, 12
    vpblendd             m0, m9, m7, 0xe2     ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    mova               xm13, xm7              ; c0 d0
    call .main                                ; e0 f0 c1 d1   c1 d1 e0 f0
    vpblendd             m9, m12, m10, 0xf0
    vpblendd            m12, m6, 0xc0
    pshufd               m9, m9, q3333
    vpblendd             m9, m6, 0xee
    vpblendd            m10, m9, m7, 0x0c     ; _ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
                                              ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    FILTER_YMM            6, 10, 9, 14        ; c2 d2 a3 b3   a3 b3 c2 d2
    vpblendd            m12, m6, 0x30         ; a0 b0 a1 b1   a3 b3 a2 b2
    vpermd               m9, m11, m12         ; a0 a1 a2 a3   b0 b1 b2 b3
    vpblendd           xm12, xm13, xm7, 0x0c  ; c0 d0 c1 d1
    mova         [dstq+strideq*0], xm9
    vextracti128 [dstq+strideq*1], m9, 1
    lea                dstq, [dstq+strideq*2]
    sub                 r5d, 2
    jg .w32_loop
    jz .w32_loop_last
    vpblendd            xm7, xm6, xm10, 0x04  ; _ _ _ 5 _ _ _ 6 0 _ _ _ 1 2 3 4
    pshufd              xm7, xm7, q1032       ; 0 _ _ _ 1 2 3 4 _ _ _ 5 _ _ _ 6
    FILTER_XMM            0, 7, 9, [base+filter_shuf1+16]
    vpblendd            xm6, xm0, 0x0c        ; c2 d2 c3 d3
    shufps              xm0, xm12, xm6, q2020 ; c0 c1 c2 c3
    shufps              xm6, xm12, xm6, q3131 ; d0 d1 d2 d3
    mova   [dstq+strideq*0], xm0
    mova   [dstq+strideq*1], xm6
    RET
ALIGN function_align
.main:
    FILTER_YMM            7, 0, 9, 8
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

cglobal ipred_cfl_top_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    lea                  t0, [ipred_cfl_left_avx2_table]
    tzcnt                wd, wm
    inc                 tlq
    movu                 m0, [tlq]
    movifnidn            hd, hm
    mov                 r6d, 0x8000
    shrx                r6d, r6d, wd
    movd                xm3, r6d
    movsxd               r6, [t0+wq*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, t0
    add                  t0, ipred_cfl_splat_avx2_table-ipred_cfl_left_avx2_table
    movsxd               wq, [t0+wq*4]
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6

cglobal ipred_cfl_left_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    mov                  hd, hm ; zero upper half
    tzcnt               r6d, hd
    sub                 tlq, hq
    tzcnt                wd, wm
    movu                 m0, [tlq]
    mov                 t0d, 0x8000
    shrx                t0d, t0d, r6d
    movd                xm3, t0d
    lea                  t0, [ipred_cfl_left_avx2_table]
    movsxd               r6, [t0+r6*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, t0
    add                  t0, ipred_cfl_splat_avx2_table-ipred_cfl_left_avx2_table
    movsxd               wq, [t0+wq*4]
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6
.h32:
    vextracti128        xm1, m0, 1
    paddw               xm0, xm1
.h16:
    punpckhqdq          xm1, xm0, xm0
    paddw               xm0, xm1
.h8:
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
.h4:
    pmaddwd             xm0, xm2
    pmulhrsw            xm0, xm3
    vpbroadcastw         m0, xm0
    jmp                  wq

cglobal ipred_cfl_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    movifnidn            hd, hm
    movifnidn            wd, wm
    tzcnt               r6d, hd
    lea                 t0d, [wq+hq]
    movd                xm4, t0d
    tzcnt               t0d, t0d
    movd                xm5, t0d
    lea                  t0, [ipred_cfl_avx2_table]
    tzcnt                wd, wd
    movsxd               r6, [t0+r6*4]
    movsxd               wq, [t0+wq*4+4*4]
    pcmpeqd              m3, m3
    psrlw               xm4, 1
    add                  r6, t0
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6
.h4:
    movd                xm0, [tlq-4]
    pmaddubsw           xm0, xm3
    jmp                  wq
.w4:
    movd                xm1, [tlq+1]
    pmaddubsw           xm1, xm3
    psubw               xm0, xm4
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    cmp                  hd, 4
    jg .w4_mul
    psrlw               xm0, 3
    jmp .w4_end
.w4_mul:
    punpckhqdq          xm1, xm0, xm0
    lea                 r2d, [hq*2]
    mov                 r6d, 0x55563334
    paddw               xm0, xm1
    shrx                r6d, r6d, r2d
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    movd                xm1, r6d
    psrlw               xm0, 2
    pmulhuw             xm0, xm1
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
    packuswb             m4, m4
    vextracti128        xm5, m4, 1
    movd   [dstq+strideq*0], xm4
    pextrd [dstq+strideq*1], xm4, 1
    movd   [dstq+strideq*2], xm5
    pextrd [dstq+r6       ], xm5, 1
    lea                dstq, [dstq+strideq*4]
    add                 acq, 32
    sub                  hd, 4
    jg .s4_loop
    RET
ALIGN function_align
.h8:
    movq                xm0, [tlq-8]
    pmaddubsw           xm0, xm3
    jmp                  wq
.w8:
    movq                xm1, [tlq+1]
    vextracti128        xm2, m0, 1
    pmaddubsw           xm1, xm3
    psubw               xm0, xm4
    paddw               xm0, xm2
    punpckhqdq          xm2, xm0, xm0
    paddw               xm0, xm2
    paddw               xm0, xm1
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    psrlw               xm0, xm5
    cmp                  hd, 8
    je .w8_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    cmp                  hd, 32
    cmove               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
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
    packuswb             m4, m5
    vextracti128        xm5, m4, 1
    movq   [dstq+strideq*0], xm4
    movq   [dstq+strideq*1], xm5
    movhps [dstq+strideq*2], xm4
    movhps [dstq+r6       ], xm5
    lea                dstq, [dstq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .s8_loop
    RET
ALIGN function_align
.h16:
    mova                xm0, [tlq-16]
    pmaddubsw           xm0, xm3
    jmp                  wq
.w16:
    movu                xm1, [tlq+1]
    vextracti128        xm2, m0, 1
    pmaddubsw           xm1, xm3
    psubw               xm0, xm4
    paddw               xm0, xm2
    paddw               xm0, xm1
    punpckhqdq          xm1, xm0, xm0
    paddw               xm0, xm1
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    psrlw               xm0, xm5
    cmp                  hd, 16
    je .w16_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    test                 hb, 8|32
    cmovz               r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
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
    packuswb             m4, m5
    vpermq               m4, m4, q3120
    mova         [dstq+strideq*0], xm4
    vextracti128 [dstq+strideq*1], m4, 1
    lea                dstq, [dstq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .s16_loop
    RET
ALIGN function_align
.h32:
    mova                 m0, [tlq-32]
    pmaddubsw            m0, m3
    jmp                  wq
.w32:
    movu                 m1, [tlq+1]
    pmaddubsw            m1, m3
    paddw                m0, m1
    vextracti128        xm1, m0, 1
    psubw               xm0, xm4
    paddw               xm0, xm1
    punpckhqdq          xm1, xm0, xm0
    paddw               xm0, xm1
    psrlq               xm1, xm0, 32
    paddw               xm0, xm1
    pmaddwd             xm0, xm3
    psrlw               xm0, xm5
    cmp                  hd, 32
    je .w32_end
    lea                 r2d, [hq*2]
    mov                 r6d, 0x33345556
    shrx                r6d, r6d, r2d
    movd                xm1, r6d
    pmulhuw             xm0, xm1
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
    packuswb             m4, m5
    vpermq               m4, m4, q3120
    mova             [dstq], m4
    add                dstq, strideq
    add                 acq, 64
    dec                  hd
    jg .s32_loop
    RET

cglobal ipred_cfl_128_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    lea                  t0, [ipred_cfl_splat_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [t0+wq*4]
    vpbroadcastd         m0, [t0-ipred_cfl_splat_avx2_table+pw_128]
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  wq

cglobal ipred_cfl_ac_420_8bpc, 4, 9, 5, ac, y, stride, wpad, hpad, w, h, sz, ac_bak
    movifnidn         hpadd, hpadm
    movifnidn            wd, wm
    mov                  hd, hm
    mov                 szd, wd
    mov             ac_bakq, acq
    imul                szd, hd
    shl               hpadd, 2
    sub                  hd, hpadd
    vpbroadcastd         m2, [pb_2]
    pxor                 m4, m4
    cmp                  wd, 8
    jg .w16
    je .w8
    ; fall-through

    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h, sz, ac_bak
.w4:
    lea            stride3q, [strideq*3]
.w4_loop:
    movq                xm0, [yq]
    movq                xm1, [yq+strideq]
    movhps              xm0, [yq+strideq*2]
    movhps              xm1, [yq+stride3q]
    pmaddubsw           xm0, xm2
    pmaddubsw           xm1, xm2
    paddw               xm0, xm1
    mova              [acq], xm0
    paddw               xm4, xm0
    lea                  yq, [yq+strideq*4]
    add                 acq, 16
    sub                  hd, 2
    jg .w4_loop
    test              hpadd, hpadd
    jz .calc_avg
    vpermq               m0, m0, q1111
.w4_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 32
    sub               hpadd, 4
    jg .w4_hpad_loop
    jmp .calc_avg

.w8:
    lea            stride3q, [strideq*3]
    test              wpadd, wpadd
    jnz .w8_wpad
.w8_loop:
    mova                xm0, [yq]
    mova                xm1, [yq+strideq]
    vinserti128          m0, [yq+strideq*2], 1
    vinserti128          m1, [yq+stride3q], 1
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    mova              [acq], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*4]
    add                 acq, 32
    sub                  hd, 2
    jg .w8_loop
    test              hpadd, hpadd
    jz .calc_avg
    jmp .w8_hpad
.w8_wpad:
    vbroadcasti128       m3, [cfl_ac_w8_pad1_shuffle]
.w8_wpad_loop:
    movq                xm0, [yq]
    movq                xm1, [yq+strideq]
    vinserti128          m0, [yq+strideq*2], 1
    vinserti128          m1, [yq+stride3q], 1
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    pshufb               m0, m3
    mova              [acq], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*4]
    add                 acq, 32
    sub                  hd, 2
    jg .w8_wpad_loop
    test              hpadd, hpadd
    jz .calc_avg
.w8_hpad:
    vpermq               m0, m0, q3232
.w8_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 32
    sub               hpadd, 2
    jg .w8_hpad_loop
    jmp .calc_avg

.w16:
    test              wpadd, wpadd
    jnz .w16_wpad
.w16_loop:
    mova                 m0, [yq]
    mova                 m1, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    mova              [acq], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    dec                  hd
    jg .w16_loop
    test              hpadd, hpadd
    jz .calc_avg
    jmp .w16_hpad_loop
.w16_wpad:
    DEFINE_ARGS ac, y, stride, wpad, hpad, iptr, h, sz, ac_bak
    lea               iptrq, [ipred_cfl_ac_420_avx2_table]
    shl               wpadd, 2
    mova                 m3, [iptrq+cfl_ac_w16_pad_shuffle- \
                              ipred_cfl_ac_420_avx2_table+wpadq*8-32]
    movsxd            wpadq, [iptrq+wpadq+4]
    add               iptrq, wpadq
    jmp iptrq
.w16_pad3:
    vpbroadcastq         m0, [yq]
    vpbroadcastq         m1, [yq+strideq]
    jmp .w16_wpad_end
.w16_pad2:
    vbroadcasti128       m0, [yq]
    vbroadcasti128       m1, [yq+strideq]
    jmp .w16_wpad_end
.w16_pad1:
    mova                 m0, [yq]
    mova                 m1, [yq+strideq]
    ; fall-through
.w16_wpad_end:
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    pshufb               m0, m3
    mova              [acq], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    dec                  hd
    jz .w16_wpad_done
    jmp iptrq
.w16_wpad_done:
    test              hpadd, hpadd
    jz .calc_avg
.w16_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 32
    dec               hpadd
    jg .w16_hpad_loop
    ; fall-through

.calc_avg:
    vpbroadcastd         m2, [pw_1]
    pmaddwd              m0, m4, m2
    vextracti128        xm1, m0, 1
    tzcnt               r1d, szd
    paddd               xm0, xm1
    movd                xm2, r1d
    movd                xm3, szd
    punpckhqdq          xm1, xm0, xm0
    paddd               xm0, xm1
    psrad               xm3, 1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm3
    paddd               xm0, xm1
    psrad               xm0, xm2
    vpbroadcastw         m0, xm0
.sub_loop:
    mova                 m1, [ac_bakq]
    psubw                m1, m0
    mova          [ac_bakq], m1
    add             ac_bakq, 32
    sub                 szd, 16
    jg .sub_loop
    RET

cglobal ipred_cfl_ac_422_8bpc, 4, 9, 6, ac, y, stride, wpad, hpad, w, h, sz, ac_bak
    movifnidn         hpadd, hpadm
    movifnidn            wd, wm
    mov                  hd, hm
    mov                 szd, wd
    mov             ac_bakq, acq
    imul                szd, hd
    shl               hpadd, 2
    sub                  hd, hpadd
    vpbroadcastd         m2, [pb_4]
    pxor                 m4, m4
    pxor                 m5, m5
    cmp                  wd, 8
    jg .w16
    je .w8
    ; fall-through

    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h, sz, ac_bak
.w4:
    lea            stride3q, [strideq*3]
.w4_loop:
    movq                xm1, [yq]
    movhps              xm1, [yq+strideq]
    movq                xm0, [yq+strideq*2]
    movhps              xm0, [yq+stride3q]
    pmaddubsw           xm0, xm2
    pmaddubsw           xm1, xm2
    mova              [acq], xm1
    mova           [acq+16], xm0
    paddw               xm4, xm0
    paddw               xm5, xm1
    lea                  yq, [yq+strideq*4]
    add                 acq, 32
    sub                  hd, 4
    jg .w4_loop
    test              hpadd, hpadd
    jz .calc_avg
    vpermq               m0, m0, q1111
.w4_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 32
    sub               hpadd, 4
    jg .w4_hpad_loop
    jmp .calc_avg

.w8:
    lea            stride3q, [strideq*3]
    test              wpadd, wpadd
    jnz .w8_wpad
.w8_loop:
    mova                xm1, [yq]
    vinserti128          m1, [yq+strideq], 1
    mova                xm0, [yq+strideq*2]
    vinserti128          m0, [yq+stride3q], 1
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m4, m0
    paddw                m5, m1
    lea                  yq, [yq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .w8_loop
    test              hpadd, hpadd
    jz .calc_avg
    jmp .w8_hpad
.w8_wpad:
    vbroadcasti128       m3, [cfl_ac_w8_pad1_shuffle]
.w8_wpad_loop:
    movq                xm1, [yq]
    vinserti128          m1, [yq+strideq], 1
    movq                xm0, [yq+strideq*2]
    vinserti128          m0, [yq+stride3q], 1
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pshufb               m0, m3
    pshufb               m1, m3
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m4, m0
    paddw                m5, m1
    lea                  yq, [yq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .w8_wpad_loop
    test              hpadd, hpadd
    jz .calc_avg
.w8_hpad:
    vpermq               m0, m0, q3232
.w8_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 32
    sub               hpadd, 2
    jg .w8_hpad_loop
    jmp .calc_avg

.w16:
    test              wpadd, wpadd
    jnz .w16_wpad
.w16_loop:
    mova                 m1, [yq]
    mova                 m0, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m4, m0
    paddw                m5, m1
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_loop
    test              hpadd, hpadd
    jz .calc_avg
    jmp .w16_hpad_loop
.w16_wpad:
    DEFINE_ARGS ac, y, stride, wpad, hpad, iptr, h, sz, ac_bak
    lea               iptrq, [ipred_cfl_ac_422_avx2_table]
    shl               wpadd, 2
    mova                 m3, [iptrq+cfl_ac_w16_pad_shuffle- \
                              ipred_cfl_ac_422_avx2_table+wpadq*8-32]
    movsxd            wpadq, [iptrq+wpadq+4]
    add               iptrq, wpadq
    jmp iptrq
.w16_pad3:
    vpbroadcastq         m1, [yq]
    vpbroadcastq         m0, [yq+strideq]
    jmp .w16_wpad_end
.w16_pad2:
    vbroadcasti128       m1, [yq]
    vbroadcasti128       m0, [yq+strideq]
    jmp .w16_wpad_end
.w16_pad1:
    mova                 m1, [yq]
    mova                 m0, [yq+strideq]
    ; fall-through
.w16_wpad_end:
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pshufb               m0, m3
    pshufb               m1, m3
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m4, m0
    paddw                m5, m1
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jz .w16_wpad_done
    jmp iptrq
.w16_wpad_done:
    test              hpadd, hpadd
    jz .calc_avg
.w16_hpad_loop:
    mova              [acq], m0
    mova           [acq+32], m0
    paddw                m4, m0
    paddw                m5, m0
    add                 acq, 64
    sub               hpadd, 2
    jg .w16_hpad_loop
    ; fall-through

.calc_avg:
    vpbroadcastd         m2, [pw_1]
    pmaddwd              m5, m5, m2
    pmaddwd              m0, m4, m2
    paddd                m0, m5
    vextracti128        xm1, m0, 1
    tzcnt               r1d, szd
    paddd               xm0, xm1
    movd                xm2, r1d
    movd                xm3, szd
    punpckhqdq          xm1, xm0, xm0
    paddd               xm0, xm1
    psrad               xm3, 1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm3
    paddd               xm0, xm1
    psrad               xm0, xm2
    vpbroadcastw         m0, xm0
.sub_loop:
    mova                 m1, [ac_bakq]
    psubw                m1, m0
    mova          [ac_bakq], m1
    add             ac_bakq, 32
    sub                 szd, 16
    jg .sub_loop
    RET

cglobal ipred_cfl_ac_444_8bpc, 4, 9, 6, ac, y, stride, wpad, hpad, w, h, sz, ac_bak
    movifnidn         hpadd, hpadm
    movifnidn            wd, wm
    mov                  hd, hm
    mov                 szd, wd
    imul                szd, hd
    shl               hpadd, 2
    sub                  hd, hpadd
    pxor                 m4, m4
    vpbroadcastd         m5, [pw_1]
    tzcnt               r8d, wd
    lea                  r5, [ipred_cfl_ac_444_avx2_table]
    movsxd               r8, [r5+r8*4+12]
    add                  r5, r8

    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h, sz, ac_bak
    mov             ac_bakq, acq
    jmp                  r5

.w4:
    lea            stride3q, [strideq*3]
    pxor                xm2, xm2
.w4_loop:
    movd                xm1, [yq]
    movd                xm0, [yq+strideq*2]
    pinsrd              xm1, [yq+strideq], 1
    pinsrd              xm0, [yq+stride3q], 1
    punpcklbw           xm1, xm2
    punpcklbw           xm0, xm2
    psllw               xm1, 3
    psllw               xm0, 3
    mova              [acq], xm1
    mova           [acq+16], xm0
    paddw               xm1, xm0
    paddw               xm4, xm1
    lea                  yq, [yq+strideq*4]
    add                 acq, 32
    sub                  hd, 4
    jg .w4_loop
    test              hpadd, hpadd
    jz .calc_avg_mul
    pshufd              xm0, xm0, q3232
    paddw               xm1, xm0, xm0
.w4_hpad_loop:
    mova              [acq], xm0
    mova           [acq+16], xm0
    paddw               xm4, xm1
    add                 acq, 32
    sub               hpadd, 4
    jg .w4_hpad_loop
    jmp .calc_avg_mul

.w8:
    lea            stride3q, [strideq*3]
    pxor                 m2, m2
.w8_loop:
    movq                xm1, [yq]
    movq                xm0, [yq+strideq*2]
    vinserti128          m1, [yq+strideq], 1
    vinserti128          m0, [yq+stride3q], 1
    punpcklbw            m1, m2
    punpcklbw            m0, m2
    psllw                m1, 3
    psllw                m0, 3
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m1, m0
    paddw                m4, m1
    lea                  yq, [yq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .w8_loop
    test              hpadd, hpadd
    jz .calc_avg_mul
    vpermq               m0, m0, q3232
    paddw                m1, m0, m0
.w8_hpad_loop:
    mova              [acq], m0
    mova           [acq+32], m0
    paddw                m4, m1
    add                 acq, 64
    sub               hpadd, 4
    jg .w8_hpad_loop
    jmp .calc_avg_mul

.w16:
    test              wpadd, wpadd
    jnz .w16_wpad
.w16_loop:
    pmovzxbw             m1, [yq]
    pmovzxbw             m0, [yq+strideq]
    psllw                m1, 3
    psllw                m0, 3
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m1, m0
    pmaddwd              m1, m5
    paddd                m4, m1
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_loop
    test              hpadd, hpadd
    jz .calc_avg
    jmp .w16_hpad
.w16_wpad:
    mova                 m3, [cfl_ac_444_w16_pad1_shuffle]
.w16_wpad_loop:
    vpbroadcastq         m1, [yq]
    vpbroadcastq         m0, [yq+strideq]
    pshufb               m1, m3
    pshufb               m0, m3
    psllw                m1, 3
    psllw                m0, 3
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m1, m0
    pmaddwd              m1, m5
    paddd                m4, m1
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_wpad_loop
    test              hpadd, hpadd
    jz .calc_avg
.w16_hpad:
    paddw                m1, m0, m0
    pmaddwd              m1, m5
.w16_hpad_loop:
    mova              [acq], m0
    mova           [acq+32], m0
    paddd                m4, m1
    add                 acq, 64
    sub               hpadd, 2
    jg .w16_hpad_loop
    jmp .calc_avg

.w32:
    test              wpadd, wpadd
    jnz .w32_wpad
.w32_loop:
    pmovzxbw             m1, [yq]
    pmovzxbw             m0, [yq+16]
    psllw                m1, 3
    psllw                m0, 3
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m2, m1, m0
    pmaddwd              m2, m5
    paddd                m4, m2
    add                  yq, strideq
    add                 acq, 64
    dec                  hd
    jg .w32_loop
    test              hpadd, hpadd
    jz .calc_avg
    jmp .w32_hpad_loop
.w32_wpad:
    DEFINE_ARGS ac, y, stride, wpad, hpad, iptr, h, sz, ac_bak
    lea               iptrq, [ipred_cfl_ac_444_avx2_table]
    add               wpadd, wpadd
    mova                 m3, [iptrq+cfl_ac_444_w16_pad1_shuffle-ipred_cfl_ac_444_avx2_table]
    movsxd            wpadq, [iptrq+wpadq+4]
    add               iptrq, wpadq
    jmp iptrq
.w32_pad3:
    vpbroadcastq         m1, [yq]
    pshufb               m1, m3
    vpermq               m0, m1, q3232
    jmp .w32_wpad_end
.w32_pad2:
    pmovzxbw             m1, [yq]
    pshufhw              m0, m1, q3333
    vpermq               m0, m0, q3333
    jmp .w32_wpad_end
.w32_pad1:
    pmovzxbw             m1, [yq]
    vpbroadcastq         m0, [yq+16]
    pshufb               m0, m3
    ; fall-through
.w32_wpad_end:
    psllw                m1, 3
    psllw                m0, 3
    mova              [acq], m1
    mova           [acq+32], m0
    paddw                m2, m1, m0
    pmaddwd              m2, m5
    paddd                m4, m2
    add                  yq, strideq
    add                 acq, 64
    dec                  hd
    jz .w32_wpad_done
    jmp iptrq
.w32_wpad_done:
    test              hpadd, hpadd
    jz .calc_avg
.w32_hpad_loop:
    mova              [acq], m1
    mova           [acq+32], m0
    paddd                m4, m2
    add                 acq, 64
    dec               hpadd
    jg .w32_hpad_loop
    jmp .calc_avg

.calc_avg_mul:
    pmaddwd              m4, m5
.calc_avg:
    vextracti128        xm1, m4, 1
    tzcnt               r1d, szd
    paddd               xm0, xm4, xm1
    movd                xm2, r1d
    movd                xm3, szd
    punpckhqdq          xm1, xm0, xm0
    paddd               xm0, xm1
    psrad               xm3, 1
    psrlq               xm1, xm0, 32
    paddd               xm0, xm3
    paddd               xm0, xm1
    psrad               xm0, xm2
    vpbroadcastw         m0, xm0
.sub_loop:
    mova                 m1, [ac_bakq]
    psubw                m1, m0
    mova          [ac_bakq], m1
    add             ac_bakq, 32
    sub                 szd, 16
    jg .sub_loop
    RET

cglobal pal_pred_8bpc, 4, 6, 5, dst, stride, pal, idx, w, h
    vbroadcasti128       m4, [palq]
    lea                  r2, [pal_pred_avx2_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r2+wq*4]
    packuswb             m4, m4
    add                  wq, r2
    lea                  r2, [strideq*3]
    jmp                  wq
.w4:
    pshufb              xm0, xm4, [idxq]
    add                idxq, 16
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+r2       ], xm0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4
    RET
ALIGN function_align
.w8:
    pshufb              xm0, xm4, [idxq+16*0]
    pshufb              xm1, xm4, [idxq+16*1]
    add                idxq, 16*2
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+r2       ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8
    RET
ALIGN function_align
.w16:
    pshufb               m0, m4, [idxq+32*0]
    pshufb               m1, m4, [idxq+32*1]
    add                idxq, 32*2
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    mova         [dstq+strideq*2], xm1
    vextracti128 [dstq+r2       ], m1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16
    RET
ALIGN function_align
.w32:
    pshufb               m0, m4, [idxq+32*0]
    pshufb               m1, m4, [idxq+32*1]
    pshufb               m2, m4, [idxq+32*2]
    pshufb               m3, m4, [idxq+32*3]
    add                idxq, 32*4
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+r2       ], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w32
    RET
ALIGN function_align
.w64:
    pshufb               m0, m4, [idxq+32*0]
    pshufb               m1, m4, [idxq+32*1]
    pshufb               m2, m4, [idxq+32*2]
    pshufb               m3, m4, [idxq+32*3]
    add                idxq, 32*4
    mova [dstq+strideq*0+32*0], m0
    mova [dstq+strideq*0+32*1], m1
    mova [dstq+strideq*1+32*0], m2
    mova [dstq+strideq*1+32*1], m3
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w64
    RET

%endif
