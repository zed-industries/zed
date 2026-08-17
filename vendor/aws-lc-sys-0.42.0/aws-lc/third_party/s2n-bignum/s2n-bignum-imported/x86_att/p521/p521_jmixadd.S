// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on NIST curve P-521 in Jacobian coordinates
//
//    extern void p521_jmixadd(uint64_t p3[static 27], const uint64_t p1[static 27],
//                             const uint64_t p2[static 18]);
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


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jmixadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jmixadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jmixadd)
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

#define NSPACE 496

// Corresponds exactly to bignum_mul_p521

#define mul_p521(P0,P1,P2)                      \
        xorl    %ebp, %ebp ;                        \
        movq    P2, %rdx ;                       \
        mulxq   P1, %r8, %r9 ;                     \
        movq    %r8, 432(%rsp) ;                   \
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
        movq    %r9, 440(%rsp) ;                   \
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
        movq    %r10, 448(%rsp) ;                  \
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
        movq    %r11, 456(%rsp) ;                  \
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
        movq    %r12, 464(%rsp) ;                  \
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
        movq    %r13, 472(%rsp) ;                  \
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
        movq    %r14, 480(%rsp) ;                  \
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
        movq    %r15, 488(%rsp) ;                  \
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
        adcq    432(%rsp), %r8 ;                   \
        adcq    440(%rsp), %r9 ;                   \
        adcq    448(%rsp), %r10 ;                  \
        adcq    456(%rsp), %r11 ;                  \
        adcq    464(%rsp), %r12 ;                  \
        adcq    472(%rsp), %r13 ;                  \
        adcq    480(%rsp), %r14 ;                  \
        adcq    488(%rsp), %r15 ;                  \
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
        movq    %r9, 440(%rsp) ;                   \
        mulxq   0x10+P1, %r10, %rbx ;              \
        adcxq   %rax, %r10 ;                        \
        movq    %r10, 448(%rsp) ;                  \
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
        movq    %r11, 456(%rsp) ;                  \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        movq    %r12, 464(%rsp) ;                  \
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
        movq    %r13, 472(%rsp) ;                  \
        mulxq   0x20+P1, %rax, %rbx ;              \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        movq    %r14, 480(%rsp) ;                  \
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
        movq    %r15, 488(%rsp) ;                  \
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
        movq    %rax, 432(%rsp) ;                  \
        movq    440(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 440(%rsp) ;                  \
        movq    448(%rsp), %rax ;                  \
        movq    0x8+P1, %rdx ;                   \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %rax, %rax ;                        \
        adoxq   %rdx, %rax ;                        \
        movq    %rax, 448(%rsp) ;                  \
        movq    456(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 456(%rsp) ;                  \
        movq    464(%rsp), %rax ;                  \
        movq    0x10+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %rax, %rax ;                        \
        adoxq   %rdx, %rax ;                        \
        movq    %rax, 464(%rsp) ;                  \
        movq    472(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 472(%rsp) ;                  \
        movq    480(%rsp), %rax ;                  \
        movq    0x18+P1, %rdx ;                  \
        mulxq   %rdx, %rdx, %rbx ;                    \
        adcxq   %rax, %rax ;                        \
        adoxq   %rdx, %rax ;                        \
        movq    %rax, 480(%rsp) ;                  \
        movq    488(%rsp), %rax ;                  \
        adcxq   %rax, %rax ;                        \
        adoxq   %rbx, %rax ;                        \
        movq    %rax, 488(%rsp) ;                  \
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
        adcq    432(%rsp), %r8 ;                   \
        adcq    440(%rsp), %r9 ;                   \
        adcq    448(%rsp), %r10 ;                  \
        adcq    456(%rsp), %r11 ;                  \
        adcq    464(%rsp), %r12 ;                  \
        adcq    472(%rsp), %r13 ;                  \
        adcq    480(%rsp), %r14 ;                  \
        adcq    488(%rsp), %r15 ;                  \
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

S2N_BN_SYMBOL(p521_jmixadd):
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

S2N_BN_SIZE_DIRECTIVE(p521_jmixadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
