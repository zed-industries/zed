; Copyright © 2018, VideoLAN and dav1d authors
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

SECTION_RODATA 32

wiener_l_shuf: db  4,  4,  4,  4,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
pb_0to31:      db  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
               db 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
wiener_shufA:  db  1,  7,  2,  8,  3,  9,  4, 10,  5, 11,  6, 12,  7, 13,  8, 14
wiener_shufB:  db  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8,  8,  9,  9, 10
wiener_shufC:  db  6,  5,  7,  6,  8,  7,  9,  8, 10,  9, 11, 10, 12, 11, 13, 12
sgr_r_ext:     times 16 db 1
               times 16 db 9

; dword version of dav1d_sgr_x_by_x[] for use with gathers, wastes a bit of
; cache but eliminates some shifts in the inner sgr loop which is overall a win
const sgr_x_by_x_avx2
              dd 255,128, 85, 64, 51, 43, 37, 32, 28, 26, 23, 21, 20, 18, 17, 16
              dd  15, 14, 13, 13, 12, 12, 11, 11, 10, 10,  9,  9,  9,  9,  8,  8
              dd   8,  8,  7,  7,  7,  7,  7,  6,  6,  6,  6,  6,  6,  6,  5,  5
              dd   5,  5,  5,  5,  5,  5,  5,  5,  4,  4,  4,  4,  4,  4,  4,  4
              dd   4,  4,  4,  4,  4,  4,  4,  4,  4,  3,  3,  3,  3,  3,  3,  3
              dd   3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  3
              dd   3,  3,  3,  3,  3,  3,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2
              dd   2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2
              dd   2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2
              dd   2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2
              dd   2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  1,  1,  1,  1,  1,  1
              dd   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1
              dd   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1
              dd   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1
              dd   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1
              dd   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  0

               times 4 db -1 ; needed for 16-bit sgr
pb_m5:         times 4 db -5
pb_3:          times 4 db 3
pw_5_6:        dw 5, 6

sgr_l_shuf:    db  0,  0,  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11
sgr_shuf:      db  1, -1,  2, -1,  3, -1,  4, -1,  5, -1,  6, -1,  7, -1,  8, -1
               db  9, -1, 10, -1, 11, -1, 12, -1

pw_256:        times 2 dw 256
pw_2056:       times 2 dw 2056
pw_m16380:     times 2 dw -16380
pd_25:         dd 25
pd_34816:      dd 34816
pd_m4096:      dd -4096
pd_0xf00801c7: dd 0xf00801c7
pd_0xf00800a4: dd 0xf00800a4

SECTION .text

DECLARE_REG_TMP 8, 7, 9, 11, 12, 13, 14 ; ring buffer pointers

INIT_YMM avx2
cglobal wiener_filter7_8bpc, 4, 15, 16, -384*12-16, dst, stride, left, lpf, \
                                                    w, h, edge, flt
    mov           fltq, r6mp
    movifnidn       hd, hm
    mov          edged, r7m
    mov             wd, wm
    vbroadcasti128  m6, [wiener_shufA]
    vpbroadcastb   m11, [fltq+ 0] ; x0 x0
    vbroadcasti128  m7, [wiener_shufB]
    vpbroadcastd   m12, [fltq+ 2]
    vbroadcasti128  m8, [wiener_shufC]
    packsswb       m12, m12       ; x1 x2
    vpbroadcastw   m13, [fltq+ 6] ; x3
    vbroadcasti128  m9, [sgr_shuf+6]
    add           lpfq, wq
    vpbroadcastd   m10, [pw_m16380]
    vpbroadcastd   m14, [fltq+16] ; y0 y1
    add           dstq, wq
    vpbroadcastd   m15, [fltq+20] ; y2 y3
    lea             t1, [rsp+wq*2+16]
    psllw          m14, 5
    neg             wq
    psllw          m15, 5
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
    movd           xm2, r10d
    vpbroadcastd    m0, [pb_3]
    vpbroadcastd    m1, [pb_m5]
    vpbroadcastb    m2, xm2
    movu            m3, [pb_0to31]
    psubb           m0, m2
    psubb           m1, m2
    pminub          m0, m3
    pminub          m1, m3
    pshufb          m4, m0
    pshufb          m5, m1
    ret
.h:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movd           xm4, [leftq]
    vpblendd        m4, [lpfq+r10-4], 0xfe
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    vbroadcasti128  m5, [lpfq+r10] ; avoid accessing memory located
    mova            m4, [lpfq+r10] ; before the start of the buffer
    palignr         m4, m5, 12
    pshufb          m4, [wiener_l_shuf]
    jmp .h_main
.h_top:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m4, [lpfq+r10-4]
.h_main:
    movu            m5, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -34
    jl .h_have_right
    call .extend_right
.h_have_right:
    pshufb          m0, m4, m6
    pmaddubsw       m0, m11
    pshufb          m1, m5, m6
    pmaddubsw       m1, m11
    pshufb          m2, m4, m7
    pmaddubsw       m2, m12
    pshufb          m3, m5, m7
    pmaddubsw       m3, m12
    paddw           m0, m2
    pshufb          m2, m4, m8
    pmaddubsw       m2, m12
    paddw           m1, m3
    pshufb          m3, m5, m8
    pmaddubsw       m3, m12
    pshufb          m4, m9
    paddw           m0, m2
    pmullw          m2, m4, m13
    pshufb          m5, m9
    paddw           m1, m3
    pmullw          m3, m5, m13
    psllw           m4, 7
    psllw           m5, 7
    paddw           m4, m10
    paddw           m5, m10
    paddw           m0, m2
    vpbroadcastd    m2, [pw_2056]
    paddw           m1, m3
    paddsw          m0, m4
    paddsw          m1, m5
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m2
    paddw           m1, m2
    mova [t1+r10*2+ 0], m0
    mova [t1+r10*2+32], m1
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movd           xm4, [leftq]
    vpblendd        m4, [lpfq+r10-4], 0xfe
    add          leftq, 4
    jmp .hv_main
.hv_extend_left:
    movu            m4, [lpfq+r10-4]
    pshufb          m4, [wiener_l_shuf]
    jmp .hv_main
.hv_bottom:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m4, [lpfq+r10-4]
.hv_main:
    movu            m5, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -34
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    pshufb          m0, m4, m6
    pmaddubsw       m0, m11
    pshufb          m1, m5, m6
    pmaddubsw       m1, m11
    pshufb          m2, m4, m7
    pmaddubsw       m2, m12
    pshufb          m3, m5, m7
    pmaddubsw       m3, m12
    paddw           m0, m2
    pshufb          m2, m4, m8
    pmaddubsw       m2, m12
    paddw           m1, m3
    pshufb          m3, m5, m8
    pmaddubsw       m3, m12
    pshufb          m4, m9
    paddw           m0, m2
    pmullw          m2, m4, m13
    pshufb          m5, m9
    paddw           m1, m3
    pmullw          m3, m5, m13
    psllw           m4, 7
    psllw           m5, 7
    paddw           m4, m10
    paddw           m5, m10
    paddw           m0, m2
    paddw           m1, m3
    mova            m2, [t4+r10*2]
    paddw           m2, [t2+r10*2]
    mova            m3, [t3+r10*2]
    paddsw          m0, m4
    vpbroadcastd    m4, [pw_2056]
    paddsw          m1, m5
    mova            m5, [t5+r10*2]
    paddw           m5, [t1+r10*2]
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m4
    paddw           m1, m4
    paddw           m4, m0, [t6+r10*2]
    mova    [t0+r10*2], m0
    punpcklwd       m0, m2, m3
    pmaddwd         m0, m15
    punpckhwd       m2, m3
    pmaddwd         m2, m15
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m14
    punpckhwd       m4, m5
    pmaddwd         m4, m14
    paddd           m0, m3
    paddd           m4, m2
    mova            m2, [t4+r10*2+32]
    paddw           m2, [t2+r10*2+32]
    mova            m3, [t3+r10*2+32]
    mova            m5, [t5+r10*2+32]
    paddw           m5, [t1+r10*2+32]
    packuswb        m0, m4
    paddw           m4, m1, [t6+r10*2+32]
    mova [t0+r10*2+32], m1
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m15
    punpckhwd       m2, m3
    pmaddwd         m2, m15
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m14
    punpckhwd       m4, m5
    pmaddwd         m4, m14
    paddd           m1, m3
    paddd           m2, m4
    packuswb        m1, m2
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
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
    mova            m2, [t4+r10*2+ 0]
    paddw           m2, [t2+r10*2+ 0]
    mova            m4, [t3+r10*2+ 0]
    mova            m6, [t1+r10*2+ 0]
    paddw           m8, m6, [t6+r10*2+ 0]
    paddw           m6, [t5+r10*2+ 0]
    mova            m3, [t4+r10*2+32]
    paddw           m3, [t2+r10*2+32]
    mova            m5, [t3+r10*2+32]
    mova            m7, [t1+r10*2+32]
    paddw           m9, m7, [t6+r10*2+32]
    paddw           m7, [t5+r10*2+32]
    punpcklwd       m0, m2, m4
    pmaddwd         m0, m15
    punpckhwd       m2, m4
    pmaddwd         m2, m15
    punpcklwd       m4, m8, m6
    pmaddwd         m4, m14
    punpckhwd       m6, m8, m6
    pmaddwd         m6, m14
    punpcklwd       m1, m3, m5
    pmaddwd         m1, m15
    punpckhwd       m3, m5
    pmaddwd         m3, m15
    punpcklwd       m5, m9, m7
    pmaddwd         m5, m14
    punpckhwd       m7, m9, m7
    pmaddwd         m7, m14
    paddd           m0, m4
    paddd           m2, m6
    paddd           m1, m5
    paddd           m3, m7
    packuswb        m0, m2
    packuswb        m1, m3
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
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

