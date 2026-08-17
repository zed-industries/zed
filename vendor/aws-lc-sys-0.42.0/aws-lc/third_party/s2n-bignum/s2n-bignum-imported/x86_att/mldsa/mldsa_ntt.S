// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Forward number-theoretic transform for ML-DSA
// Input a[256], zetas (all signed 32-bit words); output a[256] (signed 32-bit words)
//
// The transform is in-place with input and output a[256], with the output
// in bitreversed order. Computes the forward NTT of a polynomial in place,
// transforming from coefficient representation to NTT representation. The input
// polynomial a is a 256-element array of signed 32-bit integers representing
// coefficients in normal order, assumed to be coefficient-wise bound by MLDSA_Q
// (8380417) in absolute value. The zetas parameter points to precomputed twiddle
// factors (roots of unity and their Montgomery inverses) required for the NTT
// computation. The output polynomial is in bitreversed order and coefficient-wise
// bound by 42035261 (just over 5*Q) in absolute value. Output coefficients are
// congruent to the mathematical NTT result modulo Q but are not fully reduced.
//
// The implementation uses Montgomery arithmetic for efficient modular reduction
// modulo Q = 8380417, maintaining constant-time execution for side-channel
// resistance. The transform operates in the ring R_Q = Z_Q[x]/(x^256 + 1).
//
// extern void mldsa_ntt(int32_t a[static 256], const int32_t zetas[static 624]);
//
// Standard x86-64 ABI: RDI = a, RSI = zetas
// Microsoft x64 ABI:   RCX = a, RDX = zetas
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_ntt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mldsa_ntt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_ntt)
        .text

#define a %rdi
#define zetas %rsi

#define MLD_AVX2_BACKEND_DATA_OFFSET_8XQ 0
#define MLD_AVX2_BACKEND_DATA_OFFSET_8XQINV 8
#define MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV_QINV 16
#define MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV 24
#define MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV 32
#define MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS 328

.macro shuffle8 r0,r1,r2,r3
        vperm2i128      $0x20, \r1, \r0, \r2
        vperm2i128      $0x31, \r1, \r0, \r3
.endm

.macro shuffle4 r0,r1,r2,r3
        vpunpcklqdq     \r1, \r0, \r2
        vpunpckhqdq     \r1, \r0, \r3
.endm

.macro shuffle2 r0,r1,r2,r3
        vmovsldup       \r1, \r2
        vpblendd        $0xAA, \r2, \r0, \r2
        vpsrlq          $32, \r0, \r0
        vpblendd        $0xAA, \r1, \r0, \r3
.endm

.macro butterfly l,h,zl0=%ymm1,zl1=%ymm1,zh0=%ymm2,zh1=%ymm2
        vpmuldq         \zl0, \h, %ymm13
        vmovshdup       \h, %ymm12
        vpmuldq         \zl1, %ymm12, %ymm14

        vpmuldq         \zh0, \h, \h
        vpmuldq         \zh1, %ymm12, %ymm12

        vpmuldq         %ymm0, %ymm13, %ymm13
        vpmuldq         %ymm0, %ymm14, %ymm14

        vmovshdup       \h, \h
        vpblendd        $0xAA, %ymm12, \h, \h

        vpsubd          \h, \l, %ymm12
        vpaddd          \h, \l, \l

        vmovshdup       %ymm13, %ymm13
        vpblendd        $0xAA, %ymm14, %ymm13, %ymm13

        vpaddd          %ymm13, %ymm12, \h
        vpsubd          %ymm13, \l, \l
.endm

