; Copyright © 2022, VideoLAN and dav1d authors
; Copyright © 2022, Two Orioles, LLC
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

ipred_shuf:    db 14, 15, 14, 15,  0,  1,  2,  3,  6,  7,  6,  7,  0,  1,  2,  3
               db 10, 11, 10, 11,  8,  9, 10, 11,  2,  3,  2,  3,  8,  9, 10, 11
               db 12, 13, 12, 13,  4,  5,  6,  7,  4,  5,  4,  5,  4,  5,  6,  7
               db  8,  9,  8,  9, 12, 13, 14, 15,  0,  1,  0,  1, 12, 13, 14, 15
smooth_perm:   db  1,  2,  5,  6,  9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30
               db 33, 34, 37, 38, 41, 42, 45, 46, 49, 50, 53, 54, 57, 58, 61, 62
               db 65, 66, 69, 70, 73, 74, 77, 78, 81, 82, 85, 86, 89, 90, 93, 94
               db 97, 98,101,102,105,106,109,110,113,114,117,118,121,122,125,126
pal_pred_perm: db  0, 32,  1, 33,  2, 34,  3, 35,  4, 36,  5, 37,  6, 38,  7, 39
               db  8, 40,  9, 41, 10, 42, 11, 43, 12, 44, 13, 45, 14, 46, 15, 47
               db 16, 48, 17, 49, 18, 50, 19, 51, 20, 52, 21, 53, 22, 54, 23, 55
               db 24, 56, 25, 57, 26, 58, 27, 59, 28, 60, 29, 61, 30, 62, 31, 63
filter_permA:  times 4 db  6,  7,  8,  9, 14, 15,  4,  5
               times 4 db 10, 11, 12, 13,  2,  3, -1, -1
filter_permB:  times 4 db 22, 23, 24, 25, 30, 31,  6,  7
               times 4 db 26, 27, 28, 29, 14, 15, -1, -1
filter_permC:          dd  8 ; dq  8, 10,  1, 11,  0,  9
pw_1:          times 2 dw  1
                       dd 10
filter_rnd:            dd 32
                       dd  1
                       dd  8
                       dd 11
filter_shift:  times 2 dw  6
                       dd  0
               times 2 dw  4
                       dd  9

%macro JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - 2*4)
    %xdefine %%base mangle(private_prefix %+ _%1_%2)
    %%table:
    %rep %0 - 2
        dd %%base %+ .%3 - (%%table - 2*4)
        %rotate 1
    %endrep
%endmacro

JMP_TABLE ipred_paeth_16bpc,      avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_16bpc,     avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_h_16bpc,   avx512icl, w4, w8, w16, w32, w64
JMP_TABLE ipred_smooth_v_16bpc,   avx512icl, w4, w8, w16, w32, w64
JMP_TABLE pal_pred_16bpc,         avx512icl, w4, w8, w16, w32, w64

cextern smooth_weights_1d_16bpc
cextern smooth_weights_2d_16bpc
cextern filter_intra_taps

SECTION .text

%macro PAETH 3 ; top, signed_ldiff, ldiff
    paddw               m0, m%2, m2
    psubw               m1, m0, m3  ; tldiff
    psubw               m0, m%1     ; tdiff
    pabsw               m1, m1
    pabsw               m0, m0
    pcmpgtw             k1, m0, m1
    pminsw              m0, m1
    pcmpgtw             k2, m%3, m0
    vpblendmw       m0{k1}, m%1, m3
    vpblendmw       m0{k2}, m2, m0
%endmacro

INIT_ZMM avx512icl
cglobal ipred_paeth_16bpc, 3, 7, 10, dst, stride, tl, w, h
%define base r6-ipred_paeth_16bpc_avx512icl_table
    lea                 r6, [ipred_paeth_16bpc_avx512icl_table]
    tzcnt               wd, wm
    movifnidn           hd, hm
    movsxd              wq, [r6+wq*4]
    vpbroadcastw        m3, [tlq]   ; topleft
    add                 wq, r6
    jmp                 wq
