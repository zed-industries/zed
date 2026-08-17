// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Scalar product of 4-element polynomial vectors in NTT domain, with mulcache
// Inputs a[1024], b[1024], bt[512] (signed 16-bit words); output r[256] (signed 16-bit words)
//
// The inputs a and b are considered as 4-element vectors of linear
// polynomials in the NTT domain (in Montgomery form), and the bt
// argument an analogous 4-element vector of mulcaches for the bi:
//
//   a0 = a[0..255], a1 = a[256..511], a2 = a[512..767], a3 = a[768..1023]
//   b0 = b[0..255], b1 = b[256..511], b2 = b[512..767], b3 = b[768..1023]
//   bt0 = bt[0..127], bt1 = bt[128..255], bt2 = bt[256..383], bt3 = bt[384..511]
//
// Scalar multiplication of those 4-element vectors is performed,
// with base multiplication in Fq[X]/(X^2-zeta^i'), with zeta^i'
// being a power of zeta = 17, with i bit-reversed as used for NTTs,
// making use of the mulcache for optimization.
//
// All input elements are assumed <= 2^12 and the bts are
// assumed to be as computed by mlkem_mulcache_compute.
//
// extern void mlkem_basemul_k4
//      (int16_t r[static 256],const int16_t a[static 1024],
//       const int16_t b[static 1024], const int16_t bt[static 512]);
//
// Standard ARM ABI: X0 = r, X1 = a, X2 = b, X3 = bt
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_basemul_k4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_basemul_k4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_basemul_k4)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_basemul_k4):
        CFI_START

