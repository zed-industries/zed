// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jadd(uint64_t p3[static 27], const uint64_t p1[static 27],
//                          const uint64_t p2[static 27]);
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


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jadd)
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

#define NSPACE 568

// Corresponds exactly to bignum_mul_p521

#define mul_p521(P0,P1,P2)                      \
        xorl    %ebp, %ebp ;                        \
        movq    P2, %rdx ;                       \
        mulxq   P1, %r8, %r9 ;                     \
        movq    %r8, 504(%rsp) ;                   \
        mulxq   0x8+P1, %rbx, %r10 ;               \
        adcq    %rbx, %r9 ;                         \
        mulxq   0x10+P1, %rbx, %r11 ;              \
        adcq    %rbx, %r10 ;                        \
        mulxq   0x18+P1, %rbx, %r12 ;              \
        adcq    %rbx, %r11 ;                        \
        mulxq   0x20+P1, %rbx, %r13 ;              \
        adcq    %rbx, %r12 ;                        \
        mulxq   0x28+P1, %rbx, %r14 ;              \
        adcq    %rbx, %r13 ;                        \
        mulxq   0x30+P1, %rbx, %r15 ;              \
        adcq    %rbx, %r14 ;                        \
        mulxq   0x38+P1, %rbx, %r8 ;               \
        adcq    %rbx, %r15 ;                        \
        adcq    %rbp, %r8 ;                         \
        movq    0x8+P2, %rdx ;                   \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        movq    %r9, 512(%rsp) ;                   \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x38+P1, %rax, %r9 ;               \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbp, %r9 ;                         \
        adcq    %rbp, %r9 ;                         \
        movq    0x10+P2, %rdx ;                  \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        movq    %r10, 520(%rsp) ;                  \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x38+P1, %rax, %r10 ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbp, %r10 ;                        \
        adcq    %rbp, %r10 ;                        \
        movq    0x18+P2, %rdx ;                  \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        movq    %r11, 528(%rsp) ;                  \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x38+P1, %rax, %r11 ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbp, %r11 ;                        \
        adcq    %rbp, %r11 ;                        \
        movq    0x20+P2, %rdx ;                  \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        movq    %r12, 536(%rsp) ;                  \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x38+P1, %rax, %r12 ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbp, %r12 ;                        \
        adcq    %rbp, %r12 ;                        \
        movq    0x28+P2, %rdx ;                  \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        movq    %r13, 544(%rsp) ;                  \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x38+P1, %rax, %r13 ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbp, %r13 ;                        \
        adcq    %rbp, %r13 ;                        \
        movq    0x30+P2, %rdx ;                  \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        movq    %r14, 552(%rsp) ;                  \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x38+P1, %rax, %r14 ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbp, %r14 ;                        \
        adcq    %rbp, %r14 ;                        \
        movq    0x38+P2, %rdx ;                  \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        movq    %r15, 560(%rsp) ;                  \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x38+P1, %rax, %r15 ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbp, %r15 ;                        \
        adcq    %rbp, %r15 ;                        \
        movq    0x40+P1, %rdx ;                  \
        xorl    %ebp, %ebp ;                        \
        mulxq   P2, %rax, %rbx ;                   \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x8+P2, %rax, %rbx ;               \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x10+P2, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x18+P2, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x20+P2, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x28+P2, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x30+P2, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x38+P2, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbp, %rbx ;                        \
        adcq    %rbx, %rbp ;                        \
        movq    0x40+P2, %rdx ;                  \
        xorl    %eax, %eax ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x38+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %rbp ;                        \
        mulxq   0x40+P1, %rax, %rbx ;              \
        adcq    %rax, %rbp ;                        \
        movq    %r8, %rax ;                         \
        andq    $0x1ff, %rax ;                      \
        shrdq   $0x9, %r9, %r8 ;                      \
        shrdq   $0x9, %r10, %r9 ;                     \
        shrdq   $0x9, %r11, %r10 ;                    \
        shrdq   $0x9, %r12, %r11 ;                    \
        shrdq   $0x9, %r13, %r12 ;                    \
        shrdq   $0x9, %r14, %r13 ;                    \
        shrdq   $0x9, %r15, %r14 ;                    \
        shrdq   $0x9, %rbp, %r15 ;                    \
        shrq    $0x9, %rbp ;                        \
        addq    %rax, %rbp ;                        \
        stc;                                    \
        adcq    504(%rsp), %r8 ;                   \
        adcq    512(%rsp), %r9 ;                   \
        adcq    520(%rsp), %r10 ;                  \
        adcq    528(%rsp), %r11 ;                  \
        adcq    536(%rsp), %r12 ;                  \
        adcq    544(%rsp), %r13 ;                  \
        adcq    552(%rsp), %r14 ;                  \
        adcq    560(%rsp), %r15 ;                  \
        adcq    $0xfffffffffffffe00, %rbp ;         \
        cmc;                                    \
        sbbq    $0x0, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    $0x0, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        sbbq    $0x0, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        sbbq    $0x0, %r11 ;                        \
        movq    %r11, 0x18+P0 ;                  \
        sbbq    $0x0, %r12 ;                        \
        movq    %r12, 0x20+P0 ;                  \
        sbbq    $0x0, %r13 ;                        \
        movq    %r13, 0x28+P0 ;                  \
        sbbq    $0x0, %r14 ;                        \
        movq    %r14, 0x30+P0 ;                  \
        sbbq    $0x0, %r15 ;                        \
        movq    %r15, 0x38+P0 ;                  \
        sbbq    $0x0, %rbp ;                        \
        andq    $0x1ff, %rbp ;                      \
        movq    %rbp, 0x40+P0