cglobal wiener_filter5_8bpc, 4, 13, 16, 384*8+16, dst, stride, left, lpf, \
                                                  w, h, edge, flt
    mov           fltq, r6mp
    movifnidn       hd, hm
    mov          edged, r7m
    mov             wd, wm
    vbroadcasti128  m6, [wiener_shufB]
    vpbroadcastd   m12, [fltq+ 2]
    vbroadcasti128  m7, [wiener_shufC]
    packsswb       m12, m12       ; x1 x2
    vpbroadcastw   m13, [fltq+ 6] ; x3
    vbroadcasti128  m8, [sgr_shuf+6]
    add           lpfq, wq
    vpbroadcastd    m9, [pw_m16380]
    vpbroadcastd   m10, [pw_2056]
    mova           m11, [wiener_l_shuf]
    vpbroadcastd   m14, [fltq+16] ; __ y1
    add           dstq, wq
    vpbroadcastd   m15, [fltq+20] ; y2 y3
    lea             t1, [rsp+wq*2+16]
    psllw          m14, 5
    neg             wq
    psllw          m15, 5
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
    movd           xm4, [leftq]
    vpblendd        m4, [lpfq+r10-4], 0xfe
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    vbroadcasti128  m5, [lpfq+r10] ; avoid accessing memory located
    mova            m4, [lpfq+r10] ; before the start of the buffer
    palignr         m4, m5, 12
    pshufb          m4, m11
    jmp .h_main
.h_top:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m4, [lpfq+r10-4]
.h_main:
    movu            m5, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -33
    jl .h_have_right
    call mangle(private_prefix %+ _wiener_filter7_8bpc_avx2).extend_right
.h_have_right:
    pshufb          m0, m4, m6
    pmaddubsw       m0, m12
    pshufb          m1, m5, m6
    pmaddubsw       m1, m12
    pshufb          m2, m4, m7
    pmaddubsw       m2, m12
    pshufb          m3, m5, m7
    pmaddubsw       m3, m12
    pshufb          m4, m8
    paddw           m0, m2
    pmullw          m2, m4, m13
    pshufb          m5, m8
    paddw           m1, m3
    pmullw          m3, m5, m13
    psllw           m4, 7
    psllw           m5, 7
    paddw           m4, m9
    paddw           m5, m9
    paddw           m0, m2
    paddw           m1, m3
    paddsw          m0, m4
    paddsw          m1, m5
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m10
    paddw           m1, m10
    mova [t1+r10*2+ 0], m0
    mova [t1+r10*2+32], m1
    add            r10, 32
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movd           xm4, [leftq]
    vpblendd        m4, [lpfq+r10-4], 0xfe
    add          leftq, 4
    jmp .hv_main
.hv_extend_left:
    movu            m4, [lpfq+r10-4]
    pshufb          m4, m11
    jmp .hv_main
.hv_bottom:
    mov            r10, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m4, [lpfq+r10-4]
