// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-384 in Montgomery-Jacobian coordinates
//
//    extern void p384_montjdouble_alt(uint64_t p3[static 18],
//                                     const uint64_t p1[static 18]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^384 * x) mod p_384.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard x86-64 ABI: RDI = p3, RSI = p1
// Microsoft x64 ABI:   RCX = p3, RDX = p1
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjdouble_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjdouble_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjdouble_alt)
        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 48

// Pointer-offset pairs for inputs and outputs
// These assume %rdi = p3, %rsi = p1. The latter stays true
// but montsqr below modifies %rdi as well. Thus, we need
// to save %rdi and restore it before the writes to outputs.

#define x_1 0(%rsi)
#define y_1 NUMSIZE(%rsi)
#define z_1 (2*NUMSIZE)(%rsi)

#define x_3 0(%rdi)
#define y_3 NUMSIZE(%rdi)
#define z_3 (2*NUMSIZE)(%rdi)

// Pointer-offset pairs for temporaries, with some aliasing
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

// Safe place for pointer to the output

#define input_z (NUMSIZE*7)(%rsp)

#define NSPACE 344

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

// Corresponds exactly to bignum_montsqr_p384_alt except %rsi -> %rdi

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
        movq    %rdx, %rdi ;                       \
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
        adcq    %r13, %rdi ;                       \
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
        adcq    %rdi, %r13 ;                       \
        adcq    $0x0, %r8 ;                        \
        cmovneq %rax, %r14 ;                       \
        cmovneq %r9, %r15 ;                        \
        cmovneq %r10, %rcx ;                       \
        cmovneq %r11, %rbx ;                       \
        cmovneq %r12, %rbp ;                       \
        cmovneq %r13, %rdi ;                       \
        movq    %r14, P0 ;                      \
        movq    %r15, 0x8+P0 ;                  \
        movq    %rcx, 0x10+P0 ;                 \
        movq    %rbx, 0x18+P0 ;                 \
        movq    %rbp, 0x20+P0 ;                 \
        movq    %rdi, 0x28+P0

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
        movl   $0xffffffff, %ebx ;                  \
        andq   %rbx, %rcx ;                         \
        xorq   %rbx, %rbx ;                         \
        subq   %rcx, %rbx ;                         \
        subq   %rbx, %rax ;                         \
        movq   %rax, P0 ;                        \
        sbbq   %rcx, %rdx ;                         \
        movq   %rdx, 0x8+P0 ;                    \
        sbbq   %rax, %rax ;                         \
        andq   %rbx, %rcx ;                         \
        negq   %rax;                             \
        sbbq   %rcx, %r8 ;                          \
        movq   %r8, 0x10+P0 ;                    \
        sbbq   $0x0, %r9 ;                          \
        movq   %r9, 0x18+P0 ;                    \
        sbbq   $0x0, %r10 ;                         \
        movq   %r10, 0x20+P0 ;                   \
        sbbq   $0x0, %r11 ;                         \
        movq   %r11, 0x28+P0

// Simplified bignum_add_p384, without carry chain suspension

#define add_p384(P0,P1,P2)                      \
        movq   P1, %rax ;                        \
        addq   P2, %rax ;                        \
        movq   0x8+P1, %rcx ;                    \
        adcq   0x8+P2, %rcx ;                    \
        movq   0x10+P1, %r8 ;                    \
        adcq   0x10+P2, %r8 ;                    \
        movq   0x18+P1, %r9 ;                    \
        adcq   0x18+P2, %r9 ;                    \
        movq   0x20+P1, %r10 ;                   \
        adcq   0x20+P2, %r10 ;                   \
        movq   0x28+P1, %r11 ;                   \
        adcq   0x28+P2, %r11 ;                   \
        movl   $0x0, %edx ;                         \
        adcq   %rdx, %rdx ;                         \
        movq   $0xffffffff00000001, %rbp ;          \
        addq   %rbp, %rax ;                         \
        movl   $0xffffffff, %ebp ;                  \
        adcq   %rbp, %rcx ;                         \
        adcq   $0x1, %r8 ;                          \
        adcq   $0x0, %r9 ;                          \
        adcq   $0x0, %r10 ;                         \
        adcq   $0x0, %r11 ;                         \
        adcq   $0xffffffffffffffff, %rdx ;          \
        movl   $1, %ebx ;                           \
        andq   %rdx, %rbx ;                         \
        andq   %rbp, %rdx ;                         \
        xorq   %rbp, %rbp ;                         \
        subq   %rdx, %rbp ;                         \
        subq   %rbp, %rax ;                         \
        movq   %rax, P0 ;                        \
        sbbq   %rdx, %rcx ;                         \
        movq   %rcx, 0x8+P0 ;                    \
        sbbq   %rbx, %r8 ;                          \
        movq   %r8, 0x10+P0 ;                    \
        sbbq   $0x0, %r9 ;                          \
        movq   %r9, 0x18+P0 ;                    \
        sbbq   $0x0, %r10 ;                         \
        movq   %r10, 0x20+P0 ;                   \
        sbbq   $0x0, %r11 ;                         \
        movq   %r11, 0x28+P0

