// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point doubling on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjdouble(uint64_t p3[static 12],
//                                 const uint64_t p1[static 12]);
//
// Does p3 := 2 * p1 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
//
// Standard x86-64 ABI: RDI = p3, RSI = p1
// Microsoft x64 ABI:   RCX = p3, RDX = p1
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjdouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjdouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjdouble)
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

// Corresponds exactly to bignum_montmul_p256

#define montmul_p256(P0,P1,P2)                  \
        xorl    %r13d, %r13d ;                      \
        movq    P2, %rdx ;                       \
        mulxq   P1, %r8, %r9 ;                     \
        mulxq   0x8+P1, %rbx, %r10 ;               \
        adcq    %rbx, %r9 ;                         \
        mulxq   0x10+P1, %rbx, %r11 ;              \
        adcq    %rbx, %r10 ;                        \
        mulxq   0x18+P1, %rbx, %r12 ;              \
        adcq    %rbx, %r11 ;                        \
        adcq    %r13, %r12 ;                        \
        movq    0x8+P2, %rdx ;                   \
        xorl    %r14d, %r14d ;                      \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        adcq    %r14, %r13 ;                        \
        xorl    %r15d, %r15d ;                      \
        movabsq $0x100000000, %rdx ;                \
        mulxq   %r8, %rax, %rbx ;                     \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   %r9, %rax, %rbx ;                     \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        notq    %rdx;                            \
        leaq    0x2(%rdx), %rdx ;                  \
        mulxq   %r8, %rax, %rbx ;                     \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   %r9, %rax, %rbx ;                     \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        adcxq   %r15, %r13 ;                        \
        adoxq   %r15, %r14 ;                        \
        adcq    %r15, %r14 ;                        \
        movq    0x10+P2, %rdx ;                  \
        xorl    %r8d, %r8d ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        adoxq   %r8, %r14 ;                         \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcq    %rax, %r13 ;                        \
        adcq    %rbx, %r14 ;                        \
        adcq    %r8, %r15 ;                         \
        movq    0x18+P2, %rdx ;                  \
        xorl    %r9d, %r9d ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        mulxq   0x10+P1, %rax, %rbx ;              \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        adoxq   %r9, %r15 ;                         \
        mulxq   0x18+P1, %rax, %rbx ;              \
        adcq    %rax, %r14 ;                        \
        adcq    %rbx, %r15 ;                        \
        adcq    %r9, %r8 ;                          \
        xorl    %r9d, %r9d ;                        \
        movabsq $0x100000000, %rdx ;                \
        mulxq   %r10, %rax, %rbx ;                    \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   %r11, %rax, %rbx ;                    \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        notq    %rdx;                            \
        leaq    0x2(%rdx), %rdx ;                  \
        mulxq   %r10, %rax, %rbx ;                    \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   %r11, %rax, %rbx ;                    \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        adcxq   %r9, %r15 ;                         \
        adoxq   %r9, %r8 ;                          \
        adcq    %r9, %r8 ;                          \
        movl    $0x1, %ecx ;                        \
        addq    %r12, %rcx ;                        \
        decq    %rdx;                            \
        adcq    %r13, %rdx ;                        \
        decq    %r9;                             \
        movq    %r9, %rax ;                         \
        adcq    %r14, %r9 ;                         \
        movl    $0xfffffffe, %r11d ;                \
        adcq    %r15, %r11 ;                        \
        adcq    %r8, %rax ;                         \
        cmovbq  %rcx, %r12 ;                        \
        cmovbq  %rdx, %r13 ;                        \
        cmovbq  %r9, %r14 ;                         \
        cmovbq  %r11, %r15 ;                        \
        movq    %r12, P0 ;                       \
        movq    %r13, 0x8+P0 ;                   \
        movq    %r14, 0x10+P0 ;                  \
        movq    %r15, 0x18+P0

// Corresponds exactly to bignum_montsqr_p256

