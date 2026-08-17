// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Projective scalar multiplication, x coordinate only, for curve25519
// Inputs scalar[4], point[4]; output res[8]
//
// extern void curve25519_pxscalarmul_alt
//   (uint64_t res[static 8],const uint64_t scalar[static 4],
//    const uint64_t point[static 4]);
//
// Given the X coordinate of an input point = (X,Y) on curve25519, which
// could also be part of a projective representation (X,Y,1) of the same
// point, returns a projective representation (X,Z) = scalar * point, where
// scalar is a 256-bit number. The corresponding affine form is (X/Z,Y'),
// X/Z meaning division modulo 2^255-19, and Y' not being computed by
// this function (nor is any Y coordinate of the input point used).
//
// Standard x86-64 ABI: RDI = res, RSI = scalar, RDX = point
// Microsoft x64 ABI:   RCX = res, RDX = scalar, R8 = point
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_pxscalarmul_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_pxscalarmul_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_pxscalarmul_alt)
        .text

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for input arguments during main code sequence
// and additional registers for loop counter and swap flag

#define res  10*NUMSIZE(%rsp)
#define point  10*NUMSIZE+8(%rsp)
#define scalar  10*NUMSIZE+16(%rsp)
#define i  10*NUMSIZE+24(%rsp)
#define swap  10*NUMSIZE+32(%rsp)

// Pointers to input x coord (we don't use y or z) and output coords.
// These all assume the base address (point and res respectively) is
// currently in the %rbp register.

#define x 0(%rbp)
#define resx 0(%rbp)
#define resz NUMSIZE(%rbp)

// Pointer-offset pairs for temporaries on stack with some aliasing.
// Both dmsn and dnsm need space for >= 5 digits, and we allocate 8

#define zm (0*NUMSIZE)(%rsp)
#define sm (0*NUMSIZE)(%rsp)
#define dpro (0*NUMSIZE)(%rsp)

#define sn (1*NUMSIZE)(%rsp)

#define dm (2*NUMSIZE)(%rsp)

#define zn (3*NUMSIZE)(%rsp)
#define dn (3*NUMSIZE)(%rsp)
#define e (3*NUMSIZE)(%rsp)

#define dmsn (4*NUMSIZE)(%rsp)
#define p (4*NUMSIZE)(%rsp)

#define xm (6*NUMSIZE)(%rsp)
#define dnsm (6*NUMSIZE)(%rsp)
#define spro (6*NUMSIZE)(%rsp)

#define xn (8*NUMSIZE)(%rsp)
#define s (8*NUMSIZE)(%rsp)

#define d (9*NUMSIZE)(%rsp)

// Total size to reserve on the stack
// This includes space for the 5 other variables above

#define NSPACE 360

// Macros wrapping up the basic field operation calls
// bignum_mul_p25519_alt and bignum_sqr_p25519_alt.
// These two are only trivially different from pure
// function calls to those subroutines.

#define mul_p25519(P0,P1,P2)                    \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    %r13, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    %r14, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    %r15, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        shldq   $0x1, %r11, %r12 ;                    \
        leaq    0x1(%r12), %rax ;                  \
        movl    $0x13, %esi ;                       \
        bts     $63, %r11 ;                         \
        imulq   %rsi, %rax ;                        \
        addq    %rax, %r8 ;                         \
        adcq    %rcx, %r9 ;                         \
        adcq    %rcx, %r10 ;                        \
        adcq    %rcx, %r11 ;                        \
        sbbq    %rax, %rax ;                        \
        notq    %rax;                            \
        andq    %rsi, %rax ;                        \
        subq    %rax, %r8 ;                         \
        sbbq    %rcx, %r9 ;                         \
        sbbq    %rcx, %r10 ;                        \
        sbbq    %rcx, %r11 ;                        \
        btr     $63, %r11 ;                         \
        movq    %r8, P0 ;                        \
        movq    %r9, 0x8+P0 ;                    \
        movq    %r10, 0x10+P0 ;                  \
        movq    %r11, 0x18+P0

