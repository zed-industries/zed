// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply-add modulo the order of the curve25519/edwards25519 basepoint
// Inputs x[4], y[4], c[4]; output z[4]
//
//    extern void bignum_madd_n25519(uint64_t z[static 4], const uint64_t x[static 4],
//                                   const uint64_t y[static 4],
//                                   const uint64_t c[static 4]);
//
// Performs z := (x * y + c) mod n_25519, where the modulus is
// n_25519 = 2^252 + 27742317777372353535851937790883648493, the
// order of the curve25519/edwards25519 basepoint. The result z
// and the inputs x, y and c are all 4 digits (256 bits).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y, X3 = c
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_madd_n25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_madd_n25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_madd_n25519)
        .text
        .balign 4

// Backup of the input pointer so we can modify x0

#define z x19

// Temporaries for reduction phase

#define q   x2
#define n0  x3
#define n1  x4
#define t0  x5
#define t1  x6
#define t2  x7

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

// Single round of modular reduction mod_n25519, mapping
// [m4;m3;m2;m1;m0] = m to [m3;m2;m1;m0] = m mod n_25519,
// *assuming* the input m < 2^64 * n_25519. This is very
// close to the loop body of the bignum_mod_n25519 function.

#define reduce(m4,m3,m2,m1,m0)                          \
        extr    q, m4, m3, #60 __LF                        \
        and     m3, m3, #0x0FFFFFFFFFFFFFFF __LF           \
        sub     q, q, m4, lsr #60 __LF                     \
        and     t0, m4, #0xF000000000000000 __LF           \
        add     m3, m3, t0 __LF                            \
        mul     t0, n0, q __LF                             \
        mul     t1, n1, q __LF                             \
        umulh   t2, n0, q __LF                             \
        adds    t1, t1, t2 __LF                            \
        umulh   t2, n1, q __LF                             \
        adc     t2, t2, xzr __LF                           \
        subs    m0, m0, t0 __LF                            \
        sbcs    m1, m1, t1 __LF                            \
        sbcs    m2, m2, t2 __LF                            \
        sbcs    m3, m3, xzr __LF                           \
        csel    t0, n0, xzr, cc __LF                       \
        csel    t1, n1, xzr, cc __LF                       \
        adds    m0, m0, t0 __LF                            \
        and     t2, t0, #0x1000000000000000 __LF           \
        adcs    m1, m1, t1 __LF                            \
        adcs    m2, m2, xzr __LF                           \
        adc     m3, m3, t2

// Special case of "reduce" with m4 = 0. As well as not using m4,
// the quotient selection is slightly simpler, just floor(m/2^252)
// versus min (floor(m/2^252)) (2^63-1).

#define reduce0(m3,m2,m1,m0)                            \
        lsr     q, m3, #60 __LF                            \
        and     m3, m3, #0x0FFFFFFFFFFFFFFF __LF           \
        mul     t0, n0, q __LF                             \
        mul     t1, n1, q __LF                             \
        umulh   t2, n0, q __LF                             \
        adds    t1, t1, t2 __LF                            \
        umulh   t2, n1, q __LF                             \
        adc     t2, t2, xzr __LF                           \
        subs    m0, m0, t0 __LF                            \
        sbcs    m1, m1, t1 __LF                            \
        sbcs    m2, m2, t2 __LF                            \
        sbcs    m3, m3, xzr __LF                           \
        csel    t0, n0, xzr, cc __LF                       \
        csel    t1, n1, xzr, cc __LF                       \
        adds    m0, m0, t0 __LF                            \
        and     t2, t0, #0x1000000000000000 __LF           \
        adcs    m1, m1, t1 __LF                            \
        adcs    m2, m2, xzr __LF                           \
        adc     m3, m3, t2

S2N_BN_SYMBOL(bignum_madd_n25519):
        CFI_START

        CFI_PUSH2(x19,x20)

// Back up the result pointer so we can overwrite x0 in intermediate steps

        mov     z, x0

