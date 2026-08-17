; Copyright © 2019-2022, VideoLAN and dav1d authors
; Copyright © 2019-2022, Two Orioles, LLC
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

%if ARCH_X86_64

SECTION_RODATA 32
pb_mask:       db  0,128,128,  0,128,  0,  0,128,128,  0,  0,128,  0,128,128,  0
gen_shufE:     db  0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
gen_shufA:     db  0,  1,  2,  3,  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9
gen_shufB:     db  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11
gen_shufC:     db  4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13
gen_shufD:     db  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13, 12, 13, 14, 15
; note: the order of (some of) the following constants matter
pb_27_17:      times 2 db 27, 17
byte_blend:            db  0,  0,  0, -1
pb_27_17_17_27:        db 27, 17, 17, 27,  0, 32,  0, 32
pb_17_27:      times 2 db 17, 27
pb_1:          times 4 db 1
pb_23_22:              db 23, 22,  0, 32,  0, 32,  0, 32
next_upperbit_mask:    dw 0x100B, 0x2016, 0x402C, 0x8058
pw_seed_xor:   times 2 dw 0xb524
               times 2 dw 0x49d8
fg_min:        times 4 db 0
               times 4 db 16
fg_max:        times 4 db 255
               times 4 db 240
               times 4 db 235
pd_m65536:             dd -65536
pw_8:          times 2 dw 8
pw_1024:       times 2 dw 1024
hmul_bits:             dw 32768, 16384,  8192,  4096
round:                 dw  2048,  1024,   512
mul_bits:              dw   256,   128,    64,    32,    16
round_vals:            dw    32,    64,   128,   256,   512
pw_1:                  dw 1

%macro JMP_TABLE 2-*
    %1_8bpc_%2_table:
    %xdefine %%base %1_8bpc_%2_table
    %xdefine %%prefix mangle(private_prefix %+ _%1_8bpc_%2)
    %rep %0 - 2
        dd %%prefix %+ .ar%3 - %%base
        %rotate 1
    %endrep
%endmacro

JMP_TABLE generate_grain_y,      avx2, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_420, avx2, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_422, avx2, 0, 1, 2, 3
JMP_TABLE generate_grain_uv_444, avx2, 0, 1, 2, 3

SECTION .text

INIT_YMM avx2
cglobal generate_grain_y_8bpc, 2, 9, 8, buf, fg_data
%define base r4-generate_grain_y_8bpc_avx2_table
    lea              r4, [generate_grain_y_8bpc_avx2_table]
    vpbroadcastw    xm0, [fg_dataq+FGData.seed]
    mov             r6d, [fg_dataq+FGData.grain_scale_shift]
    movq            xm1, [base+next_upperbit_mask]
    movsxd           r5, [fg_dataq+FGData.ar_coeff_lag]
    movq            xm4, [base+mul_bits]
    movq            xm5, [base+hmul_bits]
    mov              r7, -73*82
    mova            xm6, [base+pb_mask]
    sub            bufq, r7
    vpbroadcastw    xm7, [base+round+r6*2]
    lea              r6, [gaussian_sequence]
    movsxd           r5, [r4+r5*4]
.loop:
    pand            xm2, xm0, xm1
    psrlw           xm3, xm2, 10
    por             xm2, xm3            ; bits 0xf, 0x1e, 0x3c and 0x78 are set
    pmullw          xm2, xm4            ; bits 0x0f00 are set
    pmulhuw         xm0, xm5
    pshufb          xm3, xm6, xm2       ; set 15th bit for next 4 seeds
    psllq           xm2, xm3, 30
    por             xm2, xm3
    psllq           xm3, xm2, 15
    por             xm2, xm0            ; aggregate each bit into next seed's high bit
    por             xm3, xm2            ; 4 next output seeds
    pshuflw         xm0, xm3, q3333
    psrlw           xm3, 5
    pand            xm2, xm0, xm1
    movq             r2, xm3
    psrlw           xm3, xm2, 10
    por             xm2, xm3
    pmullw          xm2, xm4
    pmulhuw         xm0, xm5
    movzx           r3d, r2w
    pshufb          xm3, xm6, xm2
    psllq           xm2, xm3, 30
    por             xm2, xm3
    psllq           xm3, xm2, 15
    por             xm0, xm2
    movd            xm2, [r6+r3*2]
    rorx             r3, r2, 32
    por             xm3, xm0
    shr             r2d, 16
    pinsrw          xm2, [r6+r2*2], 1
    pshuflw         xm0, xm3, q3333
    movzx           r2d, r3w
    psrlw           xm3, 5
    pinsrw          xm2, [r6+r2*2], 2
    shr             r3d, 16
    movq             r2, xm3
    pinsrw          xm2, [r6+r3*2], 3
    movzx           r3d, r2w
    pinsrw          xm2, [r6+r3*2], 4
    rorx             r3, r2, 32
    shr             r2d, 16
    pinsrw          xm2, [r6+r2*2], 5
    movzx           r2d, r3w
    pinsrw          xm2, [r6+r2*2], 6
    shr             r3d, 16
    pinsrw          xm2, [r6+r3*2], 7
    pmulhrsw        xm2, xm7
    packsswb        xm2, xm2
    movq      [bufq+r7], xm2
    add              r7, 8
    jl .loop

    ; auto-regression code
    add              r5, r4
    jmp              r5

.ar1:
    DEFINE_ARGS buf, fg_data, cf3, shift, val3, min, max, x, val0
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movsx          cf3d, byte [fg_dataq+FGData.ar_coeffs_y+3]
    movd            xm5, [fg_dataq+FGData.ar_coeffs_y]
    mova            xm2, [base+gen_shufC]
    DEFINE_ARGS buf, h, cf3, shift, val3, min, max, x, val0
    pinsrb          xm5, [base+pb_1], 3
    vpbroadcastw    xm3, [base+round_vals+shiftq*2-12]    ; rnd
    pmovsxbw        xm5, xm5
    pshufd          xm4, xm5, q0000
    pshufd          xm5, xm5, q1111
    sub            bufq, 82*73-(82*3+79)
    mov              hd, 70
    mov            mind, -128
    mov            maxd, 127
.y_loop_ar1:
    mov              xq, -76
    movsx         val3d, byte [bufq+xq-1]
.x_loop_ar1:
    pmovsxbw        xm1, [bufq+xq-82-3]
    pshufb          xm0, xm1, xm2
    punpckhwd       xm1, xm3
    pmaddwd         xm0, xm4
    pmaddwd         xm1, xm5
    paddd           xm0, xm1
.x_loop_ar1_inner:
    movd          val0d, xm0
    psrldq          xm0, 4
    imul          val3d, cf3d
    add           val3d, val0d
    movsx         val0d, byte [bufq+xq]
    sarx          val3d, val3d, shiftd
    add           val3d, val0d
    cmp           val3d, maxd
    cmovns        val3d, maxd
    cmp           val3d, mind
    cmovs         val3d, mind
    mov       [bufq+xq], val3b
    ; keep val3d in-place as left for next x iteration
    inc              xq
    jz .x_loop_ar1_end
    test             xb, 3
    jnz .x_loop_ar1_inner
    jmp .x_loop_ar1
.x_loop_ar1_end:
    add            bufq, 82
    dec              hd
    jg .y_loop_ar1
.ar0:
    RET

.ar2:
%if WIN64
    ; xmm6 and xmm7 already saved
    %assign xmm_regs_used 16
    %assign stack_size_padded 168
    SUB             rsp, stack_size_padded
    movaps   [rsp+16*2], xmm8
    movaps   [rsp+16*3], xmm9
    movaps   [rsp+16*4], xmm10
    movaps   [rsp+16*5], xmm11
    movaps   [rsp+16*6], xmm12
    movaps   [rsp+16*7], xmm13
    movaps   [rsp+16*8], xmm14
    movaps   [rsp+16*9], xmm15