// Corresponds exactly to bignum_sqr_p521

#define sqr_p521(P0,P1)                         \
        xorl    %ebp, %ebp ;                        \
        movq    P1, %rdx ;                       \
        mulxq   0x8+P1, %r9, %rax ;                \
        movq    %r9, 512(%rsp) ;                   \
        mulxq   0x10+P1, %r10, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        movq    %r10, 520(%rsp) ;                  \
        mulxq   0x18+P1, %r11, %rax ;              \
        adcxq   %rbx, %r11 ;                        \
        mulxq   0x20+P1, %r12, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        mulxq   0x28+P1, %r13, %rax ;              \
        adcxq   %rbx, %r13 ;                        \
        mulxq   0x30+P1, %r14, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        mulxq   0x38+P1, %r15, %r8 ;               \
        adcxq   %rbx, %r15 ;                        \
        adcxq   %rbp, %r8 ;                         \
        xorl    %ebp, %ebp ;                        \
        movq    0x8+P1, %rdx ;                   \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        movq    %r11, 528(%rsp) ;                  \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        movq    %r12, 536(%rsp) ;                  \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x38+P1, %rax, %r9 ;               \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbp, %r9 ;                         \
        movq    0x20+P1, %rdx ;                  \
        mulxq   0x28+P1, %rax, %r10 ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbp, %r10 ;                        \
        adcxq   %rbp, %r10 ;                        \
        xorl    %ebp, %ebp ;                        \
        movq    0x10+P1, %rdx ;                  \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        movq    %r13, 544(%rsp) ;                  \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        movq    %r14, 552(%rsp) ;                  \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x38+P1, %rax, %rbx ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        movq    0x30+P1, %rdx ;                  \
        mulxq   0x20+P1, %rax, %r11 ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbp, %r11 ;                        \
        mulxq   0x28+P1, %rax, %r12 ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbp, %r12 ;                        \
        adcxq   %rbp, %r12 ;                        \
        xorl    %ebp, %ebp ;                        \
        movq    0x18+P1, %rdx ;                  \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %r8 ;                         \
        movq    %r15, 560(%rsp) ;                  \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x38+P1, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        movq    0x38+P1, %rdx ;                  \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x28+P1, %rax, %r13 ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbp, %r13 ;                        \
        mulxq   0x30+P1, %rax, %r14 ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbp, %r14 ;                        \
        adcxq   %rbp, %r14 ;                        \
        xorl    %ebp, %ebp ;                        \
        movq    P1, %rdx ;                       \
        mulxq   %rdx, %rax, %rbx ;                    \
        movq    %rax, 504(%rsp) ;                  \
        movq    512(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 512(%rsp) ;                  \
        movq    520(%rsp), %rax ;                  \
        movq    0x8+P1, %rdx ;                   \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %rax, %rax ;                        \
        adoxq   %rdx, %rax ;                        \
        movq    %rax, 520(%rsp) ;                  \
        movq    528(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 528(%rsp) ;                  \
        movq    536(%rsp), %rax ;                  \
        movq    0x10+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %rax, %rax ;                        \
        adoxq   %rdx, %rax ;                        \
        movq    %rax, 536(%rsp) ;                  \
        movq    544(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 544(%rsp) ;                  \
        movq    552(%rsp), %rax ;                  \
        movq    0x18+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %rax, %rax ;                        \
        adoxq   %rdx, %rax ;                        \
        movq    %rax, 552(%rsp) ;                  \
        movq    560(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 560(%rsp) ;                  \
        movq    0x20+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %r8, %r8 ;                          \
        adoxq   %rdx, %r8 ;                         \
        adcxq   %r9, %r9 ;                          \
        adoxq   %rbx, %r9 ;                         \
        movq    0x28+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %r10, %r10 ;                        \
        adoxq   %rdx, %r10 ;                        \
        adcxq   %r11, %r11 ;                        \
        adoxq   %rbx, %r11 ;                        \
        movq    0x30+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %r12, %r12 ;                        \
        adoxq   %rdx, %r12 ;                        \
        adcxq   %r13, %r13 ;                        \
        adoxq   %rbx, %r13 ;                        \
        movq    0x38+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %r15 ;                    \
        adcxq   %r14, %r14 ;                        \
        adoxq   %rdx, %r14 ;                        \
        adcxq   %rbp, %r15 ;                        \
        adoxq   %rbp, %r15 ;                        \
        movq    0x40+P1, %rdx ;                  \
        movq    %rdx, %rbp ;                        \
        imulq   %rbp, %rbp ;                        \
        addq    %rdx, %rdx ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r8 ;                         \
        adoxq   %rbx, %r9 ;                         \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x28+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   0x30+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        mulxq   0x38+P1, %rax, %rbx ;              \
        adcxq   %rax, %r15 ;                        \
        adoxq   %rbx, %rbp ;                        \
        adcq    $0x0, %rbp ;                        \
        movq    %r8, %rax ;                         \
        andq    $0x1ff, %rax ;                      \
        shrdq   $0x9, %r9, %r8 ;                      \
        shrdq   $0x9, %r10, %r9 ;                     \
        shrdq   $0x9, %r11, %r10 ;                    \
        shrdq   $0x9, %r12, %r11 ;                    \
        shrdq   $0x9, %r13, %r12 ;                    \
        shrdq   $0x9, %r14, %r13 ;                    \
        shrdq   $0x9, %r15, %r14 ;                    \
        shrdq   $0x9, %rbp, %r15 ;                    \
        shrq    $0x9, %rbp ;                        \
        addq    %rax, %rbp ;                        \
        stc;                                    \
        adcq    504(%rsp), %r8 ;                   \
        adcq    512(%rsp), %r9 ;                   \
        adcq    520(%rsp), %r10 ;                  \
        adcq    528(%rsp), %r11 ;                  \
        adcq    536(%rsp), %r12 ;                  \
        adcq    544(%rsp), %r13 ;                  \
        adcq    552(%rsp), %r14 ;                  \
        adcq    560(%rsp), %r15 ;                  \
        adcq    $0xfffffffffffffe00, %rbp ;         \
        cmc;                                    \
        sbbq    $0x0, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    $0x0, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        sbbq    $0x0, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        sbbq    $0x0, %r11 ;                        \
        movq    %r11, 0x18+P0 ;                  \
        sbbq    $0x0, %r12 ;                        \
        movq    %r12, 0x20+P0 ;                  \
        sbbq    $0x0, %r13 ;                        \
        movq    %r13, 0x28+P0 ;                  \
        sbbq    $0x0, %r14 ;                        \
        movq    %r14, 0x30+P0 ;                  \
        sbbq    $0x0, %r15 ;                        \
        movq    %r15, 0x38+P0 ;                  \
        sbbq    $0x0, %rbp ;                        \
        andq    $0x1ff, %rbp ;                      \
        movq    %rbp, 0x40+P0

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

S2N_BN_SYMBOL(p521_jadd):
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

S2N_BN_SIZE_DIRECTIVE(p521_jadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
