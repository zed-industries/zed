// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jdouble_alt(uint64_t p3[static 27],
//                                 const uint64_t p1[static 27]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// It is assumed that all coordinates of the input point are fully
// reduced mod p_521 and that the z coordinate is not zero.
//
// Standard x86-64 ABI: RDI = p3, RSI = p1
// Microsoft x64 ABI:   RCX = p3, RDX = p1
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jdouble_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jdouble_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jdouble_alt)
        .text

// Size of individual field elements

#define NUMSIZE 72

// Stable homes for input arguments during main code sequence
// This is actually where they come in anyway and they stay there.

#define input_z %rdi
#define input_x %rsi

// Pointer-offset pairs for inputs and outputs

#define x_1 0(input_x)
#define y_1 NUMSIZE(input_x)
#define z_1 (2*NUMSIZE)(input_x)

#define x_3 0(input_z)
#define y_3 NUMSIZE(input_z)
#define z_3 (2*NUMSIZE)(input_z)

// Pointer-offset pairs for temporaries, with some aliasing
// The tmp field is internal storage for field mul and sqr.
// NSPACE is the total stack needed for these temporaries

#define z2 (NUMSIZE*0)(%rsp)
#define y2 (NUMSIZE*1)(%rsp)
#define x2p (NUMSIZE*2)(%rsp)
#define xy2 (NUMSIZE*3)(%rsp)

#define y4 (NUMSIZE*4)(%rsp)
#define t2 (NUMSIZE*4)(%rsp)

#define dx2 (NUMSIZE*5)(%rsp)
#define t1 (NUMSIZE*5)(%rsp)

#define d (NUMSIZE*6)(%rsp)
#define x4p (NUMSIZE*6)(%rsp)

#define tmp (NUMSIZE*7)(%rsp)

#define NSPACE 576

// Corresponds to bignum_mul_p521_alt except temp storage location

#define mul_p521(P0,P1,P2)                      \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, 504(%rsp) ;                 \
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
        adcq    %r11, %r11 ;                       \
        movq    %r9, 512(%rsp) ;                  \
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
        movq    %r10, 520(%rsp) ;                 \
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
        movq    %r11, 528(%rsp) ;                 \
        xorq    %r14, %r14 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x20+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
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
        movq    0x20+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    %r12, 536(%rsp) ;                 \
        xorq    %r15, %r15 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x28+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x20+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    %r13, 544(%rsp) ;                 \
        xorq    %r8, %r8 ;                         \
        movq    P1, %rax ;                      \
        mulq     0x30+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    %r8, %r8 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq     0x28+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x20+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x28+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x30+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    %r14, 552(%rsp) ;                 \
        xorq    %r9, %r9 ;                         \
        movq    P1, %rax ;                      \
        mulq     0x38+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    %r9, %r9 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq     0x30+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x20+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x28+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x30+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x38+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    %r15, 560(%rsp) ;                 \
        xorq    %r10, %r10 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x40+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    %r10, %r10 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x38+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    %r8, 568(%rsp) ;                  \
        xorq    %r11, %r11 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x40+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    %r11, %r11 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    %r12, %r12 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    %r13, %r13 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        xorq    %r8, %r8 ;                         \
        movq    0x30+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    %r8, %r8 ;                         \
        movq    0x38+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x40+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x38+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    0x40+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    0x40+P1, %rax ;                 \
        imulq   0x40+P2, %rax ;                 \
        addq    %r8, %rax ;                        \
        movq    568(%rsp), %r8 ;                  \
        movq    %r8, %rdx ;                        \
        andq    $0x1ff, %rdx ;                     \
        shrdq   $0x9, %r9, %r8 ;                    \
        shrdq   $0x9, %r10, %r9 ;                   \
        shrdq   $0x9, %r11, %r10 ;                  \
        shrdq   $0x9, %r12, %r11 ;                  \
        shrdq   $0x9, %r13, %r12 ;                  \
        shrdq   $0x9, %r14, %r13 ;                  \
        shrdq   $0x9, %r15, %r14 ;                  \
        shrdq   $0x9, %rax, %r15 ;                  \
        shrq    $0x9, %rax ;                       \
        addq    %rax, %rdx ;                       \
        stc;                                    \
        adcq    504(%rsp), %r8 ;                  \
        adcq    512(%rsp), %r9 ;                  \
        adcq    520(%rsp), %r10 ;                 \
        adcq    528(%rsp), %r11 ;                 \
        adcq    536(%rsp), %r12 ;                 \
        adcq    544(%rsp), %r13 ;                 \
        adcq    552(%rsp), %r14 ;                 \
        adcq    560(%rsp), %r15 ;                 \
        adcq    $0xfffffffffffffe00, %rdx ;        \
        cmc;                                    \
        sbbq    $0x0, %r8 ;                        \
        movq    %r8, P0 ;                       \
        sbbq    $0x0, %r9 ;                        \
        movq    %r9, 0x8+P0 ;                   \
        sbbq    $0x0, %r10 ;                       \
        movq    %r10, 0x10+P0 ;                 \
        sbbq    $0x0, %r11 ;                       \
        movq    %r11, 0x18+P0 ;                 \
        sbbq    $0x0, %r12 ;                       \
        movq    %r12, 0x20+P0 ;                 \
        sbbq    $0x0, %r13 ;                       \
        movq    %r13, 0x28+P0 ;                 \
        sbbq    $0x0, %r14 ;                       \
        movq    %r14, 0x30+P0 ;                 \
        sbbq    $0x0, %r15 ;                       \
        movq    %r15, 0x38+P0 ;                 \
        sbbq    $0x0, %rdx ;                       \
        andq    $0x1ff, %rdx ;                     \
        movq    %rdx, 0x40+P0