%endif
    DEFINE_ARGS buf, fg_data, h, x
    mov             r6d, [fg_dataq+FGData.ar_coeff_shift]
    pmovsxbw        xm7, [fg_dataq+FGData.ar_coeffs_y+0]    ; cf0-7
    movd            xm9, [fg_dataq+FGData.ar_coeffs_y+8]    ; cf8-11
    vpbroadcastd   xm10, [base+round_vals-14+r6*2]
    movd           xm11, [base+byte_blend+1]
    pmovsxbw        xm9, xm9
    pshufd          xm4, xm7, q0000
    mova           xm12, [base+gen_shufA]
    pshufd          xm5, xm7, q3333
    mova           xm13, [base+gen_shufB]
    pshufd          xm6, xm7, q1111
    mova           xm14, [base+gen_shufC]
    pshufd          xm7, xm7, q2222
    mova           xm15, [base+gen_shufD]
    pshufd          xm8, xm9, q0000
    psrld          xm10, 16
    pshufd          xm9, xm9, q1111
    sub            bufq, 82*73-(82*3+79)
    mov              hd, 70
.y_loop_ar2:
    mov              xq, -76
.x_loop_ar2:
    pmovsxbw        xm0, [bufq+xq-82*2-2]   ; y=-2,x=[-2,+5]
    pmovsxbw        xm1, [bufq+xq-82*1-2]   ; y=-1,x=[-2,+5]
    pshufb          xm2, xm0, xm12
    pmaddwd         xm2, xm4
    pshufb          xm3, xm1, xm13
    pmaddwd         xm3, xm5
    paddd           xm2, xm3
    pshufb          xm3, xm0, xm14
    pmaddwd         xm3, xm6
    punpckhqdq      xm0, xm0
    punpcklwd       xm0, xm1
    pmaddwd         xm0, xm7
    pshufb          xm1, xm15
    pmaddwd         xm1, xm8
    paddd           xm2, xm10
    paddd           xm2, xm3
    paddd           xm0, xm1
    paddd           xm2, xm0
    movq            xm0, [bufq+xq-2]        ; y=0,x=[-2,+5]
.x_loop_ar2_inner:
    pmovsxbw        xm1, xm0
    pmaddwd         xm3, xm9, xm1
    psrldq          xm1, 4                  ; y=0,x=0
    paddd           xm3, xm2
    psrldq          xm2, 4                  ; shift top to next pixel
    psrad           xm3, [fg_dataq+FGData.ar_coeff_shift]
    ; don't packssdw since we only care about one value
    paddw           xm3, xm1
    packsswb        xm3, xm3
    pextrb    [bufq+xq], xm3, 0
    pslldq          xm3, 2
    vpblendvb       xm0, xm3, xm11
    psrldq          xm0, 1
    inc              xq
    jz .x_loop_ar2_end
    test             xb, 3
    jnz .x_loop_ar2_inner
    jmp .x_loop_ar2
.x_loop_ar2_end:
    add            bufq, 82
    dec              hd
    jg .y_loop_ar2
    RET

INIT_YMM avx2
.ar3:
%if WIN64
    ; xmm6 and xmm7 already saved
    %assign stack_offset 16
    ALLOC_STACK   16*14
    %assign stack_size stack_size - 16*4
    %assign xmm_regs_used 12
    movaps  [rsp+16*12], xmm8
    movaps  [rsp+16*13], xmm9
    movaps  [rsp+16*14], xmm10
    movaps  [rsp+16*15], xmm11
%else
    ALLOC_STACK   16*12
%endif
    mov             r6d, [fg_dataq+FGData.ar_coeff_shift]
    movd           xm11, [base+byte_blend]
    pmovsxbw         m1, [fg_dataq+FGData.ar_coeffs_y+ 0]   ; cf0-15
    pmovsxbw        xm2, [fg_dataq+FGData.ar_coeffs_y+16]   ; cf16-23
    pshufd           m0, m1, q0000
    mova    [rsp+16* 0], m0
    pshufd           m0, m1, q1111
    mova    [rsp+16* 2], m0
    pshufd           m0, m1, q2222
    mova    [rsp+16* 4], m0
    pshufd           m1, m1, q3333
    mova    [rsp+16* 6], m1
    pshufd          xm0, xm2, q0000
    mova    [rsp+16* 8], xm0
    pshufd          xm0, xm2, q1111
    mova    [rsp+16* 9], xm0
    psrldq          xm7, xm2, 10
    mova             m8, [base+gen_shufA]
    pinsrw          xm2, [base+pw_1], 5
    mova             m9, [base+gen_shufC]
    pshufd          xm2, xm2, q2222
    movu            m10, [base+gen_shufE]
    vpbroadcastw    xm6, [base+round_vals-12+r6*2]
    pinsrw          xm7, [base+round_vals+r6*2-10], 3
    mova    [rsp+16*10], xm2
    DEFINE_ARGS buf, fg_data, h, x
    sub            bufq, 82*73-(82*3+79)
    mov              hd, 70
.y_loop_ar3:
    mov              xq, -76
.x_loop_ar3:
    movu            xm5, [bufq+xq-82*3-3]    ; y=-3,x=[-3,+12]
    vinserti128      m5, [bufq+xq-82*2-3], 1 ; y=-2,x=[-3,+12]
    movu            xm4, [bufq+xq-82*1-3]    ; y=-1,x=[-3,+12]
    punpcklbw        m3, m5, m5
    punpckhwd        m5, m4
    psraw            m3, 8
    punpcklbw        m5, m5
    psraw            m5, 8
    punpcklbw       xm4, xm4
    psraw           xm4, 8
    pshufb           m0, m3, m8
    pmaddwd          m0, [rsp+16*0]
    pshufb           m1, m3, m9
    pmaddwd          m1, [rsp+16*2]
    shufps           m2, m3, m5, q1032
    paddd            m0, m1
    pshufb           m1, m2, m8
    vperm2i128       m3, m4, 0x21
    pmaddwd          m1, [rsp+16*4]
    shufps          xm2, xm3, q1021
    vpblendd         m2, m3, 0xf0
    pshufb           m2, m10
    paddd            m0, m1
    pmaddwd          m2, [rsp+16*6]
    pshufb          xm1, xm4, xm9
    pmaddwd         xm1, [rsp+16*8]
    shufps          xm4, xm5, q1132
    paddd            m0, m2
    pshufb          xm2, xm4, xm8
    pshufd          xm4, xm4, q2121
    pmaddwd         xm2, [rsp+16*9]
    punpcklwd       xm4, xm6
    pmaddwd         xm4, [rsp+16*10]
    vextracti128    xm3, m0, 1
    paddd           xm0, xm1
    movq            xm1, [bufq+xq-3]        ; y=0,x=[-3,+4]
    paddd           xm2, xm4
    paddd           xm0, xm2
    paddd           xm0, xm3
.x_loop_ar3_inner:
    pmovsxbw        xm2, xm1
    pmaddwd         xm2, xm7
    pshufd          xm3, xm2, q1111
    paddd           xm2, xm0                ; add top
    paddd           xm2, xm3                ; left+cur
    psrldq          xm0, 4
    psrad           xm2, [fg_dataq+FGData.ar_coeff_shift]
    ; don't packssdw since we only care about one value
    packsswb        xm2, xm2
    pextrb    [bufq+xq], xm2, 0
    pslldq          xm2, 3
    vpblendvb       xm1, xm2, xm11
    psrldq          xm1, 1
    inc              xq
    jz .x_loop_ar3_end
    test             xb, 3
    jnz .x_loop_ar3_inner
    jmp .x_loop_ar3
.x_loop_ar3_end:
    add            bufq, 82
    dec              hd
    jg .y_loop_ar3
    RET

