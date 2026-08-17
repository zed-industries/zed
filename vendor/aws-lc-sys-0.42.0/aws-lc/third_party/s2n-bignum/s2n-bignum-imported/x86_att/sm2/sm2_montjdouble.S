// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on GM/T 0003-2012 curve SM2 in Montgomery-Jacobian coordinates
//
//    extern void sm2_montjdouble(uint64_t p3[static 12],
//                                const uint64_t p1[static 12]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_sm2.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard x86-64 ABI: RDI = p3, RSI = p1
// Microsoft x64 ABI:   RCX = p3, RDX = p1
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sm2_montjdouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sm2_montjdouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sm2_montjdouble)
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

#define z2 (NUMSIZE*0)(%rsp)
#define y4 (NUMSIZE*0)(%rsp)

#define y2 (NUMSIZE*1)(%rsp)

#define t1 (NUMSIZE*2)(%rsp)

#define t2 (NUMSIZE*3)(%rsp)
#define x2p (NUMSIZE*3)(%rsp)
#define dx2 (NUMSIZE*3)(%rsp)

#define xy2 (NUMSIZE*4)(%rsp)

#define x4p (NUMSIZE*5)(%rsp)
#define d (NUMSIZE*5)(%rsp)

#define NSPACE NUMSIZE*6

// Corresponds to bignum_montmul_sm2 except for registers

#define montmul_sm2(P0,P1,P2)                   \
        xorl    %ecx, %ecx ;                       \
        movq    P2, %rdx ;                      \
        mulxq   P1, %r8, %r9 ;                   \
        mulxq   0x8+P1, %rax, %r10 ;             \
        addq    %rax, %r9 ;                        \
        mulxq   0x10+P1, %rax, %r11 ;            \
        adcq    %rax, %r10 ;                       \
        mulxq   0x18+P1, %rax, %r12 ;            \
        adcq    %rax, %r11 ;                       \
        adcq    %rcx, %r12 ;                       \
        xorl    %ecx, %ecx ;                       \
        movq    0x8+P2, %rdx ;                  \
        mulxq   P1, %rax, %rbx ;                 \
        adcxq   %rax, %r9 ;                        \
        adoxq   %rbx, %r10 ;                       \
        mulxq   0x8+P1, %rax, %rbx ;             \
        adcxq   %rax, %r10 ;                       \
        adoxq   %rbx, %r11 ;                       \
        mulxq   0x10+P1, %rax, %rbx ;            \
        adcxq   %rax, %r11 ;                       \
        adoxq   %rbx, %r12 ;                       \
        mulxq   0x18+P1, %rax, %r13 ;            \
        adcxq   %rax, %r12 ;                       \
        adoxq   %rcx, %r13 ;                       \
        adcxq   %rcx, %r13 ;                       \
        xorl    %ecx, %ecx ;                       \
        movq    0x10+P2, %rdx ;                 \
        mulxq   P1, %rax, %rbx ;                 \
        adcxq   %rax, %r10 ;                       \
        adoxq   %rbx, %r11 ;                       \
        mulxq   0x8+P1, %rax, %rbx ;             \
        adcxq   %rax, %r11 ;                       \
        adoxq   %rbx, %r12 ;                       \
        mulxq   0x10+P1, %rax, %rbx ;            \
        adcxq   %rax, %r12 ;                       \
        adoxq   %rbx, %r13 ;                       \
        mulxq   0x18+P1, %rax, %r14 ;            \
        adcxq   %rax, %r13 ;                       \
        adoxq   %rcx, %r14 ;                       \
        adcxq   %rcx, %r14 ;                       \
        xorl    %ecx, %ecx ;                       \
        movq    0x18+P2, %rdx ;                 \
        mulxq   P1, %rax, %rbx ;                 \
        adcxq   %rax, %r11 ;                       \
        adoxq   %rbx, %r12 ;                       \
        mulxq   0x8+P1, %rax, %rbx ;             \
        adcxq   %rax, %r12 ;                       \
        adoxq   %rbx, %r13 ;                       \
        mulxq   0x10+P1, %rax, %rbx ;            \
        adcxq   %rax, %r13 ;                       \
        adoxq   %rbx, %r14 ;                       \
        mulxq   0x18+P1, %rax, %r15 ;            \
        adcxq   %rax, %r14 ;                       \
        adoxq   %rcx, %r15 ;                       \
        adcxq   %rcx, %r15 ;                       \
        movq    %r8, %rax ;                        \
        shlq    $0x20, %rax ;                      \
        movq    %r8, %rcx ;                        \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r8, %rax ;                        \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r9 ;                        \
        sbbq    %rcx, %r10 ;                       \
        sbbq    %rdx, %r11 ;                       \
        sbbq    %rbx, %r8 ;                        \
        movq    %r9, %rax ;                        \
        shlq    $0x20, %rax ;                      \
        movq    %r9, %rcx ;                        \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r9, %rax ;                        \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r10 ;                       \
        sbbq    %rcx, %r11 ;                       \
        sbbq    %rdx, %r8 ;                        \
        sbbq    %rbx, %r9 ;                        \
        movq    %r10, %rax ;                       \
        shlq    $0x20, %rax ;                      \
        movq    %r10, %rcx ;                       \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r10, %rax ;                       \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r11 ;                       \
        sbbq    %rcx, %r8 ;                        \
        sbbq    %rdx, %r9 ;                        \
        sbbq    %rbx, %r10 ;                       \
        movq    %r11, %rax ;                       \
        shlq    $0x20, %rax ;                      \
        movq    %r11, %rcx ;                       \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r11, %rax ;                       \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r8 ;                        \
        sbbq    %rcx, %r9 ;                        \
        sbbq    %rdx, %r10 ;                       \
        sbbq    %rbx, %r11 ;                       \
        xorl    %eax, %eax ;                       \
        addq    %r8, %r12 ;                        \
        adcq    %r9, %r13 ;                        \
        adcq    %r10, %r14 ;                       \
        adcq    %r11, %r15 ;                       \
        adcq    %rax, %rax ;                       \
        movl    $0x1, %ecx ;                       \
        movl    $0xffffffff, %edx ;                \
        xorl    %ebx, %ebx ;                       \
        addq    %r12, %rcx ;                       \
        leaq    0x1(%rdx), %r11 ;                 \
        adcq    %r13, %rdx ;                       \
        leaq    -0x1(%rbx), %r8 ;                  \
        adcq    %r14, %rbx ;                       \
        adcq    %r15, %r11 ;                       \
        adcq    %rax, %r8 ;                        \
        cmovbq  %rcx, %r12 ;                       \
        cmovbq  %rdx, %r13 ;                       \
        cmovbq  %rbx, %r14 ;                       \
        cmovbq  %r11, %r15 ;                       \
        movq    %r12, P0 ;                      \
        movq    %r13, 0x8+P0 ;                  \
        movq    %r14, 0x10+P0 ;                 \
        movq    %r15, 0x18+P0

