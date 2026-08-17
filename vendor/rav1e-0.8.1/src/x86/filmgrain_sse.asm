; Copyright © 2019-2021, VideoLAN and dav1d authors
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
%include "x86/filmgrain_common.asm"

SECTION_RODATA

pw_1024: times 8 dw 1024
pb_27_17_17_27: db 27, 17, 17, 27
                times 6 db 0, 32
pb_23_22_h: db 23, 22
            times 7 db 0, 32
pb_27_17: times 8 db 27, 17
pb_17_27: times 8 db 17, 27
pb_23_22: times 8 db 23, 22
pb_mask: db 0, 0x80, 0x80, 0, 0x80, 0, 0, 0x80, 0x80, 0, 0, 0x80, 0, 0x80, 0x80, 0
rnd_next_upperbit_mask: dw 0x100B, 0x2016, 0x402C, 0x8058
byte_blend: db 0, 0, 0, 0xff, 0, 0, 0, 0
pw_seed_xor: times 2 dw 0xb524
             times 2 dw 0x49d8
pb_1: times 4 db 1
hmul_bits: dw 32768, 16384, 8192, 4096
round: dw 2048, 1024, 512
mul_bits: dw 256, 128, 64, 32, 16
round_vals: dw 32, 64, 128, 256, 512
max: dw 255, 240, 235
min: dw 0, 16
pw_1: dw 1

%macro JMP_TABLE 2-*
    %xdefine %1_8bpc_%2_table %%table
    %xdefine %%base %1_8bpc_%2_table
    %xdefine %%prefix mangle(private_prefix %+ _%1_8bpc_%2)
    %%table:
    %rep %0 - 2
        dd %%prefix %+ .ar%3 - %%base
        %rotate 1
    %endrep
%endmacro

JMP_TABLE generate_grain_y, ssse3, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_420, ssse3, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_422, ssse3, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_444, ssse3, 0, 1, 2, 3

SECTION .text

%if ARCH_X86_32
%define PIC_ptr(a) base+a
%else
%define PIC_ptr(a) a
%endif

%macro SCRATCH 3
%if ARCH_X86_32
    mova [rsp+%3*mmsize], m%1
%define m%2 [rsp+%3*mmsize]
%else
    SWAP             %1, %2
%endif
%endmacro

INIT_XMM ssse3
cglobal generate_grain_y_8bpc, 2, 7 + 2 * ARCH_X86_64, 16, buf, fg_data
    LEA              r4, $$
%define base r4-$$
    movq             m1, [base+rnd_next_upperbit_mask]
    movq             m4, [base+mul_bits]
    movq             m7, [base+hmul_bits]
    mov             r2d, [fg_dataq+FGData.grain_scale_shift]
    movd             m2, [base+round+r2*2]
    movd             m0, [fg_dataq+FGData.seed]
    mova             m5, [base+pb_mask]
    pshuflw          m2, m2, q0000
    pshuflw          m0, m0, q0000
    mov              r2, -73*82
    sub            bufq, r2
    lea              r3, [base+gaussian_sequence]
.loop:
    pand             m6, m0, m1
    psrlw            m3, m6, 10
    por              m6, m3            ; bits 0xf, 0x1e, 0x3c and 0x78 are set
    pmullw           m6, m4            ; bits 0x0f00 are set
    pshufb           m3, m5, m6        ; set 15th bit for next 4 seeds
    psllq            m6, m3, 30
    por              m3, m6
    psllq            m6, m3, 15
    por              m3, m6            ; aggregate each bit into next seed's high bit
    pmulhuw          m6, m0, m7
    por              m3, m6            ; 4 next output seeds
    pshuflw          m0, m3, q3333
    psrlw            m3, 5
%if ARCH_X86_64
    movq             r6, m3
    mov              r8, r6
    movzx           r5d, r6w
    shr             r6d, 16
    shr              r8, 32
    movzx            r7, r8w
    shr              r8, 16

    movd             m6, [r3+r5*2]
    pinsrw           m6, [r3+r6*2], 1
    pinsrw           m6, [r3+r7*2], 2
    pinsrw           m6, [r3+r8*2], 3
%else
    movd             r6, m3
    pshuflw          m3, m3, q3232
    movzx            r5, r6w
    shr              r6, 16

    movd             m6, [r3+r5*2]
    pinsrw           m6, [r3+r6*2], 1

    movd             r6, m3
    movzx            r5, r6w
    shr              r6, 16

    pinsrw           m6, [r3+r5*2], 2
    pinsrw           m6, [r3+r6*2], 3
%endif
    pmulhrsw         m6, m2
    packsswb         m6, m6
    movd      [bufq+r2], m6
    add              r2, 4
    jl .loop

    ; auto-regression code
    movsxd           r2, [fg_dataq+FGData.ar_coeff_lag]
    movsxd           r2, [base+generate_grain_y_8bpc_ssse3_table+r2*4]
    lea              r2, [r2+base+generate_grain_y_8bpc_ssse3_table]
    jmp              r2

.ar1:
%if ARCH_X86_32
    DEFINE_ARGS buf, fg_data, cf3, unused, val3, min, max
%elif WIN64
    DEFINE_ARGS shift, fg_data, cf3, buf, val3, min, max, x, val0
    mov            bufq, r0
%else
    DEFINE_ARGS buf, fg_data, cf3, shift, val3, min, max, x, val0
%endif
    movsx          cf3d, byte [fg_dataq+FGData.ar_coeffs_y+3]
    movd             m4, [fg_dataq+FGData.ar_coeffs_y]
    mov             ecx, [fg_dataq+FGData.ar_coeff_shift]
%if ARCH_X86_32
    mov             r1m, cf3d
    DEFINE_ARGS buf, shift, val3, min, max, x, val0
%define hd r0mp
%define cf3d r1mp
%elif WIN64
    DEFINE_ARGS shift, h, cf3, buf, val3, min, max, x, val0
%else
    DEFINE_ARGS buf, h, cf3, shift, val3, min, max, x, val0
%endif
    pxor             m6, m6
    pcmpgtb          m7, m6, m4
    punpcklbw        m4, m7
    pinsrw           m4, [base+pw_1], 3
    pshufd           m5, m4, q1111
    pshufd           m4, m4, q0000
    movd             m3, [base+round_vals+shiftq*2-12]    ; rnd
    pshuflw          m3, m3, q0000
    sub            bufq, 82*73-(82*3+79)
    mov              hd, 70
    mov            mind, -128
    mov            maxd, 127
.y_loop_ar1:
    mov              xq, -76
    movsx         val3d, byte [bufq+xq-1]
.x_loop_ar1:
    movq             m0, [bufq+xq-82-1]     ; top/left
    pcmpgtb          m7, m6, m0
    punpcklbw        m0, m7
    psrldq           m2, m0, 2              ; top
    psrldq           m1, m0, 4              ; top/right
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
    movsx         val0d, byte [bufq+xq]
    add           val3d, val0d
    cmp           val3d, maxd
    cmovns        val3d, maxd
    cmp           val3d, mind
    cmovs         val3d, mind
    mov  byte [bufq+xq], val3b
    ; keep val3d in-place as left for next x iteration
    inc              xq
    jz .x_loop_ar1_end
    test             xq, 3
    jnz .x_loop_ar1_inner
    jmp .x_loop_ar1

.x_loop_ar1_end:
    add            bufq, 82
    dec              hd
    jg .y_loop_ar1
.ar0:
    RET

.ar2:
%if ARCH_X86_32
%assign stack_offset_old stack_offset
    ALLOC_STACK -16*8
%endif
    DEFINE_ARGS buf, fg_data, shift
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movd             m6, [base+round_vals-12+shiftq*2]
    movd             m7, [base+byte_blend+1]
    SCRATCH           7, 15, 7
    movq             m0, [fg_dataq+FGData.ar_coeffs_y+0]    ; cf0-7
    movd             m1, [fg_dataq+FGData.ar_coeffs_y+8]    ; cf8-11
    pxor             m7, m7
    pshuflw          m6, m6, q0000
    punpcklwd        m6, m7
    pcmpgtb          m4, m7, m0
    pcmpgtb          m5, m7, m1
    punpcklbw        m0, m4
    punpcklbw        m1, m5
    DEFINE_ARGS buf, fg_data, h, x
    pshufd           m4, m1, q0000
    pshufd           m5, m1, q1111
    pshufd           m3, m0, q3333
    pshufd           m2, m0, q2222
    pshufd           m1, m0, q1111
    pshufd           m0, m0, q0000
    SCRATCH           0, 8,  0
    SCRATCH           1, 9,  1
    SCRATCH           2, 10, 2
    SCRATCH           3, 11, 3
    SCRATCH           4, 12, 4
    SCRATCH           5, 13, 5
    SCRATCH           6, 14, 6
    sub            bufq, 82*73-(82*3+79)
    mov              hd, 70
.y_loop_ar2:
    mov              xq, -76

.x_loop_ar2:
    movq             m0, [bufq+xq-82*2-2]   ; y=-2,x=[-2,+5]
    movhps           m0, [bufq+xq-82*1-2]   ; y=-1,x=[-2,+5]
    pcmpgtb          m2, m7, m0
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2
    psrldq           m5, m0, 2              ; y=-2,x=[-1,+5]
    psrldq           m3, m1, 2              ; y=-1,x=[-1,+5]
    psrldq           m4, m1, 4              ; y=-1,x=[+0,+5]
    punpcklwd        m2, m0, m5
    punpcklwd        m3, m4
    pmaddwd          m2, m8
    pmaddwd          m3, m11
    paddd            m2, m3

    psrldq           m4, m0, 4              ; y=-2,x=[+0,+5]
    psrldq           m5, m0, 6              ; y=-2,x=[+1,+5]
    psrldq           m6, m0, 8              ; y=-2,x=[+2,+5]
    punpcklwd        m4, m5
    punpcklwd        m6, m1
    psrldq           m5, m1, 6              ; y=-1,x=[+1,+5]
    psrldq           m1, m1, 8              ; y=-1,x=[+2,+5]
    punpcklwd        m5, m1
    pmaddwd          m4, m9
    pmaddwd          m6, m10
    pmaddwd          m5, m12
    paddd            m4, m6
    paddd            m2, m5
    paddd            m2, m4
    paddd            m2, m14

    movq             m0, [bufq+xq-2]        ; y=0,x=[-2,+5]
.x_loop_ar2_inner:
    pcmpgtb          m4, m7, m0
    punpcklbw        m1, m0, m4
    pmaddwd          m3, m1, m13
    paddd            m3, m2
    psrldq           m1, 4                  ; y=0,x=0
    psrldq           m2, 4                  ; shift top to next pixel
    psrad            m3, [fg_dataq+FGData.ar_coeff_shift]
    ; don't packssdw since we only care about one value
    paddw            m3, m1
    packsswb         m3, m3
    pslldq           m3, 2
    pand             m3, m15
    pandn            m1, m15, m0
    por              m0, m1, m3
    psrldq           m0, 1
    ; overwrite 2 pixels, but that's ok
    movd      [bufq+xq-1], m0
    inc              xq
    jz .x_loop_ar2_end
    test             xq, 3
    jnz .x_loop_ar2_inner
    jmp .x_loop_ar2

.x_loop_ar2_end:
    add            bufq, 82
    dec              hd
    jg .y_loop_ar2
    RET

.ar3:
    DEFINE_ARGS buf, fg_data, shift
%if ARCH_X86_32
%assign stack_offset stack_offset_old
    ALLOC_STACK  -16*14
