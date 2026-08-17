// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jadd_alt(uint64_t p3[static 27], const uint64_t p1[static 27],
//                              const uint64_t p2[static 27]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// It is assumed that all coordinates of the input points p1 and p2 are
// fully reduced mod p_521, that both z coordinates are nonzero and
// that neither p1 =~= p2 or p1 =~= -p2, where "=~=" means "represents
// the same affine point as".
//
// Standard x86-64 ABI: RDI = p3, RSI = p1, RDX = p2
// Microsoft x64 ABI:   RCX = p3, RDX = p1, R8 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jadd_alt)
        .text

// Size of individual field elements

#define NUMSIZE 72

// Stable homes for input arguments during main code sequence
// These are where they arrive except for input_y, initially in %rdx

#define input_z %rdi
#define input_x %rsi
#define input_y %rcx

// Pointer-offset pairs for inputs and outputs

#define x_1 0(input_x)
#define y_1 NUMSIZE(input_x)
#define z_1 (2*NUMSIZE)(input_x)

#define x_2 0(input_y)
#define y_2 NUMSIZE(input_y)
#define z_2 (2*NUMSIZE)(input_y)

#define x_3 0(input_z)
#define y_3 NUMSIZE(input_z)
#define z_3 (2*NUMSIZE)(input_z)

// Pointer-offset pairs for temporaries, with some aliasing
// The tmp field is internal storage for field mul and sqr.
// NSPACE is the total stack needed for these temporaries

#define z1sq (NUMSIZE*0)(%rsp)
#define ww (NUMSIZE*0)(%rsp)
#define resx (NUMSIZE*0)(%rsp)

#define yd (NUMSIZE*1)(%rsp)
#define y2a (NUMSIZE*1)(%rsp)

#define x2a (NUMSIZE*2)(%rsp)
#define zzx2 (NUMSIZE*2)(%rsp)

#define zz (NUMSIZE*3)(%rsp)
#define t1 (NUMSIZE*3)(%rsp)

#define t2 (NUMSIZE*4)(%rsp)
#define x1a (NUMSIZE*4)(%rsp)
#define zzx1 (NUMSIZE*4)(%rsp)
#define resy (NUMSIZE*4)(%rsp)

#define xd (NUMSIZE*5)(%rsp)
#define z2sq (NUMSIZE*5)(%rsp)
#define resz (NUMSIZE*5)(%rsp)

#define y1a (NUMSIZE*6)(%rsp)

#define tmp (NUMSIZE*7)(%rsp)

#define NSPACE NUMSIZE*8

// Corresponds exactly to bignum_mul_p521_alt except temp storage

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