.w4:
    vpbroadcastq        m4, [tlq+2] ; top
    movsldup            m7, [base+ipred_shuf]
    lea                 r6, [strideq*3]
    psubw               m5, m4, m3
    pabsw               m6, m5
.w4_loop:
    sub                tlq, 16
    vbroadcasti32x4     m2, [tlq]
    pshufb              m2, m7      ; left
    PAETH                4, 5, 6
    vextracti32x4      xm1, m0, 2
    vextracti32x4      xm8, ym0, 1
    vextracti32x4      xm9, m0, 3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm8
    movq   [dstq+r6       ], xm9
    sub                 hd, 8
    jl .w4_end
    lea               dstq, [dstq+strideq*4]
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm8
    movhps [dstq+r6       ], xm9
    lea               dstq, [dstq+strideq*4]
    jg .w4_loop
.w4_end:
    RET
.w8:
    vbroadcasti32x4     m4, [tlq+2]
    movsldup            m7, [base+ipred_shuf]
    lea                 r6, [strideq*3]
    psubw               m5, m4, m3
    pabsw               m6, m5
.w8_loop:
    sub                tlq, 8
    vpbroadcastq        m2, [tlq]
    pshufb              m2, m7
    PAETH                4, 5, 6
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+r6       ], m0, 3
    lea               dstq, [dstq+strideq*4]
    sub                 hd, 4
    jg .w8_loop
    RET
.w16:
    vbroadcasti32x8     m4, [tlq+2]
    movsldup            m7, [base+ipred_shuf]
    psubw               m5, m4, m3
    pabsw               m6, m5
.w16_loop:
    sub                tlq, 4
    vpbroadcastd        m2, [tlq]
    pshufb              m2, m7
    PAETH                4, 5, 6
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea               dstq, [dstq+strideq*2]
    sub                 hd, 2
    jg .w16_loop
    RET
.w32:
    movu                m4, [tlq+2]
    psubw               m5, m4, m3
    pabsw               m6, m5
.w32_loop:
    sub                tlq, 2
    vpbroadcastw        m2, [tlq]
    PAETH                4, 5, 6
    mova            [dstq], m0
    add               dstq, strideq
    dec                 hd
    jg .w32_loop
    RET
.w64:
    movu                m4, [tlq+ 2]
    movu                m7, [tlq+66]
    psubw               m5, m4, m3
    psubw               m8, m7, m3
    pabsw               m6, m5
    pabsw               m9, m8
.w64_loop:
    sub                tlq, 2
    vpbroadcastw        m2, [tlq]
    PAETH                4, 5, 6
    mova       [dstq+64*0], m0
    PAETH                7, 8, 9
    mova       [dstq+64*1], m0
    add               dstq, strideq
    dec                 hd
    jg .w64_loop
    RET

cglobal ipred_smooth_v_16bpc, 3, 7, 7, dst, stride, tl, w, h, weights, stride3
%define base r6-$$
    lea                  r6, [$$]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [base+ipred_smooth_v_16bpc_avx512icl_table+wq*4]
    lea            weightsq, [base+smooth_weights_1d_16bpc+hq*4]
    neg                  hq
    vpbroadcastw         m6, [tlq+hq*2] ; bottom
    lea                  wq, [base+ipred_smooth_v_16bpc_avx512icl_table+wq]
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    vpbroadcastq         m5, [tlq+2]    ; top
    movsldup             m4, [ipred_shuf]
    psubw                m5, m6         ; top - bottom
.w4_loop:
    vbroadcasti32x4      m3, [weightsq+hq*2]
    pshufb               m3, m4
    pmulhrsw             m3, m5
    paddw                m3, m6
    vextracti32x4       xm0, m3, 3
    vextracti32x4       xm1, ym3, 1
    vextracti32x4       xm2, m3, 2
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm3
    add                  hq, 8
    jg .end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm2
    movq   [dstq+stride3q ], xm3
    lea                dstq, [dstq+strideq*4]
    jl .w4_loop
.end:
    RET
.w8:
    vbroadcasti32x4      m5, [tlq+2]    ; top
    movsldup             m4, [ipred_shuf]
    psubw                m5, m6         ; top - bottom
