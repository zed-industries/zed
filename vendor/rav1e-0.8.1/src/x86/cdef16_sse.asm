; Copyright © 2021, VideoLAN and dav1d authors
; Copyright © 2021, Two Orioles, LLC
; Copyright (c) 2017-2021, The rav1e contributors
; Copyright (c) 2021, Nathan Egge
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

%macro DUP8 1-*
    %rep %0
        times 8 dw %1
        %rotate 1
    %endrep
%endmacro

pri_taps:  DUP8 4, 2, 3, 3
dir_table: db  1 * 32 + 0,  2 * 32 + 0
           db  1 * 32 + 0,  2 * 32 - 2
           db -1 * 32 + 2, -2 * 32 + 4
           db  0 * 32 + 2, -1 * 32 + 4
           db  0 * 32 + 2,  0 * 32 + 4
           db  0 * 32 + 2,  1 * 32 + 4
           db  1 * 32 + 2,  2 * 32 + 4
           db  1 * 32 + 0,  2 * 32 + 2
           db  1 * 32 + 0,  2 * 32 + 0
           db  1 * 32 + 0,  2 * 32 - 2
           db -1 * 32 + 2, -2 * 32 + 4
           db  0 * 32 + 2, -1 * 32 + 4

dir_shift: times 4 dw 0x4000
           times 4 dw 0x1000

pw_128:    times 4 dw 128
pw_2048:   times 8 dw 2048
pw_m16384: times 8 dw -16384

cextern cdef_dir_8bpc_ssse3.main
cextern cdef_dir_8bpc_sse4.main
cextern shufw_6543210x

SECTION .text

%if ARCH_X86_32
DECLARE_REG_TMP 5, 3
%elif WIN64
DECLARE_REG_TMP 8, 4
%else
DECLARE_REG_TMP 8, 6
%endif

%macro CDEF_FILTER 2 ; w, h
%if ARCH_X86_64
    DEFINE_ARGS dst, stride, _, tmp, pridmp, pri, sec, dir
    mova            m8, [base+pw_2048]
%else
    DEFINE_ARGS dst, pridmp, tmp, sec, pri, _, dir
    %define         m8  [base+pw_2048]
    %define         m9  [rsp+16*1+gprsize]
    %define        m10  [rsp+16*2+gprsize]
%endif
    movifnidn     prid, r5m
    movifnidn     secd, r6m
    test          prid, prid
    jz .sec_only
    movd            m6, r5m
%if ARCH_X86_32
    mov       [rsp+24], pridmpd
%endif
    bsr        pridmpd, prid
    lea           tmpd, [priq*4]
    cmp     dword r10m, 0x3ff ; if (bpc == 10)
    cmove         prid, tmpd  ;     pri <<= 2
    mov           tmpd, r8m   ; damping
    mov           dird, r7m
    and           prid, 16
    pshufb          m6, m7    ; splat
    lea           dirq, [base+dir_table+dirq*2]
    lea           priq, [base+pri_taps+priq*2]
    test          secd, secd
    jz .pri_only
    mova         [rsp], m6
    movd            m6, secd
    tzcnt         secd, secd
    sub        pridmpd, tmpd
    sub           tmpd, secd
    pshufb          m6, m7
    xor           secd, secd
    neg        pridmpd
    cmovs      pridmpd, secd
%if ARCH_X86_32
    mov  [pri_shift+4], secd
    mov  [sec_shift+4], secd
%endif
    mov  [pri_shift+0], pridmpq
    mov  [sec_shift+0], tmpq
    lea           tmpq, [px]
%if WIN64
    movaps         r4m, m9
    movaps         r6m, m10
%elif ARCH_X86_32
    mov        pridmpd, [rsp+24]
%endif
%rep %1*%2/8
    call mangle(private_prefix %+ _cdef_filter_%1x%1_16bpc %+ SUFFIX).pri_sec
%endrep
%if WIN64
    movaps          m9, r4m
    movaps         m10, r6m
%endif
    jmp .end
.pri_only:
    sub           tmpd, pridmpd
    cmovs         tmpd, secd
%if ARCH_X86_32
    mov        pridmpd, [rsp+24]
    mov  [pri_shift+4], secd
%endif
    mov  [pri_shift+0], tmpq
    lea           tmpq, [px]
%rep %1*%2/8
    call mangle(private_prefix %+ _cdef_filter_%1x%1_16bpc %+ SUFFIX).pri
%endrep
.end:
    RET
.sec_only:
    mov           tmpd, r8m ; damping
    movd            m6, r6m
    tzcnt         secd, secd
    mov           dird, r7m
    pshufb          m6, m7
    sub           tmpd, secd
    lea           dirq, [base+dir_table+dirq*2]
%if ARCH_X86_32
    mov  [sec_shift+4], prid
%endif
    mov  [sec_shift+0], tmpq
    lea           tmpq, [px]
