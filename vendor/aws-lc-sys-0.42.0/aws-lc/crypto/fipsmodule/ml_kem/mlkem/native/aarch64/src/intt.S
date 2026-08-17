/* Copyright (c) 2022 Arm Limited
 * Copyright (c) 2022 Hanno Becker
 * Copyright (c) 2023 Amin Abdulrahman, Matthias Kannwischer
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
 *
 * - [SLOTHY_Paper]
 *   Fast and Clean: Auditable high-performance assembly via constraint solving
 *   Abdulrahman, Becker, Kannwischer, Klein
 *   https://eprint.iacr.org/2022/1303
 */

/*yaml
  Name: intt_asm
  Description: AArch64 ML-KEM inverse NTT following @[NeonNTT] and @[SLOTHY_Paper]
  Signature: void mlk_intt_asm(int16_t p[256], const int16_t twiddles12345[80], const int16_t twiddles56[384])
  ABI:
    x0:
      type: buffer
      size_bytes: 512
      permissions: read/write
      c_parameter: int16_t p[256]
      description: Input/output polynomial
    x1:
      type: buffer
      size_bytes: 160
      permissions: read-only
      c_parameter: const int16_t twiddles12345[80]
      description: Twiddle factors for layers 1-5
    x2:
      type: buffer
      size_bytes: 768
      permissions: read-only
      c_parameter: const int16_t twiddles56[384]
      description: Twiddle factors for layers 6-7
  Stack:
    bytes: 64
    description: saving callee-saved Neon registers
*/

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mlkem-native source file
 *   dev/aarch64_opt/src/intt.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_intt_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_intt_asm)
