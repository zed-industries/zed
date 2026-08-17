// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// The x25519 function for curve25519 (byte array arguments)
// Inputs scalar[32] (bytes), point[32] (bytes); output res[32] (bytes)
//
// extern void curve25519_x25519_byte_alt
//   (uint8_t res[static 32],const uint8_t scalar[static 32],
//    const uint8_t point[static 32]);
//
// Given a scalar n and the X coordinate of an input point P = (X,Y) on
// curve25519 (Y can live in any extension field of characteristic 2^255-19),
// this returns the X coordinate of n * P = (X, Y), or 0 when n * P is the
// point at infinity. Both n and X inputs are first slightly modified/mangled
// as specified in the relevant RFC (https://www.rfc-editor.org/rfc/rfc7748);
// in particular the lower three bits of n are set to zero. Does not implement
// the zero-check specified in Section 6.1.
//
// Standard ARM ABI: X0 = res, X1 = scalar, X2 = point
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_x25519_byte_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_x25519_byte_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_x25519_byte_alt)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for the input result argument during the whole body
// and other variables that are only needed prior to the modular inverse.

#define res x23
#define i x20
#define swap x21

// Pointers to result x coord to be written

#define resx res, #0

// Pointer-offset pairs for temporaries on stack with some aliasing.

#define scalar sp, #(0*NUMSIZE)

#define pointx sp, #(1*NUMSIZE)

#define zm sp, #(2*NUMSIZE)
#define sm sp, #(2*NUMSIZE)
#define dpro sp, #(2*NUMSIZE)

#define sn sp, #(3*NUMSIZE)

#define dm sp, #(4*NUMSIZE)

#define zn sp, #(5*NUMSIZE)
#define dn sp, #(5*NUMSIZE)
#define e sp, #(5*NUMSIZE)

#define dmsn sp, #(6*NUMSIZE)
#define p sp, #(6*NUMSIZE)

#define xm sp, #(7*NUMSIZE)
#define dnsm sp, #(7*NUMSIZE)
#define spro sp, #(7*NUMSIZE)

#define d sp, #(8*NUMSIZE)

#define xn sp, #(9*NUMSIZE)
#define s sp, #(9*NUMSIZE)

// Total size to reserve on the stack

#define NSPACE 10*NUMSIZE

// Macro wrapping up the basic field operation bignum_mul_p25519_alt, only
// trivially different from a pure function call to that subroutine.

#define mul_p25519(P0,P1,P2)                    \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x7, x8, [P2] __LF                  \
        mul     x12, x3, x7 __LF                   \
        umulh   x13, x3, x7 __LF                   \
        mul     x11, x3, x8 __LF                   \
        umulh   x14, x3, x8 __LF                   \
        adds    x13, x13, x11 __LF                 \
        ldp     x9, x10, [P2+16] __LF              \
        mul     x11, x3, x9 __LF                   \
        umulh   x15, x3, x9 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        umulh   x16, x3, x10 __LF                  \
        adcs    x15, x15, x11 __LF                 \
        adc     x16, x16, xzr __LF                 \
        ldp     x5, x6, [P1+16] __LF               \
        mul     x11, x4, x7 __LF                   \
        adds    x13, x13, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        mul     x11, x4, x10 __LF                  \
        adcs    x16, x16, x11 __LF                 \
        umulh   x3, x4, x10 __LF                   \
        adc     x3, x3, xzr __LF                   \
        umulh   x11, x4, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        umulh   x11, x4, x9 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        adc     x3, x3, xzr __LF                   \
        mul     x11, x5, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        mul     x11, x5, x8 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        mul     x11, x5, x9 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x5, x10 __LF                  \
        adcs    x3, x3, x11 __LF                   \
        umulh   x4, x5, x10 __LF                   \
        adc     x4, x4, xzr __LF                   \
        umulh   x11, x5, x7 __LF                   \
        adds    x15, x15, x11 __LF                 \
        umulh   x11, x5, x8 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        umulh   x11, x5, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        adc     x4, x4, xzr __LF                   \
        mul     x11, x6, x7 __LF                   \
        adds    x15, x15, x11 __LF                 \
        mul     x11, x6, x8 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x6, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        mul     x11, x6, x10 __LF                  \
        adcs    x4, x4, x11 __LF                   \
        umulh   x5, x6, x10 __LF                   \
        adc     x5, x5, xzr __LF                   \
        umulh   x11, x6, x7 __LF                   \
        adds    x16, x16, x11 __LF                 \
        umulh   x11, x6, x8 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        umulh   x11, x6, x9 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adc     x5, x5, xzr __LF                   \
        mov     x7, #0x26 __LF                     \
        mul     x11, x7, x16 __LF                  \
        umulh   x9, x7, x16 __LF                   \
        adds    x12, x12, x11 __LF                 \
        mul     x11, x7, x3 __LF                   \
        umulh   x3, x7, x3 __LF                    \
        adcs    x13, x13, x11 __LF                 \
        mul     x11, x7, x4 __LF                   \
        umulh   x4, x7, x4 __LF                    \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x7, x5 __LF                   \
        umulh   x5, x7, x5 __LF                    \
        adcs    x15, x15, x11 __LF                 \
        cset    x16, cs __LF                       \
        adds    x15, x15, x4 __LF                  \
        adc     x16, x16, x5 __LF                  \
        cmn     x15, x15 __LF                      \
        orr     x15, x15, #0x8000000000000000 __LF \
        adc     x8, x16, x16 __LF                  \
        mov     x7, #0x13 __LF                     \
        madd    x11, x7, x8, x7 __LF               \
        adds    x12, x12, x11 __LF                 \
        adcs    x13, x13, x9 __LF                  \
        adcs    x14, x14, x3 __LF                  \
        adcs    x15, x15, xzr __LF                 \
        csel    x7, x7, xzr, cc __LF               \
        subs    x12, x12, x7 __LF                  \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x14, x14, xzr __LF                 \
        sbc     x15, x15, xzr __LF                 \
        and     x15, x15, #0x7fffffffffffffff __LF \
        stp     x12, x13, [P0] __LF                \
        stp     x14, x15, [P0+16]

