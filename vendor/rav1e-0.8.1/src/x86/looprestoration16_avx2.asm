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

SECTION_RODATA 32

sgr_lshuf3:    db  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11
sgr_lshuf5:    db  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9
wiener_shufA:  db  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11
wiener_shufB:  db  6,  7,  4,  5,  8,  9,  6,  7, 10, 11,  8,  9, 12, 13, 10, 11
wiener_shufC:  db  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13, 12, 13, 14, 15
wiener_shufD:  db  2,  3, -1, -1,  4,  5, -1, -1,  6,  7, -1, -1,  8,  9, -1, -1
wiener_shufE:  db  0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
wiener_lshuf5: db  4,  5,  4,  5,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
wiener_lshuf7: db  8,  9,  8,  9,  8,  9,  8,  9,  8,  9, 10, 11, 12, 13, 14, 15
pb_0to31:      db  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
               db 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31

wiener_hshift: dw 4, 4, 1, 1
wiener_vshift: dw 1024, 1024, 4096, 4096
wiener_round:  dd 1049600, 1048832

pb_m10_m9:     times 2 db -10, -9
pb_m6_m5:      times 2 db  -6, -5
pb_m2_m1:      times 2 db  -2, -1
pb_2_3:        times 2 db   2,  3
pb_6_7:        times 2 db   6,  7
pw_1023:       times 2 dw 1023
pd_8:          dd 8
pd_25:         dd 25
pd_4096:       dd 4096
pd_34816:      dd 34816
pd_m262128:    dd -262128
pd_0xf00800a4: dd 0xf00800a4
pd_0xf00801c7: dd 0xf00801c7

%define pw_256 sgr_lshuf5

cextern sgr_x_by_x_avx2

SECTION .text

DECLARE_REG_TMP 8, 7, 9, 11, 12, 13, 14 ; wiener ring buffer pointers

INIT_YMM avx2
cglobal wiener_filter7_16bpc, 4, 15, 16, -384*12-16, dst, stride, left, lpf, \
                                                     w, h, edge, flt
%define base t4-wiener_hshift
    mov           fltq, r6mp
    movifnidn       wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    mov            t3d, r8m ; pixel_max
    vbroadcasti128  m6, [wiener_shufA]
    vpbroadcastd   m12, [fltq+ 0] ; x0 x1
    lea             t4, [wiener_hshift]
    vbroadcasti128  m7, [wiener_shufB]
    add             wd, wd
    vpbroadcastd   m13, [fltq+ 4] ; x2 x3
    shr            t3d, 11
    vpbroadcastd   m14, [fltq+16] ; y0 y1
    add           lpfq, wq
    vpbroadcastd   m15, [fltq+20] ; y2 y3
    add           dstq, wq
    vbroadcasti128  m8, [wiener_shufC]
    lea             t1, [rsp+wq+16]
    vbroadcasti128  m9, [wiener_shufD]
    neg             wq
    vpbroadcastd    m0, [base+wiener_hshift+t3*4]
    vpbroadcastd   m10, [base+wiener_round+t3*4]
    vpbroadcastd   m11, [base+wiener_vshift+t3*4]
    pmullw         m12, m0 ; upshift filter coefs to make the
    pmullw         m13, m0 ; horizontal downshift constant
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t6, t1
    mov             t5, t1
    add             t1, 384*2
    call .h_top
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    mov             t4, t1
    add             t1, 384*2
    add            r10, strideq
    mov          [rsp], r10 ; below
    call .h
    mov             t3, t1
    mov             t2, t1
    dec             hd
    jz .v1
    add           lpfq, strideq
    add             t1, 384*2
    call .h
    mov             t2, t1
    dec             hd
    jz .v2
    add           lpfq, strideq
    add             t1, 384*2
    call .h
    dec             hd
    jz .v3
.main:
    lea             t0, [t1+384*2]
.main_loop:
    call .hv
    dec             hd
    jnz .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .v3
    mov           lpfq, [rsp]
    call .hv_bottom
    add           lpfq, strideq
    call .hv_bottom
.v1:
    call .v
    RET
.no_top:
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov          [rsp], r10
    call .h
    mov             t6, t1
    mov             t5, t1
    mov             t4, t1
    mov             t3, t1
    mov             t2, t1
    dec             hd
    jz .v1
    add           lpfq, strideq
    add             t1, 384*2
    call .h
    mov             t2, t1
    dec             hd
    jz .v2
    add           lpfq, strideq
    add             t1, 384*2
    call .h
    dec             hd
    jz .v3
    lea             t0, [t1+384*2]
    call .hv
    dec             hd
    jz .v3
    add             t0, 384*8
    call .hv
    dec             hd
    jnz .main
.v3:
    call .v
.v2:
    call .v
    jmp .v1
.extend_right:
    movd           xm1, r10d
    vpbroadcastd    m0, [pb_6_7]
    movu            m2, [pb_0to31]
    vpbroadcastb    m1, xm1
    psubb           m0, m1
    pminub          m0, m2
    pshufb          m3, m0
    vpbroadcastd    m0, [pb_m2_m1]
    psubb           m0, m1
    pminub          m0, m2
    pshufb          m4, m0
    vpbroadcastd    m0, [pb_m10_m9]
    psubb           m0, m1
    pminub          m0, m2
    pshufb          m5, m0
    ret
.h:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movq           xm3, [leftq]
    vpblendd        m3, [lpfq+r10-8], 0xfc
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    vbroadcasti128  m3, [lpfq+r10] ; avoid accessing memory located
    mova            m4, [lpfq+r10] ; before the start of the buffer
    shufpd          m3, m4, 0x05
    pshufb          m3, [wiener_lshuf7]
    jmp .h_main2
.h_top:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m3, [lpfq+r10-8]
.h_main:
    mova            m4, [lpfq+r10+0]
.h_main2:
    movu            m5, [lpfq+r10+8]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -36
    jl .h_have_right
    call .extend_right
.h_have_right:
    pshufb          m0, m3, m6
    pshufb          m1, m4, m7
    paddw           m0, m1
    pshufb          m3, m8
    pmaddwd         m0, m12
    pshufb          m1, m4, m9
    paddw           m3, m1
    pshufb          m1, m4, m6
    pmaddwd         m3, m13
    pshufb          m2, m5, m7
    paddw           m1, m2
    vpbroadcastd    m2, [pd_m262128] ; (1 << 4) - (1 << 18)
    pshufb          m4, m8
    pmaddwd         m1, m12
    pshufb          m5, m9
    paddw           m4, m5
    pmaddwd         m4, m13
    paddd           m0, m2
    paddd           m1, m2
    paddd           m0, m3
    paddd           m1, m4
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    psraw           m0, 1
    mova      [t1+r10], m0
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movq           xm3, [leftq]
    vpblendd        m3, [lpfq+r10-8], 0xfc
    add          leftq, 8
    jmp .hv_main
.hv_extend_left:
    movu            m3, [lpfq+r10-8]
    pshufb          m3, [wiener_lshuf7]
    jmp .hv_main
.hv_bottom:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m3, [lpfq+r10-8]
.hv_main:
    mova            m4, [lpfq+r10+0]
    movu            m5, [lpfq+r10+8]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -36
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    pshufb          m0, m3, m6
    pshufb          m1, m4, m7
    paddw           m0, m1
    pshufb          m3, m8
    pmaddwd         m0, m12
    pshufb          m1, m4, m9
    paddw           m3, m1
    pshufb          m1, m4, m6
    pmaddwd         m3, m13
    pshufb          m2, m5, m7
    paddw           m1, m2
    vpbroadcastd    m2, [pd_m262128]
    pshufb          m4, m8
    pmaddwd         m1, m12
    pshufb          m5, m9
    paddw           m4, m5
    pmaddwd         m4, m13
    paddd           m0, m2
    paddd           m1, m2
    mova            m2, [t4+r10]
    paddw           m2, [t2+r10]
    mova            m5, [t3+r10]
    paddd           m0, m3
    paddd           m1, m4
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    mova            m4, [t5+r10]
    paddw           m4, [t1+r10]
    psraw           m0, 1
    paddw           m3, m0, [t6+r10]
    mova      [t0+r10], m0
    punpcklwd       m0, m2, m5
    pmaddwd         m0, m15
    punpckhwd       m2, m5
    pmaddwd         m2, m15
    punpcklwd       m1, m3, m4
    pmaddwd         m1, m14
    punpckhwd       m3, m4
    pmaddwd         m3, m14
    paddd           m0, m10
    paddd           m2, m10
    paddd           m0, m1
    paddd           m2, m3
    psrad           m0, 5
    psrad           m2, 5
    packusdw        m0, m2
    pmulhuw         m0, m11
    mova    [dstq+r10], m0
    add            r10, 32
    jl .hv_loop
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, t6
    add           dstq, strideq
    ret
.v:
    mov            r10, wq
