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

SECTION_RODATA

%macro DIR_TABLE 1 ; stride
    db  1 * %1 + 0,  2 * %1 + 0
    db  1 * %1 + 0,  2 * %1 - 2
    db -1 * %1 + 2, -2 * %1 + 4
    db  0 * %1 + 2, -1 * %1 + 4
    db  0 * %1 + 2,  0 * %1 + 4
    db  0 * %1 + 2,  1 * %1 + 4
    db  1 * %1 + 2,  2 * %1 + 4
    db  1 * %1 + 0,  2 * %1 + 2
    db  1 * %1 + 0,  2 * %1 + 0
    db  1 * %1 + 0,  2 * %1 - 2
    db -1 * %1 + 2, -2 * %1 + 4
    db  0 * %1 + 2, -1 * %1 + 4
%endmacro

dir_table4: DIR_TABLE 16
dir_table8: DIR_TABLE 32
pri_taps:   dw  4, 4, 3, 3, 2, 2, 3, 3

dir_shift:  times 2 dw 0x4000
            times 2 dw 0x1000

pw_2048:    times 2 dw 2048
pw_m16384:  times 2 dw -16384

cextern cdef_dir_8bpc_avx2.main

SECTION .text

%macro CDEF_FILTER 2 ; w, h
    DEFINE_ARGS dst, stride, _, dir, pridmp, pri, sec, tmp
    movifnidn     prid, r5m
    movifnidn     secd, r6m
    mov           dird, r7m
    vpbroadcastd    m8, [base+pw_2048]
    lea           dirq, [base+dir_table%1+dirq*2]
    test          prid, prid
    jz .sec_only
%if WIN64
    vpbroadcastw    m6, prim
    movaps  [rsp+16*0], xmm9
    movaps  [rsp+16*1], xmm10
%else
    movd           xm6, prid
    vpbroadcastw    m6, xm6
%endif
    lzcnt      pridmpd, prid
    rorx          tmpd, prid, 2
    cmp     dword r10m, 0xfff ; if (bpc == 12)
    cmove         prid, tmpd  ;     pri >>= 2
    mov           tmpd, r8m   ; damping
    and           prid, 4
    sub           tmpd, 31
    vpbroadcastd    m9, [base+pri_taps+priq+8*0]
    vpbroadcastd   m10, [base+pri_taps+priq+8*1]
    test          secd, secd
    jz .pri_only
%if WIN64
    movaps         r8m, xmm13
    vpbroadcastw   m13, secm
    movaps         r4m, xmm11
    movaps         r6m, xmm12
%else
    movd           xm0, secd
    vpbroadcastw   m13, xm0
%endif
    lzcnt         secd, secd
    xor           prid, prid
    add        pridmpd, tmpd
    cmovs      pridmpd, prid
    add           secd, tmpd
    lea           tmpq, [px]
    mov    [pri_shift], pridmpq
    mov    [sec_shift], secq
%rep %1*%2/16
    call mangle(private_prefix %+ _cdef_filter_%1x%1_16bpc %+ SUFFIX).pri_sec
%endrep
%if WIN64
    movaps       xmm11, r4m
    movaps       xmm12, r6m
    movaps       xmm13, r8m
%endif
    jmp .pri_end
.pri_only:
    add        pridmpd, tmpd
    cmovs      pridmpd, secd
    lea           tmpq, [px]
    mov    [pri_shift], pridmpq
%rep %1*%2/16
    call mangle(private_prefix %+ _cdef_filter_%1x%1_16bpc %+ SUFFIX).pri
%endrep
.pri_end:
%if WIN64
    movaps        xmm9, [rsp+16*0]
    movaps       xmm10, [rsp+16*1]
%endif
.end:
    RET
.sec_only:
    mov           tmpd, r8m ; damping
%if WIN64
    vpbroadcastw    m6, secm
%else
    movd           xm6, secd
    vpbroadcastw    m6, xm6
%endif
    tzcnt         secd, secd
    sub           tmpd, secd
    mov    [sec_shift], tmpq
    lea           tmpq, [px]
%rep %1*%2/16
    call mangle(private_prefix %+ _cdef_filter_%1x%1_16bpc %+ SUFFIX).sec
%endrep
    jmp .end
