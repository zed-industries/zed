; Copyright © 2019, VideoLAN and dav1d authors
; Copyright © 2019, Two Orioles, LLC
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

SECTION_RODATA 64 ; avoids cacheline splits

min_prob:  dw 60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 4, 0
pw_0xff00: times 8 dw 0xff00
pw_32:     times 8 dw 32

%if ARCH_X86_64
%define resp   resq
%define movp   movq
%define c_shuf q3333
%macro DECODE_SYMBOL_ADAPT_INIT 0-1
%endmacro
%else
%define resp   resd
%define movp   movd
%define c_shuf q1111
%macro DECODE_SYMBOL_ADAPT_INIT 0-1 0 ; hi_tok
    mov            t0, r0m
    mov            t1, r1m
%if %1 == 0
    mov            t2, r2m
%endif
%if STACK_ALIGNMENT >= 16
    sub           esp, 40-%1*4
%else
    mov           eax, esp
    and           esp, ~15
    sub           esp, 40-%1*4
    mov         [esp], eax
%endif
%endmacro
%endif

struc msac
    .buf:        resp 1
    .end:        resp 1
    .dif:        resp 1
    .rng:        resd 1
    .cnt:        resd 1
    .update_cdf: resd 1
endstruc

%define m(x, y) mangle(private_prefix %+ _ %+ x %+ y)

SECTION .text

%if WIN64
DECLARE_REG_TMP 0, 1, 2, 3, 4, 5, 7, 3, 8
%define buf rsp+stack_offset+8 ; shadow space
%elif UNIX64
DECLARE_REG_TMP 0, 1, 2, 3, 4, 5, 7, 0, 8
%define buf rsp-40 ; red zone
%else
DECLARE_REG_TMP 2, 3, 4, 1, 5, 6, 5, 2, 3
%define buf esp+8
%endif

INIT_XMM sse2
cglobal msac_decode_symbol_adapt4, 0, 6, 6
    DECODE_SYMBOL_ADAPT_INIT
    LEA           rax, pw_0xff00
    movd           m2, [t0+msac.rng]
    movq           m1, [t1]
    movp           m3, [t0+msac.dif]
    mov           t3d, [t0+msac.update_cdf]
    mov           t4d, t2d
    not            t2     ; -(n_symbols + 1)
    pshuflw        m2, m2, q0000
    movd     [buf+12], m2
    pand           m2, [rax]
    mova           m0, m1
    psrlw          m1, 6
    psllw          m1, 7
    pmulhuw        m1, m2
    movq           m2, [rax+t2*2]
    pshuflw        m3, m3, c_shuf
    paddw          m1, m2
    mova     [buf+16], m1
    psubusw        m1, m3
    pxor           m2, m2
    pcmpeqw        m1, m2 ; c >= v
    pmovmskb      eax, m1
    test          t3d, t3d
    jz .renorm ; !allow_update_cdf

; update_cdf:
    movzx         t3d, word [t1+t4*2] ; count
    pcmpeqw        m2, m2
    mov           t2d, t3d
    shr           t3d, 4
    cmp           t4d, 3
    sbb           t3d, -5 ; (count >> 4) + (n_symbols > 2) + 4
    cmp           t2d, 32
    adc           t2d, 0  ; count + (count < 32)
    movd           m3, t3d
    pavgw          m2, m1 ; i >= val ? -1 : 32768
    psubw          m2, m0 ; for (i = 0; i < val; i++)
    psubw          m0, m1 ;     cdf[i] += (32768 - cdf[i]) >> rate;
    psraw          m2, m3 ; for (; i < n_symbols; i++)
    paddw          m0, m2 ;     cdf[i] += ((  -1 - cdf[i]) >> rate) + 1;
    movq         [t1], m0
    mov     [t1+t4*2], t2w

.renorm:
    tzcnt         eax, eax
    mov            t4, [t0+msac.dif]
    movzx         t1d, word [buf+rax+16] ; v
    movzx         t2d, word [buf+rax+14] ; u
    shr           eax, 1
.renorm2:
%if ARCH_X86_64 == 0
%if STACK_ALIGNMENT >= 16
    add           esp, 40
%else
    mov           esp, [esp]