%macro GEN_GRAIN_UV_FN 3 ; ss_name, ss_x, ss_y
INIT_XMM avx2
cglobal generate_grain_uv_%1_8bpc, 4, 10, 16, buf, bufy, fg_data, uv
%define base r4-generate_grain_uv_%1_8bpc_avx2_table
    lea              r4, [generate_grain_uv_%1_8bpc_avx2_table]
    vpbroadcastw    xm0, [fg_dataq+FGData.seed]
    mov             r6d, [fg_dataq+FGData.grain_scale_shift]
    movq            xm1, [base+next_upperbit_mask]
    movq            xm4, [base+mul_bits]
    movq            xm5, [base+hmul_bits]
    mova            xm6, [base+pb_mask]
    vpbroadcastw    xm7, [base+round+r6*2]
    vpbroadcastd    xm2, [base+pw_seed_xor+uvq*4]
    pxor            xm0, xm2
    lea              r6, [gaussian_sequence]
%if %2
    mov             r7d, 73-35*%3
    add            bufq, 44
.loop_y:
    mov              r5, -44
%else
    mov              r5, -73*82
    sub            bufq, r5
%endif
.loop:
    pand            xm2, xm0, xm1
    psrlw           xm3, xm2, 10
    por             xm2, xm3            ; bits 0xf, 0x1e, 0x3c and 0x78 are set
    pmullw          xm2, xm4            ; bits 0x0f00 are set
    pmulhuw         xm0, xm5
    pshufb          xm3, xm6, xm2       ; set 15th bit for next 4 seeds
    psllq           xm2, xm3, 30
    por             xm2, xm3
    psllq           xm3, xm2, 15
    por             xm2, xm0            ; aggregate each bit into next seed's high bit
    por             xm2, xm3            ; 4 next output seeds
    pshuflw         xm0, xm2, q3333
    psrlw           xm2, 5
    movq             r8, xm2
    movzx           r9d, r8w
    movd            xm2, [r6+r9*2]
    rorx             r9, r8, 32
    shr             r8d, 16
    pinsrw          xm2, [r6+r8*2], 1
    movzx           r8d, r9w
    pinsrw          xm2, [r6+r8*2], 2
    shr             r9d, 16
    pinsrw          xm2, [r6+r9*2], 3
    pmulhrsw        xm2, xm7
    packsswb        xm2, xm2
    movd      [bufq+r5], xm2
    add              r5, 4
    jl .loop
%if %2
    add            bufq, 82
    dec             r7d
    jg .loop_y
%endif

    ; auto-regression code
    movsxd           r6, [fg_dataq+FGData.ar_coeff_lag]
    movsxd           r6, [base+generate_grain_uv_%1_8bpc_avx2_table+r6*4]
    add              r6, r4
    jmp              r6

INIT_YMM avx2
.ar0:
    DEFINE_ARGS buf, bufy, fg_data, uv, unused, shift
    imul            uvd, 28
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movd            xm2, [fg_dataq+FGData.ar_coeffs_uv+uvq]
    movd            xm3, [base+hmul_bits+shiftq*2]
    DEFINE_ARGS buf, bufy, h
    pmovsxbw        xm2, xm2
%if %2
    vpbroadcastd     m7, [base+pb_1]
    vpbroadcastw     m6, [base+hmul_bits+2+%3*2]
%endif
    vpbroadcastw     m2, xm2
    vpbroadcastw     m3, xm3
    pxor            m12, m12
%if %2
    sub            bufq, 82*(73-35*%3)+82-(82*3+41)
%else
    sub            bufq, 82*70-3
%endif
    add           bufyq, 3+82*3
    mov              hd, 70-35*%3
.y_loop_ar0:
%if %2
    ; first 32 pixels
    movu            xm4, [bufyq]
    vinserti128      m4, [bufyq+32], 1
%if %3
    movu            xm0, [bufyq+82]
    vinserti128      m0, [bufyq+82+32], 1
%endif
    movu            xm5, [bufyq+16]
    vinserti128      m5, [bufyq+48], 1
%if %3
    movu            xm1, [bufyq+82+16]
    vinserti128      m1, [bufyq+82+48], 1
%endif
    pmaddubsw        m4, m7, m4
%if %3
    pmaddubsw        m0, m7, m0
%endif
    pmaddubsw        m5, m7, m5
%if %3
    pmaddubsw        m1, m7, m1
    paddw            m4, m0
    paddw            m5, m1
%endif
    pmulhrsw         m4, m6
    pmulhrsw         m5, m6
%else
    xor             r3d, r3d
    ; first 32x2 pixels
.x_loop_ar0:
    movu             m4, [bufyq+r3]
    pcmpgtb          m0, m12, m4
    punpckhbw        m5, m4, m0
    punpcklbw        m4, m0
%endif
    pmullw           m4, m2
    pmullw           m5, m2
    pmulhrsw         m4, m3
    pmulhrsw         m5, m3
%if %2
    movu             m1, [bufq]
%else
    movu             m1, [bufq+r3]
%endif
    pcmpgtb          m8, m12, m1
    punpcklbw        m0, m1, m8
    punpckhbw        m1, m8
    paddw            m0, m4
    paddw            m1, m5
    packsswb         m0, m1
%if %2
    movu         [bufq], m0
%else
    movu      [bufq+r3], m0
    add             r3d, 32
    cmp             r3d, 64
    jl .x_loop_ar0
%endif

    ; last 6/12 pixels
    movu            xm4, [bufyq+32*2]
%if %2
%if %3
    movu            xm5, [bufyq+32*2+82]
%endif
    pmaddubsw       xm4, xm7, xm4
%if %3
    pmaddubsw       xm5, xm7, xm5
    paddw           xm4, xm5
%endif
    movq            xm0, [bufq+32]
    pmulhrsw        xm4, xm6
    pmullw          xm4, xm2
    pmulhrsw        xm4, xm3
    pcmpgtb         xm5, xm12, xm0
    punpcklbw       xm5, xm0, xm5
    paddw           xm4, xm5
    packsswb        xm4, xm4
    pblendw         xm0, xm4, xm0, 1000b
    movq      [bufq+32], xm0
%else
    movu            xm0, [bufq+64]
    pcmpgtb         xm1, xm12, xm4
    punpckhbw       xm5, xm4, xm1
    punpcklbw       xm4, xm1
    pmullw          xm5, xm2
    pmullw          xm4, xm2
    vpblendd        xm1, xm3, xm12, 0x0c
    pmulhrsw        xm5, xm1
    pmulhrsw        xm4, xm3
    pcmpgtb         xm1, xm12, xm0
    punpckhbw       xm8, xm0, xm1
    punpcklbw       xm0, xm1
    paddw           xm5, xm8
    paddw           xm0, xm4
    packsswb        xm0, xm5
    movu      [bufq+64], xm0
%endif
    add            bufq, 82
    add           bufyq, 82<<%3
    dec              hd
    jg .y_loop_ar0
    RET

INIT_XMM avx2
.ar1:
    DEFINE_ARGS buf, bufy, fg_data, uv, val3, cf3, min, max, x, shift
    imul            uvd, 28
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    movsx          cf3d, byte [fg_dataq+FGData.ar_coeffs_uv+uvq+3]
    movd            xm4, [fg_dataq+FGData.ar_coeffs_uv+uvq]
    pinsrb          xm4, [fg_dataq+FGData.ar_coeffs_uv+uvq+4], 3
    DEFINE_ARGS buf, bufy, h, val0, val3, cf3, min, max, x, shift
    pmovsxbw        xm4, xm4
    pshufd          xm5, xm4, q1111
    pshufd          xm4, xm4, q0000
    pmovsxwd        xm3, [base+round_vals+shiftq*2-12]    ; rnd
%if %2
    vpbroadcastd    xm7, [base+pb_1]
    vpbroadcastw    xm6, [base+hmul_bits+2+%3*2]
%endif
    vpbroadcastd    xm3, xm3
%if %2
    sub            bufq, 82*(73-35*%3)+44-(82*3+41)
