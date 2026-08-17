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


/*************************************************
 * Name:        mld_poly_caddq_avx2_asm
 *
 * Description: For all coefficients of in/out polynomial add Q if
 *              coefficient is negative.
 *
 * Arguments:   - int32_t *r: pointer to input/output polynomial
 **************************************************/

#include "_internal_s2n_bignum_x86_att.h"



/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/x86_64/src/poly_caddq_avx2_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_poly_caddq_avx2_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_poly_caddq_avx2_asm)
S2N_BN_SYMBOL(mldsa_poly_caddq_avx2_asm):

        .cfi_startproc
        vpxor %xmm2, %xmm2, %xmm2
        movl $0x7fe001, %eax         # imm = 0x7FE001
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        vpcmpgtd (%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd (%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, (%rdi)
        vpcmpgtd 0x20(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0x20(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0x20(%rdi)
        vpcmpgtd 0x40(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0x40(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0x40(%rdi)
        vpcmpgtd 0x60(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0x60(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0x60(%rdi)
        vpcmpgtd 0x80(%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd 0x80(%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, 0x80(%rdi)
        vpcmpgtd 0xa0(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0xa0(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0xa0(%rdi)
        vpcmpgtd 0xc0(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0xc0(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0xc0(%rdi)
        vpcmpgtd 0xe0(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0xe0(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0xe0(%rdi)
        vpcmpgtd 0x100(%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd 0x100(%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, 0x100(%rdi)
        vpcmpgtd 0x120(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0x120(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0x120(%rdi)
        vpcmpgtd 0x140(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0x140(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0x140(%rdi)
        vpcmpgtd 0x160(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0x160(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0x160(%rdi)
        vpcmpgtd 0x180(%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd 0x180(%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, 0x180(%rdi)
        vpcmpgtd 0x1a0(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0x1a0(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0x1a0(%rdi)
        vpcmpgtd 0x1c0(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0x1c0(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0x1c0(%rdi)
        vpcmpgtd 0x1e0(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0x1e0(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0x1e0(%rdi)
        vpcmpgtd 0x200(%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd 0x200(%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, 0x200(%rdi)
        vpcmpgtd 0x220(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0x220(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0x220(%rdi)
        vpcmpgtd 0x240(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0x240(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0x240(%rdi)
        vpcmpgtd 0x260(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0x260(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0x260(%rdi)
        vpcmpgtd 0x280(%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd 0x280(%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, 0x280(%rdi)
        vpcmpgtd 0x2a0(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0x2a0(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0x2a0(%rdi)
        vpcmpgtd 0x2c0(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0x2c0(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0x2c0(%rdi)
        vpcmpgtd 0x2e0(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0x2e0(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0x2e0(%rdi)
        vpcmpgtd 0x300(%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd 0x300(%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, 0x300(%rdi)
        vpcmpgtd 0x320(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0x320(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0x320(%rdi)
        vpcmpgtd 0x340(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0x340(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0x340(%rdi)
        vpcmpgtd 0x360(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0x360(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0x360(%rdi)
        vpcmpgtd 0x380(%rdi), %ymm2, %ymm0
        vpand %ymm1, %ymm0, %ymm0
        vpaddd 0x380(%rdi), %ymm0, %ymm0
        vmovdqa %ymm0, 0x380(%rdi)
        vpcmpgtd 0x3a0(%rdi), %ymm2, %ymm3
        vpand %ymm1, %ymm3, %ymm3
        vpaddd 0x3a0(%rdi), %ymm3, %ymm3
        vmovdqa %ymm3, 0x3a0(%rdi)
        vpcmpgtd 0x3c0(%rdi), %ymm2, %ymm4
        vpand %ymm1, %ymm4, %ymm4
        vpaddd 0x3c0(%rdi), %ymm4, %ymm4
        vmovdqa %ymm4, 0x3c0(%rdi)
        vpcmpgtd 0x3e0(%rdi), %ymm2, %ymm5
        vpand %ymm1, %ymm5, %ymm5
        vpaddd 0x3e0(%rdi), %ymm5, %ymm5
        vmovdqa %ymm5, 0x3e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_poly_caddq_avx2_asm)



#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
