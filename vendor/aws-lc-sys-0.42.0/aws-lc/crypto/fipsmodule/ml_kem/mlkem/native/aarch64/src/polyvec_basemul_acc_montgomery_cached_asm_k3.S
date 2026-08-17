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
  Name: polyvec_basemul_acc_montgomery_cached_asm_k3
  Description: Re-implementation of asymmetric base multiplication following @[NeonNTT] for k=3
  Signature: void mlk_polyvec_basemul_acc_montgomery_cached_asm_k3(int16_t r[256], const int16_t a[768], const int16_t b[768], const int16_t b_cache[384])
  ABI:
    x0:
      type: buffer
      size_bytes: 512
      permissions: write-only
      c_parameter: int16_t r[256]
      description: Output polynomial
    x1:
      type: buffer
      size_bytes: 1536
      permissions: read-only
      c_parameter: const int16_t a[768]
      description: Input polynomial vector a
    x2:
      type: buffer
      size_bytes: 1536
      permissions: read-only
      c_parameter: const int16_t b[768]
      description: Input polynomial vector b
    x3:
      type: buffer
      size_bytes: 768
      permissions: read-only
      c_parameter: const int16_t b_cache[384]
      description: Cached values for b
  Stack:
    bytes: 64
    description: saving callee-saved Neon registers
*/

