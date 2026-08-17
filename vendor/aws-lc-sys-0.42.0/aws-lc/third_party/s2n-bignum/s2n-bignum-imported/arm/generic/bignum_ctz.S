// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Count trailing zero bits
// Input x[k]; output function return
//
//    extern uint64_t bignum_ctz(uint64_t k, const uint64_t *x);
//
//
// In the case of a zero bignum as input the result is 64 * k
//
// In principle this has a precondition k < 2^58, but obviously that
// is always true in practice because of address space limitations
//
// Standard ARM ABI: X0 = k, X1 = x, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_ctz)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_ctz)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_ctz)
        .text
        .balign 4

#define k x0
#define x x1
#define i x2
#define w x3
#define a x4


S2N_BN_SYMBOL(bignum_ctz):
        CFI_START

// If the bignum is zero-length, x0 is already the right answer of 0

        cbz     k, Lbignum_ctz_end

// Use w = a[i] to store nonzero words in a top-down sweep
// Set the initial default to be as if we had a 1 word directly above

        mov     i, k
        mov     w, #1

Lbignum_ctz_loop:
        sub     k, k, #1
        ldr     a, [x, k, lsl #3]
        cmp     a, #0
        csel    i, k, i, ne
        csel    w, a, w, ne
        cbnz    k, Lbignum_ctz_loop

// Now w = a[i] is the lowest nonzero word, or in the zero case the
// default of the "extra" 1 = a[k]. We now want 64*i + ctz(w).
//
// ARM doesn't have a direct word ctz instruction, so we emulate it via
// ctz(w) = 64 - clz(~w & (w-1)). This is depending, for cases of the form
// ctz(....1), on the behavior clz(0) = 64, which is guaranteed according
// to the ARM manual.

        mvn     a, w
        sub     w, w, #1
        add     i, i, #1
        and     w, w, a
        lsl     i, i, #6
        clz     a, w
        sub     x0, i, a

Lbignum_ctz_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_ctz)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
