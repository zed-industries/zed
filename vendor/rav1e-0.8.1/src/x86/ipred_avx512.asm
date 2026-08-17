; Copyright © 2020, VideoLAN and dav1d authors
; Copyright © 2020, Two Orioles, LLC
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

; dav1d_filter_intra_taps[], reordered for VNNI: p1 p2 p3 p4, p6 p5 p0 __
filter_taps:  db 10,  0,  0,  0,  2, 10,  0,  0,  1,  1, 10,  0,  1,  1,  2, 10
              db  6,  0,  0,  0,  2,  6,  0,  0,  2,  2,  6,  0,  1,  2,  2,  6
              db  0, 12, -6,  0,  0,  9, -5,  0,  0,  7, -3,  0,  0,  5, -3,  0
              db 12,  2, -4,  0,  9,  2, -3,  0,  7,  2, -3,  0,  5,  3, -3,  0
              db 16,  0,  0,  0,  0, 16,  0,  0,  0,  0, 16,  0,  0,  0,  0, 16
              db 16,  0,  0,  0,  0, 16,  0,  0,  0,  0, 16,  0,  0,  0,  0, 16
              db  0, 10,-10,  0,  0,  6, -6,  0,  0,  4, -4,  0,  0,  2, -2,  0
              db 10,  0,-10,  0,  6,  0, -6,  0,  4,  0, -4,  0,  2,  0, -2,  0
              db  8,  0,  0,  0,  0,  8,  0,  0,  0,  0,  8,  0,  0,  0,  0,  8
              db  4,  0,  0,  0,  0,  4,  0,  0,  0,  0,  4,  0,  0,  0,  0,  4
              db  0, 16, -8,  0,  0, 16, -8,  0,  0, 16, -8,  0,  0, 16, -8,  0
              db 16,  0, -4,  0, 16,  0, -4,  0, 16,  0, -4,  0, 16,  0, -4,  0
              db  8,  0,  0,  0,  3,  8,  0,  0,  2,  3,  8,  0,  1,  2,  3,  8
              db  4,  0,  0,  0,  3,  4,  0,  0,  2,  3,  4,  0,  2,  2,  3,  4
              db  0, 10, -2,  0,  0,  6, -1,  0,  0,  4, -1,  0,  0,  2,  0,  0
              db 10,  3, -1,  0,  6,  4, -1,  0,  4,  4, -1,  0,  3,  3, -1,  0
              db 14,  0,  0,  0,  0, 14,  0,  0,  0,  0, 14,  0,  0,  0,  0, 14
              db 12,  0,  0,  0,  1, 12,  0,  0,  0,  0, 12,  0,  0,  0,  1, 12
              db  0, 14,-12,  0,  0, 12,-10,  0,  0, 11, -9,  0,  0, 10, -8,  0
              db 14,  0,-10,  0, 12,  0, -9,  0, 11,  1, -8,  0,  9,  1, -7,  0
filter_perm:  db  0,  1,  2,  3, 24, 25, 26, 27,  4,  5,  6,  7, 28, 29, 30, 31
              db 15, 11,  7,  3, 15, 11,  7,  3, 15, 11,  7,  3, 15, 11,  7,131
              db 31, 27, 23, 19, 31, 27, 23, 19, 31, 27, 23, 19, 31, 27, 23,147
              db 47, 43, 39, 35, 47, 43, 39, 35, 47, 43, 39, 35, 47, 43, 39,163
filter_end:   dd  2,  3, 16, 17, -1, -1, 20, 21,  0,  6, 24, 30,  1,  7, 25, 31
smooth_shuf:  db  7,  7,  7,  7,  0,  1,  0,  1,  3,  3,  3,  3,  8,  9,  8,  9
              db  5,  5,  5,  5,  4,  5,  4,  5,  1,  1,  1,  1, 12, 13, 12, 13
              db  6,  6,  6,  6,  2,  3,  2,  3,  2,  2,  2,  2, 10, 11, 10, 11
              db  4,  4,  4,  4,  6,  7,  6,  7,  0,  0,  0,  0, 14, 15, 14, 15
smooth_endA:  db  1,  3,  5,  7,  9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31
              db 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63
              db 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95
              db 97, 99,101,103,105,107,109,111,113,115,117,119,121,123,125,127
smooth_endB:  db  1,  3,  5,  7,  9, 11, 13, 15, 65, 67, 69, 71, 73, 75, 77, 79
              db 17, 19, 21, 23, 25, 27, 29, 31, 81, 83, 85, 87, 89, 91, 93, 95
              db 33, 35, 37, 39, 41, 43, 45, 47, 97, 99,101,103,105,107,109,111
              db 49, 51, 53, 55, 57, 59, 61, 63,113,115,117,119,121,123,125,127
ipred_h_shuf: db  7,  7,  7,  7,  6,  6,  6,  6,  5,  5,  5,  5,  4,  4,  4,  4
              db  3,  3,  3,  3,  2,  2,  2,  2,  1,  1,  1,  1,  0,  0,  0,  0

pb_127_m127:  times 2 db 127, -127
pb_128:       times 4 db 128
pw_128:       times 2 dw 128
pw_255:       times 2 dw 255

%define pb_1 (ipred_h_shuf+24)
%define pb_2 (ipred_h_shuf+20)
%define pb_3 (ipred_h_shuf+16)
%define pd_8 (filter_taps+128)

%macro JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - 2*4)
    %xdefine %%base mangle(private_prefix %+ _%1_%2)
    %%table:
    %rep %0 - 2
        dd %%base %+ .%3 - (%%table - 2*4)
        %rotate 1
    %endrep
%endmacro

%define ipred_dc_splat_8bpc_avx512icl_table (ipred_dc_8bpc_avx512icl_table + 10*4)

JMP_TABLE ipred_h_8bpc,          avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_paeth_8bpc,      avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_8bpc,     avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_v_8bpc,   avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_h_8bpc,   avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_dc_8bpc,         avx512icl, h4, h8, h16, h32, h64, w4, w8, w16, w32, w64, \
                                       s4-10*4, s8-10*4, s16-10*4, s32-10*4, s64-10*4
JMP_TABLE ipred_dc_left_8bpc,    avx512icl, h4, h8, h16, h32, h64
JMP_TABLE pal_pred_8bpc,         avx512icl, w4, w8, w16, w32, w64

SECTION .text

