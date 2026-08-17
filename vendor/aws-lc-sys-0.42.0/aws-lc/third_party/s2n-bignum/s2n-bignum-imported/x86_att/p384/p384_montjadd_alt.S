// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point addition on NIST curve P-384 in Montgomery-Jacobian coordinates
//
//    extern void p384_montjadd_alt(uint64_t p3[static 18],
//                                  const uint64_t p1[static 18],
//                                  const uint64_t p2[static 18]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^384 * x) mod p_384.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard x86-64 ABI: RDI = p3, RSI = p1, RDX = p2
// Microsoft x64 ABI:   RCX = p3, RDX = p1, R8 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjadd_alt)
        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 48

// Pointer-offset pairs for inputs and outputs
// These assume %rdi = p3, %rsi = p1 and %rcx = p2,
// which needs to be set up explicitly before use.
// The %rdi value never changes, however.

#define x_1 0(%rsi)
#define y_1 NUMSIZE(%rsi)
#define z_1 (2*NUMSIZE)(%rsi)

#define x_2 0(%rcx)
#define y_2 NUMSIZE(%rcx)
#define z_2 (2*NUMSIZE)(%rcx)

#define x_3 0(%rdi)
#define y_3 NUMSIZE(%rdi)
#define z_3 (2*NUMSIZE)(%rdi)

// In one place it's convenient to use another register
// since the squaring function overwrites %rcx

#define z_2_alt (2*NUMSIZE)(%rsi)

// Pointer-offset pairs for temporaries, with some aliasing
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

// Temporaries for the actual input pointers

#define input_x (NUMSIZE*7)(%rsp)
#define input_y (NUMSIZE*7+8)(%rsp)

#define NSPACE 352

// Corresponds exactly to bignum_montmul_p384_alt