.hv_main:
    movu            m5, [lpfq+r10+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -33
    jl .hv_have_right
    call mangle(private_prefix %+ _wiener_filter7_8bpc_avx2).extend_right
.hv_have_right:
    pshufb          m0, m4, m6
    pmaddubsw       m0, m12
    pshufb          m1, m5, m6
    pmaddubsw       m1, m12
    pshufb          m2, m4, m7
    pmaddubsw       m2, m12
    pshufb          m3, m5, m7
    pmaddubsw       m3, m12
    pshufb          m4, m8
    paddw           m0, m2
    pmullw          m2, m4, m13
    pshufb          m5, m8
    paddw           m1, m3
    pmullw          m3, m5, m13
    psllw           m4, 7
    psllw           m5, 7
    paddw           m4, m9
    paddw           m5, m9
    paddw           m0, m2
    paddw           m1, m3
    mova            m2, [t3+r10*2]
    paddw           m2, [t1+r10*2]
    mova            m3, [t2+r10*2]
    paddsw          m0, m4
    paddsw          m1, m5
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m10
    paddw           m1, m10
    paddw           m4, m0, [t4+r10*2]
    mova    [t0+r10*2], m0
    punpcklwd       m0, m2, m3
    pmaddwd         m0, m15
    punpckhwd       m2, m3
    pmaddwd         m2, m15
    punpcklwd       m3, m4, m4
    pmaddwd         m3, m14
    punpckhwd       m4, m4
    pmaddwd         m4, m14
    paddd           m0, m3
    paddd           m4, m2
    mova            m2, [t3+r10*2+32]
    paddw           m2, [t1+r10*2+32]
    mova            m3, [t2+r10*2+32]
    packuswb        m0, m4
    paddw           m4, m1, [t4+r10*2+32]
    mova [t0+r10*2+32], m1
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m15
    punpckhwd       m2, m3
    pmaddwd         m2, m15
    punpcklwd       m3, m4, m4
    pmaddwd         m3, m14
    punpckhwd       m4, m4
    pmaddwd         m4, m14
    paddd           m1, m3
    paddd           m2, m4
    packuswb        m1, m2
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
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
    psrld          m13, m14, 16 ; y1 __
.v_loop:
    mova            m6, [t1+r10*2+ 0]
    paddw           m2, m6, [t3+r10*2+ 0]
    mova            m4, [t2+r10*2+ 0]
    mova            m7, [t1+r10*2+32]
    paddw           m3, m7, [t3+r10*2+32]
    mova            m5, [t2+r10*2+32]
    paddw           m6, [t4+r10*2+ 0]
    paddw           m7, [t4+r10*2+32]
    punpcklwd       m0, m2, m4
    pmaddwd         m0, m15
    punpckhwd       m2, m4
    pmaddwd         m2, m15
    punpcklwd       m1, m3, m5
    pmaddwd         m1, m15
    punpckhwd       m3, m5
    pmaddwd         m3, m15
    punpcklwd       m5, m7, m6
    pmaddwd         m4, m5, m14
    punpckhwd       m7, m6
    pmaddwd         m6, m7, m14
    pmaddwd         m5, m13
    pmaddwd         m7, m13
    paddd           m0, m4
    paddd           m2, m6
    paddd           m1, m5
    paddd           m3, m7
    packuswb        m0, m2
    packuswb        m1, m3
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
    mova    [dstq+r10], m0
    add            r10, 32
    jl .v_loop
    ret

cglobal sgr_filter_5x5_8bpc, 4, 13, 16, 400*24+16, dst, stride, left, lpf, \
                                                   w, h, edge, params
%define base r12-sgr_x_by_x_avx2-256*4
    lea            r12, [sgr_x_by_x_avx2+256*4]
    mov        paramsq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    vbroadcasti128  m8, [base+sgr_shuf+0]
    vbroadcasti128  m9, [base+sgr_shuf+8]
    add           lpfq, wq
    vbroadcasti128 m10, [base+sgr_shuf+2]
    add           dstq, wq
    vbroadcasti128 m11, [base+sgr_shuf+6]
    lea             t3, [rsp+wq*4+16+400*12]
    vpbroadcastd   m12, [paramsq+0] ; s0
    pxor            m6, m6
    vpbroadcastw    m7, [paramsq+8] ; w0
    lea             t1, [rsp+wq*2+20]
    vpbroadcastd   m13, [base+pd_0xf00800a4]
    neg             wq
    vpbroadcastd   m14, [base+pd_34816]  ; (1 << 11) + (1 << 15)
    psllw           m7, 4
    vpbroadcastd   m15, [base+pd_m4096]
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
    movd           xm2, r10d
    mova            m0, [sgr_r_ext]
    vpbroadcastb    m2, xm2
    psubb           m0, m2
    pminub          m0, [pb_0to31]
    pshufb          m5, m0
    ret
.h: ; horizontal boxsum
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    vpbroadcastd   xm0, [leftq]
    mova           xm5, [lpfq+wq]
    palignr        xm5, xm0, 12
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    mova           xm5, [lpfq+wq]
    pshufb         xm5, [base+sgr_l_shuf]
    jmp .h_main
.h_top:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu           xm5, [lpfq+r10-2]
.h_main:
    vinserti128     m5, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -18
    jl .h_have_right
    call .extend_right
.h_have_right:
    pshufb          m3, m5, m8
    pmullw          m4, m3, m3
    pshufb          m2, m5, m9
    paddw           m0, m3, m2
    shufps          m3, m2, q2121
    paddw           m0, m3
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m1
    punpckhwd       m2, m3
    pmaddwd         m2, m2
    punpcklwd       m3, m4, m6
    paddd           m1, m3
    punpckhwd       m4, m6
    paddd           m2, m4
    pshufb          m4, m5, m10
    paddw           m0, m4
    pshufb          m5, m11
    paddw           m0, m5 ; sum
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m3
    punpckhwd       m4, m5
    pmaddwd         m4, m4
    test         edgeb, 16 ; y > 0
    jz .h_loop_end
    paddw           m0, [t1+r10*2+400*0]
    paddd           m1, [t1+r10*2+400*2]
    paddd           m2, [t1+r10*2+400*4]
.h_loop_end:
    paddd           m1, m3 ; sumsq
    paddd           m2, m4
    mova [t1+r10*2+400*0], m0
    mova [t1+r10*2+400*2], m1
    mova [t1+r10*2+400*4], m2
    add            r10, 16
    jl .h_loop
    ret
.top_fixup:
    lea            r10, [wq-2]
.top_fixup_loop: ; the sums of the first row needs to be doubled
    mova            m0, [t1+r10*2+400*0]
    mova            m1, [t1+r10*2+400*2]
    mova            m2, [t1+r10*2+400*4]
    paddw           m0, m0
    paddd           m1, m1
    paddd           m2, m2
    mova [t2+r10*2+400*0], m0
    mova [t2+r10*2+400*2], m1
    mova [t2+r10*2+400*4], m2
    add            r10, 16
    jl .top_fixup_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    vpbroadcastd   xm0, [leftq]
    mova           xm5, [lpfq+wq]
    palignr        xm5, xm0, 12
    add          leftq, 4
    jmp .hv_main
.hv_extend_left:
    mova           xm5, [lpfq+wq]
    pshufb         xm5, [base+sgr_l_shuf]
    jmp .hv_main
.hv_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu           xm5, [lpfq+r10-2]
.hv_main:
    vinserti128     m5, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -18
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    pshufb          m1, m5, m8
    pmullw          m4, m1, m1
    pshufb          m3, m5, m9
    paddw           m0, m1, m3
    shufps          m1, m3, q2121
    paddw           m0, m1
    punpcklwd       m2, m3, m1
    pmaddwd         m2, m2
    punpckhwd       m3, m1
    pmaddwd         m3, m3
    punpcklwd       m1, m4, m6
    paddd           m2, m1
    punpckhwd       m4, m6
    paddd           m3, m4
    pshufb          m1, m5, m10
    paddw           m0, m1
    pshufb          m5, m11
    paddw           m0, m5               ; h sum
    punpcklwd       m4, m5, m1
    pmaddwd         m4, m4
    punpckhwd       m5, m1
    pmaddwd         m5, m5
    paddw           m1, m0, [t1+r10*2+400*0]
    paddd           m2, m4               ; h sumsq
    paddd           m3, m5
    paddd           m4, m2, [t1+r10*2+400*2]
    paddd           m5, m3, [t1+r10*2+400*4]
    test            hd, hd
    jz .hv_last_row
.hv_main2:
    paddw           m1, [t2+r10*2+400*0] ; hv sum
    paddd           m4, [t2+r10*2+400*2] ; hv sumsq
    paddd           m5, [t2+r10*2+400*4]
    mova [t0+r10*2+400*0], m0
    mova [t0+r10*2+400*2], m2
    mova [t0+r10*2+400*4], m3
    vpbroadcastd    m2, [pd_25]
    punpcklwd       m0, m1, m6           ; b
    punpckhwd       m1, m6
    pmulld          m4, m2               ; a * 25
    pmulld          m5, m2
    pmaddwd         m2, m0, m0           ; b * b
    pmaddwd         m3, m1, m1
    psubd           m4, m2               ; p
    psubd           m5, m3
    pmulld          m4, m12              ; p * s
    pmulld          m5, m12
    pmaddwd         m0, m13              ; b * 164
    pmaddwd         m1, m13
    paddusw         m4, m13
    paddusw         m5, m13
    psrad           m3, m4, 20           ; min(z, 255) - 256
    vpgatherdd      m2, [r12+m3*4], m4   ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r12+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    paddd           m0, m14              ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m14
    pand            m0, m15
    pand            m1, m15
    por             m0, m2               ; a | (b << 12)
    por             m1, m3
    mova         [t3+r10*4+ 8], xm0      ; The neighbor calculations requires
    vextracti128 [t3+r10*4+40], m0, 1    ; 13 bits for a and 21 bits for b.
    mova         [t3+r10*4+24], xm1      ; Packing them allows for 12+20, but
    vextracti128 [t3+r10*4+56], m1, 1    ; that gets us most of the way.
    add            r10, 16
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    ret
.hv_last_row: ; esoteric edge case for odd heights
    mova [t1+r10*2+400*0], m1
    paddw              m1, m0
    mova [t1+r10*2+400*2], m4
    paddd              m4, m2
    mova [t1+r10*2+400*4], m5
    paddd              m5, m3
    jmp .hv_main2
.v: ; vertical boxsum + ab
    lea            r10, [wq-2]
.v_loop:
    mova            m0, [t1+r10*2+400*0]
    mova            m2, [t1+r10*2+400*2]
    mova            m3, [t1+r10*2+400*4]
    paddw           m1, m0, [t2+r10*2+400*0]
    paddd           m4, m2, [t2+r10*2+400*2]
    paddd           m5, m3, [t2+r10*2+400*4]
    paddw           m0, m0
    paddd           m2, m2
    paddd           m3, m3
    paddw           m1, m0               ; hv sum
    paddd           m4, m2               ; hv sumsq
    paddd           m5, m3
    vpbroadcastd    m2, [pd_25]
    punpcklwd       m0, m1, m6           ; b
    punpckhwd       m1, m6
    pmulld          m4, m2               ; a * 25
    pmulld          m5, m2
    pmaddwd         m2, m0, m0           ; b * b
    pmaddwd         m3, m1, m1
    psubd           m4, m2               ; p
    psubd           m5, m3
    pmulld          m4, m12              ; p * s
    pmulld          m5, m12
    pmaddwd         m0, m13              ; b * 164
    pmaddwd         m1, m13
    paddusw         m4, m13
    paddusw         m5, m13
    psrad           m3, m4, 20           ; min(z, 255) - 256
    vpgatherdd      m2, [r12+m3*4], m4   ; x
    psrad           m4, m5, 20
    vpgatherdd      m3, [r12+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    paddd           m0, m14              ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m14
    pand            m0, m15
    pand            m1, m15
    por             m0, m2               ; a | (b << 12)
    por             m1, m3
    mova         [t3+r10*4+ 8], xm0
    vextracti128 [t3+r10*4+40], m0, 1
    mova         [t3+r10*4+24], xm1
    vextracti128 [t3+r10*4+56], m1, 1
    add            r10, 16
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu            m0, [t3+r10*4+ 4]
    movu            m1, [t3+r10*4+36]
    paddd           m2, m0, [t3+r10*4+ 0]
    paddd           m3, m1, [t3+r10*4+32]
    paddd           m2, [t3+r10*4+ 8]
    paddd           m3, [t3+r10*4+40]
    paddd           m0, m2
    pslld           m2, 2
    paddd           m1, m3
    pslld           m3, 2
    paddd           m2, m0                ; ab 565
    paddd           m3, m1
    pandn           m0, m15, m2           ; a
    psrld           m2, 12                ; b
    pandn           m1, m15, m3
    psrld           m3, 12
    mova [t3+r10*4+400*4+ 0], m0
    mova [t3+r10*4+400*8+ 0], m2
    mova [t3+r10*4+400*4+32], m1
    mova [t3+r10*4+400*8+32], m3
    add            r10, 16
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu            m0, [t3+r10*4+ 4]
    movu            m1, [t3+r10*4+36]
    paddd           m2, m0, [t3+r10*4+ 0]
    paddd           m3, m1, [t3+r10*4+32]
    paddd           m2, [t3+r10*4+ 8]
    paddd           m3, [t3+r10*4+40]
    paddd           m0, m2
    pslld           m2, 2
    paddd           m1, m3
    pslld           m3, 2
    paddd           m2, m0
    paddd           m3, m1
    pandn           m0, m15, m2
    psrld           m2, 12
    pandn           m1, m15, m3
    psrld           m3, 12
    paddd           m4, m0, [t3+r10*4+400*4+ 0] ; a
    paddd           m5, m1, [t3+r10*4+400*4+32]
    mova [t3+r10*4+400*4+ 0], m0
    mova [t3+r10*4+400*4+32], m1
    paddd           m0, m2, [t3+r10*4+400*8+ 0] ; b
    paddd           m1, m3, [t3+r10*4+400*8+32]
    mova [t3+r10*4+400*8+ 0], m2
    mova [t3+r10*4+400*8+32], m3
    pmovzxbd        m2, [dstq+r10+0]
    pmovzxbd        m3, [dstq+r10+8]
    pmaddwd         m4, m2 ; a * src
    pmaddwd         m5, m3
    packssdw        m2, m3
    psubd           m0, m4 ; b - a * src + (1 << 8)
    psubd           m1, m5
    psrad           m0, 9
    psrad           m1, 9
    packssdw        m0, m1
    pmulhrsw        m0, m7
    paddw           m0, m2
    vextracti128   xm1, m0, 1
    packuswb       xm0, xm1
    pshufd         xm0, xm0, q3120
    mova    [dstq+r10], xm0
    add            r10, 16
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    pmovzxbd        m2, [dstq+r10+0]
    pmovzxbd        m3, [dstq+r10+8]
    pmaddwd         m4, m2, [t3+r10*4+400*4+ 0] ; a * src
    pmaddwd         m5, m3, [t3+r10*4+400*4+32]
    mova            m0, [t3+r10*4+400*8+ 0]     ; b
    mova            m1, [t3+r10*4+400*8+32]
    packssdw        m2, m3
    psubd           m0, m4                      ; b - a * src + (1 << 7)
    psubd           m1, m5
    psrad           m0, 8
    psrad           m1, 8
    packssdw        m0, m1
    pmulhrsw        m0, m7
    paddw           m0, m2
    vextracti128   xm1, m0, 1
    packuswb       xm0, xm1
    pshufd         xm0, xm0, q3120
    mova    [dstq+r10], xm0
    add            r10, 16
    jl .n1_loop
    add           dstq, strideq
    ret

cglobal sgr_filter_3x3_8bpc, 4, 15, 15, -400*28-16, dst, stride, left, lpf, \
                                                    w, h, edge, params
%define base r14-sgr_x_by_x_avx2-256*4
    mov        paramsq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    lea            r14, [sgr_x_by_x_avx2+256*4]
    vbroadcasti128  m8, [base+sgr_shuf+2]
    add           lpfq, wq
    vbroadcasti128  m9, [base+sgr_shuf+4]
    add           dstq, wq
    vbroadcasti128 m10, [base+sgr_shuf+6]
    lea             t3, [rsp+wq*4+16+400*12]
    vpbroadcastd   m11, [paramsq+ 4] ; s1
    pxor            m6, m6
    vpbroadcastw    m7, [paramsq+10] ; w1
    lea             t1, [rsp+wq*2+20]
    vpbroadcastd   m12, [base+pd_0xf00801c7]
    neg             wq
    vpbroadcastd   m13, [base+pd_34816] ; (1 << 11) + (1 << 15)
    psllw           m7, 4
    vpbroadcastd   m14, [base+pd_m4096]
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    add             t1, 400*6
    call .h_top
    lea             t4, [lpfq+strideq*4]
    mov           lpfq, dstq
    add             t4, strideq
    mov          [rsp], t4 ; below
    mov             t0, t2
    call .hv
.main:
    mov             t5, t3
    add             t3, 400*4
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
    lea             t0, [t1+400*6]
    mov             t2, t1
    call .v
    jmp .main
.h: ; horizontal boxsum
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    vpbroadcastd   xm0, [leftq]
    mova           xm5, [lpfq+wq]
    palignr        xm5, xm0, 12
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    mova           xm5, [lpfq+wq]
    pshufb         xm5, [base+sgr_l_shuf]
    jmp .h_main
.h_top:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu           xm5, [lpfq+r10-2]
.h_main:
    vinserti128     m5, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -17
    jl .h_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_avx2).extend_right
.h_have_right:
    pshufb          m0, m5, m8
    pmullw          m2, m0, m0
    pshufb          m4, m5, m9
    paddw           m0, m4
    pshufb          m5, m10
    paddw           m0, m5 ; sum
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m3
    punpckhwd       m4, m5
    pmaddwd         m4, m4
    punpcklwd       m1, m2, m6
    punpckhwd       m2, m6
    mova [t1+r10*2+400*0], m0
    paddd           m1, m3 ; sumsq
    paddd           m2, m4
    mova [t1+r10*2+400*2], m1
    mova [t1+r10*2+400*4], m2
    add            r10, 16
    jl .h_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    vpbroadcastd   xm0, [leftq]
    mova           xm5, [lpfq+wq]
    palignr        xm5, xm0, 12
    add          leftq, 4
    jmp .hv_main
.hv_extend_left:
    mova           xm5, [lpfq+wq]
    pshufb         xm5, [base+sgr_l_shuf]
    jmp .hv_main
.hv_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu           xm5, [lpfq+r10-2]
.hv_main:
    vinserti128     m5, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp           r10d, -17
    jl .hv_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_avx2).extend_right
