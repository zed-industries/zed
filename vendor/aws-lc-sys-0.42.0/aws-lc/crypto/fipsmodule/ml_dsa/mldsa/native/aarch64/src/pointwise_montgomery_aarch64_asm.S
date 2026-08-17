/* Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/pointwise_montgomery_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_poly_pointwise_montgomery_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_poly_pointwise_montgomery_aarch64_asm)
S2N_BN_SYMBOL(mldsa_poly_pointwise_montgomery_aarch64_asm):

        .cfi_startproc
        mov w3, #0xe001             // =57345
        movk w3, #0x7f, lsl #16
        dup v0.4s, w3
        mov w3, #0x2001             // =8193
        movk w3, #0x380, lsl #16
        dup v1.4s, w3
        mov x3, #0x40               // =64

Lmldsa_poly_pointwise_montgomery_loop_start:
        ldr q16, [x0]
        ldr q17, [x0, #0x10]
        ldr q18, [x0, #0x20]
        ldr q19, [x0, #0x30]
        ldr q21, [x1, #0x10]
        ldr q22, [x1, #0x20]
        ldr q23, [x1, #0x30]
        ldr q20, [x1], #0x40
        smull v24.2d, v16.2s, v20.2s
        smull2 v25.2d, v16.4s, v20.4s
        smull v26.2d, v17.2s, v21.2s
        smull2 v27.2d, v17.4s, v21.4s
        smull v28.2d, v18.2s, v22.2s
        smull2 v29.2d, v18.4s, v22.4s
        smull v30.2d, v19.2s, v23.2s
        smull2 v31.2d, v19.4s, v23.4s
        uzp1 v16.4s, v24.4s, v25.4s
        mul v16.4s, v16.4s, v1.4s
        smlsl v24.2d, v16.2s, v0.2s
        smlsl2 v25.2d, v16.4s, v0.4s
        uzp2 v16.4s, v24.4s, v25.4s
        uzp1 v17.4s, v26.4s, v27.4s
        mul v17.4s, v17.4s, v1.4s
        smlsl v26.2d, v17.2s, v0.2s
        smlsl2 v27.2d, v17.4s, v0.4s
        uzp2 v17.4s, v26.4s, v27.4s
        uzp1 v18.4s, v28.4s, v29.4s
        mul v18.4s, v18.4s, v1.4s
        smlsl v28.2d, v18.2s, v0.2s
        smlsl2 v29.2d, v18.4s, v0.4s
        uzp2 v18.4s, v28.4s, v29.4s
        uzp1 v19.4s, v30.4s, v31.4s
        mul v19.4s, v19.4s, v1.4s
        smlsl v30.2d, v19.2s, v0.2s
        smlsl2 v31.2d, v19.4s, v0.4s
        uzp2 v19.4s, v30.4s, v31.4s
        str q17, [x0, #0x10]
        str q18, [x0, #0x20]
        str q19, [x0, #0x30]
        str q16, [x0], #0x40
        subs x3, x3, #0x4
        cbnz x3, Lmldsa_poly_pointwise_montgomery_loop_start
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_poly_pointwise_montgomery_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