%elif WIN64
    SUB             rsp, 16*6
%assign stack_size_padded (stack_size_padded+16*6)
%assign stack_size (stack_size+16*6)
%else
    ALLOC_STACK  -16*6
%endif
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movd             m6, [base+round_vals-12+shiftq*2]
    movd             m7, [base+byte_blend]
    movu             m0, [fg_dataq+FGData.ar_coeffs_y+ 0]   ; cf0-15
    movq             m2, [fg_dataq+FGData.ar_coeffs_y+16]   ; cf16-23
    pxor             m3, m3
    pcmpgtb          m4, m3, m0
    pcmpgtb          m3, m2
    pshuflw          m6, m6, q0000
    SCRATCH           6, 14, 12
    SCRATCH           7, 15, 13
    punpckhbw        m1, m0, m4
    punpcklbw        m0, m4
    punpcklbw        m2, m3
    pshufd           m3, m0, q1111
    pshufd           m4, m0, q2222
    pshufd           m5, m0, q3333
    pshufd           m0, m0, q0000
    mova    [rsp+ 0*16], m0
    mova    [rsp+ 1*16], m3
    mova    [rsp+ 2*16], m4
    mova    [rsp+ 3*16], m5
    pshufd           m6, m1, q1111
    pshufd           m7, m1, q2222
    pshufd           m5, m1, q3333
    pshufd           m1, m1, q0000
    pshufd           m3, m2, q1111
    psrldq           m0, m2, 10
    pinsrw           m2, [base+pw_1], 5
    pshufd           m4, m2, q2222
    pshufd           m2, m2, q0000
    pinsrw           m0, [base+round_vals+shiftq*2-10], 3
    mova    [rsp+ 4*16], m1
    mova    [rsp+ 5*16], m6
    SCRATCH           7, 8,  6
    SCRATCH           5, 9,  7
    SCRATCH           2, 10, 8
    SCRATCH           3, 11, 9
    SCRATCH           4, 12, 10
    SCRATCH           0, 13, 11
    DEFINE_ARGS buf, fg_data, h, x
    sub            bufq, 82*73-(82*3+79)
    mov              hd, 70
.y_loop_ar3:
    mov              xq, -76

.x_loop_ar3:
    movu             m0, [bufq+xq-82*3-3]   ; y=-3,x=[-3,+12]
    pxor             m3, m3
    pcmpgtb          m3, m0
    punpckhbw        m2, m0, m3
    punpcklbw        m0, m3

    psrldq           m5, m0, 2
    psrldq           m6, m0, 4
    psrldq           m7, m0, 6
    punpcklwd        m4, m0, m5
    punpcklwd        m6, m7
    pmaddwd          m4, [rsp+ 0*16]
    pmaddwd          m6, [rsp+ 1*16]
    paddd            m4, m6

    movu             m1, [bufq+xq-82*2-3]   ; y=-2,x=[-3,+12]
    pxor             m5, m5
    pcmpgtb          m5, m1
    punpckhbw        m3, m1, m5
    punpcklbw        m1, m5
    palignr          m6, m2, m0, 10
    palignr          m7, m2, m0, 12
    psrldq           m0, 8
    punpcklwd        m0, m6
    punpcklwd        m7, m1
    pmaddwd          m0, [rsp+ 2*16]
    pmaddwd          m7, [rsp+ 3*16]
    paddd            m0, m7
    paddd            m0, m4

    psrldq           m4, m1, 2
    psrldq           m5, m1, 4
    psrldq           m6, m1, 6
    psrldq           m7, m1, 8
    punpcklwd        m4, m5
    punpcklwd        m6, m7
    pmaddwd          m4, [rsp+ 4*16]
    pmaddwd          m6, [rsp+ 5*16]
    paddd            m4, m6
    paddd            m0, m4

    movu             m2, [bufq+xq-82*1-3]   ; y=-1,x=[-3,+12]
    pxor             m7, m7
    pcmpgtb          m7, m2
    punpckhbw        m5, m2, m7
    punpcklbw        m2, m7
    palignr          m7, m3, m1, 10
    palignr          m3, m1, 12
    psrldq           m1, m2, 2
    punpcklwd        m7, m3
    punpcklwd        m3, m2, m1
    pmaddwd          m7, m8
    pmaddwd          m3, m9
    paddd            m7, m3
    paddd            m0, m7

    psrldq           m6, m2, 4
    psrldq           m1, m2, 6
    psrldq           m3, m2, 8
    palignr          m4, m5, m2, 10
    palignr          m5, m5, m2, 12

    punpcklwd        m6, m1
    punpcklwd        m3, m4
    punpcklwd        m5, m14
    pmaddwd          m6, m10
    pmaddwd          m3, m11
    pmaddwd          m5, m12
    paddd            m0, m6
    paddd            m3, m5
    paddd            m0, m3

    movq             m1, [bufq+xq-3]        ; y=0,x=[-3,+4]
.x_loop_ar3_inner:
    pxor             m5, m5
    pcmpgtb          m5, m1
    punpcklbw        m2, m1, m5
    pmaddwd          m2, m13
    pshufd           m3, m2, q1111
    paddd            m2, m3                 ; left+cur
    paddd            m2, m0                 ; add top
    psrldq           m0, 4
    psrad            m2, [fg_dataq+FGData.ar_coeff_shift]
    ; don't packssdw since we only care about one value
    packsswb         m2, m2
    pslldq           m2, 3
    pand             m2, m15
    pandn            m3, m15, m1
    por              m1, m2, m3
    movd    [bufq+xq-3], m1
    psrldq           m1, 1
    inc              xq
    jz .x_loop_ar3_end
    test             xq, 3
    jnz .x_loop_ar3_inner
    jmp .x_loop_ar3

.x_loop_ar3_end:
    add            bufq, 82
    dec              hd
    jg .y_loop_ar3
    RET

%macro generate_grain_uv_fn 3 ; ss_name, ss_x, ss_y
INIT_XMM ssse3
cglobal generate_grain_uv_%1_8bpc, 1, 7 + 3 * ARCH_X86_64, 16, buf, bufy, fg_data, uv
    movifnidn        r2, r2mp
    movifnidn        r3, r3mp
    LEA              r4, $$
%define base r4-$$
    movq             m1, [base+rnd_next_upperbit_mask]
    movq             m4, [base+mul_bits]
    movq             m7, [base+hmul_bits]
    mov             r5d, [fg_dataq+FGData.grain_scale_shift]
    movd             m6, [base+round+r5*2]
    mova             m5, [base+pb_mask]
    movd             m0, [fg_dataq+FGData.seed]
    movd             m2, [base+pw_seed_xor+uvq*4]
    pxor             m0, m2
    pshuflw          m6, m6, q0000
    pshuflw          m0, m0, q0000
    lea              r6, [base+gaussian_sequence]
%if %2
%if ARCH_X86_64
    mov             r7d, 73-35*%3
%else
    mov            r3mp, 73-35*%3
%endif
    add            bufq, 44
.loop_y:
    mov              r5, -44
.loop_x:
%else
    mov              r5, -82*73
    sub            bufq, r5
.loop:
%endif
    pand             m2, m0, m1
    psrlw            m3, m2, 10
    por              m2, m3             ; bits 0xf, 0x1e, 0x3c and 0x78 are set
    pmullw           m2, m4             ; bits 0x0f00 are set
    pshufb           m3, m5, m2         ; set 15th bit for next 4 seeds
    psllq            m2, m3, 30
    por              m3, m2
    psllq            m2, m3, 15
    por              m3, m2             ; aggregate each bit into next seed's high bit
    pmulhuw          m2, m0, m7
    por              m2, m3             ; 4 next output seeds
    pshuflw          m0, m2, q3333
    psrlw            m2, 5
%if ARCH_X86_64
    movd            r9d, m2
    pshuflw          m2, m2, q3232
    movzx            r8, r9w
    shr              r9, 16

    movd             m3, [r6+r8*2]
    pinsrw           m3, [r6+r9*2], 1

    movd            r9d, m2
    movzx            r8, r9w
    shr              r9, 16

    pinsrw           m3, [r6+r8*2], 2
    pinsrw           m3, [r6+r9*2], 3
%else
    movd             r2, m2
    pshuflw          m2, m2, q3232
    movzx            r1, r2w
    shr              r2, 16

    movd             m3, [r6+r1*2]
    pinsrw           m3, [r6+r2*2], 1

    movd             r2, m2
    movzx            r1, r2w
    shr              r2, 16

    pinsrw           m3, [r6+r1*2], 2
    pinsrw           m3, [r6+r2*2], 3
%endif
    pmulhrsw         m3, m6
    packsswb         m3, m3
    movd      [bufq+r5], m3
    add              r5, 4
%if %2
    jl .loop_x
    add            bufq, 82
%if ARCH_X86_64
    dec             r7d
%else
    dec            r3mp
%endif
    jg .loop_y
%else
    jl .loop
%endif

%if ARCH_X86_32
    mov              r2, r2mp
%endif

    ; auto-regression code
    movsxd           r5, [fg_dataq+FGData.ar_coeff_lag]
    movsxd           r5, [base+generate_grain_uv_%1_8bpc_ssse3_table+r5*4]
    lea              r5, [r5+base+generate_grain_uv_%1_8bpc_ssse3_table]
    jmp              r5

.ar0:
    DEFINE_ARGS buf, bufy, fg_data, uv, unused, shift
    movifnidn     bufyq, bufymp
%if ARCH_X86_32
%assign stack_offset_old stack_offset
    ALLOC_STACK   -2*16
%endif
    imul            uvd, 28
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movd             m5, [fg_dataq+FGData.ar_coeffs_uv+uvq]
    movd             m4, [base+hmul_bits+shiftq*2]
    DEFINE_ARGS buf, bufy, h, x
    pxor             m0, m0
    pcmpgtb          m0, m5
    punpcklbw        m5, m0
    movd             m7, [base+pb_1]
%if %2
    movd             m6, [base+hmul_bits+2+%3*2]
%endif
    pshuflw          m5, m5, q0000
    pshuflw          m4, m4, q0000
    pshufd           m7, m7, q0000
%if %2
    pshuflw          m6, m6, q0000
%endif
    punpcklqdq       m5, m5
    punpcklqdq       m4, m4
%if %2
    punpcklqdq       m6, m6
%endif
    pcmpeqw          m1, m1
    pslldq           m1, 12>>%2
    SCRATCH           1, 8, 0
    SCRATCH           4, 9, 1
%if %2
    sub            bufq, 82*(73-35*%3)+82-(82*3+41)
%else
    sub            bufq, 82*70-3
%endif
    add           bufyq, 3+82*3
    mov              hd, 70-35*%3
.y_loop_ar0:
    xor              xd, xd
.x_loop_ar0:
    ; first 32 pixels
%if %2
    movu             m1, [bufyq+xq*2]
%if %3
    movu             m2, [bufyq+xq*2+82]
%endif
    movu             m3, [bufyq+xq*2+16]
%if %3
    movu             m4, [bufyq+xq*2+82+16]
%endif
    pmaddubsw        m0, m7, m1
%if %3
    pmaddubsw        m1, m7, m2
%endif
    pmaddubsw        m2, m7, m3
%if %3
    pmaddubsw        m3, m7, m4
    paddw            m0, m1
    paddw            m2, m3
%endif
    pmulhrsw         m0, m6
    pmulhrsw         m2, m6
%else
    movu             m0, [bufyq+xq]
    pxor             m6, m6
    pcmpgtb          m6, m0
    punpckhbw        m2, m0, m6
    punpcklbw        m0, m6
