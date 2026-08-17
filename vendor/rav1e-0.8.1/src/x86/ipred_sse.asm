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

SECTION_RODATA 16

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

ipred_v_shuf:     db  0,  1,  0,  1,  2,  3,  2,  3,  4,  5,  4,  5,  6,  7,  6,  7
ipred_h_shuf:     db  3,  3,  3,  3,  2,  2,  2,  2,  1,  1,  1,  1,  0,  0,  0,  0
ipred_paeth_shuf: db  1,  1,  1,  1,  1,  1,  1,  1,  0,  0,  0,  0,  0,  0,  0,  0
z_upsample1:      db  1,  0,  2,  1,  3,  2,  4,  3,  5,  4,  6,  5,  7,  6,  8,  7
z_upsample2:      db  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8,  8,  8,  8,  8
z_transpose4:     db  8, 12,  0,  4,  9, 13,  1,  5, 10, 14,  2,  6, 11, 15,  3,  7
z3_shuf:          db  1,  0,  2,  1,  3,  2,  4,  3,  5,  4,  6,  5,  7,  6,  8,  7
z3_shuf_h4:       db  4,  3,  3,  2,  2,  1,  1,  0, 12, 11, 11, 10, 10,  9,  9,  8
filter_shuf1:     db  3,  4,  3,  4,  5,  6,  5,  6,  7,  2,  7,  2,  1, -1,  1, -1
filter_shuf2:     db  3,  4,  3,  4,  5,  6,  5,  6,  7, 11,  7, 11, 15, -1, 15, -1
z_filter_wh4:     db  7,  7, 19,  7,
z_filter_wh8:     db 19, 19, 11, 19, 11, 15, 15, 15, 23, 23, 23, 23, 39, 39, 39, 39
pd_32768:         dd 32768
z3_filter_k_tail: db 64,  0, 64,  0, 64,  0, 56,  8
z1_shuf_w4:       db  0,  1,  1,  2,  2,  3,  3,  4,  8,  9,  9, 10, 10, 11, 11, 12
pb_0to15:         db  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
pb_15to0:         db 15, 14, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1,  0
z_base_inc:       dw   0*64,   1*64,   2*64,   3*64,   4*64,   5*64,   6*64,   7*64
z3_base_inc:      dw   7*64,   6*64,   5*64,   4*64,   3*64,   2*64,   1*64,   0*64
z_filter_wh16:    db 19, 19, 19, 23, 23, 23, 31, 31, 31, 47, 47, 47, 79, 79, 79, -1
z_filter_t_w48:   db 55,127,  7,127, 15, 31, 39, 31,127, 39,127, 39,  7, 15, 31, 15
                  db 39, 63,  3, 63,  3,  3, 19,  3, 47, 19, 47, 19,  3,  3,  3,  3
z_filter_t_w16:   db 15, 31,  7, 15, 31,  7,  3, 31,  3,  3,  3,  3,  3,  3,  0,  0
z_filter_s:       db  0,  0,  0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,  7
                  db  7,  8,  8,  9,  9, 10, 10, 11
z_filter_k_tail:  db  0, 64,  0, 64,  8, 56,  0, 64
z2_h_shuf:        db  7,  6, 15, 14,  6,  5, 14, 13,  5,  4, 13, 12,  4,  3, 12, 11
z2_upsample:      db  7,  6, 15, 14,  5,  4, 13, 12,  3,  2, 11, 10,  1,  0,  9,  8
z2_dy_offset:     dw 88*64, 88*64, 87*64, 87*64
pw_m1to4:         dw -1, -2, -3, -4
z_filter_k:       times  4 db  0, 16
                  times  4 db  0, 20
                  times  4 db  8, 16
                  times  4 db 32, 16
                  times  4 db 24, 20
                  times  4 db 16, 16
                  times  4 db  0,  0
                  times  4 db  0,  0
pw_8:             times  8 db  8,  0
pb_3:             times 16 db 3
pb_16:            times 16 db 16
pw_62:            times  8 dw 62
pw_64:            times  8 dw 64
pw_256:           times  8 dw 256
pw_512:           times  8 dw 512
pw_m256:          times  8 dw -256
pb_2:             times  8 db 2
pb_4:             times  8 db 4
pb_8:             times  8 db 8
pb_128:           times  8 db 128
pb_m16:           times  8 db -16
pw_128:           times  4 dw 128
pw_255:           times  4 dw 255
pb_36_m4:         times  4 db 36, -4
pb_127_m127:      times  4 db 127, -127

%macro JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - 2*4)
    %xdefine %%base mangle(private_prefix %+ _%1_8bpc_%2)
    %%table:
    %rep %0 - 2
        dd %%base %+ .%3 - (%%table - 2*4)
        %rotate 1
    %endrep
%endmacro

%define ipred_dc_splat_ssse3_table (ipred_dc_ssse3_table + 10*4)
%define ipred_cfl_splat_ssse3_table (ipred_cfl_ssse3_table + 8*4)

JMP_TABLE ipred_h,          ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_dc,         ssse3, h4, h8, h16, h32, h64, w4, w8, w16, w32, w64, \
                                s4-10*4, s8-10*4, s16-10*4, s32-10*4, s64-10*4
JMP_TABLE ipred_dc_left,    ssse3, h4, h8, h16, h32, h64
JMP_TABLE ipred_smooth,     ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_v,   ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_h,   ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_paeth,      ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_z1,         ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_z2,         ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_z3,         ssse3, h4, h8, h16, h32, h64
JMP_TABLE pal_pred,         ssse3, w4, w8, w16, w32, w64
JMP_TABLE ipred_cfl,        ssse3, h4, h8, h16, h32, w4, w8, w16, w32, \
                                s4-8*4, s8-8*4, s16-8*4, s32-8*4
JMP_TABLE ipred_cfl_left,   ssse3, h4, h8, h16, h32
JMP_TABLE ipred_filter,     ssse3, w4, w8, w16, w32

cextern dr_intra_derivative
cextern filter_intra_taps

SECTION .text

;---------------------------------------------------------------------------------------
;int dav1d_ipred_h_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
%macro IPRED_SET   3                                          ; width, stride, stride size pshuflw_imm8
    pshuflw                      m1, m0, %3                   ; extend 8 byte for 2 pos
    punpcklqdq                   m1, m1
    mova           [dstq +      %2], m1
%if %1 > 16
    mova           [dstq + 16 + %2], m1
%endif
%if %1 > 32
    mova           [dstq + 32 + %2], m1
    mova           [dstq + 48 + %2], m1
%endif
%endmacro

%macro IPRED_H 1                                            ; width
    sub                         tlq, 4
    movd                         m0, [tlq]                  ; get 4 bytes of topleft data
    punpcklbw                    m0, m0                     ; extend 2 byte
%if %1 == 4
    pshuflw                      m1, m0, q2233
    movd           [dstq+strideq*0], m1
    psrlq                        m1, 32
    movd           [dstq+strideq*1], m1
    pshuflw                      m0, m0, q0011
    movd           [dstq+strideq*2], m0
    psrlq                        m0, 32
    movd           [dstq+stride3q ], m0

%elif %1 == 8
    punpcklwd                    m0, m0
    punpckhdq                    m1, m0, m0
    punpckldq                    m0, m0
    movq           [dstq+strideq*1], m1
    movhps         [dstq+strideq*0], m1
    movq           [dstq+stride3q ], m0
    movhps         [dstq+strideq*2], m0
%else
    IPRED_SET                    %1,         0, q3333
    IPRED_SET                    %1,   strideq, q2222
    IPRED_SET                    %1, strideq*2, q1111
    IPRED_SET                    %1,  stride3q, q0000
%endif
    lea                        dstq, [dstq+strideq*4]
    sub                          hd, 4
    jg .w%1
    RET
%endmacro

INIT_XMM ssse3
cglobal ipred_h_8bpc, 3, 6, 2, dst, stride, tl, w, h, stride3
    LEA                          r5, ipred_h_ssse3_table
    tzcnt                        wd, wm
    movifnidn                    hd, hm
    movsxd                       wq, [r5+wq*4]
    add                          wq, r5
    lea                    stride3q, [strideq*3]
    jmp                          wq
.w4:
    IPRED_H                       4
.w8:
    IPRED_H                       8
.w16:
    IPRED_H                      16
.w32:
    IPRED_H                      32
.w64:
    IPRED_H                      64

;---------------------------------------------------------------------------------------
;int dav1d_ipred_v_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
cglobal ipred_v_8bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    LEA                  r5, ipred_dc_splat_ssse3_table
    tzcnt                wd, wm
    movu                 m0, [tlq+ 1]
    movu                 m1, [tlq+17]
    movu                 m2, [tlq+33]
    movu                 m3, [tlq+49]
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

;---------------------------------------------------------------------------------------
;int dav1d_ipred_dc_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
cglobal ipred_dc_8bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    movifnidn                    hd, hm
    movifnidn                    wd, wm
    tzcnt                       r6d, hd
    lea                         r5d, [wq+hq]
    movd                         m4, r5d
    tzcnt                       r5d, r5d
    movd                         m5, r5d
    LEA                          r5, ipred_dc_ssse3_table
    tzcnt                        wd, wd
    movsxd                       r6, [r5+r6*4]
    movsxd                       wq, [r5+wq*4+20]
    pcmpeqd                      m3, m3
    psrlw                        m4, 1                             ; dc = (width + height) >> 1;
    add                          r6, r5
    add                          wq, r5
    lea                    stride3q, [strideq*3]
    jmp r6
.h4:
    movd                         m0, [tlq-4]
    pmaddubsw                    m0, m3
    jmp                          wq
.w4:
    movd                         m1, [tlq+1]
    pmaddubsw                    m1, m3
    psubw                        m0, m4
    paddw                        m0, m1
    pmaddwd                      m0, m3
    cmp                          hd, 4
    jg .w4_mul
    psrlw                        m0, 3                             ; dc >>= ctz(width + height);
    jmp .w4_end
.w4_mul:
    punpckhqdq                   m1, m0, m0
    paddw                        m0, m1
    psrlq                        m1, m0, 32
    paddw                        m0, m1
    psrlw                        m0, 2
    mov                         r6d, 0x5556
    mov                         r2d, 0x3334
    test                         hd, 8
    cmovz                       r6d, r2d
    movd                         m5, r6d
    pmulhuw                      m0, m5
.w4_end:
    pxor                         m1, m1
    pshufb                       m0, m1
.s4:
    movd           [dstq+strideq*0], m0
    movd           [dstq+strideq*1], m0
    movd           [dstq+strideq*2], m0
    movd           [dstq+stride3q ], m0
    lea                        dstq, [dstq+strideq*4]
    sub                          hd, 4
    jg .s4
    RET
ALIGN function_align
.h8:
    movq                         m0, [tlq-8]
    pmaddubsw                    m0, m3
    jmp                          wq
.w8:
    movq                         m1, [tlq+1]
    pmaddubsw                    m1, m3
    psubw                        m4, m0
    punpckhqdq                   m0, m0
    psubw                        m0, m4
    paddw                        m0, m1
    pshuflw                      m1, m0, q1032                  ; psrlq  m1, m0, 32
    paddw                        m0, m1
    pmaddwd                      m0, m3
    psrlw                        m0, m5
    cmp                          hd, 8
    je .w8_end
    mov                         r6d, 0x5556
    mov                         r2d, 0x3334
    cmp                          hd, 32
    cmovz                       r6d, r2d
    movd                         m1, r6d
    pmulhuw                      m0, m1
.w8_end:
    pxor                         m1, m1
    pshufb                       m0, m1
.s8:
    movq           [dstq+strideq*0], m0
    movq           [dstq+strideq*1], m0
    movq           [dstq+strideq*2], m0
    movq           [dstq+stride3q ], m0
    lea                        dstq, [dstq+strideq*4]
    sub                          hd, 4
    jg .s8
    RET
ALIGN function_align
.h16:
    mova                         m0, [tlq-16]
    pmaddubsw                    m0, m3
    jmp                          wq
.w16:
    movu                         m1, [tlq+1]
    pmaddubsw                    m1, m3
    paddw                        m0, m1
    psubw                        m4, m0
    punpckhqdq                   m0, m0
    psubw                        m0, m4
    pshuflw                      m1, m0, q1032                  ; psrlq  m1, m0, 32
    paddw                        m0, m1
    pmaddwd                      m0, m3
    psrlw                        m0, m5
    cmp                          hd, 16
    je .w16_end
    mov                         r6d, 0x5556
    mov                         r2d, 0x3334
    test                         hd, 8|32
    cmovz                       r6d, r2d
    movd                         m1, r6d
    pmulhuw                      m0, m1
.w16_end:
    pxor                         m1, m1
    pshufb                       m0, m1
.s16:
    mova           [dstq+strideq*0], m0
    mova           [dstq+strideq*1], m0
    mova           [dstq+strideq*2], m0
    mova           [dstq+stride3q ], m0
    lea                        dstq, [dstq+strideq*4]
    sub                          hd, 4
    jg .s16
    RET
ALIGN function_align
.h32:
    mova                         m0, [tlq-32]
    pmaddubsw                    m0, m3
    mova                         m2, [tlq-16]
    pmaddubsw                    m2, m3
    paddw                        m0, m2
    jmp wq
.w32:
    movu                         m1, [tlq+1]
    pmaddubsw                    m1, m3
    movu                         m2, [tlq+17]
    pmaddubsw                    m2, m3
    paddw                        m1, m2
    paddw                        m0, m1
    psubw                        m4, m0
    punpckhqdq                   m0, m0
    psubw                        m0, m4
    pshuflw                      m1, m0, q1032                   ; psrlq  m1, m0, 32
    paddw                        m0, m1
    pmaddwd                      m0, m3
    psrlw                        m0, m5
    cmp                          hd, 32
    je .w32_end
    lea                         r2d, [hq*2]
    mov                         r6d, 0x5556
    mov                         r2d, 0x3334
    test                         hd, 64|16
    cmovz                       r6d, r2d
    movd                         m1, r6d
    pmulhuw                      m0, m1
.w32_end:
    pxor                         m1, m1
    pshufb                       m0, m1
    mova                         m1, m0
.s32:
    mova                     [dstq], m0
    mova                  [dstq+16], m1
    mova             [dstq+strideq], m0
    mova          [dstq+strideq+16], m1
    mova           [dstq+strideq*2], m0
    mova        [dstq+strideq*2+16], m1
    mova            [dstq+stride3q], m0
    mova         [dstq+stride3q+16], m1
    lea                        dstq, [dstq+strideq*4]
    sub                          hd, 4
    jg .s32
    RET
ALIGN function_align
.h64:
    mova                         m0, [tlq-64]
    mova                         m1, [tlq-48]
    pmaddubsw                    m0, m3
    pmaddubsw                    m1, m3
    paddw                        m0, m1
    mova                         m1, [tlq-32]
    pmaddubsw                    m1, m3
    paddw                        m0, m1
    mova                         m1, [tlq-16]
    pmaddubsw                    m1, m3
    paddw                        m0, m1
    jmp wq
.w64:
    movu                         m1, [tlq+ 1]
    movu                         m2, [tlq+17]
    pmaddubsw                    m1, m3
    pmaddubsw                    m2, m3
    paddw                        m1, m2
    movu                         m2, [tlq+33]
    pmaddubsw                    m2, m3
    paddw                        m1, m2
    movu                         m2, [tlq+49]
    pmaddubsw                    m2, m3
    paddw                        m1, m2
    paddw                        m0, m1
    psubw                        m4, m0
    punpckhqdq                   m0, m0
    psubw                        m0, m4
    pshuflw                      m1, m0, q1032                   ; psrlq  m1, m0, 32
    paddw                        m0, m1
    pmaddwd                      m0, m3
    psrlw                        m0, m5
    cmp                          hd, 64
    je .w64_end
    mov                         r6d, 0x5556
    mov                         r2d, 0x3334
    test                         hd, 32
    cmovz                       r6d, r2d
    movd                         m1, r6d
    pmulhuw                      m0, m1
.w64_end:
    pxor                         m1, m1
    pshufb                       m0, m1
    mova                         m1, m0
    mova                         m2, m0
    mova                         m3, m0
.s64:
    mova                     [dstq], m0
    mova                  [dstq+16], m1
    mova                  [dstq+32], m2
    mova                  [dstq+48], m3
    mova             [dstq+strideq], m0
    mova          [dstq+strideq+16], m1
    mova          [dstq+strideq+32], m2
    mova          [dstq+strideq+48], m3
    lea                        dstq, [dstq+strideq*2]
    sub                          hd, 2
    jg .s64
    RET

