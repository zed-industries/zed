// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Pack ML-KEM polynomial coefficients as 12-bit numbers
// Input a[256] (signed 16-bit words); output r[384] (bytes)
//
// This accepts an array of 256 16-bit numbers assumed to be in the range
// 0 <= a[i] < 2^12 (typically they will be < 3329, the ML-KEM prime).
// It packs them into the output array as 12-bit unsigned numbers.
//
// extern void mlkem_tobytes(uint8_t r[static 384],const int16_t a[static 256]);
//
// Standard ARM ABI: X0 = r, X1 = a
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_tobytes)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_tobytes)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_tobytes)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_tobytes):
        CFI_START

// This code is essentially a verbatim copy of the mlkem-native version
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/poly_tobytes_asm.S

        mov     x2, #0x10
        ldr     q6, [x1], #0x20
        ldur    q24, [x1, #-0x10]
        ldr     q30, [x1], #0x20
        ldur    q22, [x1, #-0x10]
        ldr     q5, [x1], #0x20
        ldur    q17, [x1, #-0x10]
        ldr     q19, [x1], #0x20
        ldur    q4, [x1, #-0x10]
        lsr     x2, x2, #2
        sub     x2, x2, #0x1

mlkem_tobytes_asm_asm_loop_start:
        uzp1    v25.8h, v6.8h, v24.8h
        uzp2    v6.8h, v6.8h, v24.8h
        xtn     v24.8b, v25.8h
        shrn    v25.8b, v25.8h, #0x8
        xtn     v18.8b, v6.8h
        shrn    v26.8b, v6.8h, #0x4
        sli     v25.8b, v18.8b, #0x4
        st3     { v24.8b, v25.8b, v26.8b }, [x0], #24
        uzp1    v25.8h, v30.8h, v22.8h
        uzp2    v6.8h, v30.8h, v22.8h
        xtn     v24.8b, v25.8h
        xtn     v18.8b, v6.8h
        uzp1    v30.8h, v5.8h, v17.8h
        uzp2    v22.8h, v5.8h, v17.8h
        xtn     v5.8b, v30.8h
        xtn     v17.8b, v22.8h
        uzp1    v28.8h, v19.8h, v4.8h
        uzp2    v19.8h, v19.8h, v4.8h
        xtn     v4.8b, v28.8h
        xtn     v20.8b, v19.8h
        shrn    v25.8b, v25.8h, #0x8
        sli     v25.8b, v18.8b, #0x4
        shrn    v26.8b, v6.8h, #0x4
        st3     { v24.8b, v25.8b, v26.8b }, [x0], #24
        shrn    v6.8b, v30.8h, #0x8
        sli     v6.8b, v17.8b, #0x4
        shrn    v7.8b, v22.8h, #0x4
        st3     { v5.8b, v6.8b, v7.8b }, [x0], #24
        shrn    v5.8b, v28.8h, #0x8
        shrn    v6.8b, v19.8h, #0x4
        sli     v5.8b, v20.8b, #0x4
        st3     { v4.8b, v5.8b, v6.8b }, [x0], #24
        ldr     q6, [x1], #0x20
        ldur    q24, [x1, #-0x10]
        ldr     q30, [x1], #0x20
        ldur    q22, [x1, #-0x10]
        ldr     q5, [x1], #0x20
        ldur    q17, [x1, #-0x10]
        ldr     q19, [x1], #0x20
        ldur    q4, [x1, #-0x10]
        sub     x2, x2, #0x1
        cbnz    x2, mlkem_tobytes_asm_asm_loop_start
        uzp1    v25.8h, v30.8h, v22.8h
        uzp2    v18.8h, v30.8h, v22.8h
        uzp1    v30.8h, v6.8h, v24.8h
        uzp2    v6.8h, v6.8h, v24.8h
        uzp1    v24.8h, v5.8h, v17.8h
        uzp2    v22.8h, v5.8h, v17.8h
        uzp1    v5.8h, v19.8h, v4.8h
        uzp2    v17.8h, v19.8h, v4.8h
        xtn     v19.8b, v25.8h
        shrn    v20.8b, v25.8h, #0x8
        xtn     v25.8b, v18.8h
        shrn    v21.8b, v18.8h, #0x4
        xtn     v28.8b, v30.8h
        shrn    v29.8b, v30.8h, #0x8
        xtn     v18.8b, v6.8h
        shrn    v30.8b, v6.8h, #0x4
        xtn     v1.8b, v24.8h
        shrn    v2.8b, v24.8h, #0x8
        xtn     v6.8b, v22.8h
        shrn    v3.8b, v22.8h, #0x4
        xtn     v22.8b, v5.8h
        shrn    v23.8b, v5.8h, #0x8
        xtn     v5.8b, v17.8h
        shrn    v24.8b, v17.8h, #0x4
        sli     v20.8b, v25.8b, #0x4
        sli     v29.8b, v18.8b, #0x4
        st3     { v28.8b, v29.8b, v30.8b }, [x0], #24
        st3     { v19.8b, v20.8b, v21.8b }, [x0], #24
        sli     v2.8b, v6.8b, #0x4
        st3     { v1.8b, v2.8b, v3.8b }, [x0], #24
        sli     v23.8b, v5.8b, #0x4
        st3     { v22.8b, v23.8b, v24.8b }, [x0], #24
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_tobytes)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