.w8_loop:
    vpbroadcastq         m0, [weightsq+hq*2]
    pshufb               m0, m4
    pmulhrsw             m0, m5
    paddw                m0, m6
    vextracti32x4 [dstq+strideq*0], m0, 3
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    mova          [dstq+stride3q ], xm0
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w8_loop
    RET
.w16:
    vbroadcasti32x8      m5, [tlq+2]    ; top
    movsldup             m4, [ipred_shuf]
    psubw                m5, m6         ; top - bottom
.w16_loop:
    vpbroadcastd         m0, [weightsq+hq*2+0]
    vpbroadcastd         m1, [weightsq+hq*2+4]
    pshufb               m0, m4
    pshufb               m1, m4
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    paddw                m0, m6
    paddw                m1, m6
    vextracti32x8 [dstq+strideq*0], m0, 1
    mova          [dstq+strideq*1], ym0
    vextracti32x8 [dstq+strideq*2], m1, 1
    mova          [dstq+stride3q ], ym1
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w16_loop
    RET
.w32:
    movu                 m5, [tlq+2]
    psubw                m5, m6
.w32_loop:
    vpbroadcastw         m0, [weightsq+hq*2+0]
    vpbroadcastw         m1, [weightsq+hq*2+2]
    vpbroadcastw         m2, [weightsq+hq*2+4]
    vpbroadcastw         m3, [weightsq+hq*2+6]
    REPX   {pmulhrsw x, m5}, m0, m1, m2, m3
    REPX   {paddw    x, m6}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    lea                dstq, [dstq+strideq*4]
    add                  hq, 4
    jl .w32_loop
    RET
.w64:
    movu                 m4, [tlq+ 2]
    movu                 m5, [tlq+66]
    psubw                m4, m6
    psubw                m5, m6
.w64_loop:
    vpbroadcastw         m1, [weightsq+hq*2+0]
    vpbroadcastw         m3, [weightsq+hq*2+2]
    pmulhrsw             m0, m4, m1
    pmulhrsw             m1, m5
    pmulhrsw             m2, m4, m3
    pmulhrsw             m3, m5
    REPX      {paddw x, m6}, m0, m1, m2, m3
    mova [dstq+strideq*0+64*0], m0
    mova [dstq+strideq*0+64*1], m1
    mova [dstq+strideq*1+64*0], m2
    mova [dstq+strideq*1+64*1], m3
    lea                dstq, [dstq+strideq*2]
    add                  hq, 2
    jl .w64_loop
    RET

cglobal ipred_smooth_h_16bpc, 3, 7, 7, dst, stride, tl, w, h, stride3
    lea                  r6, [$$]
    mov                  wd, wm
    movifnidn            hd, hm
    vpbroadcastw         m6, [tlq+wq*2] ; right
    tzcnt                wd, wd
    add                  hd, hd
    movsxd               wq, [base+ipred_smooth_h_16bpc_avx512icl_table+wq*4]
    sub                 tlq, hq
    lea            stride3q, [strideq*3]
    lea                  wq, [base+ipred_smooth_h_16bpc_avx512icl_table+wq]
    jmp                  wq
.w4:
    movsldup             m4, [base+ipred_shuf]
    vpbroadcastq         m5, [base+smooth_weights_1d_16bpc+4*2]
.w4_loop:
    vbroadcasti32x4      m0, [tlq+hq-16] ; left
    pshufb               m0, m4
    psubw                m0, m6          ; left - right
    pmulhrsw             m0, m5
    paddw                m0, m6
    vextracti32x4       xm1, m0, 2
    vextracti32x4       xm2, ym0, 1
    vextracti32x4       xm3, m0, 3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm2
    movq   [dstq+stride3q ], xm3
    sub                  hd, 8*2
    jl .end
    lea                dstq, [dstq+strideq*4]
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm3
    lea                dstq, [dstq+strideq*4]
    jg .w4_loop
.end:
    RET
.w8:
    movsldup             m4, [base+ipred_shuf]
    vbroadcasti32x4      m5, [base+smooth_weights_1d_16bpc+8*2]
