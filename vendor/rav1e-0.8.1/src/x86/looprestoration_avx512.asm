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

wiener_shufA:  db  1,  2,  7,  6,  3,  4,  9,  8,  5,  6, 11, 10,  7,  8, 13, 12
wiener_shufB:  db  2,  3,  8,  7,  4,  5, 10,  9,  6,  7, 12, 11,  8,  9, 14, 13
wiener_shufC:  db  3,  4,  4,  5,  5,  6,  6,  7,  7,  8,  8,  9,  9, 10, 10, 11
wiener_shufD:  db  4,  5,  5,  6,  6,  7,  7,  8,  8,  9,  9, 10, 10, 11, 11, 12
wiener_perm32: db  1,  9,  3, 11,  5, 13,  7, 15, 33, 41, 35, 43, 37, 45, 39, 47
               db 17, 25, 19, 27, 21, 29, 23, 31, 49, 57, 51, 59, 53, 61, 55, 63
sgr_shuf:      db 128, 1, -1,  2,132,  3, -1,  4,136,  5, -1,  6,140,  7, -1,  8
               db 129, 9, -1, 10,133, 11, -1, 12,137, -1, -1, -1,141, -1,  0,128
sgr_mix_perm:  db  1,  3,  5,  7, 17, 19, 21, 23, 33, 35, 37, 39, 49, 51, 53, 55
r_ext_mask:    times 68 db -1
               times  4 db  0
wiener_x_shuf: db  0,  2, -1,  0
wiener_x_add:  db  0,  1,127,  0

pw_61448:      times 2 dw 61448
pw_164_455:    dw 164, 455
pd_m16380:     dd -16380
pd_m4096:      dd -4096
pd_m25         dd -25
pd_m9:         dd -9
pd_34816:      dd 34816
pd_8421376:    dd 8421376

cextern sgr_x_by_x

SECTION .text

DECLARE_REG_TMP 8, 7, 9, 11, 12, 13, 14 ; ring buffer pointers

INIT_ZMM avx512icl
cglobal wiener_filter7_8bpc, 4, 15, 20, -384*12-16, dst, stride, left, lpf, \
                                                    w, h, edge, flt
    mov           fltq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    vbroadcasti32x4 m6, [wiener_shufA]
    vbroadcasti32x4 m7, [wiener_shufB]
    mov           r10d, 0xfffe
    vbroadcasti32x4 m8, [wiener_shufC]
    vbroadcasti32x4 m9, [wiener_shufD]
    kmovw           k1, r10d
    vpbroadcastd    m0, [wiener_x_shuf]
    vpbroadcastd    m1, [wiener_x_add]
    mov            r10, 0xaaaaaaaaaaaaaaaa
    vpbroadcastd   m11, [fltq+ 0]
    vpbroadcastd   m12, [fltq+ 4]
    kmovq           k2, r10
    vpbroadcastd   m10, [pd_m16380]
    packsswb       m11, m11 ; x0   x1   x0   x1
    vpbroadcastd   m14, [fltq+16]
    pshufb         m12, m0
    vpbroadcastd   m15, [fltq+20]
    paddb          m12, m1  ; x2   x3+1 x2   127
    vpbroadcastd   m13, [pd_8421376]
    psllw          m14, 5   ; y0 y1
    psllw          m15, 5   ; y2 y3
    cmp             wd, 32  ; the minimum lr unit size for chroma in 4:2:0 is 32
    jle .w32                ; pixels, so we need a special case for small widths
    lea             t1, [rsp+wq*2+16]
    add           lpfq, wq
    add           dstq, wq
    neg             wq
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
    movd          xm16, [leftq]
    vmovdqu32  m16{k1}, [lpfq+r10-4]
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    vpbroadcastb  xm16, [lpfq+r10]   ; the masked load ensures that no exception
    vmovdqu32  m16{k1}, [lpfq+r10-4] ; gets raised from accessing invalid memory
    jmp .h_main
.h_top:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu           m16, [lpfq+r10-4]
.h_main:
    movu           m17, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -66
    jl .h_have_right
    push            r0
    lea             r0, [r_ext_mask+65]
    vpbroadcastb    m0, [lpfq-1]
    vpternlogd     m16, m0, [r0+r10+0], 0xe4 ; c ? a : b
    vpternlogd     m17, m0, [r0+r10+8], 0xe4
    pop             r0
.h_have_right:
    pshufb          m4, m16, m6
    mova            m0, m10
    vpdpbusd        m0, m4, m11
    pshufb          m4, m16, m7
    mova            m2, m10
    vpdpbusd        m2, m4, m11
    pshufb          m4, m17, m6
    mova            m1, m10
    vpdpbusd        m1, m4, m11
    pshufb          m4, m17, m7
    mova            m3, m10
    vpdpbusd        m3, m4, m11
    pshufb          m4, m16, m8
    vpdpbusd        m0, m4, m12
    pshufb         m16, m9
    vpdpbusd        m2, m16, m12
    pshufb          m4, m17, m8
    vpdpbusd        m1, m4, m12
    pshufb         m17, m9
    vpdpbusd        m3, m17, m12
    packssdw        m0, m2
    packssdw        m1, m3
    psraw           m0, 3
    psraw           m1, 3
    mova [t1+r10*2+ 0], m0
    mova [t1+r10*2+64], m1
    add            r10, 64
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movd          xm16, [leftq]
    vmovdqu32  m16{k1}, [lpfq+r10-4]
    add          leftq, 4
    jmp .hv_main
.hv_extend_left:
    vpbroadcastb  xm16, [lpfq+r10]
    vmovdqu32  m16{k1}, [lpfq+r10-4]
    jmp .hv_main
.hv_bottom:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu           m16, [lpfq+r10-4]
.hv_main:
    movu           m17, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -66
    jl .hv_have_right
    push            r0
    lea             r0, [r_ext_mask+65]
    vpbroadcastb    m0, [lpfq-1]
    vpternlogd     m16, m0, [r0+r10+0], 0xe4 ; c ? a : b
    vpternlogd     m17, m0, [r0+r10+8], 0xe4
    pop             r0
.hv_have_right:
    pshufb          m4, m16, m6
    mova            m0, m10
    vpdpbusd        m0, m4, m11
    pshufb          m4, m16, m7
    mova            m2, m10
    vpdpbusd        m2, m4, m11
    pshufb          m4, m17, m6
    mova            m1, m10
    vpdpbusd        m1, m4, m11
    pshufb          m4, m17, m7
    mova            m3, m10
    vpdpbusd        m3, m4, m11
    pshufb          m4, m16, m8
    vpdpbusd        m0, m4, m12
    pshufb         m16, m9
    vpdpbusd        m2, m16, m12
    pshufb          m4, m17, m8
    vpdpbusd        m1, m4, m12
    pshufb         m17, m9
    vpdpbusd        m3, m17, m12
    packssdw        m0, m2
    packssdw        m1, m3
    psraw           m0, 3
    psraw           m1, 3
    mova           m16, [t4+r10*2]
    paddw          m16, [t2+r10*2]
    mova            m3, [t3+r10*2]
    mova           m17, [t4+r10*2+64]
    paddw          m17, [t2+r10*2+64]
    mova            m5, [t3+r10*2+64]
    punpcklwd       m4, m16, m3
    mova            m2, m13
    vpdpwssd        m2, m4, m15
    punpcklwd      m18, m17, m5
    mova            m4, m13
    vpdpwssd        m4, m18, m15
    punpckhwd      m16, m3
    mova            m3, m13
    vpdpwssd        m3, m16, m15
    punpckhwd      m17, m5
    mova            m5, m13
    vpdpwssd        m5, m17, m15
    mova           m17, [t5+r10*2]
    paddw          m17, [t1+r10*2]
    paddw          m16, m0, [t6+r10*2]
    mova           m19, [t5+r10*2+64]
    paddw          m19, [t1+r10*2+64]
    paddw          m18, m1, [t6+r10*2+64]
    mova [t0+r10*2+ 0], m0
    mova [t0+r10*2+64], m1
    punpcklwd       m0, m16, m17
    vpdpwssd        m2, m0, m14
    punpcklwd       m1, m18, m19
    vpdpwssd        m4, m1, m14
    punpckhwd      m16, m17
    vpdpwssd        m3, m16, m14
    punpckhwd      m18, m19
    vpdpwssd        m5, m18, m14
    packuswb        m2, m4
    psrlw           m2, 8
    vpackuswb   m2{k2}, m3, m5
    movu    [dstq+r10], m2 ; We don't have a separate 5-tap version so the 7-tap
    add            r10, 64 ; function is used for chroma as well, and in some
    jl .hv_loop            ; esoteric edge cases chroma dst pointers may only
    mov             t6, t5 ; have a 32-byte alignment despite having a width
    mov             t5, t4 ; larger than 32, so use an unaligned store here.
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
    mova            m4, [t4+r10*2+ 0]
    paddw           m4, [t2+r10*2+ 0]
    mova            m1, [t3+r10*2+ 0]
    mova            m5, [t4+r10*2+64]
    paddw           m5, [t2+r10*2+64]
    mova            m3, [t3+r10*2+64]
    punpcklwd       m6, m4, m1
    mova            m0, m13
    vpdpwssd        m0, m6, m15
    punpcklwd       m6, m5, m3
    mova            m2, m13
    vpdpwssd        m2, m6, m15
    punpckhwd       m4, m1
    mova            m1, m13
    vpdpwssd        m1, m4, m15
    punpckhwd       m5, m3
    mova            m3, m13
    vpdpwssd        m3, m5, m15
    mova            m5, [t1+r10*2+ 0]
    paddw           m4, m5, [t6+r10*2+ 0]
    paddw           m5, [t5+r10*2+ 0]
    mova            m7, [t1+r10*2+64]
    paddw           m6, m7, [t6+r10*2+64]
    paddw           m7, [t5+r10*2+64]
    punpcklwd       m8, m4, m5
    vpdpwssd        m0, m8, m14
    punpcklwd       m8, m6, m7
    vpdpwssd        m2, m8, m14
    punpckhwd       m4, m5
    vpdpwssd        m1, m4, m14
    punpckhwd       m6, m7
    vpdpwssd        m3, m6, m14
    packuswb        m0, m2
    psrlw           m0, 8
    vpackuswb   m0{k2}, m1, m3
    movu    [dstq+r10], m0
    add            r10, 64
    jl .v_loop
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    add           dstq, strideq
    ret
