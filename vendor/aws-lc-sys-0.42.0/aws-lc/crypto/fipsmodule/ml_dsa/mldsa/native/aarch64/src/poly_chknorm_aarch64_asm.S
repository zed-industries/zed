/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#include "_internal_s2n_bignum_arm.h"


/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/poly_chknorm_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_poly_chknorm_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_poly_chknorm_aarch64_asm)
S2N_BN_SYMBOL(mldsa_poly_chknorm_aarch64_asm):

        .cfi_startproc
        dup v20.4s, w1
        eor v21.16b, v21.16b, v21.16b
        mov x2, #0x10               // =16

Lmldsa_poly_chknorm_loop:
        ldr q1, [x0, #0x10]
        ldr q2, [x0, #0x20]
        ldr q3, [x0, #0x30]
        ldr q0, [x0], #0x40
        abs v1.4s, v1.4s
        cmge v1.4s, v1.4s, v20.4s
        orr v21.16b, v21.16b, v1.16b
        abs v2.4s, v2.4s
        cmge v2.4s, v2.4s, v20.4s
        orr v21.16b, v21.16b, v2.16b
        abs v3.4s, v3.4s
        cmge v3.4s, v3.4s, v20.4s
        orr v21.16b, v21.16b, v3.16b
        abs v0.4s, v0.4s
        cmge v0.4s, v0.4s, v20.4s
        orr v21.16b, v21.16b, v0.16b
        subs x2, x2, #0x1
        b.ne Lmldsa_poly_chknorm_loop
        umaxv s21, v21.4s
        fmov w0, s21
        and w0, w0, #0x1
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_poly_chknorm_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
