// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery inverse modulo p_sm2 = 2^256 - 2^224 - 2^96 + 2^64 - 1
// Input x[4]; output z[4]
//
// extern void bignum_montinv_sm2(uint64_t z[static 4],
//                                const uint64_t x[static 4]);
//
// If the 4-digit input x is coprime to p_sm2, i.e. is not divisible
// by it, returns z < p_sm2 such that x * z == 2^512 (mod p_sm2). This
// is effectively "Montgomery inverse" because if we consider x and z as
// Montgomery forms of X and Z, i.e. x == 2^256 * X and z == 2^256 * Z
// (both mod p_sm2) then X * Z == 1 (mod p_sm2). That is, this function
// gives the analog of the modular inverse bignum_inv_sm2 but with both
// input and output in the Montgomery domain. Note that x does not need
// to be reduced modulo p_sm2, but the output always is. If the input
// is divisible (i.e. is 0 or p_sm2), then there can be no solution to
// the congruence x * z == 2^512 (mod p_sm2), and z = 0 is returned.

// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montinv_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montinv_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montinv_sm2)

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
#define g sp, #(6*N)
#define u sp, #(12*N)
#define v sp, #(16*N)

// Total size to reserve on the stack

#define NSPACE 20*N

// ---------------------------------------------------------------------------
// Core signed almost-Montgomery reduction macro. Takes input in
// [d4;d3;d2;d1;d0] and returns result in [d4;d3;d2;d1], adding to
// the existing [d4;d3;d2;d1], and re-using d0 as a temporary internally
// as well as t0, t1, t2, t3. This is almost-Montgomery, i.e. the result
// fits in 4 digits but is not necessarily strictly reduced mod p_sm2.
// ---------------------------------------------------------------------------

#define amontred(d4,d3,d2,d1,d0, t3,t2,t1,t0)                               \
/* We only know the input is -2^316 < x < 2^316. To do traditional  */      \
/* unsigned Montgomery reduction, start by adding 2^61 * p_sm2.     */      \
        mov     t0, #0xe000000000000000 __LF                                   \
        adds    d0, d0, t0 __LF                                                \
        mov     t1, #0x1fffffffffffffff __LF                                   \
        adcs    d1, d1, t1 __LF                                                \
        mov     t2, #0xffffffffe0000000 __LF                                   \
        adcs    d2, d2, t2 __LF                                                \
        sbcs    d3, d3, xzr __LF                                               \
        and     t0, t1, #0xffffffffdfffffff __LF                               \
        adc     d4, d4, t0 __LF                                                \
/* First let [t3;t2] = 2^32 * d0 and [t1;t0] = (2^32-1) * d0 */             \
        lsl     t2, d0, #32 __LF                                               \
        lsr     t3, d0, #32 __LF                                               \
        subs    t0, t2, d0 __LF                                                \
        sbc     t1, t3, xzr __LF                                               \
/* Now [d4;d3;d2;d1] := [d0;d3;d2;d1] - [t3;t2;t1;t0] */                    \
        subs    d1, d1, t0 __LF                                                \
        sbcs    d2, d2, t1 __LF                                                \
        sbcs    d3, d3, t2 __LF                                                \
        sbc     t0, d0, t3 __LF                                                \
        adds    d4, d4, t0 __LF                                                \
/* Now capture top carry and subtract p_sm2 if set (almost-Montgomery) */   \
        csetm   t0, cs __LF                                                    \
        subs    d1, d1, t0 __LF                                                \
        and     t1, t0, #0xffffffff00000000 __LF                               \
        sbcs    d2, d2, t1 __LF                                                \
        and     t2, t0, #0xfffffffeffffffff __LF                               \
        sbcs    d3, d3, t0 __LF                                                \
        sbc     d4, d4, t2

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

S2N_BN_SYMBOL(bignum_montinv_sm2):
        CFI_START

// Save registers and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_DEC_SP(NSPACE)

// Save the return pointer for the end so we can overwrite x0 later

        mov     res, x0