// Corresponds to bignum_montsqr_sm2 except for registers

#define montsqr_sm2(P0,P1)                      \
        movq    P1, %rdx ;                      \
        mulxq   %rdx, %r8, %r15 ;                   \
        mulxq   0x8+P1, %r9, %r10 ;              \
        mulxq   0x18+P1, %r11, %r12 ;            \
        movq    0x10+P1, %rdx ;                 \
        mulxq   0x18+P1, %r13, %r14 ;            \
        xorl    %ecx, %ecx ;                       \
        mulxq   P1, %rax, %rbx ;                 \
        adcxq   %rax, %r10 ;                       \
        adoxq   %rbx, %r11 ;                       \
        mulxq   0x8+P1, %rax, %rbx ;             \
        adcxq   %rax, %r11 ;                       \
        adoxq   %rbx, %r12 ;                       \
        movq    0x18+P1, %rdx ;                 \
        mulxq   0x8+P1, %rax, %rbx ;             \
        adcxq   %rax, %r12 ;                       \
        adoxq   %rbx, %r13 ;                       \
        adcxq   %rcx, %r13 ;                       \
        adoxq   %rcx, %r14 ;                       \
        adcq    %rcx, %r14 ;                       \
        xorl    %ecx, %ecx ;                       \
        adcxq   %r9, %r9 ;                         \
        adoxq   %r15, %r9 ;                        \
        movq    0x8+P1, %rdx ;                  \
        mulxq   %rdx, %rax, %rdx ;                  \
        adcxq   %r10, %r10 ;                       \
        adoxq   %rax, %r10 ;                       \
        adcxq   %r11, %r11 ;                       \
        adoxq   %rdx, %r11 ;                       \
        movq    0x10+P1, %rdx ;                 \
        mulxq   %rdx, %rax, %rdx ;                  \
        adcxq   %r12, %r12 ;                       \
        adoxq   %rax, %r12 ;                       \
        adcxq   %r13, %r13 ;                       \
        adoxq   %rdx, %r13 ;                       \
        movq    0x18+P1, %rdx ;                 \
        mulxq   %rdx, %rax, %r15 ;                  \
        adcxq   %r14, %r14 ;                       \
        adoxq   %rax, %r14 ;                       \
        adcxq   %rcx, %r15 ;                       \
        adoxq   %rcx, %r15 ;                       \
        movq    %r8, %rax ;                        \
        shlq    $0x20, %rax ;                      \
        movq    %r8, %rcx ;                        \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r8, %rax ;                        \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r9 ;                        \
        sbbq    %rcx, %r10 ;                       \
        sbbq    %rdx, %r11 ;                       \
        sbbq    %rbx, %r8 ;                        \
        movq    %r9, %rax ;                        \
        shlq    $0x20, %rax ;                      \
        movq    %r9, %rcx ;                        \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r9, %rax ;                        \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r10 ;                       \
        sbbq    %rcx, %r11 ;                       \
        sbbq    %rdx, %r8 ;                        \
        sbbq    %rbx, %r9 ;                        \
        movq    %r10, %rax ;                       \
        shlq    $0x20, %rax ;                      \
        movq    %r10, %rcx ;                       \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r10, %rax ;                       \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r11 ;                       \
        sbbq    %rcx, %r8 ;                        \
        sbbq    %rdx, %r9 ;                        \
        sbbq    %rbx, %r10 ;                       \
        movq    %r11, %rax ;                       \
        shlq    $0x20, %rax ;                      \
        movq    %r11, %rcx ;                       \
        shrq    $0x20, %rcx ;                      \
        movq    %rax, %rdx ;                       \
        movq    %rcx, %rbx ;                       \
        subq    %r11, %rax ;                       \
        sbbq    $0x0, %rcx ;                       \
        subq    %rax, %r8 ;                        \
        sbbq    %rcx, %r9 ;                        \
        sbbq    %rdx, %r10 ;                       \
        sbbq    %rbx, %r11 ;                       \
        xorl    %eax, %eax ;                       \
        addq    %r8, %r12 ;                        \
        adcq    %r9, %r13 ;                        \
        adcq    %r10, %r14 ;                       \
        adcq    %r11, %r15 ;                       \
        adcq    %rax, %rax ;                       \
        movl    $0x1, %ecx ;                       \
        movl    $0xffffffff, %edx ;                \
        xorl    %ebx, %ebx ;                       \
        addq    %r12, %rcx ;                       \
        leaq    0x1(%rdx), %r11 ;                 \
        adcq    %r13, %rdx ;                       \
        leaq    -0x1(%rbx), %r8 ;                  \
        adcq    %r14, %rbx ;                       \
        adcq    %r15, %r11 ;                       \
        adcq    %rax, %r8 ;                        \
        cmovbq  %rcx, %r12 ;                       \
        cmovbq  %rdx, %r13 ;                       \
        cmovbq  %rbx, %r14 ;                       \
        cmovbq  %r11, %r15 ;                       \
        movq    %r12, P0 ;                      \
        movq    %r13, 0x8+P0 ;                  \
        movq    %r14, 0x10+P0 ;                 \
        movq    %r15, 0x18+P0

