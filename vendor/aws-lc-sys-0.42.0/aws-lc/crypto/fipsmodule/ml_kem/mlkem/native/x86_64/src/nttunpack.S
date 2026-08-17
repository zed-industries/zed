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
 *   dev/x86_64/src/nttunpack.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_nttunpack_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_nttunpack_avx2)
S2N_BN_SYMBOL(mlkem_nttunpack_avx2):

        .cfi_startproc
        vmovdqa (%rdi), %ymm4
        vmovdqa 0x20(%rdi), %ymm5
        vmovdqa 0x40(%rdi), %ymm6
        vmovdqa 0x60(%rdi), %ymm7
        vmovdqa 0x80(%rdi), %ymm8
        vmovdqa 0xa0(%rdi), %ymm9
        vmovdqa 0xc0(%rdi), %ymm10
        vmovdqa 0xe0(%rdi), %ymm11
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm5, %ymm9 # ymm9 = ymm5[2,3],ymm9[2,3]
        vperm2i128 $0x20, %ymm10, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm6, %ymm10 # ymm10 = ymm6[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm6 # ymm6 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vpunpcklqdq %ymm5, %ymm3, %ymm7 # ymm7 = ymm3[0],ymm5[0],ymm3[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[1],ymm5[1],ymm3[3],ymm5[3]
        vpunpcklqdq %ymm10, %ymm8, %ymm3 # ymm3 = ymm8[0],ymm10[0],ymm8[2],ymm10[2]
        vpunpckhqdq %ymm10, %ymm8, %ymm10 # ymm10 = ymm8[1],ymm10[1],ymm8[3],ymm10[3]
        vpunpcklqdq %ymm6, %ymm4, %ymm8 # ymm8 = ymm4[0],ymm6[0],ymm4[2],ymm6[2]
        vpunpckhqdq %ymm6, %ymm4, %ymm6 # ymm6 = ymm4[1],ymm6[1],ymm4[3],ymm6[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm4 # ymm4 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vmovsldup %ymm8, %ymm9    # ymm9 = ymm8[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm9, %ymm7, %ymm9 # ymm9 = ymm7[0],ymm9[1],ymm7[2],ymm9[3],ymm7[4],ymm9[5],ymm7[6],ymm9[7]
        vpsrlq $0x20, %ymm7, %ymm7
        vpblendd $0xaa, %ymm8, %ymm7, %ymm8 # ymm8 = ymm7[0],ymm8[1],ymm7[2],ymm8[3],ymm7[4],ymm8[5],ymm7[6],ymm8[7]
        vmovsldup %ymm6, %ymm7    # ymm7 = ymm6[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[0],ymm7[1],ymm5[2],ymm7[3],ymm5[4],ymm7[5],ymm5[6],ymm7[7]
        vpsrlq $0x20, %ymm5, %ymm5
        vpblendd $0xaa, %ymm6, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm6[1],ymm5[2],ymm6[3],ymm5[4],ymm6[5],ymm5[6],ymm6[7]
        vmovsldup %ymm4, %ymm5    # ymm5 = ymm4[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[0],ymm5[1],ymm3[2],ymm5[3],ymm3[4],ymm5[5],ymm3[6],ymm5[7]
        vpsrlq $0x20, %ymm3, %ymm3
        vpblendd $0xaa, %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[0],ymm4[1],ymm3[2],ymm4[3],ymm3[4],ymm4[5],ymm3[6],ymm4[7]
        vmovsldup %ymm11, %ymm3   # ymm3 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[0],ymm3[1],ymm10[2],ymm3[3],ymm10[4],ymm3[5],ymm10[6],ymm3[7]
        vpsrlq $0x20, %ymm10, %ymm10
        vpblendd $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7]
        vpslld $0x10, %ymm5, %ymm10
        vpblendw $0xaa, %ymm10, %ymm9, %ymm10 # ymm10 = ymm9[0],ymm10[1],ymm9[2],ymm10[3],ymm9[4],ymm10[5],ymm9[6],ymm10[7],ymm9[8],ymm10[9],ymm9[10],ymm10[11],ymm9[12],ymm10[13],ymm9[14],ymm10[15]
        vpsrld $0x10, %ymm9, %ymm9
        vpblendw $0xaa, %ymm5, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm5[1],ymm9[2],ymm5[3],ymm9[4],ymm5[5],ymm9[6],ymm5[7],ymm9[8],ymm5[9],ymm9[10],ymm5[11],ymm9[12],ymm5[13],ymm9[14],ymm5[15]
        vpslld $0x10, %ymm4, %ymm9
        vpblendw $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7],ymm8[8],ymm9[9],ymm8[10],ymm9[11],ymm8[12],ymm9[13],ymm8[14],ymm9[15]
        vpsrld $0x10, %ymm8, %ymm8
        vpblendw $0xaa, %ymm4, %ymm8, %ymm4 # ymm4 = ymm8[0],ymm4[1],ymm8[2],ymm4[3],ymm8[4],ymm4[5],ymm8[6],ymm4[7],ymm8[8],ymm4[9],ymm8[10],ymm4[11],ymm8[12],ymm4[13],ymm8[14],ymm4[15]
        vpslld $0x10, %ymm3, %ymm8
        vpblendw $0xaa, %ymm8, %ymm7, %ymm8 # ymm8 = ymm7[0],ymm8[1],ymm7[2],ymm8[3],ymm7[4],ymm8[5],ymm7[6],ymm8[7],ymm7[8],ymm8[9],ymm7[10],ymm8[11],ymm7[12],ymm8[13],ymm7[14],ymm8[15]
        vpsrld $0x10, %ymm7, %ymm7
        vpblendw $0xaa, %ymm3, %ymm7, %ymm3 # ymm3 = ymm7[0],ymm3[1],ymm7[2],ymm3[3],ymm7[4],ymm3[5],ymm7[6],ymm3[7],ymm7[8],ymm3[9],ymm7[10],ymm3[11],ymm7[12],ymm3[13],ymm7[14],ymm3[15]
        vpslld $0x10, %ymm11, %ymm7
        vpblendw $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7],ymm6[8],ymm7[9],ymm6[10],ymm7[11],ymm6[12],ymm7[13],ymm6[14],ymm7[15]
        vpsrld $0x10, %ymm6, %ymm6
        vpblendw $0xaa, %ymm11, %ymm6, %ymm11 # ymm11 = ymm6[0],ymm11[1],ymm6[2],ymm11[3],ymm6[4],ymm11[5],ymm6[6],ymm11[7],ymm6[8],ymm11[9],ymm6[10],ymm11[11],ymm6[12],ymm11[13],ymm6[14],ymm11[15]
        vmovdqa %ymm10, (%rdi)
        vmovdqa %ymm5, 0x20(%rdi)
        vmovdqa %ymm9, 0x40(%rdi)
        vmovdqa %ymm4, 0x60(%rdi)
        vmovdqa %ymm8, 0x80(%rdi)
        vmovdqa %ymm3, 0xa0(%rdi)
        vmovdqa %ymm7, 0xc0(%rdi)
        vmovdqa %ymm11, 0xe0(%rdi)
        vmovdqa 0x100(%rdi), %ymm4
        vmovdqa 0x120(%rdi), %ymm5
        vmovdqa 0x140(%rdi), %ymm6
        vmovdqa 0x160(%rdi), %ymm7
        vmovdqa 0x180(%rdi), %ymm8
        vmovdqa 0x1a0(%rdi), %ymm9
        vmovdqa 0x1c0(%rdi), %ymm10
        vmovdqa 0x1e0(%rdi), %ymm11
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm5, %ymm9 # ymm9 = ymm5[2,3],ymm9[2,3]
        vperm2i128 $0x20, %ymm10, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm6, %ymm10 # ymm10 = ymm6[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm6 # ymm6 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vpunpcklqdq %ymm5, %ymm3, %ymm7 # ymm7 = ymm3[0],ymm5[0],ymm3[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[1],ymm5[1],ymm3[3],ymm5[3]
        vpunpcklqdq %ymm10, %ymm8, %ymm3 # ymm3 = ymm8[0],ymm10[0],ymm8[2],ymm10[2]
        vpunpckhqdq %ymm10, %ymm8, %ymm10 # ymm10 = ymm8[1],ymm10[1],ymm8[3],ymm10[3]
        vpunpcklqdq %ymm6, %ymm4, %ymm8 # ymm8 = ymm4[0],ymm6[0],ymm4[2],ymm6[2]
        vpunpckhqdq %ymm6, %ymm4, %ymm6 # ymm6 = ymm4[1],ymm6[1],ymm4[3],ymm6[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm4 # ymm4 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vmovsldup %ymm8, %ymm9    # ymm9 = ymm8[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm9, %ymm7, %ymm9 # ymm9 = ymm7[0],ymm9[1],ymm7[2],ymm9[3],ymm7[4],ymm9[5],ymm7[6],ymm9[7]
        vpsrlq $0x20, %ymm7, %ymm7
        vpblendd $0xaa, %ymm8, %ymm7, %ymm8 # ymm8 = ymm7[0],ymm8[1],ymm7[2],ymm8[3],ymm7[4],ymm8[5],ymm7[6],ymm8[7]
        vmovsldup %ymm6, %ymm7    # ymm7 = ymm6[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[0],ymm7[1],ymm5[2],ymm7[3],ymm5[4],ymm7[5],ymm5[6],ymm7[7]
        vpsrlq $0x20, %ymm5, %ymm5
        vpblendd $0xaa, %ymm6, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm6[1],ymm5[2],ymm6[3],ymm5[4],ymm6[5],ymm5[6],ymm6[7]
        vmovsldup %ymm4, %ymm5    # ymm5 = ymm4[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[0],ymm5[1],ymm3[2],ymm5[3],ymm3[4],ymm5[5],ymm3[6],ymm5[7]
        vpsrlq $0x20, %ymm3, %ymm3
        vpblendd $0xaa, %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[0],ymm4[1],ymm3[2],ymm4[3],ymm3[4],ymm4[5],ymm3[6],ymm4[7]
        vmovsldup %ymm11, %ymm3   # ymm3 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[0],ymm3[1],ymm10[2],ymm3[3],ymm10[4],ymm3[5],ymm10[6],ymm3[7]
        vpsrlq $0x20, %ymm10, %ymm10
        vpblendd $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7]
        vpslld $0x10, %ymm5, %ymm10
        vpblendw $0xaa, %ymm10, %ymm9, %ymm10 # ymm10 = ymm9[0],ymm10[1],ymm9[2],ymm10[3],ymm9[4],ymm10[5],ymm9[6],ymm10[7],ymm9[8],ymm10[9],ymm9[10],ymm10[11],ymm9[12],ymm10[13],ymm9[14],ymm10[15]
        vpsrld $0x10, %ymm9, %ymm9
        vpblendw $0xaa, %ymm5, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm5[1],ymm9[2],ymm5[3],ymm9[4],ymm5[5],ymm9[6],ymm5[7],ymm9[8],ymm5[9],ymm9[10],ymm5[11],ymm9[12],ymm5[13],ymm9[14],ymm5[15]
        vpslld $0x10, %ymm4, %ymm9
        vpblendw $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7],ymm8[8],ymm9[9],ymm8[10],ymm9[11],ymm8[12],ymm9[13],ymm8[14],ymm9[15]
        vpsrld $0x10, %ymm8, %ymm8
        vpblendw $0xaa, %ymm4, %ymm8, %ymm4 # ymm4 = ymm8[0],ymm4[1],ymm8[2],ymm4[3],ymm8[4],ymm4[5],ymm8[6],ymm4[7],ymm8[8],ymm4[9],ymm8[10],ymm4[11],ymm8[12],ymm4[13],ymm8[14],ymm4[15]
        vpslld $0x10, %ymm3, %ymm8
        vpblendw $0xaa, %ymm8, %ymm7, %ymm8 # ymm8 = ymm7[0],ymm8[1],ymm7[2],ymm8[3],ymm7[4],ymm8[5],ymm7[6],ymm8[7],ymm7[8],ymm8[9],ymm7[10],ymm8[11],ymm7[12],ymm8[13],ymm7[14],ymm8[15]
        vpsrld $0x10, %ymm7, %ymm7
        vpblendw $0xaa, %ymm3, %ymm7, %ymm3 # ymm3 = ymm7[0],ymm3[1],ymm7[2],ymm3[3],ymm7[4],ymm3[5],ymm7[6],ymm3[7],ymm7[8],ymm3[9],ymm7[10],ymm3[11],ymm7[12],ymm3[13],ymm7[14],ymm3[15]
        vpslld $0x10, %ymm11, %ymm7
        vpblendw $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7],ymm6[8],ymm7[9],ymm6[10],ymm7[11],ymm6[12],ymm7[13],ymm6[14],ymm7[15]
        vpsrld $0x10, %ymm6, %ymm6
        vpblendw $0xaa, %ymm11, %ymm6, %ymm11 # ymm11 = ymm6[0],ymm11[1],ymm6[2],ymm11[3],ymm6[4],ymm11[5],ymm6[6],ymm11[7],ymm6[8],ymm11[9],ymm6[10],ymm11[11],ymm6[12],ymm11[13],ymm6[14],ymm11[15]
        vmovdqa %ymm10, 0x100(%rdi)
        vmovdqa %ymm5, 0x120(%rdi)
        vmovdqa %ymm9, 0x140(%rdi)
        vmovdqa %ymm4, 0x160(%rdi)
        vmovdqa %ymm8, 0x180(%rdi)
        vmovdqa %ymm3, 0x1a0(%rdi)
        vmovdqa %ymm7, 0x1c0(%rdi)
        vmovdqa %ymm11, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_nttunpack_avx2)