.hv_have_right:
    pshufb          m0, m5, m8
    pmullw          m3, m0, m0
    pshufb          m1, m5, m9
    paddw           m0, m1
    pshufb          m5, m10
    paddw           m0, m5               ; h sum
    punpcklwd       m4, m5, m1
    pmaddwd         m4, m4
    punpckhwd       m5, m1
    pmaddwd         m5, m5
    paddw           m1, m0, [t2+r10*2+400*0]
    paddw           m1, [t1+r10*2+400*0] ; hv sum
    punpcklwd       m2, m3, m6
    punpckhwd       m3, m6
    paddd           m4, m2               ; h sumsq
    paddd           m5, m3
    paddd           m2, m4, [t2+r10*2+400*2]
    paddd           m3, m5, [t2+r10*2+400*4]
    paddd           m2, [t1+r10*2+400*2] ; hv sumsq
    paddd           m3, [t1+r10*2+400*4]
    mova [t0+r10*2+400*0], m0
    punpcklwd       m0, m1, m6           ; b
    punpckhwd       m1, m6
    mova [t0+r10*2+400*2], m4
    pslld           m4, m2, 3
    mova [t0+r10*2+400*4], m5
    pslld           m5, m3, 3
    paddd           m4, m2               ; a * 9
    pmaddwd         m2, m0, m0           ; b * b
    paddd           m5, m3
    pmaddwd         m3, m1, m1
    psubd           m4, m2               ; p
    psubd           m5, m3
    pmulld          m4, m11              ; p * s
    pmulld          m5, m11
    pmaddwd         m0, m12              ; b * 455
    pmaddwd         m1, m12
    paddusw         m4, m12
    paddusw         m5, m12
    psrad           m3, m4, 20           ; min(z, 255) - 256
    vpgatherdd      m2, [r14+m3*4], m4
    psrad           m4, m5, 20
    vpgatherdd      m3, [r14+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    paddd           m0, m13              ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m13
    pand            m0, m14
    pand            m1, m14
    por             m0, m2               ; a | (b << 12)
    por             m1, m3
    mova         [t3+r10*4+ 8], xm0
    vextracti128 [t3+r10*4+40], m0, 1
    mova         [t3+r10*4+24], xm1
    vextracti128 [t3+r10*4+56], m1, 1
    add            r10, 16
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    ret
.v: ; vertical boxsum + ab
    lea            r10, [wq-2]
.v_loop:
    mova            m1, [t1+r10*2+400*0]
    paddw           m1, m1
    paddw           m1, [t2+r10*2+400*0] ; hv sum
    mova            m2, [t1+r10*2+400*2]
    mova            m3, [t1+r10*2+400*4]
    paddd           m2, m2
    paddd           m3, m3
    paddd           m2, [t2+r10*2+400*2] ; hv sumsq
    paddd           m3, [t2+r10*2+400*4]
    punpcklwd       m0, m1, m6           ; b
    punpckhwd       m1, m6
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2               ; a * 9
    pmaddwd         m2, m0, m0           ; b * b
    paddd           m5, m3
    pmaddwd         m3, m1, m1
    psubd           m4, m2               ; p
    psubd           m5, m3
    pmulld          m4, m11              ; p * s
    pmulld          m5, m11
    pmaddwd         m0, m12              ; b * 455
    pmaddwd         m1, m12
    paddusw         m4, m12
    paddusw         m5, m12
    psrad           m3, m4, 20           ; min(z, 255) - 256
    vpgatherdd      m2, [r14+m3*4], m4
    psrad           m4, m5, 20
    vpgatherdd      m3, [r14+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    paddd           m0, m13              ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m13
    pand            m0, m14
    pand            m1, m14
    por             m0, m2               ; a | (b << 12)
    por             m1, m3
    mova         [t3+r10*4+ 8], xm0
    vextracti128 [t3+r10*4+40], m0, 1
    mova         [t3+r10*4+24], xm1
    vextracti128 [t3+r10*4+56], m1, 1
    add            r10, 16
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
    mov             t4, t3
    add             t3, 400*4
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
    mova [t3+r10*4+400*4], m1
    paddd           m1, m1
    mova    [t5+r10*4], m0
    psubd           m1, m3                ; ab[ 0] 343
    mova    [t4+r10*4], m1
    add            r10, 8
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
    mova            m4, [t3+r10*4+ 0]
    paddd           m4, [t3+r10*4+ 8]
    paddd           m5, m4, [t3+r10*4+ 4]
    paddd           m5, m5                ; ab[+1] 222
    mova            m2, [t3+r10*4+400*4+ 0]
    paddd           m0, m2, [t5+r10*4+ 0] ; ab[ 0] 222 + ab[-1] 343
    mova            m3, [t3+r10*4+400*4+32]
    paddd           m1, m3, [t5+r10*4+32]
    mova [t3+r10*4+400*4+ 0], m5
    paddd           m5, m5
    psubd           m5, m4                ; ab[+1] 343
    mova [t5+r10*4+ 0], m5
    paddd           m2, m5                ; ab[ 0] 222 + ab[+1] 343
    mova            m4, [t3+r10*4+32]
    paddd           m4, [t3+r10*4+40]
    paddd           m5, m4, [t3+r10*4+36]
    paddd           m5, m5
    mova [t3+r10*4+400*4+32], m5
    paddd           m5, m5
    psubd           m5, m4
    mova [t5+r10*4+32], m5
    pandn           m4, m14, m0
    psrld           m0, 12
    paddd           m3, m5
    pandn           m5, m14, m2
    psrld           m2, 12
    paddd           m4, m5                ; a
    pandn           m5, m14, m1
    psrld           m1, 12
    paddd           m0, m2                ; b + (1 << 8)
    pandn           m2, m14, m3
    psrld           m3, 12
    paddd           m5, m2
    pmovzxbd        m2, [dstq+r10+0]
    paddd           m1, m3
    pmovzxbd        m3, [dstq+r10+8]
    pmaddwd         m4, m2                ; a * src
    pmaddwd         m5, m3
    packssdw        m2, m3
    psubd           m0, m4                ; b - a * src + (1 << 8)
    psubd           m1, m5
    psrad           m0, 9
    psrad           m1, 9
    packssdw        m0, m1
    pmulhrsw        m0, m7
    paddw           m0, m2
    vextracti128   xm1, m0, 1
    packuswb       xm0, xm1
    pshufd         xm0, xm0, q3120
    mova    [dstq+r10], xm0
    add            r10, 16
    jl .n_loop
    mov            r10, t5
    mov             t5, t4
    mov             t4, r10
    add           dstq, strideq
    ret

cglobal sgr_filter_mix_8bpc, 4, 13, 16, 400*56+8, dst, stride, left, lpf, \
                                                  w, h, edge, params
%define base r12-sgr_x_by_x_avx2-256*4
    lea            r12, [sgr_x_by_x_avx2+256*4]
    mov        paramsq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    vbroadcasti128  m9, [base+sgr_shuf+0]
    vbroadcasti128 m10, [base+sgr_shuf+8]
    add           lpfq, wq
    vbroadcasti128 m11, [base+sgr_shuf+2]
    vbroadcasti128 m12, [base+sgr_shuf+6]
    add           dstq, wq
    vpbroadcastd   m15, [paramsq+8] ; w0 w1
    lea             t3, [rsp+wq*4+400*24+8]
    vpbroadcastd   m13, [paramsq+0] ; s0
    pxor            m7, m7
    vpbroadcastd   m14, [paramsq+4] ; s1
    lea             t1, [rsp+wq*2+12]
    neg             wq
    psllw          m15, 2 ; to reuse existing pd_m4096 register for rounding
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t2, t1
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_avx2).top_fixup
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
    lea             t2, [t1+400*12]
    lea            r10, [wq-2]
.top_fixup_loop:
    mova            m0, [t1+r10*2+400* 0]
    mova            m1, [t1+r10*2+400* 2]
    mova            m2, [t1+r10*2+400* 4]
    paddw           m0, m0
    mova            m3, [t1+r10*2+400* 6]
    paddd           m1, m1
    mova            m4, [t1+r10*2+400* 8]
    paddd           m2, m2
    mova            m5, [t1+r10*2+400*10]
    mova [t2+r10*2+400* 0], m0
    mova [t2+r10*2+400* 2], m1
    mova [t2+r10*2+400* 4], m2
    mova [t2+r10*2+400* 6], m3
    mova [t2+r10*2+400* 8], m4
    mova [t2+r10*2+400*10], m5
    add            r10, 16
    jl .top_fixup_loop
    call .v0
    jmp .main
.h: ; horizontal boxsums
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    vpbroadcastd   xm0, [leftq]
    mova           xm5, [lpfq+wq]
    palignr        xm5, xm0, 12
    add          leftq, 4
    jmp .h_main
.h_extend_left:
    mova           xm5, [lpfq+wq]
    pshufb         xm5, [base+sgr_l_shuf]
    jmp .h_main
.h_top:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu           xm5, [lpfq+r10-2]
.h_main:
    vinserti128     m5, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp           r10d, -18
    jl .h_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_avx2).extend_right
