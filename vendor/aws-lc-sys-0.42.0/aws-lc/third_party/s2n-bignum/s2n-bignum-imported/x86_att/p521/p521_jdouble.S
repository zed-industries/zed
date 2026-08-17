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
// Standard x86-64 ABI: RDI = p3, RSI = p1
// Microsoft x64 ABI:   RCX = p3, RDX = p1
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jdouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jdouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jdouble)
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

#define NSPACE 568

// Corresponds exactly to bignum_mul_p521

#define mul_p521(P0,P1,P2)                      \
        xorl    %ecx, %ecx ;                        \
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
        adcq    %rcx, %r8 ;                         \
        movq    0x8+P2, %rdx ;                   \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r9 ;                         \
        adcq    %rcx, %r9 ;                         \
        movq    0x10+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r10 ;                        \
        adcq    %rcx, %r10 ;                        \
        movq    0x18+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r11 ;                        \
        adcq    %rcx, %r11 ;                        \
        movq    0x20+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        movq    0x28+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r13 ;                        \
        adcq    %rcx, %r13 ;                        \
        movq    0x30+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r14 ;                        \
        adcq    %rcx, %r14 ;                        \
        movq    0x38+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r15 ;                        \
        adcq    %rcx, %r15 ;                        \
        movq    0x40+P1, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %rbx ;                        \
        adcq    %rbx, %rcx ;                        \
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
        adoxq   %rbx, %rcx ;                        \
        mulxq   0x40+P1, %rax, %rbx ;              \
        adcq    %rax, %rcx ;                        \
        movq    %r8, %rax ;                         \
        andq    $0x1ff, %rax ;                      \
        shrdq   $0x9, %r9, %r8 ;                      \
        shrdq   $0x9, %r10, %r9 ;                     \
        shrdq   $0x9, %r11, %r10 ;                    \
        shrdq   $0x9, %r12, %r11 ;                    \
        shrdq   $0x9, %r13, %r12 ;                    \
        shrdq   $0x9, %r14, %r13 ;                    \
        shrdq   $0x9, %r15, %r14 ;                    \
        shrdq   $0x9, %rcx, %r15 ;                    \
        shrq    $0x9, %rcx ;                        \
        addq    %rax, %rcx ;                        \
        stc;                                    \
        adcq    504(%rsp), %r8 ;                   \
        adcq    512(%rsp), %r9 ;                   \
        adcq    520(%rsp), %r10 ;                  \
        adcq    528(%rsp), %r11 ;                  \
        adcq    536(%rsp), %r12 ;                  \
        adcq    544(%rsp), %r13 ;                  \
        adcq    552(%rsp), %r14 ;                  \
        adcq    560(%rsp), %r15 ;                  \
        adcq    $0xfffffffffffffe00, %rcx ;         \
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
        sbbq    $0x0, %rcx ;                        \
        andq    $0x1ff, %rcx ;                      \
        movq    %rcx, 0x40+P0

// Corresponds exactly to bignum_sqr_p521