#define montmul_p384(P0,P1,P2)                  \
        movq    P2, %rbx ;                      \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        xorl    %r10d, %r10d ;                     \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        xorl    %r11d, %r11d ;                     \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        xorl    %r12d, %r12d ;                     \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        xorl    %r13d, %r13d ;                     \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        xorl    %r14d, %r14d ;                     \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        xorl    %r15d, %r15d ;                     \
        movq    %r8, %rbx ;                        \
        shlq    $0x20, %rbx ;                      \
        addq    %r8, %rbx ;                        \
        xorl    %ebp, %ebp ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r8 ;                        \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %r8, %rax ;                        \
        adcq    %rbx, %rdx ;                       \
        adcl    %ebp, %ebp ;                       \
        subq    %rax, %r9 ;                        \
        sbbq    %rdx, %r10 ;                       \
        sbbq    %rbp, %r11 ;                       \
        sbbq    $0x0, %r12 ;                       \
        sbbq    $0x0, %r13 ;                       \
        sbbq    $0x0, %rbx ;                       \
        addq    %rbx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x8+P2, %rbx ;                  \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %r8, %r8 ;                         \
        negq    %r8;                             \
        movq    %r9, %rbx ;                        \
        shlq    $0x20, %rbx ;                      \
        addq    %r9, %rbx ;                        \
        xorl    %ebp, %ebp ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r9 ;                        \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %r9, %rax ;                        \
        adcq    %rbx, %rdx ;                       \
        adcl    %ebp, %ebp ;                       \
        subq    %rax, %r10 ;                       \
        sbbq    %rdx, %r11 ;                       \
        sbbq    %rbp, %r12 ;                       \
        sbbq    $0x0, %r13 ;                       \
        sbbq    $0x0, %r14 ;                       \
        sbbq    $0x0, %rbx ;                       \
        addq    %rbx, %r15 ;                       \
        adcq    $0x0, %r8 ;                        \
        movq    0x10+P2, %rbx ;                 \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        sbbq    %r9, %r9 ;                         \
        negq    %r9;                             \
        movq    %r10, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r10, %rbx ;                       \
        xorl    %ebp, %ebp ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r10 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %r10, %rax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %ebp, %ebp ;                       \
        subq    %rax, %r11 ;                       \
        sbbq    %rdx, %r12 ;                       \
        sbbq    %rbp, %r13 ;                       \
        sbbq    $0x0, %r14 ;                       \
        sbbq    $0x0, %r15 ;                       \
        sbbq    $0x0, %rbx ;                       \
        addq    %rbx, %r8 ;                        \
        adcq    $0x0, %r9 ;                        \
        movq    0x18+P2, %rbx ;                 \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %r10, %r10 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r10, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %r10, %r10 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r10, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %r10, %r10 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r10, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %r10, %r10 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r10, %rdx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        sbbq    %r10, %r10 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r10, %rdx ;                       \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %r10, %r10 ;                       \
        negq    %r10;                            \
        movq    %r11, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r11, %rbx ;                       \
        xorl    %ebp, %ebp ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r11 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %r11, %rax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %ebp, %ebp ;                       \
        subq    %rax, %r12 ;                       \
        sbbq    %rdx, %r13 ;                       \
        sbbq    %rbp, %r14 ;                       \
        sbbq    $0x0, %r15 ;                       \
        sbbq    $0x0, %r8 ;                        \
        sbbq    $0x0, %rbx ;                       \
        addq    %rbx, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        movq    0x20+P2, %rbx ;                 \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %r11, %r11 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r11, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %r11, %r11 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r11, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %r11, %r11 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r11, %rdx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        sbbq    %r11, %r11 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r11, %rdx ;                       \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %r11, %r11 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r11, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %r11, %r11 ;                       \
        negq    %r11;                            \
        movq    %r12, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r12, %rbx ;                       \
        xorl    %ebp, %ebp ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r12 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %r12, %rax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %ebp, %ebp ;                       \
        subq    %rax, %r13 ;                       \
        sbbq    %rdx, %r14 ;                       \
        sbbq    %rbp, %r15 ;                       \
        sbbq    $0x0, %r8 ;                        \
        sbbq    $0x0, %r9 ;                        \
        sbbq    $0x0, %rbx ;                       \
        addq    %rbx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        movq    0x28+P2, %rbx ;                 \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %r12, %r12 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r12, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %r12, %r12 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r12, %rdx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %r8 ;                        \
        sbbq    %r12, %r12 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r12, %rdx ;                       \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %r12, %r12 ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r12, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %r12, %r12 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r12, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %r12, %r12 ;                       \
        negq    %r12;                            \
        movq    %r13, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r13, %rbx ;                       \
        xorl    %ebp, %ebp ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r13 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %r13, %rax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %ebp, %ebp ;                       \
        subq    %rax, %r14 ;                       \
        sbbq    %rdx, %r15 ;                       \
        sbbq    %rbp, %r8 ;                        \
        sbbq    $0x0, %r9 ;                        \
        sbbq    $0x0, %r10 ;                       \
        sbbq    $0x0, %rbx ;                       \
        addq    %rbx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorl    %edx, %edx ;                       \
        xorl    %ebp, %ebp ;                       \
        xorl    %r13d, %r13d ;                     \
        movq    $0xffffffff00000001, %rax ;        \
        addq    %r14, %rax ;                       \
        movl    $0xffffffff, %ebx ;                \
        adcq    %r15, %rbx ;                       \
        movl    $0x1, %ecx ;                       \
        adcq    %r8, %rcx ;                        \
        adcq    %r9, %rdx ;                        \
        adcq    %r10, %rbp ;                       \
        adcq    %r11, %r13 ;                       \
        adcq    $0x0, %r12 ;                       \
        cmovneq %rax, %r14 ;                       \
        cmovneq %rbx, %r15 ;                       \
        cmovneq %rcx, %r8 ;                        \
        cmovneq %rdx, %r9 ;                        \
        cmovneq %rbp, %r10 ;                       \
        cmovneq %r13, %r11 ;                       \
        movq    %r14, P0 ;                      \
        movq    %r15, 0x8+P0 ;                  \
        movq    %r8, 0x10+P0 ;                  \
        movq    %r9, 0x18+P0 ;                  \
        movq    %r10, 0x20+P0 ;                 \
        movq    %r11, 0x28+P0

