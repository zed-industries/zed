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

SECTION_RODATA 16

wiener_shufA:  db  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11
wiener_shufB:  db  6,  7,  4,  5,  8,  9,  6,  7, 10, 11,  8,  9, 12, 13, 10, 11
wiener_shufC:  db  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13, 12, 13, 14, 15
wiener_shufD:  db  2,  3, -1, -1,  4,  5, -1, -1,  6,  7, -1, -1,  8,  9, -1, -1
wiener_shufE:  db  0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
r_ext_mask:    times 72 db -1
               times  8 db  0
wiener_hshift: dw 4, 4, 1, 1
wiener_vshift: dw 1024, 1024, 4096, 4096
wiener_round:  dd 1049600, 1048832

pw_164_455:    dw 164, 455
pw_1023:       times 2 dw 1023
pw_61448:      times 2 dw 61448
pd_m262128:    dd -262128
pd_m34816:     dd -34816
pd_m25:        dd -25
pd_m9:         dd -9
pd_8:          dd 8
pd_2147483648: dd 2147483648

cextern sgr_x_by_x

SECTION .text

DECLARE_REG_TMP 8, 7, 9, 11, 12, 13, 14 ; ring buffer pointers

INIT_ZMM avx512icl
cglobal wiener_filter7_16bpc, 4, 15, 17, -384*12-16, dst, stride, left, lpf, \
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
    mov           r10d, 0xfe
    vpbroadcastd   m10, [base+wiener_round+t3*4]
    kmovb           k1, r10d
    vpbroadcastd   m11, [base+wiener_vshift+t3*4]
    pmullw         m12, m0 ; upshift filter coefs to make the
    vpbroadcastd   m16, [pd_m262128]
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
.h:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movq           xm3, [leftq]
    vmovdqu64   m3{k1}, [lpfq+r10-8]
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    mova            m4, [lpfq+r10+0]
    vpbroadcastw   xm3, xm4
    vmovdqu64   m3{k1}, [lpfq+r10-8]
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
    cmp           r10d, -68
    jl .h_have_right
    push            r0
    lea             r0, [r_ext_mask+66]
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd      m3, m0, [r0+r10+ 0], 0xe4 ; c ? a : b
    vpternlogd      m4, m0, [r0+r10+ 8], 0xe4
    vpternlogd      m5, m0, [r0+r10+16], 0xe4
    pop             r0
.h_have_right:
    pshufb          m2, m3, m6
    pshufb          m1, m4, m7
    paddw           m2, m1
    pshufb          m3, m8
    mova            m0, m16
    vpdpwssd        m0, m2, m12
    pshufb          m1, m4, m9
    paddw           m3, m1
    pshufb          m1, m4, m6
    vpdpwssd        m0, m3, m13
    pshufb          m2, m5, m7
    paddw           m2, m1
    mova            m1, m16
    pshufb          m4, m8
    vpdpwssd        m1, m2, m12
    pshufb          m5, m9
    paddw           m4, m5
    vpdpwssd        m1, m4, m13
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    psraw           m0, 1
    mova      [t1+r10], m0
    add            r10, 64
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movq           xm3, [leftq]
    vmovdqu64   m3{k1}, [lpfq+r10-8]
    add          leftq, 8
    jmp .hv_main
.hv_extend_left:
    mova            m4, [lpfq+r10+0]
    vpbroadcastw   xm3, xm4
    vmovdqu64   m3{k1}, [lpfq+r10-8]
    jmp .hv_main2
.hv_bottom:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m3, [lpfq+r10-8]
.hv_main:
    mova            m4, [lpfq+r10+0]
.hv_main2:
    movu            m5, [lpfq+r10+8]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -68
    jl .hv_have_right
    push            r0
    lea             r0, [r_ext_mask+66]
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd      m3, m0, [r0+r10+ 0], 0xe4
    vpternlogd      m4, m0, [r0+r10+ 8], 0xe4
    vpternlogd      m5, m0, [r0+r10+16], 0xe4
    pop             r0
.hv_have_right:
    pshufb          m2, m3, m6
    pshufb          m1, m4, m7
    paddw           m2, m1
    pshufb          m3, m8
    mova            m0, m16
    vpdpwssd        m0, m2, m12
    pshufb          m1, m4, m9
    paddw           m3, m1
    pshufb          m1, m4, m6
    vpdpwssd        m0, m3, m13
    pshufb          m2, m5, m7
    paddw           m2, m1
    pshufb          m4, m8
    mova            m1, m16
    vpdpwssd        m1, m2, m12
    pshufb          m5, m9
    paddw           m4, m5
    vpdpwssd        m1, m4, m13
    mova            m2, [t4+r10]
    paddw           m2, [t2+r10]
    mova            m5, [t3+r10]
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    mova            m4, [t5+r10]
    paddw           m4, [t1+r10]
    psraw           m0, 1
    paddw           m3, m0, [t6+r10]
    mova      [t0+r10], m0
    punpcklwd       m1, m2, m5
    mova            m0, m10
    vpdpwssd        m0, m1, m15
    punpckhwd       m2, m5
    mova            m1, m10
    vpdpwssd        m1, m2, m15
    punpcklwd       m2, m3, m4
    vpdpwssd        m0, m2, m14
    punpckhwd       m3, m4
    vpdpwssd        m1, m3, m14
    psrad           m0, 5
    psrad           m1, 5
    packusdw        m0, m1
    pmulhuw         m0, m11
    mova    [dstq+r10], m0
    add            r10, 64
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
    mova            m2, [t4+r10]
    paddw           m2, [t2+r10]
    mova            m3, [t3+r10]
    punpcklwd       m1, m2, m3
    mova            m0, m10
    vpdpwssd        m0, m1, m15
    punpckhwd       m2, m3
    mova            m1, m10
    vpdpwssd        m1, m2, m15
    mova            m4, [t1+r10]
    paddw           m3, m4, [t6+r10]
    paddw           m4, [t5+r10]
    punpcklwd       m2, m3, m4
    vpdpwssd        m0, m2, m14
    punpckhwd       m3, m4
    vpdpwssd        m1, m3, m14
    psrad           m0, 5
    psrad           m1, 5
    packusdw        m0, m1
    pmulhuw         m0, m11
    mova    [dstq+r10], m0
    add            r10, 64
    jl .v_loop
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    add           dstq, strideq
    ret

cglobal wiener_filter5_16bpc, 4, 14, 15, 384*8+16, dst, stride, left, lpf, \
                                                   w, h, edge, flt
%define base r13-r_ext_mask-70
    mov           fltq, r6mp
    movifnidn       wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    mov            t3d, r8m ; pixel_max
    vbroadcasti128  m5, [wiener_shufE]
    vpbroadcastw   m11, [fltq+ 2] ; x1
    vbroadcasti128  m6, [wiener_shufB]
    lea            r13, [r_ext_mask+70]
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
    vpbroadcastd    m0, [base+wiener_hshift+t3*4]
    neg             wq
    vpbroadcastd    m9, [base+wiener_round+t3*4]
    mov           r10d, 0xfffe
    vpbroadcastd   m10, [base+wiener_vshift+t3*4]
    kmovw           k1, r10d
    pmullw         m11, m0
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
.h:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movd           xm3, [leftq+4]
    vmovdqu32   m3{k1}, [lpfq+r10-4]
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    vpbroadcastw   xm3, [lpfq+r10]
    vmovdqu32   m3{k1}, [lpfq+r10-4]
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
    cmp           r10d, -66
    jl .h_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd      m3, m0, [r13+r10+0], 0xe4 ; c ? a : b
    vpternlogd      m4, m0, [r13+r10+8], 0xe4
.h_have_right:
    pshufb          m1, m3, m5
    mova            m0, m8
    vpdpwssd        m0, m1, m11
    pshufb          m2, m4, m5
    mova            m1, m8
    vpdpwssd        m1, m2, m11
    pshufb          m2, m3, m6
    pshufb          m3, m7
    paddw           m2, m3
    pshufb          m3, m4, m6
    vpdpwssd        m0, m2, m12
    pshufb          m4, m7
    paddw           m3, m4
    vpdpwssd        m1, m3, m12
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    psraw           m0, 1
    mova      [t1+r10], m0
    add            r10, 64
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movd           xm3, [leftq+4]
    vmovdqu32   m3{k1}, [lpfq+r10-4]
    add          leftq, 8
    jmp .hv_main
.hv_extend_left:
    vpbroadcastw   xm3, [lpfq+r10]
    vmovdqu32   m3{k1}, [lpfq+r10-4]
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
    cmp           r10d, -66
    jl .hv_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd      m3, m0, [r13+r10+0], 0xe4
    vpternlogd      m4, m0, [r13+r10+8], 0xe4
