/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#include "_internal_s2n_bignum_arm.h"


/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/poly_caddq_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_poly_caddq_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_poly_caddq_aarch64_asm)
S2N_BN_SYMBOL(mldsa_poly_caddq_aarch64_asm):

        .cfi_startproc
        mov w9, #0xe001             // =57345
        movk w9, #0x7f, lsl #16
        dup v4.4s, w9
        mov x1, #0x10               // =16

Lmldsa_poly_caddq_loop:
        ldr q0, [x0]
        ldr q1, [x0, #0x10]
        ldr q2, [x0, #0x20]
        ldr q3, [x0, #0x30]
        ushr v5.4s, v0.4s, #0x1f
        mla v0.4s, v5.4s, v4.4s
        ushr v5.4s, v1.4s, #0x1f
        mla v1.4s, v5.4s, v4.4s
        ushr v5.4s, v2.4s, #0x1f
        mla v2.4s, v5.4s, v4.4s
        ushr v5.4s, v3.4s, #0x1f
        mla v3.4s, v5.4s, v4.4s
        str q1, [x0, #0x10]
        str q2, [x0, #0x20]
        str q3, [x0, #0x30]
        str q0, [x0], #0x40
        subs x1, x1, #0x1
        b.ne Lmldsa_poly_caddq_loop
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_poly_caddq_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
