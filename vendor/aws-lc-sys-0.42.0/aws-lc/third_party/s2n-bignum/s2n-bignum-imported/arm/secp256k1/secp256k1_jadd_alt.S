// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on SECG curve secp256k1 in Jacobian coordinates
//
//    extern void secp256k1_jadd_alt(uint64_t p3[static 12],
//                                   const uint64_t p1[static 12],
//                                   const uint64_t p2[static 12]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// It is assumed that all coordinates of the input points p1 and p2 are
// fully reduced mod p_256k1, that both z coordinates are nonzero and
// that neither p1 =~= p2 or p1 =~= -p2, where "=~=" means "represents
// the same affine point as".
//
// Standard ARM ABI: X0 = p3, X1 = p1, X2 = p2
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(secp256k1_jadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(secp256k1_jadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(secp256k1_jadd_alt)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence

#define input_z x15
#define input_x x16
#define input_y x17

// Pointer-offset pairs for inputs and outputs

#define x_1 input_x, #0
#define y_1 input_x, #NUMSIZE
#define z_1 input_x, #(2*NUMSIZE)

#define x_2 input_y, #0
#define y_2 input_y, #NUMSIZE
#define z_2 input_y, #(2*NUMSIZE)

#define x_3 input_z, #0
#define y_3 input_z, #NUMSIZE
#define z_3 input_z, #(2*NUMSIZE)

// Pointer-offset pairs for temporaries, with some aliasing
// #NSPACE is the total stack needed for these temporaries

#define z1sq sp, #(NUMSIZE*0)
#define ww sp, #(NUMSIZE*0)
#define resx sp, #(NUMSIZE*0)

#define yd sp, #(NUMSIZE*1)
#define y2a sp, #(NUMSIZE*1)

#define x2a sp, #(NUMSIZE*2)
#define zzx2 sp, #(NUMSIZE*2)

#define zz sp, #(NUMSIZE*3)
#define t1 sp, #(NUMSIZE*3)

#define t2 sp, #(NUMSIZE*4)
#define x1a sp, #(NUMSIZE*4)
#define zzx1 sp, #(NUMSIZE*4)
#define resy sp, #(NUMSIZE*4)

#define xd sp, #(NUMSIZE*5)
#define z2sq sp, #(NUMSIZE*5)
#define resz sp, #(NUMSIZE*5)

#define y1a sp, #(NUMSIZE*6)

#define NSPACE NUMSIZE*7

// Corresponds exactly to bignum_mul_p256k1_alt

#define mul_p256k1(P0,P1,P2)                    \
        ldp     x3, x4, [P1] __LF                  \
        ldp     x7, x8, [P2] __LF                  \
        mul     x12, x3, x7 __LF                   \
        umulh   x13, x3, x7 __LF                   \
        mul     x11, x3, x8 __LF                   \
        umulh   x14, x3, x8 __LF                   \
        adds    x13, x13, x11 __LF                 \
        ldp     x9, x10, [P2+16] __LF              \
        mul     x11, x3, x9 __LF                   \
        umulh   x0, x3, x9 __LF                    \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x3, x10 __LF                  \
        umulh   x1, x3, x10 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        adc     x1, x1, xzr __LF                   \
        ldp     x5, x6, [P1+16] __LF               \
        mul     x11, x4, x7 __LF                   \
        adds    x13, x13, x11 __LF                 \
        mul     x11, x4, x8 __LF                   \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x4, x9 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        mul     x11, x4, x10 __LF                  \
        adcs    x1, x1, x11 __LF                   \
        umulh   x3, x4, x10 __LF                   \
        adc     x3, x3, xzr __LF                   \
        umulh   x11, x4, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        umulh   x11, x4, x8 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        umulh   x11, x4, x9 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        adc     x3, x3, xzr __LF                   \
        mul     x11, x5, x7 __LF                   \
        adds    x14, x14, x11 __LF                 \
        mul     x11, x5, x8 __LF                   \
        adcs    x0, x0, x11 __LF                   \
        mul     x11, x5, x9 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        mul     x11, x5, x10 __LF                  \
        adcs    x3, x3, x11 __LF                   \
        umulh   x4, x5, x10 __LF                   \
        adc     x4, x4, xzr __LF                   \
        umulh   x11, x5, x7 __LF                   \
        adds    x0, x0, x11 __LF                   \
        umulh   x11, x5, x8 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        umulh   x11, x5, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        adc     x4, x4, xzr __LF                   \
        mul     x11, x6, x7 __LF                   \
        adds    x0, x0, x11 __LF                   \
        mul     x11, x6, x8 __LF                   \
        adcs    x1, x1, x11 __LF                   \
        mul     x11, x6, x9 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        mul     x11, x6, x10 __LF                  \
        adcs    x4, x4, x11 __LF                   \
        umulh   x5, x6, x10 __LF                   \
        adc     x5, x5, xzr __LF                   \
        umulh   x11, x6, x7 __LF                   \
        adds    x1, x1, x11 __LF                   \
        umulh   x11, x6, x8 __LF                   \
        adcs    x3, x3, x11 __LF                   \
        umulh   x11, x6, x9 __LF                   \
        adcs    x4, x4, x11 __LF                   \
        adc     x5, x5, xzr __LF                   \
        mov     x7, #0x3d1 __LF                    \
        orr     x7, x7, #0x100000000 __LF          \
        mul     x11, x7, x1 __LF                   \
        umulh   x9, x7, x1 __LF                    \
        adds    x12, x12, x11 __LF                 \
        mul     x11, x7, x3 __LF                   \
        umulh   x3, x7, x3 __LF                    \
        adcs    x13, x13, x11 __LF                 \
        mul     x11, x7, x4 __LF                   \
        umulh   x4, x7, x4 __LF                    \
        adcs    x14, x14, x11 __LF                 \
        mul     x11, x7, x5 __LF                   \
        umulh   x5, x7, x5 __LF                    \
        adcs    x0, x0, x11 __LF                   \
        cset    x1, cs __LF                        \
        adds    x13, x13, x9 __LF                  \
        adcs    x14, x14, x3 __LF                  \
        adcs    x0, x0, x4 __LF                    \
        adc     x1, x1, x5 __LF                    \
        add     x8, x1, #0x1 __LF                  \
        mul     x11, x7, x8 __LF                   \
        umulh   x9, x7, x8 __LF                    \
        adds    x12, x12, x11 __LF                 \
        adcs    x13, x13, x9 __LF                  \
        adcs    x14, x14, xzr __LF                 \
        adcs    x0, x0, xzr __LF                   \
        csel    x7, x7, xzr, cc __LF               \
        subs    x12, x12, x7 __LF                  \
        sbcs    x13, x13, xzr __LF                 \
        sbcs    x14, x14, xzr __LF                 \
        sbc     x0, x0, xzr __LF                   \
        stp     x12, x13, [P0] __LF                \
        stp     x14, x0, [P0+16]

// Corresponds exactly to bignum_sqr_p256k1_alt

#define sqr_p256k1(P0,P1)                       \
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
        mov     x3, #0x3d1 __LF                    \
        orr     x3, x3, #0x100000000 __LF          \
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
        adds    x9, x9, x4 __LF                    \
        adcs    x10, x10, x13 __LF                 \
        adcs    x11, x11, x14 __LF                 \
        adc     x12, x12, x6 __LF                  \
        add     x2, x12, #0x1 __LF                 \
        mul     x7, x3, x2 __LF                    \
        umulh   x6, x3, x2 __LF                    \
        adds    x8, x8, x7 __LF                    \
        adcs    x9, x9, x6 __LF                    \
        adcs    x10, x10, xzr __LF                 \
        adcs    x11, x11, xzr __LF                 \
        csel    x3, x3, xzr, cc __LF               \
        subs    x8, x8, x3 __LF                    \
        sbcs    x9, x9, xzr __LF                   \
        sbcs    x10, x10, xzr __LF                 \
        sbc     x11, x11, xzr __LF                 \
        stp     x8, x9, [P0] __LF                  \
        stp     x10, x11, [P0+16]

// Corresponds exactly to bignum_sub_p256k1

#define sub_p256k1(P0,P1,P2)                    \
        ldp     x5, x6, [P1] __LF                  \
        ldp     x4, x3, [P2] __LF                  \
        subs    x5, x5, x4 __LF                    \
        sbcs    x6, x6, x3 __LF                    \
        ldp     x7, x8, [P1+16] __LF               \
        ldp     x4, x3, [P2+16] __LF               \
        sbcs    x7, x7, x4 __LF                    \
        sbcs    x8, x8, x3 __LF                    \
        mov     x4, #0x3d1 __LF                    \
        orr     x3, x4, #0x100000000 __LF          \
        csel    x3, x3, xzr, cc __LF               \
        subs    x5, x5, x3 __LF                    \
        sbcs    x6, x6, xzr __LF                   \
        sbcs    x7, x7, xzr __LF                   \
        sbc     x8, x8, xzr __LF                   \
        stp     x5, x6, [P0] __LF                  \
        stp     x7, x8, [P0+16]

S2N_BN_SYMBOL(secp256k1_jadd_alt):
        CFI_START

// Make room on stack for temporary variables
// Move the input arguments to stable places

        CFI_DEC_SP(NSPACE)

        mov     input_z, x0
        mov     input_x, x1
        mov     input_y, x2

// Main code, just a sequence of basic field operations

        sqr_p256k1(z1sq,z_1)
        sqr_p256k1(z2sq,z_2)

        mul_p256k1(y1a,z_2,y_1)
        mul_p256k1(y2a,z_1,y_2)

        mul_p256k1(x2a,z1sq,x_2)
        mul_p256k1(x1a,z2sq,x_1)
        mul_p256k1(y2a,z1sq,y2a)
        mul_p256k1(y1a,z2sq,y1a)

        sub_p256k1(xd,x2a,x1a)
        sub_p256k1(yd,y2a,y1a)

        sqr_p256k1(zz,xd)
        sqr_p256k1(ww,yd)

        mul_p256k1(zzx1,zz,x1a)
        mul_p256k1(zzx2,zz,x2a)

        sub_p256k1(resx,ww,zzx1)
        sub_p256k1(t1,zzx2,zzx1)

        mul_p256k1(xd,xd,z_1)

        sub_p256k1(resx,resx,zzx2)

        sub_p256k1(t2,zzx1,resx)

        mul_p256k1(t1,t1,y1a)
        mul_p256k1(resz,xd,z_2)
        mul_p256k1(t2,yd,t2)

        sub_p256k1(resy,t2,t1)

// Load in the z coordinates of the inputs to check for P1 = 0 and P2 = 0
// The condition codes get set by a comparison (P2 != 0) - (P1 != 0)
// So  "HI" <=> CF /\ ~ZF <=> P1 = 0 /\ ~(P2 = 0)
// and "LO" <=> ~CF       <=> ~(P1 = 0) /\ P2 = 0

        ldp     x0, x1, [z_1]
        ldp     x2, x3, [z_1+16]

        orr     x12, x0, x1
        orr     x13, x2, x3
        orr     x12, x12, x13
        cmp     x12, xzr
        cset    x12, ne

        ldp     x4, x5, [z_2]
        ldp     x6, x7, [z_2+16]

        orr     x13, x4, x5
        orr     x14, x6, x7
        orr     x13, x13, x14
        cmp     x13, xzr
        cset    x13, ne

        cmp     x13, x12

// Multiplex the outputs accordingly, re-using the z's in registers

        ldp     x8, x9, [resz]
        csel    x8, x0, x8, lo
        csel    x9, x1, x9, lo
        csel    x8, x4, x8, hi
        csel    x9, x5, x9, hi
        ldp     x10, x11, [resz+16]
        csel    x10, x2, x10, lo
        csel    x11, x3, x11, lo
        csel    x10, x6, x10, hi
        csel    x11, x7, x11, hi

        ldp     x12, x13, [x_1]
        ldp     x0, x1, [resx]
        csel    x0, x12, x0, lo
        csel    x1, x13, x1, lo
        ldp     x12, x13, [x_2]
        csel    x0, x12, x0, hi
        csel    x1, x13, x1, hi

        ldp     x12, x13, [x_1+16]
        ldp     x2, x3, [resx+16]
        csel    x2, x12, x2, lo
        csel    x3, x13, x3, lo
        ldp     x12, x13, [x_2+16]
        csel    x2, x12, x2, hi
        csel    x3, x13, x3, hi

        ldp     x12, x13, [y_1]
        ldp     x4, x5, [resy]
        csel    x4, x12, x4, lo
        csel    x5, x13, x5, lo
        ldp     x12, x13, [y_2]
        csel    x4, x12, x4, hi
        csel    x5, x13, x5, hi

        ldp     x12, x13, [y_1+16]
        ldp     x6, x7, [resy+16]
        csel    x6, x12, x6, lo
        csel    x7, x13, x7, lo
        ldp     x12, x13, [y_2+16]
        csel    x6, x12, x6, hi
        csel    x7, x13, x7, hi

// Finally store back the multiplexed values

        stp     x0, x1, [x_3]
        stp     x2, x3, [x_3+16]
        stp     x4, x5, [y_3]
        stp     x6, x7, [y_3+16]
        stp     x8, x9, [z_3]
        stp     x10, x11, [z_3+16]

// Restore stack and return

        CFI_INC_SP(NSPACE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(secp256k1_jadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
