; Copyright © 2018, VideoLAN and dav1d authors
; Copyright © 2018, Two Orioles, LLC
; Copyright © 2019, VideoLabs
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

%macro DUP8 1-*
    %rep %0
        times 8 db %1
        %rotate 1
    %endrep
%endmacro

div_table_sse4:  dd 840, 420, 280, 210, 168, 140, 120, 105
                 dd 420, 210, 140, 105, 105, 105, 105, 105
div_table_ssse3: dw 840, 840, 420, 420, 280, 280, 210, 210
                 dw 168, 168, 140, 140, 120, 120, 105, 105
                 dw 420, 420, 210, 210, 140, 140, 105, 105
                 dw 105, 105, 105, 105, 105, 105, 105, 105
const shufw_6543210x, \
            db 12, 13, 10, 11,  8,  9,  6,  7,  4,  5,  2,  3,  0,  1, 14, 15
shufb_lohi: db  0,  8,  1,  9,  2, 10,  3, 11,  4, 12,  5, 13,  6, 14,  7, 15
pw_8:      times 8 dw 8
pw_128:    times 8 dw 128
pw_256:    times 8 dw 256
pw_2048:   times 8 dw 2048
pw_0x7FFF: times 8 dw 0x7FFF
pw_0x8000: times 8 dw 0x8000
tap_table: ; masks for 8-bit shift emulation
           DUP8 0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80
           ; weights
           DUP8 4, 2, 3, 3, 2, 1
           ; taps indices
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

SECTION .text

%macro movif32 2
 %if ARCH_X86_32
    mov     %1, %2
 %endif
%endmacro

%macro PMOVZXBW 2-3 0 ; %3 = half
 %if cpuflag(sse4) && %3 == 0
    pmovzxbw        %1, %2
 %else
  %if %3 == 1
    movd            %1, %2
  %else
    movq            %1, %2
  %endif
    punpcklbw       %1, m7
 %endif
%endmacro

%macro PSHUFB_0 2
 %if cpuflag(ssse3)
    pshufb          %1, %2
 %else
    punpcklbw       %1, %1
    pshuflw         %1, %1, q0000
    punpcklqdq      %1, %1
 %endif
%endmacro

%macro MOVDDUP 2
%if cpuflag(ssse3)
    movddup         %1, %2
%else
    movq            %1, %2
    punpcklqdq      %1, %1
%endif
%endmacro

%macro ACCUMULATE_TAP 7 ; tap_offset, shift, shift_mask, strength, mul_tap, w, minmax
    ; load p0/p1
    movsx         offq, byte [dirq+kq+%1+14*8]  ; off1
 %if %6 == 4
    movq            m5, [stkq+offq*2+32*0]      ; p0
    movhps          m5, [stkq+offq*2+32*1]
 %else
    movu            m5, [stkq+offq*2+32*0]      ; p0
 %endif
    neg           offq                          ; -off1
 %if %6 == 4
    movq            m6, [stkq+offq*2+32*0]      ; p1
    movhps          m6, [stkq+offq*2+32*1]
 %else
    movu            m6, [stkq+offq*2+32*0]      ; p1
 %endif
 %if %7
  %if cpuflag(sse4)
    ; out of bounds values are set to a value that is a both a large unsigned
    ; value and a negative signed value.
    ; use signed max and unsigned min to remove them
    pmaxsw          m7, m5
    pminuw          m8, m5
    pmaxsw          m7, m6
    pminuw          m8, m6
  %else
    pcmpeqw         m3, m14, m5
    pminsw          m8, m5     ; min after p0
    pandn           m3, m5
    pmaxsw          m7, m3     ; max after p0
    pcmpeqw         m3, m14, m6
    pminsw          m8, m6     ; min after p1
    pandn           m3, m6
    pmaxsw          m7, m3     ; max after p1
  %endif
 %endif

    ; accumulate sum[m13] over p0/p1
    psubw           m5, m4     ; diff_p0(p0 - px)
    psubw           m6, m4     ; diff_p1(p1 - px)
    packsswb        m5, m6     ; convert pixel diff to 8-bit
 %if cpuflag(ssse3)
    pshufb          m5, m13    ; group diffs p0 and p1 into pairs
    pabsb           m6, m5
    psignb          m3, %5, m5
 %else
    movlhps         m6, m5
    punpckhbw       m6, m5
    pxor            m5, m5
    pcmpgtb         m5, m6
    paddb           m6, m5
    pxor            m6, m5
    paddb           m3, %5, m5
    pxor            m3, m5
 %endif
    pand            m9, %3, m6 ; emulate 8-bit shift
    psrlw           m9, %2
    psubusb         m5, %4, m9
    pminub          m5, m6     ; constrain(diff_p)
 %if cpuflag(ssse3)
    pmaddubsw       m5, m3     ; constrain(diff_p) * taps
 %else
    psrlw           m9, m5, 8
    psraw           m6, m3, 8
    psllw           m5, 8
    psllw           m3, 8
    pmullw          m9, m6
    pmulhw          m5, m3
    paddw           m5, m9
 %endif
    paddw           m0, m5
%endmacro

%macro LOAD_BODY 3 ; dst, src, block_width
 %if %3 == 4
    PMOVZXBW        m0, [%2+strideq*0]
    PMOVZXBW        m1, [%2+strideq*1]
    PMOVZXBW        m2, [%2+strideq*2]
    PMOVZXBW        m3, [%2+stride3q]
    mova     [%1+32*0], m0
    mova     [%1+32*1], m1
    mova     [%1+32*2], m2
    mova     [%1+32*3], m3
 %else
    movu            m0, [%2+strideq*0]
    movu            m1, [%2+strideq*1]
    movu            m2, [%2+strideq*2]
    movu            m3, [%2+stride3q]
    punpcklbw       m4, m0, m7
    punpckhbw       m0, m7
    mova  [%1+32*0+ 0], m4
    mova  [%1+32*0+16], m0
    punpcklbw       m4, m1, m7
    punpckhbw       m1, m7
    mova  [%1+32*1+ 0], m4
    mova  [%1+32*1+16], m1
    punpcklbw       m4, m2, m7
    punpckhbw       m2, m7
    mova  [%1+32*2+ 0], m4
    mova  [%1+32*2+16], m2
    punpcklbw       m4, m3, m7
    punpckhbw       m3, m7
    mova  [%1+32*3+ 0], m4
    mova  [%1+32*3+16], m3
 %endif