.h_have_right:
    pshufb          m6, m5, m9
    pshufb          m4, m5, m10
    paddw           m8, m6, m4
    shufps          m0, m6, m4, q2121
    pmullw          m3, m0, m0
    pshufb          m2, m5, m11
    paddw           m0, m2
    pshufb          m5, m12
    paddw           m0, m5 ; sum3
    punpcklwd       m1, m2, m5
    pmaddwd         m1, m1
    punpckhwd       m2, m5
    pmaddwd         m2, m2
    punpcklwd       m5, m6, m4
    pmaddwd         m5, m5
    punpckhwd       m6, m4
    pmaddwd         m6, m6
    punpcklwd       m4, m3, m7
    paddd           m1, m4 ; sumsq3
    punpckhwd       m3, m7
    paddd           m2, m3
    mova [t1+r10*2+400* 6], m0
    mova [t1+r10*2+400* 8], m1
    mova [t1+r10*2+400*10], m2
    paddw           m8, m0 ; sum5
    paddd           m5, m1 ; sumsq5
    paddd           m6, m2
    mova [t1+r10*2+400* 0], m8
    mova [t1+r10*2+400* 2], m5
    mova [t1+r10*2+400* 4], m6
    add            r10, 16
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsums + vertical boxsum3 + ab3 (even rows)
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    vpbroadcastd   xm0, [leftq]
    mova           xm5, [lpfq+wq]
    palignr        xm5, xm0, 12
    add          leftq, 4
    jmp .hv0_main