%else
    sub            bufq, 82*70-(82-3)
%endif
    add           bufyq, 79+82*3
    mov              hd, 70-35*%3
    mov            mind, -128
    mov            maxd, 127
.y_loop_ar1:
    mov              xq, -(76>>%2)
    movsx         val3d, byte [bufq+xq-1]
.x_loop_ar1:
    pmovsxbw        xm0, [bufq+xq-82-1]     ; top/left
%if %2
    movq            xm8, [bufyq+xq*2]
%if %3
    movq            xm9, [bufyq+xq*2+82]
%endif
%endif
    psrldq          xm2, xm0, 2             ; top
    psrldq          xm1, xm0, 4             ; top/right
%if %2
    pmaddubsw       xm8, xm7, xm8
%if %3
    pmaddubsw       xm9, xm7, xm9
    paddw           xm8, xm9
%endif
    pmulhrsw        xm8, xm6
%else
    pmovsxbw        xm8, [bufyq+xq]
%endif
    punpcklwd       xm0, xm2
    punpcklwd       xm1, xm8
    pmaddwd         xm0, xm4
    pmaddwd         xm1, xm5
    paddd           xm0, xm1
    paddd           xm0, xm3
.x_loop_ar1_inner:
    movd          val0d, xm0
    psrldq          xm0, 4
    imul          val3d, cf3d
    add           val3d, val0d
    sarx          val3d, val3d, shiftd
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
    add           bufyq, 82<<%3
    dec              hd
    jg .y_loop_ar1
    RET

.ar2:
    DEFINE_ARGS buf, bufy, fg_data, uv, unused, shift
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    imul            uvd, 28
    vpbroadcastw   xm13, [base+round_vals-12+shiftq*2]
    pmovsxbw        xm7, [fg_dataq+FGData.ar_coeffs_uv+uvq+0]   ; cf0-7
    pmovsxbw        xm0, [fg_dataq+FGData.ar_coeffs_uv+uvq+8]   ; cf8-12
    pinsrw          xm0, [base+pw_1], 5
%if %2
    vpbroadcastw   xm12, [base+hmul_bits+2+%3*2]
    vpbroadcastd   xm11, [base+pb_1]
%endif
    DEFINE_ARGS buf, bufy, fg_data, h, unused, x
    pshufd          xm4, xm7, q0000
    pshufd          xm5, xm7, q3333
    pshufd          xm6, xm7, q1111
    pshufd          xm7, xm7, q2222
    pshufd          xm8, xm0, q0000
    pshufd          xm9, xm0, q1111
    pshufd         xm10, xm0, q2222
%if %2
    sub            bufq, 82*(73-35*%3)+44-(82*3+41)
%else
    sub            bufq, 82*70-(82-3)
%endif
    add           bufyq, 79+82*3
    mov              hd, 70-35*%3
.y_loop_ar2:
    mov              xq, -(76>>%2)

.x_loop_ar2:
    pmovsxbw        xm0, [bufq+xq-82*2-2]   ; y=-2,x=[-2,+5]
    pmovsxbw        xm1, [bufq+xq-82*1-2]   ; y=-1,x=[-2,+5]
    pshufb          xm2, xm0, [base+gen_shufA]
    pmaddwd         xm2, xm4
    pshufb          xm3, xm1, [base+gen_shufB]
    pmaddwd         xm3, xm5
    paddd           xm2, xm3
    pshufb          xm3, xm0, [base+gen_shufC]
    pmaddwd         xm3, xm6
    punpckhqdq      xm0, xm0                 ; y=-2,x=[+2,+5]
    punpcklwd       xm0, xm1
    pmaddwd         xm0, xm7
    pshufb          xm1, [gen_shufD]
    pmaddwd         xm1, xm8
    paddd           xm2, xm3
    paddd           xm0, xm1
    paddd           xm2, xm0

%if %2
    movq            xm0, [bufyq+xq*2]
%if %3
    movq            xm3, [bufyq+xq*2+82]
%endif
    pmaddubsw       xm0, xm11, xm0
%if %3
    pmaddubsw       xm3, xm11, xm3
    paddw           xm0, xm3
%endif
    pmulhrsw        xm0, xm12
%else
    pmovsxbw        xm0, [bufyq+xq]
%endif
    punpcklwd       xm0, xm13
    pmaddwd         xm0, xm10
    paddd           xm2, xm0

    movq            xm0, [bufq+xq-2]        ; y=0,x=[-2,+5]
.x_loop_ar2_inner:
    pmovsxbw        xm0, xm0
    pmaddwd         xm3, xm0, xm9
    psrldq          xm0, 2
    paddd           xm3, xm2
    psrldq          xm2, 4                  ; shift top to next pixel
    psrad           xm3, [fg_dataq+FGData.ar_coeff_shift]
    pslldq          xm3, 2
    paddw           xm3, xm0
    pblendw         xm0, xm3, 00000010b
    packsswb        xm0, xm0
    pextrb    [bufq+xq], xm0, 1
    inc              xq
    jz .x_loop_ar2_end
    test             xb, 3
    jnz .x_loop_ar2_inner
    jmp .x_loop_ar2

.x_loop_ar2_end:
    add            bufq, 82
    add           bufyq, 82<<%3
    dec              hd
    jg .y_loop_ar2
    RET

INIT_YMM avx2
.ar3:
    DEFINE_ARGS buf, bufy, fg_data, uv, unused, shift
    mov          shiftd, [fg_dataq+FGData.ar_coeff_shift]
    imul            uvd, 28
    pmovsxbw         m0, [fg_dataq+FGData.ar_coeffs_uv+uvq+ 0] ; cf0-15
    pmovsxbw        xm1, [fg_dataq+FGData.ar_coeffs_uv+uvq+16] ; cf16-23
    vpbroadcastb    xm2, [fg_dataq+FGData.ar_coeffs_uv+uvq+24] ; cf24 [luma]
    movd           xm13, [base+round_vals-10+shiftq*2]
    vpbroadcastd   xm14, [base+round_vals-14+shiftq*2]
    pshufd           m6, m0, q0000
    pshufd           m7, m0, q1111
    pshufd           m8, m0, q2222
    pshufd           m9, m0, q3333
    pshufd         xm10, xm1, q0000
    pshufd         xm11, xm1, q1111
    pshufhw        xm12, xm1, q0000
    psraw           xm2, 8
    palignr        xm13, xm1, 10
    punpckhwd      xm12, xm2                     ; interleave luma cf
    psrld          xm14, 16
    DEFINE_ARGS buf, bufy, fg_data, h, unused, x
%if %2
    vpbroadcastw   xm15, [base+hmul_bits+2+%3*2]
    sub            bufq, 82*(73-35*%3)+44-(82*3+41)
%else
    sub            bufq, 82*70-(82-3)
%endif
    add           bufyq, 79+82*3
    mov              hd, 70-35*%3
.y_loop_ar3:
    mov              xq, -(76>>%2)
