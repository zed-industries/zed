// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended projective + precomputed mixed addition for edwards25519
// Inputs p1[16], p2[12]; output p3[16]
//
// extern void edwards25519_pepadd_alt
//   (uint64_t p3[static 16],const uint64_t p1[static 16],
//    const uint64_t p2[static 12]);
//
// The output p3 and the first input p1 are points (x,y) on edwards25519
// represented in extended projective quadruples (X,Y,Z,T) where
// x = X / Z, y = Y / Z and x * y = T / Z. The second input p2 is a triple
// encoding its point (x,y) as (y - x,y + x,2 * d * x * y) where d is the
// usual Edwards curve parameter for edwards25519.
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_pepadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_pepadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_pepadd_alt)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define p3 x17
#define p1 x19
#define p2 x20

// Pointers to input and output coordinates

#define x_1 p1, #0
#define y_1 p1, #NUMSIZE
#define z_1 p1, #(2*NUMSIZE)
#define w_1 p1, #(3*NUMSIZE)

#define ymx_2 p2, #0
#define xpy_2 p2, #NUMSIZE
#define kxy_2 p2, #(2*NUMSIZE)

#define x_3 p3, #0
#define y_3 p3, #NUMSIZE
#define z_3 p3, #(2*NUMSIZE)
#define w_3 p3, #(3*NUMSIZE)

// Pointer-offset pairs for temporaries on stack

#define t0 sp, #(0*NUMSIZE)
#define t1 sp, #(1*NUMSIZE)
#define t2 sp, #(2*NUMSIZE)
#define t3 sp, #(3*NUMSIZE)
#define t4 sp, #(4*NUMSIZE)
#define t5 sp, #(5*NUMSIZE)

// Total size to reserve on the stack

#define NSPACE 6*NUMSIZE

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

// Plain 4-digit add and doubling without any normalization
// With inputs < p_25519 (indeed < 2^255) it still gives a 4-digit result,
// indeed one < 2 * p_25519 for normalized inputs.

#define add_4(P0,P1,P2)                         \
        ldp     x0, x1, [P1] __LF                  \
        ldp     x4, x5, [P2] __LF                  \
        adds    x0, x0, x4 __LF                    \
        adcs    x1, x1, x5 __LF                    \
        ldp     x2, x3, [P1+16] __LF               \
        ldp     x6, x7, [P2+16] __LF               \
        adcs    x2, x2, x6 __LF                    \
        adc     x3, x3, x7 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16]

#define double_4(P0,P1)                         \
        ldp     x0, x1, [P1] __LF                  \
        adds    x0, x0, x0 __LF                    \
        adcs    x1, x1, x1 __LF                    \
        ldp     x2, x3, [P1+16] __LF               \
        adcs    x2, x2, x2 __LF                    \
        adc     x3, x3, x3 __LF                    \
        stp     x0, x1, [P0] __LF                  \
        stp     x2, x3, [P0+16]

// Subtraction of a pair of numbers < p_25519 just sufficient
// to give a 4-digit result. It actually always does (x - z) + (2^255-19)
// which in turn is done by (x - z) - (2^255+19) discarding the 2^256
// implicitly

#define sub_4(P0,P1,P2)                         \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x3, #19 __LF                       \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        mov     x4, #0x8000000000000000 __LF       \
        sbc     x8, x8, x4 __LF                    \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

// Modular subtraction with double modulus 2 * p_25519 = 2^256 - 38

#define sub_twice4(P0,P1,P2)                    \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x4, #38 __LF                       \
        csel    x3, x4, xzr, lo __LF               \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbc     x8, x8, xzr __LF                   \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

// Modular addition with inputs double modulus 2 * p_25519 = 2^256 - 38
// and in general only guaranteeing a 4-digit result, not even < 2 * p_25519.

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

S2N_BN_SYMBOL(edwards25519_pepadd_alt):
        CFI_START

// Save regs and make room for temporaries

        CFI_PUSH2(x19,x20)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     p3, x0
        mov     p1, x1
        mov     p2, x2

// Main sequence

        double_4(t0,z_1);

        sub_4(t1,y_1,x_1);
        add_4(t2,y_1,x_1);

        mul_4(t3,w_1,kxy_2);

        mul_4(t1,t1,ymx_2);
        mul_4(t2,t2,xpy_2);

        sub_twice4(t4,t0,t3);
        add_twice4(t0,t0,t3);
        sub_twice4(t5,t2,t1);
        add_twice4(t1,t2,t1);

        mul_p25519(z_3,t4,t0);
        mul_p25519(x_3,t5,t4);
        mul_p25519(y_3,t0,t1);
        mul_p25519(w_3,t5,t1);

// Restore stack and registers

        CFI_INC_SP(NSPACE)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_pepadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
