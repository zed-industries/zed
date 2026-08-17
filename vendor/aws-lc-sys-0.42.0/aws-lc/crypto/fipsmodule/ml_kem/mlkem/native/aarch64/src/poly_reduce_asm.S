/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/*yaml
  Name: poly_reduce_asm
  Description: Barrett reduction of polynomial coefficients
  Signature: void mlk_poly_reduce_asm(int16_t p[256])
  ABI:
    x0:
      type: buffer
      size_bytes: 512
      permissions: read/write
      c_parameter: int16_t p[256]
      description: Input/output polynomial
  Stack:
    bytes: 0
*/

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/aarch64_opt/src/poly_reduce_asm.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_reduce_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_reduce_asm)
S2N_BN_SYMBOL(mlkem_poly_reduce_asm):

        .cfi_startproc
        mov w2, #0xd01              // =3329
        dup v3.8h, w2
        mov w2, #0x4ebf             // =20159
        dup v4.8h, w2
        mov x1, #0x8                // =8
        ldr q21, [x0], #0x40
        ldur q18, [x0, #-0x20]
        ldur q0, [x0, #-0x30]
        ldur q5, [x0, #-0x10]
        ldr q26, [x0], #0x40
        sqdmulh v17.8h, v21.8h, v4.h[0]
        sqdmulh v27.8h, v18.8h, v4.h[0]
        sqdmulh v22.8h, v0.8h, v4.h[0]
        srshr v17.8h, v17.8h, #0xb
        sqdmulh v23.8h, v5.8h, v4.h[0]
        srshr v29.8h, v27.8h, #0xb
        mls v21.8h, v17.8h, v3.h[0]
        srshr v17.8h, v22.8h, #0xb
        mls v18.8h, v29.8h, v3.h[0]
        srshr v22.8h, v23.8h, #0xb
        mls v0.8h, v17.8h, v3.h[0]
        sshr v2.8h, v21.8h, #0xf
        mls v5.8h, v22.8h, v3.h[0]
        sshr v29.8h, v18.8h, #0xf
        and v19.16b, v3.16b, v2.16b
        sqdmulh v2.8h, v26.8h, v4.h[0]
        sshr v31.8h, v0.8h, #0xf
        add v17.8h, v21.8h, v19.8h
        and v21.16b, v3.16b, v29.16b
        and v31.16b, v3.16b, v31.16b
        sub x1, x1, #0x2

Lpoly_reduce_loop_start:
        add v21.8h, v18.8h, v21.8h
        ldur q18, [x0, #-0x20]
        add v25.8h, v0.8h, v31.8h
        ldur q0, [x0, #-0x30]
        stur q21, [x0, #-0x60]
        sshr v28.8h, v5.8h, #0xf
        stur q17, [x0, #-0x80]
        srshr v23.8h, v2.8h, #0xb
        sqdmulh v30.8h, v18.8h, v4.h[0]
        stur q25, [x0, #-0x70]
        and v22.16b, v3.16b, v28.16b
        sqdmulh v7.8h, v0.8h, v4.h[0]
        add v16.8h, v5.8h, v22.8h
        ldur q5, [x0, #-0x10]
        mls v26.8h, v23.8h, v3.h[0]
        stur q16, [x0, #-0x50]
        srshr v6.8h, v30.8h, #0xb
        srshr v1.8h, v7.8h, #0xb
        sqdmulh v19.8h, v5.8h, v4.h[0]
        mls v18.8h, v6.8h, v3.h[0]
        sshr v24.8h, v26.8h, #0xf
        mls v0.8h, v1.8h, v3.h[0]
        and v27.16b, v3.16b, v24.16b
        srshr v29.8h, v19.8h, #0xb
        add v17.8h, v26.8h, v27.8h
        ldr q26, [x0], #0x40
        sshr v1.8h, v18.8h, #0xf
        mls v5.8h, v29.8h, v3.h[0]
        sshr v20.8h, v0.8h, #0xf
        and v21.16b, v3.16b, v1.16b
        and v31.16b, v3.16b, v20.16b
        sqdmulh v2.8h, v26.8h, v4.h[0]
        subs x1, x1, #0x1
        cbnz x1, Lpoly_reduce_loop_start
        add v28.8h, v0.8h, v31.8h
        ldur q29, [x0, #-0x10]
        add v21.8h, v18.8h, v21.8h
        srshr v18.8h, v2.8h, #0xb
        sshr v2.8h, v5.8h, #0xf
        ldur q16, [x0, #-0x20]
        stur q17, [x0, #-0x80]
        ldur q0, [x0, #-0x30]
        and v2.16b, v3.16b, v2.16b
        sqdmulh v24.8h, v29.8h, v4.h[0]
        stur q28, [x0, #-0x70]
        stur q21, [x0, #-0x60]
        add v31.8h, v5.8h, v2.8h
        sqdmulh v6.8h, v16.8h, v4.h[0]
        stur q31, [x0, #-0x50]
        sqdmulh v17.8h, v0.8h, v4.h[0]
        srshr v22.8h, v24.8h, #0xb
        mls v26.8h, v18.8h, v3.h[0]
        srshr v31.8h, v6.8h, #0xb
        mls v29.8h, v22.8h, v3.h[0]
        srshr v19.8h, v17.8h, #0xb
        mls v16.8h, v31.8h, v3.h[0]
        sshr v7.8h, v26.8h, #0xf
        mls v0.8h, v19.8h, v3.h[0]
        and v5.16b, v3.16b, v7.16b
        sshr v22.8h, v29.8h, #0xf
        add v27.8h, v26.8h, v5.8h
        and v26.16b, v3.16b, v22.16b
        sshr v20.8h, v16.8h, #0xf
        stur q27, [x0, #-0x40]
        and v2.16b, v3.16b, v20.16b
        sshr v23.8h, v0.8h, #0xf
        add v18.8h, v29.8h, v26.8h
        add v31.8h, v16.8h, v2.8h
        and v29.16b, v3.16b, v23.16b
        stur q18, [x0, #-0x10]
        add v25.8h, v0.8h, v29.8h
        stur q31, [x0, #-0x20]
        stur q25, [x0, #-0x30]
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_reduce_asm)

