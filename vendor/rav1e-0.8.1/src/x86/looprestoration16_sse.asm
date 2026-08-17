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

wiener_shufA:  db  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11
wiener_shufB:  db  6,  7,  4,  5,  8,  9,  6,  7, 10, 11,  8,  9, 12, 13, 10, 11
wiener_shufC:  db  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13, 12, 13, 14, 15
wiener_shufD:  db  2,  3, -1, -1,  4,  5, -1, -1,  6,  7, -1, -1,  8,  9, -1, -1
wiener_shufE:  db  0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
wiener_lshuf5: db  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11
wiener_lshuf7: db  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7
sgr_lshuf3:    db  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11
sgr_lshuf5:    db  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9
pb_0to15:      db  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15

pb_m14_m13:    times 8 db -14,-13
pb_m10_m9:     times 8 db -10, -9
pb_m6_m5:      times 8 db  -6, -5
pb_m2_m1:      times 8 db  -2, -1
pb_2_3:        times 8 db   2,  3
pb_6_7:        times 8 db   6,  7
pw_256:        times 8 dw 256
pw_1023:       times 8 dw 1023
pd_8:          times 4 dd 8
pd_4096:       times 4 dd 4096
pd_34816:      times 4 dd 34816
pd_m262128:    times 4 dd -262128
pd_0xffff:     times 4 dd 0xffff
pd_0xf00800a4: times 4 dd 0xf00800a4
pd_0xf00801c7: times 4 dd 0xf00801c7
pd_0xfffffff0: times 4 dd 0xfffffff0

wiener_shifts: dw 4, 4, 2048, 2048, 1, 1, 8192, 8192
wiener_round:  dd 1049600, 1048832

cextern sgr_x_by_x

SECTION .text

%macro movif64 2 ; dst, src
 %if ARCH_X86_64
    mov             %1, %2
 %endif
%endmacro

%macro movif32 2 ; dst, src
 %if ARCH_X86_32
    mov             %1, %2
 %endif
%endmacro

INIT_XMM ssse3
%if ARCH_X86_32
DECLARE_REG_TMP 5, 6
 %if STACK_ALIGNMENT < 16
  %assign extra_stack 13*16
 %else
  %assign extra_stack 12*16
 %endif
cglobal wiener_filter7_16bpc, 4, 7, 8, -384*12-16-extra_stack, \
                              dst, stride, left, lpf, w, flt
 %if STACK_ALIGNMENT < 16
  %define lpfm        dword [esp+calloff+16*12+ 0]
  %define wm          dword [esp+calloff+16*12+ 4]
  %define hd          dword [esp+calloff+16*12+ 8]
  %define edgeb        byte [esp+calloff+16*12+12]
  %define edged       dword [esp+calloff+16*12+12]
 %else
  %define hd dword r5m
  %define edgeb byte r7m
 %endif
 %define PICmem dword [esp+calloff+4*0]
 %define t0m    dword [esp+calloff+4*1] ; wiener ring buffer pointers
 %define t1m    dword [esp+calloff+4*2]
 %define t2m    dword [esp+calloff+4*3]
 %define t3m    dword [esp+calloff+4*4]
 %define t4m    dword [esp+calloff+4*5]
 %define t5m    dword [esp+calloff+4*6]
 %define t6m    dword [esp+calloff+4*7]
 %define t2 t2m
 %define t3 t3m
 %define t4 t4m
 %define t5 t5m
 %define t6 t6m
 %define  m8 [esp+calloff+16*2]
 %define  m9 [esp+calloff+16*3]
 %define m10 [esp+calloff+16*4]
 %define m11 [esp+calloff+16*5]
 %define m12 [esp+calloff+16*6]
 %define m13 [esp+calloff+16*7]
 %define m14 [esp+calloff+16*8]
 %define m15 [esp+calloff+16*9]
 %define r10 r4
 %define base t0-wiener_shifts
 %assign calloff 0
 %if STACK_ALIGNMENT < 16
    mov             wd, [rstk+stack_offset+20]
    mov             wm, wd
    mov             r5, [rstk+stack_offset+24]
    mov             hd, r5
    mov             r5, [rstk+stack_offset+32]
    mov          edged, r5 ; edge
 %endif
%else
DECLARE_REG_TMP 8, 7, 9, 11, 12, 13, 14 ; wiener ring buffer pointers
cglobal wiener_filter7_16bpc, 4, 15, 16, -384*12-16, dst, stride, left, lpf, \
                                                     w, h, edge, flt
 %define base
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    movifnidn       wd, wm
%endif
%if ARCH_X86_64
    mov           fltq, r6mp
    movifnidn       hd, hm
    mov          edged, r7m
    mov            t3d, r8m ; pixel_max
    movq           m13, [fltq]
    movq           m15, [fltq+16]
%else
 %if STACK_ALIGNMENT < 16
    mov             t0, [rstk+stack_offset+28]
    mov             t1, [rstk+stack_offset+36] ; pixel_max
    movq            m1, [t0]    ; fx
    movq            m3, [t0+16] ; fy
    LEA             t0, wiener_shifts
 %else
    mov           fltq, r6m
    movq            m1, [fltq]
    movq            m3, [fltq+16]
    LEA             t0, wiener_shifts
    mov             t1, r8m ; pixel_max
 %endif
    mov         PICmem, t0
%endif
    mova            m6, [base+wiener_shufA]
    mova            m7, [base+wiener_shufB]
%if ARCH_X86_64
    lea             t4, [wiener_shifts]
    add             wd, wd
    pshufd         m12, m13, q0000 ; x0 x1
    pshufd         m13, m13, q1111 ; x2 x3
    pshufd         m14, m15, q0000 ; y0 y1
    pshufd         m15, m15, q1111 ; y2 y3
    mova            m8, [wiener_shufC]
    mova            m9, [wiener_shufD]
    add           lpfq, wq
    lea             t1, [rsp+wq+16]
    add           dstq, wq
    neg             wq
    shr            t3d, 11
 %define base t4-wiener_shifts
    movd           m10, [base+wiener_round+t3*4]
    movq           m11, [base+wiener_shifts+t3*8]
    pshufd         m10, m10, q0000
    pshufd          m0, m11, q0000
    pshufd         m11, m11, q1111
    pmullw         m12, m0 ; upshift filter coefs to make the
    pmullw         m13, m0 ; horizontal downshift constant
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm [rsp]
 %define base
 %define wiener_lshuf7_mem [wiener_lshuf7]
 %define pd_m262128_mem [pd_m262128]
%else
    add             wd, wd
    mova            m4, [base+wiener_shufC]
    mova            m5, [base+wiener_shufD]
    pshufd          m0, m1, q0000
    pshufd          m1, m1, q1111
    pshufd          m2, m3, q0000
    pshufd          m3, m3, q1111
    mova            m8, m4
    mova            m9, m5
    mova           m14, m2
    mova           m15, m3
    shr             t1, 11
    add           lpfq, wq
    mova            m3, [base+pd_m262128]
    movd            m4, [base+wiener_round+t1*4]
    movq            m5, [base+wiener_shifts+t1*8]
    lea             t1, [esp+extra_stack+wq+16]
    add           dstq, wq
    neg             wq
    pshufd          m4, m4, q0000
    pshufd          m2, m5, q0000
    pshufd          m5, m5, q1111
    mov             wm, wq
    pmullw          m0, m2
    pmullw          m1, m2
    mova            m2, [base+wiener_lshuf7]
 %define pd_m262128_mem [esp+calloff+16*10]
    mova pd_m262128_mem, m3
    mova           m10, m4
    mova           m11, m5
    mova           m12, m0
    mova           m13, m1
 %define wiener_lshuf7_mem [esp+calloff+16*11]
    mova wiener_lshuf7_mem, m2
%endif
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
    mov           lpfm, r10 ; below
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
    mov           lpfq, lpfm
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
    mov           lpfm, r10
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
    movif32         wq, wm
.v2:
    call .v
    movif32         wq, wm
    jmp .v1
.extend_right:
%assign stack_offset stack_offset+8
%assign calloff 8
    movif32         t0, PICmem
    pxor            m0, m0
    movd            m1, wd
    mova            m2, [base+pb_0to15]
    pshufb          m1, m0
    mova            m0, [base+pb_6_7]
    psubb           m0, m1
    pminub          m0, m2
    pshufb          m3, m0
    mova            m0, [base+pb_m2_m1]
    psubb           m0, m1
    pminub          m0, m2
    pshufb          m4, m0
    mova            m0, [base+pb_m10_m9]
    psubb           m0, m1
    pminub          m0, m2
    pshufb          m5, m0
    movif32         t0, t0m
    ret
%assign stack_offset stack_offset-4
%assign calloff 4
.h:
    movif64         wq, r4
    movif32         wq, wm
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movq            m3, [leftq]
    movhps          m3, [lpfq+wq]
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    mova            m3, [lpfq+wq]         ; avoid accessing memory located
    pshufb          m3, wiener_lshuf7_mem ; before the start of the buffer
    jmp .h_main
.h_top:
    movif64         wq, r4
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m3, [lpfq+wq-8]
.h_main:
    mova            m4, [lpfq+wq+0]
    movu            m5, [lpfq+wq+8]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             wd, -20
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
    mova            m2, pd_m262128_mem ; (1 << 4) - (1 << 18)
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
    mova       [t1+wq], m0
    add             wq, 16
    jl .h_loop
    movif32         wq, wm
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    movif64         wq, r4
    movif32        t0m, t0
    movif32        t1m, t1
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movq            m3, [leftq]
    movhps          m3, [lpfq+wq]
    add          leftq, 8
    jmp .hv_main
.hv_extend_left:
    mova            m3, [lpfq+wq]
    pshufb          m3, wiener_lshuf7_mem
    jmp .hv_main
.hv_bottom:
    movif64         wq, r4
    movif32        t0m, t0
    movif32        t1m, t1
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m3, [lpfq+wq-8]
.hv_main:
    mova            m4, [lpfq+wq+0]
    movu            m5, [lpfq+wq+8]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp             wd, -20
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    movif32         t1, t4m
    movif32         t0, t2m
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
    mova            m2, pd_m262128_mem
    pshufb          m4, m8
    pmaddwd         m1, m12
    pshufb          m5, m9
    paddw           m4, m5
    pmaddwd         m4, m13
    paddd           m0, m2
    paddd           m1, m2
%if ARCH_X86_64
    mova            m2, [t4+wq]
    paddw           m2, [t2+wq]
    mova            m5, [t3+wq]
%else
    mova            m2, [t1+wq]
    paddw           m2, [t0+wq]
    mov             t1, t3m
    mov             t0, t5m
    mova            m5, [t1+wq]
    mov             t1, t1m
%endif
    paddd           m0, m3
    paddd           m1, m4
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
%if ARCH_X86_64
    mova            m4, [t5+wq]
    paddw           m4, [t1+wq]
    psraw           m0, 1
    paddw           m3, m0, [t6+wq]
%else
    mova            m4, [t0+wq]
    paddw           m4, [t1+wq]
    mov             t0, t0m
    mov             t1, t6m
    psraw           m0, 1
    paddw           m3, m0, [t1+wq]
%endif
    mova       [t0+wq], m0
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
    psrad           m0, 6
    psrad           m2, 6
    packssdw        m0, m2
    pmulhw          m0, m11
    pxor            m1, m1
    pmaxsw          m0, m1
    mova     [dstq+wq], m0
    add             wq, 16
    jl .hv_loop
%if ARCH_X86_64
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, t6
%else
    mov             r4, t5m
    mov             t1, t4m
    mov            t6m, r4
    mov            t5m, t1
    mov             r4, t3m
    mov             t1, t2m
    mov            t4m, r4
    mov            t3m, t1
    mov             r4, t1m
    mov             t1, t0
    mov            t2m, r4
    mov             t0, t6m
    mov             wq, wm