%if %1 == %2
ALIGN function_align
.pri:
    movsx         offq, byte [dirq+4]    ; off_k0
%if %1 == 4
    mova            m1, [tmpq+32*0]
    punpcklqdq      m1, [tmpq+32*1]      ; 0 2 1 3
    movu            m2, [tmpq+offq+32*0]
    punpcklqdq      m2, [tmpq+offq+32*1] ; k0p0
    neg           offq
    movu            m3, [tmpq+offq+32*0]
    punpcklqdq      m3, [tmpq+offq+32*1] ; k0p1
%else
    mova           xm1, [tmpq+32*0]
    vinserti128     m1, [tmpq+32*1], 1
    movu           xm2, [tmpq+offq+32*0]
    vinserti128     m2, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm3, [tmpq+offq+32*0]
    vinserti128     m3, [tmpq+offq+32*1], 1
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
    movu            m4, [tmpq+offq+32*0]
    punpcklqdq      m4, [tmpq+offq+32*1] ; k1p0
    neg           offq
    movu            m5, [tmpq+offq+32*0]
    punpcklqdq      m5, [tmpq+offq+32*1] ; k1p1
%else
    movu           xm4, [tmpq+offq+32*0]
    vinserti128     m4, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm5, [tmpq+offq+32*0]
    vinserti128     m5, [tmpq+offq+32*1], 1
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
    pmullw          m0, m9               ; pri_tap_k0
    pmullw          m7, m10              ; pri_tap_k1
    paddw           m0, m7               ; sum
    psraw           m2, m0, 15
    paddw           m0, m2
    pmulhrsw        m0, m8
    add           tmpq, 32*2
    paddw           m0, m1
%if %1 == 4
    vextracti128   xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r9       ], xm1
    lea           dstq, [dstq+strideq*4]
%else
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea           dstq, [dstq+strideq*2]
%endif
    ret
ALIGN function_align
.sec:
    movsx         offq, byte [dirq+8]    ; off1_k0
%if %1 == 4
    mova            m1, [tmpq+32*0]
    punpcklqdq      m1, [tmpq+32*1]
    movu            m2, [tmpq+offq+32*0]
    punpcklqdq      m2, [tmpq+offq+32*1] ; k0s0
    neg           offq
    movu            m3, [tmpq+offq+32*0]
    punpcklqdq      m3, [tmpq+offq+32*1] ; k0s1
%else
    mova           xm1, [tmpq+32*0]
    vinserti128     m1, [tmpq+32*1], 1
    movu           xm2, [tmpq+offq+32*0]
    vinserti128     m2, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm3, [tmpq+offq+32*0]
    vinserti128     m3, [tmpq+offq+32*1], 1
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
    movu            m4, [tmpq+offq+32*0]
    punpcklqdq      m4, [tmpq+offq+32*1] ; k0s2
    neg           offq
    movu            m5, [tmpq+offq+32*0]
    punpcklqdq      m5, [tmpq+offq+32*1] ; k0s3
%else
    movu           xm4, [tmpq+offq+32*0]
    vinserti128     m4, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm5, [tmpq+offq+32*0]
    vinserti128     m5, [tmpq+offq+32*1], 1
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
    movu            m2, [tmpq+offq+32*0]
    punpcklqdq      m2, [tmpq+offq+32*1] ; k1s0
    neg           offq
    movu            m3, [tmpq+offq+32*0]
    punpcklqdq      m3, [tmpq+offq+32*1] ; k1s1
%else
    movu           xm2, [tmpq+offq+32*0]
    vinserti128     m2, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm3, [tmpq+offq+32*0]
    vinserti128     m3, [tmpq+offq+32*1], 1
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
    movu            m4, [tmpq+offq+32*0]
    punpcklqdq      m4, [tmpq+offq+32*1] ; k1s2
    neg           offq
    movu            m5, [tmpq+offq+32*0]
    punpcklqdq      m5, [tmpq+offq+32*1] ; k1s3
%else
    movu           xm4, [tmpq+offq+32*0]
    vinserti128     m4, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm5, [tmpq+offq+32*0]
    vinserti128     m5, [tmpq+offq+32*1], 1
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
    add           tmpq, 32*2
    paddw           m0, m1