%endif
    pmullw           m0, m5
    pmullw           m2, m5
    pmulhrsw         m0, m9
    pmulhrsw         m2, m9
    movu             m1, [bufq+xq]
    pxor             m4, m4
    pcmpgtb          m4, m1
    punpckhbw        m3, m1, m4
%if %2
    punpcklbw        m1, m4
    paddw            m2, m3
    paddw            m0, m1
%else
    punpcklbw        m6, m1, m4
    paddw            m2, m3
    paddw            m0, m6
%endif
    packsswb         m0, m2
%if %2
    movu      [bufq+xq], m0
    add              xd, 16
    cmp              xd, 32
    jl .x_loop_ar0

    ; last 6/12 pixels
    movu             m1, [bufyq+xq*(1+%2)]
%if %3
    movu             m2, [bufyq+xq*2+82]
%endif
    pmaddubsw        m0, m7, m1
%if %3
    pmaddubsw        m1, m7, m2
    paddw            m0, m1
%endif
    pmulhrsw         m0, m6
    pmullw           m0, m5
    pmulhrsw         m0, m9
    movq             m1, [bufq+xq]
    pxor             m4, m4
    pcmpgtb          m4, m1
    punpcklbw        m2, m1, m4
    paddw            m0, m2
    packsswb         m0, m0
    pandn            m2, m8, m0
    pand             m1, m8
    por              m2, m1
    movq      [bufq+xq], m2
%else
    add              xd, 16
    cmp              xd, 80
    je .y_loop_final_ar0
    movu   [bufq+xq-16], m0
    jmp .x_loop_ar0
.y_loop_final_ar0:
    pandn            m2, m8, m0
    pand             m1, m8
    por              m2, m1
    movu   [bufq+xq-16], m2
%endif

    add            bufq, 82
    add           bufyq, 82<<%3
    dec              hd
    jg .y_loop_ar0
    RET

.ar1:
%if ARCH_X86_32
%assign stack_offset stack_offset_old
%assign stack_size_padded 0
%xdefine rstk rsp
%endif
    DEFINE_ARGS buf, bufy, fg_data, uv, val3, cf3, min, max, x
    imul            uvd, 28
    movsx          cf3d, byte [fg_dataq+FGData.ar_coeffs_uv+uvq+3]
    movd             m4, [fg_dataq+FGData.ar_coeffs_uv+uvq-1]
    pinsrw           m4, [fg_dataq+FGData.ar_coeffs_uv+uvq+4], 2
%if ARCH_X86_32
    mov            r3mp, cf3d
    DEFINE_ARGS buf, shift, fg_data, val3, min, max, x
%elif WIN64
    DEFINE_ARGS shift, bufy, fg_data, buf, val3, cf3, min, max, x
    mov            bufq, r0
%else
    DEFINE_ARGS buf, bufy, fg_data, shift, val3, cf3, min, max, x
%endif
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movd             m3, [base+round_vals+shiftq*2-12]    ; rnd
%if %2
    movd             m7, [base+pb_1]
    movd             m6, [base+hmul_bits+2+%3*2]
%endif
    psrldq           m4, 1
%if ARCH_X86_32
    DEFINE_ARGS buf, shift, val0, val3, min, max, x
%elif WIN64
    DEFINE_ARGS shift, bufy, h, buf, val3, cf3, min, max, x, val0
%else
    DEFINE_ARGS buf, bufy, h, shift, val3, cf3, min, max, x, val0
%endif
    pxor             m5, m5
    punpcklwd        m3, m5
%if %2
    punpcklwd        m6, m6
%endif
    pcmpgtb          m5, m4
    punpcklbw        m4, m5
    pshufd           m5, m4, q1111
    pshufd           m4, m4, q0000
    pshufd           m3, m3, q0000
%if %2
    pshufd           m7, m7, q0000
    pshufd           m6, m6, q0000
    sub            bufq, 82*(73-35*%3)+44-(82*3+41)
%else
    sub            bufq, 82*69+3
%endif
%if ARCH_X86_32
    add            r1mp, 79+82*3
    mov            r0mp, 70-35*%3
%else
    add           bufyq, 79+82*3
    mov              hd, 70-35*%3
%endif
    mov            mind, -128
    mov            maxd, 127
.y_loop_ar1:
    mov              xq, -(76>>%2)
    movsx         val3d, byte [bufq+xq-1]
.x_loop_ar1:
%if %2
%if ARCH_X86_32
    mov              r2, r1mp
    movq             m0, [r2+xq*2]
%if %3
    movq             m1, [r2+xq*2+82]
%endif
%else
    movq             m0, [bufyq+xq*2]
%if %3
    movq             m1, [bufyq+xq*2+82]
%endif
%endif
    pmaddubsw        m2, m7, m0
%if %3
    pmaddubsw        m0, m7, m1
    paddw            m2, m0
%endif
    pmulhrsw         m2, m6
%else
%if ARCH_X86_32
    mov              r2, r1mp
    movd             m2, [r2+xq]
%else
    movd             m2, [bufyq+xq]
%endif
    pxor             m0, m0
    pcmpgtb          m0, m2
    punpcklbw        m2, m0
%endif

    movq             m0, [bufq+xq-82-1]     ; top/left
    pxor             m1, m1
    pcmpgtb          m1, m0
    punpcklbw        m0, m1
    psrldq           m1, m0, 4              ; top/right
    punpcklwd        m1, m2
    psrldq           m2, m0, 2              ; top
    punpcklwd        m0, m2
    pmaddwd          m0, m4
    pmaddwd          m1, m5
    paddd            m0, m1
    paddd            m0, m3
.x_loop_ar1_inner:
    movd          val0d, m0
    psrldq           m0, 4
%if ARCH_X86_32
    imul          val3d, r3mp
%else
    imul          val3d, cf3d
%endif
    add           val3d, val0d
    sar           val3d, shiftb
    movsx         val0d, byte [bufq+xq]
    add           val3d, val0d
    cmp           val3d, maxd
    cmovns        val3d, maxd
    cmp           val3d, mind
    cmovs         val3d, mind
    mov  byte [bufq+xq], val3b
    ; keep val3d in-place as left for next x iteration
    inc              xq
    jz .x_loop_ar1_end
    test             xq, 3
    jnz .x_loop_ar1_inner
    jmp .x_loop_ar1

.x_loop_ar1_end:
    add            bufq, 82
%if ARCH_X86_32
    add            r1mp, 82<<%3
    dec            r0mp
%else
    add           bufyq, 82<<%3
    dec              hd
%endif
    jg .y_loop_ar1
    RET

.ar2:
%if ARCH_X86_32
%assign stack_offset stack_offset_old
%assign stack_size_padded 0
%xdefine rstk rsp
    ALLOC_STACK   -8*16
%endif
    DEFINE_ARGS buf, bufy, fg_data, uv, unused, shift
    movifnidn     bufyq, bufymp
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    imul            uvd, 28
    movd             m7, [base+round_vals-12+shiftq*2]
    movu             m0, [fg_dataq+FGData.ar_coeffs_uv+uvq+0]   ; cf0-12
    pxor             m2, m2
    pcmpgtb          m2, m0
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2
    pinsrw           m1, [base+pw_1], 5
    punpcklwd        m7, m7
    pshufd           m7, m7, q0000
    DEFINE_ARGS buf, bufy, fg_data, h, unused, x
    pshufd           m4, m1, q0000
    pshufd           m5, m1, q1111
    pshufd           m6, m1, q2222
    pshufd           m3, m0, q3333
    pshufd           m2, m0, q2222
    pshufd           m1, m0, q1111
    pshufd           m0, m0, q0000
    SCRATCH           0, 8,  0
    SCRATCH           1, 9,  1
    SCRATCH           2, 10, 2
    SCRATCH           3, 11, 3
    SCRATCH           4, 12, 4
    SCRATCH           5, 13, 5
    SCRATCH           6, 14, 6
    SCRATCH           7, 15, 7
%if %2
    movd             m7, [base+hmul_bits+2+%3*2]
    movd             m6, [base+pb_1]
    punpcklwd        m7, m7
    pshufd           m6, m6, q0000
    pshufd           m7, m7, q0000
    sub            bufq, 82*(73-35*%3)+44-(82*3+41)
%else
    sub            bufq, 82*69+3
%endif
    add           bufyq, 79+82*3
    mov              hd, 70-35*%3
.y_loop_ar2:
    mov              xq, -(76>>%2)

.x_loop_ar2:
    pxor             m2, m2
    movq             m0, [bufq+xq-82*2-2]   ; y=-2,x=[-2,+5]
    movhps           m0, [bufq+xq-82*1-2]   ; y=-1,x=[-2,+5]
    pcmpgtb          m2, m0
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2
    psrldq           m5, m0, 2              ; y=-2,x=[-1,+5]
    psrldq           m3, m1, 2              ; y=-1,x=[-1,+5]
    psrldq           m4, m1, 4              ; y=-1,x=[+0,+5]
    punpcklwd        m2, m0, m5
    punpcklwd        m3, m4
    pmaddwd          m2, m8
    pmaddwd          m3, m11
    paddd            m2, m3

    psrldq           m4, m0, 4              ; y=-2,x=[+0,+5]
    psrldq           m5, m0, 6              ; y=-2,x=[+1,+5]
    psrldq           m0, 8                  ; y=-2,x=[+2,+5]
    punpcklwd        m4, m5
    punpcklwd        m0, m1
    psrldq           m3, m1, 6              ; y=-1,x=[+1,+5]
    psrldq           m1, m1, 8              ; y=-1,x=[+2,+5]
    punpcklwd        m3, m1
    pmaddwd          m4, m9
    pmaddwd          m0, m10
    pmaddwd          m3, m12
    paddd            m4, m0
    paddd            m2, m3
    paddd            m2, m4

%if %2
    movq             m1, [bufyq+xq*2]
%if %3
    movq             m3, [bufyq+xq*2+82]
%endif
    pmaddubsw        m0, m6, m1
%if %3
    pmaddubsw        m1, m6, m3
    paddw            m0, m1
%endif
    pmulhrsw         m0, m7
%else
    movd             m0, [bufyq+xq]
    pxor             m1, m1
    pcmpgtb          m1, m0
    punpcklbw        m0, m1
%endif
    punpcklwd        m0, m15
    pmaddwd          m0, m14
    paddd            m2, m0

    movq             m0, [bufq+xq-2]        ; y=0,x=[-2,+5]
    pxor             m4, m4
    movd             m5, [base+byte_blend+1]
    punpcklbw        m5, m5
.x_loop_ar2_inner:
    pcmpgtb          m1, m4, m0
    punpcklbw        m0, m1
    pmaddwd          m3, m0, m13
    paddd            m3, m2
    psrldq           m2, 4                  ; shift top to next pixel
    psrad            m3, [fg_dataq+FGData.ar_coeff_shift]
    pslldq           m3, 4
    pand             m3, m5
    paddw            m0, m3
    packsswb         m0, m0
    movd    [bufq+xq-2], m0
    psrldq           m0, 1
    inc              xq
    jz .x_loop_ar2_end
    test             xq, 3
    jnz .x_loop_ar2_inner
    jmp .x_loop_ar2

.x_loop_ar2_end:
    add            bufq, 82
    add           bufyq, 82<<%3
    dec              hd
    jg .y_loop_ar2
    RET