.v_loop:
    mova            m1, [t4+r10]
    paddw           m1, [t2+r10]
    mova            m2, [t3+r10]
    mova            m4, [t1+r10]
    paddw           m3, m4, [t6+r10]
    paddw           m4, [t5+r10]
    punpcklwd       m0, m1, m2
    pmaddwd         m0, m15
    punpckhwd       m1, m2
    pmaddwd         m1, m15
    punpcklwd       m2, m3, m4
    pmaddwd         m2, m14
    punpckhwd       m3, m4
    pmaddwd         m3, m14
    paddd           m0, m10
    paddd           m1, m10
    paddd           m0, m2
    paddd           m1, m3
    psrad           m0, 5
    psrad           m1, 5
    packusdw        m0, m1
    pmulhuw         m0, m11
    mova    [dstq+r10], m0
    add            r10, 32
    jl .v_loop
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    add           dstq, strideq
    ret

cglobal wiener_filter5_16bpc, 4, 13, 16, 384*8+16, dst, stride, left, lpf, \
                                                   w, h, edge, flt
%define base t4-wiener_hshift
    mov           fltq, r6mp
    movifnidn       wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    mov            t3d, r8m ; pixel_max
    vbroadcasti128  m5, [wiener_shufE]
    vpbroadcastw   m11, [fltq+ 2] ; x1
    vbroadcasti128  m6, [wiener_shufB]
    lea             t4, [wiener_hshift]
    vbroadcasti128  m7, [wiener_shufD]
    add             wd, wd
    vpbroadcastd   m12, [fltq+ 4] ; x2 x3
    shr            t3d, 11
    vpbroadcastd    m8, [pd_m262128] ; (1 << 4) - (1 << 18)
    add           lpfq, wq
    vpbroadcastw   m13, [fltq+18] ; y1
    add           dstq, wq
    vpbroadcastd   m14, [fltq+20] ; y2 y3
    lea             t1, [rsp+wq+16]
    neg             wq
    vpbroadcastd    m0, [base+wiener_hshift+t3*4]
    vpbroadcastd    m9, [base+wiener_round+t3*4]
    vpbroadcastd   m10, [base+wiener_vshift+t3*4]
    movu          xm15, [wiener_lshuf5]
    pmullw         m11, m0
    vinserti128    m15, [pb_0to31], 1
    pmullw         m12, m0
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t4, t1
    add             t1, 384*2
    call .h_top
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    mov             t3, t1
    add             t1, 384*2
    add            r10, strideq
    mov          [rsp], r10 ; below
    call .h
    mov             t2, t1
    dec             hd
    jz .v1
    add           lpfq, strideq
    add             t1, 384*2
    call .h
    dec             hd
    jz .v2
.main:
    mov             t0, t4
.main_loop:
    call .hv
    dec             hd
    jnz .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .v2
    mov           lpfq, [rsp]
    call .hv_bottom
    add           lpfq, strideq
    call .hv_bottom
.end:
    RET
.no_top:
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov          [rsp], r10
    call .h
    mov             t4, t1
    mov             t3, t1
    mov             t2, t1
    dec             hd
    jz .v1
    add           lpfq, strideq
    add             t1, 384*2
    call .h
    dec             hd
    jz .v2
    lea             t0, [t1+384*2]
    call .hv
    dec             hd
    jz .v2
    add             t0, 384*6
    call .hv
    dec             hd
    jnz .main
.v2:
    call .v
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    add           dstq, strideq
.v1:
    call .v
    jmp .end
.extend_right:
    movd           xm2, r10d
    vpbroadcastd    m0, [pb_2_3]
    vpbroadcastd    m1, [pb_m6_m5]
    vpbroadcastb    m2, xm2
    psubb           m0, m2
    psubb           m1, m2
    movu            m2, [pb_0to31]
    pminub          m0, m2
    pminub          m1, m2
    pshufb          m3, m0
    pshufb          m4, m1
    ret
.h:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movd           xm3, [leftq+4]
    vpblendd        m3, [lpfq+r10-4], 0xfe
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    vbroadcasti128  m4, [lpfq+r10] ; avoid accessing memory located
    mova            m3, [lpfq+r10] ; before the start of the buffer
    palignr         m3, m4, 12
    pshufb          m3, m15
    jmp .h_main
.h_top:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m3, [lpfq+r10-4]
.h_main:
    movu            m4, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -34
    jl .h_have_right
    call .extend_right
.h_have_right:
    pshufb          m0, m3, m5
    pmaddwd         m0, m11
    pshufb          m1, m4, m5
    pmaddwd         m1, m11
    pshufb          m2, m3, m6
    pshufb          m3, m7
    paddw           m2, m3
    pshufb          m3, m4, m6
    pmaddwd         m2, m12
    pshufb          m4, m7
    paddw           m3, m4
    pmaddwd         m3, m12
    paddd           m0, m8
    paddd           m1, m8
    paddd           m0, m2
    paddd           m1, m3
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    psraw           m0, 1
    mova      [t1+r10], m0
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movd           xm3, [leftq+4]
    vpblendd        m3, [lpfq+r10-4], 0xfe
    add          leftq, 8
    jmp .hv_main
.hv_extend_left:
    movu            m3, [lpfq+r10-4]
    pshufb          m3, m15
    jmp .hv_main
.hv_bottom:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m3, [lpfq+r10-4]
.hv_main:
    movu            m4, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -34
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    pshufb          m0, m3, m5
    pmaddwd         m0, m11
    pshufb          m1, m4, m5
    pmaddwd         m1, m11
    pshufb          m2, m3, m6
    pshufb          m3, m7
    paddw           m2, m3
    pshufb          m3, m4, m6
    pmaddwd         m2, m12
    pshufb          m4, m7
    paddw           m3, m4
    pmaddwd         m3, m12
    paddd           m0, m8
    paddd           m1, m8
    paddd           m0, m2
    mova            m2, [t3+r10]
    paddw           m2, [t1+r10]
    paddd           m1, m3
    mova            m4, [t2+r10]
    punpckhwd       m3, m2, m4
    pmaddwd         m3, m14
    punpcklwd       m2, m4
    mova            m4, [t4+r10]
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    pmaddwd         m2, m14
    psraw           m0, 1
    mova      [t0+r10], m0
    punpckhwd       m1, m0, m4
    pmaddwd         m1, m13
    punpcklwd       m0, m4
    pmaddwd         m0, m13
    paddd           m3, m9
    paddd           m2, m9
    paddd           m1, m3
    paddd           m0, m2
    psrad           m1, 5
    psrad           m0, 5
    packusdw        m0, m1
    pmulhuw         m0, m10
    mova    [dstq+r10], m0
    add            r10, 32
    jl .hv_loop
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, t4
    add           dstq, strideq
    ret
.v:
    mov            r10, wq
.v_loop:
    mova            m0, [t1+r10]
    paddw           m2, m0, [t3+r10]
    mova            m1, [t2+r10]
    mova            m4, [t4+r10]
    punpckhwd       m3, m2, m1
    pmaddwd         m3, m14
    punpcklwd       m2, m1
    pmaddwd         m2, m14
    punpckhwd       m1, m0, m4
    pmaddwd         m1, m13
    punpcklwd       m0, m4
    pmaddwd         m0, m13
    paddd           m3, m9
    paddd           m2, m9
    paddd           m1, m3
    paddd           m0, m2
    psrad           m1, 5
    psrad           m0, 5
    packusdw        m0, m1
    pmulhuw         m0, m10
    mova    [dstq+r10], m0
    add            r10, 32
    jl .v_loop
    ret

cglobal sgr_filter_5x5_16bpc, 4, 14, 15, 400*24+16, dst, stride, left, lpf, \
                                                    w, h, edge, params
    movifnidn       wd, wm
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x_avx2+256*4]
    movifnidn       hd, hm
    mov          edged, r7m
    add             wd, wd
    vpbroadcastw    m7, [paramsq+8] ; w0
    add           lpfq, wq
    vpbroadcastd    m8, [pd_8]
    add           dstq, wq
    vpbroadcastd    m9, [pd_25]
    lea             t3, [rsp+wq*2+400*12+16]
    vpbroadcastd   m10, [paramsq+0] ; s0
    lea             t4, [rsp+wq+400*20+16]
    vpbroadcastd   m11, [pd_0xf00800a4]
    lea             t1, [rsp+wq+20]
    mova          xm12, [sgr_lshuf5]
    neg             wq
    vpbroadcastd   m13, [pd_34816]  ; (1 << 11) + (1 << 15)
    pxor            m6, m6
    vpbroadcastd   m14, [pw_1023]
    psllw           m7, 4
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    call .top_fixup
    add             t1, 400*6
    call .h_top
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    add            r10, strideq
    mov          [rsp], r10 ; below
    mov             t0, t2
    dec             hd
    jz .height1
    or           edged, 16
    call .h
.main:
    add           lpfq, strideq
    call .hv
    call .prep_n
    sub             hd, 2
    jl .extend_bottom
.main_loop:
    add           lpfq, strideq
    test            hd, hd
    jz .odd_height
    call .h
    add           lpfq, strideq
    call .hv
    call .n0
    call .n1
    sub             hd, 2
    jge .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .extend_bottom
    mov           lpfq, [rsp]
    call .h_top
    add           lpfq, strideq
    call .hv_bottom
