/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#include "_internal_s2n_bignum_arm.h"


/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/poly_decompose_88_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_poly_decompose_88_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_poly_decompose_88_aarch64_asm)
S2N_BN_SYMBOL(mldsa_poly_decompose_88_aarch64_asm):

        .cfi_startproc
        mov w4, #0xe001             // =57345
        movk w4, #0x7f, lsl #16
        dup v20.4s, w4
        mov w5, #0x6c00             // =27648
        movk w5, #0x7e, lsl #16
        dup v21.4s, w5
        mov w7, #0xe800             // =59392
        movk w7, #0x2, lsl #16
        dup v22.4s, w7
        mov w11, #0x581             // =1409
        movk w11, #0x5816, lsl #16
        dup v23.4s, w11
        mov x3, #0x10               // =16

Lmldsa_poly_decompose_88_loop:
        ldr q0, [x1]
        ldr q1, [x1, #0x10]
        ldr q2, [x1, #0x20]
        ldr q3, [x1, #0x30]
        sqdmulh v5.4s, v1.4s, v23.4s
        srshr v5.4s, v5.4s, #0x11
        cmgt v24.4s, v1.4s, v21.4s
        mls v1.4s, v5.4s, v22.4s
        bic v5.16b, v5.16b, v24.16b
        add v1.4s, v1.4s, v24.4s
        sqdmulh v6.4s, v2.4s, v23.4s
        srshr v6.4s, v6.4s, #0x11
        cmgt v24.4s, v2.4s, v21.4s
        mls v2.4s, v6.4s, v22.4s
        bic v6.16b, v6.16b, v24.16b
        add v2.4s, v2.4s, v24.4s
        sqdmulh v7.4s, v3.4s, v23.4s
        srshr v7.4s, v7.4s, #0x11
        cmgt v24.4s, v3.4s, v21.4s
        mls v3.4s, v7.4s, v22.4s
        bic v7.16b, v7.16b, v24.16b
        add v3.4s, v3.4s, v24.4s
        sqdmulh v4.4s, v0.4s, v23.4s
        srshr v4.4s, v4.4s, #0x11
        cmgt v24.4s, v0.4s, v21.4s
        mls v0.4s, v4.4s, v22.4s
        bic v4.16b, v4.16b, v24.16b
        add v0.4s, v0.4s, v24.4s
        str q5, [x0, #0x10]
        str q6, [x0, #0x20]
        str q7, [x0, #0x30]
        str q4, [x0], #0x40
        str q1, [x1, #0x10]
        str q2, [x1, #0x20]
        str q3, [x1, #0x30]
        str q0, [x1], #0x40
        subs x3, x3, #0x1
        b.ne Lmldsa_poly_decompose_88_loop
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_poly_decompose_88_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