.ar3:
%if ARCH_X86_32
%assign stack_offset stack_offset_old
%assign stack_size_padded 0
%xdefine rstk rsp
%endif
    DEFINE_ARGS buf, bufy, fg_data, uv, unused, shift
    movifnidn     bufyq, bufymp
%if ARCH_X86_32
    ALLOC_STACK  -15*16
%else
    SUB             rsp, 16*7
%assign stack_size_padded (stack_size_padded+16*7)
%assign stack_size (stack_size+16*7)
%endif
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    imul            uvd, 28

    movu             m0, [fg_dataq+FGData.ar_coeffs_uv+uvq+ 0]   ; cf0-15
    pxor             m3, m3
    pcmpgtb          m3, m0
    punpckhbw        m1, m0, m3
    punpcklbw        m0, m3
    pshufd           m2, m0, q1111
    pshufd           m3, m0, q2222
    pshufd           m4, m0, q3333
    pshufd           m0, m0, q0000
    pshufd           m5, m1, q1111
    pshufd           m6, m1, q2222
    pshufd           m7, m1, q3333
    pshufd           m1, m1, q0000
    mova    [rsp+ 0*16], m0
    mova    [rsp+ 1*16], m2
    mova    [rsp+ 2*16], m3
    mova    [rsp+ 3*16], m4
    mova    [rsp+ 4*16], m1
    mova    [rsp+ 5*16], m5
    mova    [rsp+ 6*16], m6
    SCRATCH           7, 8, 7

    movu             m2, [fg_dataq+FGData.ar_coeffs_uv+uvq+16]   ; cf16-24 [24=luma]
    pxor             m4, m4
    pcmpgtb          m4, m2
    punpckhbw        m5, m2, m4
    punpcklbw        m2, m4
    pshufd           m4, m2, q3232
    punpcklwd        m3, m4, m5
    pshuflw          m5, m4, q3321
    pshufd           m4, m3, q0000
    pshufd           m3, m2, q1111
    pshufd           m2, m2, q0000
    pinsrw           m5, [base+round_vals+shiftq*2-10], 3
    SCRATCH           2, 9,  8
    SCRATCH           3, 10, 9
    SCRATCH           4, 11, 10
    SCRATCH           5, 12, 11

    movd             m2, [base+round_vals-12+shiftq*2]
%if %2
    movd             m1, [base+pb_1]
    movd             m3, [base+hmul_bits+2+%3*2]
%endif
    pxor             m0, m0
    punpcklwd        m2, m0
%if %2
    punpcklwd        m3, m3
%endif
    pshufd           m2, m2, q0000
%if %2
    pshufd           m1, m1, q0000
    pshufd           m3, m3, q0000
    SCRATCH           1, 13, 12
%endif
    SCRATCH           2, 14, 13
%if %2
    SCRATCH           3, 15, 14
%endif

    DEFINE_ARGS buf, bufy, fg_data, h, unused, x
%if %2
    sub            bufq, 82*(73-35*%3)+44-(82*3+41)
%else
    sub            bufq, 82*69+3
%endif
    add           bufyq, 79+82*3
    mov              hd, 70-35*%3
.y_loop_ar3:
    mov              xq, -(76>>%2)

.x_loop_ar3:
    movu             m0, [bufq+xq-82*3-3]   ; y=-3,x=[-3,+12]
    pxor             m4, m4
    pcmpgtb          m4, m0
    punpckhbw        m3, m0, m4
    punpcklbw        m0, m4

    psrldq           m5, m0, 2
    psrldq           m6, m0, 4
    psrldq           m7, m0, 6
    punpcklwd        m4, m0, m5
    punpcklwd        m6, m7
    pmaddwd          m4, [rsp+ 0*16]
    pmaddwd          m6, [rsp+ 1*16]
    paddd            m4, m6

    palignr          m2, m3, m0, 10
    palignr          m3, m0, 12
    psrldq           m0, 8

    movu             m1, [bufq+xq-82*2-3]   ; y=-2,x=[-3,+12]
    pxor             m6, m6
    pcmpgtb          m6, m1
    punpckhbw        m5, m1, m6
    punpcklbw        m1, m6

    punpcklwd        m0, m2
    punpcklwd        m3, m1
    pmaddwd          m0, [rsp+ 2*16]
    pmaddwd          m3, [rsp+ 3*16]
    paddd            m0, m3
    paddd            m0, m4

    movu             m2, [bufq+xq-82*1-3]   ; y=-1,x=[-3,+12]
    pxor             m7, m7
    pcmpgtb          m7, m2
    punpckhbw        m6, m2, m7
    punpcklbw        m2, m7

    palignr          m3, m5, m1, 10
    palignr          m5, m1, 12
    psrldq           m4, m2, 2

    punpcklwd        m3, m5
    punpcklwd        m5, m2, m4
    pmaddwd          m3, [rsp+ 6*16]
    pmaddwd          m5, m8
    paddd            m3, m5
    paddd            m0, m3

    psrldq           m3, m1, 2
    psrldq           m4, m1, 4
    psrldq           m5, m1, 6
    psrldq           m1, 8

    punpcklwd        m3, m4
    punpcklwd        m5, m1
    pmaddwd          m3, [rsp+ 4*16]
    pmaddwd          m5, [rsp+ 5*16]
    paddd            m3, m5
    paddd            m0, m3

%if %2
    movq             m1, [bufyq+xq*2]
%if %3
    movq             m3, [bufyq+xq*2+82]
%endif
    pmaddubsw        m7, m13, m1
%if %3
    pmaddubsw        m5, m13, m3
    paddw            m7, m5
%endif
    pmulhrsw         m7, m15
%else
    movd             m7, [bufyq+xq]
    pxor             m1, m1
    pcmpgtb          m1, m7
    punpcklbw        m7, m1
%endif

    psrldq           m1, m2, 4
    psrldq           m3, m2, 6
    palignr          m4, m6, m2, 10
    palignr          m6, m2, 12
    psrldq           m2, 8

    punpcklwd        m1, m3
    punpcklwd        m2, m4
    punpcklwd        m6, m7
    pmaddwd          m1, m9
    pmaddwd          m2, m10
    pmaddwd          m6, m11
    paddd            m1, m2
    paddd            m0, m6
    paddd            m0, m1
    paddd            m0, m14

    movq             m1, [bufq+xq-3]        ; y=0,x=[-3,+4]
    pxor             m4, m4
    movd             m5, [base+byte_blend]
.x_loop_ar3_inner:
    pcmpgtb          m2, m4, m1
    punpcklbw        m3, m1, m2
    pmaddwd          m2, m3, m12
    pshufd           m3, m2, q1111
    paddd            m2, m3                 ; left+cur
    paddd            m2, m0                 ; add top
    psrldq           m0, 4
    psrad            m2, [fg_dataq+FGData.ar_coeff_shift]
    ; don't packssdw, we only care about one value
    packsswb         m2, m2
    pandn            m3, m5, m1
    pslld            m2, 24
    pand             m2, m5
    por              m1, m2, m3
    movd    [bufq+xq-3], m1
    psrldq           m1, 1
    inc              xq
    jz .x_loop_ar3_end
    test             xq, 3
    jnz .x_loop_ar3_inner
    jmp .x_loop_ar3

.x_loop_ar3_end:
    add            bufq, 82
    add           bufyq, 82<<%3
    dec              hd
    jg .y_loop_ar3
    RET
%endmacro

generate_grain_uv_fn 420, 1, 1
generate_grain_uv_fn 422, 1, 0
generate_grain_uv_fn 444, 0, 0

%macro vpgatherdw 5-6 ; dst, src, base, tmp_gpr[x2], tmp_xmm_reg
%assign %%idx 0
%define %%tmp %2
%if %0 == 6
%define %%tmp %6
%endif
%rep 4
%if %%idx == 0
    movd        %5 %+ d, %2
    pshuflw       %%tmp, %2, q3232
%else
    movd        %5 %+ d, %%tmp
%if %%idx == 2
    punpckhqdq    %%tmp, %%tmp
%elif %%idx == 4
    psrlq         %%tmp, 32
%endif
%endif
    movzx       %4 %+ d, %5 %+ w
    shr         %5 %+ d, 16

%if %%idx == 0
    movd             %1, [%3+%4]
%else
    pinsrw           %1, [%3+%4], %%idx + 0
%endif
    pinsrw           %1, [%3+%5], %%idx + 1
%assign %%idx %%idx+2
%endrep
%endmacro

INIT_XMM ssse3
; fgy_32x32xn(dst, src, stride, fg_data, w, scaling, grain_lut, h, sby)
%if ARCH_X86_32
%if STACK_ALIGNMENT < mmsize
cglobal fgy_32x32xn_8bpc, 0, 7, 16, 0 - (5 * mmsize + 16 * gprsize), \
        dst, src, scaling, unused1, fg_data, picptr, unused2
    ; copy stack arguments to new position post-alignment, so that we
    ; don't have to keep the old stack location in a separate register
    mov              r0, r0m
    mov              r1, r2m
    mov              r2, r4m
    mov              r3, r6m
    mov              r4, r7m
    mov              r5, r8m

    mov [rsp+5*mmsize+ 4*gprsize], r0
    mov [rsp+5*mmsize+ 6*gprsize], r1
    mov [rsp+5*mmsize+ 8*gprsize], r2
    mov [rsp+5*mmsize+10*gprsize], r3
    mov [rsp+5*mmsize+11*gprsize], r4
    mov [rsp+5*mmsize+12*gprsize], r5
%else
cglobal fgy_32x32xn_8bpc, 0, 7, 16, 5 * mmsize + 4 * gprsize, \
        dst, src, scaling, unused1, fg_data, picptr, unused2
%endif
    mov            srcq, srcm
    mov        fg_dataq, r3m
    mov        scalingq, r5m
%if STACK_ALIGNMENT < mmsize
%define r0m [rsp+5*mmsize+ 4*gprsize]
%define r1m [rsp+5*mmsize+ 5*gprsize]
%define r2m [rsp+5*mmsize+ 6*gprsize]
%define r3m [rsp+5*mmsize+ 7*gprsize]
%define r4m [rsp+5*mmsize+ 8*gprsize]
%define r5m [rsp+5*mmsize+ 9*gprsize]
%define r6m [rsp+5*mmsize+10*gprsize]
%define r7m [rsp+5*mmsize+11*gprsize]
%define r8m [rsp+5*mmsize+12*gprsize]
%endif
    LEA              r5, pb_mask
%define base r5-pb_mask
    mov             r5m, picptrq
%else
cglobal fgy_32x32xn_8bpc, 6, 15, 16, dst, src, stride, fg_data, w, scaling, grain_lut
    lea              r7, [pb_mask]
%define base r7-pb_mask
%endif
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    movd             m3, [base+mul_bits+r6*2-14]
    mov             r6d, [fg_dataq+FGData.clip_to_restricted_range]
    movd             m4, [base+max+r6*4]
    movd             m5, [base+min+r6*2]
    punpcklwd        m3, m3
    punpcklwd        m4, m4
    punpcklwd        m5, m5
    pshufd           m3, m3, q0000
    pshufd           m4, m4, q0000
    pshufd           m5, m5, q0000
    SCRATCH           3, 11, 0
    SCRATCH           4, 12, 1
    SCRATCH           5, 13, 2

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, overlap
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, unused, sby, see, overlap
%endif

    mov            sbyd, r8m
    mov        overlapd, [fg_dataq+FGData.overlap_flag] ; left_overlap: overlap & 1
    test       overlapd, overlapd
    jz .no_vertical_overlap
    mova             m6, [base+pw_1024]
    mova             m7, [base+pb_27_17_17_27]
    SCRATCH           6, 14, 3
    SCRATCH           7, 15, 4
    test           sbyd, sbyd
    jnz .vertical_overlap
    ; fall-through