// First compute [x15;x14;x13;x12;x11;x10;x9;x8] = x * y. This is
// a basic 2-level Karatsuba multiplier, similar to the start of
// bignum_mul_p25519, but with changes to the register allocation,
// which in particular preserve x3/w3 for the next step.

        ldp     x0, x4, [x1]
        ldp     x5, x6, [x2]
        umull   x8, w0, w5
        lsr     x17, x0, #32
        umull   x7, w17, w5
        lsr     x16, x5, #32
        umull   x9, w16, w17
        umull   x16, w0, w16
        adds    x8, x8, x7, lsl #32
        lsr     x7, x7, #32
        adc     x9, x9, x7
        adds    x8, x8, x16, lsl #32
        lsr     x16, x16, #32
        adc     x9, x9, x16
        mul     x10, x4, x6
        umulh   x11, x4, x6
        subs    x4, x4, x0
        cneg    x4, x4, cc
        csetm   x16, cc
        adds    x10, x10, x9
        adc     x11, x11, xzr
        subs    x0, x5, x6
        cneg    x0, x0, cc
        cinv    x16, x16, cc
        mul     x7, x4, x0
        umulh   x0, x4, x0
        adds    x9, x8, x10
        adcs    x10, x10, x11
        adc     x11, x11, xzr
        cmn     x16, #0x1
        eor     x7, x7, x16
        adcs    x9, x7, x9
        eor     x0, x0, x16
        adcs    x10, x0, x10
        adc     x11, x11, x16
        ldp     x0, x4, [x1, #16]
        ldp     x5, x6, [x2, #16]
        umull   x12, w0, w5
        lsr     x17, x0, #32
        umull   x7, w17, w5
        lsr     x16, x5, #32
        umull   x13, w16, w17
        umull   x16, w0, w16
        adds    x12, x12, x7, lsl #32
        lsr     x7, x7, #32
        adc     x13, x13, x7
        adds    x12, x12, x16, lsl #32
        lsr     x16, x16, #32
        adc     x13, x13, x16
        mul     x14, x4, x6
        umulh   x15, x4, x6
        subs    x4, x4, x0
        cneg    x4, x4, cc
        csetm   x16, cc
        adds    x14, x14, x13
        adc     x15, x15, xzr
        subs    x0, x5, x6
        cneg    x0, x0, cc
        cinv    x16, x16, cc
        mul     x7, x4, x0
        umulh   x0, x4, x0
        adds    x13, x12, x14
        adcs    x14, x14, x15
        adc     x15, x15, xzr
        cmn     x16, #0x1
        eor     x7, x7, x16
        adcs    x13, x7, x13
        eor     x0, x0, x16
        adcs    x14, x0, x14
        adc     x15, x15, x16
        ldp     x0, x4, [x1, #16]
        ldp     x7, x16, [x1]
        subs    x0, x0, x7
        sbcs    x4, x4, x16
        csetm   x16, cc
        ldp     x7, x17, [x2]
        subs    x5, x7, x5
        sbcs    x6, x17, x6
        csetm   x17, cc
        eor     x0, x0, x16
        subs    x0, x0, x16
        eor     x4, x4, x16
        sbc     x4, x4, x16
        eor     x5, x5, x17
        subs    x5, x5, x17
        eor     x6, x6, x17
        sbc     x6, x6, x17
        eor     x16, x17, x16
        adds    x12, x12, x10
        adcs    x13, x13, x11
        adcs    x14, x14, xzr
        adc     x15, x15, xzr
        mul     x2, x0, x5
        umulh   x17, x0, x5
        mul     x7, x4, x6
        umulh   x1, x4, x6
        subs    x4, x4, x0
        cneg    x4, x4, cc
        csetm   x10, cc
        adds    x7, x7, x17
        adc     x1, x1, xzr
        subs    x6, x5, x6
        cneg    x6, x6, cc
        cinv    x10, x10, cc
        mul     x5, x4, x6
        umulh   x6, x4, x6
        adds    x17, x2, x7
        adcs    x7, x7, x1
        adc     x1, x1, xzr
        cmn     x10, #0x1
        eor     x5, x5, x10
        adcs    x17, x5, x17
        eor     x6, x6, x10
        adcs    x7, x6, x7
        adc     x1, x1, x10
        adds    x10, x12, x8
        adcs    x11, x13, x9
        adcs    x12, x14, x12
        adcs    x13, x15, x13
        adcs    x14, x14, xzr
        adc     x15, x15, xzr
        cmn     x16, #0x1
        eor     x2, x2, x16
        adcs    x10, x2, x10
        eor     x17, x17, x16
        adcs    x11, x17, x11
        eor     x7, x7, x16
        adcs    x12, x7, x12
        eor     x1, x1, x16
        adcs    x13, x1, x13
        adcs    x14, x14, x16
        adc     x15, x15, x16

// Add the constant term, so [x15;x14;x13;x12;x11;x10;x9;x8] = x * y + c
// It's easier to just do this now versus incorporating it into the
// Karatsuba steps above or deferring it until partway through the
// reduction, though it does result in a long carry propagation here.

        ldp     x0, x1, [x3]
        adds    x8, x8, x0
        adcs    x9, x9, x1
        ldp     x0, x1, [x3, #16]
        adcs    x10, x10, x0
        adcs    x11, x11, x1
        adcs    x12, x12, xzr
        adcs    x13, x13, xzr
        adcs    x14, x14, xzr
        adc     x15, x15, xzr

// Now do the modular reduction and write back

        movbig( n0, #0x5812, #0x631a, #0x5cf5, #0xd3ed)
        movbig( n1, #0x14de, #0xf9de, #0xa2f7, #0x9cd6)

        reduce0(x15,x14,x13,x12)
        reduce(x15,x14,x13,x12,x11)
        reduce(x14,x13,x12,x11,x10)
        reduce(x13,x12,x11,x10,x9)
        reduce(x12,x11,x10,x9,x8)

        stp     x8, x9, [z]
        stp     x10, x11, [z, #16]

// Restore registers and return

        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_madd_n25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