;---------------------------------------------------------------------------------------
;int dav1d_ipred_dc_left_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
cglobal ipred_dc_left_8bpc, 3, 7, 6, dst, stride, tl, w, h, stride3
    LEA                  r5, ipred_dc_left_ssse3_table
    mov                  hd, hm                ; zero upper half
    tzcnt               r6d, hd
    sub                 tlq, hq
    tzcnt                wd, wm
    movu                 m0, [tlq]
    movd                 m3, [r5-ipred_dc_left_ssse3_table+pd_32768]
    movd                 m2, r6d
    psrld                m3, m2
    movsxd               r6, [r5+r6*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, r5
    add                  r5, ipred_dc_splat_ssse3_table-ipred_dc_left_ssse3_table
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6
.h64:
    movu                 m1, [tlq+48]                           ; unaligned when jumping here from dc_top
    pmaddubsw            m1, m2
    paddw                m0, m1
    movu                 m1, [tlq+32]                           ; unaligned when jumping here from dc_top
    pmaddubsw            m1, m2
    paddw                m0, m1
.h32:
    movu                 m1, [tlq+16]                           ; unaligned when jumping here from dc_top
    pmaddubsw            m1, m2
    paddw                m0, m1
.h16:
    pshufd               m1, m0, q3232                          ; psrlq               m1, m0, 16
    paddw                m0, m1
.h8:
    pshuflw              m1, m0, q1032                          ; psrlq               m1, m0, 32
    paddw                m0, m1
.h4:
    pmaddwd              m0, m2
    pmulhrsw             m0, m3
    lea            stride3q, [strideq*3]
    pxor                 m1, m1
    pshufb               m0, m1
    mova                 m1, m0
    mova                 m2, m0
    mova                 m3, m0
    jmp                  wq

;---------------------------------------------------------------------------------------
;int dav1d_ipred_dc_128_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
cglobal ipred_dc_128_8bpc, 2, 7, 6, dst, stride, tl, w, h, stride3
    LEA                  r5, ipred_dc_splat_ssse3_table
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    movddup              m0, [r5-ipred_dc_splat_ssse3_table+pb_128]
    mova                 m1, m0
    mova                 m2, m0
    mova                 m3, m0
    add                  wq, r5
    lea            stride3q, [strideq*3]
    jmp                  wq

;---------------------------------------------------------------------------------------
;int dav1d_ipred_dc_top_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
cglobal ipred_dc_top_8bpc, 3, 7, 6, dst, stride, tl, w, h
    LEA                  r5, ipred_dc_left_ssse3_table
    tzcnt                wd, wm
    inc                 tlq
    movu                 m0, [tlq]
    movifnidn            hd, hm
    movd                 m3, [r5-ipred_dc_left_ssse3_table+pd_32768]
    movd                 m2, wd
    psrld                m3, m2
    movsxd               r6, [r5+wq*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, r5
    add                  r5, ipred_dc_splat_ssse3_table-ipred_dc_left_ssse3_table
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  r6

;---------------------------------------------------------------------------------------
;int dav1d_ipred_smooth_v_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
%macro SMOOTH 6 ; src[1-2], mul[1-2], add[1-2]
                ;            w * a         = (w - 128) * a + 128 * a
                ;            (256 - w) * b = (127 - w) * b + 129 * b
                ; => w * a + (256 - w) * b = [(w - 128) * a + (127 - w) * b] + [128 * a + 129 * b]
    pmaddubsw            m6, m%3, m%1
    pmaddubsw            m0, m%4, m%2                    ; (w - 128) * a + (127 - w) * b
    paddw                m6, m%5
    paddw                m0, m%6                         ; [(w - 128) * a + (127 - w) * b] + [128 * a + 129 * b + 128]
    psrlw                m6, 8
    psrlw                m0, 8
    packuswb             m6, m0
%endmacro

cglobal ipred_smooth_v_8bpc, 3, 7, 7, dst, stride, tl, w, h, weights
%define base r6-ipred_smooth_v_ssse3_table
    LEA                  r6, ipred_smooth_v_ssse3_table
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    movddup              m0, [base+pb_127_m127]
    movddup              m1, [base+pw_128]
    lea            weightsq, [base+smooth_weights+hq*4]
    neg                  hq
    movd                 m5, [tlq+hq]
    pxor                 m2, m2
    pshufb               m5, m2
    add                  wq, r6
    jmp                  wq
.w4:
    movd                 m2, [tlq+1]
    punpckldq            m2, m2
    punpcklbw            m2, m5                          ; top, bottom
    lea                  r3, [strideq*3]
    mova                 m4, [base+ipred_v_shuf]
    mova                 m5, m4
    punpckldq            m4, m4
    punpckhdq            m5, m5
    pmaddubsw            m3, m2, m0                      ; m3: 127 * top - 127 * bottom
    paddw                m1, m2                          ; m1:   1 * top + 256 * bottom + 128, overflow is ok
    paddw                m3, m1                          ; m3: 128 * top + 129 * bottom + 128
.w4_loop:
    movu                 m1, [weightsq+hq*2]
    pshufb               m0, m1, m4                      ;m2, m3, m4 and m5 should be stable in loop
    pshufb               m1, m5
    SMOOTH                0, 1, 2, 2, 3, 3
    movd   [dstq+strideq*0], m6
    pshuflw              m1, m6, q1032
    movd   [dstq+strideq*1], m1
    punpckhqdq           m6, m6
    movd   [dstq+strideq*2], m6
    psrlq                m6, 32
    movd   [dstq+r3       ], m6
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w4_loop
    RET
ALIGN function_align
.w8:
    movq                 m2, [tlq+1]
    punpcklbw            m2, m5
    mova                 m5, [base+ipred_v_shuf]
    lea                  r3, [strideq*3]
    pshufd               m4, m5, q0000
    pshufd               m5, m5, q1111
    pmaddubsw            m3, m2, m0
    paddw                m1, m2
    paddw                m3, m1                           ; m3 is output for loop
.w8_loop:
    movq                 m1, [weightsq+hq*2]
    pshufb               m0, m1, m4
    pshufb               m1, m5
    SMOOTH                0, 1, 2, 2, 3, 3
    movq   [dstq+strideq*0], m6
    movhps [dstq+strideq*1], m6
    lea                dstq, [dstq+strideq*2]
    add                  hq, 2
    jl .w8_loop
    RET
ALIGN function_align
.w16:
    movu                 m3, [tlq+1]
    punpcklbw            m2, m3, m5
    punpckhbw            m3, m5
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1                           ; m4 and m5 is output for loop
.w16_loop:
    movd                 m1, [weightsq+hq*2]
    pshuflw              m1, m1, q0000
    punpcklqdq           m1, m1
    SMOOTH 1, 1, 2, 3, 4, 5
    mova             [dstq], m6
    add                dstq, strideq
    add                  hq, 1
    jl .w16_loop
    RET
ALIGN function_align
.w32:
%if WIN64
    movaps         [rsp+24], xmm7
    %define xmm_regs_used 8
%endif
    mova                 m7, m5
.w32_loop_init:
    mov                 r3d, 2
.w32_loop:
    movddup              m0, [base+pb_127_m127]
    movddup              m1, [base+pw_128]
    movu                 m3, [tlq+1]
    punpcklbw            m2, m3, m7
    punpckhbw            m3, m7
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1
    movd                 m1, [weightsq+hq*2]
    pshuflw              m1, m1, q0000
    punpcklqdq           m1, m1
    SMOOTH                1, 1, 2, 3, 4, 5
    mova             [dstq], m6
    add                 tlq, 16
    add                dstq, 16
    dec                 r3d
    jg .w32_loop
    lea                dstq, [dstq-32+strideq]
    sub                 tlq, 32
    add                  hq, 1
    jl .w32_loop_init
    RET
ALIGN function_align
.w64:
%if WIN64
    movaps         [rsp+24], xmm7
    %define xmm_regs_used 8
%endif
    mova                 m7, m5
.w64_loop_init:
    mov                 r3d, 4
.w64_loop:
    movddup              m0, [base+pb_127_m127]
    movddup              m1, [base+pw_128]
    movu                 m3, [tlq+1]
    punpcklbw            m2, m3, m7
    punpckhbw            m3, m7
    pmaddubsw            m4, m2, m0
    pmaddubsw            m5, m3, m0
    paddw                m0, m1, m2
    paddw                m1, m3
    paddw                m4, m0
    paddw                m5, m1
    movd                 m1, [weightsq+hq*2]
    pshuflw              m1, m1, q0000
    punpcklqdq           m1, m1
    SMOOTH                1, 1, 2, 3, 4, 5
    mova             [dstq], m6
    add                 tlq, 16
    add                dstq, 16
    dec                 r3d
    jg .w64_loop
    lea                dstq, [dstq-64+strideq]
    sub                 tlq, 64
    add                  hq, 1
    jl .w64_loop_init
    RET

;---------------------------------------------------------------------------------------
;int dav1d_ipred_smooth_h_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
cglobal ipred_smooth_h_8bpc, 3, 7, 8, dst, stride, tl, w, h
%define base r6-ipred_smooth_h_ssse3_table
    LEA                  r6, ipred_smooth_h_ssse3_table
    mov                  wd, wm
    movd                 m3, [tlq+wq]
    pxor                 m1, m1
    pshufb               m3, m1                          ; right
    tzcnt                wd, wd
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    movddup              m4, [base+pb_127_m127]
    movddup              m5, [base+pw_128]
    add                  wq, r6
    jmp                  wq
.w4:
    movddup              m6, [base+smooth_weights+4*2]
    mova                 m7, [base+ipred_h_shuf]
    sub                 tlq, 4
    sub                 tlq, hq
    lea                  r3, [strideq*3]
.w4_loop:
    movd                 m2, [tlq+hq]                    ; left
    pshufb               m2, m7
    punpcklbw            m1, m2, m3                      ; left, right
    punpckhbw            m2, m3
    pmaddubsw            m0, m1, m4                      ; 127 * left - 127 * right
    paddw                m0, m1                          ; 128 * left + 129 * right
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
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    punpckhqdq           m0, m0
    movd   [dstq+strideq*2], m0
    psrlq                m0, 32
    movd   [dstq+r3       ], m0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_loop
    RET
ALIGN function_align
.w8:
    mova                 m6, [base+smooth_weights+8*2]
    mova                 m7, [base+ipred_h_shuf]
    sub                 tlq, 4
    sub                 tlq, hq
    punpckldq            m7, m7
.w8_loop:
    movd                 m2, [tlq+hq]                    ; left
    pshufb               m2, m7
    punpcklbw            m1, m2, m3                      ; left, right
    punpckhbw            m2, m3
    pmaddubsw            m0, m1, m4                      ; 127 * left - 127 * right
    paddw                m0, m1                          ; 128 * left + 129 * right
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
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    mova                 m6, [base+smooth_weights+16*2]
    mova                 m7, [base+smooth_weights+16*3]
    sub                 tlq, 1
    sub                 tlq, hq
.w16_loop:
    pxor                 m1, m1
    movd                 m2, [tlq+hq]                    ; left
    pshufb               m2, m1
    punpcklbw            m1, m2, m3                      ; left, right
    punpckhbw            m2, m3
    pmaddubsw            m0, m1, m4                      ; 127 * left - 127 * right
    paddw                m0, m1                          ; 128 * left + 129 * right
    pmaddubsw            m1, m6
    paddw                m1, m5
    paddw                m0, m1
    pmaddubsw            m1, m2, m4
    paddw                m1, m2
    pmaddubsw            m2, m7
    paddw                m2, m5
    paddw                m1, m2
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
    mova             [dstq], m0
    lea                dstq, [dstq+strideq]
    sub                  hd, 1
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    sub                 tlq, 1
    sub                 tlq, hq
    pxor                 m6, m6
.w32_loop_init:
    mov                  r5, 2
    lea                  r3, [base+smooth_weights+16*4]
.w32_loop:
    mova                 m7, [r3]
    add                  r3, 16
    movd                 m2, [tlq+hq]                    ; left
    pshufb               m2, m6
    punpcklbw            m1, m2, m3                      ; left, right
    punpckhbw            m2, m3
    pmaddubsw            m0, m1, m4                      ; 127 * left - 127 * right
    paddw                m0, m1                          ; 128 * left + 129 * right
    pmaddubsw            m1, m7
    paddw                m1, m5
    paddw                m0, m1
    pmaddubsw            m1, m2, m4
    paddw                m1, m2
    mova                 m7, [r3]
    add                  r3, 16
    pmaddubsw            m2, m7
    paddw                m2, m5
    paddw                m1, m2
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, 16
    dec                  r5
    jg .w32_loop
    lea                dstq, [dstq-32+strideq]
    sub                  hd, 1
    jg .w32_loop_init
    RET
ALIGN function_align
.w64:
    sub                 tlq, 1
    sub                 tlq, hq
    pxor                 m6, m6
.w64_loop_init:
    mov                  r5, 4
    lea                  r3, [base+smooth_weights+16*8]
.w64_loop:
    mova                 m7, [r3]
    add                  r3, 16
    movd                 m2, [tlq+hq]                    ; left
    pshufb               m2, m6
    punpcklbw            m1, m2, m3                      ; left, right
    punpckhbw            m2, m3
    pmaddubsw            m0, m1, m4                      ; 127 * left - 127 * right
    paddw                m0, m1                          ; 128 * left + 129 * right
    pmaddubsw            m1, m7
    paddw                m1, m5
    paddw                m0, m1
    pmaddubsw            m1, m2, m4
    paddw                m1, m2
    mova                 m7, [r3]
    add                  r3, 16
    pmaddubsw            m2, m7
    paddw                m2, m5
    paddw                m1, m2
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, 16
    dec                  r5
    jg .w64_loop
    lea                dstq, [dstq-64+strideq]
    sub                  hd, 1
    jg .w64_loop_init
    RET

;---------------------------------------------------------------------------------------
;int dav1d_ipred_smooth_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                                    const int width, const int height, const int a);
;---------------------------------------------------------------------------------------
%macro SMOOTH_2D_END  7                                  ; src[1-2], mul[1-2], add[1-2], m3
    pmaddubsw            m6, m%3, m%1
    mova                 m0, m6
    pmaddubsw            m6, m%4, m%2
    mova                 m1, m6
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
%ifnum %7
%else
    mova                 m3, %7
%endif
    pavgw                m0, m2
    pavgw                m1, m3
    psrlw                m0, 8
    psrlw                m1, 8
    packuswb             m0, m1
%endmacro

%macro SMOOTH_OUTPUT_16B  12      ; m1, [buffer1, buffer2, buffer3, buffer4,] [w1, w2,] m3, m7, [m0, m4, m5]
    mova                 m1, [rsp+16*%1]                  ; top
    punpckhbw            m6, m1, m0                       ; top, bottom
    punpcklbw            m1, m0                           ; top, bottom
    pmaddubsw            m2, m1, m5
    mova        [rsp+16*%2], m1
    paddw                m1, m3                           ;   1 * top + 255 * bottom + 255
    paddw                m2, m1                           ; 128 * top + 129 * bottom + 255
    mova        [rsp+16*%3], m2
    pmaddubsw            m2, m6, m5
    mova        [rsp+16*%4], m6
    paddw                m6, m3                           ;   1 * top + 255 * bottom + 255
    paddw                m2, m6                           ; 128 * top + 129 * bottom + 255
    mova        [rsp+16*%5], m2
    movd                 m1, [tlq+hq]                     ; left
    pshufb               m1, [base+pb_3]                  ; topleft[-(1 + y)]
    punpcklbw            m1, m4                           ; left, right
    pmaddubsw            m2, m1, m5                       ; 127 * left - 127 * right
    paddw                m2, m1                           ; 128 * left + 129 * right
    mova                 m3, m2
    pmaddubsw            m0, m1, %6                       ; weights_hor = &dav1d_sm_weights[width];
    pmaddubsw            m1, %7
    paddw                m2, m3, m0
    paddw                m3, m1
    movd                 m1, [v_weightsq]                 ; weights_ver = &dav1d_sm_weights[height];
    mova                 m7, [rsp+16*%9]
    pshufb               m1, m7
    mova        [rsp+16*%8], m3
    mova                 m4, [rsp+16*%2]
    mova                 m5, [rsp+16*%3]
    mova                 m3, [rsp+16*%4]
    mova                 m7, [rsp+16*%5]
    SMOOTH_2D_END         1, 1, 4, 3, 5, 7, [rsp+16*%8]
    mova             [dstq], m0
    movddup              m3, [base+pw_255]                ; recovery
    mova                 m0, [rsp+16*%10]                 ; recovery
    mova                 m4, [rsp+16*%11]                 ; recovery
    mova                 m5, [rsp+16*%12]                 ; recovery
%endmacro

cglobal ipred_smooth_8bpc, 3, 7, 8, -13*16, dst, stride, tl, w, h, v_weights
%define base r6-ipred_smooth_ssse3_table
    mov                  wd, wm
    mov                  hd, hm
    LEA                  r6, ipred_smooth_ssse3_table
    movd                 m4, [tlq+wq]                     ; right
    pxor                 m2, m2
    pshufb               m4, m2
    tzcnt                wd, wd
    mov                  r5, tlq
    sub                  r5, hq
    movsxd               wq, [r6+wq*4]
    movddup              m5, [base+pb_127_m127]
    movd                 m0, [r5]
    pshufb               m0, m2                           ; bottom
    movddup              m3, [base+pw_255]
    add                  wq, r6
    lea          v_weightsq, [base+smooth_weights+hq*2]   ; weights_ver = &dav1d_sm_weights[height]
    jmp                  wq
.w4:
    mova                 m7, [base+ipred_v_shuf]
    movd                 m1, [tlq+1]                      ; left
    pshufd               m1, m1, q0000
    sub                 tlq, 4
    lea                  r3, [strideq*3]
    sub                 tlq, hq
    punpcklbw            m1, m0                           ; top, bottom
    pshufd               m6, m7, q1100
    pshufd               m7, m7, q3322
    pmaddubsw            m2, m1, m5
    paddw                m3, m1                           ;   1 * top + 255 * bottom + 255
    paddw                m2, m3                           ; 128 * top + 129 * bottom + 255
    mova         [rsp+16*0], m1
    mova         [rsp+16*1], m2
    movq                 m1,  [base+smooth_weights+4*2]   ; weights_hor = &dav1d_sm_weights[width];
    punpcklqdq           m1, m1
    mova         [rsp+16*2], m1
    mova         [rsp+16*3], m4
    mova         [rsp+16*4], m6
    mova         [rsp+16*5], m5
.w4_loop:
    movd                 m1, [tlq+hq]                 ; left
    pshufb               m1, [base+ipred_h_shuf]
    punpcklbw            m0, m1, m4                   ; left, right
    punpckhbw            m1, m4
    pmaddubsw            m2, m0, m5                   ; 127 * left - 127 * right
    pmaddubsw            m3, m1, m5
    paddw                m2, m0                       ; 128 * left + 129 * right
    paddw                m3, m1
    mova                 m4, [rsp+16*2]
    pmaddubsw            m0, m4
    pmaddubsw            m1, m4
    paddw                m2, m0
    paddw                m3, m1
    movq                 m1, [v_weightsq]             ; weights_ver = &dav1d_sm_weights[height];
    add          v_weightsq, 8
    pshufb               m0, m1, m6
    pshufb               m1, m7
    mova                 m4, [rsp+16*0]
    mova                 m5, [rsp+16*1]
    SMOOTH_2D_END         0, 1, 4, 4, 5, 5, 3
    mova                 m4, [rsp+16*3]
    mova                 m6, [rsp+16*4]
    mova                 m5, [rsp+16*5]
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    punpckhqdq           m0, m0
    movd   [dstq+strideq*2], m0
    psrlq                m0, 32
    movd   [dstq+r3       ], m0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_loop
    RET
ALIGN function_align
.w8:
    mova                 m7, [base+ipred_v_shuf]
    movq                 m1, [tlq+1]                  ; left
    punpcklqdq           m1, m1
    sub                 tlq, 4
    sub                 tlq, hq
    punpcklbw            m1, m0
    pshufd               m6, m7, q0000
    pshufd               m7, m7, q1111
    pmaddubsw            m2, m1, m5
    paddw                m3, m1
    paddw                m2, m3
    mova         [rsp+16*0], m1
    mova         [rsp+16*1], m2
    mova                 m1, [base+smooth_weights+8*2] ; weights_hor = &dav1d_sm_weights[width];
    mova         [rsp+16*2], m1
    mova         [rsp+16*3], m4
    mova         [rsp+16*4], m6
    mova         [rsp+16*5], m5
.w8_loop:
    movd                 m1, [tlq+hq]                  ; left
    pshufb               m1, [base+ipred_h_shuf]
    pshufd               m1, m1, q1100
    punpcklbw            m0, m1, m4
    punpckhbw            m1, m4
    pmaddubsw            m2, m0, m5
    pmaddubsw            m3, m1, m5
    paddw                m2, m0
    paddw                m3, m1
    mova                 m4,  [rsp+16*2]
    pmaddubsw            m0, m4
    pmaddubsw            m1, m4
    paddw                m2, m0
    paddw                m3, m1
    movd                 m1, [v_weightsq]              ; weights_ver = &dav1d_sm_weights[height];
    add          v_weightsq, 4
    pshufb               m0, m1, m6
    pshufb               m1, m7
    mova                 m4, [rsp+16*0]
    mova                 m5, [rsp+16*1]
    SMOOTH_2D_END 0, 1, 4, 4, 5, 5, 3
    mova                 m4, [rsp+16*3]
    mova                 m6, [rsp+16*4]
    mova                 m5, [rsp+16*5]
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    mova                 m7, [base+ipred_v_shuf]
    movu                 m1, [tlq+1]                     ; left
    sub                 tlq, 4
    sub                 tlq, hq
    punpckhbw            m6, m1, m0                      ; top, bottom
    punpcklbw            m1, m0                          ; top, bottom
    pshufd               m7, m7, q0000
    mova         [rsp+16*2], m7
    pmaddubsw            m2, m6, m5
    mova         [rsp+16*5], m6
    paddw                m6, m3                          ;   1 * top + 255 * bottom + 255
    paddw                m2, m6                          ; 128 * top + 129 * bottom + 255
    mova         [rsp+16*6], m2
    pmaddubsw            m2, m1, m5
    paddw                m3, m1                          ;   1 * top + 255 * bottom + 255
    mova         [rsp+16*0], m1
    paddw                m2, m3                          ; 128 * top + 129 * bottom + 255
    mova         [rsp+16*1], m2
    mova         [rsp+16*3], m4
    mova         [rsp+16*4], m5
.w16_loop:
    movd                 m1, [tlq+hq]                    ; left
    pshufb               m1, [base+pb_3]                 ; topleft[-(1 + y)]
    punpcklbw            m1, m4                          ; left, right
    pmaddubsw            m2, m1, m5                      ; 127 * left - 127 * right
    paddw                m2, m1                          ; 128 * left + 129 * right
    mova                 m0, m1
    mova                 m3, m2
    pmaddubsw            m0, [base+smooth_weights+16*2]  ; weights_hor = &dav1d_sm_weights[width];
    pmaddubsw            m1, [base+smooth_weights+16*3]
    paddw                m2, m0
    paddw                m3, m1
    movd                 m1, [v_weightsq]                ; weights_ver = &dav1d_sm_weights[height];
    add          v_weightsq, 2
    mova                 m7, [rsp+16*2]
    pshufb               m1, m7
    mova         [rsp+16*7], m3
    mova                 m4, [rsp+16*0]
    mova                 m5, [rsp+16*1]
    mova                 m3, [rsp+16*5]
    mova                 m7, [rsp+16*6]
    SMOOTH_2D_END 1, 1, 4, 3, 5, 7, [rsp+16*7]
    mova                 m4, [rsp+16*3]
    mova                 m5, [rsp+16*4]
    mova             [dstq], m0
    lea                dstq, [dstq+strideq]
    sub                  hd, 1
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    movu                 m1, [tlq+1]                     ; top     topleft[1 + x]
    movu                 m2, [tlq+17]                    ; top
    mova         [rsp+16*0], m1
    mova         [rsp+16*1], m2
    sub                 tlq, 4
    sub                 tlq, hq
    mova                 m7, [base+ipred_v_shuf]
    pshufd               m7, m7, q0000
    mova         [rsp+16*2], m7
    mova         [rsp+16*3], m0
    mova         [rsp+16*4], m4
    mova         [rsp+16*5], m5
.w32_loop:
    SMOOTH_OUTPUT_16B 0, 6, 7, 8, 9, [base+smooth_weights+16*4], [base+smooth_weights+16*5], 10, 2, 3, 4, 5
    add                dstq, 16
    SMOOTH_OUTPUT_16B 1, 6, 7, 8, 9, [base+smooth_weights+16*6], [base+smooth_weights+16*7], 10, 2, 3, 4, 5
    lea                dstq, [dstq-16+strideq]
    add          v_weightsq, 2
    sub                  hd, 1
    jg .w32_loop
    RET
ALIGN function_align
.w64:
    movu                 m1, [tlq+1]                     ; top     topleft[1 + x]
    movu                 m2, [tlq+17]                    ; top
    mova         [rsp+16*0], m1
    mova         [rsp+16*1], m2
    movu                 m1, [tlq+33]                    ; top
    movu                 m2, [tlq+49]                    ; top
    mova        [rsp+16*11], m1
    mova        [rsp+16*12], m2
    sub                 tlq, 4
    sub                 tlq, hq
    mova                 m7, [base+ipred_v_shuf]
    pshufd               m7, m7, q0000
    mova         [rsp+16*2], m7
    mova         [rsp+16*3], m0
    mova         [rsp+16*4], m4
    mova         [rsp+16*5], m5
.w64_loop:
    SMOOTH_OUTPUT_16B  0, 6, 7, 8, 9,  [base+smooth_weights+16*8],  [base+smooth_weights+16*9], 10, 2, 3, 4, 5
    add                dstq, 16
    SMOOTH_OUTPUT_16B  1, 6, 7, 8, 9, [base+smooth_weights+16*10], [base+smooth_weights+16*11], 10, 2, 3, 4, 5
    add                dstq, 16
    SMOOTH_OUTPUT_16B 11, 6, 7, 8, 9, [base+smooth_weights+16*12], [base+smooth_weights+16*13], 10, 2, 3, 4, 5
    add                dstq, 16
    SMOOTH_OUTPUT_16B 12, 6, 7, 8, 9, [base+smooth_weights+16*14], [base+smooth_weights+16*15], 10, 2, 3, 4, 5
    lea                dstq, [dstq-48+strideq]
    add          v_weightsq, 2
    sub                  hd, 1
    jg .w64_loop
    RET

%if ARCH_X86_64
cglobal ipred_z1_8bpc, 3, 8, 11, 16*12, dst, stride, tl, w, h, angle, dx
    %define            base  r7-$$
    lea                  r7, [$$]
    mova                 m8, [base+pw_62]
    mova                 m9, [base+pw_64]
    mova                m10, [base+pw_512]
%else
cglobal ipred_z1_8bpc, 3, 7, 8, -16*13, dst, _, tl, w, h, angle, dx
    %define            base  r1-$$
    %define              m8  [base+pw_62]
    %define              m9  [base+pw_64]
    %define             m10  [base+pw_512]
    %define         strideq  r3
    %define        stridemp  dword [rsp+16*12]
    mov            stridemp, r1
    LEA                  r1, $$
%endif
    tzcnt                wd, wm
    movifnidn        angled, anglem
    movifnidn            hd, hm
    inc                 tlq
    movsxd               wq, [base+ipred_z1_ssse3_table+wq*4]
    mov                 dxd, angled
    and                 dxd, 0x7e
    add              angled, 165 ; ~90
    lea                  wq, [base+wq+ipred_z1_ssse3_table]
    movzx               dxd, word [base+dr_intra_derivative+dxq]
    xor              angled, 0x4ff ; d = 90 - angle
    jmp                  wq
.w4:
    lea                 r3d, [angleq+88]
    test                r3d, 0x480
    jnz .w4_no_upsample ; !enable_intra_edge_filter || angle >= 40
    sar                 r3d, 9
    add                 r3d, hd
    cmp                 r3d, 8
    jg .w4_no_upsample ; h > 8 || (w == h && is_sm)
    mova                 m1, [tlq-1]
    pshufb               m0, m1, [base+z_upsample1]
    pshufb               m1, [base+z_upsample2]
    movddup              m2, [base+pb_36_m4]
    add                 dxd, dxd
    pmaddubsw            m0, m2
    pshufd               m7, m1, q3333
    movd           [rsp+16], m7 ; top[max_base_x]
    pmaddubsw            m1, m2
    movd                 m6, dxd
    mov                 r5d, dxd ; xpos
    pshufb               m6, [base+pw_256]
    paddw                m1, m0
    movq                 m0, [tlq]
    pmulhrsw             m1, m10
    paddw                m7, m6, m6
    punpcklqdq           m6, m7 ; xpos0 xpos1
    packuswb             m1, m1
    punpcklbw            m0, m1
    movifnidn       strideq, stridemp
    mova              [rsp], m0
.w4_upsample_loop:
    lea                 r2d, [r5+dxq]
    shr                 r5d, 6      ; base0
    movq                 m0, [rsp+r5]
    lea                 r5d, [r2+dxq]
    shr                 r2d, 6      ; base1
    movhps               m0, [rsp+r2]
    pand                 m2, m8, m6 ; frac
    psubw                m1, m9, m2 ; 64-frac
    psllw                m2, 8
    por                  m1, m2     ; 64-frac, frac
    pmaddubsw            m0, m1
    paddw                m6, m7     ; xpos += dx
    pmulhrsw             m0, m10
    packuswb             m0, m0
    movd   [dstq+strideq*0], m0
    pshuflw              m0, m0, q1032
    movd   [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w4_upsample_loop
    RET
.w4_no_upsample:
    mov                 r3d, 7     ; max_base
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .w4_main
    lea                 r3d, [hq+3]
    movd                 m0, r3d
    movd                 m2, angled
    shr              angled, 8 ; is_sm << 1
    pxor                 m1, m1
    pshufb               m0, m1
    pshufb               m2, m1
    pcmpeqb              m1, m0, [base+z_filter_wh4]
    pand                 m1, m2
    pcmpgtb              m1, [base+z_filter_t_w48+angleq*8]
    pmovmskb            r5d, m1
    mov                 r3d, 7
    test                r5d, r5d
    jz .w4_main ; filter_strength == 0
    mova                 m3, [tlq-1]
    imul                r5d, 0x55555555
    movu                 m7, [base+z_filter_s+8]
    shr                 r5d, 30 ; filter_strength
    movddup              m0, [base+pb_8]
    pminub               m7, m0
    pshufb               m0, m3, [base+z_filter_s]
    movddup              m4, [base+z_filter_k-8+r5*8+24*0]
    pshufb               m3, m7
    movddup              m5, [base+z_filter_k-8+r5*8+24*1]
    shufps               m2, m0, m3, q2121
    movddup              m6, [base+z_filter_k-8+r5*8+24*2]
    pmaddubsw            m0, m4
    pmaddubsw            m1, m2, m4
    pmaddubsw            m2, m5
    paddd                m5, m6
    pmaddubsw            m4, m3, m5
    pmaddubsw            m3, m6
    paddw                m0, m2
    paddw                m1, m4
    paddw                m0, m3
    pshufd               m1, m1, q3333
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    mov                 r5d, 9
    mov                 tlq, rsp
    cmp                  hd, 4
    cmovne              r3d, r5d
    packuswb             m0, m1
    mova              [tlq], m0
.w4_main:
    add                 tlq, r3
    movd                 m5, dxd
    movddup              m0, [base+z_base_inc] ; base_inc << 6
    movd                 m7, [tlq] ; top[max_base_x]
    shl                 r3d, 6
    movd                 m4, r3d
    pshufb               m5, [base+pw_256]
    mov                 r5d, dxd ; xpos
    pshufb               m7, [base+pw_m256]
    sub                  r5, r3
    pshufb               m4, [base+pw_256]
    mova                 m3, [base+z1_shuf_w4]
    paddw                m6, m5, m5
    psubw                m4, m0 ; max_base_x
    punpcklqdq           m5, m6 ; xpos0 xpos1
.w4_loop:
    lea                  r3, [r5+dxq]
    sar                  r5, 6      ; base0
    movq                 m0, [tlq+r5]
    lea                  r5, [r3+dxq]
    sar                  r3, 6      ; base1
    movhps               m0, [tlq+r3]
    pand                 m2, m8, m5 ; frac
    psubw                m1, m9, m2 ; 64-frac
    psllw                m2, 8
    pshufb               m0, m3
    por                  m1, m2     ; 64-frac, frac
    pmaddubsw            m0, m1
    movifnidn       strideq, stridemp
    pcmpgtw              m1, m4, m5 ; base < max_base_x
    pmulhrsw             m0, m10
    paddw                m5, m6     ; xpos += dx
    pand                 m0, m1
    pandn                m1, m7
    por                  m0, m1
    packuswb             m0, m0
    movd   [dstq+strideq*0], m0
    pshuflw              m0, m0, q1032
    movd   [dstq+strideq*1], m0
    sub                  hd, 2
    jz .w4_end
    lea                dstq, [dstq+strideq*2]
    test                r5d, r5d
    jl .w4_loop
    packuswb             m7, m7
.w4_end_loop:
    movd   [dstq+strideq*0], m7
    movd   [dstq+strideq*1], m7
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w4_end_loop
.w4_end:
    RET
.w8:
    lea                 r3d, [angleq+88]
    and                 r3d, ~0x7f
    or                  r3d, hd
    cmp                 r3d, 8
    ja .w8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || h > 8
    mova                 m5, [base+z_upsample1]
    movu                 m3, [base+z_filter_s+6]
    movd                 m4, hd
    mova                 m0, [tlq-1]
    movu                 m1, [tlq+7]
    pxor                 m7, m7
    pshufb               m4, m7
    movddup              m7, [base+pb_36_m4]
    pminub               m4, m3
    add                 dxd, dxd
    pshufb               m2, m0, m5
    pmaddubsw            m2, m7
    pshufb               m0, m3
    pmaddubsw            m0, m7
    movd                 m6, dxd
    pshufb               m3, m1, m5
    pmaddubsw            m3, m7
    pshufb               m1, m4
    pmaddubsw            m1, m7
    pshufb               m6, [base+pw_256]
    mov                 r5d, dxd
    paddw                m2, m0
    paddw                m7, m6, m6
    paddw                m3, m1
    punpcklqdq           m6, m7 ; xpos0 xpos1
    movu                 m1, [tlq]
    pmulhrsw             m2, m10
    pmulhrsw             m3, m10
    packuswb             m2, m3
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    movifnidn       strideq, stridemp
    mova         [rsp+16*0], m0
    mova         [rsp+16*1], m1
.w8_upsample_loop:
    lea                 r2d, [r5+dxq]
    shr                 r5d, 6 ; base0
    movu                 m0, [rsp+r5]
    lea                 r5d, [r2+dxq]
    shr                 r2d, 6 ; base1
    movu                 m1, [rsp+r2]
    pand                 m2, m8, m6
    psubw                m3, m9, m2
    psllw                m2, 8
    por                  m3, m2
    punpcklqdq           m2, m3, m3 ; frac0
    pmaddubsw            m0, m2
    punpckhqdq           m3, m3     ; frac1
    pmaddubsw            m1, m3
    paddw                m6, m7
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packuswb             m0, m1
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_upsample_loop
    RET
.w8_no_upsample:
    lea                 r3d, [hq+7]
    movd                 m0, r3d
    and                 r3d, 7
    or                  r3d, 8 ; imin(h+7, 15)
    test             angled, 0x400
    jnz .w8_main
    movd                 m2, angled
    shr              angled, 8 ; is_sm << 1
    pxor                 m1, m1
    pshufb               m0, m1
    pshufb               m2, m1
    movu                 m1, [base+z_filter_wh8]
    psrldq               m3, [base+z_filter_t_w48+angleq*8], 4
    pcmpeqb              m1, m0
    pand                 m1, m2
    pcmpgtb              m1, m3
    pmovmskb            r5d, m1
    test                r5d, r5d
    jz .w8_main ; filter_strength == 0
    movd                 m3, [tlq-1]
    movu                 m0, [tlq+16*0]
    imul                r5d, 0x55555555
    movu                 m1, [tlq+16*1]
    shr                 r5d, 30 ; filter_strength
    movd                 m2, [tlq+r3]
    lea                 tlq, [rsp+16*4]
    sub                  r5, 3
    mova         [tlq-16*1], m0
    pxor                 m7, m7
    mova         [tlq+16*0], m1
    pshufb               m3, m7
    pshufb               m2, m7
    mova         [tlq-16*2], m3
    movq        [tlq+r3-15], m2
    call .filter_edge
    sar                 r5d, 1
    add                 r5d, 17
    cmp                  hd, 8
    cmova               r3d, r5d
.w8_main:
    add                 tlq, r3
    movd                 m5, dxd
    movd                 m7, [tlq]
    shl                 r3d, 6
    movu                 m3, [base+z_filter_s+2]
    movd                 m4, r3d
    pshufb               m5, [base+pw_256]
    mov                 r5d, dxd
    pshufb               m7, [base+pw_m256]
    sub                  r5, r3
    pshufb               m4, [base+pw_256]
    psubw                m4, [base+z_base_inc]
    mova                 m6, m5
.w8_loop:
    mov                  r3, r5
    sar                  r3, 6
    movu                 m0, [tlq+r3]
    pand                 m1, m8, m5
    psubw                m2, m9, m1
    psllw                m1, 8
    pshufb               m0, m3
    por                  m1, m2
    pmaddubsw            m0, m1
    pcmpgtw              m1, m4, m5
    paddw                m5, m6
    pmulhrsw             m0, m10
    pand                 m0, m1
    pandn                m1, m7
    por                  m0, m1
    packuswb             m0, m0
    movq             [dstq], m0
    dec                  hd
    jz .w8_end
    movifnidn       strideq, stridemp
    add                dstq, strideq
    add                  r5, dxq
    jl .w8_loop
    packuswb             m7, m7
.w8_end_loop:
    movq             [dstq], m7
    add                dstq, strideq
    dec                  hd
    jg .w8_end_loop
.w8_end:
    RET
.w16:
    lea                 r3d, [hq+15]
    movd                 m0, r3d
    and                 r3d, 15
    or                  r3d, 16 ; imin(h+15, 31)
    test             angled, 0x400
    jnz .w16_main
    movd                 m2, angled
    shr              angled, 8 ; is_sm << 1
    pxor                 m1, m1
    pshufb               m0, m1
    pshufb               m2, m1
    movq                 m3, [base+z_filter_t_w16+angleq*4]
    pcmpeqb              m0, [base+z_filter_wh16]
    pand                 m0, m2
    pcmpgtb              m0, m3
    pmovmskb            r5d, m0
    test                r5d, r5d
    jz .w16_main ; filter_strength == 0
    movd                 m4, [tlq-1]
    movu                 m0, [tlq+16*0]
    imul                r5d, 0x24924924
    movu                 m1, [tlq+16*1]
    shr                 r5d, 30
    movd                 m2, [tlq+30]
    adc                  r5, -4 ; filter_strength-3
    movd                 m3, [tlq+r3]
    lea                 tlq, [rsp+16*4]
    mova         [tlq-16*1], m0
    pxor                 m7, m7
    mova         [tlq+16*0], m1
    pshufb               m4, m7
    movd              [rsp], m2
    pshufb               m3, m7
    mova         [tlq-16*2], m4
    movd        [tlq+r3-16], m3
    call .filter_edge
    cmp                  hd, 16
    jle .w16_main
    pshuflw              m0, [rsp], q0000
    sar                  r5, 1
    movd                 m1, [base+z_filter_k_tail+4+r5*4]
    lea                 r3d, [r5+33]
    pmaddubsw            m0, m1
%if ARCH_X86_64
    pmulhrsw             m0, m10
%else
    pmulhrsw             m0, m4
%endif
    packuswb             m0, m0
    movd           [tlq+32], m0
.w16_main:
    add                 tlq, r3
    movd                 m5, dxd
    movd                 m7, [tlq]
    movd                 m4, r3d
    shl                 r3d, 6
    pshufb               m5, [base+pw_256]
    pxor                 m6, m6
    pshufb               m7, m6
    mov                 r5d, dxd
    pshufb               m4, m6
    sub                  r5, r3
    psubb                m4, [base+pb_0to15]
    mova                 m6, m5
.w16_loop:
    mov                  r3, r5
    sar                  r3, 6
    movu                 m1, [tlq+r3+0]
    pand                 m0, m8, m5
    movu                 m2, [tlq+r3+1]
    psubw                m3, m9, m0
    psllw                m0, 8
    por                  m3, m0
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m3
    punpckhbw            m1, m2
    pmaddubsw            m1, m3
    psrlw                m3, m5, 6
    packsswb             m3, m3
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    paddw                m5, m6
    pcmpgtb              m2, m4, m3
    packuswb             m0, m1
    pand                 m0, m2
    pandn                m2, m7
    por                  m0, m2
    mova             [dstq], m0
    dec                  hd
    jz .w16_end
    movifnidn       strideq, stridemp
    add                dstq, strideq
    add                  r5, dxq
    jl .w16_loop
.w16_end_loop:
    mova             [dstq], m7
    add                dstq, strideq
    dec                  hd
    jg .w16_end_loop
.w16_end:
    RET
.w32:
    lea                 r3d, [hq+31]
    and                 r3d, 31
    or                  r3d, 32    ; imin(h+31, 63)
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .w32_main
    movd                 m6, [tlq-1]
    movu                 m0, [tlq+16*0]
    movu                 m1, [tlq+16*1]
    movu                 m2, [tlq+16*2]
    movu                 m3, [tlq+16*3]
    movd                 m4, [tlq+62]
    movd                 m5, [tlq+r3]
    lea                 tlq, [rsp+16*6]
    mova         [tlq-16*3], m0
    pxor                 m7, m7
    mova         [tlq-16*2], m1
    pshufb               m6, m7
    mova         [tlq-16*1], m2
    xor                 r5d, r5d ; filter_strength = 3
    mova         [tlq+16*0], m3
    movd              [rsp], m4
    pshufb               m5, m7
    mova         [tlq-16*4], m6
    movd        [tlq+r3-48], m5
    call .filter_edge
    sub                 tlq, 16*2
    call .filter_edge
    cmp                  hd, 32
    jle .w32_main
    pshuflw              m0, [rsp], q0000
    movd                 m1, [base+z_filter_k_tail+4]
    add                 r3d, 2
    pmaddubsw            m0, m1
%if ARCH_X86_64
    pmulhrsw             m0, m10
%else
    pmulhrsw             m0, m4
%endif
    packuswb             m0, m0
    movd           [tlq+64], m0
.w32_main:
    add                 tlq, r3
    movd                 m0, r3d
    movd                 m7, [tlq]
    shl                 r3d, 6
    movd                 m5, dxd
    pxor                 m6, m6
    mov                 r5d, dxd
    pshufb               m0, m6
    pshufb               m5, [base+pw_256]
    sub                  r5, r3
    pshufb               m7, m6
    psubb                m0, [base+pb_0to15]
    movddup              m1, [base+pb_m16]
    mova         [rsp+16*0], m0
    paddb                m0, m1
    mova         [rsp+16*1], m0
    mova                 m6, m5
.w32_loop:
    mov                  r3, r5
    sar                  r3, 6
    movu                 m1, [tlq+r3+16*0+0]
    pand                 m0, m8, m5
    movu                 m2, [tlq+r3+16*0+1]
    psubw                m3, m9, m0
    psllw                m0, 8
    por                  m3, m0
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m3
    punpckhbw            m1, m2
    pmaddubsw            m1, m3
    psrlw                m4, m5, 6
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packsswb             m4, m4
    pcmpgtb              m2, [rsp+16*0], m4
    packuswb             m0, m1
    pand                 m0, m2
    pandn                m2, m7
    por                  m0, m2
    movu                 m1, [tlq+r3+16*1+0]
    movu                 m2, [tlq+r3+16*1+1]
    mova        [dstq+16*0], m0
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m3
    punpckhbw            m1, m2
    pmaddubsw            m1, m3
    paddw                m5, m6
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    pcmpgtb              m2, [rsp+16*1], m4
    packuswb             m0, m1
    pand                 m0, m2
    pandn                m2, m7
    por                  m0, m2
    mova        [dstq+16*1], m0
    dec                  hd
    jz .w32_end
    movifnidn       strideq, stridemp
    add                dstq, strideq
    add                  r5, dxq
    jl .w32_loop
.w32_end_loop:
    mova        [dstq+16*0], m7
    mova        [dstq+16*1], m7
    add                dstq, strideq
    dec                  hd
    jg .w32_end_loop
.w32_end:
    RET
.w64:
    lea                 r3d, [hq+63]
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .w64_main
    movd                 m4, [tlq-1]
    movu                 m0, [tlq+16*0]
    movu                 m1, [tlq+16*1]
    movu                 m2, [tlq+16*2]
    movu                 m3, [tlq+16*3]
    mova         [rsp+16*3], m0
    pxor                 m7, m7
    mova         [rsp+16*4], m1
    pshufb               m4, m7
    mova         [rsp+16*5], m2
    mova         [rsp+16*6], m3
    mova         [rsp+16*2], m4
    movu                 m0, [tlq+16*4]
    movu                 m1, [tlq+16*5]
    movu                 m2, [tlq+16*6]
    movu                 m3, [tlq+16*7]
    movd                 m4, [tlq+r3]
    lea                 tlq, [rsp+16*10]
    mova         [tlq-16*3], m0
    xor                 r5d, r5d ; filter_strength = 3
    mova         [tlq-16*2], m1
    pshufb               m4, m7
    mova         [tlq-16*1], m2
    mova         [tlq+16*0], m3
    movd      [tlq+r3-16*7], m4
    cmp                  hd, 64
    jl .w64_filter96 ; skip one call if the last 32 bytes aren't used
    call .filter_edge
.w64_filter96:
    sub                 tlq, 16*2
    call .filter_edge
    sub                 tlq, 16*2
    call .filter_edge
    sub                 tlq, 16*2
    call .filter_edge
.w64_main:
    add                 tlq, r3
    movd                 m0, r3d
    movd                 m7, [tlq]
    shl                 r3d, 6
    movd                 m5, dxd
    pxor                 m6, m6
    mov                 r5d, dxd
    pshufb               m0, m6
    sub                  r5, r3
    pshufb               m5, [base+pw_256]
    pshufb               m7, m6
    psubb                m0, [base+pb_0to15]
    movddup              m1, [base+pb_m16]
    mova         [rsp+16*0], m0
    paddb                m0, m1
    mova         [rsp+16*1], m0
    paddb                m0, m1
    mova         [rsp+16*2], m0
    paddb                m0, m1
    mova         [rsp+16*3], m0
    mova                 m6, m5
.w64_loop:
    mov                  r3, r5
    sar                  r3, 6
    movu                 m1, [tlq+r3+16*0+0]
    pand                 m0, m8, m5
    movu                 m2, [tlq+r3+16*0+1]
    psubw                m3, m9, m0
    psllw                m0, 8
    por                  m3, m0
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m3
    punpckhbw            m1, m2
    pmaddubsw            m1, m3
    psrlw                m4, m5, 6
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packsswb             m4, m4
    pcmpgtb              m2, [rsp+16*0], m4
    packuswb             m0, m1
    pand                 m0, m2
    pandn                m2, m7
    por                  m0, m2
    movu                 m1, [tlq+r3+16*1+0]
    movu                 m2, [tlq+r3+16*1+1]
    mova        [dstq+16*0], m0
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m3
    punpckhbw            m1, m2
    pmaddubsw            m1, m3
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    pcmpgtb              m2, [rsp+16*1], m4
    packuswb             m0, m1
    pand                 m0, m2
    pandn                m2, m7
    por                  m0, m2
    movu                 m1, [tlq+r3+16*2+0]
    movu                 m2, [tlq+r3+16*2+1]
    mova        [dstq+16*1], m0
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m3
    punpckhbw            m1, m2
    pmaddubsw            m1, m3
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    pcmpgtb              m2, [rsp+16*2], m4
    packuswb             m0, m1
    pand                 m0, m2
    pandn                m2, m7
    por                  m0, m2
    movu                 m1, [tlq+r3+16*3+0]
    movu                 m2, [tlq+r3+16*3+1]
    mova        [dstq+16*2], m0
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m3
    punpckhbw            m1, m2
    pmaddubsw            m1, m3
    paddw                m5, m6
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    pcmpgtb              m2, [rsp+16*3], m4
    packuswb             m0, m1
    pand                 m0, m2
    pandn                m2, m7
    por                  m0, m2
    mova        [dstq+16*3], m0
    dec                  hd
    jz .w64_end
    movifnidn       strideq, stridemp
    add                dstq, strideq
    add                  r5, dxq
    jl .w64_loop
.w64_end_loop:
    mova        [dstq+16*0], m7
    mova        [dstq+16*1], m7
    mova        [dstq+16*2], m7
    mova        [dstq+16*3], m7
    add                dstq, strideq
    dec                  hd
    jg .w64_end_loop
.w64_end:
    RET
ALIGN function_align
.filter_edge: ; 32 pixels/iteration
    movddup              m7, [base+z_filter_k+8*2+r5*8+24*0]
    movu                 m2, [tlq-18]
    movu                 m1, [tlq-17]
    movu                 m3, [tlq- 2]
    movu                 m4, [tlq- 1]
    punpcklbw            m0, m2, m1
    pmaddubsw            m0, m7
    punpckhbw            m2, m1
    pmaddubsw            m2, m7
    punpcklbw            m1, m3, m4
    pmaddubsw            m1, m7
    punpckhbw            m3, m4
    pmaddubsw            m3, m7
    movddup              m7, [base+z_filter_k+8*2+r5*8+24*1]
    mova                 m5, [tlq-16]
    movu                 m6, [tlq-15]
    punpcklbw            m4, m5, m6
    pmaddubsw            m4, m7
    punpckhbw            m5, m6
    pmaddubsw            m5, m7
    paddw                m0, m4
    paddw                m2, m5
    mova                 m5, [tlq+ 0]
    movu                 m6, [tlq+ 1]
    punpcklbw            m4, m5, m6
    pmaddubsw            m4, m7
    punpckhbw            m5, m6
    pmaddubsw            m5, m7
    paddw                m1, m4
    paddw                m3, m5
    test                r5d, r5d
    jnz .filter_end ; 3-tap
    movddup              m7, [base+z_filter_k+8*8]
    movu                 m5, [tlq-14]
    movu                 m6, [tlq+ 2]
    punpcklbw            m4, m5, m5
    pmaddubsw            m4, m7
    punpckhbw            m5, m5
    pmaddubsw            m5, m7
    paddw                m0, m4
    paddw                m2, m5
    punpcklbw            m5, m6, m6
    pmaddubsw            m5, m7
    punpckhbw            m6, m6
    pmaddubsw            m6, m7
    paddw                m1, m5
    paddw                m3, m6
.filter_end:
%if ARCH_X86_64
    REPX  {pmulhrsw x, m10}, m0, m2, m1, m3
%else
    mova                 m4, m10
    REPX  {pmulhrsw x, m4 }, m0, m2, m1, m3
%endif
    packuswb             m0, m2
    packuswb             m1, m3
    mova         [tlq+16*0], m0
    mova         [tlq+16*1], m1
    ret

%if ARCH_X86_64
cglobal ipred_z2_8bpc, 4, 12, 13, 16*16, dst, stride, tl, w, h, angle, dx, _, dy
    %define            base  r7-$$
    %define           maxwm  r6m
    %define           maxhm  r7m
    lea                  r7, [$$]
    mov                  hd, hm
    mova                 m8, [base+pw_62]
    mova                 m9, [base+pw_64]
    lea                 r9d, [wq-4]
    mova                m10, [base+pw_512]
    shl                 r9d, 6
    mova                m11, [base+z1_shuf_w4]
    or                  r9d, hd
    mova                m12, [base+z2_h_shuf]
%else
cglobal ipred_z2_8bpc, 4, 7, 8, -16*20, dst, _, tl, w, h, angle, dx
    %define            base  r1-$$
    %define              m8  [base+pw_62]
    %define              m9  [base+pw_64]
    %define             m10  [base+pw_512]
    %define             m11  [rsp+16*16]
    %define             m12  [rsp+16*17]
    %define             r9b  byte [rsp+16*18+4*0]
    %define             r9d  dword [rsp+16*18+4*0]
    %define            r10d  dword [rsp+16*18+4*1]
    %define            r11d  dword [rsp+16*18+4*2]
    %define           maxwm  [rsp+16*18+4*3]
    %define           maxhm  [rsp+16*19+4*0]
    %define        stridemp  [rsp+16*19+4*1]
    %define         strideq  r3
    %define             dyd  r4
    %define             dyq  r4
    mov            stridemp, r1
    mov                 r1d, r6m
    mov                 r4d, r7m
    mov               maxwm, r1d
    mov               maxhm, r4d
    LEA                  r1, $$
    lea                  hd, [wq-4]
    mova                 m0, [base+z1_shuf_w4]
    shl                  hd, 6
    mova                 m1, [base+z2_h_shuf]
    or                   hd, hm
    mova                m11, m0
    mov                 r9d, hd
    mova                m12, m1
%endif
    tzcnt                wd, wd
    movifnidn        angled, anglem
    movsxd               wq, [base+ipred_z2_ssse3_table+wq*4]
%if ARCH_X86_64
    movzx               dxd, angleb
%else
    movzx               dxd, byte anglem
%endif
    xor              angled, 0x400
    mova                 m0, [tlq-16*4]
    mov                 dyd, dxd
    mova                 m1, [tlq-16*3]
    neg                 dxq
    mova                 m2, [tlq-16*2]
    and                 dyd, ~1
    mova                 m3, [tlq-16*1]
    and                 dxq, ~1
    movd                 m4, [tlq]
    movu                 m5, [tlq+16*0+1]
    movu                 m6, [tlq+16*1+1]
    movzx               dyd, word [base+dr_intra_derivative+dyq-90]  ; angle - 90
    movzx               dxd, word [base+dr_intra_derivative+dxq+180] ; 180 - angle
    mova         [rsp+16*2], m0
    pxor                 m7, m7
    mova         [rsp+16*3], m1
    pshufb               m4, m7
    mova         [rsp+16*4], m2
    lea                  wq, [base+ipred_z2_ssse3_table+wq]
    mova         [rsp+16*5], m3
    neg                 dxd
    mova         [rsp+16*6], m4
    or                  dyd, 4<<16
    mova         [rsp+16*7], m4
    mova         [rsp+16*8], m5
    mova         [rsp+16*9], m6
    movq                 m0, [base+z_base_inc+2]
    movsldup             m1, [base+z2_dy_offset]
    movq                 m2, [base+pw_256] ; 4<<6
    movq    [rsp+16*14+8*0], m0
    movq    [rsp+16*15+8*0], m1
    movq    [rsp+16*15+8*1], m2
%if ARCH_X86_64
    lea                r10d, [dxq+(128<<6)] ; xpos
%else
    mov      [rsp+16*7+4*1], dyd
    lea                 r4d, [dxq+(128<<6)]
    mov                r10d, r4d
    movzx                hd, r9b
%endif
    mov                r11d, (128-4)<<6
    jmp                  wq
.w4:
    test             angled, 0x400
    jnz .w4_main
    movd                 m5, [tlq+4]
    lea                 r3d, [hq+2]
    add              angled, 1022
    pshufb               m5, m7
    shl                 r3d, 6
    movd       [rsp+16*8+4], m5
    test                r3d, angled
    jnz .w4_no_upsample_above ; angle >= 130 || h > 8 || (is_sm && h == 8)
    call .upsample_above
    sub              angled, 1075 ; angle - 53
    lea                 r3d, [hq+3]
    xor              angled, 0x7f ; 180 - angle
    movd                 m0, r3d
    movd                 m6, angled
    shr              angled, 8 ; is_sm << 1
    pshufb               m0, m7
    pshufb               m6, m7
    pcmpeqb              m0, [base+z_filter_wh4]
    pand                 m6, m0
    pcmpgtb              m6, [base+z_filter_t_w48+angleq*8]
    jmp .w8_filter_left
.upsample_above: ; w4/w8
    movq                 m3, [rsp+gprsize+16*8-2]
    movq                 m1, [rsp+gprsize+16*8-1]
    movq                 m0, [rsp+gprsize+16*8+0]
    movq                 m4, [rsp+gprsize+16*8+1]
    movddup              m5, [base+pb_36_m4]
    punpcklbw            m1, m3
    punpcklbw            m2, m0, m4
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
%if ARCH_X86_64
    mova                m11, [base+pb_0to15]
    lea                r10d, [r10+dxq+(1<<6)]
    mov                r11d, (128-7)<<6
%else
    mova                 m3, [base+pb_0to15]
    mov                 r3d, [rsp+gprsize+16*18+4*1]
    mov dword [rsp+gprsize+16*18+4*2], (128-7)<<6
    lea                 r3d, [r3+dxq+(1<<6)]
    mov [rsp+gprsize+16*18+4*1], r3d
    mova [rsp+gprsize+16*16], m3
%endif
    add                 dxd, dxd
    paddw                m1, m2
    pmulhrsw             m1, m10
    movq                 m2, [rsp+gprsize+16*14]
    paddw                m2, m2
    movq [rsp+gprsize+16*14], m2
    packuswb             m1, m1
    punpcklbw            m1, m0
    mova [rsp+gprsize+16*8], m1
    ret
.w4_no_upsample_above:
    lea                 r3d, [hq+3]
    mov               [rsp], angled
    sub              angled, 1112 ; angle - 90
    movd                 m0, r3d
    mov                 r3d, 90
    movd                 m1, angled
    sub                 r3d, angled ; 180 - angle
    shr              angled, 8 ; is_sm << 1
    movu                 m3, [base+z_filter_wh4]
    mova                 m4, [base+z_filter_t_w48+angleq*8]
    call .w8_filter_top
    mov              angled, [rsp]
    lea                 r3d, [hq+2]
    sub              angled, 139
    shl                 r3d, 6
    test                r3d, angled
    jnz .w8_filter_left ; angle <= 140 || h > 8 || (is_sm && h == 8)
.upsample_left: ; w4/w8
    neg                  hq
    movd                 m0, [tlq+hq]
    pshufb               m0, m7
    movd    [rsp+16*6+hq-4], m0
    movq                 m3, [rsp+16*5+7]
    movq                 m0, [rsp+16*5+8]
    movq                 m2, [rsp+16*5+9]
    movq                 m4, [rsp+16*5+10]
    movddup              m5, [base+pb_36_m4]
    punpcklbw            m1, m0, m3
    punpcklbw            m2, m4
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    movshdup             m3, [base+z2_dy_offset]
%if ARCH_X86_64
    mova                m12, [base+z2_upsample]
    add                 dyd, dyd
%else
    mova                 m4, [base+z2_upsample]
    shl dword [rsp+16*7+4*1], 1
    mova                m12, m4
%endif
    paddw                m1, m2
    pmulhrsw             m1, m10
    movq        [rsp+16*15], m3
    packuswb             m1, m1
    punpcklbw            m0, m1
    mova         [rsp+16*5], m0
.w4_main:
    movd                 m6, dxd
%if ARCH_X86_64
    movd                 m3, dyd
%else
    movd                 m3, [rsp+16*7+4*1]
%endif
    movddup              m0, [rsp+16*14+8*0]
    pshufb               m6, [base+pw_256]
    paddw                m7, m6, m6
    movq                 m5, [base+pw_m1to4]
    pshuflw              m4, m3, q0000
    punpcklqdq           m6, m7
    pmullw               m4, m5
    pshuflw              m3, m3, q1111
    paddw                m6, m0
    mov                 r2d, r10d
    pshuflw              m0, m4, q3333
    psubw                m4, [rsp+16*15]
    movq     [rsp+16*6+8*1], m3
    movq          [rsp+8*1], m0 ; dy*4
    mov                  r5, dstq
.w4_loop0:
    mova        [rsp+16*12], m6
    movq          [rsp+8*0], m4
    pand                 m0, m4, m8
    psraw                m4, 6
    psubw                m1, m9, m0
    psllw                m0, 8
    por                  m0, m1       ; 64-frac_y, frac_y
    movq          [rsp+8*3], m0
    pabsw                m4, m4
    movq          [rsp+8*2], m4
    movzx                hd, r9b
.w4_loop:
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6        ; base_x0
    movq                 m0, [rsp+r2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6        ; base_x1
    movhps               m0, [rsp+r3]
    lea                 r3d, [r2+dxq]
    shr                 r2d, 6        ; base_x2
    movq                 m1, [rsp+r2]
    lea                 r2d, [r3+dxq]
    shr                 r3d, 6        ; base_x3
    movhps               m1, [rsp+r3]
    pand                 m2, m8, m6
    paddsw               m5, m6, m7
    psubw                m3, m9, m2
    psllw                m2, 8
    pshufb               m0, m11
    por                  m2, m3
    pmaddubsw            m0, m2
    pand                 m2, m8, m5
    psubw                m3, m9, m2
    psllw                m2, 8
    pshufb               m1, m11
    por                  m2, m3
    pmaddubsw            m1, m2
    cmp                 r3d, 127 ; topleft
    jge .w4_toponly
    movzx               r3d, byte [rsp+8*2+0] ; base_y0
    movq                 m3, [rsp+r3]
    movzx               r3d, byte [rsp+8*2+2] ; base_y1
    movhps               m3, [rsp+r3]
    movzx               r3d, byte [rsp+8*2+4] ; base_y2
    movq                 m4, [rsp+r3]
    movzx               r3d, byte [rsp+8*2+6] ; base_y3
    movhps               m4, [rsp+r3]
    pshufb               m3, m12
    pshufb               m4, m12
    punpckldq            m2, m3, m4
    punpckhdq            m3, m4
    movddup              m4, [rsp+8*3]
    pmaddubsw            m2, m4
    pmaddubsw            m3, m4
    psraw                m6, 15       ; base_x < topleft
    pand                 m2, m6
    pandn                m6, m0
    por                  m0, m2, m6
    psraw                m6, m5, 15
    pand                 m3, m6
    pandn                m6, m1
    por                  m1, m3, m6
.w4_toponly:
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    movifnidn       strideq, stridemp
    packuswb             m0, m1
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    punpckhqdq           m0, m0
    movd   [dstq+strideq*0], m0
    psrlq                m0, 32
    movd   [dstq+strideq*1], m0
    sub                  hd, 4
    jz .w4_end
    movq                 m4, [rsp+8*2]
    movq                 m3, [rsp+16*6+8*1]
    paddw                m6, m5, m7   ; xpos += dx
    psubw                m4, m3
    movq          [rsp+8*2], m4
    lea                dstq, [dstq+strideq*2]
    cmp                 r2d, r11d
    jge .w4_loop
    movddup              m5, [rsp+8*3]
.w4_leftonly_loop:
    movzx               r2d, byte [rsp+8*2+0] ; base_y0
    movq                 m1, [rsp+r2]
    movzx               r2d, byte [rsp+8*2+2] ; base_y1
    movhps               m1, [rsp+r2]
    movzx               r2d, byte [rsp+8*2+4] ; base_y2
    movq                 m2, [rsp+r2]
    movzx               r2d, byte [rsp+8*2+6] ; base_y3
    movhps               m2, [rsp+r2]
    psubw                m4, m3
    pshufb               m1, m12
    pshufb               m2, m12
    movq          [rsp+8*2], m4
    punpckldq            m0, m1, m2
    punpckhdq            m1, m2
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packuswb             m0, m1
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    punpckhqdq           m0, m0
    movd   [dstq+strideq*0], m0
    psrlq                m0, 32
    movd   [dstq+strideq*1], m0
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 4
    jg .w4_leftonly_loop
.w4_end:
    sub                 r9d, 1<<8
    jl .w4_ret
    movq                 m4, [rsp+8*1]
    add                  r5, 4
    mov                dstq, r5
    paddw                m4, [rsp+8*0] ; base_y += 4*dy
    movzx               r2d, word [rsp+16*15+8*1]
    movddup              m6, [rsp+16*15+8*1]
    paddw                m6, [rsp+16*12] ; base_x += (4 << upsample_above)
    add                 r2d, r10d
    mov                r10d, r2d
    jmp .w4_loop0
.w4_ret:
    RET
.w8:
    test             angled, 0x400
    jnz .w4_main
    movd                 m5, [tlq+8]
    lea                 r3d, [angleq+126]
    pshufb               m5, m7
%if ARCH_X86_64
    mov                 r3b, hb
%else
    xor                 r3b, r3b
    or                  r3d, hd
%endif
    movd       [rsp+16*8+8], m5
    cmp                 r3d, 8
    ja .w8_no_upsample_above ; angle >= 130 || h > 8 || is_sm
    call .upsample_above
    sub              angled, 53
    lea                 r3d, [hq+7]
    xor              angled, 0x7f ; 180 - angle
    movu                 m1, [base+z_filter_wh8]
    movd                 m0, r3d
    movd                 m6, angled
    shr              angled, 8 ; is_sm << 1
    psrldq               m2, [base+z_filter_t_w48+angleq*8], 4
    pshufb               m0, m7
    pshufb               m6, m7
    pcmpeqb              m0, m1
    pand                 m6, m0
    pcmpgtb              m6, m2
%if ARCH_X86_64
    movq    [rsp+16*15+8*1], m10 ; 8<<6
%else
    movq                 m0, m10
    movq    [rsp+16*15+8*1], m0
%endif
    jmp .w8_filter_left
.w8_no_upsample_above:
    lea                 r3d, [hq+7]
    mov               [rsp], angled
    sub              angled, 90
    movd                 m0, r3d
    mov                 r3d, 90
    movd                 m1, angled
    sub                 r3d, angled ; 180 - angle
    shr              angled, 8 ; is_sm << 1
    movu                 m3, [base+z_filter_wh8]
    psrldq               m4, [base+z_filter_t_w48+angleq*8], 4
    call .w8_filter_top
    mov                 r3d, [rsp]
    sub                 r3d, 141
%if ARCH_X86_64
    mov                 r3b, hb
%else
    xor                 r3b, r3b
    or                  r3d, hd
%endif
    cmp                 r3d, 8
    jbe .upsample_left ; angle > 140 && h <= 8 && !is_sm
.w8_filter_left:
    pmovmskb            r5d, m6
    test                r5d, r5d
    jz .w4_main
    imul                r5d, 0x55555555
    mov                  r3, tlq
    shr                 r5d, 30
    sub                  r5, 3 ; filter_strength-3
    jmp .filter_left
.w8_filter_top:
    movd                 m6, r3d
    REPX     {pshufb x, m7}, m0, m1, m6
    pcmpeqb              m0, m3
    pand                 m1, m0
    pand                 m6, m0
    pcmpgtb              m1, m4
    pcmpgtb              m6, m4
    pmovmskb            r5d, m1
    test                r5d, r5d
    jz .w8_filter_top_end ; filter_strength == 0
    imul                r5d, 0x55555555
    movq                 m0, [rsp+gprsize+16*8-2]
    shr                 r5d, 30
    movq                 m1, [rsp+gprsize+16*8-1]
    sub                  r5, 3 ; filter_strength-3
    movddup              m7, [base+z_filter_k+8*2+r5*8+24*0]
    punpcklbw            m0, m1
    pmaddubsw            m0, m7
    movq                 m1, [rsp+gprsize+16*8+0]
    movq                 m2, [rsp+gprsize+16*8+1]
    movddup              m7, [base+z_filter_k+8*2+r5*8+24*1]
    punpcklbw            m1, m2
    pmaddubsw            m1, m7
    movq                 m2, [rsp+gprsize+16*8+2]
    movddup              m7, [base+z_filter_k+8*2+r5*8+24*2]
    punpcklbw            m2, m2
    pmaddubsw            m2, m7
    paddw                m0, m1
    paddw                m0, m2
%if ARCH_X86_64
    mov                 r3d, r7m ; maxw, offset due to call
%else
    mov                 r3d, [rsp+gprsize+16*18+4*3]
%endif
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packuswb             m0, m1
    movq [rsp+gprsize+16*8], m0
    cmp                 r3d, 8
    jge .w8_filter_top_end
    movq                 m0, [tlq+r3+1]
    movq [rsp+gprsize+r3+16*8], m0
.w8_filter_top_end:
    ret
.w16:
    test             angled, 0x400
    jnz .w4_main
    lea                 r3d, [hq+15]
    sub              angled, 90
    movd                 m0, r3d
    mov                 r3d, 90
    movd                 m1, angled
    sub                 r3d, angled ; 180 - angle
    shr              angled, 8 ; is_sm << 1
    movd                 m6, r3d
    REPX     {pshufb x, m7}, m0, m1, m6
    movq                 m3, [base+z_filter_t_w16+angleq*4]
    pcmpeqb              m0, [base+z_filter_wh16]
    pand                 m1, m0
    pand                 m6, m0
    pcmpgtb              m1, m3
    pcmpgtb              m6, m3
    pmovmskb            r5d, m1
    mov                  r3, tlq
    test                r5d, r5d
    jz .w16_filter_left ; filter_strength == 0
    imul                r5d, 0x24924924
    pshufb               m5, [base+z_filter_t_w16] ; tlq[16]
    shr                 r5d, 30
    adc                  r5, -4 ; filter_strength-3
    movd         [rsp+16*9], m5
    movddup              m7, [base+z_filter_k+8*2+r5*8+24*0]
    movu                 m1, [rsp+16*8-2]
    movu                 m2, [rsp+16*8-1]
    punpcklbw            m0, m1, m2
    pmaddubsw            m0, m7
    punpckhbw            m1, m2
    pmaddubsw            m1, m7
    movddup              m7, [base+z_filter_k+8*2+r5*8+24*1]
    mova                 m3, [rsp+16*8+0]
    movu                 m4, [rsp+16*8+1]
    punpcklbw            m2, m3, m4
    pmaddubsw            m2, m7
    punpckhbw            m3, m4
    pmaddubsw            m3, m7
    paddw                m0, m2
    paddw                m1, m3
    test                r5d, r5d
    jnz .w16_filter_end ; 3-tap
    movddup              m7, [base+z_filter_k+8*8]
    movu                 m3, [rsp+16*8+2]
    punpcklbw            m2, m3, m3
    pmaddubsw            m2, m7
    punpckhbw            m3, m3
    pmaddubsw            m3, m7
    paddw                m0, m2
    paddw                m1, m3
.w16_filter_end:
    mov                 r2d, maxwm
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packuswb             m0, m1
    mova         [rsp+16*8], m0
    cmp                 r2d, 16
    jge .w16_filter_left
    movu                 m0, [r3+r2+1]
    movu      [rsp+r2+16*8], m0
.w16_filter_left:
    pmovmskb            r5d, m6
    test                r5d, r5d
    jz .w4_main
    imul                r5d, 0x24924924
    shr                 r5d, 30
    adc                  r5, -4 ; filter_strength-3
    jmp .filter_left
.w32:
    test             angled, 0x400
    jnz .w4_main
    pshufb               m6, [base+z_filter_t_w16] ; tlq[32]
    mov                  r3, tlq
    lea                 tlq, [rsp+16*9]
    movd         [tlq+16*1], m6
    xor                 r5d, r5d ; filter_strength = 3
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    mova                 m0, [tlq+16*0]
    mova                 m1, [tlq+16*1]
    mov                 r2d, maxwm
    mova         [rsp+16*8], m0
    mova         [rsp+16*9], m1
    cmp                 r2d, 32
    jge .filter_left
    movu                 m0, [r3+r2+16*0+1]
    movu                 m1, [r3+r2+16*1+1]
    movu      [rsp+r2+16*8], m0
    movu      [rsp+r2+16*9], m1
    jmp .filter_left
.w64:
    movu                 m0, [tlq+16*2+1]
    movu                 m1, [tlq+16*3+1]
    mova        [rsp+16*10], m0
    mova        [rsp+16*11], m1
    test             angled, 0x400
    jnz .w4_main
    pshufb               m1, [base+z_filter_t_w16] ; tlq[64]
    mov                  r3, tlq
    lea                 tlq, [rsp+16*11]
    movd         [tlq+16*1], m1
    xor                 r5d, r5d ; filter_strength = 3
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    sub                 tlq, 16*2
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    mova                 m0, [tlq+16*0]
    mova                 m1, [tlq+16*1]
    mova                 m2, [tlq+16*2]
    mova                 m3, [tlq+16*3]
    mov                 r2d, maxwm
    mova        [rsp+16* 8], m0
    mova        [rsp+16* 9], m1
    mova        [rsp+16*10], m2
    mova        [rsp+16*11], m3
    cmp                 r2d, 64
    jge .filter_left
    movu                 m0, [r3+r2+16*0+1]
    movu                 m1, [r3+r2+16*1+1]
    movu     [rsp+r2+16* 8], m0
    movu     [rsp+r2+16* 9], m1
    cmp                 r2d, 32
    jge .filter_left
    movu                 m0, [r3+r2+16*2+1]
    movu                 m1, [r3+r2+16*3+1]
    movu     [rsp+r2+16*10], m0
    movu     [rsp+r2+16*11], m1
.filter_left:
    neg                  hq
    movd                 m0, [r3+hq]
    pxor                 m1, m1
    pshufb               m0, m1
    movd    [rsp+16*6+hq-4], m0
    lea                 tlq, [rsp+16*5]
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    cmp                  hd, -32
    jge .filter_left_end
    sub                 tlq, 16*2
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    mova                 m0, [tlq+16*0]
    mova                 m1, [tlq+16*1]
    mova         [rsp+16*2], m0
    mova         [rsp+16*3], m1
.filter_left_end:
    mov                 r2d, maxhm
    mova                 m0, [rsp+16*5]
    mova                 m1, [rsp+16*6]
    mova                 m2, [rsp+16*7]
    neg                  r2
    mova         [rsp+16*4], m0
    mova         [rsp+16*5], m1
    mova         [rsp+16*6], m2
    cmp                 r2d, hd
    jle .w4_main
    movu                 m0, [r3+r2-16*2]
    movu                 m1, [r3+r2-16*1]
    movu      [rsp+r2+16*4], m0
    movu      [rsp+r2+16*5], m1
    cmp                 r2d, -32
    jle .w4_main
    movu                 m0, [r3+r2-16*4]
    movu                 m1, [r3+r2-16*3]
    movu      [rsp+r2+16*2], m0
    movu      [rsp+r2+16*3], m1
    jmp .w4_main

%if ARCH_X86_64
cglobal ipred_z3_8bpc, 4, 9, 11, 16*10, dst, stride, tl, w, h, angle, dy, _, org_w
    %define            base  r7-$$
    lea                  r7, [$$]
    mova                 m8, [base+pw_62]
    mova                 m9, [base+pw_64]
    mova                m10, [base+pw_512]
    mov              org_wd, wd
%else
cglobal ipred_z3_8bpc, 4, 7, 8, -16*10, dst, stride, tl, w, h, angle, dy
    %define            base  r1-$$
    %define              m8  [base+pw_62]
    %define              m9  [base+pw_64]
    %define             m10  [base+pw_512]
    %define          org_wd  r5
    %define          org_wq  r5
    mov    [dstq+strideq*0], strideq
    mov    [dstq+strideq*1], wd
    LEA                  r1, $$
%endif
    tzcnt                hd, hm
    movifnidn        angled, anglem
    dec                 tlq
    movsxd               hq, [base+ipred_z3_ssse3_table+hq*4]
    sub              angled, 180
    mov                 dyd, angled
    neg                 dyd
    xor              angled, 0x400
    or                  dyq, ~0x7e
    lea                  hq, [base+ipred_z3_ssse3_table+hq]
    movzx               dyd, word [base+dr_intra_derivative+45*2-1+dyq]
    jmp                  hq
.h4:
    lea                 r4d, [angleq+88]
    test                r4d, 0x480
    jnz .h4_no_upsample ; !enable_intra_edge_filter || angle >= 40
    sar                 r4d, 9
    add                 r4d, wd
    cmp                 r4d, 8
    jg .h4_no_upsample ; w > 8 || (w == 8 && is_sm)
    movu                 m3, [tlq-7]
    movu                 m1, [base+z_upsample1-4]
    movu                 m4, [base+z_filter_s+2]
    pshufb               m0, m3, m1
    pxor                 m1, m1
    pshufb               m2, m3, m1
    pshufb               m1, m3, m4
    mova           [rsp+16], m2 ; top[max_base_y]
    movddup              m2, [base+pb_36_m4]
    add                 dyd, dyd
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    movd                 m5, dyd
    mov                 r5d, dyd
    pshufb               m5, [base+pw_256]
    paddw                m0, m1
    pmulhrsw             m0, m10
    shl                  wd, 2
    mov                 tlq, rsp
    sub                 rsp, wq
    packuswb             m0, m0
    punpcklbw            m0, m3
    paddw                m6, m5, m5
    punpcklqdq           m5, m6
    pshufb               m0, [base+pb_15to0]
    mova              [tlq], m0
.h4_upsample_loop:
    lea                 r4d, [r5+dyq]
    shr                 r5d, 6
    movq                 m0, [tlq+r5]
    lea                 r5d, [r4+dyq]
    shr                 r4d, 6
    movhps               m0, [tlq+r4]
    pand                 m2, m8, m5
    psubw                m1, m9, m2
    psllw                m2, 8
    por                  m1, m2
    pmaddubsw            m0, m1
    paddw                m5, m6
    pmulhrsw             m0, m10
    packuswb             m0, m0
    movq         [rsp+wq-8], m0
    sub                  wd, 8
    jg .h4_upsample_loop
    jmp .h4_transpose
.h4_no_upsample:
    mov                 r4d, 7
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .h4_main
    lea                 r4d, [wq+3]
    movd                 m0, r4d
    movd                 m2, angled
    shr              angled, 8 ; is_sm << 1
    pxor                 m1, m1
    pshufb               m0, m1
    pshufb               m2, m1
    pcmpeqb              m1, m0, [base+z_filter_wh4]
    pand                 m1, m2
    pcmpgtb              m1, [base+z_filter_t_w48+angleq*8]
    pmovmskb            r5d, m1
    mov                 r4d, 7
    test                r5d, r5d
    jz .h4_main ; filter_strength == 0
    movu                 m2, [tlq-7]
    imul                r5d, 0x55555555
    movu                 m3, [base+z_filter_s-2]
    shr                 r5d, 30 ; filter_strength
    mova                 m4, [base+z_upsample2]
    movddup              m5, [base+z_filter_k-8+r5*8+24*0]
    movddup              m6, [base+z_filter_k-8+r5*8+24*1]
    movddup              m7, [base+z_filter_k-8+r5*8+24*2]
    pshufb               m0, m2, m3
    shufps               m3, m4, q2121
    pmaddubsw            m1, m0, m5
    pmaddubsw            m0, m6
    pshufb               m5, m2, m3
    pmaddubsw            m3, m5, m6
    pmaddubsw            m5, m7
    pshufb               m2, m4
    pmaddubsw            m2, m7
    paddw                m0, m1
    paddw                m1, m3
    paddw                m0, m5
    paddw                m1, m2
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    lea                 r2d, [r4+2]
    cmp                  wd, 4
    cmovne              r4d, r2d
    pshufd               m0, m0, q0000
    lea                 tlq, [rsp+15]
    packuswb             m0, m1
    mova              [rsp], m0
.h4_main:
    movd                 m5, dyd
    movddup              m0, [base+z_base_inc] ; base_inc << 6
    sub                 tlq, r4
    shl                 r4d, 6
    movd                 m7, [tlq]
    movd                 m4, r4d
    pshufb               m5, [base+pw_256]
    neg                 dyq
    pshufb               m7, [base+pw_m256]
    mova                 m3, [base+z3_shuf_h4]
    lea                  r5, [dyq+r4+63] ; ypos
    pshufb               m4, [base+pw_256]
    psubw                m4, m0 ; max_base_y
    shl                  wd, 2
    paddw                m6, m5, m5
    sub                 rsp, wq
    punpcklqdq           m5, m6
.h4_loop:
    lea                  r4, [r5+dyq]
    sar                  r5, 6
    movq                 m0, [tlq+r5-4]
    lea                  r5, [r4+dyq]
    sar                  r4, 6
    movhps               m0, [tlq+r4-4]
    pand                 m2, m8, m5
    psubw                m1, m9, m2
    psllw                m2, 8
    pshufb               m0, m3
    por                  m1, m2
    pmaddubsw            m0, m1
    pcmpgtw              m1, m4, m5
    paddw                m5, m6
    pmulhrsw             m0, m10
    pand                 m0, m1
    pandn                m1, m7
    por                  m0, m1
    packuswb             m0, m0
    movq         [rsp+wq-8], m0
    sub                  wd, 8
    jz .h4_transpose
    test                r5d, r5d
    jg .h4_loop
    packuswb             m7, m7
.h4_end_loop:
    movq         [rsp+wq-8], m7
    sub                  wd, 8
    jg .h4_end_loop
.h4_transpose:
    mova                 m1, [base+z_transpose4]
%if ARCH_X86_32
    mov             strideq, [dstq]
    mov              org_wd, [dstq+strideq]
%endif
    lea                  r2, [strideq*3]
    lea                dstq, [dstq+org_wq-4]
.h4_transpose_loop:
    mova                 m0, [rsp]
    add                 rsp, 16
    pshufb               m0, m1
    movd   [dstq+strideq*0], m0
    pshuflw              m2, m0, q1032
    movd   [dstq+strideq*1], m2
    punpckhqdq           m0, m0
    movd   [dstq+strideq*2], m0
    psrlq                m0, 32
    movd   [dstq+r2       ], m0
    sub                dstq, 4
    sub              org_wd, 4
    jg .h4_transpose_loop
    RET
.h8:
    lea                 r4d, [angleq+88]
    and                 r4d, ~0x7f
    or                  r4d, wd
    cmp                 r4d, 8
    ja .h8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || w > 8
    mova                 m4, [tlq-15]
    and                 r4d, 4
    movu                 m3, [tlq- 9]
    movd                 m1, r4d
    movu                 m2, [base+z_filter_s+2]
    pxor                 m0, m0
    movu                 m5, [base+z_filter_s+6]
    movddup              m7, [base+pb_36_m4]
    pshufb               m1, m0 ; w & 4
    movu                 m0, [base+z_upsample1-4]
    pmaxub               m1, m0 ; clip 4x8
    add                 dyd, dyd
    pshufb               m0, m4, m1
    pmaddubsw            m0, m7
    pshufb               m1, m4, m2
    pmaddubsw            m1, m7
    pshufb               m2, m3, [base+z_upsample1]
    pmaddubsw            m2, m7
    pshufb               m3, m5
    pmaddubsw            m3, m7
    movd                 m5, dyd
    neg                 dyq
    paddw                m1, m0
    paddw                m2, m3
    pmulhrsw             m1, m10
    pmulhrsw             m2, m10
    shl                  wd, 3
    lea                 tlq, [rsp+16]
    pshufb               m5, [base+pw_256]
    sub                 rsp, wq
    packuswb             m1, m2
    lea                  r5, [dyq+63]
    punpcklbw            m0, m1, m4
    punpckhbw            m1, m4
    mova         [tlq-16*1], m0
    mova         [tlq-16*0], m1
    paddw                m6, m5, m5
    punpcklqdq           m5, m6
.h8_upsample_loop:
    lea                  r4, [r5+dyq]
    sar                  r5, 6
    movu                 m0, [tlq+r5]
    lea                  r5, [r4+dyq]
    sar                  r4, 6
    movu                 m1, [tlq+r4]
    pand                 m3, m8, m5
    psubw                m2, m9, m3
    psllw                m2, 8
    por                  m3, m2
    pshufd               m2, m3, q1010
    pmaddubsw            m0, m2
    punpckhqdq           m3, m3
    pmaddubsw            m1, m3
    paddw                m5, m6
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packuswb             m1, m0
    mova        [rsp+wq-16], m1
    sub                  wd, 16
    jg .h8_upsample_loop
    jmp .h8_transpose
.h8_no_upsample:
    lea                 r4d, [wq+7]
    movd                 m0, r4d
    and                 r4d, 7
    or                  r4d, 8 ; imin(w+7, 15)
    test             angled, 0x400
    jnz .h8_main
    movd                 m2, angled
    shr              angled, 8 ; is_sm << 1
    pxor                 m1, m1
    pshufb               m0, m1
    pshufb               m2, m1
    movu                 m1, [base+z_filter_wh8]
    psrldq               m3, [base+z_filter_t_w48+angleq*8], 4
    pcmpeqb              m1, m0
    pand                 m1, m2
    pcmpgtb              m1, m3
    pmovmskb            r5d, m1
    test                r5d, r5d
    jz .h8_main ; filter_strength == 0
    mova                 m0, [tlq-15]
    imul                r5d, 0x55555555
    movd                 m1, [tlq+1]
    neg                  r4
    movd                 m2, [tlq+r4]
    shr                 r5d, 30
    pxor                 m7, m7
    lea                 tlq, [rsp+16*2]
    sub                  r5, 3 ; filter_strength-3
    mova         [tlq+16*0], m0
    pshufb               m1, m7
    mova         [tlq+16*1], m1
    pshufb               m2, m7
    movq         [tlq+r4+8], m2
    neg                 r4d
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    sar                 r5d, 1
    add                 tlq, 31
    add                 r5d, 17
    cmp                  wd, 8
    cmova               r4d, r5d
.h8_main:
    movd                 m5, dyd
    sub                 tlq, r4
    shl                 r4d, 6
    movd                 m7, [tlq]
    movd                 m4, r4d
    pshufb               m5, [base+pw_256]
    neg                 dyq
    pshufb               m7, [base+pw_m256]
    mova                 m3, [base+z3_shuf]
    lea                  r5, [dyq+r4+63]
    pshufb               m4, [base+pw_256]
    psubw                m4, [base+z3_base_inc]
    shl                  wd, 3
    mova                 m6, m5
    sub                 rsp, wq
.h8_loop:
    mov                  r4, r5
    sar                  r4, 6
    movu                 m0, [tlq+r4-8]
    pand                 m2, m8, m5
    psubw                m1, m9, m2
    psllw                m2, 8
    pshufb               m0, m3
    por                  m1, m2
    pmaddubsw            m0, m1
    pcmpgtw              m1, m4, m5
    paddw                m5, m6
    pmulhrsw             m0, m10
    pand                 m0, m1
    pandn                m1, m7
    por                  m0, m1
    packuswb             m0, m0
    movq         [rsp+wq-8], m0
    sub                  wd, 8
    jz .h8_transpose
    add                  r5, dyq
    jg .h8_loop
    packuswb             m7, m7
.h8_end_loop:
    movq         [rsp+wq-8], m7
    sub                  wd, 8
    jg .h8_end_loop
.h8_transpose:
%if ARCH_X86_32
    mov             strideq, [dstq]
    mov              org_wd, [dstq+strideq]
%endif
    or                  r3d, 8
    cmp              org_wd, 4
%if ARCH_X86_64
    jne .end_transpose_main
%else
    jne .end_transpose_loop
%endif
    mova                 m1, [rsp+16*1]
    mova                 m0, [rsp+16*0]
    lea                  r2, [strideq*3]
    add                 rsp, 16*2
    punpcklbw            m2, m1, m0
    punpckhbw            m1, m0
    punpckhbw            m0, m1, m2
    punpcklbw            m1, m2
.write_4x8_end:
    call .write_4x8
    RET
.write_4x8:
    movd   [dstq+r2       ], m0
    pshuflw              m4, m0, q1032
    movd   [dstq+strideq*2], m4
    punpckhqdq           m0, m0
    movd   [dstq+strideq*1], m0
    psrlq                m0, 32
    movd   [dstq+strideq*0], m0
    lea                dstq, [dstq+strideq*4]
    movd   [dstq+r2       ], m1
    pshuflw              m4, m1, q1032
    movd   [dstq+strideq*2], m4
    punpckhqdq           m1, m1
    movd   [dstq+strideq*1], m1
    psrlq                m1, 32
    movd   [dstq+strideq*0], m1
    ret
.h16:
    lea                 r4d, [wq+15]
    movd                 m0, r4d
    and                 r4d, 15
    or                  r4d, 16 ; imin(w+15, 31)
    test             angled, 0x400
    jnz .h16_main
    movd                 m2, angled
    shr              angled, 8 ; is_sm << 1
    pxor                 m1, m1
    pshufb               m0, m1
    pshufb               m2, m1
    movq                 m3, [base+z_filter_t_w16+angleq*4]
    pcmpeqb              m1, m0, [base+z_filter_wh16]
    pand                 m1, m2
    pcmpgtb              m1, m3
    pmovmskb            r5d, m1
    test                r5d, r5d
    jz .h16_main ; filter_strength == 0
    mova                 m0, [tlq-16*2+1]
    imul                r5d, 0x24924924
    mova                 m1, [tlq-16*1+1]
    neg                  r4
    movd                 m2, [tlq-16*0+1]
    shr                 r5d, 30
    movd                 m3, [tlq+r4]
    adc                  r5, -4 ; filter_strength-3
    pxor                 m7, m7
    lea                 tlq, [rsp+16*2]
    mova         [tlq-16*1], m0
    pshufb               m2, m7
    mova         [tlq+16*0], m1
    pshufb               m3, m7
    mova         [tlq+16*1], m2
    movq         [tlq+r4+8], m3
    neg                 r4d
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    add                 tlq, 31
    cmp                  wd, 16
    jle .h16_main
    pshuflw              m0, [tlq-47], q0000
    sar                  r5, 1
    movq                 m1, [base+z3_filter_k_tail+r5*4]
    lea                 r4d, [r5+33]
    pmaddubsw            m0, m1
%if ARCH_X86_64
    pmulhrsw             m0, m10
%else
    pmulhrsw             m0, m4
%endif
    packuswb             m0, m0
    movd           [tlq-35], m0
.h16_main:
    movd                 m5, dyd
    sub                 tlq, r4
    movd                 m4, r4d
    shl                 r4d, 6
    movd                 m7, [tlq]
    pxor                 m6, m6
    pshufb               m5, [base+pw_256]
    neg                 dyq
    pshufb               m7, m6
    mova                 m3, [base+z3_shuf]
    lea                  r5, [dyq+r4+63]
    pshufb               m4, m6
    psubb                m4, [base+pb_15to0]
    shl                  wd, 4
    mova                 m6, m5
    sub                 rsp, wq
.h16_loop:
    mov                  r4, r5
    pand                 m2, m8, m5
    sar                  r4, 6
    psubw                m1, m9, m2
    psllw                m2, 8
    movu                 m0, [tlq+r4-8*2]
    por                  m2, m1
    movu                 m1, [tlq+r4-8*1]
    pshufb               m0, m3
    pmaddubsw            m0, m2
    pshufb               m1, m3
    pmaddubsw            m1, m2
    psrlw                m2, m5, 6
    paddw                m5, m6
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packsswb             m2, m2
    packuswb             m0, m1
    pcmpgtb              m1, m4, m2
    pand                 m0, m1
    pandn                m1, m7
    por                  m0, m1
    mova        [rsp+wq-16], m0
    sub                  wd, 16
    jz .h16_transpose
    add                  r5, dyq
    jg .h16_loop
.h16_end_loop:
    mova        [rsp+wq-16], m7
    sub                  wd, 16
    jg .h16_end_loop
.h16_transpose:
%if ARCH_X86_32
    mov             strideq, [dstq]
    mov              org_wd, [dstq+strideq]
%endif
    or                  r3d, 16
    cmp              org_wd, 4
%if ARCH_X86_64
    jne .end_transpose_main
%else
    jne .end_transpose_loop
%endif
.h16_transpose_w4:
    mova                 m2, [rsp+16*3]
    mova                 m4, [rsp+16*2]
    mova                 m3, [rsp+16*1]
    mova                 m0, [rsp+16*0]
    lea                  r2, [strideq*3]
    add                 rsp, 16*4
    punpckhbw            m1, m2, m4
    punpcklbw            m2, m4
    punpckhbw            m4, m3, m0
    punpcklbw            m3, m0
    punpckhwd            m0, m1, m4
    punpcklwd            m1, m4
    call .write_4x8
    lea                dstq, [dstq+strideq*4]
    punpckhwd            m0, m2, m3
    punpcklwd            m1, m2, m3
    jmp .write_4x8_end
.h32:
    lea                 r4d, [wq+31]
    and                 r4d, 31
    or                  r4d, 32 ; imin(w+31, 63)
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .h32_main
    mova                 m0, [tlq-16*4+1]
    mova                 m1, [tlq-16*3+1]
    mova                 m2, [tlq-16*2+1]
    mova                 m3, [tlq-16*1+1]
    movd                 m4, [tlq-16*0+1]
    neg                  r4
    movd                 m5, [tlq+r4]
    pxor                 m7, m7
    lea                 tlq, [rsp+16*4]
    mova         [tlq-16*3], m0
    mova         [tlq-16*2], m1
    xor                 r5d, r5d ; filter_strength = 3
    mova         [tlq-16*1], m2
    pshufb               m4, m7
    mova         [tlq+16*0], m3
    pshufb               m5, m7
    mova         [tlq+16*1], m4
    movq         [tlq+r4+8], m5
    neg                 r4d
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    sub                 tlq, 16*2
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    add                 tlq, 63
    cmp                  wd, 32
    jle .h32_main
    pshuflw              m0, [tlq-79], q0000
    movq                 m1, [base+z3_filter_k_tail]
    add                 r4d, 2
    pmaddubsw            m0, m1
%if ARCH_X86_64
    pmulhrsw             m0, m10
%else
    pmulhrsw             m0, m4
%endif
    packuswb             m0, m0
    movd           [tlq-67], m0
.h32_main:
    movd                 m5, dyd
    sub                 tlq, r4
    movd                 m4, r4d
    shl                 r4d, 6
    movd                 m7, [tlq]
    pxor                 m6, m6
    pshufb               m5, [base+pw_256]
    neg                 dyq
    pshufb               m7, m6
    mova                 m3, [base+z3_shuf]
    lea                  r5, [dyq+r4+63]
    pshufb               m4, m6
    psubb                m4, [base+pb_15to0]
    mova                 m6, m5
.h32_loop:
    mov                  r4, r5
    pand                 m2, m8, m5
    sar                  r4, 6
    psubw                m1, m9, m2
    psllw                m2, 8
    movu                 m0, [tlq+r4-8*4]
    por                  m2, m1
    movu                 m1, [tlq+r4-8*3]
    pshufb               m0, m3
    pmaddubsw            m0, m2
    pshufb               m1, m3
    pmaddubsw            m1, m2
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    sub                 rsp, 32
    packuswb             m0, m1
    mova         [rsp+16*0], m0
    movu                 m0, [tlq+r4-8*2]
    movu                 m1, [tlq+r4-8*1]
    pshufb               m0, m3
    pshufb               m1, m3
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    psrlw                m2, m5, 6
    paddw                m5, m6
    packsswb             m2, m2
    packuswb             m0, m1
    pcmpgtb              m1, m4, m2
    paddsb               m2, [base+pb_16]
    pand                 m0, m1
    pandn                m1, m7
    por                  m0, m1
    pcmpgtb              m1, m4, m2
    mova         [rsp+16*1], m0
    pand                 m0, m1, [rsp+16*0]
    pandn                m1, m7
    por                  m0, m1
    mova         [rsp+16*0], m0
    dec                  wd
    jz .h32_transpose
    add                  r5, dyq
    jg .h32_loop
.h32_end_loop:
    sub                 rsp, 32
    mova         [rsp+16*1], m7
    mova         [rsp+16*0], m7
    dec                  wd
    jg .h32_end_loop
.h32_transpose:
    or                  r3d, 32
    jmp .end_transpose_main
.h64:
    lea                 r4d, [wq+63]
    test             angled, 0x400 ; !enable_intra_edge_filter
    jnz .h64_main
    mova                 m0, [tlq-16*8+1]
    mova                 m1, [tlq-16*7+1]
    mova                 m2, [tlq-16*6+1]
    mova                 m3, [tlq-16*5+1]
    mova         [rsp+16*1], m0
    mova         [rsp+16*2], m1
    mova         [rsp+16*3], m2
    mova         [rsp+16*4], m3
    mova                 m0, [tlq-16*4+1]
    mova                 m1, [tlq-16*3+1]
    mova                 m2, [tlq-16*2+1]
    mova                 m3, [tlq-16*1+1]
    movd                 m4, [tlq-16*0+1]
    neg                  r4
    movd                 m5, [tlq+r4]
    pxor                 m7, m7
    lea                 tlq, [rsp+16*8]
    mova         [tlq-16*3], m0
    mova         [tlq-16*2], m1
    xor                 r5d, r5d ; filter_strength = 3
    mova         [tlq-16*1], m2
    pshufb               m4, m7
    mova         [tlq+16*0], m3
    pshufb               m5, m7
    mova         [tlq+16*1], m4
    movq         [tlq+r4+8], m5
    neg                 r4d
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    sub                 tlq, 16*2
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    sub                 tlq, 16*2
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
    sub                 tlq, 16*2
    cmp                  wd, 64
    jl .h64_filter96 ; skip one call if the last 32 bytes aren't used
    call mangle(private_prefix %+ _ipred_z1_8bpc_ssse3).filter_edge
.h64_filter96:
    add                 tlq, 127
.h64_main:
    movd                 m5, dyd
    sub                 tlq, r4
    movd                 m4, r4d
    shl                 r4d, 6
    movd                 m7, [tlq]
    pxor                 m6, m6
    pshufb               m5, [base+pw_256]
    neg                 dyq
    pshufb               m7, m6
    mova                 m3, [base+z3_shuf]
    lea                  r5, [dyq+r4+63]
    pshufb               m4, m6
    psubb                m4, [base+pb_15to0]
    mova                 m6, m5
.h64_loop:
    mov                  r4, r5
    pand                 m2, m8, m5
    sar                  r4, 6
    psubw                m1, m9, m2
    psllw                m2, 8
    movu                 m0, [tlq+r4-8*8]
    por                  m2, m1
    movu                 m1, [tlq+r4-8*7]
    pshufb               m0, m3
    pmaddubsw            m0, m2
    pshufb               m1, m3
    pmaddubsw            m1, m2
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    sub                 rsp, 64
    packuswb             m0, m1
    mova         [rsp+16*0], m0
    movu                 m0, [tlq+r4-8*6]
    movu                 m1, [tlq+r4-8*5]
    pshufb               m0, m3
    pshufb               m1, m3
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packuswb             m0, m1
    mova         [rsp+16*1], m0
    movu                 m0, [tlq+r4-8*4]
    movu                 m1, [tlq+r4-8*3]
    pshufb               m0, m3
    pshufb               m1, m3
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    packuswb             m0, m1
    mova         [rsp+16*2], m0
    movu                 m0, [tlq+r4-8*2]
    movu                 m1, [tlq+r4-8*1]
    pshufb               m0, m3
    pshufb               m1, m3
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    pmulhrsw             m0, m10
    pmulhrsw             m1, m10
    psrlw                m2, m5, 6
    paddw                m5, m6
    packsswb             m2, m2
    packuswb             m0, m1
    pcmpgtb              m1, m4, m2
    paddsb               m2, [base+pb_16]
    pand                 m0, m1
    pandn                m1, m7
    por                  m0, m1
    pcmpgtb              m1, m4, m2
    paddsb               m2, [base+pb_16]
    mova         [rsp+16*3], m0
    pand                 m0, m1, [rsp+16*2]
    pandn                m1, m7
    por                  m0, m1
    pcmpgtb              m1, m4, m2
    paddsb               m2, [base+pb_16]
    mova         [rsp+16*2], m0
    pand                 m0, m1, [rsp+16*1]
    pandn                m1, m7
    por                  m0, m1
    pcmpgtb              m1, m4, m2
    mova         [rsp+16*1], m0
    pand                 m0, m1, [rsp+16*0]
    pandn                m1, m7
    por                  m0, m1
    mova         [rsp+16*0], m0
    dec                  wd
    jz .h64_transpose
    add                  r5, dyq
    jg .h64_loop
.h64_end_loop:
    sub                 rsp, 64
    mova         [rsp+16*3], m7
    mova         [rsp+16*2], m7
    mova         [rsp+16*1], m7
    mova         [rsp+16*0], m7
    dec                  wd
    jg .h64_end_loop
.h64_transpose:
    or                  r3d, 64
.end_transpose_main:
%if ARCH_X86_64
    lea                  r5, [r3*3]
    lea                  r7, [strideq*3]
%else
    mov             strideq, [dstq]
    mov              org_wd, [dstq+strideq]
%endif
.end_transpose_loop:
    lea                  r4, [rsp+r3-8]
    lea                  r6, [dstq+org_wq-8]
.end_transpose_loop_y:
    movq                 m0, [r4+r3*1]
    movq                 m4, [r4+r3*0]
%if ARCH_X86_64
    movq                 m1, [r4+r5  ]
    movq                 m5, [r4+r3*2]
    lea                  r2, [r4+r3*4]
%else
    lea                  r2, [r4+r3*2]
    movq                 m1, [r2+r3*1]
    movq                 m5, [r2+r3*0]
    lea                  r2, [r2+r3*2]
%endif
    movq                 m2, [r2+r3*1]
    movq                 m6, [r2+r3*0]
%if ARCH_X86_64
    movq                 m3, [r2+r5  ]
    movq                 m7, [r2+r3*2]
%else
    lea                  r2, [r2+r3*2]
    movq                 m3, [r2+r3*1]
    movq                 m7, [r2+r3*0]
%endif
    sub                  r4, 8
    punpcklbw            m0, m4
    punpcklbw            m1, m5
    punpcklbw            m2, m6
    punpcklbw            m3, m7
    punpckhwd            m4, m1, m0
    punpcklwd            m1, m0
    punpckhwd            m0, m3, m2
    punpcklwd            m3, m2
    punpckhdq            m2, m3, m1
    punpckldq            m3, m1
    punpckldq            m1, m0, m4
    punpckhdq            m0, m4
    movhps   [r6+strideq*0], m0
    movq     [r6+strideq*1], m0
%if ARCH_X86_64
    movhps   [r6+strideq*2], m1
    movq     [r6+r7       ], m1
    lea                  r6, [r6+strideq*4]
%else
    lea                  r6, [r6+strideq*2]
    movhps   [r6+strideq*0], m1
    movq     [r6+strideq*1], m1
    lea                  r6, [r6+strideq*2]
%endif
    movhps   [r6+strideq*0], m2
    movq     [r6+strideq*1], m2
%if ARCH_X86_64
    movhps   [r6+strideq*2], m3
    movq     [r6+r7       ], m3
    lea                  r6, [r6+strideq*4]
%else
    lea                  r6, [r6+strideq*2]
    movhps   [r6+strideq*0], m3
    movq     [r6+strideq*1], m3
    lea                  r6, [r6+strideq*2]
%endif
    cmp                  r4, rsp
    jae .end_transpose_loop_y
    lea                 rsp, [rsp+r3*8]
    sub              org_wd, 8
    jg .end_transpose_loop
    RET

;---------------------------------------------------------------------------------------
;int dav1d_pal_pred_ssse3(pixel *dst, const ptrdiff_t stride, const uint16_t *const pal,
;                                         const uint8_t *idx, const int w, const int h);
;---------------------------------------------------------------------------------------
cglobal pal_pred_8bpc, 4, 6, 5, dst, stride, pal, idx, w, h
    mova                 m4, [palq]
    LEA                  r2, pal_pred_ssse3_table
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r2+wq*4]
    packuswb             m4, m4
    add                  wq, r2
    lea                  r2, [strideq*3]
    jmp                  wq
.w4:
    pshufb               m0, m4, [idxq]
    add                idxq, 16
    movd   [dstq          ], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq  ], m1
    punpckhqdq           m0, m0
    movd   [dstq+strideq*2], m0
    psrlq                m0, 32
    movd   [dstq+r2       ], m0
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4
    RET
ALIGN function_align
.w8:
    pshufb               m0, m4, [idxq]
    pshufb               m1, m4, [idxq+16]
    add                idxq, 32
    movq   [dstq          ], m0
    movhps [dstq+strideq  ], m0
    movq   [dstq+strideq*2], m1
    movhps [dstq+r2       ], m1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8
    RET
ALIGN function_align
.w16:
    pshufb               m0, m4, [idxq]
    pshufb               m1, m4, [idxq+16]
    pshufb               m2, m4, [idxq+32]
    pshufb               m3, m4, [idxq+48]
    add                idxq, 64
    mova   [dstq          ], m0
    mova   [dstq+strideq  ], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+r2       ], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16
    RET