%endmacro

%macro CDEF_FILTER_END 2 ; w, minmax
    pxor            m6, m6
    pcmpgtw         m6, m0
    paddw           m0, m6
 %if cpuflag(ssse3)
    pmulhrsw        m0, m15
 %else
    paddw           m0, m15
    psraw           m0, 4
 %endif
    paddw           m4, m0
 %if %2
    pminsw          m4, m7
    pmaxsw          m4, m8
 %endif
    packuswb        m4, m4
 %if %1 == 4
    movd [dstq+strideq*0], m4
    psrlq           m4, 32
    movd [dstq+strideq*1], m4
    add           stkq, 32*2
    lea           dstq, [dstq+strideq*2]
 %else
    movq        [dstq], m4
    add           stkq, 32
    add           dstq, strideq
 %endif
%endmacro

%macro CDEF_FILTER 2 ; w, h
 %if ARCH_X86_64
cglobal cdef_filter_%1x%2_8bpc, 5, 9, 16, 3 * 16 + (%2+4)*32, \
                                dst, stride, left, top, bot, pri, dst4, edge, \
                                stride3
  %define px rsp+3*16+2*32
  %define base 0
 %else
cglobal cdef_filter_%1x%2_8bpc, 2, 7, 8, - 7 * 16 - (%2+4)*32, \
                                dst, stride, left, edge, stride3
    %define       topq  r2
    %define       botq  r2
    %define      dst4q  r2
    LEA             r5, tap_table
  %define px esp+7*16+2*32
  %define base r5-tap_table
 %endif
    mov          edged, r9m
 %if cpuflag(sse4)
   %define OUT_OF_BOUNDS_MEM [base+pw_0x8000]
 %else
   %define OUT_OF_BOUNDS_MEM [base+pw_0x7FFF]
 %endif
    mova            m6, OUT_OF_BOUNDS_MEM
    pxor            m7, m7

    ; prepare pixel buffers - body/right
 %if %2 == 8
    lea          dst4q, [dstq+strideq*4]
 %endif
    lea       stride3q, [strideq*3]
    test         edgeb, 2 ; have_right
    jz .no_right
    LOAD_BODY       px, dstq, %1
 %if %2 == 8
    LOAD_BODY  px+4*32, dst4q, %1
 %endif
    jmp .body_done
.no_right:
    PMOVZXBW        m0, [dstq+strideq*0], %1 == 4
    PMOVZXBW        m1, [dstq+strideq*1], %1 == 4
    PMOVZXBW        m2, [dstq+strideq*2], %1 == 4
    PMOVZXBW        m3, [dstq+stride3q ], %1 == 4
    mova     [px+32*0], m0
    mova     [px+32*1], m1
    mova     [px+32*2], m2
    mova     [px+32*3], m3
    movd [px+32*0+%1*2], m6
    movd [px+32*1+%1*2], m6
    movd [px+32*2+%1*2], m6
    movd [px+32*3+%1*2], m6
 %if %2 == 8
    PMOVZXBW        m0, [dst4q+strideq*0], %1 == 4
    PMOVZXBW        m1, [dst4q+strideq*1], %1 == 4
    PMOVZXBW        m2, [dst4q+strideq*2], %1 == 4
    PMOVZXBW        m3, [dst4q+stride3q ], %1 == 4
    mova     [px+32*4], m0
    mova     [px+32*5], m1
    mova     [px+32*6], m2
    mova     [px+32*7], m3
    movd [px+32*4+%1*2], m6
    movd [px+32*5+%1*2], m6
    movd [px+32*6+%1*2], m6
    movd [px+32*7+%1*2], m6
 %endif
.body_done:

    ; top
    movifnidn     topq, r3mp
    test         edgeb, 4 ; have_top
    jz .no_top
    test         edgeb, 1 ; have_left
    jz .top_no_left
    test         edgeb, 2 ; have_right
    jz .top_no_right
 %if %1 == 4
    PMOVZXBW        m0, [topq+strideq*0-2]
    PMOVZXBW        m1, [topq+strideq*1-2]
 %else
    movu            m0, [topq+strideq*0-4]
    movu            m1, [topq+strideq*1-4]
    punpckhbw       m2, m0, m7
    punpcklbw       m0, m7
    punpckhbw       m3, m1, m7
    punpcklbw       m1, m7
    movu  [px-32*2+8], m2
    movu  [px-32*1+8], m3
 %endif
    movu  [px-32*2-%1], m0
    movu  [px-32*1-%1], m1
    jmp .top_done
.top_no_right:
 %if %1 == 4
    PMOVZXBW        m0, [topq+strideq*0-%1]
    PMOVZXBW        m1, [topq+strideq*1-%1]
    movu   [px-32*2-8], m0
    movu   [px-32*1-8], m1
 %else
    movu            m0, [topq+strideq*0-%1]
    movu            m1, [topq+strideq*1-%2]
    punpckhbw       m2, m0, m7
    punpcklbw       m0, m7
    punpckhbw       m3, m1, m7
    punpcklbw       m1, m7
    mova  [px-32*2-16], m0
    mova  [px-32*2+ 0], m2
    mova  [px-32*1-16], m1
    mova  [px-32*1+ 0], m3
 %endif
    movd [px-32*2+%1*2], m6
    movd [px-32*1+%1*2], m6
    jmp .top_done
