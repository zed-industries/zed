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

%macro JMP_TABLE 2-*
 %xdefine %1_jmptable %%table
 %xdefine %%base mangle(private_prefix %+ _%1_avx2)
 %%table:
 %rep %0 - 1
    dd %%base %+ .%2 - %%table
  %rotate 1
 %endrep
%endmacro

%macro CDEF_FILTER_JMP_TABLE 1
JMP_TABLE cdef_filter_%1_8bpc, \
    d6k0, d6k1, d7k0, d7k1, \
    d0k0, d0k1, d1k0, d1k1, d2k0, d2k1, d3k0, d3k1, \
    d4k0, d4k1, d5k0, d5k1, d6k0, d6k1, d7k0, d7k1, \
    d0k0, d0k1, d1k0, d1k1
%endmacro

SECTION_RODATA 32

pd_47130256:   dd  4,  7,  1,  3,  0,  2,  5,  6
blend_4x4:     dd 0x00, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00
               dd 0x80, 0x00, 0x00
blend_4x8_0:   dd 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80
blend_4x8_1:   dd 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80
               dd 0x00, 0x00
blend_4x8_2:   dd 0x0000, 0x8080, 0x8080, 0x8080, 0x8080, 0x8080, 0x8080, 0x8080
               dd 0x0000
blend_4x8_3:   dd 0x0000, 0x0000, 0x8080, 0x8080, 0x8080, 0x8080, 0x8080, 0x8080
               dd 0x0000, 0x0000
blend_8x8_0:   dq 0x00, 0x00, 0x80, 0x80, 0x80, 0x80
blend_8x8_1:   dq 0x0000, 0x0000, 0x8080, 0x8080, 0x8080, 0x8080, 0x0000, 0x0000
div_table:     dd 840, 420, 280, 210, 168, 140, 120, 105, 420, 210, 140, 105
shufw_6543210x:db 12, 13, 10, 11,  8,  9,  6,  7,  4,  5,  2,  3,  0,  1, 14, 15
shufb_lohi:    db  0,  8,  1,  9,  2, 10,  3, 11,  4, 12,  5, 13,  6, 14,  7, 15
pw_128:        times 2 dw 128
pw_2048:       times 2 dw 2048
tap_table:     ; masks for 8 bit shifts
               db 0xFF, 0x7F, 0x3F, 0x1F, 0x0F, 0x07, 0x03, 0x01
               ; weights
               db  4,  2,  3,  3,  2,  1
               db -1 * 16 + 1, -2 * 16 + 2
               db  0 * 16 + 1, -1 * 16 + 2
               db  0 * 16 + 1,  0 * 16 + 2
               db  0 * 16 + 1,  1 * 16 + 2
               db  1 * 16 + 1,  2 * 16 + 2
               db  1 * 16 + 0,  2 * 16 + 1
               db  1 * 16 + 0,  2 * 16 + 0
               db  1 * 16 + 0,  2 * 16 - 1
               ; the last 6 are repeats of the first 6 so we don't need to & 7
               db -1 * 16 + 1, -2 * 16 + 2
               db  0 * 16 + 1, -1 * 16 + 2
               db  0 * 16 + 1,  0 * 16 + 2
               db  0 * 16 + 1,  1 * 16 + 2
               db  1 * 16 + 1,  2 * 16 + 2
               db  1 * 16 + 0,  2 * 16 + 1

CDEF_FILTER_JMP_TABLE 4x4
CDEF_FILTER_JMP_TABLE 4x8
CDEF_FILTER_JMP_TABLE 8x8

SECTION .text

%macro PREP_REGS 2 ; w, h
    ; off1/2/3[k] [6 total] from [tapq+12+(dir+0/2/6)*2+k]
    mov           dird, r7m
    lea         tableq, [cdef_filter_%1x%2_8bpc_jmptable]
    lea           dirq, [tableq+dirq*2*4]
%if %1 == 4
 %if %2 == 4
  DEFINE_ARGS dst, stride, left, top, bot, pri, sec, \
              table, dir, dirjmp, stride3, k
 %else
  DEFINE_ARGS dst, stride, left, top, bot, pri, sec, \
              table, dir, dirjmp, dst4, stride3, k
    lea          dst4q, [dstq+strideq*4]
 %endif
%else
  DEFINE_ARGS dst, stride, h, top1, bot, pri, sec, \
              table, dir, dirjmp, top2, stride3, k
    mov             hq, -8
    lea          top1q, [top1q+strideq*0]
    lea          top2q, [top1q+strideq*1]
%endif
%if %1 == 4
    lea       stride3q, [strideq*3]
%endif
%endmacro

%macro LOAD_BLOCK 2-3 0 ; w, h, init_min_max
    mov             kd, 1
    pxor           m15, m15                     ; sum
%if %2 == 8
    pxor           m12, m12
 %if %1 == 4
    movd           xm4, [dstq +strideq*0]
    movd           xm6, [dstq +strideq*1]
    movd           xm5, [dstq +strideq*2]
    movd           xm7, [dstq +stride3q ]
    vinserti128     m4, [dst4q+strideq*0], 1
    vinserti128     m6, [dst4q+strideq*1], 1
    vinserti128     m5, [dst4q+strideq*2], 1
    vinserti128     m7, [dst4q+stride3q ], 1
    punpckldq       m4, m6
    punpckldq       m5, m7
 %else
    movq           xm4, [dstq+strideq*0]
    movq           xm5, [dstq+strideq*1]
    vinserti128     m4, [dstq+strideq*2], 1
    vinserti128     m5, [dstq+stride3q ], 1
 %endif
    punpcklqdq      m4, m5
%else
    movd           xm4, [dstq+strideq*0]
    movd           xm5, [dstq+strideq*1]
    vinserti128     m4, [dstq+strideq*2], 1
    vinserti128     m5, [dstq+stride3q ], 1
    punpckldq       m4, m5
%endif
%if %3 == 1
    mova            m7, m4                      ; min
    mova            m8, m4                      ; max
%endif
%endmacro

%macro ACCUMULATE_TAP_BYTE 7-8 0 ; tap_offset, shift, mask, strength
                                 ; mul_tap, w, h, clip
    ; load p0/p1
    movsxd     dirjmpq, [dirq+kq*4+%1*2*4]
    add        dirjmpq, tableq
    call       dirjmpq

%if %8 == 1
    pmaxub          m7, m5
    pminub          m8, m5
    pmaxub          m7, m6
    pminub          m8, m6
%endif

    ; accumulate sum[m15] over p0/p1
%if %7 == 4
    punpcklbw       m5, m6
    punpcklbw       m6, m4, m4
    psubusb         m9, m5, m6
    psubusb         m5, m6, m5
    por             m9, m5     ; abs_diff_p01(p01 - px)
    pcmpeqb         m5, m9
    por             m5, %5
    psignb          m6, %5, m5
    psrlw           m5, m9, %2 ; emulate 8-bit shift
    pand            m5, %3
    psubusb         m5, %4, m5
    pminub          m5, m9
    pmaddubsw       m5, m6
    paddw          m15, m5
%else
    psubusb         m9, m5, m4
    psubusb         m5, m4, m5
    psubusb        m11, m6, m4
    psubusb         m6, m4, m6
    por             m9, m5      ; abs_diff_p0(p0 - px)
    por            m11, m6      ; abs_diff_p1(p1 - px)
    pcmpeqb         m5, m9
    pcmpeqb         m6, m11
    punpckhbw      m10, m9, m11
    punpcklbw       m9, m11
    por             m5, %5
    por            m11, m6, %5
    punpckhbw       m6, m5, m11
    punpcklbw       m5, m11
    psignb         m11, %5, m6
    psrlw           m6, m10, %2 ; emulate 8-bit shift
    pand            m6, %3
    psubusb         m6, %4, m6
    pminub          m6, m10
    pmaddubsw       m6, m11
    paddw          m12, m6
    psignb         m11, %5, m5
    psrlw           m5, m9, %2  ; emulate 8-bit shift
    pand            m5, %3
    psubusb         m5, %4, m5
    pminub          m5, m9
    pmaddubsw       m5, m11
    paddw          m15, m5
%endif
%endmacro

%macro ADJUST_PIXEL 4-5 0 ; w, h, zero, pw_2048, clip
%if %2 == 4
 %if %5 == 1
    punpcklbw       m4, %3
 %endif
    pcmpgtw         %3, m15
    paddw          m15, %3
    pmulhrsw       m15, %4
 %if %5 == 0
    packsswb       m15, m15
    paddb           m4, m15
 %else
    paddw           m4, m15
    packuswb        m4, m4 ; clip px in [0x0,0xff]
    pminub          m4, m7
    pmaxub          m4, m8
 %endif
    vextracti128   xm5, m4, 1
    movd   [dstq+strideq*0], xm4
    movd   [dstq+strideq*2], xm5
    pextrd [dstq+strideq*1], xm4, 1
    pextrd [dstq+stride3q ], xm5, 1
