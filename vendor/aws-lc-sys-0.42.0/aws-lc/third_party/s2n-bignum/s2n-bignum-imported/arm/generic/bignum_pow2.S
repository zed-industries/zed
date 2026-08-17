// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Return bignum of power of 2, z := 2^n
// Input n; output z[k]
//
//    extern void bignum_pow2(uint64_t k, uint64_t *z, uint64_t n);
//
// The result is as usual mod 2^{64*k}, so will be zero if n >= 64*k.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_pow2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_pow2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_pow2)
        .text
        .balign 4

#define k x0
#define z x1
#define n x2

#define w x3
#define i x4
#define a x5


S2N_BN_SYMBOL(bignum_pow2):
        CFI_START

// If k = 0 the result is trivially zero

        cbz     k, Lbignum_pow2_end

// Create the index n at which to write the nonzero word and the word w itself
// Note that the ARM manual explicitly says that shift counts are taken modulo
// the datasize, so we don't need to mask the lower 6 bits of n ourselves.

        mov     w, #1
        lsl     w, w, n
        lsr     n, n, #6

// Now in a constant-time fashion set the n'th word to w and others to zero

        mov     i, xzr
Lbignum_pow2_loop:
        cmp     i, n
        csel    a, w, xzr, eq
        str     a, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_pow2_loop

Lbignum_pow2_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_pow2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