.no_vertical_overlap:
    mov             r8m, overlapd
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
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                unused1, unused2, see, unused3
%endif

    lea        src_bakq, [srcq+wq]
    neg              wq
    sub           dstmp, srcq
%if ARCH_X86_32
    mov             r1m, src_bakq
    mov             r4m, wq
    DEFINE_ARGS dst, src, scaling, see, unused1, unused2, unused3
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
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, unused

    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    shr           offxd, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offyq, [offyq+offxq*2+747] ; offy*stride+offx

%if ARCH_X86_32
    ; r0m=dst, r1m=src_bak, r2m=stride, r3m=see, r4m=w, r5m=picptr,
    ; r6m=grain_lut, r7m=h, r8m=overlap_v|h
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, unused
%endif

.loop_x_odd:
    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
.loop_y:
    ; src
    mova             m0, [srcq]
    pxor             m2, m2
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2                 ; m0-1: src as word

    ; scaling[src]
%if ARCH_X86_32
    vpgatherdw       m4, m0, scalingq-1, r0, r5, m3
    vpgatherdw       m5, m1, scalingq-1, r0, r5, m3
%else
    vpgatherdw       m4, m0, scalingq-1, r12, r13, m3
    vpgatherdw       m5, m1, scalingq-1, r12, r13, m3
%endif
    REPX {psrlw x, 8}, m4, m5

    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq]
    pcmpgtb          m7, m2, m3
    punpcklbw        m2, m3, m7
    punpckhbw        m3, m7

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmullw           m2, m4
    pmullw           m3, m5
    pmulhrsw         m2, m11
    pmulhrsw         m3, m11

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    packuswb         m0, m1
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

    add            srcq, r2mp
    add      grain_lutq, 82
    dec              hd
    jg .loop_y

%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end
%if ARCH_X86_32
    mov            srcq, r1mp
    add            srcq, r4mp
%else
    lea            srcq, [src_bakq+wq]
%endif
    btc       dword r8m, 2
    jc .next_blk

    add          offxyd, 16
    test      dword r8m, 2              ; r8m & 2 = have_top_overlap
    jz .loop_x_odd

%if ARCH_X86_32
    add dword [rsp+5*mmsize+1*gprsize], 16
%else
    add            r11d, 16             ; top_offxyd
%endif
    jnz .loop_x_odd_v_overlap

.next_blk:
    test      dword r8m, 1
    jz .loop_x

    test      dword r8m, 2
    jnz .loop_x_hv_overlap

    ; horizontal overlap (without vertical overlap)
.loop_x_h_overlap:
%if ARCH_X86_32
    ; r0m=dst, r1m=src_bak, r2m=stride, r3m=see, r4m=w, r5m=picptr,
    ; r6m=grain_lut, r7m=h, r8m=overlap_v|h
    DEFINE_ARGS dst, src, scaling, offxy, unused1, unused2, unused3

    add          offxyd, 16                 ; left_offxyd
    mov [rsp+5*mmsize+0*gprsize], offxyd

    DEFINE_ARGS dst, src, scaling, see, unused1, unused2, unused3

    mov            seed, r3m
%else
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, left_offxy

    lea     left_offxyd, [offyd+16]         ; previous column's offy*stride+offx
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
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, left_offxy
%endif

    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
.loop_y_h_overlap:
    ; src
    mova             m0, [srcq]
    pxor             m2, m2
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2                 ; m0-1: src as word

    ; scaling[src]
%if ARCH_X86_32
    vpgatherdw       m4, m0, scalingq-1, r0, r5, m3
    vpgatherdw       m5, m1, scalingq-1, r0, r5, m3
%else
    vpgatherdw       m4, m0, scalingq-1, r12, r13, m3
    vpgatherdw       m5, m1, scalingq-1, r12, r13, m3
%endif
    REPX {psrlw x, 8}, m4, m5

    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq]
%if ARCH_X86_32
    mov              r5, [rsp+5*mmsize+0*gprsize]
    movd             m7, [grain_lutq+r5]
%else
    movd             m7, [grain_lutq+left_offxyq]
%endif
    punpcklbw        m7, m3
    pmaddubsw        m6, m15, m7
    pmulhrsw         m6, m14
    packsswb         m6, m6
    shufps           m6, m3, q3210
    pcmpgtb          m2, m6
    punpcklbw        m7, m6, m2
    punpckhbw        m6, m2

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmullw           m7, m4
    pmullw           m6, m5
    pmulhrsw         m7, m11
    pmulhrsw         m6, m11

    ; dst = clip_pixel(src, noise)
    paddw            m0, m7
    paddw            m1, m6
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    packuswb         m0, m1
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

    add            srcq, r2mp
    add      grain_lutq, 82
    dec              hd
    jg .loop_y_h_overlap

%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end
%if ARCH_X86_32
    mov            srcq, r1m
    add            srcq, r4m
%else
    lea            srcq, [src_bakq+wq]
%endif
    xor       dword r8m, 4
    add          offxyd, 16

    ; since this half-block had left-overlap, the next does not
    test      dword r8m, 2              ; have_top_overlap
    jz .loop_x_odd
%if ARCH_X86_32
    add dword [rsp+5*mmsize+1*gprsize], 16
%else
    add            r11d, 16             ; top_offxyd
%endif
    jmp .loop_x_odd_v_overlap

.end:
    RET

.vertical_overlap:
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, overlap
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, tmp, sby, see, overlap
%endif

    or         overlapd, 2                  ; top_overlap: overlap & 2
    mov             r8m, overlapd
    movzx          sbyd, sbyb
%if ARCH_X86_32
    imul             r4, [fg_dataq+FGData.seed], 0x00010001
    DEFINE_ARGS tmp, src, scaling, sby, see, picptr, unused
%else
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
%endif
    imul           tmpd, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add            tmpd, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and            tmpd, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, tmpd
%if ARCH_X86_32
    xor            sbyd, seed               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, scaling, see, w, picptr, src_bak

    mov             r3m, seed
    mov              wq, r4m
%else
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                tmp, unused2, see, unused3
%endif

    lea        src_bakq, [srcq+wq]
    neg              wq
    sub           dstmp, srcq
%if ARCH_X86_32
    mov             r1m, src_bakq
    mov             r4m, wq
    DEFINE_ARGS tmp, src, scaling, see, unused1, picptr, unused2
%endif

.loop_x_v_overlap:
%if ARCH_X86_32
    mov            seed, r3m
%endif
    ; we assume from the block above that bits 8-15 of tmpd are zero'ed,
    ; because of the 'and tmpd, 0x00ff00ff' above
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp           tmpb                     ; parity of top_seed
    shr            seed, 16
    shl            tmpd, 16
    test           seeb, seeh
    setp           tmpb                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor            tmpd, r6d
    mov            seed, tmpd
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, unused1, unused2, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, unused, top_offxy

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
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, unused, top_offxy
%endif

    movzx    top_offxyd, offxyw
%if ARCH_X86_32
    mov [rsp+5*mmsize+1*gprsize], top_offxyd

    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif
    shr          offxyd, 16

.loop_x_odd_v_overlap:
%if ARCH_X86_32
    mov              r5, r5m
    lea              r5, [base+pb_27_17]
    mov [rsp+5*mmsize+12], r5
%else
    mova             m8, [pb_27_17]
%endif
    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
.loop_y_v_overlap:
    ; src
    mova             m0, [srcq]
    pxor             m2, m2
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2                 ; m0-1: src as word

    ; scaling[src]
%if ARCH_X86_32
    vpgatherdw       m4, m0, scalingq-1, r0, r5, m3
    vpgatherdw       m5, m1, scalingq-1, r0, r5, m3
%else
    vpgatherdw       m4, m0, scalingq-1, r12, r13, m3
    vpgatherdw       m5, m1, scalingq-1, r12, r13, m3
%endif
    REPX {psrlw x, 8}, m4, m5

    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq]
%if ARCH_X86_32
    mov              r5, [rsp+5*mmsize+1*gprsize]
    movu             m7, [grain_lutq+r5]
%else
    movu             m7, [grain_lutq+top_offxyq]
%endif
    punpckhbw        m6, m7, m3
    punpcklbw        m7, m3
%if ARCH_X86_32
    mov              r5, [rsp+5*mmsize+12]
    pmaddubsw        m3, [r5], m6
    pmaddubsw        m6, [r5], m7
%else
    pmaddubsw        m3, m8, m6
    pmaddubsw        m6, m8, m7
%endif
    pmulhrsw         m3, m14
    pmulhrsw         m6, m14
    packsswb         m6, m3
    pcmpgtb          m7, m2, m6
    punpcklbw        m2, m6, m7
    punpckhbw        m6, m7

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmullw           m2, m4
    pmullw           m6, m5
    pmulhrsw         m2, m11
    pmulhrsw         m6, m11

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m6
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    packuswb         m0, m1
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

%if ARCH_X86_32
    add dword [rsp+5*mmsize+12], mmsize
%else
    mova             m8, [pb_17_27]
%endif
    add            srcq, r2mp
    add      grain_lutq, 82
    dec              hw
    jz .end_y_v_overlap
    ; 2 lines get vertical overlap, then fall back to non-overlap code for
    ; remaining (up to) 30 lines
    btc              hd, 16
    jnc .loop_y_v_overlap
    jmp .loop_y

.end_y_v_overlap:
%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end_hv
%if ARCH_X86_32
    mov            srcq, r1mp
    add            srcq, r4mp
%else
    lea            srcq, [src_bakq+wq]
%endif
    btc       dword r8m, 2
    jc .loop_x_hv_overlap
    add          offxyd, 16
%if ARCH_X86_32
    add dword [rsp+5*mmsize+1*gprsize], 16
%else
    add      top_offxyd, 16
%endif
    jmp .loop_x_odd_v_overlap

.loop_x_hv_overlap:
%if ARCH_X86_32
    mov              r5, r5m
    lea              r5, [base+pb_27_17]
    mov [rsp+5*mmsize+12], r5

    DEFINE_ARGS tmp, src, scaling, offxy, w, picptr, src_bak

    mov              r5, [rsp+5*mmsize+1*gprsize]
    mov              r4, offxyd
    add              r5, 16
    add              r4, 16
    mov [rsp+5*mmsize+2*gprsize], r5        ; topleft_offxy
    mov [rsp+5*mmsize+0*gprsize], r4        ; left_offxy

    DEFINE_ARGS tmp, src, scaling, see, w, picptr, src_bak

    xor            tmpd, tmpd
    mov            seed, r3m
%else
    mova             m8, [pb_27_17]

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                tmp, unused2, see, unused3

    ; we assume from the block above that bits 8-15 of tmpd are zero'ed
%endif
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp           tmpb                     ; parity of top_seed
    shr            seed, 16
    shl            tmpd, 16
    test           seeb, seeh
    setp           tmpb                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor            tmpd, r6d
    mov            seed, tmpd
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, unused1, unused2, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, top_offxy, topleft_offxy

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
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut

    movzx            r5, offxyw             ; top_offxy
    mov [rsp+5*mmsize+1*gprsize], r5
%else
    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, top_offxy, topleft_offxy

    movzx    top_offxyd, offxyw
%endif
    shr          offxyd, 16

    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
.loop_y_hv_overlap:
    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq]
