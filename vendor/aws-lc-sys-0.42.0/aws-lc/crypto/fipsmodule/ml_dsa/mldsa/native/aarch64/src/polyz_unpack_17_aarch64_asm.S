/*
 * Copyright (c) The mldsa-native project authors
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

 #include "_internal_s2n_bignum_arm.h"

/*
 * WARNING: This file is auto-derived from the mldsa-native source file
 *   dev/aarch64_opt/src/polyz_unpack_17_aarch64_asm.S using scripts/simpasm. Do not modify it directly.
 */

.text
.balign 4
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_polyz_unpack_17_aarch64_asm)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_polyz_unpack_17_aarch64_asm)
S2N_BN_SYMBOL(mldsa_polyz_unpack_17_aarch64_asm):

        .cfi_startproc
        ldr q24, [x2]
        ldr q25, [x2, #0x10]
        ldr q26, [x2, #0x20]
        ldr q27, [x2, #0x30]
        mov x3, #0xfe00000000       // =1090921693184
        mov v28.d[0], x3
        mov x3, #0xfc               // =252
        movk x3, #0xfa, lsl #32
        mov v28.d[1], x3
        movi v29.4s, #0x3, msl #16
        movi v30.4s, #0x2, lsl #16
        mov x9, #0x10               // =16

Lmldsa_polyz_unpack_17_loop:
        ld1 { v0.16b, v1.16b }, [x1]
        add x1, x1, #0x14
        ld1 { v2.16b }, [x1], #16
        tbl v4.16b, { v0.16b }, v24.16b
        tbl v5.16b, { v0.16b, v1.16b }, v25.16b
        tbl v6.16b, { v1.16b }, v26.16b
        tbl v7.16b, { v1.16b, v2.16b }, v27.16b
        ushl v4.4s, v4.4s, v28.4s
        and v4.16b, v4.16b, v29.16b
        sub v4.4s, v30.4s, v4.4s
        ushl v5.4s, v5.4s, v28.4s
        and v5.16b, v5.16b, v29.16b
        sub v5.4s, v30.4s, v5.4s
        ushl v6.4s, v6.4s, v28.4s
        and v6.16b, v6.16b, v29.16b
        sub v6.4s, v30.4s, v6.4s
        ushl v7.4s, v7.4s, v28.4s
        and v7.16b, v7.16b, v29.16b
        sub v7.4s, v30.4s, v7.4s
        str q5, [x0, #0x10]
        str q6, [x0, #0x20]
        str q7, [x0, #0x30]
        str q4, [x0], #0x40
        subs x9, x9, #0x1
        b.ne Lmldsa_polyz_unpack_17_loop
        ret
        .cfi_endproc

S2N_BN_SIZE_DIRECTIVE(mldsa_polyz_unpack_17_aarch64_asm)


#if defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