.w8_loop:
    vpbroadcastq         m0, [tlq+hq-8] ; left
    pshufb               m0, m4
    psubw                m0, m6         ; left - right
    pmulhrsw             m0, m5
    paddw                m0, m6
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4*2
    jg .w8_loop
    RET
.w16:
    movsldup             m4, [base+ipred_shuf]
    vbroadcasti32x8      m5, [base+smooth_weights_1d_16bpc+16*2]
.w16_loop:
    vpbroadcastd         m0, [tlq+hq-4]
    vpbroadcastd         m1, [tlq+hq-8]
    pshufb               m0, m4
    pshufb               m1, m4
    psubw                m0, m6
    psubw                m1, m6
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    paddw                m0, m6
    paddw                m1, m6
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hq, 4*2
    jg .w16_loop
    RET
.w32:
    movu                 m5, [base+smooth_weights_1d_16bpc+32*2]
.w32_loop:
    vpbroadcastq         m3, [tlq+hq-8]
    punpcklwd            m3, m3
    psubw                m3, m6
    pshufd               m0, m3, q3333
    pshufd               m1, m3, q2222
    pshufd               m2, m3, q1111
    pshufd               m3, m3, q0000
    REPX   {pmulhrsw x, m5}, m0, m1, m2, m3
    REPX   {paddw    x, m6}, m0, m1, m2, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    mova   [dstq+strideq*2], m2
    mova   [dstq+stride3q ], m3
    lea                dstq, [dstq+strideq*4]
    sub                  hq, 4*2
    jg .w32_loop
    RET
.w64:
    movu                 m4, [base+smooth_weights_1d_16bpc+64*2]
    movu                 m5, [base+smooth_weights_1d_16bpc+64*3]
.w64_loop:
    vpbroadcastw         m1, [tlq+hq-2]
    vpbroadcastw         m3, [tlq+hq-4]
    psubw                m1, m6
    psubw                m3, m6
    pmulhrsw             m0, m4, m1
    pmulhrsw             m1, m5
    pmulhrsw             m2, m4, m3
    pmulhrsw             m3, m5
    REPX      {paddw x, m6}, m0, m1, m2, m3
    mova [dstq+strideq*0+64*0], m0
    mova [dstq+strideq*0+64*1], m1
    mova [dstq+strideq*1+64*0], m2
    mova [dstq+strideq*1+64*1], m3
    lea                dstq, [dstq+strideq*2]
    sub                  hq, 2*2
    jg .w64_loop
    RET

cglobal ipred_smooth_16bpc, 3, 7, 16, dst, stride, tl, w, h, v_weights, stride3
    lea                 r6, [$$]
    mov                 wd, wm
    movifnidn           hd, hm
    vpbroadcastw       m13, [tlq+wq*2]   ; right
    tzcnt               wd, wd
    add                 hd, hd
    movsxd              wq, [base+ipred_smooth_16bpc_avx512icl_table+wq*4]
    mov                r5d, 0x55555555
    sub                tlq, hq
    mova               m14, [base+smooth_perm]
    kmovd               k1, r5d
    vpbroadcastw        m0, [tlq]        ; bottom
    mov                 r5, 0x3333333333333333
    pxor               m15, m15
    lea                 wq, [base+ipred_smooth_16bpc_avx512icl_table+wq]
    kmovq               k2, r5
    lea         v_weightsq, [base+smooth_weights_2d_16bpc+hq*2]
    jmp                 wq
.w4:
    vpbroadcastq        m5, [tlq+hq+2]
    movshdup            m3, [base+ipred_shuf]
    movsldup            m4, [base+ipred_shuf]
    vbroadcasti32x4     m6, [base+smooth_weights_2d_16bpc+4*4]
    lea           stride3q, [strideq*3]
    punpcklwd           m5, m0           ; top, bottom
.w4_loop:
    vbroadcasti32x4     m0, [v_weightsq]
    vpbroadcastq        m2, [tlq+hq-8]
    mova                m1, m13
    pshufb              m0, m3
    pmaddwd             m0, m5
    pshufb          m1{k2}, m2, m4       ; left, right
    vpdpwssd            m0, m1, m6
    vpermb              m0, m14, m0
    pavgw              ym0, ym15
    vextracti32x4      xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    lea               dstq, [dstq+strideq*4]
    add         v_weightsq, 4*4
    sub                 hd, 4*2
    jg .w4_loop
    RET