#define sqr_p521(P0,P1)                         \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r8 ;                         \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r9 ;                         \
        movq    0x20+P1, %rdx ;                  \
        mulxq   0x28+P1, %rax, %r10 ;              \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rcx, %r10 ;                        \
        adcxq   %rcx, %r10 ;                        \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r11 ;                        \
        mulxq   0x28+P1, %rax, %r12 ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rcx, %r12 ;                        \
        adcxq   %rcx, %r12 ;                        \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r13 ;                        \
        mulxq   0x30+P1, %rax, %r14 ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rcx, %r14 ;                        \
        adcxq   %rcx, %r14 ;                        \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r15 ;                        \
        adoxq   %rcx, %r15 ;                        \
        movq    0x40+P1, %rdx ;                  \
        movq    %rdx, %rcx ;                        \
        imulq   %rcx, %rcx ;                        \
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
        adoxq   %rbx, %rcx ;                        \
        adcq    $0x0, %rcx ;                        \
        movq    %r8, %rax ;                         \
        andq    $0x1ff, %rax ;                      \
        shrdq   $0x9, %r9, %r8 ;                      \
        shrdq   $0x9, %r10, %r9 ;                     \
        shrdq   $0x9, %r11, %r10 ;                    \
        shrdq   $0x9, %r12, %r11 ;                    \
        shrdq   $0x9, %r13, %r12 ;                    \
        shrdq   $0x9, %r14, %r13 ;                    \
        shrdq   $0x9, %r15, %r14 ;                    \
        shrdq   $0x9, %rcx, %r15 ;                    \
        shrq    $0x9, %rcx ;                        \
        addq    %rax, %rcx ;                        \
        stc;                                    \
        adcq    504(%rsp), %r8 ;                   \
        adcq    512(%rsp), %r9 ;                   \
        adcq    520(%rsp), %r10 ;                  \
        adcq    528(%rsp), %r11 ;                  \
        adcq    536(%rsp), %r12 ;                  \
        adcq    544(%rsp), %r13 ;                  \
        adcq    552(%rsp), %r14 ;                  \
        adcq    560(%rsp), %r15 ;                  \
        adcq    $0xfffffffffffffe00, %rcx ;         \
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
        sbbq    $0x0, %rcx ;                        \
        andq    $0x1ff, %rcx ;                      \
        movq    %rcx, 0x40+P0

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
        xorl    %ecx, %ecx ;                        \
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
        adcq    %rcx, %r8 ;                         \
        movq    0x8+P2, %rdx ;                   \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r9 ;                         \
        adcq    %rcx, %r9 ;                         \
        movq    0x10+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r10 ;                        \
        adcq    %rcx, %r10 ;                        \
        movq    0x18+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r11 ;                        \
        adcq    %rcx, %r11 ;                        \
        movq    0x20+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        movq    0x28+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r13 ;                        \
        adcq    %rcx, %r13 ;                        \
        movq    0x30+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r14 ;                        \
        adcq    %rcx, %r14 ;                        \
        movq    0x38+P2, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %r15 ;                        \
        adcq    %rcx, %r15 ;                        \
        movq    0x40+P1, %rdx ;                  \
        xorl    %ecx, %ecx ;                        \
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
        adoxq   %rcx, %rbx ;                        \
        adcq    %rbx, %rcx ;                        \
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
        adoxq   %rbx, %rcx ;                        \
        mulxq   0x40+P1, %rax, %rbx ;              \
        adcq    %rax, %rcx ;                        \
        movq    %r8, %rax ;                         \
        andq    $0x1ff, %rax ;                      \
        shrdq   $0x9, %r9, %r8 ;                      \
        shrdq   $0x9, %r10, %r9 ;                     \
        shrdq   $0x9, %r11, %r10 ;                    \
        shrdq   $0x9, %r12, %r11 ;                    \
        shrdq   $0x9, %r13, %r12 ;                    \
        shrdq   $0x9, %r14, %r13 ;                    \
        shrdq   $0x9, %r15, %r14 ;                    \
        shrdq   $0x9, %rcx, %r15 ;                    \
        shrq    $0x9, %rcx ;                        \
        addq    %rax, %rcx ;                        \
        addq    504(%rsp), %r8 ;                   \
        movq    %r8, P0 ;                        \
        adcq    512(%rsp), %r9 ;                   \
        movq    %r9, 0x8+P0 ;                    \
        adcq    520(%rsp), %r10 ;                  \
        movq    %r10, 0x10+P0 ;                  \
        adcq    528(%rsp), %r11 ;                  \
        movq    %r11, 0x18+P0 ;                  \
        adcq    536(%rsp), %r12 ;                  \
        movq    %r12, 0x20+P0 ;                  \
        adcq    544(%rsp), %r13 ;                  \
        movq    %r13, 0x28+P0 ;                  \
        adcq    552(%rsp), %r14 ;                  \
        movq    %r14, 0x30+P0 ;                  \
        adcq    560(%rsp), %r15 ;                  \
        movq    %r15, 0x38+P0 ;                  \
        adcq    $0, %rcx ;                          \
        movq    %rcx, 0x40+P0

// P0 = C * P1 - D * P2 == C * P1 + D * (p_521 - P2)

