// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_mod_p384_6(uint64_t z[static 6], const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p384_6)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p384_6)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p384_6)
        .text
        .balign 4

#define z x0
#define x x1

#define n0 x2
#define n1 x3
#define n2 x4
#define n3 x5
#define n4 x6
#define n5 x7

#define d0 x8
#define d1 x9
#define d2 x10
#define d3 x11
#define d4 x12
#define d5 x13


S2N_BN_SYMBOL(bignum_mod_p384_6):
        CFI_START

// Load the complicated lower three words of p_384 = [-1;-1;-1;n2;n1;n0]

        mov     n0, #0x00000000ffffffff
        mov     n1, #0xffffffff00000000
        mov     n2, #0xfffffffffffffffe

// Load the input number

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        ldp     d4, d5, [x, #32]

// Do the subtraction. Since the top three words of p_384 are all 1s
// we can devolve the top to adding 0, thanks to the inverted carry.

        subs    n0, d0, n0
        sbcs    n1, d1, n1
        sbcs    n2, d2, n2
        adcs    n3, d3, xzr
        adcs    n4, d4, xzr
        adcs    n5, d5, xzr

// Now if the carry is *clear* (inversion at work) the subtraction carried
// and hence we should have done nothing, so we reset each n_i = d_i

        csel    n0, d0, n0, cc
        csel    n1, d1, n1, cc
        csel    n2, d2, n2, cc
        csel    n3, d3, n3, cc
        csel    n4, d4, n4, cc
        csel    n5, d5, n5, cc

// Store the end result

        stp     n0, n1, [z]
        stp     n2, n3, [z, #16]
        stp     n4, n5, [z, #32]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p384_6)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