.hv0_extend_left:
    mova           xm5, [lpfq+wq]
    pshufb         xm5, [base+sgr_l_shuf]
    jmp .hv0_main
.hv0_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
.hv0_loop:
    movu           xm5, [lpfq+r10-2]
.hv0_main:
    vinserti128     m5, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp           r10d, -18
    jl .hv0_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_avx2).extend_right
.hv0_have_right:
    pshufb          m6, m5, m9
    pshufb          m4, m5, m10
    paddw           m8, m6, m4
    shufps          m1, m6, m4, q2121
    pmullw          m0, m1, m1
    pshufb          m3, m5, m11
    paddw           m1, m3
    pshufb          m5, m12
    paddw           m1, m5 ; sum3
    punpcklwd       m2, m3, m5
    pmaddwd         m2, m2
    punpckhwd       m3, m5
    pmaddwd         m3, m3
    punpcklwd       m5, m6, m4
    pmaddwd         m5, m5
    punpckhwd       m6, m4
    pmaddwd         m6, m6
    punpcklwd       m4, m0, m7
    paddd           m2, m4 ; sumsq3
    punpckhwd       m0, m7
    paddd           m3, m0
    paddw           m8, m1 ; sum5
    paddd           m5, m2 ; sumsq5
    paddd           m6, m3
    mova [t3+r10*4+400*8+ 8], m8 ; we need a clean copy of the last row
    mova [t3+r10*4+400*0+ 8], m5 ; in case height is odd
    mova [t3+r10*4+400*0+40], m6
    paddw           m8, [t1+r10*2+400* 0]
    paddd           m5, [t1+r10*2+400* 2]
    paddd           m6, [t1+r10*2+400* 4]
    mova [t1+r10*2+400* 0], m8
    mova [t1+r10*2+400* 2], m5
    mova [t1+r10*2+400* 4], m6
    paddw           m0, m1, [t1+r10*2+400* 6]
    paddd           m4, m2, [t1+r10*2+400* 8]
    paddd           m5, m3, [t1+r10*2+400*10]
    mova [t1+r10*2+400* 6], m1
    mova [t1+r10*2+400* 8], m2
    mova [t1+r10*2+400*10], m3
    paddw           m1, m0, [t2+r10*2+400* 6]
    paddd           m2, m4, [t2+r10*2+400* 8]
    paddd           m3, m5, [t2+r10*2+400*10]
    mova [t2+r10*2+400* 6], m0
    mova [t2+r10*2+400* 8], m4
    mova [t2+r10*2+400*10], m5
    punpcklwd       m0, m1, m7           ; b3
    punpckhwd       m1, m7
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2               ; a3 * 9
    pmaddwd         m2, m0, m0           ; b3 * b
    paddd           m5, m3
    pmaddwd         m3, m1, m1
    psubd           m4, m2               ; p3
    vpbroadcastd    m2, [base+pd_0xf00801c7]
    psubd           m5, m3
    pmulld          m4, m14              ; p3 * s1
    pmulld          m5, m14
    pmaddwd         m0, m2               ; b3 * 455
    pmaddwd         m1, m2
    paddusw         m4, m2
    paddusw         m5, m2
    psrad           m3, m4, 20           ; min(z3, 255) - 256
    vpgatherdd      m2, [r12+m3*4], m4
    psrad           m4, m5, 20
    vpgatherdd      m3, [r12+m4*4], m5
    vpbroadcastd    m4, [base+pd_34816]
    pmulld          m0, m2
    vpbroadcastd    m5, [base+pd_m4096]
    pmulld          m1, m3
    paddd           m0, m4               ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m4
    pand            m0, m5
    pand            m1, m5
    por             m0, m2               ; a3 | (b3 << 12)
    por             m1, m3
    mova         [t3+r10*4+400*4+ 8], xm0
    vextracti128 [t3+r10*4+400*4+40], m0, 1
    mova         [t3+r10*4+400*4+24], xm1
    vextracti128 [t3+r10*4+400*4+56], m1, 1
    add            r10, 16
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    vpbroadcastd   xm0, [leftq]
    mova           xm5, [lpfq+wq]
    palignr        xm5, xm0, 12
    add          leftq, 4
    jmp .hv1_main
.hv1_extend_left:
    mova           xm5, [lpfq+wq]
    pshufb         xm5, [base+sgr_l_shuf]
    jmp .hv1_main
.hv1_bottom:
    lea            r10, [wq-2]
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
.hv1_loop:
    movu           xm5, [lpfq+r10-2]