%endif
%endif
    not            t4
    sub           t2d, t1d ; rng
    shl            t1, gprsize*8-16
    add            t4, t1  ; ~dif
.renorm3:
    mov           t1d, [t0+msac.cnt]
    movifnidn      t7, t0
.renorm4:
    bsr           ecx, t2d
    xor           ecx, 15  ; d
.renorm5:
    shl           t2d, cl
    shl            t4, cl
    mov [t7+msac.rng], t2d
    not            t4
    sub           t1d, ecx
    jae .end ; no refill required

; refill:
    mov            t2, [t7+msac.buf]
    mov           rcx, [t7+msac.end]
%if ARCH_X86_64 == 0
    push           t5
%endif
    lea            t5, [t2+gprsize]
    cmp            t5, rcx
    ja .refill_eob
    mov            t2, [t2]
    lea           ecx, [t1+23]
    add           t1d, 16
    shr           ecx, 3   ; shift_bytes
    bswap          t2
    sub            t5, rcx
    shl           ecx, 3   ; shift_bits
    shr            t2, cl
    sub           ecx, t1d ; shift_bits - 16 - cnt
    mov           t1d, gprsize*8-16
    shl            t2, cl
    mov [t7+msac.buf], t5
    sub           t1d, ecx ; cnt + gprsize*8 - shift_bits
    xor            t4, t2
%if ARCH_X86_64 == 0
    pop            t5
%endif
.end:
    mov [t7+msac.cnt], t1d
    mov [t7+msac.dif], t4
    RET
.refill_eob: ; avoid overreading the input buffer
    mov            t5, rcx
    mov           ecx, gprsize*8-24
    sub           ecx, t1d ; c
.refill_eob_loop:
    cmp            t2, t5
    jae .refill_eob_end    ; eob reached
    movzx         t1d, byte [t2]
    inc            t2
    shl            t1, cl
    xor            t4, t1
    sub           ecx, 8
    jge .refill_eob_loop
.refill_eob_end:
    mov           t1d, gprsize*8-24
%if ARCH_X86_64 == 0
    pop            t5
%endif
    sub           t1d, ecx
    mov [t7+msac.buf], t2
    mov [t7+msac.dif], t4
    mov [t7+msac.cnt], t1d
    RET

cglobal msac_decode_symbol_adapt8, 0, 6, 6
    DECODE_SYMBOL_ADAPT_INIT
    LEA           rax, pw_0xff00
    movd           m2, [t0+msac.rng]
    mova           m1, [t1]
    movp           m3, [t0+msac.dif]
    mov           t3d, [t0+msac.update_cdf]
    mov           t4d, t2d
    not            t2
    pshuflw        m2, m2, q0000
    movd     [buf+12], m2
    punpcklqdq     m2, m2
    mova           m0, m1
    psrlw          m1, 6
    pand           m2, [rax]
    psllw          m1, 7
    pmulhuw        m1, m2
    movu           m2, [rax+t2*2]
    pshuflw        m3, m3, c_shuf
    paddw          m1, m2
    punpcklqdq     m3, m3
    mova     [buf+16], m1
    psubusw        m1, m3
    pxor           m2, m2
    pcmpeqw        m1, m2
    pmovmskb      eax, m1
    test          t3d, t3d
    jz m(msac_decode_symbol_adapt4, SUFFIX).renorm
    movzx         t3d, word [t1+t4*2]
    pcmpeqw        m2, m2
    mov           t2d, t3d
    shr           t3d, 4
    cmp           t4d, 3 ; may be called with n_symbols <= 2
    sbb           t3d, -5
    cmp           t2d, 32
    adc           t2d, 0
    movd           m3, t3d
    pavgw          m2, m1
    psubw          m2, m0
    psubw          m0, m1
    psraw          m2, m3
    paddw          m0, m2
    mova         [t1], m0
    mov     [t1+t4*2], t2w
    jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm

cglobal msac_decode_symbol_adapt16, 0, 6, 6
    DECODE_SYMBOL_ADAPT_INIT
    LEA           rax, pw_0xff00
    movd           m4, [t0+msac.rng]
    mova           m2, [t1]
    mova           m3, [t1+16]
    movp           m5, [t0+msac.dif]
    mov           t3d, [t0+msac.update_cdf]
    mov           t4d, t2d
    not            t2
