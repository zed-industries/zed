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
 *   dev/x86_64/src/intt_avx2_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_invntt_avx2_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_invntt_avx2_asm)
S2N_BN_SYMBOL(mldsa_invntt_avx2_asm):

        .cfi_startproc
        vmovdqa (%rsi), %ymm0
        vmovdqa (%rdi), %ymm4
        vmovdqa 0x20(%rdi), %ymm5
        vmovdqa 0x40(%rdi), %ymm6
        vmovdqa 0x60(%rdi), %ymm7
        vmovdqa 0x80(%rdi), %ymm8
        vmovdqa 0xa0(%rdi), %ymm9
        vmovdqa 0xc0(%rdi), %ymm10
        vmovdqa 0xe0(%rdi), %ymm11
        vpermq $0x1b, 0x500(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x9a0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm5, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpermq $0x1b, 0x480(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x920(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x400(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x8a0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm9, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpermq $0x1b, 0x380(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x820(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm10, %ymm11, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x300(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x7a0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x280(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x720(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x200(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x6a0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovsldup %ymm5, %ymm3    # ymm3 = ymm5[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm4, %ymm3 # ymm3 = ymm4[0],ymm3[1],ymm4[2],ymm3[3],ymm4[4],ymm3[5],ymm4[6],ymm3[7]
        vpsrlq $0x20, %ymm4, %ymm4
        vpblendd $0xaa, %ymm5, %ymm4, %ymm5 # ymm5 = ymm4[0],ymm5[1],ymm4[2],ymm5[3],ymm4[4],ymm5[5],ymm4[6],ymm5[7]
        vmovsldup %ymm7, %ymm4    # ymm4 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7]
        vmovsldup %ymm9, %ymm6    # ymm6 = ymm9[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7]
        vpsrlq $0x20, %ymm8, %ymm8
        vpblendd $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7]
        vmovsldup %ymm11, %ymm8   # ymm8 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm8, %ymm10, %ymm8 # ymm8 = ymm10[0],ymm8[1],ymm10[2],ymm8[3],ymm10[4],ymm8[5],ymm10[6],ymm8[7]
        vpsrlq $0x20, %ymm10, %ymm10
        vpblendd $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7]
        vpermq $0x1b, 0x180(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x620(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm4, %ymm7, %ymm12
        vpaddd %ymm7, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm6, %ymm9, %ymm12
        vpaddd %ymm6, %ymm9, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm8, %ymm11, %ymm12
        vpaddd %ymm11, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpunpcklqdq %ymm4, %ymm3, %ymm10 # ymm10 = ymm3[0],ymm4[0],ymm3[2],ymm4[2]
        vpunpckhqdq %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[1],ymm4[1],ymm3[3],ymm4[3]
        vpunpcklqdq %ymm8, %ymm6, %ymm3 # ymm3 = ymm6[0],ymm8[0],ymm6[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm6, %ymm8 # ymm8 = ymm6[1],ymm8[1],ymm6[3],ymm8[3]
        vpunpcklqdq %ymm7, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm7[0],ymm5[2],ymm7[2]
        vpunpckhqdq %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[1],ymm7[1],ymm5[3],ymm7[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vpermq $0x1b, 0x100(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x5a0(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm10, %ymm4, %ymm12
        vpaddd %ymm4, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm4   # ymm4 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm4, %ymm4
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm4, %ymm12, %ymm4 # ymm4 = ymm12[0],ymm4[1],ymm12[2],ymm4[3],ymm12[4],ymm4[5],ymm12[6],ymm4[7]
        vpsubd %ymm3, %ymm8, %ymm12
        vpaddd %ymm3, %ymm8, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm5, %ymm11, %ymm12
        vpaddd %ymm5, %ymm11, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vperm2i128 $0x20, %ymm3, %ymm10, %ymm9 # ymm9 = ymm10[0,1],ymm3[0,1]
        vperm2i128 $0x31, %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[2,3],ymm3[2,3]
        vperm2i128 $0x20, %ymm5, %ymm6, %ymm10 # ymm10 = ymm6[0,1],ymm5[0,1]
        vperm2i128 $0x31, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[2,3],ymm5[2,3]
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm6 # ymm6 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm4 # ymm4 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vpbroadcastd 0x9c(%rsi), %ymm1
        vpbroadcastd 0x53c(%rsi), %ymm2
        vpsubd %ymm9, %ymm3, %ymm12
        vpaddd %ymm3, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm3   # ymm3 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm3, %ymm3
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm3, %ymm12, %ymm3 # ymm3 = ymm12[0],ymm3[1],ymm12[2],ymm3[3],ymm12[4],ymm3[5],ymm12[6],ymm3[7]
        vpsubd %ymm10, %ymm5, %ymm12
        vpaddd %ymm5, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm4, %ymm11, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm9, (%rdi)
        vmovdqa %ymm10, 0x20(%rdi)
        vmovdqa %ymm6, 0x40(%rdi)
        vmovdqa %ymm4, 0x60(%rdi)
        vmovdqa %ymm3, 0x80(%rdi)
        vmovdqa %ymm5, 0xa0(%rdi)
        vmovdqa %ymm8, 0xc0(%rdi)
        vmovdqa %ymm11, 0xe0(%rdi)
        vmovdqa 0x100(%rdi), %ymm4
        vmovdqa 0x120(%rdi), %ymm5
        vmovdqa 0x140(%rdi), %ymm6
        vmovdqa 0x160(%rdi), %ymm7
        vmovdqa 0x180(%rdi), %ymm8
        vmovdqa 0x1a0(%rdi), %ymm9
        vmovdqa 0x1c0(%rdi), %ymm10
        vmovdqa 0x1e0(%rdi), %ymm11
        vpermq $0x1b, 0x4e0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x980(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm5, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpermq $0x1b, 0x460(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x900(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x3e0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x880(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm9, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpermq $0x1b, 0x360(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x800(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm10, %ymm11, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x2e0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x780(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x260(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x700(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x1e0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x680(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovsldup %ymm5, %ymm3    # ymm3 = ymm5[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm4, %ymm3 # ymm3 = ymm4[0],ymm3[1],ymm4[2],ymm3[3],ymm4[4],ymm3[5],ymm4[6],ymm3[7]
        vpsrlq $0x20, %ymm4, %ymm4
        vpblendd $0xaa, %ymm5, %ymm4, %ymm5 # ymm5 = ymm4[0],ymm5[1],ymm4[2],ymm5[3],ymm4[4],ymm5[5],ymm4[6],ymm5[7]
        vmovsldup %ymm7, %ymm4    # ymm4 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7]
        vmovsldup %ymm9, %ymm6    # ymm6 = ymm9[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7]
        vpsrlq $0x20, %ymm8, %ymm8
        vpblendd $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7]
        vmovsldup %ymm11, %ymm8   # ymm8 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm8, %ymm10, %ymm8 # ymm8 = ymm10[0],ymm8[1],ymm10[2],ymm8[3],ymm10[4],ymm8[5],ymm10[6],ymm8[7]
        vpsrlq $0x20, %ymm10, %ymm10
        vpblendd $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7]
        vpermq $0x1b, 0x160(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x600(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm4, %ymm7, %ymm12
        vpaddd %ymm7, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm6, %ymm9, %ymm12
        vpaddd %ymm6, %ymm9, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm8, %ymm11, %ymm12
        vpaddd %ymm11, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpunpcklqdq %ymm4, %ymm3, %ymm10 # ymm10 = ymm3[0],ymm4[0],ymm3[2],ymm4[2]
        vpunpckhqdq %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[1],ymm4[1],ymm3[3],ymm4[3]
        vpunpcklqdq %ymm8, %ymm6, %ymm3 # ymm3 = ymm6[0],ymm8[0],ymm6[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm6, %ymm8 # ymm8 = ymm6[1],ymm8[1],ymm6[3],ymm8[3]
        vpunpcklqdq %ymm7, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm7[0],ymm5[2],ymm7[2]
        vpunpckhqdq %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[1],ymm7[1],ymm5[3],ymm7[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vpermq $0x1b, 0xe0(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x580(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm10, %ymm4, %ymm12
        vpaddd %ymm4, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm4   # ymm4 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm4, %ymm4
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm4, %ymm12, %ymm4 # ymm4 = ymm12[0],ymm4[1],ymm12[2],ymm4[3],ymm12[4],ymm4[5],ymm12[6],ymm4[7]
        vpsubd %ymm3, %ymm8, %ymm12
        vpaddd %ymm3, %ymm8, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm5, %ymm11, %ymm12
        vpaddd %ymm5, %ymm11, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vperm2i128 $0x20, %ymm3, %ymm10, %ymm9 # ymm9 = ymm10[0,1],ymm3[0,1]
        vperm2i128 $0x31, %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[2,3],ymm3[2,3]
        vperm2i128 $0x20, %ymm5, %ymm6, %ymm10 # ymm10 = ymm6[0,1],ymm5[0,1]
        vperm2i128 $0x31, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[2,3],ymm5[2,3]
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm6 # ymm6 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm4 # ymm4 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vpbroadcastd 0x98(%rsi), %ymm1
        vpbroadcastd 0x538(%rsi), %ymm2
        vpsubd %ymm9, %ymm3, %ymm12
        vpaddd %ymm3, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm3   # ymm3 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm3, %ymm3
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm3, %ymm12, %ymm3 # ymm3 = ymm12[0],ymm3[1],ymm12[2],ymm3[3],ymm12[4],ymm3[5],ymm12[6],ymm3[7]
        vpsubd %ymm10, %ymm5, %ymm12
        vpaddd %ymm5, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm4, %ymm11, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm9, 0x100(%rdi)
        vmovdqa %ymm10, 0x120(%rdi)
        vmovdqa %ymm6, 0x140(%rdi)
        vmovdqa %ymm4, 0x160(%rdi)
        vmovdqa %ymm3, 0x180(%rdi)
        vmovdqa %ymm5, 0x1a0(%rdi)
        vmovdqa %ymm8, 0x1c0(%rdi)
        vmovdqa %ymm11, 0x1e0(%rdi)
        vmovdqa 0x200(%rdi), %ymm4
        vmovdqa 0x220(%rdi), %ymm5
        vmovdqa 0x240(%rdi), %ymm6
        vmovdqa 0x260(%rdi), %ymm7
        vmovdqa 0x280(%rdi), %ymm8
        vmovdqa 0x2a0(%rdi), %ymm9
        vmovdqa 0x2c0(%rdi), %ymm10
        vmovdqa 0x2e0(%rdi), %ymm11
        vpermq $0x1b, 0x4c0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x960(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm5, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpermq $0x1b, 0x440(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x8e0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x3c0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x860(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm9, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpermq $0x1b, 0x340(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x7e0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm10, %ymm11, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x2c0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x760(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x240(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x6e0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x1c0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x660(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovsldup %ymm5, %ymm3    # ymm3 = ymm5[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm4, %ymm3 # ymm3 = ymm4[0],ymm3[1],ymm4[2],ymm3[3],ymm4[4],ymm3[5],ymm4[6],ymm3[7]
        vpsrlq $0x20, %ymm4, %ymm4
        vpblendd $0xaa, %ymm5, %ymm4, %ymm5 # ymm5 = ymm4[0],ymm5[1],ymm4[2],ymm5[3],ymm4[4],ymm5[5],ymm4[6],ymm5[7]
        vmovsldup %ymm7, %ymm4    # ymm4 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7]
        vmovsldup %ymm9, %ymm6    # ymm6 = ymm9[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7]
        vpsrlq $0x20, %ymm8, %ymm8
        vpblendd $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7]
        vmovsldup %ymm11, %ymm8   # ymm8 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm8, %ymm10, %ymm8 # ymm8 = ymm10[0],ymm8[1],ymm10[2],ymm8[3],ymm10[4],ymm8[5],ymm10[6],ymm8[7]
        vpsrlq $0x20, %ymm10, %ymm10
        vpblendd $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7]
        vpermq $0x1b, 0x140(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x5e0(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm4, %ymm7, %ymm12
        vpaddd %ymm7, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm6, %ymm9, %ymm12
        vpaddd %ymm6, %ymm9, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm8, %ymm11, %ymm12
        vpaddd %ymm11, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpunpcklqdq %ymm4, %ymm3, %ymm10 # ymm10 = ymm3[0],ymm4[0],ymm3[2],ymm4[2]
        vpunpckhqdq %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[1],ymm4[1],ymm3[3],ymm4[3]
        vpunpcklqdq %ymm8, %ymm6, %ymm3 # ymm3 = ymm6[0],ymm8[0],ymm6[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm6, %ymm8 # ymm8 = ymm6[1],ymm8[1],ymm6[3],ymm8[3]
        vpunpcklqdq %ymm7, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm7[0],ymm5[2],ymm7[2]
        vpunpckhqdq %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[1],ymm7[1],ymm5[3],ymm7[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vpermq $0x1b, 0xc0(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x560(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm10, %ymm4, %ymm12
        vpaddd %ymm4, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm4   # ymm4 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm4, %ymm4
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm4, %ymm12, %ymm4 # ymm4 = ymm12[0],ymm4[1],ymm12[2],ymm4[3],ymm12[4],ymm4[5],ymm12[6],ymm4[7]
        vpsubd %ymm3, %ymm8, %ymm12
        vpaddd %ymm3, %ymm8, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm5, %ymm11, %ymm12
        vpaddd %ymm5, %ymm11, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vperm2i128 $0x20, %ymm3, %ymm10, %ymm9 # ymm9 = ymm10[0,1],ymm3[0,1]
        vperm2i128 $0x31, %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[2,3],ymm3[2,3]
        vperm2i128 $0x20, %ymm5, %ymm6, %ymm10 # ymm10 = ymm6[0,1],ymm5[0,1]
        vperm2i128 $0x31, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[2,3],ymm5[2,3]
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm6 # ymm6 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm4 # ymm4 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vpbroadcastd 0x94(%rsi), %ymm1
        vpbroadcastd 0x534(%rsi), %ymm2
        vpsubd %ymm9, %ymm3, %ymm12
        vpaddd %ymm3, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm3   # ymm3 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm3, %ymm3
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm3, %ymm12, %ymm3 # ymm3 = ymm12[0],ymm3[1],ymm12[2],ymm3[3],ymm12[4],ymm3[5],ymm12[6],ymm3[7]
        vpsubd %ymm10, %ymm5, %ymm12
        vpaddd %ymm5, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm4, %ymm11, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm9, 0x200(%rdi)
        vmovdqa %ymm10, 0x220(%rdi)
        vmovdqa %ymm6, 0x240(%rdi)
        vmovdqa %ymm4, 0x260(%rdi)
        vmovdqa %ymm3, 0x280(%rdi)
        vmovdqa %ymm5, 0x2a0(%rdi)
        vmovdqa %ymm8, 0x2c0(%rdi)
        vmovdqa %ymm11, 0x2e0(%rdi)
        vmovdqa 0x300(%rdi), %ymm4
        vmovdqa 0x320(%rdi), %ymm5
        vmovdqa 0x340(%rdi), %ymm6
        vmovdqa 0x360(%rdi), %ymm7
        vmovdqa 0x380(%rdi), %ymm8
        vmovdqa 0x3a0(%rdi), %ymm9
        vmovdqa 0x3c0(%rdi), %ymm10
        vmovdqa 0x3e0(%rdi), %ymm11
        vpermq $0x1b, 0x4a0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x940(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm5, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpermq $0x1b, 0x420(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x8c0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x3a0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x840(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm9, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpermq $0x1b, 0x320(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x7c0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm10, %ymm11, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x2a0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x740(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpermq $0x1b, 0x220(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x6c0(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpermq $0x1b, 0x1a0(%rsi), %ymm3 # ymm3 = mem[3,2,1,0]
        vpermq $0x1b, 0x640(%rsi), %ymm15 # ymm15 = mem[3,2,1,0]
        vmovshdup %ymm3, %ymm1    # ymm1 = ymm3[1,1,3,3,5,5,7,7]
        vmovshdup %ymm15, %ymm2   # ymm2 = ymm15[1,1,3,3,5,5,7,7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm3, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm15, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovsldup %ymm5, %ymm3    # ymm3 = ymm5[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm3, %ymm4, %ymm3 # ymm3 = ymm4[0],ymm3[1],ymm4[2],ymm3[3],ymm4[4],ymm3[5],ymm4[6],ymm3[7]
        vpsrlq $0x20, %ymm4, %ymm4
        vpblendd $0xaa, %ymm5, %ymm4, %ymm5 # ymm5 = ymm4[0],ymm5[1],ymm4[2],ymm5[3],ymm4[4],ymm5[5],ymm4[6],ymm5[7]
        vmovsldup %ymm7, %ymm4    # ymm4 = ymm7[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm4, %ymm6, %ymm4 # ymm4 = ymm6[0],ymm4[1],ymm6[2],ymm4[3],ymm6[4],ymm4[5],ymm6[6],ymm4[7]
        vpsrlq $0x20, %ymm6, %ymm6
        vpblendd $0xaa, %ymm7, %ymm6, %ymm7 # ymm7 = ymm6[0],ymm7[1],ymm6[2],ymm7[3],ymm6[4],ymm7[5],ymm6[6],ymm7[7]
        vmovsldup %ymm9, %ymm6    # ymm6 = ymm9[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm6, %ymm8, %ymm6 # ymm6 = ymm8[0],ymm6[1],ymm8[2],ymm6[3],ymm8[4],ymm6[5],ymm8[6],ymm6[7]
        vpsrlq $0x20, %ymm8, %ymm8
        vpblendd $0xaa, %ymm9, %ymm8, %ymm9 # ymm9 = ymm8[0],ymm9[1],ymm8[2],ymm9[3],ymm8[4],ymm9[5],ymm8[6],ymm9[7]
        vmovsldup %ymm11, %ymm8   # ymm8 = ymm11[0,0,2,2,4,4,6,6]
        vpblendd $0xaa, %ymm8, %ymm10, %ymm8 # ymm8 = ymm10[0],ymm8[1],ymm10[2],ymm8[3],ymm10[4],ymm8[5],ymm10[6],ymm8[7]
        vpsrlq $0x20, %ymm10, %ymm10
        vpblendd $0xaa, %ymm11, %ymm10, %ymm11 # ymm11 = ymm10[0],ymm11[1],ymm10[2],ymm11[3],ymm10[4],ymm11[5],ymm10[6],ymm11[7]
        vpermq $0x1b, 0x120(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x5c0(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm4, %ymm7, %ymm12
        vpaddd %ymm7, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm6, %ymm9, %ymm12
        vpaddd %ymm6, %ymm9, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm8, %ymm11, %ymm12
        vpaddd %ymm11, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpunpcklqdq %ymm4, %ymm3, %ymm10 # ymm10 = ymm3[0],ymm4[0],ymm3[2],ymm4[2]
        vpunpckhqdq %ymm4, %ymm3, %ymm4 # ymm4 = ymm3[1],ymm4[1],ymm3[3],ymm4[3]
        vpunpcklqdq %ymm8, %ymm6, %ymm3 # ymm3 = ymm6[0],ymm8[0],ymm6[2],ymm8[2]
        vpunpckhqdq %ymm8, %ymm6, %ymm8 # ymm8 = ymm6[1],ymm8[1],ymm6[3],ymm8[3]
        vpunpcklqdq %ymm7, %ymm5, %ymm6 # ymm6 = ymm5[0],ymm7[0],ymm5[2],ymm7[2]
        vpunpckhqdq %ymm7, %ymm5, %ymm7 # ymm7 = ymm5[1],ymm7[1],ymm5[3],ymm7[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm5 # ymm5 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vpermq $0x1b, 0xa0(%rsi), %ymm1 # ymm1 = mem[3,2,1,0]
        vpermq $0x1b, 0x540(%rsi), %ymm2 # ymm2 = mem[3,2,1,0]
        vpsubd %ymm10, %ymm4, %ymm12
        vpaddd %ymm4, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm4   # ymm4 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm4, %ymm4
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm4, %ymm12, %ymm4 # ymm4 = ymm12[0],ymm4[1],ymm12[2],ymm4[3],ymm12[4],ymm4[5],ymm12[6],ymm4[7]
        vpsubd %ymm3, %ymm8, %ymm12
        vpaddd %ymm3, %ymm8, %ymm3
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm7, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpsubd %ymm5, %ymm11, %ymm12
        vpaddd %ymm5, %ymm11, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vperm2i128 $0x20, %ymm3, %ymm10, %ymm9 # ymm9 = ymm10[0,1],ymm3[0,1]
        vperm2i128 $0x31, %ymm3, %ymm10, %ymm3 # ymm3 = ymm10[2,3],ymm3[2,3]
        vperm2i128 $0x20, %ymm5, %ymm6, %ymm10 # ymm10 = ymm6[0,1],ymm5[0,1]
        vperm2i128 $0x31, %ymm5, %ymm6, %ymm5 # ymm5 = ymm6[2,3],ymm5[2,3]
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm6 # ymm6 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm4 # ymm4 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vpbroadcastd 0x90(%rsi), %ymm1
        vpbroadcastd 0x530(%rsi), %ymm2
        vpsubd %ymm9, %ymm3, %ymm12
        vpaddd %ymm3, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm3   # ymm3 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm3, %ymm3
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm3, %ymm12, %ymm3 # ymm3 = ymm12[0],ymm3[1],ymm12[2],ymm3[3],ymm12[4],ymm3[5],ymm12[6],ymm3[7]
        vpsubd %ymm10, %ymm5, %ymm12
        vpaddd %ymm5, %ymm10, %ymm10
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm5   # ymm5 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm5, %ymm5
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm5, %ymm12, %ymm5 # ymm5 = ymm12[0],ymm5[1],ymm12[2],ymm5[3],ymm12[4],ymm5[5],ymm12[6],ymm5[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm4, %ymm11, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm9, 0x300(%rdi)
        vmovdqa %ymm10, 0x320(%rdi)
        vmovdqa %ymm6, 0x340(%rdi)
        vmovdqa %ymm4, 0x360(%rdi)
        vmovdqa %ymm3, 0x380(%rdi)
        vmovdqa %ymm5, 0x3a0(%rdi)
        vmovdqa %ymm8, 0x3c0(%rdi)
        vmovdqa %ymm11, 0x3e0(%rdi)
        vmovdqa (%rdi), %ymm4
        vmovdqa 0x80(%rdi), %ymm5
        vmovdqa 0x100(%rdi), %ymm6
        vmovdqa 0x180(%rdi), %ymm7
        vmovdqa 0x200(%rdi), %ymm8
        vmovdqa 0x280(%rdi), %ymm9
        vmovdqa 0x300(%rdi), %ymm10
        vmovdqa 0x380(%rdi), %ymm11
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpbroadcastd 0x80(%rsi), %ymm1
        vpbroadcastd 0x520(%rsi), %ymm2
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm8, 0x200(%rdi)
        vmovdqa %ymm9, 0x280(%rdi)
        vmovdqa %ymm10, 0x300(%rdi)
        vmovdqa %ymm11, 0x380(%rdi)
        vmovdqa 0x40(%rsi), %ymm1
        vmovdqa 0x60(%rsi), %ymm2
        vpmuldq %ymm1, %ymm4, %ymm12
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm4, %ymm8    # ymm8 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm9    # ymm9 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm4, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm8[1],ymm4[2],ymm8[3],ymm4[4],ymm8[5],ymm4[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm9[1],ymm5[2],ymm9[3],ymm5[4],ymm9[5],ymm5[6],ymm9[7]
        vpmuldq %ymm1, %ymm6, %ymm12
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm6, %ymm8    # ymm8 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm9    # ymm9 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm6, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm8[1],ymm6[2],ymm8[3],ymm6[4],ymm8[5],ymm6[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm9[1],ymm7[2],ymm9[3],ymm7[4],ymm9[5],ymm7[6],ymm9[7]
        vmovdqa %ymm4, (%rdi)
        vmovdqa %ymm5, 0x80(%rdi)
        vmovdqa %ymm6, 0x100(%rdi)
        vmovdqa %ymm7, 0x180(%rdi)
        vmovdqa 0x20(%rdi), %ymm4
        vmovdqa 0xa0(%rdi), %ymm5
        vmovdqa 0x120(%rdi), %ymm6
        vmovdqa 0x1a0(%rdi), %ymm7
        vmovdqa 0x220(%rdi), %ymm8
        vmovdqa 0x2a0(%rdi), %ymm9
        vmovdqa 0x320(%rdi), %ymm10
        vmovdqa 0x3a0(%rdi), %ymm11
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpbroadcastd 0x80(%rsi), %ymm1
        vpbroadcastd 0x520(%rsi), %ymm2
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm8, 0x220(%rdi)
        vmovdqa %ymm9, 0x2a0(%rdi)
        vmovdqa %ymm10, 0x320(%rdi)
        vmovdqa %ymm11, 0x3a0(%rdi)
        vmovdqa 0x40(%rsi), %ymm1
        vmovdqa 0x60(%rsi), %ymm2
        vpmuldq %ymm1, %ymm4, %ymm12
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm4, %ymm8    # ymm8 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm9    # ymm9 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm4, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm8[1],ymm4[2],ymm8[3],ymm4[4],ymm8[5],ymm4[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm9[1],ymm5[2],ymm9[3],ymm5[4],ymm9[5],ymm5[6],ymm9[7]
        vpmuldq %ymm1, %ymm6, %ymm12
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm6, %ymm8    # ymm8 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm9    # ymm9 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm6, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm8[1],ymm6[2],ymm8[3],ymm6[4],ymm8[5],ymm6[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm9[1],ymm7[2],ymm9[3],ymm7[4],ymm9[5],ymm7[6],ymm9[7]
        vmovdqa %ymm4, 0x20(%rdi)
        vmovdqa %ymm5, 0xa0(%rdi)
        vmovdqa %ymm6, 0x120(%rdi)
        vmovdqa %ymm7, 0x1a0(%rdi)
        vmovdqa 0x40(%rdi), %ymm4
        vmovdqa 0xc0(%rdi), %ymm5
        vmovdqa 0x140(%rdi), %ymm6
        vmovdqa 0x1c0(%rdi), %ymm7
        vmovdqa 0x240(%rdi), %ymm8
        vmovdqa 0x2c0(%rdi), %ymm9
        vmovdqa 0x340(%rdi), %ymm10
        vmovdqa 0x3c0(%rdi), %ymm11
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpbroadcastd 0x80(%rsi), %ymm1
        vpbroadcastd 0x520(%rsi), %ymm2
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm8, 0x240(%rdi)
        vmovdqa %ymm9, 0x2c0(%rdi)
        vmovdqa %ymm10, 0x340(%rdi)
        vmovdqa %ymm11, 0x3c0(%rdi)
        vmovdqa 0x40(%rsi), %ymm1
        vmovdqa 0x60(%rsi), %ymm2
        vpmuldq %ymm1, %ymm4, %ymm12
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm4, %ymm8    # ymm8 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm9    # ymm9 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm4, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm8[1],ymm4[2],ymm8[3],ymm4[4],ymm8[5],ymm4[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm9[1],ymm5[2],ymm9[3],ymm5[4],ymm9[5],ymm5[6],ymm9[7]
        vpmuldq %ymm1, %ymm6, %ymm12
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm6, %ymm8    # ymm8 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm9    # ymm9 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm6, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm8[1],ymm6[2],ymm8[3],ymm6[4],ymm8[5],ymm6[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm9[1],ymm7[2],ymm9[3],ymm7[4],ymm9[5],ymm7[6],ymm9[7]
        vmovdqa %ymm4, 0x40(%rdi)
        vmovdqa %ymm5, 0xc0(%rdi)
        vmovdqa %ymm6, 0x140(%rdi)
        vmovdqa %ymm7, 0x1c0(%rdi)
        vmovdqa 0x60(%rdi), %ymm4
        vmovdqa 0xe0(%rdi), %ymm5
        vmovdqa 0x160(%rdi), %ymm6
        vmovdqa 0x1e0(%rdi), %ymm7
        vmovdqa 0x260(%rdi), %ymm8
        vmovdqa 0x2e0(%rdi), %ymm9
        vmovdqa 0x360(%rdi), %ymm10
        vmovdqa 0x3e0(%rdi), %ymm11
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpsubd %ymm4, %ymm6, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm6   # ymm6 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm6, %ymm6
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm6, %ymm12, %ymm6 # ymm6 = ymm12[0],ymm6[1],ymm12[2],ymm6[3],ymm12[4],ymm6[5],ymm12[6],ymm6[7]
        vpsubd %ymm5, %ymm7, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm7   # ymm7 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm7, %ymm7
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm7, %ymm12, %ymm7 # ymm7 = ymm12[0],ymm7[1],ymm12[2],ymm7[3],ymm12[4],ymm7[5],ymm12[6],ymm7[7]
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpsubd %ymm8, %ymm10, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm9, %ymm11, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vpbroadcastd 0x80(%rsi), %ymm1
        vpbroadcastd 0x520(%rsi), %ymm2
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm8   # ymm8 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm8, %ymm8
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm12, %ymm8 # ymm8 = ymm12[0],ymm8[1],ymm12[2],ymm8[3],ymm12[4],ymm8[5],ymm12[6],ymm8[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm9   # ymm9 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm9, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm9, %ymm9
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm9, %ymm12, %ymm9 # ymm9 = ymm12[0],ymm9[1],ymm12[2],ymm9[3],ymm12[4],ymm9[5],ymm12[6],ymm9[7]
        vpsubd %ymm6, %ymm10, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm10  # ymm10 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm10, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm10, %ymm10
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm10, %ymm12, %ymm10 # ymm10 = ymm12[0],ymm10[1],ymm12[2],ymm10[3],ymm12[4],ymm10[5],ymm12[6],ymm10[7]
        vpsubd %ymm7, %ymm11, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vpmuldq %ymm1, %ymm12, %ymm13
        vmovshdup %ymm12, %ymm11  # ymm11 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm14
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpsubd %ymm13, %ymm12, %ymm12
        vpsubd %ymm14, %ymm11, %ymm11
        vmovshdup %ymm12, %ymm12  # ymm12 = ymm12[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm11, %ymm12, %ymm11 # ymm11 = ymm12[0],ymm11[1],ymm12[2],ymm11[3],ymm12[4],ymm11[5],ymm12[6],ymm11[7]
        vmovdqa %ymm8, 0x260(%rdi)
        vmovdqa %ymm9, 0x2e0(%rdi)
        vmovdqa %ymm10, 0x360(%rdi)
        vmovdqa %ymm11, 0x3e0(%rdi)
        vmovdqa 0x40(%rsi), %ymm1
        vmovdqa 0x60(%rsi), %ymm2
        vpmuldq %ymm1, %ymm4, %ymm12
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm4, %ymm8    # ymm8 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm9    # ymm9 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm4, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm8[1],ymm4[2],ymm8[3],ymm4[4],ymm8[5],ymm4[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm9[1],ymm5[2],ymm9[3],ymm5[4],ymm9[5],ymm5[6],ymm9[7]
        vpmuldq %ymm1, %ymm6, %ymm12
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm6, %ymm8    # ymm8 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm9    # ymm9 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm14
        vpmuldq %ymm1, %ymm9, %ymm15
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm0, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vpmuldq %ymm0, %ymm15, %ymm15
        vpsubd %ymm12, %ymm6, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vpsubd %ymm14, %ymm8, %ymm8
        vpsubd %ymm15, %ymm9, %ymm9
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm8, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm8[1],ymm6[2],ymm8[3],ymm6[4],ymm8[5],ymm6[6],ymm8[7]
        vpblendd $0xaa, %ymm9, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm9[1],ymm7[2],ymm9[3],ymm7[4],ymm9[5],ymm7[6],ymm9[7]
        vmovdqa %ymm4, 0x60(%rdi)
        vmovdqa %ymm5, 0xe0(%rdi)
        vmovdqa %ymm6, 0x160(%rdi)
        vmovdqa %ymm7, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_invntt_avx2_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