// Corresponds exactly to bignum_sub_sm2

#define sub_sm2(P0,P1,P2)                       \
        movq    P1, %rax ;                      \
        subq    P2, %rax ;                      \
        movq    0x8+P1, %rcx ;                  \
        sbbq    0x8+P2, %rcx ;                  \
        movq    0x10+P1, %r8 ;                  \
        sbbq    0x10+P2, %r8 ;                  \
        movq    0x18+P1, %r9 ;                  \
        sbbq    0x18+P2, %r9 ;                  \
        movq    $0xffffffff00000000, %r10 ;        \
        sbbq    %r11, %r11 ;                       \
        andq    %r11, %r10 ;                       \
        movq    %r11, %rdx ;                       \
        btr     $0x20, %rdx ;                      \
        addq    %r11, %rax ;                       \
        movq    %rax, P0 ;                      \
        adcq    %r10, %rcx ;                       \
        movq    %rcx, 0x8+P0 ;                  \
        adcq    %r11, %r8 ;                        \
        movq    %r8, 0x10+P0 ;                  \
        adcq    %rdx, %r9 ;                        \
        movq    %r9, 0x18+P0

// Corresponds exactly to bignum_add_sm2

#define add_sm2(P0,P1,P2)                       \
        xorq    %r11, %r11 ;                       \
        movq    P1, %rax ;                      \
        addq    P2, %rax ;                      \
        movq    0x8+P1, %rcx ;                  \
        adcq    0x8+P2, %rcx ;                  \
        movq    0x10+P1, %r8 ;                  \
        adcq    0x10+P2, %r8 ;                  \
        movq    0x18+P1, %r9 ;                  \
        adcq    0x18+P2, %r9 ;                  \
        adcq    %r11, %r11 ;                       \
        subq    $0xffffffffffffffff, %rax ;        \
        movq    $0xffffffff00000000, %r10 ;        \
        sbbq    %r10, %rcx ;                       \
        sbbq    $0xffffffffffffffff, %r8 ;         \
        movq    $0xfffffffeffffffff, %rdx ;        \
        sbbq    %rdx, %r9 ;                        \
        sbbq    $0x0, %r11 ;                       \
        andq    %r11, %r10 ;                       \
        andq    %r11, %rdx ;                       \
        addq    %r11, %rax ;                       \
        movq    %rax, P0 ;                      \
        adcq    %r10, %rcx ;                       \
        movq    %rcx, 0x8+P0 ;                  \
        adcq    %r11, %r8 ;                        \
        movq    %r8, 0x10+P0 ;                  \
        adcq    %rdx, %r9 ;                        \
        movq    %r9, 0x18+P0