// Corresponds exactly to bignum_montsqr_p384_alt

#define montsqr_p384(P0,P1)                     \
        movq    P1, %rbx ;                      \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        movq    %rax, %r9 ;                        \
        movq    %rdx, %r10 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        movq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        movq    %rax, %r13 ;                       \
        movq    %rdx, %r14 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x20+P1;            \
        movq    %rax, %r15 ;                       \
        movq    %rdx, %rcx ;                       \
        movq    0x10+P1, %rbx ;                 \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x8+P1, %rbx ;                  \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    $0x0, %rcx ;                       \
        movq    0x20+P1, %rbx ;                 \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x10+P1, %rbx ;                 \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %rbp, %rdx ;                       \
        addq    %rax, %r15 ;                       \
        adcq    %rdx, %rcx ;                       \
        sbbq    %rbp, %rbp ;                       \
        xorl    %ebx, %ebx ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x28+P1;            \
        subq    %rbp, %rdx ;                       \
        xorl    %ebp, %ebp ;                       \
        addq    %rax, %rcx ;                       \
        adcq    %rdx, %rbx ;                       \
        adcl    %ebp, %ebp ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq     0x28+P1;            \
        addq    %rax, %rbx ;                       \
        adcq    %rdx, %rbp ;                       \
        xorl    %r8d, %r8d ;                       \
        addq    %r9, %r9 ;                         \
        adcq    %r10, %r10 ;                       \
        adcq    %r11, %r11 ;                       \
        adcq    %r12, %r12 ;                       \
        adcq    %r13, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        adcq    %rcx, %rcx ;                       \
        adcq    %rbx, %rbx ;                       \
        adcq    %rbp, %rbp ;                       \
        adcl    %r8d, %r8d ;                       \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %r8, P0 ;                       \
        movq    %rax, %r8 ;                        \
        movq    0x8+P1, %rax ;                  \
        movq    %rbp, 0x8+P0 ;                  \
        addq    %rdx, %r9 ;                        \
        sbbq    %rbp, %rbp ;                       \
        mulq    %rax;                            \
        negq    %rbp;                            \
        adcq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        negq    %rbp;                            \
        adcq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        negq    %rbp;                            \
        adcq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rax;                            \
        negq    %rbp;                            \
        adcq    %rax, %rcx ;                       \
        adcq    %rdx, %rbx ;                       \
        sbbq    %rbp, %rbp ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rax;                            \
        negq    %rbp;                            \
        adcq    0x8+P0, %rax ;                  \
        adcq    P0, %rdx ;                      \
        movq    %rax, %rbp ;                       \
        movq    %rdx, %rsi ;                       \
        movq    %rbx, P0 ;                      \
        movq    %r8, %rbx ;                        \
        shlq    $0x20, %rbx ;                      \
        addq    %r8, %rbx ;                        \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r8 ;                        \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        movl    $0x0, %eax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %eax, %eax ;                       \
        subq    %r8, %r9 ;                         \
        sbbq    %rdx, %r10 ;                       \
        sbbq    %rax, %r11 ;                       \
        sbbq    $0x0, %r12 ;                       \
        sbbq    $0x0, %r13 ;                       \
        movq    %rbx, %r8 ;                        \
        sbbq    $0x0, %r8 ;                        \
        movq    %r9, %rbx ;                        \
        shlq    $0x20, %rbx ;                      \
        addq    %r9, %rbx ;                        \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r9 ;                        \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %rax, %r9 ;                        \
        movl    $0x0, %eax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %eax, %eax ;                       \
        subq    %r9, %r10 ;                        \
        sbbq    %rdx, %r11 ;                       \
        sbbq    %rax, %r12 ;                       \
        sbbq    $0x0, %r13 ;                       \
        sbbq    $0x0, %r8 ;                        \
        movq    %rbx, %r9 ;                        \
        sbbq    $0x0, %r9 ;                        \
        movq    %r10, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r10, %rbx ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r10 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %rax, %r10 ;                       \
        movl    $0x0, %eax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %eax, %eax ;                       \
        subq    %r10, %r11 ;                       \
        sbbq    %rdx, %r12 ;                       \
        sbbq    %rax, %r13 ;                       \
        sbbq    $0x0, %r8 ;                        \
        sbbq    $0x0, %r9 ;                        \
        movq    %rbx, %r10 ;                       \
        sbbq    $0x0, %r10 ;                       \
        movq    %r11, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r11, %rbx ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r11 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %rax, %r11 ;                       \
        movl    $0x0, %eax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %eax, %eax ;                       \
        subq    %r11, %r12 ;                       \
        sbbq    %rdx, %r13 ;                       \
        sbbq    %rax, %r8 ;                        \
        sbbq    $0x0, %r9 ;                        \
        sbbq    $0x0, %r10 ;                       \
        movq    %rbx, %r11 ;                       \
        sbbq    $0x0, %r11 ;                       \
        movq    %r12, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r12, %rbx ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r12 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %rax, %r12 ;                       \
        movl    $0x0, %eax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %eax, %eax ;                       \
        subq    %r12, %r13 ;                       \
        sbbq    %rdx, %r8 ;                        \
        sbbq    %rax, %r9 ;                        \
        sbbq    $0x0, %r10 ;                       \
        sbbq    $0x0, %r11 ;                       \
        movq    %rbx, %r12 ;                       \
        sbbq    $0x0, %r12 ;                       \
        movq    %r13, %rbx ;                       \
        shlq    $0x20, %rbx ;                      \
        addq    %r13, %rbx ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        mulq    %rbx;                            \
        movq    %rdx, %r13 ;                       \
        movq    $0xffffffff, %rax ;                \
        mulq    %rbx;                            \
        addq    %rax, %r13 ;                       \
        movl    $0x0, %eax ;                       \
        adcq    %rbx, %rdx ;                       \
        adcl    %eax, %eax ;                       \
        subq    %r13, %r8 ;                        \
        sbbq    %rdx, %r9 ;                        \
        sbbq    %rax, %r10 ;                       \
        sbbq    $0x0, %r11 ;                       \
        sbbq    $0x0, %r12 ;                       \
        movq    %rbx, %r13 ;                       \
        sbbq    $0x0, %r13 ;                       \
        movq    P0, %rbx ;                      \
        addq    %r8, %r14 ;                        \
        adcq    %r9, %r15 ;                        \
        adcq    %r10, %rcx ;                       \
        adcq    %r11, %rbx ;                       \
        adcq    %r12, %rbp ;                       \
        adcq    %r13, %rsi ;                       \
        movl    $0x0, %r8d ;                       \
        adcq    %r8, %r8 ;                         \
        xorq    %r11, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    $0xffffffff00000001, %rax ;        \
        addq    %r14, %rax ;                       \
        movl    $0xffffffff, %r9d ;                \
        adcq    %r15, %r9 ;                        \
        movl    $0x1, %r10d ;                      \
        adcq    %rcx, %r10 ;                       \
        adcq    %rbx, %r11 ;                       \
        adcq    %rbp, %r12 ;                       \
        adcq    %rsi, %r13 ;                       \
        adcq    $0x0, %r8 ;                        \
        cmovneq %rax, %r14 ;                       \
        cmovneq %r9, %r15 ;                        \
        cmovneq %r10, %rcx ;                       \
        cmovneq %r11, %rbx ;                       \
        cmovneq %r12, %rbp ;                       \
        cmovneq %r13, %rsi ;                       \
        movq    %r14, P0 ;                      \
        movq    %r15, 0x8+P0 ;                  \
        movq    %rcx, 0x10+P0 ;                 \
        movq    %rbx, 0x18+P0 ;                 \
        movq    %rbp, 0x20+P0 ;                 \
        movq    %rsi, 0x28+P0

