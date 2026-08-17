// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

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
// extern void sha3_keccak_f1600_alt2(uint64_t a[static 25],
//                                    const uint64_t rc[static 24]);
//
// Standard ARM ABI: X0 = a, X1 = rc
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sha3_keccak_f1600_alt2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sha3_keccak_f1600_alt2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sha3_keccak_f1600_alt2)
        .text
        .balign 4

// Code was generated from simple and straightforward C with
// gcc -O3 -mcpu=cortex-a72 -ffixed-x18 -c sha3_keccak_f1600_alt2.c

S2N_BN_SYMBOL(sha3_keccak_f1600_alt2):
        CFI_START
        stp     x29, x30, [sp, #-208]!
        .cfi_adjust_cfa_offset 208
        .cfi_rel_offset x29, 0
        .cfi_rel_offset x30, 8
        mov     x2, x0
        mov     x29, sp
        ldr     x0, [x0]
        ldp     x6, x15, [x2, #48]
        CFI_STACKSAVE2(x19,x20,16)
        str     x0, [sp, #112]
        ldr     x0, [x2, #8]
        CFI_STACKSAVE2(x21,x22,32)
        ldr     x30, [x2, #24]
        str     x0, [sp, #152]
        ldr     x0, [x2, #16]
        CFI_STACKSAVE2(x23,x24,48)
        CFI_STACKSAVE2(x25,x26,64)
        str     x0, [sp, #160]
        ldr     x0, [x2, #32]
        CFI_STACKSAVE2(x27,x28,80)
        ldr     x25, [x2, #72]
        str     x0, [sp, #120]
        ldr     x0, [x2, #40]
        str     xzr, [sp, #144]
        str     x1, [sp, #192]
        str     x0, [sp, #168]
        ldr     x0, [x2, #64]
        str     x0, [sp, #128]
        ldp     x0, x13, [x2, #80]
        ldp     x5, x16, [x2, #96]
        ldp     x17, x26, [x2, #112]
        mov     x23, x16
        mov     x16, x15
        ldp     x4, x22, [x2, #128]
        mov     x15, x0
        mov     x19, x17
        ldp     x28, x24, [x2, #144]
        ldp     x3, x9, [x2, #160]
        ldp     x10, x27, [x2, #176]
        str     x30, [sp, #136]
        mov     x30, x22
        ldr     x8, [x2, #192]
        str     x2, [sp, #200]
        mov     x2, x3
        mov     x3, x4
        mov     x4, x10
        mov     x7, x8
        mov     x8, x6
        mov     x6, x28
        mov     x28, x25
        mov     x25, x13

Lsha3_keccak_f1600_alt2_loop:
        ldp     x17, x21, [sp, #144]
        eor     x11, x15, x26
        eor     x12, x5, x30
        ldp     x13, x22, [sp, #160]
        ldr     x1, [sp, #192]
        eor     x10, x13, x16
        ldr     x0, [sp, #112]
        eor     x10, x10, x12
        eor     x12, x23, x6
        eor     x10, x10, x4
        ldr     x20, [sp, #128]
        ldr     x1, [x1, x17, lsl #3]
        eor     x14, x0, x22
        eor     x0, x25, x3
        eor     x14, x14, x11
        ldr     x11, [sp, #136]
        eor     x14, x14, x2
        str     x1, [sp, #160]
        eor     x1, x21, x8
        eor     x1, x1, x0
        ldr     x0, [sp, #120]
        eor     x11, x11, x20
        eor     x1, x1, x9
        eor     x20, x14, x10, ror #63
        eor     x11, x11, x12
        eor     x12, x19, x24
        eor     x11, x11, x27
        eor     x25, x20, x25
        eor     x14, x11, x14, ror #63
        eor     x3, x20, x3
        eor     x0, x0, x28
        eor     x11, x1, x11, ror #63
        ror     x3, x3, #19
        eor     x0, x0, x12
        add     x12, x17, #0x1
        eor     x0, x0, x7
        eor     x17, x20, x21
        eor     x1, x0, x1, ror #63
        eor     x21, x20, x8
        eor     x20, x20, x9
        eor     x0, x10, x0, ror #63
        ldr     x10, [sp, #120]
        eor     x30, x11, x30
        eor     x8, x11, x13
        eor     x5, x11, x5
        str     x17, [sp, #104]
        ldr     x9, [sp, #136]
        eor     x13, x11, x16
        eor     x17, x14, x28
        eor     x16, x1, x15
        eor     x4, x11, x4
        str     x12, [sp, #144]
        eor     x11, x1, x22
        eor     x7, x14, x7
        eor     x22, x14, x24
        eor     x12, x14, x10
        eor     x10, x14, x19
        eor     x14, x0, x23
        ror     x23, x21, #20
        ror     x21, x25, #54
        ldp     x15, x25, [sp, #104]
        eor     x24, x1, x26
        eor     x19, x0, x9
        eor     x2, x1, x2
        eor     x6, x0, x6
        ror     x8, x8, #2
        ror     x24, x24, #23
        str     x21, [sp, #104]
        ldr     x26, [sp, #160]
        ror     x6, x6, #43
        ror     x5, x5, #21
        eor     x1, x1, x25
        ror     x7, x7, #50
        stp     x8, x24, [sp, #176]
        bic     x28, x6, x5
        ror     x21, x30, #49
        bic     x30, x5, x23
        ror     x4, x4, #3
        ldr     x9, [sp, #128]
        ror     x17, x17, #44
        ror     x19, x19, #36
        eor     x24, x1, x26
        bic     x26, x7, x6
        ldr     x25, [sp, #144]
        eor     x5, x26, x5
        ror     x16, x16, #61
        ror     x22, x22, #56
        ror     x14, x14, #39
        eor     x30, x30, x24
        ror     x13, x13, #58
        eor     x9, x0, x9
        eor     x0, x0, x27
        bic     x27, x23, x1
        eor     x23, x28, x23
        str     x30, [sp, #112]
        bic     x1, x1, x7
        cmp     x25, #0x18
        eor     x7, x27, x7
        bic     x25, x16, x17
        stp     x23, x5, [sp, #152]
        bic     x23, x19, x4
        eor     x5, x25, x19
        eor     x1, x1, x6
        eor     x23, x23, x3
        ror     x11, x11, #28
        bic     x24, x17, x19
        str     x7, [sp, #120]
        ror     x12, x12, #37
        bic     x7, x4, x3
        str     x23, [sp, #128]
        bic     x8, x3, x16
        ldr     x3, [sp, #104]
        ror     x15, x15, #63
        bic     x6, x22, x14
        ror     x0, x0, #8
        str     x1, [sp, #136]
        bic     x19, x13, x15
        bic     x1, x14, x13
        str     x5, [sp, #168]
        eor     x28, x24, x4
        eor     x25, x6, x13
        ldr     x13, [sp, #184]
        bic     x24, x11, x12
        bic     x6, x12, x0
        ldr     x4, [sp, #176]
        bic     x30, x0, x21
        eor     x24, x24, x0
        ror     x2, x2, #46
        ror     x20, x20, #62
        ldr     x0, [sp, #104]
        ror     x10, x10, #25
        ror     x9, x9, #9
        bic     x26, x3, x11
        bic     x3, x21, x3
        bic     x5, x2, x22
        eor     x8, x8, x17
        bic     x27, x4, x20
        bic     x17, x15, x2
        eor     x16, x7, x16
        eor     x15, x1, x15
        bic     x7, x9, x4
        eor     x19, x19, x2
        bic     x1, x13, x10
        bic     x2, x10, x9
        eor     x3, x3, x11
        mov     x11, x4
        bic     x4, x20, x13
        eor     x6, x6, x21
        eor     x26, x26, x12
        eor     x23, x17, x22
        eor     x5, x5, x14
        eor     x30, x30, x0
        eor     x2, x2, x11
        eor     x9, x1, x9
        eor     x7, x7, x20
        eor     x27, x27, x13
        eor     x4, x4, x10
        b.ne    Lsha3_keccak_f1600_alt2_loop

        ldr     x0, [sp, #112]
        mov     x10, x4
        mov     x4, x3
        mov     x3, x2
        mov     x22, x30
        ldr     x2, [sp, #200]
        mov     x13, x25
        mov     x25, x28
        ldr     x30, [sp, #136]
        str     x0, [x2]
        ldr     x0, [sp, #152]
        str     x30, [x2, #24]
        stp     x8, x16, [x2, #48]
        str     x0, [x2, #8]
        ldr     x0, [sp, #160]
        stp     x25, x15, [x2, #72]
        stp     x13, x5, [x2, #88]
        str     x0, [x2, #16]
        ldr     x0, [sp, #120]
        stp     x23, x19, [x2, #104]
        stp     x26, x4, [x2, #120]
        str     x0, [x2, #32]
        ldr     x0, [sp, #168]
        stp     x22, x6, [x2, #136]
        stp     x24, x3, [x2, #152]
        str     x0, [x2, #40]
        ldr     x0, [sp, #128]
        stp     x9, x10, [x2, #168]
        str     x27, [x2, #184]
        str     x0, [x2, #64]
        str     x7, [x2, #192]
        CFI_STACKLOAD2(x19,x20,16)
        CFI_STACKLOAD2(x21,x22,32)
        CFI_STACKLOAD2(x23,x24,48)
        CFI_STACKLOAD2(x25,x26,64)
        CFI_STACKLOAD2(x27,x28,80)
        ldp     x29, x30, [sp], #208
        .cfi_adjust_cfa_offset -208
        .cfi_restore x29
        .cfi_restore x30
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(sha3_keccak_f1600_alt2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