%rep %1*%2/8
    call mangle(private_prefix %+ _cdef_filter_%1x%1_16bpc %+ SUFFIX).sec
%endrep
    jmp .end
%if %1 == %2
 %if ARCH_X86_64
  DEFINE_ARGS dst, stride, _, tmp, off, pri, _, dir
 %else
  DEFINE_ARGS dst, stride, tmp, off, pri, _, dir
 %endif
ALIGN function_align
.pri:
    movsx         offq, byte [dirq+4]    ; off_k0
%if %1 == 4
    movq            m1, [dstq+strideq*0]
    movhps          m1, [dstq+strideq*1]
    movq            m2, [tmpq+offq+32*0] ; k0p0
    movhps          m2, [tmpq+offq+32*1]
    neg           offq
    movq            m3, [tmpq+offq+32*0] ; k0p1
    movhps          m3, [tmpq+offq+32*1]
%else
    mova            m1, [dstq]
    movu            m2, [tmpq+offq]
    neg           offq
    movu            m3, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+5]    ; off_k1
    psubw           m2, m1               ; diff_k0p0
    psubw           m3, m1               ; diff_k0p1
    pabsw           m4, m2               ; adiff_k0p0
    psrlw           m5, m4, [pri_shift+gprsize]
    psubusw         m0, m6, m5
    pabsw           m5, m3               ; adiff_k0p1
    pminsw          m0, m4
    psrlw           m4, m5, [pri_shift+gprsize]
    psignw          m0, m2               ; constrain(diff_k0p0)
    psubusw         m2, m6, m4
    pminsw          m2, m5
%if %1 == 4
    movq            m4, [tmpq+offq+32*0] ; k1p0
    movhps          m4, [tmpq+offq+32*1]
    neg           offq
    movq            m5, [tmpq+offq+32*0] ; k1p1
    movhps          m5, [tmpq+offq+32*1]
%else
    movu            m4, [tmpq+offq]
    neg           offq
    movu            m5, [tmpq+offq]
%endif
    psubw           m4, m1               ; diff_k1p0
    psubw           m5, m1               ; diff_k1p1
    psignw          m2, m3               ; constrain(diff_k0p1)
    pabsw           m3, m4               ; adiff_k1p0
    paddw           m0, m2               ; constrain(diff_k0)
    psrlw           m2, m3, [pri_shift+gprsize]
    psubusw         m7, m6, m2
    pabsw           m2, m5               ; adiff_k1p1
    pminsw          m7, m3
    psrlw           m3, m2, [pri_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k1p0)
    psubusw         m4, m6, m3
    pminsw          m4, m2
    psignw          m4, m5               ; constrain(diff_k1p1)
    paddw           m7, m4               ; constrain(diff_k1)
    pmullw          m0, [priq+16*0]      ; pri_tap_k0
    pmullw          m7, [priq+16*1]      ; pri_tap_k1
    paddw           m0, m7               ; sum
    psraw           m2, m0, 15
    paddw           m0, m2
    pmulhrsw        m0, m8
    paddw           m0, m1
%if %1 == 4
    add           tmpq, 32*2
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea           dstq, [dstq+strideq*2]
%else
    add           tmpq, 32
    mova        [dstq], m0
    add           dstq, strideq
%endif
    ret
ALIGN function_align
.sec:
    movsx         offq, byte [dirq+8]    ; off1_k0
%if %1 == 4
    movq            m1, [dstq+strideq*0]
    movhps          m1, [dstq+strideq*1]
    movq            m2, [tmpq+offq+32*0] ; k0s0
    movhps          m2, [tmpq+offq+32*1]
    neg           offq
    movq            m3, [tmpq+offq+32*0] ; k0s1
    movhps          m3, [tmpq+offq+32*1]
%else
    mova            m1, [dstq]
    movu            m2, [tmpq+offq]
    neg           offq
    movu            m3, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+0]    ; off2_k0
    psubw           m2, m1               ; diff_k0s0
    psubw           m3, m1               ; diff_k0s1
    pabsw           m4, m2               ; adiff_k0s0
    psrlw           m5, m4, [sec_shift+gprsize]
    psubusw         m0, m6, m5
    pabsw           m5, m3               ; adiff_k0s1
    pminsw          m0, m4
    psrlw           m4, m5, [sec_shift+gprsize]
    psignw          m0, m2               ; constrain(diff_k0s0)
    psubusw         m2, m6, m4
    pminsw          m2, m5
%if %1 == 4
    movq            m4, [tmpq+offq+32*0] ; k0s2
    movhps          m4, [tmpq+offq+32*1]
    neg           offq
    movq            m5, [tmpq+offq+32*0] ; k0s3
    movhps          m5, [tmpq+offq+32*1]