// Corresponds exactly to bignum_sub_p384

#define sub_p384(P0,P1,P2)                      \
        movq   P1, %rax ;                        \
        subq   P2, %rax ;                        \
        movq   0x8+P1, %rdx ;                    \
        sbbq   0x8+P2, %rdx ;                    \
        movq   0x10+P1, %r8 ;                    \
        sbbq   0x10+P2, %r8 ;                    \
        movq   0x18+P1, %r9 ;                    \
        sbbq   0x18+P2, %r9 ;                    \
        movq   0x20+P1, %r10 ;                   \
        sbbq   0x20+P2, %r10 ;                   \
        movq   0x28+P1, %r11 ;                   \
        sbbq   0x28+P2, %r11 ;                   \
        sbbq   %rcx, %rcx ;                         \
        movl   $0xffffffff, %esi ;                  \
        andq   %rsi, %rcx ;                         \
        xorq   %rsi, %rsi ;                         \
        subq   %rcx, %rsi ;                         \
        subq   %rsi, %rax ;                         \
        movq   %rax, P0 ;                        \
        sbbq   %rcx, %rdx ;                         \
        movq   %rdx, 0x8+P0 ;                    \
        sbbq   %rax, %rax ;                         \
        andq   %rsi, %rcx ;                         \
        negq   %rax;                             \
        sbbq   %rcx, %r8 ;                          \
        movq   %r8, 0x10+P0 ;                    \
        sbbq   $0x0, %r9 ;                          \
        movq   %r9, 0x18+P0 ;                    \
        sbbq   $0x0, %r10 ;                         \
        movq   %r10, 0x20+P0 ;                   \
        sbbq   $0x0, %r11 ;                         \
        movq   %r11, 0x28+P0