ALIGN function_align
.w32:
    pshufb               m0, m4, [idxq]
    pshufb               m1, m4, [idxq+16]
    pshufb               m2, m4, [idxq+32]
    pshufb               m3, m4, [idxq+48]
    add                idxq, 64
    mova  [dstq           ], m0
    mova  [dstq+16        ], m1
    mova  [dstq+strideq   ], m2
    mova  [dstq+strideq+16], m3
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32
    RET
ALIGN function_align
.w64:
    pshufb               m0, m4, [idxq]
    pshufb               m1, m4, [idxq+16]
    pshufb               m2, m4, [idxq+32]
    pshufb               m3, m4, [idxq+48]
    add                idxq, 64
    mova          [dstq   ], m0
    mova          [dstq+16], m1
    mova          [dstq+32], m2
    mova          [dstq+48], m3
    add                dstq, strideq
    sub                  hd, 1
    jg .w64
    RET

;---------------------------------------------------------------------------------------
;void dav1d_ipred_cfl_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                           const int width, const int height, const int16_t *ac, const int alpha);
;---------------------------------------------------------------------------------------
%macro IPRED_CFL 1                   ; ac in, unpacked pixels out
    psignw               m3, m%1, m1
    pabsw               m%1, m%1
    pmulhrsw            m%1, m2
    psignw              m%1, m3
    paddw               m%1, m0
