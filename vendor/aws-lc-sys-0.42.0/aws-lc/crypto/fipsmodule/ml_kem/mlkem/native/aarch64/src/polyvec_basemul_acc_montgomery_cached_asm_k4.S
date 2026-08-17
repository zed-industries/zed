/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [NeonNTT]
 *   Neon NTT: Faster Dilithium, Kyber, and Saber on Cortex-A72 and Apple M1
 *   Becker, Hwang, Kannwischer, Yang, Yang
 *   https://eprint.iacr.org/2021/986
 */

/*yaml
  Name: polyvec_basemul_acc_montgomery_cached_asm_k4
  Description: Re-implementation of asymmetric base multiplication following @[NeonNTT] for k=4
  Signature: void mlk_polyvec_basemul_acc_montgomery_cached_asm_k4(int16_t r[256], const int16_t a[1024], const int16_t b[1024], const int16_t b_cache[512])
  ABI:
    x0:
      type: buffer
      size_bytes: 512
      permissions: write-only
      c_parameter: int16_t r[256]
      description: Output polynomial
    x1:
      type: buffer
      size_bytes: 2048
      permissions: read-only
      c_parameter: const int16_t a[1024]
      description: Input polynomial vector a
    x2:
      type: buffer
      size_bytes: 2048
      permissions: read-only
      c_parameter: const int16_t b[1024]
      description: Input polynomial vector b
    x3:
      type: buffer
      size_bytes: 1024
      permissions: read-only
      c_parameter: const int16_t b_cache[512]
      description: Cached values for b
  Stack:
    bytes: 64
    description: saving callee-saved Neon registers
*/

