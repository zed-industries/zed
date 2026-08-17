// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply-add modulo the order of the curve25519/edwards25519 basepoint
// Inputs x[4], y[4], c[4]; output z[4]
//
//    extern void bignum_madd_n25519_alt(uint64_t z[static 4],
//                                       const uint64_t x[static 4],
//                                       const uint64_t y[static 4],
//                                       const uint64_t c[static 4]);
//
// Performs z := (x * y + c) mod n_25519, where the modulus is
// n_25519 = 2^252 + 27742317777372353535851937790883648493, the
// order of the curve25519/edwards25519 basepoint. The result z
// and the inputs x, y and c are all 4 digits (256 bits).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y, X3 = c
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_madd_n25519_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_madd_n25519_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_madd_n25519_alt)
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

S2N_BN_SYMBOL(bignum_madd_n25519_alt):
        CFI_START

        CFI_PUSH2(x19,x20)

// Back up the result pointer so we can overwrite x0 in intermediate steps

        mov     z, x0

// First compute [x15;x14;x13;x12;x11;x10;x9;x8] = x * y + c. This
// is a basic schoolbook multiplier similar to the start of
// bignum_mul_p25519_alt except for different registers, but it
// also adds in the c term after the first row accumulation.

        ldp     x13, x14, [x1]
        ldp     x7, x0, [x2]
        mul     x8, x13, x7
        umulh   x9, x13, x7
        mul     x16, x13, x0
        umulh   x10, x13, x0
        adds    x9, x9, x16
        ldp     x4, x5, [x2, #16]
        mul     x16, x13, x4
        umulh   x11, x13, x4
        adcs    x10, x10, x16
        mul     x16, x13, x5
        umulh   x12, x13, x5
        adcs    x11, x11, x16
        adc     x12, x12, xzr
        ldp     x15, x6, [x3]
        adds    x8, x8, x15
        adcs    x9, x9, x6
        ldp     x15, x6, [x3, #16]
        adcs    x10, x10, x15
        adcs    x11, x11, x6
        adc     x12, x12, xzr
        ldp     x15, x6, [x1, #16]
        mul     x16, x14, x7
        adds    x9, x9, x16
        mul     x16, x14, x0
        adcs    x10, x10, x16
        mul     x16, x14, x4
        adcs    x11, x11, x16
        mul     x16, x14, x5
        adcs    x12, x12, x16
        umulh   x13, x14, x5
        adc     x13, x13, xzr
        umulh   x16, x14, x7
        adds    x10, x10, x16
        umulh   x16, x14, x0
        adcs    x11, x11, x16
        umulh   x16, x14, x4
        adcs    x12, x12, x16
        adc     x13, x13, xzr
        mul     x16, x15, x7
        adds    x10, x10, x16
        mul     x16, x15, x0
        adcs    x11, x11, x16
        mul     x16, x15, x4
        adcs    x12, x12, x16
        mul     x16, x15, x5
        adcs    x13, x13, x16
        umulh   x14, x15, x5
        adc     x14, x14, xzr
        umulh   x16, x15, x7
        adds    x11, x11, x16
        umulh   x16, x15, x0
        adcs    x12, x12, x16
        umulh   x16, x15, x4
        adcs    x13, x13, x16
        adc     x14, x14, xzr
        mul     x16, x6, x7
        adds    x11, x11, x16
        mul     x16, x6, x0
        adcs    x12, x12, x16
        mul     x16, x6, x4
        adcs    x13, x13, x16
        mul     x16, x6, x5
        adcs    x14, x14, x16
        umulh   x15, x6, x5
        adc     x15, x15, xzr
        umulh   x16, x6, x7
        adds    x12, x12, x16
        umulh   x16, x6, x0
        adcs    x13, x13, x16
        umulh   x16, x6, x4
        adcs    x14, x14, x16
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

S2N_BN_SIZE_DIRECTIVE(bignum_madd_n25519_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
