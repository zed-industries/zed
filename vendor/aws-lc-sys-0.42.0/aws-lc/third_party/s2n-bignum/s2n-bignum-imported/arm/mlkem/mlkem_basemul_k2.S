// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Scalar product of 2-element polynomial vectors in NTT domain, with mulcache
// Inputs a[512], b[512], bt[256] (signed 16-bit words); output r[256] (signed 16-bit words)
//
// The inputs a and b are considered as 2-element vectors of linear
// polynomials in the NTT domain (in Montgomery form), and the bt
// argument an analogous 2-element vector of mulcaches for the bi:
//
//   a0 = a[0..255], a1 = a[256..511]
//   b0 = b[0..255], b1 = b[256..511]
//   bt0 = bt[0..127], bt1 = bt[128..255]
//
// Scalar multiplication of those 2-element vectors is performed,
// with base multiplication in Fq[X]/(X^2-zeta^i'), with zeta^i'
// being a power of zeta = 17, with i bit-reversed as used for NTTs,
// making use of the mulcache for optimization.
//
// All input elements are assumed <= 2^12 and the bts are
// assumed to be as computed by mlkem_mulcache_compute.
//
// extern void mlkem_basemul_k2
//      (int16_t r[static 256],const int16_t a[static 512],
//       const int16_t b[static 512], const int16_t bt[static 256]);
//
// Standard ARM ABI: X0 = r, X1 = a, X2 = b, X3 = bt
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_basemul_k2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_basemul_k2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_basemul_k2)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_basemul_k2):
        CFI_START