%else
    pcmpgtw         m6, %3, m12
    pcmpgtw         m5, %3, m15
    paddw          m12, m6
    paddw          m15, m5
 %if %5 == 1
    punpckhbw       m5, m4, %3
    punpcklbw       m4, %3
 %endif
    pmulhrsw       m12, %4
    pmulhrsw       m15, %4
 %if %5 == 0
    packsswb       m15, m12
    paddb           m4, m15
 %else
    paddw           m5, m12
    paddw           m4, m15
    packuswb        m4, m5 ; clip px in [0x0,0xff]
    pminub          m4, m7
    pmaxub          m4, m8
 %endif
    vextracti128   xm5, m4, 1
 %if %1 == 4
    movd   [dstq +strideq*0], xm4
    movd   [dst4q+strideq*0], xm5
    pextrd [dstq +strideq*1], xm4, 1
    pextrd [dst4q+strideq*1], xm5, 1
    pextrd [dstq +strideq*2], xm4, 2
    pextrd [dst4q+strideq*2], xm5, 2
    pextrd [dstq +stride3q ], xm4, 3
    pextrd [dst4q+stride3q ], xm5, 3
 %else
    movq   [dstq+strideq*0], xm4
    movq   [dstq+strideq*2], xm5
    movhps [dstq+strideq*1], xm4
    movhps [dstq+stride3q ], xm5
 %endif
%endif
%endmacro

%macro BORDER_PREP_REGS 2 ; w, h
    ; off1/2/3[k] [6 total] from [tapq+12+(dir+0/2/6)*2+k]
    mov           dird, r7m
    lea           dirq, [tableq+dirq*2+14]
%if %1*%2*2/mmsize > 1
 %if %1 == 4
    DEFINE_ARGS dst, stride, k, dir, stk, pri, sec, stride3, h, off
 %else
    DEFINE_ARGS dst, stride, k, dir, stk, pri, sec, h, off
 %endif
    mov             hd, %1*%2*2/mmsize
%else
    DEFINE_ARGS dst, stride, k, dir, stk, pri, sec, stride3, off
%endif
    lea           stkq, [px]
    pxor           m11, m11
%endmacro

%macro BORDER_LOAD_BLOCK 2-3 0 ; w, h, init_min_max
    mov             kd, 1
%if %1 == 4
    movq           xm4, [stkq+32*0]
    movhps         xm4, [stkq+32*1]
    movq           xm5, [stkq+32*2]
    movhps         xm5, [stkq+32*3]
    vinserti128     m4, xm5, 1
%else
    mova           xm4, [stkq+32*0]             ; px
    vinserti128     m4, [stkq+32*1], 1
%endif
    pxor           m15, m15                     ; sum
%if %3 == 1
    mova            m7, m4                      ; max
    mova            m8, m4                      ; min
%endif
%endmacro

%macro ACCUMULATE_TAP_WORD 6-7 0 ; tap_offset, shift, mask, strength
                                 ; mul_tap, w, clip
    ; load p0/p1
    movsx         offq, byte [dirq+kq+%1]       ; off1
%if %6 == 4
    movq           xm5, [stkq+offq*2+32*0]      ; p0
    movq           xm6, [stkq+offq*2+32*2]
    movhps         xm5, [stkq+offq*2+32*1]
    movhps         xm6, [stkq+offq*2+32*3]
    vinserti128     m5, xm6, 1
%else
    movu           xm5, [stkq+offq*2+32*0]      ; p0
    vinserti128     m5, [stkq+offq*2+32*1], 1
%endif
    neg           offq                          ; -off1
%if %6 == 4
    movq           xm6, [stkq+offq*2+32*0]      ; p1
    movq           xm9, [stkq+offq*2+32*2]
    movhps         xm6, [stkq+offq*2+32*1]
    movhps         xm9, [stkq+offq*2+32*3]
    vinserti128     m6, xm9, 1
%else
    movu           xm6, [stkq+offq*2+32*0]      ; p1
    vinserti128     m6, [stkq+offq*2+32*1], 1
%endif
%if %7 == 1
    ; out of bounds values are set to a value that is a both a large unsigned
    ; value and a negative signed value.
    ; use signed max and unsigned min to remove them
    pmaxsw          m7, m5                      ; max after p0
    pminuw          m8, m5                      ; min after p0
    pmaxsw          m7, m6                      ; max after p1
    pminuw          m8, m6                      ; min after p1
%endif

    ; accumulate sum[m15] over p0/p1
    ; calculate difference before converting
    psubw           m5, m4                      ; diff_p0(p0 - px)
    psubw           m6, m4                      ; diff_p1(p1 - px)

    ; convert to 8-bits with signed saturation
    ; saturating to large diffs has no impact on the results
    packsswb        m5, m6

    ; group into pairs so we can accumulate using maddubsw
    pshufb          m5, m12
    pabsb           m9, m5
    psignb         m10, %5, m5
    psrlw           m5, m9, %2                  ; emulate 8-bit shift
    pand            m5, %3
    psubusb         m5, %4, m5

    ; use unsigned min since abs diff can equal 0x80
    pminub          m5, m9
    pmaddubsw       m5, m10
    paddw          m15, m5
%endmacro

%macro BORDER_ADJUST_PIXEL 2-3 0 ; w, pw_2048, clip
    pcmpgtw         m9, m11, m15
    paddw          m15, m9
    pmulhrsw       m15, %2
    paddw           m4, m15
%if %3 == 1
    pminsw          m4, m7
    pmaxsw          m4, m8
%endif
    packuswb        m4, m4
    vextracti128   xm5, m4, 1
%if %1 == 4
    movd   [dstq+strideq*0], xm4
    pextrd [dstq+strideq*1], xm4, 1
    movd   [dstq+strideq*2], xm5
    pextrd [dstq+stride3q ], xm5, 1
%else
    movq [dstq+strideq*0], xm4
    movq [dstq+strideq*1], xm5
%endif
%endmacro

%macro CDEF_FILTER 2 ; w, h
INIT_YMM avx2
cglobal cdef_filter_%1x%2_8bpc, 5, 11, 0, dst, stride, left, top, bot, \
                                          pri, sec, dir, damping, edge
%assign stack_offset_entry stack_offset
    mov          edged, edgem
    cmp          edged, 0xf
    jne .border_block

    PUSH           r11
    PUSH           r12
%if %2 == 4
%assign regs_used 13
    ALLOC_STACK   0x60, 16
    pmovzxbw       xm0, [leftq+1]
    vpermq          m0, m0, q0110
    psrldq          m1, m0, 4
    vpalignr        m2, m0, m0, 12
    movu    [rsp+0x10], m0
    movu    [rsp+0x28], m1
    movu    [rsp+0x40], m2
%elif %1 == 4
%assign regs_used 14
    PUSH           r13
    ALLOC_STACK 8*2+%1*%2*1, 16
    pmovzxwd        m0, [leftq]
    mova    [rsp+0x10], m0
%else
%assign regs_used 15
    PUSH           r13
    PUSH           r14
    ALLOC_STACK 8*4+%1*%2*2+32, 16
    lea            r11, [strideq*3]
    movu           xm4, [dstq+strideq*2]
    pmovzxwq        m0, [leftq+0]
    pmovzxwq        m1, [leftq+8]
    vinserti128     m4, [dstq+r11], 1
    pmovzxbd        m2, [leftq+1]
    pmovzxbd        m3, [leftq+9]
    mov       [rsp+16], botq
    mova    [rsp+0x20], m0
    mova    [rsp+0x40], m1
    mova    [rsp+0x60], m2
    mova    [rsp+0x80], m3
    mova    [rsp+0xa0], m4
    lea           botq, [dstq+strideq*4]
%endif

 DEFINE_ARGS dst, stride, left, top, bot, pri, secdmp, zero, pridmp, damping
    mov       dampingd, r8m
    xor          zerod, zerod
    movifnidn     prid, prim
    sub       dampingd, 31
    movifnidn  secdmpd, secdmpm
    test          prid, prid
    jz .sec_only
    movd           xm0, prid
    lzcnt      pridmpd, prid
    add        pridmpd, dampingd
    cmovs      pridmpd, zerod
    mov        [rsp+0], pridmpq                 ; pri_shift
    test       secdmpd, secdmpd
    jz .pri_only
    movd           xm1, secdmpd
    lzcnt      secdmpd, secdmpd
    add        secdmpd, dampingd
    mov        [rsp+8], secdmpq                 ; sec_shift

 DEFINE_ARGS dst, stride, left, top, bot, pri, secdmp, table, pridmp
    lea         tableq, [tap_table]
    vpbroadcastb   m13, [tableq+pridmpq]        ; pri_shift_mask
    vpbroadcastb   m14, [tableq+secdmpq]        ; sec_shift_mask

    ; pri/sec_taps[k] [4 total]
 DEFINE_ARGS dst, stride, left, top, bot, pri, sec, table, dir
    vpbroadcastb    m0, xm0                     ; pri_strength
    vpbroadcastb    m1, xm1                     ; sec_strength
    and           prid, 1
    lea           priq, [tableq+priq*2+8]       ; pri_taps
    lea           secq, [tableq+12]             ; sec_taps

    PREP_REGS       %1, %2
%if %1*%2 > mmsize
.v_loop:
%endif
    LOAD_BLOCK      %1, %2, 1
.k_loop:
    vpbroadcastb    m2, [priq+kq]                          ; pri_taps
    vpbroadcastb    m3, [secq+kq]                          ; sec_taps
    ACCUMULATE_TAP_BYTE 2, [rsp+0], m13, m0, m2, %1, %2, 1 ; dir + 0
    ACCUMULATE_TAP_BYTE 4, [rsp+8], m14, m1, m3, %1, %2, 1 ; dir + 2
    ACCUMULATE_TAP_BYTE 0, [rsp+8], m14, m1, m3, %1, %2, 1 ; dir - 2
    dec             kq
    jge .k_loop

    vpbroadcastd   m10, [pw_2048]
    pxor            m9, m9
    ADJUST_PIXEL    %1, %2, m9, m10, 1
