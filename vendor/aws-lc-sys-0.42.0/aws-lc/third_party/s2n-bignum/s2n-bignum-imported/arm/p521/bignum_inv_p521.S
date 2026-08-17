// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Modular inverse modulo p_521 =  2^521 - 1
// Input x[9]; output z[9]
//
// extern void bignum_inv_p521(uint64_t z[static 9],const uint64_t x[static 9]);
//
// Assuming the 9-digit input x is coprime to p_521, i.e. is not divisible
// by it, returns z < p_521 such that x * z == 1 (mod p_521). Note that
// x does not need to be reduced modulo p_521, but the output always is.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_inv_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_inv_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_inv_p521)

        .text
        .balign 4

// Size in bytes of a 64-bit word

#define N 8

// Used for the return pointer

#define res x20

// Loop counter and d = 2 * delta value for divstep

#define i x21
#define d x22

// Registers used for matrix element magnitudes and signs

#define m00 x10
#define m01 x11
#define m10 x12
#define m11 x13
#define s00 x14
#define s01 x15
#define s10 x16
#define s11 x17

// Initial carries for combinations

#define car0 x9
#define car1 x19

// Input and output, plain registers treated according to pattern

#define reg0 x0, #0
#define reg1 x1, #0
#define reg2 x2, #0
#define reg3 x3, #0
#define reg4 x4, #0

#define x x1, #0
#define z x0, #0

// Pointer-offset pairs for temporaries on stack

#define f sp, #0
#define g sp, #(9*N)
#define u sp, #(18*N)
#define v sp, #(27*N)

// Total size to reserve on the stack

#define NSPACE 36*N

// Very similar to a subroutine call to the s2n-bignum word_divstep59.
// But different in register usage and returning the final matrix in
// registers as follows
//
// [ m00  m01]
// [ m10  m11]

