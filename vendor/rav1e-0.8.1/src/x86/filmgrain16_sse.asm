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
%include "x86/filmgrain_common.asm"

SECTION_RODATA 16
pd_16: times 4 dd 16
pw_1: times 8 dw 1
pw_16384: times 8 dw 16384
pw_8192: times 8 dw 8192
pw_23_22: dw 23, 22
          times 3 dw 0, 32
pb_mask: db 0, 0x80, 0x80, 0, 0x80, 0, 0, 0x80, 0x80, 0, 0, 0x80, 0, 0x80, 0x80, 0
pw_27_17_17_27: dw 27, 17, 17, 27
                times 2 dw 0, 32
rnd_next_upperbit_mask: dw 0x100B, 0x2016, 0x402C, 0x8058
pw_seed_xor: times 2 dw 0xb524
             times 2 dw 0x49d8
pb_1: times 4 db 1
hmul_bits: dw 32768, 16384, 8192, 4096
round: dw 2048, 1024, 512
mul_bits: dw 256, 128, 64, 32, 16
round_vals: dw 32, 64, 128, 256, 512, 1024
max: dw 256*4-1, 240*4, 235*4, 256*16-1, 240*16, 235*16
min: dw 0, 16*4, 16*16
; these two should be next to each other
pw_4: times 2 dw 4
pw_16: times 2 dw 16

%macro JMP_TABLE 1-*
    %xdefine %1_table %%table
    %xdefine %%base %1_table
    %xdefine %%prefix mangle(private_prefix %+ _%1)
    %%table:
    %rep %0 - 1
        dd %%prefix %+ .ar%2 - %%base
        %rotate 1
    %endrep
%endmacro

JMP_TABLE generate_grain_y_16bpc_ssse3, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_420_16bpc_ssse3, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_422_16bpc_ssse3, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_444_16bpc_ssse3, 0, 1, 2, 3

SECTION .text

%if ARCH_X86_32
%undef base
%define PIC_ptr(a) base+a
%else
%define PIC_ptr(a) a
%endif

%define m(x) mangle(private_prefix %+ _ %+ x %+ SUFFIX)

%macro vpgatherdw 5-8 8, 1 ; dst, src, base, tmp_gpr[x2], cnt, stride, tmp_xmm_reg
%assign %%idx 0
%define %%tmp %2
%if %0 == 8
%define %%tmp %8
%endif
%rep (%6/2)
%if %%idx == 0
    movd        %5 %+ d, %2
    pshuflw       %%tmp, %2, q3232
%else
    movd        %5 %+ d, %%tmp
%if %6 == 8
%if %%idx == 2
    punpckhqdq    %%tmp, %%tmp
%elif %%idx == 4
    psrlq         %%tmp, 32
%endif
%endif
%endif
    movzx       %4 %+ d, %5 %+ w
    shr         %5 %+ d, 16

%if %%idx == 0
    movd             %1, [%3+%4*%7]
%else
    pinsrw           %1, [%3+%4*%7], %%idx + 0
%endif
    pinsrw           %1, [%3+%5*%7], %%idx + 1
%assign %%idx %%idx+2
%endrep
%endmacro

%macro SPLATD 2 ; dst, src
%ifnidn %1, %2
    movd %1, %2
%endif
    pshufd %1, %1, q0000
%endmacro

%macro SPLATW 2 ; dst, src
%ifnidn %1, %2
    movd %1, %2
%endif
    pshuflw %1, %1, q0000
    punpcklqdq %1, %1
%endmacro


INIT_XMM ssse3
%if ARCH_X86_64
cglobal generate_grain_y_16bpc, 3, 8, 16, buf, fg_data, bdmax
    lea              r4, [pb_mask]
%define base r4-pb_mask
%else
cglobal generate_grain_y_16bpc, 3, 6, 8, buf, fg_data, bdmax
    LEA              r4, $$
%define base r4-$$
%endif
    movq             m1, [base+rnd_next_upperbit_mask]
    movq             m4, [base+mul_bits]
    movq             m7, [base+hmul_bits]
    mov             r3d, [fg_dataq+FGData.grain_scale_shift]
    lea             r5d, [bdmaxq+1]
    shr             r5d, 11             ; 0 for 10bpc, 2 for 12bpc
    sub              r3, r5
    SPLATW           m6, [base+round+r3*2-2]
    mova             m5, [base+pb_mask]
    SPLATW           m0, [fg_dataq+FGData.seed]
    mov              r3, -73*82*2
    sub            bufq, r3
%if ARCH_X86_64
    lea              r6, [gaussian_sequence]
%endif
.loop:
    pand             m2, m0, m1
    psrlw            m3, m2, 10
    por              m2, m3             ; bits 0xf, 0x1e, 0x3c and 0x78 are set
    pmullw           m2, m4             ; bits 0x0f00 are set
    pshufb           m3, m5, m2         ; set 15th bit for next 4 seeds
    psllq            m2, m3, 30
    por              m2, m3
    psllq            m3, m2, 15
    por              m2, m3             ; aggregate each bit into next seed's high bit
    pmulhuw          m3, m0, m7
    por              m2, m3             ; 4 next output seeds
    pshuflw          m0, m2, q3333
    psrlw            m2, 5
%if ARCH_X86_64
    vpgatherdw       m3, m2, r6, r5, r7, 4, 2
%else
    vpgatherdw       m3, m2, base+gaussian_sequence, r5, r2, 4, 2
%endif
    paddw            m3, m3             ; otherwise bpc=12 w/ grain_scale_shift=0
                                        ; shifts by 0, which pmulhrsw does not support
    pmulhrsw         m3, m6
    movq      [bufq+r3], m3
    add              r3, 4*2
    jl .loop

    ; auto-regression code
    movsxd           r3, [fg_dataq+FGData.ar_coeff_lag]
    movsxd           r3, [base+generate_grain_y_16bpc_ssse3_table+r3*4]
    lea              r3, [r3+base+generate_grain_y_16bpc_ssse3_table]
    jmp              r3

.ar1:
%if WIN64
    DEFINE_ARGS shift, fg_data, max, buf, val3, min, cf3, x, val0
    lea            bufq, [r0-2*(82*73-(82*3+79))]
    PUSH             r8
%else
%if ARCH_X86_64
    DEFINE_ARGS buf, fg_data, max, shift, val3, min, cf3, x, val0
%else ; x86-32
    DEFINE_ARGS buf, fg_data, min, val3, x, cf3, val0
    PUSH             r6
%define shiftd r1d
%endif
    sub            bufq, 2*(82*73-(82*3+79))
%endif
    movsx          cf3d, byte [fg_dataq+FGData.ar_coeffs_y+3]
    movd             m4, [fg_dataq+FGData.ar_coeffs_y]
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
%if WIN64
    DEFINE_ARGS shift, h, max, buf, val3, min, cf3, x, val0
%elif ARCH_X86_64
    DEFINE_ARGS buf, h, max, shift, val3, min, cf3, x, val0
%else ; x86-32
%undef shiftd
    DEFINE_ARGS buf, shift, min, val3, x, cf3, val0
%define hd dword r0m
%define maxd dword minm
%endif
%if cpuflag(sse4)
    pmovsxbw         m4, m4
%else
    pxor             m3, m3
    pcmpgtb          m3, m4
    punpcklbw        m4, m3
%endif
    pinsrw           m4, [base+pw_1], 3
    pshufd           m5, m4, q1111
    pshufd           m4, m4, q0000
    SPLATW           m3, [base+round_vals+shiftq*2-12]    ; rnd
    mov              hd, 70
    sar            maxd, 1
    mov            mind, maxd
    xor            mind, -1
.y_loop_ar1:
    mov              xq, -76
    movsx         val3d, word [bufq+xq*2-2]
.x_loop_ar1:
    movu             m0, [bufq+xq*2-82*2-2]     ; top/left
    psrldq           m2, m0, 2                  ; top
    psrldq           m1, m0, 4                  ; top/right
    punpcklwd        m0, m2
    punpcklwd        m1, m3
    pmaddwd          m0, m4
    pmaddwd          m1, m5
    paddd            m0, m1
.x_loop_ar1_inner:
    movd          val0d, m0
    psrldq           m0, 4
    imul          val3d, cf3d
    add           val3d, val0d
    sar           val3d, shiftb
    movsx         val0d, word [bufq+xq*2]
    add           val3d, val0d
    cmp           val3d, maxd
    cmovg         val3d, maxd
    cmp           val3d, mind
    cmovl         val3d, mind
    mov word [bufq+xq*2], val3w
    ; keep val3d in-place as left for next x iteration
    inc              xq
    jz .x_loop_ar1_end
    test             xq, 3
    jnz .x_loop_ar1_inner
    jmp .x_loop_ar1

.x_loop_ar1_end:
    add            bufq, 82*2
    dec              hd
    jg .y_loop_ar1
%if WIN64
    POP              r8
%elif ARCH_X86_32
    POP              r6
%undef maxd
%undef hd
%endif
.ar0:
    RET

.ar2:
%if ARCH_X86_32
%assign stack_offset_old stack_offset
    ALLOC_STACK -16*8
%endif
    DEFINE_ARGS buf, fg_data, bdmax, shift
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movd             m0, [base+round_vals-12+shiftq*2]
    pshuflw          m0, m0, q0000
    movu             m6, [fg_dataq+FGData.ar_coeffs_y+0]    ; cf0-11
    pxor             m2, m2
    punpcklwd        m0, m2
    pcmpgtb          m2, m6
    punpckhbw        m3, m6, m2
    punpcklbw        m6, m2
    pshufd           m2, m6, q3333
    pshufd           m1, m6, q2222
    pshufd           m7, m6, q1111
    pshufd           m6, m6, q0000
    pshufd           m4, m3, q1111
    pshufd           m3, m3, q0000
%if ARCH_X86_64
    SWAP              0, 12
    SWAP              1, 8
    SWAP              2, 9
    SWAP              3, 10
    SWAP              4, 11
%else
%define m12 [rsp+0*16]
%define m8 [rsp+1*16]
%define m9 [rsp+2*16]
%define m10 [rsp+3*16]
%define m11 [rsp+4*16]
    mova            m12, m0
    mova             m8, m1
    mova             m9, m2
    mova            m10, m3
    mova            m11, m4
    mov          bdmaxd, bdmaxm
%endif
    sar          bdmaxd, 1
    SPLATW           m0, bdmaxd                             ; max_grain
    pcmpeqw          m1, m1
%if !cpuflag(sse4)
    pcmpeqw          m2, m2
    psrldq           m2, 14
    pslldq           m2, 2
    pxor             m2, m1
%endif
    pxor             m1, m0                                 ; min_grain
%if ARCH_X86_64
    SWAP              0, 13
    SWAP              1, 14
    SWAP              2, 15
%else
%define m13 [rsp+5*16]
%define m14 [rsp+6*16]
    mova            m13, m0
    mova            m14, m1
%if !cpuflag(sse4)
%define m15 [rsp+7*16]
    mova            m15, m2
%endif
%endif
    sub            bufq, 2*(82*73-(82*3+79))
    DEFINE_ARGS buf, fg_data, h, x
    mov              hd, 70
.y_loop_ar2:
    mov              xq, -76

.x_loop_ar2:
    movu             m0, [bufq+xq*2-82*4-4]     ; y=-2,x=[-2,+5]
    movu             m1, [bufq+xq*2-82*2-4]     ; y=-1,x=[-2,+5]
    psrldq           m2, m0, 2
    psrldq           m3, m0, 4
    psrldq           m4, m0, 6
    psrldq           m5, m0, 8
    punpcklwd        m0, m2
    punpcklwd        m3, m4
    punpcklwd        m5, m1
    psrldq           m2, m1, 2
    psrldq           m4, m1, 4
    punpcklwd        m2, m4
    psrldq           m4, m1, 6
    psrldq           m1, 8
    punpcklwd        m4, m1
    pmaddwd          m0, m6
    pmaddwd          m3, m7
    pmaddwd          m5, m8
    pmaddwd          m2, m9
    pmaddwd          m4, m10
    paddd            m0, m3
    paddd            m5, m2
    paddd            m0, m4
    paddd            m0, m5                     ; accumulated top 2 rows
    paddd            m0, m12

    movu             m1, [bufq+xq*2-4]      ; y=0,x=[-2,+5]
    pshufd           m4, m1, q3321
    pxor             m2, m2
    pcmpgtw          m2, m4
    punpcklwd        m4, m2                 ; in dwords, y=0,x=[0,3]
.x_loop_ar2_inner:
    pmaddwd          m2, m1, m11
    paddd            m2, m0
    psrldq           m0, 4                  ; shift top to next pixel
    psrad            m2, [fg_dataq+FGData.ar_coeff_shift]
    paddd            m2, m4
    packssdw         m2, m2
    pminsw           m2, m13
    pmaxsw           m2, m14
    psrldq           m4, 4
    pslldq           m2, 2
    psrldq           m1, 2