// This matches the code in the mlkem-native repository
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/polyvec_basemul_acc_montgomery_cached_asm_k4.S

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
        add     x10, x1, #0x600
        add     x11, x2, #0x600
        add     x12, x3, #0x300
        mov     x13, #0x10
        ldr     q23, [x2, #0x10]
        ldr     q19, [x2], #0x20
        ldr     q17, [x5], #0x20
        uzp2    v13.8h, v19.8h, v23.8h
        uzp1    v19.8h, v19.8h, v23.8h
        ldur    q23, [x5, #-0x10]
        ldr     q30, [x1, #0x10]
        uzp2    v9.8h, v17.8h, v23.8h
        uzp1    v23.8h, v17.8h, v23.8h
        ldr     q17, [x1], #0x20
        ldr     q10, [x7, #0x10]
        uzp1    v12.8h, v17.8h, v30.8h
        uzp2    v17.8h, v17.8h, v30.8h
        smull2  v30.4s, v12.8h, v13.8h
        smull   v13.4s, v12.4h, v13.4h
        smull2  v22.4s, v12.8h, v19.8h
        smull   v12.4s, v12.4h, v19.4h
        smlal2  v30.4s, v17.8h, v19.8h
        smlal   v13.4s, v17.4h, v19.4h
        ldr     q19, [x4], #0x20
        ldur    q16, [x4, #-0x10]
        ld1     { v8.8h }, [x3], #16
        uzp1    v26.8h, v19.8h, v16.8h
        uzp2    v19.8h, v19.8h, v16.8h
        smlal2  v30.4s, v26.8h, v9.8h
        smlal   v13.4s, v26.4h, v9.4h
        smlal2  v22.4s, v17.8h, v8.8h
        smlal   v12.4s, v17.4h, v8.4h
        smlal2  v30.4s, v19.8h, v23.8h
        smlal   v13.4s, v19.4h, v23.4h
        smlal2  v22.4s, v26.8h, v23.8h
        smlal   v12.4s, v26.4h, v23.4h
        ldr     q23, [x7], #0x20
        ldr     q17, [x8, #0x10]
        uzp1    v9.8h, v23.8h, v10.8h
        uzp2    v23.8h, v23.8h, v10.8h
        ldr     q10, [x10], #0x20
        ldur    q16, [x10, #-0x10]
        ld1     { v8.8h }, [x12], #16
        uzp1    v26.8h, v10.8h, v16.8h
        uzp2    v10.8h, v10.8h, v16.8h
        ld1     { v16.8h }, [x6], #16
        ldr     q3, [x11, #0x10]
        smlal2  v22.4s, v19.8h, v16.8h
        smlal   v12.4s, v19.4h, v16.4h
        ldr     q19, [x11], #0x20
        ld1     { v16.8h }, [x9], #16
        uzp1    v4.8h, v19.8h, v3.8h
        uzp2    v19.8h, v19.8h, v3.8h
        ldr     q3, [x8], #0x20
        ldr     q31, [x2], #0x20
        uzp1    v6.8h, v3.8h, v17.8h
        uzp2    v17.8h, v3.8h, v17.8h
        smlal2  v22.4s, v9.8h, v6.8h
        smlal2  v30.4s, v9.8h, v17.8h
        smlal   v13.4s, v9.4h, v17.4h
        smlal   v12.4s, v9.4h, v6.4h
        smlal2  v22.4s, v23.8h, v16.8h
        smlal2  v30.4s, v23.8h, v6.8h
        smlal   v13.4s, v23.4h, v6.4h
        smlal   v12.4s, v23.4h, v16.4h
        smlal2  v22.4s, v26.8h, v4.8h
        smlal2  v30.4s, v26.8h, v19.8h
        smlal   v13.4s, v26.4h, v19.4h
        smlal   v12.4s, v26.4h, v4.4h
        smlal2  v22.4s, v10.8h, v8.8h
        smlal2  v30.4s, v10.8h, v4.8h
        smlal   v13.4s, v10.4h, v4.4h
        smlal   v12.4s, v10.4h, v8.4h
        ldur    q19, [x2, #-0x10]
        uzp1    v23.8h, v13.8h, v30.8h
        uzp1    v17.8h, v12.8h, v22.8h
        mul     v23.8h, v23.8h, v2.8h
        uzp2    v21.8h, v31.8h, v19.8h
        uzp1    v19.8h, v31.8h, v19.8h
        mul     v17.8h, v17.8h, v2.8h
        smlal   v13.4s, v23.4h, v0.4h
        smlal2  v30.4s, v23.8h, v0.8h
        ldr     q23, [x5], #0x20
        smlal2  v22.4s, v17.8h, v0.8h
        uzp2    v15.8h, v13.8h, v30.8h
        smlal   v12.4s, v17.4h, v0.4h
        ldur    q17, [x5, #-0x10]
        ldr     q13, [x1, #0x10]
        uzp2    v27.8h, v23.8h, v17.8h
        uzp1    v28.8h, v23.8h, v17.8h
        uzp2    v7.8h, v12.8h, v22.8h
        ldr     q23, [x1], #0x20
        zip1    v5.8h, v7.8h, v15.8h
        ldr     q3, [x7, #0x10]
        uzp1    v31.8h, v23.8h, v13.8h
        uzp2    v16.8h, v23.8h, v13.8h
        smull2  v24.4s, v31.8h, v21.8h
        ldr     q6, [x8, #0x10]
        ldr     q23, [x10], #0x20
        smlal2  v24.4s, v16.8h, v19.8h
        ldur    q17, [x10, #-0x10]
        ld1     { v22.8h }, [x12], #16
        uzp1    v30.8h, v23.8h, v17.8h
        uzp2    v11.8h, v23.8h, v17.8h
        ldr     q23, [x4], #0x20
        ldur    q17, [x4, #-0x10]
        ldr     q4, [x7], #0x20
        uzp1    v20.8h, v23.8h, v17.8h
        uzp2    v26.8h, v23.8h, v17.8h
        uzp1    v9.8h, v4.8h, v3.8h
        smlal2  v24.4s, v20.8h, v27.8h
        ld1     { v8.8h }, [x6], #16
        ldr     q25, [x11, #0x10]
        ldr     q29, [x11], #0x20
        ld1     { v12.8h }, [x9], #16
        uzp1    v10.8h, v29.8h, v25.8h
        ldr     q14, [x8], #0x20
        ld1     { v23.8h }, [x3], #16
        sub     x13, x13, #0x2

Lmlkem_basemul_k4_loop:
        smlal2  v24.4s, v26.8h, v28.8h
        uzp2    v4.8h, v4.8h, v3.8h
        smull2  v13.4s, v31.8h, v19.8h
        ldr     q3, [x2], #0x20
        uzp2    v1.8h, v29.8h, v25.8h
        smlal2  v13.4s, v16.8h, v23.8h
        ldur    q17, [x2, #-0x10]
        smull   v18.4s, v31.4h, v19.4h
        smlal2  v13.4s, v20.8h, v28.8h
        smull   v29.4s, v31.4h, v21.4h
        ldr     q21, [x5], #0x20
        smlal2  v13.4s, v26.8h, v8.8h
        smlal   v29.4s, v16.4h, v19.4h
        ldur    q19, [x5, #-0x10]
        smlal   v18.4s, v16.4h, v23.4h
        smlal   v29.4s, v20.4h, v27.4h
        uzp1    v31.8h, v14.8h, v6.8h
        uzp2    v27.8h, v21.8h, v19.8h
        smlal   v18.4s, v20.4h, v28.4h
        ldr     q25, [x1, #0x10]
        smlal   v29.4s, v26.4h, v28.4h
        smlal   v18.4s, v26.4h, v8.4h
        uzp2    v26.8h, v14.8h, v6.8h
        smlal2  v13.4s, v9.8h, v31.8h
        smlal2  v24.4s, v9.8h, v26.8h
        smlal   v29.4s, v9.4h, v26.4h
        smlal   v18.4s, v9.4h, v31.4h
        smlal2  v13.4s, v4.8h, v12.8h
        smlal2  v24.4s, v4.8h, v31.8h
        smlal   v29.4s, v4.4h, v31.4h
        smlal   v18.4s, v4.4h, v12.4h
        smlal2  v13.4s, v30.8h, v10.8h
        smlal2  v24.4s, v30.8h, v1.8h
        smlal   v29.4s, v30.4h, v1.4h
        smlal   v18.4s, v30.4h, v10.4h
        smlal2  v13.4s, v11.8h, v22.8h
        smlal2  v24.4s, v11.8h, v10.8h
        smlal   v29.4s, v11.4h, v10.4h
        smlal   v18.4s, v11.4h, v22.4h
        ldr     q22, [x1], #0x20
        uzp1    v31.8h, v29.8h, v24.8h
        uzp1    v28.8h, v21.8h, v19.8h
        mul     v19.8h, v31.8h, v2.8h
        uzp1    v31.8h, v22.8h, v25.8h
        uzp2    v16.8h, v22.8h, v25.8h
        uzp2    v21.8h, v3.8h, v17.8h
        smlal   v29.4s, v19.4h, v0.4h
        smlal2  v24.4s, v19.8h, v0.8h
        uzp1    v19.8h, v3.8h, v17.8h
        uzp1    v26.8h, v18.8h, v13.8h
        zip2    v14.8h, v7.8h, v15.8h
        mul     v23.8h, v26.8h, v2.8h
        uzp2    v15.8h, v29.8h, v24.8h
        smull2  v24.4s, v31.8h, v21.8h
        str     q14, [x0, #0x10]
        ldr     q3, [x7, #0x10]
        ldr     q6, [x8, #0x10]
        ldr     q8, [x10], #0x20
        ldur    q26, [x10, #-0x10]
        ld1     { v22.8h }, [x12], #16
        uzp1    v30.8h, v8.8h, v26.8h
        uzp2    v11.8h, v8.8h, v26.8h
        ldr     q8, [x4], #0x20
        ldur    q26, [x4, #-0x10]
        ldr     q4, [x7], #0x20
        uzp1    v20.8h, v8.8h, v26.8h
        uzp2    v26.8h, v8.8h, v26.8h
        ld1     { v8.8h }, [x6], #16
        uzp1    v9.8h, v4.8h, v3.8h
        ldr     q25, [x11, #0x10]
        ldr     q29, [x11], #0x20
        ld1     { v12.8h }, [x9], #16
        ldr     q14, [x8], #0x20
        smlal2  v24.4s, v16.8h, v19.8h
        smlal2  v13.4s, v23.8h, v0.8h
        smlal   v18.4s, v23.4h, v0.4h
        ld1     { v23.8h }, [x3], #16
        smlal2  v24.4s, v20.8h, v27.8h
        uzp2    v7.8h, v18.8h, v13.8h
        uzp1    v10.8h, v29.8h, v25.8h
        str     q5, [x0], #0x20
        zip1    v5.8h, v7.8h, v15.8h
        sub     x13, x13, #0x1
        cbnz    x13, Lmlkem_basemul_k4_loop
        smull2  v17.4s, v31.8h, v19.8h
        uzp2    v1.8h, v14.8h, v6.8h
        smull   v18.4s, v31.4h, v21.4h
        smlal2  v24.4s, v26.8h, v28.8h
        smlal2  v17.4s, v16.8h, v23.8h
        smull   v21.4s, v31.4h, v19.4h
        smlal   v18.4s, v16.4h, v19.4h
        uzp2    v31.8h, v4.8h, v3.8h
        uzp1    v3.8h, v14.8h, v6.8h
        smlal   v21.4s, v16.4h, v23.4h
        smlal   v18.4s, v20.4h, v27.4h
        uzp2    v14.8h, v29.8h, v25.8h
        smlal2  v17.4s, v20.8h, v28.8h
        smlal   v21.4s, v20.4h, v28.4h
        smlal   v18.4s, v26.4h, v28.4h
        smlal2  v24.4s, v9.8h, v1.8h
        smlal2  v17.4s, v26.8h, v8.8h
        smlal   v21.4s, v26.4h, v8.4h
        smlal   v18.4s, v9.4h, v1.4h
        smlal2  v24.4s, v31.8h, v3.8h
        smlal2  v17.4s, v9.8h, v3.8h
        smlal   v21.4s, v9.4h, v3.4h
        smlal   v18.4s, v31.4h, v3.4h
        smlal2  v24.4s, v30.8h, v14.8h
        smlal2  v17.4s, v31.8h, v12.8h
        smlal   v21.4s, v31.4h, v12.4h
        smlal   v18.4s, v30.4h, v14.4h
        smlal2  v24.4s, v11.8h, v10.8h
        smlal2  v17.4s, v30.8h, v10.8h
        smlal   v21.4s, v30.4h, v10.4h
        smlal   v18.4s, v11.4h, v10.4h
        zip2    v19.8h, v7.8h, v15.8h
        smlal2  v17.4s, v11.8h, v22.8h
        smlal   v21.4s, v11.4h, v22.4h
        uzp1    v23.8h, v18.8h, v24.8h
        str     q19, [x0, #0x10]
        mul     v19.8h, v23.8h, v2.8h
        uzp1    v23.8h, v21.8h, v17.8h
        str     q5, [x0], #0x20
        mul     v26.8h, v23.8h, v2.8h
        smlal   v18.4s, v19.4h, v0.4h
        smlal2  v24.4s, v19.8h, v0.8h
        smlal   v21.4s, v26.4h, v0.4h
        smlal2  v17.4s, v26.8h, v0.8h
        uzp2    v13.8h, v18.8h, v24.8h
        uzp2    v19.8h, v21.8h, v17.8h
        zip1    v23.8h, v19.8h, v13.8h
        zip2    v19.8h, v19.8h, v13.8h
        str     q23, [x0], #0x20
        stur    q19, [x0, #-0x10]
        CFI_STACKLOAD2(d8,d9,0)
        CFI_STACKLOAD2(d10,d11,0x10)
        CFI_STACKLOAD2(d12,d13,0x20)
        CFI_STACKLOAD2(d14,d15,0x30)
        CFI_INC_SP(64)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_basemul_k4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
