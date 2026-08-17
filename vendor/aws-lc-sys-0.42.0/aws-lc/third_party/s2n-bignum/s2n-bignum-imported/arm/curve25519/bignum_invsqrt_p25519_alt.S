// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Inverse square root modulo p_25519 = 2^255 - 19
// Input x[4]; output function return (Legendre symbol) and z[4]
//
// extern int64_t bignum_invsqrt_p25519_alt(uint64_t z[static 4],const uint64_t x[static 4]);
//
// Given a 4-digit input x, returns a modular inverse square root mod p_25519,
// i.e. a z such that x * z^2 == 1 (mod p_25519), whenever one exists. The
// inverse square root z is chosen so that its LSB is even (note that p_25519-z
// is another possibility). The function return is the Legendre/Jacobi symbol
// (x//p_25519), which indicates whether indeed x has a modular inverse square
// root and hence whether the result is meaningful:
//
//   0: x is divisible by p_25519 so trivially there is no inverse square root
//  +1: x is coprime to p_25519 and z is indeed an inverse square root
//  -1: x is coprime to p_25519 but there is no (inverse or direct) square root
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_invsqrt_p25519_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_invsqrt_p25519_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_invsqrt_p25519_alt)


        .text
        .balign 4

// Size in bytes of a 64-bit word

#define N 8

// Pointer-offset pairs for temporaries on stack

#define a sp, #0
#define b sp, #(4*N)
#define s sp, #(8*N)
#define t sp, #(12*N)

// Other temporary variables in register

#define res x19

// Total size to reserve on the stack

#define NSPACE 16*N

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

// Macros wrapping up calls to the local subroutines

#define mulp(dest,src1,src2)                                            \
        add     x0, dest __LF                                              \
        add     x1, src1 __LF                                              \
        add     x2, src2 __LF                                              \
        CFI_BL(Lbignum_invsqrt_p25519_alt_mul_p25519)

#define nsqr(dest,n,src)                                                \
        add     x0, dest __LF                                              \
        mov     x1, n __LF                                                 \
        add     x2, src __LF                                               \
        CFI_BL(Lbignum_invsqrt_p25519_alt_nsqr_p25519)

S2N_BN_SYMBOL(bignum_invsqrt_p25519_alt):
        CFI_START

// Save registers and make room for temporaries

        CFI_PUSH2(x19,x30)
        CFI_DEC_SP(NSPACE)

// Save the return pointer for the end so we can overwrite x0 later

        mov     res, x0

// Set up reduced version of the input argument a = x mod p_25519. Then
// get the candidate inverse square root s = a^{252-3}

        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]
        mov     x7, #19
        lsr     x6, x5, #63
        madd    x6, x7, x6, x7
        adds    x2, x2, x6
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        orr     x5, x5, #0x8000000000000000
        adcs    x5, x5, xzr
        csel    x6, x7, xzr, lo
        subs    x2, x2, x6
        sbcs    x3, x3, xzr
        sbcs    x4, x4, xzr
        sbc     x5, x5, xzr
        and     x5, x5, #0x7fffffffffffffff
        stp     x2, x3, [a]
        stp     x4, x5, [a+16]

  // Power 2^2 - 1 = 3

        nsqr(t,1,a)
        mulp(t,t,a)

  // Power 2^4 - 1 = 15

        nsqr(s,2,t)
        mulp(t,s,t)

  // Power 2^5 - 1 = 31

        nsqr(s,1,t)
        mulp(b,s,a)

  // Power 2^10 - 1

        nsqr(s,5,b)
        mulp(t,s,b)

  // Power 2^20 - 1

        nsqr(s,10,t)
        mulp(t,s,t)

  // Power 2^25 - 1

        nsqr(s,5,t)
        mulp(b,s,b)

  // Power 2^50 - 1

        nsqr(s,25,b)
        mulp(t,s,b)

  // Power 2^100 - 1
        nsqr(s,50,t)
        mulp(t,s,t)

  // Power 2^125 - 1

        nsqr(s,25,t)
        mulp(b,s,b)

  // Power 2^250 - 1

        nsqr(s,125,b)
        mulp(b,s,b)

  // Power 2^252 - 3

        nsqr(s,2,b)
        mulp(s,s,a)