#define sqr_p25519(P0,P1)                       \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P1;             \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r11 ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq    %rax;                            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r12 ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r13 ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r13 ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r14 ;                        \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r15 ;                        \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        shldq   $0x1, %r11, %r12 ;                    \
        leaq    0x1(%r12), %rax ;                  \
        movl    $0x13, %esi ;                       \
        bts     $63, %r11 ;                         \
        imulq   %rsi, %rax ;                        \
        addq    %rax, %r8 ;                         \
        adcq    %rcx, %r9 ;                         \
        adcq    %rcx, %r10 ;                        \
        adcq    %rcx, %r11 ;                        \
        sbbq    %rax, %rax ;                        \
        notq    %rax;                            \
        andq    %rsi, %rax ;                        \
        subq    %rax, %r8 ;                         \
        sbbq    %rcx, %r9 ;                         \
        sbbq    %rcx, %r10 ;                        \
        sbbq    %rcx, %r11 ;                        \
        btr     $63, %r11 ;                         \
        movq    %r8, P0 ;                        \
        movq    %r9, 0x8+P0 ;                    \
        movq    %r10, 0x10+P0 ;                  \
        movq    %r11, 0x18+P0

// Multiplication just giving a 5-digit result (actually < 39 * p_25519)
// by not doing anything beyond the first stage of reduction

#define mul_5(P0,P1,P2)                         \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    %r13, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    %r14, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    %r15, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0 ;                 \
        movq    %r12, 0x20+P0

// Squaring just giving a result < 2 * p_25519, which is done by
// basically skipping the +1 in the quotient estimate and the final
// optional correction.

#define sqr_4(P0,P1)                            \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P1;             \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r11 ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq    %rax;                            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r12 ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r13 ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r13 ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r14 ;                        \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r15 ;                        \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        shldq   $0x1, %r11, %r12 ;                  \
        btr     $0x3f, %r11 ;                      \
        movl    $0x13, %edx ;                      \
        imulq   %r12, %rdx ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rcx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Plain 4-digit add without any normalization
// With inputs < p_25519 (indeed < 2^255) it still gives a 4-digit result

#define add_4(P0,P1,P2)                         \
        movq    P1, %rax ;                      \
        addq    P2, %rax ;                      \
        movq    %rax, P0 ;                      \
        movq    8+P1, %rax ;                    \
        adcq    8+P2, %rax ;                    \
        movq    %rax, 8+P0 ;                    \
        movq    16+P1, %rax ;                   \
        adcq    16+P2, %rax ;                   \
        movq    %rax, 16+P0 ;                   \
        movq    24+P1, %rax ;                   \
        adcq    24+P2, %rax ;                   \
        movq    %rax, 24+P0

// Add 5-digit inputs and normalize to 4 digits

#define add5_4(P0,P1,P2)                        \
        movq    P1, %r8 ;                       \
        addq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        adcq    8+P2, %r9 ;                     \
        movq    16+P1, %r10 ;                   \
        adcq    16+P2, %r10 ;                   \
        movq    24+P1, %r11 ;                   \
        adcq    24+P2, %r11 ;                   \
        movq    32+P1, %r12 ;                   \
        adcq    32+P2, %r12 ;                   \
        xorl    %ebx, %ebx ;                       \
        shldq  $0x1, %r11, %r12 ;                   \
        btr    $0x3f, %r11 ;                       \
        movl   $0x13, %edx ;                       \
        imulq  %r12, %rdx ;                        \
        addq   %rdx, %r8 ;                         \
        adcq   %rbx, %r9 ;                         \
        adcq   %rbx, %r10 ;                        \
        adcq   %rbx, %r11 ;                        \
        movq   %r8, P0 ;                        \
        movq   %r9, 0x8+P0 ;                    \
        movq   %r10, 0x10+P0 ;                  \
        movq   %r11, 0x18+P0

// Subtraction of a pair of numbers < p_25519 just sufficient
// to give a 4-digit result. It actually always does (x - z) + (2^255-19)
// which in turn is done by (x - z) - (2^255+19) discarding the 2^256
// implicitly

#define sub_4(P0,P1,P2)                         \
        movq    P1, %r8 ;                       \
        subq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        sbbq    8+P2, %r9 ;                     \
        movq    16+P1, %r10 ;                   \
        sbbq    16+P2, %r10 ;                   \
        movq    24+P1, %rax ;                   \
        sbbq    24+P2, %rax ;                   \
        subq    $19, %r8 ;                         \
        movq    %r8, P0 ;                       \
        sbbq    $0, %r9 ;                          \
        movq    %r9, 8+P0 ;                     \
        sbbq    $0, %r10 ;                         \
        movq    %r10, 16+P0 ;                   \
        sbbq    $0, %rax ;                         \
        btc     $63, %rax ;                        \
        movq    %rax, 24+P0

// Modular subtraction with double modulus 2 * p_25519 = 2^256 - 38