%if %1 == 4
    vextracti128   xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r9       ], xm1
    lea           dstq, [dstq+strideq*4]
%else
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea           dstq, [dstq+strideq*2]
%endif
    ret
ALIGN function_align
.pri_sec:
    movsx         offq, byte [dirq+8]    ; off2_k0
%if %1 == 4
    mova            m1, [tmpq+32*0]
    punpcklqdq      m1, [tmpq+32*1]
    movu            m2, [tmpq+offq+32*0]
    punpcklqdq      m2, [tmpq+offq+32*1] ; k0s0
    neg           offq
    movu            m3, [tmpq+offq+32*0]
    punpcklqdq      m3, [tmpq+offq+32*1] ; k0s1
%else
    mova           xm1, [dstq+strideq*0]
    vinserti128     m1, [dstq+strideq*1], 1
    movu           xm2, [tmpq+offq+32*0]
    vinserti128     m2, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm3, [tmpq+offq+32*0]
    vinserti128     m3, [tmpq+offq+32*1], 1
%endif
    movsx         offq, byte [dirq+0]    ; off3_k0
    pmaxsw         m11, m2, m3
    pminuw         m12, m2, m3
    psubw           m2, m1               ; diff_k0s0
    psubw           m3, m1               ; diff_k0s1
    pabsw           m4, m2               ; adiff_k0s0
    psrlw           m5, m4, [sec_shift+gprsize]
    psubusw         m0, m13, m5
    pabsw           m5, m3               ; adiff_k0s1
    pminsw          m0, m4
    psrlw           m4, m5, [sec_shift+gprsize]
    psignw          m0, m2               ; constrain(diff_k0s0)
    psubusw         m2, m13, m4
    pminsw          m2, m5
%if %1 == 4
    movu            m4, [tmpq+offq+32*0]
    punpcklqdq      m4, [tmpq+offq+32*1] ; k0s2
    neg           offq
    movu            m5, [tmpq+offq+32*0]
    punpcklqdq      m5, [tmpq+offq+32*1] ; k0s3
%else
    movu           xm4, [tmpq+offq+32*0]
    vinserti128     m4, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm5, [tmpq+offq+32*0]
    vinserti128     m5, [tmpq+offq+32*1], 1
%endif
    movsx         offq, byte [dirq+9]    ; off2_k1
    psignw          m2, m3               ; constrain(diff_k0s1)
    pmaxsw         m11, m4
    pminuw         m12, m4
    pmaxsw         m11, m5
    pminuw         m12, m5
    psubw           m4, m1               ; diff_k0s2
    psubw           m5, m1               ; diff_k0s3
    paddw           m0, m2
    pabsw           m3, m4               ; adiff_k0s2
    psrlw           m2, m3, [sec_shift+gprsize]
    psubusw         m7, m13, m2
    pabsw           m2, m5               ; adiff_k0s3
    pminsw          m7, m3
    psrlw           m3, m2, [sec_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k0s2)
    psubusw         m4, m13, m3
    pminsw          m4, m2
%if %1 == 4
    movu            m2, [tmpq+offq+32*0]
    punpcklqdq      m2, [tmpq+offq+32*1] ; k1s0
    neg           offq
    movu            m3, [tmpq+offq+32*0]
    punpcklqdq      m3, [tmpq+offq+32*1] ; k1s1
%else
    movu           xm2, [tmpq+offq+32*0]
    vinserti128     m2, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm3, [tmpq+offq+32*0]
    vinserti128     m3, [tmpq+offq+32*1], 1
%endif
    movsx         offq, byte [dirq+1]    ; off3_k1
    paddw           m0, m7
    psignw          m4, m5               ; constrain(diff_k0s3)
    pmaxsw         m11, m2
    pminuw         m12, m2
    pmaxsw         m11, m3
    pminuw         m12, m3
    paddw           m0, m4               ; constrain(diff_k0)
    psubw           m2, m1               ; diff_k1s0
    psubw           m3, m1               ; diff_k1s1
    paddw           m0, m0               ; sec_tap_k0
    pabsw           m4, m2               ; adiff_k1s0
    psrlw           m5, m4, [sec_shift+gprsize]
    psubusw         m7, m13, m5
    pabsw           m5, m3               ; adiff_k1s1
    pminsw          m7, m4
    psrlw           m4, m5, [sec_shift+gprsize]
    psignw          m7, m2               ; constrain(diff_k1s0)
    psubusw         m2, m13, m4
    pminsw          m2, m5