// Corresponds to bignum_sqr_p521_alt except %rbp is used
// in place of %rcx and tmp is the temp storage location

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
        movq    %rdx, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r13, %r13 ;                       \
        addq    %rbx, %r11 ;                       \
        adcq    %rbp, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    %r11, 528(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x20+P1;            \
        xorq    %r14, %r14 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r14 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r14, %r14 ;                       \
        addq    %rbx, %r12 ;                       \
        adcq    %rbp, %r13 ;                       \
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
        movq    %rdx, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x20+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r15 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r15, %r15 ;                       \
        addq    %rbx, %r13 ;                       \
        adcq    %rbp, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    %r13, 544(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x30+P1;            \
        xorq    %r8, %r8 ;                         \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x20+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r8 ;                        \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r8, %r8 ;                         \
        addq    %rbx, %r14 ;                       \
        adcq    %rbp, %r15 ;                       \
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
        movq    %rdx, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r9 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r9 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x20+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r9 ;                        \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r9, %r9 ;                         \
        addq    %rbx, %r15 ;                       \
        adcq    %rbp, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    %r15, 560(%rsp) ;                 \
        movq    P1, %rax ;                      \
        mulq     0x40+P1;            \
        xorq    %r10, %r10 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r10 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r10 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r10 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r10, %r10 ;                       \
        addq    %rbx, %r8 ;                        \
        adcq    %rbp, %r9 ;                        \
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
        movq    %rdx, %rbp ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r11 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r11, %r11 ;                       \
        addq    %rbx, %r9 ;                        \
        adcq    %rbp, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x40+P1;            \
        xorq    %r12, %r12 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rbp ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r12 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r12, %r12 ;                       \
        addq    %rbx, %r10 ;                       \
        adcq    %rbp, %r11 ;                       \
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
        movq    %rdx, %rbp ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x30+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r13, %r13 ;                       \
        addq    %rbx, %r11 ;                       \
        adcq    %rbp, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x40+P1;            \
        xorq    %r14, %r14 ;                       \
        movq    %rax, %rbx ;                       \
        movq    %rdx, %rbp ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r14 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r14, %r14 ;                       \
        addq    %rbx, %r12 ;                       \
        adcq    %rbp, %r13 ;                       \
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
        movq    %rdx, %rbp ;                       \
        movq    0x30+P1, %rax ;                 \
        mulq     0x38+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        adcq    $0x0, %r15 ;                       \
        addq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcq    %r15, %r15 ;                       \
        addq    %rbx, %r13 ;                       \
        adcq    %rbp, %r14 ;                       \
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
        movq    %rdx, 0x40+P0

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

// Additional macros to help with final multiplexing

#define load9(r0,r1,r2,r3,r4,r5,r6,r7,ra,P)     \
        movq    P, r0 ;                        \
        movq    8+P, r1 ;                      \
        movq    16+P, r2 ;                     \
        movq    24+P, r3 ;                     \
        movq    32+P, r4 ;                     \
        movq    40+P, r5 ;                     \
        movq    48+P, r6 ;                     \
        movq    56+P, r7 ;                     \
        movq    64+P, ra

#define store9(P,r0,r1,r2,r3,r4,r5,r6,r7,ra)    \
        movq    r0, P ;                        \
        movq    r1, 8+P ;                      \
        movq    r2, 16+P ;                     \
        movq    r3, 24+P ;                     \
        movq    r4, 32+P ;                     \
        movq    r5, 40+P ;                     \
        movq    r6, 48+P ;                     \
        movq    r7, 56+P ;                     \
        movq    ra, 64+P

#define muxload9(r0,r1,r2,r3,r4,r5,r6,r7,ra,P0,P1,P2)   \
        movq    P0, r0 ;                               \
        cmovbq  P1, r0 ;                               \
        cmovnbe P2, r0 ;                               \
        movq    8+P0, r1 ;                             \
        cmovbq  8+P1, r1 ;                             \
        cmovnbe 8+P2, r1 ;                             \
        movq    16+P0, r2 ;                            \
        cmovbq  16+P1, r2 ;                            \
        cmovnbe 16+P2, r2 ;                            \
        movq    24+P0, r3 ;                            \
        cmovbq  24+P1, r3 ;                            \
        cmovnbe 24+P2, r3 ;                            \
        movq    32+P0, r4 ;                            \
        cmovbq  32+P1, r4 ;                            \
        cmovnbe 32+P2, r4 ;                            \
        movq    40+P0, r5 ;                            \
        cmovbq  40+P1, r5 ;                            \
        cmovnbe 40+P2, r5 ;                            \
        movq    48+P0, r6 ;                            \
        cmovbq  48+P1, r6 ;                            \
        cmovnbe 48+P2, r6 ;                            \
        movq    56+P0, r7 ;                            \
        cmovbq  56+P1, r7 ;                            \
        cmovnbe 56+P2, r7 ;                            \
        movq    64+P0, ra ;                            \
        cmovbq  64+P1, ra ;                            \
        cmovnbe 64+P2, ra

#define copy9(P0,P1)                            \
        movq    P1, %rax ;                       \
        movq    %rax, P0 ;                       \
        movq    8+P1, %rax ;                     \
        movq    %rax, 8+P0 ;                     \
        movq    16+P1, %rax ;                    \
        movq    %rax, 16+P0 ;                    \
        movq    24+P1, %rax ;                    \
        movq    %rax, 24+P0 ;                    \
        movq    32+P1, %rax ;                    \
        movq    %rax, 32+P0 ;                    \
        movq    40+P1, %rax ;                    \
        movq    %rax, 40+P0 ;                    \
        movq    48+P1, %rax ;                    \
        movq    %rax, 48+P0 ;                    \
        movq    56+P1, %rax ;                    \
        movq    %rax, 56+P0 ;                    \
        movq    64+P1, %rax ;                    \
        movq    %rax, 64+P0

S2N_BN_SYMBOL(p521_jadd_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save registers and make room on stack for temporary variables

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

// Move the input arguments to stable places (two are already there)

        movq    %rdx, input_y

// Main code, just a sequence of basic field operations

        sqr_p521(z1sq,z_1)
        sqr_p521(z2sq,z_2)

        mul_p521(y1a,z_2,y_1)
        mul_p521(y2a,z_1,y_2)

        mul_p521(x2a,z1sq,x_2)
        mul_p521(x1a,z2sq,x_1)
        mul_p521(y2a,z1sq,y2a)
        mul_p521(y1a,z2sq,y1a)

        sub_p521(xd,x2a,x1a)
        sub_p521(yd,y2a,y1a)

        sqr_p521(zz,xd)
        sqr_p521(ww,yd)

        mul_p521(zzx1,zz,x1a)
        mul_p521(zzx2,zz,x2a)

        sub_p521(resx,ww,zzx1)
        sub_p521(t1,zzx2,zzx1)

        mul_p521(xd,xd,z_1)

        sub_p521(resx,resx,zzx2)

        sub_p521(t2,zzx1,resx)

        mul_p521(t1,t1,y1a)
        mul_p521(resz,xd,z_2)
        mul_p521(t2,yd,t2)

        sub_p521(resy,t2,t1)

// Load in the z coordinates of the inputs to check for P1 = 0 and P2 = 0
// The condition codes get set by a comparison (P2 != 0) - (P1 != 0)
// So "NBE" <=> ~(CF \/ ZF) <=> P1 = 0 /\ ~(P2 = 0)
// and "B"  <=> CF          <=> ~(P1 = 0) /\ P2 = 0
// and "Z"  <=> ZF          <=> (P1 = 0 <=> P2 = 0)

        load9(%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp,z_1)
        orq     %r9, %r8
        orq     %r11, %r10
        orq     %r13, %r12
        orq     %r15, %r14
        orq     %r10, %r8
        orq     %r14, %r12
        orq     %rbp, %r8
        orq     %r12, %r8
        negq    %r8
        sbbq    %rax, %rax

        load9(%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp,z_2)
        orq     %r9, %r8
        orq     %r11, %r10
        orq     %r13, %r12
        orq     %r15, %r14
        orq     %r10, %r8
        orq     %r14, %r12
        orq     %rbp, %r8
        orq     %r12, %r8
        negq    %r8
        sbbq    %rdx, %rdx

        cmpq    %rax, %rdx

// Multiplex the outputs accordingly. Re-store them in resz until there
// are no more loads, so there are no assumptions on input-output aliasing

        muxload9(%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp,resy,y_1,y_2)
        store9(resy,%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp)
        muxload9(%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp,resz,z_1,z_2)
        store9(resz,%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp)
        muxload9(%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp,resx,x_1,x_2)
        store9(x_3,%r8,%r9,%r10,%r11,%r12,%r13,%r14,%r15,%rbp)
        copy9(y_3,resy)
        copy9(z_3,resz)

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

S2N_BN_SIZE_DIRECTIVE(p521_jadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
