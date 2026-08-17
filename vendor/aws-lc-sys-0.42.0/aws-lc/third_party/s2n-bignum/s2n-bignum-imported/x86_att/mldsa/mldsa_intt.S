// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Inverse number-theoretic transform for ML-DSA
// Input a[256], zetas (all signed 32-bit words); output a[256] (signed 32-bit words)
//
// The transform is in-place with input and output a[256]. Computes the inverse
// NTT of a polynomial in place, transforming from NTT representation to coefficient
// representation. The input polynomial a is a 256-element array of signed 32-bit
// integers representing coefficients in NTT domain, with each coefficient bounded
// by |a[i]| < Q. The zetas parameter points to precomputed twiddle factors
// (roots of unity and their Montgomery inverses) required for the inverse NTT
// computation.
//
// The output polynomial is in coefficient representation (normal order) with
// coefficients in Montgomery form, each bounded by |a[i]| < Q.
// The inverse NTT includes the division by 256 (the transform size) and maps the
// result to Montgomery domain via multiplication by 2^32/256 = 2^24.
//
// The implementation uses Montgomery arithmetic for efficient modular reduction
// modulo Q = 8380417, maintaining constant-time execution for side-channel
// resistance. The transform operates in the ring R_Q = Z_Q[x]/(x^256 + 1).
//
// extern void mldsa_intt(int32_t a[static 256], const int32_t zetas[static 624]);
//
// Standard x86-64 ABI: RDI = a, RSI = zetas
// Microsoft x64 ABI:   RCX = a, RDX = zetas
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_intt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mldsa_intt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_intt)
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

/*
 * Compute l' = l + h, h' = montmul(h - l, zh)
 *
 * Bounds: |l'| <= |l| + |h|
 *         |h'| < MONTMUL_BOUND
 *         (See the end of this file for the exact value of MONTMUL_BOUND)
 *
 * The flip parameter controls VEX encoding optimization for platform-independent
 * code. Set flip=1 when h is in %ymm0-7 and l is in %ymm8-15 to enable the 2-byte
 * VEX encoding; otherwise set flip=0.
 *
 * VEX Encoding Optimization for Platform-Independent Code
 *
 * Some assemblers (notably clang) will automatically swap operands of
 * commutative instructions like VPADDD to use shorter encodings, while others
 * (like gcc) may not. This causes different machine code across platforms.
 *
 * VEX prefixes come in two forms:
 * - 2-byte VEX (0xC5): Can only be used when the ModR/M.rm operand is %ymm0-7
 * - 3-byte VEX (0xC4): Required when ModR/M.rm operand is %ymm8-15
 *
 * When one operand is in %ymm0-7 and another is in %ymm8-15, we explicitly
 * place the lower-numbered register (%ymm0-7) as the second source operand
 * to enable the 2-byte VEX encoding. Since VPADDD is commutative, this
 * produces identical results while ensuring consistent machine code across
 * different assemblers.
 *
 * Example:
 *   VPADDD %ymm4, %ymm4, %ymm8  -> 3-byte VEX (0xC4 0xC1 0x5D 0xFE 0xE0)
 *   VPADDD %ymm4, %ymm8, %ymm4  -> 2-byte VEX (0xC5 0xBD 0xFE 0xE4) preferred
 */
.macro butterfly flip,l,h,zl0=%ymm1,zl1=%ymm1,zh0=%ymm2,zh1=%ymm2
        vpsubd          \l, \h, %ymm12
.if \flip
        vpaddd          \h, \l, \l
.else
        vpaddd          \l, \h, \l
.endif

        vpmuldq         \zl0, %ymm12, %ymm13
        vmovshdup       %ymm12, \h
        vpmuldq         \zl1, \h, %ymm14

        vpmuldq         \zh0, %ymm12, %ymm12
        vpmuldq         \zh1, \h, \h

        vpmuldq         %ymm0, %ymm13, %ymm13
        vpmuldq         %ymm0, %ymm14, %ymm14

        vpsubd          %ymm13, %ymm12, %ymm12
        vpsubd          %ymm14, \h, \h

        vmovshdup       %ymm12, %ymm12
        vpblendd        $0xAA, \h, %ymm12, \h
.endm