.macro levels0t1 off
        /* level 0 */
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+1))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+1))(%rsi), %ymm2

        vmovdqa         0+32*\off(%rdi), %ymm4
        vmovdqa         128+32*\off(%rdi), %ymm5
        vmovdqa         256+32*\off(%rdi), %ymm6
        vmovdqa         384+32*\off(%rdi), %ymm7
        vmovdqa         512+32*\off(%rdi), %ymm8
        vmovdqa         640+32*\off(%rdi), %ymm9
        vmovdqa         768+32*\off(%rdi), %ymm10
        vmovdqa         896+32*\off(%rdi), %ymm11

        butterfly       %ymm4,%ymm8
        butterfly       %ymm5,%ymm9
        butterfly       %ymm6,%ymm10
        butterfly       %ymm7,%ymm11

        /* level 1 */
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+2))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+2))(%rsi), %ymm2
        butterfly       %ymm4,%ymm6
        butterfly       %ymm5,%ymm7

        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+3))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+3))(%rsi), %ymm2
        butterfly       %ymm8,%ymm10
        butterfly       %ymm9,%ymm11

        vmovdqa         %ymm4, 0+32*\off(%rdi)
        vmovdqa         %ymm5, 128+32*\off(%rdi)
        vmovdqa         %ymm6, 256+32*\off(%rdi)
        vmovdqa         %ymm7, 384+32*\off(%rdi)
        vmovdqa         %ymm8, 512+32*\off(%rdi)
        vmovdqa         %ymm9, 640+32*\off(%rdi)
        vmovdqa         %ymm10, 768+32*\off(%rdi)
        vmovdqa         %ymm11, 896+32*\off(%rdi)
.endm

.macro levels2t7 off
        /* level 2 */
        vmovdqa         0+256*\off(%rdi), %ymm4
        vmovdqa         32+256*\off(%rdi), %ymm5
        vmovdqa         64+256*\off(%rdi), %ymm6
        vmovdqa         96+256*\off(%rdi), %ymm7
        vmovdqa         128+256*\off(%rdi), %ymm8
        vmovdqa         160+256*\off(%rdi), %ymm9
        vmovdqa         192+256*\off(%rdi), %ymm10
        vmovdqa         224+256*\off(%rdi), %ymm11
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+4+\off))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+4+\off))(%rsi), %ymm2

        butterfly       %ymm4,%ymm8
        butterfly       %ymm5,%ymm9
        butterfly       %ymm6,%ymm10
        butterfly       %ymm7,%ymm11

        shuffle8        %ymm4,%ymm8,%ymm3,%ymm8
        shuffle8        %ymm5,%ymm9,%ymm4,%ymm9
        shuffle8        %ymm6,%ymm10,%ymm5,%ymm10
        shuffle8        %ymm7,%ymm11,%ymm6,%ymm11

        /* level 3 */
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+8+8*\off))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+8+8*\off))(%rsi), %ymm2

        butterfly       %ymm3,%ymm5
        butterfly       %ymm8,%ymm10
        butterfly       %ymm4,%ymm6
        butterfly       %ymm9,%ymm11

        shuffle4        %ymm3,%ymm5,%ymm7,%ymm5
        shuffle4        %ymm8,%ymm10,%ymm3,%ymm10
        shuffle4        %ymm4,%ymm6,%ymm8,%ymm6
        shuffle4        %ymm9,%ymm11,%ymm4,%ymm11

        /* level 4 */
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+40+8*\off))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+40+8*\off))(%rsi), %ymm2

        butterfly       %ymm7,%ymm8
        butterfly       %ymm5,%ymm6
        butterfly       %ymm3,%ymm4
        butterfly       %ymm10,%ymm11

        shuffle2        %ymm7,%ymm8,%ymm9,%ymm8
        shuffle2        %ymm5,%ymm6,%ymm7,%ymm6
        shuffle2        %ymm3,%ymm4,%ymm5,%ymm4
        shuffle2        %ymm10,%ymm11,%ymm3,%ymm11

        /* level 5 */
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+72+8*\off))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+72+8*\off))(%rsi), %ymm2
        vpsrlq          $32, %ymm1, %ymm10
        vmovshdup       %ymm2, %ymm15

        butterfly       %ymm9,%ymm5,%ymm1,%ymm10,%ymm2,%ymm15
        butterfly       %ymm8,%ymm4,%ymm1,%ymm10,%ymm2,%ymm15
        butterfly       %ymm7,%ymm3,%ymm1,%ymm10,%ymm2,%ymm15
        butterfly       %ymm6,%ymm11,%ymm1,%ymm10,%ymm2,%ymm15

        /* level 6 */
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+104+8*\off))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+104+8*\off))(%rsi), %ymm2
        vpsrlq          $32, %ymm1, %ymm10
        vmovshdup       %ymm2, %ymm15
        butterfly       %ymm9,%ymm7,%ymm1,%ymm10,%ymm2,%ymm15
        butterfly       %ymm8,%ymm6,%ymm1,%ymm10,%ymm2,%ymm15

        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+104+8*\off+32))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+104+8*\off+32))(%rsi), %ymm2
        vpsrlq          $32, %ymm1, %ymm10
        vmovshdup       %ymm2, %ymm15
        butterfly       %ymm5,%ymm3,%ymm1,%ymm10,%ymm2,%ymm15
        butterfly       %ymm4,%ymm11,%ymm1,%ymm10,%ymm2,%ymm15

        /* level 7 */
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+168+8*\off))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+168+8*\off))(%rsi), %ymm2
        vpsrlq          $32, %ymm1, %ymm10
        vmovshdup       %ymm2, %ymm15
        butterfly       %ymm9,%ymm8,%ymm1,%ymm10,%ymm2,%ymm15

        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+168+8*\off+32))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+168+8*\off+32))(%rsi), %ymm2
        vpsrlq          $32, %ymm1, %ymm10
        vmovshdup       %ymm2, %ymm15
        butterfly       %ymm7,%ymm6,%ymm1,%ymm10,%ymm2,%ymm15

        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+168+8*\off+64))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+168+8*\off+64))(%rsi), %ymm2
        vpsrlq          $32, %ymm1, %ymm10
        vmovshdup       %ymm2, %ymm15
        butterfly       %ymm5,%ymm4,%ymm1,%ymm10,%ymm2,%ymm15

        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+168+8*\off+96))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+168+8*\off+96))(%rsi), %ymm2
        vpsrlq          $32, %ymm1, %ymm10
        vmovshdup       %ymm2, %ymm15
        butterfly       %ymm3,%ymm11,%ymm1,%ymm10,%ymm2,%ymm15

        vmovdqa         %ymm9, 0+256*\off(%rdi)
        vmovdqa         %ymm8, 32+256*\off(%rdi)
        vmovdqa         %ymm7, 64+256*\off(%rdi)
        vmovdqa         %ymm6, 96+256*\off(%rdi)
        vmovdqa         %ymm5, 128+256*\off(%rdi)
        vmovdqa         %ymm4, 160+256*\off(%rdi)
        vmovdqa         %ymm3, 192+256*\off(%rdi)
        vmovdqa         %ymm11, 224+256*\off(%rdi)