#define divstep59()                                                     \
        and     x4, x2, #0xfffff __LF                                      \
        orr     x4, x4, #0xfffffe0000000000 __LF                           \
        and     x5, x3, #0xfffff __LF                                      \
        orr     x5, x5, #0xc000000000000000 __LF                           \
        tst     x5, #0x1 __LF                                              \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        asr     x5, x5, #1 __LF                                            \
        add     x8, x4, #0x100, lsl #12 __LF                               \
        sbfx    x8, x8, #21, #21 __LF                                      \
        mov     x11, #0x100000 __LF                                        \
        add     x11, x11, x11, lsl #21 __LF                                \
        add     x9, x4, x11 __LF                                           \
        asr     x9, x9, #42 __LF                                           \
        add     x10, x5, #0x100, lsl #12 __LF                              \
        sbfx    x10, x10, #21, #21 __LF                                    \
        add     x11, x5, x11 __LF                                          \
        asr     x11, x11, #42 __LF                                         \
        mul     x6, x8, x2 __LF                                            \
        mul     x7, x9, x3 __LF                                            \
        mul     x2, x10, x2 __LF                                           \
        mul     x3, x11, x3 __LF                                           \
        add     x4, x6, x7 __LF                                            \
        add     x5, x2, x3 __LF                                            \
        asr     x2, x4, #20 __LF                                           \
        asr     x3, x5, #20 __LF                                           \
        and     x4, x2, #0xfffff __LF                                      \
        orr     x4, x4, #0xfffffe0000000000 __LF                           \
        and     x5, x3, #0xfffff __LF                                      \
        orr     x5, x5, #0xc000000000000000 __LF                           \
        tst     x5, #0x1 __LF                                              \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        asr     x5, x5, #1 __LF                                            \
        add     x12, x4, #0x100, lsl #12 __LF                              \
        sbfx    x12, x12, #21, #21 __LF                                    \
        mov     x15, #0x100000 __LF                                        \
        add     x15, x15, x15, lsl #21 __LF                                \
        add     x13, x4, x15 __LF                                          \
        asr     x13, x13, #42 __LF                                         \
        add     x14, x5, #0x100, lsl #12 __LF                              \
        sbfx    x14, x14, #21, #21 __LF                                    \
        add     x15, x5, x15 __LF                                          \
        asr     x15, x15, #42 __LF                                         \
        mul     x6, x12, x2 __LF                                           \
        mul     x7, x13, x3 __LF                                           \
        mul     x2, x14, x2 __LF                                           \
        mul     x3, x15, x3 __LF                                           \
        add     x4, x6, x7 __LF                                            \
        add     x5, x2, x3 __LF                                            \
        asr     x2, x4, #20 __LF                                           \
        asr     x3, x5, #20 __LF                                           \
        and     x4, x2, #0xfffff __LF                                      \
        orr     x4, x4, #0xfffffe0000000000 __LF                           \
        and     x5, x3, #0xfffff __LF                                      \
        orr     x5, x5, #0xc000000000000000 __LF                           \
        tst     x5, #0x1 __LF                                              \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        mul     x2, x12, x8 __LF                                           \
        mul     x3, x12, x9 __LF                                           \
        mul     x6, x14, x8 __LF                                           \
        mul     x7, x14, x9 __LF                                           \
        madd    x8, x13, x10, x2 __LF                                      \
        madd    x9, x13, x11, x3 __LF                                      \
        madd    x16, x15, x10, x6 __LF                                     \
        madd    x17, x15, x11, x7 __LF                                     \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        tst     x5, #0x2 __LF                                              \
        asr     x5, x5, #1 __LF                                            \
        csel    x6, x4, xzr, ne __LF                                       \
        ccmp    x1, xzr, #0x8, ne __LF                                     \
        cneg    x1, x1, ge __LF                                            \
        cneg    x6, x6, ge __LF                                            \
        csel    x4, x5, x4, ge __LF                                        \
        add     x5, x5, x6 __LF                                            \
        add     x1, x1, #0x2 __LF                                          \
        asr     x5, x5, #1 __LF                                            \
        add     x12, x4, #0x100, lsl #12 __LF                              \
        sbfx    x12, x12, #22, #21 __LF                                    \
        mov     x15, #0x100000 __LF                                        \
        add     x15, x15, x15, lsl #21 __LF                                \
        add     x13, x4, x15 __LF                                          \
        asr     x13, x13, #43 __LF                                         \
        add     x14, x5, #0x100, lsl #12 __LF                              \
        sbfx    x14, x14, #22, #21 __LF                                    \
        add     x15, x5, x15 __LF                                          \
        asr     x15, x15, #43 __LF                                         \
        mneg    x2, x12, x8 __LF                                           \
        mneg    x3, x12, x9 __LF                                           \
        mneg    x4, x14, x8 __LF                                           \
        mneg    x5, x14, x9 __LF                                           \
        msub    m00, x13, x16, x2 __LF                                     \
        msub    m01, x13, x17, x3 __LF                                     \
        msub    m10, x15, x16, x4 __LF                                     \
        msub    m11, x15, x17, x5

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(bignum_inv_p521):
        CFI_START

// Save registers and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_DEC_SP(NSPACE)

// Save the return pointer for the end so we can overwrite x0 later

        mov     res, x0

// Copy the prime p_521 = 2^521 - 1 into the f variable

        mov     x10, #0xFFFFFFFFFFFFFFFF
        stp     x10, x10, [f]
        stp     x10, x10, [f+16]
        stp     x10, x10, [f+32]
        stp     x10, x10, [f+48]
        mov     x11, #0x1FF
        str     x11, [f+64]

