/*
 * Copyright (c) The mlkem-native project authors
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [REF_AVX2]
 *   CRYSTALS-Dilithium optimized AVX2 implementation
 *   Bai, Ducas, Kiltz, Lepoint, Lyubashevsky, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/dilithium/tree/master/avx2
 */

/*
 * This file is derived from the public domain
 * AVX2 Dilithium implementation @[REF_AVX2].
 */

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/x86_64/src/nttunpack_avx2_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_nttunpack_avx2_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_nttunpack_avx2_asm)
S2N_BN_SYMBOL(mldsa_nttunpack_avx2_asm):

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
        vmovdqa %ymm9, (%rdi)
        vmovdqa %ymm8, 0x20(%rdi)
        vmovdqa %ymm7, 0x40(%rdi)
        vmovdqa %ymm6, 0x60(%rdi)
        vmovdqa %ymm5, 0x80(%rdi)
        vmovdqa %ymm4, 0xa0(%rdi)
        vmovdqa %ymm3, 0xc0(%rdi)
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
        vmovdqa %ymm9, 0x100(%rdi)
        vmovdqa %ymm8, 0x120(%rdi)
        vmovdqa %ymm7, 0x140(%rdi)
        vmovdqa %ymm6, 0x160(%rdi)
        vmovdqa %ymm5, 0x180(%rdi)
        vmovdqa %ymm4, 0x1a0(%rdi)
        vmovdqa %ymm3, 0x1c0(%rdi)
        vmovdqa %ymm11, 0x1e0(%rdi)
        vmovdqa 0x200(%rdi), %ymm4
        vmovdqa 0x220(%rdi), %ymm5
        vmovdqa 0x240(%rdi), %ymm6
        vmovdqa 0x260(%rdi), %ymm7
        vmovdqa 0x280(%rdi), %ymm8
        vmovdqa 0x2a0(%rdi), %ymm9
        vmovdqa 0x2c0(%rdi), %ymm10
        vmovdqa 0x2e0(%rdi), %ymm11
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
        vmovdqa %ymm9, 0x200(%rdi)
        vmovdqa %ymm8, 0x220(%rdi)
        vmovdqa %ymm7, 0x240(%rdi)
        vmovdqa %ymm6, 0x260(%rdi)
        vmovdqa %ymm5, 0x280(%rdi)
        vmovdqa %ymm4, 0x2a0(%rdi)
        vmovdqa %ymm3, 0x2c0(%rdi)
        vmovdqa %ymm11, 0x2e0(%rdi)
        vmovdqa 0x300(%rdi), %ymm4
        vmovdqa 0x320(%rdi), %ymm5
        vmovdqa 0x340(%rdi), %ymm6
        vmovdqa 0x360(%rdi), %ymm7
        vmovdqa 0x380(%rdi), %ymm8
        vmovdqa 0x3a0(%rdi), %ymm9
        vmovdqa 0x3c0(%rdi), %ymm10
        vmovdqa 0x3e0(%rdi), %ymm11
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
        vmovdqa %ymm9, 0x300(%rdi)
        vmovdqa %ymm8, 0x320(%rdi)
        vmovdqa %ymm7, 0x340(%rdi)
        vmovdqa %ymm6, 0x360(%rdi)
        vmovdqa %ymm5, 0x380(%rdi)
        vmovdqa %ymm4, 0x3a0(%rdi)
        vmovdqa %ymm3, 0x3c0(%rdi)
        vmovdqa %ymm11, 0x3e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_nttunpack_avx2_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