.x_loop_ar3:
    vbroadcasti128   m3, [bufq+xq-82*2-3]         ; y=-2,x=[-3,+12
    palignr         xm1, xm3, [bufq+xq-82*3-9], 6 ; y=-3,x=[-3,+12]
    vbroadcasti128   m4, [bufq+xq-82*1-3]    ; y=-1,x=[-3,+12]
    vpblendd         m3, m1, 0x0f
    pxor             m0, m0
    pcmpgtb          m2, m0, m3
    pcmpgtb          m0, m4
    punpcklbw        m1, m3, m2
    punpckhbw        m3, m2
    punpcklbw        m2, m4, m0
    punpckhbw       xm4, xm0
    pshufb           m0, m1, [base+gen_shufA]
    pmaddwd          m0, m6
    pshufb           m5, m1, [base+gen_shufC]
    pmaddwd          m5, m7
    shufps           m1, m3, q1032
    paddd            m0, m5
    pshufb           m5, m1, [base+gen_shufA]
    pmaddwd          m5, m8
    shufps          xm1, xm3, q2121
    vpblendd         m1, m2, 0xf0
    pshufb           m1, [base+gen_shufE]
    pmaddwd          m1, m9
    paddd            m0, m5
    pshufb          xm3, xm2, [base+gen_shufC]
    paddd            m0, m1
    pmaddwd         xm3, xm10
    palignr         xm1, xm4, xm2, 2
    punpckhwd       xm1, xm2, xm1
    pmaddwd         xm1, xm11
    palignr         xm4, xm2, 12
    paddd           xm3, xm1
%if %2
    vpbroadcastd    xm5, [base+pb_1]
    movq            xm1, [bufyq+xq*2]
    pmaddubsw       xm1, xm5, xm1
%if %3
    movq            xm2, [bufyq+xq*2+82]
    pmaddubsw       xm5, xm2
    paddw           xm1, xm5
%endif
    pmulhrsw        xm1, xm15
%else
    pmovsxbw        xm1, [bufyq+xq]
%endif
    punpcklwd       xm4, xm1
    pmaddwd         xm4, xm12
    movq            xm1, [bufq+xq-3]        ; y=0,x=[-3,+4]
    vextracti128    xm2, m0, 1
    paddd           xm0, xm14
    paddd           xm3, xm4
    paddd           xm0, xm3
    paddd           xm0, xm2
.x_loop_ar3_inner:
    pmovsxbw        xm1, xm1
    pmaddwd         xm2, xm13, xm1
    pshuflw         xm3, xm2, q1032
    paddd           xm2, xm0                ; add top
    paddd           xm2, xm3                ; left+cur
    psrldq          xm0, 4
    psrad           xm2, [fg_dataq+FGData.ar_coeff_shift]
    psrldq          xm1, 2
    ; don't packssdw, we only care about one value
    punpckldq       xm2, xm2
    pblendw         xm1, xm2, 0100b
    packsswb        xm1, xm1
    pextrb    [bufq+xq], xm1, 2
    inc              xq
    jz .x_loop_ar3_end
    test             xb, 3
    jnz .x_loop_ar3_inner
    jmp .x_loop_ar3
.x_loop_ar3_end:
    add            bufq, 82
    add           bufyq, 82<<%3
    dec              hd
    jg .y_loop_ar3
    RET
%endmacro

INIT_YMM avx2
cglobal fgy_32x32xn_8bpc, 6, 13, 15, dst, src, stride, fg_data, w, scaling, \
                                     grain_lut, h, sby, see, overlap
%define base r9-pd_m65536
    lea              r9, [pd_m65536]
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    mov             r7d, [fg_dataq+FGData.clip_to_restricted_range]
    mov            sbyd, sbym
    mov        overlapd, [fg_dataq+FGData.overlap_flag]
    vpbroadcastd     m8, [base+pd_m65536]
    vpbroadcastw     m9, [base+mul_bits+r6*2-14]
    vpbroadcastd    m10, [base+fg_min+r7*4]
    vpbroadcastd    m11, [base+fg_max+r7*8]
    vpbroadcastd    m12, [base+pw_1024]
    movq           xm13, [base+pb_27_17_17_27]
    test           sbyd, sbyd
    setnz           r7b
    pxor             m7, m7
    test            r7b, overlapb
    jnz .vertical_overlap

    imul           seed, sbyd, (173 << 24) | 37
    add            seed, (105 << 24) | 178
    rorx           seed, seed, 24
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, overlap

    lea        src_bakq, [srcq+wq]
    neg              wq
    sub            dstq, srcq

.loop_x:
    rorx             r6, seeq, 1
    or             seed, 0xEFF4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                ; updated seed

    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offyd, [offyq+offxq*2+747] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, overlap

    mov              hd, hm
    mov      grain_lutq, grain_lutmp
.loop_y:
    ; src
    mova             m2, [srcq]
    punpcklbw        m0, m2, m7
    punpckhbw        m1, m2, m7

    ; scaling[src]
    pandn            m4, m8, m0
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, m0, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m1
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    pblendw          m2, m4, 0xaa
    psrld            m4, m1, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m4-2], m6
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
    movu             m5, [grain_lutq+offxyq]
    punpcklbw        m4, m5, m7
    punpckhbw        m5, m7

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
    mova    [dstq+srcq], m0

    add            srcq, strideq
    add      grain_lutq, 82
    dec              hd
    jg .loop_y

    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq]
    test       overlapd, overlapd
    jz .loop_x

    ; r8m = sbym
    cmp       dword r8m, 0
    jne .loop_x_hv_overlap

    ; horizontal overlap (without vertical overlap)
.loop_x_h_overlap:
    rorx             r6, seeq, 1
    or             seed, 0xEFF4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                ; updated seed

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, left_offxy

    lea     left_offxyd, [offyq+32]         ; previous column's offy*stride+offx
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offyd, [offyq+offxq*2+747] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, left_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
.loop_y_h_overlap:
    ; src
    mova             m2, [srcq]
    punpcklbw        m0, m2, m7
    punpckhbw        m1, m2, m7

    ; scaling[src]
    pandn            m4, m8, m0
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, m0, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m1
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    pblendw          m2, m4, 0xaa
    psrld            m4, m1, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m4-2], m6
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
    movu             m5, [grain_lutq+offxyq]
    movd            xm4, [grain_lutq+left_offxyq]
    punpcklbw       xm4, xm5
    pmaddubsw       xm4, xm13, xm4
    pmulhrsw        xm4, xm12
    packsswb        xm4, xm4
    vpblendd         m4, m5, 0xfe
    punpckhbw        m5, m7
    punpcklbw        m4, m7

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
    mova    [dstq+srcq], m0

    add            srcq, strideq
    add      grain_lutq, 82
    dec              hd
    jg .loop_y_h_overlap

    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq]

    ; r8m = sbym
    cmp       dword r8m, 0
    jne .loop_x_hv_overlap
    jmp .loop_x_h_overlap

.vertical_overlap:
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                unused, sby, see, overlap

    movzx          sbyd, sbyb
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
    imul            r7d, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add             r7d, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and             r7d, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, r7d
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, overlap

    lea        src_bakq, [srcq+wq]
    neg              wq
    sub            dstq, srcq

.loop_x_v_overlap:
    vpbroadcastd    m14, [pb_27_17]

    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*2+0x10001*747+32*82]

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, overlap, top_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16
.loop_y_v_overlap:
    ; src
    mova             m2, [srcq]
    punpcklbw        m0, m2, m7
    punpckhbw        m1, m2, m7

    ; scaling[src]
    pandn            m4, m8, m0
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, m0, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m1
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    pblendw          m2, m4, 0xaa
    psrld            m4, m1, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m4-2], m6
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
    movu             m6, [grain_lutq+offxyq]
    movu             m4, [grain_lutq+top_offxyq]
    punpcklbw        m5, m4, m6
    punpckhbw        m4, m6
    pmaddubsw        m5, m14, m5
    pmaddubsw        m4, m14, m4
    pmulhrsw         m5, m12
    pmulhrsw         m4, m12
    packsswb         m5, m4
    punpcklbw        m4, m5, m7
    punpckhbw        m5, m7

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
    mova    [dstq+srcq], m0

    add            srcq, strideq
    add      grain_lutq, 82
    dec              hb
    jz .end_y_v_overlap
    vpbroadcastd    m14, [pb_17_27] ; swap weights for second v-overlap line
    ; 2 lines get vertical overlap, then fall back to non-overlap code for
    ; remaining (up to) 30 lines
    add              hd, 0x80000000
    jnc .loop_y_v_overlap
    jmp .loop_y
.end_y_v_overlap:
    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq]

    ; since fg_dataq.overlap is guaranteed to be set, we never jump
    ; back to .loop_x_v_overlap, and instead always fall-through to
    ; h+v overlap
