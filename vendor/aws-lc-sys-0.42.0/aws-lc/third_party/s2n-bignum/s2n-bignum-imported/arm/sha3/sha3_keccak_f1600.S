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
// extern void sha3_keccak_f1600(uint64_t a[static 25],
//                               const uint64_t rc[static 24]);
//
// Standard ARM ABI: X0 = a, X1 = rc
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sha3_keccak_f1600)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sha3_keccak_f1600)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sha3_keccak_f1600)
        .text
        .balign 4

S2N_BN_SYMBOL(sha3_keccak_f1600):
        CFI_START

// This is almost exactly the same as the scalar code in the mlkem-native
// repository here:
//
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/fips202/native/aarch64/src/keccak_f1600_x1_scalar_asm.S
//
// The only change is the use of simple 64-bit addressing in place of
// the unnecessary down-and-upcasting arising from [x9, w25, uxtw #3]

        CFI_DEC_SP(128)
        CFI_STACKSAVE2X(x19,x20,32,40)
        CFI_STACKSAVE2X(x21,x22,48,56)
        CFI_STACKSAVE2X(x23,x24,64,72)
        CFI_STACKSAVE2X(x25,x26,80,88)
        CFI_STACKSAVE2X(x27,x28,96,104)
        CFI_STACKSAVE2X(x29,x30,112,120)

Lsha3_keccak_f1600_initial:
        mov     x26, x1
        str     x26, [sp, #8]
        ldp     x1, x6, [x0]
        ldp     x11, x16, [x0, #16]
        ldp     x21, x2, [x0, #32]
        ldp     x7, x12, [x0, #48]
        ldp     x17, x22, [x0, #64]
        ldp     x3, x8, [x0, #80]
        ldp     x13, x28, [x0, #96]
        ldp     x23, x4, [x0, #112]
        ldp     x9, x14, [x0, #128]
        ldp     x19, x24, [x0, #144]
        ldp     x5, x10, [x0, #160]
        ldp     x15, x20, [x0, #176]
        ldr     x25, [x0, #192]
        str     x0, [sp]
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
        str     x23, [sp, #24]
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
        ldr     x27, [sp, #24]
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
        ldr     x11, [sp, #8]
        eor     x20, x17, x4, ror #21
        eor     x17, x24, x9, ror #47
        mov     x24, #0x1
        bic     x9, x0, x16, ror #9
        str     x24, [sp, #16]
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

// Main loop maintains the invariant that there are deferred rotations to
// reach the full effect of i Keccak rounds. By absorbing rotations into
// shifted operands, the total number of rotation instructions is reduced
// relative to a naive implementation.

Lsha3_keccak_f1600_loop:
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
        ror     x30, x27, #62
        eor     x30, x30, x26, ror #57
        ror     x26, x26, #58
        eor     x16, x30, x16
        eor     x28, x30, x28, ror #63
        str     x28, [sp, #24]
        eor     x29, x29, x17, ror #36
        eor     x28, x1, x2, ror #61
        eor     x19, x30, x19, ror #37
        eor     x29, x29, x20, ror #2
        eor     x28, x28, x4, ror #54
        eor     x26, x26, x0, ror #55
        eor     x28, x28, x3, ror #39
        eor     x28, x28, x5, ror #25
        ror     x0, x0, #56
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
        ldr     x25, [sp, #16]
        eor     x17, x10, x9, ror #47
        ldr     x9, [sp, #8]
        eor     x15, x20, x28, ror #27
        bic     x20, x4, x28, ror #2
        eor     x10, x20, x1, ror #50
        bic     x20, x11, x27, ror #60
        eor     x20, x20, x4, ror #21
        bic     x4, x28, x1, ror #48
        bic     x1, x1, x11, ror #57
        ldr     x28, [x9, x25, lsl #3]
        ldr     x9, [sp, #24]
        add     x25, x25, #0x1
        str     x25, [sp, #16]
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
        ble     Lsha3_keccak_f1600_loop

// Final rotations

        ror     x6, x6, #43
        ror     x11, x11, #50
        ror     x21, x21, #20
        ror     x2, x2, #61
        ror     x7, x7, #19
        ror     x12, x12, #3
        ror     x17, x17, #36
        ror     x22, x22, #44
        ror     x3, x3, #39
        ror     x8, x8, #56
        ror     x13, x13, #46
        ror     x28, x28, #63
        ror     x23, x23, #58
        ror     x4, x4, #54
        ror     x9, x9, #49
        ror     x14, x14, #8
        ror     x19, x19, #37
        ror     x24, x24, #28
        ror     x5, x5, #25
        ror     x10, x10, #23
        ror     x15, x15, #62
        ror     x20, x20, #2
        ror     x25, x25, #9
        ldr     x0, [sp]
        stp     x1, x6, [x0]
        stp     x11, x16, [x0, #16]
        stp     x21, x2, [x0, #32]
        stp     x7, x12, [x0, #48]
        stp     x17, x22, [x0, #64]
        stp     x3, x8, [x0, #80]
        stp     x13, x28, [x0, #96]
        stp     x23, x4, [x0, #112]
        stp     x9, x14, [x0, #128]
        stp     x19, x24, [x0, #144]
        stp     x5, x10, [x0, #160]
        stp     x15, x20, [x0, #176]
        str     x25, [x0, #192]
        CFI_STACKLOAD2(x19,x20,32)
        CFI_STACKLOAD2(x21,x22,48)
        CFI_STACKLOAD2(x23,x24,64)
        CFI_STACKLOAD2(x25,x26,80)
        CFI_STACKLOAD2(x27,x28,96)
        CFI_STACKLOAD2(x29,x30,112)
        CFI_INC_SP(128)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(sha3_keccak_f1600)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