%if %1*%2 > mmsize
    lea           dstq, [dstq+strideq*4]
    lea          top1q, [rsp+0xa0]
    lea          top2q, [rsp+0xb0]
    mov           botq, [rsp+16]
    add             hq, 4
    jl .v_loop
%endif
    RET

.pri_only:
 DEFINE_ARGS dst, stride, left, top, bot, pri, _, table, pridmp
    lea         tableq, [tap_table]
    vpbroadcastb   m13, [tableq+pridmpq]        ; pri_shift_mask
    ; pri/sec_taps[k] [4 total]
 DEFINE_ARGS dst, stride, left, top, bot, pri, _, table, dir
    vpbroadcastb    m0, xm0                     ; pri_strength
    and           prid, 1
    lea           priq, [tableq+priq*2+8]       ; pri_taps
    PREP_REGS       %1, %2
    vpbroadcastd    m3, [pw_2048]
    pxor            m1, m1
%if %1*%2 > mmsize
.pri_v_loop:
%endif
    LOAD_BLOCK      %1, %2
.pri_k_loop:
    vpbroadcastb    m2, [priq+kq]                       ; pri_taps
    ACCUMULATE_TAP_BYTE 2, [rsp+0], m13, m0, m2, %1, %2 ; dir + 0
    dec             kq
    jge .pri_k_loop
    ADJUST_PIXEL    %1, %2, m1, m3
%if %1*%2 > mmsize
    lea           dstq, [dstq+strideq*4]
    lea          top1q, [rsp+0xa0]
    lea          top2q, [rsp+0xb0]
    mov           botq, [rsp+16]
    add             hq, 4
    jl .pri_v_loop
%endif
    RET

.sec_only:
 DEFINE_ARGS dst, stride, left, top, bot, _, secdmp, zero, _, damping
    movd           xm1, secdmpd
    lzcnt      secdmpd, secdmpd
    add        secdmpd, dampingd
    mov        [rsp+8], secdmpq                 ; sec_shift
 DEFINE_ARGS dst, stride, left, top, bot, _, secdmp, table
    lea         tableq, [tap_table]
    vpbroadcastb   m14, [tableq+secdmpq]        ; sec_shift_mask
    ; pri/sec_taps[k] [4 total]
 DEFINE_ARGS dst, stride, left, top, bot, _, sec, table, dir
    vpbroadcastb    m1, xm1                     ; sec_strength
    lea           secq, [tableq+12]             ; sec_taps
    PREP_REGS       %1, %2
    vpbroadcastd    m2, [pw_2048]
    pxor            m0, m0
%if %1*%2 > mmsize
.sec_v_loop:
%endif
    LOAD_BLOCK      %1, %2
.sec_k_loop:
    vpbroadcastb    m3, [secq+kq]                       ; sec_taps
    ACCUMULATE_TAP_BYTE 4, [rsp+8], m14, m1, m3, %1, %2 ; dir + 2
    ACCUMULATE_TAP_BYTE 0, [rsp+8], m14, m1, m3, %1, %2 ; dir - 2
    dec             kq
    jge .sec_k_loop
    ADJUST_PIXEL    %1, %2, m0, m2
%if %1*%2 > mmsize
    lea           dstq, [dstq+strideq*4]
    lea          top1q, [rsp+0xa0]
    lea          top2q, [rsp+0xb0]
    mov           botq, [rsp+16]
    add             hq, 4
    jl .sec_v_loop
%endif
    RET

.d0k0:
%if %1 == 4
 %if %2 == 4
    vpbroadcastq    m6, [dstq+strideq*1-1]
    vpbroadcastq   m10, [dstq+strideq*2-1]
    movd           xm5, [topq+strideq*1+1]
    movd           xm9, [dstq+strideq*0+1]
    psrldq         m11, m6, 2
    psrldq         m12, m10, 2
    vinserti128     m6, [dstq+stride3q -1], 1
    vinserti128    m10, [botq          -1], 1
    vpblendd        m5, m11, 0x10
    vpblendd        m9, m12, 0x10
    movu           m11, [blend_4x4+16]
    punpckldq       m6, m10
    punpckldq       m5, m9
    vpblendvb       m6, [rsp+gprsize+0x28], m11
 %else
    movd           xm5, [topq +strideq*1+1]
    movq           xm6, [dstq +strideq*1-1]
    movq          xm10, [dstq +stride3q -1]
    movq          xm11, [dst4q+strideq*1-1]
    pinsrd         xm5, [dstq +strideq*0+1], 1
    movhps         xm6, [dstq +strideq*2-1]
    movhps        xm10, [dst4q+strideq*0-1]
    movhps        xm11, [dst4q+strideq*2-1]
    psrldq         xm9, xm6, 2
    shufps         xm5, xm9, q2010   ; -1 +0 +1 +2
    shufps         xm6, xm10, q2020  ; +1 +2 +3 +4
    psrldq         xm9, xm11, 2
    psrldq        xm10, 2
    shufps        xm10, xm9, q2020   ; +3 +4 +5 +6
    movd           xm9, [dst4q+stride3q -1]
    pinsrd         xm9, [botq           -1], 1
    shufps        xm11, xm9, q1020   ; +5 +6 +7 +8
    pmovzxbw        m9, [leftq+3]
    vinserti128     m6, xm11, 1
    movu           m11, [blend_4x8_0+4]
    vinserti128     m5, xm10, 1
    vpblendvb       m6, m9, m11
 %endif
%else
    lea            r13, [blend_8x8_0+16]
    movq           xm5, [top2q         +1]
    vbroadcasti128 m10, [dstq+strideq*1-1]
    vbroadcasti128 m11, [dstq+strideq*2-1]
    movhps         xm5, [dstq+strideq*0+1]
    vinserti128     m6, m10, [dstq+stride3q-1], 1
    vinserti128     m9, m11, [botq         -1], 1
    psrldq         m10, 2
    psrldq         m11, 2
    punpcklqdq      m6, m9
    movu            m9, [r13+hq*2*1+16*1]
    punpcklqdq     m10, m11
    vpblendd        m5, m10, 0xF0
    vpblendvb       m6, [rsp+gprsize+0x60+hq*8+64+8*1], m9
%endif
    ret
.d1k0:
.d2k0:
.d3k0:
%if %1 == 4
 %if %2 == 4
    movq           xm6, [dstq+strideq*0-1]
    movq           xm9, [dstq+strideq*1-1]
    vinserti128     m6, [dstq+strideq*2-1], 1
    vinserti128     m9, [dstq+stride3q -1], 1
    movu           m11, [rsp+gprsize+0x10]
    pcmpeqd        m12, m12
    psrldq          m5, m6, 2
    psrldq         m10, m9, 2
    psrld          m12, 24
    punpckldq       m6, m9
    punpckldq       m5, m10
    vpblendvb       m6, m11, m12
 %else
    movq           xm6, [dstq +strideq*0-1]
    movq           xm9, [dstq +strideq*2-1]
    movhps         xm6, [dstq +strideq*1-1]
    movhps         xm9, [dstq +stride3q -1]
    movq          xm10, [dst4q+strideq*0-1]
    movhps        xm10, [dst4q+strideq*1-1]
    psrldq         xm5, xm6, 2
    psrldq        xm11, xm9, 2
    shufps         xm5, xm11, q2020
    movq          xm11, [dst4q+strideq*2-1]
    movhps        xm11, [dst4q+stride3q -1]
    shufps         xm6, xm9, q2020
    shufps         xm9, xm10, xm11, q2020
    vinserti128     m6, xm9, 1
    pmovzxbw        m9, [leftq+1]
    psrldq        xm10, 2
    psrldq        xm11, 2
    shufps        xm10, xm11, q2020
    vpbroadcastd   m11, [blend_4x8_0+4]
    vinserti128     m5, xm10, 1
    vpblendvb       m6, m9, m11
 %endif
%else
    movu           xm5, [dstq+strideq*0-1]
    movu           xm9, [dstq+strideq*1-1]
    vinserti128     m5, [dstq+strideq*2-1], 1
    vinserti128     m9, [dstq+stride3q -1], 1
    movu           m10, [blend_8x8_0+16]
    punpcklqdq      m6, m5, m9
    vpblendvb       m6, [rsp+gprsize+0x60+hq*8+64], m10
    psrldq          m5, 2
    psrldq          m9, 2
    punpcklqdq      m5, m9
%endif
    ret