%if %1 == 4
    movu            m4, [tmpq+offq+32*0]
    punpcklqdq      m4, [tmpq+offq+32*1] ; k1s2
    neg           offq
    movu            m5, [tmpq+offq+32*0]
    punpcklqdq      m5, [tmpq+offq+32*1] ; k1s3
%else
    movu           xm4, [tmpq+offq+32*0]
    vinserti128     m4, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm5, [tmpq+offq+32*0]
    vinserti128     m5, [tmpq+offq+32*1], 1
%endif
    movsx         offq, byte [dirq+4]    ; off1_k0
    paddw           m0, m7
    psignw          m2, m3               ; constrain(diff_k1s1)
    pmaxsw         m11, m4
    pminuw         m12, m4
    pmaxsw         m11, m5
    pminuw         m12, m5
    psubw           m4, m1               ; diff_k1s2
    psubw           m5, m1               ; diff_k1s3
    pabsw           m3, m4               ; adiff_k1s2
    paddw           m0, m2
    psrlw           m2, m3, [sec_shift+gprsize]
    psubusw         m7, m13, m2
    pabsw           m2, m5               ; adiff_k1s3
    pminsw          m7, m3
    psrlw           m3, m2, [sec_shift+gprsize]
    psignw          m7, m4               ; constrain(diff_k1s2)
    psubusw         m4, m13, m3
    pminsw          m4, m2
    paddw           m0, m7
%if %1 == 4
    movu            m2, [tmpq+offq+32*0]
    punpcklqdq      m2, [tmpq+offq+32*1] ; k0p0
    neg           offq
    movu            m3, [tmpq+offq+32*0]
    punpcklqdq      m3, [tmpq+offq+32*1] ; k0p1
%else
    movu           xm2, [tmpq+offq+32*0]
    vinserti128     m2, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm3, [tmpq+offq+32*0]
    vinserti128     m3, [tmpq+offq+32*1], 1
%endif
    movsx         offq, byte [dirq+5]    ; off1_k1
    psignw          m4, m5               ; constrain(diff_k1s3)
    pmaxsw         m11, m2
    pminuw         m12, m2
    pmaxsw         m11, m3
    pminuw         m12, m3
    psubw           m2, m1               ; diff_k0p0
    psubw           m3, m1               ; diff_k0p1
    paddw           m0, m4
    pabsw           m4, m2               ; adiff_k0p0
    psrlw           m5, m4, [pri_shift+gprsize]
    psubusw         m7, m6, m5
    pabsw           m5, m3               ; adiff_k0p1
    pminsw          m7, m4
    psrlw           m4, m5, [pri_shift+gprsize]
    psignw          m7, m2               ; constrain(diff_k0p0)
    psubusw         m2, m6, m4
    pminsw          m2, m5
%if %1 == 4
    movu            m4, [tmpq+offq+32*0]
    punpcklqdq      m4, [tmpq+offq+32*1] ; k1p0
    neg           offq
    movu            m5, [tmpq+offq+32*0]
    punpcklqdq      m5, [tmpq+offq+32*1] ; k1p1
%else
    movu           xm4, [tmpq+offq+32*0]
    vinserti128     m4, [tmpq+offq+32*1], 1
    neg           offq
    movu           xm5, [tmpq+offq+32*0]
    vinserti128     m5, [tmpq+offq+32*1], 1
%endif
    psignw          m2, m3               ; constrain(diff_k0p1)
    paddw           m7, m2               ; constrain(diff_k0)
    pmaxsw         m11, m4
    pminuw         m12, m4
    pmaxsw         m11, m5
    pminuw         m12, m5
    psubw           m4, m1               ; diff_k1p0
    psubw           m5, m1               ; diff_k1p1
    pabsw           m3, m4               ; adiff_k1p0
    pmullw          m7, m9               ; pri_tap_k0
    paddw           m0, m7
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
    pmullw          m7, m10              ; pri_tap_k1
    paddw           m0, m7               ; sum
    psraw           m2, m0, 15
    paddw           m0, m2
    pmulhrsw        m0, m8
    add           tmpq, 32*2
    pmaxsw         m11, m1
    pminuw         m12, m1
    paddw           m0, m1
    pminsw          m0, m11
    pmaxsw          m0, m12