// This matches the code in the mlkem-native repository
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/polyvec_basemul_acc_montgomery_cached_asm_k2.S

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
        mov     x13, #0x10
        ldr     q9, [x4], #0x20
        ldur    q5, [x4, #-0x10]
        ldr     q11, [x5], #0x20
        uzp1    v23.8h, v9.8h, v5.8h
        uzp2    v9.8h, v9.8h, v5.8h
        ldr     q5, [x2], #0x20
        ldur    q7, [x5, #-0x10]
        ldur    q21, [x2, #-0x10]
        uzp2    v10.8h, v11.8h, v7.8h
        uzp1    v11.8h, v11.8h, v7.8h
        uzp1    v7.8h, v5.8h, v21.8h
        uzp2    v5.8h, v5.8h, v21.8h
        ldr     q21, [x1], #0x20
        ldur    q25, [x1, #-0x10]
        ld1     { v6.8h }, [x3], #16
        uzp1    v26.8h, v21.8h, v25.8h
        uzp2    v21.8h, v21.8h, v25.8h
        smull   v25.4s, v26.4h, v5.4h
        smull2  v5.4s, v26.8h, v5.8h
        smull   v19.4s, v26.4h, v7.4h
        smull2  v26.4s, v26.8h, v7.8h
        smlal   v25.4s, v21.4h, v7.4h
        smlal2  v5.4s, v21.8h, v7.8h
        smlal   v19.4s, v21.4h, v6.4h
        smlal2  v26.4s, v21.8h, v6.8h
        smlal   v25.4s, v23.4h, v10.4h
        smlal2  v5.4s, v23.8h, v10.8h
        smlal   v19.4s, v23.4h, v11.4h
        smlal2  v26.4s, v23.8h, v11.8h
        ld1     { v23.8h }, [x6], #16
        smlal   v25.4s, v9.4h, v11.4h
        smlal2  v5.4s, v9.8h, v11.8h
        smlal2  v26.4s, v9.8h, v23.8h
        smlal   v19.4s, v9.4h, v23.4h
        ldr     q9, [x4], #0x20
        uzp1    v11.8h, v25.8h, v5.8h
        uzp1    v23.8h, v19.8h, v26.8h
        mul     v11.8h, v11.8h, v2.8h
        mul     v23.8h, v23.8h, v2.8h
        ldr     q7, [x5], #0x20
        smlal2  v5.4s, v11.8h, v0.8h
        smlal   v25.4s, v11.4h, v0.4h
        ldr     q11, [x2], #0x20
        ldur    q21, [x2, #-0x10]
        ldur    q6, [x4, #-0x10]
        uzp1    v17.8h, v11.8h, v21.8h
        ldr     q10, [x1], #0x20
        ldur    q29, [x1, #-0x10]
        uzp2    v11.8h, v11.8h, v21.8h
        uzp1    v13.8h, v9.8h, v6.8h
        uzp1    v3.8h, v10.8h, v29.8h
        uzp2    v10.8h, v10.8h, v29.8h
        smull   v12.4s, v3.4h, v11.4h
        smull2  v11.4s, v3.8h, v11.8h
        ldur    q21, [x5, #-0x10]
        smlal   v12.4s, v10.4h, v17.4h
        smlal2  v11.4s, v10.8h, v17.8h
        uzp2    v29.8h, v7.8h, v21.8h
        uzp1    v15.8h, v7.8h, v21.8h
        smlal   v12.4s, v13.4h, v29.4h
        smlal2  v11.4s, v13.8h, v29.8h
        uzp2    v28.8h, v9.8h, v6.8h
        smlal2  v26.4s, v23.8h, v0.8h
        smlal   v12.4s, v28.4h, v15.4h
        smlal2  v11.4s, v28.8h, v15.8h
        smlal   v19.4s, v23.4h, v0.4h
        uzp2    v27.8h, v25.8h, v5.8h
        smull   v23.4s, v3.4h, v17.4h
        uzp1    v9.8h, v12.8h, v11.8h
        uzp2    v19.8h, v19.8h, v26.8h
        mul     v14.8h, v9.8h, v2.8h
        ld1     { v22.8h }, [x6], #16
        zip2    v9.8h, v19.8h, v27.8h
        smlal2  v11.4s, v14.8h, v0.8h
        ld1     { v4.8h }, [x3], #16
        sub     x13, x13, #0x2

Lmlkem_basemul_k2_loop:
        smull2  v20.4s, v3.8h, v17.8h
        ldr     q18, [x4], #0x20
        ldr     q30, [x5], #0x20
        smlal2  v20.4s, v10.8h, v4.8h
        smlal   v12.4s, v14.4h, v0.4h
        smlal   v23.4s, v10.4h, v4.4h
        str     q9, [x0, #0x10]
        smlal2  v20.4s, v13.8h, v15.8h
        ldr     q8, [x2], #0x20
        smlal   v23.4s, v13.4h, v15.4h
        smlal2  v20.4s, v28.8h, v22.8h
        zip1    v26.8h, v19.8h, v27.8h
        ldur    q9, [x2, #-0x10]
        smlal   v23.4s, v28.4h, v22.4h
        uzp2    v27.8h, v12.8h, v11.8h
        uzp1    v17.8h, v8.8h, v9.8h
        uzp2    v4.8h, v8.8h, v9.8h
        uzp1    v5.8h, v23.8h, v20.8h
        str     q26, [x0], #0x20
        mul     v31.8h, v5.8h, v2.8h
        ldur    q19, [x4, #-0x10]
        ldr     q29, [x1], #0x20
        ldur    q12, [x1, #-0x10]
        smlal2  v20.4s, v31.8h, v0.8h
        uzp1    v13.8h, v18.8h, v19.8h
        uzp1    v3.8h, v29.8h, v12.8h
        uzp2    v10.8h, v29.8h, v12.8h
        smull   v12.4s, v3.4h, v4.4h
        smull2  v11.4s, v3.8h, v4.8h
        ldur    q5, [x5, #-0x10]
        smlal   v12.4s, v10.4h, v17.4h
        smlal2  v11.4s, v10.8h, v17.8h
        uzp2    v14.8h, v30.8h, v5.8h
        uzp1    v15.8h, v30.8h, v5.8h
        smlal   v12.4s, v13.4h, v14.4h
        smlal2  v11.4s, v13.8h, v14.8h
        uzp2    v28.8h, v18.8h, v19.8h
        smlal   v23.4s, v31.4h, v0.4h
        smlal   v12.4s, v28.4h, v15.4h
        smlal2  v11.4s, v28.8h, v15.8h
        ld1     { v22.8h }, [x6], #16
        uzp2    v19.8h, v23.8h, v20.8h
        uzp1    v1.8h, v12.8h, v11.8h
        smull   v23.4s, v3.4h, v17.4h
        mul     v14.8h, v1.8h, v2.8h
        zip2    v9.8h, v19.8h, v27.8h
        ld1     { v4.8h }, [x3], #16
        smlal2  v11.4s, v14.8h, v0.8h
        sub     x13, x13, #0x1
        cbnz    x13, Lmlkem_basemul_k2_loop
        smull2  v5.4s, v3.8h, v17.8h
        smlal   v12.4s, v14.4h, v0.4h
        smlal   v23.4s, v10.4h, v4.4h
        str     q9, [x0, #0x10]
        smlal2  v5.4s, v10.8h, v4.8h
        uzp2    v11.8h, v12.8h, v11.8h
        zip1    v9.8h, v19.8h, v27.8h
        smlal   v23.4s, v13.4h, v15.4h
        smlal2  v5.4s, v13.8h, v15.8h
        str     q9, [x0], #0x20
        smlal   v23.4s, v28.4h, v22.4h
        smlal2  v5.4s, v28.8h, v22.8h
        uzp1    v9.8h, v23.8h, v5.8h
        mul     v9.8h, v9.8h, v2.8h
        smlal2  v5.4s, v9.8h, v0.8h
        smlal   v23.4s, v9.4h, v0.4h
        uzp2    v9.8h, v23.8h, v5.8h
        zip2    v5.8h, v9.8h, v11.8h
        zip1    v9.8h, v9.8h, v11.8h
        str     q5, [x0, #0x10]
        str     q9, [x0], #0x20
        CFI_STACKLOAD2(d8,d9,0)
        CFI_STACKLOAD2(d10,d11,0x10)
        CFI_STACKLOAD2(d12,d13,0x20)
        CFI_STACKLOAD2(d14,d15,0x30)
        CFI_INC_SP(64)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_basemul_k2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