.d4k0:
%if %1 == 4
 %if %2 == 4
    vpbroadcastq   m10, [dstq+strideq*1-1]
    vpbroadcastq   m11, [dstq+strideq*2-1]
    movd           xm6, [topq+strideq*1-1]
    movd           xm9, [dstq+strideq*0-1]
    psrldq          m5, m10, 2
    psrldq         m12, m11, 2
    vpblendd        m6, m10, 0x10
    vpblendd        m9, m11, 0x10
    movu           m10, [blend_4x4]
    vinserti128     m5, [dstq+stride3q +1], 1
    vinserti128    m12, [botq          +1], 1
    punpckldq       m6, m9
    punpckldq       m5, m12
    vpblendvb       m6, [rsp+gprsize+0x40], m10
 %else
    movd           xm6, [topq +strideq*1-1]
    movq           xm9, [dstq +strideq*1-1]
    movq          xm10, [dstq +stride3q -1]
    movq          xm11, [dst4q+strideq*1-1]
    pinsrd         xm6, [dstq +strideq*0-1], 1
    movhps         xm9, [dstq +strideq*2-1]
    movhps        xm10, [dst4q+strideq*0-1]
    movhps        xm11, [dst4q+strideq*2-1]
    psrldq         xm5, xm9, 2
    shufps         xm6, xm9, q2010
    psrldq         xm9, xm10, 2
    shufps         xm5, xm9, q2020
    shufps        xm10, xm11, q2020
    movd           xm9, [dst4q+stride3q +1]
    vinserti128     m6, xm10, 1
    pinsrd         xm9, [botq           +1], 1
    psrldq        xm11, 2
    pmovzxbw       m10, [leftq-1]
    shufps        xm11, xm9, q1020
    movu            m9, [blend_4x8_0]
    vinserti128     m5, xm11, 1
    vpblendvb       m6, m10, m9
 %endif
%else
    lea            r13, [blend_8x8_0+8]
    movq           xm6, [top2q         -1]
    vbroadcasti128  m5, [dstq+strideq*1-1]
    vbroadcasti128  m9, [dstq+strideq*2-1]
    movhps         xm6, [dstq+strideq*0-1]
    movu           m11, [r13+hq*2*1+16*1]
    punpcklqdq     m10, m5, m9
    vinserti128     m5, [dstq+stride3q -1], 1
    vinserti128     m9, [botq          -1], 1
    vpblendd        m6, m10, 0xF0
    vpblendvb       m6, [rsp+gprsize+0x60+hq*8+64-8*1], m11
    psrldq          m5, 2
    psrldq          m9, 2
    punpcklqdq      m5, m9
%endif
    ret
.d5k0:
.d6k0:
.d7k0:
%if %1 == 4
 %if %2 == 4
    movd           xm6, [topq+strideq*1  ]
    vpbroadcastd    m5, [dstq+strideq*1  ]
    vpbroadcastd    m9, [dstq+strideq*2  ]
    vpblendd       xm6, [dstq+strideq*0-4], 0x2
    vpblendd        m5, m9, 0x22
    vpblendd        m6, m5, 0x30
    vinserti128     m5, [dstq+stride3q   ], 1
    vpblendd        m5, [botq         -20], 0x20
 %else
    movd           xm6, [topq +strideq*1]
    movd           xm5, [dstq +strideq*1]
    movd           xm9, [dstq +stride3q ]
    movd          xm10, [dst4q+strideq*1]
    movd          xm11, [dst4q+stride3q ]
    pinsrd         xm6, [dstq +strideq*0], 1
    pinsrd         xm5, [dstq +strideq*2], 1
    pinsrd         xm9, [dst4q+strideq*0], 1
    pinsrd        xm10, [dst4q+strideq*2], 1
    pinsrd        xm11, [botq           ], 1
    punpcklqdq     xm6, xm5
    punpcklqdq     xm5, xm9
    punpcklqdq     xm9, xm10
    punpcklqdq    xm10, xm11
    vinserti128     m6, xm9, 1
    vinserti128     m5, xm10, 1
 %endif
%else
    movq           xm6, [top2q         ]
    movq           xm5, [dstq+strideq*1]
    movq           xm9, [dstq+stride3q ]
    movhps         xm6, [dstq+strideq*0]
    movhps         xm5, [dstq+strideq*2]
    movhps         xm9, [botq          ]
    vinserti128     m6, xm5, 1
    vinserti128     m5, xm9, 1
%endif
    ret
.d0k1:
%if %1 == 4
 %if %2 == 4
    movd           xm6, [dstq+strideq*2-2]
    movd           xm9, [dstq+stride3q -2]
    movd           xm5, [topq+strideq*0+2]
    movd          xm10, [topq+strideq*1+2]
    pinsrw         xm6, [leftq+4], 0
    pinsrw         xm9, [leftq+6], 0
    vinserti128     m5, [dstq+strideq*0+2], 1
    vinserti128    m10, [dstq+strideq*1+2], 1
    vinserti128     m6, [botq+strideq*0-2], 1
    vinserti128     m9, [botq+strideq*1-2], 1
    punpckldq       m5, m10
    punpckldq       m6, m9
 %else
    movq           xm6, [dstq +strideq*2-2]
    movd          xm10, [dst4q+strideq*2-2]
    movd           xm5, [topq +strideq*0+2]
    movq           xm9, [dst4q+strideq*0-2]
    movhps         xm6, [dstq +stride3q -2]
    pinsrw        xm10, [dst4q+stride3q   ], 3
    pinsrd         xm5, [topq +strideq*1+2], 1
    movhps         xm9, [dst4q+strideq*1-2]
    pinsrd        xm10, [botq +strideq*0-2], 2
    pinsrd         xm5, [dstq +strideq*0+2], 2
    pinsrd        xm10, [botq +strideq*1-2], 3
    pinsrd         xm5, [dstq +strideq*1+2], 3
    shufps        xm11, xm6, xm9, q3131
    shufps         xm6, xm9, q2020
    movu            m9, [blend_4x8_3+8]
    vinserti128     m6, xm10, 1
    vinserti128     m5, xm11, 1
    vpblendvb       m6, [rsp+gprsize+0x10+8], m9
 %endif
%else
    lea            r13, [blend_8x8_1+16]
    movq           xm6, [dstq+strideq*2-2]
    movq           xm9, [dstq+stride3q -2]
    movq           xm5, [top1q         +2]
    movq          xm10, [top2q         +2]
    movu           m11, [r13+hq*2*2+16*2]
    vinserti128     m6, [botq+strideq*0-2], 1
    vinserti128     m9, [botq+strideq*1-2], 1
    vinserti128     m5, [dstq+strideq*0+2], 1
    vinserti128    m10, [dstq+strideq*1+2], 1
    punpcklqdq      m6, m9
    punpcklqdq      m5, m10
    vpblendvb       m6, [rsp+gprsize+0x20+hq*8+64+8*2], m11
%endif
    ret
.d1k1:
%if %1 == 4
 %if %2 == 4
    vpbroadcastq    m6, [dstq+strideq*1-2]
    vpbroadcastq    m9, [dstq+strideq*2-2]
    movd           xm5, [topq+strideq*1+2]
    movd          xm10, [dstq+strideq*0+2]
    psrldq         m11, m6, 4
    psrldq         m12, m9, 4
    vpblendd        m5, m11, 0x10
    movq          xm11, [leftq+2]
    vinserti128     m6, [dstq+stride3q-2], 1
    punpckldq     xm11, xm11
    vpblendd       m10, m12, 0x10
    pcmpeqd        m12, m12
    pmovzxwd       m11, xm11
    psrld          m12, 16
    punpckldq       m6, m9
    vpbroadcastd    m9, [botq-2]
    vpblendvb       m6, m11, m12
    punpckldq       m5, m10
    vpblendd        m6, m9, 0x20
 %else
    movd           xm5, [topq +strideq*1+2]
    movq           xm6, [dstq +strideq*1-2]
    movq           xm9, [dstq +stride3q -2]
    movq          xm10, [dst4q+strideq*1-2]
    movd          xm11, [dst4q+stride3q -2]
    pinsrd         xm5, [dstq +strideq*0+2], 1
    movhps         xm6, [dstq +strideq*2-2]
    movhps         xm9, [dst4q+strideq*0-2]
    movhps        xm10, [dst4q+strideq*2-2]
    pinsrd        xm11, [botq           -2], 1
    shufps         xm5, xm6, q3110
    shufps         xm6, xm9, q2020
    shufps         xm9, xm10, q3131
    shufps        xm10, xm11, q1020
    movu           m11, [blend_4x8_2+4]
    vinserti128     m6, xm10, 1
    vinserti128     m5, xm9, 1
    vpblendvb       m6, [rsp+gprsize+0x10+4], m11
 %endif
%else
    lea            r13, [blend_8x8_1+16]
    movq           xm5, [top2q         +2]
    vbroadcasti128  m6, [dstq+strideq*1-2]
    vbroadcasti128  m9, [dstq+strideq*2-2]
    movhps         xm5, [dstq+strideq*0+2]
    shufps         m10, m6, m9, q2121
    vinserti128     m6, [dstq+stride3q -2], 1
    vinserti128     m9, [botq          -2], 1
    movu           m11, [r13+hq*2*1+16*1]
    vpblendd        m5, m10, 0xF0
    punpcklqdq      m6, m9
    vpblendvb       m6, [rsp+gprsize+0x20+hq*8+64+8*1], m11
%endif
    ret