%if WIN64
    sub           rsp, 48 ; need 36 bytes, shadow space is only 32
%endif
    pshuflw        m4, m4, q0000
    movd      [buf-4], m4
    punpcklqdq     m4, m4
    mova           m0, m2
    psrlw          m2, 6
    mova           m1, m3
    psrlw          m3, 6
    pand           m4, [rax]
    psllw          m2, 7
    psllw          m3, 7
    pmulhuw        m2, m4
    pmulhuw        m3, m4
    movu           m4, [rax+t2*2]
    pshuflw        m5, m5, c_shuf
    paddw          m2, m4
    psubw          m4, [rax-pw_0xff00+pw_32]
    punpcklqdq     m5, m5
    paddw          m3, m4
    mova        [buf], m2
    psubusw        m2, m5
    mova     [buf+16], m3
    psubusw        m3, m5
    pxor           m4, m4
    pcmpeqw        m2, m4
    pcmpeqw        m3, m4
    packsswb       m5, m2, m3
    pmovmskb      eax, m5
    test          t3d, t3d
    jz .renorm
    movzx         t3d, word [t1+t4*2]
    pcmpeqw        m4, m4
    mova           m5, m4
    lea           t2d, [t3+80] ; only support n_symbols > 2
    shr           t2d, 4
    cmp           t3d, 32
    adc           t3d, 0
    pavgw          m4, m2
    pavgw          m5, m3
    psubw          m4, m0
    psubw          m0, m2
    movd           m2, t2d
    psubw          m5, m1
    psubw          m1, m3
    psraw          m4, m2
    psraw          m5, m2
    paddw          m0, m4
    paddw          m1, m5
    mova         [t1], m0
    mova      [t1+16], m1
    mov     [t1+t4*2], t3w
.renorm:
    tzcnt         eax, eax
    mov            t4, [t0+msac.dif]
    movzx         t1d, word [buf+rax*2]
    movzx         t2d, word [buf+rax*2-2]
%if WIN64
    add           rsp, 48
%endif
    jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm2

cglobal msac_decode_bool_adapt, 0, 6, 0
    movifnidn      t1, r1mp
    movifnidn      t0, r0mp
    movzx         eax, word [t1]
    movzx         t3d, byte [t0+msac.rng+1]
    mov            t4, [t0+msac.dif]
    mov           t2d, [t0+msac.rng]
%if ARCH_X86_64
    mov           t5d, eax
%endif
    and           eax, ~63
    imul          eax, t3d
%if UNIX64
    mov            t6, t4
%endif
    shr           eax, 7
    add           eax, 4            ; v
    mov           t3d, eax
    shl           rax, gprsize*8-16 ; vw
    sub           t2d, t3d          ; r - v
    sub            t4, rax          ; dif - vw
    setb           al
    cmovb         t2d, t3d
    mov           t3d, [t0+msac.update_cdf]
%if UNIX64
    cmovb          t4, t6
%else
    cmovb          t4, [t0+msac.dif]
%endif
%if ARCH_X86_64 == 0
    movzx         eax, al
%endif
    not            t4
    test          t3d, t3d
    jz m(msac_decode_symbol_adapt4, SUFFIX).renorm3
%if UNIX64 == 0
    push           t6
%endif
    movzx         t6d, word [t1+2]
%if ARCH_X86_64 == 0
    push           t5
    movzx         t5d, word [t1]
%endif
    movifnidn      t7, t0
    lea           ecx, [t6+64]
    cmp           t6d, 32
    adc           t6d, 0
    mov        [t1+2], t6w
    imul          t6d, eax, -32769
    shr           ecx, 4   ; rate
    add           t6d, t5d ; if (bit)
    sub           t5d, eax ;     cdf[0] -= ((cdf[0] - 32769) >> rate) + 1;
    sar           t6d, cl  ; else
    sub           t5d, t6d ;     cdf[0] -= cdf[0] >> rate;
    mov          [t1], t5w
%if WIN64
    mov           t1d, [t7+msac.cnt]
    pop            t6
    jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm4
%else
%if ARCH_X86_64 == 0
    pop            t5
    pop            t6
