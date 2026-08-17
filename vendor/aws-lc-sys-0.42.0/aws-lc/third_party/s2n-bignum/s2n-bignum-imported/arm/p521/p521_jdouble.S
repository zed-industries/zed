// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jdouble(uint64_t p3[static 27], const uint64_t p1[static 27]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// It is assumed that all coordinates of the input point are fully
// reduced mod p_521 and that the z coordinate is not zero.
//
// Standard ARM ABI: X0 = p3, X1 = p1
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jdouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jdouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jdouble)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 72

// Stable homes for input arguments during main code sequence

#define input_z x27
#define input_x x28

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries

#define z2 sp, #(NUMSIZE*0)
#define y2 sp, #(NUMSIZE*1)
#define x2p sp, #(NUMSIZE*2)
#define xy2 sp, #(NUMSIZE*3)

#define y4 sp, #(NUMSIZE*4)
#define t2 sp, #(NUMSIZE*4)

#define dx2 sp, #(NUMSIZE*5)
#define t1 sp, #(NUMSIZE*5)

#define d_ sp, #(NUMSIZE*6)
#define x4p sp, #(NUMSIZE*6)

// NUMSIZE*7 is not 16-aligned so we round it up

#define NSPACE 512

// For the two "big" field operations, we use subroutines not inlining.
// Call local code very close to bignum_mul_p521 and bignum_sqr_p521.

#define mul_p521(P0,P1,P2)                      \
        add     x0, P0 __LF                        \
        add     x1, P1 __LF                        \
        add     x2, P2 __LF                        \
        CFI_BL(Lp521_jdouble_local_mul_p521)

// Call local code equivalent to bignum_sqr_p521

#define sqr_p521(P0,P1)                         \
        add     x0, P0 __LF                        \
        add     x1, P1 __LF                        \
        CFI_BL(Lp521_jdouble_local_sqr_p521)

// Corresponds exactly to bignum_add_p521

#define add_p521(P0,P1,P2)                      \
        cmp     xzr, xzr __LF                      \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        adcs    x5, x5, x4 __LF                    \
        adcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        adcs    x7, x7, x4 __LF                    \
        adcs    x8, x8, x3 __LF                    \
        ldp     x9, x10, [P1+32] __LF              \
        ldp     x4, x3, [P2+32] __LF               \
        adcs    x9, x9, x4 __LF                    \
        adcs    x10, x10, x3 __LF                  \
        ldp     x11, x12, [P1+48] __LF             \
        ldp     x4, x3, [P2+48] __LF               \
        adcs    x11, x11, x4 __LF                  \
        adcs    x12, x12, x3 __LF                  \
        ldr     x13, [P1+64] __LF                  \
        ldr     x4, [P2+64] __LF                   \
        adc     x13, x13, x4 __LF                  \
        subs    x4, x13, #512 __LF                 \
        csetm   x4, hs __LF                        \
        sbcs    x5, x5, xzr __LF                   \
        and     x4, x4, #0x200 __LF                \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbc     x13, x13, x4 __LF                  \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32] __LF              \
        stp     x11, x12, [P0+48] __LF             \
        str     x13, [P0+64]

// Corresponds exactly to bignum_sub_p521

#define sub_p521(P0,P1,P2)                      \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        ldp     x9, x10, [P1+32] __LF              \
        ldp     x4, x3, [P2+32] __LF               \
        sbcs    x9, x9, x4 __LF                    \
        sbcs    x10, x10, x3 __LF                  \
        ldp     x11, x12, [P1+48] __LF             \
        ldp     x4, x3, [P2+48] __LF               \
        sbcs    x11, x11, x4 __LF                  \
        sbcs    x12, x12, x3 __LF                  \
        ldr     x13, [P1+64] __LF                  \
        ldr     x4, [P2+64] __LF                   \
        sbcs    x13, x13, x4 __LF                  \
        sbcs    x5, x5, xzr __LF                   \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbcs    x8, x8, xzr __LF                   \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbcs    x11, x11, xzr __LF                 \
        sbcs    x12, x12, xzr __LF                 \
        sbcs    x13, x13, xzr __LF                 \
        and     x13, x13, #0x1ff __LF              \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16] __LF               \
        stp     x9, x10, [P0+32] __LF              \
        stp     x11, x12, [P0+48] __LF             \
        str     x13, [P0+64]

// P0 = C * P1 - D * P2 == C * P1 + D * (p_521 - P2)

#define cmsub_p521(P0,C,P1,D,P2)                \
        ldp     x6, x7, [P1] __LF                  \
        mov     x1, #(C) __LF                      \
        mul     x3, x1, x6 __LF                    \
        mul     x4, x1, x7 __LF                    \
        umulh   x6, x1, x6 __LF                    \
        adds    x4, x4, x6 __LF                    \
        umulh   x7, x1, x7 __LF                    \
        ldp     x8, x9, [P1+16] __LF               \
        mul     x5, x1, x8 __LF                    \
        mul     x6, x1, x9 __LF                    \
        umulh   x8, x1, x8 __LF                    \
        adcs    x5, x5, x7 __LF                    \
        umulh   x9, x1, x9 __LF                    \
        adcs    x6, x6, x8 __LF                    \
        ldp     x10, x11, [P1+32] __LF             \
        mul     x7, x1, x10 __LF                   \
        mul     x8, x1, x11 __LF                   \
        umulh   x10, x1, x10 __LF                  \
        adcs    x7, x7, x9 __LF                    \
        umulh   x11, x1, x11 __LF                  \
        adcs    x8, x8, x10 __LF                   \
        ldp     x12, x13, [P1+48] __LF             \
        mul     x9, x1, x12 __LF                   \
        mul     x10, x1, x13 __LF                  \
        umulh   x12, x1, x12 __LF                  \
        adcs    x9, x9, x11 __LF                   \
        umulh   x13, x1, x13 __LF                  \
        adcs    x10, x10, x12 __LF                 \
        ldr     x14, [P1+64] __LF                  \
        mul     x11, x1, x14 __LF                  \
        adc     x11, x11, x13 __LF                 \
        mov     x1, #(D) __LF                      \
        ldp     x20, x21, [P2] __LF                \
        mvn     x20, x20 __LF                      \
        mul     x0, x1, x20 __LF                   \
        umulh   x20, x1, x20 __LF                  \
        adds    x3, x3, x0 __LF                    \
        mvn     x21, x21 __LF                      \
        mul     x0, x1, x21 __LF                   \
        umulh   x21, x1, x21 __LF                  \
        adcs    x4, x4, x0 __LF                    \
        ldp     x22, x23, [P2+16] __LF             \
        mvn     x22, x22 __LF                      \
        mul     x0, x1, x22 __LF                   \
        umulh   x22, x1, x22 __LF                  \
        adcs    x5, x5, x0 __LF                    \
        mvn     x23, x23 __LF                      \
        mul     x0, x1, x23 __LF                   \
        umulh   x23, x1, x23 __LF                  \
        adcs    x6, x6, x0 __LF                    \
        ldp     x17, x19, [P2+32] __LF             \
        mvn     x17, x17 __LF                      \
        mul     x0, x1, x17 __LF                   \
        umulh   x17, x1, x17 __LF                  \
        adcs    x7, x7, x0 __LF                    \
        mvn     x19, x19 __LF                      \
        mul     x0, x1, x19 __LF                   \
        umulh   x19, x1, x19 __LF                  \
        adcs    x8, x8, x0 __LF                    \
        ldp     x2, x16, [P2+48] __LF              \
        mvn     x2, x2 __LF                        \
        mul     x0, x1, x2 __LF                    \
        umulh   x2, x1, x2 __LF                    \
        adcs    x9, x9, x0 __LF                    \
        mvn     x16, x16 __LF                      \
        mul     x0, x1, x16 __LF                   \
        umulh   x16, x1, x16 __LF                  \
        adcs    x10, x10, x0 __LF                  \
        ldr     x0, [P2+64] __LF                   \
        eor     x0, x0, #0x1ff __LF                \
        mul     x0, x1, x0 __LF                    \
        adc     x11, x11, x0 __LF                  \
        adds    x4, x4, x20 __LF                   \
        adcs    x5, x5, x21 __LF                   \
        and     x15, x4, x5 __LF                   \
        adcs    x6, x6, x22 __LF                   \
        and     x15, x15, x6 __LF                  \
        adcs    x7, x7, x23 __LF                   \
        and     x15, x15, x7 __LF                  \
        adcs    x8, x8, x17 __LF                   \
        and     x15, x15, x8 __LF                  \
        adcs    x9, x9, x19 __LF                   \
        and     x15, x15, x9 __LF                  \
        adcs    x10, x10, x2 __LF                  \
        and     x15, x15, x10 __LF                 \
        adc     x11, x11, x16 __LF                 \
        lsr     x12, x11, #9 __LF                  \
        orr     x11, x11, #0xfffffffffffffe00 __LF \
        cmp     xzr, xzr __LF                      \
        adcs    xzr, x3, x12 __LF                  \
        adcs    xzr, x15, xzr __LF                 \
        adcs    xzr, x11, xzr __LF                 \
        adcs    x3, x3, x12 __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adcs    x6, x6, xzr __LF                   \
        adcs    x7, x7, xzr __LF                   \
        adcs    x8, x8, xzr __LF                   \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        adc     x11, x11, xzr __LF                 \
        and     x11, x11, #0x1ff __LF              \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16] __LF               \
        stp     x7, x8, [P0+32] __LF               \
        stp     x9, x10, [P0+48] __LF              \
        str     x11, [P0+64]

