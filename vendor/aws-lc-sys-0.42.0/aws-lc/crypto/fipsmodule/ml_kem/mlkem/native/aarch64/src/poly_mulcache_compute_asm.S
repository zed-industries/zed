/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/*yaml
  Name: poly_mulcache_compute_asm
  Description: Compute multiplication cache for polynomial
  Signature: void mlk_poly_mulcache_compute_asm(int16_t cache[128], const int16_t mlk_poly[256], const int16_t zetas[128], const int16_t zetas_twisted[128])
  ABI:
    x0:
      type: buffer
      size_bytes: 256
      permissions: write-only
      c_parameter: int16_t cache[128]
      description: Output cache
    x1:
      type: buffer
      size_bytes: 512
      permissions: read-only
      c_parameter: const int16_t mlk_poly[256]
      description: Input polynomial
    x2:
      type: buffer
      size_bytes: 256
      permissions: read-only
      c_parameter: const int16_t zetas[128]
      description: Zeta values
    x3:
      type: buffer
      size_bytes: 256
      permissions: read-only
      c_parameter: const int16_t zetas_twisted[128]
      description: Twisted zeta values
  Stack:
    bytes: 0
*/

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/aarch64_opt/src/poly_mulcache_compute_asm.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_poly_mulcache_compute_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_poly_mulcache_compute_asm)
S2N_BN_SYMBOL(mlkem_poly_mulcache_compute_asm):

        .cfi_startproc
        mov w5, #0xd01              // =3329
        dup v6.8h, w5
        mov w5, #0x4ebf             // =20159
        dup v7.8h, w5
        mov x4, #0x10               // =16
        ldr q0, [x1], #0x20
        ldur q2, [x1, #-0x10]
        ldr q19, [x1], #0x20
        ldr q29, [x3], #0x10
        ldur q16, [x1, #-0x10]
        ldr q18, [x2], #0x10
        ldr q26, [x1], #0x20
        ldr q25, [x2], #0x10
        uzp2 v5.8h, v0.8h, v2.8h
        ldr q28, [x3], #0x10
        ldur q7, [x1, #-0x10]
        ldr q2, [x1], #0x20
        uzp2 v27.8h, v19.8h, v16.8h
        sqrdmulh v16.8h, v5.8h, v29.8h
        ldr q17, [x3], #0x10
        ldr q19, [x3], #0x10
        mul v5.8h, v5.8h, v18.8h
        uzp2 v29.8h, v26.8h, v7.8h
        mul v26.8h, v27.8h, v25.8h
        sqrdmulh v4.8h, v27.8h, v28.8h
        mls v5.8h, v16.8h, v6.h[0]
        lsr x4, x4, #1
        sub x4, x4, #0x2

Lpoly_mulcache_compute_loop_start:
        str q5, [x0], #0x10
        sqrdmulh v22.8h, v29.8h, v17.8h
        ldr q28, [x2], #0x10
        ldur q24, [x1, #-0x10]
        ldr q0, [x1], #0x20
        mls v26.8h, v4.8h, v6.h[0]
        ldur q16, [x1, #-0x10]
        ldr q17, [x3], #0x10
        mul v5.8h, v29.8h, v28.8h
        uzp2 v23.8h, v2.8h, v24.8h
        ldr q18, [x2], #0x10
        mls v5.8h, v22.8h, v6.h[0]
        uzp2 v29.8h, v0.8h, v16.8h
        sqrdmulh v4.8h, v23.8h, v19.8h
        ldr q2, [x1], #0x20
        ldr q19, [x3], #0x10
        str q26, [x0], #0x10
        mul v26.8h, v23.8h, v18.8h
        subs x4, x4, #0x1
        cbnz x4, Lpoly_mulcache_compute_loop_start
        mls v26.8h, v4.8h, v6.h[0]
        str q5, [x0], #0x10
        ldr q5, [x2], #0x10
        ldur q4, [x1, #-0x10]
        sqrdmulh v16.8h, v29.8h, v17.8h
        ldr q0, [x2], #0x10
        mul v29.8h, v29.8h, v5.8h
        uzp2 v18.8h, v2.8h, v4.8h
        str q26, [x0], #0x10
        sqrdmulh v17.8h, v18.8h, v19.8h
        mls v29.8h, v16.8h, v6.h[0]
        mul v26.8h, v18.8h, v0.8h
        mls v26.8h, v17.8h, v6.h[0]
        str q29, [x0], #0x10
        str q26, [x0], #0x10
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_poly_mulcache_compute_asm)