%else
    movu            m4, [tmpq+offq]
    neg           offq
    movu            m5, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+9]    ; off1_k1
    psubw           m4, m1               ; diff_k0s2
    psubw           m5, m1               ; diff_k0s3
    psignw          m2, m3               ; constrain(diff_k0s1)
    pabsw           m3, m4               ; adiff_k0s2
    paddw           m0, m2
    psrlw           m2, m3, [sec_shift+gprsize]
    psubusw         m7, m6, m2
    pabsw           m2, m5               ; adiff_k0s3
    pminsw          m7, m3
    psrlw           m3, m2, [sec_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k0s2)
    psubusw         m4, m6, m3
    pminsw          m4, m2
%if %1 == 4
    movq            m2, [tmpq+offq+32*0] ; k1s0
    movhps          m2, [tmpq+offq+32*1]
    neg           offq
    movq            m3, [tmpq+offq+32*0] ; k1s1
    movhps          m3, [tmpq+offq+32*1]
%else
    movu            m2, [tmpq+offq]
    neg           offq
    movu            m3, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+1]    ; off2_k1
    paddw           m0, m7
    psignw          m4, m5               ; constrain(diff_k0s3)
    paddw           m0, m4               ; constrain(diff_k0)
    psubw           m2, m1               ; diff_k1s0
    psubw           m3, m1               ; diff_k1s1
    paddw           m0, m0               ; sec_tap_k0
    pabsw           m4, m2               ; adiff_k1s0
    psrlw           m5, m4, [sec_shift+gprsize]
    psubusw         m7, m6, m5
    pabsw           m5, m3               ; adiff_k1s1
    pminsw          m7, m4
    psrlw           m4, m5, [sec_shift+gprsize]
    psignw          m7, m2               ; constrain(diff_k1s0)
    psubusw         m2, m6, m4
    pminsw          m2, m5
%if %1 == 4
    movq            m4, [tmpq+offq+32*0] ; k1s2
    movhps          m4, [tmpq+offq+32*1]
    neg           offq
    movq            m5, [tmpq+offq+32*0] ; k1s3
    movhps          m5, [tmpq+offq+32*1]
%else
    movu            m4, [tmpq+offq]
    neg           offq
    movu            m5, [tmpq+offq]
%endif
    paddw           m0, m7
    psubw           m4, m1               ; diff_k1s2
    psubw           m5, m1               ; diff_k1s3
    psignw          m2, m3               ; constrain(diff_k1s1)
    pabsw           m3, m4               ; adiff_k1s2
    paddw           m0, m2
    psrlw           m2, m3, [sec_shift+gprsize]
    psubusw         m7, m6, m2
    pabsw           m2, m5               ; adiff_k1s3
    pminsw          m7, m3
    psrlw           m3, m2, [sec_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k1s2)
    psubusw         m4, m6, m3
    pminsw          m4, m2
    paddw           m0, m7
    psignw          m4, m5               ; constrain(diff_k1s3)
    paddw           m0, m4               ; sum
    psraw           m2, m0, 15
    paddw           m0, m2
    pmulhrsw        m0, m8
    paddw           m0, m1
%if %1 == 4
    add           tmpq, 32*2
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea           dstq, [dstq+strideq*2]
%else
    add           tmpq, 32
    mova        [dstq], m0
    add           dstq, strideq
%endif
    ret
ALIGN function_align
.pri_sec:
    movsx         offq, byte [dirq+8]    ; off2_k0
%if %1 == 4
    movq            m1, [dstq+strideq*0]
    movhps          m1, [dstq+strideq*1]
    movq            m2, [tmpq+offq+32*0] ; k0s0
    movhps          m2, [tmpq+offq+32*1]
    neg           offq
    movq            m3, [tmpq+offq+32*0] ; k0s1
    movhps          m3, [tmpq+offq+32*1]
%else
    mova            m1, [dstq]
    movu            m2, [tmpq+offq]
    neg           offq
    movu            m3, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+0]    ; off3_k0
    pabsw           m4, m2
%if ARCH_X86_64
    pabsw          m10, m3
    pmaxsw          m9, m2, m3
    pminsw         m10, m4
%else
    pabsw           m7, m3
    pmaxsw          m5, m2, m3
    pminsw          m4, m7
    mova            m9, m5
    mova           m10, m4
%endif
    psubw           m2, m1               ; diff_k0s0
    psubw           m3, m1               ; diff_k0s1
    pabsw           m4, m2               ; adiff_k0s0
    psrlw           m5, m4, [sec_shift+gprsize]
    psubusw         m0, m6, m5
    pabsw           m5, m3               ; adiff_k0s1
    pminsw          m0, m4
    psrlw           m4, m5, [sec_shift+gprsize]
    psignw          m0, m2               ; constrain(diff_k0s0)
    psubusw         m2, m6, m4
    pminsw          m2, m5
%if %1 == 4
    movq            m4, [tmpq+offq+32*0] ; k0s2
    movhps          m4, [tmpq+offq+32*1]
    neg           offq
    movq            m5, [tmpq+offq+32*0] ; k0s3
    movhps          m5, [tmpq+offq+32*1]