%endif
    add           dstq, strideq
    ret
.v:
    movif64         wq, r4
    movif32        t0m, t0
    movif32        t1m, t1
.v_loop:
%if ARCH_X86_64
    mova            m1, [t4+wq]
    paddw           m1, [t2+wq]
    mova            m2, [t3+wq]
    mova            m4, [t1+wq]
    paddw           m3, m4, [t6+wq]
    paddw           m4, [t5+wq]
%else
    mov             t0, t4m
    mov             t1, t2m
    mova            m1, [t0+wq]
    paddw           m1, [t1+wq]
    mov             t0, t3m
    mov             t1, t1m
    mova            m2, [t0+wq]
    mova            m4, [t1+wq]
    mov             t0, t6m
    mov             t1, t5m
    paddw           m3, m4, [t0+wq]
    paddw           m4, [t1+wq]
%endif
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
    psrad           m0, 6
    psrad           m1, 6
    packssdw        m0, m1
    pmulhw          m0, m11
    pxor            m1, m1
    pmaxsw          m0, m1
    mova     [dstq+wq], m0
    add             wq, 16
    jl .v_loop
%if ARCH_X86_64
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
%else
    mov             t0, t5m
    mov             t1, t4m
    mov             r4, t3m
    mov            t6m, t0
    mov            t5m, t1
    mov            t4m, r4
    mov             r4, t2m
    mov             t1, t1m
    mov             t0, t0m
    mov            t3m, r4
    mov            t2m, t1
%endif
    add           dstq, strideq
    ret

%if ARCH_X86_32
 %if STACK_ALIGNMENT < 16
  %assign stack_size 12*16+384*8
 %else
  %assign stack_size 11*16+384*8
 %endif
cglobal wiener_filter5_16bpc, 4, 7, 8, -stack_size, dst, stride, left, \
                                                    lpf, w, flt
 %if STACK_ALIGNMENT < 16
  %define lpfm        dword [esp+calloff+4*6]
  %define wm          dword [esp+calloff+4*7]
  %define hd          dword [esp+calloff+16*10+0]
  %define edgeb        byte [esp+calloff+16*10+4]
  %define edged       dword [esp+calloff+16*10+4]
 %else
  %define hd dword r5m
  %define edgeb byte r7m
 %endif
 %define PICmem dword [esp+calloff+4*0]
 %define t0m    dword [esp+calloff+4*1] ; wiener ring buffer pointers
 %define t1m    dword [esp+calloff+4*2]
 %define t2m    dword [esp+calloff+4*3]
 %define t3m    dword [esp+calloff+4*4]
 %define t4m    dword [esp+calloff+4*5]
 %define t2 t2m
 %define t3 t3m
 %define t4 t4m
 %define  m8 [esp+calloff+16*2]
 %define  m9 [esp+calloff+16*3]
 %define m10 [esp+calloff+16*4]
 %define m11 [esp+calloff+16*5]
 %define m12 [esp+calloff+16*6]
 %define m13 [esp+calloff+16*7]
 %define m14 [esp+calloff+16*8]
 %define m15 [esp+calloff+16*9]
 %define base t0-wiener_shifts
 %assign calloff 0
 %if STACK_ALIGNMENT < 16
    mov             wd, [rstk+stack_offset+20]
    mov             wm, wd
    mov             r5, [rstk+stack_offset+24]
    mov             hd, r5
    mov             r5, [rstk+stack_offset+32]
    mov          edged, r5 ; edge
 %endif
%else
cglobal wiener_filter5_16bpc, 4, 14, 16, 384*8+16, dst, stride, left, lpf, \
                                                   w, h, edge, flt
 %define base
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    movifnidn       wd, wm
%endif
%if ARCH_X86_64
    mov           fltq, r6mp
    movifnidn       hd, hm
    mov          edged, r7m
    mov            t3d, r8m ; pixel_max
    movq           m12, [fltq]
    movq           m14, [fltq+16]
%else
 %if STACK_ALIGNMENT < 16
    mov             t0, [rstk+stack_offset+28]
    mov             t1, [rstk+stack_offset+36] ; pixel_max
    movq            m1, [t0]    ; fx
    movq            m3, [t0+16] ; fy
    LEA             t0, wiener_shifts
 %else
    mov           fltq, r6m
    movq            m1, [fltq]
    movq            m3, [fltq+16]
    LEA             t0, wiener_shifts
    mov             t1, r8m ; pixel_max
 %endif
    mov         PICmem, t0
%endif
    mova            m5, [base+wiener_shufE]
    mova            m6, [base+wiener_shufB]
    mova            m7, [base+wiener_shufD]
%if ARCH_X86_64
    lea             t4, [wiener_shifts]
    add             wd, wd
    punpcklwd      m11, m12, m12
    pshufd         m11, m11, q1111 ; x1
    pshufd         m12, m12, q1111 ; x2 x3
    punpcklwd      m13, m14, m14
    pshufd         m13, m13, q1111 ; y1
    pshufd         m14, m14, q1111 ; y2 y3
    shr            t3d, 11
    mova            m8, [pd_m262128] ; (1 << 4) - (1 << 18)
    add           lpfq, wq
    lea             t1, [rsp+wq+16]
    add           dstq, wq
    neg             wq
 %define base t4-wiener_shifts
    movd            m9, [base+wiener_round+t3*4]
    movq           m10, [base+wiener_shifts+t3*8]
    pshufd          m9, m9, q0000
    pshufd          m0, m10, q0000
    pshufd         m10, m10, q1111
    mova           m15, [wiener_lshuf5]
    pmullw         m11, m0
    pmullw         m12, m0
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm [rsp]
 %define base
%else
    add             wd, wd
    punpcklwd       m0, m1, m1
    pshufd          m0, m0, q1111 ; x1
    pshufd          m1, m1, q1111 ; x2 x3
    punpcklwd       m2, m3, m3
    pshufd          m2, m2, q1111 ; y1
    pshufd          m3, m3, q1111 ; y2 y3
    mova            m4, [base+pd_m262128] ; (1 << 4) - (1 << 18)
    mova           m13, m2
    mova           m14, m3
    mova            m8, m4
    shr             t1, 11
    add           lpfq, wq
    movd            m2, [base+wiener_round+t1*4]
    movq            m3, [base+wiener_shifts+t1*8]
 %if STACK_ALIGNMENT < 16
    lea             t1, [esp+16*11+wq+16]
 %else
    lea             t1, [esp+16*10+wq+16]
 %endif
    add           dstq, wq
    neg             wq
    pshufd          m2, m2, q0000
    pshufd          m4, m3, q0000
    pshufd          m3, m3, q1111
    mov             wm, wq
    pmullw          m0, m4
    pmullw          m1, m4
    mova            m4, [base+wiener_lshuf5]
    mova            m9, m2
    mova           m10, m3
    mova           m11, m0
    mova           m12, m1
    mova           m15, m4
%endif
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
    mov           lpfm, r10 ; below
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
    mov           lpfq, lpfm
    call .hv_bottom
    add           lpfq, strideq
    call .hv_bottom
.end:
    RET
.no_top:
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov           lpfm, r10
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
%if ARCH_X86_64
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
%else
    mov             t0, t3m
    mov             r4, t2m
    mov             t1, t1m
    mov            t4m, t0
    mov            t3m, r4
    mov            t2m, t1
    mov             wq, wm
%endif
    add           dstq, strideq
.v1:
    call .v
    jmp .end
.extend_right:
%assign stack_offset stack_offset+8
%assign calloff 8
    movif32         t0, PICmem
    pxor            m1, m1
    movd            m2, wd
    mova            m0, [base+pb_2_3]
    pshufb          m2, m1
    mova            m1, [base+pb_m6_m5]
    psubb           m0, m2
    psubb           m1, m2
    mova            m2, [base+pb_0to15]
    pminub          m0, m2
    pminub          m1, m2
    pshufb          m3, m0
    pshufb          m4, m1
    ret
%assign stack_offset stack_offset-4
%assign calloff 4
.h:
    movif64         wq, r4
    movif32         wq, wm
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    mova            m4, [lpfq+wq]
    movd            m3, [leftq+4]
    pslldq          m4, 4
    por             m3, m4
    add          leftq, 8
    jmp .h_main
.h_extend_left:
    mova            m3, [lpfq+wq] ; avoid accessing memory located
    pshufb          m3, m15       ; before the start of the buffer
    jmp .h_main