%if cpuflag(sse4)
    pblendw          m1, m2, 00000010b
%else
    pand             m1, m15
    pandn            m3, m15, m2
    por              m1, m3
%endif
    ; overwrite previous pixel, this should be ok
    movd  [bufq+xq*2-2], m1
    inc              xq
    jz .x_loop_ar2_end
    test             xq, 3
    jnz .x_loop_ar2_inner
    jmp .x_loop_ar2

.x_loop_ar2_end:
    add            bufq, 82*2
    dec              hd
    jg .y_loop_ar2
%if ARCH_X86_32
%undef m8
%undef m9
%undef m10
%undef m11
%undef m12
%undef m13
%undef m14
%undef m15
%endif
    RET

.ar3:
    DEFINE_ARGS buf, fg_data, bdmax, shift
%if WIN64
    mov              r6, rsp
    and             rsp, ~15
    sub             rsp, 64
    %define         tmp  rsp
%elif ARCH_X86_64
    %define         tmp  rsp+stack_offset-72
%else
%assign stack_offset stack_offset_old
    ALLOC_STACK  -16*12
    %define         tmp  rsp
    mov          bdmaxd, bdmaxm
%endif
    sar          bdmaxd, 1
    SPLATW           m7, bdmaxd                                 ; max_grain
    pcmpeqw          m6, m6
%if !cpuflag(sse4)
    pcmpeqw          m4, m4
    psrldq           m4, 14
    pslldq           m4, 4
    pxor             m4, m6
%endif
    pxor             m6, m7                                    ; min_grain
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]

%if ARCH_X86_64
    SWAP              6, 14
    SWAP              7, 15
%else
%define m14 [rsp+10*16]
%define m15 [esp+11*16]
    mova            m14, m6
    mova            m15, m7
%endif

    ; build cf0-1 until 18-19 in m5-12 and r0/1
    pxor             m1, m1
    movu             m0, [fg_dataq+FGData.ar_coeffs_y+ 0]       ; cf0-15
    pcmpgtb          m1, m0
    punpckhbw        m2, m0, m1
    punpcklbw        m0, m1

%if cpuflag(sse4)
    pshufd           m4, m2, q3333
%else
    pshufd           m5, m2, q3333
    mova       [tmp+48], m5
%endif
    pshufd           m3, m2, q2222
    pshufd           m1, m2, q0000
    pshufd           m2, m2, q1111
    pshufd           m7, m0, q2222
    pshufd           m6, m0, q1111
    pshufd           m5, m0, q0000
    pshufd           m0, m0, q3333

%if ARCH_X86_64
    SWAP              0, 8
    SWAP              1, 9
    SWAP              2, 10
    SWAP              3, 11
    SWAP              4, 12
%else
%define m8 [rsp+4*16]
%define m9 [esp+5*16]
%define m10 [rsp+6*16]
%define m11 [esp+7*16]
%define m12 [rsp+8*16]
    mova             m8, m0
    mova             m9, m1
    mova            m10, m2
    mova            m11, m3
    mova            m12, m4
%endif

    ; build cf20,round in r2
    ; build cf21-23,round*2 in m13
    pxor             m1, m1
    movq             m0, [fg_dataq+FGData.ar_coeffs_y+16]       ; cf16-23
    pcmpgtb          m1, m0
    punpcklbw        m0, m1
    pshufd           m1, m0, q0000
    pshufd           m2, m0, q1111
    mova       [tmp+ 0], m1
    mova       [tmp+16], m2
    psrldq           m3, m0, 10
    pinsrw           m3, [base+round_vals+shiftq*2-10], 3

%if ARCH_X86_64
    SWAP              3, 13
%else
%define m13 [esp+9*16]
    mova            m13, m3
%endif

    pinsrw           m0, [base+round_vals+shiftq*2-12], 5
    pshufd           m3, m0, q2222
    mova       [tmp+32], m3

    DEFINE_ARGS buf, fg_data, h, x
    sub            bufq, 2*(82*73-(82*3+79))
    mov              hd, 70
.y_loop_ar3:
    mov              xq, -76

.x_loop_ar3:
    movu             m0, [bufq+xq*2-82*6-6+ 0]      ; y=-3,x=[-3,+4]
    movd             m1, [bufq+xq*2-82*6-6+16]      ; y=-3,x=[+5,+6]
    palignr          m2, m1, m0, 2                  ; y=-3,x=[-2,+5]
    palignr          m1, m1, m0, 12                 ; y=-3,x=[+3,+6]
    punpckhwd        m3, m0, m2                     ; y=-3,x=[+1/+2,+2/+3,+3/+4,+4/+5]
    punpcklwd        m0, m2                         ; y=-3,x=[-3/-2,-2/-1,-1/+0,+0/+1]
    shufps           m2, m0, m3, q1032              ; y=-3,x=[-1/+0,+0/+1,+1/+2,+2/+3]

    pmaddwd          m0, m5
    pmaddwd          m2, m6
    pmaddwd          m3, m7
    paddd            m0, m2
    paddd            m0, m3
    ; m0 = top line first 6 multiplied by cf, m1 = top line last entry

    movu             m2, [bufq+xq*2-82*4-6+ 0]      ; y=-2,x=[-3,+4]
    movd             m3, [bufq+xq*2-82*4-6+16]      ; y=-2,x=[+5,+6]
    punpcklwd        m1, m2                         ; y=-3/-2,x=[+3/-3,+4/-2,+5/-1,+6/+0]
    palignr          m4, m3, m2, 2                  ; y=-3,x=[-2,+5]
    palignr          m3, m3, m2, 4                  ; y=-3,x=[-1,+6]
    punpckhwd        m2, m4, m3                     ; y=-2,x=[+2/+3,+3/+4,+4/+5,+5/+6]
    punpcklwd        m4, m3                         ; y=-2,x=[-2/-1,-1/+0,+0/+1,+1/+2]
    shufps           m3, m4, m2, q1032              ; y=-2,x=[+0/+1,+1/+2,+2/+3,+3/+4]

    pmaddwd          m1, m8
    pmaddwd          m4, m9
    pmaddwd          m3, m10
    pmaddwd          m2, m11
    paddd            m1, m4
    paddd            m3, m2
    paddd            m0, m1
    paddd            m0, m3
    ; m0 = top 2 lines multiplied by cf

    movu             m1, [bufq+xq*2-82*2-6+ 0]      ; y=-1,x=[-3,+4]
    movd             m2, [bufq+xq*2-82*2-6+16]      ; y=-1,x=[+5,+6]
    palignr          m3, m2, m1, 2                  ; y=-1,x=[-2,+5]
    palignr          m2, m2, m1, 12                 ; y=-1,x=[+3,+6]
    punpckhwd        m4, m1, m3                     ; y=-1,x=[+1/+2,+2/+3,+3/+4,+4/+5]
    punpcklwd        m1, m3                         ; y=-1,x=[-3/-2,-2/-1,-1/+0,+0/+1]
    shufps           m3, m1, m4, q1032              ; y=-1,x=[-1/+0,+0/+1,+1/+2,+2/+3]
    punpcklwd        m2, [base+pw_1]

%if cpuflag(sse4)
    pmaddwd          m1, m12
%else
    pmaddwd          m1, [tmp+48]
%endif
    pmaddwd          m3, [tmp+ 0]
    pmaddwd          m4, [tmp+16]
    pmaddwd          m2, [tmp+32]
    paddd            m1, m3
    paddd            m4, m2
    paddd            m0, m1
    paddd            m0, m4
    ; m0 = top 3 lines multiplied by cf plus rounding for downshift

    movu             m1, [bufq+xq*2-6]      ; y=0,x=[-3,+4]
.x_loop_ar3_inner:
    pmaddwd          m2, m1, m13
    pshufd           m3, m2, q1111
    paddd            m2, m3                 ; left+cur
    paddd            m2, m0                 ; add top
    psrldq           m0, 4
    psrad            m2, [fg_dataq+FGData.ar_coeff_shift]
    packssdw         m2, m2
    pminsw           m2, m15
    pmaxsw           m2, m14
    pslldq           m2, 4
    psrldq           m1, 2
%if cpuflag(sse4)
    pblendw          m1, m2, 00000100b
%else
    pand             m1, m12
    pandn            m3, m12, m2
    por              m1, m3
%endif
    ; overwrite a couple of pixels, should be ok
    movq  [bufq+xq*2-4], m1
    inc              xq
    jz .x_loop_ar3_end
    test             xq, 3
    jnz .x_loop_ar3_inner
    jmp .x_loop_ar3

.x_loop_ar3_end:
    add            bufq, 82*2
    dec              hd
    jg .y_loop_ar3
%if WIN64
    mov             rsp, r6
%elif ARCH_X86_32
%undef m8
%undef m9
%undef m10
%undef m11
%undef m12
%undef m13
%undef m14
%undef m15
%endif
    RET

%macro generate_grain_uv_fn 3 ; ss_name, ss_x, ss_y
INIT_XMM ssse3
%if ARCH_X86_64
cglobal generate_grain_uv_%1_16bpc, 4, 11, 16, buf, bufy, fg_data, uv, bdmax, x, gaussian_reg, h, pic_reg
%define base r8-pb_mask
    lea              r8, [pb_mask]
    movifnidn    bdmaxd, bdmaxm
    lea             r6d, [bdmaxq+1]
%else
cglobal generate_grain_uv_%1_16bpc, 1, 7, 8, buf, x, pic_reg, fg_data, h
%define base r2-$$
    LEA              r2, $$
    mov        fg_dataq, r2m
    mov             r6d, r4m
    inc             r6d
%endif
    movq             m1, [base+rnd_next_upperbit_mask]
    movq             m4, [base+mul_bits]
    movq             m7, [base+hmul_bits]
    mov             r5d, [fg_dataq+FGData.grain_scale_shift]
    shr             r6d, 11             ; 0 for 10bpc, 2 for 12bpc
    sub              r5, r6
    SPLATW           m6, [base+round+r5*2-2]
    mova             m5, [base+pb_mask]
    SPLATW           m0, [fg_dataq+FGData.seed]
%if ARCH_X86_64
    SPLATW           m2, [base+pw_seed_xor+uvq*4]
%else
    mov             r5d, r3m
    SPLATW           m2, [base+pw_seed_xor+r5*4]
%endif
    pxor             m0, m2
%if ARCH_X86_64
    lea              r6, [gaussian_sequence]
%endif
%if %2
    mov              hd, 73-35*%3
    add            bufq, 44*2
.loop_y:
    mov              xq, -44
%else
    mov              xq, -82*73
    add            bufq, 82*73*2
%endif
.loop_x:
    pand             m2, m0, m1
    psrlw            m3, m2, 10
    por              m2, m3             ; bits 0xf, 0x1e, 0x3c and 0x78 are set
    pmullw           m2, m4             ; bits 0x0f00 are set
    pshufb           m3, m5, m2         ; set 15th bit for next 4 seeds
    psllq            m2, m3, 30
    por              m2, m3
    psllq            m3, m2, 15
    por              m2, m3             ; aggregate each bit into next seed's high bit
    pmulhuw          m3, m0, m7
    por              m2, m3             ; 4 next output seeds
    pshuflw          m0, m2, q3333
    psrlw            m2, 5
%if ARCH_X86_64
    vpgatherdw       m3, m2, r6, r9, r10, 4, 2
%else
    vpgatherdw       m3, m2, base+gaussian_sequence, r5, r6, 4, 2
%endif
    paddw            m3, m3             ; otherwise bpc=12 w/ grain_scale_shift=0
                                        ; shifts by 0, which pmulhrsw does not support
    pmulhrsw         m3, m6
    movq    [bufq+xq*2], m3
    add              xq, 4
    jl .loop_x
%if %2
    add            bufq, 82*2
    dec              hd
    jg .loop_y
%endif

    ; auto-regression code
    movsxd           r5, [fg_dataq+FGData.ar_coeff_lag]
    movsxd           r5, [base+generate_grain_uv_%1_16bpc_ssse3_table+r5*4]
    lea              r5, [r5+base+generate_grain_uv_%1_16bpc_ssse3_table]
    jmp              r5

.ar0:
%if ARCH_X86_64
    DEFINE_ARGS buf, bufy, fg_data, uv, bdmax, shift
%else
    DEFINE_ARGS buf, bufy, pic_reg, fg_data, uv, shift
%assign stack_offset_old stack_offset
    ALLOC_STACK  -16*2
    mov           bufyq, r1m
    mov             uvd, r3m
%endif
    imul            uvd, 28
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movd             m4, [fg_dataq+FGData.ar_coeffs_uv+uvq]
    SPLATW           m3, [base+hmul_bits+shiftq*2-10]
