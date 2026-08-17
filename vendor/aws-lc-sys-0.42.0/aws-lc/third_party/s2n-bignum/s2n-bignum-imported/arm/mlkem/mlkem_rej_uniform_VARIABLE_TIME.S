// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Uniform rejection sampling for ML-KEM
// Inputs *buf (unsigned bytes), buflen, table (unsigned bytes); output r[256] (signed 16-bit words), return
//
// extern uint64_t mlkem_rej_uniform_VARIABLE_TIME
//                     (int16_t r[S2N_BIGNUM_STATIC 256],
//                      const uint8_t *buf,uint64_t buflen,
//                      const uint8_t *table);
//
// Interprets the input buffer as packed 12-bit numbers with a length of
// buflen bytes, assumed to be a multiple of 24. Fills the output array
// with those numbers from the packed buffer that are < 3329, in the order
// of appearance, returning the total number of entries written, with a
// maximum of 256. The table argument is a specific precomputed table of
// constants that is defined in this file (see also our test code):
//
//   https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/rej_uniform_table.c
//
// Unique (at the moment) among s2n-bignum functions this is *not* a
// constant-time function. The time taken depends not only on the
// buffer size "buflen", but also how many elements of the buffer are
// needed to provide the 256 entries for the output.
//
// Standard ARM ABI: X0 = buf, X1 = r, X2 = buflen, X3 = table
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_rej_uniform_VARIABLE_TIME):
        CFI_START

// This is almost identical to the code from mlkem-native:
//
//   https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/rej_uniform_asm.S
//
// The only difference is systematic use of full-length scalar registers
// Xnn instead of the mixed use of 32-bit counterparts Wnn in most
// settings where that is applicable.

        CFI_DEC_SP(576)
        mov     x7, #0x1
        movk    x7, #0x2, lsl #16
        movk    x7, #0x4, lsl #32
        movk    x7, #0x8, lsl #48
        mov     v31.d[0], x7
        mov     x7, #0x10
        movk    x7, #0x20, lsl #16
        movk    x7, #0x40, lsl #32
        movk    x7, #0x80, lsl #48
        mov     v31.d[1], x7
        mov     w11, #0xd01
        dup     v30.8h, w11
        mov     x8, sp
        mov     x7, x8
        mov     w11, #0x0
        eor     v16.16b, v16.16b, v16.16b