%endmacro

%if UNIX64
DECLARE_REG_TMP 7
%else
DECLARE_REG_TMP 5
%endif

cglobal ipred_cfl_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    movifnidn            wd, wm
    movifnidn            hd, hm
    tzcnt               r6d, hd
    lea                 t0d, [wq+hq]
    movd                 m4, t0d
    tzcnt               t0d, t0d
    movd                 m5, t0d
    LEA                  t0, ipred_cfl_ssse3_table
    tzcnt                wd, wd
    movsxd               r6, [t0+r6*4]
    movsxd               wq, [t0+wq*4+16]
    pcmpeqd              m3, m3
    psrlw                m4, 1
    add                  r6, t0
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6
.h4:
    movd                 m0, [tlq-4]
    pmaddubsw            m0, m3
    jmp                  wq
.w4:
    movd                 m1, [tlq+1]
    pmaddubsw            m1, m3
    psubw                m0, m4
    paddw                m0, m1
    pmaddwd              m0, m3
    cmp                  hd, 4
    jg .w4_mul
    psrlw                m0, 3                             ; dc >>= ctz(width + height);
    jmp .w4_end
.w4_mul:
    punpckhqdq           m1, m0, m0
    paddw                m0, m1
    pshuflw              m1, m0, q1032                     ; psrlq                m1, m0, 32
    paddw                m0, m1
    psrlw                m0, 2
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    test                 hd, 8
    cmovz               r6d, r2d
    movd                 m5, r6d
    pmulhuw              m0, m5