.endm

S2N_BN_SYMBOL(mldsa_ntt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_DEC_RSP(176)
        CFI_STACKSAVEU(%xmm6,0)
        CFI_STACKSAVEU(%xmm7,16)
        CFI_STACKSAVEU(%xmm8,32)
        CFI_STACKSAVEU(%xmm9,48)
        CFI_STACKSAVEU(%xmm10,64)
        CFI_STACKSAVEU(%xmm11,80)
        CFI_STACKSAVEU(%xmm12,96)
        CFI_STACKSAVEU(%xmm13,112)
        CFI_STACKSAVEU(%xmm14,128)
        CFI_STACKSAVEU(%xmm15,144)
        CFI_STACKSAVE(%rdi,160)
        CFI_STACKSAVE(%rsi,168)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

        vmovdqa MLD_AVX2_BACKEND_DATA_OFFSET_8XQ*4(%rsi), %ymm0

        levels0t1 0
        levels0t1 1
        levels0t1 2
        levels0t1 3

        levels2t7 0
        levels2t7 1
        levels2t7 2
        levels2t7 3

#if WINDOWS_ABI
        CFI_STACKLOADU(%xmm6,0)
        CFI_STACKLOADU(%xmm7,16)
        CFI_STACKLOADU(%xmm8,32)
        CFI_STACKLOADU(%xmm9,48)
        CFI_STACKLOADU(%xmm10,64)
        CFI_STACKLOADU(%xmm11,80)
        CFI_STACKLOADU(%xmm12,96)
        CFI_STACKLOADU(%xmm13,112)
        CFI_STACKLOADU(%xmm14,128)
        CFI_STACKLOADU(%xmm15,144)
        CFI_STACKLOAD(%rdi,160)
        CFI_STACKLOAD(%rsi,168)
        CFI_INC_RSP(176)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mldsa_ntt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
