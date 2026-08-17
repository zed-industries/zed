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
 *   dev/x86_64/src/ntt_avx2_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_ntt_avx2_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_ntt_avx2_asm)
S2N_BN_SYMBOL(mldsa_ntt_avx2_asm):

        .cfi_startproc
        vmovdqa (%rsi), %ymm0
        vpbroadcastd 0x84(%rsi), %ymm1
        vpbroadcastd 0x524(%rsi), %ymm2
        vmovdqa (%rdi), %ymm4
        vmovdqa 0x80(%rdi), %ymm5
        vmovdqa 0x100(%rdi), %ymm6
        vmovdqa 0x180(%rdi), %ymm7
        vmovdqa 0x200(%rdi), %ymm8
        vmovdqa 0x280(%rdi), %ymm9
        vmovdqa 0x300(%rdi), %ymm10
        vmovdqa 0x380(%rdi), %ymm11
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm5, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm5, %ymm5
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa %ymm4, (%rdi)
        vmovdqa %ymm5, 0x80(%rdi)
        vmovdqa %ymm6, 0x100(%rdi)
        vmovdqa %ymm7, 0x180(%rdi)
        vmovdqa %ymm8, 0x200(%rdi)
        vmovdqa %ymm9, 0x280(%rdi)
        vmovdqa %ymm10, 0x300(%rdi)
        vmovdqa %ymm11, 0x380(%rdi)
        vpbroadcastd 0x84(%rsi), %ymm1
        vpbroadcastd 0x524(%rsi), %ymm2
        vmovdqa 0x20(%rdi), %ymm4
        vmovdqa 0xa0(%rdi), %ymm5
        vmovdqa 0x120(%rdi), %ymm6
        vmovdqa 0x1a0(%rdi), %ymm7
        vmovdqa 0x220(%rdi), %ymm8
        vmovdqa 0x2a0(%rdi), %ymm9
        vmovdqa 0x320(%rdi), %ymm10
        vmovdqa 0x3a0(%rdi), %ymm11
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm5, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm5, %ymm5
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa %ymm4, 0x20(%rdi)
        vmovdqa %ymm5, 0xa0(%rdi)
        vmovdqa %ymm6, 0x120(%rdi)
        vmovdqa %ymm7, 0x1a0(%rdi)
        vmovdqa %ymm8, 0x220(%rdi)
        vmovdqa %ymm9, 0x2a0(%rdi)
        vmovdqa %ymm10, 0x320(%rdi)
        vmovdqa %ymm11, 0x3a0(%rdi)
        vpbroadcastd 0x84(%rsi), %ymm1
        vpbroadcastd 0x524(%rsi), %ymm2
        vmovdqa 0x40(%rdi), %ymm4
        vmovdqa 0xc0(%rdi), %ymm5
        vmovdqa 0x140(%rdi), %ymm6
        vmovdqa 0x1c0(%rdi), %ymm7
        vmovdqa 0x240(%rdi), %ymm8
        vmovdqa 0x2c0(%rdi), %ymm9
        vmovdqa 0x340(%rdi), %ymm10
        vmovdqa 0x3c0(%rdi), %ymm11
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm5, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm5, %ymm5
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa %ymm4, 0x40(%rdi)
        vmovdqa %ymm5, 0xc0(%rdi)
        vmovdqa %ymm6, 0x140(%rdi)
        vmovdqa %ymm7, 0x1c0(%rdi)
        vmovdqa %ymm8, 0x240(%rdi)
        vmovdqa %ymm9, 0x2c0(%rdi)
        vmovdqa %ymm10, 0x340(%rdi)
        vmovdqa %ymm11, 0x3c0(%rdi)
        vpbroadcastd 0x84(%rsi), %ymm1
        vpbroadcastd 0x524(%rsi), %ymm2
        vmovdqa 0x60(%rdi), %ymm4
        vmovdqa 0xe0(%rdi), %ymm5
        vmovdqa 0x160(%rdi), %ymm6
        vmovdqa 0x1e0(%rdi), %ymm7
        vmovdqa 0x260(%rdi), %ymm8
        vmovdqa 0x2e0(%rdi), %ymm9
        vmovdqa 0x360(%rdi), %ymm10
        vmovdqa 0x3e0(%rdi), %ymm11
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vpbroadcastd 0x88(%rsi), %ymm1
        vpbroadcastd 0x528(%rsi), %ymm2
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm5, %ymm12
        vpaddd %ymm7, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm5, %ymm5
        vpbroadcastd 0x8c(%rsi), %ymm1
        vpbroadcastd 0x52c(%rsi), %ymm2
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa %ymm4, 0x60(%rdi)
        vmovdqa %ymm5, 0xe0(%rdi)
        vmovdqa %ymm6, 0x160(%rdi)
        vmovdqa %ymm7, 0x1e0(%rdi)
        vmovdqa %ymm8, 0x260(%rdi)
        vmovdqa %ymm9, 0x2e0(%rdi)
        vmovdqa %ymm10, 0x360(%rdi)
        vmovdqa %ymm11, 0x3e0(%rdi)
        vmovdqa (%rdi), %ymm4
        vmovdqa 0x20(%rdi), %ymm5
        vmovdqa 0x40(%rdi), %ymm6
        vmovdqa 0x60(%rdi), %ymm7
        vmovdqa 0x80(%rdi), %ymm8
        vmovdqa 0xa0(%rdi), %ymm9
        vmovdqa 0xc0(%rdi), %ymm10
        vmovdqa 0xe0(%rdi), %ymm11
        vpbroadcastd 0x90(%rsi), %ymm1
        vpbroadcastd 0x530(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm5, %ymm9 # ymm9 = ymm5[2,3],ymm9[2,3]
        vperm2i128 $0x20, %ymm10, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm6, %ymm10 # ymm10 = ymm6[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm6 # ymm6 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vmovdqa 0xa0(%rsi), %ymm1
        vmovdqa 0x540(%rsi), %ymm2
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm3, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vpunpcklqdq %ymm5, %ymm3, %ymm7 # ymm7 = ymm3[0],ymm5[0],ymm3[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[1],ymm5[1],ymm3[3],ymm5[3]
        vpunpcklqdq %ymm10, %ymm8, %ymm3 # ymm3 = ymm8[0],ymm10[0],ymm8[2],ymm10[2]
        vpunpckhqdq %ymm10, %ymm8, %ymm10 # ymm10 = ymm8[1],ymm10[1],ymm8[3],ymm10[3]
        vpunpcklqdq %ymm6, %ymm4, %ymm8 # ymm8 = ymm4[0],ymm6[0],ymm4[2],ymm6[2]
        vpunpckhqdq %ymm6, %ymm4, %ymm6 # ymm6 = ymm4[1],ymm6[1],ymm4[3],ymm6[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm4 # ymm4 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vmovdqa 0x120(%rsi), %ymm1
        vmovdqa 0x5c0(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm7, %ymm12
        vpaddd %ymm7, %ymm8, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm5, %ymm12
        vpaddd %ymm6, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm3, %ymm12
        vpaddd %ymm4, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm10, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm10, %ymm10
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
        vmovdqa 0x1a0(%rsi), %ymm1
        vmovdqa 0x640(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm7, %ymm12
        vpaddd %ymm3, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm6, %ymm12
        vpaddd %ymm6, %ymm11, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm6, %ymm6
        vmovdqa 0x220(%rsi), %ymm1
        vmovdqa 0x6c0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm9, %ymm12
        vpaddd %ymm7, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm8, %ymm8
        vmovdqa 0x2a0(%rsi), %ymm1
        vmovdqa 0x740(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm3, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm4, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm4, %ymm4
        vmovdqa 0x320(%rsi), %ymm1
        vmovdqa 0x7c0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm8, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa 0x3a0(%rsi), %ymm1
        vmovdqa 0x840(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm6, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vmovdqa 0x420(%rsi), %ymm1
        vmovdqa 0x8c0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm4, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vmovdqa 0x4a0(%rsi), %ymm1
        vmovdqa 0x940(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm3, %ymm12
        vpaddd %ymm3, %ymm11, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm3, %ymm3
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
        vpbroadcastd 0x94(%rsi), %ymm1
        vpbroadcastd 0x534(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm5, %ymm9 # ymm9 = ymm5[2,3],ymm9[2,3]
        vperm2i128 $0x20, %ymm10, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm6, %ymm10 # ymm10 = ymm6[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm6 # ymm6 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vmovdqa 0xc0(%rsi), %ymm1
        vmovdqa 0x560(%rsi), %ymm2
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm3, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vpunpcklqdq %ymm5, %ymm3, %ymm7 # ymm7 = ymm3[0],ymm5[0],ymm3[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[1],ymm5[1],ymm3[3],ymm5[3]
        vpunpcklqdq %ymm10, %ymm8, %ymm3 # ymm3 = ymm8[0],ymm10[0],ymm8[2],ymm10[2]
        vpunpckhqdq %ymm10, %ymm8, %ymm10 # ymm10 = ymm8[1],ymm10[1],ymm8[3],ymm10[3]
        vpunpcklqdq %ymm6, %ymm4, %ymm8 # ymm8 = ymm4[0],ymm6[0],ymm4[2],ymm6[2]
        vpunpckhqdq %ymm6, %ymm4, %ymm6 # ymm6 = ymm4[1],ymm6[1],ymm4[3],ymm6[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm4 # ymm4 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vmovdqa 0x140(%rsi), %ymm1
        vmovdqa 0x5e0(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm7, %ymm12
        vpaddd %ymm7, %ymm8, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm5, %ymm12
        vpaddd %ymm6, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm3, %ymm12
        vpaddd %ymm4, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm10, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm10, %ymm10
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
        vmovdqa 0x1c0(%rsi), %ymm1
        vmovdqa 0x660(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm7, %ymm12
        vpaddd %ymm3, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm6, %ymm12
        vpaddd %ymm6, %ymm11, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm6, %ymm6
        vmovdqa 0x240(%rsi), %ymm1
        vmovdqa 0x6e0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm9, %ymm12
        vpaddd %ymm7, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm8, %ymm8
        vmovdqa 0x2c0(%rsi), %ymm1
        vmovdqa 0x760(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm3, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm4, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm4, %ymm4
        vmovdqa 0x340(%rsi), %ymm1
        vmovdqa 0x7e0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm8, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa 0x3c0(%rsi), %ymm1
        vmovdqa 0x860(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm6, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vmovdqa 0x440(%rsi), %ymm1
        vmovdqa 0x8e0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm4, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vmovdqa 0x4c0(%rsi), %ymm1
        vmovdqa 0x960(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm3, %ymm12
        vpaddd %ymm3, %ymm11, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm3, %ymm3
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
        vpbroadcastd 0x98(%rsi), %ymm1
        vpbroadcastd 0x538(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm5, %ymm9 # ymm9 = ymm5[2,3],ymm9[2,3]
        vperm2i128 $0x20, %ymm10, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm6, %ymm10 # ymm10 = ymm6[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm6 # ymm6 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vmovdqa 0xe0(%rsi), %ymm1
        vmovdqa 0x580(%rsi), %ymm2
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm3, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vpunpcklqdq %ymm5, %ymm3, %ymm7 # ymm7 = ymm3[0],ymm5[0],ymm3[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[1],ymm5[1],ymm3[3],ymm5[3]
        vpunpcklqdq %ymm10, %ymm8, %ymm3 # ymm3 = ymm8[0],ymm10[0],ymm8[2],ymm10[2]
        vpunpckhqdq %ymm10, %ymm8, %ymm10 # ymm10 = ymm8[1],ymm10[1],ymm8[3],ymm10[3]
        vpunpcklqdq %ymm6, %ymm4, %ymm8 # ymm8 = ymm4[0],ymm6[0],ymm4[2],ymm6[2]
        vpunpckhqdq %ymm6, %ymm4, %ymm6 # ymm6 = ymm4[1],ymm6[1],ymm4[3],ymm6[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm4 # ymm4 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vmovdqa 0x160(%rsi), %ymm1
        vmovdqa 0x600(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm7, %ymm12
        vpaddd %ymm7, %ymm8, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm5, %ymm12
        vpaddd %ymm6, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm3, %ymm12
        vpaddd %ymm4, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm10, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm10, %ymm10
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
        vmovdqa 0x1e0(%rsi), %ymm1
        vmovdqa 0x680(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm7, %ymm12
        vpaddd %ymm3, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm6, %ymm12
        vpaddd %ymm6, %ymm11, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm6, %ymm6
        vmovdqa 0x260(%rsi), %ymm1
        vmovdqa 0x700(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm9, %ymm12
        vpaddd %ymm7, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm8, %ymm8
        vmovdqa 0x2e0(%rsi), %ymm1
        vmovdqa 0x780(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm3, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm4, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm4, %ymm4
        vmovdqa 0x360(%rsi), %ymm1
        vmovdqa 0x800(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm8, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa 0x3e0(%rsi), %ymm1
        vmovdqa 0x880(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm6, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vmovdqa 0x460(%rsi), %ymm1
        vmovdqa 0x900(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm4, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vmovdqa 0x4e0(%rsi), %ymm1
        vmovdqa 0x980(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm3, %ymm12
        vpaddd %ymm3, %ymm11, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm3, %ymm3
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
        vpbroadcastd 0x9c(%rsi), %ymm1
        vpbroadcastd 0x53c(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm4, %ymm12
        vpaddd %ymm4, %ymm8, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm9, %ymm13
        vmovshdup %ymm9, %ymm12   # ymm12 = ymm9[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm9, %ymm9
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm9, %ymm9    # ymm9 = ymm9[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm9, %ymm9 # ymm9 = ymm9[0],ymm12[1],ymm9[2],ymm12[3],ymm9[4],ymm12[5],ymm9[6],ymm12[7]
        vpsubd %ymm9, %ymm5, %ymm12
        vpaddd %ymm5, %ymm9, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm9
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm6, %ymm12
        vpaddd %ymm6, %ymm10, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm6, %ymm6
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm7, %ymm12
        vpaddd %ymm7, %ymm11, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm7, %ymm7
        vperm2i128 $0x20, %ymm8, %ymm4, %ymm3 # ymm3 = ymm4[0,1],ymm8[0,1]
        vperm2i128 $0x31, %ymm8, %ymm4, %ymm8 # ymm8 = ymm4[2,3],ymm8[2,3]
        vperm2i128 $0x20, %ymm9, %ymm5, %ymm4 # ymm4 = ymm5[0,1],ymm9[0,1]
        vperm2i128 $0x31, %ymm9, %ymm5, %ymm9 # ymm9 = ymm5[2,3],ymm9[2,3]
        vperm2i128 $0x20, %ymm10, %ymm6, %ymm5 # ymm5 = ymm6[0,1],ymm10[0,1]
        vperm2i128 $0x31, %ymm10, %ymm6, %ymm10 # ymm10 = ymm6[2,3],ymm10[2,3]
        vperm2i128 $0x20, %ymm11, %ymm7, %ymm6 # ymm6 = ymm7[0,1],ymm11[0,1]
        vperm2i128 $0x31, %ymm11, %ymm7, %ymm11 # ymm11 = ymm7[2,3],ymm11[2,3]
        vmovdqa 0x100(%rsi), %ymm1
        vmovdqa 0x5a0(%rsi), %ymm2
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm3, %ymm12
        vpaddd %ymm5, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm10, %ymm13
        vmovshdup %ymm10, %ymm12  # ymm12 = ymm10[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm10, %ymm10
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm10, %ymm10  # ymm10 = ymm10[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm10, %ymm10 # ymm10 = ymm10[0],ymm12[1],ymm10[2],ymm12[3],ymm10[4],ymm12[5],ymm10[6],ymm12[7]
        vpsubd %ymm10, %ymm8, %ymm12
        vpaddd %ymm10, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm10
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm4, %ymm12
        vpaddd %ymm6, %ymm4, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm4, %ymm4
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm9, %ymm12
        vpaddd %ymm11, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm9, %ymm9
        vpunpcklqdq %ymm5, %ymm3, %ymm7 # ymm7 = ymm3[0],ymm5[0],ymm3[2],ymm5[2]
        vpunpckhqdq %ymm5, %ymm3, %ymm5 # ymm5 = ymm3[1],ymm5[1],ymm3[3],ymm5[3]
        vpunpcklqdq %ymm10, %ymm8, %ymm3 # ymm3 = ymm8[0],ymm10[0],ymm8[2],ymm10[2]
        vpunpckhqdq %ymm10, %ymm8, %ymm10 # ymm10 = ymm8[1],ymm10[1],ymm8[3],ymm10[3]
        vpunpcklqdq %ymm6, %ymm4, %ymm8 # ymm8 = ymm4[0],ymm6[0],ymm4[2],ymm6[2]
        vpunpckhqdq %ymm6, %ymm4, %ymm6 # ymm6 = ymm4[1],ymm6[1],ymm4[3],ymm6[3]
        vpunpcklqdq %ymm11, %ymm9, %ymm4 # ymm4 = ymm9[0],ymm11[0],ymm9[2],ymm11[2]
        vpunpckhqdq %ymm11, %ymm9, %ymm11 # ymm11 = ymm9[1],ymm11[1],ymm9[3],ymm11[3]
        vmovdqa 0x180(%rsi), %ymm1
        vmovdqa 0x620(%rsi), %ymm2
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm7, %ymm12
        vpaddd %ymm7, %ymm8, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm5, %ymm12
        vpaddd %ymm6, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm3, %ymm12
        vpaddd %ymm4, %ymm3, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm3, %ymm3
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm2, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm10, %ymm12
        vpaddd %ymm11, %ymm10, %ymm10
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm10, %ymm10
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
        vmovdqa 0x200(%rsi), %ymm1
        vmovdqa 0x6a0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm5, %ymm13
        vmovshdup %ymm5, %ymm12   # ymm12 = ymm5[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm5, %ymm5
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm5, %ymm5    # ymm5 = ymm5[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm5, %ymm5 # ymm5 = ymm5[0],ymm12[1],ymm5[2],ymm12[3],ymm5[4],ymm12[5],ymm5[6],ymm12[7]
        vpsubd %ymm5, %ymm9, %ymm12
        vpaddd %ymm5, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm5
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm8, %ymm12
        vpaddd %ymm4, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm8, %ymm8
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm7, %ymm12
        vpaddd %ymm3, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm7, %ymm7
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm6, %ymm12
        vpaddd %ymm6, %ymm11, %ymm6
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm6, %ymm6
        vmovdqa 0x280(%rsi), %ymm1
        vmovdqa 0x720(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm7, %ymm13
        vmovshdup %ymm7, %ymm12   # ymm12 = ymm7[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm7, %ymm7
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm7, %ymm7    # ymm7 = ymm7[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm7, %ymm7 # ymm7 = ymm7[0],ymm12[1],ymm7[2],ymm12[3],ymm7[4],ymm12[5],ymm7[6],ymm12[7]
        vpsubd %ymm7, %ymm9, %ymm12
        vpaddd %ymm7, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm7
        vpsubd %ymm13, %ymm9, %ymm9
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm8, %ymm12
        vpaddd %ymm6, %ymm8, %ymm8
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm8, %ymm8
        vmovdqa 0x300(%rsi), %ymm1
        vmovdqa 0x7a0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm3, %ymm13
        vmovshdup %ymm3, %ymm12   # ymm12 = ymm3[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm3, %ymm3
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm3, %ymm3    # ymm3 = ymm3[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm3, %ymm3 # ymm3 = ymm3[0],ymm12[1],ymm3[2],ymm12[3],ymm3[4],ymm12[5],ymm3[6],ymm12[7]
        vpsubd %ymm3, %ymm5, %ymm12
        vpaddd %ymm3, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm3
        vpsubd %ymm13, %ymm5, %ymm5
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm4, %ymm12
        vpaddd %ymm4, %ymm11, %ymm4
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm4, %ymm4
        vmovdqa 0x380(%rsi), %ymm1
        vmovdqa 0x820(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm8, %ymm13
        vmovshdup %ymm8, %ymm12   # ymm12 = ymm8[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm8, %ymm8
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm8, %ymm8    # ymm8 = ymm8[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm8, %ymm8 # ymm8 = ymm8[0],ymm12[1],ymm8[2],ymm12[3],ymm8[4],ymm12[5],ymm8[6],ymm12[7]
        vpsubd %ymm8, %ymm9, %ymm12
        vpaddd %ymm8, %ymm9, %ymm9
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm8
        vpsubd %ymm13, %ymm9, %ymm9
        vmovdqa 0x400(%rsi), %ymm1
        vmovdqa 0x8a0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm6, %ymm13
        vmovshdup %ymm6, %ymm12   # ymm12 = ymm6[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm6, %ymm6
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm6, %ymm6    # ymm6 = ymm6[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm6, %ymm6 # ymm6 = ymm6[0],ymm12[1],ymm6[2],ymm12[3],ymm6[4],ymm12[5],ymm6[6],ymm12[7]
        vpsubd %ymm6, %ymm7, %ymm12
        vpaddd %ymm6, %ymm7, %ymm7
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm6
        vpsubd %ymm13, %ymm7, %ymm7
        vmovdqa 0x480(%rsi), %ymm1
        vmovdqa 0x920(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm4, %ymm13
        vmovshdup %ymm4, %ymm12   # ymm12 = ymm4[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm4, %ymm4
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm12[1],ymm4[2],ymm12[3],ymm4[4],ymm12[5],ymm4[6],ymm12[7]
        vpsubd %ymm4, %ymm5, %ymm12
        vpaddd %ymm4, %ymm5, %ymm5
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm4
        vpsubd %ymm13, %ymm5, %ymm5
        vmovdqa 0x500(%rsi), %ymm1
        vmovdqa 0x9a0(%rsi), %ymm2
        vpsrlq $0x20, %ymm1, %ymm10
        vmovshdup %ymm2, %ymm15   # ymm15 = ymm2[1,1,3,3,5,5,7,7]
        vpmuldq %ymm1, %ymm11, %ymm13
        vmovshdup %ymm11, %ymm12  # ymm12 = ymm11[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm12, %ymm14
        vpmuldq %ymm2, %ymm11, %ymm11
        vpmuldq %ymm15, %ymm12, %ymm12
        vpmuldq %ymm0, %ymm13, %ymm13
        vpmuldq %ymm0, %ymm14, %ymm14
        vmovshdup %ymm11, %ymm11  # ymm11 = ymm11[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm12, %ymm11, %ymm11 # ymm11 = ymm11[0],ymm12[1],ymm11[2],ymm12[3],ymm11[4],ymm12[5],ymm11[6],ymm12[7]
        vpsubd %ymm11, %ymm3, %ymm12
        vpaddd %ymm3, %ymm11, %ymm3
        vmovshdup %ymm13, %ymm13  # ymm13 = ymm13[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm14, %ymm13, %ymm13 # ymm13 = ymm13[0],ymm14[1],ymm13[2],ymm14[3],ymm13[4],ymm14[5],ymm13[6],ymm14[7]
        vpaddd %ymm13, %ymm12, %ymm11
        vpsubd %ymm13, %ymm3, %ymm3
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

S2N_BN_SIZE_DIRECTIVE(mldsa_ntt_avx2_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