.w4_end:
    pshuflw              m0, m0, q0000
    punpcklqdq           m0, m0
.s4:
    movd                 m1, alpham
    pshuflw              m1, m1, q0000
    punpcklqdq           m1, m1
    lea                  r6, [strideq*3]
    pabsw                m2, m1
    psllw                m2, 9
.s4_loop:
    mova                 m4, [acq]
    mova                 m5, [acq+16]
    IPRED_CFL             4
    IPRED_CFL             5
    packuswb             m4, m5
    movd   [dstq+strideq*0], m4
    pshuflw              m4, m4, q1032
    movd   [dstq+strideq*1], m4
    punpckhqdq           m4, m4
    movd   [dstq+strideq*2], m4
    psrlq                m4, 32
    movd   [dstq+r6       ], m4
    lea                dstq, [dstq+strideq*4]
    add                 acq, 32
    sub                  hd, 4
    jg .s4_loop
    RET
ALIGN function_align
.h8:
    movq                 m0, [tlq-8]
    pmaddubsw            m0, m3
    jmp                  wq
.w8:
    movq                 m1, [tlq+1]
    pmaddubsw            m1, m3
    psubw                m4, m0
    punpckhqdq           m0, m0
    psubw                m0, m4
    paddw                m0, m1
    pshuflw              m1, m0, q1032                  ; psrlq  m1, m0, 32
    paddw                m0, m1
    pmaddwd              m0, m3
    psrlw                m0, m5
    cmp                  hd, 8
    je .w8_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    cmp                  hd, 32
    cmovz               r6d, r2d
    movd                 m1, r6d
    pmulhuw              m0, m1
