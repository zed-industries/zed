/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#include "_internal_s2n_bignum_arm.h"


/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/poly_use_hint_32_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_poly_use_hint_32_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_poly_use_hint_32_aarch64_asm)
S2N_BN_SYMBOL(mldsa_poly_use_hint_32_aarch64_asm):

        .cfi_startproc
        mov w4, #0xe001             // =57345
        movk w4, #0x7f, lsl #16
        dup v20.4s, w4
        mov w5, #0xe100             // =57600
        movk w5, #0x7b, lsl #16
        dup v21.4s, w5
        mov w7, #0xfe00             // =65024
        movk w7, #0x7, lsl #16
        dup v22.4s, w7
        mov w11, #0x401             // =1025
        movk w11, #0x4010, lsl #16
        dup v23.4s, w11
        movi v24.4s, #0xf
        mov x3, #0x10               // =16

Lmldsa_poly_use_hint_32_loop:
        ldr q1, [x0, #0x10]
        ldr q2, [x0, #0x20]
        ldr q3, [x0, #0x30]
        ldr q0, [x0]
        ldr q5, [x1, #0x10]
        ldr q6, [x1, #0x20]
        ldr q7, [x1, #0x30]
        ldr q4, [x1], #0x40
        sqdmulh v17.4s, v1.4s, v23.4s
        srshr v17.4s, v17.4s, #0x12
        cmgt v25.4s, v1.4s, v21.4s
        mls v1.4s, v17.4s, v22.4s
        bic v17.16b, v17.16b, v25.16b
        add v1.4s, v1.4s, v25.4s
        cmle v1.4s, v1.4s, #0
        orr v1.4s, #0x1
        mla v17.4s, v1.4s, v5.4s
        and v17.16b, v17.16b, v24.16b
        sqdmulh v18.4s, v2.4s, v23.4s
        srshr v18.4s, v18.4s, #0x12
        cmgt v25.4s, v2.4s, v21.4s
        mls v2.4s, v18.4s, v22.4s
        bic v18.16b, v18.16b, v25.16b
        add v2.4s, v2.4s, v25.4s
        cmle v2.4s, v2.4s, #0
        orr v2.4s, #0x1
        mla v18.4s, v2.4s, v6.4s
        and v18.16b, v18.16b, v24.16b
        sqdmulh v19.4s, v3.4s, v23.4s
        srshr v19.4s, v19.4s, #0x12
        cmgt v25.4s, v3.4s, v21.4s
        mls v3.4s, v19.4s, v22.4s
        bic v19.16b, v19.16b, v25.16b
        add v3.4s, v3.4s, v25.4s
        cmle v3.4s, v3.4s, #0
        orr v3.4s, #0x1
        mla v19.4s, v3.4s, v7.4s
        and v19.16b, v19.16b, v24.16b
        sqdmulh v16.4s, v0.4s, v23.4s
        srshr v16.4s, v16.4s, #0x12
        cmgt v25.4s, v0.4s, v21.4s
        mls v0.4s, v16.4s, v22.4s
        bic v16.16b, v16.16b, v25.16b
        add v0.4s, v0.4s, v25.4s
        cmle v0.4s, v0.4s, #0
        orr v0.4s, #0x1
        mla v16.4s, v0.4s, v4.4s
        and v16.16b, v16.16b, v24.16b
        str q17, [x0, #0x10]
        str q18, [x0, #0x20]
        str q19, [x0, #0x30]
        str q16, [x0], #0x40
        subs x3, x3, #0x1
        b.ne Lmldsa_poly_use_hint_32_loop
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_poly_use_hint_32_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
