// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Single-word negated modular inverse (-1/a) mod 2^64
// Input a; output function return
//
//    extern uint64_t word_negmodinv(uint64_t a);
//
// A 64-bit function that returns a negated multiplicative inverse mod 2^64
// of its input, assuming that input is odd. Given odd input a, the result z
// will satisfy a * z + 1 == 0 (mod 2^64), i.e. a 64-bit word multiplication
// a * z will give -1.
//
// Standard ARM ABI: X0 = a, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_negmodinv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_negmodinv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_negmodinv)
        .text
        .balign 4

// Use some more intuitive variable names but these in general are aliased
// to each other so need care when interpreting. Overall we only use the
// registers x0, x1 and x2.
//
// There does seem a slight efficiency advantage in putting e' = e^2
// before the x' = x (1 + e) each time. That's the only reason for not
// reversing those and hence being able to alias all the e values to the
// same register.

#define a x0
#define x x1
#define one x2

#define e1 x2
#define e2 x0
#define e4 x2
#define e8 x0

S2N_BN_SYMBOL(word_negmodinv):
        CFI_START

// Initial magical 5-bit approximation x = (a - a<<2) xor 2

        lsl     x, a, #2
        sub     x, a, x
        eor     x, x, #2

// Get error e = a * x + 1 for subsequent correction steps

        mov     one, #1
        madd    e1, a, x, one

// e2 = e^2, x' = x (1 + e) is good to 10 bits

        mul     e2, e1, e1
        madd    x, e1, x, x

// e4 = e^4, x' = x (1 + e^2) is good to 20 bits

        mul     e4, e2, e2
        madd    x, e2, x, x

// e8 = e^8, x' = x (1 + e^4) is good to 40 bits

        mul     e8, e4, e4
        madd    x, e4, x, x

// Final x' = x (1 + e^8) is good to the 64-bit word size

        madd    x0, e8, x, x

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_negmodinv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