INIT_ZMM avx512icl
cglobal ipred_dc_top_8bpc, 3, 7, 5, dst, stride, tl, w, h
    lea                  r5, [ipred_dc_left_8bpc_avx512icl_table]
    movd                xm0, wm
    tzcnt                wd, wm
    inc                 tlq
    movifnidn            hd, hm
    movu                ym1, [tlq]
    movd               xmm3, wd
    movsxd               r6, [r5+wq*4]
    vpbroadcastd        ym2, [r5-ipred_dc_left_8bpc_avx512icl_table+pb_1]
    psrld               xm0, 1
    vpdpbusd            ym0, ym1, ym2
    add                  r6, r5
    add                  r5, ipred_dc_splat_8bpc_avx512icl_table-ipred_dc_left_8bpc_avx512icl_table
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6

cglobal ipred_dc_left_8bpc, 3, 7, 5, dst, stride, tl, w, h, stride3
    lea                  r5, [ipred_dc_left_8bpc_avx512icl_table]
    mov                  hd, hm
    tzcnt               r6d, hd
    sub                 tlq, hq
    tzcnt                wd, wm
    movd                xm0, hm
    movu                ym1, [tlq]
    movd               xmm3, r6d
    movsxd               r6, [r5+r6*4]
    vpbroadcastd        ym2, [r5-ipred_dc_left_8bpc_avx512icl_table+pb_1]
    psrld               xm0, 1
    vpdpbusd            ym0, ym1, ym2
    add                  r6, r5
    add                  r5, ipred_dc_splat_8bpc_avx512icl_table-ipred_dc_left_8bpc_avx512icl_table
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6
.h64:
    movu                ym1, [tlq+32] ; unaligned when jumping here from dc_top
    vpdpbusd            ym0, ym1, ym2
.h32:
    vextracti32x4       xm1, ym0, 1
    paddd               xm0, xm1
.h16:
    punpckhqdq          xm1, xm0, xm0
    paddd               xm0, xm1
.h8:
    psrlq               xm1, xm0, 32
    paddd               xm0, xm1
.h4:
    vpsrlvd             xm0, xmm3
    lea            stride3q, [strideq*3]
    vpbroadcastb         m0, xm0
    jmp                  wq

cglobal ipred_dc_8bpc, 3, 7, 5, dst, stride, tl, w, h, stride3
    movifnidn            hd, hm
    movifnidn            wd, wm
    tzcnt               r6d, hd
    lea                 r5d, [wq+hq]
    movd                xm0, r5d
    tzcnt               r5d, r5d
    movd               xmm4, r5d
    lea                  r5, [ipred_dc_8bpc_avx512icl_table]
    tzcnt                wd, wd
    movsxd               r6, [r5+r6*4]
    movsxd               wq, [r5+wq*4+5*4]
    vpbroadcastd        ym3, [r5-ipred_dc_8bpc_avx512icl_table+pb_1]
    psrld               xm0, 1
    add                  r6, r5
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  r6
.h4:
    movd               xmm1, [tlq-4]
    vpdpbusd            xm0, xmm1, xm3
    jmp                  wq
.w4:
    movd               xmm1, [tlq+1]
    vpdpbusd            xm0, xmm1, xm3
    cmp                  hd, 4
    jg .w4_mul
    psrlw              xmm0, xm0, 3
    jmp .w4_end
.w4_mul:
    punpckhqdq         xmm1, xm0, xm0
    lea                 r2d, [hq*2]
    mov                 r6d, 0x55563334
    paddd              xmm1, xm0
    shrx                r6d, r6d, r2d
    psrlq              xmm0, xmm1, 32
    paddd              xmm0, xmm1
    movd               xmm1, r6d
    psrld              xmm0, 2
    pmulhuw            xmm0, xmm1
.w4_end:
    vpbroadcastb        xm0, xmm0
.s4:
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm0
    movd   [dstq+strideq*2], xm0
    movd   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s4
    RET
.h8:
    movq               xmm1, [tlq-8]
    vpdpbusd            xm0, xmm1, xm3
    jmp                  wq
.w8:
    movq               xmm1, [tlq+1]
    vextracti32x4       xm2, ym0, 1
    vpdpbusd            xm0, xmm1, xm3
    paddd              xmm2, xm2, xm0
    punpckhqdq         xmm0, xmm2, xmm2
    paddd              xmm0, xmm2
    psrlq              xmm1, xmm0, 32
    paddd              xmm0, xmm1
    vpsrlvd            xmm0, xmm4
    cmp                  hd, 8
    je .w8_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    cmp                  hd, 32
    cmove               r6d, r2d
    movd               xmm1, r6d
    pmulhuw            xmm0, xmm1
.w8_end:
    vpbroadcastb        xm0, xmm0
.s8:
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm0
    movq   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s8
    RET
.h16:
    mova               xmm1, [tlq-16]
    vpdpbusd            xm0, xmm1, xm3
    jmp                  wq
.w16:
    movu               xmm1, [tlq+1]
    vextracti32x4       xm2, ym0, 1
    vpdpbusd            xm0, xmm1, xm3
    paddd              xmm2, xm2, xm0
    punpckhqdq         xmm0, xmm2, xmm2
    paddd              xmm0, xmm2
    psrlq              xmm1, xmm0, 32
    paddd              xmm0, xmm1
    vpsrlvd            xmm0, xmm4
    cmp                  hd, 16
    je .w16_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    test                 hb, 8|32
    cmovz               r6d, r2d
    movd               xmm1, r6d
    pmulhuw            xmm0, xmm1
.w16_end:
    vpbroadcastb        xm0, xmm0
