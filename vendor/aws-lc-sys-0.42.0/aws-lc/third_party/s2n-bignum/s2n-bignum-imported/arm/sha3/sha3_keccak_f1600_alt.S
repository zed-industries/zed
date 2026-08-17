// Copyright (c) 2024 The mlkem-native project authors
// Copyright (c) 2021-2022 Arm Limited
// Copyright (c) 2022 Matthias Kannwischer
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Keccak-f1600 permutation for SHA3
// Input a[25], rc[24]; output a[25]
//
// Thinking of the input/output array a as a row-major flattening of a
// 5x5 matrix of 64-bit words, this performs the Keccak-f1600 permutation,
// all 24 rounds with the distinct round constants rc[i] for each one. For
// correct operation, the input pointer rc should point at the standard
// round constants as in the specification:
//
//   https://keccak.team/keccak_specs_summary.html#roundConstants
//
// This operation is at the core of SHA3 and is fully specified here:
//
//   https://keccak.team/files/Keccak-reference-3.0.pdf
//
// extern void sha3_keccak_f1600_alt(uint64_t a[static 25],
//                                   const uint64_t rc[static 24]);
//
// Standard ARM ABI: X0 = a, X1 = rc
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        .arch armv8.4-a+sha3
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sha3_keccak_f1600_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sha3_keccak_f1600_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sha3_keccak_f1600_alt)
        .text
        .balign 4

S2N_BN_SYMBOL(sha3_keccak_f1600_alt):
        CFI_START

// This is very similar to the vector code in the mlkem-native
// repository here:
//
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/fips202/native/aarch64/src/keccak_f1600_x1_v84a_asm.S
//
// The main difference is the use of ldp/stp dx,dy in place of
// ld2, the variant used being currently unsupported by the
// s2n-bignum formal model.

        CFI_DEC_SP(64)
        CFI_STACKSAVE2X(d8,d9,0,8)
        CFI_STACKSAVE2X(d10,d11,16,24)
        CFI_STACKSAVE2X(d12,d13,32,40)
        CFI_STACKSAVE2X(d14,d15,48,56)