// A version of multiplication that only guarantees output < 2 * p_25519.
// This basically skips the +1 and final correction in quotient estimation.

#define mul_4(P0,P1,P2)                         \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x7, x8, [P2] __LF                  \
        mul     x12, x3, x7 __LF                   \
        umulh   x13, x3, x7 __LF                   \
        mul     x11, x3, x8 __LF                   \
        umulh   x14, x3, x8 __LF                   \
        adds    x13, x13, x11 __LF                 \
        ldp     x9, x10, [P2+16] __LF              \
        mul     x11, x3, x9 __LF                   \
        umulh   x15, x3, x9 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        umulh   x16, x3, x10 __LF                  \
        adcs    x15, x15, x11 __LF                 \
        adc     x16, x16, xzr __LF                 \
        ldp     x5, x6, [P1+16] __LF               \
        mul     x11, x4, x7 __LF                   \
        adds    x13, x13, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        mul     x11, x4, x10 __LF                  \
        adcs    x16, x16, x11 __LF                 \
        umulh   x3, x4, x10 __LF                   \
        adc     x3, x3, xzr __LF                   \
        umulh   x11, x4, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        umulh   x11, x4, x9 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        adc     x3, x3, xzr __LF                   \
        mul     x11, x5, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        mul     x11, x5, x8 __LF                   \
        adcs    x15, x15, x11 __LF                 \
        mul     x11, x5, x9 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x5, x10 __LF                  \
        adcs    x3, x3, x11 __LF                   \
        umulh   x4, x5, x10 __LF                   \
        adc     x4, x4, xzr __LF                   \
        umulh   x11, x5, x7 __LF                   \
        adds    x15, x15, x11 __LF                 \
        umulh   x11, x5, x8 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        umulh   x11, x5, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        adc     x4, x4, xzr __LF                   \
        mul     x11, x6, x7 __LF                   \
        adds    x15, x15, x11 __LF                 \
        mul     x11, x6, x8 __LF                   \
        adcs    x16, x16, x11 __LF                 \
        mul     x11, x6, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        mul     x11, x6, x10 __LF                  \
        adcs    x4, x4, x11 __LF                   \
        umulh   x5, x6, x10 __LF                   \
        adc     x5, x5, xzr __LF                   \
        umulh   x11, x6, x7 __LF                   \
        adds    x16, x16, x11 __LF                 \
        umulh   x11, x6, x8 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        umulh   x11, x6, x9 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adc     x5, x5, xzr __LF                   \
        mov     x7, #0x26 __LF                     \
        mul     x11, x7, x16 __LF                  \
        umulh   x9, x7, x16 __LF                   \
        adds    x12, x12, x11 __LF                 \
        mul     x11, x7, x3 __LF                   \
        umulh   x3, x7, x3 __LF                    \
        adcs    x13, x13, x11 __LF                 \
        mul     x11, x7, x4 __LF                   \
        umulh   x4, x7, x4 __LF                    \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x7, x5 __LF                   \
        umulh   x5, x7, x5 __LF                    \
        adcs    x15, x15, x11 __LF                 \
        cset    x16, cs __LF                       \
        adds    x15, x15, x4 __LF                  \
        adc     x16, x16, x5 __LF                  \
        cmn     x15, x15 __LF                      \
        bic     x15, x15, #0x8000000000000000 __LF \
        adc     x8, x16, x16 __LF                  \
        mov     x7, #0x13 __LF                     \
        mul     x11, x7, x8 __LF                   \
        adds    x12, x12, x11 __LF                 \
        adcs    x13, x13, x9 __LF                  \
        adcs    x14, x14, x3 __LF                  \
        adc     x15, x15, xzr __LF                 \
        stp     x12, x13, [P0] __LF                \
        stp     x14, x15, [P0+16]

// Squaring just giving a result < 2 * p_25519, which is done by
// basically skipping the +1 in the quotient estimate and the final
// optional correction.