#define montsqr_p256(P0,P1)                     \
        movq    P1, %rdx ;                       \
        mulxq   %rdx, %r8, %r15 ;                     \
        mulxq   0x8+P1, %r9, %r10 ;                \
        mulxq   0x18+P1, %r11, %r12 ;              \
        movq    0x10+P1, %rdx ;                  \
        mulxq   0x18+P1, %r13, %r14 ;              \
        xorl    %ebp, %ebp ;                        \
        mulxq   P1, %rax, %rbx ;                   \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        movq    0x18+P1, %rdx ;                  \
        mulxq   0x8+P1, %rax, %rbx ;               \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        adcxq   %rbp, %r13 ;                        \
        adoxq   %rbp, %r14 ;                        \
        adcq    %rbp, %r14 ;                        \
        xorl    %ebp, %ebp ;                        \
        adcxq   %r9, %r9 ;                          \
        adoxq   %r15, %r9 ;                         \
        movq    0x8+P1, %rdx ;                   \
        mulxq   %rdx, %rax, %rdx ;                    \
        adcxq   %r10, %r10 ;                        \
        adoxq   %rax, %r10 ;                        \
        adcxq   %r11, %r11 ;                        \
        adoxq   %rdx, %r11 ;                        \
        movq    0x10+P1, %rdx ;                  \
        mulxq   %rdx, %rax, %rdx ;                    \
        adcxq   %r12, %r12 ;                        \
        adoxq   %rax, %r12 ;                        \
        adcxq   %r13, %r13 ;                        \
        adoxq   %rdx, %r13 ;                        \
        movq    0x18+P1, %rdx ;                  \
        mulxq   %rdx, %rax, %r15 ;                    \
        adcxq   %r14, %r14 ;                        \
        adoxq   %rax, %r14 ;                        \
        adcxq   %rbp, %r15 ;                        \
        adoxq   %rbp, %r15 ;                        \
        xorl    %ebp, %ebp ;                        \
        movabsq $0x100000000, %rdx ;                \
        mulxq   %r8, %rax, %rbx ;                     \
        adcxq   %rax, %r9 ;                         \
        adoxq   %rbx, %r10 ;                        \
        mulxq   %r9, %rax, %rbx ;                     \
        adcxq   %rax, %r10 ;                        \
        adoxq   %rbx, %r11 ;                        \
        movabsq $0xffffffff00000001, %rdx ;         \
        mulxq   %r8, %rax, %rbx ;                     \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   %r9, %rax, %rbx ;                     \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        adcxq   %rbp, %r13 ;                        \
        movl    %ebp, %r9d ;                        \
        adoxq   %rbp, %r9 ;                         \
        adcxq   %rbp, %r9 ;                         \
        addq    %r9, %r14 ;                         \
        adcq    %rbp, %r15 ;                        \
        movl    %ebp, %r8d ;                        \
        adcq    %rbp, %r8 ;                         \
        xorl    %ebp, %ebp ;                        \
        movabsq $0x100000000, %rdx ;                \
        mulxq   %r10, %rax, %rbx ;                    \
        adcxq   %rax, %r11 ;                        \
        adoxq   %rbx, %r12 ;                        \
        mulxq   %r11, %rax, %rbx ;                    \
        adcxq   %rax, %r12 ;                        \
        adoxq   %rbx, %r13 ;                        \
        movabsq $0xffffffff00000001, %rdx ;         \
        mulxq   %r10, %rax, %rbx ;                    \
        adcxq   %rax, %r13 ;                        \
        adoxq   %rbx, %r14 ;                        \
        mulxq   %r11, %rax, %rbx ;                    \
        adcxq   %rax, %r14 ;                        \
        adoxq   %rbx, %r15 ;                        \
        adcxq   %rbp, %r15 ;                        \
        adoxq   %rbp, %r8 ;                         \
        adcq    %rbp, %r8 ;                         \
        movl    $0x1, %ecx ;                        \
        addq    %r12, %rcx ;                        \
        leaq    -0x1(%rdx), %rdx ;                  \
        adcq    %r13, %rdx ;                        \
        leaq    -0x1(%rbp), %rbp ;                  \
        movq    %rbp, %rax ;                        \
        adcq    %r14, %rbp ;                        \
        movl    $0xfffffffe, %r11d ;                \
        adcq    %r15, %r11 ;                        \
        adcq    %r8, %rax ;                         \
        cmovbq  %rcx, %r12 ;                        \
        cmovbq  %rdx, %r13 ;                        \
        cmovbq  %rbp, %r14 ;                        \
        cmovbq  %r11, %r15 ;                        \
        movq    %r12, P0 ;                       \
        movq    %r13, 0x8+P0 ;                   \
        movq    %r14, 0x10+P0 ;                  \
        movq    %r15, 0x18+P0

