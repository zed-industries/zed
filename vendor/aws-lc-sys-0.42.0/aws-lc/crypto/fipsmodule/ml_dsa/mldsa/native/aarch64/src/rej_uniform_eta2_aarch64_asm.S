/*
 * Copyright (c) The mldsa-native project authors
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/rej_uniform_eta2_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_rej_uniform_eta2_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_rej_uniform_eta2_aarch64_asm)
S2N_BN_SYMBOL(mldsa_rej_uniform_eta2_aarch64_asm):

        .cfi_startproc
        sub sp, sp, #0x240
        .cfi_adjust_cfa_offset 0x240
        mov x7, #0x1                // =1
        movk x7, #0x2, lsl #16
        movk x7, #0x4, lsl #32
        movk x7, #0x8, lsl #48
        mov v31.d[0], x7
        mov x7, #0x10               // =16
        movk x7, #0x20, lsl #16
        movk x7, #0x40, lsl #32
        movk x7, #0x80, lsl #48
        mov v31.d[1], x7
        movi v30.8h, #0xf
        mov x8, sp
        mov x7, x8
        mov x11, #0x0               // =0
        eor v16.16b, v16.16b, v16.16b

Lmldsa_rej_uniform_eta2_initial_zero:
        str q16, [x7], #0x40
        stur q16, [x7, #-0x30]
        stur q16, [x7, #-0x20]
        stur q16, [x7, #-0x10]
        add x11, x11, #0x20
        cmp x11, #0x100
        b.lt Lmldsa_rej_uniform_eta2_initial_zero
        mov x7, x8
        mov x9, #0x0                // =0
        mov x4, #0x100              // =256

Lmldsa_rej_uniform_eta2_loop8:
        cmp x9, x4
        b.hs Lmldsa_rej_uniform_eta2_memory_copy
        sub x2, x2, #0x8
        ld1 { v0.8b }, [x1], #8
        movi v26.8b, #0xf
        and v27.8b, v0.8b, v26.8b
        ushr v28.8b, v0.8b, #0x4
        zip1 v26.8b, v27.8b, v28.8b
        zip2 v29.8b, v27.8b, v28.8b
        ushll v16.8h, v26.8b, #0x0
        ushll v17.8h, v29.8b, #0x0
        cmhi v4.8h, v30.8h, v16.8h
        cmhi v5.8h, v30.8h, v17.8h
        and v4.16b, v4.16b, v31.16b
        and v5.16b, v5.16b, v31.16b
        uaddlv s20, v4.8h
        uaddlv s21, v5.8h
        fmov w12, s20
        fmov w13, s21
        ldr q24, [x3, x12, lsl #4]
        ldr q25, [x3, x13, lsl #4]
        cnt v4.16b, v4.16b
        cnt v5.16b, v5.16b
        uaddlv s20, v4.8h
        uaddlv s21, v5.8h
        fmov w12, s20
        fmov w13, s21
        tbl v16.16b, { v16.16b }, v24.16b
        tbl v17.16b, { v17.16b }, v25.16b
        st1 { v16.8h }, [x7]
        add x7, x7, x12, lsl #1
        st1 { v17.8h }, [x7]
        add x7, x7, x13, lsl #1
        add x12, x12, x13
        add x9, x9, x12
        cmp x2, #0x8
        b.hs Lmldsa_rej_uniform_eta2_loop8

Lmldsa_rej_uniform_eta2_memory_copy:
        cmp x9, x4
        csel x9, x9, x4, lo
        mov w7, #0x199a             // =6554
        dup v26.8h, w7
        movi v27.8h, #0x5
        movi v7.8h, #0x2
        mov x11, #0x0               // =0
        mov x7, x8

Lmldsa_rej_uniform_eta2_final_copy:
        ldr q16, [x7], #0x20
        ldur q18, [x7, #-0x10]
        sqdmulh v28.8h, v16.8h, v26.8h
        mls v16.8h, v28.8h, v27.8h
        sqdmulh v28.8h, v18.8h, v26.8h
        mls v18.8h, v28.8h, v27.8h
        sub v16.8h, v7.8h, v16.8h
        sub v18.8h, v7.8h, v18.8h
        sshll2 v17.4s, v16.8h, #0x0
        sshll v16.4s, v16.4h, #0x0
        sshll2 v19.4s, v18.8h, #0x0
        sshll v18.4s, v18.4h, #0x0
        str q16, [x0], #0x40
        stur q17, [x0, #-0x30]
        stur q18, [x0, #-0x20]
        stur q19, [x0, #-0x10]
        add x11, x11, #0x10
        cmp x11, #0x100
        b.lt Lmldsa_rej_uniform_eta2_final_copy
        mov x0, x9
        add sp, sp, #0x240
        .cfi_adjust_cfa_offset -0x240
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_rej_uniform_eta2_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