.end:
    call .n0
    call .n1
.end2:
    RET
.height1:
    call .hv
    call .prep_n
    jmp .odd_height_end
.odd_height:
    call .hv
    call .n0
    call .n1
.odd_height_end:
    call .v
    call .n0
    jmp .end2
.extend_bottom:
    call .v
    jmp .end
.no_top:
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov          [rsp], r10
    call .h
    lea             t2, [t1+400*6]
    call .top_fixup
    dec             hd
    jz .no_top_height1
    or           edged, 16
    mov             t0, t1
    mov             t1, t2
    jmp .main
.no_top_height1:
    call .v
    call .prep_n
    jmp .odd_height_end
.extend_right:
    vpbroadcastw    m0, [lpfq-2]
    movu            m1, [r13+r10+ 0]
    movu            m2, [r13+r10+16]
    vpblendvb       m4, m0, m1
    vpblendvb       m5, m0, m2
    ret
.h: ; horizontal boxsum
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 10
    jmp .h_main
.h_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, xm12
    vinserti128     m4, [lpfq+wq+10], 1
    jmp .h_main
.h_top:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m4, [lpfq+r10- 2]
.h_main:
    movu            m5, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -36
    jl .h_have_right
    call .extend_right
.h_have_right:
    palignr         m2, m5, m4, 2
    paddw           m0, m4, m2
    palignr         m3, m5, m4, 6
    paddw           m0, m3
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m1
    punpckhwd       m2, m3
    pmaddwd         m2, m2
    shufpd          m5, m4, m5, 0x05
    paddw           m0, m5
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m3
    paddd           m1, m3
    punpckhwd       m3, m4, m5
    pmaddwd         m3, m3
    shufps          m4, m5, q2121
    paddw           m0, m4             ; sum
    punpcklwd       m5, m4, m6
    pmaddwd         m5, m5
    punpckhwd       m4, m6
    pmaddwd         m4, m4
    paddd           m2, m3
    test         edgeb, 16             ; y > 0
    jz .h_loop_end
    paddw           m0, [t1+r10+400*0]
    paddd           m1, [t1+r10+400*2]
    paddd           m2, [t1+r10+400*4]
.h_loop_end:
    paddd           m1, m5             ; sumsq
    paddd           m2, m4
    mova [t1+r10+400*0], m0
    mova [t1+r10+400*2], m1
    mova [t1+r10+400*4], m2
    add            r10, 32
    jl .h_loop
    ret
.top_fixup:
    lea            r10, [wq-4]
.top_fixup_loop: ; the sums of the first row needs to be doubled
    mova            m0, [t1+r10+400*0]
    mova            m1, [t1+r10+400*2]
    mova            m2, [t1+r10+400*4]
    paddw           m0, m0
    paddd           m1, m1
    paddd           m2, m2
    mova [t2+r10+400*0], m0
    mova [t2+r10+400*2], m1
    mova [t2+r10+400*4], m2
    add            r10, 32
    jl .top_fixup_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 10
    jmp .hv_main
.hv_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, xm12
    vinserti128     m4, [lpfq+wq+10], 1
    jmp .hv_main
.hv_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m4, [lpfq+r10- 2]
.hv_main:
    movu            m5, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -36
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    palignr         m3, m5, m4, 2
    paddw           m0, m4, m3
    palignr         m1, m5, m4, 6
    paddw           m0, m1
    punpcklwd       m2, m3, m1
    pmaddwd         m2, m2
    punpckhwd       m3, m1
    pmaddwd         m3, m3
    shufpd          m5, m4, m5, 0x05
    paddw           m0, m5
    punpcklwd       m1, m4, m5
    pmaddwd         m1, m1
    paddd           m2, m1
    punpckhwd       m1, m4, m5
    pmaddwd         m1, m1
    shufps          m4, m5, q2121
    paddw           m0, m4            ; h sum
    punpcklwd       m5, m4, m6
    pmaddwd         m5, m5
    punpckhwd       m4, m6
    pmaddwd         m4, m4
    paddd           m3, m1
    paddd           m2, m5            ; h sumsq
    paddd           m3, m4
    paddw           m1, m0, [t1+r10+400*0]
    paddd           m4, m2, [t1+r10+400*2]
    paddd           m5, m3, [t1+r10+400*4]
    test            hd, hd
    jz .hv_last_row