%if %1 == 4
    vextracti128   xm1, m0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+r9       ], xm1
    lea           dstq, [dstq+strideq*4]
%else
    mova         [dstq+strideq*0], xm0
    vextracti128 [dstq+strideq*1], m0, 1
    lea           dstq, [dstq+strideq*2]
%endif
    ret
%endif
%endmacro

INIT_YMM avx2
cglobal cdef_filter_4x4_16bpc, 5, 10, 9, 16*10, dst, stride, left, top, bot, \
                                                pri, sec, edge
%if WIN64
    %define         px  rsp+16*6
    %define       offq  r8
    %define  pri_shift  rsp+16*2
    %define  sec_shift  rsp+16*3
%else
    %define         px  rsp+16*4
    %define       offq  r4
    %define  pri_shift  rsp+16*0
    %define  sec_shift  rsp+16*1
%endif
    %define       base  r8-dir_table4
    mov          edged, r9m
    lea             r8, [dir_table4]
    movu           xm0, [dstq+strideq*0]
    movu           xm1, [dstq+strideq*1]
    lea             r9, [strideq*3]
    movu           xm2, [dstq+strideq*2]
    movu           xm3, [dstq+r9       ]
    vpbroadcastd    m7, [base+pw_m16384]
    mova   [px+16*0+0], xm0
    mova   [px+16*1+0], xm1
    mova   [px+16*2+0], xm2
    mova   [px+16*3+0], xm3
    test         edgeb, 4 ; HAVE_TOP
    jz .no_top
    movu           xm0, [topq+strideq*0]
    movu           xm1, [topq+strideq*1]
    mova   [px-16*2+0], xm0
    mova   [px-16*1+0], xm1
    test         edgeb, 1 ; HAVE_LEFT
    jz .top_no_left
    movd           xm0, [topq+strideq*0-4]
    movd           xm1, [topq+strideq*1-4]
    movd   [px-16*2-4], xm0
    movd   [px-16*1-4], xm1
    jmp .top_done
.no_top:
    mova   [px-16*2+0], m7
.top_no_left:
    movd   [px-16*2-4], xm7
    movd   [px-16*1-4], xm7
.top_done:
    test         edgeb, 8 ; HAVE_BOTTOM
    jz .no_bottom
    movu           xm0, [botq+strideq*0]
    movu           xm1, [botq+strideq*1]
    mova   [px+16*4+0], xm0
    mova   [px+16*5+0], xm1
    test         edgeb, 1 ; HAVE_LEFT
    jz .bottom_no_left
    movd           xm0, [botq+strideq*0-4]
    movd           xm1, [botq+strideq*1-4]
    movd   [px+16*4-4], xm0
    movd   [px+16*5-4], xm1
    jmp .bottom_done
.no_bottom:
    mova   [px+16*4+0], m7
.bottom_no_left:
    movd   [px+16*4-4], xm7
    movd   [px+16*5-4], xm7
.bottom_done:
    test         edgeb, 1 ; HAVE_LEFT
    jz .no_left
    movd           xm0, [leftq+4*0]
    movd           xm1, [leftq+4*1]
    movd           xm2, [leftq+4*2]
    movd           xm3, [leftq+4*3]
    movd   [px+16*0-4], xm0
    movd   [px+16*1-4], xm1
    movd   [px+16*2-4], xm2
    movd   [px+16*3-4], xm3
    jmp .left_done
.no_left:
    REPX {movd [px+16*x-4], xm7}, 0, 1, 2, 3
.left_done:
    test         edgeb, 2 ; HAVE_RIGHT
    jnz .padding_done
    REPX {movd [px+16*x+8], xm7}, -2, -1, 0, 1, 2, 3, 4, 5
.padding_done:
    CDEF_FILTER      4, 4