%if ARCH_X86_32
    mov              r5, [rsp+5*mmsize+1*gprsize]   ; top_offxy
    mov              r0, [rsp+5*mmsize+0*gprsize]   ; left_offxy
    movu             m6, [grain_lutq+r5]
    mov              r5, [rsp+5*mmsize+2*gprsize]   ; topleft_offxy
    movd             m4, [grain_lutq+r0]
    movd             m7, [grain_lutq+r5]
%else
    movu             m6, [grain_lutq+top_offxyq]
    movd             m4, [grain_lutq+left_offxyq]
    movd             m7, [grain_lutq+topleft_offxyq]
%endif
    ; do h interpolation first (so top | top/left -> top, left | cur -> cur)
    punpcklbw        m4, m3
    punpcklbw        m7, m6
    pmaddubsw        m2, m15, m4
    pmaddubsw        m4, m15, m7
    pmulhrsw         m2, m14
    pmulhrsw         m4, m14
    packsswb         m2, m2
    packsswb         m4, m4
    shufps           m2, m3, q3210
    shufps           m4, m6, q3210
    ; followed by v interpolation (top | cur -> cur)
    punpcklbw        m3, m4, m2
    punpckhbw        m4, m2
%if ARCH_X86_32
    mov              r5, [rsp+5*mmsize+12]
    pmaddubsw        m7, [r5], m4
    pmaddubsw        m4, [r5], m3
%else
    pmaddubsw        m7, m8, m4
    pmaddubsw        m4, m8, m3
%endif
    pmulhrsw         m7, m14
    pmulhrsw         m4, m14
    packsswb         m4, m7
    pxor             m2, m2
    pcmpgtb          m7, m2, m4
    punpcklbw        m3, m4, m7
    punpckhbw        m4, m7

    ; src
    mova             m0, [srcq]
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2                 ; m0-1: src as word

    ; scaling[src]
%if ARCH_X86_32
    vpgatherdw       m5, m0, scalingq-1, r0, r5, m7
    vpgatherdw       m6, m1, scalingq-1, r0, r5, m7
%else
    vpgatherdw       m5, m0, scalingq-1, r13, r14, m7
    vpgatherdw       m6, m1, scalingq-1, r13, r14, m7
%endif
    REPX {psrlw x, 8}, m5, m6

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmullw           m3, m5
    pmullw           m4, m6
    pmulhrsw         m3, m11
    pmulhrsw         m4, m11

    ; dst = clip_pixel(src, noise)
    paddw            m0, m3
    paddw            m1, m4
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    packuswb         m0, m1
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

%if ARCH_X86_32
    add dword [rsp+5*mmsize+12], mmsize
%else
    mova             m8, [pb_17_27]
%endif
    add            srcq, r2mp
    add      grain_lutq, 82
    dec              hw
    jz .end_y_hv_overlap
    ; 2 lines get vertical overlap, then fall back to non-overlap code for
    ; remaining (up to) 30 lines
    btc              hd, 16
    jnc .loop_y_hv_overlap
    jmp .loop_y_h_overlap

.end_y_hv_overlap:
%if ARCH_X86_32
    add            r4mp, 16
%else
    add              wq, 16
%endif
    jge .end_hv
%if ARCH_X86_32
    mov            srcq, r1m
    add            srcq, r4m
%else
    lea            srcq, [src_bakq+wq]
%endif
    xor       dword r8m, 4
    add          offxyd, 16
%if ARCH_X86_32
    add dword [rsp+5*mmsize+1*gprsize], 16
%else
    add      top_offxyd, 16
%endif
    jmp .loop_x_odd_v_overlap

.end_hv:
    RET

%macro FGUV_FN 3 ; name, ss_hor, ss_ver
INIT_XMM ssse3
%if ARCH_X86_32
; fguv_32x32xn_i420_ssse3(dst, src, stride, fg_data, w, scaling, grain_lut, h,
;                         sby, luma, lstride, uv_pl, is_id)
%if STACK_ALIGNMENT < mmsize
DECLARE_ARG 0, 1, 2, 3, 4, 5, 6, 7, 8
cglobal fguv_32x32xn_i%1_8bpc, 0, 7, 8, 0 - (7 * mmsize + (13 + 3) * gprsize), \
        tmp, src, scaling, h, fg_data, picptr, unused
    mov              r0, r0m
    mov              r1, r2m
    mov              r2, r4m
    mov              r3, r6m
    mov              r4, r7m
    mov [rsp+7*mmsize+3*gprsize], r0
    mov [rsp+7*mmsize+5*gprsize], r1
    mov [rsp+7*mmsize+7*gprsize], r2
    mov [rsp+7*mmsize+9*gprsize], r3
    mov [rsp+7*mmsize+10*gprsize], r4

    mov              r0, r8m
    mov              r1, r9m
    mov              r2, r10m
    mov              r4, r11m
    mov              r3, r12m
    mov [rsp+7*mmsize+11*gprsize], r0
    mov [rsp+7*mmsize+12*gprsize], r1
    mov [rsp+7*mmsize+13*gprsize], r2
    mov [rsp+7*mmsize+14*gprsize], r4
%else
cglobal fguv_32x32xn_i%1_8bpc, 0, 7, 8, 7 * mmsize + (4) * gprsize, \
        tmp, src, scaling, h, fg_data, picptr, unused
%endif
    mov            srcq, srcm
    mov        fg_dataq, r3m
    mov        scalingq, r5m
%if STACK_ALIGNMENT < mmsize
%define r0m [rsp+7*mmsize+ 3*gprsize]
%define r1m [rsp+7*mmsize+ 4*gprsize]
%define r2m [rsp+7*mmsize+ 5*gprsize]
%define r3m [rsp+7*mmsize+ 6*gprsize]
%define r4m [rsp+7*mmsize+ 7*gprsize]
%define r5m [rsp+7*mmsize+ 8*gprsize]
%define r6m [rsp+7*mmsize+ 9*gprsize]
%define r7m [rsp+7*mmsize+10*gprsize]
%define r8m [rsp+7*mmsize+11*gprsize]
%define r9m [rsp+7*mmsize+12*gprsize]
%define r10m [rsp+7*mmsize+13*gprsize]
%define r11m [rsp+7*mmsize+14*gprsize]
%define r12m [rsp+7*mmsize+15*gprsize]
%endif
    LEA              r5, pb_mask
%define base r5-pb_mask
    mov             r5m, r5
%else
cglobal fguv_32x32xn_i%1_8bpc, 6, 15, 16, dst, src, stride, fg_data, w, scaling, \
                                     grain_lut, tmp, sby, luma, lstride, uv_pl, is_id
    lea              r8, [pb_mask]
%define base r8-pb_mask
%endif
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    movd             m3, [base+mul_bits+r6*2-14]
    mov             r6d, [fg_dataq+FGData.clip_to_restricted_range]
    lea            tmpd, [r6d*2]
%if ARCH_X86_32 && STACK_ALIGNMENT < mmsize
    test             r3, r3
%else
    cmp      dword r12m, 0                      ; is_idm
%endif
    movd             m5, [base+min+r6*2]
    cmovne          r6d, tmpd
    movd             m4, [base+max+r6*2]
    punpcklwd        m3, m3
    punpcklwd        m5, m5
    punpcklwd        m4, m4
    pshufd           m3, m3, q0000
    pshufd           m5, m5, q0000
    pshufd           m4, m4, q0000
    SCRATCH           3, 11, 0
    SCRATCH           4, 12, 1
    SCRATCH           5, 13, 2

    cmp byte [fg_dataq+FGData.chroma_scaling_from_luma], 0
    jne .csfl

%macro %%FGUV_32x32xN_LOOP 3 ; not-csfl, ss_hor, ss_ver
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, overlap
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, unused, sby, see, overlap
%endif

%if %1
    mov             r6d, dword r11m
    movd             m0, [fg_dataq+FGData.uv_mult+r6*4]
    movd             m1, [fg_dataq+FGData.uv_luma_mult+r6*4]
    punpcklbw        m6, m1, m0
    movd             m7, [fg_dataq+FGData.uv_offset+r6*4]
    punpcklwd        m6, m6
    punpcklwd        m7, m7
    pshufd           m6, m6, q0000
    pshufd           m7, m7, q0000
    SCRATCH           6, 14, 3
    SCRATCH           7, 15, 4
%endif

    mov            sbyd, r8m
    mov        overlapd, [fg_dataq+FGData.overlap_flag] ; left_overlap: overlap & 1
    test       overlapd, overlapd
    jz %%no_vertical_overlap
%if ARCH_X86_32
%if %2
    mova             m1, [base+pb_23_22_h]
%else
    mova             m1, [base+pb_27_17_17_27]
%endif
    mova             m0, [base+pw_1024]
%else
%if %2
    mova             m1, [pb_23_22_h]
%else
    mova             m1, [pb_27_17_17_27]
%endif
    mova             m0, [pw_1024]
%endif
    SCRATCH           0, 8, 5
    SCRATCH           1, 9, 6
    test           sbyd, sbyd
    jnz %%vertical_overlap
    ; fall-through

%%no_vertical_overlap:
    mov             r8m, overlapd
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

    DEFINE_ARGS luma, src, scaling, see, w, picptr, src_bak
%define luma_bakq lumaq

    mov              wq, r4m
%if %3
    shl           r10mp, 1
%endif
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                unused2, unused3, see, overlap, unused4, src_bak, lstride, luma_bak

    mov        lstrideq, r10mp
%endif

    mov           lumaq, r9mp
    lea        src_bakq, [srcq+wq]
    lea       luma_bakq, [lumaq+wq*(1+%2)]
    neg              wq
    sub            r0mp, srcq
%if ARCH_X86_32
    mov             r1m, src_bakq
    mov            r11m, luma_bakq
    mov             r4m, wq

    DEFINE_ARGS tmp, src, scaling, see, unused1, picptr, unused2
%else
    mov           r11mp, src_bakq
    mov           r12mp, strideq
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
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, overlap, unused1, unused2, lstride

    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    shr           offxd, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyq, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+(3+(6>>%2))]  ; offy*stride+offx

%if ARCH_X86_32
    DEFINE_ARGS luma, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, overlap, unused1, unused2, lstride, luma_bak
%endif

%%loop_x_odd:
    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%%loop_y:
    ; src
%if ARCH_X86_32
    mov           lumaq, r9mp
%endif
%if %2
    mova             m4, [lumaq+ 0]
    mova             m6, [lumaq+16]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
    mov              r5, r5m
    movd             m7, [base+pb_1]
%else
    movd             m7, [pb_1]
%endif
    pshufd           m7, m7, q0000
    pxor             m2, m2
    pmaddubsw        m4, m7
    pmaddubsw        m6, m7
    pavgw            m4, m2
    pavgw            m6, m2
%else
    mova             m4, [lumaq]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
%endif
    pxor             m2, m2
%endif

%if %1
%if %2
    packuswb         m4, m6                 ; luma
%endif
    punpckhbw        m6, m4, m0
    punpcklbw        m4, m0                 ; { luma, chroma }
    pmaddubsw        m6, m14
    pmaddubsw        m4, m14
    psraw            m6, 6
    psraw            m4, 6
    paddw            m6, m15
    paddw            m4, m15
    packuswb         m4, m6                 ; pack+unpack = clip
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%elif %2 == 0
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%endif

    ; scaling[luma_src]
%if ARCH_X86_32
    vpgatherdw       m7, m4, scalingq-1, r0, r5
    vpgatherdw       m5, m6, scalingq-1, r0, r5