.s16:
    mova   [dstq+strideq*0], xm0
    mova   [dstq+strideq*1], xm0
    mova   [dstq+strideq*2], xm0
    mova   [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s16
    RET
.h32:
    mova                ym1, [tlq-32]
    vpdpbusd            ym0, ym1, ym3
    jmp                  wq
.w32:
    movu                ym1, [tlq+1]
    vpdpbusd            ym0, ym1, ym3
    vextracti32x4       xm1, ym0, 1
    paddd              xmm1, xm1, xm0
    punpckhqdq         xmm0, xmm1, xmm1
    paddd              xmm0, xmm1
    psrlq              xmm1, xmm0, 32
    paddd              xmm0, xmm1
    vpsrlvd            xmm0, xmm4
    cmp                  hd, 32
    je .w32_end
    lea                 r2d, [hq*2]
    mov                 r6d, 0x33345556
    shrx                r6d, r6d, r2d
    movd               xmm1, r6d
    pmulhuw            xmm0, xmm1
.w32_end:
    vpbroadcastb        ym0, xmm0
.s32:
    mova   [dstq+strideq*0], ym0
    mova   [dstq+strideq*1], ym0
    mova   [dstq+strideq*2], ym0
    mova   [dstq+stride3q ], ym0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s32
    RET
.h64:
    mova                ym1, [tlq-64]
    mova                ym2, [tlq-32]
    vpdpbusd            ym0, ym1, ym3
    vpdpbusd            ym0, ym2, ym3
    jmp                  wq
.w64:
    movu                ym1, [tlq+ 1]
    movu                ym2, [tlq+33]
    vpdpbusd            ym0, ym1, ym3
    vpdpbusd            ym0, ym2, ym3
    vextracti32x4       xm1, ym0, 1
    paddd              xmm1, xm1, xm0
    punpckhqdq         xmm0, xmm1, xmm1
    paddd              xmm0, xmm1
    psrlq              xmm1, xmm0, 32
    paddd              xmm0, xmm1
    vpsrlvd            xmm0, xmm4
    cmp                  hd, 64
    je .w64_end
    mov                 r6d, 0x33345556
    shrx                r6d, r6d, hd
    movd               xmm1, r6d
    pmulhuw            xmm0, xmm1
.w64_end:
    vpbroadcastb         m0, xmm0
.s64:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m0
    mova   [dstq+strideq*2], m0
    mova   [dstq+stride3q ], m0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .s64
    RET

cglobal ipred_dc_128_8bpc, 2, 7, 5, dst, stride, tl, w, h, stride3
    lea                  r5, [ipred_dc_splat_8bpc_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    vpbroadcastd         m0, [r5-ipred_dc_splat_8bpc_avx512icl_table+pb_128]
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

cglobal ipred_v_8bpc, 3, 7, 5, dst, stride, tl, w, h, stride3
    lea                  r5, [ipred_dc_splat_8bpc_avx512icl_table]
    tzcnt                wd, wm
    movu                 m0, [tlq+1]
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

cglobal ipred_h_8bpc, 3, 7, 8, dst, stride, tl, w, h, stride3
%define base r6-ipred_h_8bpc_avx512icl_table
    lea                  r6, [ipred_h_8bpc_avx512icl_table]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    lea            stride3q, [strideq*3]
    sub                 tlq, hq
    add                  wq, r6
    jmp                  wq
.w4:
    mova               xmm1, [base+ipred_h_shuf+16]
.w4_loop:
    movd               xmm0, [tlq+hq-4]
    pshufb             xmm0, xmm1
    movd   [dstq+strideq*0], xmm0
    pextrd [dstq+strideq*1], xmm0, 1
    pextrd [dstq+strideq*2], xmm0, 2
    pextrd [dstq+stride3q ], xmm0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_loop
    RET
.w8:
    movsldup           xmm2, [base+ipred_h_shuf+16]
    movshdup           xmm3, [base+ipred_h_shuf+16]
.w8_loop:
    movd               xmm1, [tlq+hq-4]
    pshufb             xmm0, xmm1, xmm2
    pshufb             xmm1, xmm3
    movq   [dstq+strideq*0], xmm0
    movq   [dstq+strideq*1], xmm1
    movhps [dstq+strideq*2], xmm0
    movhps [dstq+stride3q ], xmm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_loop
    RET
.w16:
    movsldup             m1, [base+smooth_shuf]
.w16_loop:
    vpbroadcastd         m0, [tlq+hq-4]
    pshufb               m0, m1
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16
    RET
.w32:
    vpbroadcastd        ym3, [base+pb_1]
    vpord                m2, m3, [base+pb_2] {1to16}
.w32_loop:
    vpbroadcastd         m1, [tlq+hq-4]
    pshufb               m0, m1, m2
    pshufb               m1, m3
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w32_loop
    RET
.w64:
    vpbroadcastd         m4, [base+pb_3]
    vpbroadcastd         m5, [base+pb_2]
    vpbroadcastd         m6, [base+pb_1]
    pxor                 m7, m7
.w64_loop:
    vpbroadcastd         m3, [tlq+hq-4]
    pshufb               m0, m3, m4
    pshufb               m1, m3, m5
    pshufb               m2, m3, m6
    pshufb               m3, m7
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w64_loop
    RET

%macro PAETH 0
    psubusb              m1, m5, m4
    psubusb              m0, m4, m5
    por                  m1, m0           ; tdiff
    pavgb                m2, m6, m4
    vpcmpub              k1, m1, m7, 1    ; tdiff < ldiff
    vpblendmb        m0{k1}, m4, m6
    vpternlogd           m4, m6, m8, 0x28 ; (m4 ^ m6) & m8
    psubusb              m3, m5, m2
    psubb                m2, m4
    psubusb              m2, m5
    por                  m2, m3
    pminub               m1, m7
    paddusb              m2, m2
    por                  m2, m4           ; min(tldiff, 255)
    vpcmpub              k1, m2, m1, 1    ; tldiff < ldiff && tldiff < tdiff
    vmovdqu8         m0{k1}, m5
%endmacro

cglobal ipred_paeth_8bpc, 3, 7, 10, dst, stride, tl, w, h, top, stride3
    lea                  r6, [ipred_paeth_8bpc_avx512icl_table]
    tzcnt                wd, wm
    vpbroadcastb         m5, [tlq] ; topleft
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m8, [r6-ipred_paeth_8bpc_avx512icl_table+pb_1]
    lea                topq, [tlq+1]
    sub                 tlq, hq
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
INIT_YMM avx512icl
.w4:
    vpbroadcastd         m6, [topq]
    mova                 m9, [ipred_h_shuf]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0 ; ldiff
.w4_loop:
    vpbroadcastq         m4, [tlq+hq-8]
    pshufb               m4, m9 ; left
    PAETH
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+stride3q ], xm0, 3
    sub                  hd, 8
    jl .w4_ret
    vextracti32x4       xm0, m0, 1
    lea                dstq, [dstq+strideq*4]
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+stride3q ], xm0, 3
    lea                dstq, [dstq+strideq*4]
    jg .w4_loop
.w4_ret:
    RET
INIT_ZMM avx512icl
.w8:
    vpbroadcastq         m6, [topq]
    movsldup             m9, [smooth_shuf]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w8_loop:
    vpbroadcastq         m4, [tlq+hq-8]
    pshufb               m4, m9
    PAETH
    vextracti32x4       xm1, m0, 2
    vextracti32x4       xm2, ym0, 1
    vextracti32x4       xm3, m0, 3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm2
    movq   [dstq+stride3q ], xm3
    sub                  hd, 8
    jl .w8_ret
    lea                dstq, [dstq+strideq*4]
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm3
    lea                dstq, [dstq+strideq*4]
    jg .w8_loop