.macro levels0t5 off
        vmovdqa         256*\off+0(%rdi), %ymm4
        vmovdqa         256*\off+32(%rdi), %ymm5
        vmovdqa         256*\off+64(%rdi), %ymm6
        vmovdqa         256*\off+96(%rdi), %ymm7
        vmovdqa         256*\off+128(%rdi), %ymm8
        vmovdqa         256*\off+160(%rdi), %ymm9
        vmovdqa         256*\off+192(%rdi), %ymm10
        vmovdqa         256*\off+224(%rdi), %ymm11

        /* Bounds: |ymm{i}| < q for i in 4...11 */

        /* level 0 */
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+296-8*\off-8))(%rsi), %ymm3
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+296-8*\off-8))(%rsi), %ymm15
        vmovshdup       %ymm3, %ymm1
        vmovshdup       %ymm15, %ymm2
        butterfly       0,%ymm4,%ymm5,%ymm1,%ymm3,%ymm2,%ymm15

        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+296-8*\off-40))(%rsi), %ymm3
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+296-8*\off-40))(%rsi), %ymm15
        vmovshdup       %ymm3, %ymm1
        vmovshdup       %ymm15, %ymm2
        butterfly       0,%ymm6,%ymm7,%ymm1,%ymm3,%ymm2,%ymm15

        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+296-8*\off-72))(%rsi), %ymm3
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+296-8*\off-72))(%rsi), %ymm15
        vmovshdup       %ymm3, %ymm1
        vmovshdup       %ymm15, %ymm2
        butterfly       0,%ymm8,%ymm9,%ymm1,%ymm3,%ymm2,%ymm15

        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+296-8*\off-104))(%rsi), %ymm3
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+296-8*\off-104))(%rsi), %ymm15
        vmovshdup       %ymm3, %ymm1
        vmovshdup       %ymm15, %ymm2
        butterfly       0,%ymm10,%ymm11,%ymm1,%ymm3,%ymm2,%ymm15

        /*
         * Bounds: |ymm{i}| < 2q            for i in 4, 6, 8, 10
         *                  < MONTMUL_BOUND for i in 5, 7, 9, 11
         *
         * Note that since 2^31 / q > 256, the sum of all 256 coefficients does not
         * overflow. This allows us to greatly simplify the range analysis by relaxing
         * and unifying the bounds of all coefficients on the same layer. As a concrete
         * example, here we relax the bounds on 5, 7, 9, 11 and conclude that (for all
         * relevant i)
         *
         * Bounds: |ymm{i}| < 2q
         *
         * In all but last of the following levels of butterflies, we do the same
         * relaxation without explicit mention.
         */

        /* level 1 */
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+168-8*\off-8))(%rsi), %ymm3
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+168-8*\off-8))(%rsi), %ymm15
        vmovshdup       %ymm3, %ymm1
        vmovshdup       %ymm15, %ymm2
        butterfly       0,%ymm4,%ymm6,%ymm1,%ymm3,%ymm2,%ymm15
        butterfly       0,%ymm5,%ymm7,%ymm1,%ymm3,%ymm2,%ymm15

        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+168-8*\off-40))(%rsi), %ymm3
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+168-8*\off-40))(%rsi), %ymm15
        vmovshdup       %ymm3, %ymm1
        vmovshdup       %ymm15, %ymm2
        butterfly       0,%ymm8,%ymm10,%ymm1,%ymm3,%ymm2,%ymm15
        butterfly       0,%ymm9,%ymm11,%ymm1,%ymm3,%ymm2,%ymm15

        /* Bounds: |ymm{i}| < 4q */

        /* level 2 */
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+104-8*\off-8))(%rsi), %ymm3
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+104-8*\off-8))(%rsi), %ymm15
        vmovshdup       %ymm3, %ymm1
        vmovshdup       %ymm15, %ymm2
        butterfly       0,%ymm4,%ymm8,%ymm1,%ymm3,%ymm2,%ymm15
        butterfly       0,%ymm5,%ymm9,%ymm1,%ymm3,%ymm2,%ymm15
        butterfly       0,%ymm6,%ymm10,%ymm1,%ymm3,%ymm2,%ymm15
        butterfly       0,%ymm7,%ymm11,%ymm1,%ymm3,%ymm2,%ymm15

        /* Bounds: |ymm{i}| < 8q */

        /* level 3 */
        shuffle2        %ymm4,%ymm5,%ymm3,%ymm5
        shuffle2        %ymm6,%ymm7,%ymm4,%ymm7
        shuffle2        %ymm8,%ymm9,%ymm6,%ymm9
        shuffle2        %ymm10,%ymm11,%ymm8,%ymm11

        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+72-8*\off-8))(%rsi), %ymm1
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+72-8*\off-8))(%rsi), %ymm2
        butterfly       0,%ymm3,%ymm5
        butterfly       0,%ymm4,%ymm7
        butterfly       0,%ymm6,%ymm9
        butterfly       0,%ymm8,%ymm11

        /* Bounds: |ymm{i}| < 16q */

        /* level 4 */
        shuffle4        %ymm3,%ymm4,%ymm10,%ymm4
        shuffle4        %ymm6,%ymm8,%ymm3,%ymm8
        shuffle4        %ymm5,%ymm7,%ymm6,%ymm7
        shuffle4        %ymm9,%ymm11,%ymm5,%ymm11

        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+40-8*\off-8))(%rsi), %ymm1
        vpermq          $0x1B, (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+40-8*\off-8))(%rsi), %ymm2
        butterfly       1,%ymm10,%ymm4
        butterfly       0,%ymm3,%ymm8
        butterfly       0,%ymm6,%ymm7
        butterfly       0,%ymm5,%ymm11

        /* Bounds: |ymm{i}| < 32q */

        /* level 5 */
        shuffle8        %ymm10,%ymm3,%ymm9,%ymm3
        shuffle8        %ymm6,%ymm5,%ymm10,%ymm5
        shuffle8        %ymm4,%ymm8,%ymm6,%ymm8
        shuffle8        %ymm7,%ymm11,%ymm4,%ymm11

        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+7-\off))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+7-\off))(%rsi), %ymm2
        butterfly       1,%ymm9,%ymm3
        butterfly       1,%ymm10,%ymm5
        butterfly       0,%ymm6,%ymm8
        butterfly       0,%ymm4,%ymm11

        /* Bounds: |ymm{i}| < 64q */

        vmovdqa         %ymm9, 256*\off+0(%rdi)
        vmovdqa         %ymm10, 256*\off+32(%rdi)
        vmovdqa         %ymm6, 256*\off+64(%rdi)
        vmovdqa         %ymm4, 256*\off+96(%rdi)
        vmovdqa         %ymm3, 256*\off+128(%rdi)
        vmovdqa         %ymm5, 256*\off+160(%rdi)
        vmovdqa         %ymm8, 256*\off+192(%rdi)
        vmovdqa         %ymm11, 256*\off+224(%rdi)