.w8:
    vbroadcasti32x4    ym5, [tlq+hq+2]
    movshdup            m6, [base+ipred_shuf]
    movsldup            m7, [base+ipred_shuf]
    pmovzxwd            m5, ym5
    vbroadcasti32x8     m8, [base+smooth_weights_2d_16bpc+8*4]
    lea           stride3q, [strideq*3]
    vpblendmw       m5{k1}, m0, m5       ; top, bottom
.w8_loop:
    vpbroadcastq        m0, [v_weightsq+0]
    vpbroadcastq        m1, [v_weightsq+8]
    vpbroadcastd        m3, [tlq+hq-4]
    vpbroadcastd        m4, [tlq+hq-8]
    pshufb              m0, m6
    pmaddwd             m0, m5
    pshufb              m1, m6
    pmaddwd             m1, m5
    mova                m2, m13
    pshufb          m2{k2}, m3, m7       ; left, right
    mova                m3, m13
    pshufb          m3{k2}, m4, m7
    vpdpwssd            m0, m2, m8
    vpdpwssd            m1, m3, m8
    add         v_weightsq, 4*4
    vpermt2b            m0, m14, m1
    pavgw               m0, m15
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea               dstq, [dstq+strideq*4]
    sub                 hd, 4*2
    jg .w8_loop
    RET
.w16:
    pmovzxwd            m5, [tlq+hq+2]
    mova                m6, [base+smooth_weights_2d_16bpc+16*4]
    vpblendmw       m5{k1}, m0, m5       ; top, bottom
.w16_loop:
    vpbroadcastd        m0, [v_weightsq+0]
    vpbroadcastd        m1, [v_weightsq+4]
    pmaddwd             m0, m5
    pmaddwd             m1, m5
    mova                m2, m13
    vpbroadcastw    m2{k1}, [tlq+hq-2] ; left, right
    mova                m3, m13
    vpbroadcastw    m3{k1}, [tlq+hq-4]
    vpdpwssd            m0, m2, m6
    vpdpwssd            m1, m3, m6
    add         v_weightsq, 2*4
    vpermt2b            m0, m14, m1
    pavgw               m0, m15
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea               dstq, [dstq+strideq*2]
    sub                 hq, 2*2
    jg .w16_loop
    RET
.w32:
    pmovzxwd            m5, [tlq+hq+ 2]
    pmovzxwd            m6, [tlq+hq+34]
    mova                m7, [base+smooth_weights_2d_16bpc+32*4]
    mova                m8, [base+smooth_weights_2d_16bpc+32*6]
    vpblendmw       m5{k1}, m0, m5       ; top, bottom
    vpblendmw       m6{k1}, m0, m6
.w32_loop:
    vpbroadcastd        m2, [v_weightsq+0]
    vpbroadcastd        m3, [v_weightsq+4]
    pmaddwd             m0, m5, m2
    pmaddwd             m2, m6
    pmaddwd             m1, m5, m3
    pmaddwd             m3, m6
    mova                m4, m13
    vpbroadcastw    m4{k1}, [tlq+hq-2] ; left, right
    vpdpwssd            m0, m4, m7
    vpdpwssd            m2, m4, m8
    mova                m4, m13
    vpbroadcastw    m4{k1}, [tlq+hq-4]
    vpdpwssd            m1, m4, m7
    vpdpwssd            m3, m4, m8
    add         v_weightsq, 2*4
    vpermt2b            m0, m14, m2
    vpermt2b            m1, m14, m3
    pavgw               m0, m15
    pavgw               m1, m15
    mova  [dstq+strideq*0], m0
    mova  [dstq+strideq*1], m1
    lea               dstq, [dstq+strideq*2]
    sub                 hq, 2*2
    jg .w32_loop
    RET