#define sub_twice4(P0,P1,P2)                    \
        movq    P1, %r8 ;                       \
        xorl    %ebx, %ebx ;                       \
        subq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        sbbq    8+P2, %r9 ;                     \
        movl    $38, %ecx ;                        \
        movq    16+P1, %r10 ;                   \
        sbbq    16+P2, %r10 ;                   \
        movq    24+P1, %rax ;                   \
        sbbq    24+P2, %rax ;                   \
        cmovncq %rbx, %rcx ;                       \
        subq    %rcx, %r8 ;                        \
        sbbq    %rbx, %r9 ;                        \
        sbbq    %rbx, %r10 ;                       \
        sbbq    %rbx, %rax ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 8+P0 ;                     \
        movq    %r10, 16+P0 ;                   \
        movq    %rax, 24+P0

// 5-digit subtraction with upward bias to make it positive, adding
// 1000 * (2^255 - 19) = 2^256 * 500 - 19000, then normalizing to 4 digits

#define sub5_4(P0,P1,P2)                        \
        movq    P1, %r8 ;                       \
        subq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        sbbq    8+P2, %r9 ;                     \
        movq    16+P1, %r10 ;                   \
        sbbq    16+P2, %r10 ;                   \
        movq    24+P1, %r11 ;                   \
        sbbq    24+P2, %r11 ;                   \
        movq    32+P1, %r12 ;                   \
        sbbq    32+P2, %r12 ;                   \
        xorl    %ebx, %ebx ;                       \
        subq    $19000, %r8 ;                      \
        sbbq    %rbx, %r9 ;                        \
        sbbq    %rbx, %r10 ;                       \
        sbbq    %rbx, %r11 ;                       \
        sbbq    %rbx, %r12 ;                       \
        addq    $500, %r12 ;                       \
        shldq   $0x1, %r11, %r12 ;                  \
        btr     $0x3f, %r11 ;                      \
        movl    $0x13, %edx ;                      \
        imulq   %r12, %rdx ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rbx, %r9 ;                        \
        adcq    %rbx, %r10 ;                       \
        adcq    %rbx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Combined z = c * x + y with reduction only < 2 * p_25519
// It is assumed that 19 * (c * x + y) < 2^60 * 2^256 so we
// don't need a high mul in the final part.

#define cmadd_4(P0,C1,P2,P3)                    \
        movq    $C1, %rsi ;                         \
        movq    P2, %rax ;                       \
        mulq    %rsi;                            \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        movq    0x8+P2, %rax ;                   \
        xorq    %r10, %r10 ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        movq    0x10+P2, %rax ;                  \
        mulq    %rsi;                            \
        addq    %rax, %r10 ;                        \
        adcq    $0x0, %rdx ;                        \
        movq    0x18+P2, %rax ;                  \
        movq    %rdx, %r11 ;                        \
        mulq    %rsi;                            \
        xorl    %esi, %esi ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rsi, %rdx ;                        \
        addq    P3, %r8 ;                        \
        adcq    0x8+P3, %r9 ;                    \
        adcq    0x10+P3, %r10 ;                  \
        adcq    0x18+P3, %r11 ;                  \
        adcq    %rsi, %rdx ;                        \
        shldq   $0x1, %r11, %rdx ;                  \
        btr     $63, %r11 ;                        \
        movl    $0x13, %ebx ;                      \
        imulq   %rbx, %rdx ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rsi, %r9 ;                        \
        adcq    %rsi, %r10 ;                       \
        adcq    %rsi, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Multiplex: z := if NZ then x else y

#define mux_4(P0,P1,P2)                         \
        movq    P1, %rax ;                      \
        movq    P2, %rcx ;                      \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, P0 ;                      \
        movq    8+P1, %rax ;                    \
        movq    8+P2, %rcx ;                    \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, 8+P0 ;                    \
        movq    16+P1, %rax ;                   \
        movq    16+P2, %rcx ;                   \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, 16+P0 ;                   \
        movq    24+P1, %rax ;                   \
        movq    24+P2, %rcx ;                   \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, 24+P0