#define cmsub_p521(P0,C,P1,D,P2)                \
        movq    $D, %rdx ;                         \
        movq    64+P2, %rbx ;                   \
        xorq    $0x1FF, %rbx ;                     \
        movq    P2, %rax ;                      \
        notq    %rax;                            \
        mulxq   %rax, %r8, %r9 ;                    \
        movq    8+P2, %rax ;                    \
        notq    %rax;                            \
        mulxq   %rax, %rax, %r10 ;                  \
        addq    %rax, %r9 ;                        \
        movq    16+P2, %rax ;                   \
        notq    %rax;                            \
        mulxq   %rax, %rax, %r11 ;                  \
        adcq    %rax, %r10 ;                       \
        movq    24+P2, %rax ;                   \
        notq    %rax;                            \
        mulxq   %rax, %rax, %r12 ;                  \
        adcq    %rax, %r11 ;                       \
        movq    32+P2, %rax ;                   \
        notq    %rax;                            \
        mulxq   %rax, %rax, %r13 ;                  \
        adcq    %rax, %r12 ;                       \
        movq    40+P2, %rax ;                   \
        notq    %rax;                            \
        mulxq   %rax, %rax, %r14 ;                  \
        adcq    %rax, %r13 ;                       \
        movq    48+P2, %rax ;                   \
        notq    %rax;                            \
        mulxq   %rax, %rax, %r15 ;                  \
        adcq    %rax, %r14 ;                       \
        movq    56+P2, %rax ;                   \
        notq    %rax;                            \
        mulxq   %rax, %rax, %rcx ;                  \
        adcq    %rax, %r15 ;                       \
        mulxq   %rbx, %rbx, %rax ;                  \
        adcq    %rcx, %rbx ;                       \
        xorl    %eax, %eax ;                       \
        movq    $C, %rdx ;                         \
        mulxq   P1, %rax, %rcx ;                 \
        adcxq   %rax, %r8 ;                        \
        adoxq   %rcx, %r9 ;                        \
        mulxq   8+P1, %rax, %rcx ;               \
        adcxq   %rax, %r9 ;                        \
        adoxq   %rcx, %r10 ;                       \
        mulxq   16+P1, %rax, %rcx ;              \
        adcxq   %rax, %r10 ;                       \
        adoxq   %rcx, %r11 ;                       \
        mulxq   24+P1, %rax, %rcx ;              \
        adcxq   %rax, %r11 ;                       \
        adoxq   %rcx, %r12 ;                       \
        mulxq   32+P1, %rax, %rcx ;              \
        adcxq   %rax, %r12 ;                       \
        adoxq   %rcx, %r13 ;                       \
        mulxq   40+P1, %rax, %rcx ;              \
        adcxq   %rax, %r13 ;                       \
        adoxq   %rcx, %r14 ;                       \
        mulxq   48+P1, %rax, %rcx ;              \
        adcxq   %rax, %r14 ;                       \
        adoxq   %rcx, %r15 ;                       \
        mulxq   56+P1, %rax, %rcx ;              \
        adcxq   %rax, %r15 ;                       \
        adoxq   %rcx, %rbx ;                       \
        mulxq   64+P1, %rax, %rcx ;              \
        adcxq   %rax, %rbx ;                       \
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
        movq    $3, %rdx ;                         \
        xorl    %eax, %eax ;                       \
        mulxq   P1, %rax, %rcx ;                 \
        adcxq   %rax, %r8 ;                        \
        adoxq   %rcx, %r9 ;                        \
        mulxq   8+P1, %rax, %rcx ;               \
        adcxq   %rax, %r9 ;                        \
        adoxq   %rcx, %r10 ;                       \
        mulxq   16+P1, %rax, %rcx ;              \
        adcxq   %rax, %r10 ;                       \
        adoxq   %rcx, %r11 ;                       \
        mulxq   24+P1, %rax, %rcx ;              \
        adcxq   %rax, %r11 ;                       \
        adoxq   %rcx, %r12 ;                       \
        mulxq   32+P1, %rax, %rcx ;              \
        adcxq   %rax, %r12 ;                       \
        adoxq   %rcx, %r13 ;                       \
        mulxq   40+P1, %rax, %rcx ;              \
        adcxq   %rax, %r13 ;                       \
        adoxq   %rcx, %r14 ;                       \
        mulxq   48+P1, %rax, %rcx ;              \
        adcxq   %rax, %r14 ;                       \
        adoxq   %rcx, %r15 ;                       \
        mulxq   56+P1, %rax, %rcx ;              \
        adcxq   %rax, %r15 ;                       \
        adoxq   %rcx, %rbx ;                       \
        mulxq   64+P1, %rax, %rcx ;              \
        adcxq   %rax, %rbx ;                       \
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

S2N_BN_SYMBOL(p521_jdouble):
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
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p521_jdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