.w8_ret:
    RET
.w16:
    vbroadcasti32x4      m6, [topq]
    movsldup             m9, [smooth_shuf]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w16_loop:
    vpbroadcastd         m4, [tlq+hq-4]
    pshufb               m4, m9
    PAETH
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16_loop
    RET
.w32:
    vbroadcasti32x8      m6, [topq]
    mova                ym9, ym8
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w32_loop:
    vpbroadcastd         m4, [tlq+hq-2]
    pshufb               m4, m9
    PAETH
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop
    RET
.w64:
    movu                 m6, [topq]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w64_loop:
    vpbroadcastb         m4, [tlq+hq-1]
    PAETH
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET

cglobal ipred_smooth_v_8bpc, 3, 7, 7, dst, stride, tl, w, h, weights, stride3
%define base r6-ipred_smooth_v_8bpc_avx512icl_table
    lea                  r6, [ipred_smooth_v_8bpc_avx512icl_table]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m0, [base+pb_127_m127]
    vpbroadcastd         m1, [base+pw_128]
    lea            weightsq, [base+smooth_weights+hq*4]
    neg                  hq
    vpbroadcastb         m4, [tlq+hq] ; bottom
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    vpbroadcastd         m2, [tlq+1]
    movshdup             m5, [smooth_shuf]
    mova                ym6, [smooth_endA]
    punpcklbw            m2, m4 ; top, bottom
    pmaddubsw            m3, m2, m0
    paddw                m1, m2 ;   1 * top + 256 * bottom + 128, overflow is ok
    paddw                m3, m1 ; 128 * top + 129 * bottom + 128
.w4_loop:
    vbroadcasti32x4      m0, [weightsq+hq*2]
    pshufb               m0, m5
    pmaddubsw            m0, m2, m0
    paddw                m0, m3
    vpermb               m0, m6, m0
    vextracti32x4       xm1, ym0, 1
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+stride3q ], xm1, 2
    add                  hq, 8
    jg .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm0, 3
    pextrd [dstq+stride3q ], xm1, 3
    lea                dstq, [dstq+strideq*4]
    jl .w4_loop
.ret:
    RET
.w8:
    vpbroadcastq         m2, [tlq+1]
    movshdup             m5, [smooth_shuf]
    mova                ym6, [smooth_endA]
    punpcklbw            m2, m4
    pmaddubsw            m3, m2, m0
    paddw                m1, m2
    paddw                m3, m1
.w8_loop:
    vpbroadcastq         m0, [weightsq+hq*2]
    pshufb               m0, m5
    pmaddubsw            m0, m2, m0
    paddw                m0, m3
    vpermb               m0, m6, m0
    vextracti32x4       xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w8_loop
    RET
.w16:
    vbroadcasti32x4      m3, [tlq+1]
    movshdup             m6, [smooth_shuf]
    mova                 m7, [smooth_endB]
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1
.w16_loop:
    vpbroadcastq         m1, [weightsq+hq*2]
    pshufb               m1, m6
    pmaddubsw            m0, m2, m1
    pmaddubsw            m1, m3, m1
    paddw                m0, m4
    paddw                m1, m5
    vpermt2b             m0, m7, m1
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w16_loop
    RET
.w32:
    vbroadcasti32x8      m3, [tlq+1]
    movshdup             m6, [smooth_shuf]
    mova                 m7, [smooth_endB]
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1
.w32_loop:
    vpbroadcastd         m1, [weightsq+hq*2]
    pshufb               m1, m6
    pmaddubsw            m0, m2, m1
    pmaddubsw            m1, m3, m1
    paddw                m0, m4
    paddw                m1, m5
    vpermt2b             m0, m7, m1
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    add                  hq, 2
    jl .w32_loop
    RET
.w64:
    movu                 m3, [tlq+1]
    mova                 m6, [smooth_endB]
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1
.w64_loop:
    vpbroadcastw         m1, [weightsq+hq*2]
    pmaddubsw            m0, m2, m1
    pmaddubsw            m1, m3, m1
    paddw                m0, m4
    paddw                m1, m5
    vpermt2b             m0, m6, m1
    mova             [dstq], m0
    add                dstq, strideq
    inc                  hq
    jl .w64_loop
    RET

cglobal ipred_smooth_h_8bpc, 4, 7, 11, dst, stride, tl, w, h, stride3
%define base r5-ipred_smooth_h_8bpc_avx512icl_table
    lea                  r5, [ipred_smooth_h_8bpc_avx512icl_table]
    mov                 r6d, wd
    tzcnt                wd, wd
    vpbroadcastb         m4, [tlq+r6] ; right
    mov                  hd, hm
    movsxd               wq, [r5+wq*4]
    vpbroadcastd         m5, [base+pb_127_m127]
    vpbroadcastd         m6, [base+pw_128]
    sub                 tlq, hq
    add                  wq, r5
    vpmovb2m             k1, m6
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    movsldup             m3, [smooth_shuf]
    vpbroadcastq         m7, [smooth_weights+4*2]
    mova                ym8, [smooth_endA]
.w4_loop:
    vpbroadcastq         m0, [tlq+hq-8]
    mova                 m2, m4
    vpshufb          m2{k1}, m0, m3 ; left, right
    pmaddubsw            m0, m2, m5
    pmaddubsw            m1, m2, m7
    paddw                m2, m6
    paddw                m0, m2
    paddw                m0, m1
    vpermb               m0, m8, m0
    vextracti32x4       xm1, ym0, 1
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+stride3q ], xm1, 2
    sub                  hd, 8
    jl .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm0, 3
    pextrd [dstq+stride3q ], xm1, 3
    lea                dstq, [dstq+strideq*4]
    jg .w4_loop
.ret:
    RET
.w8:
    movsldup             m3, [smooth_shuf]
    vbroadcasti32x4      m7, [smooth_weights+8*2]
    mova                ym8, [smooth_endA]
.w8_loop:
    vpbroadcastd         m0, [tlq+hq-4]
    mova                 m2, m4
    vpshufb          m2{k1}, m0, m3
    pmaddubsw            m0, m2, m5
    pmaddubsw            m1, m2, m7
    paddw                m2, m6
    paddw                m0, m2
    paddw                m0, m1
    vpermb               m0, m8, m0
    vextracti32x4       xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_loop
    RET