.hv_main2:
    paddw           m1, [t2+r10+400*0] ; hv sum
    paddd           m4, [t2+r10+400*2] ; hv sumsq
    paddd           m5, [t2+r10+400*4]
    mova [t0+r10+400*0], m0
    mova [t0+r10+400*2], m2
    mova [t0+r10+400*4], m3
    psrlw           m3, m1, 1
    paddd           m4, m8
    pavgw           m3, m6             ; (b + 2) >> 2
    paddd           m5, m8
    psrld           m4, 4              ; (a + 8) >> 4
    punpcklwd       m2, m3, m6
    psrld           m5, 4
    punpckhwd       m3, m6
    pmulld          m4, m9             ; a * 25
    pmulld          m5, m9
    pmaddwd         m2, m2             ; b * b
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmaxud          m4, m2
    pmaxud          m5, m3
    psubd           m4, m2             ; p
    psubd           m5, m3
    pmulld          m4, m10            ; p * s
    pmulld          m5, m10
    pmaddwd         m0, m11            ; b * 164
    pmaddwd         m1, m11
    paddusw         m4, m11
    paddusw         m5, m11
    psrad           m3, m4, 20         ; min(z, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m13            ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m13
    mova    [t4+r10+4], m2
    psrld           m0, 12             ; b
    psrld           m1, 12
    mova         [t3+r10*2+ 8], xm0
    vextracti128 [t3+r10*2+40], m0, 1
    mova         [t3+r10*2+24], xm1
    vextracti128 [t3+r10*2+56], m1, 1
    add            r10, 32
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    ret
.hv_last_row: ; esoteric edge case for odd heights
    mova [t1+r10+400*0], m1
    paddw            m1, m0
    mova [t1+r10+400*2], m4
    paddd            m4, m2
    mova [t1+r10+400*4], m5
    paddd            m5, m3
    jmp .hv_main2
.v: ; vertical boxsum + ab
    lea            r10, [wq-4]
.v_loop:
    mova            m0, [t1+r10+400*0]
    mova            m2, [t1+r10+400*2]
    mova            m3, [t1+r10+400*4]
    paddw           m1, m0, [t2+r10+400*0]
    paddd           m4, m2, [t2+r10+400*2]
    paddd           m5, m3, [t2+r10+400*4]
    paddw           m0, m0
    paddd           m2, m2
    paddd           m3, m3
    paddw           m1, m0             ; hv sum
    paddd           m4, m2             ; hv sumsq
    paddd           m5, m3
    psrlw           m3, m1, 1
    paddd           m4, m8
    pavgw           m3, m6             ; (b + 2) >> 2
    paddd           m5, m8
    psrld           m4, 4              ; (a + 8) >> 4
    punpcklwd       m2, m3, m6
    psrld           m5, 4
    punpckhwd       m3, m6
    pmulld          m4, m9             ; a * 25
    pmulld          m5, m9
    pmaddwd         m2, m2             ; b * b
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmaxud          m4, m2
    pmaxud          m5, m3
    psubd           m4, m2             ; p
    psubd           m5, m3
    pmulld          m4, m10            ; p * s
    pmulld          m5, m10
    pmaddwd         m0, m11            ; b * 164
    pmaddwd         m1, m11
    paddusw         m4, m11
    paddusw         m5, m11
    psrad           m3, m4, 20         ; min(z, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m13            ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m13
    mova    [t4+r10+4], m2
    psrld           m0, 12             ; b
    psrld           m1, 12
    mova         [t3+r10*2+ 8], xm0
    vextracti128 [t3+r10*2+40], m0, 1
    mova         [t3+r10*2+24], xm1
    vextracti128 [t3+r10*2+56], m1, 1
    add            r10, 32
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu            m0, [t4+r10*1+ 2]
    movu            m1, [t3+r10*2+ 4]
    movu            m2, [t3+r10*2+36]
    paddw           m3, m0, [t4+r10*1+ 0]
    paddd           m4, m1, [t3+r10*2+ 0]
    paddd           m5, m2, [t3+r10*2+32]
    paddw           m3, [t4+r10*1+ 4]
    paddd           m4, [t3+r10*2+ 8]
    paddd           m5, [t3+r10*2+40]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m4
    pslld           m4, 2
    paddd           m2, m5
    pslld           m5, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m4             ; b 565
    paddd           m2, m5
    mova [t4+r10*1+400*2+ 0], m0
    mova [t3+r10*2+400*4+ 0], m1
    mova [t3+r10*2+400*4+32], m2
    add            r10, 32
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu            m0, [t4+r10*1+ 2]
    movu            m1, [t3+r10*2+ 4]
    movu            m2, [t3+r10*2+36]
    paddw           m3, m0, [t4+r10*1+ 0]
    paddd           m4, m1, [t3+r10*2+ 0]
    paddd           m5, m2, [t3+r10*2+32]
    paddw           m3, [t4+r10*1+ 4]
    paddd           m4, [t3+r10*2+ 8]
    paddd           m5, [t3+r10*2+40]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m4
    pslld           m4, 2
    paddd           m2, m5
    pslld           m5, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m4             ; b 565
    paddd           m2, m5
    paddw           m3, m0, [t4+r10*1+400*2+ 0]
    paddd           m4, m1, [t3+r10*2+400*4+ 0]
    paddd           m5, m2, [t3+r10*2+400*4+32]
    mova [t4+r10*1+400*2+ 0], m0
    mova [t3+r10*2+400*4+ 0], m1
    mova [t3+r10*2+400*4+32], m2
    mova            m0, [dstq+r10]
    punpcklwd       m1, m0, m6          ; src
    punpcklwd       m2, m3, m6          ; a
    pmaddwd         m2, m1              ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vinserti128     m1, m4, xm5, 1
    vperm2i128      m4, m5, 0x31
    psubd           m1, m2              ; b - a * src + (1 << 8)
    psubd           m4, m3
    psrad           m1, 9
    psrad           m4, 9
    packssdw        m1, m4
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova    [dstq+r10], m0
    add            r10, 32
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova            m0, [dstq+r10]
    mova            m3, [t4+r10*1+400*2+ 0]
    mova            m4, [t3+r10*2+400*4+ 0]
    mova            m5, [t3+r10*2+400*4+32]
    punpcklwd       m1, m0, m6          ; src
    punpcklwd       m2, m3, m6          ; a
    pmaddwd         m2, m1
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vinserti128     m1, m4, xm5, 1
    vperm2i128      m4, m5, 0x31
    psubd           m1, m2              ; b - a * src + (1 << 7)
    psubd           m4, m3
    psrad           m1, 8
    psrad           m4, 8
    packssdw        m1, m4
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova    [dstq+r10], m0
    add            r10, 32
    jl .n1_loop
    add           dstq, strideq
    ret

cglobal sgr_filter_3x3_16bpc, 4, 14, 14, 400*42+8, dst, stride, left, lpf, \
                                                   w, h, edge, params
    movifnidn       wd, wm
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x_avx2+256*4]
    add             wd, wd
    movifnidn       hd, hm
    mov          edged, r7m
    add           lpfq, wq
    vpbroadcastw    m7, [paramsq+10] ; w1
    add           dstq, wq
    vpbroadcastd    m9, [paramsq+ 4] ; s1
    lea             t3, [rsp+wq*2+400*12+8]
    vpbroadcastd    m8, [pd_8]
    lea             t4, [rsp+wq+400*32+8]
    vpbroadcastd   m10, [pd_0xf00801c7]
    lea             t1, [rsp+wq+12]
    vpbroadcastd   m11, [pd_34816]
    neg             wq
    mova          xm12, [sgr_lshuf3]
    pxor            m6, m6
    vpbroadcastd   m13, [pw_1023]
    psllw           m7, 4
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    add             t1, 400*6
    call .h_top
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    add            r10, strideq
    mov          [rsp], r10 ; below
    call .hv0
.main:
    dec             hd
    jz .height1
    add           lpfq, strideq
    call .hv1
    call .prep_n
    sub             hd, 2
    jl .extend_bottom
.main_loop:
    add           lpfq, strideq
    call .hv0
    test            hd, hd
    jz .odd_height
    add           lpfq, strideq
    call .hv1
    call .n0
    call .n1
    sub             hd, 2
    jge .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .extend_bottom
    mov           lpfq, [rsp]
    call .hv0_bottom
    add           lpfq, strideq
    call .hv1_bottom
.end:
    call .n0
    call .n1
.end2:
    RET
.height1:
    call .v1
    call .prep_n
    jmp .odd_height_end
.odd_height:
    call .v1
    call .n0
    call .n1
.odd_height_end:
    call .v0
    call .v1
    call .n0
    jmp .end2
.extend_bottom:
    call .v0
    call .v1
    jmp .end
.no_top:
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov          [rsp], r10
    call .h
    lea            r10, [wq-4]
    lea             t2, [t1+400*6]
.top_fixup_loop:
    mova            m0, [t1+r10+400*0]
    mova            m1, [t1+r10+400*2]
    mova            m2, [t1+r10+400*4]
    mova [t2+r10+400*0], m0
    mova [t2+r10+400*2], m1
    mova [t2+r10+400*4], m2
    add            r10, 32
    jl .top_fixup_loop
    call .v0
    jmp .main
.extend_right:
    vpbroadcastw    m0, [lpfq-2]
    movu            m1, [r13+r10+ 2]
    movu            m2, [r13+r10+18]
    vpblendvb       m4, m0, m1
    vpblendvb       m5, m0, m2
    ret
.h: ; horizontal boxsum
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 12
    jmp .h_main
.h_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, xm12
    vinserti128     m4, [lpfq+wq+12], 1
    jmp .h_main
.h_top:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m4, [lpfq+r10+ 0]
.h_main:
    movu            m5, [lpfq+r10+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -34
    jl .h_have_right
    call .extend_right
.h_have_right:
    palignr         m0, m5, m4, 2
    paddw           m1, m4, m0
    punpcklwd       m2, m4, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m4, m0
    pmaddwd         m3, m3
    palignr         m5, m4, 4
    paddw           m1, m5             ; sum
    punpcklwd       m4, m5, m6
    pmaddwd         m4, m4
    punpckhwd       m5, m6
    pmaddwd         m5, m5
    paddd           m2, m4             ; sumsq
    paddd           m3, m5
    mova [t1+r10+400*0], m1
    mova [t1+r10+400*2], m2
    mova [t1+r10+400*4], m3
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab (even rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 12
    jmp .hv0_main
.hv0_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, xm12
    vinserti128     m4, [lpfq+wq+12], 1
    jmp .hv0_main
.hv0_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
.hv0_loop:
    movu            m4, [lpfq+r10+ 0]
.hv0_main:
    movu            m5, [lpfq+r10+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp           r10d, -34
    jl .hv0_have_right
    call .extend_right
.hv0_have_right:
    palignr         m0, m5, m4, 2
    paddw           m1, m4, m0
    punpcklwd       m2, m4, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m4, m0
    pmaddwd         m3, m3
    palignr         m5, m4, 4
    paddw           m1, m5             ; sum
    punpcklwd       m4, m5, m6
    pmaddwd         m4, m4
    punpckhwd       m5, m6
    pmaddwd         m5, m5
    paddd           m2, m4             ; sumsq
    paddd           m3, m5
    paddw           m0, m1, [t1+r10+400*0]
    paddd           m4, m2, [t1+r10+400*2]
    paddd           m5, m3, [t1+r10+400*4]
    mova [t1+r10+400*0], m1
    mova [t1+r10+400*2], m2
    mova [t1+r10+400*4], m3
    paddw           m1, m0, [t2+r10+400*0]
    paddd           m2, m4, [t2+r10+400*2]
    paddd           m3, m5, [t2+r10+400*4]
    mova [t2+r10+400*0], m0
    mova [t2+r10+400*2], m4
    mova [t2+r10+400*4], m5
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a + 8) >> 4
    psrld           m3, 4
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; ((a + 8) >> 4) * 9
    paddd           m5, m3
    psrlw           m3, m1, 1
    pavgw           m3, m6             ; (b + 2) >> 2
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m2
    punpckhwd       m3, m6
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmaxud          m4, m2
    psubd           m4, m2             ; p
    pmaxud          m5, m3
    psubd           m5, m3
    pmulld          m4, m9             ; p * s
    pmulld          m5, m9
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrad           m3, m4, 20         ; min(z, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    psrld           m0, 12
    psrld           m1, 12
    mova         [t4+r10*1+400*0+ 4], m2
    mova         [t3+r10*2+400*0+ 8], xm0
    vextracti128 [t3+r10*2+400*0+40], m0, 1
    mova         [t3+r10*2+400*0+24], xm1
    vextracti128 [t3+r10*2+400*0+56], m1, 1
    add            r10, 32
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 12
    jmp .hv1_main
.hv1_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, xm12
    vinserti128     m4, [lpfq+wq+12], 1
    jmp .hv1_main
.hv1_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
.hv1_loop:
    movu            m4, [lpfq+r10+ 0]
.hv1_main:
    movu            m5, [lpfq+r10+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp           r10d, -34
    jl .hv1_have_right
    call .extend_right
.hv1_have_right:
    palignr         m1, m5, m4, 2
    paddw           m0, m4, m1
    punpcklwd       m2, m4, m1
    pmaddwd         m2, m2
    punpckhwd       m3, m4, m1
    pmaddwd         m3, m3
    palignr         m5, m4, 4
    paddw           m0, m5             ; h sum
    punpcklwd       m1, m5, m6
    pmaddwd         m1, m1
    punpckhwd       m5, m6
    pmaddwd         m5, m5
    paddd           m2, m1             ; h sumsq
    paddd           m3, m5
    paddw           m1, m0, [t2+r10+400*0]
    paddd           m4, m2, [t2+r10+400*2]
    paddd           m5, m3, [t2+r10+400*4]
    mova [t2+r10+400*0], m0
    mova [t2+r10+400*2], m2
    mova [t2+r10+400*4], m3
    paddd           m4, m8
    paddd           m5, m8
    psrld           m4, 4              ; (a + 8) >> 4
    psrld           m5, 4
    pslld           m2, m4, 3
    pslld           m3, m5, 3
    paddd           m4, m2             ; ((a + 8) >> 4) * 9
    paddd           m5, m3
    psrlw           m3, m1, 1
    pavgw           m3, m6             ; (b + 2) >> 2
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m2
    punpckhwd       m3, m6
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmaxud          m4, m2
    psubd           m4, m2             ; p
    pmaxud          m5, m3
    psubd           m5, m3
    pmulld          m4, m9             ; p * s
    pmulld          m5, m9
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrad           m3, m4, 20         ; min(z, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    psrld           m0, 12
    psrld           m1, 12
    mova         [t4+r10*1+400*2 +4], m2
    mova         [t3+r10*2+400*4+ 8], xm0
    vextracti128 [t3+r10*2+400*4+40], m0, 1
    mova         [t3+r10*2+400*4+24], xm1
    vextracti128 [t3+r10*2+400*4+56], m1, 1
    add            r10, 32
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab (even rows)
    lea            r10, [wq-4]
.v0_loop:
    mova            m0, [t1+r10+400*0]
    mova            m4, [t1+r10+400*2]
    mova            m5, [t1+r10+400*4]
    paddw           m0, m0
    paddd           m4, m4
    paddd           m5, m5
    paddw           m1, m0, [t2+r10+400*0]
    paddd           m2, m4, [t2+r10+400*2]
    paddd           m3, m5, [t2+r10+400*4]
    mova [t2+r10+400*0], m0
    mova [t2+r10+400*2], m4
    mova [t2+r10+400*4], m5
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a + 8) >> 4
    psrld           m3, 4
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; ((a + 8) >> 4) * 9
    paddd           m5, m3
    psrlw           m3, m1, 1
    pavgw           m3, m6             ; (b + 2) >> 2
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m2
    punpckhwd       m3, m6
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmaxud          m4, m2
    psubd           m4, m2             ; p
    pmaxud          m5, m3
    psubd           m5, m3
    pmulld          m4, m9             ; p * s
    pmulld          m5, m9
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrad           m3, m4, 20         ; min(z, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    psrld           m0, 12
    psrld           m1, 12
    mova         [t4+r10*1+400*0+ 4], m2
    mova         [t3+r10*2+400*0+ 8], xm0
    vextracti128 [t3+r10*2+400*0+40], m0, 1
    mova         [t3+r10*2+400*0+24], xm1
    vextracti128 [t3+r10*2+400*0+56], m1, 1
    add            r10, 32
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
.v1_loop:
    mova            m0, [t1+r10+400*0]
    mova            m4, [t1+r10+400*2]
    mova            m5, [t1+r10+400*4]
    paddw           m1, m0, [t2+r10+400*0]
    paddd           m2, m4, [t2+r10+400*2]
    paddd           m3, m5, [t2+r10+400*4]
    mova [t2+r10+400*0], m0
    mova [t2+r10+400*2], m4
    mova [t2+r10+400*4], m5
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a + 8) >> 4
    psrld           m3, 4
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; ((a + 8) >> 4) * 9
    paddd           m5, m3
    psrlw           m3, m1, 1
    pavgw           m3, m6             ; (b + 2) >> 2
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m2
    punpckhwd       m3, m6
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmaxud          m4, m2
    psubd           m4, m2             ; p
    pmaxud          m5, m3
    psubd           m5, m3
    pmulld          m4, m9             ; p * s
    pmulld          m5, m9
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrad           m3, m4, 20         ; min(z, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    psrld           m0, 12
    psrld           m1, 12
    mova         [t4+r10*1+400*2+ 4], m2
    mova         [t3+r10*2+400*4+ 8], xm0
    vextracti128 [t3+r10*2+400*4+40], m0, 1
    mova         [t3+r10*2+400*4+24], xm1
    vextracti128 [t3+r10*2+400*4+56], m1, 1
    add            r10, 32
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    mova           xm0, [t4+r10*1+400*0+0]
    paddw          xm0, [t4+r10*1+400*0+4]
    paddw          xm2, xm0, [t4+r10*1+400*0+2]
    mova            m1, [t3+r10*2+400*0+0]
    paddd           m1, [t3+r10*2+400*0+8]
    paddd           m3, m1, [t3+r10*2+400*0+4]
    psllw          xm2, 2                ; a[-1] 444
    pslld           m3, 2                ; b[-1] 444
    psubw          xm2, xm0              ; a[-1] 343
    psubd           m3, m1               ; b[-1] 343
    mova [t4+r10*1+400* 4], xm2
    mova [t3+r10*2+400* 8], m3
    mova           xm0, [t4+r10*1+400*2+0]
    paddw          xm0, [t4+r10*1+400*2+4]
    paddw          xm2, xm0, [t4+r10*1+400*2+2]
    mova            m1, [t3+r10*2+400*4+0]
    paddd           m1, [t3+r10*2+400*4+8]
    paddd           m3, m1, [t3+r10*2+400*4+4]
    psllw          xm2, 2                 ; a[ 0] 444
    pslld           m3, 2                 ; b[ 0] 444
    mova [t4+r10*1+400* 6], xm2
    mova [t3+r10*2+400*12], m3
    psubw          xm2, xm0               ; a[ 0] 343
    psubd           m3, m1                ; b[ 0] 343
    mova [t4+r10*1+400* 8], xm2
    mova [t3+r10*2+400*16], m3
    add            r10, 16
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    mova            m3, [t4+r10*1+400*0+0]
    paddw           m3, [t4+r10*1+400*0+4]
    paddw           m1, m3, [t4+r10*1+400*0+2]
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+r10*1+400*4]
    paddw           m3, [t4+r10*1+400*6]
    mova [t4+r10*1+400*4], m2
    mova [t4+r10*1+400*6], m1
    mova            m4, [t3+r10*2+400*0+0]
    paddd           m4, [t3+r10*2+400*0+8]
    paddd           m1, m4, [t3+r10*2+400*0+4]
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m4           ; b[ 1] 343
    paddd           m4, m2, [t3+r10*2+400* 8+ 0]
    paddd           m4, [t3+r10*2+400*12+ 0]
    mova [t3+r10*2+400* 8+ 0], m2
    mova [t3+r10*2+400*12+ 0], m1
    mova            m5, [t3+r10*2+400*0+32]
    paddd           m5, [t3+r10*2+400*0+40]
    paddd           m1, m5, [t3+r10*2+400*0+36]
    pslld           m1, 2
    psubd           m2, m1, m5
    paddd           m5, m2, [t3+r10*2+400* 8+32]
    paddd           m5, [t3+r10*2+400*12+32]
    mova [t3+r10*2+400* 8+32], m2
    mova [t3+r10*2+400*12+32], m1
    mova            m0, [dstq+r10]
    punpcklwd       m1, m0, m6
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m1               ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vinserti128     m1, m4, xm5, 1
    vperm2i128      m4, m5, 0x31
    psubd           m1, m2               ; b - a * src + (1 << 8)
    psubd           m4, m3
    psrad           m1, 9
    psrad           m4, 9
    packssdw        m1, m4
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m13
    mova    [dstq+r10], m0
    add            r10, 32
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova            m3, [t4+r10*1+400*2+0]
    paddw           m3, [t4+r10*1+400*2+4]
    paddw           m1, m3, [t4+r10*1+400*2+2]
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+r10*1+400*6]
    paddw           m3, [t4+r10*1+400*8]
    mova [t4+r10*1+400*6], m1
    mova [t4+r10*1+400*8], m2
    mova            m4, [t3+r10*2+400*4+0]
    paddd           m4, [t3+r10*2+400*4+8]
    paddd           m1, m4, [t3+r10*2+400*4+4]
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m4           ; b[ 1] 343
    paddd           m4, m2, [t3+r10*2+400*12+ 0]
    paddd           m4, [t3+r10*2+400*16+ 0]
    mova [t3+r10*2+400*12+ 0], m1
    mova [t3+r10*2+400*16+ 0], m2
    mova            m5, [t3+r10*2+400*4+32]
    paddd           m5, [t3+r10*2+400*4+40]
    paddd           m1, m5, [t3+r10*2+400*4+36]
    pslld           m1, 2
    psubd           m2, m1, m5
    paddd           m5, m2, [t3+r10*2+400*12+32]
    paddd           m5, [t3+r10*2+400*16+32]
    mova [t3+r10*2+400*12+32], m1
    mova [t3+r10*2+400*16+32], m2
    mova            m0, [dstq+r10]
    punpcklwd       m1, m0, m6
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m1               ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vinserti128     m1, m4, xm5, 1
    vperm2i128      m4, m5, 0x31
    psubd           m1, m2               ; b - a * src + (1 << 8)
    psubd           m4, m3
    psrad           m1, 9
    psrad           m4, 9
    packssdw        m1, m4
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m13
    mova    [dstq+r10], m0
    add            r10, 32
    jl .n1_loop
    add           dstq, strideq
    ret

cglobal sgr_filter_mix_16bpc, 4, 14, 16, 400*66+8, dst, stride, left, lpf, \
                                                   w, h, edge, params
    movifnidn       wd, wm
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x_avx2+256*4]
    add             wd, wd
    movifnidn       hd, hm
    mov          edged, r7m
    add           lpfq, wq
    vpbroadcastd   m15, [paramsq+8] ; w0 w1
    add           dstq, wq
    vpbroadcastd   m13, [paramsq+0] ; s0
    lea             t3, [rsp+wq*2+400*24+8]
    vpbroadcastd   m14, [paramsq+4] ; s1
    lea             t4, [rsp+wq+400*52+8]
    vpbroadcastd    m9, [pd_8]
    lea             t1, [rsp+wq+12]
    vpbroadcastd   m10, [pd_34816]
    neg             wq
    vpbroadcastd   m11, [pd_4096]
    pxor            m7, m7
    vpbroadcastd   m12, [pd_0xf00801c7]
    psllw          m15, 2
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_avx2).top_fixup
    add             t1, 400*12
    call .h_top
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    add            r10, strideq
    mov          [rsp], r10 ; below
    call .hv0
.main:
    dec             hd
    jz .height1
    add           lpfq, strideq
    call .hv1
    call .prep_n
    sub             hd, 2
    jl .extend_bottom
.main_loop:
    add           lpfq, strideq
    call .hv0
    test            hd, hd
    jz .odd_height
    add           lpfq, strideq
    call .hv1
    call .n0
    call .n1
    sub             hd, 2
    jge .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .extend_bottom
    mov           lpfq, [rsp]
    call .hv0_bottom
    add           lpfq, strideq
    call .hv1_bottom
.end:
    call .n0
    call .n1
.end2:
    RET
.height1:
    call .v1
    call .prep_n
    jmp .odd_height_end
.odd_height:
    call .v1
    call .n0
    call .n1
.odd_height_end:
    call .v0
    call .v1
    call .n0
    jmp .end2
.extend_bottom:
    call .v0
    call .v1
    jmp .end
.no_top:
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov          [rsp], r10
    call .h
    lea            r10, [wq-4]
    lea             t2, [t1+400*12]
.top_fixup_loop:
    mova            m0, [t1+r10+400* 0]
    mova            m1, [t1+r10+400* 2]
    mova            m2, [t1+r10+400* 4]
    paddw           m0, m0
    mova            m3, [t1+r10+400* 6]
    paddd           m1, m1
    mova            m4, [t1+r10+400* 8]
    paddd           m2, m2
    mova            m5, [t1+r10+400*10]
    mova [t2+r10+400* 0], m0
    mova [t2+r10+400* 2], m1
    mova [t2+r10+400* 4], m2
    mova [t2+r10+400* 6], m3
    mova [t2+r10+400* 8], m4
    mova [t2+r10+400*10], m5
    add            r10, 32
    jl .top_fixup_loop
    call .v0
    jmp .main
.h: ; horizontal boxsum
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 10
    jmp .h_main
.h_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, [sgr_lshuf5]
    vinserti128     m4, [lpfq+wq+10], 1
    jmp .h_main
.h_top:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m4, [lpfq+r10- 2]
.h_main:
    movu            m5, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -36
    jl .h_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_avx2).extend_right
.h_have_right:
    palignr         m3, m5, m4, 2
    palignr         m0, m5, m4, 4
    paddw           m1, m3, m0
    punpcklwd       m2, m3, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m0
    pmaddwd         m3, m3
    palignr         m0, m5, m4, 6
    paddw           m1, m0             ; sum3
    punpcklwd       m6, m0, m7
    pmaddwd         m6, m6
    punpckhwd       m0, m7
    pmaddwd         m0, m0
    paddd           m2, m6             ; sumsq3
    shufpd          m6, m4, m5, 0x05
    punpcklwd       m5, m6, m4
    paddw           m8, m4, m6
    pmaddwd         m5, m5
    punpckhwd       m6, m4
    pmaddwd         m6, m6
    paddd           m3, m0
    mova [t1+r10+400* 6], m1
    mova [t1+r10+400* 8], m2
    mova [t1+r10+400*10], m3
    paddw           m8, m1             ; sum5
    paddd           m5, m2             ; sumsq5
    paddd           m6, m3
    mova [t1+r10+400* 0], m8
    mova [t1+r10+400* 2], m5
    mova [t1+r10+400* 4], m6
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab3 (even rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 10
    jmp .hv0_main
.hv0_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, [sgr_lshuf5]
    vinserti128     m4, [lpfq+wq+10], 1
    jmp .hv0_main
.hv0_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
.hv0_loop:
    movu            m4, [lpfq+r10- 2]
.hv0_main:
    movu            m5, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp           r10d, -36
    jl .hv0_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_avx2).extend_right
.hv0_have_right:
    palignr         m3, m5, m4, 2
    palignr         m0, m5, m4, 4
    paddw           m1, m3, m0
    punpcklwd       m2, m3, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m0
    pmaddwd         m3, m3
    palignr         m0, m5, m4, 6
    paddw           m1, m0             ; h sum3
    punpcklwd       m6, m0, m7
    pmaddwd         m6, m6
    punpckhwd       m0, m7
    pmaddwd         m0, m0
    paddd           m2, m6             ; h sumsq3
    shufpd          m6, m4, m5, 0x05
    punpcklwd       m5, m6, m4
    paddw           m8, m4, m6
    pmaddwd         m5, m5
    punpckhwd       m6, m4
    pmaddwd         m6, m6
    paddd           m3, m0
    paddw           m8, m1             ; h sum5
    paddd           m5, m2             ; h sumsq5
    paddd           m6, m3
    mova [t3+r10*2+400*8+ 8], m8 ; we need a clean copy of the last row TODO: t4?
    mova [t3+r10*2+400*0+ 8], m5 ; in case height is odd
    mova [t3+r10*2+400*0+40], m6
    paddw           m8, [t1+r10+400* 0]
    paddd           m5, [t1+r10+400* 2]
    paddd           m6, [t1+r10+400* 4]
    mova [t1+r10+400* 0], m8
    mova [t1+r10+400* 2], m5
    mova [t1+r10+400* 4], m6
    paddw           m0, m1, [t1+r10+400* 6]
    paddd           m4, m2, [t1+r10+400* 8]
    paddd           m5, m3, [t1+r10+400*10]
    mova [t1+r10+400* 6], m1
    mova [t1+r10+400* 8], m2
    mova [t1+r10+400*10], m3
    paddw           m1, m0, [t2+r10+400* 6]
    paddd           m2, m4, [t2+r10+400* 8]
    paddd           m3, m5, [t2+r10+400*10]
    mova [t2+r10+400* 6], m0
    mova [t2+r10+400* 8], m4
    mova [t2+r10+400*10], m5
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; ((a3 + 8) >> 4) * 9
    paddd           m5, m3
    psrlw           m3, m1, 1
    pavgw           m3, m7             ; (b3 + 2) >> 2
    punpcklwd       m2, m3, m7
    pmaddwd         m2, m2
    punpckhwd       m3, m7
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m7         ; b3
    punpckhwd       m1, m7
    pmaxud          m4, m2
    psubd           m4, m2             ; p3
    pmaxud          m5, m3
    psubd           m5, m3
    pmulld          m4, m14            ; p3 * s1
    pmulld          m5, m14
    pmaddwd         m0, m12            ; b3 * 455
    pmaddwd         m1, m12
    paddusw         m4, m12
    paddusw         m5, m12
    psrad           m3, m4, 20         ; min(z3, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x3
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    psrld           m0, 12
    psrld           m1, 12
    mova         [t4+r10*1+400*2+ 4], m2
    mova         [t3+r10*2+400*4+ 8], xm0
    vextracti128 [t3+r10*2+400*4+40], m0, 1
    mova         [t3+r10*2+400*4+24], xm1
    vextracti128 [t3+r10*2+400*4+56], m1, 1
    add            r10, 32
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    vpbroadcastq   xm5, [leftq]
    vinserti128     m5, [lpfq+wq], 1
    mova            m4, [lpfq+wq]
    add          leftq, 8
    palignr         m4, m5, 10
    jmp .hv1_main
.hv1_extend_left:
    mova           xm4, [lpfq+wq]
    pshufb         xm4, [sgr_lshuf5]
    vinserti128     m4, [lpfq+wq+10], 1
    jmp .hv1_main
.hv1_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
.hv1_loop:
    movu            m4, [lpfq+r10- 2]
.hv1_main:
    movu            m5, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp           r10d, -36
    jl .hv1_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_avx2).extend_right
.hv1_have_right:
    palignr         m6, m5, m4, 2
    palignr         m3, m5, m4, 4
    paddw           m2, m6, m3
    punpcklwd       m0, m6, m3
    pmaddwd         m0, m0
    punpckhwd       m6, m3
    pmaddwd         m6, m6
    palignr         m3, m5, m4, 6
    paddw           m2, m3             ; h sum3
    punpcklwd       m1, m3, m7
    pmaddwd         m1, m1
    punpckhwd       m3, m7
    pmaddwd         m3, m3
    paddd           m0, m1             ; h sumsq3
    shufpd          m1, m4, m5, 0x05
    punpckhwd       m5, m4, m1
    paddw           m8, m4, m1
    pmaddwd         m5, m5
    punpcklwd       m4, m1
    pmaddwd         m4, m4
    paddd           m6, m3
    paddw           m1, m2, [t2+r10+400* 6]
    mova [t2+r10+400* 6], m2
    paddw           m8, m2             ; h sum5
    paddd           m2, m0, [t2+r10+400* 8]
    paddd           m3, m6, [t2+r10+400*10]
    mova [t2+r10+400* 8], m0
    mova [t2+r10+400*10], m6
    paddd           m4, m0             ; h sumsq5
    paddd           m5, m6
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
    pslld           m0, m2, 3
    pslld           m6, m3, 3
    paddd           m2, m0             ; ((a3 + 8) >> 4) * 9
    paddd           m3, m6
    psrlw           m6, m1, 1
    pavgw           m6, m7             ; (b3 + 2) >> 2
    punpcklwd       m0, m6, m7
    pmaddwd         m0, m0
    punpckhwd       m6, m7
    pmaddwd         m6, m6
    pmaxud          m2, m0
    psubd           m2, m0             ; p3
    pmaxud          m3, m6
    psubd           m3, m6
    punpcklwd       m0, m1, m7         ; b3
    punpckhwd       m1, m7
    pmulld          m2, m14            ; p3 * s1
    pmulld          m3, m14
    pmaddwd         m0, m12            ; b3 * 455
    pmaddwd         m1, m12
    paddusw         m2, m12
    paddusw         m3, m12
    psrad           m7, m2, 20         ; min(z3, 255) - 256
    vpgatherdd      m6, [r13+m7*4], m2 ; x3
    psrad           m2, m3, 20
    vpgatherdd      m7, [r13+m2*4], m3
    pmulld          m0, m6
    packssdw        m6, m7
    pmulld          m7, m1
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m7, m10
    psrld           m0, 12
    psrld           m7, 12
    paddw           m1, m8, [t2+r10+400*0]
    paddd           m2, m4, [t2+r10+400*2]
    paddd           m3, m5, [t2+r10+400*4]
    paddw           m1, [t1+r10+400*0]
    paddd           m2, [t1+r10+400*2]
    paddd           m3, [t1+r10+400*4]
    mova [t2+r10+400*0], m8
    mova [t2+r10+400*2], m4
    mova [t2+r10+400*4], m5
    mova         [t4+r10*1+400*4 +4], m6
    mova         [t3+r10*2+400*8+ 8], xm0
    vextracti128 [t3+r10*2+400*8+40], m0, 1
    mova         [t3+r10*2+400*8+24], xm7
    vextracti128 [t3+r10*2+400*8+56], m7, 1
    vpbroadcastd    m4, [pd_25]
    pxor            m7, m7
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a5 + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m4             ; ((a5 + 8) >> 4) * 25
    pmulld          m3, m4
    psrlw           m5, m1, 1
    pavgw           m5, m7             ; (b5 + 2) >> 2
    punpcklwd       m4, m5, m7
    pmaddwd         m4, m4
    punpckhwd       m5, m7
    pmaddwd         m5, m5
    punpcklwd       m0, m1, m7         ; b5
    punpckhwd       m1, m7
    pmaxud          m2, m4
    psubd           m2, m4             ; p5
    vpbroadcastd    m4, [pd_0xf00800a4]
    pmaxud          m3, m5
    psubd           m3, m5
    pmulld          m2, m13            ; p5 * s0
    pmulld          m3, m13
    pmaddwd         m0, m4             ; b5 * 164
    pmaddwd         m1, m4
    paddusw         m2, m4
    paddusw         m3, m4
    psrad           m5, m2, 20         ; min(z5, 255) - 256
    vpgatherdd      m4, [r13+m5*4], m2 ; x5
    psrad           m2, m3, 20
    vpgatherdd      m5, [r13+m2*4], m3
    pmulld          m0, m4
    pmulld          m1, m5
    packssdw        m4, m5
    paddd           m0, m10            ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    psrld           m0, 12
    psrld           m1, 12
    mova         [t4+r10*1+400*0+ 4], m4
    mova         [t3+r10*2+400*0+ 8], xm0
    vextracti128 [t3+r10*2+400*0+40], m0, 1
    mova         [t3+r10*2+400*0+24], xm1
    vextracti128 [t3+r10*2+400*0+56], m1, 1
    add            r10, 32
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab3 (even rows)
    lea            r10, [wq-4]
.v0_loop:
    mova            m0, [t1+r10+400* 6]
    mova            m4, [t1+r10+400* 8]
    mova            m5, [t1+r10+400*10]
    paddw           m0, m0
    paddd           m4, m4
    paddd           m5, m5
    paddw           m1, m0, [t2+r10+400* 6]
    paddd           m2, m4, [t2+r10+400* 8]
    paddd           m3, m5, [t2+r10+400*10]
    mova [t2+r10+400* 6], m0
    mova [t2+r10+400* 8], m4
    mova [t2+r10+400*10], m5
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; ((a3 + 8) >> 4) * 9
    paddd           m5, m3
    psrlw           m3, m1, 1
    pavgw           m3, m7             ; (b3 + 2) >> 2
    punpcklwd       m2, m3, m7
    pmaddwd         m2, m2
    punpckhwd       m3, m7
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m7         ; b3
    punpckhwd       m1, m7
    pmaxud          m4, m2
    psubd           m4, m2             ; p3
    pmaxud          m5, m3
    psubd           m5, m3
    pmulld          m4, m14            ; p3 * s1
    pmulld          m5, m14
    pmaddwd         m0, m12            ; b3 * 455
    pmaddwd         m1, m12
    paddusw         m4, m12
    paddusw         m5, m12
    psrad           m3, m4, 20         ; min(z3, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x3
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    psrld           m0, 12
    psrld           m1, 12
    mova            m3, [t1+r10+400*0]
    mova            m4, [t1+r10+400*2]
    mova            m5, [t1+r10+400*4]
    mova [t3+r10*2+400*8+ 8], m3
    mova [t3+r10*2+400*0+ 8], m4
    mova [t3+r10*2+400*0+40], m5
    paddw           m3, m3 ; cc5
    paddd           m4, m4
    paddd           m5, m5
    mova [t1+r10+400*0], m3
    mova [t1+r10+400*2], m4
    mova [t1+r10+400*4], m5
    mova         [t4+r10*1+400*2+ 4], m2
    mova         [t3+r10*2+400*4+ 8], xm0
    vextracti128 [t3+r10*2+400*4+40], m0, 1
    mova         [t3+r10*2+400*4+24], xm1
    vextracti128 [t3+r10*2+400*4+56], m1, 1
    add            r10, 32
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
.v1_loop:
    mova            m4, [t1+r10+400* 6]
    mova            m5, [t1+r10+400* 8]
    mova            m6, [t1+r10+400*10]
    paddw           m1, m4, [t2+r10+400* 6]
    paddd           m2, m5, [t2+r10+400* 8]
    paddd           m3, m6, [t2+r10+400*10]
    mova [t2+r10+400* 6], m4
    mova [t2+r10+400* 8], m5
    mova [t2+r10+400*10], m6
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; ((a3 + 8) >> 4) * 9
    paddd           m5, m3
    psrlw           m3, m1, 1
    pavgw           m3, m7             ; (b3 + 2) >> 2
    punpcklwd       m2, m3, m7
    pmaddwd         m2, m2
    punpckhwd       m3, m7
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m7         ; b3
    punpckhwd       m1, m7
    pmaxud          m4, m2
    psubd           m4, m2             ; p3
    pmaxud          m5, m3
    psubd           m5, m3
    pmulld          m4, m14            ; p3 * s1
    pmulld          m5, m14
    pmaddwd         m0, m12            ; b3 * 455
    pmaddwd         m1, m12
    paddusw         m4, m12
    paddusw         m5, m12
    psrad           m3, m4, 20         ; min(z3, 255) - 256
    vpgatherdd      m2, [r13+m3*4], m4 ; x3
    psrad           m4, m5, 20
    vpgatherdd      m3, [r13+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    packssdw        m2, m3
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    psrld           m0, 12
    psrld           m8, m1, 12
    mova [t4+r10*1+400*4+4], m2
    mova            m4, [t3+r10*2+400*8+ 8]
    mova            m5, [t3+r10*2+400*0+ 8]
    mova            m6, [t3+r10*2+400*0+40]
    paddw           m1, m4, [t2+r10+400*0]
    paddd           m2, m5, [t2+r10+400*2]
    paddd           m3, m6, [t2+r10+400*4]
    paddw           m1, [t1+r10+400*0]
    paddd           m2, [t1+r10+400*2]
    paddd           m3, [t1+r10+400*4]
    mova [t2+r10+400*0], m4
    mova [t2+r10+400*2], m5
    mova [t2+r10+400*4], m6
    vpbroadcastd    m4, [pd_25]
    mova         [t3+r10*2+400*8+ 8], xm0
    vextracti128 [t3+r10*2+400*8+40], m0, 1
    mova         [t3+r10*2+400*8+24], xm8
    vextracti128 [t3+r10*2+400*8+56], m8, 1
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a5 + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m4             ; ((a5 + 8) >> 4) * 25
    pmulld          m3, m4
    psrlw           m5, m1, 1
    pavgw           m5, m7             ; (b5 + 2) >> 2
    punpcklwd       m4, m5, m7
    pmaddwd         m4, m4
    punpckhwd       m5, m7
    pmaddwd         m5, m5
    punpcklwd       m0, m1, m7         ; b5
    punpckhwd       m1, m7
    pmaxud          m2, m4
    psubd           m2, m4             ; p5
    vpbroadcastd    m4, [pd_0xf00800a4]
    pmaxud          m3, m5
    psubd           m3, m5
    pmulld          m2, m13            ; p5 * s0
    pmulld          m3, m13
    pmaddwd         m0, m4             ; b5 * 164
    pmaddwd         m1, m4
    paddusw         m2, m4
    paddusw         m3, m4
    psrad           m5, m2, 20         ; min(z5, 255) - 256
    vpgatherdd      m4, [r13+m5*4], m2 ; x5
    psrad           m2, m3, 20
    vpgatherdd      m5, [r13+m2*4], m3
    pmulld          m0, m4
    pmulld          m1, m5
    packssdw        m4, m5
    paddd           m0, m10            ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    psrld           m0, 12
    psrld           m1, 12
    mova         [t4+r10*1+400*0+ 4], m4
    mova         [t3+r10*2+400*0+ 8], xm0
    vextracti128 [t3+r10*2+400*0+40], m0, 1
    mova         [t3+r10*2+400*0+24], xm1
    vextracti128 [t3+r10*2+400*0+56], m1, 1
    add            r10, 32
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu           xm0, [t4+r10*1+400*0+2]
    paddw          xm2, xm0, [t4+r10*1+400*0+0]
    paddw          xm2, [t4+r10*1+400*0+4]
    movu            m1, [t3+r10*2+400*0+4]
    paddd           m3, m1, [t3+r10*2+400*0+0]
    paddd           m3, [t3+r10*2+400*0+8]
    paddw          xm0, xm2
    paddd           m1, m3
    psllw          xm2, 2
    pslld           m3, 2
    paddw          xm0, xm2              ; a5 565
    paddd           m1, m3               ; b5 565
    mova [t4+r10*1+400* 6], xm0
    mova [t3+r10*2+400*12], m1
    mova           xm0, [t4+r10*1+400*2+0]
    paddw          xm0, [t4+r10*1+400*2+4]
    paddw          xm2, xm0, [t4+r10*1+400*2+2]
    mova            m1, [t3+r10*2+400*4+0]
    paddd           m1, [t3+r10*2+400*4+8]
    paddd           m3, m1, [t3+r10*2+400*4+4]
    psllw          xm2, 2                ; a3[-1] 444
    pslld           m3, 2                ; b3[-1] 444
    psubw          xm2, xm0              ; a3[-1] 343
    psubd           m3, m1               ; b3[-1] 343
    mova [t4+r10*1+400* 8], xm2
    mova [t3+r10*2+400*16], m3
    mova           xm0, [t4+r10*1+400*4+0]
    paddw          xm0, [t4+r10*1+400*4+4]
    paddw          xm2, xm0, [t4+r10*1+400*4+2]
    mova            m1, [t3+r10*2+400*8+0]
    paddd           m1, [t3+r10*2+400*8+8]
    paddd           m3, m1, [t3+r10*2+400*8+4]
    psllw          xm2, 2                 ; a3[ 0] 444
    pslld           m3, 2                 ; b3[ 0] 444
    mova [t4+r10*1+400*10], xm2
    mova [t3+r10*2+400*20], m3
    psubw          xm2, xm0               ; a3[ 0] 343
    psubd           m3, m1                ; b3[ 0] 343
    mova [t4+r10*1+400*12], xm2
    mova [t3+r10*2+400*24], m3
    add            r10, 16
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu           xm2, [t4+r10*1+2]
    paddw          xm0, xm2, [t4+r10*1+0]
    paddw          xm0, [t4+r10*1+4]
    paddw          xm2, xm0
    psllw          xm0, 2
    paddw          xm0, xm2              ; a5
    movu            m1, [t3+r10*2+4]
    paddd           m4, m1, [t3+r10*2+0]
    paddd           m4, [t3+r10*2+8]
    paddd           m1, m4
    pslld           m4, 2
    paddd           m4, m1               ; b5
    paddw          xm2, xm0, [t4+r10*1+400* 6]
    mova [t4+r10*1+400* 6], xm0
    paddd           m0, m4, [t3+r10*2+400*12]
    mova [t3+r10*2+400*12], m4
    mova           xm3, [t4+r10*1+400*2+0]
    paddw          xm3, [t4+r10*1+400*2+4]
    paddw          xm5, xm3, [t4+r10*1+400*2+2]
    psllw          xm5, 2                ; a3[ 1] 444
    psubw          xm4, xm5, xm3         ; a3[ 1] 343
    paddw          xm3, xm4, [t4+r10*1+400* 8]
    paddw          xm3, [t4+r10*1+400*10]
    mova [t4+r10*1+400* 8], xm4
    mova [t4+r10*1+400*10], xm5
    mova            m1, [t3+r10*2+400*4+0]
    paddd           m1, [t3+r10*2+400*4+8]
    paddd           m5, m1, [t3+r10*2+400*4+4]
    pslld           m5, 2                ; b3[ 1] 444
    psubd           m4, m5, m1           ; b3[ 1] 343
    paddd           m1, m4, [t3+r10*2+400*16]
    paddd           m1, [t3+r10*2+400*20]
    mova [t3+r10*2+400*16], m4
    mova [t3+r10*2+400*20], m5
    pmovzxwd        m4, [dstq+r10]
    pmovzxwd        m2, xm2              ; a5
    pmovzxwd        m3, xm3              ; a3
    pmaddwd         m2, m4               ; a5 * src
    pmaddwd         m3, m4               ; a3 * src
    pslld           m4, 13
    psubd           m0, m2               ; b5 - a5 * src + (1 << 8)
    psubd           m1, m3               ; b3 - a3 * src + (1 << 8)
    psrld           m0, 9
    pslld           m1, 7
    pblendw         m0, m1, 0xaa
    pmaddwd         m0, m15
    paddd           m4, m11
    paddd           m0, m4
    psrad           m0, 7
    vextracti128   xm1, m0, 1
    packusdw       xm0, xm1              ; clip
    psrlw          xm0, 6
    mova    [dstq+r10], xm0
    add            r10, 16
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova           xm3, [t4+r10*1+400*4+0]
    paddw          xm3, [t4+r10*1+400*4+4]
    paddw          xm5, xm3, [t4+r10*1+400*4+2]
    psllw          xm5, 2                ; a3[ 1] 444
    psubw          xm4, xm5, xm3         ; a3[ 1] 343
    paddw          xm3, xm4, [t4+r10*1+400*12]
    paddw          xm3, [t4+r10*1+400*10]
    mova [t4+r10*1+400*10], xm5
    mova [t4+r10*1+400*12], xm4
    mova            m1, [t3+r10*2+400*8+0]
    paddd           m1, [t3+r10*2+400*8+8]
    paddd           m5, m1, [t3+r10*2+400*8+4]
    pslld           m5, 2                ; b3[ 1] 444
    psubd           m4, m5, m1           ; b3[ 1] 343
    paddd           m1, m4, [t3+r10*2+400*24]
    paddd           m1, [t3+r10*2+400*20]
    mova [t3+r10*2+400*20], m5
    mova [t3+r10*2+400*24], m4
    pmovzxwd        m4, [dstq+r10]
    pmovzxwd        m2, [t4+r10*1+400* 6]
    pmovzxwd        m3, xm3
    mova            m0, [t3+r10*2+400*12]
    pmaddwd         m2, m4               ; a5 * src
    pmaddwd         m3, m4               ; a3 * src
    pslld           m4, 13
    psubd           m0, m2               ; b5 - a5 * src + (1 << 8)
    psubd           m1, m3               ; b3 - a3 * src + (1 << 8)
    psrld           m0, 8
    pslld           m1, 7
    pblendw         m0, m1, 0xaa
    pmaddwd         m0, m15
    paddd           m4, m11
    paddd           m0, m4
    psrad           m0, 7
    vextracti128   xm1, m0, 1
    packusdw       xm0, xm1              ; clip
    psrlw          xm0, 6
    mova    [dstq+r10], xm0
    add            r10, 16
    jl .n1_loop
    add           dstq, strideq
    ret

%endif ; ARCH_X86_64