.w64:
    pmovzxwd            m5, [tlq+hq+ 2]
    pmovzxwd            m6, [tlq+hq+34]
    pmovzxwd            m7, [tlq+hq+66]
    pmovzxwd            m8, [tlq+hq+98]
    mova                m9, [base+smooth_weights_2d_16bpc+64*4]
    vpblendmw       m5{k1}, m0, m5       ; top, bottom
    mova               m10, [base+smooth_weights_2d_16bpc+64*5]
    vpblendmw       m6{k1}, m0, m6
    mova               m11, [base+smooth_weights_2d_16bpc+64*6]
    vpblendmw       m7{k1}, m0, m7
    mova               m12, [base+smooth_weights_2d_16bpc+64*7]
    vpblendmw       m8{k1}, m0, m8
.w64_loop:
    vpbroadcastd        m3, [v_weightsq]
    mova                m4, m13
    vpbroadcastw    m4{k1}, [tlq+hq-2] ; left, right
    pmaddwd             m0, m5, m3
    pmaddwd             m2, m6, m3
    pmaddwd             m1, m7, m3
    pmaddwd             m3, m8
    vpdpwssd            m0, m4, m9
    vpdpwssd            m2, m4, m10
    vpdpwssd            m1, m4, m11
    vpdpwssd            m3, m4, m12
    add         v_weightsq, 1*4
    vpermt2b            m0, m14, m2
    vpermt2b            m1, m14, m3
    pavgw               m0, m15
    pavgw               m1, m15
    mova       [dstq+64*0], m0
    mova       [dstq+64*1], m1
    add               dstq, strideq
    sub                 hd, 1*2
    jg .w64_loop
    RET

cglobal pal_pred_16bpc, 4, 7, 4, dst, stride, pal, idx, w, h, stride3
    lea                  r6, [pal_pred_16bpc_avx512icl_table]
    tzcnt                wd, wm
    mova                 m2, [pal_pred_perm]
    movsxd               wq, [r6+wq*4]
    mova                xm3, [palq]
    movifnidn            hd, hm
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    pmovzxbw            ym0, [idxq]
    add                idxq, 16
    vpermw              ym0, ym0, ym3
    vextracti32x4       xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w4
    RET
.w8:
    pmovzxbw             m0, [idxq]
    add                idxq, 32
    vpermw               m0, m0, m3
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w8
    RET
.w16:
    vpermb               m1, m2, [idxq]
    add                idxq, 64
    vpermw               m0, m1, m3
    psrlw                m1, 8
    vpermw               m1, m1, m3
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16
    RET
.w32:
    vpermb               m1, m2, [idxq]
    add                idxq, 64
    vpermw               m0, m1, m3
    psrlw                m1, 8
    vpermw               m1, m1, m3
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32
    RET
.w64:
    vpermb               m1, m2, [idxq]
    add                idxq, 64
    vpermw               m0, m1, m3
    psrlw                m1, 8
    vpermw               m1, m1, m3
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    add                dstq, strideq
    dec                  hd
    jg .w64
    RET

; The ipred_filter SIMD processes 4x2 blocks in the following order which
; increases parallelism compared to doing things row by row.
;     w4     w8       w16             w32
;     1     1 2     1 2 5 6     1 2 5 6 9 a d e
;     2     2 3     2 3 6 7     2 3 6 7 a b e f
;     3     3 4     3 4 7 8     3 4 7 8 b c f g
;     4     4 5     4 5 8 9     4 5 8 9 c d g h

cglobal ipred_filter_16bpc, 4, 7, 14, dst, stride, tl, w, h, filter, top
%define base r6-$$
    lea                  r6, [$$]
%ifidn filterd, filterm
    movzx           filterd, filterb
%else
    movzx           filterd, byte filterm
%endif
    shl             filterd, 6
    movifnidn            hd, hm
    movu                xm0, [tlq-6]
    pmovsxbw             m7, [base+filter_intra_taps+filterq+32*0]
    pmovsxbw             m8, [base+filter_intra_taps+filterq+32*1]
    mov                 r5d, r8m ; bitdepth_max
    movsldup             m9, [base+filter_permA]
    movshdup            m10, [base+filter_permA]
    shr                 r5d, 11  ; is_12bpc
    jnz .12bpc
    psllw                m7, 2   ; upshift multipliers so that packusdw
    psllw                m8, 2   ; will perform clipping for free