.w16:
    movsldup             m7, [smooth_shuf]
    vbroadcasti32x4      m8, [smooth_weights+16*2]
    vbroadcasti32x4      m9, [smooth_weights+16*3]
    mova                m10, [smooth_endB]
.w16_loop:
    vpbroadcastd         m0, [tlq+hq-4]
    mova                 m3, m4
    vpshufb          m3{k1}, m0, m7
    pmaddubsw            m2, m3, m5
    pmaddubsw            m0, m3, m8
    pmaddubsw            m1, m3, m9
    paddw                m3, m6
    paddw                m2, m3
    paddw                m0, m2
    paddw                m1, m2
    vpermt2b             m0, m10, m1
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16_loop
    RET
.w32:
    mova                m10, [smooth_endA]
    vpbroadcastd        ym7, [pb_1]
    vbroadcasti32x8      m8, [smooth_weights+32*2]
    vbroadcasti32x8      m9, [smooth_weights+32*3]
    vshufi32x4          m10, m10, q3120
.w32_loop:
    vpbroadcastd         m0, [tlq+hq-2]
    mova                 m3, m4
    vpshufb          m3{k1}, m0, m7
    pmaddubsw            m2, m3, m5
    pmaddubsw            m0, m3, m8
    pmaddubsw            m1, m3, m9
    paddw                m3, m6
    paddw                m2, m3
    paddw                m0, m2
    paddw                m1, m2
    vpermt2b             m0, m10, m1
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop
    RET
.w64:
    mova                 m7, [smooth_weights+64*2]
    mova                 m8, [smooth_weights+64*3]
    mova                 m9, [smooth_endA]
.w64_loop:
    mova                 m3, m4
    vpbroadcastb     m3{k1}, [tlq+hq-1]
    pmaddubsw            m2, m3, m5
    pmaddubsw            m0, m3, m7
    pmaddubsw            m1, m3, m8
    paddw                m3, m6
    paddw                m2, m3
    paddw                m0, m2
    paddw                m1, m2
    vpermt2b             m0, m9, m1
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET

cglobal ipred_smooth_8bpc, 4, 7, 16, dst, stride, tl, w, h, v_weights, stride3
%define base r5-ipred_smooth_8bpc_avx512icl_table
    lea                  r5, [ipred_smooth_8bpc_avx512icl_table]
    mov                 r6d, wd
    tzcnt                wd, wd
    mov                  hd, hm
    vpbroadcastb         m6, [tlq+r6] ; right
    sub                 tlq, hq
    movsxd               wq, [r5+wq*4]
    vpbroadcastd         m7, [base+pb_127_m127]
    vpbroadcastb         m0, [tlq]    ; bottom
    vpbroadcastd         m1, [base+pw_255]
    add                  wq, r5
    lea          v_weightsq, [base+smooth_weights+hq*2]
    vpmovb2m             k1, m1
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    vpbroadcastd         m8, [tlq+hq+1]
    movsldup             m4, [smooth_shuf]
    movshdup             m5, [smooth_shuf]
    vpbroadcastq         m9, [smooth_weights+4*2]
    mova               ym11, [smooth_endA]

    punpcklbw            m8, m0     ; top, bottom
    pmaddubsw           m10, m8, m7
    paddw                m1, m8     ;   1 * top + 256 * bottom + 255
    paddw               m10, m1     ; 128 * top + 129 * bottom + 255
.w4_loop:
    vpbroadcastq         m1, [tlq+hq-8]
    vbroadcasti32x4      m0, [v_weightsq]
    add          v_weightsq, 16
    mova                 m2, m6
    vpshufb          m2{k1}, m1, m4 ; left, right
    pmaddubsw            m1, m2, m7 ; 127 * left - 127 * right
    pshufb               m0, m5
    pmaddubsw            m0, m8, m0
    paddw                m1, m2     ; 128 * left + 129 * right
    pmaddubsw            m2, m9
    paddw                m0, m10
    paddw                m1, m2
    pavgw                m0, m1
    vpermb               m0, m11, m0
    vextracti32x4       xm1, ym0, 1
    movd   [dstq+strideq*0], xm0
    movd   [dstq+strideq*1], xm1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+stride3q ], xm1, 2
    sub                  hd, 8
    jl .ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 1
    pextrd [dstq+strideq*1], xm1, 1
    pextrd [dstq+strideq*2], xm0, 3
    pextrd [dstq+stride3q ], xm1, 3
    lea                dstq, [dstq+strideq*4]
    jg .w4_loop
.ret:
    RET
.w8:
    vpbroadcastq         m8, [tlq+hq+1]
    movsldup             m4, [smooth_shuf]
    movshdup             m5, [smooth_shuf]
    vbroadcasti32x4      m9, [smooth_weights+8*2]
    mova               ym11, [smooth_endA]
    punpcklbw            m8, m0
    pmaddubsw           m10, m8, m7
    paddw                m1, m8
    paddw               m10, m1
.w8_loop:
    vpbroadcastd         m1, [tlq+hq-4]
    vpbroadcastq         m0, [v_weightsq]
    add          v_weightsq, 8
    mova                 m2, m6
    vpshufb          m2{k1}, m1, m4
    pmaddubsw            m1, m2, m7
    pshufb               m0, m5
    pmaddubsw            m0, m8, m0
    paddw                m1, m2
    pmaddubsw            m2, m9
    paddw                m0, m10
    paddw                m1, m2
    pavgw                m0, m1
    vpermb               m0, m11, m0
    vextracti32x4       xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8_loop
    RET
.w16:
    vbroadcasti32x4      m9, [tlq+hq+1]
    movsldup             m5, [smooth_shuf]
    movshdup            m10, [smooth_shuf]
    vbroadcasti32x4     m11, [smooth_weights+16*2]
    vbroadcasti32x4     m12, [smooth_weights+16*3]
    mova                m15, [smooth_endB]
    punpcklbw            m8, m9, m0
    punpckhbw            m9, m0
    pmaddubsw           m13, m8, m7
    pmaddubsw           m14, m9, m7
    paddw                m0, m1, m8
    paddw                m1, m9
    paddw               m13, m0
    paddw               m14, m1
.w16_loop:
    vpbroadcastd         m0, [tlq+hq-4]
    vpbroadcastq         m1, [v_weightsq]
    add          v_weightsq, 8
    mova                 m4, m6
    vpshufb          m4{k1}, m0, m5
    pmaddubsw            m2, m4, m7
    pshufb               m1, m10
    pmaddubsw            m0, m8, m1
    pmaddubsw            m1, m9, m1
    paddw                m2, m4
    pmaddubsw            m3, m4, m11
    pmaddubsw            m4, m12
    paddw                m0, m13
    paddw                m1, m14
    paddw                m3, m2
    paddw                m4, m2
    pavgw                m0, m3
    pavgw                m1, m4
    vpermt2b             m0, m15, m1
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16_loop
    RET
