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
 * Name:        mlk_poly_compress_d5_avx2
 *
 * Description: Compression of a polynomial to 5 bits per coefficient.
 *
 * Arguments:   - uint8_t *r:       pointer to output byte array
 *                                  (of length MLKEM_POLYCOMPRESSEDBYTES_D5)
 *              - const int16_t *a: pointer to input polynomial
 *              - const uint8_t *data: pointer to shufbidx constant
 **************************************************/

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/poly_compress_d5.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_compress_d5_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_compress_d5_avx2)
S2N_BN_SYMBOL(mlkem_poly_compress_d5_avx2):

        .cfi_startproc
        movl $0x4ebf4ebf, %eax       # imm = 0x4EBF4EBF
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        movl $0x4000400, %eax        # imm = 0x4000400
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        movl $0x1f001f, %eax         # imm = 0x1F001F
        vmovd %eax, %xmm2
        vpbroadcastd %xmm2, %ymm2
        movl $0x20012001, %eax       # imm = 0x20012001
        vmovd %eax, %xmm3
        vpbroadcastd %xmm3, %ymm3
        movl $0x4000001, %eax        # imm = 0x4000001
        vmovd %eax, %xmm4
        vpbroadcastd %xmm4, %ymm4
        movl $0xc, %eax
        vmovq %rax, %xmm5
        vpbroadcastq %xmm5, %ymm5
        vmovdqa (%rdx), %ymm6
        vmovdqa (%rsi), %ymm7
        vmovdqa 0x20(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, (%rdi)
        vmovd %xmm8, 0x10(%rdi)
        vmovdqa 0x40(%rsi), %ymm7
        vmovdqa 0x60(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, 0x14(%rdi)
        vmovd %xmm8, 0x24(%rdi)
        vmovdqa 0x80(%rsi), %ymm7
        vmovdqa 0xa0(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, 0x28(%rdi)
        vmovd %xmm8, 0x38(%rdi)
        vmovdqa 0xc0(%rsi), %ymm7
        vmovdqa 0xe0(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, 0x3c(%rdi)
        vmovd %xmm8, 0x4c(%rdi)
        vmovdqa 0x100(%rsi), %ymm7
        vmovdqa 0x120(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, 0x50(%rdi)
        vmovd %xmm8, 0x60(%rdi)
        vmovdqa 0x140(%rsi), %ymm7
        vmovdqa 0x160(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, 0x64(%rdi)
        vmovd %xmm8, 0x74(%rdi)
        vmovdqa 0x180(%rsi), %ymm7
        vmovdqa 0x1a0(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, 0x78(%rdi)
        vmovd %xmm8, 0x88(%rdi)
        vmovdqa 0x1c0(%rsi), %ymm7
        vmovdqa 0x1e0(%rsi), %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhrsw %ymm1, %ymm7, %ymm7
        vpmulhrsw %ymm1, %ymm8, %ymm8
        vpand %ymm2, %ymm7, %ymm7
        vpand %ymm2, %ymm8, %ymm8
        vpackuswb %ymm8, %ymm7, %ymm7
        vpmaddubsw %ymm3, %ymm7, %ymm7
        vpmaddwd %ymm4, %ymm7, %ymm7
        vpsllvd %ymm5, %ymm7, %ymm7
        vpsrlvq %ymm5, %ymm7, %ymm7
        vpshufb %ymm6, %ymm7, %ymm7
        vextracti128 $0x1, %ymm7, %xmm8
        vpblendvb %xmm6, %xmm8, %xmm7, %xmm7
        vmovdqu %xmm7, 0x8c(%rdi)
        vmovd %xmm8, 0x9c(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_compress_d5_avx2)