.w8_end:
    pshuflw              m0, m0, q0000
    punpcklqdq           m0, m0
.s8:
    movd                 m1, alpham
    pshuflw              m1, m1, q0000
    punpcklqdq           m1, m1
    lea                  r6, [strideq*3]
    pabsw                m2, m1
    psllw                m2, 9
.s8_loop:
    mova                 m4, [acq]
    mova                 m5, [acq+16]
    IPRED_CFL             4
    IPRED_CFL             5
    packuswb             m4, m5
    movq   [dstq          ], m4
    movhps [dstq+strideq  ], m4
    mova                 m4, [acq+32]
    mova                 m5, [acq+48]
    IPRED_CFL             4
    IPRED_CFL             5
    packuswb             m4, m5
    movq   [dstq+strideq*2], m4
    movhps [dstq+r6       ], m4
    lea                dstq, [dstq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .s8_loop
    RET
ALIGN function_align
.h16:
    mova                 m0, [tlq-16]
    pmaddubsw            m0, m3
    jmp                  wq
.w16:
    movu                 m1, [tlq+1]
    pmaddubsw            m1, m3
    paddw                m0, m1
    psubw                m4, m0
    punpckhqdq           m0, m0
    psubw                m0, m4
    pshuflw              m1, m0, q1032                  ; psrlq  m1, m0, 32
    paddw                m0, m1
    pmaddwd              m0, m3
    psrlw                m0, m5
    cmp                  hd, 16
    je .w16_end
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    test                 hd, 8|32
    cmovz               r6d, r2d
    movd                 m1, r6d
    pmulhuw              m0, m1
.w16_end:
    pshuflw              m0, m0, q0000
    punpcklqdq           m0, m0
.s16:
    movd                 m1, alpham
    pshuflw              m1, m1, q0000
    punpcklqdq           m1, m1
    pabsw                m2, m1
    psllw                m2, 9
.s16_loop:
    mova                 m4, [acq]
    mova                 m5, [acq+16]
    IPRED_CFL             4
    IPRED_CFL             5
    packuswb             m4, m5
    mova             [dstq], m4
    mova                 m4, [acq+32]
    mova                 m5, [acq+48]
    IPRED_CFL             4
    IPRED_CFL             5
    packuswb             m4, m5
    mova     [dstq+strideq], m4
    lea                dstq, [dstq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .s16_loop
    RET
ALIGN function_align
.h32:
    mova                 m0, [tlq-32]
    pmaddubsw            m0, m3
    mova                 m2, [tlq-16]
    pmaddubsw            m2, m3
    paddw                m0, m2
    jmp                  wq
.w32:
    movu                 m1, [tlq+1]
    pmaddubsw            m1, m3
    movu                 m2, [tlq+17]
    pmaddubsw            m2, m3
    paddw                m1, m2
    paddw                m0, m1
    psubw                m4, m0
    punpckhqdq           m0, m0
    psubw                m0, m4
    pshuflw              m1, m0, q1032                   ; psrlq  m1, m0, 32
    paddw                m0, m1
    pmaddwd              m0, m3
    psrlw                m0, m5
    cmp                  hd, 32
    je .w32_end
    lea                 r2d, [hq*2]
    mov                 r6d, 0x5556
    mov                 r2d, 0x3334
    test                 hd, 64|16
    cmovz               r6d, r2d
    movd                 m1, r6d
    pmulhuw              m0, m1
.w32_end:
    pshuflw              m0, m0, q0000
    punpcklqdq           m0, m0
.s32:
    movd                 m1, alpham
    pshuflw              m1, m1, q0000
    punpcklqdq           m1, m1
    pabsw                m2, m1
    psllw                m2, 9
.s32_loop:
    mova                 m4, [acq]
    mova                 m5, [acq+16]
    IPRED_CFL             4
    IPRED_CFL             5
    packuswb             m4, m5
    mova             [dstq], m4
    mova                 m4, [acq+32]
    mova                 m5, [acq+48]
    IPRED_CFL             4
    IPRED_CFL             5
    packuswb             m4, m5
    mova          [dstq+16], m4
    add                dstq, strideq
    add                 acq, 64
    dec                  hd
    jg .s32_loop
    RET

;---------------------------------------------------------------------------------------
;void dav1d_ipred_cfl_left_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                           const int width, const int height, const int16_t *ac, const int alpha);
;---------------------------------------------------------------------------------------
cglobal ipred_cfl_left_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    mov                  hd, hm                                 ; zero upper half
    tzcnt               r6d, hd
    sub                 tlq, hq
    tzcnt                wd, wm
    movu                 m0, [tlq]
    mov                 t0d, 0x8000
    movd                 m3, t0d
    movd                 m2, r6d
    psrld                m3, m2
    LEA                  t0, ipred_cfl_left_ssse3_table
    movsxd               r6, [t0+r6*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, t0
    add                  t0, ipred_cfl_splat_ssse3_table-ipred_cfl_left_ssse3_table
    movsxd               wq, [t0+wq*4]
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6
.h32:
    movu                 m1, [tlq+16]                           ; unaligned when jumping here from dc_top
    pmaddubsw            m1, m2
    paddw                m0, m1
.h16:
    pshufd               m1, m0, q3232                          ; psrlq               m1, m0, 16
    paddw                m0, m1
.h8:
    pshuflw              m1, m0, q1032                          ; psrlq               m1, m0, 32
    paddw                m0, m1
.h4:
    pmaddwd              m0, m2
    pmulhrsw             m0, m3
    pshuflw              m0, m0, q0000
    punpcklqdq           m0, m0
    jmp                  wq

;---------------------------------------------------------------------------------------
;void dav1d_ipred_cfl_top_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                           const int width, const int height, const int16_t *ac, const int alpha);
;---------------------------------------------------------------------------------------
cglobal ipred_cfl_top_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    LEA                  t0, ipred_cfl_left_ssse3_table
    tzcnt                wd, wm
    inc                 tlq
    movu                 m0, [tlq]
    movifnidn            hd, hm
    mov                 r6d, 0x8000
    movd                 m3, r6d
    movd                 m2, wd
    psrld                m3, m2
    movsxd               r6, [t0+wq*4]
    pcmpeqd              m2, m2
    pmaddubsw            m0, m2
    add                  r6, t0
    add                  t0, ipred_cfl_splat_ssse3_table-ipred_cfl_left_ssse3_table
    movsxd               wq, [t0+wq*4]
    add                  wq, t0
    movifnidn           acq, acmp
    jmp                  r6

;---------------------------------------------------------------------------------------
;void dav1d_ipred_cfl_128_ssse3(pixel *dst, const ptrdiff_t stride, const pixel *const topleft,
;                           const int width, const int height, const int16_t *ac, const int alpha);
;---------------------------------------------------------------------------------------
cglobal ipred_cfl_128_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha
    tzcnt                wd, wm
    movifnidn            hd, hm
    LEA                  r6, ipred_cfl_splat_ssse3_table
    movsxd               wq, [r6+wq*4]
    movddup              m0, [r6-ipred_cfl_splat_ssse3_table+pw_128]
    add                  wq, r6
    movifnidn           acq, acmp
    jmp                  wq

%macro RELOAD_ACQ_32 1
    mov                 acq, ac_bakq       ; restore acq
%endmacro

%if ARCH_X86_64
cglobal ipred_cfl_ac_420_8bpc, 4, 8, 7, ac, y, stride, wpad, hpad, w, h, ac_bak
DECLARE_REG_TMP 7
    movddup              m2, [pb_2]
%else
cglobal ipred_cfl_ac_420_8bpc, 4, 7, 7, ac, y, stride, wpad, hpad, w, h
DECLARE_REG_TMP 4
%define ac_bakq acmp
    mov                 t0d, 0x02020202
    movd                 m2, t0d
    pshufd               m2, m2, q0000
%endif
    movifnidn            wd, wm
    mov                 t0d, hm
    mov                  hd, t0d
    imul                t0d, wd
    movd                 m5, t0d
    movifnidn         hpadd, hpadm
%if ARCH_X86_64
    mov             ac_bakq, acq
%endif
    shl               hpadd, 2
    sub                  hd, hpadd
    pxor                 m4, m4
    cmp                  wd, 8
    jg .w16
    je .w8
    ; fall-through
%if ARCH_X86_64
    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h, ac_bak
%else
    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h
%endif
.w4:
    lea            stride3q, [strideq*3]
.w4_loop:
    movq                 m0, [yq]
    movq                 m1, [yq+strideq]
    movhps               m0, [yq+strideq*2]
    movhps               m1, [yq+stride3q]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    mova              [acq], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*4]
    add                 acq, 16
    sub                  hd, 2
    jg .w4_loop
    test              hpadd, hpadd
    jz .calc_avg_4_8
    punpckhqdq           m0, m0
