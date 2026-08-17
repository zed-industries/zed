/*
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
 *   dev/x86_64/src/pointwise_acc_l4_avx2_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_pointwise_acc_l4_avx2_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_pointwise_acc_l4_avx2_asm)
S2N_BN_SYMBOL(mldsa_pointwise_acc_l4_avx2_asm):

        .cfi_startproc
        vmovdqa 0x20(%rcx), %ymm0
        vmovdqa (%rcx), %ymm1
        xorl %eax, %eax

Lmldsa_pointwise_acc_l4_avx2_looptop2:
        vmovdqa (%rsi), %ymm6
        vmovdqa 0x20(%rsi), %ymm8
        vmovdqa (%rdx), %ymm10
        vmovdqa 0x20(%rdx), %ymm12
        vpsrlq $0x20, %ymm6, %ymm7
        vpsrlq $0x20, %ymm8, %ymm9
        vmovshdup %ymm10, %ymm11  # ymm11 = ymm10[1,1,3,3,5,5,7,7]
        vmovshdup %ymm12, %ymm13  # ymm13 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm6, %ymm6
        vpmuldq %ymm11, %ymm7, %ymm7
        vpmuldq %ymm12, %ymm8, %ymm8
        vpmuldq %ymm13, %ymm9, %ymm9
        vmovdqa %ymm6, %ymm2
        vmovdqa %ymm7, %ymm3
        vmovdqa %ymm8, %ymm4
        vmovdqa %ymm9, %ymm5
        vmovdqa 0x400(%rsi), %ymm6
        vmovdqa 0x420(%rsi), %ymm8
        vmovdqa 0x400(%rdx), %ymm10
        vmovdqa 0x420(%rdx), %ymm12
        vpsrlq $0x20, %ymm6, %ymm7
        vpsrlq $0x20, %ymm8, %ymm9
        vmovshdup %ymm10, %ymm11  # ymm11 = ymm10[1,1,3,3,5,5,7,7]
        vmovshdup %ymm12, %ymm13  # ymm13 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm6, %ymm6
        vpmuldq %ymm11, %ymm7, %ymm7
        vpmuldq %ymm12, %ymm8, %ymm8
        vpmuldq %ymm13, %ymm9, %ymm9
        vpaddq %ymm2, %ymm6, %ymm2
        vpaddq %ymm3, %ymm7, %ymm3
        vpaddq %ymm4, %ymm8, %ymm4
        vpaddq %ymm5, %ymm9, %ymm5
        vmovdqa 0x800(%rsi), %ymm6
        vmovdqa 0x820(%rsi), %ymm8
        vmovdqa 0x800(%rdx), %ymm10
        vmovdqa 0x820(%rdx), %ymm12
        vpsrlq $0x20, %ymm6, %ymm7
        vpsrlq $0x20, %ymm8, %ymm9
        vmovshdup %ymm10, %ymm11  # ymm11 = ymm10[1,1,3,3,5,5,7,7]
        vmovshdup %ymm12, %ymm13  # ymm13 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm6, %ymm6
        vpmuldq %ymm11, %ymm7, %ymm7
        vpmuldq %ymm12, %ymm8, %ymm8
        vpmuldq %ymm13, %ymm9, %ymm9
        vpaddq %ymm2, %ymm6, %ymm2
        vpaddq %ymm3, %ymm7, %ymm3
        vpaddq %ymm4, %ymm8, %ymm4
        vpaddq %ymm5, %ymm9, %ymm5
        vmovdqa 0xc00(%rsi), %ymm6
        vmovdqa 0xc20(%rsi), %ymm8
        vmovdqa 0xc00(%rdx), %ymm10
        vmovdqa 0xc20(%rdx), %ymm12
        vpsrlq $0x20, %ymm6, %ymm7
        vpsrlq $0x20, %ymm8, %ymm9
        vmovshdup %ymm10, %ymm11  # ymm11 = ymm10[1,1,3,3,5,5,7,7]
        vmovshdup %ymm12, %ymm13  # ymm13 = ymm12[1,1,3,3,5,5,7,7]
        vpmuldq %ymm10, %ymm6, %ymm6
        vpmuldq %ymm11, %ymm7, %ymm7
        vpmuldq %ymm12, %ymm8, %ymm8
        vpmuldq %ymm13, %ymm9, %ymm9
        vpaddq %ymm2, %ymm6, %ymm2
        vpaddq %ymm3, %ymm7, %ymm3
        vpaddq %ymm4, %ymm8, %ymm4
        vpaddq %ymm5, %ymm9, %ymm5
        vpmuldq %ymm2, %ymm0, %ymm6
        vpmuldq %ymm3, %ymm0, %ymm7
        vpmuldq %ymm4, %ymm0, %ymm8
        vpmuldq %ymm5, %ymm0, %ymm9
        vpmuldq %ymm6, %ymm1, %ymm6
        vpmuldq %ymm7, %ymm1, %ymm7
        vpmuldq %ymm8, %ymm1, %ymm8
        vpmuldq %ymm9, %ymm1, %ymm9
        vpsubq %ymm6, %ymm2, %ymm2
        vpsubq %ymm7, %ymm3, %ymm3
        vpsubq %ymm8, %ymm4, %ymm4
        vpsubq %ymm9, %ymm5, %ymm5
        vpsrlq $0x20, %ymm2, %ymm2
        vmovshdup %ymm4, %ymm4    # ymm4 = ymm4[1,1,3,3,5,5,7,7]
        vpblendd $0xaa, %ymm3, %ymm2, %ymm2 # ymm2 = ymm2[0],ymm3[1],ymm2[2],ymm3[3],ymm2[4],ymm3[5],ymm2[6],ymm3[7]
        vpblendd $0xaa, %ymm5, %ymm4, %ymm4 # ymm4 = ymm4[0],ymm5[1],ymm4[2],ymm5[3],ymm4[4],ymm5[5],ymm4[6],ymm5[7]
        vmovdqa %ymm2, (%rdi)
        vmovdqa %ymm4, 0x20(%rdi)
        addq $0x40, %rsi
        addq $0x40, %rdx
        addq $0x40, %rdi
        addl $0x1, %eax
        cmpl $0x10, %eax
        jb Lmldsa_pointwise_acc_l4_avx2_looptop2
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_pointwise_acc_l4_avx2_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
