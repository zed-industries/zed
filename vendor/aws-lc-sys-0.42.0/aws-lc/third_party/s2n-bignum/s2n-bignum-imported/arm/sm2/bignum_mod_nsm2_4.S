// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_mod_nsm2_4(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Reduction is modulo the group order of the GM/T 0003-2012 curve SM2.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_nsm2_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_nsm2_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_nsm2_4)
        .text
        .balign 4

#define z x0
#define x x1

#define n0 x2
#define n1 x3
#define n2 x4
#define n3 x5

#define d0 x6
#define d1 x7
#define d2 x8
#define d3 x9

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(bignum_mod_nsm2_4):
        CFI_START

// Load the complicated three words of n_sm2, the other being all 1s

       movbig( n0, #0x53BB, #0xF409, #0x39D5, #0x4123)
       movbig( n1, #0x7203, #0xDF6B, #0x21C6, #0x052B)
       mov      n3, #0xFFFFFFFEFFFFFFFF

// Load the input number

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Do the subtraction. Since word 2 of n_sm2 is all 1s, that can be
// done by adding zero with carry, thanks to the inverted carry.

        subs    n0, d0, n0
        sbcs    n1, d1, n1
        adcs    n2, d2, xzr
        sbcs    n3, d3, n3

// Now if the carry is *clear* (inversion at work) the subtraction carried
// and hence we should have done nothing, so we reset each n_i = d_i

        csel    n0, d0, n0, cc
        csel    n1, d1, n1, cc
        csel    n2, d2, n2, cc
        csel    n3, d3, n3, cc

// Store the end result

        stp     n0, n1, [z]
        stp     n2, n3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_nsm2_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