// Copy the prime and input into the main f and g variables respectively.
// Make sure x is reduced so that g <= f as assumed in the bound proof.

        mov     x10, #0xffffffffffffffff
        mov     x11, #0xffffffff00000000
        mov     x13, #0xfffffffeffffffff
        stp     x10, x11, [f]
        stp     x10, x13, [f+2*N]
        str     xzr, [f+4*N]

        ldp     x2, x3, [x1]
        subs    x10, x2, #-1
        sbcs    x11, x3, x11
        ldp     x4, x5, [x1, #(2*N)]
        adcs    x12, x4, xzr
        sbcs    x13, x5, x13

        csel    x2, x2, x10, cc
        csel    x3, x3, x11, cc
        csel    x4, x4, x12, cc
        csel    x5, x5, x13, cc

        stp     x2, x3, [g]
        stp     x4, x5, [g+2*N]
        str     xzr, [g+4*N]

// Also maintain reduced < 2^256 vector [u,v] such that
// [f,g] == x * 2^{5*i-50} * [u,v] (mod p_sm2)
// starting with [p_sm2,x] == x * 2^{5*0-562} * [0,2^562] (mod p_sm2)
// The weird-looking 5*i modifications come in because we are doing
// 64-bit word-sized Montgomery reductions at each stage, which is
// 5 bits more than the 59-bit requirement to keep things stable.
// After the 10th and last iteration and sign adjustment, when
// f == 1 for in-scope cases, we have x * 2^{50-562} * u == 1, i.e.
// x * u == 2^512 as required.

        stp     xzr, xzr, [u]
        stp     xzr, xzr, [u+2*N]

// The starting constant 2^562 mod p_sm2 is
// 0x0018000000040000:0x00040000000c0000:0x000bfffffff80000:0x000c000000100000
// where colons separate 64-bit subwords, least significant at the right.
// These each need a couple of instructions to create  on ARM

        mov     x10, #0x0000000000100000
        orr     x10, x10, #0x000c000000000000
        mov     x11, #0x000c000000000000
        sub     x11, x11, #0x80000
        stp     x10, x11, [v]
        mov     x12, #0x0004000000000000
        orr     x12, x12, #0x00000000000c0000
        mov     x13, #0x0018000000000000
        orr     x13, x13, #0x0000000000040000
        stp     x12, x13, [v+2*N]

// Start of main loop. We jump into the middle so that the divstep
// portion is common to the special tenth iteration after a uniform
// first 9.

        mov     i, #10
        mov     d, #1
        b       Lbignum_montinv_sm2_midloop

Lbignum_montinv_sm2_loop:

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

// Digits 3 and 4 of [f,g]

        ldr     x7, [f+3*N]
        eor     x1, x7, s00
        ldr     x23, [f+4*N]
        eor     x3, x23, s00
        and     x3, x3, m00
        neg     x3, x3
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [g+3*N]
        eor     x1, x8, s01
        ldr     x24, [g+4*N]
        eor     x0, x24, s01
        and     x0, x0, m01
        sub     x3, x3, x0
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        adc     x3, x3, x1
        extr    x6, x5, x6, #59
        str     x6, [f+2*N]
        extr    x5, x3, x5, #59
        str     x5, [f+3*N]
        asr     x3, x3, #59
        str     x3, [f+4*N]

        eor     x1, x7, s10
        eor     x5, x23, s10
        and     x5, x5, m10
        neg     x5, x5
        mul     x0, x1, m10
        umulh   x1, x1, m10
        adds    x2, x2, x0
        adc     x5, x5, x1
        eor     x1, x8, s11
        eor     x0, x24, s11
        and     x0, x0, m11
        sub     x5, x5, x0
        mul     x0, x1, m11
        umulh   x1, x1, m11
        adds    x2, x2, x0
        adc     x5, x5, x1
        extr    x4, x2, x4, #59
        str     x4, [g+2*N]
        extr    x2, x5, x2, #59
        str     x2, [g+3*N]
        asr     x5, x5, #59
        str     x5, [g+4*N]

// Now the computation of the updated u and v values and their
// Montgomery reductions. A very similar accumulation except that
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

// Digits 3 and 4 of u (top is unsigned)

        ldr     x7, [u+3*N]
        eor     x1, x7, s00
        and     x3, s00, m00
        neg     x3, x3
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [v+3*N]
        eor     x1, x8, s01
        and     x0, s01, m01
        sub     x3, x3, x0
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        adc     x3, x3, x1

// Montgomery reduction of u

        ldp     x0, x1, [u]
        ldr     x6, [u+2*N]
        amontred(x3,x5,x6,x1,x0, x24,x10,x11,x14)
        stp     x1, x6, [u]
        stp     x5, x3, [u+16]

// Digits 3 and 4 of v (top is unsigned)

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

// Montgomery reduction of v

        ldp     x0, x1, [v]
        ldr     x3, [v+2*N]
        amontred(x5,x2,x3,x1,x0, x24,x10,x11,x14)
        stp     x1, x3, [v]
        stp     x2, x5, [v+16]

Lbignum_montinv_sm2_midloop:

        mov     x1, d
        ldr     x2, [f]
        ldr     x3, [g]
        divstep59()
        mov     d, x1

// Next iteration

        subs    i, i, #1
        bne     Lbignum_montinv_sm2_loop

// The 10th and last iteration does not need anything except the
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
// sign above, so then since [f,0] == x *  2^{-512} [u,v] (mod p_sm2)
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

// Digits 3 and 4 of u (top is unsigned)

        ldr     x7, [u+3*N]
        eor     x1, x7, s00
        and     x3, s00, m00
        neg     x3, x3
        mul     x0, x1, m00
        umulh   x1, x1, m00
        adds    x5, x5, x0
        adc     x3, x3, x1
        ldr     x8, [v+3*N]
        eor     x1, x8, s01
        and     x0, s01, m01
        sub     x3, x3, x0
        mul     x0, x1, m01
        umulh   x1, x1, m01
        adds    x5, x5, x0
        adc     x3, x3, x1

// Montgomery reduction of u. This needs to be strict not "almost"
// so it is followed by an optional subtraction of p_sm2

        ldp     x0, x1, [u]
        ldr     x2, [u+2*N]
        amontred(x3,x5,x2,x1,x0, x24,x10,x11,x14)

        mov     x10, #0xffffffffffffffff
        subs    x10, x1, #-1
        mov     x11, #0xffffffff00000000
        sbcs    x11, x2, x11
        mov     x13, #0xfffffffeffffffff
        adcs    x12, x5, xzr
        sbcs    x13, x3, x13

        csel    x10, x1, x10, cc
        csel    x11, x2, x11, cc
        csel    x12, x5, x12, cc
        csel    x13, x3, x13, cc

// Store it back to the final output

        stp     x10, x11, [res]
        stp     x12, x13, [res, #16]

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montinv_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