%else
    movu            m4, [tmpq+offq]
    neg           offq
    movu            m5, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+9]    ; off2_k1
    pabsw           m7, m4
    psignw          m2, m3
    pabsw           m3, m5               ; constrain(diff_k0s1)
%if ARCH_X86_64
    pmaxsw          m9, m4
    pminsw         m10, m7
    pmaxsw          m9, m5
    pminsw         m10, m3
%else
    pminsw          m7, m10
    pminsw          m7, m3
    pmaxsw          m3, m9, m4
    pmaxsw          m3, m5
    mova           m10, m7
    mova            m9, m3
%endif
    psubw           m4, m1               ; diff_k0s2
    psubw           m5, m1               ; diff_k0s3
    paddw           m0, m2
    pabsw           m3, m4               ; adiff_k0s2
    psrlw           m2, m3, [sec_shift+gprsize]
    psubusw         m7, m6, m2
    pabsw           m2, m5               ; adiff_k0s3
    pminsw          m7, m3
    psrlw           m3, m2, [sec_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k0s2)
    psubusw         m4, m6, m3
    pminsw          m4, m2
%if %1 == 4
    movq            m2, [tmpq+offq+32*0] ; k1s0
    movhps          m2, [tmpq+offq+32*1]
    neg           offq
    movq            m3, [tmpq+offq+32*0] ; k1s1
    movhps          m3, [tmpq+offq+32*1]
%else
    movu            m2, [tmpq+offq]
    neg           offq
    movu            m3, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+1]    ; off3_k1
    paddw           m0, m7
    pabsw           m7, m2
    psignw          m4, m5               ; constrain(diff_k0s3)
    pabsw           m5, m3
%if ARCH_X86_64
    pmaxsw          m9, m2
    pminsw         m10, m7
    pmaxsw          m9, m3
    pminsw         m10, m5
%else
    pminsw          m7, m10
    pminsw          m7, m5
    pmaxsw          m5, m9, m2
    pmaxsw          m5, m3
    mova           m10, m7
    mova            m9, m5
%endif
    paddw           m0, m4               ; constrain(diff_k0)
    psubw           m2, m1               ; diff_k1s0
    psubw           m3, m1               ; diff_k1s1
    paddw           m0, m0               ; sec_tap_k0
    pabsw           m4, m2               ; adiff_k1s0
    psrlw           m5, m4, [sec_shift+gprsize]
    psubusw         m7, m6, m5
    pabsw           m5, m3               ; adiff_k1s1
    pminsw          m7, m4
    psrlw           m4, m5, [sec_shift+gprsize]
    psignw          m7, m2               ; constrain(diff_k1s0)
    psubusw         m2, m6, m4
    pminsw          m2, m5
%if %1 == 4
    movq            m4, [tmpq+offq+32*0] ; k1s2
    movhps          m4, [tmpq+offq+32*1]
    neg           offq
    movq            m5, [tmpq+offq+32*0] ; k1s3
    movhps          m5, [tmpq+offq+32*1]
%else
    movu            m4, [tmpq+offq]
    neg           offq
    movu            m5, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+4]    ; off1_k0
    paddw           m0, m7
    pabsw           m7, m4
    psignw          m2, m3               ; constrain(diff_k1s1)
    pabsw           m3, m5
%if ARCH_X86_64
    pmaxsw          m9, m4
    pminsw         m10, m7
    pmaxsw          m9, m5
    pminsw         m10, m3
%else
    pminsw          m7, m10
    pminsw          m7, m3
    pmaxsw          m3, m9, m4
    pmaxsw          m3, m5
    mova           m10, m7
    mova            m9, m3
%endif
    psubw           m4, m1               ; diff_k1s2
    psubw           m5, m1               ; diff_k1s3
    pabsw           m3, m4               ; adiff_k1s2
    paddw           m0, m2
    psrlw           m2, m3, [sec_shift+gprsize]
    psubusw         m7, m6, m2
    pabsw           m2, m5               ; adiff_k1s3
    pminsw          m7, m3
    psrlw           m3, m2, [sec_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k1s2)
    psubusw         m4, m6, m3
    pminsw          m4, m2
    paddw           m0, m7
%if %1 == 4
    movq            m2, [tmpq+offq+32*0] ; k0p0
    movhps          m2, [tmpq+offq+32*1]
    neg           offq
    movq            m3, [tmpq+offq+32*0] ; k0p1
    movhps          m3, [tmpq+offq+32*1]
%else
    movu            m2, [tmpq+offq]
    neg           offq
    movu            m3, [tmpq+offq]
%endif
    movsx         offq, byte [dirq+5]    ; off1_k1
    pabsw           m7, m2
    psignw          m4, m5               ; constrain(diff_k1s3)
    pabsw           m5, m3
%if ARCH_X86_64
    pmaxsw          m9, m2
    pminsw         m10, m7
    pmaxsw          m9, m3
    pminsw         m10, m5