.d2k1:
%if %1 == 4
 %if %2 == 4
    movq          xm11, [leftq]
    movq           xm6, [dstq+strideq*0-2]
    movq           xm9, [dstq+strideq*1-2]
    vinserti128     m6, [dstq+strideq*2-2], 1
    vinserti128     m9, [dstq+stride3q -2], 1
    punpckldq     xm11, xm11
    psrldq          m5, m6, 4
    psrldq         m10, m9, 4
    pmovzxwd       m11, xm11
    punpckldq       m6, m9
    punpckldq       m5, m10
    pblendw         m6, m11, 0x05
 %else
    movq           xm5, [dstq +strideq*0-2]
    movq           xm9, [dstq +strideq*2-2]
    movq          xm10, [dst4q+strideq*0-2]
    movq          xm11, [dst4q+strideq*2-2]
    movhps         xm5, [dstq +strideq*1-2]
    movhps         xm9, [dstq +stride3q -2]
    movhps        xm10, [dst4q+strideq*1-2]
    movhps        xm11, [dst4q+stride3q -2]
    shufps         xm6, xm5, xm9, q2020
    shufps         xm5, xm9, q3131
    shufps         xm9, xm10, xm11, q2020
    shufps        xm10, xm11, q3131
    pmovzxwd       m11, [leftq]
    vinserti128     m6, xm9, 1
    vinserti128     m5, xm10, 1
    pblendw         m6, m11, 0x55
 %endif
%else
    mova           m11, [rsp+gprsize+0x20+hq*8+64]
    movu           xm5, [dstq+strideq*0-2]
    movu           xm9, [dstq+strideq*1-2]
    vinserti128     m5, [dstq+strideq*2-2], 1
    vinserti128     m9, [dstq+stride3q -2], 1
    shufps          m6, m5, m9, q1010
    shufps          m5, m9, q2121
    pblendw         m6, m11, 0x11
%endif
    ret
.d3k1:
%if %1 == 4
 %if %2 == 4
    vpbroadcastq   m11, [dstq+strideq*1-2]
    vpbroadcastq   m12, [dstq+strideq*2-2]
    movd           xm6, [topq+strideq*1-2]
    movd           xm9, [dstq+strideq*0-2]
    pblendw        m11, [leftq-16+2], 0x01
    pblendw        m12, [leftq-16+4], 0x01
    pinsrw         xm9, [leftq- 0+0], 0
    psrldq          m5, m11, 4
    psrldq         m10, m12, 4
    vinserti128     m5, [dstq+stride3q +2], 1
    vinserti128    m10, [botq          +2], 1
    vpblendd        m6, m11, 0x10
    vpblendd        m9, m12, 0x10
    punpckldq       m6, m9
    punpckldq       m5, m10
 %else
    movd           xm6, [topq +strideq*1-2]
    movq           xm5, [dstq +strideq*1-2]
    movq           xm9, [dstq +stride3q -2]
    movq          xm10, [dst4q+strideq*1-2]
    movd          xm11, [dst4q+stride3q +2]
    pinsrw         xm6, [dstq +strideq*0  ], 3
    movhps         xm5, [dstq +strideq*2-2]
    movhps         xm9, [dst4q+strideq*0-2]
    movhps        xm10, [dst4q+strideq*2-2]
    pinsrd        xm11, [botq           +2], 1
    shufps         xm6, xm5, q2010
    shufps         xm5, xm9, q3131
    shufps         xm9, xm10, q2020
    shufps        xm10, xm11, q1031
    movu           m11, [blend_4x8_2]
    vinserti128     m6, xm9, 1
    vinserti128     m5, xm10, 1
    vpblendvb       m6, [rsp+gprsize+0x10-4], m11
 %endif
%else
    lea            r13, [blend_8x8_1+8]
    movq           xm6, [top2q         -2]
    vbroadcasti128  m5, [dstq+strideq*1-2]
    vbroadcasti128 m10, [dstq+strideq*2-2]
    movhps         xm6, [dstq+strideq*0-2]
    punpcklqdq      m9, m5, m10
    vinserti128     m5, [dstq+stride3q -2], 1
    vinserti128    m10, [botq          -2], 1
    movu           m11, [r13+hq*2*1+16*1]
    vpblendd        m6, m9, 0xF0
    shufps          m5, m10, q2121
    vpblendvb       m6, [rsp+gprsize+0x20+hq*8+64-8*1], m11
%endif
    ret
.d4k1:
%if %1 == 4
 %if %2 == 4
    vinserti128     m6, [dstq+strideq*0-2], 1
    vinserti128     m9, [dstq+strideq*1-2], 1
    movd           xm5, [dstq+strideq*2+2]
    movd          xm10, [dstq+stride3q +2]
    pblendw         m6, [leftq-16+0], 0x01
    pblendw         m9, [leftq-16+2], 0x01
    vinserti128     m5, [botq+strideq*0+2], 1
    vinserti128    m10, [botq+strideq*1+2], 1
    vpblendd        m6, [topq+strideq*0-2], 0x01
    vpblendd        m9, [topq+strideq*1-2], 0x01
    punpckldq       m5, m10
    punpckldq       m6, m9
 %else
    movd           xm6, [topq +strideq*0-2]
    movq           xm5, [dstq +strideq*2-2]
    movq           xm9, [dst4q+strideq*0-2]
    movd          xm10, [dst4q+strideq*2+2]
    pinsrd         xm6, [topq +strideq*1-2], 1
    movhps         xm5, [dstq +stride3q -2]
    movhps         xm9, [dst4q+strideq*1-2]
    pinsrd        xm10, [dst4q+stride3q +2], 1
    pinsrd         xm6, [dstq +strideq*0-2], 2
    pinsrd        xm10, [botq +strideq*0+2], 2
    pinsrd         xm6, [dstq +strideq*1-2], 3
    pinsrd        xm10, [botq +strideq*1+2], 3
    shufps        xm11, xm5, xm9, q2020
    shufps         xm5, xm9, q3131
    movu            m9, [blend_4x8_3]
    vinserti128     m6, xm11, 1
    vinserti128     m5, xm10, 1
    vpblendvb       m6, [rsp+gprsize+0x10-8], m9
 %endif
%else
    lea            r13, [blend_8x8_1]
    movu           m11, [r13+hq*2*2+16*2]
    movq           xm6, [top1q         -2]
    movq           xm9, [top2q         -2]
    movq           xm5, [dstq+strideq*2+2]
    movq          xm10, [dstq+stride3q +2]
    vinserti128     m6, [dstq+strideq*0-2], 1
    vinserti128     m9, [dstq+strideq*1-2], 1
    vinserti128     m5, [botq+strideq*0+2], 1
    vinserti128    m10, [botq+strideq*1+2], 1
    punpcklqdq      m6, m9
    vpblendvb       m6, [rsp+gprsize+0x20+hq*8+64-8*2], m11
    punpcklqdq      m5, m10
%endif
    ret
.d5k1:
%if %1 == 4
 %if %2 == 4
    movd           xm6, [topq+strideq*0-1]
    movd           xm9, [topq+strideq*1-1]
    movd           xm5, [dstq+strideq*2+1]
    movd          xm10, [dstq+stride3q +1]
    pcmpeqd        m12, m12
    pmovzxbw       m11, [leftq-8+1]
    psrld          m12, 24
    vinserti128     m6, [dstq+strideq*0-1], 1
    vinserti128     m9, [dstq+strideq*1-1], 1
    vinserti128     m5, [botq+strideq*0+1], 1
    vinserti128    m10, [botq+strideq*1+1], 1
    punpckldq       m6, m9
    pxor            m9, m9
    vpblendd       m12, m9, 0x0F
    punpckldq       m5, m10
    vpblendvb       m6, m11, m12
 %else
    movd           xm6, [topq +strideq*0-1]
    movq           xm5, [dstq +strideq*2-1]
    movq           xm9, [dst4q+strideq*0-1]
    movd          xm10, [dst4q+strideq*2+1]
    pinsrd         xm6, [topq +strideq*1-1], 1
    movhps         xm5, [dstq +stride3q -1]
    movhps         xm9, [dst4q+strideq*1-1]
    pinsrd        xm10, [dst4q+stride3q +1], 1
    pinsrd         xm6, [dstq +strideq*0-1], 2
    pinsrd        xm10, [botq +strideq*0+1], 2
    pinsrd         xm6, [dstq +strideq*1-1], 3
    pinsrd        xm10, [botq +strideq*1+1], 3
    shufps        xm11, xm5, xm9, q2020
    vinserti128     m6, xm11, 1
    pmovzxbw       m11, [leftq-3]
    psrldq         xm5, 2
    psrldq         xm9, 2
    shufps         xm5, xm9, q2020
    movu            m9, [blend_4x8_1]
    vinserti128     m5, xm10, 1
    vpblendvb       m6, m11, m9
 %endif
%else
    lea            r13, [blend_8x8_0]
    movu           m11, [r13+hq*2*2+16*2]
    movq           xm6, [top1q         -1]
    movq           xm9, [top2q         -1]
    movq           xm5, [dstq+strideq*2+1]
    movq          xm10, [dstq+stride3q +1]
    vinserti128     m6, [dstq+strideq*0-1], 1
    vinserti128     m9, [dstq+strideq*1-1], 1
    vinserti128     m5, [botq+strideq*0+1], 1
    vinserti128    m10, [botq+strideq*1+1], 1
    punpcklqdq      m6, m9
    punpcklqdq      m5, m10
    vpblendvb       m6, [rsp+gprsize+0x60+hq*8+64-8*2], m11
%endif
    ret
