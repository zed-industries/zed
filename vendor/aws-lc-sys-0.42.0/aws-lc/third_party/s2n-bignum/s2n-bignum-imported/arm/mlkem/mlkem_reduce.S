// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Canonical reduction of polynomial coefficients for ML-KEM
// Input a[256] (signed 16-bit words); output a[256] (signed 16-bit words)
//
// This reduces each element of the 256-element array of 16-bit signed
// integers modulo 3329 with the result being 0 <= r < 3329, in-place.
// This is intended for use when that array represents polynomial
// coefficients for ML-KEM, but that is not relevant to its operation.
//
// extern void mlkem_reduce(int16_t a[static 256]);
//
// Standard ARM ABI: X0 = a
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_reduce)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_reduce)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_reduce)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_reduce):
        CFI_START

// This matches the code in the mlkem-native repository
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/poly_reduce_asm.S

        mov     w2, #0xd01
        dup     v3.8h, w2
        mov     w2, #0x4ebf
        dup     v4.8h, w2
        mov     x1, #0x8
        ldr     q21, [x0, #0x20]
        ldr     q23, [x0, #0x30]
        sqdmulh v7.8h, v21.8h, v4.h[0]
        sqdmulh v30.8h, v23.8h, v4.h[0]
        srshr   v7.8h, v7.8h, #0xb
        srshr   v30.8h, v30.8h, #0xb
        mls     v21.8h, v7.8h, v3.h[0]
        mls     v23.8h, v30.8h, v3.h[0]
        ldr     q5, [x0, #0x10]
        sshr    v7.8h, v21.8h, #0xf
        sshr    v30.8h, v23.8h, #0xf
        and     v7.16b, v3.16b, v7.16b
        add     v21.8h, v21.8h, v7.8h
        and     v7.16b, v3.16b, v30.16b
        add     v16.8h, v23.8h, v7.8h
        sub     x1, x1, #0x1

mlkem_reduce_loop:
        ldr     q6, [x0], #0x40
        ldr     q30, [x0, #0x20]
        sqdmulh v31.8h, v6.8h, v4.h[0]
        sqdmulh v29.8h, v5.8h, v4.h[0]
        sqdmulh v22.8h, v30.8h, v4.h[0]
        stur    q16, [x0, #-0x10]
        srshr   v20.8h, v31.8h, #0xb
        srshr   v28.8h, v29.8h, #0xb
        stur    q21, [x0, #-0x20]
        mls     v6.8h, v20.8h, v3.h[0]
        mls     v5.8h, v28.8h, v3.h[0]
        ldr     q2, [x0, #0x30]
        sshr    v31.8h, v6.8h, #0xf
        srshr   v19.8h, v22.8h, #0xb
        and     v22.16b, v3.16b, v31.16b
        add     v0.8h, v6.8h, v22.8h
        mls     v30.8h, v19.8h, v3.h[0]
        sshr    v26.8h, v5.8h, #0xf
        sqdmulh v25.8h, v2.8h, v4.h[0]
        and     v17.16b, v3.16b, v26.16b
        add     v1.8h, v5.8h, v17.8h
        sshr    v31.8h, v30.8h, #0xf
        srshr   v25.8h, v25.8h, #0xb
        stur    q1, [x0, #-0x30]
        and     v18.16b, v3.16b, v31.16b
        mls     v2.8h, v25.8h, v3.h[0]
        add     v21.8h, v30.8h, v18.8h
        ldr     q5, [x0, #0x10]
        sshr    v18.8h, v2.8h, #0xf
        stur    q0, [x0, #-0x40]
        and     v27.16b, v3.16b, v18.16b
        add     v16.8h, v2.8h, v27.8h
        sub     x1, x1, #0x1
        cbnz    x1, mlkem_reduce_loop
        sqdmulh v20.8h, v5.8h, v4.h[0]
        ldr     q24, [x0], #0x40
        stur    q21, [x0, #-0x20]
        srshr   v20.8h, v20.8h, #0xb
        sqdmulh v25.8h, v24.8h, v4.h[0]
        stur    q16, [x0, #-0x10]
        mls     v5.8h, v20.8h, v3.h[0]
        srshr   v20.8h, v25.8h, #0xb
        sshr    v2.8h, v5.8h, #0xf
        mls     v24.8h, v20.8h, v3.h[0]
        and     v20.16b, v3.16b, v2.16b
        add     v31.8h, v5.8h, v20.8h
        sshr    v20.8h, v24.8h, #0xf
        stur    q31, [x0, #-0x30]
        and     v31.16b, v3.16b, v20.16b
        add     v24.8h, v24.8h, v31.8h
        stur    q24, [x0, #-0x40]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_reduce)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