%else
    pminsw          m7, m10
    pminsw          m7, m5
    pmaxsw          m5, m9, m2
    pmaxsw          m5, m3
    mova           m10, m7
    mova            m9, m5
%endif
    psubw           m2, m1               ; diff_k0p0
    psubw           m3, m1               ; diff_k0p1
    paddw           m0, m4
    pabsw           m4, m2               ; adiff_k0p0
    psrlw           m5, m4, [pri_shift+gprsize]
    psubusw         m7, [rsp+gprsize], m5
    pabsw           m5, m3               ; adiff_k0p1
    pminsw          m7, m4
    psrlw           m4, m5, [pri_shift+gprsize]
    psignw          m7, m2               ; constrain(diff_k0p0)
    psubusw         m2, [rsp+gprsize], m4
    pminsw          m2, m5
%if %1 == 4
    movq            m4, [tmpq+offq+32*0] ; k1p0
    movhps          m4, [tmpq+offq+32*1]
    neg           offq
    movq            m5, [tmpq+offq+32*0] ; k1p1
    movhps          m5, [tmpq+offq+32*1]
%else
    movu            m4, [tmpq+offq]
    neg           offq
    movu            m5, [tmpq+offq]
%endif
    psignw          m2, m3               ; constrain(diff_k0p1)
    pabsw           m3, m4
    paddw           m7, m2               ; constrain(diff_k0)
    pabsw           m2, m5
%if ARCH_X86_64
    pmaxsw          m9, m4
    pminsw         m10, m3
    pmaxsw          m9, m5
    pminsw         m10, m2
%else
    pminsw          m3, m10
    pminsw          m3, m2
    pmaxsw          m2, m9, m4
    pmaxsw          m2, m5
    mova           m10, m3
    mova            m9, m2
%endif
    psubw           m4, m1               ; diff_k1p0
    psubw           m5, m1               ; diff_k1p1
    pabsw           m3, m4               ; adiff_k1p0
    pmullw          m7, [priq+16*0]      ; pri_tap_k0
    paddw           m0, m7
    psrlw           m2, m3, [pri_shift+gprsize]
    psubusw         m7, [rsp+16*0+gprsize], m2
    pabsw           m2, m5               ; adiff_k1p1
    pminsw          m7, m3
    psrlw           m3, m2, [pri_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k1p0)
    psubusw         m4, [rsp+16*0+gprsize], m3
    pminsw          m4, m2
    psignw          m4, m5               ; constrain(diff_k1p1)
    paddw           m7, m4               ; constrain(diff_k1)
    pmullw          m7, [priq+16*1]      ; pri_tap_k1
    paddw           m0, m7               ; sum
    psraw           m2, m0, 15
    paddw           m0, m2
    pmulhrsw        m0, m8
    paddw           m0, m1
%if ARCH_X86_64
    pmaxsw          m9, m1
    pminsw          m0, m9
%else
    pmaxsw          m2, m9, m1
    pminsw          m0, m2
%endif
    pminsw          m1, m10
    pmaxsw          m0, m1
%if %1 == 4
    add           tmpq, 32*2
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    lea           dstq, [dstq+strideq*2]
%else
    add           tmpq, 32
    mova        [dstq], m0
    add           dstq, strideq
%endif
    ret
%endif
%endmacro

INIT_XMM ssse3
%if ARCH_X86_64
cglobal cdef_filter_4x4_16bpc, 5, 9, 9, 32*10, dst, stride, left, top, bot, \
                                               pri, sec, edge
    %define         px  rsp+32*4
%else
cglobal cdef_filter_4x4_16bpc, 2, 7, 8, -32*11, dst, stride, edge, top, left
    %define       botq  topq
    %define         px  rsp+32*5
%endif
    %define       base  t0-dir_table
    %define  pri_shift  px-16*6
    %define  sec_shift  px-16*5
    mov          edged, r9m
    LEA             t0, dir_table
    movu            m0, [dstq+strideq*0]
    movu            m1, [dstq+strideq*1]
    lea             t1, [dstq+strideq*2]
    movu            m2, [t1  +strideq*0]
    movu            m3, [t1  +strideq*1]
    movddup         m7, [base+pw_m16384]
    mova   [px+32*0+0], m0
    mova   [px+32*1+0], m1
    mova   [px+32*2+0], m2
    mova   [px+32*3+0], m3
    test         edgeb, 4 ; HAVE_TOP
    jz .no_top
    movifnidn     topq, topmp
    movu            m0, [topq+strideq*0]
    movu            m1, [topq+strideq*1]
    mova   [px-32*2+0], m0
    mova   [px-32*1+0], m1
    test         edgeb, 1 ; HAVE_LEFT
    jz .top_no_left
    movd            m0, [topq+strideq*0-4]
    movd            m1, [topq+strideq*1-4]
    movd   [px-32*2-4], m0
    movd   [px-32*1-4], m1
    jmp .top_done