// A weak version of add that only guarantees sum in 4 digits

#define weakadd_sm2(P0,P1,P2)                   \
        movq    P1, %rax ;                      \
        addq    P2, %rax ;                      \
        movq    0x8+P1, %rcx ;                  \
        adcq    0x8+P2, %rcx ;                  \
        movq    0x10+P1, %r8 ;                  \
        adcq    0x10+P2, %r8 ;                  \
        movq    0x18+P1, %r9 ;                  \
        adcq    0x18+P2, %r9 ;                  \
        movq    $0xffffffff00000000, %r10 ;        \
        sbbq    %r11, %r11 ;                       \
        andq    %r11, %r10 ;                       \
        movq    %r11, %rdx ;                       \
        btr     $0x20, %rdx ;                      \
        subq    %r11, %rax ;                       \
        movq    %rax, P0 ;                      \
        sbbq    %r10, %rcx ;                       \
        movq    %rcx, 0x8+P0 ;                  \
        sbbq    %r11, %r8 ;                        \
        movq    %r8, 0x10+P0 ;                  \
        sbbq    %rdx, %r9 ;                        \
        movq    %r9, 0x18+P0

// P0 = C * P1 - D * P2  computed as d * (p_sm2 - P2) + c * P1
// Quotient estimation is done just as q = h + 1 as in bignum_triple_sm2
// This also applies to the other functions following.