.w32:
    vbroadcasti32x8      m9, [tlq+hq+1]
    movshdup            m10, [smooth_shuf]
    mova                m12, [smooth_weights+32*2]
    vpbroadcastd        ym5, [pb_1]
    mova                m15, [smooth_endB]
    punpcklbw            m8, m9, m0
    punpckhbw            m9, m0
    pmaddubsw           m13, m8, m7
    pmaddubsw           m14, m9, m7
    vshufi32x4          m11, m12, m12, q2020
    vshufi32x4          m12, m12, q3131
    paddw                m0, m1, m8
    paddw                m1, m9
    paddw               m13, m0
    paddw               m14, m1
.w32_loop:
    vpbroadcastd         m0, [tlq+hq-2]
    vpbroadcastd         m1, [v_weightsq]
    add          v_weightsq, 4
    mova                 m4, m6
    vpshufb          m4{k1}, m0, m5
    pmaddubsw            m2, m4, m7
    pshufb               m1, m10
    pmaddubsw            m0, m8, m1
    pmaddubsw            m1, m9, m1
    paddw                m2, m4
    pmaddubsw            m3, m4, m11
    pmaddubsw            m4, m12
    paddw                m0, m13
    paddw                m1, m14
    paddw                m3, m2
    paddw                m4, m2
    pavgw                m0, m3
    pavgw                m1, m4
    vpermt2b             m0, m15, m1
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop
    RET
.w64:
    movu                 m9, [tlq+hq+1]
    mova                m11, [smooth_weights+64*2]
    mova                 m2, [smooth_weights+64*3]
    mova                m14, [smooth_endB]
    punpcklbw            m8, m9, m0
    punpckhbw            m9, m0
    pmaddubsw           m12, m8, m7
    pmaddubsw           m13, m9, m7
    vshufi32x4          m10, m11, m2, q2020
    vshufi32x4          m11, m2, q3131
    paddw                m0, m1, m8
    paddw                m1, m9
    paddw               m12, m0
    paddw               m13, m1
.w64_loop:
    mova                 m4, m6
    vpbroadcastb     m4{k1}, [tlq+hq-1]
    vpbroadcastw         m1, [v_weightsq]
    add          v_weightsq, 2
    pmaddubsw            m2, m4, m7
    pmaddubsw            m0, m8, m1
    pmaddubsw            m1, m9, m1
    paddw                m2, m4
    pmaddubsw            m3, m4, m10
    pmaddubsw            m4, m11
    paddw                m0, m12
    paddw                m1, m13
    paddw                m3, m2
    paddw                m4, m2
    pavgw                m0, m3
    pavgw                m1, m4
    vpermt2b             m0, m14, m1
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET

cglobal pal_pred_8bpc, 4, 7, 5, dst, stride, pal, idx, w, h, stride3
    lea                  r6, [pal_pred_8bpc_avx512icl_table]
    tzcnt                wd, wm
    vbroadcasti32x4      m4, [palq]
    movifnidn            hd, hm
    movsxd               wq, [r6+wq*4]
    packuswb             m4, m4
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    pshufb             xmm0, xm4, [idxq]
    add                idxq, 16
    movd   [dstq+strideq*0], xmm0
    pextrd [dstq+strideq*1], xmm0, 1
    pextrd [dstq+strideq*2], xmm0, 2
    pextrd [dstq+stride3q ], xmm0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4
    RET
.w8:
    pshufb             xmm0, xm4, [idxq+16*0]
    pshufb             xmm1, xm4, [idxq+16*1]
    add                idxq, 16*2
    movq   [dstq+strideq*0], xmm0
    movhps [dstq+strideq*1], xmm0
    movq   [dstq+strideq*2], xmm1
    movhps [dstq+stride3q ], xmm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8
    RET
.w16:
    pshufb               m0, m4, [idxq]
    add                idxq, 64
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16
    RET
.w32:
    pshufb               m0, m4, [idxq+64*0]
    pshufb               m1, m4, [idxq+64*1]
    add                idxq, 64*2
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w32
    RET
.w64:
    pshufb               m0, m4, [idxq+64*0]
    pshufb               m1, m4, [idxq+64*1]
    pshufb               m2, m4, [idxq+64*2]
    pshufb               m3, m4, [idxq+64*3]
    add                idxq, 64*4
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w64
    RET

; The ipred_filter code processes 4x2 blocks in the following order
; which increases parallelism compared to doing things row by row.
; Some redundant blocks are calculated for w > 4.
;     w4     w8       w16             w32
;     1     1 2     1 2 3 4     1 2 3 4 9 a b c
;     2     2 3     2 3 4 5     2 3 4 5 a b c d
;     3     3 4     3 4 5 6     3 4 5 6 b c d e
;     4     4 5     4 5 6 7     4 5 6 7 c d e f
;     5     5 6     5 6 7 8     5 6 7 8 d e f g
;     6     6 7     6 7 8 9     6 7 8 9 e f g h
;     7     7 8     7 8 9 a     7 8 9 a f g h i
; ___ 8 ___ 8 9 ___ 8 9 a b ___ 8 9 a b g h i j ___
;           9       9 a b               h i j
;                   a b                 i j
;                   b                   j

cglobal ipred_filter_8bpc, 4, 7, 14, dst, stride, tl, w, h, flt
%define base r6-filter_taps
    lea                  r6, [filter_taps]
%ifidn fltd, fltm
    movzx              fltd, fltb
%else
    movzx              fltd, byte fltm
