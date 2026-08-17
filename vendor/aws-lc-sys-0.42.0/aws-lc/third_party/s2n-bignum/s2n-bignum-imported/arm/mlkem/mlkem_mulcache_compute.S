// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Precompute the mulcache data for a polynomial in the NTT domain
// Inputs a[256], z[128] and t[128] (signed 16-bit words); output x[128] (signed 16-bit words)
//
// The input array a is assumed to represent 128 linear polynomials
// in the NTT domain, p_i = a[2i] + a[2i+1] * X where each p_i is in
// Fq[X]/(X^2-zeta^i'), with zeta^i' being a power of zeta = 17, with i
// bit-reversed as used for NTTs. For each such polynomial, the mulcache
// value is a[2i+1] * zeta^i' (modulo 3329 as usual), a value useful to
// perform base multiplication of polynomials efficiently. The two other
// table arguments z = zetas and t = twisted zetas are expected to point
// to tables of zeta-related constants whose definitions can be found in
// the mlkem-native repo (mlkem/native/aarch64/src/aarch64_zetas.c) or
// our "tests/test.c", as "mulcache_zetas" and "mulcache_zetas_twisted"
//
// extern void mlkem_mulcache_compute
//      (int16_t x[static 128],const int16_t a[static 256],
//       const int16_t z[static 128],const int16_t t[static 128]);
//
// Standard ARM ABI: X0 = x, X1 = a, X2 = z, X3 = t
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_mulcache_compute)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_mulcache_compute)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_mulcache_compute)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_mulcache_compute):
        CFI_START

// This matches the code in the mlkem-native repository
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/poly_mulcache_compute_asm.S

        mov     w5, #0xd01
        dup     v6.8h, w5
        mov     w5, #0x4ebf
        dup     v7.8h, w5
        mov     x4, #0x10
        ldr     q1, [x1, #0x10]
        ldr     q27, [x1], #0x20
        ldr     q23, [x2], #0x10
        uzp2    v27.8h, v27.8h, v1.8h
        ldr     q1, [x3], #0x10
        mul     v2.8h, v27.8h, v23.8h
        sqrdmulh        v27.8h, v27.8h, v1.8h
        sub     x4, x4, #0x1

mlkem_mulcache_compute_loop:
        ldr     q29, [x1, #0x10]
        ldr     q21, [x2], #0x10
        mls     v2.8h, v27.8h, v6.h[0]
        ldr     q27, [x1], #0x20
        ldr     q7, [x3], #0x10
        uzp2    v28.8h, v27.8h, v29.8h
        str     q2, [x0], #0x10
        mul     v2.8h, v28.8h, v21.8h
        sqrdmulh        v27.8h, v28.8h, v7.8h
        sub     x4, x4, #0x1
        cbnz    x4, mlkem_mulcache_compute_loop

        mls     v2.8h, v27.8h, v6.h[0]
        str     q2, [x0], #0x10
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_mulcache_compute)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
