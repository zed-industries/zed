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
 * Implementation from Kyber reference repository @[REF_AVX2]
 *
 * Changes:
 * - Add call to csub in reduce128_avx to produce outputs
 *   in [0,1,...,q-1] rather than [0,1,...,q], matching the
 *   semantics of mlk_poly_reduce(),
 * - Use a macro instead of a local function call.
 */

#include "_internal_s2n_bignum_x86_att.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/x86_64/src/tomont.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_tomont_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_tomont_avx2)
S2N_BN_SYMBOL(mlkem_tomont_avx2):

        .cfi_startproc
        movl $0xd010d01, %eax        # imm = 0xD010D01
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        movl $0x50495049, %eax       # imm = 0x50495049
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        movl $0x5490549, %eax        # imm = 0x5490549
        vmovd %eax, %xmm2
        vpbroadcastd %xmm2, %ymm2
        vmovdqa (%rdi), %ymm3
        vmovdqa 0x20(%rdi), %ymm4
        vmovdqa 0x40(%rdi), %ymm5
        vmovdqa 0x60(%rdi), %ymm6
        vmovdqa 0x80(%rdi), %ymm7
        vmovdqa 0xa0(%rdi), %ymm8
        vmovdqa 0xc0(%rdi), %ymm9
        vmovdqa 0xe0(%rdi), %ymm10
        vpmullw %ymm1, %ymm3, %ymm11
        vpmulhw %ymm2, %ymm3, %ymm3
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm11, %ymm3, %ymm3
        vpmullw %ymm1, %ymm4, %ymm12
        vpmulhw %ymm2, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm4, %ymm4
        vpmullw %ymm1, %ymm5, %ymm13
        vpmulhw %ymm2, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm13, %ymm13
        vpsubw %ymm13, %ymm5, %ymm5
        vpmullw %ymm1, %ymm6, %ymm14
        vpmulhw %ymm2, %ymm6, %ymm6
        vpmulhw %ymm0, %ymm14, %ymm14
        vpsubw %ymm14, %ymm6, %ymm6
        vpmullw %ymm1, %ymm7, %ymm15
        vpmulhw %ymm2, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm15, %ymm15
        vpsubw %ymm15, %ymm7, %ymm7
        vpmullw %ymm1, %ymm8, %ymm11
        vpmulhw %ymm2, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm11, %ymm8, %ymm8
        vpmullw %ymm1, %ymm9, %ymm12
        vpmulhw %ymm2, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vpmullw %ymm1, %ymm10, %ymm13
        vpmulhw %ymm2, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm13, %ymm13
        vpsubw %ymm13, %ymm10, %ymm10
        vmovdqa %ymm3, (%rdi)
        vmovdqa %ymm4, 0x20(%rdi)
        vmovdqa %ymm5, 0x40(%rdi)
        vmovdqa %ymm6, 0x60(%rdi)
        vmovdqa %ymm7, 0x80(%rdi)
        vmovdqa %ymm8, 0xa0(%rdi)
        vmovdqa %ymm9, 0xc0(%rdi)
        vmovdqa %ymm10, 0xe0(%rdi)
        vmovdqa 0x100(%rdi), %ymm3
        vmovdqa 0x120(%rdi), %ymm4
        vmovdqa 0x140(%rdi), %ymm5
        vmovdqa 0x160(%rdi), %ymm6
        vmovdqa 0x180(%rdi), %ymm7
        vmovdqa 0x1a0(%rdi), %ymm8
        vmovdqa 0x1c0(%rdi), %ymm9
        vmovdqa 0x1e0(%rdi), %ymm10
        vpmullw %ymm1, %ymm3, %ymm11
        vpmulhw %ymm2, %ymm3, %ymm3
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm11, %ymm3, %ymm3
        vpmullw %ymm1, %ymm4, %ymm12
        vpmulhw %ymm2, %ymm4, %ymm4
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm4, %ymm4
        vpmullw %ymm1, %ymm5, %ymm13
        vpmulhw %ymm2, %ymm5, %ymm5
        vpmulhw %ymm0, %ymm13, %ymm13
        vpsubw %ymm13, %ymm5, %ymm5
        vpmullw %ymm1, %ymm6, %ymm14
        vpmulhw %ymm2, %ymm6, %ymm6
        vpmulhw %ymm0, %ymm14, %ymm14
        vpsubw %ymm14, %ymm6, %ymm6
        vpmullw %ymm1, %ymm7, %ymm15
        vpmulhw %ymm2, %ymm7, %ymm7
        vpmulhw %ymm0, %ymm15, %ymm15
        vpsubw %ymm15, %ymm7, %ymm7
        vpmullw %ymm1, %ymm8, %ymm11
        vpmulhw %ymm2, %ymm8, %ymm8
        vpmulhw %ymm0, %ymm11, %ymm11
        vpsubw %ymm11, %ymm8, %ymm8
        vpmullw %ymm1, %ymm9, %ymm12
        vpmulhw %ymm2, %ymm9, %ymm9
        vpmulhw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vpmullw %ymm1, %ymm10, %ymm13
        vpmulhw %ymm2, %ymm10, %ymm10
        vpmulhw %ymm0, %ymm13, %ymm13
        vpsubw %ymm13, %ymm10, %ymm10
        vmovdqa %ymm3, 0x100(%rdi)
        vmovdqa %ymm4, 0x120(%rdi)
        vmovdqa %ymm5, 0x140(%rdi)
        vmovdqa %ymm6, 0x160(%rdi)
        vmovdqa %ymm7, 0x180(%rdi)
        vmovdqa %ymm8, 0x1a0(%rdi)
        vmovdqa %ymm9, 0x1c0(%rdi)
        vmovdqa %ymm10, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_tomont_avx2)