.w4_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 16
    sub               hpadd, 2
    jg .w4_hpad_loop
    jmp .calc_avg_4_8
.w8:
    lea            stride3q, [strideq*3]
    test              wpadd, wpadd
    jnz .w8_wpad
.w8_loop:
    mova                 m0, [yq]
    mova                 m1, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    mova              [acq], m0
    paddw                m4, m0
    mova                 m0, [yq+strideq*2]
    mova                 m1, [yq+stride3q]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    mova           [acq+16], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*4]
    add                 acq, 32
    sub                  hd, 2
    jg .w8_loop
    test              hpadd, hpadd
    jz .calc_avg_4_8
    jmp .w8_hpad
.w8_wpad:                                              ; wpadd=1
    movddup              m0, [yq]
    movddup              m1, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    pshufhw              m0, m0, q3333
    mova              [acq], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 16
    sub                  hd, 1
    jg .w8_wpad
    test              hpadd, hpadd
    jz .calc_avg_4_8
.w8_hpad:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 16
    sub               hpadd, 1
    jg .w8_hpad
    jmp .calc_avg_4_8
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
    mova                 m6, [yq+16]
    mova                 m1, [yq+strideq+16]
    pmaddubsw            m6, m2
    pmaddubsw            m1, m2
    paddw                m6, m1
    mova           [acq+16], m6
    paddw                m4, m6
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    dec                  hd
    jg .w16_loop
    test              hpadd, hpadd
    jz .calc_avg16
    jmp .w16_hpad_loop
.w16_wpad:
    cmp               wpadd, 2
    jl .w16_pad1
    je .w16_pad2
.w16_pad3:
    movddup              m0, [yq]
    movddup              m1, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    pshufhw              m0, m0, q3333
    mova              [acq], m0
    paddw                m4, m0
    mova                 m6, m0
    punpckhqdq           m6, m0, m0
    mova           [acq+16], m6
    paddw                m4, m6
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    dec                  hd
    jg .w16_pad3
    jmp .w16_wpad_done
.w16_pad2:
    mova                 m0, [yq]
    mova                 m1, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    mova              [acq], m0
    paddw                m4, m0
    pshufhw              m6, m0, q3333
    punpckhqdq           m6, m6
    mova           [acq+16], m6
    paddw                m4, m6
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    dec                  hd
    jg .w16_pad2
    jmp .w16_wpad_done
.w16_pad1:
    mova                 m0, [yq]
    mova                 m1, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    paddw                m0, m1
    mova              [acq], m0
    paddw                m4, m0
    movddup              m6, [yq+16]
    movddup              m1, [yq+strideq+16]
    pmaddubsw            m6, m2
    pmaddubsw            m1, m2
    paddw                m6, m1
    pshufhw              m6, m6, q3333
    mova           [acq+16], m6
    paddw                m4, m6
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    dec                  hd
    jg .w16_pad1
.w16_wpad_done:
    test              hpadd, hpadd
    jz .calc_avg16
.w16_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    mova           [acq+16], m6
    paddw                m4, m6
    add                 acq, 32
    dec               hpadd
    jg .w16_hpad_loop
    jmp .calc_avg16

%if ARCH_X86_64
    DEFINE_ARGS ac, y, iptr, wpad, hpad, sz, h, ac_bak
%else
    DEFINE_ARGS ac, y, iptr, wpad, hpad, sz, h
%endif
.calc_avg_4_8:
    psrlw                m2, 9
    pmaddwd              m4, m2
    jmp .calc_avg
.calc_avg16:
    psrld                m0, m4, 16
    pslld                m4, 16
    psrld                m4, 16
    paddd                m4, m0
.calc_avg:
    movd                szd, m5
    psrad                m5, 1
    tzcnt               r1d, szd
    paddd                m4, m5
    movd                 m1, r1d
    pshufd               m0, m4, q2301
    paddd                m0, m4
    pshufd               m4, m0, q1032
    paddd                m0, m4
    psrad                m0, m1                        ; sum >>= log2sz;
    packssdw             m0, m0
    RELOAD_ACQ_32       acq
.sub_loop:
    mova                 m1, [acq]
    psubw                m1, m0                        ; ac[x] -= sum;
    mova              [acq], m1
    add                 acq, 16
    sub                 szd, 8
    jg .sub_loop
    RET

%if ARCH_X86_64
cglobal ipred_cfl_ac_422_8bpc, 4, 8, 7, ac, y, stride, wpad, hpad, w, h, ac_bak
    movddup              m2, [pb_4]
%else
cglobal ipred_cfl_ac_422_8bpc, 4, 7, 7, ac, y, stride, wpad, hpad, w, h
    mov                 t0d, 0x04040404
    movd                 m2, t0d
    pshufd               m2, m2, q0000
%endif
    movifnidn            wd, wm
    mov                 t0d, hm
    mov                  hd, t0d
    imul                t0d, wd
    movd                 m6, t0d
    movifnidn         hpadd, hpadm
%if ARCH_X86_64
    mov             ac_bakq, acq
%endif
    shl               hpadd, 2
    sub                  hd, hpadd
    pxor                 m4, m4
    pxor                 m5, m5
    cmp                  wd, 8
    jg .w16
    je .w8
    ; fall-through

%if ARCH_X86_64
    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h, ac_bak
%else
    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h
%endif
.w4:
    lea            stride3q, [strideq*3]
.w4_loop:
    movq                 m1, [yq]
    movhps               m1, [yq+strideq]
    movq                 m0, [yq+strideq*2]
    movhps               m0, [yq+stride3q]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    mova              [acq], m1
    mova           [acq+16], m0
    paddw                m4, m0
    paddw                m5, m1
    lea                  yq, [yq+strideq*4]
    add                 acq, 32
    sub                  hd, 4
    jg .w4_loop
    test              hpadd, hpadd
    jz .calc_avg_4
    punpckhqdq           m0, m0
.w4_hpad_loop:
    mova              [acq], m0
    paddw                m4, m0
    add                 acq, 16
    sub               hpadd, 2
    jg .w4_hpad_loop
    jmp .calc_avg_4
.w8:
    lea            stride3q, [strideq*3]
    test              wpadd, wpadd
    jnz .w8_wpad
.w8_loop:
    mova                 m1, [yq]
    mova                 m0, [yq+strideq]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    mova              [acq], m1
    mova           [acq+16], m0
    paddw                m4, m0
    paddw                m5, m1
    mova                 m1, [yq+strideq*2]
    mova                 m0, [yq+stride3q]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    mova           [acq+48], m0
    paddw                m4, m0
    paddw                m5, m1
    lea                  yq, [yq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .w8_loop
    test              hpadd, hpadd
    jz .calc_avg_8_16
    jmp .w8_hpad
.w8_wpad:
    movddup              m1, [yq]
    pmaddubsw            m1, m2
    pshufhw              m1, m1, q3333
    mova              [acq], m1
    paddw                m5, m1
    movddup              m0, [yq+strideq]
    pmaddubsw            m0, m2
    pshufhw              m0, m0, q3333
    mova           [acq+16], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    sub                  hd, 2
    jg .w8_wpad
    test              hpadd, hpadd
    jz .calc_avg_8_16
.w8_hpad:
    mova              [acq], m0
    paddw                m4, m0
    mova           [acq+16], m0
    paddw                m4, m0
    add                 acq, 32
    sub               hpadd, 2
    jg .w8_hpad
    jmp .calc_avg_8_16
.w16:
    test              wpadd, wpadd
    jnz .w16_wpad
.w16_loop:
    mova                 m1, [yq]
    mova                 m0, [yq+16]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    mova              [acq], m1
    mova           [acq+16], m0
    paddw                m5, m0
    paddw                m5, m1
    mova                 m1, [yq+strideq]
    mova                 m0, [yq+strideq+16]
    pmaddubsw            m0, m2
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    mova           [acq+48], m0
    paddw                m4, m0
    paddw                m4, m1
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_loop
    test              hpadd, hpadd
    jz .calc_avg_8_16
    jmp .w16_hpad_loop
.w16_wpad:
    cmp               wpadd, 2
    jl .w16_pad1
    je .w16_pad2
.w16_pad3:
    movddup              m1, [yq]
    pmaddubsw            m1, m2
    pshufhw              m1, m1, q3333
    mova              [acq], m1
    paddw                m5, m1
    punpckhqdq           m1, m1
    mova           [acq+16], m1
    paddw                m5, m1
    movddup              m1, [yq+strideq]
    pmaddubsw            m1, m2
    pshufhw              m1, m1, q3333
    mova           [acq+32], m1
    paddw                m4, m1
    punpckhqdq           m0, m1, m1
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_pad3
    jmp .w16_wpad_done
.w16_pad2:
    mova                 m1, [yq]
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1
    pshufhw              m1, m1, q3333
    punpckhqdq           m1, m1
    mova           [acq+16], m1
    paddw                m5, m1
    mova                 m1, [yq+strideq]
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    paddw                m4, m1
    mova                 m0, m1
    pshufhw              m0, m0, q3333
    punpckhqdq           m0, m0
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_pad2
    jmp .w16_wpad_done
.w16_pad1:
    mova                 m1, [yq]
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1
    movddup              m0, [yq+16]
    pmaddubsw            m0, m2
    pshufhw              m0, m0, q3333
    mova           [acq+16], m0
    paddw                m5, m0
    mova                 m1, [yq+strideq]
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    paddw                m4, m1
    movddup              m0, [yq+strideq+16]
    pmaddubsw            m0, m2
    pshufhw              m0, m0, q3333
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_pad1
.w16_wpad_done:
    test              hpadd, hpadd
    jz .calc_avg_8_16
.w16_hpad_loop:
    mova              [acq], m1
    mova           [acq+16], m0
    paddw                m4, m1
    paddw                m5, m0
    mova           [acq+32], m1
    mova           [acq+48], m0
    paddw                m4, m1
    paddw                m5, m0
    add                 acq, 64
    sub               hpadd, 2
    jg .w16_hpad_loop
    jmp .calc_avg_8_16

%if ARCH_X86_64
    DEFINE_ARGS ac, y, iptr, wpad, hpad, sz, h, ac_bak
%else
    DEFINE_ARGS ac, y, iptr, wpad, hpad, sz, h
%endif
.calc_avg_4:
    psrlw                m2, 10
    pmaddwd              m5, m2
    pmaddwd              m0, m4, m2
    jmp .calc_avg
.calc_avg_8_16:
    mova                 m0, m5
    psrld                m5, 16
    pslld                m0, 16
    psrld                m0, 16
    paddd                m5, m0
    mova                 m0, m4
    psrld                m0, 16
    pslld                m4, 16
    psrld                m4, 16
    paddd                m0, m4
.calc_avg:
    paddd                m5, m0
    movd                szd, m6
    psrad                m6, 1
    tzcnt               r1d, szd                       ; const int log2sz = ctz(width) + ctz(height);
    paddd                m5, m6
    movd                 m1, r1d
    pshufd               m0, m5, q2301
    paddd                m0, m5
    pshufd               m5, m0, q1032
    paddd                m0, m5
    psrad                m0, m1                        ; sum >>= log2sz;
    packssdw             m0, m0
    RELOAD_ACQ_32       acq                            ; ac = ac_orig
.sub_loop:
    mova                 m1, [acq]
    psubw                m1, m0
    mova              [acq], m1
    add                 acq, 16
    sub                 szd, 8
    jg .sub_loop
    RET

%if ARCH_X86_64
cglobal ipred_cfl_ac_444_8bpc, 4, 8, 7, -4*16, ac, y, stride, wpad, hpad, w, h, ac_bak
    movddup              m2, [pb_4]
%else
cglobal ipred_cfl_ac_444_8bpc, 4, 7, 7, -5*16, ac, y, stride, wpad, hpad, w, h
%define ac_bakq [rsp+16*4]
    mov                 t0d, 0x04040404
    movd                 m2, t0d
    pshufd               m2, m2, q0000
%endif
    movifnidn            wd, wm
    movifnidn         hpadd, hpadm
    movd                 m0, hpadd
    mov                 t0d, hm
    mov                  hd, t0d
    imul                t0d, wd
    movd                 m6, t0d
    movd              hpadd, m0
    mov             ac_bakq, acq
    shl               hpadd, 2
    sub                  hd, hpadd
    pxor                 m5, m5
    pxor                 m4, m4
    cmp                  wd, 16
    jg .w32
    cmp                  wd, 8
    jg .w16
    je .w8
    ; fall-through

%if ARCH_X86_64
    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h, ac_bak
%else
    DEFINE_ARGS ac, y, stride, wpad, hpad, stride3, h
%endif
.w4:
    lea            stride3q, [strideq*3]
.w4_loop:
    movd                 m1, [yq]
    movd                 m3, [yq+strideq]
    punpckldq            m1, m3
    punpcklbw            m1, m1
    movd                 m0, [yq+strideq*2]
    movd                 m3, [yq+stride3q]
    punpckldq            m0, m3
    punpcklbw            m0, m0
    pmaddubsw            m1, m2
    pmaddubsw            m0, m2
    mova              [acq], m1
    mova           [acq+16], m0
    paddw                m5, m0
    paddw                m5, m1
    lea                  yq, [yq+strideq*4]
    add                 acq, 32
    sub                  hd, 4
    jg .w4_loop
    test              hpadd, hpadd
    jz .calc_avg_4
    punpckhqdq           m0, m0
.w4_hpad_loop:
    mova              [acq], m0
    paddw                m5, m0
    add                 acq, 16
    sub               hpadd, 2
    jg .w4_hpad_loop
.calc_avg_4:
    psrlw                m2, 10
    pmaddwd              m5, m2
    jmp .calc_avg

.w8:
    lea            stride3q, [strideq*3]
    test              wpadd, wpadd
    jnz .w8_wpad
.w8_loop:
    movq                 m1, [yq]
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1
    movq                 m0, [yq+strideq]
    punpcklbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0
    movq                 m1, [yq+strideq*2]
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    paddw                m4, m1
    movq                 m0, [yq+stride3q]
    punpcklbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*4]
    add                 acq, 64
    sub                  hd, 4
    jg .w8_loop
    test              hpadd, hpadd
    jz .calc_avg_8_16
    jmp .w8_hpad
