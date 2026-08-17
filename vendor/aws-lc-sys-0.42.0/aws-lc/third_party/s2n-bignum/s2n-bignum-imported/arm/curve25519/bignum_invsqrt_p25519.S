// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Inverse square root modulo p_25519 = 2^255 - 19
// Input x[4]; output function return (Legendre symbol) and z[4]
//
// extern int64_t bignum_invsqrt_p25519(uint64_t z[static 4],const uint64_t x[static 4]);
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

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_invsqrt_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_invsqrt_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_invsqrt_p25519)


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
        CFI_BL(Lbignum_invsqrt_p25519_mul_p25519)

#define nsqr(dest,n,src)                                                \
        add     x0, dest __LF                                              \
        mov     x1, n __LF                                                 \
        add     x2, src __LF                                               \
        CFI_BL(Lbignum_invsqrt_p25519_nsqr_p25519)

S2N_BN_SYMBOL(bignum_invsqrt_p25519):
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
        CFI_BL(Lbignum_invsqrt_p25519_nsqr_p25519)
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

S2N_BN_SIZE_DIRECTIVE(bignum_invsqrt_p25519)

// *************************************************************
// Local z = x * y
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_invsqrt_p25519_mul_p25519)

Lbignum_invsqrt_p25519_mul_p25519:
        CFI_START
        ldp     x3, x4, [x1]
        ldp     x5, x6, [x2]
        umull   x7, w3, w5
        lsr     x17, x3, #32
        umull   x15, w17, w5
        lsr     x16, x5, #32
        umull   x8, w16, w17
        umull   x16, w3, w16
        adds    x7, x7, x15, lsl #32
        lsr     x15, x15, #32
        adc     x8, x8, x15
        adds    x7, x7, x16, lsl #32
        lsr     x16, x16, #32
        adc     x8, x8, x16
        mul     x9, x4, x6
        umulh   x10, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, lo
        csetm   x16, lo
        adds    x9, x9, x8
        adc     x10, x10, xzr
        subs    x3, x5, x6
        cneg    x3, x3, lo
        cinv    x16, x16, lo
        mul     x15, x4, x3
        umulh   x3, x4, x3
        adds    x8, x7, x9
        adcs    x9, x9, x10
        adc     x10, x10, xzr
        cmn     x16, #1
        eor     x15, x15, x16
        adcs    x8, x15, x8
        eor     x3, x3, x16
        adcs    x9, x3, x9
        adc     x10, x10, x16
        ldp     x3, x4, [x1, #16]
        ldp     x5, x6, [x2, #16]
        umull   x11, w3, w5
        lsr     x17, x3, #32
        umull   x15, w17, w5
        lsr     x16, x5, #32
        umull   x12, w16, w17
        umull   x16, w3, w16
        adds    x11, x11, x15, lsl #32
        lsr     x15, x15, #32
        adc     x12, x12, x15
        adds    x11, x11, x16, lsl #32
        lsr     x16, x16, #32
        adc     x12, x12, x16
        mul     x13, x4, x6
        umulh   x14, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, lo
        csetm   x16, lo
        adds    x13, x13, x12
        adc     x14, x14, xzr
        subs    x3, x5, x6
        cneg    x3, x3, lo
        cinv    x16, x16, lo
        mul     x15, x4, x3
        umulh   x3, x4, x3
        adds    x12, x11, x13
        adcs    x13, x13, x14
        adc     x14, x14, xzr
        cmn     x16, #1
        eor     x15, x15, x16
        adcs    x12, x15, x12
        eor     x3, x3, x16
        adcs    x13, x3, x13
        adc     x14, x14, x16
        ldp     x3, x4, [x1, #16]
        ldp     x15, x16, [x1]
        subs    x3, x3, x15
        sbcs    x4, x4, x16
        csetm   x16, lo
        ldp     x15, x17, [x2]
        subs    x5, x15, x5
        sbcs    x6, x17, x6
        csetm   x17, lo
        eor     x3, x3, x16
        subs    x3, x3, x16
        eor     x4, x4, x16
        sbc     x4, x4, x16
        eor     x5, x5, x17
        subs    x5, x5, x17
        eor     x6, x6, x17
        sbc     x6, x6, x17
        eor     x16, x17, x16
        adds    x11, x11, x9
        adcs    x12, x12, x10
        adcs    x13, x13, xzr
        adc     x14, x14, xzr
        mul     x2, x3, x5
        umulh   x17, x3, x5
        mul     x15, x4, x6
        umulh   x1, x4, x6
        subs    x4, x4, x3
        cneg    x4, x4, lo
        csetm   x9, lo
        adds    x15, x15, x17
        adc     x1, x1, xzr
        subs    x6, x5, x6
        cneg    x6, x6, lo
        cinv    x9, x9, lo
        mul     x5, x4, x6
        umulh   x6, x4, x6
        adds    x17, x2, x15
        adcs    x15, x15, x1
        adc     x1, x1, xzr
        cmn     x9, #1
        eor     x5, x5, x9
        adcs    x17, x5, x17
        eor     x6, x6, x9
        adcs    x15, x6, x15
        adc     x1, x1, x9
        adds    x9, x11, x7
        adcs    x10, x12, x8
        adcs    x11, x13, x11
        adcs    x12, x14, x12
        adcs    x13, x13, xzr
        adc     x14, x14, xzr
        cmn     x16, #1
        eor     x2, x2, x16
        adcs    x9, x2, x9
        eor     x17, x17, x16
        adcs    x10, x17, x10
        eor     x15, x15, x16
        adcs    x11, x15, x11
        eor     x1, x1, x16
        adcs    x12, x1, x12
        adcs    x13, x13, x16
        adc     x14, x14, x16
        mov     x3, #38
        umull   x4, w11, w3
        add     x4, x4, w7, uxtw
        lsr     x7, x7, #32
        lsr     x11, x11, #32
        umaddl  x11, w11, w3, x7
        mov     x7, x4
        umull   x4, w12, w3
        add     x4, x4, w8, uxtw
        lsr     x8, x8, #32
        lsr     x12, x12, #32
        umaddl  x12, w12, w3, x8
        mov     x8, x4
        umull   x4, w13, w3
        add     x4, x4, w9, uxtw
        lsr     x9, x9, #32
        lsr     x13, x13, #32
        umaddl  x13, w13, w3, x9
        mov     x9, x4
        umull   x4, w14, w3
        add     x4, x4, w10, uxtw
        lsr     x10, x10, #32
        lsr     x14, x14, #32
        umaddl  x14, w14, w3, x10
        mov     x10, x4
        lsr     x17, x14, #31
        mov     x5, #19
        umaddl  x5, w5, w17, x5
        add     x7, x7, x5
        adds    x7, x7, x11, lsl #32
        extr    x3, x12, x11, #32
        adcs    x8, x8, x3
        extr    x3, x13, x12, #32
        adcs    x9, x9, x3
        extr    x3, x14, x13, #32
        lsl     x5, x17, #63
        eor     x10, x10, x5
        adc     x10, x10, x3
        mov     x3, #19
        tst     x10, #0x8000000000000000
        csel    x3, x3, xzr, pl
        subs    x7, x7, x3
        sbcs    x8, x8, xzr
        sbcs    x9, x9, xzr
        sbc     x10, x10, xzr
        and     x10, x10, #0x7fffffffffffffff
        stp     x7, x8, [x0]
        stp     x9, x10, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_invsqrt_p25519_mul_p25519)

// *************************************************************
// Local z = 2^n * x
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_invsqrt_p25519_nsqr_p25519)

Lbignum_invsqrt_p25519_nsqr_p25519:
        CFI_START

// Copy input argument into [x13;x12;x11;x10]

        ldp     x10, x11, [x2]
        ldp     x12, x13, [x2, #16]

// Main squaring loop, accumulating in [x13;x12;x11;x10] consistently and
// only ensuring the intermediates are < 2 * p_25519 = 2^256 - 38

Lbignum_invsqrt_p25519_loop:
        umull   x2, w10, w10
        lsr     x14, x10, #32
        umull   x3, w14, w14
        umull   x14, w10, w14
        adds    x2, x2, x14, lsl #33
        lsr     x14, x14, #31
        adc     x3, x3, x14
        umull   x4, w11, w11
        lsr     x14, x11, #32
        umull   x5, w14, w14
        umull   x14, w11, w14
        mul     x15, x10, x11
        umulh   x16, x10, x11
        adds    x4, x4, x14, lsl #33
        lsr     x14, x14, #31
        adc     x5, x5, x14
        adds    x15, x15, x15
        adcs    x16, x16, x16
        adc     x5, x5, xzr
        adds    x3, x3, x15
        adcs    x4, x4, x16
        adc     x5, x5, xzr
        umull   x6, w12, w12
        lsr     x14, x12, #32
        umull   x7, w14, w14
        umull   x14, w12, w14
        adds    x6, x6, x14, lsl #33
        lsr     x14, x14, #31
        adc     x7, x7, x14
        umull   x8, w13, w13
        lsr     x14, x13, #32
        umull   x9, w14, w14
        umull   x14, w13, w14
        mul     x15, x12, x13
        umulh   x16, x12, x13
        adds    x8, x8, x14, lsl #33
        lsr     x14, x14, #31
        adc     x9, x9, x14
        adds    x15, x15, x15
        adcs    x16, x16, x16
        adc     x9, x9, xzr
        adds    x7, x7, x15
        adcs    x8, x8, x16
        adc     x9, x9, xzr
        subs    x10, x10, x12
        sbcs    x11, x11, x13
        csetm   x16, lo
        eor     x10, x10, x16
        subs    x10, x10, x16
        eor     x11, x11, x16
        sbc     x11, x11, x16
        adds    x6, x6, x4
        adcs    x7, x7, x5
        adcs    x8, x8, xzr
        adc     x9, x9, xzr
        umull   x12, w10, w10
        lsr     x5, x10, #32
        umull   x13, w5, w5
        umull   x5, w10, w5
        adds    x12, x12, x5, lsl #33
        lsr     x5, x5, #31
        adc     x13, x13, x5
        umull   x15, w11, w11
        lsr     x5, x11, #32
        umull   x14, w5, w5
        umull   x5, w11, w5
        mul     x4, x10, x11
        umulh   x16, x10, x11
        adds    x15, x15, x5, lsl #33
        lsr     x5, x5, #31
        adc     x14, x14, x5
        adds    x4, x4, x4
        adcs    x16, x16, x16
        adc     x14, x14, xzr
        adds    x13, x13, x4
        adcs    x15, x15, x16
        adc     x14, x14, xzr
        adds    x4, x2, x6
        adcs    x5, x3, x7
        adcs    x6, x6, x8
        adcs    x7, x7, x9
        csetm   x16, lo
        subs    x4, x4, x12
        sbcs    x5, x5, x13
        sbcs    x6, x6, x15
        sbcs    x7, x7, x14
        adcs    x8, x8, x16
        adc     x9, x9, x16
        mov     x10, #38
        umull   x12, w6, w10
        add     x12, x12, w2, uxtw
        lsr     x2, x2, #32
        lsr     x6, x6, #32
        umaddl  x6, w6, w10, x2
        mov     x2, x12
        umull   x12, w7, w10
        add     x12, x12, w3, uxtw
        lsr     x3, x3, #32
        lsr     x7, x7, #32
        umaddl  x7, w7, w10, x3
        mov     x3, x12
        umull   x12, w8, w10
        add     x12, x12, w4, uxtw
        lsr     x4, x4, #32
        lsr     x8, x8, #32
        umaddl  x8, w8, w10, x4
        mov     x4, x12
        umull   x12, w9, w10
        add     x12, x12, w5, uxtw
        lsr     x5, x5, #32
        lsr     x9, x9, #32
        umaddl  x9, w9, w10, x5
        mov     x5, x12
        lsr     x13, x9, #31
        mov     x11, #19
        umull   x11, w11, w13
        add     x2, x2, x11
        adds    x10, x2, x6, lsl #32
        extr    x12, x7, x6, #32
        adcs    x11, x3, x12
        extr    x12, x8, x7, #32
        adcs    x12, x4, x12
        extr    x14, x9, x8, #32
        lsl     x15, x13, #63
        eor     x5, x5, x15
        adc     x13, x5, x14

// Loop as applicable

        subs    x1, x1, #1
        bne     Lbignum_invsqrt_p25519_loop

// We know the intermediate result x < 2^256 - 38, and now we do strict
// modular reduction mod 2^255 - 19. Note x < 2^255 - 19 <=> x + 19 < 2^255
// which is equivalent to a "pl" condition.

        adds    x6, x10, #19
        adcs    x7, x11, xzr
        adcs    x8, x12, xzr
        adcs    x9, x13, xzr

        csel    x10, x10, x6, pl
        csel    x11, x11, x7, pl
        csel    x12, x12, x8, pl
        csel    x13, x13, x9, pl
        bic     x13, x13, #0x8000000000000000

// Copy result back into destination and return

        stp     x10, x11, [x0]
        stp     x12, x13, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_invsqrt_p25519_nsqr_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