.w32:
    lea            r10, [r_ext_mask+73]
    mova          ym18, [wiener_perm32]
    lea             t1, [rsp+16]
    sub            r10, wq
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .w32_no_top
    call .w32_h_top
    add           lpfq, strideq
    mov             t6, t1
    mov             t5, t1
    add             t1, 32*2
    call .w32_h_top
    lea             r9, [lpfq+strideq*4]
    mov           lpfq, dstq
    mov             t4, t1
    add             t1, 32*2
    add             r9, strideq
    mov          [rsp], r9 ; below
    call .w32_h
    mov             t3, t1
    mov             t2, t1
    dec             hd
    jz .w32_v1
    add           lpfq, strideq
    add             t1, 32*2
    call .w32_h
    mov             t2, t1
    dec             hd
    jz .w32_v2
    add           lpfq, strideq
    add             t1, 32*2
    call .w32_h
    dec             hd
    jz .w32_v3
.w32_main:
    lea             t0, [t1+32*2]
.w32_main_loop:
    call .w32_hv
    dec             hd
    jnz .w32_main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .w32_v3
    mov           lpfq, [rsp]
    call .w32_hv_bottom
    add           lpfq, strideq
    call .w32_hv_bottom
.w32_v1:
    call .w32_v
    RET
.w32_no_top:
    lea             r9, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea             r9, [r9+strideq*2]
    mov          [rsp], r9
    call .w32_h
    mov             t6, t1
    mov             t5, t1
    mov             t4, t1
    mov             t3, t1
    mov             t2, t1
    dec             hd
    jz .w32_v1
    add           lpfq, strideq
    add             t1, 32*2
    call .w32_h
    mov             t2, t1
    dec             hd
    jz .w32_v2
    add           lpfq, strideq
    add             t1, 32*2
    call .w32_h
    dec             hd
    jz .w32_v3
    lea             t0, [t1+32*2]
    call .w32_hv
    dec             hd
    jz .w32_v3
    add             t0, 32*8
    call .w32_hv
    dec             hd
    jnz .w32_main
.w32_v3:
    call .w32_v
.w32_v2:
    call .w32_v
    jmp .w32_v1
.w32_h:
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .w32_h_extend_left
    movd          xm16, [leftq]
    vmovdqu32 ym16{k1}, [lpfq-4]
    add          leftq, 4
    jmp .w32_h_main
.w32_h_extend_left:
    vpbroadcastb  xm16, [lpfq]   ; the masked load ensures that no exception
    vmovdqu32 ym16{k1}, [lpfq-4] ; gets raised from accessing invalid memory
    jmp .w32_h_main
.w32_h_top:
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .w32_h_extend_left
    movu          ym16, [lpfq-4]
.w32_h_main:
    vinserti32x8   m16, [lpfq+4], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .w32_h_have_right
    vpbroadcastb    m0, [lpfq+wq-1]
    movu          ym17, [r10-8]
    vinserti32x8   m17, [r10+0], 1
    vpternlogd     m16, m0, m17, 0xe4 ; c ? a : b
.w32_h_have_right:
    pshufb          m2, m16, m6
    mova            m0, m10
    vpdpbusd        m0, m2, m11
    pshufb          m2, m16, m7
    mova            m1, m10
    vpdpbusd        m1, m2, m11
    pshufb          m2, m16, m8
    vpdpbusd        m0, m2, m12
    pshufb         m16, m9
    vpdpbusd        m1, m16, m12
    packssdw        m0, m1
    psraw           m0, 3
    mova          [t1], m0
    ret
.w32_hv:
    add           lpfq, strideq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .w32_hv_extend_left
    movd          xm16, [leftq]
    vmovdqu32 ym16{k1}, [lpfq-4]
    add          leftq, 4
    jmp .w32_hv_main
.w32_hv_extend_left:
    vpbroadcastb  xm16, [lpfq]
    vmovdqu32 ym16{k1}, [lpfq-4]
    jmp .w32_hv_main
.w32_hv_bottom:
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .w32_hv_extend_left
    movu          ym16, [lpfq-4]
.w32_hv_main:
    vinserti32x8   m16, [lpfq+4], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .w32_hv_have_right
    vpbroadcastb    m0, [lpfq+wq-1]
    movu          ym17, [r10-8]
    vinserti32x8   m17, [r10+0], 1
    vpternlogd     m16, m0, m17, 0xe4
.w32_hv_have_right:
    mova            m3, [t4]
    paddw           m3, [t2]
    mova            m2, [t3]
    pshufb          m4, m16, m6
    mova            m0, m10
    vpdpbusd        m0, m4, m11
    pshufb          m4, m16, m7
    mova            m5, m10
    vpdpbusd        m5, m4, m11
    punpcklwd       m4, m3, m2
    mova            m1, m13
    vpdpwssd        m1, m4, m15
    punpckhwd       m3, m2
    mova            m2, m13
    vpdpwssd        m2, m3, m15
    pshufb          m4, m16, m8
    vpdpbusd        m0, m4, m12
    pshufb         m16, m9
    vpdpbusd        m5, m16, m12
    packssdw        m0, m5
    psraw           m0, 3
    mova            m4, [t5]
    paddw           m4, [t1]
    paddw           m3, m0, [t6]
    mova          [t0], m0
    punpcklwd       m0, m3, m4
    vpdpwssd        m1, m0, m14
    punpckhwd       m3, m4
    vpdpwssd        m2, m3, m14
    packuswb        m1, m2
    vpermb         m16, m18, m1
    mova        [dstq], ym16
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, t6
    add           dstq, strideq
    ret
.w32_v:
    mova            m2, [t4]
    paddw           m2, [t2]
    mova            m1, [t3]
    mova            m4, [t1]
    paddw           m3, m4, [t6]
    paddw           m4, [t5]
    punpcklwd       m5, m2, m1
    mova            m0, m13
    vpdpwssd        m0, m5, m15
    punpckhwd       m2, m1
    mova            m1, m13
    vpdpwssd        m1, m2, m15
    punpcklwd       m2, m3, m4
    vpdpwssd        m0, m2, m14
    punpckhwd       m3, m4
    vpdpwssd        m1, m3, m14
    packuswb        m0, m1
    vpermb         m16, m18, m0
    mova        [dstq], ym16
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    add           dstq, strideq
    ret