// Load the Keccak initial state into registers Q0..Q24

        ldp     d0, d1, [x0]
        ldp     d2, d3, [x0, #0x10]
        ldp     d4, d5, [x0, #0x20]
        ldp     d6, d7, [x0, #0x30]
        ldp     d8, d9, [x0, #0x40]
        ldp     d10, d11, [x0, #0x50]
        ldp     d12, d13, [x0, #0x60]
        ldp     d14, d15, [x0, #0x70]
        ldp     d16, d17, [x0, #0x80]
        ldp     d18, d19, [x0, #0x90]
        ldp     d20, d21, [x0, #0xa0]
        ldp     d22, d23, [x0, #0xb0]
        ldr     d24, [x0, #0xc0]

// Now 24 rounds of the iteration

        mov     x2, #24

Lsha3_keccak_f1600_alt_loop:
        eor3    v30.16b, v0.16b, v5.16b, v10.16b
        eor3    v29.16b, v1.16b, v6.16b, v11.16b
        eor3    v28.16b, v2.16b, v7.16b, v12.16b
        eor3    v27.16b, v3.16b, v8.16b, v13.16b
        eor3    v26.16b, v4.16b, v9.16b, v14.16b
        eor3    v30.16b, v30.16b, v15.16b, v20.16b
        eor3    v29.16b, v29.16b, v16.16b, v21.16b
        eor3    v28.16b, v28.16b, v17.16b, v22.16b
        eor3    v27.16b, v27.16b, v18.16b, v23.16b
        eor3    v26.16b, v26.16b, v19.16b, v24.16b
        rax1    v25.2d, v30.2d, v28.2d
        rax1    v28.2d, v28.2d, v26.2d
        rax1    v26.2d, v26.2d, v29.2d
        rax1    v29.2d, v29.2d, v27.2d
        rax1    v27.2d, v27.2d, v30.2d
        eor     v30.16b, v0.16b, v26.16b
        xar     v0.2d, v2.2d, v29.2d, #0x2
        xar     v2.2d, v12.2d, v29.2d, #0x15
        xar     v12.2d, v13.2d, v28.2d, #0x27
        xar     v13.2d, v19.2d, v27.2d, #0x38
        xar     v19.2d, v23.2d, v28.2d, #0x8
        xar     v23.2d, v15.2d, v26.2d, #0x17
        xar     v15.2d, v1.2d, v25.2d, #0x3f
        xar     v1.2d, v8.2d, v28.2d, #0x9
        xar     v8.2d, v16.2d, v25.2d, #0x13
        xar     v16.2d, v7.2d, v29.2d, #0x3a
        xar     v7.2d, v10.2d, v26.2d, #0x3d
        xar     v10.2d, v3.2d, v28.2d, #0x24
        xar     v3.2d, v18.2d, v28.2d, #0x2b
        xar     v18.2d, v17.2d, v29.2d, #0x31
        xar     v17.2d, v11.2d, v25.2d, #0x36
        xar     v11.2d, v9.2d, v27.2d, #0x2c
        xar     v9.2d, v22.2d, v29.2d, #0x3
        xar     v22.2d, v14.2d, v27.2d, #0x19
        xar     v14.2d, v20.2d, v26.2d, #0x2e
        xar     v20.2d, v4.2d, v27.2d, #0x25
        xar     v4.2d, v24.2d, v27.2d, #0x32
        xar     v24.2d, v21.2d, v25.2d, #0x3e
        xar     v21.2d, v5.2d, v26.2d, #0x1c
        xar     v27.2d, v6.2d, v25.2d, #0x14
        ld1r    { v31.2d }, [x1], #8
        bcax    v5.16b, v10.16b, v7.16b, v11.16b
        bcax    v6.16b, v11.16b, v8.16b, v7.16b
        bcax    v7.16b, v7.16b, v9.16b, v8.16b
        bcax    v8.16b, v8.16b, v10.16b, v9.16b
        bcax    v9.16b, v9.16b, v11.16b, v10.16b
        bcax    v10.16b, v15.16b, v12.16b, v16.16b
        bcax    v11.16b, v16.16b, v13.16b, v12.16b
        bcax    v12.16b, v12.16b, v14.16b, v13.16b
        bcax    v13.16b, v13.16b, v15.16b, v14.16b
        bcax    v14.16b, v14.16b, v16.16b, v15.16b
        bcax    v15.16b, v20.16b, v17.16b, v21.16b
        bcax    v16.16b, v21.16b, v18.16b, v17.16b
        bcax    v17.16b, v17.16b, v19.16b, v18.16b
        bcax    v18.16b, v18.16b, v20.16b, v19.16b
        bcax    v19.16b, v19.16b, v21.16b, v20.16b
        bcax    v20.16b, v0.16b, v22.16b, v1.16b
        bcax    v21.16b, v1.16b, v23.16b, v22.16b
        bcax    v22.16b, v22.16b, v24.16b, v23.16b
        bcax    v23.16b, v23.16b, v0.16b, v24.16b
        bcax    v24.16b, v24.16b, v1.16b, v0.16b
        bcax    v0.16b, v30.16b, v2.16b, v27.16b
        bcax    v1.16b, v27.16b, v3.16b, v2.16b
        bcax    v2.16b, v2.16b, v4.16b, v3.16b
        bcax    v3.16b, v3.16b, v30.16b, v4.16b
        bcax    v4.16b, v4.16b, v27.16b, v30.16b
        eor     v0.16b, v0.16b, v31.16b
        sub     x2, x2, #0x1
        cbnz    x2, Lsha3_keccak_f1600_alt_loop

// Store back the state

        stp     d0, d1, [x0]
        stp     d2, d3, [x0, #0x10]
        stp     d4, d5, [x0, #0x20]
        stp     d6, d7, [x0, #0x30]
        stp     d8, d9, [x0, #0x40]
        stp     d10, d11, [x0, #0x50]
        stp     d12, d13, [x0, #0x60]
        stp     d14, d15, [x0, #0x70]
        stp     d16, d17, [x0, #0x80]
        stp     d18, d19, [x0, #0x90]
        stp     d20, d21, [x0, #0xa0]
        stp     d22, d23, [x0, #0xb0]
        str     d24, [x0, #0xc0]

// Restore registers and return

        CFI_STACKLOAD2(d8,d9,0)
        CFI_STACKLOAD2(d10,d11,16)
        CFI_STACKLOAD2(d12,d13,32)
        CFI_STACKLOAD2(d14,d15,48)
        CFI_INC_SP(64)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(sha3_keccak_f1600_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