// Corresponds exactly to bignum_sub_p256

#define sub_p256(P0,P1,P2)                      \
        movq    P1, %rax ;                       \
        subq    P2, %rax ;                       \
        movq    0x8+P1, %rcx ;                   \
        sbbq    0x8+P2, %rcx ;                   \
        movq    0x10+P1, %r8 ;                   \
        sbbq    0x10+P2, %r8 ;                   \
        movq    0x18+P1, %r9 ;                   \
        sbbq    0x18+P2, %r9 ;                   \
        movl    $0xffffffff, %r10d ;                \
        sbbq    %r11, %r11 ;                        \
        xorq    %rdx, %rdx ;                        \
        andq    %r11, %r10 ;                        \
        subq    %r10, %rdx ;                        \
        addq    %r11, %rax ;                        \
        movq    %rax, P0 ;                       \
        adcq    %r10, %rcx ;                        \
        movq    %rcx, 0x8+P0 ;                   \
        adcq    $0x0, %r8 ;                         \
        movq    %r8, 0x10+P0 ;                   \
        adcq    %rdx, %r9 ;                         \
        movq    %r9, 0x18+P0

// Corresponds exactly to bignum_add_p256

#define add_p256(P0,P1,P2)                      \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                       \
        addq    P2, %rax ;                       \
        movq    0x8+P1, %rcx ;                   \
        adcq    0x8+P2, %rcx ;                   \
        movq    0x10+P1, %r8 ;                   \
        adcq    0x10+P2, %r8 ;                   \
        movq    0x18+P1, %r9 ;                   \
        adcq    0x18+P2, %r9 ;                   \
        adcq    %r11, %r11 ;                        \
        subq    $0xffffffffffffffff, %rax ;         \
        movl    $0xffffffff, %r10d ;                \
        sbbq    %r10, %rcx ;                        \
        sbbq    $0x0, %r8 ;                         \
        movq    $0xffffffff00000001, %rdx ;         \
        sbbq    %rdx, %r9 ;                         \
        sbbq    $0x0, %r11 ;                        \
        andq    %r11, %r10 ;                        \
        andq    %r11, %rdx ;                        \
        addq    %r11, %rax ;                        \
        movq    %rax, P0 ;                       \
        adcq    %r10, %rcx ;                        \
        movq    %rcx, 0x8+P0 ;                   \
        adcq    $0x0, %r8 ;                         \
        movq    %r8, 0x10+P0 ;                   \
        adcq    %rdx, %r9 ;                         \
        movq    %r9, 0x18+P0

// A weak version of add that only guarantees sum in 4 digits

