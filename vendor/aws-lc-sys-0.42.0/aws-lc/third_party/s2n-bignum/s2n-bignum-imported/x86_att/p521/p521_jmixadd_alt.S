// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jmixadd_alt(uint64_t p3[static 27],
//                                 const uint64_t p1[static 27],
//                                 const uint64_t p2[static 18]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity. It is assumed that
// all the coordinates of the input points p1 and p2 are fully reduced
// mod p_521, that the z coordinate of p1 is nonzero and that neither
// p1 =~= p2 or p1 =~= -p2, where "=~=" means "represents the same affine
// point as".
//
// Standard x86-64 ABI: RDI = p3, RSI = p1, RDX = p2
// Microsoft x64 ABI:   RCX = p3, RDX = p1, R8 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jmixadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jmixadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jmixadd_alt)
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

#define x_3 0(input_z)
#define y_3 NUMSIZE(input_z)
#define z_3 (2*NUMSIZE)(input_z)

// Pointer-offset pairs for temporaries, with some aliasing
// The tmp field is internal storage for field mul and sqr.
// NSPACE is the total stack needed for these temporaries

#define zp2 (NUMSIZE*0)(%rsp)
#define ww (NUMSIZE*0)(%rsp)
#define resx (NUMSIZE*0)(%rsp)

#define yd (NUMSIZE*1)(%rsp)
#define y2a (NUMSIZE*1)(%rsp)

#define x2a (NUMSIZE*2)(%rsp)
#define zzx2 (NUMSIZE*2)(%rsp)

#define zz (NUMSIZE*3)(%rsp)
#define t1 (NUMSIZE*3)(%rsp)

#define t2 (NUMSIZE*4)(%rsp)
#define zzx1 (NUMSIZE*4)(%rsp)
#define resy (NUMSIZE*4)(%rsp)

#define xd (NUMSIZE*5)(%rsp)
#define resz (NUMSIZE*5)(%rsp)

#define tmp (NUMSIZE*6)(%rsp)

#define NSPACE NUMSIZE*7

// Corresponds exactly to bignum_mul_p521_alt except temp storage

#define mul_p521(P0,P1,P2)                      \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, 432(%rsp) ;                 \
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
        movq    %r9, 440(%rsp) ;                  \
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
        movq    %r10, 448(%rsp) ;                 \
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
        movq    %r11, 456(%rsp) ;                 \
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
        movq    %r12, 464(%rsp) ;                 \
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
        movq    %r13, 472(%rsp) ;                 \
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
        movq    %r14, 480(%rsp) ;                 \
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
        movq    %r15, 488(%rsp) ;                 \
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
        movq    %r8, 496(%rsp) ;                  \
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
        movq    496(%rsp), %r8 ;                  \
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
        adcq    432(%rsp), %r8 ;                  \
        adcq    440(%rsp), %r9 ;                  \
        adcq    448(%rsp), %r10 ;                 \
        adcq    456(%rsp), %r11 ;                 \
        adcq    464(%rsp), %r12 ;                 \
        adcq    472(%rsp), %r13 ;                 \
        adcq    480(%rsp), %r14 ;                 \
        adcq    488(%rsp), %r15 ;                 \
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
// in place of %rcx and the output as temp storage location

#define sqr_p521(P0,P1)                         \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, 432(%rsp) ;                 \
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
        movq    %r9, 440(%rsp) ;                  \
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
        movq    %r10, 448(%rsp) ;                 \
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
        movq    %r11, 456(%rsp) ;                 \
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
        movq    %r12, 464(%rsp) ;                 \
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
        movq    %r13, 472(%rsp) ;                 \
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
        movq    %r14, 480(%rsp) ;                 \
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
        movq    %r15, 488(%rsp) ;                 \
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
        movq    %r8, 496(%rsp) ;                  \
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
        movq    496(%rsp), %r8 ;                  \
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
        adcq    432(%rsp), %r8 ;                  \
        adcq    440(%rsp), %r9 ;                  \
        adcq    448(%rsp), %r10 ;                 \
        adcq    456(%rsp), %r11 ;                 \
        adcq    464(%rsp), %r12 ;                 \
        adcq    472(%rsp), %r13 ;                 \
        adcq    480(%rsp), %r14 ;                 \
        adcq    488(%rsp), %r15 ;                 \
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