S2N_BN_SYMBOL(curve25519_pxscalarmul_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save registers, make room for temps, preserve input arguments.

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(NSPACE)

// Move the input arguments to stable places

        movq    %rdi, res
        movq    %rsi, scalar
        movq    %rdx, point

// Initialize (xn,zn) = (1,0) and (xm,zm) = (x,1) with swap = 0

        movq    $1, %rax
        movq    %rax, 256(%rsp)
        movq    %rax, (%rsp)
        xorl    %eax, %eax
        movq    %rax, swap
        movq    %rax, 96(%rsp)
        movq    %rax, 264(%rsp)
        movq    %rax, 8(%rsp)
        movq    %rax, 104(%rsp)
        movq    %rax, 272(%rsp)
        movq    %rax, 16(%rsp)
        movq    %rax, 112(%rsp)
        movq    %rax, 280(%rsp)
        movq    %rax, 24(%rsp)
        movq    %rax, 120(%rsp)
        movq    (%rdx), %rax
        movq    %rax, 192(%rsp)
        movq    8(%rdx), %rax
        movq    %rax, 200(%rsp)
        movq    16(%rdx), %rax
        movq    %rax, 208(%rsp)
        movq    24(%rdx), %rax
        movq    %rax, 216(%rsp)

// The outer loop from i = 255, ..., i = 0 (inclusive)

        movl    $255, %eax
        movq    %rax, i

Lcurve25519_pxscalarmul_alt_loop:

// sm = xm + zm; sn = xn + zn; dm = xm - zm; dn = xn - zn
// The adds don't need any normalization as they're fed to muls
// Just make sure the subs fit in 4 digits.

        sub_4(dm,xm,zm)
        add_4(sn,xn,zn)
        sub_4(dn,xn,zn)
        add_4(sm,xm,zm)

// ADDING: dmsn = dm * sn; dnsm = sm * dn
// DOUBLING: mux d = xt - zt and s = xt + zt for appropriate choice of (xt,zt)

        mul_5(dmsn,sn,dm)

        movq    scalar, %rax
        movq    i, %rdx
        movq    %rdx, %rcx
        shrq    $6, %rdx
        movq    (%rax,%rdx,8), %rdx
        shrq    %cl, %rdx
        andq    $1, %rdx
        cmpq    swap, %rdx
        movq    %rdx, swap

        mux_4(d,dm,dn)
        mux_4(s,sm,sn)

        mul_5(dnsm,sm,dn)

// DOUBLING: d = (xt - zt)^2 normalized only to 4 digits

        sqr_4(d,d)

// ADDING: dpro = (dmsn - dnsm)^2, spro = (dmsn + dnsm)^2
// DOUBLING: s = (xt + zt)^2, normalized only to 4 digits

        sub5_4(dpro,dmsn,dnsm)
        sqr_4(s,s)
        add5_4(spro,dmsn,dnsm)
        sqr_4(dpro,dpro)

// DOUBLING: p = 4 * xt * zt = s - d

        sub_twice4(p,s,d)

// ADDING: xm' = (dmsn + dnsm)^2

        sqr_p25519(xm,spro)

// DOUBLING: e = 121666 * p + d

        cmadd_4(e,0x1db42,p,d)

// DOUBLING: xn' = (xt + zt)^2 * (xt - zt)^2 = s * d

        mul_p25519(xn,s,d)

// ADDING: zm' = x * (dmsn - dnsm)^2

        movq    point, %rbp
        mul_p25519(zm,dpro,x)

// DOUBLING: zn' = (4 * xt * zt) * ((xt - zt)^2 + 121666 * (4 * xt * zt))
//               = p * (d + 121666 * p)

        mul_p25519(zn,p,e)

// Loop down as far as 0 (inclusive)

        movq    i, %rax
        subq    $1, %rax
        movq    %rax, i
        jnc     Lcurve25519_pxscalarmul_alt_loop

// The main loop does not handle the special input of the 2-torsion
// point = (0,0). In that case we may get a spurious (0,0) as output
// when we want (0,1) [for odd scalar] or (1,0) [for even scalar].
// Test if x = 0 (this is equivalent for curve25519 to y = 0) and if
// so, patch zm = 1 [for odd multiple], xn = 1 [for even multiple].

        movl    $1, %ecx
        movq    point, %rbp
        movq    (%rbp), %rax
        orq     8(%rbp), %rax
        orq     16(%rbp), %rax
        orq     24(%rbp), %rax
        cmovnzq %rcx, %rax
        xorq    $1, %rax
        orq     %rax, (%rsp)
        orq     %rax, 256(%rsp)

// Multiplex into the final outputs

        movq    res, %rbp
        movq    swap, %rax
        testq   %rax, %rax

        mux_4(resx,xm,xn)
        mux_4(resz,zm,zn)

// Restore stack and registers

        CFI_INC_RSP(NSPACE)

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(curve25519_pxscalarmul_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
