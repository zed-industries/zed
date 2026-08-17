// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Conversion of ML-KEM polynomial coefficients to Montgomery form
// Input a[256] (signed 16-bit words); output a[256] (signed 16-bit words)
//
// This converts each element of the 256-element array of 16-bit signed
// integers modulo 3329 into Montgomery form, giving a signed result
// satisfying (output[i] == 2^16 * input[i]) (mod 3329), without full
// modular reduction but with |output[i]| < 3329 guaranteed.
//
// extern void mlkem_tomont(int16_t a[static 256]);
//
// Standard ARM ABI: X0 = a
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_tomont)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_tomont)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_tomont)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_tomont):
        CFI_START

// This matches the code in the mlkem-native repository
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/poly_tomont_asm.S

        mov     w2, #0xd01
        dup     v4.8h, w2
        mov     w2, #0x4ebf
        dup     v5.8h, w2
        mov     w2, #-0x414
        dup     v2.8h, w2
        mov     w2, #-0x2824
        dup     v3.8h, w2
        mov     x1, #0x8
        ldr     q26, [x0, #0x30]
        ldr     q23, [x0, #0x10]
        mul     v17.8h, v26.8h, v2.8h
        sqrdmulh v7.8h, v26.8h, v3.8h
        ldr     q27, [x0, #0x20]
        sub     x1, x1, #0x1

mlkem_tomont_loop:
        mls     v17.8h, v7.8h, v4.h[0]
        sqrdmulh v5.8h, v23.8h, v3.8h
        ldr     q7, [x0], #0x40
        stur    q17, [x0, #-0x10]
        sqrdmulh v29.8h, v27.8h, v3.8h
        sqrdmulh v19.8h, v7.8h, v3.8h
        mul     v25.8h, v23.8h, v2.8h
        mul     v0.8h, v7.8h, v2.8h
        mul     v26.8h, v27.8h, v2.8h
        ldr     q7, [x0, #0x30]
        mls     v25.8h, v5.8h, v4.h[0]
        ldr     q23, [x0, #0x10]
        mls     v26.8h, v29.8h, v4.h[0]
        mls     v0.8h, v19.8h, v4.h[0]
        stur    q25, [x0, #-0x30]
        mul     v17.8h, v7.8h, v2.8h
        sqrdmulh v7.8h, v7.8h, v3.8h
        stur    q0, [x0, #-0x40]
        ldr     q27, [x0, #0x20]
        stur    q26, [x0, #-0x20]
        sub     x1, x1, #0x1
        cbnz    x1, mlkem_tomont_loop

        mls     v17.8h, v7.8h, v4.h[0]
        sqrdmulh v7.8h, v23.8h, v3.8h
        mul     v26.8h, v23.8h, v2.8h
        sqrdmulh v25.8h, v27.8h, v3.8h
        ldr     q23, [x0], #0x40
        mul     v27.8h, v27.8h, v2.8h
        mls     v26.8h, v7.8h, v4.h[0]
        sqrdmulh v7.8h, v23.8h, v3.8h
        mul     v23.8h, v23.8h, v2.8h
        stur    q17, [x0, #-0x10]
        mls     v27.8h, v25.8h, v4.h[0]
        stur    q26, [x0, #-0x30]
        mls     v23.8h, v7.8h, v4.h[0]
        stur    q27, [x0, #-0x20]
        stur    q23, [x0, #-0x40]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_tomont)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
