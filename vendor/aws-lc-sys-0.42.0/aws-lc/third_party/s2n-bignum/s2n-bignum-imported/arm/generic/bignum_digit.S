// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Select digit x[n]
// Inputs x[k], n; output function return
//
//    extern uint64_t bignum_digit(uint64_t k, const uint64_t *x, uint64_t n);
//
// n'th digit of a k-digit (digit=64 bits) bignum, in constant-time style.
// Indexing starts at 0, which is the least significant digit (little-endian).
// Returns zero if n >= k, i.e. we read a digit off the end of the bignum.
//
// Standard ARM ABI: X0 = k, X1 = x, X2 = n, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_digit)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_digit)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_digit)
        .text
        .balign 4

#define k x0
#define x x1
#define n x2

#define d x3
#define i x4
#define a x5


S2N_BN_SYMBOL(bignum_digit):
        CFI_START

// For length zero finish immediately (the return value in x0 is 0)

        cbz     k, Lbignum_digit_end

// Set default of zero, run over all the digits and take note of the n'th one

        mov     d, xzr
        mov     i, xzr
Lbignum_digit_loop:
        ldr     a, [x, i, lsl #3]
        cmp     i, n
        csel    d, a, d, eq
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_digit_loop

// Return

        mov     x0, d

Lbignum_digit_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_digit)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