.top_no_left:
    test         edgeb, 2 ; have_right
    jz .top_no_left_right
 %if %1 == 4
    PMOVZXBW        m0, [topq+strideq*0]
    PMOVZXBW        m1, [topq+strideq*1]
 %else
    movu            m0, [topq+strideq*0]
    movu            m1, [topq+strideq*1]
    punpckhbw       m2, m0, m7
    punpcklbw       m0, m7
    punpckhbw       m3, m1, m7
    punpcklbw       m1, m7
    movd  [px-32*2+16], m2
    movd  [px-32*1+16], m3
 %endif
    movd  [px-32*2- 4], m6
    movd  [px-32*1- 4], m6
    mova  [px-32*2+ 0], m0
    mova  [px-32*1+ 0], m1
    jmp .top_done
.top_no_left_right:
    PMOVZXBW        m0, [topq+strideq*0], %1 == 4
    PMOVZXBW        m1, [topq+strideq*1], %1 == 4
    movd   [px-32*2-4], m6
    movd   [px-32*1-4], m6
    mova   [px-32*2+0], m0
    mova   [px-32*1+0], m1
    movd [px-32*2+%1*2], m6
    movd [px-32*1+%1*2], m6
    jmp .top_done
.no_top:
    movu  [px-32*2- 4], m6
    movu  [px-32*1- 4], m6
 %if %1 == 8
    movq  [px-32*2+12], m6
    movq  [px-32*1+12], m6
 %endif
.top_done:

    ; left
    test         edgeb, 1 ; have_left
    jz .no_left
    movifnidn    leftq, leftmp
 %if %2 == 4
    movq            m0, [leftq]
 %else
    movu            m0, [leftq]
 %endif
 %if %2 == 4
    punpcklbw       m0, m7
 %else
    punpckhbw       m1, m0, m7
    punpcklbw       m0, m7
    movhlps         m3, m1
    movd   [px+32*4-4], m1
    movd   [px+32*6-4], m3
    psrlq           m1, 32
    psrlq           m3, 32
    movd   [px+32*5-4], m1
    movd   [px+32*7-4], m3
 %endif
    movhlps         m2, m0
    movd   [px+32*0-4], m0
    movd   [px+32*2-4], m2
    psrlq           m0, 32
    psrlq           m2, 32
    movd   [px+32*1-4], m0
    movd   [px+32*3-4], m2
    jmp .left_done
.no_left:
    movd   [px+32*0-4], m6
    movd   [px+32*1-4], m6
    movd   [px+32*2-4], m6
    movd   [px+32*3-4], m6
 %if %2 == 8
    movd   [px+32*4-4], m6
    movd   [px+32*5-4], m6
    movd   [px+32*6-4], m6
    movd   [px+32*7-4], m6
 %endif
.left_done:

    ; bottom
    movifnidn     botq, r4mp
    test         edgeb, 8 ; have_bottom
    jz .no_bottom
    test         edgeb, 1 ; have_left
    jz .bottom_no_left
    test         edgeb, 2 ; have_right
    jz .bottom_no_right
 %if %1 == 4
    PMOVZXBW        m0, [botq+strideq*0-(%1/2)]
    PMOVZXBW        m1, [botq+strideq*1-(%1/2)]
 %else
    movu            m0, [botq+strideq*0-4]
    movu            m1, [botq+strideq*1-4]
    punpckhbw       m2, m0, m7
    punpcklbw       m0, m7
    punpckhbw       m3, m1, m7
    punpcklbw       m1, m7
    movu [px+32*(%2+0)+8], m2
    movu [px+32*(%2+1)+8], m3
 %endif
    movu [px+32*(%2+0)-%1], m0
    movu [px+32*(%2+1)-%1], m1
    jmp .bottom_done
.bottom_no_right:
 %if %1 == 4
    PMOVZXBW        m0, [botq+strideq*0-4]
    PMOVZXBW        m1, [botq+strideq*1-4]
    movu [px+32*(%2+0)-8], m0
    movu [px+32*(%2+1)-8], m1
 %else
    movu            m0, [botq+strideq*0-8]
    movu            m1, [botq+strideq*1-8]
    punpckhbw       m2, m0, m7
    punpcklbw       m0, m7
    punpckhbw       m3, m1, m7
    punpcklbw       m1, m7
    mova [px+32*(%2+0)-16], m0
    mova [px+32*(%2+0)+ 0], m2
    mova [px+32*(%2+1)-16], m1
    mova [px+32*(%2+1)+ 0], m3
    movd [px+32*(%2-1)+16], m6 ; overwritten by first mova
 %endif
    movd [px+32*(%2+0)+%1*2], m6
    movd [px+32*(%2+1)+%1*2], m6
    jmp .bottom_done
.bottom_no_left:
    test         edgeb, 2 ; have_right
    jz .bottom_no_left_right
 %if %1 == 4
    PMOVZXBW        m0, [botq+strideq*0]
    PMOVZXBW        m1, [botq+strideq*1]
 %else
    movu            m0, [botq+strideq*0]
    movu            m1, [botq+strideq*1]
    punpckhbw       m2, m0, m7
    punpcklbw       m0, m7
    punpckhbw       m3, m1, m7
    punpcklbw       m1, m7
    mova [px+32*(%2+0)+16], m2
    mova [px+32*(%2+1)+16], m3
 %endif
    mova [px+32*(%2+0)+ 0], m0
    mova [px+32*(%2+1)+ 0], m1
    movd [px+32*(%2+0)- 4], m6
    movd [px+32*(%2+1)- 4], m6
    jmp .bottom_done
.bottom_no_left_right:
    PMOVZXBW        m0, [botq+strideq*0], %1 == 4
    PMOVZXBW        m1, [botq+strideq*1], %1 == 4
    mova [px+32*(%2+0)+ 0], m0
    mova [px+32*(%2+1)+ 0], m1
    movd [px+32*(%2+0)+%1*2], m6
    movd [px+32*(%2+1)+%1*2], m6
    movd [px+32*(%2+0)- 4], m6
    movd [px+32*(%2+1)- 4], m6
    jmp .bottom_done
.no_bottom:
    movu [px+32*(%2+0)- 4], m6
    movu [px+32*(%2+1)- 4], m6
 %if %1 == 8
    movq [px+32*(%2+0)+12], m6
    movq [px+32*(%2+1)+12], m6
 %endif
