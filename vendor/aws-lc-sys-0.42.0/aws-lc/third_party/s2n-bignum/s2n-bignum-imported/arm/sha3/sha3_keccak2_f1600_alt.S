// Copyright (c) 2024 The mlkem-native project authors
// Copyright (c) 2021-2022 Arm Limited
// Copyright (c) 2022 Matthias Kannwischer
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Keccak-f1600 permutation for SHA3, batch of two independent operations
// Input a[50], rc[24]; output a[50]
//
// The input/output argument is in effect two 25-element Keccak arrays
// a[0...24] and a[25..49], which could be considered as type a[25][2].
//
// Thinking of each such input/output array as a row-major flattening of a
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
// extern void sha3_keccak2_f1600_alt(uint64_t a[static 50],
//                                    const uint64_t rc[static 24]);
//
// Standard ARM ABI: X0 = a, X1 = rc
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        .arch armv8.4-a+sha3
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sha3_keccak2_f1600_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sha3_keccak2_f1600_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sha3_keccak2_f1600_alt)
        .text
        .balign 4

S2N_BN_SYMBOL(sha3_keccak2_f1600_alt):
        CFI_START

// This is similar to the code in the mlkem-native repository here:
//
//   mlkem/fips202/native/aarch64/src/keccak_f1600_x2_v8a_v84a_asm_hybrid.S
//
// The main difference is the avoidance of ld2/st2 in favour of explicit
// transposition operations and conventional loads and stores. We also
// avoid saving and restoring some unused integer registers.

        CFI_DEC_SP(64)
        CFI_STACKSAVE2(d8,d9,0)
        CFI_STACKSAVE2(d10,d11,0x10)
        CFI_STACKSAVE2(d12,d13,0x20)
        CFI_STACKSAVE2(d14,d15,0x30)