.hv1_main:
    vinserti128     m5, [lpfq+r10+6], 1
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp           r10d, -18
    jl .hv1_have_right
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_avx2).extend_right
.hv1_have_right:
    pshufb          m6, m5, m9
    pshufb          m3, m5, m10
    paddw           m8, m6, m3
    shufps          m2, m6, m3, q2121
    pmullw          m1, m2, m2
    pshufb          m0, m5, m11
    paddw           m2, m0
    pshufb          m5, m12
    paddw           m2, m5 ; sum3
    punpcklwd       m4, m5, m0
    pmaddwd         m4, m4
    punpckhwd       m5, m0
    pmaddwd         m5, m5
    punpcklwd       m0, m6, m3
    pmaddwd         m0, m0
    punpckhwd       m6, m3
    pmaddwd         m6, m6
    punpcklwd       m3, m1, m7
    paddd           m4, m3 ; sumsq3
    punpckhwd       m1, m7
    paddd           m5, m1
    paddw           m1, m2, [t2+r10*2+400* 6]
    mova [t2+r10*2+400* 6], m2
    paddw           m8, m2 ; sum5
    paddd           m2, m4, [t2+r10*2+400* 8]
    paddd           m3, m5, [t2+r10*2+400*10]
    mova [t2+r10*2+400* 8], m4
    mova [t2+r10*2+400*10], m5
    paddd           m4, m0 ; sumsq5
    paddd           m5, m6
    punpcklwd       m0, m1, m7           ; b3
    punpckhwd       m1, m7
    pslld           m6, m2, 3
    pslld           m7, m3, 3
    paddd           m6, m2               ; a3 * 9
    pmaddwd         m2, m0, m0           ; b3 * b3
    paddd           m7, m3
    pmaddwd         m3, m1, m1
    psubd           m6, m2               ; p3
    vpbroadcastd    m2, [base+pd_0xf00801c7]
    psubd           m7, m3
    pmulld          m6, m14              ; p3 * s1
    pmulld          m7, m14
    pmaddwd         m0, m2               ; b3 * 455
    pmaddwd         m1, m2
    paddusw         m6, m2
    paddusw         m7, m2
    psrad           m3, m6, 20           ; min(z3, 255) - 256
    vpgatherdd      m2, [r12+m3*4], m6
    psrad           m6, m7, 20
    vpgatherdd      m3, [r12+m6*4], m7
    vpbroadcastd    m6, [base+pd_34816]  ; x3
    pmulld          m0, m2
    vpbroadcastd    m7, [base+pd_m4096]
    pmulld          m1, m3
    paddd           m0, m6               ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m6
    pand            m0, m7
    pand            m7, m1
    por             m0, m2               ; a3 | (b3 << 12)
    por             m7, m3
    paddw           m1, m8, [t2+r10*2+400*0]
    paddd           m2, m4, [t2+r10*2+400*2]
    paddd           m3, m5, [t2+r10*2+400*4]
    paddw           m1, [t1+r10*2+400*0]
    paddd           m2, [t1+r10*2+400*2]
    paddd           m3, [t1+r10*2+400*4]
    mova [t2+r10*2+400*0], m8
    mova [t2+r10*2+400*2], m4
    mova [t2+r10*2+400*4], m5
    mova         [t3+r10*4+400*8+ 8], xm0
    vextracti128 [t3+r10*4+400*8+40], m0, 1
    mova         [t3+r10*4+400*8+24], xm7
    vextracti128 [t3+r10*4+400*8+56], m7, 1
    vpbroadcastd    m4, [base+pd_25]
    pxor            m7, m7
    punpcklwd       m0, m1, m7           ; b5
    punpckhwd       m1, m7
    pmulld          m2, m4               ; a5 * 25
    pmulld          m3, m4
    pmaddwd         m4, m0, m0           ; b5 * b5
    pmaddwd         m5, m1, m1
    psubd           m2, m4               ; p5
    vpbroadcastd    m4, [base+pd_0xf00800a4]
    psubd           m3, m5
    pmulld          m2, m13              ; p5 * s0
    pmulld          m3, m13
    pmaddwd         m0, m4               ; b5 * 164
    pmaddwd         m1, m4
    paddusw         m2, m4
    paddusw         m3, m4
    psrad           m5, m2, 20           ; min(z5, 255) - 256
    vpgatherdd      m4, [r12+m5*4], m2   ; x5
    psrad           m2, m3, 20
    vpgatherdd      m5, [r12+m2*4], m3
    pmulld          m0, m4
    pmulld          m1, m5
    paddd           m0, m6               ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m6
    vpbroadcastd    m6, [base+pd_m4096]
    pand            m0, m6
    pand            m1, m6
    por             m0, m4               ; a5 | (b5 << 12)
    por             m1, m5
    mova         [t3+r10*4+400*0+ 8], xm0
    vextracti128 [t3+r10*4+400*0+40], m0, 1
    mova         [t3+r10*4+400*0+24], xm1
    vextracti128 [t3+r10*4+400*0+56], m1, 1
    add            r10, 16
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab3 (even rows)
    lea            r10, [wq-2]
    vpbroadcastd    m6, [base+pd_34816]
    vpbroadcastd    m8, [base+pd_m4096]
.v0_loop:
    mova            m0, [t1+r10*2+400* 6]
    mova            m4, [t1+r10*2+400* 8]
    mova            m5, [t1+r10*2+400*10]
    paddw           m0, m0
    paddd           m4, m4
    paddd           m5, m5
    paddw           m1, m0, [t2+r10*2+400* 6]
    paddd           m2, m4, [t2+r10*2+400* 8]
    paddd           m3, m5, [t2+r10*2+400*10]
    mova [t2+r10*2+400* 6], m0
    mova [t2+r10*2+400* 8], m4
    mova [t2+r10*2+400*10], m5
    punpcklwd       m0, m1, m7           ; b3
    punpckhwd       m1, m7
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2               ; a3 * 9
    pmaddwd         m2, m0, m0           ; b3 * b3
    paddd           m5, m3
    pmaddwd         m3, m1, m1
    psubd           m4, m2               ; p3
    vpbroadcastd    m2, [base+pd_0xf00801c7]
    psubd           m5, m3
    pmulld          m4, m14              ; p3 * s1
    pmulld          m5, m14
    pmaddwd         m0, m2               ; b3 * 455
    pmaddwd         m1, m2
    paddusw         m4, m2
    paddusw         m5, m2
    psrad           m3, m4, 20           ; min(z3, 255) - 256
    vpgatherdd      m2, [r12+m3*4], m4   ; x3
    psrad           m4, m5, 20
    vpgatherdd      m3, [r12+m4*4], m5
    pmulld          m0, m2
    pmulld          m1, m3
    paddd           m0, m6               ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m6
    pand            m0, m8
    pand            m1, m8
    por             m0, m2               ; a3 | (b3 << 12)
    por             m1, m3
    mova            m2, [t1+r10*2+400*0]
    mova            m3, [t1+r10*2+400*2]
    mova            m4, [t1+r10*2+400*4]
    mova [t3+r10*4+400*8+ 8], m2
    mova [t3+r10*4+400*0+ 8], m3
    mova [t3+r10*4+400*0+40], m4
    paddw           m2, m2               ; cc5
    paddd           m3, m3
    paddd           m4, m4
    mova [t1+r10*2+400*0], m2
    mova [t1+r10*2+400*2], m3
    mova [t1+r10*2+400*4], m4
    mova         [t3+r10*4+400*4+ 8], xm0
    vextracti128 [t3+r10*4+400*4+40], m0, 1
    mova         [t3+r10*4+400*4+24], xm1
    vextracti128 [t3+r10*4+400*4+56], m1, 1
    add            r10, 16
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
    lea            r10, [wq-2]
