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
 * Name:        mlk_poly_decompress_d5_avx2
 *
 * Description: Decompression of a polynomial from 5 bits per coefficient.
 *
 * Arguments:   - int16_t *r:       pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                                  (of length MLKEM_POLYCOMPRESSEDBYTES_D5)
 *              - const uint8_t *data: pointer to constants
 *                                  (shufbidx[0:32], mask[32:64], shift[64:96])
 **************************************************/

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/poly_decompress_d5.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_decompress_d5_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_decompress_d5_avx2)
S2N_BN_SYMBOL(mlkem_poly_decompress_d5_avx2):

        .cfi_startproc
        movl $0xd010d01, %eax        # imm = 0xD010D01
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        vmovdqa (%rdx), %ymm1
        vmovdqa 0x20(%rdx), %ymm2
        vmovdqa 0x40(%rdx), %ymm3
        vmovq (%rsi), %xmm4
        vpinsrw $0x4, 0x8(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, (%rdi)
        vmovq 0xa(%rsi), %xmm4
        vpinsrw $0x4, 0x12(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x20(%rdi)
        vmovq 0x14(%rsi), %xmm4
        vpinsrw $0x4, 0x1c(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x40(%rdi)
        vmovq 0x1e(%rsi), %xmm4
        vpinsrw $0x4, 0x26(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x60(%rdi)
        vmovq 0x28(%rsi), %xmm4
        vpinsrw $0x4, 0x30(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x80(%rdi)
        vmovq 0x32(%rsi), %xmm4
        vpinsrw $0x4, 0x3a(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xa0(%rdi)
        vmovq 0x3c(%rsi), %xmm4
        vpinsrw $0x4, 0x44(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xc0(%rdi)
        vmovq 0x46(%rsi), %xmm4
        vpinsrw $0x4, 0x4e(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xe0(%rdi)
        vmovq 0x50(%rsi), %xmm4
        vpinsrw $0x4, 0x58(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x100(%rdi)
        vmovq 0x5a(%rsi), %xmm4
        vpinsrw $0x4, 0x62(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x120(%rdi)
        vmovq 0x64(%rsi), %xmm4
        vpinsrw $0x4, 0x6c(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x140(%rdi)
        vmovq 0x6e(%rsi), %xmm4
        vpinsrw $0x4, 0x76(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x160(%rdi)
        vmovq 0x78(%rsi), %xmm4
        vpinsrw $0x4, 0x80(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x180(%rdi)
        vmovq 0x82(%rsi), %xmm4
        vpinsrw $0x4, 0x8a(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1a0(%rdi)
        vmovq 0x8c(%rsi), %xmm4
        vpinsrw $0x4, 0x94(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1c0(%rdi)
        vmovq 0x96(%rsi), %xmm4
        vpinsrw $0x4, 0x9e(%rsi), %xmm4, %xmm4
        vinserti128 $0x1, %xmm4, %ymm4, %ymm4
        vpshufb %ymm1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmullw %ymm3, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_decompress_d5_avx2)