// Copy the input into the g variable, but reduce it strictly mod p_521
// so that g <= f as assumed in the bound proof. This code fragment is
// very similar to bignum_mod_p521_9 complete with carry condensation.

        ldr     x8, [x1, #64]
        lsr     x9, x8, #9

        subs    xzr, xzr, xzr
        ldp     x10, x11, [x1]
        adcs    xzr, x10, x9
        adcs    xzr, x11, xzr
        ldp     x12, x13, [x1, #16]
        and     x7, x12, x13
        adcs    xzr, x7, xzr
        ldp     x14, x15, [x1, #32]
        and     x7, x14, x15
        adcs    xzr, x7, xzr
        ldp     x16, x17, [x1, #48]
        and     x7, x16, x17
        adcs    xzr, x7, xzr
        orr     x7, x8, #~0x1FF
        adcs    x7, x7, xzr

        adcs    x10, x10, x9
        adcs    x11, x11, xzr
        adcs    x12, x12, xzr
        adcs    x13, x13, xzr
        adcs    x14, x14, xzr
        adcs    x15, x15, xzr
        adcs    x16, x16, xzr
        adcs    x17, x17, xzr
        adc     x8, x8, xzr
        and     x8, x8, #0x1FF

        stp     x10, x11, [g]
        stp     x12, x13, [g+16]
        stp     x14, x15, [g+32]
        stp     x16, x17, [g+48]
        str     x8, [g+64]

// Also maintain weakly reduced < 2*p_521 vector [u,v] such that
// [f,g] == x * 2^{1239-59*i} * [u,v] (mod p_521)
// starting with [p_521,x] == x * 2^{1239-59*0} * [0,2^-1239] (mod p_521)
// Note that because (2^{a+521} == 2^a) (mod p_521) we simply have
// (2^-1239 == 2^324) (mod p_521) so the constant initializer is simple.
//
// Based on the standard divstep bound, for inputs <= 2^b we need at least
// n >= (9437 * b + 1) / 4096. Since b is 521, that means 1201 iterations.
// Since we package divstep in multiples of 59 bits, we do 21 blocks of 59
// making *1239* total. (With a bit more effort we could avoid the full 59
// divsteps and use a shorter tail computation, but we keep it simple.)
// Hence, after the 21st iteration we have [f,g] == x * [u,v] and since
// |f| = 1 we get the modular inverse from u by flipping its sign with f.

        stp     xzr, xzr, [u]
        stp     xzr, xzr, [u+16]
        stp     xzr, xzr, [u+32]
        stp     xzr, xzr, [u+48]
        str     xzr, [u+64]

        mov     x10, #16
        stp     xzr, xzr, [v]
        stp     xzr, xzr, [v+16]
        stp     xzr, x10, [v+32]
        stp     xzr, xzr, [v+48]
        str     xzr, [v+64]

// Start of main loop. We jump into the middle so that the divstep
// portion is common to the special 21st iteration after a uniform
// first 20.

        mov     i, #21
        mov     d, #1
        b       Lbignum_inv_p521_midloop

Lbignum_inv_p521_loop:

// Separate the matrix elements into sign-magnitude pairs

        cmp     m00, xzr
        csetm   s00, mi
        cneg    m00, m00, mi

        cmp     m01, xzr
        csetm   s01, mi
        cneg    m01, m01, mi

        cmp     m10, xzr
        csetm   s10, mi
        cneg    m10, m10, mi

        cmp     m11, xzr
        csetm   s11, mi
        cneg    m11, m11, mi

// Adjust the initial values to allow for complement instead of negation
// This initial offset is the same for [f,g] and [u,v] compositions.
// Save it in stable registers for the [u,v] part and do [f,g] first.

        and     x0, m00, s00
        and     x1, m01, s01
        add     car0, x0, x1

        and     x0, m10, s10
        and     x1, m11, s11
        add     car1, x0, x1

// Now the computation of the updated f and g values. This maintains a
// 2-word carry between stages so we can conveniently insert the shift
// right by 59 before storing back, and not overwrite digits we need
// again of the old f and g values.
//
// Digit 0 of [f,g]

        ldr     x7, [f]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x4, car0, x0
        adc     x2, xzr, x1
        ldr     x8, [g]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x4, x4, x0
        adc     x2, x2, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x5, car1, x0
        adc     x3, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x5, x5, x0
        adc     x3, x3, x1

// Digit 1 of [f,g]

        ldr     x7, [f+N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [g+N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x2, x2, x0
        adc     x6, x6, x1
        extr    x4, x2, x4, #59
        str     x4, [f]

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x3, x3, x0
        adc     x4, x4, x1
        extr    x5, x3, x5, #59
        str     x5, [g]

// Digit 2 of [f,g]

        ldr     x7, [f+2*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [g+2*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x6, x6, x0
        adc     x5, x5, x1
        extr    x2, x6, x2, #59
        str     x2, [f+N]

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x4, x4, x0
        adc     x2, x2, x1
        extr    x3, x4, x3, #59
        str     x3, [g+N]

// Digit 3 of [f,g]

        ldr     x7, [f+3*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, xzr, x1
        ldr     x8, [g+3*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [f+2*N]

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x2, x2, x0
        adc     x6, x6, x1
        extr    x4, x2, x4, #59
        str     x4, [g+2*N]

// Digit 4 of [f,g]

        ldr     x7, [f+4*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x3, x3, x0
        adc     x4, xzr, x1
        ldr     x8, [g+4*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x3, x3, x0
        adc     x4, x4, x1
        extr    x5, x3, x5, #59
        str     x5, [f+3*N]

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x6, x6, x0
        adc     x5, x5, x1
        extr    x2, x6, x2, #59
        str     x2, [g+3*N]

// Digit 5 of [f,g]

        ldr     x7, [f+5*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x4, x4, x0
        adc     x2, xzr, x1
        ldr     x8, [g+5*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x4, x4, x0
        adc     x2, x2, x1
        extr    x3, x4, x3, #59
        str     x3, [f+4*N]

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x5, x5, x0
        adc     x3, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [g+4*N]

// Digit 6 of [f,g]

        ldr     x7, [f+6*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [g+6*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x2, x2, x0
        adc     x6, x6, x1
        extr    x4, x2, x4, #59
        str     x4, [f+5*N]

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x3, x3, x0
        adc     x4, x4, x1
        extr    x5, x3, x5, #59
        str     x5, [g+5*N]

// Digit 7 of [f,g]

        ldr     x7, [f+7*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [g+7*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x6, x6, x0
        adc     x5, x5, x1
        extr    x2, x6, x2, #59
        str     x2, [f+6*N]

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x4, x4, x0
        adc     x2, x2, x1
        extr    x3, x4, x3, #59
        str     x3, [g+6*N]

// Digits 8 and 9 of [f,g]

        ldr     x7, [f+8*N]
        eor     x1, x7, s00
        asr     x3, x1, #63
        and     x3, x3, m00
        neg     x3, x3
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [g+8*N]
        eor     x1, x8, s01
        asr     x0, x1, #63
        and     x0, x0, m01
        sub     x3, x3, x0
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [f+7*N]
        extr    x5, x3, x5, #59
        str     x5, [f+8*N]

        eor     x1, x7, s10
        asr     x5, x1, #63
        and     x5, x5, m10
        neg     x5, x5
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, s11
        asr     x0, x1, #63
        and     x0, x0, m11
        sub     x5, x5, x0
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x4, x2, x4, #59
        str     x4, [g+7*N]
        extr    x2, x5, x2, #59
        str     x2, [g+8*N]

// Now the computation of the updated u and v values and their
// modular reductions. A very similar accumulation except that
// the top words of u and v are unsigned and we don't shift.
//
// Digit 0 of [u,v]

        ldr     x7, [u]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x4, car0, x0
        adc     x2, xzr, x1
        ldr     x8, [v]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x4, x4, x0
        str     x4, [u]
        adc     x2, x2, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x5, car1, x0
        adc     x3, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x5, x5, x0
        str     x5, [v]
        adc     x3, x3, x1

// Digit 1 of [u,v]

        ldr     x7, [u+N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [v+N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x2, x2, x0
        str     x2, [u+N]
        adc     x6, x6, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x3, x3, x0
        str     x3, [v+N]
        adc     x4, x4, x1

// Digit 2 of [u,v]

        ldr     x7, [u+2*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [v+2*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x6, x6, x0
        str     x6, [u+2*N]
        adc     x5, x5, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x4, x4, x0
        str     x4, [v+2*N]
        adc     x2, x2, x1

// Digit 3 of [u,v]

        ldr     x7, [u+3*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, xzr, x1
        ldr     x8, [v+3*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        str     x5, [u+3*N]
        adc     x3, x3, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x2, x2, x0
        adc     x6, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x2, x2, x0
        str     x2, [v+3*N]
        adc     x6, x6, x1

// Digit 4 of [u,v]

        ldr     x7, [u+4*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x3, x3, x0
        adc     x4, xzr, x1
        ldr     x8, [v+4*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x3, x3, x0
        str     x3, [u+4*N]
        adc     x4, x4, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x6, x6, x0
        adc     x5, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x6, x6, x0
        str     x6, [v+4*N]
        adc     x5, x5, x1

// Digit 5 of [u,v]

        ldr     x7, [u+5*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x4, x4, x0
        adc     x2, xzr, x1
        ldr     x8, [v+5*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x4, x4, x0
        str     x4, [u+5*N]
        adc     x2, x2, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x5, x5, x0
        adc     x3, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x5, x5, x0
        str     x5, [v+5*N]
        adc     x3, x3, x1

// Digit 6 of [u,v]

        ldr     x7, [u+6*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [v+6*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x2, x2, x0
        str     x2, [u+6*N]
        adc     x6, x6, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x3, x3, x0
        adc     x4, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x3, x3, x0
        str     x3, [v+6*N]
        adc     x4, x4, x1

// Digit 7 of [u,v]

        ldr     x7, [u+7*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [v+7*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x6, x6, x0
        str     x6, [u+7*N]
        adc     x5, x5, x1

        eor     x1, x7, s10
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x4, x4, x0
        adc     x2, xzr, x1
        eor     x1, x8, s11
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x4, x4, x0
        str     x4, [v+7*N]
        adc     x2, x2, x1

// Digits 8 and 9 of u (top is unsigned)

        ldr     x7, [u+8*N]
        eor     x1, x7, s00
        and     x3, s00, m00
        neg     x3, x3
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [v+8*N]
        eor     x1, x8, s01
        and     x0, s01, m01
        sub     x3, x3, x0
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        adc     x3, x3, x1

// Modular reduction of u, reloading as needed from u[0],...,u[7],x5,x3

        extr    x6, x3, x5, #9
        ldp     x0, x1, [u]
        add     x6, x6, x3, asr #63
        sub     x5, x5, x6, lsl #9
        adds    x0, x0, x6
        asr     x6, x6, #63
        adcs    x1, x1, x6
        stp     x0, x1, [u]
        ldp     x0, x1, [u+16]
        adcs    x0, x0, x6
        adcs    x1, x1, x6
        stp     x0, x1, [u+16]
        ldp     x0, x1, [u+32]
        adcs    x0, x0, x6
        adcs    x1, x1, x6
        stp     x0, x1, [u+32]
        ldp     x0, x1, [u+48]
        adcs    x0, x0, x6
        adcs    x1, x1, x6
        stp     x0, x1, [u+48]
        adc     x5, x5, x6
        str     x5, [u+64]

// Digits 8 and 9 of v (top is unsigned)

        eor     x1, x7, s10
        and     x5, s10, m10
        neg     x5, x5
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, s11
        and     x0, s11, m11
        sub     x5, x5, x0
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x2, x2, x0
        adc     x5, x5, x1

// Modular reduction of v, reloading as needed from v[0],...,v[7],x2,x5

        extr    x6, x5, x2, #9
        ldp     x0, x1, [v]
        add     x6, x6, x5, asr #63
        sub     x2, x2, x6, lsl #9
        adds    x0, x0, x6
        asr     x6, x6, #63
        adcs    x1, x1, x6
        stp     x0, x1, [v]
        ldp     x0, x1, [v+16]
        adcs    x0, x0, x6
        adcs    x1, x1, x6
        stp     x0, x1, [v+16]
        ldp     x0, x1, [v+32]
        adcs    x0, x0, x6
        adcs    x1, x1, x6
        stp     x0, x1, [v+32]
        ldp     x0, x1, [v+48]
        adcs    x0, x0, x6
        adcs    x1, x1, x6
        stp     x0, x1, [v+48]
        adc     x2, x2, x6
        str     x2, [v+64]

Lbignum_inv_p521_midloop:

        mov     x1, d
        ldr     x2, [f]
        ldr     x3, [g]
        divstep59()
        mov     d, x1

// Next iteration

        subs    i, i, #1
        bne     Lbignum_inv_p521_loop

// The 21st and last iteration does not need anything except the
// u value and the sign of f; the latter can be obtained from the
// lowest word of f. So it's done differently from the main loop.
// Find the sign of the new f. For this we just need one digit
// since we know (for in-scope cases) that f is either +1 or -1.
// We don't explicitly shift right by 59 either, but looking at
// bit 63 (or any bit >= 60) of the unshifted result is enough
// to distinguish -1 from +1; this is then made into a mask.

        ldr     x0, [f]
        ldr     x1, [g]
        mul     x0, x0, m00
        madd    x1, x1, m01, x0
        asr     x0, x1, #63

// Now separate out the matrix into sign-magnitude pairs
// and adjust each one based on the sign of f.
//
// Note that at this point we expect |f|=1 and we got its
// sign above, so then since [f,0] == x * [u,v] (mod p_521)
// we want to flip the sign of u according to that of f.

        cmp     m00, xzr
        csetm   s00, mi
        cneg    m00, m00, mi
        eor     s00, s00, x0

        cmp     m01, xzr
        csetm   s01, mi
        cneg    m01, m01, mi
        eor     s01, s01, x0

        cmp     m10, xzr
        csetm   s10, mi
        cneg    m10, m10, mi
        eor     s10, s10, x0

        cmp     m11, xzr
        csetm   s11, mi
        cneg    m11, m11, mi
        eor     s11, s11, x0

// Adjust the initial value to allow for complement instead of negation

        and     x0, m00, s00
        and     x1, m01, s01
        add     car0, x0, x1

// Digit 0 of [u]

        ldr     x7, [u]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x4, car0, x0
        adc     x2, xzr, x1
        ldr     x8, [v]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x4, x4, x0
        str     x4, [u]
        adc     x2, x2, x1

// Digit 1 of [u]

        ldr     x7, [u+N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [v+N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x2, x2, x0
        str     x2, [u+N]
        adc     x6, x6, x1

// Digit 2 of [u]

        ldr     x7, [u+2*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [v+2*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x6, x6, x0
        str     x6, [u+2*N]
        adc     x5, x5, x1

// Digit 3 of [u]

        ldr     x7, [u+3*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, xzr, x1
        ldr     x8, [v+3*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        str     x5, [u+3*N]
        adc     x3, x3, x1

// Digit 4 of [u]

        ldr     x7, [u+4*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x3, x3, x0
        adc     x4, xzr, x1
        ldr     x8, [v+4*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x3, x3, x0
        str     x3, [u+4*N]
        adc     x4, x4, x1

// Digit 5 of [u]

        ldr     x7, [u+5*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x4, x4, x0
        adc     x2, xzr, x1
        ldr     x8, [v+5*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x4, x4, x0
        str     x4, [u+5*N]
        adc     x2, x2, x1

// Digit 6 of [u]

        ldr     x7, [u+6*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x2, x2, x0
        adc     x6, xzr, x1
        ldr     x8, [v+6*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x2, x2, x0
        str     x2, [u+6*N]
        adc     x6, x6, x1

// Digit 7 of [u]

        ldr     x7, [u+7*N]
        eor     x1, x7, s00
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x6, x6, x0
        adc     x5, xzr, x1
        ldr     x8, [v+7*N]
        eor     x1, x8, s01
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x6, x6, x0
        str     x6, [u+7*N]
        adc     x5, x5, x1

// Digits 8 and 9 of u (top is unsigned)

        ldr     x7, [u+8*N]
        eor     x1, x7, s00
        and     x3, s00, m00
        neg     x3, x3
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [v+8*N]
        eor     x1, x8, s01
        and     x0, s01, m01
        sub     x3, x3, x0
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        adc     x3, x3, x1

// Modular reduction of u, reloading as needed from u[0],...,u[7],x5,x3

        extr    x6, x3, x5, #9
        ldp     x10, x11, [u]
        add     x6, x6, x3, asr #63
        sub     x5, x5, x6, lsl #9
        adds    x10, x10, x6
        asr     x6, x6, #63
        adcs    x11, x11, x6
        ldp     x12, x13, [u+16]
        adcs    x12, x12, x6
        adcs    x13, x13, x6
        ldp     x14, x15, [u+32]
        adcs    x14, x14, x6
        adcs    x15, x15, x6
        ldp     x16, x17, [u+48]
        adcs    x16, x16, x6
        adcs    x17, x17, x6
        adc     x19, x5, x6

// Further strict reduction ready for the output, which just means
// a conditional subtraction of p_521

        subs    x0, x10, #-1
        adcs    x1, x11, xzr
        adcs    x2, x12, xzr
        adcs    x3, x13, xzr
        adcs    x4, x14, xzr
        adcs    x5, x15, xzr
        adcs    x6, x16, xzr
        adcs    x7, x17, xzr
        mov     x8, #0x1FF
        sbcs    x8, x19, x8

        csel    x0, x0, x10, cs
        csel    x1, x1, x11, cs
        csel    x2, x2, x12, cs
        csel    x3, x3, x13, cs
        csel    x4, x4, x14, cs
        csel    x5, x5, x15, cs
        csel    x6, x6, x16, cs
        csel    x7, x7, x17, cs
        csel    x8, x8, x19, cs

// Store it back to the final output

        stp     x0, x1, [res]
        stp     x2, x3, [res, #16]
        stp     x4, x5, [res, #32]
        stp     x6, x7, [res, #48]
        str     x8, [res, #64]

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_inv_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