#define testzero9(P)                            \
        movq    P, %rax ;                       \
        movq    8+P, %rbx ;                     \
        movq    16+P, %rdx ;                    \
        movq    24+P, %rbp ;                    \
        orq     32+P, %rax ;                    \
        orq     40+P, %rbx ;                    \
        orq     48+P, %rdx ;                    \
        orq     56+P, %rbp ;                    \
        orq     %rbx, %rax ;                       \
        orq     %rbp, %rdx ;                       \
        orq     64+P, %rax ;                    \
        orq     %rdx, %rax

#define mux9(P0,PNE,PEQ)                        \
        movq    PNE, %rax ;                     \
        movq    PEQ, %rbx ;                     \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, P0 ;                      \
        movq    8+PNE, %rax ;                   \
        movq    8+PEQ, %rbx ;                   \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 8+P0 ;                    \
        movq    16+PNE, %rax ;                  \
        movq    16+PEQ, %rbx ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 16+P0 ;                   \
        movq    24+PNE, %rax ;                  \
        movq    24+PEQ, %rbx ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 24+P0 ;                   \
        movq    32+PNE, %rax ;                  \
        movq    32+PEQ, %rbx ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 32+P0 ;                   \
        movq    40+PNE, %rax ;                  \
        movq    40+PEQ, %rbx ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 40+P0 ;                   \
        movq    48+PNE, %rax ;                  \
        movq    48+PEQ, %rbx ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 48+P0 ;                   \
        movq    56+PNE, %rax ;                  \
        movq    56+PEQ, %rbx ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 56+P0 ;                   \
        movq    64+PNE, %rax ;                  \
        movq    64+PEQ, %rbx ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 64+P0

#define mux9c(P0,PNE)                           \
        movq    PNE, %rax ;                     \
        movl    $1, %ebx ;                         \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, P0 ;                      \
        movq    8+PNE, %rax ;                   \
        movl    $0, %ebx ;                         \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 8+P0 ;                    \
        movq    16+PNE, %rax ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 16+P0 ;                   \
        movq    24+PNE, %rax ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 24+P0 ;                   \
        movq    32+PNE, %rax ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 32+P0 ;                   \
        movq    40+PNE, %rax ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 40+P0 ;                   \
        movq    48+PNE, %rax ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 48+P0 ;                   \
        movq    56+PNE, %rax ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 56+P0 ;                   \
        movq    64+PNE, %rax ;                  \
        cmovzq  %rbx, %rax ;                       \
        movq    %rax, 64+P0

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

S2N_BN_SYMBOL(p521_jmixadd_alt):
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

        sqr_p521(zp2,z_1)
        mul_p521(y2a,z_1,y_2)

        mul_p521(x2a,zp2,x_2)
        mul_p521(y2a,zp2,y2a)

        sub_p521(xd,x2a,x_1)
        sub_p521(yd,y2a,y_1)

        sqr_p521(zz,xd)
        sqr_p521(ww,yd)

        mul_p521(zzx1,zz,x_1)
        mul_p521(zzx2,zz,x2a)

        sub_p521(resx,ww,zzx1)
        sub_p521(t1,zzx2,zzx1)

        mul_p521(resz,xd,z_1)

        sub_p521(resx,resx,zzx2)

        sub_p521(t2,zzx1,resx)

        mul_p521(t1,t1,y_1)
        mul_p521(t2,yd,t2)

        sub_p521(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        testzero9(z_1)

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with an extra z = 1
// coordinate, hence giving 0 + p2 = p2 for the final result.

        mux9    (resx,resx,x_2)
        mux9    (resy,resy,y_2)

        copy9(x_3,resx)
        copy9(y_3,resy)

        mux9c(z_3,resz)

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

S2N_BN_SIZE_DIRECTIVE(p521_jmixadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