%if ARCH_X86_64
    sar          bdmaxd, 1
    SPLATW           m1, bdmaxd                     ; max_gain
%else
    SPLATW           m1, r4m
    psraw            m1, 1
%endif
    pcmpeqw          m7, m7
    pxor             m7, m1                         ; min_grain
%if ARCH_X86_64
    SWAP              1, 14
    DEFINE_ARGS buf, bufy, h, x
%else
%define m14 [rsp+0*16]
    mova            m14, m1
    DEFINE_ARGS buf, bufy, pic_reg, h, x
%endif
    pxor             m5, m5
    pcmpgtb          m5, m4
    punpcklbw        m4, m5
%if %2
    SPLATW           m6, [base+hmul_bits+2+%3*2]
%endif
    SPLATW           m4, m4
    pxor             m5, m5
%if %2
%if !cpuflag(sse4)
    pcmpeqw          m2, m2
    pslldq           m2, 12
%if ARCH_X86_64
    SWAP              2, 12
%else
%define m12 [rsp+1*16]
    mova            m12, m2
%endif
%endif
%endif
%if %2
    sub            bufq, 2*(82*(73-35*%3)+82-(82*3+41))
%else
    sub            bufq, 2*(82*70-3)
%endif
    add           bufyq, 2*(3+82*3)
    mov              hd, 70-35*%3
.y_loop_ar0:
    ; first 32 pixels
    xor              xd, xd
.x_loop_ar0:
    movu             m0, [bufyq+xq*(2<<%2)]
%if %2
%if %3
    movu             m2, [bufyq+xq*4+82*2]
    paddw            m0, m2
%endif
    movu             m1, [bufyq+xq*4     +16]
%if %3
    movu             m2, [bufyq+xq*4+82*2+16]
    paddw            m1, m2
%endif
    phaddw           m0, m1
    pmulhrsw         m0, m6
%endif
    punpckhwd        m1, m0, m5
    punpcklwd        m0, m5
    REPX {pmaddwd x, m4}, m0, m1
    REPX {psrad x, 5}, m0, m1
    packssdw         m0, m1
    pmulhrsw         m0, m3
    movu             m1, [bufq+xq*2]
    paddw            m0, m1
    pminsw           m0, m14
    pmaxsw           m0, m7
    cmp              xd, 72-40*%2
    je .end
    movu    [bufq+xq*2], m0
    add              xd, 8
    jmp .x_loop_ar0

    ; last 6/4 pixels
.end:
%if %2
%if cpuflag(sse4)
    pblendw          m0, m1, 11000000b
%else
    pand             m1, m12
    pandn            m2, m12, m0
    por              m0, m1, m2
%endif
    movu    [bufq+xq*2], m0
%else
    movq    [bufq+xq*2], m0
%endif

    add            bufq, 82*2
    add           bufyq, 82*(2<<%3)
    dec              hd
    jg .y_loop_ar0
%if ARCH_X86_32
%undef m12
%undef m14
%endif
    RET

.ar1:
%if ARCH_X86_64
    DEFINE_ARGS buf, bufy, fg_data, uv, max, cf3, min, val3, x
%else
%assign stack_offset stack_offset_old
%xdefine rstk rsp
%assign stack_size_padded 0
    DEFINE_ARGS buf, shift, pic_reg, fg_data, uv, bufy, cf3
    mov           bufyq, r1m
    mov             uvd, r3m
%endif
    imul            uvd, 28
    movsx          cf3d, byte [fg_dataq+FGData.ar_coeffs_uv+uvq+3]
    movq             m4, [fg_dataq+FGData.ar_coeffs_uv+uvq]
%if WIN64
    DEFINE_ARGS shift, bufy, h, buf, max, cf3, min, val3, x, val0
%if %2
    lea            bufq, [r0-2*(82*(73-35*%3)+44-(82*3+41))]
%else
    lea            bufq, [r0-2*(82*69+3)]
%endif
%else
%if ARCH_X86_64
    DEFINE_ARGS buf, bufy, h, shift, max, cf3, min, val3, x, val0
%else
    DEFINE_ARGS buf, shift, pic_reg, fg_data, val0, bufy, cf3
%define hd dword r1m
%define mind dword r3m
%define maxd dword r4m
%endif
%if %2
    sub            bufq, 2*(82*(73-35*%3)+44-(82*3+41))
%else
    sub            bufq, 2*(82*69+3)
%endif
%endif
%if ARCH_X86_64
    mov          shiftd, [r2+FGData.ar_coeff_shift]
%else
    mov          shiftd, [r3+FGData.ar_coeff_shift]
%endif
    pxor             m5, m5
    pcmpgtb          m5, m4
    punpcklbw        m4, m5                 ; cf0-4 in words
    pshuflw          m4, m4, q2100
    psrldq           m4, 2                  ; cf0-3,4 in words
    pshufd           m5, m4, q1111
    pshufd           m4, m4, q0000
    movd             m3, [base+round_vals+shiftq*2-12]    ; rnd
    pxor             m6, m6
    punpcklwd        m3, m6
%if %2
    SPLATW           m6, [base+hmul_bits+2+%3*2]
%endif
    SPLATD           m3, m3
    add           bufyq, 2*(79+82*3)
    mov              hd, 70-35*%3
    sar            maxd, 1
%if ARCH_X86_64
    mov            mind, maxd
    xor            mind, -1
%else
    DEFINE_ARGS buf, shift, val3, x, val0, bufy, cf3
    mov              r2, maxd
    xor              r2, -1
    mov            mind, r2
%endif
.y_loop_ar1:
    mov              xq, -(76>>%2)
    movsx         val3d, word [bufq+xq*2-2]
.x_loop_ar1:
    movu             m0, [bufq+xq*2-82*2-2] ; top/left
%if %2
    movu             m7, [bufyq+xq*4]
%if %3
    movu             m1, [bufyq+xq*4+82*2]
    phaddw           m7, m1
%else
    phaddw           m7, m7
%endif
%else
    movq             m7, [bufyq+xq*2]
%endif
    psrldq           m2, m0, 2              ; top
    psrldq           m1, m0, 4              ; top/right
    punpcklwd        m0, m2
%if %2
%if %3
    pshufd           m2, m7, q3232
    paddw            m7, m2
%endif
    pmulhrsw         m7, m6
%endif
    punpcklwd        m1, m7
    pmaddwd          m0, m4
    pmaddwd          m1, m5
    paddd            m0, m1
    paddd            m0, m3
.x_loop_ar1_inner:
    movd          val0d, m0
    psrldq           m0, 4
    imul          val3d, cf3d
    add           val3d, val0d
    sar           val3d, shiftb
    movsx         val0d, word [bufq+xq*2]
    add           val3d, val0d
    cmp           val3d, maxd
    cmovg         val3d, maxd
    cmp           val3d, mind
    cmovl         val3d, mind
    mov word [bufq+xq*2], val3w
    ; keep val3d in-place as left for next x iteration
    inc              xq
    jz .x_loop_ar1_end
    test             xq, 3
    jnz .x_loop_ar1_inner
    jmp .x_loop_ar1

.x_loop_ar1_end:
    add            bufq, 82*2
    add           bufyq, 82*2<<%3
    dec              hd
    jg .y_loop_ar1
%if ARCH_X86_32
%undef maxd
%undef mind
%undef hd
%endif
    RET

.ar2:
%if ARCH_X86_64
    DEFINE_ARGS buf, bufy, fg_data, uv, bdmax, shift
%else
    DEFINE_ARGS buf, bufy, pic_reg, fg_data, uv, shift
    ALLOC_STACK  -16*8
    mov           bufyq, r1m
    mov             uvd, r3m
%endif
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    imul            uvd, 28
%if ARCH_X86_64
    sar          bdmaxd, 1
    SPLATW           m5, bdmaxd                 ; max_grain
%else
    SPLATW           m5, r4m
    psraw            m5, 1
%endif
    pcmpeqw          m6, m6
%if !cpuflag(sse4)
    pcmpeqw          m7, m7
    psrldq           m7, 14
    pslldq           m7, 2
    pxor             m7, m6
%endif
    pxor             m6, m5                    ; min_grain
%if %2 && cpuflag(sse4)
    SPLATW           m7, [base+hmul_bits+2+%3*2]
%endif

%if ARCH_X86_64
    SWAP              5, 13
    SWAP              6, 14
    SWAP              7, 15
%else
%define m13 [rsp+5*16]
%define m14 [rsp+6*16]
%define m15 [rsp+7*16]
    mova            m13, m5
    mova            m14, m6
    mova            m15, m7
%endif

    ; coef values
    movu             m0, [fg_dataq+FGData.ar_coeffs_uv+uvq+0]
    pxor             m1, m1
    pcmpgtb          m1, m0
    punpckhbw        m2, m0, m1
    punpcklbw        m0, m1
    pinsrw           m2, [base+round_vals-12+shiftq*2], 5

    pshufd           m6, m0, q0000
    pshufd           m7, m0, q1111
    pshufd           m1, m0, q3333
    pshufd           m0, m0, q2222
    pshufd           m3, m2, q1111
    pshufd           m4, m2, q2222
    pshufd           m2, m2, q0000

%if ARCH_X86_64
    SWAP              0, 8
    SWAP              1, 9
    SWAP              2, 10
    SWAP              3, 11
    SWAP              4, 12
%else
%define m8 [rsp+0*16]
%define m9 [rsp+1*16]
%define m10 [rsp+2*16]
%define m11 [rsp+3*16]
%define m12 [rsp+4*16]
    mova             m8, m0
    mova             m9, m1
    mova            m10, m2
    mova            m11, m3
    mova            m12, m4
%endif

%if ARCH_X86_64
    DEFINE_ARGS buf, bufy, fg_data, h, x
%else
    DEFINE_ARGS buf, bufy, pic_reg, fg_data, h, x
%endif
%if %2
    sub            bufq, 2*(82*(73-35*%3)+44-(82*3+41))
%else
    sub            bufq, 2*(82*69+3)
%endif
    add           bufyq, 2*(79+82*3)
    mov              hd, 70-35*%3
.y_loop_ar2:
    mov              xq, -(76>>%2)

.x_loop_ar2:
    movu             m0, [bufq+xq*2-82*4-4]     ; y=-2,x=[-2,+5]
    movu             m5, [bufq+xq*2-82*2-4]     ; y=-1,x=[-2,+5]
    psrldq           m4, m0, 2                  ; y=-2,x=[-1,+5]
    psrldq           m1, m0, 4                  ; y=-2,x=[-0,+5]
    psrldq           m3, m0, 6                  ; y=-2,x=[+1,+5]
    psrldq           m2, m0, 8                  ; y=-2,x=[+2,+5]
    punpcklwd        m0, m4                     ; y=-2,x=[-2/-1,-1/+0,+0/+1,+1/+2]
    punpcklwd        m1, m3                     ; y=-2,x=[+0/+1,+1/+2,+2/+3,+3/+4]
    punpcklwd        m2, m5                     ; y=-2/-1,x=[+2/-2,+3/-1,+4/+0,+5/+1]
    pmaddwd          m0, m6
    pmaddwd          m1, m7
    pmaddwd          m2, m8
    paddd            m0, m1
    paddd            m0, m2
    psrldq           m3, m5, 2                  ; y=-1,x=[-1,+5]
    psrldq           m1, m5, 4                  ; y=-1,x=[-0,+5]
    psrldq           m4, m5, 6                  ; y=-1,x=[+1,+5]
    psrldq           m2, m5, 8                  ; y=-1,x=[+2,+5]
    punpcklwd        m3, m1
    punpcklwd        m4, m2
    pmaddwd          m3, m9
    pmaddwd          m4, m10
    paddd            m3, m4
    paddd            m0, m3

    ; luma component & rounding
%if %2
    movu             m1, [bufyq+xq*4]
%if %3
    movu             m2, [bufyq+xq*4+82*2]
    phaddw           m1, m2
    pshufd           m2, m1, q3232
    paddw            m1, m2
%else
    phaddw           m1, m1
%endif
%if cpuflag(sse4)
    pmulhrsw         m1, m15
%elif %3
    pmulhrsw         m1, [base+pw_8192]
%else
    pmulhrsw         m1, [base+pw_16384]
%endif
%else
    movq             m1, [bufyq+xq*2]
%endif
    punpcklwd        m1, [base+pw_1]
    pmaddwd          m1, m12
    paddd            m0, m1

    movu             m1, [bufq+xq*2-4]      ; y=0,x=[-2,+5]
    pshufd           m2, m1, q3321
    pxor             m3, m3
    pcmpgtw          m3, m2
    punpcklwd        m2, m3                 ; y=0,x=[0,3] in dword
