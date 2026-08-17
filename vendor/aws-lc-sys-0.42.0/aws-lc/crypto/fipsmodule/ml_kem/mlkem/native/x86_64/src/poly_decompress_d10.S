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
 * Name:        mlk_poly_decompress_d10_avx2
 *
 * Description: Decompression of a polynomial from 10 bits per coefficient.
 *
 * Arguments:   - int16_t *r:       pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                                  (of length MLKEM_POLYCOMPRESSEDBYTES_D10)
 *              - const uint8_t *data: pointer to shufbidx constant
 **************************************************/

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/poly_decompress_d10.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_decompress_d10_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_decompress_d10_avx2)
S2N_BN_SYMBOL(mlkem_poly_decompress_d10_avx2):

        .cfi_startproc
        movl $0xd013404, %eax        # imm = 0xD013404
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        movl $0x4, %eax
        vmovq %rax, %xmm1
        vpbroadcastq %xmm1, %ymm1
        movl $0x7fe01ff8, %eax       # imm = 0x7FE01FF8
        vmovd %eax, %xmm2
        vpbroadcastd %xmm2, %ymm2
        vmovdqa (%rdx), %ymm3
        vmovdqu (%rsi), %xmm4
        vmovd 0x10(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, (%rdi)
        vmovdqu 0x14(%rsi), %xmm4
        vmovd 0x24(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x20(%rdi)
        vmovdqu 0x28(%rsi), %xmm4
        vmovd 0x38(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x40(%rdi)
        vmovdqu 0x3c(%rsi), %xmm4
        vmovd 0x4c(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x60(%rdi)
        vmovdqu 0x50(%rsi), %xmm4
        vmovd 0x60(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x80(%rdi)
        vmovdqu 0x64(%rsi), %xmm4
        vmovd 0x74(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xa0(%rdi)
        vmovdqu 0x78(%rsi), %xmm4
        vmovd 0x88(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xc0(%rdi)
        vmovdqu 0x8c(%rsi), %xmm4
        vmovd 0x9c(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0xe0(%rdi)
        vmovdqu 0xa0(%rsi), %xmm4
        vmovd 0xb0(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x100(%rdi)
        vmovdqu 0xb4(%rsi), %xmm4
        vmovd 0xc4(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x120(%rdi)
        vmovdqu 0xc8(%rsi), %xmm4
        vmovd 0xd8(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x140(%rdi)
        vmovdqu 0xdc(%rsi), %xmm4
        vmovd 0xec(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x160(%rdi)
        vmovdqu 0xf0(%rsi), %xmm4
        vmovd 0x100(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x180(%rdi)
        vmovdqu 0x104(%rsi), %xmm4
        vmovd 0x114(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1a0(%rdi)
        vmovdqu 0x118(%rsi), %xmm4
        vmovd 0x128(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1c0(%rdi)
        vmovdqu 0x12c(%rsi), %xmm4
        vmovd 0x13c(%rsi), %xmm5
        vinserti128 $0x1, %xmm5, %ymm4, %ymm4
        vpermq $0x94, %ymm4, %ymm4     # ymm4 = ymm4[0,1,1,2]
        vpshufb %ymm3, %ymm4, %ymm4
        vpsllvd %ymm1, %ymm4, %ymm4
        vpsrlw $0x1, %ymm4, %ymm4
        vpand %ymm2, %ymm4, %ymm4
        vpmulhrsw %ymm0, %ymm4, %ymm4
        vmovdqu %ymm4, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_decompress_d10_avx2)