.w8_wpad:
    movd                 m1, [yq]
    punpcklbw            m1, m1
    punpcklqdq           m1, m1
    pmaddubsw            m1, m2
    pshufhw              m1, m1, q3333
    mova              [acq], m1
    paddw                m5, m1
    movd                 m0, [yq+strideq]
    punpcklbw            m0, m0
    punpcklqdq           m0, m0
    pmaddubsw            m0, m2
    pshufhw              m0, m0, q3333
    mova           [acq+16], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 32
    sub                  hd, 2
    jg .w8_wpad
    test              hpadd, hpadd
    jz .calc_avg_8_16
.w8_hpad:
    mova              [acq], m0
    paddw                m5, m0
    mova           [acq+16], m0
    paddw                m4, m0
    add                 acq, 32
    sub               hpadd, 2
    jg .w8_hpad
    jmp .calc_avg_8_16

.w16:
    test              wpadd, wpadd
    jnz .w16_wpad
.w16_loop:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1
    punpckhbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0
    mova                 m0, [yq+strideq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    paddw                m4, m1
    punpckhbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_loop
    test              hpadd, hpadd
    jz .calc_avg_8_16
    jmp .w16_hpad_loop
.w16_wpad:
    cmp               wpadd, 2
    jl .w16_pad1
    je .w16_pad2
.w16_pad3:
    movd                 m1, [yq]
    punpcklbw            m1, m1
    punpcklqdq           m1, m1
    pshufhw              m1, m1, q3333
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1
    punpckhqdq           m1, m1
    mova           [acq+16], m1
    paddw                m5, m1
    movd                 m1, [yq+strideq]
    punpcklbw            m1, m1
    punpcklqdq           m1, m1
    pshufhw              m1, m1, q3333
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    paddw                m4, m1
    punpckhqdq           m0, m1, m1
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_pad3
    jmp .w16_wpad_done
.w16_pad2:
    movq                 m1, [yq]
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1
    pshufhw              m1, m1, q3333
    punpckhqdq           m1, m1
    mova           [acq+16], m1
    paddw                m5, m1
    movq                 m1, [yq+strideq]
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    paddw                m4, m1
    mova                 m0, m1
    pshufhw              m0, m0, q3333
    punpckhqdq           m0, m0
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_pad2
    jmp .w16_wpad_done
.w16_pad1:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1
    punpckhbw            m0, m0
    punpcklqdq           m0, m0
    pshufhw              m0, m0, q3333
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0
    mova                 m0, [yq+strideq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova           [acq+32], m1
    paddw                m4, m1
    punpckhbw            m0, m0
    punpcklqdq           m0, m0
    pshufhw              m0, m0, q3333
    pmaddubsw            m0, m2
    mova           [acq+48], m0
    paddw                m4, m0
    lea                  yq, [yq+strideq*2]
    add                 acq, 64
    sub                  hd, 2
    jg .w16_pad1
.w16_wpad_done:
    test              hpadd, hpadd
    jz .calc_avg_8_16
.w16_hpad_loop:
    mova              [acq], m1
    mova           [acq+16], m0
    paddw                m4, m1
    paddw                m5, m0
    mova           [acq+32], m1
    mova           [acq+48], m0
    paddw                m4, m1
    paddw                m5, m0
    add                 acq, 64
    sub               hpadd, 2
    jg .w16_hpad_loop
.calc_avg_8_16:
    mova                 m0, m5
    psrld                m5, 16
    pslld                m0, 16
    psrld                m0, 16
    paddd                m5, m0
    mova                 m0, m4
    psrld                m0, 16
    pslld                m4, 16
    psrld                m4, 16
    paddd                m0, m4
    paddd                m5, m0
    jmp .calc_avg

.w32:
    pxor                 m0, m0
    mova           [rsp   ], m0
    mova           [rsp+16], m0
    mova           [rsp+32], m0
    mova           [rsp+48], m0
    test              wpadd, wpadd
    jnz .w32_wpad
.w32_loop:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    punpckhbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova                 m4, [yq+16]
    mova                 m3, m4
    punpcklbw            m3, m3
    pmaddubsw            m3, m2
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    punpckhbw            m4, m4
    pmaddubsw            m4, m2
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_loop
    test              hpadd, hpadd
    jz .calc_avg_32
    jmp .w32_hpad_loop
.w32_wpad:
    cmp               wpadd, 2
    jl .w32_pad1
    je .w32_pad2
    cmp               wpadd, 4
    jl .w32_pad3
    je .w32_pad4
    cmp               wpadd, 6
    jl .w32_pad5
    je .w32_pad6
.w32_pad7:
    movd                 m1, [yq]
    punpcklbw            m1, m1
    punpcklqdq           m1, m1
    pshufhw              m1, m1, q3333
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    mova                 m0, m1
    punpckhqdq           m0, m0
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova                 m3, m0
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    mova                 m4, m3
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_pad7
    jmp .w32_wpad_done
.w32_pad6:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    pshufhw              m0, m1, q3333
    punpckhqdq           m0, m0
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova                 m3, m0
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    mova                 m4, m3
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_pad6
    jmp .w32_wpad_done
.w32_pad5:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    mova                 m5, [rsp]
    paddw                m5, m1
    mova           [rsp   ], m5
    punpckhbw            m0, m0
    punpcklqdq           m0, m0
    pshufhw              m0, m0, q3333
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova                 m3, m0
    punpckhqdq           m3, m3
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    mova                 m4, m3
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_pad5
    jmp .w32_wpad_done
.w32_pad4:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    punpckhbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova                 m3, m0
    pshufhw              m3, m3, q3333
    punpckhqdq           m3, m3
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    mova                 m4, m3
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_pad4
    jmp .w32_wpad_done
.w32_pad3:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    punpckhbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    movd                 m3, [yq+16]
    punpcklbw            m3, m3
    punpcklqdq           m3, m3
    pshufhw              m3, m3, q3333
    pmaddubsw            m3, m2
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    mova                 m4, m3
    punpckhqdq           m4, m4
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_pad3
    jmp .w32_wpad_done
.w32_pad2:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    punpckhbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova                 m3, [yq+16]
    punpcklbw            m3, m3
    pmaddubsw            m3, m2
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    pshufhw              m4, m3, q3333
    punpckhqdq           m4, m4
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_pad2
    jmp .w32_wpad_done
.w32_pad1:
    mova                 m0, [yq]
    mova                 m1, m0
    punpcklbw            m1, m1
    pmaddubsw            m1, m2
    mova              [acq], m1
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    punpckhbw            m0, m0
    pmaddubsw            m0, m2
    mova           [acq+16], m0
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova                 m4, [yq+16]
    mova                 m3, m4
    punpcklbw            m3, m3
    pmaddubsw            m3, m2
    mova           [acq+32], m3
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    punpckhbw            m4, m4
    punpcklqdq           m4, m4
    pshufhw              m4, m4, q3333
    pmaddubsw            m4, m2
    mova           [acq+48], m4
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    lea                  yq, [yq+strideq]
    add                 acq, 64
    sub                  hd, 1
    jg .w32_pad1
.w32_wpad_done:
    test              hpadd, hpadd
    jz .calc_avg_32
.w32_hpad_loop:
    mova              [acq], m1
    mova           [acq+16], m0
    paddw                m5, m1, [rsp]
    mova           [rsp   ], m5
    paddw                m5, m0, [rsp+16]
    mova           [rsp+16], m5
    mova           [acq+32], m3
    mova           [acq+48], m4
    paddw                m5, m3, [rsp+32]
    mova           [rsp+32], m5
    paddw                m5, m4, [rsp+48]
    mova           [rsp+48], m5
    add                 acq, 64
    sub               hpadd, 1
    jg .w32_hpad_loop

%if ARCH_X86_64
    DEFINE_ARGS ac, y, iptr, wpad, hpad, sz, h, ac_bak
%else
    DEFINE_ARGS ac, y, iptr, wpad, hpad, sz, h
%endif

.calc_avg_32:
    mova                 m5, [rsp]
    mova                 m0, m5
    psrld                m5, 16
    pslld                m0, 16
    psrld                m0, 16
    paddd                m5, m0
    mova                 m0, [rsp+16]
    mova                 m3, m0
    psrld                m0, 16
    pslld                m3, 16
    psrld                m3, 16
    paddd                m0, m3
    paddd                m5, m0
    mova                 m0, [rsp+32]
    mova                 m3, m0
    psrld                m0, 16
    pslld                m3, 16
    psrld                m3, 16
    paddd                m0, m3
    mova                 m1, [rsp+48]
    mova                 m3, m1
    psrld                m1, 16
    pslld                m3, 16
    psrld                m3, 16
    paddd                m1, m3
    paddd                m1, m0
    paddd                m5, m1
.calc_avg:
    movd                szd, m6
    psrad                m6, 1
    tzcnt               r1d, szd                       ; const int log2sz = ctz(width) + ctz(height);
    paddd                m5, m6
    movd                 m1, r1d
    pshufd               m0, m5, q2301
    paddd                m0, m5
    pshufd               m5, m0, q1032
    paddd                m0, m5
    psrad                m0, m1                        ; sum >>= log2sz;
    packssdw             m0, m0
    RELOAD_ACQ_32       acq                            ; ac = ac_orig
.sub_loop:
    mova                 m1, [acq]
    psubw                m1, m0
    mova              [acq], m1
    add                 acq, 16
    sub                 szd, 8
    jg .sub_loop
    RET

; %1 simd register that hold the mask and will hold the result
; %2 simd register that holds the "true" values
; %3 location of the "false" values (simd register/memory)
%macro BLEND 3 ; mask, true, false
    pand  %2, %1
    pandn %1, %3
    por   %1, %2
%endmacro

%macro PAETH 2                                 ; top, ldiff
    pavgb                m1, m%1, m3
    pxor                 m0, m%1, m3
    pand                 m0, m4
    psubusb              m2, m5, m1
    psubb                m1, m0
    psubusb              m1, m5
    por                  m1, m2
    paddusb              m1, m1
    por                  m1, m0               ; min(tldiff, 255)
    psubusb              m2, m5, m3
    psubusb              m0, m3, m5
    por                  m2, m0               ; tdiff
%ifnum %2
    pminub               m2, m%2
    pcmpeqb              m0, m%2, m2          ; ldiff <= tdiff
%else
    mova                 m0, %2
    pminub               m2, m0
    pcmpeqb              m0, m2
%endif
    pminub               m1, m2
    pcmpeqb              m1, m2               ; ldiff <= tldiff && tdiff <= tldiff
    mova                 m2, m3
    BLEND                m0, m2, m%1
    BLEND                m1, m0, m5
%endmacro

cglobal ipred_paeth_8bpc, 3, 6, 8, -7*16, dst, stride, tl, w, h
%define base r5-ipred_paeth_ssse3_table
    tzcnt                wd, wm
    movifnidn            hd, hm
    pxor                 m0, m0
    movd                 m5, [tlq]
    pshufb               m5, m0
    LEA                  r5, ipred_paeth_ssse3_table
    movsxd               wq, [r5+wq*4]
    movddup              m4, [base+ipred_paeth_shuf]
    add                  wq, r5
    jmp                  wq
.w4:
    movd                 m6, [tlq+1]            ; top
    pshufd               m6, m6, q0000
    lea                  r3, [strideq*3]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0                 ; ldiff
.w4_loop:
    sub                 tlq, 4
    movd                 m3, [tlq]
    mova                 m1, [base+ipred_h_shuf]
    pshufb               m3, m1                 ; left
    PAETH                 6, 7
    movd   [dstq          ], m1
    pshuflw              m0, m1, q1032
    movd   [dstq+strideq  ], m0
    punpckhqdq           m1, m1
    movd   [dstq+strideq*2], m1
    psrlq                m1, 32
    movd   [dstq+r3       ], m1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4_loop
    RET
ALIGN function_align
.w8:
    movddup              m6, [tlq+1]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w8_loop:
    sub                 tlq, 2
    movd                 m3, [tlq]
    pshufb               m3, [base+ipred_paeth_shuf]
    PAETH                 6, 7
    movq     [dstq        ], m1
    movhps   [dstq+strideq], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
ALIGN function_align
.w16:
    movu                 m6, [tlq+1]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
.w16_loop:
    sub                 tlq, 1
    movd                 m3, [tlq]
    pxor                 m1, m1
    pshufb               m3, m1
    PAETH                 6, 7
    mova             [dstq], m1
    add                dstq, strideq
    sub                  hd, 1
    jg .w16_loop
    RET
ALIGN function_align
.w32:
    movu                 m6, [tlq+1]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
    mova           [rsp   ], m6
    mova           [rsp+16], m7
    movu                 m6, [tlq+17]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
    mova           [rsp+32], m6
.w32_loop:
    dec                 tlq
    movd                 m3, [tlq]
    pxor                 m1, m1
    pshufb               m3, m1
    mova                 m6, [rsp]
    PAETH                 6, [rsp+16]
    mova          [dstq   ], m1
    mova                 m6, [rsp+32]
    PAETH                 6, 7
    mova          [dstq+16], m1
    add                dstq, strideq
    dec                  hd
    jg .w32_loop
    RET
ALIGN function_align
.w64:
    movu                 m6, [tlq+1]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
    mova           [rsp   ], m6
    mova           [rsp+16], m7
    movu                 m6, [tlq+17]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
    mova           [rsp+32], m6
    mova           [rsp+48], m7
    movu                 m6, [tlq+33]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
    mova           [rsp+64], m6
    mova           [rsp+80], m7
    movu                 m6, [tlq+49]
    psubusb              m7, m5, m6
    psubusb              m0, m6, m5
    por                  m7, m0
    mova           [rsp+96], m6
.w64_loop:
    dec                 tlq
    movd                 m3, [tlq]
    pxor                 m1, m1
    pshufb               m3, m1
    mova                 m6, [rsp]
    PAETH                 6, [rsp+16]
    mova          [dstq   ], m1
    mova                 m6, [rsp+32]
    PAETH                 6, [rsp+48]
    mova          [dstq+16], m1
    mova                 m6, [rsp+64]
    PAETH                 6, [rsp+80]
    mova          [dstq+32], m1
    mova                 m6, [rsp+96]
    PAETH                 6, 7
    mova          [dstq+48], m1
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET


%macro FILTER 4  ;dst, src, tmp, shuf
%ifnum %4
    pshufb               m%2, m%4
%else
    pshufb               m%2, %4
%endif
    pshufd               m%1, m%2, q0000           ;p0 p1
    pmaddubsw            m%1, m2
    pshufd               m%3, m%2, q1111           ;p2 p3
    pmaddubsw            m%3, m3
    paddw                m%1, [base+pw_8]
    paddw                m%1, m%3
    pshufd               m%3, m%2, q2222           ;p4 p5
    pmaddubsw            m%3, m4
    paddw                m%1, m%3
    pshufd               m%3, m%2, q3333           ;p6 __
    pmaddubsw            m%3, m5
    paddw                m%1, m%3
    psraw                m%1, 4
    packuswb             m%1, m%1
%endmacro

cglobal ipred_filter_8bpc, 3, 7, 8, dst, stride, tl, w, h, filter
%define base r6-$$
    LEA                   r6, $$
    tzcnt                 wd, wm
%ifidn filterd, filterm
    movzx            filterd, filterb
%else
    movzx            filterd, byte filterm
%endif
    shl              filterd, 6
    lea              filterq, [base+filter_intra_taps+filterq]
    movq                  m0, [tlq-3]                     ;_ 6 5 0 1 2 3 4
    movsxd                wq, [base+ipred_filter_ssse3_table+wq*4]
    mova                  m2, [filterq+16*0]
    mova                  m3, [filterq+16*1]
    mova                  m4, [filterq+16*2]
    mova                  m5, [filterq+16*3]
    lea                   wq, [base+ipred_filter_ssse3_table+wq]
    mov                   hd, hm
    jmp                   wq
.w4:
    mova                  m1, [base+filter_shuf1]
    sub                  tlq, 3
    sub                  tlq, hq
    jmp .w4_loop_start
.w4_loop:
    movd                  m0, [tlq+hq]
    punpckldq             m0, m6
    lea                 dstq, [dstq+strideq*2]
.w4_loop_start:
    FILTER                 6, 0, 7, 1
    movd    [dstq+strideq*0], m6
    pshuflw               m6, m6, q1032
    movd    [dstq+strideq*1], m6
    sub                   hd, 2
    jg .w4_loop
    RET

ALIGN function_align
.w8:
    movq                  m6, [tlq+1]                   ;_ _ _ 0 1 2 3 4
    sub                  tlq, 5
    sub                  tlq, hq

.w8_loop:
    FILTER                 7, 0, 1, [base+filter_shuf1]
    punpcklqdq            m6, m7                        ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    FILTER                 0, 6, 1, [base+filter_shuf2]

    punpckldq             m6, m7, m0
    movq    [dstq+strideq*0], m6
    punpckhqdq            m6, m6
    movq    [dstq+strideq*1], m6

    movd                  m0, [tlq+hq]                  ;_ 6 5 0
    punpckldq             m0, m6                        ;_ 6 5 0 1 2 3 4

    lea                 dstq, [dstq+strideq*2]
    sub                   hd, 2
    jg .w8_loop
    RET

ALIGN function_align
.w16:
    movu                  m6, [tlq+1]                   ;top row
    sub                  tlq, 5
    sub                  tlq, hq

.w16_loop:
    FILTER                 7, 0, 1, [base+filter_shuf1]
    punpcklqdq            m0, m6, m7                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd    [dstq+strideq*0], m7
    psrlq                 m7, 32
    palignr               m7, m6, 4

    FILTER                 6, 0, 1, [base+filter_shuf2]
    punpcklqdq            m0, m7, m6                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd  [dstq+4+strideq*0], m6
    psrlq                 m6, 32
    palignr               m6, m7, 4

    FILTER                 7, 0, 1, [base+filter_shuf2]
    punpcklqdq            m0, m6, m7                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd  [dstq+8+strideq*0], m7
    psrlq                 m7, 32
    palignr               m7, m6, 4

    FILTER                 6, 0, 1, [base+filter_shuf2]
    movd [dstq+12+strideq*0], m6
    psrlq                 m6, 32
    palignr               m6, m7, 4
    mova    [dstq+strideq*1], m6

    movd                  m0, [tlq+hq]                  ;_ 6 5 0
    punpckldq             m0, m6                        ;_ 6 5 0 1 2 3 4

    lea                 dstq, [dstq+strideq*2]
    sub                   hd, 2
    jg .w16_loop
    RET

ALIGN function_align
.w32:
    movu                  m6, [tlq+1]                   ;top row
    lea              filterq, [tlq+17]
    sub                  tlq, 5
    sub                  tlq, hq

.w32_loop:
    FILTER                 7, 0, 1, [base+filter_shuf1]
    punpcklqdq            m0, m6, m7                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd    [dstq+strideq*0], m7
    psrlq                 m7, 32
    palignr               m7, m6, 4

    FILTER                 6, 0, 1, [base+filter_shuf2]
    punpcklqdq            m0, m7, m6                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd  [dstq+4+strideq*0], m6
    psrlq                 m6, 32
    palignr               m6, m7, 4

    FILTER                 7, 0, 1, [base+filter_shuf2]
    punpcklqdq            m0, m6, m7                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd  [dstq+8+strideq*0], m7
    psrlq                 m7, 32
    palignr               m7, m6, 4

    FILTER                 6, 0, 1, [base+filter_shuf2]
    movu                  m1, [filterq]
    punpckldq             m0, m7, m1                    ;_ _ _ 0 1 2 3 4 _ _ _ _ _ _ _ _
    punpcklqdq            m0, m6                        ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd [dstq+12+strideq*0], m6
    psrlq                 m6, 32
    palignr               m6, m7, 4
    mova    [dstq+strideq*1], m6

    mova                  m6, m1

    FILTER                 7, 0, 6, [base+filter_shuf2]
    punpcklqdq            m0, m1, m7                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd [dstq+16+strideq*0], m7
    psrlq                 m7, 32
    palignr               m7, m1, 4

    FILTER                 6, 0, 1, [base+filter_shuf2]
    punpcklqdq            m0, m7, m6                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd [dstq+20+strideq*0], m6
    psrlq                 m6, 32
    palignr               m6, m7, 4

    FILTER                 7, 0, 1, [base+filter_shuf2]
    punpcklqdq            m0, m6, m7                    ;_ _ _ 0 1 2 3 4 _ _ _ 5 _ _ _ 6
    movd [dstq+24+strideq*0], m7
    psrlq                 m7, 32
    palignr               m7, m6, 4

    FILTER                 6, 0, 1, [base+filter_shuf2]
    movd [dstq+28+strideq*0], m6
    psrlq                 m6, 32
    palignr               m6, m7, 4
    mova [dstq+16+strideq*1], m6

    mova                  m6, [dstq+strideq*1]
    movd                  m0, [tlq+hq]                  ;_ 6 5 0
    punpckldq             m0, m6                        ;_ 6 5 0 1 2 3 4
    lea              filterq, [dstq+16+strideq*1]
    lea                 dstq, [dstq+strideq*2]
    sub                   hd, 2
    jg .w32_loop
    RET