cglobal cdef_filter_4x8_16bpc, 5, 10, 9, 16*14, dst, stride, left, top, bot, \
                                                pri, sec, edge
    mov          edged, r9m
    movu           xm0, [dstq+strideq*0]
    movu           xm1, [dstq+strideq*1]
    lea             r9, [strideq*3]
    movu           xm2, [dstq+strideq*2]
    movu           xm3, [dstq+r9       ]
    lea             r6, [dstq+strideq*4]
    movu           xm4, [r6  +strideq*0]
    movu           xm5, [r6  +strideq*1]
    movu           xm6, [r6  +strideq*2]
    movu           xm7, [r6  +r9       ]
    lea             r8, [dir_table4]
    mova   [px+16*0+0], xm0
    mova   [px+16*1+0], xm1
    mova   [px+16*2+0], xm2
    mova   [px+16*3+0], xm3
    mova   [px+16*4+0], xm4
    mova   [px+16*5+0], xm5
    mova   [px+16*6+0], xm6
    mova   [px+16*7+0], xm7
    vpbroadcastd    m7, [base+pw_m16384]
    test         edgeb, 4 ; HAVE_TOP
    jz .no_top
    movu           xm0, [topq+strideq*0]
    movu           xm1, [topq+strideq*1]
    mova   [px-16*2+0], xm0
    mova   [px-16*1+0], xm1
    test         edgeb, 1 ; HAVE_LEFT
    jz .top_no_left
    movd           xm0, [topq+strideq*0-4]
    movd           xm1, [topq+strideq*1-4]
    movd   [px-16*2-4], xm0
    movd   [px-16*1-4], xm1
    jmp .top_done
.no_top:
    mova   [px-16*2+0], m7
.top_no_left:
    movd   [px-16*2-4], xm7
    movd   [px-16*1-4], xm7
.top_done:
    test         edgeb, 8 ; HAVE_BOTTOM
    jz .no_bottom
    movu           xm0, [botq+strideq*0]
    movu           xm1, [botq+strideq*1]
    mova   [px+16*8+0], xm0
    mova   [px+16*9+0], xm1
    test         edgeb, 1 ; HAVE_LEFT
    jz .bottom_no_left
    movd           xm0, [botq+strideq*0-4]
    movd           xm1, [botq+strideq*1-4]
    movd   [px+16*8-4], xm0
    movd   [px+16*9-4], xm1
    jmp .bottom_done
.no_bottom:
    mova   [px+16*8+0], m7
.bottom_no_left:
    movd   [px+16*8-4], xm7
    movd   [px+16*9-4], xm7
.bottom_done:
    test         edgeb, 1 ; HAVE_LEFT
    jz .no_left
    movd           xm0, [leftq+4*0]
    movd           xm1, [leftq+4*1]
    movd           xm2, [leftq+4*2]
    movd           xm3, [leftq+4*3]
    movd   [px+16*0-4], xm0
    movd   [px+16*1-4], xm1
    movd   [px+16*2-4], xm2
    movd   [px+16*3-4], xm3
    movd           xm0, [leftq+4*4]
    movd           xm1, [leftq+4*5]
    movd           xm2, [leftq+4*6]
    movd           xm3, [leftq+4*7]
    movd   [px+16*4-4], xm0
    movd   [px+16*5-4], xm1
    movd   [px+16*6-4], xm2
    movd   [px+16*7-4], xm3
    jmp .left_done
.no_left:
    REPX {movd [px+16*x-4], xm7}, 0, 1, 2, 3, 4, 5, 6, 7
.left_done:
    test         edgeb, 2 ; HAVE_RIGHT
    jnz .padding_done
    REPX {movd [px+16*x+8], xm7}, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
.padding_done:
    CDEF_FILTER      4, 8

cglobal cdef_filter_8x8_16bpc, 5, 9, 9, 32*13, dst, stride, left, top, bot, \
                                               pri, sec, edge
%if WIN64
    %define         px  rsp+32*4
%else
    %define         px  rsp+32*3