.x_loop_ar2_inner:
    pmaddwd          m3, m1, m11
    paddd            m3, m0
    psrldq           m0, 4                  ; shift top to next pixel
    psrad            m3, [fg_dataq+FGData.ar_coeff_shift]
    ; we do not need to packssdw since we only care about one value
    paddd            m3, m2
    packssdw         m3, m3
    pminsw           m3, m13
    pmaxsw           m3, m14
    psrldq           m1, 2
    pslldq           m3, 2
    psrldq           m2, 4
%if cpuflag(sse4)
    pblendw          m1, m3, 00000010b
%else
    pand             m1, m15
    pandn            m4, m15, m3
    por              m1, m4
%endif
    ; overwrite previous pixel, should be ok
    movd  [bufq+xq*2-2], m1
    inc              xq
    jz .x_loop_ar2_end
    test             xq, 3
    jnz .x_loop_ar2_inner
    jmp .x_loop_ar2

.x_loop_ar2_end:
    add            bufq, 82*2
    add           bufyq, 82*2<<%3
    dec              hd
    jg .y_loop_ar2
%if ARCH_X86_32
%undef m13
%undef m14
%undef m15
%endif
    RET

.ar3:
%if ARCH_X86_64
    DEFINE_ARGS buf, bufy, fg_data, uv, bdmax, shift
%if WIN64
    mov              r6, rsp
    and             rsp, ~15
    sub             rsp, 96
    %define         tmp  rsp
%else
    %define         tmp  rsp+stack_offset-120
%endif
%else
    DEFINE_ARGS buf, bufy, pic_reg, fg_data, uv, shift
%assign stack_offset stack_offset_old
    ALLOC_STACK  -16*14
    mov           bufyq, r1m
    mov             uvd, r3m
    %define         tmp  rsp
%endif
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    imul            uvd, 28
    SPLATW           m4, [base+round_vals-12+shiftq*2]
    pxor             m5, m5
    pcmpgtw          m5, m4
    punpcklwd        m4, m5
%if ARCH_X86_64
    sar          bdmaxd, 1
    SPLATW           m6, bdmaxd                 ; max_grain
%else
    SPLATW           m6, r4m
    psraw            m6, 1
%endif
    pcmpeqw          m7, m7
%if !cpuflag(sse4)
    pcmpeqw          m3, m3
    psrldq           m3, 14
    pslldq           m3, 4
    pxor             m3, m7
%endif
    pxor             m7, m6                     ; min_grain
%if %2 && cpuflag(sse4)
    SPLATW           m3, [base+hmul_bits+2+%3*2]
%endif

%if ARCH_X86_64
    SWAP              3, 11
    SWAP              4, 12
    SWAP              6, 14
    SWAP              7, 15
%else
%define m11 [rsp+ 9*16]
%define m12 [rsp+10*16]
%define m14 [rsp+12*16]
%define m15 [rsp+13*16]
    mova            m11, m3
    mova            m12, m4
    mova            m14, m6
    mova            m15, m7
%endif

    ; cf from y=-3,x=-3 until y=-3,x=-2
    movu             m0, [fg_dataq+FGData.ar_coeffs_uv+uvq+ 0]
    pxor             m1, m1
    pcmpgtb          m1, m0
    punpckhbw        m2, m0, m1
    punpcklbw        m0, m1
    pshufd           m1, m0, q0000
    pshufd           m3, m0, q1111
    pshufd           m4, m0, q2222
    pshufd           m0, m0, q3333
    pshufd           m5, m2, q0000
    pshufd           m6, m2, q1111
    mova     [tmp+16*0], m1
    mova     [tmp+16*1], m3
    mova     [tmp+16*2], m4
    mova     [tmp+16*3], m0
    mova     [tmp+16*4], m5
    mova     [tmp+16*5], m6
    pshufd           m6, m2, q2222
    pshufd           m7, m2, q3333

    ; cf from y=-1,x=-1 to y=0,x=-1 + luma component
    movu             m0, [fg_dataq+FGData.ar_coeffs_uv+uvq+16]
    pxor             m1, m1
    pcmpgtb          m1, m0
    punpckhbw        m2, m0, m1                 ; luma
    punpcklbw        m0, m1
    pshufd           m3, m0, q3232
    psrldq           m5, m0, 10
    ; y=0,x=[-3 to -1] + "1.0" for current pixel
    pinsrw           m5, [base+round_vals-10+shiftq*2], 3
    ; y=-1,x=[-1 to +2]
    pshufd           m1, m0, q0000
    pshufd           m0, m0, q1111
    ; y=-1,x=+3 + luma
    punpcklwd        m3, m2
    pshufd           m3, m3, q0000

%if ARCH_X86_64
    SWAP              1, 8
    SWAP              0, 9
    SWAP              3, 10
    SWAP              5, 13
    DEFINE_ARGS buf, bufy, fg_data, h, x
%else
%define m8  [rsp+ 6*16]
%define m9  [rsp+ 7*16]
%define m10 [rsp+ 8*16]
%define m13 [rsp+11*16]
    mova             m8, m1
    mova             m9, m0
    mova            m10, m3
    mova            m13, m5
    DEFINE_ARGS buf, bufy, pic_reg, fg_data, h, x
%endif
%if %2
    sub            bufq, 2*(82*(73-35*%3)+44-(82*3+41))
%else
    sub            bufq, 2*(82*69+3)
%endif
    add           bufyq, 2*(79+82*3)
    mov              hd, 70-35*%3
.y_loop_ar3:
    mov              xq, -(76>>%2)

.x_loop_ar3:
    ; first line
    movu             m0, [bufq+xq*2-82*6-6+ 0]      ; y=-3,x=[-3,+4]
    movd             m1, [bufq+xq*2-82*6-6+16]      ; y=-3,x=[+5,+6]
    palignr          m2, m1, m0, 2                  ; y=-3,x=[-2,+5]
    palignr          m1, m1, m0, 12                 ; y=-3,x=[+3,+6]
    punpckhwd        m3, m0, m2                     ; y=-3,x=[+1/+2,+2/+3,+3/+4,+4/+5]
    punpcklwd        m0, m2                         ; y=-3,x=[-3/-2,-2/-1,-1/+0,+0/+1]
    shufps           m2, m0, m3, q1032              ; y=-3,x=[-1/+0,+0/+1,+1/+2,+2/+3]

    pmaddwd          m0, [tmp+0*16]
    pmaddwd          m2, [tmp+1*16]
    pmaddwd          m3, [tmp+2*16]
    paddd            m0, m2
    paddd            m0, m3                         ; first 6 x of top y

    ; second line [m0/1 are busy]
    movu             m2, [bufq+xq*2-82*4-6+ 0]      ; y=-2,x=[-3,+4]
    movd             m3, [bufq+xq*2-82*4-6+16]      ; y=-2,x=[+5,+6]
    punpcklwd        m1, m2                         ; y=-3/-2,x=[+3/-3,+4/-2,+5/-1,+6/+0]
    palignr          m4, m3, m2, 2                  ; y=-2,x=[-2,+5]
    palignr          m3, m3, m2, 4                  ; y=-2,x=[-2,+5]
    punpckhwd        m5, m4, m3                     ; y=-2,x=[+2/+3,+3/+4,+4/+5,+5/+6]
    punpcklwd        m4, m3                         ; y=-2,x=[-2/-1,-1/+0,+0/+1,+1/+2]
    shufps           m3, m4, m5, q1032              ; t=-2,x=[+0/+1,+1/+2,+2/+3,+3/+4]
    pmaddwd          m1, [tmp+3*16]
    pmaddwd          m4, [tmp+4*16]
    pmaddwd          m3, [tmp+5*16]
    pmaddwd          m5, m6
    paddd            m1, m4
    paddd            m3, m5
    paddd            m0, m1
    paddd            m0, m3                         ; top 2 lines

    ; third line [m0 is busy] & luma + round
    movu             m1, [bufq+xq*2-82*2-6+ 0]      ; y=-1,x=[-3,+4]
    movd             m2, [bufq+xq*2-82*2-6+16]      ; y=-1,x=[+5,+6]
%if %2
    movu             m5, [bufyq+xq*4]
%if %3
    movu             m4, [bufyq+xq*4+82*2]
    phaddw           m5, m4
%else
    phaddw           m5, m5
%endif
%else
    movq             m5, [bufyq+xq*2]
%endif
    palignr          m3, m2, m1, 2                  ; y=-1,x=[-2,+5]
    palignr          m2, m2, m1, 12                 ; y=-1,x=[+3,+6]
%if %3
    pshufd           m4, m5, q3232
    paddw            m5, m4
%endif
%if %2
%if cpuflag(sse4)
    pmulhrsw         m5, m11
%elif %3
    pmulhrsw         m5, [base+pw_8192]
%else
    pmulhrsw         m5, [base+pw_16384]
%endif
%endif
    punpckhwd        m4, m1, m3                     ; y=-1,x=[+1/+2,+2/+3,+3/+4,+4/+5]
    punpcklwd        m1, m3                         ; y=-1,x=[-3/-2,-2/-1,-1/+0,+0/+1]
    shufps           m3, m1, m4, q1032              ; y=-1,x=[-1/+0,+0/+1,+1/+2,+2/+3]
    punpcklwd        m2, m5
    pmaddwd          m1, m7
    pmaddwd          m3, m8
    pmaddwd          m4, m9
    pmaddwd          m2, m10
    paddd            m1, m3
    paddd            m4, m2
    paddd            m0, m12                        ; += round
    paddd            m1, m4
    paddd            m0, m1

    movu             m1, [bufq+xq*2-6]      ; y=0,x=[-3,+4]
.x_loop_ar3_inner:
    pmaddwd          m2, m1, m13
    pshufd           m3, m2, q1111
    paddd            m2, m3                 ; left+cur
    paddd            m2, m0                 ; add top
    psrldq           m0, 4
    psrad            m2, [fg_dataq+FGData.ar_coeff_shift]
    packssdw         m2, m2
    pminsw           m2, m14
    pmaxsw           m2, m15
    pslldq           m2, 4
    psrldq           m1, 2
%if cpuflag(sse4)
    pblendw          m1, m2, 00000100b
%else
    pand             m1, m11
    pandn            m3, m11, m2
    por              m1, m3
%endif
    ; overwrite previous pixels, should be ok
    movq  [bufq+xq*2-4], m1
    inc              xq
    jz .x_loop_ar3_end
    test             xq, 3
    jnz .x_loop_ar3_inner
    jmp .x_loop_ar3

.x_loop_ar3_end:
    add            bufq, 82*2
    add           bufyq, 82*2<<%3
    dec              hd
    jg .y_loop_ar3
%if WIN64
    mov             rsp, r6
%elif ARCH_X86_32
%undef m8
%undef m9
%undef m10
%undef m11
%undef m12
%undef m13
%undef m14
%undef m15
%endif
    RET
%endmacro

generate_grain_uv_fn 420, 1, 1
generate_grain_uv_fn 422, 1, 0
generate_grain_uv_fn 444, 0, 0

%macro SCRATCH 3
%if ARCH_X86_32
    mova [rsp+%3*mmsize], m%1
%define m%2 [rsp+%3*mmsize]
%else
    SWAP             %1, %2
%endif
%endmacro

INIT_XMM ssse3
%if ARCH_X86_32
%if STACK_ALIGNMENT < mmsize
cglobal fgy_32x32xn_16bpc, 0, 7, 8, 0-(8 * mmsize + 12 * gprsize), \
        dst, src, scaling, unused1, fg_data, picptr, unused2
    ; copy stack arguments to new position post-alignment, so that we
    ; don't have to keep the old stack location in a separate register
    mov              r0, r0m
    mov              r1, r2m
    mov              r2, r4m
    mov              r3, r6m
    mov              r4, r7m
    mov              r5, r8m

%define r0m [rsp+8*mmsize+ 3*gprsize]
%define r2m [rsp+8*mmsize+ 5*gprsize]
%define r4m [rsp+8*mmsize+ 7*gprsize]
%define r6m [rsp+8*mmsize+ 9*gprsize]
%define r7m [rsp+8*mmsize+10*gprsize]
%define r8m [rsp+8*mmsize+11*gprsize]

    mov             r0m, r0
    mov             r2m, r1
    mov             r4m, r2
    mov             r6m, r3
    mov             r7m, r4
    mov             r8m, r5
%else
cglobal fgy_32x32xn_16bpc, 0, 7, 8, 8 * mmsize + 4 * gprsize, \
        dst, src, scaling, unused1, fg_data, picptr, unused2
%endif
    mov            srcq, srcm
    mov        scalingq, r5m
    mov        fg_dataq, r3m
%if STACK_ALIGNMENT < mmsize
    mov              r6, r9m

%define r9m [rsp+8*mmsize+ 4*gprsize]
%define r3m [rsp+8*mmsize+ 6*gprsize]
%define r5m [rsp+8*mmsize+ 8*gprsize]

    mov             r9m, r6