.bottom_done:

    ; actual filter
 %if ARCH_X86_64
    DEFINE_ARGS dst, stride, _, pridmp, damping, pri, sec
    mova           m13, [shufb_lohi]
 %if cpuflag(ssse3)
    mova           m15, [pw_2048]
 %else
    mova           m15, [pw_8]
 %endif
    mova           m14, m6
 %else
    DEFINE_ARGS dst, pridmp, sec, damping, pri, tap
    %xdefine        m8  m1
    %xdefine        m9  m2
    %xdefine       m10  m0
    %xdefine       m13  [base+shufb_lohi]
    %xdefine       m14  OUT_OF_BOUNDS_MEM
 %if cpuflag(ssse3)
    %xdefine       m15  [base+pw_2048]
 %else
    %xdefine       m15  [base+pw_8]
 %endif
 %endif
    movifnidn     prid, r5m
    movifnidn     secd, r6m
    mov       dampingd, r8m
    movif32 [esp+0x3C], r1d
    test          prid, prid
    jz .sec_only
    movd            m1, r5m
    bsr        pridmpd, prid
    test          secd, secd
    jz .pri_only
    movd           m10, r6m
    tzcnt         secd, secd
    and           prid, 1
    sub        pridmpd, dampingd
    sub           secd, dampingd
    xor       dampingd, dampingd
    add           prid, prid
    neg        pridmpd
    cmovs      pridmpd, dampingd
    neg           secd
    PSHUFB_0        m1, m7
    PSHUFB_0       m10, m7
 %if ARCH_X86_64
    DEFINE_ARGS dst, stride, _, pridmp, tap, pri, sec
    lea           tapq, [tap_table]
    MOVDDUP        m11, [tapq+pridmpq*8] ; pri_shift_mask
    MOVDDUP        m12, [tapq+secq*8]    ; sec_shift_mask
    mov     [rsp+0x00], pridmpq          ; pri_shift
    mov     [rsp+0x10], secq             ; sec_shift
    DEFINE_ARGS dst, stride, h, dir, tap, pri, stk, k, off
 %else
    MOVDDUP         m2, [tapq+pridmpq*8]
    MOVDDUP         m3, [tapq+secq*8]
    mov     [esp+0x04], dampingd         ; zero upper 32 bits of psrlw
    mov     [esp+0x34], dampingd         ; source operand in ACCUMULATE_TAP
    mov     [esp+0x00], pridmpd
    mov     [esp+0x30], secd
    DEFINE_ARGS dst, stride, dir, stk, pri, tap, h
  %define         offq  dstq
  %define           kd  strided
  %define           kq  strideq
    mova    [esp+0x10], m2
    mova    [esp+0x40], m3
    mova    [esp+0x20], m1
    mova    [esp+0x50], m10
 %endif
    mov           dird, r7m
    lea           stkq, [px]
    lea           priq, [tapq+8*8+priq*8] ; pri_taps
    mov             hd, %1*%2/8
    lea           dirq, [tapq+dirq*2]
.v_loop:
    movif32 [esp+0x38], dstd
    mov             kd, 1
 %if %1 == 4
    movq            m4, [stkq+32*0]
    movhps          m4, [stkq+32*1]
 %else
    mova            m4, [stkq+32*0]       ; px
 %endif
    pxor            m0, m0                ; sum
    mova            m7, m4                ; max
    mova            m8, m4                ; min
.k_loop:
    MOVDDUP         m2, [priq+kq*8]
 %if ARCH_X86_64
    ACCUMULATE_TAP 0*2, [rsp+0x00], m11, m1, m2, %1, 1
    MOVDDUP         m2, [tapq+12*8+kq*8]
    ACCUMULATE_TAP 2*2, [rsp+0x10], m12, m10, m2, %1, 1
    ACCUMULATE_TAP 6*2, [rsp+0x10], m12, m10, m2, %1, 1
 %else
    ACCUMULATE_TAP 0*2, [esp+0x00], [esp+0x10], [esp+0x20], m2, %1, 1
    MOVDDUP         m2, [tapq+12*8+kq*8]
    ACCUMULATE_TAP 2*2, [esp+0x30], [esp+0x40], [esp+0x50], m2, %1, 1
    MOVDDUP         m2, [tapq+12*8+kq*8]
    ACCUMULATE_TAP 6*2, [esp+0x30], [esp+0x40], [esp+0x50], m2, %1, 1
 %endif
    dec             kd
    jge .k_loop
    movif32       dstq, [esp+0x38]
    movif32    strideq, [esp+0x3C]
    CDEF_FILTER_END %1, 1
    dec             hd
    jg .v_loop
    RET

.pri_only:
%if ARCH_X86_64
    DEFINE_ARGS dst, stride, zero, pridmp, damping, pri, tap
    lea           tapq, [tap_table]
 %else
    DEFINE_ARGS dst, pridmp, zero, damping, pri, tap
 %endif
    and           prid, 1
    xor          zerod, zerod
    sub       dampingd, pridmpd
    cmovs     dampingd, zerod
    add           prid, prid
    PSHUFB_0        m1, m7
    MOVDDUP         m7, [tapq+dampingq*8]
    mov     [rsp+0x00], dampingq
 %if ARCH_X86_64
    DEFINE_ARGS dst, stride, h, dir, stk, pri, tap, k, off
 %else
    mov     [rsp+0x04], zerod
    DEFINE_ARGS dst, stride, dir, stk, pri, tap, h
 %endif
    mov           dird, r7m
    lea           stkq, [px]
    lea           priq, [tapq+8*8+priq*8]
    mov             hd, %1*%2/8
    lea           dirq, [tapq+dirq*2]
.pri_v_loop:
    movif32 [esp+0x38], dstd
    mov             kd, 1
 %if %1 == 4
    movq            m4, [stkq+32*0]
    movhps          m4, [stkq+32*1]
 %else
    mova            m4, [stkq+32*0]
 %endif
    pxor            m0, m0