.hv_have_right:
    pshufb          m1, m3, m5
    mova            m0, m8
    vpdpwssd        m0, m1, m11
    pshufb          m2, m4, m5
    mova            m1, m8
    vpdpwssd        m1, m2, m11
    pshufb          m2, m3, m6
    pshufb          m3, m7
    paddw           m2, m3
    pshufb          m3, m4, m6
    vpdpwssd        m0, m2, m12
    pshufb          m4, m7
    paddw           m4, m3
    vpdpwssd        m1, m4, m12
    mova            m2, [t3+r10]
    paddw           m2, [t1+r10]
    mova            m3, [t2+r10]
    punpcklwd       m4, m2, m3
    punpckhwd       m2, m3
    mova            m3, m9
    vpdpwssd        m3, m2, m14
    mova            m2, m9
    vpdpwssd        m2, m4, m14
    mova            m4, [t4+r10]
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    psraw           m0, 1
    mova      [t0+r10], m0
    punpcklwd       m1, m0, m4
    vpdpwssd        m2, m1, m13
    punpckhwd       m0, m4
    vpdpwssd        m3, m0, m13
    psrad           m2, 5
    psrad           m3, 5
    packusdw        m2, m3
    pmulhuw         m2, m10
    mova    [dstq+r10], m2
    add            r10, 64
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
    add            r10, 64
    jl .v_loop
    ret

cglobal sgr_filter_5x5_16bpc, 4, 14, 22, 416*24+8, dst, stride, left, lpf, \
                                                   w, h, edge, params
%define base r13-r_ext_mask-72
    movifnidn       wd, wm
    mov        paramsq, r6mp
    lea            r13, [r_ext_mask+72]
    mov          edged, r7m
    movifnidn       hd, hm
    pxor            m6, m6
    vpbroadcastw    m7, [paramsq+8] ; w0
    add             wd, wd
    vpbroadcastd    m8, [base+pd_8]
    add           lpfq, wq
    vpbroadcastd    m9, [base+pd_m25]
    add           dstq, wq
    vpsubd         m10, m6, [paramsq+0] {1to16} ; -s0
    lea             t3, [rsp+wq*2+416*12+8]
    vpbroadcastd   m11, [base+pw_164_455]
    lea             t4, [rsp+wq+416*20+8]
    vpbroadcastd   m12, [base+pw_61448]  ; (15 << 12) + (1 << 3)
    lea             t1, [rsp+wq+12]
    vpbroadcastd   m13, [base+pd_m34816] ; -((1 << 11) + (1 << 15))
    neg             wq
    vpbroadcastd   m14, [base+pw_1023]
    psllw           m7, 4
    mova           m18, [sgr_x_by_x+64*0]
    mov           r10d, 0xfffffff8
    mova           m19, [sgr_x_by_x+64*1]
    kmovd           k1, r10d
    mova           m20, [sgr_x_by_x+64*2]
    mov            r10, 0x3333333333333333
    mova           m21, [sgr_x_by_x+64*3]
    kmovq           k2, r10
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    call .top_fixup
    add             t1, 416*6
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
    lea             t2, [t1+416*6]
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
.h: ; horizontal boxsum
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movq          xm16, [leftq+2]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    jmp .h_main
.h_top:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu           m16, [lpfq+r10- 2]
.h_main:
    movu           m17, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -68
    jl .h_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4 ; c ? a : b
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.h_have_right:
    palignr         m2, m17, m16, 2
    paddw           m0, m16, m2
    palignr         m3, m17, m16, 6
    paddw           m0, m3
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m1
    punpckhwd       m2, m3
    pmaddwd         m2, m2
    shufpd         m17, m16, m17, 0x55
    paddw           m0, m17
    punpcklwd       m3, m16, m17
    vpdpwssd        m1, m3, m3
    punpckhwd       m3, m16, m17
    vpdpwssd        m2, m3, m3
    shufps         m16, m17, q2121
    paddw           m0, m16            ; sum
    test         edgeb, 16             ; y > 0
    jz .h_loop_end
    paddw           m0, [t1+r10+416*0]
    paddd           m1, [t1+r10+416*2]
    paddd           m2, [t1+r10+416*4]
.h_loop_end:
    punpcklwd      m17, m16, m6
    vpdpwssd        m1, m17, m17       ; sumsq
    punpckhwd      m16, m6
    vpdpwssd        m2, m16, m16
    mova [t1+r10+416*0], m0
    mova [t1+r10+416*2], m1
    mova [t1+r10+416*4], m2
    add            r10, 64
    jl .h_loop
    ret
.top_fixup:
    lea            r10, [wq-4]
.top_fixup_loop: ; the sums of the first row needs to be doubled
    mova            m0, [t1+r10+416*0]
    mova            m1, [t1+r10+416*2]
    mova            m2, [t1+r10+416*4]
    paddw           m0, m0
    paddd           m1, m1
    paddd           m2, m2
    mova [t2+r10+416*0], m0
    mova [t2+r10+416*2], m1
    mova [t2+r10+416*4], m2
    add            r10, 64
    jl .top_fixup_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movq          xm16, [leftq+2]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    add          leftq, 8
    jmp .hv_main
.hv_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    jmp .hv_main
.hv_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu           m16, [lpfq+r10- 2]
.hv_main:
    movu           m17, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -68
    jl .hv_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.hv_have_right:
    palignr         m3, m17, m16, 2
    paddw           m0, m16, m3
    palignr         m1, m17, m16, 6
    paddw           m0, m1
    punpcklwd       m2, m3, m1
    pmaddwd         m2, m2
    punpckhwd       m3, m1
    pmaddwd         m3, m3
    shufpd         m17, m16, m17, 0x55
    paddw           m0, m17
    punpcklwd       m1, m16, m17
    vpdpwssd        m2, m1, m1
    punpckhwd       m1, m16, m17
    vpdpwssd        m3, m1, m1
    shufps         m16, m17, q2121
    paddw           m0, m16           ; h sum
    punpcklwd      m17, m16, m6
    vpdpwssd        m2, m17, m17      ; h sumsq
    punpckhwd      m16, m6
    vpdpwssd        m3, m16, m16
    paddw           m1, m0, [t1+r10+416*0]
    paddd          m16, m2, [t1+r10+416*2]
    paddd          m17, m3, [t1+r10+416*4]
    test            hd, hd
    jz .hv_last_row
.hv_main2:
    paddw           m1, [t2+r10+416*0] ; hv sum
    paddd          m16, [t2+r10+416*2] ; hv sumsq
    paddd          m17, [t2+r10+416*4]
    mova [t0+r10+416*0], m0
    mova [t0+r10+416*2], m2
    mova [t0+r10+416*4], m3
    psrlw           m3, m1, 1
    paddd          m16, m8
    pavgw           m3, m6             ; (b + 2) >> 2
    paddd          m17, m8
    psrld          m16, 4              ; (a + 8) >> 4
    psrld          m17, 4
    pmulld         m16, m9             ; -a * 25
    pmulld         m17, m9
    punpcklwd       m2, m3, m6
    vpdpwssd       m16, m2, m2         ; -p
    punpckhwd       m3, m6
    vpdpwssd       m17, m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmulld         m16, m10            ; p * s
    pmulld         m17, m10
    pmaddwd         m0, m11            ; b * 164
    pmaddwd         m1, m11
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m20
    pmaxsw         m17, m6
    paddusw        m17, m12
    psraw          m17, 4              ; min(z, 255) - 256
    vpermt2b       m16, m17, m21       ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16            ; x
    pandn          m16, m13, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    packssdw       m16, m17
    psubd           m0, m13            ; x * b * 164 + (1 << 11) + (1 << 15)
    psubd           m1, m13
    mova    [t4+r10+4], m16
    psrld          m16, m0, 12         ; b
    psrld          m17, m1, 12
    mova          [t3+r10*2+  8], xm16
    mova          [t3+r10*2+ 24], xm17
    vextracti128  [t3+r10*2+ 40], ym16, 1
    vextracti128  [t3+r10*2+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+ 72], m16, 2
    vextracti32x4 [t3+r10*2+ 88], m17, 2
    vextracti32x4 [t3+r10*2+104], m16, 3
    vextracti32x4 [t3+r10*2+120], m17, 3
    add            r10, 64
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    ret
.hv_last_row: ; esoteric edge case for odd heights
    mova [t1+r10+416*0], m1
    paddw            m1, m0
    mova [t1+r10+416*2], m16
    paddd           m16, m2
    mova [t1+r10+416*4], m17
    paddd           m17, m3
    jmp .hv_main2
.v: ; vertical boxsum + ab
    lea            r10, [wq-4]