#define sqr_4(P0,P1)                            \
        ldp     x2, x3, [P1] __LF                  \
        mul     x9, x2, x3 __LF                    \
        umulh   x10, x2, x3 __LF                   \
        ldp     x4, x5, [P1+16] __LF               \
        mul     x11, x2, x5 __LF                   \
        umulh   x12, x2, x5 __LF                   \
        mul     x7, x2, x4 __LF                    \
        umulh   x6, x2, x4 __LF                    \
        adds    x10, x10, x7 __LF                  \
        adcs    x11, x11, x6 __LF                  \
        mul     x7, x3, x4 __LF                    \
        umulh   x6, x3, x4 __LF                    \
        adc     x6, x6, xzr __LF                   \
        adds    x11, x11, x7 __LF                  \
        mul     x13, x4, x5 __LF                   \
        umulh   x14, x4, x5 __LF                   \
        adcs    x12, x12, x6 __LF                  \
        mul     x7, x3, x5 __LF                    \
        umulh   x6, x3, x5 __LF                    \
        adc     x6, x6, xzr __LF                   \
        adds    x12, x12, x7 __LF                  \
        adcs    x13, x13, x6 __LF                  \
        adc     x14, x14, xzr __LF                 \
        adds    x9, x9, x9 __LF                    \
        adcs    x10, x10, x10 __LF                 \
        adcs    x11, x11, x11 __LF                 \
        adcs    x12, x12, x12 __LF                 \
        adcs    x13, x13, x13 __LF                 \
        adcs    x14, x14, x14 __LF                 \
        cset    x6, cs __LF                        \
        umulh   x7, x2, x2 __LF                    \
        mul     x8, x2, x2 __LF                    \
        adds    x9, x9, x7 __LF                    \
        mul     x7, x3, x3 __LF                    \
        adcs    x10, x10, x7 __LF                  \
        umulh   x7, x3, x3 __LF                    \
        adcs    x11, x11, x7 __LF                  \
        mul     x7, x4, x4 __LF                    \
        adcs    x12, x12, x7 __LF                  \
        umulh   x7, x4, x4 __LF                    \
        adcs    x13, x13, x7 __LF                  \
        mul     x7, x5, x5 __LF                    \
        adcs    x14, x14, x7 __LF                  \
        umulh   x7, x5, x5 __LF                    \
        adc     x6, x6, x7 __LF                    \
        mov     x3, #0x26 __LF                     \
        mul     x7, x3, x12 __LF                   \
        umulh   x4, x3, x12 __LF                   \
        adds    x8, x8, x7 __LF                    \
        mul     x7, x3, x13 __LF                   \
        umulh   x13, x3, x13 __LF                  \
        adcs    x9, x9, x7 __LF                    \
        mul     x7, x3, x14 __LF                   \
        umulh   x14, x3, x14 __LF                  \
        adcs    x10, x10, x7 __LF                  \
        mul     x7, x3, x6 __LF                    \
        umulh   x6, x3, x6 __LF                    \
        adcs    x11, x11, x7 __LF                  \
        cset    x12, cs __LF                       \
        adds    x11, x11, x14 __LF                 \
        adc     x12, x12, x6 __LF                  \
        cmn     x11, x11 __LF                      \
        bic     x11, x11, #0x8000000000000000 __LF \
        adc     x2, x12, x12 __LF                  \
        mov     x3, #0x13 __LF                     \
        mul     x7, x3, x2 __LF                    \
        adds    x8, x8, x7 __LF                    \
        adcs    x9, x9, x4 __LF                    \
        adcs    x10, x10, x13 __LF                 \
        adc     x11, x11, xzr __LF                 \
        stp     x8, x9, [P0] __LF                  \
        stp     x10, x11, [P0+16]

// Modular addition with double modulus 2 * p_25519 = 2^256 - 38.
// This only ensures that the result fits in 4 digits, not that it is reduced
// even w.r.t. double modulus. The result is always correct modulo provided
// the sum of the inputs is < 2^256 + 2^256 - 38, so in particular provided
// at least one of them is reduced double modulo.

#define add_twice4(P0,P1,P2)                    \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x7, x8, [P2] __LF                  \
        adds    x3, x3, x7 __LF                    \
        adcs    x4, x4, x8 __LF                    \
        ldp     x5, x6, [P1+16] __LF               \
        ldp     x7, x8, [P2+16] __LF               \
        adcs    x5, x5, x7 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        mov     x9, #38 __LF                       \
        csel    x9, x9, xzr, cs __LF               \
        adds    x3, x3, x9 __LF                    \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adc     x6, x6, xzr __LF                   \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16]

// Modular subtraction with double modulus 2 * p_25519 = 2^256 - 38