%endif
    %define       base  r8-dir_table8
    mov          edged, r9m
    movu            m0, [dstq+strideq*0]
    movu            m1, [dstq+strideq*1]
    lea             r6, [dstq+strideq*2]
    movu            m2, [r6  +strideq*0]
    movu            m3, [r6  +strideq*1]
    lea             r6, [r6  +strideq*2]
    movu            m4, [r6  +strideq*0]
    movu            m5, [r6  +strideq*1]
    lea             r6, [r6  +strideq*2]
    movu            m6, [r6  +strideq*0]
    movu            m7, [r6  +strideq*1]
    lea             r8, [dir_table8]
    mova   [px+32*0+0], m0
    mova   [px+32*1+0], m1
    mova   [px+32*2+0], m2
    mova   [px+32*3+0], m3
    mova   [px+32*4+0], m4
    mova   [px+32*5+0], m5
    mova   [px+32*6+0], m6
    mova   [px+32*7+0], m7
    vpbroadcastd    m7, [base+pw_m16384]
    test         edgeb, 4 ; HAVE_TOP
    jz .no_top
    movu            m0, [topq+strideq*0]
    movu            m1, [topq+strideq*1]
    mova   [px-32*2+0], m0
    mova   [px-32*1+0], m1
    test         edgeb, 1 ; HAVE_LEFT
    jz .top_no_left
    movd           xm0, [topq+strideq*0-4]
    movd           xm1, [topq+strideq*1-4]
    movd   [px-32*2-4], xm0
    movd   [px-32*1-4], xm1
    jmp .top_done
.no_top:
    mova   [px-32*2+0], m7
    mova   [px-32*1+0], m7
.top_no_left:
    movd   [px-32*2-4], xm7
    movd   [px-32*1-4], xm7
.top_done:
    test         edgeb, 8 ; HAVE_BOTTOM
    jz .no_bottom
    movu            m0, [botq+strideq*0]
    movu            m1, [botq+strideq*1]
    mova   [px+32*8+0], m0
    mova   [px+32*9+0], m1
    test         edgeb, 1 ; HAVE_LEFT
    jz .bottom_no_left
    movd           xm0, [botq+strideq*0-4]
    movd           xm1, [botq+strideq*1-4]
    movd   [px+32*8-4], xm0
    movd   [px+32*9-4], xm1
    jmp .bottom_done
.no_bottom:
    mova   [px+32*8+0], m7
    mova   [px+32*9+0], m7
.bottom_no_left:
    movd   [px+32*8-4], xm7
    movd   [px+32*9-4], xm7
.bottom_done:
    test         edgeb, 1 ; HAVE_LEFT
    jz .no_left
    movd           xm0, [leftq+4*0]
    movd           xm1, [leftq+4*1]
    movd           xm2, [leftq+4*2]
    movd           xm3, [leftq+4*3]
    movd   [px+32*0-4], xm0
    movd   [px+32*1-4], xm1
    movd   [px+32*2-4], xm2
    movd   [px+32*3-4], xm3
    movd           xm0, [leftq+4*4]
    movd           xm1, [leftq+4*5]
    movd           xm2, [leftq+4*6]
    movd           xm3, [leftq+4*7]
    movd   [px+32*4-4], xm0
    movd   [px+32*5-4], xm1
    movd   [px+32*6-4], xm2
    movd   [px+32*7-4], xm3
    jmp .left_done
.no_left:
    REPX {movd [px+32*x-4], xm7}, 0, 1, 2, 3, 4, 5, 6, 7
.left_done:
    test         edgeb, 2 ; HAVE_RIGHT
    jnz .padding_done
    REPX {movd [px+32*x+16], xm7}, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
.padding_done:
    CDEF_FILTER      8, 8

cglobal cdef_dir_16bpc, 4, 7, 6, src, stride, var, bdmax
    lea             r6, [dir_shift]
    shr         bdmaxd, 11 ; 0 for 10bpc, 1 for 12bpc
    vpbroadcastd    m4, [r6+bdmaxq*4]
    lea             r6, [strideq*3]
    mova           xm0, [srcq+strideq*0]
    mova           xm1, [srcq+strideq*1]
    mova           xm2, [srcq+strideq*2]
    mova           xm3, [srcq+r6       ]
    lea           srcq, [srcq+strideq*4]
    vinserti128     m0, [srcq+r6       ], 1
    vinserti128     m1, [srcq+strideq*2], 1
    vinserti128     m2, [srcq+strideq*1], 1
    vinserti128     m3, [srcq+strideq*0], 1
    REPX {pmulhuw x, m4}, m0, m1, m2, m3
    jmp mangle(private_prefix %+ _cdef_dir_8bpc %+ SUFFIX).main

%endif ; ARCH_X86_64