.no_top:
    mova   [px-32*2+0], m7
    mova   [px-32*1+0], m7
.top_no_left:
    movd   [px-32*2-4], m7
    movd   [px-32*1-4], m7
.top_done:
    test         edgeb, 8 ; HAVE_BOTTOM
    jz .no_bottom
    movifnidn     botq, r4mp
    movu            m0, [botq+strideq*0]
    movu            m1, [botq+strideq*1]
    mova   [px+32*4+0], m0
    mova   [px+32*5+0], m1
    test         edgeb, 1 ; HAVE_LEFT
    jz .bottom_no_left
    movd            m0, [botq+strideq*0-4]
    movd            m1, [botq+strideq*1-4]
    movd   [px+32*4-4], m0
    movd   [px+32*5-4], m1
    jmp .bottom_done
.no_bottom:
    mova   [px+32*4+0], m7
    mova   [px+32*5+0], m7
.bottom_no_left:
    movd   [px+32*4-4], m7
    movd   [px+32*5-4], m7
.bottom_done:
    test         edgeb, 1 ; HAVE_LEFT
    jz .no_left
    movifnidn    leftq, r2mp
    movd            m0, [leftq+4*0]
    movd            m1, [leftq+4*1]
    movd            m2, [leftq+4*2]
    movd            m3, [leftq+4*3]
    movd   [px+32*0-4], m0
    movd   [px+32*1-4], m1
    movd   [px+32*2-4], m2
    movd   [px+32*3-4], m3
    jmp .left_done
.no_left:
    REPX {movd [px+32*x-4], m7}, 0, 1, 2, 3
.left_done:
    test         edgeb, 2 ; HAVE_RIGHT
    jnz .padding_done
    REPX {movd [px+32*x+8], m7}, -2, -1, 0, 1, 2, 3, 4, 5
.padding_done:
    CDEF_FILTER      4, 4

%if ARCH_X86_64
cglobal cdef_filter_4x8_16bpc, 5, 9, 9, 32*14, dst, stride, left, top, bot, \
                                               pri, sec, edge
%else
cglobal cdef_filter_4x8_16bpc, 2, 7, 8, -32*15, dst, stride, edge, top, left
%endif
    mov          edged, r9m
    LEA             t0, dir_table
    movu            m0, [dstq+strideq*0]
    movu            m1, [dstq+strideq*1]
    lea             t1, [dstq+strideq*2]
    movu            m2, [t1  +strideq*0]
    movu            m3, [t1  +strideq*1]
    lea             t1, [t1  +strideq*2]
    movu            m4, [t1  +strideq*0]
    movu            m5, [t1  +strideq*1]
    lea             t1, [t1  +strideq*2]
    movu            m6, [t1  +strideq*0]
    movu            m7, [t1  +strideq*1]
    mova   [px+32*0+0], m0
    mova   [px+32*1+0], m1
    mova   [px+32*2+0], m2
    mova   [px+32*3+0], m3
    mova   [px+32*4+0], m4
    mova   [px+32*5+0], m5
    mova   [px+32*6+0], m6
    mova   [px+32*7+0], m7
    movddup         m7, [base+pw_m16384]
    test         edgeb, 4 ; HAVE_TOP
    jz .no_top
    movifnidn     topq, topmp
    movu            m0, [topq+strideq*0]
    movu            m1, [topq+strideq*1]
    mova   [px-32*2+0], m0
    mova   [px-32*1+0], m1
    test         edgeb, 1 ; HAVE_LEFT
    jz .top_no_left
    movd            m0, [topq+strideq*0-4]
    movd            m1, [topq+strideq*1-4]
    movd   [px-32*2-4], m0
    movd   [px-32*1-4], m1
    jmp .top_done
.no_top:
    mova   [px-32*2+0], m7
    mova   [px-32*1+0], m7
.top_no_left:
    movd   [px-32*2-4], m7
    movd   [px-32*1-4], m7
.top_done:
    test         edgeb, 8 ; HAVE_BOTTOM
    jz .no_bottom
    movifnidn     botq, r4mp
    movu            m0, [botq+strideq*0]
    movu            m1, [botq+strideq*1]
    mova   [px+32*8+0], m0
    mova   [px+32*9+0], m1
    test         edgeb, 1 ; HAVE_LEFT
    jz .bottom_no_left
    movd            m0, [botq+strideq*0-4]
    movd            m1, [botq+strideq*1-4]
    movd   [px+32*8-4], m0
    movd   [px+32*9-4], m1
    jmp .bottom_done
.no_bottom:
    mova   [px+32*8+0], m7
    mova   [px+32*9+0], m7
.bottom_no_left:
    movd   [px+32*8-4], m7
    movd   [px+32*9-4], m7
