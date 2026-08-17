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
  Name: polyvec_basemul_acc_montgomery_cached_asm_k2
  Description: Re-implementation of asymmetric base multiplication following @[NeonNTT] for k=2
  Signature: void mlk_polyvec_basemul_acc_montgomery_cached_asm_k2(int16_t r[256], const int16_t a[512], const int16_t b[512], const int16_t b_cache[256])
  ABI:
    x0:
      type: buffer
      size_bytes: 512
      permissions: write-only
      c_parameter: int16_t r[256]
      description: Output polynomial
    x1:
      type: buffer
      size_bytes: 1024
      permissions: read-only
      c_parameter: const int16_t a[512]
      description: Input polynomial vector a
    x2:
      type: buffer
      size_bytes: 1024
      permissions: read-only
      c_parameter: const int16_t b[512]
      description: Input polynomial vector b
    x3:
      type: buffer
      size_bytes: 512
      permissions: read-only
      c_parameter: const int16_t b_cache[256]
      description: Cached values for b
  Stack:
    bytes: 64
    description: saving callee-saved Neon registers
*/

/* Re-implementation of asymmetric base multiplication following @[NeonNTT] */

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/aarch64_opt/src/polyvec_basemul_acc_montgomery_cached_asm_k2.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k2)
S2N_BN_SYMBOL(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k2):

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
        mov x13, #0x10              // =16
        ldr q12, [x1], #0x20
        ldur q9, [x1, #-0x10]
        ldr q22, [x2], #0x20
        ldur q30, [x2, #-0x10]
        ldr q6, [x5], #0x20
        ldr q7, [x4, #0x10]
        ldr q8, [x4], #0x20
        ldur q23, [x5, #-0x10]
        uzp1 v16.8h, v12.8h, v9.8h
        uzp2 v14.8h, v12.8h, v9.8h
        uzp2 v13.8h, v22.8h, v30.8h
        uzp1 v18.8h, v22.8h, v30.8h
        ld1 { v27.8h }, [x3], #16
        ld1 { v17.8h }, [x6], #16
        smull2 v4.4s, v16.8h, v18.8h
        ldr q31, [x1, #0x10]
        smull v19.4s, v16.4h, v13.4h
        ldr q24, [x1], #0x20
        smlal v19.4s, v14.4h, v18.4h
        ldr q22, [x2], #0x20
        smlal2 v4.4s, v14.8h, v27.8h
        uzp2 v5.8h, v6.8h, v23.8h
        smull2 v29.4s, v16.8h, v13.8h
        uzp2 v26.8h, v8.8h, v7.8h
        smlal2 v29.4s, v14.8h, v18.8h
        uzp1 v30.8h, v24.8h, v31.8h
        uzp1 v8.8h, v8.8h, v7.8h
        smull v11.4s, v16.4h, v18.4h
        smlal v11.4s, v14.4h, v27.4h
        ldur q1, [x2, #-0x10]
        uzp1 v28.8h, v6.8h, v23.8h
        smlal2 v29.4s, v8.8h, v5.8h
        ldr q25, [x5], #0x20
        smlal v19.4s, v8.4h, v5.4h
        ldr q3, [x4, #0x10]
        smlal2 v29.4s, v26.8h, v28.8h
        uzp1 v27.8h, v22.8h, v1.8h
        smlal v19.4s, v26.4h, v28.4h
        ldr q12, [x4], #0x20
        smlal2 v4.4s, v8.8h, v28.8h
        ldur q21, [x5, #-0x10]
        smlal2 v4.4s, v26.8h, v17.8h
        smlal v11.4s, v8.4h, v28.4h
        ld1 { v15.8h }, [x6], #16
        smlal v11.4s, v26.4h, v17.4h
        ld1 { v20.8h }, [x3], #16
        uzp1 v28.8h, v19.8h, v29.8h
        smull2 v23.4s, v30.8h, v27.8h
        smull v26.4s, v30.4h, v27.4h
        uzp2 v16.8h, v22.8h, v1.8h
        mul v28.8h, v28.8h, v2.8h
        uzp1 v10.8h, v11.8h, v4.8h
        smull2 v8.4s, v30.8h, v16.8h
        mul v13.8h, v10.8h, v2.8h
        smlal v19.4s, v28.4h, v0.4h
        smlal2 v29.4s, v28.8h, v0.8h
        smull v18.4s, v30.4h, v16.4h
        uzp1 v30.8h, v25.8h, v21.8h
        smlal v11.4s, v13.4h, v0.4h
        uzp2 v6.8h, v24.8h, v31.8h
        uzp1 v16.8h, v12.8h, v3.8h
        smlal2 v4.4s, v13.8h, v0.8h
        uzp2 v17.8h, v25.8h, v21.8h
        smlal2 v8.4s, v6.8h, v27.8h
        uzp2 v12.8h, v12.8h, v3.8h
        smlal v18.4s, v6.4h, v27.4h
        uzp2 v9.8h, v19.8h, v29.8h
        smlal2 v8.4s, v16.8h, v17.8h
        smlal2 v8.4s, v12.8h, v30.8h
        uzp2 v19.8h, v11.8h, v4.8h
        sub x13, x13, #0x2

Lpolyvec_basemul_acc_montgomery_cached_k2_loop_start:
        smlal v18.4s, v16.4h, v17.4h
        ldr q7, [x4], #0x20
        ldr q10, [x2, #0x10]
        smlal v18.4s, v12.4h, v30.4h
        smlal2 v23.4s, v6.8h, v20.8h
        ldr q14, [x2], #0x20
        smlal2 v23.4s, v16.8h, v30.8h
        zip1 v25.8h, v19.8h, v9.8h
        zip2 v3.8h, v19.8h, v9.8h
        smlal2 v23.4s, v12.8h, v15.8h
        smlal v26.4s, v6.4h, v20.4h
        uzp1 v5.8h, v18.8h, v8.8h
        uzp2 v21.8h, v14.8h, v10.8h
        smlal v26.4s, v16.4h, v30.4h
        str q25, [x0], #0x20
        mul v29.8h, v5.8h, v2.8h
        uzp1 v24.8h, v14.8h, v10.8h
        stur q3, [x0, #-0x10]
        smlal v26.4s, v12.4h, v15.4h
        ld1 { v15.8h }, [x6], #16
        ldr q28, [x1, #0x10]
        ldr q11, [x1], #0x20
        ldr q13, [x5], #0x20
        ldur q27, [x4, #-0x10]
        smlal2 v8.4s, v29.8h, v0.8h
        ldur q22, [x5, #-0x10]
        smlal v18.4s, v29.4h, v0.4h
        uzp1 v4.8h, v26.8h, v23.8h
        uzp1 v1.8h, v11.8h, v28.8h
        uzp2 v6.8h, v11.8h, v28.8h
        uzp1 v16.8h, v7.8h, v27.8h
        mul v31.8h, v4.8h, v2.8h
        uzp2 v17.8h, v13.8h, v22.8h
        ld1 { v20.8h }, [x3], #16
        uzp2 v9.8h, v18.8h, v8.8h
        smull2 v8.4s, v1.8h, v21.8h
        uzp1 v30.8h, v13.8h, v22.8h
        smlal2 v8.4s, v6.8h, v24.8h
        smlal2 v8.4s, v16.8h, v17.8h
        uzp2 v12.8h, v7.8h, v27.8h
        smlal v26.4s, v31.4h, v0.4h
        smlal2 v23.4s, v31.8h, v0.8h
        smull v18.4s, v1.4h, v21.4h
        smlal v18.4s, v6.4h, v24.4h
        smlal2 v8.4s, v12.8h, v30.8h
        uzp2 v19.8h, v26.8h, v23.8h
        smull2 v23.4s, v1.8h, v24.8h
        smull v26.4s, v1.4h, v24.4h
        subs x13, x13, #0x1
        cbnz x13, Lpolyvec_basemul_acc_montgomery_cached_k2_loop_start
        smlal v26.4s, v6.4h, v20.4h
        smlal2 v23.4s, v6.8h, v20.8h
        smlal v26.4s, v16.4h, v30.4h
        smlal2 v23.4s, v16.8h, v30.8h
        smlal v26.4s, v12.4h, v15.4h
        smlal2 v23.4s, v12.8h, v15.8h
        smlal v18.4s, v16.4h, v17.4h
        smlal v18.4s, v12.4h, v30.4h
        zip1 v12.8h, v19.8h, v9.8h
        str q12, [x0], #0x20
        uzp1 v12.8h, v26.8h, v23.8h
        mul v6.8h, v12.8h, v2.8h
        uzp1 v12.8h, v18.8h, v8.8h
        mul v12.8h, v12.8h, v2.8h
        smlal v26.4s, v6.4h, v0.4h
        smlal2 v23.4s, v6.8h, v0.8h
        smlal2 v8.4s, v12.8h, v0.8h
        smlal v18.4s, v12.4h, v0.4h
        zip2 v12.8h, v19.8h, v9.8h
        uzp2 v6.8h, v26.8h, v23.8h
        stur q12, [x0, #-0x10]
        uzp2 v12.8h, v18.8h, v8.8h
        zip2 v1.8h, v6.8h, v12.8h
        zip1 v12.8h, v6.8h, v12.8h
        str q1, [x0, #0x10]
        str q12, [x0], #0x20
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

S2N_BN_SIZE_DIRECTIVE(mlkem_polyvec_basemul_acc_montgomery_cached_asm_k2)