.v_loop:
    mova            m2, [t1+r10+416*2]
    mova            m3, [t1+r10+416*4]
    mova            m0, [t1+r10+416*0]
    paddd          m16, m2, [t2+r10+416*2]
    paddd          m17, m3, [t2+r10+416*4]
    paddw           m1, m0, [t2+r10+416*0]
    paddd           m2, m2
    paddd           m3, m3
    paddd          m16, m2             ; hv sumsq
    paddd          m17, m3
    paddd          m16, m8
    paddd          m17, m8
    psrld          m16, 4              ; (a + 8) >> 4
    psrld          m17, 4
    pmulld         m16, m9             ; -a * 25
    pmulld         m17, m9
    paddw           m0, m0
    paddw           m1, m0             ; hv sum
    psrlw           m3, m1, 1
    pavgw           m3, m6             ; (b + 2) >> 2
    punpcklwd       m2, m3, m6
    vpdpwssd       m16, m2, m2         ; -p
    punpckhwd       m3, m6
    vpdpwssd       m17, m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmulld         m16, m10            ; p * s
    pmulld         m17, m10
    pmaddwd         m0, m11            ; b * 164
    pmaddwd         m1, m11
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m20
    pmaxsw         m17, m6
    paddusw        m17, m12
    psraw          m17, 4              ; min(z, 255) - 256
    vpermt2b       m16, m17, m21       ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16            ; x
    pandn          m16, m13, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    packssdw       m16, m17
    psubd           m0, m13            ; x * b * 164 + (1 << 11) + (1 << 15)
    psubd           m1, m13
    mova    [t4+r10+4], m16
    psrld          m16, m0, 12         ; b
    psrld          m17, m1, 12
    mova          [t3+r10*2+  8], xm16
    mova          [t3+r10*2+ 24], xm17
    vextracti128  [t3+r10*2+ 40], ym16, 1
    vextracti128  [t3+r10*2+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+ 72], m16, 2
    vextracti32x4 [t3+r10*2+ 88], m17, 2
    vextracti32x4 [t3+r10*2+104], m16, 3
    vextracti32x4 [t3+r10*2+120], m17, 3
    add            r10, 64
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu            m0, [t4+r10*1+ 2]
    movu            m1, [t3+r10*2+ 4]
    movu            m2, [t3+r10*2+68]
    paddw           m3, m0, [t4+r10*1+ 0]
    paddd          m16, m1, [t3+r10*2+ 0]
    paddd          m17, m2, [t3+r10*2+64]
    paddw           m3, [t4+r10*1+ 4]
    paddd          m16, [t3+r10*2+ 8]
    paddd          m17, [t3+r10*2+72]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m16
    pslld          m16, 2
    paddd           m2, m17
    pslld          m17, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m16            ; b 565
    paddd           m2, m17
    mova [t4+r10*1+416*2+ 0], m0
    mova [t3+r10*2+416*4+ 0], m1
    mova [t3+r10*2+416*4+64], m2
    add            r10, 64
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu            m0, [t4+r10*1+ 2]
    movu            m1, [t3+r10*2+ 4]
    movu            m2, [t3+r10*2+68]
    paddw           m3, m0, [t4+r10*1+ 0]
    paddd          m16, m1, [t3+r10*2+ 0]
    paddd          m17, m2, [t3+r10*2+64]
    paddw           m3, [t4+r10*1+ 4]
    paddd          m16, [t3+r10*2+ 8]
    paddd          m17, [t3+r10*2+72]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m16
    pslld          m16, 2
    paddd           m2, m17
    pslld          m17, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m16            ; b 565
    paddd           m2, m17
    paddw           m3, m0, [t4+r10*1+416*2+ 0]
    paddd          m16, m1, [t3+r10*2+416*4+ 0]
    paddd          m17, m2, [t3+r10*2+416*4+64]
    mova [t4+r10*1+416*2+ 0], m0
    mova [t3+r10*2+416*4+ 0], m1
    mova [t3+r10*2+416*4+64], m2
    mova            m0, [dstq+r10]
    punpcklwd       m1, m0, m6          ; src
    punpcklwd       m2, m3, m6          ; a
    pmaddwd         m2, m1              ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vshufi32x4      m1, m16, m17, q2020
    vshufi32x4     m16, m17, q3131
    psubd           m1, m2              ; b - a * src + (1 << 8)
    psubd          m16, m3
    psrad           m1, 9
    psrad          m16, 9
    packssdw        m1, m16
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova    [dstq+r10], m0
    add            r10, 64
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova            m0, [dstq+r10]
    mova            m3, [t4+r10*1+416*2+ 0]
    mova           m16, [t3+r10*2+416*4+ 0]
    mova           m17, [t3+r10*2+416*4+64]
    punpcklwd       m1, m0, m6          ; src
    punpcklwd       m2, m3, m6          ; a
    pmaddwd         m2, m1
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vshufi32x4      m1, m16, m17, q2020
    vshufi32x4     m16, m17, q3131
    psubd           m1, m2              ; b - a * src + (1 << 7)
    psubd          m16, m3
    psrad           m1, 8
    psrad          m16, 8
    packssdw        m1, m16
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova    [dstq+r10], m0
    add            r10, 64
    jl .n1_loop
    add           dstq, strideq
    ret

cglobal sgr_filter_3x3_16bpc, 4, 14, 22, 416*42+8, dst, stride, left, lpf, \
                                                   w, h, edge, params
    movifnidn       wd, wm
    mov        paramsq, r6mp
    lea            r13, [r_ext_mask+72]
    mov          edged, r7m
    movifnidn       hd, hm
    pxor            m6, m6
    vpbroadcastw    m7, [paramsq+10] ; w1
    add             wd, wd
    vpbroadcastd    m8, [base+pd_8]
    add           lpfq, wq
    vpbroadcastd    m9, [base+pd_m9]
    add           dstq, wq
    vpsubd         m10, m6, [paramsq+4] {1to16} ; -s1
    lea             t3, [rsp+wq*2+416*12+8]
    vpbroadcastd   m11, [base+pw_164_455]
    lea             t4, [rsp+wq+416*32+8]
    vpbroadcastd   m12, [base+pw_61448]
    lea             t1, [rsp+wq+12]
    vpbroadcastd   m13, [base+pd_m34816]
    neg             wq
    vpbroadcastd   m14, [base+pw_1023]
    psllw           m7, 4
    mova           m18, [sgr_x_by_x+64*0]
    mov           r10d, 0xfffffffc
    mova           m19, [sgr_x_by_x+64*1]
    kmovd           k1, r10d
    mova           m20, [sgr_x_by_x+64*2]
    mov            r10, 0x3333333333333333
    mova           m21, [sgr_x_by_x+64*3]
    kmovq           k2, r10
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    add             t1, 416*6
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
    lea             t2, [t1+416*6]
.top_fixup_loop:
    mova            m0, [t1+r10+416*0]
    mova            m1, [t1+r10+416*2]
    mova            m2, [t1+r10+416*4]
    mova [t2+r10+416*0], m0
    mova [t2+r10+416*2], m1
    mova [t2+r10+416*4], m2
    add            r10, 64
    jl .top_fixup_loop
    call .v0
    jmp .main
.h: ; horizontal boxsum
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movd          xm16, [leftq+4]
    vmovdqu16  m16{k1}, [lpfq+wq-4]
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-4]
    jmp .h_main
