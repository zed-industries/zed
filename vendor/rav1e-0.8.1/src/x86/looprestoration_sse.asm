; Copyright © 2018, VideoLAN and dav1d authors
; Copyright © 2018, Two Orioles, LLC
; Copyright © 2018, VideoLabs
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

wiener_init:   db  6,  7,  6,  7,  6,  7,  6,  7,  0,  0,  0,  0,  2,  4,  2,  4
wiener_shufA:  db  1,  7,  2,  8,  3,  9,  4, 10,  5, 11,  6, 12,  7, 13,  8, 14
wiener_shufB:  db  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8,  8,  9,  9, 10
wiener_shufC:  db  6,  5,  7,  6,  8,  7,  9,  8, 10,  9, 11, 10, 12, 11, 13, 12
wiener_shufD:  db  4, -1,  5, -1,  6, -1,  7, -1,  8, -1,  9, -1, 10, -1, 11, -1
wiener_l_shuf: db  0,  0,  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11
sgr_lshuf3:    db  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13
sgr_lshuf5:    db  0,  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12
pb_0to15:      db  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15

pb_right_ext_mask: times 24 db 0xff
                   times 8 db 0
pb_1:          times 16 db 1
pb_3:          times 16 db 3
pw_256:        times 8 dw 256
pw_2056:       times 8 dw 2056
pw_m16380:     times 8 dw -16380
pd_4096:       times 4 dd 4096
pd_34816:      times 4 dd 34816
pd_0xffff:     times 4 dd 0xffff
pd_0xf00800a4: times 4 dd 0xf00800a4
pd_0xf00801c7: times 4 dd 0xf00801c7

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

%if ARCH_X86_32
 %define PIC_base_offset $$

 %macro SETUP_PIC 1-3 1,0 ; PIC_reg, save_PIC_reg, restore_PIC_reg
  %assign pic_reg_stk_off 4
  %xdefine PIC_reg %1
  %if %2 == 1
    mov        [esp], %1
  %endif
    LEA      PIC_reg, PIC_base_offset
  %if %3 == 1
    XCHG_PIC_REG
  %endif
 %endmacro

 %macro XCHG_PIC_REG 0
    mov [esp+pic_reg_stk_off], PIC_reg
    %assign pic_reg_stk_off (pic_reg_stk_off+4) % 8
    mov PIC_reg, [esp+pic_reg_stk_off]
 %endmacro

 %define PIC_sym(sym)   (PIC_reg+(sym)-PIC_base_offset)

%else
 %macro XCHG_PIC_REG 0
 %endmacro

 %define PIC_sym(sym)   (sym)
%endif

%macro WIENER 0
%if ARCH_X86_64
DECLARE_REG_TMP 9, 7, 10, 11, 12, 13, 14 ; ring buffer pointers
cglobal wiener_filter7_8bpc, 4, 15, 16, -384*12-16, dst, stride, left, lpf, \
                                                    w, h, edge, flt, x
    %define tmpstrideq strideq
    %define base 0
    mov           fltq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    movq           m14, [fltq]
    add           lpfq, wq
    movq            m7, [fltq+16]
    add           dstq, wq
    lea             t1, [rsp+wq*2+16]
    mova           m15, [pw_2056]
    neg             wq
%if cpuflag(ssse3)
    pshufb         m14, [wiener_init]
    mova            m8, [wiener_shufA]
    pshufd         m12, m14, q2222  ; x0 x0
    mova            m9, [wiener_shufB]
    pshufd         m13, m14, q3333  ; x1 x2
    mova           m10, [wiener_shufC]
    punpcklqdq     m14, m14         ; x3
    mova           m11, [wiener_shufD]
%else
    mova           m10, [pw_m16380]
    punpcklwd      m14, m14
    pshufd         m11, m14, q0000 ; x0
    pshufd         m12, m14, q1111 ; x1
    pshufd         m13, m14, q2222 ; x2
    pshufd         m14, m14, q3333 ; x3
%endif
%else
DECLARE_REG_TMP 4, 0, _, 5
%if cpuflag(ssse3)
    %define m10         [base+wiener_shufC]
    %define m11         [base+wiener_shufD]
    %define stk_off     96
%else
    %define m10         [base+pw_m16380]
    %define m11         [stk+96]
    %define stk_off     112
%endif
cglobal wiener_filter7_8bpc, 0, 7, 8, -384*12-stk_off, _, x, left, lpf, tmpstride
    %define base        r6-pb_right_ext_mask-21
    %define stk         esp
    %define dstq        leftq
    %define edgeb       byte edged
    %define edged       [stk+ 8]
    %define dstmp       [stk+12]
    %define hd    dword [stk+16]
    %define wq          [stk+20]
    %define strideq     [stk+24]
    %define leftmp      [stk+28]
    %define t2          [stk+32]
    %define t4          [stk+36]
    %define t5          [stk+40]
    %define t6          [stk+44]
    %define m8          [base+wiener_shufA]
    %define m9          [base+wiener_shufB]
    %define m12         [stk+48]
    %define m13         [stk+64]
    %define m14         [stk+80]
    %define m15         [base+pw_2056]
    mov             r1, r6m ; flt
    mov             r0, r0m ; dst
    mov             r4, r4m ; w
    mov           lpfq, lpfm
    mov             r2, r7m ; edge
    mov             r5, r5m ; h
    movq            m3, [r1+ 0]
    movq            m7, [r1+16]
    add             r0, r4
    mov             r1, r1m ; stride
    add           lpfq, r4
    mov          edged, r2
    mov             r2, r2m ; left
    mov          dstmp, r0
    lea             t1, [rsp+r4*2+stk_off]
    mov             hd, r5
    neg             r4
    LEA             r6, pb_right_ext_mask+21
    mov             wq, r4
    mov        strideq, r1
    mov         leftmp, r2
    mov             r4, r1
%if cpuflag(ssse3)
    pshufb          m3, [base+wiener_init]
    pshufd          m1, m3, q2222
    pshufd          m2, m3, q3333
    punpcklqdq      m3, m3
%else
    punpcklwd       m3, m3
    pshufd          m0, m3, q0000
    pshufd          m1, m3, q1111
    pshufd          m2, m3, q2222
    pshufd          m3, m3, q3333
    mova           m11, m0
%endif
    mova           m12, m1
    mova           m13, m2
    mova           m14, m3
%endif
    psllw           m7, 5
    pshufd          m6, m7, q0000 ; y0 y1
    pshufd          m7, m7, q1111 ; y2 y3
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t6, t1
    mov             t5, t1
    add             t1, 384*2
    call .h_top
    lea             t3, [lpfq+tmpstrideq*4]
    mov           lpfq, dstmp
    add             t3, tmpstrideq
    mov          [rsp], t3 ; below
    mov             t4, t1
    add             t1, 384*2
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
    call mangle(private_prefix %+ _wiener_filter7_8bpc_ssse3).v
    RET
.no_top:
    lea             t3, [lpfq+tmpstrideq*4]
    mov           lpfq, dstmp
    lea             t3, [t3+tmpstrideq*2]
    mov          [rsp], t3
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
    call mangle(private_prefix %+ _wiener_filter7_8bpc_ssse3).v
.v2:
    call mangle(private_prefix %+ _wiener_filter7_8bpc_ssse3).v
    jmp .v1
.extend_right:
    movd            m2, [lpfq-4]
%if ARCH_X86_64
    push            r0
    lea             r0, [pb_right_ext_mask+21]
    movu            m0, [r0+xq+0]
    movu            m1, [r0+xq+8]
    pop             r0
%else
    movu            m0, [r6+xq+0]
    movu            m1, [r6+xq+8]
%endif
%if cpuflag(ssse3)
    pshufb          m2, [base+pb_3]
%else
    punpcklbw       m2, m2
    pshuflw         m2, m2, q3333
    punpcklqdq      m2, m2
%endif
    pand            m4, m0
    pand            m5, m1
    pandn           m0, m2
    pandn           m1, m2
    por             m4, m0
    por             m5, m1
    ret
.h:
    %define stk esp+4 ; offset due to call
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movifnidn    leftq, leftmp
    mova            m4, [lpfq+xq]
    movd            m5, [leftq]
    add          leftq, 4
    pslldq          m4, 4
    por             m4, m5
    movifnidn   leftmp, leftq
    jmp .h_main
.h_extend_left:
%if cpuflag(ssse3)
    mova            m4, [lpfq+xq]
    pshufb          m4, [base+wiener_l_shuf]
%else
    mova            m5, [lpfq+xq]
    pshufd          m4, m5, q2103
    punpcklbw       m5, m5
    punpcklwd       m5, m5
    movss           m4, m5
%endif
    jmp .h_main