// P0 = 4 * P1 - P2

#define cmsub41_p384(P0,P1,P2)                  \
        movq    40+P1, %rcx ;                   \
        movq    %rcx, %r13 ;                       \
        shrq    $62, %rcx ;                        \
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
        addq    $1, %rcx ;                         \
        subq    P2, %r8 ;                       \
        sbbq    0x8+P2, %r9 ;                   \
        sbbq    0x10+P2, %r10 ;                 \
        sbbq    0x18+P2, %r11 ;                 \
        sbbq    0x20+P2, %r12 ;                 \
        sbbq    0x28+P2, %r13 ;                 \
        sbbq    $0, %rcx ;                          \
        movq    $0xffffffff00000001, %rax ;         \
        mulq    %rcx;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        adcq    %rcx, %r10 ;                        \
        movq    %rcx, %rax ;                        \
        sbbq    %rcx, %rcx ;                        \
        movl    $0xffffffff, %edx ;                 \
        negq    %rcx;                            \
        mulq    %rdx;                            \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    %rcx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        sbbq    %rcx, %rcx ;                        \
        notq    %rcx;                            \
        movl    $0xffffffff, %edx ;                 \
        xorq    %rax, %rax ;                        \
        andq    %rcx, %rdx ;                        \
        subq    %rdx, %rax ;                        \
        andq    $0x1, %rcx ;                        \
        subq    %rax, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    %rdx, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        sbbq    %rcx, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        sbbq    $0x0, %r11 ;                        \
        movq    %r11, 0x18+P0 ;                  \
        sbbq    $0x0, %r12 ;                        \
        movq    %r12, 0x20+P0 ;                  \
        sbbq    $0x0, %r13 ;                        \
        movq    %r13, 0x28+P0

// P0 = C * P1 - D * P2

#define cmsub_p384(P0,C,P1,D,P2)                \
        movq    $0x00000000ffffffff, %r9 ;         \
        subq    P2, %r9 ;                       \
        movq    $0xffffffff00000000, %r10 ;         \
        sbbq    8+P2, %r10 ;                     \
        movq    $0xfffffffffffffffe, %r11 ;        \
        sbbq    16+P2, %r11 ;                   \
        movq    $0xffffffffffffffff, %r12 ;        \
        sbbq    24+P2, %r12 ;                   \
        movq    $0xffffffffffffffff, %r13 ;        \
        sbbq    32+P2, %r13 ;                   \
        movq    $0xffffffffffffffff, %r14 ;        \
        sbbq    40+P2, %r14 ;                   \
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
        movq    %r13, %rax ;                       \
        xorl    %r13d, %r13d ;                     \
        mulq    %rcx;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        movq    %r14, %rax ;                       \
        movl    $1, %r14d ;                        \
        mulq    %rcx;                            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        movl    $C, %ecx ;                         \
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
        sbbq    %rbx, %rbx ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rbx, %rbx ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        movq    $0xffffffff00000001, %rax ;         \
        mulq    %r14;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        adcq    %r14, %r10 ;                        \
        movq    %r14, %rax ;                        \
        sbbq    %rcx, %rcx ;                        \
        movl    $0xffffffff, %edx ;                 \
        negq    %rcx;                            \
        mulq    %rdx;                            \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    %rcx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        sbbq    %rcx, %rcx ;                        \
        notq    %rcx;                            \
        movl    $0xffffffff, %edx ;                 \
        xorq    %rax, %rax ;                        \
        andq    %rcx, %rdx ;                        \
        subq    %rdx, %rax ;                        \
        andq    $0x1, %rcx ;                        \
        subq    %rax, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    %rdx, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        sbbq    %rcx, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        sbbq    $0x0, %r11 ;                        \
        movq    %r11, 0x18+P0 ;                  \
        sbbq    $0x0, %r12 ;                        \
        movq    %r12, 0x20+P0 ;                  \
        sbbq    $0x0, %r13 ;                        \
        movq    %r13, 0x28+P0

// A weak version of add that only guarantees sum in 6 digits