// Corresponds to bignum_sqr_p521_alt except temp storage location

#define sqr_p521(P0,P1)                         \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, 504(%rsp) ;                 \
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
        movq    %r9, 512(%rsp) ;                  \
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
        movq    %r10, 520(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x18+P1;            \
        xorq    %r13, %r13 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r13, %r13 ;                       \
        addq    %rbx, %r11 ;                       \
        adcq    %rcx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    %r11, 528(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x20+P1;            \
        xorq    %r14, %r14 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r14 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r14, %r14 ;                       \
        addq    %rbx, %r12 ;                       \
        adcq    %rcx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    %r12, 536(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x28+P1;            \
        xorq    %r15, %r15 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x20+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r15 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r15, %r15 ;                       \
        addq    %rbx, %r13 ;                       \
        adcq    %rcx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    %r13, 544(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x30+P1;            \
        xorq    %r8, %r8 ;                         \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x20+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r8 ;                        \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r8, %r8 ;                         \
        addq    %rbx, %r14 ;                       \
        adcq    %rcx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    %r14, 552(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x38+P1;            \
        xorq    %r9, %r9 ;                         \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r9 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r9 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x20+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r9 ;                        \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r9, %r9 ;                         \
        addq    %rbx, %r15 ;                       \
        adcq    %rcx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    %r15, 560(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x40+P1;            \
        xorq    %r10, %r10 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r10 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r10 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r10 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r10, %r10 ;                       \
        addq    %rbx, %r8 ;                        \
        adcq    %rcx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    %r8, 568(%rsp) ;                  \
        movq    0x8+P1, %rax ;                  \
        mulq     0x40+P1;            \
        xorq    %r11, %r11 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r11 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r11, %r11 ;                       \
        addq    %rbx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x40+P1;            \
        xorq    %r12, %r12 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r12 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r12, %r12 ;                       \
        addq    %rbx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x40+P1;            \
        xorq    %r13, %r13 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r13, %r13 ;                       \
        addq    %rbx, %r11 ;                       \
        adcq    %rcx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x40+P1;            \
        xorq    %r14, %r14 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r14 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r14, %r14 ;                       \
        addq    %rbx, %r12 ;                       \
        adcq    %rcx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x40+P1;            \
        xorq    %r15, %r15 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rcx ;                       \
        adcq    $0x0, %r15 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %r15, %r15 ;                       \
        addq    %rbx, %r13 ;                       \
        adcq    %rcx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        xorq    %r8, %r8 ;                         \
        movq    0x38+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x30+P1, %rax ;                 \
        mulq     0x40+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r8 ;                        \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x38+P1, %rax ;                 \
        mulq     0x40+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    0x40+P1, %rax ;                 \
        imulq   %rax, %rax ;                       \
        addq    %r8, %rax ;                        \
        movq    568(%rsp), %r8 ;                  \
        movq    %r8, %rdx ;                        \
        andq    $0x1ff, %rdx ;                     \
        shrdq   $0x9, %r9, %r8 ;                    \
        shrdq   $0x9, %r10, %r9 ;                   \
        shrdq   $0x9, %r11, %r10 ;                  \
        shrdq   $0x9, %r12, %r11 ;                  \
        shrdq   $0x9, %r13, %r12 ;                  \
        shrdq   $0x9, %r14, %r13 ;                  \
        shrdq   $0x9, %r15, %r14 ;                  \
        shrdq   $0x9, %rax, %r15 ;                  \
        shrq    $0x9, %rax ;                       \
        addq    %rax, %rdx ;                       \
        stc;                                    \
        adcq    504(%rsp), %r8 ;                  \
        adcq    512(%rsp), %r9 ;                  \
        adcq    520(%rsp), %r10 ;                 \
        adcq    528(%rsp), %r11 ;                 \
        adcq    536(%rsp), %r12 ;                 \
        adcq    544(%rsp), %r13 ;                 \
        adcq    552(%rsp), %r14 ;                 \
        adcq    560(%rsp), %r15 ;                 \
        adcq    $0xfffffffffffffe00, %rdx ;        \
        cmc;                                    \
        sbbq    $0x0, %r8 ;                        \
        movq    %r8, P0 ;                       \
        sbbq    $0x0, %r9 ;                        \
        movq    %r9, 0x8+P0 ;                   \
        sbbq    $0x0, %r10 ;                       \
        movq    %r10, 0x10+P0 ;                 \
        sbbq    $0x0, %r11 ;                       \
        movq    %r11, 0x18+P0 ;                 \
        sbbq    $0x0, %r12 ;                       \
        movq    %r12, 0x20+P0 ;                 \
        sbbq    $0x0, %r13 ;                       \
        movq    %r13, 0x28+P0 ;                 \
        sbbq    $0x0, %r14 ;                       \
        movq    %r14, 0x30+P0 ;                 \
        sbbq    $0x0, %r15 ;                       \
        movq    %r15, 0x38+P0 ;                 \
        sbbq    $0x0, %rdx ;                       \
        andq    $0x1ff, %rdx ;                     \
        movq    %rdx, 0x40+P0 ;                 \

// Corresponds exactly to bignum_add_p521

#define add_p521(P0,P1,P2)                      \
        stc;                                    \
        movq    P1, %rax ;                       \
        adcq    P2, %rax ;                       \
        movq    0x8+P1, %rbx ;                   \
        adcq    0x8+P2, %rbx ;                   \
        movq    0x10+P1, %r8 ;                   \
        adcq    0x10+P2, %r8 ;                   \
        movq    0x18+P1, %r9 ;                   \
        adcq    0x18+P2, %r9 ;                   \
        movq    0x20+P1, %r10 ;                  \
        adcq    0x20+P2, %r10 ;                  \
        movq    0x28+P1, %r11 ;                  \
        adcq    0x28+P2, %r11 ;                  \
        movq    0x30+P1, %r12 ;                  \
        adcq    0x30+P2, %r12 ;                  \
        movq    0x38+P1, %r13 ;                  \
        adcq    0x38+P2, %r13 ;                  \
        movq    0x40+P1, %r14 ;                  \
        adcq    0x40+P2, %r14 ;                  \
        movq    $0x200, %rdx ;                      \
        andq    %r14, %rdx ;                        \
        cmpq    $0x200, %rdx ;                      \
        sbbq    $0x0, %rax ;                        \
        movq    %rax, P0 ;                       \
        sbbq    $0x0, %rbx ;                        \
        movq    %rbx, 0x8+P0 ;                   \
        sbbq    $0x0, %r8 ;                         \
        movq    %r8, 0x10+P0 ;                   \
        sbbq    $0x0, %r9 ;                         \
        movq    %r9, 0x18+P0 ;                   \
        sbbq    $0x0, %r10 ;                        \
        movq    %r10, 0x20+P0 ;                  \
        sbbq    $0x0, %r11 ;                        \
        movq    %r11, 0x28+P0 ;                  \
        sbbq    $0x0, %r12 ;                        \
        movq    %r12, 0x30+P0 ;                  \
        sbbq    $0x0, %r13 ;                        \
        movq    %r13, 0x38+P0 ;                  \
        sbbq    %rdx, %r14 ;                        \
        movq    %r14, 0x40+P0

// Corresponds exactly to bignum_sub_p521

#define sub_p521(P0,P1,P2)                      \
        movq    P1, %rax ;                       \
        subq    P2, %rax ;                       \
        movq    0x8+P1, %rdx ;                   \
        sbbq    0x8+P2, %rdx ;                   \
        movq    0x10+P1, %r8 ;                   \
        sbbq    0x10+P2, %r8 ;                   \
        movq    0x18+P1, %r9 ;                   \
        sbbq    0x18+P2, %r9 ;                   \
        movq    0x20+P1, %r10 ;                  \
        sbbq    0x20+P2, %r10 ;                  \
        movq    0x28+P1, %r11 ;                  \
        sbbq    0x28+P2, %r11 ;                  \
        movq    0x30+P1, %r12 ;                  \
        sbbq    0x30+P2, %r12 ;                  \
        movq    0x38+P1, %r13 ;                  \
        sbbq    0x38+P2, %r13 ;                  \
        movq    0x40+P1, %r14 ;                  \
        sbbq    0x40+P2, %r14 ;                  \
        sbbq    $0x0, %rax ;                        \
        movq    %rax, P0 ;                       \
        sbbq    $0x0, %rdx ;                        \
        movq    %rdx, 0x8+P0 ;                   \
        sbbq    $0x0, %r8 ;                         \
        movq    %r8, 0x10+P0 ;                   \
        sbbq    $0x0, %r9 ;                         \
        movq    %r9, 0x18+P0 ;                   \
        sbbq    $0x0, %r10 ;                        \
        movq    %r10, 0x20+P0 ;                  \
        sbbq    $0x0, %r11 ;                        \
        movq    %r11, 0x28+P0 ;                  \
        sbbq    $0x0, %r12 ;                        \
        movq    %r12, 0x30+P0 ;                  \
        sbbq    $0x0, %r13 ;                        \
        movq    %r13, 0x38+P0 ;                  \
        sbbq    $0x0, %r14 ;                        \
        andq    $0x1ff, %r14 ;                      \
        movq    %r14, 0x40+P0

// Weak multiplication not fully reducing

#define weakmul_p521(P0,P1,P2)                  \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, 504(%rsp) ;                 \
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
        adcq    %r11, %r11 ;                       \
        movq    %r9, 512(%rsp) ;                  \
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
        movq    %r10, 520(%rsp) ;                 \
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
        movq    %r11, 528(%rsp) ;                 \
        xorq    %r14, %r14 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x20+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
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
        movq    0x20+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    %r12, 536(%rsp) ;                 \
        xorq    %r15, %r15 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x28+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x20+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    %r13, 544(%rsp) ;                 \
        xorq    %r8, %r8 ;                         \
        movq    P1, %rax ;                      \
        mulq     0x30+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    %r8, %r8 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq     0x28+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x20+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x28+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x30+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    %r14, 552(%rsp) ;                 \
        xorq    %r9, %r9 ;                         \
        movq    P1, %rax ;                      \
        mulq     0x38+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    %r9, %r9 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq     0x30+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x20+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x28+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x30+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x38+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    %r15, 560(%rsp) ;                 \
        xorq    %r10, %r10 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x40+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    %r10, %r10 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x38+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    %r8, 568(%rsp) ;                  \
        xorq    %r11, %r11 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x40+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    %r11, %r11 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    %r12, %r12 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    %r13, %r13 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x20+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x40+P1, %rax ;                 \
        mulq     0x28+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        xorq    %r8, %r8 ;                         \
        movq    0x30+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    %r8, %r8 ;                         \
        movq    0x38+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x40+P1, %rax ;                 \
        mulq     0x30+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x38+P1, %rax ;                 \
        mulq     0x40+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    0x40+P1, %rax ;                 \
        mulq     0x38+P2;            \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    0x40+P1, %rax ;                 \
        imulq   0x40+P2, %rax ;                 \
        addq    %r8, %rax ;                        \
        movq    568(%rsp), %r8 ;                  \
        movq    %r8, %rdx ;                        \
        andq    $0x1ff, %rdx ;                     \
        shrdq   $0x9, %r9, %r8 ;                    \
        shrdq   $0x9, %r10, %r9 ;                   \
        shrdq   $0x9, %r11, %r10 ;                  \
        shrdq   $0x9, %r12, %r11 ;                  \
        shrdq   $0x9, %r13, %r12 ;                  \
        shrdq   $0x9, %r14, %r13 ;                  \
        shrdq   $0x9, %r15, %r14 ;                  \
        shrdq   $0x9, %rax, %r15 ;                  \
        shrq    $0x9, %rax ;                       \
        addq    %rax, %rdx ;                       \
        addq    504(%rsp), %r8 ;                  \
        adcq    512(%rsp), %r9 ;                  \
        adcq    520(%rsp), %r10 ;                 \
        adcq    528(%rsp), %r11 ;                 \
        adcq    536(%rsp), %r12 ;                 \
        adcq    544(%rsp), %r13 ;                 \
        adcq    552(%rsp), %r14 ;                 \
        adcq    560(%rsp), %r15 ;                 \
        adcq    $0, %rdx ;                         \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0 ;                 \
        movq    %r12, 0x20+P0 ;                 \
        movq    %r13, 0x28+P0 ;                 \
        movq    %r14, 0x30+P0 ;                 \
        movq    %r15, 0x38+P0 ;                 \
        movq    %rdx, 0x40+P0

// P0 = C * P1 - D * P2 == C * P1 + D * (p_521 - P2)

#define cmsub_p521(P0,C,P1,D,P2)                \
        movq    $D, %rcx ;                         \
        movq    P2, %rax ;                      \
        notq    %rax;                            \
        mulq    %rcx;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        movq    8+P2, %rax ;                    \
        notq    %rax;                            \
        mulq    %rcx;                            \
        xorl    %r10d, %r10d ;                     \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        movq    16+P2, %rax ;                   \
        notq    %rax;                            \
        mulq    %rcx;                            \
        xorl    %r11d, %r11d ;                     \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        movq    24+P2, %rax ;                   \
        notq    %rax;                            \
        mulq    %rcx;                            \
        xorl    %r12d, %r12d ;                     \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        movq    32+P2, %rax ;                   \
        notq    %rax;                            \
        mulq    %rcx;                            \
        xorl    %r13d, %r13d ;                     \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        movq    40+P2, %rax ;                   \
        notq    %rax;                            \
        mulq    %rcx;                            \
        xorl    %r14d, %r14d ;                     \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        movq    48+P2, %rax ;                   \
        notq    %rax;                            \
        mulq    %rcx;                            \
        xorl    %r15d, %r15d ;                     \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        movq    56+P2, %rax ;                   \
        notq    %rax;                            \
        mulq    %rcx;                            \
        xorl    %ebx, %ebx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %rbx ;                       \
        movq    64+P2, %rax ;                   \
        xorq    $0x1FF, %rax ;                     \
        imulq   %rcx, %rax ;                       \
        addq    %rax, %rbx ;                       \
        xorl    %eax, %eax ;                       \
        movl    $C, %ecx ;                         \
        movq    P1, %rax ;                      \
        mulq    %rcx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rbp, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %rbx ;                       \
        movq    0x40+P1, %rax ;                 \
        imulq   %rcx, %rax ;                       \
        addq    %rax, %rbx ;                       \
        movq    %r9, %rax ;                        \
        andq    %r10, %rax ;                       \
        andq    %r11, %rax ;                       \
        andq    %r12, %rax ;                       \
        andq    %r13, %rax ;                       \
        andq    %r14, %rax ;                       \
        andq    %r15, %rax ;                       \
        movq    %rbx, %rdx ;                       \
        shrq    $9, %rdx ;                         \
        orq     $~0x1FF, %rbx ;                    \
        leaq    1(%rdx), %rcx ;                   \
        addq    %r8, %rcx ;                        \
        movl    $0, %ecx ;                         \
        adcq    %rcx, %rax ;                       \
        movq    %rbx, %rax ;                       \
        adcq    %rcx, %rax ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    %r8, P0 ;                       \
        adcq    %rcx, %r9 ;                        \
        movq    %r9, 8+P0 ;                     \
        adcq    %rcx, %r10 ;                       \
        movq    %r10, 16+P0 ;                   \
        adcq    %rcx, %r11 ;                       \
        movq    %r11, 24+P0 ;                   \
        adcq    %rcx, %r12 ;                       \
        movq    %r12, 32+P0 ;                   \
        adcq    %rcx, %r13 ;                       \
        movq    %r13, 40+P0 ;                   \
        adcq    %rcx, %r14 ;                       \
        movq    %r14, 48+P0 ;                   \
        adcq    %rcx, %r15 ;                       \
        movq    %r15, 56+P0 ;                   \
        adcq    %rcx, %rbx ;                       \
        andq    $0x1FF, %rbx ;                     \
        movq    %rbx, 64+P0

// P0 = 3 * P1 - 8 * P2 == 3 * P1 + 8 * (p_521 - P2)

#define cmsub38_p521(P0,P1,P2)                  \
        movq    64+P2, %rbx ;                   \
        xorq    $0x1FF, %rbx ;                     \
        movq    56+P2, %r15 ;                   \
        notq    %r15;                            \
        shldq   $3, %r15, %rbx ;                    \
        movq    48+P2, %r14 ;                   \
        notq    %r14;                            \
        shldq   $3, %r14, %r15 ;                    \
        movq    40+P2, %r13 ;                   \
        notq    %r13;                            \
        shldq   $3, %r13, %r14 ;                    \
        movq    32+P2, %r12 ;                   \
        notq    %r12;                            \
        shldq   $3, %r12, %r13 ;                    \
        movq    24+P2, %r11 ;                   \
        notq    %r11;                            \
        shldq   $3, %r11, %r12 ;                    \
        movq    16+P2, %r10 ;                   \
        notq    %r10;                            \
        shldq   $3, %r10, %r11 ;                    \
        movq    8+P2, %r9 ;                     \
        notq    %r9;                             \
        shldq   $3, %r9, %r10 ;                     \
        movq    P2, %r8 ;                       \
        notq    %r8;                             \
        shldq   $3, %r8, %r9 ;                      \
        shlq    $3, %r8 ;                          \
        movl    $3, %ecx ;                         \
        movq    P1, %rax ;                      \
        mulq    %rcx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rbp, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x38+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %rbx ;                       \
        movq    0x40+P1, %rax ;                 \
        imulq   %rcx, %rax ;                       \
        addq    %rax, %rbx ;                       \
        movq    %r9, %rax ;                        \
        andq    %r10, %rax ;                       \
        andq    %r11, %rax ;                       \
        andq    %r12, %rax ;                       \
        andq    %r13, %rax ;                       \
        andq    %r14, %rax ;                       \
        andq    %r15, %rax ;                       \
        movq    %rbx, %rdx ;                       \
        shrq    $9, %rdx ;                         \
        orq     $~0x1FF, %rbx ;                    \
        leaq    1(%rdx), %rcx ;                   \
        addq    %r8, %rcx ;                        \
        movl    $0, %ecx ;                         \
        adcq    %rcx, %rax ;                       \
        movq    %rbx, %rax ;                       \
        adcq    %rcx, %rax ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    %r8, P0 ;                       \
        adcq    %rcx, %r9 ;                        \
        movq    %r9, 8+P0 ;                     \
        adcq    %rcx, %r10 ;                       \
        movq    %r10, 16+P0 ;                   \
        adcq    %rcx, %r11 ;                       \
        movq    %r11, 24+P0 ;                   \
        adcq    %rcx, %r12 ;                       \
        movq    %r12, 32+P0 ;                   \
        adcq    %rcx, %r13 ;                       \
        movq    %r13, 40+P0 ;                   \
        adcq    %rcx, %r14 ;                       \
        movq    %r14, 48+P0 ;                   \
        adcq    %rcx, %r15 ;                       \
        movq    %r15, 56+P0 ;                   \
        adcq    %rcx, %rbx ;                       \
        andq    $0x1FF, %rbx ;                     \
        movq    %rbx, 64+P0

// P0 = 4 * P1 - P2 = 4 * P1 + (p_521 - P2)

#define cmsub41_p521(P0,P1,P2)                  \
        movq    64+P1, %rbx ;                   \
        movq    56+P1, %r15 ;                   \
        shldq   $2, %r15, %rbx ;                    \
        movq    48+P1, %r14 ;                   \
        shldq   $2, %r14, %r15 ;                    \
        movq    40+P1, %r13 ;                   \
        shldq   $2, %r13, %r14 ;                    \
        movq    32+P1, %r12 ;                   \
        shldq   $2, %r12, %r13 ;                    \
        movq    24+P1, %r11 ;                   \
        shldq   $2, %r11, %r12 ;                    \
        movq    16+P1, %r10 ;                   \
        shldq   $2, %r10, %r11 ;                    \
        movq    8+P1, %r9 ;                     \
        shldq   $2, %r9, %r10 ;                     \
        movq    P1, %r8 ;                       \
        shldq   $2, %r8, %r9 ;                      \
        shlq    $2, %r8 ;                          \
        movq    64+P2, %rcx ;                   \
        xorq    $0x1FF, %rcx ;                     \
        movq    P2, %rax ;                      \
        notq    %rax;                            \
        addq    %rax, %r8 ;                        \
        movq    8+P2, %rax ;                    \
        notq    %rax;                            \
        adcq    %rax, %r9 ;                        \
        movq    16+P2, %rax ;                   \
        notq    %rax;                            \
        adcq    %rax, %r10 ;                       \
        movq    24+P2, %rax ;                   \
        notq    %rax;                            \
        adcq    %rax, %r11 ;                       \
        movq    32+P2, %rax ;                   \
        notq    %rax;                            \
        adcq    %rax, %r12 ;                       \
        movq    40+P2, %rax ;                   \
        notq    %rax;                            \
        adcq    %rax, %r13 ;                       \
        movq    48+P2, %rax ;                   \
        notq    %rax;                            \
        adcq    %rax, %r14 ;                       \
        movq    56+P2, %rax ;                   \
        notq    %rax;                            \
        adcq    %rax, %r15 ;                       \
        adcq    %rcx, %rbx ;                       \
        movq    %r9, %rax ;                        \
        andq    %r10, %rax ;                       \
        andq    %r11, %rax ;                       \
        andq    %r12, %rax ;                       \
        andq    %r13, %rax ;                       \
        andq    %r14, %rax ;                       \
        andq    %r15, %rax ;                       \
        movq    %rbx, %rdx ;                       \
        shrq    $9, %rdx ;                         \
        orq     $~0x1FF, %rbx ;                    \
        leaq    1(%rdx), %rcx ;                   \
        addq    %r8, %rcx ;                        \
        movl    $0, %ecx ;                         \
        adcq    %rcx, %rax ;                       \
        movq    %rbx, %rax ;                       \
        adcq    %rcx, %rax ;                       \
        adcq    %rdx, %r8 ;                        \
        movq    %r8, P0 ;                       \
        adcq    %rcx, %r9 ;                        \
        movq    %r9, 8+P0 ;                     \
        adcq    %rcx, %r10 ;                       \
        movq    %r10, 16+P0 ;                   \
        adcq    %rcx, %r11 ;                       \
        movq    %r11, 24+P0 ;                   \
        adcq    %rcx, %r12 ;                       \
        movq    %r12, 32+P0 ;                   \
        adcq    %rcx, %r13 ;                       \
        movq    %r13, 40+P0 ;                   \
        adcq    %rcx, %r14 ;                       \
        movq    %r14, 48+P0 ;                   \
        adcq    %rcx, %r15 ;                       \
        movq    %r15, 56+P0 ;                   \
        adcq    %rcx, %rbx ;                       \
        andq    $0x1FF, %rbx ;                     \
        movq    %rbx, 64+P0

S2N_BN_SYMBOL(p521_jdouble_alt):
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
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

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
        weakmul_p521(xy2,x_1,y2)

// t2 = (y + z)^2

        sqr_p521(t2,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_p521(d,12,xy2,9,x4p)
        sub_p521(t1,t2,z2)

// y4 = y^4

        sqr_p521(y4,y2)

// z_3' = 2 * y * z
// dx2 = d * x2p

        sub_p521(z_3,t1,y2)
        weakmul_p521(dx2,d,x2p)

// x' = 4 * xy2 - d

        cmsub41_p521(x_3,xy2,d)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p521(y_3,dx2,y4)

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

S2N_BN_SIZE_DIRECTIVE(p521_jdouble_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
