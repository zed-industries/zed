// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_384
// Input x[6]; output z[6]
//
//    extern void bignum_mod_n384_6(uint64_t z[static 6], const uint64_t x[static 6]);
//
// Reduction is modulo the group order of the NIST curve P-384.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n384_6)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n384_6)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n384_6)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11

#define a %rax

// Re-use the input pointer as a temporary once we're done

#define c %rsi



S2N_BN_SYMBOL(bignum_mod_n384_6):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the input and compute x + (2^384 - n_384)

        movq    $0x1313e695333ad68d, a
        movq    (x), d0
        addq    a, d0
        movq    $0xa7e5f24db74f5885, d1
        adcq    8(x), d1
        movq    $0x389cb27e0bc8d220, d2
        adcq    16(x), d2
        movq    24(x), d3
        adcq    $0, d3
        movq    32(x), d4
        adcq    $0, d4
        movq    40(x), d5
        adcq    $0, d5

// Now CF is set iff 2^384 <= x + (2^384 - n_384), i.e. iff n_384 <= x.
// Create a mask for the condition x < n. We now want to subtract the
// masked (2^384 - n_384), but because we're running out of registers
// without using a save-restore sequence, we need some contortions.
// Create the lowest digit (re-using a kept from above)

        sbbq    c, c
        notq    c
        andq    c, a

// Do the first digit of addition and writeback

        subq    a, d0
        movq    d0, (z)

// Preserve carry chain and do the next digit

        sbbq    d0, d0
        movq    $0xa7e5f24db74f5885, a
        andq    c, a
        negq    d0
        sbbq    a, d1
        movq    d1, 8(z)

// Preserve carry chain once more and do remaining digits

        sbbq    d0, d0
        movq    $0x389cb27e0bc8d220, a
        andq    c, a
        negq    d0
        sbbq    a, d2
        movq    d2, 16(z)
        sbbq    $0, d3
        movq    d3, 24(z)
        sbbq    $0, d4
        movq    d4, 32(z)
        sbbq    $0, d5
        movq    d5, 40(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n384_6)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