%else
    vpgatherdw       m7, m4, scalingq-1, r12, r2
    vpgatherdw       m5, m6, scalingq-1, r12, r2
%endif
    REPX {psrlw x, 8}, m7, m5

    ; unpack chroma_source
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2                 ; m0-1: src as word

    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq+ 0]
    pcmpgtb          m6, m2, m3
    punpcklbw        m2, m3, m6
    punpckhbw        m3, m6

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    pmullw           m2, m7
    pmullw           m3, m5
    pmulhrsw         m2, m11
    pmulhrsw         m3, m11

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    packuswb         m0, m1
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

%if ARCH_X86_32
    add            srcq, r2mp
    ; we already incremented lumaq above
%else
    add            srcq, r12mp
%if %3
    lea           lumaq, [lumaq+lstrideq*2]
%else
    add           lumaq, lstrideq
%endif
%endif
    add      grain_lutq, 82
    dec              hw
    jg %%loop_y

%if ARCH_X86_32
    DEFINE_ARGS luma, src, scaling, offxy, w, picptr, grain_lut

    mov              wq, r4m
%endif
    add              wq, 16
    jge %%end
%if ARCH_X86_32
    mov            srcq, r1mp
    mov           lumaq, r11mp
%else
    mov            srcq, r11mp
%endif
    lea           lumaq, [luma_bakq+wq*(1+%2)]
    add            srcq, wq
%if ARCH_X86_32
    mov             r4m, wq
    mov             r9m, lumaq
%endif
%if %2 == 0
    ; adjust top_offxy
%if ARCH_X86_32
    add dword [rsp+7*mmsize+1*gprsize], 16
%else
    add            r11d, 16
%endif
    add          offxyd, 16
    btc       dword r8m, 2
    jc %%loop_x_even
    test      dword r8m, 2
    jz %%loop_x_odd
    jmp %%loop_x_odd_v_overlap
%%loop_x_even:
%endif
    test      dword r8m, 1
    jz %%loop_x

    ; r8m = sbym
    test      dword r8m, 2
    jne %%loop_x_hv_overlap

    ; horizontal overlap (without vertical overlap)
%%loop_x_h_overlap:
%if ARCH_X86_32
%if %2
    lea              r6, [offxyd+16]
    mov [rsp+7*mmsize+0*gprsize], r6
%else
    mov [rsp+7*mmsize+0*gprsize], offxyd
%endif

    DEFINE_ARGS luma, src, scaling, see, w, picptr, grain_lut

    mov            seed, r3m
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, unused1, unused2, lstride

%if %2
    lea     left_offxyd, [offyd+16]         ; previous column's offy*stride+offx
%else
    mov     left_offxyd, offyd
%endif
%endif
    mov             r6d, seed
    or             seed, 0xEFF4
    shr             r6d, 1
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                ; updated seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS luma, src, scaling, offy, w, picptr, offx

    mov          offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, unused1, unused2, lstride

    mov           offyd, seed
    mov           offxd, seed
%endif
    ror           offyd, 8
    shr           offxd, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyq, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)]  ; offy*stride+offx

%if ARCH_X86_32
    DEFINE_ARGS luma, src, scaling, offxy, h, picptr, grain_lut
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, unused1, unused2, lstride, luma_bak
%endif

    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%%loop_y_h_overlap:
    ; src
%if ARCH_X86_32
    mov           lumaq, r9mp
%endif
%if %2
    mova             m4, [lumaq+ 0]
    mova             m6, [lumaq+16]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
    mov              r5, r5m
    movd             m7, [base+pb_1]
%else
    movd             m7, [pb_1]
%endif
    pshufd           m7, m7, q0000
    pxor             m2, m2
    pmaddubsw        m4, m7
    pmaddubsw        m6, m7
    pavgw            m4, m2
    pavgw            m6, m2
%else
    mova             m4, [lumaq]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
%endif
    pxor             m2, m2
%endif

%if %1
%if %2
    packuswb         m4, m6                 ; luma
%endif
    punpckhbw        m6, m4, m0
    punpcklbw        m4, m0                 ; { luma, chroma }
    pmaddubsw        m6, m14
    pmaddubsw        m4, m14
    psraw            m6, 6
    psraw            m4, 6
    paddw            m6, m15
    paddw            m4, m15
    packuswb         m4, m6                 ; pack+unpack = clip
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%elif %2 == 0
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%endif

    ; scaling[luma_src]
%if ARCH_X86_32
    vpgatherdw       m7, m4, scalingq-1, r0, r5
    vpgatherdw       m5, m6, scalingq-1, r0, r5
%else
    vpgatherdw       m7, m4, scalingq-1, r12, r2
    vpgatherdw       m5, m6, scalingq-1, r12, r2
%endif
    REPX {psrlw x, 8}, m7, m5

    ; unpack chroma_source
    punpckhbw        m1, m0, m2
    punpcklbw        m0, m2                 ; m0-1: src as word

    ; grain = grain_lut[offy+y][offx+x]
    movu             m4, [grain_lutq+offxyq+ 0]
%if ARCH_X86_32
    mov              r0, [rsp+7*mmsize+0*gprsize]
    movd             m2, [grain_lutq+r0+ 0]
%else
    movd             m2, [grain_lutq+left_offxyq+ 0]
%endif
    punpcklbw        m2, m4
    pmaddubsw        m3, m9, m2
    pmulhrsw         m3, m8
    packsswb         m3, m3
    shufps           m3, m4, q3210
    pxor             m4, m4
    pcmpgtb          m4, m3
    punpcklbw        m2, m3, m4
    punpckhbw        m3, m4

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    pmullw           m2, m7
    pmullw           m3, m5
    pmulhrsw         m2, m11
    pmulhrsw         m3, m11

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    pmaxsw           m0, m13
    pmaxsw           m1, m13
    pminsw           m0, m12
    pminsw           m1, m12
    packuswb         m0, m1
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

%if ARCH_X86_32
    add            srcq, r2mp
    ; lumaq has already been incremented above
%else
    add            srcq, r12mp
%if %3
    lea           lumaq, [lumaq+lstrideq*2]
%else
    add           lumaq, lstrideq
%endif
%endif
    add      grain_lutq, 82
    dec              hw
    jg %%loop_y_h_overlap

%if ARCH_X86_32
    DEFINE_ARGS luma, src, scaling, offxy, w, picptr, grain_lut

    mov              wq, r4m
%endif
    add              wq, 16
    jge %%end
%if ARCH_X86_32
    mov            srcq, r1mp
    mov           lumaq, r11mp
%else
    mov            srcq, r11mp
%endif
    lea           lumaq, [luma_bakq+wq*(1+%2)]
    add            srcq, wq
%if ARCH_X86_32
    mov             r4m, wq
    mov             r9m, lumaq
%endif
%if %2 == 0
    xor       dword r8m, 4
    ; adjust top_offxyd
%if ARCH_X86_32
    add dword [rsp+7*mmsize+1*gprsize], 16
%else
    add            r11d, 16
%endif
    add          offxyd, 16
%endif

    ; r8m = sbym
    test      dword r8m, 2
%if %2
    jne %%loop_x_hv_overlap
    jmp %%loop_x_h_overlap
%else
    jne %%loop_x_odd_v_overlap
    jmp %%loop_x_odd
%endif

%%end:
    RET

%%vertical_overlap:
%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, sby, fg_data, picptr, overlap
%else
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, tmp, sby, see, overlap
%endif

    or         overlapd, 2                  ; top_overlap: overlap & 2
    mov             r8m, overlapd
    movzx          sbyd, sbyb
%if ARCH_X86_32
    imul             r4, [fg_dataq+FGData.seed], 0x00010001
    DEFINE_ARGS tmp, src, scaling, sby, see, picptr, unused
%else
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
%endif
    imul           tmpd, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add            tmpd, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and            tmpd, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, tmpd
%if ARCH_X86_32
    xor            sbyd, seed               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS luma, src, scaling, see, w, picptr, src_bak

    mov             r3m, seed
    mov              wq, r4m
%if %3
    shl           r10mp, 1
%endif
%else
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                tmp, unused2, see, overlap, unused3, src_bak, lstride, luma_bak

    mov        lstrideq, r10mp
%endif

    mov           lumaq, r9mp
    lea        src_bakq, [srcq+wq]
    lea       luma_bakq, [lumaq+wq*(1+%2)]
    neg              wq
    sub            r0mp, srcq
%if ARCH_X86_32
    mov             r1m, src_bakq
    mov            r11m, luma_bakq
    mov             r4m, wq

    DEFINE_ARGS tmp, src, scaling, see, unused1, picptr, unused2
%else
    mov           r11mp, src_bakq
    mov           r12mp, strideq
%endif

%%loop_x_v_overlap:
%if ARCH_X86_32
    mov            seed, r3m
    xor            tmpd, tmpd
%endif
    ; we assume from the block above that bits 8-15 of tmpd are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp           tmpb                     ; parity of top_seed
    shr            seed, 16
    shl            tmpd, 16
    test           seeb, seeh
    setp           tmpb                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor            tmpd, r6d
    mov            seed, tmpd
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS dst, src, scaling, offy, h, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, overlap, top_offxy, unused, lstride

    mov           offxd, seed
    mov           offyd, seed
%endif
    ror           offyd, 8
    ror           offxd, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyq, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

%if ARCH_X86_32
    DEFINE_ARGS tmp, src, scaling, offxy, h, picptr, top_offxy
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, overlap, top_offxy, unused, lstride, luma_bak
%endif

    movzx    top_offxyd, offxyw
    shr          offxyd, 16
%if ARCH_X86_32
    mov [rsp+7*mmsize+1*gprsize], top_offxyd

    DEFINE_ARGS luma, src, scaling, offxy, h, picptr, grain_lut
%endif

%%loop_x_odd_v_overlap:
    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%if ARCH_X86_32
    mov              r5, r5m
%endif
%if %3
    mova             m1, [PIC_ptr(pb_23_22)]
%else
    mova             m1, [PIC_ptr(pb_27_17)]
%endif
%%loop_y_v_overlap:
%if ARCH_X86_32
    mov           lumaq, r9mp
%endif
%if %2
    mova             m4, [lumaq+ 0]
    mova             m6, [lumaq+16]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
    mov              r5, r5m
    movd             m7, [base+pb_1]
%else
    movd             m7, [pb_1]
%endif
    pshufd           m7, m7, q0000
    pxor             m2, m2
    pmaddubsw        m4, m7
    pmaddubsw        m6, m7
    pavgw            m4, m2
    pavgw            m6, m2
%else
    mova             m4, [lumaq]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
%endif
    pxor             m2, m2
%endif

%if %1
%if %2
    packuswb         m4, m6                 ; luma
%endif
    punpckhbw        m6, m4, m0
    punpcklbw        m4, m0                 ; { luma, chroma }
    pmaddubsw        m6, m14
    pmaddubsw        m4, m14
    psraw            m6, 6
    psraw            m4, 6
    paddw            m6, m15
    paddw            m4, m15
    packuswb         m4, m6                 ; pack+unpack = clip
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%elif %2 == 0
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%endif

    ; scaling[luma_src]
%if ARCH_X86_32
    vpgatherdw       m7, m4, scalingq-1, r0, r5
    vpgatherdw       m5, m6, scalingq-1, r0, r5
%else
    vpgatherdw       m7, m4, scalingq-1, r12, r2
    vpgatherdw       m5, m6, scalingq-1, r12, r2
%endif
    REPX {psrlw x, 8}, m7, m5

    ; grain = grain_lut[offy+y][offx+x]
    movu             m3, [grain_lutq+offxyq]