.endm

.macro levels6t7 off
        vmovdqa         0+32*\off(%rdi), %ymm4
        vmovdqa         128+32*\off(%rdi), %ymm5
        vmovdqa         256+32*\off(%rdi), %ymm6
        vmovdqa         384+32*\off(%rdi), %ymm7
        vmovdqa         512+32*\off(%rdi), %ymm8
        vmovdqa         640+32*\off(%rdi), %ymm9
        vmovdqa         768+32*\off(%rdi), %ymm10
        vmovdqa         896+32*\off(%rdi), %ymm11

        /* level 6 */
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+3))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+3))(%rsi), %ymm2
        butterfly       0,%ymm4,%ymm6
        butterfly       0,%ymm5,%ymm7

        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+2))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+2))(%rsi), %ymm2
        butterfly       0,%ymm8,%ymm10
        butterfly       0,%ymm9,%ymm11

        /* Bounds: |ymm{i}| < 128q */

        /* level 7 */
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV+0))(%rsi), %ymm1
        vpbroadcastd    (4*(MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS+0))(%rsi), %ymm2

        butterfly       0,%ymm4,%ymm8
        butterfly       0,%ymm5,%ymm9
        butterfly       0,%ymm6,%ymm10
        butterfly       0,%ymm7,%ymm11

        /*
         * Bounds: |ymm{i}| < 256q          for i in 4...7
         *                  < MONTMUL_BOUND for i in 8...11
         */

        vmovdqa         %ymm8, 512+32*\off(%rdi)
        vmovdqa         %ymm9, 640+32*\off(%rdi)
        vmovdqa         %ymm10, 768+32*\off(%rdi)
        vmovdqa         %ymm11, 896+32*\off(%rdi)

        /*
         * In order to (a) remove the factor of 256 arising from the 256-point INTT
         * butterflies and (b) transform the output into Montgomery domain, we need to
         * multiply all coefficients by 2^32/256.
         *
         * For %ymm{8,9,10,11}, the scaling has been merged into the last butterfly, so
         * only %ymm{4,5,6,7} need to be scaled explicitly.
         *
         * The scaling is achieved by computing montmul(-, MLD_AVX2_DIV).
         *
         * Bounds: |ymm{i}| < 256q for i in 4...7
         */

        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV_QINV))(%rsi), %ymm1
        vmovdqa         (4*(MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV))(%rsi), %ymm2
        vpmuldq         %ymm1, %ymm4, %ymm12
        vpmuldq         %ymm1, %ymm5, %ymm13
        vmovshdup       %ymm4, %ymm8
        vmovshdup       %ymm5, %ymm9
        vpmuldq         %ymm1, %ymm8, %ymm14
        vpmuldq         %ymm1, %ymm9, %ymm15
        vpmuldq         %ymm2, %ymm4, %ymm4
        vpmuldq         %ymm2, %ymm5, %ymm5
        vpmuldq         %ymm2, %ymm8, %ymm8
        vpmuldq         %ymm2, %ymm9, %ymm9
        vpmuldq         %ymm0, %ymm12, %ymm12
        vpmuldq         %ymm0, %ymm13, %ymm13
        vpmuldq         %ymm0, %ymm14, %ymm14
        vpmuldq         %ymm0, %ymm15, %ymm15
        vpsubd          %ymm12, %ymm4, %ymm4
        vpsubd          %ymm13, %ymm5, %ymm5
        vpsubd          %ymm14, %ymm8, %ymm8
        vpsubd          %ymm15, %ymm9, %ymm9
        vmovshdup       %ymm4, %ymm4
        vmovshdup       %ymm5, %ymm5
        vpblendd        $0xAA, %ymm8, %ymm4, %ymm4
        vpblendd        $0xAA, %ymm9, %ymm5, %ymm5

        vpmuldq         %ymm1, %ymm6, %ymm12
        vpmuldq         %ymm1, %ymm7, %ymm13
        vmovshdup       %ymm6, %ymm8
        vmovshdup       %ymm7, %ymm9
        vpmuldq         %ymm1, %ymm8, %ymm14
        vpmuldq         %ymm1, %ymm9, %ymm15
        vpmuldq         %ymm2, %ymm6, %ymm6
        vpmuldq         %ymm2, %ymm7, %ymm7
        vpmuldq         %ymm2, %ymm8, %ymm8
        vpmuldq         %ymm2, %ymm9, %ymm9
        vpmuldq         %ymm0, %ymm12, %ymm12
        vpmuldq         %ymm0, %ymm13, %ymm13
        vpmuldq         %ymm0, %ymm14, %ymm14
        vpmuldq         %ymm0, %ymm15, %ymm15
        vpsubd          %ymm12, %ymm6, %ymm6
        vpsubd          %ymm13, %ymm7, %ymm7
        vpsubd          %ymm14, %ymm8, %ymm8
        vpsubd          %ymm15, %ymm9, %ymm9
        vmovshdup       %ymm6, %ymm6
        vmovshdup       %ymm7, %ymm7
        vpblendd        $0xAA, %ymm8, %ymm6, %ymm6
        vpblendd        $0xAA, %ymm9, %ymm7, %ymm7

        /* Bounds: |ymm{i}| < MONTMUL_BOUND for i in 4...7 */

        /*
         * In the following we show that |montmul(a, b)| < MONTMUL_BOUND := ceil(3q/4),
         * given that a fits inside int32_t (thus |a| <= R/2) and b is signed canonical
         * (in the range -(Q-1)/2...(Q-1)/2).
         *
         * In Section 2.2 of https://eprint.iacr.org/2023/1962, they showed that b being
         * signed canonical implies:
         *
         *   |montmul(a, b)| <= (|a| (q/2) + (R/2) q) / R = (q/2) (1 + |a|/R).
         *
         * From this, simple computation gives |montmul(a, b)| <= 3q/4 < ceil(3q/4).
         *
         * See test/test_bounds.py for more empirical evidence (and some minor technical
         * details).
         *
         * TODO: Use proper citation. Currently, citations within asm can cause linter
         *       to complain about unused citation, because comments are not preserved
         *       after simpasm.
         */

        vmovdqa         %ymm4, 0+32*\off(%rdi)
        vmovdqa         %ymm5, 128+32*\off(%rdi)
        vmovdqa         %ymm6, 256+32*\off(%rdi)
        vmovdqa         %ymm7, 384+32*\off(%rdi)
.endm

S2N_BN_SYMBOL(mldsa_intt):
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

        levels0t5 0
        levels0t5 1
        levels0t5 2
        levels0t5 3

        levels6t7 0
        levels6t7 1
        levels6t7 2
        levels6t7 3

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

S2N_BN_SIZE_DIRECTIVE(mldsa_intt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