// s = a^{2^252-3} is now one candidate inverse square root.
// Generate the other one t = s * j_25519 where j_25519 = sqrt(-1)

        movbig(x0, #0xc4ee, #0x1b27, #0x4a0e, #0xa0b0)
        movbig(x1, #0x2f43, #0x1806, #0xad2f, #0xe478)
        movbig(x2, #0x2b4d, #0x0099, #0x3dfb, #0xd7a7)
        movbig(x3, #0x2b83, #0x2480, #0x4fc1, #0xdf0b)
        stp     x0, x1, [t]
        stp     x2, x3, [t+16]
        mulp(t,s,t)

// Now multiplex between them according to whether a * s^2 = 1

        nsqr(b,1,s)
        mulp(b,a,b)

        ldp     x10, x11, [b]
        eor     x10, x10, #1
        ldp     x12, x13, [b+16]
        orr     x10, x10, x11
        orr     x12, x12, x13
        orr     x10, x10, x12
        cmp     x10, xzr

        ldp     x10, x11, [s]
        ldp     x14, x15, [t]
        csel    x10, x10, x14, eq
        csel    x11, x11, x15, eq
        ldp     x12, x13, [s+16]
        ldp     x16, x17, [t+16]
        csel    x12, x12, x16, eq
        csel    x13, x13, x17, eq

// For definiteness, choose "positive" (LSB=0) inverse square root

        mov     x14, #-19
        subs    x14, x14, x10
        mov     x16, #-1
        sbcs    x15, x16, x11
        mov     x17, #0x7FFFFFFFFFFFFFFF
        sbcs    x16, x16, x12
        sbc     x17, x17, x13

        tst     x10, #1
        csel    x10, x10, x14, eq
        csel    x11, x11, x15, eq
        csel    x12, x12, x16, eq
        csel    x13, x13, x17, eq

        mov     x2, res
        stp     x10, x11, [x2]
        stp     x12, x13, [x2, #16]

// Determine if it is is indeed an inverse square root, also distinguishing
// the degenerate x * z^2 == 0 (mod p_25519) case, which is equivalent to
// x == 0 (mod p_25519). Hence return the Legendre-Jacobi symbol as required.

        add     x0, b
        mov     x1, #1
        CFI_BL(Lbignum_invsqrt_p25519_alt_nsqr_p25519)
        mulp(b,a,b)

        ldp     x10, x11, [b]
        eor     x14, x10, #1
        ldp     x12, x13, [b+16]
        orr     x10, x10, x11
        orr     x14, x14, x11
        orr     x12, x12, x13
        orr     x10, x10, x12
        orr     x14, x14, x12

        cmp     x14, xzr
        mov     x0, #1
        cneg    x0, x0, ne

        cmp     x10, xzr
        csel    x0, x0, xzr, ne

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x19,x30)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_invsqrt_p25519_alt)

// *************************************************************
// Local z = x * y
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_invsqrt_p25519_alt_mul_p25519)

Lbignum_invsqrt_p25519_alt_mul_p25519:
        CFI_START
        ldp     x3, x4, [x1]
        ldp     x7, x8, [x2]
        mul     x12, x3, x7
        umulh   x13, x3, x7
        mul     x11, x3, x8
        umulh   x14, x3, x8
        adds    x13, x13, x11
        ldp     x9, x10, [x2, #16]
        mul     x11, x3, x9
        umulh   x15, x3, x9
        adcs    x14, x14, x11
        mul     x11, x3, x10
        umulh   x16, x3, x10
        adcs    x15, x15, x11
        adc     x16, x16, xzr
        ldp     x5, x6, [x1, #16]
        mul     x11, x4, x7
        adds    x13, x13, x11
        mul     x11, x4, x8
        adcs    x14, x14, x11
        mul     x11, x4, x9
        adcs    x15, x15, x11
        mul     x11, x4, x10
        adcs    x16, x16, x11
        umulh   x3, x4, x10
        adc     x3, x3, xzr
        umulh   x11, x4, x7
        adds    x14, x14, x11
        umulh   x11, x4, x8
        adcs    x15, x15, x11
        umulh   x11, x4, x9
        adcs    x16, x16, x11
        adc     x3, x3, xzr
        mul     x11, x5, x7
        adds    x14, x14, x11
        mul     x11, x5, x8
        adcs    x15, x15, x11
        mul     x11, x5, x9
        adcs    x16, x16, x11
        mul     x11, x5, x10
        adcs    x3, x3, x11
        umulh   x4, x5, x10
        adc     x4, x4, xzr
        umulh   x11, x5, x7
        adds    x15, x15, x11
        umulh   x11, x5, x8
        adcs    x16, x16, x11
        umulh   x11, x5, x9
        adcs    x3, x3, x11
        adc     x4, x4, xzr
        mul     x11, x6, x7
        adds    x15, x15, x11
        mul     x11, x6, x8
        adcs    x16, x16, x11
        mul     x11, x6, x9
        adcs    x3, x3, x11
        mul     x11, x6, x10
        adcs    x4, x4, x11
        umulh   x5, x6, x10
        adc     x5, x5, xzr
        umulh   x11, x6, x7
        adds    x16, x16, x11
        umulh   x11, x6, x8
        adcs    x3, x3, x11
        umulh   x11, x6, x9
        adcs    x4, x4, x11
        adc     x5, x5, xzr
        mov     x7, #38
        mul     x11, x7, x16
        umulh   x9, x7, x16
        adds    x12, x12, x11
        mul     x11, x7, x3
        umulh   x3, x7, x3
        adcs    x13, x13, x11
        mul     x11, x7, x4
        umulh   x4, x7, x4
        adcs    x14, x14, x11
        mul     x11, x7, x5
        umulh   x5, x7, x5
        adcs    x15, x15, x11
        cset    x16, hs
        adds    x15, x15, x4
        adc     x16, x16, x5
        cmn     x15, x15
        orr     x15, x15, #0x8000000000000000
        adc     x8, x16, x16
        mov     x7, #19
        madd    x11, x7, x8, x7
        adds    x12, x12, x11
        adcs    x13, x13, x9
        adcs    x14, x14, x3
        adcs    x15, x15, xzr
        csel    x7, x7, xzr, lo
        subs    x12, x12, x7
        sbcs    x13, x13, xzr
        sbcs    x14, x14, xzr
        sbc     x15, x15, xzr
        and     x15, x15, #0x7fffffffffffffff
        stp     x12, x13, [x0]
        stp     x14, x15, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_invsqrt_p25519_alt_mul_p25519)

// *************************************************************
// Local z = 2^n * x
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_invsqrt_p25519_alt_nsqr_p25519)

Lbignum_invsqrt_p25519_alt_nsqr_p25519:
        CFI_START

// Copy input argument into [x5;x4;x3;x2] (overwriting input pointer x20

        ldp     x6, x3, [x2]
        ldp     x4, x5, [x2, #16]
        mov     x2, x6

// Main squaring loop, accumulating in [x5;x4;x3;x2] consistently and
// only ensuring the intermediates are < 2 * p_25519 = 2^256 - 38

Lbignum_invsqrt_p25519_alt_loop:
        mul     x9, x2, x3
        umulh   x10, x2, x3
        mul     x11, x2, x5
        umulh   x12, x2, x5
        mul     x7, x2, x4
        umulh   x6, x2, x4
        adds    x10, x10, x7
        adcs    x11, x11, x6
        mul     x7, x3, x4
        umulh   x6, x3, x4
        adc     x6, x6, xzr
        adds    x11, x11, x7
        mul     x13, x4, x5
        umulh   x14, x4, x5
        adcs    x12, x12, x6
        mul     x7, x3, x5
        umulh   x6, x3, x5
        adc     x6, x6, xzr
        adds    x12, x12, x7
        adcs    x13, x13, x6
        adc     x14, x14, xzr
        adds    x9, x9, x9
        adcs    x10, x10, x10
        adcs    x11, x11, x11
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adcs    x14, x14, x14
        cset    x6, hs
        umulh   x7, x2, x2
        mul     x8, x2, x2
        adds    x9, x9, x7
        mul     x7, x3, x3
        adcs    x10, x10, x7
        umulh   x7, x3, x3
        adcs    x11, x11, x7
        mul     x7, x4, x4
        adcs    x12, x12, x7
        umulh   x7, x4, x4
        adcs    x13, x13, x7
        mul     x7, x5, x5
        adcs    x14, x14, x7
        umulh   x7, x5, x5
        adc     x6, x6, x7
        mov     x3, #38
        mul     x7, x3, x12
        umulh   x4, x3, x12
        adds    x8, x8, x7
        mul     x7, x3, x13
        umulh   x13, x3, x13
        adcs    x9, x9, x7
        mul     x7, x3, x14
        umulh   x14, x3, x14
        adcs    x10, x10, x7
        mul     x7, x3, x6
        umulh   x6, x3, x6
        adcs    x11, x11, x7
        cset    x12, hs
        adds    x11, x11, x14
        adc     x12, x12, x6
        cmn     x11, x11
        bic     x11, x11, #0x8000000000000000
        adc     x2, x12, x12
        mov     x3, #0x13
        mul     x7, x3, x2
        adds    x2, x8, x7
        adcs    x3, x9, x4
        adcs    x4, x10, x13
        adc     x5, x11, xzr

// Loop as applicable

        subs    x1, x1, #1
        bne     Lbignum_invsqrt_p25519_alt_loop

// We know the intermediate result x < 2^256 - 38, and now we do strict
// modular reduction mod 2^255 - 19. Note x < 2^255 - 19 <=> x + 19 < 2^255
// which is equivalent to a "pl" condition.

        adds    x6, x2, #19
        adcs    x7, x3, xzr
        adcs    x8, x4, xzr
        adcs    x9, x5, xzr

        csel    x2, x2, x6, pl
        csel    x3, x3, x7, pl
        csel    x4, x4, x8, pl
        csel    x5, x5, x9, pl
        bic     x5, x5, #0x8000000000000000

// Copy result back into destination and return

        stp     x2, x3, [x0]
        stp     x4, x5, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_invsqrt_p25519_alt_nsqr_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