// P0 = 3 * P1 - 8 * P2 == 3 * P1 + 8 * (p_521 - P2)

#define cmsub38_p521(P0,P1,P2)                  \
        ldp     x6, x7, [P1] __LF                  \
        lsl     x3, x6, #1 __LF                    \
        adds    x3, x3, x6 __LF                    \
        extr    x4, x7, x6, #63 __LF               \
        adcs    x4, x4, x7 __LF                    \
        ldp     x8, x9, [P1+16] __LF               \
        extr    x5, x8, x7, #63 __LF               \
        adcs    x5, x5, x8 __LF                    \
        extr    x6, x9, x8, #63 __LF               \
        adcs    x6, x6, x9 __LF                    \
        ldp     x10, x11, [P1+32] __LF             \
        extr    x7, x10, x9, #63 __LF              \
        adcs    x7, x7, x10 __LF                   \
        extr    x8, x11, x10, #63 __LF             \
        adcs    x8, x8, x11 __LF                   \
        ldp     x12, x13, [P1+48] __LF             \
        extr    x9, x12, x11, #63 __LF             \
        adcs    x9, x9, x12 __LF                   \
        extr    x10, x13, x12, #63 __LF            \
        adcs    x10, x10, x13 __LF                 \
        ldr     x14, [P1+64] __LF                  \
        extr    x11, x14, x13, #63 __LF            \
        adc     x11, x11, x14 __LF                 \
        ldp     x20, x21, [P2] __LF                \
        mvn     x20, x20 __LF                      \
        lsl     x0, x20, #3 __LF                   \
        adds    x3, x3, x0 __LF                    \
        mvn     x21, x21 __LF                      \
        extr    x0, x21, x20, #61 __LF             \
        adcs    x4, x4, x0 __LF                    \
        ldp     x22, x23, [P2+16] __LF             \
        mvn     x22, x22 __LF                      \
        extr    x0, x22, x21, #61 __LF             \
        adcs    x5, x5, x0 __LF                    \
        and     x15, x4, x5 __LF                   \
        mvn     x23, x23 __LF                      \
        extr    x0, x23, x22, #61 __LF             \
        adcs    x6, x6, x0 __LF                    \
        and     x15, x15, x6 __LF                  \
        ldp     x20, x21, [P2+32] __LF             \
        mvn     x20, x20 __LF                      \
        extr    x0, x20, x23, #61 __LF             \
        adcs    x7, x7, x0 __LF                    \
        and     x15, x15, x7 __LF                  \
        mvn     x21, x21 __LF                      \
        extr    x0, x21, x20, #61 __LF             \
        adcs    x8, x8, x0 __LF                    \
        and     x15, x15, x8 __LF                  \
        ldp     x22, x23, [P2+48] __LF             \
        mvn     x22, x22 __LF                      \
        extr    x0, x22, x21, #61 __LF             \
        adcs    x9, x9, x0 __LF                    \
        and     x15, x15, x9 __LF                  \
        mvn     x23, x23 __LF                      \
        extr    x0, x23, x22, #61 __LF             \
        adcs    x10, x10, x0 __LF                  \
        and     x15, x15, x10 __LF                 \
        ldr     x0, [P2+64] __LF                   \
        eor     x0, x0, #0x1ff __LF                \
        extr    x0, x0, x23, #61 __LF              \
        adc     x11, x11, x0 __LF                  \
        lsr     x12, x11, #9 __LF                  \
        orr     x11, x11, #0xfffffffffffffe00 __LF \
        cmp     xzr, xzr __LF                      \
        adcs    xzr, x3, x12 __LF                  \
        adcs    xzr, x15, xzr __LF                 \
        adcs    xzr, x11, xzr __LF                 \
        adcs    x3, x3, x12 __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adcs    x6, x6, xzr __LF                   \
        adcs    x7, x7, xzr __LF                   \
        adcs    x8, x8, xzr __LF                   \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        adc     x11, x11, xzr __LF                 \
        and     x11, x11, #0x1ff __LF              \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16] __LF               \
        stp     x7, x8, [P0+32] __LF               \
        stp     x9, x10, [P0+48] __LF              \
        str     x11, [P0+64]

// P0 = 4 * P1 - P2 = 4 * P1 + (p_521 - P2)