%endif
    LEA              r5, $$
%define base r5-$$
    mov             r5m, picptrq
%else
cglobal fgy_32x32xn_16bpc, 6, 15, 16, dst, src, stride, fg_data, w, scaling, grain_lut
    lea              r8, [pb_mask]
%define base r8-pb_mask
%endif
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    SPLATW           m3, [base+mul_bits+r6*2-14]
    mov             r6d, [fg_dataq+FGData.clip_to_restricted_range]
%if ARCH_X86_32
    DECLARE_REG_TMP   0, 3
%else
    DECLARE_REG_TMP   9, 10
%endif
    mov             t0d, r9m        ; bdmax
    sar             t0d, 11         ; is_12bpc
    inc             t0d
    mov             t1d, r6d
    imul            t1d, t0d
    dec             t0d
    SPLATW           m5, [base+min+t1*2]
    lea             t0d, [t0d*3]
    lea             t0d, [r6d*2+t0d]
    SPLATW           m4, [base+max+t0*2]
    SPLATW           m2, r9m

    pcmpeqw          m1, m1
    psraw            m7, m2, 1              ; max_grain
    pxor             m1, m7                 ; min_grain
    SPLATD           m6, [base+pd_16]

    SCRATCH           1,  9, 0
    SCRATCH           2, 10, 1
    SCRATCH           3, 11, 2
    SCRATCH           4, 12, 3
    SCRATCH           5, 13, 4
    SCRATCH           6, 14, 5
    SCRATCH           7, 15, 6

    mova             m6, [base+pw_27_17_17_27]   ; for horizontal filter

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, unused2
    DECLARE_REG_TMP   0
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, unused1, \
                sby, see
    DECLARE_REG_TMP   7
%endif

    mov            sbyd, r8m
    movzx           t0d, byte [fg_dataq+FGData.overlap_flag]
    test            t0d, t0d
    jz .no_vertical_overlap
    test           sbyd, sbyd
    jnz .vertical_overlap
.no_vertical_overlap:
    mov       dword r8m, t0d

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, see, fg_data, picptr, unused
    imul           seed, (173 << 24) | 37
%else
    imul           seed, sbyd, (173 << 24) | 37
%endif
    add            seed, (105 << 24) | 178
    rol            seed, 8
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, see, w, picptr, src_bak

    mov             r3m, seed
    mov              wq, r4m
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                unused1, unused2, see, src_bak
%endif

    lea        src_bakq, [srcq+wq*2]
    mov            r9mp, src_bakq
    neg              wq
    sub           dstmp, srcq
%if ARCH_X86_32
    mov             r4m, wq
%endif

.loop_x:
%if ARCH_X86_32
    mov            seed, r3m
%endif
    mov             r6d, seed
    or             seed, 0xEFF4
    shr             r6d, 1
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                ; updated seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, unused1, unused2, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, src_bak

    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    shr           offxd, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offyq, [offyq+offxq*2+747] ; offy*stride+offx

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, src_bak
%endif

.loop_x_odd:
    movzx            hd, word r7m
    mov      grain_lutq, grain_lutmp
.loop_y:
    ; src
    pand             m0, m10, [srcq+ 0]
    pand             m1, m10, [srcq+16]          ; m0-1: src as word

    ; scaling[src]
%if ARCH_X86_32
    vpgatherdw       m2, m0, scalingq-1, r0, r5, 8, 1, m4
    vpgatherdw       m3, m1, scalingq-1, r0, r5, 8, 1, m4
%else
    vpgatherdw       m2, m0, scalingq-1, r11, r13, 8, 1, m4
    vpgatherdw       m3, m1, scalingq-1, r11, r13, 8, 1, m4
%endif
    REPX   {psrlw x, 8}, m2, m3

    ; grain = grain_lut[offy+y][offx+x]
    movu             m4, [grain_lutq+offxyq*2]
    movu             m5, [grain_lutq+offxyq*2+16]

    ; noise = round2(scaling[src] * grain, scaling_shift)
    REPX {pmullw x, m11}, m2, m3
    pmulhrsw         m4, m2
    pmulhrsw         m5, m3

    ; dst = clip_pixel(src, noise)
    paddw            m0, m4
    paddw            m1, m5
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova [dstq+srcq+ 0], m0
    mova [dstq+srcq+16], m1

    add            srcq, r2mp               ; src += stride
    add      grain_lutq, 82*2
    dec              hd
    jg .loop_y

%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end
%if ARCH_X86_32
    mov            srcq, r9mp
    add            srcq, r4mp
    add            srcq, r4mp
%else
    mov        src_bakq, r9mp
    lea            srcq, [src_bakq+wq*2]
%endif
    btc       dword r8m, 2
    jc .next_blk
    add          offxyd, 16
    test      dword r8m, 2
    jz .loop_x_odd
%if ARCH_X86_32
    add dword [rsp+8*mmsize+1*gprsize], 16
%else
    add            r12d, 16                 ; top_offxy += 16
%endif
    jmp .loop_x_odd_v_overlap

.next_blk:
    test      dword r8m, 1
    jz .loop_x

    ; r8m = sbym
    test      dword r8m, 2
    jnz .loop_x_hv_overlap

    ; horizontal overlap (without vertical overlap)
.loop_x_h_overlap:
%if ARCH_X86_32
    add          offxyd, 16
    mov [rsp+8*mmsize+0*gprsize], offxyd
    DEFINE_ARGS dst, src, scaling, see, w, picptr, src_bak
    mov            seed, r3m
%endif

    mov             r6d, seed
    or             seed, 0xEFF4
    shr             r6d, 1
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                ; updated seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, h, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, src_bak, left_offxy

    lea     left_offxyd, [offyd+16]         ; previous column's offy*stride+offx

    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    shr           offxd, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offyq, [offyq+offxq*2+747] ; offy*stride+offx

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, src_bak, left_offxy
%endif

    mov              hd, dword r7m
    mov      grain_lutq, grain_lutmp
.loop_y_h_overlap:
    ; grain = grain_lut[offy+y][offx+x]
    movu             m5, [grain_lutq+offxyq*2]
%if ARCH_X86_32
    mov              r5, [rsp+8*mmsize+0*gprsize]
    movd             m4, [grain_lutq+r5*2]
%else
    movd             m4, [grain_lutq+left_offxyq*2]
%endif
    punpcklwd        m4, m5
    pmaddwd          m4, m6
    paddd            m4, m14
    psrad            m4, 5
    packssdw         m4, m4
    pminsw           m4, m15
    pmaxsw           m4, m9
    shufps           m4, m5, q3210

    ; src
    pand             m0, m10, [srcq+ 0]
    pand             m1, m10, [srcq+16]          ; m0-1: src as word

    ; scaling[src]
%if ARCH_X86_32
    vpgatherdw       m2, m0, scalingq-1, r0, r5, 8, 1, m5
    vpgatherdw       m3, m1, scalingq-1, r0, r5, 8, 1, m5
%else
    vpgatherdw       m2, m0, scalingq-1, r13, r14, 8, 1, m5
    vpgatherdw       m3, m1, scalingq-1, r13, r14, 8, 1, m5
%endif
    REPX   {psrlw x, 8}, m2, m3

    ; noise = round2(scaling[src] * grain, scaling_shift)
    movu             m5, [grain_lutq+offxyq*2+16]
    REPX {pmullw x, m11}, m2, m3
    pmulhrsw         m4, m2
    pmulhrsw         m5, m3

    ; dst = clip_pixel(src, noise)
    paddw            m0, m4
    paddw            m1, m5
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova [dstq+srcq+ 0], m0
    mova [dstq+srcq+16], m1

    add            srcq, r2mp
    add      grain_lutq, 82*2
    dec              hd
    jg .loop_y_h_overlap

%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end
%if ARCH_X86_32
    mov            srcq, r9mp
    add            srcq, r4mp
    add            srcq, r4mp
%else
    mov        src_bakq, r9mp
    lea            srcq, [src_bakq+wq*2]
%endif
    or        dword r8m, 4
    add          offxyd, 16

    ; r8m = sbym
    test      dword r8m, 2
    jz .loop_x_odd
%if ARCH_X86_32
    add dword [rsp+8*mmsize+1*gprsize], 16
%else
    add            r12d, 16                 ; top_offxy += 16
%endif
    jmp .loop_x_odd_v_overlap

.end:
    RET

.vertical_overlap:
    or              t0d, 2
    mov             r8m, t0d

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, unused
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, unused1, \
                sby, see
%endif

    movzx          sbyd, sbyb
%if ARCH_X86_32
    imul             r4, [fg_dataq+FGData.seed], 0x00010001
    DEFINE_ARGS dst, src, scaling, sby, see, picptr, unused
%else
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
%endif
    imul            t0d, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add             t0d, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and             t0d, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, t0d
%if ARCH_X86_32
    xor            sbyd, seed

    DEFINE_ARGS dst, src, scaling, see, w, picptr, src_bak

    mov             r3m, seed
    mov              wq, r4m
%else
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                unused1, unused2, see, src_bak
%endif

    lea        src_bakq, [srcq+wq*2]
    mov            r9mp, src_bakq
    neg              wq
    sub           dstmp, srcq
%if ARCH_X86_32
    mov             r4m, wq
%endif

.loop_x_v_overlap:
%if ARCH_X86_32
    mov              r5, r5m
    SPLATD           m7, [base+pw_27_17_17_27]
    mov            seed, r3m
%else
    SPLATD           m7, [pw_27_17_17_27]
%endif

    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            t0b                     ; parity of top_seed
    shr            seed, 16
    shl             t0d, 16
    test           seeb, seeh
    setp            t0b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             t0d, r6d
    mov            seed, t0d
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, unused1, unused2, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, src_bak, unused, top_offxy

    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    ror           offxd, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyq, [offyq+offxq*2+0x10001*747+32*82]

%if ARCH_X86_32
    DEFINE_ARGS top_offxy, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, src_bak, unused, top_offxy
%endif

    movzx    top_offxyd, offxyw
%if ARCH_X86_32
    mov [rsp+8*mmsize+1*gprsize], top_offxyd

    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif
    shr          offxyd, 16

.loop_x_odd_v_overlap:
%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m7, [PIC_ptr(pw_27_17_17_27)]
    mov              hd, dword r7m
    mov      grain_lutq, grain_lutmp
.loop_y_v_overlap:
    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq*2]
%if ARCH_X86_32
    mov              r5, [rsp+8*mmsize+1*gprsize]
    movu             m2, [grain_lutq+r5*2]
%else
    movu             m2, [grain_lutq+top_offxyq*2]
%endif
    punpckhwd        m4, m2, m3
    punpcklwd        m2, m3
    REPX {pmaddwd x, m7}, m4, m2
    REPX {paddd   x, m14}, m4, m2
    REPX {psrad   x, 5}, m4, m2
    packssdw         m2, m4
    pminsw           m2, m15
    pmaxsw           m2, m9
    movu             m4, [grain_lutq+offxyq*2+16]
%if ARCH_X86_32
    movu             m3, [grain_lutq+r5*2+16]
%else
    movu             m3, [grain_lutq+top_offxyq*2+16]
%endif
    punpckhwd        m5, m3, m4
    punpcklwd        m3, m4
    REPX {pmaddwd x, m7}, m5, m3
    REPX {paddd   x, m14}, m5, m3
    REPX {psrad   x, 5}, m5, m3
    packssdw         m3, m5
    pminsw           m3, m15
    pmaxsw           m3, m9

    ; src
    pand             m0, m10, [srcq+ 0]          ; m0-1: src as word
    pand             m1, m10, [srcq+16]          ; m0-1: src as word

    ; scaling[src]
    ; noise = round2(scaling[src] * grain, scaling_shift)
%if ARCH_X86_32
    vpgatherdw       m4, m0, scalingq-1, r0, r5, 8, 1, m5
%else
    vpgatherdw       m4, m0, scalingq-1, r11, r13, 8, 1, m5
%endif
    psrlw            m4, 8
    pmullw           m4, m11
    pmulhrsw         m4, m2
%if ARCH_X86_32
    vpgatherdw       m5, m1, scalingq-1, r0, r5, 8, 1, m2
%else
    vpgatherdw       m5, m1, scalingq-1, r11, r13, 8, 1, m2
%endif
    psrlw            m5, 8
    pmullw           m5, m11
    pmulhrsw         m5, m3

    ; dst = clip_pixel(src, noise)
    paddw            m0, m4
    paddw            m1, m5
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova [dstq+srcq+ 0], m0
    mova [dstq+srcq+16], m1

    add            srcq, r2mp
    add      grain_lutq, 82*2
    dec              hw
    jz .end_y_v_overlap
    ; 2 lines get vertical overlap, then fall back to non-overlap code for
    ; remaining (up to) 30 lines
