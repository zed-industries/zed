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
pd_47130256: dd 4, 7, 1, 3, 0, 2, 5, 6
div_table: dd 840, 420, 280, 210, 168, 140, 120, 105
           dd 420, 210, 140, 105
shufw_6543210x: db 12, 13, 10, 11, 8, 9, 6, 7, 4, 5, 2, 3, 0, 1, 14, 15
shufb_lohi: db 0, 8, 1, 9, 2, 10, 3, 11, 4, 12, 5, 13, 6, 14, 7, 15
tap_table: ; masks for 8 bit shifts
           db 0xFF, 0x7F, 0x3F, 0x1F, 0x0F, 0x07, 0x03, 0x01
           ; weights
           db 4, 2, 3, 3, 2, 1
           db 0, 0 ; padding
           db -1, -2,  0,  0,  1,  2,  0,  0
           db  0, -1,  0,  1,  1,  2,  0,  0
           db  0,  0,  1,  2, -1, -2,  0,  0
           db  0,  1,  1,  2,  0, -1,  0,  0
           db  1,  2,  1,  2,  0,  0,  0,  0
           db  1,  2,  1,  2,  0,  1,  0,  0
           db  1,  2, -1, -2,  1,  2,  0,  0
           db  1,  2,  0, -1,  1,  2,  0,  0

           db  1,  2,  1,  2,  0,  0,  0,  0
           db  1,  2,  1,  2,  0, -1,  0,  0
           db  1,  2,  1,  2,  1,  2,  0,  0
           db  1,  2,  0,  1,  1,  2,  0,  0
           db  1,  2,  0,  0,  1,  2,  0,  0
           db  0,  1,  0, -1,  1,  2,  0,  0
           db  0,  0,  1,  2,  1,  2,  0,  0
           db  0, -1,  1,  2,  0,  1,  0,  0
pw_128: times 2 dw 128
pw_2048: times 2 dw 2048

SECTION .text

; stride unused
%macro ACCUMULATE_TAP 7 ; tap_offset, shift, mask, strength, mul_tap, w, stride
    ; load p0/p1
    movsx         offq, dword [offsets+kq*4+%1] ; off1
%if %6 == 4
    movq           xm5, [tmp0q+offq*2]          ; p0
    movq           xm6, [tmp2q+offq*2]
    movhps         xm5, [tmp1q+offq*2]
    movhps         xm6, [tmp3q+offq*2]
    vinserti128     m5, xm6, 1
%else
    movu           xm5, [tmp0q+offq*2]          ; p0
    vinserti128     m5, [tmp1q+offq*2], 1
%endif
    neg           offq                          ; -off1
%if %6 == 4
    movq           xm6, [tmp0q+offq*2]          ; p1
    movq           xm9, [tmp2q+offq*2]
    movhps         xm6, [tmp1q+offq*2]
    movhps         xm9, [tmp3q+offq*2]
    vinserti128     m6, xm9, 1
%else
    movu           xm6, [tmp0q+offq*2]          ; p1
    vinserti128     m6, [tmp1q+offq*2], 1
%endif
    ; out of bounds values are set to a value that is a both a large unsigned
    ; value and a negative signed value.
    ; use signed max and unsigned min to remove them
    pmaxsw          m7, m5                      ; max after p0
    pminuw          m8, m5                      ; min after p0
    pmaxsw          m7, m6                      ; max after p1
    pminuw          m8, m6                      ; min after p1

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

%macro CDEF_FILTER 2 ; w, h
INIT_YMM avx2
%if %1*%2*2/mmsize > 1
 %if %1 == 4
cglobal cdef_filter_%1x%2, 4, 13, 16, 64
 %else
cglobal cdef_filter_%1x%2, 4, 11, 16, 64
 %endif
%else
cglobal cdef_filter_%1x%2, 4, 12, 16, 64
%endif
%define offsets rsp+32
    DEFINE_ARGS dst, dst_stride, tmp, tmp_stride, pri, sec, pridmp, table, \
                secdmp, damping, dir
    lea         tableq, [tap_table]

    ; off1/2/3[k] [6 total]
    movd           xm2, tmp_strided
    vpbroadcastd    m2, xm2
    mov           dird, r6m
    pmovsxbd        m3, [tableq+dirq*8+16]
    pmulld          m2, m3
    pmovsxbd        m4, [tableq+dirq*8+16+64]
    paddd           m2, m4
    mova     [offsets], m2

    ; register to shuffle values into after packing
    vbroadcasti128 m12, [shufb_lohi]

    movifnidn     prid, prim
    mov       dampingd, r7m
    lzcnt      pridmpd, prid
%if UNIX64
    movd           xm0, prid
    movd           xm1, secd