.h_top:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu           m16, [lpfq+r10+ 0]
.h_main:
    movu           m17, [lpfq+r10+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -66
    jl .h_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.h_have_right:
    palignr         m0, m17, m16, 2
    paddw           m1, m16, m0
    punpcklwd       m2, m16, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m16, m0
    pmaddwd         m3, m3
    palignr        m17, m16, 4
    paddw           m1, m17            ; sum
    punpcklwd      m16, m17, m6
    vpdpwssd        m2, m16, m16       ; sumsq
    punpckhwd      m17, m6
    vpdpwssd        m3, m17, m17
    mova [t1+r10+416*0], m1
    mova [t1+r10+416*2], m2
    mova [t1+r10+416*4], m3
    add            r10, 64
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab (even rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movd          xm16, [leftq+4]
    vmovdqu16  m16{k1}, [lpfq+wq-4]
    add          leftq, 8
    jmp .hv0_main
.hv0_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-4]
    jmp .hv0_main
.hv0_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
.hv0_loop:
    movu           m16, [lpfq+r10+ 0]
.hv0_main:
    movu           m17, [lpfq+r10+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp           r10d, -66
    jl .hv0_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.hv0_have_right:
    palignr         m0, m17, m16, 2
    paddw           m1, m16, m0
    punpcklwd       m2, m16, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m16, m0
    pmaddwd         m3, m3
    palignr        m17, m16, 4
    paddw           m1, m17            ; sum
    punpcklwd      m16, m17, m6
    vpdpwssd        m2, m16, m16       ; sumsq
    punpckhwd      m17, m6
    vpdpwssd        m3, m17, m17
    paddw           m0, m1, [t1+r10+416*0]
    paddd          m16, m2, [t1+r10+416*2]
    paddd          m17, m3, [t1+r10+416*4]
    mova [t1+r10+416*0], m1
    mova [t1+r10+416*2], m2
    mova [t1+r10+416*4], m3
    paddw           m1, m0, [t2+r10+416*0]
    paddd           m2, m16, [t2+r10+416*2]
    paddd           m3, m17, [t2+r10+416*4]
    mova [t2+r10+416*0], m0
    mova [t2+r10+416*2], m16
    mova [t2+r10+416*4], m17
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m9             ; -((a + 8) >> 4) * 9
    pmulld          m3, m9
    psrlw          m17, m1, 1
    pavgw          m17, m6             ; (b + 2) >> 2
    punpcklwd      m16, m17, m6
    vpdpwssd        m2, m16, m16       ; -p
    punpckhwd      m17, m6
    vpdpwssd        m3, m17, m17
    punpcklwd      m16, m6, m1         ; b
    punpckhwd      m17, m6, m1
    pminsd          m2, m6
    pminsd          m3, m6
    pmulld          m2, m10            ; p * s
    pmulld          m3, m10
    pmaddwd        m16, m11            ; b * 455
    pmaddwd        m17, m11
    vpalignr    m3{k2}, m2, m2, 2
    mova            m2, m20
    paddusw         m3, m12
    psraw           m3, 4              ; min(z, 255) - 256
    vpermt2b        m2, m3, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m3
    vpermi2b        m3, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m3{k3}, m2             ; x
    pandn           m2, m13, m3
    psrld           m3, 16
    pmulld         m16, m2
    pmulld         m17, m3
    packssdw        m2, m3
    psubd          m16, m13            ; x * b * 455 + (1 << 11) + (1 << 15)
    psubd          m17, m13
    mova [t4+r10*1+416*0+4], m2
    psrld          m16, 12
    psrld          m17, 12
    mova          [t3+r10*2+416*0+  8], xm16
    mova          [t3+r10*2+416*0+ 24], xm17
    vextracti128  [t3+r10*2+416*0+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*0+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*0+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*0+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*0+104], m16, 3
    vextracti32x4 [t3+r10*2+416*0+120], m17, 3
    add            r10, 64
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movd          xm16, [leftq+4]
    vmovdqu16  m16{k1}, [lpfq+wq-4]
    add          leftq, 8
    jmp .hv1_main
.hv1_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-4]
    jmp .hv1_main
.hv1_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
.hv1_loop:
    movu           m16, [lpfq+r10+ 0]
.hv1_main:
    movu           m17, [lpfq+r10+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp           r10d, -66
    jl .hv1_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.hv1_have_right:
    palignr         m1, m17, m16, 2
    paddw           m0, m16, m1
    punpcklwd       m2, m16, m1
    pmaddwd         m2, m2
    punpckhwd       m3, m16, m1
    pmaddwd         m3, m3
    palignr        m17, m16, 4
    paddw           m0, m17            ; h sum
    punpcklwd       m1, m17, m6
    vpdpwssd        m2, m1, m1         ; h sumsq
    punpckhwd      m17, m6
    vpdpwssd        m3, m17, m17
    paddw           m1, m0, [t2+r10+416*0]
    paddd          m16, m2, [t2+r10+416*2]
    paddd          m17, m3, [t2+r10+416*4]
    mova [t2+r10+416*0], m0
    mova [t2+r10+416*2], m2
    mova [t2+r10+416*4], m3
    paddd          m16, m8
    paddd          m17, m8
    psrld          m16, 4              ; (a + 8) >> 4
    psrld          m17, 4
    pmulld         m16, m9             ; -((a + 8) >> 4) * 9
    pmulld         m17, m9
    psrlw           m3, m1, 1
    pavgw           m3, m6             ; (b + 2) >> 2
    punpcklwd       m2, m3, m6
    vpdpwssd       m16, m2, m2         ; -p
    punpckhwd       m3, m6
    vpdpwssd       m17, m3, m3
    punpcklwd       m0, m6, m1         ; b
    punpckhwd       m1, m6, m1
    pminsd         m16, m6
    pminsd         m17, m6
    pmulld         m16, m10            ; p * s
    pmulld         m17, m10
    pmaddwd         m0, m11            ; b * 455
    pmaddwd         m1, m11
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m20
    paddusw        m17, m12
    psraw          m17, 4              ; min(z, 255) - 256
    vpermt2b       m16, m17, m21       ; sgr_x_by_x[128..255]
    vpmovb2m       k3, m17
    vpermi2b       m17, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16            ; x
    pandn          m16, m13, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    packssdw       m16, m17
    psubd           m0, m13            ; x * b * 455 + (1 << 11) + (1 << 15)
    psubd           m1, m13
    mova [t4+r10*1+416*2+4], m16
    psrld          m16, m0, 12
    psrld          m17, m1, 12
    mova          [t3+r10*2+416*4+  8], xm16
    mova          [t3+r10*2+416*4+ 24], xm17
    vextracti128  [t3+r10*2+416*4+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*4+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*4+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*4+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*4+104], m16, 3
    vextracti32x4 [t3+r10*2+416*4+120], m17, 3
    add            r10, 64
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab (even rows)
    lea            r10, [wq-4]
.v0_loop:
    mova            m0, [t1+r10+416*0]
    mova           m16, [t1+r10+416*2]
    mova           m17, [t1+r10+416*4]
    paddw           m0, m0
    paddd          m16, m16
    paddd          m17, m17
    paddw           m1, m0, [t2+r10+416*0]
    paddd           m2, m16, [t2+r10+416*2]
    paddd           m3, m17, [t2+r10+416*4]
    mova [t2+r10+416*0], m0
    mova [t2+r10+416*2], m16
    mova [t2+r10+416*4], m17
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m9             ; -((a + 8) >> 4) * 9
    pmulld          m3, m9
    psrlw          m17, m1, 1
    pavgw          m17, m6             ; (b + 2) >> 2
    punpcklwd      m16, m17, m6
    vpdpwssd        m2, m16, m16       ; -p
    punpckhwd      m17, m6
    vpdpwssd        m3, m17, m17
    punpcklwd      m16, m6, m1         ; b
    punpckhwd      m17, m6, m1
    pminsd          m2, m6
    pminsd          m3, m6
    pmulld          m2, m10            ; p * s
    pmulld          m3, m10
    pmaddwd        m16, m11            ; b * 455
    pmaddwd        m17, m11
    vpalignr    m3{k2}, m2, m2, 2
    mova            m2, m20
    paddusw         m3, m12
    psraw           m3, 4              ; min(z, 255) - 256
    vpermt2b        m2, m3, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m3
    vpermi2b        m3, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m3{k3}, m2             ; x
    pandn           m2, m13, m3
    psrld           m3, 16
    pmulld         m16, m2
    pmulld         m17, m3
    packssdw        m2, m3
    psubd          m16, m13            ; x * b * 455 + (1 << 11) + (1 << 15)
    psubd          m17, m13
    mova [t4+r10*1+416*0+4], m2
    psrld          m16, 12
    psrld          m17, 12
    mova          [t3+r10*2+416*0+  8], xm16
    mova          [t3+r10*2+416*0+ 24], xm17
    vextracti128  [t3+r10*2+416*0+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*0+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*0+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*0+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*0+104], m16, 3
    vextracti32x4 [t3+r10*2+416*0+120], m17, 3
    add            r10, 64
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
.v1_loop:
    mova            m0, [t1+r10+416*0]
    mova           m16, [t1+r10+416*2]
    mova           m17, [t1+r10+416*4]
    paddw           m1, m0, [t2+r10+416*0]
    paddd           m2, m16, [t2+r10+416*2]
    paddd           m3, m17, [t2+r10+416*4]
    mova [t2+r10+416*0], m0
    mova [t2+r10+416*2], m16
    mova [t2+r10+416*4], m17
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m9             ; -((a + 8) >> 4) * 9
    pmulld          m3, m9
    psrlw          m17, m1, 1
    pavgw          m17, m6             ; (b + 2) >> 2
    punpcklwd      m16, m17, m6
    vpdpwssd        m2, m16, m16       ; -p
    punpckhwd      m17, m6
    vpdpwssd        m3, m17, m17
    punpcklwd      m16, m6, m1         ; b
    punpckhwd      m17, m6, m1
    pminsd          m2, m6
    pminsd          m3, m6
    pmulld          m2, m10            ; p * s
    pmulld          m3, m10
    pmaddwd        m16, m11            ; b * 455
    pmaddwd        m17, m11
    vpalignr    m3{k2}, m2, m2, 2
    mova            m2, m20
    paddusw         m3, m12
    psraw           m3, 4              ; min(z, 255) - 256
    vpermt2b        m2, m3, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m3
    vpermi2b        m3, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m3{k3}, m2             ; x
    pandn           m2, m13, m3
    psrld           m3, 16
    pmulld         m16, m2
    pmulld         m17, m3
    packssdw        m2, m3
    psubd          m16, m13            ; x * b * 455 + (1 << 11) + (1 << 15)
    psubd          m17, m13
    mova [t4+r10*1+416*2+4], m2
    psrld          m16, 12
    psrld          m17, 12
    mova          [t3+r10*2+416*4+  8], xm16
    mova          [t3+r10*2+416*4+ 24], xm17
    vextracti128  [t3+r10*2+416*4+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*4+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*4+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*4+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*4+104], m16, 3
    vextracti32x4 [t3+r10*2+416*4+120], m17, 3
    add            r10, 64
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    mova          ym16, [t4+r10*1+416*0+0]
    paddw         ym16, [t4+r10*1+416*0+4]
    paddw         ym17, ym16, [t4+r10*1+416*0+2]
    mova            m0, [t3+r10*2+416*0+0]
    paddd           m0, [t3+r10*2+416*0+8]
    paddd           m1, m0, [t3+r10*2+416*0+4]
    psllw         ym17, 2                ; a[-1] 444
    pslld           m1, 2                ; b[-1] 444
    psubw         ym17, ym16             ; a[-1] 343
    psubd           m1, m0               ; b[-1] 343
    vmovdqa32 [t4+r10*1+416* 4], ym17
    vmovdqa32 [t3+r10*2+416* 8], m1
    mova          ym16, [t4+r10*1+416*2+0]
    paddw         ym16, [t4+r10*1+416*2+4]
    paddw         ym17, ym16, [t4+r10*1+416*2+2]
    mova            m0, [t3+r10*2+416*4+0]
    paddd           m0, [t3+r10*2+416*4+8]
    paddd           m1, m0, [t3+r10*2+416*4+4]
    psllw         ym17, 2                 ; a[ 0] 444
    pslld           m1, 2                 ; b[ 0] 444
    vmovdqa32 [t4+r10*1+416* 6], ym17
    vmovdqa32 [t3+r10*2+416*12], m1
    psubw         ym17, ym16              ; a[ 0] 343
    psubd           m1, m0                ; b[ 0] 343
    vmovdqa32 [t4+r10*1+416* 8], ym17
    vmovdqa32 [t3+r10*2+416*16], m1
    add            r10, 32
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    mova            m3, [t4+r10*1+416*0+0]
    paddw           m3, [t4+r10*1+416*0+4]
    paddw           m1, m3, [t4+r10*1+416*0+2]
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+r10*1+416*4]
    paddw           m3, [t4+r10*1+416*6]
    mova [t4+r10*1+416*4], m2
    mova [t4+r10*1+416*6], m1
    mova           m16, [t3+r10*2+416*0+0]
    paddd          m16, [t3+r10*2+416*0+8]
    paddd           m1, m16, [t3+r10*2+416*0+4]
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m16          ; b[ 1] 343
    paddd          m16, m2, [t3+r10*2+416* 8+ 0]
    paddd          m16, [t3+r10*2+416*12+ 0]
    mova [t3+r10*2+416* 8+ 0], m2
    mova [t3+r10*2+416*12+ 0], m1
    mova           m17, [t3+r10*2+416*0+64]
    paddd          m17, [t3+r10*2+416*0+72]
    paddd           m1, m17, [t3+r10*2+416*0+68]
    pslld           m1, 2
    psubd           m2, m1, m17
    paddd          m17, m2, [t3+r10*2+416* 8+64]
    paddd          m17, [t3+r10*2+416*12+64]
    mova [t3+r10*2+416* 8+64], m2
    mova [t3+r10*2+416*12+64], m1
    mova            m0, [dstq+r10]
    punpcklwd       m1, m0, m6
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m1               ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vshufi32x4      m1, m16, m17, q2020
    vshufi32x4     m16, m17, q3131
    psubd           m1, m2               ; b - a * src + (1 << 8)
    psubd          m16, m3
    psrad           m1, 9
    psrad          m16, 9
    packssdw        m1, m16
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova    [dstq+r10], m0
    add            r10, 64
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova            m3, [t4+r10*1+416*2+0]
    paddw           m3, [t4+r10*1+416*2+4]
    paddw           m1, m3, [t4+r10*1+416*2+2]
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+r10*1+416*6]
    paddw           m3, [t4+r10*1+416*8]
    mova [t4+r10*1+416*6], m1
    mova [t4+r10*1+416*8], m2
    mova           m16, [t3+r10*2+416*4+0]
    paddd          m16, [t3+r10*2+416*4+8]
    paddd           m1, m16, [t3+r10*2+416*4+4]
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m16          ; b[ 1] 343
    paddd          m16, m2, [t3+r10*2+416*12+ 0]
    paddd          m16, [t3+r10*2+416*16+ 0]
    mova [t3+r10*2+416*12+ 0], m1
    mova [t3+r10*2+416*16+ 0], m2
    mova           m17, [t3+r10*2+416*4+64]
    paddd          m17, [t3+r10*2+416*4+72]
    paddd           m1, m17, [t3+r10*2+416*4+68]
    pslld           m1, 2
    psubd           m2, m1, m17
    paddd          m17, m2, [t3+r10*2+416*12+64]
    paddd          m17, [t3+r10*2+416*16+64]
    mova [t3+r10*2+416*12+64], m1
    mova [t3+r10*2+416*16+64], m2
    mova            m0, [dstq+r10]
    punpcklwd       m1, m0, m6
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m1               ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    vshufi32x4      m1, m16, m17, q2020
    vshufi32x4     m16, m17, q3131
    psubd           m1, m2               ; b - a * src + (1 << 8)
    psubd          m16, m3
    psrad           m1, 9
    psrad          m16, 9
    packssdw        m1, m16
    pmulhrsw        m1, m7
    paddw           m0, m1
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova    [dstq+r10], m0
    add            r10, 64
    jl .n1_loop
    add           dstq, strideq
    ret