%endif
    vpbroadcastd       xmm2, [tlq+1]        ; t0 t0 t0 t0
    movifnidn            hd, hm
    shl                fltd, 6
    vpbroadcastd         m6, [base+pd_8]
    vpbroadcastd       xmm3, [tlq-2]        ; l1 l0 tl __
    vbroadcasti32x4      m7, [r6+fltq+16*0] ; p1 p2 p3 p4
    vbroadcasti32x4      m8, [r6+fltq+16*1]
    vbroadcasti32x4      m9, [r6+fltq+16*2] ; p6 p5 p0 __
    vbroadcasti32x4     m10, [r6+fltq+16*3]
    mova               xmm0, xm6
    vpdpbusd           xmm0, xmm2, xm7
    mova               xmm1, xm6
    vpdpbusd           xmm1, xmm2, xm8
    vpdpbusd           xmm0, xmm3, xm9
    vpdpbusd           xmm1, xmm3, xm10
    packssdw           xmm0, xmm1
    cmp                  wd, 8
    jb .w4
    vpbroadcastd        ym2, [tlq+5]
    mova                m11, [base+filter_perm]
    mov                  r5, 0xffffffffffff000f
    psrldq             xmm2, 1           ; __ t0
    kmovq                k1, r5          ; 0x000f
    psraw               xm5, xmm0, 4
    packuswb           xmm2, xm5         ; __ t0 a0 b0
    pshufd          ym2{k1}, ymm2, q3333 ; b0 b0 b0 b0   t1 t1 t1 t1
    je .w8
    kxnorb               k3, k3, k3      ; 0x00ff
    vpbroadcastd        xm3, [tlq-4]
    kandnq               k2, k3, k1      ; 0xffffffffffff0000
    vpermb          ym3{k2}, ym11, ymm2  ; l3 l2 l1 __   b3 a3 t3 __
    mova                ym0, ym6
    vpdpbusd            ym0, ym2, ym7
    mova                ym1, ym6
    vpdpbusd            ym1, ym2, ym8
    pshufb          ym5{k2}, ym2, ym11   ; a0 b0   __ t0
    vpbroadcastd         m2, [tlq+9]
    vpdpbusd            ym0, ym3, ym9
    vpdpbusd            ym1, ym3, ym10
    vpbroadcastd        xm3, [tlq-6]     ; l5 l4 l3 __
    kunpckbw             k4, k1, k3      ; 0x0fff
    packssdw            ym0, ym1
    psraw               ym0, 4           ; a0 d0         a1 b1
    packuswb            ym5, ym0         ; a0 b0 c0 d0   __ t1 a1 b1
    pshufd           m2{k3}, m5, q3333   ; d0 d0 d0 d0   b1 b1 b1 b1   t2 t2 t2 t2
    vpermb           m3{k2}, m11, m5     ; l5 l4 l3 __   d3 c3 b3 __   b7 a7 t7 __
    mova                 m4, m6
    vpdpbusd             m4, m2, m7
    mova                 m1, m6
    vpdpbusd             m1, m2, m8
    psrldq               m0, m2, 1       ; __ d0         __ b0         __ t0
    vpbroadcastd         m2, [tlq+13]
    vpdpbusd             m4, m3, m9
    vpdpbusd             m1, m3, m10
    mova                m12, [base+filter_end]
    lea                 r5d, [hq-6]
    mov                  r6, dstq
    cmovp                hd, r5d         ; w == 16 ? h : h - 6
    packssdw             m4, m1
    psraw                m4, 4           ; e0 f0         c1 d1         a2 b2
    packuswb             m0, m4          ; __ d0 e0 f0   __ b1 c1 d1   __ t2 a2 b2
    pshufd           m2{k4}, m0, q3333   ; f0 f0 f0 f0   d1 d1 d1 d1   b2 b2 b2 b2   t3 t3 t3 t3
.w16_loop:
    vpbroadcastd        xm3, [tlq-8]
    vpermb           m3{k2}, m11, m0     ; l7 l6 l5 __   f3 e3 d3 __   d7 c7 b7 __   bb ab tb __
    mova                 m1, m6
    vpdpbusd             m1, m2, m7
    mova                 m0, m6
    vpdpbusd             m0, m2, m8
    sub                 tlq, 2
    vpdpbusd             m1, m3, m9
    vpdpbusd             m0, m3, m10
    packssdw             m1, m0
    mova                 m0, m4
    psraw                m4, m1, 4       ; g0 h0         e1 f1         c2 d2         a3 b3
    packuswb             m0, m4          ; e0 f0 g0 h0   c1 d1 e1 f1   a2 b2 c2 d2   __ __ a3 b3
    pshufd               m2, m0, q3333   ; h0 h0 h0 h0   f1 f1 f1 f1   d2 d2 d2 d2   b3 b3 b3 b3
    vpermt2d             m5, m12, m0     ; c0 d0 e0 f0   __ __ c1 d1   a0 a1 a2 a3   b0 b1 b2 b3
    vextracti32x4 [dstq+strideq*0], m5, 2
    vextracti32x4 [dstq+strideq*1], m5, 3
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w16_loop
    cmp                  wd, 16
    je .ret
    mova               xm13, [filter_perm+16]
    mova               xmm3, [r6+strideq*0]
    punpckhdq          xmm3, [r6+strideq*1]
    vpbroadcastd     m2{k1}, [tlq+r5+17] ; t4 t4 t4 t4   f1 f1 f1 f1   d2 d2 d2 d2   b3 b3 b3 b3
    pinsrb              xm3, xmm3, [tlq+r5+16], 7
    pshufb              xm3, xm13
    vpermb           m3{k2}, m11, m0     ; bf af tf __   h3 g3 f3 __   f7 e7 d7 __   db cb bb __
    mova                 m0, m6
    vpdpbusd             m0, m2, m7
    mova                 m1, m6
    vpdpbusd             m1, m2, m8
    kunpckbw             k5, k3, k1      ; 0xff0f
    lea                  r3, [strideq*3]
    vpdpbusd             m0, m3, m9
    vpdpbusd             m1, m3, m10
    packssdw             m0, m1
    psraw                m0, 4           ; a4 b4         g1 h1         e2 f2         c3 d3
    packuswb             m4, m0          ; g0 h0 a4 b4   e1 f1 g1 h1   c2 d2 e2 f2   __ __ c3 d3
    vpblendmb        m1{k3}, m4, m2      ; __ t4 a4 b4   e1 f1 g1 h1   c2 d2 e2 f2   __ __ c3 d3
    vpbroadcastd        ym2, [tlq+r5+21]
    pshufd           m2{k5}, m4, q3333   ; b4 b4 b4 b4   t5 t5 t5 t5   f2 f2 f2 f2   d3 d3 d3 d3
    vpermt2d             m5, m12, m4     ; e0 f0 g0 h0   __ __ e1 f1   c0 c1 c2 c3   d0 d1 d2 d3
    vextracti32x4 [dstq+strideq*0], m5, 2
    vextracti32x4 [dstq+strideq*1], m5, 3
    punpckhqdq         xmm3, [r6+r3]
    pinsrb             xmm3, [r6+strideq*2+15], 11
    pshufb              xm3, xmm3, xm13
    vpermb           m3{k2}, m11, m1     ; df cf bf __   bj aj tj __   h7 g7 f7 __   fb eb db __
    mova                 m4, m6
    vpdpbusd             m4, m2, m7
    mova                 m1, m6
    vpdpbusd             m1, m2, m8
    kxnord               k3, k3, k4      ; 0xfffff0ff
    lea                  r4, [strideq*5]
    vpdpbusd             m4, m3, m9
    vpdpbusd             m1, m3, m10
    packssdw             m4, m1
    psraw                m4, 4           ; c4 d4         a5 b5         g2 h2         e3 f3
    packuswb             m0, m4          ; a4 b4 c4 d4   g1 h1 a5 b5   e2 f2 g2 h2   __ __ e3 f3
    vpblendmw        m1{k3}, m2, m0      ; a4 b4 c4 d4   __ t5 a5 b5   e2 f2 g2 h2   __ __ e3 f3
    vpbroadcastd         m2, [tlq+r5+25]
    pshufd           m2{k3}, m0, q3333   ; d4 d4 d4 d4   b5 b5 b5 b5   t6 t6 t6 t6   f3 f3 f3 f3
    vpermt2d             m5, m12, m0     ; g0 h0 a4 b4   __ __ g1 h1   e0 e1 e2 e3   f0 f1 f2 f3
    vextracti32x4 [dstq+strideq*2], m5, 2
    vextracti32x4 [dstq+r3       ], m5, 3
    punpckhqdq         xmm3, [r6+r4]
    pinsrb             xmm3, [r6+strideq*4+15], 11
    pshufb              xm3, xmm3, xm13
    vpermb           m3{k2}, m11, m1     ; ff ef df __   dj cj bj __   bn an tn __   hb hb fb __
    mova                 m0, m6
    vpdpbusd             m0, m2, m7
    mova                 m1, m6
    vpdpbusd             m1, m2, m8
    kunpckwd             k1, k1, k2      ; 0x000f0000
    vpdpbusd             m0, m3, m9
    vpdpbusd             m1, m3, m10
    packssdw             m0, m1
    psraw                m0, 4           ; e4 f4         c5 d5         a6 b6         g3 h3
    packuswb             m4, m0          ; c4 d4 e4 f4   a5 b5 c5 d5   g2 h2 a6 b6   __ __ g3 h3
    vpblendmw        m1{k1}, m4, m2      ; c4 d4 e4 f4   a5 b5 c5 d5   __ t6 a6 b6   __ __ g3 h3
    vpbroadcastd         m2, [tlq+r5+29]
    pshufd           m2{k4}, m4, q3333   ; f4 f4 f4 f4   d5 d5 d5 d5   b6 b6 b6 b6   t7 t7 t7 t7
    vpermt2d             m5, m12, m4     ; a4 b4 c4 d4   __ __ a5 b5   g0 g1 g2 g3   h0 h1 h2 h3
    vextracti32x4 [dstq+strideq*4], m5, 2
    vextracti32x4 [dstq+r4       ], m5, 3
    lea                  r0, [strideq+r3*2]