.h_top:
    movif64         wq, r4
    movif32         wq, wm
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m3, [lpfq+wq-4]
.h_main:
    movu            m4, [lpfq+wq+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             wd, -18
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
    mova       [t1+wq], m0
    add             wq, 16
    jl .h_loop
    movif32         wq, wm
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    movif64         wq, r4
    movif32        t0m, t0
    movif32        t1m, t1
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    mova            m4, [lpfq+wq]
    movd            m3, [leftq+4]
    pslldq          m4, 4
    por             m3, m4
    add          leftq, 8
    jmp .hv_main
.hv_extend_left:
    mova            m3, [lpfq+wq]
    pshufb          m3, m15
    jmp .hv_main
.hv_bottom:
    movif64         wq, r4
    movif32        t0m, t0
    movif32        t1m, t1
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m3, [lpfq+wq-4]
.hv_main:
    movu            m4, [lpfq+wq+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp             wd, -18
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    movif32         t1, t1m
    movif32         t0, t3m
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
%if ARCH_X86_64
    mova            m2, [t3+wq]
    paddw           m2, [t1+wq]
    paddd           m1, m3
    mova            m4, [t2+wq]
%else
    mova            m2, [t0+wq]
    mov             t0, t2m
    paddw           m2, [t1+wq]
    mov             t1, t4m
    paddd           m1, m3
    mova            m4, [t0+wq]
    mov             t0, t0m
%endif
    punpckhwd       m3, m2, m4
    pmaddwd         m3, m14
    punpcklwd       m2, m4
%if ARCH_X86_64
    mova            m4, [t4+wq]
%else
    mova            m4, [t1+wq]
%endif
    psrad           m0, 4
    psrad           m1, 4
    packssdw        m0, m1
    pmaddwd         m2, m14
    psraw           m0, 1
    mova       [t0+wq], m0
    punpckhwd       m1, m0, m4
    pmaddwd         m1, m13
    punpcklwd       m0, m4
    pmaddwd         m0, m13
    paddd           m3, m9
    paddd           m2, m9
    paddd           m1, m3
    paddd           m0, m2
    psrad           m1, 6
    psrad           m0, 6
    packssdw        m0, m1
    pmulhw          m0, m10
    pxor            m1, m1
    pmaxsw          m0, m1
    mova     [dstq+wq], m0
    add             wq, 16
    jl .hv_loop
%if ARCH_X86_64
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, t4
%else
    mov             r4, t3m
    mov             t1, t2m
    mov            t4m, r4
    mov            t3m, t1
    mov             r4, t1m
    mov             t1, t0
    mov            t2m, r4
    mov             t0, t4m
    mov             wq, wm
%endif
    add           dstq, strideq
    ret
.v:
    movif64         wq, r4
    movif32        t1m, t1
.v_loop:
%if ARCH_X86_64
    mova            m0, [t1+wq]
    paddw           m2, m0, [t3+wq]
    mova            m1, [t2+wq]
    mova            m4, [t4+wq]
%else
    mov             t0, t3m
    mova            m0, [t1+wq]
    mov             t1, t2m
    paddw           m2, m0, [t0+wq]
    mov             t0, t4m
    mova            m1, [t1+wq]
    mova            m4, [t0+wq]
%endif
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
    psrad           m1, 6
    psrad           m0, 6
    packssdw        m0, m1
    pmulhw          m0, m10
    pxor            m1, m1
    pmaxsw          m0, m1
    mova     [dstq+wq], m0
    add             wq, 16
%if ARCH_X86_64
    jl .v_loop
%else
    jge .v_end
    mov             t1, t1m
    jmp .v_loop
.v_end:
%endif
    ret

%macro GATHERDD 3 ; dst, src, tmp
    movd           %3d, %2
 %if ARCH_X86_64
    movd            %1, [r13+%3]
    pextrw         %3d, %2, 2
    pinsrw          %1, [r13+%3+2], 3
    pextrw         %3d, %2, 4
    pinsrw          %1, [r13+%3+2], 5
    pextrw         %3d, %2, 6
    pinsrw          %1, [r13+%3+2], 7
 %else
    movd            %1, [base+sgr_x_by_x-0xf03+%3]
    pextrw          %3, %2, 2
    pinsrw          %1, [base+sgr_x_by_x-0xf03+%3+2], 3
    pextrw          %3, %2, 4
    pinsrw          %1, [base+sgr_x_by_x-0xf03+%3+2], 5
    pextrw          %3, %2, 6
    pinsrw          %1, [base+sgr_x_by_x-0xf03+%3+2], 7
 %endif
%endmacro

%macro GATHER_X_BY_X 5 ; dst, src0, src1, tmp32, tmp32_restore
 %if ARCH_X86_64
  %define tmp r14
 %else
  %define tmp %4
 %endif
    GATHERDD        %1, %2, tmp
    GATHERDD        %2, %3, tmp
    movif32         %4, %5
    psrld           %1, 24
    psrld           %2, 24
    packssdw        %1, %2
%endmacro

%macro MAXSD 3-4 0 ; dst, src, restore_tmp
    pcmpgtd         %3, %1, %2
    pand            %1, %3
    pandn           %3, %2
    por             %1, %3
 %if %4 == 1
    pxor            %3, %3
 %endif
%endmacro

%macro MULLD 3 ; dst, src, tmp
    pmulhuw         %3, %1, %2
    pmullw          %1, %2
    pslld           %3, 16
    paddd           %1, %3
%endmacro

%if ARCH_X86_32
DECLARE_REG_TMP 0, 1, 2, 3, 5
 %if STACK_ALIGNMENT < 16
  %assign extra_stack 5*16
 %else
  %assign extra_stack 3*16
 %endif
cglobal sgr_filter_5x5_16bpc, 1, 7, 8, -400*24-16-extra_stack, \
                              dst, stride, left, lpf, w
 %if STACK_ALIGNMENT < 16
  %define dstm         dword [esp+calloff+16*0+4*6]
  %define stridemp     dword [esp+calloff+16*0+4*7]
  %define leftm        dword [esp+calloff+16*3+4*0]
  %define lpfm         dword [esp+calloff+16*3+4*1]
  %define w0m          dword [esp+calloff+16*3+4*2]
  %define hd           dword [esp+calloff+16*3+4*3]
  %define edgeb         byte [esp+calloff+16*3+4*4]
  %define edged        dword [esp+calloff+16*3+4*4]
  %define leftmp leftm
 %else
  %define w0m wm
  %define hd dword r5m
  %define edgeb  byte r7m
  %define edged dword r7m
 %endif
 %define hvsrcm dword [esp+calloff+4*0]
 %define w1m    dword [esp+calloff+4*1]
 %define t0m    dword [esp+calloff+4*2]
 %define t2m    dword [esp+calloff+4*3]
 %define t3m    dword [esp+calloff+4*4]
 %define t4m    dword [esp+calloff+4*5]
 %define  m8 [base+pd_8]
 %define  m9 [base+pd_0xfffffff0]
 %define m10 [esp+calloff+16*2]
 %define m11 [base+pd_0xf00800a4]
 %define m12 [base+sgr_lshuf5]
 %define m13 [base+pd_34816]
 %define m14 [base+pw_1023]
 %define r10 r4
 %define base r6-$$
 %assign calloff 0
 %if STACK_ALIGNMENT < 16
    mov        strideq, [rstk+stack_offset+ 8]
    mov          leftq, [rstk+stack_offset+12]
    mov           lpfq, [rstk+stack_offset+16]
    mov             wd, [rstk+stack_offset+20]
    mov           dstm, dstq
    mov       stridemp, strideq
    mov          leftm, leftq
    mov             r1, [rstk+stack_offset+24]
    mov             r2, [rstk+stack_offset+32]
    mov           lpfm, lpfq
    mov             hd, r1
    mov          edged, r2
 %endif
%else
cglobal sgr_filter_5x5_16bpc, 4, 15, 15, -400*24-16, dst, stride, left, lpf, \
                                                     w, h, edge, params
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    movifnidn       wd, wm
%endif
%if ARCH_X86_64
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x-0xf03]
    movifnidn       hd, hm
    add             wd, wd
    mov          edged, r7m
    movu           m10, [paramsq]
    mova           m12, [sgr_lshuf5]
    add           lpfq, wq
    mova            m8, [pd_8]
    lea             t1, [rsp+wq+20]
    mova            m9, [pd_0xfffffff0]
    add           dstq, wq
    lea             t3, [rsp+wq*2+400*12+16]
    mova           m11, [pd_0xf00800a4]
    lea             t4, [rsp+wq+400*20+16]
    pshufhw         m7, m10, q0000
    pshufb         m10, [pw_256]  ; s0
    punpckhqdq      m7, m7        ; w0
    neg             wq
    mova           m13, [pd_34816]  ; (1 << 11) + (1 << 15)
    pxor            m6, m6
    mova           m14, [pw_1023]
    psllw           m7, 4
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm        [rsp]
%else
    mov             r1, [rstk+stack_offset+28] ; params
    LEA             r6, $$
    add             wd, wd
    movu            m1, [r1]
    add           lpfm, wq
    lea             t1, [rsp+extra_stack+wq+20]
    add           dstq, wq
    lea             t3, [rsp+extra_stack+wq*2+400*12+16]
    mov           dstm, dstq
    lea             t4, [rsp+extra_stack+wq+400*20+16]
    mov            t3m, t3
    pshufhw         m7, m1, q0000
    mov            t4m, t4
    pshufb          m1, [base+pw_256] ; s0
    punpckhqdq      m7, m7            ; w0
    psllw           m7, 4
    neg             wq
    mova           m10, m1
    pxor            m6, m6
    mov            w1m, wd
    sub             wd, 4
    mov           lpfq, lpfm
    mov            w0m, wd
 %define strideq r5
%endif
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, stridemp
    movif32        t2m, t1
    mov             t2, t1
    call .top_fixup
    add             t1, 400*6
    call .h_top
    movif32    strideq, stridemp
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    add            r10, strideq
    mov           lpfm, r10 ; below
    movif32        t0m, t2
    mov             t0, t2
    dec             hd
    jz .height1
    or           edged, 16
    call .h
.main:
    add           lpfq, stridemp
    movif32         t4, t4m
    call .hv
    call .prep_n
    sub             hd, 2
    jl .extend_bottom
.main_loop:
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
%if ARCH_X86_64
    test            hb, hb
%else
    mov             r4, hd
    test            r4, r4
%endif
    jz .odd_height
    call .h
    add           lpfq, stridemp
    call .hv
    movif32       dstq, dstm
    call .n0
    call .n1
    sub             hd, 2
    movif32         t0, t0m
    jge .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .extend_bottom
    mov           lpfq, lpfm
    call .h_top
    add           lpfq, stridemp
    call .hv_bottom
.end:
    movif32       dstq, dstm
    call .n0
    call .n1
.end2:
    RET
.height1:
    movif32         t4, t4m
    call .hv
    call .prep_n
    jmp .odd_height_end
.odd_height:
    call .hv
    movif32       dstq, dstm
    call .n0
    call .n1
.odd_height_end:
    call .v
    movif32       dstq, dstm
    call .n0
    jmp .end2
.extend_bottom:
    call .v
    jmp .end
.no_top:
    movif32    strideq, stridemp
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov           lpfm, r10
    call .h
    lea             t2, [t1+400*6]
    movif32        t2m, t2
    call .top_fixup
    dec             hd
    jz .no_top_height1
    or           edged, 16
    mov             t0, t1
    mov             t1, t2
    movif32        t0m, t0
    jmp .main
.no_top_height1:
    movif32         t3, t3m
    movif32         t4, t4m
    call .v
    call .prep_n
    jmp .odd_height_end
.extend_right:
    movd            m0, wd
    movd            m1, [lpfq-2]
    mova            m2, [base+pw_256]
    mova            m3, [base+pb_m14_m13]
    pshufb          m0, m6
    pshufb          m1, m2
    psubb           m2, m0
    psubb           m3, m0
    mova            m0, [base+pb_0to15]
    pcmpgtb         m2, m0
    pcmpgtb         m3, m0
    pand            m4, m2
    pand            m5, m3
    pandn           m2, m1
    pandn           m3, m1
    por             m4, m2
    por             m5, m3
    ret
%assign stack_offset stack_offset+4
%assign calloff 4
.h: ; horizontal boxsum
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
 %define leftq r4
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 10
    jmp .h_main
.h_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, m12
    jmp .h_main
.h_top:
%if ARCH_X86_64
    lea             wq, [r4-4]
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32         wq, w0m
.h_loop:
    movu            m4, [lpfq+wq- 2]
.h_main:
    movu            m5, [lpfq+wq+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             wd, -20
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
    palignr         m5, m4, 8
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
    paddw           m0, [t1+wq+400*0]
    paddd           m1, [t1+wq+400*2]
    paddd           m2, [t1+wq+400*4]
.h_loop_end:
    paddd           m1, m5             ; sumsq
    paddd           m2, m4
    mova [t1+wq+400*0], m0
    mova [t1+wq+400*2], m1
    mova [t1+wq+400*4], m2
    add             wq, 16
    jl .h_loop
    ret
.top_fixup:
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wd, w0m
%endif
.top_fixup_loop: ; the sums of the first row needs to be doubled
    mova            m0, [t1+wq+400*0]
    mova            m1, [t1+wq+400*2]
    mova            m2, [t1+wq+400*4]
    paddw           m0, m0
    paddd           m1, m1
    paddd           m2, m2
    mova [t2+wq+400*0], m0
    mova [t2+wq+400*2], m1
    mova [t2+wq+400*4], m2
    add             wq, 16
    jl .top_fixup_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 10
    jmp .hv_main
.hv_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, m12
    jmp .hv_main
.hv_bottom:
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movif32         wq, w0m
%if ARCH_X86_32
    jmp .hv_loop_start
%endif
.hv_loop:
    movif32       lpfq, hvsrcm
.hv_loop_start:
    movu            m4, [lpfq+wq- 2]
.hv_main:
    movu            m5, [lpfq+wq+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp             wd, -20
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    movif32         t3, hd
    palignr         m3, m5, m4, 2
    paddw           m0, m4, m3
    palignr         m1, m5, m4, 6
    paddw           m0, m1
    punpcklwd       m2, m3, m1
    pmaddwd         m2, m2
    punpckhwd       m3, m1
    pmaddwd         m3, m3
    palignr         m5, m4, 8
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
    paddw           m1, m0, [t1+wq+400*0]
    paddd           m4, m2, [t1+wq+400*2]
    paddd           m5, m3, [t1+wq+400*4]
%if ARCH_X86_64
    test            hd, hd
%else
    test            t3, t3
%endif
    jz .hv_last_row
.hv_main2:
    paddw           m1, [t2+wq+400*0] ; hv sum
    paddd           m4, [t2+wq+400*2] ; hv sumsq
    paddd           m5, [t2+wq+400*4]
    mova [t0+wq+400*0], m0
    mova [t0+wq+400*2], m2
    mova [t0+wq+400*4], m3
    psrlw           m3, m1, 1
    paddd           m4, m8
    pavgw           m3, m6             ; (b + 2) >> 2
    paddd           m5, m8
    pand            m4, m9             ; ((a + 8) >> 4) << 4
    pand            m5, m9
    psrld           m2, m4, 4
    psrld           m0, m5, 4
    paddd           m2, m4
    psrld           m4, 1
    paddd           m0, m5
    psrld           m5, 1
    paddd           m4, m2             ; a * 25
    paddd           m5, m0
    punpcklwd       m2, m3, m6
    punpckhwd       m3, m6
    pmaddwd         m2, m2             ; b * b
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    MAXSD           m4, m2, m6
    MAXSD           m5, m3, m6, 1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m10, m2        ; p * s
    MULLD           m5, m10, m2
    pmaddwd         m0, m11            ; b * 164
    pmaddwd         m1, m11
    paddusw         m4, m11
    paddusw         m5, m11
    psrld           m4, 20             ; min(z, 255)
    movif32         t3, t3m
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, t2, t2m
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m2
    MULLD           m1, m5, m2
    paddd           m0, m13            ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m13
    mova     [t4+wq+4], m3
    psrld           m0, 12             ; b
    psrld           m1, 12
    mova  [t3+wq*2+ 8], m0
    mova  [t3+wq*2+24], m1
    add             wq, 16
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    movif32        t2m, t2
    movif32        t0m, t0
    ret
.hv_last_row: ; esoteric edge case for odd heights
    mova [t1+wq+400*0], m1
    paddw           m1, m0
    mova [t1+wq+400*2], m4
    paddd           m4, m2
    mova [t1+wq+400*4], m5
    paddd           m5, m3
    jmp .hv_main2
.v: ; vertical boxsum + ab
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wd, w0m
%endif
.v_loop:
    mova            m0, [t1+wq+400*0]
    mova            m2, [t1+wq+400*2]
    mova            m3, [t1+wq+400*4]
    paddw           m1, m0, [t2+wq+400*0]
    paddd           m4, m2, [t2+wq+400*2]
    paddd           m5, m3, [t2+wq+400*4]
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
    pand            m4, m9             ; ((a + 8) >> 4) << 4
    pand            m5, m9
    psrld           m2, m4, 4
    psrld           m0, m5, 4
    paddd           m2, m4
    psrld           m4, 1
    paddd           m0, m5
    psrld           m5, 1
    paddd           m4, m2             ; a * 25
    paddd           m5, m0
    punpcklwd       m2, m3, m6
    punpckhwd       m3, m6
    pmaddwd         m2, m2             ; b * b
    pmaddwd         m3, m3
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    MAXSD           m4, m2, m6
    MAXSD           m5, m3, m6, 1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m10, m2        ; p * s
    MULLD           m5, m10, m2
    pmaddwd         m0, m11            ; b * 164
    pmaddwd         m1, m11
    paddusw         m4, m11
    paddusw         m5, m11
    psrld           m4, 20             ; min(z, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, t2, t2m
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m2
    MULLD           m1, m5, m2
    paddd           m0, m13            ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m13
    mova     [t4+wq+4], m3
    psrld           m0, 12             ; b
    psrld           m1, 12
    mova  [t3+wq*2+ 8], m0
    mova  [t3+wq*2+24], m1
    add             wq, 16
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    movif64         wq, r4
    movif32         wd, w1m
.prep_n_loop:
    movu            m0, [t4+wq*1+ 2]
    movu            m3, [t4+wq*1+ 4]
    movu            m1, [t3+wq*2+ 4]
    movu            m4, [t3+wq*2+ 8]
    movu            m2, [t3+wq*2+20]
    movu            m5, [t3+wq*2+24]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    paddw           m3, [t4+wq*1+ 0]
    paddd           m4, [t3+wq*2+ 0]
    paddd           m5, [t3+wq*2+16]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m4
    pslld           m4, 2
    paddd           m2, m5
    pslld           m5, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m4             ; b 565
    paddd           m2, m5
    mova [t4+wq*1+400*2+ 0], m0
    mova [t3+wq*2+400*4+ 0], m1
    mova [t3+wq*2+400*4+16], m2
    add             wq, 16
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    movif64         wq, r4
    movif32         wd, w1m
.n0_loop:
    movu            m0, [t4+wq*1+ 2]
    movu            m3, [t4+wq*1+ 4]
    movu            m1, [t3+wq*2+ 4]
    movu            m4, [t3+wq*2+ 8]
    movu            m2, [t3+wq*2+20]
    movu            m5, [t3+wq*2+24]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    paddw           m3, [t4+wq*1+ 0]
    paddd           m4, [t3+wq*2+ 0]
    paddd           m5, [t3+wq*2+16]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m4
    pslld           m4, 2
    paddd           m2, m5
    pslld           m5, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m4             ; b 565
    paddd           m2, m5
    paddw           m3, m0, [t4+wq*1+400*2+ 0]
    paddd           m4, m1, [t3+wq*2+400*4+ 0]
    paddd           m5, m2, [t3+wq*2+400*4+16]
    mova [t4+wq*1+400*2+ 0], m0
    mova [t3+wq*2+400*4+ 0], m1
    mova [t3+wq*2+400*4+16], m2
    mova            m0, [dstq+wq]
    punpcklwd       m1, m0, m6          ; src
    punpcklwd       m2, m3, m6          ; a
    pmaddwd         m2, m1              ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    psubd           m4, m2              ; b - a * src + (1 << 8)
    psubd           m5, m3
    psrad           m4, 9
    psrad           m5, 9
    packssdw        m4, m5
    pmulhrsw        m4, m7
    paddw           m0, m4
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova     [dstq+wq], m0
    add             wq, 16
    jl .n0_loop
    add           dstq, stridemp
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    movif64         wq, r4
    movif32         wd, w1m
.n1_loop:
    mova            m0, [dstq+wq]
    mova            m3, [t4+wq*1+400*2+ 0]
    mova            m4, [t3+wq*2+400*4+ 0]
    mova            m5, [t3+wq*2+400*4+16]
    punpcklwd       m1, m0, m6          ; src
    punpcklwd       m2, m3, m6          ; a
    pmaddwd         m2, m1
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    psubd           m4, m2              ; b - a * src + (1 << 7)
    psubd           m5, m3
    psrad           m4, 8
    psrad           m5, 8
    packssdw        m4, m5
    pmulhrsw        m4, m7
    paddw           m0, m4
    pmaxsw          m0, m6
    pminsw          m0, m14
    mova     [dstq+wq], m0
    add             wq, 16
    jl .n1_loop
    add           dstq, stridemp
    movif32       dstm, dstq
    ret

%if ARCH_X86_32
 %if STACK_ALIGNMENT < 16
  %assign extra_stack 4*16
 %else
  %assign extra_stack 2*16
 %endif
cglobal sgr_filter_3x3_16bpc, 1, 7, 8, -400*42-16-extra_stack, \
                              dst, stride, left, lpf, w
 %if STACK_ALIGNMENT < 16
  %define dstm         dword [esp+calloff+16*2+4*0]
  %define stridemp     dword [esp+calloff+16*2+4*1]
  %define leftm        dword [esp+calloff+16*2+4*2]
  %define lpfm         dword [esp+calloff+16*2+4*3]
  %define w0m          dword [esp+calloff+16*2+4*4]
  %define hd           dword [esp+calloff+16*2+4*5]
  %define edgeb         byte [esp+calloff+16*2+4*6]
  %define edged        dword [esp+calloff+16*2+4*6]
  %define leftmp leftm
 %else
  %define w0m wm
  %define hd dword r5m
  %define edgeb  byte r7m
  %define edged dword r7m
 %endif
 %define hvsrcm dword [esp+calloff+4*0]
 %define w1m    dword [esp+calloff+4*1]
 %define t3m    dword [esp+calloff+4*2]
 %define t4m    dword [esp+calloff+4*3]
 %define  m8 [base+pd_8]
 %define  m9 [esp+calloff+16*1]
 %define m10 [base+pd_0xf00801c7]
 %define m11 [base+pd_34816]
 %define m12 [base+sgr_lshuf3]
 %define m13 [base+pw_1023]
 %define m14 m6
 %define base r6-$$
 %assign calloff 0
 %if STACK_ALIGNMENT < 16
    mov        strideq, [rstk+stack_offset+ 8]
    mov          leftq, [rstk+stack_offset+12]
    mov           lpfq, [rstk+stack_offset+16]
    mov             wd, [rstk+stack_offset+20]
    mov           dstm, dstq
    mov       stridemp, strideq
    mov          leftm, leftq
    mov             r1, [rstk+stack_offset+24]
    mov             r2, [rstk+stack_offset+32]
    mov           lpfm, lpfq
    mov             hd, r1
    mov          edged, r2
 %endif
%else
cglobal sgr_filter_3x3_16bpc, 4, 15, 15, -400*42-8, dst, stride, left, lpf, \
                                                    w, h, edge, params
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    movifnidn       wd, wm
%endif
%if ARCH_X86_64
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x-0xf03]
    movifnidn       hd, hm
    add             wd, wd
    mov          edged, r7m
    movq            m9, [paramsq+4]
    add           lpfq, wq
    lea             t1, [rsp+wq+12]
    mova            m8, [pd_8]
    add           dstq, wq
    lea             t3, [rsp+wq*2+400*12+8]
    mova           m10, [pd_0xf00801c7]
    lea             t4, [rsp+wq+400*32+8]
    mova           m11, [pd_34816]
    pshuflw         m7, m9, q3333
    pshufb          m9, [pw_256]  ; s1
    punpcklqdq      m7, m7        ; w1
    neg             wq
    pxor            m6, m6
    mova           m13, [pw_1023]
    psllw           m7, 4
    mova           m12, [sgr_lshuf3]
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm [rsp]
%else
    mov             r1, [rstk+stack_offset+28] ; params
    LEA             r6, $$
    add             wd, wd
    movq            m1, [r1+4]
    add           lpfm, wq
    lea             t1, [rsp+extra_stack+wq+20]
    add           dstq, wq
    lea             t3, [rsp+extra_stack+wq*2+400*12+16]
    mov           dstm, dstq
    lea             t4, [rsp+extra_stack+wq+400*32+16]
    mov            t3m, t3
    pshuflw         m7, m1, q3333
    mov            t4m, t4
    pshufb          m1, [base+pw_256] ; s1
    punpcklqdq      m7, m7            ; w1
    psllw           m7, 4
    neg             wq
    mova            m9, m1
    pxor            m6, m6
    mov            w1m, wd
    sub             wd, 4
    mov           lpfq, lpfm
    mov            w0m, wd
 %define strideq r5
%endif
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, stridemp
    mov             t2, t1
    add             t1, 400*6
    call .h_top
    movif32    strideq, stridemp
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    add            r10, strideq
    mov           lpfm, r10 ; below
    movif32         t4, t4m
    call .hv0
.main:
    dec             hd
    jz .height1
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
    call .hv1
    call .prep_n
    sub             hd, 2
    jl .extend_bottom
.main_loop:
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
    call .hv0
%if ARCH_X86_64
    test            hb, hb
%else
    mov             r4, hd
    test            r4, r4
%endif
    jz .odd_height
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
    call .hv1
    call .n0
    call .n1
    sub             hd, 2
    jge .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .extend_bottom
    mov           lpfq, lpfm
    call .hv0_bottom
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
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
    movif32    strideq, stridemp
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov           lpfm, r10
    call .h
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wq, w0m
    mov         hvsrcm, lpfq
%endif
    lea             t2, [t1+400*6]
.top_fixup_loop:
    mova            m0, [t1+wq+400*0]
    mova            m1, [t1+wq+400*2]
    mova            m2, [t1+wq+400*4]
    mova [t2+wq+400*0], m0
    mova [t2+wq+400*2], m1
    mova [t2+wq+400*4], m2
    add             wq, 16
    jl .top_fixup_loop
    movif32         t3, t3m
    movif32         t4, t4m
    call .v0
    jmp .main
.extend_right:
    movd            m1, wd
    movd            m5, [lpfq-2]
    mova            m2, [base+pw_256]
    mova            m3, [base+pb_0to15]
    pshufb          m1, m6
    pshufb          m5, m2
    psubb           m2, m1
    pcmpgtb         m2, m3
    pand            m4, m2
    pandn           m2, m5
    por             m4, m2
    ret
%assign stack_offset stack_offset+4
%assign calloff 4
.h: ; horizontal boxsum
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
 %define leftq r4
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 12
    jmp .h_main
.h_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, m12
    jmp .h_main
.h_top:
%if ARCH_X86_64
    lea             wq, [r4-4]
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32         wq, w0m
.h_loop:
    movu            m4, [lpfq+wq+ 0]
.h_main:
    movu            m5, [lpfq+wq+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             wd, -18
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
    mova [t1+wq+400*0], m1
    mova [t1+wq+400*2], m2
    mova [t1+wq+400*4], m3
    add             wq, 16
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab (even rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 12
    jmp .hv0_main
.hv0_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, m12
    jmp .hv0_main
.hv0_bottom:
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movif32         wq, w0m
%if ARCH_X86_32
    jmp .hv0_loop_start
%endif
.hv0_loop:
    movif32       lpfq, hvsrcm
.hv0_loop_start:
    movu            m4, [lpfq+wq+ 0]
.hv0_main:
    movu            m5, [lpfq+wq+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp             wd, -18
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
    paddw           m0, m1, [t1+wq+400*0]
    paddd           m4, m2, [t1+wq+400*2]
    paddd           m5, m3, [t1+wq+400*4]
    mova [t1+wq+400*0], m1
    mova [t1+wq+400*2], m2
    mova [t1+wq+400*4], m3
    paddw           m1, m0, [t2+wq+400*0]
    paddd           m2, m4, [t2+wq+400*2]
    paddd           m3, m5, [t2+wq+400*4]
    mova [t2+wq+400*0], m0
    mova [t2+wq+400*2], m4
    mova [t2+wq+400*4], m5
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
    MAXSD           m4, m2, m14
    MAXSD           m5, m3, m14
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m14        ; p * s
    MULLD           m5, m9, m14
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    movif32         t3, t3m
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m14
    MULLD           m1, m5, m14
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova     [t4+wq+4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova  [t3+wq*2+ 8], m0
    mova  [t3+wq*2+24], m1
    add             wq, 16
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 12
    jmp .hv1_main
.hv1_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, m12
    jmp .hv1_main
.hv1_bottom:
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movif32         wq, w0m
%if ARCH_X86_32
    jmp .hv1_loop_start
%endif
.hv1_loop:
    movif32       lpfq, hvsrcm
.hv1_loop_start:
    movu            m4, [lpfq+wq+ 0]
.hv1_main:
    movu            m5, [lpfq+wq+16]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp             wd, -18
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
    paddw           m1, m0, [t2+wq+400*0]
    paddd           m4, m2, [t2+wq+400*2]
    paddd           m5, m3, [t2+wq+400*4]
    mova [t2+wq+400*0], m0
    mova [t2+wq+400*2], m2
    mova [t2+wq+400*4], m3
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
    MAXSD           m4, m2, m14
    MAXSD           m5, m3, m14
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m14        ; p * s
    MULLD           m5, m9, m14
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    movif32         t3, t3m
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m14
    MULLD           m1, m5, m14
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova [t4+wq*1+400*2 +4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*2+400*4+ 8], m0
    mova [t3+wq*2+400*4+24], m1
    add             wq, 16
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab (even rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wd, w0m
%endif
.v0_loop:
    mova            m0, [t1+wq+400*0]
    mova            m4, [t1+wq+400*2]
    mova            m5, [t1+wq+400*4]
    paddw           m0, m0
    paddd           m4, m4
    paddd           m5, m5
    paddw           m1, m0, [t2+wq+400*0]
    paddd           m2, m4, [t2+wq+400*2]
    paddd           m3, m5, [t2+wq+400*4]
    mova [t2+wq+400*0], m0
    mova [t2+wq+400*2], m4
    mova [t2+wq+400*4], m5
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
    MAXSD           m4, m2, m14
    MAXSD           m5, m3, m14
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m14        ; p * s
    MULLD           m5, m9, m14
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m14
    MULLD           m1, m5, m14
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova [t4+wq*1+400*0+ 4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*2+400*0+ 8], m0
    mova [t3+wq*2+400*0+24], m1
    add             wq, 16
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wd, w0m
%endif
.v1_loop:
    mova            m0, [t1+wq+400*0]
    mova            m4, [t1+wq+400*2]
    mova            m5, [t1+wq+400*4]
    paddw           m1, m0, [t2+wq+400*0]
    paddd           m2, m4, [t2+wq+400*2]
    paddd           m3, m5, [t2+wq+400*4]
    mova [t2+wq+400*0], m0
    mova [t2+wq+400*2], m4
    mova [t2+wq+400*4], m5
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
    MAXSD           m4, m2, m14
    MAXSD           m5, m3, m14
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m14        ; p * s
    MULLD           m5, m9, m14
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m14
    MULLD           m1, m5, m14
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova [t4+wq*1+400*2+ 4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*2+400*4+ 8], m0
    mova [t3+wq*2+400*4+24], m1
    add             wq, 16
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    movif64         wq, r4
    movif32         wd, w1m
.prep_n_loop:
    movu            m0, [t4+wq*1+400*0+ 4]
    movu            m1, [t3+wq*2+400*0+ 8]
    movu            m2, [t3+wq*2+400*0+24]
    movu            m3, [t4+wq*1+400*0+ 2]
    movu            m4, [t3+wq*2+400*0+ 4]
    movu            m5, [t3+wq*2+400*0+20]
    paddw           m0, [t4+wq*1+400*0+ 0]
    paddd           m1, [t3+wq*2+400*0+ 0]
    paddd           m2, [t3+wq*2+400*0+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                ; a[-1] 444
    pslld           m4, 2                ; b[-1] 444
    pslld           m5, 2
    psubw           m3, m0               ; a[-1] 343
    psubd           m4, m1               ; b[-1] 343
    psubd           m5, m2
    mova [t4+wq*1+400*4], m3
    mova [t3+wq*2+400*8+ 0], m4
    mova [t3+wq*2+400*8+16], m5
    movu            m0, [t4+wq*1+400*2+ 4]
    movu            m1, [t3+wq*2+400*4+ 8]
    movu            m2, [t3+wq*2+400*4+24]
    movu            m3, [t4+wq*1+400*2+ 2]
    movu            m4, [t3+wq*2+400*4+ 4]
    movu            m5, [t3+wq*2+400*4+20]
    paddw           m0, [t4+wq*1+400*2+ 0]
    paddd           m1, [t3+wq*2+400*4+ 0]
    paddd           m2, [t3+wq*2+400*4+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                 ; a[ 0] 444
    pslld           m4, 2                 ; b[ 0] 444
    pslld           m5, 2
    mova [t4+wq*1+400* 6], m3
    mova [t3+wq*2+400*12+ 0], m4
    mova [t3+wq*2+400*12+16], m5
    psubw           m3, m0                ; a[ 0] 343
    psubd           m4, m1                ; b[ 0] 343
    psubd           m5, m2
    mova [t4+wq*1+400* 8], m3
    mova [t3+wq*2+400*16+ 0], m4
    mova [t3+wq*2+400*16+16], m5
    add             wq, 16
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    movif64         wq, r4
    movif32         wd, w1m
.n0_loop:
    movu            m3, [t4+wq*1+400*0+4]
    movu            m1, [t4+wq*1+400*0+2]
    paddw           m3, [t4+wq*1+400*0+0]
    paddw           m1, m3
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+wq*1+400*4]
    paddw           m3, [t4+wq*1+400*6]
    mova [t4+wq*1+400*4], m2
    mova [t4+wq*1+400*6], m1
    movu            m4, [t3+wq*2+400*0+8]
    movu            m1, [t3+wq*2+400*0+4]
    paddd           m4, [t3+wq*2+400*0+0]
    paddd           m1, m4
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m4           ; b[ 1] 343
    paddd           m4, m2, [t3+wq*2+400* 8+ 0]
    paddd           m4, [t3+wq*2+400*12+ 0]
    mova [t3+wq*2+400* 8+ 0], m2
    mova [t3+wq*2+400*12+ 0], m1
    movu            m5, [t3+wq*2+400*0+24]
    movu            m1, [t3+wq*2+400*0+20]
    paddd           m5, [t3+wq*2+400*0+16]
    paddd           m1, m5
    pslld           m1, 2
    psubd           m2, m1, m5
    paddd           m5, m2, [t3+wq*2+400* 8+16]
    paddd           m5, [t3+wq*2+400*12+16]
    mova [t3+wq*2+400* 8+16], m2
    mova [t3+wq*2+400*12+16], m1
    mova            m0, [dstq+wq]
    punpcklwd       m1, m0, m6
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m1               ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    psubd           m4, m2               ; b - a * src + (1 << 8)
    psubd           m5, m3
    psrad           m4, 9
    psrad           m5, 9
    packssdw        m4, m5
    pmulhrsw        m4, m7
    paddw           m0, m4
    pmaxsw          m0, m6
    pminsw          m0, m13
    mova     [dstq+wq], m0
    add             wq, 16
    jl .n0_loop
    add           dstq, stridemp
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    movif64         wq, r4
    movif32         wd, w1m
.n1_loop:
    movu            m3, [t4+wq*1+400*2+4]
    movu            m1, [t4+wq*1+400*2+2]
    paddw           m3, [t4+wq*1+400*2+0]
    paddw           m1, m3
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+wq*1+400*6]
    paddw           m3, [t4+wq*1+400*8]
    mova [t4+wq*1+400*6], m1
    mova [t4+wq*1+400*8], m2
    movu            m4, [t3+wq*2+400*4+8]
    movu            m1, [t3+wq*2+400*4+4]
    paddd           m4, [t3+wq*2+400*4+0]
    paddd           m1, m4
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m4           ; b[ 1] 343
    paddd           m4, m2, [t3+wq*2+400*12+ 0]
    paddd           m4, [t3+wq*2+400*16+ 0]
    mova [t3+wq*2+400*12+ 0], m1
    mova [t3+wq*2+400*16+ 0], m2
    movu            m5, [t3+wq*2+400*4+24]
    movu            m1, [t3+wq*2+400*4+20]
    paddd           m5, [t3+wq*2+400*4+16]
    paddd           m1, m5
    pslld           m1, 2
    psubd           m2, m1, m5
    paddd           m5, m2, [t3+wq*2+400*12+16]
    paddd           m5, [t3+wq*2+400*16+16]
    mova [t3+wq*2+400*12+16], m1
    mova [t3+wq*2+400*16+16], m2
    mova            m0, [dstq+wq]
    punpcklwd       m1, m0, m6
    punpcklwd       m2, m3, m6
    pmaddwd         m2, m1               ; a * src
    punpckhwd       m1, m0, m6
    punpckhwd       m3, m6
    pmaddwd         m3, m1
    psubd           m4, m2               ; b - a * src + (1 << 8)
    psubd           m5, m3
    psrad           m4, 9
    psrad           m5, 9
    packssdw        m4, m5
    pmulhrsw        m4, m7
    paddw           m0, m4
    pmaxsw          m0, m6
    pminsw          m0, m13
    mova     [dstq+wq], m0
    add             wq, 16
    jl .n1_loop
    add           dstq, stridemp
    movif32       dstm, dstq
    ret

%if ARCH_X86_32
 %if STACK_ALIGNMENT < 16
  %assign extra_stack 10*16
 %else
  %assign extra_stack 8*16
 %endif
cglobal sgr_filter_mix_16bpc, 1, 7, 8, -400*66-48-extra_stack, \
                              dst, stride, left, lpf, w
 %if STACK_ALIGNMENT < 16
  %define dstm         dword [esp+calloff+16*8+4*0]
  %define stridemp     dword [esp+calloff+16*8+4*1]
  %define leftm        dword [esp+calloff+16*8+4*2]
  %define lpfm         dword [esp+calloff+16*8+4*3]
  %define w0m          dword [esp+calloff+16*8+4*4]
  %define hd           dword [esp+calloff+16*8+4*5]
  %define edgeb         byte [esp+calloff+16*8+4*6]
  %define edged        dword [esp+calloff+16*8+4*6]
  %define leftmp leftm
 %else
  %define w0m wm
  %define hd dword r5m
  %define edgeb  byte r7m
  %define edged dword r7m
 %endif
 %define hvsrcm dword [esp+calloff+4*0]
 %define w1m    dword [esp+calloff+4*1]
 %define t3m    dword [esp+calloff+4*2]
 %define t4m    dword [esp+calloff+4*3]
 %xdefine m8 m6
 %define  m9 [base+pd_8]
 %define m10 [base+pd_34816]
 %define m11 [base+pd_0xf00801c7]
 %define m12 [base+pd_0xf00800a4]
 %define m13 [esp+calloff+16*4]
 %define m14 [esp+calloff+16*5]
 %define m15 [esp+calloff+16*6]
 %define  m6 [esp+calloff+16*7]
 %define base r6-$$
 %assign calloff 0
 %if STACK_ALIGNMENT < 16
    mov        strideq, [rstk+stack_offset+ 8]
    mov          leftq, [rstk+stack_offset+12]
    mov           lpfq, [rstk+stack_offset+16]
    mov             wd, [rstk+stack_offset+20]
    mov           dstm, dstq
    mov       stridemp, strideq
    mov          leftm, leftq
    mov             r1, [rstk+stack_offset+24]
    mov             r2, [rstk+stack_offset+32]
    mov           lpfm, lpfq
    mov             hd, r1
    mov          edged, r2
 %endif
%else
cglobal sgr_filter_mix_16bpc, 4, 15, 16, -400*66-40, dst, stride, left, lpf, \
                                                     w, h, edge, params
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    movifnidn       wd, wm
%endif
%if ARCH_X86_64
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x-0xf03]
    movifnidn       hd, hm
    add             wd, wd
    mov          edged, r7m
    mova           m14, [paramsq]
    add           lpfq, wq
    mova            m9, [pd_8]
    lea             t1, [rsp+wq+44]
    mova           m10, [pd_34816]
    add           dstq, wq
    mova           m11, [pd_0xf00801c7]
    lea             t3, [rsp+wq*2+400*24+40]
    mova           m12, [pd_0xf00800a4]
    lea             t4, [rsp+wq+400*52+40]
    neg             wq
    pshufd         m15, m14, q2222 ; w0 w1
    punpcklwd      m14, m14
    pshufd         m13, m14, q0000 ; s0
    pshufd         m14, m14, q2222 ; s1
    pxor            m6, m6
    psllw          m15, 2
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm [rsp]
%else
    mov             r1, [rstk+stack_offset+28] ; params
    LEA             r6, $$
    add             wd, wd
    mova            m2, [r1]
    add           lpfm, wq
    lea             t1, [rsp+extra_stack+wq+52]
    add           dstq, wq
    lea             t3, [rsp+extra_stack+wq*2+400*24+48]
    mov           dstm, dstq
    lea             t4, [rsp+extra_stack+wq+400*52+48]
    mov            t3m, t3
    mov            t4m, t4
    neg             wq
    pshuflw         m0, m2, q0000
    pshuflw         m1, m2, q2222
    pshufhw         m2, m2, q1010
    punpcklqdq      m0, m0 ; s0
    punpcklqdq      m1, m1 ; s1
    punpckhqdq      m2, m2 ; w0 w1
    mov            w1m, wd
    pxor            m3, m3
    psllw           m2, 2
    mova           m13, m0
    mova           m14, m1
    sub             wd, 4
    mova           m15, m2
    mova            m6, m3
    mov           lpfq, lpfm
    mov            w0m, wd
 %define strideq r5
%endif
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, stridemp
    mov             t2, t1
%if ARCH_X86_64
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_ssse3).top_fixup
%else
    mov             wq, w0m
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_ssse3).top_fixup_loop
%endif
    add             t1, 400*12
    call .h_top
    movif32    strideq, stridemp
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    add            r10, strideq
    mov           lpfm, r10 ; below
    movif32         t4, t4m
    call .hv0
.main:
    dec             hd
    jz .height1
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
    call .hv1
    call .prep_n
    sub             hd, 2
    jl .extend_bottom
.main_loop:
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
    call .hv0
%if ARCH_X86_64
    test            hd, hd
%else
    mov             r4, hd
    test            r4, r4
%endif
    jz .odd_height
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
    call .hv1
    call .n0
    call .n1
    sub             hd, 2
    jge .main_loop
    test         edgeb, 8 ; LR_HAVE_BOTTOM
    jz .extend_bottom
    mov           lpfq, lpfm
    call .hv0_bottom
    movif32       lpfq, hvsrcm
    add           lpfq, stridemp
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
    movif32    strideq, stridemp
    lea            r10, [lpfq+strideq*4]
    mov           lpfq, dstq
    lea            r10, [r10+strideq*2]
    mov           lpfm, r10
    call .h
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wq, w0m
    mov         hvsrcm, lpfq
%endif
    lea             t2, [t1+400*12]
.top_fixup_loop:
    mova            m0, [t1+wq+400* 0]
    mova            m1, [t1+wq+400* 2]
    mova            m2, [t1+wq+400* 4]
    paddw           m0, m0
    mova            m3, [t1+wq+400* 6]
    paddd           m1, m1
    mova            m4, [t1+wq+400* 8]
    paddd           m2, m2
    mova            m5, [t1+wq+400*10]
    mova [t2+wq+400* 0], m0
    mova [t2+wq+400* 2], m1
    mova [t2+wq+400* 4], m2
    mova [t2+wq+400* 6], m3
    mova [t2+wq+400* 8], m4
    mova [t2+wq+400*10], m5
    add             wq, 16
    jl .top_fixup_loop
    movif32         t3, t3m
    movif32         t4, t4m
    call .v0
    jmp .main
.h: ; horizontal boxsum
%assign stack_offset stack_offset+4
%assign calloff 4
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
 %define leftq r4
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 10
    jmp .h_main
.h_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, [base+sgr_lshuf5]
    jmp .h_main
.h_top:
%if ARCH_X86_64
    lea             wq, [r4-4]
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32         wq, w0m
.h_loop:
    movu            m4, [lpfq+wq- 2]
.h_main:
    movu            m5, [lpfq+wq+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             wd, -20
    jl .h_have_right
%if ARCH_X86_32
    pxor            m8, m8
%endif
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_ssse3).extend_right
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
    punpcklwd       m7, m0, m6
    pmaddwd         m7, m7
    punpckhwd       m0, m6
    pmaddwd         m0, m0
    paddd           m2, m7             ; sumsq3
    palignr         m5, m4, 8
    punpcklwd       m7, m5, m4
    paddw           m8, m4, m5
    pmaddwd         m7, m7
    punpckhwd       m5, m4
    pmaddwd         m5, m5
    paddd           m3, m0
    mova [t1+wq+400* 6], m1
    mova [t1+wq+400* 8], m2
    mova [t1+wq+400*10], m3
    paddw           m8, m1             ; sum5
    paddd           m7, m2             ; sumsq5
    paddd           m5, m3
    mova [t1+wq+400* 0], m8
    mova [t1+wq+400* 2], m7
    mova [t1+wq+400* 4], m5
    add             wq, 16
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab3 (even rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 10
    jmp .hv0_main
.hv0_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, [base+sgr_lshuf5]
    jmp .hv0_main
.hv0_bottom:
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movif32         wq, w0m
%if ARCH_X86_32
    jmp .hv0_loop_start
%endif
.hv0_loop:
    movif32       lpfq, hvsrcm
.hv0_loop_start:
    movu            m4, [lpfq+wq- 2]
.hv0_main:
    movu            m5, [lpfq+wq+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp             wd, -20
    jl .hv0_have_right
%if ARCH_X86_32
    pxor            m8, m8
%endif
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_ssse3).extend_right
.hv0_have_right:
    palignr         m3, m5, m4, 2
    palignr         m0, m5, m4, 4
    movif32         t3, t3m
    paddw           m1, m3, m0
    punpcklwd       m2, m3, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m0
    pmaddwd         m3, m3
    palignr         m0, m5, m4, 6
    paddw           m1, m0             ; h sum3
    punpcklwd       m7, m0, m6
    pmaddwd         m7, m7
    punpckhwd       m0, m6
    pmaddwd         m0, m0
    paddd           m2, m7             ; h sumsq3
    palignr         m5, m4, 8
    punpcklwd       m7, m5, m4
    paddw           m8, m4, m5
    pmaddwd         m7, m7
    punpckhwd       m5, m4
    pmaddwd         m5, m5
    paddd           m3, m0
    paddw           m8, m1             ; h sum5
    paddd           m7, m2             ; h sumsq5
    paddd           m5, m3
    mova [t3+wq*2+400*8+ 8], m8
    mova [t3+wq*2+400*0+ 8], m7
    mova [t3+wq*2+400*0+24], m5
    paddw           m8, [t1+wq+400* 0]
    paddd           m7, [t1+wq+400* 2]
    paddd           m5, [t1+wq+400* 4]
    mova [t1+wq+400* 0], m8
    mova [t1+wq+400* 2], m7
    mova [t1+wq+400* 4], m5
    paddw           m0, m1, [t1+wq+400* 6]
    paddd           m4, m2, [t1+wq+400* 8]
    paddd           m5, m3, [t1+wq+400*10]
    mova [t1+wq+400* 6], m1
    mova [t1+wq+400* 8], m2
    mova [t1+wq+400*10], m3
    paddw           m1, m0, [t2+wq+400* 6]
    paddd           m2, m4, [t2+wq+400* 8]
    paddd           m3, m5, [t2+wq+400*10]
    mova [t2+wq+400* 6], m0
    mova [t2+wq+400* 8], m4
    mova [t2+wq+400*10], m5
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
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
%if ARCH_X86_64
    SWAP            m7, m6
%endif
    MAXSD           m4, m2, m7
    MAXSD           m5, m3, m7
    psubd           m4, m2             ; p3
    psubd           m5, m3
    MULLD           m4, m14, m7        ; p3 * s1
    MULLD           m5, m14, m7
    pmaddwd         m0, m11            ; b3 * 455
    pmaddwd         m1, m11
    paddusw         m4, m11
    paddusw         m5, m11
    psrld           m4, 20             ; min(z3, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m7
    MULLD           m1, m5, m7
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    mova [t4+wq*1+400*2+ 4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*2+400*4+ 8], m0
    mova [t3+wq*2+400*4+24], m1
    add             wq, 16
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movif32      leftq, leftm
    movddup         m5, [leftq]
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    add         leftmp, 8
    palignr         m4, m5, 10
    jmp .hv1_main
.hv1_extend_left:
    movif32         wq, w0m
    mova            m4, [lpfq+wq+4]
    pshufb          m4, [base+sgr_lshuf5]
    jmp .hv1_main
.hv1_bottom:
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movif32         wq, w0m
%if ARCH_X86_32
    jmp .hv1_loop_start
%endif
.hv1_loop:
    movif32       lpfq, hvsrcm
.hv1_loop_start:
    movu            m4, [lpfq+wq- 2]
.hv1_main:
    movu            m5, [lpfq+wq+14]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp             wd, -20
    jl .hv1_have_right
%if ARCH_X86_32
    pxor            m8, m8
%endif
    call mangle(private_prefix %+ _sgr_filter_5x5_16bpc_ssse3).extend_right
.hv1_have_right:
    palignr         m7, m5, m4, 2
    palignr         m3, m5, m4, 4
    paddw           m2, m7, m3
    punpcklwd       m0, m7, m3
    pmaddwd         m0, m0
    punpckhwd       m7, m3
    pmaddwd         m7, m7
    palignr         m3, m5, m4, 6
    paddw           m2, m3             ; h sum3
    punpcklwd       m1, m3, m6
    pmaddwd         m1, m1
    punpckhwd       m3, m6
    pmaddwd         m3, m3
    paddd           m0, m1             ; h sumsq3
    palignr         m5, m4, 8
    punpckhwd       m1, m4, m5
    paddw           m8, m4, m5
    pmaddwd         m1, m1
    punpcklwd       m4, m5
    pmaddwd         m4, m4
    paddd           m7, m3
    paddw           m5, m2, [t2+wq+400* 6]
    mova [t2+wq+400* 6], m2
    paddw           m8, m2             ; h sum5
    paddd           m2, m0, [t2+wq+400* 8]
    paddd           m3, m7, [t2+wq+400*10]
    mova [t2+wq+400* 8], m0
    mova [t2+wq+400*10], m7
    paddd           m4, m0             ; h sumsq5
    paddd           m1, m7
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
    pslld           m0, m2, 3
    pslld           m7, m3, 3
    paddd           m2, m0             ; ((a3 + 8) >> 4) * 9
    paddd           m3, m7
    psrlw           m7, m5, 1
    pavgw           m7, m6             ; (b3 + 2) >> 2
    punpcklwd       m0, m7, m6
    pmaddwd         m0, m0
    punpckhwd       m7, m6
    pmaddwd         m7, m7
%if ARCH_X86_32
    mova      [esp+20], m8
%else
    SWAP            m8, m6
%endif
    MAXSD           m2, m0, m8
    MAXSD           m3, m7, m8
    pxor            m8, m8
    psubd           m2, m0             ; p3
    psubd           m3, m7
    punpcklwd       m0, m5, m8         ; b3
    punpckhwd       m5, m8
    MULLD           m2, m14, m8        ; p3 * s1
    MULLD           m3, m14, m8
    pmaddwd         m0, m11            ; b3 * 455
    pmaddwd         m5, m11
    paddusw         m2, m11
    paddusw         m3, m11
    psrld           m2, 20             ; min(z3, 255)
    movif32         t3, t3m
    psrld           m3, 20
    GATHER_X_BY_X   m8, m2, m3, r0, dstm
    punpcklwd       m2, m8, m8
    punpckhwd       m3, m8, m8
    MULLD           m0, m2, m7
    MULLD           m5, m3, m7
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m5, m10
    psrld           m0, 12
    psrld           m5, 12
    mova [t4+wq*1+400*4+4], m8
    mova [t3+wq*2+400*8+ 8], m0
    mova [t3+wq*2+400*8+24], m5
%if ARCH_X86_32
    mova            m8, [esp+20]
%else
    SWAP            m6, m8
    pxor            m6, m6
%endif
    paddw           m5, m8, [t2+wq+400*0]
    paddd           m2, m4, [t2+wq+400*2]
    paddd           m3, m1, [t2+wq+400*4]
    paddw           m5, [t1+wq+400*0]
    paddd           m2, [t1+wq+400*2]
    paddd           m3, [t1+wq+400*4]
    mova [t2+wq+400*0], m8
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a5 + 8) >> 4
    psrld           m3, 4
    mova [t2+wq+400*2], m4
    pslld           m8, m2, 4
    mova [t2+wq+400*4], m1
    pslld           m4, m3, 4
    paddd           m8, m2
    pslld           m2, 3
    paddd           m4, m3
    pslld           m3, 3
    paddd           m2, m8             ; ((a5 + 8) >> 4) * 25
    paddd           m3, m4
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    psrlw           m1, m5, 1
    pavgw           m1, m7             ; (b5 + 2) >> 2
    punpcklwd       m4, m1, m7
    pmaddwd         m4, m4
    punpckhwd       m1, m7
    pmaddwd         m1, m1
    punpcklwd       m0, m5, m7         ; b5
    punpckhwd       m5, m7
%if ARCH_X86_64
    SWAP            m7, m6
%endif
    MAXSD           m2, m4, m7
    psubd           m2, m4             ; p5
    MAXSD           m3, m1, m7
    psubd           m3, m1
    MULLD           m2, m13, m7        ; p5 * s0
    MULLD           m3, m13, m7
    pmaddwd         m0, m12             ; b5 * 164
    pmaddwd         m5, m12
    paddusw         m2, m12
    paddusw         m3, m12
    psrld           m2, 20             ; min(z5, 255)
    psrld           m3, 20
    GATHER_X_BY_X   m1, m2, m3, r0, dstm
    punpcklwd       m2, m1, m1
    punpckhwd       m3, m1, m1
    MULLD           m0, m2, m7
    MULLD           m5, m3, m7
    paddd           m0, m10            ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m5, m10
    mova [t4+wq*1+400*0+ 4], m1
    psrld           m0, 12
    psrld           m5, 12
    mova [t3+wq*2+400*0+ 8], m0
    mova [t3+wq*2+400*0+24], m5
    add             wq, 16
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab3 (even rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wd, w0m
%endif
.v0_loop:
    mova            m0, [t1+wq+400* 6]
    mova            m4, [t1+wq+400* 8]
    mova            m5, [t1+wq+400*10]
    paddw           m0, m0
    paddd           m4, m4
    paddd           m5, m5
    paddw           m1, m0, [t2+wq+400* 6]
    paddd           m2, m4, [t2+wq+400* 8]
    paddd           m3, m5, [t2+wq+400*10]
    mova [t2+wq+400* 6], m0
    mova [t2+wq+400* 8], m4
    mova [t2+wq+400*10], m5
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
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
%if ARCH_X86_64
    SWAP            m7, m6
%endif
    MAXSD           m4, m2, m7
    MAXSD           m5, m3, m7
    psubd           m4, m2             ; p3
    psubd           m5, m3
    MULLD           m4, m14, m7        ; p3 * s1
    MULLD           m5, m14, m7
    pmaddwd         m0, m11            ; b3 * 455
    pmaddwd         m1, m11
    paddusw         m4, m11
    paddusw         m5, m11
    psrld           m4, 20             ; min(z3, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m7
    MULLD           m1, m5, m7
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    mova [t4+wq*1+400*2+4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova            m3, [t1+wq+400*0]
    mova            m4, [t1+wq+400*2]
    mova            m5, [t1+wq+400*4]
    mova [t3+wq*2+400*8+ 8], m3
    mova [t3+wq*2+400*0+ 8], m4
    mova [t3+wq*2+400*0+24], m5
    paddw           m3, m3 ; cc5
    paddd           m4, m4
    paddd           m5, m5
    mova [t1+wq+400*0], m3
    mova [t1+wq+400*2], m4
    mova [t1+wq+400*4], m5
    mova [t3+wq*2+400*4+ 8], m0
    mova [t3+wq*2+400*4+24], m1
    add             wq, 16
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-4]
%else
    mov             wd, w0m
%endif
.v1_loop:
    mova            m4, [t1+wq+400* 6]
    mova            m5, [t1+wq+400* 8]
    mova            m7, [t1+wq+400*10]
    paddw           m1, m4, [t2+wq+400* 6]
    paddd           m2, m5, [t2+wq+400* 8]
    paddd           m3, m7, [t2+wq+400*10]
    mova [t2+wq+400* 6], m4
    mova [t2+wq+400* 8], m5
    mova [t2+wq+400*10], m7
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a3 + 8) >> 4
    psrld           m3, 4
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
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
%if ARCH_X86_64
    SWAP            m7, m6
%endif
    MAXSD           m4, m2, m7
    MAXSD           m5, m3, m7
    psubd           m4, m2             ; p3
    psubd           m5, m3
    MULLD           m4, m14, m7        ; p3 * s1
    MULLD           m5, m14, m7
    pmaddwd         m0, m11            ; b3 * 455
    pmaddwd         m1, m11
    paddusw         m4, m11
    paddusw         m5, m11
    psrld           m4, 20             ; min(z3, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m7
    MULLD           m1, m5, m7
    paddd           m0, m10            ; x3 * b3 * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    mova [t4+wq*1+400*4+4], m3
    psrld           m0, 12
    psrld           m8, m1, 12
    mova            m4, [t3+wq*2+400*8+ 8]
    mova            m5, [t3+wq*2+400*0+ 8]
    mova            m7, [t3+wq*2+400*0+24]
    paddw           m1, m4, [t2+wq+400*0]
    paddd           m2, m5, [t2+wq+400*2]
    paddd           m3, m7, [t2+wq+400*4]
    paddw           m1, [t1+wq+400*0]
    paddd           m2, [t1+wq+400*2]
    paddd           m3, [t1+wq+400*4]
    mova [t2+wq+400*0], m4
    mova [t2+wq+400*2], m5
    mova [t2+wq+400*4], m7
    paddd           m2, m9
    paddd           m3, m9
    psrld           m2, 4              ; (a5 + 8) >> 4
    psrld           m3, 4
    mova         [t3+wq*2+400*8+ 8], m0
    pslld           m4, m2, 4
    mova         [t3+wq*2+400*8+24], m8
    pslld           m5, m3, 4
    paddd           m4, m2
    pslld           m2, 3
    paddd           m5, m3
    pslld           m3, 3
    paddd           m2, m4
    paddd           m3, m5
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    psrlw           m5, m1, 1
    pavgw           m5, m7             ; (b5 + 2) >> 2
    punpcklwd       m4, m5, m7
    pmaddwd         m4, m4
    punpckhwd       m5, m7
    pmaddwd         m5, m5
    punpcklwd       m0, m1, m7         ; b5
    punpckhwd       m1, m7
%if ARCH_X86_64
    SWAP            m7, m6
%endif
    MAXSD           m2, m4, m7
    psubd           m2, m4             ; p5
    MAXSD           m3, m5, m7
    psubd           m3, m5
    MULLD           m2, m13, m7        ; p5 * s0
    MULLD           m3, m13, m7
    pmaddwd         m0, m12            ; b5 * 164
    pmaddwd         m1, m12
    paddusw         m2, m12
    paddusw         m3, m12
    psrld           m2, 20             ; min(z5, 255)
    psrld           m3, 20
    GATHER_X_BY_X   m4, m2, m3, r0, dstm
    punpcklwd       m2, m4, m4
    punpckhwd       m3, m4, m4
    MULLD           m0, m2, m7
    MULLD           m1, m3, m7
    paddd           m0, m10            ; x5 * b5 * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m10
    mova [t4+wq*1+400*0+ 4], m4
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*2+400*0+ 8], m0
    mova [t3+wq*2+400*0+24], m1
    add             wq, 16
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    movif64         wq, r4
    movif32         wd, w1m
.prep_n_loop:
    movu            m0, [t4+wq*1+400*0+ 2]
    movu            m1, [t3+wq*2+400*0+ 4]
    movu            m2, [t3+wq*2+400*0+20]
    movu            m7, [t4+wq*1+400*0+ 4]
    movu            m8, [t3+wq*2+400*0+ 8]
    paddw           m3, m0, [t4+wq*1+400*0+ 0]
    paddd           m4, m1, [t3+wq*2+400*0+ 0]
    paddd           m5, m2, [t3+wq*2+400*0+16]
    paddw           m3, m7
    paddd           m4, m8
    movu            m7, [t3+wq*2+400*0+24]
    paddw           m0, m3
    paddd           m1, m4
    psllw           m3, 2
    pslld           m4, 2
    paddd           m5, m7
    paddd           m2, m5
    pslld           m5, 2
    paddw           m0, m3               ; a5 565
    paddd           m1, m4               ; b5 565
    paddd           m2, m5
    mova [t4+wq*1+400* 6+ 0], m0
    mova [t3+wq*2+400*12+ 0], m1
    mova [t3+wq*2+400*12+16], m2
    movu            m0, [t4+wq*1+400*2+ 4]
    movu            m1, [t3+wq*2+400*4+ 8]
    movu            m2, [t3+wq*2+400*4+24]
    movu            m3, [t4+wq*1+400*2+ 2]
    movu            m4, [t3+wq*2+400*4+ 4]
    movu            m5, [t3+wq*2+400*4+20]
    paddw           m0, [t4+wq*1+400*2+ 0]
    paddd           m1, [t3+wq*2+400*4+ 0]
    paddd           m2, [t3+wq*2+400*4+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                ; a3[-1] 444
    pslld           m4, 2                ; b3[-1] 444
    pslld           m5, 2
    psubw           m3, m0               ; a3[-1] 343
    psubd           m4, m1               ; b3[-1] 343
    psubd           m5, m2
    mova [t4+wq*1+400* 8+ 0], m3
    mova [t3+wq*2+400*16+ 0], m4
    mova [t3+wq*2+400*16+16], m5
    movu            m0, [t4+wq*1+400*4+ 4]
    movu            m1, [t3+wq*2+400*8+ 8]
    movu            m2, [t3+wq*2+400*8+24]
    movu            m3, [t4+wq*1+400*4+ 2]
    movu            m4, [t3+wq*2+400*8+ 4]
    movu            m5, [t3+wq*2+400*8+20]
    paddw           m0, [t4+wq*1+400*4+ 0]
    paddd           m1, [t3+wq*2+400*8+ 0]
    paddd           m2, [t3+wq*2+400*8+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                 ; a3[ 0] 444
    pslld           m4, 2                 ; b3[ 0] 444
    pslld           m5, 2
    mova [t4+wq*1+400*10+ 0], m3
    mova [t3+wq*2+400*20+ 0], m4
    mova [t3+wq*2+400*20+16], m5
    psubw           m3, m0                ; a3[ 0] 343
    psubd           m4, m1                ; b3[ 0] 343
    psubd           m5, m2
    mova [t4+wq*1+400*12+ 0], m3
    mova [t3+wq*2+400*24+ 0], m4
    mova [t3+wq*2+400*24+16], m5
    add             wq, 16
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    movif64         wq, r4
    movif32         wd, w1m
.n0_loop:
    movu            m0, [t4+wq*1+ 4]
    movu            m2, [t4+wq*1+ 2]
    paddw           m0, [t4+wq*1+ 0]
    paddw           m0, m2
    paddw           m2, m0
    psllw           m0, 2
    paddw           m0, m2               ; a5
    movu            m4, [t3+wq*2+ 8]
    movu            m5, [t3+wq*2+24]
    movu            m1, [t3+wq*2+ 4]
    movu            m3, [t3+wq*2+20]
    paddd           m4, [t3+wq*2+ 0]
    paddd           m5, [t3+wq*2+16]
    paddd           m4, m1
    paddd           m5, m3
    paddd           m1, m4
    paddd           m3, m5
    pslld           m4, 2
    pslld           m5, 2
    paddd           m4, m1               ; b5
    paddd           m5, m3
    movu            m2, [t4+wq*1+400* 6]
    paddw           m2, m0
    mova [t4+wq*1+400* 6], m0
    paddd           m0, m4, [t3+wq*2+400*12+ 0]
    paddd           m1, m5, [t3+wq*2+400*12+16]
    mova [t3+wq*2+400*12+ 0], m4
    mova [t3+wq*2+400*12+16], m5
    mova [rsp+16+ARCH_X86_32*4], m1
    movu            m3, [t4+wq*1+400*2+4]
    movu            m5, [t4+wq*1+400*2+2]
    paddw           m3, [t4+wq*1+400*2+0]
    paddw           m5, m3
    psllw           m5, 2                ; a3[ 1] 444
    psubw           m4, m5, m3           ; a3[ 1] 343
    movu            m3, [t4+wq*1+400* 8]
    paddw           m3, [t4+wq*1+400*10]
    paddw           m3, m4
    mova [t4+wq*1+400* 8], m4
    mova [t4+wq*1+400*10], m5
    movu            m1, [t3+wq*2+400*4+ 8]
    movu            m5, [t3+wq*2+400*4+ 4]
    movu            m7, [t3+wq*2+400*4+24]
    movu            m8, [t3+wq*2+400*4+20]
    paddd           m1, [t3+wq*2+400*4+ 0]
    paddd           m7, [t3+wq*2+400*4+16]
    paddd           m5, m1
    paddd           m8, m7
    pslld           m5, 2                ; b3[ 1] 444
    pslld           m8, 2
    psubd           m4, m5, m1           ; b3[ 1] 343
%if ARCH_X86_32
    mova      [esp+52], m8
    psubd           m8, m7
%else
    psubd           m6, m8, m7
    SWAP            m8, m6
%endif
    paddd           m1, m4, [t3+wq*2+400*16+ 0]
    paddd           m7, m8, [t3+wq*2+400*16+16]
    paddd           m1, [t3+wq*2+400*20+ 0]
    paddd           m7, [t3+wq*2+400*20+16]
    mova [t3+wq*2+400*16+ 0], m4
    mova [t3+wq*2+400*16+16], m8
    mova [t3+wq*2+400*20+ 0], m5
%if ARCH_X86_32
    mova            m8, [esp+52]
%else
    SWAP            m8, m6
    pxor            m6, m6
%endif
    mova [t3+wq*2+400*20+16], m8
    mova [rsp+32+ARCH_X86_32*4], m7
    movu            m5, [dstq+wq]
    punpcklwd       m4, m5, m6
    punpcklwd       m7, m2, m6
    pmaddwd         m7, m4               ; a5 * src
    punpcklwd       m8, m3, m6
    pmaddwd         m8, m4               ; a3 * src
    punpckhwd       m5, m6
    punpckhwd       m2, m6
    pmaddwd         m2, m5
    punpckhwd       m3, m6
    pmaddwd         m3, m5
    pslld           m4, 13
    pslld           m5, 13
    psubd           m0, m7               ; b5 - a5 * src + (1 << 8)
    psubd           m1, m8               ; b3 - a3 * src + (1 << 8)
    mova            m7, [base+pd_0xffff]
    psrld           m0, 9
    pslld           m1, 7
    pand            m0, m7
    pandn           m8, m7, m1
    por             m0, m8
    mova            m1, [rsp+16+ARCH_X86_32*4]
    mova            m8, [rsp+32+ARCH_X86_32*4]
    psubd           m1, m2
    psubd           m8, m3
    mova            m2, [base+pd_4096]
    psrld           m1, 9
    pslld           m8, 7
    pand            m1, m7
    pandn           m7, m8
    por             m1, m7
    pmaddwd         m0, m15
    pmaddwd         m1, m15
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    paddd           m4, m2
    paddd           m5, m2
    paddd           m0, m4
    paddd           m1, m5
    psrad           m0, 8
    psrad           m1, 8
    packssdw        m0, m1               ; clip
    pmaxsw          m0, m7
    psrlw           m0, 5
    mova     [dstq+wq], m0
    add             wq, 16
    jl .n0_loop
    add           dstq, stridemp
    ret
%if ARCH_X86_64
    SWAP            m6, m7
%endif
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    movif64         wq, r4
    movif32         wd, w1m
.n1_loop:
    movu            m3, [t4+wq*1+400*4+4]
    movu            m5, [t4+wq*1+400*4+2]
    paddw           m3, [t4+wq*1+400*4+0]
    paddw           m5, m3
    psllw           m5, 2                ; a3[ 1] 444
    psubw           m4, m5, m3           ; a3[ 1] 343
    paddw           m3, m4, [t4+wq*1+400*12]
    paddw           m3, [t4+wq*1+400*10]
    mova [t4+wq*1+400*10], m5
    mova [t4+wq*1+400*12], m4
    movu            m1, [t3+wq*2+400*8+ 8]
    movu            m5, [t3+wq*2+400*8+ 4]
    movu            m7, [t3+wq*2+400*8+24]
    movu            m8, [t3+wq*2+400*8+20]
    paddd           m1, [t3+wq*2+400*8+ 0]
    paddd           m7, [t3+wq*2+400*8+16]
    paddd           m5, m1
    paddd           m8, m7
    pslld           m5, 2                ; b3[ 1] 444
    pslld           m8, 2
    psubd           m4, m5, m1           ; b3[ 1] 343
    psubd           m0, m8, m7
    paddd           m1, m4, [t3+wq*2+400*24+ 0]
    paddd           m7, m0, [t3+wq*2+400*24+16]
    paddd           m1, [t3+wq*2+400*20+ 0]
    paddd           m7, [t3+wq*2+400*20+16]
    mova [t3+wq*2+400*20+ 0], m5
    mova [t3+wq*2+400*20+16], m8
    mova [t3+wq*2+400*24+ 0], m4
    mova [t3+wq*2+400*24+16], m0
    mova            m5, [dstq+wq]
    mova            m2, [t4+wq*1+400* 6]
    punpcklwd       m4, m5, m6
    punpcklwd       m8, m2, m6
    pmaddwd         m8, m4               ; a5 * src
    punpcklwd       m0, m3, m6
    pmaddwd         m0, m4               ; a3 * src
    punpckhwd       m5, m6
    punpckhwd       m2, m6
    pmaddwd         m2, m5
    punpckhwd       m3, m6
    pmaddwd         m3, m5
    psubd           m1, m0               ; b3 - a3 * src + (1 << 8)
    pslld           m4, 13
    pslld           m5, 13
    mova            m0, [t3+wq*2+400*12+ 0]
    psubd           m0, m8               ; b5 - a5 * src + (1 << 8)
    mova            m8, [t3+wq*2+400*12+16]
    psubd           m8, m2
    psubd           m7, m3
    mova            m2, [base+pd_0xffff]
    pslld           m1, 7
    psrld           m0, 8
    psrld           m8, 8
    pslld           m7, 7
    pand            m0, m2
    pandn           m3, m2, m1
    por             m0, m3
    pand            m8, m2
    pandn           m2, m7
    por             m2, m8
    mova            m1, [base+pd_4096]
    pmaddwd         m0, m15
    pmaddwd         m2, m15
%if ARCH_X86_64
    SWAP            m7, m6
%endif
    pxor            m7, m7
    paddd           m4, m1
    paddd           m5, m1
    paddd           m0, m4
    paddd           m2, m5
    psrad           m0, 8
    psrad           m2, 8
    packssdw        m0, m2              ; clip
    pmaxsw          m0, m7
    psrlw           m0, 5
    mova     [dstq+wq], m0
    add             wq, 16
    jl .n1_loop
    add           dstq, stridemp
    movif32       dstm, dstq
    ret