// Additional macros to help with final multiplexing

#define load6(r0,r1,r2,r3,r4,r5,P)              \
        movq    P, r0 ;                        \
        movq    8+P, r1 ;                      \
        movq    16+P, r2 ;                     \
        movq    24+P, r3 ;                     \
        movq    32+P, r4 ;                     \
        movq    40+P, r5

#define store6(P,r0,r1,r2,r3,r4,r5)             \
        movq    r0, P ;                        \
        movq    r1, 8+P ;                      \
        movq    r2, 16+P ;                     \
        movq    r3, 24+P ;                     \
        movq    r4, 32+P ;                     \
        movq    r5, 40+P ;                     \

#define czload6(r0,r1,r2,r3,r4,r5,P)            \
        cmovzq  P, r0 ;                        \
        cmovzq  8+P, r1 ;                      \
        cmovzq  16+P, r2 ;                     \
        cmovzq  24+P, r3 ;                     \
        cmovzq  32+P, r4 ;                     \
        cmovzq  40+P, r5

#define muxload6(r0,r1,r2,r3,r4,r5,P0,P1,P2)    \
        movq    P0, r0 ;                       \
        cmovbq  P1, r0 ;                       \
        cmovnbe P2, r0 ;                       \
        movq    8+P0, r1 ;                     \
        cmovbq  8+P1, r1 ;                     \
        cmovnbe 8+P2, r1 ;                     \
        movq    16+P0, r2 ;                    \
        cmovbq  16+P1, r2 ;                    \
        cmovnbe 16+P2, r2 ;                    \
        movq    24+P0, r3 ;                    \
        cmovbq  24+P1, r3 ;                    \
        cmovnbe 24+P2, r3 ;                    \
        movq    32+P0, r4 ;                    \
        cmovbq  32+P1, r4 ;                    \
        cmovnbe 32+P2, r4 ;                    \
        movq    40+P0, r5 ;                    \
        cmovbq  40+P1, r5 ;                    \
        cmovnbe 40+P2, r5

S2N_BN_SYMBOL(p384_montjadd_alt):
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
// Put the input arguments in non-volatile places on the stack

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

        movq    %rsi, input_x
        movq    %rdx, input_y