.d6k1:
%if %1 == 4
 %if %2 == 4
    movd           xm6, [topq+strideq*0]
    movd           xm9, [topq+strideq*1]
    movd           xm5, [dstq+strideq*2]
    movd          xm10, [dstq+stride3q ]
    vinserti128     m6, [dstq+strideq*0], 1
    vinserti128     m9, [dstq+strideq*1], 1
    vinserti128     m5, [botq+strideq*0], 1
    vinserti128    m10, [botq+strideq*1], 1
    punpckldq       m6, m9
    punpckldq       m5, m10
 %else
    movd           xm5, [dstq +strideq*2]
    movd           xm6, [topq +strideq*0]
    movd           xm9, [dst4q+strideq*2]
    pinsrd         xm5, [dstq +stride3q ], 1
    pinsrd         xm6, [topq +strideq*1], 1
    pinsrd         xm9, [dst4q+stride3q ], 1
    pinsrd         xm5, [dst4q+strideq*0], 2
    pinsrd         xm6, [dstq +strideq*0], 2
    pinsrd         xm9, [botq +strideq*0], 2
    pinsrd         xm5, [dst4q+strideq*1], 3
    pinsrd         xm6, [dstq +strideq*1], 3
    pinsrd         xm9, [botq +strideq*1], 3
    vinserti128     m6, xm5, 1
    vinserti128     m5, xm9, 1
 %endif
%else
    movq           xm5, [dstq+strideq*2]
    movq           xm9, [botq+strideq*0]
    movq           xm6, [top1q         ]
    movq          xm10, [dstq+strideq*0]
    movhps         xm5, [dstq+stride3q ]
    movhps         xm9, [botq+strideq*1]
    movhps         xm6, [top2q         ]
    movhps        xm10, [dstq+strideq*1]
    vinserti128     m5, xm9, 1
    vinserti128     m6, xm10, 1
%endif
    ret
.d7k1:
%if %1 == 4
 %if %2 == 4
    movd           xm5, [dstq+strideq*2-1]
    movd           xm9, [dstq+stride3q -1]
    movd           xm6, [topq+strideq*0+1]
    movd          xm10, [topq+strideq*1+1]
    pinsrb         xm5, [leftq+ 5], 0
    pinsrb         xm9, [leftq+ 7], 0
    vinserti128     m6, [dstq+strideq*0+1], 1
    vinserti128    m10, [dstq+strideq*1+1], 1
    vinserti128     m5, [botq+strideq*0-1], 1
    vinserti128     m9, [botq+strideq*1-1], 1
    punpckldq       m6, m10
    punpckldq       m5, m9
 %else
    movd           xm6, [topq +strideq*0+1]
    movq           xm9, [dstq +strideq*2-1]
    movq          xm10, [dst4q+strideq*0-1]
    movd          xm11, [dst4q+strideq*2-1]
    pinsrd         xm6, [topq +strideq*1+1], 1
    movhps         xm9, [dstq +stride3q -1]
    movhps        xm10, [dst4q+strideq*1-1]
    pinsrd        xm11, [dst4q+stride3q -1], 1
    pinsrd         xm6, [dstq +strideq*0+1], 2
    pinsrd        xm11, [botq +strideq*0-1], 2
    pinsrd         xm6, [dstq +strideq*1+1], 3
    pinsrd        xm11, [botq +strideq*1-1], 3
    shufps         xm5, xm9, xm10, q2020
    vinserti128     m5, xm11, 1
    pmovzxbw       m11, [leftq+5]
    psrldq         xm9, 2
    psrldq        xm10, 2
    shufps         xm9, xm10, q2020
    movu           m10, [blend_4x8_1+8]
    vinserti128     m6, xm9, 1
    vpblendvb       m5, m11, m10
 %endif
%else
    lea            r13, [blend_8x8_0+16]
    movq           xm5, [dstq+strideq*2-1]
    movq           xm9, [botq+strideq*0-1]
    movq           xm6, [top1q         +1]
    movq          xm10, [dstq+strideq*0+1]
    movhps         xm5, [dstq+stride3q -1]
    movhps         xm9, [botq+strideq*1-1]
    movhps         xm6, [top2q         +1]
    movhps        xm10, [dstq+strideq*1+1]
    movu           m11, [r13+hq*2*2+16*2]
    vinserti128     m5, xm9, 1
    vinserti128     m6, xm10, 1
    vpblendvb       m5, [rsp+gprsize+0x60+hq*8+64+8*2], m11
%endif
    ret

.border_block:
 DEFINE_ARGS dst, stride, left, top, bot, pri, sec, stride3, dst4, edge
%define rstk rsp
%assign stack_offset stack_offset_entry
%assign regs_used 11
    ALLOC_STACK 2*16+(%2+4)*32, 16
%define px rsp+2*16+2*32

    pcmpeqw        m14, m14
    psllw          m14, 15                  ; 0x8000

    ; prepare pixel buffers - body/right
%if %1 == 4
    INIT_XMM avx2
%endif
%if %2 == 8
    lea          dst4q, [dstq+strideq*4]
%endif
    lea       stride3q, [strideq*3]
    test         edgeb, 2                   ; have_right
    jz .no_right
    pmovzxbw        m1, [dstq+strideq*0]
    pmovzxbw        m2, [dstq+strideq*1]
    pmovzxbw        m3, [dstq+strideq*2]
    pmovzxbw        m4, [dstq+stride3q]
    mova     [px+0*32], m1
    mova     [px+1*32], m2
    mova     [px+2*32], m3
    mova     [px+3*32], m4
%if %2 == 8
    pmovzxbw        m1, [dst4q+strideq*0]
    pmovzxbw        m2, [dst4q+strideq*1]
    pmovzxbw        m3, [dst4q+strideq*2]
    pmovzxbw        m4, [dst4q+stride3q]
    mova     [px+4*32], m1
    mova     [px+5*32], m2
    mova     [px+6*32], m3
    mova     [px+7*32], m4
%endif
    jmp .body_done
.no_right:
%if %1 == 4
    movd           xm1, [dstq+strideq*0]
    movd           xm2, [dstq+strideq*1]
    movd           xm3, [dstq+strideq*2]
    movd           xm4, [dstq+stride3q]
    pmovzxbw       xm1, xm1
    pmovzxbw       xm2, xm2
    pmovzxbw       xm3, xm3
    pmovzxbw       xm4, xm4
    movq     [px+0*32], xm1
    movq     [px+1*32], xm2
    movq     [px+2*32], xm3
    movq     [px+3*32], xm4
%else
    pmovzxbw       xm1, [dstq+strideq*0]
    pmovzxbw       xm2, [dstq+strideq*1]
    pmovzxbw       xm3, [dstq+strideq*2]
    pmovzxbw       xm4, [dstq+stride3q]
    mova     [px+0*32], xm1
    mova     [px+1*32], xm2
    mova     [px+2*32], xm3
    mova     [px+3*32], xm4
%endif
    movd [px+0*32+%1*2], xm14
    movd [px+1*32+%1*2], xm14
    movd [px+2*32+%1*2], xm14
    movd [px+3*32+%1*2], xm14
%if %2 == 8
 %if %1 == 4
    movd           xm1, [dst4q+strideq*0]
    movd           xm2, [dst4q+strideq*1]
    movd           xm3, [dst4q+strideq*2]
    movd           xm4, [dst4q+stride3q]
    pmovzxbw       xm1, xm1
    pmovzxbw       xm2, xm2
    pmovzxbw       xm3, xm3
    pmovzxbw       xm4, xm4
    movq     [px+4*32], xm1
    movq     [px+5*32], xm2
    movq     [px+6*32], xm3
    movq     [px+7*32], xm4
 %else
    pmovzxbw       xm1, [dst4q+strideq*0]
    pmovzxbw       xm2, [dst4q+strideq*1]
    pmovzxbw       xm3, [dst4q+strideq*2]
    pmovzxbw       xm4, [dst4q+stride3q]
    mova     [px+4*32], xm1
    mova     [px+5*32], xm2
    mova     [px+6*32], xm3
    mova     [px+7*32], xm4
 %endif
    movd [px+4*32+%1*2], xm14
    movd [px+5*32+%1*2], xm14
    movd [px+6*32+%1*2], xm14
    movd [px+7*32+%1*2], xm14
%endif
.body_done:

    ; top
    test         edgeb, 4                    ; have_top
    jz .no_top
    test         edgeb, 1                    ; have_left
    jz .top_no_left
    test         edgeb, 2                    ; have_right
    jz .top_no_right
    pmovzxbw        m1, [topq+strideq*0-(%1/2)]
    pmovzxbw        m2, [topq+strideq*1-(%1/2)]
    movu  [px-2*32-%1], m1
    movu  [px-1*32-%1], m2
    jmp .top_done
.top_no_right:
    pmovzxbw        m1, [topq+strideq*0-%1]
    pmovzxbw        m2, [topq+strideq*1-%1]
    movu [px-2*32-%1*2], m1
    movu [px-1*32-%1*2], m2
    movd [px-2*32+%1*2], xm14
    movd [px-1*32+%1*2], xm14
    jmp .top_done
.top_no_left:
    test         edgeb, 2                   ; have_right
    jz .top_no_left_right
    pmovzxbw        m1, [topq+strideq*0]
    pmovzxbw        m2, [topq+strideq*1]
    mova   [px-2*32+0], m1
    mova   [px-1*32+0], m2
    movd   [px-2*32-4], xm14
    movd   [px-1*32-4], xm14
    jmp .top_done
.top_no_left_right:
%if %1 == 4
    movd           xm1, [topq+strideq*0]
    pinsrd         xm1, [topq+strideq*1], 1
    pmovzxbw       xm1, xm1
    movq   [px-2*32+0], xm1
    movhps [px-1*32+0], xm1