.12bpc:
    vpbroadcastd         m5, [base+filter_rnd+r5*8]
    vpbroadcastd         m6, [base+filter_shift+r5*8]
    sub                  wd, 8
    jl .w4
.w8:
    call .main4
    movsldup            m11, [filter_permB]
    lea                 r5d, [hq*2+2]
    movshdup            m12, [filter_permB]
    lea                topq, [tlq+2]
    mova                m13, [filter_permC]
    sub                  hd, 4
    vinserti32x4        ym0, [topq], 1 ; a0 b0   t0 t1
    sub                 tlq, r5
%if WIN64
    push                 r7
    push                 r8
%endif
    mov                  r7, dstq
    mov                 r8d, hd
.w8_loop:
    movlps              xm4, xm0, [tlq+hq*2]
    call .main8
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jge .w8_loop
    test                 wd, wd
    jz .end
    mov                 r2d, 0x0d
    kmovb                k1, r2d
    lea                  r2, [strideq*3]
.w16:
    movd               xmm0, [r7+strideq*1+12]
    vpblendd           xmm0, [topq+8], 0x0e ; t1 t2
    pinsrw              xm4, xmm0, [r7+strideq*0+14], 2
    call .main8
    add                  r7, 16
    vinserti32x4        ym0, [topq+16], 1   ; a2 b2   t2 t3
    mov                  hd, r8d
    mov                dstq, r7
    add                topq, 16
.w16_loop:
    movd               xmm1, [dstq+strideq*2-4]
    punpcklwd           xm4, xmm1, xmm0
    movd               xmm0, [dstq+r2-4]
    shufps          xm4{k1}, xmm0, xm0, q3210
    call .main8
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jge .w16_loop
    sub                  wd, 8
    jg .w16
.end:
    vpermb               m2, m11, m0
    mova                ym1, ym5
    vpdpwssd             m1, m2, m7
    vpermb               m2, m12, m0
    vpdpwssd             m1, m2, m8
%if WIN64
    pop                  r8
    pop                  r7
%endif
    vextracti32x8       ym2, m1, 1
    paddd               ym1, ym2
    packusdw            ym1, ym1
    vpsrlvw             ym1, ym6
    vpermt2q             m0, m13, m1
    vextracti32x4 [dstq+strideq*0], m0, 2
    vextracti32x4 [dstq+strideq*1], ym0, 1
    RET
.w4_loop:
    movlps              xm0, [tlq-10]
    lea                dstq, [dstq+strideq*2]
    sub                 tlq, 4
.w4:
    call .main4
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    sub                  hd, 2
    jg .w4_loop
    RET
ALIGN function_align
.main4:
    vpermb               m2, m9, m0
    mova                ym1, ym5
    vpdpwssd             m1, m2, m7
    vpermb               m0, m10, m0
    vpdpwssd             m1, m0, m8
    vextracti32x8       ym0, m1, 1
    paddd               ym0, ym1
    vextracti32x4       xm1, ym0, 1
    packusdw            xm0, xm1     ; clip
    vpsrlvw             xm0, xm6
    ret
ALIGN function_align
.main8:
    vpermb               m3, m11, m0
    mova                ym2, ym5
    vpdpwssd             m2, m3, m7
    vpermb               m3, m9, m4
    mova                ym1, ym5
    vpdpwssd             m1, m3, m7
    vpermb               m3, m12, m0
    vpdpwssd             m2, m3, m8
    vpermb               m3, m10, m4
    vpdpwssd             m1, m3, m8
    vextracti32x8       ym4, m2, 1
    vextracti32x8       ym3, m1, 1
    paddd               ym2, ym4
    paddd               ym1, ym3
    packusdw            ym1, ym2     ; clip
    vpsrlvw             ym1, ym6
    vpermt2q             m0, m13, m1 ; c0 d0   b0 b1   a0 a1
    vextracti32x4 [dstq+strideq*0], m0, 2
    vextracti32x4 [dstq+strideq*1], ym0, 1
    ret

%endif
