// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Scalar product of 3-element polynomial vectors in NTT domain, with mulcache
// Inputs a[768], b[768], bt[384] (signed 16-bit words); output r[256] (signed 16-bit words)
//
// The inputs a and b are considered as 3-element vectors of linear
// polynomials in the NTT domain (in Montgomery form), and the bt
// argument an analogous 3-element vector of mulcaches for the bi:
//
//   a0 = a[0..255], a1 = a[256..511], a2 = a[512..767]
//   b0 = b[0..255], b1 = b[256..511], b2 = b[512..767],
//   bt0 = bt[0..127], bt1 = bt[128..255], bt2 = bt[256..383]
//
// Scalar multiplication of those 3-element vectors is performed,
// with base multiplication in Fq[X]/(X^2-zeta^i'), with zeta^i'
// being a power of zeta = 17, with i bit-reversed as used for NTTs,
// making use of the mulcache for optimization.
//
// All input elements are assumed <= 2^12 and the bts are
// assumed to be as computed by mlkem_mulcache_compute.
//
// extern void mlkem_basemul_k3
//      (int16_t r[static 256],const int16_t a[static 768],
//       const int16_t b[static 768], const int16_t bt[static 384]);
//
// Standard ARM ABI: X0 = r, X1 = a, X2 = b, X3 = bt
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_basemul_k3)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_basemul_k3)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_basemul_k3)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_basemul_k3):
        CFI_START