// Main code, just a sequence of basic field operations
// 8 * multiply + 3 * square + 7 * subtract

        montsqr_p384(z1sq,z_1)
        movq    input_y, %rsi
        montsqr_p384(z2sq,z_2_alt)

        movq    input_x, %rsi
        movq    input_y, %rcx
        montmul_p384(y1a,z_2,y_1)
        movq    input_x, %rsi
        movq    input_y, %rcx
        montmul_p384(y2a,z_1,y_2)

        movq    input_y, %rcx
        montmul_p384(x2a,z1sq,x_2)
        movq    input_x, %rsi
        montmul_p384(x1a,z2sq,x_1)
        montmul_p384(y2a,z1sq,y2a)
        montmul_p384(y1a,z2sq,y1a)

        sub_p384(xd,x2a,x1a)
        sub_p384(yd,y2a,y1a)

        montsqr_p384(zz,xd)
        montsqr_p384(ww,yd)

        montmul_p384(zzx1,zz,x1a)
        montmul_p384(zzx2,zz,x2a)

        sub_p384(resx,ww,zzx1)
        sub_p384(t1,zzx2,zzx1)

        movq    input_x, %rsi
        montmul_p384(xd,xd,z_1)

        sub_p384(resx,resx,zzx2)

        sub_p384(t2,zzx1,resx)

        montmul_p384(t1,t1,y1a)

        movq    input_y, %rcx
        montmul_p384(resz,xd,z_2)
        montmul_p384(t2,yd,t2)

        sub_p384(resy,t2,t1)

// Load in the z coordinates of the inputs to check for P1 = 0 and P2 = 0
// The condition codes get set by a comparison (P2 != 0) - (P1 != 0)
// So "NBE" <=> ~(CF \/ ZF) <=> P1 = 0 /\ ~(P2 = 0)
// and "B"  <=> CF          <=> ~(P1 = 0) /\ P2 = 0
// and "Z"  <=> ZF          <=> (P1 = 0 <=> P2 = 0)
// Multiplex the z outputs accordingly and re-store in resz

        movq    input_y, %rcx
        load6(%r8,%r9,%r10,%r11,%rbx,%rbp,z_2)
        movq    %r8, %rax
        movq    %r9, %rdx
        orq     %r10, %rax
        orq     %r11, %rdx
        orq     %rbx, %rax
        orq     %rbp, %rdx
        orq     %rdx, %rax
        negq    %rax
        sbbq    %rax, %rax

        movq    input_x, %rsi
        load6(%r12,%r13,%r14,%r15,%rdx,%rcx,z_1)
        cmovzq  %r12, %r8
        cmovzq  %r13, %r9
        cmovzq  %r14, %r10
        cmovzq  %r15, %r11
        cmovzq  %rdx, %rbx
        cmovzq  %rcx, %rbp
        orq     %r13, %r12
        orq     %r15, %r14
        orq     %rcx, %rdx
        orq     %r14, %r12
        orq     %r12, %rdx
        negq    %rdx
        sbbq    %rdx, %rdx

        cmpq    %rdx, %rax

        czload6(%r8,%r9,%r10,%r11,%rbx,%rbp,resz)
        store6(resz,%r8,%r9,%r10,%r11,%rbx,%rbp)

// Multiplex the x and y outputs too, keeping the results in registers

        movq    input_y, %rcx
        movq    input_x, %rsi
        muxload6(%r8,%r9,%r10,%r11,%rbx,%rbp,resx,x_1,x_2)
        muxload6(%r12,%r13,%r14,%r15,%rdx,%rax,resy,y_1,y_2)

// Finally store back the multiplexed values

        store6(x_3,%r8,%r9,%r10,%r11,%rbx,%rbp)
        load6(%r8,%r9,%r10,%r11,%rbx,%rbp,resz)
        store6(y_3,%r12,%r13,%r14,%r15,%rdx,%rax)
        store6(z_3,%r8,%r9,%r10,%r11,%rbx,%rbp)

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

S2N_BN_SIZE_DIRECTIVE(p384_montjadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
