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
 *   dev/x86_64/src/reduce.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_reduce_avx2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_reduce_avx2)
S2N_BN_SYMBOL(mlkem_reduce_avx2):

        .cfi_startproc
        movl $0xd010d01, %eax        # imm = 0xD010D01
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        movl $0x4ebf4ebf, %eax       # imm = 0x4EBF4EBF
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        vmovdqa (%rdi), %ymm2
        vmovdqa 0x20(%rdi), %ymm3
        vmovdqa 0x40(%rdi), %ymm4
        vmovdqa 0x60(%rdi), %ymm5
        vmovdqa 0x80(%rdi), %ymm6
        vmovdqa 0xa0(%rdi), %ymm7
        vmovdqa 0xc0(%rdi), %ymm8
        vmovdqa 0xe0(%rdi), %ymm9
        vpmulhw %ymm1, %ymm2, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm2, %ymm2
        vpmulhw %ymm1, %ymm3, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm3, %ymm3
        vpmulhw %ymm1, %ymm4, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm4, %ymm4
        vpmulhw %ymm1, %ymm5, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm5, %ymm5
        vpmulhw %ymm1, %ymm6, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm6, %ymm6
        vpmulhw %ymm1, %ymm7, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm7, %ymm7
        vpmulhw %ymm1, %ymm8, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm8, %ymm8
        vpmulhw %ymm1, %ymm9, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vpsubw %ymm0, %ymm2, %ymm2
        vpsraw $0xf, %ymm2, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm2, %ymm2
        vpsubw %ymm0, %ymm3, %ymm3
        vpsraw $0xf, %ymm3, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm3, %ymm3
        vpsubw %ymm0, %ymm4, %ymm4
        vpsraw $0xf, %ymm4, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm4, %ymm4
        vpsubw %ymm0, %ymm5, %ymm5
        vpsraw $0xf, %ymm5, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm5, %ymm5
        vpsubw %ymm0, %ymm6, %ymm6
        vpsraw $0xf, %ymm6, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm6, %ymm6
        vpsubw %ymm0, %ymm7, %ymm7
        vpsraw $0xf, %ymm7, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm7, %ymm7
        vpsubw %ymm0, %ymm8, %ymm8
        vpsraw $0xf, %ymm8, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm8, %ymm8
        vpsubw %ymm0, %ymm9, %ymm9
        vpsraw $0xf, %ymm9, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm9, %ymm9
        vmovdqa %ymm2, (%rdi)
        vmovdqa %ymm3, 0x20(%rdi)
        vmovdqa %ymm4, 0x40(%rdi)
        vmovdqa %ymm5, 0x60(%rdi)
        vmovdqa %ymm6, 0x80(%rdi)
        vmovdqa %ymm7, 0xa0(%rdi)
        vmovdqa %ymm8, 0xc0(%rdi)
        vmovdqa %ymm9, 0xe0(%rdi)
        vmovdqa 0x100(%rdi), %ymm2
        vmovdqa 0x120(%rdi), %ymm3
        vmovdqa 0x140(%rdi), %ymm4
        vmovdqa 0x160(%rdi), %ymm5
        vmovdqa 0x180(%rdi), %ymm6
        vmovdqa 0x1a0(%rdi), %ymm7
        vmovdqa 0x1c0(%rdi), %ymm8
        vmovdqa 0x1e0(%rdi), %ymm9
        vpmulhw %ymm1, %ymm2, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm2, %ymm2
        vpmulhw %ymm1, %ymm3, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm3, %ymm3
        vpmulhw %ymm1, %ymm4, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm4, %ymm4
        vpmulhw %ymm1, %ymm5, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm5, %ymm5
        vpmulhw %ymm1, %ymm6, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm6, %ymm6
        vpmulhw %ymm1, %ymm7, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm7, %ymm7
        vpmulhw %ymm1, %ymm8, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm8, %ymm8
        vpmulhw %ymm1, %ymm9, %ymm12
        vpsraw $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw %ymm12, %ymm9, %ymm9
        vpsubw %ymm0, %ymm2, %ymm2
        vpsraw $0xf, %ymm2, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm2, %ymm2
        vpsubw %ymm0, %ymm3, %ymm3
        vpsraw $0xf, %ymm3, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm3, %ymm3
        vpsubw %ymm0, %ymm4, %ymm4
        vpsraw $0xf, %ymm4, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm4, %ymm4
        vpsubw %ymm0, %ymm5, %ymm5
        vpsraw $0xf, %ymm5, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm5, %ymm5
        vpsubw %ymm0, %ymm6, %ymm6
        vpsraw $0xf, %ymm6, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm6, %ymm6
        vpsubw %ymm0, %ymm7, %ymm7
        vpsraw $0xf, %ymm7, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm7, %ymm7
        vpsubw %ymm0, %ymm8, %ymm8
        vpsraw $0xf, %ymm8, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm8, %ymm8
        vpsubw %ymm0, %ymm9, %ymm9
        vpsraw $0xf, %ymm9, %ymm12
        vpand %ymm0, %ymm12, %ymm12
        vpaddw %ymm12, %ymm9, %ymm9
        vmovdqa %ymm2, 0x100(%rdi)
        vmovdqa %ymm3, 0x120(%rdi)
        vmovdqa %ymm4, 0x140(%rdi)
        vmovdqa %ymm5, 0x160(%rdi)
        vmovdqa %ymm6, 0x180(%rdi)
        vmovdqa %ymm7, 0x1a0(%rdi)
        vmovdqa %ymm8, 0x1c0(%rdi)
        vmovdqa %ymm9, 0x1e0(%rdi)
        retq
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_reduce_avx2)

