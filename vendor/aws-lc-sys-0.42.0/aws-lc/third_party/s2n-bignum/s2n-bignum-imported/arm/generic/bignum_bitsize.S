// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Return size of bignum in bits
// Input x[k]; output function return
//
//    extern uint64_t bignum_bitsize(uint64_t k, const uint64_t *x);
//
// In the case of a zero bignum as input the result is 0
//
// In principle this has a precondition k < 2^58, but obviously that
// is always true in practice because of address space limitations.
//
// Standard ARM ABI: X0 = k, X1 = x, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_bitsize)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_bitsize)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_bitsize)
        .text
        .balign 4

#define k x0
#define x x1
#define i x2
#define w x3
#define a x4
#define j x5


S2N_BN_SYMBOL(bignum_bitsize):
        CFI_START

// If the bignum is zero-length, x0 is already the right answer of 0

        cbz     k, Lbignum_bitsize_end

// Use w = a[i-1] to store nonzero words in a bottom-up sweep
// Set the initial default to be as if we had a 11...11 word directly below

        mov     i, xzr
        mov     w, #-1
        mov     j, xzr
Lbignum_bitsize_loop:
        ldr     a, [x, j, lsl #3]
        add     j, j, #1
        cmp     a, #0
        csel    i, j, i, ne
        csel    w, a, w, ne
        cmp     j, k
        bne     Lbignum_bitsize_loop

// Now w = a[i-1] is the highest nonzero word, or in the zero case the
// default of the "extra" 11...11 = a[0-1]. We now want 64* i - clz(w).
// Note that this code does not rely on the behavior of the clz instruction
// for zero inputs, though the ARM manual does in fact guarantee clz(0) = 64.

        lsl     i, i, #6
        clz     a, w
        sub     x0, i, a

Lbignum_bitsize_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_bitsize)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