.pri_k_loop:
    MOVDDUP         m2, [priq+kq*8]
    ACCUMULATE_TAP 0*2, [rsp], m7, m1, m2, %1, 0
    dec             kd
    jge .pri_k_loop
    movif32       dstq, [esp+0x38]
    movif32    strideq, [esp+0x3C]
    CDEF_FILTER_END %1, 0
    dec             hd
    jg .pri_v_loop
    RET

.sec_only:
%if ARCH_X86_64
    DEFINE_ARGS dst, stride, zero, dir, damping, tap, sec
%else
    DEFINE_ARGS dst, stride, sec, damping, dir, tap, zero
%endif
    movd            m1, r6m
    tzcnt         secd, secd
    mov           dird, r7m
    xor          zerod, zerod
    sub       dampingd, secd
    cmovs     dampingd, zerod
    PSHUFB_0        m1, m7
 %if ARCH_X86_64
    lea           tapq, [tap_table]
 %else
    mov     [rsp+0x04], zerod
 %endif
    mov     [rsp+0x00], dampingq
    MOVDDUP         m7, [tapq+dampingq*8]
    lea           dirq, [tapq+dirq*2]
 %if ARCH_X86_64
    DEFINE_ARGS dst, stride, h, dir, stk, tap, off, k
 %else
    DEFINE_ARGS dst, stride, off, stk, dir, tap, h
 %endif
    lea           stkq, [px]
    mov             hd, %1*%2/8
.sec_v_loop:
    mov             kd, 1
 %if %1 == 4
    movq            m4, [stkq+32*0]
    movhps          m4, [stkq+32*1]
 %else
    mova            m4, [stkq+32*0]
 %endif
    pxor            m0, m0
.sec_k_loop:
    MOVDDUP         m2, [tapq+12*8+kq*8]
    ACCUMULATE_TAP 2*2, [rsp], m7, m1, m2, %1, 0
 %if ARCH_X86_32
    MOVDDUP         m2, [tapq+12*8+kq*8]
 %endif
    ACCUMULATE_TAP 6*2, [rsp], m7, m1, m2, %1, 0
    dec             kd
    jge .sec_k_loop
    movif32    strideq, [esp+0x3C]
    CDEF_FILTER_END %1, 0
    dec             hd
    jg .sec_v_loop
    RET
%endmacro

%macro MULLD 2
 %if cpuflag(sse4)
    pmulld          %1, %2
 %else
  %if ARCH_X86_32
   %define m15 m1
  %endif
    pmulhuw        m15, %1, %2
    pmullw          %1, %2
    pslld          m15, 16
    paddd           %1, m15
 %endif
%endmacro

%macro CDEF_DIR 0
 %if ARCH_X86_64
cglobal cdef_dir_8bpc, 3, 7, 16, src, stride, var
    lea             r6, [strideq*3]
    movq            m1, [srcq+strideq*0]
    movhps          m1, [srcq+strideq*1]
    movq            m3, [srcq+strideq*2]
    movhps          m3, [srcq+r6       ]
    lea           srcq, [srcq+strideq*4]
    movq            m5, [srcq+strideq*0]
    movhps          m5, [srcq+strideq*1]
    movq            m7, [srcq+strideq*2]
    movhps          m7, [srcq+r6       ]

    pxor            m8, m8
    psadbw          m9, m1, m8
    psadbw          m2, m3, m8
    psadbw          m4, m5, m8
    psadbw          m6, m7, m8
    packssdw        m9, m2
    packssdw        m4, m6
    packssdw        m9, m4

    punpcklbw       m0, m1, m8
    punpckhbw       m1, m8
    punpcklbw       m2, m3, m8
    punpckhbw       m3, m8
    punpcklbw       m4, m5, m8
    punpckhbw       m5, m8
    punpcklbw       m6, m7, m8
    punpckhbw       m7, m8