#define cmsub41_p521(P0,P1,P2)                  \
        ldp     x6, x7, [P1] __LF                  \
        lsl     x3, x6, #2 __LF                    \
        extr    x4, x7, x6, #62 __LF               \
        ldp     x8, x9, [P1+16] __LF               \
        extr    x5, x8, x7, #62 __LF               \
        extr    x6, x9, x8, #62 __LF               \
        ldp     x10, x11, [P1+32] __LF             \
        extr    x7, x10, x9, #62 __LF              \
        extr    x8, x11, x10, #62 __LF             \
        ldp     x12, x13, [P1+48] __LF             \
        extr    x9, x12, x11, #62 __LF             \
        extr    x10, x13, x12, #62 __LF            \
        ldr     x14, [P1+64] __LF                  \
        extr    x11, x14, x13, #62 __LF            \
        ldp     x0, x1, [P2] __LF                  \
        mvn     x0, x0 __LF                        \
        adds    x3, x3, x0 __LF                    \
        sbcs    x4, x4, x1 __LF                    \
        ldp     x0, x1, [P2+16] __LF               \
        sbcs    x5, x5, x0 __LF                    \
        and     x15, x4, x5 __LF                   \
        sbcs    x6, x6, x1 __LF                    \
        and     x15, x15, x6 __LF                  \
        ldp     x0, x1, [P2+32] __LF               \
        sbcs    x7, x7, x0 __LF                    \
        and     x15, x15, x7 __LF                  \
        sbcs    x8, x8, x1 __LF                    \
        and     x15, x15, x8 __LF                  \
        ldp     x0, x1, [P2+48] __LF               \
        sbcs    x9, x9, x0 __LF                    \
        and     x15, x15, x9 __LF                  \
        sbcs    x10, x10, x1 __LF                  \
        and     x15, x15, x10 __LF                 \
        ldr     x0, [P2+64] __LF                   \
        eor     x0, x0, #0x1ff __LF                \
        adc     x11, x11, x0 __LF                  \
        lsr     x12, x11, #9 __LF                  \
        orr     x11, x11, #0xfffffffffffffe00 __LF \
        cmp     xzr, xzr __LF                      \
        adcs    xzr, x3, x12 __LF                  \
        adcs    xzr, x15, xzr __LF                 \
        adcs    xzr, x11, xzr __LF                 \
        adcs    x3, x3, x12 __LF                   \
        adcs    x4, x4, xzr __LF                   \
        adcs    x5, x5, xzr __LF                   \
        adcs    x6, x6, xzr __LF                   \
        adcs    x7, x7, xzr __LF                   \
        adcs    x8, x8, xzr __LF                   \
        adcs    x9, x9, xzr __LF                   \
        adcs    x10, x10, xzr __LF                 \
        adc     x11, x11, xzr __LF                 \
        and     x11, x11, #0x1ff __LF              \
        stp     x3, x4, [P0] __LF                  \
        stp     x5, x6, [P0+16] __LF               \
        stp     x7, x8, [P0+32] __LF               \
        stp     x9, x10, [P0+48] __LF              \
        str     x11, [P0+64]

S2N_BN_SYMBOL(p521_jdouble):
        CFI_START

// Save regs and make room on stack for temporary variables

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x28)
        CFI_PUSH2(x29,x30)
        CFI_DEC_SP(NSPACE)

// Move the input arguments to stable places

        mov     input_z, x0
        mov     input_x, x1

// Main code, just a sequence of basic field operations

// z2 = z^2
// y2 = y^2

        sqr_p521(z2,z_1)
        sqr_p521(y2,y_1)

// x2p = x^2 - z^4 = (x + z^2) * (x - z^2)

        add_p521(t1,x_1,z2)
        sub_p521(t2,x_1,z2)
        mul_p521(x2p,t1,t2)

// t1 = y + z
// x4p = x2p^2
// xy2 = x * y^2

        add_p521(t1,y_1,z_1)
        sqr_p521(x4p,x2p)
        mul_p521(xy2,x_1,y2)

// t2 = (y + z)^2

        sqr_p521(t2,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_p521(d_,12,xy2,9,x4p)
        sub_p521(t1,t2,z2)

// y4 = y^4

        sqr_p521(y4,y2)

// z_3' = 2 * y * z
// dx2 = d * x2p

        sub_p521(z_3,t1,y2)
        mul_p521(dx2,d_,x2p)

// x' = 4 * xy2 - d

        cmsub41_p521(x_3,xy2,d_)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p521(y_3,dx2,y4)

// Restore stack and registers

        CFI_INC_SP(NSPACE)

        CFI_POP2(x29,x30)
        CFI_POP2(x27,x28)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p521_jdouble)

// Local versions of the two "big" field operations, identical to
// bignum_mul_p521 and bignum_sqr_p521.

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jdouble_local_mul_p521)

