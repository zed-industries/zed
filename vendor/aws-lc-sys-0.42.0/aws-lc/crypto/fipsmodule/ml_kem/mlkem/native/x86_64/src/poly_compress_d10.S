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
 * Name:        mlk_poly_compress_d10_avx2
 *
 * Description: Compression of a polynomial to 10 bits per coefficient.
 *
 * Arguments:   - uint8_t *r:       pointer to output byte array
 *                                  (of length MLKEM_POLYCOMPRESSEDBYTES_D10)
 *              - const int16_t *a: pointer to input polynomial
 *              - const uint8_t *data: pointer to shufbidx constant
 **************************************************/

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/poly_compress_d10.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_compress_d10_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_compress_d10_avx2)
S2N_BN_SYMBOL(mlkem_poly_compress_d10_avx2):

        .cfi_startproc
        movl $0x4ebf4ebf, %eax       # imm = 0x4EBF4EBF
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        vpsllw $0x3, %ymm0, %ymm1
        movl $0xf000f, %eax          # imm = 0xF000F
        vmovd %eax, %xmm2
        vpbroadcastd %xmm2, %ymm2
        movl $0x10001000, %eax       # imm = 0x10001000
        vmovd %eax, %xmm3
        vpbroadcastd %xmm3, %ymm3
        movl $0x3ff03ff, %eax        # imm = 0x3FF03FF
        vmovd %eax, %xmm4
        vpbroadcastd %xmm4, %ymm4
        movabsq $0x400000104000001, %rax # imm = 0x400000104000001
        vmovq %rax, %xmm5
        vpbroadcastq %xmm5, %ymm5
        movl $0xc, %eax
        vmovq %rax, %xmm6
        vpbroadcastq %xmm6, %ymm6
        vmovdqa (%rdx), %ymm7
        vmovdqa (%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, (%rdi)
        vmovd %xmm9, 0x10(%rdi)
        vmovdqa 0x20(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x14(%rdi)
        vmovd %xmm9, 0x24(%rdi)
        vmovdqa 0x40(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x28(%rdi)
        vmovd %xmm9, 0x38(%rdi)
        vmovdqa 0x60(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x3c(%rdi)
        vmovd %xmm9, 0x4c(%rdi)
        vmovdqa 0x80(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x50(%rdi)
        vmovd %xmm9, 0x60(%rdi)
        vmovdqa 0xa0(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x64(%rdi)
        vmovd %xmm9, 0x74(%rdi)
        vmovdqa 0xc0(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x78(%rdi)
        vmovd %xmm9, 0x88(%rdi)
        vmovdqa 0xe0(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x8c(%rdi)
        vmovd %xmm9, 0x9c(%rdi)
        vmovdqa 0x100(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0xa0(%rdi)
        vmovd %xmm9, 0xb0(%rdi)
        vmovdqa 0x120(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0xb4(%rdi)
        vmovd %xmm9, 0xc4(%rdi)
        vmovdqa 0x140(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0xc8(%rdi)
        vmovd %xmm9, 0xd8(%rdi)
        vmovdqa 0x160(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0xdc(%rdi)
        vmovd %xmm9, 0xec(%rdi)
        vmovdqa 0x180(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0xf0(%rdi)
        vmovd %xmm9, 0x100(%rdi)
        vmovdqa 0x1a0(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x104(%rdi)
        vmovd %xmm9, 0x114(%rdi)
        vmovdqa 0x1c0(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x118(%rdi)
        vmovd %xmm9, 0x128(%rdi)
        vmovdqa 0x1e0(%rsi), %ymm8
        vpmullw %ymm1, %ymm8, %ymm9
        vpaddw %ymm2, %ymm8, %ymm10
        vpsllw $0x3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm8, %ymm8
        vpsubw %ymm10, %ymm9, %ymm10
        vpandn %ymm10, %ymm9, %ymm9
        vpsrlw $0xf, %ymm9, %ymm9
        vpsubw %ymm9, %ymm8, %ymm8
        vpmulhrsw %ymm3, %ymm8, %ymm8
        vpand %ymm4, %ymm8, %ymm8
        vpmaddwd %ymm5, %ymm8, %ymm8
        vpsllvd %ymm6, %ymm8, %ymm8
        vpsrlq $0xc, %ymm8, %ymm8
        vpshufb %ymm7, %ymm8, %ymm8
        vextracti128 $0x1, %ymm8, %xmm9
        vpblendw $0xe0, %xmm9, %xmm8, %xmm8 # xmm8 = xmm8[0,1,2,3,4],xmm9[5,6,7]
        vmovdqu %xmm8, 0x12c(%rdi)
        vmovd %xmm9, 0x13c(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_compress_d10_avx2)