/* Re-implementation of asymmetric base multiplication following @[NeonNTT] */

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/aarch64_opt/src/polyvec_basemul_acc_montgomery_cached_asm_k3.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k3)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k3)
S2N_BN_SYMBOL(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k3):

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
        mov x13, #0x10              // =16
        ldr q6, [x7], #0x20
        ldr q19, [x2, #0x10]
        ldr q23, [x1], #0x20
        ldur q14, [x1, #-0x10]
        ldr q17, [x2], #0x20
        ldr q11, [x4, #0x10]
        ldur q28, [x7, #-0x10]
        ld1 { v30.8h }, [x3], #16
        ldr q26, [x4], #0x20
        ldr q16, [x8, #0x10]
        uzp1 v8.8h, v23.8h, v14.8h
        ldr q22, [x5, #0x10]
        ldr q18, [x5], #0x20
        uzp1 v20.8h, v17.8h, v19.8h
        uzp2 v24.8h, v23.8h, v14.8h
        ldr q31, [x8], #0x20
        smull2 v4.4s, v8.8h, v20.8h
        uzp1 v25.8h, v26.8h, v11.8h
        smull v13.4s, v8.4h, v20.4h
        ld1 { v23.8h }, [x6], #16
        uzp1 v1.8h, v18.8h, v22.8h
        smlal v13.4s, v24.4h, v30.4h
        smlal2 v4.4s, v24.8h, v30.8h
        uzp2 v5.8h, v26.8h, v11.8h
        smlal2 v4.4s, v25.8h, v1.8h
        uzp1 v29.8h, v6.8h, v28.8h
        smlal2 v4.4s, v5.8h, v23.8h
        ld1 { v7.8h }, [x9], #16
        smlal v13.4s, v25.4h, v1.4h
        uzp2 v17.8h, v17.8h, v19.8h
        uzp1 v27.8h, v31.8h, v16.8h
        smlal v13.4s, v5.4h, v23.4h
        uzp2 v22.8h, v18.8h, v22.8h
        smull v18.4s, v8.4h, v17.4h
        uzp2 v28.8h, v6.8h, v28.8h
        smlal v13.4s, v29.4h, v27.4h
        smlal2 v4.4s, v29.8h, v27.8h
        uzp2 v26.8h, v31.8h, v16.8h
        smlal2 v4.4s, v28.8h, v7.8h
        ldr q3, [x7, #0x10]
        smlal v13.4s, v28.4h, v7.4h
        ldr q7, [x1], #0x20
        smlal v18.4s, v24.4h, v20.4h
        ldr q15, [x2], #0x20
        smlal v18.4s, v25.4h, v22.4h
        smull2 v8.4s, v8.8h, v17.8h
        ldur q17, [x1, #-0x10]
        uzp1 v23.8h, v13.8h, v4.8h
        smlal v18.4s, v5.4h, v1.4h
        smlal2 v8.4s, v24.8h, v20.8h
        ld1 { v16.8h }, [x3], #16
        mul v23.8h, v23.8h, v2.8h
        ldr q19, [x5, #0x10]
        ldr q14, [x4, #0x10]
        ldr q11, [x4], #0x20
        ldur q20, [x2, #-0x10]
        smlal2 v8.4s, v25.8h, v22.8h
        smlal2 v8.4s, v5.8h, v1.8h
        ldr q22, [x5], #0x20
        uzp1 v1.8h, v7.8h, v17.8h
        smlal v18.4s, v29.4h, v26.4h
        smlal v13.4s, v23.4h, v0.4h
        uzp2 v31.8h, v11.8h, v14.8h
        uzp1 v21.8h, v15.8h, v20.8h
        smlal2 v4.4s, v23.8h, v0.8h
        ld1 { v9.8h }, [x6], #16
        smlal v18.4s, v28.4h, v27.4h
        smlal2 v8.4s, v29.8h, v26.8h
        ldr q25, [x7], #0x20
        smull v26.4s, v1.4h, v21.4h
        uzp1 v24.8h, v22.8h, v19.8h
        smlal2 v8.4s, v28.8h, v27.8h
        uzp2 v28.8h, v7.8h, v17.8h
        uzp1 v29.8h, v11.8h, v14.8h
        smull2 v23.4s, v1.8h, v21.8h
        ldr q27, [x8], #0x20
        smlal2 v23.4s, v28.8h, v16.8h
        ldur q11, [x8, #-0x10]
        smlal2 v23.4s, v29.8h, v24.8h
        uzp2 v7.8h, v13.8h, v4.8h
        uzp2 v19.8h, v22.8h, v19.8h
        ld1 { v4.8h }, [x9], #16
        smlal2 v23.4s, v31.8h, v9.8h
        uzp1 v13.8h, v25.8h, v3.8h
        uzp1 v14.8h, v18.8h, v8.8h
        smlal v26.4s, v28.4h, v16.4h
        uzp2 v17.8h, v27.8h, v11.8h
        uzp2 v20.8h, v15.8h, v20.8h
        mul v14.8h, v14.8h, v2.8h
        sub x13, x13, #0x2

Lpolyvec_basemul_acc_montgomery_cached_k3_loop_start:
        uzp1 v6.8h, v27.8h, v11.8h
        smlal v26.4s, v29.4h, v24.4h
        uzp2 v16.8h, v25.8h, v3.8h
        smlal v26.4s, v31.4h, v9.4h
        ldr q3, [x7, #0x10]
        smlal v26.4s, v13.4h, v6.4h
        smlal2 v8.4s, v14.8h, v0.8h
        ldr q27, [x8], #0x20
        smlal v18.4s, v14.4h, v0.4h
        ldr q25, [x7], #0x20
        smlal2 v23.4s, v13.8h, v6.8h
        ldr q11, [x1], #0x20
        smlal2 v23.4s, v16.8h, v4.8h
        smlal v26.4s, v16.4h, v4.4h
        ldur q22, [x1, #-0x10]
        uzp2 v30.8h, v18.8h, v8.8h
        smull v18.4s, v1.4h, v20.4h
        smlal v18.4s, v28.4h, v21.4h
        ldr q14, [x2], #0x20
        smlal v18.4s, v29.4h, v19.4h
        zip1 v5.8h, v7.8h, v30.8h
        uzp1 v4.8h, v26.8h, v23.8h
        smull2 v8.4s, v1.8h, v20.8h
        zip2 v10.8h, v7.8h, v30.8h
        smlal v18.4s, v31.4h, v24.4h
        mul v12.8h, v4.8h, v2.8h
        ldr q4, [x5, #0x10]
        ldr q20, [x4, #0x10]
        ldr q1, [x4], #0x20
        ldur q30, [x2, #-0x10]
        smlal2 v8.4s, v28.8h, v21.8h
        smlal2 v8.4s, v29.8h, v19.8h
        ldr q19, [x5], #0x20
        smlal2 v8.4s, v31.8h, v24.8h
        ld1 { v15.8h }, [x3], #16
        uzp2 v31.8h, v1.8h, v20.8h
        smlal v26.4s, v12.4h, v0.4h
        smlal2 v23.4s, v12.8h, v0.8h
        uzp1 v21.8h, v14.8h, v30.8h
        uzp1 v29.8h, v1.8h, v20.8h
        uzp1 v1.8h, v11.8h, v22.8h
        smlal2 v8.4s, v13.8h, v17.8h
        ld1 { v9.8h }, [x6], #16
        smlal v18.4s, v13.4h, v17.4h
        uzp1 v24.8h, v19.8h, v4.8h
        uzp2 v7.8h, v26.8h, v23.8h
        smull v26.4s, v1.4h, v21.4h
        smlal v18.4s, v16.4h, v6.4h
        uzp2 v19.8h, v19.8h, v4.8h
        smlal2 v8.4s, v16.8h, v6.8h
        uzp2 v28.8h, v11.8h, v22.8h
        smull2 v23.4s, v1.8h, v21.8h
        uzp1 v13.8h, v25.8h, v3.8h
        smlal2 v23.4s, v28.8h, v15.8h
        ldur q11, [x8, #-0x10]
        smlal2 v23.4s, v29.8h, v24.8h
        ld1 { v4.8h }, [x9], #16
        smlal2 v23.4s, v31.8h, v9.8h
        uzp1 v12.8h, v18.8h, v8.8h
        uzp2 v20.8h, v14.8h, v30.8h
        smlal v26.4s, v28.4h, v15.4h
        str q5, [x0], #0x20
        mul v14.8h, v12.8h, v2.8h
        stur q10, [x0, #-0x10]
        uzp2 v17.8h, v27.8h, v11.8h
        subs x13, x13, #0x1
        cbnz x13, Lpolyvec_basemul_acc_montgomery_cached_k3_loop_start
        uzp2 v3.8h, v25.8h, v3.8h
        smull2 v16.4s, v1.8h, v20.8h
        smull v25.4s, v1.4h, v20.4h
        uzp1 v22.8h, v27.8h, v11.8h
        smlal2 v16.4s, v28.8h, v21.8h
        smlal v25.4s, v28.4h, v21.4h
        smlal2 v16.4s, v29.8h, v19.8h
        smlal v25.4s, v29.4h, v19.4h
        smlal2 v16.4s, v31.8h, v24.8h
        smlal v25.4s, v31.4h, v24.4h
        smlal v25.4s, v13.4h, v17.4h
        smlal2 v16.4s, v13.8h, v17.8h
        smlal2 v16.4s, v3.8h, v22.8h
        smlal v25.4s, v3.4h, v22.4h
        smlal2 v23.4s, v13.8h, v22.8h
        smlal v26.4s, v29.4h, v24.4h
        smlal v26.4s, v31.4h, v9.4h
        smlal v26.4s, v13.4h, v22.4h
        uzp1 v10.8h, v25.8h, v16.8h
        smlal2 v23.4s, v3.8h, v4.8h
        smlal v26.4s, v3.4h, v4.4h
        mul v13.8h, v10.8h, v2.8h
        smlal v18.4s, v14.4h, v0.4h
        smlal2 v8.4s, v14.8h, v0.8h
        uzp1 v3.8h, v26.8h, v23.8h
        mul v24.8h, v3.8h, v2.8h
        uzp2 v17.8h, v18.8h, v8.8h
        smlal v25.4s, v13.4h, v0.4h
        smlal2 v16.4s, v13.8h, v0.8h
        zip1 v21.8h, v7.8h, v17.8h
        zip2 v20.8h, v7.8h, v17.8h
        smlal2 v23.4s, v24.8h, v0.8h
        str q21, [x0], #0x20
        smlal v26.4s, v24.4h, v0.4h
        uzp2 v13.8h, v25.8h, v16.8h
        stur q20, [x0, #-0x10]
        uzp2 v23.8h, v26.8h, v23.8h
        zip1 v18.8h, v23.8h, v13.8h
        zip2 v13.8h, v23.8h, v13.8h
        str q18, [x0], #0x20
        stur q13, [x0, #-0x10]
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

S2N_BN_SIZE_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k3)