%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m7, [PIC_ptr(pw_27_17_17_27)+4]
    xor              hd, 0x10000
    test             hd, 0x10000
    jnz .loop_y_v_overlap
    jmp .loop_y

.end_y_v_overlap:
%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end_hv
%if ARCH_X86_32
    mov            srcq, r9mp
    add            srcq, r4mp
    add            srcq, r4mp
%else
    mov        src_bakq, r9mp
    lea            srcq, [src_bakq+wq*2]
%endif
    btc       dword r8m, 2
    jc .next_blk_v
%if ARCH_X86_32
    add dword [rsp+8*mmsize+1*gprsize], 16
%else
    add      top_offxyd, 16
%endif
    add          offxyd, 16
    jmp .loop_x_odd_v_overlap

.next_blk_v:
    ; since fg_dataq.overlap is guaranteed to be set, we never jump
    ; back to .loop_x_v_overlap, and instead always fall-through to
    ; h+v overlap

.loop_x_hv_overlap:
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, see, w, picptr, src_bak

    mov              r0, [rsp+8*mmsize+1*gprsize]
    add              r3, 16
    add              r0, 16
    mov [rsp+8*mmsize+0*gprsize], r3 ; left_offxy
    mov [rsp+8*mmsize+2*gprsize], r0 ; topleft_offxy

    mov            seed, r3m
    xor              r0, r0
%else
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
%endif
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            t0b                     ; parity of top_seed
    shr            seed, 16
    shl             t0d, 16
    test           seeb, seeh
    setp            t0b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             t0d, r6d
    mov            seed, t0d
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS  dst, src, scaling, offy, w, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, src_bak, left_offxy, top_offxy, topleft_offxy

    lea  topleft_offxyq, [top_offxyq+16]
    lea     left_offxyq, [offyq+16]
    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    ror           offxd, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyq, [offyq+offxq*2+0x10001*747+32*82]

%if ARCH_X86_32
    DEFINE_ARGS top_offxy, src, scaling, offxy, w, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, src_bak, left_offxy, top_offxy, topleft_offxy
%endif

    movzx    top_offxyd, offxyw
%if ARCH_X86_32
    mov [rsp+8*mmsize+1*gprsize], top_offxyd

    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif
    shr          offxyd, 16

%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m7, [PIC_ptr(pw_27_17_17_27)]

    movzx            hd, word r7m
    mov      grain_lutq, grain_lutmp
.loop_y_hv_overlap:
    ; grain = grain_lut[offy+y][offx+x]
    movu             m2, [grain_lutq+offxyq*2]
%if ARCH_X86_32
    mov              r0, [rsp+8*mmsize+1*gprsize] ; top_offxy
    mov              r5, [rsp+8*mmsize+0*gprsize] ; left_offxy
    movu             m4, [grain_lutq+r0*2]
    movd             m5, [grain_lutq+r5*2]
    mov              r5, [rsp+8*mmsize+2*gprsize] ; topleft_offxy
    movd             m3, [grain_lutq+r5*2]
%else
    movu             m4, [grain_lutq+top_offxyq*2]
    movd             m5, [grain_lutq+left_offxyq*2]
    movd             m3, [grain_lutq+topleft_offxyq*2]
%endif
    ; do h interpolation first (so top | top/left -> top, left | cur -> cur)
    punpcklwd        m5, m2
    punpcklwd        m3, m4
    REPX {pmaddwd x, m6}, m5, m3
    REPX {paddd   x, m14}, m5, m3
    REPX {psrad   x, 5}, m5, m3
    packssdw         m5, m3
    pminsw           m5, m15
    pmaxsw           m5, m9
    shufps           m3, m5, m2, q3210
    shufps           m5, m4, q3232
    ; followed by v interpolation (top | cur -> cur)
    movu             m0, [grain_lutq+offxyq*2+16]
%if ARCH_X86_32
    movu             m1, [grain_lutq+r0*2+16]
%else
    movu             m1, [grain_lutq+top_offxyq*2+16]
%endif
    punpcklwd        m2, m5, m3
    punpckhwd        m5, m3
    punpcklwd        m3, m1, m0
    punpckhwd        m1, m0
    REPX {pmaddwd x, m7}, m2, m5, m3, m1
    REPX {paddd   x, m14}, m2, m5, m3, m1
    REPX {psrad   x, 5}, m2, m5, m3, m1
    packssdw         m2, m5
    packssdw         m3, m1
    REPX {pminsw x, m15}, m2, m3
    REPX {pmaxsw x, m9}, m2, m3

    ; src
    pand             m0, m10, [srcq+ 0]
    pand             m1, m10, [srcq+16]          ; m0-1: src as word

    ; scaling[src]
    ; noise = round2(scaling[src] * grain, scaling_shift)
%if ARCH_X86_32
    vpgatherdw       m4, m0, scalingq-1, r0, r5, 8, 1, m5
%else
    vpgatherdw       m4, m0, scalingq-1, r14, r10, 8, 1, m5
%endif
    psrlw            m4, 8
    pmullw           m4, m11
    pmulhrsw         m2, m4
%if ARCH_X86_32
    vpgatherdw       m5, m1, scalingq-1, r0, r5, 8, 1, m4
%else
    vpgatherdw       m5, m1, scalingq-1, r14, r10, 8, 1, m4
%endif
    psrlw            m5, 8
    pmullw           m5, m11
    pmulhrsw         m3, m5

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova [dstq+srcq+ 0], m0
    mova [dstq+srcq+16], m1

    add            srcq, r2mp
    add      grain_lutq, 82*2
    dec              hw
    jz .end_y_hv_overlap
    ; 2 lines get vertical overlap, then fall back to non-overlap code for
    ; remaining (up to) 30 lines
%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m7, [PIC_ptr(pw_27_17_17_27)+4]
    xor              hd, 0x10000
    test             hd, 0x10000
    jnz .loop_y_hv_overlap
    jmp .loop_y_h_overlap

.end_y_hv_overlap:
    or        dword r8m, 4
%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end_hv
%if ARCH_X86_32
    mov              r5, r5m
    add          offxyd, 16
    add dword [rsp+8*mmsize+1*gprsize], 16 ; top_offxy += 16
    mov            srcq, r9mp
    add            srcq, r4mp
    add            srcq, r4mp
%else
    add          offxyd, 16
    add      top_offxyd, 16
    mov        src_bakq, r9mp
    lea            srcq, [src_bakq+wq*2]
%endif
    jmp .loop_x_odd_v_overlap

.end_hv:
    RET
%if ARCH_X86_32
    DECLARE_ARG 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
%endif

%macro FGUV_FN 3 ; name, ss_hor, ss_ver
INIT_XMM ssse3
%if ARCH_X86_32
%if STACK_ALIGNMENT < mmsize
cglobal fguv_32x32xn_i%1_16bpc, 0, 7, 8, 0-(8 * mmsize + 16 * gprsize), \
        tmp, src, scaling, h, fg_data, picptr, unused
    mov              r0, r0m
    mov              r1, r1m
    mov              r2, r2m
    mov              r4, r3m
    mov              r3, r4m
    mov              r5, r5m
%define r0m [rsp+8*mmsize+ 3*gprsize]
%define r1m [rsp+8*mmsize+ 4*gprsize]
%define r2m [rsp+8*mmsize+ 5*gprsize]
%define r3m [rsp+8*mmsize+ 6*gprsize]
%define r4m [rsp+8*mmsize+ 7*gprsize]
%define r5m [rsp+8*mmsize+ 8*gprsize]
    mov             r0m, r0
    mov             r2m, r2
    mov             r4m, r3
    mov             r5m, r5

    mov              r0, r6m
    mov              r2, r7m
    mov              r3, r8m
    mov              r5, r9m
%define r6m [rsp+8*mmsize+ 9*gprsize]
%define r7m [rsp+8*mmsize+10*gprsize]
%define r8m [rsp+8*mmsize+11*gprsize]
%define r9m [rsp+8*mmsize+12*gprsize]
    mov             r6m, r0
    mov             r7m, r2
    mov             r8m, r3
    mov             r9m, r5

    mov              r2, r10m
    mov              r3, r11m
    mov              r5, r12m
    mov              r0, r13m
%define r10m [rsp+8*mmsize+13*gprsize]
%define r11m [rsp+8*mmsize+14*gprsize]
%define r12m [rsp+8*mmsize+15*gprsize]
    mov            r10m, r2
    mov            r11m, r3
    mov            r12m, r5

    SPLATW           m2, r13m
%else
cglobal fguv_32x32xn_i%1_16bpc, 0, 7, 8, 8 * mmsize + (4) * gprsize, \
        tmp, src, scaling, h, fg_data, picptr, unused
    mov            srcq, srcm
    mov        fg_dataq, r3m
%endif
    LEA              r5, $$
%define base r5-$$

    DECLARE_REG_TMP   0, 2, 3
%else
cglobal fguv_32x32xn_i%1_16bpc, 6, 15, 16, dst, src, stride, fg_data, w, scaling, \
                                      grain_lut, h, sby, luma, lstride, uv_pl, is_id
%define base r8-pb_mask
    lea              r8, [pb_mask]

    DECLARE_REG_TMP   9, 10, 11
%endif
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    SPLATW           m3, [base+mul_bits+r6*2-14]
    mov             r6d, [fg_dataq+FGData.clip_to_restricted_range]
%if STACK_ALIGNMENT >= mmsize
    mov             t0d, r13m               ; bdmax
%endif
    sar             t0d, 11                 ; is_12bpc
    inc             t0d
    mov             t1d, r6d
    imul            t1d, t0d
    dec             t0d
    SPLATW           m5, [base+min+t1*2]
    lea             t1d, [t0d*3]
    mov             t2d, r12m
    inc             t2d
    imul            r6d, t2d
    add             t1d, r6d
    SPLATW           m4, [base+max+t1*2]
%if STACK_ALIGNMENT >= mmsize
    SPLATW           m2, r13m
%endif

    SCRATCH           2, 10, 2
    SCRATCH           3, 11, 3
    SCRATCH           4, 12, 4
    SCRATCH           5, 13, 5

%define mzero m7

%if %3
    SPLATD           m2, [base+pw_23_22]
%endif

%if ARCH_X86_32
    mov        scalingq, r5m
    mov             r5m, r5
%else
    mov           r13mp, strideq
%endif

    pcmpeqw          m0, m0
    psraw            m1, m10, 1
    pxor             m0, m1

    SCRATCH           0,  8, 0
    SCRATCH           1,  9, 1

    cmp byte [fg_dataq+FGData.chroma_scaling_from_luma], 0
    jne .csfl

%macro %%FGUV_32x32xN_LOOP 3 ; not-csfl, ss_h, ss_v
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, overlap

    DECLARE_REG_TMP    0
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, unused, sby, see, overlap

    DECLARE_REG_TMP    9
%endif

%if %1
    mov             r6d, r11m
    SPLATW           m0, [fg_dataq+FGData.uv_mult+r6*4]
    SPLATW           m1, [fg_dataq+FGData.uv_luma_mult+r6*4]
    punpcklwd        m6, m1, m0
    SPLATW           m5, [fg_dataq+FGData.uv_offset+r6*4]
    SPLATD           m7, [base+pw_4+t0*4]
    pmullw           m5, m7
%else
    SPLATD           m6, [base+pd_16]
%if %2
    mova             m5, [base+pw_23_22]
%else
    mova             m5, [base+pw_27_17_17_27]
%endif
%endif

    SCRATCH           6, 14, 6
    SCRATCH           5, 15, 7

%if ARCH_X86_32
    DECLARE_REG_TMP   0
%else
    DECLARE_REG_TMP   7
%endif

    mov            sbyd, r8m
    mov             t0d, [fg_dataq+FGData.overlap_flag]
    test            t0d, t0d
    jz %%no_vertical_overlap
    test           sbyd, sbyd
    jnz %%vertical_overlap

%%no_vertical_overlap:
    mov             r8m, t0d
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, see, fg_data, picptr, overlap
    imul           seed, (173 << 24) | 37
%else
    imul           seed, sbyd, (173 << 24) | 37
%endif
    add            seed, (105 << 24) | 178
    rol            seed, 8
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]
%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, see, w, picptr, luma

    mov            dstq, r0mp
    mov           lumaq, r9mp
    mov              wq, r4m
    lea              r3, [srcq+wq*2]
    mov            r1mp, r3
    lea              r3, [dstq+wq*2]
    mov           r11mp, r3
    lea              r3, [lumaq+wq*(2<<%2)]
    mov           r12mp, r3
%if %3
    shl           r10mp, 1
%endif
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                unused2, unused3, see, unused4, unused5, unused6, luma, lstride

    mov        lstrideq, r10mp
%if %3
    add        lstrideq, lstrideq