%else
    pmovzxbw       xm1, [topq+strideq*0]
    pmovzxbw       xm2, [topq+strideq*1]
    mova   [px-2*32+0], xm1
    mova   [px-1*32+0], xm2
%endif
    movd   [px-2*32-4], xm14
    movd   [px-1*32-4], xm14
    movd [px-2*32+%1*2], xm14
    movd [px-1*32+%1*2], xm14
    jmp .top_done
.no_top:
    movu   [px-2*32-%1], m14
    movu   [px-1*32-%1], m14
.top_done:

    ; left
    test         edgeb, 1                   ; have_left
    jz .no_left
    pmovzxbw       xm1, [leftq+ 0]
%if %2 == 8
    pmovzxbw       xm2, [leftq+ 8]
%endif
    movd   [px+0*32-4], xm1
    pextrd [px+1*32-4], xm1, 1
    pextrd [px+2*32-4], xm1, 2
    pextrd [px+3*32-4], xm1, 3
%if %2 == 8
    movd   [px+4*32-4], xm2
    pextrd [px+5*32-4], xm2, 1
    pextrd [px+6*32-4], xm2, 2
    pextrd [px+7*32-4], xm2, 3
%endif
    jmp .left_done
.no_left:
    movd   [px+0*32-4], xm14
    movd   [px+1*32-4], xm14
    movd   [px+2*32-4], xm14
    movd   [px+3*32-4], xm14
%if %2 == 8
    movd   [px+4*32-4], xm14
    movd   [px+5*32-4], xm14
    movd   [px+6*32-4], xm14
    movd   [px+7*32-4], xm14
%endif
.left_done:

    ; bottom
 DEFINE_ARGS dst, stride, _, _, bot, pri, sec, stride3, _, edge
    test         edgeb, 8                   ; have_bottom
    jz .no_bottom
    test         edgeb, 1                   ; have_left
    jz .bottom_no_left
    test         edgeb, 2                   ; have_right
    jz .bottom_no_right
    pmovzxbw        m1, [botq+strideq*0-(%1/2)]
    pmovzxbw        m2, [botq+strideq*1-(%1/2)]
    movu   [px+(%2+0)*32-%1], m1
    movu   [px+(%2+1)*32-%1], m2
    jmp .bottom_done
.bottom_no_right:
    pmovzxbw        m1, [botq+strideq*0-%1]
    pmovzxbw        m2, [botq+strideq*1-%1]
    movu  [px+(%2+0)*32-%1*2], m1
    movu  [px+(%2+1)*32-%1*2], m2
%if %1 == 8
    movd  [px+(%2-1)*32+%1*2], xm14                ; overwritten by previous movu
%endif
    movd  [px+(%2+0)*32+%1*2], xm14
    movd  [px+(%2+1)*32+%1*2], xm14
    jmp .bottom_done
.bottom_no_left:
    test          edgeb, 2                  ; have_right
    jz .bottom_no_left_right
    pmovzxbw        m1, [botq+strideq*0]
    pmovzxbw        m2, [botq+strideq*1]
    mova   [px+(%2+0)*32+0], m1
    mova   [px+(%2+1)*32+0], m2
    movd   [px+(%2+0)*32-4], xm14
    movd   [px+(%2+1)*32-4], xm14
    jmp .bottom_done
.bottom_no_left_right:
%if %1 == 4
    movd           xm1, [botq+strideq*0]
    pinsrd         xm1, [botq+strideq*1], 1
    pmovzxbw       xm1, xm1
    movq   [px+(%2+0)*32+0], xm1
    movhps [px+(%2+1)*32+0], xm1
%else
    pmovzxbw       xm1, [botq+strideq*0]
    pmovzxbw       xm2, [botq+strideq*1]
    mova   [px+(%2+0)*32+0], xm1
    mova   [px+(%2+1)*32+0], xm2
%endif
    movd   [px+(%2+0)*32-4], xm14
    movd   [px+(%2+1)*32-4], xm14
    movd  [px+(%2+0)*32+%1*2], xm14
    movd  [px+(%2+1)*32+%1*2], xm14
    jmp .bottom_done
.no_bottom:
    movu   [px+(%2+0)*32-%1], m14
    movu   [px+(%2+1)*32-%1], m14
.bottom_done:

    ; actual filter
 INIT_YMM avx2
 DEFINE_ARGS dst, stride, _, pridmp, damping, pri, secdmp, stride3, zero
%undef edged
    ; register to shuffle values into after packing
    vbroadcasti128 m12, [shufb_lohi]

    mov       dampingd, r8m
    xor          zerod, zerod
    movifnidn     prid, prim
    sub       dampingd, 31
    movifnidn  secdmpd, secdmpm
    test          prid, prid
    jz .border_sec_only
    movd           xm0, prid
    lzcnt      pridmpd, prid
    add        pridmpd, dampingd
    cmovs      pridmpd, zerod
    mov        [rsp+0], pridmpq                 ; pri_shift
    test       secdmpd, secdmpd
    jz .border_pri_only
    movd           xm1, secdmpd
    lzcnt      secdmpd, secdmpd
    add        secdmpd, dampingd
    mov        [rsp+8], secdmpq                 ; sec_shift

 DEFINE_ARGS dst, stride, _, pridmp, table, pri, secdmp, stride3
    lea         tableq, [tap_table]
    vpbroadcastb   m13, [tableq+pridmpq]        ; pri_shift_mask
    vpbroadcastb   m14, [tableq+secdmpq]        ; sec_shift_mask

    ; pri/sec_taps[k] [4 total]
 DEFINE_ARGS dst, stride, _, dir, table, pri, sec, stride3
    vpbroadcastb    m0, xm0                     ; pri_strength
    vpbroadcastb    m1, xm1                     ; sec_strength
    and           prid, 1
    lea           priq, [tableq+priq*2+8]       ; pri_taps
    lea           secq, [tableq+12]             ; sec_taps

    BORDER_PREP_REGS %1, %2
%if %1*%2*2/mmsize > 1
.border_v_loop:
%endif
    BORDER_LOAD_BLOCK %1, %2, 1
.border_k_loop:
    vpbroadcastb    m2, [priq+kq]               ; pri_taps
    vpbroadcastb    m3, [secq+kq]               ; sec_taps
    ACCUMULATE_TAP_WORD 0*2, [rsp+0], m13, m0, m2, %1, 1
    ACCUMULATE_TAP_WORD 2*2, [rsp+8], m14, m1, m3, %1, 1
    ACCUMULATE_TAP_WORD 6*2, [rsp+8], m14, m1, m3, %1, 1
    dec             kq
    jge .border_k_loop

    vpbroadcastd   m10, [pw_2048]
    BORDER_ADJUST_PIXEL %1, m10, 1
%if %1*%2*2/mmsize > 1
 %define vloop_lines (mmsize/(%1*2))
    lea           dstq, [dstq+strideq*vloop_lines]
    add           stkq, 32*vloop_lines
    dec             hd
    jg .border_v_loop
%endif
    RET

.border_pri_only:
 DEFINE_ARGS dst, stride, _, pridmp, table, pri, _, stride3
    lea         tableq, [tap_table]
    vpbroadcastb   m13, [tableq+pridmpq]        ; pri_shift_mask
 DEFINE_ARGS dst, stride, _, dir, table, pri, _, stride3
    vpbroadcastb    m0, xm0                     ; pri_strength
    and           prid, 1
    lea           priq, [tableq+priq*2+8]       ; pri_taps
    BORDER_PREP_REGS %1, %2
    vpbroadcastd    m1, [pw_2048]
%if %1*%2*2/mmsize > 1
.border_pri_v_loop:
%endif
    BORDER_LOAD_BLOCK %1, %2
.border_pri_k_loop:
    vpbroadcastb    m2, [priq+kq]               ; pri_taps
    ACCUMULATE_TAP_WORD 0*2, [rsp+0], m13, m0, m2, %1
    dec             kq
    jge .border_pri_k_loop
    BORDER_ADJUST_PIXEL %1, m1
%if %1*%2*2/mmsize > 1
 %define vloop_lines (mmsize/(%1*2))
    lea           dstq, [dstq+strideq*vloop_lines]
    add           stkq, 32*vloop_lines
    dec             hd
    jg .border_pri_v_loop
%endif
    RET

.border_sec_only:
 DEFINE_ARGS dst, stride, _, _, damping, _, secdmp, stride3
    movd           xm1, secdmpd
    lzcnt      secdmpd, secdmpd
    add        secdmpd, dampingd
    mov        [rsp+8], secdmpq                 ; sec_shift
 DEFINE_ARGS dst, stride, _, _, table, _, secdmp, stride3
    lea         tableq, [tap_table]
    vpbroadcastb   m14, [tableq+secdmpq]        ; sec_shift_mask
 DEFINE_ARGS dst, stride, _, dir, table, _, sec, stride3
    vpbroadcastb    m1, xm1                     ; sec_strength
    lea           secq, [tableq+12]             ; sec_taps
    BORDER_PREP_REGS %1, %2
    vpbroadcastd    m0, [pw_2048]
%if %1*%2*2/mmsize > 1
.border_sec_v_loop:
%endif
    BORDER_LOAD_BLOCK %1, %2
.border_sec_k_loop:
    vpbroadcastb    m3, [secq+kq]               ; sec_taps
    ACCUMULATE_TAP_WORD 2*2, [rsp+8], m14, m1, m3, %1
    ACCUMULATE_TAP_WORD 6*2, [rsp+8], m14, m1, m3, %1
    dec             kq
    jge .border_sec_k_loop
    BORDER_ADJUST_PIXEL %1, m0
