/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [REF_AVX2]
 *   CRYSTALS-Kyber optimized AVX2 implementation
 *   Bos, Ducas, Kiltz, Lepoint, Lyubashevsky, Schanck, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/kyber/tree/main/avx2
 */

/*
 * This file is derived from the public domain
 * AVX2 Kyber implementation @[REF_AVX2].
 */

/*************************************************
 * Name:        mlk_poly_decompress_d4_avx2
 *
 * Description: Decompression of a polynomial from 4 bits per coefficient.
 *
 * Arguments:   - int16_t *r:       pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                                  (of length MLKEM_POLYCOMPRESSEDBYTES_D4)
 *              - const int8_t *data: pointer to shufbidx constant
 **************************************************/

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/poly_decompress_d4.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_decompress_d4_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_decompress_d4_avx2)
S2N_BN_SYMBOL(mlkem_poly_decompress_d4_avx2):

        .cfi_startproc
        movl $0xd010d01, %eax        # imm = 0xD010D01
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        movl $0xf0000f, %eax         # imm = 0xF0000F
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        movl $0x800800, %eax         # imm = 0x800800
        vmovd %eax, %xmm2
        vpbroadcastd %xmm2, %ymm2
        vmovdqa (%rdx), %ymm3
        vmovq (%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, (%rdi)
        vmovq 0x8(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x20(%rdi)
        vmovq 0x10(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x40(%rdi)
        vmovq 0x18(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x60(%rdi)
        vmovq 0x20(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x80(%rdi)
        vmovq 0x28(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xa0(%rdi)
        vmovq 0x30(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xc0(%rdi)
        vmovq 0x38(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xe0(%rdi)
        vmovq 0x40(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x100(%rdi)
        vmovq 0x48(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x120(%rdi)
        vmovq 0x50(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x140(%rdi)
        vmovq 0x58(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x160(%rdi)
        vmovq 0x60(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x180(%rdi)
        vmovq 0x68(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1a0(%rdi)
        vmovq 0x70(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1c0(%rdi)
        vmovq 0x78(%rsi), %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm3, %ymm4, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpmullw %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_decompress_d4_avx2)