Lmlkem_rej_uniform_initial_zero:
        str     q16, [x7], #0x40
        stur    q16, [x7, #-0x30]
        stur    q16, [x7, #-0x20]
        stur    q16, [x7, #-0x10]
        add     x11, x11, #0x20
        cmp     x11, #0x100
        b.lt    Lmlkem_rej_uniform_initial_zero
        mov     x7, x8
        mov     w9, #0x0
        mov     w4, #0x100
        cmp     x2, #0x30
        b.lo    Lmlkem_rej_uniform_loop48_end

Lmlkem_rej_uniform_loop48:
        cmp     x9, x4
        b.hs    Lmlkem_rej_uniform_memory_copy
        sub     x2, x2, #0x30
        ld3     { v0.16b, v1.16b, v2.16b }, [x1], #48
        zip1    v4.16b, v0.16b, v1.16b
        zip2    v5.16b, v0.16b, v1.16b
        zip1    v6.16b, v1.16b, v2.16b
        zip2    v7.16b, v1.16b, v2.16b
        bic     v4.8h, #0xf0, lsl #8
        bic     v5.8h, #0xf0, lsl #8
        ushr    v6.8h, v6.8h, #0x4
        ushr    v7.8h, v7.8h, #0x4
        zip1    v16.8h, v4.8h, v6.8h
        zip2    v17.8h, v4.8h, v6.8h
        zip1    v18.8h, v5.8h, v7.8h
        zip2    v19.8h, v5.8h, v7.8h
        cmhi    v4.8h, v30.8h, v16.8h
        cmhi    v5.8h, v30.8h, v17.8h
        cmhi    v6.8h, v30.8h, v18.8h
        cmhi    v7.8h, v30.8h, v19.8h
        and     v4.16b, v4.16b, v31.16b
        and     v5.16b, v5.16b, v31.16b
        and     v6.16b, v6.16b, v31.16b
        and     v7.16b, v7.16b, v31.16b
        uaddlv  s20, v4.8h
        uaddlv  s21, v5.8h
        uaddlv  s22, v6.8h
        uaddlv  s23, v7.8h
        fmov    w12, s20
        fmov    w13, s21
        fmov    w14, s22
        fmov    w15, s23
        ldr     q24, [x3, x12, lsl #4]
        ldr     q25, [x3, x13, lsl #4]
        ldr     q26, [x3, x14, lsl #4]
        ldr     q27, [x3, x15, lsl #4]
        cnt     v4.16b, v4.16b
        cnt     v5.16b, v5.16b
        cnt     v6.16b, v6.16b
        cnt     v7.16b, v7.16b
        uaddlv  s20, v4.8h
        uaddlv  s21, v5.8h
        uaddlv  s22, v6.8h
        uaddlv  s23, v7.8h
        fmov    w12, s20
        fmov    w13, s21
        fmov    w14, s22
        fmov    w15, s23
        tbl     v16.16b, { v16.16b }, v24.16b
        tbl     v17.16b, { v17.16b }, v25.16b
        tbl     v18.16b, { v18.16b }, v26.16b
        tbl     v19.16b, { v19.16b }, v27.16b
        str     q16, [x7]
        add     x7, x7, x12, lsl #1
        str     q17, [x7]
        add     x7, x7, x13, lsl #1
        str     q18, [x7]
        add     x7, x7, x14, lsl #1
        str     q19, [x7]
        add     x7, x7, x15, lsl #1
        add     x12, x12, x13
        add     x14, x14, x15
        add     x9, x9, x12
        add     x9, x9, x14
        cmp     x2, #0x30
        b.hs    Lmlkem_rej_uniform_loop48

Lmlkem_rej_uniform_loop48_end:
        cmp     x9, x4
        b.hs    Lmlkem_rej_uniform_memory_copy
        cmp     x2, #0x18
        b.lo    Lmlkem_rej_uniform_memory_copy
        sub     x2, x2, #0x18
        ld3     { v0.8b, v1.8b, v2.8b }, [x1], #24
        zip1    v4.16b, v0.16b, v1.16b
        zip1    v5.16b, v1.16b, v2.16b
        bic     v4.8h, #0xf0, lsl #8
        ushr    v5.8h, v5.8h, #0x4
        zip1    v16.8h, v4.8h, v5.8h
        zip2    v17.8h, v4.8h, v5.8h
        cmhi    v4.8h, v30.8h, v16.8h
        cmhi    v5.8h, v30.8h, v17.8h
        and     v4.16b, v4.16b, v31.16b
        and     v5.16b, v5.16b, v31.16b
        uaddlv  s20, v4.8h
        uaddlv  s21, v5.8h
        fmov    w12, s20
        fmov    w13, s21
        ldr     q24, [x3, x12, lsl #4]
        ldr     q25, [x3, x13, lsl #4]
        cnt     v4.16b, v4.16b
        cnt     v5.16b, v5.16b
        uaddlv  s20, v4.8h
        uaddlv  s21, v5.8h
        fmov    w12, s20
        fmov    w13, s21
        tbl     v16.16b, { v16.16b }, v24.16b
        tbl     v17.16b, { v17.16b }, v25.16b
        str     q16, [x7]
        add     x7, x7, x12, lsl #1
        str     q17, [x7]
        add     x7, x7, x13, lsl #1
        add     x9, x9, x12
        add     x9, x9, x13

Lmlkem_rej_uniform_memory_copy:
        cmp     x9, x4
        csel    x9, x9, x4, lo
        mov     x11, #0x0
        mov     x7, x8

Lmlkem_rej_uniform_final_copy:
        ldr     q16, [x7], #0x40
        ldur    q17, [x7, #-0x30]
        ldur    q18, [x7, #-0x20]
        ldur    q19, [x7, #-0x10]
        str     q16, [x0], #0x40
        stur    q17, [x0, #-0x30]
        stur    q18, [x0, #-0x20]
        stur    q19, [x0, #-0x10]
        add     x11, x11, #0x20
        cmp     x11, #0x100
        b.lt    Lmlkem_rej_uniform_final_copy
        mov     x0, x9
        b       Lmlkem_rej_uniform_return

Lmlkem_rej_uniform_return:
        CFI_INC_SP(576)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
