// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjmixadd(uint64_t p3[static 12],
//                                 const uint64_t p1[static 12],
//                                 const uint64_t p2[static 8]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity.
//
// Standard x86-64 ABI: RDI = p3, RSI = p1, RDX = p2
// Microsoft x64 ABI:   RCX = p3, RDX = p1, R8 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjmixadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjmixadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjmixadd)
        .text

// Size of individual field elements

#define NUMSIZE 32

// Pointer-offset pairs for inputs and outputs.
// These assume %rdi = p3, %rsi = p1 and %rbp = p2,
// which needs to be set up explicitly before use.
// The first two hold initially, and the second is
// set up by copying the initial %rdx input to %rbp.
// Thereafter, no code macro modifies any of them.

#define x_1 0(%rsi)
#define y_1 NUMSIZE(%rsi)
#define z_1 (2*NUMSIZE)(%rsi)

#define x_2 0(%rbp)
#define y_2 NUMSIZE(%rbp)

#define x_3 0(%rdi)
#define y_3 NUMSIZE(%rdi)
#define z_3 (2*NUMSIZE)(%rdi)

// Pointer-offset pairs for temporaries, with some aliasing.
// NSPACE is the total stack needed for all temporaries.

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

// Corresponds exactly to bignum_montsqr_p256 except for
// register tweaks to avoid modifying %rbp.

#define montsqr_p256(P0,P1)                     \
        movq    P1, %rdx ;                       \
        mulxq   %rdx, %r8, %r15 ;                     \
        mulxq   0x8+P1, %r9, %r10 ;                \
        mulxq   0x18+P1, %r11, %r12 ;              \
        movq    0x10+P1, %rdx ;                  \
        mulxq   0x18+P1, %r13, %r14 ;              \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r13 ;                        \
        adoxq   %rcx, %r14 ;                        \
        adcq    %rcx, %r14 ;                        \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r15 ;                        \
        adoxq   %rcx, %r15 ;                        \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r13 ;                        \
        movl    %ecx, %r9d ;                        \
        adoxq   %rcx, %r9 ;                         \
        adcxq   %rcx, %r9 ;                         \
        addq    %r9, %r14 ;                         \
        adcq    %rcx, %r15 ;                        \
        movl    %ecx, %r8d ;                        \
        adcq    %rcx, %r8 ;                         \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r15 ;                        \
        adoxq   %rcx, %r8 ;                         \
        adcq    %rcx, %r8 ;                         \
        movl    $0x1, %ebx ;                        \
        addq    %r12, %rbx ;                        \
        leaq    -0x1(%rdx), %rdx ;                  \
        adcq    %r13, %rdx ;                        \
        leaq    -0x1(%rcx), %rcx ;                  \
        movq    %rcx, %rax ;                        \
        adcq    %r14, %rcx ;                        \
        movl    $0xfffffffe, %r11d ;                \
        adcq    %r15, %r11 ;                        \
        adcq    %r8, %rax ;                         \
        cmovbq  %rbx, %r12 ;                        \
        cmovbq  %rdx, %r13 ;                        \
        cmovbq  %rcx, %r14 ;                        \
        cmovbq  %r11, %r15 ;                        \
        movq    %r12, P0 ;                       \
        movq    %r13, 0x8+P0 ;                   \
        movq    %r14, 0x10+P0 ;                  \
        movq    %r15, 0x18+P0

// Almost-Montgomery variant which we use when an input to other muls
// with the other argument fully reduced (which is always safe).
// Again, the basic squaring code is tweaked to avoid modifying %rbp.

