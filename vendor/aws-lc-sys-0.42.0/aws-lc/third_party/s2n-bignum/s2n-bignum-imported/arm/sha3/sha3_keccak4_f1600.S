// Copyright (c) 2024 The mlkem-native project authors
// Copyright (c) 2021-2022 Arm Limited
// Copyright (c) 2022 Matthias Kannwischer
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Keccak-f1600 permutation for SHA3, batch of four independent operations
// Input a[100], rc[24]; output a[100]
//
// The input/output argument is in effect four 25-element Keccak arrays
// a[0...24], a[25..49], a[50..74] and a[75..99], which could be considered
// as type a[25][4].
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
// extern void sha3_keccak4_f1600(uint64_t a[static 100],
//                                const uint64_t rc[static 24]);
//
// Standard ARM ABI: X0 = a, X1 = rc
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        .arch armv8.4-a+sha3
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sha3_keccak4_f1600)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sha3_keccak4_f1600)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sha3_keccak4_f1600)
        .text
        .balign 4

S2N_BN_SYMBOL(sha3_keccak4_f1600):
        CFI_START

// This is similar to the code in the mlkem-native repository here:
//
//  mlkem/fips202/native/aarch64/src/keccak_f1600_x4_scalar_v84a_asm_hybrid.S
//
// The main difference is the avoidance of ld2/st2 in favour of explicit
// transposition operations and conventional loads and stores.

        CFI_DEC_SP(224)
        CFI_STACKSAVE2(x19,x20,0x30)
        CFI_STACKSAVE2(x21,x22,0x40)
        CFI_STACKSAVE2(x23,x24,0x50)
        CFI_STACKSAVE2(x25,x26,0x60)
        CFI_STACKSAVE2(x27,x28,0x70)
        CFI_STACKSAVE2(x29,x30,0x80)
        CFI_STACKSAVE2(d8,d9,0x90)
        CFI_STACKSAVE2(d10,d11,0xa0)
        CFI_STACKSAVE2(d12,d13,0xb0)
        CFI_STACKSAVE2(d14,d15,0xc0)
        mov     x29, x1
        mov     x30, #0x0
        str     x30, [sp, #0x20]
        str     x29, [sp, #0x8]
        str     x29, [sp, #0x10]
        str     x0, [sp]

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

        add     x0, x0, #0x190
        ldp     x1, x6, [x0]
        ldp     x11, x16, [x0, #0x10]
        ldp     x21, x2, [x0, #0x20]
        ldp     x7, x12, [x0, #0x30]
        ldp     x17, x22, [x0, #0x40]
        ldp     x3, x8, [x0, #0x50]
        ldp     x13, x28, [x0, #0x60]
        ldp     x23, x4, [x0, #0x70]
        ldp     x9, x14, [x0, #0x80]
        ldp     x19, x24, [x0, #0x90]
        ldp     x5, x10, [x0, #0xa0]
        ldp     x15, x20, [x0, #0xb0]
        ldr     x25, [x0, #0xc0]
        sub     x0, x0, #0x190

Lsha3_keccak4_f1600_initial:
        eor     x30, x24, x25
        eor     x27, x9, x10
        eor     x0, x30, x21
        eor     x26, x27, x6
        eor     x27, x26, x7
        eor     x29, x0, x22
        eor     x26, x29, x23
        eor     x29, x4, x5
        eor     x30, x29, x1
        eor     x0, x27, x8
        eor     x29, x30, x2
        eor     x30, x19, x20
        eor     x30, x30, x16
        eor     x27, x26, x0, ror #63
        eor     x4, x4, x27
        eor     x30, x30, x17
        eor     x30, x30, x28
        eor     x29, x29, x3
        eor     x0, x0, x30, ror #63
        eor     x30, x30, x29, ror #63
        eor     x22, x22, x30
        eor     x23, x23, x30
        str     x23, [sp, #0xd0]
        eor     x23, x14, x15
        eor     x14, x14, x0
        eor     x23, x23, x11
        eor     x15, x15, x0
        eor     x1, x1, x27
        eor     x23, x23, x12
        eor     x23, x23, x13
        eor     x11, x11, x0
        eor     x29, x29, x23, ror #63
        eor     x23, x23, x26, ror #63
        eor     x26, x13, x0
        eor     x13, x28, x23
        eor     x28, x24, x30
        eor     x24, x16, x23
        eor     x16, x21, x30
        eor     x21, x25, x30
        eor     x30, x19, x23
        eor     x19, x20, x23
        eor     x20, x17, x23
        eor     x17, x12, x0
        eor     x0, x2, x27
        eor     x2, x6, x29
        eor     x6, x8, x29
        bic     x8, x28, x13, ror #47
        eor     x12, x3, x27
        bic     x3, x13, x17, ror #19
        eor     x5, x5, x27
        ldr     x27, [sp, #0xd0]
        bic     x25, x17, x2, ror #5
        eor     x9, x9, x29
        eor     x23, x25, x5, ror #52
        eor     x3, x3, x2, ror #24
        eor     x8, x8, x17, ror #2
        eor     x17, x10, x29
        bic     x25, x12, x22, ror #47
        eor     x29, x7, x29
        bic     x10, x4, x27, ror #2
        bic     x7, x5, x28, ror #10
        eor     x10, x10, x20, ror #50
        eor     x13, x7, x13, ror #57
        bic     x7, x2, x5, ror #47
        eor     x2, x25, x24, ror #39
        bic     x25, x20, x11, ror #57
        bic     x5, x17, x4, ror #25
        eor     x25, x25, x17, ror #53
        bic     x17, x11, x17, ror #60
        eor     x28, x7, x28, ror #57
        bic     x7, x9, x12, ror #42
        eor     x7, x7, x22, ror #25
        bic     x22, x22, x24, ror #56
        bic     x24, x24, x15, ror #31
        eor     x22, x22, x15, ror #23
        bic     x20, x27, x20, ror #48
        bic     x15, x15, x9, ror #16
        eor     x12, x15, x12, ror #58
        eor     x15, x5, x27, ror #27
        eor     x5, x20, x11, ror #41
        ldr     x11, [sp, #0x8]
        eor     x20, x17, x4, ror #21
        eor     x17, x24, x9, ror #47
        mov     x24, #0x1
        bic     x9, x0, x16, ror #9
        str     x24, [sp, #0x18]
        bic     x24, x29, x1, ror #44
        bic     x27, x1, x21, ror #50
        bic     x4, x26, x29, ror #63
        eor     x1, x1, x4, ror #21
        ldr     x11, [x11]
        bic     x4, x21, x30, ror #57
        eor     x21, x24, x21, ror #30
        eor     x24, x9, x19, ror #44
        bic     x9, x14, x6, ror #5
        eor     x9, x9, x0, ror #43
        bic     x0, x6, x0, ror #38
        eor     x1, x1, x11
        eor     x11, x4, x26, ror #35
        eor     x4, x0, x16, ror #47
        bic     x0, x16, x19, ror #35
        eor     x16, x27, x30, ror #43
        bic     x27, x30, x26, ror #42
        bic     x26, x19, x14, ror #41
        eor     x19, x0, x14, ror #12
        eor     x14, x26, x6, ror #46
        eor     x6, x27, x29, ror #41
        eor     x0, x15, x11, ror #52
        eor     x0, x0, x13, ror #48
        eor     x26, x8, x9, ror #57
        eor     x27, x0, x14, ror #10
        eor     x29, x16, x28, ror #63
        eor     x26, x26, x6, ror #51
        eor     x30, x23, x22, ror #50
        eor     x0, x26, x10, ror #31
        eor     x29, x29, x19, ror #37
        eor     x27, x27, x12, ror #5
        eor     x30, x30, x24, ror #34
        eor     x0, x0, x7, ror #27
        eor     x26, x30, x21, ror #26
        eor     x26, x26, x25, ror #15
        ror     x30, x27, #0x3e
        eor     x30, x30, x26, ror #57
        ror     x26, x26, #0x3a
        eor     x16, x30, x16
        eor     x28, x30, x28, ror #63
        str     x28, [sp, #0xd0]
        eor     x29, x29, x17, ror #36
        eor     x28, x1, x2, ror #61
        eor     x19, x30, x19, ror #37
        eor     x29, x29, x20, ror #2
        eor     x28, x28, x4, ror #54
        eor     x26, x26, x0, ror #55
        eor     x28, x28, x3, ror #39
        eor     x28, x28, x5, ror #25
        ror     x0, x0, #0x38
        eor     x0, x0, x29, ror #63
        eor     x27, x28, x27, ror #61
        eor     x13, x0, x13, ror #46
        eor     x28, x29, x28, ror #63
        eor     x29, x30, x20, ror #2
        eor     x20, x26, x3, ror #39
        eor     x11, x0, x11, ror #50
        eor     x25, x28, x25, ror #9
        eor     x3, x28, x21, ror #20
        eor     x21, x26, x1
        eor     x9, x27, x9, ror #49
        eor     x24, x28, x24, ror #28
        eor     x1, x30, x17, ror #36
        eor     x14, x0, x14, ror #8
        eor     x22, x28, x22, ror #44
        eor     x8, x27, x8, ror #56
        eor     x17, x27, x7, ror #19
        eor     x15, x0, x15, ror #62
        bic     x7, x20, x22, ror #47
        eor     x4, x26, x4, ror #54
        eor     x0, x0, x12, ror #3
        eor     x28, x28, x23, ror #58
        eor     x23, x26, x2, ror #61
        eor     x26, x26, x5, ror #25
        eor     x2, x7, x16, ror #39
        bic     x7, x9, x20, ror #42
        bic     x30, x15, x9, ror #16
        eor     x7, x7, x22, ror #25
        eor     x12, x30, x20, ror #58
        bic     x20, x22, x16, ror #56
        eor     x30, x27, x6, ror #43
        eor     x22, x20, x15, ror #23
        bic     x6, x19, x13, ror #42
        eor     x6, x6, x17, ror #41
        bic     x5, x13, x17, ror #63
        eor     x5, x21, x5, ror #21
        bic     x17, x17, x21, ror #44
        eor     x27, x27, x10, ror #23
        bic     x21, x21, x25, ror #50
        bic     x20, x27, x4, ror #25
        bic     x10, x16, x15, ror #31
        eor     x16, x21, x19, ror #43
        eor     x21, x17, x25, ror #30
        bic     x19, x25, x19, ror #57
        ldr     x25, [sp, #0x18]
        eor     x17, x10, x9, ror #47
        ldr     x9, [sp, #0x8]
        eor     x15, x20, x28, ror #27
        bic     x20, x4, x28, ror #2
        eor     x10, x20, x1, ror #50
        bic     x20, x11, x27, ror #60
        eor     x20, x20, x4, ror #21
        bic     x4, x28, x1, ror #48
        bic     x1, x1, x11, ror #57
        ldr     x28, [x9, x25, lsl #3]
        ldr     x9, [sp, #0xd0]
        add     x25, x25, #0x1
        str     x25, [sp, #0x18]
        cmp     x25, #0x17
        eor     x25, x1, x27, ror #53
        bic     x27, x30, x26, ror #47
        eor     x1, x5, x28
        eor     x5, x4, x11, ror #41
        eor     x11, x19, x13, ror #35
        bic     x13, x26, x24, ror #10
        eor     x28, x27, x24, ror #57
        bic     x27, x24, x9, ror #47
        bic     x19, x23, x3, ror #9
        bic     x4, x29, x14, ror #41
        eor     x24, x19, x29, ror #44
        bic     x29, x3, x29, ror #35
        eor     x13, x13, x9, ror #57
        eor     x19, x29, x14, ror #12
        bic     x29, x9, x0, ror #19
        bic     x14, x14, x8, ror #5
        eor     x9, x14, x23, ror #43
        eor     x14, x4, x8, ror #46
        bic     x23, x8, x23, ror #38
        eor     x8, x27, x0, ror #2
        eor     x4, x23, x3, ror #47
        bic     x3, x0, x30, ror #5
        eor     x23, x3, x26, ror #52
        eor     x3, x29, x30, ror #24
        eor3    v30.16b, v0.16b, v5.16b, v10.16b
        eor3    v30.16b, v30.16b, v15.16b, v20.16b
        eor3    v29.16b, v1.16b, v6.16b, v11.16b
        eor3    v29.16b, v29.16b, v16.16b, v21.16b
        eor3    v28.16b, v2.16b, v7.16b, v12.16b
        eor3    v28.16b, v28.16b, v17.16b, v22.16b
        eor3    v27.16b, v3.16b, v8.16b, v13.16b
        eor3    v27.16b, v27.16b, v18.16b, v23.16b
        eor3    v26.16b, v4.16b, v9.16b, v14.16b
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
        ldr     x30, [sp, #0x10]
        ld1r    { v28.2d }, [x30], #8
        str     x30, [sp, #0x10]
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
        eor     v0.16b, v0.16b, v28.16b

Lsha3_keccak4_f1600_loop:
        eor     x0, x15, x11, ror #52
        eor     x0, x0, x13, ror #48
        eor     x26, x8, x9, ror #57
        eor     x27, x0, x14, ror #10
        eor3    v30.16b, v0.16b, v5.16b, v10.16b
        eor3    v30.16b, v30.16b, v15.16b, v20.16b
        eor     x29, x16, x28, ror #63
        eor     x26, x26, x6, ror #51
        eor     x30, x23, x22, ror #50
        eor3    v29.16b, v1.16b, v6.16b, v11.16b
        eor     x0, x26, x10, ror #31
        eor     x29, x29, x19, ror #37
        eor     x27, x27, x12, ror #5
        eor3    v29.16b, v29.16b, v16.16b, v21.16b
        eor     x30, x30, x24, ror #34
        eor     x0, x0, x7, ror #27
        eor     x26, x30, x21, ror #26
        eor3    v28.16b, v2.16b, v7.16b, v12.16b
        eor     x26, x26, x25, ror #15
        ror     x30, x27, #0x3e
        eor     x30, x30, x26, ror #57
        ror     x26, x26, #0x3a
        eor3    v28.16b, v28.16b, v17.16b, v22.16b
        eor     x16, x30, x16
        eor     x28, x30, x28, ror #63
        str     x28, [sp, #0xd0]
        eor3    v27.16b, v3.16b, v8.16b, v13.16b
        eor     x29, x29, x17, ror #36
        eor     x28, x1, x2, ror #61
        eor     x19, x30, x19, ror #37
        eor3    v27.16b, v27.16b, v18.16b, v23.16b
        eor     x29, x29, x20, ror #2
        eor     x28, x28, x4, ror #54
        eor     x26, x26, x0, ror #55
        eor3    v26.16b, v4.16b, v9.16b, v14.16b
        eor     x28, x28, x3, ror #39
        eor     x28, x28, x5, ror #25
        ror     x0, x0, #0x38
        eor     x0, x0, x29, ror #63
        eor3    v26.16b, v26.16b, v19.16b, v24.16b
        eor     x27, x28, x27, ror #61
        eor     x13, x0, x13, ror #46
        eor     x28, x29, x28, ror #63
        rax1    v25.2d, v30.2d, v28.2d
        eor     x29, x30, x20, ror #2
        eor     x20, x26, x3, ror #39
        eor     x11, x0, x11, ror #50
        rax1    v28.2d, v28.2d, v26.2d
        eor     x25, x28, x25, ror #9
        eor     x3, x28, x21, ror #20
        eor     x21, x26, x1
        rax1    v26.2d, v26.2d, v29.2d
        eor     x9, x27, x9, ror #49
        eor     x24, x28, x24, ror #28
        eor     x1, x30, x17, ror #36
        eor     x14, x0, x14, ror #8
        rax1    v29.2d, v29.2d, v27.2d
        eor     x22, x28, x22, ror #44
        eor     x8, x27, x8, ror #56
        eor     x17, x27, x7, ror #19
        rax1    v27.2d, v27.2d, v30.2d
        eor     x15, x0, x15, ror #62
        bic     x7, x20, x22, ror #47
        eor     x4, x26, x4, ror #54
        eor     v30.16b, v0.16b, v26.16b
        eor     x0, x0, x12, ror #3
        eor     x28, x28, x23, ror #58
        eor     x23, x26, x2, ror #61
        xar     v0.2d, v2.2d, v29.2d, #0x2
        eor     x26, x26, x5, ror #25
        eor     x2, x7, x16, ror #39
        bic     x7, x9, x20, ror #42
        bic     x30, x15, x9, ror #16
        xar     v2.2d, v12.2d, v29.2d, #0x15
        eor     x7, x7, x22, ror #25
        eor     x12, x30, x20, ror #58
        bic     x20, x22, x16, ror #56
        xar     v12.2d, v13.2d, v28.2d, #0x27
        eor     x30, x27, x6, ror #43
        eor     x22, x20, x15, ror #23
        bic     x6, x19, x13, ror #42
        xar     v13.2d, v19.2d, v27.2d, #0x38
        eor     x6, x6, x17, ror #41
        bic     x5, x13, x17, ror #63
        eor     x5, x21, x5, ror #21
        xar     v19.2d, v23.2d, v28.2d, #0x8
        bic     x17, x17, x21, ror #44
        eor     x27, x27, x10, ror #23
        bic     x21, x21, x25, ror #50
        bic     x20, x27, x4, ror #25
        xar     v23.2d, v15.2d, v26.2d, #0x17
        bic     x10, x16, x15, ror #31
        eor     x16, x21, x19, ror #43
        eor     x21, x17, x25, ror #30
        xar     v15.2d, v1.2d, v25.2d, #0x3f
        bic     x19, x25, x19, ror #57
        ldr     x25, [sp, #0x18]
        eor     x17, x10, x9, ror #47
        xar     v1.2d, v8.2d, v28.2d, #0x9
        ldr     x9, [sp, #0x8]
        eor     x15, x20, x28, ror #27
        bic     x20, x4, x28, ror #2
        xar     v8.2d, v16.2d, v25.2d, #0x13
        eor     x10, x20, x1, ror #50
        bic     x20, x11, x27, ror #60
        eor     x20, x20, x4, ror #21
        bic     x4, x28, x1, ror #48
        xar     v16.2d, v7.2d, v29.2d, #0x3a
        bic     x1, x1, x11, ror #57
        ldr     x28, [x9, x25, lsl #3]
        ldr     x9, [sp, #0xd0]
        xar     v7.2d, v10.2d, v26.2d, #0x3d
        add     x25, x25, #0x1
        str     x25, [sp, #0x18]
        cmp     x25, #0x17
        xar     v10.2d, v3.2d, v28.2d, #0x24
        eor     x25, x1, x27, ror #53
        bic     x27, x30, x26, ror #47
        eor     x1, x5, x28
        xar     v3.2d, v18.2d, v28.2d, #0x2b
        eor     x5, x4, x11, ror #41
        eor     x11, x19, x13, ror #35
        bic     x13, x26, x24, ror #10
        eor     x28, x27, x24, ror #57
        xar     v18.2d, v17.2d, v29.2d, #0x31
        bic     x27, x24, x9, ror #47
        bic     x19, x23, x3, ror #9
        bic     x4, x29, x14, ror #41
        xar     v17.2d, v11.2d, v25.2d, #0x36
        eor     x24, x19, x29, ror #44
        bic     x29, x3, x29, ror #35
        eor     x13, x13, x9, ror #57
        xar     v11.2d, v9.2d, v27.2d, #0x2c
        eor     x19, x29, x14, ror #12
        bic     x29, x9, x0, ror #19
        bic     x14, x14, x8, ror #5
        xar     v9.2d, v22.2d, v29.2d, #0x3
        eor     x9, x14, x23, ror #43
        eor     x14, x4, x8, ror #46
        bic     x23, x8, x23, ror #38
        eor     x8, x27, x0, ror #2
        xar     v22.2d, v14.2d, v27.2d, #0x19
        eor     x4, x23, x3, ror #47
        bic     x3, x0, x30, ror #5
        eor     x23, x3, x26, ror #52
        xar     v14.2d, v20.2d, v26.2d, #0x2e
        eor     x3, x29, x30, ror #24
        eor     x0, x15, x11, ror #52
        eor     x0, x0, x13, ror #48
        xar     v20.2d, v4.2d, v27.2d, #0x25
        eor     x26, x8, x9, ror #57
        eor     x27, x0, x14, ror #10
        eor     x29, x16, x28, ror #63
        xar     v4.2d, v24.2d, v27.2d, #0x32
        eor     x26, x26, x6, ror #51
        eor     x30, x23, x22, ror #50
        eor     x0, x26, x10, ror #31
        eor     x29, x29, x19, ror #37
        xar     v24.2d, v21.2d, v25.2d, #0x3e
        eor     x27, x27, x12, ror #5
        eor     x30, x30, x24, ror #34
        eor     x0, x0, x7, ror #27
        xar     v21.2d, v5.2d, v26.2d, #0x1c
        eor     x26, x30, x21, ror #26
        eor     x26, x26, x25, ror #15
        ror     x30, x27, #0x3e
        xar     v27.2d, v6.2d, v25.2d, #0x14
        eor     x30, x30, x26, ror #57
        ror     x26, x26, #0x3a
        eor     x16, x30, x16
        bcax    v5.16b, v10.16b, v7.16b, v11.16b
        eor     x28, x30, x28, ror #63
        str     x28, [sp, #0xd0]
        eor     x29, x29, x17, ror #36
        eor     x28, x1, x2, ror #61
        bcax    v6.16b, v11.16b, v8.16b, v7.16b
        eor     x19, x30, x19, ror #37
        eor     x29, x29, x20, ror #2
        eor     x28, x28, x4, ror #54
        bcax    v7.16b, v7.16b, v9.16b, v8.16b
        eor     x26, x26, x0, ror #55
        eor     x28, x28, x3, ror #39
        eor     x28, x28, x5, ror #25
        bcax    v8.16b, v8.16b, v10.16b, v9.16b
        ror     x0, x0, #0x38
        eor     x0, x0, x29, ror #63
        eor     x27, x28, x27, ror #61
        bcax    v9.16b, v9.16b, v11.16b, v10.16b
        eor     x13, x0, x13, ror #46
        eor     x28, x29, x28, ror #63
        eor     x29, x30, x20, ror #2
        eor     x20, x26, x3, ror #39
        bcax    v10.16b, v15.16b, v12.16b, v16.16b
        eor     x11, x0, x11, ror #50
        eor     x25, x28, x25, ror #9
        eor     x3, x28, x21, ror #20
        bcax    v11.16b, v16.16b, v13.16b, v12.16b
        eor     x21, x26, x1
        eor     x9, x27, x9, ror #49
        eor     x24, x28, x24, ror #28
        bcax    v12.16b, v12.16b, v14.16b, v13.16b
        eor     x1, x30, x17, ror #36
        eor     x14, x0, x14, ror #8
        eor     x22, x28, x22, ror #44
        bcax    v13.16b, v13.16b, v15.16b, v14.16b
        eor     x8, x27, x8, ror #56
        eor     x17, x27, x7, ror #19
        eor     x15, x0, x15, ror #62
        bic     x7, x20, x22, ror #47
        bcax    v14.16b, v14.16b, v16.16b, v15.16b
        eor     x4, x26, x4, ror #54
        eor     x0, x0, x12, ror #3
        eor     x28, x28, x23, ror #58
        bcax    v15.16b, v20.16b, v17.16b, v21.16b
        eor     x23, x26, x2, ror #61
        eor     x26, x26, x5, ror #25
        eor     x2, x7, x16, ror #39
        bcax    v16.16b, v21.16b, v18.16b, v17.16b
        bic     x7, x9, x20, ror #42
        bic     x30, x15, x9, ror #16
        eor     x7, x7, x22, ror #25
        bcax    v17.16b, v17.16b, v19.16b, v18.16b
        eor     x12, x30, x20, ror #58
        bic     x20, x22, x16, ror #56
        eor     x30, x27, x6, ror #43
        eor     x22, x20, x15, ror #23
        bcax    v18.16b, v18.16b, v20.16b, v19.16b
        bic     x6, x19, x13, ror #42
        eor     x6, x6, x17, ror #41
        bic     x5, x13, x17, ror #63
        bcax    v19.16b, v19.16b, v21.16b, v20.16b
        eor     x5, x21, x5, ror #21
        bic     x17, x17, x21, ror #44
        eor     x27, x27, x10, ror #23
        bcax    v20.16b, v0.16b, v22.16b, v1.16b
        bic     x21, x21, x25, ror #50
        bic     x20, x27, x4, ror #25
        bic     x10, x16, x15, ror #31
        bcax    v21.16b, v1.16b, v23.16b, v22.16b
        eor     x16, x21, x19, ror #43
        eor     x21, x17, x25, ror #30
        bic     x19, x25, x19, ror #57
        ldr     x25, [sp, #0x18]
        bcax    v22.16b, v22.16b, v24.16b, v23.16b
        eor     x17, x10, x9, ror #47
        ldr     x9, [sp, #0x8]
        eor     x15, x20, x28, ror #27
        bcax    v23.16b, v23.16b, v0.16b, v24.16b
        bic     x20, x4, x28, ror #2
        eor     x10, x20, x1, ror #50
        bic     x20, x11, x27, ror #60
        bcax    v24.16b, v24.16b, v1.16b, v0.16b
        eor     x20, x20, x4, ror #21
        bic     x4, x28, x1, ror #48
        bic     x1, x1, x11, ror #57
        bcax    v0.16b, v30.16b, v2.16b, v27.16b
        ldr     x28, [x9, x25, lsl #3]
        ldr     x9, [sp, #0xd0]
        add     x25, x25, #0x1
        str     x25, [sp, #0x18]
        bcax    v1.16b, v27.16b, v3.16b, v2.16b
        cmp     x25, #0x17
        eor     x25, x1, x27, ror #53
        bic     x27, x30, x26, ror #47
        bcax    v2.16b, v2.16b, v4.16b, v3.16b
        eor     x1, x5, x28
        eor     x5, x4, x11, ror #41
        eor     x11, x19, x13, ror #35
        bcax    v3.16b, v3.16b, v30.16b, v4.16b
        bic     x13, x26, x24, ror #10
        eor     x28, x27, x24, ror #57
        bic     x27, x24, x9, ror #47
        bcax    v4.16b, v4.16b, v27.16b, v30.16b
        bic     x19, x23, x3, ror #9
        bic     x4, x29, x14, ror #41
        eor     x24, x19, x29, ror #44
        bic     x29, x3, x29, ror #35
        eor     x13, x13, x9, ror #57
        eor     x19, x29, x14, ror #12
        bic     x29, x9, x0, ror #19
        bic     x14, x14, x8, ror #5
        eor     x9, x14, x23, ror #43
        eor     x14, x4, x8, ror #46
        bic     x23, x8, x23, ror #38
        eor     x8, x27, x0, ror #2
        eor     x4, x23, x3, ror #47
        bic     x3, x0, x30, ror #5
        eor     x23, x3, x26, ror #52
        eor     x3, x29, x30, ror #24
        ldr     x30, [sp, #0x10]
        ld1r    { v28.2d }, [x30], #8
        str     x30, [sp, #0x10]
        eor     v0.16b, v0.16b, v28.16b
        ble     Lsha3_keccak4_f1600_loop

        ror     x2, x2, #0x3d
        ror     x3, x3, #0x27
        ror     x4, x4, #0x36
        ror     x5, x5, #0x19
        ror     x6, x6, #0x2b
        ror     x7, x7, #0x13
        ror     x8, x8, #0x38
        ror     x9, x9, #0x31
        ror     x10, x10, #0x17
        ror     x11, x11, #0x32
        ror     x12, x12, #0x3
        ror     x13, x13, #0x2e
        ror     x14, x14, #0x8
        ror     x15, x15, #0x3e
        ror     x17, x17, #0x24
        ror     x28, x28, #0x3f
        ror     x19, x19, #0x25
        ror     x20, x20, #0x2
        ror     x21, x21, #0x14
        ror     x22, x22, #0x2c
        ror     x23, x23, #0x3a
        ror     x24, x24, #0x1c
        ror     x25, x25, #0x9
        ldr     x30, [sp, #0x20]
        cmp     x30, #0x1

        beq     Lsha3_keccak4_f1600_done
        mov     x30, #0x1
        str     x30, [sp, #0x20]
        ldr     x0, [sp]
        add     x0, x0, #0x190
        stp     x1, x6, [x0]
        stp     x11, x16, [x0, #0x10]
        stp     x21, x2, [x0, #0x20]
        stp     x7, x12, [x0, #0x30]
        stp     x17, x22, [x0, #0x40]
        stp     x3, x8, [x0, #0x50]
        stp     x13, x28, [x0, #0x60]
        stp     x23, x4, [x0, #0x70]
        stp     x9, x14, [x0, #0x80]
        stp     x19, x24, [x0, #0x90]
        stp     x5, x10, [x0, #0xa0]
        stp     x15, x20, [x0, #0xb0]
        str     x25, [x0, #0xc0]
        sub     x0, x0, #0x190
        add     x0, x0, #0x258
        ldp     x1, x6, [x0]
        ldp     x11, x16, [x0, #0x10]
        ldp     x21, x2, [x0, #0x20]
        ldp     x7, x12, [x0, #0x30]
        ldp     x17, x22, [x0, #0x40]
        ldp     x3, x8, [x0, #0x50]
        ldp     x13, x28, [x0, #0x60]
        ldp     x23, x4, [x0, #0x70]
        ldp     x9, x14, [x0, #0x80]
        ldp     x19, x24, [x0, #0x90]
        ldp     x5, x10, [x0, #0xa0]
        ldp     x15, x20, [x0, #0xb0]
        ldr     x25, [x0, #0xc0]
        sub     x0, x0, #0x258
        b       Lsha3_keccak4_f1600_initial


Lsha3_keccak4_f1600_done:
        ldr     x0, [sp]
        add     x0, x0, #0x258
        stp     x1, x6, [x0]
        stp     x11, x16, [x0, #0x10]
        stp     x21, x2, [x0, #0x20]
        stp     x7, x12, [x0, #0x30]
        stp     x17, x22, [x0, #0x40]
        stp     x3, x8, [x0, #0x50]
        stp     x13, x28, [x0, #0x60]
        stp     x23, x4, [x0, #0x70]
        stp     x9, x14, [x0, #0x80]
        stp     x19, x24, [x0, #0x90]
        stp     x5, x10, [x0, #0xa0]
        stp     x15, x20, [x0, #0xb0]
        str     x25, [x0, #0xc0]
        sub     x0, x0, #0x258

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

        CFI_STACKLOAD2(d14,d15,0xc0)
        CFI_STACKLOAD2(d12,d13,0xb0)
        CFI_STACKLOAD2(d10,d11,0xa0)
        CFI_STACKLOAD2(d8,d9,0x90)
        CFI_STACKLOAD2(x19,x20,0x30)
        CFI_STACKLOAD2(x21,x22,0x40)
        CFI_STACKLOAD2(x23,x24,0x50)
        CFI_STACKLOAD2(x25,x26,0x60)
        CFI_STACKLOAD2(x27,x28,0x70)
        CFI_STACKLOAD2(x29,x30,0x80)
        CFI_INC_SP(224)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(sha3_keccak4_f1600)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