// This matches the code in the mlkem-native repository
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/polyvec_basemul_acc_montgomery_cached_asm_k3.S

        CFI_DEC_SP(64)
        CFI_STACKSAVE2(d8,d9,0)
        CFI_STACKSAVE2(d10,d11,0x10)
        CFI_STACKSAVE2(d12,d13,0x20)
        CFI_STACKSAVE2(d14,d15,0x30)
        mov     w14, #0xd01
        dup     v0.8h, w14
        mov     w14, #0xcff
        dup     v2.8h, w14
        add     x4, x1, #0x200
        add     x5, x2, #0x200
        add     x6, x3, #0x100
        add     x7, x1, #0x400
        add     x8, x2, #0x400
        add     x9, x3, #0x200
        mov     x13, #0x10
        ldr     q7, [x2, #0x10]
        ldr     q20, [x2], #0x20
        ldr     q15, [x1, #0x10]
        uzp1    v8.8h, v20.8h, v7.8h
        uzp2    v7.8h, v20.8h, v7.8h
        ld1     { v20.8h }, [x3], #16
        ldr     q30, [x1], #0x20
        ldr     q11, [x4], #0x20
        uzp1    v16.8h, v30.8h, v15.8h
        uzp2    v15.8h, v30.8h, v15.8h
        smull   v30.4s, v16.4h, v7.4h
        smull2  v7.4s, v16.8h, v7.8h
        smull   v9.4s, v16.4h, v8.4h
        smull2  v16.4s, v16.8h, v8.8h
        smlal   v30.4s, v15.4h, v8.4h
        smlal2  v7.4s, v15.8h, v8.8h
        smlal   v9.4s, v15.4h, v20.4h
        smlal2  v16.4s, v15.8h, v20.8h
        ldur    q20, [x4, #-0x10]
        ldr     q15, [x5], #0x20
        uzp1    v8.8h, v11.8h, v20.8h
        uzp2    v20.8h, v11.8h, v20.8h
        ldur    q11, [x5, #-0x10]
        ld1     { v27.8h }, [x6], #16
        uzp1    v10.8h, v15.8h, v11.8h
        uzp2    v15.8h, v15.8h, v11.8h
        smlal   v9.4s, v8.4h, v10.4h
        smlal2  v16.4s, v8.8h, v10.8h
        smlal   v30.4s, v8.4h, v15.4h
        smlal2  v7.4s, v8.8h, v15.8h
        smlal   v9.4s, v20.4h, v27.4h
        smlal2  v16.4s, v20.8h, v27.8h
        smlal   v30.4s, v20.4h, v10.4h
        smlal2  v7.4s, v20.8h, v10.8h
        ldr     q20, [x7], #0x20
        ldur    q15, [x7, #-0x10]
        ldr     q8, [x8], #0x20
        uzp1    v11.8h, v20.8h, v15.8h
        uzp2    v20.8h, v20.8h, v15.8h
        ldur    q15, [x8, #-0x10]
        ld1     { v27.8h }, [x9], #16
        uzp1    v10.8h, v8.8h, v15.8h
        uzp2    v15.8h, v8.8h, v15.8h
        smlal   v9.4s, v11.4h, v10.4h
        smlal2  v16.4s, v11.8h, v10.8h
        smlal   v30.4s, v11.4h, v15.4h
        smlal2  v7.4s, v11.8h, v15.8h
        smlal   v9.4s, v20.4h, v27.4h
        smlal2  v16.4s, v20.8h, v27.8h
        smlal   v30.4s, v20.4h, v10.4h
        smlal2  v7.4s, v20.8h, v10.8h
        ldr     q15, [x2], #0x20
        uzp1    v20.8h, v9.8h, v16.8h
        uzp1    v8.8h, v30.8h, v7.8h
        mul     v20.8h, v20.8h, v2.8h
        mul     v8.8h, v8.8h, v2.8h
        ldr     q21, [x4], #0x20
        smlal   v9.4s, v20.4h, v0.4h
        smlal2  v16.4s, v20.8h, v0.8h
        smlal   v30.4s, v8.4h, v0.4h
        smlal2  v7.4s, v8.8h, v0.8h
        ldur    q6, [x4, #-0x10]
        uzp2    v27.8h, v9.8h, v16.8h
        uzp2    v10.8h, v30.8h, v7.8h
        ldur    q16, [x2, #-0x10]
        ldr     q30, [x1, #0x10]
        ld1     { v9.8h }, [x3], #16
        ldr     q1, [x5], #0x20
        ldur    q12, [x5, #-0x10]
        ld1     { v24.8h }, [x6], #16
        ldr     q19, [x7], #0x20
        ldur    q31, [x7, #-0x10]
        ldr     q17, [x8], #0x20
        ldur    q18, [x8, #-0x10]
        ld1     { v25.8h }, [x9], #16
        sub     x13, x13, #0x2

Lmlkem_basemul_k3_loop:
        ldr     q20, [x1], #0x20
        uzp1    v7.8h, v15.8h, v16.8h
        uzp2    v15.8h, v15.8h, v16.8h
        uzp1    v8.8h, v20.8h, v30.8h
        uzp2    v20.8h, v20.8h, v30.8h
        smull   v30.4s, v8.4h, v15.4h
        smull2  v15.4s, v8.8h, v15.8h
        smull   v11.4s, v8.4h, v7.4h
        smull2  v8.4s, v8.8h, v7.8h
        smlal   v30.4s, v20.4h, v7.4h
        smlal2  v15.4s, v20.8h, v7.8h
        smlal   v11.4s, v20.4h, v9.4h
        smlal2  v8.4s, v20.8h, v9.8h
        uzp1    v7.8h, v21.8h, v6.8h
        uzp2    v20.8h, v21.8h, v6.8h
        uzp1    v16.8h, v1.8h, v12.8h
        uzp2    v9.8h, v1.8h, v12.8h
        smlal   v11.4s, v7.4h, v16.4h
        smlal2  v8.4s, v7.8h, v16.8h
        smlal   v30.4s, v7.4h, v9.4h
        smlal2  v15.4s, v7.8h, v9.8h
        smlal   v11.4s, v20.4h, v24.4h
        smlal2  v8.4s, v20.8h, v24.8h
        smlal   v30.4s, v20.4h, v16.4h
        smlal2  v15.4s, v20.8h, v16.8h
        uzp1    v7.8h, v19.8h, v31.8h
        uzp2    v20.8h, v19.8h, v31.8h
        uzp1    v16.8h, v17.8h, v18.8h
        uzp2    v9.8h, v17.8h, v18.8h
        smlal   v11.4s, v7.4h, v16.4h
        smlal2  v8.4s, v7.8h, v16.8h
        smlal   v30.4s, v7.4h, v9.4h
        smlal2  v15.4s, v7.8h, v9.8h
        smlal   v11.4s, v20.4h, v25.4h
        smlal2  v8.4s, v20.8h, v25.8h
        smlal   v30.4s, v20.4h, v16.4h
        smlal2  v15.4s, v20.8h, v16.8h
        ldr     q16, [x2, #0x10]
        uzp1    v7.8h, v11.8h, v8.8h
        uzp1    v20.8h, v30.8h, v15.8h
        mul     v7.8h, v7.8h, v2.8h
        mul     v20.8h, v20.8h, v2.8h
        zip2    v9.8h, v27.8h, v10.8h
        zip1    v27.8h, v27.8h, v10.8h
        smlal   v11.4s, v7.4h, v0.4h
        smlal2  v8.4s, v7.8h, v0.8h
        smlal   v30.4s, v20.4h, v0.4h
        smlal2  v15.4s, v20.8h, v0.8h
        str     q27, [x0], #0x20
        uzp2    v27.8h, v11.8h, v8.8h
        stur    q9, [x0, #-0x10]
        uzp2    v10.8h, v30.8h, v15.8h
        ldr     q30, [x1, #0x10]
        ldr     q15, [x2], #0x20
        ld1     { v9.8h }, [x3], #16
        ldr     q21, [x4], #0x20
        ldur    q6, [x4, #-0x10]
        ldr     q1, [x5], #0x20
        ldur    q12, [x5, #-0x10]
        ld1     { v24.8h }, [x6], #16
        ldr     q19, [x7], #0x20
        ldur    q31, [x7, #-0x10]
        ldr     q17, [x8], #0x20
        ldur    q18, [x8, #-0x10]
        ld1     { v25.8h }, [x9], #16
        sub     x13, x13, #0x1
        cbnz    x13, Lmlkem_basemul_k3_loop
        ldr     q7, [x1], #0x20
        uzp1    v20.8h, v15.8h, v16.8h
        uzp2    v15.8h, v15.8h, v16.8h
        uzp1    v23.8h, v7.8h, v30.8h
        uzp2    v11.8h, v7.8h, v30.8h
        smull2  v8.4s, v23.8h, v20.8h
        smull   v5.4s, v23.4h, v20.4h
        smull2  v30.4s, v23.8h, v15.8h
        uzp1    v28.8h, v1.8h, v12.8h
        smlal2  v8.4s, v11.8h, v9.8h
        smlal   v5.4s, v11.4h, v9.4h
        uzp1    v3.8h, v21.8h, v6.8h
        smull   v16.4s, v23.4h, v15.4h
        smlal2  v8.4s, v3.8h, v28.8h
        smlal   v5.4s, v3.4h, v28.4h
        uzp2    v29.8h, v21.8h, v6.8h
        uzp1    v7.8h, v17.8h, v18.8h
        smlal2  v8.4s, v29.8h, v24.8h
        uzp1    v14.8h, v19.8h, v31.8h
        smlal   v16.4s, v11.4h, v20.4h
        smlal2  v30.4s, v11.8h, v20.8h
        smlal2  v8.4s, v14.8h, v7.8h
        uzp2    v20.8h, v1.8h, v12.8h
        uzp2    v21.8h, v19.8h, v31.8h
        smlal2  v30.4s, v3.8h, v20.8h
        smlal   v16.4s, v3.4h, v20.4h
        smlal   v5.4s, v29.4h, v24.4h
        uzp2    v9.8h, v17.8h, v18.8h
        smlal2  v30.4s, v29.8h, v28.8h
        smlal   v16.4s, v29.4h, v28.4h
        smlal   v5.4s, v14.4h, v7.4h
        smlal2  v8.4s, v21.8h, v25.8h
        smlal2  v30.4s, v14.8h, v9.8h
        smlal   v16.4s, v14.4h, v9.4h
        smlal   v5.4s, v21.4h, v25.4h
        zip1    v20.8h, v27.8h, v10.8h
        smlal2  v30.4s, v21.8h, v7.8h
        smlal   v16.4s, v21.4h, v7.4h
        uzp1    v7.8h, v5.8h, v8.8h
        str     q20, [x0], #0x20
        mul     v15.8h, v7.8h, v2.8h
        uzp1    v7.8h, v16.8h, v30.8h
        zip2    v31.8h, v27.8h, v10.8h
        mul     v20.8h, v7.8h, v2.8h
        smlal   v5.4s, v15.4h, v0.4h
        smlal2  v8.4s, v15.8h, v0.8h
        stur    q31, [x0, #-0x10]
        smlal2  v30.4s, v20.8h, v0.8h
        smlal   v16.4s, v20.4h, v0.4h
        uzp2    v15.8h, v5.8h, v8.8h
        uzp2    v20.8h, v16.8h, v30.8h
        zip1    v7.8h, v15.8h, v20.8h
        zip2    v20.8h, v15.8h, v20.8h
        str     q7, [x0], #0x20
        stur    q20, [x0, #-0x10]
        CFI_STACKLOAD2(d8,d9,0)
        CFI_STACKLOAD2(d10,d11,0x10)
        CFI_STACKLOAD2(d12,d13,0x20)
        CFI_STACKLOAD2(d14,d15,0x30)
        CFI_INC_SP(64)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_basemul_k3)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