%endif
    jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm3
%endif

cglobal msac_decode_bool_equi, 0, 6, 0
    movifnidn      t0, r0mp
    mov           t1d, [t0+msac.rng]
    mov            t4, [t0+msac.dif]
    mov           t2d, t1d
    mov           t1b, 8
    mov            t3, t4
    mov           eax, t1d
    shr           t1d, 1            ; v
    shl           rax, gprsize*8-17 ; vw
    sub           t2d, t1d          ; r - v
    sub            t4, rax          ; dif - vw
    cmovb         t2d, t1d
    mov           t1d, [t0+msac.cnt]
    cmovb          t4, t3
    movifnidn      t7, t0
    mov           ecx, 0xbfff
    setb           al ; the upper 32 bits contains garbage but that's OK
    sub           ecx, t2d
    not            t4
    ; In this case of this function, (d =) 16 - clz(v) = 2 - (v >> 14)
    ;   i.e. (0 <= d <= 2) and v < (3 << 14)
    shr           ecx, 14           ; d
%if ARCH_X86_64 == 0
    movzx         eax, al
%endif
    jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm5

cglobal msac_decode_bool, 0, 6, 0
    movifnidn      t0, r0mp
    movifnidn     t1d, r1m
    movzx         eax, byte [t0+msac.rng+1] ; r >> 8
    mov            t4, [t0+msac.dif]
    mov           t2d, [t0+msac.rng]
    and           t1d, ~63
    imul          eax, t1d
    mov            t3, t4
    shr           eax, 7
    add           eax, 4            ; v
    mov           t1d, eax
    shl           rax, gprsize*8-16 ; vw
    sub           t2d, t1d          ; r - v
    sub            t4, rax          ; dif - vw
    cmovb         t2d, t1d
    cmovb          t4, t3
    setb           al
    not            t4
%if ARCH_X86_64 == 0
    movzx         eax, al
%endif
    jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm3

%macro HI_TOK 1 ; update_cdf
%if ARCH_X86_64 == 0
    mov           eax, -24
%endif
%%loop:
%if %1
    movzx         t2d, word [t1+3*2]
%endif
    mova           m1, m0
    pshuflw        m2, m2, q0000
    psrlw          m1, 6
    movd     [buf+12], m2
    pand           m2, m4
    psllw          m1, 7
    pmulhuw        m1, m2
%if ARCH_X86_64 == 0
    add           eax, 5
    mov       [buf+8], eax
%endif
    pshuflw        m3, m3, c_shuf
    paddw          m1, m5
    movq     [buf+16], m1
    psubusw        m1, m3
    pxor           m2, m2
    pcmpeqw        m1, m2
    pmovmskb      eax, m1
%if %1
    lea           ecx, [t2+80]
    pcmpeqw        m2, m2
    shr           ecx, 4
    cmp           t2d, 32
    adc           t2d, 0
    movd           m3, ecx
    pavgw          m2, m1
    psubw          m2, m0
    psubw          m0, m1
    psraw          m2, m3
    paddw          m0, m2
    movq         [t1], m0
    mov      [t1+3*2], t2w
%endif
    tzcnt         eax, eax
    movzx         ecx, word [buf+rax+16]
    movzx         t2d, word [buf+rax+14]
    not            t4
%if ARCH_X86_64
    add           t6d, 5
%endif
    sub           eax, 5   ; setup for merging the tok_br and tok branches
    sub           t2d, ecx
    shl           rcx, gprsize*8-16
    add            t4, rcx
    bsr           ecx, t2d
    xor           ecx, 15
    shl           t2d, cl
    shl            t4, cl
    movd           m2, t2d
    mov [t7+msac.rng], t2d
    not            t4
    sub           t5d, ecx
    jae %%end
    mov            t2, [t7+msac.buf]
    mov           rcx, [t7+msac.end]
%if UNIX64 == 0
    push           t8
%endif
    lea            t8, [t2+gprsize]
    cmp            t8, rcx
    ja %%refill_eob
    mov            t2, [t2]
    lea           ecx, [t5+23]
    add           t5d, 16
    shr           ecx, 3
    bswap          t2
    sub            t8, rcx
    shl           ecx, 3
    shr            t2, cl
    sub           ecx, t5d
    mov           t5d, gprsize*8-16
    shl            t2, cl
    mov [t7+msac.buf], t8
