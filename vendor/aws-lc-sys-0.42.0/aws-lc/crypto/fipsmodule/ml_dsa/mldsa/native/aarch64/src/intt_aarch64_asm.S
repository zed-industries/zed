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

/* AArch64 ML-DSA inverse NTT following @[NeonNTT] and @[SLOTHY_Paper] */

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/intt_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_intt_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_intt_aarch64_asm)
S2N_BN_SYMBOL(mldsa_intt_aarch64_asm):

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
        dup v31.4s, w5
        mov x3, x0
        mov x4, #0x10               // =16
        ldr q27, [x3, #0x10]
        ldr q18, [x3]
        ldr q3, [x3, #0x20]
        ldr q13, [x3, #0x30]
        ldr q2, [x1, #0x30]
        ldr q8, [x3, #0x70]
        ldr q21, [x3, #0x60]
        trn1 v10.4s, v18.4s, v27.4s
        trn2 v23.4s, v18.4s, v27.4s
        trn1 v6.4s, v3.4s, v13.4s
        trn2 v18.4s, v3.4s, v13.4s
        ldr q12, [x1, #0x50]
        ldr q17, [x1, #0x40]
        trn2 v29.2d, v10.2d, v6.2d
        trn2 v14.2d, v23.2d, v18.2d
        trn1 v10.2d, v10.2d, v6.2d
        trn1 v26.2d, v23.2d, v18.2d
        sub v3.4s, v29.4s, v14.4s
        trn1 v13.4s, v21.4s, v8.4s
        ldr q6, [x1, #0x10]
        add v24.4s, v10.4s, v26.4s
        sub v30.4s, v10.4s, v26.4s
        sqrdmulh v12.4s, v3.4s, v12.4s
        ldr q9, [x1, #0x20]
        sqrdmulh v5.4s, v30.4s, v2.4s
        mul v4.4s, v3.4s, v17.4s
        ldr q3, [x3, #0x50]
        add v10.4s, v29.4s, v14.4s
        ldr q26, [x3, #0x40]
        mul v15.4s, v30.4s, v9.4s
        trn2 v30.4s, v21.4s, v8.4s
        sub v21.4s, v24.4s, v10.4s
        add v29.4s, v24.4s, v10.4s
        mls v15.4s, v5.4s, v31.s[0]
        ldr q16, [x1], #0x60
        trn2 v10.4s, v26.4s, v3.4s
        mls v4.4s, v12.4s, v31.s[0]
        trn1 v3.4s, v26.4s, v3.4s
        ldr q0, [x1, #0x50]
        trn2 v25.2d, v10.2d, v30.2d
        sqrdmulh v12.4s, v21.4s, v6.4s
        trn2 v1.2d, v3.2d, v13.2d
        mul v21.4s, v21.4s, v16.4s
        sub v23.4s, v1.4s, v25.4s
        sub v2.4s, v15.4s, v4.4s
        add v20.4s, v15.4s, v4.4s
        sqrdmulh v4.4s, v23.4s, v0.4s
        trn1 v7.2d, v3.2d, v13.2d
        sqrdmulh v3.4s, v2.4s, v6.4s
        trn1 v15.2d, v10.2d, v30.2d
        mls v21.4s, v12.4s, v31.s[0]
        ldr q12, [x1, #0x30]
        sub v13.4s, v7.4s, v15.4s
        mul v10.4s, v2.4s, v16.4s
        ldr q16, [x1, #0x40]
        ldr q5, [x1, #0x20]
        mls v10.4s, v3.4s, v31.s[0]
        trn2 v11.4s, v29.4s, v20.4s
        sqrdmulh v2.4s, v13.4s, v12.4s
        ldr d9, [x2], #0x20
        mul v17.4s, v13.4s, v5.4s
        trn1 v24.4s, v29.4s, v20.4s
        trn1 v12.4s, v21.4s, v10.4s
        trn2 v10.4s, v21.4s, v10.4s
        mul v22.4s, v23.4s, v16.4s
        ldur q26, [x2, #-0x10]
        trn1 v13.2d, v24.2d, v12.2d
        trn1 v3.2d, v11.2d, v10.2d
        mls v17.4s, v2.4s, v31.s[0]
        mls v22.4s, v4.4s, v31.s[0]
        sub v23.4s, v13.4s, v3.4s
        trn2 v10.2d, v11.2d, v10.2d
        sqrdmulh v0.4s, v23.4s, v26.s[1]
        trn2 v21.2d, v24.2d, v12.2d
        mul v29.4s, v23.4s, v26.s[0]
        sub v23.4s, v21.4s, v10.4s
        sqrdmulh v8.4s, v23.4s, v26.s[3]
        add v6.4s, v1.4s, v25.4s
        add v11.4s, v21.4s, v10.4s
        mls v29.4s, v0.4s, v31.s[0]
        add v30.4s, v13.4s, v3.4s
        ldr q2, [x1, #0x10]
        mul v3.4s, v23.4s, v26.s[2]
        add v12.4s, v7.4s, v15.4s
        mls v3.4s, v8.4s, v31.s[0]
        sub v13.4s, v30.4s, v11.4s
        sub v21.4s, v12.4s, v6.4s
        add v26.4s, v17.4s, v22.4s
        sqrdmulh v28.4s, v13.4s, v9.s[1]
        mul v18.4s, v13.4s, v9.s[0]
        sub v8.4s, v29.4s, v3.4s
        sqrdmulh v24.4s, v21.4s, v2.4s
        add v15.4s, v30.4s, v11.4s
        add v10.4s, v29.4s, v3.4s
        add v14.4s, v12.4s, v6.4s
        sqrdmulh v16.4s, v8.4s, v9.s[1]
        sub v3.4s, v17.4s, v22.4s
        mul v8.4s, v8.4s, v9.s[0]
        ldr q17, [x1], #0x60
        sub x4, x4, #0x2

Lmldsa_intt_layer5678_start:
        ldr d4, [x2], #0x20
        mul v0.4s, v21.4s, v17.4s
        ldr q7, [x3, #0xa0]
        ldr q27, [x3, #0xb0]
        mls v8.4s, v16.4s, v31.s[0]
        ldr q29, [x3, #0x80]
        trn1 v9.4s, v14.4s, v26.4s
        ldr q25, [x3, #0x90]
        mls v0.4s, v24.4s, v31.s[0]
        str q15, [x3], #0x40
        trn1 v1.4s, v7.4s, v27.4s
        ldr q15, [x1, #0x50]
        trn2 v12.4s, v7.4s, v27.4s
        sqrdmulh v16.4s, v3.4s, v2.4s
        trn1 v21.4s, v29.4s, v25.4s
        stur q8, [x3, #-0x10]
        trn2 v19.4s, v29.4s, v25.4s
        mls v18.4s, v28.4s, v31.s[0]
        ldr q22, [x1, #0x30]
        trn2 v13.2d, v21.2d, v1.2d
        trn2 v5.2d, v19.2d, v12.2d
        mul v23.4s, v3.4s, v17.4s
        trn1 v17.2d, v21.2d, v1.2d
        ldr q29, [x1, #0x40]
        mls v23.4s, v16.4s, v31.s[0]
        sub v30.4s, v13.4s, v5.4s
        trn1 v24.2d, v19.2d, v12.2d
        stur q10, [x3, #-0x30]
        sqrdmulh v16.4s, v30.4s, v15.4s
        ldr q10, [x1, #0x20]
        trn2 v12.4s, v14.4s, v26.4s
        stur q18, [x3, #-0x20]
        mul v11.4s, v30.4s, v29.4s
        sub v8.4s, v17.4s, v24.4s
        trn1 v20.4s, v0.4s, v23.4s
        ldr q2, [x1, #0x10]
        trn2 v26.4s, v0.4s, v23.4s
        sqrdmulh v19.4s, v8.4s, v22.4s
        ldur q14, [x2, #-0x10]
        trn1 v21.2d, v9.2d, v20.2d
        mul v8.4s, v8.4s, v10.4s
        trn1 v6.2d, v12.2d, v26.2d
        trn2 v0.2d, v9.2d, v20.2d
        mls v11.4s, v16.4s, v31.s[0]
        sub v7.4s, v21.4s, v6.4s
        trn2 v28.2d, v12.2d, v26.2d
        add v10.4s, v21.4s, v6.4s
        mul v18.4s, v7.4s, v14.s[0]
        add v20.4s, v13.4s, v5.4s
        sqrdmulh v30.4s, v7.4s, v14.s[1]
        sub v21.4s, v0.4s, v28.4s
        add v13.4s, v0.4s, v28.4s
        add v9.4s, v17.4s, v24.4s
        sqrdmulh v1.4s, v21.4s, v14.s[3]
        ldr q17, [x1], #0x60
        mul v14.4s, v21.4s, v14.s[2]
        sub v21.4s, v9.4s, v20.4s
        mls v18.4s, v30.4s, v31.s[0]
        mls v14.4s, v1.4s, v31.s[0]
        mls v8.4s, v19.4s, v31.s[0]
        sub v3.4s, v10.4s, v13.4s
        add v15.4s, v10.4s, v13.4s
        sqrdmulh v28.4s, v3.4s, v4.s[1]
        add v10.4s, v18.4s, v14.4s
        sub v19.4s, v18.4s, v14.4s
        mul v18.4s, v3.4s, v4.s[0]
        sub v3.4s, v8.4s, v11.4s
        add v26.4s, v8.4s, v11.4s
        sqrdmulh v16.4s, v19.4s, v4.s[1]
        add v14.4s, v9.4s, v20.4s
        mul v8.4s, v19.4s, v4.s[0]
        sqrdmulh v24.4s, v21.4s, v2.4s
        subs x4, x4, #0x1
        cbnz x4, Lmldsa_intt_layer5678_start
        sqrdmulh v19.4s, v3.4s, v2.4s
        trn2 v0.4s, v14.4s, v26.4s
        str q15, [x3], #0x40
        trn1 v30.4s, v14.4s, v26.4s
        mul v26.4s, v21.4s, v17.4s
        stur q10, [x3, #-0x30]
        ldr d29, [x2], #0x20
        mls v26.4s, v24.4s, v31.s[0]
        mul v10.4s, v3.4s, v17.4s
        ldur q14, [x2, #-0x10]
        mls v10.4s, v19.4s, v31.s[0]
        mls v8.4s, v16.4s, v31.s[0]
        trn2 v22.4s, v26.4s, v10.4s
        trn1 v25.4s, v26.4s, v10.4s
        trn1 v1.2d, v30.2d, v25.2d
        trn1 v2.2d, v0.2d, v22.2d
        trn2 v13.2d, v30.2d, v25.2d
        trn2 v7.2d, v0.2d, v22.2d
        mls v18.4s, v28.4s, v31.s[0]
        sub v22.4s, v1.4s, v2.4s
        add v3.4s, v13.4s, v7.4s
        sqrdmulh v27.4s, v22.4s, v14.s[1]
        sub v4.4s, v13.4s, v7.4s
        stur q8, [x3, #-0x10]
        sqrdmulh v0.4s, v4.4s, v14.s[3]
        add v16.4s, v1.4s, v2.4s
        stur q18, [x3, #-0x20]
        mul v23.4s, v4.4s, v14.s[2]
        add v4.4s, v16.4s, v3.4s
        sub v17.4s, v16.4s, v3.4s
        mul v14.4s, v22.4s, v14.s[0]
        str q4, [x3], #0x40
        mls v23.4s, v0.4s, v31.s[0]
        mls v14.4s, v27.4s, v31.s[0]
        mul v11.4s, v17.4s, v29.s[0]
        sqrdmulh v10.4s, v17.4s, v29.s[1]
        sub v0.4s, v14.4s, v23.4s
        add v6.4s, v14.4s, v23.4s
        sqrdmulh v12.4s, v0.4s, v29.s[1]
        mul v0.4s, v0.4s, v29.s[0]
        stur q6, [x3, #-0x30]
        mls v11.4s, v10.4s, v31.s[0]
        mls v0.4s, v12.4s, v31.s[0]
        stur q11, [x3, #-0x20]
        stur q0, [x3, #-0x10]
        mov w5, #0x3ffe             // =16382
        dup v29.4s, w5
        mov w5, #0xe03              // =3587
        movk w5, #0x40, lsl #16
        dup v30.4s, w5
        mov x4, #0x4                // =4
        ldr q0, [x2], #0x80
        ldur q1, [x2, #-0x70]
        ldur q2, [x2, #-0x60]
        ldur q3, [x2, #-0x50]
        ldur q4, [x2, #-0x40]
        ldur q5, [x2, #-0x30]
        ldur q6, [x2, #-0x20]
        ldur q7, [x2, #-0x10]
        ldr q8, [x0, #0xc0]
        ldr q27, [x0, #0x80]
        ldr q20, [x0, #0x1c0]
        ldr q23, [x0, #0x180]
        ldr q24, [x0, #0x3c0]
        ldr q28, [x0, #0x40]
        ldr q25, [x0, #0x340]
        ldr q10, [x0, #0x380]
        sub v15.4s, v27.4s, v8.4s
        ldr q26, [x0]
        ldr q18, [x0, #0x300]
        add v9.4s, v23.4s, v20.4s
        mul v11.4s, v15.4s, v4.s[0]
        sub v19.4s, v23.4s, v20.4s
        sub v22.4s, v10.4s, v24.4s
        ldr q13, [x0, #0x240]
        sqrdmulh v12.4s, v15.4s, v4.s[1]
        sub v14.4s, v26.4s, v28.4s
        ldr q17, [x0, #0x200]
        add v23.4s, v18.4s, v25.4s
        sqrdmulh v15.4s, v14.4s, v3.s[3]
        sub v18.4s, v18.4s, v25.4s
        mul v25.4s, v14.4s, v3.s[2]
        sub v16.4s, v17.4s, v13.4s
        mls v11.4s, v12.4s, v31.s[0]
        add v21.4s, v10.4s, v24.4s
        mls v25.4s, v15.4s, v31.s[0]
        mul v12.4s, v16.4s, v5.s[2]
        sub v10.4s, v23.4s, v21.4s
        sqrdmulh v14.4s, v10.4s, v3.s[1]
        add v23.4s, v23.4s, v21.4s
        add v27.4s, v27.4s, v8.4s
        mul v15.4s, v10.4s, v3.s[0]
        add v24.4s, v25.4s, v11.4s
        sub v10.4s, v25.4s, v11.4s
        ldr q25, [x0, #0x100]
        sqrdmulh v8.4s, v18.4s, v6.s[3]
        add v21.4s, v26.4s, v28.4s
        ldr q26, [x0, #0x140]
        sqrdmulh v20.4s, v16.4s, v5.s[3]
        sqrdmulh v28.4s, v22.4s, v7.s[1]
        add v11.4s, v25.4s, v26.4s
        mls v15.4s, v14.4s, v31.s[0]
        add v17.4s, v17.4s, v13.4s
        mul v13.4s, v22.4s, v7.s[0]
        ldr q14, [x0, #0x280]
        ldr q22, [x0, #0x2c0]
        mls v13.4s, v28.4s, v31.s[0]
        sub v16.4s, v25.4s, v26.4s
        sub v28.4s, v21.4s, v27.4s
        mls v12.4s, v20.4s, v31.s[0]
        add v20.4s, v11.4s, v9.4s
        add v26.4s, v14.4s, v22.4s
        sub v14.4s, v14.4s, v22.4s
        sqrdmulh v22.4s, v28.4s, v1.s[3]
        sub v9.4s, v11.4s, v9.4s
        mul v25.4s, v14.4s, v6.s[0]
        sub v11.4s, v17.4s, v26.4s
        add v17.4s, v17.4s, v26.4s
        add v26.4s, v21.4s, v27.4s
        sqrdmulh v27.4s, v11.4s, v2.s[3]
        mul v21.4s, v11.4s, v2.s[2]
        mul v11.4s, v18.4s, v6.s[2]
        mls v11.4s, v8.4s, v31.s[0]
        sqrdmulh v8.4s, v14.4s, v6.s[1]
        add v18.4s, v26.4s, v20.4s
        mls v21.4s, v27.4s, v31.s[0]
        sub v27.4s, v17.4s, v23.4s
        mul v14.4s, v28.4s, v1.s[2]
        sub v28.4s, v26.4s, v20.4s
        add v23.4s, v17.4s, v23.4s
        mls v25.4s, v8.4s, v31.s[0]
        add v26.4s, v21.4s, v15.4s
        sub v15.4s, v21.4s, v15.4s
        mul v8.4s, v19.4s, v5.s[0]
        mls v14.4s, v22.4s, v31.s[0]
        sub v22.4s, v11.4s, v13.4s
        sub v20.4s, v12.4s, v25.4s
        add v21.4s, v12.4s, v25.4s
        sqrdmulh v12.4s, v22.4s, v3.s[1]
        mul v17.4s, v16.4s, v4.s[2]
        sqrdmulh v25.4s, v16.4s, v4.s[3]
        add v16.4s, v11.4s, v13.4s
        mul v11.4s, v22.4s, v3.s[0]
        sqrdmulh v19.4s, v19.4s, v5.s[1]
        sqrdmulh v22.4s, v15.4s, v1.s[1]
        sqrdmulh v13.4s, v20.4s, v2.s[3]
        mul v20.4s, v20.4s, v2.s[2]
        mls v11.4s, v12.4s, v31.s[0]
        mls v20.4s, v13.4s, v31.s[0]
        sub v13.4s, v18.4s, v23.4s
        add v23.4s, v18.4s, v23.4s
        mls v17.4s, v25.4s, v31.s[0]
        mls v8.4s, v19.4s, v31.s[0]
        sub v18.4s, v20.4s, v11.4s
        add v20.4s, v20.4s, v11.4s
        sqrdmulh v12.4s, v9.4s, v2.s[1]
        mul v15.4s, v15.4s, v1.s[0]
        sub v11.4s, v17.4s, v8.4s
        mls v15.4s, v22.4s, v31.s[0]
        add v19.4s, v17.4s, v8.4s
        sqrdmulh v25.4s, v11.4s, v2.s[1]
        add v17.4s, v24.4s, v19.4s
        sub v24.4s, v24.4s, v19.4s
        mul v19.4s, v11.4s, v2.s[0]
        sqrdmulh v8.4s, v24.4s, v0.s[3]
        mls v19.4s, v25.4s, v31.s[0]
        mul v25.4s, v9.4s, v2.s[0]
        sub x4, x4, #0x1

Lmldsa_intt_layer1234_start:
        sub v22.4s, v21.4s, v16.4s
        mls v25.4s, v12.4s, v31.s[0]
        add v12.4s, v21.4s, v16.4s
        mul v9.4s, v24.4s, v0.s[2]
        mls v9.4s, v8.4s, v31.s[0]
        sub v16.4s, v14.4s, v25.4s
        add v14.4s, v14.4s, v25.4s
        sqrdmulh v24.4s, v22.4s, v1.s[1]
        sub v11.4s, v14.4s, v26.4s
        sqrdmulh v21.4s, v16.4s, v0.s[3]
        add v25.4s, v14.4s, v26.4s
        mul v14.4s, v22.4s, v1.s[0]
        mls v14.4s, v24.4s, v31.s[0]
        mul v16.4s, v16.4s, v0.s[2]
        mls v16.4s, v21.4s, v31.s[0]
        add v24.4s, v9.4s, v14.4s
        sqrdmulh v21.4s, v25.4s, v30.4s
        sqrdmulh v26.4s, v11.4s, v0.s[1]
        mul v8.4s, v25.4s, v29.4s
        mls v8.4s, v21.4s, v31.s[0]
        mul v25.4s, v24.4s, v29.4s
        mul v22.4s, v11.4s, v0.s[0]
        sub v11.4s, v17.4s, v12.4s
        str q8, [x0, #0x80]
        sub v8.4s, v9.4s, v14.4s
        add v14.4s, v16.4s, v15.4s
        sqrdmulh v9.4s, v11.4s, v0.s[1]
        sub v21.4s, v16.4s, v15.4s
        mls v22.4s, v26.4s, v31.s[0]
        mul v15.4s, v11.4s, v0.s[0]
        mls v15.4s, v9.4s, v31.s[0]
        add v11.4s, v17.4s, v12.4s
        mul v16.4s, v10.4s, v1.s[2]
        str q22, [x0, #0x280]
        sqrdmulh v26.4s, v10.4s, v1.s[3]
        str q15, [x0, #0x240]
        sqrdmulh v9.4s, v8.4s, v0.s[1]
        mul v12.4s, v8.4s, v0.s[0]
        mls v16.4s, v26.4s, v31.s[0]
        sqrdmulh v10.4s, v27.4s, v1.s[1]
        sqrdmulh v15.4s, v14.4s, v30.4s
        add v22.4s, v16.4s, v19.4s
        sub v19.4s, v16.4s, v19.4s
        mls v12.4s, v9.4s, v31.s[0]
        add v8.4s, v22.4s, v20.4s
        sub v26.4s, v22.4s, v20.4s
        mul v27.4s, v27.4s, v1.s[0]
        sqrdmulh v17.4s, v11.4s, v30.4s
        mls v27.4s, v10.4s, v31.s[0]
        mul v20.4s, v11.4s, v29.4s
        sqrdmulh v22.4s, v24.4s, v30.4s
        mls v20.4s, v17.4s, v31.s[0]
        mul v14.4s, v14.4s, v29.4s
        sqrdmulh v16.4s, v23.4s, v30.4s
        str q20, [x0, #0x40]
        sqrdmulh v11.4s, v21.4s, v0.s[1]
        sqrdmulh v20.4s, v28.4s, v0.s[3]
        mul v17.4s, v23.4s, v29.4s
        mul v23.4s, v28.4s, v0.s[2]
        mls v17.4s, v16.4s, v31.s[0]
        mul v10.4s, v21.4s, v0.s[0]
        str q12, [x0, #0x340]
        mls v25.4s, v22.4s, v31.s[0]
        str q17, [x0], #0x10
        ldr q17, [x0, #0x2c0]
        mls v14.4s, v15.4s, v31.s[0]
        ldr q12, [x0, #0x200]
        sqrdmulh v16.4s, v13.4s, v0.s[1]
        str q25, [x0, #0x130]
        mul v28.4s, v13.4s, v0.s[0]
        ldr q13, [x0, #0x240]
        str q14, [x0, #0x170]
        mul v14.4s, v18.4s, v1.s[0]
        mls v28.4s, v16.4s, v31.s[0]
        ldr q21, [x0, #0xc0]
        mls v10.4s, v11.4s, v31.s[0]
        mls v23.4s, v20.4s, v31.s[0]
        str q28, [x0, #0x1f0]
        ldr q11, [x0, #0x300]
        sqrdmulh v28.4s, v18.4s, v1.s[1]
        str q10, [x0, #0x370]
        mul v16.4s, v19.4s, v0.s[2]
        sqrdmulh v19.4s, v19.4s, v0.s[3]
        sqrdmulh v10.4s, v26.4s, v0.s[1]
        add v18.4s, v12.4s, v13.4s
        sub v12.4s, v12.4s, v13.4s
        mls v16.4s, v19.4s, v31.s[0]
        ldr q9, [x0, #0x280]
        ldr q20, [x0, #0x380]
        add v24.4s, v23.4s, v27.4s
        sub v25.4s, v23.4s, v27.4s
        mul v19.4s, v26.4s, v0.s[0]
        mls v14.4s, v28.4s, v31.s[0]
        add v13.4s, v9.4s, v17.4s
        sub v28.4s, v18.4s, v13.4s
        mls v19.4s, v10.4s, v31.s[0]
        sqrdmulh v23.4s, v25.4s, v0.s[1]
        mul v15.4s, v25.4s, v0.s[0]
        str q19, [x0, #0x2b0]
        ldr q19, [x0, #0x80]
        sqrdmulh v22.4s, v8.4s, v30.4s
        mls v15.4s, v23.4s, v31.s[0]
        add v26.4s, v16.4s, v14.4s
        mul v10.4s, v8.4s, v29.4s
        add v8.4s, v18.4s, v13.4s
        sqrdmulh v13.4s, v26.4s, v30.4s
        str q15, [x0, #0x2f0]
        sub v18.4s, v9.4s, v17.4s
        mul v27.4s, v26.4s, v29.4s
        sqrdmulh v23.4s, v12.4s, v5.s[3]
        mls v27.4s, v13.4s, v31.s[0]
        mul v25.4s, v18.4s, v6.s[0]
        sqrdmulh v13.4s, v18.4s, v6.s[1]
        str q27, [x0, #0x1b0]
        sub v27.4s, v19.4s, v21.4s
        add v21.4s, v19.4s, v21.4s
        mul v18.4s, v12.4s, v5.s[2]
        sub v9.4s, v16.4s, v14.4s
        mul v26.4s, v24.4s, v29.4s
        ldr q12, [x0, #0x3c0]
        mls v25.4s, v13.4s, v31.s[0]
        sub v16.4s, v20.4s, v12.4s
        sqrdmulh v17.4s, v24.4s, v30.4s
        mul v13.4s, v16.4s, v7.s[0]
        mls v18.4s, v23.4s, v31.s[0]
        mls v26.4s, v17.4s, v31.s[0]
        sqrdmulh v17.4s, v9.4s, v0.s[1]
        sqrdmulh v24.4s, v16.4s, v7.s[1]
        ldr q16, [x0, #0x340]
        str q26, [x0, #0xf0]
        sub v15.4s, v18.4s, v25.4s
        mul v23.4s, v9.4s, v0.s[0]
        ldr q19, [x0, #0x180]
        ldr q14, [x0, #0x1c0]
        sqrdmulh v9.4s, v15.4s, v2.s[3]
        add v20.4s, v20.4s, v12.4s
        add v12.4s, v11.4s, v16.4s
        mul v26.4s, v15.4s, v2.s[2]
        sub v16.4s, v11.4s, v16.4s
        sub v11.4s, v12.4s, v20.4s
        mls v23.4s, v17.4s, v31.s[0]
        add v20.4s, v12.4s, v20.4s
        sub v12.4s, v19.4s, v14.4s
        mul v17.4s, v27.4s, v4.s[0]
        mls v10.4s, v22.4s, v31.s[0]
        add v22.4s, v19.4s, v14.4s
        ldr q14, [x0, #0x40]
        str q23, [x0, #0x3b0]
        sqrdmulh v23.4s, v27.4s, v4.s[1]
        ldr q19, [x0, #0x100]
        ldr q15, [x0, #0x140]
        mls v26.4s, v9.4s, v31.s[0]
        str q10, [x0, #0xb0]
        sqrdmulh v10.4s, v11.4s, v3.s[1]
        sub v27.4s, v8.4s, v20.4s
        mls v13.4s, v24.4s, v31.s[0]
        add v8.4s, v8.4s, v20.4s
        sub v20.4s, v19.4s, v15.4s
        sqrdmulh v9.4s, v16.4s, v6.s[3]
        add v19.4s, v19.4s, v15.4s
        ldr q15, [x0]
        mul v24.4s, v20.4s, v4.s[2]
        mls v17.4s, v23.4s, v31.s[0]
        add v23.4s, v15.4s, v14.4s
        sub v14.4s, v15.4s, v14.4s
        mul v15.4s, v11.4s, v3.s[0]
        sub v11.4s, v23.4s, v21.4s
        mul v16.4s, v16.4s, v6.s[2]
        add v23.4s, v23.4s, v21.4s
        add v21.4s, v18.4s, v25.4s
        sqrdmulh v25.4s, v20.4s, v4.s[3]
        sqrdmulh v18.4s, v12.4s, v5.s[1]
        sqrdmulh v20.4s, v14.4s, v3.s[3]
        mul v12.4s, v12.4s, v5.s[0]
        mls v16.4s, v9.4s, v31.s[0]
        mls v12.4s, v18.4s, v31.s[0]
        mls v24.4s, v25.4s, v31.s[0]
        sub v25.4s, v16.4s, v13.4s
        add v16.4s, v16.4s, v13.4s
        mul v13.4s, v28.4s, v2.s[2]
        sqrdmulh v9.4s, v25.4s, v3.s[1]
        mul v18.4s, v25.4s, v3.s[0]
        sqrdmulh v25.4s, v28.4s, v2.s[3]
        mls v18.4s, v9.4s, v31.s[0]
        mls v15.4s, v10.4s, v31.s[0]
        sub v9.4s, v24.4s, v12.4s
        mul v28.4s, v14.4s, v3.s[2]
        add v12.4s, v24.4s, v12.4s
        mls v28.4s, v20.4s, v31.s[0]
        add v20.4s, v26.4s, v18.4s
        add v24.4s, v19.4s, v22.4s
        mls v13.4s, v25.4s, v31.s[0]
        sub v18.4s, v26.4s, v18.4s
        sqrdmulh v26.4s, v11.4s, v1.s[3]
        add v25.4s, v23.4s, v24.4s
        mul v14.4s, v11.4s, v1.s[2]
        sub v10.4s, v28.4s, v17.4s
        add v17.4s, v28.4s, v17.4s
        sub v28.4s, v23.4s, v24.4s
        sqrdmulh v11.4s, v9.4s, v2.s[1]
        sub v22.4s, v19.4s, v22.4s
        sub v24.4s, v17.4s, v12.4s
        mls v14.4s, v26.4s, v31.s[0]
        add v17.4s, v17.4s, v12.4s
        sqrdmulh v12.4s, v22.4s, v2.s[1]
        sub v23.4s, v13.4s, v15.4s
        add v26.4s, v13.4s, v15.4s
        mul v19.4s, v9.4s, v2.s[0]
        sqrdmulh v9.4s, v23.4s, v1.s[1]
        mul v15.4s, v23.4s, v1.s[0]
        mls v19.4s, v11.4s, v31.s[0]
        sub v13.4s, v25.4s, v8.4s
        mls v15.4s, v9.4s, v31.s[0]
        add v23.4s, v25.4s, v8.4s
        sqrdmulh v8.4s, v24.4s, v0.s[3]
        mul v25.4s, v22.4s, v2.s[0]
        subs x4, x4, #0x1
        cbnz x4, Lmldsa_intt_layer1234_start
        mul v22.4s, v24.4s, v0.s[2]
        sqrdmulh v9.4s, v23.4s, v30.4s
        mls v25.4s, v12.4s, v31.s[0]
        mul v23.4s, v23.4s, v29.4s
        mls v23.4s, v9.4s, v31.s[0]
        mls v22.4s, v8.4s, v31.s[0]
        sqrdmulh v11.4s, v10.4s, v1.s[3]
        sub v9.4s, v14.4s, v25.4s
        str q23, [x0], #0x10
        mul v23.4s, v10.4s, v1.s[2]
        sqrdmulh v24.4s, v9.4s, v0.s[3]
        mls v23.4s, v11.4s, v31.s[0]
        mul v9.4s, v9.4s, v0.s[2]
        mls v9.4s, v24.4s, v31.s[0]
        sub v24.4s, v23.4s, v19.4s
        add v19.4s, v23.4s, v19.4s
        mul v8.4s, v28.4s, v0.s[2]
        add v23.4s, v14.4s, v25.4s
        mul v11.4s, v13.4s, v0.s[0]
        sub v25.4s, v21.4s, v16.4s
        sub v10.4s, v9.4s, v15.4s
        add v15.4s, v9.4s, v15.4s
        add v16.4s, v21.4s, v16.4s
        sqrdmulh v28.4s, v28.4s, v0.s[3]
        add v9.4s, v23.4s, v26.4s
        add v14.4s, v17.4s, v16.4s
        sqrdmulh v21.4s, v13.4s, v0.s[1]
        sub v13.4s, v17.4s, v16.4s
        mul v16.4s, v14.4s, v29.4s
        sub v12.4s, v23.4s, v26.4s
        mul v17.4s, v13.4s, v0.s[0]
        mls v8.4s, v28.4s, v31.s[0]
        sqrdmulh v28.4s, v14.4s, v30.4s
        sub v14.4s, v19.4s, v20.4s
        add v20.4s, v19.4s, v20.4s
        sqrdmulh v19.4s, v13.4s, v0.s[1]
        sqrdmulh v26.4s, v27.4s, v1.s[1]
        mls v16.4s, v28.4s, v31.s[0]
        sqrdmulh v28.4s, v12.4s, v0.s[1]
        mul v27.4s, v27.4s, v1.s[0]
        str q16, [x0, #0x30]
        mul v12.4s, v12.4s, v0.s[0]
        mls v12.4s, v28.4s, v31.s[0]
        mls v27.4s, v26.4s, v31.s[0]
        sqrdmulh v26.4s, v25.4s, v1.s[1]
        str q12, [x0, #0x270]
        sqrdmulh v12.4s, v14.4s, v0.s[1]
        sub v13.4s, v8.4s, v27.4s
        add v27.4s, v8.4s, v27.4s
        mul v16.4s, v14.4s, v0.s[0]
        mul v8.4s, v20.4s, v29.4s
        mls v16.4s, v12.4s, v31.s[0]
        mul v12.4s, v18.4s, v1.s[0]
        sqrdmulh v14.4s, v20.4s, v30.4s
        str q16, [x0, #0x2b0]
        sqrdmulh v20.4s, v18.4s, v1.s[1]
        mul v25.4s, v25.4s, v1.s[0]
        mls v25.4s, v26.4s, v31.s[0]
        mls v17.4s, v19.4s, v31.s[0]
        mul v18.4s, v9.4s, v29.4s
        add v28.4s, v22.4s, v25.4s
        sqrdmulh v9.4s, v9.4s, v30.4s
        sub v23.4s, v22.4s, v25.4s
        str q17, [x0, #0x230]
        mls v11.4s, v21.4s, v31.s[0]
        mls v8.4s, v14.4s, v31.s[0]
        sqrdmulh v21.4s, v28.4s, v30.4s
        str q11, [x0, #0x1f0]
        mul v17.4s, v28.4s, v29.4s
        str q8, [x0, #0xb0]
        mul v28.4s, v10.4s, v0.s[0]
        mls v17.4s, v21.4s, v31.s[0]
        sqrdmulh v21.4s, v10.4s, v0.s[1]
        mul v8.4s, v15.4s, v29.4s
        str q17, [x0, #0x130]
        sqrdmulh v15.4s, v15.4s, v30.4s
        mul v10.4s, v24.4s, v0.s[2]
        sqrdmulh v24.4s, v24.4s, v0.s[3]
        mls v12.4s, v20.4s, v31.s[0]
        mls v8.4s, v15.4s, v31.s[0]
        mls v10.4s, v24.4s, v31.s[0]
        sqrdmulh v24.4s, v27.4s, v30.4s
        str q8, [x0, #0x170]
        mul v14.4s, v13.4s, v0.s[0]
        sub v16.4s, v10.4s, v12.4s
        sqrdmulh v17.4s, v13.4s, v0.s[1]
        add v10.4s, v10.4s, v12.4s
        mul v13.4s, v27.4s, v29.4s
        sqrdmulh v20.4s, v10.4s, v30.4s
        mls v14.4s, v17.4s, v31.s[0]
        mls v13.4s, v24.4s, v31.s[0]
        sqrdmulh v24.4s, v23.4s, v0.s[1]
        str q14, [x0, #0x2f0]
        sqrdmulh v8.4s, v16.4s, v0.s[1]
        str q13, [x0, #0xf0]
        mls v18.4s, v9.4s, v31.s[0]
        mul v9.4s, v23.4s, v0.s[0]
        mul v14.4s, v16.4s, v0.s[0]
        str q18, [x0, #0x70]
        mul v27.4s, v10.4s, v29.4s
        mls v28.4s, v21.4s, v31.s[0]
        mls v14.4s, v8.4s, v31.s[0]
        mls v27.4s, v20.4s, v31.s[0]
        str q28, [x0, #0x370]
        mls v9.4s, v24.4s, v31.s[0]
        str q14, [x0, #0x3b0]
        str q27, [x0, #0x1b0]
        str q9, [x0, #0x330]
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

S2N_BN_SIZE_DIRECTIVE(mldsa_intt_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
