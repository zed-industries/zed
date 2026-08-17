/* Copyright (c) 2022 Arm Limited
 * Copyright (c) 2022 Hanno Becker
 * Copyright (c) 2023 Amin Abdulrahman, Matthias Kannwischer
 * Copyright (c) The mlkem-native project authors
 * Copyright (c) The mldsa-native project authors
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

/* AArch64 ML-DSA forward NTT following @[NeonNTT] and @[SLOTHY_Paper] */

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/ntt_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_ntt_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_ntt_aarch64_asm)
S2N_BN_SYMBOL(mldsa_ntt_aarch64_asm):

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
        mov w5, #0xe001             // =57345
        movk w5, #0x7f, lsl #16
        dup v7.4s, w5
        mov x3, x0
        mov x4, #0x8                // =8
        ldr q0, [x1], #0x40
        ldur q1, [x1, #-0x30]
        ldur q2, [x1, #-0x20]
        ldur q3, [x1, #-0x10]
        ldr q23, [x0, #0x390]
        ldr q13, [x0, #0x380]
        ldr q22, [x0, #0x80]
        ldr q26, [x0, #0x190]
        ldr q8, [x0, #0x280]
        ldr q6, [x0, #0x210]
        mul v10.4s, v13.4s, v0.s[0]
        sqrdmulh v13.4s, v13.4s, v0.s[1]
        mul v12.4s, v8.4s, v0.s[0]
        sqrdmulh v27.4s, v8.4s, v0.s[1]
        mul v4.4s, v6.4s, v0.s[0]
        mls v10.4s, v13.4s, v7.s[0]
        ldr q13, [x0, #0x180]
        sqrdmulh v14.4s, v23.4s, v0.s[1]
        mls v12.4s, v27.4s, v7.s[0]
        add v31.4s, v13.4s, v10.4s
        sub v13.4s, v13.4s, v10.4s
        mul v10.4s, v23.4s, v0.s[0]
        sqrdmulh v8.4s, v13.4s, v1.s[1]
        sub v18.4s, v22.4s, v12.4s
        mls v10.4s, v14.4s, v7.s[0]
        mul v13.4s, v13.4s, v1.s[0]
        mls v13.4s, v8.4s, v7.s[0]
        sub v29.4s, v26.4s, v10.4s
        add v25.4s, v26.4s, v10.4s
        mul v10.4s, v31.4s, v0.s[2]
        mul v14.4s, v25.4s, v0.s[2]
        add v17.4s, v18.4s, v13.4s
        sub v15.4s, v18.4s, v13.4s
        sqrdmulh v13.4s, v31.4s, v0.s[3]
        sqrdmulh v20.4s, v15.4s, v3.s[1]
        sqrdmulh v5.4s, v17.4s, v2.s[3]
        mls v10.4s, v13.4s, v7.s[0]
        ldr q13, [x0, #0x300]
        mul v18.4s, v17.4s, v2.s[2]
        add v31.4s, v22.4s, v12.4s
        mul v23.4s, v15.4s, v3.s[0]
        ldr q17, [x0, #0x90]
        add v19.4s, v31.4s, v10.4s
        sub v16.4s, v31.4s, v10.4s
        mul v10.4s, v13.4s, v0.s[0]
        sqrdmulh v13.4s, v13.4s, v0.s[1]
        sqrdmulh v27.4s, v16.4s, v2.s[1]
        mul v11.4s, v16.4s, v2.s[0]
        mls v10.4s, v13.4s, v7.s[0]
        ldr q13, [x0, #0x290]
        ldr q22, [x0, #0x100]
        mls v11.4s, v27.4s, v7.s[0]
        sqrdmulh v15.4s, v13.4s, v0.s[1]
        sub v12.4s, v22.4s, v10.4s
        add v30.4s, v22.4s, v10.4s
        mul v10.4s, v13.4s, v0.s[0]
        ldr q28, [x0]
        sqrdmulh v13.4s, v25.4s, v0.s[3]
        sqrdmulh v27.4s, v30.4s, v0.s[3]
        mls v10.4s, v15.4s, v7.s[0]
        mls v14.4s, v13.4s, v7.s[0]
        ldr q13, [x0, #0x200]
        sqrdmulh v25.4s, v12.4s, v1.s[1]
        add v24.4s, v17.4s, v10.4s
        sub v21.4s, v17.4s, v10.4s
        sqrdmulh v8.4s, v13.4s, v0.s[1]
        sub v9.4s, v24.4s, v14.4s
        mul v26.4s, v12.4s, v1.s[0]
        mul v13.4s, v13.4s, v0.s[0]
        mls v13.4s, v8.4s, v7.s[0]
        mul v8.4s, v30.4s, v0.s[2]
        mls v8.4s, v27.4s, v7.s[0]
        add v16.4s, v28.4s, v13.4s
        sub v10.4s, v28.4s, v13.4s
        mls v26.4s, v25.4s, v7.s[0]
        sqrdmulh v12.4s, v19.4s, v1.s[3]
        sub v25.4s, v16.4s, v8.4s
        mls v23.4s, v20.4s, v7.s[0]
        sub v22.4s, v25.4s, v11.4s
        sqrdmulh v20.4s, v9.4s, v2.s[1]
        sub v15.4s, v10.4s, v26.4s
        sub x4, x4, #0x2

Lmldsa_ntt_layer123_start:
        add v31.4s, v10.4s, v26.4s
        mul v17.4s, v19.4s, v1.s[2]
        add v26.4s, v15.4s, v23.4s
        ldr q30, [x0, #0x2a0]
        sub v13.4s, v15.4s, v23.4s
        mul v23.4s, v29.4s, v1.s[0]
        add v25.4s, v25.4s, v11.4s
        str q22, [x0, #0x180]
        mul v11.4s, v9.4s, v2.s[0]
        str q13, [x0, #0x380]
        ldr q28, [x0, #0x10]
        add v10.4s, v16.4s, v8.4s
        mls v17.4s, v12.4s, v7.s[0]
        ldr q13, [x0, #0x3a0]
        str q26, [x0, #0x300]
        sqrdmulh v27.4s, v30.4s, v0.s[1]
        mls v18.4s, v5.4s, v7.s[0]
        ldr q9, [x0, #0x1a0]
        sub v16.4s, v10.4s, v17.4s
        add v15.4s, v10.4s, v17.4s
        sqrdmulh v10.4s, v6.4s, v0.s[1]
        str q16, [x0, #0x80]
        str q15, [x0], #0x10
        sqrdmulh v19.4s, v13.4s, v0.s[1]
        sub v15.4s, v31.4s, v18.4s
        mul v8.4s, v13.4s, v0.s[0]
        add v26.4s, v31.4s, v18.4s
        str q15, [x0, #0x270]
        sqrdmulh v13.4s, v29.4s, v1.s[1]
        str q26, [x0, #0x1f0]
        mls v8.4s, v19.4s, v7.s[0]
        mls v11.4s, v20.4s, v7.s[0]
        mls v23.4s, v13.4s, v7.s[0]
        add v22.4s, v9.4s, v8.4s
        ldr q6, [x0, #0x210]
        sub v29.4s, v9.4s, v8.4s
        mul v17.4s, v30.4s, v0.s[0]
        ldr q9, [x0, #0x300]
        sqrdmulh v13.4s, v22.4s, v0.s[3]
        add v18.4s, v21.4s, v23.4s
        mls v4.4s, v10.4s, v7.s[0]
        sub v31.4s, v21.4s, v23.4s
        sqrdmulh v16.4s, v31.4s, v3.s[1]
        add v19.4s, v24.4s, v14.4s
        mul v14.4s, v22.4s, v0.s[2]
        sub v10.4s, v28.4s, v4.4s
        mls v14.4s, v13.4s, v7.s[0]
        ldr q13, [x0, #0x100]
        sqrdmulh v22.4s, v9.4s, v0.s[1]
        mul v8.4s, v9.4s, v0.s[0]
        mul v23.4s, v31.4s, v3.s[0]
        mls v8.4s, v22.4s, v7.s[0]
        mls v23.4s, v16.4s, v7.s[0]
        add v16.4s, v28.4s, v4.4s
        ldr q22, [x0, #0x90]
        mul v4.4s, v6.4s, v0.s[0]
        mls v17.4s, v27.4s, v7.s[0]
        add v21.4s, v13.4s, v8.4s
        sub v27.4s, v13.4s, v8.4s
        sqrdmulh v31.4s, v21.4s, v0.s[3]
        str q25, [x0, #0xf0]
        mul v8.4s, v21.4s, v0.s[2]
        add v24.4s, v22.4s, v17.4s
        sub v21.4s, v22.4s, v17.4s
        sqrdmulh v5.4s, v18.4s, v2.s[3]
        mls v8.4s, v31.4s, v7.s[0]
        sub v9.4s, v24.4s, v14.4s
        sqrdmulh v20.4s, v27.4s, v1.s[1]
        mul v26.4s, v27.4s, v1.s[0]
        sub v25.4s, v16.4s, v8.4s
        mul v18.4s, v18.4s, v2.s[2]
        sub v22.4s, v25.4s, v11.4s
        mls v26.4s, v20.4s, v7.s[0]
        sqrdmulh v20.4s, v9.4s, v2.s[1]
        sqrdmulh v12.4s, v19.4s, v1.s[3]
        sub v15.4s, v10.4s, v26.4s
        subs x4, x4, #0x1
        cbnz x4, Lmldsa_ntt_layer123_start
        add v13.4s, v10.4s, v26.4s
        mls v18.4s, v5.4s, v7.s[0]
        str q22, [x0, #0x180]
        add v27.4s, v16.4s, v8.4s
        mul v22.4s, v19.4s, v1.s[2]
        add v26.4s, v24.4s, v14.4s
        ldr q31, [x0, #0x110]
        sub v14.4s, v15.4s, v23.4s
        add v17.4s, v15.4s, v23.4s
        mls v22.4s, v12.4s, v7.s[0]
        add v28.4s, v13.4s, v18.4s
        str q14, [x0, #0x380]
        sqrdmulh v24.4s, v6.4s, v0.s[1]
        add v5.4s, v25.4s, v11.4s
        sub v19.4s, v13.4s, v18.4s
        str q17, [x0, #0x300]
        str q5, [x0, #0x100]
        mul v16.4s, v9.4s, v2.s[0]
        ldr q18, [x0, #0x310]
        str q19, [x0, #0x280]
        mls v16.4s, v20.4s, v7.s[0]
        str q28, [x0, #0x200]
        add v13.4s, v27.4s, v22.4s
        ldr q15, [x0, #0x10]
        sub v10.4s, v27.4s, v22.4s
        mls v4.4s, v24.4s, v7.s[0]
        str q13, [x0], #0x10
        str q10, [x0, #0x70]
        sqrdmulh v12.4s, v29.4s, v1.s[1]
        mul v23.4s, v29.4s, v1.s[0]
        mul v8.4s, v26.4s, v1.s[2]
        add v20.4s, v15.4s, v4.4s
        sub v6.4s, v15.4s, v4.4s
        mls v23.4s, v12.4s, v7.s[0]
        sqrdmulh v22.4s, v18.4s, v0.s[1]
        mul v5.4s, v18.4s, v0.s[0]
        sub v28.4s, v21.4s, v23.4s
        sqrdmulh v10.4s, v26.4s, v1.s[3]
        mls v5.4s, v22.4s, v7.s[0]
        sqrdmulh v30.4s, v28.4s, v3.s[1]
        add v4.4s, v21.4s, v23.4s
        mls v8.4s, v10.4s, v7.s[0]
        add v12.4s, v31.4s, v5.4s
        sub v9.4s, v31.4s, v5.4s
        sqrdmulh v25.4s, v4.4s, v2.s[3]
        sqrdmulh v15.4s, v9.4s, v1.s[1]
        sqrdmulh v31.4s, v12.4s, v0.s[3]
        mul v18.4s, v12.4s, v0.s[2]
        mul v11.4s, v9.4s, v1.s[0]
        mls v18.4s, v31.4s, v7.s[0]
        mul v29.4s, v4.4s, v2.s[2]
        mls v29.4s, v25.4s, v7.s[0]
        add v23.4s, v20.4s, v18.4s
        mls v11.4s, v15.4s, v7.s[0]
        sub v31.4s, v20.4s, v18.4s
        add v17.4s, v23.4s, v8.4s
        add v5.4s, v31.4s, v16.4s
        mul v24.4s, v28.4s, v3.s[0]
        str q17, [x0], #0x10
        sub v19.4s, v31.4s, v16.4s
        mls v24.4s, v30.4s, v7.s[0]
        str q5, [x0, #0xf0]
        add v31.4s, v6.4s, v11.4s
        sub v26.4s, v23.4s, v8.4s
        str q19, [x0, #0x170]
        add v4.4s, v31.4s, v29.4s
        sub v13.4s, v6.4s, v11.4s
        str q26, [x0, #0x70]
        sub v11.4s, v31.4s, v29.4s
        sub v22.4s, v13.4s, v24.4s
        add v23.4s, v13.4s, v24.4s
        str q4, [x0, #0x1f0]
        str q11, [x0, #0x270]
        str q23, [x0, #0x2f0]
        str q22, [x0, #0x370]
        mov x0, x3
        mov x4, #0x8                // =8
        ldr q9, [x0, #0x40]
        ldr q23, [x1], #0x40
        ldr q21, [x2, #0x60]
        ldr q1, [x0, #0x20]
        ldur q14, [x1, #-0x30]
        ldr q13, [x0]
        ldr q11, [x2, #0x50]
        sqrdmulh v16.4s, v9.4s, v23.s[1]
        ldr q17, [x0, #0x50]
        mul v15.4s, v9.4s, v23.s[0]
        ldr q30, [x0, #0x70]
        ldr q27, [x0, #0x60]
        ldr q8, [x2, #0x30]
        sqrdmulh v12.4s, v17.4s, v23.s[1]
        ldr q6, [x0, #0x30]
        mls v15.4s, v16.4s, v7.s[0]
        sqrdmulh v18.4s, v27.4s, v23.s[1]
        sqrdmulh v19.4s, v30.4s, v23.s[1]
        add v5.4s, v13.4s, v15.4s
        mul v25.4s, v27.4s, v23.s[0]
        sub v26.4s, v13.4s, v15.4s
        mls v25.4s, v18.4s, v7.s[0]
        mul v10.4s, v17.4s, v23.s[0]
        mls v10.4s, v12.4s, v7.s[0]
        mul v4.4s, v30.4s, v23.s[0]
        sub v22.4s, v1.4s, v25.4s
        mls v4.4s, v19.4s, v7.s[0]
        add v28.4s, v1.4s, v25.4s
        sqrdmulh v19.4s, v28.4s, v23.s[3]
        sqrdmulh v9.4s, v22.4s, v14.s[1]
        add v2.4s, v6.4s, v4.4s
        mul v0.4s, v28.4s, v23.s[2]
        sqrdmulh v27.4s, v2.4s, v23.s[3]
        sub v17.4s, v6.4s, v4.4s
        mul v3.4s, v2.4s, v23.s[2]
        sqrdmulh v20.4s, v17.4s, v14.s[1]
        ldr q1, [x0, #0x10]
        mls v3.4s, v27.4s, v7.s[0]
        mls v0.4s, v19.4s, v7.s[0]
        ldur q16, [x1, #-0x20]
        add v31.4s, v1.4s, v10.4s
        mul v30.4s, v17.4s, v14.s[0]
        mls v30.4s, v20.4s, v7.s[0]
        add v27.4s, v31.4s, v3.4s
        sub v23.4s, v1.4s, v10.4s
        sub v24.4s, v31.4s, v3.4s
        sqrdmulh v4.4s, v27.4s, v14.s[3]
        sqrdmulh v10.4s, v24.4s, v16.s[1]
        mul v18.4s, v24.4s, v16.s[0]
        add v15.4s, v23.4s, v30.4s
        sub v23.4s, v23.4s, v30.4s
        mul v29.4s, v27.4s, v14.s[2]
        sub v2.4s, v5.4s, v0.4s
        add v12.4s, v5.4s, v0.4s
        mls v18.4s, v10.4s, v7.s[0]
        ldur q3, [x1, #-0x10]
        mls v29.4s, v4.4s, v7.s[0]
        mul v4.4s, v22.4s, v14.s[0]
        add v1.4s, v2.4s, v18.4s
        sub v24.4s, v2.4s, v18.4s
        mls v4.4s, v9.4s, v7.s[0]
        ldr q20, [x2, #0x10]
        add v25.4s, v12.4s, v29.4s
        mul v9.4s, v23.4s, v3.s[0]
        sub v5.4s, v12.4s, v29.4s
        sqrdmulh v31.4s, v23.4s, v3.s[1]
        trn2 v6.4s, v1.4s, v24.4s
        trn2 v10.4s, v25.4s, v5.4s
        sqrdmulh v13.4s, v15.4s, v16.s[3]
        trn2 v30.2d, v10.2d, v6.2d
        ldr q3, [x2], #0xc0
        mul v12.4s, v15.4s, v16.s[2]
        trn1 v27.2d, v10.2d, v6.2d
        mls v9.4s, v31.4s, v7.s[0]
        trn1 v22.4s, v25.4s, v5.4s
        sub v6.4s, v26.4s, v4.4s
        mls v12.4s, v13.4s, v7.s[0]
        trn1 v1.4s, v1.4s, v24.4s
        add v13.4s, v26.4s, v4.4s
        trn2 v10.2d, v22.2d, v1.2d
        mul v28.4s, v30.4s, v3.4s
        sub v31.4s, v6.4s, v9.4s
        sub x4, x4, #0x1

Lmldsa_ntt_layer45678_start:
        add v2.4s, v13.4s, v12.4s
        sqrdmulh v5.4s, v30.4s, v20.4s
        sub v25.4s, v13.4s, v12.4s
        add v17.4s, v6.4s, v9.4s
        mul v19.4s, v10.4s, v3.4s
        trn2 v4.4s, v2.4s, v25.4s
        ldur q24, [x2, #-0x50]
        trn2 v29.4s, v17.4s, v31.4s
        sqrdmulh v15.4s, v10.4s, v20.4s
        mls v28.4s, v5.4s, v7.s[0]
        trn2 v3.2d, v4.2d, v29.2d
        sqrdmulh v12.4s, v3.4s, v24.4s
        mul v16.4s, v3.4s, v21.4s
        mls v19.4s, v15.4s, v7.s[0]
        ldur q10, [x2, #-0xa0]
        add v13.4s, v27.4s, v28.4s
        mls v16.4s, v12.4s, v7.s[0]
        sqrdmulh v9.4s, v13.4s, v8.4s
        sub v30.4s, v27.4s, v28.4s
        ldr q18, [x1], #0x40
        mul v8.4s, v13.4s, v10.4s
        ldr q10, [x0, #0xd0]
        sqrdmulh v14.4s, v30.4s, v11.4s
        ldr q23, [x0, #0xe0]
        sqrdmulh v13.4s, v10.4s, v18.s[1]
        sqrdmulh v12.4s, v23.4s, v18.s[1]
        ldur q6, [x2, #-0x80]
        mul v3.4s, v10.4s, v18.s[0]
        mls v3.4s, v13.4s, v7.s[0]
        ldr q13, [x0, #0xf0]
        trn1 v27.4s, v2.4s, v25.4s
        mul v2.4s, v30.4s, v6.4s
        trn1 v20.4s, v17.4s, v31.4s
        trn1 v25.2d, v4.2d, v29.2d
        sqrdmulh v10.4s, v13.4s, v18.s[1]
        trn2 v5.2d, v27.2d, v20.2d
        ldur q6, [x2, #-0x10]
        mls v8.4s, v9.4s, v7.s[0]
        sub v15.4s, v25.4s, v16.4s
        sqrdmulh v31.4s, v5.4s, v24.4s
        sqrdmulh v30.4s, v15.4s, v6.4s
        ldur q9, [x2, #-0x30]
        mul v4.4s, v5.4s, v21.4s
        ldur q21, [x2, #-0x40]
        ldur q6, [x2, #-0x20]
        add v5.4s, v25.4s, v16.4s
        mls v4.4s, v31.4s, v7.s[0]
        mul v0.4s, v5.4s, v21.4s
        mul v17.4s, v13.4s, v18.s[0]
        mls v17.4s, v10.4s, v7.s[0]
        ldr q28, [x0, #0xb0]
        sqrdmulh v26.4s, v5.4s, v9.4s
        mul v9.4s, v15.4s, v6.4s
        trn1 v6.2d, v22.2d, v1.2d
        mls v9.4s, v30.4s, v7.s[0]
        add v25.4s, v28.4s, v17.4s
        mls v2.4s, v14.4s, v7.s[0]
        trn1 v5.2d, v27.2d, v20.2d
        ldr q20, [x2, #0x10]
        mul v29.4s, v25.4s, v18.s[2]
        add v15.4s, v6.4s, v19.4s
        ldr q30, [x0, #0xc0]
        sub v19.4s, v6.4s, v19.4s
        add v31.4s, v15.4s, v8.4s
        mls v0.4s, v26.4s, v7.s[0]
        ldur q14, [x1, #-0x30]
        add v21.4s, v19.4s, v2.4s
        sub v24.4s, v19.4s, v2.4s
        sqrdmulh v27.4s, v25.4s, v18.s[3]
        sub v26.4s, v15.4s, v8.4s
        ldr q2, [x0, #0x90]
        mul v16.4s, v30.4s, v18.s[0]
        sub v25.4s, v28.4s, v17.4s
        trn1 v11.4s, v31.4s, v26.4s
        ldr q1, [x0, #0xa0]
        trn1 v6.4s, v21.4s, v24.4s
        sqrdmulh v13.4s, v25.4s, v14.s[1]
        add v8.4s, v2.4s, v3.4s
        trn2 v28.2d, v11.2d, v6.2d
        sqrdmulh v19.4s, v30.4s, v18.s[1]
        sub v10.4s, v5.4s, v4.4s
        ldur q22, [x1, #-0x20]
        str q28, [x0, #0x20]
        mls v29.4s, v27.4s, v7.s[0]
        add v15.4s, v10.4s, v9.4s
        mul v25.4s, v25.4s, v14.s[0]
        ldur q27, [x1, #-0x10]
        trn2 v17.4s, v31.4s, v26.4s
        trn2 v21.4s, v21.4s, v24.4s
        mls v16.4s, v19.4s, v7.s[0]
        sub v24.4s, v8.4s, v29.4s
        sub v10.4s, v10.4s, v9.4s
        mls v25.4s, v13.4s, v7.s[0]
        trn1 v13.2d, v11.2d, v6.2d
        ldr q28, [x0, #0x80]
        sqrdmulh v30.4s, v24.4s, v22.s[1]
        trn2 v19.2d, v17.2d, v21.2d
        trn1 v6.2d, v17.2d, v21.2d
        mul v31.4s, v23.4s, v18.s[0]
        str q13, [x0], #0x80
        stur q6, [x0, #-0x70]
        stur q19, [x0, #-0x50]
        ldr q11, [x2, #0x50]
        mls v31.4s, v12.4s, v7.s[0]
        ldr q21, [x2, #0x60]
        trn1 v9.4s, v15.4s, v10.4s
        trn2 v6.4s, v15.4s, v10.4s
        mul v24.4s, v24.4s, v22.s[0]
        sub v10.4s, v2.4s, v3.4s
        ldr q3, [x2], #0xc0
        mls v24.4s, v30.4s, v7.s[0]
        add v26.4s, v8.4s, v29.4s
        ldur q8, [x2, #-0x90]
        add v17.4s, v5.4s, v4.4s
        sqrdmulh v2.4s, v26.4s, v14.s[3]
        sub v13.4s, v1.4s, v31.4s
        add v30.4s, v1.4s, v31.4s
        add v15.4s, v10.4s, v25.4s
        sqrdmulh v19.4s, v13.4s, v14.s[1]
        sub v25.4s, v10.4s, v25.4s
        mul v29.4s, v13.4s, v14.s[0]
        sub v5.4s, v28.4s, v16.4s
        sqrdmulh v4.4s, v30.4s, v18.s[3]
        sub v23.4s, v17.4s, v0.4s
        add v31.4s, v17.4s, v0.4s
        mul v18.4s, v30.4s, v18.s[2]
        add v1.4s, v28.4s, v16.4s
        trn2 v12.4s, v31.4s, v23.4s
        mls v29.4s, v19.4s, v7.s[0]
        trn1 v13.4s, v31.4s, v23.4s
        trn2 v30.2d, v12.2d, v6.2d
        mls v18.4s, v4.4s, v7.s[0]
        trn2 v10.2d, v13.2d, v9.2d
        trn1 v31.2d, v13.2d, v9.2d
        mul v19.4s, v26.4s, v14.s[2]
        trn1 v12.2d, v12.2d, v6.2d
        sub v6.4s, v5.4s, v29.4s
        mls v19.4s, v2.4s, v7.s[0]
        add v13.4s, v5.4s, v29.4s
        stur q10, [x0, #-0x20]
        sub v10.4s, v1.4s, v18.4s
        add v28.4s, v1.4s, v18.4s
        sqrdmulh v5.4s, v25.4s, v27.s[1]
        stur q31, [x0, #-0x40]
        add v26.4s, v10.4s, v24.4s
        sub v31.4s, v10.4s, v24.4s
        mul v9.4s, v25.4s, v27.s[0]
        stur q12, [x0, #-0x30]
        sub v24.4s, v28.4s, v19.4s
        sqrdmulh v10.4s, v15.4s, v22.s[3]
        trn1 v1.4s, v26.4s, v31.4s
        stur q30, [x0, #-0x10]
        add v30.4s, v28.4s, v19.4s
        mls v9.4s, v5.4s, v7.s[0]
        trn2 v25.4s, v26.4s, v31.4s
        trn2 v14.4s, v30.4s, v24.4s
        mul v12.4s, v15.4s, v22.s[2]
        trn1 v22.4s, v30.4s, v24.4s
        trn1 v27.2d, v14.2d, v25.2d
        mls v12.4s, v10.4s, v7.s[0]
        trn2 v30.2d, v14.2d, v25.2d
        sub v31.4s, v6.4s, v9.4s
        trn2 v10.2d, v22.2d, v1.2d
        mul v28.4s, v30.4s, v3.4s
        subs x4, x4, #0x1
        cbnz x4, Lmldsa_ntt_layer45678_start
        add v9.4s, v6.4s, v9.4s
        sqrdmulh v6.4s, v30.4s, v20.4s
        ldur q24, [x2, #-0xa0]
        add v25.4s, v13.4s, v12.4s
        sub v15.4s, v13.4s, v12.4s
        mul v19.4s, v10.4s, v3.4s
        trn2 v5.4s, v9.4s, v31.4s
        sqrdmulh v3.4s, v10.4s, v20.4s
        trn2 v10.4s, v25.4s, v15.4s
        mls v28.4s, v6.4s, v7.s[0]
        trn2 v13.2d, v10.2d, v5.2d
        ldur q30, [x2, #-0x50]
        mul v12.4s, v13.4s, v21.4s
        mls v19.4s, v3.4s, v7.s[0]
        add v20.4s, v27.4s, v28.4s
        sqrdmulh v13.4s, v13.4s, v30.4s
        sub v3.4s, v27.4s, v28.4s
        mul v24.4s, v20.4s, v24.4s
        sqrdmulh v6.4s, v3.4s, v11.4s
        ldur q27, [x2, #-0x80]
        mls v12.4s, v13.4s, v7.s[0]
        trn1 v25.4s, v25.4s, v15.4s
        mul v27.4s, v3.4s, v27.4s
        trn1 v31.4s, v9.4s, v31.4s
        trn1 v3.2d, v10.2d, v5.2d
        ldur q13, [x2, #-0x30]
        ldur q15, [x2, #-0x40]
        sqrdmulh v9.4s, v20.4s, v8.4s
        trn2 v20.2d, v25.2d, v31.2d
        ldur q10, [x2, #-0x10]
        mls v27.4s, v6.4s, v7.s[0]
        add v5.4s, v3.4s, v12.4s
        sub v6.4s, v3.4s, v12.4s
        sqrdmulh v3.4s, v20.4s, v30.4s
        trn1 v12.2d, v22.2d, v1.2d
        sqrdmulh v10.4s, v6.4s, v10.4s
        mls v24.4s, v9.4s, v7.s[0]
        sub v9.4s, v12.4s, v19.4s
        trn1 v25.2d, v25.2d, v31.2d
        sqrdmulh v31.4s, v5.4s, v13.4s
        add v30.4s, v9.4s, v27.4s
        add v13.4s, v12.4s, v19.4s
        mul v1.4s, v20.4s, v21.4s
        ldur q12, [x2, #-0x20]
        add v21.4s, v13.4s, v24.4s
        sub v13.4s, v13.4s, v24.4s
        mls v1.4s, v3.4s, v7.s[0]
        sub v3.4s, v9.4s, v27.4s
        mul v9.4s, v6.4s, v12.4s
        trn2 v12.4s, v21.4s, v13.4s
        trn1 v6.4s, v30.4s, v3.4s
        trn2 v30.4s, v30.4s, v3.4s
        mls v9.4s, v10.4s, v7.s[0]
        trn1 v13.4s, v21.4s, v13.4s
        mul v15.4s, v5.4s, v15.4s
        sub v3.4s, v25.4s, v1.4s
        add v5.4s, v25.4s, v1.4s
        mls v15.4s, v31.4s, v7.s[0]
        trn1 v21.2d, v13.2d, v6.2d
        trn2 v6.2d, v13.2d, v6.2d
        add v10.4s, v3.4s, v9.4s
        sub v13.4s, v3.4s, v9.4s
        str q21, [x0], #0x80
        trn1 v3.2d, v12.2d, v30.2d
        trn2 v31.2d, v12.2d, v30.2d
        trn1 v21.4s, v10.4s, v13.4s
        sub v30.4s, v5.4s, v15.4s
        add v12.4s, v5.4s, v15.4s
        stur q3, [x0, #-0x70]
        trn2 v13.4s, v10.4s, v13.4s
        trn1 v19.4s, v12.4s, v30.4s
        trn2 v12.4s, v12.4s, v30.4s
        stur q6, [x0, #-0x60]
        stur q31, [x0, #-0x50]
        trn1 v10.2d, v19.2d, v21.2d
        trn2 v3.2d, v19.2d, v21.2d
        trn1 v21.2d, v12.2d, v13.2d
        trn2 v13.2d, v12.2d, v13.2d
        stur q10, [x0, #-0x40]
        stur q3, [x0, #-0x20]
        stur q13, [x0, #-0x10]
        stur q21, [x0, #-0x30]
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

S2N_BN_SIZE_DIRECTIVE(mldsa_ntt_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