.loop_x_hv_overlap:
    vpbroadcastd    m14, [pb_27_17]

    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, top_offxy, topleft_offxy

    lea  topleft_offxyd, [top_offxyq+32]
    lea     left_offxyd, [offyq+32]
    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*2+0x10001*747+32*82]

    DEFINE_ARGS dst, src, stride, src_bak, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, top_offxy, topleft_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16
.loop_y_hv_overlap:
    ; src
    mova             m2, [srcq]
    punpcklbw        m0, m2, m7
    punpckhbw        m1, m2, m7

    ; scaling[src]
    pandn            m4, m8, m0
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, m0, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m1
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    pblendw          m2, m4, 0xaa
    psrld            m4, m1, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m4-2], m6
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
    movu             m6, [grain_lutq+offxyq]
    movd            xm7, [grain_lutq+left_offxyq]
    movu             m4, [grain_lutq+top_offxyq]
    movd            xm5, [grain_lutq+topleft_offxyq]
    ; do h interpolation first (so top | top/left -> top, left | cur -> cur)
    punpcklbw       xm7, xm6
    punpcklbw       xm5, xm4
    pmaddubsw       xm7, xm13, xm7
    pmaddubsw       xm5, xm13, xm5
    pmulhrsw        xm7, xm12
    pmulhrsw        xm5, xm12
    packsswb        xm7, xm7
    packsswb        xm5, xm5
    vpblendd         m7, m6, 0xfe
    vpblendd         m5, m4, 0xfe
    ; followed by v interpolation (top | cur -> cur)
    punpckhbw        m4, m6
    punpcklbw        m5, m7
    pmaddubsw        m4, m14, m4
    pmaddubsw        m5, m14, m5
    pmulhrsw         m4, m12
    pmulhrsw         m5, m12
    pxor             m7, m7
    packsswb         m5, m4
    punpcklbw        m4, m5, m7
    punpckhbw        m5, m7

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
    mova    [dstq+srcq], m0

    add            srcq, strideq
    add      grain_lutq, 82
    dec              hb
    jz .end_y_hv_overlap
    vpbroadcastd    m14, [pb_17_27] ; swap weights for second v-overlap line
    ; 2 lines get vertical overlap, then fall back to non-overlap code for
    ; remaining (up to) 30 lines
    add              hd, 0x80000000
    jnc .loop_y_hv_overlap
    jmp .loop_y_h_overlap
.end_y_hv_overlap:
    add              wq, 32
    lea            srcq, [src_bakq+wq]
    jl .loop_x_hv_overlap
.end:
    RET

%macro FGUV_FN 3 ; name, ss_hor, ss_ver
cglobal fguv_32x32xn_i%1_8bpc, 6, 15, 16, dst, src, stride, fg_data, w, scaling, \
                                          grain_lut, h, sby, luma, overlap, uv_pl, is_id
%define base r11-pd_m65536
    lea             r11, [pd_m65536]
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    mov             r7d, [fg_dataq+FGData.clip_to_restricted_range]
    mov             r9d, is_idm
    mov            sbyd, sbym
    mov        overlapd, [fg_dataq+FGData.overlap_flag]
    vpbroadcastd     m8, [base+pd_m65536]
    vpbroadcastw     m9, [base+mul_bits+r6*2-14]
    vpbroadcastd    m10, [base+fg_min+r7*4]
    shlx            r7d, r7d, r9d
    vpbroadcastd    m11, [base+fg_max+r7*4]
    vpbroadcastd    m12, [base+pw_1024]
    pxor             m7, m7
    test           sbyd, sbyd
    setnz           r7b
    cmp byte [fg_dataq+FGData.chroma_scaling_from_luma], 0
    jne .csfl

%macro %%FGUV_32x32xN_LOOP 3 ; not-csfl, ss_hor, ss_ver
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, sby, see, overlap, uv_pl
%if %1
    mov             r6d, uv_plm
    vpbroadcastd     m0, [base+pw_8]
    vbroadcasti128  m14, [fg_dataq+FGData.uv_mult+r6*4]
    vpbroadcastw    m15, [fg_dataq+FGData.uv_offset+r6*4]
    pshufb          m14, m0 ; uv_luma_mult, uv_mult
%elif %2
    vpbroadcastq    m15, [base+pb_23_22]
%else
    vpbroadcastq   xm15, [base+pb_27_17_17_27]
%endif
%if %3
    vpbroadcastw    m13, [base+pb_23_22]
%elif %2
    pshufd          m13, [base+pb_27_17], q0000 ; 8x27_17, 8x17_27
%endif
    test            r7b, overlapb
    jnz %%vertical_overlap

    imul           seed, sbyd, (173 << 24) | 37
    add            seed, (105 << 24) | 178
    rorx           seed, seed, 24
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                unused2, unused3, see, overlap, unused4, unused5, lstride

    mov           lumaq, r9mp
    lea             r12, [srcq+wq]
    lea             r13, [dstq+wq]
    lea             r14, [lumaq+wq*(1+%2)]
    mov           r11mp, r12
    mov           r12mp, r13
    mov        lstrideq, r10mp
    neg              wq

%%loop_x:
    rorx             r6, seeq, 1
    or             seed, 0xEFF4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d               ; updated seed

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, overlap, unused1, unused2, lstride

    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyd, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)]  ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, overlap, unused1, unused2, lstride

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
%%loop_y:
    ; src
%if %2
    mova            xm3, [lumaq+lstrideq*0+ 0]
    vinserti128      m3, [lumaq+lstrideq*(1+%3) +0], 1
    vpbroadcastd     m2, [pb_1]
    mova            xm0, [lumaq+lstrideq*0+16]
    vinserti128      m0, [lumaq+lstrideq*(1+%3)+16], 1
    mova            xm1, [srcq]
    vinserti128      m1, [srcq+strideq], 1
    pmaddubsw        m3, m2
    pmaddubsw        m0, m2
    pavgw            m3, m7
    pavgw            m0, m7
%else
    mova             m2, [lumaq]
    mova             m1, [srcq]
%endif
%if %1
%if %2
    packuswb         m2, m3, m0             ; luma
%endif
    punpckhbw        m3, m2, m1
    punpcklbw        m2, m1                 ; { luma, chroma }
    pmaddubsw        m3, m14
    pmaddubsw        m2, m14
    psraw            m3, 6
    psraw            m2, 6
    paddw            m3, m15
    paddw            m2, m15
    packuswb         m2, m3                 ; pack+unpack = clip
%endif
%if %1 || %2 == 0
    punpcklbw        m3, m2, m7
    punpckhbw        m0, m2, m7
%endif

    ; scaling[luma_src]
    pandn            m4, m8, m3
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m0
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    psrld            m0, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m0-2], m6
    pblendw          m2, m4, 0xaa
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
%if %2
    movu            xm5, [grain_lutq+offxyq+ 0]
    vinserti128      m5, [grain_lutq+offxyq+82], 1
%else
    movu             m5, [grain_lutq+offxyq]
%endif
    punpcklbw        m4, m5, m7
    punpckhbw        m5, m7

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; unpack chroma_source
    punpcklbw        m0, m1, m7
    punpckhbw        m1, m7

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
%if %2
    mova         [dstq], xm0
    vextracti128 [dstq+strideq], m0, 1
%else
    mova         [dstq], m0
%endif

%if %2
    lea            srcq, [srcq+strideq*2]
    lea            dstq, [dstq+strideq*2]
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
%else
    add            srcq, strideq
    add            dstq, strideq
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82<<%2
    sub              hb, 1+%2
    jg %%loop_y

    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r14+wq*(1+%2)]
    add            srcq, wq
    add            dstq, wq
    test       overlapd, overlapd
    jz %%loop_x

    ; r8m = sbym
    cmp       dword r8m, 0
    jne %%loop_x_hv_overlap

    ; horizontal overlap (without vertical overlap)