#define weakadd_p256(P0,P1,P2)                  \
        movq    P1, %rax ;                       \
        addq    P2, %rax ;                       \
        movq    0x8+P1, %rcx ;                   \
        adcq    0x8+P2, %rcx ;                   \
        movq    0x10+P1, %r8 ;                   \
        adcq    0x10+P2, %r8 ;                   \
        movq    0x18+P1, %r9 ;                   \
        adcq    0x18+P2, %r9 ;                   \
        movl    $0xffffffff, %r10d ;                \
        sbbq    %r11, %r11 ;                        \
        xorq    %rdx, %rdx ;                        \
        andq    %r11, %r10 ;                        \
        subq    %r10, %rdx ;                        \
        subq    %r11, %rax ;                        \
        movq    %rax, P0 ;                       \
        sbbq    %r10, %rcx ;                        \
        movq    %rcx, 0x8+P0 ;                   \
        sbbq    $0x0, %r8 ;                         \
        movq    %r8, 0x10+P0 ;                   \
        sbbq    %rdx, %r9 ;                         \
        movq    %r9, 0x18+P0

// P0 = C * P1 - D * P2  computed as d * (p_256 - P2) + c * P1
// Quotient estimation is done just as q = h + 1 as in bignum_triple_p256
// This also applies to the other functions following.

#define cmsub_p256(P0,C,P1,D,P2)                \
        /* First (%r11;%r10;%r9;%r8) = p_256 - P2 */ \
        movq    $0xffffffffffffffff, %r8 ;          \
        xorl    %r10d, %r10d ;                      \
        subq    P2, %r8 ;                        \
        movq    $0x00000000ffffffff, %r9 ;          \
        sbbq    0x8+P2, %r9 ;                    \
        sbbq    0x10+P2, %r10 ;                  \
        movq    $0xffffffff00000001, %r11 ;         \
        sbbq    0x18+P2, %r11 ;                  \
        /* (%r12;%r11;%r10;%r9;%r8) = D * (p_256 - P2) */  \
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
        /* (%rdx;%r11;%r10;%r9;%r8) = 2^256 + C * P1 + D * (p_256 - P2) */ \
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
        addq    %rdx, %r8 ;                         \
        movq    $0x100000000, %rax ;                \
        mulxq   %rax, %rax, %rcx ;                    \
        sbbq    $0x0, %rax ;                        \
        sbbq    $0x0, %rcx ;                        \
        subq    %rax, %r9 ;                         \
        sbbq    %rcx, %r10 ;                        \
        movq    $0xffffffff00000001, %rax ;         \
        mulxq   %rax, %rax, %rcx ;                    \
        sbbq    %rax, %r11 ;                        \
        sbbq    %rcx, %rdx ;                        \
        decq    %rdx;                            \
        movl    $0xffffffff, %eax ;                 \
        andq    %rdx, %rax ;                        \
        xorl    %ecx, %ecx ;                        \
        subq    %rax, %rcx ;                        \
        addq    %rdx, %r8 ;                         \
        movq    %r8, P0 ;                        \
        adcq    %rax, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        adcq    $0x0, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        adcq    %rcx, %r11 ;                        \
        movq    %r11, 0x18+P0

// P0 = 3 * P1 - 8 * P2, computed as (p_256 - P2) << 3 + 3 * P1