.w32_loop:
    punpckhqdq         xmm3, [r6+r0]
    pinsrb             xmm3, [r6+r3*2+15], 11
    pshufb              xm3, xmm3, xm13
    vpermb           m3{k2}, m11, m1     ; hf gf ff __   fj ej dj __   dn cn bn __   br ar tr __
.w32_loop_tail:
    mova                 m4, m6
    vpdpbusd             m4, m2, m7
    mova                 m1, m6
    vpdpbusd             m1, m2, m8
    vpdpbusd             m4, m3, m9
    vpdpbusd             m1, m3, m10
    packssdw             m4, m1
    mova                 m1, m0
    psraw                m0, m4, 4       ; g4 h4         e5 f5         c6 d6         a7 b7
    packuswb             m1, m0          ; e4 f4 g4 h4   c5 d5 e5 f5   a6 b6 c6 d6   __ __ a7 b7
    pshufd               m2, m1, q3333   ; h4 h4 h4 h4   f5 f5 f5 f5   d6 d6 d6 d6   b7 b7 b7 b7
    vpermt2d             m5, m12, m1     ; c4 d4 e4 f4   __ __ c5 d5   a4 a5 a6 a7   b4 b5 b6 b7
    vextracti32x4 [r6+strideq*0+16], m5, 2
    vextracti32x4 [r6+strideq*1+16], m5, 3
    lea                  r6, [r6+strideq*2]
    sub                 r5d, 2
    jg .w32_loop
    vpermb               m3, m11, m1
    cmp                 r5d, -6
    jg .w32_loop_tail
.ret:
    RET
.w8:
    vpermb              ym3, ym11, ymm2
.w8_loop:
    vpbroadcastd    ym3{k1}, [tlq-4]     ; l3 l2 l1 __   b3 a3 t3 __
    mova                ym0, ym6
    vpdpbusd            ym0, ym2, ym7
    mova                ym1, ym6
    vpdpbusd            ym1, ym2, ym8
    sub                 tlq, 2
    vpdpbusd            ym0, ym3, ym9
    vpdpbusd            ym1, ym3, ym10
    mova                ym3, ym5
    packssdw            ym0, ym1
    psraw               ym5, ym0, 4      ; c0 d0         a1 b1
    packuswb            ym3, ym5         ; a0 b0 c0 d0   __ __ a1 b1
    pshufd              ym2, ym3, q3333  ; d0 d0 d0 d0   b1 b1 b1 b1
    vpermb              ym3, ym11, ym3   ; a0 a1 b0 b1
    movq   [dstq+strideq*0], xm3
    movhps [dstq+strideq*1], xm3
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
.w4_loop:
    vpbroadcastd       xmm3, [tlq-4]     ; l3 l2 l1 __
    mova               xmm0, xm6
    vpdpbusd           xmm0, xmm2, xm7
    mova               xmm1, xm6
    vpdpbusd           xmm1, xmm2, xm8
    sub                 tlq, 2
    vpdpbusd           xmm0, xmm3, xm9
    vpdpbusd           xmm1, xmm3, xm10
    packssdw           xmm0, xmm1
.w4:
    psraw              xmm0, 4           ; a0 b0
    packuswb           xmm0, xmm0
    movd   [dstq+strideq*0], xmm0
    pshufd             xmm2, xmm0, q1111 ; b0 b0 b0 b0
    movd   [dstq+strideq*1], xmm2
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w4_loop
    RET

%endif ; ARCH_X86_64