cglobal_label .main
    mova            m8, [pw_128]
    psubw           m0, m8
    psubw           m1, m8
    psubw           m2, m8
    psubw           m3, m8
    psubw           m4, m8
    psubw           m5, m8
    psubw           m6, m8
    psubw           m7, m8
    psllw           m8, 3
    psubw           m9, m8                  ; partial_sum_hv[0]

    paddw           m8, m0, m1
    paddw          m10, m2, m3
    paddw           m8, m4
    paddw          m10, m5
    paddw           m8, m6
    paddw          m10, m7
    paddw           m8, m10                 ; partial_sum_hv[1]

    pmaddwd         m8, m8
    pmaddwd         m9, m9
    phaddd          m9, m8
    SWAP            m8, m9
    MULLD           m8, [div_table%+SUFFIX+48]

    pslldq          m9, m1, 2
    psrldq         m10, m1, 14
    pslldq         m11, m2, 4
    psrldq         m12, m2, 12
    pslldq         m13, m3, 6
    psrldq         m14, m3, 10
    paddw           m9, m0
    paddw          m10, m12
    paddw          m11, m13
    paddw          m10, m14                 ; partial_sum_diag[0] top/right half
    paddw           m9, m11                 ; partial_sum_diag[0] top/left half
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
    paddw           m9, m13                 ; partial_sum_diag[0][0-7]
    paddw          m10, m14                 ; partial_sum_diag[0][8-14,zero]
    pshufb         m10, [shufw_6543210x]
    punpckhwd      m11, m9, m10
    punpcklwd       m9, m10
    pmaddwd        m11, m11
    pmaddwd         m9, m9
    MULLD          m11, [div_table%+SUFFIX+16]
    MULLD           m9, [div_table%+SUFFIX+0]
    paddd           m9, m11                 ; cost[0a-d]

    pslldq         m10, m0, 14
    psrldq         m11, m0, 2
    pslldq         m12, m1, 12
    psrldq         m13, m1, 4
    pslldq         m14, m2, 10
    psrldq         m15, m2, 6
    paddw          m10, m12
    paddw          m11, m13
    paddw          m10, m14
    paddw          m11, m15
    pslldq         m12, m3, 8
    psrldq         m13, m3, 8
    pslldq         m14, m4, 6
    psrldq         m15, m4, 10
    paddw          m10, m12
    paddw          m11, m13
    paddw          m10, m14
    paddw          m11, m15
    pslldq         m12, m5, 4
    psrldq         m13, m5, 12
    pslldq         m14, m6, 2
    psrldq         m15, m6, 14
    paddw          m10, m12
    paddw          m11, m13
    paddw          m10, m14
    paddw          m11, m15                 ; partial_sum_diag[1][8-14,zero]
    paddw          m10, m7                  ; partial_sum_diag[1][0-7]
    pshufb         m11, [shufw_6543210x]
    punpckhwd      m12, m10, m11
    punpcklwd      m10, m11
    pmaddwd        m12, m12
    pmaddwd        m10, m10
    MULLD          m12, [div_table%+SUFFIX+16]
    MULLD          m10, [div_table%+SUFFIX+0]
    paddd          m10, m12                 ; cost[4a-d]
    phaddd          m9, m10                 ; cost[0a/b,4a/b]

    paddw          m10, m0, m1
    paddw          m11, m2, m3
    paddw          m12, m4, m5
    paddw          m13, m6, m7
    phaddw          m0, m4
    phaddw          m1, m5
    phaddw          m2, m6
    phaddw          m3, m7

    ; m0-3 are horizontal sums (x >> 1), m10-13 are vertical sums (y >> 1)
    pslldq          m4, m11, 2
    psrldq          m5, m11, 14
    pslldq          m6, m12, 4
    psrldq          m7, m12, 12
    pslldq         m14, m13, 6
    psrldq         m15, m13, 10
    paddw           m4, m10
    paddw           m5, m7
    paddw           m4, m6
    paddw           m5, m15                 ; partial_sum_alt[3] right
    paddw           m4, m14                 ; partial_sum_alt[3] left
    pshuflw         m6, m5, q3012
    punpckhwd       m5, m4
    punpcklwd       m4, m6
    pmaddwd         m5, m5
    pmaddwd         m4, m4
    MULLD           m5, [div_table%+SUFFIX+48]
    MULLD           m4, [div_table%+SUFFIX+32]
    paddd           m4, m5                  ; cost[7a-d]

    pslldq          m5, m10, 6
    psrldq          m6, m10, 10
    pslldq          m7, m11, 4
    psrldq         m10, m11, 12
    pslldq         m11, m12, 2
    psrldq         m12, 14
    paddw           m5, m7
    paddw           m6, m10
    paddw           m5, m11
    paddw           m6, m12
    paddw           m5, m13
    pshuflw         m7, m6, q3012
    punpckhwd       m6, m5
    punpcklwd       m5, m7
    pmaddwd         m6, m6
    pmaddwd         m5, m5
    MULLD           m6, [div_table%+SUFFIX+48]
    MULLD           m5, [div_table%+SUFFIX+32]
    paddd           m5, m6                  ; cost[5a-d]

    pslldq          m6, m1, 2
    psrldq          m7, m1, 14
    pslldq         m10, m2, 4
    psrldq         m11, m2, 12
    pslldq         m12, m3, 6
    psrldq         m13, m3, 10
    paddw           m6, m0
    paddw           m7, m11
    paddw           m6, m10
    paddw           m7, m13                 ; partial_sum_alt[3] right
    paddw           m6, m12                 ; partial_sum_alt[3] left
    pshuflw        m10, m7, q3012
    punpckhwd       m7, m6
    punpcklwd       m6, m10
    pmaddwd         m7, m7
    pmaddwd         m6, m6
    MULLD           m7, [div_table%+SUFFIX+48]
    MULLD           m6, [div_table%+SUFFIX+32]
    paddd           m6, m7                  ; cost[1a-d]

    pshufd          m0, m0, q1032
    pshufd          m1, m1, q1032
    pshufd          m2, m2, q1032
    pshufd          m3, m3, q1032

    pslldq         m10, m0, 6
    psrldq         m11, m0, 10
    pslldq         m12, m1, 4
    psrldq         m13, m1, 12
    pslldq         m14, m2, 2
    psrldq          m2, 14
    paddw          m10, m12
    paddw          m11, m13
    paddw          m10, m14
    paddw          m11, m2
    paddw          m10, m3
    pshuflw        m12, m11, q3012
    punpckhwd      m11, m10
    punpcklwd      m10, m12
    pmaddwd        m11, m11
    pmaddwd        m10, m10
    MULLD          m11, [div_table%+SUFFIX+48]
    MULLD          m10, [div_table%+SUFFIX+32]
    paddd          m10, m11                 ; cost[3a-d]

    phaddd          m9, m8                  ; cost[0,4,2,6]
    phaddd          m6, m10
    phaddd          m5, m4
    phaddd          m6, m5                  ; cost[1,3,5,7]
    pshufd          m4, m9, q3120

    ; now find the best cost
  %if cpuflag(sse4)
    pmaxsd          m9, m6
    pshufd          m0, m9, q1032
    pmaxsd          m0, m9
    pshufd          m1, m0, q2301
    pmaxsd          m0, m1                  ; best cost
  %else
    pcmpgtd         m0, m9, m6
    pand            m9, m0
    pandn           m0, m6
    por             m9, m0
    pshufd          m1, m9, q1032
    pcmpgtd         m0, m9, m1
    pand            m9, m0
    pandn           m0, m1
    por             m9, m0
    pshufd          m1, m9, q2301
    pcmpgtd         m0, m9, m1
    pand            m9, m0
    pandn           m0, m1
    por             m0, m9
  %endif

    ; get direction and variance
    punpckhdq       m1, m4, m6
    punpckldq       m4, m6
    psubd           m2, m0, m1
    psubd           m3, m0, m4
%if WIN64
    WIN64_RESTORE_XMM
    %define tmp rsp+stack_offset+8
%else
    %define tmp rsp-40
%endif
    mova    [tmp+0x00], m2                  ; emulate ymm in stack
    mova    [tmp+0x10], m3
    pcmpeqd         m1, m0                  ; compute best cost mask
    pcmpeqd         m4, m0
    packssdw        m4, m1
    pmovmskb       eax, m4                  ; get byte-idx from mask
    tzcnt          eax, eax
    mov            r1d, [tmp+rax*2]         ; get idx^4 complement from emulated ymm
    shr            eax, 1                   ; get direction by converting byte-idx to word-idx
    shr            r1d, 10
    mov         [varq], r1d
 %else
