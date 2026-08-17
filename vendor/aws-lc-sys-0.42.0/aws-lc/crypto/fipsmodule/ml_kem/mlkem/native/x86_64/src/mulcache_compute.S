/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/mulcache_compute.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_mulcache_compute_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_mulcache_compute_avx2)
S2N_BN_SYMBOL(mlkem_poly_mulcache_compute_avx2):

        .cfi_startproc
        movl $0xd010d01, %eax        # imm = 0xD010D01
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        vmovdqa 0x20(%rsi), %ymm2
        vmovdqa 0x60(%rsi), %ymm3
        vmovdqa 0x3e0(%rdx), %ymm4
        vmovdqa 0x460(%rdx), %ymm1
        vpmullw %ymm2, %ymm1, %ymm5
        vpmullw %ymm3, %ymm1, %ymm6
        vpmulhw %ymm2, %ymm4, %ymm7
        vpmulhw %ymm3, %ymm4, %ymm8
        vpmulhw %ymm5, %ymm0, %ymm9
        vpmulhw %ymm6, %ymm0, %ymm10
        vpsubw %ymm9, %ymm7, %ymm7
        vpsubw %ymm10, %ymm8, %ymm8
        vmovdqa %ymm7, (%rdi)
        vmovdqa %ymm8, 0x20(%rdi)
        vmovdqa 0xa0(%rsi), %ymm2
        vmovdqa 0xe0(%rsi), %ymm3
        vmovdqa 0x400(%rdx), %ymm4
        vmovdqa 0x480(%rdx), %ymm1
        vpmullw %ymm2, %ymm1, %ymm5
        vpmullw %ymm3, %ymm1, %ymm6
        vpmulhw %ymm2, %ymm4, %ymm7
        vpmulhw %ymm3, %ymm4, %ymm8
        vpmulhw %ymm5, %ymm0, %ymm9
        vpmulhw %ymm6, %ymm0, %ymm10
        vpsubw %ymm9, %ymm7, %ymm7
        vpsubw %ymm10, %ymm8, %ymm8
        vmovdqa %ymm7, 0x40(%rdi)
        vmovdqa %ymm8, 0x60(%rdi)
        vmovdqa 0x120(%rsi), %ymm2
        vmovdqa 0x160(%rsi), %ymm3
        vmovdqa 0x420(%rdx), %ymm4
        vmovdqa 0x4a0(%rdx), %ymm1
        vpmullw %ymm2, %ymm1, %ymm5
        vpmullw %ymm3, %ymm1, %ymm6
        vpmulhw %ymm2, %ymm4, %ymm7
        vpmulhw %ymm3, %ymm4, %ymm8
        vpmulhw %ymm5, %ymm0, %ymm9
        vpmulhw %ymm6, %ymm0, %ymm10
        vpsubw %ymm9, %ymm7, %ymm7
        vpsubw %ymm10, %ymm8, %ymm8
        vmovdqa %ymm7, 0x80(%rdi)
        vmovdqa %ymm8, 0xa0(%rdi)
        vmovdqa 0x1a0(%rsi), %ymm2
        vmovdqa 0x1e0(%rsi), %ymm3
        vmovdqa 0x440(%rdx), %ymm4
        vmovdqa 0x4c0(%rdx), %ymm1
        vpmullw %ymm2, %ymm1, %ymm5
        vpmullw %ymm3, %ymm1, %ymm6
        vpmulhw %ymm2, %ymm4, %ymm7
        vpmulhw %ymm3, %ymm4, %ymm8
        vpmulhw %ymm5, %ymm0, %ymm9
        vpmulhw %ymm6, %ymm0, %ymm10
        vpsubw %ymm9, %ymm7, %ymm7
        vpsubw %ymm10, %ymm8, %ymm8
        vmovdqa %ymm7, 0xc0(%rdi)
        vmovdqa %ymm8, 0xe0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_mulcache_compute_avx2)