/* Re-implementation of asymmetric base multiplication following @[NeonNTT] */

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/aarch64_opt/src/polyvec_basemul_acc_montgomery_cached_asm_k4.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k4)
S2N_BN_SYMBOL(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k4):

        .cfi_startproc
        sub sp, sp, #0x40
        .cfi_adjust_cfa_offset 0x40
        stp d8, d9, [sp]
        .cfi_rel_offset d8, 0x0
        .cfi_rel_offset d9, 0x8
        stp d10, d11, [sp, #0x10]
        .cfi_rel_offset d10, 0x10
        .cfi_rel_offset d11, 0x18
        stp d12, d13, [sp, #0x20]
        .cfi_rel_offset d12, 0x20
        .cfi_rel_offset d13, 0x28
        stp d14, d15, [sp, #0x30]
        .cfi_rel_offset d14, 0x30
        .cfi_rel_offset d15, 0x38
        mov w14, #0xd01             // =3329
        dup v0.8h, w14
        mov w14, #0xcff             // =3327
        dup v2.8h, w14
        add x4, x1, #0x200
        add x5, x2, #0x200
        add x6, x3, #0x100
        add x7, x1, #0x400
        add x8, x2, #0x400
        add x9, x3, #0x200
        add x10, x1, #0x600
        add x11, x2, #0x600
        add x12, x3, #0x300
        mov x13, #0x10              // =16
        ldr q28, [x1], #0x20
        ldur q5, [x1, #-0x10]
        ldr q31, [x2], #0x20
        ldur q27, [x2, #-0x10]
        ldr q7, [x5], #0x20
        ldr q10, [x4], #0x20
        ldur q18, [x5, #-0x10]
        ldur q9, [x4, #-0x10]
        uzp1 v11.8h, v28.8h, v5.8h
        uzp2 v19.8h, v28.8h, v5.8h
        uzp2 v4.8h, v31.8h, v27.8h
        uzp1 v1.8h, v31.8h, v27.8h
        ldr q29, [x7], #0x20
        ldr q28, [x8, #0x10]
        uzp1 v24.8h, v10.8h, v9.8h
        uzp1 v17.8h, v7.8h, v18.8h
        uzp2 v7.8h, v7.8h, v18.8h
        ldr q21, [x8], #0x20
        uzp2 v27.8h, v10.8h, v9.8h
        ldur q6, [x7, #-0x10]
        smull v18.4s, v11.4h, v4.4h
        ld1 { v9.8h }, [x3], #16
        smull2 v8.4s, v11.8h, v4.8h
        ldr q16, [x11], #0x20
        smlal2 v8.4s, v19.8h, v1.8h
        ldur q14, [x11, #-0x10]
        smlal v18.4s, v19.4h, v1.4h
        uzp1 v10.8h, v21.8h, v28.8h
        smlal v18.4s, v24.4h, v7.4h
        ldr q4, [x10], #0x20
        smlal2 v8.4s, v24.8h, v7.8h
        ld1 { v12.8h }, [x6], #16
        smull2 v23.4s, v11.8h, v1.8h
        uzp2 v13.8h, v29.8h, v6.8h
        smull v26.4s, v11.4h, v1.4h
        uzp1 v29.8h, v29.8h, v6.8h
        smlal v26.4s, v19.4h, v9.4h
        ldur q15, [x10, #-0x10]
        smlal2 v23.4s, v19.8h, v9.8h
        uzp2 v9.8h, v21.8h, v28.8h
        smlal v18.4s, v27.4h, v17.4h
        uzp2 v6.8h, v16.8h, v14.8h
        uzp1 v21.8h, v16.8h, v14.8h
        smlal2 v8.4s, v27.8h, v17.8h
        smlal2 v8.4s, v29.8h, v9.8h
        uzp1 v30.8h, v4.8h, v15.8h
        uzp2 v16.8h, v4.8h, v15.8h
        smlal v18.4s, v29.4h, v9.4h
        smlal2 v8.4s, v13.8h, v10.8h
        ld1 { v15.8h }, [x9], #16
        smlal v18.4s, v13.4h, v10.4h
        ldr q11, [x4], #0x20
        smlal v18.4s, v30.4h, v6.4h
        ldr q7, [x2], #0x20
        smlal2 v8.4s, v30.8h, v6.8h
        ld1 { v9.8h }, [x12], #16
        smlal2 v23.4s, v24.8h, v17.8h
        ldur q4, [x2, #-0x10]
        smlal v26.4s, v24.4h, v17.4h
        ldur q25, [x4, #-0x10]
        smlal2 v8.4s, v16.8h, v21.8h
        ldr q5, [x5], #0x20
        smlal v18.4s, v16.4h, v21.4h
        ldur q22, [x5, #-0x10]
        smlal v26.4s, v27.4h, v12.4h
        ldr q19, [x1, #0x10]
        smlal v26.4s, v29.4h, v10.4h
        ld1 { v20.8h }, [x3], #16
        smlal v26.4s, v13.4h, v15.4h
        uzp1 v24.8h, v7.8h, v4.8h
        smlal2 v23.4s, v27.8h, v12.8h
        uzp1 v28.8h, v18.8h, v8.8h
        smlal v26.4s, v30.4h, v21.4h
        uzp2 v27.8h, v11.8h, v25.8h
        smlal2 v23.4s, v29.8h, v10.8h
        uzp2 v31.8h, v7.8h, v4.8h
        smlal2 v23.4s, v13.8h, v15.8h
        uzp1 v14.8h, v5.8h, v22.8h
        uzp1 v17.8h, v11.8h, v25.8h
        smlal v26.4s, v16.4h, v9.4h
        mul v29.8h, v28.8h, v2.8h
        sub x13, x13, #0x2

Lpolyvec_basemul_acc_montgomery_cached_k4_loop_start:
        smlal2 v23.4s, v30.8h, v21.8h
        ldr q11, [x1], #0x20
        uzp2 v15.8h, v5.8h, v22.8h
        smlal v18.4s, v29.4h, v0.4h
        ldr q12, [x7], #0x20
        smlal2 v8.4s, v29.8h, v0.8h
        ldur q3, [x7, #-0x10]
        ldr q21, [x8], #0x20
        uzp1 v29.8h, v11.8h, v19.8h
        ldur q13, [x8, #-0x10]
        uzp2 v5.8h, v11.8h, v19.8h
        smlal2 v23.4s, v16.8h, v9.8h
        uzp2 v28.8h, v18.8h, v8.8h
        smull2 v8.4s, v29.8h, v31.8h
        smlal2 v8.4s, v5.8h, v24.8h
        uzp1 v7.8h, v12.8h, v3.8h
        smlal2 v8.4s, v17.8h, v15.8h
        uzp2 v11.8h, v21.8h, v13.8h
        uzp1 v4.8h, v26.8h, v23.8h
        smlal2 v8.4s, v27.8h, v14.8h
        smlal2 v8.4s, v7.8h, v11.8h
        mul v6.8h, v4.8h, v2.8h
        ldr q19, [x11], #0x20
        uzp2 v25.8h, v12.8h, v3.8h
        ldr q12, [x10], #0x20
        smull v18.4s, v29.4h, v31.4h
        ldur q3, [x10, #-0x10]
        smlal v18.4s, v5.4h, v24.4h
        uzp1 v4.8h, v21.8h, v13.8h
        smlal v18.4s, v17.4h, v15.4h
        ldur q13, [x11, #-0x10]
        ld1 { v1.8h }, [x6], #16
        smlal v26.4s, v6.4h, v0.4h
        smlal2 v23.4s, v6.8h, v0.8h
        ld1 { v10.8h }, [x9], #16
        smlal v18.4s, v27.4h, v14.4h
        uzp1 v30.8h, v12.8h, v3.8h
        smlal2 v8.4s, v25.8h, v4.8h
        uzp2 v31.8h, v19.8h, v13.8h
        smlal v18.4s, v7.4h, v11.4h
        ld1 { v9.8h }, [x12], #16
        smlal v18.4s, v25.4h, v4.4h
        uzp1 v21.8h, v19.8h, v13.8h
        uzp2 v16.8h, v12.8h, v3.8h
        smlal v18.4s, v30.4h, v31.4h
        smlal2 v8.4s, v30.8h, v31.8h
        uzp2 v31.8h, v26.8h, v23.8h
        smlal2 v8.4s, v16.8h, v21.8h
        smlal v18.4s, v16.4h, v21.4h
        zip1 v15.8h, v31.8h, v28.8h
        ldr q19, [x1, #0x10]
        smull2 v23.4s, v29.8h, v24.8h
        smull v26.4s, v29.4h, v24.4h
        ldr q3, [x2, #0x10]
        smlal v26.4s, v5.4h, v20.4h
        ldr q11, [x2], #0x20
        uzp1 v6.8h, v18.8h, v8.8h
        smlal v26.4s, v17.4h, v14.4h
        smlal v26.4s, v27.4h, v1.4h
        zip2 v13.8h, v31.8h, v28.8h
        smlal v26.4s, v7.4h, v4.4h
        str q15, [x0], #0x20
        smlal v26.4s, v25.4h, v10.4h
        stur q13, [x0, #-0x10]
        mul v29.8h, v6.8h, v2.8h
        uzp1 v24.8h, v11.8h, v3.8h
        uzp2 v31.8h, v11.8h, v3.8h
        ldr q11, [x4], #0x20
        smlal2 v23.4s, v5.8h, v20.8h
        ldur q28, [x4, #-0x10]
        smlal2 v23.4s, v17.8h, v14.8h
        ldr q5, [x5], #0x20
        smlal2 v23.4s, v27.8h, v1.8h
        ldur q22, [x5, #-0x10]
        smlal v26.4s, v30.4h, v21.4h
        ld1 { v20.8h }, [x3], #16
        smlal v26.4s, v16.4h, v9.4h
        uzp1 v17.8h, v11.8h, v28.8h
        smlal2 v23.4s, v7.8h, v4.8h
        uzp2 v27.8h, v11.8h, v28.8h
        smlal2 v23.4s, v25.8h, v10.8h
        uzp1 v14.8h, v5.8h, v22.8h
        subs x13, x13, #0x1
        cbnz x13, Lpolyvec_basemul_acc_montgomery_cached_k4_loop_start
        smlal v18.4s, v29.4h, v0.4h
        ldr q11, [x1], #0x20
        uzp2 v28.8h, v5.8h, v22.8h
        smlal2 v23.4s, v30.8h, v21.8h
        smlal2 v8.4s, v29.8h, v0.8h
        ldr q15, [x8, #0x10]
        smlal2 v23.4s, v16.8h, v9.8h
        ldr q21, [x8], #0x20
        uzp1 v22.8h, v11.8h, v19.8h
        uzp2 v12.8h, v11.8h, v19.8h
        ldr q1, [x7, #0x10]
        ld1 { v6.8h }, [x6], #16
        uzp2 v3.8h, v18.8h, v8.8h
        smull v9.4s, v22.4h, v31.4h
        smull2 v18.4s, v22.8h, v31.8h
        ldr q16, [x7], #0x20
        smull v19.4s, v22.4h, v24.4h
        uzp1 v30.8h, v21.8h, v15.8h
        uzp2 v25.8h, v21.8h, v15.8h
        smull2 v8.4s, v22.8h, v24.8h
        smlal v19.4s, v12.4h, v20.4h
        ldr q13, [x10, #0x10]
        smlal2 v8.4s, v12.8h, v20.8h
        uzp1 v29.8h, v16.8h, v1.8h
        smlal2 v18.4s, v12.8h, v24.8h
        ldr q5, [x10], #0x20
        smlal v9.4s, v12.4h, v24.4h
        ldr q4, [x11], #0x20
        smlal v9.4s, v17.4h, v28.4h
        ldur q22, [x11, #-0x10]
        smlal2 v18.4s, v17.8h, v28.8h
        uzp2 v16.8h, v16.8h, v1.8h
        smlal v19.4s, v17.4h, v14.4h
        ld1 { v28.8h }, [x9], #16
        smlal2 v8.4s, v17.8h, v14.8h
        uzp1 v7.8h, v5.8h, v13.8h
        smlal v9.4s, v27.4h, v14.4h
        uzp1 v17.8h, v4.8h, v22.8h
        smlal2 v18.4s, v27.8h, v14.8h
        uzp2 v12.8h, v5.8h, v13.8h
        uzp2 v21.8h, v4.8h, v22.8h
        smlal v19.4s, v27.4h, v6.4h
        smlal2 v8.4s, v27.8h, v6.8h
        ld1 { v15.8h }, [x12], #16
        smlal v19.4s, v29.4h, v30.4h
        uzp1 v20.8h, v26.8h, v23.8h
        smlal v9.4s, v29.4h, v25.4h
        smlal2 v18.4s, v29.8h, v25.8h
        smlal2 v8.4s, v29.8h, v30.8h
        smlal v19.4s, v16.4h, v28.4h
        smlal2 v8.4s, v16.8h, v28.8h
        smlal2 v18.4s, v16.8h, v30.8h
        smlal v9.4s, v16.4h, v30.4h
        smlal v9.4s, v7.4h, v21.4h
        smlal2 v18.4s, v7.8h, v21.8h
        smlal2 v8.4s, v7.8h, v17.8h
        smlal v19.4s, v7.4h, v17.4h
        smlal v19.4s, v12.4h, v15.4h
        smlal2 v8.4s, v12.8h, v15.8h
        smlal2 v18.4s, v12.8h, v17.8h
        smlal v9.4s, v12.4h, v17.4h
        mul v6.8h, v20.8h, v2.8h
        uzp1 v4.8h, v19.8h, v8.8h
        mul v17.8h, v4.8h, v2.8h
        uzp1 v12.8h, v9.8h, v18.8h
        smlal v26.4s, v6.4h, v0.4h
        mul v21.8h, v12.8h, v2.8h
        smlal2 v23.4s, v6.8h, v0.8h
        smlal2 v8.4s, v17.8h, v0.8h
        smlal v19.4s, v17.4h, v0.4h
        smlal2 v18.4s, v21.8h, v0.8h
        uzp2 v23.8h, v26.8h, v23.8h
        smlal v9.4s, v21.4h, v0.4h
        zip2 v12.8h, v23.8h, v3.8h
        zip1 v22.8h, v23.8h, v3.8h
        uzp2 v14.8h, v19.8h, v8.8h
        uzp2 v18.8h, v9.8h, v18.8h
        str q12, [x0, #0x10]
        str q22, [x0], #0x20
        zip2 v24.8h, v14.8h, v18.8h
        zip1 v21.8h, v14.8h, v18.8h
        str q24, [x0, #0x10]
        str q21, [x0], #0x20
        ldp d8, d9, [sp]
        .cfi_restore d8
        .cfi_restore d9
        ldp d10, d11, [sp, #0x10]
        .cfi_restore d10
        .cfi_restore d11
        ldp d12, d13, [sp, #0x20]
        .cfi_restore d12
        .cfi_restore d13
        ldp d14, d15, [sp, #0x30]
        .cfi_restore d14
        .cfi_restore d15
        add sp, sp, #0x40
        .cfi_adjust_cfa_offset -0x40
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k4)

