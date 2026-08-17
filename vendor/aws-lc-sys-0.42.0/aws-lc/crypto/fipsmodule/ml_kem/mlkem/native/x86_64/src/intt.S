/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [AVX2_NTT]
 *   Faster AVX2 optimized NTT multiplication for Ring-LWE lattice cryptography.
 *   Gregor Seiler
 *   https://eprint.iacr.org/2018/039
 *
 * - [REF_AVX2]
 *   CRYSTALS-Kyber optimized AVX2 implementation
 *   Bos, Ducas, Kiltz, Lepoint, Lyubashevsky, Schanck, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/kyber/tree/main/avx2
 */

/*
 * This file is derived from the public domain
 * AVX2 Kyber implementation @[REF_AVX2].
 *
 * The core ideas behind the implementation are described in @[AVX2_NTT].
 *
 * Changes:
 * - Different placement of modular reductions to simplify
 *   reasoning of non-overflow
 */

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/intt.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_invntt_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_invntt_avx2)
S2N_BN_SYMBOL(mlkem_invntt_avx2):

        .cfi_startproc
        movl $0xd010d01, %eax        # imm = 0xD010D01
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        movl $0xd8a1d8a1, %eax       # imm = 0xD8A1D8A1
        vmovd %eax, %xmm2
        vpbroadcastd %xmm2, %ymm2
        movl $0x5a105a1, %eax        # imm = 0x5A105A1
        vmovd %eax, %xmm3
        vpbroadcastd %xmm3, %ymm3
        vmovdqa (%rdi), %ymm4
        vmovdqa 0x40(%rdi), %ymm6
        vmovdqa 0x20(%rdi), %ymm5
        vmovdqa 0x60(%rdi), %ymm7
        vpmullw %ymm2, %ymm4, %ymm12
        vpmulhw %ymm3, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm4, %ymm4
        vpmullw %ymm2, %ymm6, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm6
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm6, %ymm6
        vpmullw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm5, %ymm5
        vpmullw %ymm2, %ymm7, %ymm12
        vpmulhw %ymm3, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm7, %ymm7
        vmovdqa 0x80(%rdi), %ymm8
        vmovdqa 0xc0(%rdi), %ymm10
        vmovdqa 0xa0(%rdi), %ymm9
        vmovdqa 0xe0(%rdi), %ymm11
        vpmullw %ymm2, %ymm8, %ymm12
        vpmulhw %ymm3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm8, %ymm8
        vpmullw %ymm2, %ymm10, %ymm12
        vpmulhw %ymm3, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm10, %ymm10
        vpmullw %ymm2, %ymm9, %ymm12
        vpmulhw %ymm3, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vpmullw %ymm2, %ymm11, %ymm12
        vpmulhw %ymm3, %ymm11, %ymm11
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm11, %ymm11
        vpermq $0x4e, 0x3a0(%rsi), %ymm15 # ymm15 = mem[2,3,0,1]
        vpermq $0x4e, 0x360(%rsi), %ymm1 # ymm1 = mem[2,3,0,1]
        vpermq $0x4e, 0x3c0(%rsi), %ymm2 # ymm2 = mem[2,3,0,1]
        vpermq $0x4e, 0x380(%rsi), %ymm3 # ymm3 = mem[2,3,0,1]
        vmovdqa (%rsi), %ymm12
        vpshufb %ymm12, %ymm15, %ymm15
        vpshufb %ymm12, %ymm1, %ymm1
        vpshufb %ymm12, %ymm2, %ymm2
        vpshufb %ymm12, %ymm3, %ymm3
        vpsubw %ymm4, %ymm6, %ymm12
        vpaddw %ymm6, %ymm4, %ymm4
        vpsubw %ymm5, %ymm7, %ymm13
        vpmullw %ymm15, %ymm12, %ymm6
        vpaddw %ymm7, %ymm5, %ymm5
        vpsubw %ymm8, %ymm10, %ymm14
        vpmullw %ymm15, %ymm13, %ymm7
        vpaddw %ymm10, %ymm8, %ymm8
        vpsubw %ymm9, %ymm11, %ymm15
        vpmullw %ymm1, %ymm14, %ymm10
        vpaddw %ymm11, %ymm9, %ymm9
        vpmullw %ymm1, %ymm15, %ymm11
        vpmulhw %ymm2, %ymm12, %ymm12
        vpmulhw %ymm2, %ymm13, %ymm13
        vpmulhw %ymm3, %ymm14, %ymm14
        vpmulhw %ymm3, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm6, %ymm6
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm6, %ymm12, %ymm6
        vpsubw %ymm7, %ymm13, %ymm7
        vpsubw %ymm10, %ymm14, %ymm10
        vpsubw %ymm11, %ymm15, %ymm11
        vpermq $0x4e, 0x320(%rsi), %ymm2 # ymm2 = mem[2,3,0,1]
        vpermq $0x4e, 0x340(%rsi), %ymm3 # ymm3 = mem[2,3,0,1]
        vmovdqa (%rsi), %ymm1
        vpshufb %ymm1, %ymm2, %ymm2
        vpshufb %ymm1, %ymm3, %ymm3
        vpsubw %ymm4, %ymm8, %ymm12
        vpaddw %ymm8, %ymm4, %ymm4
        vpsubw %ymm5, %ymm9, %ymm13
        vpmullw %ymm2, %ymm12, %ymm8
        vpaddw %ymm9, %ymm5, %ymm5
        vpsubw %ymm6, %ymm10, %ymm14
        vpmullw %ymm2, %ymm13, %ymm9
        vpaddw %ymm10, %ymm6, %ymm6
        vpsubw %ymm7, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm10
        vpaddw %ymm11, %ymm7, %ymm7
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm3, %ymm12, %ymm12
        vpmulhw %ymm3, %ymm13, %ymm13
        vpmulhw %ymm3, %ymm14, %ymm14
        vpmulhw %ymm3, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm8, %ymm12, %ymm8
        vpsubw %ymm9, %ymm13, %ymm9
        vpsubw %ymm10, %ymm14, %ymm10
        vpsubw %ymm11, %ymm15, %ymm11
        vpslld $0x10, %ymm5, %ymm3
        vpblendw $0xaa, %ymm3, %ymm4, %ymm3 # ymm3 = ymm4[0],ymm3[1],ymm4[2],ymm3[3],ymm4[4],ymm3[5],ymm4[6],ymm3[7],ymm4[8],ymm3[9],ymm4[10],ymm3[11],ymm4[12],ymm3[13],ymm4[14],ymm3[15]
        vpsrld $0x10, %ymm4, %ymm4
        vpblendw $0xaa, %ymm5, %ymm4, %ymm5 # ymm5 = ymm4[0],ymm5[1],ymm4[2],ymm5[3],ymm4[4],ymm5[5],ymm4[6],ymm5[7],ymm4[8],ymm5[9],ymm4[10],ymm5[11],ymm4[12],ymm5[13],ymm4[14],ymm5[15]
        vpslld $0x10, %ymm7, %ymm4
        vpblendw $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7],ymm6[8],ymm4[9],ymm6[10],ymm4[11],ymm6[12],ymm4[13],ymm6[14],ymm4[15]
        vpsrld $0x10, %ymm6, %ymm6
        vpblendw $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7],ymm6[8],ymm7[9],ymm6[10],ymm7[11],ymm6[12],ymm7[13],ymm6[14],ymm7[15]
        vpslld $0x10, %ymm9, %ymm6
        vpblendw $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7],ymm8[8],ymm6[9],ymm8[10],ymm6[11],ymm8[12],ymm6[13],ymm8[14],ymm6[15]
        vpsrld $0x10, %ymm8, %ymm8
        vpblendw $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7],ymm8[8],ymm9[9],ymm8[10],ymm9[11],ymm8[12],ymm9[13],ymm8[14],ymm9[15]
        vpslld $0x10, %ymm11, %ymm8
        vpblendw $0xaa, %ymm8, %ymm10, %ymm8 # ymm8 = ymm10[0],ymm8[1],ymm10[2],ymm8[3],ymm10[4],ymm8[5],ymm10[6],ymm8[7],ymm10[8],ymm8[9],ymm10[10],ymm8[11],ymm10[12],ymm8[13],ymm10[14],ymm8[15]
        vpsrld $0x10, %ymm10, %ymm10
        vpblendw $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7],ymm10[8],ymm11[9],ymm10[10],ymm11[11],ymm10[12],ymm11[13],ymm10[14],ymm11[15]
        vmovdqa 0x20(%rsi), %ymm12
        vpermd 0x2e0(%rsi), %ymm12, %ymm2
        vpermd 0x300(%rsi), %ymm12, %ymm10
        vpsubw %ymm3, %ymm5, %ymm12
        vpaddw %ymm5, %ymm3, %ymm3
        vpsubw %ymm4, %ymm7, %ymm13
        vpmullw %ymm2, %ymm12, %ymm5
        vpaddw %ymm7, %ymm4, %ymm4
        vpsubw %ymm6, %ymm9, %ymm14
        vpmullw %ymm2, %ymm13, %ymm7
        vpaddw %ymm9, %ymm6, %ymm6
        vpsubw %ymm8, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm9
        vpaddw %ymm11, %ymm8, %ymm8
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm10, %ymm12, %ymm12
        vpmulhw %ymm10, %ymm13, %ymm13
        vpmulhw %ymm10, %ymm14, %ymm14
        vpmulhw %ymm10, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm5, %ymm12, %ymm5
        vpsubw %ymm7, %ymm13, %ymm7
        vpsubw %ymm9, %ymm14, %ymm9
        vpsubw %ymm11, %ymm15, %ymm11
        movl $0x4ebf4ebf, %eax       # imm = 0x4EBF4EBF
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        vpmulhw %ymm1, %ymm3, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm3, %ymm3
        vmovsldup %ymm4, %ymm10   # ymm10 = ymm4[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm10, %ymm3, %ymm10 # ymm10 = ymm3[0],ymm10[1],ymm3[2],ymm10[3],ymm3[4],ymm10[5],ymm3[6],ymm10[7]
        vpsrlq $0x20, %ymm3, %ymm3
        vpblendd $0xaa, %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[0],ymm4[1],ymm3[2],ymm4[3],ymm3[4],ymm4[5],ymm3[6],ymm4[7]
        vmovsldup %ymm8, %ymm3    # ymm3 = ymm8[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm6, %ymm3 # ymm3 = ymm6[0],ymm3[1],ymm6[2],ymm3[3],ymm6[4],ymm3[5],ymm6[6],ymm3[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm8, %ymm6, %ymm8 # ymm8 = ymm6[0],ymm8[1],ymm6[2],ymm8[3],ymm6[4],ymm8[5],ymm6[6],ymm8[7]
        vmovsldup %ymm7, %ymm6    # ymm6 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm6[1],ymm5[2],ymm6[3],ymm5[4],ymm6[5],ymm5[6],ymm6[7]
        vpsrlq $0x20, %ymm5, %ymm5
        vpblendd $0xaa, %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[0],ymm7[1],ymm5[2],ymm7[3],ymm5[4],ymm7[5],ymm5[6],ymm7[7]
        vmovsldup %ymm11, %ymm5   # ymm5 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm5, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm5[1],ymm9[2],ymm5[3],ymm9[4],ymm5[5],ymm9[6],ymm5[7]
        vpsrlq $0x20, %ymm9, %ymm9
        vpblendd $0xaa, %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[0],ymm11[1],ymm9[2],ymm11[3],ymm9[4],ymm11[5],ymm9[6],ymm11[7]
        vpermq $0x1b, 0x2a0(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpermq $0x1b, 0x2c0(%rsi), %ymm9 # ymm9 = mem[3,2,1,0]
        vpsubw %ymm10, %ymm4, %ymm12
        vpaddw %ymm4, %ymm10, %ymm10
        vpsubw %ymm3, %ymm8, %ymm13
        vpmullw %ymm2, %ymm12, %ymm4
        vpaddw %ymm8, %ymm3, %ymm3
        vpsubw %ymm6, %ymm7, %ymm14
        vpmullw %ymm2, %ymm13, %ymm8
        vpaddw %ymm7, %ymm6, %ymm6
        vpsubw %ymm5, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm7
        vpaddw %ymm11, %ymm5, %ymm5
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm9, %ymm12, %ymm12
        vpmulhw %ymm9, %ymm13, %ymm13
        vpmulhw %ymm9, %ymm14, %ymm14
        vpmulhw %ymm9, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm4, %ymm12, %ymm4
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm7, %ymm14, %ymm7
        vpsubw %ymm11, %ymm15, %ymm11
        vpmulhw %ymm1, %ymm10, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm10, %ymm10
        vpunpcklqdq %ymm3, %ymm10, %ymm9 # ymm9 = ymm10[0],ymm3[0],ymm10[2],ymm3[2]
        vpunpckhqdq %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[1],ymm3[1],ymm10[3],ymm3[3]
        vpunpcklqdq %ymm5, %ymm6, %ymm10 # ymm10 = ymm6[0],ymm5[0],ymm6[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[1],ymm5[1],ymm6[3],ymm5[3]
        vpunpcklqdq %ymm8, %ymm4, %ymm6 # ymm6 = ymm4[0],ymm8[0],ymm4[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[1],ymm8[1],ymm4[3],ymm8[3]
        vpunpcklqdq %ymm11, %ymm7, %ymm4 # ymm4 = ymm7[0],ymm11[0],ymm7[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[1],ymm11[1],ymm7[3],ymm11[3]
        vpermq $0x4e, 0x260(%rsi), %ymm2 # ymm2 = mem[2,3,0,1]
        vpermq $0x4e, 0x280(%rsi), %ymm7 # ymm7 = mem[2,3,0,1]
        vpsubw %ymm9, %ymm3, %ymm12
        vpaddw %ymm3, %ymm9, %ymm9
        vpsubw %ymm10, %ymm5, %ymm13
        vpmullw %ymm2, %ymm12, %ymm3
        vpaddw %ymm5, %ymm10, %ymm10
        vpsubw %ymm6, %ymm8, %ymm14
        vpmullw %ymm2, %ymm13, %ymm5
        vpaddw %ymm8, %ymm6, %ymm6
        vpsubw %ymm4, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm8
        vpaddw %ymm11, %ymm4, %ymm4
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm7, %ymm12, %ymm12
        vpmulhw %ymm7, %ymm13, %ymm13
        vpmulhw %ymm7, %ymm14, %ymm14
        vpmulhw %ymm7, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm3, %ymm3
        vpmulhw %ymm0, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm3, %ymm12, %ymm3
        vpsubw %ymm5, %ymm13, %ymm5
        vpsubw %ymm8, %ymm14, %ymm8
        vpsubw %ymm11, %ymm15, %ymm11
        vpmulhw %ymm1, %ymm9, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vperm2i128 $0x20, %ymm10, %ymm9, %ymm7 # ymm7 = ymm9[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm9, %ymm10 # ymm10 = ymm9[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm4, %ymm6, %ymm9 # ymm9 = ymm6[0,1],ymm4[0,1]
        vperm2i128 $0x31, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[2,3],ymm4[2,3]
        vperm2i128 $0x20, %ymm5, %ymm3, %ymm6 # ymm6 = ymm3[0,1],ymm5[0,1]
        vperm2i128 $0x31, %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[2,3],ymm5[2,3]
        vperm2i128 $0x20, %ymm11, %ymm8, %ymm3 # ymm3 = ymm8[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm8, %ymm11 # ymm11 = ymm8[2,3],ymm11[2,3]
        vmovdqa 0x220(%rsi), %ymm2
        vmovdqa 0x240(%rsi), %ymm8
        vpsubw %ymm7, %ymm10, %ymm12
        vpaddw %ymm10, %ymm7, %ymm7
        vpsubw %ymm9, %ymm4, %ymm13
        vpmullw %ymm2, %ymm12, %ymm10
        vpaddw %ymm4, %ymm9, %ymm9
        vpsubw %ymm6, %ymm5, %ymm14
        vpmullw %ymm2, %ymm13, %ymm4
        vpaddw %ymm5, %ymm6, %ymm6
        vpsubw %ymm3, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm5
        vpaddw %ymm11, %ymm3, %ymm3
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm8, %ymm12, %ymm12
        vpmulhw %ymm8, %ymm13, %ymm13
        vpmulhw %ymm8, %ymm14, %ymm14
        vpmulhw %ymm8, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm10, %ymm12, %ymm10
        vpsubw %ymm4, %ymm13, %ymm4
        vpsubw %ymm5, %ymm14, %ymm5
        vpsubw %ymm11, %ymm15, %ymm11
        vpmulhw %ymm1, %ymm7, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm7, %ymm7
        vmovdqa %ymm7, (%rdi)
        vmovdqa %ymm9, 0x20(%rdi)
        vmovdqa %ymm6, 0x40(%rdi)
        vmovdqa %ymm3, 0x60(%rdi)
        vmovdqa %ymm10, 0x80(%rdi)
        vmovdqa %ymm4, 0xa0(%rdi)
        vmovdqa %ymm5, 0xc0(%rdi)
        vmovdqa %ymm11, 0xe0(%rdi)
        movl $0xd8a1d8a1, %eax       # imm = 0xD8A1D8A1
        vmovd %eax, %xmm2
        vpbroadcastd %xmm2, %ymm2
        movl $0x5a105a1, %eax        # imm = 0x5A105A1
        vmovd %eax, %xmm3
        vpbroadcastd %xmm3, %ymm3
        vmovdqa 0x100(%rdi), %ymm4
        vmovdqa 0x140(%rdi), %ymm6
        vmovdqa 0x120(%rdi), %ymm5
        vmovdqa 0x160(%rdi), %ymm7
        vpmullw %ymm2, %ymm4, %ymm12
        vpmulhw %ymm3, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm4, %ymm4
        vpmullw %ymm2, %ymm6, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm6
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm6, %ymm6
        vpmullw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm5, %ymm5
        vpmullw %ymm2, %ymm7, %ymm12
        vpmulhw %ymm3, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm7, %ymm7
        vmovdqa 0x180(%rdi), %ymm8
        vmovdqa 0x1c0(%rdi), %ymm10
        vmovdqa 0x1a0(%rdi), %ymm9
        vmovdqa 0x1e0(%rdi), %ymm11
        vpmullw %ymm2, %ymm8, %ymm12
        vpmulhw %ymm3, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm8, %ymm8
        vpmullw %ymm2, %ymm10, %ymm12
        vpmulhw %ymm3, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm10, %ymm10
        vpmullw %ymm2, %ymm9, %ymm12
        vpmulhw %ymm3, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vpmullw %ymm2, %ymm11, %ymm12
        vpmulhw %ymm3, %ymm11, %ymm11
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm11, %ymm11
        vpermq $0x4e, 0x1e0(%rsi), %ymm15 # ymm15 = mem[2,3,0,1]
        vpermq $0x4e, 0x1a0(%rsi), %ymm1 # ymm1 = mem[2,3,0,1]
        vpermq $0x4e, 0x200(%rsi), %ymm2 # ymm2 = mem[2,3,0,1]
        vpermq $0x4e, 0x1c0(%rsi), %ymm3 # ymm3 = mem[2,3,0,1]
        vmovdqa (%rsi), %ymm12
        vpshufb %ymm12, %ymm15, %ymm15
        vpshufb %ymm12, %ymm1, %ymm1
        vpshufb %ymm12, %ymm2, %ymm2
        vpshufb %ymm12, %ymm3, %ymm3
        vpsubw %ymm4, %ymm6, %ymm12
        vpaddw %ymm6, %ymm4, %ymm4
        vpsubw %ymm5, %ymm7, %ymm13
        vpmullw %ymm15, %ymm12, %ymm6
        vpaddw %ymm7, %ymm5, %ymm5
        vpsubw %ymm8, %ymm10, %ymm14
        vpmullw %ymm15, %ymm13, %ymm7
        vpaddw %ymm10, %ymm8, %ymm8
        vpsubw %ymm9, %ymm11, %ymm15
        vpmullw %ymm1, %ymm14, %ymm10
        vpaddw %ymm11, %ymm9, %ymm9
        vpmullw %ymm1, %ymm15, %ymm11
        vpmulhw %ymm2, %ymm12, %ymm12
        vpmulhw %ymm2, %ymm13, %ymm13
        vpmulhw %ymm3, %ymm14, %ymm14
        vpmulhw %ymm3, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm6, %ymm6
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm6, %ymm12, %ymm6
        vpsubw %ymm7, %ymm13, %ymm7
        vpsubw %ymm10, %ymm14, %ymm10
        vpsubw %ymm11, %ymm15, %ymm11
        vpermq $0x4e, 0x160(%rsi), %ymm2 # ymm2 = mem[2,3,0,1]
        vpermq $0x4e, 0x180(%rsi), %ymm3 # ymm3 = mem[2,3,0,1]
        vmovdqa (%rsi), %ymm1
        vpshufb %ymm1, %ymm2, %ymm2
        vpshufb %ymm1, %ymm3, %ymm3
        vpsubw %ymm4, %ymm8, %ymm12
        vpaddw %ymm8, %ymm4, %ymm4
        vpsubw %ymm5, %ymm9, %ymm13
        vpmullw %ymm2, %ymm12, %ymm8
        vpaddw %ymm9, %ymm5, %ymm5
        vpsubw %ymm6, %ymm10, %ymm14
        vpmullw %ymm2, %ymm13, %ymm9
        vpaddw %ymm10, %ymm6, %ymm6
        vpsubw %ymm7, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm10
        vpaddw %ymm11, %ymm7, %ymm7
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm3, %ymm12, %ymm12
        vpmulhw %ymm3, %ymm13, %ymm13
        vpmulhw %ymm3, %ymm14, %ymm14
        vpmulhw %ymm3, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm8, %ymm12, %ymm8
        vpsubw %ymm9, %ymm13, %ymm9
        vpsubw %ymm10, %ymm14, %ymm10
        vpsubw %ymm11, %ymm15, %ymm11
        vpslld $0x10, %ymm5, %ymm3
        vpblendw $0xaa, %ymm3, %ymm4, %ymm3 # ymm3 = ymm4[0],ymm3[1],ymm4[2],ymm3[3],ymm4[4],ymm3[5],ymm4[6],ymm3[7],ymm4[8],ymm3[9],ymm4[10],ymm3[11],ymm4[12],ymm3[13],ymm4[14],ymm3[15]
        vpsrld $0x10, %ymm4, %ymm4
        vpblendw $0xaa, %ymm5, %ymm4, %ymm5 # ymm5 = ymm4[0],ymm5[1],ymm4[2],ymm5[3],ymm4[4],ymm5[5],ymm4[6],ymm5[7],ymm4[8],ymm5[9],ymm4[10],ymm5[11],ymm4[12],ymm5[13],ymm4[14],ymm5[15]
        vpslld $0x10, %ymm7, %ymm4
        vpblendw $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7],ymm6[8],ymm4[9],ymm6[10],ymm4[11],ymm6[12],ymm4[13],ymm6[14],ymm4[15]
        vpsrld $0x10, %ymm6, %ymm6
        vpblendw $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7],ymm6[8],ymm7[9],ymm6[10],ymm7[11],ymm6[12],ymm7[13],ymm6[14],ymm7[15]
        vpslld $0x10, %ymm9, %ymm6
        vpblendw $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7],ymm8[8],ymm6[9],ymm8[10],ymm6[11],ymm8[12],ymm6[13],ymm8[14],ymm6[15]
        vpsrld $0x10, %ymm8, %ymm8
        vpblendw $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7],ymm8[8],ymm9[9],ymm8[10],ymm9[11],ymm8[12],ymm9[13],ymm8[14],ymm9[15]
        vpslld $0x10, %ymm11, %ymm8
        vpblendw $0xaa, %ymm8, %ymm10, %ymm8 # ymm8 = ymm10[0],ymm8[1],ymm10[2],ymm8[3],ymm10[4],ymm8[5],ymm10[6],ymm8[7],ymm10[8],ymm8[9],ymm10[10],ymm8[11],ymm10[12],ymm8[13],ymm10[14],ymm8[15]
        vpsrld $0x10, %ymm10, %ymm10
        vpblendw $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7],ymm10[8],ymm11[9],ymm10[10],ymm11[11],ymm10[12],ymm11[13],ymm10[14],ymm11[15]
        vmovdqa 0x20(%rsi), %ymm12
        vpermd 0x120(%rsi), %ymm12, %ymm2
        vpermd 0x140(%rsi), %ymm12, %ymm10
        vpsubw %ymm3, %ymm5, %ymm12
        vpaddw %ymm5, %ymm3, %ymm3
        vpsubw %ymm4, %ymm7, %ymm13
        vpmullw %ymm2, %ymm12, %ymm5
        vpaddw %ymm7, %ymm4, %ymm4
        vpsubw %ymm6, %ymm9, %ymm14
        vpmullw %ymm2, %ymm13, %ymm7
        vpaddw %ymm9, %ymm6, %ymm6
        vpsubw %ymm8, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm9
        vpaddw %ymm11, %ymm8, %ymm8
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm10, %ymm12, %ymm12
        vpmulhw %ymm10, %ymm13, %ymm13
        vpmulhw %ymm10, %ymm14, %ymm14
        vpmulhw %ymm10, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm5, %ymm12, %ymm5
        vpsubw %ymm7, %ymm13, %ymm7
        vpsubw %ymm9, %ymm14, %ymm9
        vpsubw %ymm11, %ymm15, %ymm11
        movl $0x4ebf4ebf, %eax       # imm = 0x4EBF4EBF
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        vpmulhw %ymm1, %ymm3, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm3, %ymm3
        vmovsldup %ymm4, %ymm10   # ymm10 = ymm4[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm10, %ymm3, %ymm10 # ymm10 = ymm3[0],ymm10[1],ymm3[2],ymm10[3],ymm3[4],ymm10[5],ymm3[6],ymm10[7]
        vpsrlq $0x20, %ymm3, %ymm3
        vpblendd $0xaa, %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[0],ymm4[1],ymm3[2],ymm4[3],ymm3[4],ymm4[5],ymm3[6],ymm4[7]
        vmovsldup %ymm8, %ymm3    # ymm3 = ymm8[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm6, %ymm3 # ymm3 = ymm6[0],ymm3[1],ymm6[2],ymm3[3],ymm6[4],ymm3[5],ymm6[6],ymm3[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm8, %ymm6, %ymm8 # ymm8 = ymm6[0],ymm8[1],ymm6[2],ymm8[3],ymm6[4],ymm8[5],ymm6[6],ymm8[7]
        vmovsldup %ymm7, %ymm6    # ymm6 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm6[1],ymm5[2],ymm6[3],ymm5[4],ymm6[5],ymm5[6],ymm6[7]
        vpsrlq $0x20, %ymm5, %ymm5
        vpblendd $0xaa, %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[0],ymm7[1],ymm5[2],ymm7[3],ymm5[4],ymm7[5],ymm5[6],ymm7[7]
        vmovsldup %ymm11, %ymm5   # ymm5 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm5, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm5[1],ymm9[2],ymm5[3],ymm9[4],ymm5[5],ymm9[6],ymm5[7]
        vpsrlq $0x20, %ymm9, %ymm9
        vpblendd $0xaa, %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[0],ymm11[1],ymm9[2],ymm11[3],ymm9[4],ymm11[5],ymm9[6],ymm11[7]
        vpermq $0x1b, 0xe0(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpermq $0x1b, 0x100(%rsi), %ymm9 # ymm9 = mem[3,2,1,0]
        vpsubw %ymm10, %ymm4, %ymm12
        vpaddw %ymm4, %ymm10, %ymm10
        vpsubw %ymm3, %ymm8, %ymm13
        vpmullw %ymm2, %ymm12, %ymm4
        vpaddw %ymm8, %ymm3, %ymm3
        vpsubw %ymm6, %ymm7, %ymm14
        vpmullw %ymm2, %ymm13, %ymm8
        vpaddw %ymm7, %ymm6, %ymm6
        vpsubw %ymm5, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm7
        vpaddw %ymm11, %ymm5, %ymm5
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm9, %ymm12, %ymm12
        vpmulhw %ymm9, %ymm13, %ymm13
        vpmulhw %ymm9, %ymm14, %ymm14
        vpmulhw %ymm9, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm4, %ymm12, %ymm4
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm7, %ymm14, %ymm7
        vpsubw %ymm11, %ymm15, %ymm11
        vpmulhw %ymm1, %ymm10, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm10, %ymm10
        vpunpcklqdq %ymm3, %ymm10, %ymm9 # ymm9 = ymm10[0],ymm3[0],ymm10[2],ymm3[2]
        vpunpckhqdq %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[1],ymm3[1],ymm10[3],ymm3[3]
        vpunpcklqdq %ymm5, %ymm6, %ymm10 # ymm10 = ymm6[0],ymm5[0],ymm6[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[1],ymm5[1],ymm6[3],ymm5[3]
        vpunpcklqdq %ymm8, %ymm4, %ymm6 # ymm6 = ymm4[0],ymm8[0],ymm4[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[1],ymm8[1],ymm4[3],ymm8[3]
        vpunpcklqdq %ymm11, %ymm7, %ymm4 # ymm4 = ymm7[0],ymm11[0],ymm7[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[1],ymm11[1],ymm7[3],ymm11[3]
        vpermq $0x4e, 0xa0(%rsi), %ymm2 # ymm2 = mem[2,3,0,1]
        vpermq $0x4e, 0xc0(%rsi), %ymm7 # ymm7 = mem[2,3,0,1]
        vpsubw %ymm9, %ymm3, %ymm12
        vpaddw %ymm3, %ymm9, %ymm9
        vpsubw %ymm10, %ymm5, %ymm13
        vpmullw %ymm2, %ymm12, %ymm3
        vpaddw %ymm5, %ymm10, %ymm10
        vpsubw %ymm6, %ymm8, %ymm14
        vpmullw %ymm2, %ymm13, %ymm5
        vpaddw %ymm8, %ymm6, %ymm6
        vpsubw %ymm4, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm8
        vpaddw %ymm11, %ymm4, %ymm4
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm7, %ymm12, %ymm12
        vpmulhw %ymm7, %ymm13, %ymm13
        vpmulhw %ymm7, %ymm14, %ymm14
        vpmulhw %ymm7, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm3, %ymm3
        vpmulhw %ymm0, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm3, %ymm12, %ymm3
        vpsubw %ymm5, %ymm13, %ymm5
        vpsubw %ymm8, %ymm14, %ymm8
        vpsubw %ymm11, %ymm15, %ymm11
        vpmulhw %ymm1, %ymm9, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vperm2i128 $0x20, %ymm10, %ymm9, %ymm7 # ymm7 = ymm9[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm9, %ymm10 # ymm10 = ymm9[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm4, %ymm6, %ymm9 # ymm9 = ymm6[0,1],ymm4[0,1]
        vperm2i128 $0x31, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[2,3],ymm4[2,3]
        vperm2i128 $0x20, %ymm5, %ymm3, %ymm6 # ymm6 = ymm3[0,1],ymm5[0,1]
        vperm2i128 $0x31, %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[2,3],ymm5[2,3]
        vperm2i128 $0x20, %ymm11, %ymm8, %ymm3 # ymm3 = ymm8[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm8, %ymm11 # ymm11 = ymm8[2,3],ymm11[2,3]
        vmovdqa 0x60(%rsi), %ymm2
        vmovdqa 0x80(%rsi), %ymm8
        vpsubw %ymm7, %ymm10, %ymm12
        vpaddw %ymm10, %ymm7, %ymm7
        vpsubw %ymm9, %ymm4, %ymm13
        vpmullw %ymm2, %ymm12, %ymm10
        vpaddw %ymm4, %ymm9, %ymm9
        vpsubw %ymm6, %ymm5, %ymm14
        vpmullw %ymm2, %ymm13, %ymm4
        vpaddw %ymm5, %ymm6, %ymm6
        vpsubw %ymm3, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm5
        vpaddw %ymm11, %ymm3, %ymm3
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm8, %ymm12, %ymm12
        vpmulhw %ymm8, %ymm13, %ymm13
        vpmulhw %ymm8, %ymm14, %ymm14
        vpmulhw %ymm8, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm10, %ymm12, %ymm10
        vpsubw %ymm4, %ymm13, %ymm4
        vpsubw %ymm5, %ymm14, %ymm5
        vpsubw %ymm11, %ymm15, %ymm11
        vpmulhw %ymm1, %ymm7, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm7, %ymm7
        vmovdqa %ymm7, 0x100(%rdi)
        vmovdqa %ymm9, 0x120(%rdi)
        vmovdqa %ymm6, 0x140(%rdi)
        vmovdqa %ymm3, 0x160(%rdi)
        vmovdqa %ymm10, 0x180(%rdi)
        vmovdqa %ymm4, 0x1a0(%rdi)
        vmovdqa %ymm5, 0x1c0(%rdi)
        vmovdqa %ymm11, 0x1e0(%rdi)
        vmovdqa (%rdi), %ymm4
        vmovdqa 0x100(%rdi), %ymm8
        vmovdqa 0x20(%rdi), %ymm5
        vmovdqa 0x120(%rdi), %ymm9
        vpbroadcastq 0x40(%rsi), %ymm2
        vmovdqa 0x40(%rdi), %ymm6
        vmovdqa 0x140(%rdi), %ymm10
        vmovdqa 0x60(%rdi), %ymm7
        vmovdqa 0x160(%rdi), %ymm11
        vpbroadcastq 0x48(%rsi), %ymm3
        vpsubw %ymm4, %ymm8, %ymm12
        vpaddw %ymm8, %ymm4, %ymm4
        vpsubw %ymm5, %ymm9, %ymm13
        vpmullw %ymm2, %ymm12, %ymm8
        vpaddw %ymm9, %ymm5, %ymm5
        vpsubw %ymm6, %ymm10, %ymm14
        vpmullw %ymm2, %ymm13, %ymm9
        vpaddw %ymm10, %ymm6, %ymm6
        vpsubw %ymm7, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm10
        vpaddw %ymm11, %ymm7, %ymm7
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm3, %ymm12, %ymm12
        vpmulhw %ymm3, %ymm13, %ymm13
        vpmulhw %ymm3, %ymm14, %ymm14
        vpmulhw %ymm3, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm8, %ymm12, %ymm8
        vpsubw %ymm9, %ymm13, %ymm9
        vpsubw %ymm10, %ymm14, %ymm10
        vpsubw %ymm11, %ymm15, %ymm11
        vmovdqa %ymm4, (%rdi)
        vmovdqa %ymm5, 0x20(%rdi)
        vmovdqa %ymm6, 0x40(%rdi)
        vmovdqa %ymm7, 0x60(%rdi)
        vmovdqa %ymm8, 0x100(%rdi)
        vmovdqa %ymm9, 0x120(%rdi)
        vmovdqa %ymm10, 0x140(%rdi)
        vmovdqa %ymm11, 0x160(%rdi)
        vmovdqa 0x80(%rdi), %ymm4
        vmovdqa 0x180(%rdi), %ymm8
        vmovdqa 0xa0(%rdi), %ymm5
        vmovdqa 0x1a0(%rdi), %ymm9
        vpbroadcastq 0x40(%rsi), %ymm2
        vmovdqa 0xc0(%rdi), %ymm6
        vmovdqa 0x1c0(%rdi), %ymm10
        vmovdqa 0xe0(%rdi), %ymm7
        vmovdqa 0x1e0(%rdi), %ymm11
        vpbroadcastq 0x48(%rsi), %ymm3
        vpsubw %ymm4, %ymm8, %ymm12
        vpaddw %ymm8, %ymm4, %ymm4
        vpsubw %ymm5, %ymm9, %ymm13
        vpmullw %ymm2, %ymm12, %ymm8
        vpaddw %ymm9, %ymm5, %ymm5
        vpsubw %ymm6, %ymm10, %ymm14
        vpmullw %ymm2, %ymm13, %ymm9
        vpaddw %ymm10, %ymm6, %ymm6
        vpsubw %ymm7, %ymm11, %ymm15
        vpmullw %ymm2, %ymm14, %ymm10
        vpaddw %ymm11, %ymm7, %ymm7
        vpmullw %ymm2, %ymm15, %ymm11
        vpmulhw %ymm3, %ymm12, %ymm12
        vpmulhw %ymm3, %ymm13, %ymm13
        vpmulhw %ymm3, %ymm14, %ymm14
        vpmulhw %ymm3, %ymm15, %ymm15
        vpmulhw %ymm0, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm8, %ymm12, %ymm8
        vpsubw %ymm9, %ymm13, %ymm9
        vpsubw %ymm10, %ymm14, %ymm10
        vpsubw %ymm11, %ymm15, %ymm11
        vmovdqa %ymm4, 0x80(%rdi)
        vmovdqa %ymm5, 0xa0(%rdi)
        vmovdqa %ymm6, 0xc0(%rdi)
        vmovdqa %ymm7, 0xe0(%rdi)
        vmovdqa %ymm8, 0x180(%rdi)
        vmovdqa %ymm9, 0x1a0(%rdi)
        vmovdqa %ymm10, 0x1c0(%rdi)
        vmovdqa %ymm11, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_invntt_avx2)