// Load the Keccak initial state into upper and lower parts of Q0..Q24

        add     x2, x0, #0xc8

        ldp     q24, q25, [x0]
        ldp     q26, q27, [x2]
        trn1    v0.2d, v24.2d, v26.2d
        trn2    v1.2d, v24.2d, v26.2d
        trn1    v2.2d, v25.2d, v27.2d
        trn2    v3.2d, v25.2d, v27.2d

        ldp     q24, q25, [x0, #0x20]
        ldp     q26, q27, [x2, #0x20]
        trn1    v4.2d, v24.2d, v26.2d
        trn2    v5.2d, v24.2d, v26.2d
        trn1    v6.2d, v25.2d, v27.2d
        trn2    v7.2d, v25.2d, v27.2d

        ldp     q24, q25, [x0, #0x40]
        ldp     q26, q27, [x2, #0x40]
        trn1    v8.2d, v24.2d, v26.2d
        trn2    v9.2d, v24.2d, v26.2d
        trn1    v10.2d, v25.2d, v27.2d
        trn2    v11.2d, v25.2d, v27.2d

        ldp     q24, q25, [x0, #0x60]
        ldp     q26, q27, [x2, #0x60]
        trn1    v12.2d, v24.2d, v26.2d
        trn2    v13.2d, v24.2d, v26.2d
        trn1    v14.2d, v25.2d, v27.2d
        trn2    v15.2d, v25.2d, v27.2d

        ldp     q24, q25, [x0, #0x80]
        ldp     q26, q27, [x2, #0x80]
        trn1    v16.2d, v24.2d, v26.2d
        trn2    v17.2d, v24.2d, v26.2d
        trn1    v18.2d, v25.2d, v27.2d
        trn2    v19.2d, v25.2d, v27.2d

        ldp     q24, q25, [x0, #0xa0]
        ldp     q26, q27, [x2, #0xa0]
        trn1    v20.2d, v24.2d, v26.2d
        trn2    v21.2d, v24.2d, v26.2d
        trn1    v22.2d, v25.2d, v27.2d
        trn2    v23.2d, v25.2d, v27.2d

        ldr     d24, [x0, #0xc0]
        ldr     d25, [x2, #0xc0]
        trn1    v24.2d, v24.2d, v25.2d

// Now 24 rounds of the iteration

        mov     x2, #24

Lsha3_keccak2_f1600_alt_loop:
        eor     v30.16b, v0.16b, v5.16b
        eor     v30.16b, v30.16b, v10.16b
        eor3    v30.16b, v30.16b, v15.16b, v20.16b
        eor     v29.16b, v1.16b, v6.16b
        eor     v29.16b, v29.16b, v11.16b
        eor3    v29.16b, v29.16b, v16.16b, v21.16b
        eor     v28.16b, v2.16b, v7.16b
        eor     v28.16b, v28.16b, v12.16b
        eor3    v28.16b, v28.16b, v17.16b, v22.16b
        eor     v27.16b, v3.16b, v8.16b
        eor     v27.16b, v27.16b, v13.16b
        eor3    v27.16b, v27.16b, v18.16b, v23.16b
        eor     v26.16b, v4.16b, v9.16b
        eor     v26.16b, v26.16b, v14.16b
        eor3    v26.16b, v26.16b, v19.16b, v24.16b
        add     v31.2d, v28.2d, v28.2d
        sri     v31.2d, v28.2d, #0x3f
        eor     v25.16b, v31.16b, v30.16b
        rax1    v28.2d, v28.2d, v26.2d
        add     v31.2d, v29.2d, v29.2d
        sri     v31.2d, v29.2d, #0x3f
        eor     v26.16b, v31.16b, v26.16b
        rax1    v29.2d, v29.2d, v27.2d
        add     v31.2d, v30.2d, v30.2d
        sri     v31.2d, v30.2d, #0x3f
        eor     v27.16b, v31.16b, v27.16b
        eor     v30.16b, v0.16b, v26.16b
        xar     v0.2d, v2.2d, v29.2d, #0x2
        eor     v31.16b, v12.16b, v29.16b
        shl     v2.2d, v31.2d, #0x2b
        sri     v2.2d, v31.2d, #0x15
        xar     v12.2d, v13.2d, v28.2d, #0x27
        eor     v31.16b, v19.16b, v27.16b
        shl     v13.2d, v31.2d, #0x8
        sri     v13.2d, v31.2d, #0x38
        xar     v19.2d, v23.2d, v28.2d, #0x8
        eor     v31.16b, v15.16b, v26.16b
        shl     v23.2d, v31.2d, #0x29
        sri     v23.2d, v31.2d, #0x17
        xar     v15.2d, v1.2d, v25.2d, #0x3f
        eor     v31.16b, v8.16b, v28.16b
        shl     v1.2d, v31.2d, #0x37
        sri     v1.2d, v31.2d, #0x9
        xar     v8.2d, v16.2d, v25.2d, #0x13
        eor     v31.16b, v7.16b, v29.16b
        shl     v16.2d, v31.2d, #0x6
        sri     v16.2d, v31.2d, #0x3a
        xar     v7.2d, v10.2d, v26.2d, #0x3d
        eor     v31.16b, v3.16b, v28.16b
        shl     v10.2d, v31.2d, #0x1c
        sri     v10.2d, v31.2d, #0x24
        xar     v3.2d, v18.2d, v28.2d, #0x2b
        eor     v31.16b, v17.16b, v29.16b
        shl     v18.2d, v31.2d, #0xf
        sri     v18.2d, v31.2d, #0x31
        xar     v17.2d, v11.2d, v25.2d, #0x36
        eor     v31.16b, v9.16b, v27.16b
        shl     v11.2d, v31.2d, #0x14
        sri     v11.2d, v31.2d, #0x2c
        xar     v9.2d, v22.2d, v29.2d, #0x3
        eor     v31.16b, v14.16b, v27.16b
        shl     v22.2d, v31.2d, #0x27
        sri     v22.2d, v31.2d, #0x19
        xar     v14.2d, v20.2d, v26.2d, #0x2e
        eor     v31.16b, v4.16b, v27.16b
        shl     v20.2d, v31.2d, #0x1b
        sri     v20.2d, v31.2d, #0x25
        xar     v4.2d, v24.2d, v27.2d, #0x32
        eor     v31.16b, v21.16b, v25.16b
        shl     v24.2d, v31.2d, #0x2
        sri     v24.2d, v31.2d, #0x3e
        xar     v21.2d, v5.2d, v26.2d, #0x1c
        eor     v31.16b, v6.16b, v25.16b
        shl     v27.2d, v31.2d, #0x2c
        sri     v27.2d, v31.2d, #0x14
        ld1r    { v28.2d }, [x1], #8
        bcax    v5.16b, v10.16b, v7.16b, v11.16b
        bic     v31.16b, v8.16b, v7.16b
        eor     v6.16b, v31.16b, v11.16b
        bcax    v7.16b, v7.16b, v9.16b, v8.16b
        bic     v31.16b, v10.16b, v9.16b
        eor     v8.16b, v31.16b, v8.16b
        bcax    v9.16b, v9.16b, v11.16b, v10.16b
        bic     v31.16b, v12.16b, v16.16b
        eor     v10.16b, v31.16b, v15.16b
        bcax    v11.16b, v16.16b, v13.16b, v12.16b
        bic     v31.16b, v14.16b, v13.16b
        eor     v12.16b, v31.16b, v12.16b
        bcax    v13.16b, v13.16b, v15.16b, v14.16b
        bic     v31.16b, v16.16b, v15.16b
        eor     v14.16b, v31.16b, v14.16b
        bcax    v15.16b, v20.16b, v17.16b, v21.16b
        bic     v31.16b, v18.16b, v17.16b
        eor     v16.16b, v31.16b, v21.16b
        bcax    v17.16b, v17.16b, v19.16b, v18.16b
        bic     v31.16b, v20.16b, v19.16b
        eor     v18.16b, v31.16b, v18.16b
        bcax    v19.16b, v19.16b, v21.16b, v20.16b
        bic     v31.16b, v22.16b, v1.16b
        eor     v20.16b, v31.16b, v0.16b
        bcax    v21.16b, v1.16b, v23.16b, v22.16b
        bic     v31.16b, v24.16b, v23.16b
        eor     v22.16b, v31.16b, v22.16b
        bcax    v23.16b, v23.16b, v0.16b, v24.16b
        bic     v31.16b, v1.16b, v0.16b
        eor     v24.16b, v31.16b, v24.16b
        bcax    v0.16b, v30.16b, v2.16b, v27.16b
        bic     v31.16b, v3.16b, v2.16b
        eor     v1.16b, v31.16b, v27.16b
        bcax    v2.16b, v2.16b, v4.16b, v3.16b
        bic     v31.16b, v30.16b, v4.16b
        eor     v3.16b, v31.16b, v3.16b
        bcax    v4.16b, v4.16b, v27.16b, v30.16b
        eor     v0.16b, v0.16b, v28.16b
        sub     x2, x2, #0x1
        cbnz    x2, Lsha3_keccak2_f1600_alt_loop

// Store back the state

        add     x2, x0, #0xc8

        trn1    v25.2d, v0.2d, v1.2d
        trn1    v26.2d, v2.2d, v3.2d
        stp     q25, q26, [x0]
        trn2    v25.2d, v0.2d, v1.2d
        trn2    v26.2d, v2.2d, v3.2d
        stp     q25, q26, [x2]

        trn1    v25.2d, v4.2d, v5.2d
        trn1    v26.2d, v6.2d, v7.2d
        stp     q25, q26, [x0, #0x20]
        trn2    v25.2d, v4.2d, v5.2d
        trn2    v26.2d, v6.2d, v7.2d
        stp     q25, q26, [x2, #0x20]

        trn1    v25.2d, v8.2d, v9.2d
        trn1    v26.2d, v10.2d, v11.2d
        stp     q25, q26, [x0, #0x40]
        trn2    v25.2d, v8.2d, v9.2d
        trn2    v26.2d, v10.2d, v11.2d
        stp     q25, q26, [x2, #0x40]

        trn1    v25.2d, v12.2d, v13.2d
        trn1    v26.2d, v14.2d, v15.2d
        stp     q25, q26, [x0, #0x60]
        trn2    v25.2d, v12.2d, v13.2d
        trn2    v26.2d, v14.2d, v15.2d
        stp     q25, q26, [x2, #0x60]

        trn1    v25.2d, v16.2d, v17.2d
        trn1    v26.2d, v18.2d, v19.2d
        stp     q25, q26, [x0, #0x80]
        trn2    v25.2d, v16.2d, v17.2d
        trn2    v26.2d, v18.2d, v19.2d
        stp     q25, q26, [x2, #0x80]

        trn1    v25.2d, v20.2d, v21.2d
        trn1    v26.2d, v22.2d, v23.2d
        stp     q25, q26, [x0, #0xa0]
        trn2    v25.2d, v20.2d, v21.2d
        trn2    v26.2d, v22.2d, v23.2d
        stp     q25, q26, [x2, #0xa0]

        str     d24, [x0, #0xc0]
        trn2    v24.2d, v24.2d, v24.2d
        str     d24, [x2, #0xc0]

// Restore registers and return

        CFI_STACKLOAD2(d8,d9,0)
        CFI_STACKLOAD2(d10,d11,0x10)
        CFI_STACKLOAD2(d12,d13,0x20)
        CFI_STACKLOAD2(d14,d15,0x30)
        CFI_INC_SP(64)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(sha3_keccak2_f1600_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