cglobal sgr_filter_mix_16bpc, 4, 14, 23, 416*66+8, dst, stride, left, lpf, \
                                                   w, h, edge, params
    movifnidn       wd, wm
    mov        paramsq, r6mp
    lea            r13, [r_ext_mask+72]
    mov          edged, r7m
    movifnidn       hd, hm
    vpbroadcastd    m7, [paramsq+8] ; w0 w1
    pxor            m6, m6
    vpbroadcastd    m8, [base+pd_8]
    add             wd, wd
    vpbroadcastd    m9, [base+pd_m9]
    add           lpfq, wq
    vpbroadcastd   m10, [base+pd_m25]
    add           dstq, wq
    vpsubd         m11, m6, [paramsq+0] {1to16} ; -s0
    lea             t3, [rsp+wq*2+416*24+8]
    vpsubd         m12, m6, [paramsq+4] {1to16} ; -s1
    lea             t4, [rsp+wq+416*52+8]
    vpbroadcastd   m13, [base+pw_164_455]
    lea             t1, [rsp+wq+12]
    vpbroadcastd   m14, [base+pw_61448]
    neg             wq
    vpbroadcastd   m15, [base+pd_m34816]
    psllw           m7, 2
    vpbroadcastd   m22, [base+pd_2147483648]
    mov           r10d, 0xfffffff8
    mova           m18, [sgr_x_by_x+64*0]
    kmovd           k1, r10d
    mova           m19, [sgr_x_by_x+64*1]
    mov            r10, 0x3333333333333333
    mova           m20, [sgr_x_by_x+64*2]
    kmovq           k2, r10
    mova           m21, [sgr_x_by_x+64*3]
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_avx512icl).top_fixup
    add             t1, 416*12
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
    lea             t2, [t1+416*12]
.top_fixup_loop:
    mova            m0, [t1+r10+416* 0]
    mova            m1, [t1+r10+416* 2]
    mova            m2, [t1+r10+416* 4]
    paddw           m0, m0
    mova            m3, [t1+r10+416* 6]
    paddd           m1, m1
    mova            m4, [t1+r10+416* 8]
    paddd           m2, m2
    mova            m5, [t1+r10+416*10]
    mova [t2+r10+416* 0], m0
    mova [t2+r10+416* 2], m1
    mova [t2+r10+416* 4], m2
    mova [t2+r10+416* 6], m3
    mova [t2+r10+416* 8], m4
    mova [t2+r10+416*10], m5
    add            r10, 64
    jl .top_fixup_loop
    call .v0
    jmp .main