%if UNIX64 == 0
    pop            t8
%endif
    sub           t5d, ecx
    xor            t4, t2
%%end:
    movp           m3, t4
%if ARCH_X86_64
    add           t6d, eax ; CF = tok_br < 3 || tok == 15
    jnc %%loop
    lea           eax, [t6+30]
%else
    add           eax, [buf+8]
    jnc %%loop
    add           eax, 30
%if STACK_ALIGNMENT >= 16
    add           esp, 36
%else
    mov           esp, [esp]
%endif
%endif
    mov [t7+msac.dif], t4
    shr           eax, 1
    mov [t7+msac.cnt], t5d
    RET
%%refill_eob:
    mov            t8, rcx
    mov           ecx, gprsize*8-24
    sub           ecx, t5d
%%refill_eob_loop:
    cmp            t2, t8
    jae %%refill_eob_end
    movzx         t5d, byte [t2]
    inc            t2
    shl            t5, cl
    xor            t4, t5
    sub           ecx, 8
    jge %%refill_eob_loop
%%refill_eob_end:
%if UNIX64 == 0
    pop            t8
%endif
    mov           t5d, gprsize*8-24
    mov [t7+msac.buf], t2
    sub           t5d, ecx
    jmp %%end
%endmacro

cglobal msac_decode_hi_tok, 0, 7 + ARCH_X86_64, 6
    DECODE_SYMBOL_ADAPT_INIT 1
%if ARCH_X86_64 == 0 && PIC
    LEA            t2, min_prob+12*2
    %define base t2-(min_prob+12*2)
%else
    %define base 0
%endif
    movq           m0, [t1]
    movd           m2, [t0+msac.rng]
    mov           eax, [t0+msac.update_cdf]
    movq           m4, [base+pw_0xff00]
    movp           m3, [t0+msac.dif]
    movq           m5, [base+min_prob+12*2]
    mov            t4, [t0+msac.dif]
    mov           t5d, [t0+msac.cnt]
%if ARCH_X86_64
    mov           t6d, -24
%endif
    movifnidn      t7, t0
    test          eax, eax
    jz .no_update_cdf
    HI_TOK          1
.no_update_cdf:
    HI_TOK          0

%if ARCH_X86_64
INIT_YMM avx2
cglobal msac_decode_symbol_adapt16, 3, 6, 6
    lea           rax, [pw_0xff00]
    vpbroadcastw   m2, [t0+msac.rng]
    mova           m0, [t1]
    vpbroadcastw   m3, [t0+msac.dif+6]
    vbroadcasti128 m4, [rax]
    mov           t3d, [t0+msac.update_cdf]
    mov           t4d, t2d
    not            t2
    mov            r5, rsp
%if WIN64
    and           rsp, ~31
    sub           rsp, 40
%else
    and            r5, ~31
    %define buf r5-32
%endif
    psrlw          m1, m0, 6
    movd      [buf-4], xm2
    pand           m2, m4
    psllw          m1, 7
    pmulhuw        m1, m2
    paddw          m1, [rax+t2*2]
    mova        [buf], m1
    pmaxuw         m1, m3
    pcmpeqw        m1, m3
    pmovmskb      eax, m1
    test          t3d, t3d
    jz .renorm
    movzx         t3d, word [t1+t4*2]
    pcmpeqw        m2, m2
    lea           t2d, [t3+80]
    shr           t2d, 4
    cmp           t3d, 32
    adc           t3d, 0
    movd          xm3, t2d
    pavgw          m2, m1
    psubw          m2, m0
    psubw          m0, m1
    psraw          m2, xm3
    paddw          m0, m2
    mova         [t1], m0
    mov     [t1+t4*2], t3w
.renorm:
    tzcnt         eax, eax
    mov            t4, [t0+msac.dif]
    movzx         t1d, word [buf+rax-0]
    movzx         t2d, word [buf+rax-2]
    shr           eax, 1
%if WIN64
    mov           rsp, r5
%endif
    vzeroupper
    jmp m(msac_decode_symbol_adapt4, _sse2).renorm2
%endif