.bottom_done:
    test         edgeb, 1 ; HAVE_LEFT
    jz .no_left
    movifnidn    leftq, r2mp
    movd            m0, [leftq+4*0]
    movd            m1, [leftq+4*1]
    movd            m2, [leftq+4*2]
    movd            m3, [leftq+4*3]
    movd   [px+32*0-4], m0
    movd   [px+32*1-4], m1
    movd   [px+32*2-4], m2
    movd   [px+32*3-4], m3
    movd            m0, [leftq+4*4]
    movd            m1, [leftq+4*5]
    movd            m2, [leftq+4*6]
    movd            m3, [leftq+4*7]
    movd   [px+32*4-4], m0
    movd   [px+32*5-4], m1
    movd   [px+32*6-4], m2
    movd   [px+32*7-4], m3
    jmp .left_done
.no_left:
    REPX {movd [px+32*x-4], m7}, 0, 1, 2, 3, 4, 5, 6, 7
.left_done:
    test         edgeb, 2 ; HAVE_RIGHT
    jnz .padding_done
    REPX {movd [px+32*x+8], m7}, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
.padding_done:
    CDEF_FILTER      4, 8

%if ARCH_X86_64
cglobal cdef_filter_8x8_16bpc, 5, 9, 9, 32*14, dst, stride, left, top, bot, \
                                               pri, sec, edge
%else
cglobal cdef_filter_8x8_16bpc, 2, 7, 8, -32*15, dst, stride, edge, top, left
%endif
    mov          edged, r9m
    LEA             t0, dir_table
    mova            m0, [dstq+strideq*0+ 0]
    movd            m1, [dstq+strideq*0+16]
    mova            m2, [dstq+strideq*1+ 0]
    movd            m3, [dstq+strideq*1+16]
    lea             t1, [dstq+strideq*2]
    mova            m4, [t1  +strideq*0+ 0]
    movd            m5, [t1  +strideq*0+16]
    mova            m6, [t1  +strideq*1+ 0]
    movd            m7, [t1  +strideq*1+16]
    lea             t1, [t1  +strideq*2]
    mova  [px+32*0+ 0], m0
    movd  [px+32*0+16], m1
    mova  [px+32*1+ 0], m2
    movd  [px+32*1+16], m3
    mova  [px+32*2+ 0], m4
    movd  [px+32*2+16], m5
    mova  [px+32*3+ 0], m6
    movd  [px+32*3+16], m7
    mova            m0, [t1  +strideq*0+ 0]
    movd            m1, [t1  +strideq*0+16]
    mova            m2, [t1  +strideq*1+ 0]
    movd            m3, [t1  +strideq*1+16]
    lea             t1, [t1  +strideq*2]
    mova            m4, [t1  +strideq*0+ 0]
    movd            m5, [t1  +strideq*0+16]
    mova            m6, [t1  +strideq*1+ 0]
    movd            m7, [t1  +strideq*1+16]
    mova  [px+32*4+ 0], m0
    movd  [px+32*4+16], m1
    mova  [px+32*5+ 0], m2
    movd  [px+32*5+16], m3
    mova  [px+32*6+ 0], m4
    movd  [px+32*6+16], m5
    mova  [px+32*7+ 0], m6
    movd  [px+32*7+16], m7
    movddup         m7, [base+pw_m16384]
    test         edgeb, 4 ; HAVE_TOP
    jz .no_top
    movifnidn     topq, topmp
    mova            m0, [topq+strideq*0+ 0]
    mova            m1, [topq+strideq*0+16]
    mova            m2, [topq+strideq*1+ 0]
    mova            m3, [topq+strideq*1+16]
    mova  [px-32*2+ 0], m0
    movd  [px-32*2+16], m1
    mova  [px-32*1+ 0], m2
    movd  [px-32*1+16], m3
    test         edgeb, 1 ; HAVE_LEFT
    jz .top_no_left
    movd            m0, [topq+strideq*0-4]
    movd            m1, [topq+strideq*1-4]
    movd   [px-32*2-4], m0
    movd   [px-32*1-4], m1
    jmp .top_done
.no_top:
    mova  [px-32*2+ 0], m7
    movd  [px-32*2+16], m7
    mova  [px-32*1+ 0], m7
    movd  [px-32*1+16], m7
.top_no_left:
    movd  [px-32*2- 4], m7
    movd  [px-32*1- 4], m7
.top_done:
    test         edgeb, 8 ; HAVE_BOTTOM
    jz .no_bottom
    movifnidn     botq, r4mp
    mova            m0, [botq+strideq*0+ 0]
    movd            m1, [botq+strideq*0+16]
    mova            m2, [botq+strideq*1+ 0]
    movd            m3, [botq+strideq*1+16]
    mova  [px+32*8+ 0], m0
    movd  [px+32*8+16], m1
    mova  [px+32*9+ 0], m2
    movd  [px+32*9+16], m3
    test         edgeb, 1 ; HAVE_LEFT
    jz .bottom_no_left
    movd            m0, [botq+strideq*0-4]
    movd            m1, [botq+strideq*1-4]
    movd  [px+32*8- 4], m0
    movd  [px+32*9- 4], m1
    jmp .bottom_done