S2N_BN_SYMBOL(mlkem_intt_asm):

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
        mov w5, #0xd01              // =3329
        mov v7.h[0], w5
        mov w5, #0x4ebf             // =20159
        mov v7.h[1], w5
        mov w5, #0x200              // =512
        dup v29.8h, w5
        mov w5, #0x13b0             // =5040
        dup v30.8h, w5
        mov x3, x0
        mov x4, #0x8                // =8
        ldr q13, [x3, #0x20]
        ldr q8, [x3, #0x30]
        ldr q6, [x3]
        ldr q16, [x3, #0x10]
        ldr q4, [x3, #0x50]
        ldr q11, [x3, #0x40]
        ldr q3, [x3, #0x70]
        trn1 v23.4s, v13.4s, v8.4s
        ldr q0, [x3, #0x60]
        trn2 v19.4s, v6.4s, v16.4s
        trn2 v21.4s, v13.4s, v8.4s
        trn1 v6.4s, v6.4s, v16.4s
        ldr q24, [x2, #0x20]
        trn1 v10.2d, v19.2d, v21.2d
        ldr q16, [x2], #0x60
        trn1 v5.2d, v6.2d, v23.2d
        trn1 v28.4s, v0.4s, v3.4s
        trn2 v18.2d, v6.2d, v23.2d
        mul v31.8h, v10.8h, v29.8h
        trn2 v13.4s, v0.4s, v3.4s
        ldur q14, [x2, #-0x50]
        sqrdmulh v26.8h, v18.8h, v30.8h
        ldur q20, [x2, #-0x20]
        mul v17.8h, v18.8h, v29.8h
        trn2 v18.2d, v19.2d, v21.2d
        mul v9.8h, v18.8h, v29.8h
        trn1 v12.4s, v11.4s, v4.4s
        sqrdmulh v22.8h, v18.8h, v30.8h
        sqrdmulh v3.8h, v10.8h, v30.8h
        sqrdmulh v25.8h, v5.8h, v30.8h
        mls v9.8h, v22.8h, v7.h[0]
        mls v17.8h, v26.8h, v7.h[0]
        trn2 v26.4s, v11.4s, v4.4s
        mul v8.8h, v5.8h, v29.8h
        trn1 v10.2d, v26.2d, v13.2d
        ldur q11, [x2, #-0x10]
        mls v31.8h, v3.8h, v7.h[0]
        trn1 v6.2d, v12.2d, v28.2d
        trn2 v3.2d, v26.2d, v13.2d
        ldur q4, [x2, #-0x30]
        mls v8.8h, v25.8h, v7.h[0]
        sub v19.8h, v17.8h, v9.8h
        trn2 v13.2d, v12.2d, v28.2d
        sqrdmulh v1.8h, v3.8h, v30.8h
        add v9.8h, v17.8h, v9.8h
        mul v18.8h, v19.8h, v20.8h
        add v28.8h, v8.8h, v31.8h
        sqrdmulh v20.8h, v19.8h, v11.8h
        sub v12.8h, v28.8h, v9.8h
        sub v23.8h, v8.8h, v31.8h
        sqrdmulh v11.8h, v13.8h, v30.8h
        sqrdmulh v5.8h, v23.8h, v4.8h
        mul v0.8h, v23.8h, v24.8h
        mul v2.8h, v13.8h, v29.8h
        mls v0.8h, v5.8h, v7.h[0]
        add v24.8h, v28.8h, v9.8h
        mls v18.8h, v20.8h, v7.h[0]
        sqrdmulh v15.8h, v6.8h, v30.8h
        sqrdmulh v25.8h, v12.8h, v14.8h
        mul v21.8h, v12.8h, v16.8h
        sub v23.8h, v0.8h, v18.8h
        sqrdmulh v8.8h, v23.8h, v14.8h
        mul v23.8h, v23.8h, v16.8h
        mls v21.8h, v25.8h, v7.h[0]
        mls v23.8h, v8.8h, v7.h[0]
        mul v14.8h, v3.8h, v29.8h
        add v3.8h, v0.8h, v18.8h
        trn2 v4.4s, v24.4s, v3.4s
        mls v14.8h, v1.8h, v7.h[0]
        trn1 v9.4s, v24.4s, v3.4s
        trn2 v12.4s, v21.4s, v23.4s
        mls v2.8h, v11.8h, v7.h[0]
        trn1 v28.4s, v21.4s, v23.4s
        ldr q11, [x1], #0x10
        mul v31.8h, v10.8h, v29.8h
        trn1 v25.2d, v4.2d, v12.2d
        trn1 v20.2d, v9.2d, v28.2d
        ldr q23, [x2, #0x50]
        trn2 v13.2d, v4.2d, v12.2d
        sqrdmulh v21.8h, v10.8h, v30.8h
        trn2 v4.2d, v9.2d, v28.2d
        ldr q9, [x2, #0x40]
        mul v27.8h, v6.8h, v29.8h
        add v26.8h, v20.8h, v25.8h
        sub v3.8h, v2.8h, v14.8h
        sqdmulh v12.8h, v26.8h, v7.h[1]
        add v5.8h, v4.8h, v13.8h
        sub v8.8h, v4.8h, v13.8h
        add v10.8h, v2.8h, v14.8h
        sqdmulh v6.8h, v5.8h, v7.h[1]
        ldr q2, [x2, #0x10]
        mls v27.8h, v15.8h, v7.h[0]
        ldr q15, [x2, #0x20]
        srshr v12.8h, v12.8h, #0xb
        mls v31.8h, v21.8h, v7.h[0]
        srshr v6.8h, v6.8h, #0xb
        sqrdmulh v23.8h, v3.8h, v23.8h
        mls v26.8h, v12.8h, v7.h[0]
        add v21.8h, v27.8h, v31.8h
        mls v5.8h, v6.8h, v7.h[0]
        sub v6.8h, v27.8h, v31.8h
        sub v14.8h, v21.8h, v10.8h
        ldr q27, [x2], #0x60
        mul v3.8h, v3.8h, v9.8h
        mls v3.8h, v23.8h, v7.h[0]
        ldur q13, [x2, #-0x30]
        sub v12.8h, v26.8h, v5.8h
        add v5.8h, v26.8h, v5.8h
        sqrdmulh v31.8h, v8.8h, v11.h[5]
        sqrdmulh v19.8h, v12.8h, v11.h[1]
        mul v24.8h, v12.8h, v11.h[0]
        sqrdmulh v13.8h, v6.8h, v13.8h
        mls v24.8h, v19.8h, v7.h[0]
        sub x4, x4, #0x2

Lintt_layer4567_start:
        add v16.8h, v21.8h, v10.8h
        mul v18.8h, v6.8h, v15.8h
        sub v19.8h, v20.8h, v25.8h
        ldr q21, [x3, #0xa0]
        str q5, [x3], #0x40
        mls v18.8h, v13.8h, v7.h[0]
        sqrdmulh v15.8h, v14.8h, v2.8h
        ldr q10, [x3, #0x50]
        ldr q12, [x3, #0x40]
        stur q24, [x3, #-0x20]
        mul v5.8h, v8.8h, v11.h[4]
        sub v0.8h, v18.8h, v3.8h
        ldr q24, [x3, #0x70]
        mls v5.8h, v31.8h, v7.h[0]
        ldr q26, [x2, #0x50]
        trn2 v1.4s, v12.4s, v10.4s
        add v6.8h, v18.8h, v3.8h
        sqrdmulh v20.8h, v0.8h, v2.8h
        trn1 v13.4s, v12.4s, v10.4s
        trn1 v18.4s, v16.4s, v6.4s
        mul v22.8h, v0.8h, v27.8h
        trn1 v17.4s, v21.4s, v24.4s
        sqrdmulh v0.8h, v19.8h, v11.h[3]
        trn1 v25.2d, v13.2d, v17.2d
        mls v22.8h, v20.8h, v7.h[0]
        trn2 v21.4s, v21.4s, v24.4s
        mul v24.8h, v25.8h, v29.8h
        trn2 v28.2d, v13.2d, v17.2d
        sqrdmulh v4.8h, v25.8h, v30.8h
        trn2 v3.2d, v1.2d, v21.2d
        mul v17.8h, v28.8h, v29.8h
        sqrdmulh v31.8h, v28.8h, v30.8h
        ldr q2, [x2, #0x10]
        mls v24.8h, v4.8h, v7.h[0]
        mul v4.8h, v19.8h, v11.h[2]
        ldr q19, [x2, #0x40]
        mls v4.8h, v0.8h, v7.h[0]
        mul v0.8h, v14.8h, v27.8h
        mls v0.8h, v15.8h, v7.h[0]
        sub v8.8h, v4.8h, v5.8h
        mul v12.8h, v3.8h, v29.8h
        mul v23.8h, v8.8h, v11.h[0]
        trn2 v28.4s, v16.4s, v6.4s
        sqrdmulh v10.8h, v8.8h, v11.h[1]
        trn1 v9.4s, v0.4s, v22.4s
        trn2 v22.4s, v0.4s, v22.4s
        ldr q11, [x1], #0x10
        mls v17.8h, v31.8h, v7.h[0]
        trn1 v20.2d, v18.2d, v9.2d
        trn2 v14.2d, v18.2d, v9.2d
        ldr q15, [x2, #0x20]
        trn1 v6.2d, v1.2d, v21.2d
        sqrdmulh v9.8h, v3.8h, v30.8h
        trn1 v25.2d, v28.2d, v22.2d
        trn2 v16.2d, v28.2d, v22.2d
        mls v23.8h, v10.8h, v7.h[0]
        add v1.8h, v20.8h, v25.8h
        sqrdmulh v21.8h, v6.8h, v30.8h
        add v8.8h, v14.8h, v16.8h
        ldr q27, [x2], #0x60
        sqdmulh v28.8h, v8.8h, v7.h[1]
        mls v12.8h, v9.8h, v7.h[0]
        sqdmulh v31.8h, v1.8h, v7.h[1]
        mul v0.8h, v6.8h, v29.8h
        sub v10.8h, v17.8h, v12.8h
        mls v0.8h, v21.8h, v7.h[0]
        srshr v21.8h, v28.8h, #0xb
        srshr v13.8h, v31.8h, #0xb
        sqrdmulh v22.8h, v10.8h, v26.8h
        mls v8.8h, v21.8h, v7.h[0]
        mls v1.8h, v13.8h, v7.h[0]
        add v21.8h, v24.8h, v0.8h
        stur q23, [x3, #-0x10]
        sub v6.8h, v24.8h, v0.8h
        mul v3.8h, v10.8h, v19.8h
        add v0.8h, v4.8h, v5.8h
        sqdmulh v13.8h, v0.8h, v7.h[1]
        ldur q10, [x2, #-0x30]
        add v5.8h, v1.8h, v8.8h
        mls v3.8h, v22.8h, v7.h[0]
        sub v8.8h, v1.8h, v8.8h
        mul v24.8h, v8.8h, v11.h[0]
        sqrdmulh v8.8h, v8.8h, v11.h[1]
        srshr v1.8h, v13.8h, #0xb
        sqrdmulh v13.8h, v6.8h, v10.8h
        mls v0.8h, v1.8h, v7.h[0]
        add v10.8h, v17.8h, v12.8h
        mls v24.8h, v8.8h, v7.h[0]
        sub v8.8h, v14.8h, v16.8h
        sqrdmulh v31.8h, v8.8h, v11.h[5]
        sub v14.8h, v21.8h, v10.8h
        stur q0, [x3, #-0x30]
        subs x4, x4, #0x1
        cbnz x4, Lintt_layer4567_start
        mul v15.8h, v6.8h, v15.8h
        sub v22.8h, v20.8h, v25.8h
        add v4.8h, v21.8h, v10.8h
        str q24, [x3, #0x20]
        mls v15.8h, v13.8h, v7.h[0]
        str q5, [x3], #0x40
        ldr q9, [x1], #0x10
        sqrdmulh v28.8h, v14.8h, v2.8h
        mul v16.8h, v14.8h, v27.8h
        sub v18.8h, v15.8h, v3.8h
        add v15.8h, v15.8h, v3.8h
        sqrdmulh v0.8h, v18.8h, v2.8h
        trn2 v24.4s, v4.4s, v15.4s
        trn1 v2.4s, v4.4s, v15.4s
        mul v18.8h, v18.8h, v27.8h
        mls v16.8h, v28.8h, v7.h[0]
        mls v18.8h, v0.8h, v7.h[0]
        mul v23.8h, v8.8h, v11.h[4]
        sqrdmulh v12.8h, v22.8h, v11.h[3]
        trn1 v17.4s, v16.4s, v18.4s
        trn2 v4.4s, v16.4s, v18.4s
        mls v23.8h, v31.8h, v7.h[0]
        trn2 v3.2d, v2.2d, v17.2d
        trn2 v6.2d, v24.2d, v4.2d
        mul v26.8h, v22.8h, v11.h[2]
        trn1 v28.2d, v2.2d, v17.2d
        mls v26.8h, v12.8h, v7.h[0]
        add v25.8h, v3.8h, v6.8h
        sub v18.8h, v3.8h, v6.8h
        trn1 v24.2d, v24.2d, v4.2d
        sqdmulh v1.8h, v25.8h, v7.h[1]
        sub v27.8h, v28.8h, v24.8h
        sqrdmulh v2.8h, v18.8h, v9.h[5]
        add v28.8h, v28.8h, v24.8h
        mul v24.8h, v27.8h, v9.h[2]
        sqdmulh v12.8h, v28.8h, v7.h[1]
        mul v20.8h, v18.8h, v9.h[4]
        mls v20.8h, v2.8h, v7.h[0]
        srshr v1.8h, v1.8h, #0xb
        sqrdmulh v19.8h, v27.8h, v9.h[3]
        srshr v15.8h, v12.8h, #0xb
        mls v25.8h, v1.8h, v7.h[0]
        add v8.8h, v26.8h, v23.8h
        sub v4.8h, v26.8h, v23.8h
        mls v28.8h, v15.8h, v7.h[0]
        mls v24.8h, v19.8h, v7.h[0]
        mul v2.8h, v4.8h, v11.h[0]
        sub v19.8h, v28.8h, v25.8h
        sqrdmulh v15.8h, v4.8h, v11.h[1]
        add v25.8h, v28.8h, v25.8h
        sub v10.8h, v24.8h, v20.8h
        str q25, [x3], #0x40
        sqrdmulh v22.8h, v19.8h, v9.h[1]
        add v28.8h, v24.8h, v20.8h
        sqrdmulh v25.8h, v10.8h, v9.h[1]
        mul v27.8h, v19.8h, v9.h[0]
        mul v26.8h, v10.8h, v9.h[0]
        sqdmulh v20.8h, v28.8h, v7.h[1]
        sqdmulh v16.8h, v8.8h, v7.h[1]
        mls v26.8h, v25.8h, v7.h[0]
        mls v2.8h, v15.8h, v7.h[0]
        srshr v15.8h, v20.8h, #0xb
        srshr v1.8h, v16.8h, #0xb
        mls v27.8h, v22.8h, v7.h[0]
        mls v28.8h, v15.8h, v7.h[0]
        mls v8.8h, v1.8h, v7.h[0]
        stur q27, [x3, #-0x20]
        stur q2, [x3, #-0x50]
        stur q28, [x3, #-0x30]
        stur q26, [x3, #-0x10]
        stur q8, [x3, #-0x70]
        mov x4, #0x4                // =4
        ldr q0, [x1], #0x20
        ldur q1, [x1, #-0x10]
        ldr q26, [x0]
        ldr q13, [x0, #0x40]
        ldr q28, [x0, #0xc0]
        ldr q2, [x0, #0x140]
        ldr q6, [x0, #0x80]
        ldr q9, [x0, #0x100]
        ldr q29, [x0, #0x1c0]
        ldr q23, [x0, #0x180]
        sub v17.8h, v26.8h, v13.8h
        add v4.8h, v26.8h, v13.8h
        ldr q25, [x0, #0xd0]
        ldr q24, [x0, #0x50]
        add v5.8h, v6.8h, v28.8h
        mul v19.8h, v17.8h, v0.h[6]
        sub v10.8h, v6.8h, v28.8h
        ldr q30, [x0, #0x150]
        sqrdmulh v12.8h, v17.8h, v0.h[7]
        add v17.8h, v9.8h, v2.8h
        sub v28.8h, v9.8h, v2.8h
        ldr q2, [x0, #0x90]
        sub v26.8h, v23.8h, v29.8h
        sqrdmulh v31.8h, v10.8h, v1.h[1]
        add v22.8h, v23.8h, v29.8h
        ldr q3, [x0, #0x110]
        sqrdmulh v9.8h, v28.8h, v1.h[3]
        sub v20.8h, v4.8h, v5.8h
        sub v27.8h, v17.8h, v22.8h
        ldr q29, [x0, #0x10]
        add v16.8h, v4.8h, v5.8h
        sqrdmulh v4.8h, v26.8h, v1.h[5]
        add v6.8h, v17.8h, v22.8h
        ldr q22, [x0, #0x1d0]
        mul v8.8h, v28.8h, v1.h[2]
        sub v21.8h, v2.8h, v25.8h
        sub v5.8h, v16.8h, v6.8h
        mul v17.8h, v26.8h, v1.h[4]
        mul v26.8h, v10.8h, v1.h[0]
        mls v26.8h, v31.8h, v7.h[0]
        mls v17.8h, v4.8h, v7.h[0]
        mls v19.8h, v12.8h, v7.h[0]
        mls v8.8h, v9.8h, v7.h[0]
        sqrdmulh v10.8h, v27.8h, v0.h[5]
        sub v12.8h, v19.8h, v26.8h
        add v9.8h, v19.8h, v26.8h
        sqrdmulh v26.8h, v20.8h, v0.h[3]
        sub v11.8h, v8.8h, v17.8h
        add v14.8h, v8.8h, v17.8h
        sqrdmulh v13.8h, v12.8h, v0.h[3]
        add v23.8h, v9.8h, v14.8h
        sqrdmulh v28.8h, v11.8h, v0.h[5]
        sub v19.8h, v9.8h, v14.8h
        mul v17.8h, v27.8h, v0.h[4]
        str q23, [x0, #0x40]
        mul v14.8h, v20.8h, v0.h[2]
        mul v8.8h, v11.8h, v0.h[4]
        mul v4.8h, v12.8h, v0.h[2]
        mls v14.8h, v26.8h, v7.h[0]
        mls v17.8h, v10.8h, v7.h[0]
        mls v8.8h, v28.8h, v7.h[0]
        mls v4.8h, v13.8h, v7.h[0]
        sub v10.8h, v14.8h, v17.8h
        add v20.8h, v14.8h, v17.8h
        sqrdmulh v28.8h, v5.8h, v0.h[1]
        mul v18.8h, v5.8h, v0.h[0]
        str q20, [x0, #0x80]
        sub v13.8h, v4.8h, v8.8h
        mul v23.8h, v10.8h, v0.h[0]
        mul v17.8h, v19.8h, v0.h[0]
        sqrdmulh v9.8h, v13.8h, v0.h[1]
        mls v18.8h, v28.8h, v7.h[0]
        sqrdmulh v10.8h, v10.8h, v0.h[1]
        sub x4, x4, #0x2

Lintt_layer123_start:
        sub v12.8h, v3.8h, v30.8h
        mul v11.8h, v21.8h, v1.h[0]
        add v28.8h, v4.8h, v8.8h
        ldr q20, [x0, #0x190]
        add v27.8h, v16.8h, v6.8h
        sqrdmulh v8.8h, v12.8h, v1.h[3]
        add v16.8h, v29.8h, v24.8h
        str q28, [x0, #0xc0]
        mls v23.8h, v10.8h, v7.h[0]
        str q27, [x0], #0x10
        add v15.8h, v20.8h, v22.8h
        str q18, [x0, #0xf0]
        mul v14.8h, v13.8h, v0.h[0]
        add v2.8h, v2.8h, v25.8h
        sub v26.8h, v20.8h, v22.8h
        mul v4.8h, v12.8h, v1.h[2]
        sub v5.8h, v16.8h, v2.8h
        str q23, [x0, #0x170]
        add v20.8h, v3.8h, v30.8h
        sqrdmulh v27.8h, v26.8h, v1.h[5]
        add v16.8h, v16.8h, v2.8h
        mul v18.8h, v26.8h, v1.h[4]
        sub v31.8h, v20.8h, v15.8h
        mls v4.8h, v8.8h, v7.h[0]
        sub v28.8h, v29.8h, v24.8h
        mls v18.8h, v27.8h, v7.h[0]
        ldr q22, [x0, #0x1d0]
        mul v26.8h, v28.8h, v0.h[6]
        mul v2.8h, v5.8h, v0.h[2]
        sub v12.8h, v4.8h, v18.8h
        sqrdmulh v24.8h, v28.8h, v0.h[7]
        mls v14.8h, v9.8h, v7.h[0]
        sqrdmulh v10.8h, v12.8h, v0.h[5]
        mls v26.8h, v24.8h, v7.h[0]
        ldr q24, [x0, #0x50]
        mul v8.8h, v12.8h, v0.h[4]
        str q14, [x0, #0x1b0]
        add v28.8h, v4.8h, v18.8h
        sqrdmulh v5.8h, v5.8h, v0.h[3]
        add v6.8h, v20.8h, v15.8h
        sqrdmulh v3.8h, v19.8h, v0.h[1]
        sub v13.8h, v16.8h, v6.8h
        sqrdmulh v12.8h, v21.8h, v1.h[1]
        sqrdmulh v21.8h, v13.8h, v0.h[1]
        sqrdmulh v27.8h, v31.8h, v0.h[5]
        ldr q25, [x0, #0xd0]
        mls v11.8h, v12.8h, v7.h[0]
        mul v23.8h, v31.8h, v0.h[4]
        mul v18.8h, v13.8h, v0.h[0]
        add v30.8h, v26.8h, v11.8h
        sub v13.8h, v26.8h, v11.8h
        mls v23.8h, v27.8h, v7.h[0]
        add v12.8h, v30.8h, v28.8h
        sub v19.8h, v30.8h, v28.8h
        mls v2.8h, v5.8h, v7.h[0]
        str q12, [x0, #0x40]
        sqrdmulh v26.8h, v13.8h, v0.h[3]
        mls v8.8h, v10.8h, v7.h[0]
        ldr q30, [x0, #0x150]
        sub v20.8h, v2.8h, v23.8h
        mul v4.8h, v13.8h, v0.h[2]
        add v13.8h, v2.8h, v23.8h
        mls v4.8h, v26.8h, v7.h[0]
        ldr q2, [x0, #0x90]
        mul v23.8h, v20.8h, v0.h[0]
        ldr q29, [x0, #0x10]
        sqrdmulh v10.8h, v20.8h, v0.h[1]
        str q13, [x0, #0x80]
        sub v13.8h, v4.8h, v8.8h
        mls v17.8h, v3.8h, v7.h[0]
        ldr q3, [x0, #0x110]
        mls v18.8h, v21.8h, v7.h[0]
        sub v21.8h, v2.8h, v25.8h
        sqrdmulh v9.8h, v13.8h, v0.h[1]
        str q17, [x0, #0x130]
        mul v17.8h, v19.8h, v0.h[0]
        subs x4, x4, #0x1
        cbnz x4, Lintt_layer123_start
        mls v23.8h, v10.8h, v7.h[0]
        ldr q11, [x0, #0x190]
        str q18, [x0, #0x100]
        add v27.8h, v3.8h, v30.8h
        mul v13.8h, v13.8h, v0.h[0]
        sub v5.8h, v29.8h, v24.8h
        add v14.8h, v16.8h, v6.8h
        mls v13.8h, v9.8h, v7.h[0]
        add v10.8h, v11.8h, v22.8h
        str q23, [x0, #0x180]
        sub v20.8h, v11.8h, v22.8h
        sub v23.8h, v27.8h, v10.8h
        sqrdmulh v16.8h, v21.8h, v1.h[1]
        sqrdmulh v31.8h, v23.8h, v0.h[5]
        str q13, [x0, #0x1c0]
        add v13.8h, v4.8h, v8.8h
        mul v18.8h, v21.8h, v1.h[0]
        str q13, [x0, #0xc0]
        sqrdmulh v13.8h, v19.8h, v0.h[1]
        sqrdmulh v28.8h, v20.8h, v1.h[5]
        str q14, [x0], #0x10
        mul v4.8h, v20.8h, v1.h[4]
        mls v17.8h, v13.8h, v7.h[0]
        sub v13.8h, v3.8h, v30.8h
        sqrdmulh v8.8h, v13.8h, v1.h[3]
        mul v12.8h, v13.8h, v1.h[2]
        mls v4.8h, v28.8h, v7.h[0]
        mls v12.8h, v8.8h, v7.h[0]
        mls v18.8h, v16.8h, v7.h[0]
        str q17, [x0, #0x130]
        sqrdmulh v15.8h, v5.8h, v0.h[7]
        add v11.8h, v27.8h, v10.8h
        mul v16.8h, v5.8h, v0.h[6]
        sub v8.8h, v12.8h, v4.8h
        sqrdmulh v28.8h, v8.8h, v0.h[5]
        add v13.8h, v2.8h, v25.8h
        mls v16.8h, v15.8h, v7.h[0]
        add v26.8h, v12.8h, v4.8h
        mul v8.8h, v8.8h, v0.h[4]
        add v4.8h, v29.8h, v24.8h
        mls v8.8h, v28.8h, v7.h[0]
        sub v20.8h, v4.8h, v13.8h
        add v14.8h, v4.8h, v13.8h
        add v12.8h, v16.8h, v18.8h
        sqrdmulh v22.8h, v20.8h, v0.h[3]
        add v27.8h, v14.8h, v11.8h
        sub v13.8h, v16.8h, v18.8h
        mul v4.8h, v20.8h, v0.h[2]
        str q27, [x0], #0x10
        sub v24.8h, v12.8h, v26.8h
        sqrdmulh v3.8h, v13.8h, v0.h[3]
        mul v13.8h, v13.8h, v0.h[2]
        sqrdmulh v27.8h, v24.8h, v0.h[1]
        mls v13.8h, v3.8h, v7.h[0]
        mul v9.8h, v24.8h, v0.h[0]
        mls v9.8h, v27.8h, v7.h[0]
        add v30.8h, v13.8h, v8.8h
        sub v13.8h, v13.8h, v8.8h
        mls v4.8h, v22.8h, v7.h[0]
        str q30, [x0, #0xb0]
        sqrdmulh v16.8h, v13.8h, v0.h[1]
        str q9, [x0, #0x130]
        mul v9.8h, v13.8h, v0.h[0]
        add v13.8h, v12.8h, v26.8h
        str q13, [x0, #0x30]
        mul v13.8h, v23.8h, v0.h[4]
        sub v23.8h, v14.8h, v11.8h
        mls v13.8h, v31.8h, v7.h[0]
        mls v9.8h, v16.8h, v7.h[0]
        mul v30.8h, v23.8h, v0.h[0]
        sub v24.8h, v4.8h, v13.8h
        add v13.8h, v4.8h, v13.8h
        sqrdmulh v23.8h, v23.8h, v0.h[1]
        str q9, [x0, #0x1b0]
        str q13, [x0, #0x70]
        sqrdmulh v13.8h, v24.8h, v0.h[1]
        mul v21.8h, v24.8h, v0.h[0]
        mls v30.8h, v23.8h, v7.h[0]
        mls v21.8h, v13.8h, v7.h[0]
        str q30, [x0, #0xf0]
        str q21, [x0, #0x170]
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

S2N_BN_SIZE_DIRECTIVE(mlkem_intt_asm)