%endif
    mov           lumaq, r9mp
    lea             r10, [srcq+wq*2]
    lea             r11, [dstq+wq*2]
    lea             r12, [lumaq+wq*(2<<%2)]
    mov           r10mp, r10
    mov           r11mp, r11
    mov           r12mp, r12
%endif
    neg              wq
%if ARCH_X86_32
    mov           r4mp, wq
%endif

%%loop_x:
%if ARCH_X86_32
    mov            seed, r3m
%endif

    mov             r6d, seed
    or             seed, 0xEFF4
    shr             r6d, 1
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d               ; updated seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, w, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, unused1, unused2, unused3, luma, lstride

    mov           offxd, seed
    mov           offyd, seed
%endif
    ror           offyd, 8
    shr           offxd, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyq, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)]  ; offy*stride+offx

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, unused1, unused2, unused3, luma, lstride
%endif

%if %2 == 0
%%loop_x_odd:
%endif
    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%%loop_y:
    ; src
    mova             m0, [srcq]
    mova             m1, [srcq+16]          ; m0-1: src as word

    ; luma_src
    pxor          mzero, mzero
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, luma, grain_lut

    mov           lumaq, r9m
%endif
    mova             m4, [lumaq+ 0]
    mova             m6, [lumaq+(16<<%2)]
%if %2
    phaddw           m4, [lumaq+16]
    phaddw           m6, [lumaq+48]
%endif
%if ARCH_X86_32
    add           lumaq, r10mp
    mov             r9m, lumaq
%endif
%if %2
    pavgw            m4, mzero
    pavgw            m6, mzero
%endif

%if %1
    punpckhwd        m3, m4, m0
    punpcklwd        m4, m0
    punpckhwd        m5, m6, m1
    punpcklwd        m6, m1                 ; { luma, chroma }
    REPX {pmaddwd x, m14}, m3, m4, m5, m6
    REPX {psrad   x, 6}, m3, m4, m5, m6
    packssdw         m4, m3
    packssdw         m6, m5
    REPX {paddw x, m15}, m4, m6
    REPX {pmaxsw x, mzero}, m4, m6
    REPX {pminsw x, m10}, m4, m6             ; clip_pixel()
%else
    REPX  {pand x, m10}, m4, m6
%endif

    ; scaling[luma_src]
%if ARCH_X86_32
    vpgatherdw       m3, m4, scalingq-1, r0, r5, 8, 1
    vpgatherdw       m5, m6, scalingq-1, r0, r5, 8, 1
%else
    vpgatherdw       m3, m4, scalingq-1, r10, r12, 8, 1
    vpgatherdw       m5, m6, scalingq-1, r10, r12, 8, 1
%endif
    REPX   {psrlw x, 8}, m3, m5

    ; grain = grain_lut[offy+y][offx+x]
    movu             m4, [grain_lutq+offxyq*2]
    movu             m6, [grain_lutq+offxyq*2+16]

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    REPX {pmullw x, m11}, m3, m5
    pmulhrsw         m4, m3
    pmulhrsw         m6, m5

    ; dst = clip_pixel(src, noise)
    paddw            m0, m4
    paddw            m1, m6
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova      [dstq+ 0], m0
    mova      [dstq+16], m1

%if ARCH_X86_32
    add            srcq, r2mp
    add            dstq, r2mp
    mov           dstmp, dstq
%else
    add            srcq, r13mp
    add            dstq, r13mp
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82*2
    dec              hd
    jg %%loop_y

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, w, picptr, luma

    mov              wq, r4mp
%endif
    add              wq, 16
    jge %%end
%if ARCH_X86_32
    mov            srcq, r1mp
%else
    mov            srcq, r10mp
%endif
    mov            dstq, r11mp
    mov           lumaq, r12mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]
%if ARCH_X86_32
    mov             r0m, dstq
    mov             r9m, lumaq
    mov             r4m, wq
%endif
%if %2 == 0
    btc       dword r8m, 2
    jc %%next_blk
    add          offxyd, 16
    test      dword r8m, 2
    jz %%loop_x_odd
%if ARCH_X86_32
    add dword [rsp+8*mmsize+1*gprsize], 16
%else
    add            r11d, 16
%endif
    jmp %%loop_x_odd_v_overlap
%%next_blk:
%endif
    test      dword r8m, 1
    je %%loop_x

    ; r8m = sbym
    test      dword r8m, 2
    jnz %%loop_x_hv_overlap

    ; horizontal overlap (without vertical overlap)
%%loop_x_h_overlap:
%if ARCH_X86_32
    add          offxyd, 16
    mov [rsp+8*mmsize+0*gprsize], offxyd

    DEFINE_ARGS dst, src, scaling, see, w, picptr, grain_lut

    mov            seed, r3m
%endif
    mov             r6d, seed
    or             seed, 0xEFF4
    shr             r6d, 1
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d               ; updated seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, w, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, unused1, unused2, luma, lstride

    lea     left_offxyd, [offyd+16]         ; previous column's offy*stride+offx
    mov           offxd, seed
    mov           offyd, seed
%endif
    ror           offyd, 8
    shr           offxd, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyq, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)]  ; offy*stride+offx

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, unused1, unused2, luma, lstride
%endif

    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%%loop_y_h_overlap:
    mova             m0, [srcq]
    mova             m1, [srcq+16]

    ; luma_src
    pxor          mzero, mzero
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, luma, grain_lut
    mov           lumaq, r9m
%endif
    mova             m4, [lumaq+ 0]
    mova             m6, [lumaq+(16<<%2)]
%if %2
    phaddw           m4, [lumaq+16]
    phaddw           m6, [lumaq+48]
%endif
%if ARCH_X86_32
    add           lumaq, r10mp
    mov             r9m, lumaq
%endif
%if %2
    pavgw            m4, mzero
    pavgw            m6, mzero
%endif

%if %1
    punpckhwd        m3, m4, m0
    punpcklwd        m4, m0
    punpckhwd        m5, m6, m1
    punpcklwd        m6, m1                 ; { luma, chroma }
    REPX {pmaddwd x, m14}, m3, m4, m5, m6
    REPX {psrad   x, 6}, m3, m4, m5, m6
    packssdw         m4, m3
    packssdw         m6, m5
    REPX {paddw x, m15}, m4, m6
    REPX {pmaxsw x, mzero}, m4, m6
    REPX {pminsw x, m10}, m4, m6             ; clip_pixel()
%else
    REPX  {pand x, m10}, m4, m6
%endif

    ; grain = grain_lut[offy+y][offx+x]
    movu             m7, [grain_lutq+offxyq*2]
%if ARCH_X86_32
    mov              r5, [rsp+8*mmsize+0*gprsize]
    movd             m5, [grain_lutq+r5*2]
%else
    movd             m5, [grain_lutq+left_offxyq*2+ 0]
%endif
    punpcklwd        m5, m7                ; {left0, cur0}
%if %1
%if ARCH_X86_32
    mov              r5, r5m
%endif
%if %2
    pmaddwd          m5, [PIC_ptr(pw_23_22)]
%else
    pmaddwd          m5, [PIC_ptr(pw_27_17_17_27)]
%endif
    paddd            m5, [PIC_ptr(pd_16)]
%else
    pmaddwd          m5, m15
    paddd            m5, m14
%endif
    psrad            m5, 5
    packssdw         m5, m5
    pmaxsw           m5, m8
    pminsw           m5, m9
    shufps           m5, m7, q3210
    movu             m3, [grain_lutq+offxyq*2+16]

    ; scaling[luma_src]
%if ARCH_X86_32
    vpgatherdw       m7, m4, scalingq-1, r0, r5, 8, 1
    vpgatherdw       m4, m6, scalingq-1, r0, r5, 8, 1
%else
    vpgatherdw       m7, m4, scalingq-1, r2, r12, 8, 1
    vpgatherdw       m4, m6, scalingq-1, r2, r12, 8, 1
%endif
    REPX   {psrlw x, 8}, m7, m4

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    REPX {pmullw x, m11}, m7, m4
    pmulhrsw         m5, m7
    pmulhrsw         m3, m4

    ; dst = clip_pixel(src, noise)
    paddw            m0, m5
    paddw            m1, m3
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova      [dstq+ 0], m0
    mova      [dstq+16], m1

%if ARCH_X86_32
    add            srcq, r2mp
    add            dstq, r2mp
    mov           dstmp, dstq
%else
    add            srcq, r13mp
    add            dstq, r13mp
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82*2
    dec              hd
    jg %%loop_y_h_overlap

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, w, luma, grain_lut
    mov              wq, r4mp
%endif
    add              wq, 16
    jge %%end
%if ARCH_X86_32
    mov            srcq, r1mp
%else
    mov            srcq, r10mp
%endif
    mov            dstq, r11mp
    mov           lumaq, r12mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]
%if ARCH_X86_32
    mov            r0mp, dstq
    mov            r9mp, lumaq
    mov             r4m, wq
%endif

%if %2
    ; r8m = sbym
    test      dword r8m, 2
    jne %%loop_x_hv_overlap
    jmp %%loop_x_h_overlap
%else
    or        dword r8m, 4
    add          offxyd, 16

    ; r8m = sbym
    test      dword r8m, 2
    jz %%loop_x_odd
%if ARCH_X86_32
    add dword [rsp+8*mmsize+1*gprsize], 16
%else
    add            r11d, 16                 ; top_offxy += 16
%endif
    jmp %%loop_x_odd_v_overlap
%endif

%%end:
    RET

%%vertical_overlap:
    or              t0d, 2
    mov             r8m, t0d

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, overlap
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, unused, \
                sby, see, unused1, unused2, unused3, lstride
%endif

    movzx          sbyd, sbyb
%if ARCH_X86_32
    imul             r4, [fg_dataq+FGData.seed], 0x00010001

    DEFINE_ARGS tmp, src, scaling, sby, see, picptr, unused
%else
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
%endif
    imul            t0d, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add             t0d, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and             t0d, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, t0d
%if ARCH_X86_32
    xor            sbyd, seed

    DEFINE_ARGS dst, src, scaling, see, w, picptr, luma

    mov             r3m, seed
    mov            dstq, r0mp
    mov           lumaq, r9mp
    mov              wq, r4m
    lea              r3, [srcq+wq*2]
    mov            r1mp, r3
    lea              r3, [dstq+wq*2]
    mov           r11mp, r3
    lea              r3, [lumaq+wq*(2<<%2)]
    mov           r12mp, r3
%if %3
    shl           r10mp, 1
%endif
%else
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                unused1, unused2, see, unused3, unused4, unused5, luma, lstride

    mov        lstrideq, r10mp
%if %3
    add        lstrideq, lstrideq
%endif
    mov           lumaq, r9mp
    lea             r10, [srcq+wq*2]
    lea             r11, [dstq+wq*2]
    lea             r12, [lumaq+wq*(2<<%2)]
    mov           r10mp, r10
    mov           r11mp, r11
    mov           r12mp, r12
%endif
    neg              wq
%if ARCH_X86_32
    mov             r4m, wq
%endif

%%loop_x_v_overlap:
%if ARCH_X86_32
    mov            seed, r3m
    xor             t0d, t0d
%else
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
%endif
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            t0b                     ; parity of top_seed
    shr            seed, 16
    shl             t0d, 16
    test           seeb, seeh
    setp            t0b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             t0d, r6d
    mov            seed, t0d
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed
%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, w, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, unused1, top_offxy, unused2, luma, lstride

    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    ror           offxd, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyq, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

%if ARCH_X86_32
    DEFINE_ARGS top_offxy, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, unused1, top_offxy, unused2, luma, lstride
%endif
    movzx    top_offxyd, offxyw
%if ARCH_X86_32
    mov [rsp+8*mmsize+1*gprsize], top_offxyd
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif
    shr          offxyd, 16

%if %2 == 0
%%loop_x_odd_v_overlap:
%endif
%if %3 == 0
%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m2, [PIC_ptr(pw_27_17_17_27)]
%endif

    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%%loop_y_v_overlap:
    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq*2]
%if ARCH_X86_32
    mov              r0, [rsp+mmsize*8+gprsize*1] ; top_offxy
    movu             m5, [grain_lutq+r0*2]
%else
    movu             m5, [grain_lutq+top_offxyq*2]
%endif
    punpckhwd        m7, m5, m3
    punpcklwd        m5, m3                 ; {top/cur interleaved}
    REPX {pmaddwd x, m2}, m7, m5
%if %1
%if ARCH_X86_32
    mov              r5, r5m
%endif
    REPX  {paddd x, [PIC_ptr(pd_16)]}, m7, m5
%else
    REPX  {paddd x, m14}, m7, m5
%endif
    REPX   {psrad x, 5}, m7, m5
    packssdw         m3, m5, m7
    pmaxsw           m3, m8
    pminsw           m3, m9

    ; grain = grain_lut[offy+y][offx+x]
    movu             m4, [grain_lutq+offxyq*2+16]