cglobal cdef_dir_8bpc, 2, 4, 8, 96, src, stride, var, stride3
%define base r2-shufw_6543210x
    LEA             r2, shufw_6543210x
    pxor            m0, m0
    lea       stride3q, [strideq*3]
    movq            m5, [srcq+strideq*0]
    movhps          m5, [srcq+strideq*1]
    movq            m7, [srcq+strideq*2]
    movhps          m7, [srcq+stride3q]
    mova            m1, [base+pw_128]
    psadbw          m2, m5, m0
    psadbw          m3, m7, m0
    packssdw        m2, m3
    punpcklbw       m4, m5, m0
    punpckhbw       m5, m0
    punpcklbw       m6, m7, m0
    punpckhbw       m7, m0
    psubw           m4, m1
    psubw           m5, m1
    psubw           m6, m1
    psubw           m7, m1

    mova    [esp+0x00], m4
    mova    [esp+0x10], m5
    mova    [esp+0x20], m6
    mova    [esp+0x50], m7

    lea           srcq, [srcq+strideq*4]
    movq            m5, [srcq+strideq*0]
    movhps          m5, [srcq+strideq*1]
    movq            m7, [srcq+strideq*2]
    movhps          m7, [srcq+stride3q]
    psadbw          m3, m5, m0
    psadbw          m0, m7
    packssdw        m3, m0
    pxor            m0, m0
    punpcklbw       m4, m5, m0
    punpckhbw       m5, m0
    punpcklbw       m6, m7, m0
    punpckhbw       m7, m0
