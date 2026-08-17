// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on SECG curve secp256k1 in Jacobian coordinates
//
//    extern void secp256k1_jdouble_alt(uint64_t p3[static 12],
//                                      const uint64_t p1[static 12]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// It is assumed that all coordinates of the input point are fully
// reduced mod p_256k1 and that the z coordinate is not zero.
//
// Standard x86-64 ABI: RDI = p3, RSI = p1
// Microsoft x64 ABI:   RCX = p3, RDX = p1
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(secp256k1_jdouble_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(secp256k1_jdouble_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(secp256k1_jdouble_alt)
        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Pointer-offset pairs for inputs and outputs
// These assume %rdi = p3, %rsi = p1, which is true when the
// arguments come in initially and is not disturbed throughout.

#define x_1 0(%rsi)
#define y_1 NUMSIZE(%rsi)
#define z_1 (2*NUMSIZE)(%rsi)

#define x_3 0(%rdi)
#define y_3 NUMSIZE(%rdi)
#define z_3 (2*NUMSIZE)(%rdi)

// Pointer-offset pairs for temporaries, with some aliasing
// NSPACE is the total stack needed for these temporaries

#define x_2 (NUMSIZE*0)(%rsp)
#define y_2 (NUMSIZE*1)(%rsp)
#define d (NUMSIZE*2)(%rsp)
#define tmp (NUMSIZE*3)(%rsp)
#define x_4 (NUMSIZE*4)(%rsp)
#define y_4 (NUMSIZE*6)(%rsp)
#define dx2 (NUMSIZE*8)(%rsp)
#define xy2 (NUMSIZE*10)(%rsp)

#define NSPACE NUMSIZE*12

// Corresponds to bignum_mul_p256k1_alt except %rsi -> %rbx

#define mul_p256k1(P0,P1,P2)                    \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        xorq    %r10, %r10 ;                       \
        xorq    %r11, %r11 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    %r12, %r12 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    %r13, %r13 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        movq    $0x1000003d1, %rbx ;               \
        movq    %r12, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rcx, %rcx ;                       \
        movq    %r13, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r14, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r15, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        xorq    %rcx, %rcx ;                       \
        addq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        adcq    %rcx, %r12 ;                       \
        leaq    0x1(%r12), %rax ;                 \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        sbbq    %rax, %rax ;                       \
        notq    %rax;                            \
        andq    %rbx, %rax ;                       \
        subq    %rax, %r8 ;                        \
        sbbq    %rcx, %r9 ;                        \
        sbbq    %rcx, %r10 ;                       \
        sbbq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Corresponds to bignum_sqr_p256k1_alt except for %rsi -> %rbx

#define sqr_p256k1(P0,P1)                       \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        xorq    %r10, %r10 ;                       \
        xorq    %r11, %r11 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x8+P1;             \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r11 ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rax;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r12 ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r14 ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r15 ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        movq    $0x1000003d1, %rbx ;               \
        movq    %r12, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rcx, %rcx ;                       \
        movq    %r13, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r14, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r15, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        xorq    %rcx, %rcx ;                       \
        addq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        adcq    %rcx, %r12 ;                       \
        leaq    0x1(%r12), %rax ;                 \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        sbbq    %rax, %rax ;                       \
        notq    %rax;                            \
        andq    %rbx, %rax ;                       \
        subq    %rax, %r8 ;                        \
        sbbq    %rcx, %r9 ;                        \
        sbbq    %rcx, %r10 ;                       \
        sbbq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Rough versions producing 5-word results

#define roughmul_p256k1(P0,P1,P2)               \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        xorq    %r10, %r10 ;                       \
        xorq    %r11, %r11 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    %r12, %r12 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    %r13, %r13 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        movq    $0x1000003d1, %rbx ;               \
        movq    %r12, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rcx, %rcx ;                       \
        movq    %r13, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r14, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r15, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        xorq    %rcx, %rcx ;                       \
        addq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        adcq    %rcx, %r12 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0 ;                 \
        movq    %r12, 0x20+P0

#define roughsqr_p256k1(P0,P1)                  \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        xorq    %r10, %r10 ;                       \
        xorq    %r11, %r11 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x8+P1;             \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r11 ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rax;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r12 ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r14 ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r15 ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        movq    $0x1000003d1, %rbx ;               \
        movq    %r12, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rcx, %rcx ;                       \
        movq    %r13, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r14, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r15, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        xorq    %rcx, %rcx ;                       \
        addq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        adcq    %rcx, %r12 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0 ;                  \
        movq    %r12, 0x20+P0

// Weak doubling operation, staying in 4 digits but not in general
// fully normalizing

#define weakdouble_p256k1(P0,P1)                \
        movq    24+P1, %r11 ;                    \
        movq    16+P1, %r10 ;                    \
        movq    $0x1000003d1, %rax ;               \
        xorq    %rdx, %rdx ;                       \
        shldq   $1, %r10, %r11 ;                      \
        cmovncq %rdx, %rax ;                       \
        movq    8+P1, %r9 ;                      \
        shldq   $1, %r9, %r10 ;                       \
        movq    P1, %r8 ;                        \
        shldq   $1, %r8, %r9 ;                        \
        shlq    $1, %r8 ;                           \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 8+P0 ;                     \
        movq    %r10, 16+P0 ;                   \
        movq    %r11, 24+P0

// P0 = C * P1 - D * P2 with 5-word inputs P1 and P2
// Only used here with C = 12, D = 9, but could be used more generally.
// We actually compute C * P1 + D * (2^33 * p_256k1 - P2)

#define cmsub_p256k1(P0,C,P1,D,P2)              \
        movq    $0xfffff85e00000000, %r9 ;         \
        subq    P2, %r9 ;                       \
        movq    $0xfffffffffffffffd, %r10 ;        \
        sbbq    8+P2, %r10 ;                    \
        movq    $0xffffffffffffffff, %r11 ;        \
        sbbq    16+P2, %r11 ;                   \
        movq    $0xffffffffffffffff, %r12 ;        \
        sbbq    24+P2, %r12 ;                   \
        movq    $0x00000001ffffffff, %r13 ;        \
        sbbq    32+P2, %r13 ;                   \
        movq    $D, %rcx ;                         \
        movq    %r9, %rax ;                        \
        mulq    %rcx;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        movq    %r10, %rax ;                       \
        xorl    %r10d, %r10d ;                     \
        mulq    %rcx;                            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        movq    %r11, %rax ;                       \
        xorl    %r11d, %r11d ;                     \
        mulq    %rcx;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        movq    %r12, %rax ;                       \
        xorl    %r12d, %r12d ;                     \
        mulq    %rcx;                            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        imulq   %r13, %rcx ;                       \
        addq    %rcx, %r12 ;                       \
        movq    $C, %rcx ;                         \
        movq    P1, %rax ;                      \
        mulq    %rcx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rbx, %rbx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rbx, %rbx ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rbx, %rbx ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rcx;                            \
        addq    %rax, %r12 ;                       \
        leaq    0x1(%r12), %rax ;                  \
        movq    $0x1000003d1, %rcx ;                \
        mulq    %rcx;                            \
        xorl    %ebx, %ebx ;                       \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        adcq    %rbx, %r10 ;                        \
        adcq    %rbx, %r11 ;                        \
        cmovbq  %rbx, %rcx ;                        \
        subq    %rcx, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    %rbx, %r9 ;                         \
        movq    %r9, 8+P0 ;                      \
        sbbq    %rbx, %r10 ;                        \
        movq    %r10, 16+P0 ;                    \
        sbbq    %rbx, %r11 ;                        \
        movq    %r11, 24+P0 ;                    \

// P0 = 3 * P1 - 8 * P2 with 5-digit P1 and P2
// We actually compute 3 * P1 + (2^33 * p_256k1 - P2) << 3

#define cmsub38_p256k1(P0,P1,P2)                \
        movq    $0xfffff85e00000000, %r8 ;         \
        subq    P2, %r8 ;                       \
        movq    $0xfffffffffffffffd, %r9 ;         \
        sbbq    8+P2, %r9 ;                     \
        movq    $0xffffffffffffffff, %r10 ;        \
        sbbq    16+P2, %r10 ;                   \
        movq    $0xffffffffffffffff, %r11 ;        \
        sbbq    24+P2, %r11 ;                   \
        movq    $0x00000001ffffffff, %r12 ;        \
        sbbq    32+P2, %r12 ;                   \
        shldq   $3, %r11, %r12 ;                    \
        shldq   $3, %r10, %r11 ;                    \
        shldq   $3, %r9, %r10 ;                     \
        shldq   $3, %r8, %r9 ;                      \
        shlq    $3, %r8 ;                          \
        movl    $3, %ecx ;                         \
        movq    P1, %rax ;                      \
        mulq    %rcx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rbx, %rbx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rbx, %rbx ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rbx, %rbx ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rcx;                            \
        addq    %rax, %r12 ;                       \
        leaq    0x1(%r12), %rax ;                  \
        movq    $0x1000003d1, %rcx ;                \
        mulq    %rcx;                            \
        xorl    %ebx, %ebx ;                       \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        adcq    %rbx, %r10 ;                        \
        adcq    %rbx, %r11 ;                        \
        cmovbq  %rbx, %rcx ;                        \
        subq    %rcx, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    %rbx, %r9 ;                         \
        movq    %r9, 8+P0 ;                      \
        sbbq    %rbx, %r10 ;                        \
        movq    %r10, 16+P0 ;                    \
        sbbq    %rbx, %r11 ;                        \
        movq    %r11, 24+P0 ;                    \

// P0 = 4 * P1 - P2 with 5-digit P1, 4-digit P2 and result.
// This is done by direct subtraction of P2 since the method
// in bignum_cmul_p256k1 etc. for quotient estimation still
// works when the value to be reduced is negative, as
// long as it is  > -p_256k1, which is the case here.

#define cmsub41_p256k1(P0,P1,P2)                \
        movq    32+P1, %r12 ;                   \
        movq    24+P1, %r11 ;                   \
        shldq   $2, %r11, %r12 ;                    \
        movq    16+P1, %r10 ;                   \
        shldq   $2, %r10, %r11 ;                    \
        movq    8+P1, %r9 ;                     \
        shldq   $2, %r9, %r10 ;                     \
        movq    P1, %r8 ;                       \
        shldq   $2, %r8, %r9 ;                      \
        shlq    $2, %r8 ;                          \
        subq    P2, %r8 ;                       \
        sbbq    8+P2, %r9 ;                     \
        sbbq    16+P2, %r10 ;                   \
        sbbq    24+P2, %r11 ;                   \
        sbbq    $0, %r12 ;                         \
        leaq    0x1(%r12), %rax ;                  \
        movq    $0x1000003d1, %rcx ;                \
        mulq    %rcx;                            \
        xorq    %rbx, %rbx ;                       \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        adcq    $0x0, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        cmovbq  %rbx, %rcx ;                        \
        subq    %rcx, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    %rbx, %r9 ;                         \
        movq    %r9, 8+P0 ;                      \
        sbbq    %rbx, %r10 ;                        \
        movq    %r10, 16+P0 ;                    \
        sbbq    %rbx, %r11 ;                        \
        movq    %r11, 24+P0 ;                    \

S2N_BN_SYMBOL(secp256k1_jdouble_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save registers and make room on stack for temporary variables

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

// Main sequence of operations

        // y_2 = y^2

        sqr_p256k1(y_2,y_1)

        // x_2 = x^2

        sqr_p256k1(x_2,x_1)

        // tmp = 2 * y_1 (in 4 words but not fully normalized)

        weakdouble_p256k1(tmp,y_1)

        // xy2 = x * y^2 (5-digit partially reduced)
        // x_4 = x^4 (5-digit partially reduced)

        roughmul_p256k1(xy2,x_1,y_2)
        roughsqr_p256k1(x_4,x_2)

        // z_3 = 2 * y_1 * z_1

        mul_p256k1(z_3,z_1,tmp)

        // d = 12 * xy2 - 9 * x_4

        cmsub_p256k1(d,12,xy2,9,x_4)

        // y4 = y2^2 (5-digit partially reduced)

        roughsqr_p256k1(y_4,y_2)

        // dx2 = d * x_2 (5-digit partially reduced)

        roughmul_p256k1(dx2,x_2,d)

        // x_3 = 4 * xy2 - d

        cmsub41_p256k1(x_3,xy2,d)

        // y_3 = 3 * dx2 - 8 * y_4

        cmsub38_p256k1(y_3,dx2,y_4)

// Restore stack and registers

        CFI_INC_RSP(NSPACE)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(secp256k1_jdouble_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