#define cmsub_sm2(P0,C,P1,D,P2)                \
        /* First (%r11;%r10;%r9;%r8) = p_sm2 - P2 */ \
        movq    $0xffffffffffffffff, %r8 ;          \
        movq    %r8, %r10 ;                        \
        subq    P2, %r8 ;                        \
        movq    $0xffffffff00000000, %r9 ;          \
        sbbq    0x8+P2, %r9 ;                    \
        sbbq    0x10+P2, %r10 ;                  \
        movq    $0xfffffffeffffffff, %r11 ;         \
        sbbq    0x18+P2, %r11 ;                  \
        /* (%r12;%r11;%r10;%r9;%r8) = D * (p_sm2 - P2) */  \
        xorl    %r12d, %r12d ;                      \
        movq    $D, %rdx ;                         \
        mulxq   %r8, %r8, %rax ;                    \
        mulxq   %r9, %r9, %rcx ;                    \
        addq    %rax, %r9 ;                        \
        mulxq   %r10, %r10, %rax ;                  \
        adcq    %rcx, %r10 ;                       \
        mulxq   %r11, %r11, %rcx ;                  \
        adcq    %rax, %r11 ;                       \
        adcq    %rcx, %r12 ;                       \
        /* (%rdx;%r11;%r10;%r9;%r8) = 2^256 + C * P1 + D * (p_sm2 - P2) */ \
        movq    $C, %rdx ;                         \
        xorl    %eax, %eax ;                       \
        mulxq   P1, %rax, %rcx ;                 \
        adcxq   %rax, %r8 ;                        \
        adoxq   %rcx, %r9 ;                        \
        mulxq   0x8+P1, %rax, %rcx ;             \
        adcxq   %rax, %r9 ;                        \
        adoxq   %rcx, %r10 ;                       \
        mulxq   0x10+P1, %rax, %rcx ;            \
        adcxq   %rax, %r10 ;                       \
        adoxq   %rcx, %r11 ;                       \
        mulxq   0x18+P1, %rax, %rdx ;            \
        adcxq   %rax, %r11 ;                       \
        adoxq   %r12, %rdx ;                       \
        adcq    $1, %rdx ;                         \
        /* Now the tail for modular reduction from tripling */ \
        movq    %rdx, %rax ;                       \
        shlq    $0x20, %rax ;                      \
        movq    %rax, %rcx ;                       \
        subq    %rdx, %rax ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rax, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        sbbq    %rdx, %rdx ;                       \
        notq    %rdx;                            \
        movq    $0xffffffff00000000, %rax ;        \
        andq    %rdx, %rax ;                       \
        movq    %rdx, %rcx ;                       \
        btr     $0x20, %rcx ;                      \
        addq    %rdx, %r8 ;                        \
        movq    %r8, P0 ;                       \
        adcq    %rax, %r9 ;                        \
        movq    %r9, 0x8+P0 ;                   \
        adcq    %rdx, %r10 ;                       \
        movq    %r10, 0x10+P0 ;                 \
        adcq    %rcx, %r11 ;                       \
        movq    %r11, 0x18+P0

// P0 = 3 * P1 - 8 * P2, computed as (p_sm2 - P2) << 3 + 3 * P1

#define cmsub38_sm2(P0,P1,P2)                  \
        /* First (%r11;%r10;%r9;%r8) = p_sm2 - P2 */ \
        movq    $0xffffffffffffffff, %r8 ;          \
        movq    %r8, %r10 ;                        \
        subq    P2, %r8 ;                        \
        movq    $0xffffffff00000000, %r9 ;          \
        sbbq    0x8+P2, %r9 ;                    \
        sbbq    0x10+P2, %r10 ;                  \
        movq    $0xfffffffeffffffff, %r11 ;         \
        sbbq    0x18+P2, %r11 ;                  \
        /* (%r12;%r11;%r10;%r9;%r8) = (p_sm2 - P2) << 3 */  \
        movq    %r11, %r12 ;                       \
        shldq   $3, %r10, %r11 ;                    \
        shldq   $3, %r9, %r10 ;                     \
        shldq   $3, %r8, %r9 ;                      \
        shlq    $3, %r8 ;                          \
        shrq    $61, %r12 ;                        \
        /* (%rdx;%r11;%r10;%r9;%r8) = 2^256 + 3 * P1 + 8 * (p_sm2 - P2) */ \
        movq    $3, %rdx ;                         \
        xorl    %eax, %eax ;                       \
        mulxq   P1, %rax, %rcx ;                 \
        adcxq   %rax, %r8 ;                        \
        adoxq   %rcx, %r9 ;                        \
        mulxq   0x8+P1, %rax, %rcx ;             \
        adcxq   %rax, %r9 ;                        \
        adoxq   %rcx, %r10 ;                       \
        mulxq   0x10+P1, %rax, %rcx ;            \
        adcxq   %rax, %r10 ;                       \
        adoxq   %rcx, %r11 ;                       \
        mulxq   0x18+P1, %rax, %rdx ;            \
        adcxq   %rax, %r11 ;                       \
        adoxq   %r12, %rdx ;                       \
        adcq    $1, %rdx ;                         \
        /* Now the tail for modular reduction from tripling */ \
        movq    %rdx, %rax ;                       \
        shlq    $0x20, %rax ;                      \
        movq    %rax, %rcx ;                       \
        subq    %rdx, %rax ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rax, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        sbbq    %rdx, %rdx ;                       \
        notq    %rdx;                            \
        movq    $0xffffffff00000000, %rax ;        \
        andq    %rdx, %rax ;                       \
        movq    %rdx, %rcx ;                       \
        btr     $0x20, %rcx ;                      \
        addq    %rdx, %r8 ;                        \
        movq    %r8, P0 ;                       \
        adcq    %rax, %r9 ;                        \
        movq    %r9, 0x8+P0 ;                   \
        adcq    %rdx, %r10 ;                       \
        movq    %r10, 0x10+P0 ;                 \
        adcq    %rcx, %r11 ;                       \
        movq    %r11, 0x18+P0