%if ARCH_X86_32
    movu             m5, [grain_lutq+r0*2+16]
%else
    movu             m5, [grain_lutq+top_offxyq*2+16]
%endif
    punpckhwd        m7, m5, m4
    punpcklwd        m5, m4                 ; {top/cur interleaved}
    REPX {pmaddwd x, m2}, m7, m5
%if %1
    REPX  {paddd x, [PIC_ptr(pd_16)]}, m7, m5
%else
    REPX  {paddd x, m14}, m7, m5
%endif
    REPX   {psrad x, 5}, m7, m5
    packssdw         m4, m5, m7
    pmaxsw           m4, m8
    pminsw           m4, m9

    ; src
    mova             m0, [srcq]
    mova             m1, [srcq+16]

    ; luma_src
    pxor          mzero, mzero
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, luma, grain_lut

    mov           lumaq, r9mp
%endif
    mova             m5, [lumaq+ 0]
    mova             m6, [lumaq+(16<<%2)]
%if %2
    phaddw           m5, [lumaq+16]
    phaddw           m6, [lumaq+48]
%endif
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
%endif
%if %2
    pavgw            m5, mzero
    pavgw            m6, mzero
%endif

%if %1
    punpckhwd        m7, m5, m0
    punpcklwd        m5, m0
    REPX {pmaddwd x, m14}, m7, m5
    REPX {psrad   x, 6}, m7, m5
    packssdw         m5, m7
    punpckhwd        m7, m6, m1
    punpcklwd        m6, m1                 ; { luma, chroma }
    REPX {pmaddwd x, m14}, m7, m6
    REPX {psrad   x, 6}, m7, m6
    packssdw         m6, m7
    pxor          mzero, mzero
    REPX {paddw x, m15}, m5, m6
    REPX {pmaxsw x, mzero}, m5, m6
    REPX {pminsw x, m10}, m5, m6            ; clip_pixel()
%else
    REPX  {pand x, m10}, m5, m6
%endif

    ; scaling[luma_src]
%if ARCH_X86_32
    vpgatherdw       m7, m5, scalingq-1, r0, r5, 8, 1
    vpgatherdw       m5, m6, scalingq-1, r0, r5, 8, 1
%else
    vpgatherdw       m7, m5, scalingq-1, r10, r12, 8, 1
    vpgatherdw       m5, m6, scalingq-1, r10, r12, 8, 1
%endif
    REPX   {psrlw x, 8}, m7, m5

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    REPX {pmullw x, m11}, m7, m5
    pmulhrsw         m3, m7
    pmulhrsw         m4, m5

    ; dst = clip_pixel(src, noise)
    paddw            m0, m3
    paddw            m1, m4
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova      [dstq+ 0], m0
    mova      [dstq+16], m1

    dec              hw
    jle %%end_y_v_overlap
%if ARCH_X86_32
    add            srcq, r2mp
    add            dstq, r2mp
    mov           dstmp, dstq
%else
    add            srcq, r13mp
    add            dstq, r13mp
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82*2
%if %3
    jmp %%loop_y
%else
    btc              hd, 16
    jc %%loop_y
%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m2, [PIC_ptr(pw_27_17_17_27)+4]
    jmp %%loop_y_v_overlap
%endif

%%end_y_v_overlap:
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, w, luma, grain_lut

    mov              wq, r4m
%endif
    add              wq, 16
    jge %%end_hv
%if ARCH_X86_32
    mov            srcq, r1mp
%else
    mov            srcq, r10mp
%endif
    mov            dstq, r11mp
    mov           lumaq, r12mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]
%if ARCH_X86_32
    mov            r0mp, dstq
    mov            r9mp, lumaq
    mov             r4m, wq
%endif

%if %2
    ; since fg_dataq.overlap is guaranteed to be set, we never jump
    ; back to .loop_x_v_overlap, and instead always fall-through to
    ; h+v overlap
%else
    btc       dword r8m, 2
    jc %%loop_x_hv_overlap
    add          offxyd, 16
%if ARCH_X86_32
    add dword [rsp+8*mmsize+1*gprsize], 16
%else
    add            r11d, 16
%endif
    jmp %%loop_x_odd_v_overlap
%endif

%%loop_x_hv_overlap:
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, w, picptr, grain_lut

    mov             t0d, [rsp+mmsize*8+gprsize*1] ; top_offxy
    add          offxyd, 16
    add             t0d, 16
    mov [rsp+mmsize*8+gprsize*0], offxyd ; left_offxyd
    mov [rsp+mmsize*8+gprsize*2], t0d ; topleft_offxyd

    DEFINE_ARGS dst, src, scaling, see, w, picptr, grain_lut

    mov            seed, r3m
    xor             t0d, t0d
%else
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
%endif
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            t0b                     ; parity of top_seed
    shr            seed, 16
    shl             t0d, 16
    test           seeb, seeh
    setp            t0b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             t0d, r6d
    mov            seed, t0d
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed
%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, w, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, top_offxy, topleft_offxy, luma, lstride

    lea  topleft_offxyq, [top_offxyq+16]
    lea     left_offxyq, [offyq+16]
    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    ror           offxd, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyq, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, top_offxy
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, top_offxy, topleft_offxy, luma, lstride
%endif
    movzx    top_offxyd, offxyw
%if ARCH_X86_32
    mov [rsp+8*mmsize+1*gprsize], top_offxyd

    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif
    shr          offxyd, 16

%if %3 == 0
%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m2, [PIC_ptr(pw_27_17_17_27)]
%endif

    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%%loop_y_hv_overlap:
    ; grain = grain_lut[offy+y][offx+x]
%if ARCH_X86_32
    mov              r5, [rsp+8*mmsize+0*gprsize] ; left_offxy
    mov              r0, [rsp+8*mmsize+1*gprsize] ; top_offxy
    movd             m5, [grain_lutq+r5*2]
%else
    movd             m5, [grain_lutq+left_offxyq*2]
%endif
    movu             m7, [grain_lutq+offxyq*2]
%if ARCH_X86_32
    mov              r5, [rsp+8*mmsize+2*gprsize]
    movu             m4, [grain_lutq+r0*2]
%if %2
    pinsrw           m5, [grain_lutq+r5*2], 2
%else
    movd             m3, [grain_lutq+r5*2]
%endif
%else
    movu             m4, [grain_lutq+top_offxyq*2]
%if %2
    pinsrw           m5, [grain_lutq+topleft_offxyq*2], 2 ; { left, _, top/left }
%else
    movd             m3, [grain_lutq+topleft_offxyq*2]
%endif
%endif
%if %2 == 0
    punpckldq        m5, m3
%endif
    punpckldq        m3, m7, m4             ; { cur0/1,top0/1,cur2/3,top2/3 }
    punpcklwd        m5, m3                 ; { left/cur0,_/cur1,topleft/top0,_/top1 }
%if %1
%if ARCH_X86_32
    mov              r5, r5m
%endif
%if %2
    movddup          m0, [PIC_ptr(pw_23_22)]
%else
    movddup          m0, [PIC_ptr(pw_27_17_17_27)]
%endif
%else
    pshufd           m0, m15, q1010
%endif
    pmaddwd          m5, m0
%if %1
    paddd            m5, [PIC_ptr(pd_16)]
%else
    paddd            m5, m14
%endif
    psrad            m5, 5
    packssdw         m5, m5
    pmaxsw           m5, m8
    pminsw           m5, m9
    shufps           m5, m3, q3210          ; cur0/1,top0/1,cur2/3,top2/3
    shufps           m3, m5, m7, q3220      ; cur0-7 post-h_filter
    shufps           m5, m4, q3231          ; top0-7 post-h_filter

    punpckhwd        m7, m5, m3
    punpcklwd        m5, m3                 ; {top/cur interleaved}
    REPX {pmaddwd x, m2}, m7, m5
%if %1
    REPX  {paddd x, [PIC_ptr(pd_16)]}, m5, m7
%else
    REPX  {paddd x, m14}, m5, m7
%endif
    REPX   {psrad x, 5}, m5, m7
    packssdw         m3, m5, m7
    pmaxsw           m3, m8
    pminsw           m3, m9

    ; right half
    movu             m4, [grain_lutq+offxyq*2+16]
%if ARCH_X86_32
    movu             m0, [grain_lutq+r0*2+16]
%else
    movu             m0, [grain_lutq+top_offxyq*2+16]
%endif
    punpckhwd        m1, m0, m4
    punpcklwd        m0, m4                 ; {top/cur interleaved}
    REPX {pmaddwd x, m2}, m1, m0
%if %1
    REPX  {paddd x, [PIC_ptr(pd_16)]}, m1, m0
%else
    REPX  {paddd x, m14}, m1, m0
%endif
    REPX   {psrad x, 5}, m1, m0
    packssdw         m4, m0, m1
    pmaxsw           m4, m8
    pminsw           m4, m9

    ; src
    mova             m0, [srcq]
    mova             m1, [srcq+16]

    ; luma_src
    pxor          mzero, mzero
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, luma, grain_lut

    mov           lumaq, r9mp
%endif
    mova             m6, [lumaq+ 0]
    mova             m5, [lumaq+(16<<%2)]
%if %2
    phaddw           m6, [lumaq+16]
    phaddw           m5, [lumaq+48]
%endif
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
%endif
%if %2
    pavgw            m6, mzero
    pavgw            m5, mzero
%endif

%if %1
    punpckhwd        m7, m6, m0
    punpcklwd        m6, m0
    REPX {pmaddwd x, m14}, m7, m6
    REPX {psrad   x, 6}, m7, m6
    packssdw         m6, m7
    punpckhwd        m7, m5, m1
    punpcklwd        m5, m1                 ; { luma, chroma }
    REPX {pmaddwd x, m14}, m7, m5
    REPX {psrad   x, 6}, m7, m5
    packssdw         m5, m7
    pxor          mzero, mzero
    REPX {paddw x, m15}, m6, m5
    REPX {pmaxsw x, mzero}, m6, m5
    REPX {pminsw x, m10}, m6, m5            ; clip_pixel()
%else
    REPX  {pand x, m10}, m6, m5
%endif

    ; scaling[luma_src]
%if ARCH_X86_32
    vpgatherdw       m7, m6, scalingq-1, r0, r5, 8, 1
    vpgatherdw       m6, m5, scalingq-1, r0, r5, 8, 1
%else
%if %3 == 0
    ; register shortage :)
    push            r12
%endif
    vpgatherdw       m7, m6, scalingq-1, r2, r12, 8, 1
    vpgatherdw       m6, m5, scalingq-1, r2, r12, 8, 1
%if %3 == 0
    pop             r12
%endif
%endif
    REPX   {psrlw x, 8}, m7, m6

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    REPX {pmullw x, m11}, m7, m6
    pmulhrsw         m3, m7
    pmulhrsw         m4, m6

    ; dst = clip_pixel(src, noise)
    paddw            m0, m3
    paddw            m1, m4
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    movifnidn      dstq, dstmp
    mova      [dstq+ 0], m0
    mova      [dstq+16], m1

%if ARCH_X86_32
    add            srcq, r2mp
    add            dstq, r2mp
    mov           dstmp, dstq
%else
    add            srcq, r13mp
    add            dstq, r13mp
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82*2
    dec              hw
%if %3
    jg %%loop_y_h_overlap
%else
    jle %%end_y_hv_overlap
    btc              hd, 16
    jc %%loop_y_h_overlap
%if ARCH_X86_32
    mov              r5, r5m
%endif
    SPLATD           m2, [PIC_ptr(pw_27_17_17_27)+4]
    jmp %%loop_y_hv_overlap
%%end_y_hv_overlap:
%endif
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, w, luma, grain_lut

    mov              wq, r4m
%endif
    add              wq, 16
    jge %%end_hv
%if ARCH_X86_32
    mov            srcq, r1mp
%else
    mov            srcq, r10mp
%endif
    mov            dstq, r11mp
    mov           lumaq, r12mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]
%if ARCH_X86_32
    mov           dstmp, dstq
    mov            r9mp, lumaq
    mov             r4m, wq
%endif
%if %2
    jmp %%loop_x_hv_overlap
%else
    or        dword r8m, 4
    add          offxyd, 16
%if ARCH_X86_32
    add dword [rsp+8*mmsize+1*gprsize], 16
%else
    add            r11d, 16                 ; top_offxy += 16
%endif
    jmp %%loop_x_odd_v_overlap
%endif

%%end_hv:
    RET
%endmacro

    %%FGUV_32x32xN_LOOP 1, %2, %3
.csfl:
    %%FGUV_32x32xN_LOOP 0, %2, %3

%if STACK_ALIGNMENT < mmsize
DECLARE_ARG 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
%endif
%endmacro

FGUV_FN 420, 1, 1
FGUV_FN 422, 1, 0
FGUV_FN 444, 0, 0