%if ARCH_X86_32
    mov              r0, [rsp+7*mmsize+1*gprsize]
    movu             m4, [grain_lutq+r0]
%else
    movu             m4, [grain_lutq+top_offxyq]
%endif
    punpckhbw        m6, m4, m3
    punpcklbw        m4, m3
    pmaddubsw        m2, m1, m6
    pmaddubsw        m3, m1, m4
    pmulhrsw         m2, m8
    pmulhrsw         m3, m8
    packsswb         m3, m2
    pxor             m6, m6
    pcmpgtb          m6, m3
    punpcklbw        m2, m3, m6
    punpckhbw        m3, m6

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    pmullw           m2, m7
    pmullw           m3, m5
    pmulhrsw         m2, m11
    pmulhrsw         m3, m11

    ; unpack chroma_source
    pxor             m4, m4
    punpckhbw        m6, m0, m4
    punpcklbw        m0, m4                 ; m0-1: src as word

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m6, m3
    pmaxsw           m0, m13
    pmaxsw           m6, m13
    pminsw           m0, m12
    pminsw           m6, m12
    packuswb         m0, m6
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

    dec              hw
    je %%end_y_v_overlap
%if ARCH_X86_32
    add            srcq, r2mp
    ; lumaq has already been incremented above
%else
    add            srcq, r12mp
%if %3
    lea           lumaq, [lumaq+lstrideq*2]
%else
    add           lumaq, lstrideq
%endif
%endif
    add      grain_lutq, 82
%if %3 == 0
    btc              hd, 16
%if ARCH_X86_32
    mov              r5, r5m
%endif
    mova             m1, [PIC_ptr(pb_17_27)]
    jnc %%loop_y_v_overlap
%endif
    jmp %%loop_y

%%end_y_v_overlap:
%if ARCH_X86_32
    DEFINE_ARGS luma, src, scaling, offxy, w, picptr, grain_lut

    mov              wq, r4m
%endif
    add              wq, 16
    jge %%end_hv
%if ARCH_X86_32
    mov            srcq, r1mp
    mov           lumaq, r11mp
%else
    mov            srcq, r11mp
%endif
    lea           lumaq, [luma_bakq+wq*(1+%2)]
    add            srcq, wq
%if ARCH_X86_32
    mov             r4m, wq
    mov             r9m, lumaq
%endif

%if %2
    ; since fg_dataq.overlap is guaranteed to be set, we never jump
    ; back to .loop_x_v_overlap, and instead always fall-through to
    ; h+v overlap
%else
%if ARCH_X86_32
    add dword [rsp+7*mmsize+1*gprsize], 16
%else
    add      top_offxyd, 16
%endif
    add          offxyd, 16
    btc       dword r8m, 2
    jnc %%loop_x_odd_v_overlap
%endif

%%loop_x_hv_overlap:
%if ARCH_X86_32
    DEFINE_ARGS tmp, src, scaling, offxy, w, picptr, unused

    mov              r6, [rsp+7*mmsize+1*gprsize]
%if %2
    lea              r0, [r3d+16]
    add              r6, 16
    mov [rsp+7*mmsize+0*gprsize], r0        ; left_offxy
%else
    mov [rsp+7*mmsize+0*gprsize], r3        ; left_offxy
%endif
    mov [rsp+7*mmsize+2*gprsize], r6        ; topleft_offxy

    DEFINE_ARGS tmp, src, scaling, see, w, picptr, unused

    mov            seed, r3m
    xor            tmpd, tmpd
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                tmp, offxy, see, left_offxy, top_offxy, topleft_offxy, lstride

%if %2
    lea  topleft_offxyq, [top_offxyq+16]
    lea     left_offxyq, [offxyq+16]
%else
    mov  topleft_offxyq, top_offxyq
    mov     left_offxyq, offxyq
%endif

    ; we assume from the block above that bits 8-15 of tmpd are zero'ed
%endif
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp           tmpb                     ; parity of top_seed
    shr            seed, 16
    shl            tmpd, 16
    test           seeb, seeh
    setp           tmpb                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor            tmpd, r6d
    mov            seed, tmpd
    ror            seed, 1                  ; updated (cur_seed << 16) | top_seed

%if ARCH_X86_32
    mov             r3m, seed

    DEFINE_ARGS tmp, src, scaling, offy, w, picptr, offx

    mov           offxd, offyd
%else
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, top_offxy, topleft_offxy, lstride

    mov           offxd, seed
    mov           offyd, seed
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
    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, top_offxy, topleft_offxy, lstride, luma_bak
%endif

    movzx    top_offxyd, offxyw
    shr          offxyd, 16
%if ARCH_X86_32
    mov [rsp+7*mmsize+1*gprsize], top_offxyd
%endif

    mov              hd, r7m
    mov      grain_lutq, grain_lutmp
%if ARCH_X86_32
    mov              r5, r5m
%endif
%if %3
    mova             m3, [PIC_ptr(pb_23_22)]
%else
    mova             m3, [PIC_ptr(pb_27_17)]
%endif
%%loop_y_hv_overlap:
    ; grain = grain_lut[offy+y][offx+x]
%if ARCH_X86_32
    mov              r0, [rsp+7*mmsize+2*gprsize]       ; topleft_offxy
    mov              r5, [rsp+7*mmsize+1*gprsize]       ; top_offxy
    movd             m1, [grain_lutq+r0]
    mov              r0, [rsp+7*mmsize+0*gprsize]       ; left_offxy
%else
    movd             m1, [grain_lutq+topleft_offxyq]
%endif
    movu             m2, [grain_lutq+offxyq]
%if ARCH_X86_32
    movu             m6, [grain_lutq+r5]
    movd             m4, [grain_lutq+r0]
%else
    movu             m6, [grain_lutq+top_offxyq]
    movd             m4, [grain_lutq+left_offxyq]
%endif
    ; do h interpolation first (so top | top/left -> top, left | cur -> cur)
    punpcklbw        m1, m6
    punpcklbw        m4, m2
    pmaddubsw        m0, m9, m1
    pmaddubsw        m1, m9, m4
    REPX {pmulhrsw x, m8}, m0, m1
    packsswb         m0, m1
    shufps           m4, m0, m2, q3232
    shufps           m0, m6, q3210
    ; followed by v interpolation (top | cur -> cur)
    punpcklbw        m2, m0, m4
    punpckhbw        m0, m4
    pmaddubsw        m4, m3, m0
    pmaddubsw        m1, m3, m2
    pmulhrsw         m4, m8
    pmulhrsw         m1, m8
    packsswb         m1, m4

    ; src
%if ARCH_X86_32
    DEFINE_ARGS luma, src, scaling, offxy, w, picptr, grain_lut

    mov           lumaq, r9mp
%endif
%if %2
    mova             m4, [lumaq+ 0]
    mova             m6, [lumaq+16]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
    mov              r5, r5m
    movd             m7, [base+pb_1]
%else
    movd             m7, [pb_1]
%endif
    pshufd           m7, m7, q0000
    pxor             m2, m2
    pmaddubsw        m4, m7
    pmaddubsw        m6, m7
    pavgw            m4, m2
    pavgw            m6, m2
%else
    mova             m4, [lumaq]
    mova             m0, [srcq]
%if ARCH_X86_32
    add           lumaq, r10mp
    mov            r9mp, lumaq
%endif
    pxor             m2, m2
%endif

%if %1
%if %2
    packuswb         m4, m6                 ; luma
%endif
    punpckhbw        m6, m4, m0
    punpcklbw        m4, m0                 ; { luma, chroma }
    pmaddubsw        m6, m14
    pmaddubsw        m4, m14
    psraw            m6, 6
    psraw            m4, 6
    paddw            m6, m15
    paddw            m4, m15
    packuswb         m4, m6                 ; pack+unpack = clip
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%elif %2 == 0
    punpckhbw        m6, m4, m2
    punpcklbw        m4, m2
%endif

    ; scaling[src]
%if ARCH_X86_32
    vpgatherdw       m7, m4, scalingq-1, r0, r5
    vpgatherdw       m5, m6, scalingq-1, r0, r5
%else
%if %3
    vpgatherdw       m7, m4, scalingq-1, r2, r12
    vpgatherdw       m5, m6, scalingq-1, r2, r12
%else
    vpgatherdw       m7, m4, scalingq-1, r2, r13
    vpgatherdw       m5, m6, scalingq-1, r2, r13
%endif
%endif
    REPX {psrlw x, 8}, m7, m5

    ; unpack grain
    pxor             m4, m4
    pcmpgtb          m4, m1
    punpcklbw        m2, m1, m4
    punpckhbw        m1, m4

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmullw           m2, m7
    pmullw           m1, m5
    pmulhrsw         m2, m11
    pmulhrsw         m1, m11

%if ARCH_X86_32
    DEFINE_ARGS dst, src, scaling, offxy, h, picptr, grain_lut
%endif

    ; unpack chroma source
    pxor             m4, m4
    punpckhbw        m5, m0, m4
    punpcklbw        m0, m4                 ; m0-1: src as word

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m5, m1
    pmaxsw           m0, m13
    pmaxsw           m5, m13
    pminsw           m0, m12
    pminsw           m5, m12
    packuswb         m0, m5
    movifnidn      dstq, dstmp
    mova    [dstq+srcq], m0

%if ARCH_X86_32
    add            srcq, r2mp
    ; lumaq has been adjusted above already
%else
    add            srcq, r12mp
%if %3
    lea           lumaq, [lumaq+lstrideq*(1+%2)]
%else
    add           lumaq, r10mp
%endif
%endif
    add      grain_lutq, 82
    dec              hw
%if %3
    jg %%loop_y_h_overlap
%else
    jle %%end_y_hv_overlap
%if ARCH_X86_32
    mov              r5, r5m
%endif
    mova             m3, [PIC_ptr(pb_17_27)]
    btc              hd, 16
    jnc %%loop_y_hv_overlap
%if ARCH_X86_64
    mov        lstrideq, r10mp
%endif
    jmp %%loop_y_h_overlap
%%end_y_hv_overlap:
%if ARCH_X86_64
    mov        lstrideq, r10mp
%endif
%endif

%if ARCH_X86_32
    DEFINE_ARGS luma, src, scaling, offxy, w, picptr, grain_lut

    mov              wq, r4m
%endif
    add              wq, 16
    jge %%end_hv
%if ARCH_X86_32
    mov            srcq, r1mp
    mov           lumaq, r11mp
%else
    mov            srcq, r11mp
%endif
    lea           lumaq, [luma_bakq+wq*(1+%2)]
    add            srcq, wq
%if ARCH_X86_32
    mov             r4m, wq
    mov             r9m, lumaq
%endif
%if %2
    jmp %%loop_x_hv_overlap
%else
%if ARCH_X86_32
    add dword [rsp+7*mmsize+1*gprsize], 16
%else
    add      top_offxyd, 16
%endif
    add          offxyd, 16
    xor       dword r8m, 4
    jmp %%loop_x_odd_v_overlap
%endif

%%end_hv:
    RET
%endmacro

    %%FGUV_32x32xN_LOOP 1, %2, %3
.csfl:
    %%FGUV_32x32xN_LOOP 0, %2, %3
%endmacro

FGUV_FN 420, 1, 1

%if STACK_ALIGNMENT < mmsize
DECLARE_ARG 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
%endif

FGUV_FN 422, 1, 0

%if STACK_ALIGNMENT < mmsize
DECLARE_ARG 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
%endif

FGUV_FN 444, 0, 0