%%loop_x_h_overlap:
    rorx             r6, seeq, 1
    or             seed, 0xEFF4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d               ; updated seed

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, unused1, unused2, lstride

    lea     left_offxyd, [offyq+(32>>%2)]         ; previous column's offy*stride+offx
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyd, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)]  ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, unused1, unused2, lstride

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
%%loop_y_h_overlap:
    ; src
%if %2
    mova            xm3, [lumaq+lstrideq*0+ 0]
    vinserti128      m3, [lumaq+lstrideq*(1+%3)+ 0], 1
    vpbroadcastd     m2, [pb_1]
    mova            xm0, [lumaq+lstrideq*0+16]
    vinserti128      m0, [lumaq+lstrideq*(1+%3)+16], 1
    mova            xm1, [srcq]
    vinserti128      m1, [srcq+strideq], 1
    pmaddubsw        m3, m2
    pmaddubsw        m0, m2
    pavgw            m3, m7
    pavgw            m0, m7
%else
    mova             m2, [lumaq]
    mova             m1, [srcq]
%endif
%if %1
%if %2
    packuswb         m2, m3, m0             ; luma
%endif
    punpckhbw        m3, m2, m1
    punpcklbw        m2, m1                 ; { luma, chroma }
    pmaddubsw        m3, m14
    pmaddubsw        m2, m14
    psraw            m3, 6
    psraw            m2, 6
    paddw            m3, m15
    paddw            m2, m15
    packuswb         m2, m3                 ; pack+unpack = clip
%endif
%if %1 || %2 == 0
    punpcklbw        m3, m2, m7
    punpckhbw        m0, m2, m7
%endif

    ; scaling[luma_src]
    pandn            m4, m8, m3
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m0
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    psrld            m0, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m0-2], m6
    pblendw          m2, m4, 0xaa
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
%if %2
    movu            xm5, [grain_lutq+offxyq+ 0]
    vinserti128      m5, [grain_lutq+offxyq+82], 1
    movd            xm4, [grain_lutq+left_offxyq+ 0]
    vinserti128      m4, [grain_lutq+left_offxyq+82], 1
    punpcklbw        m4, m5
%if %1
    vpbroadcastq     m0, [pb_23_22]
    pmaddubsw        m4, m0, m4
%else
    pmaddubsw        m4, m15, m4
%endif
    pmulhrsw         m4, m12
    packsswb         m4, m4
    vpblendd         m4, m5, 0xee
%else
    movu             m5, [grain_lutq+offxyq]
    movd            xm4, [grain_lutq+left_offxyq]
    punpcklbw       xm4, xm5
%if %1
    movq            xm0, [pb_27_17_17_27]
    pmaddubsw       xm4, xm0, xm4
%else
    pmaddubsw       xm4, xm15, xm4
%endif
    pmulhrsw        xm4, xm12
    packsswb        xm4, xm4
    vpblendd         m4, m5, 0xfe
%endif
    punpckhbw        m5, m7
    punpcklbw        m4, m7

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; unpack chroma_source
    punpcklbw        m0, m1, m7
    punpckhbw        m1, m7

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
%if %2
    mova         [dstq], xm0
    vextracti128 [dstq+strideq], m0, 1
%else
    mova         [dstq], m0
%endif

%if %2
    lea            srcq, [srcq+strideq*2]
    lea            dstq, [dstq+strideq*2]
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
%else
    add            srcq, strideq
    add            dstq, strideq
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82*(1+%2)
    sub              hb, 1+%2
    jg %%loop_y_h_overlap

    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r14+wq*(1+%2)]
    add            srcq, wq
    add            dstq, wq

    ; r8m = sbym
    cmp       dword r8m, 0
    jne %%loop_x_hv_overlap
    jmp %%loop_x_h_overlap

%%vertical_overlap:
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, unused, \
                sby, see, overlap, unused1, unused2, lstride

    movzx          sbyd, sbyb
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
    imul            r7d, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add             r7d, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and             r7d, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, r7d
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                unused1, unused2, see, overlap, unused3, unused4, lstride

    mov           lumaq, r9mp
    lea             r12, [srcq+wq]
    lea             r13, [dstq+wq]
    lea             r14, [lumaq+wq*(1+%2)]
    mov           r11mp, r12
    mov           r12mp, r13
    mov        lstrideq, r10mp
    neg              wq

%%loop_x_v_overlap:
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, overlap, top_offxy, unused, lstride

    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, overlap, top_offxy, unused, lstride

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16
%if %2 == 0
    vpbroadcastd    m13, [pb_27_17]
%endif
%%loop_y_v_overlap:
    ; src
%if %2
    mova            xm3, [lumaq+lstrideq*0+ 0]
    vinserti128      m3, [lumaq+lstrideq*(1+%3)+ 0], 1
    vpbroadcastd     m2, [pb_1]
    mova            xm0, [lumaq+lstrideq*0+16]
    vinserti128      m0, [lumaq+lstrideq*(1+%3)+16], 1
    mova            xm1, [srcq]
    vinserti128      m1, [srcq+strideq], 1
    pmaddubsw        m3, m2
    pmaddubsw        m0, m2
    pavgw            m3, m7
    pavgw            m0, m7
%else
    mova             m2, [lumaq]
    mova             m1, [srcq]
%endif
%if %1
%if %2
    packuswb         m2, m3, m0             ; luma
%endif
    punpckhbw        m3, m2, m1
    punpcklbw        m2, m1                 ; { luma, chroma }
    pmaddubsw        m3, m14
    pmaddubsw        m2, m14
    psraw            m3, 6
    psraw            m2, 6
    paddw            m3, m15
    paddw            m2, m15
    packuswb         m2, m3                 ; pack+unpack = clip
%endif
%if %1 || %2 == 0
    punpcklbw        m3, m2, m7
    punpckhbw        m0, m2, m7
%endif

    ; scaling[luma_src]
    pandn            m4, m8, m3
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m0
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    psrld            m0, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m0-2], m6
    pblendw          m2, m4, 0xaa
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
%if %3 == 0
%if %2
    movu            xm0, [grain_lutq+offxyq]
    vinserti128      m0, [grain_lutq+offxyq+82], 1
    movu            xm4, [grain_lutq+top_offxyq]
    vinserti128      m4, [grain_lutq+top_offxyq+82], 1
%else
    movu             m0, [grain_lutq+offxyq]
    movu             m4, [grain_lutq+top_offxyq]
%endif
    punpcklbw        m5, m4, m0
    punpckhbw        m4, m0
    pmaddubsw        m5, m13, m5
    pmaddubsw        m4, m13, m4
    pmulhrsw         m5, m12
    pmulhrsw         m4, m12
    packsswb         m5, m4
%else
    movq            xm4, [grain_lutq+offxyq]
    vinserti128      m4, [grain_lutq+offxyq+8], 1
    movq            xm5, [grain_lutq+top_offxyq]
    vinserti128      m5, [grain_lutq+top_offxyq+8], 1
    punpcklbw        m5, m4
    pmaddubsw        m5, m13, m5
    pmulhrsw         m5, m12
    vextracti128    xm4, m5, 1
    packsswb        xm5, xm4
    ; only interpolate first line, insert second line unmodified
    vinserti128      m5, [grain_lutq+offxyq+82], 1
%endif
    punpcklbw        m4, m5, m7
    punpckhbw        m5, m7

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; unpack chroma_source
    punpcklbw        m0, m1, m7
    punpckhbw        m1, m7

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
%if %2
    mova         [dstq], xm0
    vextracti128 [dstq+strideq], m0, 1
%else
    mova         [dstq], m0
%endif

    sub              hb, 1+%2
    jle %%end_y_v_overlap
%if %2
    lea            srcq, [srcq+strideq*2]
    lea            dstq, [dstq+strideq*2]
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
%else
    add            srcq, strideq
    add            dstq, strideq
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82<<%2
%if %2 == 0
    vpbroadcastd    m13, [pb_17_27]
    add              hd, 0x80000000
    jnc %%loop_y_v_overlap