%endif
    lzcnt      secdmpd, secm
    sub       dampingd, 31
    DEFINE_ARGS dst, dst_stride, tmp, tmp_stride, pri, sec, pridmp, table, \
                secdmp, damping, zero
    xor          zerod, zerod
    add        pridmpd, dampingd
    cmovl      pridmpd, zerod
    add        secdmpd, dampingd
    cmovl      secdmpd, zerod
    mov        [rsp+0], pridmpq                 ; pri_shift
    mov        [rsp+8], secdmpq                 ; sec_shift

    DEFINE_ARGS dst, dst_stride, tmp, tmp_stride, pri, sec, pridmp, table, \
                secdmp
    vpbroadcastb   m13, [tableq+pridmpq]        ; pri_shift_mask
    vpbroadcastb   m14, [tableq+secdmpq]        ; sec_shift_mask

    ; pri/sec_taps[k] [4 total]
    DEFINE_ARGS dst, dst_stride, tmp, tmp_stride, pri, sec, dummy, table, \
                secdmp
%if UNIX64
    vpbroadcastb    m0, xm0                     ; pri_strength
    vpbroadcastb    m1, xm1                     ; sec_strength
%else
    vpbroadcastb    m0, prim
    vpbroadcastb    m1, secm
%endif
    and           prid, 1
    lea           priq, [tableq+priq*2+8]       ; pri_taps
    lea           secq, [tableq+12]             ; sec_taps
%if %1*%2*2/mmsize > 1
 %if %1 == 4
    DEFINE_ARGS dst, dst_stride, tmp0, tmp_stride, pri, sec, dst_stride3, h, off, k, tmp1, tmp2, tmp3
    lea   dst_stride3q, [dst_strideq*3]
 %else
    DEFINE_ARGS dst, dst_stride, tmp0, tmp_stride, pri, sec, h, off, k, tmp1
 %endif
    mov             hd, %1*%2*2/mmsize
%else
    DEFINE_ARGS dst, dst_stride, tmp0, tmp_stride, pri, sec, dst_stride3, off, k, tmp1, tmp2, tmp3
    lea   dst_stride3q, [dst_strideq*3]
%endif
    pxor           m11, m11
%if %1*%2*2/mmsize > 1
.v_loop:
%endif
    lea          tmp1q, [tmp0q+tmp_strideq*2]
%if %1 == 4
    lea          tmp2q, [tmp0q+tmp_strideq*4]
    lea          tmp3q, [tmp1q+tmp_strideq*4]
%endif
    mov             kd, 1
%if %1 == 4
    movq           xm4, [tmp0q]
    movhps         xm4, [tmp1q]
    movq           xm5, [tmp2q]
    movhps         xm5, [tmp3q]
    vinserti128     m4, xm5, 1
%else
    mova           xm4, [tmp0q]                  ; px
    vinserti128     m4, [tmp1q], 1
%endif
    pxor           m15, m15                     ; sum
    mova            m7, m4                      ; max
    mova            m8, m4                      ; min
.k_loop:
    vpbroadcastb    m2, [priq+kq]               ; pri_taps
    vpbroadcastb    m3, [secq+kq]               ; sec_taps

    ACCUMULATE_TAP 0*8, [rsp+0], m13, m0, m2, %1, %3
    ACCUMULATE_TAP 1*8, [rsp+8], m14, m1, m3, %1, %3
    ACCUMULATE_TAP 2*8, [rsp+8], m14, m1, m3, %1, %3

    dec             kq
    jge .k_loop

    vpbroadcastd   m10, [pw_2048]
    pcmpgtw         m9, m11, m15
    paddw          m15, m9
    pmulhrsw       m15, m10
    paddw           m4, m15
    pminsw          m4, m7
    pmaxsw          m4, m8
    packuswb        m4, m4
    vextracti128   xm5, m4, 1
%if %1 == 4
    movd [dstq+dst_strideq*0], xm4
    pextrd [dstq+dst_strideq*1], xm4, 1
    movd [dstq+dst_strideq*2], xm5
    pextrd [dstq+dst_stride3q], xm5, 1
%else
    movq [dstq+dst_strideq*0], xm4
    movq [dstq+dst_strideq*1], xm5
%endif

%if %1*%2*2/mmsize > 1
 %define vloop_lines (mmsize/(%1*2))
    lea           dstq, [dstq+dst_strideq*vloop_lines]
    lea          tmp0q, [tmp0q+tmp_strideq*2*vloop_lines]
    dec             hd
    jg .v_loop
%endif

    RET
%endmacro

CDEF_FILTER 8, 8
CDEF_FILTER 4, 8
CDEF_FILTER 4, 4

%endif ; ARCH_X86_64