#define weakadd_p384(P0,P1,P2)                  \
        movq   P1, %rax ;                        \
        addq   P2, %rax ;                        \
        movq   0x8+P1, %rcx ;                    \
        adcq   0x8+P2, %rcx ;                    \
        movq   0x10+P1, %r8 ;                    \
        adcq   0x10+P2, %r8 ;                    \
        movq   0x18+P1, %r9 ;                    \
        adcq   0x18+P2, %r9 ;                    \
        movq   0x20+P1, %r10 ;                   \
        adcq   0x20+P2, %r10 ;                   \
        movq   0x28+P1, %r11 ;                   \
        adcq   0x28+P2, %r11 ;                   \
        sbbq   %rdx, %rdx ;                         \
        movl   $1, %ebx ;                           \
        andq   %rdx, %rbx ;                         \
        movl   $0xffffffff, %ebp ;                  \
        andq   %rbp, %rdx ;                         \
        xorq   %rbp, %rbp ;                         \
        subq   %rdx, %rbp ;                         \
        addq   %rbp, %rax ;                         \
        movq   %rax, P0 ;                        \
        adcq   %rdx, %rcx ;                         \
        movq   %rcx, 0x8+P0 ;                    \
        adcq   %rbx, %r8 ;                          \
        movq   %r8, 0x10+P0 ;                    \
        adcq   $0x0, %r9 ;                          \
        movq   %r9, 0x18+P0 ;                    \
        adcq   $0x0, %r10 ;                         \
        movq   %r10, 0x20+P0 ;                   \
        adcq   $0x0, %r11 ;                         \
        movq   %r11, 0x28+P0

// P0 = 3 * P1 - 8 * P2

#define cmsub38_p384(P0,P1,P2)                  \
        movq    $0x00000000ffffffff, %r8 ;         \
        subq    P2, %r8 ;                       \
        movq    $0xffffffff00000000, %r9 ;         \
        sbbq    8+P2, %r9 ;                     \
        movq    $0xfffffffffffffffe, %r10 ;        \
        sbbq    16+P2, %r10 ;                   \
        movq    $0xffffffffffffffff, %r11 ;        \
        sbbq    24+P2, %r11 ;                   \
        movq    $0xffffffffffffffff, %r12 ;        \
        sbbq    32+P2, %r12 ;                   \
        movq    $0xffffffffffffffff, %r13 ;        \
        sbbq    40+P2, %r13 ;                   \
        movq    %r13, %r14 ;                       \
        shrq    $61, %r14 ;                        \
        shldq   $3, %r12, %r13 ;                    \
        shldq   $3, %r11, %r12 ;                    \
        shldq   $3, %r10, %r11 ;                    \
        shldq   $3, %r9, %r10 ;                     \
        shldq   $3, %r8, %r9 ;                      \
        shlq    $3, %r8 ;                          \
        addq    $1, %r14 ;                         \
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
        sbbq    %rbx, %rbx ;                       \
        movq    0x20+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rbx, %rbx ;                       \
        movq    0x28+P1, %rax ;                 \
        mulq    %rcx;                            \
        subq    %rbx, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        movq    $0xffffffff00000001, %rax ;         \
        mulq    %r14;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        adcq    %r14, %r10 ;                        \
        movq    %r14, %rax ;                        \
        sbbq    %rcx, %rcx ;                        \
        movl    $0xffffffff, %edx ;                 \
        negq    %rcx;                            \
        mulq    %rdx;                            \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    %rcx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        sbbq    %rcx, %rcx ;                        \
        notq    %rcx;                            \
        movl    $0xffffffff, %edx ;                 \
        xorq    %rax, %rax ;                        \
        andq    %rcx, %rdx ;                        \
        subq    %rdx, %rax ;                        \
        andq    $0x1, %rcx ;                        \
        subq    %rax, %r8 ;                         \
        movq    %r8, P0 ;                        \
        sbbq    %rdx, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        sbbq    %rcx, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        sbbq    $0x0, %r11 ;                        \
        movq    %r11, 0x18+P0 ;                  \
        sbbq    $0x0, %r12 ;                        \
        movq    %r12, 0x20+P0 ;                  \
        sbbq    $0x0, %r13 ;                        \
        movq    %r13, 0x28+P0

S2N_BN_SYMBOL(p384_montjdouble_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save registers and make room on stack for temporary variables
// Save the output pointer %rdi which gets overwritten in earlier
// operations before it is used.

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

        movq    %rdi, input_z

// Main code, just a sequence of basic field operations

// z2 = z^2
// y2 = y^2

        montsqr_p384(z2,z_1)
        montsqr_p384(y2,y_1)

// x2p = x^2 - z^4 = (x + z^2) * (x - z^2)

        weakadd_p384(t1,x_1,z2)
        sub_p384(t2,x_1,z2)
        montmul_p384(x2p,t1,t2)

// t1 = y + z
// x4p = x2p^2
// xy2 = x * y^2

        add_p384(t1,y_1,z_1)
        montsqr_p384(x4p,x2p)
        montmul_p384(xy2,x_1,y2)

// t2 = (y + z)^2

        montsqr_p384(t2,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_p384(d,12,xy2,9,x4p)
        sub_p384(t1,t2,z2)

// y4 = y^4

        montsqr_p384(y4,y2)

// Restore the output pointer to write to x_3, y_3 and z_3.

        movq    input_z, %rdi

// z_3' = 2 * y * z
// dx2 = d * x2p

        sub_p384(z_3,t1,y2)
        montmul_p384(dx2,d,x2p)

// x' = 4 * xy2 - d

        cmsub41_p384(x_3,xy2,d)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p384(y_3,dx2,y4)

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

S2N_BN_SIZE_DIRECTIVE(p384_montjdouble_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