.h_top:
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m4, [lpfq+xq-4]
.h_main:
    movu            m5, [lpfq+xq+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             xd, -18
    jl .h_have_right
    call .extend_right
.h_have_right:
%macro %%h7 0
%if cpuflag(ssse3)
    pshufb          m0, m4, m8
    pmaddubsw       m0, m12
    pshufb          m1, m5, m8
    pmaddubsw       m1, m12
    pshufb          m2, m4, m9
    pmaddubsw       m2, m13
    pshufb          m3, m5, m9
    pmaddubsw       m3, m13
    paddw           m0, m2
    pshufb          m2, m4, m10
    pmaddubsw       m2, m13
    paddw           m1, m3
    pshufb          m3, m5, m10
    pmaddubsw       m3, m13
    pshufb          m4, m11
    paddw           m0, m2
    pmullw          m2, m14, m4
    pshufb          m5, m11
    paddw           m1, m3
    pmullw          m3, m14, m5
    psllw           m4, 7
    psllw           m5, 7
    paddw           m0, m2
    mova            m2, [base+pw_m16380]
    paddw           m1, m3
    paddw           m4, m2
    paddw           m5, m2
    paddsw          m0, m4
    paddsw          m1, m5
%else
    psrldq          m0, m4, 1
    pslldq          m1, m4, 1
    pxor            m3, m3
    punpcklbw       m0, m3
    punpckhbw       m1, m3
    paddw           m0, m1
    pmullw          m0, m11
    psrldq          m1, m4, 2
    pslldq          m2, m4, 2
    punpcklbw       m1, m3
    punpckhbw       m2, m3
    paddw           m1, m2
    pmullw          m1, m12
    paddw           m0, m1
    pshufd          m2, m4, q0321
    punpcklbw       m2, m3
    pmullw          m1, m14, m2
    paddw           m0, m1
    psrldq          m1, m4, 3
    pslldq          m4, 3
    punpcklbw       m1, m3
    punpckhbw       m4, m3
    paddw           m1, m4
    pmullw          m1, m13
    paddw           m0, m1
    psllw           m2, 7
    paddw           m2, m10
    paddsw          m0, m2
    psrldq          m1, m5, 1
    pslldq          m2, m5, 1
    punpcklbw       m1, m3
    punpckhbw       m2, m3
    paddw           m1, m2
    pmullw          m1, m11
    psrldq          m2, m5, 2
    pslldq          m4, m5, 2
    punpcklbw       m2, m3
    punpckhbw       m4, m3
    paddw           m2, m4
    pmullw          m2, m12
    paddw           m1, m2
    pshufd          m4, m5, q0321
    punpcklbw       m4, m3
    pmullw          m2, m14, m4
    paddw           m1, m2
    psrldq          m2, m5, 3
    pslldq          m5, 3
    punpcklbw       m2, m3
    punpckhbw       m5, m3
    paddw           m2, m5
    pmullw          m2, m13
    paddw           m1, m2
    psllw           m4, 7
    paddw           m4, m10
    paddsw          m1, m4
%endif
%endmacro
    %%h7
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m15
    paddw           m1, m15
    mova  [t1+xq*2+ 0], m0
    mova  [t1+xq*2+16], m1
    add             xq, 16
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movifnidn    leftq, leftmp
    mova            m4, [lpfq+xq]
    movd            m5, [leftq]
    add          leftq, 4
    pslldq          m4, 4
    por             m4, m5
    movifnidn   leftmp, leftq
    jmp .hv_main
.hv_extend_left:
%if cpuflag(ssse3)
    mova            m4, [lpfq+xq]
    pshufb          m4, [base+wiener_l_shuf]
%else
    mova            m5, [lpfq+xq]
    pshufd          m4, m5, q2103
    punpcklbw       m5, m5
    punpcklwd       m5, m5
    movss           m4, m5
%endif
    jmp .hv_main
.hv_bottom:
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m4, [lpfq+xq-4]
.hv_main:
    movu            m5, [lpfq+xq+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp             xd, -18
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    %%h7
%if ARCH_X86_64
    mova            m2, [t4+xq*2]
    paddw           m2, [t2+xq*2]
%else
    mov             r2, t4
    mova            m2, [r2+xq*2]
    mov             r2, t2
    paddw           m2, [r2+xq*2]
    mov             r2, t5
%endif
    mova            m3, [t3+xq*2]
%if ARCH_X86_64
    mova            m5, [t5+xq*2]
%else
    mova            m5, [r2+xq*2]
    mov             r2, t6
%endif
    paddw           m5, [t1+xq*2]
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m15
    paddw           m1, m15
%if ARCH_X86_64
    paddw           m4, m0, [t6+xq*2]
%else
    paddw           m4, m0, [r2+xq*2]
    mov             r2, t4
%endif
    mova     [t0+xq*2], m0
    punpcklwd       m0, m2, m3
    pmaddwd         m0, m7
    punpckhwd       m2, m3
    pmaddwd         m2, m7
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m6
    punpckhwd       m4, m5
    pmaddwd         m4, m6
    paddd           m0, m3
    mova            m3, [t3+xq*2+16]
    paddd           m4, m2
%if ARCH_X86_64
    mova            m2, [t4+xq*2+16]
    paddw           m2, [t2+xq*2+16]
    mova            m5, [t5+xq*2+16]
%else
    mova            m2, [r2+xq*2+16]
    mov             r2, t2
    paddw           m2, [r2+xq*2+16]
    mov             r2, t5
    mova            m5, [r2+xq*2+16]
    mov             r2, t6
%endif
    paddw           m5, [t1+xq*2+16]
    packuswb        m0, m4
%if ARCH_X86_64
    paddw           m4, m1, [t6+xq*2+16]
%else
    paddw           m4, m1, [r2+xq*2+16]
    mov           dstq, dstmp
%endif
    mova  [t0+xq*2+16], m1
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m7
    punpckhwd       m2, m3
    pmaddwd         m2, m7
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m6
    punpckhwd       m4, m5
    pmaddwd         m4, m6
    paddd           m1, m3
    paddd           m2, m4
    packuswb        m1, m2
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
    mova     [dstq+xq], m0
    add             xq, 16
    jl .hv_loop
    add           dstq, strideq
%if ARCH_X86_64
    mov             t6, t5
    mov             t5, t4
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, t6
%else
    mov          dstmp, dstq
    mov             r1, t5
    mov             r2, t4
    mov             t6, r1
    mov             t5, r2
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, r1
%endif
    ret
%if cpuflag(ssse3) ; identical in sse2 and ssse3, so share code
.v:
    mov             xq, wq
.v_loop:
%if ARCH_X86_64
    mova            m1, [t4+xq*2]
    paddw           m1, [t2+xq*2]
%else
    mov             r2, t4
    mova            m1, [r2+xq*2]
    mov             r2, t2
    paddw           m1, [r2+xq*2]
    mov             r2, t6
%endif
    mova            m2, [t3+xq*2]
    mova            m4, [t1+xq*2]
%if ARCH_X86_64
    paddw           m3, m4, [t6+xq*2]
    paddw           m4, [t5+xq*2]
%else
    paddw           m3, m4, [r2+xq*2]
    mov             r2, t5
    paddw           m4, [r2+xq*2]
    mov             r2, t4
%endif
    punpcklwd       m0, m1, m2
    pmaddwd         m0, m7
    punpckhwd       m1, m2
    pmaddwd         m1, m7
    punpcklwd       m2, m3, m4
    pmaddwd         m2, m6
    punpckhwd       m3, m4
    pmaddwd         m3, m6
    paddd           m0, m2
    paddd           m1, m3
%if ARCH_X86_64
    mova            m2, [t4+xq*2+16]
    paddw           m2, [t2+xq*2+16]
%else
    mova            m2, [r2+xq*2+16]
    mov             r2, t2
    paddw           m2, [r2+xq*2+16]
    mov             r2, t6
%endif
    mova            m3, [t3+xq*2+16]
    mova            m5, [t1+xq*2+16]
%if ARCH_X86_64
    paddw           m4, m5, [t6+xq*2+16]
    paddw           m5, [t5+xq*2+16]
%else
    paddw           m4, m5, [r2+xq*2+16]
    mov             r2, t5
    paddw           m5, [r2+xq*2+16]
    movifnidn     dstq, dstmp
%endif
    packuswb        m0, m1
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m7
    punpckhwd       m2, m3
    pmaddwd         m2, m7
    punpcklwd       m3, m4, m5
    pmaddwd         m3, m6
    punpckhwd       m4, m5
    pmaddwd         m4, m6
    paddd           m1, m3
    paddd           m2, m4
    packuswb        m1, m2
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
    mova     [dstq+xq], m0
    add             xq, 16
    jl .v_loop
    add           dstq, strideq
%if ARCH_X86_64
    mov             t6, t5
    mov             t5, t4
%else
    mov          dstmp, dstq
    mov             r1, t5
    mov             r2, t4
    mov             t6, r1
    mov             t5, r2
%endif
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    ret
%endif

%if ARCH_X86_64
cglobal wiener_filter5_8bpc, 4, 13, 16, 384*8+16, dst, stride, left, lpf, \
                                                  w, h, edge, flt, x
    mov           fltq, r6mp
    mov             wd, wm
    movifnidn       hd, hm
    mov          edged, r7m
    movq           m14, [fltq]
    add           lpfq, wq
    movq            m7, [fltq+16]
    add           dstq, wq
    mova            m8, [pw_m16380]
    lea             t1, [rsp+wq*2+16]
    mova           m15, [pw_2056]
    neg             wq
%if cpuflag(ssse3)
    pshufb         m14, [wiener_init]
    mova            m9, [wiener_shufB]
    pshufd         m13, m14, q3333  ; x1 x2
    mova           m10, [wiener_shufC]
    punpcklqdq     m14, m14         ; x3
    mova           m11, [wiener_shufD]
    mova           m12, [wiener_l_shuf]
%else
    punpcklwd      m14, m14
    pshufd         m11, m14, q1111 ; x1
    pshufd         m13, m14, q2222 ; x2
    pshufd         m14, m14, q3333 ; x3
%endif
%else
%if cpuflag(ssse3)
    %define stk_off     80
%else
    %define m11         [stk+80]
    %define stk_off     96
%endif
cglobal wiener_filter5_8bpc, 0, 7, 8, -384*8-stk_off, _, x, left, lpf, tmpstride
    %define stk         esp
    %define leftmp      [stk+28]
    %define m8          [base+pw_m16380]
    %define m12         [base+wiener_l_shuf]
    %define m14         [stk+48]
    mov             r1, r6m ; flt
    mov             r0, r0m ; dst
    mov             r4, r4m ; w
    mov           lpfq, lpfm
    mov             r2, r7m ; edge
    mov             r5, r5m ; h
    movq            m2, [r1+ 0]
    movq            m7, [r1+16]
    add             r0, r4
    mov             r1, r1m ; stride
    add           lpfq, r4
    mov          edged, r2
    mov             r2, r2m ; left
    mov          dstmp, r0
    lea             t1, [rsp+r4*2+stk_off]
    mov             hd, r5
    neg             r4
    LEA             r6, pb_right_ext_mask+21
    mov             wq, r4
    mov        strideq, r1
    mov         leftmp, r2
    mov             r4, r1
%if cpuflag(ssse3)
    pshufb          m2, [base+wiener_init]
    pshufd          m1, m2, q3333
    punpcklqdq      m2, m2
%else
    punpcklwd       m2, m2
    pshufd          m0, m2, q1111
    pshufd          m1, m2, q2222
    pshufd          m2, m2, q3333
    mova           m11, m0
%endif
    mova           m13, m1
    mova           m14, m2
%endif
    psllw           m7, 5
    pshufd          m6, m7, q0000 ; __ y1
    pshufd          m7, m7, q1111 ; y2 y3
    test         edgeb, 4 ; LR_HAVE_TOP
    jz .no_top
    call .h_top
    add           lpfq, strideq
    mov             t4, t1
    add             t1, 384*2
    call .h_top
    lea             xq, [lpfq+tmpstrideq*4]
    mov           lpfq, dstmp
    mov             t3, t1
    add             t1, 384*2
    add             xq, tmpstrideq
    mov          [rsp], xq ; below
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
    lea             t3, [lpfq+tmpstrideq*4]
    mov           lpfq, dstmp
    lea             t3, [t3+tmpstrideq*2]
    mov          [rsp], t3
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
    call mangle(private_prefix %+ _wiener_filter5_8bpc_ssse3).v
    add           dstq, strideq
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    movifnidn    dstmp, dstq
.v1:
    call mangle(private_prefix %+ _wiener_filter5_8bpc_ssse3).v
    jmp .end
.h:
    %define stk esp+4
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movifnidn    leftq, leftmp
    mova            m4, [lpfq+xq]
    movd            m5, [leftq]
    add          leftq, 4
    pslldq          m4, 4
    por             m4, m5
    movifnidn   leftmp, leftq
    jmp .h_main
.h_extend_left:
%if cpuflag(ssse3)
    mova            m4, [lpfq+xq]
    pshufb          m4, m12
%else
    mova            m5, [lpfq+xq]
    pshufd          m4, m5, q2103
    punpcklbw       m5, m5
    punpcklwd       m5, m5
    movss           m4, m5
%endif
    jmp .h_main
.h_top:
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
.h_loop:
    movu            m4, [lpfq+xq-4]
.h_main:
    movu            m5, [lpfq+xq+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             xd, -17
    jl .h_have_right
    call mangle(private_prefix %+ _wiener_filter7_8bpc %+ SUFFIX).extend_right
.h_have_right:
%macro %%h5 0
%if cpuflag(ssse3)
    pshufb          m0, m4, m9
    pmaddubsw       m0, m13
    pshufb          m1, m5, m9
    pmaddubsw       m1, m13
    pshufb          m2, m4, m10
    pmaddubsw       m2, m13
    pshufb          m3, m5, m10
    pmaddubsw       m3, m13
    pshufb          m4, m11
    paddw           m0, m2
    pmullw          m2, m14, m4
    pshufb          m5, m11
    paddw           m1, m3
    pmullw          m3, m14, m5
    psllw           m4, 7
    psllw           m5, 7
    paddw           m4, m8
    paddw           m5, m8
    paddw           m0, m2
    paddw           m1, m3
    paddsw          m0, m4
    paddsw          m1, m5
%else
    psrldq          m0, m4, 2
    pslldq          m1, m4, 2
    pxor            m3, m3
    punpcklbw       m0, m3
    punpckhbw       m1, m3
    paddw           m0, m1
    pmullw          m0, m11
    pshufd          m2, m4, q0321
    punpcklbw       m2, m3
    pmullw          m1, m14, m2
    paddw           m0, m1
    psrldq          m1, m4, 3
    pslldq          m4, 3
    punpcklbw       m1, m3
    punpckhbw       m4, m3
    paddw           m1, m4
    pmullw          m1, m13
    paddw           m0, m1
    psllw           m2, 7
    paddw           m2, m8
    paddsw          m0, m2
    psrldq          m1, m5, 2
    pslldq          m4, m5, 2
    punpcklbw       m1, m3
    punpckhbw       m4, m3
    paddw           m1, m4
    pmullw          m1, m11
    pshufd          m4, m5, q0321
    punpcklbw       m4, m3
    pmullw          m2, m14, m4
    paddw           m1, m2
    psrldq          m2, m5, 3
    pslldq          m5, 3
    punpcklbw       m2, m3
    punpckhbw       m5, m3
    paddw           m2, m5
    pmullw          m2, m13
    paddw           m1, m2
    psllw           m4, 7
    paddw           m4, m8
    paddsw          m1, m4
%endif
%endmacro
    %%h5
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m15
    paddw           m1, m15
    mova  [t1+xq*2+ 0], m0
    mova  [t1+xq*2+16], m1
    add             xq, 16
    jl .h_loop
    ret
ALIGN function_align
.hv:
    add           lpfq, strideq
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movifnidn    leftq, leftmp
    mova            m4, [lpfq+xq]
    movd            m5, [leftq]
    add          leftq, 4
    pslldq          m4, 4
    por             m4, m5
    movifnidn   leftmp, leftq
    jmp .hv_main
.hv_extend_left:
%if cpuflag(ssse3)
    mova            m4, [lpfq+xq]
    pshufb          m4, m12
%else
    mova            m5, [lpfq+xq]
    pshufd          m4, m5, q2103
    punpcklbw       m5, m5
    punpcklwd       m5, m5
    movss           m4, m5
%endif
    jmp .hv_main
.hv_bottom:
    mov             xq, wq
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
.hv_loop:
    movu            m4, [lpfq+xq-4]
.hv_main:
    movu            m5, [lpfq+xq+4]
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp             xd, -17
    jl .hv_have_right
    call mangle(private_prefix %+ _wiener_filter7_8bpc %+ SUFFIX).extend_right
.hv_have_right:
    %%h5
    mova            m2, [t3+xq*2]
    paddw           m2, [t1+xq*2]
    psraw           m0, 3
    psraw           m1, 3
    paddw           m0, m15
    paddw           m1, m15
%if ARCH_X86_64
    mova            m3, [t2+xq*2]
    paddw           m4, m0, [t4+xq*2]
%else
    mov             r2, t2
    mova            m3, [r2+xq*2]
    mov             r2, t4
    paddw           m4, m0, [r2+xq*2]
%endif
    mova     [t0+xq*2], m0
    punpcklwd       m0, m2, m3
    pmaddwd         m0, m7
    punpckhwd       m2, m3
    pmaddwd         m2, m7
    punpcklwd       m3, m4, m4
    pmaddwd         m3, m6
    punpckhwd       m4, m4
    pmaddwd         m4, m6
    paddd           m0, m3
    paddd           m4, m2
    mova            m2, [t3+xq*2+16]
    paddw           m2, [t1+xq*2+16]
    packuswb        m0, m4
%if ARCH_X86_64
    mova            m3, [t2+xq*2+16]
    paddw           m4, m1, [t4+xq*2+16]
%else
    paddw           m4, m1, [r2+xq*2+16]
    mov             r2, t2
    mova            m3, [r2+xq*2+16]
    mov           dstq, dstmp
%endif
    mova  [t0+xq*2+16], m1
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m7
    punpckhwd       m2, m3
    pmaddwd         m2, m7
    punpcklwd       m3, m4, m4
    pmaddwd         m3, m6
    punpckhwd       m4, m4
    pmaddwd         m4, m6
    paddd           m1, m3
    paddd           m2, m4
    packuswb        m1, m2
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
    mova     [dstq+xq], m0
    add             xq, 16
    jl .hv_loop
    add           dstq, strideq
    mov             t4, t3
    mov             t3, t2
    mov             t2, t1
    mov             t1, t0
    mov             t0, t4
    movifnidn    dstmp, dstq
    ret
%if cpuflag(ssse3)
.v:
    mov             xq, wq
.v_loop:
    mova            m3, [t1+xq*2]
    paddw           m1, m3, [t3+xq*2]
%if ARCH_X86_64
    mova            m2, [t2+xq*2]
    paddw           m3, [t4+xq*2]
%else
    mov             r2, t2
    mova            m2, [r2+xq*2]
    mov             r2, t4
    paddw           m3, [r2+xq*2]
%endif
    punpcklwd       m0, m1, m2
    pmaddwd         m0, m7
    punpckhwd       m1, m2
    pmaddwd         m1, m7
    punpcklwd       m2, m3
    pmaddwd         m2, m6
    punpckhwd       m3, m3
    pmaddwd         m3, m6
    paddd           m0, m2
    paddd           m1, m3
    mova            m4, [t1+xq*2+16]
    paddw           m2, m4, [t3+xq*2+16]
%if ARCH_X86_64
    mova            m3, [t2+xq*2+16]
    paddw           m4, [t4+xq*2+16]
%else
    paddw           m4, [r2+xq*2+16]
    mov             r2, t2
    mova            m3, [r2+xq*2+16]
    mov           dstq, dstmp
%endif
    packuswb        m0, m1
    punpcklwd       m1, m2, m3
    pmaddwd         m1, m7
    punpckhwd       m2, m3
    pmaddwd         m2, m7
    punpcklwd       m3, m4
    pmaddwd         m3, m6
    punpckhwd       m4, m4
    pmaddwd         m4, m6
    paddd           m1, m3
    paddd           m2, m4
    packuswb        m1, m2
    psrlw           m0, 8
    psrlw           m1, 8
    packuswb        m0, m1
    mova     [dstq+xq], m0
    add             xq, 16
    jl .v_loop
    ret
%endif
%endmacro

INIT_XMM sse2
WIENER

INIT_XMM ssse3
WIENER

;;;;;;;;;;;;;;;;;;;;;;;;;;
;;      self-guided     ;;
;;;;;;;;;;;;;;;;;;;;;;;;;;

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
cglobal sgr_filter_5x5_8bpc, 1, 7, 8, -400*24-16-extra_stack, \
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
 %define  m8 [base+pb_1]
 %define  m9 [esp+calloff+16*2]
 %define m10 [base+pd_0xf00800a4]
 %define m11 [base+sgr_lshuf5]
 %define m12 [base+pd_34816]
 %define m13 [base+pb_0to15]
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
DECLARE_REG_TMP 8, 7, 9, 11, 12
cglobal sgr_filter_5x5_8bpc, 4, 15, 14, -400*24-16, dst, stride, left, lpf, \
                                                    w, h, edge, params
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    mov             wd, wm
%endif
%if ARCH_X86_64
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x-0xf03]
    movifnidn       hd, hm
    mov          edged, r7m
    movu            m9, [paramsq]
    add           lpfq, wq
    mova            m8, [pb_1]
    lea             t1, [rsp+wq*2+20]
    mova           m10, [pd_0xf00800a4]
    add           dstq, wq
    lea             t3, [rsp+wq*4+400*12+16]
    mova           m12, [pd_34816]  ; (1 << 11) + (1 << 15)
    lea             t4, [rsp+wq*2+400*20+16]
    pshufhw         m7, m9, q0000
    pshufb          m9, [pw_256]  ; s0
    punpckhqdq      m7, m7        ; w0
    neg             wq
    mova           m13, [pb_0to15]
    pxor            m6, m6
    mova           m11, [sgr_lshuf5]
    psllw           m7, 4
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm [rsp]
%else
    mov             r1, [rstk+stack_offset+28] ; params
    LEA             r6, $$
    movu            m1, [r1]
    add           lpfm, wq
    lea             t1, [rsp+extra_stack+wq*2+20]
    add           dstq, wq
    lea             t3, [rsp+extra_stack+wq*4+400*12+16]
    mov           dstm, dstq
    lea             t4, [rsp+extra_stack+wq*2+400*20+16]
    mov            t3m, t3
    pshufhw         m7, m1, q0000
    mov            t4m, t4
    pshufb          m1, [base+pw_256] ; s0
    punpckhqdq      m7, m7            ; w0
    psllw           m7, 4
    neg             wq
    mova            m9, m1
    pxor            m6, m6
    mov            w1m, wd
    sub             wd, 2
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
%assign stack_offset stack_offset+8
%assign calloff 8
    movd            m1, wd
    movd            m3, [lpfq-1]
    pshufb          m1, m6
    pshufb          m3, m6
    psubb           m2, m8, m1
    pcmpgtb         m2, m13
    pand            m5, m2
    pandn           m2, m3
    por             m5, m2
    ret
%assign stack_offset stack_offset-4
%assign calloff 4
.h: ; horizontal boxsum
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
 %define leftq r4
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 13
    jmp .h_main
.h_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, m11
    jmp .h_main
.h_top:
%if ARCH_X86_64
    lea             wq, [r4-2]
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32         wq, w0m
.h_loop:
    movu            m5, [lpfq+wq-1]
.h_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             wd, -10
    jl .h_have_right
    call .extend_right
.h_have_right:
    punpcklbw       m4, m5, m6
    punpckhbw       m5, m6
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
    paddw           m0, [t1+wq*2+400*0]
    paddd           m1, [t1+wq*2+400*2]
    paddd           m2, [t1+wq*2+400*4]
.h_loop_end:
    paddd           m1, m5             ; sumsq
    paddd           m2, m4
    mova [t1+wq*2+400*0], m0
    mova [t1+wq*2+400*2], m1
    mova [t1+wq*2+400*4], m2
    add             wq, 8
    jl .h_loop
    ret
.top_fixup:
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov             wd, w0m
%endif
.top_fixup_loop: ; the sums of the first row needs to be doubled
    mova            m0, [t1+wq*2+400*0]
    mova            m1, [t1+wq*2+400*2]
    mova            m2, [t1+wq*2+400*4]
    paddw           m0, m0
    paddd           m1, m1
    paddd           m2, m2
    mova [t2+wq*2+400*0], m0
    mova [t2+wq*2+400*2], m1
    mova [t2+wq*2+400*4], m2
    add             wq, 8
    jl .top_fixup_loop
    ret
ALIGN function_align
.hv: ; horizontal boxsum + vertical boxsum + ab
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 13
    jmp .hv_main
.hv_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, m11
    jmp .hv_main
.hv_bottom:
%if ARCH_X86_64
    lea             wq, [r4-2]
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
    movu            m5, [lpfq+wq-1]
.hv_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv_have_right
    cmp             wd, -10
    jl .hv_have_right
    call .extend_right
.hv_have_right:
    movif32         t3, hd
    punpcklbw       m4, m5, m6
    punpckhbw       m5, m6
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
    paddw           m1, m0, [t1+wq*2+400*0]
    paddd           m4, m2, [t1+wq*2+400*2]
    paddd           m5, m3, [t1+wq*2+400*4]
%if ARCH_X86_64
    test            hd, hd
%else
    test            t3, t3
%endif
    jz .hv_last_row
.hv_main2:
    paddw           m1, [t2+wq*2+400*0] ; hv sum
    paddd           m4, [t2+wq*2+400*2] ; hv sumsq
    paddd           m5, [t2+wq*2+400*4]
    mova [t0+wq*2+400*0], m0
    pslld           m0, m4, 4
    mova [t0+wq*2+400*2], m2
    mova [t0+wq*2+400*4], m3
    pslld           m2, m4, 3
    paddd           m4, m0
    pslld           m0, m5, 4
    paddd           m4, m2             ; a * 25
    pslld           m2, m5, 3
    paddd           m5, m0
    paddd           m5, m2
    punpcklwd       m0, m1, m6         ; b
    punpckhwd       m1, m6
    pmaddwd         m2, m0, m0         ; b * b
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m2         ; p * s
    MULLD           m5, m9, m2
    pmaddwd         m0, m10            ; b * 164
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    movif32         t3, t3m
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, t2, t2m
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m2
    MULLD           m1, m5, m2
    paddd           m0, m12            ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m12
    mova   [t4+wq*2+4], m3
    psrld           m0, 12             ; b
    psrld           m1, 12
    mova  [t3+wq*4+ 8], m0
    mova  [t3+wq*4+24], m1
    add             wq, 8
    jl .hv_loop
    mov             t2, t1
    mov             t1, t0
    mov             t0, t2
    movif32        t2m, t2
    movif32        t0m, t0
    ret
.hv_last_row: ; esoteric edge case for odd heights
    mova [t1+wq*2+400*0], m1
    paddw             m1, m0
    mova [t1+wq*2+400*2], m4
    paddd             m4, m2
    mova [t1+wq*2+400*4], m5
    paddd             m5, m3
    jmp .hv_main2
.v: ; vertical boxsum + ab
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov             wd, w0m
%endif
.v_loop:
    mova            m0, [t1+wq*2+400*0]
    mova            m2, [t1+wq*2+400*2]
    mova            m3, [t1+wq*2+400*4]
    paddw           m1, m0, [t2+wq*2+400*0]
    paddd           m4, m2, [t2+wq*2+400*2]
    paddd           m5, m3, [t2+wq*2+400*4]
    paddw           m0, m0
    paddd           m2, m2
    paddd           m3, m3
    paddw           m1, m0             ; hv sum
    paddd           m4, m2             ; hv sumsq
    pslld           m0, m4, 4
    paddd           m5, m3
    pslld           m2, m4, 3
    paddd           m4, m0
    pslld           m0, m5, 4
    paddd           m4, m2             ; a * 25
    pslld           m2, m5, 3
    paddd           m5, m0
    paddd           m5, m2
    punpcklwd       m0, m1, m6
    punpckhwd       m1, m6
    pmaddwd         m2, m0, m0         ; b * b
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m2         ; p * s
    MULLD           m5, m9, m2
    pmaddwd         m0, m10            ; b * 164
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, t2, t2m
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m2
    MULLD           m1, m5, m2
    paddd           m0, m12            ; x * b * 164 + (1 << 11) + (1 << 15)
    paddd           m1, m12
    mova   [t4+wq*2+4], m3
    psrld           m0, 12             ; b
    psrld           m1, 12
    mova  [t3+wq*4+ 8], m0
    mova  [t3+wq*4+24], m1
    add             wq, 8
    jl .v_loop
    ret
.prep_n: ; initial neighbor setup
    movif64         wq, r4
    movif32         wd, w1m
.prep_n_loop:
    movu            m0, [t4+wq*2+ 2]
    movu            m3, [t4+wq*2+ 4]
    movu            m1, [t3+wq*4+ 4]
    movu            m4, [t3+wq*4+ 8]
    movu            m2, [t3+wq*4+20]
    movu            m5, [t3+wq*4+24]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    paddw           m3, [t4+wq*2+ 0]
    paddd           m4, [t3+wq*4+ 0]
    paddd           m5, [t3+wq*4+16]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m4
    pslld           m4, 2
    paddd           m2, m5
    pslld           m5, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m4             ; b 565
    paddd           m2, m5
    mova [t4+wq*2+400*2+ 0], m0
    mova [t3+wq*4+400*4+ 0], m1
    mova [t3+wq*4+400*4+16], m2
    add             wq, 8
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    movif64         wq, r4
    movif32         wd, w1m
.n0_loop:
    movu            m0, [t4+wq*2+ 2]
    movu            m3, [t4+wq*2+ 4]
    movu            m1, [t3+wq*4+ 4]
    movu            m4, [t3+wq*4+ 8]
    movu            m2, [t3+wq*4+20]
    movu            m5, [t3+wq*4+24]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    paddw           m3, [t4+wq*2+ 0]
    paddd           m4, [t3+wq*4+ 0]
    paddd           m5, [t3+wq*4+16]
    paddw           m0, m3
    psllw           m3, 2
    paddd           m1, m4
    pslld           m4, 2
    paddd           m2, m5
    pslld           m5, 2
    paddw           m0, m3             ; a 565
    paddd           m1, m4             ; b 565
    paddd           m2, m5
    paddw           m3, m0, [t4+wq*2+400*2+ 0]
    paddd           m4, m1, [t3+wq*4+400*4+ 0]
    paddd           m5, m2, [t3+wq*4+400*4+16]
    mova [t4+wq*2+400*2+ 0], m0
    mova [t3+wq*4+400*4+ 0], m1
    mova [t3+wq*4+400*4+16], m2
    movq            m0, [dstq+wq]
    punpcklbw       m0, m6
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
    packuswb        m0, m0
    movq     [dstq+wq], m0
    add             wq, 8
    jl .n0_loop
    add           dstq, stridemp
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    movif64         wq, r4
    movif32         wd, w1m
.n1_loop:
    movq            m0, [dstq+wq]
    mova            m3, [t4+wq*2+400*2+ 0]
    mova            m4, [t3+wq*4+400*4+ 0]
    mova            m5, [t3+wq*4+400*4+16]
    punpcklbw       m0, m6
    punpcklwd       m1, m0, m6          ; src
    punpcklwd       m2, m3, m6          ; a
    pmaddwd         m2, m1              ; a * src
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
    packuswb        m0, m0
    movq     [dstq+wq], m0
    add             wq, 8
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
cglobal sgr_filter_3x3_8bpc, 1, 7, 8, -400*42-16-extra_stack, \
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
 %define  m8 [base+pb_0to15]
 %define  m9 [esp+calloff+16*1]
 %define m10 [base+pd_0xf00801c7]
 %define m11 [base+pd_34816]
 %define m12 m6
 %define m13 [base+sgr_lshuf3]
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
cglobal sgr_filter_3x3_8bpc, 4, 15, 14, -400*42-8, dst, stride, left, lpf, \
                                                   w, h, edge, params
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    mov             wd, wm
%endif
%if ARCH_X86_64
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x-0xf03]
    mov             hd, hm
    mov          edged, r7m
    movq            m9, [paramsq+4]
    add           lpfq, wq
    lea             t1, [rsp+wq*2+12]
    mova            m8, [pb_0to15]
    add           dstq, wq
    lea             t3, [rsp+wq*4+400*12+8]
    mova           m10, [pd_0xf00801c7]
    lea             t4, [rsp+wq*2+400*32+8]
    mova           m11, [pd_34816]
    pshuflw         m7, m9, q3333
    pshufb          m9, [pw_256]  ; s1
    punpcklqdq      m7, m7        ; w1
    neg             wq
    pxor            m6, m6
    mova           m13, [sgr_lshuf3]
    psllw           m7, 4
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm [rsp]
%else
    mov             r1, [rstk+stack_offset+28] ; params
    LEA             r6, $$
    movq            m1, [r1+4]
    add           lpfm, wq
    lea             t1, [rsp+extra_stack+wq*2+20]
    add           dstq, wq
    lea             t3, [rsp+extra_stack+wq*4+400*12+16]
    mov           dstm, dstq
    lea             t4, [rsp+extra_stack+wq*2+400*32+16]
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
    sub             wd, 2
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
    lea             wq, [r4-2]
%else
    mov             wq, w0m
    mov         hvsrcm, lpfq
%endif
    lea             t2, [t1+400*6]
.top_fixup_loop:
    mova            m0, [t1+wq*2+400*0]
    mova            m1, [t1+wq*2+400*2]
    mova            m2, [t1+wq*2+400*4]
    mova [t2+wq*2+400*0], m0
    mova [t2+wq*2+400*2], m1
    mova [t2+wq*2+400*4], m2
    add             wq, 8
    jl .top_fixup_loop
    movif32         t3, t3m
    movif32         t4, t4m
    call .v0
    jmp .main
.extend_right:
%assign stack_offset stack_offset+8
%assign calloff 8
    movd            m0, [lpfq-1]
    movd            m1, wd
    mova            m3, m8
    pshufb          m0, m6
    pshufb          m1, m6
    mova            m2, m6
    psubb           m2, m1
    pcmpgtb         m2, m3
    pand            m5, m2
    pandn           m2, m0
    por             m5, m2
    ret
%assign stack_offset stack_offset-4
%assign calloff 4
.h: ; horizontal boxsum
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
 %define leftq r4
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 14
    jmp .h_main
.h_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, m13
    jmp .h_main
.h_top:
%if ARCH_X86_64
    lea             wq, [r4-2]
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32         wq, w0m
.h_loop:
    movu            m5, [lpfq+wq]
.h_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .h_have_right
    cmp             wd, -9
    jl .h_have_right
    call .extend_right
.h_have_right:
    punpcklbw       m4, m5, m6
    punpckhbw       m5, m6
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
    mova [t1+wq*2+400*0], m1
    mova [t1+wq*2+400*2], m2
    mova [t1+wq*2+400*4], m3
    add             wq, 8
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab (even rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 14
    jmp .hv0_main
.hv0_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, m13
    jmp .hv0_main
.hv0_bottom:
%if ARCH_X86_64
    lea             wq, [r4-2]
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
    movu            m5, [lpfq+wq]
.hv0_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv0_have_right
    cmp             wd, -9
    jl .hv0_have_right
    call .extend_right
.hv0_have_right:
    punpcklbw       m4, m5, m6
    punpckhbw       m5, m6
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
    paddw           m0, m1, [t1+wq*2+400*0]
    paddd           m4, m2, [t1+wq*2+400*2]
    paddd           m5, m3, [t1+wq*2+400*4]
    mova [t1+wq*2+400*0], m1
    mova [t1+wq*2+400*2], m2
    mova [t1+wq*2+400*4], m3
    paddw           m1, m0, [t2+wq*2+400*0]
    paddd           m2, m4, [t2+wq*2+400*2]
    paddd           m3, m5, [t2+wq*2+400*4]
    mova [t2+wq*2+400*0], m0
    mova [t2+wq*2+400*2], m4
    mova [t2+wq*2+400*4], m5
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; a * 9
    paddd           m5, m3
    punpcklwd       m0, m1, m6         ; b
    pmaddwd         m2, m0, m0         ; b * b
    punpckhwd       m1, m6
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m12        ; p * s
    MULLD           m5, m9, m12
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
    MULLD           m0, m4, m12
    MULLD           m1, m5, m12
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova   [t4+wq*2+4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova  [t3+wq*4+ 8], m0
    mova  [t3+wq*4+24], m1
    add             wq, 8
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 14
    jmp .hv1_main
.hv1_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, m13
    jmp .hv1_main
.hv1_bottom:
%if ARCH_X86_64
    lea             wq, [r4-2]
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
    movu            m5, [lpfq+wq]
.hv1_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
    jnz .hv1_have_right
    cmp             wd, -9
    jl .hv1_have_right
    call .extend_right
.hv1_have_right:
    punpcklbw       m4, m5, m6
    punpckhbw       m5, m6
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
    paddw           m1, m0, [t2+wq*2+400*0]
    paddd           m4, m2, [t2+wq*2+400*2]
    paddd           m5, m3, [t2+wq*2+400*4]
    mova [t2+wq*2+400*0], m0
    mova [t2+wq*2+400*2], m2
    mova [t2+wq*2+400*4], m3
    pslld           m2, m4, 3
    pslld           m3, m5, 3
    paddd           m4, m2             ; a * 9
    paddd           m5, m3
    punpcklwd       m0, m1, m6         ; b
    pmaddwd         m2, m0, m0         ; b * b
    punpckhwd       m1, m6
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m12        ; p * s
    MULLD           m5, m9, m12
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
    MULLD           m0, m4, m12
    MULLD           m1, m5, m12
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova [t4+wq*2+400*2 +4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*4+400*4+ 8], m0
    mova [t3+wq*4+400*4+24], m1
    add             wq, 8
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab (even rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov             wd, w0m
%endif
.v0_loop:
    mova            m0, [t1+wq*2+400*0]
    mova            m4, [t1+wq*2+400*2]
    mova            m5, [t1+wq*2+400*4]
    paddw           m0, m0
    paddd           m4, m4
    paddd           m5, m5
    paddw           m1, m0, [t2+wq*2+400*0]
    paddd           m2, m4, [t2+wq*2+400*2]
    paddd           m3, m5, [t2+wq*2+400*4]
    mova [t2+wq*2+400*0], m0
    mova [t2+wq*2+400*2], m4
    mova [t2+wq*2+400*4], m5
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; a * 9
    paddd           m5, m3
    punpcklwd       m0, m1, m6         ; b
    pmaddwd         m2, m0, m0         ; b * b
    punpckhwd       m1, m6
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m12        ; p * s
    MULLD           m5, m9, m12
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m12
    MULLD           m1, m5, m12
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova   [t4+wq*2+4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova  [t3+wq*4+ 8], m0
    mova  [t3+wq*4+24], m1
    add             wq, 8
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov             wd, w0m
%endif
.v1_loop:
    mova            m0, [t1+wq*2+400*0]
    mova            m4, [t1+wq*2+400*2]
    mova            m5, [t1+wq*2+400*4]
    paddw           m1, m0, [t2+wq*2+400*0]
    paddd           m2, m4, [t2+wq*2+400*2]
    paddd           m3, m5, [t2+wq*2+400*4]
    mova [t2+wq*2+400*0], m0
    mova [t2+wq*2+400*2], m4
    mova [t2+wq*2+400*4], m5
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; a * 9
    paddd           m5, m3
    punpcklwd       m0, m1, m6         ; b
    pmaddwd         m2, m0, m0         ; b * b
    punpckhwd       m1, m6
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p
    psubd           m5, m3
    MULLD           m4, m9, m12        ; p * s
    MULLD           m5, m9, m12
    pmaddwd         m0, m10            ; b * 455
    pmaddwd         m1, m10
    paddusw         m4, m10
    paddusw         m5, m10
    psrld           m4, 20             ; min(z, 255)
    psrld           m5, 20
    GATHER_X_BY_X   m3, m4, m5, r0, dstm
    punpcklwd       m4, m3, m3
    punpckhwd       m5, m3, m3
    MULLD           m0, m4, m12
    MULLD           m1, m5, m12
%if ARCH_X86_32
    pxor            m6, m6
%endif
    paddd           m0, m11            ; x * b * 455 + (1 << 11) + (1 << 15)
    paddd           m1, m11
    mova [t4+wq*2+400*2+ 4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*4+400*4+ 8], m0
    mova [t3+wq*4+400*4+24], m1
    add             wq, 8
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    movif64         wq, r4
    movif32         wd, w1m
.prep_n_loop:
    movu            m0, [t4+wq*2+400*0+ 4]
    movu            m1, [t3+wq*4+400*0+ 8]
    movu            m2, [t3+wq*4+400*0+24]
    movu            m3, [t4+wq*2+400*0+ 2]
    movu            m4, [t3+wq*4+400*0+ 4]
    movu            m5, [t3+wq*4+400*0+20]
    paddw           m0, [t4+wq*2+400*0+ 0]
    paddd           m1, [t3+wq*4+400*0+ 0]
    paddd           m2, [t3+wq*4+400*0+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                ; a[-1] 444
    pslld           m4, 2                ; b[-1] 444
    pslld           m5, 2
    psubw           m3, m0               ; a[-1] 343
    psubd           m4, m1               ; b[-1] 343
    psubd           m5, m2
    mova [t4+wq*2+400*4], m3
    mova [t3+wq*4+400*8+ 0], m4
    mova [t3+wq*4+400*8+16], m5
    movu            m0, [t4+wq*2+400*2+ 4]
    movu            m1, [t3+wq*4+400*4+ 8]
    movu            m2, [t3+wq*4+400*4+24]
    movu            m3, [t4+wq*2+400*2+ 2]
    movu            m4, [t3+wq*4+400*4+ 4]
    movu            m5, [t3+wq*4+400*4+20]
    paddw           m0, [t4+wq*2+400*2+ 0]
    paddd           m1, [t3+wq*4+400*4+ 0]
    paddd           m2, [t3+wq*4+400*4+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                 ; a[ 0] 444
    pslld           m4, 2                 ; b[ 0] 444
    pslld           m5, 2
    mova [t4+wq*2+400* 6], m3
    mova [t3+wq*4+400*12+ 0], m4
    mova [t3+wq*4+400*12+16], m5
    psubw           m3, m0                ; a[ 0] 343
    psubd           m4, m1                ; b[ 0] 343
    psubd           m5, m2
    mova [t4+wq*2+400* 8], m3
    mova [t3+wq*4+400*16+ 0], m4
    mova [t3+wq*4+400*16+16], m5
    add             wq, 8
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    movif64         wq, r4
    movif32         wd, w1m
.n0_loop:
    movu            m3, [t4+wq*2+400*0+4]
    movu            m1, [t4+wq*2+400*0+2]
    paddw           m3, [t4+wq*2+400*0+0]
    paddw           m1, m3
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+wq*2+400*4]
    paddw           m3, [t4+wq*2+400*6]
    mova [t4+wq*2+400*4], m2
    mova [t4+wq*2+400*6], m1
    movu            m4, [t3+wq*4+400*0+8]
    movu            m1, [t3+wq*4+400*0+4]
    paddd           m4, [t3+wq*4+400*0+0]
    paddd           m1, m4
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m4           ; b[ 1] 343
    paddd           m4, m2, [t3+wq*4+400* 8+ 0]
    paddd           m4, [t3+wq*4+400*12+ 0]
    mova [t3+wq*4+400* 8+ 0], m2
    mova [t3+wq*4+400*12+ 0], m1
    movu            m5, [t3+wq*4+400*0+24]
    movu            m1, [t3+wq*4+400*0+20]
    paddd           m5, [t3+wq*4+400*0+16]
    paddd           m1, m5
    pslld           m1, 2
    psubd           m2, m1, m5
    paddd           m5, m2, [t3+wq*4+400* 8+16]
    paddd           m5, [t3+wq*4+400*12+16]
    mova [t3+wq*4+400* 8+16], m2
    mova [t3+wq*4+400*12+16], m1
    movq            m0, [dstq+wq]
    punpcklbw       m0, m6
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
    packuswb        m0, m0
    movq     [dstq+wq], m0
    add             wq, 8
    jl .n0_loop
    add           dstq, stridemp
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    movif64         wq, r4
    movif32         wd, w1m
.n1_loop:
    movu            m3, [t4+wq*2+400*2+4]
    movu            m1, [t4+wq*2+400*2+2]
    paddw           m3, [t4+wq*2+400*2+0]
    paddw           m1, m3
    psllw           m1, 2                ; a[ 1] 444
    psubw           m2, m1, m3           ; a[ 1] 343
    paddw           m3, m2, [t4+wq*2+400*6]
    paddw           m3, [t4+wq*2+400*8]
    mova [t4+wq*2+400*6], m1
    mova [t4+wq*2+400*8], m2
    movu            m4, [t3+wq*4+400*4+8]
    movu            m1, [t3+wq*4+400*4+4]
    paddd           m4, [t3+wq*4+400*4+0]
    paddd           m1, m4
    pslld           m1, 2                ; b[ 1] 444
    psubd           m2, m1, m4           ; b[ 1] 343
    paddd           m4, m2, [t3+wq*4+400*12+ 0]
    paddd           m4, [t3+wq*4+400*16+ 0]
    mova [t3+wq*4+400*12+ 0], m1
    mova [t3+wq*4+400*16+ 0], m2
    movu            m5, [t3+wq*4+400*4+24]
    movu            m1, [t3+wq*4+400*4+20]
    paddd           m5, [t3+wq*4+400*4+16]
    paddd           m1, m5
    pslld           m1, 2
    psubd           m2, m1, m5
    paddd           m5, m2, [t3+wq*4+400*12+16]
    paddd           m5, [t3+wq*4+400*16+16]
    mova [t3+wq*4+400*12+16], m1
    mova [t3+wq*4+400*16+16], m2
    movq            m0, [dstq+wq]
    punpcklbw       m0, m6
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
    packuswb        m0, m0
    movq     [dstq+wq], m0
    add             wq, 8
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
cglobal sgr_filter_mix_8bpc, 1, 7, 8, -400*66-48-extra_stack, \
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
 %define  m9 [base+pd_0xffff]
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
cglobal sgr_filter_mix_8bpc, 4, 15, 16, -400*66-40, dst, stride, left, lpf, \
                                                    w, h, edge, params
%endif
%if ARCH_X86_64 || STACK_ALIGNMENT >= 16
    mov             wd, wm
%endif
%if ARCH_X86_64
    mov        paramsq, r6mp
    lea            r13, [sgr_x_by_x-0xf03]
    movifnidn       hd, hm
    mov          edged, r7m
    mova           m15, [paramsq]
    add           lpfq, wq
    mova            m9, [pd_0xffff]
    lea             t1, [rsp+wq*2+44]
    mova           m10, [pd_34816]
    add           dstq, wq
    lea             t3, [rsp+wq*4+400*24+40]
    mova           m11, [pd_0xf00801c7]
    lea             t4, [rsp+wq*2+400*52+40]
    mova           m12, [base+pd_0xf00800a4]
    neg             wq
    pshuflw        m13, m15, q0000
    pshuflw        m14, m15, q2222
    pshufhw        m15, m15, q1010
    punpcklqdq     m13, m13 ; s0
    punpcklqdq     m14, m14 ; s1
    punpckhqdq     m15, m15 ; w0 w1
    pxor            m6, m6
    psllw          m15, 2
 DEFINE_ARGS dst, stride, left, lpf, _, h, edge, _, _, _, w
 %define lpfm [rsp]
%else
    mov             r1, [rstk+stack_offset+28] ; params
    LEA             r6, $$
    mova            m2, [r1]
    add           lpfm, wq
    lea             t1, [rsp+extra_stack+wq*2+52]
    add           dstq, wq
    lea             t3, [rsp+extra_stack+wq*4+400*24+48]
    mov           dstm, dstq
    lea             t4, [rsp+extra_stack+wq*2+400*52+48]
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
    sub             wd, 2
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
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_ssse3).top_fixup
%else
    mov             wq, w0m
    call mangle(private_prefix %+ _sgr_filter_5x5_8bpc_ssse3).top_fixup_loop
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
    lea             wq, [r4-2]
%else
    mov             wq, w0m
    mov         hvsrcm, lpfq
%endif
    lea             t2, [t1+400*12]
.top_fixup_loop:
    mova            m0, [t1+wq*2+400* 0]
    mova            m1, [t1+wq*2+400* 2]
    mova            m2, [t1+wq*2+400* 4]
    paddw           m0, m0
    mova            m3, [t1+wq*2+400* 6]
    paddd           m1, m1
    mova            m4, [t1+wq*2+400* 8]
    paddd           m2, m2
    mova            m5, [t1+wq*2+400*10]
    mova [t2+wq*2+400* 0], m0
    mova [t2+wq*2+400* 2], m1
    mova [t2+wq*2+400* 4], m2
    mova [t2+wq*2+400* 6], m3
    mova [t2+wq*2+400* 8], m4
    mova [t2+wq*2+400*10], m5
    add             wq, 8
    jl .top_fixup_loop
    movif32         t3, t3m
    movif32         t4, t4m
    call .v0
    jmp .main
.extend_right:
%assign stack_offset stack_offset+8
%assign calloff 8
%if ARCH_X86_64
    SWAP            m8, m6
%endif
    movd            m1, wd
    movd            m3, [lpfq-1]
    pshufb          m1, m8
    pshufb          m3, m8
    psubb           m2, [base+pb_1], m1
    pcmpgtb         m2, [base+pb_0to15]
    pand            m5, m2
    pandn           m2, m3
    por             m5, m2
%if ARCH_X86_64
    SWAP            m6, m8
%endif
    ret
%assign stack_offset stack_offset-4
%assign calloff 4
.h: ; horizontal boxsum
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
 %define leftq r4
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 13
    jmp .h_main
.h_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, [base+sgr_lshuf5]
    jmp .h_main
.h_top:
%if ARCH_X86_64
    lea             wq, [r4-2]
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .h_extend_left
    movif32         wq, w0m
.h_loop:
    movu            m5, [lpfq+wq-1]
.h_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
%if ARCH_X86_32
    pxor            m8, m8
%else
    SWAP            m8, m6
%endif
    jnz .h_have_right
    cmp             wd, -10
    jl .h_have_right
    call .extend_right
.h_have_right:
    punpcklbw       m4, m5, m8
    punpckhbw       m5, m8
    palignr         m3, m5, m4, 2
    palignr         m0, m5, m4, 4
    paddw           m1, m3, m0
    punpcklwd       m2, m3, m0
    pmaddwd         m2, m2
    punpckhwd       m3, m0
    pmaddwd         m3, m3
    palignr         m0, m5, m4, 6
    paddw           m1, m0             ; sum3
    punpcklwd       m7, m0, m8
    pmaddwd         m7, m7
    punpckhwd       m0, m8
    pmaddwd         m0, m0
%if ARCH_X86_64
    SWAP            m6, m8
%endif
    paddd           m2, m7             ; sumsq3
    palignr         m5, m4, 8
    punpcklwd       m7, m5, m4
    paddw           m8, m4, m5
    pmaddwd         m7, m7
    punpckhwd       m5, m4
    pmaddwd         m5, m5
    paddd           m3, m0
    mova [t1+wq*2+400* 6], m1
    mova [t1+wq*2+400* 8], m2
    mova [t1+wq*2+400*10], m3
    paddw           m8, m1             ; sum5
    paddd           m7, m2             ; sumsq5
    paddd           m5, m3
    mova [t1+wq*2+400* 0], m8
    mova [t1+wq*2+400* 2], m7
    mova [t1+wq*2+400* 4], m5
    add             wq, 8
    jl .h_loop
    ret
ALIGN function_align
.hv0: ; horizontal boxsum + vertical boxsum + ab3 (even rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv0_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 13
    jmp .hv0_main
.hv0_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, [base+sgr_lshuf5]
    jmp .hv0_main
.hv0_bottom:
%if ARCH_X86_64
    lea             wq, [r4-2]
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
    movu            m5, [lpfq+wq-1]
.hv0_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
%if ARCH_X86_32
    pxor            m8, m8
%else
    SWAP            m8, m6
%endif
    jnz .hv0_have_right
    cmp             wd, -10
    jl .hv0_have_right
    call .extend_right
.hv0_have_right:
    punpcklbw       m4, m5, m8
    punpckhbw       m5, m8
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
    punpcklwd       m7, m0, m8
    pmaddwd         m7, m7
    punpckhwd       m0, m8
%if ARCH_X86_64
    SWAP            m6, m8
%endif
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
    mova [t3+wq*4+400*8+ 8], m8
    mova [t3+wq*4+400*0+ 8], m7
    mova [t3+wq*4+400*0+24], m5
    paddw           m8, [t1+wq*2+400* 0]
    paddd           m7, [t1+wq*2+400* 2]
    paddd           m5, [t1+wq*2+400* 4]
    mova [t1+wq*2+400* 0], m8
    mova [t1+wq*2+400* 2], m7
    mova [t1+wq*2+400* 4], m5
    paddw           m0, m1, [t1+wq*2+400* 6]
    paddd           m4, m2, [t1+wq*2+400* 8]
    paddd           m5, m3, [t1+wq*2+400*10]
    mova [t1+wq*2+400* 6], m1
    mova [t1+wq*2+400* 8], m2
    mova [t1+wq*2+400*10], m3
    paddw           m1, m0, [t2+wq*2+400* 6]
    paddd           m2, m4, [t2+wq*2+400* 8]
    paddd           m3, m5, [t2+wq*2+400*10]
    mova [t2+wq*2+400* 6], m0
    mova [t2+wq*2+400* 8], m4
    mova [t2+wq*2+400*10], m5
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; a3 * 9
    paddd           m5, m3
    punpcklwd       m0, m1, m7         ; b3
    pmaddwd         m2, m0, m0
    punpckhwd       m1, m7
    pmaddwd         m3, m1, m1
%if ARCH_X86_64
    SWAP            m7, m6
%endif
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
    mova [t4+wq*2+400*2+ 4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova [t3+wq*4+400*4+ 8], m0
    mova [t3+wq*4+400*4+24], m1
    add             wq, 8
    jl .hv0_loop
    ret
ALIGN function_align
.hv1: ; horizontal boxsums + vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov         hvsrcm, lpfq
%endif
    test         edgeb, 1 ; LR_HAVE_LEFT
    jz .hv1_extend_left
    movif32      leftq, leftm
    movddup         m4, [leftq-4]
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    add         leftmp, 4
    palignr         m5, m4, 13
    jmp .hv1_main
.hv1_extend_left:
    movif32         wq, w0m
    mova            m5, [lpfq+wq+2]
    pshufb          m5, [base+sgr_lshuf5]
    jmp .hv1_main
.hv1_bottom:
%if ARCH_X86_64
    lea             wq, [r4-2]
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
    movu            m5, [lpfq+wq-1]
.hv1_main:
    test         edgeb, 2 ; LR_HAVE_RIGHT
%if ARCH_X86_32
    pxor            m8, m8
%else
    SWAP            m8, m6
%endif
    jnz .hv1_have_right
    cmp             wd, -10
    jl .hv1_have_right
    call .extend_right
.hv1_have_right:
    punpcklbw       m4, m5, m8
    punpckhbw       m5, m8
    palignr         m7, m5, m4, 2
    palignr         m3, m5, m4, 4
    paddw           m2, m7, m3
    punpcklwd       m0, m7, m3
    pmaddwd         m0, m0
    punpckhwd       m7, m3
    pmaddwd         m7, m7
    palignr         m3, m5, m4, 6
    paddw           m2, m3             ; h sum3
    punpcklwd       m1, m3, m8
    pmaddwd         m1, m1
    punpckhwd       m3, m8
%if ARCH_X86_64
    SWAP            m6, m8
%endif
    pmaddwd         m3, m3
    paddd           m0, m1             ; h sumsq3
    palignr         m5, m4, 8
    punpckhwd       m1, m4, m5
    paddw           m8, m4, m5
    pmaddwd         m1, m1
    punpcklwd       m4, m5
    pmaddwd         m4, m4
    paddd           m7, m3
    paddw           m5, m2, [t2+wq*2+400* 6]
    mova [t2+wq*2+400* 6], m2
    paddw           m8, m2             ; h sum5
    paddd           m2, m0, [t2+wq*2+400* 8]
    paddd           m3, m7, [t2+wq*2+400*10]
    mova [t2+wq*2+400* 8], m0
    mova [t2+wq*2+400*10], m7
    paddd           m4, m0             ; h sumsq5
    paddd           m1, m7
    pslld           m0, m2, 3
    pslld           m7, m3, 3
    paddd           m2, m0             ; a3 * 9
    paddd           m3, m7
%if ARCH_X86_32
    mova      [esp+20], m8
    pxor            m8, m8
%else
    SWAP            m8, m6
%endif
    punpcklwd       m0, m5, m8         ; b3
    pmaddwd         m7, m0, m0
    punpckhwd       m5, m8
    pmaddwd         m8, m5, m5
    psubd           m2, m7             ; p3
    psubd           m3, m8
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
    mova [t4+wq*2+400*4+ 4], m8
    mova [t3+wq*4+400*8+ 8], m0
    mova [t3+wq*4+400*8+24], m5
%if ARCH_X86_32
    mova            m8, [esp+20]
%else
    SWAP            m6, m8
    pxor            m6, m6
%endif
    paddw           m5, m8, [t2+wq*2+400*0]
    paddd           m2, m4, [t2+wq*2+400*2]
    paddd           m3, m1, [t2+wq*2+400*4]
    paddw           m5, [t1+wq*2+400*0]
    paddd           m2, [t1+wq*2+400*2]
    paddd           m3, [t1+wq*2+400*4]
    mova [t2+wq*2+400*0], m8
    pslld           m0, m2, 4
    mova [t2+wq*2+400*2], m4
    pslld           m8, m3, 4
    mova [t2+wq*2+400*4], m1
    pslld           m4, m2, 3
    paddd           m2, m0
    pslld           m7, m3, 3
    paddd           m3, m8
    paddd           m2, m4             ; a5 * 25
    paddd           m3, m7
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    punpcklwd       m0, m5, m7         ; b5
    pmaddwd         m4, m0, m0
    punpckhwd       m5, m7
    pmaddwd         m1, m5, m5
%if ARCH_X86_64
    SWAP            m7, m6
%endif
    psubd           m2, m4             ; p5
    psubd           m3, m1
    MULLD           m2, m13, m7        ; p5 * s0
    MULLD           m3, m13, m7
    pmaddwd         m0, m12            ; b5 * 164
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
    mova   [t4+wq*2+4], m1
    psrld           m0, 12
    psrld           m5, 12
    mova  [t3+wq*4+ 8], m0
    mova  [t3+wq*4+24], m5
    add             wq, 8
    jl .hv1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.v0: ; vertical boxsums + ab3 (even rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov             wd, w0m
%endif
.v0_loop:
    mova            m0, [t1+wq*2+400* 6]
    mova            m4, [t1+wq*2+400* 8]
    mova            m5, [t1+wq*2+400*10]
    paddw           m0, m0
    paddd           m4, m4
    paddd           m5, m5
    paddw           m1, m0, [t2+wq*2+400* 6]
    paddd           m2, m4, [t2+wq*2+400* 8]
    paddd           m3, m5, [t2+wq*2+400*10]
    mova [t2+wq*2+400* 6], m0
    mova [t2+wq*2+400* 8], m4
    mova [t2+wq*2+400*10], m5
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; a3 * 9
    paddd           m5, m3
    punpcklwd       m0, m1, m7         ; b3
    pmaddwd         m2, m0, m0
    punpckhwd       m1, m7
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p3
    psubd           m5, m3
%if ARCH_X86_64
    SWAP            m7, m6
%endif
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
    mova [t4+wq*2+400*2+4], m3
    psrld           m0, 12
    psrld           m1, 12
    mova            m3, [t1+wq*2+400*0]
    mova            m4, [t1+wq*2+400*2]
    mova            m5, [t1+wq*2+400*4]
    mova [t3+wq*4+400*8+ 8], m3
    mova [t3+wq*4+400*0+ 8], m4
    mova [t3+wq*4+400*0+24], m5
    paddw           m3, m3 ; cc5
    paddd           m4, m4
    paddd           m5, m5
    mova [t1+wq*2+400*0], m3
    mova [t1+wq*2+400*2], m4
    mova [t1+wq*2+400*4], m5
    mova [t3+wq*4+400*4+ 8], m0
    mova [t3+wq*4+400*4+24], m1
    add             wq, 8
    jl .v0_loop
    ret
.v1: ; vertical boxsums + ab (odd rows)
%if ARCH_X86_64
    lea             wq, [r4-2]
%else
    mov             wd, w0m
%endif
.v1_loop:
    mova            m4, [t1+wq*2+400* 6]
    mova            m5, [t1+wq*2+400* 8]
    mova            m7, [t1+wq*2+400*10]
    paddw           m1, m4, [t2+wq*2+400* 6]
    paddd           m2, m5, [t2+wq*2+400* 8]
    paddd           m3, m7, [t2+wq*2+400*10]
    mova [t2+wq*2+400* 6], m4
    mova [t2+wq*2+400* 8], m5
    mova [t2+wq*2+400*10], m7
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    pslld           m4, m2, 3
    pslld           m5, m3, 3
    paddd           m4, m2             ; ((a3 + 8) >> 4) * 9
    paddd           m5, m3
    punpcklwd       m0, m1, m7         ; b3
    pmaddwd         m2, m0, m0
    punpckhwd       m1, m7
    pmaddwd         m3, m1, m1
    psubd           m4, m2             ; p3
    psubd           m5, m3
%if ARCH_X86_64
    SWAP            m7, m6
%endif
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
    mova [t4+wq*2+400*4+4], m3
    psrld           m0, 12
    psrld           m8, m1, 12
    mova            m4, [t3+wq*4+400*8+ 8]
    mova            m5, [t3+wq*4+400*0+ 8]
    mova            m7, [t3+wq*4+400*0+24]
    paddw           m1, m4, [t2+wq*2+400*0]
    paddd           m2, m5, [t2+wq*2+400*2]
    paddd           m3, m7, [t2+wq*2+400*4]
    paddw           m1, [t1+wq*2+400*0]
    paddd           m2, [t1+wq*2+400*2]
    paddd           m3, [t1+wq*2+400*4]
    mova [t2+wq*2+400*0], m4
    mova [t2+wq*2+400*2], m5
    mova [t2+wq*2+400*4], m7
    pslld           m4, m2, 4
    mova [t3+wq*4+400*8+ 8], m0
    pslld           m5, m3, 4
    mova [t3+wq*4+400*8+24], m8
    pslld           m7, m2, 3
    paddd           m2, m4
    pslld           m8, m3, 3
    paddd           m3, m5
    paddd           m2, m7             ; a5 * 25
    paddd           m3, m8
%if ARCH_X86_32
    pxor            m7, m7
%else
    SWAP            m7, m6
%endif
    punpcklwd       m0, m1, m7         ; b5
    pmaddwd         m4, m0, m0
    punpckhwd       m1, m7
    pmaddwd         m5, m1, m1
    psubd           m2, m4             ; p5
    psubd           m3, m5
%if ARCH_X86_64
    SWAP            m7, m6
%endif
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
    mova   [t4+wq*2+4], m4
    psrld           m0, 12
    psrld           m1, 12
    mova  [t3+wq*4+ 8], m0
    mova  [t3+wq*4+24], m1
    add             wq, 8
    jl .v1_loop
    mov            r10, t2
    mov             t2, t1
    mov             t1, r10
    ret
.prep_n: ; initial neighbor setup
    movif64         wq, r4
    movif32         wd, w1m
.prep_n_loop:
    movu            m0, [t4+wq*2+400*0+ 2]
    movu            m1, [t3+wq*4+400*0+ 4]
    movu            m2, [t3+wq*4+400*0+20]
    movu            m7, [t4+wq*2+400*0+ 4]
    movu            m8, [t3+wq*4+400*0+ 8]
    paddw           m3, m0, [t4+wq*2+400*0+ 0]
    paddd           m4, m1, [t3+wq*4+400*0+ 0]
    paddd           m5, m2, [t3+wq*4+400*0+16]
    paddw           m3, m7
    paddd           m4, m8
    movu            m7, [t3+wq*4+400*0+24]
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
    mova [t4+wq*2+400* 6+ 0], m0
    mova [t3+wq*4+400*12+ 0], m1
    mova [t3+wq*4+400*12+16], m2
    movu            m0, [t4+wq*2+400*2+ 4]
    movu            m1, [t3+wq*4+400*4+ 8]
    movu            m2, [t3+wq*4+400*4+24]
    movu            m3, [t4+wq*2+400*2+ 2]
    movu            m4, [t3+wq*4+400*4+ 4]
    movu            m5, [t3+wq*4+400*4+20]
    paddw           m0, [t4+wq*2+400*2+ 0]
    paddd           m1, [t3+wq*4+400*4+ 0]
    paddd           m2, [t3+wq*4+400*4+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                ; a3[-1] 444
    pslld           m4, 2                ; b3[-1] 444
    pslld           m5, 2
    psubw           m3, m0               ; a3[-1] 343
    psubd           m4, m1               ; b3[-1] 343
    psubd           m5, m2
    mova [t4+wq*2+400* 8+ 0], m3
    mova [t3+wq*4+400*16+ 0], m4
    mova [t3+wq*4+400*16+16], m5
    movu            m0, [t4+wq*2+400*4+ 4]
    movu            m1, [t3+wq*4+400*8+ 8]
    movu            m2, [t3+wq*4+400*8+24]
    movu            m3, [t4+wq*2+400*4+ 2]
    movu            m4, [t3+wq*4+400*8+ 4]
    movu            m5, [t3+wq*4+400*8+20]
    paddw           m0, [t4+wq*2+400*4+ 0]
    paddd           m1, [t3+wq*4+400*8+ 0]
    paddd           m2, [t3+wq*4+400*8+16]
    paddw           m3, m0
    paddd           m4, m1
    paddd           m5, m2
    psllw           m3, 2                 ; a3[ 0] 444
    pslld           m4, 2                 ; b3[ 0] 444
    pslld           m5, 2
    mova [t4+wq*2+400*10+ 0], m3
    mova [t3+wq*4+400*20+ 0], m4
    mova [t3+wq*4+400*20+16], m5
    psubw           m3, m0                ; a3[ 0] 343
    psubd           m4, m1                ; b3[ 0] 343
    psubd           m5, m2
    mova [t4+wq*2+400*12+ 0], m3
    mova [t3+wq*4+400*24+ 0], m4
    mova [t3+wq*4+400*24+16], m5
    add             wq, 8
    jl .prep_n_loop
    ret
ALIGN function_align
.n0: ; neighbor + output (even rows)
    movif64         wq, r4
    movif32         wd, w1m
.n0_loop:
    movu            m0, [t4+wq*2+ 4]
    movu            m2, [t4+wq*2+ 2]
    paddw           m0, [t4+wq*2+ 0]
    paddw           m0, m2
    paddw           m2, m0
    psllw           m0, 2
    paddw           m0, m2               ; a5
    movu            m4, [t3+wq*4+ 8]
    movu            m5, [t3+wq*4+24]
    movu            m1, [t3+wq*4+ 4]
    movu            m3, [t3+wq*4+20]
    paddd           m4, [t3+wq*4+ 0]
    paddd           m5, [t3+wq*4+16]
    paddd           m4, m1
    paddd           m5, m3
    paddd           m1, m4
    paddd           m3, m5
    pslld           m4, 2
    pslld           m5, 2
    paddd           m4, m1               ; b5
    paddd           m5, m3
    movu            m2, [t4+wq*2+400* 6]
    paddw           m2, m0
    mova [t4+wq*2+400* 6], m0
    paddd           m0, m4, [t3+wq*4+400*12+ 0]
    paddd           m1, m5, [t3+wq*4+400*12+16]
    mova [t3+wq*4+400*12+ 0], m4
    mova [t3+wq*4+400*12+16], m5
    mova [rsp+16+ARCH_X86_32*4], m1
    movu            m3, [t4+wq*2+400*2+4]
    movu            m5, [t4+wq*2+400*2+2]
    paddw           m3, [t4+wq*2+400*2+0]
    paddw           m5, m3
    psllw           m5, 2                ; a3[ 1] 444
    psubw           m4, m5, m3           ; a3[ 1] 343
    movu            m3, [t4+wq*2+400* 8]
    paddw           m3, [t4+wq*2+400*10]
    paddw           m3, m4
    mova [t4+wq*2+400* 8], m4
    mova [t4+wq*2+400*10], m5
    movu            m1, [t3+wq*4+400*4+ 8]
    movu            m5, [t3+wq*4+400*4+ 4]
    movu            m7, [t3+wq*4+400*4+24]
    movu            m8, [t3+wq*4+400*4+20]
    paddd           m1, [t3+wq*4+400*4+ 0]
    paddd           m7, [t3+wq*4+400*4+16]
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
    paddd           m1, m4, [t3+wq*4+400*16+ 0]
    paddd           m7, m8, [t3+wq*4+400*16+16]
    paddd           m1, [t3+wq*4+400*20+ 0]
    paddd           m7, [t3+wq*4+400*20+16]
    mova [t3+wq*4+400*16+ 0], m4
    mova [t3+wq*4+400*16+16], m8
    mova [t3+wq*4+400*20+ 0], m5
%if ARCH_X86_32
    mova            m8, [esp+52]
%else
    SWAP            m8, m6
    pxor            m6, m6
%endif
    mova [t3+wq*4+400*20+16], m8
    mova [rsp+32+ARCH_X86_32*4], m7
    movq            m4, [dstq+wq]
    punpcklbw       m4, m6
    punpcklwd       m5, m4, m6
    punpcklwd       m7, m2, m6
    pmaddwd         m7, m5               ; a5 * src
    punpcklwd       m8, m3, m6
    pmaddwd         m8, m5               ; a3 * src
    punpckhwd       m5, m4, m6
    punpckhwd       m2, m6
    pmaddwd         m2, m5
    punpckhwd       m3, m6
    pmaddwd         m3, m5
    psubd           m0, m7               ; b5 - a5 * src + (1 << 8) - (src << 13)
    psubd           m1, m8               ; b3 - a3 * src + (1 << 8) - (src << 13)
    psrld           m0, 9
    pslld           m1, 7
    pand            m0, m9
    pandn           m8, m9, m1
    por             m0, m8
    mova            m1, [rsp+16+ARCH_X86_32*4]
    psubd           m1, m2
    mova            m2, [rsp+32+ARCH_X86_32*4]
    psubd           m2, m3
    mova            m3, [base+pd_4096]
    psrld           m1, 9
    pslld           m2, 7
    pand            m1, m9
    pandn           m5, m9, m2
    por             m1, m5
    pmaddwd         m0, m15
    pmaddwd         m1, m15
    paddd           m0, m3
    paddd           m1, m3
    psrad           m0, 13
    psrad           m1, 13
    packssdw        m0, m1
    paddw           m0, m4
    packuswb        m0, m0
    movq     [dstq+wq], m0
    add             wq, 8
    jl .n0_loop
    add           dstq, stridemp
    ret
ALIGN function_align
.n1: ; neighbor + output (odd rows)
    movif64         wq, r4
    movif32         wd, w1m
.n1_loop:
    movu            m3, [t4+wq*2+400*4+4]
    movu            m5, [t4+wq*2+400*4+2]
    paddw           m3, [t4+wq*2+400*4+0]
    paddw           m5, m3
    psllw           m5, 2                ; a3[ 1] 444
    psubw           m4, m5, m3           ; a3[ 1] 343
    paddw           m3, m4, [t4+wq*2+400*12]
    paddw           m3, [t4+wq*2+400*10]
    mova [t4+wq*2+400*10], m5
    mova [t4+wq*2+400*12], m4
    movu            m1, [t3+wq*4+400*8+ 8]
    movu            m5, [t3+wq*4+400*8+ 4]
    movu            m7, [t3+wq*4+400*8+24]
    movu            m8, [t3+wq*4+400*8+20]
    paddd           m1, [t3+wq*4+400*8+ 0]
    paddd           m7, [t3+wq*4+400*8+16]
    paddd           m5, m1
    paddd           m8, m7
    pslld           m5, 2                ; b3[ 1] 444
    pslld           m8, 2
    psubd           m4, m5, m1           ; b3[ 1] 343
    psubd           m0, m8, m7
    paddd           m1, m4, [t3+wq*4+400*24+ 0]
    paddd           m7, m0, [t3+wq*4+400*24+16]
    paddd           m1, [t3+wq*4+400*20+ 0]
    paddd           m7, [t3+wq*4+400*20+16]
    mova [t3+wq*4+400*20+ 0], m5
    mova [t3+wq*4+400*20+16], m8
    mova [t3+wq*4+400*24+ 0], m4
    mova [t3+wq*4+400*24+16], m0
    movq            m5, [dstq+wq]
    mova            m2, [t4+wq*2+400* 6]
    punpcklbw       m5, m6
    punpcklwd       m4, m5, m6
    punpcklwd       m8, m2, m6
    pmaddwd         m8, m4               ; a5 * src
    punpcklwd       m0, m3, m6
    pmaddwd         m0, m4               ; a3 * src
    punpckhwd       m4, m5, m6
    punpckhwd       m2, m6
    pmaddwd         m2, m4
    punpckhwd       m3, m6
    pmaddwd         m3, m4
    psubd           m1, m0               ; b3 - a3 * src + (1 << 8) - (src << 13)
    mova            m0, [t3+wq*4+400*12+ 0]
    psubd           m0, m8               ; b5 - a5 * src + (1 << 8) - (src << 13)
    mova            m4, [t3+wq*4+400*12+16]
    psubd           m4, m2
    psubd           m7, m3
    pslld           m1, 7
    psrld           m0, 8
    psrld           m4, 8
    pslld           m7, 7
    pandn           m3, m9, m1
    pand            m0, m9
    por             m0, m3
    pand            m4, m9
    pandn           m2, m9, m7
    por             m2, m4
    mova            m1, [base+pd_4096]
    pmaddwd         m0, m15
    pmaddwd         m2, m15
    paddd           m0, m1
    paddd           m2, m1
    psrad           m0, 13
    psrad           m2, 13
    packssdw        m0, m2
    paddw           m0, m5
    packuswb        m0, m0
    movq     [dstq+wq], m0
    add             wq, 8
    jl .n1_loop
    add           dstq, stridemp
    movif32       dstm, dstq
    ret