Lp521_jdouble_local_mul_p521:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_DEC_SP(80)
        ldr q6, [x2]
        ldp x10, x17, [x1, #16]
        ldr q4, [x1]
        ldr q16, [x2, #32]
        ldp x5, x20, [x2, #16]
        ldr q2, [x1, #32]
        movi v31.2D, #0x00000000ffffffff
        uzp2 v17.4S, v6.4S, v6.4S
        rev64 v7.4S, v6.4S
        ldp x15, x21, [x1]
        xtn v25.2S, v6.2D
        xtn v22.2S, v4.2D
        subs x14, x10, x17
        mul v7.4S, v7.4S, v4.4S
        csetm x8, cc
        rev64 v3.4S, v16.4S
        xtn v1.2S, v16.2D
        ldp x13, x16, [x2]
        mul x26, x10, x5
        uzp2 v16.4S, v16.4S, v16.4S
        uaddlp v26.2D, v7.4S
        cneg x4, x14, cc
        subs x24, x15, x21
        xtn v5.2S, v2.2D
        mul v28.4S, v3.4S, v2.4S
        shl v26.2D, v26.2D, #32
        mul x22, x17, x20
        umull v20.2D, v22.2S, v25.2S
        uzp2 v6.4S, v4.4S, v4.4S
        umull v18.2D, v22.2S, v17.2S
        uzp2 v4.4S, v2.4S, v2.4S
        cneg x14, x24, cc
        csetm x7, cc
        umulh x11, x17, x20
        usra v18.2D, v20.2D, #32
        uaddlp v7.2D, v28.4S
        subs x19, x16, x13
        umlal v26.2D, v22.2S, v25.2S
        cneg x19, x19, cc
        shl v28.2D, v7.2D, #32
        umull v7.2D, v5.2S, v1.2S
        umull v30.2D, v5.2S, v16.2S
        cinv x6, x7, cc
        mul x25, x14, x19
        umlal v28.2D, v5.2S, v1.2S
        umull v21.2D, v6.2S, v17.2S
        umulh x14, x14, x19
        usra v30.2D, v7.2D, #32
        subs x9, x20, x5
        and v29.16B, v18.16B, v31.16B
        cinv x23, x8, cc
        mov x8, v26.d[1]
        cneg x12, x9, cc
        usra v21.2D, v18.2D, #32
        umlal v29.2D, v6.2S, v25.2S
        mul x24, x4, x12
        umull v18.2D, v4.2S, v16.2S
        movi v25.2D, #0x00000000ffffffff
        eor x9, x14, x6
        and v7.16B, v30.16B, v25.16B
        usra v21.2D, v29.2D, #32
        umulh x7, x10, x5
        usra v18.2D, v30.2D, #32
        umlal v7.2D, v4.2S, v1.2S
        mov x19, v21.d[0]
        umulh x3, x4, x12
        mov x14, v21.d[1]
        usra v18.2D, v7.2D, #32
        adds x4, x8, x19
        mov x8, v26.d[0]
        adcs x19, x26, x14
        adcs x14, x22, x7
        adc x12, x11, xzr
        adds x11, x4, x8
        adcs x26, x19, x4
        adcs x22, x14, x19
        eor x4, x24, x23
        adcs x14, x12, x14
        eor x7, x25, x6
        adc x25, xzr, x12
        eor x19, x3, x23
        adds x3, x26, x8
        adcs x24, x22, x11
        adcs x12, x14, x26
        adcs x22, x25, x22
        adcs x26, xzr, x14
        adc x14, xzr, x25
        cmn x23, #0x1
        adcs x22, x22, x4
        adcs x19, x26, x19
        adc x25, x14, x23
        subs x14, x21, x17
        cneg x23, x14, cc
        csetm x26, cc
        subs x4, x20, x16
        cneg x14, x4, cc
        cinv x4, x26, cc
        cmn x6, #0x1
        adcs x11, x11, x7
        mul x7, x23, x14
        adcs x9, x3, x9
        adcs x26, x24, x6
        umulh x3, x23, x14
        adcs x14, x12, x6
        adcs x22, x22, x6
        adcs x12, x19, x6
        extr x24, x11, x8, #55
        adc x6, x25, x6
        subs x19, x15, x17
        csetm x17, cc
        cneg x23, x19, cc
        subs x19, x20, x13
        lsl x25, x8, #9
        eor x8, x7, x4
        cneg x20, x19, cc
        umulh x7, x23, x20
        cinv x19, x17, cc
        subs x17, x15, x10
        csetm x15, cc
        stp x25, x24, [sp, #32]
        cneg x24, x17, cc
        mul x20, x23, x20
        subs x25, x5, x13
        cneg x13, x25, cc
        cinv x15, x15, cc
        mul x25, x24, x13
        subs x21, x21, x10
        csetm x23, cc
        cneg x17, x21, cc
        subs x21, x5, x16
        umulh x13, x24, x13
        cinv x10, x23, cc
        cneg x23, x21, cc
        cmn x4, #0x1
        adcs x14, x14, x8
        eor x21, x3, x4
        adcs x21, x22, x21
        eor x5, x20, x19
        adcs x24, x12, x4
        mul x12, x17, x23
        eor x8, x25, x15
        adc x25, x6, x4
        cmn x15, #0x1
        adcs x6, x9, x8
        ldp x20, x8, [x2, #48]
        eor x9, x13, x15
        adcs x4, x26, x9
        umulh x26, x17, x23
        ldp x17, x13, [x1, #48]
        adcs x9, x14, x15
        adcs x16, x21, x15
        adcs x14, x24, x15
        eor x21, x7, x19
        mul x23, x17, x20
        adc x24, x25, x15
        cmn x19, #0x1
        adcs x7, x4, x5
        adcs x9, x9, x21
        umulh x3, x13, x8
        adcs x16, x16, x19
        adcs x22, x14, x19
        eor x5, x12, x10
        adc x12, x24, x19
        cmn x10, #0x1
        adcs x19, x7, x5
        eor x14, x26, x10
        mov x7, v28.d[1]
        adcs x24, x9, x14
        extr x4, x19, x6, #55
        umulh x15, x17, x20
        mov x14, v18.d[1]
        lsr x9, x19, #55
        adcs x5, x16, x10
        mov x16, v18.d[0]
        adcs x19, x22, x10
        str x9, [sp, #64]
        extr x25, x6, x11, #55
        adc x21, x12, x10
        subs x26, x17, x13
        stp x25, x4, [sp, #48]
        stp x19, x21, [sp, #16]
        csetm x6, cc
        cneg x4, x26, cc
        mul x19, x13, x8
        subs x11, x8, x20
        stp x24, x5, [sp]
        ldp x21, x10, [x1, #32]
        cinv x12, x6, cc
        cneg x6, x11, cc
        mov x9, v28.d[0]
        umulh x25, x4, x6
        adds x22, x7, x16
        ldp x16, x5, [x2, #32]
        adcs x14, x23, x14
        adcs x11, x19, x15
        adc x24, x3, xzr
        adds x3, x22, x9
        adcs x15, x14, x22
        mul x22, x4, x6
        adcs x6, x11, x14
        adcs x4, x24, x11
        eor x14, x25, x12
        adc x26, xzr, x24
        subs x7, x21, x10
        csetm x23, cc
        cneg x19, x7, cc
        subs x24, x5, x16
        cneg x11, x24, cc
        cinv x7, x23, cc
        adds x25, x15, x9
        eor x23, x22, x12
        adcs x22, x6, x3
        mul x24, x19, x11
        adcs x15, x4, x15
        adcs x6, x26, x6
        umulh x19, x19, x11
        adcs x11, xzr, x4
        adc x26, xzr, x26
        cmn x12, #0x1
        adcs x4, x6, x23
        eor x6, x24, x7
        adcs x14, x11, x14
        adc x26, x26, x12
        subs x11, x10, x13
        cneg x12, x11, cc
        csetm x11, cc
        eor x19, x19, x7
        subs x24, x8, x5
        cinv x11, x11, cc
        cneg x24, x24, cc
        cmn x7, #0x1
        adcs x3, x3, x6
        mul x23, x12, x24
        adcs x25, x25, x19
        adcs x6, x22, x7
        umulh x19, x12, x24
        adcs x22, x15, x7
        adcs x12, x4, x7
        eor x24, x23, x11
        adcs x4, x14, x7
        adc x26, x26, x7
        eor x19, x19, x11
        subs x14, x21, x17
        cneg x7, x14, cc
        csetm x14, cc
        subs x23, x20, x16
        cinv x14, x14, cc
        cneg x23, x23, cc
        cmn x11, #0x1
        adcs x22, x22, x24
        mul x24, x7, x23
        adcs x15, x12, x19
        adcs x4, x4, x11
        adc x19, x26, x11
        umulh x26, x7, x23
        subs x7, x21, x13
        eor x11, x24, x14
        cneg x23, x7, cc
        csetm x12, cc
        subs x7, x8, x16
        cneg x7, x7, cc
        cinv x12, x12, cc
        cmn x14, #0x1
        eor x26, x26, x14
        adcs x11, x25, x11
        mul x25, x23, x7
        adcs x26, x6, x26
        adcs x6, x22, x14
        adcs x24, x15, x14
        umulh x23, x23, x7
        adcs x4, x4, x14
        adc x22, x19, x14
        eor x14, x25, x12
        eor x7, x23, x12
        cmn x12, #0x1
        adcs x14, x26, x14
        ldp x19, x25, [x2]
        ldp x15, x23, [x2, #16]
        adcs x26, x6, x7
        adcs x24, x24, x12
        adcs x7, x4, x12
        adc x4, x22, x12
        subs x19, x19, x16
        ldp x16, x22, [x1]
        sbcs x6, x25, x5
        ldp x12, x25, [x1, #16]
        sbcs x15, x15, x20
        sbcs x8, x23, x8
        csetm x23, cc
        subs x21, x21, x16
        eor x16, x19, x23
        sbcs x19, x10, x22
        eor x22, x6, x23
        eor x8, x8, x23
        sbcs x6, x17, x12
        sbcs x13, x13, x25
        csetm x12, cc
        subs x10, x10, x17
        cneg x17, x10, cc
        csetm x25, cc
        subs x5, x20, x5
        eor x10, x19, x12
        cneg x19, x5, cc
        eor x20, x15, x23
        eor x21, x21, x12
        cinv x15, x25, cc
        mul x25, x17, x19
        subs x16, x16, x23
        sbcs x5, x22, x23
        eor x6, x6, x12
        sbcs x20, x20, x23
        eor x22, x13, x12
        sbc x8, x8, x23
        subs x21, x21, x12
        umulh x19, x17, x19
        sbcs x10, x10, x12
        sbcs x17, x6, x12
        eor x6, x19, x15
        eor x19, x25, x15
        umulh x25, x17, x20
        sbc x13, x22, x12
        cmn x15, #0x1
        adcs x22, x14, x19
        adcs x19, x26, x6
        ldp x6, x26, [sp]
        adcs x14, x24, x15
        umulh x24, x21, x16
        adcs x7, x7, x15
        adc x15, x4, x15
        adds x4, x9, x6
        eor x9, x23, x12
        adcs x12, x3, x26
        stp x4, x12, [sp]
        ldp x4, x26, [sp, #16]
        umulh x12, x10, x5
        ldp x6, x23, [sp, #32]
        adcs x3, x11, x4
        mul x4, x13, x8
        adcs x26, x22, x26
        ldp x22, x11, [sp, #48]
        adcs x6, x19, x6
        stp x3, x26, [sp, #16]
        mul x26, x10, x5
        adcs x14, x14, x23
        stp x6, x14, [sp, #32]
        ldr x6, [sp, #64]
        adcs x22, x7, x22
        adcs x14, x15, x11
        mul x11, x17, x20
        adc x19, x6, xzr
        stp x22, x14, [sp, #48]
        adds x14, x26, x24
        str x19, [sp, #64]
        umulh x19, x13, x8
        adcs x7, x11, x12
        adcs x22, x4, x25
        mul x6, x21, x16
        adc x19, x19, xzr
        subs x11, x17, x13
        cneg x12, x11, cc
        csetm x11, cc
        subs x24, x8, x20
        cinv x11, x11, cc
        cneg x24, x24, cc
        adds x4, x14, x6
        adcs x14, x7, x14
        mul x3, x12, x24
        adcs x7, x22, x7
        adcs x22, x19, x22
        umulh x12, x12, x24
        adc x24, xzr, x19
        adds x19, x14, x6
        eor x3, x3, x11
        adcs x26, x7, x4
        adcs x14, x22, x14
        adcs x25, x24, x7
        adcs x23, xzr, x22
        eor x7, x12, x11
        adc x12, xzr, x24
        subs x22, x21, x10
        cneg x24, x22, cc
        csetm x22, cc
        subs x15, x5, x16
        cinv x22, x22, cc
        cneg x15, x15, cc
        cmn x11, #0x1
        adcs x3, x25, x3
        mul x25, x24, x15
        adcs x23, x23, x7
        adc x11, x12, x11
        subs x7, x10, x13
        umulh x15, x24, x15
        cneg x12, x7, cc
        csetm x7, cc
        eor x24, x25, x22
        eor x25, x15, x22
        cmn x22, #0x1
        adcs x24, x4, x24
        adcs x19, x19, x25
        adcs x15, x26, x22
        adcs x4, x14, x22
        adcs x26, x3, x22
        adcs x25, x23, x22
        adc x23, x11, x22
        subs x14, x21, x17
        cneg x3, x14, cc
        csetm x11, cc
        subs x14, x8, x5
        cneg x14, x14, cc
        cinv x7, x7, cc
        subs x13, x21, x13
        cneg x21, x13, cc
        csetm x13, cc
        mul x22, x12, x14
        subs x8, x8, x16
        cinv x13, x13, cc
        umulh x14, x12, x14
        cneg x12, x8, cc
        subs x8, x20, x16
        cneg x8, x8, cc
        cinv x16, x11, cc
        eor x22, x22, x7
        cmn x7, #0x1
        eor x14, x14, x7
        adcs x4, x4, x22
        mul x11, x3, x8
        adcs x22, x26, x14
        adcs x14, x25, x7
        eor x25, x24, x9
        adc x26, x23, x7
        umulh x7, x3, x8
        subs x17, x10, x17
        cneg x24, x17, cc
        eor x3, x11, x16
        csetm x11, cc
        subs x20, x20, x5
        cneg x5, x20, cc
        cinv x11, x11, cc
        cmn x16, #0x1
        mul x17, x21, x12
        eor x8, x7, x16
        adcs x10, x19, x3
        and x19, x9, #0x1ff
        adcs x20, x15, x8
        umulh x15, x21, x12
        eor x12, x10, x9
        eor x8, x6, x9
        adcs x6, x4, x16
        adcs x4, x22, x16
        adcs x21, x14, x16
        adc x7, x26, x16
        mul x10, x24, x5
        cmn x13, #0x1
        ldp x3, x14, [x1]
        eor x17, x17, x13
        umulh x5, x24, x5
        adcs x20, x20, x17
        eor x17, x15, x13
        adcs x16, x6, x17
        eor x22, x10, x11
        adcs x23, x4, x13
        extr x10, x14, x3, #52
        and x26, x3, #0xfffffffffffff
        adcs x24, x21, x13
        and x15, x10, #0xfffffffffffff
        adc x6, x7, x13
        cmn x11, #0x1
        adcs x17, x20, x22
        eor x4, x5, x11
        ldp x21, x10, [sp]
        adcs x7, x16, x4
        eor x16, x17, x9
        eor x13, x7, x9
        ldp x3, x17, [sp, #16]
        adcs x7, x23, x11
        eor x23, x7, x9
        ldp x5, x22, [sp, #32]
        adcs x7, x24, x11
        adc x24, x6, x11
        ldr x6, [x2, #64]
        adds x20, x8, x21
        lsl x11, x20, #9
        eor x4, x7, x9
        orr x7, x11, x19
        eor x8, x24, x9
        adcs x11, x25, x10
        mul x26, x6, x26
        ldp x19, x24, [sp, #48]
        adcs x12, x12, x3
        adcs x16, x16, x17
        adcs x9, x13, x5
        ldr x25, [sp, #64]
        extr x20, x11, x20, #55
        adcs x13, x23, x22
        adcs x4, x4, x19
        extr x23, x12, x11, #55
        adcs x8, x8, x24
        adc x11, x25, xzr
        adds x21, x9, x21
        extr x9, x16, x12, #55
        lsr x12, x16, #55
        adcs x10, x13, x10
        mul x15, x6, x15
        adcs x13, x4, x3
        ldp x16, x4, [x2]
        ldr x3, [x1, #64]
        adcs x17, x8, x17
        adcs x5, x5, x7
        adcs x20, x22, x20
        adcs x8, x19, x23
        and x22, x16, #0xfffffffffffff
        ldp x19, x7, [x1, #16]
        adcs x9, x24, x9
        extr x24, x4, x16, #52
        adc x16, x12, x25
        mul x22, x3, x22
        and x25, x24, #0xfffffffffffff
        extr x14, x19, x14, #40
        and x12, x14, #0xfffffffffffff
        extr x23, x7, x19, #28
        ldp x19, x24, [x2, #16]
        mul x14, x3, x25
        and x23, x23, #0xfffffffffffff
        add x22, x26, x22
        lsl x11, x11, #48
        lsr x26, x22, #52
        lsl x25, x22, #12
        mul x22, x6, x12
        extr x12, x19, x4, #40
        add x4, x15, x14
        mul x15, x6, x23
        add x4, x4, x26
        extr x23, x24, x19, #28
        ldp x14, x19, [x1, #32]
        and x26, x12, #0xfffffffffffff
        extr x12, x4, x25, #12
        and x25, x23, #0xfffffffffffff
        adds x21, x21, x12
        mul x12, x3, x26
        extr x23, x14, x7, #16
        and x23, x23, #0xfffffffffffff
        mul x7, x3, x25
        ldp x25, x26, [x2, #32]
        add x12, x22, x12
        extr x22, x19, x14, #56
        mul x23, x6, x23
        lsr x14, x14, #4
        extr x24, x25, x24, #16
        add x7, x15, x7
        and x15, x24, #0xfffffffffffff
        and x22, x22, #0xfffffffffffff
        lsr x24, x4, #52
        mul x15, x3, x15
        and x14, x14, #0xfffffffffffff
        add x12, x12, x24
        lsl x24, x4, #12
        lsr x4, x12, #52
        extr x24, x12, x24, #24
        adcs x10, x10, x24
        lsl x24, x12, #12
        add x12, x7, x4
        mul x22, x6, x22
        add x4, x23, x15
        extr x7, x12, x24, #36
        adcs x13, x13, x7
        lsl x15, x12, #12
        add x7, x4, x11
        lsr x24, x12, #52
        ldp x23, x11, [x2, #48]
        add x4, x7, x24
        mul x12, x6, x14
        extr x7, x26, x25, #56
        extr x14, x4, x15, #48
        and x2, x7, #0xfffffffffffff
        extr x24, x11, x23, #32
        ldp x15, x7, [x1, #48]
        and x1, x24, #0xfffffffffffff
        lsr x24, x4, #52
        mul x2, x3, x2
        extr x26, x23, x26, #44
        lsr x23, x25, #4
        and x23, x23, #0xfffffffffffff
        and x25, x26, #0xfffffffffffff
        extr x26, x7, x15, #32
        extr x19, x15, x19, #44
        mul x23, x3, x23
        and x15, x26, #0xfffffffffffff
        lsl x26, x4, #12
        and x4, x19, #0xfffffffffffff
        lsr x11, x11, #20
        mul x19, x6, x4
        adcs x17, x17, x14
        add x14, x22, x2
        add x22, x12, x23
        lsr x7, x7, #20
        add x22, x22, x24
        extr x2, x22, x26, #60
        mul x24, x3, x25
        lsr x22, x22, #52
        add x14, x14, x22
        lsl x22, x2, #8
        extr x22, x14, x22, #8
        lsl x2, x14, #12
        mul x1, x3, x1
        adcs x12, x5, x22
        mul x5, x6, x15
        and x26, x10, x13
        and x4, x26, x17
        add x23, x19, x24
        lsr x14, x14, #52
        mul x22, x3, x11
        add x11, x23, x14
        extr x25, x11, x2, #20
        lsl x19, x11, #12
        adcs x25, x20, x25
        and x14, x4, x12
        add x1, x5, x1
        and x14, x14, x25
        mul x15, x6, x7
        add x26, x15, x22
        mul x6, x6, x3
        lsr x22, x11, #52
        add x4, x1, x22
        lsr x1, x4, #52
        extr x3, x4, x19, #32
        lsl x15, x4, #12
        add x7, x26, x1
        adcs x23, x8, x3
        extr x20, x7, x15, #44
        and x3, x14, x23
        lsr x19, x7, #44
        adcs x7, x9, x20
        add x11, x6, x19
        adc x4, x16, x11
        lsr x14, x4, #9
        cmp xzr, xzr
        and x15, x3, x7
        orr x3, x4, #0xfffffffffffffe00
        adcs xzr, x21, x14
        adcs xzr, x15, xzr
        adcs xzr, x3, xzr
        adcs x11, x21, x14
        and x14, x11, #0x1ff
        adcs x1, x10, xzr
        extr x10, x1, x11, #9
        str x14, [x0, #64]
        adcs x14, x13, xzr
        extr x11, x14, x1, #9
        adcs x1, x17, xzr
        extr x4, x1, x14, #9
        stp x10, x11, [x0]
        adcs x11, x12, xzr
        extr x14, x11, x1, #9
        adcs x10, x25, xzr
        extr x11, x10, x11, #9
        stp x4, x14, [x0, #16]
        adcs x14, x23, xzr
        extr x10, x14, x10, #9
        adcs x1, x7, xzr
        stp x11, x10, [x0, #32]
        extr x14, x1, x14, #9
        adc x10, x3, xzr
        extr x26, x10, x1, #9
        stp x14, x26, [x0, #48]
        CFI_INC_SP(80)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jdouble_local_mul_p521)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jdouble_local_sqr_p521)

Lp521_jdouble_local_sqr_p521:
        CFI_START
        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        ldr q23, [x1, #32]
        ldp x9, x2, [x1, #32]
        ldr q16, [x1, #32]
        ldr q20, [x1, #48]
        ldp x6, x13, [x1, #48]
        rev64 v2.4S, v23.4S
        mul x14, x9, x2
        ldr q31, [x1, #48]
        subs x22, x9, x2
        uzp2 v26.4S, v23.4S, v23.4S
        mul v30.4S, v2.4S, v16.4S
        xtn v0.2S, v20.2D
        csetm x12, cc
        xtn v21.2S, v16.2D
        xtn v23.2S, v23.2D
        umulh x10, x9, x6
        rev64 v27.4S, v31.4S
        umull v2.2D, v21.2S, v26.2S
        cneg x23, x22, cc
        uaddlp v25.2D, v30.4S
        umull v18.2D, v21.2S, v23.2S
        mul x22, x9, x6
        mul v6.4S, v27.4S, v20.4S
        uzp2 v17.4S, v20.4S, v20.4S
        shl v20.2D, v25.2D, #32
        uzp2 v27.4S, v31.4S, v31.4S
        mul x16, x2, x13
        umlal v20.2D, v21.2S, v23.2S
        usra v2.2D, v18.2D, #32
        adds x8, x22, x10
        umull v25.2D, v17.2S, v27.2S
        xtn v31.2S, v31.2D
        movi v1.2D, #0xffffffff
        adc x3, x10, xzr
        umulh x21, x2, x13
        uzp2 v21.4S, v16.4S, v16.4S
        umull v18.2D, v0.2S, v27.2S
        subs x19, x13, x6
        and v7.16B, v2.16B, v1.16B
        umull v27.2D, v0.2S, v31.2S
        cneg x20, x19, cc
        movi v30.2D, #0xffffffff
        umull v16.2D, v21.2S, v26.2S
        umlal v7.2D, v21.2S, v23.2S
        mul x19, x23, x20
        cinv x7, x12, cc
        uaddlp v6.2D, v6.4S
        eor x12, x19, x7
        adds x11, x8, x16
        umulh x10, x23, x20
        ldr q1, [x1]
        usra v16.2D, v2.2D, #32
        adcs x19, x3, x21
        shl v2.2D, v6.2D, #32
        adc x20, x21, xzr
        adds x17, x19, x16
        usra v18.2D, v27.2D, #32
        adc x19, x20, xzr
        cmn x7, #0x1
        umlal v2.2D, v0.2S, v31.2S
        umulh x16, x9, x2
        adcs x8, x11, x12
        usra v16.2D, v7.2D, #32
        ldr x12, [x1, #64]
        eor x20, x10, x7
        umulh x10, x6, x13
        mov x23, v2.d[0]
        mov x3, v2.d[1]
        adcs x21, x17, x20
        usra v25.2D, v18.2D, #32
        and v23.16B, v18.16B, v30.16B
        adc x7, x19, x7
        adds x22, x22, x22
        ldr q7, [x1, #16]
        adcs x17, x8, x8
        umlal v23.2D, v17.2S, v31.2S
        mov x19, v16.d[0]
        mul x11, x12, x12
        ldr q4, [x1]
        usra v25.2D, v23.2D, #32
        add x5, x12, x12
        adcs x15, x21, x21
        ldr q28, [x1]
        mov x12, v20.d[1]
        adcs x24, x7, x7
        mov x21, v16.d[1]
        adc x4, xzr, xzr
        adds x19, x19, x14
        ldr q18, [x1, #16]
        xtn v26.2S, v1.2D
        adcs x8, x12, x16
        adc x21, x21, xzr
        adds x7, x19, x14
        xtn v23.2S, v7.2D
        rev64 v21.4S, v28.4S
        adcs x12, x8, x16
        ldp x20, x19, [x1]
        mov x16, v25.d[1]
        xtn v22.2S, v28.2D
        adc x14, x21, xzr
        adds x8, x22, x12
        uzp2 v24.4S, v28.4S, v28.4S
        rev64 v28.4S, v18.4S
        mul x12, x6, x13
        mul v16.4S, v21.4S, v1.4S
        shrn v31.2S, v7.2D, #32
        adcs x22, x17, x14
        mov x14, v25.d[0]
        and x21, x20, #0xfffffffffffff
        umull v17.2D, v26.2S, v24.2S
        ldr q2, [x1, #32]
        adcs x17, x15, xzr
        ldr q30, [x1, #48]
        umull v7.2D, v26.2S, v22.2S
        adcs x15, x24, xzr
        ldr q0, [x1, #16]
        movi v6.2D, #0xffffffff
        adc x4, x4, xzr
        adds x14, x14, x12
        uzp1 v27.4S, v18.4S, v4.4S
        uzp2 v19.4S, v1.4S, v1.4S
        adcs x24, x3, x10
        mul x3, x5, x21
        umull v29.2D, v23.2S, v31.2S
        ldr q5, [x1]
        adc x21, x16, xzr
        adds x16, x14, x12
        extr x12, x19, x20, #52
        umull v18.2D, v19.2S, v24.2S
        adcs x24, x24, x10
        and x10, x12, #0xfffffffffffff
        ldp x14, x12, [x1, #16]
        usra v17.2D, v7.2D, #32
        adc x21, x21, xzr
        adds x23, x23, x17
        mul x17, x5, x10
        shl v21.2D, v29.2D, #33
        lsl x10, x3, #12
        lsr x1, x3, #52
        rev64 v29.4S, v2.4S
        uaddlp v25.2D, v16.4S
        add x17, x17, x1
        adcs x16, x16, x15
        extr x3, x14, x19, #40
        mov x15, v20.d[0]
        extr x10, x17, x10, #12
        and x3, x3, #0xfffffffffffff
        shl v3.2D, v25.2D, #32
        and v6.16B, v17.16B, v6.16B
        mul x1, x5, x3
        usra v18.2D, v17.2D, #32
        adcs x3, x24, x4
        extr x4, x12, x14, #28
        umlal v6.2D, v19.2S, v22.2S
        xtn v20.2S, v2.2D
        umlal v3.2D, v26.2S, v22.2S
        movi v26.2D, #0xffffffff
        lsr x24, x17, #52
        and x4, x4, #0xfffffffffffff
        uzp2 v19.4S, v2.4S, v2.4S
        add x1, x1, x24
        mul x24, x5, x4
        lsl x4, x17, #12
        xtn v24.2S, v5.2D
        extr x17, x1, x4, #24
        adc x21, x21, xzr
        umlal v21.2D, v23.2S, v23.2S
        adds x4, x15, x10
        lsl x10, x1, #12
        adcs x15, x7, x17
        mul v23.4S, v28.4S, v4.4S
        and x7, x4, #0x1ff
        lsr x17, x1, #52
        umulh x1, x19, x12
        uzp2 v17.4S, v5.4S, v5.4S
        extr x4, x15, x4, #9
        add x24, x24, x17
        mul v29.4S, v29.4S, v5.4S
        extr x17, x24, x10, #36
        extr x10, x9, x12, #16
        uzp1 v28.4S, v4.4S, v4.4S
        adcs x17, x8, x17
        and x8, x10, #0xfffffffffffff
        umull v16.2D, v24.2S, v20.2S
        extr x10, x17, x15, #9
        mul x15, x5, x8
        stp x4, x10, [x0]
        lsl x4, x24, #12
        lsr x8, x9, #4
        uaddlp v4.2D, v23.4S
        and x8, x8, #0xfffffffffffff
        umull v23.2D, v24.2S, v19.2S
        mul x8, x5, x8
        extr x10, x2, x9, #56
        lsr x24, x24, #52
        and x10, x10, #0xfffffffffffff
        add x15, x15, x24
        extr x4, x15, x4, #48
        mul x24, x5, x10
        lsr x10, x15, #52
        usra v23.2D, v16.2D, #32
        add x10, x8, x10
        shl v4.2D, v4.2D, #32
        adcs x22, x22, x4
        extr x4, x6, x2, #44
        lsl x15, x15, #12
        lsr x8, x10, #52
        extr x15, x10, x15, #60
        and x10, x4, #0xfffffffffffff
        umlal v4.2D, v28.2S, v27.2S
        add x8, x24, x8
        extr x4, x13, x6, #32
        mul x24, x5, x10
        uzp2 v16.4S, v30.4S, v30.4S
        lsl x10, x15, #8
        rev64 v28.4S, v30.4S
        and x15, x4, #0xfffffffffffff
        extr x4, x8, x10, #8
        mul x10, x5, x15
        lsl x15, x8, #12
        adcs x23, x23, x4
        lsr x4, x8, #52
        lsr x8, x13, #20
        add x4, x24, x4
        mul x8, x5, x8
        lsr x24, x4, #52
        extr x15, x4, x15, #20
        lsl x4, x4, #12
        add x10, x10, x24
        adcs x15, x16, x15
        extr x4, x10, x4, #32
        umulh x5, x20, x14
        adcs x3, x3, x4
        usra v18.2D, v6.2D, #32
        lsl x16, x10, #12
        extr x24, x15, x23, #9
        lsr x10, x10, #52
        uzp2 v27.4S, v0.4S, v0.4S
        add x8, x8, x10
        extr x10, x3, x15, #9
        extr x4, x22, x17, #9
        and v25.16B, v23.16B, v26.16B
        lsr x17, x8, #44
        extr x15, x8, x16, #44
        extr x16, x23, x22, #9
        xtn v7.2S, v30.2D
        mov x8, v4.d[0]
        stp x24, x10, [x0, #32]
        uaddlp v30.2D, v29.4S
        stp x4, x16, [x0, #16]
        umulh x24, x20, x19
        adcs x15, x21, x15
        adc x16, x11, x17
        subs x11, x20, x19
        xtn v5.2S, v0.2D
        csetm x17, cc
        extr x3, x15, x3, #9
        mov x22, v4.d[1]
        cneg x21, x11, cc
        subs x10, x12, x14
        mul v31.4S, v28.4S, v0.4S
        cneg x10, x10, cc
        cinv x11, x17, cc
        shl v4.2D, v30.2D, #32
        umull v28.2D, v5.2S, v16.2S
        extr x23, x16, x15, #9
        adds x4, x8, x5
        mul x17, x21, x10
        umull v22.2D, v5.2S, v7.2S
        adc x15, x5, xzr
        adds x4, x4, x22
        uaddlp v2.2D, v31.4S
        lsr x5, x16, #9
        adcs x16, x15, x1
        mov x15, v18.d[0]
        adc x1, x1, xzr
        umulh x10, x21, x10
        adds x22, x16, x22
        umlal v4.2D, v24.2S, v20.2S
        umull v30.2D, v27.2S, v16.2S
        stp x3, x23, [x0, #48]
        add x3, x7, x5
        adc x16, x1, xzr
        usra v28.2D, v22.2D, #32
        mul x23, x20, x19
        eor x1, x17, x11
        cmn x11, #0x1
        mov x17, v18.d[1]
        umull v18.2D, v17.2S, v19.2S
        adcs x7, x4, x1
        eor x1, x10, x11
        umlal v25.2D, v17.2S, v20.2S
        movi v16.2D, #0xffffffff
        adcs x22, x22, x1
        usra v18.2D, v23.2D, #32
        umulh x4, x14, x14
        adc x1, x16, x11
        adds x10, x8, x8
        shl v23.2D, v2.2D, #32
        str x3, [x0, #64]
        adcs x5, x7, x7
        and v16.16B, v28.16B, v16.16B
        usra v30.2D, v28.2D, #32
        adcs x7, x22, x22
        mov x21, v3.d[1]
        adcs x11, x1, x1
        umlal v16.2D, v27.2S, v7.2S
        adc x22, xzr, xzr
        adds x16, x15, x23
        mul x8, x14, x12
        umlal v23.2D, v5.2S, v7.2S
        usra v18.2D, v25.2D, #32
        umulh x15, x14, x12
        adcs x21, x21, x24
        usra v30.2D, v16.2D, #32
        adc x1, x17, xzr
        adds x3, x16, x23
        adcs x21, x21, x24
        adc x1, x1, xzr
        adds x24, x10, x21
        umulh x21, x12, x12
        adcs x16, x5, x1
        adcs x10, x7, xzr
        mov x17, v21.d[1]
        adcs x23, x11, xzr
        adc x5, x22, xzr
        adds x1, x4, x8
        adcs x22, x17, x15
        ldp x17, x4, [x0]
        mov x11, v21.d[0]
        adc x21, x21, xzr
        adds x1, x1, x8
        adcs x15, x22, x15
        adc x8, x21, xzr
        adds x22, x11, x10
        mov x21, v3.d[0]
        adcs x11, x1, x23
        ldp x1, x10, [x0, #16]
        adcs x15, x15, x5
        adc x7, x8, xzr
        adds x8, x17, x21
        mov x23, v4.d[1]
        ldp x5, x21, [x0, #32]
        adcs x17, x4, x3
        ldr x4, [x0, #64]
        mov x3, v18.d[0]
        adcs x24, x1, x24
        stp x8, x17, [x0]
        adcs x17, x10, x16
        ldp x1, x16, [x0, #48]
        adcs x5, x5, x22
        adcs x8, x21, x11
        stp x5, x8, [x0, #32]
        adcs x1, x1, x15
        mov x15, v23.d[1]
        adcs x21, x16, x7
        stp x1, x21, [x0, #48]
        adc x10, x4, xzr
        subs x7, x14, x12
        mov x16, v18.d[1]
        cneg x5, x7, cc
        csetm x4, cc
        subs x11, x13, x6
        mov x8, v23.d[0]
        cneg x7, x11, cc
        cinv x21, x4, cc
        mov x11, v30.d[0]
        adds x4, x23, x3
        mul x22, x5, x7
        mov x23, v30.d[1]
        adcs x8, x8, x16
        adcs x16, x15, x11
        adc x11, x23, xzr
        umulh x3, x5, x7
        stp x24, x17, [x0, #16]
        mov x5, v4.d[0]
        subs x15, x20, x19
        cneg x7, x15, cc
        str x10, [x0, #64]
        csetm x1, cc
        subs x24, x2, x9
        cneg x17, x24, cc
        cinv x15, x1, cc
        adds x23, x4, x5
        umulh x1, x7, x17
        adcs x24, x8, x4
        adcs x10, x16, x8
        eor x8, x22, x21
        adcs x16, x11, x16
        mul x22, x7, x17
        eor x17, x1, x15
        adc x1, xzr, x11
        adds x11, x24, x5
        eor x7, x3, x21
        adcs x3, x10, x23
        adcs x24, x16, x24
        adcs x4, x1, x10
        eor x10, x22, x15
        adcs x16, xzr, x16
        adc x1, xzr, x1
        cmn x21, #0x1
        adcs x8, x4, x8
        adcs x22, x16, x7
        adc x7, x1, x21
        subs x21, x19, x12
        csetm x4, cc
        cneg x1, x21, cc
        subs x21, x13, x2
        cinv x16, x4, cc
        cneg x4, x21, cc
        cmn x15, #0x1
        adcs x21, x23, x10
        mul x23, x1, x4
        adcs x11, x11, x17
        adcs x3, x3, x15
        umulh x1, x1, x4
        adcs x24, x24, x15
        adcs x8, x8, x15
        adcs x22, x22, x15
        eor x17, x23, x16
        adc x15, x7, x15
        subs x7, x20, x14
        cneg x7, x7, cc
        csetm x4, cc
        subs x10, x20, x12
        cneg x23, x10, cc
        csetm x10, cc
        subs x12, x6, x9
        cinv x20, x4, cc
        cneg x12, x12, cc
        cmn x16, #0x1
        eor x1, x1, x16
        adcs x17, x24, x17
        mul x4, x7, x12
        adcs x8, x8, x1
        umulh x1, x7, x12
        adcs x24, x22, x16
        adc x7, x15, x16
        subs x12, x13, x9
        cneg x12, x12, cc
        cinv x13, x10, cc
        subs x19, x19, x14
        mul x9, x23, x12
        cneg x19, x19, cc
        csetm x10, cc
        eor x16, x1, x20
        subs x22, x6, x2
        umulh x12, x23, x12
        eor x1, x4, x20
        cinv x4, x10, cc
        cneg x22, x22, cc
        cmn x20, #0x1
        adcs x15, x11, x1
        eor x6, x12, x13
        adcs x10, x3, x16
        adcs x17, x17, x20
        eor x23, x9, x13
        adcs x2, x8, x20
        mul x11, x19, x22
        adcs x24, x24, x20
        adc x7, x7, x20
        cmn x13, #0x1
        adcs x3, x10, x23
        umulh x22, x19, x22
        adcs x17, x17, x6
        eor x12, x22, x4
        extr x22, x15, x21, #63
        adcs x8, x2, x13
        extr x21, x21, x5, #63
        ldp x16, x23, [x0]
        adcs x20, x24, x13
        eor x1, x11, x4
        adc x6, x7, x13
        cmn x4, #0x1
        ldp x2, x7, [x0, #16]
        adcs x1, x3, x1
        extr x19, x1, x15, #63
        adcs x14, x17, x12
        extr x1, x14, x1, #63
        lsl x17, x5, #1
        adcs x8, x8, x4
        extr x12, x8, x14, #8
        ldp x15, x11, [x0, #32]
        adcs x9, x20, x4
        adc x3, x6, x4
        adds x16, x12, x16
        extr x6, x9, x8, #8
        ldp x14, x12, [x0, #48]
        extr x8, x3, x9, #8
        adcs x20, x6, x23
        ldr x24, [x0, #64]
        lsr x6, x3, #8
        adcs x8, x8, x2
        and x2, x1, #0x1ff
        and x1, x20, x8
        adcs x4, x6, x7
        adcs x3, x17, x15
        and x1, x1, x4
        adcs x9, x21, x11
        and x1, x1, x3
        adcs x6, x22, x14
        and x1, x1, x9
        and x21, x1, x6
        adcs x14, x19, x12
        adc x1, x24, x2
        cmp xzr, xzr
        orr x12, x1, #0xfffffffffffffe00
        lsr x1, x1, #9
        adcs xzr, x16, x1
        and x21, x21, x14
        adcs xzr, x21, xzr
        adcs xzr, x12, xzr
        adcs x21, x16, x1
        adcs x1, x20, xzr
        adcs x19, x8, xzr
        stp x21, x1, [x0]
        adcs x1, x4, xzr
        adcs x21, x3, xzr
        stp x19, x1, [x0, #16]
        adcs x1, x9, xzr
        stp x21, x1, [x0, #32]
        adcs x21, x6, xzr
        adcs x1, x14, xzr
        stp x21, x1, [x0, #48]
        adc x1, x12, xzr
        and x1, x1, #0x1ff
        str x1, [x0, #64]
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jdouble_local_sqr_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
