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
  Name: ntt_asm
  Description: AArch64 ML-KEM forward NTT following @[NeonNTT] and @[SLOTHY_Paper]
  Signature: void mlk_ntt_asm(int16_t p[256], const int16_t twiddles12345[80], const int16_t twiddles56[384])
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
 *   dev/aarch64_opt/src/ntt.S using scripts/simpasm. Do not modify it directly.
 */

#if defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_ntt_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_ntt_asm)
S2N_BN_SYMBOL(mlkem_ntt_asm):

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
        mov x3, x0
        mov x4, #0x4                // =4
        ldr q0, [x1], #0x20
        ldur q1, [x1, #-0x10]
        ldr q21, [x0, #0x40]
        ldr q5, [x0, #0x1c0]
        ldr q30, [x0, #0x110]
        ldr q24, [x0, #0x140]
        ldr q12, [x0, #0x80]
        sqrdmulh v9.8h, v5.8h, v0.h[1]
        mul v23.8h, v5.8h, v0.h[0]
        sqrdmulh v17.8h, v24.8h, v0.h[1]
        ldr q13, [x0, #0xc0]
        mls v23.8h, v9.8h, v7.h[0]
        mul v8.8h, v24.8h, v0.h[0]
        mls v8.8h, v17.8h, v7.h[0]
        add v9.8h, v13.8h, v23.8h
        sub v10.8h, v13.8h, v23.8h
        mul v11.8h, v30.8h, v0.h[0]
        ldr q13, [x0, #0x180]
        sqrdmulh v28.8h, v9.8h, v0.h[3]
        sub v29.8h, v21.8h, v8.8h
        mul v26.8h, v9.8h, v0.h[2]
        add v8.8h, v21.8h, v8.8h
        mul v2.8h, v13.8h, v0.h[0]
        mls v26.8h, v28.8h, v7.h[0]
        mul v28.8h, v10.8h, v0.h[4]
        sqrdmulh v23.8h, v10.8h, v0.h[5]
        add v22.8h, v8.8h, v26.8h
        sqrdmulh v10.8h, v13.8h, v0.h[1]
        sqrdmulh v21.8h, v22.8h, v0.h[7]
        ldr q13, [x0, #0x100]
        mul v16.8h, v22.8h, v0.h[6]
        mls v28.8h, v23.8h, v7.h[0]
        mls v2.8h, v10.8h, v7.h[0]
        sqrdmulh v23.8h, v13.8h, v0.h[1]
        sub v10.8h, v29.8h, v28.8h
        add v17.8h, v29.8h, v28.8h
        mls v16.8h, v21.8h, v7.h[0]
        sub v18.8h, v12.8h, v2.8h
        ldr q29, [x0]
        sqrdmulh v14.8h, v17.8h, v1.h[3]
        add v22.8h, v12.8h, v2.8h
        sqrdmulh v9.8h, v18.8h, v0.h[5]
        mul v21.8h, v13.8h, v0.h[0]
        ldr q13, [x0, #0x150]
        mul v5.8h, v18.8h, v0.h[4]
        mls v5.8h, v9.8h, v7.h[0]
        mul v18.8h, v13.8h, v0.h[0]
        mls v21.8h, v23.8h, v7.h[0]
        sqrdmulh v2.8h, v13.8h, v0.h[1]
        mul v13.8h, v17.8h, v1.h[2]
        sub v4.8h, v29.8h, v21.8h
        mls v13.8h, v14.8h, v7.h[0]
        add v25.8h, v29.8h, v21.8h
        add v6.8h, v4.8h, v5.8h
        sqrdmulh v15.8h, v22.8h, v0.h[3]
        sub v21.8h, v4.8h, v5.8h
        sub v5.8h, v8.8h, v26.8h
        mul v23.8h, v22.8h, v0.h[2]
        add v28.8h, v6.8h, v13.8h
        sub v13.8h, v6.8h, v13.8h
        mul v4.8h, v5.8h, v1.h[0]
        sub x4, x4, #0x2

Lntt_layer123_start:
        mls v23.8h, v15.8h, v7.h[0]
        ldr q6, [x0, #0x190]
        ldr q15, [x0, #0x90]
        ldr q19, [x0, #0x10]
        mul v22.8h, v10.8h, v1.h[4]
        ldr q24, [x0, #0x50]
        str q13, [x0, #0x140]
        sqrdmulh v13.8h, v6.8h, v0.h[1]
        sub v20.8h, v25.8h, v23.8h
        sqrdmulh v3.8h, v30.8h, v0.h[1]
        str q28, [x0, #0x100]
        ldr q30, [x0, #0x120]
        mul v8.8h, v6.8h, v0.h[0]
        sqrdmulh v27.8h, v10.8h, v1.h[5]
        mls v11.8h, v3.8h, v7.h[0]
        mls v18.8h, v2.8h, v7.h[0]
        ldr q31, [x0, #0x160]
        sqrdmulh v10.8h, v5.8h, v1.h[1]
        mls v8.8h, v13.8h, v7.h[0]
        ldr q13, [x0, #0x1d0]
        sub v14.8h, v24.8h, v18.8h
        add v9.8h, v24.8h, v18.8h
        sqrdmulh v2.8h, v31.8h, v0.h[1]
        mls v4.8h, v10.8h, v7.h[0]
        add v10.8h, v25.8h, v23.8h
        sub v24.8h, v19.8h, v11.8h
        add v25.8h, v19.8h, v11.8h
        sqrdmulh v28.8h, v13.8h, v0.h[1]
        mul v11.8h, v30.8h, v0.h[0]
        mul v17.8h, v13.8h, v0.h[0]
        sub v13.8h, v10.8h, v16.8h
        sub v6.8h, v15.8h, v8.8h
        mls v17.8h, v28.8h, v7.h[0]
        str q13, [x0, #0x40]
        mls v22.8h, v27.8h, v7.h[0]
        ldr q13, [x0, #0xd0]
        add v26.8h, v20.8h, v4.8h
        mul v18.8h, v31.8h, v0.h[0]
        add v27.8h, v10.8h, v16.8h
        str q26, [x0, #0x80]
        sqrdmulh v31.8h, v6.8h, v0.h[5]
        add v3.8h, v21.8h, v22.8h
        str q27, [x0], #0x10
        mul v26.8h, v6.8h, v0.h[4]
        add v6.8h, v13.8h, v17.8h
        sub v5.8h, v13.8h, v17.8h
        str q3, [x0, #0x170]
        sub v17.8h, v21.8h, v22.8h
        sqrdmulh v10.8h, v6.8h, v0.h[3]
        sub v13.8h, v20.8h, v4.8h
        add v20.8h, v15.8h, v8.8h
        sqrdmulh v12.8h, v5.8h, v0.h[5]
        str q13, [x0, #0xb0]
        mul v8.8h, v6.8h, v0.h[2]
        str q17, [x0, #0x1b0]
        mls v8.8h, v10.8h, v7.h[0]
        mul v29.8h, v5.8h, v0.h[4]
        mls v29.8h, v12.8h, v7.h[0]
        sub v5.8h, v9.8h, v8.8h
        add v3.8h, v9.8h, v8.8h
        sqrdmulh v15.8h, v20.8h, v0.h[3]
        mul v4.8h, v5.8h, v1.h[0]
        add v6.8h, v14.8h, v29.8h
        sqrdmulh v9.8h, v3.8h, v0.h[7]
        sqrdmulh v12.8h, v6.8h, v1.h[3]
        sub v10.8h, v14.8h, v29.8h
        mul v23.8h, v6.8h, v1.h[2]
        mls v26.8h, v31.8h, v7.h[0]
        mls v23.8h, v12.8h, v7.h[0]
        mul v16.8h, v3.8h, v0.h[6]
        add v13.8h, v24.8h, v26.8h
        sub v21.8h, v24.8h, v26.8h
        mls v16.8h, v9.8h, v7.h[0]
        add v28.8h, v13.8h, v23.8h
        sub v13.8h, v13.8h, v23.8h
        mul v23.8h, v20.8h, v0.h[2]
        subs x4, x4, #0x1
        cbnz x4, Lntt_layer123_start
        sqrdmulh v3.8h, v5.8h, v1.h[1]
        mls v23.8h, v15.8h, v7.h[0]
        ldr q5, [x0, #0x190]
        mul v29.8h, v10.8h, v1.h[4]
        mls v4.8h, v3.8h, v7.h[0]
        sub v19.8h, v25.8h, v23.8h
        sqrdmulh v31.8h, v5.8h, v0.h[1]
        sqrdmulh v6.8h, v30.8h, v0.h[1]
        sub v3.8h, v19.8h, v4.8h
        mul v5.8h, v5.8h, v0.h[0]
        str q3, [x0, #0xc0]
        sqrdmulh v12.8h, v10.8h, v1.h[5]
        mls v18.8h, v2.8h, v7.h[0]
        ldr q3, [x0, #0x1d0]
        mls v5.8h, v31.8h, v7.h[0]
        sqrdmulh v10.8h, v3.8h, v0.h[1]
        mls v11.8h, v6.8h, v7.h[0]
        ldr q31, [x0, #0x90]
        mul v30.8h, v3.8h, v0.h[0]
        mls v30.8h, v10.8h, v7.h[0]
        sub v10.8h, v31.8h, v5.8h
        mls v29.8h, v12.8h, v7.h[0]
        ldr q6, [x0, #0xd0]
        sqrdmulh v15.8h, v10.8h, v0.h[5]
        mul v17.8h, v10.8h, v0.h[4]
        add v10.8h, v6.8h, v30.8h
        sub v6.8h, v6.8h, v30.8h
        sqrdmulh v12.8h, v10.8h, v0.h[3]
        sub v27.8h, v21.8h, v29.8h
        sqrdmulh v3.8h, v6.8h, v0.h[5]
        mul v10.8h, v10.8h, v0.h[2]
        ldr q20, [x0, #0x50]
        mls v10.8h, v12.8h, v7.h[0]
        mul v2.8h, v6.8h, v0.h[4]
        add v6.8h, v20.8h, v18.8h
        add v5.8h, v31.8h, v5.8h
        mls v2.8h, v3.8h, v7.h[0]
        sub v31.8h, v6.8h, v10.8h
        sqrdmulh v12.8h, v5.8h, v0.h[3]
        sub v22.8h, v20.8h, v18.8h
        add v6.8h, v6.8h, v10.8h
        mul v20.8h, v31.8h, v1.h[0]
        add v30.8h, v22.8h, v2.8h
        sqrdmulh v3.8h, v6.8h, v0.h[7]
        sqrdmulh v10.8h, v30.8h, v1.h[3]
        mul v9.8h, v30.8h, v1.h[2]
        ldr q30, [x0, #0x10]
        mls v17.8h, v15.8h, v7.h[0]
        mls v9.8h, v10.8h, v7.h[0]
        mul v15.8h, v6.8h, v0.h[6]
        add v24.8h, v30.8h, v11.8h
        sub v10.8h, v22.8h, v2.8h
        mls v15.8h, v3.8h, v7.h[0]
        add v6.8h, v19.8h, v4.8h
        add v22.8h, v25.8h, v23.8h
        sqrdmulh v3.8h, v10.8h, v1.h[5]
        str q13, [x0, #0x140]
        sub v19.8h, v30.8h, v11.8h
        add v25.8h, v22.8h, v16.8h
        mul v5.8h, v5.8h, v0.h[2]
        sub v13.8h, v22.8h, v16.8h
        str q28, [x0, #0x100]
        mls v5.8h, v12.8h, v7.h[0]
        str q13, [x0, #0x40]
        str q6, [x0, #0x80]
        add v21.8h, v21.8h, v29.8h
        sqrdmulh v13.8h, v31.8h, v1.h[1]
        str q25, [x0], #0x10
        add v12.8h, v19.8h, v17.8h
        sub v31.8h, v19.8h, v17.8h
        mul v30.8h, v10.8h, v1.h[4]
        str q21, [x0, #0x170]
        add v21.8h, v24.8h, v5.8h
        add v6.8h, v12.8h, v9.8h
        mls v30.8h, v3.8h, v7.h[0]
        str q27, [x0, #0x1b0]
        sub v10.8h, v21.8h, v15.8h
        sub v12.8h, v12.8h, v9.8h
        mls v20.8h, v13.8h, v7.h[0]
        str q6, [x0, #0x100]
        str q10, [x0, #0x40]
        sub v13.8h, v24.8h, v5.8h
        add v3.8h, v21.8h, v15.8h
        str q12, [x0, #0x140]
        sub v10.8h, v31.8h, v30.8h
        add v21.8h, v31.8h, v30.8h
        str q3, [x0], #0x10
        add v12.8h, v13.8h, v20.8h
        sub v13.8h, v13.8h, v20.8h
        str q21, [x0, #0x170]
        str q10, [x0, #0x1b0]
        str q12, [x0, #0x70]
        str q13, [x0, #0xb0]
        mov x0, x3
        mov x4, #0x8                // =8
        ldr q2, [x0, #0x20]
        ldr q13, [x1], #0x10
        ldr q30, [x0, #0x30]
        ldr q25, [x2, #0x40]
        ldr q5, [x0]
        ldr q18, [x0, #0x60]
        ldr q12, [x0, #0x70]
        sqrdmulh v17.8h, v2.8h, v13.h[1]
        ldr q4, [x1], #0x10
        ldr q23, [x0, #0x10]
        sqrdmulh v21.8h, v30.8h, v13.h[1]
        ldr q24, [x2, #0x20]
        ldr q9, [x2], #0x60
        mul v10.8h, v30.8h, v13.h[0]
        mul v11.8h, v2.8h, v13.h[0]
        mls v10.8h, v21.8h, v7.h[0]
        sqrdmulh v29.8h, v12.8h, v4.h[1]
        mul v1.8h, v12.8h, v4.h[0]
        add v21.8h, v23.8h, v10.8h
        sub v10.8h, v23.8h, v10.8h
        mul v8.8h, v18.8h, v4.h[0]
        sqrdmulh v23.8h, v21.8h, v13.h[3]
        mul v2.8h, v21.8h, v13.h[2]
        mls v1.8h, v29.8h, v7.h[0]
        mls v2.8h, v23.8h, v7.h[0]
        ldur q15, [x2, #-0x50]
        sqrdmulh v0.8h, v10.8h, v13.h[5]
        mls v11.8h, v17.8h, v7.h[0]
        ldr q29, [x0, #0x50]
        mul v23.8h, v10.8h, v13.h[4]
        mls v23.8h, v0.8h, v7.h[0]
        sub v16.8h, v29.8h, v1.8h
        add v3.8h, v5.8h, v11.8h
        sub v31.8h, v5.8h, v11.8h
        sqrdmulh v22.8h, v16.8h, v4.h[5]
        add v30.8h, v3.8h, v2.8h
        sub v0.8h, v3.8h, v2.8h
        sqrdmulh v28.8h, v18.8h, v4.h[1]
        add v21.8h, v31.8h, v23.8h
        sub v19.8h, v31.8h, v23.8h
        mul v26.8h, v16.8h, v4.h[4]
        trn2 v3.4s, v30.4s, v0.4s
        ldur q23, [x2, #-0x10]
        trn2 v18.4s, v21.4s, v19.4s
        mls v26.8h, v22.8h, v7.h[0]
        trn1 v13.4s, v30.4s, v0.4s
        mls v8.8h, v28.8h, v7.h[0]
        trn2 v31.2d, v3.2d, v18.2d
        trn1 v11.4s, v21.4s, v19.4s
        add v27.8h, v29.8h, v1.8h
        sqrdmulh v6.8h, v31.8h, v15.8h
        trn1 v2.2d, v13.2d, v11.2d
        trn2 v13.2d, v13.2d, v11.2d
        mul v1.8h, v31.8h, v9.8h
        ldr q11, [x0, #0x40]
        sqrdmulh v29.8h, v13.8h, v15.8h
        mls v1.8h, v6.8h, v7.h[0]
        trn1 v6.2d, v3.2d, v18.2d
        mul v17.8h, v13.8h, v9.8h
        sub v13.8h, v11.8h, v8.8h
        sqrdmulh v10.8h, v27.8h, v4.h[3]
        sub v12.8h, v13.8h, v26.8h
        sub v18.8h, v6.8h, v1.8h
        mls v17.8h, v29.8h, v7.h[0]
        add v30.8h, v6.8h, v1.8h
        add v6.8h, v13.8h, v26.8h
        ldur q13, [x2, #-0x30]
        sqrdmulh v16.8h, v18.8h, v23.8h
        trn1 v28.4s, v6.4s, v12.4s
        mul v23.8h, v18.8h, v25.8h
        ldr q25, [x2, #0x10]
        add v20.8h, v2.8h, v17.8h
        mul v0.8h, v30.8h, v24.8h
        sqrdmulh v29.8h, v30.8h, v13.8h
        sub v30.8h, v2.8h, v17.8h
        mls v23.8h, v16.8h, v7.h[0]
        sub x4, x4, #0x2

Lntt_layer4567_start:
        ldr q19, [x2, #0x50]
        sub v31.8h, v30.8h, v23.8h
        mls v0.8h, v29.8h, v7.h[0]
        add v16.8h, v11.8h, v8.8h
        ldr q18, [x0, #0xa0]
        trn2 v14.4s, v6.4s, v12.4s
        mul v26.8h, v27.8h, v4.h[2]
        ldr q4, [x1], #0x10
        ldr q24, [x2, #0x40]
        ldr q21, [x0, #0xb0]
        mls v26.8h, v10.8h, v7.h[0]
        add v23.8h, v30.8h, v23.8h
        sub v15.8h, v20.8h, v0.8h
        ldr q9, [x0, #0x90]
        add v10.8h, v20.8h, v0.8h
        mul v8.8h, v18.8h, v4.h[0]
        ldr q1, [x2], #0x60
        trn1 v27.4s, v23.4s, v31.4s
        sqrdmulh v12.8h, v18.8h, v4.h[1]
        trn1 v5.4s, v10.4s, v15.4s
        sub v30.8h, v16.8h, v26.8h
        trn2 v13.2d, v5.2d, v27.2d
        sqrdmulh v2.8h, v21.8h, v4.h[1]
        add v29.8h, v16.8h, v26.8h
        mul v0.8h, v21.8h, v4.h[0]
        str q13, [x0, #0x20]
        trn1 v11.4s, v29.4s, v30.4s
        mls v8.8h, v12.8h, v7.h[0]
        trn2 v26.4s, v29.4s, v30.4s
        trn2 v6.2d, v11.2d, v28.2d
        mls v0.8h, v2.8h, v7.h[0]
        trn2 v16.2d, v26.2d, v14.2d
        trn1 v26.2d, v26.2d, v14.2d
        trn1 v20.2d, v5.2d, v27.2d
        sqrdmulh v29.8h, v6.8h, v25.8h
        trn2 v15.4s, v10.4s, v15.4s
        sqrdmulh v13.8h, v16.8h, v25.8h
        str q20, [x0], #0x40
        sub v30.8h, v9.8h, v0.8h
        add v27.8h, v9.8h, v0.8h
        mul v17.8h, v6.8h, v1.8h
        sqrdmulh v22.8h, v30.8h, v4.h[5]
        mul v18.8h, v16.8h, v1.8h
        mls v18.8h, v13.8h, v7.h[0]
        mul v2.8h, v30.8h, v4.h[4]
        mls v2.8h, v22.8h, v7.h[0]
        trn2 v22.4s, v23.4s, v31.4s
        sub v3.8h, v26.8h, v18.8h
        ldur q25, [x2, #-0x30]
        mls v17.8h, v29.8h, v7.h[0]
        trn2 v31.2d, v15.2d, v22.2d
        trn1 v20.2d, v15.2d, v22.2d
        add v16.8h, v26.8h, v18.8h
        sqrdmulh v26.8h, v3.8h, v19.8h
        trn1 v21.2d, v11.2d, v28.2d
        ldr q11, [x0, #0x40]
        sqrdmulh v29.8h, v16.8h, v25.8h
        stur q20, [x0, #-0x30]
        add v20.8h, v21.8h, v17.8h
        stur q31, [x0, #-0x10]
        mul v23.8h, v3.8h, v24.8h
        ldr q25, [x2, #0x10]
        sub v13.8h, v11.8h, v8.8h
        mls v23.8h, v26.8h, v7.h[0]
        ldur q1, [x2, #-0x40]
        sub v12.8h, v13.8h, v2.8h
        add v6.8h, v13.8h, v2.8h
        sqrdmulh v10.8h, v27.8h, v4.h[3]
        sub v30.8h, v21.8h, v17.8h
        mul v0.8h, v16.8h, v1.8h
        trn1 v28.4s, v6.4s, v12.4s
        subs x4, x4, #0x1
        cbnz x4, Lntt_layer4567_start
        add v22.8h, v11.8h, v8.8h
        mul v27.8h, v27.8h, v4.h[2]
        trn2 v17.4s, v6.4s, v12.4s
        ldr q15, [x2], #0x60
        mls v27.8h, v10.8h, v7.h[0]
        add v4.8h, v30.8h, v23.8h
        sub v18.8h, v30.8h, v23.8h
        ldur q6, [x2, #-0x30]
        mls v0.8h, v29.8h, v7.h[0]
        ldur q12, [x2, #-0x40]
        ldur q24, [x2, #-0x20]
        ldur q2, [x2, #-0x10]
        trn1 v9.4s, v4.4s, v18.4s
        add v10.8h, v22.8h, v27.8h
        sub v13.8h, v22.8h, v27.8h
        sub v1.8h, v20.8h, v0.8h
        trn2 v21.4s, v10.4s, v13.4s
        add v27.8h, v20.8h, v0.8h
        trn2 v3.2d, v21.2d, v17.2d
        trn1 v13.4s, v10.4s, v13.4s
        trn1 v31.4s, v27.4s, v1.4s
        sqrdmulh v10.8h, v3.8h, v25.8h
        trn2 v5.2d, v13.2d, v28.2d
        trn1 v13.2d, v13.2d, v28.2d
        trn1 v21.2d, v21.2d, v17.2d
        sqrdmulh v17.8h, v5.8h, v25.8h
        trn2 v30.2d, v31.2d, v9.2d
        mul v25.8h, v3.8h, v15.8h
        str q30, [x0, #0x20]
        trn2 v30.4s, v4.4s, v18.4s
        mls v25.8h, v10.8h, v7.h[0]
        trn2 v3.4s, v27.4s, v1.4s
        mul v20.8h, v5.8h, v15.8h
        trn2 v10.2d, v3.2d, v30.2d
        mls v20.8h, v17.8h, v7.h[0]
        str q10, [x0, #0x30]
        sub v18.8h, v21.8h, v25.8h
        add v10.8h, v21.8h, v25.8h
        trn1 v3.2d, v3.2d, v30.2d
        sqrdmulh v30.8h, v18.8h, v2.8h
        mul v12.8h, v10.8h, v12.8h
        sqrdmulh v6.8h, v10.8h, v6.8h
        str q3, [x0, #0x10]
        add v21.8h, v13.8h, v20.8h
        mul v10.8h, v18.8h, v24.8h
        sub v13.8h, v13.8h, v20.8h
        mls v10.8h, v30.8h, v7.h[0]
        mls v12.8h, v6.8h, v7.h[0]
        trn1 v30.2d, v31.2d, v9.2d
        sub v3.8h, v13.8h, v10.8h
        add v6.8h, v13.8h, v10.8h
        add v10.8h, v21.8h, v12.8h
        sub v21.8h, v21.8h, v12.8h
        trn2 v13.4s, v6.4s, v3.4s
        trn1 v12.4s, v10.4s, v21.4s
        trn2 v21.4s, v10.4s, v21.4s
        trn1 v3.4s, v6.4s, v3.4s
        str q30, [x0], #0x40
        trn2 v10.2d, v21.2d, v13.2d
        trn1 v13.2d, v21.2d, v13.2d
        trn2 v21.2d, v12.2d, v3.2d
        trn1 v3.2d, v12.2d, v3.2d
        str q10, [x0, #0x30]
        str q13, [x0, #0x10]
        str q3, [x0], #0x40
        stur q21, [x0, #-0x20]
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

S2N_BN_SIZE_DIRECTIVE(mlkem_ntt_asm)