%if %1*%2*2/mmsize > 1
 %define vloop_lines (mmsize/(%1*2))
    lea           dstq, [dstq+strideq*vloop_lines]
    add           stkq, 32*vloop_lines
    dec             hd
    jg .border_sec_v_loop
%endif
    RET
%endmacro

CDEF_FILTER 8, 8
CDEF_FILTER 4, 8
CDEF_FILTER 4, 4

INIT_YMM avx2
cglobal cdef_dir_8bpc, 3, 4, 6, src, stride, var, stride3
    lea       stride3q, [strideq*3]
    movq           xm0, [srcq+strideq*0]
    movq           xm1, [srcq+strideq*1]
    movq           xm2, [srcq+strideq*2]
    movq           xm3, [srcq+stride3q ]
    lea           srcq, [srcq+strideq*4]
    vpbroadcastq    m4, [srcq+stride3q ]
    vpbroadcastq    m5, [srcq+strideq*2]
    vpblendd        m0, m4, 0xf0
    vpblendd        m1, m5, 0xf0
    vpbroadcastq    m4, [srcq+strideq*1]
    vpbroadcastq    m5, [srcq+strideq*0]
    vpblendd        m2, m4, 0xf0
    vpblendd        m3, m5, 0xf0
    pxor            m4, m4
    punpcklbw       m0, m4
    punpcklbw       m1, m4
    punpcklbw       m2, m4
    punpcklbw       m3, m4
cglobal_label .main
    vpbroadcastd    m4, [pw_128]
    PROLOGUE 3, 4, 15
    psubw           m0, m4
    psubw           m1, m4
    psubw           m2, m4
    psubw           m3, m4

    ; shuffle registers to generate partial_sum_diag[0-1] together
    vperm2i128      m7, m0, m0, 0x01
    vperm2i128      m6, m1, m1, 0x01
    vperm2i128      m5, m2, m2, 0x01
    vperm2i128      m4, m3, m3, 0x01

    ; start with partial_sum_hv[0-1]
    paddw           m8, m0, m1
    paddw           m9, m2, m3
    phaddw         m10, m0, m1
    phaddw         m11, m2, m3
    paddw           m8, m9
    phaddw         m10, m11
    vextracti128   xm9, m8, 1
    vextracti128  xm11, m10, 1
    paddw          xm8, xm9                 ; partial_sum_hv[1]
    phaddw        xm10, xm11                ; partial_sum_hv[0]
    vinserti128     m8, xm10, 1
    vpbroadcastd    m9, [div_table+44]
    pmaddwd         m8, m8
    pmulld          m8, m9                  ; cost6[2a-d] | cost2[a-d]

    ; create aggregates [lower half]:
    ; m9 = m0:01234567+m1:x0123456+m2:xx012345+m3:xxx01234+
    ;      m4:xxxx0123+m5:xxxxx012+m6:xxxxxx01+m7:xxxxxxx0
    ; m10=             m1:7xxxxxxx+m2:67xxxxxx+m3:567xxxxx+
    ;      m4:4567xxxx+m5:34567xxx+m6:234567xx+m7:1234567x
    ; and [upper half]:
    ; m9 = m0:xxxxxxx0+m1:xxxxxx01+m2:xxxxx012+m3:xxxx0123+
    ;      m4:xxx01234+m5:xx012345+m6:x0123456+m7:01234567
    ; m10= m0:1234567x+m1:234567xx+m2:34567xxx+m3:4567xxxx+
    ;      m4:567xxxxx+m5:67xxxxxx+m6:7xxxxxxx
    ; and then shuffle m11 [shufw_6543210x], unpcklwd, pmaddwd, pmulld, paddd

    pslldq          m9, m1, 2
    psrldq         m10, m1, 14
    pslldq         m11, m2, 4
    psrldq         m12, m2, 12
    pslldq         m13, m3, 6
    psrldq         m14, m3, 10
    paddw           m9, m11
    paddw          m10, m12
    paddw           m9, m13
    paddw          m10, m14
    pslldq         m11, m4, 8
    psrldq         m12, m4, 8
    pslldq         m13, m5, 10
    psrldq         m14, m5, 6
    paddw           m9, m11
    paddw          m10, m12
    paddw           m9, m13
    paddw          m10, m14
    pslldq         m11, m6, 12
    psrldq         m12, m6, 4
    pslldq         m13, m7, 14
    psrldq         m14, m7, 2
    paddw           m9, m11
    paddw          m10, m12
    paddw           m9, m13
    paddw          m10, m14                 ; partial_sum_diag[0/1][8-14,zero]
    vbroadcasti128 m14, [shufw_6543210x]
    vbroadcasti128 m13, [div_table+16]
    vbroadcasti128 m12, [div_table+0]
    paddw           m9, m0                  ; partial_sum_diag[0/1][0-7]
    pshufb         m10, m14
    punpckhwd      m11, m9, m10
    punpcklwd       m9, m10
    pmaddwd        m11, m11
    pmaddwd         m9, m9
    pmulld         m11, m13
    pmulld          m9, m12
    paddd           m9, m11                 ; cost0[a-d] | cost4[a-d]

    ; merge horizontally and vertically for partial_sum_alt[0-3]
    paddw          m10, m0, m1
    paddw          m11, m2, m3
    paddw          m12, m4, m5
    paddw          m13, m6, m7
    phaddw          m0, m4
    phaddw          m1, m5
    phaddw          m2, m6
    phaddw          m3, m7

    ; create aggregates [lower half]:
    ; m4 = m10:01234567+m11:x0123456+m12:xx012345+m13:xxx01234
    ; m11=              m11:7xxxxxxx+m12:67xxxxxx+m13:567xxxxx
    ; and [upper half]:
    ; m4 = m10:xxx01234+m11:xx012345+m12:x0123456+m13:01234567
    ; m11= m10:567xxxxx+m11:67xxxxxx+m12:7xxxxxxx
    ; and then pshuflw m11 3012, unpcklwd, pmaddwd, pmulld, paddd

    pslldq          m4, m11, 2
    psrldq         m11, 14
    pslldq          m5, m12, 4
    psrldq         m12, 12
    pslldq          m6, m13, 6
    psrldq         m13, 10
    paddw           m4, m10
    paddw          m11, m12
    vpbroadcastd   m12, [div_table+44]
    paddw           m5, m6
    paddw          m11, m13                 ; partial_sum_alt[3/2] right
    vbroadcasti128 m13, [div_table+32]
    paddw           m4, m5                  ; partial_sum_alt[3/2] left
    pshuflw         m5, m11, q3012
    punpckhwd       m6, m11, m4
    punpcklwd       m4, m5
    pmaddwd         m6, m6
    pmaddwd         m4, m4
    pmulld          m6, m12
    pmulld          m4, m13
    paddd           m4, m6                  ; cost7[a-d] | cost5[a-d]

    ; create aggregates [lower half]:
    ; m5 = m0:01234567+m1:x0123456+m2:xx012345+m3:xxx01234
    ; m1 =             m1:7xxxxxxx+m2:67xxxxxx+m3:567xxxxx
    ; and [upper half]:
    ; m5 = m0:xxx01234+m1:xx012345+m2:x0123456+m3:01234567
    ; m1 = m0:567xxxxx+m1:67xxxxxx+m2:7xxxxxxx
    ; and then pshuflw m1 3012, unpcklwd, pmaddwd, pmulld, paddd

    pslldq          m5, m1, 2
    psrldq          m1, 14
    pslldq          m6, m2, 4
    psrldq          m2, 12
    pslldq          m7, m3, 6
    psrldq          m3, 10
    paddw           m5, m0
    paddw           m1, m2
    paddw           m6, m7
    paddw           m1, m3                  ; partial_sum_alt[0/1] right
    paddw           m5, m6                  ; partial_sum_alt[0/1] left
    pshuflw         m0, m1, q3012
    punpckhwd       m1, m5
    punpcklwd       m5, m0
    pmaddwd         m1, m1
    pmaddwd         m5, m5
    pmulld          m1, m12
    pmulld          m5, m13
    paddd           m5, m1                  ; cost1[a-d] | cost3[a-d]

    mova           xm0, [pd_47130256+ 16]
    mova            m1, [pd_47130256]
    phaddd          m9, m8
    phaddd          m5, m4
    phaddd          m9, m5
    vpermd          m0, m9                  ; cost[0-3]
    vpermd          m1, m9                  ; cost[4-7] | cost[0-3]

    ; now find the best cost
    pmaxsd         xm2, xm0, xm1
    pshufd         xm3, xm2, q1032
    pmaxsd         xm2, xm3
    pshufd         xm3, xm2, q2301
    pmaxsd         xm2, xm3 ; best cost

    ; find the idx using minpos
    ; make everything other than the best cost negative via subtraction
    ; find the min of unsigned 16-bit ints to sort out the negative values
    psubd          xm4, xm1, xm2
    psubd          xm3, xm0, xm2
    packssdw       xm3, xm4
    phminposuw     xm3, xm3

    ; convert idx to 32-bits
    psrld          xm3, 16
    movd           eax, xm3

    ; get idx^4 complement
    vpermd          m3, m1
    psubd          xm2, xm3
    psrld          xm2, 10
    movd        [varq], xm2
    RET

%endif ; ARCH_X86_64