.h: ; horizontal boxsum
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movq          xm16, [leftq+2]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    jmp .h_main
.h_top:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu           m16, [lpfq+r10- 2]
.h_main:
    movu           m17, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -68
    jl .h_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.h_have_right:
    palignr         m3, m17, m16, 2
    palignr         m0, m17, m16, 4
    paddw           m1, m3, m0
    punpcklwd       m2, m3, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m0
    pmaddwd         m3, m3
    palignr         m0, m17, m16, 6
    paddw           m1, m0             ; sum3
    punpcklwd       m4, m0, m6
    vpdpwssd        m2, m4, m4         ; sumsq3
    punpckhwd       m0, m6
    vpdpwssd        m3, m0, m0
    shufpd          m4, m16, m17, 0x55
    punpcklwd      m17, m4, m16
    paddw           m0, m16, m4
    punpckhwd       m4, m16
    mova [t1+r10+416* 6], m1
    mova [t1+r10+416* 8], m2
    mova [t1+r10+416*10], m3
    paddw           m1, m0             ; sum5
    vpdpwssd        m2, m17, m17       ; sumsq5
    vpdpwssd        m3, m4, m4
    mova [t1+r10+416* 0], m1
    mova [t1+r10+416* 2], m2
    mova [t1+r10+416* 4], m3
    add            r10, 64
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab3 (even rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movq          xm16, [leftq+2]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    add          leftq, 8
    jmp .hv0_main
.hv0_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    jmp .hv0_main
.hv0_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
.hv0_loop:
    movu           m16, [lpfq+r10- 2]
.hv0_main:
    movu           m17, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp           r10d, -68
    jl .hv0_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.hv0_have_right:
    palignr         m3, m17, m16, 2
    palignr         m0, m17, m16, 4
    paddw           m1, m3, m0
    punpcklwd       m2, m3, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m0
    pmaddwd         m3, m3
    palignr         m0, m17, m16, 6
    paddw           m1, m0             ; h sum3
    punpcklwd       m4, m0, m6
    vpdpwssd        m2, m4, m4         ; h sumsq3
    punpckhwd       m0, m6
    vpdpwssd        m3, m0, m0
    shufpd         m17, m16, m17, 0x55
    paddw           m4, m1, [t1+r10+416* 6]
    paddd           m5, m2, [t1+r10+416* 8]
    mova [t1+r10+416* 6], m1
    mova [t1+r10+416* 8], m2
    paddw           m1, m16
    paddw           m1, m17            ; h sum5
    punpcklwd       m0, m17, m16
    vpdpwssd        m2, m0, m0         ; h sumsq5
    paddd           m0, m3, [t1+r10+416*10]
    mova [t1+r10+416*10], m3
    punpckhwd      m17, m16
    vpdpwssd        m3, m17, m17
    mova [t3+r10*2+416*8+ 8], m1       ; we need a clean copy of the last row
    mova [t3+r10*2+416*0+ 8], m2       ; in case height is odd
    mova [t3+r10*2+416*0+72], m3
    paddw           m1, [t1+r10+416* 0]
    paddd           m2, [t1+r10+416* 2]
    paddd           m3, [t1+r10+416* 4]
    mova [t1+r10+416* 0], m1
    mova [t1+r10+416* 2], m2
    mova [t1+r10+416* 4], m3
    paddw          m17, m4, [t2+r10+416* 6]
    paddd           m2, m5, [t2+r10+416* 8]
    paddd           m3, m0, [t2+r10+416*10]
    mova [t2+r10+416* 6], m4
    mova [t2+r10+416* 8], m5
    mova [t2+r10+416*10], m0
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m9             ; -((a3 + 8) >> 4) * 9
    pmulld          m3, m9
    psrlw           m5, m17, 1
    pavgw           m5, m6             ; (b3 + 2) >> 2
    punpcklwd       m4, m5, m6
    vpdpwssd        m2, m4, m4         ; -p3
    punpckhwd       m5, m6
    vpdpwssd        m3, m5, m5
    punpcklwd      m16, m6, m17        ; b3
    punpckhwd      m17, m6, m17
    pminsd          m2, m6
    pminsd          m3, m6
    pmulld          m2, m12            ; p3 * s1
    pmulld          m3, m12
    pmaddwd        m16, m13            ; b3 * 455
    pmaddwd        m17, m13
    vpalignr    m3{k2}, m2, m2, 2
    mova            m2, m20
    paddusw         m3, m14
    psraw           m3, 4              ; min(z3, 255) - 256
    vpermt2b        m2, m3, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m3
    vpermi2b        m3, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m3{k3}, m2             ; x3
    pandn           m2, m15, m3
    psrld           m3, 16
    pmulld         m16, m2
    pmulld         m17, m3
    packssdw        m2, m3
    mova [t4+r10*1+416*2+4], m2
    psubd          m16, m15            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    psubd          m17, m15
    psrld          m16, 12
    psrld          m17, 12
    mova          [t3+r10*2+416*4+  8], xm16
    mova          [t3+r10*2+416*4+ 24], xm17
    vextracti128  [t3+r10*2+416*4+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*4+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*4+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*4+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*4+104], m16, 3
    vextracti32x4 [t3+r10*2+416*4+120], m17, 3
    add            r10, 64
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movq          xm16, [leftq+2]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    add          leftq, 8
    jmp .hv1_main
.hv1_extend_left:
    vpbroadcastw  xm16, [lpfq+wq]
    vmovdqu16  m16{k1}, [lpfq+wq-6]
    jmp .hv1_main
.hv1_bottom:
    lea            r10, [wq-4]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
.hv1_loop:
    movu           m16, [lpfq+r10- 2]
.hv1_main:
    movu           m17, [lpfq+r10+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp           r10d, -68
    jl .hv1_have_right
    vpbroadcastw    m0, [lpfq-2]
    vpternlogd     m16, m0, [r13+r10+ 0], 0xe4
    vpternlogd     m17, m0, [r13+r10+16], 0xe4
.hv1_have_right:
    palignr         m1, m17, m16, 2
    palignr         m3, m17, m16, 4
    paddw           m2, m1, m3
    punpcklwd       m0, m1, m3
    pmaddwd         m0, m0
    punpckhwd       m1, m3
    pmaddwd         m1, m1
    palignr         m3, m17, m16, 6
    paddw           m2, m3             ; h sum3
    punpcklwd       m5, m3, m6
    vpdpwssd        m0, m5, m5         ; h sumsq3
    punpckhwd       m3, m6
    vpdpwssd        m1, m3, m3
    shufpd          m3, m16, m17, 0x55
    punpcklwd       m5, m16, m3
    paddw           m4, m16, m3
    punpckhwd      m16, m3
    paddw          m17, m2, [t2+r10+416* 6]
    mova [t2+r10+416* 6], m2
    paddw           m4, m2             ; h sum5
    paddd           m2, m0, [t2+r10+416* 8]
    paddd           m3, m1, [t2+r10+416*10]
    mova [t2+r10+416* 8], m0
    mova [t2+r10+416*10], m1
    vpdpwssd        m0, m5, m5         ; h sumsq5
    vpdpwssd        m1, m16, m16
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m9             ; -((a3 + 8) >> 4) * 9
    pmulld          m3, m9
    psrlw          m16, m17, 1
    pavgw          m16, m6             ; (b3 + 2) >> 2
    punpcklwd       m5, m16, m6
    vpdpwssd        m2, m5, m5         ; -p3
    punpckhwd      m16, m6
    vpdpwssd        m3, m16, m16
    punpcklwd      m16, m6, m17        ; b3
    punpckhwd      m17, m6, m17
    pminsd          m2, m6
    pminsd          m3, m6
    pmulld          m2, m12            ; p3 * s1
    pmulld          m3, m12
    pmaddwd        m16, m13            ; b3 * 455
    pmaddwd        m17, m13
    vpalignr    m3{k2}, m2, m2, 2
    mova            m2, m20
    paddusw         m3, m14
    psraw           m3, 4              ; min(z3, 255) - 256
    vpermt2b        m2, m3, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m3
    vpermi2b        m3, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m3{k3}, m2             ; x3
    pandn           m2, m15, m3
    psrld           m3, 16
    pmulld         m16, m2
    pmulld         m17, m3
    packssdw        m2, m3
    mova [t4+r10*1+416*4+4], m2
    psubd          m16, m15            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    psubd          m17, m15
    psrld          m16, 12
    psrld          m17, 12
    paddw           m5, m4, [t2+r10+416*0]
    paddd           m2, m0, [t2+r10+416*2]
    paddd           m3, m1, [t2+r10+416*4]
    paddw           m5, [t1+r10+416*0]
    paddd           m2, [t1+r10+416*2]
    paddd           m3, [t1+r10+416*4]
    mova [t2+r10+416*0], m4
    mova [t2+r10+416*2], m0
    mova [t2+r10+416*4], m1
    mova          [t3+r10*2+416*8+  8], xm16
    mova          [t3+r10*2+416*8+ 24], xm17
    vextracti128  [t3+r10*2+416*8+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*8+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*8+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*8+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*8+104], m16, 3
    vextracti32x4 [t3+r10*2+416*8+120], m17, 3
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a5 + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m10            ; -((a5 + 8) >> 4) * 25
    pmulld          m3, m10
    psrlw          m17, m5, 1
    pavgw          m17, m6             ; (b5 + 2) >> 2
    punpcklwd      m16, m17, m6
    vpdpwssd        m2, m16, m16       ; -p5
    punpckhwd      m17, m6
    vpdpwssd        m3, m17, m17
    punpcklwd      m16, m5, m6         ; b5
    punpckhwd      m17, m5, m6
    pmulld          m2, m11            ; p5 * s0
    pmulld          m3, m11
    pmaddwd        m16, m13            ; b5 * 164
    pmaddwd        m17, m13
    vpalignr    m3{k2}, m2, m2, 2
    mova            m2, m20
    pmaxsw          m3, m6
    paddusw         m3, m14
    psraw           m3, 4              ; min(z5, 255) - 256
    vpermt2b        m2, m3, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m3
    vpermi2b        m3, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m3{k3}, m2             ; x5
    pandn           m2, m15, m3
    psrld           m3, 16
    pmulld         m16, m2
    pmulld         m17, m3
    packssdw        m2, m3
    mova [t4+r10*1+416*0+4], m2
    psubd          m16, m15            ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    psubd          m17, m15
    psrld          m16, 12
    psrld          m17, 12
    mova          [t3+r10*2+416*0+  8], xm16
    mova          [t3+r10*2+416*0+ 24], xm17
    vextracti128  [t3+r10*2+416*0+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*0+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*0+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*0+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*0+104], m16, 3
    vextracti32x4 [t3+r10*2+416*0+120], m17, 3
    add            r10, 64
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab3 (even rows)
    lea            r10, [wq-4]
.v0_loop:
    mova           m16, [t1+r10+416* 6]
    mova            m2, [t1+r10+416* 8]
    mova            m3, [t1+r10+416*10]
    paddw          m16, m16
    paddd           m2, m2
    paddd           m3, m3
    paddw          m17, m16, [t2+r10+416* 6]
    paddd           m4, m2, [t2+r10+416* 8]
    paddd           m5, m3, [t2+r10+416*10]
    mova [t2+r10+416* 6], m16
    mova [t2+r10+416* 8], m2
    mova [t2+r10+416*10], m3
    paddd           m4, m8
    paddd           m5, m8
    psrld           m4, 4              ; (a3 + 8) >> 4
    psrld           m5, 4
    pmulld          m4, m9             ; -((a3 + 8) >> 4) * 9
    pmulld          m5, m9
    psrlw           m3, m17, 1
    pavgw           m3, m6             ; (b3 + 2) >> 2
    punpcklwd       m2, m3, m6
    vpdpwssd        m4, m2, m2         ; -p3
    punpckhwd       m3, m6
    vpdpwssd        m5, m3, m3
    punpcklwd      m16, m6, m17        ; b3
    punpckhwd      m17, m6, m17
    pminsd          m4, m6
    pminsd          m5, m6
    pmulld          m4, m12            ; p3 * s1
    pmulld          m5, m12
    pmaddwd        m16, m13            ; b3 * 455
    pmaddwd        m17, m13
    vpalignr    m5{k2}, m4, m4, 2
    mova            m4, m20
    paddusw         m5, m14
    psraw           m5, 4              ; min(z3, 255) - 256
    vpermt2b        m4, m5, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m5
    vpermi2b        m5, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m5{k3}, m4             ; x3
    pandn           m4, m15, m5
    psrld           m5, 16
    pmulld         m16, m4
    pmulld         m17, m5
    packssdw        m4, m5
    mova [t4+r10*1+416*2+4], m4
    psubd          m16, m15            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    psubd          m17, m15
    psrld          m16, 12
    psrld          m17, 12
    mova            m3, [t1+r10+416*0]
    mova            m4, [t1+r10+416*2]
    mova            m5, [t1+r10+416*4]
    mova [t3+r10*2+416*8+ 8], m3
    mova [t3+r10*2+416*0+ 8], m4
    mova [t3+r10*2+416*0+72], m5
    paddw           m3, m3              ; cc5
    paddd           m4, m4
    paddd           m5, m5
    mova [t1+r10+416*0], m3
    mova [t1+r10+416*2], m4
    mova [t1+r10+416*4], m5
    mova          [t3+r10*2+416*4+  8], xm16
    mova          [t3+r10*2+416*4+ 24], xm17
    vextracti128  [t3+r10*2+416*4+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*4+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*4+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*4+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*4+104], m16, 3
    vextracti32x4 [t3+r10*2+416*4+120], m17, 3
    add            r10, 64
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
    lea            r10, [wq-4]
.v1_loop:
    mova           m16, [t1+r10+416* 6]
    mova            m2, [t1+r10+416* 8]
    mova            m3, [t1+r10+416*10]
    paddw          m17, m16, [t2+r10+416* 6]
    paddd           m4, m2, [t2+r10+416* 8]
    paddd           m5, m3, [t2+r10+416*10]
    mova [t2+r10+416* 6], m16
    mova [t2+r10+416* 8], m2
    mova [t2+r10+416*10], m3
    paddd           m4, m8
    paddd           m5, m8
    psrld           m4, 4              ; (a3 + 8) >> 4
    psrld           m5, 4
    pmulld          m4, m9              ; -((a3 + 8) >> 4) * 9
    pmulld          m5, m9
    psrlw           m3, m17, 1
    pavgw           m3, m6             ; (b3 + 2) >> 2
    punpcklwd       m2, m3, m6
    vpdpwssd        m4, m2, m2         ; -p3
    punpckhwd       m3, m6
    vpdpwssd        m5, m3, m3
    punpcklwd      m16, m6, m17        ; b3
    punpckhwd      m17, m6, m17
    pminsd          m4, m6
    pminsd          m5, m6
    pmulld          m4, m12            ; p3 * s1
    pmulld          m5, m12
    pmaddwd        m16, m13            ; b3 * 455
    pmaddwd        m17, m13
    vpalignr    m5{k2}, m4, m4, 2
    mova            m4, m20
    paddusw         m5, m14
    psraw           m5, 4              ; min(z3, 255) - 256
    vpermt2b        m4, m5, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m5
    vpermi2b        m5, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m5{k3}, m4             ; x3
    pandn           m4, m15, m5
    psrld           m5, 16
    pmulld         m16, m4
    pmulld         m17, m5
    packssdw        m4, m5
    mova [t4+r10*1+416*4+4], m4
    psubd          m16, m15            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    psubd          m17, m15
    psrld          m16, 12
    psrld          m17, 12
    mova            m0, [t3+r10*2+416*8+ 8]
    mova            m4, [t3+r10*2+416*0+ 8]
    mova            m5, [t3+r10*2+416*0+72]
    paddw           m1, m0, [t2+r10+416*0]
    paddd           m2, m4, [t2+r10+416*2]
    paddd           m3, m5, [t2+r10+416*4]
    paddw           m1, [t1+r10+416*0]
    paddd           m2, [t1+r10+416*2]
    paddd           m3, [t1+r10+416*4]
    mova [t2+r10+416*0], m0
    mova [t2+r10+416*2], m4
    mova [t2+r10+416*4], m5
    mova          [t3+r10*2+416*8+  8], xm16
    mova          [t3+r10*2+416*8+ 24], xm17
    vextracti128  [t3+r10*2+416*8+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*8+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*8+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*8+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*8+104], m16, 3
    vextracti32x4 [t3+r10*2+416*8+120], m17, 3
    paddd           m2, m8
    paddd           m3, m8
    psrld           m2, 4              ; (a5 + 8) >> 4
    psrld           m3, 4
    pmulld          m2, m10            ; -((a5 + 8) >> 4) * 25
    pmulld          m3, m10
    psrlw           m5, m1, 1
    pavgw           m5, m6             ; (b5 + 2) >> 2
    punpcklwd       m4, m5, m6
    vpdpwssd        m2, m4, m4         ; -p5
    punpckhwd       m5, m6
    vpdpwssd        m3, m5, m5
    punpcklwd      m16, m1, m6         ; b5
    punpckhwd      m17, m1, m6
    pmulld          m2, m11            ; p5 * s0
    pmulld          m3, m11
    pmaddwd        m16, m13            ; b5 * 164
    pmaddwd        m17, m13
    vpalignr    m3{k2}, m2, m2, 2
    mova            m2, m20
    pmaxsw          m3, m6
    paddusw         m3, m14
    psraw           m3, 4              ; min(z5, 255) - 256
    vpermt2b        m2, m3, m21        ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m3
    vpermi2b        m3, m18, m19       ; sgr_x_by_x[  0..127]
    vmovdqu8    m3{k3}, m2             ; x5
    pandn           m2, m15, m3
    psrld           m3, 16
    pmulld         m16, m2
    pmulld         m17, m3
    packssdw        m2, m3
    mova [t4+r10*1+416*0+4], m2
    psubd          m16, m15            ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    psubd          m17, m15
    psrld          m16, 12
    psrld          m17, 12
    mova          [t3+r10*2+416*0+  8], xm16
    mova          [t3+r10*2+416*0+ 24], xm17
    vextracti128  [t3+r10*2+416*0+ 40], ym16, 1
    vextracti128  [t3+r10*2+416*0+ 56], ym17, 1
    vextracti32x4 [t3+r10*2+416*0+ 72], m16, 2
    vextracti32x4 [t3+r10*2+416*0+ 88], m17, 2
    vextracti32x4 [t3+r10*2+416*0+104], m16, 3
    vextracti32x4 [t3+r10*2+416*0+120], m17, 3
    add            r10, 64
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu           ym0, [t4+r10*1+416*0+2]
    paddw          ym2, ym0, [t4+r10*1+416*0+0]
    paddw          ym2, [t4+r10*1+416*0+4]
    movu            m1, [t3+r10*2+416*0+4]
    paddd           m3, m1, [t3+r10*2+416*0+0]
    paddd           m3, [t3+r10*2+416*0+8]
    paddw          ym0, ym2
    paddd           m1, m3
    psllw          ym2, 2
    pslld           m3, 2
    paddw          ym0, ym2              ; a5 565
    paddd           m1, m3               ; b5 565
    mova [t4+r10*1+416* 6], ym0
    mova [t3+r10*2+416*12], m1
    mova           ym0, [t4+r10*1+416*2+0]
    paddw          ym0, [t4+r10*1+416*2+4]
    paddw          ym2, ym0, [t4+r10*1+416*2+2]
    mova            m1, [t3+r10*2+416*4+0]
    paddd           m1, [t3+r10*2+416*4+8]
    paddd           m3, m1, [t3+r10*2+416*4+4]
    psllw          ym2, 2                ; a3[-1] 444
    pslld           m3, 2                ; b3[-1] 444
    psubw          ym2, ym0              ; a3[-1] 343
    psubd           m3, m1               ; b3[-1] 343
    mova [t4+r10*1+416* 8], ym2
    mova [t3+r10*2+416*16], m3
    mova           ym0, [t4+r10*1+416*4+0]
    paddw          ym0, [t4+r10*1+416*4+4]
    paddw          ym2, ym0, [t4+r10*1+416*4+2]
    mova            m1, [t3+r10*2+416*8+0]
    paddd           m1, [t3+r10*2+416*8+8]
    paddd           m3, m1, [t3+r10*2+416*8+4]
    psllw          ym2, 2                 ; a3[ 0] 444
    pslld           m3, 2                 ; b3[ 0] 444
    mova [t4+r10*1+416*10], ym2
    mova [t3+r10*2+416*20], m3
    psubw          ym2, ym0               ; a3[ 0] 343
    psubd           m3, m1                ; b3[ 0] 343
    mova [t4+r10*1+416*12], ym2
    mova [t3+r10*2+416*24], m3
    add            r10, 32
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu           ym2, [t4+r10*1+2]
    paddw          ym0, ym2, [t4+r10*1+0]
    paddw          ym0, [t4+r10*1+4]
    paddw          ym2, ym0
    psllw          ym0, 2
    paddw          ym0, ym2              ; a5
    movu            m1, [t3+r10*2+4]
    paddd           m4, m1, [t3+r10*2+0]
    paddd           m4, [t3+r10*2+8]
    paddd           m1, m4
    pslld           m4, 2
    paddd           m4, m1               ; b5
    paddw          ym2, ym0, [t4+r10*1+416* 6]
    mova [t4+r10*1+416* 6], ym0
    paddd           m0, m4, [t3+r10*2+416*12]
    mova [t3+r10*2+416*12], m4
    mova           ym3, [t4+r10*1+416*2+0]
    paddw          ym3, [t4+r10*1+416*2+4]
    paddw          ym5, ym3, [t4+r10*1+416*2+2]
    psllw          ym5, 2                ; a3[ 1] 444
    psubw          ym4, ym5, ym3         ; a3[ 1] 343
    paddw          ym3, ym4, [t4+r10*1+416* 8]
    paddw          ym3, [t4+r10*1+416*10]
    mova [t4+r10*1+416* 8], ym4
    mova [t4+r10*1+416*10], ym5
    mova            m1, [t3+r10*2+416*4+0]
    paddd           m1, [t3+r10*2+416*4+8]
    paddd           m5, m1, [t3+r10*2+416*4+4]
    pslld           m5, 2                ; b3[ 1] 444
    psubd           m4, m5, m1           ; b3[ 1] 343
    paddd           m1, m4, [t3+r10*2+416*16]
    paddd           m1, [t3+r10*2+416*20]
    mova [t3+r10*2+416*16], m4
    mova [t3+r10*2+416*20], m5
    pmovzxwd        m4, [dstq+r10]
    pmovzxwd        m2, ym2              ; a5
    pmovzxwd        m3, ym3              ; a3
    pmaddwd         m2, m4               ; a5 * src
    pmaddwd         m3, m4               ; a3 * src
    vpshldd         m4, m22, 13
    psubd           m0, m2               ; b5 - a5 * src + (1 << 8)
    psubd           m1, m3               ; b3 - a3 * src + (1 << 8)
    psrld           m0, 9
    pslld           m1, 7
    vpblendmb   m0{k2}, m1, m0
    vpdpwssd        m4, m0, m7
    psrad           m4, 7
    pmaxsd          m4, m6
    vpmovusdw     ym16, m4               ; clip
    psrlw         ym16, 6
    mova    [dstq+r10], ym16
    add            r10, 32
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova           ym3, [t4+r10*1+416*4+0]
    paddw          ym3, [t4+r10*1+416*4+4]
    paddw          ym5, ym3, [t4+r10*1+416*4+2]
    psllw          ym5, 2                ; a3[ 1] 444
    psubw          ym4, ym5, ym3         ; a3[ 1] 343
    paddw          ym3, ym4, [t4+r10*1+416*12]
    paddw          ym3, [t4+r10*1+416*10]
    mova [t4+r10*1+416*10], ym5
    mova [t4+r10*1+416*12], ym4
    mova            m0, [t3+r10*2+416*8+0]
    paddd           m0, [t3+r10*2+416*8+8]
    paddd           m5, m0, [t3+r10*2+416*8+4]
    pslld           m5, 2                ; b3[ 1] 444
    psubd           m4, m5, m0           ; b3[ 1] 343
    paddd           m0, m4, [t3+r10*2+416*24]
    paddd           m0, [t3+r10*2+416*20]
    mova [t3+r10*2+416*20], m5
    mova [t3+r10*2+416*24], m4
    pmovzxwd        m4, [dstq+r10]
    pmovzxwd        m2, [t4+r10*1+416* 6]
    pmovzxwd        m3, ym3
    mova            m1, [t3+r10*2+416*12]
    pmaddwd         m2, m4               ; a5 * src
    pmaddwd         m3, m4               ; a3 * src
    vpshldd         m4, m22, 13
    psubd           m1, m2               ; b5 - a5 * src + (1 << 8)
    psubd           m0, m3               ; b3 - a3 * src + (1 << 8)
    pslld           m0, 7
    vpalignr    m0{k2}, m1, m1, 1
    vpdpwssd        m4, m0, m7
    psrad           m4, 7
    pmaxsd          m4, m6
    vpmovusdw     ym16, m4               ; clip
    psrlw         ym16, 6
    mova    [dstq+r10], ym16
    add            r10, 32
    jl .n1_loop
    add           dstq, strideq
    ret

%endif ; ARCH_X86_64
