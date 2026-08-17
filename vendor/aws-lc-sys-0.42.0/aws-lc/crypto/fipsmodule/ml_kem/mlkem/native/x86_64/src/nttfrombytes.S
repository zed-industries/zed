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

#include "_internal_s2n_bignum_x86_att.h"


/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/nttfrombytes.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_nttfrombytes_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_nttfrombytes_avx2)
S2N_BN_SYMBOL(mlkem_nttfrombytes_avx2):

        .cfi_startproc
        movl $0xfff0fff, %eax        # imm = 0xFFF0FFF
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        vmovdqu (%rsi), %ymm4
        vmovdqu 0x20(%rsi), %ymm5
        vmovdqu 0x40(%rsi), %ymm6
        vmovdqu 0x60(%rsi), %ymm7
        vmovdqu 0x80(%rsi), %ymm8
        vmovdqu 0xa0(%rsi), %ymm9
        vperm2i128 $0x20, %ymm7, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm7[0,1]
        vperm2i128 $0x31, %ymm7, %ymm4, %ymm7 # ymm7 = ymm4[2,3],ymm7[2,3]
        vperm2i128 $0x20, %ymm8, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm5, %ymm8 # ymm8 = ymm5[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm6, %ymm9 # ymm9 = ymm6[2,3],ymm9[2,3]
        vpunpcklqdq %ymm8, %ymm3, %ymm6 # ymm6 = ymm3[0],ymm8[0],ymm3[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm3, %ymm8 # ymm8 = ymm3[1],ymm8[1],ymm3[3],ymm8[3]
        vpunpcklqdq %ymm5, %ymm7, %ymm3 # ymm3 = ymm7[0],ymm5[0],ymm7[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm7, %ymm5 # ymm5 = ymm7[1],ymm5[1],ymm7[3],ymm5[3]
        vpunpcklqdq %ymm9, %ymm4, %ymm7 # ymm7 = ymm4[0],ymm9[0],ymm4[2],ymm9[2]
        vpunpckhqdq %ymm9, %ymm4, %ymm9 # ymm9 = ymm4[1],ymm9[1],ymm4[3],ymm9[3]
        vmovsldup %ymm5, %ymm4    # ymm4 = ymm5[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[0],ymm5[1],ymm6[2],ymm5[3],ymm6[4],ymm5[5],ymm6[6],ymm5[7]
        vmovsldup %ymm7, %ymm6    # ymm6 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7]
        vpsrlq $0x20, %ymm8, %ymm8
        vpblendd $0xaa, %ymm7, %ymm8, %ymm7 # ymm7 = ymm8[0],ymm7[1],ymm8[2],ymm7[3],ymm8[4],ymm7[5],ymm8[6],ymm7[7]
        vmovsldup %ymm9, %ymm8    # ymm8 = ymm9[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm8, %ymm3, %ymm8 # ymm8 = ymm3[0],ymm8[1],ymm3[2],ymm8[3],ymm3[4],ymm8[5],ymm3[6],ymm8[7]
        vpsrlq $0x20, %ymm3, %ymm3
        vpblendd $0xaa, %ymm9, %ymm3, %ymm9 # ymm9 = ymm3[0],ymm9[1],ymm3[2],ymm9[3],ymm3[4],ymm9[5],ymm3[6],ymm9[7]
        vpslld $0x10, %ymm7, %ymm10
        vpblendw $0xaa, %ymm10, %ymm4, %ymm10 # ymm10 = ymm4[0],ymm10[1],ymm4[2],ymm10[3],ymm4[4],ymm10[5],ymm4[6],ymm10[7],ymm4[8],ymm10[9],ymm4[10],ymm10[11],ymm4[12],ymm10[13],ymm4[14],ymm10[15]
        vpsrld $0x10, %ymm4, %ymm4
        vpblendw $0xaa, %ymm7, %ymm4, %ymm7 # ymm7 = ymm4[0],ymm7[1],ymm4[2],ymm7[3],ymm4[4],ymm7[5],ymm4[6],ymm7[7],ymm4[8],ymm7[9],ymm4[10],ymm7[11],ymm4[12],ymm7[13],ymm4[14],ymm7[15]
        vpslld $0x10, %ymm8, %ymm4
        vpblendw $0xaa, %ymm4, %ymm5, %ymm4 # ymm4 = ymm5[0],ymm4[1],ymm5[2],ymm4[3],ymm5[4],ymm4[5],ymm5[6],ymm4[7],ymm5[8],ymm4[9],ymm5[10],ymm4[11],ymm5[12],ymm4[13],ymm5[14],ymm4[15]
        vpsrld $0x10, %ymm5, %ymm5
        vpblendw $0xaa, %ymm8, %ymm5, %ymm8 # ymm8 = ymm5[0],ymm8[1],ymm5[2],ymm8[3],ymm5[4],ymm8[5],ymm5[6],ymm8[7],ymm5[8],ymm8[9],ymm5[10],ymm8[11],ymm5[12],ymm8[13],ymm5[14],ymm8[15]
        vpslld $0x10, %ymm9, %ymm5
        vpblendw $0xaa, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[0],ymm5[1],ymm6[2],ymm5[3],ymm6[4],ymm5[5],ymm6[6],ymm5[7],ymm6[8],ymm5[9],ymm6[10],ymm5[11],ymm6[12],ymm5[13],ymm6[14],ymm5[15]
        vpsrld $0x10, %ymm6, %ymm6
        vpblendw $0xaa, %ymm9, %ymm6, %ymm9 # ymm9 = ymm6[0],ymm9[1],ymm6[2],ymm9[3],ymm6[4],ymm9[5],ymm6[6],ymm9[7],ymm6[8],ymm9[9],ymm6[10],ymm9[11],ymm6[12],ymm9[13],ymm6[14],ymm9[15]
        vpsrlw $0xc, %ymm10, %ymm11
        vpsllw $0x4, %ymm7, %ymm12
        vpor %ymm11, %ymm12, %ymm11
        vpand %ymm0, %ymm10, %ymm10
        vpand %ymm0, %ymm11, %ymm11
        vpsrlw $0x8, %ymm7, %ymm12
        vpsllw $0x8, %ymm4, %ymm13
        vpor %ymm12, %ymm13, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpsrlw $0x4, %ymm4, %ymm13
        vpand %ymm0, %ymm13, %ymm13
        vpsrlw $0xc, %ymm8, %ymm14
        vpsllw $0x4, %ymm5, %ymm15
        vpor %ymm14, %ymm15, %ymm14
        vpand %ymm0, %ymm8, %ymm8
        vpand %ymm0, %ymm14, %ymm14
        vpsrlw $0x8, %ymm5, %ymm15
        vpsllw $0x8, %ymm9, %ymm1
        vpor %ymm15, %ymm1, %ymm15
        vpand %ymm0, %ymm15, %ymm15
        vpsrlw $0x4, %ymm9, %ymm1
        vpand %ymm0, %ymm1, %ymm1
        vmovdqa %ymm10, (%rdi)
        vmovdqa %ymm11, 0x20(%rdi)
        vmovdqa %ymm12, 0x40(%rdi)
        vmovdqa %ymm13, 0x60(%rdi)
        vmovdqa %ymm8, 0x80(%rdi)
        vmovdqa %ymm14, 0xa0(%rdi)
        vmovdqa %ymm15, 0xc0(%rdi)
        vmovdqa %ymm1, 0xe0(%rdi)
        vmovdqu 0xc0(%rsi), %ymm4
        vmovdqu 0xe0(%rsi), %ymm5
        vmovdqu 0x100(%rsi), %ymm6
        vmovdqu 0x120(%rsi), %ymm7
        vmovdqu 0x140(%rsi), %ymm8
        vmovdqu 0x160(%rsi), %ymm9
        vperm2i128 $0x20, %ymm7, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm7[0,1]
        vperm2i128 $0x31, %ymm7, %ymm4, %ymm7 # ymm7 = ymm4[2,3],ymm7[2,3]
        vperm2i128 $0x20, %ymm8, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm5, %ymm8 # ymm8 = ymm5[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm6, %ymm9 # ymm9 = ymm6[2,3],ymm9[2,3]
        vpunpcklqdq %ymm8, %ymm3, %ymm6 # ymm6 = ymm3[0],ymm8[0],ymm3[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm3, %ymm8 # ymm8 = ymm3[1],ymm8[1],ymm3[3],ymm8[3]
        vpunpcklqdq %ymm5, %ymm7, %ymm3 # ymm3 = ymm7[0],ymm5[0],ymm7[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm7, %ymm5 # ymm5 = ymm7[1],ymm5[1],ymm7[3],ymm5[3]
        vpunpcklqdq %ymm9, %ymm4, %ymm7 # ymm7 = ymm4[0],ymm9[0],ymm4[2],ymm9[2]
        vpunpckhqdq %ymm9, %ymm4, %ymm9 # ymm9 = ymm4[1],ymm9[1],ymm4[3],ymm9[3]
        vmovsldup %ymm5, %ymm4    # ymm4 = ymm5[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[0],ymm5[1],ymm6[2],ymm5[3],ymm6[4],ymm5[5],ymm6[6],ymm5[7]
        vmovsldup %ymm7, %ymm6    # ymm6 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7]
        vpsrlq $0x20, %ymm8, %ymm8
        vpblendd $0xaa, %ymm7, %ymm8, %ymm7 # ymm7 = ymm8[0],ymm7[1],ymm8[2],ymm7[3],ymm8[4],ymm7[5],ymm8[6],ymm7[7]
        vmovsldup %ymm9, %ymm8    # ymm8 = ymm9[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm8, %ymm3, %ymm8 # ymm8 = ymm3[0],ymm8[1],ymm3[2],ymm8[3],ymm3[4],ymm8[5],ymm3[6],ymm8[7]
        vpsrlq $0x20, %ymm3, %ymm3
        vpblendd $0xaa, %ymm9, %ymm3, %ymm9 # ymm9 = ymm3[0],ymm9[1],ymm3[2],ymm9[3],ymm3[4],ymm9[5],ymm3[6],ymm9[7]
        vpslld $0x10, %ymm7, %ymm10
        vpblendw $0xaa, %ymm10, %ymm4, %ymm10 # ymm10 = ymm4[0],ymm10[1],ymm4[2],ymm10[3],ymm4[4],ymm10[5],ymm4[6],ymm10[7],ymm4[8],ymm10[9],ymm4[10],ymm10[11],ymm4[12],ymm10[13],ymm4[14],ymm10[15]
        vpsrld $0x10, %ymm4, %ymm4
        vpblendw $0xaa, %ymm7, %ymm4, %ymm7 # ymm7 = ymm4[0],ymm7[1],ymm4[2],ymm7[3],ymm4[4],ymm7[5],ymm4[6],ymm7[7],ymm4[8],ymm7[9],ymm4[10],ymm7[11],ymm4[12],ymm7[13],ymm4[14],ymm7[15]
        vpslld $0x10, %ymm8, %ymm4
        vpblendw $0xaa, %ymm4, %ymm5, %ymm4 # ymm4 = ymm5[0],ymm4[1],ymm5[2],ymm4[3],ymm5[4],ymm4[5],ymm5[6],ymm4[7],ymm5[8],ymm4[9],ymm5[10],ymm4[11],ymm5[12],ymm4[13],ymm5[14],ymm4[15]
        vpsrld $0x10, %ymm5, %ymm5
        vpblendw $0xaa, %ymm8, %ymm5, %ymm8 # ymm8 = ymm5[0],ymm8[1],ymm5[2],ymm8[3],ymm5[4],ymm8[5],ymm5[6],ymm8[7],ymm5[8],ymm8[9],ymm5[10],ymm8[11],ymm5[12],ymm8[13],ymm5[14],ymm8[15]
        vpslld $0x10, %ymm9, %ymm5
        vpblendw $0xaa, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[0],ymm5[1],ymm6[2],ymm5[3],ymm6[4],ymm5[5],ymm6[6],ymm5[7],ymm6[8],ymm5[9],ymm6[10],ymm5[11],ymm6[12],ymm5[13],ymm6[14],ymm5[15]
        vpsrld $0x10, %ymm6, %ymm6
        vpblendw $0xaa, %ymm9, %ymm6, %ymm9 # ymm9 = ymm6[0],ymm9[1],ymm6[2],ymm9[3],ymm6[4],ymm9[5],ymm6[6],ymm9[7],ymm6[8],ymm9[9],ymm6[10],ymm9[11],ymm6[12],ymm9[13],ymm6[14],ymm9[15]
        vpsrlw $0xc, %ymm10, %ymm11
        vpsllw $0x4, %ymm7, %ymm12
        vpor %ymm11, %ymm12, %ymm11
        vpand %ymm0, %ymm10, %ymm10
        vpand %ymm0, %ymm11, %ymm11
        vpsrlw $0x8, %ymm7, %ymm12
        vpsllw $0x8, %ymm4, %ymm13
        vpor %ymm12, %ymm13, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpsrlw $0x4, %ymm4, %ymm13
        vpand %ymm0, %ymm13, %ymm13
        vpsrlw $0xc, %ymm8, %ymm14
        vpsllw $0x4, %ymm5, %ymm15
        vpor %ymm14, %ymm15, %ymm14
        vpand %ymm0, %ymm8, %ymm8
        vpand %ymm0, %ymm14, %ymm14
        vpsrlw $0x8, %ymm5, %ymm15
        vpsllw $0x8, %ymm9, %ymm1
        vpor %ymm15, %ymm1, %ymm15
        vpand %ymm0, %ymm15, %ymm15
        vpsrlw $0x4, %ymm9, %ymm1
        vpand %ymm0, %ymm1, %ymm1
        vmovdqa %ymm10, 0x100(%rdi)
        vmovdqa %ymm11, 0x120(%rdi)
        vmovdqa %ymm12, 0x140(%rdi)
        vmovdqa %ymm13, 0x160(%rdi)
        vmovdqa %ymm8, 0x180(%rdi)
        vmovdqa %ymm14, 0x1a0(%rdi)
        vmovdqa %ymm15, 0x1c0(%rdi)
        vmovdqa %ymm1, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_nttfrombytes_avx2)