cglobal sgr_filter_5x5_8bpc, 4, 13, 23, 416*24+16, dst, stride, left, lpf, \
                                                   w, h, edge, params
    mov        paramsq, r6mp
    mov             wd, wm
    mov             hd, hm
    mov          edged, r7m
    vbroadcasti32x4 m5, [sgr_shuf+1]
    add           lpfq, wq
    vbroadcasti32x4 m6, [sgr_shuf+9]
    add           dstq, wq
    vbroadcasti32x4 m7, [sgr_shuf+3]
    lea             t3, [rsp+wq*4+16+416*12]
    vbroadcasti32x4 m8, [sgr_shuf+7]
    pxor            m4, m4
    vpbroadcastd    m9, [pd_m25]
    vpsubd         m11, m4, [paramsq+0] {1to16} ; -s0
    vpbroadcastw   m15, [paramsq+8]             ; w0
    lea             t1, [rsp+wq*2+20]
    vpbroadcastd   m10, [pw_164_455]
    neg             wq
    vpbroadcastd   m12, [pw_61448]              ; (15 << 12) + (1 << 3)
    mov           r10d, 0xfe
    vpbroadcastd   m13, [pd_m4096]
    kmovb           k1, r10d
    vpbroadcastd   m14, [pd_34816]              ; (1 << 11) + (1 << 15)
    mov            r10, 0x3333333333333333
    mova           m18, [sgr_x_by_x+64*0]
    kmovq           k2, r10
    mova           m19, [sgr_x_by_x+64*1]
    lea            r12, [r_ext_mask+75]
    mova           m20, [sgr_x_by_x+64*2]
    psllw          m15, 4
    mova           m21, [sgr_x_by_x+64*3]
    lea            r10, [lpfq+strideq*4]
    mova          ym22, [sgr_shuf]
    add            r10, strideq
    mov          [rsp], r10 ; below
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
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movd          xm17, [leftq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    vpbroadcastb  xm17, [lpfq+wq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    jmp .h_main
.h_top:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu          ym17, [lpfq+r10-2]
.h_main:
    vinserti32x8   m17, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -34
    jl .h_have_right
    vpbroadcastb    m0, [lpfq-1]
    movu          ym16, [r12+r10-8]
    vinserti32x8   m16, [r12+r10+0], 1
    vpternlogd     m17, m0, m16, 0xe4
.h_have_right:
    pshufb          m3, m17, m5
    pmullw          m2, m3, m3
    pshufb          m1, m17, m6
    paddw           m0, m3, m1
    shufps          m3, m1, q2121
    paddw           m0, m3
    punpcklwd      m16, m3, m1
    punpckhwd       m3, m1
    punpcklwd       m1, m2, m4
    vpdpwssd        m1, m16, m16
    punpckhwd       m2, m4
    vpdpwssd        m2, m3, m3
    pshufb         m16, m17, m7
    paddw           m0, m16
    pshufb         m17, m8
    paddw           m0, m17              ; sum
    punpcklwd       m3, m16, m17
    vpdpwssd        m1, m3, m3           ; sumsq
    punpckhwd      m16, m17
    vpdpwssd        m2, m16, m16
    test         edgeb, 16 ; y > 0
    jz .h_loop_end
    paddw           m0, [t1+r10*2+416*0]
    paddd           m1, [t1+r10*2+416*2]
    paddd           m2, [t1+r10*2+416*4]
.h_loop_end:
    mova [t1+r10*2+416*0], m0
    mova [t1+r10*2+416*2], m1
    mova [t1+r10*2+416*4], m2
    add            r10, 32
    jl .h_loop
    ret
.top_fixup:
    lea            r10, [wq-2]
.top_fixup_loop: ; the sums of the first row needs to be doubled
    mova            m0, [t1+r10*2+416*0]
    mova            m1, [t1+r10*2+416*2]
    mova            m2, [t1+r10*2+416*4]
    paddw           m0, m0
    paddd           m1, m1
    paddd           m2, m2
    mova [t2+r10*2+416*0], m0
    mova [t2+r10*2+416*2], m1
    mova [t2+r10*2+416*4], m2
    add            r10, 32
    jl .top_fixup_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movd          xm17, [leftq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    add          leftq, 4
    jmp .hv_main
.hv_extend_left:
    vpbroadcastb  xm17, [lpfq+wq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    jmp .hv_main
.hv_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu          ym17, [lpfq+r10-2]
.hv_main:
    vinserti32x8   m17, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -34
    jl .hv_have_right
    vpbroadcastb    m0, [lpfq-1]
    movu          ym16, [r12+r10-8]
    vinserti32x8   m16, [r12+r10+0], 1
    vpternlogd     m17, m0, m16, 0xe4
.hv_have_right:
    pshufb          m1, m17, m5
    pmullw          m3, m1, m1
    pshufb          m2, m17, m6
    paddw           m0, m1, m2
    shufps          m1, m2, q2121
    paddw           m0, m1
    punpcklwd      m16, m1, m2
    punpckhwd       m1, m2
    punpcklwd       m2, m3, m4
    vpdpwssd        m2, m16, m16
    punpckhwd       m3, m4
    vpdpwssd        m3, m1, m1
    pshufb         m16, m17, m7
    paddw           m0, m16
    pshufb         m17, m8
    paddw           m0, m17              ; h sum
    punpcklwd       m1, m16, m17
    vpdpwssd        m2, m1, m1           ; h sumsq
    punpckhwd      m16, m17
    vpdpwssd        m3, m16, m16
    paddw           m1, m0, [t1+r10*2+416*0]
    paddd          m16, m2, [t1+r10*2+416*2]
    paddd          m17, m3, [t1+r10*2+416*4]
    test            hd, hd
    jz .hv_last_row
.hv_main2:
    paddd          m16, [t2+r10*2+416*2] ; hv sumsq
    paddd          m17, [t2+r10*2+416*4]
    paddw           m1, [t2+r10*2+416*0] ; hv sum
    mova [t0+r10*2+416*2], m2
    mova [t0+r10*2+416*4], m3
    mova [t0+r10*2+416*0], m0
    pmulld         m16, m9               ; -a * 25
    pmulld         m17, m9
    punpcklwd       m0, m1, m4           ; b
    vpdpwssd       m16, m0, m0           ; -p
    punpckhwd       m1, m4
    vpdpwssd       m17, m1, m1
    pmaddwd         m0, m10              ; b * 164
    pmaddwd         m1, m10
    pmulld         m16, m11              ; p * s
    pmulld         m17, m11
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m20
    paddusw        m17, m12
    psraw          m17, 4                ; min(z, 255) - 256
    vpermt2b       m16, m17, m21         ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m18, m19         ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16              ; x
    pandn          m16, m13, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    paddd           m0, m14              ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m14
    vpternlogd     m16, m0, m13, 0xd8    ; a | (b << 12)
    vpternlogd     m17, m1, m13, 0xd8
    mova          [t3+r10*4+  8], m16    ; The neighbor calculations requires
    mova          [t3+r10*4+ 24], xm17   ; 13 bits for a and 21 bits for b.
    vextracti32x4 [t3+r10*4+ 56], m17, 2 ; Packing them allows for 12+20, but
    mova          [t3+r10*4+ 72], m17    ; that gets us most of the way.
    vextracti128  [t3+r10*4+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+104], m16, 3
    add            r10, 32
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    ret
.hv_last_row: ; esoteric edge case for odd heights
    mova [t1+r10*2+416*0], m1
    paddw              m1, m0
    mova [t1+r10*2+416*2], m16
    paddd             m16, m2
    mova [t1+r10*2+416*4], m17
    paddd             m17, m3
    jmp .hv_main2
.v: ; vertical boxsum + ab
    lea            r10, [wq-2]
.v_loop:
    mova            m2, [t1+r10*2+416*2]
    paddd          m16, m2, [t2+r10*2+416*2]
    mova            m3, [t1+r10*2+416*4]
    paddd          m17, m3, [t2+r10*2+416*4]
    paddd           m2, m2
    paddd           m3, m3
    paddd          m16, m2               ; hv sumsq
    paddd          m17, m3
    pmulld         m16, m9               ; -a * 25
    pmulld         m17, m9
    mova            m0, [t1+r10*2+416*0]
    paddw           m1, m0, [t2+r10*2+416*0]
    paddw           m0, m0
    paddw           m1, m0               ; hv sum
    punpcklwd       m0, m1, m4           ; b
    vpdpwssd       m16, m0, m0           ; -p
    punpckhwd       m1, m4
    vpdpwssd       m17, m1, m1
    pmaddwd         m0, m10              ; b * 164
    pmaddwd         m1, m10
    pmulld         m16, m11              ; p * s
    pmulld         m17, m11
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m20
    paddusw        m17, m12
    psraw          m17, 4                ; min(z, 255) - 256
    vpermt2b       m16, m17, m21         ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m18, m19         ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16              ; x
    pandn          m16, m13, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    paddd           m0, m14              ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m14
    vpternlogd     m16, m0, m13, 0xd8    ; a | (b << 12)
    vpternlogd     m17, m1, m13, 0xd8
    mova          [t3+r10*4+  8], m16
    mova          [t3+r10*4+ 24], xm17
    vextracti32x4 [t3+r10*4+ 56], m17, 2
    mova          [t3+r10*4+ 72], m17
    vextracti128  [t3+r10*4+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+104], m16, 3
    add            r10, 32
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu            m0, [t3+r10*4+ 4]
    movu            m1, [t3+r10*4+68]
    paddd           m2, m0, [t3+r10*4+ 0]
    paddd           m3, m1, [t3+r10*4+64]
    paddd           m2, [t3+r10*4+ 8]
    paddd           m3, [t3+r10*4+72]
    paddd           m0, m2
    pslld           m2, 2
    paddd           m1, m3
    pslld           m3, 2
    paddd           m2, m0                ; ab 565
    paddd           m3, m1
    pandn           m0, m13, m2           ; a
    psrld           m2, 12                ; b
    pandn           m1, m13, m3
    psrld           m3, 12
    mova [t3+r10*4+416*4+ 0], m0
    mova [t3+r10*4+416*8+ 0], m2
    mova [t3+r10*4+416*4+64], m1
    mova [t3+r10*4+416*8+64], m3
    add            r10, 32
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu           m16, [t3+r10*4+ 4]
    movu           m17, [t3+r10*4+68]
    paddd           m0, m16, [t3+r10*4+ 0]
    paddd           m1, m17, [t3+r10*4+64]
    paddd           m0, [t3+r10*4+ 8]
    paddd           m1, [t3+r10*4+72]
    paddd          m16, m0
    pslld           m0, 2
    paddd          m17, m1
    pslld           m1, 2
    paddd           m0, m16
    paddd           m1, m17
    pandn          m16, m13, m0
    psrld           m0, 12
    pandn          m17, m13, m1
    psrld           m1, 12
    paddd           m2, m16, [t3+r10*4+416*4+ 0] ; a
    paddd           m3, m17, [t3+r10*4+416*4+64]
    mova [t3+r10*4+416*4+ 0], m16
    mova [t3+r10*4+416*4+64], m17
    paddd          m16, m0, [t3+r10*4+416*8+ 0] ; b + (1 << 8)
    paddd          m17, m1, [t3+r10*4+416*8+64]
    mova [t3+r10*4+416*8+ 0], m0
    mova [t3+r10*4+416*8+64], m1
    pmovzxbd        m0, [dstq+r10+ 0]
    pmovzxbd        m1, [dstq+r10+16]
    pmaddwd         m2, m0                      ; a * src
    pmaddwd         m3, m1
    packssdw        m0, m1
    psubd          m16, m2                      ; b - a * src + (1 << 8)
    psubd          m17, m3
    psrad          m16, 9
    psrad          m17, 9
    packssdw       m16, m17
    pmulhrsw       m16, m15
    paddw          m16, m0
    packuswb       m16, m16
    vpermd         m16, m22, m16
    mova    [dstq+r10], ym16
    add            r10, 32
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    pmovzxbd        m0, [dstq+r10+ 0]
    pmovzxbd        m1, [dstq+r10+16]
    pmaddwd         m2, m0, [t3+r10*4+416*4+ 0] ; a * src
    pmaddwd         m3, m1, [t3+r10*4+416*4+64]
    mova           m16, [t3+r10*4+416*8+ 0]     ; b + (1 << 7)
    mova           m17, [t3+r10*4+416*8+64]
    packssdw        m0, m1
    psubd          m16, m2                      ; b - a * src + (1 << 7)
    psubd          m17, m3
    psrad          m16, 8
    psrad          m17, 8
    packssdw       m16, m17
    pmulhrsw       m16, m15
    paddw          m16, m0
    packuswb       m16, m16
    vpermd         m16, m22, m16
    mova    [dstq+r10], ym16
    add            r10, 32
    jl .n1_loop
    add           dstq, strideq
    ret

cglobal sgr_filter_3x3_8bpc, 4, 15, 22, -416*28-16, dst, stride, left, lpf, \
                                                    w, h, edge, params
    mov        paramsq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    vbroadcasti32x4 m5, [sgr_shuf+3]
    add           lpfq, wq
    vbroadcasti32x4 m6, [sgr_shuf+5]
    add           dstq, wq
    vbroadcasti32x4 m7, [sgr_shuf+7]
    pxor            m4, m4
    vpbroadcastd    m8, [pd_m9]
    vpsubd         m11, m4, [paramsq+4] {1to16} ; -s1
    vpbroadcastw   m15, [paramsq+10]            ; w1
    lea             t1, [rsp+wq*2+20]
    vpbroadcastd   m10, [pw_164_455]
    lea             t3, [rsp+wq*4+16+416*12]
    vpbroadcastd   m12, [pw_61448]              ; (15 << 12) + (1 << 3)
    neg             wq
    vpbroadcastd   m13, [pd_m4096]
    mov           r10d, 0xfe
    vpbroadcastd   m14, [pd_34816]              ; (1 << 11) + (1 << 15)
    kmovb           k1, r10d
    mova           m18, [sgr_x_by_x+64*0]
    mov            r10, 0x3333333333333333
    mova           m19, [sgr_x_by_x+64*1]
    kmovq           k2, r10
    mova           m20, [sgr_x_by_x+64*2]
    psllw          m15, 4
    mova           m21, [sgr_x_by_x+64*3]
    lea            r14, [r_ext_mask+75]
    mova           ym9, [sgr_shuf]
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    add             t1, 416*6
    call .h_top
    lea             t4, [lpfq+strideq*4]
    mov           lpfq, dstq
    add             t4, strideq
    mov          [rsp], t4 ; below
    mov             t0, t2
    call .hv
.main:
    mov             t5, t3
    add             t3, 416*4
    dec             hd
    jz .height1
    add           lpfq, strideq
    call .hv
    call .prep_n
    dec             hd
    jz .extend_bottom
.main_loop:
    add           lpfq, strideq
    call .hv
    call .n
    dec             hd
    jnz .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .extend_bottom
    mov           lpfq, [rsp]
    call .hv_bottom
    call .n
    add           lpfq, strideq
    call .hv_bottom
.end:
    call .n
    RET
.height1:
    call .v
    call .prep_n
    mov             t2, t1
    call .v
    jmp .end
.extend_bottom:
    call .v
    call .n
    mov             t2, t1
    call .v
    jmp .end
.no_top:
    lea             t4, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea             t4, [t4+strideq*2]
    mov          [rsp], t4
    call .h
    lea             t0, [t1+416*6]
    mov             t2, t1
    call .v
    jmp .main
.h: ; horizontal boxsum
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movd          xm17, [leftq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    vpbroadcastb  xm17, [lpfq+wq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    jmp .h_main
.h_top:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu          ym17, [lpfq+r10-2]
.h_main:
    vinserti32x8   m17, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -33
    jl .h_have_right
    vpbroadcastb    m0, [lpfq-1]
    movu          ym16, [r14+r10-8]
    vinserti32x8   m16, [r14+r10+0], 1
    vpternlogd     m17, m0, m16, 0xe4
.h_have_right:
    pshufb          m0, m17, m5
    pmullw          m2, m0, m0
    pshufb         m16, m17, m6
    paddw           m0, m16
    pshufb         m17, m7
    paddw           m0, m17    ; sum
    punpcklwd       m3, m16, m17
    punpcklwd       m1, m2, m4
    vpdpwssd        m1, m3, m3 ; sumsq
    punpckhwd      m16, m17
    punpckhwd       m2, m4
    vpdpwssd        m2, m16, m16
    mova [t1+r10*2+416*0], m0
    mova [t1+r10*2+416*2], m1
    mova [t1+r10*2+416*4], m2
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movd          xm17, [leftq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    add          leftq, 4
    jmp .hv_main
.hv_extend_left:
    vpbroadcastb  xm17, [lpfq+wq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    jmp .hv_main
.hv_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu          ym17, [lpfq+r10-2]
.hv_main:
    vinserti32x8   m17, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -33
    jl .hv_have_right
    vpbroadcastb    m0, [lpfq-1]
    movu          ym16, [r14+r10-8]
    vinserti32x8   m16, [r14+r10+0], 1
    vpternlogd     m17, m0, m16, 0xe4
.hv_have_right:
    pshufb          m0, m17, m5
    pmullw          m3, m0, m0
    pshufb          m1, m17, m6
    paddw           m0, m1
    pshufb         m17, m7
    paddw           m0, m17              ; h sum
    punpcklwd      m16, m17, m1
    punpcklwd       m2, m3, m4
    vpdpwssd        m2, m16, m16         ; h sumsq
    punpckhwd      m17, m1
    punpckhwd       m3, m4
    vpdpwssd        m3, m17, m17
    paddw           m1, m0, [t2+r10*2+416*0]
    paddw           m1, [t1+r10*2+416*0] ; hv sum
    paddd          m16, m2, [t2+r10*2+416*2]
    paddd          m17, m3, [t2+r10*2+416*4]
    paddd          m16, [t1+r10*2+416*2] ; hv sumsq
    paddd          m17, [t1+r10*2+416*4]
    mova [t0+r10*2+416*0], m0
    mova [t0+r10*2+416*2], m2
    mova [t0+r10*2+416*4], m3
    pmulld         m16, m8               ; -a * 9
    pmulld         m17, m8
    punpcklwd       m0, m4, m1           ; b
    vpdpwssd       m16, m0, m0           ; -p
    punpckhwd       m1, m4, m1
    vpdpwssd       m17, m1, m1
    pmaddwd         m0, m10              ; b * 455
    pmaddwd         m1, m10
    pmulld         m16, m11              ; p * s
    pmulld         m17, m11
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m20
    paddusw        m17, m12
    psraw          m17, 4                ; min(z, 255) - 256
    vpermt2b       m16, m17, m21         ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m18, m19         ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16              ; x
    pandn          m16, m13, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    paddd           m0, m14              ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m14
    vpternlogd     m16, m0, m13, 0xd8    ; a | (b << 12)
    vpternlogd     m17, m1, m13, 0xd8
    mova          [t3+r10*4+  8], m16
    mova          [t3+r10*4+ 24], xm17
    vextracti32x4 [t3+r10*4+ 56], m17, 2
    mova          [t3+r10*4+ 72], m17
    vextracti128  [t3+r10*4+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+104], m16, 3
    add            r10, 32
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    ret
.v: ; vertical boxsum + ab
    lea            r10, [wq-2]
.v_loop:
    mova           m16, [t1+r10*2+416*2]
    mova           m17, [t1+r10*2+416*4]
    paddd          m16, m16
    paddd          m17, m17
    paddd          m16, [t2+r10*2+416*2] ; hv sumsq
    paddd          m17, [t2+r10*2+416*4]
    pmulld         m16, m8               ; -a * 9
    pmulld         m17, m8
    mova            m1, [t1+r10*2+416*0]
    paddw           m1, m1
    paddw           m1, [t2+r10*2+416*0] ; hv sum
    punpcklwd       m0, m4, m1           ; b
    vpdpwssd       m16, m0, m0           ; -p
    punpckhwd       m1, m4, m1
    vpdpwssd       m17, m1, m1
    pmaddwd         m0, m10              ; b * 455
    pmaddwd         m1, m10
    pmulld         m16, m11              ; p * s
    pmulld         m17, m11
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m20
    paddusw        m17, m12
    psraw          m17, 4                ; min(z, 255) - 256
    vpermt2b       m16, m17, m21         ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m18, m19         ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16              ; x
    pandn          m16, m13, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    paddd           m0, m14              ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m14
    vpternlogd     m16, m0, m13, 0xd8    ; a | (b << 12)
    vpternlogd     m17, m1, m13, 0xd8
    mova          [t3+r10*4+  8], m16
    mova          [t3+r10*4+ 24], xm17
    vextracti32x4 [t3+r10*4+ 56], m17, 2
    mova          [t3+r10*4+ 72], m17
    vextracti128  [t3+r10*4+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+104], m16, 3
    add            r10, 32
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
    mov             t4, t3
    add             t3, 416*4
.prep_n_loop:
    mova            m2, [t5+r10*4+0]
    mova            m3, [t4+r10*4+0]
    paddd           m2, [t5+r10*4+8]
    paddd           m3, [t4+r10*4+8]
    paddd           m0, m2, [t5+r10*4+4]
    paddd           m1, m3, [t4+r10*4+4]
    pslld           m0, 2
    paddd           m1, m1                ; ab[ 0] 222
    psubd           m0, m2                ; ab[-1] 343
    mova [t3+r10*4+416*4], m1
    paddd           m1, m1
    mova    [t5+r10*4], m0
    psubd           m1, m3                ; ab[ 0] 343
    mova    [t4+r10*4], m1
    add            r10, 16
    jl .prep_n_loop
    ret
; a+b are packed together in a single dword, but we can't do the
; full neighbor calculations before splitting them since we don't
; have sufficient precision. The solution is to do the calculations
; in two equal halves and split a and b before doing the final sum.
ALIGN function_align
.n: ; neighbor + output
    mov            r10, wq
.n_loop:
    mova           m16, [t3+r10*4+ 0]
    paddd          m16, [t3+r10*4+ 8]
    paddd          m17, m16, [t3+r10*4+ 4]
    paddd          m17, m17               ; ab[+1] 222
    mova            m2, [t3+r10*4+416*4+ 0]
    paddd           m0, m2, [t5+r10*4+ 0] ; ab[ 0] 222 + ab[-1] 343
    mova            m3, [t3+r10*4+416*4+64]
    paddd           m1, m3, [t5+r10*4+64]
    mova [t3+r10*4+416*4+ 0], m17
    paddd          m17, m17
    psubd          m17, m16               ; ab[+1] 343
    mova [t5+r10*4+ 0], m17
    paddd           m2, m17               ; ab[ 0] 222 + ab[+1] 343
    mova           m16, [t3+r10*4+64]
    paddd          m16, [t3+r10*4+72]
    paddd          m17, m16, [t3+r10*4+68]
    paddd          m17, m17
    mova [t3+r10*4+416*4+64], m17
    paddd          m17, m17
    psubd          m17, m16
    mova [t5+r10*4+64], m17
    pandn          m16, m13, m0
    psrld           m0, 12
    paddd           m3, m17
    pandn          m17, m13, m2
    psrld           m2, 12
    paddd          m16, m17               ; a
    pandn          m17, m13, m1
    psrld           m1, 12
    paddd           m0, m2                ; b + (1 << 8)
    pandn           m2, m13, m3
    psrld           m3, 12
    paddd          m17, m2
    pmovzxbd        m2, [dstq+r10+ 0]
    paddd           m1, m3
    pmovzxbd        m3, [dstq+r10+16]
    pmaddwd        m16, m2                ; a * src
    pmaddwd        m17, m3
    packssdw        m2, m3
    psubd           m0, m16               ; b - a * src + (1 << 8)
    psubd           m1, m17
    psrad           m0, 9
    psrad           m1, 9
    packssdw        m0, m1
    pmulhrsw        m0, m15
    paddw           m0, m2
    packuswb        m0, m0
    vpermd         m16, m9, m0
    mova    [dstq+r10], ym16
    add            r10, 32
    jl .n_loop
    mov            r10, t5
    mov             t5, t4
    mov             t4, r10
    add           dstq, strideq
    ret

cglobal sgr_filter_mix_8bpc, 4, 13, 28, 416*56+8, dst, stride, left, lpf, \
                                                  w, h, edge, params
    mov        paramsq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    vbroadcasti128  m5, [sgr_shuf+1]
    add           lpfq, wq
    vbroadcasti128  m6, [sgr_shuf+9]
    add           dstq, wq
    vbroadcasti128  m7, [sgr_shuf+3]
    lea             t3, [rsp+wq*4+416*24+8]
    vbroadcasti128  m8, [sgr_shuf+7]
    pxor            m4, m4
    vpbroadcastd    m9, [pd_m9]
    vpsubd         m11, m4, [paramsq+0] {1to16} ; -s0
    vpbroadcastd   m14, [pw_61448]
    vpsubd         m12, m4, [paramsq+4] {1to16} ; -s1
    vpbroadcastd   m26, [paramsq+8]             ; w0 w1
    lea             t1, [rsp+wq*2+12]
    vpbroadcastd   m10, [pd_m25]
    neg             wq
    vpbroadcastd   m13, [pw_164_455]
    mov           r10d, 0xfe
    vpbroadcastd   m15, [pd_34816]
    kmovb           k1, r10d
    mova           m20, [sgr_x_by_x+64*0]
    mov            r10, 0x3333333333333333
    mova           m21, [sgr_x_by_x+64*1]
    kmovq           k2, r10
    mova           m22, [sgr_x_by_x+64*2]
    lea            r12, [r_ext_mask+75]
    mova           m23, [sgr_x_by_x+64*3]
    vpbroadcastd   m24, [pd_m4096]
    vpbroadcastd   m25, [sgr_shuf+28]           ; 0x8000____
    psllw          m26, 5
    mova          xm27, [sgr_mix_perm]
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_avx512icl).top_fixup
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
    lea             t2, [t1+416*12]
    lea            r10, [wq-2]
.top_fixup_loop:
    mova            m0, [t1+r10*2+416* 0]
    mova            m1, [t1+r10*2+416* 2]
    mova            m2, [t1+r10*2+416* 4]
    paddw           m0, m0
    mova            m3, [t1+r10*2+416* 6]
    paddd           m1, m1
    mova           m16, [t1+r10*2+416* 8]
    paddd           m2, m2
    mova           m17, [t1+r10*2+416*10]
    mova [t2+r10*2+416* 0], m0
    mova [t2+r10*2+416* 2], m1
    mova [t2+r10*2+416* 4], m2
    mova [t2+r10*2+416* 6], m3
    mova [t2+r10*2+416* 8], m16
    mova [t2+r10*2+416*10], m17
    add            r10, 32
    jl .top_fixup_loop
    call .v0
    jmp .main
.h: ; horizontal boxsums
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movd          xm17, [leftq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    vpbroadcastb  xm17, [lpfq+wq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    jmp .h_main
.h_top:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu          ym17, [lpfq+r10-2]
.h_main:
    vinserti32x8   m17, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -34
    jl .h_have_right
    vpbroadcastb    m0, [lpfq-1]
    movu          ym16, [r12+r10-8]
    vinserti32x8   m16, [r12+r10+0], 1
    vpternlogd     m17, m0, m16, 0xe4
.h_have_right:
    pshufb          m3, m17, m5
    pshufb         m18, m17, m6
    shufps          m0, m3, m18, q2121
    pmullw          m2, m0, m0
    pshufb         m19, m17, m7
    paddw           m0, m19
    pshufb         m17, m8
    paddw           m0, m17           ; sum3
    punpcklwd      m16, m19, m17
    punpcklwd       m1, m2, m4
    vpdpwssd        m1, m16, m16      ; sumsq3
    punpckhwd      m19, m17
    punpckhwd       m2, m4
    vpdpwssd        m2, m19, m19
    mova [t1+r10*2+416* 6], m0
    mova [t1+r10*2+416* 8], m1
    mova [t1+r10*2+416*10], m2
    punpcklwd      m19, m3, m18
    paddw           m0, m3
    vpdpwssd        m1, m19, m19      ; sumsq5
    punpckhwd       m3, m18
    paddw           m0, m18           ; sum5
    vpdpwssd        m2, m3, m3
    mova [t1+r10*2+416* 0], m0
    mova [t1+r10*2+416* 2], m1
    mova [t1+r10*2+416* 4], m2
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsums + vertical boxsum3 + ab3 (even rows)
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movd          xm17, [leftq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    add          leftq, 4
    jmp .hv0_main
.hv0_extend_left:
    vpbroadcastb  xm17, [lpfq+wq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    jmp .hv0_main
.hv0_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
.hv0_loop:
    movu          ym17, [lpfq+r10-2]
.hv0_main:
    vinserti32x8   m17, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp           r10d, -34
    jl .hv0_have_right
    vpbroadcastb    m0, [lpfq-1]
    movu          ym16, [r12+r10-8]
    vinserti32x8   m16, [r12+r10+0], 1
    vpternlogd     m17, m0, m16, 0xe4
.hv0_have_right:
    pshufb         m18, m17, m5
    pshufb         m19, m17, m6
    shufps          m1, m18, m19, q2121
    pmullw          m3, m1, m1
    pshufb          m0, m17, m7
    paddw           m1, m0
    pshufb         m17, m8
    paddw           m1, m17           ; sum3
    punpcklwd      m16, m0, m17
    punpcklwd       m2, m3, m4
    vpdpwssd        m2, m16, m16      ; sumsq3
    punpckhwd       m0, m17
    punpckhwd       m3, m4
    vpdpwssd        m3, m0, m0
    paddw           m0, m1, [t1+r10*2+416* 6]
    paddd          m16, m2, [t1+r10*2+416* 8]
    paddd          m17, m3, [t1+r10*2+416*10]
    mova [t1+r10*2+416* 6], m1
    mova [t1+r10*2+416* 8], m2
    mova [t1+r10*2+416*10], m3
    paddw           m1, m18
    paddw           m1, m19           ; sum5
    mova [t3+r10*4+416*8+ 8], m1
    paddw           m1, [t1+r10*2+416* 0]
    mova [t1+r10*2+416* 0], m1
    punpcklwd       m1, m18, m19
    vpdpwssd        m2, m1, m1        ; sumsq5
    punpckhwd      m18, m19
    vpdpwssd        m3, m18, m18
    mova [t3+r10*4+416*0+ 8], m2      ; we need a clean copy of the last row
    mova [t3+r10*4+416*0+72], m3      ; in case height is odd
    paddd           m2, [t1+r10*2+416* 2]
    paddd           m3, [t1+r10*2+416* 4]
    mova [t1+r10*2+416* 2], m2
    mova [t1+r10*2+416* 4], m3
    paddw           m1, m0, [t2+r10*2+416* 6]
    paddd           m2, m16, [t2+r10*2+416* 8]
    paddd           m3, m17, [t2+r10*2+416*10]
    mova [t2+r10*2+416* 6], m0
    mova [t2+r10*2+416* 8], m16
    mova [t2+r10*2+416*10], m17
    pmulld         m16, m2, m9        ; -a3 * 9
    pmulld         m17, m3, m9
    punpcklwd       m0, m4, m1        ; b3
    vpdpwssd       m16, m0, m0        ; -p3
    punpckhwd       m1, m4, m1
    vpdpwssd       m17, m1, m1
    pmulld         m16, m12           ; p3 * s1
    pmulld         m17, m12
    pmaddwd         m0, m13           ; b3 * 455
    pmaddwd         m1, m13
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m22
    paddusw        m17, m14
    psraw          m17, 4             ; min(z3, 255) - 256
    vpermt2b       m16, m17, m23      ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m20, m21      ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16           ; x3
    pandn          m16, m24, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    paddd           m0, m15           ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m15
    vpternlogd     m16, m0, m24, 0xd8 ; a3 | (b3 << 12)
    vpternlogd     m17, m1, m24, 0xd8
    mova          [t3+r10*4+416*4+  8], m16
    mova          [t3+r10*4+416*4+ 24], xm17
    vextracti32x4 [t3+r10*4+416*4+ 56], m17, 2
    mova          [t3+r10*4+416*4+ 72], m17
    vextracti128  [t3+r10*4+416*4+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+416*4+104], m16, 3
    add            r10, 32
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movd          xm17, [leftq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    add          leftq, 4
    jmp .hv1_main
.hv1_extend_left:
    vpbroadcastb  xm17, [lpfq+wq]
    vmovdqu32 ym17{k1}, [lpfq+wq-4]
    jmp .hv1_main
.hv1_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
.hv1_loop:
    movu          ym17, [lpfq+r10-2]
.hv1_main:
    vinserti32x8   m17, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp           r10d, -34
    jl .hv1_have_right
    vpbroadcastb    m0, [lpfq-1]
    movu          ym16, [r12+r10-8]
    vinserti32x8   m16, [r12+r10+0], 1
    vpternlogd    m17, m0, m16, 0xe4
.hv1_have_right:
    pshufb          m3, m17, m5
    pshufb         m19, m17, m6
    shufps          m2, m3, m19, q2121
    pmullw          m1, m2, m2
    pshufb         m18, m17, m7
    paddw           m2, m18
    pshufb         m17, m8
    paddw           m2, m17           ; sum3
    punpcklwd      m16, m17, m18
    punpcklwd       m0, m1, m4
    vpdpwssd        m0, m16, m16      ; sumsq3
    punpckhwd      m17, m18
    punpckhwd       m1, m4
    vpdpwssd        m1, m17, m17
    paddd          m16, m0, [t2+r10*2+416* 8]
    paddd          m17, m1, [t2+r10*2+416*10]
    mova [t2+r10*2+416* 8], m0
    mova [t2+r10*2+416*10], m1
    punpcklwd      m18, m3, m19
    vpdpwssd        m0, m18, m18      ; sumsq5
    punpckhwd      m18, m3, m19
    vpdpwssd        m1, m18, m18
    paddw           m3, m19
    pmulld         m16, m9            ; -a3 * 9
    pmulld         m17, m9
    paddd          m18, m0, [t2+r10*2+416*2]
    paddd          m19, m1, [t2+r10*2+416*4]
    paddd          m18, [t1+r10*2+416*2]
    paddd          m19, [t1+r10*2+416*4]
    mova [t2+r10*2+416*2], m0
    mova [t2+r10*2+416*4], m1
    pmulld         m18, m10           ; -a5 * 25
    pmulld         m19, m10
    paddw           m1, m2, [t2+r10*2+416* 6]
    mova [t2+r10*2+416* 6], m2
    paddw           m2, m3            ; sum5
    paddw           m3, m2, [t2+r10*2+416*0]
    paddw           m3, [t1+r10*2+416*0]
    mova [t2+r10*2+416*0], m2
    punpcklwd       m0, m4, m1        ; b3
    vpdpwssd       m16, m0, m0        ; -p3
    punpckhwd       m1, m4, m1
    vpdpwssd       m17, m1, m1
    punpcklwd       m2, m3, m4        ; b5
    vpdpwssd       m18, m2, m2        ; -p5
    punpckhwd       m3, m4
    vpdpwssd       m19, m3, m3
    pmulld         m16, m12           ; p3 * s1
    pmulld         m17, m12
    pmulld         m18, m11           ; p5 * s0
    pmulld         m19, m11
    pmaddwd         m0, m13           ; b3 * 455
    pmaddwd         m1, m13
    pmaddwd         m2, m13           ; b5 * 164
    pmaddwd         m3, m13
    vpalignr   m17{k2}, m16, m16, 2
    vpalignr   m19{k2}, m18, m18, 2
    paddusw        m17, m14
    mova           m16, m22
    psraw          m17, 4             ; min(z3, 255) - 256
    vpermt2b       m16, m17, m23      ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m20, m21      ; sgr_x_by_x[  0..127]
    paddusw        m19, m14
    mova           m18, m22
    psraw          m19, 4             ; min(z5, 255) - 256
    vpermt2b       m18, m19, m23      ; sgr_x_by_x[128..255]
    vpmovb2m        k4, m19
    vpermi2b       m19, m20, m21      ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16           ; x3
    vmovdqu8   m19{k4}, m18           ; x5
    pandn          m16, m24, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    pandn          m18, m24, m19
    psrld          m19, 16
    pmulld          m2, m18
    pmulld          m3, m19
    paddd           m0, m15           ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m15
    vpternlogd     m16, m0, m24, 0xd8 ; a3 | (b3 << 12)
    vpternlogd     m17, m1, m24, 0xd8
    mova          [t3+r10*4+416*8+  8], m16
    mova          [t3+r10*4+416*8+ 24], xm17
    vextracti32x4 [t3+r10*4+416*8+ 56], m17, 2
    paddd           m2, m15           ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m3, m15
    mova          [t3+r10*4+416*8+ 72], m17
    vextracti128  [t3+r10*4+416*8+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+416*8+104], m16, 3
    vpternlogd     m18, m2, m24, 0xd8 ; a5 | (b5 << 12)
    vpternlogd     m19, m3, m24, 0xd8
    mova          [t3+r10*4+416*0+  8], m18
    mova          [t3+r10*4+416*0+ 24], xm19
    vextracti32x4 [t3+r10*4+416*0+ 56], m19, 2
    mova          [t3+r10*4+416*0+ 72], m19
    vextracti128  [t3+r10*4+416*0+ 72], ym18, 1
    vextracti32x4 [t3+r10*4+416*0+104], m18, 3
    add            r10, 32
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab3 (even rows)
    lea            r10, [wq-2]
.v0_loop:
    mova            m2, [t1+r10*2+416* 8]
    mova            m3, [t1+r10*2+416*10]
    paddd           m2, m2
    paddd           m3, m3
    paddd          m16, m2, [t2+r10*2+416* 8]
    paddd          m17, m3, [t2+r10*2+416*10]
    mova            m0, [t1+r10*2+416* 6]
    paddw           m0, m0
    paddw           m1, m0, [t2+r10*2+416* 6]
    pmulld         m16, m9            ; -a3 * 9
    pmulld         m17, m9
    mova [t2+r10*2+416* 6], m0
    mova [t2+r10*2+416* 8], m2
    mova [t2+r10*2+416*10], m3
    mova            m2, [t1+r10*2+416*0]
    mova            m3, [t1+r10*2+416*2]
    mova           m18, [t1+r10*2+416*4]
    punpcklwd       m0, m4, m1        ; b3
    vpdpwssd       m16, m0, m0        ; -p3
    punpckhwd       m1, m4, m1
    vpdpwssd       m17, m1, m1
    pmulld         m16, m12           ; p3 * s1
    pmulld         m17, m12
    pmaddwd         m0, m13           ; b3 * 455
    pmaddwd         m1, m13
    mova [t3+r10*4+416*8+ 8], m2
    mova [t3+r10*4+416*0+ 8], m3
    mova [t3+r10*4+416*0+72], m18
    vpalignr   m17{k2}, m16, m16, 2
    mova           m16, m22
    paddusw        m17, m14
    psraw          m17, 4             ; min(z3, 255) - 256
    vpermt2b       m16, m17, m23      ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m20, m21      ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16           ; x3
    pandn          m16, m24, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    paddw           m2, m2            ; cc5
    paddd           m3, m3
    paddd          m18, m18
    mova [t1+r10*2+416*0], m2
    mova [t1+r10*2+416*2], m3
    mova [t1+r10*2+416*4], m18
    paddd           m0, m15           ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m15
    vpternlogd     m16, m0, m24, 0xd8 ; a3 | (b3 << 12)
    vpternlogd     m17, m1, m24, 0xd8
    mova          [t3+r10*4+416*4+  8], m16
    mova          [t3+r10*4+416*4+ 24], xm17
    vextracti32x4 [t3+r10*4+416*4+ 56], m17, 2
    mova          [t3+r10*4+416*4+ 72], m17
    vextracti128  [t3+r10*4+416*4+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+416*4+104], m16, 3
    add            r10, 32
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
    lea            r10, [wq-2]
.v1_loop:
    mova            m0, [t1+r10*2+416* 8]
    paddd          m16, m0, [t2+r10*2+416* 8]
    mova            m1, [t1+r10*2+416*10]
    paddd          m17, m1, [t2+r10*2+416*10]
    mova            m2, [t3+r10*4+416*0+ 8]
    paddd          m18, m2, [t2+r10*2+416* 2]
    mova            m3, [t3+r10*4+416*0+72]
    paddd          m19, m3, [t2+r10*2+416* 4]
    paddd          m18, [t1+r10*2+416* 2]
    paddd          m19, [t1+r10*2+416* 4]
    mova [t2+r10*2+416* 8], m0
    mova [t2+r10*2+416*10], m1
    mova [t2+r10*2+416* 2], m2
    mova [t2+r10*2+416* 4], m3
    pmulld         m16, m9            ; -a3 * 9
    pmulld         m17, m9
    pmulld         m18, m10           ; -a5 * 25
    pmulld         m19, m10
    mova            m0, [t1+r10*2+416* 6]
    paddw           m1, m0, [t2+r10*2+416* 6]
    mova            m2, [t3+r10*4+416*8+ 8]
    paddw           m3, m2, [t2+r10*2+416*0]
    paddw           m3, [t1+r10*2+416*0]
    mova [t2+r10*2+416* 6], m0
    mova [t2+r10*2+416*0], m2
    punpcklwd       m0, m4, m1        ; b3
    vpdpwssd       m16, m0, m0        ; -p3
    punpckhwd       m1, m4, m1
    vpdpwssd       m17, m1, m1
    punpcklwd       m2, m3, m4        ; b5
    vpdpwssd       m18, m2, m2        ; -p5
    punpckhwd       m3, m4
    vpdpwssd       m19, m3, m3
    pmulld         m16, m12           ; p3 * s1
    pmulld         m17, m12
    pmulld         m18, m11           ; p5 * s0
    pmulld         m19, m11
    pmaddwd         m0, m13           ; b3 * 455
    pmaddwd         m1, m13
    pmaddwd         m2, m13           ; b5 * 164
    pmaddwd         m3, m13
    vpalignr   m17{k2}, m16, m16, 2
    vpalignr   m19{k2}, m18, m18, 2
    paddusw        m17, m14
    mova           m16, m22
    psraw          m17, 4             ; min(z3, 255) - 256
    vpermt2b       m16, m17, m23      ; sgr_x_by_x[128..255]
    vpmovb2m        k3, m17
    vpermi2b       m17, m20, m21      ; sgr_x_by_x[  0..127]
    paddusw        m19, m14
    mova           m18, m22
    psraw          m19, 4             ; min(z5, 255) - 256
    vpermt2b       m18, m19, m23      ; sgr_x_by_x[128..255]
    vpmovb2m        k4, m19
    vpermi2b       m19, m20, m21      ; sgr_x_by_x[  0..127]
    vmovdqu8   m17{k3}, m16           ; x3
    vmovdqu8   m19{k4}, m18           ; x5
    pandn          m16, m24, m17
    psrld          m17, 16
    pmulld          m0, m16
    pmulld          m1, m17
    pandn          m18, m24, m19
    psrld          m19, m19, 16
    pmulld          m2, m18
    pmulld          m3, m19
    paddd           m0, m15           ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m15
    vpternlogd     m16, m0, m24, 0xd8 ; a3 | (b3 << 12)
    vpternlogd     m17, m1, m24, 0xd8
    mova          [t3+r10*4+416*8+  8], m16
    mova          [t3+r10*4+416*8+ 24], xm17
    vextracti32x4 [t3+r10*4+416*8+ 56], m17, 2
    paddd           m2, m15           ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m3, m15
    mova          [t3+r10*4+416*8+ 72], m17
    vextracti128  [t3+r10*4+416*8+ 72], ym16, 1
    vextracti32x4 [t3+r10*4+416*8+104], m16, 3
    vpternlogd     m18, m2, m24, 0xd8 ; a5 | (b5 << 12)
    vpternlogd     m19, m3, m24, 0xd8
    mova          [t3+r10*4+416*0+  8], m18
    mova          [t3+r10*4+416*0+ 24], xm19
    vextracti32x4 [t3+r10*4+416*0+ 56], m19, 2
    mova          [t3+r10*4+416*0+ 72], m19
    vextracti128  [t3+r10*4+416*0+ 72], ym18, 1
    vextracti32x4 [t3+r10*4+416*0+104], m18, 3
    add            r10, 32
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu            m0, [t3+r10*4+416*0+4]
    paddd           m1, m0, [t3+r10*4+416*0+0]
    mova           m16, [t3+r10*4+416*4+0]
    paddd           m1, [t3+r10*4+416*0+8]
    mova           m17, [t3+r10*4+416*8+0]
    paddd          m16, [t3+r10*4+416*4+8]
    paddd          m17, [t3+r10*4+416*8+8]
    paddd           m2, m16, [t3+r10*4+416*4+4]
    paddd           m3, m17, [t3+r10*4+416*8+4]
    paddd           m0, m1
    pslld           m1, 2
    pslld           m2, 2
    paddd           m1, m0            ; ab5 565
    paddd           m3, m3            ; ab3[ 0] 222
    psubd           m2, m16           ; ab3[-1] 343
    mova [t3+r10*4+416*20], m3
    pandn           m0, m24, m1       ; a5 565
    mova [t3+r10*4+416*24], m2
    psrld           m1, 12            ; b5 565
    mova [t3+r10*4+416*12], m0
    paddd           m3, m3
    mova [t3+r10*4+416*16], m1
    psubd           m3, m17           ; ab3[ 0] 343
    mova [t3+r10*4+416*28], m3
    add            r10, 16
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu            m2, [t3+r10*4+4]
    paddd           m3, m2, [t3+r10*4+0]
    paddd           m3, [t3+r10*4+8]
    mova            m1, [t3+r10*4+416*4+0]
    paddd           m2, m3
    pslld           m3, 2
    paddd           m1, [t3+r10*4+416*4+8]
    paddd           m3, m2
    pandn           m2, m24, m3
    psrld           m3, 12
    paddd           m0, m2, [t3+r10*4+416*12] ; a5
    paddd          m16, m3, [t3+r10*4+416*16] ; b5 + (1 << 8)
    mova [t3+r10*4+416*12], m2
    mova [t3+r10*4+416*16], m3
    paddd           m2, m1, [t3+r10*4+416*4+4]
    paddd           m2, m2                    ; ab3[ 1] 222
    mova            m3, [t3+r10*4+416*20]
    paddd          m17, m3, [t3+r10*4+416*24] ; ab3[ 0] 222 + ab3[-1] 343
    mova [t3+r10*4+416*20], m2
    paddd           m2, m2
    psubd           m2, m1                    ; ab3[ 1] 343
    mova [t3+r10*4+416*24], m2
    paddd           m2, m3                    ; ab3[ 0] 222 + ab3[ 1] 343
    pandn           m1, m24, m17
    psrld          m17, 12
    pandn           m3, m24, m2
    psrld           m2, 12
    paddd           m1, m3                    ; a3
    pmovzxbd        m3, [dstq+r10]
    paddd          m17, m2                    ; b3 + (1 << 8)
    pmaddwd         m0, m3                    ; a5 * src
    pmaddwd         m1, m3                    ; a3 * src
    vpshldd         m3, m25, 16               ; (dst << 16) + (1 << 15)
    psubd          m16, m0                    ; b5 - a5 * src + (1 << 8)
    psubd          m17, m1                    ; b3 - a3 * src + (1 << 8)
    psrld          m16, 9
    pslld          m17, 7
    vmovdqu8   m17{k2}, m16
    vpdpwssd        m3, m17, m26
    packuswb        m3, m2
    vpermb         m16, m27, m3
    mova    [dstq+r10], xm16
    add            r10, 16
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova            m1, [t3+r10*4+416*8+0]
    paddd           m1, [t3+r10*4+416*8+8]
    paddd           m2, m1, [t3+r10*4+416*8+4]
    paddd           m2, m2                    ; ab3[ 1] 222
    mova            m0, [t3+r10*4+416*20]
    paddd          m17, m0, [t3+r10*4+416*28] ; ab3[ 0] 222 + ab3[-1] 343
    pmovzxbd        m3, [dstq+r10]
    mova [t3+r10*4+416*20], m2
    paddd           m2, m2
    psubd           m2, m1                    ; ab3[ 1] 343
    mova [t3+r10*4+416*28], m2
    paddd           m0, m2                    ; ab3[ 0] 222 + ab3[ 1] 343
    pandn           m1, m24, m17
    psrld          m17, 12
    pandn           m2, m24, m0
    psrld           m0, 12
    paddd           m1, m2                    ; a3
    paddd          m17, m0                    ; b3 + (1 << 8)
    mova           m16, [t3+r10*4+416*16]     ; b5 + (1 << 7)
    pmaddwd         m1, m3                    ; a3 * src
    pmaddwd         m0, m3, [t3+r10*4+416*12] ; a5 * src
    vpshldd         m3, m25, 16               ; (dst << 16) + (1 << 15)
    psubd          m17, m1                    ; b3 - a3 * src + (1 << 8)
    psubd          m16, m0                    ; b5 - a5 * src + (1 << 7)
    pslld          m17, 7
    palignr    m17{k2}, m16, m16, 1
    vpdpwssd        m3, m17, m26
    packuswb        m3, m3
    vpermb         m16, m27, m3
    mova    [dstq+r10], xm16
    add            r10, 16
    jl .n1_loop
    add           dstq, strideq
    ret

%endif ; ARCH_X86_64