cglobal_label .main
    psubw           m4, m1
    psubw           m5, m1
    psubw           m6, m1
    psubw           m7, m1
    packssdw        m2, m3
    psllw           m1, 3
    psubw           m2, m1                  ; partial_sum_hv[0]
    pmaddwd         m2, m2

    mova            m3, [esp+0x50]
    mova            m0, [esp+0x00]
    paddw           m0, [esp+0x10]
    paddw           m1, m3, [esp+0x20]
    paddw           m0, m4
    paddw           m1, m5
    paddw           m0, m6
    paddw           m1, m7
    paddw           m0, m1                  ; partial_sum_hv[1]
    pmaddwd         m0, m0

    phaddd          m2, m0
    MULLD           m2, [base+div_table%+SUFFIX+48]
    mova    [esp+0x30], m2

    mova            m1, [esp+0x10]
    pslldq          m0, m1, 2
    psrldq          m1, 14
    paddw           m0, [esp+0x00]
    pslldq          m2, m3, 6
    psrldq          m3, 10
    paddw           m0, m2
    paddw           m1, m3
    mova            m3, [esp+0x20]
    pslldq          m2, m3, 4
    psrldq          m3, 12
    paddw           m0, m2                  ; partial_sum_diag[0] top/left half
    paddw           m1, m3                  ; partial_sum_diag[0] top/right half
    pslldq          m2, m4, 8
    psrldq          m3, m4, 8
    paddw           m0, m2
    paddw           m1, m3
    pslldq          m2, m5, 10
    psrldq          m3, m5, 6
    paddw           m0, m2
    paddw           m1, m3
    pslldq          m2, m6, 12
    psrldq          m3, m6, 4
    paddw           m0, m2
    paddw           m1, m3
    pslldq          m2, m7, 14
    psrldq          m3, m7, 2
    paddw           m0, m2                  ; partial_sum_diag[0][0-7]
    paddw           m1, m3                  ; partial_sum_diag[0][8-14,zero]
    mova            m3, [esp+0x50]
    pshufb          m1, [base+shufw_6543210x]
    punpckhwd       m2, m0, m1
    punpcklwd       m0, m1
    pmaddwd         m2, m2
    pmaddwd         m0, m0
    MULLD           m2, [base+div_table%+SUFFIX+16]
    MULLD           m0, [base+div_table%+SUFFIX+ 0]
    paddd           m0, m2                  ; cost[0a-d]
    mova    [esp+0x40], m0

    mova            m1, [esp+0x00]
    pslldq          m0, m1, 14
    psrldq          m1, 2
    paddw           m0, m7
    pslldq          m2, m3, 8
    psrldq          m3, 8
    paddw           m0, m2
    paddw           m1, m3
    mova            m3, [esp+0x20]
    pslldq          m2, m3, 10
    psrldq          m3, 6
    paddw           m0, m2
    paddw           m1, m3
    mova            m3, [esp+0x10]
    pslldq          m2, m3, 12
    psrldq          m3, 4
    paddw           m0, m2
    paddw           m1, m3
    pslldq          m2, m4, 6
    psrldq          m3, m4, 10
    paddw           m0, m2
    paddw           m1, m3
    pslldq          m2, m5, 4
    psrldq          m3, m5, 12
    paddw           m0, m2
    paddw           m1, m3
    pslldq          m2, m6, 2
    psrldq          m3, m6, 14
    paddw           m0, m2                  ; partial_sum_diag[1][0-7]
    paddw           m1, m3                  ; partial_sum_diag[1][8-14,zero]
    mova            m3, [esp+0x50]
    pshufb          m1, [base+shufw_6543210x]
    punpckhwd       m2, m0, m1
    punpcklwd       m0, m1
    pmaddwd         m2, m2
    pmaddwd         m0, m0
    MULLD           m2, [base+div_table%+SUFFIX+16]
    MULLD           m0, [base+div_table%+SUFFIX+ 0]
    paddd           m0, m2                  ; cost[4a-d]
    phaddd          m1, [esp+0x40], m0      ; cost[0a/b,4a/b]
    phaddd          m1, [esp+0x30]          ; cost[0,4,2,6]
    mova    [esp+0x30], m1

    phaddw          m0, [esp+0x00], m4
    phaddw          m1, [esp+0x10], m5
    paddw           m4, m5
    mova            m2, [esp+0x20]
    paddw           m5, m2, m3
    phaddw          m2, m6
    paddw           m6, m7
    phaddw          m3, m7
    mova            m7, [esp+0x00]
    paddw           m7, [esp+0x10]
    mova    [esp+0x00], m0
    mova    [esp+0x10], m1
    mova    [esp+0x20], m2

    pslldq          m1, m4, 4
    pslldq          m2, m6, 6
    pslldq          m0, m5, 2
    paddw           m1, m2
    paddw           m0, m7
    psrldq          m2, m5, 14
    paddw           m0, m1                  ; partial_sum_alt[3] left
    psrldq          m1, m4, 12
    paddw           m1, m2
    psrldq          m2, m6, 10
    paddw           m1, m2                  ; partial_sum_alt[3] right
    pshuflw         m1, m1, q3012
    punpckhwd       m2, m0, m1
    punpcklwd       m0, m1
    pmaddwd         m2, m2
    pmaddwd         m0, m0
    MULLD           m2, [base+div_table%+SUFFIX+48]
    MULLD           m0, [base+div_table%+SUFFIX+32]
    paddd           m0, m2                  ; cost[7a-d]
    mova    [esp+0x40], m0

    pslldq          m0, m7, 6
    psrldq          m7, 10
    pslldq          m1, m5, 4
    psrldq          m5, 12
    pslldq          m2, m4, 2
    psrldq          m4, 14
    paddw           m0, m6
    paddw           m7, m5
    paddw           m0, m1
    paddw           m7, m4
    paddw           m0, m2
    pshuflw         m2, m7, q3012
    punpckhwd       m7, m0
    punpcklwd       m0, m2
    pmaddwd         m7, m7
    pmaddwd         m0, m0
    MULLD           m7, [base+div_table%+SUFFIX+48]
    MULLD           m0, [base+div_table%+SUFFIX+32]
    paddd           m0, m7                  ; cost[5a-d]
    mova    [esp+0x50], m0

    mova            m7, [esp+0x10]
    mova            m2, [esp+0x20]
    pslldq          m0, m7, 2
    psrldq          m7, 14
    pslldq          m4, m2, 4
    psrldq          m2, 12
    pslldq          m5, m3, 6
    psrldq          m6, m3, 10
    paddw           m0, [esp+0x00]
    paddw           m7, m2
    paddw           m4, m5
    paddw           m7, m6                  ; partial_sum_alt[3] right
    paddw           m0, m4                  ; partial_sum_alt[3] left
    pshuflw         m2, m7, q3012
    punpckhwd       m7, m0
    punpcklwd       m0, m2
    pmaddwd         m7, m7
    pmaddwd         m0, m0
    MULLD           m7, [base+div_table%+SUFFIX+48]
    MULLD           m0, [base+div_table%+SUFFIX+32]
    paddd           m0, m7                  ; cost[1a-d]
    SWAP            m0, m4

    pshufd          m0, [esp+0x00], q1032
    pshufd          m1, [esp+0x10], q1032
    pshufd          m2, [esp+0x20], q1032
    pshufd          m3, m3, q1032
    mova    [esp+0x00], m4

    pslldq          m4, m0, 6
    psrldq          m0, 10
    pslldq          m5, m1, 4
    psrldq          m1, 12
    pslldq          m6, m2, 2
    psrldq          m2, 14
    paddw           m4, m3
    paddw           m0, m1
    paddw           m5, m6
    paddw           m0, m2
    paddw           m4, m5
    pshuflw         m2, m0, q3012
    punpckhwd       m0, m4
    punpcklwd       m4, m2
    pmaddwd         m0, m0
    pmaddwd         m4, m4
    MULLD           m0, [base+div_table%+SUFFIX+48]
    MULLD           m4, [base+div_table%+SUFFIX+32]
    paddd           m4, m0                   ; cost[3a-d]

    mova            m1, [esp+0x00]
    mova            m2, [esp+0x50]
    mova            m0, [esp+0x30]          ; cost[0,4,2,6]
    phaddd          m1, m4
    phaddd          m2, [esp+0x40]          ; cost[1,3,5,7]
    phaddd          m1, m2
    pshufd          m2, m0, q3120

    ; now find the best cost
  %if cpuflag(sse4)
    pmaxsd          m0, m1
    pshufd          m3, m0, q1032
    pmaxsd          m3, m0
    pshufd          m0, m3, q2301
    pmaxsd          m0, m3
  %else
    pcmpgtd         m3, m0, m1
    pand            m0, m3
    pandn           m3, m1
    por             m0, m3
    pshufd          m4, m0, q1032
    pcmpgtd         m3, m0, m4
    pand            m0, m3
    pandn           m3, m4
    por             m0, m3
    pshufd          m4, m0, q2301
    pcmpgtd         m3, m0, m4
    pand            m0, m3
    pandn           m3, m4
    por             m0, m3
  %endif

    ; get direction and variance
    mov           vard, varm
    punpckhdq       m3, m2, m1
    punpckldq       m2, m1
    psubd           m1, m0, m3
    psubd           m4, m0, m2
    mova    [esp+0x00], m1                  ; emulate ymm in stack
    mova    [esp+0x10], m4
    pcmpeqd         m3, m0                  ; compute best cost mask
    pcmpeqd         m2, m0
    packssdw        m2, m3
    pmovmskb       eax, m2                  ; get byte-idx from mask
    tzcnt          eax, eax
    mov            r1d, [esp+eax*2]         ; get idx^4 complement from emulated ymm
    shr            eax, 1                   ; get direction by converting byte-idx to word-idx
    shr            r1d, 10
    mov         [vard], r1d
 %endif

    RET
%endmacro

INIT_XMM sse4
CDEF_FILTER 8, 8
CDEF_FILTER 4, 8
CDEF_FILTER 4, 4
CDEF_DIR

INIT_XMM ssse3
CDEF_FILTER 8, 8
CDEF_FILTER 4, 8
CDEF_FILTER 4, 4
CDEF_DIR

INIT_XMM sse2
CDEF_FILTER 8, 8
CDEF_FILTER 4, 8
CDEF_FILTER 4, 4