%endif
    jmp %%loop_y

%%end_y_v_overlap:
    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r14+wq*(1+%2)]
    add            srcq, wq
    add            dstq, wq

    ; since fg_dataq.overlap is guaranteed to be set, we never jump
    ; back to .loop_x_v_overlap, and instead always fall-through to
    ; h+v overlap

%%loop_x_hv_overlap:
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                offx, offy, see, left_offxy, top_offxy, topleft_offxy, lstride

    lea  topleft_offxyd, [top_offxyq+(32>>%2)]
    lea     left_offxyd, [offyq+(32>>%2)]
    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

    DEFINE_ARGS dst, src, stride, luma, w, scaling, grain_lut, \
                h, offxy, see, left_offxy, top_offxy, topleft_offxy, lstride

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16
%if %2 == 0
    vpbroadcastd    m13, [pb_27_17]
%endif
%%loop_y_hv_overlap:
    ; src
%if %2
    mova            xm3, [lumaq+lstrideq*0+ 0]
    vinserti128      m3, [lumaq+lstrideq*(1+%3)+ 0], 1
    vpbroadcastd     m2, [pb_1]
    mova            xm0, [lumaq+lstrideq*0+16]
    vinserti128      m0, [lumaq+lstrideq*(1+%3)+16], 1
    mova            xm1, [srcq]
    vinserti128      m1, [srcq+strideq], 1
    pmaddubsw        m3, m2
    pmaddubsw        m0, m2
    pavgw            m3, m7
    pavgw            m0, m7
%else
    mova             m2, [lumaq]
    mova             m1, [srcq]
%endif
%if %1
%if %2
    packuswb         m2, m3, m0             ; luma
%endif
    punpckhbw        m3, m2, m1
    punpcklbw        m2, m1                 ; { luma, chroma }
    pmaddubsw        m3, m14
    pmaddubsw        m2, m14
    psraw            m3, 6
    psraw            m2, 6
    paddw            m3, m15
    paddw            m2, m15
    packuswb         m2, m3                 ; pack+unpack = clip
%endif
%if %1 || %2 == 0
    punpcklbw        m3, m2, m7
    punpckhbw        m0, m2, m7
%endif

    ; scaling[luma_src]
    pandn            m4, m8, m3
    mova             m6, m8
    vpgatherdd       m2, [scalingq+m4-0], m8
    psrld            m3, 16
    mova             m8, m6
    vpgatherdd       m4, [scalingq+m3-2], m6
    pandn            m5, m8, m0
    mova             m6, m8
    vpgatherdd       m3, [scalingq+m5-0], m8
    psrld            m0, 16
    mova             m8, m6
    vpgatherdd       m5, [scalingq+m0-2], m6
    pblendw          m2, m4, 0xaa
    pblendw          m3, m5, 0xaa

    ; grain = grain_lut[offy+y][offx+x]
%if %2
    movu            xm4, [grain_lutq+offxyq]
    vinserti128      m4, [grain_lutq+offxyq+82], 1
    movd            xm0, [grain_lutq+left_offxyq]
    vinserti128      m0, [grain_lutq+left_offxyq+82], 1
    movd            xm6, [grain_lutq+topleft_offxyq]
%if %3
    movq            xm5, [grain_lutq+top_offxyq]
    vinserti128      m5, [grain_lutq+top_offxyq+8], 1
%else
    vinserti128      m6, [grain_lutq+topleft_offxyq+82], 1
    movu            xm5, [grain_lutq+top_offxyq]
    vinserti128      m5, [grain_lutq+top_offxyq+82], 1
%endif

    ; do h interpolation first (so top | top/left -> top, left | cur -> cur)
    punpcklbw        m0, m4
%if %3
    punpcklbw       xm6, xm5
%else
    punpcklbw        m6, m5
%endif
    punpcklqdq       m0, m6
%if %1
    vpbroadcastq     m6, [pb_23_22]
    pmaddubsw        m0, m6, m0
%else
    pmaddubsw        m0, m15, m0
%endif
    pmulhrsw         m0, m12
    packsswb         m0, m0
    vpblendd         m4, m0, 0x11
%if %3
    pshuflw         xm0, xm0, q1032
    vpblendd         m5, m0, 0x01
%else
    pshuflw          m0, m0, q1032
    vpblendd         m5, m0, 0x11
%endif
%else
    movu             m4, [grain_lutq+offxyq]
    movd            xm0, [grain_lutq+left_offxyq]
    movu             m5, [grain_lutq+top_offxyq]
    movd            xm6, [grain_lutq+topleft_offxyq]
    punpcklbw       xm0, xm4
    punpcklbw       xm6, xm5
    punpcklqdq      xm0, xm6
%if %1
    vpbroadcastq    xm6, [pb_27_17_17_27]
    pmaddubsw       xm0, xm6, xm0
%else
    pmaddubsw       xm0, xm15, xm0
%endif
    pmulhrsw        xm0, xm12
    packsswb        xm0, xm0
    vpblendd         m4, m0, 0x01
    pshuflw         xm0, xm0, q1032
    vpblendd         m5, m0, 0x01
%endif

    ; followed by v interpolation (top | cur -> cur)
%if %3
    vpermq           m0, m4, q3120
    punpcklbw        m5, m0
    pmaddubsw        m5, m13, m5
    pmulhrsw         m5, m12
    vextracti128    xm0, m5, 1
    packsswb        xm5, xm0
    vpblendd         m5, m4, 0xf0
%else
    punpckhbw        m0, m5, m4
    punpcklbw        m5, m4
    pmaddubsw        m4, m13, m0
    pmaddubsw        m5, m13, m5
    pmulhrsw         m4, m12
    pmulhrsw         m5, m12
    packsswb         m5, m4
%endif
    punpcklbw        m4, m5, m7
    punpckhbw        m5, m7

    ; noise = round2(scaling[src] * grain, scaling_shift)
    pmaddubsw        m2, m4
    pmaddubsw        m3, m5
    pmulhrsw         m2, m9
    pmulhrsw         m3, m9

    ; unpack chroma source
    punpcklbw        m0, m1, m7
    punpckhbw        m1, m7

    ; dst = clip_pixel(src, noise)
    paddw            m0, m2
    paddw            m1, m3
    packuswb         m0, m1
    pmaxub           m0, m10
    pminub           m0, m11
%if %2
    mova         [dstq], xm0
    vextracti128 [dstq+strideq], m0, 1
%else
    mova         [dstq], m0
%endif

%if %2
    lea            srcq, [srcq+strideq*2]
    lea            dstq, [dstq+strideq*2]
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
%else
    add            srcq, strideq
    add            dstq, strideq
    add           lumaq, lstrideq
%endif
    add      grain_lutq, 82<<%2
    sub              hb, 1+%2
%if %2
    jg %%loop_y_h_overlap
%else
    je %%end_y_hv_overlap
    vpbroadcastd    m13, [pb_17_27]
    add              hd, 0x80000000
    jnc %%loop_y_hv_overlap
    jmp %%loop_y_h_overlap
%endif

%%end_y_hv_overlap:
    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r14+wq*(1+%2)]
    add            srcq, wq
    add            dstq, wq
    jmp %%loop_x_hv_overlap
%endmacro

    %%FGUV_32x32xN_LOOP 1, %2, %3
.csfl:
    %%FGUV_32x32xN_LOOP 0, %2, %3
.end:
    RET
%endmacro

GEN_GRAIN_UV_FN 420, 1, 1
FGUV_FN         420, 1, 1
GEN_GRAIN_UV_FN 422, 1, 0
FGUV_FN         422, 1, 0
GEN_GRAIN_UV_FN 444, 0, 0
FGUV_FN         444, 0, 0

%endif ; ARCH_X86_64