// P0 = 4 * P1 - P2, by direct subtraction of P2,
// since the quotient estimate still works safely
// for initial value > -p_sm2

#define cmsub41_sm2(P0,P1,P2)                  \
        movq    0x18+P1, %r11 ;                  \
        movq    %r11, %rdx ;                       \
        movq    0x10+P1, %r10 ;                  \
        shldq   $2, %r10, %r11 ;                    \
        movq    0x8+P1, %r9 ;                    \
        shldq   $2, %r9, %r10 ;                     \
        movq    P1, %r8 ;                        \
        shldq   $2, %r8, %r9 ;                      \
        shlq    $2, %r8 ;                          \
        shrq    $62, %rdx ;                        \
        addq    $1, %rdx ;                         \
        subq    P2, %r8 ;                       \
        sbbq    0x8+P2, %r9 ;                   \
        sbbq    0x10+P2, %r10 ;                 \
        sbbq    0x18+P2, %r11 ;                 \
        sbbq    $0, %rdx ;                         \
        /* Now the tail for modular reduction from tripling */ \
        movq    %rdx, %rax ;                       \
        shlq    $0x20, %rax ;                      \
        movq    %rax, %rcx ;                       \
        subq    %rdx, %rax ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rax, %r9 ;                        \
        adcq    $0x0, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        sbbq    %rdx, %rdx ;                       \
        notq    %rdx;                            \
        movq    $0xffffffff00000000, %rax ;        \
        andq    %rdx, %rax ;                       \
        movq    %rdx, %rcx ;                       \
        btr     $0x20, %rcx ;                      \
        addq    %rdx, %r8 ;                        \
        movq    %r8, P0 ;                       \
        adcq    %rax, %r9 ;                        \
        movq    %r9, 0x8+P0 ;                   \
        adcq    %rdx, %r10 ;                       \
        movq    %r10, 0x10+P0 ;                 \
        adcq    %rcx, %r11 ;                       \
        movq    %r11, 0x18+P0

S2N_BN_SYMBOL(sm2_montjdouble):
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

        montsqr_sm2(z2,z_1)
        montsqr_sm2(y2,y_1)

// x2p = x^2 - z^4 = (x + z^2) * (x - z^2)

        sub_sm2(t2,x_1,z2)
        weakadd_sm2(t1,x_1,z2)
        montmul_sm2(x2p,t1,t2)

// t1 = y + z
// xy2 = x * y^2
// x4p = x2p^2

        add_sm2(t1,y_1,z_1)
        montmul_sm2(xy2,x_1,y2)
        montsqr_sm2(x4p,x2p)

// t1 = (y + z)^2

        montsqr_sm2(t1,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_sm2(d,12,xy2,9,x4p)
        sub_sm2(t1,t1,z2)

// y4 = y^4

        montsqr_sm2(y4,y2)

// dx2 = d * x2p

        montmul_sm2(dx2,d,x2p)

// z_3' = 2 * y * z

        sub_sm2(z_3,t1,y2)

// x' = 4 * xy2 - d

        cmsub41_sm2(x_3,xy2,d)

// y' = 3 * dx2 - 8 * y4

        cmsub38_sm2(y_3,dx2,y4)

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

S2N_BN_SIZE_DIRECTIVE(sm2_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