#define amontsqr_p256(P0,P1)                    \
        movq    P1, %rdx ;                       \
        mulxq   %rdx, %r8, %r15 ;                     \
        mulxq   0x8+P1, %r9, %r10 ;                \
        mulxq   0x18+P1, %r11, %r12 ;              \
        movq    0x10+P1, %rdx ;                  \
        mulxq   0x18+P1, %r13, %r14 ;              \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r13 ;                        \
        adoxq   %rcx, %r14 ;                        \
        adcq    %rcx, %r14 ;                        \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r15 ;                        \
        adoxq   %rcx, %r15 ;                        \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r13 ;                        \
        movl    %ecx, %r9d ;                        \
        adoxq   %rcx, %r9 ;                         \
        adcxq   %rcx, %r9 ;                         \
        addq    %r9, %r14 ;                         \
        adcq    %rcx, %r15 ;                        \
        movl    %ecx, %r8d ;                        \
        adcq    %rcx, %r8 ;                         \
        xorl    %ecx, %ecx ;                        \
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
        adcxq   %rcx, %r15 ;                        \
        adoxq   %rcx, %r8 ;                         \
        adcq    %rcx, %r8 ;                         \
        movl    $0x1, %r8d ;                        \
        leaq    -0x1(%rdx), %rdx ;                  \
        leaq    -0x1(%rcx), %rax ;                  \
        movl    $0xfffffffe, %r11d ;                \
        cmovzq  %rcx, %r8 ;                         \
        cmovzq  %rcx, %rdx ;                        \
        cmovzq  %rcx, %rax ;                        \
        cmovzq  %rcx, %r11 ;                        \
        addq    %r8, %r12 ;                         \
        adcq    %rdx, %r13 ;                        \
        adcq    %rax, %r14 ;                        \
        adcq    %r11, %r15 ;                        \
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

// Additional macros to help with final multiplexing

#define testzero4(P)                            \
        movq    P, %rax ;                       \
        movq    8+P, %rdx ;                     \
        orq     16+P, %rax ;                    \
        orq     24+P, %rdx ;                    \
        orq     %rdx, %rax

#define mux4(r0,r1,r2,r3,PNE,PEQ)               \
        movq    PNE, r0 ;                      \
        movq    PEQ, %rax ;                     \
        cmovzq  %rax, r0 ;                        \
        movq    8+PNE, r1 ;                    \
        movq    8+PEQ, %rax ;                   \
        cmovzq  %rax, r1 ;                        \
        movq    16+PNE, r2 ;                   \
        movq    16+PEQ, %rax ;                  \
        cmovzq  %rax, r2 ;                        \
        movq    24+PNE, r3 ;                   \
        movq    24+PEQ, %rax ;                  \
        cmovzq  %rax, r3

#define load4(r0,r1,r2,r3,P)                    \
        movq    P, r0 ;                        \
        movq    8+P, r1 ;                      \
        movq    16+P, r2 ;                     \
        movq    24+P, r3

#define store4(P,r0,r1,r2,r3)                   \
        movq    r0, P ;                        \
        movq    r1, 8+P ;                      \
        movq    r2, 16+P ;                     \
        movq    r3, 24+P

S2N_BN_SYMBOL(p256_montjmixadd):
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
// Put the input y in %rbp where it lasts as long as it's needed.

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

        movq    %rdx, %rbp

// Main code, just a sequence of basic field operations
// 8 * multiply + 3 * square + 7 * subtract

        amontsqr_p256(zp2,z_1)

        montmul_p256(y2a,z_1,y_2)
        montmul_p256(x2a,zp2,x_2)
        montmul_p256(y2a,zp2,y2a)

        sub_p256(xd,x2a,x_1)

        sub_p256(yd,y2a,y_1)

        amontsqr_p256(zz,xd)
        montsqr_p256(ww,yd)

        montmul_p256(zzx1,zz,x_1)
        montmul_p256(zzx2,zz,x2a)

        sub_p256(resx,ww,zzx1)
        sub_p256(t1,zzx2,zzx1)

        montmul_p256(resz,xd,z_1)

        sub_p256(resx,resx,zzx2)

        sub_p256(t2,zzx1,resx)

        montmul_p256(t1,t1,y_1)
        montmul_p256(t2,yd,t2)

        sub_p256(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        testzero4(z_1)

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with a z = 1 coordinate (in
// Montgomery form so not the simple constant 1 but rather 2^256 - p_256),
// hence giving 0 + p2 = p2 for the final result.

        mux4(%r8,%r9,%r10,%r11,resx,x_2)
        mux4(%r12,%r13,%r14,%r15,resy,y_2)

        store4(x_3,%r8,%r9,%r10,%r11)
        store4(y_3,%r12,%r13,%r14,%r15)

        load4(%r8,%r9,%r10,%r11,resz)
        movl    $1, %eax
        cmovzq  %rax, %r8
        movq    $0xffffffff00000000, %rax
        cmovzq  %rax, %r9
        movq    $0xffffffffffffffff, %rax
        cmovzq  %rax, %r10
        movl    $0x00000000fffffffe, %eax
        cmovzq  %rax, %r11

        store4(z_3,%r8,%r9,%r10,%r11)

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

S2N_BN_SIZE_DIRECTIVE(p256_montjmixadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