#define cmsub38_p256(P0,P1,P2)                  \
        /* First (%r11;%r10;%r9;%r8) = p_256 - P2 */ \
        movq    $0xffffffffffffffff, %r8 ;          \
        xorl    %r10d, %r10d ;                      \
        subq    P2, %r8 ;                        \
        movq    $0x00000000ffffffff, %r9 ;          \
        sbbq    0x8+P2, %r9 ;                    \
        sbbq    0x10+P2, %r10 ;                  \
        movq    $0xffffffff00000001, %r11 ;         \
        sbbq    0x18+P2, %r11 ;                  \
        /* (%r12;%r11;%r10;%r9;%r8) = (p_256 - P2) << 3 */  \
        movq    %r11, %r12 ;                       \
        shldq   $3, %r10, %r11 ;                    \
        shldq   $3, %r9, %r10 ;                     \
        shldq   $3, %r8, %r9 ;                      \
        shlq    $3, %r8 ;                          \
        shrq    $61, %r12 ;                        \
        /* (%rdx;%r11;%r10;%r9;%r8) = 2^256 + 3 * P1 + 8 * (p_256 - P2) */ \
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
        addq    %rdx, %r8 ;                         \
        movq    $0x100000000, %rax ;                \
        mulxq   %rax, %rax, %rcx ;                    \
        sbbq    $0x0, %rax ;                        \
        sbbq    $0x0, %rcx ;                        \
        subq    %rax, %r9 ;                         \
        sbbq    %rcx, %r10 ;                        \
        movq    $0xffffffff00000001, %rax ;         \
        mulxq   %rax, %rax, %rcx ;                    \
        sbbq    %rax, %r11 ;                        \
        sbbq    %rcx, %rdx ;                        \
        decq    %rdx;                            \
        movl    $0xffffffff, %eax ;                 \
        andq    %rdx, %rax ;                        \
        xorl    %ecx, %ecx ;                        \
        subq    %rax, %rcx ;                        \
        addq    %rdx, %r8 ;                         \
        movq    %r8, P0 ;                        \
        adcq    %rax, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        adcq    $0x0, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        adcq    %rcx, %r11 ;                        \
        movq    %r11, 0x18+P0

// P0 = 4 * P1 - P2, by direct subtraction of P2,
// since the quotient estimate still works safely
// for initial value > -p_256

#define cmsub41_p256(P0,P1,P2)                  \
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
        addq    %rdx, %r8 ;                         \
        movq    $0x100000000, %rax ;                \
        mulxq   %rax, %rax, %rcx ;                    \
        sbbq    $0x0, %rax ;                        \
        sbbq    $0x0, %rcx ;                        \
        subq    %rax, %r9 ;                         \
        sbbq    %rcx, %r10 ;                        \
        movq    $0xffffffff00000001, %rax ;         \
        mulxq   %rax, %rax, %rcx ;                    \
        sbbq    %rax, %r11 ;                        \
        sbbq    %rcx, %rdx ;                        \
        decq    %rdx;                            \
        movl    $0xffffffff, %eax ;                 \
        andq    %rdx, %rax ;                        \
        xorl    %ecx, %ecx ;                        \
        subq    %rax, %rcx ;                        \
        addq    %rdx, %r8 ;                         \
        movq    %r8, P0 ;                        \
        adcq    %rax, %r9 ;                         \
        movq    %r9, 0x8+P0 ;                    \
        adcq    $0x0, %r10 ;                        \
        movq    %r10, 0x10+P0 ;                  \
        adcq    %rcx, %r11 ;                        \
        movq    %r11, 0x18+P0

S2N_BN_SYMBOL(p256_montjdouble):
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

        montsqr_p256(z2,z_1)
        montsqr_p256(y2,y_1)

// x2p = x^2 - z^4 = (x + z^2) * (x - z^2)

        sub_p256(t2,x_1,z2)
        weakadd_p256(t1,x_1,z2)
        montmul_p256(x2p,t1,t2)

// t1 = y + z
// xy2 = x * y^2
// x4p = x2p^2

        add_p256(t1,y_1,z_1)
        montmul_p256(xy2,x_1,y2)
        montsqr_p256(x4p,x2p)

// t1 = (y + z)^2

        montsqr_p256(t1,t1)

// d = 12 * xy2 - 9 * x4p
// t1 = y^2 + 2 * y * z

        cmsub_p256(d,12,xy2,9,x4p)
        sub_p256(t1,t1,z2)

// y4 = y^4

        montsqr_p256(y4,y2)

// dx2 = d * x2p

        montmul_p256(dx2,d,x2p)

// z_3' = 2 * y * z

        sub_p256(z_3,t1,y2)

// x' = 4 * xy2 - d

        cmsub41_p256(x_3,xy2,d)

// y' = 3 * dx2 - 8 * y4

        cmsub38_p256(y_3,dx2,y4)

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

S2N_BN_SIZE_DIRECTIVE(p256_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