.v1_loop:
    mova            m4, [t1+r10*2+400* 6]
    mova            m5, [t1+r10*2+400* 8]
    mova            m6, [t1+r10*2+400*10]
    paddw           m1, m4, [t2+r10*2+400* 6]
    paddd           m2, m5, [t2+r10*2+400* 8]
    paddd           m3, m6, [t2+r10*2+400*10]
    mova [t2+r10*2+400* 6], m4
    mova [t2+r10*2+400* 8], m5
    mova [t2+r10*2+400*10], m6
    punpcklwd       m0, m1, m7           ; b3
    punpckhwd       m1, m7
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2               ; a3 * 9
    pmaddwd         m2, m0, m0           ; b3 * b3
    paddd           m5, m3
    pmaddwd         m3, m1, m1
    psubd           m4, m2               ; p3
    vpbroadcastd    m2, [base+pd_0xf00801c7]
    psubd           m5, m3
    pmulld          m4, m14              ; p3 * s1
    pmulld          m5, m14
    pmaddwd         m0, m2               ; b3 * 455
    pmaddwd         m1, m2
    paddusw         m4, m2
    paddusw         m5, m2
    psrad           m3, m4, 20           ; min(z3, 255) - 256
    vpgatherdd      m2, [r12+m3*4], m4   ; x3
    psrad           m4, m5, 20
    vpgatherdd      m3, [r12+m4*4], m5
    vpbroadcastd    m4, [base+pd_34816]
    pmulld          m0, m2
    vpbroadcastd    m8, [base+pd_m4096]
    pmulld          m1, m3
    paddd           m0, m4               ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m4
    pand            m0, m8
    pand            m8, m1
    por             m0, m2               ; a3 | (b3 << 12)
    por             m8, m3
    mova            m4, [t3+r10*4+400*8+ 8]
    mova            m5, [t3+r10*4+400*0+ 8]
    mova            m6, [t3+r10*4+400*0+40]
    paddw           m1, m4, [t2+r10*2+400*0]
    paddd           m2, m5, [t2+r10*2+400*2]
    paddd           m3, m6, [t2+r10*2+400*4]
    paddw           m1, [t1+r10*2+400*0]
    paddd           m2, [t1+r10*2+400*2]
    paddd           m3, [t1+r10*2+400*4]
    mova [t2+r10*2+400*0], m4
    mova [t2+r10*2+400*2], m5
    mova [t2+r10*2+400*4], m6
    vpbroadcastd    m4, [base+pd_25]
    mova         [t3+r10*4+400*8+ 8], xm0
    vextracti128 [t3+r10*4+400*8+40], m0, 1
    mova         [t3+r10*4+400*8+24], xm8
    vextracti128 [t3+r10*4+400*8+56], m8, 1
    punpcklwd       m0, m1, m7           ; b5
    punpckhwd       m1, m7
    pmulld          m2, m4               ; a5 * 25
    pmulld          m3, m4
    pmaddwd         m4, m0, m0           ; b5 * b5
    pmaddwd         m5, m1, m1
    psubd           m2, m4               ; p5
    vpbroadcastd    m4, [base+pd_0xf00800a4]
    psubd           m3, m5
    pmulld          m2, m13              ; p5 * s0
    pmulld          m3, m13
    pmaddwd         m0, m4               ; b5 * 164
    pmaddwd         m1, m4
    paddusw         m2, m4
    paddusw         m3, m4
    psrad           m5, m2, 20           ; min(z5, 255) - 256
    vpgatherdd      m4, [r12+m5*4], m2   ; x5
    psrad           m2, m3, 20
    vpgatherdd      m5, [r12+m2*4], m3
    pmulld          m0, m4
    vpbroadcastd    m6, [base+pd_34816]
    pmulld          m1, m5
    paddd           m0, m6               ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m6
    vpbroadcastd    m6, [base+pd_m4096]
    pand            m0, m6
    pand            m1, m6
    por             m0, m4               ; a5 | (b5 << 12)
    por             m1, m5
    mova         [t3+r10*4+400*0+ 8], xm0
    vextracti128 [t3+r10*4+400*0+40], m0, 1
    mova         [t3+r10*4+400*0+24], xm1
    vextracti128 [t3+r10*4+400*0+56], m1, 1
    add            r10, 16
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    mov            r10, wq
.prep_n_loop:
    movu            m0, [t3+r10*4+400*0+4]
    paddd           m1, m0, [t3+r10*4+400*0+0]
    mova            m4, [t3+r10*4+400*4+0]
    paddd           m1, [t3+r10*4+400*0+8]
    mova            m5, [t3+r10*4+400*8+0]
    paddd           m4, [t3+r10*4+400*4+8]
    paddd           m5, [t3+r10*4+400*8+8]
    paddd           m2, m4, [t3+r10*4+400*4+4]
    paddd           m3, m5, [t3+r10*4+400*8+4]
    paddd           m0, m1
    pslld           m1, 2
    pslld           m2, 2
    paddd           m1, m0                ; ab5 565
    paddd           m3, m3                ; ab3[ 0] 222
    psubd           m2, m4                ; ab3[-1] 343
    mova [t3+r10*4+400*20], m3
    pandn           m0, m6, m1            ; a5 565
    mova [t3+r10*4+400*24], m2
    psrld           m1, 12                ; b5 565
    mova [t3+r10*4+400*12], m0
    paddd           m3, m3
    mova [t3+r10*4+400*16], m1
    psubd           m3, m5                ; ab3[ 0] 343
    mova [t3+r10*4+400*28], m3
    add            r10, 8
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    mov            r10, wq
.n0_loop:
    movu            m0, [t3+r10*4+4]
    paddd           m4, m0, [t3+r10*4+0]
    paddd           m4, [t3+r10*4+8]
    paddd           m0, m4
    pslld           m4, 2
    paddd           m4, m0
    pandn           m0, m6, m4
    psrld           m4, 12
    paddd           m2, m0, [t3+r10*4+400*12] ; a5
    mova [t3+r10*4+400*12], m0
    paddd           m0, m4, [t3+r10*4+400*16] ; b5 + (1 << 8)
    mova [t3+r10*4+400*16], m4
    mova            m3, [t3+r10*4+400*4+0]
    paddd           m3, [t3+r10*4+400*4+8]
    paddd           m5, m3, [t3+r10*4+400*4+4]
    paddd           m5, m5                    ; ab3[ 1] 222
    mova            m4, [t3+r10*4+400*20]
    paddd           m1, m4, [t3+r10*4+400*24] ; ab3[ 0] 222 + ab3[-1] 343
    mova [t3+r10*4+400*20], m5
    paddd           m5, m5
    psubd           m5, m3                    ; ab3[ 1] 343
    mova [t3+r10*4+400*24], m5
    paddd           m4, m5                    ; ab3[ 0] 222 + ab3[ 1] 343
    pandn           m3, m6, m1
    psrld           m1, 12
    pandn           m5, m6, m4
    psrld           m4, 12
    paddd           m3, m5                    ; a3
    paddd           m1, m4                    ; b3 + (1 << 8)
    pmovzxbd        m4, [dstq+r10]
    pmaddwd         m2, m4                    ; a5 * src
    pmaddwd         m3, m4                    ; a3 * src
    psubd           m0, m2                    ; b5 - a5 * src + (1 << 8)
    psubd           m1, m3                    ; b3 - a3 * src + (1 << 8)
    psrld           m0, 9
    pslld           m1, 7
    pblendw         m0, m1, 0xaa
    pmaddwd         m0, m15
    psubd           m0, m6
    psrad           m0, 13
    paddd           m0, m4
    vextracti128   xm1, m0, 1
    packssdw       xm0, xm1
    packuswb       xm0, xm0
    movq    [dstq+r10], xm0
    add            r10, 8
    jl .n0_loop
    add           dstq, strideq
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    mov            r10, wq
.n1_loop:
    mova            m3, [t3+r10*4+400*8+0]
    paddd           m3, [t3+r10*4+400*8+8]
    paddd           m5, m3, [t3+r10*4+400*8+4]
    paddd           m5, m5                    ; ab3[ 1] 222
    mova            m4, [t3+r10*4+400*20]
    paddd           m1, m4, [t3+r10*4+400*28] ; ab3[ 0] 222 + ab3[-1] 343
    mova [t3+r10*4+400*20], m5
    paddd           m5, m5
    psubd           m5, m3                    ; ab3[ 1] 343
    mova [t3+r10*4+400*28], m5
    paddd           m4, m5                    ; ab3[ 0] 222 + ab3[ 1] 343
    pandn           m3, m6, m1
    psrld           m1, 12
    pandn           m5, m6, m4
    psrld           m4, 12
    paddd           m3, m5                    ; -a3
    paddd           m1, m4                    ;  b3 + (1 << 8)
    pmovzxbd        m4, [dstq+r10]
    pmaddwd         m2, m4, [t3+r10*4+400*12] ; -a5 * src
    mova            m0, [t3+r10*4+400*16]     ;  b5 + (1 << 7)
    pmaddwd         m3, m4                    ; -a3 * src
    psubd           m0, m2                    ; a5 * src + b5 + (1 << 7)
    psubd           m1, m3                    ; a3 * src + b3 + (1 << 8)
    psrld           m0, 8
    pslld           m1, 7
    pblendw         m0, m1, 0xaa
    pmaddwd         m0, m15
    psubd           m0, m6
    psrad           m0, 13
    paddd           m0, m4
    vextracti128   xm1, m0, 1
    packssdw       xm0, xm1
    packuswb       xm0, xm0
    movq    [dstq+r10], xm0
    add            r10, 8
    jl .n1_loop
    add           dstq, strideq
    ret

%endif ; ARCH_X86_64