#define sub_twice4(p0,p1,p2)                    \
        ldp     x5, x6, [p1] __LF                  \
        ldp     x4, x3, [p2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [p1+16] __LF               \
        ldp     x4, x3, [p2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x4, #38 __LF                       \
        csel    x3, x4, xzr, lo __LF               \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbc     x8, x8, xzr __LF                   \
        stp     x5, x6, [p0] __LF                  \
        stp     x7, x8, [p0+16]

// Combined z = c * x + y with reduction only < 2 * p_25519
// where c is initially in the X1 register. It is assumed
// that 19 * (c * x + y) < 2^60 * 2^256 so we don't need a
// high mul in the final part.

#define cmadd_4(p0,p2,p3)                       \
        ldp     x7, x8, [p2] __LF                  \
        ldp     x9, x10, [p2+16] __LF              \
        mul     x3, x1, x7 __LF                    \
        mul     x4, x1, x8 __LF                    \
        mul     x5, x1, x9 __LF                    \
        mul     x6, x1, x10 __LF                   \
        umulh   x7, x1, x7 __LF                    \
        umulh   x8, x1, x8 __LF                    \
        umulh   x9, x1, x9 __LF                    \
        umulh   x10, x1, x10 __LF                  \
        adds    x4, x4, x7 __LF                    \
        adcs    x5, x5, x8 __LF                    \
        adcs    x6, x6, x9 __LF                    \
        adc     x10, x10, xzr __LF                 \
        ldp     x7, x8, [p3] __LF                  \
        adds    x3, x3, x7 __LF                    \
        adcs    x4, x4, x8 __LF                    \
        ldp     x7, x8, [p3+16] __LF               \
        adcs    x5, x5, x7 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        adc     x10, x10, xzr __LF                 \
        cmn     x6, x6 __LF                        \
        bic     x6, x6, #0x8000000000000000 __LF   \
        adc     x8, x10, x10 __LF                  \
        mov     x9, #19 __LF                       \
        mul     x7, x8, x9 __LF                    \
        adds    x3, x3, x7 __LF                    \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adc     x6, x6, xzr __LF                   \
        stp     x3, x4, [p0] __LF                  \
        stp     x5, x6, [p0+16]

// Multiplex: z := if NZ then x else y

#define mux_4(p0,p1,p2)                         \
        ldp     x0, x1, [p1] __LF                  \
        ldp     x2, x3, [p2] __LF                  \
        csel    x0, x0, x2, ne __LF                \
        csel    x1, x1, x3, ne __LF                \
        stp     x0, x1, [p0] __LF                  \
        ldp     x0, x1, [p1+16] __LF               \
        ldp     x2, x3, [p2+16] __LF               \
        csel    x0, x0, x2, ne __LF                \
        csel    x1, x1, x3, ne __LF                \
        stp     x0, x1, [p0+16]

S2N_BN_SYMBOL(curve25519_x25519_byte_alt):
        CFI_START

// Save regs and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_DEC_SP(NSPACE)

// Move the output pointer to a stable place

        mov     res, x0

// Copy the inputs to the local variables with minimal mangling:
//
//  - The scalar is in principle turned into 01xxx...xxx000 but
//    in the structure below the special handling of these bits is
//    explicit in the main computation; the scalar is just copied.
//
//  - The point x coord is reduced mod 2^255 by masking off the
//    top bit. In the main loop we only need reduction < 2 * p_25519.

        ldrb    w10, [x1]
        ldrb    w0, [x1, #1]
        orr     x10, x10, x0, lsl #8
        ldrb    w0, [x1, #2]
        orr     x10, x10, x0, lsl #16
        ldrb    w0, [x1, #3]
        orr     x10, x10, x0, lsl #24
        ldrb    w0, [x1, #4]
        orr     x10, x10, x0, lsl #32
        ldrb    w0, [x1, #5]
        orr     x10, x10, x0, lsl #40
        ldrb    w0, [x1, #6]
        orr     x10, x10, x0, lsl #48
        ldrb    w0, [x1, #7]
        orr     x10, x10, x0, lsl #56
        ldrb    w11, [x1, #8]
        ldrb    w0, [x1, #9]
        orr     x11, x11, x0, lsl #8
        ldrb    w0, [x1, #10]
        orr     x11, x11, x0, lsl #16
        ldrb    w0, [x1, #11]
        orr     x11, x11, x0, lsl #24
        ldrb    w0, [x1, #12]
        orr     x11, x11, x0, lsl #32
        ldrb    w0, [x1, #13]
        orr     x11, x11, x0, lsl #40
        ldrb    w0, [x1, #14]
        orr     x11, x11, x0, lsl #48
        ldrb    w0, [x1, #15]
        orr     x11, x11, x0, lsl #56
        stp     x10, x11, [scalar]

        ldrb    w12, [x1, #16]
        ldrb    w0, [x1, #17]
        orr     x12, x12, x0, lsl #8
        ldrb    w0, [x1, #18]
        orr     x12, x12, x0, lsl #16
        ldrb    w0, [x1, #19]
        orr     x12, x12, x0, lsl #24
        ldrb    w0, [x1, #20]
        orr     x12, x12, x0, lsl #32
        ldrb    w0, [x1, #21]
        orr     x12, x12, x0, lsl #40
        ldrb    w0, [x1, #22]
        orr     x12, x12, x0, lsl #48
        ldrb    w0, [x1, #23]
        orr     x12, x12, x0, lsl #56
        ldrb    w13, [x1, #24]
        ldrb    w0, [x1, #25]
        orr     x13, x13, x0, lsl #8
        ldrb    w0, [x1, #26]
        orr     x13, x13, x0, lsl #16
        ldrb    w0, [x1, #27]
        orr     x13, x13, x0, lsl #24
        ldrb    w0, [x1, #28]
        orr     x13, x13, x0, lsl #32
        ldrb    w0, [x1, #29]
        orr     x13, x13, x0, lsl #40
        ldrb    w0, [x1, #30]
        orr     x13, x13, x0, lsl #48
        ldrb    w0, [x1, #31]
        orr     x13, x13, x0, lsl #56
        stp     x12, x13, [scalar+16]

        ldrb    w10, [x2]
        ldrb    w0, [x2, #1]
        orr     x10, x10, x0, lsl #8
        ldrb    w0, [x2, #2]
        orr     x10, x10, x0, lsl #16
        ldrb    w0, [x2, #3]
        orr     x10, x10, x0, lsl #24
        ldrb    w0, [x2, #4]
        orr     x10, x10, x0, lsl #32
        ldrb    w0, [x2, #5]
        orr     x10, x10, x0, lsl #40
        ldrb    w0, [x2, #6]
        orr     x10, x10, x0, lsl #48
        ldrb    w0, [x2, #7]
        orr     x10, x10, x0, lsl #56
        ldrb    w11, [x2, #8]
        ldrb    w0, [x2, #9]
        orr     x11, x11, x0, lsl #8
        ldrb    w0, [x2, #10]
        orr     x11, x11, x0, lsl #16
        ldrb    w0, [x2, #11]
        orr     x11, x11, x0, lsl #24
        ldrb    w0, [x2, #12]
        orr     x11, x11, x0, lsl #32
        ldrb    w0, [x2, #13]
        orr     x11, x11, x0, lsl #40
        ldrb    w0, [x2, #14]
        orr     x11, x11, x0, lsl #48
        ldrb    w0, [x2, #15]
        orr     x11, x11, x0, lsl #56
        stp     x10, x11, [pointx]

        ldrb    w12, [x2, #16]
        ldrb    w0, [x2, #17]
        orr     x12, x12, x0, lsl #8
        ldrb    w0, [x2, #18]
        orr     x12, x12, x0, lsl #16
        ldrb    w0, [x2, #19]
        orr     x12, x12, x0, lsl #24
        ldrb    w0, [x2, #20]
        orr     x12, x12, x0, lsl #32
        ldrb    w0, [x2, #21]
        orr     x12, x12, x0, lsl #40
        ldrb    w0, [x2, #22]
        orr     x12, x12, x0, lsl #48
        ldrb    w0, [x2, #23]
        orr     x12, x12, x0, lsl #56
        ldrb    w13, [x2, #24]
        ldrb    w0, [x2, #25]
        orr     x13, x13, x0, lsl #8
        ldrb    w0, [x2, #26]
        orr     x13, x13, x0, lsl #16
        ldrb    w0, [x2, #27]
        orr     x13, x13, x0, lsl #24
        ldrb    w0, [x2, #28]
        orr     x13, x13, x0, lsl #32
        ldrb    w0, [x2, #29]
        orr     x13, x13, x0, lsl #40
        ldrb    w0, [x2, #30]
        orr     x13, x13, x0, lsl #48
        ldrb    w0, [x2, #31]
        orr     x13, x13, x0, lsl #56
        and     x13, x13, #0x7fffffffffffffff
        stp     x12, x13, [pointx+16]

// Initialize with explicit doubling in order to handle set bit 254.
// Set swap = 1 and (xm,zm) = (x,1) then double as (xn,zn) = 2 * (x,1).
// We use the fact that the point x coordinate is still in registers.
// Since zm = 1 we could do the doubling with an operation count of
// 2 * S + M instead of 2 * S + 2 * M, but it doesn't seem worth
// the slight complication arising from a different linear combination.

        mov     swap, #1
        stp     x10, x11, [xm]
        stp     x12, x13, [xm+16]
        stp     swap, xzr, [zm]
        stp     xzr, xzr, [zm+16]

        sub_twice4(d,xm,zm)
        add_twice4(s,xm,zm)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        mov     x1, 0xdb42
        orr     x1, x1, 0x10000
        cmadd_4(e,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

// The main loop over unmodified bits from i = 253, ..., i = 3 (inclusive).
// This is a classic Montgomery ladder, with the main coordinates only
// reduced mod 2 * p_25519, some intermediate results even more loosely.

        mov     i, #253

Lcurve25519_x25519_byte_alt_scalarloop:

// sm = xm + zm; sn = xn + zn; dm = xm - zm; dn = xn - zn

        sub_twice4(dm,xm,zm)
        add_twice4(sn,xn,zn)
        sub_twice4(dn,xn,zn)
        add_twice4(sm,xm,zm)

// ADDING: dmsn = dm * sn
// DOUBLING: mux d = xt - zt and s = xt + zt for appropriate choice of (xt,zt)

        mul_4(dmsn,sn,dm)

        lsr     x0, i, #6
        ldr     x2, [sp, x0, lsl #3]    // Exploiting scalar = sp exactly
        lsr     x2, x2, i
        and     x2, x2, #1

        cmp     swap, x2
        mov     swap, x2

        mux_4(d,dm,dn)
        mux_4(s,sm,sn)

// ADDING: dnsm = sm * dn

        mul_4(dnsm,sm,dn)

// DOUBLING: d = (xt - zt)^2

        sqr_4(d,d)

// ADDING: dpro = (dmsn - dnsm)^2, spro = (dmsn + dnsm)^2
// DOUBLING: s = (xt + zt)^2

        sub_twice4(dpro,dmsn,dnsm)
        sqr_4(s,s)
        add_twice4(spro,dmsn,dnsm)
        sqr_4(dpro,dpro)

// DOUBLING: p = 4 * xt * zt = s - d

        sub_twice4(p,s,d)

// ADDING: xm' = (dmsn + dnsm)^2

        sqr_4(xm,spro)

// DOUBLING: e = 121666 * p + d

        mov     x1, 0xdb42
        orr     x1, x1, 0x10000
        cmadd_4(e,p,d)

// DOUBLING: xn' = (xt + zt)^2 * (xt - zt)^2 = s * d

        mul_4(xn,s,d)

// ADDING: zm' = x * (dmsn - dnsm)^2

        mul_4(zm,dpro,pointx)

// DOUBLING: zn' = (4 * xt * zt) * ((xt - zt)^2 + 121666 * (4 * xt * zt))
//               = p * (d + 121666 * p)

        mul_4(zn,p,e)

// Loop down as far as 3 (inclusive)

        sub     i, i, #1
        cmp     i, #3
        bcs     Lcurve25519_x25519_byte_alt_scalarloop

// Multiplex directly into (xn,zn) then do three pure doubling steps;
// this accounts for the implicit zeroing of the three lowest bits
// of the scalar.

        cmp     swap, xzr
        mux_4(xn,xm,xn)
        mux_4(zn,zm,zn)

        sub_twice4(d,xn,zn)
        add_twice4(s,xn,zn)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        mov     x1, 0xdb42
        orr     x1, x1, 0x10000
        cmadd_4(e,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

        sub_twice4(d,xn,zn)
        add_twice4(s,xn,zn)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        mov     x1, 0xdb42
        orr     x1, x1, 0x10000
        cmadd_4(e,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

        sub_twice4(d,xn,zn)
        add_twice4(s,xn,zn)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        mov     x1, 0xdb42
        orr     x1, x1, 0x10000
        cmadd_4(e,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

// The projective result of the scalar multiplication is now (xn,zn).
// Prepare to call the modular inverse function to get zn' = 1/zn

        add     x0, zn
        add     x1, zn

// Inline copy of bignum_inv_p25519, identical except for stripping out
// the prologue and epilogue saving and restoring registers and making
// and reclaiming room on the stack. For more details and explanations see
// "arm/curve25519/bignum_inv_p25519.S". Note that the stack it uses for
// its own temporaries is 128 bytes, so it has no effect on variables
// that are needed in the rest of our computation here: res, xn and zn.

        mov     x20, x0
        mov     x10, #0xffffffffffffffed
        mov     x11, #0xffffffffffffffff
        stp     x10, x11, [sp]
        mov     x12, #0x7fffffffffffffff
        stp     x11, x12, [sp, #16]
        ldp     x2, x3, [x1]
        ldp     x4, x5, [x1, #16]
        mov     x7, #0x13
        lsr     x6, x5, #63
        madd    x6, x7, x6, x7
        adds    x2, x2, x6
        adcs    x3, x3, xzr
        adcs    x4, x4, xzr
        orr     x5, x5, #0x8000000000000000
        adcs    x5, x5, xzr
        csel    x6, x7, xzr, cc
        subs    x2, x2, x6
        sbcs    x3, x3, xzr
        sbcs    x4, x4, xzr
        sbc     x5, x5, xzr
        and     x5, x5, #0x7fffffffffffffff
        stp     x2, x3, [sp, #32]
        stp     x4, x5, [sp, #48]
        stp     xzr, xzr, [sp, #64]
        stp     xzr, xzr, [sp, #80]
        mov     x10, #0x2099
        movk    x10, #0x7502, lsl #16
        movk    x10, #0x9e23, lsl #32
        movk    x10, #0xa0f9, lsl #48
        mov     x11, #0x2595
        movk    x11, #0x1d13, lsl #16
        movk    x11, #0x8f3f, lsl #32
        movk    x11, #0xa8c6, lsl #48
        mov     x12, #0x5242
        movk    x12, #0x5ac, lsl #16
        movk    x12, #0x8938, lsl #32
        movk    x12, #0x6c6c, lsl #48
        mov     x13, #0x615
        movk    x13, #0x4177, lsl #16
        movk    x13, #0x8b2, lsl #32
        movk    x13, #0x2765, lsl #48
        stp     x10, x11, [sp, #96]
        stp     x12, x13, [sp, #112]
        mov     x21, #0xa
        mov     x22, #0x1
        b       Lcurve25519_x25519_byte_alt_invmidloop
Lcurve25519_x25519_byte_alt_invloop:
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        and     x0, x12, x16
        and     x1, x13, x17
        add     x19, x0, x1
        ldr     x7, [sp]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #32]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x7, [sp, #8]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #40]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        adc     x6, x6, x1
        extr    x4, x2, x4, #59
        str     x4, [sp]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        adc     x4, x4, x1
        extr    x5, x3, x5, #59
        str     x5, [sp, #32]
        ldr     x7, [sp, #16]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #48]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        adc     x5, x5, x1
        extr    x2, x6, x2, #59
        str     x2, [sp, #8]
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        adc     x2, x2, x1
        extr    x3, x4, x3, #59
        str     x3, [sp, #40]
        ldr     x7, [sp, #24]
        eor     x1, x7, x14
        asr     x3, x1, #63
        and     x3, x3, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #56]
        eor     x1, x8, x15
        asr     x0, x1, #63
        and     x0, x0, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [sp, #16]
        extr    x5, x3, x5, #59
        str     x5, [sp, #24]
        eor     x1, x7, x16
        asr     x5, x1, #63
        and     x5, x5, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        asr     x0, x1, #63
        and     x0, x0, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x4, x2, x4, #59
        str     x4, [sp, #48]
        extr    x2, x5, x2, #59
        str     x2, [sp, #56]
        ldr     x7, [sp, #64]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #96]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #64]
        adc     x2, x2, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x5, x19, x0
        adc     x3, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x5, x5, x0
        str     x5, [sp, #96]
        adc     x3, x3, x1
        ldr     x7, [sp, #72]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #104]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #72]
        adc     x6, x6, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x3, x3, x0
        str     x3, [sp, #104]
        adc     x4, x4, x1
        ldr     x7, [sp, #80]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #112]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #80]
        adc     x5, x5, x1
        eor     x1, x7, x16
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, x17
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x4, x4, x0
        str     x4, [sp, #112]
        adc     x2, x2, x1
        ldr     x7, [sp, #88]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #120]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x3, x5, #63
        ldp     x0, x1, [sp, #64]
        add     x6, x6, x3, asr #63
        mov     x3, #0x13
        mul     x4, x6, x3
        add     x5, x5, x6, lsl #63
        smulh   x3, x6, x3
        ldr     x6, [sp, #80]
        adds    x0, x0, x4
        adcs    x1, x1, x3
        asr     x3, x3, #63
        adcs    x6, x6, x3
        adc     x5, x5, x3
        stp     x0, x1, [sp, #64]
        stp     x6, x5, [sp, #80]
        eor     x1, x7, x16
        and     x5, x16, x12
        neg     x5, x5
        mul     x0, x1, x12
        umulh   x1, x1, x12
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, x17
        and     x0, x17, x13
        sub     x5, x5, x0
        mul     x0, x1, x13
        umulh   x1, x1, x13
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x6, x5, x2, #63
        ldp     x0, x1, [sp, #96]
        add     x6, x6, x5, asr #63
        mov     x5, #0x13
        mul     x4, x6, x5
        add     x2, x2, x6, lsl #63
        smulh   x5, x6, x5
        ldr     x3, [sp, #112]
        adds    x0, x0, x4
        adcs    x1, x1, x5
        asr     x5, x5, #63
        adcs    x3, x3, x5
        adc     x2, x2, x5
        stp     x0, x1, [sp, #96]
        stp     x3, x2, [sp, #112]
Lcurve25519_x25519_byte_alt_invmidloop:
        mov     x1, x22
        ldr     x2, [sp]
        ldr     x3, [sp, #32]
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x8, x4, #0x100, lsl #12
        sbfx    x8, x8, #21, #21
        mov     x11, #0x100000
        add     x11, x11, x11, lsl #21
        add     x9, x4, x11
        asr     x9, x9, #42
        add     x10, x5, #0x100, lsl #12
        sbfx    x10, x10, #21, #21
        add     x11, x5, x11
        asr     x11, x11, #42
        mul     x6, x8, x2
        mul     x7, x9, x3
        mul     x2, x10, x2
        mul     x3, x11, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #21, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #42
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #21, #21
        add     x15, x5, x15
        asr     x15, x15, #42
        mul     x6, x12, x2
        mul     x7, x13, x3
        mul     x2, x14, x2
        mul     x3, x15, x3
        add     x4, x6, x7
        add     x5, x2, x3
        asr     x2, x4, #20
        asr     x3, x5, #20
        and     x4, x2, #0xfffff
        orr     x4, x4, #0xfffffe0000000000
        and     x5, x3, #0xfffff
        orr     x5, x5, #0xc000000000000000
        tst     x5, #0x1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        mul     x2, x12, x8
        mul     x3, x12, x9
        mul     x6, x14, x8
        mul     x7, x14, x9
        madd    x8, x13, x10, x2
        madd    x9, x13, x11, x3
        madd    x16, x15, x10, x6
        madd    x17, x15, x11, x7
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        tst     x5, #0x2
        asr     x5, x5, #1
        csel    x6, x4, xzr, ne
        ccmp    x1, xzr, #0x8, ne
        cneg    x1, x1, ge
        cneg    x6, x6, ge
        csel    x4, x5, x4, ge
        add     x5, x5, x6
        add     x1, x1, #0x2
        asr     x5, x5, #1
        add     x12, x4, #0x100, lsl #12
        sbfx    x12, x12, #22, #21
        mov     x15, #0x100000
        add     x15, x15, x15, lsl #21
        add     x13, x4, x15
        asr     x13, x13, #43
        add     x14, x5, #0x100, lsl #12
        sbfx    x14, x14, #22, #21
        add     x15, x5, x15
        asr     x15, x15, #43
        mneg    x2, x12, x8
        mneg    x3, x12, x9
        mneg    x4, x14, x8
        mneg    x5, x14, x9
        msub    x10, x13, x16, x2
        msub    x11, x13, x17, x3
        msub    x12, x15, x16, x4
        msub    x13, x15, x17, x5
        mov     x22, x1
        subs    x21, x21, #0x1
        b.ne    Lcurve25519_x25519_byte_alt_invloop
        ldr     x0, [sp]
        ldr     x1, [sp, #32]
        mul     x0, x0, x10
        madd    x1, x1, x11, x0
        asr     x0, x1, #63
        cmp     x10, xzr
        csetm   x14, mi
        cneg    x10, x10, mi
        eor     x14, x14, x0
        cmp     x11, xzr
        csetm   x15, mi
        cneg    x11, x11, mi
        eor     x15, x15, x0
        cmp     x12, xzr
        csetm   x16, mi
        cneg    x12, x12, mi
        eor     x16, x16, x0
        cmp     x13, xzr
        csetm   x17, mi
        cneg    x13, x13, mi
        eor     x17, x17, x0
        and     x0, x10, x14
        and     x1, x11, x15
        add     x9, x0, x1
        ldr     x7, [sp, #64]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x4, x9, x0
        adc     x2, xzr, x1
        ldr     x8, [sp, #96]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x4, x4, x0
        str     x4, [sp, #64]
        adc     x2, x2, x1
        ldr     x7, [sp, #72]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [sp, #104]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x2, x2, x0
        str     x2, [sp, #72]
        adc     x6, x6, x1
        ldr     x7, [sp, #80]
        eor     x1, x7, x14
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [sp, #112]
        eor     x1, x8, x15
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x6, x6, x0
        str     x6, [sp, #80]
        adc     x5, x5, x1
        ldr     x7, [sp, #88]
        eor     x1, x7, x14
        and     x3, x14, x10
        neg     x3, x3
        mul     x0, x1, x10
        umulh   x1, x1, x10
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [sp, #120]
        eor     x1, x8, x15
        and     x0, x15, x11
        sub     x3, x3, x0
        mul     x0, x1, x11
        umulh   x1, x1, x11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x3, x5, #63
        ldp     x0, x1, [sp, #64]
        tst     x3, x3
        cinc    x6, x6, pl
        mov     x3, #0x13
        mul     x4, x6, x3
        add     x5, x5, x6, lsl #63
        smulh   x6, x6, x3
        ldr     x2, [sp, #80]
        adds    x0, x0, x4
        adcs    x1, x1, x6
        asr     x6, x6, #63
        adcs    x2, x2, x6
        adcs    x5, x5, x6
        csel    x3, x3, xzr, mi
        subs    x0, x0, x3
        sbcs    x1, x1, xzr
        sbcs    x2, x2, xzr
        sbc     x5, x5, xzr
        and     x5, x5, #0x7fffffffffffffff
        mov     x4, x20
        stp     x0, x1, [x4]
        stp     x2, x5, [x4, #16]

// Now the result is xn * (1/zn), fully reduced modulo p.
// Note that in the degenerate case zn = 0 (mod p_25519), the
// modular inverse code above will produce 1/zn = 0, giving
// the correct overall X25519 result of zero for the point at
// infinity.

        mul_p25519(zn,xn,zn)

        ldp     x10, x11, [zn]
        strb    w10, [resx]
        lsr     x10, x10, #8
        strb    w10, [resx+1]
        lsr     x10, x10, #8
        strb    w10, [resx+2]
        lsr     x10, x10, #8
        strb    w10, [resx+3]
        lsr     x10, x10, #8
        strb    w10, [resx+4]
        lsr     x10, x10, #8
        strb    w10, [resx+5]
        lsr     x10, x10, #8
        strb    w10, [resx+6]
        lsr     x10, x10, #8
        strb    w10, [resx+7]

        strb    w11, [resx+8]
        lsr     x11, x11, #8
        strb    w11, [resx+9]
        lsr     x11, x11, #8
        strb    w11, [resx+10]
        lsr     x11, x11, #8
        strb    w11, [resx+11]
        lsr     x11, x11, #8
        strb    w11, [resx+12]
        lsr     x11, x11, #8
        strb    w11, [resx+13]
        lsr     x11, x11, #8
        strb    w11, [resx+14]
        lsr     x11, x11, #8
        strb    w11, [resx+15]

        ldp     x12, x13, [zn+16]
        strb    w12, [resx+16]
        lsr     x12, x12, #8
        strb    w12, [resx+17]
        lsr     x12, x12, #8
        strb    w12, [resx+18]
        lsr     x12, x12, #8
        strb    w12, [resx+19]
        lsr     x12, x12, #8
        strb    w12, [resx+20]
        lsr     x12, x12, #8
        strb    w12, [resx+21]
        lsr     x12, x12, #8
        strb    w12, [resx+22]
        lsr     x12, x12, #8
        strb    w12, [resx+23]

        strb    w13, [resx+24]
        lsr     x13, x13, #8
        strb    w13, [resx+25]
        lsr     x13, x13, #8
        strb    w13, [resx+26]
        lsr     x13, x13, #8
        strb    w13, [resx+27]
        lsr     x13, x13, #8
        strb    w13, [resx+28]
        lsr     x13, x13, #8
        strb    w13, [resx+29]
        lsr     x13, x13, #8
        strb    w13, [resx+30]
        lsr     x13, x13, #8
        strb    w13, [resx+31]

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(curve25519_x25519_byte_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