.no_bottom:
    mova  [px+32*8+ 0], m7
    movd  [px+32*8+16], m7
    mova  [px+32*9+ 0], m7
    movd  [px+32*9+16], m7
.bottom_no_left:
    movd  [px+32*8- 4], m7
    movd  [px+32*9- 4], m7
.bottom_done:
    test         edgeb, 1 ; HAVE_LEFT
    jz .no_left
    movifnidn    leftq, r2mp
    movd            m0, [leftq+4*0]
    movd            m1, [leftq+4*1]
    movd            m2, [leftq+4*2]
    movd            m3, [leftq+4*3]
    movd  [px+32*0- 4], m0
    movd  [px+32*1- 4], m1
    movd  [px+32*2- 4], m2
    movd  [px+32*3- 4], m3
    movd            m0, [leftq+4*4]
    movd            m1, [leftq+4*5]
    movd            m2, [leftq+4*6]
    movd            m3, [leftq+4*7]
    movd  [px+32*4- 4], m0
    movd  [px+32*5- 4], m1
    movd  [px+32*6- 4], m2
    movd  [px+32*7- 4], m3
    jmp .left_done
.no_left:
    REPX {movd [px+32*x- 4], m7}, 0, 1, 2, 3, 4, 5, 6, 7
.left_done:
    test         edgeb, 2 ; HAVE_RIGHT
    jnz .padding_done
    REPX {movd [px+32*x+16], m7}, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
.padding_done:
    CDEF_FILTER      8, 8

%macro CDEF_DIR 0
%if ARCH_X86_64
cglobal cdef_dir_16bpc, 4, 7, 16, src, stride, var, bdmax
    lea             r6, [dir_shift]
    shr         bdmaxd, 11 ; 0 for 10bpc, 1 for 12bpc
    movddup         m7, [r6+bdmaxq*8]
    lea             r6, [strideq*3]
    mova            m0, [srcq+strideq*0]
    mova            m1, [srcq+strideq*1]
    mova            m2, [srcq+strideq*2]
    mova            m3, [srcq+r6       ]
    lea           srcq, [srcq+strideq*4]
    mova            m4, [srcq+strideq*0]
    mova            m5, [srcq+strideq*1]
    mova            m6, [srcq+strideq*2]
    REPX {pmulhuw x, m7}, m0, m1, m2, m3, m4, m5, m6
    pmulhuw         m7, [srcq+r6       ]
    pxor            m8, m8
    packuswb        m9, m0, m1
    packuswb       m10, m2, m3
    packuswb       m11, m4, m5
    packuswb       m12, m6, m7
    REPX {psadbw x, m8}, m9, m10, m11, m12
    packssdw        m9, m10
    packssdw       m11, m12
    packssdw        m9, m11
    jmp mangle(private_prefix %+ _cdef_dir_8bpc %+ SUFFIX).main
%else
cglobal cdef_dir_16bpc, 2, 4, 8, 96, src, stride, var, bdmax
    mov         bdmaxd, bdmaxm
    LEA             r2, dir_shift
    shr         bdmaxd, 11
    movddup         m7, [r2+bdmaxq*8]
    lea             r3, [strideq*3]
    pmulhuw         m3, m7, [srcq+strideq*0]
    pmulhuw         m4, m7, [srcq+strideq*1]
    pmulhuw         m5, m7, [srcq+strideq*2]
    pmulhuw         m6, m7, [srcq+r3       ]
    movddup         m1, [r2-dir_shift+pw_128]
    lea           srcq, [srcq+strideq*4]
    pxor            m0, m0
    packuswb        m2, m3, m4
    psubw           m3, m1
    psubw           m4, m1
    mova    [esp+0x00], m3
    mova    [esp+0x10], m4
    packuswb        m3, m5, m6
    psadbw          m2, m0
    psadbw          m3, m0
    psubw           m5, m1
    psubw           m6, m1
    packssdw        m2, m3
    mova    [esp+0x20], m5
    mova    [esp+0x50], m6
    pmulhuw         m4, m7, [srcq+strideq*0]
    pmulhuw         m5, m7, [srcq+strideq*1]
    pmulhuw         m6, m7, [srcq+strideq*2]
    pmulhuw         m7,     [srcq+r3       ]
    packuswb        m3, m4, m5
    packuswb        m1, m6, m7
    psadbw          m3, m0
    psadbw          m1, m0
    packssdw        m3, m1
    movddup         m1, [r2-dir_shift+pw_128]
    LEA             r2, shufw_6543210x
    jmp mangle(private_prefix %+ _cdef_dir_8bpc %+ SUFFIX).main
%endif
%endmacro

INIT_XMM ssse3
CDEF_DIR

INIT_XMM sse4
CDEF_DIR
